#!/usr/bin/env python3
"""Build the pinned complete portable dav1d reference without editing upstream."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess

ROOT = Path(__file__).resolve().parents[1]


def copy_inputs(record, target):
    source = ROOT / record['source_root']
    for name, expected in record['files'].items():
        content = (source / name).read_bytes()
        if hashlib.sha256(content).hexdigest() != expected:
            raise SystemExit(f'Modified test input: {source / name}')
        output = target / name
        output.parent.mkdir(parents=True, exist_ok=True)
        if not output.exists() or output.read_bytes() != content:
            output.write_bytes(content)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--debug', action='store_true')
    args = parser.parse_args()
    subprocess.run(['python3', ROOT / 'tools/verify_upstream.py'], check=True)
    meson = shutil.which('meson') or str(ROOT / 'build/dav1d-tools/bin/meson')
    source = ROOT / 'build/dav1d-native-source'
    copy_inputs(json.loads((ROOT / 'provenance/dav1d-source.json').read_text()), source)
    tests = json.loads((ROOT / 'provenance/dav1d-test-inputs.json').read_text())
    for name, target in [('checkasm', source / 'subprojects/checkasm'),
                         ('dav1d-test-data', source / 'tests/dav1d-test-data')]:
        copy_inputs(tests[name], target)
    build = ROOT / ('build/dav1d-reference-debug' if args.debug else 'build/dav1d-reference-release')
    flags = '-ffp-contract=off -fvisibility=default'
    setup = [meson, 'setup', build, source, '-Denable_asm=false', '-Dbitdepths=8,16',
             '-Dtrim_dsp=false', '-Denable_tests=true', '-Denable_tools=true',
             '-Denable_seek_stress=true', '-Dtestdata_tests=true', '-Dxxhash_muxer=disabled',
             '-Dcheckasm:docs=disabled', '-Dcheckasm:tests=disabled',
             '--wrap-mode=nofallback', f'-Dc_args={flags}',
             '--buildtype=debug' if args.debug else '--buildtype=release']
    if (build / 'build.ninja').exists():
        setup.append('--reconfigure')
    subprocess.run([str(x) for x in setup], check=True)
    subprocess.run(['ninja', '-C', build], check=True)
    database = json.loads((build / 'compile_commands.json').read_text())
    units = [entry for entry in database if entry['output'].startswith('src/libdav1d')]
    if len(units) != 51:
        raise SystemExit(f'Unexpected portable decoder source closure: {len(units)}')
    configuration = (build / 'config.h').read_text()
    for setting in ['HAVE_ASM 0', 'CONFIG_8BPC 1', 'CONFIG_16BPC 1', 'TRIM_DSP_FUNCTIONS 0', 'CONFIG_LOG 1']:
        if f'#define {setting}\n' not in configuration:
            raise SystemExit(f'Missing selected configuration: {setting}')
    profile = {'reference': 'dav1d 1.5.4 portable original C', 'acceptance_reference': True,
               'profile': 'debug' if args.debug else 'release', 'c_flags': flags,
               'compiler': subprocess.check_output([os.environ.get('CC', 'cc'), '--version'], text=True).splitlines()[0],
               'library_translation_units': len(units), 'bitdepth_instantiations': [8, 16],
               'assembly': False, 'trim_dsp': False, 'logging': True,
               'config_sha256': hashlib.sha256(configuration.encode()).hexdigest(),
               'compile_commands_sha256': hashlib.sha256((build / 'compile_commands.json').read_bytes()).hexdigest()}
    (build / 'reference-profile.json').write_text(json.dumps(profile, indent=2) + '\n')
    print(f'Built {len(units)} original portable decoder translation units: {build}')


if __name__ == '__main__':
    main()
