#!/usr/bin/env python3
"""Dispatch dav1d DSP families to the kernels rebuilt with intrinsics.

Usage: python3 -I tools/enable_dav1d_dispatch.py ENTRY...   enable ENTRY and apply
       python3 -I tools/enable_dav1d_dispatch.py --apply    apply the committed list
       python3 -I tools/enable_dav1d_dispatch.py --rehash   re-record the unit digests
       python3 -I tools/enable_dav1d_dispatch.py --check    compare the tree with the output

An ENTRY is `<family>_<bpc>bpc_<arch>`, for example `itx_8bpc_arm`, or
`<family>_<arch>` for a family without a bit depth (`msac_arm`): the
family's 8 bpc init is to call `crate::simd::<arch>::<family>::
<family>_dsp_init_<arch>_8bpc`, the owned mirror of the original's
`<family>_dsp_init_<arch>` (src/<arch>/<family>.h). The committed list is
`crates/dav1d/dispatch.txt`.

The translation is made with `-Denable_asm=false` (ADR 0025), so its
`dav1d_<family>_dsp_init_<bpc>bpc` functions assign only C kernels and
`dav1d_init_cpu` is empty. This tool makes the two edits the original's
`HAVE_ASM` code makes, in the owned translated units (ADR 0059) of every
profile of the matching target, and nothing else:

1. `dav1d_init_cpu` (src/cpu.c) stores `crate::simd::cpu::
   dav1d_get_cpu_flags_native()` in `dav1d_cpu_flags`, in all four profiles.
2. For each listed entry, the family's init calls the owned function where
   the original calls `<family>_dsp_init_<arch>` (for itx: after `all_simd`
   is declared and before it is read). `arm` entries go into the macOS
   profiles (`src/`, `src/release/`, aarch64), `x86` entries into the Linux
   profiles (`src/linux/`, `src/linux/release/`, x86-64).

3. For the entry `msac_<arch>` (the entropy decoder; no bit depth): the
   original selects msac's assembly at compile time, by renaming the seven
   calls of `src/msac.h` in every unit that includes it (`src/arm/msac.h`).
   The translation calls the C functions by name. Each call site in the
   translated `decode`, `recon_tmpl` (both bit depths) and `msac` units is
   renamed to the owned function `crate::simd::<arch>::msac::<macro name>`
   that the original's macro names at that call site; the three symbol
   sizes, which the translation calls as one C function, follow
   `SYMBOL_ADAPT` (the macros of the original's call sites, in order).
   Definitions, declarations and string literals keep their C names (the
   Linux debug profiles carry glibc's assertion strings with the enclosing
   function's C signature). For `msac_x86` the owned functions are those of
   `src/x86/msac.h`: SSE2 kernels chosen at compile time, the 16-symbol
   function by the CPU flags (`simd/x86/msac.rs`).

Each inserted statement ends with the marker comment `// dav1d-dispatch`;
a renamed call is recognizable by its `crate::simd::<arch>::msac::` path.
`--check` writes nothing: it removes every marked statement from each unit,
re-applies the committed list and requires the result to equal the unit,
and it requires the recorded digests (`tools/check_translation.py`'s
definition) to match. `tools/transpile_dav1d.py` applies `dispatch_text` after
`tools/stable_rust.py`'s rewrites, so a regeneration reproduces the edits.
libavif-rs ADR 0062 records the approach.
"""
import hashlib
import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
CRATE = ROOT / 'crates/dav1d'
LIST = CRATE / 'dispatch.txt'
MARKER = '// dav1d-dispatch'
# profile directory below crates/dav1d/src -> (manifest, architecture)
PROFILES = {
    '': ('TRANSLATION.json', 'arm'),
    'release/': ('TRANSLATION-release.json', 'arm'),
    'linux/': ('TRANSLATION-linux.json', 'x86'),
    'linux/release/': ('TRANSLATION-linux-release.json', 'x86'),
}
# family -> (unit, init function, anchor statement after which the call goes;
# None: at the end of the init, where the original calls it after the C
# assignments)
FAMILIES = {
    'cdef': ('cdef_tmpl.rs', 'dav1d_cdef_dsp_init_{bpc}bpc', None),
    'ipred': ('ipred_tmpl.rs', 'dav1d_intra_pred_dsp_init_{bpc}bpc', None),
    'itx': ('itx_tmpl.rs', 'dav1d_itx_dsp_init_{bpc}bpc',
            'let mut all_simd: core::ffi::c_int = 0 as core::ffi::c_int;'),
    'loopfilter': ('loopfilter_tmpl.rs', 'dav1d_loop_filter_dsp_init_{bpc}bpc', None),
    'looprestoration': ('looprestoration_tmpl.rs', 'dav1d_loop_restoration_dsp_init_{bpc}bpc', None),
}
CALLS = {
    'cdef': 'crate::simd::{arch}::cdef::cdef_dsp_init_{arch}_{bpc}bpc(c.cast());',
    'ipred': 'crate::simd::{arch}::ipred::intra_pred_dsp_init_{arch}_{bpc}bpc(c.cast());',
    'itx': 'crate::simd::{arch}::itx::itx_dsp_init_{arch}_{bpc}bpc(c.cast(), bpc, &raw mut all_simd);',
    'loopfilter': 'crate::simd::{arch}::loopfilter::loop_filter_dsp_init_{arch}_{bpc}bpc(c.cast());',
    'looprestoration': ('crate::simd::{arch}::looprestoration::loop_restoration_dsp_init_{arch}_{bpc}bpc('
                        'c.cast(), bpc);'),
}
INIT_CPU_EMPTY = 'pub unsafe extern "C" fn dav1d_init_cpu() {}'
INIT_CPU = ('pub unsafe extern "C" fn dav1d_init_cpu() {\n'
            f'    dav1d_cpu_flags = crate::simd::cpu::dav1d_get_cpu_flags_native(); {MARKER}\n'
            '}')
ENTRY = re.compile(r'(?P<family>[a-z]+)(?:_(?P<bpc>8|16)bpc)?_(?P<arch>arm|x86)')
# Families without a bit depth, and the units whose calls they rename.
RENAMED = {'msac': ('common/decode.rs', 'b8/recon_tmpl.rs', 'b16/recon_tmpl.rs', 'common/msac.rs')}
MSAC_CALLS = ('symbol_adapt', 'bool_adapt', 'bool_equi', 'bool', 'hi_tok')
# The macro of each `dav1d_msac_decode_symbol_adapt{4,8,16}` call site of the
# original, in source order, per unit (decode.c; recon_tmpl.c with its
# `case_sz` and three `DECODE_COEFS_CLASS` expansions; msac.c's
# `dav1d_msac_decode_hi_tok_c`). The translation keeps that order; the unit
# test `tests/ports/enable_dav1d_dispatch_test.py` derives these lists
# from the original sources.
SYMBOL_ADAPT = {
    'common/decode.rs': [16, 4, 4, 4, 8, 8, 8, 4, 4, 16, 8, 16, 8, 16, 16, 8, 8, 4, 8, 16, 4, 16, 4, 4, 4, 16, 4],
    'b8/recon_tmpl.rs': [8, 8, 16, 16, 8, 8, 8, 8, 16, 16, 16, 4, 4, 4, 4, 4, 4, 4, 4, 8],
    'b16/recon_tmpl.rs': [8, 8, 16, 16, 8, 8, 8, 8, 16, 16, 16, 4, 4, 4, 4, 4, 4, 4, 4, 8],
    'common/msac.rs': [4, 4, 4, 4],
}
CALL = re.compile(r'(?<![\w"])(?<!fn )dav1d_msac_decode_(' + '|'.join(MSAC_CALLS) + r')_c\(')
RENAMED_CALL = re.compile(r'crate::simd::(?:arm|x86)::msac::dav1d_msac_decode_('
                          + '|'.join(MSAC_CALLS) + r')(?:4|8|16)?\(')


def parse_entry(entry):
    match = ENTRY.fullmatch(entry)
    family = match and match['family']
    if not match or (family not in FAMILIES and family not in RENAMED) \
            or (family in FAMILIES) != (match['bpc'] is not None):
        raise SystemExit(f'unknown entry {entry!r}; expected <family>_<8|16>bpc_<arm|x86> '
                         f'with family in {sorted(FAMILIES)} or <family>_<arm|x86> with family in '
                         f'{sorted(RENAMED)}')
    return family, match['bpc'], match['arch']


def in_string(text, position):
    """Whether `position` lies in a string literal of its line.

    glibc's assertion strings in the Linux debug profiles carry the enclosing
    function's C signature (`*b"unsigned int dav1d_msac_decode_symbol_adapt_c(
    MsacContext *const, ...)\\0"`), which reads like a call."""
    return text[text.rfind('\n', 0, position) + 1:position].count('"') % 2 == 1


def rename_calls(rel, text, unit, arch):
    """msac: each C call to the owned function the original's macro names."""
    sizes = iter(SYMBOL_ADAPT[unit])
    count = len(SYMBOL_ADAPT[unit])
    found = len([m for m in CALL.finditer(text) if m[1] == 'symbol_adapt' and not in_string(text, m.start())])
    if found != count:
        raise SystemExit(f'{rel}: {found} symbol_adapt calls, SYMBOL_ADAPT lists {count}')

    def replace(match):
        if in_string(text, match.start()):
            return match[0]
        size = str(next(sizes)) if match[1] == 'symbol_adapt' else ''
        return f'crate::simd::{arch}::msac::dav1d_msac_decode_{match[1]}{size}('
    return CALL.sub(replace, text)


def read_list():
    if not LIST.exists():
        return []
    entries = [line.strip() for line in LIST.read_text().splitlines()
               if line.strip() and not line.startswith('#')]
    for entry in entries:
        parse_entry(entry)
    return entries


def list_text(entries):
    return ('# Families dispatched to the rebuilt kernels (tools/enable_dav1d_dispatch.py).\n'
            + ''.join(f'{e}\n' for e in sorted(set(entries))))


def strip(text):
    """The unit without the tool's edits."""
    text = text.replace(INIT_CPU, INIT_CPU_EMPTY)
    text = RENAMED_CALL.sub(lambda m: f'dav1d_msac_decode_{m[1]}_c(', text)
    return ''.join(line for line in text.splitlines(keepends=True) if not line.rstrip().endswith(MARKER))


def function_span(text, name):
    match = re.search(r'pub unsafe extern "C" fn ' + re.escape(name) + r'\(', text)
    if not match:
        raise SystemExit(f'init function not found: {name}')
    start = text.index('{', match.end())
    depth = 0
    for index in range(start, len(text)):
        if text[index] == '{':
            depth += 1
        elif text[index] == '}':
            depth -= 1
            if depth == 0:
                return start, index
    raise SystemExit(f'unbalanced init function: {name}')


def dispatch_text(rel, text, entries=None):
    """The unit `rel` (relative to ROOT) with the edits for `entries`.

    Idempotent; takes the committed list when `entries` is None."""
    entries = read_list() if entries is None else entries
    text = strip(text)
    prefix = rel.removeprefix('crates/dav1d/src/')
    profile = next((p for p in sorted(PROFILES, key=len, reverse=True)
                    if prefix.startswith(p) and prefix[len(p):].count('/') == 1), None)
    if profile is None:
        return text
    group, unit = prefix[len(profile):].split('/')
    if group == 'common' and unit == 'cpu.rs':
        if text.count(INIT_CPU_EMPTY) != 1:
            raise SystemExit(f'{rel}: empty dav1d_init_cpu not found')
        return text.replace(INIT_CPU_EMPTY, INIT_CPU)
    arch = PROFILES[profile][1]
    for entry in sorted(entries):
        family, bpc, entry_arch = parse_entry(entry)
        if family in RENAMED:
            if entry_arch == arch and f'{group}/{unit}' in RENAMED[family]:
                text = rename_calls(rel, text, f'{group}/{unit}', arch)
            continue
        if entry_arch != arch or group != f'b{bpc}' or unit != FAMILIES[family][0]:
            continue
        _, init, anchor = FAMILIES[family]
        start, end = function_span(text, init.format(bpc=bpc))
        body = text[start:end]
        call = CALLS[family].format(arch=arch, bpc=bpc)
        if anchor is None:
            # Before the closing brace: after the last statement.
            position = start + len(body.rstrip())
            text = text[:position] + f'\n    {call} {MARKER}' + text[position:]
            continue
        if body.count(anchor) != 1:
            raise SystemExit(f'{rel}: anchor of {entry} not found once')
        position = start + body.index(anchor) + len(anchor)
        text = text[:position] + f'\n    {call} {MARKER}' + text[position:]
    return text


def units():
    """(relative unit path, manifest path) of every dispatch-relevant unit."""
    out = []
    for profile, (manifest, _) in PROFILES.items():
        path = CRATE / manifest
        for unit in json.loads(path.read_text())['units']:
            rel = unit['rust']
            name = rel.rsplit('/', 1)[1]
            renamed = {u for units_ in RENAMED.values() for u in units_}
            if name == 'cpu.rs' and '/common/' in rel or name in {f[0] for f in FAMILIES.values()} \
                    or '/'.join(rel.rsplit('/', 2)[1:]) in renamed:
                if rel.startswith(f'crates/dav1d/src/{profile}') and rel[len(f'crates/dav1d/src/{profile}'):].count('/') == 1:
                    out.append((rel, path))
    return out


def digests(text):
    """`tools/check_translation.py`'s digest definition."""
    production = text.split('\n#[cfg(feature = "port-test-bridges")]', 1)[0]
    return hashlib.sha256(text.encode()).hexdigest(), hashlib.sha256(production.encode()).hexdigest()


def stale_digests():
    stale = []
    for manifest in {m for m, _ in PROFILES.values()}:
        path = CRATE / manifest
        for unit in json.loads(path.read_text())['units']:
            full, production = digests((ROOT / unit['rust']).read_text())
            if unit['rust_sha256'] != full or unit.get('production_rust_sha256', production) != production:
                stale.append(unit['rust'])
    return stale


def rehash():
    for manifest in sorted({m for m, _ in PROFILES.values()}):
        path = CRATE / manifest
        record = json.loads(path.read_text())
        dirty = False
        for unit in record['units']:
            full, production = digests((ROOT / unit['rust']).read_text())
            if unit['rust_sha256'] != full:
                unit['rust_sha256'] = full
                dirty = True
            if 'production_rust_sha256' in unit and unit['production_rust_sha256'] != production:
                unit['production_rust_sha256'] = production
                dirty = True
        if dirty:
            path.write_text(json.dumps(record, indent=2) + '\n')


def apply(entries):
    changed = 0
    for rel, _ in units():
        path = ROOT / rel
        text = path.read_text()
        new = dispatch_text(rel, text, entries)
        if new != text:
            path.write_text(new)
            changed += 1
    rehash()
    return changed


def check():
    entries = read_list()
    if LIST.exists() and LIST.read_text() != list_text(entries):
        raise SystemExit(f'{LIST.relative_to(ROOT)} is not in canonical form')
    differing = [rel for rel, _ in units()
                 if dispatch_text(rel, (ROOT / rel).read_text(), entries) != (ROOT / rel).read_text()]
    stale = stale_digests()
    for rel in differing:
        print(f'not dispatched as listed: {rel}')
    for rel in stale:
        print(f'stale digest: {rel}')
    if differing or stale:
        raise SystemExit('the tree differs from the output of tools/enable_dav1d_dispatch.py')
    print(f'dav1d dispatch up to date: {len(entries)} entries, {len(units())} units')


def main():
    args = sys.argv[1:]
    if args == ['--check']:
        return check()
    if args == ['--rehash']:
        return rehash()
    if args == ['--apply']:
        print(f'changed {apply(read_list())} units')
        return
    if not args or any(a.startswith('-') for a in args):
        raise SystemExit(__doc__)
    for entry in args:
        parse_entry(entry)
    entries = sorted(set(read_list()) | set(args))
    LIST.write_text(list_text(entries))
    print(f'changed {apply(entries)} units')


if __name__ == '__main__':
    main()
