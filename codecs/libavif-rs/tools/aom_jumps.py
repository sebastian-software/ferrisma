"""Map libaom's scoped nonlocal errors to typed Rust unwinding boundaries.

glibc's `setjmp` macro expands to `_setjmp`. The mapping normalizes that
declaration and its direct call sites before mapping the 32 scoped error
targets of each profile; a surviving libc nonlocal-jump call or a wrong target
or transfer count fails the structural validation. Direct C2Rust `setjmp` and
`longjmp` calls are not an accepted Rust implementation: error targets,
diagnostics, return values, cleanup, and worker synchronization keep an
explicit supported control-flow mapping (ADR 0036).
"""
import re

from dav1d_storage import tokens

NONLOCAL_CALL = re.compile(r'\b(?:_?setjmp|_?longjmp|__sigsetjmp|sigsetjmp|siglongjmp|__longjmp_chk)\s*\(')
JUMP_CORRECTIONS = ('glibc-setjmp-aliases', 'scoped-nonlocal-error-targets',
                    'typed-nonlocal-error-transfer', 'unwinding-codec-and-callback-abi',
                    'system-and-vmaf-c-abi-declarations')


def delimiters(lexemes):
    stack, pairs, owners = [], {}, {}
    for i, token in enumerate(lexemes):
        owners[i] = stack[-1] if stack else None
        if token[0] in ['(', '[', '{']:
            stack.append(i)
        elif token[0] in [')', ']', '}']:
            start = stack.pop()
            if (lexemes[start][0], token[0]) not in [('(', ')'), ('[', ']'), ('{', '}')]:
                raise SystemExit('Unbalanced generated Rust delimiters')
            pairs[start] = i
            pairs[i] = start
    if stack:
        raise SystemExit('Unclosed generated Rust delimiter')
    return pairs, owners


def return_values(body):
    lexemes = tokens(body)
    pairs, _ = delimiters(lexemes)
    replacements = []
    i = 0
    while i < len(lexemes):
        # Nested mapped error scopes own their closure returns. The surrounding
        # dispatch's return is still a return from the original C function.
        if lexemes[i][0] == '{' and i >= 2 and [t[0] for t in lexemes[i - 2:i]] == ['|', '|']:
            i = pairs[i] + 1
            continue
        if lexemes[i][0] != 'return':
            i += 1
            continue
        end = i + 1
        while end < len(lexemes) and lexemes[end][0] not in [';', '}']:
            end = pairs[end] + 1 if lexemes[end][0] in ['(', '[', '{'] else end + 1
        stop = lexemes[end].start() if end < len(lexemes) else len(body)
        consumed = lexemes[end].end() if end < len(lexemes) and lexemes[end][0] == ';' else stop
        value = body[lexemes[i].end():stop].strip() or '()'
        replacements.append((lexemes[i].start(), consumed,
                             f'return crate::jump::Scope::Returned({value});'))
        i = end + 1
    for start, end, replacement in reversed(replacements):
        body = body[:start] + replacement + body[end:]
    return body


def ends_with_return(body):
    lexemes = tokens(body)
    pairs, _ = delimiters(lexemes)
    i = 0
    while i < len(lexemes):
        if lexemes[i][0] == 'return':
            end = i + 1
            while end < len(lexemes) and lexemes[end][0] != ';':
                end = pairs[end] + 1 if lexemes[end][0] in ['(', '[', '{'] else end + 1
            if end == len(lexemes) or end == len(lexemes) - 1:
                return True
        i = pairs[i] + 1 if lexemes[i][0] in ['(', '[', '{'] else i + 1
    return False


def map_scopes(text):
    # Remove only the runtime declarations; all actual jump sites must be
    # recognized below. No direct libc nonlocal jump may survive the import.
    # glibc's setjmp macro expands to _setjmp, which has the same scoped
    # error-target role here. Normalize its declaration and direct call sites.
    text, aliases = re.subn(r'\b_setjmp(?=\s*\()', 'setjmp', text)
    text = re.sub(r'    fn setjmp\([^;]+;\n', '', text)
    text = re.sub(r'    fn longjmp\([^;]+;\n', '', text)
    text, throws = re.subn(r'\blongjmp\(([^;]+)\);', r'crate::jump::raise(\1);', text)
    count = 0
    while re.search(r'\bsetjmp\(', text):
        lexemes = tokens(text)
        pairs, owners = delimiters(lexemes)
        calls = [i for i, token in enumerate(lexemes) if token[0] == 'setjmp' and lexemes[i + 1][0] == '(']
        i = calls[-1]
        if i < 1 or lexemes[i - 1][0] != 'if':
            raise SystemExit('Unmapped setjmp expression; expected original if condition')
        close = pairs[i + 1]
        condition_end = close + 1
        while lexemes[condition_end][0] != '{':
            condition_end += 1
        condition = ''.join(token[0] for token in lexemes[close + 1:condition_end])
        condition = condition.removesuffix('ascore::ffi::c_int')
        if condition not in ['!=0', '==0']:
            raise SystemExit(f'Unmapped setjmp comparison: {condition}')
        branch_end = pairs[condition_end]
        if lexemes[branch_end + 1][0] == 'else':
            raise SystemExit('Unmapped setjmp else branch')
        pointer = text[lexemes[i + 1].end():lexemes[close].start()]
        branch = text[lexemes[condition_end].end():lexemes[branch_end].start()].rstrip()
        start = lexemes[i - 1].start()
        if condition == '==0':
            body = return_values(branch)
            replacement = f'''match crate::jump::scope(({pointer}).cast(), || {{
{body}
crate::jump::Scope::Done
}}) {{
Some(crate::jump::Scope::Returned(value)) => return value,
Some(crate::jump::Scope::Done) | None => (),
}}'''
            end = lexemes[branch_end].end()
        else:
            owner = owners[i - 1]
            while owner is not None and lexemes[owner][0] != '{':
                owner = owners[owner]
            if owner is None:
                raise SystemExit('No enclosing scope for original setjmp')
            end = lexemes[pairs[owner]].start()
            original_body = text[lexemes[branch_end].end():end].rstrip()
            function_bodies = set()
            for function, token in enumerate(lexemes):
                if token[0] == 'fn' and function + 2 < len(lexemes) and lexemes[function + 2][0] == '(':
                    cursor = pairs[function + 2] + 1
                    while lexemes[cursor][0] not in ['{', ';']:
                        cursor += 1
                    if lexemes[cursor][0] == '{':
                        function_bodies.add(cursor)
            if owner in function_bodies or ends_with_return(original_body):
                # The closure contains the rest of a function or a block that
                # explicitly returns. Preserve its original return type; no
                # synthetic fallthrough variant can make that path reachable.
                replacement = f'''loop {{
match crate::jump::value_scope(({pointer}).cast(), || {{{original_body}}}) {{
Some(value) => return value,
None => {{{branch}}}
}}
}}'''
            else:
                body = return_values(original_body)
                replacement = f'''loop {{
match crate::jump::scope(({pointer}).cast(), || {{
{body}
crate::jump::Scope::Done
}}) {{
Some(crate::jump::Scope::Returned(value)) => return value,
Some(crate::jump::Scope::Done) => break,
None => {{{branch}}}
}}
}}'''
        text = text[:start] + replacement + text[end:]
        count += 1
    if NONLOCAL_CALL.search(text):
        raise SystemExit('Unmapped libc nonlocal error transfer')
    return text, {name: value for name, value in {
        'glibc-setjmp-aliases': aliases,
        'scoped-nonlocal-error-targets': count,
        'typed-nonlocal-error-transfer': throws,
    }.items() if value}


def map_jumps(text, ported_functions):
    text, changes = map_scopes(text)
    # All translated frames, declarations, callbacks, and callback casts that
    # may carry a libaom error must permit unwinding. The calling convention and
    # machine representation remain C; foreign consumers are still unchanged.
    abi_count = text.count('extern "C"')
    text = text.replace('extern "C"', 'extern "C-unwind"')
    # System/quality-engine calls do not transfer libaom's typed errors. Keep
    # their actual C ABI, especially the allocator/memory runtime signatures
    # Rust validates against its own runtime. Only translated codec functions
    # and callback frames need C-unwind declarations.
    native = []
    def declaration(match):
        if match[1] in ported_functions:
            return match[0]
        native.append(match[0])
        return ''
    text = re.sub(r'(?m)^    (?:pub )?fn (\w+)\([^;]+;\n', declaration, text)
    if native:
        text += '\nextern "C" {\n' + ''.join(native) + '}\n'
    changes.update({name: value for name, value in {
        'unwinding-codec-and-callback-abi': abi_count,
        'system-and-vmaf-c-abi-declarations': len(native),
    }.items() if value})
    return text, changes
