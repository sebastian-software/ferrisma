#!/usr/bin/env python3
"""Generate isolated native root profiles from the actual original C build."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import platform
import re
import shlex
import subprocess

from import_translation import translate
from transpile_dav1d import current_variadic

ROOT = Path(__file__).resolve().parents[1]


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def corrected(source, raw, native):
    # Darwin's long-double compatibility rule must never narrow a Linux value.
    if native == 'linux' and re.search(r'\bf128\b', raw):
        raise SystemExit(f'Linux long double requires a separately reviewed mapping: {source}')
    text, changes = translate(source, raw)
    text, count = current_variadic(text)
    if count:
        changes['current-rust-variadic-list'] = count
    return text, changes


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--build', required=True, type=Path)
    parser.add_argument('--profile', required=True, choices=['debug', 'release'])
    parser.add_argument('--variant', default='default', choices=['default', 'mini', 'extended-pixi', 'mini-extended-pixi'])
    parser.add_argument('--clang-include', required=True, type=Path)
    parser.add_argument('--c2rust', default='c2rust-transpile')
    parser.add_argument('--check', action='store_true')
    args = parser.parse_args()
    system, machine = platform.system(), platform.machine()
    if system == 'Linux' and machine == 'x86_64':
        native, target = 'linux', 'x86_64-unknown-linux-gnu'
    elif system == 'Darwin' and machine == 'arm64':
        native, target = 'macos', 'aarch64-apple-darwin'
    else:
        raise SystemExit('Generate root profiles on the declared native Linux x86-64 or macOS ARM64 host')
    if not (args.clang_include / 'stdarg.h').is_file():
        parser.error('--clang-include must contain the pinned LLVM 16 standard headers')
    subprocess.run(['python3', ROOT / 'tools/verify_upstream.py'], check=True)
    build = args.build.resolve()
    cache = (build / 'CMakeCache.txt').read_text()
    if f'CMAKE_BUILD_TYPE:STRING={args.profile.title()}\n' not in cache:
        raise SystemExit('The original C preprocessing profile does not match the requested root profile')
    settings = dict(re.findall(r'^([^#\s:][^\n:]*):[^\n=]+=(.*)$', cache, re.M))
    for setting, enabled in [('AVIF_ENABLE_EXPERIMENTAL_MINI', 'mini' in args.variant),
                             ('AVIF_ENABLE_EXPERIMENTAL_EXTENDED_PIXI', 'extended-pixi' in args.variant)]:
        if settings.get(setting) != ('ON' if enabled else 'OFF'):
            raise SystemExit(f'The actual C container configuration differs: {setting}')
    if settings.get('AVIF_CODEC_AVM') != 'OFF' or settings.get('AVIF_LIBYUV') != 'OFF':
        raise SystemExit('This root-generation closure requires the selected default codecs and bundled scaler')
    entries = [entry for entry in json.loads((build / 'compile_commands.json').read_text())
               if 'avif_obj.dir' in entry['command']]
    expected = {str(ROOT / 'upstream/libavif/src' / name) for name in
                [Path(unit['source']).name for unit in json.loads((ROOT / 'TRANSLATION.json').read_text())['units']
                 if unit['source'].startswith('upstream/libavif/src/')]}
    expected |= {str(ROOT / unit['source']) for unit in json.loads((ROOT / 'TRANSLATION.json').read_text())['units']
                 if '/third_party/libyuv/' in unit['source']}
    if len(entries) != 28 or {entry['file'] for entry in entries} != expected:
        raise SystemExit('Root generation requires all 23 default library units and five bundled scaler units')
    for entry in entries:
        flags = shlex.split(entry['command'])
        if '-ffp-contract=off' not in flags or any('rust_archive_anchor' in flag for flag in flags):
            raise SystemExit('Use the complete noncontracting original C reference, never a Rust consumer build')
        if ('-DNDEBUG' in flags) != (args.profile == 'release'):
            raise SystemExit('The actual original C assertion configuration differs from the declared profile')
    work = ROOT / f'build/root-translate-{native}-{args.profile}-{args.variant}'
    work.mkdir(parents=True, exist_ok=True)
    database = work / 'compile_commands.json'
    for entry in entries:
        entry['command'] += ' -isystem ' + shlex.quote(str(args.clang_include.resolve()))
    database.write_text(json.dumps(entries, indent=2) + '\n')
    raw_root = work / 'raw'
    command = [args.c2rust, str(database), '--emit-build-files', '--fail-on-error',
               '--overwrite-existing', '--output-dir', str(raw_root)]
    environment = os.environ.copy()
    if environment.get('C2RUST_LIBRARY_PATH'):
        environment['DYLD_LIBRARY_PATH'] = environment['C2RUST_LIBRARY_PATH']
    translator_version = subprocess.check_output([args.c2rust, '--version'], env=environment, text=True).strip()
    if translator_version != 'transpile 0.21.0':
        raise SystemExit('Native root profiles require the recorded C2Rust 0.21.0')
    result = subprocess.run(command, cwd=ROOT, env=environment, text=True,
                            stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
    (work / 'transpile.log').write_text(result.stdout)
    if result.returncode or 'error:' in result.stdout or 'error generated' in result.stdout:
        raise SystemExit(f'Native root translation failed; inspect {work / "transpile.log"}')
    destination = ROOT / f'src/profiles/{native}/{args.profile}/{args.variant}'
    units, modules, bundled = [], [], []
    for entry in entries:
        source = Path(entry['file'])
        relative = source.relative_to(ROOT / 'upstream/libavif')
        raw_path = raw_root / 'src' / relative.with_suffix('.rs')
        raw = raw_path.read_text()
        text, changes = corrected(source, raw, native)
        name = Path('libyuv') / relative.name if relative.parts[0] == 'third_party' else Path(relative.name)
        output = destination / name.with_suffix('.rs')
        if args.check:
            if not output.is_file() or output.read_text() != text:
                raise SystemExit(f'Native root translation is not reproducible: {output}')
        else:
            if output.exists() and output.read_text() != text:
                raise SystemExit(f'Refusing to overwrite a different generated root profile: {output}')
            output.parent.mkdir(parents=True, exist_ok=True)
            output.write_text(text)
        units.append(dict(source=str(source.relative_to(ROOT)), source_sha256=digest(source), rust=str(output.relative_to(ROOT)),
                          raw_sha256=digest(raw_path), rust_sha256=hashlib.sha256(text.encode()).hexdigest(), corrections=changes))
        (bundled if name.parts[0] == 'libyuv' else modules).append(name.stem)
    lines = ['// Generated by tools/transpile_root.py from the complete native C reference.']
    for name in modules:
        lines += [f'pub mod {name};']
    lines += ['pub mod libyuv {'] + [f'    pub mod {name};' for name in bundled] + ['}']
    module_text = '\n'.join(lines) + '\n'
    selector = destination / 'modules.rs'
    if args.check:
        if selector.read_text() != module_text:
            raise SystemExit('The native root module selector differs')
    else:
        selector.write_text(module_text)
    record = dict(translator='c2rust 0.21.0', translator_version_output=translator_version,
                  llvm='16.0.6', clang_stdarg_sha256=digest(args.clang_include / 'stdarg.h'), generation_target=target,
                  preprocessing_profile=args.profile, variant=args.variant,
                  original_translation_units=28, units=units,
                  selector=str(selector.relative_to(ROOT)), selector_sha256=digest(selector),
                  original_compile_commands_sha256=digest(build / 'compile_commands.json'),
                  original_cmake_cache_sha256=digest(build / 'CMakeCache.txt'),
                  original_tests_executed=False, native_acceptance_claimed=False)
    manifest = ROOT / f'translations/root-{native}-{args.profile}-{args.variant}.json'
    if not args.check:
        manifest.write_text(json.dumps(record, indent=2) + '\n')
    print(f'{"Reproduced" if args.check else "Generated"} 28 native {target} {args.profile} root units; execution acceptance is separate')


if __name__ == '__main__':
    main()
