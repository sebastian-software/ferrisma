#!/usr/bin/env python3
"""Record completed native dav1d comparisons and local libavif integration."""
import argparse
from collections import Counter
from datetime import datetime, timezone
import hashlib
import json
from pathlib import Path
import subprocess

from record_sharpyuv_validation import inventory

ROOT = Path(__file__).resolve().parents[1]


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def read(path):
    return json.loads(path.read_text())


def successful_run(path, jobs):
    value = read(path)
    if value['status'] != 'completed' or value['conclusion'] != 'success' or len(value['jobs']) != jobs:
        raise SystemExit(f'Missing completed successful CI evidence: {path}')
    if any(job['conclusion'] != 'success' for job in value['jobs']):
        raise SystemExit(f'Unsuccessful CI job: {path}')
    return {key: value[key] for key in ['url', 'headSha', 'conclusion']} | {
        'jobs': [{key: job[key] for key in ['name', 'url', 'conclusion']} for job in value['jobs']]}


def command_identity(command):
    # Original consumer paths differ; retain the executable's relative path and
    # every original argument, including duplicate test names with different args.
    parts = Path(command[0]).parts
    index = next(i for i, part in enumerate(parts) if part.startswith(('dav1d-reference-', 'dav1d-rust-')))
    return ('/'.join(parts[index + 1:]), *command[1:])


def corpus(artifact, profile):
    reference = artifact / f'dav1d-reference-{profile}'
    candidate = artifact / f'dav1d-rust-{profile}'
    definitions = read(candidate / 'original-tests.json')
    rows = read(candidate / 'original-results.json')
    original = [json.loads(line) for line in (reference / 'meson-logs/testlog.json').read_text().splitlines()]
    if len(rows) != 1711 or len(definitions) != len(rows) or len(original) != len(rows):
        raise SystemExit(f'Incomplete original inventory: {artifact.name}/{profile}')
    if any(row['result'] != 'OK' or row['returncode'] != 0 for row in rows + original):
        raise SystemExit('Failing original corpus test')
    expected = Counter((test['name'], command_identity(test['cmd'])) for test in definitions)
    actual = Counter((row['name'], command_identity(row['command'])) for row in rows)
    baseline = Counter((row['name'].split(' / ', 1)[1], command_identity(row['command'])) for row in original)
    if expected != actual or expected != baseline:
        raise SystemExit('C/Rust case names or original command arguments differ')
    multiplier = 4 if profile == 'debug' else 1
    for test, row in zip(definitions, rows):
        if (row['timeout_multiplier'] != multiplier or row['original_timeout'] != test['timeout']
                or row['timeout'] != test['timeout'] * multiplier):
            raise SystemExit('Unrecorded change to original test time budgets')
    suites = Counter('+'.join(test['suite']) for test in definitions)
    return {'original_registrations_each': len(rows), 'matching_case_and_command_inventory': True,
            'rust_timeout_multiplier': multiplier, 'suites': dict(sorted(suites.items())),
            'original_definitions_sha256': digest(candidate / 'original-tests.json'),
            'rust_results_sha256': digest(candidate / 'original-results.json'),
            'c_results_sha256': digest(reference / 'meson-logs/testlog.json'),
            'c_junit_sha256': digest(reference / 'meson-logs/testlog.junit.xml')}


def api(path, cases, sanitizer=None):
    value = read(path)
    if value['cases'] != cases or len(value['results']) != cases or value['differences']:
        raise SystemExit(f'Incomplete public API parity: {path}')
    if value['sanitizer'] != sanitizer or value['driver_instrumented'] != bool(sanitizer):
        raise SystemExit(f'Unexpected caller instrumentation: {path}')
    if value['driver_sha256'] != digest(ROOT / 'tests/ports/dav1d_api.c'):
        raise SystemExit('The public API report used a different caller source')
    for row in value['results']:
        if isinstance(row['result'], dict) and row['result']['live'] != [0, 0, 0]:
            raise SystemExit('Packet, user-data, or picture resources remain live')
    return {key: value[key] for key in ['cases', 'differences', 'sanitizer', 'driver_instrumented',
                                      'driver_sha256', 'reference_sha256', 'candidate_archive_sha256']} | {
        'summary_sha256': digest(path), 'tracked_resources_live_after_close': 0}


def linkage(path):
    value = read(path)
    if (value['rust_codegen_objects'] <= 0 or value['native_decoder_objects']
            or value['unresolved_decoder_symbols']):
        raise SystemExit(f'Native decoder fallback or incomplete symbols: {path}')
    return value | {'summary_sha256': digest(path)}


def faults(path, sanitizer=None):
    value = read(path)
    if (value['paired_scenarios'] != 2048 or len(value['records']) != 2048
            or value['header_variants'] != 512 or value['sanitizer'] != sanitizer
            or value['caller_instrumented'] != bool(sanitizer) or value['unwrapped_decoder_objects']):
        raise SystemExit(f'Incomplete original fault-fuzzer evidence: {path}')
    expected = ['malloc', 'posix_memalign', 'pthread_create', 'pthread_cond_init', 'pthread_mutex_init']
    if value['hooks'] != expected or value['observer_sha256'] != digest(ROOT / 'tools/dav1d-fault-runtime.c'):
        raise SystemExit('Different fault hooks or observer source')
    for name, sha in value['original_callers_sha256'].items():
        if sha != digest(ROOT / 'upstream/dav1d/tests/libfuzzer' / name):
            raise SystemExit('Original fault-fuzzer source differs')
    for implementation in ['c', 'rust']:
        for key in ['calls', 'failures']:
            if len(value['totals'][implementation][key]) != 5 or any(count <= 0 for count in value['totals'][implementation][key]):
                raise SystemExit('An original failure class was not exercised')
    return {key: value[key] for key in ['profile', 'sanitizer', 'caller_instrumented', 'paired_scenarios',
        'header_variants', 'hooks', 'totals', 'original_callers_sha256', 'observer_sha256',
        'candidate_archive_sha256', 'rewritten_rust_codegen_objects', 'unwrapped_decoder_objects']} | {
            'summary_sha256': digest(path), 'oracle': 'unchanged original fuzzer exit/crash behavior',
            'hook_event_count_equality_required': False}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--ci-run', type=Path, required=True)
    parser.add_argument('--ci-artifacts', type=Path, required=True)
    parser.add_argument('--translation-run', type=Path, required=True,
                        help='Completed Linux generation workflow in verify mode')
    parser.add_argument('--translation-artifacts', type=Path, required=True,
                        help='Downloaded native Linux generation artifact, including static fault-fuzzer reports')
    args = parser.parse_args()
    native_ci = successful_run(args.ci_run, 2)
    translation_ci = successful_run(args.translation_run, 1)
    subprocess.run(['python3', ROOT / 'tools/check_translation.py'], check=True)
    subprocess.run(['python3', ROOT / 'tools/generate_licenses.py', '--check'], check=True)
    source = read(ROOT / 'provenance/dav1d-source.json')
    record = {'date': datetime.now(timezone.utc).date().isoformat(), 'component': 'dav1d 1.5.4',
              'upstream_revision': source['revision'], 'source_files_verified': len(source['files']),
              'source_manifest_sha256': digest(ROOT / 'provenance/dav1d-source.json'),
              'test_input_manifest_sha256': digest(ROOT / 'provenance/dav1d-test-inputs.json'),
              'translation_manifests_sha256': {}, 'explicit_object_alignments_retained': {}, 'native_ci': native_ci,
              'linux_translation_verification': translation_ci, 'native_targets': {},
              'local_integration': {'platform': 'macOS ARM64',
                                    'head_sha': subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=ROOT, text=True).strip(),
                                    'rustc': subprocess.check_output(['rustc', '--version'], text=True).strip()}}
    for path in sorted((ROOT / 'crates/dav1d').glob('TRANSLATION*.json')):
        relative = str(path.relative_to(ROOT))
        committed = subprocess.check_output(['git', 'show', f"{native_ci['headSha']}:{relative}"], cwd=ROOT)
        if hashlib.sha256(committed).hexdigest() != digest(path):
            raise SystemExit('Native CI did not test the current translated implementation')
        if 'linux' in path.name:
            regenerated = subprocess.check_output(['git', 'show', f"{translation_ci['headSha']}:{relative}"], cwd=ROOT)
            if hashlib.sha256(regenerated).hexdigest() != digest(path):
                raise SystemExit('Translation verification did not check the current Linux implementation')
        record['translation_manifests_sha256'][relative] = digest(path)
        record['explicit_object_alignments_retained'][relative] = sum(
            unit['corrections'].get('original-c-object-alignment', 0) for unit in read(path)['units'])
    expected_artifacts = ['dav1d-macos-15-arm64', 'dav1d-ubuntu-24.04-x86_64']
    for name in expected_artifacts:
        artifact = args.ci_artifacts / name
        abi_path = artifact / 'dav1d-abi/summary.json'
        abi = read(abi_path)
        if abi['differences'] or abi['checks'] < 10000 or abi['record_instances'] < 1000:
            raise SystemExit(f'Incomplete layout evidence: {name}')
        target = {'abi': {key: abi[key] for key in ['record_instances', 'checks', 'differences']} | {
            'excluded_entries': len(abi['excluded']), 'summary_sha256': digest(abi_path)}, 'profiles': {}}
        for profile in ['debug', 'release']:
            reference = read(artifact / f'dav1d-reference-{profile}/reference-profile.json')
            if (not reference['acceptance_reference'] or reference['c_flags'] != '-ffp-contract=off -fvisibility=default'
                    or reference['library_translation_units'] != 51 or reference['bitdepth_instantiations'] != [8, 16]
                    or reference['assembly'] or reference['trim_dsp'] or not reference['logging']):
                raise SystemExit('Unexpected decoder reference configuration')
            dsp_path = artifact / f'dav1d-dsp-{profile}/inventory.json'
            dsp = read(dsp_path)
            if (dsp['source_groups'] != 17 or len(dsp['original_test_instantiations']) != 17
                    or dsp['function_selections_each'] != 3155 or dsp['unsupported_selections_each'] != 1923):
                raise SystemExit('Incomplete original DSP inventory')
            target['profiles'][profile] = {'reference': reference, 'corpus': corpus(artifact, profile),
                'api': api(artifact / f'dav1d-api-{profile}/summary.json', 82 if profile == 'debug' else 83),
                'dsp': {key: dsp[key] for key in ['source_groups', 'function_selections_each', 'unsupported_selections_each', 'seed']} | {
                    'inventory_sha256': digest(dsp_path), 'results_sha256': digest(artifact / f'dav1d-dsp-{profile}/results.log')},
                'linkage': linkage(artifact / f'dav1d-rust-{profile}/linkage-audit.json')}
        record['native_targets'][name] = target
    for label, baseline, candidate in [
        ('default', 'baseline', 'port'), ('debug', 'baseline', 'port-debug'),
        ('asan', 'baseline-asan-llvm16', 'port-asan-llvm16'),
        ('all_features', 'baseline-mini-extended-pixi-libgav1-rav1e-svt-libyuv-avm',
         'port-mini-extended-pixi-libgav1-rav1e-svt-libyuv-avm')]:
        record['local_integration'][label] = inventory(ROOT / 'build' / f'{baseline}-results.xml',
                                                      ROOT / 'build' / f'{candidate}-results.xml')
    record['local_integration']['decoder_linkage'] = linkage(ROOT / 'build/port/dav1d-linkage-audit.json')
    initial_path = ROOT / 'build/dav1d-rust-debug-original-limits.json'
    initial = read(initial_path)
    timeouts = [row for row in initial if row['result'] == 'TIMEOUT']
    if len(initial) != 1711 or len(timeouts) != 3 or any(row['result'] not in ['OK', 'TIMEOUT'] for row in initial):
        raise SystemExit('Unexpected original Debug time-budget diagnostic')
    record['debug_original_time_budget_diagnostic'] = {
        'platform': 'macOS ARM64', 'passing': len(initial) - len(timeouts),
        'timeouts': [{key: row[key] for key in ['index', 'name', 'timeout']} for row in timeouts],
        'summary_sha256': digest(initial_path), 'performance_parity_claimed': False}
    record['public_api_asan'] = api(ROOT / 'build/dav1d-api-release-asan/summary.json', 83, 'address') | {
        'platform': 'macOS ARM64', 'c_compiler': 'LLVM 16.0.6', 'runtime': 'matching Rust nightly ASan v8'}
    record['original_static_fault_fuzzer'] = {'local_macos_arm64': {
        'release': faults(ROOT / 'build/dav1d-faults-release/summary.json'),
        'debug': faults(ROOT / 'build/dav1d-faults-debug/summary.json'),
        'asan': faults(ROOT / 'build/dav1d-faults-release-asan/summary.json', 'address')},
        'native_linux_x86_64': {profile: faults(args.translation_artifacts / 'build' /
                f'dav1d-faults-{profile}/summary.json') for profile in ['release', 'debug']}}
    differential = ROOT / 'build/differential/summary.json'
    scenarios = read(differential)
    if len(scenarios) != 10 or not all(row['png_byte_equal'] for row in scenarios):
        raise SystemExit('Strict downstream image parity not established')
    record['local_integration']['differential'] = {'exact_avif_scenarios': 10, 'exact_png_scenarios': 10,
                                                  'summary_sha256': digest(differential)}
    record['remaining_scope'] = ['libaom and optional native engine ports', 'root libavif on Linux',
        'additional targets and SIMD', 'safe Rust API and optimization', 'continuous fuzzing and external Argon corpus',
        'complete leak, race, and undefined-behavior sanitizer coverage', 'crate registry publication']
    (ROOT / 'docs/dav1d-validation-results.json').write_text(json.dumps(record, indent=2) + '\n')
    print('Recorded four native decoder profiles, original test inventories, and downstream integration')


if __name__ == '__main__':
    main()
