#!/usr/bin/env python3
"""Verify all recorded Rust hashes and account for every upstream library C file."""
import hashlib
import json
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[1]
subprocess.run(["python3", str(ROOT / "tools/verify_upstream.py")], check=True)
manifests = [ROOT / "TRANSLATION.json", *sorted((ROOT / "translations").glob("*.json")),
             *sorted((ROOT / "crates").glob("*/TRANSLATION.json"))]
sources, outputs = set(), set()
for path in manifests:
    for unit in json.loads(path.read_text()).get("units", []):
        source, target = ROOT / unit["source"], ROOT / unit["rust"]
        if not source.is_file() or not target.is_file():
            raise SystemExit(f"Missing translation input/output in {path.name}: {unit}")
        if hashlib.sha256(target.read_bytes()).hexdigest() != unit["rust_sha256"]:
            raise SystemExit(f"Translated file differs from its manifest: {target}")
        sources.add(source.resolve())
        outputs.add(target.resolve())
expected = set((ROOT / "upstream/libavif/src").glob("*.c"))
missing = expected - sources
if missing:
    raise SystemExit("Untranslated library sources: " + ", ".join(str(path) for path in sorted(missing)))
print(f"All {len(expected)} libavif C source files are represented; verified {len(outputs)} Rust implementation files")
sharp = set((ROOT / "upstream/libwebp/sharpyuv").glob("*.c"))
if sharp - sources:
    raise SystemExit("Untranslated portable SharpYUV sources")
print(f"All {len(sharp)} portable SharpYUV translation units are represented")
