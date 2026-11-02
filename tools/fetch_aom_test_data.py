#!/usr/bin/env python3
"""Fetch original libaom fixtures and verify the upstream SHA-1 manifest."""
import argparse
from concurrent.futures import ThreadPoolExecutor
import hashlib
import json
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[1]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--profile', choices=['debug', 'release'], default='debug')
    parser.add_argument('--jobs', type=int, default=4)
    args = parser.parse_args()
    selected = json.loads((ROOT / f'build/aom-native-{args.profile}/test-data.json').read_text())
    original = dict(line.split(' *')[::-1] for line in
                    (ROOT / 'upstream/aom/test/test-data.sha1').read_text().splitlines())
    if any(original.get(name) != digest for name, digest in selected.items()):
        raise SystemExit('Selected test data differs from the pinned upstream manifest')
    target = ROOT / 'build/deps/aom-test-data'
    target.mkdir(parents=True, exist_ok=True)

    def fetch(item):
        name, expected = item
        path = target / name
        if not path.exists() or hashlib.sha1(path.read_bytes()).hexdigest() != expected:
            temporary = path.with_suffix(path.suffix + '.download')
            subprocess.run(['curl', '--fail', '--location', '--silent', '--show-error',
                            '--retry', '3', '--output', temporary,
                            f'https://storage.googleapis.com/aom-test-data/{name}'], check=True)
            if hashlib.sha1(temporary.read_bytes()).hexdigest() != expected:
                raise SystemExit(f'Upstream test fixture checksum mismatch: {name}')
            temporary.replace(path)
        content = path.read_bytes()
        return name, dict(sha1=expected, sha256=hashlib.sha256(content).hexdigest(), size=len(content))

    files = {}
    with ThreadPoolExecutor(max_workers=args.jobs) as executor:
        for name, value in executor.map(fetch, selected.items()):
            files[name] = value
            if len(files) % 50 == 0:
                print(f'Verified {len(files)}/{len(selected)} original fixtures', flush=True)
    record = dict(repository='https://storage.googleapis.com/aom-test-data',
                  source_manifest='upstream/aom/test/test-data.sha1',
                  source_root='build/deps/aom-test-data', files=files)
    (target / 'verified-inputs.json').write_text(json.dumps(record, indent=2) + '\n')
    print(f'Verified {len(files)} original fixtures, {sum(v["size"] for v in files.values())} bytes')


if __name__ == '__main__':
    main()
