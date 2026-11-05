#!/usr/bin/env python3
"""Run observable native root build and validation phases with shared commands."""
import argparse
from pathlib import Path
import platform
import shlex
import subprocess

import root_variants

ROOT = Path(__file__).resolve().parents[1]


def run(command):
    command = [str(value) for value in command]
    print('+ ' + shlex.join(command), flush=True)
    subprocess.run(command, cwd=ROOT, check=True)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--phase', choices=['all', 'references', 'generate', 'consumers', 'validate'], default='all')
    parser.add_argument('--profile', choices=['both', 'release', 'debug'], default='release')
    parser.add_argument('--variant', default='default',
                        help='Container features and optional external engines, for example libgav1-rav1e-svt-libyuv')
    parser.add_argument('--tier', choices=['prepare', 'smoke', 'regression', 'full'], default='smoke')
    parser.add_argument('--clang-include', type=Path, required=True)
    parser.add_argument('--jobs', type=int, default=2)
    parser.add_argument('--sanitizer', choices=['none', 'address'], default='none',
                        help='Also build and compare separately instrumented original C and Rust consumers')
    parser.add_argument('--package', choices=['profile', 'production'], default='profile',
                        help='Link consumers to an isolated generated profile or to the production package')
    args = parser.parse_args()
    if args.jobs < 1:
        parser.error('--jobs must be positive')
    if args.sanitizer != 'none' and (args.profile != 'release' or args.tier != 'full'):
        parser.error('Root sanitizer parity uses original Release preprocessing and the complete original suite')
    try:
        selected = root_variants.features(args.variant)
    except ValueError as error:
        parser.error(str(error))
    engines = root_variants.engines(args.variant)
    production = args.package == 'production'
    if production and args.sanitizer != 'none':
        parser.error('Production sanitizer parity is not yet part of the admission evidence')
    if args.sanitizer != 'none' and engines:
        parser.error('Root sanitizer parity currently covers container configurations without optional engines')
    system, machine = platform.system(), platform.machine()
    if (system, machine) == ('Linux', 'x86_64'):
        native, triple = 'linux', 'x86_64-unknown-linux-gnu'
    elif (system, machine) == ('Darwin', 'arm64'):
        native, triple = 'macos', 'aarch64-apple-darwin'
    else:
        parser.error('Native root preparation currently requires GNU Linux x86-64 or macOS ARM64')
    if production and native == 'linux' and args.variant not in root_variants.LINUX_PRODUCTION:
        parser.error('The production package admits no Linux profile for this configuration; validate it isolated first')
    if production and native == 'macos' and args.profile != 'release':
        # The macOS package compiles its accepted Release-preprocessed sources
        # in both Cargo profiles; this harness has no matching Debug reference.
        parser.error('macOS production consumers compare original Release preprocessing')
    phases = ['references', 'generate', 'consumers', 'validate'] if args.phase == 'all' else [args.phase]
    for phase in phases:
        if args.tier == 'prepare' and phase in ['consumers', 'validate']:
            continue
        for profile in (['release', 'debug'] if args.profile == 'both' else [args.profile]):
            print(f'Native root phase: {native} / {profile} / {args.variant} / {phase}', flush=True)
            debug = ['--debug'] if profile == 'debug' else []
            original_debug = ['--upstream-debug'] if profile == 'debug' else []
            features = ['--' + name for name in selected]
            baseline = f'build/root-c-{native}-{profile}-{args.variant}'
            candidate = f'build/root-rust-{"production-" if production else ""}{native}-{profile}-{args.variant}'
            # The sanitized consumers are separate builds. Source generation
            # always uses the ordinary original preprocessing reference.
            asan = args.sanitizer == 'address'
            manifest = f'translations/root-{native}-{profile}-{args.variant}.json'
            references = f'build/root-references-{triple}-{profile}'
            if phase == 'references':
                run(['python3', 'tools/build_root_references.py', '--jobs', args.jobs, *debug])
                run(['python3', 'tools/build_aom.py', '--jobs', args.jobs, '--library-only', *debug])
                if engines:
                    # The pinned original engines are shared by both consumer sets.
                    run(['python3', 'tools/prepare_optional_deps.py', '--jobs', args.jobs,
                         *['--' + name for name in engines]])
                run(['python3', 'tools/build.py', 'baseline', '--fetch-deps', '--jobs', args.jobs,
                     *debug, *original_debug, *features, '--aom-reference-build', f'build/aom-native-{profile}',
                     '--root-references', references, '--build-dir', baseline])
                if asan:
                    run(['python3', 'tools/aom_sanitizer.py', '--root-reference'])
                    run(['python3', 'tools/build.py', 'baseline', '--asan', '--instrumented-aom-reference',
                         '--jobs', args.jobs, *features, '--root-references', references,
                         '--build-dir', baseline + '-asan'])
            elif phase == 'generate':
                run(['python3', 'tools/transpile_root.py', '--build', baseline, '--profile', profile,
                     '--variant', args.variant, '--clang-include', args.clang_include])
                if production:
                    # The package compiles the committed profile; regeneration
                    # from this host's actual C reference must not change it.
                    run(['git', 'diff', '--exit-code', '--', f'src/profiles/{native}/{profile}/{args.variant}'])
                    run(['python3', 'tools/update_production_selection.py', '--check'])
                else:
                    run(['python3', 'tools/check_root_profile.py', manifest])
            elif phase == 'consumers':
                run(['python3', 'tools/build.py', 'port', '--jobs', args.jobs, *debug, *original_debug, *features,
                     *(['--asan'] if asan else []), '--root-references', references,
                     '--production-profile' if production else '--root-profile', manifest,
                     '--build-dir', candidate + ('-asan' if asan else '')])
            elif asan:
                run(['python3', 'tools/validate_root_profile.py', '--baseline', baseline + '-asan',
                     '--port', candidate + '-asan', '--reference-baseline', baseline, '--sanitizer', 'address',
                     '--root-profile', manifest, '--tier', args.tier, '--jobs', args.jobs])
            else:
                run(['python3', 'tools/validate_root_profile.py', '--baseline', baseline, '--port', candidate,
                     '--root-profile', manifest, '--tier', args.tier, '--jobs', args.jobs,
                     *(['--production'] if production else [])])


if __name__ == '__main__':
    main()
