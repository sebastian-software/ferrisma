#!/usr/bin/env python3
"""Import hash-verified native Linux generation and apply the current VaList spelling."""
import argparse
import hashlib
import json
from pathlib import Path

from transpile_dav1d import current_variadic

ROOT = Path(__file__).resolve().parents[1]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('artifact', type=Path)
    args = parser.parse_args()
    source = args.artifact / 'crates/dav1d'
    destination = ROOT / 'crates/dav1d'
    for suffix in ['-linux', '-linux-release']:
        record = json.loads((source / f'TRANSLATION{suffix}.json').read_text())
        if record['generation_target'] != 'x86_64-unknown-linux-gnu' or len(record['units']) != 51:
            raise SystemExit('Unexpected native Linux source closure')
        for unit in record['units']:
            relative = Path(unit['rust'])
            if not relative.as_posix().startswith('crates/dav1d/src/linux/'):
                raise SystemExit('Unexpected output path in native artifact')
            path = args.artifact / relative
            text = path.read_text()
            if hashlib.sha256(text.encode()).hexdigest() != unit['rust_sha256']:
                raise SystemExit(f'Artifact output differs: {relative}')
            text, count = current_variadic(text)
            if count:
                unit['corrections']['current-rust-variadic-list'] = count
                unit['rust_sha256'] = hashlib.sha256(text.encode()).hexdigest()
            target = ROOT / relative
            if target.exists() and target.read_text() != text:
                raise SystemExit(f'Refusing to overwrite a different Linux profile: {relative}')
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_text(text)
        (destination / f'TRANSLATION{suffix}.json').write_text(json.dumps(record, indent=2) + '\n')
        module = f'src/modules{suffix}.rs'
        (destination / module).write_bytes((source / module).read_bytes())
    print('Imported 102 native Linux translation units with verified artifact hashes')


if __name__ == '__main__':
    main()
