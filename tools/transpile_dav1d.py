#!/usr/bin/env python3
"""Reproduce both original dav1d bit-depth instantiations with C2Rust."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import sys

from import_translation import translate
from dav1d_layout import retain_alignment

ROOT = Path(__file__).resolve().parents[1]
CRATE = ROOT / 'crates/dav1d'


def current_variadic(text):
    count = text.count('::core::ffi::VaListImpl')
    text = text.replace('::core::ffi::VaListImpl', '::core::ffi::VaList')
    text, converted = re.subn(r'\b(\w+)\.as_va_list\(\)(?: as ::core::ffi::VaList)?', r'\1', text)
    return text, count + converted


def empty_register_barrier(text):
    # The original x86 GNU asm has two read/write registers and no instructions.
    # Rust requires template references; an assembler comment emits no code.
    return re.subn(r'(asm!\(\n\s*)"\\n", inlateout\(reg\) mask, inlateout\(reg\) mul,',
                   r'\1"/* {0} {1} */", inlateout(reg) mask, inlateout(reg) mul,', text)


def corrected(source, raw, entry):
    if sys.platform == 'linux' and re.search(r'\bf128\b', raw):
        raise SystemExit('Linux long-double translation needs its own mapping')
    text, changes = translate(source, raw, codec_registry=False)
    text, count = re.subn(r'(?<!fn )\b__assert_rtn\(', 'crate::platform::assertion(', text)
    if count:
        changes['target-runtime-assertion'] = count
    text = re.sub(r'    fn __assert_rtn\([\s\S]*?    \) -> !;\n', '', text)
    for name in ['pthread_mutex_t', 'pthread_cond_t', 'pthread_attr_t', 'pthread_t',
                 'pthread_once_t', 'pthread_mutexattr_t', 'pthread_condattr_t']:
        text, count = re.subn(r'pub type ' + name + r' = [^;]+;', f'pub type {name} = libc::{name};', text)
        if count:
            changes['target-libc-thread-storage'] = changes.get('target-libc-thread-storage', 0) + count
    text = text.replace('pub type va_list = __builtin_va_list;', "pub type va_list<'a> = core::ffi::VaList<'a>;")
    if 'core::ffi::VaList' in text:
        changes['variadic-abi'] = 1
    text, count = re.subn(r'    let mut (\w+): va_list = 0 as \*mut core::ffi::c_char;\n    \1 = args.clone\(\);',
                          r'    let \1 = args.clone();', text)
    if count:
        changes['variadic-local'] = count
    text, count = re.subn(r'static mut (\w+): pthread_once_t = \{[\s\S]*?\n    \};',
                          r'static mut \1: pthread_once_t = libc::PTHREAD_ONCE_INIT;', text)
    if count:
        changes['target-libc-once-initializer'] = count
    text, count = re.subn(r'(let mut \w+: pthread_attr_t =) _opaque_pthread_attr_t \{[^}]*\}',
                          r'\1 core::mem::zeroed()', text)
    if count:
        changes['target-libc-attribute-storage'] = count
    # The pinned Rust compiler spells atomic orderings as const arguments.
    # Preserve the original operation, width, strong CAS, and memory ordering.
    def atomic(match):
        operation, order, failure = match.groups()
        orders = {'seqcst': 'SeqCst', 'relaxed': 'Relaxed', 'acquire': 'Acquire'}
        value = '{core::intrinsics::AtomicOrdering::' + orders[order] + '}'
        if operation == 'cxchg':
            parameters = '_, ' + value + ', {core::intrinsics::AtomicOrdering::' + orders[failure] + '}'
        elif operation in ['load', 'store']:
            parameters = '_, ' + value + ', false'
        elif operation in ['xadd', 'xsub', 'or']:
            parameters = '_, _, ' + value
        else:
            parameters = '_, ' + value
        return '::core::intrinsics::atomic_' + operation + '::<' + parameters + '>'
    text, count = re.subn(r'::core::intrinsics::atomic_(cxchg|load|store|or|xadd|xsub|xchg)_(seqcst|relaxed|acquire)(?:_(seqcst|relaxed|acquire))?', atomic, text)
    if count:
        changes['pinned-rust-atomic-intrinsic-spelling'] = count
    # This is an identity pointer cast after the shared static-address rule.
    text, count = re.subn(r'(core::ptr::addr_of!\(\w+\)\.cast::<[^>]+>\(\)) as \*mut _', r'\1', text)
    if count:
        changes['remove-inferred-identity-pointer-cast'] = count
    text, count = re.subn(r'pub type FILE = \w+;', 'pub type FILE = libc::FILE;', text)
    if count:
        changes['target-libc-file-handle'] = count
    if source.stem == 'intra_edge':
        for field, element in [('branch_sb128', 'EdgeBranch'), ('branch_sb64', 'EdgeBranch'),
                               ('tip_sb128', 'EdgeTip'), ('tip_sb64', 'EdgeTip')]:
            text, count = re.subn(r'\(nodes\.' + field + r'\)\s*\.as_mut_ptr\(\)',
                                  f'core::ptr::addr_of_mut!(nodes.{field}).cast::<{element}>()', text)
            if count:
                changes['raw-static-field-array-address'] = changes.get('raw-static-field-array-address', 0) + count
    for name, record_type in re.findall(r'\bstatic mut (\w+): (\w+)', text):
        record = re.search(r'pub (?:struct|union) ' + record_type + r' \{(.*?)\n\}', text, re.S)
        if record:
            for field, array in re.findall(r'^\s*pub (\w+): (\[[^\n]+\]),', record[1], re.M):
                element = array[1:-1].rsplit(';', 1)[0].strip()
                for method, address in [('as_mut_ptr', 'addr_of_mut'), ('as_ptr', 'addr_of')]:
                    text, count = re.subn(r'\(' + name + r'\.' + field + r'\)\s*\.' + method + r'\(\)',
                        f'core::ptr::{address}!({name}.{field}).cast::<{element}>()', text)
                    if count:
                        changes['raw-static-record-array-address'] = changes.get('raw-static-record-array-address', 0) + count
    # C address-taking has no Rust exclusive/shared borrow semantics. This is
    # particularly important for shared atomic fields and one-time static init.
    text, count = re.subn(r'&mut\s+', '&raw mut ', text)
    if count:
        changes['raw-c-mutable-address'] = count
    names = set(re.findall(r'\bstatic mut (\w+):', text))
    for name in sorted(names, key=len, reverse=True):
        text, count = re.subn(r'&' + name + r'\b', '&raw const ' + name, text)
        if count:
            changes['raw-c-static-address'] = changes.get('raw-c-static-address', 0) + count
    text, count = re.subn(r'let ref mut (fresh\d+) = ([\s\S]*?);', r'let \1 = &raw mut \2;', text)
    if count:
        changes['raw-c-assignment-temporary'] = count
    text, count, storage_count = retain_alignment(text, entry)
    if count:
        changes['original-c-record-field-alignment'] = count
    if storage_count:
        changes['original-c-object-alignment'] = storage_count
    text, count = current_variadic(text)
    if count:
        changes['current-rust-variadic-list'] = count
    text, count = empty_register_barrier(text)
    if count:
        changes['empty-readwrite-register-barrier'] = count
    return text, changes


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--check', action='store_true')
    parser.add_argument('--import-only', action='store_true')
    parser.add_argument('--release', action='store_true', help='Retain original NDEBUG preprocessing separately')
    parser.add_argument('--clang-include', type=Path)
    args = parser.parse_args()
    subprocess.run(['python3', ROOT / 'tools/verify_upstream.py'], check=True)
    profile = 'release' if args.release else 'debug'
    linux = sys.platform == 'linux'
    platform_prefix = 'linux/' if linux else ''
    build = ROOT / f'build/dav1d-reference-{profile}'
    entries = [e for e in json.loads((build / 'compile_commands.json').read_text())
               if e['output'].startswith('src/libdav1d')]
    if len(entries) != 51:
        raise SystemExit(f'Unexpected decoder closure: {len(entries)}')
    resource = args.clang_include or (Path('/usr/lib/llvm-16/lib/clang/16/include') if linux else
                                     ROOT / 'build/toolchain/llvm@16/16.0.6_1/lib/clang/16/include')
    environment = os.environ.copy()
    environment['DYLD_LIBRARY_PATH'] = os.environ.get('C2RUST_LIBRARY_PATH', str(resource.parents[2]) + ':/opt/homebrew/lib')
    units, modules = [], []
    for group in ['common', 'b8', 'b16']:
        source_prefix = platform_prefix + ('release/' if args.release else '')
        modules.append((f'#[path = "{source_prefix}{group}"] ' if source_prefix else '') + f'pub mod {group} {{')
        selected = [dict(e) for e in entries if (
            ('bitdepth_' not in e['output']) if group == 'common' else
            f'bitdepth_{group[1:]}.a.p' in e['output'])]
        work = ROOT / 'build/dav1d-translate' / platform_prefix / profile / group
        work.mkdir(parents=True, exist_ok=True)
        for entry in selected:
            entry['command'] += f' -isystem {resource}'
        database = work / 'compile_commands.json'
        database.write_text(json.dumps(selected, indent=2) + '\n')
        raw_root = ROOT / 'build/dav1d-generated' / platform_prefix / ('release' if args.release else '') / group
        raw_root.parent.mkdir(parents=True, exist_ok=True)
        if not args.import_only:
            result = subprocess.run(['c2rust-transpile', database, '--emit-build-files', '--fail-on-error',
                                     '--output-dir', raw_root], env=environment, text=True,
                                    stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
            (work / 'transpile.log').write_text(result.stdout)
            if result.returncode or 'error:' in result.stdout or 'error generated' in result.stdout:
                raise SystemExit(f'C2Rust failed; inspect {work / "transpile.log"}')
        for entry in selected:
            source = ROOT / 'upstream/dav1d/src' / Path(entry['file']).name
            raw = (raw_root / 'src' / source.with_suffix('.rs').name).read_text()
            text, changes = corrected(source, raw, entry)
            target = CRATE / 'src' / source_prefix / group / source.with_suffix('.rs').name
            target.parent.mkdir(parents=True, exist_ok=True)
            if args.check:
                if target.read_text() != text:
                    raise SystemExit(f'Translation differs: {target}')
            else:
                target.write_text(text)
            units.append({'source': str(source.relative_to(ROOT)), 'instantiation': group,
                          'rust': str(target.relative_to(ROOT)), 'corrections': changes,
                          'raw_sha256': hashlib.sha256(raw.encode()).hexdigest(),
                          'rust_sha256': hashlib.sha256(text.encode()).hexdigest()})
            module = 'decoder' if source.stem == 'lib' else source.stem
            modules.append(f'    #[path = "{source.stem}.rs"] pub mod r#{module};')
        modules.append('}')
    suffix = ('-linux' if linux else '') + ('-release' if args.release else '')
    module_output = CRATE / f'src/modules{suffix}.rs'
    module_text = '// Generated by tools/transpile_dav1d.py.\n' + '\n'.join(modules) + '\n'
    if args.check:
        if module_output.read_text() != module_text:
            raise SystemExit('Module inventory differs')
    else:
        module_output.write_text(module_text)
    record = {'translator': 'c2rust 0.21.0', 'llvm': '16.0.6', 'preprocessing_profile': profile,
              'generation_target': 'x86_64-unknown-linux-gnu' if linux else 'aarch64-apple-darwin', 'units': units}
    output = CRATE / f'TRANSLATION{suffix}.json'
    encoded = json.dumps(record, indent=2) + '\n'
    if args.check:
        if output.read_text() != encoded:
            raise SystemExit('Translation manifest differs')
    else:
        output.write_text(encoded)
    print(f'{"Verified" if args.check else "Imported"} {len(units)} decoder translation units')


if __name__ == '__main__':
    main()
