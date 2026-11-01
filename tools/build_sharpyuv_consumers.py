#!/usr/bin/env python3
"""Build all 12 original WebP FuzzTest consumers with C or Rust SharpYUV."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import tarfile
import tempfile

ROOT = Path(__file__).resolve().parents[1]
REVISION = '078ea0871cc96d3a69bad406577f176a4fa14ae9'


def run(command, **kwargs):
    subprocess.run([str(arg) for arg in command], cwd=kwargs.pop('cwd', ROOT), check=True, **kwargs)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('implementation', choices=['c', 'rust'])
    parser.add_argument('--fetch-deps', action='store_true')
    parser.add_argument('--jobs', type=int, default=4)
    args = parser.parse_args()
    run(['python3', ROOT / 'tools/verify_upstream.py'])
    base = Path(os.environ.get('SHARPYUV_BUILD_ROOT', ROOT / 'build'))
    dependency = ROOT / 'build/deps/fuzztest'
    if not dependency.exists():
        if not args.fetch_deps:
            raise SystemExit('Missing pinned FuzzTest; use --fetch-deps')
        run(['git', 'clone', '--no-checkout', 'https://github.com/google/fuzztest.git', dependency])
        run(['git', '-C', dependency, 'checkout', '--detach', REVISION])
    actual = subprocess.check_output(['git', '-C', str(dependency), 'rev-parse', 'HEAD'], text=True).strip()
    if actual != REVISION:
        raise SystemExit(f'Unexpected FuzzTest revision: {actual}')
    # FetchContent source overrides skip PATCH_COMMAND. Apply the exact upstream
    # patch to a disposable export, preserving both vendored projects unchanged.
    exported = base / 'sharpyuv-fuzztest-source'
    patch = ROOT / 'upstream/libwebp/tests/fuzzer/patch.sh'
    patch_hash = hashlib.sha256(patch.read_bytes()).hexdigest()
    stamp = exported / '.libavif-rs-patch.json'
    expected = {'revision': REVISION, 'patch_sha256': patch_hash}
    if not stamp.exists():
        exported.mkdir(parents=True, exist_ok=True)
        with tempfile.TemporaryFile() as archive:
            run(['git', '-C', dependency, 'archive', REVISION], stdout=archive)
            archive.seek(0)
            with tarfile.open(fileobj=archive) as files:
                if sys.version_info >= (3, 12):
                    files.extractall(exported, filter='data')
                else:
                    # This archive is produced locally from the checked commit.
                    files.extractall(exported)
        environment = os.environ.copy()
        if sys.platform == 'darwin':
            # Translate only GNU sed's in-place argument spelling for BSD sed.
            wrapper = base / 'sharpyuv-test-tools'
            wrapper.mkdir(exist_ok=True)
            script = wrapper / 'sed'
            script.write_text('#!/usr/bin/env python3\nimport os, sys\na = sys.argv[1:]\n'
                              'i = a.index("-i") if "-i" in a else -1\n'
                              'if i >= 0: a.insert(i + 1, "")\n'
                              'os.execv("/usr/bin/sed", ["sed"] + a)\n')
            script.chmod(0o755)
            environment['PATH'] = str(wrapper) + os.pathsep + environment['PATH']
        run(['sh', patch], cwd=exported, env=environment)
        stamp.write_text(json.dumps(expected, indent=2) + '\n')
    if json.loads(stamp.read_text()) != expected:
        raise SystemExit('FuzzTest patch record differs; regenerate the disposable export')
    build = base / ('sharpyuv-webp-' + args.implementation)
    command = ['cmake', '-S', ROOT / 'upstream/libwebp', '-B', build, '-G', 'Ninja',
               '-DCMAKE_BUILD_TYPE=Release', '-DCMAKE_EXPORT_COMPILE_COMMANDS=ON',
               '-DCMAKE_POLICY_VERSION_MINIMUM=3.5', '-DBUILD_SHARED_LIBS=OFF',
               '-DWEBP_LINK_STATIC=OFF', '-DWEBP_ENABLE_SIMD=OFF', '-DWEBP_USE_THREAD=ON',
               '-DWEBP_BUILD_FUZZTEST=ON', '-DWEBP_BUILD_LIBWEBPMUX=ON', '-DWEBP_BUILD_WEBPINFO=ON',
               '-DFUZZTEST_BUILD_TESTING=OFF', '-DFUZZTEST_FUZZING_MODE=OFF',
               '-DANTLR_BUILD_CPP_TESTS=OFF',
               '-DCMAKE_PROJECT_WebP_INCLUDE=' + str(ROOT / 'tools/webp-consumers.cmake'),
               '-DFETCHCONTENT_SOURCE_DIR_FUZZTEST=' + str(exported)]
    for name in ['ANIM_UTILS', 'CWEBP', 'DWEBP', 'GIF2WEBP', 'IMG2WEBP', 'VWEBP', 'WEBPMUX', 'EXTRAS']:
        command.append('-DWEBP_BUILD_' + name + '=OFF')
    if not args.fetch_deps:
        command.append('-DFETCHCONTENT_FULLY_DISCONNECTED=ON')
    if args.implementation == 'rust':
        run(['cargo', 'build', '-p', 'libavif-sharpyuv', '--locked', '--release'])
        target = Path(os.environ.get('CARGO_TARGET_DIR', ROOT / 'target'))
        command += ['-DSHARPYUV_RS_ARCHIVE=' + str(target / 'release/libsharpyuv.a')]
        # Reuse the exact sources fetched by the original native reference,
        # while keeping objects, executables, and results in separate builds.
        for name in ['abseil-cpp', 're2', 'googletest', 'antlr_cpp']:
            path = base / 'sharpyuv-webp-c/_deps' / (name + '-src')
            if path.exists():
                command += ['-DFETCHCONTENT_SOURCE_DIR_' + name.upper() + '=' + str(path)]
    run(command)
    run(['cmake', '--build', build, '-j', args.jobs])
    entries = json.loads((build / 'compile_commands.json').read_text())
    units = [entry['file'] for entry in entries if 'sharpyuv.dir' in entry['command']]
    if args.implementation == 'rust' and (len(units) != 1 or not units[0].endswith('sharpyuv_rust_anchor.c')):
        raise SystemExit(f'Unexpected native SharpYUV implementations: {units}')
    record = dict(expected, implementation=args.implementation, sharpyuv_units=units,
                  fuzztest_mode='original unit-test mode (bounded randomized smoke tests)')
    (build / 'replacement.json').write_text(json.dumps(record, indent=2) + '\n')
    run(['python3', ROOT / 'tools/verify_upstream.py'])


if __name__ == '__main__':
    main()
