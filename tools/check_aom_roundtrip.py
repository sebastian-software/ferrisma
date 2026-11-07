#!/usr/bin/env python3
"""Compare odd-sized public encoder/decoder results without CLI control setup."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import subprocess

from aom_sanitizer import instrumented_libraries

ROOT = Path(__file__).resolve().parents[1]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--debug', action='store_true')
    parser.add_argument('--asan', action='store_true')
    args = parser.parse_args()
    if args.asan and args.debug:
        parser.error('The sanitizer configuration retains original Release preprocessing')
    profile = 'debug' if args.debug else 'release'
    work = ROOT / (f'build/aom-roundtrip-{profile}' + ('-asan' if args.asan else ''))
    work.mkdir(parents=True, exist_ok=True)
    source = ROOT / 'tests/ports/aom_roundtrip.c'
    reference, candidate = ROOT / f'build/aom-native-{profile}/libaom.a', ROOT / f'target/{profile}/libaom.a'
    compiler, environment, runtime = os.environ.get('CC', 'cc'), os.environ.copy(), []
    if args.asan:
        reference, candidate, compiler, environment, runtime = instrumented_libraries()
    caller = work / 'caller.o'
    subprocess.run([compiler, '-std=c11', '-ffp-contract=off', '-g', '-I', ROOT / 'upstream/aom',
                    *(['-fsanitize=address', '-fno-omit-frame-pointer'] if args.asan else []),
                    '-c', source, '-o', caller], env=environment, check=True)
    for implementation, archive in [('c', reference), ('rust', candidate)]:
        subprocess.run([compiler, caller, archive, *runtime,
                        '-lm', '-lpthread', '-o', work / implementation], env=environment, check=True)
    records = []
    for depth in [8, 10, 12]:
        for chroma, mono in [(420, 0), (422, 0), (444, 0), (420, 1)]:
            outputs = []
            name = f'{depth}-{chroma}-{mono}'
            for implementation in ['c', 'rust']:
                with (work / f'{name}-{implementation}.log').open('wb') as log:
                    value = subprocess.check_output([work / implementation, str(depth), str(chroma), str(mono)],
                                                    env=environment, stderr=log)
                (work / f'{name}-{implementation}.bin').write_bytes(value)
                outputs.append(value)
            if outputs[0] != outputs[1]:
                raise SystemExit(f'Public API bitstream/pixel/metadata mismatch: {name}')
            records.append(dict(bitdepth=depth, chroma=chroma, monochrome=mono, frames=3,
                                exact=True, bytes=len(outputs[0]), sha256=hashlib.sha256(outputs[0]).hexdigest()))
            print(f'Exact public round trip: {name}', flush=True)
    (work / 'summary.json').write_text(json.dumps(dict(profile=profile, cases=records,
        sanitizer='address' if args.asan else None, caller_instrumented=args.asan,
        original_archive_sha256=hashlib.sha256(reference.read_bytes()).hexdigest(),
        rust_archive_sha256=hashlib.sha256(candidate.read_bytes()).hexdigest(),
        source_sha256=hashlib.sha256(source.read_bytes()).hexdigest()), indent=2) + '\n')


if __name__ == '__main__':
    main()
