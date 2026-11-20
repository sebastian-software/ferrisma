"""Typing-aware C-subset printer for libaom's x86 intrinsics files (not C2Rust).

Usage: python3 tools/x86_c2r/c2r.py configs/NAME.json [WORK_DIR]
       (normally driven by tools/x86_c2r/assemble.py)

Needs pycparser and clang's x86 intrinsic headers (CLANG_INCLUDE, default:
the Command Line Tools on macOS) for the intrinsic signatures. Written for the
x86 counterparts of the still-image Neon families (port ADR 0058, amendment
of 2026-11-14); like the printer of the high-bitdepth transforms it is not
part of the build. Its output is reviewed and tested like a hand
translation: tools/x86_c2r/callseq.py compares the call sequence of every
function with the C, and the gates of ADR 0056/0058 compare the result with
the original's SIMD build.

The config names the C source, the target feature, the stub prelude (typedefs
and prototypes of external functions), Rust-side macros for C statement
macros, exported RTCD names, and functions to skip. The printer keeps the C
statement order, names, constants and comments; it makes C's integer
conversions explicit and fails on any construct it does not know.
"""
import json
import os
import re
import sys
from pathlib import Path

from pycparser import c_ast, c_parser

import intrin

INTRIN = intrin.load()

SCALAR = {
    'int': 'i32', 'signed int': 'i32', 'unsigned int': 'u32', 'unsigned': 'u32', 'short': 'i16',
    'unsigned short': 'u16', 'char': 'i8', 'signed char': 'i8', 'unsigned char': 'u8', 'long long': 'i64',
    'unsigned long long': 'u64', 'long': 'i64', 'unsigned long': 'u64',
    'int8_t': 'i8', 'int16_t': 'i16', 'int32_t': 'i32', 'int64_t': 'i64', 'uint8_t': 'u8', 'uint16_t': 'u16',
    'uint32_t': 'u32', 'uint64_t': 'u64', 'size_t': 'usize', 'intptr_t': 'isize', 'ptrdiff_t': 'isize',
    'tran_low_t': 'i32', 'TX_SIZE': 'u8', 'TX_TYPE': 'u8', 'ITX_TYPE_1D': 'u8', 'bool': 'bool',
    'double': 'f64', 'float': 'f32', 'TX_CLASS': 'u8', 'PREDICTION_MODE': 'u8', 'qm_val_t': 'u8', 'CONV_BUF_TYPE': 'u16', 'TXFM_TYPE': 'u8', 'TX_TYPE_1D': 'u8',
}
VECTORS = {'__m128i', '__m256i', '__m128', '__m256', '__m128d', '__m256d', '__m64'}
RANK = {'i32': (1, False), 'u32': (1, True), 'i64': (2, False), 'u64': (2, True), 'isize': (2, False),
        'usize': (2, True)}
# Shift intrinsics whose count is an `int` in C and an immediate in Rust.
IMM_SHIFTS = re.compile(r'^_mm(256)?_(slli|srli|srai)_(epi16|epi32|epi64)$')
REG_SHIFT = {'slli': 'sll', 'srli': 'srl', 'srai': 'sra'}


class Unsupported(Exception):
    pass


# Types: ('int', rust) | ('lit',) integer literal/const expr | ('bool',) | ('vec', name) |
# ('ptr', T, const) | ('arr', T, size_rust) | ('fn', ret, [params], name) | ('fnptr', typedef) |
# ('struct', name) | ('void',) | ('float', 'f32'|'f64') | ('flit',) floating literal
def rust_type(t):
    k = t[0]
    if k in ('int', 'float'):
        return t[1]
    if k == 'flit':
        return 'f64'
    if k == 'lit':
        return 'i32'
    if k == 'bool':
        return 'bool'
    if k == 'vec':
        # Rust dropped the MMX type; a `__m64 *` only ever addresses 8 bytes here.
        return 'u64' if t[1] == '__m64' else t[1]
    if k == 'ptr':
        inner = 'core::ffi::c_void' if t[1][0] == 'void' else rust_type(t[1])
        return ('*const ' if t[2] else '*mut ') + inner
    if k == 'arr':
        return f'[{rust_type(t[1])}; {t[2]}]'
    if k == 'fnptr':
        return t[1]
    if k == 'struct':
        return t[1]
    if k == 'void':
        return '()'
    raise Unsupported(f'type {t}')


def is_int(t):
    return t[0] in ('int', 'lit', 'bool')


def promote(t):
    if t[0] == 'lit':
        return t
    if t[0] == 'bool':
        return ('int', 'i32')
    if t[0] == 'int' and t[1] in ('i8', 'i16', 'u8', 'u16'):
        return ('int', 'i32')
    return t


def common(a, b):
    if a[0] in ('float', 'flit') or b[0] in ('float', 'flit'):
        # float op double is double; an unsuffixed literal is a double.
        if ('float', 'f64') in (a, b) or ('flit',) in (a, b) and a != b:
            return ('float', 'f64') if ('flit',) not in (a, b) or ('float', 'f32') not in (a, b) else ('float', 'f64')
        return a if a[0] == 'float' else b
    a, b = promote(a), promote(b)
    if a[0] == 'lit':
        return b
    if b[0] == 'lit':
        return a
    ra, rb = RANK[a[1]], RANK[b[1]]
    if ra[0] != rb[0]:
        return a if ra[0] > rb[0] else b
    return a if ra[1] else b


# Rust precedence, higher binds tighter.
PREC = {'postfix': 14, 'unary': 13, 'as': 12, '*': 11, '/': 11, '%': 11, '+': 10, '-': 10, '<<': 9, '>>': 9,
        '&': 8, '^': 7, '|': 6, '==': 5, '!=': 5, '<': 5, '>': 5, '<=': 5, '>=': 5, '&&': 4, '||': 3,
        'atom': 15, 'low': 0}


class E:
    """An emitted expression: code, type, precedence, and whether it is a constant expression."""

    def __init__(self, code, typ, prec='atom', const=False, place=False):
        self.code, self.typ, self.prec, self.const, self.place = code, typ, PREC[prec], const, place

    def at(self, prec):
        """Code usable as an operand that needs at least precedence `prec`."""
        return self.code if self.prec >= PREC[prec] else f'({self.code})'


class Printer:
    def __init__(self, config, comments, source_lines):
        self.cfg = config
        self.feature = config['feature']
        self.macros = config.get('macros', {})
        self.exports = set(config.get('rtcd', []))
        self.public = set(config.get('public', []))
        self.consts = dict(config.get('consts', {}))
        self.structs = config.get('structs', {})
        self.fnptrs = {}
        self.funcs = {}
        self.globals = {}
        self.typedefs = {}
        self.comments = comments
        self.comment_lines = sorted(comments)
        self.emitted_comments = set()
        self.out = []
        self.indent = 0
        self.scopes = []
        self.fn_ret = None
        self.source_lines = source_lines
        self.aligned = {}
        self.aligned_locals = set()
        self.aligned_globals = set()
        self.fn_body = None
        self.const_vars = set()
        self.loop_next = []

    # ---------------------------------------------------------------- types
    def ctype(self, node, quals=()):
        if isinstance(node, c_ast.TypeDecl):
            return self.ctype(node.type, node.quals)
        if isinstance(node, c_ast.IdentifierType):
            name = ' '.join(node.names)
            if name in SCALAR:
                r = SCALAR[name]
                return ('bool',) if r == 'bool' else ('float', r) if r in ('f32', 'f64') else ('int', r)
            if name in VECTORS:
                return ('vec', name)
            if name == 'void':
                return ('void',)
            if name in self.fnptrs:
                return ('fnptr', name)
            if name in self.structs:
                return ('struct', name)
            if name in self.typedefs:
                return self.typedefs[name]
            raise Unsupported(f'type name {name}')
        if isinstance(node, c_ast.PtrDecl):
            inner = node.type
            const = isinstance(inner, c_ast.TypeDecl) and 'const' in inner.quals
            return ('ptr', self.ctype(inner), const)
        if isinstance(node, c_ast.ArrayDecl):
            size = self.expr(node.dim).code if node.dim is not None else None
            return ('arr', self.ctype(node.type), f'({size}) as usize' if size and not size.isdigit() else size)
        if isinstance(node, c_ast.Struct):
            return ('struct', node.name)
        if isinstance(node, c_ast.FuncDecl):
            return self.functype(node, None)
        raise Unsupported(f'type node {type(node).__name__}')

    def functype(self, decl, name):
        params = []
        if decl.args:
            for p in decl.args.params:
                if isinstance(p, c_ast.Typename) and isinstance(p.type, c_ast.TypeDecl) and \
                        isinstance(p.type.type, c_ast.IdentifierType) and p.type.type.names == ['void']:
                    continue
                t = self.ctype(p.type)
                if t[0] == 'arr':  # array parameters decay
                    t = ('ptr', t[1], False)
                params.append((p.name, t))
        return ('fn', self.ctype(decl.type), params, name)

    # ---------------------------------------------------------------- scopes
    def lookup(self, name):
        for scope in reversed(self.scopes):
            if name in scope:
                return scope[name]
        if name in self.globals:
            return self.globals[name]
        if name in self.funcs:
            return self.funcs[name]
        if name in self.consts:
            return ('const', self.consts[name])
        if name in INTRIN:
            return ('intrin', name)
        raise Unsupported(f'unknown identifier {name}')

    # ---------------------------------------------------------------- casts
    def cast(self, e, target):
        """Convert e to type target the way C's implicit conversion does."""
        t = e.typ
        if target[0] == 'bool':
            return self.boolean(e)
        if target[0] == 'float':
            if t == target or t[0] == 'flit':
                return e
            if t[0] in ('int', 'lit', 'float'):
                return E(f'{e.at("as")} as {target[1]}', target, 'as', e.const)
            raise Unsupported(f'convert {t} to {target}')
        if target[0] == 'int' and t[0] in ('float', 'flit'):
            # C truncates toward zero; the values the kernels convert are in range.
            return E(f'{e.at("as")} as {target[1]}', target, 'as', e.const)
        if target[0] == 'int':
            if t[0] == 'lit':
                if target[1] == 'i32':
                    return e
                if e.code.lstrip('-').isdigit() or re.fullmatch(r'-?0x[0-9a-fA-F]+', e.code):
                    v = int(e.code, 0) if not e.code.startswith('-') else -int(e.code[1:], 0)
                    bits = {'i8': 8, 'u8': 8, 'i16': 16, 'u16': 16, 'i32': 32, 'u32': 32}.get(target[1], 64)
                    signed = target[1].startswith('i')
                    lo, hi = (-(1 << (bits - 1)), (1 << (bits - 1)) - 1) if signed else (0, (1 << bits) - 1)
                    if not lo <= v <= hi:
                        # C converts the int constant modulo 2^N.
                        return E(f'{e.code.lstrip("-") if v >= 0 else e.code}_i32 as {target[1]}'
                                 if v >= 0 else f'({e.code}_i32) as {target[1]}', target, 'as', True)
                    return E(e.code, target, 'unary' if e.code.startswith('-') else 'atom', True)
                return E(f'{e.at("as")} as {target[1]}', target, 'as', e.const)
            if t == target:
                return e
            if t[0] in ('int', 'bool'):
                return E(f'{e.at("as")} as {target[1]}', target, 'as', e.const)
            raise Unsupported(f'convert {t} to {target}')
        if target[0] == 'ptr':
            if t[0] == 'arr':
                e = self.decay(e)
                t = e.typ
            if t == target:
                return e
            if t[0] == 'ptr':
                return E(f'{e.at("as")} as {rust_type(target)}', target, 'as')
            if t[0] == 'lit' and e.code == '0':
                return E('core::ptr::null_mut()' if not target[2] else 'core::ptr::null()', target)
            raise Unsupported(f'convert {t} to {target}')
        if target[0] == 'fnptr':
            if t[0] == 'fn':
                return E(f'Some({e.code})', target)
            if t[0] == 'lit' and e.code == '0':
                return E('None', target)
            if t == target:
                return e
            raise Unsupported(f'convert {t} to {target}')
        if t != target and not (t[0] == 'vec' and target[0] == 'vec' and t[1] == target[1]):
            if t[0] == 'struct' and target[0] == 'struct':
                return e
            raise Unsupported(f'convert {t} to {target}')
        return e

    def decay(self, e):
        """An array as a pointer to its first element: as_mut_ptr for locals, as_ptr for statics."""
        base = e.code.split('[')[0]
        base = base[:-2] if base.endswith('.0') else base
        static = base in self.globals
        method = 'as_ptr' if static else 'as_mut_ptr'
        return E(f'{e.at("postfix")}.{method}()', ('ptr', e.typ[1], static), 'postfix')

    def boolean(self, e):
        if e.typ[0] == 'bool':
            return e
        if e.typ[0] in ('int', 'lit'):
            return E(f'{e.at("<")} != 0', ('bool',), '!=')
        if e.typ[0] == 'ptr':
            return E(f'!{e.at("postfix")}.is_null()', ('bool',), 'unary')
        raise Unsupported(f'boolean of {e.typ}')

    def as_int(self, e):
        if e.typ[0] == 'bool':
            return E(f'{e.at("as")} as i32', ('int', 'i32'), 'as')
        return e

    # ---------------------------------------------------------------- expressions
    def expr(self, n):
        m = getattr(self, 'x_' + type(n).__name__, None)
        if m is None:
            raise Unsupported(f'expression {type(n).__name__} at {n.coord}')
        return m(n)

    def x_Constant(self, n):
        if n.type in ('int', 'unsigned int', 'long int', 'long long int', 'unsigned long long int'):
            v = n.value.rstrip('uUlL')
            if n.type.startswith('unsigned'):
                # The suffix keeps the type through `!` and `as` (`~0u` is 0xffffffff).
                t = 'u64' if 'long long' in n.type else 'u32'
                return E(f'{v}_{t}', ('int', t), const=True)
            return E(v, ('lit',), const=True)
        if n.type == 'char':
            return E(n.value.replace("'", "b'", 1) + ' as i8', ('int', 'i8'), 'as', True)
        if n.type in ('double', 'float'):
            v = n.value.rstrip('fF')
            return E(v if '.' in v or 'e' in v.lower() else v + '.0', ('flit',), const=True)
        raise Unsupported(f'constant {n.type} {n.value}')

    def x_ID(self, n):
        name = n.name
        if name == 'NULL':
            return E('0', ('lit',), const=True)
        v = self.lookup(name)
        if v[0] == 'const':
            return E(name, ('lit',) if v[1] in ('i32', None) else ('int', v[1]), const=True)
        if v[0] == 'intrin':
            raise Unsupported(f'intrinsic {name} used as a value')
        if v[0] == 'fn':
            return E(name, v)
        if (name in self.aligned_globals or name in self.cfg.get('aligned_externs', [])) and \
                not any(name in sc for sc in self.scopes):
            return E(f'{name}.0', v, 'postfix', place=True)
        if name in self.aligned_locals and any(name in sc for sc in self.scopes):
            return E(f'{rename(name)}.0', v, 'postfix', place=True)
        return E(rename(name), v, place=True)

    def x_UnaryOp(self, n):
        op = n.op
        if op == 'sizeof':
            t = self.ctype(n.expr.type) if isinstance(n.expr, c_ast.Typename) else self.expr(n.expr).typ
            return E(f'core::mem::size_of::<{rust_type(t)}>()', ('int', 'usize'), 'postfix')
        if op in ('p++', 'p--', '++', '--'):
            raise Unsupported(f'{op} inside an expression at {n.coord}')
        e = self.expr(n.expr)
        if op == '-':
            if e.typ[0] == 'lit' and e.prec == PREC['atom'] and e.code.isdigit():
                return E('-' + e.code, e.typ, 'unary', True)
            e = self.as_int(e)
            t = promote(e.typ)
            e = self.cast(e, t) if t[0] != 'lit' else e
            return E(f'-{e.at("unary")}', t, 'unary', e.const)
        if op == '+':
            return e
        if op == '~':
            t = promote(self.as_int(e).typ)
            e = self.cast(self.as_int(e), t) if t[0] != 'lit' else e
            return E(f'!{e.at("unary")}', t, 'unary', e.const)
        if op == '!':
            b = self.boolean(e)
            return E(f'!{b.at("unary")}', ('bool',), 'unary', e.const)
        if op == '*':
            if e.typ[0] == 'ptr' and isinstance(n.expr, c_ast.Cast) and e.typ[1][0] == 'int':
                # `*(int *)p`: a scalar access through a cast pointer, which x86
                # compilers emit as an unaligned move.
                r = E(f'core::ptr::read_unaligned({e.code})', e.typ[1], 'postfix')
                r.unaligned = e.code
                return r
            if e.typ[0] == 'ptr':
                return E(f'*{e.at("unary")}', e.typ[1], 'unary', place=True)
            if e.typ[0] == 'arr':
                return E(f'{e.at("postfix")}[0]', e.typ[1], 'postfix', place=True)
            raise Unsupported(f'deref of {e.typ}')
        if op == '&':
            if isinstance(n.expr, c_ast.ArrayRef):
                base = self.expr(n.expr.name)
                idx = self.index_value(n.expr.subscript)
                if base.typ[0] == 'arr':
                    p = self.decay(base)
                elif base.typ[0] == 'ptr':
                    p = base
                else:
                    raise Unsupported(f'&[] of {base.typ}')
                return self.ptr_offset(p, idx)
            if e.typ[0] == 'fn':
                return e
            if isinstance(n.expr, c_ast.ID) and n.expr.name in self.const_vars:
                return E(f'&raw const {e.at("postfix")}', ('ptr', e.typ, True), 'unary')
            return E(f'&raw mut {e.at("postfix")}', ('ptr', e.typ, False), 'unary')
        raise Unsupported(f'unary {op}')

    def index_value(self, sub):
        return self.as_int(self.expr(sub))

    def ptr_offset(self, p, idx, negate=False):
        if idx.typ[0] == 'lit' and idx.code.lstrip('-').isdigit():
            v = -int(idx.code) if negate else int(idx.code)
            if v == 0:
                return p
            return E(f'{p.at("postfix")}.offset({v})', p.typ, 'postfix')
        code = idx.code if idx.typ == ('int', 'isize') else f'{idx.at("as")} as isize'
        if negate:
            code = f'-({code})'
        return E(f'{p.at("postfix")}.offset({code})', p.typ, 'postfix')

    def x_ArrayRef(self, n):
        base = self.expr(n.name)
        idx = self.index_value(n.subscript)
        if base.typ[0] == 'arr':
            if idx.typ[0] == 'lit' and idx.code.isdigit():
                i = idx.code
            elif idx.typ == ('int', 'usize'):
                i = idx.code
            else:
                i = f'{idx.at("as")} as usize'
            return E(f'{base.at("postfix")}[{i}]', base.typ[1], 'postfix', place=True)
        if base.typ[0] == 'ptr':
            p = self.ptr_offset(base, idx)
            if isinstance(n.name, c_ast.Cast) and base.typ[1][0] == 'int':
                r = E(f'core::ptr::read_unaligned({p.code})', base.typ[1], 'postfix')
                r.unaligned = p.code
                return r
            return E(f'*{p.at("unary")}', base.typ[1], 'unary', place=True)
        raise Unsupported(f'index into {base.typ} at {n.coord}')

    def x_StructRef(self, n):
        base = self.expr(n.name)
        if n.type == '->':
            if base.typ[0] != 'ptr' or base.typ[1][0] != 'struct':
                raise Unsupported(f'-> on {base.typ}')
            sname = base.typ[1][1]
            code = f'(*{base.at("unary")}).{n.field.name}'
        else:
            sname = base.typ[1]
            code = f'{base.at("postfix")}.{n.field.name}'
        ftype = self.structs[sname][n.field.name]
        return E(code, ftype_parse(ftype), 'postfix', place=True)

    def x_Cast(self, n):
        target = self.ctype(n.to_type.type)
        e = self.expr(n.expr)
        if target[0] == 'void':
            return E(f'let _ = {e.code}', ('void',), 'low')
        if target[0] in ('int', 'bool'):
            e = self.as_int(e)
            if e.typ == target:
                return e
            if e.typ[0] == 'lit' and e.prec == PREC['atom']:
                return self.cast(e, target)
            return E(f'{e.at("as")} as {target[1]}', target, 'as', e.const)
        if target[0] == 'ptr':
            if e.typ[0] == 'arr':
                e = self.decay(e)
            return E(f'{e.at("as")} as {rust_type(target)}', target, 'as')
        if target[0] == 'float':
            e = self.as_int(e)
            if e.typ == target:
                return e
            return E(f'{e.at("as")} as {target[1]}', target, 'as', e.const)
        raise Unsupported(f'cast to {target}')

    def x_BinaryOp(self, n):
        op = n.op
        a, b = self.expr(n.left), self.expr(n.right)
        if op in ('&&', '||'):
            a, b = self.boolean(a), self.boolean(b)
            return E(f'{a.at(op)} {op} {b.at(op)}' if True else '', ('bool',), op, a.const and b.const)
        if op in ('+', '-') and a.typ[0] in ('ptr', 'arr'):
            if a.typ[0] == 'arr':
                a = self.decay(a)
            if b.typ[0] == 'ptr' and op == '-':
                return E(f'{a.at("postfix")}.offset_from({b.code})', ('int', 'isize'), 'postfix')
            return self.ptr_offset(a, self.as_int(b), op == '-')
        if op == '+' and b.typ[0] in ('ptr', 'arr'):
            if b.typ[0] == 'arr':
                b = self.decay(b)
            return self.ptr_offset(b, self.as_int(a))
        if op in ('==', '!=') and (a.typ[0] in ('ptr', 'fnptr') or b.typ[0] in ('ptr', 'fnptr')):
            x, y = (a, b) if a.typ[0] in ('ptr', 'fnptr') else (b, a)
            if y.typ[0] == 'lit' and y.code == '0':
                test = 'is_null' if x.typ[0] == 'ptr' else 'is_none'
                neg = '' if op == '==' else '!'
                return E(f'{neg}{x.at("postfix")}.{test}()', ('bool',), 'unary' if neg else 'postfix')
            return E(f'{a.at("&")} {op} {self.cast(b, a.typ).at("&")}', ('bool',), op)
        if op in ('<', '>', '<=', '>=') and a.typ[0] == 'ptr' and b.typ[0] == 'ptr':
            return E(f'{a.at("&")} {op} {self.cast(b, a.typ).at("&")}', ('bool',), op)
        a, b = self.as_int(a), self.as_int(b)
        if a.typ[0] in ('float', 'flit') or b.typ[0] in ('float', 'flit'):
            if op not in ('+', '-', '*', '/', '==', '!=', '<', '>', '<=', '>='):
                raise Unsupported(f'floating point {op}')
            t = common(a.typ, b.typ)
            if t == ('flit',):
                t = ('float', 'f64')
            ac, bc = self.cast(a, t), self.cast(b, t)
            if op in ('==', '!=', '<', '>', '<=', '>='):
                left = ac.code if ac.prec > PREC['as'] else f'({ac.code})'
                return E(f'{left} {op} {bc.at("&")}', ('bool',), op)
            right = bc.code if bc.prec > PREC[op] else f'({bc.code})'
            return E(f'{ac.at(op)} {op} {right}', t, op)
        if op in ('<<', '>>'):
            t = promote(a.typ)
            ac = self.cast(a, t) if t[0] != 'lit' else a
            bc = b if b.typ[0] != 'bool' else self.as_int(b)
            left = ac.code if ac.prec > PREC['as'] else f'({ac.code})'
            right = bc.code if bc.prec >= PREC['as'] else f'({bc.code})'
            return E(f'{left} {op} {right}', t, op, a.const and b.const)
        t = common(a.typ, b.typ)
        ac = a if t[0] == 'lit' else self.cast(a, t)
        bc = b if t[0] == 'lit' else self.cast(b, t)
        if op in ('==', '!=', '<', '>', '<=', '>='):
            # Rust comparisons are non-associative: parenthesize operands of equal precedence.
            left = ac.code if ac.prec > PREC['as'] or (ac.prec >= PREC['&'] and ac.prec < PREC['as']) else f'({ac.code})'
            if ac.prec == PREC['as'] and op in ('<', '<='):
                left = f'({ac.code})'
            return E(f'{left} {op} {bc.at("&")}', ('bool',), op, a.const and b.const)
        if op not in ('+', '-', '*', '/', '%', '&', '|', '^'):
            raise Unsupported(f'binary {op}')
        # Left associative: the right operand needs a strictly higher precedence.
        if op in ('&', '|', '^'):
            left = ac.code if ac.prec >= PREC['as'] else f'({ac.code})'
            right = bc.code if bc.prec >= PREC['as'] else f'({bc.code})'
        else:
            left = ac.at(op)
            right = bc.code if bc.prec > PREC[op] else f'({bc.code})'
        return E(f'{left} {op} {right}', t, op, a.const and b.const)

    def x_TernaryOp(self, n):
        c = self.boolean(self.expr(n.cond))
        a, b = self.expr(n.iftrue), self.expr(n.iffalse)
        if is_int(a.typ) and is_int(b.typ):
            a, b = self.as_int(a), self.as_int(b)
            t = common(a.typ, b.typ)
            if t[0] != 'lit':
                a, b = self.cast(a, t), self.cast(b, t)
        elif {a.typ[0], b.typ[0]} & {'float', 'flit'} and {a.typ[0], b.typ[0]} <= {'float', 'flit', 'int', 'lit'}:
            t = common(self.as_int(a).typ, self.as_int(b).typ)
            if t == ('flit',):
                t = ('float', 'f64')
            a, b = self.cast(a, t), self.cast(b, t)
        else:
            t = a.typ
            b = self.cast(b, t)
        return E(f'if {c.code} {{ {a.code} }} else {{ {b.code} }}', t, 'low', a.const and b.const and c.const)

    def x_Assignment(self, n):
        raise Unsupported(f'assignment inside an expression at {n.coord}')

    def x_FuncCall(self, n):
        args = n.args.exprs if n.args else []
        if isinstance(n.name, c_ast.ID):
            name = n.name.name
            if name in self.macros:
                mac = self.macros[name]
                parts = [self.expr(a).code for a in args] + list(mac.get('implicit', []))
                rt = ftype_parse(mac['type']) if isinstance(mac, dict) and 'type' in mac else ('void',)
                return E(f'{name}!({", ".join(parts)})', rt, 'postfix')
            if name == 'CONVERT_TO_SHORTPTR':
                # aom_ports/mem.h: ((uint16_t *)(((uintptr_t)(x)) << 1))
                x = self.expr(args[0])
                return E(f'(({x.at("as")} as usize) << 1) as *mut u16', ('ptr', ('int', 'u16'), False), 'as')
            if name == 'CONVERT_TO_BYTEPTR':
                x = self.expr(args[0])
                return E(f'(({x.at("as")} as usize) >> 1) as *mut u8', ('ptr', ('int', 'u8'), False), 'as')
            if name == '_MM_SHUFFLE':
                # xmmintrin.h: (((z) << 6) | ((y) << 4) | ((x) << 2) | (w))
                z, y, x, w = (self.as_int(self.expr(a)) for a in args)
                return E(f'(({z.code}) << 6) | (({y.code}) << 4) | (({x.code}) << 2) | ({w.code})', ('lit',), '|', True)
            if name in ('LIKELY', 'UNLIKELY'):
                # aom_dsp/aom_dsp_common.h: __builtin_expect(v, 1), a hint only.
                return self.expr(args[0])
            if name in ('memcpy', 'memset'):
                d, x, n_ = (self.expr(a) for a in args)
                if d.typ[0] == 'arr':
                    d = self.decay(d)
                if name == 'memset':
                    return E(f'core::ptr::write_bytes({d.at("as")} as *mut u8, {self.cast(self.as_int(x), ("int", "u8")).code}, '
                             f'{self.cast(self.as_int(n_), ("int", "usize")).code})', ('void',), 'postfix')
                if x.typ[0] == 'arr':
                    x = self.decay(x)
                return E(f'core::ptr::copy_nonoverlapping({x.at("as")} as *const u8, {d.at("as")} as *mut u8, '
                         f'{self.cast(self.as_int(n_), ("int", "usize")).code})', ('void',), 'postfix')
            if name == 'static_assert':
                return E(f'const _: () = assert!({self.boolean(self.expr(args[0])).code})', ('void',), 'low')
            if name == 'assert':
                a0 = args[0]
                if isinstance(a0, c_ast.BinaryOp) and a0.op == '&&' and isinstance(a0.right, c_ast.Constant) \
                        and a0.right.type == 'string':
                    return E(f'debug_assert!({self.boolean(self.expr(a0.left)).code}, {a0.right.value})',
                             ('void',), 'postfix')
                return E(f'debug_assert!({self.boolean(self.expr(a0)).code})', ('void',), 'postfix')
            if name in INTRIN and name not in self.funcs:
                return self.intrinsic(name, args, n)
            target = self.lookup(name)
            if target[0] == 'fn':
                return self.call(name, target, args)
            if target[0] == 'fnptr':
                sig = self.fnptrs[target[1]]
                return self.call(f'{rename(name)}.unwrap()', sig, args)
            raise Unsupported(f'call of {name}: {target}')
        callee = self.expr(n.name)
        if callee.typ[0] == 'fnptr':
            sig = self.fnptrs[callee.typ[1]]
            return self.call(f'{callee.at("postfix")}.unwrap()', sig, args)
        raise Unsupported(f'call through {callee.typ}')

    def call(self, code, sig, args):
        _, ret, params, _ = sig
        if len(params) != len(args):
            raise Unsupported(f'{code}: {len(args)} arguments for {len(params)} parameters')
        parts = [self.cast(self.as_int(self.expr(a)) if is_int(p[1]) else self.expr(a), p[1]).code
                 for a, p in zip(args, params)]
        return E(f'{code}({", ".join(parts)})', ret, 'postfix')

    def intrinsic(self, name, args, n):
        if name == '_mm_loadh_pi':
            # Rust has no MMX `__m64` intrinsics any more. MOVHPS and MOVHPD load
            # the same 64 bits into the upper half and keep the lower one, so the
            # double form gives the same bits; the pointer need not be aligned.
            a = self.cast(self.expr(args[0]), ('vec', '__m128'))
            p = self.expr(args[1])
            return E(f'_mm_castpd_ps(_mm_loadh_pd(_mm_castps_pd({a.code}), '
                     f'&core::ptr::read_unaligned({p.at("as")} as *const f64)))', ('vec', '__m128'), 'postfix')
        info = INTRIN[name]
        es = [self.expr(a) for a in args]
        m = IMM_SHIFTS.match(name)
        if m:
            count = self.as_int(es[1])
            if count.const:
                return E(f'{name}::<{{ {count.code} }}>({es[0].code})', es[0].typ, 'postfix')
            wide = '256' if m.group(1) else ''
            reg = f'_mm{wide}_{REG_SHIFT[m.group(2)]}_{m.group(3)}'
            c = self.cast(count, ('int', 'i32'))
            return E(f'{reg}({es[0].code}, _mm_cvtsi32_si128({c.code}))', es[0].typ, 'postfix')
        if info['macro']:
            imm = self.as_int(es[-1])
            if not imm.const:
                raise Unsupported(f'{name}: non-constant immediate at {n.coord}')
            rest = []
            for e in es[:-1]:
                if e.typ[0] == 'arr':  # a table passed to a gather decays to its first element
                    e = self.decay(e)
                rest.append(self.cast(self.as_int(e), ('int', 'i32')).code if is_int(e.typ) else e.code)
            ret = MACRO_RET.get(name) or (('vec', '__m256i') if name.startswith('_mm256') else ('vec', '__m128i'))
            imm_code = imm.code if imm.prec == PREC['atom'] else f'{{ {imm.code} }}'
            return E(f'{name}::<{imm_code}>({", ".join(rest)})', ret, 'postfix')
        params = info['params']
        if len(params) != len(es):
            raise Unsupported(f'{name}: argument count')
        parts = []
        for e, p in zip(es, params):
            c = self.cast(self.as_int(e) if is_int(e.typ) else e, self.intrin_param(name, p)).code
            if name in RUST_REF and p.strip().endswith('*'):
                # Rust takes `&T`; the C pointer need not be aligned for the
                # instruction, so the value is read unaligned first.
                c = f'&core::ptr::read_unaligned({c})'
            parts.append(c)
        return E(f'{name}({", ".join(parts)})', self.intrin_type(name, info['ret']), 'postfix')

    def intrin_param(self, name, p):
        p = p.strip()
        if p.endswith('*'):
            inner = p[:-1].strip()
            const = 'load' in name or 'lddqu' in name
            if name in RUST_PTR:
                return RUST_PTR[name]
            if inner == 'void':
                raise Unsupported(f'{name}: void pointer')
            return ('ptr', self.intrin_type(name, inner), const)
        return self.intrin_type(name, p)

    def intrin_type(self, name, p):
        if p in VECTORS:
            return ('vec', p)
        if p == 'void':
            return ('void',)
        if p in SCALAR:
            r = SCALAR[p]
            return ('float', r) if r in ('f32', 'f64') else ('int', r)
        raise Unsupported(f'{name}: intrinsic type {p}')

    # ---------------------------------------------------------------- statements
    def line(self, text):
        self.out.append('    ' * self.indent + text if text else '')

    def flush_comments(self, upto):
        for ln in self.comment_lines:
            if ln >= upto:
                break
            if ln in self.emitted_comments:
                continue
            self.emitted_comments.add(ln)
            for c in self.comments[ln]:
                if c.startswith('\x00'):
                    c = c[1:]
                    if self.out and self.out[-1].strip() and not self.out[-1].strip().startswith('//') \
                            and not self.out[-1].rstrip().endswith('{'):
                        self.out[-1] += '  ' + c
                        continue
                self.line(c)

    def stmt(self, n):
        if n.coord is not None:
            self.flush_comments(n.coord.line + 1 if isinstance(n, (c_ast.Case, c_ast.Default)) else n.coord.line)
        m = getattr(self, 's_' + type(n).__name__, None)
        if m is None:
            e = self.expr(n)
            self.line(e.code + ';')
            return
        m(n)

    def block(self, items):
        self.scopes.append({})
        for s in items or []:
            self.stmt(s)
        self.scopes.pop()

    def s_Compound(self, n):
        self.line('{')
        self.indent += 1
        self.block(n.block_items)
        if n.coord is not None:
            pass
        self.indent -= 1
        self.line('}')

    def s_EmptyStatement(self, n):
        pass

    def s_Decl(self, n):
        t = self.ctype(n.type)
        if t[0] == 'fn':
            raise Unsupported(f'local function declaration {n.name}')
        const = isinstance(n.type, c_ast.TypeDecl) and 'const' in n.type.quals
        name = rename(n.name)
        if n.init is not None and any(isinstance(c, c_ast.ID) and c.name == n.name for c in walk(n.init)):
            # C puts the declarator in scope in its own initializer (`T *p = f(sizeof(*p))`).
            self.scopes[-1][n.name] = t
        if const:
            self.const_vars.add(n.name)
        else:
            self.const_vars.discard(n.name)
        if t[0] == 'arr' and n.name in self.aligned and n.init is None:
            # DECLARE_ALIGNED(N, ...): the C alignment, through a wrapper.
            self.line(f'let mut {name}: Aligned{self.aligned[n.name]}<{rust_type(t)}> = core::mem::zeroed();')
            self.scopes[-1][n.name] = t
            self.aligned_locals.add(n.name)
            return
        if t[0] == 'arr':
            if n.init is None:
                self.line(f'let mut {name}: {rust_type(t)} = core::mem::zeroed();')
            else:
                init = self.initializer(n.init, t)
                self.line(f'let {"" if const else "mut "}{name}: {rust_type(t)} = {init};')
        else:
            if n.init is None:
                if t[0] in ('int', 'bool'):
                    self.line(f'let mut {name}: {rust_type(t)} = {"false" if t[0] == "bool" else 0};')
                elif address_taken(self.fn_body, n.name) or n.name in self.cfg.get('zero_init', []):
                    # Written through `&name` by a callee, or (zero_init) first written under a
                    # condition the borrow checker cannot follow: starts zeroed.
                    self.line(f'let mut {name}: {rust_type(t)} = core::mem::zeroed();')
                else:
                    self.line(f'let mut {name}: {rust_type(t)};')
            else:
                e = self.expr(n.init)
                e = self.cast(self.as_int(e) if t[0] == 'int' else e, t)
                self.line(f'let {"" if const else "mut "}{name}: {rust_type(t)} = {e.code};')
        self.scopes[-1][n.name] = t

    def initializer(self, init, t):
        if isinstance(init, c_ast.InitList):
            if t[0] != 'arr':
                raise Unsupported('struct initializer')
            items = [self.initializer(x, t[1]) for x in init.exprs]
            size = t[2]
            if size is not None and size.isdigit() and len(items) < int(size):
                items += [zero_value(t[1])] * (int(size) - len(items))
            return '[' + ', '.join(items) + ']'
        e = self.expr(init)
        return self.cast(self.as_int(e) if t[0] == 'int' else e, t).code

    def s_DeclList(self, n):
        for d in n.decls:
            self.s_Decl(d)

    def s_Assignment(self, n):
        if isinstance(n.rvalue, c_ast.Assignment) and n.op == '=':
            # `a = b = v`: the inner assignment first, then `a = b`.
            self.s_Assignment(n.rvalue)
            n = c_ast.Assignment('=', n.lvalue, n.rvalue.lvalue, n.coord)
        lhs = self.expr(n.lvalue)
        rhs = self.expr(n.rvalue)
        op = n.op
        if lhs.typ[0] == 'ptr' and op in ('+=', '-='):
            p = self.ptr_offset(lhs, self.as_int(rhs), op == '-=')
            self.line(f'{lhs.code} = {p.code};')
            return
        if op == '=':
            r = self.cast(self.as_int(rhs) if lhs.typ[0] == 'int' else rhs, lhs.typ)
            if getattr(lhs, 'unaligned', None):
                self.line(f'core::ptr::write_unaligned({lhs.unaligned}, {r.code});')
                return
            self.line(f'{lhs.code} = {r.code};')
            return
        if getattr(lhs, 'unaligned', None):
            raise Unsupported('compound assignment through an unaligned pointer')
        if op in ('<<=', '>>='):
            self.line(f'{lhs.code} {op} {self.as_int(rhs).code};')
            return
        r = self.cast(self.as_int(rhs), lhs.typ)
        self.line(f'{lhs.code} {op} {r.code};')

    def s_UnaryOp(self, n):
        if n.op in ('p++', '++', 'p--', '--'):
            e = self.expr(n.expr)
            d = '+=' if '+' in n.op else '-='
            if e.typ[0] == 'ptr':
                self.line(f'{e.code} = {e.at("postfix")}.offset({1 if d == "+=" else -1});')
            else:
                self.line(f'{e.code} {d} 1;')
            return
        self.line(self.expr(n).code + ';')

    def s_FuncCall(self, n):
        e = self.expr(n)
        self.line(e.code + ';')

    def s_Cast(self, n):
        e = self.expr(n)
        self.line(e.code + ';')

    def s_Return(self, n):
        if n.expr is None:
            self.line('return;')
            return
        e = self.expr(n.expr)
        e = self.cast(self.as_int(e) if self.fn_ret[0] == 'int' else e, self.fn_ret)
        self.line(f'return {e.code};')

    def s_If(self, n, prefix='if'):
        c = self.boolean(self.expr(n.cond))
        self.line(f'{prefix} {c.code} {{')
        self.body(n.iftrue)
        node = n.iffalse
        while isinstance(node, c_ast.If):
            if node.coord is not None:
                self.flush_comments(node.coord.line)
            c = self.boolean(self.expr(node.cond))
            self.line(f'}} else if {c.code} {{')
            self.body(node.iftrue)
            node = node.iffalse
        if node is not None:
            self.line('} else {')
            self.body(node)
        self.line('}')

    def body(self, n):
        self.indent += 1
        if isinstance(n, c_ast.Compound):
            self.block(n.block_items)
        else:
            self.block([n])
        self.indent -= 1

    def s_While(self, n):
        c = self.boolean(self.expr(n.cond))
        self.line(f'while {c.code} {{')
        self.loop_next.append(None)
        self.body(n.stmt)
        self.loop_next.pop()
        self.line('}')

    def s_DoWhile(self, n):
        self.line('loop {')
        self.loop_next.append('dowhile')
        self.body(n.stmt)
        self.loop_next.pop()
        self.indent += 1
        cond = n.cond
        if isinstance(cond, c_ast.UnaryOp) and cond.op in ('++', '--') and isinstance(cond.expr, c_ast.ID):
            self.s_UnaryOp(cond)
            cond = cond.expr
        elif isinstance(cond, c_ast.BinaryOp) and isinstance(cond.left, c_ast.Assignment):
            # `while ((p += n) < end)`: the assignment, then the test of the new value.
            self.s_Assignment(cond.left)
            cond = c_ast.BinaryOp(cond.op, cond.left.lvalue, cond.right, cond.coord)
        elif isinstance(cond, c_ast.BinaryOp) and isinstance(cond.left, c_ast.UnaryOp) and \
                cond.left.op in ('++', '--') and isinstance(cond.left.expr, c_ast.ID):
            # `while (++i < n)`: the increment, then the test of the new value.
            self.s_UnaryOp(cond.left)
            cond = c_ast.BinaryOp(cond.op, cond.left.expr, cond.right, cond.coord)
        c = self.boolean(self.expr(cond))
        self.line(f'if !{c.at("unary")} {{')
        self.line('    break;')
        self.line('}')
        self.indent -= 1
        self.line('}')

    def s_For(self, n):
        self.scopes.append({})
        simple = self.simple_for(n)
        if simple:
            var, start, stop, inclusive, step = simple
            rng = f'{start}..{"=" if inclusive else ""}{stop}'
            if step != 1:
                rng = f'({rng}).step_by({step})'
            self.line(f'for {rename(var)} in {rng} {{')
            self.loop_next.append(None)
            self.body(n.stmt)
            self.loop_next.pop()
            self.line('}')
        else:
            self.line('{')
            self.indent += 1
            if n.init is not None:
                self.stmt(n.init)
            c = self.boolean(self.expr(n.cond)) if n.cond is not None else E('true', ('bool',))
            self.line(f'while {c.code} {{')
            self.indent += 1
            # `continue` runs the increment first, as C's `for` does.
            self.loop_next.append(n.next)
            self.block(n.stmt.block_items if isinstance(n.stmt, c_ast.Compound) else [n.stmt])
            self.loop_next.pop()
            if n.next is not None:
                if isinstance(n.next, c_ast.ExprList):
                    for x in n.next.exprs:
                        self.stmt(x)
                else:
                    self.stmt(n.next)
            self.indent -= 1
            self.line('}')
            self.indent -= 1
            self.line('}')
        self.scopes.pop()

    def simple_for(self, n):
        init = n.init
        if isinstance(init, c_ast.DeclList) and len(init.decls) == 1:
            d = init.decls[0]
            var, t, start = d.name, self.ctype(d.type), d.init
            declared = True
        elif isinstance(init, c_ast.Assignment) and init.op == '=' and isinstance(init.lvalue, c_ast.ID):
            var, start = init.lvalue.name, init.rvalue
            t = self.lookup(var)
            declared = False
        else:
            return None
        if t[0] != 'int' or start is None:
            return None
        cond = n.cond
        if not (isinstance(cond, c_ast.BinaryOp) and cond.op in ('<', '<=') and isinstance(cond.left, c_ast.ID)
                and cond.left.name == var):
            return None
        nx = n.next
        if isinstance(nx, c_ast.UnaryOp) and nx.op in ('p++', '++') and isinstance(nx.expr, c_ast.ID) \
                and nx.expr.name == var:
            step = 1
        elif isinstance(nx, c_ast.Assignment) and nx.op == '+=' and isinstance(nx.lvalue, c_ast.ID) and \
                nx.lvalue.name == var and isinstance(nx.rvalue, c_ast.Constant):
            step = nx.rvalue.value if int(nx.rvalue.value, 0) != 1 else 1
        else:
            return None
        if assigns(n.stmt, var) or not declared:
            return None
        # The range bound is evaluated once: it must not change in the body.
        bound_ids = [c.name for c in walk(cond.right) if isinstance(c, c_ast.ID)]
        if any(assigns(n.stmt, b) for b in bound_ids) or any(isinstance(c, (c_ast.FuncCall, c_ast.UnaryOp))
                                                           and getattr(c, 'op', '') != '-' for c in walk(cond.right)
                                                           if not isinstance(c, c_ast.ID)):
            return None
        self.scopes[-1][var] = t
        s = self.cast(self.as_int(self.expr(start)), t)
        e = self.cast(self.as_int(self.expr(cond.right)), t)
        return var, s.at('+'), e.at('+'), cond.op == '<=', step

    def s_Switch(self, n):
        v = self.as_int(self.expr(n.cond))
        v = self.cast(v, promote(v.typ)) if v.typ[0] != 'lit' else v
        self.line(f'match {v.code} {{')
        self.indent += 1
        groups = []  # [labels, body]
        for item in n.stmt.block_items or []:
            if isinstance(item, (c_ast.Case, c_ast.Default)):
                labels, node = [], item
                while True:
                    labels.append(None if isinstance(node, c_ast.Default) else node.expr)
                    stmts = node.stmts or []
                    if stmts and isinstance(stmts[0], (c_ast.Case, c_ast.Default)):
                        if len(stmts) != 1:
                            raise Unsupported(f'case label with code at {n.coord}')
                        node = stmts[0]
                        continue
                    break
                if groups and not groups[-1][1]:
                    groups[-1][0] += labels  # `case A:` directly followed by `case B:`
                    groups[-1][1] = list(stmts)
                else:
                    groups.append([labels, list(stmts)])
            else:
                if not groups:
                    raise Unsupported(f'statement before the first case at {n.coord}')
                groups[-1][1].append(item)
        has_default = False
        for k, (labels, body) in enumerate(groups):
            last = k == len(groups) - 1
            body, terminated = strip_break(body)
            if not terminated and not last:
                raise Unsupported(f'switch fallthrough at {n.coord}')
            if contains_list(body, c_ast.Break):
                raise Unsupported(f'break inside a case body at {n.coord}')
            if None in labels:
                has_default = True
                pats = ['_']
            else:
                pats = [self.expr(l).code for l in labels]
            self.line(f'{" | ".join(pats)} => {{')
            self.indent += 1
            self.block(body)
            self.indent -= 1
            self.line('}')
        if not has_default:
            self.line('_ => {}')
        self.indent -= 1
        self.line('}')

    def s_Break(self, n):
        self.line('break;')

    def s_Continue(self, n):
        nxt = self.loop_next[-1] if self.loop_next else None
        if nxt == 'dowhile':
            raise Unsupported(f'continue in do-while at {n.coord}')
        if nxt is not None:
            for x in (nxt.exprs if isinstance(nxt, c_ast.ExprList) else [nxt]):
                self.stmt(x)
        self.line('continue;')

    # ---------------------------------------------------------------- top level
    def function(self, fd):
        decl = fd.decl
        name = decl.name
        sig = self.funcs[name]
        _, ret, params, _ = sig
        storage = decl.storage
        self.flush_comments(decl.coord.line)
        args = ', '.join(f'{"mut " if assigned_param(fd.body, p) else ""}{rename(p)}: {rust_type(t)}'
                         for p, t in params)
        r = '' if ret[0] == 'void' else f' -> {rust_type(ret)}'
        if name in self.exports:
            self.line('#[no_mangle]')
            self.line(f'#[target_feature(enable = "{self.feature}")]')
            self.line(f'pub unsafe extern "C-unwind" fn {name}({args}){r} {{')
        else:
            self.line('#[inline]' if 'static' in storage or 'inline' in (decl.funcspec or []) else '')
            if self.out[-1] == '':
                self.out.pop()
            self.line(f'#[target_feature(enable = "{self.feature}")]')
            if name in self.cfg.get('extern_c', []) or re.match(self.cfg.get('extern_c_regex', '$^'), name):
                self.out[-1] = self.out[-1]
                self.line(f'pub unsafe extern "C-unwind" fn {name}({args}){r} {{')
                self.fn_ret = ret
                self.fn_body = fd.body
                self.scopes = [{p: t for p, t in params}]
                self.indent += 1
                self.block(fd.body.block_items)
                self.indent -= 1
                self.line('}')
                self.line('')
                return
            vis = '' if ('static' in storage and name not in self.public and not self.cfg.get('public_all')) else 'pub '
            self.line(f'{vis}unsafe fn {name}({args}){r} {{')
        self.fn_ret = ret
        self.fn_body = fd.body
        self.scopes = [{p: t for p, t in params}]
        self.indent += 1
        self.block(fd.body.block_items)
        self.flush_comments(fd.body.end_coord.line if getattr(fd.body, 'end_coord', None) else decl.coord.line)
        self.indent -= 1
        self.line('}')
        self.line('')

    def global_decl(self, d):
        self.flush_comments(d.coord.line)
        t = self.ctype(d.type)
        if t[0] == 'fn':
            return
        if t[0] == 'arr' and t[2] is None and isinstance(d.init, c_ast.InitList):
            t = ('arr', t[1], str(len(d.init.exprs)))
        name = d.name
        self.globals[name] = t
        if d.init is None:
            raise Unsupported(f'global without initializer {name}')
        init = self.initializer(d.init, t)
        if name in self.aligned:
            self.aligned_globals.add(name)
            a = self.aligned[name]
            vis = 'pub ' if self.cfg.get('public_all') else ''
            self.line(f'{vis}static {name}: Aligned{a}<{rust_type(t)}> = Aligned{a}({init});')
            self.line('')
            return
        vis = 'pub ' if self.cfg.get('public_all') else ''
        self.line(f'{vis}static {name}: {rust_type(t)} = {init};')
        self.line('')


MACRO_RET = {'_mm_extract_epi16': ('int', 'i32'), '_mm_extract_epi32': ('int', 'i32'),
             '_mm_extract_epi8': ('int', 'i32'), '_mm_extract_epi64': ('int', 'i64'),
             '_mm256_extract_epi32': ('int', 'i32'), '_mm256_extract_epi16': ('int', 'i32'),
             '_mm256_extract_epi8': ('int', 'i32'), '_mm256_extracti128_si256': ('vec', '__m128i'),
             '_mm256_extractf128_si256': ('vec', '__m128i'), '_mm_shuffle_ps': ('vec', '__m128'),
             '_mm256_extractf128_ps': ('vec', '__m128')}
RUST_PTR = {'_mm_storeu_si32': ('ptr', ('int', 'u8'), False), '_mm_loadu_si32': ('ptr', ('int', 'u8'), True),
            '_mm_storeu_si16': ('ptr', ('int', 'u8'), False), '_mm_loadu_si16': ('ptr', ('int', 'u8'), True),
            '_mm_storeu_si64': ('ptr', ('int', 'u8'), False), '_mm_loadu_si64': ('ptr', ('int', 'u8'), True)}
RUST_REF = {'_mm256_broadcast_pd', '_mm256_broadcast_ps', '_mm256_broadcast_ss', '_mm_broadcast_ss',
            '_mm256_broadcast_sd'}
RENAMES = {'in': 'in_', 'type': 'type_', 'match': 'match_', 'loop': 'loop_', 'mod': 'mod_', 'ref': 'ref_',
           'fn': 'fn_', 'impl': 'impl_', 'move': 'move_', 'use': 'use_', 'where': 'where_', 'box': 'box_',
           'final': 'final_', 'override': 'override_'}


def rename(name):
    return RENAMES.get(name, name)


def zero_value(t):
    if t[0] in ('int', 'lit'):
        return '0'
    if t[0] == 'float':
        return '0.0'
    if t[0] == 'fnptr':
        return 'None'
    if t[0] == 'arr':
        return f'[{zero_value(t[1])}; {t[2]}]'
    raise Unsupported(f'zero of {t}')


def ftype_parse(text):
    """'i32' / 'u8' / '*const i32' / '__m128i' / '[i8; 12]' -> internal type."""
    text = text.strip()
    m = re.fullmatch(r'\[(.+);\s*(\d+)\]', text)
    if m:
        return ('arr', ftype_parse(m.group(1)), m.group(2))
    if text.startswith('*const '):
        return ('ptr', ftype_parse(text[7:]), True)
    if text.startswith('*mut '):
        return ('ptr', ftype_parse(text[5:]), False)
    if text in VECTORS:
        return ('vec', text)
    if text == '()':
        return ('void',)
    if text == 'bool':
        return ('bool',)
    if re.fullmatch(r'[iu](8|16|32|64|size)', text):
        return ('int', text)
    if text in ('f32', 'f64'):
        return ('float', text)
    return ('struct', text)


def contains(node, kind):
    if isinstance(node, kind):
        return True
    if isinstance(node, (c_ast.For, c_ast.While, c_ast.DoWhile, c_ast.Switch)) and kind in (c_ast.Continue,
                                                                                          c_ast.Break):
        return False
    return any(contains(c, kind) for _, c in node.children())


def strip_break(body):
    """(body without its terminating `break`s, whether it cannot fall through)."""
    if not body:
        return body, False
    last = body[-1]
    if isinstance(last, c_ast.Break):
        return body[:-1], True
    if isinstance(last, c_ast.Return):
        return body, True
    if isinstance(last, c_ast.Compound):
        inner, t = strip_break(list(last.block_items or []))
        return body[:-1] + [c_ast.Compound(inner, last.coord)], t
    if isinstance(last, c_ast.If) and last.iffalse is not None:
        a, ta = strip_break([last.iftrue])
        b, tb = strip_break([last.iffalse])
        if ta and tb:
            mk = lambda x: x[0] if len(x) == 1 else c_ast.Compound(x, last.coord)
            return body[:-1] + [c_ast.If(last.cond, mk(a) if a else c_ast.Compound([], last.coord),
                                         mk(b) if b else c_ast.Compound([], last.coord), last.coord)], True
    return body, False


def contains_list(nodes, kind):
    return any(contains(n, kind) for n in nodes or [])


def assigns(node, var):
    for _, c in node.children():
        if isinstance(c, c_ast.Assignment) and isinstance(c.lvalue, c_ast.ID) and c.lvalue.name == var:
            return True
        if isinstance(c, c_ast.UnaryOp) and c.op in ('p++', '++', 'p--', '--') and \
                isinstance(c.expr, c_ast.ID) and c.expr.name == var:
            return True
        if isinstance(c, c_ast.UnaryOp) and c.op == '&' and isinstance(c.expr, c_ast.ID) and c.expr.name == var:
            return True
        if assigns(c, var):
            return True
    return False


def walk(node):
    yield node
    for _, c in node.children():
        yield from walk(c)


def address_taken(node, var):
    if node is None:
        return False
    for _, c in node.children():
        if isinstance(c, c_ast.UnaryOp) and c.op == '&' and isinstance(c.expr, c_ast.ID) and c.expr.name == var:
            return True
        if address_taken(c, var):
            return True
    return False


def assigned_param(body, name):
    return assigns(body, name)


CONFIG = {'CONFIG_REALTIME_ONLY': 0, 'CONFIG_AV1_DECODER': 1, 'CONFIG_AV1_ENCODER': 1,
          'CONFIG_AV1_HIGHBITDEPTH': 1, 'CONFIG_EXCLUDE_SIMD_MISMATCH': 0, 'AOM_ARCH_X86_64': 1,
          'AOM_ARCH_X86': 0, 'HAVE_SSE2': 1, 'HAVE_SSSE3': 1, 'HAVE_SSE4_1': 1, 'HAVE_AVX2': 1,
          'HAVE_AVX512': 0, 'CONFIG_MULTITHREAD': 1, '__cplusplus': 0, 'NDEBUG': 1}


DEFINES = {}


def eval_cond(kind, cond):
    if kind == 'ifdef':
        return cond in CONFIG and cond != '__cplusplus' or cond.startswith('AOM_')
    if kind == 'ifndef':
        # Include guards are taken.
        return True if cond.startswith('AOM_') else cond not in CONFIG
    expr = re.sub(r'defined\s*\(?\s*(\w+)\s*\)?', lambda m: '1' if m.group(1) in CONFIG else '0', cond)
    def name(m):
        n = m.group(0)
        if n.isdigit():
            return n
        if n not in CONFIG:
            raise SystemExit(f'#if with unknown name {n}: {cond}')
        return str(CONFIG[n])
    expr = re.sub(r'\b[A-Za-z_]\w*\b', name, expr)
    expr = expr.replace('||', ' or ').replace('&&', ' and ').replace('!', ' not ').replace(' not =', '!=')
    return bool(eval(expr))


def clean_source(text):
    """Blank comments and preprocessor lines, keeping line numbers. Returns (clean, comments by line)."""
    comments = {}
    out = []
    i, line = 0, 1
    n = len(text)
    buf = []
    while i < n:
        if text.startswith('//', i):
            j = text.find('\n', i)
            j = n if j < 0 else j
            start = text.rfind('\n', 0, i) + 1
            body = text[i:j].rstrip()
            if text[start:i].strip():
                body = '\x00' + body  # trailing comment
            comments.setdefault(line, []).append(body)
            buf.append(' ' * (j - i))
            i = j
        elif text.startswith('/*', i):
            j = text.index('*/', i) + 2
            chunk = text[i:j]
            lines = chunk.split('\n')
            if lines[0].startswith('/*') and len(lines) == 1:
                comments.setdefault(line, []).append('// ' + chunk[2:-2].strip())
            else:
                raw = [l.strip() for l in lines]
                raw[0] = raw[0][2:].lstrip('*') if raw[0].startswith('/*') else raw[0]
                raw[-1] = raw[-1][:-2].rstrip('*') if raw[-1].endswith('*/') else raw[-1]
                body = [l.lstrip('*').rstrip() for l in raw]
                if body and not body[0].strip():
                    body = body[1:]
                if body and not body[-1].strip():
                    body = body[:-1]
                comments.setdefault(line, []).extend('//' + (' ' + b.strip() if b.strip() else '')
                                                     for b in body if True)
            buf.append(''.join('\n' if ch == '\n' else ' ' for ch in chunk))
            line += chunk.count('\n')
            i = j
        elif text.startswith('"', i):
            j = i + 1
            while text[j] != '"':
                j += 2 if text[j] == '\\' else 1
            buf.append(text[i:j + 1])
            i = j + 1
        else:
            ch = text[i]
            if ch == '\n':
                line += 1
            buf.append(ch)
            i += 1
    clean = ''.join(buf)
    lines = clean.split('\n')
    pre = []
    k = 0
    while k < len(lines):
        if lines[k].lstrip().startswith('#'):
            start = k
            while lines[k].rstrip().endswith('\\'):
                lines[k] = ''
                k += 1
            pre.append((start + 1, ' '.join(lines[start:k + 1])))
            lines[k] = ''
        k += 1
    # Conditional compilation with the reference configuration.
    stack = []  # (taking, any_taken)
    raw = text.split('\n')
    for k, l in enumerate(raw):
        d = l.strip()
        m = re.match(r'#\s*(if|ifdef|ifndef|elif|else|endif)\b(.*)', d)
        active = all(t for t, _ in stack)
        if m:
            kind, cond = m.group(1), re.sub(r'//.*|/\*.*?\*/', '', m.group(2)).strip()
            if kind in ('if', 'ifdef', 'ifndef'):
                v = eval_cond(kind, cond)
                stack.append((v, v))
            elif kind == 'elif':
                t, anyt = stack.pop()
                v = (not anyt) and eval_cond('if', cond)
                stack.append((v, anyt or v))
            elif kind == 'else':
                t, anyt = stack.pop()
                stack.append((not anyt, True))
            else:
                stack.pop()
            lines[k] = ''
        elif not active:
            lines[k] = ''
        elif d.startswith('#') and re.match(r'#\s*define\s+(\w+)(\s|$)', d):
            md = re.match(r'#\s*define\s+(\w+)\s*(.*)', d)
            if not md.group(1).startswith('AOM_'):
                DEFINES[md.group(1)] = (k + 1, re.sub(r'//.*|/\*.*?\*/', '', md.group(2)).strip())
    k = 0
    while k < len(lines):
        if text.split('\n')[k].strip().startswith('#ifdef __cplusplus'):
            j = k
            while not text.split('\n')[j].strip().startswith('#endif'):
                lines[j] = ''
                j += 1
            lines[j] = ''
            k = j
        k += 1
    return '\n'.join(lines), comments, pre


def declare_aligned(text):
    """(alignment, type, declarator, start, end) of every DECLARE_ALIGNED(n, type, decl),
    with balanced parentheses in the declarator."""
    out = []
    for m in re.finditer(r'DECLARE_ALIGNED\(\s*(\d+)\s*,\s*', text):
        depth, k = 1, m.end()
        while depth:
            depth += {'(': 1, ')': -1}.get(text[k], 0)
            k += 1
        inner = text[m.end():k - 1]
        typ, decl = inner.split(',', 1) if ',' in inner else (inner, '')
        out.append((int(m.group(1)), typ.strip(), decl.strip().split('[')[0].strip(), m.start(), k))
    return out


def strip_declare_aligned(text, keep_lines=False):
    """Replace DECLARE_ALIGNED(n, type, decl) by `type decl`. With keep_lines
    the line breaks inside the macro call follow the declaration, so that the
    source keeps its line numbers and every comment its place."""
    for align, typ, name, start, end in reversed(declare_aligned(text)):
        inner = text[text.index('(', start) + 1:end - 1]
        typ, decl = inner.split(',', 1)[1].split(',', 1)
        breaks = '\n' * text[start:end].count('\n') if keep_lines else ''
        text = text[:start] + typ.strip() + ' ' + decl.strip() + breaks + text[end:]
    return text


def macro_blocks(text, pattern):
    """The `#define NAME...` blocks (with continuation lines) whose NAME matches."""
    out, lines, k = [], text.split('\n'), 0
    while k < len(lines):
        m = re.match(r'\s*#\s*define\s+(\w+)', lines[k])
        start = k
        while lines[k].rstrip().endswith('\\'):
            k += 1
        if m and re.match(pattern, m.group(1)):
            out.append((start, k))
        k += 1
    return out


def cpp_expand(text, spec):
    """Expand the function-generating macros of `spec['headers']` (and the
    source's own matching macros) with the C preprocessor; everything else of
    the source is kept, #include lines are dropped, and the reference
    configuration (CONFIG) is defined. Used for the CfL wrapper macros."""
    import subprocess
    defs = []
    for header in spec['headers']:
        h = (ROOT / header).read_text().split('\n')
        keep = set()
        for a, b in macro_blocks('\n'.join(h), spec['macros']):
            keep |= set(range(a, b + 1))
        # The conditional directives too, so that a macro defined twice under
        # `#if CONFIG_...` / `#else` takes the reference configuration's form.
        keep |= {k for k, l in enumerate(h) if re.match(r'\s*#\s*(if|ifdef|ifndef|elif|else|endif)\b', l)}
        defs.append('\n'.join(h[k] for k in sorted(keep)))
    src = text.split('\n')
    src = [l if not re.match(r'\s*#\s*include\b', l) else '' for l in src]
    full = '\n'.join(defs) + '\n' + '\n'.join(src)
    args = (['cc', '-E', '-P', '-C', '-x', 'c', '-'] + [f'-D{k}={v}' for k, v in CONFIG.items() if k != '__cplusplus']
            + [f'-D{d}' for d in spec.get('defines', [])])
    r = subprocess.run(args, input=full, capture_output=True, text=True)
    if r.returncode:
        raise SystemExit(f'cpp failed: {r.stderr[:2000]}')
    return r.stdout


ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent


def translate(config_path, work=None):
    """Translate one config; returns the Rust text (also written to WORK/output)."""
    DEFINES.clear()
    config_path = Path(config_path)
    if not config_path.is_absolute() and not config_path.exists():
        config_path = HERE / config_path
    cfg = json.loads(config_path.read_text())
    base = HERE
    src_path = ROOT / cfg['source']
    text = src_path.read_text()
    if cfg.get('cpp'):
        text = cpp_expand(text, cfg['cpp'])
    for a, b in cfg.get('replace', []):
        if a not in text:
            raise SystemExit(f'replace target not found: {a[:60]}')
        text = text.replace(a, b)
    clean, comments, pre = clean_source(text)
    clean = re.sub(r'\bAOM_FORCE_INLINE\b', 'inline', clean)
    object_consts = []
    for dname, (dline, value) in DEFINES.items():
        if dname in cfg.get('macros', {}):
            continue
        if value in ('', 'restrict', '__restrict'):
            clean = re.sub(r'\b' + dname + r'\b', '', clean)
        elif re.fullmatch(r'[\w\s\(\)\+\-\*<>]+', value) and not re.match(r'\w+\(', value):
            object_consts.append((dname, value, dline))
            cfg.setdefault('consts', {})[dname] = 'i32'
        else:
            if os.environ.get('C2R_VERBOSE'):
                print(f'note: function-like or complex #define {dname} {value}', file=sys.stderr)
    aligned = {name: align for align, _, name, _, _ in declare_aligned(clean)}
    clean = strip_declare_aligned(clean, cfg.get('keep_lines', False))
    if cfg.get('only'):
        # File-scope invocations of function-generating macros (AOM_VAR_*, VAR_FN,
        # ...) define functions outside `only`; they are blanked before parsing.
        clean = re.sub(r'(?m)^[A-Z][A-Z0-9_]*\([^;{}\n]*\)[ \t]*;?[ \t]*$', '', clean)
    for skip in cfg.get('skip_ranges', []):
        lines = clean.split('\n')
        for k in range(skip[0] - 1, skip[1]):
            lines[k] = ''
        clean = '\n'.join(lines)
    prelude = (base / cfg['prelude']).read_text()
    full = '# 1 "prelude"\n' + prelude + f'\n# 1 "{src_path.name}"\n' + clean
    ast = c_parser.CParser().parse(full, filename=src_path.name)
    p = Printer(cfg, comments, text.split('\n'))
    p.aligned = aligned
    p.emitted_comments.update(l for l in p.comment_lines if l < cfg.get('comment_start', 0))
    for d in [x for x in pre if not re.match(r'#\s*(include|ifndef AOM_|define AOM_|endif\s*$|endif\s*//)', x[1])]:
        if os.environ.get('C2R_VERBOSE'):
            print(f'note: preprocessor line {d[0]}: {d[1]}', file=sys.stderr)
    # First pass: typedefs, prototypes, signatures.
    for ext in ast.ext:
        if isinstance(ext, c_ast.Typedef):
            t = ext.type
            if isinstance(t, c_ast.PtrDecl) and isinstance(t.type, c_ast.FuncDecl):
                p.fnptrs[ext.name] = p.functype(t.type, ext.name)
            elif isinstance(t, c_ast.TypeDecl) and isinstance(t.type, c_ast.Struct):
                pass
            continue
        decl = ext.decl if isinstance(ext, c_ast.FuncDef) else ext
        if isinstance(decl, c_ast.Decl) and isinstance(decl.type, c_ast.FuncDecl):
            p.funcs[decl.name] = p.functype(decl.type, decl.name)
        elif isinstance(decl, c_ast.Decl) and 'extern' in decl.storage:
            p.globals[decl.name] = p.ctype(decl.type)
    for name, sig in p.fnptrs.items():
        p.line(f'// {name}')
    p.out = []
    skip = set(cfg.get('skip', []))
    if cfg.get('only'):
        # Keep the named functions and everything of this file they reach.
        defs = {}
        for ext in ast.ext:
            if ext.coord is not None and ext.coord.file == src_path.name:
                if isinstance(ext, c_ast.FuncDef):
                    defs[ext.decl.name] = ext
                elif isinstance(ext, c_ast.Decl) and ext.name:
                    defs[ext.name] = ext
        keep, todo = set(), list(cfg['only'])
        while todo:
            name = todo.pop()
            if name in keep or name not in defs:
                continue
            keep.add(name)
            todo += [c.name for c in walk(defs[name]) if isinstance(c, c_ast.ID)]
        skip |= set(defs) - keep
    failed = []
    for ext in ast.ext:
        if ext.coord is None or ext.coord.file != src_path.name:
            continue
        if isinstance(ext, c_ast.FuncDef):
            if ext.decl.name in skip:
                p.emitted_comments.update(l for l in p.comment_lines if l < ext.decl.coord.line)
                continue
            try:
                p.function(ext)
            except (Unsupported, KeyError, IndexError, TypeError, AttributeError, ValueError) as err:
                print(f'error in {ext.decl.name}: {type(err).__name__} {err}', file=sys.stderr)
                failed.append(ext.decl.name)
                p.line(f'// UNSUPPORTED {ext.decl.name}: {err}')
                p.indent = 0
        elif isinstance(ext, c_ast.Decl):
            if isinstance(ext.type, c_ast.FuncDecl):
                continue
            if ext.name in skip:
                continue
            p.global_decl(ext)
        elif isinstance(ext, c_ast.Typedef):
            continue
    p.flush_comments(10 ** 9)
    if object_consts:
        p.out[:0] = [f'const {n}: i32 = {v};' for n, v, _ in object_consts] + ['']
    if failed:
        raise SystemExit(f'{config_path.name}: {len(failed)} functions not translated: {failed}')
    text = '\n'.join(p.out).rstrip() + '\n'
    work = Path(work) if work else ROOT / 'build/x86_c2r'
    work.mkdir(parents=True, exist_ok=True)
    (work / cfg['output']).write_text(text)
    return text


def main():
    translate(sys.argv[1], sys.argv[2] if len(sys.argv) > 2 else None)


if __name__ == '__main__':
    main()
