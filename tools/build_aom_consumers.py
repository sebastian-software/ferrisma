#!/usr/bin/env python3
"""Relink original libaom tests/apps/examples with the Rust codec archive.

Upstream compiles `common/args_helper.c` and `av1/arg_defs.c` into both the
library and the app object targets. The bridge removes only those
source-identical app copies so their functions and data resolve to Rust in both
profiles, rejects every other overlap with production sources, and records the
exact link command and map of each consumer.

Upstream builds `av1/ratectrl_rtc.cc` into the separate `aom_av1_rc` library,
which it does not install. That optional wrapper is outside the 166-unit
archive and is not a root libavif dependency. The original `test_aom_rc`
executable keeps it as a native consumer of the Rust codec: its passing cases
do not establish a Rust wrapper implementation. Native C++ rate-control,
libyuv, WebM, and GoogleTest code must not be counted as Rust coverage.

`--reference-build` lets a task worktree reuse the native reference sources,
objects, and executables of another checkout without rebuilding or modifying
them; the generated reference headers are checked against the translation
manifest first. Rust libraries, consumers, and reports stay in this worktree.
"""
import argparse
import hashlib
import json
from pathlib import Path
import re
import shlex
import subprocess
import sys

from build_aom import library_entries

ROOT = Path(__file__).resolve().parents[1]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--debug', action='store_true')
    parser.add_argument('--only', action='append', help='Build a named original consumer first')
    parser.add_argument('--reference-build', type=Path, help='Read an existing native reference without modifying it')
    args = parser.parse_args()
    profile = 'debug' if args.debug else 'release'
    build = (args.reference_build or ROOT / f'build/aom-native-{profile}').resolve()
    platform = 'macos' if sys.platform == 'darwin' else 'linux'
    manifest = json.loads((ROOT / f'crates/aom/TRANSLATION-{platform}-{profile}.json').read_text())
    if json.loads((build / 'reference-profile.json').read_text())['profile'] != profile:
        raise SystemExit('The native reference and Rust profile differ')
    for name, expected in manifest['generated_reference_headers'].items():
        if hashlib.sha256((build / name).read_bytes()).hexdigest() != expected:
            raise SystemExit(f'The native reference and Rust preprocessing differ: {name}')
    output = ROOT / f'build/aom-rust-{profile}'
    output.mkdir(parents=True, exist_ok=True)
    cargo = ['cargo', 'build', '-p', 'libavif-aom', '--locked']
    if not args.debug:
        cargo.append('--release')
    subprocess.run(cargo, check=True)
    archive = ROOT / f'target/{profile}/libaom.a'
    library_sources = {Path(entry['file']) for entry in library_entries(build)}
    entries = json.loads((build / 'compile_commands.json').read_text())
    compiled_sources = {str((Path(entry['directory']) / entry['output']).relative_to(build)): Path(entry['file'])
                        for entry in entries}
    source_root = next(source.parents[1] for source in compiled_sources.values()
                       if source.as_posix().endswith('/common/args_helper.c'))
    executables = re.findall(r'^build ([^ :]+): C(?:XX)?_EXECUTABLE_LINKER',
                             (build / 'build.ninja').read_text(), re.M)
    if args.only:
        if set(args.only) - set(executables):
            raise SystemExit('Unknown original consumer requested')
        executables = [name for name in executables if name in args.only]
    record_path = output / 'linkage.json'
    linkage = {item['executable']: item for item in json.loads(record_path.read_text())} if record_path.exists() else {}
    archive_sha256 = hashlib.sha256(archive.read_bytes()).hexdigest()
    for executable in executables:
        commands = subprocess.check_output(['ninja', '-C', build, '-t', 'commands', executable], text=True)
        command = shlex.split(commands.splitlines()[-1])
        if command[:2] == [':', '&&'] and command[-2:] == ['&&', ':']:
            command = command[2:-2]
        if any(token in [':', '&&', '||', ';'] for token in command):
            raise SystemExit(f'Unmapped original link command wrapper: {executable}')
        overlaps = [token for token in command if compiled_sources.get(token) in library_sources]
        # Upstream compiles args_helper.c and arg_defs.c into both codec and app
        # object targets. C archives select only one copy; Rust codegen units
        # can combine these definitions with other codec functions. Retain the
        # Rust implementations and remove only identical-source app copies.
        for token in overlaps:
            source = compiled_sources[token]
            if source.relative_to(source_root).as_posix() not in [
                    'common/args_helper.c', 'av1/arg_defs.c']:
                raise SystemExit(f'Unreviewed app/codec source overlap: {token}')
            command.remove(token)
        target = output / executable
        command[command.index('-o') + 1] = str(target)
        replacements = 0
        for i, token in enumerate(command):
            if token == 'libaom.a':
                command[i] = str(archive)
                replacements += 1
        if replacements != 1:
            raise SystemExit(f'Expected one explicit codec replacement: {executable}: {replacements}')
        if any(token == 'libaom.a' or token.startswith('-laom') for token in command):
            raise SystemExit('Original codec library remains in candidate link command')
        link_map = output / f'{executable}.map'
        command.append(f'-Wl,-map,{link_map}' if sys.platform == 'darwin' else f'-Wl,-Map,{link_map},--cref')
        subprocess.run(command, cwd=build, check=True)
        linkage[executable] = dict(executable=executable, codec_replacements=replacements,
                            rust_archive_sha256=archive_sha256,
                            executable_sha256=hashlib.sha256(target.read_bytes()).hexdigest(),
                            unchanged_original_test_objects=True,
                            identical_source_app_objects_replaced=overlaps, command=command)
        print(f'Relinked original {executable} with the Rust codec', flush=True)
    record_path.write_text(json.dumps([linkage[name] for name in sorted(linkage)], indent=2) + '\n')


if __name__ == '__main__':
    main()
