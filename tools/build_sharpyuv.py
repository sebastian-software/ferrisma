#!/usr/bin/env python3
"""Build the pinned original portable SharpYUV reference."""
import argparse
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[1]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--debug", action="store_true")
    args = parser.parse_args()
    subprocess.run(["python3", ROOT / "tools/verify_upstream.py"], check=True)
    build = ROOT / "build" / ("sharpyuv-reference-debug" if args.debug else "sharpyuv-reference")
    command = ["cmake", "-S", ROOT / "upstream/libwebp", "-B", build, "-G", "Ninja",
               "-DCMAKE_BUILD_TYPE=" + ("Debug" if args.debug else "Release"),
               "-DCMAKE_EXPORT_COMPILE_COMMANDS=ON", "-DBUILD_SHARED_LIBS=ON",
               "-DWEBP_ENABLE_SIMD=OFF", "-DWEBP_USE_THREAD=ON"]
    for name in ["ANIM_UTILS", "CWEBP", "DWEBP", "GIF2WEBP", "IMG2WEBP", "VWEBP", "WEBPINFO",
                 "LIBWEBPMUX", "WEBPMUX", "EXTRAS", "FUZZTEST"]:
        command.append(f"-DWEBP_BUILD_{name}=OFF")
    subprocess.run([str(arg) for arg in command], check=True)
    subprocess.run(["cmake", "--build", build, "--target", "sharpyuv", "-j", "4"], check=True)


if __name__ == "__main__":
    main()
