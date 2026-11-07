"""Isolate every original codec global for independent C test oracles."""
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys

from build_aom import LIBRARY_UNITS, library_entries
from check_aom_linkage import definitions

ROOT = Path(__file__).resolve().parents[1]
PREFIX = 'port_aom_reference_'


def reference_archive(profile, build=None):
    build = (build or ROOT / f'build/aom-native-{profile}').resolve()
    source = build / 'libaom.a'
    work = ROOT / f'build/aom-reference-{profile}'
    work.mkdir(parents=True, exist_ok=True)
    if len(library_entries(build)) != LIBRARY_UNITS:
        raise SystemExit('Independent C reference requires the complete original archive')
    symbols = definitions(source)
    decoration = '_' if sys.platform == 'darwin' else ''
    mapping = work / 'symbols.txt'
    mapping.write_text(''.join(f'{decoration}{name} {decoration}{PREFIX}{name}\n'
                               for name in sorted(symbols)))
    prefix = Path(os.environ.get('LIBAVIF_RS_LLVM_PREFIX', ROOT / 'build/toolchain/llvm@16/16.0.6_1'))
    objcopy = str(prefix / 'bin/llvm-objcopy') if sys.platform == 'darwin' else shutil.which('llvm-objcopy-16')
    if not objcopy or not Path(objcopy).exists():
        raise SystemExit('Independent C reference requires the pinned LLVM 16 object tool')
    tool_version = subprocess.check_output([objcopy, '--version'], text=True).strip()
    if 'LLVM version 16.0.6\n' not in tool_version + '\n':
        raise SystemExit('Independent C reference requires LLVM 16.0.6')
    key = hashlib.sha256(source.read_bytes() + mapping.read_bytes() + tool_version.encode()
                         + Path(__file__).read_bytes()).hexdigest()
    target, report = work / 'libaom-reference.a', work / 'summary.json'
    cached = json.loads(report.read_text()) if report.exists() else {}
    if (cached.get('key') != key or not target.exists()
            or hashlib.sha256(target.read_bytes()).hexdigest() != cached.get('archive_sha256')):
        subprocess.run([objcopy, '--redefine-syms=' + str(mapping), source, target], check=True)
    # Check definitions and relocations, including calls/data inside the oracle.
    renamed = definitions(target)
    if set(renamed) != {PREFIX + name for name in symbols}:
        raise SystemExit('Independent C reference changed or leaked original global definitions')
    undefined = subprocess.check_output(['nm', '-u', target], text=True)
    unrenamed = {line.split()[-1].removeprefix(decoration) for line in undefined.splitlines()
                 if line.split() and not line.endswith(':')} & symbols.keys()
    if unrenamed:
        raise SystemExit('Independent C reference still calls original codec globals: ' + ', '.join(sorted(unrenamed)))
    # llvm-objcopy rebuilds the archive index. Darwin ar lists that metadata as
    # __.SYMDEF[ SORTED]; it is not one of the original codec object members.
    def object_members(path):
        return [name for name in subprocess.check_output(['ar', '-t', path], text=True).splitlines()
                if name not in ['__.SYMDEF', '__.SYMDEF SORTED']]

    members = object_members(source)
    if len(members) != LIBRARY_UNITS or object_members(target) != members:
        raise SystemExit('Independent C reference changed original archive members')
    record = dict(key=key, profile=profile, original_translation_units=LIBRARY_UNITS, archive_members=len(members),
                  renamed_original_globals=len(symbols), unrenamed_original_codec_references=[],
                  original_archive_sha256=hashlib.sha256(source.read_bytes()).hexdigest(),
                  archive_sha256=hashlib.sha256(target.read_bytes()).hexdigest(),
                  rename_map_sha256=hashlib.sha256(mapping.read_bytes()).hexdigest(),
                  object_tool=tool_version, archive_symbol_index_rebuilt=True,
                  original_c_preprocessing_and_instructions_preserved=True)
    report.write_text(json.dumps(record, indent=2) + '\n')
    return target, symbols, record
