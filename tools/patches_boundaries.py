#!/usr/bin/env python3
"""Census every original patch-tree request in independent original processes.

The actual Rust fixture generator supplies all 1,728 unchanged requests. Source
metadata independently predicts zero-size aliased weighted-add calls. Native
Linux UBSan must diagnose exactly those calls; every defined observation remains
an exact ordinary/instrumented comparison. No source algorithm is reproduced.
"""
import argparse
import hashlib
import json
from pathlib import Path
import platform
import re
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parent.parent


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def invoke(binary, request):
    result = subprocess.run([str(binary)], input=request + '\n', text=True,
                            capture_output=True)
    return {'status': result.returncode, 'stdout': result.stdout, 'stderr': result.stderr}


def exact_null_memcpy(stderr):
    lines = [line for line in stderr.splitlines() if line]
    return (len(lines) == 3
            and 'alpha.cc:70:' in lines[0]
            and lines[0].endswith('runtime error: null pointer passed as argument 1, '
                                  'which is declared to never be null')
            and lines[1].endswith('note: nonnull attribute specified here')
            and 'SUMMARY: UndefinedBehaviorSanitizer: undefined-behavior ' in lines[2]
            and lines[2].rstrip().endswith('alpha.cc:70:12'))


def actual_requests(binary, temporary):
    source = (ROOT / 'tests/parity/patches.rs').read_text()
    functions = []
    for name in ('fixture', 'tokens', 'read', 'patch_tree_cases'):
        match = re.search(r'fn ' + name + r'\(.*?^\}', source, re.M | re.S)
        if match is None:
            raise RuntimeError('Actual Rust patch fixture generator missing: ' + name)
        functions.append(match.group(0))
    program = r'''
use std::fmt::Write as _;
use std::io::Write as _;
use std::process::{Command,Stdio};
fn original_observations(requests:&[String])->Vec<String>{
    let mut child=Command::new(std::env::args().nth(1).unwrap())
        .stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    for request in requests {writeln!(child.stdin.as_mut().unwrap(),"{request}").unwrap();}
    drop(child.stdin.take());
    let output=child.wait_with_output().unwrap();
    assert!(output.status.success()&&output.stderr.is_empty(),"{:?}",output);
    String::from_utf8(output.stdout).unwrap().lines().map(str::to_owned).collect()
}
fn hex(bytes:&[u8])->String{bytes.iter().map(|v|format!("{v:02x}")).collect()}
fn unhex(text:&str)->Vec<u8>{(0..text.len()).step_by(2)
    .map(|i|u8::from_str_radix(&text[i..i+2],16).unwrap()).collect()}
'''
    program += '\n'.join(functions)
    program += '\nfn main(){for (request,undefined) in patch_tree_cases(){'
    program += 'println!("{} {}",u8::from(undefined),request);}}\n'
    code, generator = temporary / 'requests.rs', temporary / 'requests'
    code.write_text(program)
    subprocess.run(['rustc', '--edition=2024', str(code), '-o', str(generator)], check=True)
    result = subprocess.run([str(generator), str(binary)], text=True, capture_output=True, check=True)
    if result.stderr:
        raise RuntimeError('Unexpected source encoder stderr: ' + result.stderr)
    cases = [(line[2:], line.startswith('1 ')) for line in result.stdout.splitlines()]
    if len(cases) != 1728 or sum(undefined for _, undefined in cases) != 48:
        raise RuntimeError('Actual original patch tree inventory changed')
    return cases


def source_metadata(binary, cases):
    # Decode the same exact original streams without rendering, so the original
    # dictionary supplies positions, reference rectangles and blendings. This
    # observes source state; it does not decode entropy in the audit driver.
    requests = {}
    for request, _ in cases:
        fields = request.split()
        fields[7] = '0'
        requests[' '.join(fields)] = None
    result = subprocess.run([str(binary)], input='\n'.join(requests) + '\n',
                            text=True, capture_output=True, check=True)
    lines = result.stdout.splitlines()
    if result.stderr or len(lines) != len(requests):
        raise RuntimeError('Unexpected original metadata response: ' + result.stderr)
    for request, line in zip(requests, lines):
        if not line.startswith('D 0 '):
            raise RuntimeError('Original fixture stream failed to decode: ' + request)
        requests[request] = {
            'observation': line,
            'positions': [tuple(map(int, values)) for values in re.findall(r' P (\d+) (\d+) (\d+)', line)],
            'references': [tuple(map(int, values)) for values in re.findall(r' R (\d+) (\d+) (\d+) (\d+) (\d+)', line)],
            'blendings': [tuple(map(int, values)) for values in re.findall(r' B (\d+) (\d+) (\d+)', line)],
        }
    evidence = []
    for index, (request, predicted) in enumerate(cases):
        fields = request.split()
        n, render = int(fields[1]), int(fields[7])
        x0, width, left, right = map(int, fields[8:12])
        fields[7] = '0'
        metadata = requests[' '.join(fields)]
        triggers = []
        if render:
            x0 -= left
            span = width + left + right
            for pos_index, (bx, by, ref_index) in enumerate(metadata['positions']):
                reference = metadata['references'][ref_index]
                patch_width, patch_height = reference[3:5]
                # Exact AddOneRow strict endpoint checks (source lines340–343).
                if bx >= x0 + span or bx + patch_width < x0:
                    continue
                count = min(bx + patch_width, x0 + span) - max(bx, x0)
                if count == 0 and by < int(fields[4]) and patch_height:
                    aliases = [i for i in range(n)
                               if metadata['blendings'][pos_index * (n + 1) + 1 + i][0] in (6, 7)
                               and metadata['blendings'][pos_index * (n + 1) + 1 + i][1] == i]
                    if aliases:
                        triggers.append({'position_index': pos_index, 'position': [bx, by, ref_index],
                                         'reference': reference, 'aliased_extra_channels': aliases})
        if bool(triggers) != predicted:
            raise RuntimeError('Source metadata disagrees with predicted boundary at ' + str(index))
        evidence.append(triggers)
    return requests, evidence


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--output', type=Path, default=ROOT / 'target/patches-boundaries.json')
    args = parser.parse_args()
    builds = {name: ROOT / 'target' / path / 'jxl_oracle'
              for name, path in (('ordinary', 'oracle'), ('ASan', 'oracle-asan'))}
    source = ROOT / 'upstream/libjxl'
    pin = json.loads((ROOT / 'upstream.json').read_text())['revision']
    revision = subprocess.check_output(['git', '-C', str(source), 'rev-parse', 'HEAD'], text=True).strip()
    dirty = subprocess.check_output(['git', '-C', str(source), 'status', '--porcelain',
                                     '--ignore-submodules=none'], text=True).strip()
    if revision != pin or dirty:
        raise RuntimeError('Pinned original source or dependencies changed')
    report = {'platform': platform.platform(), 'revision': revision,
              'port_revision': subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=ROOT, text=True).strip(),
              'generator_source_sha256': sha(ROOT / 'tests/parity/patches.rs'),
              'source_hashes': {name: sha(source / 'lib/jxl' / name)
                                for name in ('alpha.cc', 'blending.cc', 'dec_patch_dictionary.cc')},
              'graphs': {}, 'native_linux_confirmed': False}
    with tempfile.TemporaryDirectory(prefix='jxl-patch-boundaries-') as directory:
        cases = actual_requests(builds['ordinary'], Path(directory))
        metadata, evidence = source_metadata(builds['ordinary'], cases)
        report['source_metadata'] = metadata
        report['cases'] = [{'index': index, 'request': request, 'predicted_undefined': undefined,
                            'source_triggers': evidence[index], 'observations': {}}
                           for index, (request, undefined) in enumerate(cases)]
        report['requests'], report['predicted_undefined'] = len(cases), sum(v for _, v in cases)
        report['requests_sha256'] = hashlib.sha256(
            ('\n'.join(request for request, _ in cases) + '\n').encode()).hexdigest()
        errors = []
        for graph, binary in builds.items():
            before = sha(binary)
            configuration = invoke(binary, 'patch-sanitizers')
            expected = '1\n' if graph == 'ASan' else '0\n'
            if configuration != {'status': 0, 'stdout': expected, 'stderr': ''}:
                raise RuntimeError('Original instrumentation changed: ' + graph)
            diagnosed = []
            for row in report['cases']:
                observation = invoke(binary, row['request'])
                row['observations'][graph] = observation
                recognized = exact_null_memcpy(observation['stderr'])
                if observation['status'] == 0 and len(observation['stdout'].splitlines()) != 1:
                    errors.append(f"Original response count changed in {graph} at {row['index']}")
                if recognized and observation['status'] != 0:
                    diagnosed.append(row['index'])
                elif observation['status'] != 0 or observation['stderr']:
                    errors.append(f"Unexpected {graph} source failure at {row['index']}")
                if not row['predicted_undefined'] and (recognized or observation['status'] != 0):
                    errors.append(f"Defined {graph} request diagnosed at {row['index']}")
                if graph == 'ASan' and platform.system() == 'Linux':
                    if row['predicted_undefined'] != (recognized and observation['status'] != 0):
                        errors.append(f"Native Linux boundary set changed at {row['index']}")
                if not row['predicted_undefined'] and graph == 'ASan':
                    if observation != row['observations']['ordinary']:
                        errors.append(f"Defined ordinary/ASan observation mismatch at {row['index']}")
            if sha(binary) != before:
                raise RuntimeError('Active original binary changed')
            report['graphs'][graph] = {'binary_sha256': before, 'diagnosed_indices': diagnosed}
            print(f'{graph}: independently executed all {len(cases)} original requests; '
                  f'{len(diagnosed)} exact alpha.cc:70 diagnostics', flush=True)
        report['errors'] = errors
        report['native_linux_confirmed'] = platform.system() == 'Linux' and not errors
        args.output.parent.mkdir(parents=True, exist_ok=True)
        args.output.write_text(json.dumps(report, indent=2) + '\n')
        print(f'{len(cases)-report["predicted_undefined"]} defined observations retained; '
              f'native Linux confirmation={report["native_linux_confirmed"]}; proof={args.output}', flush=True)
        if errors:
            raise RuntimeError('\n'.join(errors))


if __name__ == '__main__':
    main()
