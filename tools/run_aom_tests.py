#!/usr/bin/env python3
"""Run unchanged original libaom registrations with explicit shard inventories."""
import argparse
from concurrent.futures import ThreadPoolExecutor
import hashlib
import json
import os
from pathlib import Path
import subprocess
import xml.etree.ElementTree as ET

from build_aom import test_names

ROOT = Path(__file__).resolve().parents[1]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--debug', action='store_true')
    parser.add_argument('--rust', action='store_true')
    parser.add_argument('--jobs', type=int, default=4)
    parser.add_argument('--shards', type=int, default=32)
    parser.add_argument('--filter', default='*')
    args = parser.parse_args()
    profile = 'debug' if args.debug else 'release'
    original = ROOT / f'build/aom-native-{profile}'
    build = ROOT / f'build/aom-rust-{profile}' if args.rust else original
    executable = build / 'test_libaom'
    names = test_names(subprocess.check_output([executable, '--gtest_list_tests'], text=True))
    reference_names = test_names((original / 'test_libaom-inventory.txt').read_text())
    if names != reference_names:
        raise SystemExit('Candidate changed original test names, order, or parameters')
    suffix = 'tests' if args.filter == '*' else 'tests-focused-' + hashlib.sha256(args.filter.encode()).hexdigest()[:12]
    output = build / suffix
    output.mkdir(parents=True, exist_ok=True)
    environment = os.environ.copy()
    environment['LIBAOM_TEST_DATA_PATH'] = str(ROOT / 'build/deps/aom-test-data')

    def run(index):
        env = dict(environment, GTEST_TOTAL_SHARDS=str(args.shards), GTEST_SHARD_INDEX=str(index))
        xml = output / f'shard-{index}.xml'
        xml.unlink(missing_ok=True)
        command = [str(executable), f'--gtest_filter={args.filter}', f'--gtest_output=xml:{xml}']
        with (output / f'shard-{index}.log').open('w') as log:
            result = subprocess.run(command, cwd=original, env=env, stdout=log, stderr=subprocess.STDOUT)
        cases = []
        if xml.exists():
            for case in ET.parse(xml).getroot().iter('testcase'):
                cases.append(dict(name=case.attrib['classname'] + '.' + case.attrib['name'],
                                  status=case.attrib.get('status'), result=case.attrib.get('result'),
                                  seconds=float(case.attrib.get('time', 0)),
                                  failures=[entry.attrib.get('message', '') for entry in case.findall('failure')],
                                  skipped=[entry.attrib.get('message', '') for entry in case.findall('skipped')]))
        record = dict(shard=index, shards=args.shards, exit_code=result.returncode, command=command, cases=cases)
        (output / f'shard-{index}.json').write_text(json.dumps(record, indent=2) + '\n')
        print(f'Shard {index + 1}/{args.shards}: exit {result.returncode}, {len(cases)} XML cases', flush=True)
        return record

    with ThreadPoolExecutor(max_workers=args.jobs) as executor:
        records = list(executor.map(run, range(args.shards)))
    # GoogleTest repeats disabled, not-run registrations in each shard's XML.
    executed = [case for record in records for case in record['cases'] if case['status'] == 'run']
    if len(set(case['name'] for case in executed)) != len(executed):
        raise SystemExit('Duplicate executed tests across original shards')
    expected = [name for name in names if not any(part.startswith('DISABLED_')
                for part in name.replace('.', '/').split('/'))]
    missing = sorted(set(expected) - {case['name'] for case in executed}) if args.filter == '*' else []
    summary = dict(implementation='rust' if args.rust else 'original C', profile=profile,
                   filter=args.filter, registrations=len(names), executed=len(executed),
                   upstream_disabled=len(names) - len(expected), missing=missing,
                   failures=sum(bool(case['failures']) for case in executed),
                   skipped=sum(bool(case['skipped']) for case in executed),
                   unsuccessful_shards=[r['shard'] for r in records if r['exit_code']],
                   original_tests_unchanged=True, shard_count=args.shards)
    if args.rust:
        reference = original / suffix
        native_summary = json.loads((reference / 'summary.json').read_text())
        native_cases = [case for index in range(native_summary['shard_count'])
                        for case in json.loads((reference / f'shard-{index}.json').read_text())['cases']
                        if case['status'] == 'run']
        native_names = sorted(case['name'] for case in native_cases)
        rust_names = sorted(case['name'] for case in executed)
        if native_names != rust_names or native_summary['failures'] or native_summary['unsuccessful_shards']:
            raise SystemExit('Rust execution does not match a passing original case inventory')
        if sorted(case['name'] for case in native_cases if case['skipped']) != sorted(
                case['name'] for case in executed if case['skipped']):
            raise SystemExit('Original and Rust dynamic skip inventories differ')
        summary['matches_original_executed_inventory'] = True
    (output / 'summary.json').write_text(json.dumps(summary, indent=2) + '\n')
    print(json.dumps(summary, indent=2))
    if missing or summary['failures'] or summary['unsuccessful_shards']:
        raise SystemExit(1)


if __name__ == '__main__':
    main()
