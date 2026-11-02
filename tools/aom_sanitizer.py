"""Build the complete original and translated archives with one ASan runtime."""
from concurrent.futures import ThreadPoolExecutor
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys

from build_aom import library_entries
from check_abi import compiler_flags

ROOT = Path(__file__).resolve().parents[1]


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def instrumented_libraries():
    if sys.platform != 'darwin':
        raise SystemExit('The recorded libaom sanitizer runtime mapping targets macOS ARM64')
    subprocess.run(['python3', ROOT / 'tools/verify_upstream.py'], check=True)
    subprocess.run(['python3', ROOT / 'tools/check_translation.py'], check=True)
    work = ROOT / 'build/aom-sanitizer'
    work.mkdir(exist_ok=True)
    prefix = Path(os.environ.get('LIBAVIF_RS_LLVM_PREFIX', ROOT / 'build/toolchain/llvm@16/16.0.6_1'))
    compiler = str(prefix / 'bin/clang')
    environment = os.environ.copy()
    environment['DYLD_LIBRARY_PATH'] = str(prefix / 'lib') + ':/opt/homebrew/lib'
    sysroot = Path(subprocess.check_output(['rustc', '--print', 'sysroot'], text=True).strip())
    runtime = sysroot / 'lib/rustlib/aarch64-apple-darwin/lib'
    dylib = runtime / 'librustc-nightly_rt.asan.dylib'
    if not dylib.exists():
        raise SystemExit('Missing matching Rust ASan runtime')
    reference = work / 'libaom-reference.a'
    candidate = work / 'target/aarch64-apple-darwin/release/libaom.a'
    record_path = work / 'build.json'
    files = ['provenance/aom-source.json', 'crates/aom/TRANSLATION-macos-release.json',
             'crates/aom/src/lib.rs', 'crates/aom/src/platform.rs', 'crates/aom/src/jump.rs',
             'crates/aom/src/modules-macos-release.rs', 'crates/aom/src/api-macos-release.rs',
             'crates/aom/build.rs', 'crates/aom/Cargo.toml', 'Cargo.toml', 'Cargo.lock',
             'build/aom-native-release/library-compile-commands.json', 'tools/aom_sanitizer.py']
    key = hashlib.sha256(json.dumps({name: digest(ROOT / name) for name in files}, sort_keys=True).encode()
                         + subprocess.check_output(['rustc', '-vV']) + dylib.read_bytes()
                         + subprocess.check_output([compiler, '--version'], env=environment)
                         + subprocess.check_output(['pkg-config', '--libs', '--modversion', 'libvmaf'])
                         + os.environ.get('PKG_CONFIG_PATH', '').encode()).hexdigest()
    cached = json.loads(record_path.read_text()) if record_path.exists() else {}
    if (cached.get('key') != key or not reference.exists() or not candidate.exists()
            or cached.get('original_sha256') != digest(reference)
            or cached.get('rust_sha256') != digest(candidate)):
        entries = library_entries(ROOT / 'build/aom-native-release')
        if len(entries) != 168:
            raise SystemExit('Missing complete original sanitizer source closure')

        def compile_unit(item):
            index, entry = item
            flags = compiler_flags(entry)[1:]
            output = work / f'reference-{index:03d}.o'
            subprocess.run([compiler, *flags, '-fsanitize=address', '-fno-omit-frame-pointer',
                            '-c', entry['file'], '-o', output], cwd=entry['directory'], env=environment, check=True)
            return output

        with ThreadPoolExecutor(max_workers=2) as executor:
            objects = list(executor.map(compile_unit, enumerate(entries)))
        reference.unlink(missing_ok=True)
        subprocess.run(['ar', 'rcs', reference, *objects], check=True)
        rust_environment = environment.copy()
        rust_environment.pop('DYLD_LIBRARY_PATH', None)
        rust_environment.pop('RUSTFLAGS', None)
        rust_environment['CARGO_ENCODED_RUSTFLAGS'] = '\x1f'.join([
            '-Zsanitizer=address', '-Zexternal-clangrt', '-Cforce-frame-pointers=yes',
            '-Lnative=' + str(runtime), '-ldylib=rustc-nightly_rt.asan',
            '-Clink-arg=-Wl,-rpath,' + str(runtime), '--cfg', 'libavif_rs_asan'])
        subprocess.run(['cargo', 'build', '-p', 'libavif-aom', '--release', '--locked',
                        '--target', 'aarch64-apple-darwin', '--target-dir', work / 'target'],
                       cwd=ROOT, env=rust_environment, check=True)
        record = dict(key=key, original_translation_units=len(entries),
                      compiler=subprocess.check_output([compiler, '--version'], env=environment, text=True).splitlines()[0],
                      original_sha256=digest(reference), rust_sha256=digest(candidate), runtime_sha256=digest(dylib),
                      original_release_preprocessing=True, sanitizer='address', native_vmaf_uninstrumented=True)
        record_path.write_text(json.dumps(record, indent=2) + '\n')
    link = ['-fno-sanitize=address', '-L' + str(runtime), '-lrustc-nightly_rt.asan',
            '-Wl,-rpath,' + str(runtime)]
    return reference, candidate, compiler, environment, link
