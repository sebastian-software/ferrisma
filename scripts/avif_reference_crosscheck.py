#!/usr/bin/env python3
"""Cross-check Ferrisma's AVIF input against independent readers (issue #54).

For every fixture this runs `examples/avif_reference_crosscheck.rs` and
compares its output with:

- libheif (`heif-info`, `heif-dec -C nn`): an independent ISOBMFF/HEIF parser
  that applies irot/imir/clap itself, with its own YUV-to-RGB conversion. Its
  AV1 decoder plugin is reported per run (C libaom on the reference host).
- ImageMagick (`magick ... icc:`), which reads the ICC profile through libheif.
- FFmpeg/libdav1d (`ffprobe`, `ffmpeg -c:v libdav1d`): an independent AV1
  sequence-header reader and the dav1d decoder, for opaque untransformed items.

It needs the release example binary, heif-info/heif-dec, magick, ffprobe and
ffmpeg on PATH. It writes temporary files below `--work` and prints one line
per fixture plus a JSON report (`--output`). It is evidence tooling, not a
CI gate; the tolerances below are part of the recorded result.

    cargo build --release --features avif-read --example avif_reference_crosscheck
    python3 scripts/avif_reference_crosscheck.py --output build/avif-crosscheck.json
"""

import argparse
import hashlib
import json
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
LIBAVIF = ROOT / 'codecs/libavif-rs/upstream/libavif'
ANDROID = LIBAVIF / 'android_jni/avifandroidjni/src/androidTest/assets/avif'
DATA = LIBAVIF / 'tests/data'
ORIENTATION = ROOT / 'tests/fixtures/orientation'

# Fixture, expected engine outcome ('ok' or an error kind substring), purpose.
FIXTURES = [
    (DATA / 'white_1x1.avif', 'ok', '1x1 still'),
    (DATA / 'colors_sdr_srgb.avif', 'ok', '8-bit sRGB nclx'),
    (DATA / 'colors_text_sdr_srgb.avif', 'ok', '8-bit sRGB nclx with text'),
    (DATA / 'paris_icc_exif_xmp.avif', 'UnsupportedColor', 'ICC needs the icc feature'),
    (DATA / 'sofa_grid1x5_420.avif', 'ok', '1x5 grid, 4:2:0'),
    (DATA / 'sofa_grid1x5_420_dimg_repeat.avif', 'any', 'grid with repeated dimg'),
    (DATA / 'sofa_grid1x5_420_reversed_dimg_order.avif', 'any', 'grid with reversed dimg'),
    (DATA / 'color_grid_alpha_nogrid.avif', 'ok', 'color grid, alpha item'),
    (DATA / 'color_grid_alpha_grid_shared_tiles.avif', 'any', 'shared grid tiles'),
    (DATA / 'abc_color_irot_alpha_irot.avif', 'ok', 'irot on color and alpha'),
    (DATA / 'abc_color_irot_alpha_NOirot.avif', 'ok', 'irot on color only; ALPHA_TRANSFORM'),
    (DATA / 'clap_irot_imir_non_essential.avif', 'UnsupportedInput', 'clap rejection'),
    (DATA / 'clop_irot_imor.avif', 'ok', 'unknown non-essential clop/imor; ALPHA_TRANSFORM'),
    (DATA / 'extended_pixi.avif', 'any', 'extended pixi'),
    (DATA / 'draw_points_idat.avif', 'any', 'item data in idat'),
    (DATA / 'weld_sato_12B_8B_q0.avif', 'any', '12-bit color, 8-bit alpha'),
    (DATA / 'seine_sdr_gainmap_srgb.avif', 'ok', 'SDR base with gain map'),
    (DATA / 'seine_sdr_gainmap_srgb_icc.avif', 'UnsupportedColor', 'gain map and ICC'),
    (DATA / 'colors_hdr_srgb.avif', 'any', '10-bit PQ sRGB primaries'),
    (DATA / 'colors_hdr_p3.avif', 'any', '10-bit PQ Display P3'),
    (DATA / 'colors_hdr_rec2020.avif', 'any', '10-bit PQ BT.2020'),
    (DATA / 'colors_wcg_hdr_rec2020.avif', 'any', 'WCG HDR BT.2020'),
    (DATA / 'colors-animated-8bpc.avif', 'UnsupportedInput', 'animation rejection'),
    (DATA / 'alpha_noispe.avif', 'MalformedInput', 'malformed: alpha without ispe'),
    (ANDROID / 'fox.profile0.8bpc.yuv420.avif', 'ok', '8-bit 4:2:0'),
    (ANDROID / 'fox.profile0.10bpc.yuv420.avif', 'ok', '10-bit 4:2:0'),
    (ANDROID / 'fox.profile1.8bpc.yuv444.avif', 'ok', '8-bit 4:4:4'),
    (ANDROID / 'fox.profile1.10bpc.yuv444.avif', 'ok', '10-bit 4:4:4'),
    (ANDROID / 'fox.profile2.8bpc.yuv422.avif', 'ok', '8-bit 4:2:2'),
    (ANDROID / 'fox.profile2.10bpc.yuv422.avif', 'ok', '10-bit 4:2:2'),
    (ANDROID / 'fox.profile2.12bpc.yuv420.avif', 'ok', '12-bit 4:2:0'),
    (ANDROID / 'fox.profile2.12bpc.yuv422.avif', 'ok', '12-bit 4:2:2'),
    (ANDROID / 'fox.profile2.12bpc.yuv444.avif', 'ok', '12-bit 4:4:4'),
    (ANDROID / 'fox.profile0.8bpc.yuv420.monochrome.avif', 'UnsupportedInput', 'monochrome rejection'),
    (ANDROID / 'blue-and-magenta-crop.avif', 'UnsupportedInput', 'clap rejection'),
] + [(ORIENTATION / f'avif-{n}.avif', 'any', f'irot/imir orientation {n}') for n in range(1, 9)]

# Tolerances, in code values, chosen before comparison and kept in the report.
ENGINE_MAX = 2          # engine sRGB8 rows against libheif nearest-neighbor RGB
ENGINE_MEAN = 0.5
WRAPPER_MAX = 1.0       # wrapper 8-bit samples against libheif
# libavif converts 10/12-bit YUV straight to 16-bit RGB; libheif rounds RGB to
# the source depth first. Their difference reaches 1.4 source codes.
WRAPPER_MAX_HIGH = 1.5
# libavif keeps decoding files whose alpha item lacks the color item's irot
# (aviftransformtest.cc AlphaIrotNoIrot); libheif leaves that alpha untransformed.
# Fixtures marked ALPHA_TRANSFORM must match in color and differ only in alpha.
FFMPEG_MEAN = 3.0       # advisory only: FFmpeg's scaler differs in rounding


def run(args, **kw):
    return subprocess.run([str(a) for a in args], capture_output=True, **kw)


def heif_info(path):
    out = run(['heif-info', path], text=True)
    if out.returncode != 0:
        return {'error': (out.stderr or out.stdout).strip().splitlines()[-1:]}
    text = out.stdout
    info = {}
    images = re.findall(r'^image: (\d+)x(\d+) \(id=\d+\)(, primary)?', text, re.M)
    info['images'] = len(images)
    for index, (width, height, primary) in enumerate(images, 1):
        if primary:
            info['width'], info['height'], info['primary_index'] = int(width), int(height), index
    m = re.search(r'bit depth: (\d+)', text)
    info['depth'] = int(m[1]) if m else None
    info['alpha'] = bool(re.search(r'alpha channel: yes', text))
    m = re.search(r'color profile: (\w+)', text)
    info['profile'] = m[1] if m else None
    cicp = [re.search(rf'{name}: (\d+)', text) for name in (
        'colour primaries', 'transfer characteristics', 'matrix coefficients', 'full range flag')]
    info['nclx'] = [int(c[1]) for c in cicp] if all(cicp) else None
    m = re.search(r'^transformations:\n((?:  .*\n)*)', text, re.M)
    info['transforms'] = [line.strip() for line in (m[1] if m else '').splitlines()
                          if line.strip() != 'none']
    return info


def heif_decoder():
    out = run(['heif-dec', '--list-decoders'], text=True).stdout
    m = re.search(r'AVIF decoders:\n- (.*)', out)
    return m[1].strip() if m else 'unknown'


def heif_rgba16(path, work, heif):
    for stale in work.glob('heif*.png'):
        stale.unlink()
    out = run(['heif-dec', '--quiet', '-C', 'nn', path, work / 'heif.png'])
    if out.returncode != 0:
        return None, out.stderr.decode(errors='replace').strip()[-200:]
    # heif-dec writes every top-level image, numbered in heif-info's order.
    png = work / ('heif.png' if heif['images'] == 1 else f'heif-{heif["primary_index"]}.png')
    raw = run(['magick', png, '-endian', 'LSB', '-depth', '16', 'rgba:-'])
    if raw.returncode != 0:
        return None, raw.stderr.decode(errors='replace').strip()[-200:]
    data = raw.stdout
    return [int.from_bytes(data[i:i + 2], 'little') for i in range(0, len(data), 2)], None


def magick_icc(path, work):
    target = work / 'magick.icc'
    target.unlink(missing_ok=True)
    out = run(['magick', path, f'icc:{target}'])
    return target.read_bytes() if out.returncode == 0 and target.exists() else None


def ffprobe(path):
    out = run(['ffprobe', '-v', 'error', '-select_streams', 'v:0', '-show_entries',
               'stream=codec_name,width,height,pix_fmt,color_range,color_space,color_transfer,'
               'color_primaries', '-of', 'json', path], text=True)
    try:
        streams = json.loads(out.stdout).get('streams', [])
    except json.JSONDecodeError:
        streams = []
    return streams[0] if streams else {'error': out.stderr.strip()[-200:]}


def ffmpeg_rgb16(path, width, height):
    out = run(['ffmpeg', '-v', 'error', '-c:v', 'libdav1d', '-i', path, '-frames:v', '1',
               '-vf', 'scale=in_color_matrix=auto:in_range=auto:out_range=full'
               ':flags=accurate_rnd+full_chroma_int+neighbor',
               '-pix_fmt', 'rgb48le', '-f', 'rawvideo', '-'])
    data = out.stdout
    if out.returncode != 0 or len(data) != width * height * 6:
        return None
    return [int.from_bytes(data[i:i + 2], 'little') for i in range(0, len(data), 2)]


def diff(a, b, scale_a, scale_b, channels, select, visible=False):
    """Max and mean absolute difference of selected channels, in output codes.
    With `visible`, skip pixels whose straight alpha is zero in `a`: their color
    has no meaning, and the engine does not preserve it."""
    worst, total, count = 0.0, 0.0, 0
    for i in range(0, min(len(a) // channels[0], len(b) // channels[1])):
        if visible and a[i * channels[0] + 3] == 0:
            continue
        for c in select:
            d = abs(a[i * channels[0] + c] * scale_a - b[i * channels[1] + c] * scale_b)
            worst = max(worst, d)
            total += d
            count += 1
    return round(worst, 3), round(total / max(count, 1), 4)


def check(binary, path, expected, purpose, work):
    prefix = work / path.stem
    for suffix in ('.wrapper.raw', '.engine.rgba', '.icc'):
        Path(f'{prefix}{suffix}').unlink(missing_ok=True)
    out = run([binary, path, prefix], text=True)
    if out.returncode != 0:
        return dict(fixture=path.name, purpose=purpose, verdict='FAIL',
                    notes=[f'example crashed: {out.stderr.strip()[-300:]}'])
    ours = json.loads(out.stdout)
    wrapper, engine = ours['wrapper'], ours['engine']
    heif = heif_info(path)
    probe = ffprobe(path)
    result = dict(fixture=str(path.relative_to(ROOT)), purpose=purpose,
                  sha256=hashlib.sha256(path.read_bytes()).hexdigest(),
                  wrapper=wrapper, engine=engine, libheif=heif, ffprobe=probe, notes=[])
    notes, failed = result['notes'], False
    outcome = 'ok' if 'error' not in engine else engine['kind']
    if expected not in ('any', outcome):
        failed = True
        notes.append(f'expected engine outcome {expected}, got {outcome}')

    # Metadata against libheif's independent parser.
    if 'error' not in wrapper and 'width' in heif:
        if wrapper['depth'] != heif['depth']:
            failed = True
            notes.append(f'depth {wrapper["depth"]} vs libheif {heif["depth"]}')
        if wrapper['alpha'] != heif['alpha']:
            failed = True
            notes.append(f'alpha {wrapper["alpha"]} vs libheif {heif["alpha"]}')
        if heif['profile'] == 'nclx' and wrapper['container_cicp'] != heif['nclx']:
            failed = True
            notes.append(f'container CICP {wrapper["container_cicp"]} vs libheif {heif["nclx"]}')
        icc = Path(f'{prefix}.icc')
        reference_icc = magick_icc(path, work)
        if icc.exists() != (reference_icc is not None) or (
                icc.exists() and icc.read_bytes() != reference_icc):
            failed = True
            notes.append(f'ICC bytes differ from ImageMagick/libheif '
                         f'({wrapper["icc_bytes"]} vs {len(reference_icc or b"")})')
        elif icc.exists():
            notes.append(f'ICC identical to ImageMagick/libheif ({wrapper["icc_bytes"]} bytes)')
        has_clap = any(t.startswith('crop') for t in heif['transforms'])
        if has_clap != bool(wrapper['transform_flags'] & 2):
            failed = True
            notes.append(f'clap flag {wrapper["transform_flags"]} vs libheif {heif["transforms"]}')
        if has_clap and outcome == 'ok':
            failed = True
            notes.append('clap was accepted')
    if 'error' not in wrapper and 'color_primaries' in probe:
        notes.append('ffprobe colors: ' + '/'.join(
            str(probe.get(k)) for k in ('color_primaries', 'color_transfer', 'color_space',
                                        'color_range')))

    # Pixels against libheif's nearest-neighbor RGB.
    if outcome == 'ok':
        heif_pixels, error = heif_rgba16(path, work, heif)
        if heif_pixels is None:
            notes.append(f'libheif decode failed: {error}')
        elif (engine['width'], engine['height']) != (heif.get('width'), heif.get('height')):
            failed = True
            notes.append(f'upright size {engine["width"]}x{engine["height"]} vs libheif '
                         f'{heif.get("width")}x{heif.get("height")}')
        else:
            rows = Path(f'{prefix}.engine.rgba').read_bytes()
            rgb = diff(rows, heif_pixels, 1, 255 / 65535, (4, 4), (0, 1, 2), visible=True)
            alpha = diff(rows, heif_pixels, 1, 255 / 65535, (4, 4), (3,))
            result['engine_vs_libheif'] = dict(rgb_max_mean=rgb, alpha_max_mean=alpha)
            known_alpha = 'ALPHA_TRANSFORM' in purpose
            if known_alpha:
                if alpha[0] <= 1:
                    failed = True
                    notes.append('expected the documented alpha-transform divergence')
                else:
                    notes.append(f'documented alpha-transform divergence: alpha {alpha}')
            if rgb[0] > ENGINE_MAX or rgb[1] > ENGINE_MEAN or alpha[0] > 1 and not known_alpha:
                failed = True
                notes.append(f'engine pixels differ from libheif: rgb {rgb}, alpha {alpha}')
            if not heif['transforms'] and wrapper.get('decode') == 'ok':
                raw = Path(f'{prefix}.wrapper.raw').read_bytes()
                channels = 4 if wrapper['alpha'] else 3
                if wrapper['depth'] == 8:
                    samples, scale = list(raw), 1
                else:
                    samples = [int.from_bytes(raw[i:i + 2], sys.byteorder)
                               for i in range(0, len(raw), 2)]
                    scale = ((1 << wrapper['depth']) - 1) / 65535
                ref_scale = ((1 << wrapper['depth']) - 1) / 65535
                exact = diff(samples, heif_pixels, scale, ref_scale, (channels, 4), (0, 1, 2))
                result['wrapper_vs_libheif_source_depth'] = exact
                if exact[0] > (WRAPPER_MAX if wrapper['depth'] == 8 else WRAPPER_MAX_HIGH):
                    failed = True
                    notes.append(f'wrapper samples differ from libheif at source depth: {exact}')
                if not wrapper['alpha'] and probe.get('codec_name') == 'av1':
                    dav1d = ffmpeg_rgb16(path, wrapper['width'], wrapper['height'])
                    if dav1d is None:
                        notes.append('ffmpeg/libdav1d decode unavailable for this layout')
                    else:
                        advisory = diff(samples, dav1d, scale, ref_scale, (3, 3), (0, 1, 2))
                        result['wrapper_vs_ffmpeg_dav1d_source_depth'] = advisory
                        if advisory[1] > FFMPEG_MEAN:
                            notes.append(f'advisory: FFmpeg/libdav1d mean differs: {advisory}')
    result['verdict'] = 'FAIL' if failed else 'ok'
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument('--binary', type=Path,
                        default=ROOT / 'target/release/examples/avif_reference_crosscheck')
    parser.add_argument('--work', type=Path, default=ROOT / 'target/avif-crosscheck')
    parser.add_argument('--output', type=Path)
    args = parser.parse_args()
    args.work.mkdir(parents=True, exist_ok=True)
    tools = {
        'libheif': run(['heif-info', '--version'], text=True).stdout.strip(),
        'libheif_avif_decoder': heif_decoder(),
        'imagemagick': run(['magick', '-version'], text=True).stdout.splitlines()[0],
        'ffmpeg': run(['ffmpeg', '-version'], text=True).stdout.splitlines()[0],
    }
    results = []
    for path, expected, purpose in FIXTURES:
        result = check(args.binary, path, expected, purpose, args.work)
        results.append(result)
        outcome = 'ok' if 'error' not in result.get('engine', {}) else result['engine'].get('kind')
        print(f'{result["verdict"]:4} {path.name:48} engine={outcome} '
              f'{result.get("engine_vs_libheif", "")} {"; ".join(result["notes"])}', flush=True)
    failures = [r['fixture'] for r in results if r['verdict'] != 'ok']
    report = dict(check='Ferrisma AVIF input against libheif, ImageMagick and FFmpeg/libdav1d',
                  tools=tools, tolerances=dict(engine_max=ENGINE_MAX, engine_mean=ENGINE_MEAN,
                                               wrapper_max=WRAPPER_MAX,
                                               wrapper_max_high_depth=WRAPPER_MAX_HIGH,
                                               ffmpeg_mean_advisory=FFMPEG_MEAN),
                  results=results, failures=failures)
    if args.output:
        args.output.parent.mkdir(parents=True, exist_ok=True)
        args.output.write_text(json.dumps(report, indent=2) + '\n')
    print(f'{len(results) - len(failures)} of {len(results)} fixtures agree; tools: {tools}')
    if failures:
        raise SystemExit(1)


if __name__ == '__main__':
    main()
