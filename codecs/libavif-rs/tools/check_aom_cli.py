#!/usr/bin/env python3
"""Compare exact AV1 streams and native-plane output through original CLI bodies."""
import argparse
import hashlib
import json
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[1]


def pixels(width, height, depth, chroma, frames):
    result = bytearray()
    for frame in range(frames):
        for plane in range(3):
            w = width if plane == 0 or chroma == '444' else (width + 1) // 2
            h = height if plane == 0 or chroma != '420' else (height + 1) // 2
            for y in range(h):
                for x in range(w):
                    value = (x * 97 + y * 61 + plane * 173 + frame * 211 + (x * y * (frame + 1))) & ((1 << depth) - 1)
                    result.extend(value.to_bytes(1 if depth == 8 else 2, 'little'))
    return bytes(result)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--debug', action='store_true')
    args = parser.parse_args()
    profile = 'debug' if args.debug else 'release'
    work = ROOT / f'build/aom-cli-{profile}'
    work.mkdir(parents=True, exist_ok=True)
    cases = [(f'intra-{depth}-{chroma}', depth, chroma, 2, 1, 1, False)
             for depth in [8, 10, 12] for chroma in ['420', '422', '444']]
    cases += [(f'lossless-{depth}', depth, '420', 0, 2, 1, True) for depth in [8, 10, 12]]
    cases += [('inter-threaded', 8, '420', 1, 6, 4, False),
              ('good-quality', 10, '420', 0, 4, 1, False)]
    records = []
    for name, depth, chroma, usage, frames, threads, lossless in cases:
        # The pinned original CLI rejects this 12-bit 4:2:2 control sequence
        # before encoding, in both native Debug and Release builds. Preserve
        # that defined exit/diagnostic behavior; do not call it stream coverage
        # or alter the original consumer to make the supplemental case pass.
        rejection = name in ['intra-12-422', 'intra-12-444']
        width, height = (128, 96) if threads > 1 else (32, 32)
        raw = work / f'{name}.yuv'
        raw.write_bytes(pixels(width, height, depth, chroma, frames))
        profile_number = 2 if depth == 12 or chroma == '422' else 1 if chroma == '444' else 0
        options = [f'--width={width}', f'--height={height}', f'--limit={frames}', '--passes=1',
                   f'--usage={usage}', f'--cpu-used={6 if usage == 0 else 8}', f'--threads={threads}',
                   '--end-usage=q', '--cq-level=28', '--ivf', '--lag-in-frames=0',
                   f'--bit-depth={depth}', f'--input-bit-depth={depth}',
                   f'--profile={profile_number}', f'--i{chroma}']
        if depth == 12:
            options += [f'--input-chroma-subsampling-x={0 if chroma == "444" else 1}',
                        f'--input-chroma-subsampling-y={1 if chroma == "420" else 0}']
        if lossless:
            options.append('--lossless=1')
        if threads > 1:
            options.append('--row-mt=1')
        commands = []
        failures = []
        for implementation, directory in [('c', f'aom-native-{profile}'), ('rust', f'aom-rust-{profile}')]:
            encoded = work / f'{name}-{implementation}.ivf'
            command = [str(ROOT / f'build/{directory}/aomenc'), *options, '-o', str(encoded), str(raw)]
            with (work / f'{name}-{implementation}-encode.log').open('w') as log:
                result = subprocess.run(command, stdout=log, stderr=subprocess.STDOUT)
            commands.append(command)
            if rejection:
                failures.append((result.returncode, (work / f'{name}-{implementation}-encode.log').read_bytes()))
                continue
            result.check_returncode()
            # Decode exactly the same reference stream in both implementations.
            # No forced chroma conversion or output bit-depth reduction is used.
            command = [str(ROOT / f'build/{directory}/aomdec'), '--rawvideo', '--threads=2',
                       '-o', str(work / f'{name}-{implementation}-decoded.yuv'),
                       str(work / f'{name}-c.ivf')]
            with (work / f'{name}-{implementation}-decode.log').open('w') as log:
                subprocess.run(command, stdout=log, stderr=subprocess.STDOUT, check=True)
            commands.append(command)
        if rejection:
            if failures[0] != failures[1] or failures[0][0] != 1 or failures[0][1] != b'Failed to set chroma subsampling x: Unspecified internal error\n':
                raise SystemExit('Original CLI rejection changed or differs in Rust')
            records.append(dict(name=name, bitdepth=depth, chroma=chroma,
                                original_cli_rejection=True, exit_code=1,
                                diagnostic=failures[0][1].decode(), exact=True, commands=commands))
            print(f'Exact original CLI rejection: {name}', flush=True)
            continue
        encoded = [(work / f'{name}-{implementation}.ivf').read_bytes() for implementation in ['c', 'rust']]
        decoded = [(work / f'{name}-{implementation}-decoded.yuv').read_bytes() for implementation in ['c', 'rust']]
        if encoded[0] != encoded[1] or decoded[0] != decoded[1]:
            raise SystemExit(f'Original/Rust byte mismatch: {name}')
        if len(decoded[0]) != raw.stat().st_size:
            raise SystemExit(f'Original CLI changed native-plane output size: {name}')
        if lossless and decoded[0] != raw.read_bytes():
            raise SystemExit(f'Lossless output differs from the source: {name}')
        records.append(dict(name=name, bitdepth=depth, chroma=chroma, frames=frames, threads=threads,
                            lossless=lossless, input_sha256=hashlib.sha256(raw.read_bytes()).hexdigest(),
                            encoded_bytes=len(encoded[0]), decoded_bytes=len(decoded[0]),
                            encoded_sha256=hashlib.sha256(encoded[0]).hexdigest(),
                            decoded_sha256=hashlib.sha256(decoded[0]).hexdigest(),
                            exact=True, commands=commands))
        print(f'Exact stream and native decoded planes: {name}', flush=True)
    (work / 'summary.json').write_text(json.dumps(dict(profile=profile, cases=records,
        original_cli_bodies_unchanged=True, codec_archive_replaced=True), indent=2) + '\n')


if __name__ == '__main__':
    main()
