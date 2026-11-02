#!/usr/bin/env python3
"""Diagnose or audit native original chroma without changing active graphs.

The baseline is each unchanged archive-backed oracle. Candidate binaries link
pristine stage translation units compiled with the exact archive commands plus
one explicitly reported optimization candidate. All sanitizer options remain.
Audit mode verifies the platform-scoped factory binding and selected graph.
This script records evidence; it does not select or install a new reference.
"""
import argparse
from collections import Counter
import gzip
import hashlib
import json
from pathlib import Path
import platform
import shlex
import shutil
import struct
import subprocess

ROOT = Path(__file__).resolve().parent.parent
SOURCES = ('stage_ycbcr.cc', 'stage_chroma_upsampling.cc')
FACTORIES = ('jxl::GetYCbCrStage(', 'jxl::GetChromaUpsamplingStage(')
WRAPPERS = ('chroma_ycbcr_stage.cc', 'chroma_upsampling_stage.cc')
IS_SELECTED_O0 = platform.system() == 'Linux' and platform.machine().lower() in ('x86_64', 'amd64')
OPTIONS = {'no-unroll': ['-fno-unroll-loops'], 'O1': ['-O1'], 'O0': ['-O0']}
GEOMETRY = [(0, 0, 0, 0), (0, 0, 1, 3), (0, 0, 7, 0),
            (1, 2, 0, 3), (2, 3, 8, 1), (10, 7, 33, 0)]


def bits(value):
    return struct.unpack('<I', struct.pack('<f', value))[0]


def next_word(seed):
    seed ^= (seed << 13) & ((1 << 64) - 1)
    seed ^= seed >> 7
    seed ^= (seed << 17) & ((1 << 64) - 1)
    return seed


def row(t, c, geometry, values):
    return ' '.join(map(str, ['cs-row', t, c, *geometry, *values]))


def requests():
    # Exact existing Rust corpus, including every exceptional request.
    cases = [f'cs-factory {t} {c}' for t in range(3)
             for c in (0, 1, 2, 3, 4, (1 << (struct.calcsize('P') * 8)) - 1)]
    patterns = [[0] * 9, [0x80000000] * 9, [0x3f800000] * 9,
                [0, 0x80000000, 1, 0x80000001, 0x007fffff, 0x00800000,
                 0x7f7fffff, 0xff7fffff, 0x3f800001],
                [0x7fa12345, 0xffa54321, 0x7fc98765, 0xffc43210,
                 0x7f800000, 0xff800000, 0x3f000000, 0x3f800000, 0x80000000]]
    seed = 7919
    for _ in range(20):
        values = []
        for _ in range(9):
            seed = next_word(seed)
            values.append(seed & 0xffffffff)
        patterns.append(values)
    for _ in range(12):
        values = []
        for _ in range(9):
            seed = next_word(seed)
            signed = seed & 0xffffffff
            if signed >= 1 << 31:
                signed -= 1 << 32
            remainder = signed % 1024 if signed >= 0 else -((-signed) % 1024)
            values.append(bits(remainder / 512.))
        patterns.append(values)
    cases += [row(t, c, geometry, values) for t in range(3) for c in range(5)
              for values in patterns for geometry in GEOMETRY]
    cases += [f'cs-simple {mask} {w} {h} 2 {mode} {reverse} 2'
              for mask in (64, 1, 8, 9, 5, 40, 45, 63, 109, 127)
              for w, h in ((1, 1), (2, 3), (17, 9), (65, 3), (255, 2),
                           (257, 3), (3, 257), (513, 1))
              for mode in (0, 1, 3) for reverse in (0, 1)]
    words = [0, 0x80000000, 1, 0x80000001, 0x00800000, 0x7f7fffff,
             0xff7fffff, 0x7f800000, 0xff800000, 0x7fc12345, 0xffc98765,
             0x7fa12345, 0xffa98765]
    cases += [row(t, 2, geometry, [a, b, 0x3f000000] * 3)
              for t in range(3) for a in words for b in words
              for geometry in ((0, 0, 3, 0), (2, 3, 7, 1))]
    if len(cases) != 4842:
        raise RuntimeError('Existing chroma corpus count changed')
    # Additional finite tiny values, cancellation pairs, and signed zeros.
    tiny = [[0, 0x80000000, 1, 0x80000001, 2, 0x80000002, 3, 0x80000003, 0],
            [0x007fffff, 0x807fffff, 0x00800000, 0x80800000, 0x00800001,
             0x80800001, 1, 0x80000001, 0],
            [bits(v) for v in (1., -1., .5, -.5, 0., -0., 1., -.5, .5)],
            [0x7f7fffff, 0xff7fffff, 0x3f800000, 0xbf800000, 0,
             0x80000000, 1, 0x80000001, 0x00800000]]
    tiny += [[word] * 9 for word in (0, 0x80000000, 1, 0x80000001)]
    cases += [row(t, c, geometry, values) for t in range(3) for c in range(5)
              for values in tiny for geometry in GEOMETRY]
    if len(cases) != 5562:
        raise RuntimeError('Chroma diagnostic corpus count changed')
    return cases


def finite(request):
    fields = request.split()
    if fields[0] == 'cs-factory':
        return True
    if fields[0] == 'cs-simple':
        return fields[5] != '3'
    return all(int(v) & 0x7f800000 != 0x7f800000 for v in fields[7:])


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def run(command, directory, log):
    log.write('+ ' + shlex.join(map(str, command)) + '\n')
    log.flush()
    subprocess.run(list(map(str, command)), cwd=directory, stdout=log,
                   stderr=subprocess.STDOUT, check=True)


def symbols(path):
    return subprocess.check_output(['nm', '-C', str(path)], text=True)


def original_link(build, directory, objects):
    link = shlex.split((build / 'CMakeFiles/jxl_oracle.dir/link.txt').read_text())
    selected = {f'CMakeFiles/jxl_oracle.dir/{name}.o' for name in WRAPPERS}
    # Remove only the new selected wrappers; every old observer/library/linker
    # option remains byte-for-byte identical to the established graph.
    link = [arg for arg in link if arg not in selected]
    link[1:1] = list(map(str, objects))
    binary = directory / 'jxl_oracle'
    link[link.index('-o') + 1] = str(binary)
    link.insert(1, ('-Wl,-Map,' if platform.system() == 'Linux' else '-Wl,-map,') + str(directory / 'link.map'))
    return link, binary


def default_binary(build, directory):
    directory.mkdir()
    link, binary = original_link(build, directory, [])
    with (directory / 'build.log').open('w') as log:
        run(link, build, log)
    (directory / 'commands.json').write_text(json.dumps({'link': link}, indent=2) + '\n')
    return binary


def selected_binding(build, directory):
    entries = json.loads((build / 'compile_commands.json').read_text())
    result = {'kind': 'isolated-O0' if IS_SELECTED_O0 else 'unchanged-archive'}
    link = shlex.split((build / 'CMakeFiles/jxl_oracle.dir/link.txt').read_text())
    for wrapper, source, factory in zip(WRAPPERS, SOURCES, FACTORIES):
        entry, = [e for e in entries if Path(e['file']).resolve() == ROOT / 'tools/oracle' / wrapper]
        command = entry.get('arguments') or shlex.split(entry['command'])
        if command.count('-O0') != int(IS_SELECTED_O0):
            raise RuntimeError('Selected chroma wrapper optimization changed')
        if ('-fsanitize=address,undefined' in command) != (build.name == 'oracle-asan'):
            raise RuntimeError('Selected chroma wrapper sanitizer options changed')
        relative = f'CMakeFiles/jxl_oracle.dir/{wrapper}.o'
        if link.count(relative) != 1:
            raise RuntimeError('Selected wrapper link binding changed')
        obj = build / relative
        observed = symbols(obj)
        if IS_SELECTED_O0:
            if ' T ' + factory not in observed or 'ProcessRow(' not in observed:
                raise RuntimeError('Selected wrapper no longer defines the real stage/factory')
        else:
            if factory in observed:
                raise RuntimeError('Non-Linux-x86 factory must remain in original archive')
            original, = [e for e in entries if Path(e['file']).resolve() == ROOT / 'upstream/libjxl/lib/jxl/render_pipeline' / source]
            original_command = original.get('arguments') or shlex.split(original['command'])
            if '-O3' not in original_command or '-O0' in original_command:
                raise RuntimeError('Unchanged archive optimization changed')
            original_object = Path(original['directory']) / original_command[original_command.index('-o') + 1]
            archive_symbols = symbols(original_object)
            if ' T ' + factory not in archive_symbols:
                raise RuntimeError('Original archive no longer defines the actual factory')
            (directory / (source + '.archive.symbols.txt')).write_text(archive_symbols)
        (directory / (wrapper + '.symbols.txt')).write_text(observed)
        result[wrapper] = {'object_sha256': sha(obj), 'wrapper_sha256': sha(ROOT / 'tools/oracle' / wrapper), 'command': command}
    return result


def compile_candidate(build, directory, options):
    entries = json.loads((build / 'compile_commands.json').read_text())
    observer, = [e for e in entries if Path(e['file']).resolve() ==
                 ROOT / 'tools/oracle/chroma_stages.cc']
    observer_object = build / 'CMakeFiles/jxl_oracle.dir/chroma_stages.cc.o'
    observer_symbols = symbols(observer_object)
    if any(' U ' + factory not in observer_symbols for factory in FACTORIES):
        raise RuntimeError('Original observer no longer calls archive factories')
    objects, commands = [], []
    with (directory / 'build.log').open('w') as log:
        for source, factory in zip(SOURCES, FACTORIES):
            entry, = [e for e in entries if Path(e['file']).resolve() ==
                      ROOT / 'upstream/libjxl/lib/jxl/render_pipeline' / source]
            command = entry.get('arguments') or shlex.split(entry['command'])
            if '-O3' not in command or any(flag in command for flag in ('-O0', '-O1')):
                raise RuntimeError('Default archive chroma optimization changed')
            sanitized = '-fsanitize=address,undefined' in command
            if sanitized != (build.name == 'oracle-asan'):
                raise RuntimeError('Chroma source sanitizer configuration changed')
            obj = directory / (source + '.o')
            command = list(command) + options
            command[command.index('-o') + 1] = str(obj)
            for i, arg in enumerate(command):
                if arg == '-MF':
                    command[i + 1] = str(obj) + '.d'
                elif arg.startswith('-MF'):
                    command[i] = '-MF' + str(obj) + '.d'
            run(command, entry['directory'], log)
            observed_symbols = symbols(obj)
            (directory / (source + '.symbols.txt')).write_text(observed_symbols)
            if ' T ' + factory not in observed_symbols:
                raise RuntimeError('Candidate does not define actual original factory')
            if 'ProcessRow(' not in observed_symbols:
                raise RuntimeError('Candidate lacks actual original stage method')
            # Keep source instructions for independently checking operand order.
            if platform.system() == 'Linux' and shutil.which('objdump'):
                with (directory / (source + '.asm')).open('w') as output:
                    subprocess.run(['objdump', '-d', '-C', str(obj)], stdout=output, check=True)
            objects.append(str(obj))
            commands.append(command)
        link, binary = original_link(build, directory, objects)
        run(link, build, log)
    (directory / 'commands.json').write_text(json.dumps({'compile': commands, 'link': link}, indent=2) + '\n')
    return binary


def observe(binary, cases, output):
    with gzip.open(output, 'wt', compresslevel=1) as stream:
        for start in range(0, len(cases), 32):
            chunk = cases[start:start + 32]
            result = subprocess.run([str(binary)], input='\n'.join(chunk) + '\n',
                                    text=True, capture_output=True, check=True)
            if len(result.stdout.splitlines()) != len(chunk):
                raise RuntimeError('Chroma observation count changed')
            if result.stderr:
                # Sanitizer diagnostics are never discarded or normalized.
                raise RuntimeError('Unexpected original stderr: ' + result.stderr)
            stream.write(result.stdout)


def compare(first, second, cases, finite_only=False):
    mismatches, examples, shown = Counter(), [], Counter()
    with gzip.open(first, 'rt') as a, gzip.open(second, 'rt') as b:
        for index, request in enumerate(cases):
            left, right = next(a).strip(), next(b).strip()
            if finite_only and not finite(request):
                continue
            if left != right:
                fields = request.split()
                group = fields[0] + ('-' + fields[1] if fields[0] == 'cs-row' else '')
                mismatches[group] += 1
                if shown[group] < 8:
                    shown[group] += 1
                    values = list(zip(left.split(), right.split()))
                    examples.append({'group': group, 'index': index, 'request': request,
                                     'differences': [(i, x, y) for i, (x, y) in enumerate(values) if x != y][:8],
                                     'word_counts': [len(left.split()), len(right.split())]})
        if next(a, None) is not None or next(b, None) is not None:
            raise RuntimeError('Unexpected trailing observation')
    return {'mismatches': sum(mismatches.values()), 'by_group': dict(mismatches), 'examples': examples}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--output', type=Path, default=ROOT / 'target/chroma-reference')
    parser.add_argument('--candidates', nargs='+', choices=OPTIONS, default=list(OPTIONS))
    parser.add_argument('--emit-only', action='store_true')
    parser.add_argument('--audit', action='store_true', help='Require selected original stages to agree, including finite/default equivalence')
    args = parser.parse_args()
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=False)
    cases = requests()
    (output / 'requests.txt').write_text('\n'.join(cases) + '\n')
    if args.emit_only:
        print(f'{len(cases)} diagnostic requests, including all 4842 existing requests')
        return
    source = ROOT / 'upstream/libjxl'
    pin = json.loads((ROOT / 'upstream.json').read_text())['revision']
    revision = subprocess.check_output(['git', '-C', str(source), 'rev-parse', 'HEAD'], text=True).strip()
    dirty = subprocess.check_output(['git', '-C', str(source), 'status', '--porcelain', '--untracked-files=normal', '--ignore-submodules=none'], text=True).strip()
    if revision != pin or dirty:
        raise RuntimeError('Pinned original source or dependency tree changed')
    report = {'machine': platform.platform(), 'source_revision': revision,
              'port_revision': subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=ROOT, text=True).strip(),
              'requests': len(cases),
              'old_requests': 4842, 'finite_requests': sum(map(finite, cases)),
              'source_hashes': {s: sha(ROOT / 'upstream/libjxl/lib/jxl/render_pipeline' / s) for s in SOURCES},
              'graphs': {}, 'comparisons': {}}
    for graph in ('oracle', 'oracle-asan'):
        build = ROOT / 'target' / graph
        directory = output / graph
        directory.mkdir()
        binary = build / 'jxl_oracle'
        before = sha(binary)
        entries = json.loads((build / 'compile_commands.json').read_text())
        source_entry, = [e for e in entries if Path(e['file']).resolve() == source / 'lib/jxl/render_pipeline/stage_ycbcr.cc']
        compiler = (source_entry.get('arguments') or shlex.split(source_entry['command']))[0]
        archive = build / 'upstream/lib/libjxl-internal.a'
        metadata = {'compiler_version': subprocess.check_output([compiler, '--version'], text=True),
                    'selected_binary_sha256': before,
                    'archive_sha256': sha(archive),
                    'observer_sha256': sha(build / 'CMakeFiles/jxl_oracle.dir/chroma_stages.cc.o'),
                    'compile_database_sha256': sha(build / 'compile_commands.json')}
        (directory / 'compile_commands.json').write_bytes((build / 'compile_commands.json').read_bytes())
        (directory / 'link.txt').write_bytes((build / 'CMakeFiles/jxl_oracle.dir/link.txt').read_bytes())
        print(f'{graph}: observing unchanged default archive', flush=True)
        baseline = default_binary(build, directory / 'default')
        metadata['default_binary_sha256'] = sha(baseline)
        observe(baseline, cases, directory / 'default.txt.gz')
        if args.audit:
            metadata['selected_binding'] = selected_binding(build, directory)
            observe(binary, cases, directory / 'selected.txt.gz')
        for name in (['O0'] if args.audit else args.candidates):
            candidate = directory / name
            candidate.mkdir()
            print(f'{graph}: compiling and observing pristine {name} candidate', flush=True)
            selected = compile_candidate(build, candidate, OPTIONS[name])
            observe(selected, cases, candidate / 'observations.txt.gz')
        if sha(binary) != before or sha(archive) != metadata['archive_sha256']:
            raise RuntimeError('Active oracle was modified')
        report['graphs'][graph] = metadata
    def record(name, left, right, finite_only=False):
        report['comparisons'][name] = compare(output / left, output / right, cases, finite_only)
        result = report['comparisons'][name]
        print(f'{name}: {result["mismatches"]} mismatching requests {result["by_group"]}', flush=True)
    record('default ordinary-vs-asan', 'oracle/default.txt.gz', 'oracle-asan/default.txt.gz')
    for name in (['O0'] if args.audit else args.candidates):
        record(name + ' ordinary-vs-asan', f'oracle/{name}/observations.txt.gz', f'oracle-asan/{name}/observations.txt.gz')
        for graph in ('oracle', 'oracle-asan'):
            for baseline in ('oracle', 'oracle-asan'):
                record(f'{name} {graph} finite-vs-{baseline}', f'{graph}/{name}/observations.txt.gz', f'{baseline}/default.txt.gz', True)
    if args.audit:
        record('selected ordinary-vs-asan', 'oracle/selected.txt.gz', 'oracle-asan/selected.txt.gz')
        for graph in ('oracle', 'oracle-asan'):
            if IS_SELECTED_O0:
                record(f'selected {graph} vs-independent-O0', f'{graph}/selected.txt.gz', f'{graph}/O0/observations.txt.gz')
            else:
                record(f'selected {graph} all-vs-default', f'{graph}/selected.txt.gz', f'{graph}/default.txt.gz')
            for baseline in ('oracle', 'oracle-asan'):
                record(f'selected {graph} finite-vs-{baseline}', f'{graph}/selected.txt.gz', f'{baseline}/default.txt.gz', True)
        failures = [name for name, result in report['comparisons'].items()
                    if name != 'default ordinary-vs-asan' and result['mismatches']]
        report['audit_passed'] = not failures
    else:
        failures = []
    (output / 'report.json').write_text(json.dumps(report, indent=2) + '\n')
    print(f'Evidence saved to {output}; active graphs and reference selection unchanged', flush=True)
    if failures:
        raise RuntimeError('Selected chroma acceptance failed: ' + ', '.join(failures))


if __name__ == '__main__':
    main()
