#!/usr/bin/env python3
"""Generate deterministic JPEG writer inputs with an actual cjpeg executable.

This is fixture production, never a Rust build/runtime dependency. No JPEG
bytes, markers, Huffman codes, or entropy data are constructed here.
"""
import argparse
import hashlib
import json
from pathlib import Path
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parent.parent


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--cjpeg', required=True, type=Path)
    args = parser.parse_args()
    executable = args.cjpeg.resolve()
    version = subprocess.run([str(executable), '-version'], capture_output=True, text=True, check=True)
    output = ROOT / 'tests/data/jpeg-writer'
    output.mkdir(parents=True, exist_ok=True)
    report = {'generator': 'tools/generate_jpeg_writer_fixtures.py',
              'cjpeg_version': (version.stdout + version.stderr).strip(),
              'cjpeg_sha256': hashlib.sha256(executable.read_bytes()).hexdigest(),
              'pixel_recipe': '((x*37+y*71+x*y*3+c*83)^((x<<2)+y*5))%256', 'files': {}}
    specifications = [
        ('baseline.jpg', 17, 9, 3, ['-quality', '90', '-sample', '1x1,1x1,1x1']),
        ('progressive.jpg', 65, 33, 3, ['-quality', '90', '-sample', '2x2,1x1,1x1', '-progressive']),
        ('restarts.jpg', 65, 33, 3, ['-quality', '90', '-sample', '2x1,1x1,1x1', '-restart', '1B']),
        ('gray.jpg', 17, 9, 1, ['-quality', '90', '-grayscale']),
        ('chunked-baseline.jpg', 257, 129, 3, ['-quality', '96', '-sample', '1x1,1x1,1x1']),
        ('large-progressive.jpg', 257, 129, 3, ['-quality', '96', '-sample', '2x2,1x1,1x1', '-progressive']),
    ]
    with tempfile.TemporaryDirectory(prefix='jpeg-writer-input-') as temp:
        for name, width, height, channels, options in specifications:
            pixels = bytes(((x*37+y*71+x*y*3+c*83)^((x<<2)+y*5))%256
                           for y in range(height) for x in range(width) for c in range(channels))
            raw = (f'P{6 if channels == 3 else 5}\n{width} {height}\n255\n'.encode() + pixels)
            source = Path(temp) / ('input.ppm' if channels == 3 else 'input.pgm')
            source.write_bytes(raw)
            command = [str(executable), *options, '-outfile', str(output/name), str(source)]
            subprocess.run(command, check=True)
            encoded = (output/name).read_bytes()
            report['files'][name] = {'width': width, 'height': height, 'channels': channels,
                                     'options': options, 'input_sha256': hashlib.sha256(raw).hexdigest(),
                                     'bytes': len(encoded), 'sha256': hashlib.sha256(encoded).hexdigest()}
    (output/'provenance.json').write_text(json.dumps(report, indent=2)+'\n')
    print(json.dumps(report, indent=2))


if __name__ == '__main__':
    main()
