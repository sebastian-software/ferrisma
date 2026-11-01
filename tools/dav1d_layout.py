"""Retain original C record/field alignment omitted by C2Rust 0.21.0."""
import json
import re
import subprocess

from check_abi import compiler_flags, walk


def alignment(node):
    values = [int(value['value']) for attribute in node.get('inner', [])
              if attribute.get('kind') == 'AlignedAttr'
              for value in walk(attribute) if value.get('kind') == 'ConstantExpr' and 'value' in value]
    return max(values, default=0)


def retain_alignment(text, entry):
    flags = compiler_flags(entry)
    # C2Rust needs LLVM 16's resource headers; the original Clang layout query
    # uses its own matching built-in headers, as in the reference build.
    if flags[-2] == '-isystem' and '/lib/clang/16/include' in flags[-1]:
        flags = flags[:-2]
    ast = json.loads(subprocess.check_output(flags +
        ['-Xclang', '-ast-dump=json', '-fsyntax-only', entry['file']],
        cwd=entry['directory'], stderr=subprocess.PIPE))
    nodes = list(walk(ast))
    records = [node for node in nodes if node.get('kind') == 'RecordDecl' and node.get('completeDefinition')]
    named = {node['name']: node for node in records if node.get('name')}
    alias_alignment = {node['name']: alignment(node) for node in nodes if node.get('kind') == 'TypedefDecl' and node.get('name')}
    by_id = {node['id']: node for node in records}
    for node in nodes:
        if node.get('kind') == 'TypedefDecl':
            for child in walk(node):
                identifier = child.get('decl', {}).get('id') or child.get('ownedTagDecl', {}).get('id')
                if identifier in by_id:
                    named[node['name']] = by_id[identifier]
                    break
    definitions = {name: body for name, body in re.findall(r'pub (?:struct|union) (\w+) \{(.*?)\n\}', text, re.S)}
    # Resolve anonymous children through the original containing record. Field
    # names alone cannot distinguish the decoder's nested anonymous unions.
    for _ in range(len(definitions)):
        previous = len(named)
        for name, body in definitions.items():
            if name not in named:
                continue
            fields, pending = [], None
            for child in named[name].get('inner', []):
                if child.get('kind') == 'RecordDecl' and child.get('completeDefinition'):
                    pending = child
                elif child.get('kind') == 'FieldDecl':
                    fields.append((child, pending))
                    pending = None
            rust_fields = re.findall(r'^\s*pub (\w+): ([^\n]+)', body, re.M)
            if len(fields) == len(rust_fields):
                for (_, child), (_, field_type) in zip(fields, rust_fields):
                    anonymous = re.fullmatch(r'(C2RustUnnamed\w*),', field_type)
                    if child is not None and anonymous:
                        named[anonymous[1]] = child
        if len(named) == previous:
            break
    changes, initializers = 0, {}

    def normalized(name):
        if name.startswith('c2rust_unnamed'):
            return None
        return name.removesuffix('_0')

    def record(match):
        nonlocal changes
        prefix, kind, name, body = match.groups()
        rust_fields = re.findall(r'^\s*pub (\w+):', body, re.M)
        node = named.get(name)
        if node is None and name.startswith('C2RustUnnamed'):
            candidates = [candidate for candidate in records
                          if [field.get('name') for field in candidate.get('inner', []) if field.get('kind') == 'FieldDecl']
                          == [normalized(field) for field in rust_fields]]
            signatures = {(alignment(candidate), tuple(alignment(field) for field in candidate.get('inner', [])
                                                       if field.get('kind') == 'FieldDecl')) for candidate in candidates}
            if len(signatures) > 1:
                raise SystemExit(f'Ambiguous anonymous alignment: {name}: {rust_fields}')
            node = candidates[0] if candidates else None
        if node is None:
            return match.group()
        fields = [field for field in node.get('inner', []) if field.get('kind') == 'FieldDecl']
        if len(fields) != len(rust_fields):
            if any(alignment(field) for field in fields):
                raise SystemExit(f'Cannot map aligned fields in {name}')
            return match.group()
        markers = []
        for field, rust_field in zip(fields, rust_fields):
            value = alignment(field)
            if value:
                marker = f'__dav1d_align_{rust_field}'
                body = re.sub(r'(?m)^(\s*)pub ' + rust_field + ':',
                    rf'\1pub {marker}: [crate::platform::Align{value}; 0],\n\1pub {rust_field}:', body)
                markers.append(marker)
                changes += 1
        value = max(alignment(node), alias_alignment.get(name, 0))
        if value:
            if '#[repr(C, packed)]' in prefix:
                if name not in ['refmvs_refpair', 'refmvs_block']:
                    raise SystemExit(f'Unmapped packed and aligned C typedef: {name}')
                # The 12-byte block contains an eight-byte-aligned union: cap
                # field alignment at four instead of adding incompatible align.
                # The two-byte ref-pair union already has natural alignment two.
                replacement = '#[repr(C, packed(4))]' if name == 'refmvs_block' else '#[repr(C)]'
                prefix = prefix.replace('#[repr(C, packed)]', replacement)
                if name == 'refmvs_refpair':
                    value = 0
            if value:
                prefix = prefix.replace('#[repr(C)]', f'#[repr(C, align({value}))]')
            changes += 1
        if markers and kind == 'struct':
            initializers[name] = markers
        return prefix + f'pub {kind} {name} {{' + body + '\n}'

    text = re.sub(r'((?:#\[[^\n]+\]\n)+)pub (struct|union) (\w+) \{(.*?)\n\}', record, text, flags=re.S)
    for name, markers in initializers.items():
        pattern = r'\b' + re.escape(name) + r' \{'
        def initializer(match):
            if re.search(r'\bstruct\s*$', text[max(0, match.start() - 20):match.start()]):
                return match.group()
            return match.group() + ' ' + ' '.join(f'{marker}: [],' for marker in markers)
        text = re.sub(pattern, initializer, text)
    return text, changes
