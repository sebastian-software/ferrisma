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

// Translated by hand from upstream/aom/aom_dsp/arm/highbd_quantize_neon.c
// (ADR 0056). Only the AOM_ARCH_AARCH64 branches are retained. The selected
// configuration has CONFIG_REALTIME_ONLY=0 and SKIP_EOB_FACTOR_ADJUST set.
use core::arch::aarch64::*;

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

#[inline(always)]
unsafe fn sum_abs_coeff(a: uint32x4_t) -> u32 {
    vaddvq_u32(a)
}

#[inline(always)]
#[allow(clippy::too_many_arguments)]
unsafe fn quantize_4(
    coeff_ptr: *const tran_low_t,
    qcoeff_ptr: *mut tran_low_t,
    dqcoeff_ptr: *mut tran_low_t,
    v_quant_s32: int32x4_t,
    v_dequant_s32: int32x4_t,
    v_round_s32: int32x4_t,
    v_zbin_s32: int32x4_t,
    v_quant_shift_s32: int32x4_t,
    log_scale: i32,
) -> uint16x4_t {
    let v_coeff = vld1q_s32(coeff_ptr);
    let v_coeff_sign = vreinterpretq_s32_u32(vcltq_s32(v_coeff, vdupq_n_s32(0)));
    let v_abs_coeff = vabsq_s32(v_coeff);
    // if (abs_coeff < zbins[rc != 0]),
    let v_zbin_mask = vcgeq_s32(v_abs_coeff, v_zbin_s32);
    let v_log_scale = vdupq_n_s32(log_scale);
    // const int64_t tmp = (int64_t)abs_coeff + log_scaled_round;
    let v_tmp = vaddq_s32(v_abs_coeff, v_round_s32);
    //  const int32_t tmpw32 = tmp * wt;
    let v_tmpw32 = vmulq_s32(v_tmp, vdupq_n_s32(1 << AOM_QM_BITS));
    //  const int32_t tmp2 = (int32_t)((tmpw32 * quant64) >> 16);
    let v_tmp2 = vqdmulhq_s32(v_tmpw32, v_quant_s32);
    // const int32_t tmp3 =
    //    ((((tmp2 + tmpw32)<< log_scale) * (int64_t)(quant_shift << 15)) >> 32);
    let v_tmp3 = vqdmulhq_s32(vshlq_s32(vaddq_s32(v_tmp2, v_tmpw32), v_log_scale),
                              v_quant_shift_s32);
    // const int abs_qcoeff = vmask ? (int)tmp3 >> AOM_QM_BITS : 0;
    let v_abs_qcoeff = vandq_s32(vreinterpretq_s32_u32(v_zbin_mask),
                                 vshrq_n_s32::<AOM_QM_BITS>(v_tmp3));
    // const tran_low_t abs_dqcoeff = (abs_qcoeff * dequant_iwt) >> log_scale;
    // vshlq_s32 will shift right if shift value is negative.
    let v_abs_dqcoeff = vshlq_s32(vmulq_s32(v_abs_qcoeff, v_dequant_s32), vnegq_s32(v_log_scale));
    //  qcoeff_ptr[rc] = (tran_low_t)((abs_qcoeff ^ coeff_sign) - coeff_sign);
    let v_qcoeff = vsubq_s32(veorq_s32(v_abs_qcoeff, v_coeff_sign), v_coeff_sign);
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
    let v_iscan = vld1q_s16(iscan.add(0));
    let v_iscan_plus1 = vaddq_s16(v_iscan, vdupq_n_s16(1));
    let v_nz_iscan = vbslq_s16(v_mask, v_iscan_plus1, vdupq_n_s16(0));
    vmaxq_s16(v_eobmax, v_nz_iscan)
}

#[inline(always)]
unsafe fn get_min_max_lane_eob(
    iscan: *const i16,
    v_eobmin: *mut int16x8_t,
    v_eobmax: *mut int16x8_t,
    v_mask: uint16x8_t,
    n_coeffs: isize,
) {
    let v_iscan = vld1q_s16(iscan.add(0));
    let v_nz_iscan_max = vbslq_s16(v_mask, v_iscan, vdupq_n_s16(-1));
    let v_nz_iscan_min = vbslq_s16(v_mask, v_iscan, vdupq_n_s16(n_coeffs as i16));
    *v_eobmin = vminq_s16(*v_eobmin, v_nz_iscan_min);
    *v_eobmax = vmaxq_s16(*v_eobmax, v_nz_iscan_max);
}

#[inline(always)]
unsafe fn get_max_eob(v_eobmax: int16x8_t) -> u16 {
    vmaxvq_s16(v_eobmax) as u16
}

#[inline(always)]
unsafe fn get_min_eob(v_eobmin: int16x8_t) -> u16 {
    vminvq_s16(v_eobmin) as u16
}

#[allow(clippy::too_many_arguments)]
unsafe fn highbd_quantize_b_neon(
    mut coeff_ptr: *const tran_low_t,
    n_coeffs: isize,
    zbin_ptr: *const i16,
    round_ptr: *const i16,
    quant_ptr: *const i16,
    quant_shift_ptr: *const i16,
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
    let v_quant_shift = vld1_s16(quant_shift_ptr);
    let v_zbin_no_scale = vld1_s16(zbin_ptr);
    let v_zbin_log_scale = vqrdmulh_n_s16(v_zbin_no_scale, (1i32 << (15 - log_scale)) as i16);
    let v_zbin = vbsl_s16(v_round_select, v_zbin_log_scale, v_zbin_no_scale);
    let mut v_round_s32 = vmovl_s16(v_round);
    let mut v_quant_s32 = vshlq_n_s32::<15>(vmovl_s16(v_quant));
    let mut v_dequant_s32 = vmovl_s16(v_dequant);
    let mut v_quant_shift_s32 = vshlq_n_s32::<15>(vmovl_s16(v_quant_shift));
    let mut v_zbin_s32 = vmovl_s16(v_zbin);
    let mut v_mask_lo: uint16x4_t;
    let mut v_mask_hi: uint16x4_t;
    let mut v_eobmax = vdupq_n_s16(-1);

    let mut non_zero_count: isize = n_coeffs;

    debug_assert!(n_coeffs > 8);
    // Pre-scan pass
    let v_zbin_s32x = vdupq_lane_s32::<1>(vget_low_s32(v_zbin_s32));
    let mut i: isize = n_coeffs;
    loop {
        let v_coeff_a = vld1q_s32(coeff_ptr.offset(i - 4));
        let v_coeff_b = vld1q_s32(coeff_ptr.offset(i - 8));
        let v_abs_coeff_a = vabsq_s32(v_coeff_a);
        let v_abs_coeff_b = vabsq_s32(v_coeff_b);
        let v_mask_a = vcgeq_s32(v_abs_coeff_a, v_zbin_s32x);
        let v_mask_b = vcgeq_s32(v_abs_coeff_b, v_zbin_s32x);
        // If the coefficient is in the base ZBIN range, then discard.
        if sum_abs_coeff(v_mask_a).wrapping_add(sum_abs_coeff(v_mask_b)) == 0 {
            non_zero_count -= 8;
        } else {
            break;
        }
        i -= 8;
        if i <= 0 {
            break;
        }
    }

    let remaining_zcoeffs: isize = n_coeffs - non_zero_count;
    core::ptr::write_bytes(qcoeff_ptr.offset(non_zero_count), 0, remaining_zcoeffs as usize);
    core::ptr::write_bytes(dqcoeff_ptr.offset(non_zero_count), 0, remaining_zcoeffs as usize);

    // DC and first 3 AC
    v_mask_lo = quantize_4(coeff_ptr, qcoeff_ptr, dqcoeff_ptr, v_quant_s32, v_dequant_s32,
                           v_round_s32, v_zbin_s32, v_quant_shift_s32, log_scale);

    // overwrite the DC constants with AC constants
    v_round_s32 = vdupq_lane_s32::<1>(vget_low_s32(v_round_s32));
    v_quant_s32 = vdupq_lane_s32::<1>(vget_low_s32(v_quant_s32));
    v_dequant_s32 = vdupq_lane_s32::<1>(vget_low_s32(v_dequant_s32));
    v_quant_shift_s32 = vdupq_lane_s32::<1>(vget_low_s32(v_quant_shift_s32));
    v_zbin_s32 = vdupq_lane_s32::<1>(vget_low_s32(v_zbin_s32));

    // 4 more AC
    v_mask_hi = quantize_4(coeff_ptr.add(4), qcoeff_ptr.add(4), dqcoeff_ptr.add(4), v_quant_s32,
                           v_dequant_s32, v_round_s32, v_zbin_s32, v_quant_shift_s32, log_scale);

    v_eobmax = get_max_lane_eob(iscan, v_eobmax, vcombine_u16(v_mask_lo, v_mask_hi));

    let mut count: isize = non_zero_count - 8;
    while count > 0 {
        coeff_ptr = coeff_ptr.add(8);
        qcoeff_ptr = qcoeff_ptr.add(8);
        dqcoeff_ptr = dqcoeff_ptr.add(8);
        iscan = iscan.add(8);
        v_mask_lo = quantize_4(coeff_ptr, qcoeff_ptr, dqcoeff_ptr, v_quant_s32, v_dequant_s32,
                               v_round_s32, v_zbin_s32, v_quant_shift_s32, log_scale);
        v_mask_hi = quantize_4(coeff_ptr.add(4), qcoeff_ptr.add(4), dqcoeff_ptr.add(4),
                               v_quant_s32, v_dequant_s32, v_round_s32, v_zbin_s32,
                               v_quant_shift_s32, log_scale);
        // Find the max lane eob for 8 coeffs.
        v_eobmax = get_max_lane_eob(iscan, v_eobmax, vcombine_u16(v_mask_lo, v_mask_hi));
        count -= 8;
    }

    *eob_ptr = get_max_eob(v_eobmax);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_quantize_b_neon(
    coeff_ptr: *const tran_low_t, n_coeffs: isize, zbin_ptr: *const i16, round_ptr: *const i16,
    quant_ptr: *const i16, quant_shift_ptr: *const i16, qcoeff_ptr: *mut tran_low_t,
    dqcoeff_ptr: *mut tran_low_t, dequant_ptr: *const i16, eob_ptr: *mut u16, scan: *const i16,
    iscan: *const i16,
) {
    highbd_quantize_b_neon(coeff_ptr, n_coeffs, zbin_ptr, round_ptr, quant_ptr, quant_shift_ptr,
                           qcoeff_ptr, dqcoeff_ptr, dequant_ptr, eob_ptr, scan, iscan, 0);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_quantize_b_32x32_neon(
    coeff_ptr: *const tran_low_t, n_coeffs: isize, zbin_ptr: *const i16, round_ptr: *const i16,
    quant_ptr: *const i16, quant_shift_ptr: *const i16, qcoeff_ptr: *mut tran_low_t,
    dqcoeff_ptr: *mut tran_low_t, dequant_ptr: *const i16, eob_ptr: *mut u16, scan: *const i16,
    iscan: *const i16,
) {
    highbd_quantize_b_neon(coeff_ptr, n_coeffs, zbin_ptr, round_ptr, quant_ptr, quant_shift_ptr,
                           qcoeff_ptr, dqcoeff_ptr, dequant_ptr, eob_ptr, scan, iscan, 1);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_quantize_b_64x64_neon(
    coeff_ptr: *const tran_low_t, n_coeffs: isize, zbin_ptr: *const i16, round_ptr: *const i16,
    quant_ptr: *const i16, quant_shift_ptr: *const i16, qcoeff_ptr: *mut tran_low_t,
    dqcoeff_ptr: *mut tran_low_t, dequant_ptr: *const i16, eob_ptr: *mut u16, scan: *const i16,
    iscan: *const i16,
) {
    highbd_quantize_b_neon(coeff_ptr, n_coeffs, zbin_ptr, round_ptr, quant_ptr, quant_shift_ptr,
                           qcoeff_ptr, dqcoeff_ptr, dequant_ptr, eob_ptr, scan, iscan, 2);
}

#[allow(clippy::too_many_arguments)]
unsafe fn highbd_quantize_b_adaptive_neon(
    mut coeff_ptr: *const tran_low_t,
    n_coeffs: isize,
    zbin_ptr: *const i16,
    round_ptr: *const i16,
    quant_ptr: *const i16,
    quant_shift_ptr: *const i16,
    mut qcoeff_ptr: *mut tran_low_t,
    mut dqcoeff_ptr: *mut tran_low_t,
    dequant_ptr: *const i16,
    eob_ptr: *mut u16,
    scan: *const i16,
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
    let v_quant_shift = vld1_s16(quant_shift_ptr);
    let v_zbin_no_scale = vld1_s16(zbin_ptr);
    let v_zbin_log_scale = vqrdmulh_n_s16(v_zbin_no_scale, (1i32 << (15 - log_scale)) as i16);
    let v_zbin = vbsl_s16(v_round_select, v_zbin_log_scale, v_zbin_no_scale);
    let mut v_round_s32 = vmovl_s16(v_round);
    let mut v_quant_s32 = vshlq_n_s32::<15>(vmovl_s16(v_quant));
    let mut v_dequant_s32 = vmovl_s16(v_dequant);
    let mut v_quant_shift_s32 = vshlq_n_s32::<15>(vmovl_s16(v_quant_shift));
    let mut v_zbin_s32 = vmovl_s16(v_zbin);
    let mut v_mask_lo: uint16x4_t;
    let mut v_mask_hi: uint16x4_t;
    let mut v_eobmax = vdupq_n_s16(-1);
    let mut v_eobmin = vdupq_n_s16(n_coeffs as i16);

    debug_assert!(n_coeffs > 8);
    // Pre-scan pass
    let v_zbin_s32x = vdupq_lane_s32::<1>(vget_low_s32(v_zbin_s32));
    let prescan_add_1: i32 =
        ROUND_POWER_OF_TWO!(*dequant_ptr.add(1) as i32 * EOB_FACTOR, 7 + AOM_QM_BITS);
    let v_zbin_prescan = vaddq_s32(v_zbin_s32x, vdupq_n_s32(prescan_add_1));
    let mut non_zero_count: isize = n_coeffs;
    let mut i: isize = n_coeffs;
    loop {
        let v_coeff_a = vld1q_s32(coeff_ptr.offset(i - 4));
        let v_coeff_b = vld1q_s32(coeff_ptr.offset(i - 8));
        let v_abs_coeff_a = vabsq_s32(v_coeff_a);
        let v_abs_coeff_b = vabsq_s32(v_coeff_b);
        let v_mask_a = vcgeq_s32(v_abs_coeff_a, v_zbin_prescan);
        let v_mask_b = vcgeq_s32(v_abs_coeff_b, v_zbin_prescan);
        // If the coefficient is in the base ZBIN range, then discard.
        if sum_abs_coeff(v_mask_a).wrapping_add(sum_abs_coeff(v_mask_b)) == 0 {
            non_zero_count -= 8;
        } else {
            break;
        }
        i -= 8;
        if i <= 0 {
            break;
        }
    }

    let remaining_zcoeffs: isize = n_coeffs - non_zero_count;
    core::ptr::write_bytes(qcoeff_ptr.offset(non_zero_count), 0, remaining_zcoeffs as usize);
    core::ptr::write_bytes(dqcoeff_ptr.offset(non_zero_count), 0, remaining_zcoeffs as usize);

    // DC and first 3 AC
    v_mask_lo = quantize_4(coeff_ptr, qcoeff_ptr, dqcoeff_ptr, v_quant_s32, v_dequant_s32,
                           v_round_s32, v_zbin_s32, v_quant_shift_s32, log_scale);

    // overwrite the DC constants with AC constants
    v_round_s32 = vdupq_lane_s32::<1>(vget_low_s32(v_round_s32));
    v_quant_s32 = vdupq_lane_s32::<1>(vget_low_s32(v_quant_s32));
    v_dequant_s32 = vdupq_lane_s32::<1>(vget_low_s32(v_dequant_s32));
    v_quant_shift_s32 = vdupq_lane_s32::<1>(vget_low_s32(v_quant_shift_s32));
    v_zbin_s32 = vdupq_lane_s32::<1>(vget_low_s32(v_zbin_s32));

    // 4 more AC
    v_mask_hi = quantize_4(coeff_ptr.add(4), qcoeff_ptr.add(4), dqcoeff_ptr.add(4), v_quant_s32,
                           v_dequant_s32, v_round_s32, v_zbin_s32, v_quant_shift_s32, log_scale);

    get_min_max_lane_eob(iscan, &mut v_eobmin, &mut v_eobmax, vcombine_u16(v_mask_lo, v_mask_hi),
                         n_coeffs);

    let mut count: isize = non_zero_count - 8;
    while count > 0 {
        coeff_ptr = coeff_ptr.add(8);
        qcoeff_ptr = qcoeff_ptr.add(8);
        dqcoeff_ptr = dqcoeff_ptr.add(8);
        iscan = iscan.add(8);
        v_mask_lo = quantize_4(coeff_ptr, qcoeff_ptr, dqcoeff_ptr, v_quant_s32, v_dequant_s32,
                               v_round_s32, v_zbin_s32, v_quant_shift_s32, log_scale);
        v_mask_hi = quantize_4(coeff_ptr.add(4), qcoeff_ptr.add(4), dqcoeff_ptr.add(4),
                               v_quant_s32, v_dequant_s32, v_round_s32, v_zbin_s32,
                               v_quant_shift_s32, log_scale);

        get_min_max_lane_eob(iscan, &mut v_eobmin, &mut v_eobmax,
                             vcombine_u16(v_mask_lo, v_mask_hi), n_coeffs);
        count -= 8;
    }

    // `int eob = get_max_eob(v_eobmax);` converts the uint16_t result to int.
    let mut eob: i32 = get_max_eob(v_eobmax) as i32;

    let first: i32 = get_min_eob(v_eobmin) as i32;
    if eob >= 0 && first == eob {
        let rc: i32 = *scan.offset(eob as isize) as i32;
        if *qcoeff_ptr.offset(rc as isize) == 1 || *qcoeff_ptr.offset(rc as isize) == -1 {
            let zbins: [i32; 2] = [ROUND_POWER_OF_TWO!(*zbin_ptr.add(0) as i32, log_scale),
                                   ROUND_POWER_OF_TWO!(*zbin_ptr.add(1) as i32, log_scale)];
            let nzbins: [i32; 2] = [zbins[0] * -1, zbins[1] * -1];
            let wt: qm_val_t = (1 << AOM_QM_BITS) as qm_val_t;
            let coeff: i32 = *coeff_ptr.offset(rc as isize) * wt as i32;
            let factor: i32 = EOB_FACTOR + SKIP_EOB_FACTOR_ADJUST;
            let prescan_add_val: i32 =
                ROUND_POWER_OF_TWO!(*dequant_ptr.add((rc != 0) as usize) as i32 * factor, 7);
            if coeff < (zbins[(rc != 0) as usize] * (1 << AOM_QM_BITS) + prescan_add_val)
                && coeff > (nzbins[(rc != 0) as usize] * (1 << AOM_QM_BITS) - prescan_add_val)
            {
                *qcoeff_ptr.offset(rc as isize) = 0;
                *dqcoeff_ptr.offset(rc as isize) = 0;
                eob = -1;
            }
        }
    }
    *eob_ptr = (eob + 1) as u16;
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_quantize_b_adaptive_neon(
    coeff_ptr: *const tran_low_t, n_coeffs: isize, zbin_ptr: *const i16, round_ptr: *const i16,
    quant_ptr: *const i16, quant_shift_ptr: *const i16, qcoeff_ptr: *mut tran_low_t,
    dqcoeff_ptr: *mut tran_low_t, dequant_ptr: *const i16, eob_ptr: *mut u16, scan: *const i16,
    iscan: *const i16,
) {
    highbd_quantize_b_adaptive_neon(coeff_ptr, n_coeffs, zbin_ptr, round_ptr, quant_ptr,
                                    quant_shift_ptr, qcoeff_ptr, dqcoeff_ptr, dequant_ptr,
                                    eob_ptr, scan, iscan, 0);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_quantize_b_32x32_adaptive_neon(
    coeff_ptr: *const tran_low_t, n_coeffs: isize, zbin_ptr: *const i16, round_ptr: *const i16,
    quant_ptr: *const i16, quant_shift_ptr: *const i16, qcoeff_ptr: *mut tran_low_t,
    dqcoeff_ptr: *mut tran_low_t, dequant_ptr: *const i16, eob_ptr: *mut u16, scan: *const i16,
    iscan: *const i16,
) {
    highbd_quantize_b_adaptive_neon(coeff_ptr, n_coeffs, zbin_ptr, round_ptr, quant_ptr,
                                    quant_shift_ptr, qcoeff_ptr, dqcoeff_ptr, dequant_ptr,
                                    eob_ptr, scan, iscan, 1);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_quantize_b_64x64_adaptive_neon(
    coeff_ptr: *const tran_low_t, n_coeffs: isize, zbin_ptr: *const i16, round_ptr: *const i16,
    quant_ptr: *const i16, quant_shift_ptr: *const i16, qcoeff_ptr: *mut tran_low_t,
    dqcoeff_ptr: *mut tran_low_t, dequant_ptr: *const i16, eob_ptr: *mut u16, scan: *const i16,
    iscan: *const i16,
) {
    highbd_quantize_b_adaptive_neon(coeff_ptr, n_coeffs, zbin_ptr, round_ptr, quant_ptr,
                                    quant_shift_ptr, qcoeff_ptr, dqcoeff_ptr, dequant_ptr,
                                    eob_ptr, scan, iscan, 2);
}
