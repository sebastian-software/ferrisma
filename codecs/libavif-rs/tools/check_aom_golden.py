#!/usr/bin/env python3
"""Check the Rust AOM encoder against recorded golden AVIF outputs (ADR 0059).

Each case encodes one still image through `examples/encode_still.rs`
(avifImageRGBToYUV + avifEncoderWrite, the calls Ferrixel makes) with the RTCD
dispatch detected at run time, and compares the SHA-256 of the AVIF bytes with
the record in `crates/aom/golden/release.json`, which lists every profile
(`<os>-<arch>-release`) that reproduces it. A profile whose floating-point
results differ gets its own `crates/aom/golden/<profile>.json`.
This check builds no C library and fits the pull-request tier.

Next to each Rust output, the record holds `c_reference_sha256`: the output
of the C reference for the same case. `--implementation c` links
`tools/encode_still.c`, which makes the same libavif calls, to the original
libavif, portable libaom (`_c` functions only) and SharpYUV builds of root
parity validation (`tools/root_pipeline.py --phase references`) and compares
its outputs with that field. A case is C-confirmed when both hashes are
equal; the Rust check reports how many are. `aom-golden-c.yml` runs the C
side nightly and on pull requests that change the record.

The corpus is derived from PNG files in the committed upstream libavif test
data by this script alone (Python standard library: PNG decoding, 16-to-8-bit
rounding, crop, transpose, mirror). Unlike the SIMD identity corpus, it does
not depend on ImageMagick or `sips`, so runner image updates cannot change the
inputs. Each record also carries the digest of the derived bitmap, and a
mismatch there is reported as an input change, not an encoder change.

The settings matrix follows Ferrixel's use of libavif: 8- and 10-bit (and
12-bit), 4:2:0 and 4:4:4, a quality range including lossless, speeds 4 to 10,
one thread and four threads with automatic tiling, and RGBA input with an
alpha plane. Thread counts are fixed, because libavif derives the tile layout
from them.

    python3 tools/check_aom_golden.py            # check
    python3 tools/check_aom_golden.py --record   # record or add this profile
    python3 tools/check_aom_golden.py --record --replace   # intended change
    python3 tools/check_aom_golden.py --implementation c            # C reference
    python3 tools/check_aom_golden.py --implementation c --record   # record it

Golden outputs change only through a reviewed diff: a C reference change or a
deliberate encoder change (ADR 0059).
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import platform
import struct
import subprocess
import sys
import time
import zlib

ROOT = Path(__file__).resolve().parents[1]
DATA = ROOT / 'upstream/libavif/tests/data'
GOLDEN = ROOT / 'crates/aom/golden'
RECORDED_FROM = ('avif_sha256: the Rust port (crates/aom, frozen libaom 3.15.2 translation) through the '
                 'root libavif port. c_reference_sha256: the C reference (tools/encode_still.c linked to the '
                 'original libavif, portable libaom, and SharpYUV builds of root parity validation; see '
                 'c_reference). A case is C-confirmed when both hashes are equal.')

# name, source PNG, operations applied in order, purpose
CORPUS = [
    ('weld-1368x2048', 'weld_16bit.png', [('transpose',), ('mirror2x2',)],
     'large portrait photo; four tiles with four threads'),
    ('weld-1024x684', 'weld_16bit.png', [], '16-bit photo rounded to 8 bits'),
    ('weld-1001x677', 'weld_16bit.png', [('crop', 0, 0, 1001, 677)],
     'odd dimensions, partial superblocks'),
    ('kodim03-768x512', 'kodim03_grayscale_cicp.png', [], 'grayscale content'),
    ('paris-403x302', 'paris_icc_exif_xmp.png', [], 'small photo with odd dimensions'),
    ('abc-512x256', 'abc.png', [], 'synthetic text and flat areas with alpha'),
]


def case(speed=6, depth=10, yuv444=False, quality=56, threads=1, alpha=False):
    return dict(speed=speed, depth=depth, yuv444=yuv444, quality=quality, threads=threads,
                alpha=alpha)


# Every image: Ferrixel's common settings.
BASE = [case(speed=6, depth=8), case(speed=6, depth=10), case(speed=9, depth=10),
        case(speed=6, depth=10, yuv444=True), case(speed=6, depth=10, threads=4)]
# Smaller images additionally cover the rest of the range.
EXTRA = [case(speed=4), case(speed=10, depth=8), case(quality=20), case(quality=90),
         case(depth=12), case(depth=8, yuv444=True, quality=100), case(speed=9, depth=8, threads=4)]
SMALL = {'weld-1001x677', 'kodim03-768x512', 'paris-403x302', 'abc-512x256'}
ALPHA = {'abc-512x256': [case(depth=8, alpha=True), case(depth=10, alpha=True),
                         case(speed=9, depth=10, yuv444=True, alpha=True),
                         case(depth=8, quality=100, yuv444=True, alpha=True)]}


def matrix():
    seen = set()
    for name, *_ in CORPUS:
        for settings in BASE + (EXTRA if name in SMALL else []) + ALPHA.get(name, []):
            if label(name, settings) in seen:
                raise SystemExit(f'Duplicate case {label(name, settings)}')
            seen.add(label(name, settings))
            yield name, settings


# --- PNG decoding (non-interlaced; gray, RGB, palette, gray+alpha, RGBA) ---

def read_png(path):
    data = path.read_bytes()
    if data[:8] != b'\x89PNG\r\n\x1a\n':
        raise SystemExit(f'{path} is not a PNG file')
    position, idat, palette, transparency = 8, [], None, None
    while position < len(data):
        length, kind = struct.unpack('>I4s', data[position:position + 8])
        body = data[position + 8:position + 8 + length]
        position += 12 + length
        if kind == b'IHDR':
            width, height, depth, color, _, _, interlace = struct.unpack('>IIBBBBB', body)
        elif kind == b'PLTE':
            palette = [tuple(body[i:i + 3]) for i in range(0, len(body), 3)]
        elif kind == b'tRNS':
            transparency = body
        elif kind == b'IDAT':
            idat.append(body)
        elif kind == b'IEND':
            break
    if interlace or depth not in (8, 16) or color not in (0, 2, 3, 4, 6):
        raise SystemExit(f'{path}: unsupported PNG layout (depth {depth}, color {color})')
    samples = {0: 1, 2: 3, 3: 1, 4: 2, 6: 4}[color]
    bpp = samples * depth // 8
    stride = width * bpp
    raw = zlib.decompress(b''.join(idat))
    rows, previous = [], bytearray(stride)
    for y in range(height):
        start = y * (stride + 1)
        kind, row = raw[start], bytearray(raw[start + 1:start + 1 + stride])
        for x in range(stride):
            left = row[x - bpp] if x >= bpp else 0
            up = previous[x]
            corner = previous[x - bpp] if x >= bpp else 0
            if kind == 1:
                row[x] = (row[x] + left) & 0xFF
            elif kind == 2:
                row[x] = (row[x] + up) & 0xFF
            elif kind == 3:
                row[x] = (row[x] + ((left + up) >> 1)) & 0xFF
            elif kind == 4:
                p = left + up - corner
                pa, pb, pc = abs(p - left), abs(p - up), abs(p - corner)
                row[x] = (row[x] + (left if pa <= pb and pa <= pc else up if pb <= pc else corner)) & 0xFF
            elif kind != 0:
                raise SystemExit(f'{path}: bad filter type {kind}')
        rows.append(row)
        previous = row
    # Convert to 8-bit RGBA rows; 16-bit samples are rounded (v * 255 / 65535).
    image = []
    for row in rows:
        if depth == 16:
            values = [((row[i] << 8 | row[i + 1]) * 255 + 32767) // 65535 for i in range(0, stride, 2)]
        else:
            values = list(row)
        pixels = []
        for x in range(width):
            v = values[x * samples:(x + 1) * samples]
            if color == 0:
                pixels.append((v[0], v[0], v[0], 255))
            elif color == 2:
                pixels.append((v[0], v[1], v[2], 255))
            elif color == 3:
                alpha = transparency[v[0]] if transparency and v[0] < len(transparency) else 255
                pixels.append((*palette[v[0]], alpha))
            elif color == 4:
                pixels.append((v[0], v[0], v[0], v[1]))
            else:
                pixels.append(tuple(v))
        image.append(pixels)
    return image


def apply(image, operation):
    if operation[0] == 'transpose':
        return [list(row) for row in zip(*image)]
    if operation[0] == 'mirror2x2':
        rows = [row + row[::-1] for row in image]
        return rows + rows[::-1]
    if operation[0] == 'crop':
        _, x, y, width, height = operation
        return [row[x:x + width] for row in image[y:y + height]]
    raise SystemExit(f'Unknown operation {operation}')


def write_bmp(image, path):
    """32-bit top-down BMP with a BITMAPV4HEADER and B, G, R, A byte masks."""
    height, width = len(image), len(image[0])
    pixels = b''.join(bytes((b, g, r, a)) for row in image for r, g, b, a in row)
    header = struct.pack('<IiiHHIIiiII', 108, width, -height, 1, 32, 3, len(pixels), 2835, 2835, 0, 0)
    header += struct.pack('<IIII', 0x00FF0000, 0x0000FF00, 0x000000FF, 0xFF000000)
    header += struct.pack('<I', 0x73524742) + bytes(36) + bytes(12)  # 'sRGB', endpoints, gamma
    offset = 14 + len(header)
    path.write_bytes(b'BM' + struct.pack('<IHHI', offset + len(pixels), 0, 0, offset) + header + pixels)


def prepare(work):
    images, corpus = {}, []
    for name, source, operations, purpose in CORPUS:
        target = work / f'{name}.bmp'
        image = read_png(DATA / source)
        for operation in operations:
            image = apply(image, operation)
        width, height = len(image[0]), len(image)
        if f'{width}x{height}' not in name:
            raise SystemExit(f'{name}: derived size is {width}x{height}')
        write_bmp(image, target)
        images[name] = target
        corpus.append(dict(name=name, source=f'upstream/libavif/tests/data/{source}',
                           source_sha256=hashlib.sha256((DATA / source).read_bytes()).hexdigest(),
                           operations=[list(operation) for operation in operations], purpose=purpose,
                           bmp_sha256=hashlib.sha256(target.read_bytes()).hexdigest()))
    return images, corpus


def label(name, settings):
    return (f'{name} speed={settings["speed"]} depth={settings["depth"]} '
            f'{"444" if settings["yuv444"] else "420"} quality={settings["quality"]} '
            f'threads={settings["threads"]}{" alpha" if settings["alpha"] else ""}')


def encode(binary, image, output, settings):
    environment = {key: value for key, value in os.environ.items()
                   if key not in ('AOM_SIMD_CAPS', 'AOM_SIMD_CAPS_MASK')}
    arguments = [binary, image, output, f'--speed={settings["speed"]}', f'--depth={settings["depth"]}',
                 f'--threads={settings["threads"]}', f'--quality={settings["quality"]}', '--report-rtcd']
    if settings['yuv444']:
        arguments.append('--yuv444')
    if settings['alpha']:
        arguments.append('--alpha')
    result = subprocess.run([str(item) for item in arguments], env=environment, check=True,
                            capture_output=True, text=True)
    lines = dict(line.split('=', 1) for line in result.stdout.split() if '=' in line)
    data = output.read_bytes()
    return hashlib.sha256(data).hexdigest(), len(data), lines['rtcd']


def host_profile(build):
    system = {'Darwin': 'macos', 'Linux': 'linux'}.get(platform.system(), platform.system().lower())
    machine = {'arm64': 'aarch64', 'AMD64': 'x86_64'}.get(platform.machine(), platform.machine())
    return f'{system}-{machine}-{build}'


def tool_version(command):
    try:
        return subprocess.run(command, capture_output=True, text=True, check=True).stdout.strip()
    except (OSError, subprocess.CalledProcessError):
        return None


def golden_file(profile, explicit):
    """A profile-specific record overrides the shared one (ADR 0059: one
    recording where the floating-point profiles agree)."""
    if explicit:
        return explicit
    specific = GOLDEN / f'{profile}.json'
    return specific if specific.exists() else GOLDEN / 'release.json'


def evidence():
    """The CI run that produced a record, or `local`."""
    if os.environ.get('GITHUB_RUN_ID'):
        return (f'{os.environ.get("GITHUB_SERVER_URL", "https://github.com")}/'
                f'{os.environ.get("GITHUB_REPOSITORY")}/actions/runs/{os.environ["GITHUB_RUN_ID"]}')
    return 'local'


def digest_file(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def relative(path):
    path = path.resolve()
    return str(path.relative_to(ROOT)) if path.is_relative_to(ROOT) else str(path)


def build_c_encoder(args, native, triple):
    """Link tools/encode_still.c to the original C reference that root parity
    validation builds (`tools/root_pipeline.py --phase references`)."""
    baseline = (args.c_baseline or ROOT / f'build/root-c-{native}-release-default').resolve()
    aom = (args.c_aom or ROOT / 'build/aom-native-release').resolve()
    references = (args.c_references or ROOT / f'build/root-references-{triple}-release').resolve()
    libavif = sorted(path for path in baseline.rglob('libavif.a') if '_deps' not in path.parts)
    if len(libavif) != 1:
        raise SystemExit(f'Expected one C libavif baseline archive under {baseline}, found {len(libavif)}')
    profile = json.loads((aom / 'reference-profile.json').read_text())
    if (profile['profile'] != 'release' or '-DAOM_TARGET_CPU=generic' not in profile['options']
            or '-ffp-contract=off' not in profile['c_flags'].split()):
        raise SystemExit('The C golden reference requires the noncontracting portable Release libaom reference')
    archives = [libavif[0], aom / 'libaom.a']
    archives += [path for path in (references / 'dav1d/src/libdav1d.a', references / 'sharpyuv/libsharpyuv.a')
                 if path.exists()]
    compiler = os.environ.get('CC', 'cc')
    binary = ROOT / 'build/aom-golden/encode_still_c'
    binary.parent.mkdir(parents=True, exist_ok=True)
    subprocess.run([compiler, '-O2', '-ffp-contract=off', '-I', ROOT / 'upstream/libavif/include',
                    ROOT / 'tools/encode_still.c', *archives, '-lm', '-lpthread', '-o', binary],
                   check=True)
    cache = baseline / 'CMakeCache.txt'
    options = {}
    if cache.exists():
        for line in cache.read_text().splitlines():
            name = line.split(':', 1)[0]
            if name.startswith(('AVIF_CODEC_', 'AVIF_LIB')) and '-' not in name and '=' in line:
                options[name] = line.split('=', 1)[1]
    provenance = dict(
        harness='tools/encode_still.c', harness_sha256=digest_file(ROOT / 'tools/encode_still.c'),
        archives={relative(path): digest_file(path) for path in archives},
        libavif_options=options,
        libaom=dict(options=profile['options'], c_flags=profile['c_flags'], compiler=profile['compiler'],
                    config_sha256=profile['config_sha256']),
        compiler=tool_version([compiler, '--version']).splitlines()[0],
        host=dict(system=platform.platform(), machine=platform.machine()),
        evidence=evidence(),
    )
    return binary, provenance


def main():
    parser = argparse.ArgumentParser(description=__doc__,
                                     formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument('--implementation', choices=['rust', 'c'], default='rust',
                        help='Encode with the Rust port (default) or with the C reference')
    parser.add_argument('--binary', type=Path, help='Use an already built encoder')
    parser.add_argument('--c-baseline', type=Path,
                        help='C libavif baseline build (default: build/root-c-<os>-release-default)')
    parser.add_argument('--c-aom', type=Path, help='C libaom reference (default: build/aom-native-release)')
    parser.add_argument('--c-references', type=Path,
                        help='dav1d and SharpYUV references (default: build/root-references-<target>-release)')
    parser.add_argument('--golden', type=Path,
                        help='Golden file (default: crates/aom/golden/<profile>.json if present, '
                             'else crates/aom/golden/release.json)')
    parser.add_argument('--record', action='store_true',
                        help='Record this profile: create the golden file, or add the profile to a '
                             'file whose outputs it reproduces; with --implementation c, record the C '
                             'reference hashes next to the Rust ones')
    parser.add_argument('--replace', action='store_true',
                        help='With --record: replace outputs that changed on purpose (a reviewed diff)')
    parser.add_argument('--output', type=Path, help='Also write the observed report here')
    args = parser.parse_args()
    profile = host_profile('release')
    native = profile.split('-')[0]
    triple = {'macos-aarch64-release': 'aarch64-apple-darwin',
              'linux-x86_64-release': 'x86_64-unknown-linux-gnu'}.get(profile)
    golden_path = golden_file(profile, args.golden)
    reference = args.implementation == 'c'

    binary, provenance = args.binary, None
    if reference:
        if binary is None:
            binary, provenance = build_c_encoder(args, native, triple)
        else:
            provenance = dict(harness='tools/encode_still.c', binary=relative(binary),
                              host=dict(system=platform.platform(), machine=platform.machine()))
    elif binary is None:
        subprocess.run(['cargo', 'build', '--release', '--locked', '--example', 'encode_still'],
                       cwd=ROOT, check=True)
        target = Path(os.environ.get('CARGO_TARGET_DIR', ROOT.parents[1] / 'target'))
        binary = target / 'release/examples/encode_still'
    work = ROOT / 'build/aom-golden'
    work.mkdir(parents=True, exist_ok=True)
    started = time.monotonic()
    images, corpus = prepare(work)

    expected = json.loads(golden_path.read_text()) if golden_path.exists() else None
    if reference and expected is None:
        raise SystemExit(f'The C reference is compared with an existing golden record: {golden_path}')
    recorded = {}
    if expected is not None:
        recorded_inputs = {item['name']: item['bmp_sha256'] for item in expected['corpus']}
        changed = [item['name'] for item in corpus if recorded_inputs.get(item['name']) != item['bmp_sha256']]
        if changed:
            raise SystemExit(f'Input bitmaps differ from the golden record ({", ".join(changed)}): '
                             'the corpus derivation changed, not the encoder.')
        recorded = {label(item['image'], item): item for item in expected['cases']}

    cases, selections, failures, unconfirmed, differs_from_rust = [], {}, [], [], []
    for name, settings in matrix():
        text = label(name, settings)
        digest, size, rtcd = encode(binary, images[name], work / 'out.avif', settings)
        case = dict(image=name, **settings, bytes=size, avif_sha256=digest)
        selections[rtcd] = selections.get(rtcd, 0) + 1
        want = recorded.get(text)
        if want is None:
            status = 'new ' if expected is None else 'NEW '
            if expected is not None:
                failures.append(f'{text}: no golden record')
        elif reference:
            # Compare with the recorded C output; before the first C recording,
            # with the Rust golden output.
            target = want.get('c_reference_sha256', want['avif_sha256'])
            if digest != want['avif_sha256']:
                differs_from_rust.append(f'{text}: C {digest} ({size} bytes), Rust golden '
                                         f'{want["avif_sha256"]} ({want["bytes"]} bytes)')
            status = 'ok  ' if digest == target else 'FAIL'
            if digest != target:
                failures.append(f'{text}: C {digest} ({size} bytes), recorded C reference {target}')
        else:
            status = 'ok  ' if want['avif_sha256'] == digest else 'FAIL'
            if want['avif_sha256'] != digest:
                failures.append(f'{text}: {digest} ({size} bytes), golden {want["avif_sha256"]} '
                                f'({want["bytes"]} bytes)')
            if 'c_reference_sha256' in want:
                case['c_reference_sha256'] = want['c_reference_sha256']
            if want.get('c_reference_sha256') != digest:
                unconfirmed.append(text)
        cases.append(case)
        print(f'{status} {text} bytes={size} rtcd={rtcd}', flush=True)
    labels = {label(case['image'], case) for case in cases}
    failures.extend(f'{text}: golden record without a case' for text in recorded if text not in labels)
    seconds = time.monotonic() - started

    if reference:
        report = dict(check='SHA-256 of AVIF output from the C reference (tools/encode_still.c)',
                      profile=profile, reference=provenance, cases=cases, seconds=round(seconds, 1),
                      failed=failures, differs_from_rust=differs_from_rust)
        if args.output:
            args.output.parent.mkdir(parents=True, exist_ok=True)
            args.output.write_text(json.dumps(report, indent=2) + '\n')
        print(f'{len(cases) - len(failures)} of {len(cases)} C reference outputs match {golden_path} '
              f'for {profile} in {seconds:.0f} s; {len(cases) - len(differs_from_rust)} of {len(cases)} '
              'equal the Rust golden outputs (C-confirmed)')
        if differs_from_rust:
            print('C reference outputs that differ from the Rust golden outputs:', *differs_from_rust,
                  sep='\n  ', file=sys.stderr)
        if args.record:
            if differs_from_rust and not args.replace:
                raise SystemExit('Not recorded: the C reference differs from the Rust golden outputs. '
                                 'Find the cause before recording (--replace records it anyway).')
            by_label = {label(case['image'], case): case for case in cases}
            for item in expected['cases']:
                item['c_reference_sha256'] = by_label[label(item['image'], item)]['avif_sha256']
            expected.setdefault('c_reference', {})[profile] = provenance
            expected['recorded_from'] = RECORDED_FROM
            golden_path.write_text(json.dumps(expected, indent=2) + '\n')
            print(f'Recorded the C reference for {profile} in {golden_path}')
            return
        if not failures and profile not in expected.get('c_reference', {}):
            print(f'Note: {profile} reproduces every recorded C output but is not listed under '
                  f'c_reference in {golden_path}; its provenance:')
            print(json.dumps({profile: provenance}, indent=2))
        if failures:
            print('C reference mismatches:', *failures, sep='\n  ', file=sys.stderr)
            raise SystemExit(1)
        return

    rust = dict(
        translation=f'crates/aom/TRANSLATION-{native}-release.json',
        translation_sha256=digest_file(ROOT / f'crates/aom/TRANSLATION-{native}-release.json'),
        cargo_profile='release',
        rustc=tool_version(['rustc', '--version']),
        host=dict(system=platform.platform(), machine=platform.machine()),
        rtcd=selections,
        evidence=evidence(),
    )
    unlisted = expected is not None and not failures and profile not in expected['profiles']
    report = dict(
        check='SHA-256 of AVIF output from encode_still equals the golden record (ADR 0059)',
        recorded_from=RECORDED_FROM,
        floating_point='Rust default code generation, which never contracts (ADR 0021, ADR 0024)',
        dispatch='RTCD defaults detected at run time; SIMD equals C by tools/check_aom_simd_identity.py',
        profiles={**(expected or {}).get('profiles', {}), profile: rust},
        c_reference=(expected or {}).get('c_reference', {}),
        encoder=('examples/encode_still.rs: BT.709 primaries, sRGB transfer, BT.601 matrix, full range; '
                 'avifImageRGBToYUV then avifEncoderWrite; autoTiling when threads != 1'),
        corpus=corpus,
        cases=cases,
    )
    if args.output:
        args.output.parent.mkdir(parents=True, exist_ok=True)
        args.output.write_text(json.dumps(dict(report, profile=profile, seconds=round(seconds, 1),
                                               failed=failures), indent=2) + '\n')
    if args.record:
        if expected is not None and failures and not (args.replace or args.golden):
            print(*failures, sep='\n', file=sys.stderr)
            raise SystemExit(f'{profile} does not reproduce {golden_path}. For an intended change, '
                             'add --replace; for a profile whose floating-point results differ, '
                             f'record it separately with --golden {GOLDEN / profile}.json')
        if expected is not None and failures:
            report['profiles'] = {profile: rust}
        golden_path.parent.mkdir(parents=True, exist_ok=True)
        golden_path.write_text(json.dumps(report, indent=2) + '\n')
        print(f'Recorded {len(cases)} cases for {profile} in {seconds:.0f} s: {golden_path}')
        return
    if expected is None:
        raise SystemExit(f'No golden file for {profile}: {golden_path}. Review the observed outputs '
                         'and record them with --record.')
    print(f'{len(cases) - len(failures)} of {len(cases)} cases match {golden_path} '
          f'for {profile} in {seconds:.0f} s; {len(cases) - len(unconfirmed)} of {len(cases)} are '
          'C-confirmed')
    if unconfirmed and len(unconfirmed) < len(cases):
        print('Not C-confirmed (the golden output differs from the recorded C reference):',
              *unconfirmed, sep='\n  ')
    if unlisted:
        print(f'Note: {profile} reproduces every output but is not listed in {golden_path}; '
              'add it with --record (the observed report from --output already lists it).')
    if failures:
        print('Golden output mismatches:', *failures, sep='\n  ', file=sys.stderr)
        raise SystemExit(1)


if __name__ == '__main__':
    main()
