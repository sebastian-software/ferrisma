#!/usr/bin/env python3
"""Generate optional Rust macro subjects from the original libaom headers."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import shlex
import subprocess
import sys

from aom_corrections import compatibility
from aom_jumps import map_jumps
from build_aom import LIBRARY_UNITS, library_entries
from check_abi import compiler_flags
from transpile_dav1d import corrected

ROOT = Path(__file__).resolve().parents[1]


def sha256(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--release', action='store_true')
    parser.add_argument('--check', action='store_true')
    parser.add_argument('--import-only', action='store_true')
    parser.add_argument('--reference-build', type=Path, help='Read an existing matching native build without modifying it')
    parser.add_argument('--clang-include', type=Path)
    args = parser.parse_args()
    subprocess.run(['python3', ROOT / 'tools/verify_upstream.py'], check=True)
    profile = 'release' if args.release else 'debug'
    platform = 'linux' if sys.platform == 'linux' else 'macos'
    build = (args.reference_build or ROOT / f'build/aom-native-{profile}').resolve()
    reference = json.loads((build / 'reference-profile.json').read_text())
    if reference['profile'] != profile:
        raise SystemExit('The native reference uses different preprocessing')
    expected_config = ROOT / f'crates/aom/reference/{platform}/{profile}/config/aom_config.h'
    if sha256(build / 'config/aom_config.h') != sha256(expected_config):
        raise SystemExit('The native reference differs from the recorded portable configuration')
    work = ROOT / f'build/aom-macro-translate/{profile}'
    work.mkdir(parents=True, exist_ok=True)
    raw_root = ROOT / f'build/aom-macro-generated/{profile}'
    raw_root.parent.mkdir(parents=True, exist_ok=True)
    source = ROOT / 'tests/ports/aom_macro_subjects.c'
    original = next(entry for entry in library_entries(build) if Path(entry['file']).name == 'odintrin.c')
    resource = args.clang_include or (Path('/usr/lib/llvm-16/lib/clang/16/include') if platform == 'linux'
                else ROOT / 'build/toolchain/llvm@16/16.0.6_1/lib/clang/16/include')
    flags = compiler_flags(original)
    ndebug = any(flag == '-DNDEBUG' or flag.startswith('-DNDEBUG=') for flag in flags)
    if '-ffp-contract=off' not in flags or ndebug != args.release:
        raise SystemExit('The native reference does not preserve the pinned numeric/preprocessing flags')
    if any(flag in ['-MD', '-MMD', '-MP', '-MG'] or flag.startswith(('-MF', '-MT', '-MQ', '-MJ'))
           for flag in flags):
        raise SystemExit('The read-only reference flags request dependency output files')
    arguments = [*flags, '-isystem', str(resource), '-c', str(source),
                 '-o', str(work / 'aom_macro_subjects.o')]
    entry = dict(directory=original['directory'], file=str(source), command=shlex.join(arguments))
    database = work / 'compile_commands.json'
    database.write_text(json.dumps([entry], indent=2) + '\n')
    environment = os.environ.copy()
    if platform == 'macos':
        environment['DYLD_LIBRARY_PATH'] = os.environ.get('C2RUST_LIBRARY_PATH',
                                                         str(resource.parents[2]) + ':/opt/homebrew/lib')
    if not args.import_only:
        with (work / 'transpile.log').open('w') as log:
            result = subprocess.run(['c2rust-transpile', database, '--emit-build-files', '--fail-on-error',
                                     '--output-dir', raw_root], env=environment,
                                    stdout=log, stderr=subprocess.STDOUT)
        if result.returncode:
            raise SystemExit(f'C2Rust macro translation failed; inspect {work / "transpile.log"}')
    outputs = list(raw_root.rglob('aom_macro_subjects.rs'))
    if len(outputs) != 1:
        raise SystemExit('Expected exactly one original macro-subject translation')
    raw = outputs[0].read_text()
    text, changes = corrected(source, raw, entry, allow_pruned_alignment=True)
    text, jump_changes = map_jumps(text, {'port_aom_macro_OD_DIVU_SMALL'})
    changes.update(jump_changes)
    text, extra = compatibility(text)
    changes.update(extra)
    text = text.replace('__dav1d_align_', '__aom_align_')
    target = ROOT / f'crates/aom/src/macros-{platform}-{profile}.rs'
    headers = ['aom_dsp/odintrin.h', 'aom_dsp/aom_dsp_common.h', 'aom_ports/bitops.h', 'aom/aom_integer.h']
    provenance = json.loads((ROOT / 'provenance/aom-source.json').read_text())['files']
    reference_source = Path(original['file']).parents[1]
    for header in headers:
        if sha256(reference_source / header) != provenance[header]:
            raise SystemExit(f'Modified original macro header: {header}')
    record = dict(kind=f'optional original macro test subjects; outside the {LIBRARY_UNITS} production translation units',
                  translator='c2rust 0.21.0', llvm='16.0.6', preprocessing_profile=profile,
                  generation_target='x86_64-unknown-linux-gnu' if platform == 'linux' else 'aarch64-apple-darwin',
                  source_wrapper_sha256=sha256(source),
                  source_manifest_sha256=sha256(ROOT / 'provenance/aom-source.json'),
                  original_headers={header: provenance[header] for header in headers},
                  config_sha256=sha256(build / 'config/aom_config.h'),
                  subjects=[dict(macro='OD_DIVU_SMALL', export='port_aom_macro_OD_DIVU_SMALL',
                                 original_test='test/divu_small_test.cc')],
                  units=[dict(source=str(source.relative_to(ROOT)), rust=str(target.relative_to(ROOT)),
                              raw_sha256=hashlib.sha256(raw.encode()).hexdigest(),
                              rust_sha256=hashlib.sha256(text.encode()).hexdigest(), corrections=changes)])
    manifest = ROOT / f'translations/aom-macros-{platform}-{profile}.json'
    for path, content in [(target, text), (manifest, json.dumps(record, indent=2) + '\n')]:
        if args.check:
            if path.read_text() != content:
                raise SystemExit(f'Original macro subject differs: {path}')
        else:
            path.write_text(content)
    print(f'Generated original OD_DIVU_SMALL Rust test subject: {platform}/{profile}')


if __name__ == '__main__':
    main()
