"""Restore explicit C object alignment without changing values or operations."""
import re


# Generated C2Rust strings and comments must not be treated as Rust identifiers
# or delimiters. The translator emits ordinary quoted strings, not raw strings.
LEXEME = re.compile(r'//[^\n]*|/\*[\s\S]*?\*/|b?"(?:\\.|[^"\\])*"|[A-Za-z_]\w*|::|[^\s]')


def tokens(text):
    return [match for match in LEXEME.finditer(text)
            if not match[0].startswith(('//', '/*', '"', 'b"'))]


def uses(text, names):
    lexemes = tokens(text)
    replacements = []
    for index, token in enumerate(lexemes):
        if token[0] not in names:
            continue
        previous = lexemes[index - 1][0] if index else ''
        following = lexemes[index + 1][0] if index + 1 < len(lexemes) else ''
        # Preserve field names, paths, declarations, and explicit field labels.
        if previous in ['.', '::', "'"] or following == ':':
            continue
        replacements.append((token.start(), token.end(), token[0] + '.0'))
    for start, end, replacement in reversed(replacements):
        text = text[:start] + replacement + text[end:]
    return text


def wrap(text, name, value, storage):
    pattern = rf'\b{storage}\s+(?:mut\s+)?{re.escape(name)}:\s*'
    declarations = list(re.finditer(pattern, text))
    if len(declarations) != 1:
        raise SystemExit(f'Expected one aligned {storage} declaration: {name}')
    declaration = declarations[0]
    depth, equal, end = 0, None, None
    for token in tokens(text[declaration.end():]):
        offset = declaration.end() + token.start()
        if token[0] == '=' and not depth and equal is None:
            equal = offset
        elif token[0] == ';' and not depth:
            end = offset
            break
        elif token[0] in ['(', '[', '{']:
            depth += 1
        elif token[0] in [')', ']', '}']:
            depth -= 1
    if equal is None or end is None:
        raise SystemExit(f'Cannot delimit aligned initializer: {name}')
    original_type = text[declaration.end():equal].strip()
    initializer = text[equal + 1:end].strip()
    wrapper = f'crate::platform::Aligned{value}'
    return (text[:declaration.end()] + f'{wrapper}<{original_type}> = '
            + f'{wrapper}({initializer})' + text[end:])


def function_span(text, name):
    matches = list(re.finditer(r'\bfn\s+' + re.escape(name) + r'\s*\(', text))
    if len(matches) != 1:
        raise SystemExit(f'Cannot resolve aligned-variable function: {name}')
    start = matches[0].start()
    depth, opened = 0, False
    for token in tokens(text[start:]):
        if token[0] == '{':
            depth += 1
            opened = True
        elif token[0] == '}':
            depth -= 1
            if opened and not depth:
                return start, start + token.end()
    raise SystemExit(f'Cannot delimit generated function: {name}')


def retain_storage_alignment(text, ast, alignment, allow_pruned=False):
    groups = {}

    def visit(node, owner=None):
        if node.get('kind') == 'FunctionDecl':
            owner = node.get('name')
        if node.get('kind') == 'VarDecl' and alignment(node):
            value = alignment(node)
            if value not in [2, 4, 8, 16, 32, 64, 128, 256]:
                raise SystemExit(f'Unsupported explicit C object alignment: {value}')
            storage = 'static' if owner is None or node.get('storageClass') == 'static' else 'let'
            c_type = node.get('type', {}).get('desugaredQualType', node.get('type', {}).get('qualType', ''))
            array = '[' in c_type and '(*' not in c_type
            groups.setdefault(owner, []).append((node['name'], value, storage, array))
        for child in node.get('inner', []):
            visit(child, owner)

    visit(ast)
    count = 0
    for owner, variables in groups.items():
        if allow_pruned and owner and not re.search(r'\bfn\s+' + re.escape(owner) + r'\s*\(', text):
            continue
        start, end = function_span(text, owner) if owner else (0, len(text))
        body = text[start:end]
        resolved = {}
        for name, value, storage, array in variables:
            bindings = re.findall(rf'\b{storage}\s+(?:mut\s+)?({re.escape(name)}(?:_\d+)?):\s*(\[?)', body)
            candidates = [candidate for candidate, bracket in bindings if not allow_pruned or bool(bracket) == array]
            if allow_pruned and not candidates and not re.search(r'\b' + re.escape(name) + r'\b', body):
                continue
            if allow_pruned:
                original_values = [other_value for other_name, other_value, other_storage, other_array in variables
                                   if other_name == name and other_storage == storage and other_array == array]
                if len(candidates) == len(original_values) and set(original_values) == {value}:
                    # Original independent C scopes may reuse a local name;
                    # C2Rust suffixes it when it lifts those declarations.
                    # Mapping every candidate is unambiguous only when the
                    # number and explicit alignment agree with all originals.
                    for candidate in candidates:
                        if candidate in resolved and resolved[candidate] != (value, storage):
                            raise SystemExit(f'Conflicting aligned C binding: {owner}/{candidate}')
                        resolved[candidate] = (value, storage)
                    continue
            if len(candidates) != 1 or candidates[0] in resolved:
                raise SystemExit(f'Ambiguous aligned C object: {owner}/{name}: {candidates}')
            resolved[candidates[0]] = (value, storage)
        for name, (value, storage) in resolved.items():
            body = wrap(body, name, value, storage)
            count += 1
        body = uses(body, resolved)
        text = text[:start] + body + text[end:]
    return text, count
