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

// Translated by hand from upstream/aom/aom_dsp/arm/highbd_loopfilter_neon.c
// (ADR 0056). Function names, helper structure, and intrinsic sequences follow
// the original; only the AOM_ARCH_AARCH64 branch of permute_acdb64 is
// retained. C locals that the original leaves uninitialized start zeroed here.
// The `int` thresholds passed to `uint16_t` parameters keep C's implicit
// conversion (`as u16`).
#![allow(non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![allow(clippy::all)]

use core::arch::aarch64::*;

use crate::simd::aom_dsp__arm__mem_neon::*;
use crate::simd::aom_dsp__arm__transpose_neon::*;

#[inline(always)]
unsafe fn clip3_s16(val: int16x4_t, low: int16x4_t, high: int16x4_t) -> int16x4_t {
    vmin_s16(vmax_s16(val, low), high)
}

#[inline(always)]
unsafe fn convert_to_unsigned_pixel_u16(val: int16x8_t, bitdepth: i32) -> uint16x8_t {
    let low: int16x8_t = vdupq_n_s16(0);
    let high: uint16x8_t = vdupq_n_u16(((1 << bitdepth) - 1) as u16);

    vminq_u16(vreinterpretq_u16_s16(vmaxq_s16(val, low)), high)
}

// (abs(p1 - p0) > thresh) || (abs(q1 - q0) > thresh)
#[inline(always)]
unsafe fn hev(abd_p0p1_q0q1: uint16x8_t, thresh: u16) -> uint16x4_t {
    let a: uint16x8_t = vcgtq_u16(abd_p0p1_q0q1, vdupq_n_u16(thresh));
    vorr_u16(vget_low_u16(a), vget_high_u16(a))
}

// abs(p0 - q0) * 2 + abs(p1 - q1) / 2 <= outer_thresh
#[inline(always)]
unsafe fn outer_threshold(p1: uint16x4_t, p0: uint16x4_t, q0: uint16x4_t, q1: uint16x4_t,
                          outer_thresh: u16) -> uint16x4_t {
    let abd_p0q0: uint16x4_t = vabd_u16(p0, q0);
    let abd_p1q1: uint16x4_t = vabd_u16(p1, q1);
    let p0q0_double: uint16x4_t = vshl_n_u16::<1>(abd_p0q0);
    let p1q1_half: uint16x4_t = vshr_n_u16::<1>(abd_p1q1);
    let sum: uint16x4_t = vadd_u16(p0q0_double, p1q1_half);
    vcle_u16(sum, vdup_n_u16(outer_thresh))
}

// abs(p1 - p0) <= inner_thresh && abs(q1 - q0) <= inner_thresh &&
//   outer_threshold()
#[inline(always)]
unsafe fn needs_filter4(abd_p0p1_q0q1: uint16x8_t, inner_thresh: u16,
                        outer_mask: uint16x4_t) -> uint16x4_t {
    let a: uint16x8_t = vcleq_u16(abd_p0p1_q0q1, vdupq_n_u16(inner_thresh));
    let inner_mask: uint16x4_t = vand_u16(vget_low_u16(a), vget_high_u16(a));
    vand_u16(inner_mask, outer_mask)
}

// abs(p2 - p1) <= inner_thresh && abs(p1 - p0) <= inner_thresh &&
//   abs(q1 - q0) <= inner_thresh && abs(q2 - q1) <= inner_thresh &&
//   outer_threshold()
#[inline(always)]
unsafe fn needs_filter6(abd_p0p1_q0q1: uint16x8_t, abd_p1p2_q1q2: uint16x8_t,
                        inner_thresh: u16, outer_mask: uint16x4_t) -> uint16x4_t {
    let a: uint16x8_t = vmaxq_u16(abd_p0p1_q0q1, abd_p1p2_q1q2);
    let b: uint16x8_t = vcleq_u16(a, vdupq_n_u16(inner_thresh));
    let inner_mask: uint16x4_t = vand_u16(vget_low_u16(b), vget_high_u16(b));
    vand_u16(inner_mask, outer_mask)
}

// abs(p3 - p2) <= inner_thresh && abs(p2 - p1) <= inner_thresh &&
//   abs(p1 - p0) <= inner_thresh && abs(q1 - q0) <= inner_thresh &&
//   abs(q2 - q1) <= inner_thresh && abs(q3 - q2) <= inner_thresh
//   outer_threshold()
#[inline(always)]
unsafe fn needs_filter8(abd_p0p1_q0q1: uint16x8_t, abd_p1p2_q1q2: uint16x8_t,
                        abd_p2p3_q2q3: uint16x8_t, inner_thresh: u16,
                        outer_mask: uint16x4_t) -> uint16x4_t {
    let a: uint16x8_t = vmaxq_u16(abd_p0p1_q0q1, abd_p1p2_q1q2);
    let b: uint16x8_t = vmaxq_u16(a, abd_p2p3_q2q3);
    let c: uint16x8_t = vcleq_u16(b, vdupq_n_u16(inner_thresh));
    let inner_mask: uint16x4_t = vand_u16(vget_low_u16(c), vget_high_u16(c));
    vand_u16(inner_mask, outer_mask)
}

// -----------------------------------------------------------------------------
// filterN_masks functions.

#[inline(always)]
unsafe fn filter4_masks(p0q0: uint16x8_t, p1q1: uint16x8_t, hev_thresh: u16,
                        outer_mask: uint16x4_t, inner_thresh: u16, hev_mask: *mut uint16x4_t,
                        needs_filter4_mask: *mut uint16x4_t) {
    let p0p1_q0q1: uint16x8_t = vabdq_u16(p0q0, p1q1);
    // This includes cases where needs_filter4() is not true and so filter2() will
    // not be applied.
    let hev_tmp_mask: uint16x4_t = hev(p0p1_q0q1, hev_thresh);

    *needs_filter4_mask = needs_filter4(p0p1_q0q1, inner_thresh, outer_mask);

    // filter2() will only be applied if both needs_filter4() and hev() are true.
    *hev_mask = vand_u16(hev_tmp_mask, *needs_filter4_mask);
}

// abs(p1 - p0) <= flat_thresh && abs(q1 - q0) <= flat_thresh &&
//   abs(p2 - p0) <= flat_thresh && abs(q2 - q0) <= flat_thresh
// |flat_thresh| == 4 for 10 bit decode.
#[inline(always)]
unsafe fn is_flat3(abd_p0p1_q0q1: uint16x8_t, abd_p0p2_q0q2: uint16x8_t,
                   bitdepth: i32) -> uint16x4_t {
    let flat_thresh: i32 = 1 << (bitdepth - 8);
    let a: uint16x8_t = vmaxq_u16(abd_p0p1_q0q1, abd_p0p2_q0q2);
    let b: uint16x8_t = vcleq_u16(a, vdupq_n_u16(flat_thresh as u16));
    vand_u16(vget_low_u16(b), vget_high_u16(b))
}

#[inline(always)]
unsafe fn filter6_masks(p2q2: uint16x8_t, p1q1: uint16x8_t, p0q0: uint16x8_t, hev_thresh: u16,
                        outer_mask: uint16x4_t, inner_thresh: u16, bitdepth: i32,
                        needs_filter6_mask: *mut uint16x4_t, is_flat3_mask: *mut uint16x4_t,
                        hev_mask: *mut uint16x4_t) {
    let abd_p0p1_q0q1: uint16x8_t = vabdq_u16(p0q0, p1q1);
    *hev_mask = hev(abd_p0p1_q0q1, hev_thresh);
    *is_flat3_mask = is_flat3(abd_p0p1_q0q1, vabdq_u16(p0q0, p2q2), bitdepth);
    *needs_filter6_mask =
        needs_filter6(abd_p0p1_q0q1, vabdq_u16(p1q1, p2q2), inner_thresh, outer_mask);
}

// is_flat4 uses N=1, IsFlatOuter4 uses N=4.
// abs(p[N] - p0) <= flat_thresh && abs(q[N] - q0) <= flat_thresh &&
//   abs(p[N+1] - p0) <= flat_thresh && abs(q[N+1] - q0) <= flat_thresh &&
//   abs(p[N+2] - p0) <= flat_thresh && abs(q[N+1] - q0) <= flat_thresh
// |flat_thresh| == 4 for 10 bit decode.
#[inline(always)]
unsafe fn is_flat4(abd_pnp0_qnq0: uint16x8_t, abd_pn1p0_qn1q0: uint16x8_t,
                   abd_pn2p0_qn2q0: uint16x8_t, bitdepth: i32) -> uint16x4_t {
    let flat_thresh: i32 = 1 << (bitdepth - 8);
    let a: uint16x8_t = vmaxq_u16(abd_pnp0_qnq0, abd_pn1p0_qn1q0);
    let b: uint16x8_t = vmaxq_u16(a, abd_pn2p0_qn2q0);
    let c: uint16x8_t = vcleq_u16(b, vdupq_n_u16(flat_thresh as u16));
    vand_u16(vget_low_u16(c), vget_high_u16(c))
}

#[inline(always)]
unsafe fn filter8_masks(p3q3: uint16x8_t, p2q2: uint16x8_t, p1q1: uint16x8_t, p0q0: uint16x8_t,
                        hev_thresh: u16, outer_mask: uint16x4_t, inner_thresh: u16,
                        bitdepth: i32, needs_filter8_mask: *mut uint16x4_t,
                        is_flat4_mask: *mut uint16x4_t, hev_mask: *mut uint16x4_t) {
    let abd_p0p1_q0q1: uint16x8_t = vabdq_u16(p0q0, p1q1);
    *hev_mask = hev(abd_p0p1_q0q1, hev_thresh);
    let v_is_flat4: uint16x4_t =
        is_flat4(abd_p0p1_q0q1, vabdq_u16(p0q0, p2q2), vabdq_u16(p0q0, p3q3), bitdepth);
    *needs_filter8_mask = needs_filter8(abd_p0p1_q0q1, vabdq_u16(p1q1, p2q2),
                                        vabdq_u16(p2q2, p3q3), inner_thresh, outer_mask);
    // |is_flat4_mask| is used to decide where to use the result of filter8.
    // In rare cases, |is_flat4| can be true where |needs_filter8_mask| is false,
    // overriding the question of whether to use filter8. Because filter4 doesn't
    // apply to p2q2, |is_flat4_mask| chooses directly between filter8 and the
    // source value. To be correct, the mask must account for this override.
    *is_flat4_mask = vand_u16(v_is_flat4, *needs_filter8_mask);
}

// -----------------------------------------------------------------------------
// filterN functions.

// Calculate filter4() or filter2() based on |hev_mask|.
#[inline(always)]
unsafe fn filter4(p0q0: uint16x8_t, p0q1: uint16x8_t, p1q1: uint16x8_t, hev_mask: uint16x4_t,
                  bitdepth: i32, p1q1_result: *mut uint16x8_t, p0q0_result: *mut uint16x8_t) {
    let q0p1: uint16x8_t = vextq_u16::<4>(p0q0, p1q1);
    // a = 3 * (q0 - p0) + Clip3(p1 - q1, min_signed_val, max_signed_val);
    // q0mp0 means "q0 minus p0".
    let q0mp0_p1mq1: int16x8_t = vreinterpretq_s16_u16(vsubq_u16(q0p1, p0q1));
    let q0mp0_3: int16x4_t = vmul_n_s16(vget_low_s16(q0mp0_p1mq1), 3);

    // If this is for filter2() then include |p1mq1|. Otherwise zero it.
    let min_signed_pixel: int16x4_t = vdup_n_s16((-(1 << (bitdepth - 1))) as i16);
    let max_signed_pixel: int16x4_t = vdup_n_s16(((1 << (bitdepth - 1)) - 1) as i16);
    let p1mq1: int16x4_t = vget_high_s16(q0mp0_p1mq1);
    let p1mq1_saturated: int16x4_t = clip3_s16(p1mq1, min_signed_pixel, max_signed_pixel);
    let hev_option: int16x4_t = vand_s16(vreinterpret_s16_u16(hev_mask), p1mq1_saturated);

    let a: int16x4_t = vadd_s16(q0mp0_3, hev_option);

    // Need to figure out what's going on here because there are some unnecessary
    // tricks to accommodate 8x8 as smallest 8bpp vector

    // We can not shift with rounding because the clamp comes *before* the
    // shifting. a1 = Clip3(a + 4, min_signed_val, max_signed_val) >> 3; a2 =
    // Clip3(a + 3, min_signed_val, max_signed_val) >> 3;
    let plus_four: int16x4_t =
        clip3_s16(vadd_s16(a, vdup_n_s16(4)), min_signed_pixel, max_signed_pixel);
    let plus_three: int16x4_t =
        clip3_s16(vadd_s16(a, vdup_n_s16(3)), min_signed_pixel, max_signed_pixel);
    let a1: int16x4_t = vshr_n_s16::<3>(plus_four);
    let a2: int16x4_t = vshr_n_s16::<3>(plus_three);

    // a3 = (a1 + 1) >> 1;
    let a3: int16x4_t = vrshr_n_s16::<1>(a1);

    let a3_ma3: int16x8_t = vcombine_s16(a3, vneg_s16(a3));
    let p1q1_a3: int16x8_t = vaddq_s16(vreinterpretq_s16_u16(p1q1), a3_ma3);

    // Need to shift the second term or we end up with a2_ma2.
    let a2_ma1: int16x8_t = vcombine_s16(a2, vneg_s16(a1));
    let p0q0_a: int16x8_t = vaddq_s16(vreinterpretq_s16_u16(p0q0), a2_ma1);
    *p1q1_result = convert_to_unsigned_pixel_u16(p1q1_a3, bitdepth);
    *p0q0_result = convert_to_unsigned_pixel_u16(p0q0_a, bitdepth);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_lpf_horizontal_4_neon(s: *mut u16, pitch: i32,
                                                                 blimit: *const u8,
                                                                 limit: *const u8,
                                                                 thresh: *const u8, bd: i32) {
    let pitch = pitch as isize;
    let mut src: [uint16x4_t; 4] = [vdup_n_u16(0); 4];
    let [s0, s1, s2, s3] = &mut src;
    load_u16_4x4(s.offset(-2 * pitch), pitch, s0, s1, s2, s3);

    // Adjust thresholds to bitdepth.
    let outer_thresh: i32 = (*blimit as i32) << (bd - 8);
    let inner_thresh: i32 = (*limit as i32) << (bd - 8);
    let hev_thresh: i32 = (*thresh as i32) << (bd - 8);
    let outer_mask: uint16x4_t =
        outer_threshold(src[0], src[1], src[2], src[3], outer_thresh as u16);
    let mut hev_mask: uint16x4_t = vdup_n_u16(0);
    let mut needs_filter4_mask: uint16x4_t = vdup_n_u16(0);
    let p0q0: uint16x8_t = vcombine_u16(src[1], src[2]);
    let p1q1: uint16x8_t = vcombine_u16(src[0], src[3]);
    filter4_masks(p0q0, p1q1, hev_thresh as u16, outer_mask, inner_thresh as u16, &mut hev_mask,
                  &mut needs_filter4_mask);

    if vget_lane_u64::<0>(vreinterpret_u64_u16(needs_filter4_mask)) == 0 {
        // None of the values will be filtered.
        return;
    }

    // Copy the masks to the high bits for packed comparisons later.
    let hev_mask_8: uint16x8_t = vcombine_u16(hev_mask, hev_mask);
    let needs_filter4_mask_8: uint16x8_t = vcombine_u16(needs_filter4_mask, needs_filter4_mask);

    let mut f_p1q1: uint16x8_t = vdupq_n_u16(0);
    let mut f_p0q0: uint16x8_t = vdupq_n_u16(0);
    let p0q1: uint16x8_t = vcombine_u16(src[1], src[3]);
    filter4(p0q0, p0q1, p1q1, hev_mask, bd, &mut f_p1q1, &mut f_p0q0);

    // Already integrated the hev mask when calculating the filtered values.
    let p0q0_output: uint16x8_t = vbslq_u16(needs_filter4_mask_8, f_p0q0, p0q0);

    // p1/q1 are unmodified if only hev() is true. This works because it was and'd
    // with |needs_filter4_mask| previously.
    let p1q1_mask: uint16x8_t = veorq_u16(hev_mask_8, needs_filter4_mask_8);
    let p1q1_output: uint16x8_t = vbslq_u16(p1q1_mask, f_p1q1, p1q1);

    store_u16_4x4(s.offset(-2 * pitch), pitch, vget_low_u16(p1q1_output),
                  vget_low_u16(p0q0_output), vget_high_u16(p0q0_output),
                  vget_high_u16(p1q1_output));
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_lpf_horizontal_4_dual_neon(
    s: *mut u16, pitch: i32, blimit0: *const u8, limit0: *const u8, thresh0: *const u8,
    blimit1: *const u8, limit1: *const u8, thresh1: *const u8, bd: i32,
) {
    aom_highbd_lpf_horizontal_4_neon(s, pitch, blimit0, limit0, thresh0, bd);
    aom_highbd_lpf_horizontal_4_neon(s.offset(4), pitch, blimit1, limit1, thresh1, bd);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_lpf_vertical_4_neon(s: *mut u16, pitch: i32,
                                                               blimit: *const u8,
                                                               limit: *const u8,
                                                               thresh: *const u8, bd: i32) {
    let pitch = pitch as isize;
    // Offset by 2 uint16_t values to load from first p1 position.
    let mut src: [uint16x4_t; 4] = [vdup_n_u16(0); 4];
    {
        let [s0, s1, s2, s3] = &mut src;
        load_u16_4x4(s.offset(-2), pitch, s0, s1, s2, s3);
    }
    transpose_array_inplace_u16_4x4(src.as_mut_ptr());

    // Adjust thresholds to bitdepth.
    let outer_thresh: i32 = (*blimit as i32) << (bd - 8);
    let inner_thresh: i32 = (*limit as i32) << (bd - 8);
    let hev_thresh: i32 = (*thresh as i32) << (bd - 8);
    let outer_mask: uint16x4_t =
        outer_threshold(src[0], src[1], src[2], src[3], outer_thresh as u16);
    let mut hev_mask: uint16x4_t = vdup_n_u16(0);
    let mut needs_filter4_mask: uint16x4_t = vdup_n_u16(0);
    let p0q0: uint16x8_t = vcombine_u16(src[1], src[2]);
    let p1q1: uint16x8_t = vcombine_u16(src[0], src[3]);
    filter4_masks(p0q0, p1q1, hev_thresh as u16, outer_mask, inner_thresh as u16, &mut hev_mask,
                  &mut needs_filter4_mask);

    if vget_lane_u64::<0>(vreinterpret_u64_u16(needs_filter4_mask)) == 0 {
        // None of the values will be filtered.
        return;
    }

    // Copy the masks to the high bits for packed comparisons later.
    let hev_mask_8: uint16x8_t = vcombine_u16(hev_mask, hev_mask);
    let needs_filter4_mask_8: uint16x8_t = vcombine_u16(needs_filter4_mask, needs_filter4_mask);

    let mut f_p1q1: uint16x8_t = vdupq_n_u16(0);
    let mut f_p0q0: uint16x8_t = vdupq_n_u16(0);
    let p0q1: uint16x8_t = vcombine_u16(src[1], src[3]);
    filter4(p0q0, p0q1, p1q1, hev_mask, bd, &mut f_p1q1, &mut f_p0q0);

    // Already integrated the hev mask when calculating the filtered values.
    let p0q0_output: uint16x8_t = vbslq_u16(needs_filter4_mask_8, f_p0q0, p0q0);

    // p1/q1 are unmodified if only hev() is true. This works because it was and'd
    // with |needs_filter4_mask| previously.
    let p1q1_mask: uint16x8_t = veorq_u16(hev_mask_8, needs_filter4_mask_8);
    let p1q1_output: uint16x8_t = vbslq_u16(p1q1_mask, f_p1q1, p1q1);

    let mut output: [uint16x4_t; 4] = [
        vget_low_u16(p1q1_output),
        vget_low_u16(p0q0_output),
        vget_high_u16(p0q0_output),
        vget_high_u16(p1q1_output),
    ];
    transpose_array_inplace_u16_4x4(output.as_mut_ptr());

    store_u16_4x4(s.offset(-2), pitch, output[0], output[1], output[2], output[3]);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_lpf_vertical_4_dual_neon(
    s: *mut u16, pitch: i32, blimit0: *const u8, limit0: *const u8, thresh0: *const u8,
    blimit1: *const u8, limit1: *const u8, thresh1: *const u8, bd: i32,
) {
    aom_highbd_lpf_vertical_4_neon(s, pitch, blimit0, limit0, thresh0, bd);
    aom_highbd_lpf_vertical_4_neon(s.offset(4 * pitch as isize), pitch, blimit1, limit1, thresh1,
                                   bd);
}

#[inline(always)]
unsafe fn filter6(p2q2: uint16x8_t, p1q1: uint16x8_t, p0q0: uint16x8_t,
                  p1q1_output: *mut uint16x8_t, p0q0_output: *mut uint16x8_t) {
    // Sum p1 and q1 output from opposite directions.
    // The formula is regrouped to allow 3 doubling operations to be combined.
    //
    // p1 = (3 * p2) + (2 * p1) + (2 * p0) + q0
    //      ^^^^^^^^
    // q1 = p0 + (2 * q0) + (2 * q1) + (3 * q2)
    //                                 ^^^^^^^^
    // p1q1 = p2q2 + 2 * (p2q2 + p1q1 + p0q0) + q0p0
    //                    ^^^^^^^^^^^
    let mut sum: uint16x8_t = vaddq_u16(p2q2, p1q1);

    // p1q1 = p2q2 + 2 * (p2q2 + p1q1 + p0q0) + q0p0
    //                                ^^^^^^
    sum = vaddq_u16(sum, p0q0);

    // p1q1 = p2q2 + 2 * (p2q2 + p1q1 + p0q0) + q0p0
    //        ^^^^^^                          ^^^^^^
    // Should dual issue with the left shift.
    let q0p0: uint16x8_t = vextq_u16::<4>(p0q0, p0q0);
    let outer_sum: uint16x8_t = vaddq_u16(p2q2, q0p0);
    // p1q1 = p2q2 + 2 * (p2q2 + p1q1 + p0q0) + q0p0
    //        ^^^^^^^^^^^                       ^^^^
    sum = vmlaq_n_u16(outer_sum, sum, 2);

    *p1q1_output = vrshrq_n_u16::<3>(sum);

    // Convert to p0 and q0 output:
    // p0 = p1 - (2 * p2) + q0 + q1
    // q0 = q1 - (2 * q2) + p0 + p1
    // p0q0 = p1q1 - (2 * p2q2) + q0p0 + q1p1
    //        ^^^^^^^^^^^^^^^^^
    sum = vmlsq_n_u16(sum, p2q2, 2);
    let q1p1: uint16x8_t = vextq_u16::<4>(p1q1, p1q1);
    sum = vaddq_u16(sum, vaddq_u16(q0p0, q1p1));

    *p0q0_output = vrshrq_n_u16::<3>(sum);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_lpf_horizontal_6_neon(s: *mut u16, pitch: i32,
                                                                 blimit: *const u8,
                                                                 limit: *const u8,
                                                                 thresh: *const u8, bd: i32) {
    let pitch = pitch as isize;
    let mut src: [uint16x4_t; 6] = [vdup_n_u16(0); 6];
    {
        let [s0, s1, s2, s3, s4, s5] = &mut src;
        load_u16_4x6(s.offset(-3 * pitch), pitch, s0, s1, s2, s3, s4, s5);
    }

    // Adjust thresholds to bitdepth.
    let outer_thresh: i32 = (*blimit as i32) << (bd - 8);
    let inner_thresh: i32 = (*limit as i32) << (bd - 8);
    let hev_thresh: i32 = (*thresh as i32) << (bd - 8);
    let outer_mask: uint16x4_t =
        outer_threshold(src[1], src[2], src[3], src[4], outer_thresh as u16);
    let mut hev_mask: uint16x4_t = vdup_n_u16(0);
    let mut needs_filter_mask: uint16x4_t = vdup_n_u16(0);
    let mut is_flat3_mask: uint16x4_t = vdup_n_u16(0);
    let p0q0: uint16x8_t = vcombine_u16(src[2], src[3]);
    let p1q1: uint16x8_t = vcombine_u16(src[1], src[4]);
    let p2q2: uint16x8_t = vcombine_u16(src[0], src[5]);
    filter6_masks(p2q2, p1q1, p0q0, hev_thresh as u16, outer_mask, inner_thresh as u16, bd,
                  &mut needs_filter_mask, &mut is_flat3_mask, &mut hev_mask);

    if vget_lane_u64::<0>(vreinterpret_u64_u16(needs_filter_mask)) == 0 {
        // None of the values will be filtered.
        return;
    }

    let mut p0q0_output: uint16x8_t;
    let mut p1q1_output: uint16x8_t;
    let mut f6_p1q1: uint16x8_t = vdupq_n_u16(0);
    let mut f6_p0q0: uint16x8_t = vdupq_n_u16(0);
    // Not needing filter4() at all is a very common case, so isolate it to avoid
    // needlessly computing filter4().
    if vget_lane_s64::<0>(vreinterpret_s64_u16(is_flat3_mask)) == -1
        && vget_lane_s64::<0>(vreinterpret_s64_u16(needs_filter_mask)) == -1
    {
        filter6(p2q2, p1q1, p0q0, &mut f6_p1q1, &mut f6_p0q0);
        p1q1_output = f6_p1q1;
        p0q0_output = f6_p0q0;
    } else {
        // Copy the masks to the high bits for packed comparisons later.
        let hev_mask_8: uint16x8_t = vcombine_u16(hev_mask, hev_mask);
        let is_flat3_mask_8: uint16x8_t = vcombine_u16(is_flat3_mask, is_flat3_mask);
        let needs_filter_mask_8: uint16x8_t = vcombine_u16(needs_filter_mask, needs_filter_mask);

        let mut f4_p1q1: uint16x8_t = vdupq_n_u16(0);
        let mut f4_p0q0: uint16x8_t = vdupq_n_u16(0);
        let p0q1: uint16x8_t = vcombine_u16(src[2], src[4]);
        filter4(p0q0, p0q1, p1q1, hev_mask, bd, &mut f4_p1q1, &mut f4_p0q0);
        f4_p1q1 = vbslq_u16(hev_mask_8, p1q1, f4_p1q1);

        // Because we did not return after testing |needs_filter_mask| we know it is
        // nonzero. |is_flat3_mask| controls whether the needed filter is filter4 or
        // filter6. Therefore if it is false when |needs_filter_mask| is true,
        // filter6 output is not used.
        let need_filter6: uint64x1_t = vreinterpret_u64_u16(is_flat3_mask);
        if vget_lane_u64::<0>(need_filter6) == 0 {
            // filter6() does not apply, but filter4() applies to one or more values.
            p0q0_output = p0q0;
            p1q1_output = vbslq_u16(needs_filter_mask_8, f4_p1q1, p1q1);
            p0q0_output = vbslq_u16(needs_filter_mask_8, f4_p0q0, p0q0);
        } else {
            filter6(p2q2, p1q1, p0q0, &mut f6_p1q1, &mut f6_p0q0);
            p1q1_output = vbslq_u16(is_flat3_mask_8, f6_p1q1, f4_p1q1);
            p1q1_output = vbslq_u16(needs_filter_mask_8, p1q1_output, p1q1);
            p0q0_output = vbslq_u16(is_flat3_mask_8, f6_p0q0, f4_p0q0);
            p0q0_output = vbslq_u16(needs_filter_mask_8, p0q0_output, p0q0);
        }
    }

    store_u16_4x4(s.offset(-2 * pitch), pitch, vget_low_u16(p1q1_output),
                  vget_low_u16(p0q0_output), vget_high_u16(p0q0_output),
                  vget_high_u16(p1q1_output));
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_lpf_horizontal_6_dual_neon(
    s: *mut u16, pitch: i32, blimit0: *const u8, limit0: *const u8, thresh0: *const u8,
    blimit1: *const u8, limit1: *const u8, thresh1: *const u8, bd: i32,
) {
    aom_highbd_lpf_horizontal_6_neon(s, pitch, blimit0, limit0, thresh0, bd);
    aom_highbd_lpf_horizontal_6_neon(s.offset(4), pitch, blimit1, limit1, thresh1, bd);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_lpf_vertical_6_neon(s: *mut u16, pitch: i32,
                                                               blimit: *const u8,
                                                               limit: *const u8,
                                                               thresh: *const u8, bd: i32) {
    let pitch = pitch as isize;
    // Overread by 2 values. These overreads become the high halves of src_raw[2]
    // and src_raw[3] after transpose.
    let mut src_raw: [uint16x8_t; 4] = [vdupq_n_u16(0); 4];
    {
        let [r0, r1, r2, r3] = &mut src_raw;
        load_u16_8x4(s.offset(-3), pitch, r0, r1, r2, r3);
    }

    transpose_array_inplace_u16_4x8(src_raw.as_mut_ptr());
    // p2, p1, p0, q0, q1, q2
    let src: [uint16x4_t; 6] = [
        vget_low_u16(src_raw[0]),
        vget_low_u16(src_raw[1]),
        vget_low_u16(src_raw[2]),
        vget_low_u16(src_raw[3]),
        vget_high_u16(src_raw[0]),
        vget_high_u16(src_raw[1]),
    ];

    // Adjust thresholds to bitdepth.
    let outer_thresh: i32 = (*blimit as i32) << (bd - 8);
    let inner_thresh: i32 = (*limit as i32) << (bd - 8);
    let hev_thresh: i32 = (*thresh as i32) << (bd - 8);
    let outer_mask: uint16x4_t =
        outer_threshold(src[1], src[2], src[3], src[4], outer_thresh as u16);
    let mut hev_mask: uint16x4_t = vdup_n_u16(0);
    let mut needs_filter_mask: uint16x4_t = vdup_n_u16(0);
    let mut is_flat3_mask: uint16x4_t = vdup_n_u16(0);
    let p0q0: uint16x8_t = vcombine_u16(src[2], src[3]);
    let p1q1: uint16x8_t = vcombine_u16(src[1], src[4]);
    let p2q2: uint16x8_t = vcombine_u16(src[0], src[5]);
    filter6_masks(p2q2, p1q1, p0q0, hev_thresh as u16, outer_mask, inner_thresh as u16, bd,
                  &mut needs_filter_mask, &mut is_flat3_mask, &mut hev_mask);

    if vget_lane_u64::<0>(vreinterpret_u64_u16(needs_filter_mask)) == 0 {
        // None of the values will be filtered.
        return;
    }

    let mut p0q0_output: uint16x8_t;
    let mut p1q1_output: uint16x8_t;
    // Because we did not return after testing |needs_filter_mask| we know it is
    // nonzero. |is_flat3_mask| controls whether the needed filter is filter4 or
    // filter6. Therefore if it is false when |needs_filter_mask| is true, filter6
    // output is not used.
    let mut f6_p1q1: uint16x8_t = vdupq_n_u16(0);
    let mut f6_p0q0: uint16x8_t = vdupq_n_u16(0);
    let need_filter6: uint64x1_t = vreinterpret_u64_u16(is_flat3_mask);
    // Not needing filter4() at all is a very common case, so isolate it to avoid
    // needlessly computing filter4().
    if vget_lane_s64::<0>(vreinterpret_s64_u16(is_flat3_mask)) == -1
        && vget_lane_s64::<0>(vreinterpret_s64_u16(needs_filter_mask)) == -1
    {
        filter6(p2q2, p1q1, p0q0, &mut f6_p1q1, &mut f6_p0q0);
        p1q1_output = f6_p1q1;
        p0q0_output = f6_p0q0;
    } else {
        // Copy the masks to the high bits for packed comparisons later.
        let hev_mask_8: uint16x8_t = vcombine_u16(hev_mask, hev_mask);
        let is_flat3_mask_8: uint16x8_t = vcombine_u16(is_flat3_mask, is_flat3_mask);
        let needs_filter_mask_8: uint16x8_t = vcombine_u16(needs_filter_mask, needs_filter_mask);

        let mut f4_p1q1: uint16x8_t = vdupq_n_u16(0);
        let mut f4_p0q0: uint16x8_t = vdupq_n_u16(0);
        // ZIP1 p0q0, p1q1 may perform better here.
        let p0q1: uint16x8_t = vcombine_u16(src[2], src[4]);
        filter4(p0q0, p0q1, p1q1, hev_mask, bd, &mut f4_p1q1, &mut f4_p0q0);
        f4_p1q1 = vbslq_u16(hev_mask_8, p1q1, f4_p1q1);
        if vget_lane_u64::<0>(need_filter6) == 0 {
            // filter6() does not apply, but filter4() applies to one or more values.
            p0q0_output = p0q0;
            p1q1_output = vbslq_u16(needs_filter_mask_8, f4_p1q1, p1q1);
            p0q0_output = vbslq_u16(needs_filter_mask_8, f4_p0q0, p0q0);
        } else {
            filter6(p2q2, p1q1, p0q0, &mut f6_p1q1, &mut f6_p0q0);
            p1q1_output = vbslq_u16(is_flat3_mask_8, f6_p1q1, f4_p1q1);
            p1q1_output = vbslq_u16(needs_filter_mask_8, p1q1_output, p1q1);
            p0q0_output = vbslq_u16(is_flat3_mask_8, f6_p0q0, f4_p0q0);
            p0q0_output = vbslq_u16(needs_filter_mask_8, p0q0_output, p0q0);
        }
    }

    let mut output: [uint16x4_t; 4] = [
        vget_low_u16(p1q1_output),
        vget_low_u16(p0q0_output),
        vget_high_u16(p0q0_output),
        vget_high_u16(p1q1_output),
    ];
    transpose_array_inplace_u16_4x4(output.as_mut_ptr());

    store_u16_4x4(s.offset(-2), pitch, output[0], output[1], output[2], output[3]);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_lpf_vertical_6_dual_neon(
    s: *mut u16, pitch: i32, blimit0: *const u8, limit0: *const u8, thresh0: *const u8,
    blimit1: *const u8, limit1: *const u8, thresh1: *const u8, bd: i32,
) {
    aom_highbd_lpf_vertical_6_neon(s, pitch, blimit0, limit0, thresh0, bd);
    aom_highbd_lpf_vertical_6_neon(s.offset(4 * pitch as isize), pitch, blimit1, limit1, thresh1,
                                   bd);
}

#[inline(always)]
unsafe fn filter8(p3q3: uint16x8_t, p2q2: uint16x8_t, p1q1: uint16x8_t, p0q0: uint16x8_t,
                  p2q2_output: *mut uint16x8_t, p1q1_output: *mut uint16x8_t,
                  p0q0_output: *mut uint16x8_t) {
    // Sum p2 and q2 output from opposite directions.
    // The formula is regrouped to allow 2 doubling operations to be combined.
    // p2 = (3 * p3) + (2 * p2) + p1 + p0 + q0
    //      ^^^^^^^^
    // q2 = p0 + q0 + q1 + (2 * q2) + (3 * q3)
    //                                ^^^^^^^^
    // p2q2 = p3q3 + 2 * (p3q3 + p2q2) + p1q1 + p0q0 + q0p0
    //                    ^^^^^^^^^^^
    let p23q23: uint16x8_t = vaddq_u16(p3q3, p2q2);

    // Add two other terms to make dual issue with shift more likely.
    // p2q2 = p3q3 + 2 * (p3q3 + p2q2) + p1q1 + p0q0 + q0p0
    //                                   ^^^^^^^^^^^
    let p01q01: uint16x8_t = vaddq_u16(p0q0, p1q1);

    // p2q2 = p3q3 + 2 * (p3q3 + p2q2) + p1q1 + p0q0 + q0p0
    //               ^^^^^             ^^^^^^^^^^^^^
    let mut sum: uint16x8_t = vmlaq_n_u16(p01q01, p23q23, 2);

    // p2q2 = p3q3 + 2 * (p3q3 + p2q2) + p1q1 + p0q0 + q0p0
    //        ^^^^^^
    sum = vaddq_u16(sum, p3q3);

    // p2q2 = p3q3 + 2 * (p3q3 + p2q2) + p1q1 + p0q0 + q0p0
    //                                               ^^^^^^
    let q0p0: uint16x8_t = vextq_u16::<4>(p0q0, p0q0);
    sum = vaddq_u16(sum, q0p0);

    *p2q2_output = vrshrq_n_u16::<3>(sum);

    // Convert to p1 and q1 output:
    // p1 = p2 - p3 - p2 + p1 + q1
    // q1 = q2 - q3 - q2 + q0 + p1
    sum = vsubq_u16(sum, p23q23);
    let q1p1: uint16x8_t = vextq_u16::<4>(p1q1, p1q1);
    sum = vaddq_u16(sum, vaddq_u16(p1q1, q1p1));

    *p1q1_output = vrshrq_n_u16::<3>(sum);

    // Convert to p0 and q0 output:
    // p0 = p1 - p3 - p1 + p0 + q2
    // q0 = q1 - q3 - q1 + q0 + p2
    sum = vsubq_u16(sum, vaddq_u16(p3q3, p1q1));
    let q2p2: uint16x8_t = vextq_u16::<4>(p2q2, p2q2);
    sum = vaddq_u16(sum, vaddq_u16(p0q0, q2p2));

    *p0q0_output = vrshrq_n_u16::<3>(sum);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_lpf_horizontal_8_neon(s: *mut u16, pitch: i32,
                                                                 blimit: *const u8,
                                                                 limit: *const u8,
                                                                 thresh: *const u8, bd: i32) {
    let pitch = pitch as isize;
    let mut src: [uint16x4_t; 8] = [vdup_n_u16(0); 8];
    {
        let [s0, s1, s2, s3, s4, s5, s6, s7] = &mut src;
        load_u16_4x8(s.offset(-4 * pitch), pitch, s0, s1, s2, s3, s4, s5, s6, s7);
    }

    // Adjust thresholds to bitdepth.
    let outer_thresh: i32 = (*blimit as i32) << (bd - 8);
    let inner_thresh: i32 = (*limit as i32) << (bd - 8);
    let hev_thresh: i32 = (*thresh as i32) << (bd - 8);
    let outer_mask: uint16x4_t =
        outer_threshold(src[2], src[3], src[4], src[5], outer_thresh as u16);
    let mut hev_mask: uint16x4_t = vdup_n_u16(0);
    let mut needs_filter_mask: uint16x4_t = vdup_n_u16(0);
    let mut is_flat4_mask: uint16x4_t = vdup_n_u16(0);
    let p0q0: uint16x8_t = vcombine_u16(src[3], src[4]);
    let p1q1: uint16x8_t = vcombine_u16(src[2], src[5]);
    let p2q2: uint16x8_t = vcombine_u16(src[1], src[6]);
    let p3q3: uint16x8_t = vcombine_u16(src[0], src[7]);
    filter8_masks(p3q3, p2q2, p1q1, p0q0, hev_thresh as u16, outer_mask, inner_thresh as u16,
                  bd, &mut needs_filter_mask, &mut is_flat4_mask, &mut hev_mask);

    if vget_lane_u64::<0>(vreinterpret_u64_u16(needs_filter_mask)) == 0 {
        // None of the values will be filtered.
        return;
    }

    let mut p0q0_output: uint16x8_t;
    let mut p1q1_output: uint16x8_t;
    let mut p2q2_output: uint16x8_t;
    let mut f8_p2q2: uint16x8_t = vdupq_n_u16(0);
    let mut f8_p1q1: uint16x8_t = vdupq_n_u16(0);
    let mut f8_p0q0: uint16x8_t = vdupq_n_u16(0);
    // Not needing filter4() at all is a very common case, so isolate it to avoid
    // needlessly computing filter4().
    if vget_lane_s64::<0>(vreinterpret_s64_u16(is_flat4_mask)) == -1
        && vget_lane_s64::<0>(vreinterpret_s64_u16(needs_filter_mask)) == -1
    {
        filter8(p3q3, p2q2, p1q1, p0q0, &mut f8_p2q2, &mut f8_p1q1, &mut f8_p0q0);
        p2q2_output = f8_p2q2;
        p1q1_output = f8_p1q1;
        p0q0_output = f8_p0q0;
    } else {
        // Copy the masks to the high bits for packed comparisons later.
        let hev_mask_8: uint16x8_t = vcombine_u16(hev_mask, hev_mask);
        let needs_filter_mask_8: uint16x8_t = vcombine_u16(needs_filter_mask, needs_filter_mask);

        let mut f4_p1q1: uint16x8_t = vdupq_n_u16(0);
        let mut f4_p0q0: uint16x8_t = vdupq_n_u16(0);
        let p0q1: uint16x8_t = vcombine_u16(src[3], src[5]);
        filter4(p0q0, p0q1, p1q1, hev_mask, bd, &mut f4_p1q1, &mut f4_p0q0);
        f4_p1q1 = vbslq_u16(hev_mask_8, p1q1, f4_p1q1);

        // Because we did not return after testing |needs_filter_mask| we know it is
        // nonzero. |is_flat4_mask| controls whether the needed filter is filter4 or
        // filter8. Therefore if it is false when |needs_filter_mask| is true,
        // filter8 output is not used.
        let need_filter8: uint64x1_t = vreinterpret_u64_u16(is_flat4_mask);
        if vget_lane_u64::<0>(need_filter8) == 0 {
            // filter8() does not apply, but filter4() applies to one or more values.
            p2q2_output = p2q2;
            p1q1_output = vbslq_u16(needs_filter_mask_8, f4_p1q1, p1q1);
            p0q0_output = vbslq_u16(needs_filter_mask_8, f4_p0q0, p0q0);
        } else {
            let is_flat4_mask_8: uint16x8_t = vcombine_u16(is_flat4_mask, is_flat4_mask);
            filter8(p3q3, p2q2, p1q1, p0q0, &mut f8_p2q2, &mut f8_p1q1, &mut f8_p0q0);
            p2q2_output = vbslq_u16(is_flat4_mask_8, f8_p2q2, p2q2);
            p1q1_output = vbslq_u16(is_flat4_mask_8, f8_p1q1, f4_p1q1);
            p1q1_output = vbslq_u16(needs_filter_mask_8, p1q1_output, p1q1);
            p0q0_output = vbslq_u16(is_flat4_mask_8, f8_p0q0, f4_p0q0);
            p0q0_output = vbslq_u16(needs_filter_mask_8, p0q0_output, p0q0);
        }
    }

    store_u16_4x6(s.offset(-3 * pitch), pitch, vget_low_u16(p2q2_output),
                  vget_low_u16(p1q1_output), vget_low_u16(p0q0_output),
                  vget_high_u16(p0q0_output), vget_high_u16(p1q1_output),
                  vget_high_u16(p2q2_output));
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_lpf_horizontal_8_dual_neon(
    s: *mut u16, pitch: i32, blimit0: *const u8, limit0: *const u8, thresh0: *const u8,
    blimit1: *const u8, limit1: *const u8, thresh1: *const u8, bd: i32,
) {
    aom_highbd_lpf_horizontal_8_neon(s, pitch, blimit0, limit0, thresh0, bd);
    aom_highbd_lpf_horizontal_8_neon(s.offset(4), pitch, blimit1, limit1, thresh1, bd);
}

#[inline(always)]
unsafe fn reverse_low_half(a: uint16x8_t) -> uint16x8_t {
    vcombine_u16(vrev64_u16(vget_low_u16(a)), vget_high_u16(a))
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_lpf_vertical_8_neon(s: *mut u16, pitch: i32,
                                                               blimit: *const u8,
                                                               limit: *const u8,
                                                               thresh: *const u8, bd: i32) {
    let pitch = pitch as isize;
    // src_raw[n] contains p3, p2, p1, p0, q0, q1, q2, q3 for row n.
    // To get desired pairs after transpose, one half should be reversed.
    let mut src: [uint16x8_t; 4] = [vdupq_n_u16(0); 4];
    {
        let [s0, s1, s2, s3] = &mut src;
        load_u16_8x4(s.offset(-4), pitch, s0, s1, s2, s3);
    }

    // src[0] = p0q0
    // src[1] = p1q1
    // src[2] = p2q2
    // src[3] = p3q3
    loop_filter_transpose_u16_4x8q(src.as_mut_ptr());

    // Adjust thresholds to bitdepth.
    let outer_thresh: i32 = (*blimit as i32) << (bd - 8);
    let inner_thresh: i32 = (*limit as i32) << (bd - 8);
    let hev_thresh: i32 = (*thresh as i32) << (bd - 8);
    let outer_mask: uint16x4_t = outer_threshold(vget_low_u16(src[1]), vget_low_u16(src[0]),
                                                 vget_high_u16(src[0]), vget_high_u16(src[1]),
                                                 outer_thresh as u16);
    let mut hev_mask: uint16x4_t = vdup_n_u16(0);
    let mut needs_filter_mask: uint16x4_t = vdup_n_u16(0);
    let mut is_flat4_mask: uint16x4_t = vdup_n_u16(0);
    let p0q0: uint16x8_t = src[0];
    let p1q1: uint16x8_t = src[1];
    let p2q2: uint16x8_t = src[2];
    let p3q3: uint16x8_t = src[3];
    filter8_masks(p3q3, p2q2, p1q1, p0q0, hev_thresh as u16, outer_mask, inner_thresh as u16,
                  bd, &mut needs_filter_mask, &mut is_flat4_mask, &mut hev_mask);

    if vget_lane_u64::<0>(vreinterpret_u64_u16(needs_filter_mask)) == 0 {
        // None of the values will be filtered.
        return;
    }

    let mut p0q0_output: uint16x8_t;
    let mut p1q1_output: uint16x8_t;
    let mut p2q2_output: uint16x8_t;
    let mut f8_p2q2: uint16x8_t = vdupq_n_u16(0);
    let mut f8_p1q1: uint16x8_t = vdupq_n_u16(0);
    let mut f8_p0q0: uint16x8_t = vdupq_n_u16(0);
    // Not needing filter4() at all is a very common case, so isolate it to avoid
    // needlessly computing filter4().
    if vget_lane_s64::<0>(vreinterpret_s64_u16(is_flat4_mask)) == -1
        && vget_lane_s64::<0>(vreinterpret_s64_u16(needs_filter_mask)) == -1
    {
        filter8(p3q3, p2q2, p1q1, p0q0, &mut f8_p2q2, &mut f8_p1q1, &mut f8_p0q0);
        p2q2_output = f8_p2q2;
        p1q1_output = f8_p1q1;
        p0q0_output = f8_p0q0;
    } else {
        // Copy the masks to the high bits for packed comparisons later.
        let hev_mask_8: uint16x8_t = vcombine_u16(hev_mask, hev_mask);
        let needs_filter_mask_8: uint16x8_t = vcombine_u16(needs_filter_mask, needs_filter_mask);

        let mut f4_p1q1: uint16x8_t = vdupq_n_u16(0);
        let mut f4_p0q0: uint16x8_t = vdupq_n_u16(0);
        let p0q1: uint16x8_t = vcombine_u16(vget_low_u16(p0q0), vget_high_u16(p1q1));
        filter4(p0q0, p0q1, p1q1, hev_mask, bd, &mut f4_p1q1, &mut f4_p0q0);
        f4_p1q1 = vbslq_u16(hev_mask_8, p1q1, f4_p1q1);

        // Because we did not return after testing |needs_filter_mask| we know it is
        // nonzero. |is_flat4_mask| controls whether the needed filter is filter4 or
        // filter8. Therefore if it is false when |needs_filter_mask| is true,
        // filter8 output is not used.
        let need_filter8: uint64x1_t = vreinterpret_u64_u16(is_flat4_mask);
        if vget_lane_u64::<0>(need_filter8) == 0 {
            // filter8() does not apply, but filter4() applies to one or more values.
            p2q2_output = p2q2;
            p1q1_output = vbslq_u16(needs_filter_mask_8, f4_p1q1, p1q1);
            p0q0_output = vbslq_u16(needs_filter_mask_8, f4_p0q0, p0q0);
        } else {
            let is_flat4_mask_8: uint16x8_t = vcombine_u16(is_flat4_mask, is_flat4_mask);
            filter8(p3q3, p2q2, p1q1, p0q0, &mut f8_p2q2, &mut f8_p1q1, &mut f8_p0q0);
            p2q2_output = vbslq_u16(is_flat4_mask_8, f8_p2q2, p2q2);
            p1q1_output = vbslq_u16(is_flat4_mask_8, f8_p1q1, f4_p1q1);
            p1q1_output = vbslq_u16(needs_filter_mask_8, p1q1_output, p1q1);
            p0q0_output = vbslq_u16(is_flat4_mask_8, f8_p0q0, f4_p0q0);
            p0q0_output = vbslq_u16(needs_filter_mask_8, p0q0_output, p0q0);
        }
    }

    let mut output: [uint16x8_t; 4] = [p0q0_output, p1q1_output, p2q2_output, p3q3];
    // After transpose, |output| will contain rows of the form:
    // p0 p1 p2 p3 q0 q1 q2 q3
    transpose_array_inplace_u16_4x8(output.as_mut_ptr());

    // Reverse p values to produce original order:
    // p3 p2 p1 p0 q0 q1 q2 q3
    store_u16_8x4(s.offset(-4), pitch, reverse_low_half(output[0]), reverse_low_half(output[1]),
                  reverse_low_half(output[2]), reverse_low_half(output[3]));
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_lpf_vertical_8_dual_neon(
    s: *mut u16, pitch: i32, blimit0: *const u8, limit0: *const u8, thresh0: *const u8,
    blimit1: *const u8, limit1: *const u8, thresh1: *const u8, bd: i32,
) {
    aom_highbd_lpf_vertical_8_neon(s, pitch, blimit0, limit0, thresh0, bd);
    aom_highbd_lpf_vertical_8_neon(s.offset(4 * pitch as isize), pitch, blimit1, limit1, thresh1,
                                   bd);
}

#[inline(always)]
unsafe fn filter14(p6q6: uint16x8_t, p5q5: uint16x8_t, p4q4: uint16x8_t, p3q3: uint16x8_t,
                   p2q2: uint16x8_t, p1q1: uint16x8_t, p0q0: uint16x8_t,
                   p5q5_output: *mut uint16x8_t, p4q4_output: *mut uint16x8_t,
                   p3q3_output: *mut uint16x8_t, p2q2_output: *mut uint16x8_t,
                   p1q1_output: *mut uint16x8_t, p0q0_output: *mut uint16x8_t) {
    // Sum p5 and q5 output from opposite directions.
    // p5 = (7 * p6) + (2 * p5) + (2 * p4) + p3 + p2 + p1 + p0 + q0
    //      ^^^^^^^^
    // q5 = p0 + q0 + q1 + q2 + q3 + (2 * q4) + (2 * q5) + (7 * q6)
    //                                                     ^^^^^^^^
    let p6q6_x7: uint16x8_t = vsubq_u16(vshlq_n_u16::<3>(p6q6), p6q6);

    // p5 = (7 * p6) + (2 * p5) + (2 * p4) + p3 + p2 + p1 + p0 + q0
    //                 ^^^^^^^^^^^^^^^^^^^
    // q5 = p0 + q0 + q1 + q2 + q3 + (2 * q4) + (2 * q5) + (7 * q6)
    //                               ^^^^^^^^^^^^^^^^^^^
    let p45q45: uint16x8_t = vaddq_u16(p5q5, p4q4);
    let mut sum: uint16x8_t = vmlaq_n_u16(p6q6_x7, p45q45, 2);

    // p5 = (7 * p6) + (2 * p5) + (2 * p4) + p3 + p2 + p1 + p0 + q0
    //                                       ^^^^^^^
    // q5 = p0 + q0 + q1 + q2 + q3 + (2 * q4) + (2 * q5) + (7 * q6)
    //                     ^^^^^^^
    sum = vaddq_u16(vaddq_u16(p3q3, p2q2), sum);

    // p5 = (7 * p6) + (2 * p5) + (2 * p4) + p3 + p2 + p1 + p0 + q0
    //                                                 ^^^^^^^
    // q5 = p0 + q0 + q1 + q2 + q3 + (2 * q4) + (2 * q5) + (7 * q6)
    //           ^^^^^^^
    sum = vaddq_u16(vaddq_u16(p1q1, p0q0), sum);

    // p5 = (7 * p6) + (2 * p5) + (2 * p4) + p3 + p2 + p1 + p0 + q0
    //                                                           ^^
    // q5 = p0 + q0 + q1 + q2 + q3 + (2 * q4) + (2 * q5) + (7 * q6)
    //      ^^
    let q0p0: uint16x8_t = vextq_u16::<4>(p0q0, p0q0);
    sum = vaddq_u16(sum, q0p0);

    *p5q5_output = vrshrq_n_u16::<4>(sum);

    // Convert to p4 and q4 output:
    // p4 = p5 - (2 * p6) + p3 + q1
    // q4 = q5 - (2 * q6) + q3 + p1
    sum = vsubq_u16(sum, vshlq_n_u16::<1>(p6q6));
    let q1p1: uint16x8_t = vextq_u16::<4>(p1q1, p1q1);
    sum = vaddq_u16(vaddq_u16(p3q3, q1p1), sum);

    *p4q4_output = vrshrq_n_u16::<4>(sum);

    // Convert to p3 and q3 output:
    // p3 = p4 - p6 - p5 + p2 + q2
    // q3 = q4 - q6 - q5 + q2 + p2
    sum = vsubq_u16(sum, vaddq_u16(p6q6, p5q5));
    let q2p2: uint16x8_t = vextq_u16::<4>(p2q2, p2q2);
    sum = vaddq_u16(vaddq_u16(p2q2, q2p2), sum);

    *p3q3_output = vrshrq_n_u16::<4>(sum);

    // Convert to p2 and q2 output:
    // p2 = p3 - p6 - p4 + p1 + q3
    // q2 = q3 - q6 - q4 + q1 + p3
    sum = vsubq_u16(sum, vaddq_u16(p6q6, p4q4));
    let q3p3: uint16x8_t = vextq_u16::<4>(p3q3, p3q3);
    sum = vaddq_u16(vaddq_u16(p1q1, q3p3), sum);

    *p2q2_output = vrshrq_n_u16::<4>(sum);

    // Convert to p1 and q1 output:
    // p1 = p2 - p6 - p3 + p0 + q4
    // q1 = q2 - q6 - q3 + q0 + p4
    sum = vsubq_u16(sum, vaddq_u16(p6q6, p3q3));
    let q4p4: uint16x8_t = vextq_u16::<4>(p4q4, p4q4);
    sum = vaddq_u16(vaddq_u16(p0q0, q4p4), sum);

    *p1q1_output = vrshrq_n_u16::<4>(sum);

    // Convert to p0 and q0 output:
    // p0 = p1 - p6 - p2 + q0 + q5
    // q0 = q1 - q6 - q2 + p0 + p5
    sum = vsubq_u16(sum, vaddq_u16(p6q6, p2q2));
    let q5p5: uint16x8_t = vextq_u16::<4>(p5q5, p5q5);
    sum = vaddq_u16(vaddq_u16(q0p0, q5p5), sum);

    *p0q0_output = vrshrq_n_u16::<4>(sum);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_lpf_horizontal_14_neon(s: *mut u16, pitch: i32,
                                                                  blimit: *const u8,
                                                                  limit: *const u8,
                                                                  thresh: *const u8, bd: i32) {
    let pitch = pitch as isize;
    let mut src: [uint16x4_t; 14] = [vdup_n_u16(0); 14];
    {
        let [s0, s1, s2, s3, s4, s5, s6, s7, s8, s9, s10, s11, s12, s13] = &mut src;
        load_u16_4x14(s.offset(-7 * pitch), pitch, s0, s1, s2, s3, s4, s5, s6, s7, s8, s9, s10,
                      s11, s12, s13);
    }

    // Adjust thresholds to bitdepth.
    let outer_thresh: i32 = (*blimit as i32) << (bd - 8);
    let inner_thresh: i32 = (*limit as i32) << (bd - 8);
    let hev_thresh: i32 = (*thresh as i32) << (bd - 8);
    let outer_mask: uint16x4_t =
        outer_threshold(src[5], src[6], src[7], src[8], outer_thresh as u16);
    let mut hev_mask: uint16x4_t = vdup_n_u16(0);
    let mut needs_filter_mask: uint16x4_t = vdup_n_u16(0);
    let mut is_flat4_mask: uint16x4_t = vdup_n_u16(0);
    let p0q0: uint16x8_t = vcombine_u16(src[6], src[7]);
    let p1q1: uint16x8_t = vcombine_u16(src[5], src[8]);
    let p2q2: uint16x8_t = vcombine_u16(src[4], src[9]);
    let p3q3: uint16x8_t = vcombine_u16(src[3], src[10]);
    filter8_masks(p3q3, p2q2, p1q1, p0q0, hev_thresh as u16, outer_mask, inner_thresh as u16,
                  bd, &mut needs_filter_mask, &mut is_flat4_mask, &mut hev_mask);

    if vget_lane_u64::<0>(vreinterpret_u64_u16(needs_filter_mask)) == 0 {
        // None of the values will be filtered.
        return;
    }
    let p4q4: uint16x8_t = vcombine_u16(src[2], src[11]);
    let p5q5: uint16x8_t = vcombine_u16(src[1], src[12]);
    let p6q6: uint16x8_t = vcombine_u16(src[0], src[13]);
    // Mask to choose between the outputs of filter8 and filter14.
    // As with the derivation of |is_flat4_mask|, the question of whether to use
    // filter14 is only raised where |is_flat4_mask| is true.
    let is_flat4_outer_mask: uint16x4_t = vand_u16(
        is_flat4_mask,
        is_flat4(vabdq_u16(p0q0, p4q4), vabdq_u16(p0q0, p5q5), vabdq_u16(p0q0, p6q6), bd),
    );

    let mut p0q0_output: uint16x8_t;
    let mut p1q1_output: uint16x8_t;
    let mut p2q2_output: uint16x8_t;
    let mut p3q3_output: uint16x8_t;
    let mut p4q4_output: uint16x8_t;
    let mut p5q5_output: uint16x8_t;
    let mut f8_p2q2: uint16x8_t = vdupq_n_u16(0);
    let mut f8_p1q1: uint16x8_t = vdupq_n_u16(0);
    let mut f8_p0q0: uint16x8_t = vdupq_n_u16(0);
    let mut f14_p5q5: uint16x8_t = vdupq_n_u16(0);
    let mut f14_p4q4: uint16x8_t = vdupq_n_u16(0);
    let mut f14_p3q3: uint16x8_t = vdupq_n_u16(0);
    let mut f14_p2q2: uint16x8_t = vdupq_n_u16(0);
    let mut f14_p1q1: uint16x8_t = vdupq_n_u16(0);
    let mut f14_p0q0: uint16x8_t = vdupq_n_u16(0);
    if vget_lane_s64::<0>(vreinterpret_s64_u16(is_flat4_outer_mask)) == -1 {
        // filter14() applies to all values.
        filter14(p6q6, p5q5, p4q4, p3q3, p2q2, p1q1, p0q0, &mut f14_p5q5, &mut f14_p4q4,
                 &mut f14_p3q3, &mut f14_p2q2, &mut f14_p1q1, &mut f14_p0q0);
        p5q5_output = f14_p5q5;
        p4q4_output = f14_p4q4;
        p3q3_output = f14_p3q3;
        p2q2_output = f14_p2q2;
        p1q1_output = f14_p1q1;
        p0q0_output = f14_p0q0;
    } else if vget_lane_s64::<0>(vreinterpret_s64_u16(is_flat4_mask)) == -1
        && vget_lane_u64::<0>(vreinterpret_u64_u16(is_flat4_outer_mask)) == 0
    {
        // filter8() applies to all values.
        filter8(p3q3, p2q2, p1q1, p0q0, &mut f8_p2q2, &mut f8_p1q1, &mut f8_p0q0);
        p5q5_output = p5q5;
        p4q4_output = p4q4;
        p3q3_output = p3q3;
        p2q2_output = f8_p2q2;
        p1q1_output = f8_p1q1;
        p0q0_output = f8_p0q0;
    } else {
        // Copy the masks to the high bits for packed comparisons later.
        let hev_mask_8: uint16x8_t = vcombine_u16(hev_mask, hev_mask);
        let needs_filter_mask_8: uint16x8_t = vcombine_u16(needs_filter_mask, needs_filter_mask);

        let mut f4_p1q1: uint16x8_t = vdupq_n_u16(0);
        let mut f4_p0q0: uint16x8_t = vdupq_n_u16(0);
        let p0q1: uint16x8_t = vcombine_u16(src[6], src[8]);
        filter4(p0q0, p0q1, p1q1, hev_mask, bd, &mut f4_p1q1, &mut f4_p0q0);
        f4_p1q1 = vbslq_u16(hev_mask_8, p1q1, f4_p1q1);
        // Because we did not return after testing |needs_filter_mask| we know it is
        // nonzero. |is_flat4_mask| controls whether the needed filter is filter4 or
        // filter8. Therefore if it is false when |needs_filter_mask| is true,
        // filter8 output is not used.
        let need_filter8: uint64x1_t = vreinterpret_u64_u16(is_flat4_mask);
        if vget_lane_u64::<0>(need_filter8) == 0 {
            // filter8() and filter14() do not apply, but filter4() applies to one or
            // more values.
            p5q5_output = p5q5;
            p4q4_output = p4q4;
            p3q3_output = p3q3;
            p2q2_output = p2q2;
            p1q1_output = vbslq_u16(needs_filter_mask_8, f4_p1q1, p1q1);
            p0q0_output = vbslq_u16(needs_filter_mask_8, f4_p0q0, p0q0);
        } else {
            let use_filter8_mask: uint16x8_t = vcombine_u16(is_flat4_mask, is_flat4_mask);
            filter8(p3q3, p2q2, p1q1, p0q0, &mut f8_p2q2, &mut f8_p1q1, &mut f8_p0q0);
            let need_filter14: uint64x1_t = vreinterpret_u64_u16(is_flat4_outer_mask);
            if vget_lane_u64::<0>(need_filter14) == 0 {
                // filter14() does not apply, but filter8() and filter4() apply to one
                // or more values.
                p5q5_output = p5q5;
                p4q4_output = p4q4;
                p3q3_output = p3q3;
                p2q2_output = vbslq_u16(use_filter8_mask, f8_p2q2, p2q2);
                p1q1_output = vbslq_u16(use_filter8_mask, f8_p1q1, f4_p1q1);
                p1q1_output = vbslq_u16(needs_filter_mask_8, p1q1_output, p1q1);
                p0q0_output = vbslq_u16(use_filter8_mask, f8_p0q0, f4_p0q0);
                p0q0_output = vbslq_u16(needs_filter_mask_8, p0q0_output, p0q0);
            } else {
                // All filters may contribute values to final outputs.
                let use_filter14_mask: uint16x8_t =
                    vcombine_u16(is_flat4_outer_mask, is_flat4_outer_mask);
                filter14(p6q6, p5q5, p4q4, p3q3, p2q2, p1q1, p0q0, &mut f14_p5q5, &mut f14_p4q4,
                         &mut f14_p3q3, &mut f14_p2q2, &mut f14_p1q1, &mut f14_p0q0);
                p5q5_output = vbslq_u16(use_filter14_mask, f14_p5q5, p5q5);
                p4q4_output = vbslq_u16(use_filter14_mask, f14_p4q4, p4q4);
                p3q3_output = vbslq_u16(use_filter14_mask, f14_p3q3, p3q3);
                p2q2_output = vbslq_u16(use_filter14_mask, f14_p2q2, f8_p2q2);
                p2q2_output = vbslq_u16(use_filter8_mask, p2q2_output, p2q2);
                p2q2_output = vbslq_u16(needs_filter_mask_8, p2q2_output, p2q2);
                p1q1_output = vbslq_u16(use_filter14_mask, f14_p1q1, f8_p1q1);
                p1q1_output = vbslq_u16(use_filter8_mask, p1q1_output, f4_p1q1);
                p1q1_output = vbslq_u16(needs_filter_mask_8, p1q1_output, p1q1);
                p0q0_output = vbslq_u16(use_filter14_mask, f14_p0q0, f8_p0q0);
                p0q0_output = vbslq_u16(use_filter8_mask, p0q0_output, f4_p0q0);
                p0q0_output = vbslq_u16(needs_filter_mask_8, p0q0_output, p0q0);
            }
        }
    }

    store_u16_4x12(s.offset(-6 * pitch), pitch, vget_low_u16(p5q5_output),
                   vget_low_u16(p4q4_output), vget_low_u16(p3q3_output),
                   vget_low_u16(p2q2_output), vget_low_u16(p1q1_output),
                   vget_low_u16(p0q0_output), vget_high_u16(p0q0_output),
                   vget_high_u16(p1q1_output), vget_high_u16(p2q2_output),
                   vget_high_u16(p3q3_output), vget_high_u16(p4q4_output),
                   vget_high_u16(p5q5_output));
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_lpf_horizontal_14_dual_neon(
    s: *mut u16, pitch: i32, blimit0: *const u8, limit0: *const u8, thresh0: *const u8,
    blimit1: *const u8, limit1: *const u8, thresh1: *const u8, bd: i32,
) {
    aom_highbd_lpf_horizontal_14_neon(s, pitch, blimit0, limit0, thresh0, bd);
    aom_highbd_lpf_horizontal_14_neon(s.offset(4), pitch, blimit1, limit1, thresh1, bd);
}

#[inline(always)]
unsafe fn permute_acdb64(ab: uint16x8_t, cd: uint16x8_t) -> uint16x8x2_t {
    let mut acdb: uint16x8x2_t = uint16x8x2_t(vdupq_n_u16(0), vdupq_n_u16(0));
    // a[b] <- [c]d
    acdb.0 = vreinterpretq_u16_u64(vtrn1q_u64(vreinterpretq_u64_u16(ab),
                                              vreinterpretq_u64_u16(cd)));
    // [a]b <- c[d]
    acdb.1 = vreinterpretq_u16_u64(vtrn2q_u64(vreinterpretq_u64_u16(cd),
                                              vreinterpretq_u64_u16(ab)));
    acdb
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_lpf_vertical_14_neon(s: *mut u16, pitch: i32,
                                                                blimit: *const u8,
                                                                limit: *const u8,
                                                                thresh: *const u8, bd: i32) {
    let pitch = pitch as isize;
    // Low halves:  p7 p6 p5 p4
    // High halves: p3 p2 p1 p0
    let mut src_p: [uint16x8_t; 4] = [vdupq_n_u16(0); 4];
    {
        let [s0, s1, s2, s3] = &mut src_p;
        load_u16_8x4(s.offset(-8), pitch, s0, s1, s2, s3);
    }
    // p7 will be the low half of src_p[0]. Not used until the end.
    transpose_array_inplace_u16_4x8(src_p.as_mut_ptr());

    // Low halves:  q0 q1 q2 q3
    // High halves: q4 q5 q6 q7
    let mut src_q: [uint16x8_t; 4] = [vdupq_n_u16(0); 4];
    {
        let [s0, s1, s2, s3] = &mut src_q;
        load_u16_8x4(s, pitch, s0, s1, s2, s3);
    }
    // q7 will be the high half of src_q[3]. Not used until the end.
    transpose_array_inplace_u16_4x8(src_q.as_mut_ptr());

    // Adjust thresholds to bitdepth.
    let outer_thresh: i32 = (*blimit as i32) << (bd - 8);
    let inner_thresh: i32 = (*limit as i32) << (bd - 8);
    let hev_thresh: i32 = (*thresh as i32) << (bd - 8);
    let outer_mask: uint16x4_t = outer_threshold(vget_high_u16(src_p[2]),
                                                 vget_high_u16(src_p[3]),
                                                 vget_low_u16(src_q[0]), vget_low_u16(src_q[1]),
                                                 outer_thresh as u16);
    let p0q0: uint16x8_t = vextq_u16::<4>(src_p[3], src_q[0]);
    let p1q1: uint16x8_t = vextq_u16::<4>(src_p[2], src_q[1]);
    let p2q2: uint16x8_t = vextq_u16::<4>(src_p[1], src_q[2]);
    let p3q3: uint16x8_t = vextq_u16::<4>(src_p[0], src_q[3]);
    let mut hev_mask: uint16x4_t = vdup_n_u16(0);
    let mut needs_filter_mask: uint16x4_t = vdup_n_u16(0);
    let mut is_flat4_mask: uint16x4_t = vdup_n_u16(0);
    filter8_masks(p3q3, p2q2, p1q1, p0q0, hev_thresh as u16, outer_mask, inner_thresh as u16,
                  bd, &mut needs_filter_mask, &mut is_flat4_mask, &mut hev_mask);

    if vget_lane_u64::<0>(vreinterpret_u64_u16(needs_filter_mask)) == 0 {
        // None of the values will be filtered.
        return;
    }

    let p4q4: uint16x8_t = vcombine_u16(vget_low_u16(src_p[3]), vget_high_u16(src_q[0]));
    let p5q5: uint16x8_t = vcombine_u16(vget_low_u16(src_p[2]), vget_high_u16(src_q[1]));
    let p6q6: uint16x8_t = vcombine_u16(vget_low_u16(src_p[1]), vget_high_u16(src_q[2]));
    let p7q7: uint16x8_t = vcombine_u16(vget_low_u16(src_p[0]), vget_high_u16(src_q[3]));
    // Mask to choose between the outputs of filter8 and filter14.
    // As with the derivation of |is_flat4_mask|, the question of whether to use
    // filter14 is only raised where |is_flat4_mask| is true.
    let is_flat4_outer_mask: uint16x4_t = vand_u16(
        is_flat4_mask,
        is_flat4(vabdq_u16(p0q0, p4q4), vabdq_u16(p0q0, p5q5), vabdq_u16(p0q0, p6q6), bd),
    );

    let mut p0q0_output: uint16x8_t;
    let mut p1q1_output: uint16x8_t;
    let mut p2q2_output: uint16x8_t;
    let mut p3q3_output: uint16x8_t;
    let mut p4q4_output: uint16x8_t;
    let mut p5q5_output: uint16x8_t;
    let mut f8_p2q2: uint16x8_t = vdupq_n_u16(0);
    let mut f8_p1q1: uint16x8_t = vdupq_n_u16(0);
    let mut f8_p0q0: uint16x8_t = vdupq_n_u16(0);
    let mut f14_p5q5: uint16x8_t = vdupq_n_u16(0);
    let mut f14_p4q4: uint16x8_t = vdupq_n_u16(0);
    let mut f14_p3q3: uint16x8_t = vdupq_n_u16(0);
    let mut f14_p2q2: uint16x8_t = vdupq_n_u16(0);
    let mut f14_p1q1: uint16x8_t = vdupq_n_u16(0);
    let mut f14_p0q0: uint16x8_t = vdupq_n_u16(0);
    if vget_lane_s64::<0>(vreinterpret_s64_u16(is_flat4_outer_mask)) == -1 {
        // filter14() applies to all values.
        filter14(p6q6, p5q5, p4q4, p3q3, p2q2, p1q1, p0q0, &mut f14_p5q5, &mut f14_p4q4,
                 &mut f14_p3q3, &mut f14_p2q2, &mut f14_p1q1, &mut f14_p0q0);
        p5q5_output = f14_p5q5;
        p4q4_output = f14_p4q4;
        p3q3_output = f14_p3q3;
        p2q2_output = f14_p2q2;
        p1q1_output = f14_p1q1;
        p0q0_output = f14_p0q0;
    } else if vget_lane_s64::<0>(vreinterpret_s64_u16(is_flat4_mask)) == -1
        && vget_lane_u64::<0>(vreinterpret_u64_u16(is_flat4_outer_mask)) == 0
    {
        // filter8() applies to all values.
        filter8(p3q3, p2q2, p1q1, p0q0, &mut f8_p2q2, &mut f8_p1q1, &mut f8_p0q0);
        p5q5_output = p5q5;
        p4q4_output = p4q4;
        p3q3_output = p3q3;
        p2q2_output = f8_p2q2;
        p1q1_output = f8_p1q1;
        p0q0_output = f8_p0q0;
    } else {
        // Copy the masks to the high bits for packed comparisons later.
        let hev_mask_8: uint16x8_t = vcombine_u16(hev_mask, hev_mask);
        let needs_filter_mask_8: uint16x8_t = vcombine_u16(needs_filter_mask, needs_filter_mask);

        let mut f4_p1q1: uint16x8_t = vdupq_n_u16(0);
        let mut f4_p0q0: uint16x8_t = vdupq_n_u16(0);
        let p0q1: uint16x8_t = vcombine_u16(vget_low_u16(p0q0), vget_high_u16(p1q1));
        filter4(p0q0, p0q1, p1q1, hev_mask, bd, &mut f4_p1q1, &mut f4_p0q0);
        f4_p1q1 = vbslq_u16(hev_mask_8, p1q1, f4_p1q1);
        // Because we did not return after testing |needs_filter_mask| we know it is
        // nonzero. |is_flat4_mask| controls whether the needed filter is filter4 or
        // filter8. Therefore if it is false when |needs_filter_mask| is true,
        // filter8 output is not used.
        let need_filter8: uint64x1_t = vreinterpret_u64_u16(is_flat4_mask);
        if vget_lane_u64::<0>(need_filter8) == 0 {
            // filter8() and filter14() do not apply, but filter4() applies to one or
            // more values.
            p5q5_output = p5q5;
            p4q4_output = p4q4;
            p3q3_output = p3q3;
            p2q2_output = p2q2;
            p1q1_output = vbslq_u16(needs_filter_mask_8, f4_p1q1, p1q1);
            p0q0_output = vbslq_u16(needs_filter_mask_8, f4_p0q0, p0q0);
        } else {
            let use_filter8_mask: uint16x8_t = vcombine_u16(is_flat4_mask, is_flat4_mask);
            filter8(p3q3, p2q2, p1q1, p0q0, &mut f8_p2q2, &mut f8_p1q1, &mut f8_p0q0);
            let need_filter14: uint64x1_t = vreinterpret_u64_u16(is_flat4_outer_mask);
            if vget_lane_u64::<0>(need_filter14) == 0 {
                // filter14() does not apply, but filter8() and filter4() apply to one
                // or more values.
                p5q5_output = p5q5;
                p4q4_output = p4q4;
                p3q3_output = p3q3;
                p2q2_output = vbslq_u16(use_filter8_mask, f8_p2q2, p2q2);
                p1q1_output = vbslq_u16(use_filter8_mask, f8_p1q1, f4_p1q1);
                p1q1_output = vbslq_u16(needs_filter_mask_8, p1q1_output, p1q1);
                p0q0_output = vbslq_u16(use_filter8_mask, f8_p0q0, f4_p0q0);
                p0q0_output = vbslq_u16(needs_filter_mask_8, p0q0_output, p0q0);
            } else {
                // All filters may contribute values to final outputs.
                let use_filter14_mask: uint16x8_t =
                    vcombine_u16(is_flat4_outer_mask, is_flat4_outer_mask);
                filter14(p6q6, p5q5, p4q4, p3q3, p2q2, p1q1, p0q0, &mut f14_p5q5, &mut f14_p4q4,
                         &mut f14_p3q3, &mut f14_p2q2, &mut f14_p1q1, &mut f14_p0q0);
                p5q5_output = vbslq_u16(use_filter14_mask, f14_p5q5, p5q5);
                p4q4_output = vbslq_u16(use_filter14_mask, f14_p4q4, p4q4);
                p3q3_output = vbslq_u16(use_filter14_mask, f14_p3q3, p3q3);
                p2q2_output = vbslq_u16(use_filter14_mask, f14_p2q2, f8_p2q2);
                p2q2_output = vbslq_u16(use_filter8_mask, p2q2_output, p2q2);
                p2q2_output = vbslq_u16(needs_filter_mask_8, p2q2_output, p2q2);
                p1q1_output = vbslq_u16(use_filter14_mask, f14_p1q1, f8_p1q1);
                p1q1_output = vbslq_u16(use_filter8_mask, p1q1_output, f4_p1q1);
                p1q1_output = vbslq_u16(needs_filter_mask_8, p1q1_output, p1q1);
                p0q0_output = vbslq_u16(use_filter14_mask, f14_p0q0, f8_p0q0);
                p0q0_output = vbslq_u16(use_filter8_mask, p0q0_output, f4_p0q0);
                p0q0_output = vbslq_u16(needs_filter_mask_8, p0q0_output, p0q0);
            }
        }
    }

    // To get the correctly ordered rows from the transpose, we need:
    // p7p3 p6p2 p5p1 p4p0
    // q0q4 q1q5 q2q6 q3q7
    let p7p3_q3q7: uint16x8x2_t = permute_acdb64(p7q7, p3q3_output);
    let p6p2_q2q6: uint16x8x2_t = permute_acdb64(p6q6, p2q2_output);
    let p5p1_q1q5: uint16x8x2_t = permute_acdb64(p5q5_output, p1q1_output);
    let p4p0_q0q4: uint16x8x2_t = permute_acdb64(p4q4_output, p0q0_output);

    let mut output_p: [uint16x8_t; 4] = [p7p3_q3q7.0, p6p2_q2q6.0, p5p1_q1q5.0, p4p0_q0q4.0];
    let mut output_q: [uint16x8_t; 4] = [p4p0_q0q4.1, p5p1_q1q5.1, p6p2_q2q6.1, p7p3_q3q7.1];

    transpose_array_inplace_u16_4x8(output_p.as_mut_ptr());
    transpose_array_inplace_u16_4x8(output_q.as_mut_ptr());

    // Reverse p values to produce original order:
    // p3 p2 p1 p0 q0 q1 q2 q3
    store_u16_8x4(s.offset(-8), pitch, output_p[0], output_p[1], output_p[2], output_p[3]);
    store_u16_8x4(s, pitch, output_q[0], output_q[1], output_q[2], output_q[3]);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_lpf_vertical_14_dual_neon(
    s: *mut u16, pitch: i32, blimit0: *const u8, limit0: *const u8, thresh0: *const u8,
    blimit1: *const u8, limit1: *const u8, thresh1: *const u8, bd: i32,
) {
    aom_highbd_lpf_vertical_14_neon(s, pitch, blimit0, limit0, thresh0, bd);
    aom_highbd_lpf_vertical_14_neon(s.offset(4 * pitch as isize), pitch, blimit1, limit1,
                                    thresh1, bd);
}
