#!/usr/bin/env python3
"""Compare explicit odd-address input validation with the unchanged C codec."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import subprocess


ROOT = Path(__file__).resolve().parents[1]


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--debug', action='store_true')
    parser.add_argument('--reference-build', type=Path)
    parser.add_argument('--rust-archive', type=Path)
    parser.add_argument('--output-dir', type=Path)
    args = parser.parse_args()
    profile = 'debug' if args.debug else 'release'
    native = args.reference_build or ROOT / f'build/aom-native-{profile}'
    reference = native / 'libaom.a'
    candidate = args.rust_archive or ROOT / f'target/{profile}/libaom.a'
    work = args.output_dir or ROOT / f'build/aom-unaligned-input-{profile}'
    work.mkdir(parents=True, exist_ok=True)
    (work / 'summary.json').unlink(missing_ok=True)
    platform = 'macos' if os.uname().sysname == 'Darwin' else 'linux'
    manifest = ROOT / f'crates/aom/TRANSLATION-{platform}-{profile}.json'
    translation = json.loads(manifest.read_text())
    if (json.loads((native / 'reference-profile.json').read_text())['profile'] != profile or
            digest(native / 'config/aom_config.h') !=
            translation['generated_reference_headers']['config/aom_config.h']):
        raise SystemExit('The native reference and Rust preprocessing differ')
    source = ROOT / 'tests/ports/aom_unaligned_input.c'
    compiler = os.environ.get('CC', 'cc')
    caller = work / 'caller.o'
    subprocess.run([compiler, '-std=c11', '-ffp-contract=off', '-g', '-I', ROOT / 'upstream/aom',
                    '-c', source, '-o', caller], check=True)
    for implementation, archive in [('c', reference), ('rust', candidate)]:
        subprocess.run([compiler, caller, archive, '-lm', '-lpthread',
                        '-o', work / implementation], check=True)
    records = []
    for depth in [8, 10, 12]:
        for offset in [0, 1]:
            for invalid_plane in range(4):
                name = f'{depth}-{offset}-{invalid_plane}'
                outputs = []
                for implementation in ['c', 'rust']:
                    with (work / f'{name}-{implementation}.log').open('wb') as log:
                        value = subprocess.check_output([work / implementation, str(depth), str(offset),
                                                         str(invalid_plane)], stderr=log)
                    (work / f'{name}-{implementation}.bin').write_bytes(value)
                    outputs.append(value)
                if outputs[0] != outputs[1]:
                    raise SystemExit(f'External high-bit-depth validation differs: {name}')
                records.append(dict(bitdepth=depth, address_parity=offset,
                                    invalid_plane=invalid_plane or None, exact=True,
                                    bytes=len(outputs[0]), sha256=hashlib.sha256(outputs[0]).hexdigest()))
                print(f'Exact external input validation: {name}', flush=True)
    (work / 'summary.json').write_text(json.dumps(dict(profile=profile,
        scope='Supplementary zero-sample encoding and last-sample range rejection; no original-suite credit',
        cases=records, original_archive_sha256=digest(reference), rust_archive_sha256=digest(candidate),
        source_sha256=digest(source), translation_manifest_sha256=digest(manifest)), indent=2) + '\n')


if __name__ == '__main__':
    main()
