"""Narrow C2Rust compatibility rules for the selected libaom C configuration."""
import re

from aom_jumps import delimiters
from dav1d_storage import tokens


def control_va_list(text):
    # glibc's array va_list decays in the C callback typedef. Rust's VaList
    # already implements that parameter ABI; keep it consistent with the
    # translated callback definitions instead of casting function pointers.
    pattern = (r'(pub type aom_codec_control_fn_t = Option<\s*'
               r'unsafe extern "C-unwind" fn\(\s*\*mut aom_codec_alg_priv_t,\s*)'
               r'\*mut __va_list_tag(,\s*\) -> aom_codec_err_t,\s*>;)')
    if re.search(pattern, text):
        record = (r'pub struct __va_list_tag \{\s*pub gp_offset: core::ffi::c_uint,\s*'
                  r'pub fp_offset: core::ffi::c_uint,\s*'
                  r'pub overflow_arg_area: \*mut core::ffi::c_void,\s*'
                  r'pub reg_save_area: \*mut core::ffi::c_void,\s*\}')
        if (not re.search(record, text) or ('pub type __builtin_va_list =' in text
                and 'pub type __builtin_va_list = [__va_list_tag; 1];' not in text)):
            raise SystemExit('Unexpected array-decayed libaom control va_list representation')
    return re.subn(pattern, r"\1core::ffi::VaList<'_>\2", text)


def cast_calls(text, pattern, target):
    replacements = []
    for match in re.finditer(pattern, text):
        lexemes = tokens(text[match.end() - 1:])
        depth, end = 0, None
        for token in lexemes:
            if token[0] == '(':
                depth += 1
            elif token[0] == ')':
                depth -= 1
                if depth == 0:
                    end = match.end() - 1 + token.end()
                    break
        if end is None:
            raise SystemExit('Unbalanced C2Rust conversion call')
        expression = text[match.end():end - 1]
        replacements.append((match.start(), end, f'({expression}) as {target}'))
    for start, end, value in reversed(replacements):
        text = text[:start] + value + text[end:]
    return text, len(replacements)


def compatibility(text):
    changes = {}
    text, count = re.subn(r'\.arg::<', '.next_arg::<', text)
    if count:
        changes['current-rust-variadic-argument-method'] = count
    text, count = re.subn(r'let mut (\w+): va_list = 0 as \*mut core::ffi::c_char;\s*\1 = (\w+)\.clone\(\);',
                          r'let mut \1 = \2.clone();', text)
    if count:
        changes['variadic-list-clone-initializer'] = count
    count = 0
    if 'as bool' in text:
        lexemes = tokens(text)
        pairs, _ = delimiters(lexemes)
        boolean_functions = set()
        for i, token in enumerate(lexemes):
            if token[0] == 'fn' and i + 2 < len(lexemes) and lexemes[i + 2][0] == '(':
                close = pairs[i + 2]
                if [t[0] for t in lexemes[close + 1:close + 4]] == ['-', '>', 'bool']:
                    boolean_functions.add(lexemes[i + 1][0])
        conversions = []
        for i, token in enumerate(lexemes):
            if token[0] != ')' or [t[0] for t in lexemes[i + 1:i + 3]] != ['as', 'bool']:
                continue
            opening = pairs[i]
            if opening and lexemes[opening - 1][0] in boolean_functions:
                continue  # C2Rust's existing bool-to-bool cast is already valid.
            operand = text[lexemes[opening].end():token.start()]
            if not re.search(r'\bas\s+core::ffi::c_int\b', operand):
                raise SystemExit('Unmapped C2Rust integer-to-boolean conversion')
            conversions.append((token.end(), lexemes[i + 2].end()))
        for start, end in reversed(conversions):
            text = text[:start] + ' != 0' + text[end:]
        count = len(conversions)
    if count:
        changes['c-boolean-integer-conversion'] = count
    text, count = cast_calls(text, r'core::ffi::c_double::new\(', 'core::ffi::c_double')
    if count:
        changes['darwin-long-double-constructor'] = count
    text, count = re.subn(r'_opaque_pthread_attr_t \{\s*__sig: 0,\s*__opaque: \[0; \d+\],?\s*\}',
                          'core::mem::zeroed()', text)
    if count:
        changes['target-libc-thread-attribute-zero-value'] = count
    text, count = cast_calls(text, r'::core::mem::transmute::<\s*core::ffi::c_int,\s*BLOCK_SIZE,\s*>\(', 'BLOCK_SIZE')
    if count:
        if not re.search(r'pub type BLOCK_SIZE = (?:uint8_t|core::ffi::c_uchar);', text):
            raise SystemExit('Narrow packed enum arithmetic requires the original uint8_t type')
        changes['packed-byte-enum-assignment-cast'] = count
    text, count = control_va_list(text)
    if count:
        changes['sysv-array-decayed-control-va-list'] = count
    return text, changes
