#!/usr/bin/env python3
"""Reproduce the x86 kernel files that tools/x86_c2r/c2r.py generated.

Usage: python3 tools/x86_c2r/assemble.py [--out DIR] [--check]

Each file is the printer output for one original C file or header (configs/)
plus the hand-written parts in parts/: file headers with the translation
notes, the Rust macros for C statement macros, and the hand translations that
existed before (parts/*.hand.rs, the files as they were on main before the
printer was used). Without --out the files are written into
crates/aom/src/simd; --check writes them into a scratch directory and fails
unless every one equals the committed file.

Families: the 8-bit inverse transform, the loop filters and the 8-bit intra
predictors (port ADR 0058, amendment of 2026-11-14).
"""
import argparse, os, re, subprocess, sys, tempfile
from pathlib import Path
HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
import c2r  # noqa: E402
ROOT = c2r.ROOT
SIMD = ROOT / 'crates/aom/src/simd'
U = str(ROOT / 'upstream/aom') + '/'
OUT = None
WORK = None


LIC = lambda year: f'''/*
 * Copyright (c) {year}, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */
'''
NOTES = '''//
// Translation notes:
// - Every function carries `{feat}`, the flag upstream compiles the file with.
//   The RTCD specializations are exported under their upstream names.
// - Pointer and array parameters stay raw pointers; local arrays are Rust
//   arrays and start zeroed. DECLARE_ALIGNED arrays keep their alignment
//   through `Aligned16`/`Aligned32`.
// - C integer conversions are explicit (`as`); `assert` becomes
//   `debug_assert!`. Immediate operands become const generics; shift counts
//   that are run-time `int` values use the count-register intrinsics, which is
//   what GCC and Clang emit for a non-constant count.
// - Scalar accesses through cast pointers (`*(int *)p`) are unaligned reads
//   and writes, as x86 compilers emit them.
#![allow(non_snake_case, non_upper_case_globals, unused_assignments, unused_mut, unused_parens)]
#![allow(unused_variables, dead_code, clippy::all)]

use core::arch::x86_64::*;
'''
PRINTER = ('transliterated with a typing-aware C-subset printer (not C2Rust) and\n'
           '// reviewed against the C function by function: the same sequence of intrinsic\n'
           '// and helper calls, constants and operators in order (ADR 0058)')
ALIGN = '''
#[repr(C, align(16))]
struct Aligned16<T>(T);
#[repr(C, align(32))]
struct Aligned32<T>(T);
'''
MACROS = '''
// From aom_dsp/x86/lpf_common_sse2.h: `memcpy` of the low or high 8 bytes of a
// vector.
macro_rules! mm_storelu {
    ($dst:expr, $v:expr) => {
        core::ptr::copy_nonoverlapping((&raw const $v).cast::<u8>(), ($dst).cast::<u8>(), 8)
    };
}
pub(crate) use mm_storelu;
macro_rules! mm_storehu {
    ($dst:expr, $v:expr) => {
        core::ptr::copy_nonoverlapping((&raw const $v).cast::<u8>().add(8), ($dst).cast::<u8>(), 8)
    };
}
pub(crate) use mm_storehu;
'''

def run(cfg):
    return c2r.translate(HERE / 'configs' / cfg, WORK)


def write(rel, text):
    path = OUT / rel
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text)
    # The port's crates are outside the workspace's rustfmt (rustfmt.toml ignores
    # codecs/libavif-rs); the printed files are formatted with the stable
    # rustfmt and its default configuration.
    empty = WORK / 'rustfmt.toml'
    empty.write_text('')
    # X86_C2R_RUSTFMT_TOOLCHAIN pins the toolchain in CI (the formatting
    # of the committed files is that of rustfmt 1.9.0, Rust 1.98.1).
    toolchain = os.environ.get('X86_C2R_RUSTFMT_TOOLCHAIN', 'stable')
    r = subprocess.run(['rustup', 'run', toolchain, 'rustfmt', '--edition', '2021', '--config-path', str(empty),
                        str(path)], capture_output=True, text=True)
    if r.returncode:
        raise SystemExit(f'rustfmt failed on {path}: {r.stderr}')


def part(name):
    return (HERE / 'parts' / name).read_text()


def inverse_transform():
    sse2 = run('av1_txfm_sse2_h.json')
    sse2 = '\n'.join(l for l in sse2.split('\n') if l not in ('// vertical and horizontal', '// __cplusplus'))
    write('av1/common/x86/av1_txfm_sse2.rs', part('txfm_sse2_head.rs')
          + 'use crate::simd::aom_dsp__x86__txfm_common_sse2::pair_set_epi16;\n\n// From av1/common/av1_txfm.h.\n'
          'const NewSqrt2Bits: i32 = 12;\n// 2^12 * sqrt(2)\nconst NewSqrt2: i32 = 5793;\n\n' + sse2)
    hand = part('av1_inv_txfm_ssse3.hand.rs').split('\n', 27)[27]
    hand = re.sub(r'^static tx_size_wide:', 'pub static tx_size_wide:', hand, flags=re.M)
    hand = re.sub(r'^static tx_size_high:', 'pub static tx_size_high:', hand, flags=re.M)
    hand = re.sub(r'^fn AOMMIN', 'pub fn AOMMIN', hand, flags=re.M)
    write('av1/common/x86/av1_inv_txfm_ssse3.rs', part('ssse3_head.rs') + hand + part('ssse3_mid.rs')
          + run('av1_inv_txfm_ssse3.json'))
    h = run('av1_inv_txfm_avx2_h.json')
    h = '\n'.join(l for l in h.split('\n') if l not in ('// half input is zero', '// AOM_AV1_COMMON_X86_AV1_INV_TXFM_AVX2_H_'))
    write('av1/common/x86/av1_inv_txfm_avx2.rs', part('avx2_head.rs') + h
          + '\n// ---------------------------------------------------------------------------\n'
          '// av1/common/x86/av1_inv_txfm_avx2.c\n\n' + run('av1_inv_txfm_avx2.json'))


def loop_filter():
    # lpf_common_sse2.rs: the existing hand translation plus the rest of the header.
    orig = part('lpf_common_sse2.hand.rs')
    head_end = orig.index('use core::arch::x86_64::*;')
    body = orig[orig.index('\n', head_end) + 1:]
    lpfh = run('lpf_common_sse2_h.json')
    write('aom_dsp/x86/lpf_common_sse2.rs', LIC(2017) + '''
// Translated from upstream/aom/aom_dsp/x86/lpf_common_sse2.h (ADR 0056, ADR
// 0058). The high-bitdepth transposes that the directional predictors of
// aom_dsp/x86/intrapred_avx2.c call (highbd_transpose4x8_8x4{,_low,_high}_sse2
// and highbd_transpose8x8{,_low,_high}_sse2) were translated by hand first;
// the other inline functions, which the loop filters use, were
// ''' + PRINTER + '''. mm_storelu and mm_storehu
// become macro_rules with the same `memcpy`. Every helper carries `sse2`. The
// `__m128i *` parameters stay raw pointers, so the callers' `&x[i]` arguments
// keep the C aliasing.
#![allow(non_snake_case, non_upper_case_globals, unused_assignments, unused_mut, unused_parens)]
#![allow(unused_variables, dead_code, clippy::all)]

use core::arch::x86_64::*;
''' + MACROS + body.rstrip() + '\n\n' + lpfh)

    USE_LPF = 'use crate::simd::aom_dsp__x86__lpf_common_sse2::*;\nuse crate::simd::aom_dsp__x86__synonyms::*;\n'
    for rel, cfg, year, feat, src, extra in [
            ('aom_dsp/x86/loopfilter_sse2.rs', 'loopfilter_sse2.json', 2017, 'sse2', 'loopfilter_sse2.c',
             'use crate::aom_dsp__aom_dsp_rtcd::{aom_lpf_horizontal_14_quad, aom_lpf_horizontal_6_quad,\n    aom_lpf_horizontal_8_quad};\n'),
            ('aom_dsp/x86/loopfilter_avx2.rs', 'loopfilter_avx2.json', 2018, 'avx2', 'loopfilter_avx2.c', ''),
            ('aom_dsp/x86/highbd_loopfilter_sse2.rs', 'highbd_loopfilter_sse2.json', 2016, 'sse2', 'highbd_loopfilter_sse2.c', ''),
            ('aom_dsp/x86/highbd_loopfilter_avx2.rs', 'highbd_loopfilter_avx2.json', 2017, 'avx2', 'highbd_loopfilter_avx2.c',
             'use crate::simd::aom_dsp__x86__highbd_loopfilter_sse2::*;\n')]:
        text = run(cfg)
        import re as _re
        year = _re.search(r'Copyright \(c\) (\d+)', open(U + 'aom_dsp/x86/' + src).read()).group(1)
        write(rel, LIC(year) + f'\n// Translated from upstream/aom/aom_dsp/x86/{src} (ADR 0056, ADR 0058).\n'
              f'// Upstream compiles it with -m{feat}. The functions were\n// {PRINTER}.\n'
              + NOTES.format(feat=feat) + USE_LPF + extra + ALIGN + '\n' + text)



def intra_predictors():
    def year(src):
        return re.search(r'Copyright \(c\) (\d+)', open(U + src).read()).group(1)
    ALIGN = '''
#[repr(C, align(8))]
struct Aligned8<T>(T);
#[repr(C, align(16))]
pub struct Aligned16<T>(pub T);
#[repr(C, align(32))]
pub struct Aligned32<T>(pub T);
'''
    common = run('intrapred_common_h.json').replace('pub static', 'static')
    common = '\n// From aom_dsp/intrapred_common.h.\n' + common
    x86h = run('intrapred_x86_h.json')
    write('aom_dsp/x86/intrapred_x86.rs', LIC(year('aom_dsp/x86/intrapred_x86.h')) +
          '\n// Translated from upstream/aom/aom_dsp/x86/intrapred_x86.h (ADR 0056, ADR 0058).\n'
          f'// The inline functions were\n// {PRINTER}.\n' + NOTES.format(feat='sse2') + '\n' + x86h)
    utils = run('intrapred_utils_h.json')
    write('aom_dsp/x86/intrapred_utils.rs', LIC(year('aom_dsp/x86/intrapred_utils.h')) +
          '\n// Translated from upstream/aom/aom_dsp/x86/intrapred_utils.h (ADR 0056, ADR 0058).\n'
          f'// The tables and inline functions were\n// {PRINTER}.\n' + NOTES.format(feat='sse2')
          + ALIGN.replace('struct Aligned8<T>(T);', 'pub struct Aligned8<T>(pub T);') + '\n' + utils)
    USE = ('use crate::simd::aom_dsp__x86__intrapred_x86::*;\nuse crate::simd::aom_dsp__x86__intrapred_utils::*;\n'
           'use crate::simd::aom_dsp__x86__lpf_common_sse2::*;\nuse crate::simd::aom_dsp__x86__synonyms::*;\n')
    for rel, cfg, src, feat, tables in [
            ('aom_dsp/x86/intrapred_sse2.rs', 'intrapred_sse2.json', 'aom_dsp/x86/intrapred_sse2.c', 'sse2', False),
            ('aom_dsp/x86/intrapred_ssse3.rs', 'intrapred_ssse3.json', 'aom_dsp/x86/intrapred_ssse3.c', 'ssse3', True),
            ('aom_dsp/x86/intrapred_sse4.rs', 'intrapred_sse4.json', 'aom_dsp/x86/intrapred_sse4.c', 'sse4.1', False)]:
        text = run(cfg)
        write(rel, LIC(year(src)) + f'\n// Translated from upstream/aom/{src} (ADR 0056, ADR 0058).\n'
              f'// Upstream compiles it with -m{feat.replace(".", "")}. The functions were\n// {PRINTER}.\n'
              + NOTES.format(feat=feat) + USE
              + (common if tables else '\nconst SMOOTH_WEIGHT_LOG2_SCALE: i32 = 8;\n') + '\n' + text)
    # intrapred_avx2.rs: the existing high-bitdepth translation plus the 8-bit functions.
    orig = part('intrapred_avx2.hand.rs')
    new = run('intrapred_avx2.json')
    head, rest = orig.split('use core::arch::x86_64::*;\n', 1)
    head = head.replace('''// Only the high-bitdepth directional predictors are translated:
// av1_highbd_dr_prediction_z1_avx2, av1_highbd_dr_prediction_z2_avx2 and
// av1_highbd_dr_prediction_z3_avx2, with the CONFIG_AV1_HIGHBITDEPTH tables
// and transposes and the static helpers they call. The low-bitdepth DC, V, H,
// Paeth and directional predictors of the same file are not translated.''', f'''// The high-bitdepth directional predictors (av1_highbd_dr_prediction_z1_avx2,
// av1_highbd_dr_prediction_z2_avx2 and av1_highbd_dr_prediction_z3_avx2, with
// the CONFIG_AV1_HIGHBITDEPTH tables and transposes and the static helpers they
// call) were translated by hand first. The low-bitdepth DC, V, H, Paeth, smooth
// and directional predictors that follow them were
// {PRINTER}, with the notes of the other x86 files below.''')
    head = head.replace('''// The predictors use no helper from intrapred_x86.h or intrapred_utils.h (the
// tables and transposes there are low-bitdepth); the high-bitdepth transposes
// they call from lpf_common_sse2.h are in crate::simd::aom_dsp__x86__lpf_common_sse2.
''', '''// The high-bitdepth predictors use no helper from intrapred_x86.h or
// intrapred_utils.h (the tables and transposes there are low-bitdepth); the
// 8-bit ones use them from crate::simd::aom_dsp__x86__intrapred_x86 and
// crate::simd::aom_dsp__x86__intrapred_utils. The transposes from
// lpf_common_sse2.h are in crate::simd::aom_dsp__x86__lpf_common_sse2.
''')
    rest = rest.replace('use crate::simd::aom_dsp__x86__lpf_common_sse2::*;\n',
                        USE.replace('use crate::simd::aom_dsp__x86__intrapred_utils::*;\n',
                                    'use crate::simd::aom_dsp__x86__intrapred_utils::{\n    transpose16x16_sse2, transpose4x16_sse2, transpose, transpose_TX_16X16, EvenOddMaskx,\n    LoadMaskx, LoadMaskz2,\n};\n'), 1)
    write('aom_dsp/x86/intrapred_avx2.rs', head.rstrip() + '\n#![allow(non_snake_case, non_upper_case_globals, unused_assignments, unused_mut, unused_parens)]\n'
          '#![allow(unused_variables, dead_code, clippy::all)]\n\nuse core::arch::x86_64::*;\n' + rest.rstrip()
          + '\n' + common.replace('Aligned8', 'Aligned8Table')
          + '\n#[repr(C, align(8))]\nstruct Aligned8Table<T>(T);\n\n'
          + '// ---------------------------------------------------------------------------\n'
          + '// The 8-bit predictors of aom_dsp/x86/intrapred_avx2.c.\n\n' + new)



def year_of(src):
    return re.search(r'Copyright \(c\) (\d+)', (ROOT / 'upstream/aom' / src).read_text()).group(1)


def simple_file(rel, cfg, src, feat, uses, extra='', note=''):
    """One printed C file with the standard header and notes."""
    text = run(cfg)
    write(rel, LIC(year_of(src)) + f'\n// Translated from upstream/aom/{src} (ADR 0056, ADR 0058).\n'
          f'// Upstream compiles it with -m{feat.replace(".", "")}. The functions were\n// {PRINTER}.\n'
          + note + NOTES.format(feat=feat) + uses + extra + '\n' + text)


def quantizers():
    hand = part('quantize_x86.hand.rs')
    head, body = hand.split('use core::arch::x86_64::*;\n', 1)
    write('aom_dsp/x86/quantize_x86.rs', LIC(year_of('aom_dsp/x86/quantize_x86.h')) + '''
// Translated from upstream/aom/aom_dsp/x86/quantize_x86.h (ADR 0056, ADR
// 0058). invert_sign_32_sse2 and calculate_non_zero_count, which the
// high-bitdepth adaptive quantizers use, were translated by hand first; the
// other inline functions, which the 8-bit quantizers use, were
// ''' + PRINTER + '''. Every helper carries
// `sse2`, the smallest feature of its intrinsics.
#![allow(non_snake_case, non_upper_case_globals, unused_assignments, unused_mut, unused_parens)]
#![allow(unused_variables, dead_code, clippy::all)]

use core::arch::x86_64::*;

// From aom_dsp/aom_dsp_common.h.
const AOM_QM_BITS: i32 = 5;
''' + body.rstrip() + '\n\n' + run('quantize_x86_h.json'))
    q = 'use crate::simd::aom_dsp__x86__quantize_x86::*;\nuse crate::simd::aom_dsp__x86__synonyms::*;\n'
    c = 'use crate::aom_dsp__quantize::aom_quantize_b_helper_c;\n'
    simple_file('aom_dsp/x86/quantize_sse2.rs', 'quantize_sse2.json', 'aom_dsp/x86/quantize_sse2.c', 'sse2', q)
    simple_file('aom_dsp/x86/quantize_ssse3.rs', 'quantize_ssse3.json', 'aom_dsp/x86/quantize_ssse3.c', 'ssse3', q + c)
    simple_file('aom_dsp/x86/quantize_avx2.rs', 'quantize_avx2.json', 'aom_dsp/x86/quantize_avx2.c', 'avx2',
                q + 'use crate::simd::aom_dsp__x86__synonyms_avx2::*;\n')
    simple_file('av1/encoder/x86/av1_quantize_sse2.rs', 'av1_quantize_sse2.json', 'av1/encoder/x86/av1_quantize_sse2.c',
                'sse2', q)
    simple_file('av1/encoder/x86/av1_quantize_avx2.rs', 'av1_quantize_avx2.json', 'av1/encoder/x86/av1_quantize_avx2.c',
                'avx2', q + 'use crate::simd::aom_dsp__x86__synonyms_avx2::*;\n', '\n// stdint.h\nconst INT16_MAX: i32 = 0x7fff;\n')


def sse_and_sum_squares():
    x = 'use crate::simd::aom_dsp__x86__synonyms::*;\n'
    y = x + 'use crate::simd::aom_dsp__x86__synonyms_avx2::*;\n'
    simple_file('aom_dsp/x86/sse_sse4.rs', 'sse_sse4.json', 'aom_dsp/x86/sse_sse4.c', 'sse4.1', x)
    simple_file('aom_dsp/x86/sse_avx2.rs', 'sse_avx2.json', 'aom_dsp/x86/sse_avx2.c', 'avx2', y)
    sq = 'use crate::aom_dsp__sum_squares::{aom_sum_squares_2d_i16_c, aom_sum_squares_i16_c, aom_sum_sse_2d_i16_c};\n'
    blk = 'use crate::aom_dsp__blk_sse_sum::aom_get_blk_sse_sum_c;\n'
    simple_file('aom_dsp/x86/sum_squares_sse2.rs', 'sum_squares_sse2.json', 'aom_dsp/x86/sum_squares_sse2.c', 'sse2',
                x + sq)
    simple_file('aom_dsp/x86/sum_squares_avx2.rs', 'sum_squares_avx2.json', 'aom_dsp/x86/sum_squares_avx2.c', 'avx2',
                y + sq + 'use crate::simd::aom_dsp__x86__sum_squares_sse2::*;\n')
    simple_file('aom_dsp/x86/blk_sse_sum_sse2.rs', 'blk_sse_sum_sse2.json', 'aom_dsp/x86/blk_sse_sum_sse2.c', 'sse2',
                x + blk)
    simple_file('aom_dsp/x86/blk_sse_sum_avx2.rs', 'blk_sse_sum_avx2.json', 'aom_dsp/x86/blk_sse_sum_avx2.c', 'avx2',
                y + blk)
    part_note = ('// Only the functions of this file that the still-image encoder dispatches (the block\n'
                 '// variances and MSEs, aom_get_var_sse_sum_*, aom_mse_wxh_16bit*, aom_mse_16xh_16bit*) and\n'
                 '// the static helpers they call are translated; the macros that define the block\n'
                 '// variances are expanded by the preprocessor. The sub-pixel and averaging kernels are\n'
                 '// not translated (working decision O2, 2026-11-13, delegated, revisable by the owner).\n')
    hb_note = ('// Only the functions of this file that call no NASM kernel are translated:\n'
               '// aom_mse_wxh_16bit_highbd_sse2 and its helpers. The block variances and MSEs call\n'
               '// aom_highbd_calc{8x8,16x16}var_sse2 of highbd_variance_impl_sse2.asm and stay on `_c`.\n')
    rnd = '// ROUND_POWER_OF_TWO (aom_ports/mem.h) is written out through the config\'s `replace`.\n'
    hb_fn = ('\n// typedef void (*high_variance_fn_t)(const uint16_t *src, int src_stride,\n'
             '//                                    const uint16_t *ref, int ref_stride,\n'
             '//                                    uint32_t *sse, int *sum);\n'
             'type high_variance_fn_t = Option<unsafe fn(*const u16, i32, *const u16, i32, *mut u32, *mut i32)>;\n')
    for rel, cfg, src, feat, uses, note, extra in [
            ('aom_dsp/x86/variance_sse2.rs', 'variance_sse2_mse.json', 'aom_dsp/x86/variance_sse2.c', 'sse2',
             x + 'use crate::simd::aom_dsp__x86__mem_sse2::loadu_int32;\n', part_note, ''),
            ('aom_dsp/x86/variance_avx2.rs', 'variance_avx2_mse.json', 'aom_dsp/x86/variance_avx2.c', 'avx2', y,
             part_note, ''),
            ('aom_dsp/x86/highbd_variance_sse2.rs', 'highbd_variance_sse2_mse.json', 'aom_dsp/x86/highbd_variance_sse2.c',
             'sse2', x, hb_note, ''),
            ('aom_dsp/x86/highbd_variance_avx2.rs', 'highbd_variance_avx2_mse.json', 'aom_dsp/x86/highbd_variance_avx2.c',
             'avx2', y, part_note + rnd, hb_fn),
            ('aom_dsp/x86/highbd_variance_sse4.rs', 'highbd_variance_sse4.json', 'aom_dsp/x86/highbd_variance_sse4.c',
             'sse4.1', '', '// Only the block variances are translated; the sub-pixel kernels are not.\n' + rnd, '')]:
        simple_file(rel, cfg, src, feat, uses, extra=extra, note=note)

def txb():
    x = 'use crate::simd::aom_dsp__x86__synonyms::*;\n'
    pad = '\n// From av1/common/enums.h.\nconst TX_PAD_HOR: i32 = 4;\nconst TX_PAD_BOTTOM: i32 = 4;\n'
    simple_file('av1/encoder/x86/encodetxb_sse4.rs', 'encodetxb_sse4.json', 'av1/encoder/x86/encodetxb_sse4.c', 'sse4.1',
                x, pad)
    simple_file('av1/encoder/x86/encodetxb_avx2.rs', 'encodetxb_avx2.json', 'av1/encoder/x86/encodetxb_avx2.c', 'avx2',
                x + 'use crate::simd::aom_dsp__x86__synonyms_avx2::*;\n', pad)


FWD_USES = ('use crate::av1__common__idct::TxfmParam;\n'
            'use crate::av1__encoder__av1_fwd_txfm2d::*;\n'
            'use crate::simd::aom_dsp__x86__transpose_sse2::*;\n'
            'use crate::simd::aom_dsp__x86__txfm_common_sse2::*;\n'
            'use crate::simd::av1__common__x86__av1_inv_txfm_ssse3::{\n'
            '    cospi_arr, get_flip_cfg, get_rect_tx_log_ratio, get_txh_idx, get_txw_idx, sinpi_arr, tx_size_high,\n'
            '    tx_size_wide, AOMMIN, ADST_ADST, ADST_DCT, ADST_FLIPADST, DCT_ADST, DCT_DCT, DCT_FLIPADST, FLIPADST_ADST,\n'
            '    FLIPADST_DCT, FLIPADST_FLIPADST, H_ADST, H_DCT, H_FLIPADST, IDTX, NewInvSqrt2, NewSqrt2, NewSqrt2Bits,\n'
            '    TX_16X16, TX_16X32, TX_16X4, TX_16X64, TX_16X8, TX_32X16, TX_32X32, TX_32X64, TX_32X8, TX_4X16, TX_4X4,\n'
            '    TX_4X8, TX_64X16, TX_64X32, TX_64X64, TX_8X16, TX_8X32, TX_8X4, TX_8X8, TX_SIZES, TX_SIZES_ALL, TX_TYPES,\n'
            '    V_ADST, V_DCT, V_FLIPADST,\n'
            '};\n'
            'use crate::simd::av1__common__x86__av1_txfm_sse2::*;\n')
FWD_TYPES = ('\n// av1/common/enums.h\ntype TX_SIZE = u8;\ntype TX_TYPE = u8;\n\n' + 'extern "C-unwind" {\n    // av1/encoder/hybrid_fwd_txfm.c, declared with this crate\'s TxfmParam as the Neon\n    // translation does.\n    fn av1_lowbd_fwd_txfm_c(src_diff: *const i16, coeff: *mut i32, diff_stride: i32,\n                            txfm_param: *mut TxfmParam);\n}\n')


def forward_transform():
    head = run('av1_fwd_txfm_sse2_h.json')
    body = run('av1_fwd_txfm_sse2.json')
    write('av1/encoder/x86/av1_fwd_txfm_sse2.rs', LIC(year_of('av1/encoder/x86/av1_fwd_txfm_sse2.c'))
          + '\n// Translated from upstream/aom/av1/encoder/x86/av1_fwd_txfm_sse2.h and av1_fwd_txfm_sse2.c\n'
          '// (ADR 0056, ADR 0058). Upstream compiles the .c file with -msse2. The inline functions,\n'
          '// tables and functions were\n// ' + PRINTER + '.\n'
          '// av1_lowbd_fwd_txfm_sse2 is the RTCD specialization only on 32-bit x86; on x86-64 it is a\n'
          '// plain function, and the 2D functions are reached from the SSE4.1 and AVX2 entries.\n'
          + NOTES.format(feat='sse2') + FWD_USES + FWD_TYPES
          + '\n/// transform_1d_sse2 (av1/common/x86/av1_txfm_sse2.h).\n'
          'pub type transform_1d_sse2 = Option<unsafe fn(*const __m128i, *mut __m128i, i8)>;\n\n'
          '// av1/encoder/x86/av1_fwd_txfm_sse2.h\n\n' + head
          + '\n// ---------------------------------------------------------------------------\n'
          '// av1/encoder/x86/av1_fwd_txfm_sse2.c\n\n' + body)
    ah = run('av1_fwd_txfm_avx2_h.json')
    ab = run('av1_fwd_txfm2d_avx2.json')
    write('av1/encoder/x86/av1_fwd_txfm2d_avx2.rs', LIC(year_of('av1/encoder/x86/av1_fwd_txfm2d_avx2.c'))
          + '\n// Translated from upstream/aom/av1/encoder/x86/av1_fwd_txfm_avx2.h and av1_fwd_txfm2d_avx2.c\n'
          '// (ADR 0056, ADR 0058). Upstream compiles the .c file with -mavx2. The functions were\n// '
          + PRINTER + '.\n'
          '// TRANSPOSE_8X8_AVX2() reads b0..b3 and writes c0..c3 of its caller; the Rust macro takes\n'
          '// them as arguments, and its two call sites pass them by name.\n'
          + NOTES.format(feat='avx2') + FWD_USES
          + 'use crate::simd::aom_dsp__x86__txfm_common_avx2::*;\n'
          'use crate::simd::av1__encoder__x86__av1_fwd_txfm_sse2::*;\n'
          'use crate::simd::av1__encoder__x86__av1_txfm1d_sse4::*;\n' + FWD_TYPES
          + '\ntype transform_1d_avx2 = Option<unsafe fn(*const __m256i, *mut __m256i, i8)>;\n'
          'type FwdTxfm2dFunc = Option<unsafe fn(*const i16, *mut i32, i32, u8, i32)>;\n'
          + part('fwd_transpose_8x8_avx2.rs')
          + '\n// av1/encoder/x86/av1_fwd_txfm_avx2.h\n\n' + ah
          + '\n// ---------------------------------------------------------------------------\n'
          '// av1/encoder/x86/av1_fwd_txfm2d_avx2.c\n\n' + ab)
    hand = part('av1_fwd_txfm2d_sse4.hand.rs')
    write('av1/encoder/x86/av1_fwd_txfm2d_sse4.rs', hand.rstrip() + '\n\n'
          '// ---------------------------------------------------------------------------\n'
          '// The low-bitdepth functions of av1_fwd_txfm2d_sse4.c (lowbd_fwd_txfm2d_*_sse4_1 and\n'
          '// av1_lowbd_fwd_txfm_sse4_1), ' + PRINTER + '.\n\n'
          + part('fwd_sse4_lowbd_uses.rs') + '\n' + run('av1_fwd_txfm2d_sse4.json'))
    hand = part('highbd_fwd_txfm_sse4.hand.rs')
    write('av1/encoder/x86/highbd_fwd_txfm_sse4.rs', hand.rstrip() + '\n\n'
          '// ---------------------------------------------------------------------------\n'
          '// av1_fwht4x4_sse4_1, ' + PRINTER + '.\n\n'
          '// From aom_dsp/txfm_common.h.\nconst UNIT_QUANT_SHIFT: i32 = 2;\n\n'
          + run('highbd_fwd_txfm_sse4_fwht.json'))


CFL_HEAD = ('\n// av1/common/blockd.h\n'
            'pub const CFL_BUF_LINE: i32 = 32;\n'
            'pub const CFL_BUF_LINE_I128: i32 = CFL_BUF_LINE >> 3;\n'
            'pub const CFL_BUF_LINE_I256: i32 = CFL_BUF_LINE >> 4;\n'
            '// av1/common/enums.h\n'
            'const TX_SIZES_ALL: i32 = 19;\n\n'
            '// av1/common/cfl.h and config/av1_rtcd.h: the function types the RTCD getters return.\n'
            'pub type cfl_subsample_lbd_fn = Option<unsafe extern "C-unwind" fn(*const u8, i32, *mut u16)>;\n'
            'pub type cfl_subsample_hbd_fn = Option<unsafe extern "C-unwind" fn(*const u16, i32, *mut u16)>;\n'
            'pub type cfl_subtract_average_fn = Option<unsafe extern "C-unwind" fn(*const u16, *mut i16)>;\n'
            'pub type cfl_predict_lbd_fn = Option<unsafe extern "C-unwind" fn(*const i16, *mut u8, i32, i32)>;\n'
            'pub type cfl_predict_hbd_fn = Option<unsafe extern "C-unwind" fn(*const i16, *mut u16, i32, i32, i32)>;\n')
CFL_NOTE = ('// The size wrappers and RTCD getters that the CFL_* macros of av1/common/cfl.h generate are\n'
            '// expanded with the C preprocessor before printing (tools/x86_c2r/c2r.py `cpp`); the\n'
            '// wrappers keep the C ABI because the getters return them as C function pointers.\n')


def cfl():
    x = 'use crate::simd::aom_dsp__x86__synonyms::*;\n'
    simple_file('av1/common/x86/cfl_sse2.rs', 'cfl_sse2.json', 'av1/common/x86/cfl_sse2.c', 'sse2', x, CFL_HEAD,
                note=CFL_NOTE)
    simple_file('av1/common/x86/cfl_ssse3.rs', 'cfl_ssse3.json', 'av1/common/x86/cfl_ssse3.c', 'ssse3', x,
                CFL_HEAD.replace('pub const CFL', 'const CFL').replace('pub type', 'type'), note=CFL_NOTE)
    simple_file('av1/common/x86/cfl_avx2.rs', 'cfl_avx2.json', 'av1/common/x86/cfl_avx2.c', 'avx2',
                x + 'use crate::simd::aom_dsp__x86__synonyms_avx2::*;\n'
                'use crate::simd::av1__common__x86__cfl_ssse3::*;\nuse crate::simd::av1__common__x86__cfl_sse2::*;\n',
                CFL_HEAD.replace('pub const CFL', 'const CFL').replace('pub type', 'type'), note=CFL_NOTE)


LR_NOTE = ('// The restoration and filter macros of av1/common/restoration.h, aom_dsp/aom_filter.h,\n'
           '// av1/common/enums.h, aom_dsp/aom_dsp_common.h, aom_ports/mem.h and av1/common/convolve.h\n'
           '// are expanded with the C preprocessor before printing (tools/x86_c2r/c2r.py `cpp`).\n')
LR_ALIGNED16 = '\n#[repr(C, align(16))]\nstruct Aligned16<T>(T);\n'


def loop_restoration():
    x = 'use crate::simd::aom_dsp__x86__synonyms::*;\n'
    y = x + 'use crate::simd::aom_dsp__x86__synonyms_avx2::*;\n'
    write('aom_dsp/x86/convolve_avx2.rs', LIC(year_of('aom_dsp/x86/convolve_avx2.h'))
          + '\n// Translated from upstream/aom/aom_dsp/x86/convolve_avx2.h (ADR 0056, ADR 0058).\n'
          '// Only the tables and inline functions the Wiener filter of\n'
          '// av1/common/x86/wiener_convolve_avx2.c uses are translated (convolve, convolve_lowbd_x and\n'
          '// the filter tables); the inter-prediction convolutions are not (working decision O2,\n'
          '// 2026-11-13, delegated, revisable by the owner). They were\n// ' + PRINTER + '.\n'
          + NOTES.format(feat='avx2') + y
          + '\n#[repr(C, align(32))]\npub struct Aligned32<T>(pub T);\n\n' + run('convolve_avx2_h.json'))
    wiener = ('use crate::av1__common__convolve::{\n    av1_highbd_wiener_convolve_add_src_c, av1_wiener_convolve_add_src_c, WienerConvolveParams,\n};\n')
    for rel, cfg, src, feat, uses in [
            ('av1/common/x86/wiener_convolve_sse2.rs', 'wiener_convolve_sse2.json', 'av1/common/x86/wiener_convolve_sse2.c',
             'sse2', x + wiener),
            ('av1/common/x86/wiener_convolve_avx2.rs', 'wiener_convolve_avx2.json', 'av1/common/x86/wiener_convolve_avx2.c',
             'avx2', y + wiener + 'use crate::simd::aom_dsp__x86__convolve_avx2::*;\n'),
            ('av1/common/x86/highbd_wiener_convolve_ssse3.rs', 'highbd_wiener_convolve_ssse3.json',
             'av1/common/x86/highbd_wiener_convolve_ssse3.c', 'ssse3', x + wiener),
            ('av1/common/x86/highbd_wiener_convolve_avx2.rs', 'highbd_wiener_convolve_avx2.json',
             'av1/common/x86/highbd_wiener_convolve_avx2.c', 'avx2', y + wiener + 'use crate::simd::aom_dsp__x86__convolve_avx2::*;\n'),
            ('av1/common/x86/selfguided_sse4.rs', 'selfguided_sse4.json', 'av1/common/x86/selfguided_sse4.c', 'sse4.1',
             x + 'use crate::av1__common__restoration::{\n    av1_decode_xq, av1_one_by_x, av1_sgr_params, av1_x_by_xplus1, sgr_params_type,\n};\n'
             'use crate::aom_mem__aom_mem::{aom_free, aom_memalign};\n'),
            ('av1/common/x86/selfguided_avx2.rs', 'selfguided_avx2.json', 'av1/common/x86/selfguided_avx2.c', 'avx2',
             y + 'use crate::av1__common__restoration::{\n    av1_decode_xq, av1_one_by_x, av1_sgr_params, av1_x_by_xplus1, sgr_params_type,\n};\n'
             'use crate::aom_mem__aom_mem::{aom_free, aom_memalign};\n'
             'use crate::simd::av1__common__x86__selfguided_sse4::{\n'
             '    av1_apply_selfguided_restoration_sse4_1, av1_selfguided_restoration_sse4_1,\n};\n')]:
        extra = LR_ALIGNED16 if feat in ('sse2', 'ssse3') else ''
        simple_file(rel, cfg, src, feat, uses, extra, note=LR_NOTE)


HORVER_NOTE = ('// The final floating-point block is the same sequence of single-precision\n'
               '// conversions and operations as the original, with no contraction, and calls\n'
               "// the C library's sqrtf as the original does.\n")
SQRTF = '\nextern "C" {\n    fn sqrtf(_: f32) -> f32;\n}\n'


def mem_sse2():
    write('aom_dsp/x86/mem_sse2.rs', LIC(year_of('aom_dsp/x86/mem_sse2.h'))
          + '\n// Translated from upstream/aom/aom_dsp/x86/mem_sse2.h (ADR 0056, ADR 0058).\n'
          '// Only the functions the translated x86 kernels call are translated; they were\n// ' + PRINTER + '.\n'
          + NOTES.format(feat='sse2') + '\n' + run('mem_sse2_h.json'))


def horver_correlation():
    mem_sse2()
    x = 'use crate::simd::aom_dsp__x86__synonyms::*;\n'
    simple_file('av1/encoder/x86/rdopt_sse4.rs', 'rdopt_sse4.json', 'av1/encoder/x86/rdopt_sse4.c', 'sse4.1',
                x, SQRTF, note=HORVER_NOTE)
    simple_file('av1/encoder/x86/rdopt_avx2.rs', 'rdopt_avx2.json', 'av1/encoder/x86/rdopt_avx2.c', 'avx2',
                x + 'use crate::simd::aom_dsp__x86__mem_sse2::loadu_int64;\n'
                'use crate::simd::aom_dsp__x86__synonyms_avx2::*;\n', SQRTF, note=HORVER_NOTE)


NZ_NOTE = ('// The TX_PAD_HOR, SIG_COEF_CONTEXTS_2D and TX_CLASS constants and get_txb_bhl,\n'
           '// get_txb_wide and get_txb_high (with av1_get_adjusted_tx_size and\n'
           '// tx_size_high_log2) of av1/common/txb_common.h, blockd.h and common_data.h\n'
           '// were transcribed by hand (tools/x86_c2r/parts/txb_common_sse2.hand.rs).\n')


def nz_map_contexts():
    simple_file('av1/encoder/x86/encodetxb_sse2.rs', 'encodetxb_sse2.json', 'av1/encoder/x86/encodetxb_sse2.c', 'sse2',
                'use crate::simd::aom_dsp__x86__mem_sse2::{load_8bit_4x4_to_1_reg_sse2, load_8bit_8x2_to_1_reg_sse2};\n'
                'use crate::simd::av1__common__x86__av1_inv_txfm_ssse3::{tx_size_high, tx_size_wide};\n',
                '\n' + part('txb_common_sse2.hand.rs'), note=NZ_NOTE)


def interp_cubic_rate_dist():
    simple_file('av1/encoder/x86/model_rd_sse2.rs', 'model_rd_sse2.json', 'av1/encoder/x86/model_rd_sse2.c', 'sse2', '')


CNN_NOTE = ('// The two statement macros of the file (PERFORM_CONVOLVE_FOR_3_5X5_BLOCKS,\n'
            '// PERFORM_CONVOLVE_FOR_1_5X5_BLOCK) were expanded with the C preprocessor\n'
            '// before printing; their `do { ... } while (0)` stays a loop that runs once.\n'
            '// Each `_mm*_mul_ps` and `_mm*_add_ps` and each scalar `*` and `+` is a\n'
            '// separate rounding, as in the original built with -ffp-contract=off\n'
            '// (ADR 0024); Rust never contracts.\n')


def cnn():
    simple_file('av1/encoder/x86/cnn_avx2.rs', 'cnn_avx2.json', 'av1/encoder/x86/cnn_avx2.c', 'avx2',
                'use crate::av1__encoder__cnn::{av1_cnn_convolve_no_maxpool_padding_valid_c, CNN_LAYER_CONFIG};\n',
                ALIGN, note=CNN_NOTE)


HADAMARD_NOTE = ('// Only the Hadamard kernels of the file are translated (the config\'s `only`\n'
                 '// list; the averaging, SATD, projection and variance kernels after them are\n'
                 '// not on the still-image path).\n')


def hadamard():
    for ext in ['sse2', 'avx2']:
        h = f'aom_dsp/x86/bitdepth_conversion_{ext}.h'
        write(f'aom_dsp/x86/bitdepth_conversion_{ext}.rs', LIC(year_of(h))
              + f'\n// Translated from upstream/aom/{h} (ADR 0056, ADR 0058).\n'
              '// The functions were\n// ' + PRINTER + '.\n'
              + NOTES.format(feat=ext) + '\n' + run(f'bitdepth_conversion_{ext}_h.json'))
    simple_file('aom_dsp/x86/avg_intrin_sse2.rs', 'avg_intrin_sse2.json', 'aom_dsp/x86/avg_intrin_sse2.c', 'sse2',
                'use crate::simd::aom_dsp__x86__bitdepth_conversion_sse2::*;\n', ALIGN, note=HADAMARD_NOTE)
    simple_file('aom_dsp/x86/avg_intrin_avx2.rs', 'avg_intrin_avx2.json', 'aom_dsp/x86/avg_intrin_avx2.c', 'avx2',
                'use crate::simd::aom_dsp__x86__bitdepth_conversion_avx2::*;\n', ALIGN, note=HADAMARD_NOTE)


NN_NOTE = ('// Each `_mm*_mul_ps` and `_mm*_add_ps` and each scalar `*` and `+` is a\n'
           '// separate rounding, as in the original built with -ffp-contract=off\n'
           '// (ADR 0024); `float` literals stay single precision.\n')


def nn():
    nn_uses = 'use crate::av1__encoder__ml::{av1_nn_output_prec_reduce, NN_CONFIG};\n'
    nn_const = '\n// av1/encoder/ml.h\nconst NN_MAX_NODES_PER_LAYER: i32 = 128;\n'
    simple_file('av1/encoder/x86/ml_sse3.rs', 'ml_sse3.json', 'av1/encoder/x86/ml_sse3.c', 'sse3', nn_uses,
                nn_const, note=NN_NOTE + '// approx_exp\'s object-like macros A, B and C are written out in place.\n')
    simple_file('av1/encoder/x86/ml_avx2.rs', 'ml_avx2.json', 'av1/encoder/x86/ml_avx2.c', 'avx2',
                nn_uses + 'use crate::simd::av1__encoder__x86__ml_sse3::{av1_nn_propagate_4to1_sse3,\n'
                '    av1_nn_propagate_4to4_sse3, av1_nn_propagate_4to8_sse3};\n', nn_const,
                note=NN_NOTE + '// The statement macro CALC_OUTPUT_FOR_2ROWS was expanded with the C\n'
                '// preprocessor before printing; AOMMAX(bias_val, 0) is written out.\n')


def highbd_quantize_fp_sse4():
    simple_file('av1/encoder/x86/av1_highbd_quantize_sse4.rs', 'av1_highbd_quantize_sse4.json',
                'av1/encoder/x86/av1_highbd_quantize_sse4.c', 'sse4.1', '',
                note='// ROUND_POWER_OF_TWO (aom_ports/mem.h) is written out through the config\'s `replace`.\n')


def main():
    global OUT, WORK
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--out', type=Path)
    parser.add_argument('--check', action='store_true')
    args = parser.parse_args()
    with tempfile.TemporaryDirectory() as tmp:
        WORK = Path(tmp) / 'work'
        OUT = args.out or (Path(tmp) / 'simd' if args.check else SIMD)
        inverse_transform()
        loop_filter()
        intra_predictors()
        quantizers()
        sse_and_sum_squares()
        txb()
        forward_transform()
        cfl()
        loop_restoration()
        horver_correlation()
        nz_map_contexts()
        interp_cubic_rate_dist()
        cnn()
        hadamard()
        nn()
        highbd_quantize_fp_sse4()
        if args.check:
            bad = []
            for path in sorted(OUT.rglob('*.rs')):
                rel = path.relative_to(OUT)
                if path.read_bytes() != (SIMD / rel).read_bytes():
                    bad.append(str(rel))
            print(f'{len(list(OUT.rglob("*.rs")))} files, {len(bad)} differ: {bad}')
            # The integer aliases enable_aom_dispatch.py adds to the Linux RTCD units.
            subprocess.run([sys.executable, str(ROOT / 'tools/enable_aom_dispatch.py'), '--linux', '--check'],
                           check=True)
            if bad:
                raise SystemExit(1)


if __name__ == '__main__':
    main()
