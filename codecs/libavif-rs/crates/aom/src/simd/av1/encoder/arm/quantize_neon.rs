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

// Translated by hand from upstream/aom/av1/encoder/arm/quantize_neon.c
// (ADR 0056). Intrinsic sequences, helper structure, and names follow the
// original; only the AOM_ARCH_AARCH64 branches are retained. C locals that the
// original leaves uninitialized start zeroed here.
#![allow(clippy::too_many_arguments)]

use core::arch::aarch64::*;

use crate::simd::aom_dsp__arm__mem_neon::{load_tran_low_to_s16q, store_s16q_to_tran_low};
use crate::simd::aom_dsp__arm__sum_neon::horizontal_long_add_u16x8;
use crate::simd::types::*;

// aom_dsp/aom_dsp_common.h
const AOM_QM_BITS: i32 = 5;

// aom_ports/mem.h
macro_rules! ROUND_POWER_OF_TWO {
    ($value:expr, $n:expr) => {
        (($value) + ((1 << ($n)) >> 1)) >> ($n)
    };
}

#[inline(always)]
unsafe fn get_max_eob(v_eobmax: int16x8_t) -> u16 {
    let max_val: i16 = vmaxvq_s16(v_eobmax);
    (max_val as u16).wrapping_add(1)
}

#[inline(always)]
unsafe fn get_max_lane_eob(iscan: *const i16, v_eobmax: int16x8_t, v_mask: uint16x8_t) -> int16x8_t {
    let v_iscan: int16x8_t = vld1q_s16(iscan);
    let v_nz_iscan: int16x8_t = vbslq_s16(v_mask, v_iscan, vdupq_n_s16(-1));
    vmaxq_s16(v_eobmax, v_nz_iscan)
}

#[inline(always)]
unsafe fn quantize_fp_8(
    coeff_ptr: *const tran_low_t,
    qcoeff_ptr: *mut tran_low_t,
    dqcoeff_ptr: *mut tran_low_t,
    v_quant: int16x8_t,
    v_dequant: int16x8_t,
    v_round: int16x8_t,
    v_zero: int16x8_t,
) -> uint16x8_t {
    let v_coeff: int16x8_t = load_tran_low_to_s16q(coeff_ptr);
    let v_coeff_sign: int16x8_t = vshrq_n_s16::<15>(v_coeff);
    let v_abs: int16x8_t = vabsq_s16(v_coeff);
    let v_tmp: int16x8_t = vqaddq_s16(v_abs, v_round);
    let v_tmp2: int16x8_t = vshrq_n_s16::<1>(vqdmulhq_s16(v_tmp, v_quant));
    let v_nz_mask: uint16x8_t = vcgtq_s16(v_tmp2, v_zero);
    let v_qcoeff_a: int16x8_t = veorq_s16(v_tmp2, v_coeff_sign);
    let v_qcoeff: int16x8_t = vsubq_s16(v_qcoeff_a, v_coeff_sign);
    let v_dqcoeff: int16x8_t = vmulq_s16(v_qcoeff, v_dequant);
    store_s16q_to_tran_low(qcoeff_ptr, v_qcoeff);
    store_s16q_to_tran_low(dqcoeff_ptr, v_dqcoeff);
    v_nz_mask
}

#[no_mangle]
pub unsafe extern "C-unwind" fn av1_quantize_fp_neon(
    mut coeff_ptr: *const tran_low_t,
    mut count: isize,
    _zbin_ptr: *const i16,
    round_ptr: *const i16,
    quant_ptr: *const i16,
    _quant_shift_ptr: *const i16,
    mut qcoeff_ptr: *mut tran_low_t,
    mut dqcoeff_ptr: *mut tran_low_t,
    dequant_ptr: *const i16,
    eob_ptr: *mut u16,
    _scan: *const i16,
    mut iscan: *const i16,
) {
    // TODO(jingning) Decide the need of these arguments after the
    // quantization process is completed.

    // Quantization pass: All coefficients with index >= zero_flag are
    // skippable. Note: zero_flag can be zero.
    let v_zero: int16x8_t = vdupq_n_s16(0);
    let mut v_quant: int16x8_t = vld1q_s16(quant_ptr);
    let mut v_dequant: int16x8_t = vld1q_s16(dequant_ptr);
    let mut v_round: int16x8_t = vld1q_s16(round_ptr);
    let mut v_eobmax_76543210: int16x8_t = vdupq_n_s16(-1);
    let mut v_nz_mask: uint16x8_t;
    // process dc and the first seven ac coeffs
    v_nz_mask = quantize_fp_8(coeff_ptr, qcoeff_ptr, dqcoeff_ptr, v_quant, v_dequant, v_round, v_zero);
    v_eobmax_76543210 = get_max_lane_eob(iscan, v_eobmax_76543210, v_nz_mask);
    // overwrite the dc constants with ac constants
    v_quant = vdupq_lane_s16::<1>(vget_low_s16(v_quant));
    v_dequant = vdupq_lane_s16::<1>(vget_low_s16(v_dequant));
    v_round = vdupq_lane_s16::<1>(vget_low_s16(v_round));

    count -= 8;
    // now process the rest of the ac coeffs
    loop {
        coeff_ptr = coeff_ptr.add(8);
        qcoeff_ptr = qcoeff_ptr.add(8);
        dqcoeff_ptr = dqcoeff_ptr.add(8);
        iscan = iscan.add(8);
        v_nz_mask =
            quantize_fp_8(coeff_ptr, qcoeff_ptr, dqcoeff_ptr, v_quant, v_dequant, v_round, v_zero);
        v_eobmax_76543210 = get_max_lane_eob(iscan, v_eobmax_76543210, v_nz_mask);
        count -= 8;
        if count <= 0 {
            break;
        }
    }
    *eob_ptr = get_max_eob(v_eobmax_76543210);
}

#[inline(always)]
unsafe fn quantize_lp_8(
    coeff_ptr: *const i16,
    qcoeff_ptr: *mut i16,
    dqcoeff_ptr: *mut i16,
    v_quant: int16x8_t,
    v_dequant: int16x8_t,
    v_round: int16x8_t,
    v_zero: int16x8_t,
) -> uint16x8_t {
    let v_coeff: int16x8_t = vld1q_s16(coeff_ptr);
    let v_coeff_sign: int16x8_t = vshrq_n_s16::<15>(v_coeff);
    let v_abs: int16x8_t = vabsq_s16(v_coeff);
    let v_tmp: int16x8_t = vqaddq_s16(v_abs, v_round);
    let v_tmp2: int16x8_t = vshrq_n_s16::<1>(vqdmulhq_s16(v_tmp, v_quant));
    let v_nz_mask: uint16x8_t = vcgtq_s16(v_tmp2, v_zero);
    let v_qcoeff_a: int16x8_t = veorq_s16(v_tmp2, v_coeff_sign);
    let v_qcoeff: int16x8_t = vsubq_s16(v_qcoeff_a, v_coeff_sign);
    let v_dqcoeff: int16x8_t = vmulq_s16(v_qcoeff, v_dequant);
    vst1q_s16(qcoeff_ptr, v_qcoeff);
    vst1q_s16(dqcoeff_ptr, v_dqcoeff);
    v_nz_mask
}

#[no_mangle]
pub unsafe extern "C-unwind" fn av1_quantize_lp_neon(
    mut coeff_ptr: *const i16,
    n_coeffs: isize,
    round_ptr: *const i16,
    quant_ptr: *const i16,
    mut qcoeff_ptr: *mut i16,
    mut dqcoeff_ptr: *mut i16,
    dequant_ptr: *const i16,
    eob_ptr: *mut u16,
    _scan: *const i16,
    mut iscan: *const i16,
) {
    // Quantization pass: All coefficients with index >= zero_flag are
    // skippable. Note: zero_flag can be zero.
    let v_zero: int16x8_t = vdupq_n_s16(0);
    let mut v_quant: int16x8_t = vld1q_s16(quant_ptr);
    let mut v_dequant: int16x8_t = vld1q_s16(dequant_ptr);
    let mut v_round: int16x8_t = vld1q_s16(round_ptr);
    let mut v_eobmax_76543210: int16x8_t = vdupq_n_s16(-1);
    let mut v_nz_mask: uint16x8_t;
    let mut count: isize = n_coeffs;

    // process dc and the first seven ac coeffs
    v_nz_mask = quantize_lp_8(coeff_ptr, qcoeff_ptr, dqcoeff_ptr, v_quant, v_dequant, v_round, v_zero);
    v_eobmax_76543210 = get_max_lane_eob(iscan, v_eobmax_76543210, v_nz_mask);
    // overwrite the dc constants with ac constants
    v_quant = vdupq_lane_s16::<1>(vget_low_s16(v_quant));
    v_dequant = vdupq_lane_s16::<1>(vget_low_s16(v_dequant));
    v_round = vdupq_lane_s16::<1>(vget_low_s16(v_round));

    count -= 8;
    // now process the rest of the ac coeffs
    loop {
        coeff_ptr = coeff_ptr.add(8);
        qcoeff_ptr = qcoeff_ptr.add(8);
        dqcoeff_ptr = dqcoeff_ptr.add(8);
        iscan = iscan.add(8);
        v_nz_mask =
            quantize_lp_8(coeff_ptr, qcoeff_ptr, dqcoeff_ptr, v_quant, v_dequant, v_round, v_zero);
        v_eobmax_76543210 = get_max_lane_eob(iscan, v_eobmax_76543210, v_nz_mask);
        count -= 8;
        if count == 0 {
            break;
        }
    }
    *eob_ptr = get_max_eob(v_eobmax_76543210);
}

#[inline(always)]
unsafe fn quantize_fp_logscale_8(
    coeff_ptr: *const tran_low_t,
    qcoeff_ptr: *mut tran_low_t,
    dqcoeff_ptr: *mut tran_low_t,
    v_quant: int16x8_t,
    v_dequant: int16x8_t,
    v_round: int16x8_t,
    v_zero: int16x8_t,
    log_scale: i32,
) -> uint16x8_t {
    let v_log_scale_minus_1: int16x8_t = vdupq_n_s16((log_scale - 1) as i16);
    let v_neg_log_scale_plus_1: int16x8_t = vdupq_n_s16((-(1 + log_scale)) as i16);
    let v_coeff: int16x8_t = load_tran_low_to_s16q(coeff_ptr);
    let v_coeff_sign: int16x8_t = vshrq_n_s16::<15>(v_coeff);
    let v_abs_coeff: int16x8_t = vabsq_s16(v_coeff);
    let v_mask: uint16x8_t = vcgeq_s16(v_abs_coeff, vshlq_s16(v_dequant, v_neg_log_scale_plus_1));
    // const int64_t tmp = vmask ? (int64_t)abs_coeff + log_scaled_round : 0
    let v_tmp: int16x8_t =
        vandq_s16(vqaddq_s16(v_abs_coeff, v_round), vreinterpretq_s16_u16(v_mask));
    let v_tmp2: int16x8_t = vqdmulhq_s16(vshlq_s16(v_tmp, v_log_scale_minus_1), v_quant);
    let v_nz_mask: uint16x8_t = vcgtq_s16(v_tmp2, v_zero);
    let v_qcoeff: int16x8_t = vsubq_s16(veorq_s16(v_tmp2, v_coeff_sign), v_coeff_sign);
    // Multiplying by dequant here will use all 16 bits. Cast to unsigned before
    // shifting right. (vshlq_s16 will shift right if shift value is negative)
    let v_abs_dqcoeff: uint16x8_t = vshlq_u16(
        vreinterpretq_u16_s16(vmulq_s16(v_tmp2, v_dequant)),
        vdupq_n_s16((-log_scale) as i16),
    );
    let v_dqcoeff: int16x8_t =
        vsubq_s16(veorq_s16(vreinterpretq_s16_u16(v_abs_dqcoeff), v_coeff_sign), v_coeff_sign);
    store_s16q_to_tran_low(qcoeff_ptr, v_qcoeff);
    store_s16q_to_tran_low(dqcoeff_ptr, v_dqcoeff);
    v_nz_mask
}

#[inline(always)]
unsafe fn quantize_fp_logscale2_8(
    coeff_ptr: *const tran_low_t,
    qcoeff_ptr: *mut tran_low_t,
    dqcoeff_ptr: *mut tran_low_t,
    v_quant: int16x8_t,
    v_dequant: int16x8_t,
    v_round: int16x8_t,
    v_zero: int16x8_t,
) -> uint16x8_t {
    let v_coeff: int16x8_t = load_tran_low_to_s16q(coeff_ptr);
    let v_coeff_sign: int16x8_t = vshrq_n_s16::<15>(v_coeff);
    let v_abs_coeff: int16x8_t = vabsq_s16(v_coeff);
    let v_mask: uint16x8_t = vcgeq_u16(
        vshlq_n_u16::<1>(vreinterpretq_u16_s16(v_abs_coeff)),
        vshrq_n_u16::<2>(vreinterpretq_u16_s16(v_dequant)),
    );
    // abs_coeff = vmask ? (int64_t)abs_coeff + log_scaled_round : 0
    let v_tmp: int16x8_t =
        vandq_s16(vqaddq_s16(v_abs_coeff, v_round), vreinterpretq_s16_u16(v_mask));
    // tmp32 = (int)((abs_coeff * quant_ptr[rc != 0]) >> (16 - log_scale));
    let v_tmp2: int16x8_t = vorrq_s16(
        vshlq_n_s16::<1>(vqdmulhq_s16(v_tmp, v_quant)),
        vreinterpretq_s16_u16(vshrq_n_u16::<14>(vreinterpretq_u16_s16(vmulq_s16(v_tmp, v_quant)))),
    );
    let v_nz_mask: uint16x8_t = vcgtq_s16(v_tmp2, v_zero);
    let v_qcoeff: int16x8_t = vsubq_s16(veorq_s16(v_tmp2, v_coeff_sign), v_coeff_sign);
    // const tran_low_t abs_dqcoeff = (tmp32 * dequant_ptr[rc != 0]) >> log_scale;
    let v_abs_dqcoeff: int16x8_t = vorrq_s16(
        vshlq_n_s16::<13>(vqdmulhq_s16(v_tmp2, v_dequant)),
        vreinterpretq_s16_u16(vshrq_n_u16::<2>(vreinterpretq_u16_s16(vmulq_s16(
            v_tmp2, v_dequant,
        )))),
    );
    let v_dqcoeff: int16x8_t = vsubq_s16(veorq_s16(v_abs_dqcoeff, v_coeff_sign), v_coeff_sign);
    store_s16q_to_tran_low(qcoeff_ptr, v_qcoeff);
    store_s16q_to_tran_low(dqcoeff_ptr, v_dqcoeff);
    v_nz_mask
}

#[inline(always)]
unsafe fn quantize_fp_no_qmatrix_neon(
    mut coeff_ptr: *const tran_low_t,
    n_coeffs: isize,
    round_ptr: *const i16,
    quant_ptr: *const i16,
    mut qcoeff_ptr: *mut tran_low_t,
    mut dqcoeff_ptr: *mut tran_low_t,
    dequant_ptr: *const i16,
    eob_ptr: *mut u16,
    mut iscan: *const i16,
    log_scale: i32,
) {
    let v_zero: int16x8_t = vdupq_n_s16(0);
    let mut v_quant: int16x8_t = vld1q_s16(quant_ptr);
    let mut v_dequant: int16x8_t = vld1q_s16(dequant_ptr);
    let v_round_no_scale: int16x8_t = vld1q_s16(round_ptr);
    let mut v_round: int16x8_t =
        vqrdmulhq_n_s16(v_round_no_scale, (1i32 << (15 - log_scale)) as i16);
    let mut v_eobmax_76543210: int16x8_t = vdupq_n_s16(-1);
    let mut non_zero_count: isize = n_coeffs;

    debug_assert!(n_coeffs > 16);
    // Pre-scan pass
    let v_dequant_scaled: int16x8_t = vshlq_s16(v_dequant, vdupq_n_s16((-(1 + log_scale)) as i16));
    let v_zbin_s16: int16x8_t = vdupq_lane_s16::<1>(vget_low_s16(v_dequant_scaled));
    let mut i: isize = n_coeffs;
    loop {
        let v_coeff_a: int16x8_t = load_tran_low_to_s16q(coeff_ptr.offset(i - 8));
        let v_coeff_b: int16x8_t = load_tran_low_to_s16q(coeff_ptr.offset(i - 16));
        let v_abs_coeff_a: int16x8_t = vabsq_s16(v_coeff_a);
        let v_abs_coeff_b: int16x8_t = vabsq_s16(v_coeff_b);
        let v_mask_a: uint16x8_t = vcgeq_s16(v_abs_coeff_a, v_zbin_s16);
        let v_mask_b: uint16x8_t = vcgeq_s16(v_abs_coeff_b, v_zbin_s16);
        // If the coefficient is in the base ZBIN range, then discard.
        if horizontal_long_add_u16x8(v_mask_a, v_mask_b) == 0 {
            non_zero_count -= 16;
        } else {
            break;
        }
        i -= 16;
        if i <= 0 {
            break;
        }
    }

    let remaining_zcoeffs: isize = n_coeffs - non_zero_count;
    core::ptr::write_bytes(qcoeff_ptr.offset(non_zero_count), 0, remaining_zcoeffs as usize);
    core::ptr::write_bytes(dqcoeff_ptr.offset(non_zero_count), 0, remaining_zcoeffs as usize);

    // process dc and the first seven ac coeffs
    let mut v_nz_mask: uint16x8_t;
    if log_scale == 2 {
        v_nz_mask = quantize_fp_logscale2_8(
            coeff_ptr, qcoeff_ptr, dqcoeff_ptr, v_quant, v_dequant, v_round, v_zero,
        );
    } else {
        v_nz_mask = quantize_fp_logscale_8(
            coeff_ptr, qcoeff_ptr, dqcoeff_ptr, v_quant, v_dequant, v_round, v_zero, log_scale,
        );
    }
    v_eobmax_76543210 = get_max_lane_eob(iscan, v_eobmax_76543210, v_nz_mask);
    // overwrite the dc constants with ac constants
    v_quant = vdupq_lane_s16::<1>(vget_low_s16(v_quant));
    v_dequant = vdupq_lane_s16::<1>(vget_low_s16(v_dequant));
    v_round = vdupq_lane_s16::<1>(vget_low_s16(v_round));

    let mut count: isize = non_zero_count - 8;
    while count > 0 {
        coeff_ptr = coeff_ptr.add(8);
        qcoeff_ptr = qcoeff_ptr.add(8);
        dqcoeff_ptr = dqcoeff_ptr.add(8);
        iscan = iscan.add(8);
        if log_scale == 2 {
            v_nz_mask = quantize_fp_logscale2_8(
                coeff_ptr, qcoeff_ptr, dqcoeff_ptr, v_quant, v_dequant, v_round, v_zero,
            );
        } else {
            v_nz_mask = quantize_fp_logscale_8(
                coeff_ptr, qcoeff_ptr, dqcoeff_ptr, v_quant, v_dequant, v_round, v_zero,
                log_scale,
            );
        }
        v_eobmax_76543210 = get_max_lane_eob(iscan, v_eobmax_76543210, v_nz_mask);
        count -= 8;
    }
    *eob_ptr = get_max_eob(v_eobmax_76543210);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn av1_quantize_fp_32x32_neon(
    coeff_ptr: *const tran_low_t,
    n_coeffs: isize,
    _zbin_ptr: *const i16,
    round_ptr: *const i16,
    quant_ptr: *const i16,
    _quant_shift_ptr: *const i16,
    qcoeff_ptr: *mut tran_low_t,
    dqcoeff_ptr: *mut tran_low_t,
    dequant_ptr: *const i16,
    eob_ptr: *mut u16,
    _scan: *const i16,
    iscan: *const i16,
) {
    quantize_fp_no_qmatrix_neon(
        coeff_ptr, n_coeffs, round_ptr, quant_ptr, qcoeff_ptr, dqcoeff_ptr, dequant_ptr, eob_ptr,
        iscan, 1,
    );
}

#[no_mangle]
pub unsafe extern "C-unwind" fn av1_quantize_fp_64x64_neon(
    coeff_ptr: *const tran_low_t,
    n_coeffs: isize,
    _zbin_ptr: *const i16,
    round_ptr: *const i16,
    quant_ptr: *const i16,
    _quant_shift_ptr: *const i16,
    qcoeff_ptr: *mut tran_low_t,
    dqcoeff_ptr: *mut tran_low_t,
    dequant_ptr: *const i16,
    eob_ptr: *mut u16,
    _scan: *const i16,
    iscan: *const i16,
) {
    quantize_fp_no_qmatrix_neon(
        coeff_ptr, n_coeffs, round_ptr, quant_ptr, qcoeff_ptr, dqcoeff_ptr, dequant_ptr, eob_ptr,
        iscan, 2,
    );
}

#[inline(always)]
unsafe fn quantize_b_logscale0_8(
    coeff: int16x8_t,
    abs: int16x8_t,
    cond: uint16x8_t,
    round: int16x8_t,
    dequant: int16x8_t,
    quant: int16x8_t,
    quant_shift: int16x8_t,
    qcoeff_ptr: *mut tran_low_t,
    dqcoeff_ptr: *mut tran_low_t,
) -> uint16x8_t {
    let zero: int16x8_t = vdupq_n_s16(0);

    let coeff_sign: int16x8_t = vreinterpretq_s16_u16(vcltq_s16(coeff, zero));

    let mut tmp: int16x8_t = vqaddq_s16(abs, round);
    tmp = vsraq_n_s16::<1>(tmp, vqdmulhq_s16(tmp, quant));
    tmp = vqdmulhq_s16(tmp, quant_shift);

    let mut qcoeff: int16x8_t = vsubq_s16(veorq_s16(tmp, coeff_sign), coeff_sign);
    qcoeff = vbslq_s16(cond, qcoeff, zero);
    store_s16q_to_tran_low(qcoeff_ptr, qcoeff);

    let mut dqcoeff: int16x8_t = vmulq_s16(tmp, dequant);
    dqcoeff = vsubq_s16(veorq_s16(dqcoeff, coeff_sign), coeff_sign);
    dqcoeff = vbslq_s16(cond, dqcoeff, zero);
    store_s16q_to_tran_low(dqcoeff_ptr, dqcoeff);

    let tmp_mask: uint16x8_t = vcgtq_s16(tmp, zero);
    let nz_mask: uint16x8_t = vandq_u16(tmp_mask, cond);

    nz_mask
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_quantize_b_neon(
    coeff_ptr: *const tran_low_t,
    n_coeffs: isize,
    zbin_ptr: *const i16,
    round_ptr: *const i16,
    quant_ptr: *const i16,
    quant_shift_ptr: *const i16,
    qcoeff_ptr: *mut tran_low_t,
    dqcoeff_ptr: *mut tran_low_t,
    dequant_ptr: *const i16,
    eob_ptr: *mut u16,
    _scan: *const i16,
    iscan: *const i16,
) {
    let mut v_eobmax_76543210: int16x8_t = vdupq_n_s16(-1);

    let v_zbins: int16x8_t = vdupq_n_s16(*zbin_ptr.add(1));
    let v_round: int16x8_t = vdupq_n_s16(*round_ptr.add(1));
    let v_dequant: int16x8_t = vdupq_n_s16(*dequant_ptr.add(1));
    let v_quant: int16x8_t = vdupq_n_s16(*quant_ptr.add(1));
    // Shift by 1 in order to save one shift in the kernel function.
    let v_quant_shift: int16x8_t = vdupq_n_s16(*quant_shift_ptr.add(1) >> 1);

    let v_zbins0: int16x8_t = vsetq_lane_s16::<0>(*zbin_ptr, v_zbins);
    let mut v_coeff: int16x8_t = load_tran_low_to_s16q(coeff_ptr);
    let mut v_abs: int16x8_t = vabsq_s16(v_coeff);
    let mut v_cond: uint16x8_t = vcgeq_s16(v_abs, v_zbins0);

    let mut nz_check: u64 = vget_lane_u64::<0>(vreinterpret_u64_u8(vmovn_u16(v_cond)));
    if nz_check != 0 {
        let v_round0: int16x8_t = vsetq_lane_s16::<0>(*round_ptr, v_round);
        let v_quant0: int16x8_t = vsetq_lane_s16::<0>(*quant_ptr, v_quant);
        let v_dequant0: int16x8_t = vsetq_lane_s16::<0>(*dequant_ptr, v_dequant);
        // Shift by 1 in order to save one shift in the kernel function.
        let v_quant_shift0: int16x8_t = vsetq_lane_s16::<0>(*quant_shift_ptr >> 1, v_quant_shift);

        let v_nz_mask: uint16x8_t = quantize_b_logscale0_8(
            v_coeff, v_abs, v_cond, v_round0, v_dequant0, v_quant0, v_quant_shift0, qcoeff_ptr,
            dqcoeff_ptr,
        );

        let v_iscan: int16x8_t = vld1q_s16(iscan);
        let v_eobmax: int16x8_t = vmaxq_s16(v_iscan, v_eobmax_76543210);
        v_eobmax_76543210 = vbslq_s16(v_nz_mask, v_eobmax, v_eobmax_76543210);
    } else {
        store_s16q_to_tran_low(qcoeff_ptr, vdupq_n_s16(0));
        store_s16q_to_tran_low(dqcoeff_ptr, vdupq_n_s16(0));
    }

    let mut i: i32 = 8;
    while (i as isize) < n_coeffs {
        v_coeff = load_tran_low_to_s16q(coeff_ptr.offset(i as isize));
        v_abs = vabsq_s16(v_coeff);
        v_cond = vcgeq_s16(v_abs, v_zbins);

        nz_check = vget_lane_u64::<0>(vreinterpret_u64_u8(vmovn_u16(v_cond)));
        if nz_check != 0 {
            let v_nz_mask: uint16x8_t = quantize_b_logscale0_8(
                v_coeff,
                v_abs,
                v_cond,
                v_round,
                v_dequant,
                v_quant,
                v_quant_shift,
                qcoeff_ptr.offset(i as isize),
                dqcoeff_ptr.offset(i as isize),
            );

            let v_iscan: int16x8_t = vld1q_s16(iscan.offset(i as isize));
            let v_eobmax: int16x8_t = vmaxq_s16(v_iscan, v_eobmax_76543210);
            v_eobmax_76543210 = vbslq_s16(v_nz_mask, v_eobmax, v_eobmax_76543210);
        } else {
            store_s16q_to_tran_low(qcoeff_ptr.offset(i as isize), vdupq_n_s16(0));
            store_s16q_to_tran_low(dqcoeff_ptr.offset(i as isize), vdupq_n_s16(0));
        }
        i += 8;
    }
    *eob_ptr = get_max_eob(v_eobmax_76543210);
}

macro_rules! QM_MULL_SHIFT {
    ($x0:expr, $x1:expr) => {
        vreinterpretq_s16_u16(vorrq_u16(
            vreinterpretq_u16_s16(vshlq_n_s16::<{ 15 - AOM_QM_BITS }>(vqdmulhq_s16(
                $x0,
                vreinterpretq_s16_u16($x1),
            ))),
            vshrq_n_u16::<AOM_QM_BITS>(vmulq_u16(vreinterpretq_u16_s16($x0), $x1)),
        ))
    };
}

unsafe fn aom_quantize_b_helper_16x16_neon(
    coeff_ptr: *const tran_low_t,
    n_coeffs: isize,
    zbin_ptr: *const i16,
    round_ptr: *const i16,
    quant_ptr: *const i16,
    quant_shift_ptr: *const i16,
    qcoeff_ptr: *mut tran_low_t,
    dqcoeff_ptr: *mut tran_low_t,
    dequant_ptr: *const i16,
    eob_ptr: *mut u16,
    _scan: *const i16,
    iscan: *const i16,
    qm_ptr: *const qm_val_t,
    iqm_ptr: *const qm_val_t,
) {
    let mut vwt: uint16x8_t = vdupq_n_u16(0);
    let mut viwt: uint16x8_t;
    let zbins: [i32; 2] = [*zbin_ptr as i32, *zbin_ptr.add(1) as i32];

    core::ptr::write_bytes(qcoeff_ptr, 0, n_coeffs as usize);
    core::ptr::write_bytes(dqcoeff_ptr, 0, n_coeffs as usize);

    let zero: int16x8_t = vdupq_n_s16(0);
    let mut v_eobmax_76543210: int16x8_t = vreinterpretq_s16_u16(vceqq_s16(zero, zero));

    let mut vzbins: int16x8_t = vdupq_n_s16(zbins[1] as i16);
    let mut vround: int16x8_t = vdupq_n_s16(*round_ptr.add(1));
    let mut vdequant: int16x8_t = vdupq_n_s16(*dequant_ptr.add(1));
    let mut vquant: int16x8_t = vdupq_n_s16(*quant_ptr.add(1));
    let mut vquant_shift: int16x8_t = vdupq_n_s16(*quant_shift_ptr.add(1));

    let mut v_coeff: int16x8_t = load_tran_low_to_s16q(coeff_ptr);
    let mut v_coeff_sign: int16x8_t = vshrq_n_s16::<15>(v_coeff);
    let mut v_abs: int16x8_t = vabsq_s16(v_coeff);
    vzbins = vsetq_lane_s16::<0>(zbins[0] as i16, vzbins);
    let mut vcond: uint16x8_t;
    if qm_ptr.is_null() {
        vcond = vcgeq_s16(v_abs, vzbins);
    } else {
        vwt = vmovl_u8(vld1_u8(qm_ptr));
        vcond = vcgeq_s16(QM_MULL_SHIFT!(v_abs, vwt), vzbins);
    }
    let mut nz_check: u64 = vget_lane_u64::<0>(vreinterpret_u64_u8(vmovn_u16(vcond)));
    if nz_check != 0 {
        vround = vsetq_lane_s16::<0>(*round_ptr, vround);
        vquant = vsetq_lane_s16::<0>(*quant_ptr, vquant);
        vdequant = vsetq_lane_s16::<0>(*dequant_ptr, vdequant);
        vquant_shift = vsetq_lane_s16::<0>(*quant_shift_ptr, vquant_shift);

        let vtmp: int16x8_t = vqaddq_s16(v_abs, vround);

        let mut vtmp2: int16x8_t;
        if qm_ptr.is_null() {
            vtmp2 = vsraq_n_s16::<1>(vtmp, vqdmulhq_s16(vtmp, vquant));
        } else {
            vtmp2 = QM_MULL_SHIFT!(vtmp, vwt);
            vtmp2 = vaddq_s16(vtmp2, vtmp);
        }

        vtmp2 = vshrq_n_s16::<1>(vqdmulhq_s16(vtmp2, vquant_shift));
        let mut vdest: int16x8_t = vsubq_s16(veorq_s16(vtmp2, v_coeff_sign), v_coeff_sign);
        let mut coeff_nz_mask: int16x8_t =
            vbslq_s16(vcond, vdest, load_tran_low_to_s16q(qcoeff_ptr));
        store_s16q_to_tran_low(qcoeff_ptr, coeff_nz_mask);

        if !iqm_ptr.is_null() {
            viwt = vmovl_u8(vld1_u8(iqm_ptr));
            vdequant = QM_MULL_SHIFT!(vdequant, viwt);
        }
        let v_deq_abs: int16x8_t = vmulq_s16(vtmp2, vdequant);
        vdest = vsubq_s16(veorq_s16(v_deq_abs, v_coeff_sign), v_coeff_sign);
        coeff_nz_mask = vbslq_s16(vcond, vdest, load_tran_low_to_s16q(dqcoeff_ptr));
        store_s16q_to_tran_low(dqcoeff_ptr, coeff_nz_mask);

        vround = vsetq_lane_s16::<0>(*round_ptr.add(1), vround);
        vquant = vsetq_lane_s16::<0>(*quant_ptr.add(1), vquant);
        vdequant = vsetq_lane_s16::<0>(*dequant_ptr.add(1), vdequant);
        vquant_shift = vsetq_lane_s16::<0>(*quant_shift_ptr.add(1), vquant_shift);

        let vtmp_mask: uint16x8_t = vcgtq_s16(vtmp2, zero);
        let v_nz_mask: uint16x8_t = vandq_u16(vtmp_mask, vcond);
        let v_iscan: int16x8_t = vld1q_s16(iscan);
        vcond = vandq_u16(v_nz_mask, vcgtq_s16(v_iscan, v_eobmax_76543210));
        v_eobmax_76543210 = vbslq_s16(vcond, v_iscan, v_eobmax_76543210);
    }
    vzbins = vsetq_lane_s16::<0>(zbins[1] as i16, vzbins);

    let mut i: i32 = 8;
    while (i as isize) < n_coeffs {
        let i_ = i as isize;
        v_coeff = load_tran_low_to_s16q(coeff_ptr.offset(i_));
        v_coeff_sign = vshrq_n_s16::<15>(v_coeff);
        v_abs = vabsq_s16(v_coeff);

        if qm_ptr.is_null() {
            vcond = vcgeq_s16(v_abs, vzbins);
        } else {
            vwt = vmovl_u8(vld1_u8(qm_ptr.offset(i_)));
            vcond = vcgeq_s16(QM_MULL_SHIFT!(v_abs, vwt), vzbins);
        }
        nz_check = vget_lane_u64::<0>(vreinterpret_u64_u8(vmovn_u16(vcond)));
        if nz_check != 0 {
            let vtmp: int16x8_t = vqaddq_s16(v_abs, vround);

            let mut vtmp2: int16x8_t;
            if qm_ptr.is_null() {
                vtmp2 = vsraq_n_s16::<1>(vtmp, vqdmulhq_s16(vtmp, vquant));
            } else {
                vtmp2 = QM_MULL_SHIFT!(vtmp, vwt);
                vtmp2 = vaddq_s16(vtmp2, vtmp);
            }

            vtmp2 = vshrq_n_s16::<1>(vqdmulhq_s16(vtmp2, vquant_shift));
            let mut vdest: int16x8_t = vsubq_s16(veorq_s16(vtmp2, v_coeff_sign), v_coeff_sign);
            let mut coeff_nz_mask: int16x8_t =
                vbslq_s16(vcond, vdest, load_tran_low_to_s16q(qcoeff_ptr.offset(i_)));
            store_s16q_to_tran_low(qcoeff_ptr.offset(i_), coeff_nz_mask);

            if !iqm_ptr.is_null() {
                viwt = vmovl_u8(vld1_u8(iqm_ptr.offset(i_)));
                vdequant = QM_MULL_SHIFT!(vdequant, viwt);
            }
            let v_deq_abs: int16x8_t = vmulq_s16(vtmp2, vdequant);
            vdest = vsubq_s16(veorq_s16(v_deq_abs, v_coeff_sign), v_coeff_sign);
            coeff_nz_mask = vbslq_s16(vcond, vdest, load_tran_low_to_s16q(dqcoeff_ptr.offset(i_)));
            store_s16q_to_tran_low(dqcoeff_ptr.offset(i_), coeff_nz_mask);

            let vtmp_mask: uint16x8_t = vcgtq_s16(vtmp2, zero);
            let v_nz_mask: uint16x8_t = vandq_u16(vtmp_mask, vcond);
            let v_iscan: int16x8_t = vld1q_s16(iscan.offset(i_));
            vcond = vandq_u16(v_nz_mask, vcgtq_s16(v_iscan, v_eobmax_76543210));
            v_eobmax_76543210 = vbslq_s16(vcond, v_iscan, v_eobmax_76543210);
        }
        i += 8;
    }
    *eob_ptr = get_max_eob(v_eobmax_76543210);
}

unsafe fn aom_quantize_b_helper_32x32_neon(
    coeff_ptr: *const tran_low_t,
    n_coeffs: isize,
    zbin_ptr: *const i16,
    round_ptr: *const i16,
    quant_ptr: *const i16,
    quant_shift_ptr: *const i16,
    qcoeff_ptr: *mut tran_low_t,
    dqcoeff_ptr: *mut tran_low_t,
    dequant_ptr: *const i16,
    eob_ptr: *mut u16,
    _scan: *const i16,
    iscan: *const i16,
    qm_ptr: *const qm_val_t,
    iqm_ptr: *const qm_val_t,
) {
    let mut vwt: uint16x8_t = vdupq_n_u16(0);
    let mut viwt: uint16x8_t;
    let log_scale: i32 = 1;
    let zbins: [i32; 2] = [
        ROUND_POWER_OF_TWO!(*zbin_ptr as i32, log_scale),
        ROUND_POWER_OF_TWO!(*zbin_ptr.add(1) as i32, log_scale),
    ];

    core::ptr::write_bytes(qcoeff_ptr, 0, n_coeffs as usize);
    core::ptr::write_bytes(dqcoeff_ptr, 0, n_coeffs as usize);

    let zero: int16x8_t = vdupq_n_s16(0);
    let mut v_eobmax_76543210: int16x8_t = vreinterpretq_s16_u16(vceqq_s16(zero, zero));
    let v_log_scale: int16x8_t = v_eobmax_76543210;

    let mut vzbins: int16x8_t = vdupq_n_s16(zbins[1] as i16);
    let mut vround: int16x8_t =
        vdupq_n_s16(ROUND_POWER_OF_TWO!(*round_ptr.add(1) as i32, log_scale) as i16);
    let mut vdequant: int16x8_t = vdupq_n_s16(*dequant_ptr.add(1));
    let mut vquant: int16x8_t = vdupq_n_s16(*quant_ptr.add(1));
    let mut vquant_shift: int16x8_t = vdupq_n_s16(*quant_shift_ptr.add(1));

    let mut v_coeff: int16x8_t = load_tran_low_to_s16q(coeff_ptr);
    let mut v_coeff_sign: int16x8_t = vshrq_n_s16::<15>(v_coeff);
    let mut v_abs: int16x8_t = vabsq_s16(v_coeff);
    vzbins = vsetq_lane_s16::<0>(zbins[0] as i16, vzbins);
    let mut vcond: uint16x8_t;
    if qm_ptr.is_null() {
        vcond = vcgeq_s16(v_abs, vzbins);
    } else {
        vwt = vmovl_u8(vld1_u8(qm_ptr));
        vcond = vcgeq_s16(QM_MULL_SHIFT!(v_abs, vwt), vzbins);
    }
    let mut nz_check: u64 = vget_lane_u64::<0>(vreinterpret_u64_u8(vmovn_u16(vcond)));
    if nz_check != 0 {
        vround = vsetq_lane_s16::<0>(ROUND_POWER_OF_TWO!(*round_ptr as i32, log_scale) as i16, vround);
        vquant = vsetq_lane_s16::<0>(*quant_ptr, vquant);
        vdequant = vsetq_lane_s16::<0>(*dequant_ptr, vdequant);
        vquant_shift = vsetq_lane_s16::<0>(*quant_shift_ptr, vquant_shift);

        let vtmp: int16x8_t = vqaddq_s16(v_abs, vround);

        let mut vtmp2: int16x8_t;
        if qm_ptr.is_null() {
            vtmp2 = vsraq_n_s16::<1>(vtmp, vqdmulhq_s16(vtmp, vquant));
        } else {
            vtmp2 = QM_MULL_SHIFT!(vtmp, vwt);
            vtmp2 = vaddq_s16(vtmp2, vtmp);
        }

        vtmp2 = vqdmulhq_s16(vtmp2, vquant_shift);
        let mut vdest: int16x8_t = vsubq_s16(veorq_s16(vtmp2, v_coeff_sign), v_coeff_sign);
        let mut coeff_nz_mask: int16x8_t =
            vbslq_s16(vcond, vdest, load_tran_low_to_s16q(qcoeff_ptr));
        store_s16q_to_tran_low(qcoeff_ptr, coeff_nz_mask);

        if !iqm_ptr.is_null() {
            viwt = vmovl_u8(vld1_u8(iqm_ptr));
            vdequant = QM_MULL_SHIFT!(vdequant, viwt);
        }
        let v_deq_abs: int16x8_t = vreinterpretq_s16_u16(vshlq_u16(
            vreinterpretq_u16_s16(vmulq_s16(vtmp2, vdequant)),
            v_log_scale,
        ));
        vdest = vsubq_s16(veorq_s16(v_deq_abs, v_coeff_sign), v_coeff_sign);
        coeff_nz_mask = vbslq_s16(vcond, vdest, load_tran_low_to_s16q(dqcoeff_ptr));
        store_s16q_to_tran_low(dqcoeff_ptr, coeff_nz_mask);

        vzbins = vsetq_lane_s16::<0>(zbins[1] as i16, vzbins);
        vround =
            vsetq_lane_s16::<0>(ROUND_POWER_OF_TWO!(*round_ptr.add(1) as i32, log_scale) as i16, vround);
        vquant = vsetq_lane_s16::<0>(*quant_ptr.add(1), vquant);
        vdequant = vsetq_lane_s16::<0>(*dequant_ptr.add(1), vdequant);
        vquant_shift = vsetq_lane_s16::<0>(*quant_shift_ptr.add(1), vquant_shift);

        let vtmp_mask: uint16x8_t = vcgtq_s16(vtmp2, zero);
        let v_nz_mask: uint16x8_t = vandq_u16(vtmp_mask, vcond);
        let v_iscan: int16x8_t = vld1q_s16(iscan);
        vcond = vandq_u16(v_nz_mask, vcgtq_s16(v_iscan, v_eobmax_76543210));
        v_eobmax_76543210 = vbslq_s16(vcond, v_iscan, v_eobmax_76543210);
    }
    vzbins = vsetq_lane_s16::<0>(zbins[1] as i16, vzbins);

    let mut i: i32 = 8;
    while (i as isize) < n_coeffs {
        let i_ = i as isize;
        v_coeff = load_tran_low_to_s16q(coeff_ptr.offset(i_));
        v_coeff_sign = vshrq_n_s16::<15>(v_coeff);
        v_abs = vabsq_s16(v_coeff);

        if qm_ptr.is_null() {
            vcond = vcgeq_s16(v_abs, vzbins);
        } else {
            vwt = vmovl_u8(vld1_u8(qm_ptr.offset(i_)));
            vcond = vcgeq_s16(QM_MULL_SHIFT!(v_abs, vwt), vzbins);
        }
        nz_check = vget_lane_u64::<0>(vreinterpret_u64_u8(vmovn_u16(vcond)));
        if nz_check != 0 {
            let vtmp: int16x8_t = vqaddq_s16(v_abs, vround);

            let mut vtmp2: int16x8_t;
            if qm_ptr.is_null() {
                vtmp2 = vsraq_n_s16::<1>(vtmp, vqdmulhq_s16(vtmp, vquant));
            } else {
                vtmp2 = QM_MULL_SHIFT!(vtmp, vwt);
                vtmp2 = vaddq_s16(vtmp2, vtmp);
            }
            vtmp2 = vqdmulhq_s16(vtmp2, vquant_shift);

            let mut vdest: int16x8_t = vsubq_s16(veorq_s16(vtmp2, v_coeff_sign), v_coeff_sign);
            let mut coeff_nz_mask: int16x8_t =
                vbslq_s16(vcond, vdest, load_tran_low_to_s16q(qcoeff_ptr.offset(i_)));
            store_s16q_to_tran_low(qcoeff_ptr.offset(i_), coeff_nz_mask);

            if !iqm_ptr.is_null() {
                viwt = vmovl_u8(vld1_u8(iqm_ptr.offset(i_)));
                vdequant = QM_MULL_SHIFT!(vdequant, viwt);
            }
            let v_deq_abs: int16x8_t = vreinterpretq_s16_u16(vshlq_u16(
                vreinterpretq_u16_s16(vmulq_s16(vtmp2, vdequant)),
                v_log_scale,
            ));
            vdest = vsubq_s16(veorq_s16(v_deq_abs, v_coeff_sign), v_coeff_sign);
            coeff_nz_mask = vbslq_s16(vcond, vdest, load_tran_low_to_s16q(dqcoeff_ptr.offset(i_)));
            store_s16q_to_tran_low(dqcoeff_ptr.offset(i_), coeff_nz_mask);

            let vtmp_mask: uint16x8_t = vcgtq_s16(vtmp2, zero);
            let v_nz_mask: uint16x8_t = vandq_u16(vtmp_mask, vcond);
            let v_iscan: int16x8_t = vld1q_s16(iscan.offset(i_));
            vcond = vandq_u16(v_nz_mask, vcgtq_s16(v_iscan, v_eobmax_76543210));
            v_eobmax_76543210 = vbslq_s16(vcond, v_iscan, v_eobmax_76543210);
        }
        i += 8;
    }
    *eob_ptr = get_max_eob(v_eobmax_76543210);
}

unsafe fn aom_quantize_b_helper_64x64_neon(
    coeff_ptr: *const tran_low_t,
    n_coeffs: isize,
    zbin_ptr: *const i16,
    round_ptr: *const i16,
    quant_ptr: *const i16,
    quant_shift_ptr: *const i16,
    qcoeff_ptr: *mut tran_low_t,
    dqcoeff_ptr: *mut tran_low_t,
    dequant_ptr: *const i16,
    eob_ptr: *mut u16,
    _scan: *const i16,
    iscan: *const i16,
    qm_ptr: *const qm_val_t,
    iqm_ptr: *const qm_val_t,
) {
    let mut vwt: uint16x8_t = vdupq_n_u16(0);
    let mut viwt: uint16x8_t;
    let log_scale: i32 = 2;
    let v_log_scale: int16x8_t = vreinterpretq_s16_s64(vdupq_n_s64(0xFFFEFFFEFFFEFFFEu64 as i64));

    let zbins: [i32; 2] = [
        ROUND_POWER_OF_TWO!(*zbin_ptr as i32, log_scale),
        ROUND_POWER_OF_TWO!(*zbin_ptr.add(1) as i32, log_scale),
    ];

    core::ptr::write_bytes(qcoeff_ptr, 0, n_coeffs as usize);
    core::ptr::write_bytes(dqcoeff_ptr, 0, n_coeffs as usize);

    let zero: int16x8_t = vdupq_n_s16(0);
    let mut v_eobmax_76543210: int16x8_t = vreinterpretq_s16_u16(vceqq_s16(zero, zero));
    let v_ones: int16x8_t = vnegq_s16(v_eobmax_76543210);

    let mut vzbins: int16x8_t = vdupq_n_s16(zbins[1] as i16);
    let mut vround: int16x8_t =
        vdupq_n_s16(ROUND_POWER_OF_TWO!(*round_ptr.add(1) as i32, log_scale) as i16);
    let mut vdequant: int16x8_t = vdupq_n_s16(*dequant_ptr.add(1));
    let mut vquant: int16x8_t = vdupq_n_s16(*quant_ptr.add(1));
    let mut vquant_shift: int16x8_t = vdupq_n_s16(*quant_shift_ptr.add(1));

    let mut v_coeff: int16x8_t = load_tran_low_to_s16q(coeff_ptr);
    let mut v_coeff_sign: int16x8_t = vshrq_n_s16::<15>(v_coeff);
    let mut v_abs: int16x8_t = vabsq_s16(v_coeff);
    vzbins = vsetq_lane_s16::<0>(zbins[0] as i16, vzbins);
    let mut vcond: uint16x8_t;
    if qm_ptr.is_null() {
        vcond = vcgeq_s16(v_abs, vzbins);
    } else {
        vwt = vmovl_u8(vld1_u8(qm_ptr));
        vcond = vcgeq_s16(QM_MULL_SHIFT!(v_abs, vwt), vzbins);
    }
    let mut nz_check: u64 = vget_lane_u64::<0>(vreinterpret_u64_u8(vmovn_u16(vcond)));
    if nz_check != 0 {
        vround = vsetq_lane_s16::<0>(ROUND_POWER_OF_TWO!(*round_ptr as i32, log_scale) as i16, vround);
        vquant = vsetq_lane_s16::<0>(*quant_ptr, vquant);
        vdequant = vsetq_lane_s16::<0>(*dequant_ptr, vdequant);
        vquant_shift = vsetq_lane_s16::<0>(*quant_shift_ptr, vquant_shift);
        let vtmp: int16x8_t = vqaddq_s16(v_abs, vround);

        let mut vtmp2: int16x8_t;
        if qm_ptr.is_null() {
            vtmp2 = vsraq_n_s16::<1>(vtmp, vqdmulhq_s16(vtmp, vquant));
        } else {
            vtmp2 = QM_MULL_SHIFT!(vtmp, vwt);
            vtmp2 = vaddq_s16(vtmp2, vtmp);
        }

        let ones: int16x8_t = vandq_s16(vshrq_n_s16::<14>(vmulq_s16(vtmp2, vquant_shift)), v_ones);
        vtmp2 = vaddq_s16(vshlq_s16(vqdmulhq_s16(vtmp2, vquant_shift), v_ones), ones);
        let mut vdest: int16x8_t = vsubq_s16(veorq_s16(vtmp2, v_coeff_sign), v_coeff_sign);
        let mut coeff_nz_mask: int16x8_t =
            vbslq_s16(vcond, vdest, load_tran_low_to_s16q(qcoeff_ptr));
        store_s16q_to_tran_low(qcoeff_ptr, coeff_nz_mask);

        if !iqm_ptr.is_null() {
            viwt = vmovl_u8(vld1_u8(iqm_ptr));
            vdequant = QM_MULL_SHIFT!(vdequant, viwt);
        }
        let mut v_deq_abs: int16x8_t = vreinterpretq_s16_u16(vshlq_u16(
            vreinterpretq_u16_s16(vmulq_s16(vtmp2, vdequant)),
            v_log_scale,
        ));
        v_deq_abs = vorrq_s16(vshlq_n_s16::<13>(vqdmulhq_s16(vtmp2, vdequant)), v_deq_abs);
        vdest = vsubq_s16(veorq_s16(v_deq_abs, v_coeff_sign), v_coeff_sign);
        coeff_nz_mask = vbslq_s16(vcond, vdest, load_tran_low_to_s16q(dqcoeff_ptr));
        store_s16q_to_tran_low(dqcoeff_ptr, coeff_nz_mask);

        vround =
            vsetq_lane_s16::<0>(ROUND_POWER_OF_TWO!(*round_ptr.add(1) as i32, log_scale) as i16, vround);
        vquant = vsetq_lane_s16::<0>(*quant_ptr.add(1), vquant);
        vdequant = vsetq_lane_s16::<0>(*dequant_ptr.add(1), vdequant);
        vquant_shift = vsetq_lane_s16::<0>(*quant_shift_ptr.add(1), vquant_shift);

        let vtmp_mask: uint16x8_t = vcgtq_s16(vtmp2, zero);
        let v_nz_mask: uint16x8_t = vandq_u16(vtmp_mask, vcond);
        let v_iscan: int16x8_t = vld1q_s16(iscan);
        vcond = vandq_u16(v_nz_mask, vcgtq_s16(v_iscan, v_eobmax_76543210));
        v_eobmax_76543210 = vbslq_s16(vcond, v_iscan, v_eobmax_76543210);
    }
    vzbins = vsetq_lane_s16::<0>(zbins[1] as i16, vzbins);

    let mut i: i32 = 8;
    while (i as isize) < n_coeffs {
        let i_ = i as isize;
        v_coeff = load_tran_low_to_s16q(coeff_ptr.offset(i_));
        v_coeff_sign = vshrq_n_s16::<15>(v_coeff);
        v_abs = vabsq_s16(v_coeff);

        if qm_ptr.is_null() {
            vcond = vcgeq_s16(v_abs, vzbins);
        } else {
            vwt = vmovl_u8(vld1_u8(qm_ptr.offset(i_)));
            vcond = vcgeq_s16(QM_MULL_SHIFT!(v_abs, vwt), vzbins);
        }
        nz_check = vget_lane_u64::<0>(vreinterpret_u64_u8(vmovn_u16(vcond)));
        if nz_check != 0 {
            let vtmp: int16x8_t = vqaddq_s16(v_abs, vround);

            let mut vtmp2: int16x8_t;
            if qm_ptr.is_null() {
                vtmp2 = vsraq_n_s16::<1>(vtmp, vqdmulhq_s16(vtmp, vquant));
            } else {
                vtmp2 = QM_MULL_SHIFT!(vtmp, vwt);
                vtmp2 = vaddq_s16(vtmp2, vtmp);
            }

            let ones: int16x8_t =
                vandq_s16(vshrq_n_s16::<14>(vmulq_s16(vtmp2, vquant_shift)), v_ones);
            vtmp2 = vaddq_s16(vshlq_s16(vqdmulhq_s16(vtmp2, vquant_shift), v_ones), ones);
            let mut vdest: int16x8_t = vsubq_s16(veorq_s16(vtmp2, v_coeff_sign), v_coeff_sign);
            let mut coeff_nz_mask: int16x8_t =
                vbslq_s16(vcond, vdest, load_tran_low_to_s16q(qcoeff_ptr.offset(i_)));
            store_s16q_to_tran_low(qcoeff_ptr.offset(i_), coeff_nz_mask);

            if !iqm_ptr.is_null() {
                viwt = vmovl_u8(vld1_u8(iqm_ptr.offset(i_)));
                vdequant = QM_MULL_SHIFT!(vdequant, viwt);
            }
            let mut v_deq_abs: int16x8_t = vreinterpretq_s16_u16(vshlq_u16(
                vreinterpretq_u16_s16(vmulq_s16(vtmp2, vdequant)),
                v_log_scale,
            ));
            v_deq_abs = vorrq_s16(vshlq_n_s16::<13>(vqdmulhq_s16(vtmp2, vdequant)), v_deq_abs);
            vdest = vsubq_s16(veorq_s16(v_deq_abs, v_coeff_sign), v_coeff_sign);
            coeff_nz_mask = vbslq_s16(vcond, vdest, load_tran_low_to_s16q(dqcoeff_ptr.offset(i_)));
            store_s16q_to_tran_low(dqcoeff_ptr.offset(i_), coeff_nz_mask);

            let vtmp_mask: uint16x8_t = vcgtq_s16(vtmp2, zero);
            let v_nz_mask: uint16x8_t = vandq_u16(vtmp_mask, vcond);
            let v_iscan: int16x8_t = vld1q_s16(iscan.offset(i_));
            vcond = vandq_u16(v_nz_mask, vcgtq_s16(v_iscan, v_eobmax_76543210));
            v_eobmax_76543210 = vbslq_s16(vcond, v_iscan, v_eobmax_76543210);
        }
        i += 8;
    }
    *eob_ptr = get_max_eob(v_eobmax_76543210);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_quantize_b_helper_neon(
    coeff_ptr: *const tran_low_t,
    n_coeffs: isize,
    zbin_ptr: *const i16,
    round_ptr: *const i16,
    quant_ptr: *const i16,
    quant_shift_ptr: *const i16,
    qcoeff_ptr: *mut tran_low_t,
    dqcoeff_ptr: *mut tran_low_t,
    dequant_ptr: *const i16,
    eob_ptr: *mut u16,
    scan: *const i16,
    iscan: *const i16,
    qm_ptr: *const qm_val_t,
    iqm_ptr: *const qm_val_t,
    log_scale: i32,
) {
    match log_scale {
        // log_scale for AV1 encoder can be only 0, 1, 2
        0 => {
            aom_quantize_b_helper_16x16_neon(
                coeff_ptr, n_coeffs, zbin_ptr, round_ptr, quant_ptr, quant_shift_ptr, qcoeff_ptr,
                dqcoeff_ptr, dequant_ptr, eob_ptr, scan, iscan, qm_ptr, iqm_ptr,
            );
        }
        1 => {
            aom_quantize_b_helper_32x32_neon(
                coeff_ptr, n_coeffs, zbin_ptr, round_ptr, quant_ptr, quant_shift_ptr, qcoeff_ptr,
                dqcoeff_ptr, dequant_ptr, eob_ptr, scan, iscan, qm_ptr, iqm_ptr,
            );
        }
        2 => {
            aom_quantize_b_helper_64x64_neon(
                coeff_ptr, n_coeffs, zbin_ptr, round_ptr, quant_ptr, quant_shift_ptr, qcoeff_ptr,
                dqcoeff_ptr, dequant_ptr, eob_ptr, scan, iscan, qm_ptr, iqm_ptr,
            );
        }
        _ => {}
    }
}

#[inline(always)]
unsafe fn quantize_b_logscale1_8(
    coeff: int16x8_t,
    abs: int16x8_t,
    cond: uint16x8_t,
    round: int16x8_t,
    dequant: int16x8_t,
    quant: int16x8_t,
    quant_shift: int16x8_t,
    qcoeff_ptr: *mut tran_low_t,
    dqcoeff_ptr: *mut tran_low_t,
) -> uint16x8_t {
    let zero: int16x8_t = vdupq_n_s16(0);

    let coeff_sign: int16x8_t = vreinterpretq_s16_u16(vcltq_s16(coeff, zero));

    let mut tmp: int16x8_t = vqaddq_s16(abs, round);
    tmp = vsraq_n_s16::<1>(tmp, vqdmulhq_s16(tmp, quant));
    tmp = vqdmulhq_s16(tmp, quant_shift);

    let mut qcoeff: int16x8_t = vsubq_s16(veorq_s16(tmp, coeff_sign), coeff_sign);
    qcoeff = vbslq_s16(cond, qcoeff, zero);
    store_s16q_to_tran_low(qcoeff_ptr, qcoeff);

    // Shift by log_scale = 1.
    let mut dqcoeff: int16x8_t = vreinterpretq_s16_u16(vhaddq_u16(
        vreinterpretq_u16_s16(vmulq_s16(tmp, dequant)),
        vdupq_n_u16(0),
    ));
    dqcoeff = vsubq_s16(veorq_s16(dqcoeff, coeff_sign), coeff_sign);
    dqcoeff = vbslq_s16(cond, dqcoeff, zero);
    store_s16q_to_tran_low(dqcoeff_ptr, dqcoeff);

    let tmp_mask: uint16x8_t = vcgtq_s16(tmp, zero);
    let nz_mask: uint16x8_t = vandq_u16(tmp_mask, cond);

    nz_mask
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_quantize_b_32x32_neon(
    coeff_ptr: *const tran_low_t,
    n_coeffs: isize,
    zbin_ptr: *const i16,
    round_ptr: *const i16,
    quant_ptr: *const i16,
    quant_shift_ptr: *const i16,
    qcoeff_ptr: *mut tran_low_t,
    dqcoeff_ptr: *mut tran_low_t,
    dequant_ptr: *const i16,
    eob_ptr: *mut u16,
    _scan: *const i16,
    iscan: *const i16,
) {
    let log_scale: i32 = 1;
    let zbins: [i32; 2] = [
        ROUND_POWER_OF_TWO!(*zbin_ptr as i32, log_scale),
        ROUND_POWER_OF_TWO!(*zbin_ptr.add(1) as i32, log_scale),
    ];
    let rounds: [i32; 2] = [
        ROUND_POWER_OF_TWO!(*round_ptr as i32, log_scale),
        ROUND_POWER_OF_TWO!(*round_ptr.add(1) as i32, log_scale),
    ];

    let mut v_eobmax_76543210: int16x8_t = vdupq_n_s16(-1);

    let v_zbins: int16x8_t = vdupq_n_s16(zbins[1] as i16);
    let v_round: int16x8_t = vdupq_n_s16(rounds[1] as i16);
    let v_dequant: int16x8_t = vdupq_n_s16(*dequant_ptr.add(1));
    let v_quant: int16x8_t = vdupq_n_s16(*quant_ptr.add(1));
    let v_quant_shift: int16x8_t = vdupq_n_s16(*quant_shift_ptr.add(1));

    let v_zbins0: int16x8_t = vsetq_lane_s16::<0>(zbins[0] as i16, v_zbins);
    let mut v_coeff: int16x8_t = load_tran_low_to_s16q(coeff_ptr);
    let mut v_abs: int16x8_t = vabsq_s16(v_coeff);
    let mut v_cond: uint16x8_t = vcgeq_s16(v_abs, v_zbins0);

    let mut nz_check: u64 = vget_lane_u64::<0>(vreinterpret_u64_u8(vmovn_u16(v_cond)));
    if nz_check != 0 {
        let v_round0: int16x8_t = vsetq_lane_s16::<0>(rounds[0] as i16, v_round);
        let v_quant0: int16x8_t = vsetq_lane_s16::<0>(*quant_ptr, v_quant);
        let v_dequant0: int16x8_t = vsetq_lane_s16::<0>(*dequant_ptr, v_dequant);
        let v_quant_shift0: int16x8_t = vsetq_lane_s16::<0>(*quant_shift_ptr, v_quant_shift);

        let v_nz_mask: uint16x8_t = quantize_b_logscale1_8(
            v_coeff, v_abs, v_cond, v_round0, v_dequant0, v_quant0, v_quant_shift0, qcoeff_ptr,
            dqcoeff_ptr,
        );

        let v_iscan: int16x8_t = vld1q_s16(iscan);
        let v_eobmax: int16x8_t = vmaxq_s16(v_iscan, v_eobmax_76543210);
        v_eobmax_76543210 = vbslq_s16(v_nz_mask, v_eobmax, v_eobmax_76543210);
    } else {
        store_s16q_to_tran_low(qcoeff_ptr, vdupq_n_s16(0));
        store_s16q_to_tran_low(dqcoeff_ptr, vdupq_n_s16(0));
    }

    let mut i: i32 = 8;
    while (i as isize) < n_coeffs {
        v_coeff = load_tran_low_to_s16q(coeff_ptr.offset(i as isize));
        v_abs = vabsq_s16(v_coeff);
        v_cond = vcgeq_s16(v_abs, v_zbins);

        nz_check = vget_lane_u64::<0>(vreinterpret_u64_u8(vmovn_u16(v_cond)));
        if nz_check != 0 {
            let v_nz_mask: uint16x8_t = quantize_b_logscale1_8(
                v_coeff,
                v_abs,
                v_cond,
                v_round,
                v_dequant,
                v_quant,
                v_quant_shift,
                qcoeff_ptr.offset(i as isize),
                dqcoeff_ptr.offset(i as isize),
            );

            let v_iscan: int16x8_t = vld1q_s16(iscan.offset(i as isize));
            let v_eobmax: int16x8_t = vmaxq_s16(v_iscan, v_eobmax_76543210);
            v_eobmax_76543210 = vbslq_s16(v_nz_mask, v_eobmax, v_eobmax_76543210);
        } else {
            store_s16q_to_tran_low(qcoeff_ptr.offset(i as isize), vdupq_n_s16(0));
            store_s16q_to_tran_low(dqcoeff_ptr.offset(i as isize), vdupq_n_s16(0));
        }
        i += 8;
    }
    *eob_ptr = get_max_eob(v_eobmax_76543210);
}

#[inline(always)]
unsafe fn quantize_b_logscale2_8(
    coeff: int16x8_t,
    abs: int16x8_t,
    cond: uint16x8_t,
    round: int16x8_t,
    dequant: int16x8_t,
    quant: int16x8_t,
    quant_shift: int16x8_t,
    qcoeff_ptr: *mut tran_low_t,
    dqcoeff_ptr: *mut tran_low_t,
) -> uint16x8_t {
    let zero: int16x8_t = vdupq_n_s16(0);
    let one: int16x8_t = vdupq_n_s16(1);

    let coeff_sign: int16x8_t = vreinterpretq_s16_u16(vcltq_s16(coeff, zero));

    let mut tmp: int16x8_t = vqaddq_s16(abs, round);
    tmp = vsraq_n_s16::<1>(tmp, vqdmulhq_s16(tmp, quant));
    let ones: int16x8_t = vandq_s16(vshrq_n_s16::<14>(vmulq_s16(tmp, quant_shift)), one);
    tmp = vqdmulhq_s16(tmp, quant_shift);
    tmp = vaddq_s16(vshlq_s16(tmp, one), ones);

    let mut qcoeff: int16x8_t = vsubq_s16(veorq_s16(tmp, coeff_sign), coeff_sign);
    qcoeff = vbslq_s16(cond, qcoeff, zero);
    store_s16q_to_tran_low(qcoeff_ptr, qcoeff);

    // Shift right by log_scale = 2.
    let mut dqcoeff: int16x8_t =
        vreinterpretq_s16_u16(vshrq_n_u16::<2>(vreinterpretq_u16_s16(vmulq_s16(tmp, dequant))));
    dqcoeff = vorrq_s16(vshlq_n_s16::<13>(vqdmulhq_s16(tmp, dequant)), dqcoeff);
    dqcoeff = vsubq_s16(veorq_s16(dqcoeff, coeff_sign), coeff_sign);
    dqcoeff = vbslq_s16(cond, dqcoeff, zero);
    store_s16q_to_tran_low(dqcoeff_ptr, dqcoeff);

    let tmp_mask: uint16x8_t = vcgtq_s16(tmp, zero);
    let nz_mask: uint16x8_t = vandq_u16(tmp_mask, cond);

    nz_mask
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_quantize_b_64x64_neon(
    coeff_ptr: *const tran_low_t,
    n_coeffs: isize,
    zbin_ptr: *const i16,
    round_ptr: *const i16,
    quant_ptr: *const i16,
    quant_shift_ptr: *const i16,
    qcoeff_ptr: *mut tran_low_t,
    dqcoeff_ptr: *mut tran_low_t,
    dequant_ptr: *const i16,
    eob_ptr: *mut u16,
    _scan: *const i16,
    iscan: *const i16,
) {
    let log_scale: i32 = 2;
    let zbins: [i32; 2] = [
        ROUND_POWER_OF_TWO!(*zbin_ptr as i32, log_scale),
        ROUND_POWER_OF_TWO!(*zbin_ptr.add(1) as i32, log_scale),
    ];
    let rounds: [i32; 2] = [
        ROUND_POWER_OF_TWO!(*round_ptr as i32, log_scale),
        ROUND_POWER_OF_TWO!(*round_ptr.add(1) as i32, log_scale),
    ];

    let mut v_eobmax_76543210: int16x8_t = vdupq_n_s16(-1);

    let v_zbins: int16x8_t = vdupq_n_s16(zbins[1] as i16);
    let v_round: int16x8_t = vdupq_n_s16(rounds[1] as i16);
    let v_dequant: int16x8_t = vdupq_n_s16(*dequant_ptr.add(1));
    let v_quant: int16x8_t = vdupq_n_s16(*quant_ptr.add(1));
    let v_quant_shift: int16x8_t = vdupq_n_s16(*quant_shift_ptr.add(1));

    let v_zbins0: int16x8_t = vsetq_lane_s16::<0>(zbins[0] as i16, v_zbins);
    let mut v_coeff: int16x8_t = load_tran_low_to_s16q(coeff_ptr);
    let mut v_abs: int16x8_t = vabsq_s16(v_coeff);
    let mut v_cond: uint16x8_t = vcgeq_s16(v_abs, v_zbins0);

    let mut nz_check: u64 = vget_lane_u64::<0>(vreinterpret_u64_u8(vmovn_u16(v_cond)));
    if nz_check != 0 {
        let v_round0: int16x8_t = vsetq_lane_s16::<0>(rounds[0] as i16, v_round);
        let v_quant0: int16x8_t = vsetq_lane_s16::<0>(*quant_ptr, v_quant);
        let v_dequant0: int16x8_t = vsetq_lane_s16::<0>(*dequant_ptr, v_dequant);
        let v_quant_shift0: int16x8_t = vsetq_lane_s16::<0>(*quant_shift_ptr, v_quant_shift);

        let v_nz_mask: uint16x8_t = quantize_b_logscale2_8(
            v_coeff, v_abs, v_cond, v_round0, v_dequant0, v_quant0, v_quant_shift0, qcoeff_ptr,
            dqcoeff_ptr,
        );

        let v_iscan: int16x8_t = vld1q_s16(iscan);
        let v_eobmax: int16x8_t = vmaxq_s16(v_iscan, v_eobmax_76543210);
        v_eobmax_76543210 = vbslq_s16(v_nz_mask, v_eobmax, v_eobmax_76543210);
    } else {
        store_s16q_to_tran_low(qcoeff_ptr, vdupq_n_s16(0));
        store_s16q_to_tran_low(dqcoeff_ptr, vdupq_n_s16(0));
    }

    let mut i: i32 = 8;
    while (i as isize) < n_coeffs {
        v_coeff = load_tran_low_to_s16q(coeff_ptr.offset(i as isize));
        v_abs = vabsq_s16(v_coeff);
        v_cond = vcgeq_s16(v_abs, v_zbins);

        nz_check = vget_lane_u64::<0>(vreinterpret_u64_u8(vmovn_u16(v_cond)));
        if nz_check != 0 {
            let v_nz_mask: uint16x8_t = quantize_b_logscale2_8(
                v_coeff,
                v_abs,
                v_cond,
                v_round,
                v_dequant,
                v_quant,
                v_quant_shift,
                qcoeff_ptr.offset(i as isize),
                dqcoeff_ptr.offset(i as isize),
            );

            let v_iscan: int16x8_t = vld1q_s16(iscan.offset(i as isize));
            let v_eobmax: int16x8_t = vmaxq_s16(v_iscan, v_eobmax_76543210);
            v_eobmax_76543210 = vbslq_s16(v_nz_mask, v_eobmax, v_eobmax_76543210);
        } else {
            store_s16q_to_tran_low(qcoeff_ptr.offset(i as isize), vdupq_n_s16(0));
            store_s16q_to_tran_low(dqcoeff_ptr.offset(i as isize), vdupq_n_s16(0));
        }
        i += 8;
    }
    *eob_ptr = get_max_eob(v_eobmax_76543210);
}
