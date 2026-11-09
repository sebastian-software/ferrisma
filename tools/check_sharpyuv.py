#!/usr/bin/env python3
"""Compare independent C/Rust processes over the portable SharpYUV API.

Each profile compares exact return values, matrices, and output bytes over
1,271 comparison groups: all 960 combinations of RGB depth (8, 10, 12, 16),
YUV depth (8, 10, 12), the sixteen defined transfer functions, and the five
predefined matrices; interleaved, RGBA-step, and planar input with padded
rows and aligned 16-bit storage; nine rotating shapes; 256 seeded
conversions up to 64 by 32; computed matrices, version and options behavior,
defined null and invalid-argument rejection, repeated callback
initialization, and cold conversions on concurrent threads. Input
immutability and output guard bytes are checked.

Inputs that trigger undefined behavior in the C source are not a defined
equivalence oracle and are excluded. The inventory is not the complete
Cartesian product of shapes and layouts and proves no equivalence. The
original truncation of 16-bit RGB input is preserved, not redesigned.

The primary C reference uses `-ffp-contract=off` and Rust uses its normal
arithmetic (ADR 0024). The compiler-default Clang diagnostic of
`tools/diagnose_sharpyuv_numerics.py` differed in 47 of the 1,271 groups on
the local macOS host; its reports are excluded from acceptance and the
primary comparison keeps requiring exact results.
"""
import argparse
from concurrent.futures import ThreadPoolExecutor
import ctypes as C
import hashlib
import itertools
import json
import os
from pathlib import Path
import platform
import random
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]


class Matrix(C.Structure):
    _fields_ = [("y", C.c_int * 4), ("u", C.c_int * 4), ("v", C.c_int * 4)]


class Options(C.Structure):
    _fields_ = [("matrix", C.POINTER(Matrix)), ("transfer", C.c_uint)]


class ColorSpace(C.Structure):
    _fields_ = [("kr", C.c_float), ("kb", C.c_float), ("depth", C.c_int), ("range", C.c_uint)]


def worker(library):
    lib = C.CDLL(str(library.resolve()), mode=C.RTLD_LOCAL)
    lib.SharpYuvGetVersion.restype = C.c_int
    lib.SharpYuvGetConversionMatrix.argtypes = [C.c_uint]
    lib.SharpYuvGetConversionMatrix.restype = C.POINTER(Matrix)
    lib.SharpYuvComputeConversionMatrix.argtypes = [C.POINTER(ColorSpace), C.POINTER(Matrix)]
    lib.SharpYuvComputeConversionMatrix.restype = None
    lib.SharpYuvOptionsInitInternal.argtypes = [C.POINTER(Matrix), C.POINTER(Options), C.c_int]
    lib.SharpYuvOptionsInitInternal.restype = C.c_int
    lib.SharpYuvInit.argtypes = [C.c_void_p]
    lib.SharpYuvInit.restype = None
    arguments = [C.c_void_p] * 3 + [C.c_int] * 3 + [C.c_void_p, C.c_int] * 3 + [C.c_int] * 3
    lib.SharpYuvConvert.argtypes = arguments + [C.POINTER(Matrix)]
    lib.SharpYuvConvert.restype = C.c_int
    lib.SharpYuvConvertWithOptions.argtypes = arguments + [C.POINTER(Options)]
    lib.SharpYuvConvertWithOptions.restype = C.c_int
    results = {}

    def conversion(rgb, yuv, transfer, matrix_type, layout, width, height, seed, override=None):
        rng = random.Random(seed)
        unit = 1 if rgb == 8 else 2
        step = unit * (1 if layout == 2 else 4 if layout == 1 else 3)
        stride = (width * step + 6 * unit)
        span = stride * height
        raw = (C.c_uint16 * ((3 * span + 64 + 1) // 2))()
        C.memset(C.addressof(raw), 0xA5, C.sizeof(raw))
        base = C.addressof(raw) + 32
        offsets = [0, span, 2 * span] if layout == 2 else [0, unit, 2 * unit]
        sample = C.c_ubyte if unit == 1 else C.c_uint16
        for y in range(height):
            for x in range(width):
                for channel in range(3):
                    value = rng.randrange(1 << rgb)
                    if seed % 4 == 0:
                        value = ((1 << rgb) - 1) if (x + y + channel) % 2 else 0
                    sample.from_address(base + offsets[channel] + y * stride + x * step).value = value
        before = bytes(raw)
        out_unit = 1 if yuv == 8 else 2
        sizes = [(width, height), ((width + 1) // 2, (height + 1) // 2)] * 1
        sizes.append(sizes[1])
        planes = []
        pointers = []
        strides = []
        for w, h in sizes:
            s = (w + 5) * out_unit
            buf = (C.c_uint16 * ((s * h + 65) // 2))()
            C.memset(C.addressof(buf), 0xA5, C.sizeof(buf))
            planes.append(buf)
            pointers.append(C.addressof(buf) + 32)
            strides.append(s)
        matrix = lib.SharpYuvGetConversionMatrix(matrix_type)
        options = Options()
        assert lib.SharpYuvOptionsInitInternal(matrix, C.byref(options), 0x40002) == 1
        options.transfer = transfer
        values = [base + offset for offset in offsets] + [step, stride, rgb]
        for pointer, s in zip(pointers, strides):
            values += [pointer, s]
        values += [yuv, width, height]
        if override:
            for index, value in override.items():
                values[index] = value
        code = lib.SharpYuvConvertWithOptions(*values, C.byref(options))
        assert bytes(raw) == before, "Input or input guards modified"
        for plane in planes:
            assert bytes(plane)[:32] == b'\xA5' * 32, "Output prefix guard modified"
            assert bytes(plane)[-32:] == b'\xA5' * 32, "Output suffix guard modified"
        digest = hashlib.sha256(b''.join(bytes(p) for p in planes)).hexdigest()
        if not override and transfer == 13:
            for plane in planes:
                C.memset(C.addressof(plane), 0xA5, C.sizeof(plane))
            assert lib.SharpYuvConvert(*values, matrix) == code
            assert hashlib.sha256(b''.join(bytes(p) for p in planes)).hexdigest() == digest
        return [code, digest]

    # Each process starts cold; CDLL releases the GIL for real simultaneous
    # initialization. All buffers are per-call and remain live until return.
    with ThreadPoolExecutor(max_workers=8) as pool:
        cold = list(pool.map(lambda _: conversion(8, 8, 13, 0, 0, 17, 9, 42), range(24)))
    assert all(result == cold[0] for result in cold)
    results['concurrent-cold-initialization-24'] = cold
    callback_type = C.CFUNCTYPE(C.c_int, C.c_uint)
    callback = callback_type(lambda _: 0)
    for _ in range(3):
        lib.SharpYuvInit(C.cast(callback, C.c_void_p))
    lib.SharpYuvInit(None)
    results['version'] = lib.SharpYuvGetVersion()
    for index in range(5):
        matrix = lib.SharpYuvGetConversionMatrix(index)
        assert bool(matrix)
        results[f'predefined-matrix-{index}'] = list(matrix.contents.y) + list(matrix.contents.u) + list(matrix.contents.v)
        assert C.addressof(matrix.contents) == C.addressof(lib.SharpYuvGetConversionMatrix(index).contents)
    results['invalid-matrix'] = [bool(lib.SharpYuvGetConversionMatrix(i)) for i in [5, 6, 0xffffffff]]
    assert results['invalid-matrix'] == [False] * 3
    for kr, kb in [(0.299, 0.114), (0.2126, 0.0722), (0.2627, 0.0593)]:
        for depth, value_range in itertools.product([8, 10, 12], [0, 1]):
            output = Matrix()
            lib.SharpYuvComputeConversionMatrix(C.byref(ColorSpace(kr, kb, depth, value_range)), C.byref(output))
            results[f'computed-matrix-{kr}-{kb}-{depth}-{value_range}'] = list(output.y) + list(output.u) + list(output.v)
    matrix = lib.SharpYuvGetConversionMatrix(0)
    for version in [0x40002, 0x40000, 0x400ff, 0x30000, 0x1000000, 0]:
        options = Options(None, 99)
        code = lib.SharpYuvOptionsInitInternal(matrix, C.byref(options), version)
        results[f'options-version-{version}'] = [code, options.transfer, bool(options.matrix)]
    options = Options(None, 99)
    results['null-options'] = lib.SharpYuvOptionsInitInternal(matrix, None, 0x40002)
    results['null-matrix'] = lib.SharpYuvOptionsInitInternal(None, C.byref(options), 0x40002)
    assert results['null-options'] == results['null-matrix'] == 0
    transfers = [1, *range(4, 19)]
    shapes = [(1, 1), (1, 9), (9, 1), (2, 2), (3, 5), (15, 7), (16, 8), (17, 9), (33, 11)]
    for index, (rgb, yuv, transfer, matrix_type) in enumerate(itertools.product([8, 10, 12, 16], [8, 10, 12], transfers, range(5))):
        width, height = shapes[index % len(shapes)]
        name = f'convert-rgb{rgb}-yuv{yuv}-tf{transfer}-matrix{matrix_type}-layout{index % 3}-{width}x{height}'
        results[name] = conversion(rgb, yuv, transfer, matrix_type, index % 3, width, height, index)
        assert results[name][0] == 1, name
    rng = random.Random(0x41564946)
    for index in range(256):
        rgb, yuv = rng.choice([8, 10, 12, 16]), rng.choice([8, 10, 12])
        results[f'seeded-{index}'] = conversion(rgb, yuv, rng.choice(transfers), rng.randrange(5),
                                              rng.randrange(3), rng.randrange(1, 65), rng.randrange(1, 33), rng.randrange(1 << 30))
        assert results[f'seeded-{index}'][0] == 1
    # Only rejection cases with defined preceding C arithmetic are included.
    invalid = [{13: 0}, {13: -1}, {13: 0x7fffffff}, {14: 0}, {14: -1}, {14: 0x7fffffff},
               {5: 9}, {5: 15}, {12: 9}, {12: 11}, {3: 3}, {4: 3}, {7: 3}, {9: 3}, {11: 3}]
    invalid += [{index: None} for index in [0, 1, 2, 6, 8, 10]]
    for index, override in enumerate(invalid):
        results[f'reject-{index}-{override}'] = conversion(10, 10, 13, 0, 0, 3, 3, 111, override)
        assert results[f'reject-{index}-{override}'][0] == 0
    return results


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--worker', type=Path)
    parser.add_argument('--out', type=Path)
    parser.add_argument('--debug', action='store_true')
    parser.add_argument('--reference', type=Path)
    parser.add_argument('--candidate', type=Path)
    parser.add_argument('--label', help='Keep an alternate reference result separate from the primary profile')
    args = parser.parse_args()
    if args.worker:
        args.out.write_text(json.dumps(worker(args.worker), indent=2) + '\n')
        return
    suffix = '.dylib' if sys.platform == 'darwin' else '.so'
    profile = 'debug' if args.debug else 'release'
    base = Path(os.environ.get('SHARPYUV_BUILD_ROOT', ROOT / 'build'))
    reference = args.reference or base / ('sharpyuv-reference-debug' if args.debug else 'sharpyuv-reference') / ('libsharpyuv' + suffix)
    candidate = args.candidate or Path(os.environ.get('CARGO_TARGET_DIR', ROOT / 'target')) / profile / ('libsharpyuv' + suffix)
    configuration_path = reference.parent / 'reference-profile.json'
    configuration = json.loads(configuration_path.read_text()) if configuration_path.exists() else None
    if args.reference is None and (not configuration or not configuration['acceptance_reference']
                                   or configuration['c_flags'] != '-ffp-contract=off'):
        raise SystemExit('Build the primary reference with tools/build_sharpyuv.py; its numeric contract must be -ffp-contract=off')
    work = base / ('sharpyuv-parity-' + (args.label or profile))
    work.mkdir(parents=True, exist_ok=True)
    results = []
    for name, library in [('c', reference), ('rust', candidate)]:
        output = work / (name + '.json')
        subprocess.run([sys.executable, __file__, '--worker', library, '--out', output], check=True)
        results.append(json.loads(output.read_text()))
    differences = [name for name in results[0].keys() | results[1].keys() if results[0].get(name) != results[1].get(name)]
    report = {'platform': platform.platform(), 'profile': profile, 'reference': str(reference),
              'candidate': str(candidate), 'cases': len(results[0]), 'differences': differences,
              'reference_sha256': hashlib.sha256(reference.read_bytes()).hexdigest(),
              'candidate_sha256': hashlib.sha256(candidate.read_bytes()).hexdigest()}
    report['reference_configuration'] = configuration
    (work / 'summary.json').write_text(json.dumps(report, indent=2) + '\n')
    if differences:
        for name in differences[:8]:
            print(name, '\n C:', results[0].get(name), '\n Rust:', results[1].get(name))
        raise SystemExit(f'{len(differences)} SharpYUV mismatches')
    print(f'Exact SharpYUV parity: {len(results[0])} API/corpus cases, {profile}, {platform.machine()}')


if __name__ == '__main__':
    main()
