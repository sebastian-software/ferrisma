#!/usr/bin/env python3
"""Reproduce genuine geometry fixtures and audit actual original entry points.

JPEG inputs are stored fixtures. Their JPEG XL payloads come from the pinned
public encoder; orientation/intrinsic variants use its actual metadata writer
and preserve frame payloads byte for byte. No codec arithmetic is implemented.
"""
import argparse
from collections import Counter
import gzip
import itertools
import json
import os
from pathlib import Path
import platform
import re
import shlex
import subprocess

from check import check_source
from chroma_image_reference import bindings, observe, sha

ROOT = Path(__file__).resolve().parent.parent
DATA = ROOT / 'tests/data/decoder-geometry'
MANIFEST = ROOT / 'tests/data/decoder-geometry-fixtures.json'


def link_input_hashes(build):
    command = shlex.split((build / 'CMakeFiles/jxl_oracle.dir/link.txt').read_text())
    files = [Path(value) if Path(value).is_absolute() else build / value
             for value in command if value.endswith(('.o', '.a'))]
    if not files:
        raise RuntimeError('Original geometry link inputs missing')
    return {str(path): sha(path) for path in files}


def geometry_bindings(build, directory):
    result = bindings(build, directory)
    entries = json.loads((build / 'compile_commands.json').read_text())
    entry, = [entry for entry in entries
              if Path(entry['file']).resolve() == ROOT / 'tools/oracle/decoder_geometry.cc']
    command = entry.get('arguments') or shlex.split(entry['command'])
    if '-O3' not in command or '-DNDEBUG' not in command or any(
            flag in command for flag in ('-O0', '-O1', '-fno-access-control')):
        raise RuntimeError('Geometry observer compilation changed')
    instrumented = build.name == 'oracle-asan'
    if ('-fsanitize=address,undefined' in command) != instrumented:
        raise RuntimeError('Geometry observer sanitizer coverage changed')
    if instrumented and '-fno-sanitize-recover=all' not in command:
        raise RuntimeError('Geometry observer sanitizer recovery changed')
    obj = 'CMakeFiles/jxl_oracle.dir/decoder_geometry.cc.o'
    if result['link'].count(obj) != 1:
        raise RuntimeError('Missing or duplicate geometry observer')
    expected = {'WriteCodestreamHeaders': 'enc_fields.cc.o',
                'DecodeFrame': 'dec_frame.cc.o',
                'ConvertToExternal': 'sample_conversion.cc.o'}
    include = ROOT / 'tools/oracle/uint_output.inc'
    if include.read_text().count('#include "lib/jxl/dec_external_image.cc"') != 1:
        raise RuntimeError('Original external-conversion include changed')
    contents = (directory / 'link.map').read_text()
    found = {}
    if platform.system() == 'Darwin':
        objects = dict(re.findall(r'^\[\s*(\d+)\]\s+(.+)$', contents, re.M))
        for line in contents.splitlines():
            match = re.match(r'^0x[0-9A-Fa-f]+\s+0x[0-9A-Fa-f]+\s+\[\s*(\d+)\]\s+(\S+)$', line)
            if not match:
                continue
            for name, source in expected.items():
                if match[2].lstrip('_').startswith('ZN3jxl' + str(len(name)) + name):
                    if source not in objects.get(match[1], '') or name in found:
                        raise RuntimeError('Unexpected geometry source binding: ' + line)
                    found[name] = {'object': objects[match[1]], 'symbol': line}
    else:
        current = ''
        for line in contents.splitlines():
            if re.search(r'\.o\)?\s*$', line):
                current = line
            if not re.match(r'^\s+0x[0-9a-f]+\s+', line):
                continue
            for name, source in expected.items():
                if re.search(r'jxl::' + name + r'\(', line) and line.rstrip().endswith(')'):
                    if source not in current or name in found:
                        raise RuntimeError('Unexpected geometry source binding: ' + line)
                    found[name] = {'object': current, 'symbol': line}
    if set(found) != set(expected):
        raise RuntimeError('Missing geometry source bindings: ' + str(set(expected) - set(found)))
    result.update(geometry_compile=command, geometry_symbols=found,
                  geometry_observer_sha256=sha(ROOT / 'tools/oracle/decoder_geometry.cc'),
                  external_include_sha256=sha(include),
                  geometry_object_sha256=sha(build / obj))
    return result


def fixture_observations(binary, manifest):
    results = []
    for item in manifest['cases']:
        jpeg, jxl = DATA / item['jpeg'], DATA / item['jxl']
        if sha(jpeg) != item['jpeg_sha256'] or sha(jxl) != item['jxl_sha256']:
            raise RuntimeError('Stored genuine JPEG/JPEG XL bytes changed: ' + item['name'])
        writer, header = observe(binary, ['ycc-jpeg-encode ' + jpeg.read_bytes().hex(),
                                          'ycc-header ' + jxl.read_bytes().hex()])
        if writer != item['source_encoder_observation'] or header != item['source_header_observation']:
            raise RuntimeError('Original JPEG encoder/header changed: ' + item['name'])
        words = writer.split()
        at = words.index('W')
        if words[at + 1:at + 3] != ['0', '0'] or bytes.fromhex(words[at + 3]) != jxl.read_bytes():
            raise RuntimeError('Original public JPEG encoder did not reproduce the file')
        results.extend([writer, header])
    for item in manifest['sequence_cases']:
        path = DATA / item['jxl']
        writer, = observe(binary, [item['source_request']])
        if sha(path) != item['jxl_sha256'] or writer != item['source_encoder_observation']:
            raise RuntimeError('Original sequence encoder changed: ' + item['name'])
        words = writer.split()
        if item['source_request'].startswith('fs-encode'):
            if words[0] != '0':
                raise RuntimeError('Original sequence encoder failed: ' + item['name'])
            payload = words[1]
        elif item['source_request'].startswith(('ec-encode ', 'ec-mixed-ups ')):
            payload = words[0]
        else:
            raise RuntimeError('Unknown source fixture encoder protocol: ' + item['name'])
        if bytes.fromhex(payload) != path.read_bytes():
            raise RuntimeError('Original sequence encoder did not reproduce the file: ' + item['name'])
        results.append(writer)
    base = (DATA / 'sampling-01-13x11.jxl').read_bytes()
    for item in manifest['orientation_captures']:
        rewritten, = observe(binary, [f'geo-rewrite {base.hex()} {item["orientation"]} 31 17'])
        words = rewritten.split()
        if words[1] != 'G' or words[4] != 'P':
            raise RuntimeError('Original geometry writer protocol changed')
        first, second = int(words[2]), int(words[3])
        payload = bytes.fromhex(words[0])
        if base[first:] != payload[second:] or base[first:].hex() != words[5]:
            raise RuntimeError('Original geometry rewrite changed frame payloads')
        path = DATA / item['jxl']
        if sha(path) != item['jxl_sha256'] or payload != path.read_bytes():
            raise RuntimeError('Original geometry writer did not reproduce the file')
        results.append(rewritten)
    captures = [item for group in ('cases', 'sequence_cases', 'orientation_captures')
                for item in manifest[group] if 'native_rgba_capture' in item]
    if len(captures) != 154:
        raise RuntimeError('Independent native geometry capture inventory changed')
    for item in captures:
        path = DATA / item['native_rgba_capture']
        wire = (DATA / item['jxl']).read_bytes().hex()
        observation, = observe(binary, [f'geo-export {wire} 8 0 4 0 1 16 0 1'])
        if sha(path) != item['native_rgba_sha256'] or observation != item['native_rgba_source_observation']:
            raise RuntimeError('Independent native source output changed: ' + item['name'])
        if observation.split()[0] != '0' or bytes.fromhex(observation.split()[-1]) != path.read_bytes():
            raise RuntimeError('Independent native source capture did not reproduce')
        results.append(observation)
    return results


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--requests', type=Path,
                        help='Reuse an actual JXL_DECODER_GEOMETRY_CAPTURE from the parity tests')
    args = parser.parse_args()
    check_source()
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=False)
    manifest = json.loads(MANIFEST.read_text())
    source_revision = subprocess.check_output(
        ['git', '-C', str(ROOT / 'upstream/libjxl'), 'rev-parse', 'HEAD'], text=True).strip()
    if source_revision != manifest['libjxl_revision']:
        raise RuntimeError('Geometry fixture source revision changed')
    patterns = {','.join(values) for values in itertools.product(('1x1', '2x2', '2x1', '1x2'), repeat=3)}
    if len(manifest['cases']) != 128 or Counter(
            (item['jpeg_sampling'], item['width'], item['height']) for item in manifest['cases']) != Counter(
                (pattern, width, height) for pattern in patterns for width, height in ((13, 11), (1, 1))):
        raise RuntimeError('Original JPEG sampling/dimension inventory changed')
    if len(manifest['sequence_cases']) != 20 or [item['orientation'] for item in manifest['orientation_captures']] != list(range(1, 9)):
        raise RuntimeError('Sequence/orientation fixture inventory changed')
    bases = manifest['cases'] + manifest['sequence_cases']
    expected_bases = ''.join(item['name'] + '\t' + item['jxl'] + '\n' for item in bases)
    if (ROOT / 'tests/data/decoder-geometry.requests').read_text() != expected_bases:
        raise RuntimeError('Geometry source base manifest changed')
    captures = [item for item in bases + manifest['orientation_captures']
                if 'native_rgba_capture' in item]
    expected_captures = ''.join(item['name'] + '\t' + item['jxl'] + '\t' +
                                item['native_rgba_capture'] + '\n' for item in captures)
    if (ROOT / 'tests/data/decoder-geometry-native.requests').read_text() != expected_captures:
        raise RuntimeError('Geometry native capture manifest changed')
    capture = output / 'requests.txt'
    if args.requests:
        capture.write_bytes(args.requests.read_bytes())
    else:
        env = dict(os.environ, CARGO_BUILD_JOBS='2', RUST_TEST_THREADS='2', JXL_DECODER_TARGET='scalar',
                   JXL_CPP_ORACLE=str(ROOT / 'target/oracle/jxl_oracle'),
                   JXL_DECODER_GEOMETRY_CAPTURE=str(capture))
        with (output / 'capture.log').open('w') as log:
            subprocess.run(['cargo', 'test', '--locked', '--offline', '--all-features',
                            '--test', 'parity', 'decoder_geometry::', '--', '--nocapture'],
                           cwd=ROOT, env=env, stdout=log, stderr=subprocess.STDOUT, check=True)
    requests = capture.read_text().splitlines()
    if Counter(request.split()[0] for request in requests) != Counter({'fs-decode': 21312, 'geo-export': 37376}):
        raise RuntimeError('Actual Rust geometry request inventory changed')
    report = {'source_revision': source_revision, 'fixture_manifest_sha256': sha(MANIFEST),
              'requests': len(requests), 'requests_sha256': sha(capture), 'graphs': {}}
    previous = None
    for graph in ('oracle', 'oracle-asan'):
        build, directory = ROOT / 'target' / graph, output / graph
        directory.mkdir()
        binary = build / 'jxl_oracle'
        before = sha(binary)
        inputs = link_input_hashes(build)
        binding = geometry_bindings(build, directory)
        fixtures = fixture_observations(binary, manifest)
        (directory / 'fixture-observations.txt').write_text('\n'.join(fixtures) + '\n')
        path = directory / 'observations.txt.gz'
        with gzip.open(path, 'wt', compresslevel=1) as stream:
            for start in range(0, len(requests), 8):
                stream.write('\n'.join(observe(binary, requests[start:start + 8])) + '\n')
        if previous:
            with gzip.open(previous, 'rt') as first, gzip.open(path, 'rt') as second:
                for index, (left, right) in enumerate(itertools.zip_longest(first, second)):
                    if left != right:
                        raise RuntimeError('Original geometry graph drift at request ' + str(index))
            if (previous.parent / 'fixture-observations.txt').read_bytes() != (directory / 'fixture-observations.txt').read_bytes():
                raise RuntimeError('Original geometry fixture/capture graph drift')
        if sha(binary) != before or link_input_hashes(build) != inputs:
            raise RuntimeError('Active original geometry binary, object or archive changed')
        report['graphs'][graph] = {'binary_sha256': before, 'binding': binding,
                                   'link_input_sha256': inputs,
                                   'observations_sha256': sha(path),
                                   'fixture_observations_sha256': sha(directory / 'fixture-observations.txt')}
        previous = path
    report.update(audit_passed=True, jpeg_encodings=128, sequence_encodings=20,
                  orientation_writes=8, independent_native_captures=154)
    (output / 'report.json').write_text(json.dumps(report, indent=2) + '\n')
    print(f'128 JPEG and 20 sequence encodings, 8 orientation writes, 154 independent captures and {len(requests)} exact requests agree')
    print('Original metadata/frame/output and selected chroma bindings verified; proof=' + str(output))


if __name__ == '__main__':
    main()
