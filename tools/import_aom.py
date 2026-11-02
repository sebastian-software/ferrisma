#!/usr/bin/env python3
"""Import the complete pinned libaom source, including its original test suite."""
import argparse
import hashlib
import io
import json
from pathlib import Path
import subprocess
import tarfile

ROOT = Path(__file__).resolve().parents[1]
REVISION = 'af3dc9aadc793c1b00edc003e3fc19a44fdeb574'
REPOSITORY = 'https://aomedia.googlesource.com/aom'


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--fetch', action='store_true')
    args = parser.parse_args()
    source = ROOT / 'build/deps/aom'
    if not source.exists():
        if not args.fetch:
            raise SystemExit('Missing original checkout; use --fetch')
        subprocess.run(['git', 'clone', '--depth=1', '--branch', 'v3.15.2',
                        REPOSITORY, source], check=True)
    actual = subprocess.check_output(['git', '-C', source, 'rev-parse', 'HEAD'], text=True).strip()
    if actual != REVISION:
        raise SystemExit(f'Unexpected source revision: {actual}')
    archive = subprocess.check_output(['git', '-C', source, 'archive', REVISION])
    files = {}
    with tarfile.open(fileobj=io.BytesIO(archive)) as snapshot:
        for member in snapshot.getmembers():
            if member.isdir():
                continue
            if not member.isfile():
                raise SystemExit(f'Unexpected non-file source entry: {member.name}')
            content = snapshot.extractfile(member).read()
            target = ROOT / 'upstream/aom' / member.name
            if target.exists() and target.read_bytes() != content:
                raise SystemExit(f'Refusing to replace modified source: {target}')
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes(content)
            files[member.name] = hashlib.sha256(content).hexdigest()
    record = dict(component='libaom 3.15.2', repository=REPOSITORY,
                  revision=REVISION, tag='v3.15.2', source_root='upstream/aom',
                  crate='crates/aom', license='BSD-2-Clause AND BSD-3-Clause AND MIT',
                  license_files=['LICENSE', 'PATENTS', 'AUTHORS',
                                 'third_party/fastfeat/LICENSE',
                                 'third_party/vector/LICENSE'], files=dict(sorted(files.items())))
    (ROOT / 'provenance/aom-source.json').write_text(json.dumps(record, indent=2) + '\n')
    print(f'Pinned libaom 3.15.2: {len(files)} original files at {REVISION}')


if __name__ == '__main__':
    main()
