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

// Translated by hand from upstream/aom/aom_dsp/arm/loopfilter_neon.c
// (ADR 0056). Function names, helper structure, and intrinsic sequences follow
// the original. C locals that the original leaves uninitialized start zeroed
// here. The original's `vld1_dup_u32` / `vld1_lane_u32` / `vst1_lane_u32` on
// `(uint32_t *)` casts of arbitrary byte addresses become the unaligned
// equivalents below (same LD1R / LD1 / ST1 lane semantics without Rust's
// alignment requirement on `*const u32`), as in mem_neon.rs.
#![allow(non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![allow(clippy::all)]

use core::arch::aarch64::*;
use core::ptr;

use crate::simd::aom_dsp__arm__mem_neon::*;
use crate::simd::aom_dsp__arm__transpose_neon::*;

// av1/common/enums.h
const MI_SIZE: isize = 4;

/// LD1R from a possibly unaligned address.
#[inline(always)]
unsafe fn vld1_dup_u32_unaligned(p: *const u32) -> uint32x2_t {
    vdup_n_u32(ptr::read_unaligned(p))
}

/// ST1 (single lane) to a possibly unaligned address.
#[inline(always)]
unsafe fn vst1_lane_u32_unaligned<const LANE: i32>(p: *mut u32, v: uint32x2_t) {
    ptr::write_unaligned(p, vget_lane_u32::<LANE>(v));
}

#[inline(always)]
unsafe fn lpf_mask(p3q3: uint8x8_t, p2q2: uint8x8_t, p1q1: uint8x8_t, p0q0: uint8x8_t,
                   blimit: u8, limit: u8) -> uint8x8_t {
    // Calculate mask values for four samples
    let p0q0_p1q1: uint32x2x2_t;
    let temp_16x8: uint16x8_t;
    let mut temp0_16x4: uint16x4_t;
    let temp1_16x4: uint16x4_t;
    let mut mask_8x8: uint8x8_t;
    let mut temp_8x8: uint8x8_t;
    let limit_8x8: uint8x8_t = vdup_n_u8(limit);
    let blimit_16x4: uint16x4_t = vdup_n_u16(blimit as u16);

    mask_8x8 = vabd_u8(p3q3, p2q2);
    mask_8x8 = vmax_u8(mask_8x8, vabd_u8(p2q2, p1q1));
    mask_8x8 = vmax_u8(mask_8x8, vabd_u8(p1q1, p0q0));
    mask_8x8 = vcle_u8(mask_8x8, limit_8x8);

    temp_8x8 = vreinterpret_u8_u32(vrev64_u32(vreinterpret_u32_u8(mask_8x8)));
    mask_8x8 = vand_u8(mask_8x8, temp_8x8);

    p0q0_p1q1 = vtrn_u32(vreinterpret_u32_u8(p0q0), vreinterpret_u32_u8(p1q1));
    temp_8x8 = vabd_u8(vreinterpret_u8_u32(p0q0_p1q1.0), vreinterpret_u8_u32(p0q0_p1q1.1));
    temp_16x8 = vmovl_u8(temp_8x8);
    temp0_16x4 = vshl_n_u16::<1>(vget_low_u16(temp_16x8));
    temp1_16x4 = vshr_n_u16::<1>(vget_high_u16(temp_16x8));
    temp0_16x4 = vadd_u16(temp0_16x4, temp1_16x4);
    temp0_16x4 = vcle_u16(temp0_16x4, blimit_16x4);
    temp_8x8 = vmovn_u16(vcombine_u16(temp0_16x4, temp0_16x4));

    mask_8x8 = vand_u8(mask_8x8, temp_8x8);

    mask_8x8
}

#[inline(always)]
unsafe fn lpf_mask2(p1q1: uint8x8_t, p0q0: uint8x8_t, blimit: u8, limit: u8) -> uint8x8_t {
    let p0q0_p1q1: uint32x2x2_t;
    let temp_16x8: uint16x8_t;
    let mut temp0_16x4: uint16x4_t;
    let temp1_16x4: uint16x4_t;
    let blimit_16x4: uint16x4_t = vdup_n_u16(blimit as u16);
    let limit_8x8: uint8x8_t = vdup_n_u8(limit);
    let mut mask_8x8: uint8x8_t;
    let mut temp_8x8: uint8x8_t;

    mask_8x8 = vabd_u8(p1q1, p0q0);
    mask_8x8 = vcle_u8(mask_8x8, limit_8x8);

    temp_8x8 = vreinterpret_u8_u32(vrev64_u32(vreinterpret_u32_u8(mask_8x8)));
    mask_8x8 = vand_u8(mask_8x8, temp_8x8);

    p0q0_p1q1 = vtrn_u32(vreinterpret_u32_u8(p0q0), vreinterpret_u32_u8(p1q1));
    temp_8x8 = vabd_u8(vreinterpret_u8_u32(p0q0_p1q1.0), vreinterpret_u8_u32(p0q0_p1q1.1));
    temp_16x8 = vmovl_u8(temp_8x8);
    temp0_16x4 = vshl_n_u16::<1>(vget_low_u16(temp_16x8));
    temp1_16x4 = vshr_n_u16::<1>(vget_high_u16(temp_16x8));
    temp0_16x4 = vadd_u16(temp0_16x4, temp1_16x4);
    temp0_16x4 = vcle_u16(temp0_16x4, blimit_16x4);
    temp_8x8 = vmovn_u16(vcombine_u16(temp0_16x4, temp0_16x4));

    mask_8x8 = vand_u8(mask_8x8, temp_8x8);

    mask_8x8
}

#[inline(always)]
unsafe fn lpf_flat_mask4(p3q3: uint8x8_t, p2q2: uint8x8_t, p1q1: uint8x8_t,
                         p0q0: uint8x8_t) -> uint8x8_t {
    let thresh_8x8: uint8x8_t = vdup_n_u8(1); // for bd==8 threshold is always 1
    let mut flat_8x8: uint8x8_t;
    let temp_8x8: uint8x8_t;

    flat_8x8 = vabd_u8(p1q1, p0q0);
    flat_8x8 = vmax_u8(flat_8x8, vabd_u8(p2q2, p0q0));
    flat_8x8 = vmax_u8(flat_8x8, vabd_u8(p3q3, p0q0));
    flat_8x8 = vcle_u8(flat_8x8, thresh_8x8);

    temp_8x8 = vreinterpret_u8_u32(vrev64_u32(vreinterpret_u32_u8(flat_8x8)));
    flat_8x8 = vand_u8(flat_8x8, temp_8x8);

    flat_8x8
}

#[inline(always)]
unsafe fn lpf_flat_mask3(p2q2: uint8x8_t, p1q1: uint8x8_t, p0q0: uint8x8_t) -> uint8x8_t {
    let thresh_8x8: uint8x8_t = vdup_n_u8(1); // for bd==8 threshold is always 1
    let mut flat_8x8: uint8x8_t;
    let temp_8x8: uint8x8_t;

    flat_8x8 = vabd_u8(p1q1, p0q0);
    flat_8x8 = vmax_u8(flat_8x8, vabd_u8(p2q2, p0q0));
    flat_8x8 = vcle_u8(flat_8x8, thresh_8x8);

    temp_8x8 = vreinterpret_u8_u32(vrev64_u32(vreinterpret_u32_u8(flat_8x8)));
    flat_8x8 = vand_u8(flat_8x8, temp_8x8);

    flat_8x8
}

#[inline(always)]
unsafe fn lpf_mask3_chroma(p2q2: uint8x8_t, p1q1: uint8x8_t, p0q0: uint8x8_t, blimit: u8,
                           limit: u8) -> uint8x8_t {
    // Calculate mask3 values for four samples
    let p0q0_p1q1: uint32x2x2_t;
    let temp_16x8: uint16x8_t;
    let mut temp0_16x4: uint16x4_t;
    let temp1_16x4: uint16x4_t;
    let mut mask_8x8: uint8x8_t;
    let mut temp_8x8: uint8x8_t;
    let limit_8x8: uint8x8_t = vdup_n_u8(limit);
    let blimit_16x4: uint16x4_t = vdup_n_u16(blimit as u16);

    mask_8x8 = vabd_u8(p2q2, p1q1);
    mask_8x8 = vmax_u8(mask_8x8, vabd_u8(p1q1, p0q0));
    mask_8x8 = vcle_u8(mask_8x8, limit_8x8);

    temp_8x8 = vreinterpret_u8_u32(vrev64_u32(vreinterpret_u32_u8(mask_8x8)));
    mask_8x8 = vand_u8(mask_8x8, temp_8x8);

    p0q0_p1q1 = vtrn_u32(vreinterpret_u32_u8(p0q0), vreinterpret_u32_u8(p1q1));
    temp_8x8 = vabd_u8(vreinterpret_u8_u32(p0q0_p1q1.0), vreinterpret_u8_u32(p0q0_p1q1.1));
    temp_16x8 = vmovl_u8(temp_8x8);
    temp0_16x4 = vshl_n_u16::<1>(vget_low_u16(temp_16x8));
    temp1_16x4 = vshr_n_u16::<1>(vget_high_u16(temp_16x8));
    temp0_16x4 = vadd_u16(temp0_16x4, temp1_16x4);
    temp0_16x4 = vcle_u16(temp0_16x4, blimit_16x4);
    temp_8x8 = vmovn_u16(vcombine_u16(temp0_16x4, temp0_16x4));

    mask_8x8 = vand_u8(mask_8x8, temp_8x8);

    mask_8x8
}

#[inline(always)]
unsafe fn filter4(p0q0: uint8x8_t, p1q1: uint8x8_t, p0q0_output: *mut uint8x8_t,
                  p1q1_output: *mut uint8x8_t, mask_8x8: uint8x8_t, thresh: u8) {
    let thresh_f4: uint8x8_t = vdup_n_u8(thresh);
    let sign_mask: int8x8_t = vdup_n_s8(0x80u8 as i8);
    let val_4: int8x8_t = vdup_n_s8(4);
    let val_3: int8x8_t = vdup_n_s8(3);

    let pq_s0: int8x8_t = veor_s8(vreinterpret_s8_u8(p0q0), sign_mask);
    let pq_s1: int8x8_t = veor_s8(vreinterpret_s8_u8(p1q1), sign_mask);

    let ps0_qs0: int32x2x2_t = vtrn_s32(vreinterpret_s32_s8(pq_s0), vreinterpret_s32_s8(pq_s0));
    let ps1_qs1: int32x2x2_t = vtrn_s32(vreinterpret_s32_s8(pq_s1), vreinterpret_s32_s8(pq_s1));
    let ps0_s8: int8x8_t = vreinterpret_s8_s32(ps0_qs0.0);
    let qs0_s8: int8x8_t = vreinterpret_s8_s32(ps0_qs0.1);
    let ps1_s8: int8x8_t = vreinterpret_s8_s32(ps1_qs1.0);
    let qs1_s8: int8x8_t = vreinterpret_s8_s32(ps1_qs1.1);

    // hev_mask
    let temp0_8x8: uint8x8_t = vcgt_u8(vabd_u8(p0q0, p1q1), thresh_f4);
    let temp1_8x8: uint8x8_t = vreinterpret_u8_u32(vrev64_u32(vreinterpret_u32_u8(temp0_8x8)));
    let hev_8x8: int8x8_t = vreinterpret_s8_u8(vorr_u8(temp0_8x8, temp1_8x8));

    // add outer taps if we have high edge variance
    let mut filter_s8: int8x8_t = vqsub_s8(ps1_s8, qs1_s8);
    filter_s8 = vand_s8(filter_s8, hev_8x8);

    // inner taps
    let temp_s8: int8x8_t = vqsub_s8(qs0_s8, ps0_s8);
    let mut filter_s16: int16x8_t = vmovl_s8(filter_s8);
    filter_s16 = vmlal_s8(filter_s16, temp_s8, val_3);
    filter_s8 = vqmovn_s16(filter_s16);
    filter_s8 = vand_s8(filter_s8, vreinterpret_s8_u8(mask_8x8));

    let mut filter1_s8: int8x8_t = vqadd_s8(filter_s8, val_4);
    let mut filter2_s8: int8x8_t = vqadd_s8(filter_s8, val_3);
    filter1_s8 = vshr_n_s8::<3>(filter1_s8);
    filter2_s8 = vshr_n_s8::<3>(filter2_s8);

    let oq0: int8x8_t = veor_s8(vqsub_s8(qs0_s8, filter1_s8), sign_mask);
    let op0: int8x8_t = veor_s8(vqadd_s8(ps0_s8, filter2_s8), sign_mask);

    filter_s8 = vrshr_n_s8::<1>(filter1_s8);
    filter_s8 = vbic_s8(filter_s8, hev_8x8);

    let oq1: int8x8_t = veor_s8(vqsub_s8(qs1_s8, filter_s8), sign_mask);
    let op1: int8x8_t = veor_s8(vqadd_s8(ps1_s8, filter_s8), sign_mask);

    *p0q0_output = vreinterpret_u8_s8(vext_s8::<4>(op0, oq0));
    *p1q1_output = vreinterpret_u8_s8(vext_s8::<4>(op1, oq1));
}

#[inline(always)]
unsafe fn filter8(p0q0: uint8x8_t, p1q1: uint8x8_t, p2q2: uint8x8_t, p3q3: uint8x8_t,
                  p0q0_output: *mut uint8x8_t, p1q1_output: *mut uint8x8_t,
                  p2q2_output: *mut uint8x8_t) {
    // Reverse p and q.
    let q0p0: uint8x8_t = vext_u8::<4>(p0q0, p0q0);
    let q1p1: uint8x8_t = vext_u8::<4>(p1q1, p1q1);
    let q2p2: uint8x8_t = vext_u8::<4>(p2q2, p2q2);

    let p0q0_p1q1: uint16x8_t = vaddl_u8(p0q0, p1q1);
    let p2q2_p3q3: uint16x8_t = vaddl_u8(p3q3, p2q2);
    let out: uint16x8_t = vaddq_u16(p0q0_p1q1, p2q2_p3q3);

    let q0p0_p3q3: uint16x8_t = vaddl_u8(q0p0, p3q3);
    let out_q0p0_p3q3: uint16x8_t = vaddq_u16(out, q0p0_p3q3);

    let out_pq2: uint16x8_t = vaddq_u16(out_q0p0_p3q3, p2q2_p3q3);

    let p1q1_q1p1: uint16x8_t = vaddl_u8(p1q1, q1p1);
    let out_pq1: uint16x8_t = vaddq_u16(out_q0p0_p3q3, p1q1_q1p1);

    let q0p0_p0q0: uint16x8_t = vaddl_u8(q0p0, p0q0);
    let q1p1_q2p2: uint16x8_t = vaddl_u8(q1p1, q2p2);
    let mut out_pq0: uint16x8_t = vaddq_u16(q0p0_p0q0, q1p1_q2p2);
    out_pq0 = vaddq_u16(out_pq0, out);

    *p0q0_output = vrshrn_n_u16::<3>(out_pq0);
    *p1q1_output = vrshrn_n_u16::<3>(out_pq1);
    *p2q2_output = vrshrn_n_u16::<3>(out_pq2);
}

#[inline(always)]
unsafe fn filter14(p0q0: uint8x8_t, p1q1: uint8x8_t, p2q2: uint8x8_t, p3q3: uint8x8_t,
                   p4q4: uint8x8_t, p5q5: uint8x8_t, p6q6: uint8x8_t,
                   p0q0_output: *mut uint8x8_t, p1q1_output: *mut uint8x8_t,
                   p2q2_output: *mut uint8x8_t, p3q3_output: *mut uint8x8_t,
                   p4q4_output: *mut uint8x8_t, p5q5_output: *mut uint8x8_t) {
    // Reverse p and q.
    let q0p0: uint8x8_t = vext_u8::<4>(p0q0, p0q0);
    let q1p1: uint8x8_t = vext_u8::<4>(p1q1, p1q1);
    let q2p2: uint8x8_t = vext_u8::<4>(p2q2, p2q2);
    let q3p3: uint8x8_t = vext_u8::<4>(p3q3, p3q3);
    let q4p4: uint8x8_t = vext_u8::<4>(p4q4, p4q4);
    let q5p5: uint8x8_t = vext_u8::<4>(p5q5, p5q5);

    let p0q0_p1q1: uint16x8_t = vaddl_u8(p0q0, p1q1);
    let p2q2_p3q3: uint16x8_t = vaddl_u8(p2q2, p3q3);
    let mut out: uint16x8_t = vaddq_u16(p0q0_p1q1, p2q2_p3q3);

    let q0p0_p4q4: uint16x8_t = vaddl_u8(q0p0, p4q4);
    let p5q5_p6q6: uint16x8_t = vaddl_u8(p5q5, p6q6);
    let tmp: uint16x8_t = vaddq_u16(q0p0_p4q4, p5q5_p6q6);
    // This offset removes the need for a rounding shift at the end.
    let tmp_offset: uint16x8_t = vaddq_u16(tmp, vdupq_n_u16(1 << 3));
    out = vaddq_u16(out, tmp_offset);

    let mut out_pq5: uint16x8_t = vaddw_u8(out, p4q4);
    let mut out_pq4: uint16x8_t = vaddw_u8(out_pq5, p3q3);
    let mut out_pq3: uint16x8_t = vaddw_u8(out_pq4, p2q2);

    out_pq5 = vaddw_u8(out_pq5, p5q5);

    let mut out_pq0: uint16x8_t = vaddw_u8(out, p1q1);
    let mut out_pq1: uint16x8_t = vaddw_u8(out_pq0, p2q2);
    let mut out_pq2: uint16x8_t = vaddw_u8(out_pq1, p3q3);

    let p0q0_q0p0: uint16x8_t = vaddl_u8(p0q0, q0p0);
    out_pq0 = vaddq_u16(out_pq0, p0q0_q0p0);

    let p0q0_p6q6: uint16x8_t = vaddl_u8(p0q0, p6q6);
    out_pq1 = vaddq_u16(out_pq1, p0q0_p6q6);
    let p5q5_q1p1: uint16x8_t = vaddl_u8(p5q5, q1p1);
    out_pq4 = vaddq_u16(out_pq4, p5q5_q1p1);

    let p6q6_p6q6: uint16x8_t = vaddl_u8(p6q6, p6q6);
    out_pq2 = vaddq_u16(out_pq2, p6q6_p6q6);
    let mut p6q6_temp: uint16x8_t = vaddw_u8(p6q6_p6q6, p6q6);
    out_pq3 = vaddq_u16(out_pq3, p6q6_temp);
    p6q6_temp = vaddw_u8(p6q6_temp, p6q6);
    out_pq4 = vaddq_u16(out_pq4, p6q6_temp);
    p6q6_temp = vaddq_u16(p6q6_temp, p6q6_p6q6);
    out_pq5 = vaddq_u16(out_pq5, p6q6_temp);

    let mut qp_sum: uint16x8_t = vaddl_u8(q2p2, q1p1);
    out_pq3 = vaddq_u16(out_pq3, qp_sum);

    qp_sum = vaddw_u8(qp_sum, q3p3);
    out_pq2 = vaddq_u16(out_pq2, qp_sum);

    qp_sum = vaddw_u8(qp_sum, q4p4);
    out_pq1 = vaddq_u16(out_pq1, qp_sum);

    qp_sum = vaddw_u8(qp_sum, q5p5);
    out_pq0 = vaddq_u16(out_pq0, qp_sum);

    *p0q0_output = vshrn_n_u16::<4>(out_pq0);
    *p1q1_output = vshrn_n_u16::<4>(out_pq1);
    *p2q2_output = vshrn_n_u16::<4>(out_pq2);
    *p3q3_output = vshrn_n_u16::<4>(out_pq3);
    *p4q4_output = vshrn_n_u16::<4>(out_pq4);
    *p5q5_output = vshrn_n_u16::<4>(out_pq5);
}

#[inline(always)]
unsafe fn lpf_14_neon(p6q6: *mut uint8x8_t, p5q5: *mut uint8x8_t, p4q4: *mut uint8x8_t,
                      p3q3: *mut uint8x8_t, p2q2: *mut uint8x8_t, p1q1: *mut uint8x8_t,
                      p0q0: *mut uint8x8_t, blimit: u8, limit: u8, thresh: u8) {
    let mut out_f14_pq0: uint8x8_t = vdup_n_u8(0);
    let mut out_f14_pq1: uint8x8_t = vdup_n_u8(0);
    let mut out_f14_pq2: uint8x8_t = vdup_n_u8(0);
    let mut out_f14_pq3: uint8x8_t = vdup_n_u8(0);
    let mut out_f14_pq4: uint8x8_t = vdup_n_u8(0);
    let mut out_f14_pq5: uint8x8_t = vdup_n_u8(0);
    let mut out_f7_pq0: uint8x8_t = vdup_n_u8(0);
    let mut out_f7_pq1: uint8x8_t = vdup_n_u8(0);
    let mut out_f7_pq2: uint8x8_t = vdup_n_u8(0);
    let mut out_f4_pq0: uint8x8_t = vdup_n_u8(0);
    let mut out_f4_pq1: uint8x8_t = vdup_n_u8(0);

    // Calculate filter masks.
    let mask_8x8: uint8x8_t = lpf_mask(*p3q3, *p2q2, *p1q1, *p0q0, blimit, limit);
    let flat_8x8: uint8x8_t = lpf_flat_mask4(*p3q3, *p2q2, *p1q1, *p0q0);
    let flat2_8x8: uint8x8_t = lpf_flat_mask4(*p6q6, *p5q5, *p4q4, *p0q0);

    // No filtering.
    if vget_lane_u64::<0>(vreinterpret_u64_u8(mask_8x8)) == 0 {
        return;
    }

    let filter8_cond: uint8x8_t = vand_u8(flat_8x8, mask_8x8);
    let filter4_cond: uint8x8_t = vmvn_u8(filter8_cond);
    let filter14_cond: uint8x8_t = vand_u8(filter8_cond, flat2_8x8);

    if vget_lane_s64::<0>(vreinterpret_s64_u8(filter14_cond)) == -1 {
        // Only filter14() applies.
        filter14(*p0q0, *p1q1, *p2q2, *p3q3, *p4q4, *p5q5, *p6q6, &mut out_f14_pq0,
                 &mut out_f14_pq1, &mut out_f14_pq2, &mut out_f14_pq3, &mut out_f14_pq4,
                 &mut out_f14_pq5);

        *p0q0 = out_f14_pq0;
        *p1q1 = out_f14_pq1;
        *p2q2 = out_f14_pq2;
        *p3q3 = out_f14_pq3;
        *p4q4 = out_f14_pq4;
        *p5q5 = out_f14_pq5;
    } else if vget_lane_u64::<0>(vreinterpret_u64_u8(filter14_cond)) == 0
        && vget_lane_s64::<0>(vreinterpret_s64_u8(filter8_cond)) == -1
    {
        // Only filter8() applies.
        filter8(*p0q0, *p1q1, *p2q2, *p3q3, &mut out_f7_pq0, &mut out_f7_pq1, &mut out_f7_pq2);

        *p0q0 = out_f7_pq0;
        *p1q1 = out_f7_pq1;
        *p2q2 = out_f7_pq2;
    } else {
        filter4(*p0q0, *p1q1, &mut out_f4_pq0, &mut out_f4_pq1, mask_8x8, thresh);

        if vget_lane_u64::<0>(vreinterpret_u64_u8(filter14_cond)) == 0
            && vget_lane_u64::<0>(vreinterpret_u64_u8(filter8_cond)) == 0
        {
            // filter8() and filter14() do not apply, but filter4() applies to one or
            // more values.
            *p0q0 = vbsl_u8(filter4_cond, out_f4_pq0, *p0q0);
            *p1q1 = vbsl_u8(filter4_cond, out_f4_pq1, *p1q1);
        } else {
            filter8(*p0q0, *p1q1, *p2q2, *p3q3, &mut out_f7_pq0, &mut out_f7_pq1,
                    &mut out_f7_pq2);

            if vget_lane_u64::<0>(vreinterpret_u64_u8(filter14_cond)) == 0 {
                // filter14() does not apply, but filter8() and filter4() apply to one
                // or more values. filter4 outputs
                *p0q0 = vbsl_u8(filter4_cond, out_f4_pq0, *p0q0);
                *p1q1 = vbsl_u8(filter4_cond, out_f4_pq1, *p1q1);

                // filter8 outputs
                *p0q0 = vbsl_u8(filter8_cond, out_f7_pq0, *p0q0);
                *p1q1 = vbsl_u8(filter8_cond, out_f7_pq1, *p1q1);
                *p2q2 = vbsl_u8(filter8_cond, out_f7_pq2, *p2q2);
            } else {
                // All filters may contribute values to final outputs.
                filter14(*p0q0, *p1q1, *p2q2, *p3q3, *p4q4, *p5q5, *p6q6, &mut out_f14_pq0,
                         &mut out_f14_pq1, &mut out_f14_pq2, &mut out_f14_pq3,
                         &mut out_f14_pq4, &mut out_f14_pq5);

                // filter4 outputs
                *p0q0 = vbsl_u8(filter4_cond, out_f4_pq0, *p0q0);
                *p1q1 = vbsl_u8(filter4_cond, out_f4_pq1, *p1q1);

                // filter8 outputs
                *p0q0 = vbsl_u8(filter8_cond, out_f7_pq0, *p0q0);
                *p1q1 = vbsl_u8(filter8_cond, out_f7_pq1, *p1q1);
                *p2q2 = vbsl_u8(filter8_cond, out_f7_pq2, *p2q2);

                // filter14 outputs
                *p0q0 = vbsl_u8(filter14_cond, out_f14_pq0, *p0q0);
                *p1q1 = vbsl_u8(filter14_cond, out_f14_pq1, *p1q1);
                *p2q2 = vbsl_u8(filter14_cond, out_f14_pq2, *p2q2);
                *p3q3 = vbsl_u8(filter14_cond, out_f14_pq3, *p3q3);
                *p4q4 = vbsl_u8(filter14_cond, out_f14_pq4, *p4q4);
                *p5q5 = vbsl_u8(filter14_cond, out_f14_pq5, *p5q5);
            }
        }
    }
}

#[inline(always)]
unsafe fn lpf_8_neon(p3q3: *mut uint8x8_t, p2q2: *mut uint8x8_t, p1q1: *mut uint8x8_t,
                     p0q0: *mut uint8x8_t, blimit: u8, limit: u8, thresh: u8) {
    let mut out_f7_pq0: uint8x8_t = vdup_n_u8(0);
    let mut out_f7_pq1: uint8x8_t = vdup_n_u8(0);
    let mut out_f7_pq2: uint8x8_t = vdup_n_u8(0);
    let mut out_f4_pq0: uint8x8_t = vdup_n_u8(0);
    let mut out_f4_pq1: uint8x8_t = vdup_n_u8(0);

    // Calculate filter masks.
    let mask_8x8: uint8x8_t = lpf_mask(*p3q3, *p2q2, *p1q1, *p0q0, blimit, limit);
    let flat_8x8: uint8x8_t = lpf_flat_mask4(*p3q3, *p2q2, *p1q1, *p0q0);

    // No filtering.
    if vget_lane_u64::<0>(vreinterpret_u64_u8(mask_8x8)) == 0 {
        return;
    }

    let filter8_cond: uint8x8_t = vand_u8(flat_8x8, mask_8x8);
    let filter4_cond: uint8x8_t = vmvn_u8(filter8_cond);

    // Not needing filter4() at all is a very common case, so isolate it to avoid
    // needlessly computing filter4().
    if vget_lane_s64::<0>(vreinterpret_s64_u8(filter8_cond)) == -1 {
        filter8(*p0q0, *p1q1, *p2q2, *p3q3, &mut out_f7_pq0, &mut out_f7_pq1, &mut out_f7_pq2);

        *p0q0 = out_f7_pq0;
        *p1q1 = out_f7_pq1;
        *p2q2 = out_f7_pq2;
    } else {
        filter4(*p0q0, *p1q1, &mut out_f4_pq0, &mut out_f4_pq1, mask_8x8, thresh);

        if vget_lane_u64::<0>(vreinterpret_u64_u8(filter8_cond)) == 0 {
            // filter8() does not apply, but filter4() applies to one or more values.
            *p0q0 = vbsl_u8(filter4_cond, out_f4_pq0, *p0q0);
            *p1q1 = vbsl_u8(filter4_cond, out_f4_pq1, *p1q1);
        } else {
            filter8(*p0q0, *p1q1, *p2q2, *p3q3, &mut out_f7_pq0, &mut out_f7_pq1,
                    &mut out_f7_pq2);

            // filter4 outputs
            *p0q0 = vbsl_u8(filter4_cond, out_f4_pq0, *p0q0);
            *p1q1 = vbsl_u8(filter4_cond, out_f4_pq1, *p1q1);

            // filter8 outputs
            *p0q0 = vbsl_u8(filter8_cond, out_f7_pq0, *p0q0);
            *p1q1 = vbsl_u8(filter8_cond, out_f7_pq1, *p1q1);
            *p2q2 = vbsl_u8(filter8_cond, out_f7_pq2, *p2q2);
        }
    }
}

#[inline(always)]
unsafe fn filter6(p0q0: uint8x8_t, p1q1: uint8x8_t, p2q2: uint8x8_t,
                  p0q0_output: *mut uint8x8_t, p1q1_output: *mut uint8x8_t) {
    let q0p0: uint8x8_t = vext_u8::<4>(p0q0, p0q0);

    let p0q0_p1q1: uint16x8_t = vaddl_u8(p0q0, p1q1);
    let mut out: uint16x8_t = vaddq_u16(p0q0_p1q1, p0q0_p1q1);

    let q0p0_p2q2: uint16x8_t = vaddl_u8(q0p0, p2q2);
    out = vaddq_u16(out, q0p0_p2q2);

    let q0p0_q1p1: uint16x8_t = vextq_u16::<4>(p0q0_p1q1, p0q0_p1q1);
    let out_pq0: uint16x8_t = vaddq_u16(out, q0p0_q1p1);

    let p2q2_p2q2: uint16x8_t = vaddl_u8(p2q2, p2q2);
    let out_pq1: uint16x8_t = vaddq_u16(out, p2q2_p2q2);

    *p0q0_output = vrshrn_n_u16::<3>(out_pq0);
    *p1q1_output = vrshrn_n_u16::<3>(out_pq1);
}

#[inline(always)]
unsafe fn lpf_6_neon(p2q2: *mut uint8x8_t, p1q1: *mut uint8x8_t, p0q0: *mut uint8x8_t,
                     blimit: u8, limit: u8, thresh: u8) {
    let mut out_f6_pq0: uint8x8_t = vdup_n_u8(0);
    let mut out_f6_pq1: uint8x8_t = vdup_n_u8(0);
    let mut out_f4_pq0: uint8x8_t = vdup_n_u8(0);
    let mut out_f4_pq1: uint8x8_t = vdup_n_u8(0);

    // Calculate filter masks.
    let mask_8x8: uint8x8_t = lpf_mask3_chroma(*p2q2, *p1q1, *p0q0, blimit, limit);
    let flat_8x8: uint8x8_t = lpf_flat_mask3(*p2q2, *p1q1, *p0q0);

    // No filtering.
    if vget_lane_u64::<0>(vreinterpret_u64_u8(mask_8x8)) == 0 {
        return;
    }

    let filter6_cond: uint8x8_t = vand_u8(flat_8x8, mask_8x8);
    let filter4_cond: uint8x8_t = vmvn_u8(filter6_cond);

    // Not needing filter4 at all is a very common case, so isolate it to avoid
    // needlessly computing filter4.
    if vget_lane_s64::<0>(vreinterpret_s64_u8(filter6_cond)) == -1 {
        filter6(*p0q0, *p1q1, *p2q2, &mut out_f6_pq0, &mut out_f6_pq1);

        *p0q0 = out_f6_pq0;
        *p1q1 = out_f6_pq1;
    } else {
        filter4(*p0q0, *p1q1, &mut out_f4_pq0, &mut out_f4_pq1, mask_8x8, thresh);

        if vget_lane_u64::<0>(vreinterpret_u64_u8(filter6_cond)) == 0 {
            // filter6 does not apply, but filter4 applies to one or more values.
            *p0q0 = vbsl_u8(filter4_cond, out_f4_pq0, *p0q0);
            *p1q1 = vbsl_u8(filter4_cond, out_f4_pq1, *p1q1);
        } else {
            // All filters may contribute to the final output.
            filter6(*p0q0, *p1q1, *p2q2, &mut out_f6_pq0, &mut out_f6_pq1);

            // filter4 outputs
            *p0q0 = vbsl_u8(filter4_cond, out_f4_pq0, *p0q0);
            *p1q1 = vbsl_u8(filter4_cond, out_f4_pq1, *p1q1);

            // filter6 outputs
            *p0q0 = vbsl_u8(filter6_cond, out_f6_pq0, *p0q0);
            *p1q1 = vbsl_u8(filter6_cond, out_f6_pq1, *p1q1);
        }
    }
}

#[inline(always)]
unsafe fn lpf_4_neon(p1q1: *mut uint8x8_t, p0q0: *mut uint8x8_t, blimit: u8, limit: u8,
                     thresh: u8) {
    let mut out_f4_pq0: uint8x8_t = vdup_n_u8(0);
    let mut out_f4_pq1: uint8x8_t = vdup_n_u8(0);

    // Calculate filter mask
    let mask_8x8: uint8x8_t = lpf_mask2(*p1q1, *p0q0, blimit, limit);

    // No filtering.
    if vget_lane_u64::<0>(vreinterpret_u64_u8(mask_8x8)) == 0 {
        return;
    }

    filter4(*p0q0, *p1q1, &mut out_f4_pq0, &mut out_f4_pq1, mask_8x8, thresh);

    *p0q0 = out_f4_pq0;
    *p1q1 = out_f4_pq1;
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_lpf_vertical_14_neon(src: *mut u8, stride: i32,
                                                         blimit: *const u8, limit: *const u8,
                                                         thresh: *const u8) {
    let stride = stride as isize;
    let mut row0: uint8x16_t = vdupq_n_u8(0);
    let mut row1: uint8x16_t = vdupq_n_u8(0);
    let mut row2: uint8x16_t = vdupq_n_u8(0);
    let mut row3: uint8x16_t = vdupq_n_u8(0);
    let mut pxp3: uint8x8_t;
    let mut p6p2: uint8x8_t;
    let mut p5p1: uint8x8_t;
    let mut p4p0: uint8x8_t;
    let mut q0q4: uint8x8_t;
    let mut q1q5: uint8x8_t;
    let mut q2q6: uint8x8_t;
    let mut q3qy: uint8x8_t;
    let mut p6q6_p2q2: uint32x2x2_t;
    let mut p5q5_p1q1: uint32x2x2_t;
    let mut p4q4_p0q0: uint32x2x2_t;
    let mut pxqx_p3q3: uint32x2x2_t;
    let mut pq_rev: uint32x2_t;
    let mut p0q0: uint8x8_t;
    let mut p1q1: uint8x8_t;
    let mut p2q2: uint8x8_t;
    let mut p3q3: uint8x8_t;
    let mut p4q4: uint8x8_t;
    let mut p5q5: uint8x8_t;
    let mut p6q6: uint8x8_t;

    // row0: x p6 p5 p4 p3 p2 p1 p0 | q0 q1 q2 q3 q4 q5 q6 y
    // row1: x p6 p5 p4 p3 p2 p1 p0 | q0 q1 q2 q3 q4 q5 q6 y
    // row2: x p6 p5 p4 p3 p2 p1 p0 | q0 q1 q2 q3 q4 q5 q6 y
    // row3: x p6 p5 p4 p3 p2 p1 p0 | q0 q1 q2 q3 q4 q5 q6 y
    load_u8_16x4(src.offset(-8), stride, &mut row0, &mut row1, &mut row2, &mut row3);

    pxp3 = vget_low_u8(row0);
    p6p2 = vget_low_u8(row1);
    p5p1 = vget_low_u8(row2);
    p4p0 = vget_low_u8(row3);
    transpose_elems_inplace_u8_8x4(&mut pxp3, &mut p6p2, &mut p5p1, &mut p4p0);

    q0q4 = vget_high_u8(row0);
    q1q5 = vget_high_u8(row1);
    q2q6 = vget_high_u8(row2);
    q3qy = vget_high_u8(row3);
    transpose_elems_inplace_u8_8x4(&mut q0q4, &mut q1q5, &mut q2q6, &mut q3qy);

    pq_rev = vrev64_u32(vreinterpret_u32_u8(q3qy));
    pxqx_p3q3 = vtrn_u32(vreinterpret_u32_u8(pxp3), pq_rev);

    pq_rev = vrev64_u32(vreinterpret_u32_u8(q1q5));
    p5q5_p1q1 = vtrn_u32(vreinterpret_u32_u8(p5p1), pq_rev);

    pq_rev = vrev64_u32(vreinterpret_u32_u8(q0q4));
    p4q4_p0q0 = vtrn_u32(vreinterpret_u32_u8(p4p0), pq_rev);

    pq_rev = vrev64_u32(vreinterpret_u32_u8(q2q6));
    p6q6_p2q2 = vtrn_u32(vreinterpret_u32_u8(p6p2), pq_rev);

    p0q0 = vreinterpret_u8_u32(p4q4_p0q0.1);
    p1q1 = vreinterpret_u8_u32(p5q5_p1q1.1);
    p2q2 = vreinterpret_u8_u32(p6q6_p2q2.1);
    p3q3 = vreinterpret_u8_u32(pxqx_p3q3.1);
    p4q4 = vreinterpret_u8_u32(p4q4_p0q0.0);
    p5q5 = vreinterpret_u8_u32(p5q5_p1q1.0);
    p6q6 = vreinterpret_u8_u32(p6q6_p2q2.0);

    lpf_14_neon(&mut p6q6, &mut p5q5, &mut p4q4, &mut p3q3, &mut p2q2, &mut p1q1, &mut p0q0,
                *blimit, *limit, *thresh);

    pxqx_p3q3 = vtrn_u32(pxqx_p3q3.0, vreinterpret_u32_u8(p3q3));
    p5q5_p1q1 = vtrn_u32(vreinterpret_u32_u8(p5q5), vreinterpret_u32_u8(p1q1));
    p4q4_p0q0 = vtrn_u32(vreinterpret_u32_u8(p4q4), vreinterpret_u32_u8(p0q0));
    p6q6_p2q2 = vtrn_u32(vreinterpret_u32_u8(p6q6), vreinterpret_u32_u8(p2q2));

    pxqx_p3q3.1 = vrev64_u32(pxqx_p3q3.1);
    p5q5_p1q1.1 = vrev64_u32(p5q5_p1q1.1);
    p4q4_p0q0.1 = vrev64_u32(p4q4_p0q0.1);
    p6q6_p2q2.1 = vrev64_u32(p6q6_p2q2.1);

    q0q4 = vreinterpret_u8_u32(p4q4_p0q0.1);
    q1q5 = vreinterpret_u8_u32(p5q5_p1q1.1);
    q2q6 = vreinterpret_u8_u32(p6q6_p2q2.1);
    q3qy = vreinterpret_u8_u32(pxqx_p3q3.1);
    transpose_elems_inplace_u8_8x4(&mut q0q4, &mut q1q5, &mut q2q6, &mut q3qy);

    pxp3 = vreinterpret_u8_u32(pxqx_p3q3.0);
    p6p2 = vreinterpret_u8_u32(p6q6_p2q2.0);
    p5p1 = vreinterpret_u8_u32(p5q5_p1q1.0);
    p4p0 = vreinterpret_u8_u32(p4q4_p0q0.0);
    transpose_elems_inplace_u8_8x4(&mut pxp3, &mut p6p2, &mut p5p1, &mut p4p0);

    row0 = vcombine_u8(pxp3, q0q4);
    row1 = vcombine_u8(p6p2, q1q5);
    row2 = vcombine_u8(p5p1, q2q6);
    row3 = vcombine_u8(p4p0, q3qy);

    store_u8_16x4(src.offset(-8), stride, row0, row1, row2, row3);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_lpf_vertical_14_dual_neon(
    s: *mut u8, pitch: i32, blimit0: *const u8, limit0: *const u8, thresh0: *const u8,
    blimit1: *const u8, limit1: *const u8, thresh1: *const u8,
) {
    aom_lpf_vertical_14_neon(s, pitch, blimit0, limit0, thresh0);
    aom_lpf_vertical_14_neon(s.offset(4 * pitch as isize), pitch, blimit1, limit1, thresh1);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_lpf_vertical_14_quad_neon(s: *mut u8, pitch: i32,
                                                              blimit: *const u8,
                                                              limit: *const u8,
                                                              thresh: *const u8) {
    aom_lpf_vertical_14_dual_neon(s, pitch, blimit, limit, thresh, blimit, limit, thresh);
    aom_lpf_vertical_14_dual_neon(s.offset(2 * MI_SIZE * pitch as isize), pitch, blimit, limit,
                                  thresh, blimit, limit, thresh);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_lpf_vertical_8_neon(src: *mut u8, stride: i32,
                                                        blimit: *const u8, limit: *const u8,
                                                        thresh: *const u8) {
    let stride = stride as isize;
    let mut p2q2_p1q1: uint32x2x2_t;
    let mut p3q3_p0q0: uint32x2x2_t;
    let mut pq_rev: uint32x2_t;
    let mut p3q0: uint8x8_t = vdup_n_u8(0);
    let mut p2q1: uint8x8_t = vdup_n_u8(0);
    let mut p1q2: uint8x8_t = vdup_n_u8(0);
    let mut p0q3: uint8x8_t = vdup_n_u8(0);
    let mut p0q0: uint8x8_t;
    let mut p1q1: uint8x8_t;
    let mut p2q2: uint8x8_t;
    let mut p3q3: uint8x8_t;

    // row0: p3 p2 p1 p0 | q0 q1 q2 q3
    // row1: p3 p2 p1 p0 | q0 q1 q2 q3
    // row2: p3 p2 p1 p0 | q0 q1 q2 q3
    // row3: p3 p2 p1 p0 | q0 q1 q2 q3
    load_u8_8x4(src.offset(-4), stride, &mut p3q0, &mut p2q1, &mut p1q2, &mut p0q3);

    transpose_elems_inplace_u8_8x4(&mut p3q0, &mut p2q1, &mut p1q2, &mut p0q3);

    pq_rev = vrev64_u32(vreinterpret_u32_u8(p0q3));
    p3q3_p0q0 = vtrn_u32(vreinterpret_u32_u8(p3q0), pq_rev);

    pq_rev = vrev64_u32(vreinterpret_u32_u8(p1q2));
    p2q2_p1q1 = vtrn_u32(vreinterpret_u32_u8(p2q1), pq_rev);

    p0q0 = vreinterpret_u8_u32(vrev64_u32(p3q3_p0q0.1));
    p1q1 = vreinterpret_u8_u32(vrev64_u32(p2q2_p1q1.1));
    p2q2 = vreinterpret_u8_u32(p2q2_p1q1.0);
    p3q3 = vreinterpret_u8_u32(p3q3_p0q0.0);

    lpf_8_neon(&mut p3q3, &mut p2q2, &mut p1q1, &mut p0q0, *blimit, *limit, *thresh);

    pq_rev = vrev64_u32(vreinterpret_u32_u8(p0q0));
    p3q3_p0q0 = vtrn_u32(vreinterpret_u32_u8(p3q3), pq_rev);

    pq_rev = vrev64_u32(vreinterpret_u32_u8(p1q1));
    p2q2_p1q1 = vtrn_u32(vreinterpret_u32_u8(p2q2), pq_rev);

    p0q3 = vreinterpret_u8_u32(vrev64_u32(p3q3_p0q0.1));
    p1q2 = vreinterpret_u8_u32(vrev64_u32(p2q2_p1q1.1));
    p2q1 = vreinterpret_u8_u32(p2q2_p1q1.0);
    p3q0 = vreinterpret_u8_u32(p3q3_p0q0.0);
    transpose_elems_inplace_u8_8x4(&mut p3q0, &mut p2q1, &mut p1q2, &mut p0q3);

    store_u8_8x4(src.offset(-4), stride, p3q0, p2q1, p1q2, p0q3);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_lpf_vertical_8_dual_neon(
    s: *mut u8, pitch: i32, blimit0: *const u8, limit0: *const u8, thresh0: *const u8,
    blimit1: *const u8, limit1: *const u8, thresh1: *const u8,
) {
    aom_lpf_vertical_8_neon(s, pitch, blimit0, limit0, thresh0);
    aom_lpf_vertical_8_neon(s.offset(4 * pitch as isize), pitch, blimit1, limit1, thresh1);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_lpf_vertical_8_quad_neon(s: *mut u8, pitch: i32,
                                                             blimit: *const u8,
                                                             limit: *const u8,
                                                             thresh: *const u8) {
    aom_lpf_vertical_8_dual_neon(s, pitch, blimit, limit, thresh, blimit, limit, thresh);
    aom_lpf_vertical_8_dual_neon(s.offset(2 * MI_SIZE * pitch as isize), pitch, blimit, limit,
                                 thresh, blimit, limit, thresh);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_lpf_vertical_6_neon(src: *mut u8, stride: i32,
                                                        blimit: *const u8, limit: *const u8,
                                                        thresh: *const u8) {
    let stride = stride as isize;
    let mut p2q2_p1q1: uint32x2x2_t;
    let mut pxqy_p0q0: uint32x2x2_t;
    let mut pq_rev: uint32x2_t;
    let mut pxq0: uint8x8_t = vdup_n_u8(0);
    let mut p2q1: uint8x8_t = vdup_n_u8(0);
    let mut p1q2: uint8x8_t = vdup_n_u8(0);
    let mut p0qy: uint8x8_t = vdup_n_u8(0);
    let mut p0q0: uint8x8_t;
    let mut p1q1: uint8x8_t;
    let mut p2q2: uint8x8_t;
    let pxqy: uint8x8_t;

    // row0: px p2 p1 p0 | q0 q1 q2 qy
    // row1: px p2 p1 p0 | q0 q1 q2 qy
    // row2: px p2 p1 p0 | q0 q1 q2 qy
    // row3: px p2 p1 p0 | q0 q1 q2 qy
    load_u8_8x4(src.offset(-4), stride, &mut pxq0, &mut p2q1, &mut p1q2, &mut p0qy);

    transpose_elems_inplace_u8_8x4(&mut pxq0, &mut p2q1, &mut p1q2, &mut p0qy);

    pq_rev = vrev64_u32(vreinterpret_u32_u8(p0qy));
    pxqy_p0q0 = vtrn_u32(vreinterpret_u32_u8(pxq0), pq_rev);

    pq_rev = vrev64_u32(vreinterpret_u32_u8(p1q2));
    p2q2_p1q1 = vtrn_u32(vreinterpret_u32_u8(p2q1), pq_rev);

    p0q0 = vreinterpret_u8_u32(vrev64_u32(pxqy_p0q0.1));
    p1q1 = vreinterpret_u8_u32(vrev64_u32(p2q2_p1q1.1));
    p2q2 = vreinterpret_u8_u32(p2q2_p1q1.0);
    pxqy = vreinterpret_u8_u32(pxqy_p0q0.0);

    lpf_6_neon(&mut p2q2, &mut p1q1, &mut p0q0, *blimit, *limit, *thresh);

    pq_rev = vrev64_u32(vreinterpret_u32_u8(p0q0));
    pxqy_p0q0 = vtrn_u32(vreinterpret_u32_u8(pxqy), pq_rev);

    pq_rev = vrev64_u32(vreinterpret_u32_u8(p1q1));
    p2q2_p1q1 = vtrn_u32(vreinterpret_u32_u8(p2q2), pq_rev);

    p0qy = vreinterpret_u8_u32(vrev64_u32(pxqy_p0q0.1));
    p1q2 = vreinterpret_u8_u32(vrev64_u32(p2q2_p1q1.1));
    p2q1 = vreinterpret_u8_u32(p2q2_p1q1.0);
    pxq0 = vreinterpret_u8_u32(pxqy_p0q0.0);
    transpose_elems_inplace_u8_8x4(&mut pxq0, &mut p2q1, &mut p1q2, &mut p0qy);

    store_u8_8x4(src.offset(-4), stride, pxq0, p2q1, p1q2, p0qy);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_lpf_vertical_6_dual_neon(
    s: *mut u8, pitch: i32, blimit0: *const u8, limit0: *const u8, thresh0: *const u8,
    blimit1: *const u8, limit1: *const u8, thresh1: *const u8,
) {
    aom_lpf_vertical_6_neon(s, pitch, blimit0, limit0, thresh0);
    aom_lpf_vertical_6_neon(s.offset(4 * pitch as isize), pitch, blimit1, limit1, thresh1);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_lpf_vertical_6_quad_neon(s: *mut u8, pitch: i32,
                                                             blimit: *const u8,
                                                             limit: *const u8,
                                                             thresh: *const u8) {
    aom_lpf_vertical_6_dual_neon(s, pitch, blimit, limit, thresh, blimit, limit, thresh);
    aom_lpf_vertical_6_dual_neon(s.offset(2 * MI_SIZE * pitch as isize), pitch, blimit, limit,
                                 thresh, blimit, limit, thresh);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_lpf_vertical_4_neon(src: *mut u8, stride: i32,
                                                        blimit: *const u8, limit: *const u8,
                                                        thresh: *const u8) {
    let stride = stride as isize;
    let p1q0_p0q1: uint32x2x2_t;
    let p1q1_p0q0: uint32x2x2_t;
    let p1p0_q1q0: uint32x2x2_t;
    let pq_rev: uint32x2_t;
    let mut p1p0: uint8x8_t = vdup_n_u8(0);
    let mut q0q1: uint8x8_t = vdup_n_u8(0);
    let mut p0q0: uint8x8_t;
    let mut p1q1: uint8x8_t;

    // row0: p1 p0 | q0 q1
    // row1: p1 p0 | q0 q1
    // row2: p1 p0 | q0 q1
    // row3: p1 p0 | q0 q1
    load_unaligned_u8_4x4(src.offset(-2), stride, &mut p1p0, &mut q0q1);

    transpose_elems_inplace_u8_4x4(&mut p1p0, &mut q0q1);

    p1q0_p0q1 = vtrn_u32(vreinterpret_u32_u8(p1p0), vreinterpret_u32_u8(q0q1));

    pq_rev = vrev64_u32(p1q0_p0q1.1);
    p1q1_p0q0 = vtrn_u32(p1q0_p0q1.0, pq_rev);

    p1q1 = vreinterpret_u8_u32(p1q1_p0q0.0);
    p0q0 = vreinterpret_u8_u32(p1q1_p0q0.1);

    lpf_4_neon(&mut p1q1, &mut p0q0, *blimit, *limit, *thresh);

    p1p0_q1q0 = vtrn_u32(vreinterpret_u32_u8(p1q1), vreinterpret_u32_u8(p0q0));

    p1p0 = vreinterpret_u8_u32(p1p0_q1q0.0);
    q0q1 = vreinterpret_u8_u32(vrev64_u32(p1p0_q1q0.1));

    transpose_elems_inplace_u8_4x4(&mut p1p0, &mut q0q1);

    store_u8x4_strided_x2(src.offset(-2), 2 * stride, p1p0);
    store_u8x4_strided_x2(src.offset(stride - 2), 2 * stride, q0q1);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_lpf_vertical_4_dual_neon(
    s: *mut u8, pitch: i32, blimit0: *const u8, limit0: *const u8, thresh0: *const u8,
    blimit1: *const u8, limit1: *const u8, thresh1: *const u8,
) {
    aom_lpf_vertical_4_neon(s, pitch, blimit0, limit0, thresh0);
    aom_lpf_vertical_4_neon(s.offset(4 * pitch as isize), pitch, blimit1, limit1, thresh1);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_lpf_vertical_4_quad_neon(s: *mut u8, pitch: i32,
                                                             blimit: *const u8,
                                                             limit: *const u8,
                                                             thresh: *const u8) {
    aom_lpf_vertical_4_dual_neon(s, pitch, blimit, limit, thresh, blimit, limit, thresh);
    aom_lpf_vertical_4_dual_neon(s.offset(2 * MI_SIZE * pitch as isize), pitch, blimit, limit,
                                 thresh, blimit, limit, thresh);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_lpf_horizontal_14_neon(src: *mut u8, stride: i32,
                                                           blimit: *const u8,
                                                           limit: *const u8,
                                                           thresh: *const u8) {
    let stride = stride as isize;
    let mut p6q6: uint8x8_t = load_u8_4x2(src.offset(-7 * stride), 13 * stride);
    let mut p5q5: uint8x8_t = load_u8_4x2(src.offset(-6 * stride), 11 * stride);
    let mut p4q4: uint8x8_t = load_u8_4x2(src.offset(-5 * stride), 9 * stride);
    let mut p3q3: uint8x8_t = load_u8_4x2(src.offset(-4 * stride), 7 * stride);
    let mut p2q2: uint8x8_t = load_u8_4x2(src.offset(-3 * stride), 5 * stride);
    let mut p1q1: uint8x8_t = load_u8_4x2(src.offset(-2 * stride), 3 * stride);
    let mut p0q0: uint8x8_t = load_u8_4x2(src.offset(-1 * stride), 1 * stride);

    lpf_14_neon(&mut p6q6, &mut p5q5, &mut p4q4, &mut p3q3, &mut p2q2, &mut p1q1, &mut p0q0,
                *blimit, *limit, *thresh);

    store_u8x4_strided_x2(src.offset(-1 * stride), 1 * stride, p0q0);
    store_u8x4_strided_x2(src.offset(-2 * stride), 3 * stride, p1q1);
    store_u8x4_strided_x2(src.offset(-3 * stride), 5 * stride, p2q2);
    store_u8x4_strided_x2(src.offset(-4 * stride), 7 * stride, p3q3);
    store_u8x4_strided_x2(src.offset(-5 * stride), 9 * stride, p4q4);
    store_u8x4_strided_x2(src.offset(-6 * stride), 11 * stride, p5q5);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_lpf_horizontal_14_dual_neon(
    s: *mut u8, pitch: i32, blimit0: *const u8, limit0: *const u8, thresh0: *const u8,
    blimit1: *const u8, limit1: *const u8, thresh1: *const u8,
) {
    aom_lpf_horizontal_14_neon(s, pitch, blimit0, limit0, thresh0);
    aom_lpf_horizontal_14_neon(s.offset(4), pitch, blimit1, limit1, thresh1);
}

// TODO(any): Rewrite in NEON (similar to quad SSE2 functions) for better speed
// up.
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_lpf_horizontal_14_quad_neon(s: *mut u8, pitch: i32,
                                                                blimit: *const u8,
                                                                limit: *const u8,
                                                                thresh: *const u8) {
    aom_lpf_horizontal_14_dual_neon(s, pitch, blimit, limit, thresh, blimit, limit, thresh);
    aom_lpf_horizontal_14_dual_neon(s.offset(2 * MI_SIZE), pitch, blimit, limit, thresh, blimit,
                                    limit, thresh);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_lpf_horizontal_8_neon(src: *mut u8, stride: i32,
                                                          blimit: *const u8, limit: *const u8,
                                                          thresh: *const u8) {
    let stride = stride as isize;
    let mut p0q0: uint8x8_t;
    let mut p1q1: uint8x8_t;
    let mut p2q2: uint8x8_t;
    let mut p3q3: uint8x8_t;

    p3q3 = vreinterpret_u8_u32(vld1_dup_u32_unaligned(src.offset(-4 * stride) as *const u32));
    p2q2 = vreinterpret_u8_u32(vld1_dup_u32_unaligned(src.offset(-3 * stride) as *const u32));
    p1q1 = vreinterpret_u8_u32(vld1_dup_u32_unaligned(src.offset(-2 * stride) as *const u32));
    p0q0 = vreinterpret_u8_u32(vld1_dup_u32_unaligned(src.offset(-1 * stride) as *const u32));
    p0q0 = vreinterpret_u8_u32(vld1_lane_u32_unaligned::<1>(src.offset(0 * stride) as *const u32,
                                                            vreinterpret_u32_u8(p0q0)));
    p1q1 = vreinterpret_u8_u32(vld1_lane_u32_unaligned::<1>(src.offset(1 * stride) as *const u32,
                                                            vreinterpret_u32_u8(p1q1)));
    p2q2 = vreinterpret_u8_u32(vld1_lane_u32_unaligned::<1>(src.offset(2 * stride) as *const u32,
                                                            vreinterpret_u32_u8(p2q2)));
    p3q3 = vreinterpret_u8_u32(vld1_lane_u32_unaligned::<1>(src.offset(3 * stride) as *const u32,
                                                            vreinterpret_u32_u8(p3q3)));

    lpf_8_neon(&mut p3q3, &mut p2q2, &mut p1q1, &mut p0q0, *blimit, *limit, *thresh);

    vst1_lane_u32_unaligned::<0>(src.offset(-4 * stride) as *mut u32, vreinterpret_u32_u8(p3q3));
    vst1_lane_u32_unaligned::<0>(src.offset(-3 * stride) as *mut u32, vreinterpret_u32_u8(p2q2));
    vst1_lane_u32_unaligned::<0>(src.offset(-2 * stride) as *mut u32, vreinterpret_u32_u8(p1q1));
    vst1_lane_u32_unaligned::<0>(src.offset(-1 * stride) as *mut u32, vreinterpret_u32_u8(p0q0));
    vst1_lane_u32_unaligned::<1>(src.offset(0 * stride) as *mut u32, vreinterpret_u32_u8(p0q0));
    vst1_lane_u32_unaligned::<1>(src.offset(1 * stride) as *mut u32, vreinterpret_u32_u8(p1q1));
    vst1_lane_u32_unaligned::<1>(src.offset(2 * stride) as *mut u32, vreinterpret_u32_u8(p2q2));
    vst1_lane_u32_unaligned::<1>(src.offset(3 * stride) as *mut u32, vreinterpret_u32_u8(p3q3));
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_lpf_horizontal_8_dual_neon(
    s: *mut u8, pitch: i32, blimit0: *const u8, limit0: *const u8, thresh0: *const u8,
    blimit1: *const u8, limit1: *const u8, thresh1: *const u8,
) {
    aom_lpf_horizontal_8_neon(s, pitch, blimit0, limit0, thresh0);
    aom_lpf_horizontal_8_neon(s.offset(4), pitch, blimit1, limit1, thresh1);
}

// TODO(any): Rewrite in NEON (similar to quad SSE2 functions) for better speed
// up.
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_lpf_horizontal_8_quad_neon(s: *mut u8, pitch: i32,
                                                               blimit: *const u8,
                                                               limit: *const u8,
                                                               thresh: *const u8) {
    aom_lpf_horizontal_8_dual_neon(s, pitch, blimit, limit, thresh, blimit, limit, thresh);
    aom_lpf_horizontal_8_dual_neon(s.offset(2 * MI_SIZE), pitch, blimit, limit, thresh, blimit,
                                   limit, thresh);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_lpf_horizontal_6_neon(src: *mut u8, stride: i32,
                                                          blimit: *const u8, limit: *const u8,
                                                          thresh: *const u8) {
    let stride = stride as isize;
    let mut p0q0: uint8x8_t;
    let mut p1q1: uint8x8_t;
    let mut p2q2: uint8x8_t;

    p2q2 = vreinterpret_u8_u32(vld1_dup_u32_unaligned(src.offset(-3 * stride) as *const u32));
    p1q1 = vreinterpret_u8_u32(vld1_dup_u32_unaligned(src.offset(-2 * stride) as *const u32));
    p0q0 = vreinterpret_u8_u32(vld1_dup_u32_unaligned(src.offset(-1 * stride) as *const u32));
    p0q0 = vreinterpret_u8_u32(vld1_lane_u32_unaligned::<1>(src.offset(0 * stride) as *const u32,
                                                            vreinterpret_u32_u8(p0q0)));
    p1q1 = vreinterpret_u8_u32(vld1_lane_u32_unaligned::<1>(src.offset(1 * stride) as *const u32,
                                                            vreinterpret_u32_u8(p1q1)));
    p2q2 = vreinterpret_u8_u32(vld1_lane_u32_unaligned::<1>(src.offset(2 * stride) as *const u32,
                                                            vreinterpret_u32_u8(p2q2)));

    lpf_6_neon(&mut p2q2, &mut p1q1, &mut p0q0, *blimit, *limit, *thresh);

    vst1_lane_u32_unaligned::<0>(src.offset(-3 * stride) as *mut u32, vreinterpret_u32_u8(p2q2));
    vst1_lane_u32_unaligned::<0>(src.offset(-2 * stride) as *mut u32, vreinterpret_u32_u8(p1q1));
    vst1_lane_u32_unaligned::<0>(src.offset(-1 * stride) as *mut u32, vreinterpret_u32_u8(p0q0));
    vst1_lane_u32_unaligned::<1>(src.offset(0 * stride) as *mut u32, vreinterpret_u32_u8(p0q0));
    vst1_lane_u32_unaligned::<1>(src.offset(1 * stride) as *mut u32, vreinterpret_u32_u8(p1q1));
    vst1_lane_u32_unaligned::<1>(src.offset(2 * stride) as *mut u32, vreinterpret_u32_u8(p2q2));
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_lpf_horizontal_6_dual_neon(
    s: *mut u8, pitch: i32, blimit0: *const u8, limit0: *const u8, thresh0: *const u8,
    blimit1: *const u8, limit1: *const u8, thresh1: *const u8,
) {
    aom_lpf_horizontal_6_neon(s, pitch, blimit0, limit0, thresh0);
    aom_lpf_horizontal_6_neon(s.offset(4), pitch, blimit1, limit1, thresh1);
}

// TODO(any): Rewrite in NEON (similar to quad SSE2 functions) for better speed
// up.
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_lpf_horizontal_6_quad_neon(s: *mut u8, pitch: i32,
                                                               blimit: *const u8,
                                                               limit: *const u8,
                                                               thresh: *const u8) {
    aom_lpf_horizontal_6_dual_neon(s, pitch, blimit, limit, thresh, blimit, limit, thresh);
    aom_lpf_horizontal_6_dual_neon(s.offset(2 * MI_SIZE), pitch, blimit, limit, thresh, blimit,
                                   limit, thresh);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_lpf_horizontal_4_neon(src: *mut u8, stride: i32,
                                                          blimit: *const u8, limit: *const u8,
                                                          thresh: *const u8) {
    let stride = stride as isize;
    let mut p1q1: uint8x8_t = load_u8_4x2(src.offset(-2 * stride), 3 * stride);
    let mut p0q0: uint8x8_t = load_u8_4x2(src.offset(-1 * stride), 1 * stride);

    lpf_4_neon(&mut p1q1, &mut p0q0, *blimit, *limit, *thresh);

    store_u8x4_strided_x2(src.offset(-1 * stride), 1 * stride, p0q0);
    store_u8x4_strided_x2(src.offset(-2 * stride), 3 * stride, p1q1);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_lpf_horizontal_4_dual_neon(
    s: *mut u8, pitch: i32, blimit0: *const u8, limit0: *const u8, thresh0: *const u8,
    blimit1: *const u8, limit1: *const u8, thresh1: *const u8,
) {
    aom_lpf_horizontal_4_neon(s, pitch, blimit0, limit0, thresh0);
    aom_lpf_horizontal_4_neon(s.offset(4), pitch, blimit1, limit1, thresh1);
}

// TODO(any): Rewrite in NEON (similar to quad SSE2 functions) for better speed
// up.
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_lpf_horizontal_4_quad_neon(s: *mut u8, pitch: i32,
                                                               blimit: *const u8,
                                                               limit: *const u8,
                                                               thresh: *const u8) {
    aom_lpf_horizontal_4_dual_neon(s, pitch, blimit, limit, thresh, blimit, limit, thresh);
    aom_lpf_horizontal_4_dual_neon(s.offset(2 * MI_SIZE), pitch, blimit, limit, thresh, blimit,
                                   limit, thresh);
}
