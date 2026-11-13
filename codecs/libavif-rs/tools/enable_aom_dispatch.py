#!/usr/bin/env python3
"""Enable Neon RTCD dispatch for named functions in the frozen macOS translation.

Usage: python3 tools/enable_aom_dispatch.py NAME...

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
NAMES = sys.argv[1:]
sys.path.insert(0, str(ROOT / 'tools'))
import transpile_aom  # noqa: E402

CRATE = ROOT / 'crates/aom'
OVERLAY = CRATE / 'rtcd/arm64.pl'
RTCD_UNITS = {'config/aom_dsp_rtcd.h': 'aom_dsp/aom_dsp_rtcd.rs',
              'config/av1_rtcd.h': 'av1/common/av1_rtcd.rs',
              'config/aom_scale_rtcd.h': 'aom_scale/aom_scale_rtcd.rs'}
EXPECT = '.expect("non-null function pointer")'


def update_overlay():
    text = OVERLAY.read_text()
    m = re.search(r'(my %ported_neon = map \{ \$_ => 1 \} qw/\n)(.*?)(/;\n)', text, re.S)
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
    text, n_ptr = ptr.subn(name, text)
    call = re.compile(r'(?<!fn )\b' + name + r'_c\(')
    text, n_call = call.subn(name + EXPECT + '(', text)
    if not (n_ptr or n_call):
        return text
    report.append(f'{rel}: {name} calls={n_call} pointers={n_ptr}')
    decl = re.compile(r'\n(\s*)fn ' + name + r'_c\((.*?)\)(\s*->\s*[^;{]+?)?;', re.S)
    static = f'static mut {name}: {opt};'
    remaining = len(re.findall(r'\b' + name + r'_c\b', text))
    m = decl.search(text)
    if m and remaining == 1:
        text = text[:m.start()] + '\n' + m.group(1) + static + text[m.end():]
    else:
        m2 = re.search(r'extern "C-unwind" \{\n', text)
        if not m2:
            raise SystemExit(f'No extern block in {rel}')
        text = text[:m2.end()] + '    ' + static + '\n' + text[m2.end():]
    return text


def main():
    update_overlay()
    report = []
    for profile in ['release', 'debug']:
        ref = CRATE / f'reference/macos/{profile}'
        manifest_path = CRATE / f'TRANSLATION-macos-{profile}.json'
        manifest = json.loads(manifest_path.read_text())
        with tempfile.TemporaryDirectory() as tmp:
            dispatch = transpile_aom.dispatch_headers('macos', ref, Path(tmp))
            headers = {n: (Path(tmp) / n).read_text() for n in dispatch['headers']}
            for n in dispatch['headers']:
                shutil.copy(Path(tmp) / n, ref / 'dispatch' / n)
        manifest['rtcd_dispatch'] = dispatch
        units = {u['rust']: u for u in manifest['units']}
        texts = {r: (ROOT / r).read_text() for r in units}
        original = dict(texts)
        for name in NAMES:
            header = [h for h, t in headers.items() if re.search(r'\(\*' + name + r'\)\(', t)]
            if len(header) != 1:
                raise SystemExit(f'{name}: not dispatched by the regenerated headers ({header})')
            rtcd_rel = f'crates/aom/src/macos/{profile}/' + RTCD_UNITS[header[0]]
            params, ret = find_signature(texts, name)
            texts[rtcd_rel] = edit_rtcd_unit(texts[rtcd_rel], name, params, ret, header_order(headers[header[0]]))
            for rel, unit in units.items():
                if rel == rtcd_rel or not re.search(r'\b' + name + r'_c\b', texts[rel]):
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
