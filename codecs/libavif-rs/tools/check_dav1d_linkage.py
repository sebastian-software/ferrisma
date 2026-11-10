#!/usr/bin/env python3
"""Reject native dav1d objects, dependencies, and unresolved decoder symbols."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--debug', action='store_true')
    parser.add_argument('--root-library', action='store_true', help='Audit dav1d embedded in the libavif Rust archive and original CLI')
    args = parser.parse_args()
    profile = 'debug' if args.debug else 'release'
    archive = ROOT.parents[1] / 'target' / profile / ('libavif.a' if args.root_library else 'libdav1d.a')
    members = subprocess.check_output(['ar', '-t', archive], text=True).splitlines()
    rust_members = [name for name in members if name.startswith('dav1d.') and name.endswith('.rcgu.o')]
    manifest = json.loads((ROOT / 'crates/dav1d/TRANSLATION.json').read_text())
    native_names = {Path(unit['source']).name + '.o' for unit in manifest['units']}
    if not rust_members or any(name in native_names for name in members):
        raise SystemExit('Missing Rust codegen objects or original decoder objects present')
    work = ROOT / 'build/dav1d-linkage' / profile
    work.mkdir(parents=True, exist_ok=True)
    symbols = ''
    # Inspect the decoder's native codegen objects directly. Apple's nm cannot
    # read the newer LLVM bitcode embedded in some prebuilt Rust runtime members.
    for member in rust_members:
        path = work / Path(member).name
        path.write_bytes(subprocess.check_output(['ar', '-p', archive, member]))
        symbols += subprocess.check_output(['nm', '-g', path], text=True)
    defined, undefined = set(), set()
    for kind, symbol in re.findall(r'\b([A-Za-z]) (_?dav1d_\w+)$', symbols, re.M):
        (undefined if kind == 'U' else defined).add(symbol.removeprefix('_'))
    if undefined - defined:
        raise SystemExit('Unresolved decoder implementation symbols: ' + ', '.join(sorted(undefined - defined)))
    required = {'dav1d_open', 'dav1d_close', 'dav1d_send_data', 'dav1d_get_picture',
                'dav1d_apply_grain', 'dav1d_parse_sequence_header', 'dav1d_data_wrap',
                'dav1d_picture_unref', 'dav1d_flush', 'dav1d_version'}
    if required - defined:
        raise SystemExit('Missing original public API implementations')
    consumer = ROOT / ('build/port-debug' if args.debug else 'build/port') if args.root_library else ROOT / f'build/dav1d-rust-{profile}'
    dependencies = {}
    darwin = sys.platform == 'darwin'
    library_name = 'libavif' if args.root_library else 'libdav1d'
    paths = [ROOT.parents[1] / 'target' / profile / (library_name + ('.dylib' if darwin else '.so')),
             consumer / ('avifdec' if args.root_library else 'tools/dav1d')]
    for path in paths:
        output = subprocess.check_output(['otool', '-L', path] if darwin else ['ldd', path], text=True)
        linked = [line.strip().split(' (')[0] for line in output.splitlines()[1:]] if darwin else output.splitlines()
        # The dylib's own install name is the first entry in its header and may
        # contain Cargo's internal build path rather than the copied file path.
        external = linked[1:] if path.suffix == '.dylib' else linked
        if any('dav1d' in name for name in external):
            raise SystemExit(f'Native decoder dependency in {path}: {external}')
        dependencies[os.path.relpath(path, ROOT)] = external
    if not darwin and not args.root_library:
        dynamic = subprocess.check_output(['readelf', '-d', ROOT.parents[1] / 'target' / profile / 'libdav1d.so'], text=True)
        if 'SYMBOLIC' not in dynamic:
            raise SystemExit('Rust internal symbols are not bound within its own shared implementation')
    report = {'profile': profile, 'rust_codegen_objects': len(rust_members),
              'defined_decoder_symbols': len(defined), 'unresolved_decoder_symbols': [],
              'native_decoder_objects': [], 'dynamic_dependencies': dependencies,
              'archive_sha256': hashlib.sha256(archive.read_bytes()).hexdigest(),
              'binary_sha256': {os.path.relpath(path, ROOT): hashlib.sha256(path.read_bytes()).hexdigest()
                                for path in paths}}
    (consumer / ('dav1d-linkage-audit.json' if args.root_library else 'linkage-audit.json')).write_text(json.dumps(report, indent=2) + '\n')
    print(f'Rust decoder supplied by {len(rust_members)} codegen objects; no native decoder fallback')


if __name__ == '__main__':
    main()
