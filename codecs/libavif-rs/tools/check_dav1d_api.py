#!/usr/bin/env python3
"""Compare public decoder ownership, callbacks, failures, and pixels in isolation."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys

from check_abi import compiler_flags

ROOT = Path(__file__).resolve().parents[1]


def instrumented_libraries(work):
    if sys.platform != 'darwin':
        raise SystemExit('The sanitizer runtime mapping currently targets macOS ARM64')
    prefix = Path(os.environ.get('LIBAVIF_RS_LLVM_PREFIX', ROOT / 'build/toolchain/llvm@16/16.0.6_1'))
    compiler = str(prefix / 'bin/clang')
    environment = os.environ.copy()
    environment['DYLD_LIBRARY_PATH'] = str(prefix / 'lib') + ':/opt/homebrew/lib'
    database = ROOT / 'build/dav1d-reference-release/compile_commands.json'
    entries = [entry for entry in json.loads(database.read_text()) if entry['output'].startswith('src/libdav1d')]
    if len(entries) != 51:
        raise SystemExit('Missing original portable sanitizer source closure')
    objects = []
    for index, entry in enumerate(entries):
        original = compiler_flags(entry)[1:]
        flags, offset = [], 0
        while offset < len(original):
            value = original[offset]
            if value in ['-MF', '-MQ']:
                offset += 2
                continue
            if value != '-MD':
                flags.append(value)
            offset += 1
        output = work / f'reference-{index}.o'
        subprocess.run([compiler, *flags, '-fsanitize=address', '-fno-omit-frame-pointer',
                        '-c', entry['file'], '-o', output], cwd=entry['directory'], env=environment, check=True)
        objects.append(output)
    reference = work / 'libdav1d-reference.a'
    subprocess.run(['ar', 'rcs', reference, *objects], check=True)
    sysroot = Path(subprocess.check_output(['rustc', '--print', 'sysroot'], text=True).strip())
    runtime = sysroot / 'lib/rustlib/aarch64-apple-darwin/lib'
    if not (runtime / 'librustc-nightly_rt.asan.dylib').exists():
        raise SystemExit('Missing matching Rust ASan runtime')
    rust_environment = environment.copy()
    rust_environment.pop('DYLD_LIBRARY_PATH', None)
    rust_environment.pop('RUSTFLAGS', None)
    rust_environment['CARGO_ENCODED_RUSTFLAGS'] = '\x1f'.join([
        '-Zsanitizer=address', '-Zexternal-clangrt', '-Cforce-frame-pointers=yes',
        '-Lnative=' + str(runtime), '-ldylib=rustc-nightly_rt.asan',
        '-Clink-arg=-Wl,-rpath,' + str(runtime), '--cfg', 'libavif_rs_asan'])
    subprocess.run(['cargo', 'build', '-p', 'libavif-dav1d', '--release', '--locked',
                    '--target', 'aarch64-apple-darwin', '--target-dir', work / 'target'],
                   cwd=ROOT, env=rust_environment, check=True)
    candidate = work / 'target/aarch64-apple-darwin/release/libdav1d.a'
    link = ['-fno-sanitize=address', '-L' + str(runtime), '-lrustc-nightly_rt.asan',
            '-Wl,-rpath,' + str(runtime), '-liconv', '-lresolv', '-lpthread']
    return reference, candidate, compiler, environment, link


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--debug', action='store_true')
    parser.add_argument('--asan', action='store_true')
    args = parser.parse_args()
    if args.asan and args.debug:
        parser.error('The sanitizer configuration uses original Release preprocessing')
    profile = 'debug' if args.debug else 'release'
    filename = 'libdav1d.7.dylib' if sys.platform == 'darwin' else 'libdav1d.so.7.0.0'
    reference = ROOT / f'build/dav1d-reference-{profile}/src' / filename
    candidate = ROOT.parents[1] / 'target' / profile / 'libdav1d.a'
    work = ROOT / (f'build/dav1d-api-{profile}' + ('-asan' if args.asan else ''))
    work.mkdir(exist_ok=True)
    source = ROOT / 'tests/ports/dav1d_api.c'
    compiler, environment, runtime_link = 'cc', os.environ.copy(), []
    if args.asan:
        reference, candidate, compiler, environment, runtime_link = instrumented_libraries(work)
    driver = work / 'driver.o'
    compile_command = [compiler, '-std=c99', '-O2', '-g', '-ffp-contract=off', '-I', ROOT / 'upstream/dav1d/include']
    if args.asan:
        compile_command += ['-fsanitize=address', '-fno-omit-frame-pointer']
    subprocess.run([str(value) for value in compile_command + ['-c', source, '-o', driver]], env=environment, check=True)
    for name, library in [('c', reference), ('rust', candidate)]:
        command = [compiler, driver, library, *runtime_link, '-o', work / name]
        if name == 'c' and not args.asan:
            command.append(f'-Wl,-rpath,{reference.parent}')
        if sys.platform == 'linux':
            command += ['-lm', '-ldl', '-lpthread', '-lrt', '-lutil']
        subprocess.run([str(value) for value in command], env=environment, check=True)
    data = ROOT / 'build/deps/dav1d-test-data'
    fixtures = ['8-bit/data/00000000.ivf', '8-bit/film_grain/clip_0.ivf',
                '10-bit/data/00000943.ivf', '12-bit/data/00000791.ivf']
    cases = [] if args.debug else [('invalid', ['invalid'])]
    cases.append(('logging-and-error-properties', ['logging']))
    cases.append(('automatic-thread-selection',
                  [str(data / fixtures[0]), '0', '0', '1', '1', '0']))
    for fixture in fixtures:
        for threads, delay in [(1, 1), (2, 1), (2, 2), (4, 2)]:
            for grain in [0, 1]:
                for custom in [0, 1]:
                    cases.append((f'{fixture}-{threads}-{delay}-{grain}-{custom}',
                        [str(data / fixture), str(threads), str(delay), str(grain), str(custom), '0']))
            cases.append((f'{fixture}-{threads}-{delay}-allocation-failure',
                          [str(data / fixture), str(threads), str(delay), '1', '1', '1']))
    records = []
    for index, (label, arguments) in enumerate(cases):
        results = []
        for name in ['c', 'rust']:
            result = subprocess.run([work / name, *arguments], env=environment,
                                    stdout=subprocess.PIPE, stderr=subprocess.PIPE, timeout=120)
            (work / f'{index}-{name}.log').write_bytes(result.stderr)
            if result.returncode:
                raise SystemExit(f'API consumer failed: {label} ({name}); inspect {work / f"{index}-{name}.log"}')
            value = json.loads(result.stdout)
            (work / f'{index}-{name}.json').write_text(json.dumps(value, indent=2) + '\n')
            results.append(value)
        if results[0] != results[1]:
            raise SystemExit(f'Public API difference: {label}; inspect {work / str(index)}-*.json')
        records.append({'case': label, 'result': results[0]})
    report = {'profile': profile, 'cases': len(cases), 'differences': [], 'results': records,
              'sanitizer': 'address' if args.asan else None, 'driver_instrumented': args.asan,
              'driver_sha256': hashlib.sha256(source.read_bytes()).hexdigest(),
              'reference_sha256': hashlib.sha256(reference.read_bytes()).hexdigest(),
              'candidate_archive_sha256': hashlib.sha256(candidate.read_bytes()).hexdigest()}
    (work / 'summary.json').write_text(json.dumps(report, indent=2) + '\n')
    print(f'{len(cases)} exact public API/ownership comparisons; all callback allocations released')


if __name__ == '__main__':
    main()
