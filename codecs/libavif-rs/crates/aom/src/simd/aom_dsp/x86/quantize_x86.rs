/*
 * Copyright (c) 2018, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

// Translated from upstream/aom/aom_dsp/x86/quantize_x86.h (ADR 0056, ADR
// 0058). invert_sign_32_sse2 and calculate_non_zero_count, which the
// high-bitdepth adaptive quantizers use, were translated by hand first; the
// other inline functions, which the 8-bit quantizers use, were
// transliterated with a typing-aware C-subset printer (not C2Rust) and
// reviewed against the C function by function: the same sequence of intrinsic
// and helper calls, constants and operators in order (ADR 0058). Every helper carries
// `sse2`, the smallest feature of its intrinsics.
#![allow(
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut,
    unused_parens
)]
#![allow(unused_variables, dead_code, clippy::all)]

use core::arch::x86_64::*;

// From aom_dsp/aom_dsp_common.h.
const AOM_QM_BITS: i32 = 5;

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn invert_sign_32_sse2(mut a: __m128i, sign: __m128i) -> __m128i {
    a = _mm_xor_si128(a, sign);
    _mm_sub_epi32(a, sign)
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn calculate_non_zero_count(mask: __m128i) -> i32 {
    let mut mask0: __m128i;
    let mut mask1: __m128i;
    let non_zero_count: i32;
    mask0 = _mm_unpackhi_epi64(mask, mask);
    mask1 = _mm_max_epi16(mask0, mask);
    mask0 = _mm_shuffle_epi32::<1>(mask1);
    mask0 = _mm_max_epi16(mask0, mask1);
    mask1 = _mm_srli_epi32::<16>(mask0);
    mask0 = _mm_max_epi16(mask0, mask1);
    non_zero_count = _mm_extract_epi16::<0>(mask0) + 1;

    non_zero_count
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn load_b_values(
    zbin_ptr: *const i16,
    zbin: *mut __m128i,
    round_ptr: *const i16,
    round: *mut __m128i,
    quant_ptr: *const i16,
    quant: *mut __m128i,
    dequant_ptr: *const i16,
    dequant: *mut __m128i,
    shift_ptr: *const i16,
    shift: *mut __m128i,
) {
    *zbin = _mm_load_si128(zbin_ptr as *const __m128i);
    *round = _mm_load_si128(round_ptr as *const __m128i);
    *quant = _mm_load_si128(quant_ptr as *const __m128i);
    *zbin = _mm_sub_epi16(*zbin, _mm_set1_epi16(1));
    *dequant = _mm_load_si128(dequant_ptr as *const __m128i);
    *shift = _mm_load_si128(shift_ptr as *const __m128i);
}

// With ssse3 and later abs() and sign() are preferred.
#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn invert_sign_sse2(mut a: __m128i, sign: __m128i) -> __m128i {
    a = _mm_xor_si128(a, sign);
    return _mm_sub_epi16(a, sign);
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn calculate_qcoeff(
    coeff: *mut __m128i,
    round: __m128i,
    quant: __m128i,
    shift: __m128i,
) {
    let mut tmp: __m128i;
    let mut qcoeff: __m128i;
    qcoeff = _mm_adds_epi16(*coeff, round);
    tmp = _mm_mulhi_epi16(qcoeff, quant);
    qcoeff = _mm_add_epi16(tmp, qcoeff);
    *coeff = _mm_mulhi_epi16(qcoeff, shift);
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn calculate_qcoeff_log_scale(
    coeff: *mut __m128i,
    round: __m128i,
    quant: __m128i,
    shift: *const __m128i,
    log_scale: *const i32,
) {
    let mut tmp: __m128i;
    let mut tmp1: __m128i;
    let mut qcoeff: __m128i;
    qcoeff = _mm_adds_epi16(*coeff, round);
    tmp = _mm_mulhi_epi16(qcoeff, quant);
    qcoeff = _mm_add_epi16(tmp, qcoeff);
    tmp = _mm_mullo_epi16(qcoeff, *shift);
    tmp = _mm_srl_epi16(tmp, _mm_cvtsi32_si128(16 - *log_scale));
    tmp1 = _mm_mulhi_epi16(qcoeff, *shift);
    tmp1 = _mm_sll_epi16(tmp1, _mm_cvtsi32_si128(*log_scale));
    *coeff = _mm_or_si128(tmp, tmp1);
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn calculate_dqcoeff(qcoeff: __m128i, dequant: __m128i) -> __m128i {
    return _mm_mullo_epi16(qcoeff, dequant);
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn calculate_dqcoeff_and_store_log_scale(
    qcoeff: __m128i,
    dequant: __m128i,
    zero: __m128i,
    dqcoeff: *mut i32,
    log_scale: *const i32,
) {
    // calculate abs
    let mut coeff_sign: __m128i = _mm_srai_epi16::<{ 15 }>(qcoeff);
    let mut coeff: __m128i = invert_sign_sse2(qcoeff, coeff_sign);
    let sign_0: __m128i = _mm_unpacklo_epi16(coeff_sign, zero);
    let sign_1: __m128i = _mm_unpackhi_epi16(coeff_sign, zero);
    let low: __m128i = _mm_mullo_epi16(coeff, dequant);
    let high: __m128i = _mm_mulhi_epi16(coeff, dequant);
    let mut dqcoeff32_0: __m128i = _mm_unpacklo_epi16(low, high);
    let mut dqcoeff32_1: __m128i = _mm_unpackhi_epi16(low, high);
    dqcoeff32_0 = _mm_srl_epi32(dqcoeff32_0, _mm_cvtsi32_si128(*log_scale));
    dqcoeff32_1 = _mm_srl_epi32(dqcoeff32_1, _mm_cvtsi32_si128(*log_scale));
    dqcoeff32_0 = invert_sign_32_sse2(dqcoeff32_0, sign_0);
    dqcoeff32_1 = invert_sign_32_sse2(dqcoeff32_1, sign_1);
    _mm_store_si128(dqcoeff as *mut __m128i, dqcoeff32_0);
    _mm_store_si128(dqcoeff.offset(4) as *mut __m128i, dqcoeff32_1);
}

// Scan 16 values for eob reference in scan_ptr. Use masks (-1) from comparing
// to zbin to add 1 to the index in 'scan'.
#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn scan_for_eob(
    coeff0: *mut __m128i,
    coeff1: *mut __m128i,
    zbin_mask0: __m128i,
    zbin_mask1: __m128i,
    scan_ptr: *const i16,
    index: i32,
    zero: __m128i,
) -> __m128i {
    let zero_coeff0: __m128i = _mm_cmpeq_epi16(*coeff0, zero);
    let zero_coeff1: __m128i = _mm_cmpeq_epi16(*coeff1, zero);
    let mut scan0: __m128i = _mm_load_si128(scan_ptr.offset(index as isize) as *const __m128i);
    let mut scan1: __m128i =
        _mm_load_si128(scan_ptr.offset(index as isize).offset(8) as *const __m128i);
    let mut eob0: __m128i;
    let mut eob1: __m128i;
    // Add one to convert from indices to counts
    scan0 = _mm_sub_epi16(scan0, zbin_mask0);
    scan1 = _mm_sub_epi16(scan1, zbin_mask1);
    eob0 = _mm_andnot_si128(zero_coeff0, scan0);
    eob1 = _mm_andnot_si128(zero_coeff1, scan1);
    return _mm_max_epi16(eob0, eob1);
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn accumulate_eob(mut eob: __m128i) -> i16 {
    let mut eob_shuffled: __m128i;
    eob_shuffled = _mm_shuffle_epi32::<0xe>(eob);
    eob = _mm_max_epi16(eob, eob_shuffled);
    eob_shuffled = _mm_shufflelo_epi16::<0xe>(eob);
    eob = _mm_max_epi16(eob, eob_shuffled);
    eob_shuffled = _mm_shufflelo_epi16::<0x1>(eob);
    eob = _mm_max_epi16(eob, eob_shuffled);
    return _mm_extract_epi16::<1>(eob) as i16;
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn load_coefficients(coeff_ptr: *const i32) -> __m128i {
    const _: () = assert!(core::mem::size_of::<i32>() == 4);
    let coeff1: __m128i = _mm_load_si128(coeff_ptr as *mut __m128i as *const __m128i);
    let coeff2: __m128i = _mm_load_si128(coeff_ptr.offset(4) as *mut __m128i as *const __m128i);
    return _mm_packs_epi32(coeff1, coeff2);
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn store_coefficients(coeff_vals: __m128i, coeff_ptr: *mut i32) {
    const _: () = assert!(core::mem::size_of::<i32>() == 4);
    let mut one: __m128i = _mm_set1_epi16(1);
    let mut coeff_vals_hi: __m128i = _mm_mulhi_epi16(coeff_vals, one);
    let mut coeff_vals_lo: __m128i = _mm_mullo_epi16(coeff_vals, one);
    let mut coeff_vals_1: __m128i = _mm_unpacklo_epi16(coeff_vals_lo, coeff_vals_hi);
    let mut coeff_vals_2: __m128i = _mm_unpackhi_epi16(coeff_vals_lo, coeff_vals_hi);
    _mm_store_si128(coeff_ptr as *mut __m128i, coeff_vals_1);
    _mm_store_si128(coeff_ptr.offset(4) as *mut __m128i, coeff_vals_2);
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn update_mask1(
    cmp_mask0: *mut __m128i,
    cmp_mask1: *mut __m128i,
    iscan_ptr: *const i16,
    is_found: *mut i32,
    mask: *mut __m128i,
) {
    let mut all_zero: __m128i;
    let mut temp_mask: __m128i = _mm_setzero_si128();
    all_zero = _mm_or_si128(*cmp_mask0, *cmp_mask1);
    if _mm_movemask_epi8(all_zero) != 0 {
        let mut iscan0: __m128i = _mm_load_si128(iscan_ptr as *const __m128i);
        let mut mask0: __m128i = _mm_and_si128(*cmp_mask0, iscan0);
        let mut iscan1: __m128i = _mm_load_si128(iscan_ptr.offset(8) as *const __m128i);
        let mut mask1: __m128i = _mm_and_si128(*cmp_mask1, iscan1);
        temp_mask = _mm_max_epi16(mask0, mask1);
        *is_found = 1;
    }
    *mask = _mm_max_epi16(temp_mask, *mask);
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn update_mask0(
    qcoeff0: *mut __m128i,
    qcoeff1: *mut __m128i,
    threshold: *mut __m128i,
    iscan_ptr: *const i16,
    is_found: *mut i32,
    mask: *mut __m128i,
) {
    let mut zero: __m128i = _mm_setzero_si128();
    let mut coeff: [__m128i; 4] = core::mem::zeroed();
    let mut cmp_mask0: __m128i = core::mem::zeroed();
    let mut cmp_mask1: __m128i = core::mem::zeroed();
    let mut cmp_mask2: __m128i;
    let mut cmp_mask3: __m128i;
    coeff[0] = _mm_unpacklo_epi16(*qcoeff0, zero);
    coeff[1] = _mm_unpackhi_epi16(*qcoeff0, zero);
    coeff[2] = _mm_unpacklo_epi16(*qcoeff1, zero);
    coeff[3] = _mm_unpackhi_epi16(*qcoeff1, zero);
    coeff[0] = _mm_slli_epi32::<{ AOM_QM_BITS }>(coeff[0]);
    cmp_mask0 = _mm_cmpgt_epi32(coeff[0], *threshold);
    coeff[1] = _mm_slli_epi32::<{ AOM_QM_BITS }>(coeff[1]);
    cmp_mask1 = _mm_cmpgt_epi32(coeff[1], *threshold.offset(1));
    coeff[2] = _mm_slli_epi32::<{ AOM_QM_BITS }>(coeff[2]);
    cmp_mask2 = _mm_cmpgt_epi32(coeff[2], *threshold.offset(1));
    coeff[3] = _mm_slli_epi32::<{ AOM_QM_BITS }>(coeff[3]);
    cmp_mask3 = _mm_cmpgt_epi32(coeff[3], *threshold.offset(1));
    cmp_mask0 = _mm_packs_epi32(cmp_mask0, cmp_mask1);
    cmp_mask1 = _mm_packs_epi32(cmp_mask2, cmp_mask3);
    update_mask1(
        &raw mut cmp_mask0,
        &raw mut cmp_mask1,
        iscan_ptr,
        is_found,
        mask,
    );
}
