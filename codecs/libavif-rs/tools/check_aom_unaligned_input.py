#!/usr/bin/env python3
"""Compare explicit odd-address input validation with the unchanged original.

Every case runs twice, once per reference (engine ADR-0041: scalar against
scalar, SIMD against SIMD on the same target; ADR 0046, amendment of
2026-11-21):

| Mode | Port archive | Original |
| --- | --- | --- |
| `simd` | RTCD dispatch on (no `AOM_SIMD_CAPS*`) | `build_aom.py --library-only --simd neon` (macOS ARM64) or `--simd avx2` (x86-64 GNU Linux, run-time detection) |
| `scalar` | `AOM_SIMD_CAPS_MASK=0` (every `_c` path) | `build_aom.py --library-only` (generic C) |

Both originals are built for the same profile (`--debug` or not). Every case
must be byte-identical in both modes.

    python3 tools/build_aom.py --library-only [--debug]
    python3 tools/build_aom.py --library-only [--debug] --simd avx2   # or neon
    python3 tools/check_aom_unaligned_input.py [--debug]
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import platform
import subprocess
import sys

from build_aom import reference_build

import aom_manifest


ROOT = Path(__file__).resolve().parents[1]
MACOS = sys.platform == 'darwin' and platform.machine() == 'arm64'
LINUX = sys.platform.startswith('linux') and platform.machine() == 'x86_64'
SIMD = 'neon' if MACOS else 'avx2'
# mode: (port environment, original build)
MODES = {'simd': ({}, SIMD), 'scalar': ({'AOM_SIMD_CAPS_MASK': '0'}, 'generic')}


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def environment(extra):
    env = {key: value for key, value in os.environ.items() if key not in ('AOM_SIMD_CAPS', 'AOM_SIMD_CAPS_MASK')}
    env.update(extra)
    return env


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument('--debug', action='store_true')
    parser.add_argument('--reference-build', type=Path, help='Generic C original (scalar mode)')
    parser.add_argument('--simd-reference-build', type=Path,
                        help=f'Original SIMD build of this target ({SIMD}; simd mode)')
    parser.add_argument('--rust-archive', type=Path)
    parser.add_argument('--output-dir', type=Path)
    args = parser.parse_args()
    if not (MACOS or LINUX):
        raise SystemExit('The port runs on macOS ARM64 and x86-64 GNU Linux only (crates/aom/src/lib.rs)')
    profile = 'debug' if args.debug else 'release'
    native = args.reference_build or reference_build(profile)
    simd_native = args.simd_reference_build or reference_build(profile, SIMD)
    candidate = args.rust_archive or ROOT.parents[1] / f'target/{profile}/libaom.a'
    work = args.output_dir or ROOT / f'build/aom-unaligned-input-{profile}'
    work.mkdir(parents=True, exist_ok=True)
    (work / 'summary.json').unlink(missing_ok=True)
    platform_name = 'macos' if MACOS else 'linux'
    manifest = aom_manifest.PATH
    translation = aom_manifest.view(platform_name, profile)
    if (json.loads((native / 'reference-profile.json').read_text())['profile'] != profile or
            digest(native / 'config/aom_config.h') !=
            translation['generated_reference_headers']['config/aom_config.h']):
        raise SystemExit('The native reference and Rust preprocessing differ')
    simd_profile_path = simd_native / 'reference-profile.json'
    if not simd_profile_path.exists():
        raise SystemExit(f'Missing original {SIMD} reference: run python3 tools/build_aom.py --library-only'
                         f'{" --debug" if args.debug else ""} --simd {SIMD}')
    simd_profile = json.loads(simd_profile_path.read_text())
    if (simd_profile.get('profile') != profile or simd_profile.get('simd') != SIMD
            or simd_profile.get('fp_contract') != 'off'
            or simd_profile.get('target_arch') != ('arm64' if MACOS else 'x86_64')):
        raise SystemExit(f'{simd_native} is not the noncontracting native {SIMD} {profile} reference')
    originals = {'generic': native / 'libaom.a', SIMD: simd_native / 'libaom.a'}
    source = ROOT / 'tests/ports/aom_unaligned_input.c'
    compiler = os.environ.get('CC', 'cc')
    caller = work / 'caller.o'
    subprocess.run([compiler, '-std=c11', '-ffp-contract=off', '-g', '-I', ROOT / 'upstream/aom',
                    '-c', source, '-o', caller], check=True)
    for implementation, archive in [*[(f'c-{name}', path) for name, path in originals.items()],
                                    ('rust', candidate)]:
        subprocess.run([compiler, caller, archive, '-lm', '-lpthread',
                        '-o', work / implementation], check=True)
    records = []
    for depth in [8, 10, 12]:
        for offset in [0, 1]:
            for invalid_plane in range(4):
                for mode, (port_env, original) in MODES.items():
                    name = f'{depth}-{offset}-{invalid_plane}'
                    outputs = []
                    for implementation, env in [(f'c-{original}', {}), ('rust', port_env)]:
                        with (work / f'{name}-{mode}-{implementation}.log').open('wb') as log:
                            value = subprocess.check_output([work / implementation, str(depth), str(offset),
                                                             str(invalid_plane)], stderr=log,
                                                            env=environment(env))
                        (work / f'{name}-{mode}-{implementation}.bin').write_bytes(value)
                        outputs.append(value)
                    if outputs[0] != outputs[1]:
                        raise SystemExit(f'External high-bit-depth validation differs: {name} ({mode}: port '
                                         f'{hashlib.sha256(outputs[1]).hexdigest()[:16]}, original {original} '
                                         f'{hashlib.sha256(outputs[0]).hexdigest()[:16]})')
                    records.append(dict(bitdepth=depth, address_parity=offset,
                                        invalid_plane=invalid_plane or None, mode=mode, original=original,
                                        port_environment=port_env, exact=True, bytes=len(outputs[0]),
                                        sha256=hashlib.sha256(outputs[0]).hexdigest()))
                    print(f'Exact external input validation: {name} {mode} (original {original})', flush=True)
    (work / 'summary.json').write_text(json.dumps(dict(profile=profile,
        scope='Supplementary zero-sample encoding and last-sample range rejection; no original-suite credit',
        references={mode: dict(port_environment=port_env, original=original) for mode, (port_env, original)
                    in MODES.items()},
        cases=records, original_archive_sha256=digest(originals['generic']),
        original_simd_archive_sha256=digest(originals[SIMD]), rust_archive_sha256=digest(candidate),
        source_sha256=digest(source), translation_manifest_sha256=digest(manifest)), indent=2) + '\n')


if __name__ == '__main__':
    main()
