#!/usr/bin/env python3
"""Audit actual original JPEG frame/group/writer code and exact full-file parity."""
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
from check import check_source
from skcms_binding import live_bindings

ROOT = Path(__file__).resolve().parent.parent
DATA = ROOT / 'tests/data'


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def observe(binary, requests):
    result = subprocess.run([str(binary)], input='\n'.join(requests) + '\n',
                            text=True, capture_output=True, check=True)
    rows = result.stdout.splitlines()
    if result.stderr or len(rows) != len(requests):
        raise RuntimeError('Original JPEG reconstruction observation failed: ' + result.stderr)
    return rows


def audit_binding(build, directory):
    entries = json.loads((build / 'compile_commands.json').read_text())
    units = [ROOT / 'tools/oracle' / name for name in
             ('jpeg_reconstruction.cc', 'group_render.cc', 'jpeg_writer.cc', 'chroma_image.cc')]
    units += [ROOT / 'upstream/libjxl/lib/jxl' / name for name in
              ('dec_frame.cc', 'dec_group.cc', 'jpeg/dec_jpeg_data_writer.cc',
               'jpeg/enc_jpeg_data_reader.cc', 'jpeg/enc_jpeg_data.cc', 'encode.cc')]
    commands = {}
    for unit in units:
        entry, = [entry for entry in entries if Path(entry['file']).resolve() == unit]
        options = entry.get('arguments') or shlex.split(entry['command'])
        sanitizers = ['-fsanitize=address,undefined', '-fno-sanitize-recover=all'] if build.name == 'oracle-asan' else []
        if [option for option in options if 'sanitize' in option] != sanitizers:
            raise RuntimeError('Original nonrecovering sanitizer coverage changed: ' + str(unit))
        if [option for option in options if option.startswith('-O')] != ['-O3'] or '-DNDEBUG' not in options:
            raise RuntimeError('Original source/observer optimization changed: ' + str(unit))
        if any(option.startswith(('-ffast-math', '-ffp-contract', '-ffp-model',
                                  '-funsafe-math', '-fno-signed-zeros')) for option in options):
            raise RuntimeError('Unexpected original numeric override: ' + str(unit))
        commands[str(unit)] = options
    fragments = json.loads((ROOT / 'tools/oracle/source_fragments.json').read_text())
    selected = [f for f in fragments if f['output'] in {
        'original_dequant_block.inc', 'original_group_get_block.inc',
        'original_group_transpose.inc', 'original_decode_group_impl.inc'}]
    if len(selected) != 4:
        raise RuntimeError('Complete original group fragment manifest missing')
    for fragment in selected:
        source = ROOT / 'upstream/libjxl' / fragment['path']
        text = source.read_text()
        if text.count(fragment['start']) != 1:
            raise RuntimeError('Ambiguous original group fragment')
        start = text.index(fragment['start'])
        body = text[start:text.index(fragment['end'], start)] + '\n'
        if hashlib.sha256(body.encode()).hexdigest() != fragment['sha256']:
            raise RuntimeError('Original group implementation changed')
        notice = ''.join(text.splitlines(keepends=True)[:5])
        line = text[:start].count('\n') + 1
        expected = notice + f'#line {line} {json.dumps(str(source))}\n' + body
        if (build / 'generated' / fragment['output']).read_text() != expected:
            raise RuntimeError('Compiled generated group body differs from pristine source')
    link = shlex.split((build / 'CMakeFiles/jxl_oracle.dir/link.txt').read_text())
    inputs = [Path(value) if Path(value).is_absolute() else build / value
              for value in link if value.endswith(('.o', '.a'))]
    inputs += [build / 'jxl_oracle', build / 'compile_commands.json',
               build / 'CMakeFiles/jxl_oracle.dir/link.txt']
    inputs += [build / 'generated' / f['output'] for f in selected]
    before = {str(path): sha(path) for path in inputs}
    binary, mapping = directory / 'bound', directory / 'link.map'
    link[link.index('-o') + 1] = str(binary)
    link.append(('-Wl,-map,' if platform.system() == 'Darwin' else '-Wl,-Map,') + str(mapping))
    with (directory / 'link.log').open('w') as log:
        subprocess.run(link, cwd=build, stdout=log, stderr=subprocess.STDOUT, check=True)
    expected = {
        'DecodeFrame': (r'_+ZN3jxl11DecodeFrame[^.]*', r'libjxl-internal\.a\(dec_frame\.cc\.o\)'),
        'DecodeGroupImpl': (r'_+ZN3jxl8N_SCALAR15DecodeGroupImpl[^.]*', r'libjxl-internal\.a\(dec_group\.cc\.o\)'),
        'selected-group': (r'_+ZN3jxl8N_SCALAR11OracleGroup15DecodeGroupImpl[^.]*', r'group_render\.cc\.o'),
        'WriteJpeg': (r'_+ZN3jxl4jpeg9WriteJpeg[^.]*', r'libjxl-internal\.a\(dec_jpeg_data_writer\.cc\.o\)'),
        'ReadJpeg': (r'_+ZN3jxl4jpeg8ReadJpeg[^.]*', r'libjxl-internal\.a\(enc_jpeg_data_reader\.cc\.o\)'),
        'ParseJPG': (r'_+ZN3jxl4jpeg8ParseJPG[^.]*', r'libjxl-internal\.a\(enc_jpeg_data\.cc\.o\)'),
        'JxlEncoderAddJPEGFrame': (r'_?JxlEncoderAddJPEGFrame', r'libjxl-internal\.a\(encode\.cc\.o\)'),
        'frame-observer': (r'_+Z25JpegReconstructionCommand[^.]*', r'jpeg_reconstruction\.cc\.o'),
        'group-observer': (r'_+Z18GroupRenderCommand[^.]*', r'group_render\.cc\.o'),
    }
    bindings = live_bindings(binary, mapping, expected)
    if any(sha(Path(path)) != value for path, value in before.items()):
        raise RuntimeError('Binding audit mutated an active original input')
    return {'compile': commands, 'symbols': bindings, 'unchanged_inputs': before}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=False)
    check_source()
    capture = output / 'requests.txt'
    environment = dict(os.environ, RUST_TEST_THREADS='1', JXL_JPEG_RECONSTRUCTION_CAPTURE=str(capture),
                       JXL_CPP_ORACLE=str(ROOT / 'target/oracle/jxl_oracle'))
    for name in ('jpeg_reconstruction::', 'group_render::jpeg_groups'):
        with (output / (name.replace(':', '_') + '.log')).open('w') as log:
            subprocess.run(['cargo', 'test', '--locked', '--offline', '--all-features', '--test', 'parity',
                            name, '--', '--nocapture'], cwd=ROOT, env=environment,
                           stdout=log, stderr=subprocess.STDOUT, check=True)
    requests = capture.read_text().splitlines()
    pairs = [line.split('\t') for line in (DATA / 'jpeg-reconstruction/fixtures.tsv').read_text().splitlines()]
    partial = sum(min(40, len((DATA / jxl).read_bytes())) + 8 for _, jxl in pairs)
    inventory = Counter(request.split()[0] for request in requests)
    if len(pairs) != 162 or inventory != Counter({
            'jr-frame': 162 + partial, 'jr-frame-mut': 162 * 8 * 2, 'dq-jpeg': 3344}):
        raise RuntimeError('Complete original JPEG case inventory changed: ' + str(inventory))
    rust = capture.with_suffix('.rust')
    initial = capture.with_suffix('.cpp')
    if sha(rust) != sha(initial):
        raise RuntimeError('Actual Rust/source capture differs')
    report = {'requests': len(requests), 'inventory': dict(inventory), 'requests_sha256': sha(capture),
              'rust_observations_sha256': sha(rust), 'graphs': {}, 'fixture_files': {}}
    for manifest_name, key, root in [('ycbcr-jpeg-fixtures.json', 'cases', DATA),
                                     ('decoder-geometry-fixtures.json', 'cases', DATA / 'decoder-geometry'),
                                     ('jpeg-reconstruction/provenance.json', 'cases', DATA)]:
        manifest = DATA / manifest_name
        report['fixture_files'][str(manifest)] = sha(manifest)
        for item in json.loads(manifest.read_text())[key]:
            for kind in ('jpeg', 'jxl'):
                path = root / item[kind]
                if sha(path) != item[kind + '_sha256']:
                    raise RuntimeError('Genuine original fixture bytes changed: ' + str(path))
                report['fixture_files'][str(path)] = sha(path)
    captures = json.loads((DATA / 'jpeg-reconstruction/native-captures.json').read_text())
    if len(captures) != 18:
        raise RuntimeError('Complete and partial native capture inventory changed')
    for graph in ('oracle', 'oracle-asan'):
        directory = output / graph
        directory.mkdir()
        build = ROOT / 'target' / graph
        binary = build / 'jxl_oracle'
        binding = audit_binding(build, directory)
        before = sha(binary)
        observations = directory / 'observations.txt.gz'
        with gzip.open(observations, 'wt', compresslevel=1) as target, rust.open() as expected:
            for start in range(0, len(requests), 4):
                for row in observe(binary, requests[start:start + 4]):
                    actual = expected.readline().rstrip('\n')
                    if actual != row:
                        raise RuntimeError(f'Original {graph} differs from retained Rust at {start}')
                    target.write(row + '\n')
            if expected.read():
                raise RuntimeError('Rust/source observation count changed')
        for capture_row in captures:
            wire = (DATA / capture_row['jxl']).read_bytes()
            if capture_row['remove_last_byte']:
                wire = wire[:-1]
            fields = DATA / capture_row['jpeg_data']
            original = DATA / capture_row['original']
            if sha(fields) != capture_row['jpeg_data_sha256'] or sha(original) != capture_row['original_sha256']:
                raise RuntimeError('Retained original capture bytes changed')
            row, = observe(binary, ['jr-frame ' + wire.hex() + ' ' + fields.read_text().strip()])
            if row != original.read_text().strip():
                raise RuntimeError('Native original complete/partial capture differs')
        undefined = []
        probes = [(f'dq-jpeg 0 1 0 1 -1 {transform} 0 {gray} 4 -9 0 0 {dc}',
                   'dec_group.cc:418:', 'outside the range of representable values')
                  for transform in (1, 2) for gray in (0, 1) for dc in (4, 5)]
        probes += [(f'dq-jpeg 0 1 0 {accumulate} -1 1 0 {gray} 4 -9 7 0 0',
                    'dec_group.cc:247:', 'division by zero')
                   for gray in (0, 1) for accumulate in (0, 1)]
        for request, location, diagnostic in probes:
            result = subprocess.run([str(binary)], input=request + '\n', text=True, capture_output=True)
            if graph == 'oracle-asan' and (result.returncode == 0 or location not in result.stderr or diagnostic not in result.stderr):
                raise RuntimeError('Original undefined caller domain lacks its positive sanitizer evidence')
            undefined.append({'request': request, 'returncode': result.returncode,
                              'stdout': result.stdout, 'stderr': result.stderr,
                              'classification': 'original undefined behavior; no Rust expected output'})
        (directory / 'undefined-source.json').write_text(json.dumps(undefined, indent=2) + '\n')
        if sha(binary) != before or any(sha(Path(path)) != digest for path, digest in binding['unchanged_inputs'].items()):
            raise RuntimeError('Original observation modified active build inputs')
        report['graphs'][graph] = {'binary_sha256': before, 'binding': binding,
                                   'observations_sha256': sha(observations), 'native_captures': 18,
                                   'undefined_source_probes': len(undefined)}
    (output / 'report.json').write_text(json.dumps(report, indent=2) + '\n')
    print(f'Original JPEG reconstruction: {len(requests)} exact requests, 162 files, 18 native captures, 9 live bindings per graph')


if __name__ == '__main__':
    main()
