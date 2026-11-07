#!/usr/bin/env python3
"""Compare every single-allocation failure in small public encoder/decoder runs."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys

from build_aom import LIBRARY_UNITS, library_entries
from aom_sanitizer import instrumented_libraries

ROOT = Path(__file__).resolve().parents[1]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--debug', action='store_true')
    parser.add_argument('--asan', action='store_true')
    parser.add_argument('--thread-failures', action='store_true')
    args = parser.parse_args()
    if args.asan and args.debug:
        parser.error('The sanitizer configuration retains original Release preprocessing')
    profile = 'debug' if args.debug else 'release'
    darwin = sys.platform == 'darwin'
    kind = 'thread-faults' if args.thread_failures else 'faults'
    hooks = ['pthread_create'] if args.thread_failures else ['malloc', 'calloc', 'realloc', 'free']
    work = ROOT / (f'build/aom-{kind}-{profile}' + ('-asan' if args.asan else ''))
    work.mkdir(parents=True, exist_ok=True)
    reference = (ROOT / f'build/aom-native-{profile}').resolve()
    archive = ROOT / f'target/{profile}/libaom.a'
    compiler, runtime = os.environ.get('CC', 'cc'), []
    prefix = Path(os.environ.get('LIBAVIF_RS_LLVM_PREFIX', ROOT / 'build/toolchain/llvm@16/16.0.6_1'))
    objcopy = str(prefix / 'bin/llvm-objcopy') if darwin else shutil.which('objcopy')
    if not objcopy or not Path(objcopy).is_file():
        raise SystemExit('Fault observation requires llvm-objcopy or objcopy')
    environment = os.environ.copy()
    if darwin:
        environment['DYLD_LIBRARY_PATH'] = str(prefix / 'lib') + ':/opt/homebrew/lib'
    leading = '_' if darwin else ''
    changes = [argument for name in hooks for argument in
               ['--redefine-sym', f'{leading}{name}={leading}port_aom_{name}']]
    originals = [Path(entry['directory']) / entry['output'] for entry in library_entries(reference)]
    if args.asan:
        c_archive, archive, compiler, _, runtime = instrumented_libraries()
        directory = work / 'c-original-objects'
        directory.mkdir(exist_ok=True)
        originals = []
        for member in subprocess.check_output(['ar', '-t', c_archive], text=True).splitlines():
            if member.endswith('.o'):
                target = directory / member
                target.write_bytes(subprocess.check_output(['ar', '-p', c_archive, member]))
                originals.append(target)
    members = [name for name in subprocess.check_output(['ar', '-t', archive], text=True).splitlines()
               if name.startswith('aom.') and name.endswith('.rcgu.o')]
    if len(originals) != LIBRARY_UNITS or not members:
        raise SystemExit('Incomplete C or Rust codec object closure')
    extracted = work / 'rust-original-objects'
    extracted.mkdir(exist_ok=True)
    rust = []
    for member in members:
        target = extracted / member
        target.write_bytes(subprocess.check_output(['ar', '-p', archive, member]))
        rust.append(target)
    native = ['-lm', '-lpthread', *(['-lresolv', '-liconv'] if darwin else ['-ldl', '-lrt', '-lutil'])]
    source = ROOT / ('tests/ports/aom_thread_faults.c' if args.thread_failures else 'tests/ports/aom_faults.c')
    caller = work / 'caller.o'
    subprocess.run([compiler, '-std=c11', '-g', '-I', ROOT / 'upstream/aom',
                    *(['-fsanitize=address', '-fno-omit-frame-pointer'] if args.asan else []),
                    '-c', source, '-o', caller], env=environment, check=True)
    for label, inputs in [('c', originals), ('rust', rust)]:
        directory = work / f'{label}-objects'
        directory.mkdir(exist_ok=True)
        objects = []
        for index, original in enumerate(inputs):
            target = directory / f'{index:03d}.o'
            subprocess.run([objcopy, *changes, original, target], env=environment, check=True)
            objects.append(target)
        undefined = subprocess.check_output(['nm', '-u', *objects], text=True)
        if any(line.split()[-1] in {leading + name for name in hooks}
               for line in undefined.splitlines() if line.split()):
            raise SystemExit('An allocator reference remains unobserved in codec objects')
        linkmap = work / f'{label}.map'
        mapping = f'-Wl,-map,{linkmap}' if darwin else f'-Wl,-Map,{linkmap}'
        subprocess.run([compiler, caller, *objects, *([archive] if label == 'rust' else []), *native,
                        *runtime, mapping, '-o', work / label], env=environment, check=True)
        if label == 'rust' and re.search(re.escape(archive.name.encode()) + rb'\(aom\.', linkmap.read_bytes()):
            raise SystemExit('An unobserved Rust codec member entered the fault oracle')
    fixture = ROOT / f'build/aom-cli-{profile}' / ('inter-threaded-c.ivf' if args.thread_failures else 'intra-8-420-c.ivf')
    if not fixture.is_file():
        raise SystemExit('Run check_aom_cli.py for the independent decoder input first')

    def run(label, mode, index):
        result = subprocess.run([work / label, mode, str(index), fixture], env=environment, stdout=subprocess.PIPE,
                                stderr=subprocess.PIPE, timeout=60)
        if result.returncode:
            (work / 'failure.log').write_bytes(result.stdout + result.stderr)
            raise SystemExit(f'Fault caller failed: {label}/{mode}/{index}: {result.returncode}')
        value = json.loads(result.stdout)
        if not args.thread_failures and value['unknown_frees']:
            raise SystemExit(f'Unobserved allocation ownership: {label}/{mode}/{index}: {value}')
        return value

    records = []
    for mode in ['encoder', 'decoder']:
        baseline = [run(label, mode, 0) for label in ['c', 'rust']]
        if (baseline[0] != baseline[1] or baseline[0]['init'] or baseline[0]['operation']
                or (not args.thread_failures and baseline[0]['live']) or not baseline[0]['calls']):
            raise SystemExit(f'Baseline differs or leaks: {mode}: {baseline}')
        records.append(dict(mode=mode, failure_point=0, original=baseline[0], rust=baseline[1]))
        for index in range(1, baseline[0]['calls'] + 1):
            values = [run(label, mode, index) for label in ['c', 'rust']]
            record = dict(mode=mode, failure_point=index, original=values[0], rust=values[1])
            records.append(record)
            if values[0] != values[1] or values[0]['failures'] != 1:
                (work / 'mismatch.json').write_text(json.dumps(record, indent=2) + '\n')
                raise SystemExit(f'Allocation failure behavior differs: {mode}/{index}: {values}')
            if not args.thread_failures and values[0]['live']:
                raise SystemExit(f'The original and Rust fault scenario retain codec allocations: {mode}/{index}')
        print(f'{mode}: baseline and {baseline[0]["calls"]} {"thread creation" if args.thread_failures else "allocation"} failures match', flush=True)
    report = dict(profile=profile, target='macOS ARM64' if darwin else 'GNU Linux x86-64',
                  sanitizer='address' if args.asan else None, caller_instrumented=args.asan,
                  hooks=hooks, single_threaded=not args.thread_failures,
                  codec_allocation_ownership_observed=not args.thread_failures, paired_scenarios=len(records),
                  original_translation_units=len(originals), rewritten_rust_codegen_objects=len(rust),
                  source_sha256=hashlib.sha256(source.read_bytes()).hexdigest(),
                  input_sha256=hashlib.sha256(fixture.read_bytes()).hexdigest(),
                  rust_runtime_and_unwind_payload_unhooked=True,
                  records=records)
    (work / 'summary.json').write_text(json.dumps(report, indent=2) + '\n')


if __name__ == '__main__':
    main()
