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


def definitions(archive, codec_members_only=False):
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
        if name in result:
            raise SystemExit(f'Duplicate defined global: {name}')
        result[name] = dict(kind=kind, member=member)
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--debug', action='store_true')
    parser.add_argument('--archives-only', action='store_true')
    args = parser.parse_args()
    profile = 'debug' if args.debug else 'release'
    original = ROOT / f'build/aom-native-{profile}'
    output = ROOT / f'build/aom-rust-{profile}'
    reference_archive, rust_archive = original / 'libaom.a', ROOT / f'target/{profile}/libaom.a'
    reference, candidate = definitions(reference_archive), definitions(rust_archive, codec_members_only=True)
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
        if sys.platform != 'darwin':
            raise SystemExit('Detailed consumer-map ownership currently requires Darwin; use --archives-only')
        native_objects = [str(Path(entry['output']).relative_to(original))
                          if Path(entry['output']).is_absolute() else entry['output']
                          for entry in library_entries(original)]
        for path in sorted(output.glob('*.map')):
            owners, checked = {}, []
            for line in path.read_text(errors='surrogateescape').splitlines():
                owner = re.fullmatch(r'\[\s*(\d+)\]\s+(.+)', line)
                if owner:
                    index, name = owner.groups()
                    owners[index] = name
                    if any(name == value or name.endswith('/' + value) for value in native_objects):
                        raise SystemExit(f'Original codec object loaded into candidate: {path.name}: {name}')
                    if str(reference_archive) + '(' in name:
                        raise SystemExit(f'Original archive loaded into candidate: {path.name}: {name}')
                symbol = re.fullmatch(r'0x[0-9A-Fa-f]+\s+0x[0-9A-Fa-f]+\s+\[\s*(\d+)\]\s+_(\w+)', line)
                if symbol and symbol[2] in reference:
                    name = owners[symbol[1]]
                    if not re.search(r'libaom\.a\(aom\.[^/]*\.rcgu\.o\)$', name):
                        raise SystemExit(f'Loaded original global is supplied outside Rust: {path.name}: {symbol[2]}: {name}')
                    checked.append(symbol[2])
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
                  header_inline_test_coverage_proven=False, native_vmaf_retained=True)
    (output / 'symbol-audit.json').write_text(json.dumps(report, indent=2) + '\n')
    print(f'{len(reference)} original globals resolve to Rust codec objects; {len(consumers)} consumer maps audited')


if __name__ == '__main__':
    main()
