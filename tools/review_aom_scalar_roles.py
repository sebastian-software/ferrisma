#!/usr/bin/env python3
"""Check the finite scalar-reference review against original registrations."""
import argparse
import hashlib
import json
from pathlib import Path
import re

from build_aom import test_names

ROOT = Path(__file__).resolve().parents[1]
UNREGISTERED = {
    'av1_scale_test.cc', 'cfl_test.cc', 'hiprec_convolve_test_util.cc',
    'selfguided_filter_test.cc', 'av1_k_means_test.cc', 'av1_nn_predict_test.cc',
    'av1_round_shift_array_test.cc', 'comp_mask_pred_test.cc', 'disflow_test.cc',
    'encodetxb_test.cc', 'frame_resize_test.cc', 'horver_correlation_test.cc',
    'model_rd_test.cc', 'pickrst_test.cc', 'reconinter_test.cc', 'sum_squares_test.cc',
}
REVIEW = {
    'convolve_test.cc': 'The unscaled subjects use the original independent C++ subpixel-filter arithmetic and guard checks. The scaled subject uses the isolated C oracle in scaled-convolution.',
    'cnn_test.cc': 'The 15 CNNTest cases retain original fixed expected arrays, output dimensions, tolerances, and worker parameters. The SIMD CNNConvolveTest class has no generic registration.',
    'dr_prediction_test.cc': 'The original C parameter matrices have null subject slots: SaturatedValues copies the reference and OperationCheck returns early. These original passes receive no independent comparison credit. The separate directional-prediction matrix compares C with Rust.',
    'av1_convolve_test.cc': 'All 912 original scalar algorithm comparisons use the convolution, intrabc, convolution-2d, and compound C-oracle bindings. Five additional original cases verify parameter construction and function identity.',
    'av1_fwd_txfm1d_test.cc': 'Accuracy uses the independent C++ floating-point reference transform; cosine data uses native C++ math. The round_shift assertions use the separately generated Rust inline export.',
    'av1_softmax_test.cc': 'Both original cases use the isolated C softmax oracle and retain original tolerances, including the anchor case.',
    'av1_wedge_utils_test.cc': 'Both generic functional cases retain independent C++ residual-blending and sum-of-squares arithmetic. SIMD equivalence classes have no generic registrations.',
    'avg_test.cc': 'Original ReferenceAverage arithmetic supplies independent average expectations. SATD minimum/maximum and random cases retain original constants; all 32 SATD cases additionally keep C in the first parameter slot and Rust in the second through satd. Average timing calls and the redundant expected_c comparison remain Rust-bound and receive no separate independent-reference credit.',
    'blend_a64_mask_1d_test.cc': 'The eight original horizontal/vertical mask comparisons use the isolated two-dimensional C oracle through blend-mask-1d.',
    'blend_a64_mask_test.cc': 'Only the high-bit-depth d16 class is registered generically. Its first parameter binds the isolated C oracle and its second remains Rust through blend-mask-d16.',
    'fft_test.cc': 'Correctness retains original independent C++ Fourier-transform arithmetic. The original active Benchmark cases remain active and are not treated as independent correctness comparisons.',
    'fwht4x4_test.cc': 'The original forward/inverse transform and coefficient checks retain native C++ reference arithmetic and original round-trip/error assertions, including the original dynamic-skip behavior.',
    'hadamard_test.cc': 'The original C++ reference Hadamard calculations supply expected coefficients; generic scalar parameter functions resolve to Rust.',
    'minmax_test.cc': 'Native C++ minimum/maximum loops supply expected values independently of the Rust parameter functions.',
    'sad_test.cc': 'Original ReferenceSAD, ReferenceSADSkip, ReferenceSADavg, and multidestination loops compute expected absolute differences in C++; scalar parameter functions resolve to Rust.',
    'subtract_test.cc': 'Original CheckResult compares the Rust output with direct C++ source-minus-predictor arithmetic. Original speed cases remain disabled.',
    'variance_test.cc': 'Most subjects retain independent C++ variance/subpixel/OBMC arithmetic or known sums. All 40 MSE and quad/dual sum comparisons use the five isolated C oracles in variance-mse-sum. AVX2-only variance-statistic classes are outside the generic registration set.',
    'warp_filter_test.cc': 'The 80 original affine-warp cases retain their separate original helper object and use the isolated C reference through warp-affine.',
    'warp_filter_test_util.cc': 'This helper defines the original affine-warp comparison body registered by warp_filter_test.cc. The low-bit-depth oracle is isolated C; the high-bit-depth helper has no generic registration.',
    'wiener_test.cc': 'The generic parameter is the original independent optimized C++ statistics helper. The production av1_compute_stats functions resolve to Rust on the other side of the assertions. Original tested window sizes use C++ arithmetic; native expected-value helpers remain native. SearchWienerTest retains its original regression cases.',
}


def digest(value):
    return hashlib.sha256(json.dumps(value, sort_keys=True).encode()).hexdigest()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--reference-build', type=Path, default=ROOT / 'build/aom-native-release')
    parser.add_argument('--check', action='store_true')
    args = parser.parse_args()
    reference = args.reference_build.resolve()
    profile = json.loads((reference / 'reference-profile.json').read_text())
    if profile['profile'] != 'release':
        raise SystemExit('The finite scalar registration review requires the original Release inventory')
    names = test_names((reference / 'test_libaom-inventory.txt').read_text())
    if len(names) != 18729:
        raise SystemExit('The original generic registration boundary changed')
    inventory = json.loads((ROOT / 'tests/ports/fixtures/aom-test-bindings-macos-release.json').read_text())
    originals = json.loads((ROOT / 'provenance/aom-source.json').read_text())['files']
    reviewed, records = set(), []
    for source in inventory['sources']:
        path = source['source']
        bindings = [b for b in source['direct_bindings']
                    if b['kind'] == 'external original codec global' and b['name'].endswith('_c')]
        if not bindings or path in reviewed:
            continue
        reviewed.add(path)
        short = Path(path).name
        if short not in UNREGISTERED and short not in REVIEW:
            raise SystemExit(f'Unreviewed scalar-reference input: {path}')
        content = (ROOT / 'upstream/aom' / path).read_bytes()
        if hashlib.sha256(content).hexdigest() != source['source_sha256'] or originals[path] != source['source_sha256']:
            raise SystemExit(f'Modified original scalar-reference input: {path}')
        classes = set(re.findall(r'\bTEST(?:_F|_P)?\(\s*(\w+)\s*,', content.decode('utf-8-sig')))
        selected = sorted(n for n in names if n.split('.')[0].split('/')[-1] in classes)
        disabled = [n for n in selected if any(part.startswith('DISABLED_') for part in re.split(r'[./]', n))]
        if short in UNREGISTERED and selected:
            raise SystemExit(f'A previously unregistered scalar family now has original cases: {path}')
        rationale = REVIEW.get(short, 'Original SIMD-only parameter classes have no generic registrations. Any supplementary scalar execution is recorded separately and receives zero original-case credit.')
        records.append(dict(source=path, source_sha256=source['source_sha256'],
            direct_scalar_bindings_sha256=digest(bindings), scalar_globals_in_direct_inventory=len(bindings),
            registered_test_classes=sorted(set(n.split('.')[0] for n in selected)),
            original_registrations=len(selected), original_active_registrations=len(selected) - len(disabled),
            original_disabled_registrations=len(disabled), original_case_names_sha256=digest(selected),
            reviewed_assertion_strategy=rationale))
    if {Path(p).name for p in reviewed} != UNREGISTERED | REVIEW.keys():
        raise SystemExit('The finite scalar-reference source inventory changed')
    report = dict(scope='All selected C++ inputs with a scalar _c global in the direct compiler inventory; physical template bodies reviewed in the original source',
        original_source_revision='af3dc9aadc793c1b00edc003e3fc19a44fdeb574',
        original_registrations=len(names), original_case_names_sha256=digest(sorted(names)),
        distinct_original_inputs=len(records), zero_registration_inputs=len(UNREGISTERED),
        original_source_and_assertions_unchanged=True, static_review_assigns_no_execution_credit=True,
        complete_runtime_or_indirect_coverage_claimed=False,
        registration_counts_exclude_helper_cases_registered_by_other_inputs=True,
        native_cpp_reference_fixture_and_timing_helpers_retained=True, sources=records)
    path = ROOT / 'tests/ports/fixtures/aom-scalar-role-review.json'
    text = json.dumps(report, indent=2) + '\n'
    if args.check:
        if path.read_text() != text:
            raise SystemExit('The finite scalar-reference review is stale')
    else:
        path.write_text(text)
    print(f'Reviewed {len(records)} scalar-reference inputs against all {len(names)} original registrations; no pass counts inferred')


if __name__ == '__main__':
    main()
