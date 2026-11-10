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

// Translated by hand from upstream/aom/av1/encoder/x86/av1_highbd_quantize_avx2.c
// (ADR 0056, ADR 0058). Upstream compiles this file with -mavx2.
//
// Here log_scale is a run-time argument of the exported function, so the C
// shifts by `16 - log_scale`, `1 + log_scale` and `log_scale` are not
// immediates. A C compiler lowers such a _mm256_s{rl,ll,ra}i_epi* call with a
// non-constant count to the shift-by-register instruction with the count
// zero-extended into the low quadword of an XMM register; the translation
// writes that form directly as _mm256_s{rl,ll,ra}_epi* with _mm_cvtsi32_si128.
use core::arch::x86_64::*;

use crate::simd::types::*;

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn init_one_qp(p: *const __m128i, qp: *mut __m256i) {
    let zero = _mm_setzero_si128();
    let dc = _mm_unpacklo_epi16(*p, zero);
    let ac = _mm_unpackhi_epi16(*p, zero);
    *qp = _mm256_insertf128_si256::<1>(_mm256_castsi128_si256(dc), ac);
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn update_qp(qp: &mut [__m256i; 3]) {
    qp[0] = _mm256_permute2x128_si256::<0x11>(qp[0], qp[0]);
    qp[1] = _mm256_permute2x128_si256::<0x11>(qp[1], qp[1]);
    qp[2] = _mm256_permute2x128_si256::<0x11>(qp[2], qp[2]);
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn init_qp(round_ptr: *const i16, quant_ptr: *const i16, dequant_ptr: *const i16,
                  log_scale: i32, qp: &mut [__m256i; 3]) {
    let mut round = _mm_loadu_si128(round_ptr as *const __m128i);
    if log_scale != 0 {
        let round_scale = _mm_set1_epi16((1i32 << (15 - log_scale)) as i16);
        round = _mm_mulhrs_epi16(round, round_scale);
    }
    let quant = _mm_loadu_si128(quant_ptr as *const __m128i);
    let dequant = _mm_loadu_si128(dequant_ptr as *const __m128i);

    init_one_qp(&round, &mut qp[0]);
    init_one_qp(&quant, &mut qp[1]);
    init_one_qp(&dequant, &mut qp[2]);
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn quantize(qp: &[__m256i; 3], c: *const __m256i, iscan_ptr: *const i16, log_scale: i32,
                   qcoeff: *mut tran_low_t, dqcoeff: *mut tran_low_t, eob: *mut __m256i) {
    let abs_coeff = _mm256_abs_epi32(*c);
    let mut q = _mm256_add_epi32(abs_coeff, qp[0]);

    let mut q_lo = _mm256_mul_epi32(q, qp[1]);
    let mut q_hi = _mm256_srli_epi64::<32>(q);
    let qp_hi = _mm256_srli_epi64::<32>(qp[1]);
    q_hi = _mm256_mul_epi32(q_hi, qp_hi);
    q_lo = _mm256_srl_epi64(q_lo, _mm_cvtsi32_si128(16 - log_scale));
    q_hi = _mm256_srl_epi64(q_hi, _mm_cvtsi32_si128(16 - log_scale));
    q_hi = _mm256_slli_epi64::<32>(q_hi);
    q = _mm256_or_si256(q_lo, q_hi);
    let abs_s = _mm256_sll_epi32(abs_coeff, _mm_cvtsi32_si128(1 + log_scale));
    let mask = _mm256_cmpgt_epi32(qp[2], abs_s);
    q = _mm256_andnot_si256(mask, q);

    let mut dq = _mm256_mullo_epi32(q, qp[2]);
    dq = _mm256_sra_epi32(dq, _mm_cvtsi32_si128(log_scale));
    q = _mm256_sign_epi32(q, *c);
    dq = _mm256_sign_epi32(dq, *c);

    _mm256_storeu_si256(qcoeff as *mut __m256i, q);
    _mm256_storeu_si256(dqcoeff as *mut __m256i, dq);

    let isc = _mm_loadu_si128(iscan_ptr as *const __m128i);
    let zr = _mm_setzero_si128();
    let lo = _mm_unpacklo_epi16(isc, zr);
    let hi = _mm_unpackhi_epi16(isc, zr);
    let iscan = _mm256_insertf128_si256::<1>(_mm256_castsi128_si256(lo), hi);

    let zero = _mm256_setzero_si256();
    let zc = _mm256_cmpeq_epi32(dq, zero);
    let nz = _mm256_cmpeq_epi32(zc, zero);
    let mut cur_eob = _mm256_sub_epi32(iscan, nz);
    cur_eob = _mm256_and_si256(cur_eob, nz);
    *eob = _mm256_max_epi32(cur_eob, *eob);
}

#[no_mangle]
#[target_feature(enable = "avx2")]
#[allow(clippy::too_many_arguments)]
pub unsafe extern "C-unwind" fn av1_highbd_quantize_fp_avx2(
    mut coeff_ptr: *const tran_low_t, mut n_coeffs: isize, _zbin_ptr: *const i16,
    round_ptr: *const i16, quant_ptr: *const i16, _quant_shift_ptr: *const i16,
    mut qcoeff_ptr: *mut tran_low_t, mut dqcoeff_ptr: *mut tran_low_t, dequant_ptr: *const i16,
    eob_ptr: *mut u16, _scan: *const i16, mut iscan: *const i16, log_scale: i32) {
    let step: u32 = 8;
    let mut qp = [_mm256_setzero_si256(); 3];
    let mut coeff: __m256i;

    init_qp(round_ptr, quant_ptr, dequant_ptr, log_scale, &mut qp);
    coeff = _mm256_loadu_si256(coeff_ptr as *const __m256i);

    let mut eob = _mm256_setzero_si256();
    quantize(&qp, &coeff, iscan, log_scale, qcoeff_ptr, dqcoeff_ptr, &mut eob);

    coeff_ptr = coeff_ptr.add(step as usize);
    qcoeff_ptr = qcoeff_ptr.add(step as usize);
    dqcoeff_ptr = dqcoeff_ptr.add(step as usize);
    iscan = iscan.add(step as usize);
    n_coeffs -= step as isize;

    update_qp(&mut qp);
    while n_coeffs > 0 {
        coeff = _mm256_loadu_si256(coeff_ptr as *const __m256i);
        quantize(&qp, &coeff, iscan, log_scale, qcoeff_ptr, dqcoeff_ptr, &mut eob);

        coeff_ptr = coeff_ptr.add(step as usize);
        qcoeff_ptr = qcoeff_ptr.add(step as usize);
        dqcoeff_ptr = dqcoeff_ptr.add(step as usize);
        iscan = iscan.add(step as usize);
        n_coeffs -= step as isize;
    }
    {
        let mut eob_s: __m256i;
        eob_s = _mm256_shuffle_epi32::<0xe>(eob);
        eob = _mm256_max_epi16(eob, eob_s);
        eob_s = _mm256_shufflelo_epi16::<0xe>(eob);
        eob = _mm256_max_epi16(eob, eob_s);
        eob_s = _mm256_shufflelo_epi16::<1>(eob);
        eob = _mm256_max_epi16(eob, eob_s);
        let final_eob = _mm_max_epi16(_mm256_castsi256_si128(eob),
                                      _mm256_extractf128_si256::<1>(eob));
        *eob_ptr = _mm_extract_epi16::<0>(final_eob) as u16;
    }
}
