#!/usr/bin/env python3
"""Run original checkasm bodies across the original's and the port's pointers.

Without `--simd` (ADR 0029): the original portable C against the port's
scalar path. With `--simd neon` or `--simd avx2` (ADR 0062): the original
built with its assembly (`tools/build_dav1d.py --asm`) against the port's
intrinsics kernels, both detecting the CPU and restricted to the same level
with `dav1d_set_cpu_flags_mask`; every DSP pointer either side selects is
compared, kernels the port has not rebuilt yet through its C. The test bodies
are compiled with the portable configuration in both modes.

msac has no DSP table: the original selects its assembly at compile time
(`src/arm/msac.h`) and its checkasm test takes the assembly functions by
name under `HAVE_ASM`. With `--simd neon` that branch of the unchanged test
body is enabled (the two textual edits are in `MSAC_SIMD_EDITS`), and the
seven `_neon` names resolve to the original's hidden assembly functions and
to the port's rebuilt kernels through test-only accessors; the port is then
built with its `port-test-bridges` feature (accessors only) into
`build/dav1d-bridges-target`.

The original checkasm framework also needs NASM on GNU Linux x86-64. That
assembler builds test infrastructure only; codec assembly stays disabled.
ADR 0029 records the test-only binding bridge and native helpers.
"""
import argparse
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys

from build_dav1d import copy_inputs

ROOT = Path(__file__).resolve().parents[1]
# dav1d_set_cpu_flags_mask values of each SIMD level (src/arm/cpu.h,
# src/x86/cpu.h): NEON alone (no DotProd/I8MM/SVE); SSE2, SSSE3, SSE4.1 and
# AVX2 without AVX-512, with SLOW_GATHER passed through as detected.
SIMD_MASKS = {'neon': 0x1, 'avx2': 0x2f}
# The seven msac entry points of src/arm/msac.h, with their C types.
MSAC_NEON = {
    'dav1d_msac_decode_symbol_adapt4_neon': 'unsigned (*)(MsacContext *, uint16_t *, size_t)',
    'dav1d_msac_decode_symbol_adapt8_neon': 'unsigned (*)(MsacContext *, uint16_t *, size_t)',
    'dav1d_msac_decode_symbol_adapt16_neon': 'unsigned (*)(MsacContext *, uint16_t *, size_t)',
    'dav1d_msac_decode_hi_tok_neon': 'unsigned (*)(MsacContext *, uint16_t *)',
    'dav1d_msac_decode_bool_adapt_neon': 'unsigned (*)(MsacContext *, uint16_t *)',
    'dav1d_msac_decode_bool_equi_neon': 'unsigned (*)(MsacContext *)',
    'dav1d_msac_decode_bool_neon': 'unsigned (*)(MsacContext *, unsigned)',
}
# The test body's assembly branch, enabled at the NEON level: HAVE_ASM is 0
# in the portable configuration the bodies are compiled with, and the CPU
# flags are those of the library under test.
MSAC_SIMD_EDITS = [('#if (ARCH_AARCH64 || ARCH_ARM) && HAVE_ASM', '#if (ARCH_AARCH64 || ARCH_ARM)'),
                   ('if (dav1d_get_cpu_flags() & DAV1D_ARM_CPU_FLAG_NEON) {',
                    'if (port_cpu_level() & DAV1D_ARM_CPU_FLAG_NEON) {')]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--debug', action='store_true')
    parser.add_argument('--simd', choices=sorted(SIMD_MASKS))
    args = parser.parse_args()
    profile = 'debug' if args.debug else 'release'
    reference = ROOT / f'build/dav1d-reference-{profile}'
    oracle = ROOT / f'build/dav1d-asm-{profile}' if args.simd else None
    source = ROOT / 'build/dav1d-native-source'
    framework = ROOT / 'build/dav1d-checkasm-native'
    inputs_record = json.loads((ROOT / 'provenance/dav1d-test-inputs.json').read_text())
    copy_inputs(inputs_record['checkasm'], source / 'subprojects/checkasm')
    meson = shutil.which('meson') or str(ROOT / 'build/dav1d-tools/bin/meson')
    setup = [meson, 'setup', framework, source / 'subprojects/checkasm',
             '-Ddocs=disabled', '-Dtests=disabled', '-Ddefault_library=static', '-Dc_args=-ffp-contract=off']
    if (framework / 'build.ninja').exists():
        setup.append('--reconfigure')
    subprocess.run([str(item) for item in setup], check=True)
    subprocess.run(['ninja', '-C', framework], check=True)
    work = ROOT / (f'build/dav1d-dsp-{profile}' + (f'-{args.simd}' if args.simd else ''))
    work.mkdir(exist_ok=True)
    darwin = sys.platform == 'darwin'
    library = reference / ('src/libdav1d.7.dylib' if darwin else 'src/libdav1d.so.7.0.0')
    candidate = ROOT.parents[1] / 'target' / ('debug' if args.debug else 'release') / ('libdav1d.dylib' if darwin else 'libdav1d.so')
    msac_simd = args.simd == 'neon'
    if msac_simd:
        bridges = ROOT / 'build/dav1d-bridges-target'
        subprocess.run(['cargo', 'build', '-p', 'libavif-dav1d', '--locked', '--features', 'port-test-bridges',
                        '--target-dir', bridges, *([] if args.debug else ['--release'])], check=True, cwd=ROOT)
        candidate = bridges / ('debug' if args.debug else 'release') / ('libdav1d.dylib' if darwin else 'libdav1d.so')
    symbol_command = ['nm', '-gU', library] if darwin else ['nm', '-D', '--defined-only', library]
    symbols = {line.split()[-1].removeprefix('_') for line in subprocess.check_output(symbol_command, text=True).splitlines() if line.strip()}
    if oracle:
        # The original with its assembly: every decoder object of its static
        # archive, linked whole into the reference DSO.
        archive = oracle / 'src/libdav1d.a'
        profile_record = json.loads((oracle / 'reference-profile.json').read_text())
        if not profile_record.get('assembly'):
            raise SystemExit(f'Not an assembly oracle: {oracle}')
        listing = subprocess.check_output(['nm', '-gU' if darwin else '--defined-only', archive], text=True)
        oracle_symbols = {line.split()[-1].removeprefix('_') for line in listing.splitlines()
                          if len(line.split()) == 3 and line.split()[1] in 'TDBSR'}
        missing = {name for name in symbols if name.startswith('dav1d_')} - oracle_symbols
        if missing:
            raise SystemExit(f'Assembly oracle lacks portable symbols: {sorted(missing)[:5]}')
    flags = ['cc', '-std=c99', '-O2', '-g', '-ffp-contract=off', '-DCHECKASM_HAVE_GENERIC=1', '-D_GNU_SOURCE']
    for directory in [reference, source, source / 'include', source / 'include/dav1d',
                      ROOT / 'build/deps/checkasm/include', framework / 'include', ROOT / 'tools']:
        flags += ['-I', str(directory)]
    # Upstream deliberately hides these constant tables. Expose their addresses
    # in a test-only DSO assembled from the same 51 original reference objects.
    tables = ['dav1d_block_dimensions', 'dav1d_scans', 'dav1d_sgr_params',
              'dav1d_tx_type_class', 'dav1d_txfm_dimensions']
    accessor = work / 'table-accessors.c'
    accessor.write_text('#include "config.h"\n#include "src/tables.h"\n#include "src/scan.h"\n#include "src/cpu.h"\n' +
        '\n'.join(f'void *port_table_{name}(void) {{ return (void *)&{name}; }}' for name in tables) + '\n' +
        'unsigned port_cpu_flags(void) { return dav1d_get_cpu_flags(); }\n' +
        (''.join(f'extern void {name}(void);\nvoid *port_table_{name}(void) {{ return (void *)&{name}; }}\n'
                 for name in MSAC_NEON) if msac_simd and oracle else ''))
    accessor_object = work / 'table-accessors.o'
    subprocess.run(flags + ['-c', accessor, '-o', accessor_object], check=True)
    units = [entry for entry in json.loads((reference / 'compile_commands.json').read_text())
             if entry['output'].startswith('src/libdav1d')]
    library = work / ('libdav1d-dsp-reference.dylib' if darwin else 'libdav1d-dsp-reference.so')
    if not darwin:
        # Every original reference object is PIC; match it in the accessor.
        subprocess.run(flags + ['-fPIC', '-c', accessor, '-o', accessor_object], check=True)
    if oracle:
        library = work / ('libdav1d-dsp-asm.dylib' if darwin else 'libdav1d-dsp-asm.so')
        whole = ['-Wl,-force_load', str(archive)] if darwin else ['-Wl,--whole-archive', str(archive), '-Wl,--no-whole-archive']
        subprocess.run(['cc', '-dynamiclib' if darwin else '-shared', '-o', library, accessor_object, *whole,
                        *([] if darwin else ['-lpthread', '-lm'])], check=True)
    else:
        subprocess.run(['cc', '-dynamiclib' if darwin else '-shared', '-o', library, accessor_object,
                        *[reference / unit['output'] for unit in units]], check=True)
    symbols.update(tables)
    objects = []
    templates = ['cdef', 'filmgrain', 'ipred', 'itx', 'loopfilter', 'looprestoration', 'mc']
    inputs = [(name, None) for name in ['msac', 'pal', 'refmvs']]
    inputs += [(name, depth) for depth in [8, 16] for name in templates]
    inventory = []
    for name, depth in inputs:
        original = source / f'tests/checkasm/{name}.c'
        text = original.read_text()
        extra = []
        if name == 'msac' and msac_simd:
            for old, new in MSAC_SIMD_EDITS:
                if text.count(old) != 1:
                    raise SystemExit(f'Original msac test differs: {old}')
                text = text.replace(old, new)
            original = work / 'msac-neon.c'
            original.write_text(text)
            extra = [f'#define {symbol} (*({kind})port_lookup("{symbol}"))' for symbol, kind in MSAC_NEON.items()]
        declarations = [line for line in re.findall(r'^#include[^\n]+', text, re.M) if not line.endswith('.c"')]
        macros = []
        for symbol in sorted(set(re.findall(r'\bdav1d_\w+', text))):
            if symbol not in symbols and depth is not None:
                symbol += f'_{depth}bpc'
            if symbol in symbols:
                macros.append(f'#define {symbol} (*(__typeof__(&{symbol}))port_lookup("{symbol}"))')
        header = work / f'{name}-{depth}.h'
        header.write_text('\n'.join(declarations + ['#include "dav1d-checkasm-runtime.h"'] + macros + extra) + '\n')
        output = work / f'{name}-{depth}.o'
        command = flags + ([f'-DBITDEPTH={depth}'] if depth is not None else [])
        subprocess.run(command + ['-include', header, '-c', original, '-o', output], check=True)
        objects.append(output)
        inventory.append({'source': f'upstream/dav1d/tests/checkasm/{name}.c', 'bitdepth': depth,
                          'implementation_bindings': macros})
    driver = (source / 'tests/checkasm/checkasm.c').read_text()
    replacements = [('dav1d_set_cpu_flags_mask((unsigned) flags);', 'port_set_phase(flags);'),
                    ('.cpu_flags      = flags,', '.cpu_flags      = port_implementations,'),
                    ('cfg.cpu = dav1d_get_cpu_flags();', 'cfg.cpu = 1;')]
    for old, new in replacements:
        if driver.count(old) != 1:
            raise SystemExit(f'Original checkasm driver differs: {old}')
        driver = driver.replace(old, new)
    driver_source = work / 'driver.c'
    driver_source.write_text(driver)
    for input_file in [driver_source, ROOT / 'tools/dav1d-checkasm-runtime.c']:
        output = work / (input_file.stem + '.o')
        subprocess.run(flags + ['-include', ROOT / 'tools/dav1d-checkasm-runtime.h',
                                '-c', input_file, '-o', output], check=True)
        objects.append(output)
    executable = work / 'checkasm'
    subprocess.run(['cc', '-o', executable, *objects, framework / 'src/libcheckasm.a',
                    library, '-lm', *([] if darwin else ['-ldl', '-lpthread'])], check=True)
    environment = os.environ.copy()
    environment.update(DAV1D_REFERENCE=str(library), DAV1D_CANDIDATE=str(candidate))
    environment.pop('DAV1D_PORT_SIMD_MASK', None)
    if args.simd:
        environment['DAV1D_PORT_SIMD_MASK'] = hex(SIMD_MASKS[args.simd])
    result = subprocess.run([executable, '42'], env=environment, text=True,
                            stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
    (work / 'results.log').write_text(result.stdout)
    selections = re.findall(r'\[port\] checked C=(\d+) Rust=(\d+) function selections', result.stdout)
    unsupported = re.findall(r'\[port\] null C=(\d+) Rust=(\d+) unsupported selections', result.stdout)
    if result.returncode or len(selections) != 1 or selections[0][0] != selections[0][1] or int(selections[0][0]) == 0:
        raise SystemExit(f'Original DSP comparison failed: inspect {work / "results.log"}')
    if len(unsupported) != 1 or unsupported[0][0] != unsupported[0][1]:
        raise SystemExit('Unsupported DSP selection inventories differ')
    levels = dict(re.findall(r'\[port\] phase=(\d) cpu_flags=(0x[0-9a-f]+)', result.stdout))
    expected = hex(SIMD_MASKS[args.simd]) if args.simd else '0x0'
    if args.simd:
        # Both sides at the same level, and the level fully available here.
        if levels.get('0') != levels.get('1') or int(levels.get('1', '0'), 16) & 0xf != SIMD_MASKS[args.simd] & 0xf:
            raise SystemExit(f'CPU levels differ or are unavailable: {levels} (expected {expected})')
    elif levels != {'0': '0x0', '1': '0x0'}:
        raise SystemExit(f'Scalar comparison ran with CPU flags: {levels}')
    (work / 'inventory.json').write_text(json.dumps({'original_test_instantiations': inventory,
        'source_groups': len(inputs), 'function_selections_each': int(selections[0][0]),
        'unsupported_selections_each': int(unsupported[0][0]),
        'seed': 42, 'reference': str(library), 'candidate': str(candidate),
        'simd_level': args.simd, 'cpu_flags': levels}, indent=2) + '\n')
    print(f'Passed {len(inputs)} original DSP test groups at level {args.simd or "scalar"} '
          f'with {selections[0][0]} original/Rust function selections each')


if __name__ == '__main__':
    main()
