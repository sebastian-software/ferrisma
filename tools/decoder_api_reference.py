#!/usr/bin/env python3
"""Audit exact pinned decoder helpers and both original observation graphs."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import runpy
import shlex
import subprocess

ROOT = Path(__file__).resolve().parent.parent


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    args.output = args.output.resolve()
    args.output.mkdir(parents=True, exist_ok=True)
    capture = args.output / 'requests.txt'
    if capture.exists():
        raise RuntimeError('Use a fresh output directory to retain each audit')
    runpy.run_path(str(ROOT / 'tools/check.py'))['check_source']()
    fragments = json.loads((ROOT / 'tools/oracle/source_fragments.json').read_text())
    selected = [f for f in fragments if f['output'].startswith('original_decoder_')]
    if len(selected) != 5:
        raise RuntimeError('Complete original decoder-helper manifest missing')
    for fragment in selected:
        source = (ROOT / 'upstream/libjxl' / fragment['path']).read_text()
        if source.count(fragment['start']) != 1:
            raise RuntimeError('Ambiguous original function')
        start = source.index(fragment['start'])
        body = source[start:source.index(fragment['end'], start)] + '\n'
        if hashlib.sha256(body.encode()).hexdigest() != fragment['sha256']:
            raise RuntimeError('Original helper body changed')
    before = {}
    commands = {}
    public_symbols = {}
    for graph in ('oracle', 'oracle-asan'):
        build = ROOT / 'target' / graph
        entries = json.loads((build / 'compile_commands.json').read_text())
        matches = [entry for entry in entries if Path(entry['file']).name == 'decoder_api.cc']
        if len(matches) != 1:
            raise RuntimeError('Original decoder observer compilation missing')
        entry = matches[0]
        flags = entry.get('arguments') or shlex.split(entry['command'])
        instrumented = '-fsanitize=address,undefined' in flags
        if instrumented != (graph == 'oracle-asan'):
            raise RuntimeError('Original graph instrumentation changed')
        if instrumented and '-fno-sanitize-recover=all' not in flags:
            raise RuntimeError('Original nonrecovering sanitizer configuration missing')
        if any(flag.startswith('-fno-sanitize=') or flag in ('-O0', '-ffast-math') for flag in flags):
            raise RuntimeError('Unexpected observer compiler override')
        commands[graph] = flags
        for path in [build / 'jxl_oracle', build / 'compile_commands.json', *[
                build / 'generated' / f['output'] for f in selected]]:
            before[str(path)] = sha(path)
        symbols = subprocess.check_output(['nm', '-C', str(build / 'jxl_oracle')], text=True)
        # The final macOS executable internalizes this archive text symbol.
        matches = re.findall(r'^.*\b[Tt] _?JxlSignatureCheck$', symbols, re.M)
        if len(matches) != 1:
            raise RuntimeError('Actual public archive signature symbol missing')
        public_symbols[graph] = matches[0]
    environment = dict(os.environ, JXL_CPP_ORACLE=str(ROOT / 'target/oracle/jxl_oracle'),
                       JXL_DECODER_API_REQUEST_CAPTURE=str(capture),
                       CARGO_BUILD_JOBS='2', RUST_TEST_THREADS='2')
    with (args.output / 'capture.log').open('w') as log:
        subprocess.run(['cargo', 'test', '--locked', '--offline', '--all-features',
                        '--test', 'parity', 'decoder_api::', '--', '--nocapture'],
                       cwd=ROOT, env=environment, stdout=log, stderr=subprocess.STDOUT, check=True)
    requests = capture.read_text().splitlines()
    kinds = {line.split()[0] for line in requests}
    if kinds != {'da-constants', 'da-bounds', 'da-signature', 'da-box'}:
        raise RuntimeError('Incomplete actual Rust-generated request corpus')
    previous = None
    observations = {}
    for graph in ('oracle', 'oracle-asan'):
        result = subprocess.run([str(ROOT / 'target' / graph / 'jxl_oracle')],
                                input=capture.read_bytes(), capture_output=True, check=True)
        if result.stderr or len(result.stdout.splitlines()) != len(requests):
            raise RuntimeError('Unexpected original diagnostic or response count')
        if previous is not None and result.stdout != previous:
            raise RuntimeError('Ordinary and instrumented original helpers disagree')
        previous = result.stdout
        path = args.output / (graph + '.txt')
        path.write_bytes(result.stdout)
        observations[graph] = sha(path)
    if any(sha(Path(path)) != digest for path, digest in before.items()):
        raise RuntimeError('Audit changed an active original build artifact')
    report = {'requests': len(requests), 'requests_sha256': sha(capture),
              'ordinary_asan_exact': True, 'source_fragments': selected,
              'compile_commands': commands, 'public_signature_symbols': public_symbols,
              'observations': observations,
              'unchanged_active_artifacts': before}
    (args.output / 'report.json').write_text(json.dumps(report, indent=2) + '\n')
    print(f'Both original graphs agree on all {len(requests)} decoder-helper requests')


if __name__ == '__main__':
    main()
