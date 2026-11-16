#!/usr/bin/env python3
"""Rewrite the frozen translations so that they build on stable Rust.

Usage: python3 -I tools/stable_rust.py            apply every rewrite, re-record digests
       python3 -I tools/stable_rust.py --check    fail if the tree differs from the output

The translations of libavif (`src`), dav1d (`crates/dav1d`) and libaom
(`crates/aom`) were generated for a pinned nightly. Two of the nightly
features they used have no stable form in the translated code, and this tool
replaces them, in every translation profile listed in a translation manifest,
by the same rewrite each time:

* `core_intrinsics`: C2Rust lowers each C11 `atomic_*` call to
  `::core::intrinsics::atomic_<op>::<_, {AtomicOrdering::<O>}...>(...)`. The
  rewrite calls `crate::platform::atomic_<op>_<o>(...)` instead, the owned
  dav1d helper that performs the same operation through
  `core::sync::atomic::Atomic*::from_ptr` with the ordering the intrinsic named
  (`atomic_cxchg` keeps its success and failure orderings). No ordering
  changes.
* `extern_types`: C2Rust declares each incomplete C struct as `pub type X;`
  inside an `extern` block. The rewrite removes the declaration and defines
  `X` after that block as an opaque `#[repr(C)]` tuple struct of size zero
  (private fields) that cannot be constructed outside its module and is neither `Send`, `Sync` nor
  `Unpin` (the `PhantomData<(*mut u8, PhantomPinned)>` pattern). The compiler
  only accepted the extern types behind pointers, so every use stays a
  pointer to `X`, and a pointer to a zero-sized struct is thin, like a pointer
  to an extern type.

`c_variadic`, the third feature, is stable since Rust 1.99 and needs no
rewrite (port ADR 0060).

The same run also applies the rewrites of engine ADR-0044 (no assembly in the
project), which every regeneration must reproduce as well:

* `xgetbv`: libaom's `aom_ports/x86.h` and libwebp's `src/dsp/cpu.c` read XCR0
  with the raw opcode bytes of XGETBV, which C2Rust lowers to an inline
  assembly statement. The rewrite reads it through
  `core::arch::x86_64::_xgetbv(0)` (the originals' own MSVC branch calls
  `_xgetbv(0)`): the same instruction with the same ECX, returning EDX:EAX as
  one value as the original function does, in a `#[target_feature(enable =
  "xsave")]` wrapper. The callers are unchanged: the original calls the
  function only inside its branch that tested CPUID.1:ECX bit 27 (OSXSAVE).
* empty barrier: dav1d's `get_dc_sign_ctx` (`src/recon_tmpl.c`) has, for
  x86-64 GNU compilers only, `__asm__("" : "+r"(mask), "+r"(mul))`, a hint
  that keeps the compiler from folding two 64-bit constants. It emits no
  instruction, and its read-write operands leave both values as they were,
  so deleting the statement changes no value. C2Rust's form, and the form
  with an assembler comment as its template that the port carried before
  2026-11-16, are deleted (`empty_register_barrier` of
  `tools/transpile_dav1d.py`, which `tools/import_dav1d_linux.py` also
  calls, is this rewrite).

Rewrites that cannot be expressed as one of these patterns are listed in
`tools/stable_rust_edits.json` and applied after them, so a rerun reproduces
them as well. The tool is idempotent: on its own output it changes nothing.

After rewriting, the tool records the new digests of every changed unit in
every manifest that lists it, with the digest definition of
`tools/check_translation.py`. For the libaom manifests it runs the port's own
`tools/enable_aom_dispatch.py --rehash` (ADR 0059, owned code) for each
platform. Rebasing onto a base that changed translated units therefore means:
take the base's version of the units and manifests, then rerun this tool.
"""
import hashlib
import json
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
EDITS = ROOT / 'tools/stable_rust_edits.json'
PRODUCTION_SPLIT = '\n#[cfg(feature = "port-test-bridges")]'

ORDER = r'\{core::intrinsics::AtomicOrdering::(SeqCst|Relaxed|Acquire|Release|AcqRel)\}'
ATOMIC = re.compile(r'::core::intrinsics::atomic_(load|store|or|xadd|xsub|xchg|cxchg)'
                    r'::<_, (?:_, )?' + ORDER + r'(?:, ' + ORDER + r'|, false)?>\(')
EXTERN_OPEN = re.compile(r'^(?:unsafe )?extern "C(?:-unwind)?" \{$')
EXTERN_TYPE = re.compile(r'^    pub type (\w+);$')
# A tuple struct: the ABI layout checks (tools/check_*abi.py, dav1d_layout.py)
# collect `pub struct X {` records as C layouts to compare, and an opaque type
# has no C layout.
OPAQUE = '''#[repr(C)]
pub struct {name}([u8; 0], core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>);'''


def manifests():
    return [ROOT / 'TRANSLATION.json', *sorted((ROOT / 'translations').glob('*.json')),
            *sorted((ROOT / 'crates').glob('*/TRANSLATION*.json'))]


def rewrite_atomics(text, counts):
    def repl(m):
        op, order, failure = m.group(1), m.group(2).lower(), m.group(3)
        if op == 'cxchg':
            if failure is None:
                raise SystemExit(f'atomic_cxchg without a failure ordering: {m.group(0)}')
            name = f'atomic_cxchg_{order}_{failure.lower()}'
        else:
            if failure is not None:
                raise SystemExit(f'unexpected second ordering: {m.group(0)}')
            name = f'atomic_{op}_{order}'
        counts['atomics'] = counts.get('atomics', 0) + 1
        return f'crate::platform::{name}('
    return ATOMIC.sub(repl, text)


def rewrite_extern_types(text, counts):
    lines = text.split('\n')
    out, pending, inside = [], [], False
    for line in lines:
        if not inside and EXTERN_OPEN.match(line):
            inside = True
            out.append(line)
            continue
        if inside:
            m = EXTERN_TYPE.match(line)
            if m:
                pending.append(m.group(1))
                continue
            if line == '}':
                inside = False
                out.append(line)
                for name in pending:
                    out.extend(OPAQUE.format(name=name).split('\n'))
                counts['extern-types'] = counts.get('extern-types', 0) + len(pending)
                pending = []
                continue
        elif EXTERN_TYPE.match(line):
            raise SystemExit(f'extern type outside an extern block: {line}')
        out.append(line)
    if inside or pending:
        raise SystemExit('unterminated extern block')
    return '\n'.join(out)


# The translated `xgetbv()` of libaom (`extern "C-unwind"`) and SharpYUV
# (`extern "C"`), exactly as C2Rust emits it.
XGETBV = re.compile(
    r'(unsafe extern "C(?:-unwind)?" fn xgetbv\(\) -> uint64_t \{\n'
    r'    let ecx: uint32_t = 0 as uint32_t;\n)'
    r'    let mut eax: uint32_t = 0;\n'
    r'    let mut edx: uint32_t = 0;\n'
    r'    asm!\(\n'
    r'        "\.byte 0x0f, 0x01, 0xd0\\n", "\\n", lateout\("ax"\) eax, lateout\("dx"\) edx,\n'
    r'        inlateout\("cx"\) ecx => _, options\(preserves_flags, att_syntax\)\n'
    r'    \);\n'
    r'    return \(edx as uint64_t\) << 32 as core::ffi::c_int \| eax as uint64_t;\n'
    r'\}\n')
XGETBV_BODY = """    // XGETBV (0F 01 D0) through its intrinsic instead of the original's raw
    // opcode bytes (engine ADR-0044): the same instruction with ECX = 0,
    // returning EDX:EAX as one value as the original does.
    #[target_feature(enable = "xsave")]
    unsafe fn read_xcr(xcr: uint32_t) -> uint64_t {
        core::arch::x86_64::_xgetbv(xcr)
    }
    // SAFETY: XGETBV needs CR4.OSXSAVE, which CPUID.1:ECX bit 27 (OSXSAVE)
    // reports and the OS can set only on a CPU with XSAVE, the feature
    // `read_xcr` enables. The original calls xgetbv() only inside its branch
    // that requires CPUID.1:ECX bits 27 and 28; the callers keep that branch.
    return read_xcr(ecx);
}
"""
ASM_IMPORT = 'use core::arch::asm;\n'
# dav1d's empty x86-64 register barrier: C2Rust's template ("\n") and the
# assembler comment the port used as its template before 2026-11-16.
REGISTER_BARRIER = re.compile(
    r'(?m)^([ \t]*)asm!\(\s*"(?:\\n|/\* \{0\} \{1\} \*/)",\s*inlateout\(reg\) mask,\s*'
    r'inlateout\(reg\) mul,\s*options\([^)]*\)\s*\);\n')
REGISTER_BARRIER_NOTE = (r"\1// dav1d's empty x86-64 register barrier on `mask` and `mul` emits no" '\n'
                         r'\1// instruction and is not translated (engine ADR-0044).' '\n')


def rewrite_xgetbv(text, counts):
    text, n = XGETBV.subn(lambda m: m.group(1) + XGETBV_BODY, text)
    if n:
        counts['xgetbv-intrinsic'] = counts.get('xgetbv-intrinsic', 0) + n
    return text


def remove_register_barrier(text, counts=None):
    """Delete dav1d's empty register barrier; return (text, count)."""
    text, n = REGISTER_BARRIER.subn(REGISTER_BARRIER_NOTE, text)
    if n and counts is not None:
        counts['register-barrier'] = counts.get('register-barrier', 0) + n
    return text, n


def drop_unused_asm_import(text, counts):
    if text.count(ASM_IMPORT) == 1 and not re.search(r'\basm!', text):
        text = text.replace(ASM_IMPORT, '')
        counts['asm-import'] = counts.get('asm-import', 0) + 1
    return text


def apply_edits(rel, text, edits, counts):
    for edit in edits.get(rel, []):
        old, new = edit['old'], edit['new']
        if text.count(old) == 1:
            text = text.replace(old, new)
            counts['hand-edits'] = counts.get('hand-edits', 0) + 1
        elif old not in text and new in text:
            pass  # already applied
        else:
            raise SystemExit(f'hand edit does not apply exactly once: {rel}: {edit.get("why")}')
    return text


def rewrite(rel, text, edits, counts):
    text = rewrite_atomics(text, counts)
    text = rewrite_extern_types(text, counts)
    text = rewrite_xgetbv(text, counts)
    text, _ = remove_register_barrier(text, counts)
    text = drop_unused_asm_import(text, counts)
    return apply_edits(rel, text, edits, counts)


def stable_text(rel, text):
    """The stable rewrite of one generated unit (ROOT-relative path `rel`).

    The generators (transpile_aom.py, transpile_dav1d.py, transpile_root.py,
    import_translation.py) call this on each unit's final text, before they
    compare it with the committed unit or record its digest, so that their
    `--check` modes compare like with like.
    """
    edits = {}
    for edit in json.loads(EDITS.read_text())['edits']:
        if edit['file'] == rel:
            edits.setdefault(rel, []).append(edit)
    return rewrite(rel, text, edits, {})


def digests(text):
    production = text.split(PRODUCTION_SPLIT, 1)[0]
    return (hashlib.sha256(text.encode()).hexdigest(),
            hashlib.sha256(production.encode()).hexdigest())


def profile_of(rel):
    parts = rel.split('/')
    if parts[0] == 'src':
        if parts[1] == 'profiles':
            return '/'.join(['root'] + parts[2:5])
        if parts[1] == 'variants':
            return 'root/macos/variants/' + parts[2]
        return 'root/macos'
    crate, rest = parts[1], parts[3:]
    if crate == 'sharpyuv':
        return 'sharpyuv'
    if crate == 'aom':
        if len(rest) == 1:  # crates/aom/src/macros-<platform>-<profile>.rs
            return 'aom/' + '/'.join(rest[0][:-3].split('-')[-2:])
        return f'aom/{rest[0]}/{rest[1]}'
    platform = 'linux' if rest[0] == 'linux' else 'macos'
    release = 'release' in rest[:2]
    return f'dav1d/{platform}/{"release" if release else "debug"}'


def main():
    check = sys.argv[1:] == ['--check']
    if sys.argv[1:] not in ([], ['--check']):
        raise SystemExit(__doc__)
    raw_edits = json.loads(EDITS.read_text())['edits']
    edits = {}
    for edit in raw_edits:
        edits.setdefault(edit['file'], []).append(edit)
    records = {path: json.loads(path.read_text()) for path in manifests()}
    files = sorted({unit['rust'] for record in records.values() for unit in record.get('units', [])})
    unknown = set(edits) - set(files)
    if unknown:
        raise SystemExit(f'hand edits name files outside the manifests: {sorted(unknown)}')
    changed, stale, totals, outputs = [], [], {}, {}
    for rel in files:
        path = ROOT / rel
        text = path.read_text()
        counts = {}
        new = rewrite(rel, text, edits, counts)
        for key, value in counts.items():
            per = totals.setdefault(key, {})
            per[profile_of(rel)] = per.get(profile_of(rel), 0) + value
        if new != text:
            changed.append(rel)
            outputs[path] = new
    if not check:  # write only after every file was rewritten without error
        for path, new in outputs.items():
            path.write_text(new)
    for path, record in records.items():
        for unit in record.get('units', []):
            text = (ROOT / unit['rust']).read_text()
            full, production = digests(text)
            if unit['rust_sha256'] != full or (
                    'production_rust_sha256' in unit and unit['production_rust_sha256'] != production):
                stale.append((path, unit))
    for key in sorted(totals):
        for profile in sorted(totals[key]):
            print(f'{key}\t{profile}\t{totals[key][profile]}')
    if check:
        if changed or stale:
            for rel in changed:
                print(f'not rewritten: {rel}')
            for path, unit in stale:
                print(f'stale digest: {path.relative_to(ROOT)}: {unit["rust"]}')
            raise SystemExit('the tree differs from the output of tools/stable_rust.py')
        print(f'stable rewrites up to date in {len(files)} translated files')
        return
    # Re-record digests: the libaom manifests through the port's own rehash
    # (ADR 0059), the others with the same digest definition.
    aom_stale = {path.stem.split('-')[1] for path, _ in stale if path.parent.name == 'aom'}
    for platform in sorted(aom_stale):
        args = ['--linux', '--rehash'] if platform == 'linux' else ['--rehash']
        subprocess.run([sys.executable, '-I', str(ROOT / 'tools/enable_aom_dispatch.py'), *args],
                       check=True, cwd=ROOT, stdout=subprocess.DEVNULL)
    for path, record in records.items():
        if path.parent.name == 'aom':
            continue
        dirty = False
        for unit in record.get('units', []):
            full, production = digests((ROOT / unit['rust']).read_text())
            if unit['rust_sha256'] != full:
                unit['rust_sha256'] = full
                dirty = True
            if 'production_rust_sha256' in unit and unit['production_rust_sha256'] != production:
                unit['production_rust_sha256'] = production
                dirty = True
        if dirty:
            path.write_text(json.dumps(record, indent=2) + '\n')
    print(f'rewrote {len(changed)} of {len(files)} translated files')


if __name__ == '__main__':
    main()
