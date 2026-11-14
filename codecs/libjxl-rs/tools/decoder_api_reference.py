#!/usr/bin/env python3
"""Audit exact pinned decoder helpers and both original observation graphs."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import platform
import re
import runpy
import shlex
import subprocess
from skcms_binding import live_bindings

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
    selected = [f for f in fragments if f['output'].startswith('original_decoder_')
                and not f['output'].startswith('original_decoder_input_')]
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
    bindings = {}
    for graph in ('oracle', 'oracle-asan'):
        build = ROOT / 'target' / graph
        entries = json.loads((build / 'compile_commands.json').read_text())
        matches = [entry for entry in entries if Path(entry['file']).name == 'decoder_api.cc']
        if len(matches) != 1:
            raise RuntimeError('Original decoder observer compilation missing')
        entry = matches[0]
        flags = entry.get('arguments') or shlex.split(entry['command'])
        original, = [row for row in entries if Path(row['file']).resolve() ==
                     ROOT / 'upstream/libjxl/lib/jxl/decode.cc']
        for compilation in (entry, original):
            options = compilation.get('arguments') or shlex.split(compilation['command'])
            expected = ['-fsanitize=address,undefined', '-fno-sanitize-recover=all'] if graph == 'oracle-asan' else []
            if [option for option in options if 'sanitize' in option] != expected:
                raise RuntimeError('Original nonrecovering sanitizer configuration changed')
            if [option for option in options if option.startswith('-O')] != ['-O3'] or '-DNDEBUG' not in options:
                raise RuntimeError('Original source/observer optimization configuration changed')
            if any(option.startswith(('-ffast-math', '-ffp-contract', '-ffp-model',
                                      '-funsafe-math', '-fno-signed-zeros')) for option in options):
                raise RuntimeError('Unexpected source/observer numeric override')
        commands[graph] = flags
        for fragment in selected:
            source = ROOT / 'upstream/libjxl' / fragment['path']
            text = source.read_text()
            start = text.index(fragment['start'])
            body = text[start:text.index(fragment['end'], start)] + '\n'
            notice = ''.join(text.splitlines(keepends=True)[:5])
            line = text[:start].count('\n') + 1
            expected = notice + f'#line {line} {json.dumps(str(source))}\n' + body
            if (build / 'generated' / fragment['output']).read_text() != expected:
                raise RuntimeError('Compiled generated helper differs from exact original source')
        for path in [build / 'jxl_oracle', build / 'compile_commands.json',
                     build / 'CMakeFiles/jxl_oracle.dir/link.txt',
                     build / 'CMakeFiles/jxl_oracle.dir/decoder_api.cc.o',
                     build / 'upstream/lib/libjxl-internal.a', *[
                build / 'generated' / f['output'] for f in selected]]:
            before[str(path)] = sha(path)
        symbols = subprocess.check_output(['nm', '-C', str(build / 'jxl_oracle')], text=True)
        # The final macOS executable internalizes this archive text symbol.
        matches = re.findall(r'^.*\b[Tt] _?JxlSignatureCheck$', symbols, re.M)
        if len(matches) != 1:
            raise RuntimeError('Actual public archive signature symbol missing')
        public_symbols[graph] = matches[0]
        directory = args.output / graph
        directory.mkdir()
        link = shlex.split((build / 'CMakeFiles/jxl_oracle.dir/link.txt').read_text())
        binary, mapping = directory / 'public-bound', directory / 'public.map'
        link[link.index('-o') + 1] = str(binary)
        link.append(('-Wl,-map,' if platform.system() == 'Darwin' else '-Wl,-Map,') + str(mapping))
        with (directory / 'link.log').open('w') as log:
            subprocess.run(link, cwd=build, stdout=log, stderr=subprocess.STDOUT, check=True)
        bindings[graph] = live_bindings(binary, mapping, {
            'JxlSignatureCheck': (r'_?JxlSignatureCheck',
                                 r'libjxl-internal\.a(?:\[\d+\])?\((?:.*/)?decode\.cc\.o\)$'),
            'DecoderApiCommand': (r'_+Z17DecoderApiCommand[^.]*', r'(?:^|/)decoder_api\.cc\.o$'),
        })
    environment = dict(os.environ, JXL_DECODER_TARGET='scalar', JXL_CPP_ORACLE=str(ROOT / 'target/oracle/jxl_oracle'),
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
    if len(requests) != 92452:
        raise RuntimeError('Retained decoder-helper request inventory changed')
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
              'live_source_bindings': bindings,
              'observations': observations,
              'unchanged_active_artifacts': before}
    (args.output / 'report.json').write_text(json.dumps(report, indent=2) + '\n')
    print(f'Both original graphs agree on all {len(requests)} decoder-helper requests')


if __name__ == '__main__':
    main()
