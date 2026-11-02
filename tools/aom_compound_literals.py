"""Name file-scope float compound literals for C2Rust's integer-only exporter."""
import re


def name_float_tables(original):
    # C2Rust 0.21.0 calls evaluateConstantInt on Clang's floating ConstantExpr
    # children in file-scope compound literals, corrupting the exporter stack.
    # At file scope each compound literal already has static storage duration.
    # Name those same writable arrays without changing values or pointer types.
    declarations = []

    def replace(match):
        name = f'c2rust_aom_float_table_{len(declarations)}'
        declarations.append(f'static float {name}[] = {{{match[1]}}};')
        return name + '\n' * match[0].count('\n')

    # The selected source uses this form only in the file-scope pointer
    # tables. Fail closed if the count or literal grammar changes upstream.
    pattern = r'\(float\[\]\)\{([\s\S]*?)\}'
    text = re.sub(pattern, replace, original)
    if len(declarations) != 9:
        raise SystemExit(f'Unexpected libaom compound float table count: {len(declarations)}')
    for declaration in declarations:
        values = declaration.split('{', 1)[1].split('}', 1)[0]
        if not re.fullmatch(r'[\s,0-9.f+-]+', values):
            raise SystemExit('Nonliteral expression in compound float table')
    return ('\n'.join(declarations) + '\n#line 1 "upstream/aom/av1/encoder/tx_search.c"\n'
            + text), len(declarations)
