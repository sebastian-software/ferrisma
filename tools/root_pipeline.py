#!/usr/bin/env python3
"""Run observable native root build and validation phases with shared commands."""
import argparse
from pathlib import Path
import platform
import shlex
import subprocess

ROOT = Path(__file__).resolve().parents[1]


def run(command):
    command = [str(value) for value in command]
    print('+ ' + shlex.join(command), flush=True)
    subprocess.run(command, cwd=ROOT, check=True)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--phase', choices=['all', 'references', 'generate', 'consumers', 'validate'], default='all')
    parser.add_argument('--profile', choices=['both', 'release', 'debug'], default='release')
    parser.add_argument('--variant', choices=['default', 'mini', 'extended-pixi', 'mini-extended-pixi'], default='default')
    parser.add_argument('--tier', choices=['prepare', 'smoke', 'full'], default='smoke')
    parser.add_argument('--clang-include', type=Path, required=True)
    parser.add_argument('--jobs', type=int, default=2)
    args = parser.parse_args()
    if args.jobs < 1:
        parser.error('--jobs must be positive')
    system, machine = platform.system(), platform.machine()
    if (system, machine) == ('Linux', 'x86_64'):
        native, triple = 'linux', 'x86_64-unknown-linux-gnu'
    elif (system, machine) == ('Darwin', 'arm64'):
        native, triple = 'macos', 'aarch64-apple-darwin'
    else:
        parser.error('Native root preparation currently requires GNU Linux x86-64 or macOS ARM64')
    phases = ['references', 'generate', 'consumers', 'validate'] if args.phase == 'all' else [args.phase]
    for phase in phases:
        if args.tier == 'prepare' and phase in ['consumers', 'validate']:
            continue
        for profile in (['release', 'debug'] if args.profile == 'both' else [args.profile]):
            print(f'Native root phase: {native} / {profile} / {args.variant} / {phase}', flush=True)
            debug = ['--debug'] if profile == 'debug' else []
            original_debug = ['--upstream-debug'] if profile == 'debug' else []
            features = [flag for name, flag in [('mini', '--mini'), ('extended-pixi', '--extended-pixi')]
                        if name in args.variant]
            baseline = f'build/root-c-{native}-{profile}-{args.variant}'
            candidate = f'build/root-rust-{native}-{profile}-{args.variant}'
            manifest = f'translations/root-{native}-{profile}-{args.variant}.json'
            references = f'build/root-references-{triple}-{profile}'
            if phase == 'references':
                run(['python3', 'tools/build_root_references.py', '--jobs', args.jobs, *debug])
                run(['python3', 'tools/build_aom.py', '--jobs', args.jobs, '--library-only', *debug])
                run(['python3', 'tools/build.py', 'baseline', '--fetch-deps', '--jobs', args.jobs,
                     *debug, *original_debug, *features, '--aom-reference-build', f'build/aom-native-{profile}',
                     '--root-references', references, '--build-dir', baseline])
            elif phase == 'generate':
                run(['python3', 'tools/transpile_root.py', '--build', baseline, '--profile', profile,
                     '--variant', args.variant, '--clang-include', args.clang_include])
                run(['python3', 'tools/check_root_profile.py', manifest])
            elif phase == 'consumers':
                run(['python3', 'tools/build.py', 'port', '--jobs', args.jobs, *debug, *original_debug, *features,
                     '--root-references', references, '--root-profile', manifest, '--build-dir', candidate])
            else:
                run(['python3', 'tools/validate_root_profile.py', '--baseline', baseline, '--port', candidate,
                     '--root-profile', manifest, '--tier', args.tier, '--jobs', args.jobs])


if __name__ == '__main__':
    main()
