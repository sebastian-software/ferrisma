#!/usr/bin/env python3
"""Enable RTCD dispatch for named functions in the frozen translation.

Usage: python3 tools/enable_aom_dispatch.py NAME...            (Neon, macOS ARM64)
       python3 tools/enable_aom_dispatch.py --linux NAME_EXT...  (x86, GNU Linux x86-64)
       python3 tools/enable_aom_dispatch.py --linux --check      (integer aliases of the Linux RTCD units)

With --linux each argument is one specialization, `<function>_<sse2|ssse3|sse4_1|avx2>`,
as crates/aom/rtcd/x86_64.pl lists them; the tool dispatches every listed
variant of a function (ADR 0058, amendment of 2026-11-14).

ADR 0059 makes the RTCD units owned code; ADR 0056 (amendment of 2026-11-13)
describes this tool.

1. Adds NAME to crates/aom/rtcd/arm64.pl.
2. Regenerates the dispatch headers with tools/transpile_aom.py:dispatch_headers
   (the original cmake/rtcd.pl plus the overlay) into
   crates/aom/reference/macos/<profile>/dispatch and records them in the
   manifest's rtcd_dispatch block.
3. Edits the owned Rust units the way C2Rust translates a dispatched RTCD
   function: an RTCD pointer global plus setup in the RTCD unit, and pointer
   calls and declarations in each caller whose C source uses the macro name.
4. Records the new digests of every changed unit in the manifest.
"""
import hashlib
import json
import re
import shutil
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
LINUX = sys.argv[1:2] == ['--linux']
NAMES = sys.argv[2:] if LINUX else sys.argv[1:]
X86_EXTENSIONS = ('sse2', 'ssse3', 'sse4_1', 'avx2')
X86_FLAGS = {'sse2': 'HAS_SSE2', 'ssse3': 'HAS_SSSE3', 'sse4_1': 'HAS_SSE4_1', 'avx2': 'HAS_AVX2'}
sys.path.insert(0, str(ROOT / 'tools'))
import transpile_aom  # noqa: E402

CRATE = ROOT / 'crates/aom'
OVERLAY = CRATE / ('rtcd/x86_64.pl' if LINUX else 'rtcd/arm64.pl')
PLATFORM = 'linux' if LINUX else 'macos'
RTCD_UNITS = {'config/aom_dsp_rtcd.h': 'aom_dsp/aom_dsp_rtcd.rs',
              'config/av1_rtcd.h': 'av1/common/av1_rtcd.rs',
              'config/aom_scale_rtcd.h': 'aom_scale/aom_scale_rtcd.rs'}
EXPECT = '.expect("non-null function pointer")'


def update_overlay():
    text = OVERLAY.read_text()
    table = 'ported_x86' if LINUX else 'ported_neon'
    m = re.search(r'(my %' + table + r' = map \{ \$_ => 1 \} qw/\n)(.*?)(/;\n)', text, re.S)
    names = set(m.group(2).split()) | set(NAMES)
    body = ''.join(f'  {n}\n' for n in sorted(names))
    OVERLAY.write_text(text[:m.start(2)] + body + text[m.end(2):])


def split_top(s):
    parts, depth, cur = [], 0, ''
    for ch in s:
        if ch in '(<[':
            depth += 1
        elif ch in ')>]':
            depth -= 1
        if ch == ',' and depth == 0:
            parts.append(cur.strip())
            cur = ''
        else:
            cur += ch
    if cur.strip():
        parts.append(cur.strip())
    return parts


def find_signature(units_text, name):
    """(params text with names, return text or '') from a translated decl/def."""
    decl = re.compile(r'\n\s*fn ' + name + r'_c\((.*?)\)(\s*->\s*[^;{]+?)?;', re.S)
    for text in units_text.values():
        m = decl.search(text)
        if m:
            return m.group(1), (m.group(2) or '').strip()
    defn = re.compile(r'pub unsafe extern "C-unwind" fn ' + name + r'_c\((.*?)\)(\s*->\s*[^{]+?)?\s*\{', re.S)
    for text in units_text.values():
        m = defn.search(text)
        if m:
            params = ', '.join(p.removeprefix('mut ') for p in split_top(m.group(1)))
            return params, (m.group(2) or '').strip()
    raise SystemExit(f'No signature for {name}_c')


def fn_type(params, ret):
    types = [p.split(':', 1)[1].strip() for p in split_top(params)]
    ret = ret if ret else '-> ()'
    return f'unsafe extern "C-unwind" fn({", ".join(types)}) {ret}'


def header_order(header_text):
    return re.findall(r'^\s*(\w+) = \1_c;', header_text, re.M)


def edit_rtcd_unit(text, name, params, ret, order):
    ftype = fn_type(params, ret)
    opt = f'Option<{ftype}>'
    if re.search(r'pub static mut ' + name + r':', text):
        raise SystemExit(f'{name} is already dispatched')
    # extern declarations
    decl_params = ', '.join(split_top(params))
    r = f' {ret}' if ret else ''
    anchor = '    fn aom_arm_cpu_caps() -> core::ffi::c_int;\n'
    assert anchor in text
    text = text.replace(anchor, f'    fn {name}_c({decl_params}){r};\n    fn {name}_neon({decl_params}){r};\n' + anchor, 1)
    # pointer global, alphabetical among the existing ones
    static = f'#[no_mangle]\npub static mut {name}: {opt} = None;\n'
    statics = [(m.start(), m.group(1)) for m in re.finditer(r'^#\[no_mangle\]\npub static mut (\w+):', text, re.M)]
    later = [pos for pos, n in statics if n > name]
    pos = later[0] if later else text.index('pub const HAS_NEON')
    text = text[:pos] + static + text[pos:]
    # setup assignment in header order
    block = (f'    {name} = Some({name}_c as {ftype}) as {opt};\n'
             f'    if flags & HAS_NEON != 0 {{\n'
             f'        {name} = Some({name}_neon as {ftype}) as {opt};\n'
             f'    }}\n')
    start = text.index('unsafe extern "C-unwind" fn setup_rtcd_internal() {')
    end = text.index('\n}\n', start) + 1
    idx = order.index(name)
    insert = end
    for later_name in order[idx + 1:]:
        m = re.compile(r'^    ' + later_name + r' = Some\(', re.M).search(text, start, end)
        if m:
            insert = m.start()
            break
    return text[:insert] + block + text[insert:]


def header_variants(header_text, name):
    """The x86 specializations the generated setup assigns to `name`, in order."""
    return re.findall(r'if \(flags & HAS_\w+\) ' + name + r' = ' + name + r'_(\w+);', header_text)


INT_ALIASES = {f'{u}int{w}_t': (w, f'{"u" if u else "i"}{w}') for w in (8, 16, 32, 64) for u in ('', 'u')}


def ensure_int_aliases(text, ftype):
    """The glibc `<stdint.h>` aliases (`pub type __int64_t = i64;` and
    `pub type int64_t = __int64_t;`) that a new declaration in a Linux RTCD
    unit names but the unit does not define yet, inserted as C2Rust lays them
    out: each after the last alias of a smaller width in its group."""
    for name, (width, prim) in sorted(INT_ALIASES.items()):
        if not re.search(r'\b' + name + r'\b', ftype) or re.search(r'^pub type ' + name + ' = ', text, re.M):
            continue
        for alias, value in ((f'__{name}', prim), (name, f'__{name}')):
            if re.search(r'^pub type ' + alias + ' = ', text, re.M):
                continue
            group = r'__u?int(\d+)_t' if alias.startswith('__') else r'u?int(\d+)_t'
            lines = [m for m in re.finditer(r'^pub type (' + group + r') = [^\n]*\n', text, re.M)
                     if int(m.group(2)) < width]
            if not lines:
                raise SystemExit(f'no place for pub type {alias} in the RTCD unit')
            pos = lines[-1].end()
            text = text[:pos] + f'pub type {alias} = {value};\n' + text[pos:]
    return text


RUST_WORDS = {'unsafe', 'extern', 'fn', 'mut', 'const', 'Option', 'core', 'ffi', 'c_int', 'c_uint', 'c_char',
              'c_void', 'c_float', 'c_double', 'c_long', 'c_ulong', 'c_short', 'c_ushort', 'c_uchar', 'c_schar', 'isize', 'usize', 'i8',
              'i16', 'i32', 'i64', 'u8', 'u16', 'u32', 'u64', 'f32', 'f64', 'bool', 'C', 'unwind'}


def type_definition(text, name):
    """The text of `pub type NAME = ...;` (rustfmt may wrap it) in a unit, or
    of a plain `pub struct NAME { ... }` with its attribute lines, or None."""
    m = re.search(r'^((?:#\[[^\n]*\]\n)*)pub struct ' + name + r' \{\n(?:    [^\n]*\n)*\}\n', text, re.M)
    if m:
        return m.group(0)
    m = re.search(r'^pub type ' + name + r' = ', text, re.M)
    if not m:
        return None
    depth, k = 0, m.end()
    while True:
        ch = text[k]
        if ch in '<([':
            depth += 1
        elif ch in '>)]' and text[k - 1] != '-':
            depth -= 1
        elif ch == ';' and depth == 0:
            return text[m.start():k + 1] + '\n'
        k += 1


def ensure_copied_types(text, ftype, others):
    """Type aliases and records a new RTCD declaration names (the CfL
    function-pointer types, WienerConvolveParams) that the unit lacks, copied verbatim from the first unit of the
    same profile that defines them, after the unit's last `pub type`."""
    for name in sorted(set(re.findall(r'\b([A-Za-z_]\w*)\b', ftype)) - RUST_WORDS - set(INT_ALIASES)):
        if re.search(r'^pub (type|struct|union) ' + name + r'\b', text, re.M):
            continue
        definition = next((d for d in (type_definition(o, name) for o in others) if d), None)
        if definition is None:
            raise SystemExit(f'no definition of type {name} to copy into the RTCD unit')
        last = list(re.finditer(r'^pub type (\w+) = ', text, re.M))[-1]
        pos = last.start() + len(type_definition(text[last.start():], last.group(1)))
        text = text[:pos] + definition + text[pos:]
        text = ensure_int_aliases(text, definition)
    return text


def check_int_aliases():
    """--check: every integer alias the Linux RTCD units use is defined there
    in its canonical form."""
    bad = []
    for profile in ['release', 'debug']:
        for rel in RTCD_UNITS.values():
            path = CRATE / f'src/linux/{profile}' / rel
            text = path.read_text()
            used = ' '.join(re.findall(r'^pub static mut \w+: (Option<.*?>) = None;', text, re.M | re.S))
            others = [p.read_text() for p in sorted((CRATE / f'src/linux/{profile}').rglob('*.rs')) if p != path]
            if ensure_int_aliases(text, text) != text or ensure_copied_types(text, used, others) != text:
                bad.append(str(path.relative_to(ROOT)))
    if bad:
        raise SystemExit(f'type aliases missing: {bad}')
    print('Linux RTCD units: integer and copied type aliases complete')


def edit_rtcd_unit_linux(text, name, params, ret, order, variants, others=()):
    ftype = fn_type(params, ret)
    opt = f'Option<{ftype}>'
    if re.search(r'pub static mut ' + name + r':', text):
        raise SystemExit(f'{name} is already dispatched')
    text = ensure_int_aliases(text, ftype)
    text = ensure_copied_types(text, ftype, others)
    decl_params = ', '.join(split_top(params))
    r = f' {ret}' if ret else ''
    anchor = 'extern "C-unwind" {\n'
    decls = ''.join(f'    fn {name}_{ext}({decl_params}){r};\n' for ext in ['c', *variants])
    text = text.replace(anchor, anchor + decls, 1)
    static = f'#[no_mangle]\npub static mut {name}: {opt} = None;\n'
    statics = [(m.start(), m.group(1)) for m in re.finditer(r'^#\[no_mangle\]\npub static mut (\w+):', text, re.M)]
    later = [pos for pos, n in statics if n > name]
    pos = later[0] if later else text.index('pub const HAS_MMX')
    text = text[:pos] + static + text[pos:]
    block = f'    {name} = Some({name}_c as {ftype}) as {opt};\n'
    for ext in variants:
        block += (f'    if flags & {X86_FLAGS[ext]} != 0 {{\n'
                  f'        {name} = Some({name}_{ext} as {ftype}) as {opt};\n'
                  f'    }}\n')
    start = text.index('unsafe extern "C-unwind" fn setup_rtcd_internal() {')
    end = text.index('\n}\n', start) + 1
    idx = order.index(name)
    insert = end
    for later_name in order[idx + 1:]:
        m = re.compile(r'^    ' + later_name + r' = Some\(', re.M).search(text, start, end)
        if m:
            insert = m.start()
            break
    return text[:insert] + block + text[insert:]


LITERAL = re.compile(r'''b?r(#*)".*?"\1|b?"(?:\\.|[^"\\])*"|b?'(?:\\.|[^'\\])'|//[^\n]*''', re.S)


def sub_code(pattern, repl, text):
    """pattern.subn() applied only outside string, byte-string and char
    literals and line comments: the debug profiles keep the C expression and
    function text of `assert` as byte strings, which must stay unchanged."""
    # ABI strings (`extern "C-unwind"`) are part of the code the patterns match.
    abi = '\x00ABI\x00'
    text = text.replace('extern "C-unwind"', abi)
    out, total, pos = [], 0, 0
    for m in LITERAL.finditer(text):
        code, n = pattern.subn(repl, text[pos:m.start()].replace(abi, 'extern "C-unwind"'))
        out += [code, m.group(0)]
        total += n
        pos = m.end()
    code, n = pattern.subn(repl, text[pos:].replace(abi, 'extern "C-unwind"'))
    out.append(code)
    return ''.join(out).replace(abi, 'extern "C-unwind"'), total + n


def count_code(pattern, text):
    return sub_code(pattern, lambda m: m.group(0), text)[1]


def edit_caller(text, csource, name, params, ret, report, rel):
    ftype = fn_type(params, ret)
    opt = f'Option<{ftype}>'
    # Explicit uses of F_c in the C source, other than its own definition or
    # prototype (a line that starts with the return type), stay direct calls.
    definition = re.compile(r'^[A-Za-z_][\w \*]*\b' + name + r'_c\(', re.M)
    explicit_c = len(re.findall(r'\b' + name + r'_c\b', csource)) - len(definition.findall(csource))
    if explicit_c:
        report.append(f'MANUAL {rel}: C source names {name}_c explicitly {explicit_c}x; left unchanged')
        return text
    # function-pointer values: Some(F_c as fn(...) -> R)
    ptr = re.compile(r'Some\(\s*' + name + r'_c\s+as\s+unsafe extern "C-unwind" fn\((?:[^()]|\([^()]*\))*\)\s*->\s*[^,()]+?(?:\(\))?,?\s*\)', re.S)
    text, n_ptr = sub_code(ptr, name, text)
    call = re.compile(r'(?<!fn )\b' + name + r'_c\(')
    text, n_call = sub_code(call, name + EXPECT + '(', text)
    if not (n_ptr or n_call):
        return text
    report.append(f'{rel}: {name} calls={n_call} pointers={n_ptr}')
    decl = re.compile(r'\n(\s*)fn ' + name + r'_c\((.*?)\)(\s*->\s*[^;{]+?)?;', re.S)
    static = f'static mut {name}: {opt};'
    remaining = count_code(re.compile(r'\b' + name + r'_c\b'), text)
    m = decl.search(text)
    if m and remaining == 1:
        text = text[:m.start()] + '\n' + m.group(1) + static + text[m.end():]
    else:
        m2 = re.search(r'extern "C-unwind" \{\n', text)
        if not m2:
            raise SystemExit(f'No extern block in {rel}')
        text = text[:m2.end()] + '    ' + static + '\n' + text[m2.end():]
    return text


def rehash():
    """Record the digests of hand-edited owned units (both macOS profiles)."""
    for profile in ['release', 'debug']:
        manifest_path = CRATE / f'TRANSLATION-{PLATFORM}-{profile}.json'
        manifest = json.loads(manifest_path.read_text())
        for unit in manifest['units']:
            text = (ROOT / unit['rust']).read_text()
            digest = hashlib.sha256(text.encode()).hexdigest()
            if digest != unit['rust_sha256']:
                print(f'rehash {unit["rust"]}')
                unit['rust_sha256'] = digest
                if 'production_rust_sha256' in unit:
                    production = text.split('\n#[cfg(feature = "port-test-bridges")]', 1)[0]
                    unit['production_rust_sha256'] = hashlib.sha256(production.encode()).hexdigest()
        manifest_path.write_text(json.dumps(manifest, indent=2) + '\n')


def main():
    if LINUX and NAMES == ['--check']:
        check_int_aliases()
        return
    if NAMES == ['--rehash']:
        rehash()
        return
    update_overlay()
    report = []
    functions = NAMES
    if LINUX:
        functions = []
        for spec in NAMES:
            base, ext = re.fullmatch(r'(\w+?)_(sse2|ssse3|sse4_1|avx2)', spec).groups()
            if base not in functions:
                functions.append(base)
    for profile in ['release', 'debug']:
        ref = CRATE / f'reference/{PLATFORM}/{profile}'
        manifest_path = CRATE / f'TRANSLATION-{PLATFORM}-{profile}.json'
        manifest = json.loads(manifest_path.read_text())
        with tempfile.TemporaryDirectory() as tmp:
            dispatch = transpile_aom.dispatch_headers(PLATFORM, ref, Path(tmp))
            headers = {n: (Path(tmp) / n).read_text() for n in dispatch['headers']}
            for n in dispatch['headers']:
                shutil.copy(Path(tmp) / n, ref / 'dispatch' / n)
        manifest['rtcd_dispatch'] = dispatch
        units = {u['rust']: u for u in manifest['units']}
        texts = {r: (ROOT / r).read_text() for r in units}
        original = dict(texts)
        # Which units mention which `<name>_c`, computed once.
        mentions = {r: set(re.findall(r'\b(\w+)_c\b', t)) for r, t in texts.items()}
        for name in functions:
            header = [h for h, t in headers.items() if re.search(r'\(\*' + name + r'\)\(', t)]
            if len(header) != 1:
                raise SystemExit(f'{name}: not dispatched by the regenerated headers ({header})')
            rtcd_rel = f'crates/aom/src/{PLATFORM}/{profile}/' + RTCD_UNITS[header[0]]
            candidates = {r: texts[r] for r in units if name in mentions[r]}
            params, ret = find_signature(candidates, name)
            if LINUX:
                variants = header_variants(headers[header[0]], name)
                wanted = [spec[len(name) + 1:] for spec in NAMES
                          if spec.startswith(name + '_') and spec[len(name) + 1:] in X86_EXTENSIONS]
                if sorted(variants) != sorted(wanted):
                    raise SystemExit(f'{name}: header dispatches {variants}, requested {wanted}')
                others = [texts[r] for r in sorted(units) if r != rtcd_rel]
                texts[rtcd_rel] = edit_rtcd_unit_linux(texts[rtcd_rel], name, params, ret,
                                                       header_order(headers[header[0]]), variants, others)
            else:
                texts[rtcd_rel] = edit_rtcd_unit(texts[rtcd_rel], name, params, ret,
                                                 header_order(headers[header[0]]))
            for rel, unit in units.items():
                if rel == rtcd_rel or name not in mentions[rel]:
                    continue
                csource = (ROOT / unit['source']).read_text()
                texts[rel] = edit_caller(texts[rel], csource, name, params, ret, report, f'{profile}:{rel}')
        for rel, unit in units.items():
            if texts[rel] != original[rel]:
                (ROOT / rel).write_text(texts[rel])
                unit['rust_sha256'] = hashlib.sha256(texts[rel].encode()).hexdigest()
                if 'production_rust_sha256' in unit:
                    production = texts[rel].split('\n#[cfg(feature = "port-test-bridges")]', 1)[0]
                    unit['production_rust_sha256'] = hashlib.sha256(production.encode()).hexdigest()
        manifest_path.write_text(json.dumps(manifest, indent=2) + '\n')
    print('\n'.join(report))


if __name__ == '__main__':
    main()
