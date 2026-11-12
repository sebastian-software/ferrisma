#!/usr/bin/env python3
"""Audit actual ICCReader sources, symbol bindings, and incremental observations.

The Rust differential tests emit their complete real request corpus. Temporary
links retain every active object and option, adding only a source-binding map.
Both unchanged original graphs must agree on every initialized observation.
"""
import argparse
import gzip
import hashlib
import json
import os
from pathlib import Path
import platform
import re
import shlex
import subprocess

ROOT = Path(__file__).resolve().parent.parent
METHODS = ('Init', 'Process', 'CheckEOI')


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def text_sha(text):
    return hashlib.sha256(text.encode()).hexdigest()


def normalize(text, substitutions, message):
    """Undo exactly the listed substitutions, each of which must occur once."""
    for new, old in substitutions:
        if text.count(new) != 1:
            raise RuntimeError(message)
        text = text.replace(new, old)
    return text


def verify_existing_methods():
    source = (ROOT / 'src/jxl/dec_ans.rs').read_text()
    suffix = source[source.index('    pub fn create(\n'):]
    # The pinned reader caches the mode scalar at construction. Only these
    # three pre-existing accesses and one constructor initializer may differ.
    initializer = '            use_prefix_code: code.use_prefix_code,\n'
    if suffix.count(initializer) != 1 or suffix.count('self.use_prefix_code') != 3:
        raise RuntimeError('Cached mode substitutions changed')
    if suffix.count('code.use_prefix_code') != 2:
        raise RuntimeError('Constructor argument mode checks changed')
    suffix = suffix.replace(initializer, '').replace('self.use_prefix_code', 'self.code.use_prefix_code')
    # Memory-manager propagation (PR #56): the LZ77 window allocates through
    # the code's memory manager. Only this one call differs.
    suffix = normalize(suffix, [
        ('            Some(AlignedMemory::create(\n                code.memory_manager,\n'
         '                WINDOW_SIZE * 4,\n                0,\n            )?)\n',
         '            Some(AlignedMemory::create(WINDOW_SIZE * 4, 0)?)\n'),
    ], 'ANS window allocation substitution changed')
    # Clippy 1.100 (2026-11-12): is_huff_rle_only gained one lint allowance for
    # its source index loop; its body is unchanged. Only this attribute differs.
    suffix = normalize(suffix, [
        ('    #[allow(clippy::needless_range_loop, reason = "Keep the source index loops.")]\n'
         '    pub fn is_huff_rle_only(',
         '    pub fn is_huff_rle_only('),
    ], 'ANS lint allowance substitution changed')
    # The Rust 1.95 lint expectation on read_hybrid_uint_clustered_inlined is the
    # only change since ac734e93...; without it the bodies hash as before.
    if text_sha(suffix) != '48b995a3edccb5d9669fb4028560c16853d1345fa08cf477b3452872313792a5':
        raise RuntimeError('Existing ANS decoding/checkpoint bodies changed')
    source = (ROOT / 'src/jxl/icc_codec.rs').read_text()
    commands = source[source.index('pub fn shuffle('):source.index('/// Incremental reader')].strip()
    # Memory-manager propagation (PR #56): Shuffle and UnpredictICC allocate
    # their PaddedBytes through the result's memory manager, as the source's
    # memory_manager arguments do. Only these eight sites differ.
    commands = normalize(commands, [
        ('pub fn shuffle(memory_manager: MemoryManagerPtr, data: &mut [u8], width: usize) -> Status {',
         'pub fn shuffle(data: &mut [u8], width: usize) -> Status {'),
        ('    let mut result = PaddedBytes::with_initial_space_in(memory_manager, size)?;',
         '    let mut result = PaddedBytes::with_initial_space(size)?;'),
        ('    let memory_manager = result.memory_manager();\n', ''),
        ('    let mut header = PaddedBytes::new(memory_manager);',
         '    let mut header = PaddedBytes::default();'),
        ('                    let mut shuffled =\n'
         '                        PaddedBytes::with_initial_space_in(memory_manager, num as usize)?;',
         '                    let mut shuffled = PaddedBytes::with_initial_space(num as usize)?;'),
        ('                    shuffle(\n                        memory_manager,\n'
         '                        &mut shuffled.bytes_mut()[..num as usize],\n                        if command',
         '                    shuffle(\n'
         '                        &mut shuffled.bytes_mut()[..num as usize],\n                        if command'),
        ('                let mut shuffled =\n'
         '                    PaddedBytes::with_initial_space_in(memory_manager, num as usize)?;',
         '                let mut shuffled = PaddedBytes::with_initial_space(num as usize)?;'),
        ('                    shuffle(\n                        memory_manager,\n'
         '                        &mut shuffled.bytes_mut()[..num as usize],\n                        width,\n'
         '                    )?;',
         '                    shuffle(&mut shuffled.bytes_mut()[..num as usize], width)?;'),
    ], 'ICC command memory-manager substitutions changed')
    if text_sha(commands) != '871d116b5173938e95d92512b81549fa3db5652547cc1b7034763489d04679a2':
        raise RuntimeError('Frozen ICC command prerequisites changed')
    return {'ans_methods_sha256': text_sha(suffix), 'icc_commands_sha256': text_sha(commands),
            'cached_mode_mapping': {'reader_accesses': 3, 'create_initializer': 1,
                                    'unchanged_create_mode_conditions': 1}}


def verify_map(contents):
    if platform.system() == 'Darwin':
        matches = re.findall(r'^\[\s*(\d+)\]\s+.*CMakeFiles/jxl_oracle.dir/icc_codec\.cc\.o$', contents, re.M)
        if len(matches) != 1:
            raise RuntimeError('Pristine ICC object absent from link map')
        for method in METHODS:
            mangled = f'_ZN3jxl9ICCReader{len(method)}{method}E'
            rows = [line for line in contents.splitlines() if mangled in line and 'FDE for:' not in line
                    and re.search(r'\[\s*' + matches[0] + r'\]\s+', line)]
            if len(rows) != 1:
                raise RuntimeError('ICC method binding changed: ' + method)
    else:
        current = ''
        bound = []
        for line in contents.splitlines():
            if 'CMakeFiles/jxl_oracle.dir/' in line and '.o' in line:
                current = line
            if line.rstrip().endswith(')') and re.search(r'^\s+0x[0-9a-f]+\s+', line) and any(
                    'jxl::ICCReader::' + method + '(' in line for method in METHODS):
                if 'CMakeFiles/jxl_oracle.dir/icc_codec.cc.o' not in current:
                    raise RuntimeError('ICC method bound outside pristine unit: ' + line)
                bound.append(line)
        if len(bound) != len(METHODS):
            raise RuntimeError('ICC public/private method bindings changed')


def bind(build, directory):
    entries = json.loads((build / 'compile_commands.json').read_text())
    receipt = {}
    for name in ('icc_codec.cc', 'icc_reader.cc'):
        entry, = [entry for entry in entries if Path(entry['file']).resolve() == ROOT / 'tools/oracle' / name]
        command = entry.get('arguments') or shlex.split(entry['command'])
        if '-O3' not in command or any(arg in command for arg in ('-O0', '-O1')):
            raise RuntimeError('Original ICC numeric optimization changed')
        if command.count('-fno-access-control') != int(name == 'icc_reader.cc'):
            raise RuntimeError('Private access observer scope changed')
        if ('-fsanitize=address,undefined' in command) != (build.name == 'oracle-asan'):
            raise RuntimeError('ICC sanitizer configuration changed')
        obj = build / f'CMakeFiles/jxl_oracle.dir/{name}.o'
        symbols = subprocess.check_output(['nm', '-C', str(obj)], text=True)
        (directory / (name + '.symbols.txt')).write_text(symbols)
        for method in METHODS:
            expected = (' T ' if name == 'icc_codec.cc' else ' U ') + 'jxl::ICCReader::' + method + '('
            if expected not in symbols:
                raise RuntimeError('Real ICC method definition/call missing: ' + expected)
        receipt[name] = {'command': command, 'object_sha256': sha(obj),
                         'observer_sha256': sha(ROOT / 'tools/oracle' / name)}
    link = shlex.split((build / 'CMakeFiles/jxl_oracle.dir/link.txt').read_text())
    binary, map_file = directory / 'jxl_oracle', directory / 'link.map'
    for name in ('icc_codec.cc', 'icc_reader.cc'):
        if link.count(f'CMakeFiles/jxl_oracle.dir/{name}.o') != 1:
            raise RuntimeError('Duplicate or missing ICC observation object')
    link[link.index('-o') + 1] = str(binary)
    link.append(('-Wl,-map,' if platform.system() == 'Darwin' else '-Wl,-Map,') + str(map_file))
    with (directory / 'link.log').open('w') as log:
        subprocess.run(link, cwd=build, stdout=log, stderr=subprocess.STDOUT, check=True)
    verify_map(map_file.read_text())
    receipt['link'] = link
    (directory / 'commands.json').write_text(json.dumps(receipt, indent=2) + '\n')
    return binary, receipt


def records(line):
    fields, at = line.split(), 0
    while at < len(fields):
        if fields[at] != 'S':
            raise RuntimeError('Unknown ICC observer record')
        fixed = fields[at + 1:at + 17]
        op, init, process, bits, close, index, skip, base, encoded, size, capacity, defined = map(int, fixed[:12])
        output = fixed[14]
        contexts = int(fixed[15])
        at += 17 + contexts
        sizes = list(map(int, fields[at:at + 3]))
        ans = list(map(int, fields[at + 3:at + 18]))
        has_window, window = map(int, fields[at + 18:at + 20])
        at += 20 + window
        same_window, same_storage, reset = map(int, fields[at:at + 3])
        at += 3
        cached = None
        if at < len(fields) and fields[at] == 'C':
            cached = list(map(int, fields[at + 1:at + 6]))
            at += 6
        yield {'op': op, 'init': init, 'process': process, 'bits': bits, 'close': close,
               'i': index, 'skip': skip, 'base': base, 'encoded': encoded, 'size': size,
               'capacity': capacity, 'defined': defined, 'output': output, 'sizes': sizes,
               'ans': ans, 'has_window': has_window, 'window': window,
               'same_window': same_window, 'same_storage': same_storage, 'reset': reset, 'cached': cached}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--output', type=Path, default=ROOT / 'target/icc-reader-reference')
    args = parser.parse_args()
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=False)
    source = ROOT / 'upstream/libjxl'
    pin = json.loads((ROOT / 'upstream.json').read_text())['revision']
    revision = subprocess.check_output(['git', '-C', str(source), 'rev-parse', 'HEAD'], text=True).strip()
    dirty = subprocess.check_output(['git', '-C', str(source), 'status', '--porcelain', '--ignore-submodules=none'], text=True).strip()
    if revision != pin or dirty:
        raise RuntimeError('Pinned ICC source/dependencies changed')
    report = {'source_revision': revision, 'platform': platform.platform(),
              'port_revision': subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=ROOT, text=True).strip(),
              'preserved_methods': verify_existing_methods(),
              'source_hashes': {name: sha(source / 'lib/jxl' / name) for name in ('icc_codec.cc', 'icc_codec.h', 'dec_ans.h', 'dec_ans.cc')},
              'graphs': {}}
    capture = output / 'requests.txt'
    environment = dict(os.environ, JXL_ICC_READER_CAPTURE=str(capture),
                       CARGO_BUILD_JOBS='2', RUST_TEST_THREADS='2')
    command = ['cargo', 'test', '--locked', '--offline', '--all-features', '--test', 'parity', 'icc_reader_', '--', '--nocapture']
    with (output / 'capture.log').open('w') as log:
        subprocess.run(command, cwd=ROOT, env=environment, stdout=log, stderr=subprocess.STDOUT, check=True)
    cases = capture.read_text().splitlines()
    if not cases or any(not case.startswith(('ir-run ', 'ir-cache ')) for case in cases):
        raise RuntimeError('Actual reader request capture missing')
    report['requests'], report['requests_sha256'] = len(cases), sha(capture)
    from collections import Counter
    fixed = (ROOT / 'tests/data/icc-reader-cache.requests').read_text().splitlines()
    if len(fixed) != 10 or Counter(fixed) - Counter(cases):
        raise RuntimeError('Additive cached-mode source requests missing')
    report['cached_mode_requests_sha256'] = sha(ROOT / 'tests/data/icc-reader-cache.requests')
    report['cached_mode_requests'] = len(fixed)
    statistics = {'steps': 0, 'not_enough_bytes': 0, 'fatal': 0, 'rollback_prefix': 0,
                  'reset': 0, 'conditional_reset': 0, 'ans_steps': 0, 'huffman_steps': 0,
                  'maximum_window': 0, 'ratio_guard': 0, 'different_base': 0,
                  'cached_huffman_to_ans': 0, 'cached_ans_to_huffman': 0,
                  'unused_changed_huffman_view': 0}
    previous = None
    for graph in ('oracle', 'oracle-asan'):
        build, directory = ROOT / 'target' / graph, output / graph
        directory.mkdir()
        before = sha(build / 'jxl_oracle')
        binary, receipt = bind(build, directory)
        observation = directory / 'observations.txt.gz'
        with gzip.open(observation, 'wt', compresslevel=1) as stream:
            for start in range(0, len(cases), 8):
                chunk = cases[start:start + 8]
                result = subprocess.run([str(binary)], input='\n'.join(chunk) + '\n', text=True, capture_output=True, check=True)
                lines = result.stdout.splitlines()
                if result.stderr or len(lines) != len(chunk):
                    raise RuntimeError('Unexpected original ICC stderr/response count: ' + result.stderr)
                stream.write(result.stdout)
                if graph == 'oracle':
                    for line in lines:
                        for row in records(line):
                            cached = row['cached']
                            if cached is not None:
                                mode, owner, alias, huffman, configs = cached
                                if not configs or (huffman if mode else alias) != 1:
                                    raise RuntimeError('Reached cached source view is invalid')
                                statistics['cached_huffman_to_ans'] += mode == 1 and owner == 0
                                statistics['cached_ans_to_huffman'] += mode == 0 and owner == 1
                                statistics['unused_changed_huffman_view'] += mode == 0 and huffman == 0
                            statistics['steps'] += 1
                            statistics['not_enough_bytes'] += row['init'] == -1 or row['process'] == -1
                            statistics['fatal'] += row['init'] > 0 or row['process'] > 0
                            statistics['rollback_prefix'] += row['process'] == -1 and row['defined'] > row['i']
                            statistics['reset'] += row['op'] == 3
                            statistics['conditional_reset'] += row['reset']
                            statistics['ans_steps'] += row['ans'][1] > 0
                            statistics['huffman_steps'] += row['sizes'][0] > 0 and row['ans'][1] == 0
                            statistics['maximum_window'] = max(statistics['maximum_window'], row['window'])
                            statistics['ratio_guard'] += (row['init'] == 0 and row['process'] == 1 and row['i'] == 65536
                                and row['encoded'] == 65537 and row['output'] == '4567' and (row['bits'] - row['base']) < 2048)
                            statistics['different_base'] += row['base'] == 7 and row['op'] == 2
        if sha(build / 'jxl_oracle') != before:
            raise RuntimeError('Active ICC reference binary changed')
        report['graphs'][graph] = {'active_binary_sha256': before, 'binding': receipt, 'observations_sha256': sha(observation)}
        if previous is not None:
            with gzip.open(previous, 'rt') as left, gzip.open(observation, 'rt') as right:
                for index, request in enumerate(cases):
                    a, b = next(left).strip(), next(right).strip()
                    if a != b:
                        raise RuntimeError(f'Original ordinary/ASan ICC drift at {index}: {request}')
                if next(left, None) is not None or next(right, None) is not None:
                    raise RuntimeError('Unexpected trailing ICC observation')
        previous = observation
    for key in ('not_enough_bytes', 'fatal', 'rollback_prefix', 'reset', 'conditional_reset', 'ans_steps', 'huffman_steps', 'ratio_guard', 'different_base', 'cached_huffman_to_ans',
                'cached_ans_to_huffman', 'unused_changed_huffman_view'):
        if not statistics[key]:
            raise RuntimeError('Actual ICC source branch missing: ' + key)
    if statistics['maximum_window'] != 1 << 20:
        raise RuntimeError('Original LZ77 window wrap not reached')
    report['statistics'], report['audit_passed'] = statistics, True
    (output / 'report.json').write_text(json.dumps(report, indent=2) + '\n')
    print(f'{len(cases)} actual requests and {statistics["steps"]} original steps agree ordinary/ASan', flush=True)
    print(json.dumps(statistics, sort_keys=True), flush=True)
    print('Source and active original binaries remain unchanged; proof=' + str(output), flush=True)


if __name__ == '__main__':
    main()
