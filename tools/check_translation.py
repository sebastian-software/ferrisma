#!/usr/bin/env python3
"""Verify all recorded Rust hashes and account for every upstream library C file."""
import hashlib
import json
import re
from pathlib import Path
import subprocess

from aom_jumps import NONLOCAL_CALL

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
        if 'production_rust_sha256' in unit:
            production = target.read_text().split('\n#[cfg(feature = "port-test-bridges")]', 1)[0]
            if hashlib.sha256(production.encode()).hexdigest() != unit['production_rust_sha256']:
                raise SystemExit(f'Production translation body differs from its manifest: {target}')
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
    names = ['TRANSLATION.json', 'TRANSLATION-release.json']
    if (dav1d.parent / 'TRANSLATION-linux.json').exists():
        names += ['TRANSLATION-linux.json', 'TRANSLATION-linux-release.json']
    for name in names:
        units = json.loads((dav1d.parent / name).read_text())['units']
        actual = {(unit['instantiation'], Path(unit['source']).name) for unit in units}
        if actual != expected or len(units) != len(expected):
            raise SystemExit(f'Incomplete portable dav1d source instantiations: {name}')
    print(f'All {len(expected)} portable dav1d source instantiations are represented in {len(names)} platform/build profiles')
aom = ROOT / 'crates/aom'
if (aom / 'TRANSLATION-macos-debug.json').exists():
    reference = json.loads((ROOT / 'docs/aom-reference-macos-debug.json').read_text())
    expected = {name.replace('<generated>/', 'generated/') for name in reference['library_sources']}
    for path in sorted(aom.glob('TRANSLATION-*.json')):
        record = json.loads(path.read_text())
        platform, profile = path.stem.removeprefix('TRANSLATION-').split('-')
        actual = []
        for unit in record['units']:
            if NONLOCAL_CALL.search((ROOT / unit['rust']).read_text()):
                raise SystemExit(f'Unmapped libc nonlocal error transfer: {unit["rust"]}')
            source = Path(unit['source'])
            prefix = 'upstream/aom' if source.is_relative_to('upstream/aom') else f'crates/aom/reference/{platform}/{profile}'
            actual.append(str(source.relative_to(prefix)))
        for correction, expected_count in [('scoped-nonlocal-error-targets', 32),
                                            ('typed-nonlocal-error-transfer', 1),
                                            ('unaligned-external-highbd-validation-load', 1)]:
            if sum(unit['corrections'].get(correction, 0) for unit in record['units']) != expected_count:
                raise SystemExit(f'Incomplete libaom translation correction: {path}: {correction}')
        if set(actual) != expected or len(actual) != len(expected):
            raise SystemExit(f'Incomplete portable libaom source closure: {path}')
        for name, digest in record['generated_reference_headers'].items():
            header = aom / f'reference/{platform}/{profile}' / name
            if hashlib.sha256(header.read_bytes()).hexdigest() != digest:
                raise SystemExit(f'Generated libaom reference header changed: {header}')
        config = (aom / f'reference/{platform}/{profile}/config/aom_config.h').read_text()
        for setting in ['CONFIG_AV1_ENCODER 1', 'CONFIG_AV1_DECODER 1', 'CONFIG_AV1_HIGHBITDEPTH 1',
                        'CONFIG_MULTITHREAD 1', 'CONFIG_REALTIME_ONLY 0', 'CONFIG_TUNE_VMAF 0']:
            if f'#define {setting}\n' not in config:
                raise SystemExit(f'Incomplete libaom configuration: {path}: {setting}')
    print(f'All {len(expected)} complete portable libaom translation units are represented in {len(list(aom.glob("TRANSLATION-*.json")))} profiles')
