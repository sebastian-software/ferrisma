#!/usr/bin/env python3
"""Check the Rust AOM encoder against recorded golden AVIF outputs (ADR 0059).

Each case encodes one still image through `examples/encode_still.rs`
(avifImageRGBToYUV + avifEncoderWrite, the calls Ferrixel makes) with the RTCD
dispatch detected at run time, and compares the SHA-256 of the AVIF bytes with
the record in `crates/aom/golden/release.json`, which lists every profile
(`<os>-<arch>-release`) that reproduces it. A profile whose floating-point
results differ gets its own `crates/aom/golden/<profile>.json`.
No C library is built.

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


def main():
    parser = argparse.ArgumentParser(description=__doc__,
                                     formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument('--binary', type=Path, help='Use an already built encode_still example')
    parser.add_argument('--golden', type=Path,
                        help='Golden file (default: crates/aom/golden/<profile>.json if present, '
                             'else crates/aom/golden/release.json)')
    parser.add_argument('--record', action='store_true',
                        help='Record this profile: create the golden file, or add the profile to a '
                             'file whose outputs it reproduces')
    parser.add_argument('--replace', action='store_true',
                        help='With --record: replace outputs that changed on purpose (a reviewed diff)')
    parser.add_argument('--output', type=Path, help='Also write the observed report here')
    args = parser.parse_args()
    profile = host_profile('release')
    translation = f'crates/aom/TRANSLATION-{profile.split("-")[0]}-release.json'
    golden_path = golden_file(profile, args.golden)

    binary = args.binary
    if binary is None:
        subprocess.run(['cargo', 'build', '--release', '--locked', '--example', 'encode_still'],
                       cwd=ROOT, check=True)
        target = Path(os.environ.get('CARGO_TARGET_DIR', ROOT / 'target'))
        binary = target / 'release/examples/encode_still'
    work = ROOT / 'build/aom-golden'
    work.mkdir(parents=True, exist_ok=True)
    started = time.monotonic()
    images, corpus = prepare(work)

    expected = json.loads(golden_path.read_text()) if golden_path.exists() else None
    expected_cases = {}
    if expected is not None:
        recorded_inputs = {item['name']: item['bmp_sha256'] for item in expected['corpus']}
        changed = [item['name'] for item in corpus if recorded_inputs.get(item['name']) != item['bmp_sha256']]
        if changed:
            raise SystemExit(f'Input bitmaps differ from the golden record ({", ".join(changed)}): '
                             'the corpus derivation changed, not the encoder.')
        expected_cases = {label(item['image'], item): item for item in expected['cases']}

    cases, selections, failures = [], {}, []
    for name, settings in matrix():
        text = label(name, settings)
        digest, size, rtcd = encode(binary, images[name], work / 'out.avif', settings)
        cases.append(dict(image=name, **settings, bytes=size, avif_sha256=digest))
        selections[rtcd] = selections.get(rtcd, 0) + 1
        if expected is None:
            status = 'new '
        else:
            want = expected_cases.pop(text, None)
            if want is None:
                status = 'NEW '
                failures.append(f'{text}: no golden record')
            elif want['avif_sha256'] != digest:
                status = 'FAIL'
                failures.append(f'{text}: {digest} ({size} bytes), golden {want["avif_sha256"]} '
                                f'({want["bytes"]} bytes)')
            else:
                status = 'ok  '
        print(f'{status} {text} bytes={size} rtcd={rtcd}', flush=True)
    failures.extend(f'{text}: golden record without a case' for text in expected_cases)
    unlisted = expected is not None and not failures and profile not in expected['profiles']

    provenance = dict(
        translation=translation,
        translation_sha256=hashlib.sha256((ROOT / translation).read_bytes()).hexdigest(),
        cargo_profile='release',
        rustc=tool_version(['rustc', '--version']),
        host=dict(system=platform.platform(), machine=platform.machine()),
        rtcd=selections,
    )
    report = dict(
        check='SHA-256 of AVIF output from encode_still equals the golden record (ADR 0059)',
        recorded_from='Rust port (crates/aom, frozen libaom 3.15.2 translation) through the root '
                      'libavif port; not yet re-recorded from the C reference',
        floating_point='Rust default code generation, which never contracts (ADR 0021, ADR 0024)',
        dispatch='RTCD defaults detected at run time; SIMD equals C by tools/check_aom_simd_identity.py',
        profiles={**(expected or {}).get('profiles', {}), profile: provenance},
        encoder=('examples/encode_still.rs: BT.709 primaries, sRGB transfer, BT.601 matrix, full range; '
                 'avifImageRGBToYUV then avifEncoderWrite; autoTiling when threads != 1'),
        corpus=corpus,
        cases=cases,
    )
    seconds = time.monotonic() - started
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
            report['profiles'] = {profile: provenance}
        golden_path.parent.mkdir(parents=True, exist_ok=True)
        golden_path.write_text(json.dumps(report, indent=2) + '\n')
        print(f'Recorded {len(cases)} cases for {profile} in {seconds:.0f} s: {golden_path}')
        return
    if expected is None:
        raise SystemExit(f'No golden file for {profile}: {golden_path}. Review the observed outputs '
                         'and record them with --record.')
    print(f'{len(cases) - len(failures)} of {len(cases)} cases match {golden_path} '
          f'for {profile} in {seconds:.0f} s')
    if unlisted:
        print(f'Note: {profile} reproduces every output but is not listed in {golden_path}; '
              'add it with --record (the observed report from --output already lists it).')
    if failures:
        print('Golden output mismatches:', *failures, sep='\n  ', file=sys.stderr)
        raise SystemExit(1)


if __name__ == '__main__':
    main()
