/*
 * Copyright (c) 2017, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

// Translated from upstream/aom/av1/encoder/x86/av1_quantize_avx2.c (ADR 0056, ADR 0058).
// Upstream compiles it with -mavx2. The functions were
// transliterated with a typing-aware C-subset printer (not C2Rust) and
// reviewed against the C function by function: the same sequence of intrinsic
// and helper calls, constants and operators in order (ADR 0058).
//
// Translation notes:
// - Every function carries `avx2`, the flag upstream compiles the file with.
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
#![allow(
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut,
    unused_parens
)]
#![allow(unused_variables, dead_code, clippy::all)]

use crate::simd::aom_dsp__x86__quantize_x86::*;
use crate::simd::aom_dsp__x86__synonyms::*;
use crate::simd::aom_dsp__x86__synonyms_avx2::*;
use core::arch::x86_64::*;

// stdint.h
const INT16_MAX: i32 = 0x7fff;

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn write_zero(qcoeff: *mut i32) {
    let zero: __m256i = _mm256_setzero_si256();
    _mm256_storeu_si256(qcoeff as *mut __m256i, zero);
    _mm256_storeu_si256((qcoeff as *mut __m256i).offset(1), zero);
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn init_one_qp(p: *const __m128i, qp: *mut __m256i) {
    let ac: __m128i = _mm_unpackhi_epi64(*p, *p);
    *qp = _mm256_insertf128_si256::<1>(_mm256_castsi128_si256(*p), ac);
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn init_qp(
    round_ptr: *const i16,
    quant_ptr: *const i16,
    dequant_ptr: *const i16,
    log_scale: i32,
    thr: *mut __m256i,
    qp: *mut __m256i,
) {
    let mut round: __m128i = _mm_loadu_si128(round_ptr as *const __m128i);
    let quant: __m128i = _mm_loadu_si128(quant_ptr as *const __m128i);
    let dequant: __m128i = _mm_loadu_si128(dequant_ptr as *const __m128i);
    if log_scale > 0 {
        let rnd: __m128i = _mm_set1_epi16(((1 as i32) << (log_scale - 1)) as i16);
        round = _mm_add_epi16(round, rnd);
        round = _mm_sra_epi16(round, _mm_cvtsi32_si128(log_scale));
    }
    init_one_qp(&raw mut round as *const __m128i, qp);
    init_one_qp(&raw const quant, qp.offset(1));
    if log_scale == 1 {
        *qp.offset(1) = _mm256_sll_epi16(*qp.offset(1), _mm_cvtsi32_si128(log_scale));
    }
    init_one_qp(&raw const dequant, qp.offset(2));
    *thr = _mm256_sra_epi16(*qp.offset(2), _mm_cvtsi32_si128(1 + log_scale));
    // Subtracting 1 here eliminates a _mm256_cmpeq_epi16() instruction when
    // calculating the zbin mask.
    *thr = _mm256_sub_epi16(*thr, _mm256_set1_epi16(1));
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn update_qp(thr: *mut __m256i, qp: *mut __m256i) {
    *qp = _mm256_permute2x128_si256::<0x11>(*qp, *qp);
    *qp.offset(1) = _mm256_permute2x128_si256::<0x11>(*qp.offset(1), *qp.offset(1));
    *qp.offset(2) = _mm256_permute2x128_si256::<0x11>(*qp.offset(2), *qp.offset(2));
    *thr = _mm256_permute2x128_si256::<0x11>(*thr, *thr);
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn load_coefficients_avx2(coeff_ptr: *const i32) -> __m256i {
    let coeff1: __m256i = _mm256_load_si256(coeff_ptr as *mut __m256i as *const __m256i);
    let coeff2: __m256i = _mm256_load_si256(coeff_ptr.offset(8) as *mut __m256i as *const __m256i);
    return _mm256_packs_epi32(coeff1, coeff2);
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn store_coefficients_avx2(coeff_vals: __m256i, coeff_ptr: *mut i32) {
    let mut coeff_sign: __m256i = _mm256_srai_epi16::<{ 15 }>(coeff_vals);
    let mut coeff_vals_lo: __m256i = _mm256_unpacklo_epi16(coeff_vals, coeff_sign);
    let mut coeff_vals_hi: __m256i = _mm256_unpackhi_epi16(coeff_vals, coeff_sign);
    _mm256_store_si256(coeff_ptr as *mut __m256i, coeff_vals_lo);
    _mm256_store_si256(coeff_ptr.offset(8) as *mut __m256i, coeff_vals_hi);
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn quant_gather_eob(eob: __m256i) -> u16 {
    let eob_lo: __m128i = _mm256_castsi256_si128(eob);
    let eob_hi: __m128i = _mm256_extractf128_si256::<1>(eob);
    let mut eob_s: __m128i = _mm_max_epi16(eob_lo, eob_hi);
    eob_s = _mm_subs_epu16(_mm_set1_epi16(INT16_MAX as i16), eob_s);
    eob_s = _mm_minpos_epu16(eob_s);
    return (INT16_MAX - _mm_extract_epi16::<0>(eob_s)) as u16;
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn accumulate_eob256(eob256: __m256i) -> i16 {
    let eob_lo: __m128i = _mm256_castsi256_si128(eob256);
    let eob_hi: __m128i = _mm256_extractf128_si256::<1>(eob256);
    let mut eob: __m128i = _mm_max_epi16(eob_lo, eob_hi);
    let mut eob_shuffled: __m128i = _mm_shuffle_epi32::<0xe>(eob);
    eob = _mm_max_epi16(eob, eob_shuffled);
    eob_shuffled = _mm_shufflelo_epi16::<0xe>(eob);
    eob = _mm_max_epi16(eob, eob_shuffled);
    eob_shuffled = _mm_shufflelo_epi16::<0x1>(eob);
    eob = _mm_max_epi16(eob, eob_shuffled);
    return _mm_extract_epi16::<1>(eob) as i16;
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn quantize_lp_16_first(
    coeff_ptr: *const i16,
    iscan_ptr: *const i16,
    qcoeff_ptr: *mut i16,
    dqcoeff_ptr: *mut i16,
    round256: *mut __m256i,
    quant256: *mut __m256i,
    dequant256: *mut __m256i,
    eob: *mut __m256i,
) {
    let coeff: __m256i = _mm256_loadu_si256(coeff_ptr as *const __m256i);
    let abs_coeff: __m256i = _mm256_abs_epi16(coeff);
    let tmp_rnd: __m256i = _mm256_adds_epi16(abs_coeff, *round256);
    let abs_qcoeff: __m256i = _mm256_mulhi_epi16(tmp_rnd, *quant256);
    let qcoeff: __m256i = _mm256_sign_epi16(abs_qcoeff, coeff);
    let dqcoeff: __m256i = _mm256_mullo_epi16(qcoeff, *dequant256);
    let nz_mask: __m256i = _mm256_cmpgt_epi16(abs_qcoeff, _mm256_setzero_si256());
    _mm256_storeu_si256(qcoeff_ptr as *mut __m256i, qcoeff);
    _mm256_storeu_si256(dqcoeff_ptr as *mut __m256i, dqcoeff);
    let iscan: __m256i = _mm256_loadu_si256(iscan_ptr as *const __m256i);
    let iscan_plus1: __m256i = _mm256_sub_epi16(iscan, nz_mask);
    let nz_iscan: __m256i = _mm256_and_si256(iscan_plus1, nz_mask);
    *eob = _mm256_max_epi16(*eob, nz_iscan);
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn quantize_lp_16(
    coeff_ptr: *const i16,
    n_coeffs: isize,
    iscan_ptr: *const i16,
    qcoeff_ptr: *mut i16,
    dqcoeff_ptr: *mut i16,
    round256: *mut __m256i,
    quant256: *mut __m256i,
    dequant256: *mut __m256i,
    eob: *mut __m256i,
) {
    let coeff: __m256i = _mm256_loadu_si256(coeff_ptr.offset(n_coeffs) as *const __m256i);
    let abs_coeff: __m256i = _mm256_abs_epi16(coeff);
    let tmp_rnd: __m256i = _mm256_adds_epi16(abs_coeff, *round256);
    let abs_qcoeff: __m256i = _mm256_mulhi_epi16(tmp_rnd, *quant256);
    let qcoeff: __m256i = _mm256_sign_epi16(abs_qcoeff, coeff);
    let dqcoeff: __m256i = _mm256_mullo_epi16(qcoeff, *dequant256);
    let nz_mask: __m256i = _mm256_cmpgt_epi16(abs_qcoeff, _mm256_setzero_si256());
    _mm256_storeu_si256(qcoeff_ptr.offset(n_coeffs) as *mut __m256i, qcoeff);
    _mm256_storeu_si256(dqcoeff_ptr.offset(n_coeffs) as *mut __m256i, dqcoeff);
    let iscan: __m256i = _mm256_loadu_si256(iscan_ptr.offset(n_coeffs) as *const __m256i);
    let iscan_plus1: __m256i = _mm256_sub_epi16(iscan, nz_mask);
    let nz_iscan: __m256i = _mm256_and_si256(iscan_plus1, nz_mask);
    *eob = _mm256_max_epi16(*eob, nz_iscan);
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn av1_quantize_lp_avx2(
    coeff_ptr: *const i16,
    n_coeffs: isize,
    round_ptr: *const i16,
    quant_ptr: *const i16,
    qcoeff_ptr: *mut i16,
    dqcoeff_ptr: *mut i16,
    dequant_ptr: *const i16,
    eob_ptr: *mut u16,
    scan: *const i16,
    iscan: *const i16,
) {
    let _ = scan;
    let mut eob256: __m256i = _mm256_setzero_si256();
    // Setup global values.
    let mut round256: __m256i = _mm256_castsi128_si256(_mm_load_si128(round_ptr as *const __m128i));
    let mut quant256: __m256i = _mm256_castsi128_si256(_mm_load_si128(quant_ptr as *const __m128i));
    let mut dequant256: __m256i =
        _mm256_castsi128_si256(_mm_load_si128(dequant_ptr as *const __m128i));
    // Populate upper AC values.
    round256 = _mm256_permute4x64_epi64::<0x54>(round256);
    quant256 = _mm256_permute4x64_epi64::<0x54>(quant256);
    dequant256 = _mm256_permute4x64_epi64::<0x54>(dequant256);
    // Process DC and the first 15 AC coeffs.
    quantize_lp_16_first(
        coeff_ptr,
        iscan,
        qcoeff_ptr,
        dqcoeff_ptr,
        &raw mut round256,
        &raw mut quant256,
        &raw mut dequant256,
        &raw mut eob256,
    );
    if n_coeffs > 16 {
        // Overwrite the DC constants with AC constants
        dequant256 = _mm256_permute2x128_si256::<0x31>(dequant256, dequant256);
        quant256 = _mm256_permute2x128_si256::<0x31>(quant256, quant256);
        round256 = _mm256_permute2x128_si256::<0x31>(round256, round256);
        // AC only loop.
        for idx in (16..n_coeffs as i32).step_by(16) {
            quantize_lp_16(
                coeff_ptr,
                idx as isize,
                iscan,
                qcoeff_ptr,
                dqcoeff_ptr,
                &raw mut round256,
                &raw mut quant256,
                &raw mut dequant256,
                &raw mut eob256,
            );
        }
    }
    *eob_ptr = accumulate_eob256(eob256) as u16;
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn get_max_lane_eob(iscan: *const i16, v_eobmax: __m256i, v_mask: __m256i) -> __m256i {
    let v_iscan: __m256i = _mm256_loadu_si256(iscan as *const __m256i);
    let v_iscan_perm: __m256i = _mm256_permute4x64_epi64::<0xD8>(v_iscan);
    let v_iscan_plus1: __m256i = _mm256_sub_epi16(v_iscan_perm, v_mask);
    let v_nz_iscan: __m256i = _mm256_and_si256(v_iscan_plus1, v_mask);
    return _mm256_max_epi16(v_eobmax, v_nz_iscan);
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn quantize_fp_16(
    thr: *const __m256i,
    qp: *const __m256i,
    coeff_ptr: *const i32,
    iscan_ptr: *const i16,
    qcoeff_ptr: *mut i32,
    dqcoeff_ptr: *mut i32,
    eob: *mut __m256i,
) {
    let coeff: __m256i = load_coefficients_avx2(coeff_ptr);
    let abs_coeff: __m256i = _mm256_abs_epi16(coeff);
    let mask: __m256i = _mm256_cmpgt_epi16(abs_coeff, *thr);
    let nzflag: i32 = _mm256_movemask_epi8(mask);
    if nzflag != 0 {
        let tmp_rnd: __m256i = _mm256_adds_epi16(abs_coeff, *qp);
        let abs_q: __m256i = _mm256_mulhi_epi16(tmp_rnd, *qp.offset(1));
        let q: __m256i = _mm256_sign_epi16(abs_q, coeff);
        let dq: __m256i = _mm256_mullo_epi16(q, *qp.offset(2));
        let nz_mask: __m256i = _mm256_cmpgt_epi16(abs_q, _mm256_setzero_si256());
        store_coefficients_avx2(q, qcoeff_ptr);
        store_coefficients_avx2(dq, dqcoeff_ptr);
        *eob = get_max_lane_eob(iscan_ptr, *eob, nz_mask);
    } else {
        write_zero(qcoeff_ptr);
        write_zero(dqcoeff_ptr);
    }
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn av1_quantize_fp_avx2(
    mut coeff_ptr: *const i32,
    mut n_coeffs: isize,
    zbin_ptr: *const i16,
    round_ptr: *const i16,
    quant_ptr: *const i16,
    quant_shift_ptr: *const i16,
    mut qcoeff_ptr: *mut i32,
    mut dqcoeff_ptr: *mut i32,
    dequant_ptr: *const i16,
    eob_ptr: *mut u16,
    scan_ptr: *const i16,
    mut iscan_ptr: *const i16,
) {
    let _ = scan_ptr;
    let _ = zbin_ptr;
    let _ = quant_shift_ptr;
    let log_scale: i32 = 0;
    let step: i32 = 16;
    let mut qp: [__m256i; 3] = core::mem::zeroed();
    let mut thr: __m256i = core::mem::zeroed();
    let mut eob: __m256i = _mm256_setzero_si256();
    init_qp(
        round_ptr,
        quant_ptr,
        dequant_ptr,
        log_scale,
        &raw mut thr,
        qp.as_mut_ptr(),
    );
    quantize_fp_16(
        &raw mut thr as *const __m256i,
        qp.as_mut_ptr() as *const __m256i,
        coeff_ptr,
        iscan_ptr,
        qcoeff_ptr,
        dqcoeff_ptr,
        &raw mut eob,
    );
    coeff_ptr = coeff_ptr.offset(step as isize);
    qcoeff_ptr = qcoeff_ptr.offset(step as isize);
    dqcoeff_ptr = dqcoeff_ptr.offset(step as isize);
    iscan_ptr = iscan_ptr.offset(step as isize);
    n_coeffs -= step as isize;
    update_qp(&raw mut thr, qp.as_mut_ptr());
    while n_coeffs > 0 {
        quantize_fp_16(
            &raw mut thr as *const __m256i,
            qp.as_mut_ptr() as *const __m256i,
            coeff_ptr,
            iscan_ptr,
            qcoeff_ptr,
            dqcoeff_ptr,
            &raw mut eob,
        );
        coeff_ptr = coeff_ptr.offset(step as isize);
        qcoeff_ptr = qcoeff_ptr.offset(step as isize);
        dqcoeff_ptr = dqcoeff_ptr.offset(step as isize);
        iscan_ptr = iscan_ptr.offset(step as isize);
        n_coeffs -= step as isize;
    }
    *eob_ptr = quant_gather_eob(eob);
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn quantize_fp_32x32(
    thr: *const __m256i,
    qp: *const __m256i,
    coeff_ptr: *const i32,
    iscan_ptr: *const i16,
    qcoeff_ptr: *mut i32,
    dqcoeff_ptr: *mut i32,
    eob: *mut __m256i,
) {
    let coeff: __m256i = load_coefficients_avx2(coeff_ptr);
    let abs_coeff: __m256i = _mm256_abs_epi16(coeff);
    let mask: __m256i = _mm256_cmpgt_epi16(abs_coeff, *thr);
    let nzflag: i32 = _mm256_movemask_epi8(mask);
    if nzflag != 0 {
        let tmp_rnd: __m256i = _mm256_adds_epi16(abs_coeff, *qp);
        let abs_q: __m256i = _mm256_mulhi_epu16(tmp_rnd, *qp.offset(1));
        let q: __m256i = _mm256_sign_epi16(abs_q, coeff);
        let abs_dq: __m256i = _mm256_srli_epi16::<{ 1 }>(_mm256_mullo_epi16(abs_q, *qp.offset(2)));
        let nz_mask: __m256i = _mm256_cmpgt_epi16(abs_q, _mm256_setzero_si256());
        let dq: __m256i = _mm256_sign_epi16(abs_dq, coeff);
        store_coefficients_avx2(q, qcoeff_ptr);
        store_coefficients_avx2(dq, dqcoeff_ptr);
        *eob = get_max_lane_eob(iscan_ptr, *eob, nz_mask);
    } else {
        write_zero(qcoeff_ptr);
        write_zero(dqcoeff_ptr);
    }
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn av1_quantize_fp_32x32_avx2(
    mut coeff_ptr: *const i32,
    mut n_coeffs: isize,
    zbin_ptr: *const i16,
    round_ptr: *const i16,
    quant_ptr: *const i16,
    quant_shift_ptr: *const i16,
    mut qcoeff_ptr: *mut i32,
    mut dqcoeff_ptr: *mut i32,
    dequant_ptr: *const i16,
    eob_ptr: *mut u16,
    scan_ptr: *const i16,
    mut iscan_ptr: *const i16,
) {
    let _ = scan_ptr;
    let _ = zbin_ptr;
    let _ = quant_shift_ptr;
    let log_scale: i32 = 1;
    let step: u32 = 16;
    let mut qp: [__m256i; 3] = core::mem::zeroed();
    let mut thr: __m256i = core::mem::zeroed();
    let mut eob: __m256i = _mm256_setzero_si256();
    init_qp(
        round_ptr,
        quant_ptr,
        dequant_ptr,
        log_scale,
        &raw mut thr,
        qp.as_mut_ptr(),
    );
    quantize_fp_32x32(
        &raw mut thr as *const __m256i,
        qp.as_mut_ptr() as *const __m256i,
        coeff_ptr,
        iscan_ptr,
        qcoeff_ptr,
        dqcoeff_ptr,
        &raw mut eob,
    );
    coeff_ptr = coeff_ptr.offset(step as isize);
    qcoeff_ptr = qcoeff_ptr.offset(step as isize);
    dqcoeff_ptr = dqcoeff_ptr.offset(step as isize);
    iscan_ptr = iscan_ptr.offset(step as isize);
    n_coeffs -= step as isize;
    update_qp(&raw mut thr, qp.as_mut_ptr());
    while n_coeffs > 0 {
        quantize_fp_32x32(
            &raw mut thr as *const __m256i,
            qp.as_mut_ptr() as *const __m256i,
            coeff_ptr,
            iscan_ptr,
            qcoeff_ptr,
            dqcoeff_ptr,
            &raw mut eob,
        );
        coeff_ptr = coeff_ptr.offset(step as isize);
        qcoeff_ptr = qcoeff_ptr.offset(step as isize);
        dqcoeff_ptr = dqcoeff_ptr.offset(step as isize);
        iscan_ptr = iscan_ptr.offset(step as isize);
        n_coeffs -= step as isize;
    }
    *eob_ptr = quant_gather_eob(eob);
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn quantize_fp_64x64(
    thr: *const __m256i,
    qp: *const __m256i,
    coeff_ptr: *const i32,
    iscan_ptr: *const i16,
    qcoeff_ptr: *mut i32,
    dqcoeff_ptr: *mut i32,
    eob: *mut __m256i,
) {
    let coeff: __m256i = load_coefficients_avx2(coeff_ptr);
    let abs_coeff: __m256i = _mm256_abs_epi16(coeff);
    let mask: __m256i = _mm256_cmpgt_epi16(abs_coeff, *thr);
    let nzflag: i32 = _mm256_movemask_epi8(mask);
    if nzflag != 0 {
        let tmp_rnd: __m256i = _mm256_and_si256(_mm256_adds_epi16(abs_coeff, *qp), mask);
        let qh: __m256i = _mm256_slli_epi16::<{ 2 }>(_mm256_mulhi_epi16(tmp_rnd, *qp.offset(1)));
        let ql: __m256i = _mm256_srli_epi16::<{ 14 }>(_mm256_mullo_epi16(tmp_rnd, *qp.offset(1)));
        let abs_q: __m256i = _mm256_or_si256(qh, ql);
        let dqh: __m256i = _mm256_slli_epi16::<{ 14 }>(_mm256_mulhi_epi16(abs_q, *qp.offset(2)));
        let dql: __m256i = _mm256_srli_epi16::<{ 2 }>(_mm256_mullo_epi16(abs_q, *qp.offset(2)));
        let abs_dq: __m256i = _mm256_or_si256(dqh, dql);
        let q: __m256i = _mm256_sign_epi16(abs_q, coeff);
        let dq: __m256i = _mm256_sign_epi16(abs_dq, coeff);
        // Check the signed q/dq value here instead of the absolute value. When
        // dequant equals 4, the dequant threshold (*thr) becomes 0 after being
        // scaled down by (1 + log_scale). See init_qp(). When *thr is 0 and the
        // abs_coeff is 0, the nzflag will be set. As a result, the eob will be
        // incorrectly calculated. The psign instruction corrects the error by
        // zeroing out q/dq if coeff is zero.
        let z_mask: __m256i = _mm256_cmpeq_epi16(dq, _mm256_setzero_si256());
        let nz_mask: __m256i = _mm256_cmpeq_epi16(z_mask, _mm256_setzero_si256());
        store_coefficients_avx2(q, qcoeff_ptr);
        store_coefficients_avx2(dq, dqcoeff_ptr);
        *eob = get_max_lane_eob(iscan_ptr, *eob, nz_mask);
    } else {
        write_zero(qcoeff_ptr);
        write_zero(dqcoeff_ptr);
    }
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn av1_quantize_fp_64x64_avx2(
    mut coeff_ptr: *const i32,
    mut n_coeffs: isize,
    zbin_ptr: *const i16,
    round_ptr: *const i16,
    quant_ptr: *const i16,
    quant_shift_ptr: *const i16,
    mut qcoeff_ptr: *mut i32,
    mut dqcoeff_ptr: *mut i32,
    dequant_ptr: *const i16,
    eob_ptr: *mut u16,
    scan_ptr: *const i16,
    mut iscan_ptr: *const i16,
) {
    let _ = scan_ptr;
    let _ = zbin_ptr;
    let _ = quant_shift_ptr;
    let log_scale: i32 = 2;
    let step: u32 = 16;
    let mut qp: [__m256i; 3] = core::mem::zeroed();
    let mut thr: __m256i = core::mem::zeroed();
    let mut eob: __m256i = _mm256_setzero_si256();
    init_qp(
        round_ptr,
        quant_ptr,
        dequant_ptr,
        log_scale,
        &raw mut thr,
        qp.as_mut_ptr(),
    );
    quantize_fp_64x64(
        &raw mut thr as *const __m256i,
        qp.as_mut_ptr() as *const __m256i,
        coeff_ptr,
        iscan_ptr,
        qcoeff_ptr,
        dqcoeff_ptr,
        &raw mut eob,
    );
    coeff_ptr = coeff_ptr.offset(step as isize);
    qcoeff_ptr = qcoeff_ptr.offset(step as isize);
    dqcoeff_ptr = dqcoeff_ptr.offset(step as isize);
    iscan_ptr = iscan_ptr.offset(step as isize);
    n_coeffs -= step as isize;
    update_qp(&raw mut thr, qp.as_mut_ptr());
    while n_coeffs > 0 {
        quantize_fp_64x64(
            &raw mut thr as *const __m256i,
            qp.as_mut_ptr() as *const __m256i,
            coeff_ptr,
            iscan_ptr,
            qcoeff_ptr,
            dqcoeff_ptr,
            &raw mut eob,
        );
        coeff_ptr = coeff_ptr.offset(step as isize);
        qcoeff_ptr = qcoeff_ptr.offset(step as isize);
        dqcoeff_ptr = dqcoeff_ptr.offset(step as isize);
        iscan_ptr = iscan_ptr.offset(step as isize);
        n_coeffs -= step as isize;
    }
    *eob_ptr = quant_gather_eob(eob);
}
