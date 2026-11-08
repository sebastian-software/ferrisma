/*
 * Copyright (c) 2016, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

// Translated by hand from upstream/aom/av1/common/x86/highbd_inv_txfm_avx2.c
// (ADR 0056, ADR 0058). Upstream compiles this file with -mavx2. Function
// names, helper structure, comments, constants, and the intrinsic sequence
// follow the original statement by statement. The function bodies were
// transliterated with a typing-aware C-subset printer (not C2Rust) and checked
// against the C function by function: the same sequence of intrinsic and
// helper calls, and the same identifiers, constants, and operators in order.
//
// Translation notes:
// - Every function carries `avx2`. The RTCD specialization
//   av1_highbd_inv_txfm_add_avx2 is exported under its upstream name. It
//   delegates the 4xN and Nx4 sizes to av1_highbd_inv_txfm_add_sse4_1 and the
//   identity types to av1_highbd_inv_txfm2d_add_universe_sse4_1, both from
//   the translation of highbd_inv_txfm_sse4.c, as upstream does.
// - Immediate operands become const generics. Shift counts that are run-time
//   `int` values in C (`_mm256_srai_epi32(x, bit)`,
//   `_mm256_srai_epi32(x, shift)`, `_mm256_slli_epi16(one, bd)`) are
//   translated with the count-register intrinsics
//   (`_mm256_sra_epi32(x, _mm_cvtsi32_si128(bit))`, ...), which is what GCC
//   and Clang emit for a non-constant count: the count is zero-extended by
//   MOVD and counts above the lane width saturate identically, so the result
//   is bit-identical for every `int` count.
// - Pointer and array parameters stay raw pointers (`in[i]` ->
//   `*in_.offset(i)`, `out + 3` -> `out.add(3)`), so in-place calls keep the
//   C aliasing. Local arrays are Rust arrays (`u[i]` -> `u[i as usize]`,
//   `&u[i]` -> `u.as_mut_ptr().offset(i)`) and start zeroed; C locals that
//   are declared without an initializer start zeroed (integers, and vectors
//   whose address is taken first) or uninitialized (other vectors, which the
//   compiler checks are written first).
// - `for` loops whose counter the body does not modify become range loops;
//   the others keep their `while` form with the increment at the end.
// - `switch` becomes `match` on the `int`-promoted value; `assert` becomes
//   `debug_assert!`. Calling a NULL `transform_1d_avx2` entry, which C leaves
//   undefined, panics through `Option::unwrap`.
// - C integer promotions are explicit: `int8_t`, `uint8_t`, `TX_SIZE`, and
//   `TX_TYPE` operands are converted to `i32` before arithmetic, as C does.
// - Header-local (`static`) tables, inline functions, and macros of the
//   included headers are transcribed below, as in the Neon translation;
//   exported tables and functions come from the translated modules.
#![allow(non_snake_case, non_upper_case_globals, unused_assignments, unused_mut, unused_parens)]
#![allow(unused_variables, dead_code, clippy::all)]

use core::arch::x86_64::*;

use crate::av1__common__av1_inv_txfm2d::av1_inv_txfm_shift_ls;
use crate::av1__common__av1_txfm::av1_cospi_arr_data;
use crate::av1__common__idct::TxfmParam;
use crate::simd::aom_dsp__x86__txfm_common_avx2::{
    round_shift_array_32_avx2, round_shift_rect_array_32_avx2,
};
use crate::simd::av1__common__x86__av1_inv_txfm_ssse3::{
    get_eobx_eoby_scan_default, hitx_1d_tab, lowbd_txfm_all_1d_zeros_idx, vitx_1d_tab,
};
use crate::simd::av1__common__x86__highbd_inv_txfm_sse4::{
    av1_highbd_inv_txfm2d_add_universe_sse4_1, av1_highbd_inv_txfm_add_sse4_1,
};
use crate::simd::types::tran_low_t;

// ---------------------------------------------------------------------------
// Header items used by the original translation unit.

// av1/common/enums.h
type TX_TYPE = u8;
type TX_SIZE = u8;

const TX_4X4: i32 = 0;
const TX_8X8: i32 = 1;
const TX_16X16: i32 = 2;
const TX_32X32: i32 = 3;
const TX_64X64: i32 = 4;
const TX_4X8: i32 = 5;
const TX_8X4: i32 = 6;
const TX_8X16: i32 = 7;
const TX_16X8: i32 = 8;
const TX_16X32: i32 = 9;
const TX_32X16: i32 = 10;
const TX_32X64: i32 = 11;
const TX_64X32: i32 = 12;
const TX_4X16: i32 = 13;
const TX_16X4: i32 = 14;
const TX_8X32: i32 = 15;
const TX_32X8: i32 = 16;
const TX_16X64: i32 = 17;
const TX_64X16: i32 = 18;
const TX_SIZES_ALL: i32 = 19;
const TX_SIZES: i32 = TX_4X8;

const DCT_DCT: i32 = 0;
const ADST_DCT: i32 = 1;
const DCT_ADST: i32 = 2;
const ADST_ADST: i32 = 3;
const FLIPADST_DCT: i32 = 4;
const DCT_FLIPADST: i32 = 5;
const FLIPADST_FLIPADST: i32 = 6;
const ADST_FLIPADST: i32 = 7;
const FLIPADST_ADST: i32 = 8;
const IDTX: i32 = 9;
const V_DCT: i32 = 10;
const H_DCT: i32 = 11;
const V_ADST: i32 = 12;
const H_ADST: i32 = 13;
const V_FLIPADST: i32 = 14;
const H_FLIPADST: i32 = 15;
const TX_TYPES: i32 = 16;

// av1/common/av1_inv_txfm1d_cfg.h
const INV_COS_BIT: i32 = 12;

// av1/common/av1_txfm.h
const cos_bit_min: i32 = 10;
const NewSqrt2Bits: i32 = 12;
// 2^12 * sqrt(2)
const NewSqrt2: i32 = 5793;
// 2^12 / sqrt(2)
const NewInvSqrt2: i32 = 2896;

#[inline]
unsafe fn cospi_arr(n: i32) -> *const i32 {
    (&raw const av1_cospi_arr_data[(n - cos_bit_min) as usize]).cast::<i32>()
}

#[inline]
fn get_log_range_out(bd: i32) -> i32 {
    let log_range_out = AOMMAX(16, bd + 6);
    // bd is limited by the bitstream to 12. This assert is to satisfy static
    // analyzers that may assume `log_range_out - 1` is greater than 31.
    debug_assert!(log_range_out <= 18);
    log_range_out
}

#[inline]
fn get_log_range(bd: i32, do_cols: i32) -> i32 {
    let log_range = AOMMAX(16, bd + if do_cols != 0 { 6 } else { 8 });
    // bd is limited by the bitstream to 12. This assert is to satisfy static
    // analyzers that may assume `log_range - 1` is greater than 31.
    debug_assert!(log_range <= 20);
    log_range
}

#[inline]
unsafe fn get_flip_cfg(tx_type: TX_TYPE, ud_flip: *mut i32, lr_flip: *mut i32) {
    match tx_type as i32 {
        DCT_DCT | ADST_DCT | DCT_ADST | ADST_ADST => {
            *ud_flip = 0;
            *lr_flip = 0;
        }
        IDTX | V_DCT | H_DCT | V_ADST | H_ADST => {
            *ud_flip = 0;
            *lr_flip = 0;
        }
        FLIPADST_DCT | FLIPADST_ADST | V_FLIPADST => {
            *ud_flip = 1;
            *lr_flip = 0;
        }
        DCT_FLIPADST | ADST_FLIPADST | H_FLIPADST => {
            *ud_flip = 0;
            *lr_flip = 1;
        }
        FLIPADST_FLIPADST => {
            *ud_flip = 1;
            *lr_flip = 1;
        }
        _ => {
            *ud_flip = 0;
            *lr_flip = 0;
            debug_assert!(false);
        }
    }
}

// Utility function that returns the log of the ratio of the col and row
// sizes.
#[inline]
fn get_rect_tx_log_ratio(col: i32, row: i32) -> i32 {
    if col == row {
        return 0;
    }
    if col > row {
        if col == row * 2 {
            return 1;
        }
        if col == row * 4 {
            return 2;
        }
        debug_assert!(false, "Unsupported transform size");
    } else {
        if row == col * 2 {
            return -1;
        }
        if row == col * 4 {
            return -2;
        }
        debug_assert!(false, "Unsupported transform size");
    }
    0 // Invalid
}

#[inline]
fn get_txw_idx(tx_size: TX_SIZE) -> i32 {
    tx_size_wide_log2[tx_size as usize] - tx_size_wide_log2[0]
}

#[inline]
fn get_txh_idx(tx_size: TX_SIZE) -> i32 {
    tx_size_high_log2[tx_size as usize] - tx_size_high_log2[0]
}

// av1/common/common_data.h
// Transform block width in pixels
static tx_size_wide: [i32; 19] =
    [4, 8, 16, 32, 64, 4, 8, 8, 16, 16, 32, 32, 64, 4, 16, 8, 32, 16, 64];
// Transform block height in pixels
static tx_size_high: [i32; 19] =
    [4, 8, 16, 32, 64, 8, 4, 16, 8, 32, 16, 64, 32, 16, 4, 32, 8, 64, 16];
// Transform block width in log2
static tx_size_wide_log2: [i32; 19] = [2, 3, 4, 5, 6, 2, 3, 3, 4, 4, 5, 5, 6, 2, 4, 3, 5, 4, 6];
// Transform block height in log2
static tx_size_high_log2: [i32; 19] = [2, 3, 4, 5, 6, 3, 2, 4, 3, 5, 4, 6, 5, 4, 2, 5, 3, 6, 4];

// av1/common/blockd.h
static av1_ext_tx_used: [[i32; 16]; 6] = [
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0],
    [1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0],
    [1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
];

// aom_dsp/aom_dsp_common.h
#[inline]
fn AOMMIN(x: i32, y: i32) -> i32 {
    if x < y {
        x
    } else {
        y
    }
}

#[inline]
fn AOMMAX(x: i32, y: i32) -> i32 {
    if x > y {
        x
    } else {
        y
    }
}

// aom_ports/mem.h
#[inline]
fn CONVERT_TO_SHORTPTR(x: *mut u8) -> *mut u16 {
    ((x as usize) << 1) as *mut u16
}

// ---------------------------------------------------------------------------

// Note:
//  Total 32x4 registers to represent 32x32 block coefficients.
//  For high bit depth, each coefficient is 4-byte.
//  Each __m256i register holds 8 coefficients.
//  So each "row" we needs 4 register. Totally 32 rows
//  Register layout:
//   v0,   v1,   v2,   v3,
//   v4,   v5,   v6,   v7,
//   ... ...
//   v124, v125, v126, v127

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn highbd_clamp_epi16_avx2(mut u: __m256i, mut bd: i32) -> __m256i {
    let mut zero: __m256i = _mm256_setzero_si256();
    let mut one: __m256i = _mm256_set1_epi16(1);
    let mut max: __m256i = _mm256_sub_epi16(_mm256_sll_epi16(one, _mm_cvtsi32_si128(bd)), one);
    let mut clamped: __m256i;
    let mut mask: __m256i;

    mask = _mm256_cmpgt_epi16(u, max);
    clamped = _mm256_andnot_si256(mask, u);
    mask = _mm256_and_si256(mask, max);
    clamped = _mm256_or_si256(mask, clamped);
    mask = _mm256_cmpgt_epi16(clamped, zero);
    clamped = _mm256_and_si256(clamped, mask);

    return clamped;
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn round_shift_4x4_avx2(mut in_: *mut __m256i, mut shift: i32) {
    if shift != 0 {
        let mut rnding: __m256i = _mm256_set1_epi32(1 << (shift - 1));
        *in_.add(0) = _mm256_add_epi32(*in_.add(0), rnding);
        *in_.add(1) = _mm256_add_epi32(*in_.add(1), rnding);
        *in_.add(2) = _mm256_add_epi32(*in_.add(2), rnding);
        *in_.add(3) = _mm256_add_epi32(*in_.add(3), rnding);

        *in_.add(0) = _mm256_sra_epi32(*in_.add(0), _mm_cvtsi32_si128(shift));
        *in_.add(1) = _mm256_sra_epi32(*in_.add(1), _mm_cvtsi32_si128(shift));
        *in_.add(2) = _mm256_sra_epi32(*in_.add(2), _mm_cvtsi32_si128(shift));
        *in_.add(3) = _mm256_sra_epi32(*in_.add(3), _mm_cvtsi32_si128(shift));
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn round_shift_8x8_avx2(mut in_: *mut __m256i, mut shift: i32) {
    round_shift_4x4_avx2(in_, shift);
    round_shift_4x4_avx2(in_.add(4), shift);
    round_shift_4x4_avx2(in_.add(8), shift);
    round_shift_4x4_avx2(in_.add(12), shift);
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn highbd_clamp_epi32_avx2(
    mut in_: *mut __m256i,
    mut out: *mut __m256i,
    mut clamp_lo: *const __m256i,
    mut clamp_hi: *const __m256i,
    mut size: i32,
) {
    let mut a0: __m256i;
    let mut a1: __m256i;
    for i in (0..size).step_by(4) {
        a0 = _mm256_max_epi32(*in_.offset(i as isize), *clamp_lo);
        *out.offset(i as isize) = _mm256_min_epi32(a0, *clamp_hi);

        a1 = _mm256_max_epi32(*in_.offset((i + 1) as isize), *clamp_lo);
        *out.offset((i + 1) as isize) = _mm256_min_epi32(a1, *clamp_hi);

        a0 = _mm256_max_epi32(*in_.offset((i + 2) as isize), *clamp_lo);
        *out.offset((i + 2) as isize) = _mm256_min_epi32(a0, *clamp_hi);

        a1 = _mm256_max_epi32(*in_.offset((i + 3) as isize), *clamp_lo);
        *out.offset((i + 3) as isize) = _mm256_min_epi32(a1, *clamp_hi);
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn highbd_get_recon_16x8_avx2(
    mut pred: __m256i,
    mut res0: __m256i,
    mut res1: __m256i,
    mut bd: i32,
) -> __m256i {
    let mut x0: __m256i = _mm256_cvtepi16_epi32(_mm256_castsi256_si128(pred));
    let mut x1: __m256i = _mm256_cvtepi16_epi32(_mm256_extractf128_si256::<1>(pred));

    x0 = _mm256_add_epi32(res0, x0);
    x1 = _mm256_add_epi32(res1, x1);
    x0 = _mm256_packus_epi32(x0, x1);
    x0 = _mm256_permute4x64_epi64::<0xd8>(x0);
    x0 = highbd_clamp_epi16_avx2(x0, bd);
    return x0;
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn highbd_write_buffer_16xn_avx2(
    mut in_: *mut __m256i,
    mut output: *mut u16,
    mut stride: i32,
    mut flipud: i32,
    mut height: i32,
    mut bd: i32,
) {
    let mut j: i32 = if flipud != 0 { (height - 1) } else { 0 };
    let mut step: i32 = if flipud != 0 { -1 } else { 1 };
    {
        let mut i: i32 = 0;
        while i < height {
            let mut v: __m256i =
                _mm256_loadu_si256(output.offset((i * stride) as isize) as *const __m256i);
            let mut u: __m256i = highbd_get_recon_16x8_avx2(
                v,
                *in_.offset(j as isize),
                *in_.offset((j + height) as isize),
                bd,
            );

            _mm256_storeu_si256(output.offset((i * stride) as isize) as *mut __m256i, u);
            i += 1;
            j += step;
        }
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn highbd_get_recon_8x8_avx2(mut pred: __m256i, mut res: __m256i, mut bd: i32) -> __m256i {
    let mut x0: __m256i = pred;
    x0 = _mm256_add_epi32(res, x0);
    x0 = _mm256_packus_epi32(x0, x0);
    x0 = _mm256_permute4x64_epi64::<0xd8>(x0);
    x0 = highbd_clamp_epi16_avx2(x0, bd);
    return x0;
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn highbd_write_buffer_8xn_avx2(
    mut in_: *mut __m256i,
    mut output: *mut u16,
    mut stride: i32,
    mut flipud: i32,
    mut height: i32,
    mut bd: i32,
) {
    let mut j: i32 = if flipud != 0 { (height - 1) } else { 0 };
    let mut temp: __m128i;
    let mut step: i32 = if flipud != 0 { -1 } else { 1 };
    {
        let mut i: i32 = 0;
        while i < height {
            temp = _mm_loadu_si128(output.offset((i * stride) as isize) as *const __m128i);
            let mut v: __m256i = _mm256_cvtepi16_epi32(temp);
            let mut u: __m256i = highbd_get_recon_8x8_avx2(v, *in_.offset(j as isize), bd);
            let mut u1: __m128i = _mm256_castsi256_si128(u);
            _mm_storeu_si128(output.offset((i * stride) as isize) as *mut __m128i, u1);
            i += 1;
            j += step;
        }
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn neg_shift_avx2(
    mut in0: __m256i,
    mut in1: __m256i,
    mut out0: *mut __m256i,
    mut out1: *mut __m256i,
    mut clamp_lo: *const __m256i,
    mut clamp_hi: *const __m256i,
    mut shift: i32,
) {
    let mut offset: __m256i = _mm256_set1_epi32((1 << shift) >> 1);
    let mut a0: __m256i = _mm256_add_epi32(offset, in0);
    let mut a1: __m256i = _mm256_sub_epi32(offset, in1);

    a0 = _mm256_sra_epi32(a0, _mm_cvtsi32_si128(shift));
    a1 = _mm256_sra_epi32(a1, _mm_cvtsi32_si128(shift));

    a0 = _mm256_max_epi32(a0, *clamp_lo);
    a0 = _mm256_min_epi32(a0, *clamp_hi);
    a1 = _mm256_max_epi32(a1, *clamp_lo);
    a1 = _mm256_min_epi32(a1, *clamp_hi);

    *out0 = a0;
    *out1 = a1;
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn transpose_8x8_avx2(mut in_: *const __m256i, mut out: *mut __m256i) {
    let mut u0: __m256i;
    let mut u1: __m256i;
    let mut u2: __m256i;
    let mut u3: __m256i;
    let mut u4: __m256i;
    let mut u5: __m256i;
    let mut u6: __m256i;
    let mut u7: __m256i;
    let mut x0: __m256i;
    let mut x1: __m256i;

    u0 = _mm256_unpacklo_epi32(*in_.add(0), *in_.add(1));
    u1 = _mm256_unpackhi_epi32(*in_.add(0), *in_.add(1));

    u2 = _mm256_unpacklo_epi32(*in_.add(2), *in_.add(3));
    u3 = _mm256_unpackhi_epi32(*in_.add(2), *in_.add(3));

    u4 = _mm256_unpacklo_epi32(*in_.add(4), *in_.add(5));
    u5 = _mm256_unpackhi_epi32(*in_.add(4), *in_.add(5));

    u6 = _mm256_unpacklo_epi32(*in_.add(6), *in_.add(7));
    u7 = _mm256_unpackhi_epi32(*in_.add(6), *in_.add(7));

    x0 = _mm256_unpacklo_epi64(u0, u2);
    x1 = _mm256_unpacklo_epi64(u4, u6);
    *out.add(0) = _mm256_permute2f128_si256::<0x20>(x0, x1);
    *out.add(4) = _mm256_permute2f128_si256::<0x31>(x0, x1);

    x0 = _mm256_unpackhi_epi64(u0, u2);
    x1 = _mm256_unpackhi_epi64(u4, u6);
    *out.add(1) = _mm256_permute2f128_si256::<0x20>(x0, x1);
    *out.add(5) = _mm256_permute2f128_si256::<0x31>(x0, x1);

    x0 = _mm256_unpacklo_epi64(u1, u3);
    x1 = _mm256_unpacklo_epi64(u5, u7);
    *out.add(2) = _mm256_permute2f128_si256::<0x20>(x0, x1);
    *out.add(6) = _mm256_permute2f128_si256::<0x31>(x0, x1);

    x0 = _mm256_unpackhi_epi64(u1, u3);
    x1 = _mm256_unpackhi_epi64(u5, u7);
    *out.add(3) = _mm256_permute2f128_si256::<0x20>(x0, x1);
    *out.add(7) = _mm256_permute2f128_si256::<0x31>(x0, x1);
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn transpose_8x8_flip_avx2(mut in_: *const __m256i, mut out: *mut __m256i) {
    let mut u0: __m256i;
    let mut u1: __m256i;
    let mut u2: __m256i;
    let mut u3: __m256i;
    let mut u4: __m256i;
    let mut u5: __m256i;
    let mut u6: __m256i;
    let mut u7: __m256i;
    let mut x0: __m256i;
    let mut x1: __m256i;

    u0 = _mm256_unpacklo_epi32(*in_.add(7), *in_.add(6));
    u1 = _mm256_unpackhi_epi32(*in_.add(7), *in_.add(6));

    u2 = _mm256_unpacklo_epi32(*in_.add(5), *in_.add(4));
    u3 = _mm256_unpackhi_epi32(*in_.add(5), *in_.add(4));

    u4 = _mm256_unpacklo_epi32(*in_.add(3), *in_.add(2));
    u5 = _mm256_unpackhi_epi32(*in_.add(3), *in_.add(2));

    u6 = _mm256_unpacklo_epi32(*in_.add(1), *in_.add(0));
    u7 = _mm256_unpackhi_epi32(*in_.add(1), *in_.add(0));

    x0 = _mm256_unpacklo_epi64(u0, u2);
    x1 = _mm256_unpacklo_epi64(u4, u6);
    *out.add(0) = _mm256_permute2f128_si256::<0x20>(x0, x1);
    *out.add(4) = _mm256_permute2f128_si256::<0x31>(x0, x1);

    x0 = _mm256_unpackhi_epi64(u0, u2);
    x1 = _mm256_unpackhi_epi64(u4, u6);
    *out.add(1) = _mm256_permute2f128_si256::<0x20>(x0, x1);
    *out.add(5) = _mm256_permute2f128_si256::<0x31>(x0, x1);

    x0 = _mm256_unpacklo_epi64(u1, u3);
    x1 = _mm256_unpacklo_epi64(u5, u7);
    *out.add(2) = _mm256_permute2f128_si256::<0x20>(x0, x1);
    *out.add(6) = _mm256_permute2f128_si256::<0x31>(x0, x1);

    x0 = _mm256_unpackhi_epi64(u1, u3);
    x1 = _mm256_unpackhi_epi64(u5, u7);
    *out.add(3) = _mm256_permute2f128_si256::<0x20>(x0, x1);
    *out.add(7) = _mm256_permute2f128_si256::<0x31>(x0, x1);
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn load_buffer_32bit_input(
    mut in_: *const i32,
    mut stride: i32,
    mut out: *mut __m256i,
    mut out_size: i32,
) {
    for i in 0..out_size {
        *out.offset(i as isize) =
            _mm256_loadu_si256(in_.offset((i * stride) as isize) as *const __m256i);
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn half_btf_0_avx2(
    mut w0: *const __m256i,
    mut n0: *const __m256i,
    mut rounding: *const __m256i,
    mut bit: i32,
) -> __m256i {
    let mut x: __m256i;
    x = _mm256_mullo_epi32(*w0, *n0);
    x = _mm256_add_epi32(x, *rounding);
    x = _mm256_sra_epi32(x, _mm_cvtsi32_si128(bit));
    return x;
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn half_btf_avx2(
    mut w0: *const __m256i,
    mut n0: *const __m256i,
    mut w1: *const __m256i,
    mut n1: *const __m256i,
    mut rounding: *const __m256i,
    mut bit: i32,
) -> __m256i {
    let mut x: __m256i;
    let mut y: __m256i;

    x = _mm256_mullo_epi32(*w0, *n0);
    y = _mm256_mullo_epi32(*w1, *n1);
    x = _mm256_add_epi32(x, y);
    x = _mm256_add_epi32(x, *rounding);
    x = _mm256_sra_epi32(x, _mm_cvtsi32_si128(bit));
    return x;
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn addsub_avx2(
    mut in0: __m256i,
    mut in1: __m256i,
    mut out0: *mut __m256i,
    mut out1: *mut __m256i,
    mut clamp_lo: *const __m256i,
    mut clamp_hi: *const __m256i,
) {
    let mut a0: __m256i = _mm256_add_epi32(in0, in1);
    let mut a1: __m256i = _mm256_sub_epi32(in0, in1);

    a0 = _mm256_max_epi32(a0, *clamp_lo);
    a0 = _mm256_min_epi32(a0, *clamp_hi);
    a1 = _mm256_max_epi32(a1, *clamp_lo);
    a1 = _mm256_min_epi32(a1, *clamp_hi);

    *out0 = a0;
    *out1 = a1;
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn idct32_stage4_avx2(
    mut bf1: *mut __m256i,
    mut cospim8: *const __m256i,
    mut cospi56: *const __m256i,
    mut cospi8: *const __m256i,
    mut cospim56: *const __m256i,
    mut cospim40: *const __m256i,
    mut cospi24: *const __m256i,
    mut cospi40: *const __m256i,
    mut cospim24: *const __m256i,
    mut rounding: *const __m256i,
    mut bit: i32,
) {
    let mut temp1: __m256i;
    let mut temp2: __m256i;
    temp1 = half_btf_avx2(cospim8, bf1.add(17), cospi56, bf1.add(30), rounding, bit);
    *bf1.add(30) = half_btf_avx2(cospi56, bf1.add(17), cospi8, bf1.add(30), rounding, bit);
    *bf1.add(17) = temp1;

    temp2 = half_btf_avx2(cospim56, bf1.add(18), cospim8, bf1.add(29), rounding, bit);
    *bf1.add(29) = half_btf_avx2(cospim8, bf1.add(18), cospi56, bf1.add(29), rounding, bit);
    *bf1.add(18) = temp2;

    temp1 = half_btf_avx2(cospim40, bf1.add(21), cospi24, bf1.add(26), rounding, bit);
    *bf1.add(26) = half_btf_avx2(cospi24, bf1.add(21), cospi40, bf1.add(26), rounding, bit);
    *bf1.add(21) = temp1;

    temp2 = half_btf_avx2(cospim24, bf1.add(22), cospim40, bf1.add(25), rounding, bit);
    *bf1.add(25) = half_btf_avx2(cospim40, bf1.add(22), cospi24, bf1.add(25), rounding, bit);
    *bf1.add(22) = temp2;
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn idct32_stage5_avx2(
    mut bf1: *mut __m256i,
    mut cospim16: *const __m256i,
    mut cospi48: *const __m256i,
    mut cospi16: *const __m256i,
    mut cospim48: *const __m256i,
    mut clamp_lo: *const __m256i,
    mut clamp_hi: *const __m256i,
    mut rounding: *const __m256i,
    mut bit: i32,
) {
    let mut temp1: __m256i;
    let mut temp2: __m256i;
    temp1 = half_btf_avx2(cospim16, bf1.add(9), cospi48, bf1.add(14), rounding, bit);
    *bf1.add(14) = half_btf_avx2(cospi48, bf1.add(9), cospi16, bf1.add(14), rounding, bit);
    *bf1.add(9) = temp1;

    temp2 = half_btf_avx2(cospim48, bf1.add(10), cospim16, bf1.add(13), rounding, bit);
    *bf1.add(13) = half_btf_avx2(cospim16, bf1.add(10), cospi48, bf1.add(13), rounding, bit);
    *bf1.add(10) = temp2;

    addsub_avx2(*bf1.add(16), *bf1.add(19), bf1.add(16), bf1.add(19), clamp_lo, clamp_hi);
    addsub_avx2(*bf1.add(17), *bf1.add(18), bf1.add(17), bf1.add(18), clamp_lo, clamp_hi);
    addsub_avx2(*bf1.add(23), *bf1.add(20), bf1.add(23), bf1.add(20), clamp_lo, clamp_hi);
    addsub_avx2(*bf1.add(22), *bf1.add(21), bf1.add(22), bf1.add(21), clamp_lo, clamp_hi);
    addsub_avx2(*bf1.add(24), *bf1.add(27), bf1.add(24), bf1.add(27), clamp_lo, clamp_hi);
    addsub_avx2(*bf1.add(25), *bf1.add(26), bf1.add(25), bf1.add(26), clamp_lo, clamp_hi);
    addsub_avx2(*bf1.add(31), *bf1.add(28), bf1.add(31), bf1.add(28), clamp_lo, clamp_hi);
    addsub_avx2(*bf1.add(30), *bf1.add(29), bf1.add(30), bf1.add(29), clamp_lo, clamp_hi);
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn idct32_stage6_avx2(
    mut bf1: *mut __m256i,
    mut cospim32: *const __m256i,
    mut cospi32: *const __m256i,
    mut cospim16: *const __m256i,
    mut cospi48: *const __m256i,
    mut cospi16: *const __m256i,
    mut cospim48: *const __m256i,
    mut clamp_lo: *const __m256i,
    mut clamp_hi: *const __m256i,
    mut rounding: *const __m256i,
    mut bit: i32,
) {
    let mut temp1: __m256i;
    let mut temp2: __m256i;
    temp1 = half_btf_avx2(cospim32, bf1.add(5), cospi32, bf1.add(6), rounding, bit);
    *bf1.add(6) = half_btf_avx2(cospi32, bf1.add(5), cospi32, bf1.add(6), rounding, bit);
    *bf1.add(5) = temp1;

    addsub_avx2(*bf1.add(8), *bf1.add(11), bf1.add(8), bf1.add(11), clamp_lo, clamp_hi);
    addsub_avx2(*bf1.add(9), *bf1.add(10), bf1.add(9), bf1.add(10), clamp_lo, clamp_hi);
    addsub_avx2(*bf1.add(15), *bf1.add(12), bf1.add(15), bf1.add(12), clamp_lo, clamp_hi);
    addsub_avx2(*bf1.add(14), *bf1.add(13), bf1.add(14), bf1.add(13), clamp_lo, clamp_hi);

    temp1 = half_btf_avx2(cospim16, bf1.add(18), cospi48, bf1.add(29), rounding, bit);
    *bf1.add(29) = half_btf_avx2(cospi48, bf1.add(18), cospi16, bf1.add(29), rounding, bit);
    *bf1.add(18) = temp1;
    temp2 = half_btf_avx2(cospim16, bf1.add(19), cospi48, bf1.add(28), rounding, bit);
    *bf1.add(28) = half_btf_avx2(cospi48, bf1.add(19), cospi16, bf1.add(28), rounding, bit);
    *bf1.add(19) = temp2;
    temp1 = half_btf_avx2(cospim48, bf1.add(20), cospim16, bf1.add(27), rounding, bit);
    *bf1.add(27) = half_btf_avx2(cospim16, bf1.add(20), cospi48, bf1.add(27), rounding, bit);
    *bf1.add(20) = temp1;
    temp2 = half_btf_avx2(cospim48, bf1.add(21), cospim16, bf1.add(26), rounding, bit);
    *bf1.add(26) = half_btf_avx2(cospim16, bf1.add(21), cospi48, bf1.add(26), rounding, bit);
    *bf1.add(21) = temp2;
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn idct32_stage7_avx2(
    mut bf1: *mut __m256i,
    mut cospim32: *const __m256i,
    mut cospi32: *const __m256i,
    mut clamp_lo: *const __m256i,
    mut clamp_hi: *const __m256i,
    mut rounding: *const __m256i,
    mut bit: i32,
) {
    let mut temp1: __m256i;
    let mut temp2: __m256i;
    addsub_avx2(*bf1.add(0), *bf1.add(7), bf1.add(0), bf1.add(7), clamp_lo, clamp_hi);
    addsub_avx2(*bf1.add(1), *bf1.add(6), bf1.add(1), bf1.add(6), clamp_lo, clamp_hi);
    addsub_avx2(*bf1.add(2), *bf1.add(5), bf1.add(2), bf1.add(5), clamp_lo, clamp_hi);
    addsub_avx2(*bf1.add(3), *bf1.add(4), bf1.add(3), bf1.add(4), clamp_lo, clamp_hi);

    temp1 = half_btf_avx2(cospim32, bf1.add(10), cospi32, bf1.add(13), rounding, bit);
    *bf1.add(13) = half_btf_avx2(cospi32, bf1.add(10), cospi32, bf1.add(13), rounding, bit);
    *bf1.add(10) = temp1;
    temp2 = half_btf_avx2(cospim32, bf1.add(11), cospi32, bf1.add(12), rounding, bit);
    *bf1.add(12) = half_btf_avx2(cospi32, bf1.add(11), cospi32, bf1.add(12), rounding, bit);
    *bf1.add(11) = temp2;

    addsub_avx2(*bf1.add(16), *bf1.add(23), bf1.add(16), bf1.add(23), clamp_lo, clamp_hi);
    addsub_avx2(*bf1.add(17), *bf1.add(22), bf1.add(17), bf1.add(22), clamp_lo, clamp_hi);
    addsub_avx2(*bf1.add(18), *bf1.add(21), bf1.add(18), bf1.add(21), clamp_lo, clamp_hi);
    addsub_avx2(*bf1.add(19), *bf1.add(20), bf1.add(19), bf1.add(20), clamp_lo, clamp_hi);
    addsub_avx2(*bf1.add(31), *bf1.add(24), bf1.add(31), bf1.add(24), clamp_lo, clamp_hi);
    addsub_avx2(*bf1.add(30), *bf1.add(25), bf1.add(30), bf1.add(25), clamp_lo, clamp_hi);
    addsub_avx2(*bf1.add(29), *bf1.add(26), bf1.add(29), bf1.add(26), clamp_lo, clamp_hi);
    addsub_avx2(*bf1.add(28), *bf1.add(27), bf1.add(28), bf1.add(27), clamp_lo, clamp_hi);
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn idct32_stage8_avx2(
    mut bf1: *mut __m256i,
    mut cospim32: *const __m256i,
    mut cospi32: *const __m256i,
    mut clamp_lo: *const __m256i,
    mut clamp_hi: *const __m256i,
    mut rounding: *const __m256i,
    mut bit: i32,
) {
    let mut temp1: __m256i;
    let mut temp2: __m256i;
    addsub_avx2(*bf1.add(0), *bf1.add(15), bf1.add(0), bf1.add(15), clamp_lo, clamp_hi);
    addsub_avx2(*bf1.add(1), *bf1.add(14), bf1.add(1), bf1.add(14), clamp_lo, clamp_hi);
    addsub_avx2(*bf1.add(2), *bf1.add(13), bf1.add(2), bf1.add(13), clamp_lo, clamp_hi);
    addsub_avx2(*bf1.add(3), *bf1.add(12), bf1.add(3), bf1.add(12), clamp_lo, clamp_hi);
    addsub_avx2(*bf1.add(4), *bf1.add(11), bf1.add(4), bf1.add(11), clamp_lo, clamp_hi);
    addsub_avx2(*bf1.add(5), *bf1.add(10), bf1.add(5), bf1.add(10), clamp_lo, clamp_hi);
    addsub_avx2(*bf1.add(6), *bf1.add(9), bf1.add(6), bf1.add(9), clamp_lo, clamp_hi);
    addsub_avx2(*bf1.add(7), *bf1.add(8), bf1.add(7), bf1.add(8), clamp_lo, clamp_hi);

    temp1 = half_btf_avx2(cospim32, bf1.add(20), cospi32, bf1.add(27), rounding, bit);
    *bf1.add(27) = half_btf_avx2(cospi32, bf1.add(20), cospi32, bf1.add(27), rounding, bit);
    *bf1.add(20) = temp1;
    temp2 = half_btf_avx2(cospim32, bf1.add(21), cospi32, bf1.add(26), rounding, bit);
    *bf1.add(26) = half_btf_avx2(cospi32, bf1.add(21), cospi32, bf1.add(26), rounding, bit);
    *bf1.add(21) = temp2;
    temp1 = half_btf_avx2(cospim32, bf1.add(22), cospi32, bf1.add(25), rounding, bit);
    *bf1.add(25) = half_btf_avx2(cospi32, bf1.add(22), cospi32, bf1.add(25), rounding, bit);
    *bf1.add(22) = temp1;
    temp2 = half_btf_avx2(cospim32, bf1.add(23), cospi32, bf1.add(24), rounding, bit);
    *bf1.add(24) = half_btf_avx2(cospi32, bf1.add(23), cospi32, bf1.add(24), rounding, bit);
    *bf1.add(23) = temp2;
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn idct32_stage9_avx2(
    mut bf1: *mut __m256i,
    mut out: *mut __m256i,
    mut do_cols: i32,
    mut bd: i32,
    mut out_shift: i32,
    mut clamp_lo: *const __m256i,
    mut clamp_hi: *const __m256i,
) {
    addsub_avx2(*bf1.add(0), *bf1.add(31), out.add(0), out.add(31), clamp_lo, clamp_hi);
    addsub_avx2(*bf1.add(1), *bf1.add(30), out.add(1), out.add(30), clamp_lo, clamp_hi);
    addsub_avx2(*bf1.add(2), *bf1.add(29), out.add(2), out.add(29), clamp_lo, clamp_hi);
    addsub_avx2(*bf1.add(3), *bf1.add(28), out.add(3), out.add(28), clamp_lo, clamp_hi);
    addsub_avx2(*bf1.add(4), *bf1.add(27), out.add(4), out.add(27), clamp_lo, clamp_hi);
    addsub_avx2(*bf1.add(5), *bf1.add(26), out.add(5), out.add(26), clamp_lo, clamp_hi);
    addsub_avx2(*bf1.add(6), *bf1.add(25), out.add(6), out.add(25), clamp_lo, clamp_hi);
    addsub_avx2(*bf1.add(7), *bf1.add(24), out.add(7), out.add(24), clamp_lo, clamp_hi);
    addsub_avx2(*bf1.add(8), *bf1.add(23), out.add(8), out.add(23), clamp_lo, clamp_hi);
    addsub_avx2(*bf1.add(9), *bf1.add(22), out.add(9), out.add(22), clamp_lo, clamp_hi);
    addsub_avx2(*bf1.add(10), *bf1.add(21), out.add(10), out.add(21), clamp_lo, clamp_hi);
    addsub_avx2(*bf1.add(11), *bf1.add(20), out.add(11), out.add(20), clamp_lo, clamp_hi);
    addsub_avx2(*bf1.add(12), *bf1.add(19), out.add(12), out.add(19), clamp_lo, clamp_hi);
    addsub_avx2(*bf1.add(13), *bf1.add(18), out.add(13), out.add(18), clamp_lo, clamp_hi);
    addsub_avx2(*bf1.add(14), *bf1.add(17), out.add(14), out.add(17), clamp_lo, clamp_hi);
    addsub_avx2(*bf1.add(15), *bf1.add(16), out.add(15), out.add(16), clamp_lo, clamp_hi);
    if do_cols == 0 {
        let mut log_range_out: i32 = get_log_range_out(bd);
        let mut clamp_lo_out: __m256i = _mm256_set1_epi32(-(1 << (log_range_out - 1)));
        let mut clamp_hi_out: __m256i = _mm256_set1_epi32((1 << (log_range_out - 1)) - 1);
        round_shift_8x8_avx2(out, out_shift);
        round_shift_8x8_avx2(out.add(16), out_shift);
        highbd_clamp_epi32_avx2(out, out, &clamp_lo_out, &clamp_hi_out, 32);
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn idct32_low1_avx2(
    mut in_: *mut __m256i,
    mut out: *mut __m256i,
    mut bit: i32,
    mut do_cols: i32,
    mut bd: i32,
    mut out_shift: i32,
) {
    let mut cospi: *const i32 = cospi_arr(bit);
    let mut cospi32: __m256i = _mm256_set1_epi32(*cospi.add(32));
    let mut rounding: __m256i = _mm256_set1_epi32(1 << (bit - 1));
    let mut log_range: i32 = get_log_range(bd, do_cols);
    let mut clamp_lo: __m256i = _mm256_set1_epi32(-(1 << (log_range - 1)));
    let mut clamp_hi: __m256i = _mm256_set1_epi32((1 << (log_range - 1)) - 1);
    let mut x: __m256i;
    // stage 0
    // stage 1
    // stage 2
    // stage 3
    // stage 4
    // stage 5
    x = _mm256_mullo_epi32(*in_.add(0), cospi32);
    x = _mm256_add_epi32(x, rounding);
    x = _mm256_sra_epi32(x, _mm_cvtsi32_si128(bit));

    // stage 6
    // stage 7
    // stage 8
    // stage 9
    if do_cols == 0 {
        let mut log_range_out: i32 = get_log_range_out(bd);
        let mut offset: __m256i = _mm256_set1_epi32((1 << out_shift) >> 1);
        clamp_lo = _mm256_set1_epi32(-(1 << (log_range_out - 1)));
        clamp_hi = _mm256_set1_epi32((1 << (log_range_out - 1)) - 1);
        x = _mm256_add_epi32(offset, x);
        x = _mm256_sra_epi32(x, _mm_cvtsi32_si128(out_shift));
    }
    x = _mm256_max_epi32(x, clamp_lo);
    x = _mm256_min_epi32(x, clamp_hi);
    *out.add(0) = x;
    *out.add(1) = x;
    *out.add(2) = x;
    *out.add(3) = x;
    *out.add(4) = x;
    *out.add(5) = x;
    *out.add(6) = x;
    *out.add(7) = x;
    *out.add(8) = x;
    *out.add(9) = x;
    *out.add(10) = x;
    *out.add(11) = x;
    *out.add(12) = x;
    *out.add(13) = x;
    *out.add(14) = x;
    *out.add(15) = x;
    *out.add(16) = x;
    *out.add(17) = x;
    *out.add(18) = x;
    *out.add(19) = x;
    *out.add(20) = x;
    *out.add(21) = x;
    *out.add(22) = x;
    *out.add(23) = x;
    *out.add(24) = x;
    *out.add(25) = x;
    *out.add(26) = x;
    *out.add(27) = x;
    *out.add(28) = x;
    *out.add(29) = x;
    *out.add(30) = x;
    *out.add(31) = x;
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn idct32_low8_avx2(
    mut in_: *mut __m256i,
    mut out: *mut __m256i,
    mut bit: i32,
    mut do_cols: i32,
    mut bd: i32,
    mut out_shift: i32,
) {
    let mut cospi: *const i32 = cospi_arr(bit);
    let mut cospi62: __m256i = _mm256_set1_epi32(*cospi.add(62));
    let mut cospi14: __m256i = _mm256_set1_epi32(*cospi.add(14));
    let mut cospi54: __m256i = _mm256_set1_epi32(*cospi.add(54));
    let mut cospi6: __m256i = _mm256_set1_epi32(*cospi.add(6));
    let mut cospi10: __m256i = _mm256_set1_epi32(*cospi.add(10));
    let mut cospi2: __m256i = _mm256_set1_epi32(*cospi.add(2));
    let mut cospim58: __m256i = _mm256_set1_epi32(-*cospi.add(58));
    let mut cospim50: __m256i = _mm256_set1_epi32(-*cospi.add(50));
    let mut cospi60: __m256i = _mm256_set1_epi32(*cospi.add(60));
    let mut cospi12: __m256i = _mm256_set1_epi32(*cospi.add(12));
    let mut cospi4: __m256i = _mm256_set1_epi32(*cospi.add(4));
    let mut cospim52: __m256i = _mm256_set1_epi32(-*cospi.add(52));
    let mut cospi56: __m256i = _mm256_set1_epi32(*cospi.add(56));
    let mut cospi24: __m256i = _mm256_set1_epi32(*cospi.add(24));
    let mut cospi40: __m256i = _mm256_set1_epi32(*cospi.add(40));
    let mut cospi8: __m256i = _mm256_set1_epi32(*cospi.add(8));
    let mut cospim40: __m256i = _mm256_set1_epi32(-*cospi.add(40));
    let mut cospim8: __m256i = _mm256_set1_epi32(-*cospi.add(8));
    let mut cospim56: __m256i = _mm256_set1_epi32(-*cospi.add(56));
    let mut cospim24: __m256i = _mm256_set1_epi32(-*cospi.add(24));
    let mut cospi32: __m256i = _mm256_set1_epi32(*cospi.add(32));
    let mut cospim32: __m256i = _mm256_set1_epi32(-*cospi.add(32));
    let mut cospi48: __m256i = _mm256_set1_epi32(*cospi.add(48));
    let mut cospim48: __m256i = _mm256_set1_epi32(-*cospi.add(48));
    let mut cospi16: __m256i = _mm256_set1_epi32(*cospi.add(16));
    let mut cospim16: __m256i = _mm256_set1_epi32(-*cospi.add(16));
    let mut rounding: __m256i = _mm256_set1_epi32(1 << (bit - 1));
    let mut log_range: i32 = get_log_range(bd, do_cols);
    let mut clamp_lo: __m256i = _mm256_set1_epi32(-(1 << (log_range - 1)));
    let mut clamp_hi: __m256i = _mm256_set1_epi32((1 << (log_range - 1)) - 1);
    let mut bf1: [__m256i; 32] = core::mem::zeroed();

    {
        // stage 0
        // stage 1
        bf1[0] = *in_.add(0);
        bf1[4] = *in_.add(4);
        bf1[8] = *in_.add(2);
        bf1[12] = *in_.add(6);
        bf1[16] = *in_.add(1);
        bf1[20] = *in_.add(5);
        bf1[24] = *in_.add(3);
        bf1[28] = *in_.add(7);

        // stage 2
        bf1[31] = half_btf_0_avx2(&cospi2, bf1.as_mut_ptr().add(16), &rounding, bit);
        bf1[16] = half_btf_0_avx2(&cospi62, bf1.as_mut_ptr().add(16), &rounding, bit);
        bf1[19] = half_btf_0_avx2(&cospim50, bf1.as_mut_ptr().add(28), &rounding, bit);
        bf1[28] = half_btf_0_avx2(&cospi14, bf1.as_mut_ptr().add(28), &rounding, bit);
        bf1[27] = half_btf_0_avx2(&cospi10, bf1.as_mut_ptr().add(20), &rounding, bit);
        bf1[20] = half_btf_0_avx2(&cospi54, bf1.as_mut_ptr().add(20), &rounding, bit);
        bf1[23] = half_btf_0_avx2(&cospim58, bf1.as_mut_ptr().add(24), &rounding, bit);
        bf1[24] = half_btf_0_avx2(&cospi6, bf1.as_mut_ptr().add(24), &rounding, bit);

        // stage 3
        bf1[15] = half_btf_0_avx2(&cospi4, bf1.as_mut_ptr().add(8), &rounding, bit);
        bf1[8] = half_btf_0_avx2(&cospi60, bf1.as_mut_ptr().add(8), &rounding, bit);

        bf1[11] = half_btf_0_avx2(&cospim52, bf1.as_mut_ptr().add(12), &rounding, bit);
        bf1[12] = half_btf_0_avx2(&cospi12, bf1.as_mut_ptr().add(12), &rounding, bit);
        bf1[17] = bf1[16];
        bf1[18] = bf1[19];
        bf1[21] = bf1[20];
        bf1[22] = bf1[23];
        bf1[25] = bf1[24];
        bf1[26] = bf1[27];
        bf1[29] = bf1[28];
        bf1[30] = bf1[31];

        // stage 4
        bf1[7] = half_btf_0_avx2(&cospi8, bf1.as_mut_ptr().add(4), &rounding, bit);
        bf1[4] = half_btf_0_avx2(&cospi56, bf1.as_mut_ptr().add(4), &rounding, bit);

        bf1[9] = bf1[8];
        bf1[10] = bf1[11];
        bf1[13] = bf1[12];
        bf1[14] = bf1[15];

        idct32_stage4_avx2(
            bf1.as_mut_ptr(),
            &cospim8,
            &cospi56,
            &cospi8,
            &cospim56,
            &cospim40,
            &cospi24,
            &cospi40,
            &cospim24,
            &rounding,
            bit,
        );

        // stage 5
        bf1[0] = half_btf_0_avx2(&cospi32, bf1.as_mut_ptr().add(0), &rounding, bit);
        bf1[1] = bf1[0];
        bf1[5] = bf1[4];
        bf1[6] = bf1[7];

        idct32_stage5_avx2(
            bf1.as_mut_ptr(),
            &cospim16,
            &cospi48,
            &cospi16,
            &cospim48,
            &clamp_lo,
            &clamp_hi,
            &rounding,
            bit,
        );

        // stage 6
        bf1[3] = bf1[0];
        bf1[2] = bf1[1];

        idct32_stage6_avx2(
            bf1.as_mut_ptr(),
            &cospim32,
            &cospi32,
            &cospim16,
            &cospi48,
            &cospi16,
            &cospim48,
            &clamp_lo,
            &clamp_hi,
            &rounding,
            bit,
        );

        // stage 7
        idct32_stage7_avx2(
            bf1.as_mut_ptr(),
            &cospim32,
            &cospi32,
            &clamp_lo,
            &clamp_hi,
            &rounding,
            bit,
        );

        // stage 8
        idct32_stage8_avx2(
            bf1.as_mut_ptr(),
            &cospim32,
            &cospi32,
            &clamp_lo,
            &clamp_hi,
            &rounding,
            bit,
        );

        // stage 9
        idct32_stage9_avx2(bf1.as_mut_ptr(), out, do_cols, bd, out_shift, &clamp_lo, &clamp_hi);
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn idct32_low16_avx2(
    mut in_: *mut __m256i,
    mut out: *mut __m256i,
    mut bit: i32,
    mut do_cols: i32,
    mut bd: i32,
    mut out_shift: i32,
) {
    let mut cospi: *const i32 = cospi_arr(bit);
    let mut cospi62: __m256i = _mm256_set1_epi32(*cospi.add(62));
    let mut cospi30: __m256i = _mm256_set1_epi32(*cospi.add(30));
    let mut cospi46: __m256i = _mm256_set1_epi32(*cospi.add(46));
    let mut cospi14: __m256i = _mm256_set1_epi32(*cospi.add(14));
    let mut cospi54: __m256i = _mm256_set1_epi32(*cospi.add(54));
    let mut cospi22: __m256i = _mm256_set1_epi32(*cospi.add(22));
    let mut cospi38: __m256i = _mm256_set1_epi32(*cospi.add(38));
    let mut cospi6: __m256i = _mm256_set1_epi32(*cospi.add(6));
    let mut cospi26: __m256i = _mm256_set1_epi32(*cospi.add(26));
    let mut cospi10: __m256i = _mm256_set1_epi32(*cospi.add(10));
    let mut cospi18: __m256i = _mm256_set1_epi32(*cospi.add(18));
    let mut cospi2: __m256i = _mm256_set1_epi32(*cospi.add(2));
    let mut cospim58: __m256i = _mm256_set1_epi32(-*cospi.add(58));
    let mut cospim42: __m256i = _mm256_set1_epi32(-*cospi.add(42));
    let mut cospim50: __m256i = _mm256_set1_epi32(-*cospi.add(50));
    let mut cospim34: __m256i = _mm256_set1_epi32(-*cospi.add(34));
    let mut cospi60: __m256i = _mm256_set1_epi32(*cospi.add(60));
    let mut cospi28: __m256i = _mm256_set1_epi32(*cospi.add(28));
    let mut cospi44: __m256i = _mm256_set1_epi32(*cospi.add(44));
    let mut cospi12: __m256i = _mm256_set1_epi32(*cospi.add(12));
    let mut cospi20: __m256i = _mm256_set1_epi32(*cospi.add(20));
    let mut cospi4: __m256i = _mm256_set1_epi32(*cospi.add(4));
    let mut cospim52: __m256i = _mm256_set1_epi32(-*cospi.add(52));
    let mut cospim36: __m256i = _mm256_set1_epi32(-*cospi.add(36));
    let mut cospi56: __m256i = _mm256_set1_epi32(*cospi.add(56));
    let mut cospi24: __m256i = _mm256_set1_epi32(*cospi.add(24));
    let mut cospi40: __m256i = _mm256_set1_epi32(*cospi.add(40));
    let mut cospi8: __m256i = _mm256_set1_epi32(*cospi.add(8));
    let mut cospim40: __m256i = _mm256_set1_epi32(-*cospi.add(40));
    let mut cospim8: __m256i = _mm256_set1_epi32(-*cospi.add(8));
    let mut cospim56: __m256i = _mm256_set1_epi32(-*cospi.add(56));
    let mut cospim24: __m256i = _mm256_set1_epi32(-*cospi.add(24));
    let mut cospi32: __m256i = _mm256_set1_epi32(*cospi.add(32));
    let mut cospim32: __m256i = _mm256_set1_epi32(-*cospi.add(32));
    let mut cospi48: __m256i = _mm256_set1_epi32(*cospi.add(48));
    let mut cospim48: __m256i = _mm256_set1_epi32(-*cospi.add(48));
    let mut cospi16: __m256i = _mm256_set1_epi32(*cospi.add(16));
    let mut cospim16: __m256i = _mm256_set1_epi32(-*cospi.add(16));
    let mut rounding: __m256i = _mm256_set1_epi32(1 << (bit - 1));
    let mut log_range: i32 = get_log_range(bd, do_cols);
    let mut clamp_lo: __m256i = _mm256_set1_epi32(-(1 << (log_range - 1)));
    let mut clamp_hi: __m256i = _mm256_set1_epi32((1 << (log_range - 1)) - 1);
    let mut bf1: [__m256i; 32] = core::mem::zeroed();

    {
        // stage 0
        // stage 1
        bf1[0] = *in_.add(0);
        bf1[2] = *in_.add(8);
        bf1[4] = *in_.add(4);
        bf1[6] = *in_.add(12);
        bf1[8] = *in_.add(2);
        bf1[10] = *in_.add(10);
        bf1[12] = *in_.add(6);
        bf1[14] = *in_.add(14);
        bf1[16] = *in_.add(1);
        bf1[18] = *in_.add(9);
        bf1[20] = *in_.add(5);
        bf1[22] = *in_.add(13);
        bf1[24] = *in_.add(3);
        bf1[26] = *in_.add(11);
        bf1[28] = *in_.add(7);
        bf1[30] = *in_.add(15);

        // stage 2
        bf1[31] = half_btf_0_avx2(&cospi2, bf1.as_mut_ptr().add(16), &rounding, bit);
        bf1[16] = half_btf_0_avx2(&cospi62, bf1.as_mut_ptr().add(16), &rounding, bit);
        bf1[17] = half_btf_0_avx2(&cospim34, bf1.as_mut_ptr().add(30), &rounding, bit);
        bf1[30] = half_btf_0_avx2(&cospi30, bf1.as_mut_ptr().add(30), &rounding, bit);
        bf1[29] = half_btf_0_avx2(&cospi18, bf1.as_mut_ptr().add(18), &rounding, bit);
        bf1[18] = half_btf_0_avx2(&cospi46, bf1.as_mut_ptr().add(18), &rounding, bit);
        bf1[19] = half_btf_0_avx2(&cospim50, bf1.as_mut_ptr().add(28), &rounding, bit);
        bf1[28] = half_btf_0_avx2(&cospi14, bf1.as_mut_ptr().add(28), &rounding, bit);
        bf1[27] = half_btf_0_avx2(&cospi10, bf1.as_mut_ptr().add(20), &rounding, bit);
        bf1[20] = half_btf_0_avx2(&cospi54, bf1.as_mut_ptr().add(20), &rounding, bit);
        bf1[21] = half_btf_0_avx2(&cospim42, bf1.as_mut_ptr().add(26), &rounding, bit);
        bf1[26] = half_btf_0_avx2(&cospi22, bf1.as_mut_ptr().add(26), &rounding, bit);
        bf1[25] = half_btf_0_avx2(&cospi26, bf1.as_mut_ptr().add(22), &rounding, bit);
        bf1[22] = half_btf_0_avx2(&cospi38, bf1.as_mut_ptr().add(22), &rounding, bit);
        bf1[23] = half_btf_0_avx2(&cospim58, bf1.as_mut_ptr().add(24), &rounding, bit);
        bf1[24] = half_btf_0_avx2(&cospi6, bf1.as_mut_ptr().add(24), &rounding, bit);

        // stage 3
        bf1[15] = half_btf_0_avx2(&cospi4, bf1.as_mut_ptr().add(8), &rounding, bit);
        bf1[8] = half_btf_0_avx2(&cospi60, bf1.as_mut_ptr().add(8), &rounding, bit);
        bf1[9] = half_btf_0_avx2(&cospim36, bf1.as_mut_ptr().add(14), &rounding, bit);
        bf1[14] = half_btf_0_avx2(&cospi28, bf1.as_mut_ptr().add(14), &rounding, bit);
        bf1[13] = half_btf_0_avx2(&cospi20, bf1.as_mut_ptr().add(10), &rounding, bit);
        bf1[10] = half_btf_0_avx2(&cospi44, bf1.as_mut_ptr().add(10), &rounding, bit);
        bf1[11] = half_btf_0_avx2(&cospim52, bf1.as_mut_ptr().add(12), &rounding, bit);
        bf1[12] = half_btf_0_avx2(&cospi12, bf1.as_mut_ptr().add(12), &rounding, bit);

        addsub_avx2(
            bf1[16],
            bf1[17],
            bf1.as_mut_ptr().add(16),
            bf1.as_mut_ptr().add(17),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            bf1[19],
            bf1[18],
            bf1.as_mut_ptr().add(19),
            bf1.as_mut_ptr().add(18),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            bf1[20],
            bf1[21],
            bf1.as_mut_ptr().add(20),
            bf1.as_mut_ptr().add(21),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            bf1[23],
            bf1[22],
            bf1.as_mut_ptr().add(23),
            bf1.as_mut_ptr().add(22),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            bf1[24],
            bf1[25],
            bf1.as_mut_ptr().add(24),
            bf1.as_mut_ptr().add(25),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            bf1[27],
            bf1[26],
            bf1.as_mut_ptr().add(27),
            bf1.as_mut_ptr().add(26),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            bf1[28],
            bf1[29],
            bf1.as_mut_ptr().add(28),
            bf1.as_mut_ptr().add(29),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            bf1[31],
            bf1[30],
            bf1.as_mut_ptr().add(31),
            bf1.as_mut_ptr().add(30),
            &clamp_lo,
            &clamp_hi,
        );

        // stage 4
        bf1[7] = half_btf_0_avx2(&cospi8, bf1.as_mut_ptr().add(4), &rounding, bit);
        bf1[4] = half_btf_0_avx2(&cospi56, bf1.as_mut_ptr().add(4), &rounding, bit);
        bf1[5] = half_btf_0_avx2(&cospim40, bf1.as_mut_ptr().add(6), &rounding, bit);
        bf1[6] = half_btf_0_avx2(&cospi24, bf1.as_mut_ptr().add(6), &rounding, bit);

        addsub_avx2(
            bf1[8],
            bf1[9],
            bf1.as_mut_ptr().add(8),
            bf1.as_mut_ptr().add(9),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            bf1[11],
            bf1[10],
            bf1.as_mut_ptr().add(11),
            bf1.as_mut_ptr().add(10),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            bf1[12],
            bf1[13],
            bf1.as_mut_ptr().add(12),
            bf1.as_mut_ptr().add(13),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            bf1[15],
            bf1[14],
            bf1.as_mut_ptr().add(15),
            bf1.as_mut_ptr().add(14),
            &clamp_lo,
            &clamp_hi,
        );

        idct32_stage4_avx2(
            bf1.as_mut_ptr(),
            &cospim8,
            &cospi56,
            &cospi8,
            &cospim56,
            &cospim40,
            &cospi24,
            &cospi40,
            &cospim24,
            &rounding,
            bit,
        );

        // stage 5
        bf1[0] = half_btf_0_avx2(&cospi32, bf1.as_mut_ptr().add(0), &rounding, bit);
        bf1[1] = bf1[0];
        bf1[3] = half_btf_0_avx2(&cospi16, bf1.as_mut_ptr().add(2), &rounding, bit);
        bf1[2] = half_btf_0_avx2(&cospi48, bf1.as_mut_ptr().add(2), &rounding, bit);

        addsub_avx2(
            bf1[4],
            bf1[5],
            bf1.as_mut_ptr().add(4),
            bf1.as_mut_ptr().add(5),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            bf1[7],
            bf1[6],
            bf1.as_mut_ptr().add(7),
            bf1.as_mut_ptr().add(6),
            &clamp_lo,
            &clamp_hi,
        );

        idct32_stage5_avx2(
            bf1.as_mut_ptr(),
            &cospim16,
            &cospi48,
            &cospi16,
            &cospim48,
            &clamp_lo,
            &clamp_hi,
            &rounding,
            bit,
        );

        // stage 6
        addsub_avx2(
            bf1[0],
            bf1[3],
            bf1.as_mut_ptr().add(0),
            bf1.as_mut_ptr().add(3),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            bf1[1],
            bf1[2],
            bf1.as_mut_ptr().add(1),
            bf1.as_mut_ptr().add(2),
            &clamp_lo,
            &clamp_hi,
        );

        idct32_stage6_avx2(
            bf1.as_mut_ptr(),
            &cospim32,
            &cospi32,
            &cospim16,
            &cospi48,
            &cospi16,
            &cospim48,
            &clamp_lo,
            &clamp_hi,
            &rounding,
            bit,
        );

        // stage 7
        idct32_stage7_avx2(
            bf1.as_mut_ptr(),
            &cospim32,
            &cospi32,
            &clamp_lo,
            &clamp_hi,
            &rounding,
            bit,
        );

        // stage 8
        idct32_stage8_avx2(
            bf1.as_mut_ptr(),
            &cospim32,
            &cospi32,
            &clamp_lo,
            &clamp_hi,
            &rounding,
            bit,
        );

        // stage 9
        idct32_stage9_avx2(bf1.as_mut_ptr(), out, do_cols, bd, out_shift, &clamp_lo, &clamp_hi);
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn idct32_avx2(
    mut in_: *mut __m256i,
    mut out: *mut __m256i,
    mut bit: i32,
    mut do_cols: i32,
    mut bd: i32,
    mut out_shift: i32,
) {
    let mut cospi: *const i32 = cospi_arr(bit);
    let mut cospi62: __m256i = _mm256_set1_epi32(*cospi.add(62));
    let mut cospi30: __m256i = _mm256_set1_epi32(*cospi.add(30));
    let mut cospi46: __m256i = _mm256_set1_epi32(*cospi.add(46));
    let mut cospi14: __m256i = _mm256_set1_epi32(*cospi.add(14));
    let mut cospi54: __m256i = _mm256_set1_epi32(*cospi.add(54));
    let mut cospi22: __m256i = _mm256_set1_epi32(*cospi.add(22));
    let mut cospi38: __m256i = _mm256_set1_epi32(*cospi.add(38));
    let mut cospi6: __m256i = _mm256_set1_epi32(*cospi.add(6));
    let mut cospi58: __m256i = _mm256_set1_epi32(*cospi.add(58));
    let mut cospi26: __m256i = _mm256_set1_epi32(*cospi.add(26));
    let mut cospi42: __m256i = _mm256_set1_epi32(*cospi.add(42));
    let mut cospi10: __m256i = _mm256_set1_epi32(*cospi.add(10));
    let mut cospi50: __m256i = _mm256_set1_epi32(*cospi.add(50));
    let mut cospi18: __m256i = _mm256_set1_epi32(*cospi.add(18));
    let mut cospi34: __m256i = _mm256_set1_epi32(*cospi.add(34));
    let mut cospi2: __m256i = _mm256_set1_epi32(*cospi.add(2));
    let mut cospim58: __m256i = _mm256_set1_epi32(-*cospi.add(58));
    let mut cospim26: __m256i = _mm256_set1_epi32(-*cospi.add(26));
    let mut cospim42: __m256i = _mm256_set1_epi32(-*cospi.add(42));
    let mut cospim10: __m256i = _mm256_set1_epi32(-*cospi.add(10));
    let mut cospim50: __m256i = _mm256_set1_epi32(-*cospi.add(50));
    let mut cospim18: __m256i = _mm256_set1_epi32(-*cospi.add(18));
    let mut cospim34: __m256i = _mm256_set1_epi32(-*cospi.add(34));
    let mut cospim2: __m256i = _mm256_set1_epi32(-*cospi.add(2));
    let mut cospi60: __m256i = _mm256_set1_epi32(*cospi.add(60));
    let mut cospi28: __m256i = _mm256_set1_epi32(*cospi.add(28));
    let mut cospi44: __m256i = _mm256_set1_epi32(*cospi.add(44));
    let mut cospi12: __m256i = _mm256_set1_epi32(*cospi.add(12));
    let mut cospi52: __m256i = _mm256_set1_epi32(*cospi.add(52));
    let mut cospi20: __m256i = _mm256_set1_epi32(*cospi.add(20));
    let mut cospi36: __m256i = _mm256_set1_epi32(*cospi.add(36));
    let mut cospi4: __m256i = _mm256_set1_epi32(*cospi.add(4));
    let mut cospim52: __m256i = _mm256_set1_epi32(-*cospi.add(52));
    let mut cospim20: __m256i = _mm256_set1_epi32(-*cospi.add(20));
    let mut cospim36: __m256i = _mm256_set1_epi32(-*cospi.add(36));
    let mut cospim4: __m256i = _mm256_set1_epi32(-*cospi.add(4));
    let mut cospi56: __m256i = _mm256_set1_epi32(*cospi.add(56));
    let mut cospi24: __m256i = _mm256_set1_epi32(*cospi.add(24));
    let mut cospi40: __m256i = _mm256_set1_epi32(*cospi.add(40));
    let mut cospi8: __m256i = _mm256_set1_epi32(*cospi.add(8));
    let mut cospim40: __m256i = _mm256_set1_epi32(-*cospi.add(40));
    let mut cospim8: __m256i = _mm256_set1_epi32(-*cospi.add(8));
    let mut cospim56: __m256i = _mm256_set1_epi32(-*cospi.add(56));
    let mut cospim24: __m256i = _mm256_set1_epi32(-*cospi.add(24));
    let mut cospi32: __m256i = _mm256_set1_epi32(*cospi.add(32));
    let mut cospim32: __m256i = _mm256_set1_epi32(-*cospi.add(32));
    let mut cospi48: __m256i = _mm256_set1_epi32(*cospi.add(48));
    let mut cospim48: __m256i = _mm256_set1_epi32(-*cospi.add(48));
    let mut cospi16: __m256i = _mm256_set1_epi32(*cospi.add(16));
    let mut cospim16: __m256i = _mm256_set1_epi32(-*cospi.add(16));
    let mut rounding: __m256i = _mm256_set1_epi32(1 << (bit - 1));
    let mut log_range: i32 = get_log_range(bd, do_cols);
    let mut clamp_lo: __m256i = _mm256_set1_epi32(-(1 << (log_range - 1)));
    let mut clamp_hi: __m256i = _mm256_set1_epi32((1 << (log_range - 1)) - 1);
    let mut bf1: [__m256i; 32] = core::mem::zeroed();
    let mut bf0: [__m256i; 32] = core::mem::zeroed();

    {
        // stage 0
        // stage 1
        bf1[0] = *in_.add(0);
        bf1[1] = *in_.add(16);
        bf1[2] = *in_.add(8);
        bf1[3] = *in_.add(24);
        bf1[4] = *in_.add(4);
        bf1[5] = *in_.add(20);
        bf1[6] = *in_.add(12);
        bf1[7] = *in_.add(28);
        bf1[8] = *in_.add(2);
        bf1[9] = *in_.add(18);
        bf1[10] = *in_.add(10);
        bf1[11] = *in_.add(26);
        bf1[12] = *in_.add(6);
        bf1[13] = *in_.add(22);
        bf1[14] = *in_.add(14);
        bf1[15] = *in_.add(30);
        bf1[16] = *in_.add(1);
        bf1[17] = *in_.add(17);
        bf1[18] = *in_.add(9);
        bf1[19] = *in_.add(25);
        bf1[20] = *in_.add(5);
        bf1[21] = *in_.add(21);
        bf1[22] = *in_.add(13);
        bf1[23] = *in_.add(29);
        bf1[24] = *in_.add(3);
        bf1[25] = *in_.add(19);
        bf1[26] = *in_.add(11);
        bf1[27] = *in_.add(27);
        bf1[28] = *in_.add(7);
        bf1[29] = *in_.add(23);
        bf1[30] = *in_.add(15);
        bf1[31] = *in_.add(31);

        // stage 2
        bf0[0] = bf1[0];
        bf0[1] = bf1[1];
        bf0[2] = bf1[2];
        bf0[3] = bf1[3];
        bf0[4] = bf1[4];
        bf0[5] = bf1[5];
        bf0[6] = bf1[6];
        bf0[7] = bf1[7];
        bf0[8] = bf1[8];
        bf0[9] = bf1[9];
        bf0[10] = bf1[10];
        bf0[11] = bf1[11];
        bf0[12] = bf1[12];
        bf0[13] = bf1[13];
        bf0[14] = bf1[14];
        bf0[15] = bf1[15];
        bf0[16] = half_btf_avx2(
            &cospi62,
            bf1.as_mut_ptr().add(16),
            &cospim2,
            bf1.as_mut_ptr().add(31),
            &rounding,
            bit,
        );
        bf0[17] = half_btf_avx2(
            &cospi30,
            bf1.as_mut_ptr().add(17),
            &cospim34,
            bf1.as_mut_ptr().add(30),
            &rounding,
            bit,
        );
        bf0[18] = half_btf_avx2(
            &cospi46,
            bf1.as_mut_ptr().add(18),
            &cospim18,
            bf1.as_mut_ptr().add(29),
            &rounding,
            bit,
        );
        bf0[19] = half_btf_avx2(
            &cospi14,
            bf1.as_mut_ptr().add(19),
            &cospim50,
            bf1.as_mut_ptr().add(28),
            &rounding,
            bit,
        );
        bf0[20] = half_btf_avx2(
            &cospi54,
            bf1.as_mut_ptr().add(20),
            &cospim10,
            bf1.as_mut_ptr().add(27),
            &rounding,
            bit,
        );
        bf0[21] = half_btf_avx2(
            &cospi22,
            bf1.as_mut_ptr().add(21),
            &cospim42,
            bf1.as_mut_ptr().add(26),
            &rounding,
            bit,
        );
        bf0[22] = half_btf_avx2(
            &cospi38,
            bf1.as_mut_ptr().add(22),
            &cospim26,
            bf1.as_mut_ptr().add(25),
            &rounding,
            bit,
        );
        bf0[23] = half_btf_avx2(
            &cospi6,
            bf1.as_mut_ptr().add(23),
            &cospim58,
            bf1.as_mut_ptr().add(24),
            &rounding,
            bit,
        );
        bf0[24] = half_btf_avx2(
            &cospi58,
            bf1.as_mut_ptr().add(23),
            &cospi6,
            bf1.as_mut_ptr().add(24),
            &rounding,
            bit,
        );
        bf0[25] = half_btf_avx2(
            &cospi26,
            bf1.as_mut_ptr().add(22),
            &cospi38,
            bf1.as_mut_ptr().add(25),
            &rounding,
            bit,
        );
        bf0[26] = half_btf_avx2(
            &cospi42,
            bf1.as_mut_ptr().add(21),
            &cospi22,
            bf1.as_mut_ptr().add(26),
            &rounding,
            bit,
        );
        bf0[27] = half_btf_avx2(
            &cospi10,
            bf1.as_mut_ptr().add(20),
            &cospi54,
            bf1.as_mut_ptr().add(27),
            &rounding,
            bit,
        );
        bf0[28] = half_btf_avx2(
            &cospi50,
            bf1.as_mut_ptr().add(19),
            &cospi14,
            bf1.as_mut_ptr().add(28),
            &rounding,
            bit,
        );
        bf0[29] = half_btf_avx2(
            &cospi18,
            bf1.as_mut_ptr().add(18),
            &cospi46,
            bf1.as_mut_ptr().add(29),
            &rounding,
            bit,
        );
        bf0[30] = half_btf_avx2(
            &cospi34,
            bf1.as_mut_ptr().add(17),
            &cospi30,
            bf1.as_mut_ptr().add(30),
            &rounding,
            bit,
        );
        bf0[31] = half_btf_avx2(
            &cospi2,
            bf1.as_mut_ptr().add(16),
            &cospi62,
            bf1.as_mut_ptr().add(31),
            &rounding,
            bit,
        );

        // stage 3
        bf1[0] = bf0[0];
        bf1[1] = bf0[1];
        bf1[2] = bf0[2];
        bf1[3] = bf0[3];
        bf1[4] = bf0[4];
        bf1[5] = bf0[5];
        bf1[6] = bf0[6];
        bf1[7] = bf0[7];
        bf1[8] = half_btf_avx2(
            &cospi60,
            bf0.as_mut_ptr().add(8),
            &cospim4,
            bf0.as_mut_ptr().add(15),
            &rounding,
            bit,
        );
        bf1[9] = half_btf_avx2(
            &cospi28,
            bf0.as_mut_ptr().add(9),
            &cospim36,
            bf0.as_mut_ptr().add(14),
            &rounding,
            bit,
        );
        bf1[10] = half_btf_avx2(
            &cospi44,
            bf0.as_mut_ptr().add(10),
            &cospim20,
            bf0.as_mut_ptr().add(13),
            &rounding,
            bit,
        );
        bf1[11] = half_btf_avx2(
            &cospi12,
            bf0.as_mut_ptr().add(11),
            &cospim52,
            bf0.as_mut_ptr().add(12),
            &rounding,
            bit,
        );
        bf1[12] = half_btf_avx2(
            &cospi52,
            bf0.as_mut_ptr().add(11),
            &cospi12,
            bf0.as_mut_ptr().add(12),
            &rounding,
            bit,
        );
        bf1[13] = half_btf_avx2(
            &cospi20,
            bf0.as_mut_ptr().add(10),
            &cospi44,
            bf0.as_mut_ptr().add(13),
            &rounding,
            bit,
        );
        bf1[14] = half_btf_avx2(
            &cospi36,
            bf0.as_mut_ptr().add(9),
            &cospi28,
            bf0.as_mut_ptr().add(14),
            &rounding,
            bit,
        );
        bf1[15] = half_btf_avx2(
            &cospi4,
            bf0.as_mut_ptr().add(8),
            &cospi60,
            bf0.as_mut_ptr().add(15),
            &rounding,
            bit,
        );

        addsub_avx2(
            bf0[16],
            bf0[17],
            bf1.as_mut_ptr().add(16),
            bf1.as_mut_ptr().add(17),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            bf0[19],
            bf0[18],
            bf1.as_mut_ptr().add(19),
            bf1.as_mut_ptr().add(18),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            bf0[20],
            bf0[21],
            bf1.as_mut_ptr().add(20),
            bf1.as_mut_ptr().add(21),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            bf0[23],
            bf0[22],
            bf1.as_mut_ptr().add(23),
            bf1.as_mut_ptr().add(22),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            bf0[24],
            bf0[25],
            bf1.as_mut_ptr().add(24),
            bf1.as_mut_ptr().add(25),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            bf0[27],
            bf0[26],
            bf1.as_mut_ptr().add(27),
            bf1.as_mut_ptr().add(26),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            bf0[28],
            bf0[29],
            bf1.as_mut_ptr().add(28),
            bf1.as_mut_ptr().add(29),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            bf0[31],
            bf0[30],
            bf1.as_mut_ptr().add(31),
            bf1.as_mut_ptr().add(30),
            &clamp_lo,
            &clamp_hi,
        );

        // stage 4
        bf0[0] = bf1[0];
        bf0[1] = bf1[1];
        bf0[2] = bf1[2];
        bf0[3] = bf1[3];
        bf0[4] = half_btf_avx2(
            &cospi56,
            bf1.as_mut_ptr().add(4),
            &cospim8,
            bf1.as_mut_ptr().add(7),
            &rounding,
            bit,
        );
        bf0[5] = half_btf_avx2(
            &cospi24,
            bf1.as_mut_ptr().add(5),
            &cospim40,
            bf1.as_mut_ptr().add(6),
            &rounding,
            bit,
        );
        bf0[6] = half_btf_avx2(
            &cospi40,
            bf1.as_mut_ptr().add(5),
            &cospi24,
            bf1.as_mut_ptr().add(6),
            &rounding,
            bit,
        );
        bf0[7] = half_btf_avx2(
            &cospi8,
            bf1.as_mut_ptr().add(4),
            &cospi56,
            bf1.as_mut_ptr().add(7),
            &rounding,
            bit,
        );

        addsub_avx2(
            bf1[8],
            bf1[9],
            bf0.as_mut_ptr().add(8),
            bf0.as_mut_ptr().add(9),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            bf1[11],
            bf1[10],
            bf0.as_mut_ptr().add(11),
            bf0.as_mut_ptr().add(10),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            bf1[12],
            bf1[13],
            bf0.as_mut_ptr().add(12),
            bf0.as_mut_ptr().add(13),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            bf1[15],
            bf1[14],
            bf0.as_mut_ptr().add(15),
            bf0.as_mut_ptr().add(14),
            &clamp_lo,
            &clamp_hi,
        );

        bf0[16] = bf1[16];
        bf0[17] = half_btf_avx2(
            &cospim8,
            bf1.as_mut_ptr().add(17),
            &cospi56,
            bf1.as_mut_ptr().add(30),
            &rounding,
            bit,
        );
        bf0[18] = half_btf_avx2(
            &cospim56,
            bf1.as_mut_ptr().add(18),
            &cospim8,
            bf1.as_mut_ptr().add(29),
            &rounding,
            bit,
        );
        bf0[19] = bf1[19];
        bf0[20] = bf1[20];
        bf0[21] = half_btf_avx2(
            &cospim40,
            bf1.as_mut_ptr().add(21),
            &cospi24,
            bf1.as_mut_ptr().add(26),
            &rounding,
            bit,
        );
        bf0[22] = half_btf_avx2(
            &cospim24,
            bf1.as_mut_ptr().add(22),
            &cospim40,
            bf1.as_mut_ptr().add(25),
            &rounding,
            bit,
        );
        bf0[23] = bf1[23];
        bf0[24] = bf1[24];
        bf0[25] = half_btf_avx2(
            &cospim40,
            bf1.as_mut_ptr().add(22),
            &cospi24,
            bf1.as_mut_ptr().add(25),
            &rounding,
            bit,
        );
        bf0[26] = half_btf_avx2(
            &cospi24,
            bf1.as_mut_ptr().add(21),
            &cospi40,
            bf1.as_mut_ptr().add(26),
            &rounding,
            bit,
        );
        bf0[27] = bf1[27];
        bf0[28] = bf1[28];
        bf0[29] = half_btf_avx2(
            &cospim8,
            bf1.as_mut_ptr().add(18),
            &cospi56,
            bf1.as_mut_ptr().add(29),
            &rounding,
            bit,
        );
        bf0[30] = half_btf_avx2(
            &cospi56,
            bf1.as_mut_ptr().add(17),
            &cospi8,
            bf1.as_mut_ptr().add(30),
            &rounding,
            bit,
        );
        bf0[31] = bf1[31];

        // stage 5
        bf1[0] = half_btf_avx2(
            &cospi32,
            bf0.as_mut_ptr().add(0),
            &cospi32,
            bf0.as_mut_ptr().add(1),
            &rounding,
            bit,
        );
        bf1[1] = half_btf_avx2(
            &cospi32,
            bf0.as_mut_ptr().add(0),
            &cospim32,
            bf0.as_mut_ptr().add(1),
            &rounding,
            bit,
        );
        bf1[2] = half_btf_avx2(
            &cospi48,
            bf0.as_mut_ptr().add(2),
            &cospim16,
            bf0.as_mut_ptr().add(3),
            &rounding,
            bit,
        );
        bf1[3] = half_btf_avx2(
            &cospi16,
            bf0.as_mut_ptr().add(2),
            &cospi48,
            bf0.as_mut_ptr().add(3),
            &rounding,
            bit,
        );
        addsub_avx2(
            bf0[4],
            bf0[5],
            bf1.as_mut_ptr().add(4),
            bf1.as_mut_ptr().add(5),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            bf0[7],
            bf0[6],
            bf1.as_mut_ptr().add(7),
            bf1.as_mut_ptr().add(6),
            &clamp_lo,
            &clamp_hi,
        );
        bf1[8] = bf0[8];
        bf1[9] = half_btf_avx2(
            &cospim16,
            bf0.as_mut_ptr().add(9),
            &cospi48,
            bf0.as_mut_ptr().add(14),
            &rounding,
            bit,
        );
        bf1[10] = half_btf_avx2(
            &cospim48,
            bf0.as_mut_ptr().add(10),
            &cospim16,
            bf0.as_mut_ptr().add(13),
            &rounding,
            bit,
        );
        bf1[11] = bf0[11];
        bf1[12] = bf0[12];
        bf1[13] = half_btf_avx2(
            &cospim16,
            bf0.as_mut_ptr().add(10),
            &cospi48,
            bf0.as_mut_ptr().add(13),
            &rounding,
            bit,
        );
        bf1[14] = half_btf_avx2(
            &cospi48,
            bf0.as_mut_ptr().add(9),
            &cospi16,
            bf0.as_mut_ptr().add(14),
            &rounding,
            bit,
        );
        bf1[15] = bf0[15];
        addsub_avx2(
            bf0[16],
            bf0[19],
            bf1.as_mut_ptr().add(16),
            bf1.as_mut_ptr().add(19),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            bf0[17],
            bf0[18],
            bf1.as_mut_ptr().add(17),
            bf1.as_mut_ptr().add(18),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            bf0[23],
            bf0[20],
            bf1.as_mut_ptr().add(23),
            bf1.as_mut_ptr().add(20),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            bf0[22],
            bf0[21],
            bf1.as_mut_ptr().add(22),
            bf1.as_mut_ptr().add(21),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            bf0[24],
            bf0[27],
            bf1.as_mut_ptr().add(24),
            bf1.as_mut_ptr().add(27),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            bf0[25],
            bf0[26],
            bf1.as_mut_ptr().add(25),
            bf1.as_mut_ptr().add(26),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            bf0[31],
            bf0[28],
            bf1.as_mut_ptr().add(31),
            bf1.as_mut_ptr().add(28),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            bf0[30],
            bf0[29],
            bf1.as_mut_ptr().add(30),
            bf1.as_mut_ptr().add(29),
            &clamp_lo,
            &clamp_hi,
        );

        // stage 6
        addsub_avx2(
            bf1[0],
            bf1[3],
            bf0.as_mut_ptr().add(0),
            bf0.as_mut_ptr().add(3),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            bf1[1],
            bf1[2],
            bf0.as_mut_ptr().add(1),
            bf0.as_mut_ptr().add(2),
            &clamp_lo,
            &clamp_hi,
        );
        bf0[4] = bf1[4];
        bf0[5] = half_btf_avx2(
            &cospim32,
            bf1.as_mut_ptr().add(5),
            &cospi32,
            bf1.as_mut_ptr().add(6),
            &rounding,
            bit,
        );
        bf0[6] = half_btf_avx2(
            &cospi32,
            bf1.as_mut_ptr().add(5),
            &cospi32,
            bf1.as_mut_ptr().add(6),
            &rounding,
            bit,
        );
        bf0[7] = bf1[7];
        addsub_avx2(
            bf1[8],
            bf1[11],
            bf0.as_mut_ptr().add(8),
            bf0.as_mut_ptr().add(11),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            bf1[9],
            bf1[10],
            bf0.as_mut_ptr().add(9),
            bf0.as_mut_ptr().add(10),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            bf1[15],
            bf1[12],
            bf0.as_mut_ptr().add(15),
            bf0.as_mut_ptr().add(12),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            bf1[14],
            bf1[13],
            bf0.as_mut_ptr().add(14),
            bf0.as_mut_ptr().add(13),
            &clamp_lo,
            &clamp_hi,
        );
        bf0[16] = bf1[16];
        bf0[17] = bf1[17];
        bf0[18] = half_btf_avx2(
            &cospim16,
            bf1.as_mut_ptr().add(18),
            &cospi48,
            bf1.as_mut_ptr().add(29),
            &rounding,
            bit,
        );
        bf0[19] = half_btf_avx2(
            &cospim16,
            bf1.as_mut_ptr().add(19),
            &cospi48,
            bf1.as_mut_ptr().add(28),
            &rounding,
            bit,
        );
        bf0[20] = half_btf_avx2(
            &cospim48,
            bf1.as_mut_ptr().add(20),
            &cospim16,
            bf1.as_mut_ptr().add(27),
            &rounding,
            bit,
        );
        bf0[21] = half_btf_avx2(
            &cospim48,
            bf1.as_mut_ptr().add(21),
            &cospim16,
            bf1.as_mut_ptr().add(26),
            &rounding,
            bit,
        );
        bf0[22] = bf1[22];
        bf0[23] = bf1[23];
        bf0[24] = bf1[24];
        bf0[25] = bf1[25];
        bf0[26] = half_btf_avx2(
            &cospim16,
            bf1.as_mut_ptr().add(21),
            &cospi48,
            bf1.as_mut_ptr().add(26),
            &rounding,
            bit,
        );
        bf0[27] = half_btf_avx2(
            &cospim16,
            bf1.as_mut_ptr().add(20),
            &cospi48,
            bf1.as_mut_ptr().add(27),
            &rounding,
            bit,
        );
        bf0[28] = half_btf_avx2(
            &cospi48,
            bf1.as_mut_ptr().add(19),
            &cospi16,
            bf1.as_mut_ptr().add(28),
            &rounding,
            bit,
        );
        bf0[29] = half_btf_avx2(
            &cospi48,
            bf1.as_mut_ptr().add(18),
            &cospi16,
            bf1.as_mut_ptr().add(29),
            &rounding,
            bit,
        );
        bf0[30] = bf1[30];
        bf0[31] = bf1[31];

        // stage 7
        addsub_avx2(
            bf0[0],
            bf0[7],
            bf1.as_mut_ptr().add(0),
            bf1.as_mut_ptr().add(7),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            bf0[1],
            bf0[6],
            bf1.as_mut_ptr().add(1),
            bf1.as_mut_ptr().add(6),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            bf0[2],
            bf0[5],
            bf1.as_mut_ptr().add(2),
            bf1.as_mut_ptr().add(5),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            bf0[3],
            bf0[4],
            bf1.as_mut_ptr().add(3),
            bf1.as_mut_ptr().add(4),
            &clamp_lo,
            &clamp_hi,
        );
        bf1[8] = bf0[8];
        bf1[9] = bf0[9];
        bf1[10] = half_btf_avx2(
            &cospim32,
            bf0.as_mut_ptr().add(10),
            &cospi32,
            bf0.as_mut_ptr().add(13),
            &rounding,
            bit,
        );
        bf1[11] = half_btf_avx2(
            &cospim32,
            bf0.as_mut_ptr().add(11),
            &cospi32,
            bf0.as_mut_ptr().add(12),
            &rounding,
            bit,
        );
        bf1[12] = half_btf_avx2(
            &cospi32,
            bf0.as_mut_ptr().add(11),
            &cospi32,
            bf0.as_mut_ptr().add(12),
            &rounding,
            bit,
        );
        bf1[13] = half_btf_avx2(
            &cospi32,
            bf0.as_mut_ptr().add(10),
            &cospi32,
            bf0.as_mut_ptr().add(13),
            &rounding,
            bit,
        );
        bf1[14] = bf0[14];
        bf1[15] = bf0[15];
        addsub_avx2(
            bf0[16],
            bf0[23],
            bf1.as_mut_ptr().add(16),
            bf1.as_mut_ptr().add(23),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            bf0[17],
            bf0[22],
            bf1.as_mut_ptr().add(17),
            bf1.as_mut_ptr().add(22),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            bf0[18],
            bf0[21],
            bf1.as_mut_ptr().add(18),
            bf1.as_mut_ptr().add(21),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            bf0[19],
            bf0[20],
            bf1.as_mut_ptr().add(19),
            bf1.as_mut_ptr().add(20),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            bf0[31],
            bf0[24],
            bf1.as_mut_ptr().add(31),
            bf1.as_mut_ptr().add(24),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            bf0[30],
            bf0[25],
            bf1.as_mut_ptr().add(30),
            bf1.as_mut_ptr().add(25),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            bf0[29],
            bf0[26],
            bf1.as_mut_ptr().add(29),
            bf1.as_mut_ptr().add(26),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            bf0[28],
            bf0[27],
            bf1.as_mut_ptr().add(28),
            bf1.as_mut_ptr().add(27),
            &clamp_lo,
            &clamp_hi,
        );

        // stage 8
        addsub_avx2(
            bf1[0],
            bf1[15],
            bf0.as_mut_ptr().add(0),
            bf0.as_mut_ptr().add(15),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            bf1[1],
            bf1[14],
            bf0.as_mut_ptr().add(1),
            bf0.as_mut_ptr().add(14),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            bf1[2],
            bf1[13],
            bf0.as_mut_ptr().add(2),
            bf0.as_mut_ptr().add(13),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            bf1[3],
            bf1[12],
            bf0.as_mut_ptr().add(3),
            bf0.as_mut_ptr().add(12),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            bf1[4],
            bf1[11],
            bf0.as_mut_ptr().add(4),
            bf0.as_mut_ptr().add(11),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            bf1[5],
            bf1[10],
            bf0.as_mut_ptr().add(5),
            bf0.as_mut_ptr().add(10),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            bf1[6],
            bf1[9],
            bf0.as_mut_ptr().add(6),
            bf0.as_mut_ptr().add(9),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            bf1[7],
            bf1[8],
            bf0.as_mut_ptr().add(7),
            bf0.as_mut_ptr().add(8),
            &clamp_lo,
            &clamp_hi,
        );
        bf0[16] = bf1[16];
        bf0[17] = bf1[17];
        bf0[18] = bf1[18];
        bf0[19] = bf1[19];
        bf0[20] = half_btf_avx2(
            &cospim32,
            bf1.as_mut_ptr().add(20),
            &cospi32,
            bf1.as_mut_ptr().add(27),
            &rounding,
            bit,
        );
        bf0[21] = half_btf_avx2(
            &cospim32,
            bf1.as_mut_ptr().add(21),
            &cospi32,
            bf1.as_mut_ptr().add(26),
            &rounding,
            bit,
        );
        bf0[22] = half_btf_avx2(
            &cospim32,
            bf1.as_mut_ptr().add(22),
            &cospi32,
            bf1.as_mut_ptr().add(25),
            &rounding,
            bit,
        );
        bf0[23] = half_btf_avx2(
            &cospim32,
            bf1.as_mut_ptr().add(23),
            &cospi32,
            bf1.as_mut_ptr().add(24),
            &rounding,
            bit,
        );
        bf0[24] = half_btf_avx2(
            &cospi32,
            bf1.as_mut_ptr().add(23),
            &cospi32,
            bf1.as_mut_ptr().add(24),
            &rounding,
            bit,
        );
        bf0[25] = half_btf_avx2(
            &cospi32,
            bf1.as_mut_ptr().add(22),
            &cospi32,
            bf1.as_mut_ptr().add(25),
            &rounding,
            bit,
        );
        bf0[26] = half_btf_avx2(
            &cospi32,
            bf1.as_mut_ptr().add(21),
            &cospi32,
            bf1.as_mut_ptr().add(26),
            &rounding,
            bit,
        );
        bf0[27] = half_btf_avx2(
            &cospi32,
            bf1.as_mut_ptr().add(20),
            &cospi32,
            bf1.as_mut_ptr().add(27),
            &rounding,
            bit,
        );
        bf0[28] = bf1[28];
        bf0[29] = bf1[29];
        bf0[30] = bf1[30];
        bf0[31] = bf1[31];

        // stage 9
        addsub_avx2(bf0[0], bf0[31], out.add(0), out.add(31), &clamp_lo, &clamp_hi);
        addsub_avx2(bf0[1], bf0[30], out.add(1), out.add(30), &clamp_lo, &clamp_hi);
        addsub_avx2(bf0[2], bf0[29], out.add(2), out.add(29), &clamp_lo, &clamp_hi);
        addsub_avx2(bf0[3], bf0[28], out.add(3), out.add(28), &clamp_lo, &clamp_hi);
        addsub_avx2(bf0[4], bf0[27], out.add(4), out.add(27), &clamp_lo, &clamp_hi);
        addsub_avx2(bf0[5], bf0[26], out.add(5), out.add(26), &clamp_lo, &clamp_hi);
        addsub_avx2(bf0[6], bf0[25], out.add(6), out.add(25), &clamp_lo, &clamp_hi);
        addsub_avx2(bf0[7], bf0[24], out.add(7), out.add(24), &clamp_lo, &clamp_hi);
        addsub_avx2(bf0[8], bf0[23], out.add(8), out.add(23), &clamp_lo, &clamp_hi);
        addsub_avx2(bf0[9], bf0[22], out.add(9), out.add(22), &clamp_lo, &clamp_hi);
        addsub_avx2(bf0[10], bf0[21], out.add(10), out.add(21), &clamp_lo, &clamp_hi);
        addsub_avx2(bf0[11], bf0[20], out.add(11), out.add(20), &clamp_lo, &clamp_hi);
        addsub_avx2(bf0[12], bf0[19], out.add(12), out.add(19), &clamp_lo, &clamp_hi);
        addsub_avx2(bf0[13], bf0[18], out.add(13), out.add(18), &clamp_lo, &clamp_hi);
        addsub_avx2(bf0[14], bf0[17], out.add(14), out.add(17), &clamp_lo, &clamp_hi);
        addsub_avx2(bf0[15], bf0[16], out.add(15), out.add(16), &clamp_lo, &clamp_hi);
        if do_cols == 0 {
            let mut log_range_out: i32 = get_log_range_out(bd);
            let mut clamp_lo_out: __m256i = _mm256_set1_epi32(-(1 << (log_range_out - 1)));
            let mut clamp_hi_out: __m256i = _mm256_set1_epi32((1 << (log_range_out - 1)) - 1);
            round_shift_8x8_avx2(out, out_shift);
            round_shift_8x8_avx2(out.add(16), out_shift);
            highbd_clamp_epi32_avx2(out, out, &clamp_lo_out, &clamp_hi_out, 32);
        }
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn idct16_low1_avx2(
    mut in_: *mut __m256i,
    mut out: *mut __m256i,
    mut bit: i32,
    mut do_cols: i32,
    mut bd: i32,
    mut out_shift: i32,
) {
    let mut cospi: *const i32 = cospi_arr(bit);
    let mut cospi32: __m256i = _mm256_set1_epi32(*cospi.add(32));
    let mut rnding: __m256i = _mm256_set1_epi32(1 << (bit - 1));
    let mut log_range: i32 = get_log_range(bd, do_cols);
    let mut clamp_lo: __m256i = _mm256_set1_epi32(-(1 << (log_range - 1)));
    let mut clamp_hi: __m256i = _mm256_set1_epi32((1 << (log_range - 1)) - 1);

    {
        // stage 0
        // stage 1
        // stage 2
        // stage 3
        // stage 4
        *in_.add(0) = _mm256_mullo_epi32(*in_.add(0), cospi32);
        *in_.add(0) = _mm256_add_epi32(*in_.add(0), rnding);
        *in_.add(0) = _mm256_sra_epi32(*in_.add(0), _mm_cvtsi32_si128(bit));

        // stage 5
        // stage 6
        // stage 7
        if do_cols == 0 {
            let mut log_range_out: i32 = get_log_range_out(bd);
            clamp_lo = _mm256_set1_epi32(-(1 << (log_range_out - 1)));
            clamp_hi = _mm256_set1_epi32((1 << (log_range_out - 1)) - 1);
            let mut offset: __m256i = _mm256_set1_epi32((1 << out_shift) >> 1);
            *in_.add(0) = _mm256_add_epi32(*in_.add(0), offset);
            *in_.add(0) = _mm256_sra_epi32(*in_.add(0), _mm_cvtsi32_si128(out_shift));
        }
        *in_.add(0) = _mm256_max_epi32(*in_.add(0), clamp_lo);
        *in_.add(0) = _mm256_min_epi32(*in_.add(0), clamp_hi);
        *out.add(0) = *in_.add(0);
        *out.add(1) = *in_.add(0);
        *out.add(2) = *in_.add(0);
        *out.add(3) = *in_.add(0);
        *out.add(4) = *in_.add(0);
        *out.add(5) = *in_.add(0);
        *out.add(6) = *in_.add(0);
        *out.add(7) = *in_.add(0);
        *out.add(8) = *in_.add(0);
        *out.add(9) = *in_.add(0);
        *out.add(10) = *in_.add(0);
        *out.add(11) = *in_.add(0);
        *out.add(12) = *in_.add(0);
        *out.add(13) = *in_.add(0);
        *out.add(14) = *in_.add(0);
        *out.add(15) = *in_.add(0);
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn idct16_low8_avx2(
    mut in_: *mut __m256i,
    mut out: *mut __m256i,
    mut bit: i32,
    mut do_cols: i32,
    mut bd: i32,
    mut out_shift: i32,
) {
    let mut cospi: *const i32 = cospi_arr(bit);
    let mut cospi60: __m256i = _mm256_set1_epi32(*cospi.add(60));
    let mut cospi28: __m256i = _mm256_set1_epi32(*cospi.add(28));
    let mut cospi44: __m256i = _mm256_set1_epi32(*cospi.add(44));
    let mut cospi20: __m256i = _mm256_set1_epi32(*cospi.add(20));
    let mut cospi12: __m256i = _mm256_set1_epi32(*cospi.add(12));
    let mut cospi4: __m256i = _mm256_set1_epi32(*cospi.add(4));
    let mut cospi56: __m256i = _mm256_set1_epi32(*cospi.add(56));
    let mut cospi24: __m256i = _mm256_set1_epi32(*cospi.add(24));
    let mut cospim40: __m256i = _mm256_set1_epi32(-*cospi.add(40));
    let mut cospi8: __m256i = _mm256_set1_epi32(*cospi.add(8));
    let mut cospi32: __m256i = _mm256_set1_epi32(*cospi.add(32));
    let mut cospi48: __m256i = _mm256_set1_epi32(*cospi.add(48));
    let mut cospi16: __m256i = _mm256_set1_epi32(*cospi.add(16));
    let mut cospim16: __m256i = _mm256_set1_epi32(-*cospi.add(16));
    let mut cospim48: __m256i = _mm256_set1_epi32(-*cospi.add(48));
    let mut cospim36: __m256i = _mm256_set1_epi32(-*cospi.add(36));
    let mut cospim52: __m256i = _mm256_set1_epi32(-*cospi.add(52));
    let mut rnding: __m256i = _mm256_set1_epi32(1 << (bit - 1));
    let mut log_range: i32 = get_log_range(bd, do_cols);
    let mut clamp_lo: __m256i = _mm256_set1_epi32(-(1 << (log_range - 1)));
    let mut clamp_hi: __m256i = _mm256_set1_epi32((1 << (log_range - 1)) - 1);
    let mut u: [__m256i; 16] = core::mem::zeroed();
    let mut x: __m256i;
    let mut y: __m256i;

    {
        // stage 0
        // stage 1
        u[0] = *in_.add(0);
        u[2] = *in_.add(4);
        u[4] = *in_.add(2);
        u[6] = *in_.add(6);
        u[8] = *in_.add(1);
        u[10] = *in_.add(5);
        u[12] = *in_.add(3);
        u[14] = *in_.add(7);

        // stage 2
        u[15] = half_btf_0_avx2(&cospi4, u.as_mut_ptr().add(8), &rnding, bit);
        u[8] = half_btf_0_avx2(&cospi60, u.as_mut_ptr().add(8), &rnding, bit);

        u[9] = half_btf_0_avx2(&cospim36, u.as_mut_ptr().add(14), &rnding, bit);
        u[14] = half_btf_0_avx2(&cospi28, u.as_mut_ptr().add(14), &rnding, bit);

        u[13] = half_btf_0_avx2(&cospi20, u.as_mut_ptr().add(10), &rnding, bit);
        u[10] = half_btf_0_avx2(&cospi44, u.as_mut_ptr().add(10), &rnding, bit);

        u[11] = half_btf_0_avx2(&cospim52, u.as_mut_ptr().add(12), &rnding, bit);
        u[12] = half_btf_0_avx2(&cospi12, u.as_mut_ptr().add(12), &rnding, bit);

        // stage 3
        u[7] = half_btf_0_avx2(&cospi8, u.as_mut_ptr().add(4), &rnding, bit);
        u[4] = half_btf_0_avx2(&cospi56, u.as_mut_ptr().add(4), &rnding, bit);
        u[5] = half_btf_0_avx2(&cospim40, u.as_mut_ptr().add(6), &rnding, bit);
        u[6] = half_btf_0_avx2(&cospi24, u.as_mut_ptr().add(6), &rnding, bit);

        addsub_avx2(u[8], u[9], u.as_mut_ptr().add(8), u.as_mut_ptr().add(9), &clamp_lo, &clamp_hi);
        addsub_avx2(
            u[11],
            u[10],
            u.as_mut_ptr().add(11),
            u.as_mut_ptr().add(10),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            u[12],
            u[13],
            u.as_mut_ptr().add(12),
            u.as_mut_ptr().add(13),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            u[15],
            u[14],
            u.as_mut_ptr().add(15),
            u.as_mut_ptr().add(14),
            &clamp_lo,
            &clamp_hi,
        );

        // stage 4
        x = _mm256_mullo_epi32(u[0], cospi32);
        u[0] = _mm256_add_epi32(x, rnding);
        u[0] = _mm256_sra_epi32(u[0], _mm_cvtsi32_si128(bit));
        u[1] = u[0];

        u[3] = half_btf_0_avx2(&cospi16, u.as_mut_ptr().add(2), &rnding, bit);
        u[2] = half_btf_0_avx2(&cospi48, u.as_mut_ptr().add(2), &rnding, bit);

        addsub_avx2(u[4], u[5], u.as_mut_ptr().add(4), u.as_mut_ptr().add(5), &clamp_lo, &clamp_hi);
        addsub_avx2(u[7], u[6], u.as_mut_ptr().add(7), u.as_mut_ptr().add(6), &clamp_lo, &clamp_hi);

        x = half_btf_avx2(
            &cospim16,
            u.as_mut_ptr().add(9),
            &cospi48,
            u.as_mut_ptr().add(14),
            &rnding,
            bit,
        );
        u[14] = half_btf_avx2(
            &cospi48,
            u.as_mut_ptr().add(9),
            &cospi16,
            u.as_mut_ptr().add(14),
            &rnding,
            bit,
        );
        u[9] = x;
        y = half_btf_avx2(
            &cospim48,
            u.as_mut_ptr().add(10),
            &cospim16,
            u.as_mut_ptr().add(13),
            &rnding,
            bit,
        );
        u[13] = half_btf_avx2(
            &cospim16,
            u.as_mut_ptr().add(10),
            &cospi48,
            u.as_mut_ptr().add(13),
            &rnding,
            bit,
        );
        u[10] = y;

        // stage 5
        addsub_avx2(u[0], u[3], u.as_mut_ptr().add(0), u.as_mut_ptr().add(3), &clamp_lo, &clamp_hi);
        addsub_avx2(u[1], u[2], u.as_mut_ptr().add(1), u.as_mut_ptr().add(2), &clamp_lo, &clamp_hi);

        x = _mm256_mullo_epi32(u[5], cospi32);
        y = _mm256_mullo_epi32(u[6], cospi32);
        u[5] = _mm256_sub_epi32(y, x);
        u[5] = _mm256_add_epi32(u[5], rnding);
        u[5] = _mm256_sra_epi32(u[5], _mm_cvtsi32_si128(bit));

        u[6] = _mm256_add_epi32(y, x);
        u[6] = _mm256_add_epi32(u[6], rnding);
        u[6] = _mm256_sra_epi32(u[6], _mm_cvtsi32_si128(bit));

        addsub_avx2(
            u[8],
            u[11],
            u.as_mut_ptr().add(8),
            u.as_mut_ptr().add(11),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            u[9],
            u[10],
            u.as_mut_ptr().add(9),
            u.as_mut_ptr().add(10),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            u[15],
            u[12],
            u.as_mut_ptr().add(15),
            u.as_mut_ptr().add(12),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            u[14],
            u[13],
            u.as_mut_ptr().add(14),
            u.as_mut_ptr().add(13),
            &clamp_lo,
            &clamp_hi,
        );

        // stage 6
        addsub_avx2(u[0], u[7], u.as_mut_ptr().add(0), u.as_mut_ptr().add(7), &clamp_lo, &clamp_hi);
        addsub_avx2(u[1], u[6], u.as_mut_ptr().add(1), u.as_mut_ptr().add(6), &clamp_lo, &clamp_hi);
        addsub_avx2(u[2], u[5], u.as_mut_ptr().add(2), u.as_mut_ptr().add(5), &clamp_lo, &clamp_hi);
        addsub_avx2(u[3], u[4], u.as_mut_ptr().add(3), u.as_mut_ptr().add(4), &clamp_lo, &clamp_hi);

        x = _mm256_mullo_epi32(u[10], cospi32);
        y = _mm256_mullo_epi32(u[13], cospi32);
        u[10] = _mm256_sub_epi32(y, x);
        u[10] = _mm256_add_epi32(u[10], rnding);
        u[10] = _mm256_sra_epi32(u[10], _mm_cvtsi32_si128(bit));

        u[13] = _mm256_add_epi32(x, y);
        u[13] = _mm256_add_epi32(u[13], rnding);
        u[13] = _mm256_sra_epi32(u[13], _mm_cvtsi32_si128(bit));

        x = _mm256_mullo_epi32(u[11], cospi32);
        y = _mm256_mullo_epi32(u[12], cospi32);
        u[11] = _mm256_sub_epi32(y, x);
        u[11] = _mm256_add_epi32(u[11], rnding);
        u[11] = _mm256_sra_epi32(u[11], _mm_cvtsi32_si128(bit));

        u[12] = _mm256_add_epi32(x, y);
        u[12] = _mm256_add_epi32(u[12], rnding);
        u[12] = _mm256_sra_epi32(u[12], _mm_cvtsi32_si128(bit));
        // stage 7
        addsub_avx2(u[0], u[15], out.add(0), out.add(15), &clamp_lo, &clamp_hi);
        addsub_avx2(u[1], u[14], out.add(1), out.add(14), &clamp_lo, &clamp_hi);
        addsub_avx2(u[2], u[13], out.add(2), out.add(13), &clamp_lo, &clamp_hi);
        addsub_avx2(u[3], u[12], out.add(3), out.add(12), &clamp_lo, &clamp_hi);
        addsub_avx2(u[4], u[11], out.add(4), out.add(11), &clamp_lo, &clamp_hi);
        addsub_avx2(u[5], u[10], out.add(5), out.add(10), &clamp_lo, &clamp_hi);
        addsub_avx2(u[6], u[9], out.add(6), out.add(9), &clamp_lo, &clamp_hi);
        addsub_avx2(u[7], u[8], out.add(7), out.add(8), &clamp_lo, &clamp_hi);

        if do_cols == 0 {
            let mut log_range_out: i32 = get_log_range_out(bd);
            let mut clamp_lo_out: __m256i = _mm256_set1_epi32(-(1 << (log_range_out - 1)));
            let mut clamp_hi_out: __m256i = _mm256_set1_epi32((1 << (log_range_out - 1)) - 1);
            round_shift_8x8_avx2(out, out_shift);
            highbd_clamp_epi32_avx2(out, out, &clamp_lo_out, &clamp_hi_out, 16);
        }
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn idct16_avx2(
    mut in_: *mut __m256i,
    mut out: *mut __m256i,
    mut bit: i32,
    mut do_cols: i32,
    mut bd: i32,
    mut out_shift: i32,
) {
    let mut cospi: *const i32 = cospi_arr(bit);
    let mut cospi60: __m256i = _mm256_set1_epi32(*cospi.add(60));
    let mut cospim4: __m256i = _mm256_set1_epi32(-*cospi.add(4));
    let mut cospi28: __m256i = _mm256_set1_epi32(*cospi.add(28));
    let mut cospim36: __m256i = _mm256_set1_epi32(-*cospi.add(36));
    let mut cospi44: __m256i = _mm256_set1_epi32(*cospi.add(44));
    let mut cospi20: __m256i = _mm256_set1_epi32(*cospi.add(20));
    let mut cospim20: __m256i = _mm256_set1_epi32(-*cospi.add(20));
    let mut cospi12: __m256i = _mm256_set1_epi32(*cospi.add(12));
    let mut cospim52: __m256i = _mm256_set1_epi32(-*cospi.add(52));
    let mut cospi52: __m256i = _mm256_set1_epi32(*cospi.add(52));
    let mut cospi36: __m256i = _mm256_set1_epi32(*cospi.add(36));
    let mut cospi4: __m256i = _mm256_set1_epi32(*cospi.add(4));
    let mut cospi56: __m256i = _mm256_set1_epi32(*cospi.add(56));
    let mut cospim8: __m256i = _mm256_set1_epi32(-*cospi.add(8));
    let mut cospi24: __m256i = _mm256_set1_epi32(*cospi.add(24));
    let mut cospim40: __m256i = _mm256_set1_epi32(-*cospi.add(40));
    let mut cospi40: __m256i = _mm256_set1_epi32(*cospi.add(40));
    let mut cospi8: __m256i = _mm256_set1_epi32(*cospi.add(8));
    let mut cospi32: __m256i = _mm256_set1_epi32(*cospi.add(32));
    let mut cospi48: __m256i = _mm256_set1_epi32(*cospi.add(48));
    let mut cospi16: __m256i = _mm256_set1_epi32(*cospi.add(16));
    let mut cospim16: __m256i = _mm256_set1_epi32(-*cospi.add(16));
    let mut cospim48: __m256i = _mm256_set1_epi32(-*cospi.add(48));
    let mut rnding: __m256i = _mm256_set1_epi32(1 << (bit - 1));
    let mut log_range: i32 = get_log_range(bd, do_cols);
    let mut clamp_lo: __m256i = _mm256_set1_epi32(-(1 << (log_range - 1)));
    let mut clamp_hi: __m256i = _mm256_set1_epi32((1 << (log_range - 1)) - 1);
    let mut u: [__m256i; 16] = core::mem::zeroed();
    let mut v: [__m256i; 16] = core::mem::zeroed();
    let mut x: __m256i;
    let mut y: __m256i;

    {
        // stage 0
        // stage 1
        u[0] = *in_.add(0);
        u[1] = *in_.add(8);
        u[2] = *in_.add(4);
        u[3] = *in_.add(12);
        u[4] = *in_.add(2);
        u[5] = *in_.add(10);
        u[6] = *in_.add(6);
        u[7] = *in_.add(14);
        u[8] = *in_.add(1);
        u[9] = *in_.add(9);
        u[10] = *in_.add(5);
        u[11] = *in_.add(13);
        u[12] = *in_.add(3);
        u[13] = *in_.add(11);
        u[14] = *in_.add(7);
        u[15] = *in_.add(15);

        // stage 2
        v[0] = u[0];
        v[1] = u[1];
        v[2] = u[2];
        v[3] = u[3];
        v[4] = u[4];
        v[5] = u[5];
        v[6] = u[6];
        v[7] = u[7];

        v[8] = half_btf_avx2(
            &cospi60,
            u.as_mut_ptr().add(8),
            &cospim4,
            u.as_mut_ptr().add(15),
            &rnding,
            bit,
        );
        v[9] = half_btf_avx2(
            &cospi28,
            u.as_mut_ptr().add(9),
            &cospim36,
            u.as_mut_ptr().add(14),
            &rnding,
            bit,
        );
        v[10] = half_btf_avx2(
            &cospi44,
            u.as_mut_ptr().add(10),
            &cospim20,
            u.as_mut_ptr().add(13),
            &rnding,
            bit,
        );
        v[11] = half_btf_avx2(
            &cospi12,
            u.as_mut_ptr().add(11),
            &cospim52,
            u.as_mut_ptr().add(12),
            &rnding,
            bit,
        );
        v[12] = half_btf_avx2(
            &cospi52,
            u.as_mut_ptr().add(11),
            &cospi12,
            u.as_mut_ptr().add(12),
            &rnding,
            bit,
        );
        v[13] = half_btf_avx2(
            &cospi20,
            u.as_mut_ptr().add(10),
            &cospi44,
            u.as_mut_ptr().add(13),
            &rnding,
            bit,
        );
        v[14] = half_btf_avx2(
            &cospi36,
            u.as_mut_ptr().add(9),
            &cospi28,
            u.as_mut_ptr().add(14),
            &rnding,
            bit,
        );
        v[15] = half_btf_avx2(
            &cospi4,
            u.as_mut_ptr().add(8),
            &cospi60,
            u.as_mut_ptr().add(15),
            &rnding,
            bit,
        );

        // stage 3
        u[0] = v[0];
        u[1] = v[1];
        u[2] = v[2];
        u[3] = v[3];
        u[4] = half_btf_avx2(
            &cospi56,
            v.as_mut_ptr().add(4),
            &cospim8,
            v.as_mut_ptr().add(7),
            &rnding,
            bit,
        );
        u[5] = half_btf_avx2(
            &cospi24,
            v.as_mut_ptr().add(5),
            &cospim40,
            v.as_mut_ptr().add(6),
            &rnding,
            bit,
        );
        u[6] = half_btf_avx2(
            &cospi40,
            v.as_mut_ptr().add(5),
            &cospi24,
            v.as_mut_ptr().add(6),
            &rnding,
            bit,
        );
        u[7] = half_btf_avx2(
            &cospi8,
            v.as_mut_ptr().add(4),
            &cospi56,
            v.as_mut_ptr().add(7),
            &rnding,
            bit,
        );
        addsub_avx2(v[8], v[9], u.as_mut_ptr().add(8), u.as_mut_ptr().add(9), &clamp_lo, &clamp_hi);
        addsub_avx2(
            v[11],
            v[10],
            u.as_mut_ptr().add(11),
            u.as_mut_ptr().add(10),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            v[12],
            v[13],
            u.as_mut_ptr().add(12),
            u.as_mut_ptr().add(13),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            v[15],
            v[14],
            u.as_mut_ptr().add(15),
            u.as_mut_ptr().add(14),
            &clamp_lo,
            &clamp_hi,
        );

        // stage 4
        x = _mm256_mullo_epi32(u[0], cospi32);
        y = _mm256_mullo_epi32(u[1], cospi32);
        v[0] = _mm256_add_epi32(x, y);
        v[0] = _mm256_add_epi32(v[0], rnding);
        v[0] = _mm256_sra_epi32(v[0], _mm_cvtsi32_si128(bit));

        v[1] = _mm256_sub_epi32(x, y);
        v[1] = _mm256_add_epi32(v[1], rnding);
        v[1] = _mm256_sra_epi32(v[1], _mm_cvtsi32_si128(bit));

        v[2] = half_btf_avx2(
            &cospi48,
            u.as_mut_ptr().add(2),
            &cospim16,
            u.as_mut_ptr().add(3),
            &rnding,
            bit,
        );
        v[3] = half_btf_avx2(
            &cospi16,
            u.as_mut_ptr().add(2),
            &cospi48,
            u.as_mut_ptr().add(3),
            &rnding,
            bit,
        );
        addsub_avx2(u[4], u[5], v.as_mut_ptr().add(4), v.as_mut_ptr().add(5), &clamp_lo, &clamp_hi);
        addsub_avx2(u[7], u[6], v.as_mut_ptr().add(7), v.as_mut_ptr().add(6), &clamp_lo, &clamp_hi);
        v[8] = u[8];
        v[9] = half_btf_avx2(
            &cospim16,
            u.as_mut_ptr().add(9),
            &cospi48,
            u.as_mut_ptr().add(14),
            &rnding,
            bit,
        );
        v[10] = half_btf_avx2(
            &cospim48,
            u.as_mut_ptr().add(10),
            &cospim16,
            u.as_mut_ptr().add(13),
            &rnding,
            bit,
        );
        v[11] = u[11];
        v[12] = u[12];
        v[13] = half_btf_avx2(
            &cospim16,
            u.as_mut_ptr().add(10),
            &cospi48,
            u.as_mut_ptr().add(13),
            &rnding,
            bit,
        );
        v[14] = half_btf_avx2(
            &cospi48,
            u.as_mut_ptr().add(9),
            &cospi16,
            u.as_mut_ptr().add(14),
            &rnding,
            bit,
        );
        v[15] = u[15];

        // stage 5
        addsub_avx2(v[0], v[3], u.as_mut_ptr().add(0), u.as_mut_ptr().add(3), &clamp_lo, &clamp_hi);
        addsub_avx2(v[1], v[2], u.as_mut_ptr().add(1), u.as_mut_ptr().add(2), &clamp_lo, &clamp_hi);
        u[4] = v[4];

        x = _mm256_mullo_epi32(v[5], cospi32);
        y = _mm256_mullo_epi32(v[6], cospi32);
        u[5] = _mm256_sub_epi32(y, x);
        u[5] = _mm256_add_epi32(u[5], rnding);
        u[5] = _mm256_sra_epi32(u[5], _mm_cvtsi32_si128(bit));

        u[6] = _mm256_add_epi32(y, x);
        u[6] = _mm256_add_epi32(u[6], rnding);
        u[6] = _mm256_sra_epi32(u[6], _mm_cvtsi32_si128(bit));

        u[7] = v[7];
        addsub_avx2(
            v[8],
            v[11],
            u.as_mut_ptr().add(8),
            u.as_mut_ptr().add(11),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            v[9],
            v[10],
            u.as_mut_ptr().add(9),
            u.as_mut_ptr().add(10),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            v[15],
            v[12],
            u.as_mut_ptr().add(15),
            u.as_mut_ptr().add(12),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            v[14],
            v[13],
            u.as_mut_ptr().add(14),
            u.as_mut_ptr().add(13),
            &clamp_lo,
            &clamp_hi,
        );

        // stage 6
        addsub_avx2(u[0], u[7], v.as_mut_ptr().add(0), v.as_mut_ptr().add(7), &clamp_lo, &clamp_hi);
        addsub_avx2(u[1], u[6], v.as_mut_ptr().add(1), v.as_mut_ptr().add(6), &clamp_lo, &clamp_hi);
        addsub_avx2(u[2], u[5], v.as_mut_ptr().add(2), v.as_mut_ptr().add(5), &clamp_lo, &clamp_hi);
        addsub_avx2(u[3], u[4], v.as_mut_ptr().add(3), v.as_mut_ptr().add(4), &clamp_lo, &clamp_hi);
        v[8] = u[8];
        v[9] = u[9];

        x = _mm256_mullo_epi32(u[10], cospi32);
        y = _mm256_mullo_epi32(u[13], cospi32);
        v[10] = _mm256_sub_epi32(y, x);
        v[10] = _mm256_add_epi32(v[10], rnding);
        v[10] = _mm256_sra_epi32(v[10], _mm_cvtsi32_si128(bit));

        v[13] = _mm256_add_epi32(x, y);
        v[13] = _mm256_add_epi32(v[13], rnding);
        v[13] = _mm256_sra_epi32(v[13], _mm_cvtsi32_si128(bit));

        x = _mm256_mullo_epi32(u[11], cospi32);
        y = _mm256_mullo_epi32(u[12], cospi32);
        v[11] = _mm256_sub_epi32(y, x);
        v[11] = _mm256_add_epi32(v[11], rnding);
        v[11] = _mm256_sra_epi32(v[11], _mm_cvtsi32_si128(bit));

        v[12] = _mm256_add_epi32(x, y);
        v[12] = _mm256_add_epi32(v[12], rnding);
        v[12] = _mm256_sra_epi32(v[12], _mm_cvtsi32_si128(bit));

        v[14] = u[14];
        v[15] = u[15];

        // stage 7
        addsub_avx2(v[0], v[15], out.add(0), out.add(15), &clamp_lo, &clamp_hi);
        addsub_avx2(v[1], v[14], out.add(1), out.add(14), &clamp_lo, &clamp_hi);
        addsub_avx2(v[2], v[13], out.add(2), out.add(13), &clamp_lo, &clamp_hi);
        addsub_avx2(v[3], v[12], out.add(3), out.add(12), &clamp_lo, &clamp_hi);
        addsub_avx2(v[4], v[11], out.add(4), out.add(11), &clamp_lo, &clamp_hi);
        addsub_avx2(v[5], v[10], out.add(5), out.add(10), &clamp_lo, &clamp_hi);
        addsub_avx2(v[6], v[9], out.add(6), out.add(9), &clamp_lo, &clamp_hi);
        addsub_avx2(v[7], v[8], out.add(7), out.add(8), &clamp_lo, &clamp_hi);

        if do_cols == 0 {
            let mut log_range_out: i32 = get_log_range_out(bd);
            let mut clamp_lo_out: __m256i = _mm256_set1_epi32(-(1 << (log_range_out - 1)));
            let mut clamp_hi_out: __m256i = _mm256_set1_epi32((1 << (log_range_out - 1)) - 1);
            round_shift_8x8_avx2(out, out_shift);
            highbd_clamp_epi32_avx2(out, out, &clamp_lo_out, &clamp_hi_out, 16);
        }
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn iadst16_low1_avx2(
    mut in_: *mut __m256i,
    mut out: *mut __m256i,
    mut bit: i32,
    mut do_cols: i32,
    mut bd: i32,
    mut out_shift: i32,
) {
    let mut cospi: *const i32 = cospi_arr(bit);
    let mut cospi2: __m256i = _mm256_set1_epi32(*cospi.add(2));
    let mut cospi62: __m256i = _mm256_set1_epi32(*cospi.add(62));
    let mut cospi8: __m256i = _mm256_set1_epi32(*cospi.add(8));
    let mut cospi56: __m256i = _mm256_set1_epi32(*cospi.add(56));
    let mut cospi48: __m256i = _mm256_set1_epi32(*cospi.add(48));
    let mut cospi16: __m256i = _mm256_set1_epi32(*cospi.add(16));
    let mut cospi32: __m256i = _mm256_set1_epi32(*cospi.add(32));
    let mut rnding: __m256i = _mm256_set1_epi32(1 << (bit - 1));
    let mut zero: __m256i = _mm256_setzero_si256();
    let mut v: [__m256i; 16] = core::mem::zeroed();
    let mut x: __m256i;
    let mut y: __m256i;
    let mut temp1: __m256i;
    let mut temp2: __m256i;

    // Calculate the column 0, 1, 2, 3
    {
        // stage 0
        // stage 1
        // stage 2
        x = _mm256_mullo_epi32(*in_.add(0), cospi62);
        v[0] = _mm256_add_epi32(x, rnding);
        v[0] = _mm256_sra_epi32(v[0], _mm_cvtsi32_si128(bit));

        x = _mm256_mullo_epi32(*in_.add(0), cospi2);
        v[1] = _mm256_sub_epi32(zero, x);
        v[1] = _mm256_add_epi32(v[1], rnding);
        v[1] = _mm256_sra_epi32(v[1], _mm_cvtsi32_si128(bit));

        // stage 3
        v[8] = v[0];
        v[9] = v[1];

        // stage 4
        temp1 = _mm256_mullo_epi32(v[8], cospi8);
        x = _mm256_mullo_epi32(v[9], cospi56);
        temp1 = _mm256_add_epi32(temp1, x);
        temp1 = _mm256_add_epi32(temp1, rnding);
        temp1 = _mm256_sra_epi32(temp1, _mm_cvtsi32_si128(bit));

        temp2 = _mm256_mullo_epi32(v[8], cospi56);
        x = _mm256_mullo_epi32(v[9], cospi8);
        temp2 = _mm256_sub_epi32(temp2, x);
        temp2 = _mm256_add_epi32(temp2, rnding);
        temp2 = _mm256_sra_epi32(temp2, _mm_cvtsi32_si128(bit));
        v[8] = temp1;
        v[9] = temp2;

        // stage 5
        v[4] = v[0];
        v[5] = v[1];
        v[12] = v[8];
        v[13] = v[9];

        // stage 6
        temp1 = _mm256_mullo_epi32(v[4], cospi16);
        x = _mm256_mullo_epi32(v[5], cospi48);
        temp1 = _mm256_add_epi32(temp1, x);
        temp1 = _mm256_add_epi32(temp1, rnding);
        temp1 = _mm256_sra_epi32(temp1, _mm_cvtsi32_si128(bit));

        temp2 = _mm256_mullo_epi32(v[4], cospi48);
        x = _mm256_mullo_epi32(v[5], cospi16);
        temp2 = _mm256_sub_epi32(temp2, x);
        temp2 = _mm256_add_epi32(temp2, rnding);
        temp2 = _mm256_sra_epi32(temp2, _mm_cvtsi32_si128(bit));
        v[4] = temp1;
        v[5] = temp2;

        temp1 = _mm256_mullo_epi32(v[12], cospi16);
        x = _mm256_mullo_epi32(v[13], cospi48);
        temp1 = _mm256_add_epi32(temp1, x);
        temp1 = _mm256_add_epi32(temp1, rnding);
        temp1 = _mm256_sra_epi32(temp1, _mm_cvtsi32_si128(bit));

        temp2 = _mm256_mullo_epi32(v[12], cospi48);
        x = _mm256_mullo_epi32(v[13], cospi16);
        temp2 = _mm256_sub_epi32(temp2, x);
        temp2 = _mm256_add_epi32(temp2, rnding);
        temp2 = _mm256_sra_epi32(temp2, _mm_cvtsi32_si128(bit));
        v[12] = temp1;
        v[13] = temp2;

        // stage 7
        v[2] = v[0];
        v[3] = v[1];
        v[6] = v[4];
        v[7] = v[5];
        v[10] = v[8];
        v[11] = v[9];
        v[14] = v[12];
        v[15] = v[13];

        // stage 8
        y = _mm256_mullo_epi32(v[2], cospi32);
        x = _mm256_mullo_epi32(v[3], cospi32);
        v[2] = _mm256_add_epi32(y, x);
        v[2] = _mm256_add_epi32(v[2], rnding);
        v[2] = _mm256_sra_epi32(v[2], _mm_cvtsi32_si128(bit));

        v[3] = _mm256_sub_epi32(y, x);
        v[3] = _mm256_add_epi32(v[3], rnding);
        v[3] = _mm256_sra_epi32(v[3], _mm_cvtsi32_si128(bit));

        y = _mm256_mullo_epi32(v[6], cospi32);
        x = _mm256_mullo_epi32(v[7], cospi32);
        v[6] = _mm256_add_epi32(y, x);
        v[6] = _mm256_add_epi32(v[6], rnding);
        v[6] = _mm256_sra_epi32(v[6], _mm_cvtsi32_si128(bit));

        v[7] = _mm256_sub_epi32(y, x);
        v[7] = _mm256_add_epi32(v[7], rnding);
        v[7] = _mm256_sra_epi32(v[7], _mm_cvtsi32_si128(bit));

        y = _mm256_mullo_epi32(v[10], cospi32);
        x = _mm256_mullo_epi32(v[11], cospi32);
        v[10] = _mm256_add_epi32(y, x);
        v[10] = _mm256_add_epi32(v[10], rnding);
        v[10] = _mm256_sra_epi32(v[10], _mm_cvtsi32_si128(bit));

        v[11] = _mm256_sub_epi32(y, x);
        v[11] = _mm256_add_epi32(v[11], rnding);
        v[11] = _mm256_sra_epi32(v[11], _mm_cvtsi32_si128(bit));

        y = _mm256_mullo_epi32(v[14], cospi32);
        x = _mm256_mullo_epi32(v[15], cospi32);
        v[14] = _mm256_add_epi32(y, x);
        v[14] = _mm256_add_epi32(v[14], rnding);
        v[14] = _mm256_sra_epi32(v[14], _mm_cvtsi32_si128(bit));

        v[15] = _mm256_sub_epi32(y, x);
        v[15] = _mm256_add_epi32(v[15], rnding);
        v[15] = _mm256_sra_epi32(v[15], _mm_cvtsi32_si128(bit));

        // stage 9
        if do_cols != 0 {
            *out.add(0) = v[0];
            *out.add(1) = _mm256_sub_epi32(_mm256_setzero_si256(), v[8]);
            *out.add(2) = v[12];
            *out.add(3) = _mm256_sub_epi32(_mm256_setzero_si256(), v[4]);
            *out.add(4) = v[6];
            *out.add(5) = _mm256_sub_epi32(_mm256_setzero_si256(), v[14]);
            *out.add(6) = v[10];
            *out.add(7) = _mm256_sub_epi32(_mm256_setzero_si256(), v[2]);
            *out.add(8) = v[3];
            *out.add(9) = _mm256_sub_epi32(_mm256_setzero_si256(), v[11]);
            *out.add(10) = v[15];
            *out.add(11) = _mm256_sub_epi32(_mm256_setzero_si256(), v[7]);
            *out.add(12) = v[5];
            *out.add(13) = _mm256_sub_epi32(_mm256_setzero_si256(), v[13]);
            *out.add(14) = v[9];
            *out.add(15) = _mm256_sub_epi32(_mm256_setzero_si256(), v[1]);
        } else {
            let mut log_range_out: i32 = get_log_range_out(bd);
            let mut clamp_lo_out: __m256i = _mm256_set1_epi32(-(1 << (log_range_out - 1)));
            let mut clamp_hi_out: __m256i = _mm256_set1_epi32((1 << (log_range_out - 1)) - 1);

            neg_shift_avx2(
                v[0],
                v[8],
                out.add(0),
                out.add(1),
                &clamp_lo_out,
                &clamp_hi_out,
                out_shift,
            );
            neg_shift_avx2(
                v[12],
                v[4],
                out.add(2),
                out.add(3),
                &clamp_lo_out,
                &clamp_hi_out,
                out_shift,
            );
            neg_shift_avx2(
                v[6],
                v[14],
                out.add(4),
                out.add(5),
                &clamp_lo_out,
                &clamp_hi_out,
                out_shift,
            );
            neg_shift_avx2(
                v[10],
                v[2],
                out.add(6),
                out.add(7),
                &clamp_lo_out,
                &clamp_hi_out,
                out_shift,
            );
            neg_shift_avx2(
                v[3],
                v[11],
                out.add(8),
                out.add(9),
                &clamp_lo_out,
                &clamp_hi_out,
                out_shift,
            );
            neg_shift_avx2(
                v[15],
                v[7],
                out.add(10),
                out.add(11),
                &clamp_lo_out,
                &clamp_hi_out,
                out_shift,
            );
            neg_shift_avx2(
                v[5],
                v[13],
                out.add(12),
                out.add(13),
                &clamp_lo_out,
                &clamp_hi_out,
                out_shift,
            );
            neg_shift_avx2(
                v[9],
                v[1],
                out.add(14),
                out.add(15),
                &clamp_lo_out,
                &clamp_hi_out,
                out_shift,
            );
        }
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn iadst16_low8_avx2(
    mut in_: *mut __m256i,
    mut out: *mut __m256i,
    mut bit: i32,
    mut do_cols: i32,
    mut bd: i32,
    mut out_shift: i32,
) {
    let mut cospi: *const i32 = cospi_arr(bit);
    let mut cospi2: __m256i = _mm256_set1_epi32(*cospi.add(2));
    let mut cospi62: __m256i = _mm256_set1_epi32(*cospi.add(62));
    let mut cospi10: __m256i = _mm256_set1_epi32(*cospi.add(10));
    let mut cospi54: __m256i = _mm256_set1_epi32(*cospi.add(54));
    let mut cospi18: __m256i = _mm256_set1_epi32(*cospi.add(18));
    let mut cospi46: __m256i = _mm256_set1_epi32(*cospi.add(46));
    let mut cospi26: __m256i = _mm256_set1_epi32(*cospi.add(26));
    let mut cospi38: __m256i = _mm256_set1_epi32(*cospi.add(38));
    let mut cospi34: __m256i = _mm256_set1_epi32(*cospi.add(34));
    let mut cospi30: __m256i = _mm256_set1_epi32(*cospi.add(30));
    let mut cospi42: __m256i = _mm256_set1_epi32(*cospi.add(42));
    let mut cospi22: __m256i = _mm256_set1_epi32(*cospi.add(22));
    let mut cospi50: __m256i = _mm256_set1_epi32(*cospi.add(50));
    let mut cospi14: __m256i = _mm256_set1_epi32(*cospi.add(14));
    let mut cospi58: __m256i = _mm256_set1_epi32(*cospi.add(58));
    let mut cospi6: __m256i = _mm256_set1_epi32(*cospi.add(6));
    let mut cospi8: __m256i = _mm256_set1_epi32(*cospi.add(8));
    let mut cospi56: __m256i = _mm256_set1_epi32(*cospi.add(56));
    let mut cospi40: __m256i = _mm256_set1_epi32(*cospi.add(40));
    let mut cospi24: __m256i = _mm256_set1_epi32(*cospi.add(24));
    let mut cospim56: __m256i = _mm256_set1_epi32(-*cospi.add(56));
    let mut cospim24: __m256i = _mm256_set1_epi32(-*cospi.add(24));
    let mut cospi48: __m256i = _mm256_set1_epi32(*cospi.add(48));
    let mut cospi16: __m256i = _mm256_set1_epi32(*cospi.add(16));
    let mut cospim48: __m256i = _mm256_set1_epi32(-*cospi.add(48));
    let mut cospi32: __m256i = _mm256_set1_epi32(*cospi.add(32));
    let mut rnding: __m256i = _mm256_set1_epi32(1 << (bit - 1));
    let mut log_range: i32 = get_log_range(bd, do_cols);
    let mut clamp_lo: __m256i = _mm256_set1_epi32(-(1 << (log_range - 1)));
    let mut clamp_hi: __m256i = _mm256_set1_epi32((1 << (log_range - 1)) - 1);
    let mut u: [__m256i; 16] = core::mem::zeroed();
    let mut x: __m256i;
    let mut y: __m256i;

    {
        // stage 0
        // stage 1
        // stage 2
        let mut zero: __m256i = _mm256_setzero_si256();
        x = _mm256_mullo_epi32(*in_.add(0), cospi62);
        u[0] = _mm256_add_epi32(x, rnding);
        u[0] = _mm256_sra_epi32(u[0], _mm_cvtsi32_si128(bit));

        x = _mm256_mullo_epi32(*in_.add(0), cospi2);
        u[1] = _mm256_sub_epi32(zero, x);
        u[1] = _mm256_add_epi32(u[1], rnding);
        u[1] = _mm256_sra_epi32(u[1], _mm_cvtsi32_si128(bit));

        x = _mm256_mullo_epi32(*in_.add(2), cospi54);
        u[2] = _mm256_add_epi32(x, rnding);
        u[2] = _mm256_sra_epi32(u[2], _mm_cvtsi32_si128(bit));

        x = _mm256_mullo_epi32(*in_.add(2), cospi10);
        u[3] = _mm256_sub_epi32(zero, x);
        u[3] = _mm256_add_epi32(u[3], rnding);
        u[3] = _mm256_sra_epi32(u[3], _mm_cvtsi32_si128(bit));

        x = _mm256_mullo_epi32(*in_.add(4), cospi46);
        u[4] = _mm256_add_epi32(x, rnding);
        u[4] = _mm256_sra_epi32(u[4], _mm_cvtsi32_si128(bit));

        x = _mm256_mullo_epi32(*in_.add(4), cospi18);
        u[5] = _mm256_sub_epi32(zero, x);
        u[5] = _mm256_add_epi32(u[5], rnding);
        u[5] = _mm256_sra_epi32(u[5], _mm_cvtsi32_si128(bit));

        x = _mm256_mullo_epi32(*in_.add(6), cospi38);
        u[6] = _mm256_add_epi32(x, rnding);
        u[6] = _mm256_sra_epi32(u[6], _mm_cvtsi32_si128(bit));

        x = _mm256_mullo_epi32(*in_.add(6), cospi26);
        u[7] = _mm256_sub_epi32(zero, x);
        u[7] = _mm256_add_epi32(u[7], rnding);
        u[7] = _mm256_sra_epi32(u[7], _mm_cvtsi32_si128(bit));

        u[8] = _mm256_mullo_epi32(*in_.add(7), cospi34);
        u[8] = _mm256_add_epi32(u[8], rnding);
        u[8] = _mm256_sra_epi32(u[8], _mm_cvtsi32_si128(bit));

        u[9] = _mm256_mullo_epi32(*in_.add(7), cospi30);
        u[9] = _mm256_add_epi32(u[9], rnding);
        u[9] = _mm256_sra_epi32(u[9], _mm_cvtsi32_si128(bit));

        u[10] = _mm256_mullo_epi32(*in_.add(5), cospi42);
        u[10] = _mm256_add_epi32(u[10], rnding);
        u[10] = _mm256_sra_epi32(u[10], _mm_cvtsi32_si128(bit));

        u[11] = _mm256_mullo_epi32(*in_.add(5), cospi22);
        u[11] = _mm256_add_epi32(u[11], rnding);
        u[11] = _mm256_sra_epi32(u[11], _mm_cvtsi32_si128(bit));

        u[12] = _mm256_mullo_epi32(*in_.add(3), cospi50);
        u[12] = _mm256_add_epi32(u[12], rnding);
        u[12] = _mm256_sra_epi32(u[12], _mm_cvtsi32_si128(bit));

        u[13] = _mm256_mullo_epi32(*in_.add(3), cospi14);
        u[13] = _mm256_add_epi32(u[13], rnding);
        u[13] = _mm256_sra_epi32(u[13], _mm_cvtsi32_si128(bit));

        u[14] = _mm256_mullo_epi32(*in_.add(1), cospi58);
        u[14] = _mm256_add_epi32(u[14], rnding);
        u[14] = _mm256_sra_epi32(u[14], _mm_cvtsi32_si128(bit));

        u[15] = _mm256_mullo_epi32(*in_.add(1), cospi6);
        u[15] = _mm256_add_epi32(u[15], rnding);
        u[15] = _mm256_sra_epi32(u[15], _mm_cvtsi32_si128(bit));

        // stage 3
        addsub_avx2(u[0], u[8], u.as_mut_ptr().add(0), u.as_mut_ptr().add(8), &clamp_lo, &clamp_hi);
        addsub_avx2(u[1], u[9], u.as_mut_ptr().add(1), u.as_mut_ptr().add(9), &clamp_lo, &clamp_hi);
        addsub_avx2(
            u[2],
            u[10],
            u.as_mut_ptr().add(2),
            u.as_mut_ptr().add(10),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            u[3],
            u[11],
            u.as_mut_ptr().add(3),
            u.as_mut_ptr().add(11),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            u[4],
            u[12],
            u.as_mut_ptr().add(4),
            u.as_mut_ptr().add(12),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            u[5],
            u[13],
            u.as_mut_ptr().add(5),
            u.as_mut_ptr().add(13),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            u[6],
            u[14],
            u.as_mut_ptr().add(6),
            u.as_mut_ptr().add(14),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            u[7],
            u[15],
            u.as_mut_ptr().add(7),
            u.as_mut_ptr().add(15),
            &clamp_lo,
            &clamp_hi,
        );

        // stage 4
        y = _mm256_mullo_epi32(u[8], cospi56);
        x = _mm256_mullo_epi32(u[9], cospi56);
        u[8] = _mm256_mullo_epi32(u[8], cospi8);
        u[8] = _mm256_add_epi32(u[8], x);
        u[8] = _mm256_add_epi32(u[8], rnding);
        u[8] = _mm256_sra_epi32(u[8], _mm_cvtsi32_si128(bit));

        x = _mm256_mullo_epi32(u[9], cospi8);
        u[9] = _mm256_sub_epi32(y, x);
        u[9] = _mm256_add_epi32(u[9], rnding);
        u[9] = _mm256_sra_epi32(u[9], _mm_cvtsi32_si128(bit));

        x = _mm256_mullo_epi32(u[11], cospi24);
        y = _mm256_mullo_epi32(u[10], cospi24);
        u[10] = _mm256_mullo_epi32(u[10], cospi40);
        u[10] = _mm256_add_epi32(u[10], x);
        u[10] = _mm256_add_epi32(u[10], rnding);
        u[10] = _mm256_sra_epi32(u[10], _mm_cvtsi32_si128(bit));

        x = _mm256_mullo_epi32(u[11], cospi40);
        u[11] = _mm256_sub_epi32(y, x);
        u[11] = _mm256_add_epi32(u[11], rnding);
        u[11] = _mm256_sra_epi32(u[11], _mm_cvtsi32_si128(bit));

        x = _mm256_mullo_epi32(u[13], cospi8);
        y = _mm256_mullo_epi32(u[12], cospi8);
        u[12] = _mm256_mullo_epi32(u[12], cospim56);
        u[12] = _mm256_add_epi32(u[12], x);
        u[12] = _mm256_add_epi32(u[12], rnding);
        u[12] = _mm256_sra_epi32(u[12], _mm_cvtsi32_si128(bit));

        x = _mm256_mullo_epi32(u[13], cospim56);
        u[13] = _mm256_sub_epi32(y, x);
        u[13] = _mm256_add_epi32(u[13], rnding);
        u[13] = _mm256_sra_epi32(u[13], _mm_cvtsi32_si128(bit));

        x = _mm256_mullo_epi32(u[15], cospi40);
        y = _mm256_mullo_epi32(u[14], cospi40);
        u[14] = _mm256_mullo_epi32(u[14], cospim24);
        u[14] = _mm256_add_epi32(u[14], x);
        u[14] = _mm256_add_epi32(u[14], rnding);
        u[14] = _mm256_sra_epi32(u[14], _mm_cvtsi32_si128(bit));

        x = _mm256_mullo_epi32(u[15], cospim24);
        u[15] = _mm256_sub_epi32(y, x);
        u[15] = _mm256_add_epi32(u[15], rnding);
        u[15] = _mm256_sra_epi32(u[15], _mm_cvtsi32_si128(bit));

        // stage 5
        addsub_avx2(u[0], u[4], u.as_mut_ptr().add(0), u.as_mut_ptr().add(4), &clamp_lo, &clamp_hi);
        addsub_avx2(u[1], u[5], u.as_mut_ptr().add(1), u.as_mut_ptr().add(5), &clamp_lo, &clamp_hi);
        addsub_avx2(u[2], u[6], u.as_mut_ptr().add(2), u.as_mut_ptr().add(6), &clamp_lo, &clamp_hi);
        addsub_avx2(u[3], u[7], u.as_mut_ptr().add(3), u.as_mut_ptr().add(7), &clamp_lo, &clamp_hi);
        addsub_avx2(
            u[8],
            u[12],
            u.as_mut_ptr().add(8),
            u.as_mut_ptr().add(12),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            u[9],
            u[13],
            u.as_mut_ptr().add(9),
            u.as_mut_ptr().add(13),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            u[10],
            u[14],
            u.as_mut_ptr().add(10),
            u.as_mut_ptr().add(14),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            u[11],
            u[15],
            u.as_mut_ptr().add(11),
            u.as_mut_ptr().add(15),
            &clamp_lo,
            &clamp_hi,
        );

        // stage 6
        x = _mm256_mullo_epi32(u[5], cospi48);
        y = _mm256_mullo_epi32(u[4], cospi48);
        u[4] = _mm256_mullo_epi32(u[4], cospi16);
        u[4] = _mm256_add_epi32(u[4], x);
        u[4] = _mm256_add_epi32(u[4], rnding);
        u[4] = _mm256_sra_epi32(u[4], _mm_cvtsi32_si128(bit));

        x = _mm256_mullo_epi32(u[5], cospi16);
        u[5] = _mm256_sub_epi32(y, x);
        u[5] = _mm256_add_epi32(u[5], rnding);
        u[5] = _mm256_sra_epi32(u[5], _mm_cvtsi32_si128(bit));

        x = _mm256_mullo_epi32(u[7], cospi16);
        y = _mm256_mullo_epi32(u[6], cospi16);
        u[6] = _mm256_mullo_epi32(u[6], cospim48);
        u[6] = _mm256_add_epi32(u[6], x);
        u[6] = _mm256_add_epi32(u[6], rnding);
        u[6] = _mm256_sra_epi32(u[6], _mm_cvtsi32_si128(bit));

        x = _mm256_mullo_epi32(u[7], cospim48);
        u[7] = _mm256_sub_epi32(y, x);
        u[7] = _mm256_add_epi32(u[7], rnding);
        u[7] = _mm256_sra_epi32(u[7], _mm_cvtsi32_si128(bit));

        x = _mm256_mullo_epi32(u[13], cospi48);
        y = _mm256_mullo_epi32(u[12], cospi48);
        u[12] = _mm256_mullo_epi32(u[12], cospi16);
        u[12] = _mm256_add_epi32(u[12], x);
        u[12] = _mm256_add_epi32(u[12], rnding);
        u[12] = _mm256_sra_epi32(u[12], _mm_cvtsi32_si128(bit));

        x = _mm256_mullo_epi32(u[13], cospi16);
        u[13] = _mm256_sub_epi32(y, x);
        u[13] = _mm256_add_epi32(u[13], rnding);
        u[13] = _mm256_sra_epi32(u[13], _mm_cvtsi32_si128(bit));

        x = _mm256_mullo_epi32(u[15], cospi16);
        y = _mm256_mullo_epi32(u[14], cospi16);
        u[14] = _mm256_mullo_epi32(u[14], cospim48);
        u[14] = _mm256_add_epi32(u[14], x);
        u[14] = _mm256_add_epi32(u[14], rnding);
        u[14] = _mm256_sra_epi32(u[14], _mm_cvtsi32_si128(bit));

        x = _mm256_mullo_epi32(u[15], cospim48);
        u[15] = _mm256_sub_epi32(y, x);
        u[15] = _mm256_add_epi32(u[15], rnding);
        u[15] = _mm256_sra_epi32(u[15], _mm_cvtsi32_si128(bit));

        // stage 7
        addsub_avx2(u[0], u[2], u.as_mut_ptr().add(0), u.as_mut_ptr().add(2), &clamp_lo, &clamp_hi);
        addsub_avx2(u[1], u[3], u.as_mut_ptr().add(1), u.as_mut_ptr().add(3), &clamp_lo, &clamp_hi);
        addsub_avx2(u[4], u[6], u.as_mut_ptr().add(4), u.as_mut_ptr().add(6), &clamp_lo, &clamp_hi);
        addsub_avx2(u[5], u[7], u.as_mut_ptr().add(5), u.as_mut_ptr().add(7), &clamp_lo, &clamp_hi);
        addsub_avx2(
            u[8],
            u[10],
            u.as_mut_ptr().add(8),
            u.as_mut_ptr().add(10),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            u[9],
            u[11],
            u.as_mut_ptr().add(9),
            u.as_mut_ptr().add(11),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            u[12],
            u[14],
            u.as_mut_ptr().add(12),
            u.as_mut_ptr().add(14),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            u[13],
            u[15],
            u.as_mut_ptr().add(13),
            u.as_mut_ptr().add(15),
            &clamp_lo,
            &clamp_hi,
        );

        // stage 8
        y = _mm256_mullo_epi32(u[2], cospi32);
        x = _mm256_mullo_epi32(u[3], cospi32);
        u[2] = _mm256_add_epi32(y, x);
        u[2] = _mm256_add_epi32(u[2], rnding);
        u[2] = _mm256_sra_epi32(u[2], _mm_cvtsi32_si128(bit));

        u[3] = _mm256_sub_epi32(y, x);
        u[3] = _mm256_add_epi32(u[3], rnding);
        u[3] = _mm256_sra_epi32(u[3], _mm_cvtsi32_si128(bit));
        y = _mm256_mullo_epi32(u[6], cospi32);
        x = _mm256_mullo_epi32(u[7], cospi32);
        u[6] = _mm256_add_epi32(y, x);
        u[6] = _mm256_add_epi32(u[6], rnding);
        u[6] = _mm256_sra_epi32(u[6], _mm_cvtsi32_si128(bit));

        u[7] = _mm256_sub_epi32(y, x);
        u[7] = _mm256_add_epi32(u[7], rnding);
        u[7] = _mm256_sra_epi32(u[7], _mm_cvtsi32_si128(bit));

        y = _mm256_mullo_epi32(u[10], cospi32);
        x = _mm256_mullo_epi32(u[11], cospi32);
        u[10] = _mm256_add_epi32(y, x);
        u[10] = _mm256_add_epi32(u[10], rnding);
        u[10] = _mm256_sra_epi32(u[10], _mm_cvtsi32_si128(bit));

        u[11] = _mm256_sub_epi32(y, x);
        u[11] = _mm256_add_epi32(u[11], rnding);
        u[11] = _mm256_sra_epi32(u[11], _mm_cvtsi32_si128(bit));

        y = _mm256_mullo_epi32(u[14], cospi32);
        x = _mm256_mullo_epi32(u[15], cospi32);
        u[14] = _mm256_add_epi32(y, x);
        u[14] = _mm256_add_epi32(u[14], rnding);
        u[14] = _mm256_sra_epi32(u[14], _mm_cvtsi32_si128(bit));

        u[15] = _mm256_sub_epi32(y, x);
        u[15] = _mm256_add_epi32(u[15], rnding);
        u[15] = _mm256_sra_epi32(u[15], _mm_cvtsi32_si128(bit));

        // stage 9
        if do_cols != 0 {
            *out.add(0) = u[0];
            *out.add(1) = _mm256_sub_epi32(_mm256_setzero_si256(), u[8]);
            *out.add(2) = u[12];
            *out.add(3) = _mm256_sub_epi32(_mm256_setzero_si256(), u[4]);
            *out.add(4) = u[6];
            *out.add(5) = _mm256_sub_epi32(_mm256_setzero_si256(), u[14]);
            *out.add(6) = u[10];
            *out.add(7) = _mm256_sub_epi32(_mm256_setzero_si256(), u[2]);
            *out.add(8) = u[3];
            *out.add(9) = _mm256_sub_epi32(_mm256_setzero_si256(), u[11]);
            *out.add(10) = u[15];
            *out.add(11) = _mm256_sub_epi32(_mm256_setzero_si256(), u[7]);
            *out.add(12) = u[5];
            *out.add(13) = _mm256_sub_epi32(_mm256_setzero_si256(), u[13]);
            *out.add(14) = u[9];
            *out.add(15) = _mm256_sub_epi32(_mm256_setzero_si256(), u[1]);
        } else {
            let mut log_range_out: i32 = get_log_range_out(bd);
            let mut clamp_lo_out: __m256i = _mm256_set1_epi32(-(1 << (log_range_out - 1)));
            let mut clamp_hi_out: __m256i = _mm256_set1_epi32((1 << (log_range_out - 1)) - 1);

            neg_shift_avx2(
                u[0],
                u[8],
                out.add(0),
                out.add(1),
                &clamp_lo_out,
                &clamp_hi_out,
                out_shift,
            );
            neg_shift_avx2(
                u[12],
                u[4],
                out.add(2),
                out.add(3),
                &clamp_lo_out,
                &clamp_hi_out,
                out_shift,
            );
            neg_shift_avx2(
                u[6],
                u[14],
                out.add(4),
                out.add(5),
                &clamp_lo_out,
                &clamp_hi_out,
                out_shift,
            );
            neg_shift_avx2(
                u[10],
                u[2],
                out.add(6),
                out.add(7),
                &clamp_lo_out,
                &clamp_hi_out,
                out_shift,
            );
            neg_shift_avx2(
                u[3],
                u[11],
                out.add(8),
                out.add(9),
                &clamp_lo_out,
                &clamp_hi_out,
                out_shift,
            );
            neg_shift_avx2(
                u[15],
                u[7],
                out.add(10),
                out.add(11),
                &clamp_lo_out,
                &clamp_hi_out,
                out_shift,
            );
            neg_shift_avx2(
                u[5],
                u[13],
                out.add(12),
                out.add(13),
                &clamp_lo_out,
                &clamp_hi_out,
                out_shift,
            );
            neg_shift_avx2(
                u[9],
                u[1],
                out.add(14),
                out.add(15),
                &clamp_lo_out,
                &clamp_hi_out,
                out_shift,
            );
        }
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn iadst16_avx2(
    mut in_: *mut __m256i,
    mut out: *mut __m256i,
    mut bit: i32,
    mut do_cols: i32,
    mut bd: i32,
    mut out_shift: i32,
) {
    let mut cospi: *const i32 = cospi_arr(bit);
    let mut cospi2: __m256i = _mm256_set1_epi32(*cospi.add(2));
    let mut cospi62: __m256i = _mm256_set1_epi32(*cospi.add(62));
    let mut cospi10: __m256i = _mm256_set1_epi32(*cospi.add(10));
    let mut cospi54: __m256i = _mm256_set1_epi32(*cospi.add(54));
    let mut cospi18: __m256i = _mm256_set1_epi32(*cospi.add(18));
    let mut cospi46: __m256i = _mm256_set1_epi32(*cospi.add(46));
    let mut cospi26: __m256i = _mm256_set1_epi32(*cospi.add(26));
    let mut cospi38: __m256i = _mm256_set1_epi32(*cospi.add(38));
    let mut cospi34: __m256i = _mm256_set1_epi32(*cospi.add(34));
    let mut cospi30: __m256i = _mm256_set1_epi32(*cospi.add(30));
    let mut cospi42: __m256i = _mm256_set1_epi32(*cospi.add(42));
    let mut cospi22: __m256i = _mm256_set1_epi32(*cospi.add(22));
    let mut cospi50: __m256i = _mm256_set1_epi32(*cospi.add(50));
    let mut cospi14: __m256i = _mm256_set1_epi32(*cospi.add(14));
    let mut cospi58: __m256i = _mm256_set1_epi32(*cospi.add(58));
    let mut cospi6: __m256i = _mm256_set1_epi32(*cospi.add(6));
    let mut cospi8: __m256i = _mm256_set1_epi32(*cospi.add(8));
    let mut cospi56: __m256i = _mm256_set1_epi32(*cospi.add(56));
    let mut cospi40: __m256i = _mm256_set1_epi32(*cospi.add(40));
    let mut cospi24: __m256i = _mm256_set1_epi32(*cospi.add(24));
    let mut cospim56: __m256i = _mm256_set1_epi32(-*cospi.add(56));
    let mut cospim24: __m256i = _mm256_set1_epi32(-*cospi.add(24));
    let mut cospi48: __m256i = _mm256_set1_epi32(*cospi.add(48));
    let mut cospi16: __m256i = _mm256_set1_epi32(*cospi.add(16));
    let mut cospim48: __m256i = _mm256_set1_epi32(-*cospi.add(48));
    let mut cospi32: __m256i = _mm256_set1_epi32(*cospi.add(32));
    let mut rnding: __m256i = _mm256_set1_epi32(1 << (bit - 1));
    let mut log_range: i32 = get_log_range(bd, do_cols);
    let mut clamp_lo: __m256i = _mm256_set1_epi32(-(1 << (log_range - 1)));
    let mut clamp_hi: __m256i = _mm256_set1_epi32((1 << (log_range - 1)) - 1);
    let mut u: [__m256i; 16] = core::mem::zeroed();
    let mut v: [__m256i; 16] = core::mem::zeroed();
    let mut x: __m256i;
    let mut y: __m256i;

    {
        // stage 0
        // stage 1
        // stage 2
        v[0] = _mm256_mullo_epi32(*in_.add(15), cospi2);
        x = _mm256_mullo_epi32(*in_.add(0), cospi62);
        v[0] = _mm256_add_epi32(v[0], x);
        v[0] = _mm256_add_epi32(v[0], rnding);
        v[0] = _mm256_sra_epi32(v[0], _mm_cvtsi32_si128(bit));

        v[1] = _mm256_mullo_epi32(*in_.add(15), cospi62);
        x = _mm256_mullo_epi32(*in_.add(0), cospi2);
        v[1] = _mm256_sub_epi32(v[1], x);
        v[1] = _mm256_add_epi32(v[1], rnding);
        v[1] = _mm256_sra_epi32(v[1], _mm_cvtsi32_si128(bit));

        v[2] = _mm256_mullo_epi32(*in_.add(13), cospi10);
        x = _mm256_mullo_epi32(*in_.add(2), cospi54);
        v[2] = _mm256_add_epi32(v[2], x);
        v[2] = _mm256_add_epi32(v[2], rnding);
        v[2] = _mm256_sra_epi32(v[2], _mm_cvtsi32_si128(bit));

        v[3] = _mm256_mullo_epi32(*in_.add(13), cospi54);
        x = _mm256_mullo_epi32(*in_.add(2), cospi10);
        v[3] = _mm256_sub_epi32(v[3], x);
        v[3] = _mm256_add_epi32(v[3], rnding);
        v[3] = _mm256_sra_epi32(v[3], _mm_cvtsi32_si128(bit));

        v[4] = _mm256_mullo_epi32(*in_.add(11), cospi18);
        x = _mm256_mullo_epi32(*in_.add(4), cospi46);
        v[4] = _mm256_add_epi32(v[4], x);
        v[4] = _mm256_add_epi32(v[4], rnding);
        v[4] = _mm256_sra_epi32(v[4], _mm_cvtsi32_si128(bit));

        v[5] = _mm256_mullo_epi32(*in_.add(11), cospi46);
        x = _mm256_mullo_epi32(*in_.add(4), cospi18);
        v[5] = _mm256_sub_epi32(v[5], x);
        v[5] = _mm256_add_epi32(v[5], rnding);
        v[5] = _mm256_sra_epi32(v[5], _mm_cvtsi32_si128(bit));

        v[6] = _mm256_mullo_epi32(*in_.add(9), cospi26);
        x = _mm256_mullo_epi32(*in_.add(6), cospi38);
        v[6] = _mm256_add_epi32(v[6], x);
        v[6] = _mm256_add_epi32(v[6], rnding);
        v[6] = _mm256_sra_epi32(v[6], _mm_cvtsi32_si128(bit));

        v[7] = _mm256_mullo_epi32(*in_.add(9), cospi38);
        x = _mm256_mullo_epi32(*in_.add(6), cospi26);
        v[7] = _mm256_sub_epi32(v[7], x);
        v[7] = _mm256_add_epi32(v[7], rnding);
        v[7] = _mm256_sra_epi32(v[7], _mm_cvtsi32_si128(bit));

        v[8] = _mm256_mullo_epi32(*in_.add(7), cospi34);
        x = _mm256_mullo_epi32(*in_.add(8), cospi30);
        v[8] = _mm256_add_epi32(v[8], x);
        v[8] = _mm256_add_epi32(v[8], rnding);
        v[8] = _mm256_sra_epi32(v[8], _mm_cvtsi32_si128(bit));

        v[9] = _mm256_mullo_epi32(*in_.add(7), cospi30);
        x = _mm256_mullo_epi32(*in_.add(8), cospi34);
        v[9] = _mm256_sub_epi32(v[9], x);
        v[9] = _mm256_add_epi32(v[9], rnding);
        v[9] = _mm256_sra_epi32(v[9], _mm_cvtsi32_si128(bit));

        v[10] = _mm256_mullo_epi32(*in_.add(5), cospi42);
        x = _mm256_mullo_epi32(*in_.add(10), cospi22);
        v[10] = _mm256_add_epi32(v[10], x);
        v[10] = _mm256_add_epi32(v[10], rnding);
        v[10] = _mm256_sra_epi32(v[10], _mm_cvtsi32_si128(bit));

        v[11] = _mm256_mullo_epi32(*in_.add(5), cospi22);
        x = _mm256_mullo_epi32(*in_.add(10), cospi42);
        v[11] = _mm256_sub_epi32(v[11], x);
        v[11] = _mm256_add_epi32(v[11], rnding);
        v[11] = _mm256_sra_epi32(v[11], _mm_cvtsi32_si128(bit));

        v[12] = _mm256_mullo_epi32(*in_.add(3), cospi50);
        x = _mm256_mullo_epi32(*in_.add(12), cospi14);
        v[12] = _mm256_add_epi32(v[12], x);
        v[12] = _mm256_add_epi32(v[12], rnding);
        v[12] = _mm256_sra_epi32(v[12], _mm_cvtsi32_si128(bit));

        v[13] = _mm256_mullo_epi32(*in_.add(3), cospi14);
        x = _mm256_mullo_epi32(*in_.add(12), cospi50);
        v[13] = _mm256_sub_epi32(v[13], x);
        v[13] = _mm256_add_epi32(v[13], rnding);
        v[13] = _mm256_sra_epi32(v[13], _mm_cvtsi32_si128(bit));

        v[14] = _mm256_mullo_epi32(*in_.add(1), cospi58);
        x = _mm256_mullo_epi32(*in_.add(14), cospi6);
        v[14] = _mm256_add_epi32(v[14], x);
        v[14] = _mm256_add_epi32(v[14], rnding);
        v[14] = _mm256_sra_epi32(v[14], _mm_cvtsi32_si128(bit));

        v[15] = _mm256_mullo_epi32(*in_.add(1), cospi6);
        x = _mm256_mullo_epi32(*in_.add(14), cospi58);
        v[15] = _mm256_sub_epi32(v[15], x);
        v[15] = _mm256_add_epi32(v[15], rnding);
        v[15] = _mm256_sra_epi32(v[15], _mm_cvtsi32_si128(bit));

        // stage 3
        addsub_avx2(v[0], v[8], u.as_mut_ptr().add(0), u.as_mut_ptr().add(8), &clamp_lo, &clamp_hi);
        addsub_avx2(v[1], v[9], u.as_mut_ptr().add(1), u.as_mut_ptr().add(9), &clamp_lo, &clamp_hi);
        addsub_avx2(
            v[2],
            v[10],
            u.as_mut_ptr().add(2),
            u.as_mut_ptr().add(10),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            v[3],
            v[11],
            u.as_mut_ptr().add(3),
            u.as_mut_ptr().add(11),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            v[4],
            v[12],
            u.as_mut_ptr().add(4),
            u.as_mut_ptr().add(12),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            v[5],
            v[13],
            u.as_mut_ptr().add(5),
            u.as_mut_ptr().add(13),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            v[6],
            v[14],
            u.as_mut_ptr().add(6),
            u.as_mut_ptr().add(14),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            v[7],
            v[15],
            u.as_mut_ptr().add(7),
            u.as_mut_ptr().add(15),
            &clamp_lo,
            &clamp_hi,
        );

        // stage 4
        v[0] = u[0];
        v[1] = u[1];
        v[2] = u[2];
        v[3] = u[3];
        v[4] = u[4];
        v[5] = u[5];
        v[6] = u[6];
        v[7] = u[7];

        v[8] = _mm256_mullo_epi32(u[8], cospi8);
        x = _mm256_mullo_epi32(u[9], cospi56);
        v[8] = _mm256_add_epi32(v[8], x);
        v[8] = _mm256_add_epi32(v[8], rnding);
        v[8] = _mm256_sra_epi32(v[8], _mm_cvtsi32_si128(bit));

        v[9] = _mm256_mullo_epi32(u[8], cospi56);
        x = _mm256_mullo_epi32(u[9], cospi8);
        v[9] = _mm256_sub_epi32(v[9], x);
        v[9] = _mm256_add_epi32(v[9], rnding);
        v[9] = _mm256_sra_epi32(v[9], _mm_cvtsi32_si128(bit));

        v[10] = _mm256_mullo_epi32(u[10], cospi40);
        x = _mm256_mullo_epi32(u[11], cospi24);
        v[10] = _mm256_add_epi32(v[10], x);
        v[10] = _mm256_add_epi32(v[10], rnding);
        v[10] = _mm256_sra_epi32(v[10], _mm_cvtsi32_si128(bit));

        v[11] = _mm256_mullo_epi32(u[10], cospi24);
        x = _mm256_mullo_epi32(u[11], cospi40);
        v[11] = _mm256_sub_epi32(v[11], x);
        v[11] = _mm256_add_epi32(v[11], rnding);
        v[11] = _mm256_sra_epi32(v[11], _mm_cvtsi32_si128(bit));

        v[12] = _mm256_mullo_epi32(u[12], cospim56);
        x = _mm256_mullo_epi32(u[13], cospi8);
        v[12] = _mm256_add_epi32(v[12], x);
        v[12] = _mm256_add_epi32(v[12], rnding);
        v[12] = _mm256_sra_epi32(v[12], _mm_cvtsi32_si128(bit));

        v[13] = _mm256_mullo_epi32(u[12], cospi8);
        x = _mm256_mullo_epi32(u[13], cospim56);
        v[13] = _mm256_sub_epi32(v[13], x);
        v[13] = _mm256_add_epi32(v[13], rnding);
        v[13] = _mm256_sra_epi32(v[13], _mm_cvtsi32_si128(bit));

        v[14] = _mm256_mullo_epi32(u[14], cospim24);
        x = _mm256_mullo_epi32(u[15], cospi40);
        v[14] = _mm256_add_epi32(v[14], x);
        v[14] = _mm256_add_epi32(v[14], rnding);
        v[14] = _mm256_sra_epi32(v[14], _mm_cvtsi32_si128(bit));

        v[15] = _mm256_mullo_epi32(u[14], cospi40);
        x = _mm256_mullo_epi32(u[15], cospim24);
        v[15] = _mm256_sub_epi32(v[15], x);
        v[15] = _mm256_add_epi32(v[15], rnding);
        v[15] = _mm256_sra_epi32(v[15], _mm_cvtsi32_si128(bit));

        // stage 5
        addsub_avx2(v[0], v[4], u.as_mut_ptr().add(0), u.as_mut_ptr().add(4), &clamp_lo, &clamp_hi);
        addsub_avx2(v[1], v[5], u.as_mut_ptr().add(1), u.as_mut_ptr().add(5), &clamp_lo, &clamp_hi);
        addsub_avx2(v[2], v[6], u.as_mut_ptr().add(2), u.as_mut_ptr().add(6), &clamp_lo, &clamp_hi);
        addsub_avx2(v[3], v[7], u.as_mut_ptr().add(3), u.as_mut_ptr().add(7), &clamp_lo, &clamp_hi);
        addsub_avx2(
            v[8],
            v[12],
            u.as_mut_ptr().add(8),
            u.as_mut_ptr().add(12),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            v[9],
            v[13],
            u.as_mut_ptr().add(9),
            u.as_mut_ptr().add(13),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            v[10],
            v[14],
            u.as_mut_ptr().add(10),
            u.as_mut_ptr().add(14),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            v[11],
            v[15],
            u.as_mut_ptr().add(11),
            u.as_mut_ptr().add(15),
            &clamp_lo,
            &clamp_hi,
        );

        // stage 6
        v[0] = u[0];
        v[1] = u[1];
        v[2] = u[2];
        v[3] = u[3];

        v[4] = _mm256_mullo_epi32(u[4], cospi16);
        x = _mm256_mullo_epi32(u[5], cospi48);
        v[4] = _mm256_add_epi32(v[4], x);
        v[4] = _mm256_add_epi32(v[4], rnding);
        v[4] = _mm256_sra_epi32(v[4], _mm_cvtsi32_si128(bit));

        v[5] = _mm256_mullo_epi32(u[4], cospi48);
        x = _mm256_mullo_epi32(u[5], cospi16);
        v[5] = _mm256_sub_epi32(v[5], x);
        v[5] = _mm256_add_epi32(v[5], rnding);
        v[5] = _mm256_sra_epi32(v[5], _mm_cvtsi32_si128(bit));

        v[6] = _mm256_mullo_epi32(u[6], cospim48);
        x = _mm256_mullo_epi32(u[7], cospi16);
        v[6] = _mm256_add_epi32(v[6], x);
        v[6] = _mm256_add_epi32(v[6], rnding);
        v[6] = _mm256_sra_epi32(v[6], _mm_cvtsi32_si128(bit));

        v[7] = _mm256_mullo_epi32(u[6], cospi16);
        x = _mm256_mullo_epi32(u[7], cospim48);
        v[7] = _mm256_sub_epi32(v[7], x);
        v[7] = _mm256_add_epi32(v[7], rnding);
        v[7] = _mm256_sra_epi32(v[7], _mm_cvtsi32_si128(bit));

        v[8] = u[8];
        v[9] = u[9];
        v[10] = u[10];
        v[11] = u[11];

        v[12] = _mm256_mullo_epi32(u[12], cospi16);
        x = _mm256_mullo_epi32(u[13], cospi48);
        v[12] = _mm256_add_epi32(v[12], x);
        v[12] = _mm256_add_epi32(v[12], rnding);
        v[12] = _mm256_sra_epi32(v[12], _mm_cvtsi32_si128(bit));

        v[13] = _mm256_mullo_epi32(u[12], cospi48);
        x = _mm256_mullo_epi32(u[13], cospi16);
        v[13] = _mm256_sub_epi32(v[13], x);
        v[13] = _mm256_add_epi32(v[13], rnding);
        v[13] = _mm256_sra_epi32(v[13], _mm_cvtsi32_si128(bit));

        v[14] = _mm256_mullo_epi32(u[14], cospim48);
        x = _mm256_mullo_epi32(u[15], cospi16);
        v[14] = _mm256_add_epi32(v[14], x);
        v[14] = _mm256_add_epi32(v[14], rnding);
        v[14] = _mm256_sra_epi32(v[14], _mm_cvtsi32_si128(bit));

        v[15] = _mm256_mullo_epi32(u[14], cospi16);
        x = _mm256_mullo_epi32(u[15], cospim48);
        v[15] = _mm256_sub_epi32(v[15], x);
        v[15] = _mm256_add_epi32(v[15], rnding);
        v[15] = _mm256_sra_epi32(v[15], _mm_cvtsi32_si128(bit));

        // stage 7
        addsub_avx2(v[0], v[2], u.as_mut_ptr().add(0), u.as_mut_ptr().add(2), &clamp_lo, &clamp_hi);
        addsub_avx2(v[1], v[3], u.as_mut_ptr().add(1), u.as_mut_ptr().add(3), &clamp_lo, &clamp_hi);
        addsub_avx2(v[4], v[6], u.as_mut_ptr().add(4), u.as_mut_ptr().add(6), &clamp_lo, &clamp_hi);
        addsub_avx2(v[5], v[7], u.as_mut_ptr().add(5), u.as_mut_ptr().add(7), &clamp_lo, &clamp_hi);
        addsub_avx2(
            v[8],
            v[10],
            u.as_mut_ptr().add(8),
            u.as_mut_ptr().add(10),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            v[9],
            v[11],
            u.as_mut_ptr().add(9),
            u.as_mut_ptr().add(11),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            v[12],
            v[14],
            u.as_mut_ptr().add(12),
            u.as_mut_ptr().add(14),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            v[13],
            v[15],
            u.as_mut_ptr().add(13),
            u.as_mut_ptr().add(15),
            &clamp_lo,
            &clamp_hi,
        );

        // stage 8
        v[0] = u[0];
        v[1] = u[1];

        y = _mm256_mullo_epi32(u[2], cospi32);
        x = _mm256_mullo_epi32(u[3], cospi32);
        v[2] = _mm256_add_epi32(y, x);
        v[2] = _mm256_add_epi32(v[2], rnding);
        v[2] = _mm256_sra_epi32(v[2], _mm_cvtsi32_si128(bit));

        v[3] = _mm256_sub_epi32(y, x);
        v[3] = _mm256_add_epi32(v[3], rnding);
        v[3] = _mm256_sra_epi32(v[3], _mm_cvtsi32_si128(bit));

        v[4] = u[4];
        v[5] = u[5];

        y = _mm256_mullo_epi32(u[6], cospi32);
        x = _mm256_mullo_epi32(u[7], cospi32);
        v[6] = _mm256_add_epi32(y, x);
        v[6] = _mm256_add_epi32(v[6], rnding);
        v[6] = _mm256_sra_epi32(v[6], _mm_cvtsi32_si128(bit));

        v[7] = _mm256_sub_epi32(y, x);
        v[7] = _mm256_add_epi32(v[7], rnding);
        v[7] = _mm256_sra_epi32(v[7], _mm_cvtsi32_si128(bit));

        v[8] = u[8];
        v[9] = u[9];

        y = _mm256_mullo_epi32(u[10], cospi32);
        x = _mm256_mullo_epi32(u[11], cospi32);
        v[10] = _mm256_add_epi32(y, x);
        v[10] = _mm256_add_epi32(v[10], rnding);
        v[10] = _mm256_sra_epi32(v[10], _mm_cvtsi32_si128(bit));

        v[11] = _mm256_sub_epi32(y, x);
        v[11] = _mm256_add_epi32(v[11], rnding);
        v[11] = _mm256_sra_epi32(v[11], _mm_cvtsi32_si128(bit));

        v[12] = u[12];
        v[13] = u[13];

        y = _mm256_mullo_epi32(u[14], cospi32);
        x = _mm256_mullo_epi32(u[15], cospi32);
        v[14] = _mm256_add_epi32(y, x);
        v[14] = _mm256_add_epi32(v[14], rnding);
        v[14] = _mm256_sra_epi32(v[14], _mm_cvtsi32_si128(bit));

        v[15] = _mm256_sub_epi32(y, x);
        v[15] = _mm256_add_epi32(v[15], rnding);
        v[15] = _mm256_sra_epi32(v[15], _mm_cvtsi32_si128(bit));

        // stage 9
        if do_cols != 0 {
            *out.add(0) = v[0];
            *out.add(1) = _mm256_sub_epi32(_mm256_setzero_si256(), v[8]);
            *out.add(2) = v[12];
            *out.add(3) = _mm256_sub_epi32(_mm256_setzero_si256(), v[4]);
            *out.add(4) = v[6];
            *out.add(5) = _mm256_sub_epi32(_mm256_setzero_si256(), v[14]);
            *out.add(6) = v[10];
            *out.add(7) = _mm256_sub_epi32(_mm256_setzero_si256(), v[2]);
            *out.add(8) = v[3];
            *out.add(9) = _mm256_sub_epi32(_mm256_setzero_si256(), v[11]);
            *out.add(10) = v[15];
            *out.add(11) = _mm256_sub_epi32(_mm256_setzero_si256(), v[7]);
            *out.add(12) = v[5];
            *out.add(13) = _mm256_sub_epi32(_mm256_setzero_si256(), v[13]);
            *out.add(14) = v[9];
            *out.add(15) = _mm256_sub_epi32(_mm256_setzero_si256(), v[1]);
        } else {
            let mut log_range_out: i32 = get_log_range_out(bd);
            let mut clamp_lo_out: __m256i = _mm256_set1_epi32(-(1 << (log_range_out - 1)));
            let mut clamp_hi_out: __m256i = _mm256_set1_epi32((1 << (log_range_out - 1)) - 1);

            neg_shift_avx2(
                v[0],
                v[8],
                out.add(0),
                out.add(1),
                &clamp_lo_out,
                &clamp_hi_out,
                out_shift,
            );
            neg_shift_avx2(
                v[12],
                v[4],
                out.add(2),
                out.add(3),
                &clamp_lo_out,
                &clamp_hi_out,
                out_shift,
            );
            neg_shift_avx2(
                v[6],
                v[14],
                out.add(4),
                out.add(5),
                &clamp_lo_out,
                &clamp_hi_out,
                out_shift,
            );
            neg_shift_avx2(
                v[10],
                v[2],
                out.add(6),
                out.add(7),
                &clamp_lo_out,
                &clamp_hi_out,
                out_shift,
            );
            neg_shift_avx2(
                v[3],
                v[11],
                out.add(8),
                out.add(9),
                &clamp_lo_out,
                &clamp_hi_out,
                out_shift,
            );
            neg_shift_avx2(
                v[15],
                v[7],
                out.add(10),
                out.add(11),
                &clamp_lo_out,
                &clamp_hi_out,
                out_shift,
            );
            neg_shift_avx2(
                v[5],
                v[13],
                out.add(12),
                out.add(13),
                &clamp_lo_out,
                &clamp_hi_out,
                out_shift,
            );
            neg_shift_avx2(
                v[9],
                v[1],
                out.add(14),
                out.add(15),
                &clamp_lo_out,
                &clamp_hi_out,
                out_shift,
            );
        }
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn idct8x8_low1_avx2(
    mut in_: *mut __m256i,
    mut out: *mut __m256i,
    mut bit: i32,
    mut do_cols: i32,
    mut bd: i32,
    mut out_shift: i32,
) {
    let mut cospi: *const i32 = cospi_arr(bit);
    let mut cospi32: __m256i = _mm256_set1_epi32(*cospi.add(32));
    let mut rnding: __m256i = _mm256_set1_epi32(1 << (bit - 1));
    let mut log_range: i32 = get_log_range(bd, do_cols);
    let mut clamp_lo: __m256i = _mm256_set1_epi32(-(1 << (log_range - 1)));
    let mut clamp_hi: __m256i = _mm256_set1_epi32((1 << (log_range - 1)) - 1);
    let mut x: __m256i;

    // stage 0
    // stage 1
    // stage 2
    // stage 3
    x = _mm256_mullo_epi32(*in_.add(0), cospi32);
    x = _mm256_add_epi32(x, rnding);
    x = _mm256_sra_epi32(x, _mm_cvtsi32_si128(bit));

    // stage 4
    // stage 5
    if do_cols == 0 {
        let mut log_range_out: i32 = get_log_range_out(bd);
        let mut offset: __m256i = _mm256_set1_epi32((1 << out_shift) >> 1);
        clamp_lo = _mm256_set1_epi32(-(1 << (log_range_out - 1)));
        clamp_hi = _mm256_set1_epi32((1 << (log_range_out - 1)) - 1);
        x = _mm256_add_epi32(x, offset);
        x = _mm256_sra_epi32(x, _mm_cvtsi32_si128(out_shift));
    }
    x = _mm256_max_epi32(x, clamp_lo);
    x = _mm256_min_epi32(x, clamp_hi);
    *out.add(0) = x;
    *out.add(1) = x;
    *out.add(2) = x;
    *out.add(3) = x;
    *out.add(4) = x;
    *out.add(5) = x;
    *out.add(6) = x;
    *out.add(7) = x;
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn idct8x8_avx2(
    mut in_: *mut __m256i,
    mut out: *mut __m256i,
    mut bit: i32,
    mut do_cols: i32,
    mut bd: i32,
    mut out_shift: i32,
) {
    let mut cospi: *const i32 = cospi_arr(bit);
    let mut cospi56: __m256i = _mm256_set1_epi32(*cospi.add(56));
    let mut cospim8: __m256i = _mm256_set1_epi32(-*cospi.add(8));
    let mut cospi24: __m256i = _mm256_set1_epi32(*cospi.add(24));
    let mut cospim40: __m256i = _mm256_set1_epi32(-*cospi.add(40));
    let mut cospi40: __m256i = _mm256_set1_epi32(*cospi.add(40));
    let mut cospi8: __m256i = _mm256_set1_epi32(*cospi.add(8));
    let mut cospi32: __m256i = _mm256_set1_epi32(*cospi.add(32));
    let mut cospi48: __m256i = _mm256_set1_epi32(*cospi.add(48));
    let mut cospim16: __m256i = _mm256_set1_epi32(-*cospi.add(16));
    let mut cospi16: __m256i = _mm256_set1_epi32(*cospi.add(16));
    let mut rnding: __m256i = _mm256_set1_epi32(1 << (bit - 1));
    let mut log_range: i32 = get_log_range(bd, do_cols);
    let mut clamp_lo: __m256i = _mm256_set1_epi32(-(1 << (log_range - 1)));
    let mut clamp_hi: __m256i = _mm256_set1_epi32((1 << (log_range - 1)) - 1);
    let mut u0: __m256i = core::mem::zeroed();
    let mut u1: __m256i = core::mem::zeroed();
    let mut u2: __m256i = core::mem::zeroed();
    let mut u3: __m256i = core::mem::zeroed();
    let mut u4: __m256i;
    let mut u5: __m256i;
    let mut u6: __m256i;
    let mut u7: __m256i;
    let mut v0: __m256i;
    let mut v1: __m256i;
    let mut v2: __m256i;
    let mut v3: __m256i;
    let mut v4: __m256i = core::mem::zeroed();
    let mut v5: __m256i = core::mem::zeroed();
    let mut v6: __m256i = core::mem::zeroed();
    let mut v7: __m256i = core::mem::zeroed();
    let mut x: __m256i;
    let mut y: __m256i;

    // stage 0
    // stage 1
    // stage 2
    u0 = *in_.add(0);
    u1 = *in_.add(4);
    u2 = *in_.add(2);
    u3 = *in_.add(6);

    x = _mm256_mullo_epi32(*in_.add(1), cospi56);
    y = _mm256_mullo_epi32(*in_.add(7), cospim8);
    u4 = _mm256_add_epi32(x, y);
    u4 = _mm256_add_epi32(u4, rnding);
    u4 = _mm256_sra_epi32(u4, _mm_cvtsi32_si128(bit));

    x = _mm256_mullo_epi32(*in_.add(1), cospi8);
    y = _mm256_mullo_epi32(*in_.add(7), cospi56);
    u7 = _mm256_add_epi32(x, y);
    u7 = _mm256_add_epi32(u7, rnding);
    u7 = _mm256_sra_epi32(u7, _mm_cvtsi32_si128(bit));

    x = _mm256_mullo_epi32(*in_.add(5), cospi24);
    y = _mm256_mullo_epi32(*in_.add(3), cospim40);
    u5 = _mm256_add_epi32(x, y);
    u5 = _mm256_add_epi32(u5, rnding);
    u5 = _mm256_sra_epi32(u5, _mm_cvtsi32_si128(bit));

    x = _mm256_mullo_epi32(*in_.add(5), cospi40);
    y = _mm256_mullo_epi32(*in_.add(3), cospi24);
    u6 = _mm256_add_epi32(x, y);
    u6 = _mm256_add_epi32(u6, rnding);
    u6 = _mm256_sra_epi32(u6, _mm_cvtsi32_si128(bit));

    // stage 3
    x = _mm256_mullo_epi32(u0, cospi32);
    y = _mm256_mullo_epi32(u1, cospi32);
    v0 = _mm256_add_epi32(x, y);
    v0 = _mm256_add_epi32(v0, rnding);
    v0 = _mm256_sra_epi32(v0, _mm_cvtsi32_si128(bit));

    v1 = _mm256_sub_epi32(x, y);
    v1 = _mm256_add_epi32(v1, rnding);
    v1 = _mm256_sra_epi32(v1, _mm_cvtsi32_si128(bit));

    x = _mm256_mullo_epi32(u2, cospi48);
    y = _mm256_mullo_epi32(u3, cospim16);
    v2 = _mm256_add_epi32(x, y);
    v2 = _mm256_add_epi32(v2, rnding);
    v2 = _mm256_sra_epi32(v2, _mm_cvtsi32_si128(bit));

    x = _mm256_mullo_epi32(u2, cospi16);
    y = _mm256_mullo_epi32(u3, cospi48);
    v3 = _mm256_add_epi32(x, y);
    v3 = _mm256_add_epi32(v3, rnding);
    v3 = _mm256_sra_epi32(v3, _mm_cvtsi32_si128(bit));

    addsub_avx2(u4, u5, &mut v4, &mut v5, &clamp_lo, &clamp_hi);
    addsub_avx2(u7, u6, &mut v7, &mut v6, &clamp_lo, &clamp_hi);

    // stage 4
    addsub_avx2(v0, v3, &mut u0, &mut u3, &clamp_lo, &clamp_hi);
    addsub_avx2(v1, v2, &mut u1, &mut u2, &clamp_lo, &clamp_hi);
    u4 = v4;
    u7 = v7;

    x = _mm256_mullo_epi32(v5, cospi32);
    y = _mm256_mullo_epi32(v6, cospi32);
    u6 = _mm256_add_epi32(y, x);
    u6 = _mm256_add_epi32(u6, rnding);
    u6 = _mm256_sra_epi32(u6, _mm_cvtsi32_si128(bit));

    u5 = _mm256_sub_epi32(y, x);
    u5 = _mm256_add_epi32(u5, rnding);
    u5 = _mm256_sra_epi32(u5, _mm_cvtsi32_si128(bit));

    addsub_avx2(u0, u7, out.add(0), out.add(7), &clamp_lo, &clamp_hi);
    addsub_avx2(u1, u6, out.add(1), out.add(6), &clamp_lo, &clamp_hi);
    addsub_avx2(u2, u5, out.add(2), out.add(5), &clamp_lo, &clamp_hi);
    addsub_avx2(u3, u4, out.add(3), out.add(4), &clamp_lo, &clamp_hi);
    // stage 5
    if do_cols == 0 {
        let mut log_range_out: i32 = get_log_range_out(bd);
        let mut clamp_lo_out: __m256i = _mm256_set1_epi32(-(1 << (log_range_out - 1)));
        let mut clamp_hi_out: __m256i = _mm256_set1_epi32((1 << (log_range_out - 1)) - 1);

        round_shift_4x4_avx2(out, out_shift);
        round_shift_4x4_avx2(out.add(4), out_shift);
        highbd_clamp_epi32_avx2(out, out, &clamp_lo_out, &clamp_hi_out, 8);
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn iadst8x8_low1_avx2(
    mut in_: *mut __m256i,
    mut out: *mut __m256i,
    mut bit: i32,
    mut do_cols: i32,
    mut bd: i32,
    mut out_shift: i32,
) {
    let mut cospi: *const i32 = cospi_arr(bit);
    let mut cospi4: __m256i = _mm256_set1_epi32(*cospi.add(4));
    let mut cospi60: __m256i = _mm256_set1_epi32(*cospi.add(60));
    let mut cospi16: __m256i = _mm256_set1_epi32(*cospi.add(16));
    let mut cospi48: __m256i = _mm256_set1_epi32(*cospi.add(48));
    let mut cospi32: __m256i = _mm256_set1_epi32(*cospi.add(32));
    let mut rnding: __m256i = _mm256_set1_epi32(1 << (bit - 1));
    let mut kZero: __m256i = _mm256_setzero_si256();
    let mut u: [__m256i; 8] = core::mem::zeroed();
    let mut x: __m256i;

    // stage 0
    // stage 1
    // stage 2

    x = _mm256_mullo_epi32(*in_.add(0), cospi60);
    u[0] = _mm256_add_epi32(x, rnding);
    u[0] = _mm256_sra_epi32(u[0], _mm_cvtsi32_si128(bit));

    x = _mm256_mullo_epi32(*in_.add(0), cospi4);
    u[1] = _mm256_sub_epi32(kZero, x);
    u[1] = _mm256_add_epi32(u[1], rnding);
    u[1] = _mm256_sra_epi32(u[1], _mm_cvtsi32_si128(bit));

    // stage 3
    // stage 4
    let mut temp1: __m256i;
    let mut temp2: __m256i;
    temp1 = _mm256_mullo_epi32(u[0], cospi16);
    x = _mm256_mullo_epi32(u[1], cospi48);
    temp1 = _mm256_add_epi32(temp1, x);
    temp1 = _mm256_add_epi32(temp1, rnding);
    temp1 = _mm256_sra_epi32(temp1, _mm_cvtsi32_si128(bit));
    u[4] = temp1;

    temp2 = _mm256_mullo_epi32(u[0], cospi48);
    x = _mm256_mullo_epi32(u[1], cospi16);
    u[5] = _mm256_sub_epi32(temp2, x);
    u[5] = _mm256_add_epi32(u[5], rnding);
    u[5] = _mm256_sra_epi32(u[5], _mm_cvtsi32_si128(bit));

    // stage 5
    // stage 6
    temp1 = _mm256_mullo_epi32(u[0], cospi32);
    x = _mm256_mullo_epi32(u[1], cospi32);
    u[2] = _mm256_add_epi32(temp1, x);
    u[2] = _mm256_add_epi32(u[2], rnding);
    u[2] = _mm256_sra_epi32(u[2], _mm_cvtsi32_si128(bit));

    u[3] = _mm256_sub_epi32(temp1, x);
    u[3] = _mm256_add_epi32(u[3], rnding);
    u[3] = _mm256_sra_epi32(u[3], _mm_cvtsi32_si128(bit));

    temp1 = _mm256_mullo_epi32(u[4], cospi32);
    x = _mm256_mullo_epi32(u[5], cospi32);
    u[6] = _mm256_add_epi32(temp1, x);
    u[6] = _mm256_add_epi32(u[6], rnding);
    u[6] = _mm256_sra_epi32(u[6], _mm_cvtsi32_si128(bit));

    u[7] = _mm256_sub_epi32(temp1, x);
    u[7] = _mm256_add_epi32(u[7], rnding);
    u[7] = _mm256_sra_epi32(u[7], _mm_cvtsi32_si128(bit));

    // stage 7
    if do_cols != 0 {
        *out.add(0) = u[0];
        *out.add(1) = _mm256_sub_epi32(kZero, u[4]);
        *out.add(2) = u[6];
        *out.add(3) = _mm256_sub_epi32(kZero, u[2]);
        *out.add(4) = u[3];
        *out.add(5) = _mm256_sub_epi32(kZero, u[7]);
        *out.add(6) = u[5];
        *out.add(7) = _mm256_sub_epi32(kZero, u[1]);
    } else {
        let mut log_range_out: i32 = get_log_range_out(bd);
        let mut clamp_lo_out: __m256i = _mm256_set1_epi32(-(1 << (log_range_out - 1)));
        let mut clamp_hi_out: __m256i = _mm256_set1_epi32((1 << (log_range_out - 1)) - 1);

        neg_shift_avx2(u[0], u[4], out.add(0), out.add(1), &clamp_lo_out, &clamp_hi_out, out_shift);
        neg_shift_avx2(u[6], u[2], out.add(2), out.add(3), &clamp_lo_out, &clamp_hi_out, out_shift);
        neg_shift_avx2(u[3], u[7], out.add(4), out.add(5), &clamp_lo_out, &clamp_hi_out, out_shift);
        neg_shift_avx2(u[5], u[1], out.add(6), out.add(7), &clamp_lo_out, &clamp_hi_out, out_shift);
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn iadst8x8_avx2(
    mut in_: *mut __m256i,
    mut out: *mut __m256i,
    mut bit: i32,
    mut do_cols: i32,
    mut bd: i32,
    mut out_shift: i32,
) {
    let mut cospi: *const i32 = cospi_arr(bit);
    let mut cospi4: __m256i = _mm256_set1_epi32(*cospi.add(4));
    let mut cospi60: __m256i = _mm256_set1_epi32(*cospi.add(60));
    let mut cospi20: __m256i = _mm256_set1_epi32(*cospi.add(20));
    let mut cospi44: __m256i = _mm256_set1_epi32(*cospi.add(44));
    let mut cospi36: __m256i = _mm256_set1_epi32(*cospi.add(36));
    let mut cospi28: __m256i = _mm256_set1_epi32(*cospi.add(28));
    let mut cospi52: __m256i = _mm256_set1_epi32(*cospi.add(52));
    let mut cospi12: __m256i = _mm256_set1_epi32(*cospi.add(12));
    let mut cospi16: __m256i = _mm256_set1_epi32(*cospi.add(16));
    let mut cospi48: __m256i = _mm256_set1_epi32(*cospi.add(48));
    let mut cospim48: __m256i = _mm256_set1_epi32(-*cospi.add(48));
    let mut cospi32: __m256i = _mm256_set1_epi32(*cospi.add(32));
    let mut rnding: __m256i = _mm256_set1_epi32(1 << (bit - 1));
    let mut kZero: __m256i = _mm256_setzero_si256();
    let mut log_range: i32 = get_log_range(bd, do_cols);
    let mut clamp_lo: __m256i = _mm256_set1_epi32(-(1 << (log_range - 1)));
    let mut clamp_hi: __m256i = _mm256_set1_epi32((1 << (log_range - 1)) - 1);
    let mut u: [__m256i; 8] = core::mem::zeroed();
    let mut v: [__m256i; 8] = core::mem::zeroed();
    let mut x: __m256i;

    // stage 0
    // stage 1
    // stage 2

    u[0] = _mm256_mullo_epi32(*in_.add(7), cospi4);
    x = _mm256_mullo_epi32(*in_.add(0), cospi60);
    u[0] = _mm256_add_epi32(u[0], x);
    u[0] = _mm256_add_epi32(u[0], rnding);
    u[0] = _mm256_sra_epi32(u[0], _mm_cvtsi32_si128(bit));

    u[1] = _mm256_mullo_epi32(*in_.add(7), cospi60);
    x = _mm256_mullo_epi32(*in_.add(0), cospi4);
    u[1] = _mm256_sub_epi32(u[1], x);
    u[1] = _mm256_add_epi32(u[1], rnding);
    u[1] = _mm256_sra_epi32(u[1], _mm_cvtsi32_si128(bit));

    u[2] = _mm256_mullo_epi32(*in_.add(5), cospi20);
    x = _mm256_mullo_epi32(*in_.add(2), cospi44);
    u[2] = _mm256_add_epi32(u[2], x);
    u[2] = _mm256_add_epi32(u[2], rnding);
    u[2] = _mm256_sra_epi32(u[2], _mm_cvtsi32_si128(bit));

    u[3] = _mm256_mullo_epi32(*in_.add(5), cospi44);
    x = _mm256_mullo_epi32(*in_.add(2), cospi20);
    u[3] = _mm256_sub_epi32(u[3], x);
    u[3] = _mm256_add_epi32(u[3], rnding);
    u[3] = _mm256_sra_epi32(u[3], _mm_cvtsi32_si128(bit));

    u[4] = _mm256_mullo_epi32(*in_.add(3), cospi36);
    x = _mm256_mullo_epi32(*in_.add(4), cospi28);
    u[4] = _mm256_add_epi32(u[4], x);
    u[4] = _mm256_add_epi32(u[4], rnding);
    u[4] = _mm256_sra_epi32(u[4], _mm_cvtsi32_si128(bit));

    u[5] = _mm256_mullo_epi32(*in_.add(3), cospi28);
    x = _mm256_mullo_epi32(*in_.add(4), cospi36);
    u[5] = _mm256_sub_epi32(u[5], x);
    u[5] = _mm256_add_epi32(u[5], rnding);
    u[5] = _mm256_sra_epi32(u[5], _mm_cvtsi32_si128(bit));

    u[6] = _mm256_mullo_epi32(*in_.add(1), cospi52);
    x = _mm256_mullo_epi32(*in_.add(6), cospi12);
    u[6] = _mm256_add_epi32(u[6], x);
    u[6] = _mm256_add_epi32(u[6], rnding);
    u[6] = _mm256_sra_epi32(u[6], _mm_cvtsi32_si128(bit));

    u[7] = _mm256_mullo_epi32(*in_.add(1), cospi12);
    x = _mm256_mullo_epi32(*in_.add(6), cospi52);
    u[7] = _mm256_sub_epi32(u[7], x);
    u[7] = _mm256_add_epi32(u[7], rnding);
    u[7] = _mm256_sra_epi32(u[7], _mm_cvtsi32_si128(bit));

    // stage 3
    addsub_avx2(u[0], u[4], v.as_mut_ptr().add(0), v.as_mut_ptr().add(4), &clamp_lo, &clamp_hi);
    addsub_avx2(u[1], u[5], v.as_mut_ptr().add(1), v.as_mut_ptr().add(5), &clamp_lo, &clamp_hi);
    addsub_avx2(u[2], u[6], v.as_mut_ptr().add(2), v.as_mut_ptr().add(6), &clamp_lo, &clamp_hi);
    addsub_avx2(u[3], u[7], v.as_mut_ptr().add(3), v.as_mut_ptr().add(7), &clamp_lo, &clamp_hi);

    // stage 4
    u[0] = v[0];
    u[1] = v[1];
    u[2] = v[2];
    u[3] = v[3];

    u[4] = _mm256_mullo_epi32(v[4], cospi16);
    x = _mm256_mullo_epi32(v[5], cospi48);
    u[4] = _mm256_add_epi32(u[4], x);
    u[4] = _mm256_add_epi32(u[4], rnding);
    u[4] = _mm256_sra_epi32(u[4], _mm_cvtsi32_si128(bit));

    u[5] = _mm256_mullo_epi32(v[4], cospi48);
    x = _mm256_mullo_epi32(v[5], cospi16);
    u[5] = _mm256_sub_epi32(u[5], x);
    u[5] = _mm256_add_epi32(u[5], rnding);
    u[5] = _mm256_sra_epi32(u[5], _mm_cvtsi32_si128(bit));

    u[6] = _mm256_mullo_epi32(v[6], cospim48);
    x = _mm256_mullo_epi32(v[7], cospi16);
    u[6] = _mm256_add_epi32(u[6], x);
    u[6] = _mm256_add_epi32(u[6], rnding);
    u[6] = _mm256_sra_epi32(u[6], _mm_cvtsi32_si128(bit));

    u[7] = _mm256_mullo_epi32(v[6], cospi16);
    x = _mm256_mullo_epi32(v[7], cospim48);
    u[7] = _mm256_sub_epi32(u[7], x);
    u[7] = _mm256_add_epi32(u[7], rnding);
    u[7] = _mm256_sra_epi32(u[7], _mm_cvtsi32_si128(bit));

    // stage 5
    addsub_avx2(u[0], u[2], v.as_mut_ptr().add(0), v.as_mut_ptr().add(2), &clamp_lo, &clamp_hi);
    addsub_avx2(u[1], u[3], v.as_mut_ptr().add(1), v.as_mut_ptr().add(3), &clamp_lo, &clamp_hi);
    addsub_avx2(u[4], u[6], v.as_mut_ptr().add(4), v.as_mut_ptr().add(6), &clamp_lo, &clamp_hi);
    addsub_avx2(u[5], u[7], v.as_mut_ptr().add(5), v.as_mut_ptr().add(7), &clamp_lo, &clamp_hi);

    // stage 6
    u[0] = v[0];
    u[1] = v[1];
    u[4] = v[4];
    u[5] = v[5];

    v[0] = _mm256_mullo_epi32(v[2], cospi32);
    x = _mm256_mullo_epi32(v[3], cospi32);
    u[2] = _mm256_add_epi32(v[0], x);
    u[2] = _mm256_add_epi32(u[2], rnding);
    u[2] = _mm256_sra_epi32(u[2], _mm_cvtsi32_si128(bit));

    u[3] = _mm256_sub_epi32(v[0], x);
    u[3] = _mm256_add_epi32(u[3], rnding);
    u[3] = _mm256_sra_epi32(u[3], _mm_cvtsi32_si128(bit));

    v[0] = _mm256_mullo_epi32(v[6], cospi32);
    x = _mm256_mullo_epi32(v[7], cospi32);
    u[6] = _mm256_add_epi32(v[0], x);
    u[6] = _mm256_add_epi32(u[6], rnding);
    u[6] = _mm256_sra_epi32(u[6], _mm_cvtsi32_si128(bit));

    u[7] = _mm256_sub_epi32(v[0], x);
    u[7] = _mm256_add_epi32(u[7], rnding);
    u[7] = _mm256_sra_epi32(u[7], _mm_cvtsi32_si128(bit));

    // stage 7
    if do_cols != 0 {
        *out.add(0) = u[0];
        *out.add(1) = _mm256_sub_epi32(kZero, u[4]);
        *out.add(2) = u[6];
        *out.add(3) = _mm256_sub_epi32(kZero, u[2]);
        *out.add(4) = u[3];
        *out.add(5) = _mm256_sub_epi32(kZero, u[7]);
        *out.add(6) = u[5];
        *out.add(7) = _mm256_sub_epi32(kZero, u[1]);
    } else {
        let mut log_range_out: i32 = get_log_range_out(bd);
        let mut clamp_lo_out: __m256i = _mm256_set1_epi32(-(1 << (log_range_out - 1)));
        let mut clamp_hi_out: __m256i = _mm256_set1_epi32((1 << (log_range_out - 1)) - 1);

        neg_shift_avx2(u[0], u[4], out.add(0), out.add(1), &clamp_lo_out, &clamp_hi_out, out_shift);
        neg_shift_avx2(u[6], u[2], out.add(2), out.add(3), &clamp_lo_out, &clamp_hi_out, out_shift);
        neg_shift_avx2(u[3], u[7], out.add(4), out.add(5), &clamp_lo_out, &clamp_hi_out, out_shift);
        neg_shift_avx2(u[5], u[1], out.add(6), out.add(7), &clamp_lo_out, &clamp_hi_out, out_shift);
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn idct64_stage8_avx2(
    mut u: *mut __m256i,
    mut cospim32: *const __m256i,
    mut cospi32: *const __m256i,
    mut cospim16: *const __m256i,
    mut cospi48: *const __m256i,
    mut cospi16: *const __m256i,
    mut cospim48: *const __m256i,
    mut clamp_lo: *const __m256i,
    mut clamp_hi: *const __m256i,
    mut rnding: *const __m256i,
    mut bit: i32,
) {
    let mut i: i32 = 0;
    let mut temp1: __m256i;
    let mut temp2: __m256i;
    let mut temp3: __m256i;
    let mut temp4: __m256i;
    temp1 = half_btf_avx2(cospim32, u.add(10), cospi32, u.add(13), rnding, bit);
    *u.add(13) = half_btf_avx2(cospi32, u.add(10), cospi32, u.add(13), rnding, bit);
    *u.add(10) = temp1;
    temp2 = half_btf_avx2(cospim32, u.add(11), cospi32, u.add(12), rnding, bit);
    *u.add(12) = half_btf_avx2(cospi32, u.add(11), cospi32, u.add(12), rnding, bit);
    *u.add(11) = temp2;

    for i in 16..20 {
        addsub_avx2(
            *u.offset(i as isize),
            *u.offset((i ^ 7) as isize),
            u.offset(i as isize),
            u.offset((i ^ 7) as isize),
            clamp_lo,
            clamp_hi,
        );
        addsub_avx2(
            *u.offset((i ^ 15) as isize),
            *u.offset((i ^ 8) as isize),
            u.offset((i ^ 15) as isize),
            u.offset((i ^ 8) as isize),
            clamp_lo,
            clamp_hi,
        );
    }

    temp1 = half_btf_avx2(cospim16, u.add(36), cospi48, u.add(59), rnding, bit);
    temp2 = half_btf_avx2(cospim16, u.add(37), cospi48, u.add(58), rnding, bit);
    temp3 = half_btf_avx2(cospim16, u.add(38), cospi48, u.add(57), rnding, bit);
    temp4 = half_btf_avx2(cospim16, u.add(39), cospi48, u.add(56), rnding, bit);
    *u.add(56) = half_btf_avx2(cospi48, u.add(39), cospi16, u.add(56), rnding, bit);
    *u.add(57) = half_btf_avx2(cospi48, u.add(38), cospi16, u.add(57), rnding, bit);
    *u.add(58) = half_btf_avx2(cospi48, u.add(37), cospi16, u.add(58), rnding, bit);
    *u.add(59) = half_btf_avx2(cospi48, u.add(36), cospi16, u.add(59), rnding, bit);
    *u.add(36) = temp1;
    *u.add(37) = temp2;
    *u.add(38) = temp3;
    *u.add(39) = temp4;

    temp1 = half_btf_avx2(cospim48, u.add(40), cospim16, u.add(55), rnding, bit);
    temp2 = half_btf_avx2(cospim48, u.add(41), cospim16, u.add(54), rnding, bit);
    temp3 = half_btf_avx2(cospim48, u.add(42), cospim16, u.add(53), rnding, bit);
    temp4 = half_btf_avx2(cospim48, u.add(43), cospim16, u.add(52), rnding, bit);
    *u.add(52) = half_btf_avx2(cospim16, u.add(43), cospi48, u.add(52), rnding, bit);
    *u.add(53) = half_btf_avx2(cospim16, u.add(42), cospi48, u.add(53), rnding, bit);
    *u.add(54) = half_btf_avx2(cospim16, u.add(41), cospi48, u.add(54), rnding, bit);
    *u.add(55) = half_btf_avx2(cospim16, u.add(40), cospi48, u.add(55), rnding, bit);
    *u.add(40) = temp1;
    *u.add(41) = temp2;
    *u.add(42) = temp3;
    *u.add(43) = temp4;
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn idct64_stage9_avx2(
    mut u: *mut __m256i,
    mut cospim32: *const __m256i,
    mut cospi32: *const __m256i,
    mut clamp_lo: *const __m256i,
    mut clamp_hi: *const __m256i,
    mut rnding: *const __m256i,
    mut bit: i32,
) {
    let mut i: i32 = 0;
    let mut temp1: __m256i;
    let mut temp2: __m256i;
    let mut temp3: __m256i;
    let mut temp4: __m256i;
    for i in 0..8 {
        addsub_avx2(
            *u.offset(i as isize),
            *u.offset((15 - i) as isize),
            u.offset(i as isize),
            u.offset((15 - i) as isize),
            clamp_lo,
            clamp_hi,
        );
    }

    temp1 = half_btf_avx2(cospim32, u.add(20), cospi32, u.add(27), rnding, bit);
    temp2 = half_btf_avx2(cospim32, u.add(21), cospi32, u.add(26), rnding, bit);
    temp3 = half_btf_avx2(cospim32, u.add(22), cospi32, u.add(25), rnding, bit);
    temp4 = half_btf_avx2(cospim32, u.add(23), cospi32, u.add(24), rnding, bit);
    *u.add(24) = half_btf_avx2(cospi32, u.add(23), cospi32, u.add(24), rnding, bit);
    *u.add(25) = half_btf_avx2(cospi32, u.add(22), cospi32, u.add(25), rnding, bit);
    *u.add(26) = half_btf_avx2(cospi32, u.add(21), cospi32, u.add(26), rnding, bit);
    *u.add(27) = half_btf_avx2(cospi32, u.add(20), cospi32, u.add(27), rnding, bit);
    *u.add(20) = temp1;
    *u.add(21) = temp2;
    *u.add(22) = temp3;
    *u.add(23) = temp4;
    for i in 32..40 {
        addsub_avx2(
            *u.offset(i as isize),
            *u.offset((i ^ 15) as isize),
            u.offset(i as isize),
            u.offset((i ^ 15) as isize),
            clamp_lo,
            clamp_hi,
        );
    }

    for i in 48..56 {
        addsub_avx2(
            *u.offset((i ^ 15) as isize),
            *u.offset(i as isize),
            u.offset((i ^ 15) as isize),
            u.offset(i as isize),
            clamp_lo,
            clamp_hi,
        );
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn idct64_stage10_avx2(
    mut u: *mut __m256i,
    mut cospim32: *const __m256i,
    mut cospi32: *const __m256i,
    mut clamp_lo: *const __m256i,
    mut clamp_hi: *const __m256i,
    mut rnding: *const __m256i,
    mut bit: i32,
) {
    let mut temp1: __m256i;
    let mut temp2: __m256i;
    let mut temp3: __m256i;
    let mut temp4: __m256i;
    for i in 0..16 {
        addsub_avx2(
            *u.offset(i as isize),
            *u.offset((31 - i) as isize),
            u.offset(i as isize),
            u.offset((31 - i) as isize),
            clamp_lo,
            clamp_hi,
        );
    }

    temp1 = half_btf_avx2(cospim32, u.add(40), cospi32, u.add(55), rnding, bit);
    temp2 = half_btf_avx2(cospim32, u.add(41), cospi32, u.add(54), rnding, bit);
    temp3 = half_btf_avx2(cospim32, u.add(42), cospi32, u.add(53), rnding, bit);
    temp4 = half_btf_avx2(cospim32, u.add(43), cospi32, u.add(52), rnding, bit);
    *u.add(52) = half_btf_avx2(cospi32, u.add(43), cospi32, u.add(52), rnding, bit);
    *u.add(53) = half_btf_avx2(cospi32, u.add(42), cospi32, u.add(53), rnding, bit);
    *u.add(54) = half_btf_avx2(cospi32, u.add(41), cospi32, u.add(54), rnding, bit);
    *u.add(55) = half_btf_avx2(cospi32, u.add(40), cospi32, u.add(55), rnding, bit);
    *u.add(40) = temp1;
    *u.add(41) = temp2;
    *u.add(42) = temp3;
    *u.add(43) = temp4;

    temp1 = half_btf_avx2(cospim32, u.add(44), cospi32, u.add(51), rnding, bit);
    temp2 = half_btf_avx2(cospim32, u.add(45), cospi32, u.add(50), rnding, bit);
    temp3 = half_btf_avx2(cospim32, u.add(46), cospi32, u.add(49), rnding, bit);
    temp4 = half_btf_avx2(cospim32, u.add(47), cospi32, u.add(48), rnding, bit);
    *u.add(48) = half_btf_avx2(cospi32, u.add(47), cospi32, u.add(48), rnding, bit);
    *u.add(49) = half_btf_avx2(cospi32, u.add(46), cospi32, u.add(49), rnding, bit);
    *u.add(50) = half_btf_avx2(cospi32, u.add(45), cospi32, u.add(50), rnding, bit);
    *u.add(51) = half_btf_avx2(cospi32, u.add(44), cospi32, u.add(51), rnding, bit);
    *u.add(44) = temp1;
    *u.add(45) = temp2;
    *u.add(46) = temp3;
    *u.add(47) = temp4;
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn idct64_stage11_avx2(
    mut u: *mut __m256i,
    mut out: *mut __m256i,
    mut do_cols: i32,
    mut bd: i32,
    mut out_shift: i32,
    mut clamp_lo: *const __m256i,
    mut clamp_hi: *const __m256i,
) {
    for i in 0..32 {
        addsub_avx2(
            *u.offset(i as isize),
            *u.offset((63 - i) as isize),
            out.offset(i as isize),
            out.offset((63 - i) as isize),
            clamp_lo,
            clamp_hi,
        );
    }

    if do_cols == 0 {
        let mut log_range_out: i32 = get_log_range_out(bd);
        let mut clamp_lo_out: __m256i = _mm256_set1_epi32(-(1 << (log_range_out - 1)));
        let mut clamp_hi_out: __m256i = _mm256_set1_epi32((1 << (log_range_out - 1)) - 1);

        round_shift_8x8_avx2(out, out_shift);
        round_shift_8x8_avx2(out.add(16), out_shift);
        round_shift_8x8_avx2(out.add(32), out_shift);
        round_shift_8x8_avx2(out.add(48), out_shift);
        highbd_clamp_epi32_avx2(out, out, &clamp_lo_out, &clamp_hi_out, 64);
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn idct64_low1_avx2(
    mut in_: *mut __m256i,
    mut out: *mut __m256i,
    mut bit: i32,
    mut do_cols: i32,
    mut bd: i32,
    mut out_shift: i32,
) {
    let mut cospi: *const i32 = cospi_arr(bit);
    let mut rnding: __m256i = _mm256_set1_epi32(1 << (bit - 1));
    let mut log_range: i32 = get_log_range(bd, do_cols);
    let mut clamp_lo: __m256i = _mm256_set1_epi32(-(1 << (log_range - 1)));
    let mut clamp_hi: __m256i = _mm256_set1_epi32((1 << (log_range - 1)) - 1);

    let mut cospi32: __m256i = _mm256_set1_epi32(*cospi.add(32));

    {
        let mut x: __m256i;

        // stage 1
        // stage 2
        // stage 3
        // stage 4
        // stage 5
        // stage 6
        x = half_btf_0_avx2(&cospi32, in_.add(0), &rnding, bit);

        // stage 8
        // stage 9
        // stage 10
        // stage 11
        if do_cols == 0 {
            let mut log_range_out: i32 = get_log_range_out(bd);
            clamp_lo = _mm256_set1_epi32(-(1 << (log_range_out - 1)));
            clamp_hi = _mm256_set1_epi32((1 << (log_range_out - 1)) - 1);
            if out_shift != 0 {
                let mut offset: __m256i = _mm256_set1_epi32((1 << out_shift) >> 1);
                x = _mm256_add_epi32(x, offset);
                x = _mm256_sra_epi32(x, _mm_cvtsi32_si128(out_shift));
            }
        }
        x = _mm256_max_epi32(x, clamp_lo);
        x = _mm256_min_epi32(x, clamp_hi);
        *out.add(0) = x;
        *out.add(1) = x;
        *out.add(2) = x;
        *out.add(3) = x;
        *out.add(4) = x;
        *out.add(5) = x;
        *out.add(6) = x;
        *out.add(7) = x;
        *out.add(8) = x;
        *out.add(9) = x;
        *out.add(10) = x;
        *out.add(11) = x;
        *out.add(12) = x;
        *out.add(13) = x;
        *out.add(14) = x;
        *out.add(15) = x;
        *out.add(16) = x;
        *out.add(17) = x;
        *out.add(18) = x;
        *out.add(19) = x;
        *out.add(20) = x;
        *out.add(21) = x;
        *out.add(22) = x;
        *out.add(23) = x;
        *out.add(24) = x;
        *out.add(25) = x;
        *out.add(26) = x;
        *out.add(27) = x;
        *out.add(28) = x;
        *out.add(29) = x;
        *out.add(30) = x;
        *out.add(31) = x;
        *out.add(32) = x;
        *out.add(33) = x;
        *out.add(34) = x;
        *out.add(35) = x;
        *out.add(36) = x;
        *out.add(37) = x;
        *out.add(38) = x;
        *out.add(39) = x;
        *out.add(40) = x;
        *out.add(41) = x;
        *out.add(42) = x;
        *out.add(43) = x;
        *out.add(44) = x;
        *out.add(45) = x;
        *out.add(46) = x;
        *out.add(47) = x;
        *out.add(48) = x;
        *out.add(49) = x;
        *out.add(50) = x;
        *out.add(51) = x;
        *out.add(52) = x;
        *out.add(53) = x;
        *out.add(54) = x;
        *out.add(55) = x;
        *out.add(56) = x;
        *out.add(57) = x;
        *out.add(58) = x;
        *out.add(59) = x;
        *out.add(60) = x;
        *out.add(61) = x;
        *out.add(62) = x;
        *out.add(63) = x;
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn idct64_low8_avx2(
    mut in_: *mut __m256i,
    mut out: *mut __m256i,
    mut bit: i32,
    mut do_cols: i32,
    mut bd: i32,
    mut out_shift: i32,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut cospi: *const i32 = cospi_arr(bit);
    let mut rnding: __m256i = _mm256_set1_epi32(1 << (bit - 1));
    let mut log_range: i32 = get_log_range(bd, do_cols);
    let mut clamp_lo: __m256i = _mm256_set1_epi32(-(1 << (log_range - 1)));
    let mut clamp_hi: __m256i = _mm256_set1_epi32((1 << (log_range - 1)) - 1);

    let mut cospi1: __m256i = _mm256_set1_epi32(*cospi.add(1));
    let mut cospi2: __m256i = _mm256_set1_epi32(*cospi.add(2));
    let mut cospi3: __m256i = _mm256_set1_epi32(*cospi.add(3));
    let mut cospi4: __m256i = _mm256_set1_epi32(*cospi.add(4));
    let mut cospi6: __m256i = _mm256_set1_epi32(*cospi.add(6));
    let mut cospi8: __m256i = _mm256_set1_epi32(*cospi.add(8));
    let mut cospi12: __m256i = _mm256_set1_epi32(*cospi.add(12));
    let mut cospi16: __m256i = _mm256_set1_epi32(*cospi.add(16));
    let mut cospi20: __m256i = _mm256_set1_epi32(*cospi.add(20));
    let mut cospi24: __m256i = _mm256_set1_epi32(*cospi.add(24));
    let mut cospi28: __m256i = _mm256_set1_epi32(*cospi.add(28));
    let mut cospi32: __m256i = _mm256_set1_epi32(*cospi.add(32));
    let mut cospi40: __m256i = _mm256_set1_epi32(*cospi.add(40));
    let mut cospi44: __m256i = _mm256_set1_epi32(*cospi.add(44));
    let mut cospi48: __m256i = _mm256_set1_epi32(*cospi.add(48));
    let mut cospi56: __m256i = _mm256_set1_epi32(*cospi.add(56));
    let mut cospi60: __m256i = _mm256_set1_epi32(*cospi.add(60));
    let mut cospim4: __m256i = _mm256_set1_epi32(-*cospi.add(4));
    let mut cospim8: __m256i = _mm256_set1_epi32(-*cospi.add(8));
    let mut cospim12: __m256i = _mm256_set1_epi32(-*cospi.add(12));
    let mut cospim16: __m256i = _mm256_set1_epi32(-*cospi.add(16));
    let mut cospim20: __m256i = _mm256_set1_epi32(-*cospi.add(20));
    let mut cospim24: __m256i = _mm256_set1_epi32(-*cospi.add(24));
    let mut cospim28: __m256i = _mm256_set1_epi32(-*cospi.add(28));
    let mut cospim32: __m256i = _mm256_set1_epi32(-*cospi.add(32));
    let mut cospim36: __m256i = _mm256_set1_epi32(-*cospi.add(36));
    let mut cospim40: __m256i = _mm256_set1_epi32(-*cospi.add(40));
    let mut cospim48: __m256i = _mm256_set1_epi32(-*cospi.add(48));
    let mut cospim52: __m256i = _mm256_set1_epi32(-*cospi.add(52));
    let mut cospim56: __m256i = _mm256_set1_epi32(-*cospi.add(56));
    let mut cospi63: __m256i = _mm256_set1_epi32(*cospi.add(63));
    let mut cospim57: __m256i = _mm256_set1_epi32(-*cospi.add(57));
    let mut cospi7: __m256i = _mm256_set1_epi32(*cospi.add(7));
    let mut cospi5: __m256i = _mm256_set1_epi32(*cospi.add(5));
    let mut cospi59: __m256i = _mm256_set1_epi32(*cospi.add(59));
    let mut cospim61: __m256i = _mm256_set1_epi32(-*cospi.add(61));
    let mut cospim58: __m256i = _mm256_set1_epi32(-*cospi.add(58));
    let mut cospi62: __m256i = _mm256_set1_epi32(*cospi.add(62));

    {
        let mut u: [__m256i; 64] = core::mem::zeroed();

        // stage 1
        u[0] = *in_.add(0);
        u[8] = *in_.add(4);
        u[16] = *in_.add(2);
        u[24] = *in_.add(6);
        u[32] = *in_.add(1);
        u[40] = *in_.add(5);
        u[48] = *in_.add(3);
        u[56] = *in_.add(7);

        // stage 2
        u[63] = half_btf_0_avx2(&cospi1, u.as_mut_ptr().add(32), &rnding, bit);
        u[32] = half_btf_0_avx2(&cospi63, u.as_mut_ptr().add(32), &rnding, bit);
        u[39] = half_btf_0_avx2(&cospim57, u.as_mut_ptr().add(56), &rnding, bit);
        u[56] = half_btf_0_avx2(&cospi7, u.as_mut_ptr().add(56), &rnding, bit);
        u[55] = half_btf_0_avx2(&cospi5, u.as_mut_ptr().add(40), &rnding, bit);
        u[40] = half_btf_0_avx2(&cospi59, u.as_mut_ptr().add(40), &rnding, bit);
        u[47] = half_btf_0_avx2(&cospim61, u.as_mut_ptr().add(48), &rnding, bit);
        u[48] = half_btf_0_avx2(&cospi3, u.as_mut_ptr().add(48), &rnding, bit);

        // stage 3
        u[31] = half_btf_0_avx2(&cospi2, u.as_mut_ptr().add(16), &rnding, bit);
        u[16] = half_btf_0_avx2(&cospi62, u.as_mut_ptr().add(16), &rnding, bit);
        u[23] = half_btf_0_avx2(&cospim58, u.as_mut_ptr().add(24), &rnding, bit);
        u[24] = half_btf_0_avx2(&cospi6, u.as_mut_ptr().add(24), &rnding, bit);
        u[33] = u[32];
        u[38] = u[39];
        u[41] = u[40];
        u[46] = u[47];
        u[49] = u[48];
        u[54] = u[55];
        u[57] = u[56];
        u[62] = u[63];

        // stage 4
        let mut temp1: __m256i;
        let mut temp2: __m256i;
        u[15] = half_btf_0_avx2(&cospi4, u.as_mut_ptr().add(8), &rnding, bit);
        u[8] = half_btf_0_avx2(&cospi60, u.as_mut_ptr().add(8), &rnding, bit);
        u[17] = u[16];
        u[22] = u[23];
        u[25] = u[24];
        u[30] = u[31];

        temp1 = half_btf_avx2(
            &cospim4,
            u.as_mut_ptr().add(33),
            &cospi60,
            u.as_mut_ptr().add(62),
            &rnding,
            bit,
        );
        u[62] = half_btf_avx2(
            &cospi60,
            u.as_mut_ptr().add(33),
            &cospi4,
            u.as_mut_ptr().add(62),
            &rnding,
            bit,
        );
        u[33] = temp1;

        temp2 = half_btf_avx2(
            &cospim36,
            u.as_mut_ptr().add(38),
            &cospi28,
            u.as_mut_ptr().add(57),
            &rnding,
            bit,
        );
        u[38] = half_btf_avx2(
            &cospim28,
            u.as_mut_ptr().add(38),
            &cospim36,
            u.as_mut_ptr().add(57),
            &rnding,
            bit,
        );
        u[57] = temp2;

        temp1 = half_btf_avx2(
            &cospim20,
            u.as_mut_ptr().add(41),
            &cospi44,
            u.as_mut_ptr().add(54),
            &rnding,
            bit,
        );
        u[54] = half_btf_avx2(
            &cospi44,
            u.as_mut_ptr().add(41),
            &cospi20,
            u.as_mut_ptr().add(54),
            &rnding,
            bit,
        );
        u[41] = temp1;

        temp2 = half_btf_avx2(
            &cospim12,
            u.as_mut_ptr().add(46),
            &cospim52,
            u.as_mut_ptr().add(49),
            &rnding,
            bit,
        );
        u[49] = half_btf_avx2(
            &cospim52,
            u.as_mut_ptr().add(46),
            &cospi12,
            u.as_mut_ptr().add(49),
            &rnding,
            bit,
        );
        u[46] = temp2;

        // stage 5
        u[9] = u[8];
        u[14] = u[15];

        temp1 = half_btf_avx2(
            &cospim8,
            u.as_mut_ptr().add(17),
            &cospi56,
            u.as_mut_ptr().add(30),
            &rnding,
            bit,
        );
        u[30] = half_btf_avx2(
            &cospi56,
            u.as_mut_ptr().add(17),
            &cospi8,
            u.as_mut_ptr().add(30),
            &rnding,
            bit,
        );
        u[17] = temp1;

        temp2 = half_btf_avx2(
            &cospim24,
            u.as_mut_ptr().add(22),
            &cospim40,
            u.as_mut_ptr().add(25),
            &rnding,
            bit,
        );
        u[25] = half_btf_avx2(
            &cospim40,
            u.as_mut_ptr().add(22),
            &cospi24,
            u.as_mut_ptr().add(25),
            &rnding,
            bit,
        );
        u[22] = temp2;

        u[35] = u[32];
        u[34] = u[33];
        u[36] = u[39];
        u[37] = u[38];
        u[43] = u[40];
        u[42] = u[41];
        u[44] = u[47];
        u[45] = u[46];
        u[51] = u[48];
        u[50] = u[49];
        u[52] = u[55];
        u[53] = u[54];
        u[59] = u[56];
        u[58] = u[57];
        u[60] = u[63];
        u[61] = u[62];

        // stage 6
        temp1 = half_btf_0_avx2(&cospi32, u.as_mut_ptr().add(0), &rnding, bit);
        u[1] = half_btf_0_avx2(&cospi32, u.as_mut_ptr().add(0), &rnding, bit);
        u[0] = temp1;

        temp2 = half_btf_avx2(
            &cospim16,
            u.as_mut_ptr().add(9),
            &cospi48,
            u.as_mut_ptr().add(14),
            &rnding,
            bit,
        );
        u[14] = half_btf_avx2(
            &cospi48,
            u.as_mut_ptr().add(9),
            &cospi16,
            u.as_mut_ptr().add(14),
            &rnding,
            bit,
        );
        u[9] = temp2;
        u[19] = u[16];
        u[18] = u[17];
        u[20] = u[23];
        u[21] = u[22];
        u[27] = u[24];
        u[26] = u[25];
        u[28] = u[31];
        u[29] = u[30];

        temp1 = half_btf_avx2(
            &cospim8,
            u.as_mut_ptr().add(34),
            &cospi56,
            u.as_mut_ptr().add(61),
            &rnding,
            bit,
        );
        u[61] = half_btf_avx2(
            &cospi56,
            u.as_mut_ptr().add(34),
            &cospi8,
            u.as_mut_ptr().add(61),
            &rnding,
            bit,
        );
        u[34] = temp1;
        temp2 = half_btf_avx2(
            &cospim8,
            u.as_mut_ptr().add(35),
            &cospi56,
            u.as_mut_ptr().add(60),
            &rnding,
            bit,
        );
        u[60] = half_btf_avx2(
            &cospi56,
            u.as_mut_ptr().add(35),
            &cospi8,
            u.as_mut_ptr().add(60),
            &rnding,
            bit,
        );
        u[35] = temp2;
        temp1 = half_btf_avx2(
            &cospim56,
            u.as_mut_ptr().add(36),
            &cospim8,
            u.as_mut_ptr().add(59),
            &rnding,
            bit,
        );
        u[59] = half_btf_avx2(
            &cospim8,
            u.as_mut_ptr().add(36),
            &cospi56,
            u.as_mut_ptr().add(59),
            &rnding,
            bit,
        );
        u[36] = temp1;
        temp2 = half_btf_avx2(
            &cospim56,
            u.as_mut_ptr().add(37),
            &cospim8,
            u.as_mut_ptr().add(58),
            &rnding,
            bit,
        );
        u[58] = half_btf_avx2(
            &cospim8,
            u.as_mut_ptr().add(37),
            &cospi56,
            u.as_mut_ptr().add(58),
            &rnding,
            bit,
        );
        u[37] = temp2;
        temp1 = half_btf_avx2(
            &cospim40,
            u.as_mut_ptr().add(42),
            &cospi24,
            u.as_mut_ptr().add(53),
            &rnding,
            bit,
        );
        u[53] = half_btf_avx2(
            &cospi24,
            u.as_mut_ptr().add(42),
            &cospi40,
            u.as_mut_ptr().add(53),
            &rnding,
            bit,
        );
        u[42] = temp1;
        temp2 = half_btf_avx2(
            &cospim40,
            u.as_mut_ptr().add(43),
            &cospi24,
            u.as_mut_ptr().add(52),
            &rnding,
            bit,
        );
        u[52] = half_btf_avx2(
            &cospi24,
            u.as_mut_ptr().add(43),
            &cospi40,
            u.as_mut_ptr().add(52),
            &rnding,
            bit,
        );
        u[43] = temp2;
        temp1 = half_btf_avx2(
            &cospim24,
            u.as_mut_ptr().add(44),
            &cospim40,
            u.as_mut_ptr().add(51),
            &rnding,
            bit,
        );
        u[51] = half_btf_avx2(
            &cospim40,
            u.as_mut_ptr().add(44),
            &cospi24,
            u.as_mut_ptr().add(51),
            &rnding,
            bit,
        );
        u[44] = temp1;
        temp2 = half_btf_avx2(
            &cospim24,
            u.as_mut_ptr().add(45),
            &cospim40,
            u.as_mut_ptr().add(50),
            &rnding,
            bit,
        );
        u[50] = half_btf_avx2(
            &cospim40,
            u.as_mut_ptr().add(45),
            &cospi24,
            u.as_mut_ptr().add(50),
            &rnding,
            bit,
        );
        u[45] = temp2;

        // stage 7
        u[3] = u[0];
        u[2] = u[1];
        u[11] = u[8];
        u[10] = u[9];
        u[12] = u[15];
        u[13] = u[14];

        temp1 = half_btf_avx2(
            &cospim16,
            u.as_mut_ptr().add(18),
            &cospi48,
            u.as_mut_ptr().add(29),
            &rnding,
            bit,
        );
        u[29] = half_btf_avx2(
            &cospi48,
            u.as_mut_ptr().add(18),
            &cospi16,
            u.as_mut_ptr().add(29),
            &rnding,
            bit,
        );
        u[18] = temp1;
        temp2 = half_btf_avx2(
            &cospim16,
            u.as_mut_ptr().add(19),
            &cospi48,
            u.as_mut_ptr().add(28),
            &rnding,
            bit,
        );
        u[28] = half_btf_avx2(
            &cospi48,
            u.as_mut_ptr().add(19),
            &cospi16,
            u.as_mut_ptr().add(28),
            &rnding,
            bit,
        );
        u[19] = temp2;
        temp1 = half_btf_avx2(
            &cospim48,
            u.as_mut_ptr().add(20),
            &cospim16,
            u.as_mut_ptr().add(27),
            &rnding,
            bit,
        );
        u[27] = half_btf_avx2(
            &cospim16,
            u.as_mut_ptr().add(20),
            &cospi48,
            u.as_mut_ptr().add(27),
            &rnding,
            bit,
        );
        u[20] = temp1;
        temp2 = half_btf_avx2(
            &cospim48,
            u.as_mut_ptr().add(21),
            &cospim16,
            u.as_mut_ptr().add(26),
            &rnding,
            bit,
        );
        u[26] = half_btf_avx2(
            &cospim16,
            u.as_mut_ptr().add(21),
            &cospi48,
            u.as_mut_ptr().add(26),
            &rnding,
            bit,
        );
        u[21] = temp2;
        for i in (32..64).step_by(16) {
            for j in i..i + 4 {
                addsub_avx2(
                    u[j as usize],
                    u[(j ^ 7) as usize],
                    u.as_mut_ptr().offset(j as isize),
                    u.as_mut_ptr().offset((j ^ 7) as isize),
                    &clamp_lo,
                    &clamp_hi,
                );
                addsub_avx2(
                    u[(j ^ 15) as usize],
                    u[(j ^ 8) as usize],
                    u.as_mut_ptr().offset((j ^ 15) as isize),
                    u.as_mut_ptr().offset((j ^ 8) as isize),
                    &clamp_lo,
                    &clamp_hi,
                );
            }
        }

        // stage 8
        u[7] = u[0];
        u[6] = u[1];
        u[5] = u[2];
        u[4] = u[3];

        idct64_stage8_avx2(
            u.as_mut_ptr(),
            &cospim32,
            &cospi32,
            &cospim16,
            &cospi48,
            &cospi16,
            &cospim48,
            &clamp_lo,
            &clamp_hi,
            &rnding,
            bit,
        );

        // stage 9
        idct64_stage9_avx2(u.as_mut_ptr(), &cospim32, &cospi32, &clamp_lo, &clamp_hi, &rnding, bit);

        // stage 10
        idct64_stage10_avx2(
            u.as_mut_ptr(),
            &cospim32,
            &cospi32,
            &clamp_lo,
            &clamp_hi,
            &rnding,
            bit,
        );

        // stage 11
        idct64_stage11_avx2(u.as_mut_ptr(), out, do_cols, bd, out_shift, &clamp_lo, &clamp_hi);
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn idct64_low16_avx2(
    mut in_: *mut __m256i,
    mut out: *mut __m256i,
    mut bit: i32,
    mut do_cols: i32,
    mut bd: i32,
    mut out_shift: i32,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut cospi: *const i32 = cospi_arr(bit);
    let mut rnding: __m256i = _mm256_set1_epi32(1 << (bit - 1));
    let mut log_range: i32 = get_log_range(bd, do_cols);
    let mut clamp_lo: __m256i = _mm256_set1_epi32(-(1 << (log_range - 1)));
    let mut clamp_hi: __m256i = _mm256_set1_epi32((1 << (log_range - 1)) - 1);

    let mut cospi1: __m256i = _mm256_set1_epi32(*cospi.add(1));
    let mut cospi2: __m256i = _mm256_set1_epi32(*cospi.add(2));
    let mut cospi3: __m256i = _mm256_set1_epi32(*cospi.add(3));
    let mut cospi4: __m256i = _mm256_set1_epi32(*cospi.add(4));
    let mut cospi5: __m256i = _mm256_set1_epi32(*cospi.add(5));
    let mut cospi6: __m256i = _mm256_set1_epi32(*cospi.add(6));
    let mut cospi7: __m256i = _mm256_set1_epi32(*cospi.add(7));
    let mut cospi8: __m256i = _mm256_set1_epi32(*cospi.add(8));
    let mut cospi9: __m256i = _mm256_set1_epi32(*cospi.add(9));
    let mut cospi10: __m256i = _mm256_set1_epi32(*cospi.add(10));
    let mut cospi11: __m256i = _mm256_set1_epi32(*cospi.add(11));
    let mut cospi12: __m256i = _mm256_set1_epi32(*cospi.add(12));
    let mut cospi13: __m256i = _mm256_set1_epi32(*cospi.add(13));
    let mut cospi14: __m256i = _mm256_set1_epi32(*cospi.add(14));
    let mut cospi15: __m256i = _mm256_set1_epi32(*cospi.add(15));
    let mut cospi16: __m256i = _mm256_set1_epi32(*cospi.add(16));
    let mut cospi20: __m256i = _mm256_set1_epi32(*cospi.add(20));
    let mut cospi24: __m256i = _mm256_set1_epi32(*cospi.add(24));
    let mut cospi28: __m256i = _mm256_set1_epi32(*cospi.add(28));
    let mut cospi32: __m256i = _mm256_set1_epi32(*cospi.add(32));
    let mut cospi36: __m256i = _mm256_set1_epi32(*cospi.add(36));
    let mut cospi40: __m256i = _mm256_set1_epi32(*cospi.add(40));
    let mut cospi44: __m256i = _mm256_set1_epi32(*cospi.add(44));
    let mut cospi48: __m256i = _mm256_set1_epi32(*cospi.add(48));
    let mut cospi51: __m256i = _mm256_set1_epi32(*cospi.add(51));
    let mut cospi52: __m256i = _mm256_set1_epi32(*cospi.add(52));
    let mut cospi54: __m256i = _mm256_set1_epi32(*cospi.add(54));
    let mut cospi55: __m256i = _mm256_set1_epi32(*cospi.add(55));
    let mut cospi56: __m256i = _mm256_set1_epi32(*cospi.add(56));
    let mut cospi59: __m256i = _mm256_set1_epi32(*cospi.add(59));
    let mut cospi60: __m256i = _mm256_set1_epi32(*cospi.add(60));
    let mut cospi62: __m256i = _mm256_set1_epi32(*cospi.add(62));
    let mut cospi63: __m256i = _mm256_set1_epi32(*cospi.add(63));

    let mut cospim4: __m256i = _mm256_set1_epi32(-*cospi.add(4));
    let mut cospim8: __m256i = _mm256_set1_epi32(-*cospi.add(8));
    let mut cospim12: __m256i = _mm256_set1_epi32(-*cospi.add(12));
    let mut cospim16: __m256i = _mm256_set1_epi32(-*cospi.add(16));
    let mut cospim20: __m256i = _mm256_set1_epi32(-*cospi.add(20));
    let mut cospim24: __m256i = _mm256_set1_epi32(-*cospi.add(24));
    let mut cospim28: __m256i = _mm256_set1_epi32(-*cospi.add(28));
    let mut cospim32: __m256i = _mm256_set1_epi32(-*cospi.add(32));
    let mut cospim36: __m256i = _mm256_set1_epi32(-*cospi.add(36));
    let mut cospim40: __m256i = _mm256_set1_epi32(-*cospi.add(40));
    let mut cospim44: __m256i = _mm256_set1_epi32(-*cospi.add(44));
    let mut cospim48: __m256i = _mm256_set1_epi32(-*cospi.add(48));
    let mut cospim49: __m256i = _mm256_set1_epi32(-*cospi.add(49));
    let mut cospim50: __m256i = _mm256_set1_epi32(-*cospi.add(50));
    let mut cospim52: __m256i = _mm256_set1_epi32(-*cospi.add(52));
    let mut cospim53: __m256i = _mm256_set1_epi32(-*cospi.add(53));
    let mut cospim56: __m256i = _mm256_set1_epi32(-*cospi.add(56));
    let mut cospim57: __m256i = _mm256_set1_epi32(-*cospi.add(57));
    let mut cospim58: __m256i = _mm256_set1_epi32(-*cospi.add(58));
    let mut cospim60: __m256i = _mm256_set1_epi32(-*cospi.add(60));
    let mut cospim61: __m256i = _mm256_set1_epi32(-*cospi.add(61));

    {
        let mut u: [__m256i; 64] = core::mem::zeroed();
        let mut tmp1: __m256i;
        let mut tmp2: __m256i;
        let mut tmp3: __m256i;
        let mut tmp4: __m256i;
        // stage 1
        u[0] = *in_.add(0);
        u[32] = *in_.add(1);
        u[36] = *in_.add(9);
        u[40] = *in_.add(5);
        u[44] = *in_.add(13);
        u[48] = *in_.add(3);
        u[52] = *in_.add(11);
        u[56] = *in_.add(7);
        u[60] = *in_.add(15);
        u[16] = *in_.add(2);
        u[20] = *in_.add(10);
        u[24] = *in_.add(6);
        u[28] = *in_.add(14);
        u[4] = *in_.add(8);
        u[8] = *in_.add(4);
        u[12] = *in_.add(12);

        // stage 2
        u[63] = half_btf_0_avx2(&cospi1, u.as_mut_ptr().add(32), &rnding, bit);
        u[32] = half_btf_0_avx2(&cospi63, u.as_mut_ptr().add(32), &rnding, bit);
        u[35] = half_btf_0_avx2(&cospim49, u.as_mut_ptr().add(60), &rnding, bit);
        u[60] = half_btf_0_avx2(&cospi15, u.as_mut_ptr().add(60), &rnding, bit);
        u[59] = half_btf_0_avx2(&cospi9, u.as_mut_ptr().add(36), &rnding, bit);
        u[36] = half_btf_0_avx2(&cospi55, u.as_mut_ptr().add(36), &rnding, bit);
        u[39] = half_btf_0_avx2(&cospim57, u.as_mut_ptr().add(56), &rnding, bit);
        u[56] = half_btf_0_avx2(&cospi7, u.as_mut_ptr().add(56), &rnding, bit);
        u[55] = half_btf_0_avx2(&cospi5, u.as_mut_ptr().add(40), &rnding, bit);
        u[40] = half_btf_0_avx2(&cospi59, u.as_mut_ptr().add(40), &rnding, bit);
        u[43] = half_btf_0_avx2(&cospim53, u.as_mut_ptr().add(52), &rnding, bit);
        u[52] = half_btf_0_avx2(&cospi11, u.as_mut_ptr().add(52), &rnding, bit);
        u[47] = half_btf_0_avx2(&cospim61, u.as_mut_ptr().add(48), &rnding, bit);
        u[48] = half_btf_0_avx2(&cospi3, u.as_mut_ptr().add(48), &rnding, bit);
        u[51] = half_btf_0_avx2(&cospi13, u.as_mut_ptr().add(44), &rnding, bit);
        u[44] = half_btf_0_avx2(&cospi51, u.as_mut_ptr().add(44), &rnding, bit);

        // stage 3
        u[31] = half_btf_0_avx2(&cospi2, u.as_mut_ptr().add(16), &rnding, bit);
        u[16] = half_btf_0_avx2(&cospi62, u.as_mut_ptr().add(16), &rnding, bit);
        u[19] = half_btf_0_avx2(&cospim50, u.as_mut_ptr().add(28), &rnding, bit);
        u[28] = half_btf_0_avx2(&cospi14, u.as_mut_ptr().add(28), &rnding, bit);
        u[27] = half_btf_0_avx2(&cospi10, u.as_mut_ptr().add(20), &rnding, bit);
        u[20] = half_btf_0_avx2(&cospi54, u.as_mut_ptr().add(20), &rnding, bit);
        u[23] = half_btf_0_avx2(&cospim58, u.as_mut_ptr().add(24), &rnding, bit);
        u[24] = half_btf_0_avx2(&cospi6, u.as_mut_ptr().add(24), &rnding, bit);
        u[33] = u[32];
        u[34] = u[35];
        u[37] = u[36];
        u[38] = u[39];
        u[41] = u[40];
        u[42] = u[43];
        u[45] = u[44];
        u[46] = u[47];
        u[49] = u[48];
        u[50] = u[51];
        u[53] = u[52];
        u[54] = u[55];
        u[57] = u[56];
        u[58] = u[59];
        u[61] = u[60];
        u[62] = u[63];

        // stage 4
        u[15] = half_btf_0_avx2(&cospi4, u.as_mut_ptr().add(8), &rnding, bit);
        u[8] = half_btf_0_avx2(&cospi60, u.as_mut_ptr().add(8), &rnding, bit);
        u[11] = half_btf_0_avx2(&cospim52, u.as_mut_ptr().add(12), &rnding, bit);
        u[12] = half_btf_0_avx2(&cospi12, u.as_mut_ptr().add(12), &rnding, bit);

        u[17] = u[16];
        u[18] = u[19];
        u[21] = u[20];
        u[22] = u[23];
        u[25] = u[24];
        u[26] = u[27];
        u[29] = u[28];
        u[30] = u[31];

        tmp1 = half_btf_avx2(
            &cospim4,
            u.as_mut_ptr().add(33),
            &cospi60,
            u.as_mut_ptr().add(62),
            &rnding,
            bit,
        );
        tmp2 = half_btf_avx2(
            &cospim60,
            u.as_mut_ptr().add(34),
            &cospim4,
            u.as_mut_ptr().add(61),
            &rnding,
            bit,
        );
        tmp3 = half_btf_avx2(
            &cospim36,
            u.as_mut_ptr().add(37),
            &cospi28,
            u.as_mut_ptr().add(58),
            &rnding,
            bit,
        );
        tmp4 = half_btf_avx2(
            &cospim28,
            u.as_mut_ptr().add(38),
            &cospim36,
            u.as_mut_ptr().add(57),
            &rnding,
            bit,
        );
        u[57] = half_btf_avx2(
            &cospim36,
            u.as_mut_ptr().add(38),
            &cospi28,
            u.as_mut_ptr().add(57),
            &rnding,
            bit,
        );
        u[58] = half_btf_avx2(
            &cospi28,
            u.as_mut_ptr().add(37),
            &cospi36,
            u.as_mut_ptr().add(58),
            &rnding,
            bit,
        );
        u[61] = half_btf_avx2(
            &cospim4,
            u.as_mut_ptr().add(34),
            &cospi60,
            u.as_mut_ptr().add(61),
            &rnding,
            bit,
        );
        u[62] = half_btf_avx2(
            &cospi60,
            u.as_mut_ptr().add(33),
            &cospi4,
            u.as_mut_ptr().add(62),
            &rnding,
            bit,
        );
        u[33] = tmp1;
        u[34] = tmp2;
        u[37] = tmp3;
        u[38] = tmp4;

        tmp1 = half_btf_avx2(
            &cospim20,
            u.as_mut_ptr().add(41),
            &cospi44,
            u.as_mut_ptr().add(54),
            &rnding,
            bit,
        );
        tmp2 = half_btf_avx2(
            &cospim44,
            u.as_mut_ptr().add(42),
            &cospim20,
            u.as_mut_ptr().add(53),
            &rnding,
            bit,
        );
        tmp3 = half_btf_avx2(
            &cospim52,
            u.as_mut_ptr().add(45),
            &cospi12,
            u.as_mut_ptr().add(50),
            &rnding,
            bit,
        );
        tmp4 = half_btf_avx2(
            &cospim12,
            u.as_mut_ptr().add(46),
            &cospim52,
            u.as_mut_ptr().add(49),
            &rnding,
            bit,
        );
        u[49] = half_btf_avx2(
            &cospim52,
            u.as_mut_ptr().add(46),
            &cospi12,
            u.as_mut_ptr().add(49),
            &rnding,
            bit,
        );
        u[50] = half_btf_avx2(
            &cospi12,
            u.as_mut_ptr().add(45),
            &cospi52,
            u.as_mut_ptr().add(50),
            &rnding,
            bit,
        );
        u[53] = half_btf_avx2(
            &cospim20,
            u.as_mut_ptr().add(42),
            &cospi44,
            u.as_mut_ptr().add(53),
            &rnding,
            bit,
        );
        u[54] = half_btf_avx2(
            &cospi44,
            u.as_mut_ptr().add(41),
            &cospi20,
            u.as_mut_ptr().add(54),
            &rnding,
            bit,
        );
        u[41] = tmp1;
        u[42] = tmp2;
        u[45] = tmp3;
        u[46] = tmp4;

        // stage 5
        u[7] = half_btf_0_avx2(&cospi8, u.as_mut_ptr().add(4), &rnding, bit);
        u[4] = half_btf_0_avx2(&cospi56, u.as_mut_ptr().add(4), &rnding, bit);

        u[9] = u[8];
        u[10] = u[11];
        u[13] = u[12];
        u[14] = u[15];

        tmp1 = half_btf_avx2(
            &cospim8,
            u.as_mut_ptr().add(17),
            &cospi56,
            u.as_mut_ptr().add(30),
            &rnding,
            bit,
        );
        tmp2 = half_btf_avx2(
            &cospim56,
            u.as_mut_ptr().add(18),
            &cospim8,
            u.as_mut_ptr().add(29),
            &rnding,
            bit,
        );
        tmp3 = half_btf_avx2(
            &cospim40,
            u.as_mut_ptr().add(21),
            &cospi24,
            u.as_mut_ptr().add(26),
            &rnding,
            bit,
        );
        tmp4 = half_btf_avx2(
            &cospim24,
            u.as_mut_ptr().add(22),
            &cospim40,
            u.as_mut_ptr().add(25),
            &rnding,
            bit,
        );
        u[25] = half_btf_avx2(
            &cospim40,
            u.as_mut_ptr().add(22),
            &cospi24,
            u.as_mut_ptr().add(25),
            &rnding,
            bit,
        );
        u[26] = half_btf_avx2(
            &cospi24,
            u.as_mut_ptr().add(21),
            &cospi40,
            u.as_mut_ptr().add(26),
            &rnding,
            bit,
        );
        u[29] = half_btf_avx2(
            &cospim8,
            u.as_mut_ptr().add(18),
            &cospi56,
            u.as_mut_ptr().add(29),
            &rnding,
            bit,
        );
        u[30] = half_btf_avx2(
            &cospi56,
            u.as_mut_ptr().add(17),
            &cospi8,
            u.as_mut_ptr().add(30),
            &rnding,
            bit,
        );
        u[17] = tmp1;
        u[18] = tmp2;
        u[21] = tmp3;
        u[22] = tmp4;

        for i in (32..64).step_by(8) {
            addsub_avx2(
                u[(i + 0) as usize],
                u[(i + 3) as usize],
                u.as_mut_ptr().offset((i + 0) as isize),
                u.as_mut_ptr().offset((i + 3) as isize),
                &clamp_lo,
                &clamp_hi,
            );
            addsub_avx2(
                u[(i + 1) as usize],
                u[(i + 2) as usize],
                u.as_mut_ptr().offset((i + 1) as isize),
                u.as_mut_ptr().offset((i + 2) as isize),
                &clamp_lo,
                &clamp_hi,
            );

            addsub_avx2(
                u[(i + 7) as usize],
                u[(i + 4) as usize],
                u.as_mut_ptr().offset((i + 7) as isize),
                u.as_mut_ptr().offset((i + 4) as isize),
                &clamp_lo,
                &clamp_hi,
            );
            addsub_avx2(
                u[(i + 6) as usize],
                u[(i + 5) as usize],
                u.as_mut_ptr().offset((i + 6) as isize),
                u.as_mut_ptr().offset((i + 5) as isize),
                &clamp_lo,
                &clamp_hi,
            );
        }

        // stage 6
        tmp1 = half_btf_0_avx2(&cospi32, u.as_mut_ptr().add(0), &rnding, bit);
        u[1] = half_btf_0_avx2(&cospi32, u.as_mut_ptr().add(0), &rnding, bit);
        u[0] = tmp1;
        u[5] = u[4];
        u[6] = u[7];

        tmp1 = half_btf_avx2(
            &cospim16,
            u.as_mut_ptr().add(9),
            &cospi48,
            u.as_mut_ptr().add(14),
            &rnding,
            bit,
        );
        u[14] = half_btf_avx2(
            &cospi48,
            u.as_mut_ptr().add(9),
            &cospi16,
            u.as_mut_ptr().add(14),
            &rnding,
            bit,
        );
        u[9] = tmp1;
        tmp2 = half_btf_avx2(
            &cospim48,
            u.as_mut_ptr().add(10),
            &cospim16,
            u.as_mut_ptr().add(13),
            &rnding,
            bit,
        );
        u[13] = half_btf_avx2(
            &cospim16,
            u.as_mut_ptr().add(10),
            &cospi48,
            u.as_mut_ptr().add(13),
            &rnding,
            bit,
        );
        u[10] = tmp2;

        for i in (16..32).step_by(8) {
            addsub_avx2(
                u[(i + 0) as usize],
                u[(i + 3) as usize],
                u.as_mut_ptr().offset((i + 0) as isize),
                u.as_mut_ptr().offset((i + 3) as isize),
                &clamp_lo,
                &clamp_hi,
            );
            addsub_avx2(
                u[(i + 1) as usize],
                u[(i + 2) as usize],
                u.as_mut_ptr().offset((i + 1) as isize),
                u.as_mut_ptr().offset((i + 2) as isize),
                &clamp_lo,
                &clamp_hi,
            );

            addsub_avx2(
                u[(i + 7) as usize],
                u[(i + 4) as usize],
                u.as_mut_ptr().offset((i + 7) as isize),
                u.as_mut_ptr().offset((i + 4) as isize),
                &clamp_lo,
                &clamp_hi,
            );
            addsub_avx2(
                u[(i + 6) as usize],
                u[(i + 5) as usize],
                u.as_mut_ptr().offset((i + 6) as isize),
                u.as_mut_ptr().offset((i + 5) as isize),
                &clamp_lo,
                &clamp_hi,
            );
        }

        tmp1 = half_btf_avx2(
            &cospim8,
            u.as_mut_ptr().add(34),
            &cospi56,
            u.as_mut_ptr().add(61),
            &rnding,
            bit,
        );
        tmp2 = half_btf_avx2(
            &cospim8,
            u.as_mut_ptr().add(35),
            &cospi56,
            u.as_mut_ptr().add(60),
            &rnding,
            bit,
        );
        tmp3 = half_btf_avx2(
            &cospim56,
            u.as_mut_ptr().add(36),
            &cospim8,
            u.as_mut_ptr().add(59),
            &rnding,
            bit,
        );
        tmp4 = half_btf_avx2(
            &cospim56,
            u.as_mut_ptr().add(37),
            &cospim8,
            u.as_mut_ptr().add(58),
            &rnding,
            bit,
        );
        u[58] = half_btf_avx2(
            &cospim8,
            u.as_mut_ptr().add(37),
            &cospi56,
            u.as_mut_ptr().add(58),
            &rnding,
            bit,
        );
        u[59] = half_btf_avx2(
            &cospim8,
            u.as_mut_ptr().add(36),
            &cospi56,
            u.as_mut_ptr().add(59),
            &rnding,
            bit,
        );
        u[60] = half_btf_avx2(
            &cospi56,
            u.as_mut_ptr().add(35),
            &cospi8,
            u.as_mut_ptr().add(60),
            &rnding,
            bit,
        );
        u[61] = half_btf_avx2(
            &cospi56,
            u.as_mut_ptr().add(34),
            &cospi8,
            u.as_mut_ptr().add(61),
            &rnding,
            bit,
        );
        u[34] = tmp1;
        u[35] = tmp2;
        u[36] = tmp3;
        u[37] = tmp4;

        tmp1 = half_btf_avx2(
            &cospim40,
            u.as_mut_ptr().add(42),
            &cospi24,
            u.as_mut_ptr().add(53),
            &rnding,
            bit,
        );
        tmp2 = half_btf_avx2(
            &cospim40,
            u.as_mut_ptr().add(43),
            &cospi24,
            u.as_mut_ptr().add(52),
            &rnding,
            bit,
        );
        tmp3 = half_btf_avx2(
            &cospim24,
            u.as_mut_ptr().add(44),
            &cospim40,
            u.as_mut_ptr().add(51),
            &rnding,
            bit,
        );
        tmp4 = half_btf_avx2(
            &cospim24,
            u.as_mut_ptr().add(45),
            &cospim40,
            u.as_mut_ptr().add(50),
            &rnding,
            bit,
        );
        u[50] = half_btf_avx2(
            &cospim40,
            u.as_mut_ptr().add(45),
            &cospi24,
            u.as_mut_ptr().add(50),
            &rnding,
            bit,
        );
        u[51] = half_btf_avx2(
            &cospim40,
            u.as_mut_ptr().add(44),
            &cospi24,
            u.as_mut_ptr().add(51),
            &rnding,
            bit,
        );
        u[52] = half_btf_avx2(
            &cospi24,
            u.as_mut_ptr().add(43),
            &cospi40,
            u.as_mut_ptr().add(52),
            &rnding,
            bit,
        );
        u[53] = half_btf_avx2(
            &cospi24,
            u.as_mut_ptr().add(42),
            &cospi40,
            u.as_mut_ptr().add(53),
            &rnding,
            bit,
        );
        u[42] = tmp1;
        u[43] = tmp2;
        u[44] = tmp3;
        u[45] = tmp4;

        // stage 7
        u[3] = u[0];
        u[2] = u[1];
        tmp1 = half_btf_avx2(
            &cospim32,
            u.as_mut_ptr().add(5),
            &cospi32,
            u.as_mut_ptr().add(6),
            &rnding,
            bit,
        );
        u[6] = half_btf_avx2(
            &cospi32,
            u.as_mut_ptr().add(5),
            &cospi32,
            u.as_mut_ptr().add(6),
            &rnding,
            bit,
        );
        u[5] = tmp1;
        addsub_avx2(
            u[8],
            u[11],
            u.as_mut_ptr().add(8),
            u.as_mut_ptr().add(11),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            u[9],
            u[10],
            u.as_mut_ptr().add(9),
            u.as_mut_ptr().add(10),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            u[15],
            u[12],
            u.as_mut_ptr().add(15),
            u.as_mut_ptr().add(12),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            u[14],
            u[13],
            u.as_mut_ptr().add(14),
            u.as_mut_ptr().add(13),
            &clamp_lo,
            &clamp_hi,
        );

        tmp1 = half_btf_avx2(
            &cospim16,
            u.as_mut_ptr().add(18),
            &cospi48,
            u.as_mut_ptr().add(29),
            &rnding,
            bit,
        );
        tmp2 = half_btf_avx2(
            &cospim16,
            u.as_mut_ptr().add(19),
            &cospi48,
            u.as_mut_ptr().add(28),
            &rnding,
            bit,
        );
        tmp3 = half_btf_avx2(
            &cospim48,
            u.as_mut_ptr().add(20),
            &cospim16,
            u.as_mut_ptr().add(27),
            &rnding,
            bit,
        );
        tmp4 = half_btf_avx2(
            &cospim48,
            u.as_mut_ptr().add(21),
            &cospim16,
            u.as_mut_ptr().add(26),
            &rnding,
            bit,
        );
        u[26] = half_btf_avx2(
            &cospim16,
            u.as_mut_ptr().add(21),
            &cospi48,
            u.as_mut_ptr().add(26),
            &rnding,
            bit,
        );
        u[27] = half_btf_avx2(
            &cospim16,
            u.as_mut_ptr().add(20),
            &cospi48,
            u.as_mut_ptr().add(27),
            &rnding,
            bit,
        );
        u[28] = half_btf_avx2(
            &cospi48,
            u.as_mut_ptr().add(19),
            &cospi16,
            u.as_mut_ptr().add(28),
            &rnding,
            bit,
        );
        u[29] = half_btf_avx2(
            &cospi48,
            u.as_mut_ptr().add(18),
            &cospi16,
            u.as_mut_ptr().add(29),
            &rnding,
            bit,
        );
        u[18] = tmp1;
        u[19] = tmp2;
        u[20] = tmp3;
        u[21] = tmp4;

        for i in (32..64).step_by(16) {
            for j in i..i + 4 {
                addsub_avx2(
                    u[j as usize],
                    u[(j ^ 7) as usize],
                    u.as_mut_ptr().offset(j as isize),
                    u.as_mut_ptr().offset((j ^ 7) as isize),
                    &clamp_lo,
                    &clamp_hi,
                );
                addsub_avx2(
                    u[(j ^ 15) as usize],
                    u[(j ^ 8) as usize],
                    u.as_mut_ptr().offset((j ^ 15) as isize),
                    u.as_mut_ptr().offset((j ^ 8) as isize),
                    &clamp_lo,
                    &clamp_hi,
                );
            }
        }

        // stage 8
        for i in 0..4 {
            addsub_avx2(
                u[i as usize],
                u[(7 - i) as usize],
                u.as_mut_ptr().offset(i as isize),
                u.as_mut_ptr().offset((7 - i) as isize),
                &clamp_lo,
                &clamp_hi,
            );
        }

        idct64_stage8_avx2(
            u.as_mut_ptr(),
            &cospim32,
            &cospi32,
            &cospim16,
            &cospi48,
            &cospi16,
            &cospim48,
            &clamp_lo,
            &clamp_hi,
            &rnding,
            bit,
        );

        // stage 9
        idct64_stage9_avx2(u.as_mut_ptr(), &cospim32, &cospi32, &clamp_lo, &clamp_hi, &rnding, bit);

        // stage 10
        idct64_stage10_avx2(
            u.as_mut_ptr(),
            &cospim32,
            &cospi32,
            &clamp_lo,
            &clamp_hi,
            &rnding,
            bit,
        );

        // stage 11
        idct64_stage11_avx2(u.as_mut_ptr(), out, do_cols, bd, out_shift, &clamp_lo, &clamp_hi);
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn idct64_avx2(
    mut in_: *mut __m256i,
    mut out: *mut __m256i,
    mut bit: i32,
    mut do_cols: i32,
    mut bd: i32,
    mut out_shift: i32,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut cospi: *const i32 = cospi_arr(bit);
    let mut rnding: __m256i = _mm256_set1_epi32(1 << (bit - 1));
    let mut log_range: i32 = get_log_range(bd, do_cols);
    let mut clamp_lo: __m256i = _mm256_set1_epi32(-(1 << (log_range - 1)));
    let mut clamp_hi: __m256i = _mm256_set1_epi32((1 << (log_range - 1)) - 1);

    let mut cospi1: __m256i = _mm256_set1_epi32(*cospi.add(1));
    let mut cospi2: __m256i = _mm256_set1_epi32(*cospi.add(2));
    let mut cospi3: __m256i = _mm256_set1_epi32(*cospi.add(3));
    let mut cospi4: __m256i = _mm256_set1_epi32(*cospi.add(4));
    let mut cospi5: __m256i = _mm256_set1_epi32(*cospi.add(5));
    let mut cospi6: __m256i = _mm256_set1_epi32(*cospi.add(6));
    let mut cospi7: __m256i = _mm256_set1_epi32(*cospi.add(7));
    let mut cospi8: __m256i = _mm256_set1_epi32(*cospi.add(8));
    let mut cospi9: __m256i = _mm256_set1_epi32(*cospi.add(9));
    let mut cospi10: __m256i = _mm256_set1_epi32(*cospi.add(10));
    let mut cospi11: __m256i = _mm256_set1_epi32(*cospi.add(11));
    let mut cospi12: __m256i = _mm256_set1_epi32(*cospi.add(12));
    let mut cospi13: __m256i = _mm256_set1_epi32(*cospi.add(13));
    let mut cospi14: __m256i = _mm256_set1_epi32(*cospi.add(14));
    let mut cospi15: __m256i = _mm256_set1_epi32(*cospi.add(15));
    let mut cospi16: __m256i = _mm256_set1_epi32(*cospi.add(16));
    let mut cospi17: __m256i = _mm256_set1_epi32(*cospi.add(17));
    let mut cospi18: __m256i = _mm256_set1_epi32(*cospi.add(18));
    let mut cospi19: __m256i = _mm256_set1_epi32(*cospi.add(19));
    let mut cospi20: __m256i = _mm256_set1_epi32(*cospi.add(20));
    let mut cospi21: __m256i = _mm256_set1_epi32(*cospi.add(21));
    let mut cospi22: __m256i = _mm256_set1_epi32(*cospi.add(22));
    let mut cospi23: __m256i = _mm256_set1_epi32(*cospi.add(23));
    let mut cospi24: __m256i = _mm256_set1_epi32(*cospi.add(24));
    let mut cospi25: __m256i = _mm256_set1_epi32(*cospi.add(25));
    let mut cospi26: __m256i = _mm256_set1_epi32(*cospi.add(26));
    let mut cospi27: __m256i = _mm256_set1_epi32(*cospi.add(27));
    let mut cospi28: __m256i = _mm256_set1_epi32(*cospi.add(28));
    let mut cospi29: __m256i = _mm256_set1_epi32(*cospi.add(29));
    let mut cospi30: __m256i = _mm256_set1_epi32(*cospi.add(30));
    let mut cospi31: __m256i = _mm256_set1_epi32(*cospi.add(31));
    let mut cospi32: __m256i = _mm256_set1_epi32(*cospi.add(32));
    let mut cospi35: __m256i = _mm256_set1_epi32(*cospi.add(35));
    let mut cospi36: __m256i = _mm256_set1_epi32(*cospi.add(36));
    let mut cospi38: __m256i = _mm256_set1_epi32(*cospi.add(38));
    let mut cospi39: __m256i = _mm256_set1_epi32(*cospi.add(39));
    let mut cospi40: __m256i = _mm256_set1_epi32(*cospi.add(40));
    let mut cospi43: __m256i = _mm256_set1_epi32(*cospi.add(43));
    let mut cospi44: __m256i = _mm256_set1_epi32(*cospi.add(44));
    let mut cospi46: __m256i = _mm256_set1_epi32(*cospi.add(46));
    let mut cospi47: __m256i = _mm256_set1_epi32(*cospi.add(47));
    let mut cospi48: __m256i = _mm256_set1_epi32(*cospi.add(48));
    let mut cospi51: __m256i = _mm256_set1_epi32(*cospi.add(51));
    let mut cospi52: __m256i = _mm256_set1_epi32(*cospi.add(52));
    let mut cospi54: __m256i = _mm256_set1_epi32(*cospi.add(54));
    let mut cospi55: __m256i = _mm256_set1_epi32(*cospi.add(55));
    let mut cospi56: __m256i = _mm256_set1_epi32(*cospi.add(56));
    let mut cospi59: __m256i = _mm256_set1_epi32(*cospi.add(59));
    let mut cospi60: __m256i = _mm256_set1_epi32(*cospi.add(60));
    let mut cospi62: __m256i = _mm256_set1_epi32(*cospi.add(62));
    let mut cospi63: __m256i = _mm256_set1_epi32(*cospi.add(63));

    let mut cospim4: __m256i = _mm256_set1_epi32(-*cospi.add(4));
    let mut cospim8: __m256i = _mm256_set1_epi32(-*cospi.add(8));
    let mut cospim12: __m256i = _mm256_set1_epi32(-*cospi.add(12));
    let mut cospim16: __m256i = _mm256_set1_epi32(-*cospi.add(16));
    let mut cospim20: __m256i = _mm256_set1_epi32(-*cospi.add(20));
    let mut cospim24: __m256i = _mm256_set1_epi32(-*cospi.add(24));
    let mut cospim28: __m256i = _mm256_set1_epi32(-*cospi.add(28));
    let mut cospim32: __m256i = _mm256_set1_epi32(-*cospi.add(32));
    let mut cospim33: __m256i = _mm256_set1_epi32(-*cospi.add(33));
    let mut cospim34: __m256i = _mm256_set1_epi32(-*cospi.add(34));
    let mut cospim36: __m256i = _mm256_set1_epi32(-*cospi.add(36));
    let mut cospim37: __m256i = _mm256_set1_epi32(-*cospi.add(37));
    let mut cospim40: __m256i = _mm256_set1_epi32(-*cospi.add(40));
    let mut cospim41: __m256i = _mm256_set1_epi32(-*cospi.add(41));
    let mut cospim42: __m256i = _mm256_set1_epi32(-*cospi.add(42));
    let mut cospim44: __m256i = _mm256_set1_epi32(-*cospi.add(44));
    let mut cospim45: __m256i = _mm256_set1_epi32(-*cospi.add(45));
    let mut cospim48: __m256i = _mm256_set1_epi32(-*cospi.add(48));
    let mut cospim49: __m256i = _mm256_set1_epi32(-*cospi.add(49));
    let mut cospim50: __m256i = _mm256_set1_epi32(-*cospi.add(50));
    let mut cospim52: __m256i = _mm256_set1_epi32(-*cospi.add(52));
    let mut cospim53: __m256i = _mm256_set1_epi32(-*cospi.add(53));
    let mut cospim56: __m256i = _mm256_set1_epi32(-*cospi.add(56));
    let mut cospim57: __m256i = _mm256_set1_epi32(-*cospi.add(57));
    let mut cospim58: __m256i = _mm256_set1_epi32(-*cospi.add(58));
    let mut cospim60: __m256i = _mm256_set1_epi32(-*cospi.add(60));
    let mut cospim61: __m256i = _mm256_set1_epi32(-*cospi.add(61));

    {
        let mut u: [__m256i; 64] = core::mem::zeroed();
        let mut v: [__m256i; 64] = core::mem::zeroed();

        // stage 1
        u[32] = *in_.add(1);
        u[34] = *in_.add(17);
        u[36] = *in_.add(9);
        u[38] = *in_.add(25);
        u[40] = *in_.add(5);
        u[42] = *in_.add(21);
        u[44] = *in_.add(13);
        u[46] = *in_.add(29);
        u[48] = *in_.add(3);
        u[50] = *in_.add(19);
        u[52] = *in_.add(11);
        u[54] = *in_.add(27);
        u[56] = *in_.add(7);
        u[58] = *in_.add(23);
        u[60] = *in_.add(15);
        u[62] = *in_.add(31);

        v[16] = *in_.add(2);
        v[18] = *in_.add(18);
        v[20] = *in_.add(10);
        v[22] = *in_.add(26);
        v[24] = *in_.add(6);
        v[26] = *in_.add(22);
        v[28] = *in_.add(14);
        v[30] = *in_.add(30);

        u[8] = *in_.add(4);
        u[10] = *in_.add(20);
        u[12] = *in_.add(12);
        u[14] = *in_.add(28);

        v[4] = *in_.add(8);
        v[6] = *in_.add(24);

        u[0] = *in_.add(0);
        u[2] = *in_.add(16);

        // stage 2
        v[32] = half_btf_0_avx2(&cospi63, u.as_mut_ptr().add(32), &rnding, bit);
        v[33] = half_btf_0_avx2(&cospim33, u.as_mut_ptr().add(62), &rnding, bit);
        v[34] = half_btf_0_avx2(&cospi47, u.as_mut_ptr().add(34), &rnding, bit);
        v[35] = half_btf_0_avx2(&cospim49, u.as_mut_ptr().add(60), &rnding, bit);
        v[36] = half_btf_0_avx2(&cospi55, u.as_mut_ptr().add(36), &rnding, bit);
        v[37] = half_btf_0_avx2(&cospim41, u.as_mut_ptr().add(58), &rnding, bit);
        v[38] = half_btf_0_avx2(&cospi39, u.as_mut_ptr().add(38), &rnding, bit);
        v[39] = half_btf_0_avx2(&cospim57, u.as_mut_ptr().add(56), &rnding, bit);
        v[40] = half_btf_0_avx2(&cospi59, u.as_mut_ptr().add(40), &rnding, bit);
        v[41] = half_btf_0_avx2(&cospim37, u.as_mut_ptr().add(54), &rnding, bit);
        v[42] = half_btf_0_avx2(&cospi43, u.as_mut_ptr().add(42), &rnding, bit);
        v[43] = half_btf_0_avx2(&cospim53, u.as_mut_ptr().add(52), &rnding, bit);
        v[44] = half_btf_0_avx2(&cospi51, u.as_mut_ptr().add(44), &rnding, bit);
        v[45] = half_btf_0_avx2(&cospim45, u.as_mut_ptr().add(50), &rnding, bit);
        v[46] = half_btf_0_avx2(&cospi35, u.as_mut_ptr().add(46), &rnding, bit);
        v[47] = half_btf_0_avx2(&cospim61, u.as_mut_ptr().add(48), &rnding, bit);
        v[48] = half_btf_0_avx2(&cospi3, u.as_mut_ptr().add(48), &rnding, bit);
        v[49] = half_btf_0_avx2(&cospi29, u.as_mut_ptr().add(46), &rnding, bit);
        v[50] = half_btf_0_avx2(&cospi19, u.as_mut_ptr().add(50), &rnding, bit);
        v[51] = half_btf_0_avx2(&cospi13, u.as_mut_ptr().add(44), &rnding, bit);
        v[52] = half_btf_0_avx2(&cospi11, u.as_mut_ptr().add(52), &rnding, bit);
        v[53] = half_btf_0_avx2(&cospi21, u.as_mut_ptr().add(42), &rnding, bit);
        v[54] = half_btf_0_avx2(&cospi27, u.as_mut_ptr().add(54), &rnding, bit);
        v[55] = half_btf_0_avx2(&cospi5, u.as_mut_ptr().add(40), &rnding, bit);
        v[56] = half_btf_0_avx2(&cospi7, u.as_mut_ptr().add(56), &rnding, bit);
        v[57] = half_btf_0_avx2(&cospi25, u.as_mut_ptr().add(38), &rnding, bit);
        v[58] = half_btf_0_avx2(&cospi23, u.as_mut_ptr().add(58), &rnding, bit);
        v[59] = half_btf_0_avx2(&cospi9, u.as_mut_ptr().add(36), &rnding, bit);
        v[60] = half_btf_0_avx2(&cospi15, u.as_mut_ptr().add(60), &rnding, bit);
        v[61] = half_btf_0_avx2(&cospi17, u.as_mut_ptr().add(34), &rnding, bit);
        v[62] = half_btf_0_avx2(&cospi31, u.as_mut_ptr().add(62), &rnding, bit);
        v[63] = half_btf_0_avx2(&cospi1, u.as_mut_ptr().add(32), &rnding, bit);

        // stage 3
        u[16] = half_btf_0_avx2(&cospi62, v.as_mut_ptr().add(16), &rnding, bit);
        u[17] = half_btf_0_avx2(&cospim34, v.as_mut_ptr().add(30), &rnding, bit);
        u[18] = half_btf_0_avx2(&cospi46, v.as_mut_ptr().add(18), &rnding, bit);
        u[19] = half_btf_0_avx2(&cospim50, v.as_mut_ptr().add(28), &rnding, bit);
        u[20] = half_btf_0_avx2(&cospi54, v.as_mut_ptr().add(20), &rnding, bit);
        u[21] = half_btf_0_avx2(&cospim42, v.as_mut_ptr().add(26), &rnding, bit);
        u[22] = half_btf_0_avx2(&cospi38, v.as_mut_ptr().add(22), &rnding, bit);
        u[23] = half_btf_0_avx2(&cospim58, v.as_mut_ptr().add(24), &rnding, bit);
        u[24] = half_btf_0_avx2(&cospi6, v.as_mut_ptr().add(24), &rnding, bit);
        u[25] = half_btf_0_avx2(&cospi26, v.as_mut_ptr().add(22), &rnding, bit);
        u[26] = half_btf_0_avx2(&cospi22, v.as_mut_ptr().add(26), &rnding, bit);
        u[27] = half_btf_0_avx2(&cospi10, v.as_mut_ptr().add(20), &rnding, bit);
        u[28] = half_btf_0_avx2(&cospi14, v.as_mut_ptr().add(28), &rnding, bit);
        u[29] = half_btf_0_avx2(&cospi18, v.as_mut_ptr().add(18), &rnding, bit);
        u[30] = half_btf_0_avx2(&cospi30, v.as_mut_ptr().add(30), &rnding, bit);
        u[31] = half_btf_0_avx2(&cospi2, v.as_mut_ptr().add(16), &rnding, bit);

        for i in (32..64).step_by(4) {
            addsub_avx2(
                v[(i + 0) as usize],
                v[(i + 1) as usize],
                u.as_mut_ptr().offset((i + 0) as isize),
                u.as_mut_ptr().offset((i + 1) as isize),
                &clamp_lo,
                &clamp_hi,
            );
            addsub_avx2(
                v[(i + 3) as usize],
                v[(i + 2) as usize],
                u.as_mut_ptr().offset((i + 3) as isize),
                u.as_mut_ptr().offset((i + 2) as isize),
                &clamp_lo,
                &clamp_hi,
            );
        }

        // stage 4
        v[8] = half_btf_0_avx2(&cospi60, u.as_mut_ptr().add(8), &rnding, bit);
        v[9] = half_btf_0_avx2(&cospim36, u.as_mut_ptr().add(14), &rnding, bit);
        v[10] = half_btf_0_avx2(&cospi44, u.as_mut_ptr().add(10), &rnding, bit);
        v[11] = half_btf_0_avx2(&cospim52, u.as_mut_ptr().add(12), &rnding, bit);
        v[12] = half_btf_0_avx2(&cospi12, u.as_mut_ptr().add(12), &rnding, bit);
        v[13] = half_btf_0_avx2(&cospi20, u.as_mut_ptr().add(10), &rnding, bit);
        v[14] = half_btf_0_avx2(&cospi28, u.as_mut_ptr().add(14), &rnding, bit);
        v[15] = half_btf_0_avx2(&cospi4, u.as_mut_ptr().add(8), &rnding, bit);

        for i in (16..32).step_by(4) {
            addsub_avx2(
                u[(i + 0) as usize],
                u[(i + 1) as usize],
                v.as_mut_ptr().offset((i + 0) as isize),
                v.as_mut_ptr().offset((i + 1) as isize),
                &clamp_lo,
                &clamp_hi,
            );
            addsub_avx2(
                u[(i + 3) as usize],
                u[(i + 2) as usize],
                v.as_mut_ptr().offset((i + 3) as isize),
                v.as_mut_ptr().offset((i + 2) as isize),
                &clamp_lo,
                &clamp_hi,
            );
        }

        for i in (32..64).step_by(4) {
            v[(i + 0) as usize] = u[(i + 0) as usize];
            v[(i + 3) as usize] = u[(i + 3) as usize];
        }

        v[33] = half_btf_avx2(
            &cospim4,
            u.as_mut_ptr().add(33),
            &cospi60,
            u.as_mut_ptr().add(62),
            &rnding,
            bit,
        );
        v[34] = half_btf_avx2(
            &cospim60,
            u.as_mut_ptr().add(34),
            &cospim4,
            u.as_mut_ptr().add(61),
            &rnding,
            bit,
        );
        v[37] = half_btf_avx2(
            &cospim36,
            u.as_mut_ptr().add(37),
            &cospi28,
            u.as_mut_ptr().add(58),
            &rnding,
            bit,
        );
        v[38] = half_btf_avx2(
            &cospim28,
            u.as_mut_ptr().add(38),
            &cospim36,
            u.as_mut_ptr().add(57),
            &rnding,
            bit,
        );
        v[41] = half_btf_avx2(
            &cospim20,
            u.as_mut_ptr().add(41),
            &cospi44,
            u.as_mut_ptr().add(54),
            &rnding,
            bit,
        );
        v[42] = half_btf_avx2(
            &cospim44,
            u.as_mut_ptr().add(42),
            &cospim20,
            u.as_mut_ptr().add(53),
            &rnding,
            bit,
        );
        v[45] = half_btf_avx2(
            &cospim52,
            u.as_mut_ptr().add(45),
            &cospi12,
            u.as_mut_ptr().add(50),
            &rnding,
            bit,
        );
        v[46] = half_btf_avx2(
            &cospim12,
            u.as_mut_ptr().add(46),
            &cospim52,
            u.as_mut_ptr().add(49),
            &rnding,
            bit,
        );
        v[49] = half_btf_avx2(
            &cospim52,
            u.as_mut_ptr().add(46),
            &cospi12,
            u.as_mut_ptr().add(49),
            &rnding,
            bit,
        );
        v[50] = half_btf_avx2(
            &cospi12,
            u.as_mut_ptr().add(45),
            &cospi52,
            u.as_mut_ptr().add(50),
            &rnding,
            bit,
        );
        v[53] = half_btf_avx2(
            &cospim20,
            u.as_mut_ptr().add(42),
            &cospi44,
            u.as_mut_ptr().add(53),
            &rnding,
            bit,
        );
        v[54] = half_btf_avx2(
            &cospi44,
            u.as_mut_ptr().add(41),
            &cospi20,
            u.as_mut_ptr().add(54),
            &rnding,
            bit,
        );
        v[57] = half_btf_avx2(
            &cospim36,
            u.as_mut_ptr().add(38),
            &cospi28,
            u.as_mut_ptr().add(57),
            &rnding,
            bit,
        );
        v[58] = half_btf_avx2(
            &cospi28,
            u.as_mut_ptr().add(37),
            &cospi36,
            u.as_mut_ptr().add(58),
            &rnding,
            bit,
        );
        v[61] = half_btf_avx2(
            &cospim4,
            u.as_mut_ptr().add(34),
            &cospi60,
            u.as_mut_ptr().add(61),
            &rnding,
            bit,
        );
        v[62] = half_btf_avx2(
            &cospi60,
            u.as_mut_ptr().add(33),
            &cospi4,
            u.as_mut_ptr().add(62),
            &rnding,
            bit,
        );

        // stage 5
        u[4] = half_btf_0_avx2(&cospi56, v.as_mut_ptr().add(4), &rnding, bit);
        u[5] = half_btf_0_avx2(&cospim40, v.as_mut_ptr().add(6), &rnding, bit);
        u[6] = half_btf_0_avx2(&cospi24, v.as_mut_ptr().add(6), &rnding, bit);
        u[7] = half_btf_0_avx2(&cospi8, v.as_mut_ptr().add(4), &rnding, bit);

        for i in (8..16).step_by(4) {
            addsub_avx2(
                v[(i + 0) as usize],
                v[(i + 1) as usize],
                u.as_mut_ptr().offset((i + 0) as isize),
                u.as_mut_ptr().offset((i + 1) as isize),
                &clamp_lo,
                &clamp_hi,
            );
            addsub_avx2(
                v[(i + 3) as usize],
                v[(i + 2) as usize],
                u.as_mut_ptr().offset((i + 3) as isize),
                u.as_mut_ptr().offset((i + 2) as isize),
                &clamp_lo,
                &clamp_hi,
            );
        }

        for i in (16..32).step_by(4) {
            u[(i + 0) as usize] = v[(i + 0) as usize];
            u[(i + 3) as usize] = v[(i + 3) as usize];
        }

        u[17] = half_btf_avx2(
            &cospim8,
            v.as_mut_ptr().add(17),
            &cospi56,
            v.as_mut_ptr().add(30),
            &rnding,
            bit,
        );
        u[18] = half_btf_avx2(
            &cospim56,
            v.as_mut_ptr().add(18),
            &cospim8,
            v.as_mut_ptr().add(29),
            &rnding,
            bit,
        );
        u[21] = half_btf_avx2(
            &cospim40,
            v.as_mut_ptr().add(21),
            &cospi24,
            v.as_mut_ptr().add(26),
            &rnding,
            bit,
        );
        u[22] = half_btf_avx2(
            &cospim24,
            v.as_mut_ptr().add(22),
            &cospim40,
            v.as_mut_ptr().add(25),
            &rnding,
            bit,
        );
        u[25] = half_btf_avx2(
            &cospim40,
            v.as_mut_ptr().add(22),
            &cospi24,
            v.as_mut_ptr().add(25),
            &rnding,
            bit,
        );
        u[26] = half_btf_avx2(
            &cospi24,
            v.as_mut_ptr().add(21),
            &cospi40,
            v.as_mut_ptr().add(26),
            &rnding,
            bit,
        );
        u[29] = half_btf_avx2(
            &cospim8,
            v.as_mut_ptr().add(18),
            &cospi56,
            v.as_mut_ptr().add(29),
            &rnding,
            bit,
        );
        u[30] = half_btf_avx2(
            &cospi56,
            v.as_mut_ptr().add(17),
            &cospi8,
            v.as_mut_ptr().add(30),
            &rnding,
            bit,
        );

        for i in (32..64).step_by(8) {
            addsub_avx2(
                v[(i + 0) as usize],
                v[(i + 3) as usize],
                u.as_mut_ptr().offset((i + 0) as isize),
                u.as_mut_ptr().offset((i + 3) as isize),
                &clamp_lo,
                &clamp_hi,
            );
            addsub_avx2(
                v[(i + 1) as usize],
                v[(i + 2) as usize],
                u.as_mut_ptr().offset((i + 1) as isize),
                u.as_mut_ptr().offset((i + 2) as isize),
                &clamp_lo,
                &clamp_hi,
            );

            addsub_avx2(
                v[(i + 7) as usize],
                v[(i + 4) as usize],
                u.as_mut_ptr().offset((i + 7) as isize),
                u.as_mut_ptr().offset((i + 4) as isize),
                &clamp_lo,
                &clamp_hi,
            );
            addsub_avx2(
                v[(i + 6) as usize],
                v[(i + 5) as usize],
                u.as_mut_ptr().offset((i + 6) as isize),
                u.as_mut_ptr().offset((i + 5) as isize),
                &clamp_lo,
                &clamp_hi,
            );
        }

        // stage 6
        v[0] = half_btf_0_avx2(&cospi32, u.as_mut_ptr().add(0), &rnding, bit);
        v[1] = half_btf_0_avx2(&cospi32, u.as_mut_ptr().add(0), &rnding, bit);
        v[2] = half_btf_0_avx2(&cospi48, u.as_mut_ptr().add(2), &rnding, bit);
        v[3] = half_btf_0_avx2(&cospi16, u.as_mut_ptr().add(2), &rnding, bit);

        addsub_avx2(u[4], u[5], v.as_mut_ptr().add(4), v.as_mut_ptr().add(5), &clamp_lo, &clamp_hi);
        addsub_avx2(u[7], u[6], v.as_mut_ptr().add(7), v.as_mut_ptr().add(6), &clamp_lo, &clamp_hi);

        for i in (8..16).step_by(4) {
            v[(i + 0) as usize] = u[(i + 0) as usize];
            v[(i + 3) as usize] = u[(i + 3) as usize];
        }

        v[9] = half_btf_avx2(
            &cospim16,
            u.as_mut_ptr().add(9),
            &cospi48,
            u.as_mut_ptr().add(14),
            &rnding,
            bit,
        );
        v[10] = half_btf_avx2(
            &cospim48,
            u.as_mut_ptr().add(10),
            &cospim16,
            u.as_mut_ptr().add(13),
            &rnding,
            bit,
        );
        v[13] = half_btf_avx2(
            &cospim16,
            u.as_mut_ptr().add(10),
            &cospi48,
            u.as_mut_ptr().add(13),
            &rnding,
            bit,
        );
        v[14] = half_btf_avx2(
            &cospi48,
            u.as_mut_ptr().add(9),
            &cospi16,
            u.as_mut_ptr().add(14),
            &rnding,
            bit,
        );

        for i in (16..32).step_by(8) {
            addsub_avx2(
                u[(i + 0) as usize],
                u[(i + 3) as usize],
                v.as_mut_ptr().offset((i + 0) as isize),
                v.as_mut_ptr().offset((i + 3) as isize),
                &clamp_lo,
                &clamp_hi,
            );
            addsub_avx2(
                u[(i + 1) as usize],
                u[(i + 2) as usize],
                v.as_mut_ptr().offset((i + 1) as isize),
                v.as_mut_ptr().offset((i + 2) as isize),
                &clamp_lo,
                &clamp_hi,
            );

            addsub_avx2(
                u[(i + 7) as usize],
                u[(i + 4) as usize],
                v.as_mut_ptr().offset((i + 7) as isize),
                v.as_mut_ptr().offset((i + 4) as isize),
                &clamp_lo,
                &clamp_hi,
            );
            addsub_avx2(
                u[(i + 6) as usize],
                u[(i + 5) as usize],
                v.as_mut_ptr().offset((i + 6) as isize),
                v.as_mut_ptr().offset((i + 5) as isize),
                &clamp_lo,
                &clamp_hi,
            );
        }

        for i in (32..64).step_by(8) {
            v[(i + 0) as usize] = u[(i + 0) as usize];
            v[(i + 1) as usize] = u[(i + 1) as usize];
            v[(i + 6) as usize] = u[(i + 6) as usize];
            v[(i + 7) as usize] = u[(i + 7) as usize];
        }

        v[34] = half_btf_avx2(
            &cospim8,
            u.as_mut_ptr().add(34),
            &cospi56,
            u.as_mut_ptr().add(61),
            &rnding,
            bit,
        );
        v[35] = half_btf_avx2(
            &cospim8,
            u.as_mut_ptr().add(35),
            &cospi56,
            u.as_mut_ptr().add(60),
            &rnding,
            bit,
        );
        v[36] = half_btf_avx2(
            &cospim56,
            u.as_mut_ptr().add(36),
            &cospim8,
            u.as_mut_ptr().add(59),
            &rnding,
            bit,
        );
        v[37] = half_btf_avx2(
            &cospim56,
            u.as_mut_ptr().add(37),
            &cospim8,
            u.as_mut_ptr().add(58),
            &rnding,
            bit,
        );
        v[42] = half_btf_avx2(
            &cospim40,
            u.as_mut_ptr().add(42),
            &cospi24,
            u.as_mut_ptr().add(53),
            &rnding,
            bit,
        );
        v[43] = half_btf_avx2(
            &cospim40,
            u.as_mut_ptr().add(43),
            &cospi24,
            u.as_mut_ptr().add(52),
            &rnding,
            bit,
        );
        v[44] = half_btf_avx2(
            &cospim24,
            u.as_mut_ptr().add(44),
            &cospim40,
            u.as_mut_ptr().add(51),
            &rnding,
            bit,
        );
        v[45] = half_btf_avx2(
            &cospim24,
            u.as_mut_ptr().add(45),
            &cospim40,
            u.as_mut_ptr().add(50),
            &rnding,
            bit,
        );
        v[50] = half_btf_avx2(
            &cospim40,
            u.as_mut_ptr().add(45),
            &cospi24,
            u.as_mut_ptr().add(50),
            &rnding,
            bit,
        );
        v[51] = half_btf_avx2(
            &cospim40,
            u.as_mut_ptr().add(44),
            &cospi24,
            u.as_mut_ptr().add(51),
            &rnding,
            bit,
        );
        v[52] = half_btf_avx2(
            &cospi24,
            u.as_mut_ptr().add(43),
            &cospi40,
            u.as_mut_ptr().add(52),
            &rnding,
            bit,
        );
        v[53] = half_btf_avx2(
            &cospi24,
            u.as_mut_ptr().add(42),
            &cospi40,
            u.as_mut_ptr().add(53),
            &rnding,
            bit,
        );
        v[58] = half_btf_avx2(
            &cospim8,
            u.as_mut_ptr().add(37),
            &cospi56,
            u.as_mut_ptr().add(58),
            &rnding,
            bit,
        );
        v[59] = half_btf_avx2(
            &cospim8,
            u.as_mut_ptr().add(36),
            &cospi56,
            u.as_mut_ptr().add(59),
            &rnding,
            bit,
        );
        v[60] = half_btf_avx2(
            &cospi56,
            u.as_mut_ptr().add(35),
            &cospi8,
            u.as_mut_ptr().add(60),
            &rnding,
            bit,
        );
        v[61] = half_btf_avx2(
            &cospi56,
            u.as_mut_ptr().add(34),
            &cospi8,
            u.as_mut_ptr().add(61),
            &rnding,
            bit,
        );

        // stage 7
        addsub_avx2(v[0], v[3], u.as_mut_ptr().add(0), u.as_mut_ptr().add(3), &clamp_lo, &clamp_hi);
        addsub_avx2(v[1], v[2], u.as_mut_ptr().add(1), u.as_mut_ptr().add(2), &clamp_lo, &clamp_hi);

        u[4] = v[4];
        u[7] = v[7];
        u[5] = half_btf_avx2(
            &cospim32,
            v.as_mut_ptr().add(5),
            &cospi32,
            v.as_mut_ptr().add(6),
            &rnding,
            bit,
        );
        u[6] = half_btf_avx2(
            &cospi32,
            v.as_mut_ptr().add(5),
            &cospi32,
            v.as_mut_ptr().add(6),
            &rnding,
            bit,
        );

        addsub_avx2(
            v[8],
            v[11],
            u.as_mut_ptr().add(8),
            u.as_mut_ptr().add(11),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            v[9],
            v[10],
            u.as_mut_ptr().add(9),
            u.as_mut_ptr().add(10),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            v[15],
            v[12],
            u.as_mut_ptr().add(15),
            u.as_mut_ptr().add(12),
            &clamp_lo,
            &clamp_hi,
        );
        addsub_avx2(
            v[14],
            v[13],
            u.as_mut_ptr().add(14),
            u.as_mut_ptr().add(13),
            &clamp_lo,
            &clamp_hi,
        );

        for i in (16..32).step_by(8) {
            u[(i + 0) as usize] = v[(i + 0) as usize];
            u[(i + 1) as usize] = v[(i + 1) as usize];
            u[(i + 6) as usize] = v[(i + 6) as usize];
            u[(i + 7) as usize] = v[(i + 7) as usize];
        }

        u[18] = half_btf_avx2(
            &cospim16,
            v.as_mut_ptr().add(18),
            &cospi48,
            v.as_mut_ptr().add(29),
            &rnding,
            bit,
        );
        u[19] = half_btf_avx2(
            &cospim16,
            v.as_mut_ptr().add(19),
            &cospi48,
            v.as_mut_ptr().add(28),
            &rnding,
            bit,
        );
        u[20] = half_btf_avx2(
            &cospim48,
            v.as_mut_ptr().add(20),
            &cospim16,
            v.as_mut_ptr().add(27),
            &rnding,
            bit,
        );
        u[21] = half_btf_avx2(
            &cospim48,
            v.as_mut_ptr().add(21),
            &cospim16,
            v.as_mut_ptr().add(26),
            &rnding,
            bit,
        );
        u[26] = half_btf_avx2(
            &cospim16,
            v.as_mut_ptr().add(21),
            &cospi48,
            v.as_mut_ptr().add(26),
            &rnding,
            bit,
        );
        u[27] = half_btf_avx2(
            &cospim16,
            v.as_mut_ptr().add(20),
            &cospi48,
            v.as_mut_ptr().add(27),
            &rnding,
            bit,
        );
        u[28] = half_btf_avx2(
            &cospi48,
            v.as_mut_ptr().add(19),
            &cospi16,
            v.as_mut_ptr().add(28),
            &rnding,
            bit,
        );
        u[29] = half_btf_avx2(
            &cospi48,
            v.as_mut_ptr().add(18),
            &cospi16,
            v.as_mut_ptr().add(29),
            &rnding,
            bit,
        );

        for i in (32..64).step_by(16) {
            for j in i..i + 4 {
                addsub_avx2(
                    v[j as usize],
                    v[(j ^ 7) as usize],
                    u.as_mut_ptr().offset(j as isize),
                    u.as_mut_ptr().offset((j ^ 7) as isize),
                    &clamp_lo,
                    &clamp_hi,
                );
                addsub_avx2(
                    v[(j ^ 15) as usize],
                    v[(j ^ 8) as usize],
                    u.as_mut_ptr().offset((j ^ 15) as isize),
                    u.as_mut_ptr().offset((j ^ 8) as isize),
                    &clamp_lo,
                    &clamp_hi,
                );
            }
        }

        // stage 8
        for i in 0..4 {
            addsub_avx2(
                u[i as usize],
                u[(7 - i) as usize],
                v.as_mut_ptr().offset(i as isize),
                v.as_mut_ptr().offset((7 - i) as isize),
                &clamp_lo,
                &clamp_hi,
            );
        }

        v[8] = u[8];
        v[9] = u[9];
        v[14] = u[14];
        v[15] = u[15];

        v[10] = half_btf_avx2(
            &cospim32,
            u.as_mut_ptr().add(10),
            &cospi32,
            u.as_mut_ptr().add(13),
            &rnding,
            bit,
        );
        v[11] = half_btf_avx2(
            &cospim32,
            u.as_mut_ptr().add(11),
            &cospi32,
            u.as_mut_ptr().add(12),
            &rnding,
            bit,
        );
        v[12] = half_btf_avx2(
            &cospi32,
            u.as_mut_ptr().add(11),
            &cospi32,
            u.as_mut_ptr().add(12),
            &rnding,
            bit,
        );
        v[13] = half_btf_avx2(
            &cospi32,
            u.as_mut_ptr().add(10),
            &cospi32,
            u.as_mut_ptr().add(13),
            &rnding,
            bit,
        );

        for i in 16..20 {
            addsub_avx2(
                u[i as usize],
                u[(i ^ 7) as usize],
                v.as_mut_ptr().offset(i as isize),
                v.as_mut_ptr().offset((i ^ 7) as isize),
                &clamp_lo,
                &clamp_hi,
            );
            addsub_avx2(
                u[(i ^ 15) as usize],
                u[(i ^ 8) as usize],
                v.as_mut_ptr().offset((i ^ 15) as isize),
                v.as_mut_ptr().offset((i ^ 8) as isize),
                &clamp_lo,
                &clamp_hi,
            );
        }

        for i in 32..36 {
            v[i as usize] = u[i as usize];
            v[(i + 12) as usize] = u[(i + 12) as usize];
            v[(i + 16) as usize] = u[(i + 16) as usize];
            v[(i + 28) as usize] = u[(i + 28) as usize];
        }

        v[36] = half_btf_avx2(
            &cospim16,
            u.as_mut_ptr().add(36),
            &cospi48,
            u.as_mut_ptr().add(59),
            &rnding,
            bit,
        );
        v[37] = half_btf_avx2(
            &cospim16,
            u.as_mut_ptr().add(37),
            &cospi48,
            u.as_mut_ptr().add(58),
            &rnding,
            bit,
        );
        v[38] = half_btf_avx2(
            &cospim16,
            u.as_mut_ptr().add(38),
            &cospi48,
            u.as_mut_ptr().add(57),
            &rnding,
            bit,
        );
        v[39] = half_btf_avx2(
            &cospim16,
            u.as_mut_ptr().add(39),
            &cospi48,
            u.as_mut_ptr().add(56),
            &rnding,
            bit,
        );
        v[40] = half_btf_avx2(
            &cospim48,
            u.as_mut_ptr().add(40),
            &cospim16,
            u.as_mut_ptr().add(55),
            &rnding,
            bit,
        );
        v[41] = half_btf_avx2(
            &cospim48,
            u.as_mut_ptr().add(41),
            &cospim16,
            u.as_mut_ptr().add(54),
            &rnding,
            bit,
        );
        v[42] = half_btf_avx2(
            &cospim48,
            u.as_mut_ptr().add(42),
            &cospim16,
            u.as_mut_ptr().add(53),
            &rnding,
            bit,
        );
        v[43] = half_btf_avx2(
            &cospim48,
            u.as_mut_ptr().add(43),
            &cospim16,
            u.as_mut_ptr().add(52),
            &rnding,
            bit,
        );
        v[52] = half_btf_avx2(
            &cospim16,
            u.as_mut_ptr().add(43),
            &cospi48,
            u.as_mut_ptr().add(52),
            &rnding,
            bit,
        );
        v[53] = half_btf_avx2(
            &cospim16,
            u.as_mut_ptr().add(42),
            &cospi48,
            u.as_mut_ptr().add(53),
            &rnding,
            bit,
        );
        v[54] = half_btf_avx2(
            &cospim16,
            u.as_mut_ptr().add(41),
            &cospi48,
            u.as_mut_ptr().add(54),
            &rnding,
            bit,
        );
        v[55] = half_btf_avx2(
            &cospim16,
            u.as_mut_ptr().add(40),
            &cospi48,
            u.as_mut_ptr().add(55),
            &rnding,
            bit,
        );
        v[56] = half_btf_avx2(
            &cospi48,
            u.as_mut_ptr().add(39),
            &cospi16,
            u.as_mut_ptr().add(56),
            &rnding,
            bit,
        );
        v[57] = half_btf_avx2(
            &cospi48,
            u.as_mut_ptr().add(38),
            &cospi16,
            u.as_mut_ptr().add(57),
            &rnding,
            bit,
        );
        v[58] = half_btf_avx2(
            &cospi48,
            u.as_mut_ptr().add(37),
            &cospi16,
            u.as_mut_ptr().add(58),
            &rnding,
            bit,
        );
        v[59] = half_btf_avx2(
            &cospi48,
            u.as_mut_ptr().add(36),
            &cospi16,
            u.as_mut_ptr().add(59),
            &rnding,
            bit,
        );

        // stage 9
        for i in 0..8 {
            addsub_avx2(
                v[i as usize],
                v[(15 - i) as usize],
                u.as_mut_ptr().offset(i as isize),
                u.as_mut_ptr().offset((15 - i) as isize),
                &clamp_lo,
                &clamp_hi,
            );
        }

        for i in 16..20 {
            u[i as usize] = v[i as usize];
            u[(i + 12) as usize] = v[(i + 12) as usize];
        }

        u[20] = half_btf_avx2(
            &cospim32,
            v.as_mut_ptr().add(20),
            &cospi32,
            v.as_mut_ptr().add(27),
            &rnding,
            bit,
        );
        u[21] = half_btf_avx2(
            &cospim32,
            v.as_mut_ptr().add(21),
            &cospi32,
            v.as_mut_ptr().add(26),
            &rnding,
            bit,
        );
        u[22] = half_btf_avx2(
            &cospim32,
            v.as_mut_ptr().add(22),
            &cospi32,
            v.as_mut_ptr().add(25),
            &rnding,
            bit,
        );
        u[23] = half_btf_avx2(
            &cospim32,
            v.as_mut_ptr().add(23),
            &cospi32,
            v.as_mut_ptr().add(24),
            &rnding,
            bit,
        );
        u[24] = half_btf_avx2(
            &cospi32,
            v.as_mut_ptr().add(23),
            &cospi32,
            v.as_mut_ptr().add(24),
            &rnding,
            bit,
        );
        u[25] = half_btf_avx2(
            &cospi32,
            v.as_mut_ptr().add(22),
            &cospi32,
            v.as_mut_ptr().add(25),
            &rnding,
            bit,
        );
        u[26] = half_btf_avx2(
            &cospi32,
            v.as_mut_ptr().add(21),
            &cospi32,
            v.as_mut_ptr().add(26),
            &rnding,
            bit,
        );
        u[27] = half_btf_avx2(
            &cospi32,
            v.as_mut_ptr().add(20),
            &cospi32,
            v.as_mut_ptr().add(27),
            &rnding,
            bit,
        );

        for i in 32..40 {
            addsub_avx2(
                v[i as usize],
                v[(i ^ 15) as usize],
                u.as_mut_ptr().offset(i as isize),
                u.as_mut_ptr().offset((i ^ 15) as isize),
                &clamp_lo,
                &clamp_hi,
            );
        }

        for i in 48..56 {
            addsub_avx2(
                v[(i ^ 15) as usize],
                v[i as usize],
                u.as_mut_ptr().offset((i ^ 15) as isize),
                u.as_mut_ptr().offset(i as isize),
                &clamp_lo,
                &clamp_hi,
            );
        }

        // stage 10
        for i in 0..16 {
            addsub_avx2(
                u[i as usize],
                u[(31 - i) as usize],
                v.as_mut_ptr().offset(i as isize),
                v.as_mut_ptr().offset((31 - i) as isize),
                &clamp_lo,
                &clamp_hi,
            );
        }

        for i in 32..40 {
            v[i as usize] = u[i as usize];
        }

        v[40] = half_btf_avx2(
            &cospim32,
            u.as_mut_ptr().add(40),
            &cospi32,
            u.as_mut_ptr().add(55),
            &rnding,
            bit,
        );
        v[41] = half_btf_avx2(
            &cospim32,
            u.as_mut_ptr().add(41),
            &cospi32,
            u.as_mut_ptr().add(54),
            &rnding,
            bit,
        );
        v[42] = half_btf_avx2(
            &cospim32,
            u.as_mut_ptr().add(42),
            &cospi32,
            u.as_mut_ptr().add(53),
            &rnding,
            bit,
        );
        v[43] = half_btf_avx2(
            &cospim32,
            u.as_mut_ptr().add(43),
            &cospi32,
            u.as_mut_ptr().add(52),
            &rnding,
            bit,
        );
        v[44] = half_btf_avx2(
            &cospim32,
            u.as_mut_ptr().add(44),
            &cospi32,
            u.as_mut_ptr().add(51),
            &rnding,
            bit,
        );
        v[45] = half_btf_avx2(
            &cospim32,
            u.as_mut_ptr().add(45),
            &cospi32,
            u.as_mut_ptr().add(50),
            &rnding,
            bit,
        );
        v[46] = half_btf_avx2(
            &cospim32,
            u.as_mut_ptr().add(46),
            &cospi32,
            u.as_mut_ptr().add(49),
            &rnding,
            bit,
        );
        v[47] = half_btf_avx2(
            &cospim32,
            u.as_mut_ptr().add(47),
            &cospi32,
            u.as_mut_ptr().add(48),
            &rnding,
            bit,
        );
        v[48] = half_btf_avx2(
            &cospi32,
            u.as_mut_ptr().add(47),
            &cospi32,
            u.as_mut_ptr().add(48),
            &rnding,
            bit,
        );
        v[49] = half_btf_avx2(
            &cospi32,
            u.as_mut_ptr().add(46),
            &cospi32,
            u.as_mut_ptr().add(49),
            &rnding,
            bit,
        );
        v[50] = half_btf_avx2(
            &cospi32,
            u.as_mut_ptr().add(45),
            &cospi32,
            u.as_mut_ptr().add(50),
            &rnding,
            bit,
        );
        v[51] = half_btf_avx2(
            &cospi32,
            u.as_mut_ptr().add(44),
            &cospi32,
            u.as_mut_ptr().add(51),
            &rnding,
            bit,
        );
        v[52] = half_btf_avx2(
            &cospi32,
            u.as_mut_ptr().add(43),
            &cospi32,
            u.as_mut_ptr().add(52),
            &rnding,
            bit,
        );
        v[53] = half_btf_avx2(
            &cospi32,
            u.as_mut_ptr().add(42),
            &cospi32,
            u.as_mut_ptr().add(53),
            &rnding,
            bit,
        );
        v[54] = half_btf_avx2(
            &cospi32,
            u.as_mut_ptr().add(41),
            &cospi32,
            u.as_mut_ptr().add(54),
            &rnding,
            bit,
        );
        v[55] = half_btf_avx2(
            &cospi32,
            u.as_mut_ptr().add(40),
            &cospi32,
            u.as_mut_ptr().add(55),
            &rnding,
            bit,
        );

        for i in 56..64 {
            v[i as usize] = u[i as usize];
        }

        // stage 11
        for i in 0..32 {
            addsub_avx2(
                v[i as usize],
                v[(63 - i) as usize],
                out.offset(i as isize),
                out.offset((63 - i) as isize),
                &clamp_lo,
                &clamp_hi,
            );
        }
        if do_cols == 0 {
            let mut log_range_out: i32 = get_log_range_out(bd);
            let mut clamp_lo_out: __m256i = _mm256_set1_epi32(-(1 << (log_range_out - 1)));
            let mut clamp_hi_out: __m256i = _mm256_set1_epi32((1 << (log_range_out - 1)) - 1);

            round_shift_8x8_avx2(out, out_shift);
            round_shift_8x8_avx2(out.add(16), out_shift);
            round_shift_8x8_avx2(out.add(32), out_shift);
            round_shift_8x8_avx2(out.add(48), out_shift);
            highbd_clamp_epi32_avx2(out, out, &clamp_lo_out, &clamp_hi_out, 64);
        }
    }
}

// `typedef void (*transform_1d_avx2)(...)`; `Option` for the NULL entries.
type transform_1d_avx2 = Option<unsafe fn(*mut __m256i, *mut __m256i, i32, i32, i32, i32)>;

static highbd_txfm_all_1d_zeros_w8_arr: [[[transform_1d_avx2; 4]; 3]; 5] = [
    [[None, None, None, None], [None, None, None, None], [None, None, None, None]],
    [
        [Some(idct8x8_low1_avx2), Some(idct8x8_avx2), None, None],
        [Some(iadst8x8_low1_avx2), Some(iadst8x8_avx2), None, None],
        [None, None, None, None],
    ],
    [
        [Some(idct16_low1_avx2), Some(idct16_low8_avx2), Some(idct16_avx2), None],
        [Some(iadst16_low1_avx2), Some(iadst16_low8_avx2), Some(iadst16_avx2), None],
        [None, None, None, None],
    ],
    [
        [
            Some(idct32_low1_avx2),
            Some(idct32_low8_avx2),
            Some(idct32_low16_avx2),
            Some(idct32_avx2),
        ],
        [None, None, None, None],
        [None, None, None, None],
    ],
    [
        [
            Some(idct64_low1_avx2),
            Some(idct64_low8_avx2),
            Some(idct64_low16_avx2),
            Some(idct64_avx2),
        ],
        [None, None, None, None],
        [None, None, None, None],
    ],
];

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn highbd_inv_txfm2d_add_no_identity_avx2(
    mut input: *const i32,
    mut output: *mut u16,
    mut stride: i32,
    mut tx_type: TX_TYPE,
    mut tx_size: TX_SIZE,
    mut eob: i32,
    mut bd: i32,
) {
    let mut buf1: [__m256i; 64 * 8] = core::mem::zeroed();
    let mut eobx: i32 = 0;
    let mut eoby: i32 = 0;
    get_eobx_eoby_scan_default(&mut eobx, &mut eoby, tx_size, eob);
    let mut shift: *const i8 = av1_inv_txfm_shift_ls[tx_size as usize];
    let mut txw_idx: i32 = get_txw_idx(tx_size);
    let mut txh_idx: i32 = get_txh_idx(tx_size);
    let mut txfm_size_col: i32 = tx_size_wide[tx_size as usize];
    let mut txfm_size_row: i32 = tx_size_high[tx_size as usize];
    let mut buf_size_w_div8: i32 = txfm_size_col >> 3;
    let mut buf_size_nonzero_w: i32 = (eobx + 8) >> 3 << 3;
    let mut buf_size_nonzero_h_div8: i32 = (eoby + 8) >> 3;
    let mut input_stride: i32 = AOMMIN(32, txfm_size_row);
    let mut rect_type: i32 = get_rect_tx_log_ratio(txfm_size_col, txfm_size_row);
    let mut fun_idx_x: i32 = lowbd_txfm_all_1d_zeros_idx[eobx as usize];
    let mut fun_idx_y: i32 = lowbd_txfm_all_1d_zeros_idx[eoby as usize];
    let mut row_txfm: transform_1d_avx2 = highbd_txfm_all_1d_zeros_w8_arr[txw_idx as usize]
        [hitx_1d_tab[tx_type as usize] as usize][fun_idx_x as usize];
    let mut col_txfm: transform_1d_avx2 = highbd_txfm_all_1d_zeros_w8_arr[txh_idx as usize]
        [vitx_1d_tab[tx_type as usize] as usize][fun_idx_y as usize];

    debug_assert!(col_txfm.is_some());
    debug_assert!(row_txfm.is_some());
    let mut ud_flip: i32 = 0;
    let mut lr_flip: i32 = 0;
    get_flip_cfg(tx_type, &mut ud_flip, &mut lr_flip);

    // 1st stage: column transform
    for i in 0..buf_size_nonzero_h_div8 {
        let mut buf0: [__m256i; 64] = core::mem::zeroed();
        load_buffer_32bit_input(
            input.offset((i * 8) as isize),
            input_stride,
            buf0.as_mut_ptr(),
            buf_size_nonzero_w,
        );
        if rect_type == 1 || rect_type == -1 {
            round_shift_rect_array_32_avx2(
                buf0.as_mut_ptr(),
                buf0.as_mut_ptr(),
                buf_size_nonzero_w,
                0,
                NewInvSqrt2,
            );
        }
        row_txfm.unwrap()(
            buf0.as_mut_ptr(),
            buf0.as_mut_ptr(),
            INV_COS_BIT,
            0,
            bd,
            -(*shift.add(0) as i32),
        );

        let mut _buf1: *mut __m256i = buf1.as_mut_ptr().offset((i * 8) as isize);
        if lr_flip != 0 {
            for j in 0..buf_size_w_div8 {
                transpose_8x8_flip_avx2(
                    buf0.as_mut_ptr().offset((j * 8) as isize),
                    _buf1.offset(((buf_size_w_div8 - 1 - j) * txfm_size_row) as isize),
                );
            }
        } else {
            for j in 0..buf_size_w_div8 {
                transpose_8x8_avx2(
                    buf0.as_mut_ptr().offset((j * 8) as isize),
                    _buf1.offset((j * txfm_size_row) as isize),
                );
            }
        }
    }
    // 2nd stage: column transform
    for i in 0..buf_size_w_div8 {
        col_txfm.unwrap()(
            buf1.as_mut_ptr().offset((i * txfm_size_row) as isize),
            buf1.as_mut_ptr().offset((i * txfm_size_row) as isize),
            INV_COS_BIT,
            1,
            bd,
            0,
        );

        round_shift_array_32_avx2(
            buf1.as_mut_ptr().offset((i * txfm_size_row) as isize),
            buf1.as_mut_ptr().offset((i * txfm_size_row) as isize),
            txfm_size_row,
            -(*shift.add(1) as i32),
        );
    }

    // write to buffer
    if txfm_size_col >= 16 {
        for i in 0..(txfm_size_col >> 4) {
            highbd_write_buffer_16xn_avx2(
                buf1.as_mut_ptr().offset((i * txfm_size_row * 2) as isize),
                output.offset((16 * i) as isize),
                stride,
                ud_flip,
                txfm_size_row,
                bd,
            );
        }
    } else if txfm_size_col == 8 {
        highbd_write_buffer_8xn_avx2(buf1.as_mut_ptr(), output, stride, ud_flip, txfm_size_row, bd);
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn av1_highbd_inv_txfm2d_add_universe_avx2(
    mut input: *const i32,
    mut output: *mut u8,
    mut stride: i32,
    mut tx_type: TX_TYPE,
    mut tx_size: TX_SIZE,
    mut eob: i32,
    mut bd: i32,
) {
    match tx_type as i32 {
        DCT_DCT | ADST_DCT | DCT_ADST | ADST_ADST | FLIPADST_DCT | DCT_FLIPADST
        | FLIPADST_FLIPADST | ADST_FLIPADST | FLIPADST_ADST => {
            highbd_inv_txfm2d_add_no_identity_avx2(
                input,
                CONVERT_TO_SHORTPTR(output),
                stride,
                tx_type,
                tx_size,
                eob,
                bd,
            );
        }
        IDTX | H_DCT | H_ADST | H_FLIPADST | V_DCT | V_ADST | V_FLIPADST => {
            av1_highbd_inv_txfm2d_add_universe_sse4_1(
                input, output, stride, tx_type, tx_size, eob, bd,
            );
        }
        _ => {
            debug_assert!(false);
        }
    }
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn av1_highbd_inv_txfm_add_avx2(
    mut input: *const tran_low_t,
    mut dest: *mut u8,
    mut stride: i32,
    mut txfm_param: *const TxfmParam,
) {
    debug_assert!(
        av1_ext_tx_used[(*txfm_param).tx_set_type as usize][(*txfm_param).tx_type as usize] != 0
    );
    let mut tx_size: TX_SIZE = (*txfm_param).tx_size;
    match tx_size as i32 {
        TX_4X8 | TX_8X4 | TX_4X4 | TX_16X4 | TX_4X16 => {
            av1_highbd_inv_txfm_add_sse4_1(input, dest, stride, txfm_param);
        }
        _ => {
            av1_highbd_inv_txfm2d_add_universe_avx2(
                input,
                dest,
                stride,
                (*txfm_param).tx_type,
                (*txfm_param).tx_size,
                (*txfm_param).eob,
                (*txfm_param).bd,
            );
        }
    }
}
