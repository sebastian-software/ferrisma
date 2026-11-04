#!/usr/bin/env python3
"""Typecheck one complete native generated root profile before integration."""
import argparse
import hashlib
import json
from pathlib import Path
import platform
import subprocess

ROOT = Path(__file__).resolve().parents[1]


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def validate_profile(path, target):
    record = json.loads(path.read_text())
    if (record.get('generation_target') != target or record.get('preprocessing_profile') not in ['debug', 'release']
            or record.get('variant') not in ['default', 'mini', 'extended-pixi', 'mini-extended-pixi']
            or record.get('original_translation_units') != 28 or len(record.get('units', [])) != 28):
        raise SystemExit('Incomplete or mismatched native root profile')
    native = 'linux' if target == 'x86_64-unknown-linux-gnu' else 'macos' if target == 'aarch64-apple-darwin' else None
    if native is None:
        raise SystemExit('No accepted root-generation target mapping')
    prefix = Path(f'src/profiles/{native}/{record["preprocessing_profile"]}/{record["variant"]}')
    expected = {unit['source'] for unit in json.loads((ROOT / 'TRANSLATION.json').read_text())['units']}
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


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('manifest', type=Path)
    args = parser.parse_args()
    target = ('x86_64-unknown-linux-gnu' if platform.system() == 'Linux' and platform.machine() == 'x86_64'
              else 'aarch64-apple-darwin' if platform.system() == 'Darwin' and platform.machine() == 'arm64'
              else None)
    record = validate_profile(args.manifest.resolve(), target)
    work = ROOT / f'build/root-typecheck-{target}-{record["preprocessing_profile"]}-{record["variant"]}'
    work.mkdir(parents=True, exist_ok=True)
    selector = ROOT / record['selector']
    (work / 'lib.rs').write_text('#![feature(extern_types, c_variadic)]\n'
        '#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]\n'
        '#![allow(dead_code, unused_assignments, unused_mut)]\n'
        'extern crate aom as _;\nextern crate dav1d as _;\nextern crate sharpyuv as _;\n'
        + f'include!({json.dumps(str(selector))});\n')
    manifest = ('[package]\nname="libavif-native-root-profile"\nversion="0.0.0"\nedition="2021"\n'
                '[workspace]\n[lib]\npath="lib.rs"\n[dependencies]\nlibc="=0.2.191"\n')
    for alias, name in [('aom', 'aom'), ('dav1d', 'dav1d'), ('sharpyuv', 'sharpyuv')]:
        manifest += f'{alias}={{package="libavif-{name}",path={json.dumps(str(ROOT / "crates" / name))}}}\n'
    manifest += ('[features]\ndefault=[]\nmini=[]\nextended-pixi=[]\nlibgav1=[]\nrav1e=[]\nsvt=[]\nlibyuv=[]\navm=[]\n'
                 '[profile.dev]\noverflow-checks=false\npanic="unwind"\n'
                 '[profile.release]\noverflow-checks=false\npanic="unwind"\n')
    (work / 'Cargo.toml').write_text(manifest)
    command = ['cargo', 'check', '--offline', '--manifest-path', str(work / 'Cargo.toml'),
               '--target-dir', str(ROOT / 'build/root-typecheck-target')]
    if record['preprocessing_profile'] == 'release':
        command.append('--release')
    if record['variant'] != 'default':
        command += ['--features', record['variant'].replace('mini-extended-pixi', 'mini,extended-pixi')]
    subprocess.run(command, cwd=ROOT, check=True)
    proof = dict(target=target, profile=record['preprocessing_profile'], variant=record['variant'],
                 complete_source_profile_typechecked=True, native_execution_acceptance_claimed=False,
                 translation_manifest_sha256=digest(args.manifest), units=28)
    (work / 'summary.json').write_text(json.dumps(proof, indent=2) + '\n')
    print('Complete native root profile typechecks; original execution/ABI/linkage acceptance remains separate')


if __name__ == '__main__':
    main()
