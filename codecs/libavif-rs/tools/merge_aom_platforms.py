#!/usr/bin/env python3
"""Merge the per-target one-source libaom trees into one Rust tree (route B).

`tools/aom_one_source.py` translates the unchanged C once per target. The
macOS ARM64 and GNU Linux x86-64 trees then differ only in target facts (CPU
dispatch, va_list spelling and a few small items). This tool merges them into
one tree, unit by unit:

* every unit is split into top-level items in source order (functions,
  statics, consts, records with their attributes and comments, type aliases,
  `use`, impl blocks, macros, and each declaration of an `extern` block);
  comments and attributes directly before an item belong to it, the rest of
  the item's last line (a trailing comment) too;
* the two item sequences are aligned on (kind, name) with difflib, so the
  output keeps C2Rust's order;
* an item with the same text on both sides (exactly, or differing only in
  whitespace, line wrapping and trailing commas; then the macOS text is kept)
  is emitted once; an item whose text differs is emitted twice, under
  `#[cfg(target_arch = "aarch64")]` (macOS text) and then
  `#[cfg(target_arch = "x86_64")]` (Linux text); an item on one side only gets
  that side's attribute. In an `extern` block present on both sides the
  attribute goes on the foreign item and shared declarations stay in the
  block; an `extern` block present on one side only is attributed as a whole,
  so no target ever sees an empty block that was not in its input.

If `tools/aom_va_list.py` is importable, its `normalize(text) -> (text,
count)` is applied to both inputs before merging (the merge and `--verify`
then work on the normalized texts); otherwise that step is skipped with a note.

`--verify` re-reads the written tree and projects it back to each target
(drops the items attributed to the other target and removes the attributes
this tool added). The macOS projection must equal its input byte for byte; the
Linux projection must equal its input item by item, exactly or, for the shared
items whose macOS text was kept, after whitespace normalization. Any other
difference exits 1.

    python3 -I tools/merge_aom_platforms.py --macos DIR --linux DIR \\
        --output DIR [--report FILE] [--modules] [--verify]
"""
import argparse
from collections import Counter
import difflib
import gzip
import io
import json
from pathlib import Path
import re
import sys
import tarfile

ARCHES = {'macos': 'aarch64', 'linux': 'x86_64'}
MARK = '#[cfg(target_arch = "{}")]'
MARK_PREFIX = '#[cfg(target_arch = '
MARKS = {arch: MARK.format(arch) for arch in ARCHES.values()}

TOKEN = re.compile(r'''
    (?P<nl>\n)
  | (?P<ws>[ \t\r\f\v]+)
  | (?P<lc>//[^\n]*)
  | (?P<bc>/\*)
  | (?P<raw>b?r\#*")
  | (?P<str>b?"(?:[^"\\]|\\.)*")
  | (?P<chr>b?'(?:[^'\\\n]|\\(?:x[0-9a-fA-F]{2}|u\{[0-9a-fA-F]+\}|.))')
  | (?P<life>'[A-Za-z_]\w*)
  | (?P<id>r\#[A-Za-z_]\w*|[A-Za-z_]\w*)
  | (?P<num>\d[\w.]*)
  | (?P<p>.)
''', re.X | re.S)
TRIVIA = {'nl', 'ws', 'lc', 'bc'}
OPEN = {'(': ')', '[': ']', '{': '}'}
CLOSE = {')', ']', '}'}
NAMED = {'fn', 'struct', 'union', 'enum', 'type', 'mod', 'trait', 'static'}


class MergeError(Exception):
    pass


def tokenize(text):
    """(kinds, values, starts) with a sentinel start at len(text)."""
    kinds, values, starts = [], [], []
    position, end = 0, len(text)
    match = TOKEN.match
    while position < end:
        found = match(text, position)
        kind = found.lastgroup
        stop = found.end()
        if kind == 'bc':
            depth, stop = 1, position + 2
            while depth:
                opening = text.find('/*', stop)
                closing = text.find('*/', stop)
                if closing < 0:
                    raise MergeError(f'unterminated block comment at {position}')
                if 0 <= opening < closing:
                    depth, stop = depth + 1, opening + 2
                else:
                    depth, stop = depth - 1, closing + 2
        elif kind == 'raw':
            hashes = stop - position - (3 if text[position] == 'b' else 2)
            closing = text.find('"' + '#' * hashes, stop)
            if closing < 0:
                raise MergeError(f'unterminated raw string at {position}')
            kind, stop = 'str', closing + 1 + hashes
        kinds.append(kind)
        values.append(text[position:stop])
        starts.append(position)
        position = stop
    starts.append(end)
    return kinds, values, starts


class Item:
    """One item: text, (kind, name) key, offset of its first attribute or
    keyword (`sig`), whether it starts a line (`bol`), and for an extern block
    the header, the foreign items and the footer."""

    __slots__ = ('kind', 'name', 'text', 'sig', 'body', 'bol', 'header', 'children', 'footer', '_norm')

    def __init__(self, kind, name, text, sig, body, bol, header=None, children=None, footer=None):
        self.kind, self.name, self.text = kind, name, text
        self.sig, self.body, self.bol = sig, body, bol
        self.header, self.children, self.footer = header, children, footer
        self._norm = None

    @property
    def key(self):
        return (self.kind, self.name)

    @property
    def norm(self):
        if self._norm is None:
            self._norm = normalized(self.text)
        return self._norm


def normalized(text):
    """Token sequence without whitespace and without wrapping trailing commas
    (before `]` and `}` always, before `)` only beside another comma, so a
    one-element tuple stays a tuple)."""
    kinds, values, _ = tokenize(text)
    out, stack = [], []
    for kind, value in zip(kinds, values):
        if kind in ('nl', 'ws'):
            continue
        if kind == 'p':
            if value in OPEN:
                stack.append(0)
            elif value in CLOSE:
                commas = stack.pop() if stack else 0
                if out and out[-1] == ',' and (value != ')' or commas >= 2):
                    out.pop()
            elif value == ',' and stack:
                stack[-1] += 1
        out.append(value)
    return tuple(out)


class Splitter:
    def __init__(self, text):
        self.text = text
        self.kinds, self.values, self.starts = tokenize(text)
        self.count = len(self.kinds)

    def significant(self, index, stop):
        while index < stop and self.kinds[index] in TRIVIA:
            index += 1
        return index

    def matching(self, index, stop):
        depth = 0
        values, kinds = self.values, self.kinds
        while index < stop:
            if kinds[index] == 'p':
                value = values[index]
                if value in OPEN:
                    depth += 1
                elif value in CLOSE:
                    depth -= 1
                    if depth == 0:
                        return index
            index += 1
        raise MergeError(f'unbalanced brackets near offset {self.starts[min(index, self.count)]}')

    def line_rest(self, index, stop):
        """Index after the rest of the line (spaces, a line comment, newline)."""
        probe = index
        while probe < stop and self.kinds[probe] == 'ws':
            probe += 1
        if probe < stop and self.kinds[probe] == 'lc':
            probe += 1
        if probe < stop and self.kinds[probe] == 'nl':
            return probe + 1
        return probe if probe >= stop else index

    def split(self, index=0, stop=None, head=False):
        """Items of [index, stop) and the text of the trailing trivia."""
        stop = self.count if stop is None else stop
        values, kinds, starts = self.values, self.kinds, self.starts
        items = []
        while True:
            begin = index
            cursor = self.significant(index, stop)
            sig = cursor
            while cursor < stop and values[cursor] == '#':
                bracket = self.significant(cursor + 1, stop)
                if bracket < stop and values[bracket] == '[':
                    cursor = self.significant(self.matching(bracket, stop) + 1, stop)
                else:
                    break
            if cursor >= stop:
                if cursor != sig:
                    raise MergeError(f'attribute without item near offset {starts[sig]}')
                return items, self.text[starts[begin]:starts[stop]]
            body = cursor
            if values[body] == '#':
                inner = self.significant(body + 1, stop)
                if values[inner] != '!':
                    raise MergeError(f'unexpected # near offset {starts[body]}')
                end = self.matching(self.significant(inner + 1, stop), stop) + 1
                kind, name, block = 'inner', None, None
            else:
                end, kind, name, block = self.item_end(body, stop)
            end = self.line_rest(end, stop)
            if head and not items:
                split_at = self.blank_line(begin, sig)
                if split_at is not None:
                    items.append(self.make('head', '', begin, split_at, split_at, split_at))
                    begin = split_at
            if kind == 'inner':
                name = ''.join(v for k, v in zip(kinds[body:end], values[body:end]) if k not in TRIVIA)
            item = self.make(kind, name, begin, end, sig, body)
            if block is not None:
                header_end = self.line_rest(block[0] + 1, block[1])
                children, footer_trivia = self.split(header_end, block[1])
                item.header = self.text[starts[begin]:starts[header_end]]
                item.children = children
                item.footer = footer_trivia + self.text[starts[block[1]]:starts[end]]
            items.append(item)
            index = end

    def blank_line(self, begin, sig):
        """Index after the last blank line of a prefix that holds a comment."""
        kinds = self.kinds
        last, previous_nl, comment = None, None, False
        for index in range(begin, sig):
            kind = kinds[index]
            if kind == 'nl':
                if previous_nl is not None and comment:
                    last = index + 1
                previous_nl = index
            elif kind != 'ws':
                previous_nl = None
                comment = True
        return last

    def make(self, kind, name, begin, end, sig, body):
        offset = self.starts[begin]
        bol = offset == 0 or self.text[offset - 1] == '\n'
        return Item(kind, name, self.text[offset:self.starts[end]],
                    self.starts[sig] - offset, self.starts[body] - offset, bol)

    def item_end(self, body, stop):
        values, kinds = self.values, self.kinds
        kind, name, block = None, None, None
        cursor = body
        words = []
        while cursor < stop and kind is None:
            value = values[cursor]
            if kinds[cursor] in TRIVIA:
                cursor += 1
                continue
            words.append(value)
            if value == 'pub':
                after = self.significant(cursor + 1, stop)
                cursor = self.matching(after, stop) + 1 if values[after] == '(' else after
                continue
            if value in ('unsafe', 'async', 'default'):
                cursor += 1
                continue
            if value == 'extern':
                after = self.significant(cursor + 1, stop)
                abi = ''
                if kinds[after] == 'str':
                    abi = values[after]
                    after = self.significant(after + 1, stop)
                if values[after] == '{':
                    kind, name = 'extern', ' '.join(words + [abi])
                    block = (after, self.matching(after, stop))
                    break
                cursor = after
                continue
            if value == 'const':
                after = self.significant(cursor + 1, stop)
                if values[after] in ('fn', 'unsafe', 'extern', 'async'):
                    cursor = after
                    continue
                kind = 'const'
                name = values[after]
                break
            if value in NAMED:
                after = self.significant(cursor + 1, stop)
                if value == 'static' and values[after] == 'mut':
                    after = self.significant(after + 1, stop)
                kind, name = value, values[after]
                break
            if value == 'macro_rules':
                after = self.significant(self.significant(cursor + 1, stop) + 1, stop)
                kind, name = 'macro', values[after]
                break
            if value in ('impl', 'use'):
                kind = value
                break
            kind = 'other'
            break
        # Scan the body to its end: `;` at depth 0, or a closing `}` back at
        # depth 0 that no `;` follows.
        depth = 0
        index = body
        while index < stop:
            if kinds[index] == 'p':
                value = values[index]
                if value in OPEN:
                    depth += 1
                elif value in CLOSE:
                    depth -= 1
                    if depth < 0:
                        raise MergeError(f'unbalanced close near offset {self.starts[index]}')
                    if depth == 0 and value == '}':
                        after = self.significant(index + 1, stop)
                        if after >= stop or values[after] != ';':
                            break
                elif value == ';' and depth == 0:
                    break
            index += 1
        else:
            raise MergeError(f'item without end near offset {self.starts[body]}')
        end = index + 1
        if kind in ('impl', 'use', 'other'):
            header_stop = end
            if kind == 'impl':
                for probe in range(body, end):
                    if values[probe] == '{':
                        header_stop = probe
                        break
            limit = body + 40 if kind == 'other' else header_stop
            name = ' '.join(v for k, v in zip(kinds[body:limit], values[body:limit]) if k not in TRIVIA)
        if kind == 'extern' and block is not None:
            end = block[1] + 1
        return end, kind, name, block


def split_unit(text, head=True):
    items, tail = Splitter(text).split(head=head)
    return items, tail


def flatten(item):
    if item.children is None:
        return item.text
    return item.header + ''.join(child.text for child in item.children) + item.footer


def attributed(item, arch):
    """The item's text with this tool's cfg attribute before its first
    attribute or keyword."""
    mark = MARKS[arch]
    text, sig = item.text, item.sig
    line_start = text.rfind('\n', 0, sig) + 1
    if (line_start or item.bol) and not text[line_start:sig].strip(' \t'):
        indent = text[line_start:sig]
        return text[:line_start] + indent + mark + '\n' + text[line_start:]
    return text[:sig] + mark + ' ' + text[sig:]


def unattributed(item):
    """(arch or None, text without this tool's attribute)."""
    text = item.text
    position = text.find(MARK_PREFIX, 0, item.body)
    if position < 0:
        return None, text
    for arch, mark in MARKS.items():
        if text.startswith(mark, position):
            break
    else:
        raise MergeError(f'unknown cfg attribute in {item.key}')
    if text.find(MARK_PREFIX, position + 1, item.body) >= 0:
        raise MergeError(f'two cfg attributes in {item.key}')
    after = position + len(mark)
    line_start = text.rfind('\n', 0, position) + 1
    if text[after:after + 1] == '\n' and (line_start or item.bol) and not text[line_start:position].strip(' \t'):
        return arch, text[:line_start] + text[after + 1:]
    if text[after:after + 1] == ' ':
        return arch, text[:position] + text[after + 1:]
    raise MergeError(f'malformed cfg attribute in {item.key}')


class Stats:
    FIELDS = ('shared', 'shared_whitespace', 'pairs', 'aarch64_only', 'x86_64_only')

    def __init__(self):
        self.items = Counter()
        self.bytes = Counter()
        self.kinds = Counter()

    def add(self, field, text_bytes, kind):
        self.items[field] += 1
        self.bytes[field] += text_bytes
        if field not in ('shared', 'shared_whitespace'):
            self.kinds[f'{field}:{kind}'] += 1

    def as_dict(self):
        return {'items': {f: self.items[f] for f in self.FIELDS},
                'bytes': {f: self.bytes[f] for f in self.FIELDS},
                'differing_kinds': dict(sorted(self.kinds.items()))}


def merge_items(mac, linux, out, stats):
    matcher = difflib.SequenceMatcher(None, [i.key for i in mac], [i.key for i in linux], autojunk=False)
    for tag, a1, a2, b1, b2 in matcher.get_opcodes():
        if tag == 'equal':
            for a, b in zip(mac[a1:a2], linux[b1:b2]):
                merge_pair(a, b, out, stats)
            continue
        for a in mac[a1:a2]:
            single(a, 'aarch64', out, stats)
        for b in linux[b1:b2]:
            single(b, 'x86_64', out, stats)


def single(item, arch, out, stats):
    if item.kind in ('head', 'inner'):
        raise MergeError(f'{item.kind} item on one side only: {item.text[:80]!r}')
    out.append(attributed(item, arch))
    stats.add(f'{arch}_only', len(item.text), item.kind)


def merge_pair(a, b, out, stats):
    if a.text == b.text:
        out.append(a.text)
        stats.add('shared', len(a.text), a.kind)
        return
    if a.children is not None and b.children is not None:
        if normalized(a.header) == normalized(b.header) and normalized(a.footer) == normalized(b.footer):
            out.append(a.header)
            merge_items(a.children, b.children, out, stats)
            out.append(a.footer)
            return
    if a.norm == b.norm:
        out.append(a.text)
        stats.add('shared_whitespace', len(a.text), a.kind)
        return
    if a.kind in ('head', 'inner'):
        raise MergeError(f'{a.kind} items differ: {a.text[:80]!r}')
    out.append(attributed(a, 'aarch64'))
    out.append(attributed(b, 'x86_64'))
    stats.add('pairs', len(a.text) + len(b.text), a.kind)


def merge_unit(mac_text, linux_text):
    for label, text in (('macos', mac_text), ('linux', linux_text)):
        if MARK_PREFIX in text:
            raise MergeError(f'{label} input already contains {MARK_PREFIX!r}')
    mac, mac_tail = split_unit(mac_text)
    linux, linux_tail = split_unit(linux_text)
    if normalized(mac_tail) != normalized(linux_tail):
        raise MergeError('the trailing trivia differs')
    if mac and linux and mac[0].kind == 'head' and linux[0].kind == 'head' and mac[0].norm != linux[0].norm:
        raise MergeError('the leading comment block differs')
    stats = Stats()
    out = []
    merge_items(mac, linux, out, stats)
    out.append(mac_tail)
    return ''.join(out), stats


def project(text, arch):
    items, tail = split_unit(text)
    return ''.join(project_items(items, arch)) + tail


def project_items(items, arch):
    for item in items:
        target, stripped = unattributed(item)
        if target is not None and target != arch:
            continue
        if item.children is not None and target is None:
            yield item.header + ''.join(project_items(item.children, arch)) + item.footer
        else:
            yield stripped


def compare_items(expected, got):
    """Number of whitespace-only differences, or None on a real difference."""
    if len(expected) != len(got):
        return None
    loose = 0
    for e, g in zip(expected, got):
        if e.text == g.text:
            continue
        if e.children is not None and g.children is not None:
            if normalized(e.header) != normalized(g.header) or normalized(e.footer) != normalized(g.footer):
                return None
            inner = compare_items(e.children, g.children)
            if inner is None:
                return None
            loose += inner + (e.header != g.header) + (e.footer != g.footer)
            continue
        if e.norm != g.norm:
            return None
        loose += 1
    return loose


def verify_unit(expected, projected):
    """('exact' | 'whitespace', count) or None."""
    if expected == projected:
        return 'exact', 0
    e_items, e_tail = split_unit(expected)
    g_items, g_tail = split_unit(projected)
    if normalized(e_tail) != normalized(g_tail):
        return None
    loose = compare_items(e_items, g_items)
    if loose is None:
        return None
    return 'whitespace', loose + (e_tail != g_tail)


def va_list_hook():
    sys.path.insert(0, str(Path(__file__).resolve().parent))
    try:
        import aom_va_list
    except ImportError as error:
        return None, f'tools/aom_va_list.py not importable ({error}); va_list normalization skipped'
    finally:
        sys.path.pop(0)
    return aom_va_list.normalize, 'tools/aom_va_list.py applied'


def tree_gzip(root):
    buffer = io.BytesIO()
    with gzip.GzipFile(fileobj=buffer, mode='wb', compresslevel=9, mtime=0) as compressed:
        with tarfile.open(fileobj=compressed, mode='w', format=tarfile.PAX_FORMAT) as archive:
            for path in sorted(p for p in root.rglob('*') if p.is_file()):
                data = path.read_bytes()
                info = tarfile.TarInfo(str(path.relative_to(root)))
                info.size, info.mtime, info.mode = len(data), 0, 0o644
                archive.addfile(info, io.BytesIO(data))
    return len(buffer.getvalue())


def tree_bytes(root):
    return sum(p.stat().st_size for p in root.rglob('*') if p.is_file())


def write_modules(out):
    """The module selector, as tools/aom_one_source.py writes it."""
    lines = ['// Generated by tools/merge_aom_platforms.py (prototype selector).']
    for path in sorted(out.rglob('*.rs')):
        relative = path.relative_to(out).with_suffix('')
        lines.append(f'#[path = "{path}"] pub mod {"__".join(relative.parts)};')
    target = out.parent / f'{out.name}.modules.rs'
    target.write_text('\n'.join(lines) + '\n')
    return target


def units(root):
    return sorted(str(p.relative_to(root)) for p in root.rglob('*.rs'))


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument('--macos', type=Path, required=True)
    parser.add_argument('--linux', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--report', type=Path)
    parser.add_argument('--modules', action='store_true', help='also write <output>.modules.rs')
    parser.add_argument('--verify', action='store_true', help='project the written tree back to both targets')
    parser.add_argument('--no-va-list', action='store_true', help='skip tools/aom_va_list.py even if importable (diagnosis)')
    args = parser.parse_args(argv)
    mac_root, linux_root, out_root = args.macos.resolve(), args.linux.resolve(), args.output.resolve()
    names = units(mac_root)
    if names != units(linux_root):
        only = sorted(set(names) ^ set(units(linux_root)))
        raise SystemExit(f'unit sets differ: {only[:10]}')
    normalize, note = (None, 'va_list normalization disabled (--no-va-list)') if args.no_va_list else va_list_hook()
    print(note)
    report = {'units': {}, 'va_list': {'note': note, 'macos_changes': 0, 'linux_changes': 0}}
    totals = Stats()
    inputs = {}
    failures = []
    for name in names:
        mac_text = (mac_root / name).read_text()
        linux_text = (linux_root / name).read_text()
        if normalize is not None:
            mac_text, mac_changes = normalize(mac_text)
            linux_text, linux_changes = normalize(linux_text)
            report['va_list']['macos_changes'] += mac_changes
            report['va_list']['linux_changes'] += linux_changes
        try:
            merged, stats = merge_unit(mac_text, linux_text)
        except MergeError as error:
            failures.append(f'{name}: {error}')
            continue
        target = out_root / name
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text(merged)
        inputs[name] = (mac_text, linux_text)
        entry = stats.as_dict()
        entry['output_bytes'] = len(merged.encode())
        report['units'][name] = entry
        totals.items.update(stats.items)
        totals.bytes.update(stats.bytes)
        totals.kinds.update(stats.kinds)
    if failures:
        print('\n'.join(failures), file=sys.stderr)
        raise SystemExit(1)
    report['totals'] = totals.as_dict()
    report['totals']['units'] = len(names)
    report['totals']['units_identical'] = sum(1 for m, l in inputs.values() if m == l)
    report['totals']['output_tree_bytes'] = tree_bytes(out_root)
    report['totals']['output_tree_gzip9'] = tree_gzip(out_root)
    report['totals']['macos_tree_bytes'] = tree_bytes(mac_root)
    report['totals']['macos_tree_gzip9'] = tree_gzip(mac_root)
    report['totals']['linux_tree_bytes'] = tree_bytes(linux_root)
    report['totals']['linux_tree_gzip9'] = tree_gzip(linux_root)
    t = report['totals']
    print(f"{len(names)} units ({t['units_identical']} identical): "
          + ', '.join(f'{f} {t["items"][f]}' for f in Stats.FIELDS))
    print(f"output {t['output_tree_bytes']} bytes, gzip -9 {t['output_tree_gzip9']} "
          f"(macOS {t['macos_tree_bytes']}/{t['macos_tree_gzip9']}, "
          f"Linux {t['linux_tree_bytes']}/{t['linux_tree_gzip9']})")
    if args.modules:
        print(f'modules: {write_modules(out_root)}')
    status = 0
    if args.verify:
        verification = {'macos': Counter(), 'linux': Counter()}
        for name in names:
            merged = (out_root / name).read_text()
            for label, expected in zip(('macos', 'linux'), inputs[name]):
                result = verify_unit(expected, project(merged, ARCHES[label]))
                if result is None:
                    print(f'verify: {label} projection of {name} differs', file=sys.stderr)
                    verification[label]['different'] += 1
                    status = 1
                elif result[0] == 'exact':
                    verification[label]['exact'] += 1
                else:
                    verification[label]['whitespace_units'] += 1
                    verification[label]['whitespace_items'] += result[1]
        for label, counts in verification.items():
            print(f'verify {label}: ' + ', '.join(f'{k} {v}' for k, v in sorted(counts.items())))
        report['verify'] = {label: dict(counts) for label, counts in verification.items()}
        report['verify']['ok'] = status == 0
    if args.report:
        args.report.parent.mkdir(parents=True, exist_ok=True)
        args.report.write_text(json.dumps(report, indent=2) + '\n')
    return status


if __name__ == '__main__':
    sys.exit(main())
