/*
 * Copyright (c) 2022, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

// Translated by hand from upstream/aom/av1/encoder/arm/av1_highbd_quantize_neon.c
// (ADR 0056). Intrinsic sequences, helper structure, and names follow the
// original; only the AOM_ARCH_AARCH64 branches are retained.
use core::arch::aarch64::*;

use crate::simd::types::*;

#[inline(always)]
unsafe fn quantize_4(
    coeff_ptr: *const tran_low_t,
    qcoeff_ptr: *mut tran_low_t,
    dqcoeff_ptr: *mut tran_low_t,
    v_quant_s32: int32x4_t,
    v_dequant_s32: int32x4_t,
    v_round_s32: int32x4_t,
    log_scale: i32,
) -> uint16x4_t {
    let v_coeff = vld1q_s32(coeff_ptr);
    let v_coeff_sign = vreinterpretq_s32_u32(vcltq_s32(v_coeff, vdupq_n_s32(0)));
    let v_log_scale = vdupq_n_s32(log_scale);
    let v_abs_coeff = vabsq_s32(v_coeff);
    // ((abs_coeff << (1 + log_scale)) >= dequant_ptr[rc01])
    let v_abs_coeff_scaled = vshlq_s32(v_abs_coeff, vdupq_n_s32(1 + log_scale));
    let v_mask = vcgeq_s32(v_abs_coeff_scaled, v_dequant_s32);
    // const int64_t tmp = vmask ? (int64_t)abs_coeff + log_scaled_round : 0
    let v_tmp = vandq_s32(vaddq_s32(v_abs_coeff, v_round_s32), vreinterpretq_s32_u32(v_mask));
    //  const int abs_qcoeff = (int)((tmp * quant) >> (16 - log_scale));
    let v_abs_qcoeff = vqdmulhq_s32(vshlq_s32(v_tmp, v_log_scale), v_quant_s32);
    //  qcoeff_ptr[rc] = (tran_low_t)((abs_qcoeff ^ coeff_sign) - coeff_sign);
    let v_qcoeff = vsubq_s32(veorq_s32(v_abs_qcoeff, v_coeff_sign), v_coeff_sign);
    // vshlq_s32 will shift right if shift value is negative.
    let v_abs_dqcoeff = vshlq_s32(vmulq_s32(v_abs_qcoeff, v_dequant_s32), vnegq_s32(v_log_scale));
    //  dqcoeff_ptr[rc] = (tran_low_t)((abs_dqcoeff ^ coeff_sign) - coeff_sign);
    let v_dqcoeff = vsubq_s32(veorq_s32(v_abs_dqcoeff, v_coeff_sign), v_coeff_sign);

    vst1q_s32(qcoeff_ptr, v_qcoeff);
    vst1q_s32(dqcoeff_ptr, v_dqcoeff);

    // Used to find eob.
    let nz_qcoeff_mask = vcgtq_s32(v_abs_qcoeff, vdupq_n_s32(0));
    vmovn_u32(nz_qcoeff_mask)
}

#[inline(always)]
unsafe fn get_max_lane_eob(iscan: *const i16, v_eobmax: int16x8_t, v_mask: uint16x8_t) -> int16x8_t {
    let v_iscan = vld1q_s16(iscan);
    let v_nz_iscan = vbslq_s16(v_mask, v_iscan, vdupq_n_s16(-1));
    vmaxq_s16(v_eobmax, v_nz_iscan)
}

#[inline(always)]
unsafe fn get_max_eob(v_eobmax: int16x8_t) -> u16 {
    let max_val: i16 = vmaxvq_s16(v_eobmax);
    (max_val as u16).wrapping_add(1)
}

#[no_mangle]
pub unsafe extern "C-unwind" fn av1_highbd_quantize_fp_neon(
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
    log_scale: i32,
) {
    let v_quant = vld1_s16(quant_ptr);
    let v_dequant = vld1_s16(dequant_ptr);
    let v_zero = vdup_n_s16(0);
    let v_round_select = vcgt_s16(vdup_n_s16(log_scale as i16), v_zero);
    let v_round_no_scale = vld1_s16(round_ptr);
    let v_round_log_scale = vqrdmulh_n_s16(v_round_no_scale, (1i32 << (15 - log_scale)) as i16);
    let v_round = vbsl_s16(v_round_select, v_round_log_scale, v_round_no_scale);
    let mut v_round_s32 = vaddl_s16(v_round, v_zero);
    let mut v_quant_s32 = vshlq_n_s32::<15>(vaddl_s16(v_quant, v_zero));
    let mut v_dequant_s32 = vaddl_s16(v_dequant, v_zero);
    let mut v_mask_lo: uint16x4_t;
    let mut v_mask_hi: uint16x4_t;
    let mut v_eobmax = vdupq_n_s16(-1);

    // DC and first 3 AC
    v_mask_lo = quantize_4(coeff_ptr, qcoeff_ptr, dqcoeff_ptr, v_quant_s32, v_dequant_s32,
                           v_round_s32, log_scale);

    // overwrite the DC constants with AC constants
    v_round_s32 = vdupq_lane_s32::<1>(vget_low_s32(v_round_s32));
    v_quant_s32 = vdupq_lane_s32::<1>(vget_low_s32(v_quant_s32));
    v_dequant_s32 = vdupq_lane_s32::<1>(vget_low_s32(v_dequant_s32));

    // 4 more AC
    v_mask_hi = quantize_4(coeff_ptr.add(4), qcoeff_ptr.add(4), dqcoeff_ptr.add(4), v_quant_s32,
                           v_dequant_s32, v_round_s32, log_scale);

    // Find the max lane eob for the first 8 coeffs.
    v_eobmax = get_max_lane_eob(iscan, v_eobmax, vcombine_u16(v_mask_lo, v_mask_hi));

    count -= 8;
    loop {
        coeff_ptr = coeff_ptr.add(8);
        qcoeff_ptr = qcoeff_ptr.add(8);
        dqcoeff_ptr = dqcoeff_ptr.add(8);
        iscan = iscan.add(8);
        v_mask_lo = quantize_4(coeff_ptr, qcoeff_ptr, dqcoeff_ptr, v_quant_s32, v_dequant_s32,
                               v_round_s32, log_scale);
        v_mask_hi = quantize_4(coeff_ptr.add(4), qcoeff_ptr.add(4), dqcoeff_ptr.add(4),
                               v_quant_s32, v_dequant_s32, v_round_s32, log_scale);
        // Find the max lane eob for 8 coeffs.
        v_eobmax = get_max_lane_eob(iscan, v_eobmax, vcombine_u16(v_mask_lo, v_mask_hi));
        count -= 8;
        if count == 0 {
            break;
        }
    }

    *eob_ptr = get_max_eob(v_eobmax);
}
