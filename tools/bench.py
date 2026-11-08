#!/usr/bin/env python3
"""Measure the port's decode speed against libjpeg-turbo (ADR 0004).

Builds examples/bench.rs in release mode and compares it with the oracle's
bench probe (tools/oracle/probes/wp9/bench.c), which decodes the same way
through the C library:

  simd      the port with its default `simd` feature against the simd oracle
            (WITH_SIMD=1, upstream's default build): Neon on ARM64, AVX2
            where an x86-64 CPU has it
  sse2      on x86-64, the same with JSIMD_FORCESSE2=1 in both (ADR 0008)
  portable  the port without default features against the release oracle
            (WITH_SIMD=0, the reference configuration)

The inputs are a photo made by tiling upstream's testimages/testorig.ppm to
3632x2384 pixels (16 x 16 tiles), encoded by the oracle's cjpeg as baseline
4:2:0, 4:2:2 and 4:4:4 and progressive 4:2:0 at quality 90, plus upstream's
testorig.jpg. The cases select the kernels in turn: the islow, ifast and float
IDCTs, the 4x4 and 2x2 IDCTs of the scales 1/2 and 1/4, fancy h2v2 and h2v1
upsampling, merged h2v2 and h2v1 upsampling, and the color conversion to RGB
and RGBX. Speed is measured on native hosts only, not under Rosetta 2. Each case decodes REPEATS times per run; the table shows the
median time per decode over RUNS runs and requires both sides to produce the
same output (FNV-1a checksum). The JSON record goes to build/bench/.
"""

import argparse
import json
import os
from pathlib import Path
import platform
import statistics
import subprocess
import sys

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(Path(__file__).resolve().parent))

import build_oracle  # noqa: E402

UPSTREAM = ROOT / "upstream/libjpeg-turbo"
OUT = ROOT / "build/bench"
TILES = 16

# The J_COLOR_SPACE numbers of jpeglib.h for the probe's -out.
JCS = {"rgb": 6, "rgbx": 7, "gray": 1}

CASES = [
    ("baseline 4:2:0, RGB", "photo-420.jpg", []),
    ("baseline 4:2:2, RGB", "photo-422.jpg", []),
    ("baseline 4:4:4, RGB", "photo-444.jpg", []),
    ("progressive 4:2:0, RGB", "photo-progressive.jpg", []),
    ("baseline 4:2:0, RGBX", "photo-420.jpg", ["-format", "rgbx"]),
    ("baseline 4:2:0, fast IDCT", "photo-420.jpg", ["-dct", "ifast"]),
    ("baseline 4:2:0, float IDCT", "photo-420.jpg", ["-dct", "float"]),
    ("baseline 4:2:0, merged upsampling", "photo-420.jpg", ["-nofancy"]),
    ("baseline 4:2:2, merged upsampling", "photo-422.jpg", ["-nofancy"]),
    ("baseline 4:2:0, scale 1/2", "photo-420.jpg", ["-scale", "1", "2"]),
    ("baseline 4:2:0, scale 1/4", "photo-420.jpg", ["-scale", "1", "4"]),
    ("baseline 4:2:0, scale 1/8", "photo-420.jpg", ["-scale", "1", "8"]),
    ("testorig.jpg (227x149)", "testorig.jpg", []),
]


def run(*args, **kwargs):
    return subprocess.run(list(map(str, args)), cwd=ROOT, check=True, **kwargs)


def read_ppm(path):
    """The width, height and pixel bytes of a binary PPM (P6, maxval 255)."""
    data = path.read_bytes()
    fields, position = [], 0
    while len(fields) < 4:
        while data[position:position + 1].isspace():
            position += 1
        if data[position:position + 1] == b"#":
            position = data.index(b"\n", position)
            continue
        end = position
        while not data[end:end + 1].isspace():
            end += 1
        fields.append(data[position:end])
        position = end
    if fields[0] != b"P6" or fields[3] != b"255":
        raise SystemExit(f"{path}: not an 8-bit binary PPM")
    width, height = int(fields[1]), int(fields[2])
    return width, height, data[position + 1:position + 1 + width * height * 3]


def make_inputs(oracle):
    """Writes the tiled photo and its encodings into build/bench/."""
    OUT.mkdir(parents=True, exist_ok=True)
    width, height, pixels = read_ppm(UPSTREAM / "testimages/testorig.ppm")
    photo = OUT / "photo.ppm"
    if not photo.exists():
        rows = [pixels[y * width * 3:(y + 1) * width * 3] * TILES for y in range(height)]
        body = b"".join(rows) * TILES
        photo.write_bytes(b"P6\n%d %d\n255\n" % (width * TILES, height * TILES) + body)
    cjpeg = oracle / "libjpeg-turbo/cjpeg-static"
    for name, args in [("photo-420.jpg", []), ("photo-422.jpg", ["-sample", "2x1"]),
                       ("photo-444.jpg", ["-sample", "1x1"]),
                       ("photo-progressive.jpg", ["-progressive"])]:
        target = OUT / name
        if not target.exists():
            run(cjpeg, "-quality", "90", *args, "-outfile", target, photo)
    (OUT / "testorig.jpg").write_bytes((UPSTREAM / "testimages/testorig.jpg").read_bytes())


def build_port(features, name):
    """Builds examples/bench.rs with `features` and copies it to build/bench."""
    run("cargo", "build", "--locked", "--release", "--example", "bench", *features)
    target = OUT / name
    target.write_bytes((ROOT / "target/release/examples/bench").read_bytes())
    target.chmod(0o755)
    return target


def probe_args(args):
    """The probe's options for the example's `args`."""
    result, i = [], 0
    while i < len(args):
        if args[i] == "-format":
            result += ["-out", str(JCS[args[i + 1]])]
            i += 2
        else:
            result.append(args[i])
            i += 1
    return result


def measure(command, runs, env):
    """The median nanoseconds per decode over `runs` runs, and the output's
    dims and checksum."""
    times, record = [], None
    for _ in range(runs):
        output = subprocess.run(list(map(str, command)), check=True, env=env,
                                stdout=subprocess.PIPE, universal_newlines=True).stdout
        fields = dict(line.split("=", 1) for line in output.split())
        times.append(int(fields["nanoseconds"]) / int(fields["decodes"]))
        current = (fields["dims"], fields["checksum"])
        if record and current != record:
            raise SystemExit(f"{command}: output changed between runs")
        record = current
    return statistics.median(times), record


def main():
    parser = argparse.ArgumentParser(description=__doc__,
                                     formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--mode", choices=["simd", "sse2", "portable", "all"], default="all",
                        help="what to measure (default: all that the host has)")
    parser.add_argument("--repeats", type=int, default=10, help="decodes per run")
    parser.add_argument("--runs", type=int, default=5, help="runs per case")
    args = parser.parse_args()

    x86_64 = build_oracle.host_arch() == "x86_64"
    if args.mode == "all":
        modes = ["simd", "sse2", "portable"] if x86_64 else ["simd", "portable"]
    elif args.mode == "sse2" and not x86_64:
        raise SystemExit("--mode sse2 needs an x86-64 host")
    else:
        modes = [args.mode]
    env = dict(os.environ)
    for name in ("JSIMD_FORCENONE", "JSIMD_FORCESSE2", "JSIMD_FORCENEON"):
        env.pop(name, None)
    jobs = "8"
    results = {"host": platform.platform(), "machine": platform.machine(),
               "repeats": args.repeats, "runs": args.runs, "modes": {}}
    for mode in modes:
        variant = "release" if mode == "portable" else "simd"
        mode_env = {**env, "JSIMD_FORCESSE2": "1"} if mode == "sse2" else env
        oracle = build_oracle.build(variant, jobs, fresh=False)
        make_inputs(oracle)
        features = ["--no-default-features"] if mode == "portable" else []
        port = build_port(features, f"bench-{mode}")
        probe = oracle / "probes/wp9/bench"
        rows = []
        print(f"\n{mode}: port ({'default features' if features == [] else 'no default features'}) "
              f"against the {variant} oracle"
              f"{', JSIMD_FORCESSE2=1' if mode == 'sse2' else ''}, "
              f"median of {args.runs} runs x {args.repeats} decodes")
        print("| Case | libjpeg-turbo ms | port ms | port / C |")
        print("| --- | ---: | ---: | ---: |")
        for label, name, case_args in CASES:
            path = OUT / name
            c_ns, c_record = measure([probe, "-repeats", args.repeats,
                                      *probe_args(case_args), path], args.runs, mode_env)
            rust_ns, rust_record = measure([port, "-repeats", args.repeats, *case_args, path],
                                           args.runs, mode_env)
            if c_record != rust_record:
                raise SystemExit(f"{label}: outputs differ: C {c_record}, port {rust_record}")
            ratio = rust_ns / c_ns
            rows.append({"case": label, "file": name, "args": case_args, "dims": c_record[0],
                         "c_ns": c_ns, "port_ns": rust_ns, "ratio": ratio})
            print(f"| {label} | {c_ns / 1e6:.2f} | {rust_ns / 1e6:.2f} | {ratio:.2f} |",
                  flush=True)
        results["modes"][mode] = {"oracle": variant, "features": features,
                                  "forcesse2": mode == "sse2", "cases": rows}
    (OUT / "results.json").write_text(json.dumps(results, indent=2) + "\n")
    print(f"\nRecord: {(OUT / 'results.json').relative_to(ROOT)}")


if __name__ == "__main__":
    main()
