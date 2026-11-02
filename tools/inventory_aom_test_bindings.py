#!/usr/bin/env python3
"""Inventory direct original C++ test bindings, including production inlines.

This is a correspondence review inventory, not an acceptance or coverage proof.
It preserves the selected preprocessor configuration and original test bodies.
"""
import argparse
import ctypes as C
import hashlib
import json
from pathlib import Path
import re
import subprocess
import sys

from check_abi import compiler_flags
from check_aom_linkage import definitions

ROOT = Path(__file__).resolve().parents[1]


class Cursor(C.Structure):
    _fields_ = [('kind', C.c_int), ('xdata', C.c_int), ('data', C.c_void_p * 3)]


class String(C.Structure):
    _fields_ = [('data', C.c_void_p), ('flags', C.c_uint)]


class Location(C.Structure):
    _fields_ = [('data', C.c_void_p * 2), ('integer', C.c_uint)]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--debug', action='store_true')
    parser.add_argument('--only', help='Inspect one original C++ test filename')
    parser.add_argument('--libclang', type=Path)
    args = parser.parse_args()
    profile = 'debug' if args.debug else 'release'
    build = ROOT / f'build/aom-native-{profile}'
    library = args.libclang or Path('/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/lib/libclang.dylib'
                                   if sys.platform == 'darwin' else '/usr/lib/llvm-16/lib/libclang.so.1')
    clang = C.CDLL(str(library))

    def api(name, result, arguments):
        fn = getattr(clang, name)
        fn.restype, fn.argtypes = result, arguments
        return fn

    create = api('clang_createIndex', C.c_void_p, [C.c_int, C.c_int])
    dispose_index = api('clang_disposeIndex', None, [C.c_void_p])
    parse = api('clang_parseTranslationUnit', C.c_void_p,
                [C.c_void_p, C.c_char_p, C.POINTER(C.c_char_p), C.c_int, C.c_void_p, C.c_uint, C.c_uint])
    dispose = api('clang_disposeTranslationUnit', None, [C.c_void_p])
    root_cursor = api('clang_getTranslationUnitCursor', Cursor, [C.c_void_p])
    callback_type = C.CFUNCTYPE(C.c_uint, Cursor, Cursor, C.c_void_p)
    visit = api('clang_visitChildren', C.c_uint, [Cursor, callback_type, C.c_void_p])
    spelling = api('clang_getCursorSpelling', String, [Cursor])
    string_bytes = api('clang_getCString', C.c_char_p, [String])
    dispose_string = api('clang_disposeString', None, [String])
    location = api('clang_getCursorLocation', Location, [Cursor])
    expansion = api('clang_getExpansionLocation', None,
                    [Location, C.POINTER(C.c_void_p), C.POINTER(C.c_uint), C.POINTER(C.c_uint), C.POINTER(C.c_uint)])
    file_name = api('clang_getFileName', String, [C.c_void_p])
    referenced = api('clang_getCursorReferenced', Cursor, [Cursor])
    inlined = api('clang_Cursor_isFunctionInlined', C.c_uint, [Cursor])
    linkage = api('clang_getCursorLinkage', C.c_int, [Cursor])
    diagnostics = api('clang_getNumDiagnostics', C.c_uint, [C.c_void_p])
    diagnostic = api('clang_getDiagnostic', C.c_void_p, [C.c_void_p, C.c_uint])
    severity = api('clang_getDiagnosticSeverity', C.c_int, [C.c_void_p])
    message = api('clang_getDiagnosticSpelling', String, [C.c_void_p])
    dispose_diagnostic = api('clang_disposeDiagnostic', None, [C.c_void_p])
    clang_version = api('clang_getClangVersion', String, [])

    def text(value):
        result = (string_bytes(value) or b'').decode()
        dispose_string(value)
        return result

    def site(cursor):
        file, line, column, offset = C.c_void_p(), C.c_uint(), C.c_uint(), C.c_uint()
        expansion(location(cursor), C.byref(file), C.byref(line), C.byref(column), C.byref(offset))
        return (text(file_name(file)) if file else '', line.value)

    globals = definitions(build / 'libaom.a')
    platform = 'macos' if sys.platform == 'darwin' else 'linux'
    manifest = json.loads((ROOT / f'crates/aom/TRANSLATION-{platform}-{profile}.json').read_text())
    rust_names = set()
    for unit in manifest['units']:
        rust_names.update(re.findall(r'\bfn (\w+)\(', (ROOT / unit['rust']).read_text()))
    source_root = ROOT / 'build/aom-native-source'
    source_hashes = json.loads((ROOT / 'provenance/aom-source.json').read_text())['files']
    verified_paths = set()

    def verify_original(path):
        path = Path(path)
        if path.is_relative_to(source_root) and path not in verified_paths:
            name = str(path.relative_to(source_root))
            if hashlib.sha256(path.read_bytes()).hexdigest() != source_hashes[name]:
                raise ValueError(f'Modified original correspondence input: {name}')
            verified_paths.add(path)
    entries = [entry for entry in json.loads((build / 'compile_commands.json').read_text())
               if Path(entry['file']).is_relative_to(source_root / 'test') and entry['file'].endswith('.cc')
               and ('/test_aom_' in entry['output'] or '/test_libaom.' in entry['output'])]
    if args.only:
        entries = [entry for entry in entries if Path(entry['file']).name == args.only]
    if not entries:
        raise SystemExit('No selected original test compilation inputs')
    index, reports = create(0, 0), []
    for entry in entries:
        source = Path(entry['file'])
        verify_original(source)
        flags = compiler_flags(entry)
        resource = subprocess.check_output([flags[0], '-print-resource-dir'], text=True).strip()
        arguments = [*flags[1:], '-resource-dir=' + resource, '-working-directory=' + entry['directory']]
        encoded = (C.c_char_p * len(arguments))(*(value.encode() for value in arguments))
        tu = parse(index, str(source).encode(), encoded, len(arguments), None, 0, 0)
        if not tu:
            raise SystemExit(f'libclang could not parse {source}')
        errors = []
        for i in range(diagnostics(tu)):
            value = diagnostic(tu, i)
            if severity(value) >= 3:
                errors.append(text(message(value)))
            dispose_diagnostic(value)
        if errors:
            raise SystemExit(f'Original test parse errors: {source}: {errors}')
        bindings, exceptions = {}, []

        @callback_type
        def inspect(cursor, parent, opaque):
            try:
                file, line = site(cursor)
                if file != str(source):
                    return 1  # Skip included declarations and bodies.
                if cursor.kind not in [101, 103]:  # DeclRefExpr or CallExpr.
                    return 2
                target = referenced(cursor)
                if target.kind == 8:  # CXCursor_FunctionDecl; actual preprocessed callee/reference.
                    name = text(spelling(target))
                    definition_file, definition_line = site(target)
                    if name in globals and linkage(target) == 4:
                        kind = 'external original codec global'
                    elif inlined(target) and Path(definition_file).is_relative_to(source_root):
                        relative = Path(definition_file).relative_to(source_root)
                        if relative.parts[0] in ['test', 'third_party']:
                            return 2
                        kind = 'production header inline; role review required'
                    else:
                        return 2
                    key = (name, definition_file, kind)
                    declaration_path = Path(definition_file)
                    verify_original(declaration_path)
                    declaration = (str(declaration_path.relative_to(source_root))
                                   if declaration_path.is_relative_to(source_root)
                                   else '<generated>/' + str(declaration_path.relative_to(build)))
                    record = bindings.setdefault(key, dict(name=name, kind=kind,
                        declaration=declaration, declaration_line=definition_line,
                        direct_reference_lines=set(), translated_name_present=name in rust_names))
                    record['direct_reference_lines'].add(line)
                return 2
            except Exception as error:
                exceptions.append(str(error))
                return 0

        visit(root_cursor(tu), inspect, None)
        if exceptions:
            raise SystemExit(exceptions[0])
        for record in bindings.values():
            record['direct_reference_lines'] = sorted(record['direct_reference_lines'])
        report = dict(source=str(source.relative_to(source_root)), source_sha256=hashlib.sha256(source.read_bytes()).hexdigest(),
                      original_object=str((Path(entry['directory']) / entry['output']).relative_to(build)),
                      direct_bindings=sorted(bindings.values(), key=lambda value: (value['kind'], value['name'])))
        reports.append(report)
        dispose(tu)
        print(f'{report["source"]}: {len(bindings)} direct bindings', flush=True)
    dispose_index(index)
    output = ROOT / f'build/aom-test-bindings-{profile}'
    output.mkdir(exist_ok=True)
    report = dict(profile=profile, target=platform, libclang=str(library), clang_version=text(clang_version()),
                  config_sha256=hashlib.sha256((build / 'config/aom_config.h').read_bytes()).hexdigest(),
                  original_test_inputs=len(entries),
                  scope='Direct references expanded in selected original C++ test inputs; no coverage or role inference',
                  complete_test_correspondence_proven=False, sources=reports)
    (output / ('focused.json' if args.only else 'inventory.json')).write_text(json.dumps(report, indent=2) + '\n')


if __name__ == '__main__':
    main()
