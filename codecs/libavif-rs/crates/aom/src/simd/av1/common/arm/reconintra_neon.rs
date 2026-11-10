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

// Translated by hand from upstream/aom/av1/common/arm/reconintra_neon.c
// (ADR 0056). Intrinsic sequences, helper structure, and names follow the
// original.
//
// Translation notes:
// - `tx_size_wide` / `tx_size_high` (av1/common/common_data.h) are
//   header-local in the original and are transcribed below.
// - The stack scratch arrays `edge[]` are zero-initialised; the original
//   leaves the padding beyond the copied edge uninitialised, and those lanes
//   are either masked off or written past the compared edge in both versions.
use core::arch::aarch64::*;

use crate::simd::aom_dsp__arm__mem_neon::*;

const MAX_UPSAMPLE_SZ: usize = 16;
const FILTER_INTRA_SCALE_BITS: i32 = 4;
const FILTER_INTRA_MODES: usize = 5;

/// `TX_SIZE` from av1/common/enums.h.
pub type TX_SIZE = u8;

// Transcribed from upstream/aom/av1/common/common_data.h (header-local).
// Transform block width in pixels
static tx_size_wide: [i32; 19] = [4, 8, 16, 32, 64, 4, 8, 8, 16, 16, 32, 32, 64, 4, 16, 8, 32, 16, 64];

// Transcribed from upstream/aom/av1/common/common_data.h (header-local).
// Transform block height in pixels
static tx_size_high: [i32; 19] = [4, 8, 16, 32, 64, 8, 4, 16, 8, 32, 16, 64, 32, 16, 4, 32, 8, 64, 16];

// These kernels are a transposed version of those defined in reconintra.c,
// with the absolute value of the negatives taken in the top row.
#[repr(C, align(16))]
struct Aligned16<T>(T);

#[rustfmt::skip]
static av1_filter_intra_taps_neon: Aligned16<[[[u8; 8]; 7]; FILTER_INTRA_MODES]> = Aligned16([
    [
        [  6,  5,  3,  3,  4,  3,  3,  3 ],
        [ 10,  2,  1,  1,  6,  2,  2,  1 ],
        [  0, 10,  1,  1,  0,  6,  2,  2 ],
        [  0,  0, 10,  2,  0,  0,  6,  2 ],
        [  0,  0,  0, 10,  0,  0,  0,  6 ],
        [ 12,  9,  7,  5,  2,  2,  2,  3 ],
        [  0,  0,  0,  0, 12,  9,  7,  5 ],
    ],
    [
        [ 10,  6,  4,  2, 10,  6,  4,  2 ],
        [ 16,  0,  0,  0, 16,  0,  0,  0 ],
        [  0, 16,  0,  0,  0, 16,  0,  0 ],
        [  0,  0, 16,  0,  0,  0, 16,  0 ],
        [  0,  0,  0, 16,  0,  0,  0, 16 ],
        [ 10,  6,  4,  2,  0,  0,  0,  0 ],
        [  0,  0,  0,  0, 10,  6,  4,  2 ],
    ],
    [
        [  8,  8,  8,  8,  4,  4,  4,  4 ],
        [  8,  0,  0,  0,  4,  0,  0,  0 ],
        [  0,  8,  0,  0,  0,  4,  0,  0 ],
        [  0,  0,  8,  0,  0,  0,  4,  0 ],
        [  0,  0,  0,  8,  0,  0,  0,  4 ],
        [ 16, 16, 16, 16,  0,  0,  0,  0 ],
        [  0,  0,  0,  0, 16, 16, 16, 16 ],
    ],
    [
        [  2,  1,  1,  0,  1,  1,  1,  1 ],
        [  8,  3,  2,  1,  4,  3,  2,  2 ],
        [  0,  8,  3,  2,  0,  4,  3,  2 ],
        [  0,  0,  8,  3,  0,  0,  4,  3 ],
        [  0,  0,  0,  8,  0,  0,  0,  4 ],
        [ 10,  6,  4,  2,  3,  4,  4,  3 ],
        [  0,  0,  0,  0, 10,  6,  4,  3 ],
    ],
    [
        [ 12, 10,  9,  8, 10,  9,  8,  7 ],
        [ 14,  0,  0,  0, 12,  1,  0,  0 ],
        [  0, 14,  0,  0,  0, 12,  0,  0 ],
        [  0,  0, 14,  0,  0,  0, 12,  1 ],
        [  0,  0,  0, 14,  0,  0,  0, 12 ],
        [ 14, 12, 11, 10,  0,  0,  1,  1 ],
        [  0,  0,  0,  0, 14, 12, 11,  9 ],
    ],
]);

#[inline(always)]
#[allow(clippy::too_many_arguments)]
unsafe fn filter_intra_predictor(
    s0: uint8x8_t,
    s1: uint8x8_t,
    s2: uint8x8_t,
    s3: uint8x8_t,
    s4: uint8x8_t,
    s5: uint8x8_t,
    s6: uint8x8_t,
    f0: uint8x8_t,
    f1: uint8x8_t,
    f2: uint8x8_t,
    f3: uint8x8_t,
    f4: uint8x8_t,
    f5: uint8x8_t,
    f6: uint8x8_t,
) -> uint8x8_t {
    let mut acc = vmull_u8(s1, f1);
    // First row of each filter has all negative values so subtract.
    acc = vmlsl_u8(acc, s0, f0);
    acc = vmlal_u8(acc, s2, f2);
    acc = vmlal_u8(acc, s3, f3);
    acc = vmlal_u8(acc, s4, f4);
    acc = vmlal_u8(acc, s5, f5);
    acc = vmlal_u8(acc, s6, f6);

    vqrshrun_n_s16::<FILTER_INTRA_SCALE_BITS>(vreinterpretq_s16_u16(acc))
}

#[no_mangle]
pub unsafe extern "C-unwind" fn av1_filter_intra_predictor_neon(
    dst: *mut u8,
    stride: isize,
    tx_size: TX_SIZE,
    above: *const u8,
    left: *const u8,
    mode: i32,
) {
    let width = tx_size_wide[tx_size as usize];
    let height = tx_size_high[tx_size as usize];
    debug_assert!(width <= 32 && height <= 32);

    let taps = &av1_filter_intra_taps_neon.0[mode as usize];
    let f0 = vld1_u8(taps[0].as_ptr());
    let f1 = vld1_u8(taps[1].as_ptr());
    let f2 = vld1_u8(taps[2].as_ptr());
    let f3 = vld1_u8(taps[3].as_ptr());
    let f4 = vld1_u8(taps[4].as_ptr());
    let f5 = vld1_u8(taps[5].as_ptr());
    let f6 = vld1_u8(taps[6].as_ptr());

    // Computing 4 cols per iteration (instead of 8) for 8x<h> blocks is faster.
    if width <= 8 {
        let mut s0 = vdup_n_u8(*above.offset(-1));
        let mut s5 = vdup_n_u8(*left.add(0));
        let mut s6 = vdup_n_u8(*left.add(1));

        let mut c: i32 = 0;
        loop {
            let s1234 = load_unaligned_u8_4x1(above.offset(c as isize));
            let s1 = vdup_lane_u8::<0>(s1234);
            let s2 = vdup_lane_u8::<1>(s1234);
            let s3 = vdup_lane_u8::<2>(s1234);
            let s4 = vdup_lane_u8::<3>(s1234);

            let res = filter_intra_predictor(s0, s1, s2, s3, s4, s5, s6, f0, f1, f2, f3, f4, f5, f6);

            store_u8x4_strided_x2(dst.offset(c as isize), stride, res);

            s0 = s4;
            s5 = vdup_lane_u8::<3>(res);
            s6 = vdup_lane_u8::<7>(res);

            c += 4;
            if c >= width {
                break;
            }
        }

        let mut r: i32 = 2;
        loop {
            s0 = vdup_n_u8(*left.offset(r as isize - 1));
            s5 = vdup_n_u8(*left.offset(r as isize + 0));
            s6 = vdup_n_u8(*left.offset(r as isize + 1));

            c = 0;
            loop {
                let s1234 = load_u8_4x1(dst.offset((r as isize - 1) * stride + c as isize));
                let s1 = vdup_lane_u8::<0>(s1234);
                let s2 = vdup_lane_u8::<1>(s1234);
                let s3 = vdup_lane_u8::<2>(s1234);
                let s4 = vdup_lane_u8::<3>(s1234);

                let res = filter_intra_predictor(s0, s1, s2, s3, s4, s5, s6, f0, f1, f2, f3, f4, f5, f6);

                store_u8x4_strided_x2(dst.offset(r as isize * stride + c as isize), stride, res);

                s0 = s4;
                s5 = vdup_lane_u8::<3>(res);
                s6 = vdup_lane_u8::<7>(res);

                c += 4;
                if c >= width {
                    break;
                }
            }

            r += 2;
            if r >= height {
                break;
            }
        }
    } else {
        let mut s0_lo = vdup_n_u8(*above.offset(-1));
        let mut s5_lo = vdup_n_u8(*left.add(0));
        let mut s6_lo = vdup_n_u8(*left.add(1));

        let mut c: i32 = 0;
        loop {
            let s1234 = vld1_u8(above.offset(c as isize));
            let s1_lo = vdup_lane_u8::<0>(s1234);
            let s2_lo = vdup_lane_u8::<1>(s1234);
            let s3_lo = vdup_lane_u8::<2>(s1234);
            let s4_lo = vdup_lane_u8::<3>(s1234);

            let res_lo = filter_intra_predictor(s0_lo, s1_lo, s2_lo, s3_lo, s4_lo, s5_lo, s6_lo, f0, f1, f2, f3, f4, f5, f6);

            let s0_hi = s4_lo;
            let s1_hi = vdup_lane_u8::<4>(s1234);
            let s2_hi = vdup_lane_u8::<5>(s1234);
            let s3_hi = vdup_lane_u8::<6>(s1234);
            let s4_hi = vdup_lane_u8::<7>(s1234);
            let s5_hi = vdup_lane_u8::<3>(res_lo);
            let s6_hi = vdup_lane_u8::<7>(res_lo);

            let res_hi = filter_intra_predictor(s0_hi, s1_hi, s2_hi, s3_hi, s4_hi, s5_hi, s6_hi, f0, f1, f2, f3, f4, f5, f6);

            let res = vzip_u32(vreinterpret_u32_u8(res_lo), vreinterpret_u32_u8(res_hi));

            vst1_u8(dst.offset(0 * stride + c as isize), vreinterpret_u8_u32(res.0));
            vst1_u8(dst.offset(1 * stride + c as isize), vreinterpret_u8_u32(res.1));

            s0_lo = s4_hi;
            s5_lo = vdup_lane_u8::<3>(res_hi);
            s6_lo = vdup_lane_u8::<7>(res_hi);
            c += 8;
            if c >= width {
                break;
            }
        }

        let mut r: i32 = 2;
        loop {
            s0_lo = vdup_n_u8(*left.offset(r as isize - 1));
            s5_lo = vdup_n_u8(*left.offset(r as isize + 0));
            s6_lo = vdup_n_u8(*left.offset(r as isize + 1));

            c = 0;
            loop {
                let s1234 = vld1_u8(dst.offset((r as isize - 1) * stride + c as isize));
                let s1_lo = vdup_lane_u8::<0>(s1234);
                let s2_lo = vdup_lane_u8::<1>(s1234);
                let s3_lo = vdup_lane_u8::<2>(s1234);
                let s4_lo = vdup_lane_u8::<3>(s1234);

                let res_lo =
                    filter_intra_predictor(s0_lo, s1_lo, s2_lo, s3_lo, s4_lo, s5_lo, s6_lo, f0, f1, f2, f3, f4, f5, f6);

                let s0_hi = s4_lo;
                let s1_hi = vdup_lane_u8::<4>(s1234);
                let s2_hi = vdup_lane_u8::<5>(s1234);
                let s3_hi = vdup_lane_u8::<6>(s1234);
                let s4_hi = vdup_lane_u8::<7>(s1234);
                let s5_hi = vdup_lane_u8::<3>(res_lo);
                let s6_hi = vdup_lane_u8::<7>(res_lo);

                let res_hi =
                    filter_intra_predictor(s0_hi, s1_hi, s2_hi, s3_hi, s4_hi, s5_hi, s6_hi, f0, f1, f2, f3, f4, f5, f6);

                let res = vzip_u32(vreinterpret_u32_u8(res_lo), vreinterpret_u32_u8(res_hi));

                vst1_u8(dst.offset((r as isize + 0) * stride + c as isize), vreinterpret_u8_u32(res.0));
                vst1_u8(dst.offset((r as isize + 1) * stride + c as isize), vreinterpret_u8_u32(res.1));

                s0_lo = s4_hi;
                s5_lo = vdup_lane_u8::<3>(res_hi);
                s6_lo = vdup_lane_u8::<7>(res_hi);
                c += 8;
                if c >= width {
                    break;
                }
            }

            r += 2;
            if r >= height {
                break;
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C-unwind" fn av1_filter_intra_edge_neon(p: *mut u8, mut sz: i32, strength: i32) {
    if strength == 0 {
        return;
    }
    debug_assert!((0..=129).contains(&sz));

    let mut edge = [0u8; 160]; // Max value of sz + enough padding for vector accesses.
    core::ptr::copy_nonoverlapping(p, edge.as_mut_ptr().add(1), sz as usize);

    // Populate extra space appropriately.
    edge[0] = edge[1];
    edge[sz as usize + 1] = edge[sz as usize];
    edge[sz as usize + 2] = edge[sz as usize];

    // Don't overwrite first pixel.
    let mut dst = p.add(1);
    sz -= 1;

    if strength == 1 {
        // Filter: {4, 8, 4}.
        let mut src = edge.as_ptr().add(1);

        while sz >= 8 {
            let s0 = vld1_u8(src);
            let s1 = vld1_u8(src.add(1));
            let s2 = vld1_u8(src.add(2));

            // Make use of the identity:
            // (4*a + 8*b + 4*c) >> 4 == (a + (b << 1) + c) >> 2
            let t0 = vaddl_u8(s0, s2);
            let t1 = vaddl_u8(s1, s1);
            let sum = vaddq_u16(t0, t1);
            let res = vrshrn_n_u16::<2>(sum);

            vst1_u8(dst, res);

            src = src.add(8);
            dst = dst.add(8);
            sz -= 8;
        }

        if sz > 0 {
            // Handle sz < 8 to avoid modifying out-of-bounds values.
            let s0 = vld1_u8(src);
            let s1 = vld1_u8(src.add(1));
            let s2 = vld1_u8(src.add(2));

            let t0 = vaddl_u8(s0, s2);
            let t1 = vaddl_u8(s1, s1);
            let sum = vaddq_u16(t0, t1);
            let mut res = vrshrn_n_u16::<2>(sum);

            // Mask off out-of-bounds indices.
            let current_dst = vld1_u8(dst);
            let mask = vcgt_u8(vdup_n_u8(sz as u8), vcreate_u8(0x0706050403020100));
            res = vbsl_u8(mask, res, current_dst);

            vst1_u8(dst, res);
        }
    } else if strength == 2 {
        // Filter: {5, 6, 5}.
        let mut src = edge.as_ptr().add(1);

        let filter = uint8x8x3_t(vdup_n_u8(5), vdup_n_u8(6), vdup_n_u8(5));

        while sz >= 8 {
            let s0 = vld1_u8(src);
            let s1 = vld1_u8(src.add(1));
            let s2 = vld1_u8(src.add(2));

            let mut accum = vmull_u8(s0, filter.0);
            accum = vmlal_u8(accum, s1, filter.1);
            accum = vmlal_u8(accum, s2, filter.2);
            let res = vrshrn_n_u16::<4>(accum);

            vst1_u8(dst, res);

            src = src.add(8);
            dst = dst.add(8);
            sz -= 8;
        }

        if sz > 0 {
            // Handle sz < 8 to avoid modifying out-of-bounds values.
            let s0 = vld1_u8(src);
            let s1 = vld1_u8(src.add(1));
            let s2 = vld1_u8(src.add(2));

            let mut accum = vmull_u8(s0, filter.0);
            accum = vmlal_u8(accum, s1, filter.1);
            accum = vmlal_u8(accum, s2, filter.2);
            let mut res = vrshrn_n_u16::<4>(accum);

            // Mask off out-of-bounds indices.
            let current_dst = vld1_u8(dst);
            let mask = vcgt_u8(vdup_n_u8(sz as u8), vcreate_u8(0x0706050403020100));
            res = vbsl_u8(mask, res, current_dst);

            vst1_u8(dst, res);
        }
    } else {
        // Filter {2, 4, 4, 4, 2}.
        let mut src = edge.as_ptr();

        while sz >= 8 {
            let s0 = vld1_u8(src);
            let s1 = vld1_u8(src.add(1));
            let s2 = vld1_u8(src.add(2));
            let s3 = vld1_u8(src.add(3));
            let s4 = vld1_u8(src.add(4));

            // Make use of the identity:
            // (2*a + 4*b + 4*c + 4*d + 2*e) >> 4 == (a + ((b + c + d) << 1) + e) >> 3
            let t0 = vaddl_u8(s0, s4);
            let mut t1 = vaddl_u8(s1, s2);
            t1 = vaddw_u8(t1, s3);
            t1 = vaddq_u16(t1, t1);
            let sum = vaddq_u16(t0, t1);
            let res = vrshrn_n_u16::<3>(sum);

            vst1_u8(dst, res);

            src = src.add(8);
            dst = dst.add(8);
            sz -= 8;
        }

        if sz > 0 {
            // Handle sz < 8 to avoid modifying out-of-bounds values.
            let s0 = vld1_u8(src);
            let s1 = vld1_u8(src.add(1));
            let s2 = vld1_u8(src.add(2));
            let s3 = vld1_u8(src.add(3));
            let s4 = vld1_u8(src.add(4));

            let t0 = vaddl_u8(s0, s4);
            let mut t1 = vaddl_u8(s1, s2);
            t1 = vaddw_u8(t1, s3);
            t1 = vaddq_u16(t1, t1);
            let sum = vaddq_u16(t0, t1);
            let mut res = vrshrn_n_u16::<3>(sum);

            // Mask off out-of-bounds indices.
            let current_dst = vld1_u8(dst);
            let mask = vcgt_u8(vdup_n_u8(sz as u8), vcreate_u8(0x0706050403020100));
            res = vbsl_u8(mask, res, current_dst);

            vst1_u8(dst, res);
        }
    }
}

#[no_mangle]
pub unsafe extern "C-unwind" fn av1_upsample_intra_edge_neon(p: *mut u8, mut sz: i32) {
    if sz == 0 {
        return;
    }

    debug_assert!(sz as usize <= MAX_UPSAMPLE_SZ);

    let mut edge = [0u8; MAX_UPSAMPLE_SZ + 3];
    let mut src = edge.as_ptr();

    // Copy p[-1..(sz-1)] and pad out both ends.
    edge[0] = *p.offset(-1);
    edge[1] = *p.offset(-1);
    core::ptr::copy_nonoverlapping(p, edge.as_mut_ptr().add(2), sz as usize);
    edge[sz as usize + 2] = *p.offset(sz as isize - 1);
    *p.offset(-2) = *p.offset(-1);

    let mut dst = p.offset(-1);

    loop {
        let s0 = vld1_u8(src);
        let s1 = vld1_u8(src.add(1));
        let s2 = vld1_u8(src.add(2));
        let s3 = vld1_u8(src.add(3));

        let t0 = vreinterpretq_s16_u16(vaddl_u8(s0, s3));
        let mut t1 = vreinterpretq_s16_u16(vaddl_u8(s1, s2));
        t1 = vmulq_n_s16(t1, 9);
        t1 = vsubq_s16(t1, t0);

        let res = uint8x8x2_t(vqrshrun_n_s16::<4>(t1), s2);

        vst2_u8(dst, res);

        src = src.wrapping_add(8);
        dst = dst.wrapping_add(16);
        sz -= 8;
        if sz <= 0 {
            break;
        }
    }
}
