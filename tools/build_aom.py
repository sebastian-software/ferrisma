#!/usr/bin/env python3
"""Build the complete scalar libaom reference and inventory its original tests."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import shlex
import subprocess

from build_dav1d import copy_inputs

ROOT = Path(__file__).resolve().parents[1]


def library_entries(build):
    line = next(line for line in (build / 'build.ninja').read_text().splitlines()
                if line.startswith('build libaom.a:'))
    objects = shlex.split(line.split(' |')[0])[3:]
    database = json.loads((build / 'compile_commands.json').read_text())
    entries = [entry for entry in database
               if str(Path(entry['output']).relative_to(build)) in objects]
    if len(entries) != len(objects) or not all(e['file'].endswith('.c') for e in entries):
        raise SystemExit('Archive closure does not match the C compilation database')
    return entries


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--debug', action='store_true')
    parser.add_argument('--jobs', type=int, default=8)
    args = parser.parse_args()
    subprocess.run(['python3', ROOT / 'tools/verify_upstream.py'], check=True)
    source = ROOT / 'build/aom-native-source'
    copy_inputs(json.loads((ROOT / 'provenance/aom-source.json').read_text()), source)
    profile = 'debug' if args.debug else 'release'
    build = ROOT / f'build/aom-native-{profile}'
    flags = f'-ffp-contract=off -fmacro-prefix-map={source}=upstream/aom'
    options = ['-DAOM_TARGET_CPU=generic', '-DCONFIG_TUNE_VMAF=1',
               '-DCMAKE_POSITION_INDEPENDENT_CODE=ON', '-DENABLE_DOCS=OFF']
    command = ['cmake', '-S', source, '-B', build, '-G', 'Ninja',
               f'-DCMAKE_BUILD_TYPE={profile.title()}',
               '-DCMAKE_EXPORT_COMPILE_COMMANDS=ON',
               f'-DCMAKE_C_FLAGS={flags}', f'-DCMAKE_CXX_FLAGS={flags}', *options]
    subprocess.run([str(item) for item in command], check=True)
    subprocess.run(['ninja', '-C', build, '-j', str(args.jobs)], check=True)
    entries = library_entries(build)
    if len(entries) != 168:
        raise SystemExit(f'Unexpected complete portable source closure: {len(entries)}')
    config = (build / 'config/aom_config.h').read_text()
    for setting in ['CONFIG_AV1_ENCODER 1', 'CONFIG_AV1_DECODER 1',
                    'CONFIG_AV1_HIGHBITDEPTH 1', 'CONFIG_MULTITHREAD 1',
                    'CONFIG_DENOISE 1', 'CONFIG_QUANT_MATRIX 1',
                    'CONFIG_REALTIME_ONLY 0', 'CONFIG_TUNE_VMAF 1']:
        if f'#define {setting}\n' not in config:
            raise SystemExit(f'Missing required configuration: {setting}')
    for line in config.splitlines():
        if line.startswith('#define HAVE_') and any(word in line for word in
            ['NEON', 'SVE', 'SSE', 'AVX', 'MMX', 'VSX', 'RVV', 'ARM_CRC']):
            if not line.endswith(' 0'):
                raise SystemExit(f'Unexpected optimized implementation: {line}')
    (build / 'library-compile-commands.json').write_text(json.dumps(entries, indent=2) + '\n')
    inventories = {}
    for name in ['test_libaom', 'test_aom_rc', 'test_intra_pred_speed']:
        listing = subprocess.check_output([build / name, '--gtest_list_tests'], text=True)
        (build / f'{name}-inventory.txt').write_text(listing)
        inventories[name] = dict(registrations=sum(line.startswith('  ') for line in listing.splitlines()),
                                 sha256=hashlib.sha256(listing.encode()).hexdigest())
    data_commands = subprocess.check_output(['ninja', '-C', build, '-t', 'commands', 'testdata'], text=True)
    data = {}
    for command in data_commands.splitlines():
        values = dict(argument[2:].split('=', 1) for argument in shlex.split(command)
                      if argument.startswith('-DAOM_') and '=' in argument)
        name = values['AOM_TEST_FILE']
        data[name] = values['AOM_TEST_CHECKSUM']
    (build / 'test-data.json').write_text(json.dumps(data, indent=2, sort_keys=True) + '\n')
    record = dict(reference='libaom 3.15.2 complete portable C', profile=profile,
                  options=options, c_flags=flags.replace(str(source), '<source>'),
                  compiler=subprocess.check_output([os.environ.get('CC', 'cc'), '--version'],
                                                   text=True).splitlines()[0],
                  library_translation_units=len(entries), generated_units=3,
                  original_test_inventories=inventories, original_test_inputs=len(data),
                  config_sha256=hashlib.sha256(config.encode()).hexdigest(),
                  transitive_runtime_dependencies={'libvmaf': subprocess.check_output(
                      ['pkg-config', '--modversion', 'libvmaf'], text=True).strip()})
    (build / 'reference-profile.json').write_text(json.dumps(record, indent=2) + '\n')
    print(json.dumps(record, indent=2))


if __name__ == '__main__':
    main()
