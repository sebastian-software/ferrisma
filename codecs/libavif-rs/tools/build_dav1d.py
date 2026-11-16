#!/usr/bin/env python3
"""Build the pinned complete portable dav1d reference without editing upstream.

The reference configuration has both bit-depth instantiations, logging,
threading, all scalar DSP functions, no assembly, and `-ffp-contract=off`. The
original CLI muxers, demuxers, test code, and system facilities stay native test
infrastructure. The optional xxHash output muxer is outside the decoder and is
disabled (`-Dxxhash_muxer=disabled`); the original built-in MD5 verifier stays
enabled. The separate checkasm framework and the decoder corpus are fetched
into `build/deps` (`tools/import_dav1d.py --fetch`) and hashed in
`provenance/dav1d-test-inputs.json`.

`--asm` builds the SIMD oracle of ADR 0062 instead (`build/dav1d-asm-<profile>`):
the same source and flags with the original's assembly (`-Denable_asm=true`,
NASM on x86-64), `-Dtrim_dsp=false` so that `dav1d_set_cpu_flags_mask` can
select every level down to C, a static library and the original CLI. It is a
test reference only; nothing of it is linked into the port (engine ADR-0044).
With `--cross x86_64-linux-gnu` (inside the container of `tools/qemu_x86_64`)
it is cross-built for GNU Linux x86-64 and its tools run under
`qemu-x86_64 -cpu max`.
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess

ROOT = Path(__file__).resolve().parents[1]


def ninja_jobs():
    """`-j N` from DAV1D_NINJA_JOBS (shared hosts); Ninja's default otherwise."""
    jobs = os.environ.get('DAV1D_NINJA_JOBS')
    return ['-j', jobs] if jobs else []


def copy_inputs(record, target):
    source = ROOT / record['source_root']
    for name, expected in record['files'].items():
        content = (source / name).read_bytes()
        if hashlib.sha256(content).hexdigest() != expected:
            raise SystemExit(f'Modified test input: {source / name}')
        output = target / name
        output.parent.mkdir(parents=True, exist_ok=True)
        if not output.exists() or output.read_bytes() != content:
            output.write_bytes(content)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--debug', action='store_true')
    parser.add_argument('--asm', action='store_true', help='SIMD oracle with the original assembly (ADR 0062)')
    parser.add_argument('--cross', choices=['x86_64-linux-gnu'], help='cross-build (QEMU container)')
    args = parser.parse_args()
    if args.asm:
        return build_asm(args)
    subprocess.run(['python3', ROOT / 'tools/verify_upstream.py'], check=True)
    meson = shutil.which('meson') or str(ROOT / 'build/dav1d-tools/bin/meson')
    source = ROOT / 'build/dav1d-native-source'
    copy_inputs(json.loads((ROOT / 'provenance/dav1d-source.json').read_text()), source)
    tests = json.loads((ROOT / 'provenance/dav1d-test-inputs.json').read_text())
    for name, target in [('checkasm', source / 'subprojects/checkasm'),
                         ('dav1d-test-data', source / 'tests/dav1d-test-data')]:
        copy_inputs(tests[name], target)
    build = ROOT / ('build/dav1d-reference-debug' if args.debug else 'build/dav1d-reference-release')
    flags = '-ffp-contract=off -fvisibility=default'
    setup = [meson, 'setup', build, source, '-Denable_asm=false', '-Dbitdepths=8,16',
             '-Dtrim_dsp=false', '-Denable_tests=true', '-Denable_tools=true',
             '-Denable_seek_stress=true', '-Dtestdata_tests=true', '-Dxxhash_muxer=disabled',
             '-Dcheckasm:docs=disabled', '-Dcheckasm:tests=disabled',
             '--wrap-mode=nofallback', f'-Dc_args={flags}',
             '--buildtype=debug' if args.debug else '--buildtype=release']
    if (build / 'build.ninja').exists():
        setup.append('--reconfigure')
    subprocess.run([str(x) for x in setup], check=True)
    subprocess.run(['ninja', '-C', build, *ninja_jobs()], check=True)
    database = json.loads((build / 'compile_commands.json').read_text())
    units = [entry for entry in database if entry['output'].startswith('src/libdav1d')]
    if len(units) != 51:
        raise SystemExit(f'Unexpected portable decoder source closure: {len(units)}')
    configuration = (build / 'config.h').read_text()
    for setting in ['HAVE_ASM 0', 'CONFIG_8BPC 1', 'CONFIG_16BPC 1', 'TRIM_DSP_FUNCTIONS 0', 'CONFIG_LOG 1']:
        if f'#define {setting}\n' not in configuration:
            raise SystemExit(f'Missing selected configuration: {setting}')
    profile = {'reference': 'dav1d 1.5.4 portable original C', 'acceptance_reference': True,
               'profile': 'debug' if args.debug else 'release', 'c_flags': flags,
               'compiler': subprocess.check_output([os.environ.get('CC', 'cc'), '--version'], text=True).splitlines()[0],
               'library_translation_units': len(units), 'bitdepth_instantiations': [8, 16],
               'assembly': False, 'trim_dsp': False, 'logging': True,
               'config_sha256': hashlib.sha256(configuration.encode()).hexdigest(),
               'compile_commands_sha256': hashlib.sha256((build / 'compile_commands.json').read_bytes()).hexdigest()}
    (build / 'reference-profile.json').write_text(json.dumps(profile, indent=2) + '\n')
    print(f'Built {len(units)} original portable decoder translation units: {build}')


def build_asm(args):
    """The original with its assembly, as the SIMD oracle (ADR 0062)."""
    subprocess.run(['python3', ROOT / 'tools/verify_upstream.py'], check=True)
    meson = shutil.which('meson') or str(ROOT / 'build/dav1d-tools/bin/meson')
    source = ROOT / 'build/dav1d-native-source'
    copy_inputs(json.loads((ROOT / 'provenance/dav1d-source.json').read_text()), source)
    tests = json.loads((ROOT / 'provenance/dav1d-test-inputs.json').read_text())
    copy_inputs(tests['checkasm'], source / 'subprojects/checkasm')
    profile = 'debug' if args.debug else 'release'
    suffix = '-x86_64' if args.cross else ''
    build = ROOT / f'build/dav1d-asm-{profile}{suffix}'
    flags = '-ffp-contract=off -fvisibility=default'
    setup = [meson, 'setup', build, source, '-Denable_asm=true', '-Dbitdepths=8,16',
             '-Dtrim_dsp=false', '-Denable_tests=false', '-Denable_tools=true', '-Dxxhash_muxer=disabled',
             '--default-library=static', '--wrap-mode=nofallback', f'-Dc_args={flags}',
             '--buildtype=debug' if args.debug else '--buildtype=release']
    if args.cross:
        cross = build.parent / f'dav1d-cross-{args.cross}.ini'
        cross.write_text('[binaries]\nc = \'x86_64-linux-gnu-gcc\'\nar = \'x86_64-linux-gnu-ar\'\n'
                         'strip = \'x86_64-linux-gnu-strip\'\nnasm = \'nasm\'\n'
                         "exe_wrapper = ['qemu-x86_64', '-cpu', 'max', '-L', '/usr/x86_64-linux-gnu']\n"
                         '[host_machine]\nsystem = \'linux\'\ncpu_family = \'x86_64\'\n'
                         'cpu = \'x86_64\'\nendian = \'little\'\n')
        setup += ['--cross-file', cross]
    if (build / 'build.ninja').exists():
        setup.append('--reconfigure')
    subprocess.run([str(x) for x in setup], check=True)
    subprocess.run(['ninja', '-C', build, *ninja_jobs(), 'src/libdav1d.a', 'tools/dav1d'], check=True)
    configuration = (build / 'config.h').read_text()
    for setting in ['HAVE_ASM 1', 'CONFIG_8BPC 1', 'CONFIG_16BPC 1', 'TRIM_DSP_FUNCTIONS 0', 'CONFIG_LOG 1']:
        if f'#define {setting}\n' not in configuration:
            raise SystemExit(f'Missing selected configuration: {setting}')
    assembly = sorted(str(p.relative_to(ROOT)) for p in (ROOT / 'upstream/dav1d/src').rglob('*')
                      if p.suffix in ('.S', '.asm'))
    compiler = os.environ.get('CC', 'x86_64-linux-gnu-gcc' if args.cross else 'cc')
    profile_record = {'reference': 'dav1d 1.5.4 original with assembly (SIMD oracle)', 'acceptance_reference': True,
                      'profile': profile, 'c_flags': flags, 'assembly': True, 'trim_dsp': False,
                      'target': args.cross or 'native',
                      'compiler': subprocess.check_output([compiler, '--version'], text=True).splitlines()[0],
                      'bitdepth_instantiations': [8, 16],
                      'levels': 'selected at run time with dav1d_set_cpu_flags_mask (neon 0x1; avx2 0x2f)',
                      'config_sha256': hashlib.sha256(configuration.encode()).hexdigest(),
                      'assembly_sources_sha256': hashlib.sha256(b''.join(
                          (ROOT / p).read_bytes() for p in assembly)).hexdigest(),
                      'archive_sha256': hashlib.sha256((build / 'src/libdav1d.a').read_bytes()).hexdigest()}
    (build / 'reference-profile.json').write_text(json.dumps(profile_record, indent=2) + '\n')
    print(f'Built the original with its assembly: {build}')


if __name__ == '__main__':
    main()
