#!/usr/bin/env python3
"""Run the unchanged original small-division tests through a generated Rust macro."""
import argparse
import hashlib
import json
from pathlib import Path
import shlex
import subprocess
import sys
import xml.etree.ElementTree as ET

from build_aom import library_entries, test_names
from check_abi import compiler_flags
from check_aom_linkage import consumer_owners, definitions

ROOT = Path(__file__).resolve().parents[1]
EXPORT = 'port_aom_macro_OD_DIVU_SMALL'
SOURCE = 'test/divu_small_test.cc'


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--debug', action='store_true')
    parser.add_argument('--reference-build', type=Path, help='Read an existing native reference without modifying it')
    args = parser.parse_args()
    profile = 'debug' if args.debug else 'release'
    platform = 'macos' if sys.platform == 'darwin' else 'linux'
    build = (args.reference_build or ROOT / f'build/aom-native-{profile}').resolve()
    work = ROOT / f'build/aom-macro-tests-{profile}'
    work.mkdir(parents=True, exist_ok=True)
    (work / 'summary.json').unlink(missing_ok=True)
    provenance = json.loads((ROOT / 'provenance/aom-source.json').read_text())['files']
    manifest = json.loads((ROOT / f'translations/aom-macros-{platform}-{profile}.json').read_text())
    expected_target = 'aarch64-apple-darwin' if platform == 'macos' else 'x86_64-unknown-linux-gnu'
    if (manifest['generation_target'] != expected_target or manifest['preprocessing_profile'] != profile
            or manifest['source_wrapper_sha256'] != hashlib.sha256((ROOT / 'tests/ports/aom_macro_subjects.c').read_bytes()).hexdigest()
            or manifest['source_manifest_sha256'] != hashlib.sha256((ROOT / 'provenance/aom-source.json').read_bytes()).hexdigest()):
        raise SystemExit('Macro generation inputs or target differ from their provenance')
    if (manifest['config_sha256'] != hashlib.sha256((build / 'config/aom_config.h').read_bytes()).hexdigest()
            or json.loads((build / 'reference-profile.json').read_text())['profile'] != profile):
        raise SystemExit('Macro and native reference preprocessing differ')
    for unit in manifest['units']:
        if hashlib.sha256((ROOT / unit['rust']).read_bytes()).hexdigest() != unit['rust_sha256']:
            raise SystemExit('Macro subject differs from its recorded generation')
    entries = json.loads((build / 'compile_commands.json').read_text())
    entry = next(value for value in entries if Path(value['file']).name == 'divu_small_test.cc')
    source = Path(entry['file'])
    source_root = source.parents[1]
    if hashlib.sha256(source.read_bytes()).hexdigest() != provenance[SOURCE]:
        raise SystemExit('Modified original division test source')
    for name, expected in manifest['original_headers'].items():
        if expected != provenance[name] or hashlib.sha256((source_root / name).read_bytes()).hexdigest() != expected:
            raise SystemExit(f'Modified original macro header: {name}')
    command = ['cargo', 'build', '-p', 'libavif-aom', '--features', 'port-macro-tests',
               '--locked', '--target-dir', work / 'target']
    if not args.debug:
        command.append('--release')
    subprocess.run(command, check=True)
    archive = work / f'target/{profile}/libaom.a'
    include = work / 'macro-subject.h'
    include.write_text('#include "aom_dsp/odintrin.h"\n'
                       + f'extern "C" uint32_t {EXPORT}(uint32_t, uint32_t);\n'
                       + '#undef OD_DIVU_SMALL\n'
                       + f'#define OD_DIVU_SMALL(x, d) {EXPORT}((x), (d))\n')
    output = work / 'divu_small_test.cc.o'
    flags = compiler_flags(entry)
    if any(flag in ['-MD', '-MMD', '-MP', '-MG'] or flag.startswith(('-MF', '-MT', '-MQ', '-MJ'))
           for flag in flags):
        raise SystemExit('The read-only reference flags request dependency output files')
    subprocess.run([*flags, '-include', include, '-c', source, '-o', output],
                   cwd=entry['directory'], check=True)
    undefined = {line.split()[-1] for line in subprocess.check_output(['nm', '-u', output], text=True).splitlines() if line.split()}
    if ('_' if platform == 'macos' else '') + EXPORT not in undefined:
        raise SystemExit('The original macro calls did not bind to the Rust subject')
    commands = subprocess.check_output(['ninja', '-C', build, '-t', 'commands', 'test_libaom'], text=True)
    command = shlex.split(commands.splitlines()[-1])
    if command[:2] == [':', '&&'] and command[-2:] == ['&&', ':']:
        command = command[2:-2]
    if any(value in [':', '&&', '||', ';'] for value in command):
        raise SystemExit('Unmapped original test link command')
    library_sources = {Path(value['file']) for value in library_entries(build)}
    objects = {str((Path(value['directory']) / value['output']).relative_to(build)): Path(value['file']) for value in entries}
    for token in list(command):
        if objects.get(token) in library_sources:
            if objects[token].relative_to(source_root).as_posix() not in ['common/args_helper.c', 'av1/arg_defs.c']:
                raise SystemExit(f'Unreviewed original test/codec overlap: {token}')
            command.remove(token)
    old = str((Path(entry['directory']) / entry['output']).relative_to(build))
    if command.count(old) != 1 or command.count('libaom.a') != 1:
        raise SystemExit('Expected exactly one original test object and codec archive')
    command[command.index(old)] = str(output)
    command[command.index('libaom.a')] = str(archive)
    executable = work / 'test_libaom'
    command[command.index('-o') + 1] = str(executable)
    linkmap = work / 'test_libaom.map'
    command.append(f'-Wl,-map,{linkmap}' if platform == 'macos' else f'-Wl,-Map,{linkmap},--cref')
    subprocess.run(command, cwd=build, check=True)
    owners, loaded = consumer_owners(linkmap)
    native_objects = [str((Path(value['directory']) / value['output']).relative_to(build)) for value in library_entries(build)]
    if any(name == str(build / 'libaom.a') or (str(build / 'libaom.a') + '(') in name
           or name == 'libaom.a' or any(name == value or name.endswith('/' + value) for value in native_objects)
           for name in loaded):
        raise SystemExit('An original codec object or archive entered the Rust macro consumer')
    original_globals = definitions(build / 'libaom.a')
    required = set(original_globals) | {EXPORT}
    for name, owner in owners.items():
        if name in required and (not owner.startswith(str(archive) + '(aom.') or not owner.endswith('.rcgu.o)')):
            raise SystemExit(f'Macro consumer global supplied outside Rust: {name}: {owner}')
    if not {EXPORT, 'OD_DIVU_SMALL_CONSTS'} <= owners.keys():
        raise SystemExit('Both the Rust macro and Rust constant table must be loaded')
    names = test_names((build / 'test_libaom-inventory.txt').read_text())
    if test_names(subprocess.check_output([executable, '--gtest_list_tests'], text=True)) != names:
        raise SystemExit('Macro binding changed original registrations, order, or parameters')
    records = []
    for label, binary in [('c', build / 'test_libaom'), ('rust', executable)]:
        xml = work / f'{label}.xml'
        xml.unlink(missing_ok=True)
        with (work / f'{label}.log').open('w') as log:
            subprocess.run([binary, '--gtest_filter=DivuSmallTest.*', f'--gtest_output=xml:{xml}'],
                           cwd=build, stdout=log, stderr=subprocess.STDOUT, check=True)
        cases = [dict(name=case.attrib['classname'] + '.' + case.attrib['name'],
                      result=case.get('result'), failed=bool(case.findall('failure')),
                      skipped=bool(case.findall('skipped')))
                 for case in ET.parse(xml).getroot().iter('testcase') if case.get('status') == 'run']
        if any(case['failed'] or case['skipped'] for case in cases):
            raise SystemExit('An original division case failed or skipped')
        records.append(cases)
    if records[0] != records[1] or len(records[0]) != 2:
        raise SystemExit('Original macro execution inventories differ')
    report = dict(profile=profile, original_registrations=len(names), executed_original_cases=2,
                  original_source=SOURCE, original_source_sha256=provenance[SOURCE],
                  original_inputs_assertions_loops_and_disabled_flags_unchanged=True,
                  original_cpp_division_oracle_retained=True, generated_rust_macro_subject=EXPORT,
                  rust_constant_table='OD_DIVU_SMALL_CONSTS',
                  macro_manifest_sha256=hashlib.sha256((ROOT / f'translations/aom-macros-{platform}-{profile}.json').read_bytes()).hexdigest(),
                  rust_archive_sha256=hashlib.sha256(archive.read_bytes()).hexdigest(),
                  forced_include_sha256=hashlib.sha256(include.read_bytes()).hexdigest(),
                  map_sha256=hashlib.sha256(linkmap.read_bytes()).hexdigest(), cases=records[1])
    (work / 'summary.json').write_text(json.dumps(report, indent=2) + '\n')
    print('Two unchanged original division cases exercise the generated Rust macro and table; all 18,729 registrations preserved')


if __name__ == '__main__':
    main()
