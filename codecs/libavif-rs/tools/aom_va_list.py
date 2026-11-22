"""Spell C `va_list` one way in the one-source libaom translation.

C2Rust prints the C `va_list` of the two targets differently, although both
spellings name the same Rust type, `core::ffi::VaList`, the portable C va_list
on every ABI:

- arm64-apple-macosx: `pub type __builtin_va_list = *mut core::ffi::c_char;`,
  an alias `pub type va_list<'a> = core::ffi::VaList<'a>;` and `va_list` in
  signatures; `va_start` becomes `let ap = args.clone();`.
- x86_64-unknown-linux-gnu: `pub type __builtin_va_list = [__va_list_tag; 1];`
  with the SysV `__va_list_tag` record, signatures spelled
  `::core::ffi::VaList` or `core::ffi::VaList<'_>`, and `va_start` split into
  `let mut ap: ::core::ffi::VaList;` followed by `ap = args.clone();`.

`normalize()` rewrites one unit to a single spelling:

- every type use is `core::ffi::VaList<'_>`. That is the type the target
  aliases resolve to, written where it is used, so no alias has to be placed
  at the same spot in both units; `'_` is the lifetime both spellings already
  elide (parameters, function pointer types, casts, `let` annotations), and
  the unprefixed `core::` path is how the rest of the unit names `core::ffi`.
  The target records (`__builtin_va_list`, `__va_list_tag`) and the
  `va_list` alias are removed when nothing else in the unit names them.
- `va_start` (and `va_copy`) is `let mut ap: core::ffi::VaList<'_> = args.clone();`
  at the position of the `clone()`, the statement that copies the list on
  both targets. The Linux declaration is merged only when the assignment is
  the very next statement, so no copy point moves; the explicit type keeps
  the binding's type independent of inference, and `mut` is the Linux form
  (the crate allows `unused_mut`).
- the printer wraps a line at 100 columns, so a statement whose length
  changed with the spelling (or, once, a call passing the list) can break
  differently. Exactly those parenthesized groups get the printer's layout:
  a one-line group that holds the list type and is longer than 100 columns is
  broken one argument per line, and a broken group that holds the list type
  or a list variable is joined when the joined line fits in 100 columns.

Only code is rewritten (comments, strings and character literals are skipped),
every argument, call and copy stays where it was, and the result is a fixed
point: `normalize(normalize(t)) == normalize(t)`.

usage: python3 -I tools/aom_va_list.py MACOS_DIR LINUX_DIR
Normalizes both trees in memory and prints, per unit, the top-level items that
still differ and mention the list (target 0) and the other differing items.
"""
import collections
import functools
import re
import sys
from pathlib import Path

CANON = "core::ffi::VaList<'_>"
WIDTH = 100
VA_MENTION = re.compile(r"\b(?:va_list|VaList|__va_list_tag|__builtin_va_list)\b")
RECORDS = ('va_list', '__builtin_va_list', '__va_list_tag')

_NEXT = re.compile(r'//|/\*|"|\'')
_RAW_PREFIX = re.compile(r'(?<![\w])b?r(#*)$')
_CHAR = re.compile(r"'(?:\\(?:u\{[0-9a-fA-F]+\}|x[0-9a-fA-F]{2}|.)|[^\\'\n])'")
_NOT_NEWLINE = re.compile(r'[^\n]')


@functools.lru_cache(maxsize=32)
def mask(text):
    """Text with comments and literal contents blanked (same length, newlines kept)."""
    out, i, n = [], 0, len(text)

    def keep(a, b):
        out.append(text[a:b])

    def blank(a, b):
        out.append(_NOT_NEWLINE.sub(' ', text[a:b]))

    while True:
        m = _NEXT.search(text, i)
        if not m:
            keep(i, n)
            break
        k = m.start()
        keep(i, k)
        token = m.group()
        if token == '//':
            j = text.find('\n', k)
            j = n if j < 0 else j
            blank(k, j)
        elif token == '/*':
            depth, j = 1, k + 2
            while j < n and depth:
                if text.startswith('/*', j):
                    depth, j = depth + 1, j + 2
                elif text.startswith('*/', j):
                    depth, j = depth - 1, j + 2
                else:
                    j += 1
            blank(k, j)
        elif token == '"':
            raw = _RAW_PREFIX.search(text, max(0, k - 66), k)
            if raw:
                close = '"' + raw.group(1)
                j = text.find(close, k + 1)
                j = n if j < 0 else j
            else:
                close, j = '"', k + 1
                while j < n and text[j] != '"':
                    j += 2 if text[j] == '\\' else 1
                j = min(j, n)
            keep(k, k + 1)
            blank(k + 1, j)
            keep(j, min(n, j + len(close)))
            j = min(n, j + len(close))
        else:
            c = _CHAR.match(text, k)
            if c:
                keep(k, k + 1)
                blank(k + 1, c.end() - 1)
                keep(c.end() - 1, c.end())
                j = c.end()
            else:  # a lifetime
                keep(k, k + 1)
                j = k + 1
        i = j
    result = ''.join(out)
    assert len(result) == n
    return result


def _sub(text, pattern, repl, flags=0):
    """re.sub over the code of `text` only; `repl` gets the match on the masked text."""
    masked = mask(text)
    parts, last, count = [], 0, 0
    for m in re.finditer(pattern, masked, flags):
        new = repl(m)
        if new is None:
            continue
        parts.append(text[last:m.start()])
        parts.append(new)
        last = m.end()
        count += 1
    parts.append(text[last:])
    return ''.join(parts), count


# ------------------------------------------------------------------ items
def split_items(text):
    """[(trivia, item)] covering `text`; trivia is whitespace and comments."""
    masked = mask(text)
    n, i, chunks = len(text), 0, []
    while i < n:
        j = i
        # comments are blanked in the mask: trivia ends at the first code char
        while j < n and masked[j].isspace():
            j += 1
        if j >= n:
            chunks.append((text[i:], ''))
            break
        depth, k = 0, j
        while k < n:
            c = masked[k]
            if c in '([{':
                depth += 1
            elif c in ')]}':
                depth -= 1
                if depth == 0 and c == '}':
                    m = re.compile(r'\s*;').match(masked, k + 1)
                    k = m.end() if m else k + 1
                    break
            elif c == ';' and depth == 0:
                k += 1
                break
            k += 1
        else:
            k = n
        # attributes `#[..]` are balanced with no `;` or `}` at depth zero,
        # so they are scanned together with the item they annotate
        chunks.append((text[i:j], text[j:k]))
        i = k
    return chunks


def join_items(chunks):
    return ''.join(trivia + item for trivia, item in chunks)


def defined_name(item):
    body = re.sub(r'^(?:\s*#\[[^\]]*\])*\s*', '', mask(item))
    m = re.match(r'(?:pub(?:\([^)]*\))?\s+)?(?:type|struct|union)\s+(\w+)', body)
    return m.group(1) if m else None


# ---------------------------------------------------------------- spelling
_PREFIX = r'(?<![\w:.])'


def _spell(text):
    """Every type use of the list as CANON; returns (text, count)."""
    total = 0
    # `::core::ffi::VaList<..>` -> `core::ffi::VaList<..>`
    text, c = _sub(text, _PREFIX + r'::core::ffi::VaList(?=\s*<)', lambda m: 'core::ffi::VaList')
    total += c
    # bare `core::ffi::VaList` / `::core::ffi::VaList` -> CANON
    text, c = _sub(text, _PREFIX + r'(?:::)?core::ffi::VaList\b(?!\s*<)', lambda m: CANON)
    total += c

    def alias(m):
        # the alias definition itself stays until it is removed as unused
        if re.search(r'\btype\s+$', m.string[max(0, m.start() - 16):m.start()]):
            return None
        return 'core::ffi::VaList' + m.group(1) if m.group(1) else CANON

    text, c = _sub(text, _PREFIX + r'va_list\b(\s*<)?', alias)
    total += c
    return text, total


def _list_names(item):
    """Variables of the item that hold a list: `...` parameters and CANON bindings."""
    masked = mask(item)
    names = set(re.findall(r'\b(\w+)\s*:\s*\.\.\.', masked))
    names |= set(re.findall(r'\b(\w+)\s*:\s*' + re.escape(CANON), masked))
    return names


def _starts(item, names):
    """`va_start`/`va_copy` as one statement at the position of the copy."""
    total = 0
    pattern = (r"^([ \t]*)let mut (\w+): " + re.escape(CANON) + r";\n\1\2 = (\w+)\.clone\(\);$")
    item, c = _sub(item, pattern, lambda m: (
        f'{m.group(1)}let mut {m.group(2)}: {CANON} = {m.group(3)}.clone();'
        if m.group(3) in names else None), re.M)
    total += c
    pattern = r'^([ \t]*)let (?:mut )?(\w+) = (\w+)\.clone\(\);$'
    item, c = _sub(item, pattern, lambda m: (
        f'{m.group(1)}let mut {m.group(2)}: {CANON} = {m.group(3)}.clone();'
        if m.group(3) in names else None), re.M)
    total += c
    return item, total


# ------------------------------------------------------------------ layout
def _top_level_commas(masked, a, b):
    """Positions of commas at depth zero in masked[a:b], or None if unbalanced."""
    depth, commas = 0, []
    for k in range(a, b):
        c = masked[k]
        if c in '([{':
            depth += 1
        elif c in ')]}':
            depth -= 1
        elif c == '<' and masked[k + 1:k + 2] not in ('=', '<') and masked[k - 1:k] not in ('<',):
            depth += 1
        elif c == '>' and masked[k - 1:k] not in ('-', '=', '>'):
            depth -= 1
        elif c == ',' and depth == 0:
            commas.append(k)
        if depth < 0:
            return None
    return commas if depth == 0 else None


def _matching(masked, k):
    depth = 0
    for j in range(k, len(masked)):
        if masked[j] in '([{':
            depth += 1
        elif masked[j] in ')]}':
            depth -= 1
            if depth == 0:
                return j if masked[j] == ')' else None
    return None


def _wrap(item):
    """Break a one-line group that holds CANON and overflows, one argument per line."""
    lines, count = item.split('\n'), 0
    masked_lines = mask(item).split('\n')
    out = []
    for line, masked in zip(lines, masked_lines):
        if len(line) <= WIDTH or CANON not in masked:
            out.append(line)
            continue
        indent = line[:len(line) - len(line.lstrip())]
        done = False
        for k, c in enumerate(masked):
            if c != '(':
                continue
            end = _matching(masked, k)
            if end is None or CANON not in masked[k:end]:
                continue
            inner = masked[k + 1:end]
            if not inner.strip():
                continue
            commas = _top_level_commas(masked, k + 1, end)
            if commas is None:
                continue
            bounds = [k + 1] + [p + 1 for p in commas] + [end]
            args = [line[bounds[x]:bounds[x + 1]].strip().rstrip(',').strip()
                    for x in range(len(bounds) - 1)]
            args = [a for a in args if a]
            new = [line[:k + 1]] + [f'{indent}    {a},' for a in args] + [indent + line[end:]]
            if all(len(x) <= WIDTH for x in new):
                out.extend(new)
                count += 1
                done = True
            break
        if not done:
            out.append(line)
    return '\n'.join(out), count


def _join(item, names):
    """Join a broken group that holds the list when it fits on one line."""
    lines, count = item.split('\n'), 0
    masked_lines = mask(item).split('\n')
    word = re.compile(r'\b(?:' + '|'.join(map(re.escape, sorted(names))) + r')\b') if names else None
    out, i = [], 0
    while i < len(lines):
        line, masked = lines[i], masked_lines[i]
        if masked.rstrip().endswith('(') and masked.rstrip() == masked:
            indent = len(line) - len(line.lstrip())
            j, args, ok = i + 1, [], True
            while j < len(lines):
                ml = masked_lines[j]
                ind = len(ml) - len(ml.lstrip())
                if ind == indent + 4 and ml.endswith(',') and _top_level_commas(ml, 0, len(ml) - 1) == []:
                    args.append(lines[j].strip()[:-1])
                    j += 1
                    continue
                if ind == indent and ml.lstrip().startswith(')') and args:
                    break
                ok = False
                break
            else:
                ok = False
            if ok:
                group = '\n'.join(masked_lines[i + 1:j])
                holds = CANON in group or (word is not None and any(
                    word.fullmatch(mask(a).strip()) for a in args))
                joined = line + ', '.join(args) + lines[j].lstrip()
                if holds and len(joined) <= WIDTH:
                    out.append(joined)
                    count += 1
                    i = j + 1
                    continue
        out.append(line)
        i += 1
    return '\n'.join(out), count


# -------------------------------------------------------------- normalize
def normalize(text):
    """One unit in the single list spelling; returns (text, changed items)."""
    chunks = split_items(text)
    changed = 0
    result = []
    for trivia, item in chunks:
        if not VA_MENTION.search(mask(item)) and not re.search(r':\s*\.\.\.', mask(item)):
            result.append((trivia, item))
            continue
        new, _ = _spell(item)
        names = _list_names(new)
        new, _ = _starts(new, names)
        new, _ = _wrap(new)
        new, _ = _join(new, names)
        changed += new != item
        result.append((trivia, new))
    # target records and the alias, when nothing else names them
    removed = True
    while removed:
        removed = False
        for index, (trivia, item) in enumerate(result):
            name = defined_name(item) if item else None
            if name not in RECORDS:
                continue
            rest = mask(join_items(result[:index] + result[index + 1:]))
            if re.search(r'\b' + re.escape(name) + r'\b', rest):
                continue
            if trivia.strip():
                result[index] = (trivia, '')
            else:
                del result[index]
            changed += 1
            removed = True
            break
    return join_items(result), changed


# ------------------------------------------------------------ comparison
def _keyed(text):
    """{(kind, name, nth): item text} of the top-level items, extern blocks expanded."""
    out, seen = {}, collections.Counter()

    def add(item, where):
        masked = re.sub(r'^(?:\s*#\[[^\]]*\])*\s*', '', mask(item))
        m = re.search(r'\b(fn|struct|union|type|static(?:\s+mut)?|const|mod|impl)\s+(\w+)', masked)
        key = (where + (m.group(1) if m else 'other'), m.group(2) if m else masked[:40])
        seen[key] += 1
        out[key + (seen[key],)] = item

    for _trivia, item in split_items(text):
        masked = mask(item)
        m = re.match(r'(?:\s*#\[[^\]]*\])*\s*(?:unsafe\s+)?extern\s+"[^"]*"\s*\{', masked)
        if m:
            inner = item[m.end():item.rstrip().rfind('}')]
            for _t, x in split_items(inner):
                add(x, 'extern ')
        elif item:
            add(item, '')
    return out


def _differences(a, b):
    ka, kb = _keyed(a), _keyed(b)
    va = other = 0
    for key in sorted(set(ka) | set(kb), key=str):
        x, y = ka.get(key), kb.get(key)
        if x == y:
            continue
        if any(t is not None and VA_MENTION.search(mask(t)) for t in (x, y)):
            va += 1
        else:
            other += 1
    return va, other


def main(argv=None):
    argv = sys.argv[1:] if argv is None else argv
    if len(argv) != 2:
        raise SystemExit(__doc__.rsplit('usage:', 1)[1].split('\n')[0].join(['usage:', '']))
    macos, linux = map(Path, argv)
    units = sorted(p.relative_to(macos) for p in macos.rglob('*.rs'))
    if sorted(p.relative_to(linux) for p in linux.rglob('*.rs')) != units:
        raise SystemExit('The trees do not hold the same units')
    totals = collections.Counter()
    for unit in units:
        a, b = (macos / unit).read_text(), (linux / unit).read_text()
        na, ca = normalize(a)
        nb, cb = normalize(b)
        if normalize(na)[0] != na or normalize(nb)[0] != nb:
            raise SystemExit(f'{unit}: normalize is not idempotent')
        before = _differences(a, b)
        after = _differences(na, nb)
        totals.update({'va_before': before[0], 'other_before': before[1],
                       'va_after': after[0], 'other_after': after[1],
                       'changed_macos': ca, 'changed_linux': cb})
        if before != (0, 0) or ca or cb:
            print(f'{unit}: va_list {before[0]} -> {after[0]}, other {before[1]} -> {after[1]}'
                  f' (items rewritten: macos {ca}, linux {cb})')
    print(f'total: va_list {totals["va_before"]} -> {totals["va_after"]}, '
          f'other {totals["other_before"]} -> {totals["other_after"]}, '
          f'items rewritten: macos {totals["changed_macos"]}, linux {totals["changed_linux"]}, '
          f'units {len(units)}')
    return 1 if totals['va_after'] else 0


if __name__ == '__main__':
    sys.exit(main())
