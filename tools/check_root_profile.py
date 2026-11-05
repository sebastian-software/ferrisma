#!/usr/bin/env python3
"""Check or build an isolated native root profile before production admission."""
import argparse
import hashlib
import json
from pathlib import Path
import os
import platform
import shlex
import subprocess

import root_variants

ROOT = Path(__file__).resolve().parents[1]


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def validate_profile(path, target):
    record = json.loads(path.read_text())
    try:
        expected = set(root_variants.sources(record.get('variant', '')))
    except ValueError:
        raise SystemExit('Unknown native root configuration')
    if (record.get('generation_target') != target or record.get('preprocessing_profile') not in ['debug', 'release']
            or record.get('original_translation_units') != len(expected) or len(record.get('units', [])) != len(expected)):
        raise SystemExit('Incomplete or mismatched native root profile')
    native = 'linux' if target == 'x86_64-unknown-linux-gnu' else 'macos' if target == 'aarch64-apple-darwin' else None
    if native is None:
        raise SystemExit('No accepted root-generation target mapping')
    prefix = Path(f'src/profiles/{native}/{record["preprocessing_profile"]}/{record["variant"]}')
    if {unit['source'] for unit in record['units']} != expected:
        raise SystemExit('The native root profile changed the original source closure')
    outputs = set()
    for unit in record['units']:
        output = Path(unit['rust'])
        if (not output.is_relative_to(prefix) or not (ROOT / output).resolve().is_relative_to(ROOT.resolve())
                or output in outputs or digest(ROOT / output) != unit['rust_sha256']
                or digest(ROOT / unit['source']) != unit['source_sha256']):
            raise SystemExit('Changed, duplicate, or misplaced native root translation')
        outputs.add(output)
    selector = Path(record['selector'])
    if (selector != prefix / 'modules.rs' or not (ROOT / selector).resolve().is_relative_to(ROOT.resolve())
            or digest(ROOT / selector) != record['selector_sha256']):
        raise SystemExit('Changed native root module selector')
    return record


def profile_work(record, sanitizer=None):
    return ROOT / (f'build/root-typecheck-{record["generation_target"]}-'
                   f'{record["preprocessing_profile"]}-{record["variant"]}' + (f'-{sanitizer}' if sanitizer else ''))


def profile_target(record, sanitizer=None):
    # A new feature or sanitizer configuration must not replace archives or
    # shared libraries referenced by an earlier native validation proof.
    return (ROOT / 'build/root-typecheck-target' / record['generation_target']
            / (record['variant'] + (f'-{sanitizer}' if sanitizer else '')))


def sanitizer_flags(environment):
    """Instrument every Rust crate of the profile; the explicit target excludes build scripts."""
    flags = (environment['CARGO_ENCODED_RUSTFLAGS'].split('\x1f') if environment.get('CARGO_ENCODED_RUSTFLAGS')
             else shlex.split(environment.get('RUSTFLAGS', '')))
    flags += ['-Zsanitizer=address', '-Zexternal-clangrt', '-Cforce-frame-pointers=yes', '--cfg', 'libavif_rs_asan']
    # The original consumers link the selected nightly's runtime; see ADR 0009.
    # Mach-O dylibs must name the shared runtime; ELF consumers link it once.
    from aom_sanitizer import native_runtime
    triple, runtime, _ = native_runtime()
    if triple == 'aarch64-apple-darwin':
        flags += ['-Lnative=' + str(runtime.parent), '-ldylib=rustc-nightly_rt.asan',
                  '-Clink-arg=-Wl,-rpath,' + str(runtime.parent)]
    return flags


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('manifest', type=Path)
    parser.add_argument('--build', action='store_true', help='Produce native archives for the original consumer harness')
    parser.add_argument('--sanitizer', choices=['address'], help='Instrument the Rust profile for ASan parity')
    args = parser.parse_args()
    target = ('x86_64-unknown-linux-gnu' if platform.system() == 'Linux' and platform.machine() == 'x86_64'
              else 'aarch64-apple-darwin' if platform.system() == 'Darwin' and platform.machine() == 'arm64'
              else None)
    record = validate_profile(args.manifest.resolve(), target)
    if args.sanitizer and record['preprocessing_profile'] != 'release':
        parser.error('The sanitizer configuration uses original Release preprocessing')
    work = profile_work(record, args.sanitizer)
    work.mkdir(parents=True, exist_ok=True)
    selector = ROOT / record['selector']
    (work / 'lib.rs').write_text('#![feature(extern_types, c_variadic)]\n'
        + ('#![cfg_attr(libavif_rs_asan, feature(sanitize))]\n' if args.sanitizer else '')
        + '#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]\n'
        '#![allow(dead_code, unused_assignments, unused_mut)]\n'
        'extern crate aom as _;\nextern crate dav1d as _;\nextern crate sharpyuv as _;\n'
        + f'include!({json.dumps(str(selector))});\n')
    manifest = ('[package]\nname="libavif-native-root-profile"\nversion="0.0.0"\nedition="2021"\n'
                '[workspace]\n[lib]\nname="avif"\npath="lib.rs"\n'
                'crate-type=["staticlib","rlib","cdylib"]\n[dependencies]\nlibc="=0.2.191"\n')
    for alias, name in [('aom', 'aom'), ('dav1d', 'dav1d'), ('sharpyuv', 'sharpyuv')]:
        manifest += f'{alias}={{package="libavif-{name}",path={json.dumps(str(ROOT / "crates" / name))}}}\n'
    manifest += ('[features]\ndefault=[]\nmini=[]\nextended-pixi=[]\nlibgav1=[]\nrav1e=[]\nsvt=[]\nlibyuv=[]\navm=[]\n'
                 '[profile.dev]\noverflow-checks=false\npanic="unwind"\n'
                 '[profile.release]\noverflow-checks=false\npanic="unwind"\n')
    (work / 'Cargo.toml').write_text(manifest)
    # Bind translated ELF globals locally when an independent C oracle is also
    # loaded by a differential consumer. This supplies no codec implementation.
    # Selected optional engines stay the original external libraries; their
    # static archives are not bundled into the Rust staticlib (ADR 0007).
    links, _ = root_variants.native_links(record['variant'])
    (work / 'build.rs').write_text('fn main() {\n'
        '    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("linux") {\n'
        '        println!("cargo:rustc-link-arg-cdylib=-Wl,-Bsymbolic");\n    }\n'
        + ''.join(f'    println!({json.dumps(link)});\n' for link in links) + '}\n')
    destination = profile_target(record, args.sanitizer)
    command = ['cargo', 'build' if args.build else 'check', '--offline', '--manifest-path', str(work / 'Cargo.toml'),
               '--target-dir', str(destination)]
    environment = os.environ.copy()
    output = destination
    if args.sanitizer:
        environment['CARGO_ENCODED_RUSTFLAGS'] = '\x1f'.join(sanitizer_flags(environment))
        environment.pop('RUSTFLAGS', None)
        # An explicit target keeps build scripts uninstrumented.
        command += ['--target', target]
        output = destination / target
    if record['preprocessing_profile'] == 'release':
        command.append('--release')
    if record['variant'] != 'default':
        command += ['--features', root_variants.cargo_features(record['variant'])]
    subprocess.run(command, cwd=ROOT, check=True, env=environment)
    proof = dict(target=target, profile=record['preprocessing_profile'], variant=record['variant'],
                 complete_source_profile_typechecked=True, native_execution_acceptance_claimed=False,
                 translation_manifest_sha256=digest(args.manifest), units=len(record['units']),
                 **({'native_engine_links': links} if links else {}))
    if args.build:
        suffix = '.so' if target == 'x86_64-unknown-linux-gnu' else '.dylib'
        names = ['libavif.a', 'libavif.rlib', 'libavif' + suffix]
        proof['artifacts'] = {name: dict(path=str(path), sha256=digest(path)) for name, path in
            [(name, output / record['preprocessing_profile'] / name) for name in names]}
    proof['command'] = command
    if args.sanitizer:
        proof['sanitizer'] = args.sanitizer
        proof['rustflags'] = environment['CARGO_ENCODED_RUSTFLAGS'].split('\x1f')
    (work / 'summary.json').write_text(json.dumps(proof, indent=2) + '\n')
    print('Complete native root profile ' + ('built' if args.build else 'typechecked')
          + '; original execution/ABI/linkage acceptance remains separate')


if __name__ == '__main__':
    main()
