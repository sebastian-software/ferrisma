#!/usr/bin/env python3
"""Build the pinned original portable SharpYUV reference."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import shlex
import subprocess

ROOT = Path(__file__).resolve().parents[1]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--debug", action="store_true")
    parser.add_argument("--compiler-default-diagnostic", action="store_true",
                        help="Build a separate non-acceptance reference with compiler-default contraction")
    args = parser.parse_args()
    subprocess.run(["python3", ROOT / "tools/verify_upstream.py"], check=True)
    base = Path(os.environ.get("SHARPYUV_BUILD_ROOT", ROOT / "build"))
    name = "sharpyuv-compiler-default" if args.compiler_default_diagnostic else "sharpyuv-reference"
    build = base / (name + ("-debug" if args.debug else ""))
    command = ["cmake", "-S", ROOT / "upstream/libwebp", "-B", build, "-G", "Ninja",
               "-DCMAKE_BUILD_TYPE=" + ("Debug" if args.debug else "Release"),
               "-DCMAKE_EXPORT_COMPILE_COMMANDS=ON", "-DBUILD_SHARED_LIBS=ON",
               "-DCMAKE_C_FLAGS=" + ("" if args.compiler_default_diagnostic else "-ffp-contract=off"),
               "-DWEBP_ENABLE_SIMD=OFF", "-DWEBP_USE_THREAD=ON"]
    for name in ["ANIM_UTILS", "CWEBP", "DWEBP", "GIF2WEBP", "IMG2WEBP", "VWEBP", "WEBPINFO",
                 "LIBWEBPMUX", "WEBPMUX", "EXTRAS", "FUZZTEST"]:
        command.append(f"-DWEBP_BUILD_{name}=OFF")
    subprocess.run([str(arg) for arg in command], check=True)
    subprocess.run(["cmake", "--build", build, "--target", "sharpyuv", "-j", "4"], check=True)
    entries = json.loads((build / 'compile_commands.json').read_text())
    entry = next(entry for entry in entries if 'sharpyuv.dir' in entry['command'])
    compiler = shlex.split(entry['command'])[0]
    profile = {'acceptance_reference': not args.compiler_default_diagnostic,
               'c_flags': '' if args.compiler_default_diagnostic else '-ffp-contract=off',
               'build_type': 'Debug' if args.debug else 'Release',
               'compiler': subprocess.check_output([compiler, '--version'], text=True).splitlines()[0],
               'sharp_compile_command': entry['command'],
               'compile_commands_sha256': hashlib.sha256((build / 'compile_commands.json').read_bytes()).hexdigest(),
               'manual_simd': False, 'threading': True}
    (build / 'reference-profile.json').write_text(json.dumps(profile, indent=2) + '\n')


if __name__ == "__main__":
    main()
