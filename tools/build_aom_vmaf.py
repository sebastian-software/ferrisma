#!/usr/bin/env python3
"""Build the pinned native VMAF dependency for isolated libaom references."""
import argparse
import json
import os
from pathlib import Path
import shutil
import subprocess

ROOT = Path(__file__).resolve().parents[1]
REVISION = 'f85a853692a8c730d0270cd733c8bb30b5b93b7c'


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--fetch', action='store_true')
    args = parser.parse_args()
    source = ROOT / 'build/deps/vmaf'
    if not source.exists():
        if not args.fetch:
            raise SystemExit('Missing pinned VMAF source; use --fetch')
        subprocess.run(['git', 'clone', '--depth=1', '--branch', 'v3.2.1',
                        'https://github.com/Netflix/vmaf.git', source], check=True)
    if subprocess.check_output(['git', '-C', source, 'rev-parse', 'HEAD'], text=True).strip() != REVISION:
        raise SystemExit('Unexpected VMAF revision')
    if subprocess.check_output(['git', '-C', source, 'status', '--porcelain', '--untracked-files=no'], text=True).strip():
        raise SystemExit('Modified original VMAF source')
    meson = str(ROOT / 'build/aom-tools/bin/meson')
    if not Path(meson).exists():
        meson = shutil.which('meson') or str(ROOT / 'build/dav1d-tools/bin/meson')
    if subprocess.check_output([meson, '--version'], text=True).strip() != '1.7.2':
        raise SystemExit('Use Meson 1.7.2 for the recorded native VMAF reference')
    build = ROOT / 'build/aom-vmaf'
    prefix = ROOT / 'build/aom-vmaf-install'
    command = [meson, 'setup', build, source / 'libvmaf', '--buildtype=release',
               '--prefix', prefix, '--libdir=lib', '-Ddefault_library=static',
               '-Denable_asm=false', '-Denable_tests=false', '-Denable_docs=false',
               '-Denable_tools=false', '-Dbuilt_in_models=true', '-Denable_float=true']
    if (build / 'build.ninja').exists():
        command.append('--reconfigure')
    subprocess.run([str(value) for value in command], check=True)
    subprocess.run(['ninja', '-C', build, '-j', '4', 'install'], check=True)
    environment = dict(os.environ, PKG_CONFIG_PATH=str(prefix / 'lib/pkgconfig'))
    version = subprocess.check_output(['pkg-config', '--modversion', 'libvmaf'], env=environment, text=True).strip()
    if version != '3.2.0':
        raise SystemExit(f'Unexpected upstream VMAF project version: {version}')
    record = dict(repository='https://github.com/Netflix/vmaf.git', tag='v3.2.1', revision=REVISION,
                  upstream_project_version=version, assembly=False, float_extractors=True,
                  native_quality_engine=True, rust_port=False,
                  pkg_config_path=str(prefix / 'lib/pkgconfig'))
    (build / 'reference-profile.json').write_text(json.dumps(record, indent=2) + '\n')
    print(f'Native VMAF reference installed. Set PKG_CONFIG_PATH={prefix / "lib/pkgconfig"}')


if __name__ == '__main__':
    main()
