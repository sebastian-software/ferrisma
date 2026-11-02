#!/usr/bin/env python3
"""Compare actual pristine alpha objects without replacing an active reference.

This diagnostic does not choose an oracle configuration or accept mismatches.
It records the native selected/default graphs and temporary source-scoped
candidates, with the original sanitizer flags and public symbol binding intact.
"""
import hashlib
import json
from pathlib import Path
import platform
import shlex
import subprocess
import tempfile

from alpha_reference import ROOT, link, observe, requests, verify_binding, verify_rust_requests


def competing_requests():
    words = [0, 0x80000000, 0x3e800000, 0x3f800000, 0x7f800000,
             0xff800000, 0x7fc12345, 0xffc23456, 0x7fa12345, 0xffa23456]
    # Every pair of source operands competes in both orders, while the other
    # operands retain finite values. Each logical request has non-null backing.
    samples = []
    for first in range(4):
        for second in range(first + 1, 4):
            for a in words:
                for b in words:
                    row = [0x3f400000, 0x3f000000, 0x3e800000, 0x3f400000, 0x3f800000]
                    row[first], row[second] = a, b
                    samples.append(row)
    result = []
    for op in (0, 1):
        for premul in range(2):
            for clamp in range(2):
                # bi/bai/fi/fai map the four independently varied inputs.
                values = ['patch-alpha', op, premul, clamp, len(samples), 0, 1, 2, 3]
                values.extend(row[c] for c in range(5) for row in samples)
                result.append(' '.join(map(str, values)))
    return result


def differences(label, cases, left, right, record, finite=None):
    bad = []
    for i, (a, b) in enumerate(zip(left, right)):
        if finite is not None and not finite[i]:
            continue
        if a != b:
            aa, bb = a.split(), b.split()
            words = [(j, x, y) for j, (x, y) in enumerate(zip(aa, bb)) if x != y]
            bad.append({'index': i, 'request': cases[i], 'left': a, 'right': b,
                        'word_deltas': words, 'lengths': [len(aa), len(bb)]})
    count = sum(finite) if finite is not None else len(cases)
    print(f'{label}: {count-len(bad)}/{count} exact requests; {len(bad)} mismatches', flush=True)
    for row in bad[:2]:
        # The original 17-pixel requests are printed in full. Large additional
        # pair corpora remain complete in the JSON artifact, with exact deltas.
        request = row['request']
        print(f"{label} case {row['index']} request: "
              + (request if len(request) < 2500 else ' '.join(request.split()[:9])
                 + ' [complete request retained in target/alpha-diagnostics.json]'), flush=True)
        print(f"{label} case {row['index']} first word deltas (index,left,right): "
              + str(row['word_deltas'][:24]), flush=True)
    record[label] = {'compared': count, 'mismatches': bad}


def candidate(build, destination, options):
    destination.mkdir(parents=True)
    obj = destination / 'CMakeFiles/jxl_oracle.dir/alpha.cc.o'
    obj.parent.mkdir(parents=True)
    entries = json.loads((build / 'compile_commands.json').read_text())
    entry, = [e for e in entries if Path(e['file']).resolve() == ROOT / 'tools/oracle/alpha.cc']
    command = shlex.split(entry['command'])
    # Reproduce each explicit experiment even after the active selected unit
    # adopts strict-O0. Normalize only its scoped scalar options; all original
    # sanitizer, target, include and definition flags remain unchanged.
    command = [arg for arg in command if arg != '-ffp-model=strict']
    command[command.index('-o') + 1] = str(obj)
    # Some generators include dependency-output flags in compile_commands.
    # Candidate compilation must never replace an active dependency file.
    for index, arg in enumerate(command):
        if arg == '-MF':
            command[index + 1] = str(destination / 'alpha.cc.o.d')
        elif arg.startswith('-MF'):
            command[index] = '-MF' + str(destination / 'alpha.cc.o.d')
    command.extend(['-O3', *options])
    print('temporary pristine alpha compile: ' + shlex.join(command), flush=True)
    subprocess.run(command, cwd=build, check=True)
    command = shlex.split((build / 'CMakeFiles/jxl_oracle.dir/link.txt').read_text())
    command[command.index('CMakeFiles/jxl_oracle.dir/alpha.cc.o')] = str(obj)
    binary, map_file = destination / 'jxl_oracle', destination / 'selected.map'
    command[command.index('-o') + 1] = str(binary)
    command.append('-Wl,-map,' + str(map_file) if platform.system() == 'Darwin'
                   else '-Wl,-Map,' + str(map_file))
    subprocess.run(command, cwd=build, check=True)
    verify_binding(map_file.read_text())
    return binary


def capture_object(record, label, path, map_file=None):
    command = ['xcrun', 'llvm-objdump', '-d', '--demangle'] if platform.system() == 'Darwin' else [
        'objdump', '-d', '-C']
    record[label] = {
        'sha256': hashlib.sha256(path.read_bytes()).hexdigest(),
        'disassembly': subprocess.check_output(command + [str(path)], text=True),
    }
    if map_file is not None:
        record[label]['binding_map'] = map_file.read_text()


def main():
    print('native alpha diagnostic platform: ' + platform.platform(), flush=True)
    source = ROOT / 'upstream/libjxl/lib/jxl/alpha.cc'
    digest = hashlib.sha256(source.read_bytes()).hexdigest()
    if digest != '0935796743e0f4524bf0f506883d0bbc605f344bd8c939c6ba501b10c7c85b52':
        raise RuntimeError('Pinned original alpha source changed')
    cases, finite = requests()
    extra = competing_requests()
    all_cases = cases + extra
    all_finite = finite + [False] * len(extra)
    builds = [ROOT / 'target/oracle', ROOT / 'target/oracle-asan']
    active = [build / 'CMakeFiles/jxl_oracle.dir/alpha.cc.o' for build in builds]
    active += [path.with_suffix(path.suffix + '.d') for path in active
               if path.with_suffix(path.suffix + '.d').is_file()]
    receipts = [hashlib.sha256(path.read_bytes()).hexdigest() for path in active]
    record = {'platform': platform.platform(), 'source_sha256': digest,
              'original_requests': len(cases), 'additional_competing_requests': len(extra),
              'additional_competing_pixels': 600 * len(extra), 'comparisons': {},
              'objects': {}, 'candidate_options': {}}
    results = record['comparisons']
    with tempfile.TemporaryDirectory(prefix='jxl-alpha-diagnostic-') as directory:
        temporary = Path(directory)
        verify_rust_requests(cases, temporary)
        defaults, selected = [], []
        for name, build in zip(('ordinary', 'ASan'), builds):
            local = temporary / ('active-' + name)
            local.mkdir()
            defaults.append(observe(link(build, local, False), all_cases))
            selected.append(observe(link(build, local, True), all_cases))
            capture_object(record['objects'], 'active-selected-' + name,
                           build / 'CMakeFiles/jxl_oracle.dir/alpha.cc.o', local / 'selected.map')
            capture_object(record['objects'], 'untouched-default-' + name,
                           build / 'upstream/lib/CMakeFiles/jxl_dec-obj.dir/jxl/alpha.cc.o')
        differences('active selected ordinary/ASan', all_cases, *selected, results)
        for i, name in enumerate(('ordinary', 'ASan')):
            differences('active selected/default finite ' + name, all_cases,
                        selected[i], defaults[i], results, all_finite)
        # Preserve O3/flags in the first candidate; O1/O0 are progressively
        # broader source-scoped experiments. Strict FP is a separate source
        # operator-order experiment, with no instruction-set feature assumption.
        # No candidate replaces active files, algorithms or sanitizer flags.
        for label, options in [('no-unroll', ['-fno-unroll-loops']),
                               ('O1', ['-O1']), ('O0', ['-O0']),
                               ('strict-O3', ['-ffp-model=strict']),
                               ('strict-O0', ['-O0', '-ffp-model=strict'])]:
            record['candidate_options'][label] = ['-O3', *options]
            observed = []
            for name, build in zip(('ordinary', 'ASan'), builds):
                destination = temporary / (label + '-' + name)
                binary = candidate(build, destination, options)
                observed.append(observe(binary, all_cases))
                capture_object(record['objects'], label + '-' + name,
                               destination / 'CMakeFiles/jxl_oracle.dir/alpha.cc.o',
                               destination / 'selected.map')
            differences(label + ' ordinary/ASan', all_cases, *observed, results)
            for i, name in enumerate(('ordinary', 'ASan')):
                differences(label + '/default finite ' + name, all_cases,
                            observed[i], defaults[i], results, all_finite)
    if receipts != [hashlib.sha256(path.read_bytes()).hexdigest() for path in active]:
        raise RuntimeError('An active alpha object changed during diagnostics')
    output = ROOT / 'target/alpha-diagnostics.json'
    output.write_text(json.dumps(record, indent=2) + '\n')
    print('Full native requests, outputs and deltas retained in ' + str(output), flush=True)
    print('Active original objects remain byte-identical; no configuration selected.', flush=True)


if __name__ == '__main__':
    main()
