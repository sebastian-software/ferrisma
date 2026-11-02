#!/usr/bin/env python3
"""Run unchanged original header-subject assertions against optional Rust exports."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import shlex
import subprocess
import sys
import xml.etree.ElementTree as ET

from aom_inline_bridges import FILTER, SUBJECTS
from build_aom import library_entries, test_names
from check_abi import compiler_flags

ROOT = Path(__file__).resolve().parents[1]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--debug', action='store_true')
    args = parser.parse_args()
    profile = 'debug' if args.debug else 'release'
    build = ROOT / f'build/aom-native-{profile}'
    work = ROOT / f'build/aom-inlines-{profile}'
    work.mkdir(parents=True, exist_ok=True)
    command = ['cargo', 'build', '-p', 'libavif-aom', '--features', 'port-test-bridges',
               '--locked', '--target-dir', work / 'target']
    if not args.debug:
        command.append('--release')
    subprocess.run(command, check=True)
    archive = work / f'target/{profile}/libaom.a'
    source_root = ROOT / 'build/aom-native-source'
    source_manifest = json.loads((ROOT / 'provenance/aom-source.json').read_text())['files']
    entries = json.loads((build / 'compile_commands.json').read_text())
    replacement, subjects = {}, []
    for filename, (headers, names) in SUBJECTS.items():
        entry = next(value for value in entries if Path(value['file']).name == filename)
        source = Path(entry['file'])
        relative = source.relative_to(source_root).as_posix()
        if hashlib.sha256(source.read_bytes()).hexdigest() != source_manifest[relative]:
            raise SystemExit(f'Modified original test source: {relative}')
        include = work / (filename + '.h')
        headers = [headers] if isinstance(headers, str) else headers
        text = '\n'.join(f'#include "{header}"' for header in headers) + '\nextern "C" {\n'
        text += ''.join(f'decltype({name}) port_aom_inline_{name};\n' for name in names)
        text += '}\n' + ''.join(f'#define {name} port_aom_inline_{name}\n' for name in names)
        include.write_text(text)
        output = work / (filename + '.o')
        subprocess.run([*compiler_flags(entry), '-include', include, '-c', source, '-o', output],
                       cwd=entry['directory'], check=True)
        undefined = subprocess.check_output(['nm', '-u', output], text=True)
        for name in names:
            symbol = ('_' if sys.platform == 'darwin' else '') + 'port_aom_inline_' + name
            if not any(line.split()[-1] == symbol for line in undefined.splitlines() if line.split()):
                raise SystemExit(f'Original test subject did not bind to its Rust export: {relative}: {name}')
        old = str((Path(entry['directory']) / entry['output']).relative_to(build))
        replacement[old] = str(output)
        subjects.append(dict(source=relative, source_sha256=source_manifest[relative], functions=names,
                             forced_include_sha256=hashlib.sha256(include.read_bytes()).hexdigest(),
                             assertions_and_cases_unchanged=True))
    commands = subprocess.check_output(['ninja', '-C', build, '-t', 'commands', 'test_libaom'], text=True)
    command = shlex.split(commands.splitlines()[-1])
    if command[:2] == [':', '&&'] and command[-2:] == ['&&', ':']:
        command = command[2:-2]
    if any(token in [':', '&&', '||', ';'] for token in command):
        raise SystemExit('Unmapped original test link command')
    library_sources = {Path(entry['file']) for entry in library_entries(build)}
    objects = {str((Path(entry['directory']) / entry['output']).relative_to(build)): Path(entry['file']) for entry in entries}
    for token in list(command):
        if objects.get(token) in library_sources:
            if objects[token].relative_to(source_root).as_posix() not in ['common/args_helper.c', 'av1/arg_defs.c']:
                raise SystemExit(f'Unreviewed test/app and codec overlap: {token}')
            command.remove(token)
    for original, new in replacement.items():
        if command.count(original) != 1:
            raise SystemExit(f'Expected exactly one original test object: {original}')
        command[command.index(original)] = new
    if command.count('libaom.a') != 1:
        raise SystemExit('Expected exactly one original codec archive')
    command[command.index('libaom.a')] = str(archive)
    executable = work / 'test_libaom'
    command[command.index('-o') + 1] = str(executable)
    link_map = work / 'test_libaom.map'
    command.append(f'-Wl,-map,{link_map}' if sys.platform == 'darwin' else f'-Wl,-Map,{link_map}')
    subprocess.run(command, cwd=build, check=True)
    reference_names = test_names((build / 'test_libaom-inventory.txt').read_text())
    if test_names(subprocess.check_output([executable, '--gtest_list_tests'], text=True)) != reference_names:
        raise SystemExit('Header bridge changed original registrations, order, or parameters')
    records = []
    environment = dict(os.environ, LIBAOM_TEST_DATA_PATH=str(ROOT / 'build/deps/aom-test-data'))
    for label, binary in [('c', build / 'test_libaom'), ('rust', executable)]:
        xml = work / f'{label}.xml'
        with (work / f'{label}.log').open('w') as log:
            subprocess.run([binary, f'--gtest_filter={FILTER}', f'--gtest_output=xml:{xml}'],
                           cwd=build, env=environment, stdout=log, stderr=subprocess.STDOUT, check=True)
        cases = [dict(name=case.attrib['classname'] + '.' + case.attrib['name'],
                      status=case.attrib.get('status'), result=case.attrib.get('result'),
                      failed=bool(case.findall('failure')), skipped=bool(case.findall('skipped')))
                 for case in ET.parse(xml).getroot().iter('testcase') if case.attrib.get('status') == 'run']
        if any(value['failed'] or value['skipped'] for value in cases):
            raise SystemExit('An original inline-subject assertion failed or skipped')
        records.append(cases)
    if records[0] != records[1] or len(records[0]) != 12:
        raise SystemExit('Original/Rust inline-subject execution inventories differ')
    report = dict(profile=profile, original_registrations=len(reference_names), executed_original_cases=12,
                  original_cases_and_assertions_unchanged=True, optional_rust_bridge_exports=10,
                  original_inline_subjects_execute_rust=True, native_reference_and_fixture_helpers_retained=True,
                  complete_inline_role_review=False, subjects=subjects, cases=records[1],
                  rust_archive_sha256=hashlib.sha256(archive.read_bytes()).hexdigest(),
                  map_sha256=hashlib.sha256(link_map.read_bytes()).hexdigest())
    (work / 'summary.json').write_text(json.dumps(report, indent=2) + '\n')
    print('12 original cases exercise 10 Rust inline subjects; all 18,729 registrations preserved')


if __name__ == '__main__':
    main()
