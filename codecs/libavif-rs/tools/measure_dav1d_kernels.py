#!/usr/bin/env python3
"""Instructions per call of rebuilt dav1d kernels against the original's assembly.

Usage: python3 tools/measure_dav1d_kernels.py [--family itx|itx16|msac|lpf] [--calls N]   (macOS ARM64)

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
"""
MSAC = [('dav1d_msac_decode_symbol_adapt4_neon', ns) for ns in (1, 2, 3)]
MSAC += [('dav1d_msac_decode_symbol_adapt8_neon', ns) for ns in (4, 7)]
MSAC += [('dav1d_msac_decode_symbol_adapt16_neon', ns) for ns in (10, 12, 15)]
MSAC += [('dav1d_msac_decode_hi_tok_neon', 3), ('dav1d_msac_decode_bool_adapt_neon', 1),
         ('dav1d_msac_decode_bool_equi_neon', 0), ('dav1d_msac_decode_bool_neon', 0)]
import argparse
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


def instructions(command):
    result = subprocess.run(['/usr/bin/time', '-l', *command], capture_output=True, text=True, check=True)
    count = int(re.search(r'(\d+)\s+instructions retired', result.stderr).group(1))
    checksum = re.search(r'checksum=(\w+)', result.stdout).group(1)
    return count, checksum


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--calls', type=int, default=32000000,
                        help='pixels processed per kernel and side (calls = this / block area)')
    parser.add_argument('--only', help='substring filter on kernel names')
    parser.add_argument('--family', choices=['itx', 'itx16', 'msac', 'lpf'], default='itx')
    args = parser.parse_args()
    work = ROOT / 'build/dav1d-kernel-bench'
    work.mkdir(parents=True, exist_ok=True)
    driver = work / 'dav1d-kernel-bench'
    subprocess.run(['cc', '-O2', '-o', driver, ROOT / 'tools/dav1d-kernel-bench.c'], check=True)
    original = ROOT / 'build/dav1d-dsp-release-neon/libdav1d-dsp-asm.dylib'
    port = ROOT.parents[1] / 'target/release/libdav1d.dylib'
    rows = []
    if args.family == 'lpf':
        calls = args.calls // 256
        names = ['h_sb_y', 'v_sb_y', 'h_sb_uv', 'v_sb_uv']
        for bpc in (8, 16):
            for slot, name in enumerate(names):
                kernel = f'dav1d_lpf_{name}_{bpc}bpc_neon'
                if args.only and args.only not in kernel:
                    continue
                per_call, checksums = {}, {}
                for side, library in (('original', original), ('port', port)):
                    command = [driver, library, '0x1', 'lpf', str(bpc), str(slot), str(calls)]
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
        (ROOT / 'build/dav1d-kernel-instructions-lpf.json').write_text(json.dumps(
            {'calls': calls, 'level': 'neon', 'original': str(original), 'port': str(port), 'kernels': rows},
            indent=2) + '\n')
        if not all(row['identical_output'] for row in rows):
            raise SystemExit('outputs differ')
        return
    if args.family == 'msac':
        port = ROOT / 'build/dav1d-bridges-target/release/libdav1d.dylib'
        calls = args.calls // 8
        for name, ns in MSAC:
            if args.only and args.only not in name:
                continue
            per_call, checksums = {}, {}
            for side, library in (('original', original), ('port', port)):
                command = [driver, library, '0x1', 'msac', name, str(ns), str(calls)]
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
            {'calls': calls, 'level': 'neon', 'original': str(original), 'port': str(port), 'kernels': rows},
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
