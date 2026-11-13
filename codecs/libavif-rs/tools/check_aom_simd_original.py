#!/usr/bin/env python3
"""Compare the Rust AOM port with the original libaom at the same SIMD level.

Owner decision D4 requires the port's scalar path to be bit-identical to the
original's scalar build and each SIMD path of the port to be bit-identical to
the original's SIMD build at the same instruction-set level, per target. The
other AOM gates compare the port with itself (`tests/simd`,
`tools/check_aom_simd_identity.py`) or with the generic C reference
(`tools/check_aom_golden.py`); this one compares it with the original's own
SIMD build.

Levels and the original build each one is compared with:

| Host | Level | Port | Original |
| --- | --- | --- | --- |
| macOS ARM64 | `neon` | RTCD detection (Neon) | `build_aom.py --simd neon` |
| macOS ARM64 | `scalar` | `AOM_SIMD_CAPS_MASK=0` | `build_aom.py` (generic) |
| x86-64 GNU Linux | `avx2` | RTCD detection (AVX2) | `build_aom.py --simd avx2` |
| x86-64 GNU Linux | `sse4.1` | `AOM_SIMD_CAPS` up to SSE4.2 | the same, same `AOM_SIMD_CAPS` |
| x86-64 GNU Linux | `scalar` | `AOM_SIMD_CAPS_MASK=0` | `build_aom.py` (generic) |

The arm64 original requires Neon, so its RTCD headers call `_neon` directly
and `AOM_SIMD_CAPS_MASK` cannot turn it off; the generic build is its scalar
counterpart. Every original is built with `-ffp-contract=off`, the declared
noncontracting reference (ADR 0024); upstream's own release build contracts
on AArch64, which changes libavif's RGB-to-YUV conversion (issue #203; ADR 0056,
amendment of 2026-11-13).

Each case is still encoded by the port (`examples/encode_still.rs`) and by the
original (`tools/encode_still.c` linked to the original C libavif and the
libaom build of the level), with the same libavif calls the engine bridge makes
(crates/ferrisma-avif-write). The AVIF bytes must be equal. The port's AVIF is
then decoded by the port (`examples/decode_planes.rs`, the reader's settings:
AOM decoder, one thread, nearest upsampling) and by the original
(`tools/decode_still.c`) at the same level; the decoded planes and RGB must be
equal.

The corpus is the 62-case golden corpus of `tools/check_aom_golden.py`
(derived from upstream PNGs with the Python standard library) plus an
extension: the libavif test photograph `dog_exif_extended_xmp_icc.jpg`,
decoded by `djpeg` (integer IDCT), turned upright and box-downscaled by 2 in
Python to 1512x2016, with the settings of issue #203 (speed 6 quality 55 and
speed 7 quality 56, 10-bit 4:4:4, one and four threads), the CI codec setting
(speed 6, quality 75), and further speeds on the small images. `--image
PATH.bmp` adds an image of your own (for example the engine's encoder input)
with the #203 settings.

    python3 tools/build_aom.py --library-only [--simd neon|avx2]   # once per level
    python3 tools/check_aom_simd_original.py                     # all levels
    python3 tools/check_aom_simd_original.py --level neon --corpus extended

On macOS ARM64 the x86-64 originals can be built (`--x86-64`, `--simd avx2`)
and run under Rosetta 2, but the port compiles only for macOS ARM64 and GNU
Linux x86-64, so the x86-64 levels run on GNU Linux. Any difference fails.
"""
import argparse
import concurrent.futures
import hashlib
import json
import os
from pathlib import Path
import platform
import shutil
import subprocess
import sys
import time

from build_aom import reference_build
from check_aom_golden import label, matrix as golden_matrix, prepare as golden_prepare, write_bmp

ROOT = Path(__file__).resolve().parents[1]
DATA = ROOT / 'upstream/libavif/tests/data'
WORK = ROOT / 'build/aom-simd-original'
MACOS = sys.platform == 'darwin' and platform.machine() == 'arm64'
LINUX = sys.platform.startswith('linux') and platform.machine() == 'x86_64'
ARCH = 'arm64' if MACOS else 'x86_64'
# x86_simd_caps() flags up to SSE4.2 (aom_ports/x86.h), as in
# tools/check_aom_simd_identity.py.
SSE4_CAPS = hex(0x1 | 0x2 | 0x4 | 0x8 | 0x10 | 0x20 | 0x100)

# level: (port environment, expected port RTCD report, original build, original environment)
LEVELS = {
    'neon': ({}, 'simd(neon)', 'neon', {}),
    'avx2': ({}, 'simd(avx2)', 'avx2', {}),
    'sse4.1': ({'AOM_SIMD_CAPS': SSE4_CAPS}, 'simd(sse4.1)', 'avx2', {'AOM_SIMD_CAPS': SSE4_CAPS}),
    'scalar': ({'AOM_SIMD_CAPS_MASK': '0'}, 'c', 'generic', {}),
}
HOST_LEVELS = ['neon', 'scalar'] if MACOS else ['avx2', 'sse4.1', 'scalar']


def case(speed=6, depth=10, yuv444=False, quality=56, threads=1, alpha=False):
    return dict(speed=speed, depth=depth, yuv444=yuv444, quality=quality, threads=threads, alpha=alpha)


# Issue #203 and the CI codec comparison (scripts/bench/ci-perf/codec_avif.sh).
ISSUE_203 = [case(speed=6, quality=55, yuv444=True), case(speed=6, quality=55, yuv444=True, threads=4),
             case(speed=7, quality=56, yuv444=True), case(speed=7, quality=56, yuv444=True, threads=4)]
PHOTO = ISSUE_203 + [case(speed=6, quality=75, yuv444=True), case(speed=6, quality=75, depth=8),
                     case(speed=8, quality=60, depth=8), case(speed=5, quality=50, yuv444=True)]
SMALL = {'paris-403x302': [case(speed=s, depth=d, quality=q) for s, d, q in
                           [(2, 10, 56), (3, 8, 40), (5, 8, 56), (7, 10, 56), (8, 8, 70)]]
         + [case(speed=7, quality=55, yuv444=True), case(speed=6, quality=0)],
         'abc-512x256': [case(speed=7, depth=8, alpha=True), case(speed=5, depth=10, quality=80, alpha=True),
                         case(speed=8, depth=8, yuv444=True, alpha=True)],
         'kodim03-768x512': [case(speed=7, depth=8, quality=30), case(speed=5, depth=10, yuv444=True)]}


def digest(data):
    return hashlib.sha256(data).hexdigest()


def run(command, **kwargs):
    return subprocess.run([str(item) for item in command], check=True, **kwargs)


def dog_photo(work):
    """`dog_exif_extended_xmp_icc.jpg` upright (EXIF orientation 6) and box
    downscaled by 2: 1512x2016. djpeg's default integer IDCT is exact."""
    target = work / 'dog-1512x2016.bmp'
    djpeg = shutil.which('djpeg')
    if not djpeg:
        raise SystemExit('The extended corpus needs djpeg (libjpeg-turbo) to decode the dog photograph')
    version = subprocess.run([djpeg, '-version'], capture_output=True, text=True).stderr.strip().splitlines()
    if not target.exists():
        ppm = subprocess.run([djpeg, '-dct', 'int', '-ppm', DATA / 'dog_exif_extended_xmp_icc.jpg'],
                             check=True, capture_output=True).stdout
        header, offset = [], 0
        while len(header) < 4:
            end = ppm.index(b'\n', offset)
            header += ppm[offset:end].split()
            offset = end + 1
        width, height, maximum = int(header[1]), int(header[2]), int(header[3])
        if header[0] != b'P6' or maximum != 255:
            raise SystemExit('djpeg did not write an 8-bit binary PPM')
        pixels = ppm[offset:offset + width * height * 3]
        rows = [pixels[y * width * 3:(y + 1) * width * 3] for y in range(height)]
        out_width, out_height = width // 2, height // 2
        small = []
        for y in range(out_height):
            top, bottom = rows[2 * y], rows[2 * y + 1]
            row = []
            for x in range(out_width):
                i = 6 * x
                row.append(tuple((top[i + c] + top[i + 3 + c] + bottom[i + c] + bottom[i + 3 + c] + 2) >> 2
                                 for c in range(3)) + (255,))
            small.append(row)
        # Orientation 6: rotate 90 degrees clockwise.
        upright = [list(row) for row in zip(*small[::-1])]
        write_bmp(upright, target)
    return target, dict(name='dog-1512x2016', source='upstream/libavif/tests/data/dog_exif_extended_xmp_icc.jpg',
                        operations=['djpeg -dct int', 'box 2x2 (rounded)', 'rotate 90 clockwise (EXIF 6)'],
                        djpeg=version[0] if version else 'unknown', bmp_sha256=digest(target.read_bytes()),
                        purpose='photograph of issue #203 at its settings')


def build_libavif():
    """The original C libavif (no libaom inside; linked per level)."""
    build = ROOT / f'build/libavif-c-{ARCH}-release'
    archive = build / 'libavif_internal.a'
    flags = '-ffp-contract=off'
    command = ['cmake', '-S', ROOT / 'upstream/libavif', '-B', build, '-DCMAKE_BUILD_TYPE=Release',
               '-DBUILD_SHARED_LIBS=OFF', f'-DCMAKE_C_FLAGS={flags}', '-DAVIF_CODEC_AOM=SYSTEM',
               '-DAVIF_CODEC_AOM_ENCODE=ON', '-DAVIF_CODEC_AOM_DECODE=ON', '-DAVIF_CODEC_DAV1D=OFF',
               '-DAVIF_LIBYUV=OFF', '-DAVIF_LIBSHARPYUV=OFF', '-DAVIF_LIBXML2=OFF', '-DAVIF_BUILD_APPS=OFF',
               '-DAVIF_BUILD_TESTS=OFF', '-DAVIF_BUILD_EXAMPLES=OFF',
               f'-DAOM_INCLUDE_DIR={ROOT / "build/aom-native-source"}',
               f'-DAOM_LIBRARY={reference_build("release") / "libaom.a"}']
    if shutil.which('ninja'):
        command += ['-G', 'Ninja']
    if not archive.exists():
        run(command, capture_output=True)
        run(['cmake', '--build', build, '--target', 'avif_internal', '--parallel', '6'], capture_output=True)
    return archive, dict(cmake_options=[str(item) for item in command[5:] if str(item).startswith('-DAVIF')],
                         c_flags=flags, archive_sha256=digest(archive.read_bytes()),
                         note=('libavif_internal.a: the libavif objects without the merged libaom, so each '
                               'level links its own libaom build; AVIF_LIBSHARPYUV and AVIF_LIBYUV are off and '
                               'neither is reached with the default chroma downsampling'))


def original_tools(build_name, libavif):
    profile_dir = reference_build('release', build_name)
    profile_path = profile_dir / 'reference-profile.json'
    if not profile_path.exists():
        option = [] if build_name == 'generic' else ['--simd', build_name]
        raise SystemExit(f'Missing original {build_name} reference: run '
                         f'python3 tools/build_aom.py --library-only {" ".join(option)}')
    profile = json.loads(profile_path.read_text())
    if profile.get('fp_contract', 'off') != 'off' or profile.get('simd', 'generic') != build_name:
        raise SystemExit(f'{profile_dir} is not the noncontracting {build_name} reference')
    tools = {}
    for name in ['encode_still', 'decode_still']:
        binary = WORK / 'bin' / f'{name}-{build_name}'
        binary.parent.mkdir(parents=True, exist_ok=True)
        run([os.environ.get('CC', 'cc'), '-O2', '-ffp-contract=off', '-I', ROOT / 'upstream/libavif/include',
             ROOT / f'tools/{name}.c', libavif, profile_dir / 'libaom.a', '-lm', '-lpthread', '-o', binary])
        tools[name] = binary
    return tools, dict(build=str(profile_dir.relative_to(ROOT)), options=profile['options'],
                       c_flags=profile['c_flags'], compiler=profile['compiler'], have=profile.get('have'),
                       config_sha256=profile['config_sha256'], archive_sha256=profile.get('archive_sha256'),
                       rtcd_headers_sha256=profile.get('rtcd_headers_sha256'))


def port_tools(args):
    if args.port_bin:
        return {name: args.port_bin / name for name in ['encode_still', 'decode_planes']}
    run(['cargo', 'build', '--release', '--locked', '--example', 'encode_still', '--example', 'decode_planes'],
        cwd=ROOT)
    target = Path(os.environ.get('CARGO_TARGET_DIR', ROOT.parents[1] / 'target'))
    return {name: target / 'release/examples' / name for name in ['encode_still', 'decode_planes']}


def environment(extra):
    env = {key: value for key, value in os.environ.items() if key not in ('AOM_SIMD_CAPS', 'AOM_SIMD_CAPS_MASK')}
    env.update(extra)
    return env


def arguments(settings):
    return ([f'--speed={settings["speed"]}', f'--depth={settings["depth"]}', f'--threads={settings["threads"]}',
             f'--quality={settings["quality"]}'] + (['--yuv444'] if settings['yuv444'] else [])
            + (['--alpha'] if settings['alpha'] else []))


def one_case(level, port, original, image, name, settings, directory):
    port_env, want_rtcd, _, original_env = LEVELS[level]
    directory.mkdir(parents=True, exist_ok=True)
    port_avif, c_avif = directory / 'port.avif', directory / 'c.avif'
    result = run([port['encode_still'], image, port_avif, *arguments(settings), '--report-rtcd'],
                 env=environment(port_env), capture_output=True, text=True)
    rtcd = dict(line.split('=', 1) for line in result.stdout.split() if '=' in line)['rtcd']
    run([original['encode_still'], image, c_avif, *arguments(settings)], env=environment(original_env),
        capture_output=True)
    port_bytes, c_bytes = port_avif.read_bytes(), c_avif.read_bytes()
    decoded = {}
    for side, tool, env in [('port', port['decode_planes'], port_env), ('c', original['decode_still'], original_env)]:
        planes, rgb = directory / f'{side}.planes', directory / f'{side}.rgb'
        run([tool, port_avif, planes, rgb], env=environment(env), capture_output=True)
        decoded[side] = (digest(planes.read_bytes()), digest(rgb.read_bytes()))
    record = dict(level=level, image=name, **settings, port_rtcd=rtcd, port_bytes=len(port_bytes),
                  c_bytes=len(c_bytes), port_avif_sha256=digest(port_bytes), c_avif_sha256=digest(c_bytes),
                  port_planes_sha256=decoded['port'][0], c_planes_sha256=decoded['c'][0],
                  port_rgb_sha256=decoded['port'][1], c_rgb_sha256=decoded['c'][1])
    problems = []
    if rtcd != want_rtcd:
        problems.append(f'port RTCD reports {rtcd}, expected {want_rtcd}')
    if port_bytes != c_bytes:
        problems.append(f'AVIF differs: port {len(port_bytes)} bytes, original {len(c_bytes)} bytes')
    if decoded['port'][0] != decoded['c'][0]:
        problems.append('decoded planes differ')
    if decoded['port'][1] != decoded['c'][1]:
        problems.append('decoded RGB differs')
    record['problems'] = problems
    return record


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument('--level', action='append', choices=list(LEVELS),
                        help='Level to compare (repeatable; default: every level of this host)')
    parser.add_argument('--corpus', choices=['golden', 'extended', 'all'], default='all')
    parser.add_argument('--image', type=Path, action='append', default=[],
                        help='Additional 24- or 32-bit BMP, encoded with the issue #203 settings')
    parser.add_argument('--port-bin', type=Path, help='Directory with already built encode_still and decode_planes')
    parser.add_argument('--jobs', type=int, default=4)
    parser.add_argument('--output', type=Path, help='Report path (default: build/aom-simd-original-<host>.json)')
    args = parser.parse_args()
    if not (MACOS or LINUX):
        raise SystemExit('The port runs on macOS ARM64 and x86-64 GNU Linux only (crates/aom/src/lib.rs)')
    levels = args.level or HOST_LEVELS
    unsupported = [level for level in levels if level not in HOST_LEVELS]
    if unsupported:
        raise SystemExit(f'Levels {unsupported} do not run on this host; it runs {HOST_LEVELS}')

    started = time.monotonic()
    WORK.mkdir(parents=True, exist_ok=True)
    libavif, libavif_record = build_libavif()
    originals, provenance = {}, {}
    for level in levels:
        build_name = LEVELS[level][2]
        if build_name not in originals:
            originals[build_name], provenance[build_name] = original_tools(build_name, libavif)
    port = port_tools(args)

    images, corpus, cases = {}, [], []
    if args.corpus in ('golden', 'all'):
        golden_images, golden_corpus = golden_prepare(WORK)
        images.update(golden_images)
        corpus += golden_corpus
        cases += [(name, settings) for name, settings in golden_matrix()]
    if args.corpus in ('extended', 'all'):
        if 'paris-403x302' not in images:
            golden_images, golden_corpus = golden_prepare(WORK)
            images.update(golden_images)
            corpus += [item for item in golden_corpus if item['name'] in SMALL]
        photo, record = dog_photo(WORK)
        images[record['name']] = photo
        corpus.append(record)
        cases += [(record['name'], settings) for settings in PHOTO]
        cases += [(name, settings) for name, extra in SMALL.items() for settings in extra]
    for path in args.image:
        name = f'image-{path.stem}'
        images[name] = path.resolve()
        corpus.append(dict(name=name, source=str(path), bmp_sha256=digest(path.read_bytes()),
                           purpose='--image, issue #203 settings'))
        cases += [(name, settings) for settings in ISSUE_203]

    jobs = []
    with concurrent.futures.ThreadPoolExecutor(max_workers=args.jobs) as pool:
        for level in levels:
            original = originals[LEVELS[level][2]]
            for index, (name, settings) in enumerate(cases):
                directory = WORK / 'cases' / level / f'{index:03d}'
                jobs.append(pool.submit(one_case, level, port, original, images[name], name, settings, directory))
        records = []
        for job in jobs:
            record = job.result()
            records.append(record)
            text = f'{record["level"]:6} {label(record["image"], record)}'
            status = 'ok  ' if not record['problems'] else 'FAIL'
            print(f'{status} {text} bytes={record["port_bytes"]}'
                  + ('' if not record['problems'] else ' -- ' + '; '.join(record['problems'])), flush=True)
    failures = [r for r in records if r['problems']]
    summary = {level: dict(cases=sum(r['level'] == level for r in records),
                           identical=sum(r['level'] == level and not r['problems'] for r in records))
               for level in levels}
    report = dict(
        check=('Port against the original at the same SIMD level (decision D4): AVIF bytes of a still encode, '
               'and decoded planes and RGB of the port AVIF, through the same libavif calls'),
        host=dict(system=platform.platform(), machine=platform.machine()),
        levels={level: dict(port_environment=LEVELS[level][0], port_rtcd=LEVELS[level][1],
                            original=LEVELS[level][2], original_environment=LEVELS[level][3])
                for level in levels},
        originals=provenance, libavif=libavif_record,
        port=dict(binaries={name: str(path) for name, path in port.items()},
                  rustc=subprocess.run(['rustc', '--version'], capture_output=True, text=True).stdout.strip()),
        corpus=corpus, summary=summary, seconds=round(time.monotonic() - started, 1),
        cases=records, failed=[f'{r["level"]} {label(r["image"], r)}: {"; ".join(r["problems"])}' for r in failures])
    output = args.output or ROOT / f'build/aom-simd-original-{"macos-aarch64" if MACOS else "linux-x86_64"}.json'
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(json.dumps(report, indent=2) + '\n')
    for level, counts in summary.items():
        print(f'{level}: {counts["identical"]} of {counts["cases"]} cases identical to the original')
    print(f'report: {output}')
    if failures:
        raise SystemExit(1)


if __name__ == '__main__':
    main()
