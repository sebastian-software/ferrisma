#!/usr/bin/env python3
"""Translate libaom once from prepared C inputs (route B prototype, phase 1).

The C sources stay unchanged; only their presentation to C2Rust changes:

* no NDEBUG and CONFIG_DEBUG 1 for every profile, with a forced include
  (`tools/aom_one_source/prelude.h`) and a neutral <assert.h> that turn
  `assert(e)` into a check gated by `ferrisma_debug_assertions()`;
* a small neutral system-header set (`tools/aom_one_source/include`, searched
  instead of the macOS SDK or glibc headers) with one spelling of every
  fixed-width type, and platform shims for the few C-library details that
  differ (standard streams, errno, floating-point exceptions);
* the target triple of the platform (`--target`), so only target facts
  (predefined macros, va_list, CPU dispatch headers) can still differ.

Each unit then goes through the same correction pipeline as
`tools/transpile_aom.py` (same functions, same order), followed by
`one_source_text()`, which maps the prelude's hooks to Rust. Output goes to
`build/aom-one-source/out/<platform>/<profile>/` with the layout of
`crates/aom/src/<platform>/<profile>/`, so the trees can be compared with the
item-level analysis (items.py). Nothing in `crates/` is written.

    python3 tools/aom_one_source.py --platform macos [--release]
    python3 tools/aom_one_source.py --platform linux [--release]
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import shlex
import shutil
import subprocess

from aom_compound_literals import name_float_tables
from aom_corrections import compatibility
from aom_inline_bridges import append_bridges
from aom_jumps import map_jumps
from build_aom import LIBRARY_UNITS, library_entries
from transpile_aom import RTCD_DISPATCH, SIMD_SUFFIX, dispatch_headers, map_cpuid
from transpile_dav1d import corrected
from stable_rust import stable_text

ROOT = Path(__file__).resolve().parents[1]
PREPARED = ROOT / 'tools/aom_one_source'
TARGETS = {'macos': 'arm64-apple-macosx11.0.0', 'linux': 'x86_64-unknown-linux-gnu'}
LLVM = Path(os.environ.get('LIBAVIF_RS_LLVM16', '/opt/homebrew/opt/llvm@16'))
C2RUST = Path(os.environ.get('C2RUST_TRANSPILE', ROOT / 'build/toolchain/c2rust/bin/c2rust-transpile'))
# Flags of the reference build that select a profile or a host; the prepared
# presentation replaces them (profile: none, host: --target).
DROPPED = {'-g', '-O0', '-O1', '-O2', '-O3', '-DNDEBUG', '-fPIC'}
CONFIG_OVERRIDES = {'CONFIG_DEBUG': '1', 'HAVE_FEXCEPT': '1'}

# Hooks of prelude.h and the neutral headers, and their Rust form.
HOOK_CALLS = {
    'ferrisma_debug_assertions()': 'cfg!(debug_assertions)',
}
SHIMS = ['ferrisma_assert_fail', 'ferrisma_stderr', 'ferrisma_stdout', 'ferrisma_errno',
         'ferrisma_feenableexcept', 'ferrisma_fedisableexcept']
PLACEHOLDER_RECORDS = re.compile(
    r'(?:#\[derive\([^)]*\)\]\n)?(?:#\[repr\(C\)\]\n)?pub struct (ferrisma_pthread\w*|ferrisma_jmp_buf_element) \{.*?\n\}\n',
    re.S)


def one_source_text(text, changes):
    """Map the prepared presentation's hooks to the owned Rust platform layer."""
    text = re.sub(r'\n    fn ferrisma_debug_assertions\(\) -> bool;', '', text)
    for call, replacement in HOOK_CALLS.items():
        count = text.count(call)
        if count:
            text = text.replace(call, replacement)
            changes['one-source-debug-assertions'] = count
    for name in SHIMS:
        text, count = re.subn(r'\n    fn ' + name + r'\([^;]*;', '', text)
        text, uses = re.subn(r'(?<![\w:])' + name + r'\(', f'crate::platform::{name}(', text)
        if uses:
            changes['one-source-platform-shim'] = changes.get('one-source-platform-shim', 0) + uses
    text, count = PLACEHOLDER_RECORDS.subn('', text)
    if count:
        changes['one-source-placeholder-record'] = count
    text, count = re.subn(r'(?:#\[derive\([^)]*\)\]\n)?(?:#\[repr\(C\)\]\n)?pub struct timeval \{.*?\n\}\n',
                          'pub type timeval = libc::timeval;\n', text, flags=re.S)
    if count:
        changes['one-source-timeval'] = count
    text, count = re.subn(r'pub type jmp_buf = [^\n]+;', 'pub type jmp_buf = crate::platform::jmp_buf;', text)
    if count:
        changes['one-source-jmp-buf'] = count
    # C's `{0}` for the jmp_buf member of aom_internal_error_info.
    text, count = re.subn(r'\[ferrisma_jmp_buf_element \{[^{}]*?\}; \d+\]', 'core::mem::zeroed()', text)
    if count:
        changes['one-source-jmp-buf-zero'] = count
    # C2Rust zero-initializes the uninitialized pthread_attr_t local of
    # aom_thread.c with the placeholder record; libc's type is zeroed the same.
    text, count = re.subn(r'ferrisma_pthread_\w+ \{[^{}]*\}', 'core::mem::zeroed()', text)
    if count:
        changes['one-source-pthread-zero'] = count
    text = re.sub(r'\n    pub type ferrisma_FILE;', '', text)
    text = re.sub(r'extern "C(?:-unwind)?" \{\n\}\n', '', text)
    return text


def prepared_config(build, directory):
    """The generic configuration with the one-source overrides."""
    target = directory / 'config'
    target.mkdir(parents=True, exist_ok=True)
    for path in (build / 'config').iterdir():
        content = path.read_text()
        if path.name == 'aom_config.h':
            for name, value in CONFIG_OVERRIDES.items():
                content, count = re.subn(rf'^#define {name} \d+$', f'#define {name} {value}', content, flags=re.M)
                if count != 1:
                    raise SystemExit(f'{name} not found once in aom_config.h')
        (target / path.name).write_text(content)
    return directory


def prepared_command(entry, build, platform, config_root, dispatch_root):
    flags = shlex.split(entry['command'])
    result = [str(LLVM / 'bin/clang'), f'--target={TARGETS[platform]}', '-nostdlibinc',
              '-isystem', str(PREPARED / 'include'), '-include', str(PREPARED / 'prelude.h')]
    if dispatch_root is not None:
        result.append(f'-I{dispatch_root}')
    index = 1
    while index < len(flags):
        flag = flags[index]
        if flag == '-arch':
            index += 2
            continue
        if flag in DROPPED:
            index += 1
            continue
        if flag == f'-I{build}':
            flag = f'-I{config_root}'
        result.append(flag)
        index += 1
    return shlex.join(result)


def write_modules(out):
    """The module selector of the prototype (lib.rs, cfg aom_one_source_prototype)."""
    lines = ['// Generated by tools/aom_one_source.py (prototype selector).']
    for path in sorted(out.rglob('*.rs')):
        relative = path.relative_to(out).with_suffix('')
        lines.append(f'#[path = "{path}"] pub mod {"__".join(relative.parts)};')
    target = out.parent / f'{out.name}.modules.rs'
    target.write_text('\n'.join(lines) + '\n')
    return target


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument('--platform', choices=sorted(TARGETS), required=True)
    parser.add_argument('--release', action='store_true',
                        help='start from the Release compilation database (the prepared input is the same)')
    parser.add_argument('--import-only', action='store_true')
    parser.add_argument('--units', nargs='*', help='only these source paths (diagnosis)')
    args = parser.parse_args()
    profile = 'release' if args.release else 'debug'
    platform = args.platform
    build = ROOT / f'build/aom-native-{profile}'
    entries = library_entries(build)
    if len(entries) != LIBRARY_UNITS:
        raise SystemExit(f'Unexpected archive closure: {len(entries)}')
    work = ROOT / f'build/aom-one-source/work/{platform}/{profile}'
    work.mkdir(parents=True, exist_ok=True)
    config_root = prepared_config(build, work / 'prepared')
    dispatch_root = work / 'dispatch'
    dispatch = dispatch_headers(platform, config_root, dispatch_root)
    inputs = []
    for original_entry in entries:
        entry = dict(original_entry)
        source = Path(entry['file'])
        original = source
        if args.units and not any(str(source).endswith(unit) for unit in args.units):
            continue
        if source.name == 'tx_search.c':
            content, _ = name_float_tables(source.read_text())
            source = work / 'tx_search.c'
            source.write_text(content)
            entry['command'] = entry['command'].replace(entry['file'], str(source))
            entry['file'] = str(source)
        entry['command'] = prepared_command(entry, build, platform, config_root,
                                            dispatch_root if dispatch else None)
        # transpile_aom.py appends the resource directory last, and the layout
        # query of dav1d_layout.py drops exactly that final pair.
        entry['command'] += f' -isystem {LLVM}/lib/clang/16/include'
        inputs.append((original, source, entry))
    database = work / 'compile_commands.json'
    database.write_text(json.dumps([e for _, _, e in inputs], indent=2) + '\n')
    raw_root = work / 'raw'
    if not args.import_only:
        shutil.rmtree(raw_root, ignore_errors=True)
        environment = os.environ.copy()
        environment['DYLD_LIBRARY_PATH'] = f'{LLVM}/lib'
        with (work / 'transpile.log').open('w') as log:
            result = subprocess.run([str(C2RUST), str(database), '--emit-build-files',
                                     '--fail-on-error', '--output-dir', str(raw_root)],
                                    env=environment, stdout=log, stderr=subprocess.STDOUT)
        if result.returncode:
            raise SystemExit(f'C2Rust failed; inspect {work / "transpile.log"}')
    ported_functions = set()
    raw_paths = {}
    for original, normalized, _ in inputs:
        relative = normalized.relative_to(ROOT / 'build').with_suffix('.rs')
        relative = Path(*(part.replace('-', '_') for part in relative.parts))
        candidates = list((raw_root / 'src').rglob(relative.name)) if not (raw_root / 'src' / relative).exists() else [raw_root / 'src' / relative]
        # C2Rust names outputs after the common root of its inputs.
        match = [p for p in candidates if str(p).endswith(str(Path(*relative.parts[1:])))]
        if len(match) != 1:
            raise SystemExit(f'No unique C2Rust output for {relative}: {candidates}')
        raw_paths[original] = match[0]
        ported_functions.update(re.findall(r'pub unsafe extern "C" fn (\w+)\(', match[0].read_text()))
    if dispatch is not None:
        for name in dispatch['headers']:
            ported_functions.update(re.findall(rf'\b(\w+_{SIMD_SUFFIX[platform]})\(',
                                               (dispatch_root / name).read_text()))
        if platform == 'macos':
            ported_functions.add('aom_arm_cpu_caps')
    out = ROOT / f'build/aom-one-source/out/{platform}/{profile}'
    shutil.rmtree(out, ignore_errors=True)
    emitted_bridges, record = set(), []
    for original, normalized, entry in inputs:
        generated = original.is_relative_to(build)
        relative = (Path('generated') / original.relative_to(build) if generated
                    else original.relative_to(ROOT / 'build/aom-native-source'))
        source = original if generated else ROOT / 'upstream/aom' / relative
        raw = raw_paths[original].read_text()
        text, changes = corrected(source, raw, entry, allow_pruned_alignment=True)
        text, jump_changes = map_jumps(text, ported_functions)
        changes.update(jump_changes)
        text, extra_changes = compatibility(text)
        changes.update(extra_changes)
        if dispatch is not None and platform == 'linux' and 'fn x86_simd_caps()' in text:
            text, count = map_cpuid(text)
            changes['cpuid-register-mapping'] = count
        text = text.replace('__dav1d_align_', '__aom_align_')
        text = one_source_text(text, changes)
        rel = f'crates/aom/src/{platform}/{profile}/{relative.with_suffix(".rs")}'
        text = stable_text(rel, text)
        text, _ = append_bridges(text, emitted_bridges)
        target = out / relative.with_suffix('.rs')
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text(text)
        record.append(dict(source=str(relative), raw_sha256=hashlib.sha256(raw.encode()).hexdigest(),
                           rust_sha256=hashlib.sha256(text.encode()).hexdigest(), corrections=changes))
    (out.parent / f'{profile}.json').write_text(json.dumps(dict(
        translator='c2rust 0.21.0', llvm='16.0.6', target=TARGETS[platform], profile_database=profile,
        prelude_sha256=hashlib.sha256((PREPARED / 'prelude.h').read_bytes()).hexdigest(),
        neutral_headers={str(p.relative_to(PREPARED)): hashlib.sha256(p.read_bytes()).hexdigest()
                         for p in sorted((PREPARED / 'include').rglob('*.h'))},
        rtcd_dispatch=dispatch, units=record), indent=2) + '\n')
    write_modules(out)
    print(f'{len(record)} units written to {out}')


if __name__ == '__main__':
    main()
