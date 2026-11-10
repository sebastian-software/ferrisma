/*
 * Copyright (c) 2019, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

// Translated by hand from upstream/aom/aom_dsp/x86/highbd_adaptive_quantize_sse2.c
// (ADR 0056, ADR 0058). Upstream compiles this file with -msse2. The selected
// configuration has CONFIG_REALTIME_ONLY=0 and SKIP_EOB_FACTOR_ADJUST set.
//
// Only aom_highbd_quantize_b_64x64_adaptive_sse2() and the helpers it uses are
// translated: the 16x16 and 32x32 variants have AVX2 counterparts in
// highbd_adaptive_quantize_avx2.rs, and the kernel set takes the highest
// variant up to AVX2 (ADR 0058).
//
// log_scale is a local constant, so it becomes the const generic LOG_SCALE of
// the helpers that shift by it, and the shift argument of
// highbd_mul_shift_sse2() becomes the const generic SHIFT. Rust cannot derive
// `16 - log_scale` from a const generic, so highbd_calculate_qcoeff() takes it
// as QSHIFT, checked against LOG_SCALE at compile time.
use core::arch::x86_64::*;

use crate::simd::aom_dsp__x86__quantize_x86::*;
use crate::simd::types::*;

// aom_dsp/quantize.h and aom_dsp/aom_dsp_common.h
const EOB_FACTOR: i32 = 325;
const SKIP_EOB_FACTOR_ADJUST: i32 = 200;
const AOM_QM_BITS: i32 = 5;

// aom_ports/mem.h
macro_rules! ROUND_POWER_OF_TWO {
    ($value:expr, $n:expr) => {
        (($value) + ((1 << ($n)) >> 1)) >> ($n)
    };
}

// aom_dsp/aom_dsp_common.h
macro_rules! AOMSIGN {
    ($x:expr) => {
        ($x) >> 31
    };
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn highbd_invert_sign_64bit_sse2(mut a: __m128i, sign: __m128i) -> __m128i {
    a = _mm_xor_si128(a, sign);
    _mm_sub_epi64(a, sign)
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn highbd_mul_shift_sse2<const SHIFT: i32>(x: *const __m128i, y: *const __m128i,
                                                  p: *mut __m128i) {
    let sign = _mm_srai_epi32::<31>(*y);
    let sign_lo = _mm_unpacklo_epi32(sign, sign);
    let sign_hi = _mm_unpackhi_epi32(sign, sign);
    let abs_y = invert_sign_32_sse2(*y, sign);
    let mut prod_lo = _mm_mul_epu32(*x, abs_y);
    let mut prod_hi = _mm_srli_epi64::<32>(*x);
    let mult_hi = _mm_srli_epi64::<32>(abs_y);
    prod_hi = _mm_mul_epu32(prod_hi, mult_hi);
    prod_lo = highbd_invert_sign_64bit_sse2(prod_lo, sign_lo);
    prod_hi = highbd_invert_sign_64bit_sse2(prod_hi, sign_hi);

    prod_lo = _mm_srli_epi64::<SHIFT>(prod_lo);
    let mask = _mm_set_epi32(0, -1, 0, -1);
    prod_lo = _mm_and_si128(prod_lo, mask);
    prod_hi = _mm_srli_epi64::<SHIFT>(prod_hi);

    prod_hi = _mm_slli_epi64::<32>(prod_hi);
    *p = _mm_or_si128(prod_lo, prod_hi);
}

// QSHIFT is 16 - log_scale.
#[inline]
#[target_feature(enable = "sse2")]
unsafe fn highbd_calculate_qcoeff<const LOG_SCALE: i32, const QSHIFT: i32>(
    coeff: *mut __m128i, round: *const __m128i, quant: *const __m128i, shift: *const __m128i) {
    const { assert!(QSHIFT == 16 - LOG_SCALE) };
    let mut tmp = _mm_setzero_si128();
    let mut qcoeff: __m128i;
    qcoeff = _mm_add_epi32(*coeff, *round);
    highbd_mul_shift_sse2::<16>(&qcoeff, quant, &mut tmp);
    qcoeff = _mm_add_epi32(tmp, qcoeff);
    highbd_mul_shift_sse2::<QSHIFT>(&qcoeff, shift, coeff);
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn highbd_update_mask1(cmp_mask0: *const __m128i, iscan_ptr: *const i16,
                              is_found: *mut i32, mask: *mut __m128i) {
    let mut temp_mask = _mm_setzero_si128();
    if _mm_movemask_epi8(*cmp_mask0) != 0 {
        let iscan0 = _mm_load_si128(iscan_ptr as *const __m128i);
        let mask0 = _mm_and_si128(*cmp_mask0, iscan0);
        temp_mask = mask0;
        *is_found = 1;
    }
    *mask = _mm_max_epi16(temp_mask, *mask);
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn highbd_update_mask0(qcoeff0: *const __m128i, qcoeff1: *const __m128i,
                              threshold: &[__m128i; 2], iscan_ptr: *const i16,
                              is_found: *mut i32, mask: *mut __m128i) {
    let mut coeff = [_mm_setzero_si128(); 2];
    let mut cmp_mask0: __m128i;
    let cmp_mask1: __m128i;

    coeff[0] = _mm_slli_epi32::<AOM_QM_BITS>(*qcoeff0);
    cmp_mask0 = _mm_cmpgt_epi32(coeff[0], threshold[0]);
    coeff[1] = _mm_slli_epi32::<AOM_QM_BITS>(*qcoeff1);
    cmp_mask1 = _mm_cmpgt_epi32(coeff[1], threshold[1]);

    cmp_mask0 = _mm_packs_epi32(cmp_mask0, cmp_mask1);

    highbd_update_mask1(&cmp_mask0, iscan_ptr, is_found, mask);
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn highbd_calculate_dqcoeff<const LOG_SCALE: i32>(qcoeff: __m128i, dequant: __m128i)
                                                         -> __m128i {
    let coeff_sign = _mm_srai_epi32::<31>(qcoeff);
    let mut abs_coeff = invert_sign_32_sse2(qcoeff, coeff_sign);
    let abs_coeff_ptr: *mut __m128i = &mut abs_coeff;
    highbd_mul_shift_sse2::<LOG_SCALE>(abs_coeff_ptr, &dequant, abs_coeff_ptr);
    invert_sign_32_sse2(abs_coeff, coeff_sign)
}

#[no_mangle]
#[target_feature(enable = "sse2")]
#[allow(clippy::too_many_arguments)]
pub unsafe extern "C-unwind" fn aom_highbd_quantize_b_64x64_adaptive_sse2(
    coeff_ptr: *const tran_low_t, n_coeffs: isize, zbin_ptr: *const i16, round_ptr: *const i16,
    quant_ptr: *const i16, quant_shift_ptr: *const i16, qcoeff_ptr: *mut tran_low_t,
    dqcoeff_ptr: *mut tran_low_t, dequant_ptr: *const i16, eob_ptr: *mut u16, scan: *const i16,
    iscan: *const i16) {
    let mut index: i32 = 8;
    const LOG_SCALE: i32 = 2;
    let mut non_zero_count: i32 = 0;
    let mut non_zero_count_prescan_add_zero: i32 = 0;
    let mut is_found0: i32 = 0;
    let mut is_found1: i32 = 0;
    let mut eob: i32 = -1;
    let zero = _mm_setzero_si128();
    let one = _mm_set1_epi32(1);
    let log_scale_vec = _mm_set1_epi32(LOG_SCALE);
    let mut zbin: __m128i;
    let mut round: __m128i;
    let mut quant: __m128i;
    let mut dequant: __m128i;
    let mut shift: __m128i;
    let mut coeff0: __m128i;
    let mut coeff1: __m128i;
    let mut coeff0_sign: __m128i;
    let mut coeff1_sign: __m128i;
    let mut qcoeff0: __m128i;
    let mut qcoeff1: __m128i;
    let mut cmp_mask0: __m128i;
    let mut cmp_mask1: __m128i;
    let mut cmp_mask: __m128i;
    let mut all_zero: __m128i;
    let mut mask0 = zero;
    let mut mask1 = zero;

    let zbins: [i32; 2] = [ROUND_POWER_OF_TWO!(*zbin_ptr.add(0) as i32, LOG_SCALE),
                           ROUND_POWER_OF_TWO!(*zbin_ptr.add(1) as i32, LOG_SCALE)];
    let mut prescan_add = [0i32; 2];
    let mut thresh = [0i32; 4];
    let wt: qm_val_t = (1 << AOM_QM_BITS) as qm_val_t;
    for i in 0..2 {
        prescan_add[i] = ROUND_POWER_OF_TWO!(*dequant_ptr.add(i) as i32 * EOB_FACTOR, 7);
        thresh[i] = (zbins[i] * wt as i32 + prescan_add[i]) - 1;
    }
    thresh[3] = thresh[1];
    thresh[2] = thresh[3];
    let mut threshold = [zero; 2];
    threshold[0] = _mm_loadu_si128(thresh.as_ptr() as *const __m128i);
    threshold[1] = _mm_unpackhi_epi64(threshold[0], threshold[0]);

    let mut first: i32 = -1;
    // Setup global values.
    zbin = _mm_load_si128(zbin_ptr as *const __m128i);
    round = _mm_load_si128(round_ptr as *const __m128i);
    quant = _mm_load_si128(quant_ptr as *const __m128i);
    dequant = _mm_load_si128(dequant_ptr as *const __m128i);
    shift = _mm_load_si128(quant_shift_ptr as *const __m128i);

    let zbin_sign = _mm_srai_epi16::<15>(zbin);
    let round_sign = _mm_srai_epi16::<15>(round);
    let quant_sign = _mm_srai_epi16::<15>(quant);
    let dequant_sign = _mm_srai_epi16::<15>(dequant);
    let shift_sign = _mm_srai_epi16::<15>(shift);

    zbin = _mm_unpacklo_epi16(zbin, zbin_sign);
    round = _mm_unpacklo_epi16(round, round_sign);
    quant = _mm_unpacklo_epi16(quant, quant_sign);
    dequant = _mm_unpacklo_epi16(dequant, dequant_sign);
    shift = _mm_unpacklo_epi16(shift, shift_sign);

    // Shift with rounding.
    zbin = _mm_add_epi32(zbin, log_scale_vec);
    round = _mm_add_epi32(round, log_scale_vec);
    zbin = _mm_srli_epi32::<LOG_SCALE>(zbin);
    round = _mm_srli_epi32::<LOG_SCALE>(round);
    zbin = _mm_sub_epi32(zbin, one);

    // Do DC and first 15 AC.
    coeff0 = _mm_load_si128(coeff_ptr as *const __m128i);
    coeff1 = _mm_load_si128(coeff_ptr.add(4) as *const __m128i);

    coeff0_sign = _mm_srai_epi32::<31>(coeff0);
    coeff1_sign = _mm_srai_epi32::<31>(coeff1);
    qcoeff0 = invert_sign_32_sse2(coeff0, coeff0_sign);
    qcoeff1 = invert_sign_32_sse2(coeff1, coeff1_sign);

    highbd_update_mask0(&qcoeff0, &qcoeff1, &threshold, iscan, &mut is_found0, &mut mask0);

    cmp_mask0 = _mm_cmpgt_epi32(qcoeff0, zbin);
    zbin = _mm_unpackhi_epi64(zbin, zbin); // Switch DC to AC
    cmp_mask1 = _mm_cmpgt_epi32(qcoeff1, zbin);
    cmp_mask = _mm_packs_epi32(cmp_mask0, cmp_mask1);
    highbd_update_mask1(&cmp_mask, iscan, &mut is_found1, &mut mask1);

    threshold[0] = threshold[1];
    all_zero = _mm_or_si128(cmp_mask0, cmp_mask1);
    if _mm_movemask_epi8(all_zero) == 0 {
        _mm_store_si128(qcoeff_ptr as *mut __m128i, zero);
        _mm_store_si128(qcoeff_ptr.add(4) as *mut __m128i, zero);
        _mm_store_si128(dqcoeff_ptr as *mut __m128i, zero);
        _mm_store_si128(dqcoeff_ptr.add(4) as *mut __m128i, zero);

        round = _mm_unpackhi_epi64(round, round);
        quant = _mm_unpackhi_epi64(quant, quant);
        shift = _mm_unpackhi_epi64(shift, shift);
        dequant = _mm_unpackhi_epi64(dequant, dequant);
    } else {
        highbd_calculate_qcoeff::<LOG_SCALE, { 16 - LOG_SCALE }>(&mut qcoeff0, &round, &quant,
                                                                  &shift);

        round = _mm_unpackhi_epi64(round, round);
        quant = _mm_unpackhi_epi64(quant, quant);
        shift = _mm_unpackhi_epi64(shift, shift);
        highbd_calculate_qcoeff::<LOG_SCALE, { 16 - LOG_SCALE }>(&mut qcoeff1, &round, &quant,
                                                                  &shift);

        // Reinsert signs
        qcoeff0 = invert_sign_32_sse2(qcoeff0, coeff0_sign);
        qcoeff1 = invert_sign_32_sse2(qcoeff1, coeff1_sign);

        // Mask out zbin threshold coeffs
        qcoeff0 = _mm_and_si128(qcoeff0, cmp_mask0);
        qcoeff1 = _mm_and_si128(qcoeff1, cmp_mask1);

        _mm_store_si128(qcoeff_ptr as *mut __m128i, qcoeff0);
        _mm_store_si128(qcoeff_ptr.add(4) as *mut __m128i, qcoeff1);

        coeff0 = highbd_calculate_dqcoeff::<LOG_SCALE>(qcoeff0, dequant);
        dequant = _mm_unpackhi_epi64(dequant, dequant);
        coeff1 = highbd_calculate_dqcoeff::<LOG_SCALE>(qcoeff1, dequant);
        _mm_store_si128(dqcoeff_ptr as *mut __m128i, coeff0);
        _mm_store_si128(dqcoeff_ptr.add(4) as *mut __m128i, coeff1);
    }

    // AC only loop.
    while (index as isize) < n_coeffs {
        coeff0 = _mm_load_si128(coeff_ptr.offset(index as isize) as *const __m128i);
        coeff1 = _mm_load_si128(coeff_ptr.offset(index as isize + 4) as *const __m128i);

        coeff0_sign = _mm_srai_epi32::<31>(coeff0);
        coeff1_sign = _mm_srai_epi32::<31>(coeff1);
        qcoeff0 = invert_sign_32_sse2(coeff0, coeff0_sign);
        qcoeff1 = invert_sign_32_sse2(coeff1, coeff1_sign);

        highbd_update_mask0(&qcoeff0, &qcoeff1, &threshold, iscan.offset(index as isize),
                            &mut is_found0, &mut mask0);

        cmp_mask0 = _mm_cmpgt_epi32(qcoeff0, zbin);
        cmp_mask1 = _mm_cmpgt_epi32(qcoeff1, zbin);
        cmp_mask = _mm_packs_epi32(cmp_mask0, cmp_mask1);
        highbd_update_mask1(&cmp_mask, iscan.offset(index as isize), &mut is_found1, &mut mask1);

        all_zero = _mm_or_si128(cmp_mask0, cmp_mask1);
        if _mm_movemask_epi8(all_zero) == 0 {
            _mm_store_si128(qcoeff_ptr.offset(index as isize) as *mut __m128i, zero);
            _mm_store_si128(qcoeff_ptr.offset(index as isize + 4) as *mut __m128i, zero);
            _mm_store_si128(dqcoeff_ptr.offset(index as isize) as *mut __m128i, zero);
            _mm_store_si128(dqcoeff_ptr.offset(index as isize + 4) as *mut __m128i, zero);
            index += 8;
            continue;
        }
        highbd_calculate_qcoeff::<LOG_SCALE, { 16 - LOG_SCALE }>(&mut qcoeff0, &round, &quant,
                                                                  &shift);
        highbd_calculate_qcoeff::<LOG_SCALE, { 16 - LOG_SCALE }>(&mut qcoeff1, &round, &quant,
                                                                  &shift);

        qcoeff0 = invert_sign_32_sse2(qcoeff0, coeff0_sign);
        qcoeff1 = invert_sign_32_sse2(qcoeff1, coeff1_sign);

        qcoeff0 = _mm_and_si128(qcoeff0, cmp_mask0);
        qcoeff1 = _mm_and_si128(qcoeff1, cmp_mask1);

        _mm_store_si128(qcoeff_ptr.offset(index as isize) as *mut __m128i, qcoeff0);
        _mm_store_si128(qcoeff_ptr.offset(index as isize + 4) as *mut __m128i, qcoeff1);

        coeff0 = highbd_calculate_dqcoeff::<LOG_SCALE>(qcoeff0, dequant);
        coeff1 = highbd_calculate_dqcoeff::<LOG_SCALE>(qcoeff1, dequant);

        _mm_store_si128(dqcoeff_ptr.offset(index as isize) as *mut __m128i, coeff0);
        _mm_store_si128(dqcoeff_ptr.offset(index as isize + 4) as *mut __m128i, coeff1);

        index += 8;
    }
    if is_found0 != 0 {
        non_zero_count = calculate_non_zero_count(mask0);
    }
    if is_found1 != 0 {
        non_zero_count_prescan_add_zero = calculate_non_zero_count(mask1);
    }

    let mut i = non_zero_count_prescan_add_zero - 1;
    while i >= non_zero_count {
        let rc = *scan.offset(i as isize) as i32;
        *qcoeff_ptr.offset(rc as isize) = 0;
        *dqcoeff_ptr.offset(rc as isize) = 0;
        i -= 1;
    }

    let mut i = non_zero_count - 1;
    while i >= 0 {
        let rc = *scan.offset(i as isize) as i32;
        if *qcoeff_ptr.offset(rc as isize) != 0 {
            eob = i;
            break;
        }
        i -= 1;
    }

    *eob_ptr = (eob + 1) as u16;
    // SKIP_EOB_FACTOR_ADJUST
    // TODO(Aniket): Experiment the following loop with intrinsic by combining
    // with the quantization loop above
    for i in 0..non_zero_count {
        let rc = *scan.offset(i as isize) as i32;
        let qcoeff = *qcoeff_ptr.offset(rc as isize);
        if qcoeff != 0 {
            first = i;
            break;
        }
    }
    if (*eob_ptr as i32 - 1) >= 0 && first == (*eob_ptr as i32 - 1) {
        let rc = *scan.offset((*eob_ptr as i32 - 1) as isize) as i32;
        if *qcoeff_ptr.offset(rc as isize) == 1 || *qcoeff_ptr.offset(rc as isize) == -1 {
            let coeff: i32 = *coeff_ptr.offset(rc as isize) * wt as i32;
            let coeff_sign: i32 = AOMSIGN!(coeff);
            let abs_coeff: i32 = (coeff ^ coeff_sign) - coeff_sign;
            let factor: i32 = EOB_FACTOR + SKIP_EOB_FACTOR_ADJUST;
            let prescan_add_val: i32 =
                ROUND_POWER_OF_TWO!(*dequant_ptr.add((rc != 0) as usize) as i32 * factor, 7);
            if abs_coeff < (zbins[(rc != 0) as usize] * (1 << AOM_QM_BITS) + prescan_add_val) {
                *qcoeff_ptr.offset(rc as isize) = 0;
                *dqcoeff_ptr.offset(rc as isize) = 0;
                *eob_ptr = 0;
            }
        }
    }
}
