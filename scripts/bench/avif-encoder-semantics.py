#!/usr/bin/env python3
"""Inspect the output semantics of Ferrisma's AVIF encoder (issue #53).

Encodes the same Ferrisma rows with the engine's only AVIF encoder, the
workspace libavif/AOM writer (report `encoder=libavif-rs/aom`), at 8 and 10
bits (`--avif-depth=`), and inspects every file with independent readers:
FFmpeg/libdav1d (`ffprobe`), libheif (`heif-info`, `heif-dec`), and the
standalone SSIMULACRA2 scorer, which decodes with `avif-decode`/rav1d. It
records bit depth, chroma layout, CICP, alpha, dimensions, bytes and score.
No timing is taken.

The rav1e lane (`--avif-encoder=rav1e`) left with rav1e on 2026-11-15; its
recorded results are in
docs/benchmarks/M2-avif-encoder-default-evidence-2026-11-12.md (the encoder
comparison itself in docs/benchmarks/encoder-adoption.md).

    cargo build --release --features cli --bin ferrisma
    cargo build --release --locked --manifest-path scripts/bench/quality-match/Cargo.toml
    python3 -I scripts/bench/avif-encoder-semantics.py OUT_DIR

Scores are not quality-matched.
"""

import hashlib
import json
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
CLI = ROOT / 'target/release/ferrisma'
SCORER = ROOT / 'target/release/ferrisma-quality-match'
FLOWER = ROOT / 'codecs/libjxl-rs/tests/data/decode-test/flower.png.im_q85_420.jpg'
ALPHA = ROOT / 'codecs/libavif-rs/upstream/libavif/tests/data/abc_color_irot_alpha_irot.avif'

ENCODERS = [
    ('aom-8', ['--avif-depth=8']),
    ('aom-10', ['--avif-depth=10']),
]
SOURCES = [
    # name, input, width, height, policy, scorer source
    ('flower', FLOWER, 800, 533, '--assume-srgb', f'{FLOWER}'),
    ('abc-alpha', ALPHA, 256, 512, '--assume-srgb', None),
]


def run(args, **kw):
    return subprocess.run([str(a) for a in args], capture_output=True, text=True, **kw)


def probe(path):
    out = run(['ffprobe', '-v', 'error', '-show_entries',
               'stream=index,codec_name,width,height,pix_fmt,color_range,color_space,'
               'color_transfer,color_primaries', '-of', 'json', path])
    return json.loads(out.stdout or '{}').get('streams', [])


def heif(path):
    text = run(['heif-info', path]).stdout
    get = lambda pattern: (m := re.search(pattern, text)) and m[1]
    return dict(size=get(r'image: (\d+x\d+)'), layout=get(r'colorspace: (.*)'),
                depth=get(r'bit depth: (\d+)'), alpha=get(r'alpha channel: (\w+)'),
                nclx=[get(rf'{n}: (\d+)') for n in ('colour primaries', 'transfer characteristics',
                                                     'matrix coefficients', 'full range flag')])


def alpha_values(path, work):
    png = work / 'alpha.png'
    for stale in work.glob('alpha*.png'):
        stale.unlink()
    if run(['heif-dec', '--quiet', path, png]).returncode != 0:
        return None
    raw = subprocess.run(['magick', str(png), '-alpha', 'extract', '-depth', '8', 'gray:-'],
                         capture_output=True).stdout
    return raw


def main():
    out = Path(sys.argv[1]).resolve()
    out.mkdir(parents=True, exist_ok=True)
    rows = []
    for name, source, width, height, policy, score_source in SOURCES:
        ref = out / f'{name}.ref'
        if score_source:
            run([SCORER, 'ref', score_source, width, height, ref])
        source_alpha = alpha_values(source, out) if name == 'abc-alpha' else None
        for chroma in ('444', '420'):
            for label, flags in ENCODERS:
                avif = out / f'{name}-{chroma}-{label}.avif'
                cmd = [CLI, source, avif, width, height, policy, '--quality=80', '--speed=6',
                       '--encoder-threads=1', '--transform-threads=1', f'--chroma={chroma}', *flags]
                result = run(cmd)
                # Keep the byte and memory fields of the report, never its timings.
                report = next((line for line in result.stdout.splitlines()
                               if line.startswith('report:')), '')
                report = ' '.join(f for f in report.split() if not re.search(r'_(us|ms)=', f))
                row = dict(source=name, chroma=chroma, encoder=label, quality=80, speed=6,
                           status=result.returncode, report=report)
                if result.returncode == 0:
                    data = avif.read_bytes()
                    row.update(bytes=len(data), sha256=hashlib.sha256(data).hexdigest(),
                               ffprobe=probe(avif), libheif=heif(avif))
                    if score_source:
                        score = run([SCORER, 'score', ref, avif]).stdout.split()
                        row['ssimulacra2_rav1d_decode'] = score[-1] if score else None
                    if source_alpha is not None:
                        decoded = alpha_values(avif, out)
                        if decoded and len(decoded) == len(source_alpha):
                            row['alpha_max_abs_diff_vs_source'] = max(
                                abs(a - b) for a, b in zip(decoded, source_alpha))
                else:
                    row['error'] = result.stderr.strip()[-300:]
                rows.append(row)
                print(json.dumps({k: row.get(k) for k in (
                    'source', 'chroma', 'encoder', 'bytes', 'ssimulacra2_rav1d_decode',
                    'alpha_max_abs_diff_vs_source', 'libheif', 'error')}), flush=True)
                for stream in row.get('ffprobe', []):
                    print('   ffprobe', json.dumps(stream), flush=True)
    (out / 'semantics.json').write_text(json.dumps(rows, indent=2) + '\n')


if __name__ == '__main__':
    main()
