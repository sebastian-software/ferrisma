#!/usr/bin/env python3
"""Compare named public and internal dav1d record layouts with original Clang."""
import json
from pathlib import Path
import re
import subprocess

from check_abi import compiler_flags, walk

ROOT = Path(__file__).resolve().parents[1]


def main():
    build = ROOT / 'build/dav1d-reference-debug'
    entries = [e for e in json.loads((build / 'compile_commands.json').read_text())
               if e['output'].startswith('src/libdav1d')]
    work = ROOT / 'build/dav1d-abi'
    work.mkdir(exist_ok=True)
    expected, rust, excluded = [], [], []
    record_count = 0
    for index, entry in enumerate(entries):
        rust.append(f'fn probe_{index}() {{')
        group = 'b8' if 'bitdepth_8.' in entry['output'] else 'b16' if 'bitdepth_16.' in entry['output'] else 'common'
        source = (Path(entry['directory']) / entry['file']).resolve()
        module = 'dav1d::' + group + '::r#' + ('decoder' if source.stem == 'lib' else source.stem)
        translated = (ROOT / 'crates/dav1d/src' / group / source.with_suffix('.rs').name).read_text()
        records = re.findall(r'pub (struct|union) (\w+) \{(.*?)\n\}', translated, re.S)
        flags = compiler_flags(entry)
        ast = json.loads(subprocess.check_output(flags + ['-Xclang', '-ast-dump=json', '-fsyntax-only', source],
                                                 cwd=entry['directory'], stderr=subprocess.PIPE))
        nodes = list(walk(ast))
        tags = {n['name']: n for n in nodes if n.get('kind') == 'RecordDecl' and n.get('completeDefinition') and n.get('name')}
        aliases = {n['name']: n for n in nodes if n.get('kind') == 'TypedefDecl' and n.get('name')}
        by_id = {n['id']: n for n in nodes if 'id' in n and n.get('completeDefinition')}
        c_values = []
        for kind, name, body in records:
            if name.startswith('C2RustUnnamed'):
                excluded.append(f'{module}::{name}: anonymous record')
                continue
            record, c_type = tags.get(name), f'{kind} {name}'
            alias = aliases.get(name)
            if alias and alias.get('type', {}).get('qualType', '').startswith(('struct ', 'union ')):
                c_type = name
                if record is None:
                    for node in walk(alias):
                        reference = node.get('decl', {}).get('id') or node.get('ownedTagDecl', {}).get('id')
                        if reference in by_id:
                            record = by_id[reference]
                            break
            if record is None:
                raise SystemExit(f'Cannot resolve C record: {module}::{name}')
            pairs = [(f'sizeof({c_type})', f'core::mem::size_of::<{module}::{name}>()', 'size'),
                     (f'_Alignof({c_type})', f'core::mem::align_of::<{module}::{name}>()', 'alignment')]
            fields = [n for n in record.get('inner', []) if n.get('kind') == 'FieldDecl']
            rust_fields = [name for name in re.findall(r'^\s*pub (\w+):', body, re.M)
                           if not name.startswith('__dav1d_align_')]
            if len(fields) == len(rust_fields) and all(n.get('name') and not n.get('isBitfield') for n in fields):
                pairs += [(f'__builtin_offsetof({c_type}, {field["name"]})',
                           f'core::mem::offset_of!({module}::{name}, {rust_field})', field['name'])
                          for field, rust_field in zip(fields, rust_fields)]
            else:
                excluded.append(f'{module}::{name}: field offsets only; anonymous fields or bitfields')
            for c_expression, rust_expression, label in pairs:
                c_values.append(c_expression)
                rust.append(f'println!("{{}}", {rust_expression});')
                expected.append((f'{module}::{name}.{label}', None))
            record_count += 1
        if c_values:
            probe, ir = work / f'probe-{index}.c', work / f'probe-{index}.ll'
            probe.write_text(f'#include "{source}"\nconst unsigned long long dav1d_abi_values[] = {{\n' + ',\n'.join(c_values) + '\n};\n')
            subprocess.run(flags + ['-S', '-emit-llvm', '-o', ir, probe], cwd=entry['directory'], check=True, stderr=subprocess.PIPE)
            line = next(line for line in ir.read_text().splitlines() if line.startswith('@dav1d_abi_values ='))
            values = [int(value) for value in re.findall(r'i64 (\d+)', line)]
            if len(values) != len(c_values):
                raise SystemExit(f'Unexpected Clang ABI values: {source}')
            for offset, value in enumerate(values, len(expected) - len(values)):
                expected[offset] = (expected[offset][0], value)
        rust.append('}')
    rust.append('fn main() {' + ''.join(f'probe_{index}();' for index in range(len(entries))) + '}')
    (work / 'probe.rs').write_text('\n'.join(rust) + '\n')
    (work / 'Cargo.toml').write_text('[package]\nname="dav1d-abi-probe"\nversion="0.0.0"\nedition="2021"\n[workspace]\n[[bin]]\nname="probe"\npath="probe.rs"\n[dependencies]\n'
        + f'dav1d={{package="libavif-dav1d",path={json.dumps(str(ROOT / "crates/dav1d"))}}}\n')
    actual = [int(line) for line in subprocess.check_output(['cargo', 'run', '--offline', '--quiet', '--manifest-path', work / 'Cargo.toml',
                                                           '--target-dir', ROOT / 'target/dav1d-abi'], text=True).splitlines()]
    if len(actual) != len(expected):
        raise SystemExit('Different ABI check inventories')
    differences = [f'{name}: C={value}, Rust={other}' for (name, value), other in zip(expected, actual) if value != other]
    report = {'record_instances': record_count, 'checks': len(expected), 'differences': differences, 'excluded': excluded}
    (work / 'summary.json').write_text(json.dumps(report, indent=2) + '\n')
    print(f'{record_count} records, {len(expected)} layout checks, {len(differences)} differences')
    if differences:
        raise SystemExit('\n'.join(differences[:100]))


if __name__ == '__main__':
    main()
