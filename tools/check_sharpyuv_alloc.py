#!/usr/bin/env python3
"""Inject each allocation failure into C/Rust copies using test-only symbols."""
import argparse
import json
import os
from pathlib import Path
import shlex
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--asan', action='store_true', help='Instrument C/Rust and the driver with the matching macOS ASan runtime')
args = parser.parse_args()
base = Path(os.environ.get('SHARPYUV_BUILD_ROOT', ROOT / 'build'))
work = base / ('sharpyuv-allocation-asan' if args.asan else 'sharpyuv-allocation')
work.mkdir(parents=True, exist_ok=True)
compiler = shlex.split(os.environ.get('CC', 'cc'))
source = ROOT / 'upstream/libwebp'
reference = base / 'sharpyuv-reference'
common = ['-I' + str(source), '-I' + str(reference), '-DHAVE_CONFIG_H', '-O2', '-DNDEBUG', '-pthread', '-ffp-contract=off']
environment = os.environ.copy()
runtime_link = []
if args.asan:
    if sys.platform != 'darwin':
        raise SystemExit('This sanitizer driver currently targets macOS ARM64')
    prefix = Path(os.environ.get('LIBAVIF_RS_LLVM_PREFIX', ROOT / 'build/toolchain/llvm@16/16.0.6_1'))
    compiler = [str(prefix / 'bin/clang')]
    # Set inside this subprocess: the system Python removes incoming DYLD vars.
    environment['DYLD_LIBRARY_PATH'] = str(prefix / 'lib') + ':/opt/homebrew/lib'
    common += ['-fsanitize=address', '-fno-omit-frame-pointer']
    sysroot = Path(subprocess.check_output(['rustc', '--print', 'sysroot'], text=True).strip())
    runtime = sysroot / 'lib/rustlib/aarch64-apple-darwin/lib'
    if not (runtime / 'librustc-nightly_rt.asan.dylib').is_file():
        raise SystemExit('Missing matching Rust ASan runtime')
    runtime_link = ['-fno-sanitize=address', '-L' + str(runtime), '-lrustc-nightly_rt.asan', '-Wl,-rpath,' + str(runtime)]
    environment['CARGO_ENCODED_RUSTFLAGS'] = '\x1f'.join([
        '-Zsanitizer=address', '-Zexternal-clangrt',
        '-Cforce-frame-pointers=yes', '-Lnative=' + str(runtime), '-ldylib=rustc-nightly_rt.asan',
        '-Clink-arg=-Wl,-rpath,' + str(runtime)])
    environment.pop('RUSTFLAGS', None)
objects = []
for unit in sorted((source / 'sharpyuv').glob('*.c')):
    target = work / (unit.stem + '.o')
    subprocess.run(compiler + common + ['-Dmalloc=sharp_test_malloc', '-Dfree=sharp_test_free',
        '-c', str(unit), '-o', str(target)], check=True, env=environment)
    objects.append(str(target))
command = ['cargo', 'rustc', '-p', 'libavif-sharpyuv', '--locked', '--release', '--lib',
           '--crate-type', 'staticlib', '--target-dir', str(work / 'target'), '--', '--cfg', 'sharpyuv_test_allocator']
rust_environment = environment.copy()
# LLVM 16 is needed only by the C compiler. Rust must load its own LLVM 23.
rust_environment.pop('DYLD_LIBRARY_PATH', None)
result = subprocess.run(command, cwd=ROOT, env=rust_environment, text=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
if result.returncode:
    raise SystemExit(result.stderr)
outputs = []
driver = work / 'allocation-driver.o'
subprocess.run(compiler + common + ['-c', str(ROOT / 'tests/sharpyuv/allocation.c'),
                                  '-o', str(driver)], check=True, env=environment)
for name, libraries in [('c', objects),
                        ('rust', [str(work / 'target/release/libsharpyuv.a')])]:
    executable = work / name
    native = ['-lm', '-lpthread']
    native += ['-lresolv', '-liconv'] if sys.platform == 'darwin' else ['-ldl']
    subprocess.run(compiler + common + [str(driver), *libraries, *native,
                                      *runtime_link, '-o', str(executable)], check=True, env=environment)
    outputs.append(json.loads(subprocess.check_output([executable], text=True, env=environment)))
if outputs[0] != outputs[1]:
    raise SystemExit(f'Allocation behavior differs: {outputs}')
(work / 'results.json').write_text(json.dumps(outputs[0], indent=2) + '\n')
print(f'Identical allocation failure/cleanup: {len(outputs[0]) - 1} failure points and successful conversion')
if args.asan:
    print('C/Rust allocation paths and the common caller were instrumented with AddressSanitizer')
