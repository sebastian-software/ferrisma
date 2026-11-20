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
import sys
import tempfile

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


def debug_only_statements(text):
    """Make each `#[cfg(debug_assertions)] { .. }` block a statement.

    A block that ends its enclosing block would be the tail expression, where
    an attribute does not remove it; a `;` makes it a statement."""
    marker = '#[cfg(debug_assertions)]\n{'
    position = 0
    while (start := text.find(marker, position)) >= 0:
        index, depth = start + len(marker) - 1, 0
        while True:
            char = text[index]
            if char == '"' or text.startswith('b"', index):
                index = text.index('"', index) + 1
                while text[index] != '"':
                    index += 2 if text[index] == '\\' else 1
            elif char == "'" and re.match(r"'(\\.|[^\\'])'", text[index:index + 4]):
                index = re.match(r"'(\\.|[^\\'])'", text[index:index + 4]).end() + index - 1
            elif char == '{':
                depth += 1
            elif char == '}':
                depth -= 1
                if depth == 0:
                    break
            index += 1
        following = text[index + 1:].lstrip()
        if following.startswith('}'):
            text = text[:index + 1] + ';' + text[index + 1:]
        position = index + 1
    return text


def one_source_text(text, changes):
    """Map the prepared presentation's hooks to the owned Rust platform layer."""
    text = re.sub(r'\n    fn ferrisma_debug_(?:assertions|only)\(\) -> bool;', '', text)
    text, count = re.subn(r'\bif ferrisma_debug_only\(\) \{', '#[cfg(debug_assertions)]\n{', text)
    if count:
        changes['one-source-debug-only-block'] = count
        text = debug_only_statements(text)
    if 'ferrisma_debug_only' in text:
        raise SystemExit('unexpected form of a Debug-only block')
    for item in DEBUG_BLOCKS['debug_only_fields']:
        text, count = re.subn(r'^( *)(pub ' + item['field'] + r': )', r'\1#[cfg(debug_assertions)]\n\1\2', text, flags=re.M)
        if count:
            changes['one-source-debug-only-field'] = count
    for item in DEBUG_BLOCKS['debug_only_exports']:
        text, count = re.subn(r'#\[no_mangle\]\n(pub unsafe extern "C-unwind" fn ' + item['function'] + r'\()',
                              r'#[cfg_attr(debug_assertions, no_mangle)]\n\1', text)
        if count:
            changes['one-source-debug-only-export'] = count
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


DEBUG_BLOCKS = json.loads((PREPARED / 'debug_blocks.json').read_text())
DIRECTIVE = re.compile(r'^\s*#\s*(if|ifdef|ifndef|elif|else|endif)\b')


def prepared_sources(source_root, directory):
    """Copies of the files with Debug-only blocks, presented per debug_blocks.json.

    Each listed directive becomes FERRISMA_DEBUG_ONLY_BEGIN (after `condition`,
    if the directive also tested another macro) and its #endif becomes
    FERRISMA_DEBUG_ONLY_END (prelude.h). Every other line stays as it is, so
    __LINE__ and the statements are the original's. Returns the rewritten
    paths relative to the source root."""
    by_file = {}
    for block in DEBUG_BLOCKS['blocks']:
        by_file.setdefault(block['file'], []).append(block)
    for statement in DEBUG_BLOCKS.get('statements', []):
        by_file.setdefault(statement['file'], [])
    for name, blocks in by_file.items():
        lines = (source_root / name).read_text().split('\n')
        for statement in [s for s in DEBUG_BLOCKS.get('statements', []) if s['file'] == name]:
            first, last = statement['line'] - 1, statement['last_line'] - 1
            if lines[first].strip() != statement['expect'] or not lines[last].rstrip().endswith(';'):
                raise SystemExit(f'{name}:{statement["line"]}: unexpected statement {lines[first]!r}')
            lines[first] = 'FERRISMA_DEBUG_ONLY_BEGIN ' + lines[first]
            lines[last] = lines[last] + ' FERRISMA_DEBUG_ONLY_END'
        for block in blocks:
            start = block['line'] - 1
            if lines[start].strip() != block['expect']:
                raise SystemExit(f'{name}:{block["line"]}: expected {block["expect"]!r}, found {lines[start]!r}')
            depth, end, middle = 0, None, None
            for index in range(start, len(lines)):
                match = DIRECTIVE.match(lines[index])
                if not match:
                    continue
                kind = match.group(1)
                if kind in ('if', 'ifdef', 'ifndef'):
                    depth += 1
                elif kind == 'endif':
                    depth -= 1
                    if depth == 0:
                        end = index
                        break
                elif depth == 1:
                    if kind == 'elif' or not block.get('drop_else'):
                        raise SystemExit(f'{name}:{block["line"]}: unexpected #{kind} in a Debug-only block')
                    middle = index
            if end is None:
                raise SystemExit(f'{name}:{block["line"]}: unterminated block')
            opening = 'FERRISMA_DEBUG_ONLY_BEGIN'
            if 'condition' in block:
                # Keep the line count: the marker opens the next line.
                lines[start] = block['condition']
                lines[start + 1] = opening + ' ' + lines[start + 1]
            else:
                lines[start] = opening
            if middle is not None:
                # The #else branch only casts the Debug-only variable to void.
                for index in range(middle, end):
                    lines[index] = ''
            if 'condition' in block:
                lines[end - 1] = lines[end - 1] + ' FERRISMA_DEBUG_ONLY_END'
            else:
                lines[end] = 'FERRISMA_DEBUG_ONLY_END'
        target = directory / name
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text('\n'.join(lines))
    return set(by_file)


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


def prepared_command(entry, build, platform, config_root, dispatch_root, source_root=None):
    flags = shlex.split(entry['command'])
    result = [str(LLVM / 'bin/clang'), f'--target={TARGETS[platform]}', '-nostdlibinc',
              '-isystem', str(PREPARED / 'include'), '-include', str(PREPARED / 'prelude.h')]
    if source_root is not None:
        # Prepared copies shadow the originals; __FILE__ keeps the one form.
        result += [f'-I{source_root}', f'-fmacro-prefix-map={source_root}=upstream/aom']
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


def install():
    """Merge both targets' outputs into the one owned tree and record it.

    Writes crates/aom/src/libaom (tools/merge_aom_platforms.py, with the
    va_list spelling of tools/aom_va_list.py), src/modules.rs, src/api.rs,
    crates/aom/reference/one-source and crates/aom/TRANSLATION.json."""
    import aom_manifest
    from aom_api import facade
    from merge_aom_platforms import project, ARCHES
    from stable_rust import digests
    outputs = {platform: ROOT / f'build/aom-one-source/out/{platform}/debug' for platform in TARGETS}
    records = {platform: json.loads((ROOT / f'build/aom-one-source/out/{platform}/debug.json').read_text())
               for platform in TARGETS}
    tree = ROOT / aom_manifest.TREE
    shutil.rmtree(tree, ignore_errors=True)
    report = ROOT / 'build/aom-one-source/merge-report.json'
    subprocess.run([sys.executable, '-I', str(ROOT / 'tools/merge_aom_platforms.py'),
                    '--macos', str(outputs['macos']), '--linux', str(outputs['linux']),
                    '--output', str(tree), '--report', str(report), '--verify'], check=True)
    order = [unit['source'] for unit in records['macos']['units']]
    if order != [unit['source'] for unit in records['linux']['units']]:
        raise SystemExit('The targets translated different unit sequences')
    build = ROOT / 'build/aom-native-debug'
    one_reference = ROOT / 'crates/aom/reference/one-source'
    shutil.rmtree(one_reference, ignore_errors=True)
    units, modules, bridges = [], ['// Generated by tools/aom_one_source.py --install.'], []
    by_platform = {platform: {unit['source']: unit for unit in records[platform]['units']} for platform in TARGETS}
    for relative in map(Path, order):
        rust = Path(aom_manifest.TREE) / relative.with_suffix('.rs')
        if relative.parts[0] == 'generated':
            source = one_reference / relative
            source.parent.mkdir(parents=True, exist_ok=True)
            source.write_bytes((build / relative.relative_to('generated')).read_bytes())
        else:
            source = ROOT / 'upstream/aom' / relative
        text = (ROOT / rust).read_text()
        full, production = digests(text)
        for name in re.findall(r'pub (?:unsafe )?extern "C-unwind" fn port_aom_inline_(\w+)\(', text):
            bridges.append(dict(function=name, source=str(source.relative_to(ROOT)), export='port_aom_inline_' + name))
        units.append(dict(source=str(source.relative_to(ROOT)), rust=str(rust), rust_sha256=full,
                          production_rust_sha256=production,
                          raw_sha256={p: by_platform[p][str(relative)]['raw_sha256'] for p in TARGETS},
                          corrections={p: by_platform[p][str(relative)]['corrections'] for p in TARGETS}))
        modules.append(f'#[path = "libaom/{relative.with_suffix(".rs")}"] pub mod {"__".join(relative.with_suffix("").parts)};')
    (ROOT / 'crates/aom/src/modules.rs').write_text('\n'.join(modules) + '\n')
    # One public facade: derived from each target's projection, which must agree.
    facades = set()
    with tempfile.TemporaryDirectory() as scratch:
        for platform in TARGETS:
            projected = []
            for unit in units:
                path = Path(scratch) / platform / unit['rust']
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_text(project((ROOT / unit['rust']).read_text(), ARCHES[platform]))
                projected.append(dict(unit, rust=str(path)))
            facades.add(facade(projected, build))
    if len(facades) != 1:
        raise SystemExit('The targets export different public facades')
    (ROOT / 'crates/aom/src/api.rs').write_text(facades.pop())
    targets = {}
    for platform in TARGETS:
        work = ROOT / f'build/aom-one-source/work/{platform}/debug'
        reference = aom_manifest.reference(platform)
        headers = {}
        for directory, label in [(work / 'prepared/config', 'config'), (work / 'dispatch/config', 'dispatch/config')]:
            for path in sorted(directory.glob('*.h')):
                target = reference / label / path.name
                target.parent.mkdir(parents=True, exist_ok=True)
                target.write_bytes(path.read_bytes())
                headers[f'{label}/{path.name}'] = aom_manifest.sha256(path)
        # The original C configuration of each profile (unchanged reference
        # builds, crates/aom/reference/<platform>/<profile>/config).
        builds = {profile: {f'config/{path.name}': aom_manifest.sha256(path) for path in
                            sorted((ROOT / f'crates/aom/reference/{platform}/{profile}/config').glob('*.h'))}
                  for profile in aom_manifest.PROFILES}
        targets[platform] = dict(clang_target=records[platform]['target'],
                                 rtcd_dispatch=records[platform]['rtcd_dispatch'],
                                 presentation_headers=headers, reference_builds=builds)
    merge = json.loads(report.read_text())
    aom_manifest.save(dict(
        translator='c2rust 0.21.0', llvm='16.0.6',
        presentation=dict(tool='tools/aom_one_source.py', prelude_sha256=records['macos']['prelude_sha256'],
                          neutral_headers=records['macos']['neutral_headers'],
                          debug_blocks_sha256=aom_manifest.sha256(PREPARED / 'debug_blocks.json'),
                          config_overrides=CONFIG_OVERRIDES, dropped_flags=sorted(DROPPED),
                          profile_database='build/aom-native-debug'),
        merge=dict(tool='tools/merge_aom_platforms.py', va_list=merge['va_list'], totals=merge['totals'],
                   verify=merge['verify']),
        targets=targets, optional_inline_test_bridges=bridges, units=units))
    print(f'{len(units)} units installed in {aom_manifest.TREE}')


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument('--platform', choices=sorted(TARGETS), required=True)
    parser.add_argument('--release', action='store_true',
                        help='start from the Release compilation database (the prepared input is the same)')
    parser.add_argument('--import-only', action='store_true')
    parser.add_argument('--units', nargs='*', help='only these source paths (diagnosis)')
    parser.add_argument('--install', action='store_true',
                        help='merge build/aom-one-source/out/{macos,linux}/debug into crates/aom (after both targets)')
    if '--install' in sys.argv[1:]:
        return install()
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
    native_source = ROOT / 'build/aom-native-source'
    source_root = work / 'prepared-src'
    shutil.rmtree(source_root, ignore_errors=True)
    rewritten = prepared_sources(native_source, source_root)
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
        if source.is_relative_to(native_source) and str(source.relative_to(native_source)) in rewritten:
            source = source_root / source.relative_to(native_source)
            entry['command'] = entry['command'].replace(entry['file'], str(source))
            entry['file'] = str(source)
        entry['command'] = prepared_command(entry, build, platform, config_root,
                                            dispatch_root if dispatch else None, source_root)
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
