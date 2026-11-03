#!/usr/bin/env python3
"""Reuse verified original Release pairs and schedule only missing partitions."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import platform
import subprocess

from merge_aom_subshards import xml_cases
from run_aom_paired_shard import passing_pair

ROOT = Path(__file__).resolve().parents[1]
PROGRAMS = {'test_libaom': 120, 'test_aom_rc': 1, 'test_intra_pred_speed': 1}


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def source_pair(directory, program, index):
    """Distinguish complete original evidence from cancellation or real failure."""
    for side in ('c', 'rust'):
        xml = directory / f'shard-{index}-{side}.xml'
        if xml.exists():
            if any(case['failed'] for case in xml_cases(xml)):
                raise ValueError('Original Release XML contains an explicit test failure')
        log = directory / f'shard-{index}-{side}.log'
        if log.exists() and any(line.startswith('[  FAILED  ]')
                                for line in log.read_text(errors='replace').splitlines()):
            raise ValueError('Original Release log contains an explicit test failure')
    path = directory / f'shard-{index}.json'
    if not path.exists():
        return None
    record = json.loads(path.read_text())
    if any(case['failed'] for result in record['results'].values() for case in result['cases']):
        raise ValueError('Original Release report contains an explicit test failure')
    if not record.get('valid'):
        exits = [result['exit_code'] for result in record['results'].values()]
        if not any(code in (-2, -15, 130, 143) for code in exits) or any(
                code not in (0, -2, -15, 130, 143) for code in exits):
            raise ValueError('Original Release process failed; diagnose before scheduling a rerun')
        return None
    identity = record['identity']
    if (identity['profile'] != 'release' or identity['program'] != program
            or identity['shards'] != PROGRAMS[program] or identity['filter'] != '*'
            or record['index'] != index or 'subshards' in record or not passing_pair(record)):
        raise ValueError('Original Release pair has an unexpected execution contract')
    for side in ('c', 'rust'):
        if xml_cases(directory / f'shard-{index}-{side}.xml') != record['results'][side]['cases']:
            raise ValueError('Original Release report differs from its actual XML')
    return record


def verify_reused(plan, root):
    for name, expected in plan['reused_files_sha256'].items():
        path = (root / name).resolve()
        if root.resolve() not in path.parents or digest(path) != expected:
            raise ValueError('A complete original Release result changed during continuation')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--source-run', type=int, required=True)
    parser.add_argument('--runner', choices=['ubuntu-24.04', 'macos-26', 'macos-15'], required=True)
    args = parser.parse_args()
    expected_host = 'arm64' if args.runner.startswith('macos-') else 'x86_64'
    if platform.machine() != expected_host:
        raise ValueError('Plan must validate source binaries on their original native architecture')
    repository = os.environ.get('GITHUB_REPOSITORY', 'sebastian-software/libavif-rs')
    source = json.loads(subprocess.check_output(['gh', 'api',
        f'repos/{repository}/actions/runs/{args.source_run}'], text=True))
    if source['path'].split('@', 1)[0] != '.github/workflows/aom-original-native.yml':
        raise ValueError('Require the original native source workflow')
    # This workflow creates the build before any dependent matrix jobs. Only
    # its native runner is needed here; acceptance is checked from actual XML.
    jobs = json.loads(subprocess.check_output(['gh', 'api',
        f'repos/{repository}/actions/runs/{args.source_run}/jobs?per_page=1'], text=True))['jobs']
    builds = [job for job in jobs if job['name'] == 'build' and job['conclusion'] == 'success']
    if len(builds) != 1 or args.runner not in builds[0]['labels']:
        raise ValueError('Require a successful source build on the same native runner')
    matrix, reused, files = [], [], {}
    for program, shards in PROGRAMS.items():
        directory = ROOT / f'build/aom-paired-release-{program}'
        for index in range(shards):
            record = source_pair(directory, program, index)
            if record is None:
                matrix.append(dict(program=program, shards=shards, index=index))
                continue
            # The established runner checks inventories, original parameters,
            # binary/config/manifest/fixture hashes, and Rust symbol ownership.
            subprocess.run(['python3', str(ROOT / 'tools/run_aom_paired_shard.py'),
                '--program', program, '--shards', str(shards), '--index', str(index), '--resume'],
                cwd=ROOT, check=True)
            reused.append(dict(program=program, index=index))
            for path in directory.glob(f'shard-{index}.*'):
                files[str(path.relative_to(ROOT))] = digest(path)
            for side in ('c', 'rust'):
                for suffix in ('xml', 'log'):
                    path = directory / f'shard-{index}-{side}.{suffix}'
                    if path.exists():
                        files[str(path.relative_to(ROOT))] = digest(path)
    report = dict(acceptance_contract='ADR-0048', source_run=args.source_run,
        source_commit=source['head_sha'], runner=args.runner,
        source_workflow_status=source['status'], source_workflow_conclusion=source['conclusion'],
        full_debug_required=False, matrix=dict(include=matrix), reused_partitions=reused,
        reused_files_sha256=files)
    path = ROOT / 'build/aom-release-source-plan.json'
    path.write_text(json.dumps(report, indent=2) + '\n')
    if os.environ.get('GITHUB_OUTPUT'):
        with open(os.environ['GITHUB_OUTPUT'], 'a') as output:
            print('matrix=' + json.dumps(report['matrix']), file=output)
            print('has_work=' + str(bool(matrix)).lower(), file=output)
    print(f'Reused {len(reused)} complete original Release pairs; scheduled {len(matrix)} missing pairs')


if __name__ == '__main__':
    main()
