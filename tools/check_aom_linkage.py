#!/usr/bin/env python3
"""Audit original libaom global symbols and loaded consumer implementations."""
import argparse
import hashlib
import json
from pathlib import Path
import re
import subprocess
import sys

from build_aom import library_entries

ROOT = Path(__file__).resolve().parents[1]


def definitions(archive, codec_members_only=False, only_symbols=None):
    result, member = {}, None
    inputs = [archive]
    if codec_members_only:
        # Prebuilt std objects contain newer LLVM bitcode that the host nm
        # cannot parse. Inspect every codec codegen object directly; consumer
        # maps independently audit actual ownership after linking the runtime.
        directory = ROOT / 'build/aom-symbol-objects' / archive.parent.name
        directory.mkdir(parents=True, exist_ok=True)
        inputs = []
        for name in subprocess.check_output(['ar', '-t', archive], text=True).splitlines():
            if name.startswith('aom.') and name.endswith('.rcgu.o'):
                target = directory / name
                target.write_bytes(subprocess.check_output(['ar', '-p', archive, name]))
                inputs.append(target)
        if not inputs:
            raise SystemExit('No codec codegen objects found')
    for line in subprocess.check_output(['nm', '-g', *inputs], text=True).splitlines():
        if line.endswith(':'):
            member = Path(line[:-1]).name
            continue
        fields = line.split()
        if len(fields) != 3 or fields[1].upper() == 'U':
            continue
        _, kind, name = fields
        if sys.platform == 'darwin':
            if not name.startswith('_'):
                raise SystemExit(f'Unexpected Darwin global name: {name}')
            name = name[1:]
        if only_symbols is not None and name not in only_symbols:
            continue
        if name in result:
            raise SystemExit(f'Duplicate defined global: {name}')
        result[name] = dict(kind=kind, member=member)
    return result


def consumer_owners(path):
    """Read definition owners, preserving the linker's archive/member identity."""
    text = path.read_text(errors='surrogateescape')
    symbols, objects = {}, set()
    if sys.platform == 'darwin':
        owners = {}
        for line in text.splitlines():
            owner = re.fullmatch(r'\[\s*(\d+)\]\s+(.+)', line)
            if owner:
                owners[owner[1]] = owner[2]
                objects.add(owner[2])
            symbol = re.fullmatch(r'0x[0-9A-Fa-f]+\s+0x[0-9A-Fa-f]+\s+\[\s*(\d+)\]\s+_(\w+)', line)
            if symbol:
                symbols[symbol[2]] = owners[symbol[1]]
    else:
        if 'Cross Reference Table\n' not in text:
            raise SystemExit('GNU consumer ownership requires a link map with --cref')
        for line in text.split('Cross Reference Table\n', 1)[1].splitlines():
            fields = line.split()
            # The first row names the definition; indented rows are references.
            if len(fields) == 2 and not line[:1].isspace():
                symbols[fields[0]] = fields[1]
                objects.add(fields[1])
        objects.update(re.findall(r'^LOAD (.+)$', text, re.M))
    return symbols, objects


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--debug', action='store_true')
    parser.add_argument('--archives-only', action='store_true')
    parser.add_argument('--reference-build', type=Path, help='Read an existing native reference without modifying it')
    args = parser.parse_args()
    profile = 'debug' if args.debug else 'release'
    original = (args.reference_build or ROOT / f'build/aom-native-{profile}').resolve()
    platform = 'macos' if sys.platform == 'darwin' else 'linux'
    translation = json.loads((ROOT / f'crates/aom/TRANSLATION-{platform}-{profile}.json').read_text())
    if (json.loads((original / 'reference-profile.json').read_text())['profile'] != profile
            or hashlib.sha256((original / 'config/aom_config.h').read_bytes()).hexdigest()
            != translation['generated_reference_headers']['config/aom_config.h']):
        raise SystemExit('The native reference and Rust preprocessing differ')
    output = ROOT / f'build/aom-rust-{profile}'
    reference_archive, rust_archive = original / 'libaom.a', ROOT / f'target/{profile}/libaom.a'
    reference = definitions(reference_archive)
    # ELF codegen members repeat coalescible compiler metadata such as
    # __rustc_debug_gdb_scripts_section__. Audit every required original codec
    # name strictly, rather than treating Rust debugger helpers as codec APIs.
    candidate = definitions(rust_archive, codec_members_only=True, only_symbols=set(reference))
    missing = sorted(reference.keys() - candidate.keys())
    if missing:
        raise SystemExit('Missing original globals: ' + ', '.join(missing))
    for name, record in reference.items():
        rust = candidate[name]
        if not rust['member'].startswith('aom.') or not rust['member'].endswith('.rcgu.o'):
            raise SystemExit(f'Original global is supplied outside Rust codec objects: {name}: {rust}')
        if (record['kind'].upper() == 'T') != (rust['kind'].upper() == 'T'):
            raise SystemExit(f'Function/data symbol kind differs: {name}')
    consumers = []
    if not args.archives_only:
        native_objects = [str(Path(entry['output']).relative_to(original))
                          if Path(entry['output']).is_absolute() else entry['output']
                          for entry in library_entries(original)]
        for path in sorted(output.glob('*.map')):
            owners, objects = consumer_owners(path)
            checked = []
            for name in objects:
                if any(name == value or name.endswith('/' + value) for value in native_objects):
                    raise SystemExit(f'Original codec object loaded into candidate: {path.name}: {name}')
                if name == str(reference_archive) or str(reference_archive) + '(' in name or name == 'libaom.a':
                    raise SystemExit(f'Original archive loaded into candidate: {path.name}: {name}')
            for symbol, name in owners.items():
                if symbol in reference:
                    if not name.startswith(str(rust_archive) + '(aom.') or not name.endswith('.rcgu.o)'):
                        raise SystemExit(f'Loaded original global is supplied outside Rust: {path.name}: {symbol}: {name}')
                    checked.append(symbol)
            if not checked:
                raise SystemExit(f'No original globals found in consumer map: {path}')
            consumers.append(dict(executable=path.stem, checked_original_globals=len(set(checked)),
                                  map_sha256=hashlib.sha256(path.read_bytes()).hexdigest(),
                                  original_codec_objects_loaded=0, original_globals_outside_rust=0))
        if not consumers:
            raise SystemExit('No original consumer link maps to audit')
    report = dict(profile=profile, original_global_definitions=len(reference),
                  original_global_functions=sum(v['kind'].upper() == 'T' for v in reference.values()),
                  missing_original_globals=missing, original_globals_outside_rust=[],
                  original_archive_sha256=hashlib.sha256(reference_archive.read_bytes()).hexdigest(),
                  rust_archive_sha256=hashlib.sha256(rust_archive.read_bytes()).hexdigest(),
                  consumers=consumers, archives_only=args.archives_only,
                  rust_definition_scope='Every original codec global in all Rust codec codegen members; compiler metadata excluded',
                  header_inline_test_coverage_proven=False)
    (output / 'symbol-audit.json').write_text(json.dumps(report, indent=2) + '\n')
    print(f'{len(reference)} original globals resolve to Rust codec objects; {len(consumers)} consumer maps audited')


if __name__ == '__main__':
    main()
