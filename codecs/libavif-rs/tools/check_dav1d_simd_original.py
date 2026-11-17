#!/usr/bin/env python3
"""Whole-image decodes of the port against the original's SIMD and C builds.

Usage: python3 tools/check_dav1d_simd_original.py --level neon|avx2|sse2|scalar [--image F.avif]... [--jobs N]

The gate of ADR 0062 at one level, on this host:

- `neon` / `avx2`: the original built with its assembly
  (`tools/build_dav1d.py --asm`, `build/dav1d-asm-release/tools/dav1d`) and
  the original CLI relinked against the port (`tools/build_dav1d_consumers.py`,
  `build/dav1d-rust-release/tools/dav1d`), both with the same `--cpumask`.
- `sse2` (x86-64): as `avx2`, both sides restricted to SSE2. It is the
  level at which the original selects msac's SSE2 16-symbol function
  (`src/x86/msac.h`: on a CPU without AVX2), which the AVX2 level never
  reaches; the DSP kernels the original selects there are compared with the
  port's C.
- `scalar`: the original portable build (`build/dav1d-reference-release`)
  against the port's portable build (`--cfg dav1d_portable`,
  `tools/build_dav1d_consumers.py --portable`,
  `build/dav1d-rust-release-portable/tools/dav1d`), also with `--cpumask 0`.
  The original selects msac's assembly at compile time, the port as well,
  so only a build configuration gives the scalar path.

The corpus is every decode command of the original Meson test suite that
verifies an MD5 (the dav1d-test-data streams, Argon samples included),
re-run with `--muxer md5` so that the decoded picture hash of both sides is
compared, plus the AV1 items of every tracked AVIF under
`upstream/libavif/tests/data` (color and alpha, extracted as OBU streams) and
any `--image`. Both sides must give the same exit status and the same output;
a corpus whose expected MD5 is known must also give that MD5. Results go to
`build/dav1d-simd-original-<level>.json`.
"""
import argparse
from concurrent.futures import ThreadPoolExecutor
import hashlib
import json
from pathlib import Path
import struct
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]
CPUMASK = {'neon': 'neon', 'avx2': 'avx2', 'sse2': 'sse2', 'scalar': '0'}


def boxes(data, start, end):
    while start + 8 <= end:
        size, kind = struct.unpack('>I4s', data[start:start + 8])
        header = 8
        if size == 1:
            size, header = struct.unpack('>Q', data[start + 8:start + 16])[0], 16
        elif size == 0:
            size = end - start
        if size < header or start + size > end:
            return
        yield kind.decode('latin-1'), start + header, start + size
        start += size


def av1_items(data):
    """{item id: payload} of the `av01` items of an AVIF file (iloc, construction method 0)."""
    meta = next(((s, e) for k, s, e in boxes(data, 0, len(data)) if k == 'meta'), None)
    if not meta:
        return {}
    children = {k: (s, e) for k, s, e in boxes(data, meta[0] + 4, meta[1])}
    if 'iinf' not in children or 'iloc' not in children:
        return {}
    kinds = {}
    s, e = children['iinf']
    version = data[s]
    first = s + 4 + (2 if version == 0 else 4)
    for kind, es, ee in boxes(data, first, e):
        if kind == 'infe' and data[es] >= 2:
            item = struct.unpack('>H', data[es + 4:es + 6])[0] if data[es] == 2 else struct.unpack('>I', data[es + 4:es + 8])[0]
            offset = es + (8 if data[es] == 2 else 10)
            kinds[item] = data[offset:offset + 4].decode('latin-1')
    s, e = children['iloc']
    version, position = data[s], s + 4
    offset_size, length_size = data[position] >> 4, data[position] & 15
    base_size = data[position + 1] >> 4
    index_size = data[position + 1] & 15 if version in (1, 2) else 0
    position += 2

    def read(n):
        nonlocal position
        value = int.from_bytes(data[position:position + n], 'big') if n else 0
        position += n
        return value
    items = {}
    for _ in range(read(2) if version < 2 else read(4)):
        item = read(2) if version < 2 else read(4)
        method = read(2) & 15 if version in (1, 2) else 0
        read(2)
        base = read(base_size)
        parts = []
        for _ in range(read(2)):
            read(index_size)
            parts.append((base + read(offset_size), read(length_size)))
        if kinds.get(item) == 'av01' and method == 0:
            items[item] = b''.join(data[o:o + n] for o, n in parts)
    return items


def corpus(work, images):
    """(name, arguments, expected md5 or None) of every decode."""
    cases = []
    tests = json.loads((ROOT / 'build/dav1d-reference-release/meson-info/intro-tests.json').read_text())
    for test in tests:
        command = test['cmd']
        if not command[0].endswith('tools/dav1d') or '--verify' not in command:
            continue
        arguments = list(command[1:])
        index = arguments.index('--verify')
        expected = arguments[index + 1]
        del arguments[index:index + 2]
        cases.append((test['name'], arguments, expected))
    obu = work / 'avif-items'
    obu.mkdir(parents=True, exist_ok=True)
    sources = sorted((ROOT / 'upstream/libavif/tests/data').rglob('*.avif')) + [Path(p) for p in images]
    for source in sources:
        for item, payload in av1_items(source.read_bytes()).items():
            name = f'{source.stem}.item{item}'
            target = obu / f'{name}.obu'
            target.write_bytes(payload)
            cases.append((f'avif/{name}', ['-i', str(target), '--demuxer', 'section5'], None))
    return cases


def decode(binary, arguments, cpumask, workdir):
    command = [str(binary), *arguments, '--muxer', 'md5', '-o', '-', '--quiet', '--threads', '1']
    if cpumask is not None:
        command += ['--cpumask', cpumask]
    result = subprocess.run(command, cwd=workdir, stdout=subprocess.PIPE, stderr=subprocess.PIPE, timeout=600)
    return result.returncode, result.stdout.decode(errors='replace').strip()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--level', choices=sorted(CPUMASK), required=True)
    parser.add_argument('--image', action='append', default=[])
    parser.add_argument('--jobs', type=int, default=4)
    parser.add_argument('--suffix', default='', help='oracle suffix, e.g. -x86_64 in the QEMU container')
    args = parser.parse_args()
    build = ROOT / 'build'
    if args.level == 'scalar':
        original, original_mask = build / f'dav1d-reference-release{args.suffix}/tools/dav1d', None
    else:
        original, original_mask = build / f'dav1d-asm-release{args.suffix}/tools/dav1d', CPUMASK[args.level]
        if not json.loads((original.parents[1] / 'reference-profile.json').read_text()).get('assembly'):
            raise SystemExit('the SIMD level needs the assembly oracle (tools/build_dav1d.py --asm)')
    port = build / (f'dav1d-rust-release{args.suffix}' + ('-portable' if args.level == 'scalar' else '')) / 'tools/dav1d'
    work = build / f'dav1d-simd-original-{args.level}'
    cases = corpus(work, args.image)
    workdir = ROOT / 'build/dav1d-reference-release'

    def one(case):
        name, arguments, expected = case
        a = decode(original, arguments, original_mask, workdir)
        b = decode(port, arguments, CPUMASK[args.level], workdir)
        return {'name': name, 'original': a, 'port': b, 'expected': expected,
                'equal': a == b, 'expected_ok': expected is None or a[1] == expected}
    with ThreadPoolExecutor(max_workers=args.jobs) as executor:
        results = list(executor.map(one, cases))
    differing = [r for r in results if not r['equal'] or not r['expected_ok']]
    decoded = sum(1 for r in results if r['original'][0] == 0)
    summary = {'level': args.level, 'cpumask': CPUMASK[args.level], 'original': str(original),
               'port': str(port), 'cases': len(results), 'decoded': decoded,
               'identical': len(results) - len(differing),
               'corpus_sha256': hashlib.sha256(json.dumps([c[:2] for c in cases]).encode()).hexdigest(),
               'differing': differing}
    (build / f'dav1d-simd-original-{args.level}.json').write_text(json.dumps(summary, indent=2) + '\n')
    print(f'{args.level}: {summary["identical"]} of {len(results)} decodes identical '
          f'({decoded} decoded successfully by the original)')
    if differing:
        for r in differing[:20]:
            print(f'  differs: {r["name"]}: original {r["original"]} port {r["port"]} expected {r["expected"]}')
        sys.exit(1)


if __name__ == '__main__':
    main()
