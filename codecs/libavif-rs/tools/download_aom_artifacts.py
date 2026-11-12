#!/usr/bin/env python3
"""Download every matching artifact through explicit GitHub API pagination.

Bulk downloads must traverse every API page. A composition once rejected eight
missing but passing main partitions because the download had enumerated only
the first 100 of 110 published artifacts. The listing therefore uses
`gh api --paginate`, requires distinct unexpired matches, checks every ZIP
against the digest GitHub publishes, preserves existing evidence, and records
per-file hashes. Hosted runner jobs end at six hours, which is why long
original shards are split finer (see `tools/aom_subshards.py`).
"""
import argparse
import fnmatch
import hashlib
import json
from pathlib import Path, PurePosixPath
import stat
import subprocess
import zipfile

ROOT = Path(__file__).resolve().parents[1]


def digest(path):
    value = hashlib.sha256()
    with path.open('rb') as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b''):
            value.update(chunk)
    return value.hexdigest()


def list_artifacts(repository, run, pattern):
    output = subprocess.check_output(['gh', 'api', '--paginate',
        f'repos/{repository}/actions/runs/{run}/artifacts?per_page=25',
        '--jq', '.artifacts[] | @json'], text=True)
    all_items = [json.loads(line) for line in output.splitlines()]
    selected = [item for item in all_items if fnmatch.fnmatchcase(item['name'], pattern)]
    if (not selected or any(item['expired'] for item in selected)
            or len({item['name'] for item in selected}) != len(selected)
            or len({item['id'] for item in selected}) != len(selected)):
        raise ValueError('Require distinct, unexpired matching artifacts from all API pages')
    return all_items, selected


def merge_archive(archive, destination):
    """Preserve existing evidence and validate all ZIP entries before any write."""
    destination = destination.resolve()
    files = {}
    with zipfile.ZipFile(archive) as zipped:
        for info in zipped.infolist():
            relative = PurePosixPath(info.filename)
            if (relative.is_absolute() or '..' in relative.parts or '\\' in info.filename
                    or not relative.parts):
                raise ValueError('Artifact path must remain inside the evidence directory')
            kind = stat.S_IFMT(info.external_attr >> 16)
            if kind not in (0, stat.S_IFREG, stat.S_IFDIR):
                raise ValueError('Artifacts must contain ordinary files, not links or devices')
            if info.is_dir():
                continue
            target = (destination / relative.as_posix()).resolve()
            if destination not in target.parents or target in files:
                raise ValueError('Artifact has an escaping or duplicate file path')
            content = zipped.read(info)
            if target.exists() and (not target.is_file() or target.read_bytes() != content):
                raise ValueError('Conflicting original evidence must remain unchanged')
            files[target] = content
    if not files:
        raise ValueError('Artifact contains no evidence files')
    hashes = {}
    for target, content in files.items():
        if not target.exists():
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes(content)
        hashes[target.relative_to(destination).as_posix()] = hashlib.sha256(content).hexdigest()
    return hashes


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--run', required=True, type=int)
    parser.add_argument('--pattern', required=True)
    parser.add_argument('--repository', default='sebastian-software/ferrisma')
    parser.add_argument('--destination', type=Path, default=ROOT / 'build')
    args = parser.parse_args()
    if args.run < 1:
        parser.error('Require a positive workflow run ID')
    all_items, selected = list_artifacts(args.repository, args.run, args.pattern)
    cache = ROOT / f'build/aom-artifact-downloads/{args.run}'
    cache.mkdir(parents=True, exist_ok=True)
    binary_api = ['gh', 'api']
    if '--allow-escape-sequences' in subprocess.check_output(['gh', 'api', '--help'], text=True):
        binary_api.append('--allow-escape-sequences')
    proof = []
    for artifact in selected:
        archive = cache / f'{artifact["id"]}.zip'
        temporary = archive.with_suffix('.download')
        with temporary.open('wb') as output:
            subprocess.run(binary_api + [f'repos/{args.repository}/actions/artifacts/{artifact["id"]}/zip'],
                           stdout=output, check=True)
        actual = digest(temporary)
        expected = artifact.get('digest')
        if expected is not None and expected != 'sha256:' + actual:
            raise ValueError('Downloaded ZIP differs from the GitHub artifact digest')
        if archive.exists() and digest(archive) != actual:
            raise ValueError('Previously preserved artifact ZIP changed')
        temporary.replace(archive)
        files = merge_archive(archive, args.destination)
        proof.append(dict(id=artifact['id'], name=artifact['name'], zip_sha256=actual,
                          github_digest=expected, files_sha256=files))
    record = dict(source_run=args.run, repository=args.repository, pattern=args.pattern,
                  api_artifacts_enumerated=len(all_items), matched_artifacts=len(selected),
                  downloaded_artifacts=len(proof), complete_matching_download=True, artifacts=proof)
    suffix = hashlib.sha256(args.pattern.encode()).hexdigest()[:12]
    (cache / f'manifest-{suffix}.json').write_text(json.dumps(record, indent=2) + '\n')
    print(f'Downloaded all {len(proof)} matching artifacts from {len(all_items)} paginated records')


if __name__ == '__main__':
    main()
