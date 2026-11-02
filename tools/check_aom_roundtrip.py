#!/usr/bin/env python3
"""Compare odd-sized public encoder/decoder results without CLI control setup."""
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
    parser.add_argument('--debug', action='store_true')
    args = parser.parse_args()
    profile = 'debug' if args.debug else 'release'
    work = ROOT / f'build/aom-roundtrip-{profile}'
    work.mkdir(parents=True, exist_ok=True)
    vmaf = shlex.split(subprocess.check_output(['pkg-config', '--libs', 'libvmaf'], text=True))
    source = ROOT / 'tests/ports/aom_roundtrip.c'
    for implementation, archive in [('c', ROOT / f'build/aom-native-{profile}/libaom.a'),
                                     ('rust', ROOT / f'target/{profile}/libaom.a')]:
        subprocess.run([os.environ.get('CC', 'cc'), '-std=c11', '-ffp-contract=off',
                        '-I', ROOT / 'upstream/aom', source, archive, *vmaf,
                        '-lm', '-lpthread', '-o', work / implementation], check=True)
    records = []
    for depth in [8, 10, 12]:
        for chroma, mono in [(420, 0), (422, 0), (444, 0), (420, 1)]:
            outputs = []
            name = f'{depth}-{chroma}-{mono}'
            for implementation in ['c', 'rust']:
                with (work / f'{name}-{implementation}.log').open('wb') as log:
                    value = subprocess.check_output([work / implementation, str(depth), str(chroma), str(mono)], stderr=log)
                (work / f'{name}-{implementation}.bin').write_bytes(value)
                outputs.append(value)
            if outputs[0] != outputs[1]:
                raise SystemExit(f'Public API bitstream/pixel/metadata mismatch: {name}')
            records.append(dict(bitdepth=depth, chroma=chroma, monochrome=mono, frames=3,
                                exact=True, bytes=len(outputs[0]), sha256=hashlib.sha256(outputs[0]).hexdigest()))
            print(f'Exact public round trip: {name}', flush=True)
    (work / 'summary.json').write_text(json.dumps(dict(profile=profile, cases=records,
        source_sha256=hashlib.sha256(source.read_bytes()).hexdigest()), indent=2) + '\n')


if __name__ == '__main__':
    main()
