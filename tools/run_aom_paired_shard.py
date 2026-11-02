#!/usr/bin/env python3
"""Run or collect matching original C/Rust GoogleTest shards without changing tests."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import xml.etree.ElementTree as ET

from build_aom import test_names

ROOT = Path(__file__).resolve().parents[1]
PROGRAMS = ['test_libaom', 'test_aom_rc', 'test_intra_pred_speed']


def digest(path):
    value = hashlib.sha256()
    with path.open('rb') as source:
        for chunk in iter(lambda: source.read(1024 * 1024), b''):
            value.update(chunk)
    return value.hexdigest()


def disabled(name):
    return any(part.startswith('DISABLED_') for part in name.replace('.', '/').split('/'))


def inventory(binary, filter_value=None):
    command = [str(binary), '--gtest_list_tests']
    if filter_value is not None:
        command.append('--gtest_filter=' + filter_value)
    listing = subprocess.check_output(command, text=True)
    # GoogleTest prints this diagnostic before a filtered listing.
    lines = [line for line in listing.splitlines()
             if not line.startswith('Note: Google Test filter = ')]
    return test_names('\n'.join(lines))


def execute(binary, label, args, work, native):
    xml = work / f'shard-{args.index}-{label}.xml'
    xml.unlink(missing_ok=True)
    environment = dict(os.environ, GTEST_TOTAL_SHARDS=str(args.shards), GTEST_SHARD_INDEX=str(args.index),
                       LIBAOM_TEST_DATA_PATH=str(args.fixtures))
    command = [str(binary), '--gtest_filter=' + args.filter, '--gtest_output=xml:' + str(xml)]
    with (work / f'shard-{args.index}-{label}.log').open('w') as log:
        result = subprocess.run(command, cwd=native, env=environment, stdout=log, stderr=subprocess.STDOUT)
    cases, valid_xml, xml_errors = [], False, []
    if xml.exists():
        try:
            document = ET.parse(xml).getroot()
            valid_xml = not int(document.get('errors', 0)) and not int(document.get('failures', 0))
            for case in document.iter('testcase'):
                if case.get('status') == 'run':
                    cases.append(dict(name=case.attrib['classname'] + '.' + case.attrib['name'],
                        result=case.get('result'), failed=bool(case.findall('failure')),
                        skipped=bool(case.findall('skipped'))))
        except (ET.ParseError, ValueError) as error:
            xml_errors.append(str(error))
    return dict(exit_code=result.returncode, valid_xml=valid_xml, xml_errors=xml_errors, cases=cases)


def passing_pair(record):
    c, rust = record['results']['c'], record['results']['rust']
    return (all(not value['exit_code'] and value['valid_xml']
                and not any(case['failed'] for case in value['cases']) for value in [c, rust])
            and c['cases'] == rust['cases'])


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--debug', action='store_true')
    parser.add_argument('--program', choices=PROGRAMS, default='test_libaom')
    parser.add_argument('--shards', type=int, default=32)
    parser.add_argument('--index', type=int)
    parser.add_argument('--collect', action='store_true')
    parser.add_argument('--resume', action='store_true', help='Reuse only an identical, passing paired shard')
    parser.add_argument('--filter', default='*')
    parser.add_argument('--output-dir', type=Path)
    parser.add_argument('--reference-build', type=Path, help='Read an existing native reference without modifying it')
    parser.add_argument('--fixtures', type=Path, default=ROOT / 'build/deps/aom-test-data')
    args = parser.parse_args()
    if args.shards < 1 or (not args.collect and (args.index is None or not 0 <= args.index < args.shards)):
        parser.error('Provide a positive shard count and one valid index, or use --collect')
    if args.collect and args.index is not None:
        parser.error('--collect and --index are mutually exclusive')
    if args.collect and args.resume:
        parser.error('--collect and --resume are mutually exclusive')
    profile = 'debug' if args.debug else 'release'
    native = (args.reference_build or ROOT / f'build/aom-native-{profile}').resolve()
    rust = ROOT / f'build/aom-rust-{profile}'
    args.fixtures = args.fixtures.resolve()
    platform = 'macos' if sys.platform == 'darwin' else 'linux'
    manifest = ROOT / f'crates/aom/TRANSLATION-{platform}-{profile}.json'
    translation = json.loads(manifest.read_text())
    if (json.loads((native / 'reference-profile.json').read_text())['profile'] != profile
            or digest(native / 'config/aom_config.h') != translation['generated_reference_headers']['config/aom_config.h']):
        raise SystemExit('The native reference and Rust preprocessing differ')
    suffix = '' if args.filter == '*' else '-focused-' + hashlib.sha256(args.filter.encode()).hexdigest()[:12]
    work = (args.output_dir or ROOT / f'build/aom-paired-{profile}-{args.program}{suffix}').resolve()
    work.mkdir(parents=True, exist_ok=True)
    (work / 'summary.json').unlink(missing_ok=True)
    binaries = dict(c=native / args.program, rust=rust / args.program)
    names = test_names((native / f'{args.program}-inventory.txt').read_text())
    if any(inventory(binary) != names for binary in binaries.values()):
        raise SystemExit('C/Rust binaries differ from the original registration inventory')
    selected = inventory(binaries['c'], args.filter)
    expected = [name for name in selected if not disabled(name)]
    linkage = {item['executable']: item for item in json.loads((rust / 'linkage.json').read_text())}.get(args.program)
    if linkage is None or 'executable_sha256' not in linkage:
        raise SystemExit('Rebuild this Rust consumer to record its executable hash before sharding')
    rust_digest = digest(binaries['rust'])
    if rust_digest != linkage['executable_sha256'] or not linkage['unchanged_original_test_objects']:
        raise SystemExit('Rust consumer does not match its original-object linkage record')
    audit = json.loads((rust / 'symbol-audit.json').read_text())
    owners = {item['executable']: item for item in audit['consumers']}.get(args.program)
    if (owners is None or audit['missing_original_globals'] or audit['original_globals_outside_rust']
            or owners['original_codec_objects_loaded'] or owners['original_globals_outside_rust']
            or linkage['rust_archive_sha256'] != audit['rust_archive_sha256']):
        raise SystemExit('Rust consumer is missing a passing codec symbol-ownership audit')
    fixtures_sha256, fixture_metadata = None, None
    if args.program == 'test_libaom' and args.filter == '*':
        fixtures = args.fixtures
        metadata = work / 'fixtures.json' if args.collect else fixtures / 'verified-inputs.json'
        verified = json.loads(metadata.read_text())['files']
        required = json.loads((native / 'test-data.json').read_text())
        if (set(verified) != set(required)
                or any(verified[name]['sha1'] != expected
                       or (not args.collect and (not (fixtures / name).is_file()
                           or (fixtures / name).stat().st_size != verified[name]['size']))
                       for name, expected in required.items())):
            raise SystemExit('The complete original selected fixture inventory is not verified/present')
        fixtures_sha256 = hashlib.sha256(json.dumps(verified, sort_keys=True, separators=(',', ':')).encode()).hexdigest()
        if not args.collect:
            fixture_metadata = metadata.read_bytes()
    identity = dict(profile=profile, program=args.program, shards=args.shards, filter=args.filter,
                    c_executable_sha256=digest(binaries['c']), rust_executable_sha256=rust_digest,
                    original_inventory_sha256=hashlib.sha256(json.dumps(names, separators=(',', ':')).encode()).hexdigest(),
                    config_sha256=digest(native / 'config/aom_config.h'),
                    translation_manifest_sha256=digest(manifest),
                    verified_fixtures_sha256=fixtures_sha256,
                    rust_archive_sha256=linkage['rust_archive_sha256'])
    if not args.collect:
        path = work / f'shard-{args.index}.json'
        expected_shard = expected[args.index::args.shards]
        if args.resume and path.exists():
            record = json.loads(path.read_text())
            if (record['identity'] != identity or record['index'] != args.index
                    or not record['valid'] or not passing_pair(record)
                    or [case['name'] for case in record['results']['c']['cases']] != expected_shard):
                raise SystemExit('Existing shard does not match the passing original execution contract; retained without changes')
            print(f'Reused verified {profile}/{args.program} shard {args.index + 1}/{args.shards}', flush=True)
            return
        if fixture_metadata is not None:
            (work / 'fixtures.json').write_bytes(fixture_metadata)
        path.unlink(missing_ok=True)
        record = dict(identity=identity, index=args.index, results={})
        for label, binary in binaries.items():
            record['results'][label] = execute(binary, label, args, work, native)
        c, candidate = record['results']['c'], record['results']['rust']
        record['valid'] = passing_pair(record) and [case['name'] for case in c['cases']] == expected_shard
        path.write_text(json.dumps(record, indent=2) + '\n')
        print(f'{profile}/{args.program} shard {args.index + 1}/{args.shards}: '
              f'C={len(c["cases"])}, Rust={len(candidate["cases"])}, valid={record["valid"]}', flush=True)
        if not record['valid']:
            raise SystemExit('Original C/Rust shard failed or changed its executed/skip inventory')
        return
    missing_shards, invalid_shards, cases = [], [], []
    for index in range(args.shards):
        path = work / f'shard-{index}.json'
        if not path.exists():
            missing_shards.append(index)
            continue
        record = json.loads(path.read_text())
        if (record['identity'] != identity or record['index'] != index
                or not record['valid'] or not passing_pair(record)):
            invalid_shards.append(index)
        cases.extend(record['results']['rust']['cases'])
    executed = [case['name'] for case in cases]
    missing, extra = sorted(set(expected) - set(executed)), sorted(set(executed) - set(expected))
    summary = dict(identity=identity, original_registrations=len(names), selected_registrations=len(selected),
                   original_active_selected=len(expected), upstream_disabled=sum(map(disabled, names)),
                   executed=len(executed), dynamic_skips=sum(case['skipped'] for case in cases),
                   failures=sum(case['failed'] for case in cases), missing_shards=missing_shards,
                   invalid_shards=invalid_shards, missing_cases=missing, unexpected_cases=extra,
                   duplicate_cases=len(executed) - len(set(executed)),
                   original_cases_parameters_and_disabled_flags_unchanged=True)
    valid = not any(summary[name] for name in ['missing_shards', 'invalid_shards', 'missing_cases',
                                              'unexpected_cases', 'duplicate_cases', 'failures'])
    summary['valid'] = valid
    (work / 'summary.json').write_text(json.dumps(summary, indent=2) + '\n')
    print(f'{profile}/{args.program}: {len(executed)}/{len(expected)} original active selected cases; '
          f'{summary["dynamic_skips"]} matching dynamic skips; valid={valid}')
    if not valid:
        raise SystemExit('The complete paired original-suite inventory is not passing')


if __name__ == '__main__':
    main()
