#!/usr/bin/env python3
"""Record reviewed static helper bindings without inferring execution coverage."""
import argparse
import hashlib
import json
from pathlib import Path

from aom_inline_bridges import SUBJECTS
from review_aom_header_roles import FIXTURES, ORACLES, TIMERS

ROOT = Path(__file__).resolve().parents[1]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--inventory', type=Path,
                        default=ROOT / 'build/aom-test-bindings-release/inventory-test-headers.json')
    args = parser.parse_args()
    inventory = json.loads(args.inventory.read_text())
    original = json.loads((ROOT / 'provenance/aom-source.json').read_text())['files']
    if inventory['original_test_inputs'] != 149 or inventory['profile'] != 'release' or inventory['target'] != 'macos':
        raise SystemExit('Unexpected extended original binding inventory')
    for name, expected in inventory['verified_original_inputs'].items():
        if original[name] != expected or hashlib.sha256((ROOT / 'upstream/aom' / name).read_bytes()).hexdigest() != expected:
            raise SystemExit(f'Modified original correspondence input: {name}')
    subjects = {name for _, names in SUBJECTS.values() for name in names}
    known = subjects | FIXTURES.keys() | ORACLES.keys() | TIMERS | {'clamp', 'av1_use_intra_edge_upsample'}
    functions = {}
    for source in inventory['sources']:
        for binding in source['direct_bindings']:
            if binding['kind'] != 'production header inline; role review required':
                continue
            name = binding['name']
            if name not in known:
                raise SystemExit(f'Unreviewed included header function: {name}')
            record = functions.setdefault(name, dict(declaration=binding['declaration'],
                declaration_line=binding['declaration_line'], translated_name_present=binding['translated_name_present'], sites={}))
            for site in binding['reference_sites']:
                caller = site['source']
                if name in SUBJECTS.get(Path(caller).name, (None, []))[1]:
                    role = 'reviewed Rust subject in the separate original-assertion bridge execution'
                elif name in TIMERS:
                    role = 'native measurement infrastructure'
                elif name in ORACLES:
                    role = 'native expected-output helper'
                else:
                    role = 'native fixture/reference helper'
                record['sites'][(caller, site['line'])] = dict(**site, reviewed_role=role)
    if set(functions) != known:
        raise SystemExit('The finite extended header function inventory changed')
    for record in functions.values():
        record['sites'] = [record['sites'][key] for key in sorted(record['sites'])]
    report = dict(scope=inventory['scope'], original_test_compilation_inputs=149,
        verified_original_inputs=inventory['verified_original_inputs'],
        inventory_sha256=hashlib.sha256(args.inventory.read_bytes()).hexdigest(),
        config_sha256=inventory['config_sha256'], clang_version=inventory['clang_version'],
        target='aarch64-apple-darwin', profile='release', distinct_header_functions=len(functions),
        includes_original_cpp_helper_bodies_and_dependent_lookup_candidates=True,
        additional_function='av1_use_intra_edge_upsample',
        additional_function_role='Selects input upsampling in DrPredTest::SetParams; retain its original native fixture behavior.',
        complete_runtime_or_indirect_call_coverage_proven=False,
        source_and_static_candidates_receive_no_execution_credit=True, functions=functions)
    (ROOT / 'docs/aom-included-header-role-review.json').write_text(json.dumps(report, indent=2) + '\n')
    print(f'Reviewed {len(functions)} static header functions across all 149 original C++ compilation inputs')


if __name__ == '__main__':
    main()
