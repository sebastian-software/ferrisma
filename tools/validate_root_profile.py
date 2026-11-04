#!/usr/bin/env python3
"""Validate native root consumers against the identical original C configuration."""
import argparse
import json
from pathlib import Path
import re
import subprocess
import shutil
import tempfile

from check_root_profile import ROOT, digest, profile_work, validate_profile
from compare_results import results

SMOKE = ['avif16bittest', 'avifallocationtest', 'avifbasictest', 'avifchangesettingtest',
         'avifcodectest', 'avifdecodetest', 'avifimagetest', 'avifsize0test']


def run(command):
    subprocess.run([str(value) for value in command], cwd=ROOT, check=True)


def inventory(build):
    # Even --show-only rewrites CTest's LastTest.log. Query copies of the exact
    # generated registrations, leaving existing actual execution logs intact.
    with tempfile.TemporaryDirectory(prefix='libavif-root-inventory-') as directory:
        query = Path(directory)
        for source in build.rglob('CTestTestfile.cmake'):
            destination = query / source.relative_to(build)
            destination.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(source, destination)
        record = json.loads(subprocess.check_output(['ctest', '--test-dir', str(query), '--show-only=json-v1'], text=True))
    names = [test['name'] for test in record['tests']]
    if not names or len(names) != len(set(names)):
        raise SystemExit('Missing or duplicate original CTest registrations')
    return names


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--baseline', type=Path, required=True)
    parser.add_argument('--port', type=Path, required=True)
    parser.add_argument('--root-profile', type=Path, required=True)
    parser.add_argument('--aom-reference-build', type=Path, help='Read an existing matching original AOM archive without rebuilding it')
    parser.add_argument('--tier', choices=['smoke', 'full'], required=True)
    parser.add_argument('--jobs', type=int, default=2)
    args = parser.parse_args()
    if args.jobs < 1:
        parser.error('--jobs must be positive')
    baseline, port = args.baseline.resolve(), args.port.resolve()
    manifest = args.root_profile.resolve()
    record = json.loads(manifest.read_text())
    validate_profile(manifest, record['generation_target'])
    if digest(baseline / 'compile_commands.json') != record['original_compile_commands_sha256']:
        raise SystemExit('The original root reference differs from the generated profile compilation database')
    if digest(baseline / 'CMakeCache.txt') != record['original_cmake_cache_sha256']:
        raise SystemExit('The original root reference configuration differs from the generated profile')
    full = inventory(baseline)
    if full != inventory(port):
        raise SystemExit('The original C/Rust CTest registration inventories differ')
    expected = full if args.tier == 'full' else SMOKE
    if not set(expected).issubset(full):
        raise SystemExit('The original native reference is missing required smoke registrations')
    proof = json.loads((profile_work(record) / 'summary.json').read_text())
    if proof['translation_manifest_sha256'] != digest(manifest):
        raise SystemExit('The isolated native root build proof differs from the current translation')
    for item in proof['artifacts'].values():
        if digest(Path(item['path'])) != item['sha256']:
            raise SystemExit('The isolated native root artifact changed after its build')
    parent = port / 'root-validation'
    parent.mkdir(exist_ok=True)
    output = Path(tempfile.mkdtemp(prefix=args.tier + '-', dir=parent))
    summary = output / 'summary.json'
    feature = [] if record['variant'] == 'default' else ['--features', record['variant'].replace('mini-extended-pixi', 'mini,extended-pixi')]
    run(['python3', 'tools/check_abi.py', '--build', baseline, '--root-profile', manifest, *feature])
    archive = Path(proof['artifacts']['libavif.a']['path'])
    run(['python3', 'tools/check_aom_root_linkage.py', '--archive', archive, '--consumer-build', port,
         '--reference-build', args.aom_reference_build or ROOT / f'build/aom-native-{record["preprocessing_profile"]}',
         *(['--debug'] if record['preprocessing_profile'] == 'debug' else [])])
    executions = {}
    for implementation, build in [('c', baseline), ('rust', port)]:
        executions[implementation] = output / implementation
        run(['python3', 'tools/run_tests.py', build, '--jobs', args.jobs,
             '--evidence-dir', executions[implementation],
             *(['--tests-regex', '^(' + '|'.join(map(re.escape, SMOKE)) + ')$'] if args.tier == 'smoke' else [])])
        actual = results(executions[implementation] / 'results.xml')
        if set(actual) != set(expected):
            raise SystemExit('Actual execution does not cover the selected original CTest inventory')
    run(['python3', 'tools/compare_results.py', '--baseline', executions['c'] / 'results.xml',
         '--port', executions['rust'] / 'results.xml', '--report', output / 'parity.json',
         '--baseline-log', executions['c'] / 'LastTest.log', '--port-log', executions['rust'] / 'LastTest.log'])
    for decoder in ['default', 'aom']:
        run(['python3', 'tools/differential.py', '--baseline', baseline, '--port', port,
             '--output-dir', output / f'bytes-{decoder}', *(['--decoder-codec', decoder] if decoder != 'default' else [])])
    shared = next(item['path'] for name, item in proof['artifacts'].items() if name.endswith(('.so', '.dylib')))
    run(['python3', 'tools/smoke_shared.py', '--library', shared])
    run(['python3', 'tools/verify_upstream.py', '--tracked'])
    snapshots = {manifest: output / 'configuration/translation.json',
        ROOT / (f'build/abi-{record["generation_target"]}-{record["preprocessing_profile"]}-'
                f'{record["variant"]}/summary.json'): output / 'configuration/abi.json'}
    for implementation, build in [('c', baseline), ('rust', port)]:
        for name in ['CMakeCache.txt', 'compile_commands.json']:
            snapshots[build / name] = output / 'configuration' / implementation / name
    snapshots[port / 'aom-root-linkage.json'] = output / 'configuration/linkage.json'
    for source, destination in snapshots.items():
        destination.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(source, destination)
    evidence = [path for path in output.rglob('*') if path.is_file()]
    summary.write_text(json.dumps(dict(target=record['generation_target'], profile=record['preprocessing_profile'],
        variant=record['variant'], tier=args.tier, complete_original_root_suite=args.tier == 'full',
        all_features_acceptance_claimed=False, production_target_admission_claimed=False,
        original_ctest_registrations=len(full), executed_ctest_registrations=len(expected),
        configuration_origins={str(destination.relative_to(output)): str(source.relative_to(ROOT))
                               for source, destination in snapshots.items()},
        artifacts=proof['artifacts'], evidence={str(path.relative_to(ROOT)): digest(path) for path in evidence}), indent=2) + '\n')
    print(f'Native root {args.tier} validation passed; {len(expected)}/{len(full)} original registrations; {summary}')


if __name__ == '__main__':
    main()
