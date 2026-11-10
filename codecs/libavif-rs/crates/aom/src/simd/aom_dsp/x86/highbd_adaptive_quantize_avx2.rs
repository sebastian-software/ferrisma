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

// Translated by hand from upstream/aom/aom_dsp/x86/highbd_adaptive_quantize_avx2.c
// (ADR 0056, ADR 0058). Upstream compiles this file with -mavx2. The selected
// configuration has CONFIG_REALTIME_ONLY=0 and SKIP_EOB_FACTOR_ADJUST set.
//
// log_scale is a local constant of each exported function, so it becomes the
// const generic LOG_SCALE of the helpers that shift by it, and the shift
// argument of highbd_mul_shift_avx2() becomes the const generic SHIFT. Rust
// cannot derive `16 - log_scale` from a const generic, so
// highbd_calculate_qcoeff_avx2() takes it as QSHIFT, checked against
// LOG_SCALE at compile time.
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
#[target_feature(enable = "avx2")]
#[allow(clippy::too_many_arguments)]
unsafe fn highbd_load_b_values_avx2(zbin_ptr: *const i16, zbin: *mut __m256i,
                                    round_ptr: *const i16, round: *mut __m256i,
                                    quant_ptr: *const i16, quant: *mut __m256i,
                                    dequant_ptr: *const i16, dequant: *mut __m256i,
                                    shift_ptr: *const i16, shift: *mut __m256i) {
    *zbin = _mm256_cvtepi16_epi32(_mm_load_si128(zbin_ptr as *const __m128i));
    *zbin = _mm256_sub_epi32(*zbin, _mm256_set1_epi32(1));
    *round = _mm256_cvtepi16_epi32(_mm_load_si128(round_ptr as *const __m128i));
    *quant = _mm256_cvtepi16_epi32(_mm_load_si128(quant_ptr as *const __m128i));
    *dequant = _mm256_cvtepi16_epi32(_mm_load_si128(dequant_ptr as *const __m128i));
    *shift = _mm256_cvtepi16_epi32(_mm_load_si128(shift_ptr as *const __m128i));
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn highbd_update_mask1_avx2(cmp_mask: *const __m256i, iscan_ptr: *const i16,
                                   is_found: *mut i32, mask: *mut __m256i) {
    let mut temp_mask = _mm256_setzero_si256();
    if _mm256_movemask_epi8(*cmp_mask) != 0 {
        let iscan = _mm256_loadu_si256(iscan_ptr as *const __m256i);
        temp_mask = _mm256_and_si256(*cmp_mask, iscan);
        *is_found = 1;
    }
    *mask = _mm256_max_epi16(temp_mask, *mask);
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn highbd_update_mask0_avx2(qcoeff0: *const __m256i, qcoeff1: *const __m256i,
                                   threshold: &[__m256i; 2], iscan_ptr: *const i16,
                                   is_found: *mut i32, mask: *mut __m256i) {
    let mut coeff = [_mm256_setzero_si256(); 2];
    let mut cmp_mask0: __m256i;
    let cmp_mask1: __m256i;
    coeff[0] = _mm256_slli_epi32::<AOM_QM_BITS>(*qcoeff0);
    cmp_mask0 = _mm256_cmpgt_epi32(coeff[0], threshold[0]);
    coeff[1] = _mm256_slli_epi32::<AOM_QM_BITS>(*qcoeff1);
    cmp_mask1 = _mm256_cmpgt_epi32(coeff[1], threshold[1]);
    cmp_mask0 = _mm256_permute4x64_epi64::<0xd8>(_mm256_packs_epi32(cmp_mask0, cmp_mask1));
    highbd_update_mask1_avx2(&cmp_mask0, iscan_ptr, is_found, mask);
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn highbd_mul_shift_avx2<const SHIFT: i32>(x: *const __m256i, y: *const __m256i,
                                                  p: *mut __m256i) {
    let mut prod_lo = _mm256_mul_epi32(*x, *y);
    let mut prod_hi = _mm256_srli_epi64::<32>(*x);
    let mult_hi = _mm256_srli_epi64::<32>(*y);
    prod_hi = _mm256_mul_epi32(prod_hi, mult_hi);

    prod_lo = _mm256_srli_epi64::<SHIFT>(prod_lo);
    prod_hi = _mm256_srli_epi64::<SHIFT>(prod_hi);

    prod_hi = _mm256_slli_epi64::<32>(prod_hi);
    *p = _mm256_blend_epi32::<0xaa>(prod_lo, prod_hi);
}

// QSHIFT is 16 - log_scale.
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn highbd_calculate_qcoeff_avx2<const LOG_SCALE: i32, const QSHIFT: i32>(
    coeff: *mut __m256i, round: *const __m256i, quant: *const __m256i, shift: *const __m256i) {
    const { assert!(QSHIFT == 16 - LOG_SCALE) };
    let mut tmp = _mm256_setzero_si256();
    let mut qcoeff: __m256i;
    qcoeff = _mm256_add_epi32(*coeff, *round);
    highbd_mul_shift_avx2::<16>(&qcoeff, quant, &mut tmp);
    qcoeff = _mm256_add_epi32(tmp, qcoeff);
    highbd_mul_shift_avx2::<QSHIFT>(&qcoeff, shift, coeff);
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn highbd_calculate_dqcoeff_avx2(qcoeff: __m256i, dequant: __m256i) -> __m256i {
    _mm256_mullo_epi32(qcoeff, dequant)
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn highbd_calculate_dqcoeff_log_scale_avx2<const LOG_SCALE: i32>(qcoeff: __m256i,
                                                                        dequant: __m256i)
                                                                        -> __m256i {
    let mut abs_coeff = _mm256_abs_epi32(qcoeff);
    let abs_coeff_ptr: *mut __m256i = &mut abs_coeff;
    highbd_mul_shift_avx2::<LOG_SCALE>(abs_coeff_ptr, &dequant, abs_coeff_ptr);
    _mm256_sign_epi32(abs_coeff, qcoeff)
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn highbd_store_coefficients_avx2(coeff0: __m256i, coeff1: __m256i,
                                         coeff_ptr: *mut tran_low_t) {
    _mm256_store_si256(coeff_ptr as *mut __m256i, coeff0);
    _mm256_store_si256(coeff_ptr.add(8) as *mut __m256i, coeff1);
}

#[no_mangle]
#[target_feature(enable = "avx2")]
#[allow(clippy::too_many_arguments)]
pub unsafe extern "C-unwind" fn aom_highbd_quantize_b_adaptive_avx2(
    coeff_ptr: *const tran_low_t, n_coeffs: isize, zbin_ptr: *const i16, round_ptr: *const i16,
    quant_ptr: *const i16, quant_shift_ptr: *const i16, qcoeff_ptr: *mut tran_low_t,
    dqcoeff_ptr: *mut tran_low_t, dequant_ptr: *const i16, eob_ptr: *mut u16, scan: *const i16,
    iscan: *const i16) {
    let mut index: i32 = 16;
    let mut non_zero_count: i32 = 0;
    let mut non_zero_count_prescan_add_zero: i32 = 0;
    let mut is_found0: i32 = 0;
    let mut is_found1: i32 = 0;
    let mut eob: i32 = -1;
    let zero = _mm256_setzero_si256();
    let mut zbin = zero;
    let mut round = zero;
    let mut quant = zero;
    let mut dequant = zero;
    let mut shift = zero;
    let mut coeff0: __m256i;
    let mut qcoeff0: __m256i;
    let mut coeff1: __m256i;
    let mut qcoeff1: __m256i;
    let mut cmp_mask: __m256i;
    let mut mask0 = zero;
    let mut mask1 = zero;
    let temp_mask0: __m128i;
    let temp_mask1: __m128i;
    let mut prescan_add = [0i32; 2];
    let mut thresh = [0i32; 2];
    const LOG_SCALE: i32 = 0;
    let wt: qm_val_t = (1 << AOM_QM_BITS) as qm_val_t;
    for i in 0..2 {
        prescan_add[i] = ROUND_POWER_OF_TWO!(*dequant_ptr.add(i) as i32 * EOB_FACTOR, 7);
        thresh[i] = (*zbin_ptr.add(i) as i32 * wt as i32 + prescan_add[i]) - 1;
    }
    let mut threshold = [zero; 2];
    threshold[0] = _mm256_set1_epi32(thresh[0]);
    threshold[1] = _mm256_set1_epi32(thresh[1]);
    threshold[0] = _mm256_blend_epi32::<0xfe>(threshold[0], threshold[1]);

    let mut first: i32 = -1;

    // Setup global values.
    highbd_load_b_values_avx2(zbin_ptr, &mut zbin, round_ptr, &mut round, quant_ptr, &mut quant,
                              dequant_ptr, &mut dequant, quant_shift_ptr, &mut shift);

    // Do DC and first 15 AC.
    coeff0 = _mm256_load_si256(coeff_ptr as *const __m256i);
    qcoeff0 = _mm256_abs_epi32(coeff0);
    coeff1 = _mm256_load_si256(coeff_ptr.add(8) as *const __m256i);
    qcoeff1 = _mm256_abs_epi32(coeff1);
    highbd_update_mask0_avx2(&qcoeff0, &qcoeff1, &threshold, iscan, &mut is_found0, &mut mask0);
    let mut temp0 = _mm256_cmpgt_epi32(qcoeff0, zbin);
    zbin = _mm256_unpackhi_epi64(zbin, zbin);
    let mut temp1 = _mm256_cmpgt_epi32(qcoeff1, zbin);
    cmp_mask = _mm256_permute4x64_epi64::<0xd8>(_mm256_packs_epi32(temp0, temp1));
    highbd_update_mask1_avx2(&cmp_mask, iscan, &mut is_found1, &mut mask1);
    threshold[0] = threshold[1];
    if _mm256_movemask_epi8(cmp_mask) == 0 {
        _mm256_store_si256(qcoeff_ptr as *mut __m256i, zero);
        _mm256_store_si256(qcoeff_ptr.add(8) as *mut __m256i, zero);
        _mm256_store_si256(dqcoeff_ptr as *mut __m256i, zero);
        _mm256_store_si256(dqcoeff_ptr.add(8) as *mut __m256i, zero);
        round = _mm256_unpackhi_epi64(round, round);
        quant = _mm256_unpackhi_epi64(quant, quant);
        shift = _mm256_unpackhi_epi64(shift, shift);
        dequant = _mm256_unpackhi_epi64(dequant, dequant);
    } else {
        highbd_calculate_qcoeff_avx2::<LOG_SCALE, { 16 - LOG_SCALE }>(&mut qcoeff0, &round,
                                                                       &quant, &shift);
        round = _mm256_unpackhi_epi64(round, round);
        quant = _mm256_unpackhi_epi64(quant, quant);
        shift = _mm256_unpackhi_epi64(shift, shift);
        highbd_calculate_qcoeff_avx2::<LOG_SCALE, { 16 - LOG_SCALE }>(&mut qcoeff1, &round,
                                                                       &quant, &shift);
        // Reinsert signs
        qcoeff0 = _mm256_sign_epi32(qcoeff0, coeff0);
        qcoeff1 = _mm256_sign_epi32(qcoeff1, coeff1);
        // Mask out zbin threshold coeffs
        qcoeff0 = _mm256_and_si256(qcoeff0, temp0);
        qcoeff1 = _mm256_and_si256(qcoeff1, temp1);
        highbd_store_coefficients_avx2(qcoeff0, qcoeff1, qcoeff_ptr);
        coeff0 = highbd_calculate_dqcoeff_avx2(qcoeff0, dequant);
        dequant = _mm256_unpackhi_epi64(dequant, dequant);
        coeff1 = highbd_calculate_dqcoeff_avx2(qcoeff1, dequant);
        highbd_store_coefficients_avx2(coeff0, coeff1, dqcoeff_ptr);
    }

    // AC only loop.
    while (index as isize) < n_coeffs {
        coeff0 = _mm256_load_si256(coeff_ptr.offset(index as isize) as *const __m256i);
        qcoeff0 = _mm256_abs_epi32(coeff0);
        coeff1 = _mm256_load_si256(coeff_ptr.offset(index as isize + 8) as *const __m256i);
        qcoeff1 = _mm256_abs_epi32(coeff1);
        highbd_update_mask0_avx2(&qcoeff0, &qcoeff1, &threshold, iscan.offset(index as isize),
                                 &mut is_found0, &mut mask0);
        temp0 = _mm256_cmpgt_epi32(qcoeff0, zbin);
        temp1 = _mm256_cmpgt_epi32(qcoeff1, zbin);
        cmp_mask = _mm256_permute4x64_epi64::<0xd8>(_mm256_packs_epi32(temp0, temp1));
        highbd_update_mask1_avx2(&cmp_mask, iscan.offset(index as isize), &mut is_found1,
                                 &mut mask1);
        if _mm256_movemask_epi8(cmp_mask) == 0 {
            _mm256_store_si256(qcoeff_ptr.offset(index as isize) as *mut __m256i, zero);
            _mm256_store_si256(qcoeff_ptr.offset(index as isize + 8) as *mut __m256i, zero);
            _mm256_store_si256(dqcoeff_ptr.offset(index as isize) as *mut __m256i, zero);
            _mm256_store_si256(dqcoeff_ptr.offset(index as isize + 8) as *mut __m256i, zero);
            index += 16;
            continue;
        }
        highbd_calculate_qcoeff_avx2::<LOG_SCALE, { 16 - LOG_SCALE }>(&mut qcoeff0, &round,
                                                                       &quant, &shift);
        highbd_calculate_qcoeff_avx2::<LOG_SCALE, { 16 - LOG_SCALE }>(&mut qcoeff1, &round,
                                                                       &quant, &shift);
        qcoeff0 = _mm256_sign_epi32(qcoeff0, coeff0);
        qcoeff1 = _mm256_sign_epi32(qcoeff1, coeff1);
        qcoeff0 = _mm256_and_si256(qcoeff0, temp0);
        qcoeff1 = _mm256_and_si256(qcoeff1, temp1);
        highbd_store_coefficients_avx2(qcoeff0, qcoeff1, qcoeff_ptr.offset(index as isize));
        coeff0 = highbd_calculate_dqcoeff_avx2(qcoeff0, dequant);
        coeff1 = highbd_calculate_dqcoeff_avx2(qcoeff1, dequant);
        highbd_store_coefficients_avx2(coeff0, coeff1, dqcoeff_ptr.offset(index as isize));
        index += 16;
    }
    if is_found0 != 0 {
        temp_mask0 = _mm_max_epi16(_mm256_castsi256_si128(mask0),
                                   _mm256_extracti128_si256::<1>(mask0));
        non_zero_count = calculate_non_zero_count(temp_mask0);
    }
    if is_found1 != 0 {
        temp_mask1 = _mm_max_epi16(_mm256_castsi256_si128(mask1),
                                   _mm256_extracti128_si256::<1>(mask1));
        non_zero_count_prescan_add_zero = calculate_non_zero_count(temp_mask1);
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
            if abs_coeff
                < (*zbin_ptr.add((rc != 0) as usize) as i32 * (1 << AOM_QM_BITS) + prescan_add_val)
            {
                *qcoeff_ptr.offset(rc as isize) = 0;
                *dqcoeff_ptr.offset(rc as isize) = 0;
                *eob_ptr = 0;
            }
        }
    }
}

#[no_mangle]
#[target_feature(enable = "avx2")]
#[allow(clippy::too_many_arguments)]
pub unsafe extern "C-unwind" fn aom_highbd_quantize_b_32x32_adaptive_avx2(
    coeff_ptr: *const tran_low_t, n_coeffs: isize, zbin_ptr: *const i16, round_ptr: *const i16,
    quant_ptr: *const i16, quant_shift_ptr: *const i16, qcoeff_ptr: *mut tran_low_t,
    dqcoeff_ptr: *mut tran_low_t, dequant_ptr: *const i16, eob_ptr: *mut u16, scan: *const i16,
    iscan: *const i16) {
    let mut index: i32 = 16;
    let mut non_zero_count: i32 = 0;
    let mut non_zero_count_prescan_add_zero: i32 = 0;
    let mut is_found0: i32 = 0;
    let mut is_found1: i32 = 0;
    let mut eob: i32 = -1;
    const LOG_SCALE: i32 = 1;
    let zero = _mm256_setzero_si256();
    let mut zbin: __m256i;
    let mut round: __m256i;
    let mut quant: __m256i;
    let mut dequant: __m256i;
    let mut shift: __m256i;
    let mut coeff0: __m256i;
    let mut qcoeff0: __m256i;
    let mut coeff1: __m256i;
    let mut qcoeff1: __m256i;
    let mut cmp_mask: __m256i;
    let mut mask0 = zero;
    let mut mask1 = zero;
    let temp_mask0: __m128i;
    let temp_mask1: __m128i;
    let one = _mm256_set1_epi32(1);
    let log_scale_vec = _mm256_set1_epi32(LOG_SCALE);
    let mut prescan_add = [0i32; 2];
    let mut thresh = [0i32; 2];
    let zbins: [i32; 2] = [ROUND_POWER_OF_TWO!(*zbin_ptr.add(0) as i32, LOG_SCALE),
                           ROUND_POWER_OF_TWO!(*zbin_ptr.add(1) as i32, LOG_SCALE)];
    let wt: qm_val_t = (1 << AOM_QM_BITS) as qm_val_t;
    for i in 0..2 {
        prescan_add[i] = ROUND_POWER_OF_TWO!(*dequant_ptr.add(i) as i32 * EOB_FACTOR, 7);
        thresh[i] = (zbins[i] * wt as i32 + prescan_add[i]) - 1;
    }
    let mut threshold = [zero; 2];
    threshold[0] = _mm256_set1_epi32(thresh[0]);
    threshold[1] = _mm256_set1_epi32(thresh[1]);
    threshold[0] = _mm256_blend_epi32::<0xfe>(threshold[0], threshold[1]);

    let mut first: i32 = -1;

    // Setup global values.
    zbin = _mm256_cvtepi16_epi32(_mm_load_si128(zbin_ptr as *const __m128i));
    round = _mm256_cvtepi16_epi32(_mm_load_si128(round_ptr as *const __m128i));
    quant = _mm256_cvtepi16_epi32(_mm_load_si128(quant_ptr as *const __m128i));
    dequant = _mm256_cvtepi16_epi32(_mm_load_si128(dequant_ptr as *const __m128i));
    shift = _mm256_cvtepi16_epi32(_mm_load_si128(quant_shift_ptr as *const __m128i));

    // Shift with rounding.
    zbin = _mm256_add_epi32(zbin, log_scale_vec);
    round = _mm256_add_epi32(round, log_scale_vec);
    zbin = _mm256_srli_epi32::<LOG_SCALE>(zbin);
    round = _mm256_srli_epi32::<LOG_SCALE>(round);
    zbin = _mm256_sub_epi32(zbin, one);

    // Do DC and first 15 AC.
    coeff0 = _mm256_load_si256(coeff_ptr as *const __m256i);
    qcoeff0 = _mm256_abs_epi32(coeff0);
    coeff1 = _mm256_load_si256(coeff_ptr.add(8) as *const __m256i);
    qcoeff1 = _mm256_abs_epi32(coeff1);
    highbd_update_mask0_avx2(&qcoeff0, &qcoeff1, &threshold, iscan, &mut is_found0, &mut mask0);
    let mut temp0 = _mm256_cmpgt_epi32(qcoeff0, zbin);
    zbin = _mm256_permute2x128_si256::<0x11>(zbin, zbin);
    let mut temp1 = _mm256_cmpgt_epi32(qcoeff1, zbin);
    cmp_mask = _mm256_permute4x64_epi64::<0xd8>(_mm256_packs_epi32(temp0, temp1));
    highbd_update_mask1_avx2(&cmp_mask, iscan, &mut is_found1, &mut mask1);
    threshold[0] = threshold[1];
    if _mm256_movemask_epi8(cmp_mask) == 0 {
        _mm256_store_si256(qcoeff_ptr as *mut __m256i, zero);
        _mm256_store_si256(qcoeff_ptr.add(8) as *mut __m256i, zero);
        _mm256_store_si256(dqcoeff_ptr as *mut __m256i, zero);
        _mm256_store_si256(dqcoeff_ptr.add(8) as *mut __m256i, zero);
        round = _mm256_permute2x128_si256::<0x11>(round, round);
        quant = _mm256_permute2x128_si256::<0x11>(quant, quant);
        shift = _mm256_permute2x128_si256::<0x11>(shift, shift);
        dequant = _mm256_permute2x128_si256::<0x11>(dequant, dequant);
    } else {
        highbd_calculate_qcoeff_avx2::<LOG_SCALE, { 16 - LOG_SCALE }>(&mut qcoeff0, &round,
                                                                       &quant, &shift);
        round = _mm256_permute2x128_si256::<0x11>(round, round);
        quant = _mm256_permute2x128_si256::<0x11>(quant, quant);
        shift = _mm256_permute2x128_si256::<0x11>(shift, shift);
        highbd_calculate_qcoeff_avx2::<LOG_SCALE, { 16 - LOG_SCALE }>(&mut qcoeff1, &round,
                                                                       &quant, &shift);
        // Reinsert signs
        qcoeff0 = _mm256_sign_epi32(qcoeff0, coeff0);
        qcoeff1 = _mm256_sign_epi32(qcoeff1, coeff1);
        // Mask out zbin threshold coeffs
        qcoeff0 = _mm256_and_si256(qcoeff0, temp0);
        qcoeff1 = _mm256_and_si256(qcoeff1, temp1);
        highbd_store_coefficients_avx2(qcoeff0, qcoeff1, qcoeff_ptr);
        coeff0 = highbd_calculate_dqcoeff_log_scale_avx2::<LOG_SCALE>(qcoeff0, dequant);
        dequant = _mm256_permute2x128_si256::<0x11>(dequant, dequant);
        coeff1 = highbd_calculate_dqcoeff_log_scale_avx2::<LOG_SCALE>(qcoeff1, dequant);
        highbd_store_coefficients_avx2(coeff0, coeff1, dqcoeff_ptr);
    }

    // AC only loop.
    while (index as isize) < n_coeffs {
        coeff0 = _mm256_load_si256(coeff_ptr.offset(index as isize) as *const __m256i);
        qcoeff0 = _mm256_abs_epi32(coeff0);
        coeff1 = _mm256_load_si256(coeff_ptr.offset(index as isize + 8) as *const __m256i);
        qcoeff1 = _mm256_abs_epi32(coeff1);
        highbd_update_mask0_avx2(&qcoeff0, &qcoeff1, &threshold, iscan.offset(index as isize),
                                 &mut is_found0, &mut mask0);
        temp0 = _mm256_cmpgt_epi32(qcoeff0, zbin);
        temp1 = _mm256_cmpgt_epi32(qcoeff1, zbin);
        cmp_mask = _mm256_permute4x64_epi64::<0xd8>(_mm256_packs_epi32(temp0, temp1));
        highbd_update_mask1_avx2(&cmp_mask, iscan.offset(index as isize), &mut is_found1,
                                 &mut mask1);
        if _mm256_movemask_epi8(cmp_mask) == 0 {
            _mm256_store_si256(qcoeff_ptr.offset(index as isize) as *mut __m256i, zero);
            _mm256_store_si256(qcoeff_ptr.offset(index as isize + 8) as *mut __m256i, zero);
            _mm256_store_si256(dqcoeff_ptr.offset(index as isize) as *mut __m256i, zero);
            _mm256_store_si256(dqcoeff_ptr.offset(index as isize + 8) as *mut __m256i, zero);
            index += 16;
            continue;
        }
        highbd_calculate_qcoeff_avx2::<LOG_SCALE, { 16 - LOG_SCALE }>(&mut qcoeff0, &round,
                                                                       &quant, &shift);
        highbd_calculate_qcoeff_avx2::<LOG_SCALE, { 16 - LOG_SCALE }>(&mut qcoeff1, &round,
                                                                       &quant, &shift);
        qcoeff0 = _mm256_sign_epi32(qcoeff0, coeff0);
        qcoeff1 = _mm256_sign_epi32(qcoeff1, coeff1);
        qcoeff0 = _mm256_and_si256(qcoeff0, temp0);
        qcoeff1 = _mm256_and_si256(qcoeff1, temp1);
        highbd_store_coefficients_avx2(qcoeff0, qcoeff1, qcoeff_ptr.offset(index as isize));
        coeff0 = highbd_calculate_dqcoeff_log_scale_avx2::<LOG_SCALE>(qcoeff0, dequant);
        coeff1 = highbd_calculate_dqcoeff_log_scale_avx2::<LOG_SCALE>(qcoeff1, dequant);
        highbd_store_coefficients_avx2(coeff0, coeff1, dqcoeff_ptr.offset(index as isize));
        index += 16;
    }
    if is_found0 != 0 {
        temp_mask0 = _mm_max_epi16(_mm256_castsi256_si128(mask0),
                                   _mm256_extracti128_si256::<1>(mask0));
        non_zero_count = calculate_non_zero_count(temp_mask0);
    }
    if is_found1 != 0 {
        temp_mask1 = _mm_max_epi16(_mm256_castsi256_si128(mask1),
                                   _mm256_extracti128_si256::<1>(mask1));
        non_zero_count_prescan_add_zero = calculate_non_zero_count(temp_mask1);
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
