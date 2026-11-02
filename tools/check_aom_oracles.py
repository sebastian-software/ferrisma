#!/usr/bin/env python3
"""Preserve original convolution assertions with separate C oracle/Rust subjects."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import shlex
import subprocess
import sys
import xml.etree.ElementTree as ET

from aom_reference import PREFIX, reference_archive
from build_aom import library_entries, test_names
from check_abi import compiler_flags
from check_aom_linkage import consumer_owners
from dav1d_storage import tokens

ROOT = Path(__file__).resolve().parents[1]
SOURCE = 'test/av1_convolve_test.cc'
# Reviewed oracle calls only. Parameter initializers and indirect subjects keep
# their original names. Include disabled speed-call sites without enabling them.
SITES = {
    'av1_convolve_x_sr_c': [361, 388],
    'av1_highbd_convolve_x_sr_c': [577, 604],
    'av1_convolve_y_sr_c': [794, 815],
    'av1_highbd_convolve_y_sr_c': [991, 1012],
    'aom_convolve_copy_c': [1166],
    'aom_highbd_convolve_copy_c': [1212],
}
FILTER = ':'.join(f'C/AV1Convolve{name}Test.RunTest/*' for name in
                  ['X', 'XHighbd', 'Y', 'YHighbd', 'Copy', 'CopyHighbd'])


def audit_map(path, symbols, rust_archive, oracle_archive):
    loaded, oracle_loaded = set(), set()
    owners, _ = consumer_owners(path)
    for name, owner in owners.items():
        if name in symbols:
            if not owner.startswith(str(rust_archive) + '(aom.') or not owner.endswith('.rcgu.o)'):
                raise SystemExit(f'Original subject global supplied outside Rust: {name}: {owner}')
            loaded.add(name)
        if name.startswith(PREFIX) and name[len(PREFIX):] in symbols:
            if not owner.startswith(str(oracle_archive) + '('):
                raise SystemExit(f'Oracle global supplied outside isolated C: {name}: {owner}')
            oracle_loaded.add(name[len(PREFIX):])
    if not set(SITES) <= loaded or not set(SITES) <= oracle_loaded:
        raise SystemExit('The original subject and independent C oracle are not both loaded')
    return dict(rust_original_globals=len(loaded), isolated_c_oracle_globals=len(oracle_loaded),
                reviewed_subject_and_oracle_functions=sorted(SITES),
                map_sha256=hashlib.sha256(path.read_bytes()).hexdigest())


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--debug', action='store_true')
    args = parser.parse_args()
    profile = 'debug' if args.debug else 'release'
    build = ROOT / f'build/aom-native-{profile}'
    work = ROOT / f'build/aom-oracles-{profile}'
    work.mkdir(parents=True, exist_ok=True)
    archive = ROOT / f'target/{profile}/libaom.a'
    oracle, symbols, oracle_record = reference_archive(profile)
    entries = json.loads((build / 'compile_commands.json').read_text())
    source_root = ROOT / 'build/aom-native-source'
    source = source_root / SOURCE
    source_hash = json.loads((ROOT / 'provenance/aom-source.json').read_text())['files'][SOURCE]
    if hashlib.sha256(source.read_bytes()).hexdigest() != source_hash:
        raise SystemExit('Modified original convolution test source')
    entry = next(value for value in entries if Path(value['file']) == source)
    original = source.read_text()
    lexemes, replacements, sites = tokens(original), [], {name: [] for name in SITES}
    for index, token in enumerate(lexemes[:-1]):
        if token[0] in SITES and lexemes[index + 1][0] == '(':
            sites[token[0]].append(original.count('\n', 0, token.start()) + 1)
            replacements.append((token.start(), token.end(), PREFIX + token[0]))
    if sites != SITES:
        raise SystemExit(f'Original oracle call sites changed: {sites}')
    text = original
    for start, end, name in reversed(replacements):
        text = text[:start] + name + text[end:]
    include = work / 'oracles.h'
    include.write_text('#include "config/av1_rtcd.h"\n#include "config/aom_dsp_rtcd.h"\nextern "C" {\n'
                       + ''.join(f'decltype({name}) {PREFIX}{name};\n' for name in SITES) + '}\n')
    copied = work / 'av1_convolve_test.cc'
    copied.write_text(f'#line 1 "{SOURCE}"\n' + text)
    output = work / 'av1_convolve_test.cc.o'
    subprocess.run([*compiler_flags(entry), '-include', include, '-c', copied, '-o', output],
                   cwd=entry['directory'], check=True)
    undefined = {line.split()[-1] for line in subprocess.check_output(['nm', '-u', output], text=True).splitlines() if line.split()}
    decoration = '_' if sys.platform == 'darwin' else ''
    if not {decoration + prefix + name for prefix in ['', PREFIX] for name in SITES} <= undefined:
        raise SystemExit('Original parameterized subjects and isolated C oracles must both remain external')
    commands = subprocess.check_output(['ninja', '-C', build, '-t', 'commands', 'test_libaom'], text=True)
    command = shlex.split(commands.splitlines()[-1])
    if command[:2] == [':', '&&'] and command[-2:] == ['&&', ':']:
        command = command[2:-2]
    if any(token in [':', '&&', '||', ';'] for token in command):
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
        raise SystemExit('Expected one original test object and one codec archive')
    command[command.index(old)] = str(output)
    index = command.index('libaom.a')
    command[index:index + 1] = [str(archive), str(oracle)]
    executable = work / 'test_libaom'
    command[command.index('-o') + 1] = str(executable)
    link_map = work / 'test_libaom.map'
    command.append(f'-Wl,-map,{link_map}' if sys.platform == 'darwin' else f'-Wl,-Map,{link_map},--cref')
    subprocess.run(command, cwd=build, check=True)
    ownership = audit_map(link_map, symbols, archive, oracle)
    names = test_names((build / 'test_libaom-inventory.txt').read_text())
    if test_names(subprocess.check_output([executable, '--gtest_list_tests'], text=True)) != names:
        raise SystemExit('Independent oracle binding changed original registrations, order, or parameters')
    results = []
    environment = dict(os.environ, LIBAOM_TEST_DATA_PATH=str(ROOT / 'build/deps/aom-test-data'))
    for label, binary in [('c', build / 'test_libaom'), ('rust', executable)]:
        xml = work / f'{label}.xml'
        with (work / f'{label}.log').open('w') as log:
            subprocess.run([binary, f'--gtest_filter={FILTER}', f'--gtest_output=xml:{xml}'],
                           cwd=build, env=environment, stdout=log, stderr=subprocess.STDOUT, check=True)
        cases = [dict(name=case.attrib['classname'] + '.' + case.attrib['name'],
                      failed=bool(case.findall('failure')), skipped=bool(case.findall('skipped')))
                 for case in ET.parse(xml).getroot().iter('testcase') if case.attrib.get('status') == 'run']
        if any(case['failed'] or case['skipped'] for case in cases):
            raise SystemExit('An original independent-oracle assertion failed or skipped')
        results.append(cases)
    if results[0] != results[1] or len(results[1]) != 243:
        raise SystemExit('Original independent-oracle execution inventories differ')
    record = dict(profile=profile, original_registrations=len(names), executed_original_cases=len(results[1]),
                  original_assertions_parameters_and_disabled_flags_unchanged=True,
                  original_source=SOURCE, original_source_sha256=source_hash, oracle_sites=SITES,
                  binding_copy_sha256=hashlib.sha256(copied.read_bytes()).hexdigest(),
                  complete_dsp_role_review=False, reference_archive=oracle_record, ownership=ownership,
                  rust_archive_sha256=hashlib.sha256(archive.read_bytes()).hexdigest(), cases=results[1])
    (work / 'summary.json').write_text(json.dumps(record, indent=2) + '\n')
    print('243 original convolution cases compare six Rust subjects with isolated C oracles')


if __name__ == '__main__':
    main()
