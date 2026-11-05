"""Build the complete original and translated archives with one ASan runtime."""
from concurrent.futures import ThreadPoolExecutor
import hashlib
import json
import os
import shutil
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


def native_runtime():
    """Return the selected nightly's ASan runtime and its native consumer link flags."""
    import platform
    if sys.platform == 'darwin' and platform.machine() == 'arm64':
        triple, name = 'aarch64-apple-darwin', 'librustc-nightly_rt.asan.dylib'
    elif sys.platform == 'linux' and platform.machine() == 'x86_64':
        triple, name = 'x86_64-unknown-linux-gnu', 'librustc-nightly_rt.asan.a'
    else:
        raise SystemExit('Root ASan parity requires native macOS ARM64 or GNU Linux x86-64')
    sysroot = Path(subprocess.check_output(['rustc', '--print', 'sysroot'], text=True).strip())
    directory = sysroot / f'lib/rustlib/{triple}/lib'
    runtime = directory / name
    if not runtime.is_file():
        raise SystemExit(f'The selected nightly toolchain has no matching ASan runtime: {runtime}')
    if triple == 'aarch64-apple-darwin':
        link = ['-fno-sanitize=address', '-L' + str(directory), '-lrustc-nightly_rt.asan',
                '-Wl,-rpath,' + str(directory)]
    else:
        # ELF uses the static runtime. Link it once into each original executable,
        # as rustc does for instrumented Rust executables; never mix in Clang's.
        link = ['-fno-sanitize=address', '-Wl,--whole-archive', str(runtime), '-Wl,--no-whole-archive',
                '-lpthread', '-lrt', '-lm', '-ldl']
    return triple, runtime, link


def llvm_prefix():
    default = ROOT / 'build/toolchain/llvm@16/16.0.6_1' if sys.platform == 'darwin' else Path('/usr/lib/llvm-16')
    prefix = Path(os.environ.get('LIBAVIF_RS_LLVM_PREFIX', default))
    compiler = prefix / 'bin/clang'
    if not compiler.is_file():
        raise SystemExit('ASan parity requires LLVM 16; set LIBAVIF_RS_LLVM_PREFIX to its installation prefix')
    version = subprocess.check_output([compiler, '--version'], text=True,
                                      env=dict(os.environ, DYLD_LIBRARY_PATH=str(prefix / 'lib') + ':/opt/homebrew/lib')).splitlines()[0]
    if ' version 16.' not in version:
        raise SystemExit(f'ASan parity requires LLVM 16 native consumers: {version}')
    return prefix


def instrumented_reference():
    """Build only the complete instrumented original C AOM closure for root consumers.

    The archive contains all 168 units of the matching Release reference, with
    each original compilation command plus ASan instrumentation. Native VMAF
    remains uninstrumented. The Rust implementation is built by its consumer.
    """
    triple, runtime, link = native_runtime()
    prefix = llvm_prefix()
    compiler = str(prefix / 'bin/clang')
    environment = os.environ.copy()
    if sys.platform == 'darwin':
        environment['DYLD_LIBRARY_PATH'] = str(prefix / 'lib') + ':/opt/homebrew/lib'
    subprocess.run(['python3', ROOT / 'tools/verify_upstream.py'], check=True)
    native = ROOT / 'build/aom-native-release'
    profile = json.loads((native / 'reference-profile.json').read_text())
    if profile['profile'] != 'release' or profile['library_translation_units'] != 168:
        raise SystemExit('The instrumented AOM closure requires the complete original Release reference')
    work = ROOT / f'build/aom-root-asan-reference-{triple}'
    work.mkdir(parents=True, exist_ok=True)
    archive = work / 'libaom.a'
    record_path = work / 'build.json'
    files = ['provenance/aom-source.json', 'build/aom-native-release/library-compile-commands.json',
             'build/aom-native-release/reference-profile.json', 'tools/aom_sanitizer.py']
    key = hashlib.sha256(json.dumps({name: digest(ROOT / name) for name in files}, sort_keys=True).encode()
                         + subprocess.check_output([compiler, '--version'], env=environment)
                         + runtime.read_bytes()).hexdigest()
    cached = json.loads(record_path.read_text()) if record_path.exists() else {}
    if cached.get('key') != key or not archive.exists() or cached.get('original_sha256') != digest(archive):
        entries = library_entries(native)
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
        archive.unlink(missing_ok=True)
        subprocess.run(['ar', 'rcs', archive, *objects], check=True)
        record = dict(key=key, target=triple, original_translation_units=len(entries),
                      compiler=subprocess.check_output([compiler, '--version'], env=environment, text=True).splitlines()[0],
                      original_sha256=digest(archive), runtime=str(runtime), runtime_sha256=digest(runtime),
                      original_release_preprocessing=True, sanitizer='address', native_vmaf_uninstrumented=True)
        record_path.write_text(json.dumps(record, indent=2) + '\n')
    shutil.copyfile(native / 'reference-profile.json', work / 'reference-profile.json')
    return work, prefix, link


if __name__ == '__main__':
    import argparse
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--root-reference', action='store_true',
                        help='Build only the complete instrumented original C closure for root consumers')
    if not parser.parse_args().root_reference:
        parser.error('Use --root-reference; standalone checks import instrumented_libraries()')
    directory, _, _ = instrumented_reference()
    print(f'Complete instrumented original AOM reference: {directory / "libaom.a"}')
