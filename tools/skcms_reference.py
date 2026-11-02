#!/usr/bin/env python3
"""Audit the actual pinned skcms transfer/matrix source and retained domains.

Both optimized original graphs retain all compiler and nonrecovering sanitizer
options. Temporary evidence links only add maps; no numeric body is supplied.
"""
import argparse
from collections import Counter
import gzip
import hashlib
import json
from pathlib import Path
import os
import platform
import re
import shlex
import struct
import subprocess

ROOT = Path(__file__).resolve().parent.parent
FUNCTIONS = [
    'skcms_TransferFunction_' + name for name in (
        'getType', 'eval', 'invert', 'makePQish', 'makeScaledHLGish', 'makePQ',
        'makeHLG', 'isSRGBish', 'isPQish', 'isHLGish', 'isPQ', 'isHLG')
] + ['skcms_' + name for name in (
    'Matrix3x3_invert', 'Matrix3x3_concat', 'AdaptToXYZD50', 'PrimariesToXYZD50')]


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def git(path, *args):
    return subprocess.check_output(['git', '-C', str(path), *args], text=True).strip()


def observe(binary, requests):
    result = subprocess.run([str(binary)], input='\n'.join(requests) + '\n',
                            text=True, capture_output=True)
    rows = result.stdout.splitlines()
    if result.returncode or result.stderr or len(rows) != len(requests):
        raise RuntimeError(f'Original observation failed ({result.returncode}): {result.stderr[:1500]}')
    return rows


def bindings(contents, binary):
    found = {}
    if platform.system() == 'Darwin':
        objects_text = contents.split('# Object files:\n', 1)[1].split('# Sections:', 1)[0]
        objects = dict(re.findall(r'^\[\s*(\d+)\]\s+(.+)$', objects_text, re.M))
        live = contents.split('# Symbols:\n', 1)[1].split('# Dead Stripped Symbols:', 1)[0]
        for address, size, index, name in re.findall(
                r'^(0x[0-9A-Fa-f]+)\s+(0x[0-9A-Fa-f]+)\s+\[\s*(\d+)\]\s+_(skcms_\w+)\s*$', live, re.M):
            if name in FUNCTIONS:
                if name in found or int(size, 16) == 0:
                    raise RuntimeError('Duplicate or empty live original function: ' + name)
                found[name] = {'address': int(address, 16), 'owner': objects[index]}
    else:
        live = contents.split('Linker script and memory map\n', 1)[1]
        section, owner = None, None
        for line in live.splitlines():
            start = re.match(r'^ (\.\S+)(?:\s+(.*))?$', line)
            if start:
                section, owner = start[1], None
            extent = re.match(r'^\s+(?:\.\S+\s+)?(0x[0-9a-fA-F]+)\s+(0x[0-9a-fA-F]+)\s+(\S+\.a\([^()]+\))\s*$', line)
            if extent and section and section.startswith('.text'):
                owner = (int(extent[1], 16), int(extent[2], 16), extent[3])
            symbol = re.match(r'^\s+(0x[0-9a-fA-F]+)\s+(skcms_\w+)\s*$', line)
            if symbol and owner and symbol[2] in FUNCTIONS:
                address, name = int(symbol[1], 16), symbol[2]
                if not owner[0] <= address < owner[0] + owner[1] or name in found:
                    raise RuntimeError('Incorrect live original address range: ' + name)
                found[name] = {'address': address, 'owner': owner[2]}
            if line and not line[0].isspace():
                section, owner = None, None
    if set(found) != set(FUNCTIONS):
        raise RuntimeError('Missing original public bindings: ' + str(set(FUNCTIONS) - set(found)))
    symbols = subprocess.check_output(['nm', str(binary)], text=True)
    for name, data in found.items():
        addresses = re.findall(r'^([0-9a-fA-F]+) [Tt] _?' + re.escape(name) + r'$', symbols, re.M)
        if len(addresses) != 1 or int(addresses[0], 16) != data['address']:
            raise RuntimeError('Original live map and executable addresses differ: ' + name)
        if not re.search(r'libjxl_cms\.a(?:\[\d+\])?\(skcms\.cc\.o\)$', data['owner']):
            # Mach-O may retain the original member's absolute source path.
            if 'libjxl_cms.a' not in data['owner'] or not data['owner'].endswith('/skcms.cc.o)'):
                raise RuntimeError('Incorrect original archive/object binding: ' + name + ' in ' + data['owner'])
    return found


def configuration(build, directory):
    entries = json.loads((build / 'compile_commands.json').read_text())
    commands, objects = {}, []
    for name in ('skcms.cc', 'skcms_transfer.cc', 'skcms_matrix.cc'):
        entry, = [e for e in entries if Path(e['file']).name == name]
        command = entry.get('arguments') or shlex.split(entry['command'])
        if [x for x in command if x.startswith('-O')] != ['-O3'] or '-DNDEBUG' not in command or '-ffast-math' in command:
            raise RuntimeError('Original skcms optimization changed: ' + name)
        if any('ffp-model' in x or 'ffp-contract' in x for x in command):
            raise RuntimeError('Original skcms numerical configuration changed: ' + name)
        sanitized = build.name == 'oracle-asan'
        wanted = ['-fsanitize=address,undefined', '-fno-sanitize-recover=all'] if sanitized else []
        if [x for x in command if 'sanitize' in x] != wanted:
            raise RuntimeError('Original nonrecovering instrumentation changed: ' + name)
        commands[name] = command
        obj = Path(command[command.index('-o') + 1])
        objects.append(obj if obj.is_absolute() else Path(entry['directory']) / obj)
    link = shlex.split((build / 'CMakeFiles/jxl_oracle.dir/link.txt').read_text())
    for name in ('skcms_transfer.cc', 'skcms_matrix.cc'):
        if link.count('CMakeFiles/jxl_oracle.dir/' + name + '.o') != 1:
            raise RuntimeError('Missing or duplicate skcms observer: ' + name)
    active = [build / 'jxl_oracle', build / 'compile_commands.json',
              build / 'CMakeFiles/jxl_oracle.dir/link.txt', build / 'upstream/lib/libjxl_cms.a', *objects]
    hashes = {str(p.relative_to(ROOT)): sha(p) for p in active}
    binary, mapping = directory / 'bound', directory / 'link.map'
    link[link.index('-o') + 1] = str(binary)
    link.append(('-Wl,-map,' if platform.system() == 'Darwin' else '-Wl,-Map,') + str(mapping))
    with (directory / 'link.log').open('w') as log:
        subprocess.run(link, cwd=build, stdout=log, stderr=subprocess.STDOUT, check=True)
    bound = bindings(mapping.read_text(), binary)
    dump = (['otool', '-tvV'] if platform.system() == 'Darwin' else ['objdump', '-d', '-C'])
    with gzip.open(directory / 'source-object.asm.gz', 'wb') as assembly:
        assembly.write(subprocess.check_output([*dump, str(objects[0])]))
    return {'commands': commands, 'link': link, 'bindings': bound,
            'active_hashes': hashes, 'map_sha256': sha(mapping),
            'source_assembly_sha256': sha(directory / 'source-object.asm.gz')}


def retained_matrix_requests():
    path = ROOT / 'tests/data/skcms-matrix-requests.bin'
    manifest = json.loads((ROOT / 'tests/data/skcms-matrix-requests.json').read_text())
    if sha(path) != manifest['fixture_sha256']:
        raise RuntimeError('Retained matrix fixture changed')
    data, offset, rows = path.read_bytes(), 0, []
    commands = ('skcms-concat', 'skcms-invert', 'skcms-adapt', 'skcms-primaries')
    while offset < len(data):
        tag, flag = data[offset:offset + 2]
        n = (18, 18, 11, 17)[tag]
        words = struct.unpack_from('<' + 'I' * n, data, offset + 2)
        rows.append(commands[tag] + ' ' + ' '.join(map(str, (flag, *words))))
        offset += 2 + 4 * n
    wire = ('\n'.join(rows) + '\n').encode()
    if len(rows) != 71226 or hashlib.sha256(wire).hexdigest() != manifest['original_request_text_sha256']:
        raise RuntimeError('Original matrix preflight requests were not retained exactly')
    return rows


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=False)
    pin = json.loads((ROOT / 'upstream.json').read_text())
    source = ROOT / pin['path']
    sources = [source] + [source / 'third_party' / n for n in ('highway', 'brotli', 'skcms')]
    pins = {}
    for path in sources:
        wanted = pin['revision'] if path == source else git(source, 'ls-tree', 'HEAD', 'third_party/' + path.name).split()[2]
        if git(path, 'rev-parse', 'HEAD') != wanted or git(path, 'status', '--porcelain', '--ignore-submodules=none'):
            raise RuntimeError('Pinned original source changed: ' + str(path))
        pins[str(path.relative_to(ROOT))] = wanted
    report = {'platform': platform.platform(), 'pins': pins, 'graphs': {}}
    for graph in ('oracle', 'oracle-asan'):
        directory = output / graph
        directory.mkdir()
        report['graphs'][graph] = configuration(ROOT / 'target' / graph, directory)
    tf_capture, matrix_capture = output / 'transfer.requests', output / 'matrix.requests'
    env = dict(os.environ, CARGO_BUILD_JOBS='2', RUST_TEST_THREADS='2',
               JXL_CPP_ORACLE=str(ROOT / 'target/oracle/jxl_oracle'),
               JXL_SKCMS_TRANSFER_CAPTURE=str(tf_capture),
               JXL_SKCMS_MATRIX_CAPTURE=str(matrix_capture))
    with (output / 'capture.log').open('w') as log:
        # Keep this audit scoped to its two public operation families. Other
        # skcms modules have independently configured private source observers.
        for module in ('skcms_transfer::', 'skcms_matrix::'):
            subprocess.run(['cargo', 'test', '--locked', '--offline', '--all-features', '--test',
                            'parity', module, '--', '--nocapture'], cwd=ROOT, env=env,
                           stdout=log, stderr=subprocess.STDOUT, check=True)
    transfer, matrix = tf_capture.read_text().splitlines(), matrix_capture.read_text().splitlines()
    if Counter(x.split()[0] for x in transfer) != Counter({'skcms-tf': 89838, 'skcms-tf-in-place': 24310, 'skcms-make': 5120}):
        raise RuntimeError('Transfer-function corpus inventory changed')
    if Counter(x.split()[0] for x in matrix) != Counter({'skcms-concat': 24324, 'skcms-invert': 40524, 'skcms-adapt': 10290, 'skcms-primaries': 12306}):
        raise RuntimeError('Matrix corpus inventory changed')
    retained = retained_matrix_requests()
    if Counter(retained) - Counter(matrix):
        raise RuntimeError('A prior matrix request was removed')
    previous = None
    requests = transfer + matrix
    for graph in ('oracle', 'oracle-asan'):
        observations = []
        with gzip.open(output / graph / 'observations.txt.gz', 'wt') as archive:
            for start in range(0, len(requests), 4096):
                rows = observe(ROOT / 'target' / graph / 'jxl_oracle', requests[start:start + 4096])
                observations.extend(rows)
                archive.write('\n'.join(rows) + '\n')
        if previous is not None and observations != previous:
            raise RuntimeError('Ordinary and ASan/UBSan defined source observations differ')
        previous = observations
        report['graphs'][graph]['defined_observations_sha256'] = hashlib.sha256(('\n'.join(observations) + '\n').encode()).hexdigest()
    exceptional = ROOT / 'tests/data/skcms-transfer-exceptional.tsv'
    fields = [row.split('\t', 1) for row in exceptional.read_text().splitlines()]
    undefined = [wire for kind, wire in fields if kind == 'U']
    if len(undefined) != 362 or Counter(wire for kind, wire in fields if kind == 'D') - Counter(transfer):
        raise RuntimeError('Original exceptional domain census changed')
    directory = output / 'source-only'
    directory.mkdir()
    probes = []
    for i, wire in enumerate(undefined):
        for graph in ('oracle', 'oracle-asan'):
            result = subprocess.run([str(ROOT / 'target' / graph / 'jxl_oracle')], input=wire + '\n', text=True, capture_output=True)
            for suffix, data in [('request', wire + '\n'), ('stdout', result.stdout), ('stderr', result.stderr)]:
                (directory / f'{i:03}-{graph}.{suffix}').write_text(data)
            if graph == 'oracle-asan' and (result.returncode == 0 or not re.search(
                    r'skcms_internals\.h:149:\d+: runtime error:.*outside the range of representable values', result.stderr)):
                raise RuntimeError('Missing positive original float-to-int diagnostic: ' + str(i))
            probes.append({'index': i, 'graph': graph, 'returncode': result.returncode,
                           'stderr_sha256': hashlib.sha256(result.stderr.encode()).hexdigest()})
    for graph, data in report['graphs'].items():
        if any(sha(ROOT / name) != digest for name, digest in data['active_hashes'].items()):
            raise RuntimeError('Evidence changed an active original binary/object/archive/command')
    report.update({'defined_requests': len(requests), 'transfer_requests': len(transfer),
                   'matrix_requests': len(matrix), 'retained_matrix_preflight': len(retained),
                   'transfer_requests_sha256': sha(tf_capture), 'matrix_requests_sha256': sha(matrix_capture),
                   'original_only_requests': len(undefined), 'original_only_probes': probes,
                   'source_sha256': {name: sha(source / 'third_party/skcms' / name) for name in (
                       'skcms.cc', 'src/skcms_public.h', 'src/skcms_internals.h')},
                   'scope': 'Pinned transfer-function and matrix operations. ICC parser, curve fitting, pixel transform, JXL default CMS and complete-image profile admission remain separate boundaries.'})
    (output / 'report.json').write_text(json.dumps(report, indent=2) + '\n')
    print(f'{len(requests)} exact defined requests; all 71,226 prior matrix requests retained; '
          '16 live public bindings in both original CMS archives; 362 positive original-only float-to-int boundaries')


if __name__ == '__main__':
    main()
