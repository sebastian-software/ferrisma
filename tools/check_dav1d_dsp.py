#!/usr/bin/env python3
"""Run original checkasm bodies across original C and Rust scalar pointers."""
import argparse
import json
import os
from pathlib import Path
import re
import shutil
import subprocess

from build_dav1d import copy_inputs

ROOT = Path(__file__).resolve().parents[1]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--debug', action='store_true')
    args = parser.parse_args()
    profile = 'debug' if args.debug else 'release'
    reference = ROOT / f'build/dav1d-reference-{profile}'
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
    work = ROOT / f'build/dav1d-dsp-{profile}'
    work.mkdir(exist_ok=True)
    library = reference / 'src/libdav1d.7.dylib'
    candidate = ROOT / 'target' / ('debug' if args.debug else 'release') / 'libdav1d.dylib'
    symbols = {line.split()[-1].removeprefix('_') for line in subprocess.check_output(['nm', '-gU', library], text=True).splitlines() if line.strip()}
    flags = ['cc', '-std=c99', '-O2', '-g', '-ffp-contract=off', '-DCHECKASM_HAVE_GENERIC=1']
    for directory in [reference, source, source / 'include', source / 'include/dav1d',
                      ROOT / 'build/deps/checkasm/include', framework / 'include', ROOT / 'tools']:
        flags += ['-I', str(directory)]
    # Upstream deliberately hides these constant tables. Expose their addresses
    # in a test-only DSO assembled from the same 51 original reference objects.
    tables = ['dav1d_block_dimensions', 'dav1d_scans', 'dav1d_sgr_params',
              'dav1d_tx_type_class', 'dav1d_txfm_dimensions']
    accessor = work / 'table-accessors.c'
    accessor.write_text('#include "config.h"\n#include "src/tables.h"\n#include "src/scan.h"\n' +
        '\n'.join(f'void *port_table_{name}(void) {{ return (void *)&{name}; }}' for name in tables) + '\n')
    accessor_object = work / 'table-accessors.o'
    subprocess.run(flags + ['-c', accessor, '-o', accessor_object], check=True)
    units = [entry for entry in json.loads((reference / 'compile_commands.json').read_text())
             if entry['output'].startswith('src/libdav1d')]
    library = work / 'libdav1d-dsp-reference.dylib'
    subprocess.run(['cc', '-dynamiclib', '-o', library, accessor_object,
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
        declarations = [line for line in re.findall(r'^#include[^\n]+', text, re.M) if not line.endswith('.c"')]
        macros = []
        for symbol in sorted(set(re.findall(r'\bdav1d_\w+', text))):
            if symbol not in symbols and depth is not None:
                symbol += f'_{depth}bpc'
            if symbol in symbols:
                macros.append(f'#define {symbol} (*(__typeof__(&{symbol}))port_lookup("{symbol}"))')
        header = work / f'{name}-{depth}.h'
        header.write_text('\n'.join(declarations + ['#include "dav1d-checkasm-runtime.h"'] + macros) + '\n')
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
                    library, '-lm'], check=True)
    environment = os.environ.copy()
    environment.update(DAV1D_REFERENCE=str(library), DAV1D_CANDIDATE=str(candidate))
    result = subprocess.run([executable, '42'], env=environment, text=True,
                            stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
    (work / 'results.log').write_text(result.stdout)
    selections = re.findall(r'\[port\] checked C=(\d+) Rust=(\d+) function selections', result.stdout)
    unsupported = re.findall(r'\[port\] null C=(\d+) Rust=(\d+) unsupported selections', result.stdout)
    if result.returncode or len(selections) != 1 or selections[0][0] != selections[0][1] or int(selections[0][0]) == 0:
        raise SystemExit(f'Original DSP comparison failed: inspect {work / "results.log"}')
    if len(unsupported) != 1 or unsupported[0][0] != unsupported[0][1]:
        raise SystemExit('Unsupported DSP selection inventories differ')
    (work / 'inventory.json').write_text(json.dumps({'original_test_instantiations': inventory,
        'source_groups': len(inputs), 'function_selections_each': int(selections[0][0]),
        'unsupported_selections_each': int(unsupported[0][0]),
        'seed': 42, 'reference': str(library), 'candidate': str(candidate)}, indent=2) + '\n')
    print(f'Passed {len(inputs)} original DSP test groups with {selections[0][0]} C/Rust function selections each')


if __name__ == '__main__':
    main()
