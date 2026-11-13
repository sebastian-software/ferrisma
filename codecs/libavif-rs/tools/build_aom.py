#!/usr/bin/env python3
"""Build the complete scalar libaom reference and inventory its original tests.

The reference is upstream's `AOM_TARGET_CPU=generic` build with encoder and
decoder, 8, 10, and 12-bit support, threads, denoising, quantization
matrices, the complete non-realtime-only encoder, and the original default
experiments, compiled with `-ffp-contract=off` and `CONFIG_TUNE_VMAF=0`
(ADR 0057). Debug and Release keep their actual original assertion and
preprocessing differences. The library archive has 166 C translation units,
including three generated configuration or no-op units and four bundled
fastfeat and vector units. Bundled libyuv, WebM I/O, GoogleTest, the apps and
examples, and the optional C++ rate-control wrapper are outside it; they stay
original consumers and test infrastructure.

The original test inventories are 18,729 main-suite registrations (1,501
upstream-disabled), 38 rate-control registrations, and 38 intra-prediction
speed registrations, all of the latter disabled upstream. Registrations
include disabled tests, so they are not executed-pass counts. Upstream selects
568 test fixtures; their checksums are in the immutable `test/test-data.sha1`
and `tools/fetch_aom_test_data.py` also records SHA-256 and sizes. Never
regenerate goldens. Linking a test executable alone does not prove that every
original assertion exercises Rust.

`--simd neon` and `--simd avx2` build the same sources as native SIMD
references for the SIMD-against-SIMD gate of `tools/check_aom_simd_original.py`
(owner decisions D4 and D6): `AOM_TARGET_CPU=arm64` restricted to Neon (no
DotProd, I8MM, SVE or SVE2), or `AOM_TARGET_CPU=x86_64` up to AVX2 (no
AVX-512), each with upstream's run-time CPU detection and RTCD. On macOS ARM64
the x86-64 reference is built with `-DCMAKE_OSX_ARCHITECTURES=x86_64` and runs
under Rosetta 2 (`ROSETTA_ADVERTISE_AVX=1` exposes AVX2). Both keep the
declared noncontracting floating-point profile (`-ffp-contract=off`, ADR 0024);
`--fp-contract on` builds the compiler default that upstream's own release
build uses, only to explain differences. Every build records its configuration
identity (options, flags, `HAVE_*` settings, configuration and RTCD header
digests, archive digest) in `reference-profile.json`. Output directories:
`build/aom-native-<profile>` (generic, unchanged),
`build/aom-native-<simd>-<profile>`, with a `-contract` suffix for
`--fp-contract on`, and `build/aom-native-x86_64-<profile>` for the generic
reference built for Rosetta 2 with `--x86-64`. On macOS ARM64:

    python3 tools/build_aom.py --library-only                   # generic, arm64
    python3 tools/build_aom.py --library-only --simd neon       # Neon only
    python3 tools/build_aom.py --library-only --x86-64          # generic, x86-64
    python3 tools/build_aom.py --library-only --simd avx2       # up to AVX2, x86-64
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import platform
import shlex
import subprocess
import sys

from build_dav1d import copy_inputs

ROOT = Path(__file__).resolve().parents[1]
# Translation units of the still-image profile (ADR 0057): the complete generic
# archive without the two VMAF units, av1/encoder/tune_vmaf.c and aom_dsp/vmaf.c.
LIBRARY_UNITS = 166


def test_names(listing):
    names, suite = [], None
    for line in listing.splitlines():
        name = line.split('#', 1)[0].strip()
        if not name:
            continue
        if not line.startswith('  '):
            if not name.endswith('.'):
                raise SystemExit(f'Unexpected GoogleTest inventory line: {name}')
            suite = name
        else:
            if suite is None:
                raise SystemExit('Test registration without suite')
            names.append(suite + name)
    if len(set(names)) != len(names):
        raise SystemExit('Duplicate original GoogleTest names')
    return names


# Native SIMD references (module docstring). The levels are D6's v1.0 scope.
SIMD_REFERENCES = {
    'neon': dict(arch='arm64', options=['-DAOM_TARGET_CPU=arm64', '-DENABLE_NEON_DOTPROD=0',
                                        '-DENABLE_NEON_I8MM=0', '-DENABLE_SVE=0', '-DENABLE_SVE2=0'],
                 enabled=['HAVE_NEON'],
                 disabled=['HAVE_NEON_DOTPROD', 'HAVE_NEON_I8MM', 'HAVE_SVE', 'HAVE_SVE2']),
    'avx2': dict(arch='x86_64', options=['-DAOM_TARGET_CPU=x86_64', '-DENABLE_AVX512=0'],
                 enabled=['HAVE_MMX', 'HAVE_SSE', 'HAVE_SSE2', 'HAVE_SSE3', 'HAVE_SSSE3',
                          'HAVE_SSE4_1', 'HAVE_SSE4_2', 'HAVE_AVX', 'HAVE_AVX2'],
                 disabled=['HAVE_AVX512']),
}
RTCD_HEADERS = ['config/aom_dsp_rtcd.h', 'config/aom_scale_rtcd.h', 'config/av1_rtcd.h']


def reference_build(profile, simd='generic', contract=False, cross_x86_64=False):
    """The build directory of a reference (see the module docstring)."""
    name = ('aom-native' + ('' if simd == 'generic' else f'-{simd}')
            + ('-x86_64' if cross_x86_64 and simd == 'generic' else '') + f'-{profile}')
    return ROOT / 'build' / (name + ('-contract' if contract else ''))


def library_entries(build, sources=('.c',)):
    line = next(line for line in (build / 'build.ninja').read_text().splitlines()
                if line.startswith('build libaom.a:'))
    objects = shlex.split(line.split(' |')[0])[3:]
    database = json.loads((build / 'compile_commands.json').read_text())
    entries = [entry for entry in database
               if str((Path(entry['directory']) / entry['output']).relative_to(build)) in objects]
    if len(entries) != len(objects) or not all(e['file'].endswith(sources) for e in entries):
        raise SystemExit('Archive closure does not match the C compilation database')
    return entries


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--debug', action='store_true')
    parser.add_argument('--jobs', type=int, default=8)
    parser.add_argument('--library-only', action='store_true',
                        help=f'Build all {LIBRARY_UNITS} original library units for root validation without building standalone test binaries')
    parser.add_argument('--simd', choices=['generic', *SIMD_REFERENCES], default='generic',
                        help='generic (default): the portable reference; neon or avx2: a native '
                             'SIMD reference at that level (requires --library-only)')
    parser.add_argument('--fp-contract', choices=['off', 'on'], default='off',
                        help='off (default): the declared noncontracting reference; on: the '
                             "compiler default of upstream's release build, for diagnosis only")
    parser.add_argument('--x86-64', action='store_true',
                        help='On macOS ARM64, build the generic reference for x86-64 (Rosetta 2) '
                             'into build/aom-native-x86_64-<profile>; requires --library-only')
    args = parser.parse_args()
    simd = SIMD_REFERENCES.get(args.simd)
    if args.x86_64 and (simd or not args.library_only):
        parser.error('--x86-64 applies to the generic library-only reference; --simd avx2 implies it')
    if simd and not args.library_only:
        parser.error('SIMD references build the library only; pass --library-only')
    if args.fp_contract == 'on' and not simd:
        parser.error('--fp-contract on is a diagnostic SIMD reference; the generic reference stays noncontracting')
    subprocess.run(['python3', ROOT / 'tools/verify_upstream.py'], check=True)
    source = ROOT / 'build/aom-native-source'
    copy_inputs(json.loads((ROOT / 'provenance/aom-source.json').read_text()), source)
    profile = 'debug' if args.debug else 'release'
    host_arch = {'arm64': 'arm64', 'aarch64': 'arm64', 'x86_64': 'x86_64', 'AMD64': 'x86_64'}[platform.machine()]
    target_arch = simd['arch'] if simd else ('x86_64' if args.x86_64 else host_arch)
    cross = target_arch != host_arch
    if cross and (sys.platform != 'darwin' or target_arch != 'x86_64'):
        raise SystemExit(f'References build natively, or for x86-64 on macOS (Rosetta 2); not {target_arch} here')
    build = reference_build(profile, args.simd, args.fp_contract == 'on', cross)
    flags = (('-ffp-contract=off ' if args.fp_contract == 'off' else '')
             + f'-fmacro-prefix-map={source}=upstream/aom')
    options = [*(simd['options'] if simd else ['-DAOM_TARGET_CPU=generic']), '-DCONFIG_TUNE_VMAF=0',
               '-DCMAKE_POSITION_INDEPENDENT_CODE=ON', '-DENABLE_DOCS=OFF']
    if cross:
        options.append('-DCMAKE_OSX_ARCHITECTURES=x86_64')
    command = ['cmake', '-S', source, '-B', build, '-G', 'Ninja',
               f'-DCMAKE_BUILD_TYPE={profile.title()}',
               '-DCMAKE_EXPORT_COMPILE_COMMANDS=ON',
               f'-DCMAKE_C_FLAGS={flags}', f'-DCMAKE_CXX_FLAGS={flags}', *options]
    subprocess.run([str(item) for item in command], check=True)
    subprocess.run(['ninja', '-C', build, '-j', str(args.jobs), *(['libaom.a'] if args.library_only else [])], check=True)
    entries = library_entries(build, ('.c', '.asm') if simd else ('.c',))
    if not simd and len(entries) != LIBRARY_UNITS:
        raise SystemExit(f'Unexpected complete portable source closure: {len(entries)}')
    config = (build / 'config/aom_config.h').read_text()
    for setting in ['CONFIG_AV1_ENCODER 1', 'CONFIG_AV1_DECODER 1',
                    'CONFIG_AV1_HIGHBITDEPTH 1', 'CONFIG_MULTITHREAD 1',
                    'CONFIG_DENOISE 1', 'CONFIG_QUANT_MATRIX 1',
                    'CONFIG_REALTIME_ONLY 0', 'CONFIG_TUNE_VMAF 0']:
        if f'#define {setting}\n' not in config:
            raise SystemExit(f'Missing required configuration: {setting}')
    have = {}
    for line in config.splitlines():
        if line.startswith('#define HAVE_') and any(word in line for word in
            ['NEON', 'SVE', 'SSE', 'AVX', 'MMX', 'VSX', 'RVV', 'ARM_CRC']):
            name, value = line.split()[1:3]
            have[name] = int(value)
            if not simd and value != '0':
                raise SystemExit(f'Unexpected optimized implementation: {line}')
    if simd and (any(have.get(name) != 1 for name in simd['enabled'])
                 or any(have.get(name, 0) != 0 for name in simd['disabled'])):
        raise SystemExit(f'The {args.simd} reference has the wrong SIMD level: {have}')
    for setting in (['CONFIG_RUNTIME_CPU_DETECT 1', 'CONFIG_EXCLUDE_SIMD_MISMATCH 0',
                     'CONFIG_HIGHWAY 0'] if simd else []):
        if f'#define {setting}\n' not in config:
            raise SystemExit(f'Missing required configuration: {setting}')
    (build / 'library-compile-commands.json').write_text(json.dumps(entries, indent=2) + '\n')
    inventories = {}
    for name in ([] if args.library_only else ['test_libaom', 'test_aom_rc', 'test_intra_pred_speed']):
        listing = subprocess.check_output([build / name, '--gtest_list_tests'], text=True)
        (build / f'{name}-inventory.txt').write_text(listing)
        names = test_names(listing)
        normalized = json.dumps(names, separators=(',', ':'))
        inventories[name] = dict(registrations=len(names),
                                 names_sha256=hashlib.sha256(normalized.encode()).hexdigest(),
                                 upstream_disabled=sum(any(part.startswith('DISABLED_') for part in
                                     item.replace('.', '/').split('/')) for item in names))
        (build / f'{name}-inventory.json').write_text(json.dumps(names, indent=2) + '\n')
    data_commands = subprocess.check_output(['ninja', '-C', build, '-t', 'commands', 'testdata'], text=True)
    data = {}
    for command in data_commands.splitlines():
        values = dict(argument[2:].split('=', 1) for argument in shlex.split(command)
                      if argument.startswith('-DAOM_') and '=' in argument)
        name = values['AOM_TEST_FILE']
        data[name] = values['AOM_TEST_CHECKSUM']
    (build / 'test-data.json').write_text(json.dumps(data, indent=2, sort_keys=True) + '\n')
    archive = build / 'libaom.a'
    record = dict(reference=('libaom 3.15.2 complete portable C' if not simd else
                             f'libaom 3.15.2 native {args.simd} SIMD reference (run-time CPU detection)'),
                  profile=profile, simd=args.simd, target_arch=target_arch,
                  fp_contract=args.fp_contract, have=have,
                  rtcd_headers_sha256={name: hashlib.sha256((build / name).read_bytes()).hexdigest()
                                       for name in RTCD_HEADERS},
                  archive_sha256=hashlib.sha256(archive.read_bytes()).hexdigest(),
                  options=options, c_flags=flags.replace(str(source), '<source>'),
                  compiler=subprocess.check_output([os.environ.get('CC', 'cc'), '--version'],
                                                   text=True).splitlines()[0],
                  library_only_reference=args.library_only, original_tests_executed=False,
                  library_translation_units=len(entries), generated_units=3,
                  assembly_units=sum(entry['file'].endswith('.asm') for entry in entries),
                  original_test_inventories=inventories, original_test_inputs=len(data),
                  config_sha256=hashlib.sha256(config.encode()).hexdigest(),
                  transitive_runtime_dependencies={})
    (build / 'reference-profile.json').write_text(json.dumps(record, indent=2) + '\n')
    print(json.dumps(record, indent=2))


if __name__ == '__main__':
    main()
