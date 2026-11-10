#!/usr/bin/env python3
"""Run the unchanged original static allocation/thread-failure fuzzer with C/Rust.

Four original 8, 10, and 12-bit and film-grain payloads receive 512
reserved-header variants each, which controls the original input-derived fault
seed. Each profile runs 2,048 paired scenarios and requires observed calls and
failures for malloc, aligned allocation, thread creation, condition
initialization, and mutex initialization in both implementations. Sampling does
not enumerate every failure position or thread schedule.

The static fault-fuzzer bridge needs GNU objcopy on Linux, or LLVM objcopy
under LIBAVIF_RS_LLVM_PREFIX on macOS (the task-local LLVM 16 prefix by
default). `--asan` currently targets macOS ARM64 only. ADR 0034 documents the
object rewriting, runtime isolation, and recovery of compiler-folded
malloc and zeroing operations. These test adapters are absent from production
builds.
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys

from check_dav1d_api import instrumented_libraries

ROOT = Path(__file__).resolve().parents[1]
HOOKS = ['malloc', 'posix_memalign', 'pthread_create', 'pthread_cond_init', 'pthread_mutex_init']
FIXTURES = ['8-bit/data/00000000.ivf', '8-bit/film_grain/clip_0.ivf',
            '10-bit/data/00000943.ivf', '12-bit/data/00000791.ivf']


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--debug', action='store_true')
    parser.add_argument('--asan', action='store_true')
    args = parser.parse_args()
    if args.asan and args.debug:
        parser.error('The sanitizer profile retains original Release preprocessing')
    profile = 'debug' if args.debug else 'release'
    darwin = sys.platform == 'darwin'
    work = ROOT / (f'build/dav1d-faults-{profile}' + ('-asan' if args.asan else ''))
    work.mkdir(parents=True, exist_ok=True)
    environment = os.environ.copy()
    compiler = os.environ.get('CC', 'cc')
    native = ['-lpthread', '-lm', *(['-lresolv', '-liconv'] if darwin else ['-ldl', '-lrt', '-lutil'])]
    runtime = []
    reference = ROOT / f'build/dav1d-reference-{profile}'
    archive = ROOT.parents[1] / f'target/{profile}/libdav1d.a'
    database = json.loads((reference / 'compile_commands.json').read_text())
    originals = [Path(entry['directory']) / entry['output'] for entry in database if entry['output'].startswith('src/libdav1d')]
    if len(originals) != 51:
        raise SystemExit('Missing complete original decoder closure')
    for path in (ROOT / 'upstream/dav1d/src').rglob('*'):
        if path.suffix in ['.c', '.h'] and re.search(r'\bcalloc\s*\(', path.read_text()):
            raise SystemExit('Explicit source calloc needs a separate original fault-hook contract')
    if args.asan:
        c_archive, archive, compiler, environment, runtime = instrumented_libraries(work)
        originals = []
        for member in subprocess.check_output(['ar', '-t', c_archive], text=True).splitlines():
            if not member.endswith('.o'):
                continue  # Apple's ar lists its symbol index as an archive member.
            path = work / member
            path.write_bytes(subprocess.check_output(['ar', '-p', c_archive, member]))
            originals.append(path)
        if len(originals) != 51:
            raise SystemExit('Missing instrumented original decoder objects')
    prefix = Path(os.environ.get('LIBAVIF_RS_LLVM_PREFIX', ROOT / 'build/toolchain/llvm@16/16.0.6_1'))
    objcopy = str(prefix / 'bin/llvm-objcopy') if darwin else shutil.which('objcopy')
    if not objcopy or not Path(objcopy).is_file():
        raise SystemExit('The original fault-fuzzer bridge requires objcopy')
    copy_environment = environment.copy()
    if darwin:
        copy_environment['DYLD_LIBRARY_PATH'] = str(prefix / 'lib') + ':/opt/homebrew/lib'
    leading = '_' if darwin else ''
    changes = [argument for name in HOOKS for argument in ['--redefine-sym', f'{leading}{name}={leading}__wrap_{name}']]
    changes += ['--redefine-sym', f'{leading}calloc={leading}port_wrap_folded_calloc']

    def rewrite(paths, label):
        output = work / label
        output.mkdir(exist_ok=True)
        results = []
        for index, path in enumerate(paths):
            target = output / f'{index:03d}.o'
            subprocess.run([objcopy, *changes, str(path), str(target)], env=copy_environment, check=True)
            results.append(target)
        symbols = subprocess.check_output(['nm', '-u', *results], text=True)
        if any(line.split()[-1] in {leading + name for name in HOOKS + ['calloc']} for line in symbols.splitlines() if line.split()):
            raise SystemExit('An original failure-hook reference remains in decoder objects')
        return results

    c_objects = rewrite(originals, 'c-objects')
    members = [member for member in subprocess.check_output(['ar', '-t', archive], text=True).splitlines()
               if member.startswith('dav1d.') and member.endswith('.rcgu.o')]
    if not members:
        raise SystemExit('Missing Rust decoder codegen objects')
    extracted = work / 'rust-original-objects'
    extracted.mkdir(exist_ok=True)
    rust_originals = []
    for member in members:
        path = extracted / member
        path.write_bytes(subprocess.check_output(['ar', '-p', archive, member]))
        rust_originals.append(path)
    rust_objects = rewrite(rust_originals, 'rust-objects')
    source = ROOT / 'upstream/dav1d/tests/libfuzzer'
    flags = ['-std=c11', '-g', '-D_GNU_SOURCE', '-DDAV1D_ALLOC_FAIL', '-ffp-contract=off', '-pthread',
             '-I' + str(reference), '-I' + str(ROOT / 'upstream/dav1d'),
             '-I' + str(ROOT / 'upstream/dav1d/include')]
    flags += ['-O0'] if args.debug else ['-O2', '-DNDEBUG']
    if args.asan:
        flags += ['-fsanitize=address', '-fno-omit-frame-pointer']
    caller = []
    original_callers = ['dav1d_fuzzer.c', 'main.c', 'alloc_fail.c']
    source_record = json.loads((ROOT / 'provenance/dav1d-source.json').read_text())
    for filename in original_callers:
        path = source / filename
        if digest(path) != source_record['files'][f'tests/libfuzzer/{filename}']:
            raise SystemExit('Modified original fault-fuzzer source')
        target = work / (path.stem + '.o')
        definitions = [f'-D__wrap_{name}=port_original_wrap_{name}' for name in HOOKS] if filename == 'alloc_fail.c' else []
        subprocess.run([compiler, *flags, *definitions, '-c', path, '-o', target], env=environment, check=True)
        caller.append(target)
    observer = ROOT / 'tools/dav1d-fault-runtime.c'
    target = work / 'observer.o'
    subprocess.run([compiler, *flags, '-c', observer, '-o', target], env=environment, check=True)
    caller.append(target)
    for label, objects in [('c', c_objects), ('rust', rust_objects)]:
        linkmap = work / f'{label}-link.map'
        mapping = f'-Wl,-map,{linkmap}' if darwin else f'-Wl,-Map,{linkmap}'
        # All rewritten decoder objects are explicit. The original Rust archive
        # supplies only runtime/dependency objects; no decoder member may load.
        subprocess.run([compiler, *caller, *objects, *([archive] if label == 'rust' else []),
                        *native, *runtime, mapping, '-o', work / label], env=environment, check=True)
        if label == 'rust' and re.search(re.escape(archive.name.encode()) + rb'\(dav1d\.', linkmap.read_bytes()):
            raise SystemExit('An unwrapped original Rust decoder object entered the test')
    test_inputs = json.loads((ROOT / 'provenance/dav1d-test-inputs.json').read_text())['dav1d-test-data']
    inputs = work / 'inputs'
    inputs.mkdir(exist_ok=True)
    totals = {label: {'calls': [0] * 5, 'failures': [0] * 5, 'folded_calloc_calls': 0} for label in ['c', 'rust']}
    records = []
    for fixture in FIXTURES:
        original = (ROOT / test_inputs['source_root'] / fixture).read_bytes()
        if hashlib.sha256(original).hexdigest() != test_inputs['files'][fixture]:
            raise SystemExit('Modified original fault-fuzzer input')
        for variant in range(512):
            content = original[:28] + variant.to_bytes(4, 'little') + original[32:]
            path = inputs / (fixture.replace('/', '-') + f'-{variant}.ivf')
            path.write_bytes(content)
            row = {'fixture': fixture, 'header_variant': variant, 'input_sha256': digest(path)}
            for label in ['c', 'rust']:
                result = subprocess.run([work / label, path], env=environment, text=True,
                                        stdout=subprocess.PIPE, stderr=subprocess.STDOUT, timeout=30)
                matches = re.findall(r'^\[port-faults\] (\{[^\n]+\})$', result.stdout, re.M)
                if result.returncode or len(matches) != 1:
                    (work / 'failure.log').write_text(result.stdout)
                    raise SystemExit(f'Original fault fuzzer failed: {profile}/{fixture}/{variant}/{label}: {result.returncode}')
                counts = json.loads(matches[0])
                row[label] = counts
                for key in ['calls', 'failures']:
                    totals[label][key] = [left + right for left, right in zip(totals[label][key], counts[key])]
                totals[label]['folded_calloc_calls'] += counts['folded_calloc_calls']
            records.append(row)
    if any(value <= 0 for counts in totals.values() for key in ['calls', 'failures'] for value in counts[key]):
        raise SystemExit(f'Missing call or failure coverage for an original hook: {totals}')
    report = {'profile': profile, 'sanitizer': 'address' if args.asan else None,
              'caller_instrumented': args.asan, 'paired_scenarios': len(records), 'header_variants': 512,
              'hooks': HOOKS, 'totals': totals, 'original_callers_sha256': {name: digest(source / name) for name in original_callers},
              'observer_sha256': digest(observer), 'candidate_archive_sha256': digest(archive),
              'rewritten_rust_codegen_objects': len(members), 'unwrapped_decoder_objects': [],
              'records': records}
    (work / 'summary.json').write_text(json.dumps(report, indent=2) + '\n')
    print(f'{len(records)} paired original fault-fuzzer scenarios; all five failure classes observed in C and Rust')


if __name__ == '__main__':
    main()
