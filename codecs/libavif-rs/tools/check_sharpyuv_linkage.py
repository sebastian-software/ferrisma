#!/usr/bin/env python3
"""Audit Rust SharpYUV exports, archive members, and original consumer linkage."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]
PUBLIC = ['SharpYuvGetVersion', 'SharpYuvConvert', 'SharpYuvConvertWithOptions',
          'SharpYuvOptionsInitInternal', 'SharpYuvComputeConversionMatrix',
          'SharpYuvGetConversionMatrix', 'SharpYuvInit']


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--consumer', type=Path, help='Original consumer CMake build directory')
    parser.add_argument('--executable', type=Path, help='Linked original test consumer or application')
    args = parser.parse_args()
    target = Path(os.environ.get('CARGO_TARGET_DIR', ROOT.parents[1] / 'target')) / 'release'
    archive = target / 'libsharpyuv.a'
    library = target / ('libsharpyuv.dylib' if sys.platform == 'darwin' else 'libsharpyuv.so')
    members = subprocess.check_output(['ar', '-t', str(archive)], text=True).splitlines()
    if any(re.search(r'sharpyuv[^/]*\.c\.o$', member) for member in members):
        raise SystemExit('Original C SharpYUV object found in the Rust archive')
    if not any(member.startswith('sharpyuv.') and member.endswith('.rcgu.o') for member in members):
        raise SystemExit('Missing Rust SharpYUV code generation unit')
    symbols = subprocess.check_output(['nm', '-gU' if sys.platform == 'darwin' else '-D', str(library)], text=True)
    for name in PUBLIC:
        if not re.search(r'\bT\s+_?' + name + r'$', symbols, re.M):
            raise SystemExit('Missing defined export: ' + name)
    report = {'exports': PUBLIC, 'archive_members': members,
              'archive_sha256': hashlib.sha256(archive.read_bytes()).hexdigest(),
              'library_sha256': hashlib.sha256(library.read_bytes()).hexdigest()}
    if args.consumer:
        entries = json.loads((args.consumer / 'compile_commands.json').read_text())
        sharp = [e['file'] for e in entries if '/sharpyuv/' in e['file'] and e['file'].endswith('.c')]
        if sharp:
            raise SystemExit(f'Original SharpYUV implementations compiled into Rust consumer: {sharp}')
        commands = subprocess.check_output(['ninja', '-C', str(args.consumer), '-t', 'commands'], text=True)
        if re.search(r'(?<![\w/])(?:-lsharpyuv|/[^\s]+/libsharpyuv\.(?:so|dylib))', commands):
            raise SystemExit('Native shared SharpYUV remains in the consumer link commands')
        if 'libavif.a' not in commands and str(archive) not in commands:
            raise SystemExit('Rust archive is absent from consumer linkage')
        report['consumer_compile_commands_sha256'] = hashlib.sha256((args.consumer / 'compile_commands.json').read_bytes()).hexdigest()
    if args.executable:
        symbols = subprocess.check_output(['nm', '-gU' if sys.platform == 'darwin' else '--defined-only', str(args.executable)], text=True)
        for name in ['SharpYuvConvert', 'SharpYuvInit']:
            if not re.search(r'\bT\s+_?' + name + r'$', symbols, re.M):
                raise SystemExit('Consumer does not define Rust symbol ' + name)
        dependencies = subprocess.check_output(['otool', '-L', str(args.executable)] if sys.platform == 'darwin'
                                              else ['ldd', str(args.executable)], text=True)
        if re.search(r'libsharpyuv\.(?:dylib|so)', dependencies):
            raise SystemExit('Consumer dynamically loads native SharpYUV')
        report['consumer_dependencies'] = dependencies
    base = Path(os.environ.get('SHARPYUV_BUILD_ROOT', ROOT / 'build'))
    path = (args.consumer / 'sharpyuv-linkage.json') if args.consumer else (base / 'sharpyuv-linkage.json')
    path.write_text(json.dumps(report, indent=2) + '\n')
    print('Verified SharpYUV Rust archive, exports, and requested consumer linkage')


if __name__ == '__main__':
    main()
