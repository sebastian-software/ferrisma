#!/usr/bin/env python3
"""Capture genuine raw JXL from the pinned original public JPEG encoder."""
import argparse
import hashlib
import json
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parent.parent


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--oracle", type=Path, default=ROOT / "target/oracle/jxl_oracle")
    parser.add_argument("--cjpeg", type=Path, help="Regenerate the three original RGB JPEG inputs")
    args = parser.parse_args()
    destination = ROOT / "tests/data/jpeg-reconstruction"
    destination.mkdir(exist_ok=True)
    if args.cjpeg:
        width, height = 31, 19
        pixels = bytes((x * 17 + y * 23 + c * 61 + (x * y) % 47) % 256
                       for y in range(height) for x in range(width) for c in range(3))
        ppm = f"P6\n{width} {height}\n255\n".encode() + pixels
        version = subprocess.run([str(args.cjpeg), "-version"], text=True, capture_output=True, check=True)
        rgb = {"cjpeg_version": (version.stdout + version.stderr).strip(),
               "cjpeg_sha256": hashlib.sha256(args.cjpeg.resolve().read_bytes()).hexdigest(),
               "pixel_recipe": "RGB8(x,y,c)=(x*17+y*23+c*61+(x*y)%47)%256; 31x19 binary P6 PPM",
               "cases": []}
        for name, extra in [("rgb-baseline", ["-baseline"]),
                            ("rgb-progressive", ["-progressive"]),
                            ("rgb-restarts", ["-restart", "1B"])]:
            options = ["-rgb", "-quality", "90", "-dct", "int", *extra]
            result = subprocess.run([str(args.cjpeg), *options], input=ppm, capture_output=True, check=True)
            path = destination / (name + ".jpg")
            path.write_bytes(result.stdout)
            rgb["cases"].append({"jpeg": path.name, "options": options,
                                  "sha256": hashlib.sha256(result.stdout).hexdigest()})
        (destination / "rgb-inputs.json").write_text(json.dumps(rgb, indent=2) + "\n")
    cases = []
    inputs = sorted((ROOT / "tests/data/jpeg-writer").glob("*.jpg"))
    inputs += sorted(destination.glob("rgb-*.jpg"))
    for path in inputs:
        jpeg = path.read_bytes()
        request = "ycc-jpeg-encode " + jpeg.hex() + "\n"
        result = subprocess.run([str(args.oracle)], input=request, text=True,
                                capture_output=True, check=True).stdout.strip()
        prefix, encoded = result.rsplit(" ", 1)
        if " API 0 0 0 0 W 0 0" not in prefix:
            raise RuntimeError(result)
        wire = bytes.fromhex(encoded)
        target = destination / (path.stem + ".jxl")
        target.write_bytes(wire)
        cases.append({"jpeg": str(path.relative_to(ROOT / "tests/data")),
                      "jxl": str(target.relative_to(ROOT / "tests/data")),
                      "jpeg_sha256": hashlib.sha256(jpeg).hexdigest(),
                      "jxl_sha256": hashlib.sha256(wire).hexdigest(),
                      "source_encoder_observation": result})
    (destination / "provenance.json").write_text(json.dumps({
        "libjxl_revision": "a16d32d0ac113f5e54125e49bb571281c5d0b704",
        "description": "Existing genuine cjpeg files; original ParseJPG and JxlEncoderAddJPEGFrame with container and reconstruction-metadata storage disabled.",
        "cases": cases}, indent=2) + "\n")
    captures = []
    for case in cases:
        jpeg = (ROOT / "tests/data" / case["jpeg"]).read_bytes()
        wire = (ROOT / "tests/data" / case["jxl"]).read_bytes()
        parsed = subprocess.run([str(args.oracle)], input="jw-fixture " + jpeg.hex() + "\n",
                                text=True, capture_output=True, check=True).stdout.strip()
        status, fields = parsed.split(" ", 1)
        if status != "0":
            raise RuntimeError("Actual original JPEG fixture reader failed")
        stem = Path(case["jxl"]).stem
        data = destination / (stem + ".jpeg-data.txt")
        data.write_text(fields + "\n")
        for partial in (False, True):
            selected = wire[:-1] if partial else wire
            request = "jr-frame " + selected.hex() + " " + fields
            observed = subprocess.run([str(args.oracle)], input=request + "\n", text=True,
                                      capture_output=True, check=True).stdout.strip()
            target = destination / (stem + ("-partial" if partial else "") + ".original.txt")
            target.write_text(observed + "\n")
            captures.append({"jxl": case["jxl"], "remove_last_byte": partial,
                             "jpeg_data": str(data.relative_to(ROOT / "tests/data")),
                             "jpeg_data_sha256": hashlib.sha256(data.read_bytes()).hexdigest(),
                             "original": str(target.relative_to(ROOT / "tests/data")),
                             "original_sha256": hashlib.sha256(target.read_bytes()).hexdigest()})
    (destination / "native-captures.json").write_text(json.dumps(captures, indent=2) + "\n")
    (destination / "native-captures.tsv").write_text("".join(
        "\t".join([capture["jxl"], str(int(capture["remove_last_byte"])),
                    capture["jpeg_data"], capture["original"]]) + "\n" for capture in captures))
    rows = []
    for case in json.loads((ROOT / "tests/data/ycbcr-jpeg-fixtures.json").read_text())["cases"]:
        rows.append((case["jpeg"], case["jxl"]))
    for case in json.loads((ROOT / "tests/data/decoder-geometry-fixtures.json").read_text())["cases"]:
        rows.append(("decoder-geometry/" + case["jpeg"], "decoder-geometry/" + case["jxl"]))
    rows.extend((case["jpeg"], case["jxl"]) for case in cases)
    (destination / "fixtures.tsv").write_text("".join(jpeg + "\t" + jxl + "\n" for jpeg, jxl in rows))
    print(f"Captured {len(cases)} original public encoder files; retained {len(rows)} JPEG pairs")


if __name__ == "__main__":
    main()
