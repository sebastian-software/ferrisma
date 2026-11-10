#!/usr/bin/env python3
"""Measure Rust AOM encoder time with each RTCD selection on x86-64 GNU Linux.

Encodes the largest image of the identity corpus with `examples/encode_still.rs`
and records the time of avifEncoderWrite for three selections of the same
binary: run-time detection (AVX2 on current runners), AOM_SIMD_CAPS limited to
SSE4.1, and AOM_SIMD_CAPS_MASK=0, which selects every `_c` path and matches the
portable translation before ADR 0058. Each round runs the three selections in
turn; the record keeps the median and minimum of all rounds and requires equal
AVIF sizes, since tools/check_aom_simd_identity.py already compares the bytes.
"""
import argparse
import itertools
import json
import os
from pathlib import Path
import platform
import statistics
import subprocess
import sys

from check_aom_simd_identity import ROOT, SSE4_CAPS, prepare

IMAGE = 'dog-1600x2133'
SELECTIONS = {
    'detected': {},
    'sse4.1': {'AOM_SIMD_CAPS': SSE4_CAPS},
    'c (AOM_SIMD_CAPS_MASK=0)': {'AOM_SIMD_CAPS_MASK': '0'},
}


def encode(binary, image, output, speed, depth, threads, overrides):
    environment = {key: value for key, value in os.environ.items()
                   if key not in ('AOM_SIMD_CAPS', 'AOM_SIMD_CAPS_MASK')}
    environment.update(overrides)
    result = subprocess.run([str(binary), str(image), str(output), f'--speed={speed}', f'--depth={depth}',
                             f'--threads={threads}', '--quality=56', '--report-rtcd'],
                            env=environment, check=True, capture_output=True, text=True)
    lines = dict(line.split('=', 1) for line in result.stdout.split() if '=' in line)
    return int(lines['encode_us']) / 1000, int(lines['bytes']), lines['rtcd']


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--binary', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--rounds', type=int, default=5)
    args = parser.parse_args()
    if (platform.system(), platform.machine()) != ('Linux', 'x86_64'):
        raise SystemExit('This measurement runs on x86-64 GNU Linux')
    work = ROOT / 'build/aom-simd-identity'
    work.mkdir(parents=True, exist_ok=True)
    image = prepare(work)[IMAGE]
    cores = os.cpu_count()
    results = []
    for speed, depth, threads in itertools.product([6, 7], [10, 8], [1, cores]):
        times = {name: [] for name in SELECTIONS}
        sizes, rtcd = {}, {}
        for _ in range(args.rounds):
            for name, overrides in SELECTIONS.items():
                milliseconds, size, selection = encode(args.binary, image, work / 'speed.avif',
                                                       speed, depth, threads, overrides)
                times[name].append(milliseconds)
                sizes[name], rtcd[name] = size, selection
        if len(set(sizes.values())) != 1:
            raise SystemExit(f'AVIF sizes differ between selections: {sizes}')
        median = {name: round(statistics.median(values), 1) for name, values in times.items()}
        reference = median['c (AOM_SIMD_CAPS_MASK=0)']
        results.append(dict(speed=speed, depth=depth, threads=threads,
                            median_ms=median,
                            min_ms={name: round(min(values), 1) for name, values in times.items()},
                            time_vs_c_percent={name: round(100 * value / reference, 1)
                                               for name, value in median.items()},
                            bytes=sizes[next(iter(sizes))], rtcd=rtcd))
        row = '  '.join(f'{name}={value}' for name, value in median.items())
        print(f'speed={speed} depth={depth} threads={threads}  {row}', flush=True)
    cpu = next((line.split(':', 1)[1].strip() for line in Path('/proc/cpuinfo').read_text().splitlines()
                if line.startswith('model name')), 'unknown')
    report = dict(
        check='Rust AOM encoder time with x86 RTCD dispatch, SSE4.1 caps, and every _c path (ADR 0058)',
        host=dict(cpu=cpu, cores=cores, system=platform.platform(), shared='GitHub-hosted runner'),
        input=dict(image=f'{IMAGE} of tools/check_aom_simd_identity.py'),
        encoder=('examples/encode_still.rs: avifImageRGBToYUV then avifEncoderWrite, quality 56, 4:2:0, '
                 'autoTiling when threads != 1; time of avifEncoderWrite only'),
        method=f'{args.rounds} rounds; each round runs the three selections of one binary in turn; median and minimum',
        results=results)
    text = json.dumps(report, indent=2) + '\n'
    args.output.write_text(text)
    sys.stdout.write(text)


if __name__ == '__main__':
    main()
