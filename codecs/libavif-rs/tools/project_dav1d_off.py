#!/usr/bin/env python3
"""Project AVIF_CODEC_DAV1D=OFF onto the AVM variants (port ADR 0061).

The four AVM variants (`avm`, `mini-avm`, `extended-pixi-avm`,
`mini-extended-pixi-avm`) are regenerated only from a local macOS build with
AVM v1.0.0 (`tools/transpile.py --variant ...`), which no hosted workflow
provides. Upstream reads AVIF_CODEC_DAV1D in one place, the codec table of
`src/avif.c`, and every translated avif.rs takes that table from
`translations/codec-registry.json` (`tools/import_translation.py`). So the
DAV1D=OFF translation of an AVM avif.c differs from its DAV1D=ON translation
by exactly what the registry change and C2Rust's pruning of the then unused
declarations produce:

- the codec table is the registry's table,
- its array type (in the table and in `availableCodecsCount`) is the
  registry's array type,
- the two declarations `avifCodecCreateDav1d` and `avifCodecVersionDav1d`
  are gone,

and the `codec_dav1d.c` unit leaves the variant. This tool applies that rule
to the variant avif.rs files of `--from` (the last revision whose AVM
variants are DAV1D=ON translations). It refuses unless the same rule maps the
base avif.rs and every native root profile's avif.rs of that revision onto
the regenerated DAV1D=OFF files of the working tree: byte for byte, or for the
GNU Linux Debug profiles equal under `tools/transpile_root.comparable_profile`
(assertion file names of the generating checkout and unused assertion labels).

Each projected unit keeps its C2Rust `raw_sha256`, which is the hash of a
DAV1D=ON translation, and records that in `projection`. `--check` recomputes
the projection from the recorded revision and compares the tree; without the
revision in the local history it checks the projected structure only.
"""
import argparse
import hashlib
import json
from pathlib import Path
import re
import subprocess

from transpile_root import comparable_profile

ROOT = Path(__file__).resolve().parents[1]
VARIANTS = ['avm', 'mini-avm', 'extended-pixi-avm', 'mini-extended-pixi-avm']
DAV1D_UNIT = 'upstream/libavif/src/codec_dav1d.c'
DECLARATIONS = ['    fn avifCodecCreateDav1d() -> *mut avifCodec;\n',
                '    fn avifCodecVersionDav1d() -> *const core::ffi::c_char;\n']
START, END = 'static mut availableCodecs:', '\nstatic mut availableCodecsCount:'
ARRAY = re.compile(r'\[AvailableCodec; \d+ \+ cfg!\(feature = "libgav1"\) as usize \+ cfg!\(feature = "rav1e"\) '
                   r'as usize \+ cfg!\(feature = "svt"\) as usize \+ cfg!\(feature = "avm"\) as usize\]')
SOURCE = ROOT / 'upstream/libavif/src/avif.c'


def digest(text):
    return hashlib.sha256(text.encode()).hexdigest()


def project(text, registry):
    """Return the DAV1D=OFF form of one DAV1D=ON avif.rs translation."""
    start = text.index(START)
    end = text.index(END, start)
    table = text[start:end]
    if 'AVIF_CODEC_CHOICE_DAV1D' not in table:
        raise SystemExit('The input avif.rs is not a DAV1D=ON translation')
    text = text[:start] + registry['table'] + text[end:]
    text, count = ARRAY.subn(registry['array_type'], text)
    if count != 2:
        raise SystemExit(f'Expected the codec array type twice, found {count}')
    for declaration in DECLARATIONS:
        if text.count(declaration) != 1:
            raise SystemExit(f'Expected one dav1d declaration: {declaration.strip()}')
        text = text.replace(declaration, '')
    return text


def show(revision, path):
    return subprocess.check_output(['git', 'show', f'{revision}:./{path}'], cwd=ROOT, text=True)


def verify(revision, registry):
    """Require the rule to reproduce every real DAV1D=OFF regeneration of avif.rs."""
    pairs = ['src/avif.rs'] + sorted(str(path.relative_to(ROOT)) for path in (ROOT / 'src/profiles').glob('*/*/*/avif.rs'))
    evidence = []
    for path in pairs:
        projected, regenerated = project(show(revision, path), registry), (ROOT / path).read_text()
        if projected == regenerated:
            evidence.append(dict(rust=path, reproduced='byte-identical'))
            continue
        if path.startswith('src/profiles/linux/debug/'):
            # comparable_profile normalizes the generating checkout's own path.
            current = re.sub(r'b"/home/runner/work/[^"]*?/upstream/libavif/src/avif\.c\\0"',
                             'b"' + str(SOURCE) + '\\\\0"', regenerated)
            previous = re.sub(r'b"/home/runner/work/[^"]*?/upstream/libavif/src/avif\.c\\0"',
                              'b"' + str(SOURCE) + '\\\\0"', projected)
            if comparable_profile(SOURCE, previous) == comparable_profile(SOURCE, current):
                evidence.append(dict(rust=path, reproduced='comparable (assertion file names and unused labels)'))
                continue
        raise SystemExit(f'The projection does not reproduce the regenerated DAV1D=OFF translation: {path}')
    return evidence


def projected_variant(variant, revision, registry, evidence):
    manifest = json.loads(show(revision, f'translations/{variant}.json'))
    if 'projection' in manifest:
        raise SystemExit(f'{revision} already holds a projected {variant}')
    units, outputs = [], {}
    for unit in manifest['units']:
        if unit['source'] == DAV1D_UNIT:
            continue
        if unit['rust'].startswith(f'src/variants/{variant}/') and unit['source'] == 'upstream/libavif/src/avif.c':
            text = project(show(revision, unit['rust']), registry)
            outputs[unit['rust']] = text
            unit = dict(unit, rust_sha256=digest(text), projection=dict(
                tool='tools/project_dav1d_off.py', configuration='AVIF_CODEC_DAV1D=OFF',
                raw_sha256_configuration='AVIF_CODEC_DAV1D=ON: C2Rust output of the AVM v1.0.0 build, not regenerated',
                from_revision=revision, from_rust_sha256=unit['rust_sha256']))
        units.append(unit)
    if not outputs:
        raise SystemExit(f'No variant-specific avif.rs in {variant}')
    manifest['units'] = units
    manifest['projection'] = dict(
        tool='tools/project_dav1d_off.py', configuration='AVIF_CODEC_DAV1D=OFF (port ADR 0061)', from_revision=revision,
        reason='AVM v1.0.0 variants regenerate only locally; the rule reproduces the real DAV1D=OFF regenerations',
        verified_against=evidence)
    return manifest, outputs


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument('--from', dest='revision', help='Revision whose AVM variants are DAV1D=ON translations')
    parser.add_argument('--check', action='store_true', help='Compare with the tree without writing')
    args = parser.parse_args()
    registry = json.loads((ROOT / 'translations/codec-registry.json').read_text())
    if 'AVIF_CODEC_CHOICE_DAV1D' in registry['table']:
        raise SystemExit('Commit the DAV1D=OFF codec registry first')
    if args.check:
        for variant in VARIANTS:
            manifest = json.loads((ROOT / f'translations/{variant}.json').read_text())
            revision = manifest.get('projection', {}).get('from_revision')
            if not revision or any(unit['source'] == DAV1D_UNIT for unit in manifest['units']):
                raise SystemExit(f'{variant} is not a projected DAV1D=OFF variant')
            known = subprocess.run(['git', 'cat-file', '-e', f'{revision}^{{commit}}'], cwd=ROOT,
                                   capture_output=True).returncode == 0
            for unit in manifest['units']:
                if 'projection' in unit:
                    text = (ROOT / unit['rust']).read_text()
                    table = text[text.index(START):text.index(END)]
                    if (table != registry['table'] or any(line in text for line in DECLARATIONS)
                            or digest(text) != unit['rust_sha256']):
                        raise SystemExit(f'Projected unit changed: {unit["rust"]}')
            if known:
                expected, outputs = projected_variant(variant, revision, registry, manifest['projection']['verified_against'])
                if expected != manifest or any((ROOT / path).read_text() != text for path, text in outputs.items()):
                    raise SystemExit(f'The projection of {variant} is not reproducible from {revision}')
        print(f'Verified the projected DAV1D=OFF AVM variants ({len(VARIANTS)})')
        return
    if not args.revision:
        parser.error('--from names the revision whose AVM variants are DAV1D=ON translations')
    revision = subprocess.check_output(['git', 'rev-parse', '--verify', args.revision + '^{commit}'],
                                       cwd=ROOT, text=True).strip()
    evidence = verify(revision, registry)
    for variant in VARIANTS:
        manifest, outputs = projected_variant(variant, revision, registry, evidence)
        for path, text in outputs.items():
            (ROOT / path).write_text(text)
        (ROOT / f'translations/{variant}.json').write_text(json.dumps(manifest, indent=2) + '\n')
    exact = sum(item['reproduced'] == 'byte-identical' for item in evidence)
    print(f'Projected {len(VARIANTS)} AVM variants; the rule reproduced {len(evidence)} regenerated avif.rs '
          f'({exact} byte-identical, {len(evidence) - exact} comparable)')


if __name__ == '__main__':
    main()
