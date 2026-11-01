#!/usr/bin/env python3
"""Check every pinned upstream file and reject missing or extra files."""
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
manifest = json.loads((ROOT / "UPSTREAM.json").read_text())
source = ROOT / "upstream/libavif"
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
    raise SystemExit("Upstream snapshot differs:\n" + "\n".join(differences))
print(f"Verified {len(actual)} upstream files at {manifest['revision']}")
