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

// Translated by hand from upstream/aom/aom_dsp/x86/highbd_quantize_intrin_avx2.c
// (ADR 0056, ADR 0058). Upstream compiles this file with -mavx2.
//
// log_scale is a compile-time constant at every call site, so it becomes the
// const generic LOG_SCALE. Rust cannot derive the shift immediate
// `16 - log_scale` from a const generic, so the helpers that shift by it take
// it as a second const generic SHIFT, checked against LOG_SCALE at compile time.
use core::arch::x86_64::*;

use crate::simd::types::*;

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn init_one_qp(p: *const __m128i, qp: *mut __m256i) {
    let sign = _mm_srai_epi16::<15>(*p);
    let dc = _mm_unpacklo_epi16(*p, sign);
    let ac = _mm_unpackhi_epi16(*p, sign);
    *qp = _mm256_insertf128_si256::<1>(_mm256_castsi128_si256(dc), ac);
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn update_qp(qp: &mut [__m256i; 5]) {
    let mut i = 0;
    while i < 5 {
        qp[i] = _mm256_permute2x128_si256::<0x11>(qp[i], qp[i]);
        i += 1;
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn init_qp<const LOG_SCALE: i32>(zbin_ptr: *const i16, round_ptr: *const i16,
                                        quant_ptr: *const i16, dequant_ptr: *const i16,
                                        quant_shift_ptr: *const i16, qp: &mut [__m256i; 5]) {
    let zbin = _mm_loadu_si128(zbin_ptr as *const __m128i);
    let round = _mm_loadu_si128(round_ptr as *const __m128i);
    let quant = _mm_loadu_si128(quant_ptr as *const __m128i);
    let dequant = _mm_loadu_si128(dequant_ptr as *const __m128i);
    let quant_shift = _mm_loadu_si128(quant_shift_ptr as *const __m128i);
    init_one_qp(&zbin, &mut qp[0]);
    init_one_qp(&round, &mut qp[1]);
    init_one_qp(&quant, &mut qp[2]);
    init_one_qp(&dequant, &mut qp[3]);
    init_one_qp(&quant_shift, &mut qp[4]);
    if LOG_SCALE > 0 {
        let rnd = _mm256_set1_epi32((1i32 << (LOG_SCALE - 1)) as i16 as i32);
        qp[0] = _mm256_add_epi32(qp[0], rnd);
        qp[0] = _mm256_srai_epi32::<LOG_SCALE>(qp[0]);

        qp[1] = _mm256_add_epi32(qp[1], rnd);
        qp[1] = _mm256_srai_epi32::<LOG_SCALE>(qp[1]);
    }
    // Subtracting 1 here eliminates a _mm256_cmpeq_epi32() instruction when
    // calculating the zbin mask.
    qp[0] = _mm256_sub_epi32(qp[0], _mm256_set1_epi32(1));
}

// Note:
// *x is vector multiplied by *y which is 16 int32_t parallel multiplication
// and right shift 16.  The output, 16 int32_t is save in *p.
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn mm256_mul_shift_epi32(x: *const __m256i, y: *const __m256i) -> __m256i {
    let mut prod_lo = _mm256_mul_epi32(*x, *y);
    let mut prod_hi = _mm256_srli_epi64::<32>(*x);
    let mult_hi = _mm256_srli_epi64::<32>(*y);
    prod_hi = _mm256_mul_epi32(prod_hi, mult_hi);

    prod_lo = _mm256_srli_epi64::<16>(prod_lo);
    let mask = _mm256_set_epi32(0, -1, 0, -1, 0, -1, 0, -1);
    prod_lo = _mm256_and_si256(prod_lo, mask);
    prod_hi = _mm256_srli_epi64::<16>(prod_hi);

    prod_hi = _mm256_slli_epi64::<32>(prod_hi);
    _mm256_or_si256(prod_lo, prod_hi)
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn get_max_lane_eob(iscan_ptr: *const i16, eobmax: __m256i, nz_mask: __m256i) -> __m256i {
    let packed_nz_mask = _mm256_packs_epi32(nz_mask, nz_mask);
    let packed_nz_mask_perm = _mm256_permute4x64_epi64::<0xD8>(packed_nz_mask);
    let iscan = _mm256_castsi128_si256(_mm_loadu_si128(iscan_ptr as *const __m128i));
    let iscan_plus1 = _mm256_sub_epi16(iscan, packed_nz_mask_perm);
    let nz_iscan = _mm256_and_si256(iscan_plus1, packed_nz_mask_perm);
    _mm256_max_epi16(eobmax, nz_iscan)
}

// Get the max eob from the lower 128 bits.
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn get_max_eob(mut eob: __m256i) -> u16 {
    let mut eob_s: __m256i;
    eob_s = _mm256_shuffle_epi32::<0xe>(eob);
    eob = _mm256_max_epi16(eob, eob_s);
    eob_s = _mm256_shufflelo_epi16::<0xe>(eob);
    eob = _mm256_max_epi16(eob, eob_s);
    eob_s = _mm256_shufflelo_epi16::<1>(eob);
    eob = _mm256_max_epi16(eob, eob_s);
    _mm256_extract_epi16::<0>(eob) as u16
}

// SHIFT is 16 - log_scale.
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn mm256_mul_shift_epi32_logscale<const LOG_SCALE: i32, const SHIFT: i32>(
    x: *const __m256i, y: *const __m256i) -> __m256i {
    const { assert!(SHIFT == 16 - LOG_SCALE) };
    let mut prod_lo = _mm256_mul_epi32(*x, *y);
    let mut prod_hi = _mm256_srli_epi64::<32>(*x);
    let mult_hi = _mm256_srli_epi64::<32>(*y);
    prod_hi = _mm256_mul_epi32(prod_hi, mult_hi);
    prod_lo = _mm256_srli_epi64::<SHIFT>(prod_lo);
    let mask = _mm256_set_epi32(0, -1, 0, -1, 0, -1, 0, -1);
    prod_lo = _mm256_and_si256(prod_lo, mask);
    prod_hi = _mm256_srli_epi64::<SHIFT>(prod_hi);
    prod_hi = _mm256_slli_epi64::<32>(prod_hi);
    _mm256_or_si256(prod_lo, prod_hi)
}

// SHIFT is 16 - log_scale.
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn quantize_logscale<const LOG_SCALE: i32, const SHIFT: i32>(
    qp: &[__m256i; 5], coeff_ptr: *const tran_low_t, iscan_ptr: *const i16,
    qcoeff: *mut tran_low_t, dqcoeff: *mut tran_low_t, eob: *mut __m256i) {
    let coeff = _mm256_loadu_si256(coeff_ptr as *const __m256i);
    let abs_coeff = _mm256_abs_epi32(coeff);
    let zbin_mask = _mm256_cmpgt_epi32(abs_coeff, qp[0]);

    if _mm256_movemask_epi8(zbin_mask) == 0 {
        let zero = _mm256_setzero_si256();
        _mm256_storeu_si256(qcoeff as *mut __m256i, zero);
        _mm256_storeu_si256(dqcoeff as *mut __m256i, zero);
        return;
    }

    let tmp_rnd = _mm256_and_si256(_mm256_add_epi32(abs_coeff, qp[1]), zbin_mask);
    // const int64_t tmp2 = ((tmpw * quant_ptr[rc != 0]) >> 16) + tmpw;
    let tmp = mm256_mul_shift_epi32_logscale::<0, 16>(&tmp_rnd, &qp[2]);
    let tmp2 = _mm256_add_epi32(tmp, tmp_rnd);
    // const int abs_qcoeff = (int)((tmp2 * quant_shift_ptr[rc != 0]) >>
    //                              (16 - log_scale + AOM_QM_BITS));
    let abs_q = mm256_mul_shift_epi32_logscale::<LOG_SCALE, SHIFT>(&tmp2, &qp[4]);
    let abs_dq = _mm256_srli_epi32::<LOG_SCALE>(_mm256_mullo_epi32(abs_q, qp[3]));
    let nz_mask = _mm256_cmpgt_epi32(abs_q, _mm256_setzero_si256());
    let q = _mm256_sign_epi32(abs_q, coeff);
    let dq = _mm256_sign_epi32(abs_dq, coeff);

    _mm256_storeu_si256(qcoeff as *mut __m256i, q);
    _mm256_storeu_si256(dqcoeff as *mut __m256i, dq);

    *eob = get_max_lane_eob(iscan_ptr, *eob, nz_mask);
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn quantize(qp: &[__m256i; 5], coeff_ptr: *const tran_low_t, iscan_ptr: *const i16,
                   qcoeff: *mut tran_low_t, dqcoeff: *mut tran_low_t, eob: *mut __m256i) {
    let coeff = _mm256_loadu_si256(coeff_ptr as *const __m256i);
    let abs_coeff = _mm256_abs_epi32(coeff);
    let zbin_mask = _mm256_cmpgt_epi32(abs_coeff, qp[0]);

    if _mm256_movemask_epi8(zbin_mask) == 0 {
        let zero = _mm256_setzero_si256();
        _mm256_storeu_si256(qcoeff as *mut __m256i, zero);
        _mm256_storeu_si256(dqcoeff as *mut __m256i, zero);
        return;
    }

    let tmp_rnd = _mm256_and_si256(_mm256_add_epi32(abs_coeff, qp[1]), zbin_mask);
    let tmp = mm256_mul_shift_epi32(&tmp_rnd, &qp[2]);
    let tmp2 = _mm256_add_epi32(tmp, tmp_rnd);
    let abs_q = mm256_mul_shift_epi32(&tmp2, &qp[4]);
    let abs_dq = _mm256_mullo_epi32(abs_q, qp[3]);
    let nz_mask = _mm256_cmpgt_epi32(abs_q, _mm256_setzero_si256());
    let q = _mm256_sign_epi32(abs_q, coeff);
    let dq = _mm256_sign_epi32(abs_dq, coeff);

    _mm256_storeu_si256(qcoeff as *mut __m256i, q);
    _mm256_storeu_si256(dqcoeff as *mut __m256i, dq);

    *eob = get_max_lane_eob(iscan_ptr, *eob, nz_mask);
}

#[no_mangle]
#[target_feature(enable = "avx2")]
#[allow(clippy::too_many_arguments)]
pub unsafe extern "C-unwind" fn aom_highbd_quantize_b_avx2(
    mut coeff_ptr: *const tran_low_t, mut n_coeffs: isize, zbin_ptr: *const i16,
    round_ptr: *const i16, quant_ptr: *const i16, quant_shift_ptr: *const i16,
    mut qcoeff_ptr: *mut tran_low_t, mut dqcoeff_ptr: *mut tran_low_t, dequant_ptr: *const i16,
    eob_ptr: *mut u16, _scan: *const i16, mut iscan: *const i16) {
    let step: isize = 8;

    let mut eob = _mm256_setzero_si256();
    let mut qp = [_mm256_setzero_si256(); 5];

    init_qp::<0>(zbin_ptr, round_ptr, quant_ptr, dequant_ptr, quant_shift_ptr, &mut qp);

    quantize(&qp, coeff_ptr, iscan, qcoeff_ptr, dqcoeff_ptr, &mut eob);

    coeff_ptr = coeff_ptr.offset(step);
    qcoeff_ptr = qcoeff_ptr.offset(step);
    dqcoeff_ptr = dqcoeff_ptr.offset(step);
    iscan = iscan.offset(step);
    n_coeffs -= step;

    update_qp(&mut qp);

    while n_coeffs > 0 {
        quantize(&qp, coeff_ptr, iscan, qcoeff_ptr, dqcoeff_ptr, &mut eob);

        coeff_ptr = coeff_ptr.offset(step);
        qcoeff_ptr = qcoeff_ptr.offset(step);
        dqcoeff_ptr = dqcoeff_ptr.offset(step);
        iscan = iscan.offset(step);
        n_coeffs -= step;
    }

    *eob_ptr = get_max_eob(eob);
}

#[no_mangle]
#[target_feature(enable = "avx2")]
#[allow(clippy::too_many_arguments)]
pub unsafe extern "C-unwind" fn aom_highbd_quantize_b_32x32_avx2(
    mut coeff_ptr: *const tran_low_t, mut n_coeffs: isize, zbin_ptr: *const i16,
    round_ptr: *const i16, quant_ptr: *const i16, quant_shift_ptr: *const i16,
    mut qcoeff_ptr: *mut tran_low_t, mut dqcoeff_ptr: *mut tran_low_t, dequant_ptr: *const i16,
    eob_ptr: *mut u16, _scan: *const i16, mut iscan: *const i16) {
    let step: u32 = 8;

    let mut eob = _mm256_setzero_si256();
    let mut qp = [_mm256_setzero_si256(); 5];
    init_qp::<1>(zbin_ptr, round_ptr, quant_ptr, dequant_ptr, quant_shift_ptr, &mut qp);

    quantize_logscale::<1, { 16 - 1 }>(&qp, coeff_ptr, iscan, qcoeff_ptr, dqcoeff_ptr, &mut eob);

    coeff_ptr = coeff_ptr.add(step as usize);
    qcoeff_ptr = qcoeff_ptr.add(step as usize);
    dqcoeff_ptr = dqcoeff_ptr.add(step as usize);
    iscan = iscan.add(step as usize);
    n_coeffs -= step as isize;

    update_qp(&mut qp);

    while n_coeffs > 0 {
        quantize_logscale::<1, { 16 - 1 }>(&qp, coeff_ptr, iscan, qcoeff_ptr, dqcoeff_ptr,
                                           &mut eob);

        coeff_ptr = coeff_ptr.add(step as usize);
        qcoeff_ptr = qcoeff_ptr.add(step as usize);
        dqcoeff_ptr = dqcoeff_ptr.add(step as usize);
        iscan = iscan.add(step as usize);
        n_coeffs -= step as isize;
    }

    *eob_ptr = get_max_eob(eob);
}

#[no_mangle]
#[target_feature(enable = "avx2")]
#[allow(clippy::too_many_arguments)]
pub unsafe extern "C-unwind" fn aom_highbd_quantize_b_64x64_avx2(
    mut coeff_ptr: *const tran_low_t, mut n_coeffs: isize, zbin_ptr: *const i16,
    round_ptr: *const i16, quant_ptr: *const i16, quant_shift_ptr: *const i16,
    mut qcoeff_ptr: *mut tran_low_t, mut dqcoeff_ptr: *mut tran_low_t, dequant_ptr: *const i16,
    eob_ptr: *mut u16, _scan: *const i16, mut iscan: *const i16) {
    let step: isize = 8;

    let mut eob = _mm256_setzero_si256();
    let mut qp = [_mm256_setzero_si256(); 5];
    init_qp::<2>(zbin_ptr, round_ptr, quant_ptr, dequant_ptr, quant_shift_ptr, &mut qp);

    quantize_logscale::<2, { 16 - 2 }>(&qp, coeff_ptr, iscan, qcoeff_ptr, dqcoeff_ptr, &mut eob);

    coeff_ptr = coeff_ptr.offset(step);
    qcoeff_ptr = qcoeff_ptr.offset(step);
    dqcoeff_ptr = dqcoeff_ptr.offset(step);
    iscan = iscan.offset(step);
    n_coeffs -= step;

    update_qp(&mut qp);

    while n_coeffs > 0 {
        quantize_logscale::<2, { 16 - 2 }>(&qp, coeff_ptr, iscan, qcoeff_ptr, dqcoeff_ptr,
                                           &mut eob);

        coeff_ptr = coeff_ptr.offset(step);
        qcoeff_ptr = qcoeff_ptr.offset(step);
        dqcoeff_ptr = dqcoeff_ptr.offset(step);
        iscan = iscan.offset(step);
        n_coeffs -= step;
    }

    *eob_ptr = get_max_eob(eob);
}
