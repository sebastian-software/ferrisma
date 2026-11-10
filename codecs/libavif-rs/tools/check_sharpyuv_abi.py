#!/usr/bin/env python3
"""Check original C layouts and a direct Rust caller against each other."""
import argparse
import json
import os
from pathlib import Path
import shlex
import subprocess

ROOT = Path(__file__).resolve().parents[1]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--debug', action='store_true')
    args = parser.parse_args()
    base = Path(os.environ.get('SHARPYUV_BUILD_ROOT', ROOT / 'build'))
    work = base / 'sharpyuv-abi'
    work.mkdir(parents=True, exist_ok=True)
    reference = base / ('sharpyuv-reference-debug' if args.debug else 'sharpyuv-reference')
    executable = work / 'c-layout'
    command = shlex.split(os.environ.get('CC', 'cc')) + ['-std=c11', '-pthread', '-I' + str(ROOT / 'upstream/libwebp'),
        str(ROOT / 'tests/sharpyuv/abi.c'), '-L' + str(reference), '-lsharpyuv', '-Wl,-rpath,' + str(reference), '-o', str(executable)]
    subprocess.run(command, check=True)
    native = json.loads(subprocess.check_output([executable], text=True))
    command = ['cargo', 'run', '-p', 'libavif-sharpyuv', '--locked', '--example', 'layout']
    if not args.debug:
        command.append('--release')
    result = subprocess.run(command, cwd=ROOT, text=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    if result.returncode:
        raise SystemExit(result.stderr)
    candidate = json.loads(result.stdout)
    if native != candidate:
        raise SystemExit(f'Layout or Rust API mismatch: C={native}, Rust={candidate}')
    (work / ('debug.json' if args.debug else 'release.json')).write_text(json.dumps(native, indent=2) + '\n')
    print(f'Identical C/Rust ABI and direct Rust API: {sum(len(values) for values in native.values())} checks')


if __name__ == '__main__':
    main()
