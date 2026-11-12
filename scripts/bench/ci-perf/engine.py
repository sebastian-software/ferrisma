#!/usr/bin/env python3
"""Engine stage breakdown for the v1.0 CI performance runs.

Usage:
  python3 -I engine.py inputs  --cli BIN --inputs DIR --work DIR --out DIR --sizes "12 24"
  python3 -I engine.py run     --cli BIN --work DIR --out DIR --size 12 --part decode|encode
                               [--reps 7] [--warmups 1] [--avif]

`inputs` encodes every input format from the photoN masters with the system
encoders (cjpeg, cwebp, cjxl, avifenc; their versions go to
engine-inputs-N.json) and records bytes, SHA-256 and the CLI color policy each
input needs (`--assume-srgb` if it is accepted, else `--use-rendered-icc`).

`run` times the release CLI as fresh processes with the interleaved runner:

- part `decode`: every input format and JPEG XL decoder, transcoded to lossy
  WebP (the cheapest encoder, constant across rows), resized to 2048 px wide
  and not resized, with one thread and all cores. Gives decode and transform
  per input format.
- part `encode`: the JPEG q90 input to AVIF (workspace libavif/AOM writer and
  rav1e), lossy WebP and lossless JPEG XL (effort 2, the only admitted output
  profile), resized and not resized, one thread and all cores. Gives the
  encode stage per output format.

The CLI report line provides open/decode/transform/encode/total and the
engine's peak managed bytes; /usr/bin/time provides process wall and max RSS.
Stages are additive in the CLI report, so an input-format x output-format
cell is decode(input) + transform + encode(output) of these two parts.
"""

import argparse
import hashlib
import json
import os
import shutil
import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
COMMON = ["--budget-mb=4096", "--max-output-mb=1024", "--max-input-mb=1024"]


def sha256(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def tool_version(command):
    try:
        result = subprocess.run(command, capture_output=True, text=True, timeout=30)
        text = (result.stdout + result.stderr).strip().splitlines()
        return text[0] if text else ""
    except Exception as error:  # noqa: BLE001 - recorded, not fatal
        return f"unavailable: {error}"


def ppm_dims(path):
    with open(path, "rb") as f:
        head = f.read(64).split()
    return int(head[1]), int(head[2])


def run(command):
    print("+ " + " ".join(map(str, command)), file=sys.stderr, flush=True)
    subprocess.run([str(c) for c in command], check=True)


def make_inputs(args):
    work = Path(args.work)
    work.mkdir(parents=True, exist_ok=True)
    inputs = Path(args.inputs)
    cjpeg = shutil.which("cjpeg") or "/opt/homebrew/opt/jpeg-turbo/bin/cjpeg"
    tools = {"cjpeg": cjpeg, "cwebp": shutil.which("cwebp"), "cjxl": shutil.which("cjxl"),
             "avifenc": shutil.which("avifenc")}
    versions = {"cjpeg": tool_version([cjpeg, "-version"]),
                "cwebp": tool_version([tools["cwebp"] or "cwebp", "-version"]),
                "cjxl": tool_version([tools["cjxl"] or "cjxl", "--version"]),
                "avifenc": tool_version([tools["avifenc"] or "avifenc", "--version"])}
    for size in args.sizes.split():
        ppm = inputs / f"photo{size}.ppm"
        png = inputs / f"photo{size}.png"
        width, height = ppm_dims(ppm)
        entries = []

        def add(name, path, producer, display, note=""):
            entries.append({"name": name, "path": str(path), "producer": producer,
                            "display": display, "note": note})

        out = work / f"photo{size}-q90.jpg"
        run([tools["cjpeg"], "-quality", "90", "-outfile", out, ppm])
        add("jpeg-q90", out, "cjpeg -quality 90 (4:2:0 baseline)", (width, height))
        original = inputs / f"photo{size}-original.jpg"
        # The camera originals: dog is EXIF orientation 6 (display is transposed),
        # mountain is orientation 1; both carry an sRGB ICC profile.
        display = (height, width) if size == "12" else (width, height)
        add("jpeg-camera", original, "camera original (sRGB ICC, EXIF)", display,
            "rendered sRGB ICC; dog has EXIF orientation 6")
        if tools["cwebp"]:
            out = work / f"photo{size}-q75.webp"
            run([tools["cwebp"], "-quiet", "-q", "75", ppm, "-o", out])
            add("webp-lossy", out, "cwebp -q 75", (width, height))
            out = work / f"photo{size}-lossless.webp"
            run([tools["cwebp"], "-quiet", "-lossless", ppm, "-o", out])
            add("webp-lossless", out, "cwebp -lossless", (width, height))
        if tools["cjxl"]:
            out = work / f"photo{size}-e7d1.jxl"
            run([tools["cjxl"], "--quiet", "-e", "7", "-d", "1", png, out])
            add("jxl-vardct", out, "cjxl -e 7 -d 1", (width, height))
            out = work / f"photo{size}-d0e7.jxl"
            run([tools["cjxl"], "--quiet", "-e", "7", "-d", "0", png, out])
            add("jxl-lossless", out, "cjxl -e 7 -d 0", (width, height))
        if tools["avifenc"] and args.avif:
            out = work / f"photo{size}-q60.avif"
            run([tools["avifenc"], "-q", "60", "-s", "6", "-j", "all", png, out])
            add("avif", out, "avifenc -q 60 -s 6 (8-bit 4:2:0)", (width, height))
        for entry in entries:
            entry["bytes"] = Path(entry["path"]).stat().st_size
            entry["sha256"] = sha256(entry["path"])
            entry["policy"] = probe_policy(args.cli, entry, work)
        record = {"size": size, "master": {"width": width, "height": height,
                                           "sha256": sha256(ppm)},
                  "tool_versions": versions, "inputs": entries}
        (Path(args.out) / f"engine-inputs-{size}.json").write_text(json.dumps(record, indent=2) + "\n")
        (work / f"engine-inputs-{size}.json").write_text(json.dumps(record, indent=2) + "\n")


def probe_policy(cli, entry, work):
    width, height = entry["display"]
    target = work / "probe.webp"
    for policy in ("--assume-srgb", "--use-rendered-icc"):
        result = subprocess.run([cli, entry["path"], str(target), "256",
                                 str(max(1, round(256 * height / width))), policy, *COMMON],
                                capture_output=True, text=True)
        if result.returncode == 0:
            return policy
        print(f"policy {policy} rejected for {entry['name']}: {result.stderr.strip()[-300:]}",
              file=sys.stderr)
    return None


def candidate(cli, work, entry, output, extra, resize, threads, meta):
    width, height = entry["display"]
    if resize:
        w, h = 2048, max(1, round(2048 * height / width))
    else:
        w, h = width, height
    target = work / "out" / f"{meta['input']}-{meta['output']}-{'r' if resize else 'n'}-t{threads}.{output}"
    target.parent.mkdir(parents=True, exist_ok=True)
    cmd = [cli, entry["path"], str(target), str(w), str(h), entry["policy"], *COMMON,
           # The WebP and JPEG XL writers are serial and accept only one thread.
           f"--encoder-threads={threads if output == 'avif' else 1}",
           f"--transform-threads={threads}", *extra]
    ident = f"{meta['input']}>{meta['output']}:{'2048w' if resize else 'full'}:t{'N' if threads == 0 else threads}"
    return {"id": ident, "cmd": cmd, "output": str(target),
            "meta": {**meta, "resize": f"{w}x{h}" if resize else "none", "threads": threads,
                     "encoder_threads": threads if output == "avif" else 1,
                     "dims_out": [w, h]}}


def plan(args):
    work = Path(args.work)
    record = json.loads((work / f"engine-inputs-{args.size}.json").read_text())
    inputs = {e["name"]: e for e in record["inputs"] if e.get("policy")}
    candidates = []
    if args.part.startswith("decode"):
        # decode-a: JPEG and WebP inputs; decode-b: JPEG XL and AVIF inputs;
        # decode: all (one job per part keeps 24 MP runs under the job limit).
        wanted = {"decode-a": ("jpeg", "webp"), "decode-b": ("jxl", "avif")}.get(
            args.part, ("jpeg", "webp", "jxl", "avif"))
        for name, entry in inputs.items():
            if not name.startswith(wanted):
                continue
            decoders = [("", [])]
            if name.startswith("jxl"):
                decoders = [("libjxl-rs", ["--jxl-decoder=libjxl-rs"]),
                            ("jxl-oxide", ["--jxl-decoder=jxl-oxide"])]
            for decoder, flags in decoders:
                label = f"{name}/{decoder}" if decoder else name
                for resize in (True, False):
                    for threads in (1, 0):
                        candidates.append(candidate(
                            args.cli, work, entry, "webp", flags, resize, threads,
                            {"part": args.part, "input": label, "output": "webp-q75",
                             "size": args.size}))
    else:
        entry = inputs["jpeg-q90"]
        outputs = [("webp-q75", "webp", []),
                   ("avif-rav1e", "avif", ["--avif-encoder=rav1e"])]
        if args.avif:
            outputs.insert(0, ("avif-aom", "avif", ["--avif-encoder=libavif"]))
        if args.jxl_out:
            # The CLI's default .jxl output is lossy (distance 1.0), so each
            # row names its mode. The serial lossy encoder takes about a
            # minute or more per 24 MP sample, so that row runs at 12 MP only.
            outputs.append(("jxl-lossless-e2", "jxl", ["--jxl-lossless"]))
            if str(args.size) == "12":
                outputs.append(("jxl-lossy-d1", "jxl", ["--jxl-distance=1.0"]))
        for label, ext, flags in outputs:
            for resize in (True, False):
                for threads in (1, 0):
                    candidates.append(candidate(
                        args.cli, work, entry, ext, flags, resize, threads,
                        {"part": "encode", "input": "jpeg-q90", "output": label,
                         "size": args.size}))
    return {"reps": args.reps, "warmups": args.warmups,
            "group": f"engine/{args.part.split('-')[0]}/photo{args.size}", "candidates": candidates}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("mode", choices=["inputs", "run"])
    parser.add_argument("--cli", required=True)
    parser.add_argument("--inputs")
    parser.add_argument("--work", required=True)
    parser.add_argument("--out", required=True)
    parser.add_argument("--sizes", default="12 24")
    parser.add_argument("--size")
    parser.add_argument("--part", choices=["decode", "decode-a", "decode-b", "encode"])
    parser.add_argument("--reps", type=int, default=7)
    parser.add_argument("--warmups", type=int, default=1)
    parser.add_argument("--avif", action="store_true", help="AVIF input and the AOM writer exist")
    parser.add_argument("--jxl-out", action="store_true", help="the CLI has jpeg-xl-write")
    args = parser.parse_args()
    Path(args.out).mkdir(parents=True, exist_ok=True)
    if args.mode == "inputs":
        make_inputs(args)
        return
    plan_record = plan(args)
    path = Path(args.work) / f"plan-engine-{args.part}-{args.size}.json"
    path.write_text(json.dumps(plan_record, indent=2) + "\n")
    subprocess.run([sys.executable, "-I", str(HERE / "run_interleaved.py"), str(path),
                    str(Path(args.out) / f"engine-{args.part}-{args.size}.json")], check=True)
    # Outputs are large; keep only their hashes (in the record).
    shutil.rmtree(Path(args.work) / "out", ignore_errors=True)


if __name__ == "__main__":
    main()
