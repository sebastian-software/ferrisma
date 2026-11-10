#!/usr/bin/env python3
"""Copy only hash-pinned passing source pairs; retain partial originals in staging."""
import argparse
import hashlib
import json
from pathlib import Path, PurePosixPath

from plan_aom_release import PROGRAMS, source_pair

ROOT = Path(__file__).resolve().parents[1]


def digest(content):
    return hashlib.sha256(content).hexdigest()


def project_source(staging, root, plan):
    selected = {(item['program'], item['index']) for item in plan['reused_partitions']}
    if len(selected) != len(plan['reused_partitions']):
        raise ValueError('Duplicate source partitions in the reuse plan')
    passing, expected, copies = set(), set(), {}
    for program, count in PROGRAMS.items():
        directory = staging / f'aom-paired-release-{program}'
        for index in range(count):
            record = source_pair(directory, program, index)
            if record is None:
                continue
            passing.add((program, index))
            prefix = f'build/aom-paired-release-{program}/shard-{index}'
            expected.add(prefix + '.json')
            expected.update(prefix + '-' + side + '.' + suffix
                            for side in ('c', 'rust') for suffix in ('xml', 'log'))
            if program == 'test_libaom':
                path = directory / 'fixtures.json'
                fixture = path.read_bytes()
                files = json.loads(fixture)['files']
                canonical = digest(json.dumps(files, sort_keys=True, separators=(',', ':')).encode())
                if canonical != record['identity']['verified_fixtures_sha256']:
                    raise ValueError('Original fixture inventory differs from its passing source identity')
                copies['build/aom-paired-release-test_libaom/fixtures.json'] = fixture
    if passing != selected or set(plan['reused_files_sha256']) != expected:
        raise ValueError('Reuse plan must preserve every passing source pair and its complete evidence')
    for name, expected_hash in plan['reused_files_sha256'].items():
        relative = PurePosixPath(name)
        if relative.is_absolute() or '..' in relative.parts or relative.parts[0] != 'build':
            raise ValueError('Original evidence path escapes the selected build directory')
        source = (staging / Path(*relative.parts[1:])).resolve()
        if staging.resolve() not in source.parents:
            raise ValueError('Source evidence resolves outside staging')
        content = source.read_bytes()
        if digest(content) != expected_hash:
            raise ValueError('Original passing evidence changed after planning')
        copies[name] = content
    for name, content in copies.items():
        target = (root / name).resolve()
        if root.resolve() not in target.parents or (target.exists() and target.read_bytes() != content):
            raise ValueError('Existing original evidence must remain unchanged')
    for name, content in copies.items():
        target = root / name
        target.parent.mkdir(parents=True, exist_ok=True)
        if not target.exists():
            target.write_bytes(content)
    return dict(source_run=plan['source_run'], passing_source_partitions=len(passing),
                copied_actual_files_sha256={name: digest(content) for name, content in copies.items()},
                incomplete_original_evidence_retained_in_staging=True)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--staging', type=Path, default=ROOT / 'build/aom-source-release-evidence')
    parser.add_argument('--source-plan', type=Path, default=ROOT / 'build/aom-release-source-plan.json')
    args = parser.parse_args()
    report = project_source(args.staging, ROOT, json.loads(args.source_plan.read_text()))
    (ROOT / 'build/aom-source-release-projection.json').write_text(json.dumps(report, indent=2) + '\n')
    print(f'Projected {report["passing_source_partitions"]} unchanged passing source pairs; partial originals remain separate')


if __name__ == '__main__':
    main()
