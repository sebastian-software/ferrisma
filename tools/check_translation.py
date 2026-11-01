#!/usr/bin/env python3
"""Verify all recorded Rust hashes and account for every upstream library C file."""
import hashlib
import json
import re
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[1]
subprocess.run(["python3", str(ROOT / "tools/verify_upstream.py")], check=True)
manifests = [ROOT / "TRANSLATION.json", *sorted((ROOT / "translations").glob("*.json")),
             *sorted((ROOT / "crates").glob("*/TRANSLATION*.json"))]
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
dav1d = ROOT / 'crates/dav1d/TRANSLATION.json'
if dav1d.exists():
    meson = (ROOT / 'upstream/dav1d/src/meson.build').read_text()
    common = re.search(r'libdav1d_sources = files\((.*?)\n\)', meson, re.S)[1]
    templates = re.search(r'libdav1d_tmpl_sources = files\((.*?)\n\)', meson, re.S)[1]
    expected = {('common', name) for name in re.findall(r"'([^']+\.c)'", common)}
    expected |= {(group, name) for group in ['b8', 'b16'] for name in re.findall(r"'([^']+\.c)'", templates)}
    for name in ['TRANSLATION.json', 'TRANSLATION-release.json']:
        units = json.loads((dav1d.parent / name).read_text())['units']
        actual = {(unit['instantiation'], Path(unit['source']).name) for unit in units}
        if actual != expected or len(units) != len(expected):
            raise SystemExit(f'Incomplete portable dav1d source instantiations: {name}')
    print(f'All {len(expected)} portable dav1d source instantiations are represented in both profiles')
