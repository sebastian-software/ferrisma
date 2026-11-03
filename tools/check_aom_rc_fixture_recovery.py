#!/usr/bin/env python3
"""Classify only the original rate-control missing-video failures and retain proof."""
import argparse
import hashlib
import json
from pathlib import Path
import platform
import re
import shutil
import subprocess
import xml.etree.ElementTree as ET

from build_aom import test_names
from plan_aom_release import source_pair

ROOT = Path(__file__).resolve().parents[1]
VIDEOS = {'niklas_640_480_30.yuv', 'hantro_collage_w352h288.yuv'}


def digest(path):
    value = hashlib.sha256()
    with path.open('rb') as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b''):
            value.update(chunk)
    return value.hexdigest()


def validate_failure(directory, names, identity):
    record = json.loads((directory / 'shard-0.json').read_text())
    if (record['identity'] != identity or identity['program'] != 'test_aom_rc'
            or identity['profile'] != 'release' or identity['shards'] != 1
            or identity['filter'] != '*' or record['index'] != 0 or record['valid']):
        raise ValueError('Not the failed original Release rate-control pair')
    expected_failures = json.loads((ROOT / 'docs/aom-rate-control-fixture-failure.json').read_text())['failed_case_names']
    for side in ('c', 'rust'):
        result = record['results'][side]
        path = directory / f'shard-0-{side}.xml'
        document = ET.parse(path).getroot()
        # xml_cases deliberately rejects failures; preserve them here for classification.
        cases = [c for c in document.iter('testcase') if c.get('status') == 'run']
        if (result['exit_code'] != 1 or result['valid_xml'] or result['xml_errors']
                or document.get('errors') != '0' or int(document.get('failures', -1)) != 16
                or [c.attrib['classname'] + '.' + c.attrib['name'] for c in cases] != names
                or [c.attrib['classname'] + '.' + c.attrib['name'] for c in cases if c.findall('failure')] != expected_failures
                or any(c.findall('skipped') for c in cases)):
            raise ValueError('Failure differs from the complete original missing-fixture boundary')
        actual = [dict(name=c.attrib['classname'] + '.' + c.attrib['name'], result=c.get('result'),
                       failed=bool(c.findall('failure')), skipped=False) for c in cases]
        if actual != result['cases']:
            raise ValueError('Failed report differs from actual original XML')
        videos = set()
        for case in cases:
            failures = case.findall('failure')
            if not failures:
                continue
            messages = [f.attrib.get('message', '') for f in failures]
            opening = [m for m in messages if re.fullmatch(
                r'upstream/aom/test/yuv_video_source.h:47\nExpected: \(input_file_\) != \(nullptr\), actual: NULL vs \(nullptr\)\nInput file open failed\. Filename: (niklas_640_480_30\.yuv|hantro_collage_w352h288\.yuv)\n', m)]
            propagation = [m for m in messages if re.fullmatch(
                r"upstream/aom/test/(encode_test_driver\.cc:194\nExpected: video->Begin\(\)|ratectrl_rtc_test\.cc:(210|226|245|260|301|314|328|342)\nExpected: RunLoop\(&video\)) doesn't generate new fatal failures in the current thread\.\n  Actual: it does\.\n", m)]
            if (len(failures) != 3 or len(opening) != 1 or len(propagation) != 2
                    or sum(m.startswith('upstream/aom/test/encode_test_driver.cc:194\n') for m in propagation) != 1):
                raise ValueError('A codec assertion or another failure cannot be recovered as missing data')
            videos.add(opening[0].split('Filename: ')[1].strip())
        if videos != VIDEOS:
            raise ValueError('Unexpected original missing video inventory')
    if record['results']['c']['cases'] != record['results']['rust']['cases']:
        raise ValueError('C and Rust did not fail the same original cases')
    return record


def api(path):
    return json.loads(subprocess.check_output(['gh', 'api',
        'repos/sebastian-software/libavif-rs/' + path], text=True))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--source-run', required=True, type=int)
    parser.add_argument('--continuation-run', required=True, type=int)
    parser.add_argument('--runner', required=True, choices=['ubuntu-24.04', 'macos-26', 'macos-15'])
    parser.add_argument('--verify-success', action='store_true')
    args = parser.parse_args()
    if platform.machine() != ('arm64' if args.runner.startswith('macos-') else 'x86_64'):
        raise ValueError('Recovery must execute on the original native architecture')
    source = api(f'actions/runs/{args.source_run}')
    continuation = api(f'actions/runs/{args.continuation_run}')
    if (source['path'].split('@')[0] != '.github/workflows/aom-original-native.yml'
            or continuation['path'].split('@')[0] != '.github/workflows/aom-original-release.yml'):
        raise ValueError('Recovery source workflows differ from the original contract')
    build_jobs = api(f'actions/runs/{args.source_run}/jobs?per_page=1')['jobs']
    if not any(job['name'] == 'build' and job['conclusion'] == 'success'
               and args.runner in job['labels'] for job in build_jobs):
        raise ValueError('Require a successful frozen source build on the same native runner')
    failed = ROOT / 'build/aom-rc-fixture-failure'
    plan = json.loads((failed / 'aom-release-source-plan.json').read_text())
    if (plan['source_run'] != args.source_run or plan['source_commit'] != source['head_sha']
            or plan['runner'] != args.runner):
        raise ValueError('Recovery uses another native source or runner')
    native, rust = ROOT / 'build/aom-native-release', ROOT / 'build/aom-rust-release'
    original = failed / 'aom-paired-release-test_aom_rc'
    identity = json.loads((original / 'shard-0.json').read_text())['identity']
    expected_hashes = {'c_executable_sha256': digest(native / 'test_aom_rc'),
        'rust_executable_sha256': digest(rust / 'test_aom_rc'),
        'config_sha256': digest(native / 'config/aom_config.h'),
        'translation_manifest_sha256': digest(ROOT / ('crates/aom/TRANSLATION-' +
            ('macos' if args.runner.startswith('macos-') else 'linux') + '-release.json'))}
    if any(identity.get(key) != value for key, value in expected_hashes.items()):
        raise ValueError('Recovery must use the exact failed frozen binaries and configuration')
    names = test_names((native / 'test_aom_rc-inventory.txt').read_text())
    if len(names) != 38:
        raise ValueError('Original rate-control inventory changed')
    validate_failure(original, names, identity)
    report = dict(source_run=args.source_run, source_commit=source['head_sha'],
        continuation_run=args.continuation_run, runner=args.runner, identity=identity,
        classified_failure='Only original file-open assertions and their fatal-failure propagation',
        failed_actual_files_sha256={str(p.relative_to(failed)): digest(p)
            for p in failed.rglob('*') if p.is_file()}, repair_complete=False,
        complete_native_acceptance=False, original_cases=38)
    if args.verify_success:
        previous = json.loads((ROOT / 'build/aom-rc-fixture-recovery.json').read_text())
        if previous != report:
            raise ValueError('Original failure evidence changed during recovery')
        directory = ROOT / 'build/aom-paired-release-test_aom_rc'
        recovered = source_pair(directory, 'test_aom_rc', 0)
        if (recovered is None or recovered['identity'] != identity
                or [c['name'] for c in recovered['results']['c']['cases']] != names
                or any(c['skipped'] for c in recovered['results']['c']['cases'])):
            raise ValueError('All 38 unchanged original C/Rust cases must pass')
        fixtures = json.loads((ROOT / 'build/deps/aom-test-data/verified-inputs.json').read_text())
        required = json.loads((native / 'test-data.json').read_text())
        if not fixtures['complete_selected_corpus'] or set(fixtures['files']) != set(required):
            raise ValueError('Require the complete verified original fixture corpus')
        for name in VIDEOS:
            record = fixtures['files'][name]
            if (record['sha1'] != required[name] or digest(ROOT / 'build/deps/aom-test-data' / name) != record['sha256']):
                raise ValueError('Original rate-control fixture content changed')
        target = ROOT / 'build/aom-rc-fixture-verified-inputs.json'
        shutil.copyfile(ROOT / 'build/deps/aom-test-data/verified-inputs.json', target)
        report.update(repair_complete=True, fixture_provenance_sha256=digest(target),
            recovered_actual_files_sha256={str(p.relative_to(ROOT / 'build')): digest(p)
                for p in directory.iterdir() if p.is_file()})
    (ROOT / 'build/aom-rc-fixture-recovery.json').write_text(json.dumps(report, indent=2) + '\n')


if __name__ == '__main__':
    main()
