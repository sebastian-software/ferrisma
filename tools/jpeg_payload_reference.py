#!/usr/bin/env python3
"""Audit the complete pinned DecodeJPEGData source and retained owner histories."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import platform
import shlex
import subprocess
from skcms_binding import live_bindings
from check import check_source
ROOT = Path(__file__).resolve().parent.parent

def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()

def observe(binary, payload):
    result = subprocess.run([str(binary)], input=payload, capture_output=True, check=True)
    if result.stderr:
        raise RuntimeError(result.stderr.decode()[:2000])
    if len(result.stdout.splitlines()) != len(payload.splitlines()):
        raise RuntimeError('Original JPEG payload response inventory changed')
    return result.stdout

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=False)
    check_source()
    capture = output / 'requests.txt'
    env = dict(os.environ, JXL_JPEG_PAYLOAD_CAPTURE=str(capture),
               JXL_CPP_ORACLE=str(ROOT / 'target/oracle/jxl_oracle'))
    with (output / 'parity.log').open('w') as log:
        subprocess.run(['cargo', 'test', '--locked', '--offline', '--all-features', '--test',
                        'parity', 'jpeg_metadata::jpeg_metadata_complete_brotli_payloads',
                        '--', '--nocapture'], cwd=ROOT, env=env, stdout=log,
                       stderr=subprocess.STDOUT, check=True)
    payload = capture.read_bytes()
    if not payload or any(not q.startswith(b'jm-ops ') for q in payload.splitlines()):
        raise RuntimeError('Complete payload history inventory missing')
    expected = None
    report = {'requests': len(payload.splitlines()), 'requests_sha256': sha(capture), 'graphs': {}}
    for graph in ['oracle', 'oracle-asan']:
        build = ROOT / 'target' / graph
        directory = output / graph
        directory.mkdir()
        entries = json.loads((build / 'compile_commands.json').read_text())
        checked = []
        for name in ['dec_jpeg_data.cc', 'jpeg_metadata.cc']:
            entry, = [e for e in entries if Path(e['file']).name == name]
            flags = entry.get('arguments') or shlex.split(entry['command'])
            if [f for f in flags if f.startswith('-O')] != ['-O3'] or '-DNDEBUG' not in flags:
                raise RuntimeError('Original JPEG payload optimization changed')
            wanted = ['-fsanitize=address,undefined', '-fno-sanitize-recover=all'] if graph == 'oracle-asan' else []
            if [f for f in flags if f.startswith('-f') and 'sanitize' in f] != wanted:
                raise RuntimeError('Original JPEG payload instrumentation changed')
            checked.append(entry)
        link = shlex.split((build / 'CMakeFiles/jxl_oracle.dir/link.txt').read_text())
        inputs = [build / 'jxl_oracle', build / 'compile_commands.json', build / 'CMakeFiles/jxl_oracle.dir/link.txt']
        inputs += [Path(x) if Path(x).is_absolute() else build / x for x in link if x.endswith(('.a', '.o'))]
        before = {str(p): sha(p) for p in inputs}
        binary, mapping = directory / 'bound', directory / 'link.map'
        link[link.index('-o') + 1] = str(binary)
        link.append(('-Wl,-map,' if platform.system() == 'Darwin' else '-Wl,-Map,') + str(mapping))
        with (directory / 'link.log').open('w') as log:
            subprocess.run(link, cwd=build, stdout=log, stderr=subprocess.STDOUT, check=True)
        bindings = live_bindings(binary, mapping, {
            'DecodeJPEGData': (r'_+ZN3jxl4jpeg14DecodeJPEGData[^.]*', r'libjxl-internal\.a\(dec_jpeg_data\.cc\.o\)'),
            'BrotliDecoderDecompressStream': (r'_?BrotliDecoderDecompressStream', r'libbrotlidec\.a\(decode\.c\.o\)'),
        })
        observations = observe(binary, payload)
        if expected is not None and observations != expected:
            raise RuntimeError('Original ordinary/instrumented payload histories differ')
        expected = observations
        (directory / 'observations.txt').write_bytes(observations)
        if any(sha(p) != digest for p, digest in before.items()):
            raise RuntimeError('Source audit changed an active original input')
        report['graphs'][graph] = {'compile': checked, 'bindings': bindings,
            'unchanged_inputs': before, 'observations_sha256': sha(directory / 'observations.txt')}
    check_source()
    (output / 'report.json').write_text(json.dumps(report, indent=2) + '\n')
    print(f"Complete DecodeJPEGData: {report['requests']} exact source owner histories")

if __name__ == '__main__':
    main()
