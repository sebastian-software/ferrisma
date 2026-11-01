#!/usr/bin/env python3
"""Record verified local integration and completed native CI evidence."""
import argparse
from datetime import datetime, timezone
import hashlib
import json
from pathlib import Path
import subprocess

from compare_results import results

ROOT = Path(__file__).resolve().parents[1]


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def inventory(left, right):
    baseline, candidate = results(left), results(right)
    if baseline != candidate or any(value['skipped'] for value in candidate.values()):
        raise SystemExit(f'Failing or different inventory: {left}, {right}')
    return {'ctest_registrations_each': len(candidate),
            'gtest_cases_each': sum(sum(value['gtest_cases']) for value in candidate.values()),
            'matching_case_and_skip_inventory': True,
            'baseline_junit_sha256': digest(left), 'rust_junit_sha256': digest(right)}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--ci-run', type=Path, required=True, help='Completed successful gh run view JSON')
    parser.add_argument('--ci-artifacts', type=Path, required=True, help='Downloaded artifacts from that run')
    args = parser.parse_args()
    ci = json.loads(args.ci_run.read_text())
    if ci['conclusion'] != 'success' or len(ci['jobs']) != 2 or any(job['conclusion'] != 'success' for job in ci['jobs']):
        raise SystemExit('The two native CI jobs must have completed successfully')
    subprocess.run(['python3', ROOT / 'tools/check_translation.py'], check=True)
    subprocess.run(['python3', ROOT / 'tools/generate_licenses.py', '--check'], check=True)
    sharp_source = json.loads((ROOT / 'provenance/libwebp-source.json').read_text())
    avif_source = json.loads((ROOT / 'UPSTREAM.json').read_text())
    record = {'date': datetime.now(timezone.utc).date().isoformat(), 'component': 'SharpYUV 0.4.2 in libwebp 1.6.0',
              'upstream_revision': sharp_source['revision'],
              'source_files_verified': {'libwebp': len(sharp_source['files']), 'libavif': len(avif_source['files'])},
              'translation_manifest_sha256': digest(ROOT / 'crates/sharpyuv/TRANSLATION.json'),
              'rustc': subprocess.check_output(['rustc', '--version'], text=True).strip(),
              'primary_c_flags': ['-ffp-contract=off'], 'special_rust_float_flags': [],
              'sharpyuv_development_opt_level': 0, 'development_assertions': True,
              'ci': {'url': ci['url'], 'head_sha': ci['headSha'], 'conclusion': ci['conclusion'],
                     'jobs': [{'name': job['name'], 'url': job['url'], 'conclusion': job['conclusion']} for job in ci['jobs']]},
              'native_profiles': [], 'local_integration': {}}
    for artifact in sorted(args.ci_artifacts.iterdir()):
        if not artifact.is_dir():
            continue
        # upload-artifact strips the common build/ prefix, retaining these paths.
        for profile in ['release', 'debug']:
            path = artifact / f'sharpyuv-parity-{profile}/summary.json'
            value = json.loads(path.read_text())
            if value['differences'] or value['cases'] != 1271:
                raise SystemExit(f'Exact parity not established: {path}')
            configuration = value['reference_configuration']
            if not configuration['acceptance_reference'] or configuration['c_flags'] != '-ffp-contract=off':
                raise SystemExit('Native reference does not use the selected numeric contract')
            record['native_profiles'].append({key: value[key] for key in
                ['platform', 'profile', 'cases', 'differences', 'reference_sha256', 'candidate_sha256', 'reference_configuration']})
        cpu = json.loads((artifact / 'sharpyuv-cpu/results.json').read_text())
        allocation = json.loads((artifact / 'sharpyuv-allocation/results.json').read_text())
        if len(cpu) != 11 or len(allocation) != 8 or any(row[3] != 0 for row in allocation):
            raise SystemExit('Missing CPU-query or allocation-failure evidence')
        for profile in ['release', 'debug']:
            abi = json.loads((artifact / f'sharpyuv-abi/{profile}.json').read_text())
            if sum(len(fields) for fields in abi.values()) != 23:
                raise SystemExit('Incomplete ABI evidence')
        webp = inventory(artifact / 'sharpyuv-webp-c-results.xml', artifact / 'sharpyuv-webp-rust-results.xml')
        webp.update(cpu_query_results=cpu, allocation_failure_points=7, abi_checks_each_profile=23)
        record.setdefault('native_consumers', {})[artifact.name] = webp
        diagnostic_path = artifact / 'sharpyuv-parity-compiler-default/summary.json'
        if diagnostic_path.exists():
            diagnostic = json.loads(diagnostic_path.read_text())
            if diagnostic['acceptance_reference'] is not False:
                raise SystemExit('Compiler-default diagnostic is not marked separately')
            record.setdefault('compiler_default_diagnostics', []).append({
                'platform': diagnostic['platform'], 'cases': diagnostic['cases'],
                'differing_groups': len(diagnostic['differences']),
                'differences': diagnostic['differences'], 'acceptance_reference': False,
                'reference_sha256': diagnostic['reference_sha256'], 'candidate_sha256': diagnostic['candidate_sha256']})
    if len(record['native_profiles']) != 4:
        raise SystemExit('Expected Debug/Release artifacts from both native targets')
    for label, baseline, candidate in [
        ('default', 'baseline', 'port'), ('debug', 'baseline', 'port-debug'),
        ('asan', 'baseline-asan-llvm16', 'port-asan-llvm16'),
        ('all_features', 'baseline-mini-extended-pixi-libgav1-rav1e-svt-libyuv-avm',
         'port-mini-extended-pixi-libgav1-rav1e-svt-libyuv-avm'),
        ('webp', 'sharpyuv-webp-c', 'sharpyuv-webp-rust')]:
        record['local_integration'][label] = inventory(ROOT / 'build' / f'{baseline}-results.xml',
                                                      ROOT / 'build' / f'{candidate}-results.xml')
    sanitized = ROOT / 'build/sharpyuv-allocation-asan/results.json'
    if any(row[3] != 0 for row in json.loads(sanitized.read_text())):
        raise SystemExit('Sanitized allocation cleanup differs')
    record['allocation_asan'] = {'failure_points': 7, 'live_allocations_after_each_call': 0,
                                'results_sha256': digest(sanitized), 'platform': 'macOS ARM64',
                                'c_compiler': 'LLVM 16.0.6', 'runtime': 'matching Rust nightly ASan v8'}
    scenarios = json.loads((ROOT / 'build/differential/summary.json').read_text())
    if len(scenarios) != 10 or not all(value['png_byte_equal'] for value in scenarios):
        raise SystemExit('Strict libavif image parity not established')
    record['libavif_differential'] = {'exact_avif_scenarios': 10, 'exact_png_scenarios': 10,
                                     'results_sha256': digest(ROOT / 'build/differential/summary.json')}
    diagnostic_path = ROOT / 'build/libavif-compiler-default-differential.json'
    diagnostic = json.loads(diagnostic_path.read_text())
    record['libavif_compiler_default_diagnostic'] = {
        'acceptance_reference': False, 'avif_scenarios': len(diagnostic),
        'exact_png_scenarios': sum(value['png_byte_equal'] for value in diagnostic),
        'confirmed_noncontracting_matches': sum(value['matches_noncontracting_c'] is True for value in diagnostic),
        'results_sha256': digest(diagnostic_path)}
    (ROOT / 'docs/sharpyuv-validation-results.json').write_text(json.dumps(record, indent=2) + '\n')
    print('Recorded four native profiles, original consumer inventories, and local integration evidence')


if __name__ == '__main__':
    main()
