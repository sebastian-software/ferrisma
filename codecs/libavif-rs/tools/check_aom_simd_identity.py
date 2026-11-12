#!/usr/bin/env python3
"""Require byte-identical AVIF output from the Rust AOM encoder with SIMD on and off.

Each case encodes one still image twice through `examples/encode_still.rs`
(avifImageRGBToYUV + avifEncoderWrite): once with the RTCD defaults detected at
run time and once with AOM_SIMD_CAPS_MASK=0, which selects every `_c`
implementation (ADR 0056). The AVIF bytes must match exactly. The SIMD run
must report that the RTCD pointers select the hand-translated kernels, and the
scalar run that they select `_c`, so an inactive dispatcher cannot pass. The
SSE4.1 run on x86-64 must report SSE4.1 kernels without AVX2 ones.

The corpus is derived from images in the upstream libavif test data with
macOS `sips` on macOS ARM64 (Neon) and with ImageMagick on x86-64 GNU Linux
(ADR 0058), where each case is also encoded with AOM_SIMD_CAPS limiting the
CPU to SSE4.1, so that the SSE2 and SSE4.1 kernels an AVX2 runner would not
select are compared as well. The two hosts derive their bitmaps with different
tools; each report records its own bitmap digests.
"""
import argparse
import hashlib
import itertools
import json
import os
from pathlib import Path
import platform
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]
DATA = ROOT / 'upstream/libavif/tests/data'

# name, source, sips operations (applied in order), purpose
CORPUS = [
    ('dog-1600x2133', 'dog_exif_extended_xmp_icc.jpg',
     [['-r', '90'], ['-z', '2133', '1600']], 'photo at the size used for Ferrisma measurements'),
    ('dog-1001x777', 'dog_exif_extended_xmp_icc.jpg',
     [['-z', '777', '1001']], 'odd dimensions, partial superblocks'),
    ('weld-1024x684', 'weld_16bit.png', [], '16-bit PNG reduced to 8-bit RGB'),
    ('kodim03-768x512', 'kodim03_grayscale_cicp.png', [], 'grayscale content'),
    ('paris-403x302', 'paris_icc_exif_xmp.png', [], 'small photo with odd dimensions'),
    ('abc-512x256', 'abc.png', [], 'synthetic text and flat areas'),
]

# Every image: speeds 6 and 7, 8- and 10-bit, one thread and all threads.
BASE = [dict(speed=s, depth=d, threads=t, quality=56, yuv444=False)
        for s, d, t in itertools.product([6, 7], [8, 10], [1, 0])]
# The small images additionally cover other speeds, qualities, 4:4:4, 12-bit,
# and lossless coding.
EXTRA = [dict(speed=4, depth=10, threads=1, quality=56, yuv444=False),
         dict(speed=9, depth=10, threads=1, quality=56, yuv444=False),
         dict(speed=6, depth=10, threads=1, quality=20, yuv444=False),
         dict(speed=6, depth=10, threads=1, quality=90, yuv444=False),
         dict(speed=6, depth=10, threads=1, quality=56, yuv444=True),
         dict(speed=6, depth=12, threads=1, quality=56, yuv444=False),
         dict(speed=6, depth=8, threads=1, quality=100, yuv444=True)]
SMALL = {'weld-1024x684', 'kodim03-768x512', 'paris-403x302', 'abc-512x256'}

LINUX = sys.platform == 'linux'
# x86_simd_caps() flags up to SSE4.2 (aom_ports/x86.h): MMX, SSE, SSE2, SSE3,
# SSSE3, SSE4_1 and SSE4_2, without AVX, AVX2 or AVX-512.
SSE4_CAPS = hex(0x1 | 0x2 | 0x4 | 0x8 | 0x10 | 0x20 | 0x100)


def imagemagick(operation):
    if operation[0] == '-r':
        return ['-rotate', operation[1]]
    if operation[0] == '-z':
        return ['-resize', f'{operation[2]}x{operation[1]}!']
    raise SystemExit(f'Unmapped sips operation {operation}')


def prepare(work):
    images = {}
    for name, source, operations, _ in CORPUS:
        target = work / f'{name}.bmp'
        if not target.exists() and LINUX:
            subprocess.run(['convert', DATA / source, *(item for operation in operations
                            for item in imagemagick(operation)),
                            '-alpha', 'off', '-type', 'TrueColor', '-depth', '8', f'BMP3:{target}'],
                           check=True, capture_output=True)
        if not target.exists():
            current = DATA / source
            for index, operation in enumerate(operations):
                step = work / f'{name}.step{index}.bmp'
                subprocess.run(['sips', *operation, '-s', 'format', 'bmp', current, '--out', step],
                               check=True, capture_output=True)
                current = step
            subprocess.run(['sips', '-s', 'format', 'bmp', current, '--out', target],
                           check=True, capture_output=True)
        images[name] = target
    return images


def encode(binary, image, output, case, scalar, caps=None):
    environment = os.environ.copy()
    environment.pop('AOM_SIMD_CAPS', None)
    if scalar:
        environment['AOM_SIMD_CAPS_MASK'] = '0'
    else:
        environment.pop('AOM_SIMD_CAPS_MASK', None)
    if caps is not None:
        environment['AOM_SIMD_CAPS'] = caps
    arguments = [binary, image, output, f'--speed={case["speed"]}', f'--depth={case["depth"]}',
                 f'--threads={case["threads"]}', f'--quality={case["quality"]}', '--report-rtcd']
    if case['yuv444']:
        arguments.append('--yuv444')
    result = subprocess.run([str(item) for item in arguments], env=environment, check=True,
                            capture_output=True, text=True)
    lines = dict(line.split('=', 1) for line in result.stdout.split() if '=' in line)
    return hashlib.sha256(output.read_bytes()).hexdigest(), output.stat().st_size, lines['rtcd']


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--binary', type=Path, help='Use an already built encode_still example')
    parser.add_argument('--output', type=Path, default=ROOT / ('build/aom-simd-identity-validation-linux.json'
                        if LINUX else 'build/aom-simd-identity-validation.json'))
    args = parser.parse_args()
    if (platform.system(), platform.machine()) not in {('Darwin', 'arm64'), ('Linux', 'x86_64')}:
        raise SystemExit('The identity corpus runs on macOS ARM64 or x86-64 GNU Linux')
    binary = args.binary
    if binary is None:
        subprocess.run(['cargo', 'build', '--release', '--locked', '--example', 'encode_still'],
                       cwd=ROOT, check=True)
        target = Path(os.environ.get('CARGO_TARGET_DIR', ROOT.parents[1] / 'target'))
        binary = target / 'release/examples/encode_still'
    work = ROOT / 'build/aom-simd-identity'
    work.mkdir(parents=True, exist_ok=True)
    images = prepare(work)
    cases, failures = [], []
    for name, _, _, _ in CORPUS:
        for case in BASE + (EXTRA if name in SMALL else []):
            label = (f'{name} speed={case["speed"]} depth={case["depth"]} threads={case["threads"]} '
                     f'quality={case["quality"]} {"444" if case["yuv444"] else "420"}')
            simd = encode(binary, images[name], work / 'simd.avif', case, scalar=False)
            scalar = encode(binary, images[name], work / 'scalar.avif', case, scalar=True)
            record = dict(image=name, **case, bytes=simd[1], avif_sha256=simd[0],
                          scalar_avif_sha256=scalar[0], simd_rtcd=simd[2], scalar_rtcd=scalar[2])
            ok = simd[0] == scalar[0] and simd[2].startswith('simd') and scalar[2].startswith('c')
            if LINUX:
                sse4 = encode(binary, images[name], work / 'sse4.avif', case, scalar=False, caps=SSE4_CAPS)
                record.update(sse4_caps=SSE4_CAPS, sse4_avif_sha256=sse4[0], sse4_rtcd=sse4[2])
                ok = ok and sse4[0] == scalar[0] and sse4[2] == 'simd(sse4.1)'
            cases.append(record)
            print(f'{"ok  " if ok else "FAIL"} {label} bytes={simd[1]}', flush=True)
            if not ok:
                failures.append(label)
    report = dict(
        check=('AVIF bytes with RTCD SIMD dispatch equal those with AOM_SIMD_CAPS_MASK=0 (ADR 0056)'
               + (' and with AOM_SIMD_CAPS limited to SSE4.1 (ADR 0058)' if LINUX else '')),
        host=dict(system=platform.platform(), machine=platform.machine()),
        corpus=[dict(name=n, source=f'upstream/libavif/tests/data/{s}',
                     **({'imagemagick': [item for operation in o for item in imagemagick(operation)]}
                        if LINUX else {'sips': o}),
                     purpose=p, bmp_sha256=hashlib.sha256(images[n].read_bytes()).hexdigest())
                for n, s, o, p in CORPUS],
        encoder='examples/encode_still.rs (avifImageRGBToYUV, avifEncoderWrite; autoTiling when threads != 1)',
        cases=cases, identical=len(cases) - len(failures), failed=failures)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + '\n')
    print(f'{len(cases) - len(failures)} of {len(cases)} cases byte-identical; report: {args.output}')
    if failures:
        raise SystemExit(1)


if __name__ == '__main__':
    main()
