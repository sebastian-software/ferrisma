#!/usr/bin/env python3
"""Compare deterministic AVIF bitstreams and decoded image bytes across implementations."""
import argparse
import hashlib
import json
from pathlib import Path
import subprocess
import shlex
import shutil

ROOT = Path(__file__).resolve().parents[1]


def noncontracting_decoder(baseline):
    """Build a diagnostic C oracle changing only FP contraction in reformat.c."""
    work = ROOT / "build/fp-oracle"
    work.mkdir(exist_ok=True)
    entry = next(entry for entry in json.loads((baseline / "compile_commands.json").read_text())
                 if entry["file"].endswith("/src/reformat.c"))
    command = shlex.split(entry["command"])
    command[command.index("-o") + 1] = str(work / "reformat.c.o")
    command.append("-ffp-contract=off")
    subprocess.run(command, cwd=entry["directory"], check=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    shutil.copyfile(baseline / "libavif_internal.a", work / "libavif_internal.a")
    subprocess.run(["ar", "r", str(work / "libavif_internal.a"), str(work / "reformat.c.o")], check=True)
    line = subprocess.check_output(["ninja", "-C", str(baseline), "-t", "commands", "avifdec"], text=True).splitlines()[-1]
    tokens = shlex.split(line)
    if tokens[:2] != [":", "&&"] or tokens[-2:] != ["&&", ":"]:
        raise SystemExit("Unexpected Ninja linker command format for the diagnostic decoder")
    command = tokens[2:-2]
    command[command.index("-o") + 1] = str(work / "avifdec")
    command = [str(work / "libavif_internal.a") if token == "libavif_internal.a" else token for token in command]
    subprocess.run(command, cwd=baseline, check=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    return work / "avifdec"


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--baseline", type=Path, default=ROOT / "build/baseline")
    parser.add_argument("--port", type=Path, default=ROOT / "build/port")
    parser.add_argument("--diagnose-float", action="store_true",
                        help="Diagnose PNG mismatches against a separate C oracle with FP contraction disabled")
    args = parser.parse_args()
    work = ROOT / "build/differential"
    work.mkdir(parents=True, exist_ok=True)
    data = ROOT / "upstream/libavif/tests/data"
    report = []
    fixtures = [
        ("abc.png", []),
        ("abc.png", ["--depth", "10", "--yuv", "420", "--range", "limited"]),
        ("abc.png", ["--depth", "12", "--yuv", "422"]),
        ("abc.png", ["--lossless"]),
        ("abc.png", ["--premultiply"]),
        ("abc.png", ["--yuv", "420", "--sharpyuv"]),
        ("abc.png", ["--progressive"]),
        ("draw_points.png", ["--irot", "1", "--imir", "0"]),
        ("ffffcc-gamma2.2.png", ["--depth", "12"]),
        ("dog_exif_extended_xmp_icc.jpg", []),
    ]
    for index, (fixture, options) in enumerate(fixtures):
        products = []
        for implementation, build in [("c", args.baseline), ("rust", args.port)]:
            encoded = work / f"{index}-{implementation}.avif"
            decoded = work / f"{index}-{implementation}.png"
            commands = [
                [build / "avifenc", "--speed", "10", "--jobs", "1", "--codec", "aom",
                 "--creation-time", "1", "--modification-time", "1", *options, data / fixture, encoded],
                [build / "avifdec", "--jobs", "1", encoded, decoded],
            ]
            for command in commands:
                subprocess.run([str(arg) for arg in command], check=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
            products.append((encoded.read_bytes(), decoded.read_bytes()))
        if products[0][0] != products[1][0]:
            raise SystemExit(f"AVIF bitstream mismatch: {fixture} {options}; inspect {work}")
        png_equal = products[0][1] == products[1][1]
        contraction_match = None
        if not png_equal and args.diagnose_float:
            decoder = noncontracting_decoder(args.baseline)
            decoded = work / f"{index}-c-no-fp-contract.png"
            subprocess.run([str(decoder), "--jobs", "1", str(work / f"{index}-c.avif"), str(decoded)],
                           check=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
            contraction_match = decoded.read_bytes() == products[1][1]
            if not contraction_match:
                raise SystemExit(f"Unexplained decoded-image mismatch: {fixture} {options}")
        report.append({"fixture": fixture, "options": options,
                       "avif_sha256": hashlib.sha256(products[0][0]).hexdigest(),
                       "c_png_sha256": hashlib.sha256(products[0][1]).hexdigest(),
                       "rust_png_sha256": hashlib.sha256(products[1][1]).hexdigest(),
                       "png_byte_equal": png_equal,
                       "matches_noncontracting_c": contraction_match})
    (work / "summary.json").write_text(json.dumps(report, indent=2) + "\n")
    exact = sum(item["png_byte_equal"] for item in report)
    print(f"Byte-identical C/Rust output: {len(report)} AVIF bitstreams and {exact}/{len(report)} decoded PNGs")
    if exact != len(report):
        if not args.diagnose_float:
            raise SystemExit("Strict PNG parity failed; rerun with --diagnose-float to investigate, not to certify byte parity")
        print(f"Confirmed FP contraction differences: {len(report) - exact}; full PNG byte parity is NOT established")


if __name__ == "__main__":
    main()
