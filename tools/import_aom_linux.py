#!/usr/bin/env python3
"""Import checked native Linux CI profiles with inventoried compatibility rules."""
import argparse
import hashlib
import json
from pathlib import Path
import re

from aom_inline_bridges import SUBJECTS, append_bridges
from aom_corrections import control_va_list
from aom_jumps import JUMP_CORRECTIONS, map_scopes
from build_aom import LIBRARY_UNITS

ROOT = Path(__file__).resolve().parents[1]


def merge_jump_corrections(original, jump_changes, removed_native_declarations):
    changes = dict(original)
    jumps = {name: changes.pop(name) for name in JUMP_CORRECTIONS if name in changes}
    jumps.update(jump_changes)
    if removed_native_declarations:
        remaining = jumps['system-and-vmaf-c-abi-declarations'] - removed_native_declarations
        if remaining < 0:
            raise SystemExit('Removed more glibc declarations than the artifact recorded')
        if remaining:
            jumps['system-and-vmaf-c-abi-declarations'] = remaining
        else:
            jumps.pop('system-and-vmaf-c-abi-declarations')
    # Preserve the jump stage's original position before compatibility
    # corrections, as well as the order within that stage.
    ordered, inserted = {}, False
    for name in original:
        if name in JUMP_CORRECTIONS:
            if not inserted:
                ordered.update((key, jumps[key]) for key in JUMP_CORRECTIONS if key in jumps)
                inserted = True
        else:
            ordered[name] = changes[name]
    if not inserted:
        ordered.update((key, jumps[key]) for key in JUMP_CORRECTIONS if key in jumps)
    return ordered


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--artifact', required=True, type=Path)
    parser.add_argument('--run-id', required=True, type=int)
    parser.add_argument('--check', action='store_true', help='Verify committed files against the transformed artifact')
    parser.add_argument('--refresh', action='store_true', help='Update a verified existing import after compatibility-rule changes')
    args = parser.parse_args()
    if args.check and args.refresh:
        parser.error('--check and --refresh are mutually exclusive')
    artifact = args.artifact.resolve()
    original = json.loads((ROOT / 'docs/aom-reference-macos-debug.json').read_text())
    expected = {name.replace('<generated>/', 'generated/') for name in original['library_sources']}
    records, current_artifacts = [], []
    for profile in ['debug', 'release']:
        relative_manifest = Path(f'crates/aom/TRANSLATION-linux-{profile}.json')
        manifest_path = artifact / relative_manifest
        manifest = json.loads(manifest_path.read_text())
        refresh_paths = set()
        if args.refresh:
            existing = json.loads((ROOT / relative_manifest).read_text())
            if (existing['generated_reference_headers'] != manifest['generated_reference_headers']
                    or [(u['source'], u['rust'], u['raw_sha256']) for u in existing['units']]
                    != [(u['source'], u['rust'], u['raw_sha256']) for u in manifest['units']]):
                raise SystemExit('Existing import does not match this original artifact')
            for unit in existing['units']:
                target = ROOT / unit['rust']
                if hashlib.sha256(target.read_bytes()).hexdigest() != unit['rust_sha256']:
                    raise SystemExit(f'Existing imported source was modified: {target}')
                refresh_paths.add(Path(unit['rust']))
            refresh_paths.add(relative_manifest)
        if (manifest['generation_target'] != 'x86_64-unknown-linux-gnu'
                or manifest['preprocessing_profile'] != profile or len(manifest['units']) != LIBRARY_UNITS):
            raise SystemExit('Unexpected native Linux translation profile')
        # The current translator already applies the callback correction, the
        # glibc error-target mapping and the optional inline wrappers on Linux;
        # its artifacts are imported unchanged. Earlier artifacts predate those
        # stages and are transformed here.
        current = bool(manifest.get('optional_inline_test_bridges'))
        current_artifacts.append(current)
        closure, prepared, units, emitted, bridges, corrected_units = [], {}, [], set(), [], []
        for unit in ([] if current else manifest['units']):
            source, target = Path(unit['source']), Path(unit['rust'])
            prefix = Path('upstream/aom') if source.is_relative_to('upstream/aom') else Path(f'crates/aom/reference/linux/{profile}')
            closure.append(str(source.relative_to(prefix)))
            if not target.is_relative_to(f'crates/aom/src/linux/{profile}'):
                raise SystemExit('Translation output escapes the selected Linux profile')
            content = (artifact / target).read_bytes()
            if hashlib.sha256(content).hexdigest() != unit['rust_sha256']:
                raise SystemExit(f'Artifact translation hash differs: {target}')
            text, count = control_va_list(content.decode())
            removed_native_declarations = len(re.findall(r'    fn _setjmp\([^;]+;\n', text))
            text, jump_changes = map_scopes(text)
            changes = merge_jump_corrections(unit['corrections'], jump_changes, removed_native_declarations)
            if count:
                changes['sysv-array-decayed-control-va-list'] = count
            if count or jump_changes:
                corrected_units.append(str(target))
            production_sha256 = hashlib.sha256(text.encode()).hexdigest()
            text, names = append_bridges(text, emitted)
            if names:
                changes['optional-inline-test-bridge'] = len(names)
            bridges += [dict(function=name, source=str(source), export='port_aom_inline_' + name) for name in names]
            units.append(dict(source=str(source), rust=str(target), raw_sha256=unit['raw_sha256'],
                              rust_sha256=hashlib.sha256(text.encode()).hexdigest(),
                              production_rust_sha256=production_sha256, corrections=changes))
            prepared[target] = text.encode()
        if current:
            for unit in manifest['units']:
                source, target = Path(unit['source']), Path(unit['rust'])
                prefix = Path('upstream/aom') if source.is_relative_to('upstream/aom') else Path(f'crates/aom/reference/linux/{profile}')
                closure.append(str(source.relative_to(prefix)))
                if not target.is_relative_to(f'crates/aom/src/linux/{profile}'):
                    raise SystemExit('Translation output escapes the selected Linux profile')
                content = (artifact / target).read_bytes()
                if hashlib.sha256(content).hexdigest() != unit['rust_sha256']:
                    raise SystemExit(f'Artifact translation hash differs: {target}')
                prepared[target] = content
            emitted = {item['function'] for item in manifest['optional_inline_test_bridges']}
            bridges = manifest['optional_inline_test_bridges']
            for relative in [Path(f'crates/aom/src/macros-linux-{profile}.rs'),
                             Path(f'translations/aom-macros-linux-{profile}.json')]:
                if (artifact / relative).is_file():
                    prepared[relative] = (artifact / relative).read_bytes()
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
                        'CONFIG_MULTITHREAD 1', 'CONFIG_REALTIME_ONLY 0', 'CONFIG_TUNE_VMAF 0']:
            if f'#define {setting}\n' not in config:
                raise SystemExit(f'Incomplete native Linux configuration: {setting}')
        for path in (artifact / reference).rglob('*'):
            if path.is_file():
                prepared[path.relative_to(artifact)] = path.read_bytes()
        for filename in [f'modules-linux-{profile}.rs', f'api-linux-{profile}.rs']:
            relative = Path('crates/aom/src') / filename
            prepared[relative] = (artifact / relative).read_bytes()
        if current:
            prepared[relative_manifest] = manifest_path.read_bytes()
        else:
            result = dict(translator=manifest['translator'], llvm=manifest['llvm'], preprocessing_profile=profile,
                          generation_target=manifest['generation_target'], generated_reference_headers=manifest['generated_reference_headers'],
                          optional_inline_test_bridges=bridges, units=units)
            prepared[relative_manifest] = (json.dumps(result, indent=2) + '\n').encode()
        for relative, content in prepared.items():
            target = ROOT / relative
            if args.check and not target.exists():
                raise SystemExit(f'Missing imported profile: {relative}')
            if target.exists() and target.read_bytes() != content and relative not in refresh_paths:
                raise SystemExit(f'Refusing to overwrite an existing different profile: {relative}')
        if not args.check:
            for relative, content in prepared.items():
                target = ROOT / relative
                target.parent.mkdir(parents=True, exist_ok=True)
                target.write_bytes(content)
        reference_record = json.loads((artifact / f'build/aom-native-{profile}/reference-profile.json').read_text())
        records.append(dict(profile=profile, native_reference=reference_record,
                            artifact_manifest_sha256=hashlib.sha256(manifest_path.read_bytes()).hexdigest(),
                            imported_manifest_sha256=hashlib.sha256(prepared[relative_manifest]).hexdigest(),
                            **(dict(corrections_recorded_in_manifest=True) if current else
                               dict(production_bodies_unchanged=not corrected_units,
                                    corrected_production_bodies=corrected_units)),
                            optional_inline_wrappers=len(bridges)))
    if len(set(current_artifacts)) != 1:
        raise SystemExit('Both Linux profiles must come from the same translator generation')
    transformation = ('None: the current translator generated both profiles with every correction and optional inline wrapper'
                      if current_artifacts[0] else
                      'Map the array-decayed control va_list callback type and glibc _setjmp error targets, then append the same optional inline wrappers as the current translator')
    record = dict(run_id=args.run_id, source_revision='af3dc9aadc793c1b00edc003e3fc19a44fdeb574',
                  status='native Linux source profiles generated; compilation and acceptance pending',
                  artifact_transformation=transformation,
                  native_reference_and_original_tests_preserved=True, profiles=records)
    record_path = ROOT / 'docs/aom-linux-source-import.json'
    content = json.dumps(record, indent=2) + '\n'
    if args.check:
        if record_path.read_text() != content:
            raise SystemExit('Linux source import provenance differs')
    else:
        record_path.write_text(content)
    print('Verified both complete native Linux profiles, compatibility corrections, and optional inline wrappers')


if __name__ == '__main__':
    main()
