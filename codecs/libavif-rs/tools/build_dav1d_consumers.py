#!/usr/bin/env python3
"""Relink unchanged original dav1d consumers against the standalone Rust archive.

With `--portable` the port is built with `--cfg dav1d_portable`, the
counterpart of the original's portable build (`-Denable_asm=false`): no CPU
flags and msac's C functions (ADR 0062). It goes to its own target directory
(`build/dav1d-portable-target`) and the consumers to
`build/dav1d-rust-<profile>-portable`; the whole-image scalar gate
(`tools/check_dav1d_simd_original.py --level scalar`) decodes with them.
"""
import argparse
import json
import os
import re
from pathlib import Path
import shlex
import subprocess

ROOT = Path(__file__).resolve().parents[1]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--debug', action='store_true')
    parser.add_argument('--portable', action='store_true')
    args = parser.parse_args()
    profile = 'debug' if args.debug else 'release'
    build = ROOT / f'build/dav1d-reference-{profile}'
    output = ROOT / (f'build/dav1d-rust-{profile}' + ('-portable' if args.portable else ''))
    tests = json.loads((build / 'meson-info/intro-tests.json').read_text())
    cargo = ['cargo', 'build', '-p', 'ferrisma-dav1d', '--locked']
    if not args.debug:
        cargo.append('--release')
    environment = os.environ.copy()
    target = ROOT.parents[1] / 'target'
    if args.portable:
        target = ROOT / 'build/dav1d-portable-target'
        cargo += ['--target-dir', str(target)]
        environment['RUSTFLAGS'] = (environment.get('RUSTFLAGS', '') + ' --cfg dav1d_portable').strip()
    subprocess.run(cargo, check=True, env=environment)
    archive = target / ('debug' if args.debug else 'release') / 'libdav1d.a'
    linkage = []
    for executable in sorted({test['cmd'][0] for test in tests}):
        relative = Path(executable).relative_to(build)
        target = output / relative
        target.parent.mkdir(parents=True, exist_ok=True)
        command = subprocess.check_output(['ninja', '-C', build, '-t', 'commands', relative], text=True).splitlines()[-1]
        tokens = shlex.split(command)
        tokens[tokens.index('-o') + 1] = str(target)
        count = 0
        for index, token in enumerate(tokens):
            if re.fullmatch(r'src/libdav1d\.(?:(?:\d+\.)*dylib|a|so(?:\.\d+)*)', token):
                tokens[index] = str(archive)
                count += 1
        if relative.parts[0] in ['tools'] or 'libfuzzer' in relative.parts or relative.name == 'seek_stress':
            if count != 1:
                raise SystemExit(f'Expected one explicit decoder replacement: {relative}')
        if any(token.startswith('src/libdav1d') for token in tokens):
            raise SystemExit('Original decoder link input remains')
        subprocess.run(tokens, cwd=build, check=True)
        linkage.append({'executable': str(relative), 'decoder_replacements': count, 'link_command': tokens})
    for test in tests:
        test['cmd'][0] = str(output / Path(test['cmd'][0]).relative_to(build))
    (output / 'original-tests.json').write_text(json.dumps(tests, indent=2) + '\n')
    (output / 'linkage.json').write_text(json.dumps(linkage, indent=2) + '\n')
    print(f'Relinked {len(linkage)} unchanged original consumers; retained {len(tests)} original test definitions')


if __name__ == '__main__':
    main()
