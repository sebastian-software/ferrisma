#!/usr/bin/env python3
"""Audit the actual pinned output factory, memory callbacks and source domain.

Temporary relinks add a source-object map only. Original undefined requests are
retained individually with positive nonrecovering sanitizer diagnostics. No
pixel conversion, decoder, allocator or callback algorithm is reimplemented.
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

ROOT = Path(__file__).resolve().parent.parent


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def git(path, *args):
    return subprocess.check_output(['git', '-C', str(path), *args], text=True).strip()


def observe(binary, requests):
    result = subprocess.run([str(binary)], input='\n'.join(requests) + '\n',
                            text=True, capture_output=True)
    rows = result.stdout.splitlines()
    if result.returncode or result.stderr or len(rows) != len(requests):
        raise RuntimeError(f'Original observation failed ({result.returncode}): {result.stderr}')
    return rows


def configuration(build, directory):
    entries = json.loads((build / 'compile_commands.json').read_text())
    commands = {}
    for name in ('output_stage.cc', 'frame_sequence_writes.cc', 'memory_manager_internal.cc'):
        entry, = [e for e in entries if Path(e['file']).name == name]
        command = entry.get('arguments') or shlex.split(entry['command'])
        if '-O3' not in command or any(x in command for x in ('-O0', '-O1', '-O2', '-ffast-math')):
            raise RuntimeError('Original output configuration changed: ' + name)
        sanitized = build.name == 'oracle-asan'
        if ('-fsanitize=address,undefined' in command) != sanitized:
            raise RuntimeError('Sanitizer configuration changed: ' + name)
        if sanitized and '-fno-sanitize-recover=all' not in command:
            raise RuntimeError('Recovering original undefined behavior: ' + name)
        if name != 'memory_manager_internal.cc' and '-fno-access-control' not in command:
            raise RuntimeError('Missing scoped observer visibility: ' + name)
        commands[name] = command
    included = (ROOT / 'tools/oracle/frame_sequence_writes.cc').read_text()
    if included.count('#include "lib/jxl/render_pipeline/stage_write.cc"') != 1 or re.search(
            r'^\s*#\s*define\s+GetWriteToOutputStage\b', included, re.M):
        raise RuntimeError('Actual public output factory was replaced')
    link = shlex.split((build / 'CMakeFiles/jxl_oracle.dir/link.txt').read_text())
    for name in ('output_stage.cc', 'frame_sequence_writes.cc'):
        if link.count('CMakeFiles/jxl_oracle.dir/' + name + '.o') != 1:
            raise RuntimeError('Missing or duplicate output observer: ' + name)
    binary, mapping = directory / 'bound', directory / 'link.map'
    link[link.index('-o') + 1] = str(binary)
    link.append(('-Wl,-map,' if platform.system() == 'Darwin' else '-Wl,-Map,') + str(mapping))
    with (directory / 'link.log').open('w') as log:
        subprocess.run(link, cwd=build, stdout=log, stderr=subprocess.STDOUT, check=True)
    stage = 'frame_sequence_writes.cc.o'
    manager = 'memory_manager_internal.cc.o'
    expected = {
        'factory': ('jxl::GetWriteToOutputStage(', stage),
        'process': ('jxl::N_SCALAR::WriteToOutputStage::ProcessRow(', stage),
        'prepare': ('jxl::N_SCALAR::WriteToOutputStage::PrepareForThreads(', stage),
        'mode': ('jxl::N_SCALAR::WriteToOutputStage::GetChannelMode(', stage),
        'name': ('jxl::N_SCALAR::WriteToOutputStage::GetName(', stage),
        'destroy': ('jxl::N_SCALAR::WriteToOutputStage::~WriteToOutputStage(', stage),
        'manager_init': ('jxl::MemoryManagerInit(', manager),
        'manager_alloc': ('jxl::MemoryManagerAlloc(', manager),
        'manager_free': ('jxl::MemoryManagerFree(', manager),
        'aligned_create': ('jxl::AlignedMemory::Create(', manager),
    }
    contents = mapping.read_text()
    rows = []
    if platform.system() == 'Darwin':
        objects = dict(re.findall(r'^\[\s*(\d+)\]\s+(.+)$', contents, re.M))
        live = []
        for line in contents.splitlines():
            row = re.match(r'^0x[0-9A-Fa-f]+\s+0x[0-9A-Fa-f]+\s+\[\s*(\d+)\]\s+(\S+)$', line)
            if row and 'FDE' not in line:
                live.append((objects[row[1]], row[2], line))
        names = subprocess.check_output(['c++filt'], input='\n'.join(
            symbol[1:] if symbol.startswith('__Z') else symbol for _, symbol, _ in live) + '\n', text=True).splitlines()
        rows = [(owner, name, original) for (owner, _, original), name in zip(live, names)]
    else:
        current = ''
        for line in contents.splitlines():
            # GNU maps place the contributing input object on its section line,
            # followed by the live demangled symbol's address line.
            if re.search(r'\.o\)?\s*$', line):
                current = line
            if re.match(r'^\s+0x[0-9a-f]+\s+', line):
                rows.append((current, line.strip().split(None, 1)[-1], line))
    found = {}
    for owner, name, original in rows:
        for key, (prefix, source) in expected.items():
            if name.startswith(prefix):
                if source not in owner:
                    raise RuntimeError('Wrong original binding: ' + original + ' in ' + owner)
                found.setdefault(key, []).append({'object': owner, 'symbol': original})
    if set(found) != set(expected):
        raise RuntimeError('Missing live original bindings: ' + str(set(expected) - set(found)))
    # The scalar helper factory may be inlined/dead-stripped. The public
    # factory and all reached vtable methods above must remain genuinely live.
    return {'commands': commands, 'link': link, 'bindings': found,
            'map_sha256': sha(mapping), 'include_sha256': sha(ROOT / 'tools/oracle/frame_sequence_writes.cc')}


def standard_library(command, directory):
    flags = []
    for i, flag in enumerate(command):
        if flag.startswith('-stdlib=') or flag.startswith('-std='):
            flags.append(flag)
        if flag == '-isysroot':
            flags.extend((flag, command[i + 1]))
    if platform.system() == 'Linux' and '-stdlib=libc++' not in flags:
        raise RuntimeError('Linux output cleanup requires the configured libc++ reference')
    compiler = subprocess.check_output([command[0], '--version'], text=True)
    macros = subprocess.check_output([command[0], *flags, '-dM', '-E', '-x', 'c++', '-include', 'vector', '-'], input='', text=True)
    if not re.search(r'^#define _LIBCPP_VERSION \d+', macros, re.M) or '__GLIBCXX__' in macros:
        raise RuntimeError('Output vector cleanup is not using actual libc++')
    (directory / 'stdlib-macros.txt').write_text(macros)
    source = directory / 'vector-cleanup.cc'
    source.write_text('#include <vector>\n#include <cstdio>\nstruct Item {int n; ~Item(){std::printf("%d ", n);}};\nint main(){{std::vector<Item> v(3);for(int i=0;i<3;i++)v[i].n=i+1;}return 0;}\n')
    binary = directory / 'vector-cleanup'
    subprocess.run([command[0], *flags, '-O3', str(source), '-o', str(binary)], check=True, capture_output=True)
    order = subprocess.check_output([str(binary)], text=True)
    if order != '3 2 1 ':
        raise RuntimeError('Actual standard-library vector destruction order differs: ' + order)
    return {'compiler': compiler, 'flags': flags,
            'libcpp_version': re.search(r'^#define _LIBCPP_VERSION (\d+)', macros, re.M)[1],
            'destruction_order': order, 'source_sha256': sha(source),
            'macros_sha256': sha(directory / 'stdlib-macros.txt')}


def original_only_depth_requests():
    for bits in (31, 32, 33, 63, 2**64 - 1):
        for kind in (2, 3):
            for channels in range(1, 5):
                for orientation in (1, 2, 5, 8):
                    for mode in (0, 1):
                        yield bits, (f'wo-run 3 2 0 0 {2**64-1} {orientation} 0 0 '
                                     f'{kind} {bits} {channels} {mode} 3 0 '
                                     '2 0 1 1 0 0 3 0 0 0 ' + ' '.join(['1065353216'] * 9))


def initial_depth_controls():
    # Retain the entire first 192-request depth-31 extension. Two/four-channel
    # requests synthesize alpha=1 and are undefined even for input zero; the
    # other 96 remain exact members of the active defined corpus.
    for kind in (2, 3):
        for channels in range(1, 5):
            for orientation in (1, 2, 5, 8):
                for mode in (0, 1):
                    for word in (0, 0x3f000000, 0x3f7fffff):
                        yield channels, (f'wo-run 3 2 0 0 {2**64-1} {orientation} 0 0 '
                                         f'{kind} 31 {channels} {mode} 3 0 '
                                         '3 0 2 1 0 0 3 0 0 0 1 0 0 3 0 1 1 ' +
                                         ' '.join([str(word)] * 9))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=False)
    pin = json.loads((ROOT / 'upstream.json').read_text())
    sources = [ROOT / pin['path']] + [ROOT / pin['path'] / 'third_party' / name for name in ('highway', 'brotli', 'skcms')]
    pins = {}
    for source in sources:
        wanted = pin['revision'] if source == sources[0] else git(sources[0], 'ls-tree', 'HEAD', 'third_party/' + source.name).split()[2]
        if git(source, 'rev-parse', 'HEAD') != wanted or git(source, 'status', '--porcelain', '--ignore-submodules=none'):
            raise RuntimeError('Pinned source changed: ' + str(source))
        pins[str(source.relative_to(ROOT))] = wanted
    capture = output / 'requests.txt'
    env = dict(os.environ, CARGO_BUILD_JOBS='2', RUST_TEST_THREADS='2', JXL_DECODER_TARGET='scalar', JXL_OUTPUT_STAGE_CAPTURE=str(capture))
    with (output / 'capture.log').open('w') as log:
        subprocess.run(['cargo', 'test', '--locked', '--offline', '--all-features', '--test', 'parity',
                        'output_stage::', '--', '--nocapture'], cwd=ROOT, env=env, stdout=log, stderr=subprocess.STDOUT, check=True)
    requests = capture.read_text().splitlines()
    counts = Counter(row.split()[0] for row in requests)
    if counts != Counter({'wo-run': 3812, 'wo-mm': 192}):
        raise RuntimeError('Actual output corpus inventory changed: ' + str(counts))
    fixtures = ROOT / 'tests/data/output-stage-native.tsv'
    native = [line.split('\t') for line in fixtures.read_text().splitlines() if line and not line.startswith('#')]
    manifest = json.loads((ROOT / 'tests/data/output-stage-native.json').read_text())
    if sha(fixtures) != manifest['fixture_sha256'] or len(native) != 9 or manifest['source_revision'] != pin['revision']:
        raise RuntimeError('Immutable native source captures changed')
    report = {'pins': pins, 'requests': len(requests), 'request_kinds': counts,
              'requests_sha256': sha(capture), 'native_captures': len(native),
              'native_capture_sha256': sha(fixtures), 'graphs': {}}
    previous = None
    for graph in ('oracle', 'oracle-asan'):
        build, directory = ROOT / 'target' / graph, output / graph
        directory.mkdir()
        paths = [build / 'jxl_oracle', build / 'compile_commands.json',
                 build / 'CMakeFiles/jxl_oracle.dir/link.txt']
        paths += [build / 'CMakeFiles/jxl_oracle.dir' / (n + '.o') for n in ('output_stage.cc', 'frame_sequence_writes.cc')]
        paths += [build / 'upstream/lib/libjxl-internal.a']
        before = {str(p.relative_to(ROOT)): sha(p) for p in paths}
        configuration_report = configuration(build, directory)
        stdlib = standard_library(configuration_report['commands']['frame_sequence_writes.cc'], directory)
        observations = []
        with gzip.open(directory / 'observations.txt.gz', 'wt') as archive:
            for i in range(0, len(requests), 4):
                rows = observe(build / 'jxl_oracle', requests[i:i+4])
                observations.extend(rows)
                archive.write('\n'.join(rows) + '\n')
        if previous is not None and observations != previous:
            raise RuntimeError('Ordinary and sanitizer original observations differ')
        previous = observations
        for fields in native:
            request, expected = fields[-2:]
            if observe(build / 'jxl_oracle', [request]) != [expected]:
                raise RuntimeError('Immutable native original observation differs')
        if {str(p.relative_to(ROOT)): sha(p) for p in paths} != before:
            raise RuntimeError('Temporary evidence changed active binaries, objects or flags')
        report['graphs'][graph] = {'source_configuration': configuration_report,
                                   'standard_library': stdlib, 'active_hashes': before,
                                   'observation_sha256': hashlib.sha256(('\n'.join(observations)+'\n').encode()).hexdigest()}
    domain = []
    directory = output / 'original-only-depth'
    directory.mkdir()
    probes = [(bits, request, True) for bits, request in original_only_depth_requests()]
    initial = list(initial_depth_controls())
    probes += [(31, request, channels % 2 == 0) for channels, request in initial]
    # The second extension controlled four-channel alpha while leaving the
    # two-channel synthetic alpha unchanged. Retain all 48 new distinct
    # requests; the other 144 are already present in the first inventory.
    second = []
    for channels, request in initial:
        if channels == 4:
            words = request.split()
            words[3], words[5], words[7] = '1', '3', '1'
            words[15:15] = ['0', '16', '1', '3', '3', '0']
            second.append(' '.join(words))
    probes += [(31, request, False) for request in second]
    report['initial_depth31_retention'] = {'requests': len(initial), 'defined_still_exact': 96,
        'source_undefined_synthesized_alpha': 96, 'second_extension_new_defined': 48,
        'requests_sha256': hashlib.sha256(('\n'.join(row for _, row in initial)+'\n').encode()).hexdigest()}
    for index, (bits, request, undefined) in enumerate(probes):
        result = subprocess.run([str(ROOT / 'target/oracle-asan/jxl_oracle')], input=request+'\n', text=True, capture_output=True)
        (directory / f'{index:03d}.request').write_text(request+'\n')
        (directory / f'{index:03d}.stdout').write_text(result.stdout)
        (directory / f'{index:03d}.stderr').write_text(result.stderr)
        if not undefined:
            if request not in requests or result.returncode or result.stderr or observe(ROOT / 'target/oracle/jxl_oracle', [request]) != result.stdout.splitlines():
                raise RuntimeError('Initial defined depth-31 request was not retained exactly')
            positive = None
            classification = 'defined-still-exact'
        elif bits == 31:
            positive = re.search(r'hwy/base\.h:2459:27: runtime error: 2\.14748e\+09 is outside the range of representable values of type .int.', result.stderr)
            classification = 'source-undefined-float-to-int32'
        else:
            positive = re.search(r'stage_write\.cc:551:\d+: runtime error: shift exponent \d+ is too large for 32-bit type', result.stderr)
            classification = 'source-undefined-uint32-shift'
        if undefined and (result.returncode == 0 or not positive or 'SUMMARY: UndefinedBehaviorSanitizer:' not in result.stderr):
            raise RuntimeError(f'Missing positive original-only depth diagnostic {index}: {result.stderr}')
        domain.append({'index': index, 'bits': bits, 'request_sha256': hashlib.sha256((request+'\n').encode()).hexdigest(),
                       'returncode': result.returncode, 'classification': classification, 'diagnostic': positive[0] if positive else None,
                       'stderr_sha256': sha(directory / f'{index:03d}.stderr')})
    report['original_only_depth'] = {'requests': len(domain), 'classes': Counter(x['classification'] for x in domain), 'probes': domain}
    if Counter(x['classification'] for x in domain) != Counter({'source-undefined-float-to-int32': 160, 'source-undefined-uint32-shift': 256, 'defined-still-exact': 144}):
        raise RuntimeError('Source-only reached-domain census changed')
    report['source_sha256'] = {name: sha(ROOT / pin['path'] / name) for name in (
        'lib/jxl/render_pipeline/stage_write.h', 'lib/jxl/render_pipeline/stage_write.cc',
        'lib/jxl/memory_manager_internal.h', 'lib/jxl/memory_manager_internal.cc',
        'lib/include/jxl/memory_manager.h', 'third_party/highway/hwy/base.h',
        'third_party/highway/hwy/ops/scalar-inl.h')}
    report['storage_observation'] = 'Every initialized caller byte including padding/tail; callback data backed by actual returned custom allocations. Free captures omit only unchanged outer caller-supplied 0xa5 bytes, retaining every changed byte and intervening sentinel.'
    report['scope'] = 'Complete original scalar output-stage leaf and callback-backed allocator. Public incremental decoder orchestration and other custom allocator propagation remain separate.'
    (output / 'report.json').write_text(json.dumps(report, indent=2) + '\n')
    print(f'{len(requests)} exact requests, 9 native captures, {len(domain)} original-only depth census requests (416 positive UB, 144 retained defined); both pristine source graphs verified')


if __name__ == '__main__':
    main()
