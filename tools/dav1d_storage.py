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


def retain_storage_alignment(text, ast, alignment):
    groups = {}

    def visit(node, owner=None):
        if node.get('kind') == 'FunctionDecl':
            owner = node.get('name')
        if node.get('kind') == 'VarDecl' and alignment(node):
            value = alignment(node)
            if value not in [2, 4, 8, 16, 32, 64]:
                raise SystemExit(f'Unsupported explicit C object alignment: {value}')
            groups.setdefault(owner, []).append((node['name'], value))
        for child in node.get('inner', []):
            visit(child, owner)

    visit(ast)
    count = 0
    for owner, variables in groups.items():
        start, end = function_span(text, owner) if owner else (0, len(text))
        body = text[start:end]
        resolved = {}
        for name, value in variables:
            storage = 'let' if owner else 'static'
            candidates = re.findall(rf'\b{storage}\s+(?:mut\s+)?({re.escape(name)}(?:_\d+)?):', body)
            if len(candidates) != 1 or candidates[0] in resolved:
                raise SystemExit(f'Ambiguous aligned C object: {owner}/{name}: {candidates}')
            resolved[candidates[0]] = (value, storage)
        for name, (value, storage) in resolved.items():
            body = wrap(body, name, value, storage)
            count += 1
        body = uses(body, resolved)
        text = text[:start] + body + text[end:]
    return text, count
