#!/usr/bin/env python3
"""Check the finite role review of the selected direct C++ header bindings."""
import argparse
import hashlib
import json
from pathlib import Path

from aom_inline_bridges import SUBJECTS

ROOT = Path(__file__).resolve().parents[1]
FIXTURES = {
    'av1_get_dx': 'Selects the original directional-prediction input angle.',
    'av1_get_dy': 'Selects the original directional-prediction input angle.',
    'av1_get_interp_filter_kernel': 'Selects input filter coefficients for both the subject and reference.',
    'av1_get_interp_filter_params_with_block_size': 'Constructs the original convolution input parameters.',
    'av1_get_max_eob': 'Sets coefficient-buffer bounds and expected end-of-block state in the dropout fixture.',
    'get_scan': 'Converts fixture coefficients between lexical and scan order around the dropout subject.',
    'get_padded_idx': 'Addresses the input levels buffer in the coefficient-context fixture.',
    'get_txb_bhl': 'Sets input buffer indexing in the coefficient-context fixture.',
    'get_txb_high': 'Sets the original transform dimensions in scan/context fixtures.',
    'get_txb_wide': 'Sets the original transform dimensions in scan/context fixtures.',
    'set_levels': 'Sets the input levels buffer origin in the coefficient-context fixture.',
    'get_conv_params': 'Constructs the original warp-filter input parameters.',
    'get_conv_params_no_round': 'Constructs the original convolution/warp/mask input parameters.',
    'get_conv_params_wiener': 'Constructs the original Wiener-convolution input parameters.',
    'get_wedge_types_lookup': 'Bounds the original input mask enumeration.',
    'is_chroma_reference': 'Constructs the original decoder fixture before displacement validation.',
    'fclamp': 'Limits generated noise-model fixture samples to the original input range.',
}
ORACLES = {
    'av1_get_mv_class': 'Used inside ReferenceBuildNmvComponentCostTable, the C++ expected-cost implementation.',
    'clip_pixel_highbd': 'Clips expected samples inside the C++ reference subpixel filters.',
    'fdct_round_shift': 'Used by the C++ reference forward transform in av1_txfm_test.cc.',
    'get_rect_tx_log_ratio': 'Used by the C++ reference transform implementation in av1_txfm_test.cc.',
    'find_average': 'Used inside the independent C++ Wiener-statistics reference.',
    'find_average_highbd': 'Used inside the independent high-bit-depth C++ Wiener-statistics reference.',
    'clamp64': 'Limits expected affine coordinates in the C++ warp-filter reference.',
}
TIMERS = {'aom_usec_timer_start', 'aom_usec_timer_mark', 'aom_usec_timer_elapsed'}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--check', action='store_true')
    args = parser.parse_args()
    inventory_path = ROOT / 'docs/aom-test-bindings-macos-release.json'
    inventory = json.loads(inventory_path.read_text())
    originals = json.loads((ROOT / 'provenance/aom-source.json').read_text())['files']
    subject_names = {name for _, names in SUBJECTS.values() for name in names}
    known = subject_names | FIXTURES.keys() | ORACLES.keys() | TIMERS | {'clamp'}
    observed, records = set(), []
    for source in inventory['sources']:
        path = source['source']
        content = (ROOT / 'upstream/aom' / path).read_bytes()
        if hashlib.sha256(content).hexdigest() != source['source_sha256'] or originals[path] != source['source_sha256']:
            raise SystemExit(f'Modified original direct-binding input: {path}')
        for binding in source['direct_bindings']:
            if binding['kind'] != 'production header inline; role review required':
                continue
            name = binding['name']
            observed.add(name)
            if name not in known:
                raise SystemExit(f'Unreviewed production header binding: {path}: {name}')
            subjects = SUBJECTS.get(Path(path).name, (None, []))[1]
            if name in subjects:
                role = 'Rust test subject through an optional original-ABI export'
                reason = 'The unchanged original assertions execute the reviewed Rust helper; separate C execution retains the baseline.'
            elif name in TIMERS:
                role, reason = 'native measurement infrastructure', 'Measures original loops; does not compute asserted codec output.'
            elif name in FIXTURES:
                role, reason = 'native input/setup helper', FIXTURES[name]
            elif name in ORACLES:
                role, reason = 'native expected-output helper', ORACLES[name]
            elif name == 'clamp':
                role, reason = 'native input/expected-output helper', 'Bounds generated CDEF/coefficients/wedge inputs or expected warp coordinates; no standalone clamp test is registered.'
            elif name == 'aom_memset16':
                role, reason = 'native input/setup helper', 'Initializes original image or coefficient fixture buffers outside AomMemTest.'
            elif name == 'get_msb':
                role = 'native fixture or macro expansion in the original consumer'
                reason = 'Sets resize fixture dimensions or expands original OD_DIVU_SMALL; separate macro execution binds the complete macro subject to Rust.'
            elif name == 'get_default_scan':
                role, reason = 'native input/setup helper', 'Selects coefficient scan data for the original inverse-transform fixture.'
            elif name == 'round_shift':
                role, reason = 'native expected-output helper', 'Rounds values in the independent C++ inverse-transform reference.'
            else:
                raise SystemExit(f'Unreviewed helper use: {path}: {name}')
            records.append(dict(source=path, source_sha256=source['source_sha256'], **binding,
                                reviewed_role=role, rationale=reason))
    if observed != known:
        raise SystemExit(f'The reviewed direct header inventory changed: {sorted(observed ^ known)}')
    record = dict(scope='All direct production-header inline bindings in the selected 149 C++ inputs',
        target='aarch64-apple-darwin', profile='release', original_test_inputs=inventory['original_test_inputs'],
        inventory_sha256=hashlib.sha256(inventory_path.read_bytes()).hexdigest(),
        distinct_reviewed_header_functions=len(observed), direct_header_role_review_complete=True,
        complete_included_template_and_indirect_role_review=False, execution_coverage_claimed_by_this_review=False,
        cpp_rate_control_boundary='av1/ratectrl_rtc.cc builds the separate aom_av1_rc library, which upstream does not install. It is outside the selected 166-C-unit libaom archive and is retained as an original native consumer in test_aom_rc. No Rust wrapper implementation is claimed.',
        other_native_test_infrastructure=['GoogleTest', 'libyuv', 'WebM I/O', 'original C++ reference/fixture code'],
        bindings=records)
    path = ROOT / 'docs/aom-header-role-review.json'
    text = json.dumps(record, indent=2) + '\n'
    if args.check:
        if path.read_text() != text:
            raise SystemExit('The direct header role review is stale')
    else:
        path.write_text(text)
    print(f'Reviewed {len(records)} direct bindings of {len(observed)} header functions; no execution credit inferred')


if __name__ == '__main__':
    main()
