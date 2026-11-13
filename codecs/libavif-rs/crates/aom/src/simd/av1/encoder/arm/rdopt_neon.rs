/*
 * Copyright (c) 2020, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

// Translated by hand from upstream/aom/av1/encoder/arm/rdopt_neon.c
// (ADR 0056). Intrinsic sequences, helper structure, and names follow the
// original; only the AOM_ARCH_AARCH64 branches are retained. C's integer
// promotions are explicit: int16_t operands of `*` and `+` are widened to
// int (i32) before the operation and the result to int64_t on accumulation.
// The final floating-point block is the same sequence of single-precision
// conversions and operations as the original (and as
// av1_get_horver_correlation_full_c), with no contraction (ADR 0024), and
// calls the C library's sqrtf as the translated C does.
#![allow(unused_mut, unused_parens, clippy::all)]

use core::arch::aarch64::*;

extern "C" {
    fn sqrtf(_: f32) -> f32;
}

// Process horizontal and vertical correlations in a 4x4 block of pixels.
// We actually use the 4x4 pixels to calculate correlations corresponding to
// the top-left 3x3 pixels, so this function must be called with 1x1 overlap,
// moving the window along/down by 3 pixels at a time.
#[inline(always)]
unsafe fn horver_correlation_4x4(
    diff: *const i16,
    stride: i32,
    xy_sum_32: *mut int32x4_t,
    xz_sum_32: *mut int32x4_t,
    x_sum_32: *mut int32x4_t,
    x2_sum_32: *mut int32x4_t,
) {
    // Pixels in this 4x4   [ a b c d ]
    // are referred to as:  [ e f g h ]
    //                      [ i j k l ]
    //                      [ m n o p ]

    let pixelsa_2_lo: int16x4_t = vld1_s16(diff.offset((0 * stride) as isize));
    let pixelsa_2_sli: int16x4_t =
        vreinterpret_s16_s64(vshl_n_s64::<16>(vreinterpret_s64_s16(pixelsa_2_lo)));
    let pixelsb_2_lo: int16x4_t = vld1_s16(diff.offset((1 * stride) as isize));
    let pixelsb_2_sli: int16x4_t =
        vreinterpret_s16_s64(vshl_n_s64::<16>(vreinterpret_s64_s16(pixelsb_2_lo)));
    let pixelsa_1_lo: int16x4_t = vld1_s16(diff.offset((2 * stride) as isize));
    let pixelsa_1_sli: int16x4_t =
        vreinterpret_s16_s64(vshl_n_s64::<16>(vreinterpret_s64_s16(pixelsa_1_lo)));
    let pixelsb_1_lo: int16x4_t = vld1_s16(diff.offset((3 * stride) as isize));
    let pixelsb_1_sli: int16x4_t =
        vreinterpret_s16_s64(vshl_n_s64::<16>(vreinterpret_s64_s16(pixelsb_1_lo)));

    let slli_a: int16x8_t = vcombine_s16(pixelsa_1_sli, pixelsa_2_sli);

    *xy_sum_32 = vmlal_s16(*xy_sum_32, pixelsa_1_lo, pixelsa_1_sli);
    *xy_sum_32 = vmlal_s16(*xy_sum_32, pixelsa_2_lo, pixelsa_2_sli);
    *xy_sum_32 = vmlal_s16(*xy_sum_32, pixelsb_2_lo, pixelsb_2_sli);

    *xz_sum_32 = vmlal_s16(*xz_sum_32, pixelsa_1_sli, pixelsb_1_sli);
    *xz_sum_32 = vmlal_s16(*xz_sum_32, pixelsa_2_sli, pixelsb_2_sli);
    *xz_sum_32 = vmlal_s16(*xz_sum_32, pixelsa_1_sli, pixelsb_2_sli);

    // Now calculate the straight sums, x_sum += a+b+c+e+f+g+i+j+k
    // (sum up every element in slli_a and swap_b)
    *x_sum_32 = vpadalq_s16(*x_sum_32, slli_a);
    *x_sum_32 = vaddw_s16(*x_sum_32, pixelsb_2_sli);

    // Also sum their squares
    *x2_sum_32 = vmlal_s16(*x2_sum_32, pixelsa_1_sli, pixelsa_1_sli);
    *x2_sum_32 = vmlal_s16(*x2_sum_32, pixelsa_2_sli, pixelsa_2_sli);
    *x2_sum_32 = vmlal_s16(*x2_sum_32, pixelsb_2_sli, pixelsb_2_sli);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn av1_get_horver_correlation_full_neon(
    diff: *const i16,
    stride: i32,
    width: i32,
    height: i32,
    hcorr: *mut f32,
    vcorr: *mut f32,
) {
    // `diff[k]` of the original.
    let d = |k: i32| -> i32 { *diff.offset(k as isize) as i32 };
    // The following notation is used:
    // x - current pixel
    // y - right neighbour pixel
    // z - below neighbour pixel
    // w - down-right neighbour pixel
    let mut xy_sum: i64 = 0;
    let mut xz_sum: i64 = 0;
    let mut x_sum: i64 = 0;
    let mut x2_sum: i64 = 0;
    let zero: int32x4_t = vdupq_n_s32(0);
    let mut v_x_sum: int64x2_t = vreinterpretq_s64_s32(zero);
    let mut v_xy_sum: int64x2_t = vreinterpretq_s64_s32(zero);
    let mut v_xz_sum: int64x2_t = vreinterpretq_s64_s32(zero);
    let mut v_x2_sum: int64x2_t = vreinterpretq_s64_s32(zero);
    // Process horizontal and vertical correlations through the body in 4x4
    // blocks.  This excludes the final row and column and possibly one extra
    // column depending how 3 divides into width and height

    let mut i: i32 = 0;
    while i <= height - 4 {
        let mut xy_sum_32: int32x4_t = zero;
        let mut xz_sum_32: int32x4_t = zero;
        let mut x_sum_32: int32x4_t = zero;
        let mut x2_sum_32: int32x4_t = zero;
        let mut j: i32 = 0;
        while j <= width - 4 {
            horver_correlation_4x4(diff.offset((i * stride + j) as isize), stride, &mut xy_sum_32,
                                   &mut xz_sum_32, &mut x_sum_32, &mut x2_sum_32);
            j += 3;
        }
        v_xy_sum = vpadalq_s32(v_xy_sum, xy_sum_32);
        v_xz_sum = vpadalq_s32(v_xz_sum, xz_sum_32);
        v_x_sum = vpadalq_s32(v_x_sum, x_sum_32);
        v_x2_sum = vpadalq_s32(v_x2_sum, x2_sum_32);
        i += 3;
    }
    xy_sum = vaddvq_s64(v_xy_sum);
    xz_sum = vaddvq_s64(v_xz_sum);
    x2_sum = vaddvq_s64(v_x2_sum);
    x_sum = vaddvq_s64(v_x_sum);
    // x_sum now covers every pixel except the final 1-2 rows and 1-2 cols
    let mut x_finalrow: i64 = 0;
    let mut x_finalcol: i64 = 0;
    let mut x2_finalrow: i64 = 0;
    let mut x2_finalcol: i64 = 0;

    // Do we have 2 rows remaining or just the one?  Note that width and height
    // are powers of 2, so each modulo 3 must be 1 or 2.
    if height % 3 == 1 {
        // Just horiz corrs on the final row
        let x0: i32 = d((height - 1) * stride);
        x_sum += x0 as i64;
        x_finalrow += x0 as i64;
        x2_sum += (x0 * x0) as i64;
        x2_finalrow += (x0 * x0) as i64;
        if width >= 8 {
            let mut v_y_sum: int32x4_t = zero;
            let mut v_y2_sum: int32x4_t = zero;
            let mut v_xy_sum_a: int32x4_t = zero;
            let mut k: i32 = width - 1;
            let mut j: i32 = 0;
            while (k - 8) > 0 {
                let v_x: int16x8_t = vld1q_s16(diff.offset(((height - 1) * stride + j) as isize));
                let v_y: int16x8_t =
                    vld1q_s16(diff.offset(((height - 1) * stride + j + 1) as isize));
                let v_x_lo: int16x4_t = vget_low_s16(v_x);
                let v_x_hi: int16x4_t = vget_high_s16(v_x);
                let v_y_lo: int16x4_t = vget_low_s16(v_y);
                let v_y_hi: int16x4_t = vget_high_s16(v_y);
                v_xy_sum_a = vmlal_s16(v_xy_sum_a, v_x_lo, v_y_lo);
                v_xy_sum_a = vmlal_s16(v_xy_sum_a, v_x_hi, v_y_hi);
                v_y2_sum = vmlal_s16(v_y2_sum, v_y_lo, v_y_lo);
                v_y2_sum = vmlal_s16(v_y2_sum, v_y_hi, v_y_hi);
                v_y_sum = vpadalq_s16(v_y_sum, v_y);
                k -= 8;
                j += 8;
            }

            let v_l: int16x8_t = vld1q_s16(diff.offset(((height - 1) * stride) as isize).offset(j as isize));
            let v_x: int16x8_t = vextq_s16::<1>(vextq_s16::<7>(vreinterpretq_s16_s32(zero), v_l),
                                                vreinterpretq_s16_s32(zero));
            let v_y: int16x8_t = vextq_s16::<1>(v_l, vreinterpretq_s16_s32(zero));
            let v_x_lo: int16x4_t = vget_low_s16(v_x);
            let v_x_hi: int16x4_t = vget_high_s16(v_x);
            let v_y_lo: int16x4_t = vget_low_s16(v_y);
            let v_y_hi: int16x4_t = vget_high_s16(v_y);
            v_xy_sum_a = vmlal_s16(v_xy_sum_a, v_x_lo, v_y_lo);
            v_xy_sum_a = vmlal_s16(v_xy_sum_a, v_x_hi, v_y_hi);
            v_y2_sum = vmlal_s16(v_y2_sum, v_y_lo, v_y_lo);
            v_y2_sum = vmlal_s16(v_y2_sum, v_y_hi, v_y_hi);
            let v_y_sum_a: int32x4_t = vpadalq_s16(v_y_sum, v_y);
            let v_xy_sum2: int64x2_t = vpaddlq_s32(v_xy_sum_a);
            let v_y2_sum_a: int64x2_t = vpaddlq_s32(v_y2_sum);
            xy_sum += vaddvq_s64(v_xy_sum2);
            let y: i32 = vaddvq_s32(v_y_sum_a);
            let y2: i64 = vaddvq_s64(v_y2_sum_a);
            x_sum += y as i64;
            x2_sum += y2;
            x_finalrow += y as i64;
            x2_finalrow += y2;
        } else {
            let mut j: i32 = 0;
            while j < width - 1 {
                let x: i32 = d((height - 1) * stride + j);
                let y: i32 = d((height - 1) * stride + j + 1);
                xy_sum += (x * y) as i64;
                x_sum += y as i64;
                x2_sum += (y * y) as i64;
                x_finalrow += y as i64;
                x2_finalrow += (y * y) as i64;
                j += 1;
            }
        }
    } else {
        // Two rows remaining to do
        let x0: i32 = d((height - 2) * stride);
        let z0: i32 = d((height - 1) * stride);
        x_sum += (x0 + z0) as i64;
        x2_sum += (x0 * x0 + z0 * z0) as i64;
        x_finalrow += z0 as i64;
        x2_finalrow += (z0 * z0) as i64;
        if width >= 8 {
            let mut v_y2_sum: int32x4_t = zero;
            let mut v_w2_sum: int32x4_t = zero;
            let mut v_xy_sum_a: int32x4_t = zero;
            let mut v_xz_sum_a: int32x4_t = zero;
            let mut v_x_sum_a: int32x4_t = zero;
            let mut v_w_sum: int32x4_t = zero;
            let mut k: i32 = width - 1;
            let mut j: i32 = 0;
            while (k - 8) > 0 {
                let v_x: int16x8_t = vld1q_s16(diff.offset(((height - 2) * stride + j) as isize));
                let v_y: int16x8_t =
                    vld1q_s16(diff.offset(((height - 2) * stride + j + 1) as isize));
                let v_z: int16x8_t = vld1q_s16(diff.offset(((height - 1) * stride + j) as isize));
                let v_w: int16x8_t =
                    vld1q_s16(diff.offset(((height - 1) * stride + j + 1) as isize));

                let v_x_lo: int16x4_t = vget_low_s16(v_x);
                let v_y_lo: int16x4_t = vget_low_s16(v_y);
                let v_z_lo: int16x4_t = vget_low_s16(v_z);
                let v_w_lo: int16x4_t = vget_low_s16(v_w);
                let v_x_hi: int16x4_t = vget_high_s16(v_x);
                let v_y_hi: int16x4_t = vget_high_s16(v_y);
                let v_z_hi: int16x4_t = vget_high_s16(v_z);
                let v_w_hi: int16x4_t = vget_high_s16(v_w);

                v_xy_sum_a = vmlal_s16(v_xy_sum_a, v_x_lo, v_y_lo);
                v_xy_sum_a = vmlal_s16(v_xy_sum_a, v_x_hi, v_y_hi);
                v_xy_sum_a = vmlal_s16(v_xy_sum_a, v_z_lo, v_w_lo);
                v_xy_sum_a = vmlal_s16(v_xy_sum_a, v_z_hi, v_w_hi);

                v_xz_sum_a = vmlal_s16(v_xz_sum_a, v_x_lo, v_z_lo);
                v_xz_sum_a = vmlal_s16(v_xz_sum_a, v_x_hi, v_z_hi);

                v_w2_sum = vmlal_s16(v_w2_sum, v_w_lo, v_w_lo);
                v_w2_sum = vmlal_s16(v_w2_sum, v_w_hi, v_w_hi);
                v_y2_sum = vmlal_s16(v_y2_sum, v_y_lo, v_y_lo);
                v_y2_sum = vmlal_s16(v_y2_sum, v_y_hi, v_y_hi);

                v_w_sum = vpadalq_s16(v_w_sum, v_w);
                v_x_sum_a = vpadalq_s16(v_x_sum_a, v_y);
                v_x_sum_a = vpadalq_s16(v_x_sum_a, v_w);

                k -= 8;
                j += 8;
            }
            let v_l: int16x8_t = vld1q_s16(diff.offset(((height - 2) * stride) as isize).offset(j as isize));
            let v_x: int16x8_t = vextq_s16::<1>(vextq_s16::<7>(vreinterpretq_s16_s32(zero), v_l),
                                                vreinterpretq_s16_s32(zero));
            let v_y: int16x8_t = vextq_s16::<1>(v_l, vreinterpretq_s16_s32(zero));
            let v_l_2: int16x8_t =
                vld1q_s16(diff.offset(((height - 1) * stride) as isize).offset(j as isize));
            let v_z: int16x8_t = vextq_s16::<1>(vextq_s16::<7>(vreinterpretq_s16_s32(zero), v_l_2),
                                                vreinterpretq_s16_s32(zero));
            let v_w: int16x8_t = vextq_s16::<1>(v_l_2, vreinterpretq_s16_s32(zero));

            let v_x_lo: int16x4_t = vget_low_s16(v_x);
            let v_y_lo: int16x4_t = vget_low_s16(v_y);
            let v_z_lo: int16x4_t = vget_low_s16(v_z);
            let v_w_lo: int16x4_t = vget_low_s16(v_w);
            let v_x_hi: int16x4_t = vget_high_s16(v_x);
            let v_y_hi: int16x4_t = vget_high_s16(v_y);
            let v_z_hi: int16x4_t = vget_high_s16(v_z);
            let v_w_hi: int16x4_t = vget_high_s16(v_w);

            v_xy_sum_a = vmlal_s16(v_xy_sum_a, v_x_lo, v_y_lo);
            v_xy_sum_a = vmlal_s16(v_xy_sum_a, v_x_hi, v_y_hi);
            v_xy_sum_a = vmlal_s16(v_xy_sum_a, v_z_lo, v_w_lo);
            v_xy_sum_a = vmlal_s16(v_xy_sum_a, v_z_hi, v_w_hi);

            v_xz_sum_a = vmlal_s16(v_xz_sum_a, v_x_lo, v_z_lo);
            v_xz_sum_a = vmlal_s16(v_xz_sum_a, v_x_hi, v_z_hi);

            v_w2_sum = vmlal_s16(v_w2_sum, v_w_lo, v_w_lo);
            v_w2_sum = vmlal_s16(v_w2_sum, v_w_hi, v_w_hi);
            v_y2_sum = vmlal_s16(v_y2_sum, v_y_lo, v_y_lo);
            v_y2_sum = vmlal_s16(v_y2_sum, v_y_hi, v_y_hi);

            v_w_sum = vpadalq_s16(v_w_sum, v_w);
            v_x_sum_a = vpadalq_s16(v_x_sum_a, v_y);
            v_x_sum_a = vpadalq_s16(v_x_sum_a, v_w);

            xy_sum += vaddvq_s64(vpaddlq_s32(v_xy_sum_a));
            xz_sum += vaddvq_s64(vpaddlq_s32(v_xz_sum_a));
            x_sum += vaddvq_s32(v_x_sum_a) as i64;
            x_finalrow += vaddvq_s32(v_w_sum) as i64;
            let y2: i64 = vaddvq_s64(vpaddlq_s32(v_y2_sum));
            let w2: i64 = vaddvq_s64(vpaddlq_s32(v_w2_sum));
            x2_sum += y2 + w2;
            x2_finalrow += w2;
        } else {
            let mut j: i32 = 0;
            while j < width - 1 {
                let x: i32 = d((height - 2) * stride + j);
                let y: i32 = d((height - 2) * stride + j + 1);
                let z: i32 = d((height - 1) * stride + j);
                let w: i32 = d((height - 1) * stride + j + 1);

                // Horizontal and vertical correlations for the penultimate row:
                xy_sum += (x * y) as i64;
                xz_sum += (x * z) as i64;

                // Now just horizontal correlations for the final row:
                xy_sum += (z * w) as i64;

                x_sum += (y + w) as i64;
                x2_sum += (y * y + w * w) as i64;
                x_finalrow += w as i64;
                x2_finalrow += (w * w) as i64;
                j += 1;
            }
        }
    }

    // Do we have 2 columns remaining or just the one?
    if width % 3 == 1 {
        // Just vert corrs on the final col
        let x0: i32 = d(width - 1);
        x_sum += x0 as i64;
        x_finalcol += x0 as i64;
        x2_sum += (x0 * x0) as i64;
        x2_finalcol += (x0 * x0) as i64;
        let mut i: i32 = 0;
        while i < height - 1 {
            let x: i32 = d(i * stride + width - 1);
            let z: i32 = d((i + 1) * stride + width - 1);
            xz_sum += (x * z) as i64;
            x_finalcol += z as i64;
            x2_finalcol += (z * z) as i64;
            // So the bottom-right elements don't get counted twice:
            if i < height - (if height % 3 == 1 { 2 } else { 3 }) {
                x_sum += z as i64;
                x2_sum += (z * z) as i64;
            }
            i += 1;
        }
    } else {
        // Two cols remaining
        let x0: i32 = d(width - 2);
        let y0: i32 = d(width - 1);
        x_sum += (x0 + y0) as i64;
        x2_sum += (x0 * x0 + y0 * y0) as i64;
        x_finalcol += y0 as i64;
        x2_finalcol += (y0 * y0) as i64;
        let mut i: i32 = 0;
        while i < height - 1 {
            let x: i32 = d(i * stride + width - 2);
            let y: i32 = d(i * stride + width - 1);
            let z: i32 = d((i + 1) * stride + width - 2);
            let w: i32 = d((i + 1) * stride + width - 1);

            // Horizontal and vertical correlations for the penultimate col:
            // Skip these on the last iteration of this loop if we also had two
            // rows remaining, otherwise the final horizontal and vertical correlation
            // get erroneously processed twice
            if i < height - 2 || height % 3 == 1 {
                xy_sum += (x * y) as i64;
                xz_sum += (x * z) as i64;
            }

            x_finalcol += w as i64;
            x2_finalcol += (w * w) as i64;
            // So the bottom-right elements don't get counted twice:
            if i < height - (if height % 3 == 1 { 2 } else { 3 }) {
                x_sum += (z + w) as i64;
                x2_sum += (z * z + w * w) as i64;
            }

            // Now just vertical correlations for the final column:
            xz_sum += (y * w) as i64;
            i += 1;
        }
    }

    // Calculate the simple sums and squared-sums
    let mut x_firstrow: i64 = 0;
    let mut x_firstcol: i64 = 0;
    let mut x2_firstrow: i64 = 0;
    let mut x2_firstcol: i64 = 0;

    if width >= 8 {
        let mut v_x_firstrow: int32x4_t = zero;
        let mut v_x2_firstrow: int32x4_t = zero;
        let mut j: i32 = 0;
        while j < width {
            let v_diff: int16x8_t = vld1q_s16(diff.offset(j as isize));
            let v_diff_lo: int16x4_t = vget_low_s16(v_diff);
            let v_diff_hi: int16x4_t = vget_high_s16(v_diff);
            v_x_firstrow = vpadalq_s16(v_x_firstrow, v_diff);
            v_x2_firstrow = vmlal_s16(v_x2_firstrow, v_diff_lo, v_diff_lo);
            v_x2_firstrow = vmlal_s16(v_x2_firstrow, v_diff_hi, v_diff_hi);
            j += 8;
        }
        x_firstrow += vaddvq_s32(v_x_firstrow) as i64;
        x2_firstrow += vaddvq_s32(v_x2_firstrow) as i64;
    } else {
        let mut j: i32 = 0;
        while j < width {
            x_firstrow += d(j) as i64;
            x2_firstrow += (d(j) * d(j)) as i64;
            j += 1;
        }
    }
    let mut i: i32 = 0;
    while i < height {
        x_firstcol += d(i * stride) as i64;
        x2_firstcol += (d(i * stride) * d(i * stride)) as i64;
        i += 1;
    }

    let xhor_sum: i64 = x_sum - x_finalcol;
    let xver_sum: i64 = x_sum - x_finalrow;
    let y_sum: i64 = x_sum - x_firstcol;
    let z_sum: i64 = x_sum - x_firstrow;
    let x2hor_sum: i64 = x2_sum - x2_finalcol;
    let x2ver_sum: i64 = x2_sum - x2_finalrow;
    let y2_sum: i64 = x2_sum - x2_firstcol;
    let z2_sum: i64 = x2_sum - x2_firstrow;

    let num_hor: f32 = (height * (width - 1)) as f32;
    let num_ver: f32 = ((height - 1) * width) as f32;

    let xhor_var_n: f32 = x2hor_sum as f32 - (xhor_sum * xhor_sum) as f32 / num_hor;
    let xver_var_n: f32 = x2ver_sum as f32 - (xver_sum * xver_sum) as f32 / num_ver;

    let y_var_n: f32 = y2_sum as f32 - (y_sum * y_sum) as f32 / num_hor;
    let z_var_n: f32 = z2_sum as f32 - (z_sum * z_sum) as f32 / num_ver;

    let xy_var_n: f32 = xy_sum as f32 - (xhor_sum * y_sum) as f32 / num_hor;
    let xz_var_n: f32 = xz_sum as f32 - (xver_sum * z_sum) as f32 / num_ver;

    if xhor_var_n > 0.0 && y_var_n > 0.0 {
        *hcorr = xy_var_n / sqrtf(xhor_var_n * y_var_n);
        *hcorr = if *hcorr < 0.0 { 0.0 } else { *hcorr };
    } else {
        *hcorr = 1.0;
    }
    if xver_var_n > 0.0 && z_var_n > 0.0 {
        *vcorr = xz_var_n / sqrtf(xver_var_n * z_var_n);
        *vcorr = if *vcorr < 0.0 { 0.0 } else { *vcorr };
    } else {
        *vcorr = 1.0;
    }
}
