#!/usr/bin/env python3
"""Build pinned original decoder/conversion references for native root validation."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import platform
import re
import shlex
import shutil
import subprocess

ROOT = Path(__file__).resolve().parents[1]


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def run(command):
    subprocess.run([str(value) for value in command], cwd=ROOT, check=True)


def scalar_macros(entry):
    """Inspect the actual SharpYUV DSP configuration with its compilation flags."""
    arguments = shlex.split(entry['command'])
    flags = []
    iterator = iter(arguments)
    for argument in iterator:
        if argument == '-o':
            next(iterator)
        elif argument not in ['-c', entry['file']]:
            flags.append(argument)
    result = subprocess.run(flags + ['-dM', '-E', '-x', 'c', '-'],
                            input='#include "sharpyuv/sharpyuv_dsp.h"\n',
                            cwd=entry['directory'], text=True, capture_output=True, check=True)
    macros = dict(re.findall(r'^#define (\w+)(?: (.*))?$', result.stdout, re.M))
    if 'HAVE_CONFIG_H' not in macros or any(name.startswith(('WEBP_USE_', 'WEBP_HAVE_'))
            and name.removeprefix('WEBP_USE_').removeprefix('WEBP_HAVE_') in
            ['SSE2', 'SSE41', 'AVX2', 'NEON', 'MIPS32', 'MIPS_DSP_R2', 'MSA'] for name in macros):
        raise SystemExit('The original SharpYUV root reference must retain scalar DSP selection')
    return result.stdout


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--debug', action='store_true')
    parser.add_argument('--jobs', type=int, default=2)
    args = parser.parse_args()
    if args.jobs < 1:
        parser.error('--jobs must be positive')
    target = ('aarch64-apple-darwin' if platform.system() == 'Darwin' and platform.machine() == 'arm64'
              else 'x86_64-unknown-linux-gnu' if platform.system() == 'Linux' and platform.machine() == 'x86_64'
              else None)
    if target is None:
        raise SystemExit('Root references currently target native macOS ARM64 or GNU Linux x86-64')
    run(['python3', 'tools/verify_upstream.py'])
    profile = 'debug' if args.debug else 'release'
    work = ROOT / f'build/root-references-{target}-{profile}'
    work.mkdir(parents=True, exist_ok=True)
    meson = shutil.which('meson')
    if not meson or subprocess.check_output([meson, '--version'], text=True).strip() != '1.7.2':
        raise SystemExit('Root references require the recorded Meson 1.7.2')
    dav1d = work / 'dav1d'
    setup = [meson, 'setup', dav1d, ROOT / 'upstream/dav1d',
             '--buildtype=' + profile, '-Ddefault_library=static', '-Db_staticpic=true',
             '-Denable_asm=false', '-Dbitdepths=8,16', '-Dtrim_dsp=false',
             '-Denable_tests=false', '-Denable_tools=false', '-Dxxhash_muxer=disabled',
             '-Dc_args=-ffp-contract=off -fvisibility=default', '--wrap-mode=nofallback']
    if (dav1d / 'build.ninja').exists():
        setup.append('--reconfigure')
    run(setup)
    run(['ninja', '-C', dav1d, '-j', args.jobs])
    sharp = work / 'sharpyuv'
    command = ['cmake', '-S', ROOT / 'upstream/libwebp', '-B', sharp, '-G', 'Ninja',
               '-DCMAKE_BUILD_TYPE=' + profile.title(), '-DBUILD_SHARED_LIBS=OFF',
               '-DCMAKE_POSITION_INDEPENDENT_CODE=ON', '-DCMAKE_EXPORT_COMPILE_COMMANDS=ON',
               '-DCMAKE_C_FLAGS=-ffp-contract=off', '-DWEBP_ENABLE_SIMD=OFF', '-DWEBP_USE_THREAD=ON',
               f'-DCMAKE_PROJECT_WebP_INCLUDE={ROOT}/tools/root-reference-webp.cmake']
    for name in ['ANIM_UTILS', 'CWEBP', 'DWEBP', 'GIF2WEBP', 'IMG2WEBP', 'VWEBP',
                 'WEBPINFO', 'LIBWEBPMUX', 'WEBPMUX', 'EXTRAS', 'FUZZTEST']:
        command.append(f'-DWEBP_BUILD_{name}=OFF')
    run(command)
    sharp_commands = json.loads((sharp / 'compile_commands.json').read_text())
    sharp_units = [entry for entry in sharp_commands if 'sharpyuv.dir' in entry['command']]
    if len(sharp_units) != 7:
        raise SystemExit('The root reference must retain all seven selected SharpYUV units')
    macros = scalar_macros(next(entry for entry in sharp_units if entry['file'].endswith('/sharpyuv_dsp.c')))
    (sharp / 'scalar-macros.txt').write_text(macros)
    run(['cmake', '--build', sharp, '--target', 'sharpyuv', '--parallel', args.jobs])
    libraries = {'dav1d': dav1d / 'src/libdav1d.a', 'sharpyuv': sharp / 'libsharpyuv.a'}
    for name, path in libraries.items():
        if not path.is_file():
            raise SystemExit(f'Missing complete original {name} reference: {path}')
    commands = json.loads((dav1d / 'compile_commands.json').read_text())
    decoder_units = [entry for entry in commands if entry.get('output', '').startswith('src/libdav1d')]
    if len(decoder_units) != 51:
        raise SystemExit('The root reference must retain all 51 portable dav1d source instantiations')
    record = dict(target=target, profile=profile, original_dav1d_units=51,
                  original_sharpyuv_units=7, source_and_features_unchanged=True,
                  standalone_test_execution_claimed=False,
                  sharpyuv_scalar_macros_sha256=digest(sharp / 'scalar-macros.txt'),
                  references={name: dict(path=str(path), sha256=digest(path)) for name, path in libraries.items()},
                  source_manifests_sha256={name: digest(ROOT / name) for name in
                      ['provenance/dav1d-source.json', 'provenance/libwebp-source.json']},
                  compiler=subprocess.check_output([os.environ.get('CC', 'cc'), '--version'], text=True).splitlines()[0])
    (work / 'reference-profile.json').write_text(json.dumps(record, indent=2) + '\n')
    print(f'Original root decoder/conversion references: {work}')


if __name__ == '__main__':
    main()
