#!/usr/bin/env python3
"""Check every pinned upstream file and reject missing or extra files."""
import argparse
import hashlib
import json
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[1]
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--tracked', action='store_true', help='Also require every snapshot file in the Git index')
args = parser.parse_args()
tracked = set(subprocess.check_output(['git', 'ls-files', '-z'], cwd=ROOT).decode().split('\0')) if args.tracked else None
for record in [ROOT / "UPSTREAM.json", *sorted((ROOT / "provenance").glob("*-source.json"))]:
    manifest = json.loads(record.read_text())
    source = ROOT / manifest.get("source_root", "upstream/libavif")
    actual = {
        str(path.relative_to(source)): hashlib.sha256(path.read_bytes()).hexdigest()
        for path in source.rglob("*")
        if path.is_file()
    }
    expected = manifest["files"]
    differences = sorted(
        path for path in actual.keys() | expected.keys()
        if actual.get(path) != expected.get(path)
    )
    if differences:
        raise SystemExit(f"Upstream snapshot {source} differs:\n" + "\n".join(differences))
    if tracked is not None:
        missing = sorted(str((source / name).relative_to(ROOT)) for name in expected
                         if str((source / name).relative_to(ROOT)) not in tracked)
        if missing:
            raise SystemExit('Original snapshot files are absent from the Git index:\n' + '\n'.join(missing))
    print(f"Verified {len(actual)} upstream files at {manifest['revision']}")
