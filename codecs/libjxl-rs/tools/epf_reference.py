#!/usr/bin/env python3
"""Audit selected pristine scalar EPF stages against untouched default objects.

Only protocol generation and process orchestration live here. The default
observer omits the pristine stage include, so its real factory resolves from
the unchanged original archive. Every compiler/sanitizer/linker option remains
except this new observer's scoped -O0. Active build graphs are never replaced.
"""
import json
from pathlib import Path
import platform
import shlex
import struct
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parent.parent


def bits(v):
    return struct.unpack('<I', struct.pack('<f', v))[0]


def next_word(seed):
    seed ^= (seed << 13) & ((1 << 64) - 1)
    seed ^= seed >> 7
    seed ^= (seed << 17) & ((1 << 64) - 1)
    return seed


DEFAULT = [1063675494, 1087373312, 1059760811, 1109393408,
           1084227584, 1080033280, 1055286886, 1058642330]
GEOMETRY = [(0, 0, 0, 0, 0, 0), (1, 2, 0, 3, 23, 41),
            (0, 0, 1, 0, 0, 0), (2, 3, 17, 1, 9, 7),
            (20, 7, 33, 0, 23, 8), (0, 0, 65, 0, 7, 15)]


def request(p, lf, geometry, values, sigmas):
    return ' '.join(map(str, ['epf-row', p, 3, *lf, *geometry, *values, *sigmas]))


def exceptional_requests():
    values = [[0] * 9, [0x80000000] * 9, [0x3f800000] * 9,
              [0, 0x80000000, 1, 0x80000001, 0x00800000, 0x7f7fffff,
               0xff7fffff, 0x3f800000, 0xbf800000],
              [0x7fa12345, 0xffa54321, 0x7fc98765, 0xffc43210,
               0x7f800000, 0xff800000, 0x3f000000, 0x3f800000, 0x80000000]]
    seed = 937
    for _ in range(12):
        row = []
        for _ in range(9):
            seed = next_word(seed)
            row.append(seed & 0xffffffff)
        values.append(row)
    filters = [DEFAULT, [0, 0, DEFAULT[2], 0, 0, 0, *DEFAULT[6:]],
               [bits(v) for v in [0.9, 3.7, 0.3, 0.25, -0.5, 2.0]] + DEFAULT[6:],
               DEFAULT[:3] + [0x7fa12345, 0xffc98765, 0x7f800000] + DEFAULT[6:]]
    sigmas = [[bits(v) for v in [-10., -4., -3.]],
              [0xc079ef80, bits(-0.0001), bits(-1.)],
              [0x7fa12345, 0xff800000, 0xffc98765]]
    return [request(p, lf, geom, row, sigma)
            for p in range(3) for lf in filters for row in values
            for geom in GEOMETRY for sigma in sigmas]


def finite_requests():
    # Same active-weight and threshold-neighbor requests as the Rust fixture.
    filters = [DEFAULT,
               [bits(v) for v in [0.9, 6.5, 0.67, 0.01, 0.02, 0.03]] + DEFAULT[6:],
               [0x3f666665, 0x40d00001, bits(1.9), bits(0.0001), bits(0.003),
                bits(0.2), 0x7f800000, 0x7fa12345]]
    geometry = [(0, 0, 1, 0, 0, 0), (1, 1, 7, 2, 9, 1),
                (2, 3, 8, 1, 9, 7), (0, 0, 9, 0, 7, 8),
                (3, 2, 31, 5, 23, 15), (20, 7, 65, 0, 23, 17)]
    minimum = bits(-3.905243)
    sigmas = [minimum + 1, minimum, minimum - 1, bits(-0.001)]
    seed, cases = 7891, []
    for p in range(3):
        for lf in filters:
            for _ in range(8):
                row = []
                for _ in range(9):
                    seed = next_word(seed)
                    signed = seed & 0xffffffff
                    if signed >= 1 << 31:
                        signed -= 1 << 32
                    remainder = signed % 1024 if signed >= 0 else -((-signed) % 1024)
                    row.append(bits(remainder / 131072.))
                for geom in geometry:
                    for sigma in sigmas:
                        cases.append(request(p, lf, geom, row, [sigma] * 3))
    return cases


def tiny_requests():
    patterns = [
        [0, 0x80000000, 1, 0x80000001, 2, 0x80000002, 3, 0x80000003, 0],
        [0x007fffff, 0x807fffff, 0x00800000, 0x80800000, 0x00800001, 0x80800001, 1, 0x80000001, 0],
        [0x3f800000, 0xbf800000, 0x3f000000, 0xbf000000, 0, 0x80000000, 1, 0x80000001, 0x3f800001],
        [0x7f7fffff, 0xff7fffff, 0x3f800000, 0xbf800000, 0, 0x80000000, 1, 0x80000001, 0x00800000],
        [1] * 9, [0x80000001] * 9, [0] * 9, [0x80000000] * 9]
    scales = [[bits(1.)] * 3, [0] * 3, [1, 0x00800000, bits(0.5)]]
    geometry = [(0, 0, 1, 0, 0, 0), (1, 2, 7, 3, 23, 1),
                (2, 2, 8, 1, 9, 7), (20, 7, 17, 0, 23, 8)]
    return [request(p, DEFAULT[:3] + scale + DEFAULT[6:], geom, row,
                    list(map(bits, [-1., -0.001, -3.])))
            for p in range(3) for row in patterns for scale in scales
            for geom in geometry]


def parameter_requests():
    words = [0, 0x80000000, 1, 0x80000001, 0x00800000, 0x7f7fffff,
             0xff7fffff, 0x7f800000, 0xff800000, 0x7fc12345, 0xffc98765,
             0x7fa12345, 0xffa98765]
    pixels = [[0x3f800000] * 9,
              [0x7fa12345, 0xffa54321, 0x7fc98765, 0xffc43210,
               0x7f800000, 0xff800000, 0x3f000000, 0x3f800000, 0x80000000]]
    cases = []
    for p in range(3):
        for slot in range(8):
            for word in words:
                lf = DEFAULT.copy()
                lf[slot] = word
                for row in pixels:
                    for geom in [(1, 2, 3, 3, 23, 41), (0, 0, 1, 0, 0, 0)]:
                        cases.append(request(p, lf, geom, row, [bits(-1.)] * 3))
    return cases


def is_finite(request):
    words = request.split()
    # zeroflush arguments are unused by pristine Weight; keep their exceptional
    # requests in the audit while classifying the actual arithmetic operands.
    operands = words[3:9] + words[17:29]
    return all(int(v) & 0x7f800000 != 0x7f800000 for v in operands)


def observe(binary, cases):
    result = subprocess.run([str(binary)], input='\n'.join(cases) + '\n',
                            text=True, capture_output=True, check=True)
    lines = result.stdout.splitlines()
    if len(lines) != len(cases):
        raise RuntimeError('EPF observation count changed')
    return lines


def default_binary(build, temporary):
    entries = json.loads((build / 'compile_commands.json').read_text())
    entry, = [e for e in entries if Path(e['file']).resolve() == ROOT / 'tools/oracle/epf.cc']
    command = shlex.split(entry['command'])
    if command.count('-O0') != 1 or '-O3' not in command:
        raise RuntimeError('Unexpected selected EPF compiler configuration')
    command.remove('-O0')
    command.insert(1, '-DJXL_EPF_DEFAULT_REFERENCE=1')
    obj, binary = temporary / 'epf.cc.o', temporary / 'jxl_oracle'
    command[command.index('-o') + 1] = str(obj)
    for i, arg in enumerate(command):
        if arg == '-MF':
            command[i + 1] = str(temporary / 'epf.cc.o.d')
        elif arg.startswith('-MF'):
            command[i] = '-MF' + str(temporary / 'epf.cc.o.d')
    subprocess.run(command, cwd=entry['directory'], check=True)
    selected_object = build / 'CMakeFiles/jxl_oracle.dir/epf.cc.o'
    selected_symbols = subprocess.check_output(['nm', '-C', str(selected_object)], text=True)
    default_symbols = subprocess.check_output(['nm', '-C', str(obj)], text=True)
    if ' T jxl::GetEPFStage(' not in selected_symbols or ' U jxl::GetEPFStage(' not in default_symbols:
        raise RuntimeError('EPF selected/default factory binding changed')
    archive_entry, = [e for e in entries if Path(e['file']).resolve() ==
                     ROOT / 'upstream/libjxl/lib/jxl/render_pipeline/stage_epf.cc']
    archive_command = shlex.split(archive_entry['command'])
    if '-O3' not in archive_command or '-O0' in archive_command:
        raise RuntimeError('Original EPF archive compiler configuration changed')
    link = shlex.split((build / 'CMakeFiles/jxl_oracle.dir/link.txt').read_text())
    index, = [i for i, arg in enumerate(link) if arg == 'CMakeFiles/jxl_oracle.dir/epf.cc.o']
    link[index] = str(obj)
    link[link.index('-o') + 1] = str(binary)
    subprocess.run(link, cwd=build, check=True)
    return binary


def audit():
    cases = exceptional_requests() + finite_requests() + tiny_requests() + parameter_requests()
    if len(cases) != 6936:
        raise RuntimeError('EPF corpus coverage changed')
    finite = list(map(is_finite, cases))
    names = ('oracle', 'oracle-asan')
    # The audit runs on native targets, as required by the oracle contract.
    # Account for the source's architecture-dependent row prefix in captures.
    offset = 24 if platform.machine().lower() in ('arm', 'arm64', 'aarch64') else 32
    captured_word = 2 * offset + 13
    with tempfile.TemporaryDirectory(prefix='jxl-epf-reference-') as temporary:
        temporary = Path(temporary)
        selected, defaults = [], []
        for name in names:
            directory = temporary / name
            directory.mkdir()
            build = ROOT / 'target' / name
            selected.append(build / 'jxl_oracle')
            defaults.append(default_binary(build, directory))
        for start in range(0, len(cases), 8):
            chunk = cases[start:start + 8]
            previous = None
            for name, original, default in zip(names, selected, defaults):
                actual, baseline = observe(original, chunk), observe(default, chunk)
                for local, (chosen, source) in enumerate(zip(actual, baseline)):
                    index = start + local
                    if finite[index] and chosen != source:
                        raise RuntimeError(f'Finite EPF source drift in {name}, case {index}')
                    if index == 1299:
                        print(f'{name}: default/selected request1299 word{captured_word}='
                              f'{source.split()[captured_word]}/{chosen.split()[captured_word]}', flush=True)
                if previous is not None and actual != previous:
                    raise RuntimeError(f'Selected ordinary/ASan EPF observations differ in cases {start}..{start+len(chunk)-1}')
                previous = actual
    for name in names:
        print(f'{name}: {sum(finite)} finite EPF requests match default archive source', flush=True)
    print(f'Selected ordinary/ASan EPF stages agree on {len(cases)} requests', flush=True)


if __name__ == '__main__':
    audit()
