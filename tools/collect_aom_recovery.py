#!/usr/bin/env python3
"""Collect complete original AOM acceptance with verified timeout recovery."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess

from aom_subshards import passing_subshards
from merge_aom_subshards import xml_cases

ROOT = Path(__file__).resolve().parents[1]
EXPECTED = {'test_libaom': (18729, 17228, 1501), 'test_aom_rc': (38, 38, 0),
            'test_intra_pred_speed': (38, 0, 38)}


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def install_recovery(staging, root, source_run, seen):
    """Validate all recovery evidence before replacing incomplete source records."""
    build = staging / 'build'
    contract_path = build / 'aom-recovery-source-contract.json'
    contract = json.loads(contract_path.read_text())
    profile = contract['profile']
    if contract['source_run'] != source_run or profile not in ('debug', 'release'):
        raise ValueError('Recovery belongs to another original run or profile')
    indices = {int(index) for index in contract['indices']}
    if not indices or any(not 0 <= index < 120 for index in indices):
        raise ValueError('Invalid original recovery indices')
    directory = build / f'aom-paired-{profile}-test_libaom'
    if {p.name for p in directory.glob('shard-*.json')} != {f'shard-{i}.json' for i in indices}:
        raise ValueError('Recovery records differ from their confirmed time-limit contract')
    for index in indices:
        if (profile, index) in seen:
            raise ValueError('Overlapping recovery runs are not accepted')
        entry = contract['indices'][str(index)]
        if not any('exceeded the maximum execution time' in item['message'] for item in entry['annotations']):
            raise ValueError('Recovery has no confirmed original time-limit annotation')
        path = directory / f'shard-{index}.json'
        record = json.loads(path.read_text())
        identity = record['identity']
        if (identity['profile'] != profile or identity['program'] != 'test_libaom'
                or identity['shards'] != 120 or identity['filter'] != '*'
                or record['index'] != index or not passing_subshards(record, staging)):
            raise ValueError('Recovered original partition or actual XML is invalid')
        existing = root / 'build' / path.relative_to(build)
        if existing.exists():
            previous = json.loads(existing.read_text())
            if previous.get('valid'):
                raise ValueError('A passing source record must not be overwritten by recovery')
            if previous.get('identity') != identity:
                raise ValueError('Recovery differs from the original incomplete execution identity')
            if any(case['failed'] for side in previous['results'].values() for case in side['cases']):
                raise ValueError('Recovery cannot hide an explicit original test failure')
    copies = []
    for path in sorted(build.rglob('*')):
        if path.is_symlink():
            raise ValueError('Recovery evidence must contain ordinary files, not symlinks')
        if not path.is_file() or path == contract_path:
            continue
        relative = path.relative_to(build)
        if relative.parts[0] not in (f'aom-paired-{profile}-test_libaom',
                                    f'aom-recovery-{profile}-test_libaom', f'aom-native-{profile}'):
            raise ValueError('Unexpected recovery artifact path')
        if relative.parts[0] == f'aom-native-{profile}' and relative.as_posix() != f'aom-native-{profile}/reference-profile.json':
            raise ValueError('Recovery may not replace original binaries or inventories')
        target = root / 'build' / relative
        if target.exists() and target.read_bytes() != path.read_bytes():
            if relative.parts[0] != f'aom-paired-{profile}-test_libaom' or target.suffix != '.json' or not target.name.startswith('shard-'):
                raise ValueError('Conflicting original or recovery evidence; retained without changes')
        copies.append((path, target))
    for source, target in copies:
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(source, target)
    seen.update((profile, index) for index in indices)
    return dict(profile=profile, original_indices=sorted(indices),
                source_contract_sha256=digest(contract_path))


def verify_original_xml(root):
    checked = 0
    for path in sorted((root / 'build').glob('aom-paired-*/shard-*.json')):
        record = json.loads(path.read_text())
        if 'subshards' in record:
            if not passing_subshards(record, root):
                raise ValueError('Recovered report or individual XML changed after import')
        else:
            index = record['index']
            for side in ('c', 'rust'):
                if xml_cases(path.parent / f'shard-{index}-{side}.xml') != record['results'][side]['cases']:
                    raise ValueError('Original report differs from its actual test XML')
        checked += 1
    return checked


def api(repository, endpoint):
    return json.loads(subprocess.check_output(['gh', 'api', f'repos/{repository}/{endpoint}'], text=True))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--source-run', type=int, required=True)
    parser.add_argument('--recovery-run', type=int, action='append', default=[])
    parser.add_argument('--repository', default=os.environ.get('GITHUB_REPOSITORY', 'sebastian-software/libavif-rs'))
    args = parser.parse_args()
    if args.source_run < 1 or len(set(args.recovery_run)) != len(args.recovery_run):
        parser.error('Require a positive original source run and distinct recovery runs')
    source = api(args.repository, f'actions/runs/{args.source_run}')
    if source['path'].split('@', 1)[0] != '.github/workflows/aom-original-native.yml':
        raise ValueError('Source run is not the complete original native acceptance workflow')
    seen, recoveries = set(), []
    for run in args.recovery_run:
        metadata = api(args.repository, f'actions/runs/{run}')
        if (metadata['status'] != 'completed' or metadata['conclusion'] != 'success'
                or metadata['path'].split('@', 1)[0] != '.github/workflows/aom-original-recovery.yml'):
            raise ValueError('Recovery must have complete successful native execution and collection')
        endpoint = f'repos/{args.repository}/actions/runs/{run}/artifacts?per_page=100'
        lines = subprocess.check_output(['gh', 'api', '--paginate', endpoint, '--jq', '.artifacts[] | @json'], text=True)
        artifacts = [json.loads(line) for line in lines.splitlines()]
        final = [item for item in artifacts if item['name'] in
                 ('aom-original-recovered-debug', 'aom-original-recovered-release') and not item['expired']]
        if len(final) != 1:
            raise ValueError('Require one complete recovered artifact per recovery run')
        staging = ROOT / f'build/aom-collection-downloads/{run}'
        if staging.exists():
            raise ValueError('Recovery staging already exists; retain it and use a fresh collection checkout')
        subprocess.run(['gh', 'run', 'download', str(run), '--repo', args.repository,
                        '--name', final[0]['name'], '--dir', str(staging / 'build')], check=True)
        imported = install_recovery(staging, ROOT, args.source_run, seen)
        imported.update(run=run, execution_commit=metadata['head_sha'], artifact_id=final[0]['id'])
        destination = ROOT / f'build/aom-recovery-contracts/{run}.json'
        destination.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(staging / 'build/aom-recovery-source-contract.json', destination)
        recoveries.append(imported)
    checked = verify_original_xml(ROOT)
    results = []
    for profile in ('debug', 'release'):
        for program, (registered, active, disabled) in EXPECTED.items():
            command = ['python3', str(ROOT / 'tools/run_aom_paired_shard.py'), '--program', program,
                       '--shards', '120' if program == 'test_libaom' else '1', '--collect']
            if profile == 'debug':
                command.append('--debug')
            execution = subprocess.run(command, cwd=ROOT)
            path = ROOT / f'build/aom-paired-{profile}-{program}/summary.json'
            summary = json.loads(path.read_text()) if path.exists() else {}
            valid = (execution.returncode == 0 and summary.get('valid') is True
                     and summary.get('original_registrations') == registered
                     and summary.get('executed') == active and summary.get('upstream_disabled') == disabled)
            results.append(dict(profile=profile, program=program, valid=valid,
                                collector_exit_code=execution.returncode, summary=summary))
    valid = len(results) == 6 and all(item['valid'] for item in results)
    report = dict(valid=valid, source_run=args.source_run, source_commit=source['head_sha'],
                  source_workflow_status=source['status'], source_workflow_conclusion=source['conclusion'],
                  collection_commit=subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=ROOT, text=True).strip(),
                  recoveries=recoveries, original_and_recovered_shard_records_checked=checked,
                  complete_original_case_and_matching_skip_inventories_required=True,
                  root_default_engine_switched=False, profiles=results)
    (ROOT / 'build/aom-complete-acceptance.json').write_text(json.dumps(report, indent=2) + '\n')
    if not valid:
        raise SystemExit('Complete original native acceptance is not passing; no default switch is authorized by this evidence')


if __name__ == '__main__':
    main()
