#!/usr/bin/env python3
"""Audit every original AOM global and the root consumers' runtime closure."""
import argparse
from functools import lru_cache
import hashlib
import json
from pathlib import Path
import re
import subprocess
import sys
import platform

from check_aom_linkage import consumer_owners, definitions

ROOT = Path(__file__).resolve().parents[1]
FORBIDDEN = re.compile(r'lib(?:aom|dav1d|sharpyuv)(?:[.-]|$)')


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


@lru_cache(maxsize=None)
def dynamic_info(path):
    load = subprocess.check_output(['otool', '-l', path], text=True)
    search = re.findall(r'cmd LC_RPATH\n\s+cmdsize \d+\n\s+path (.+?) \(offset \d+\)', load)
    imports = [line.strip().split(' (')[0] for line in
               subprocess.check_output(['otool', '-L', path], text=True).splitlines()[1:]]
    if path.suffix == '.dylib':
        imports = imports[1:]  # LC_ID_DYLIB is not a dependency.
    return tuple(search), tuple(imports)


def dynamic_graph(start):
    """Resolve non-system Mach-O dependencies, including inherited rpaths."""
    if sys.platform == 'linux':
        return elf_dynamic_graph(start)
    graph, pending = {}, [(start.resolve(), [])]
    executable_directory = start.resolve().parent
    while pending:
        path, inherited = pending.pop()
        key = str(path)
        if key in graph:
            continue
        own_paths, dependencies = dynamic_info(path)

        def expand(value):
            return value.replace('@loader_path', str(path.parent)).replace('@executable_path', str(executable_directory))

        search = [expand(value) for value in own_paths] + inherited
        graph[key] = list(dependencies)
        for name in dependencies:
            if FORBIDDEN.search(Path(name).name):
                raise SystemExit(f'Native default implementation remains in the runtime closure: {path}: {name}')
            if name.startswith(('/usr/lib/', '/System/Library/')):
                continue  # OS libraries may reside only in the dyld shared cache.
            if name.startswith('@rpath/'):
                candidates = [Path(prefix) / name.removeprefix('@rpath/') for prefix in search]
            else:
                candidates = [Path(expand(name))]
            resolved = next((candidate.resolve() for candidate in candidates if candidate.is_file()), None)
            if resolved is None:
                raise SystemExit(f'Unresolved non-system runtime dependency: {path}: {name}')
            pending.append((resolved, search))
    return graph


def elf_dynamic_graph(start):
    """Trace the owned binary with its native loader, then inspect every ELF edge."""
    trace = subprocess.check_output(['ldd', str(start)], text=True, stderr=subprocess.STDOUT)
    if 'not found' in trace:
        raise SystemExit(f'Unresolved native ELF runtime dependency: {start}: {trace}')
    resolved = dict(re.findall(r'^\s*(\S+) => (/\S+) \(', trace, re.M))
    # The ELF interpreter is printed as an absolute path without an => alias.
    # It can also be a DT_NEEDED entry and must be inspected, not ignored.
    for value in re.findall(r'^\s*(/\S+) \(', trace, re.M):
        resolved[Path(value).name] = value
    graph, pending = {}, [start.resolve()]
    while pending:
        path = pending.pop()
        if str(path) in graph:
            continue
        dynamic = subprocess.check_output(['readelf', '-d', str(path)], text=True)
        imports = re.findall(r'\(NEEDED\).*?\[(.+?)\]', dynamic)
        graph[str(path)] = imports
        for name in imports:
            if FORBIDDEN.search(Path(name).name):
                raise SystemExit(f'Native default implementation remains in the runtime closure: {path}: {name}')
            candidate = Path(name) if Path(name).is_absolute() else Path(resolved.get(name, '/nonexistent'))
            if not candidate.is_file():
                raise SystemExit(f'Unresolved ELF DT_NEEDED entry: {path}: {name}')
            pending.append(candidate.resolve())
    return graph


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--debug', action='store_true')
    parser.add_argument('--archive', required=True, type=Path)
    parser.add_argument('--consumer-build', required=True, type=Path)
    parser.add_argument('--reference-build', required=True, type=Path)
    args = parser.parse_args()
    if not ((sys.platform == 'darwin' and platform.machine() == 'arm64')
            or (sys.platform == 'linux' and platform.machine() == 'x86_64')):
        parser.error('Root ownership audits require native macOS ARM64 or GNU Linux x86-64')
    profile = 'debug' if args.debug else 'release'
    archive, build, native = args.archive.resolve(), args.consumer_build.resolve(), args.reference_build.resolve()
    native_platform = 'macos' if sys.platform == 'darwin' else 'linux'
    manifest_path = ROOT / f'crates/aom/TRANSLATION-{native_platform}-{profile}.json'
    translation = json.loads(manifest_path.read_text())
    if json.loads((native / 'reference-profile.json').read_text())['profile'] != profile:
        raise SystemExit('The original AOM and root Rust preprocessing profiles differ')
    for name, expected in translation['generated_reference_headers'].items():
        if digest(native / name) != expected:
            raise SystemExit(f'The original AOM preprocessing differs: {name}')
    reference = definitions(native / 'libaom.a')
    candidate = definitions(archive, codec_members_only=True, only_symbols=set(reference))
    if reference.keys() - candidate.keys():
        raise SystemExit('The root archive is missing original AOM implementations')
    for name, original in reference.items():
        if (original['kind'].upper() == 'T') != (candidate[name]['kind'].upper() == 'T'):
            raise SystemExit(f'Original function/data kind changed: {name}')
    entries = json.loads((build / 'compile_commands.json').read_text())
    core = [entry['file'] for entry in entries if 'avif_obj.dir' in entry['command']]
    if len(core) != 1 or not core[0].endswith('/rust_archive_anchor.c'):
        raise SystemExit('Original libavif implementation objects remain in the Rust consumers')
    executables = re.findall(r'^build ([^ :]+): C(?:XX)?_EXECUTABLE_LINKER', (build / 'build.ninja').read_text(), re.M)
    if not executables:
        raise SystemExit('No original root consumer targets found')
    consumers = []
    for name in executables:
        binary = build / name
        link_map = Path(str(binary) + '.map')
        owners, _ = consumer_owners(link_map)
        loaded = {symbol: member for symbol, member in owners.items() if symbol in reference}
        for symbol, member in loaded.items():
            if not member.startswith(str(archive) + '(aom.') or not member.endswith('.rcgu.o)'):
                raise SystemExit(f'Original AOM implementation loaded outside Rust: {name}: {symbol}: {member}')
        for prefix, symbols in [('dav1d', ['dav1d_open', 'dav1d_get_picture']),
                                ('sharpyuv', ['SharpYuvConvert', 'SharpYuvInit'])]:
            for symbol in symbols:
                if symbol in owners and not (owners[symbol].startswith(str(archive) + '(' + prefix + '.')
                                               and owners[symbol].endswith('.rcgu.o)')):
                    raise SystemExit(f'Previously replaced implementation loaded outside Rust: {name}: {symbol}')
        if name in ['avifenc', 'avifdec'] and not loaded:
            raise SystemExit(f'Root CLI loaded no original AOM definitions: {name}')
        consumers.append(dict(executable=name, original_aom_globals_loaded=len(loaded),
                              executable_sha256=digest(binary), map_sha256=digest(link_map),
                              dynamic_dependencies=dynamic_graph(binary)))
    shared = archive.with_suffix('.dylib' if sys.platform == 'darwin' else '.so')
    shared_graph = dynamic_graph(shared)
    record = dict(platform=native_platform, profile=profile, original_aom_globals=len(reference),
        original_aom_globals_in_rust=len(candidate), missing_original_globals=[],
        original_aom_globals_outside_rust=[], original_libavif_implementation_objects=[],
        native_default_engine_dependencies=[],
        original_reference_archive_sha256=digest(native / 'libaom.a'),
        translation_manifest_sha256=digest(manifest_path), rust_root_archive_sha256=digest(archive),
        rust_shared_library_sha256=digest(shared), shared_library_dependencies=shared_graph, consumers=consumers)
    (build / 'aom-root-linkage.json').write_text(json.dumps(record, indent=2) + '\n')
    print(f'{len(reference)} original AOM globals supplied by Rust; {len(consumers)} original consumers have no native default engines')


if __name__ == '__main__':
    main()
