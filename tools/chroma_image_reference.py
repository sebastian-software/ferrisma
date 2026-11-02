#!/usr/bin/env python3
"""Audit genuine JPEG-derived complete files, source bindings and partial state.

The fixture encoder calls pinned ParseJPG and public JxlEncoder entry points.
Temporary relinks add only a binding map; source objects, active binaries and
archive flags remain unchanged. No decoder or image arithmetic is implemented.
"""
import argparse
from collections import Counter
import gzip
import hashlib
import json
import os
from pathlib import Path
import platform
import re
import shlex
import subprocess

from chroma_reference import selected_binding

ROOT = Path(__file__).resolve().parent.parent


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def observe(binary, requests):
    result = subprocess.run([str(binary)], input='\n'.join(requests) + '\n',
                            text=True, capture_output=True, check=True)
    rows = result.stdout.splitlines()
    if result.stderr or len(rows) != len(requests):
        raise RuntimeError('Original JPEG/chroma observation failed: ' + result.stderr)
    return rows


def bindings(build, directory):
    entries = json.loads((build / 'compile_commands.json').read_text())
    entry, = [entry for entry in entries if Path(entry['file']).resolve() == ROOT / 'tools/oracle/chroma_image.cc']
    command = entry.get('arguments') or shlex.split(entry['command'])
    if '-O3' not in command or any(flag in command for flag in ('-O0', '-O1', '-fno-access-control')):
        raise RuntimeError('JPEG/header observer compilation changed')
    if ('-fsanitize=address,undefined' in command) != (build.name == 'oracle-asan'):
        raise RuntimeError('JPEG/header observer sanitizer coverage changed')
    link = shlex.split((build / 'CMakeFiles/jxl_oracle.dir/link.txt').read_text())
    relative = 'CMakeFiles/jxl_oracle.dir/chroma_image.cc.o'
    if link.count(relative) != 1:
        raise RuntimeError('Missing or duplicate JPEG fixture adapter')
    binary, mapping = directory / 'bound', directory / 'link.map'
    link[link.index('-o') + 1] = str(binary)
    link.append(('-Wl,-map,' if platform.system() == 'Darwin' else '-Wl,-Map,') + str(mapping))
    with (directory / 'link.log').open('w') as log:
        subprocess.run(link, cwd=build, stdout=log, stderr=subprocess.STDOUT, check=True)
    expected = {'JxlEncoderAddJPEGFrame': 'encode.cc.o', 'JxlEncoderProcessOutput': 'encode.cc.o',
                'ParseJPG': 'enc_jpeg_data.cc.o', 'PreparePipeline': 'dec_cache.cc.o',
                'GetYCbCrStage': 'stage_ycbcr.cc.o',
                'GetChromaUpsamplingStage': 'stage_chroma_upsampling.cc.o'}
    if platform.system() == 'Linux' and platform.machine().lower() in ('x86_64', 'amd64'):
        expected.update(GetYCbCrStage='chroma_ycbcr_stage.cc.o',
                        GetChromaUpsamplingStage='chroma_upsampling_stage.cc.o')
    contents = mapping.read_text()
    found = {}
    if platform.system() == 'Darwin':
        objects = dict(re.findall(r'^\[\s*(\d+)\]\s+(.+)$', contents, re.M))
        for line in contents.splitlines():
            row = re.match(r'^0x[0-9A-Fa-f]+\s+0x[0-9A-Fa-f]+\s+\[\s*(\d+)\]\s+(\S+)$', line)
            if not row or 'FDE' in line:
                continue
            symbol = row[2]
            for name, source in expected.items():
                namespace = '4jpeg' if name == 'ParseJPG' else ('18PassesDecoderState' if name == 'PreparePipeline' else '')
                selected = symbol == '_' + name if name.startswith('Jxl') else (
                    symbol.lstrip('_').startswith('ZN3jxl' + namespace + str(len(name)) + name))
                if selected:
                    if source not in objects.get(row[1], '') or name in found:
                        raise RuntimeError('Unexpected original symbol binding: ' + line)
                    found[name] = {'object': objects[row[1]], 'symbol': line}
    else:
        current = ''
        for line in contents.splitlines():
            if re.search(r'\.o\)?\s*$', line):
                current = line
            if not re.match(r'^\s+0x[0-9a-f]+\s+', line):
                continue
            for name, source in expected.items():
                selected = line.rstrip().endswith(name) if name.startswith('Jxl') else (
                    line.rstrip().endswith(')') and re.search(r'jxl::(?:jpeg::|PassesDecoderState::)?' + name + r'\(', line))
                if selected:
                    if source not in current or name in found:
                        raise RuntimeError('Unexpected original symbol binding: ' + line + ' in ' + current)
                    found[name] = {'object': current, 'symbol': line}
    if set(found) != set(expected):
        raise RuntimeError('Missing original source bindings: ' + str(set(expected) - set(found)))
    return {'compile': command, 'link': link, 'symbols': found,
            'observer_sha256': sha(ROOT / 'tools/oracle/chroma_image.cc'),
            'observer_object_sha256': sha(build / relative),
            'stage_configuration': selected_binding(build, directory)}


def captured_words(observation, width, height):
    words = observation.split()
    end = max(index for index, value in enumerate(words) if value == 'E')
    if words[end + 1] != '0':
        raise RuntimeError('Genuine complete source file did not succeed')
    at = next(index for index, value in enumerate(words) if value == 'L' and words[index + 1] == '3') + 2
    pixels = []
    for channel in range(3):
        if list(map(int, words[at:at + 2])) != [channel, width * height]:
            raise RuntimeError('Original initialized output coverage is incomplete')
        at += 2
        for y in range(height):
            for x in range(width):
                if list(map(int, words[at:at + 2])) != [x, y]:
                    raise RuntimeError('Original pixel order changed')
                pixels.append(int(words[at + 2]))
                at += 3
    return pixels


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=False)
    manifest = json.loads((ROOT / 'tests/data/ycbcr-jpeg-fixtures.json').read_text())
    source = ROOT / 'upstream/libjxl'
    revision = subprocess.check_output(['git', '-C', str(source), 'rev-parse', 'HEAD'], text=True).strip()
    dirty = subprocess.check_output(['git', '-C', str(source), 'status', '--porcelain', '--ignore-submodules=none'], text=True).strip()
    if revision != manifest['source_revision'] or dirty:
        raise RuntimeError('Pinned original source/dependencies changed')
    if len(manifest['cases']) != 25 or Counter(item['sampling'] for item in manifest['cases']) != Counter({name: 5 for name in ('444', '422', '420', '440', 'gray')}):
        raise RuntimeError('Genuine sampling/geometry fixture inventory changed')
    capture = output / 'requests.txt'
    environment = dict(os.environ, CARGO_BUILD_JOBS='2', RUST_TEST_THREADS='2',
                       JXL_CHROMA_IMAGE_CAPTURE=str(capture))
    with (output / 'capture.log').open('w') as log:
        subprocess.run(['cargo', 'test', '--locked', '--offline', '--all-features', '--test', 'parity',
                        'chroma_image::', '--', '--nocapture'], cwd=ROOT, env=environment,
                       stdout=log, stderr=subprocess.STDOUT, check=True)
    requests = capture.read_text().splitlines()
    if not requests or any(not request.startswith(('ycc-header ', 'fs-decode ')) for request in requests):
        raise RuntimeError('Actual complete-file corpus missing')
    report = {'source_revision': revision, 'fixture_manifest_sha256': sha(ROOT / 'tests/data/ycbcr-jpeg-fixtures.json'),
              'requests': len(requests), 'requests_sha256': sha(capture), 'graphs': {}}
    previous = None
    for graph in ('oracle', 'oracle-asan'):
        build, directory = ROOT / 'target' / graph, output / graph
        directory.mkdir()
        binary = build / 'jxl_oracle'
        before = sha(binary)
        binding = bindings(build, directory)
        writers = []
        for item in manifest['cases']:
            jpeg, jxl = (ROOT / 'tests/data' / item[name] for name in ('jpeg', 'jxl'))
            if sha(jpeg) != item['jpeg_sha256'] or sha(jxl) != item['jxl_sha256']:
                raise RuntimeError('Genuine fixture bytes changed')
            wire, = observe(binary, ['ycc-jpeg-encode ' + jpeg.read_bytes().hex()])
            if wire != item['original_writer_observation'] or bytes.fromhex(wire.split()[-1]) != jxl.read_bytes():
                raise RuntimeError('Actual original JPEG encoder bytes/metadata changed: ' + item['name'])
            import struct
            pixels = ROOT / 'tests/data' / item['float_pixels']
            if sha(pixels) != item['float_pixels_sha256']:
                raise RuntimeError('Captured original pixels changed')
            complete, = observe(binary, ['fs-decode ' + jxl.read_bytes().hex()])
            actual = captured_words(complete, item['width'], item['height'])
            if struct.pack('<' + 'I' * len(actual), *actual) != pixels.read_bytes():
                raise RuntimeError('Captured native original words disagree: ' + item['name'])
            header, = observe(binary, ['ycc-header ' + jxl.read_bytes().hex()])
            fields = list(map(int, header.split()))
            hs, vs = {'444': (0, 0), '422': (1, 0), '420': (1, 1),
                      '440': (0, 1), 'gray': (0, 0)}[item['sampling']]
            w, h = item['width'], item['height']
            xb, yb = ((w + (8 << hs) - 1) // (8 << hs)) << hs, ((h + (8 << vs) - 1) // (8 << vs)) << vs
            if len(fields) != 26 or fields[2:9] != [2, 0, 128 if hs or vs else 0, 8, int(item['sampling'] == 'gray'), 0, 0]:
                raise RuntimeError('Actual original JPEG frame metadata differs: ' + item['name'])
            if fields[9:15] != [w, h, xb * 8, yb * 8, xb, yb] or fields[19:26] != [hs, vs, 0, 0, hs, vs, 0]:
                raise RuntimeError('Actual original sampling/padding differs: ' + item['name'])
            group = fields[15]
            if fields[16:19] != [((w + group - 1) // group) * ((h + group - 1) // group), (w + group - 1) // group, (h + group - 1) // group]:
                raise RuntimeError('Actual original subsampled group dimensions differ')
            writers.append(wire)
        (directory / 'writer-observations.txt').write_text('\n'.join(writers) + '\n')
        observations = directory / 'observations.txt.gz'
        with gzip.open(observations, 'wt', compresslevel=1) as stream:
            for start in range(0, len(requests), 4):
                stream.write('\n'.join(observe(binary, requests[start:start + 4])) + '\n')
        if previous is not None:
            with gzip.open(previous, 'rt') as left, gzip.open(observations, 'rt') as right:
                for index, request in enumerate(requests):
                    if next(left).strip() != next(right).strip():
                        raise RuntimeError(f'Original ordinary/ASan drift at {index}: {request}')
                if next(left, None) is not None or next(right, None) is not None:
                    raise RuntimeError('Trailing complete-file observations')
        if sha(binary) != before:
            raise RuntimeError('Active original binary changed')
        report['graphs'][graph] = {'binary_sha256': before, 'binding': binding,
                                   'observations_sha256': sha(observations)}
        previous = observations
    report['audit_passed'] = True
    (output / 'report.json').write_text(json.dumps(report, indent=2) + '\n')
    print(f'25 genuine original JPEG encodings and {len(requests)} complete-file/header requests agree ordinary/ASan')
    print('Actual writer, parser, PreparePipeline and selected chroma factory bindings verified; proof=' + str(output))


if __name__ == '__main__':
    main()
