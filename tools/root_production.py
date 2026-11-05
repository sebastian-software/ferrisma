#!/usr/bin/env python3
"""Prove which translation the production root package compiled (ADR 0054)."""
import hashlib
from pathlib import Path
import platform

ROOT = Path(__file__).resolve().parents[1]
LINUX = 'x86_64-unknown-linux-gnu'
MACOS = 'aarch64-apple-darwin'
# The crate root and the target-level selector that includes a profile's own
# module selector. They contain no translated implementation.
SELECTORS = {LINUX: 'src/linux_profiles.rs', MACOS: 'src/modules.rs'}


def digest(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def native_target():
    system, machine = platform.system(), platform.machine()
    return (LINUX if (system, machine) == ('Linux', 'x86_64')
            else MACOS if (system, machine) == ('Darwin', 'arm64') else None)


def cargo_target(features):
    """Keep each production feature configuration's archives separate."""
    variant = '-'.join(features)
    return ROOT / ('build/aom-root-cargo-target' + ('-' + variant if variant else ''))


def dependency_sources(dep_info, root=ROOT):
    """Return the crate sources below src/ named by Cargo's uplifted dep-info."""
    text = Path(dep_info).read_text().replace('\\\n', ' ')
    rule = next(line for line in text.splitlines() if ': ' in line)
    names = rule.split(': ', 1)[1].replace('\\ ', '\0').split()
    sources = root.resolve() / 'src'
    selected = {}
    for name in names:
        path = Path(name.replace('\0', ' '))
        path = path if path.is_absolute() else root / path
        if path.resolve().is_relative_to(sources):
            selected[str(path.resolve().relative_to(root.resolve()))] = digest(path)
    if not selected:
        raise SystemExit(f'Cargo dep-info names no production root sources: {dep_info}')
    return dict(sorted(selected.items()))


def match_profile(compiled, record, target):
    """Require the production sources to be exactly the generated profile's units.

    On GNU Linux the package compiles the profile files themselves. The macOS
    package retains its existing source tree, whose units must be the same
    bytes as the generated macOS profile used for the comparison.
    """
    if record.get('generation_target') != target or target not in SELECTORS:
        raise SystemExit('The production package and native profile name different targets')
    compiled = dict(compiled)
    for fixed in ['src/lib.rs', SELECTORS[target]]:
        if compiled.pop(fixed, None) is None:
            raise SystemExit(f'The production package did not compile its selector: {fixed}')
    units = record['units']
    if target == LINUX:
        if compiled.pop(record['selector'], None) != record['selector_sha256']:
            raise SystemExit('The production package did not compile the generated profile selector')
        expected = {unit['rust']: unit['rust_sha256'] for unit in units}
        if compiled != expected:
            raise SystemExit('The production package compiled a different translation than the native profile')
    elif sorted(compiled.values()) != sorted(unit['rust_sha256'] for unit in units):
        raise SystemExit('The production package units differ from the generated native profile')
    if len(compiled) != record['original_translation_units']:
        raise SystemExit('The production package lacks the complete native profile closure')
    return dict(units=len(compiled), selector=record['selector'] if target == LINUX else SELECTORS[target],
                exact_profile_paths=target == LINUX)


def record_build(target_dir, rust_profile, features, command, manifest, target):
    """Record the production package's actual artifacts and compiled source closure."""
    from check_root_profile import validate_profile
    record = validate_profile(Path(manifest).resolve(), target)
    variant = '-'.join(features) or 'default'
    if record['variant'] != variant:
        raise SystemExit('The production features differ from the generated native profile')
    if record['preprocessing_profile'] != rust_profile and not (
            target == MACOS and record['preprocessing_profile'] == 'release'):
        raise SystemExit('The production Rust profile differs from the native profile preprocessing')
    output = Path(target_dir) / rust_profile
    suffix = '.so' if target == LINUX else '.dylib'
    artifacts = {name: dict(path=str(output / name), sha256=digest(output / name))
                 for name in ['libavif.a', 'libavif.rlib', 'libavif' + suffix]}
    compiled = dependency_sources(output / 'libavif.d')
    return dict(package='libavif-rs', target=target, rust_profile=rust_profile,
                debug_assertions=rust_profile == 'debug', features=list(features), variant=variant,
                command=[str(value) for value in command], artifacts=artifacts, compiled_sources=compiled,
                root_profile=dict(manifest=str(Path(manifest).resolve().relative_to(ROOT)),
                                  manifest_sha256=digest(manifest),
                                  preprocessing_profile=record['preprocessing_profile'],
                                  **match_profile(compiled, record, target)))


def verify_build(proof, manifest, target):
    """Recheck a recorded production build against current files and the profile."""
    from check_root_profile import validate_profile
    record = validate_profile(Path(manifest).resolve(), target)
    if (proof.get('package') != 'libavif-rs' or proof.get('target') != target
            or proof['root_profile']['manifest_sha256'] != digest(manifest) or proof['variant'] != record['variant']):
        raise SystemExit('The production build proof differs from the current native profile')
    for item in proof['artifacts'].values():
        if digest(item['path']) != item['sha256']:
            raise SystemExit('The production root artifact changed after its build')
    current = {name: digest(ROOT / name) for name in proof['compiled_sources']}
    if current != proof['compiled_sources']:
        raise SystemExit('A compiled production source changed after its build')
    match_profile(current, record, target)
    return record
