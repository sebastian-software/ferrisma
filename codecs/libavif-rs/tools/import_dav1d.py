#!/usr/bin/env python3
"""Import the pinned original dav1d source and record its original test inputs."""
import argparse
import hashlib
import json
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[1]
DEPENDENCIES = {
    'dav1d': ('54706fc6bc0cdecab7e9593974a4039cc038fca7', '1.5.4'),
    'checkasm': ('0df02535c7435cf3969ca141c9e3ff7b1c1e6c28', 'v1.2.0'),
    'dav1d-test-data': ('61afa1ceb6029be1a8ea3f6b9c9a9672700f13bb', None),
}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--fetch', action='store_true')
    args = parser.parse_args()
    records = {}
    for name, (revision, tag) in DEPENDENCIES.items():
        source = ROOT / 'build/deps' / name
        repository = f'https://code.videolan.org/videolan/{name}.git'
        if not source.exists():
            if not args.fetch:
                raise SystemExit(f'Missing {source}; use --fetch')
            source.parent.mkdir(parents=True, exist_ok=True)
            subprocess.run(['git', 'init', source], check=True)
            subprocess.run(['git', '-C', source, 'fetch', '--depth=1', repository, revision], check=True)
            subprocess.run(['git', '-C', source, 'checkout', '--detach', 'FETCH_HEAD'], check=True)
        actual = subprocess.check_output(['git', '-C', source, 'rev-parse', 'HEAD'], text=True).strip()
        if actual != revision:
            raise SystemExit(f'Unexpected {name} revision: {actual}')
        names = subprocess.check_output(['git', '-C', source, 'ls-files', '-z']).decode().rstrip('\0').split('\0')
        files = {}
        for relative in names:
            original = subprocess.check_output(['git', '-C', source, 'show', f'{revision}:{relative}'])
            if (source / relative).read_bytes() != original:
                raise SystemExit(f'Modified original input: {name}/{relative}')
            files[relative] = hashlib.sha256(original).hexdigest()
            if name == 'dav1d':
                target = ROOT / 'upstream/dav1d' / relative
                if target.exists() and target.read_bytes() != original:
                    raise SystemExit(f'Refusing to overwrite {target}')
                target.parent.mkdir(parents=True, exist_ok=True)
                target.write_bytes(original)
        record = {'repository': repository, 'revision': revision, 'tag': tag, 'files': files}
        if name == 'dav1d':
            record.update(component='dav1d 1.5.4', source_root='upstream/dav1d',
                          crate='crates/dav1d', license='BSD-2-Clause',
                          license_files=['COPYING', 'doc/PATENTS'])
            output = ROOT / 'provenance/dav1d-source.json'
            output.write_text(json.dumps(record, indent=2) + '\n')
        else:
            record['source_root'] = f'build/deps/{name}'
            records[name] = record
        print(f'Pinned {name}: {len(files)} original files at {revision}')
    (ROOT / 'provenance/dav1d-test-inputs.json').write_text(json.dumps(records, indent=2) + '\n')


if __name__ == '__main__':
    main()
