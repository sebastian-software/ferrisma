#!/usr/bin/env python3
"""Run the imported codec checks from any directory without changing their oracles."""

import argparse
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]
CODECS = {"jpeg": "libjpeg-turbo-rs", "webp": "libwebp-rs", "jxl": "libjxl-rs", "avif": "libavif-rs"}


def run(directory, *command):
    print(f"[{directory.name}] {' '.join(map(str, command))}", flush=True)
    subprocess.run(list(map(str, command)), cwd=directory, check=True)


def check(codec, stage):
    directory = ROOT / "codecs" / CODECS[codec]
    py = sys.executable
    if codec != "avif":
        command = [py, "tools/check.py", "--stage", "test" if stage == "oracle" else stage]
        if stage == "oracle" and codec != "jxl":
            command += ["--oracle"]
        run(directory, *command)
        if stage == "oracle" and codec == "webp":
            run(directory, *command, "--no-simd")
        return
    if stage == "static":
        run(directory, py, "tools/verify_upstream.py", "--tracked")
        run(directory, py, "tools/check_translation.py")
        run(directory, py, "tools/generate_licenses.py", "--check")
        run(directory, py, "-m", "unittest", "discover", "-s", "tests/ports", "-p", "*_test.py")
    elif stage == "test":
        run(directory, "cargo", "test", "--locked", "--offline", "--release", "-p", "libavif-rs",
            "-p", "libavif-aom", "-p", "libavif-dav1d", "-p", "libavif-sharpyuv")
    else:
        # Bounded local acceptance. The full original suites and sanitizer tiers
        # remain in the avif-* workflows and in the component contributor guide.
        run(directory, py, "tools/build_sharpyuv.py")
        run(directory, "cargo", "build", "--release", "--locked", "--offline", "-p", "libavif-sharpyuv")
        run(directory, py, "tools/check_sharpyuv.py")
        run(directory, "cargo", "test", "--release", "--locked", "--offline",
            "-p", "libavif-sharpyuv", "--test", "cross_port")
        run(directory, py, "tools/check_aom_golden.py")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--codec", action="append", choices=CODECS,
                        help="Repeat to select ports; defaults to all four")
    parser.add_argument("--stage", choices=["static", "test", "oracle"], default="static")
    args = parser.parse_args()
    for codec in args.codec or CODECS:
        check(codec, args.stage)


if __name__ == "__main__":
    main()
