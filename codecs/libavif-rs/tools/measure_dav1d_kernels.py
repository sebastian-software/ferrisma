#!/usr/bin/env python3
"""Instructions per call of rebuilt dav1d kernels against the original's assembly.

Usage: python3 tools/measure_dav1d_kernels.py [--iterations N]   (macOS ARM64)

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
"""
import argparse
import json
from pathlib import Path
import re
import subprocess

ROOT = Path(__file__).resolve().parents[1]
TX = {'4x4': 0, '8x8': 1, '16x16': 2, '4x8': 5, '8x4': 6, '8x16': 7, '16x8': 8, '4x16': 13, '16x4': 14}
TYPES = {'dct_dct': 0, 'identity_identity': 9}


def instructions(command):
    result = subprocess.run(['/usr/bin/time', '-l', *command], capture_output=True, text=True, check=True)
    count = int(re.search(r'(\d+)\s+instructions retired', result.stderr).group(1))
    checksum = re.search(r'checksum=(\w+)', result.stdout).group(1)
    return count, checksum


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--iterations', type=int, default=1000000)
    args = parser.parse_args()
    work = ROOT / 'build/dav1d-kernel-bench'
    work.mkdir(parents=True, exist_ok=True)
    driver = work / 'dav1d-kernel-bench'
    subprocess.run(['cc', '-O2', '-o', driver, ROOT / 'tools/dav1d-kernel-bench.c'], check=True)
    original = ROOT / 'build/dav1d-dsp-release-neon/libdav1d-dsp-asm.dylib'
    port = ROOT.parents[1] / 'target/release/libdav1d.dylib'
    rows = []
    for name, tx in TX.items():
        for kind, txtype in TYPES.items():
            for eob in ([0, 1 << 20] if kind == 'dct_dct' else [1 << 20]):
                per_call = {}
                checksums = {}
                for side, library in (('original', original), ('port', port)):
                    base, _ = instructions([driver, library, '0x1', str(tx), str(txtype), str(eob),
                                            str(args.iterations), 'baseline'])
                    total, checksum = instructions([driver, library, '0x1', str(tx), str(txtype), str(eob),
                                                    str(args.iterations), 'call'])
                    per_call[side] = (total - base) / args.iterations
                    checksums[side] = checksum
                rows.append({'kernel': f'inv_txfm_add_{kind}_{name}_8bpc_neon',
                             'path': 'dc only (eob 0)' if eob == 0 else 'full (eob max)',
                             'original_instructions': round(per_call['original'], 1),
                             'port_instructions': round(per_call['port'], 1),
                             'ratio': round(per_call['port'] / per_call['original'], 3),
                             'identical_output': checksums['original'] == checksums['port']})
                row = rows[-1]
                print(f"{row['kernel']:52s} {row['path']:16s} {row['original_instructions']:9.1f} "
                      f"{row['port_instructions']:9.1f} {row['ratio']:6.3f} {row['identical_output']}")
    (ROOT / 'build/dav1d-kernel-instructions.json').write_text(json.dumps(
        {'iterations': args.iterations, 'level': 'neon', 'original': str(original), 'port': str(port),
         'kernels': rows}, indent=2) + '\n')
    if not all(row['identical_output'] for row in rows):
        raise SystemExit('outputs differ')


if __name__ == '__main__':
    main()
