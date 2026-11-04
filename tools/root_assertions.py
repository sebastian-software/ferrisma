"""Retain one evaluation of each original glibc assertion in C2Rust output."""
import re

from aom_jumps import delimiters

# Preserve quoted diagnostics and character literals when comparing assertions.
LEXEME = re.compile(r'//[^\n]*|/\*[\s\S]*?\*/|b?"(?:\\.|[^"\\])*"|'
                    r"b?'(?:\\(?:u\{[0-9a-fA-F_]+\}|x[0-9a-fA-F]{2}|.)|[^'\\\n])'|"
                    r'[A-Za-z_]\w*|::|[^\s]')


def single_evaluation(text):
    lexemes = [match for match in LEXEME.finditer(text) if not match[0].startswith(('//', '/*'))]
    pairs, owners = delimiters(lexemes)

    def fingerprint(start, stop):
        result = []
        for index in range(start, stop):
            if lexemes[index][0] == ',' and index + 1 < stop and lexemes[index + 1][0] == ')':
                opening = pairs[index + 1]
                callee = lexemes[opening - 1][0] if opening else ''
                if callee == '>' or (re.fullmatch(r'[A-Za-z_]\w*', callee)
                        and callee not in ['if', 'while', 'for', 'match', 'return', 'break', 'as', 'let', 'unsafe', 'fn']):
                    continue  # Optional trailing comma in a function call, not a tuple.
            result.append(lexemes[index][0])
        return result

    replacements, sites = [], []
    for index in range(len(lexemes) - 3):
        if ([token[0] for token in lexemes[index:index + 4]][::2] != ["'", ':']
                or not re.fullmatch(r'c_\d+', lexemes[index + 1][0]) or lexemes[index + 3][0] != '{'):
            continue
        stop = pairs[index + 3]
        body = lexemes[index + 4:stop]
        values = [token[0] for token in body]
        if '__assert_fail' not in values:
            continue
        if values[0] not in ['if', '__assert_fail'] or values.count('__assert_fail') != 1:
            raise SystemExit('Unrecognized generated glibc assertion statement expression')
        if lexemes[index - 1][0] == '}':
            opening = pairs[index - 1]
            if lexemes[opening - 1][0] != 'else' or lexemes[opening - 2][0] != '}':
                raise SystemExit('Unexpected first generated glibc assertion branch')
            true_end = opening - 2
            if pairs[true_end] != true_end - 1:
                raise SystemExit('The generated assertion success branch must be empty')
            starts = [i for i in range(true_end) if lexemes[i][0] == 'if' and owners[i] == owners[index]]
            if not starts:
                raise SystemExit('Cannot locate the unevaluated glibc assertion copy')
            start = starts[-1]
        elif lexemes[index - 1][0] == ';' and lexemes[index - 2][0] == ')':
            start = pairs[index - 2] - 1
            if lexemes[start][0] != '__assert_fail':
                raise SystemExit('Unexpected unconditional glibc assertion copy')
        else:
            raise SystemExit('Cannot delimit the first generated glibc assertion copy')
        if owners[start] != owners[index] or fingerprint(start, index) != fingerprint(index + 4, stop):
            raise SystemExit('The generated glibc assertion copies differ; require an explicit mapping')
        # glibc's sizeof operand must not execute. Retain the identical complete
        # runtime statement expression, including its original failure call.
        first, last = lexemes[start].start(), lexemes[index - 1].end()
        line = text.rfind('\n', 0, first) + 1
        if not text[line:first].strip():
            first = line
        newline = text.find('\n', last)
        if newline != -1 and not text[last:newline].strip():
            last = newline + 1
        replacements.append((first, last))
        sites.append(text[body[0].start():body[-1].end()])
    calls = sum(token[0] == '__assert_fail' and (i == 0 or lexemes[i - 1][0] != 'fn')
                for i, token in enumerate(lexemes))
    if calls != 2 * len(replacements):
        raise SystemExit('Require a complete one-to-one inventory of generated glibc assertion copies')
    for start, end in reversed(replacements):
        text = text[:start] + text[end:]
    return text, len(replacements), sites
