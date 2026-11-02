#!/usr/bin/env python3
"""Import checked native Linux CI profiles without changing production bodies."""
import argparse
import hashlib
import json
from pathlib import Path

from aom_inline_bridges import SUBJECTS, append_bridges

ROOT = Path(__file__).resolve().parents[1]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--artifact', required=True, type=Path)
    parser.add_argument('--run-id', required=True, type=int)
    args = parser.parse_args()
    artifact = args.artifact.resolve()
    original = json.loads((ROOT / 'docs/aom-reference-macos-debug.json').read_text())
    expected = {name.replace('<generated>/', 'generated/') for name in original['library_sources']}
    records = []
    for profile in ['debug', 'release']:
        relative_manifest = Path(f'crates/aom/TRANSLATION-linux-{profile}.json')
        manifest_path = artifact / relative_manifest
        manifest = json.loads(manifest_path.read_text())
        if (manifest['generation_target'] != 'x86_64-unknown-linux-gnu'
                or manifest['preprocessing_profile'] != profile or len(manifest['units']) != 168):
            raise SystemExit('Unexpected native Linux translation profile')
        if manifest.get('optional_inline_test_bridges'):
            raise SystemExit('This importer expects profiles generated before optional inline bridges')
        closure, prepared, units, emitted, bridges = [], {}, [], set(), []
        for unit in manifest['units']:
            source, target = Path(unit['source']), Path(unit['rust'])
            prefix = Path('upstream/aom') if source.is_relative_to('upstream/aom') else Path(f'crates/aom/reference/linux/{profile}')
            closure.append(str(source.relative_to(prefix)))
            if not target.is_relative_to(f'crates/aom/src/linux/{profile}'):
                raise SystemExit('Translation output escapes the selected Linux profile')
            content = (artifact / target).read_bytes()
            if hashlib.sha256(content).hexdigest() != unit['rust_sha256']:
                raise SystemExit(f'Artifact translation hash differs: {target}')
            text, names = append_bridges(content.decode(), emitted)
            changes = dict(unit['corrections'])
            if names:
                changes['optional-inline-test-bridge'] = len(names)
            bridges += [dict(function=name, source=str(source), export='port_aom_inline_' + name) for name in names]
            units.append(dict(source=str(source), rust=str(target), raw_sha256=unit['raw_sha256'],
                              rust_sha256=hashlib.sha256(text.encode()).hexdigest(),
                              production_rust_sha256=unit['rust_sha256'], corrections=changes))
            prepared[target] = text.encode()
        if set(closure) != expected or len(closure) != len(expected):
            raise SystemExit('Linux profile changed the complete original archive closure')
        if emitted != {name for _, names in SUBJECTS.values() for name in names}:
            raise SystemExit('Missing original inline subjects in the Linux artifact')
        reference = Path(f'crates/aom/reference/linux/{profile}')
        for name, digest in manifest['generated_reference_headers'].items():
            if hashlib.sha256((artifact / reference / name).read_bytes()).hexdigest() != digest:
                raise SystemExit(f'Artifact configuration hash differs: {name}')
        config = (artifact / reference / 'config/aom_config.h').read_text()
        for setting in ['CONFIG_AV1_ENCODER 1', 'CONFIG_AV1_DECODER 1', 'CONFIG_AV1_HIGHBITDEPTH 1',
                        'CONFIG_MULTITHREAD 1', 'CONFIG_REALTIME_ONLY 0', 'CONFIG_TUNE_VMAF 1']:
            if f'#define {setting}\n' not in config:
                raise SystemExit(f'Incomplete native Linux configuration: {setting}')
        for path in (artifact / reference).rglob('*'):
            if path.is_file():
                prepared[path.relative_to(artifact)] = path.read_bytes()
        for filename in [f'modules-linux-{profile}.rs', f'api-linux-{profile}.rs']:
            relative = Path('crates/aom/src') / filename
            prepared[relative] = (artifact / relative).read_bytes()
        result = dict(translator=manifest['translator'], llvm=manifest['llvm'], preprocessing_profile=profile,
                      generation_target=manifest['generation_target'], generated_reference_headers=manifest['generated_reference_headers'],
                      optional_inline_test_bridges=bridges, units=units)
        prepared[relative_manifest] = (json.dumps(result, indent=2) + '\n').encode()
        for relative, content in prepared.items():
            target = ROOT / relative
            if target.exists() and target.read_bytes() != content:
                raise SystemExit(f'Refusing to overwrite an existing different profile: {relative}')
        for relative, content in prepared.items():
            target = ROOT / relative
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes(content)
        reference_record = json.loads((artifact / f'build/aom-native-{profile}/reference-profile.json').read_text())
        records.append(dict(profile=profile, native_reference=reference_record,
                            artifact_manifest_sha256=hashlib.sha256(manifest_path.read_bytes()).hexdigest(),
                            imported_manifest_sha256=hashlib.sha256(prepared[relative_manifest]).hexdigest(),
                            production_bodies_unchanged=True, optional_inline_wrappers=len(bridges)))
    record = dict(run_id=args.run_id, source_revision='af3dc9aadc793c1b00edc003e3fc19a44fdeb574',
                  status='native Linux source profiles generated; compilation and acceptance pending',
                  artifact_transformation='Append the same optional inline wrappers as the current translator',
                  native_reference_and_original_tests_preserved=True, profiles=records)
    (ROOT / 'docs/aom-linux-source-import.json').write_text(json.dumps(record, indent=2) + '\n')
    print('Imported both complete native Linux profiles; all 336 production bodies are unchanged')


if __name__ == '__main__':
    main()
