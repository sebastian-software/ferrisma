#!/usr/bin/env python3
"""Instructions per call of rebuilt dav1d kernels against the original's assembly.

Usage: python3 tools/measure_dav1d_kernels.py [--family itx|itx16|msac|lpf|cdef|ipred|lr] [--calls N]   (macOS ARM64)

For each 8 bpc inverse-transform entry point rebuilt so far (ADR 0062), the
driver `tools/dav1d-kernel-bench.c` calls the kernel N times on fixed inputs
from the original with its assembly (`build/dav1d-dsp-release-neon/
libdav1d-dsp-asm.dylib`, made by `tools/check_dav1d_dsp.py --simd neon`) and
from the port's cdylib (`target/release/libdav1d.dylib`), both at the NEON
level, and once without the call. `/usr/bin/time -l` reports the instructions
retired by each process; (call - baseline) / N is the instructions per call,
which do not depend on the load of the machine. Both libraries must leave the
same destination checksum. Results: `build/dav1d-kernel-instructions.json`.
Instruction counts are not timings; the benchmark record keeps the timings.

`--family msac`: the seven entry points of `msac.S` (several symbol counts
for the three symbol sizes), from the same oracle and from the port built
with its test-only accessors (`build/dav1d-bridges-target`, made by
`check_dav1d_dsp.py --simd neon`); `--calls` / 8 calls each.
Results: `build/dav1d-kernel-instructions-msac.json`.

`--family lpf`: the four loop filter entry points at 8 and 16 bpc (10-bit
pixels) on one superblock edge of 32 units with mixed filter widths (see
`dav1d-kernel-bench.c`); `--calls` / 256 calls each. Results:
`build/dav1d-kernel-instructions-lpf.json`.

`--family cdef`: `dir` and the three filters (`fb[0..2]`, through the
`DEFINE_FILTER` wrappers: padding and filter) at 8 and 16 bpc (10-bit
pixels), the filters with the edge flags 0xf (fully edged), 0x0, 0x5 and
0xa; `--calls` / 64 calls each. Results:
`build/dav1d-kernel-instructions-cdef.json`.

`--family ipred`: every slot of `dav1d_intra_pred_dsp_init_<bpc>bpc` that
`intra_pred_dsp_init_arm` assigns, for every block size the original's
checkasm test calls it with (`dav1d-kernel-bench.c` has the inputs): the
fourteen `intra_pred` modes (Z1 to Z3 each over the 27 test angles with the
flag sets 0, 0x400 (intra edge filter) and 0x600 (and smooth); the filter
predictor over its five filters), the four `cfl_pred` modes over four
alphas, the three `cfl_ac` layouts without padding and with the largest
padding, and `pal_pred`; `--calls` / block area calls each, at least 250,000
(a process's instruction count varies by some 10^5 between runs, which is
visible in a kernel of 100 instructions below that). `--bpc 8` or
`--bpc 16` restricts the bit depth, `--angle N` pins Z1 to Z3 to one angle
(0..90, relative to the mode). Results:
`build/dav1d-kernel-instructions-ipred.json`.

`--family lr`: the five loop restoration slots of
`dav1d_loop_restoration_dsp_init_<bpc>bpc` (`wiener[0]`, 7 taps, and
`wiener[1]`, 5 taps; `sgr[0..2]`: 5x5, 3x3, mix; the self-guided slots
through the C wrappers of `src/arm/looprestoration.h`, which the original
compiles around its row kernels) on units of 256x64 (a luma unit's stripe),
64x64, 32x32 and 16x16 pixels with the edge flags 0xf (all four edges
present) and 0x0 (every edge padded), and 64x64 also with 0x5 (left, top)
and 0xa (right, bottom); 10-bit pixels at 16 bpc; `--calls` / unit area
calls each, at least 250,000. `--bpc` as for `ipred`; `--jobs N` measures N
rows at a time (instruction counts do not depend on the load). Results:
`build/dav1d-kernel-instructions-lr.json`.

`--port PATH` measures another build of the port's cdylib (a development
copy) and `--output PATH` writes the JSON elsewhere; both for `ipred`, `lr`
and `lpf` only.

On GNU Linux x86-64 (families `msac` and `lpf`) the level is AVX2 (mask
0x2f; oracle `build/dav1d-dsp-release-avx2/libdav1d-dsp-asm.so`, made by
`check_dav1d_dsp.py --simd avx2`) and the kernels are those of
`src/x86/msac.h` and `src/x86/loopfilter.h`. The instructions are counted by
Valgrind's Callgrind there (`Collected`, the x86-64 instructions the process
executes), which needs no hardware counter and so also works where the
process runs under an emulator (Rosetta in a Linux VM on the development
Mac): the count is that of the x86-64 code, whatever executes it.
"""
import os
import sys

X86 = os.uname().machine == 'x86_64'
LEVEL = 'avx2' if X86 else 'neon'
MASK = '0x2f' if X86 else '0x1'
DSO = 'dylib' if sys.platform == 'darwin' else 'so'
MSAC = [('dav1d_msac_decode_symbol_adapt4_neon', ns) for ns in (1, 2, 3)]
MSAC += [('dav1d_msac_decode_symbol_adapt8_neon', ns) for ns in (4, 7)]
MSAC += [('dav1d_msac_decode_symbol_adapt16_neon', ns) for ns in (10, 12, 15)]
MSAC += [('dav1d_msac_decode_hi_tok_neon', 3), ('dav1d_msac_decode_bool_adapt_neon', 1),
         ('dav1d_msac_decode_bool_equi_neon', 0), ('dav1d_msac_decode_bool_neon', 0)]
if X86:
    MSAC = [('dav1d_msac_decode_symbol_adapt4_sse2', ns) for ns in (1, 2, 3)]
    MSAC += [('dav1d_msac_decode_symbol_adapt8_sse2', ns) for ns in (4, 7)]
    MSAC += [('dav1d_msac_decode_symbol_adapt16_avx2', ns) for ns in (10, 12, 15)]
    MSAC += [('dav1d_msac_decode_hi_tok_sse2', 3), ('dav1d_msac_decode_bool_adapt_sse2', 1),
             ('dav1d_msac_decode_bool_equi_sse2', 0), ('dav1d_msac_decode_bool_sse2', 0)]
import argparse
from concurrent.futures import ThreadPoolExecutor
import json
from pathlib import Path
import re
import subprocess

ROOT = Path(__file__).resolve().parents[1]
SIZES = ['4x4', '8x8', '16x16', '32x32', '64x64', '4x8', '8x4', '8x16', '16x8', '16x32', '32x16', '32x64',
         '64x32', '4x16', '16x4', '8x32', '32x8', '16x64', '64x16']
TYPES = ['DCT_DCT', 'ADST_DCT', 'DCT_ADST', 'ADST_ADST', 'FLIPADST_DCT', 'DCT_FLIPADST', 'FLIPADST_FLIPADST',
         'ADST_FLIPADST', 'FLIPADST_ADST', 'IDTX', 'V_DCT', 'H_DCT', 'V_ADST', 'H_ADST', 'V_FLIPADST',
         'H_FLIPADST', 'WHT_WHT']


def rebuilt_kernels(bpc=8):
    """(name, size, tx index, type index) of every slot simd/arm/itx.rs assigns at BPC."""
    text = (ROOT / 'crates/dav1d/src/simd/arm/itx.rs').read_text()
    out = []
    for size, txtype, name in re.findall(r't\[R?TX_(\w+)\]\[(\w+)\] = Some\((dav1d_inv_txfm_add_\w+_%dbpc_neon)\)' % bpc,
                                         text):
        size = size.lower()
        out.append((name, size, SIZES.index(size), TYPES.index(txtype)))
    return out


INTRA_MODES = ['dc', 'v', 'h', 'dc_left', 'dc_top', 'dc_128', 'z1', 'z2', 'z3', 'smooth', 'smooth_v', 'smooth_h',
               'paeth', 'filter']


def block_sizes(max_w, max_h):
    """The (w, h) the original's checkasm test uses: h from w / 4 to w * 4."""
    return [(w, h) for w in (4, 8, 16, 32, 64) if w <= max_w
            for h in (4, 8, 16, 32, 64) if max(w // 4, 4) <= h <= min(w * 4, max_h)]


def ipred_rows(bpc):
    """(kernel label, path label, slot, w, h, param) of every measured call set."""
    rows = []
    for slot, mode in enumerate(INTRA_MODES):
        limit = 32 if mode == 'filter' else 64
        for w, h in block_sizes(limit, limit):
            for flags in ((0, 0x400, 0x600) if mode in ('z1', 'z2', 'z3') else (0,)):
                path = f'flags {flags:#x}' if mode in ('z1', 'z2', 'z3') else '-'
                rows.append((f'ipred {mode} {w}x{h} {bpc}bpc', path, slot, w, h, flags))
    for layout, (name, max_w, max_h) in enumerate([('420', 16, 16), ('422', 16, 32), ('444', 32, 32)]):
        for w, h in block_sizes(max_w, max_h):
            for pad in (0, 1):
                rows.append((f'cfl_ac {name} {w}x{h} {bpc}bpc', 'max pad' if pad else 'no pad', 14 + layout, w, h, pad))
    for index, name in ((0, 'dc'), (3, 'left'), (4, 'top'), (5, '128')):
        for w, h in block_sizes(32, 32):
            rows.append((f'cfl_pred {name} {w}x{h} {bpc}bpc', '-', 17 + index, w, h, 0))
    for w, h in block_sizes(64, 64):
        rows.append((f'pal_pred {w}x{h} {bpc}bpc', '-', 23, w, h, 0))
    return rows


LR_SLOTS = ['wiener7', 'wiener5', 'sgr_5x5', 'sgr_3x3', 'sgr_mix']
LR_SIZES = [(256, 64), (64, 64), (32, 32), (16, 16)]


def lr_rows(bpc):
    """(kernel label, path label, slot, w, h, edges) of every measured call set."""
    rows = []
    for slot, name in enumerate(LR_SLOTS):
        for w, h in LR_SIZES:
            for edges in ((0xf, 0x0, 0x5, 0xa) if (w, h) == (64, 64) else (0xf, 0x0)):
                rows.append((f'lr {name} {w}x{h} {bpc}bpc', f'edges {edges:#x}', slot, w, h, edges))
    return rows


def instructions(command):
    if sys.platform != 'darwin':
        result = subprocess.run(['valgrind', '--tool=callgrind', '--callgrind-out-file=/dev/null', *command],
                                capture_output=True, text=True, check=True)
        count = int(re.search(r'Collected : (\d+)', result.stderr).group(1))
        return count, re.search(r'checksum=(\w+)', result.stdout).group(1)
    result = subprocess.run(['/usr/bin/time', '-l', *command], capture_output=True, text=True, check=True)
    count = int(re.search(r'(\d+)\s+instructions retired', result.stderr).group(1))
    checksum = re.search(r'checksum=(\w+)', result.stdout).group(1)
    return count, checksum


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--calls', type=int, default=32000000,
                        help='pixels processed per kernel and side (calls = this / block area)')
    parser.add_argument('--only', help='substring filter on kernel names')
    parser.add_argument('--family', choices=['itx', 'itx16', 'msac', 'lpf', 'cdef', 'ipred', 'lr'], default='itx')
    parser.add_argument('--bpc', type=int, choices=[8, 16], help='ipred, lr: only this bit depth')
    parser.add_argument('--jobs', type=int, default=1, help='lr: rows measured at a time')
    parser.add_argument('--angle', type=int, default=-1, help='ipred: one angle (0..90) for Z1 to Z3')
    parser.add_argument('--port', type=Path, help='ipred, lr, lpf: the port cdylib to measure')
    parser.add_argument('--output', type=Path, help='ipred, lr, lpf: where to write the JSON')
    args = parser.parse_args()
    if X86 and args.family not in ('msac', 'lpf'):
        raise SystemExit(f'--family {args.family} has no x86-64 kernels yet')
    work = ROOT / 'build/dav1d-kernel-bench'
    work.mkdir(parents=True, exist_ok=True)
    driver = work / 'dav1d-kernel-bench'
    subprocess.run(['cc', '-O2', '-o', driver, ROOT / 'tools/dav1d-kernel-bench.c'], check=True)
    original = ROOT / f'build/dav1d-dsp-release-{LEVEL}/libdav1d-dsp-asm.{DSO}'
    port = ROOT.parents[1] / f'target/release/libdav1d.{DSO}'
    rows = []
    if args.family == 'ipred':
        port = args.port or port
        for bpc in ([args.bpc] if args.bpc else [8, 16]):
            for kernel, path, slot, w, h, param in ipred_rows(bpc):
                if args.only and args.only not in kernel:
                    continue
                iterations = max(250000, args.calls // (w * h))
                per_call, checksums = {}, {}
                for side, library in (('original', original), ('port', port)):
                    command = [driver, library, '0x1', 'ipred', str(bpc), str(slot), str(w), str(h), hex(param),
                               str(args.angle), str(iterations)]
                    base, _ = instructions(command + ['baseline'])
                    result = subprocess.run(['/usr/bin/time', '-l', *command, 'call'], capture_output=True, text=True,
                                            check=True)
                    total = int(re.search(r'(\d+)\s+instructions retired', result.stderr).group(1))
                    calls = int(re.search(r'calls=(\d+)', result.stdout).group(1))
                    per_call[side] = (total - base) / calls
                    checksums[side] = re.search(r'checksum=(\w+)', result.stdout).group(1)
                row = {'kernel': kernel, 'path': path, 'calls': calls,
                       'original_instructions': round(per_call['original'], 1),
                       'port_instructions': round(per_call['port'], 1),
                       'ratio': round(per_call['port'] / per_call['original'], 3),
                       'identical_output': checksums['original'] == checksums['port']}
                rows.append(row)
                print(f"{row['kernel']:52s} {row['path']:16s} {row['original_instructions']:9.1f} "
                      f"{row['port_instructions']:9.1f} {row['ratio']:6.3f} {row['identical_output']}", flush=True)
        (args.output or ROOT / 'build/dav1d-kernel-instructions-ipred.json').write_text(json.dumps(
            {'pixels_per_kernel': args.calls, 'level': 'neon', 'original': str(original), 'port': str(port),
             'kernels': rows}, indent=2) + '\n')
        if not all(row['identical_output'] for row in rows):
            raise SystemExit('outputs differ')
        return
    if args.family == 'lr':
        port = args.port or port

        def measure(job):
            kernel, path, slot, w, h, edges = job
            iterations = max(250000, args.calls // (w * h))
            per_call, checksums = {}, {}
            for side, library in (('original', original), ('port', port)):
                command = [driver, library, '0x1', 'lr', kernel.split()[-1][:-3], str(slot), str(w), str(h),
                           hex(edges), str(iterations)]
                base, _ = instructions(command + ['baseline'])
                total, checksums[side] = instructions(command + ['call'])
                per_call[side] = (total - base) / iterations
            return {'kernel': kernel, 'path': path, 'calls': iterations,
                    'original_instructions': round(per_call['original'], 1),
                    'port_instructions': round(per_call['port'], 1),
                    'ratio': round(per_call['port'] / per_call['original'], 3),
                    'identical_output': checksums['original'] == checksums['port']}

        jobs = [job for bpc in ([args.bpc] if args.bpc else [8, 16]) for job in lr_rows(bpc)
                if not args.only or args.only in job[0]]
        with ThreadPoolExecutor(max_workers=max(1, args.jobs)) as pool:
            for row in pool.map(measure, jobs):
                rows.append(row)
                print(f"{row['kernel']:52s} {row['path']:16s} {row['original_instructions']:11.1f} "
                      f"{row['port_instructions']:11.1f} {row['ratio']:6.3f} {row['identical_output']}", flush=True)
        (args.output or ROOT / 'build/dav1d-kernel-instructions-lr.json').write_text(json.dumps(
            {'pixels_per_kernel': args.calls, 'level': 'neon', 'original': str(original), 'port': str(port),
             'kernels': rows}, indent=2) + '\n')
        if not all(row['identical_output'] for row in rows):
            raise SystemExit('outputs differ')
        return
    if args.family == 'cdef':
        calls = args.calls // 64
        slots = [('dir', 0, ['0xf']), ('fb[0] 8x8', 1, None), ('fb[1] 4x8', 2, None), ('fb[2] 4x4', 3, None)]
        for bpc in (8, 16):
            for name, slot, edge_sets in slots:
                kernel = f'cdef {name} {bpc}bpc'
                if args.only and args.only not in kernel:
                    continue
                for edges in edge_sets or ['0xf', '0x0', '0x5', '0xa']:
                    per_call, checksums = {}, {}
                    for side, library in (('original', original), ('port', port)):
                        command = [driver, library, '0x1', 'cdef', str(bpc), str(slot), edges, str(calls)]
                        base, _ = instructions(command + ['baseline'])
                        total, checksum = instructions(command + ['call'])
                        per_call[side] = (total - base) / calls
                        checksums[side] = checksum
                    row = {'kernel': kernel, 'path': '-' if slot == 0 else f'edges {edges}', 'calls': calls,
                           'original_instructions': round(per_call['original'], 1),
                           'port_instructions': round(per_call['port'], 1),
                           'ratio': round(per_call['port'] / per_call['original'], 3),
                           'identical_output': checksums['original'] == checksums['port']}
                    rows.append(row)
                    print(f"{row['kernel']:52s} {row['path']:16s} {row['original_instructions']:9.1f} "
                          f"{row['port_instructions']:9.1f} {row['ratio']:6.3f} {row['identical_output']}", flush=True)
        (ROOT / 'build/dav1d-kernel-instructions-cdef.json').write_text(json.dumps(
            {'calls': calls, 'level': 'neon', 'original': str(original), 'port': str(port), 'kernels': rows},
            indent=2) + '\n')
        if not all(row['identical_output'] for row in rows):
            raise SystemExit('outputs differ')
        return
    if args.family == 'lpf':
        port = args.port or port
        calls = args.calls // 256
        names = ['h_sb_y', 'v_sb_y', 'h_sb_uv', 'v_sb_uv']
        for bpc in (8, 16):
            for slot, name in enumerate(names):
                kernel = f'dav1d_lpf_{name}_{bpc}bpc_{LEVEL}'
                if args.only and args.only not in kernel:
                    continue
                per_call, checksums = {}, {}
                for side, library in (('original', original), ('port', port)):
                    command = [driver, library, MASK, 'lpf', str(bpc), str(slot), str(calls)]
                    base, _ = instructions(command + ['baseline'])
                    total, checksum = instructions(command + ['call'])
                    per_call[side] = (total - base) / calls
                    checksums[side] = checksum
                row = {'kernel': kernel, 'path': '32 units', 'calls': calls,
                       'original_instructions': round(per_call['original'], 1),
                       'port_instructions': round(per_call['port'], 1),
                       'ratio': round(per_call['port'] / per_call['original'], 3),
                       'identical_output': checksums['original'] == checksums['port']}
                rows.append(row)
                print(f"{row['kernel']:52s} {row['path']:16s} {row['original_instructions']:9.1f} "
                      f"{row['port_instructions']:9.1f} {row['ratio']:6.3f} {row['identical_output']}", flush=True)
        (args.output or ROOT / 'build/dav1d-kernel-instructions-lpf.json').write_text(json.dumps(
            {'calls': calls, 'level': LEVEL, 'original': str(original), 'port': str(port), 'kernels': rows},
            indent=2) + '\n')
        if not all(row['identical_output'] for row in rows):
            raise SystemExit('outputs differ')
        return
    if args.family == 'msac':
        port = ROOT / f'build/dav1d-bridges-target/release/libdav1d.{DSO}'
        calls = args.calls // 8
        for name, ns in MSAC:
            if args.only and args.only not in name:
                continue
            per_call, checksums = {}, {}
            for side, library in (('original', original), ('port', port)):
                command = [driver, library, MASK, 'msac', name, str(ns), str(calls)]
                base, _ = instructions(command + ['baseline'])
                total, checksum = instructions(command + ['call'])
                per_call[side] = (total - base) / calls
                checksums[side] = checksum
            row = {'kernel': name, 'path': f'n_symbols {ns}' if 'symbol' in name else 'adapt' if 'adapt' in name
                   or 'hi_tok' in name else '-', 'calls': calls,
                   'original_instructions': round(per_call['original'], 1),
                   'port_instructions': round(per_call['port'], 1),
                   'ratio': round(per_call['port'] / per_call['original'], 3),
                   'identical_output': checksums['original'] == checksums['port']}
            rows.append(row)
            print(f"{row['kernel']:52s} {row['path']:16s} {row['original_instructions']:9.1f} "
                  f"{row['port_instructions']:9.1f} {row['ratio']:6.3f} {row['identical_output']}", flush=True)
        (ROOT / 'build/dav1d-kernel-instructions-msac.json').write_text(json.dumps(
            {'calls': calls, 'level': LEVEL, 'original': str(original), 'port': str(port), 'kernels': rows},
            indent=2) + '\n')
        if not all(row['identical_output'] for row in rows):
            raise SystemExit('outputs differ')
        return
    sixteen = args.family == 'itx16'
    for name, size, tx, txtype in rebuilt_kernels(16 if sixteen else 8):
        if args.only and args.only not in name:
            continue
        w, h = map(int, size.split('x'))
        iterations = max(2000, args.calls // (w * h))
        # The largest eob a stream can code: the last coefficient index.
        last = min(w, 32) * min(h, 32) - 1
        for eob in ([0, last] if name.startswith('dav1d_inv_txfm_add_dct_dct') else [last]):
            per_call, checksums = {}, {}
            for side, library in (('original', original), ('port', port)):
                command = [driver, library, '0x1', *(['itx16'] if sixteen else []), str(tx), str(txtype), str(eob),
                           str(iterations)]
                base, _ = instructions(command + ['baseline'])
                total, checksum = instructions(command + ['call'])
                per_call[side] = (total - base) / iterations
                checksums[side] = checksum
            row = {'kernel': name, 'path': 'dc only (eob 0)' if eob == 0 else 'full (last eob)',
                   'calls': iterations,
                   'original_instructions': round(per_call['original'], 1),
                   'port_instructions': round(per_call['port'], 1),
                   'ratio': round(per_call['port'] / per_call['original'], 3),
                   'identical_output': checksums['original'] == checksums['port']}
            rows.append(row)
            print(f"{row['kernel']:52s} {row['path']:16s} {row['original_instructions']:9.1f} "
                  f"{row['port_instructions']:9.1f} {row['ratio']:6.3f} {row['identical_output']}", flush=True)
    (ROOT / ('build/dav1d-kernel-instructions' + ('-itx16' if sixteen else '') + '.json')).write_text(json.dumps(
        {'pixels_per_kernel': args.calls, 'level': 'neon', 'original': str(original), 'port': str(port),
         'kernels': rows}, indent=2) + '\n')
    if not all(row['identical_output'] for row in rows):
        raise SystemExit('outputs differ')


if __name__ == '__main__':
    main()
