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
(`src/arm/msac.h`, `src/x86/msac.h`) and its checkasm test takes the assembly
functions by name under `HAVE_ASM`. With `--simd` that branch of the
unchanged test body is enabled (the textual edits are in `MSAC_SIMD_EDITS`),
and the seven `_neon` names, or the six `_sse2` names and the `_avx2`
16-symbol function, resolve to the original's hidden assembly functions and
to the port's rebuilt kernels through test-only accessors; the port is then
built with its `port-test-bridges` feature (accessors only) into
`build/dav1d-bridges-target`. At the AVX2 level the test selects the AVX2
16-symbol function, as the original does on such a CPU. The SSE2 16-symbol
function, which the original selects only on a CPU without AVX2, is outside
the scope (engine ADR-0041 section 4; ADR 0062, "Levels") and not rebuilt:
the test body's assignment of it, which the AVX2 assignment overwrites at
this level, is taken out (`MSAC_SIMD_EDITS`), and there is no gate level
below AVX2 on x86-64. On x86-64 the test body of msac is compiled
with `HAVE_ASM` 1: the original's `MsacContext` has one more member there
(`symbol_adapt16`), which the original's `dav1d_msac_init` writes; the
port's functions use the members before it.

Every run repeats the bodies with each seed of `SEEDS`: checkasm draws its
block contents, strengths and sizes from the seed, and a fault in a narrow
rounding step can stay hidden at one seed (ADR 0062, validation of
2026-11-17). All seeds must pass; `results.log` holds every seed's output in
the order of `SEEDS`.

The original checkasm framework also needs NASM on GNU Linux x86-64. That
assembler builds test infrastructure only; codec assembly stays disabled.
ADR 0029 records the test-only binding bridge and native helpers.
"""
import argparse
from concurrent.futures import ThreadPoolExecutor
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
# The architecture each level belongs to (`uname -m` names).
SIMD_MACHINES = {'neon': ('arm64', 'aarch64'), 'avx2': ('x86_64',)}
# checkasm's random seeds. 42 is the gate's first seed and stays; 1 to 7 were
# added on 2026-11-17, after a one-constant fault in the 16 bpc chroma loop
# filter passed at 42 alone (the numbers were fixed before the fault was run
# against them).
# 1129287872 and 2326805120 were added on 2026-11-17 for loop restoration,
# whose test draws one width and one height per function and seed. checkasm
# seeds xorshift128 without mixing, so neighboring seeds draw related sizes:
# at the first eight the 5x5 and 3x3 self-guided filters only got heights
# that are multiples of 8 and the combined filter only widths that are
# multiples of 8, and the wrappers' tails for a height of 1 (`vert_1`) and,
# at 5x5, for an odd height (`odd`) never ran. The first new seed draws a
# height of 1 for all three filters (5x5 and 3x3 at 186x1, combined at
# 257x1), the second 235x7 for 5x5 and 3x3 and 135x1 for the combined filter
# (ADR 0062, validation of 2026-11-17, "gate seeds").
# 15 was added on the same day for the 8 bpc loop filter: at the ten seeds
# before it the test's random edges never produce a pixel that the 16-pixel
# filter of luma (flat16) writes at 8 bpc, so a fault in its arithmetic
# passed them all, on NEON and on AVX2; at 15 both luma groups reach it.
# Never remove a seed to make a run pass: a failure at any seed is a finding.
SEEDS = [42, 1, 2, 3, 4, 5, 6, 7, 1129287872, 2326805120, 15]
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
# The seven msac entry points of src/x86/msac.h that the original runs on an
# x86-64 machine with AVX2 (the eighth, the SSE2 16-symbol function, is
# selected only without AVX2 and is not rebuilt).
MSAC_X86 = {
    'dav1d_msac_decode_symbol_adapt4_sse2': 'unsigned (*)(MsacContext *, uint16_t *, size_t)',
    'dav1d_msac_decode_symbol_adapt8_sse2': 'unsigned (*)(MsacContext *, uint16_t *, size_t)',
    'dav1d_msac_decode_symbol_adapt16_avx2': 'unsigned (*)(MsacContext *, uint16_t *, size_t)',
    'dav1d_msac_decode_hi_tok_sse2': 'unsigned (*)(MsacContext *, uint16_t *)',
    'dav1d_msac_decode_bool_adapt_sse2': 'unsigned (*)(MsacContext *, uint16_t *)',
    'dav1d_msac_decode_bool_equi_sse2': 'unsigned (*)(MsacContext *)',
    'dav1d_msac_decode_bool_sse2': 'unsigned (*)(MsacContext *, unsigned)',
}
MSAC_KERNELS = {'neon': MSAC_NEON, 'avx2': MSAC_X86}
# The test body's assembly branch, enabled at the SIMD level: HAVE_ASM is 0
# in the portable configuration the bodies are compiled with, and the CPU
# flags are those of the library under test. On x86-64 the msac test body is
# compiled with HAVE_ASM 1 instead (MSAC_X86_PROLOGUE), which enables the
# branch and gives the test the original's MsacContext with assembly. The
# third x86 edit takes out the assignment of the SSE2 16-symbol function,
# which the port does not have: at the AVX2 level, the only x86-64 level of
# this gate, the assignment under DAV1D_X86_CPU_FLAG_AVX2 two statements
# below replaces it on both sides before anything is called (the run fails
# unless both sides report AVX2), so the functions compared are the same
# with and without that line.
MSAC_SIMD_EDITS = {
    'neon': [('#if (ARCH_AARCH64 || ARCH_ARM) && HAVE_ASM', '#if (ARCH_AARCH64 || ARCH_ARM)'),
             ('if (dav1d_get_cpu_flags() & DAV1D_ARM_CPU_FLAG_NEON) {',
              'if (port_cpu_level() & DAV1D_ARM_CPU_FLAG_NEON) {')],
    'x86': [('if (dav1d_get_cpu_flags() & DAV1D_X86_CPU_FLAG_SSE2) {',
             'if (port_cpu_level() & DAV1D_X86_CPU_FLAG_SSE2) {'),
            ('if (dav1d_get_cpu_flags() & DAV1D_X86_CPU_FLAG_AVX2) {',
             'if (port_cpu_level() & DAV1D_X86_CPU_FLAG_AVX2) {'),
            ('        c.decode_symbol_adapt16 = dav1d_msac_decode_symbol_adapt16_sse2;\n', '')],
}
MSAC_X86_PROLOGUE = ['#include "config.h"', '#undef HAVE_ASM', '#define HAVE_ASM 1']


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--debug', action='store_true')
    parser.add_argument('--simd', choices=sorted(SIMD_MASKS))
    parser.add_argument('--jobs', type=int, default=min(4, os.cpu_count() or 1),
                        help='seeds run at the same time (default: 4, or the CPU count if lower)')
    args = parser.parse_args()
    profile = 'debug' if args.debug else 'release'
    if args.simd and os.uname().machine not in SIMD_MACHINES[args.simd]:
        raise SystemExit(f'--simd {args.simd} needs one of {SIMD_MACHINES[args.simd]}, not {os.uname().machine}')
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
    msac_simd = args.simd is not None
    msac_kernels = MSAC_KERNELS.get(args.simd, {})
    if msac_simd:
        bridges = ROOT / 'build/dav1d-bridges-target'
        subprocess.run(['cargo', 'build', '-p', 'ferrisma-dav1d', '--locked', '--features', 'port-test-bridges',
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
                 for name in msac_kernels) if msac_simd and oracle else ''))
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
        prologue = []
        if name == 'msac' and msac_simd:
            for old, new in MSAC_SIMD_EDITS['neon' if args.simd == 'neon' else 'x86']:
                if text.count(old) != 1:
                    raise SystemExit(f'Original msac test differs: {old}')
                text = text.replace(old, new)
            original = work / f'msac-{args.simd}.c'
            original.write_text(text)
            extra = [f'#define {symbol} (*({kind})port_lookup("{symbol}"))' for symbol, kind in msac_kernels.items()]
            if args.simd != 'neon':
                prologue = MSAC_X86_PROLOGUE
        declarations = prologue + [line for line in re.findall(r'^#include[^\n]+', text, re.M)
                                   if not line.endswith('.c"')]
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

    def run(seed):
        return subprocess.run([executable, str(seed)], env=environment, text=True,
                              stdout=subprocess.PIPE, stderr=subprocess.STDOUT)

    with ThreadPoolExecutor(max_workers=max(1, args.jobs)) as pool:
        results = list(pool.map(run, SEEDS))
    (work / 'results.log').write_text(''.join(result.stdout for result in results))
    expected = hex(SIMD_MASKS[args.simd]) if args.simd else '0x0'
    failed = []
    counts = set()
    for seed, result in zip(SEEDS, results):
        if f'Random seed: {seed}\n' not in result.stdout:
            raise SystemExit(f'checkasm did not run with seed {seed}: inspect {work / "results.log"}')
        selections = re.findall(r'\[port\] checked C=(\d+) Rust=(\d+) function selections', result.stdout)
        unsupported = re.findall(r'\[port\] null C=(\d+) Rust=(\d+) unsupported selections', result.stdout)
        if result.returncode or len(selections) != 1 or selections[0][0] != selections[0][1] or int(selections[0][0]) == 0:
            failed.append(seed)
            continue
        if len(unsupported) != 1 or unsupported[0][0] != unsupported[0][1]:
            raise SystemExit(f'Unsupported DSP selection inventories differ at seed {seed}')
        levels = dict(re.findall(r'\[port\] phase=(\d) cpu_flags=(0x[0-9a-f]+)', result.stdout))
        if args.simd:
            # Both sides at the same level, and the level fully available here.
            if levels.get('0') != levels.get('1') or int(levels.get('1', '0'), 16) & 0xf != SIMD_MASKS[args.simd] & 0xf:
                raise SystemExit(f'CPU levels differ or are unavailable: {levels} (expected {expected})')
        elif levels != {'0': '0x0', '1': '0x0'}:
            raise SystemExit(f'Scalar comparison ran with CPU flags: {levels}')
        counts.add((selections[0][0], unsupported[0][0], tuple(sorted(levels.items()))))
    if failed:
        raise SystemExit(f'Original DSP comparison failed at seed(s) {failed} of {SEEDS}: inspect {work / "results.log"}')
    if len(counts) != 1:
        raise SystemExit(f'Selection inventories differ between seeds: {sorted(counts)}')
    selected, unselected, levels = counts.pop()
    levels = dict(levels)
    (work / 'inventory.json').write_text(json.dumps({'original_test_instantiations': inventory,
        'source_groups': len(inputs), 'function_selections_each': int(selected),
        'unsupported_selections_each': int(unselected),
        'seed': SEEDS[0], 'seeds': SEEDS, 'reference': str(library), 'candidate': str(candidate),
        'simd_level': args.simd, 'cpu_flags': levels}, indent=2) + '\n')
    print(f'Passed {len(inputs)} original DSP test groups at level {args.simd or "scalar"} '
          f'with {selected} original/Rust function selections each, at each of the {len(SEEDS)} seeds '
          f'{", ".join(map(str, SEEDS))}')


if __name__ == '__main__':
    main()
