#!/usr/bin/env python3
"""Enable RTCD dispatch for named functions in the frozen translation.

Usage: python3 tools/enable_aom_dispatch.py NAME...            (Neon, macOS ARM64)
       python3 tools/enable_aom_dispatch.py --linux NAME_EXT...  (x86, GNU Linux x86-64)
       python3 tools/enable_aom_dispatch.py [--linux] --rehash   (record the digests of the units)
       python3 tools/enable_aom_dispatch.py [--linux] --check    (compare the tree with the tool's output)

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

`--check` writes nothing. For the committed overlay it re-derives what the
tool writes and fails on any difference from the tree (ADR 0059, amendment
of 2026-11-15): the overlay's canonical form; the dispatch headers of both
profiles and the manifest's `rtcd_dispatch` block; in each RTCD unit the
pointer global of every dispatched function, its `_c` and variant
declarations, and the whole body of `setup_rtcd_internal`, statement by
statement from the regenerated header (whitespace and trailing commas aside,
so the setup C2Rust emitted at the freeze compares like the tool's); that no
other unit still calls or takes the address of a dispatched `_c` function
where the tool would rewrite it; the recorded digests of every unit; and on
Linux the integer and copied type aliases. CI runs both platforms.
"""
import hashlib
import json
import re
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


def overlay_table(text, linux):
    """The match of the overlay's table of ported names."""
    table = 'ported_x86' if linux else 'ported_neon'
    return re.search(r'(my %' + table + r' = map \{ \$_ => 1 \} qw/\n)(.*?)(/;\n)', text, re.S)


def overlay_text(text, names, linux):
    """The overlay with `names` added to its table, sorted one per line."""
    m = overlay_table(text, linux)
    names = set(m.group(2).split()) | set(names)
    body = ''.join(f'  {n}\n' for n in sorted(names))
    return text[:m.start(2)] + body + text[m.end(2):]


def update_overlay():
    """Add NAMES to the overlay; return its previous text."""
    text = OVERLAY.read_text()
    OVERLAY.write_text(overlay_text(text, NAMES, LINUX))
    return text


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


def edit_rtcd_unit(text, name, params, ret, order, others=()):
    ftype = fn_type(params, ret)
    opt = f'Option<{ftype}>'
    if re.search(r'pub static mut ' + name + r':', text):
        raise SystemExit(f'{name} is already dispatched')
    text = ensure_copied_types(text, ftype, others, linux=False)
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
    out: each group ordered by width, the signed alias before the unsigned one
    of the same width."""
    for name, (width, prim) in sorted(INT_ALIASES.items()):
        if not re.search(r'\b' + name + r'\b', ftype) or re.search(r'^pub type ' + name + ' = ', text, re.M):
            continue
        for alias, value in ((f'__{name}', prim), (name, f'__{name}')):
            if re.search(r'^pub type ' + alias + ' = ', text, re.M):
                continue
            group = r'__(u?)int(\d+)_t' if alias.startswith('__') else r'(u?)int(\d+)_t'
            key = (width, name.startswith('u'))
            members = list(re.finditer(r'^pub type (' + group + r') = [^\n]*\n', text, re.M))
            before = [m for m in members if (int(m.group(3)), m.group(2) == 'u') < key]
            if before:
                pos = before[-1].end()
            elif members:
                pos = members[0].start()
            else:
                raise SystemExit(f'no place for pub type {alias} in the RTCD unit')
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


def definition_types(definition):
    """The text of the types a copied definition names: the right-hand side
    of a `pub type`, or the field types of a `pub struct`."""
    if re.search(r'^pub struct', definition, re.M):
        return ' '.join(re.findall(r'^    pub \w+: (.*),$', definition, re.M))
    return definition.split('=', 1)[1]


def ensure_copied_types(text, ftype, others, linux=True):
    """Type aliases and records a new RTCD declaration names (the CfL
    function-pointer types, WienerConvolveParams, CNN_LAYER_CONFIG) that the
    unit lacks, copied verbatim from the first unit of the same profile that
    defines them, after the unit's last `pub type`; then, the same way, the
    types that each copied definition names in turn, as C2Rust gives every
    unit its own copy of the records it uses."""
    for name in sorted(set(re.findall(r'\b([A-Za-z_]\w*)\b', ftype)) - RUST_WORDS - set(INT_ALIASES)):
        if re.search(r'^pub (type|struct|union) ' + name + r'\b', text, re.M):
            continue
        definition = next((d for d in (type_definition(o, name) for o in others) if d), None)
        if definition is None:
            raise SystemExit(f'no definition of type {name} to copy into the RTCD unit')
        last = list(re.finditer(r'^pub type (\w+) = ', text, re.M))[-1]
        pos = last.start() + len(type_definition(text[last.start():], last.group(1)))
        text = text[:pos] + definition + text[pos:]
        if linux:
            text = ensure_int_aliases(text, definition)
        text = ensure_copied_types(text, definition_types(definition), others, linux)
    return text


def check_int_aliases(linux=True):
    """--check: every type the RTCD units' pointer globals name is defined
    there, with the types those definitions name (copied records), and on
    Linux every integer alias in its canonical form."""
    platform = 'linux' if linux else 'macos'
    bad = []
    for profile in ['release', 'debug']:
        for rel in RTCD_UNITS.values():
            path = CRATE / f'src/{platform}/{profile}' / rel
            text = path.read_text()
            used = ' '.join(re.findall(r'^pub static mut \w+: (Option<.*?>) = None;', text, re.M | re.S))
            others = [p.read_text() for p in sorted((CRATE / f'src/{platform}/{profile}').rglob('*.rs')) if p != path]
            if linux and ensure_int_aliases(text, text) != text:
                bad.append(str(path.relative_to(ROOT)))
            elif ensure_copied_types(text, used, others, linux) != text:
                bad.append(str(path.relative_to(ROOT)))
    if bad:
        raise SystemExit(f'type aliases missing: {bad}')
    print(f'{platform} RTCD units: copied type aliases and records complete')


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


def normalized(text):
    """Code without whitespace and without trailing commas, so that the
    rustfmt-wrapped setup C2Rust emitted and the tool's one-line form
    compare equal."""
    text = re.sub(r'\s+', '', text)
    return re.sub(r',([)>\]}])', r'\1', text)


HEADER_SETUP = re.compile(r'static void setup_rtcd_internal\(void\)\n\{\n(.*?)\n\}\n', re.S)
HEADER_ASSIGN = re.compile(r'^\s*(?:if \(flags & (HAS_\w+)\) )?(\w+) = (\w+);$')
RUST_SETUP = 'unsafe extern "C-unwind" fn setup_rtcd_internal() {'


def header_setup(header_text):
    """[(flag or None, function, implementation)] of the generated setup, in order."""
    body = HEADER_SETUP.search(header_text).group(1)
    statements = []
    for line in body.split('\n'):
        line = line.strip()
        if not line or line in ('(void)flags;',) or re.fullmatch(r'int flags = \w+\(\);', line):
            continue
        m = HEADER_ASSIGN.match(line)
        if not m or not m.group(3).startswith(m.group(2) + '_'):
            raise SystemExit(f'unexpected statement in a generated setup_rtcd_internal: {line}')
        statements.append(m.groups())
    return statements


def expected_setup(header_text, signature):
    """The Rust body of setup_rtcd_internal the tool writes for this header;
    `signature(name)` gives the function type of `name`."""
    caps = re.search(r'int flags = (\w+)\(\);', header_text).group(1)
    out = [f'    let mut flags: core::ffi::c_int = {caps}();\n']
    for flag, name, implementation in header_setup(header_text):
        ftype = signature(name)
        assign = f'{name} = Some({implementation} as {ftype}) as Option<{ftype}>;'
        if flag is None:
            if implementation != name + '_c':
                raise SystemExit(f'{name}: the generated setup starts with {implementation}')
            out.append(f'    {assign}\n')
        else:
            out.append(f'    if flags & {flag} != 0 {{\n        {assign}\n    }}\n')
    return ''.join(out)


def rtcd_unit_problems(rel, text, header_text, signature):
    """How one RTCD unit differs from what the tool writes for its header."""
    problems = []
    setup = header_setup(header_text)
    dispatched = list(dict.fromkeys(name for _, name, _ in setup))
    start = text.find(RUST_SETUP)
    if start < 0:
        return [f'{rel}: no setup_rtcd_internal']
    start += len(RUST_SETUP) + 1
    end = text.index('\n}\n', start) + 1
    if normalized(text[start:end]) != normalized(expected_setup(header_text, signature)):
        problems.append(f'{rel}: setup_rtcd_internal differs from the regenerated header')
    statics = dict(re.findall(r'^#\[no_mangle\]\npub static mut (\w+): (Option<.*?>) = None;$', text, re.M | re.S))
    every = set(re.findall(r'^pub static mut (\w+): Option<', text, re.M))
    for name in sorted(every - set(dispatched)):
        problems.append(f'{rel}: pointer global {name} is not dispatched by the regenerated header')
    for name in dispatched:
        ftype = signature(name)
        if name not in statics:
            problems.append(f'{rel}: no #[no_mangle] pointer global for {name}')
        elif normalized(statics[name]) != normalized(f'Option<{ftype}>'):
            problems.append(f'{rel}: pointer global {name} has another type')
        for implementation in dict.fromkeys(i for _, n, i in setup if n == name):
            if len(re.findall(r'^\s*fn ' + implementation + r'\(', text, re.M)) != 1:
                problems.append(f'{rel}: no single declaration of {implementation}')
    return problems


def check(linux):
    """--check: compare the tree with what the tool writes for the committed
    configuration, without writing; exit non-zero on any difference."""
    platform = 'linux' if linux else 'macos'
    overlay = CRATE / ('rtcd/x86_64.pl' if linux else 'rtcd/arm64.pl')
    problems = []
    text = overlay.read_text()
    if overlay_text(text, [], linux) != text:
        problems.append(f'{overlay.relative_to(ROOT)}: table not in the canonical sorted form')
    if linux:
        names = overlay_table(text, linux).group(2).split()
        wanted = {}
        for spec in names:
            m = re.fullmatch(r'(\w+?)_(sse2|ssse3|sse4_1|avx2)', spec)
            if not m:
                problems.append(f'{overlay.relative_to(ROOT)}: {spec} is not <function>_<extension>')
                continue
            wanted.setdefault(m.group(1), set()).add(m.group(2))
    functions = 0
    for profile in ['release', 'debug']:
        ref = CRATE / f'reference/{platform}/{profile}'
        manifest_path = CRATE / f'TRANSLATION-{platform}-{profile}.json'
        manifest = json.loads(manifest_path.read_text())
        with tempfile.TemporaryDirectory() as tmp:
            dispatch = transpile_aom.dispatch_headers(platform, ref, Path(tmp))
            headers = {n: (Path(tmp) / n).read_text() for n in dispatch['headers']}
            for n in dispatch['headers']:
                if (ref / 'dispatch' / n).read_bytes() != (Path(tmp) / n).read_bytes():
                    problems.append(f'{(ref / "dispatch" / n).relative_to(ROOT)}: differs from the regenerated header')
        if manifest.get('rtcd_dispatch') != dispatch:
            problems.append(f'{manifest_path.relative_to(ROOT)}: rtcd_dispatch block differs')
        if manifest_path.read_text() != json.dumps(manifest, indent=2) + '\n':
            problems.append(f'{manifest_path.relative_to(ROOT)}: not in the form the tool writes')
        units = {u['rust']: u for u in manifest['units']}
        texts = {r: (ROOT / r).read_text() for r in units}
        for rel, unit in units.items():
            full = hashlib.sha256(texts[rel].encode()).hexdigest()
            production = hashlib.sha256(texts[rel].split('\n#[cfg(feature = "port-test-bridges")]', 1)[0].encode()).hexdigest()
            if unit['rust_sha256'] != full or unit.get('production_rust_sha256', production) != production:
                problems.append(f'{rel}: recorded digest differs ({manifest_path.name})')
        mentions = {r: set(re.findall(r'\b(\w+)_c\b', t)) for r, t in texts.items()}
        signatures = {}

        def signature(name):
            if name not in signatures:
                candidates = {r: texts[r] for r in units if name in mentions[r]}
                signatures[name] = fn_type(*find_signature(candidates, name))
            return signatures[name]
        rtcd_rels = set()
        for header, unit_rel in RTCD_UNITS.items():
            rel = f'crates/aom/src/{platform}/{profile}/{unit_rel}'
            rtcd_rels.add(rel)
            problems += rtcd_unit_problems(rel, texts[rel], headers[header], signature)
            if linux:
                for flag, name, implementation in header_setup(headers[header]):
                    if flag is not None and implementation[len(name) + 1:] not in wanted.get(name, ()):
                        problems.append(f'{rel}: {implementation} is not listed in the overlay')
        dispatched = sorted({name for h in headers.values() for _, name, _ in header_setup(h)})
        functions = len(dispatched)
        report = []
        for name in dispatched:
            params, ret = find_signature({r: texts[r] for r in units if name in mentions[r]}, name)
            for rel, unit in units.items():
                if rel in rtcd_rels or name not in mentions[rel]:
                    continue
                csource = (ROOT / unit['source']).read_text()
                if edit_caller(texts[rel], csource, name, params, ret, report, rel) != texts[rel]:
                    problems.append(f'{profile}:{rel}: still calls or takes the address of {name}_c')
    try:
        check_int_aliases(linux)
    except SystemExit as error:
        problems.append(str(error))
    if problems:
        print('\n'.join(problems))
        raise SystemExit(f'{len(problems)} differences from the output of tools/enable_aom_dispatch.py')
    print(f'{platform}: RTCD dispatch of {functions} functions in both profiles matches the tool output')


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
    if NAMES == ['--check']:
        check(LINUX)
        return
    if NAMES == ['--rehash']:
        rehash()
        return
    # The header generator reads the overlay from disk, so it is updated first
    # and restored if anything fails; every other file is written only after
    # both profiles were edited without error.
    overlay_before = update_overlay()
    report = []
    try:
        pending = plan(report)
    except BaseException:
        OVERLAY.write_text(overlay_before)
        raise
    for path, content in pending.items():
        if isinstance(content, bytes):
            path.write_bytes(content)
        else:
            path.write_text(content)
    print('\n'.join(report))


def plan(report):
    """The new content of every file the dispatch changes, by path."""
    pending = {}
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
                pending[ref / 'dispatch' / n] = (Path(tmp) / n).read_bytes()
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
                others = [texts[r] for r in sorted(units) if r != rtcd_rel]
                texts[rtcd_rel] = edit_rtcd_unit(texts[rtcd_rel], name, params, ret,
                                                 header_order(headers[header[0]]), others)
            for rel, unit in units.items():
                if rel == rtcd_rel or name not in mentions[rel]:
                    continue
                csource = (ROOT / unit['source']).read_text()
                texts[rel] = edit_caller(texts[rel], csource, name, params, ret, report, f'{profile}:{rel}')
        for rel, unit in units.items():
            if texts[rel] != original[rel]:
                pending[ROOT / rel] = texts[rel]
                unit['rust_sha256'] = hashlib.sha256(texts[rel].encode()).hexdigest()
                if 'production_rust_sha256' in unit:
                    production = texts[rel].split('\n#[cfg(feature = "port-test-bridges")]', 1)[0]
                    unit['production_rust_sha256'] = hashlib.sha256(production.encode()).hexdigest()
        pending[manifest_path] = json.dumps(manifest, indent=2) + '\n'
    return pending


if __name__ == '__main__':
    main()
