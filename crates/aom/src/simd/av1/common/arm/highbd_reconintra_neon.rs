/*
 * Copyright (c) 2023, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

// Translated by hand from upstream/aom/av1/common/arm/highbd_reconintra_neon.c
// (ADR 0056). Intrinsic sequences, helper structure, and names follow the
// original.
//
// Translation note: the stack scratch arrays `edge[]` are zero-initialised;
// the original leaves the padding beyond the copied edge uninitialised, and
// those lanes are either masked off or written past the compared edge in both
// versions.
use core::arch::aarch64::*;

const MAX_UPSAMPLE_SZ: usize = 16;

#[repr(C, align(16))]
struct Aligned16<T>(T);

#[no_mangle]
pub unsafe extern "C-unwind" fn av1_highbd_filter_intra_edge_neon(p: *mut u16, mut sz: i32, strength: i32) {
    if strength == 0 {
        return;
    }
    debug_assert!((0..=129).contains(&sz));

    static idx: Aligned16<[u16; 8]> = Aligned16([0, 1, 2, 3, 4, 5, 6, 7]);
    let index = vld1q_u16(idx.0.as_ptr());

    let mut edge = [0u16; 160]; // Max value of sz + enough padding for vector accesses.
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
            let s0 = vld1q_u16(src);
            let s1 = vld1q_u16(src.add(1));
            let s2 = vld1q_u16(src.add(2));

            // Make use of the identity:
            // (4*a + 8*b + 4*c) >> 4 == (a + (b << 1) + c) >> 2
            let t0 = vaddq_u16(s0, s2);
            let t1 = vaddq_u16(s1, s1);
            let sum = vaddq_u16(t0, t1);
            let res = vrshrq_n_u16::<2>(sum);

            vst1q_u16(dst, res);

            src = src.add(8);
            dst = dst.add(8);
            sz -= 8;
        }

        if sz > 0 {
            // Handle sz < 8 to avoid modifying out-of-bounds values.
            let s0 = vld1q_u16(src);
            let s1 = vld1q_u16(src.add(1));
            let s2 = vld1q_u16(src.add(2));

            // Make use of the identity:
            // (4*a + 8*b + 4*c) >> 4 == (a + (b << 1) + c) >> 2
            let t0 = vaddq_u16(s0, s2);
            let t1 = vaddq_u16(s1, s1);
            let sum = vaddq_u16(t0, t1);
            let mut res = vrshrq_n_u16::<2>(sum);

            // Mask off out-of-bounds indices.
            let current_dst = vld1q_u16(dst);
            let mask = vcgtq_u16(vdupq_n_u16(sz as u16), index);
            res = vbslq_u16(mask, res, current_dst);

            vst1q_u16(dst, res);
        }
    } else if strength == 2 {
        // Filter: {5, 6, 5}.
        let mut src = edge.as_ptr().add(1);

        let filter = uint16x8x3_t(vdupq_n_u16(5), vdupq_n_u16(6), vdupq_n_u16(5));
        while sz >= 8 {
            let s0 = vld1q_u16(src);
            let s1 = vld1q_u16(src.add(1));
            let s2 = vld1q_u16(src.add(2));

            let mut accum = vmulq_u16(s0, filter.0);
            accum = vmlaq_u16(accum, s1, filter.1);
            accum = vmlaq_u16(accum, s2, filter.2);
            let res = vrshrq_n_u16::<4>(accum);

            vst1q_u16(dst, res);

            src = src.add(8);
            dst = dst.add(8);
            sz -= 8;
        }

        if sz > 0 {
            // Handle sz < 8 to avoid modifying out-of-bounds values.
            let s0 = vld1q_u16(src);
            let s1 = vld1q_u16(src.add(1));
            let s2 = vld1q_u16(src.add(2));

            let mut accum = vmulq_u16(s0, filter.0);
            accum = vmlaq_u16(accum, s1, filter.1);
            accum = vmlaq_u16(accum, s2, filter.2);
            let mut res = vrshrq_n_u16::<4>(accum);

            // Mask off out-of-bounds indices.
            let current_dst = vld1q_u16(dst);
            let mask = vcgtq_u16(vdupq_n_u16(sz as u16), index);
            res = vbslq_u16(mask, res, current_dst);

            vst1q_u16(dst, res);
        }
    } else {
        // Filter {2, 4, 4, 4, 2}.
        let mut src = edge.as_ptr();

        while sz >= 8 {
            let s0 = vld1q_u16(src);
            let s1 = vld1q_u16(src.add(1));
            let s2 = vld1q_u16(src.add(2));
            let s3 = vld1q_u16(src.add(3));
            let s4 = vld1q_u16(src.add(4));

            // Make use of the identity:
            // (2*a + 4*b + 4*c + 4*d + 2*e) >> 4 == (a + ((b + c + d) << 1) + e) >> 3
            let t0 = vaddq_u16(s0, s4);
            let mut t1 = vaddq_u16(s1, s2);
            t1 = vaddq_u16(t1, s3);
            t1 = vaddq_u16(t1, t1);
            let sum = vaddq_u16(t0, t1);
            let res = vrshrq_n_u16::<3>(sum);

            vst1q_u16(dst, res);

            src = src.add(8);
            dst = dst.add(8);
            sz -= 8;
        }

        if sz > 0 {
            // Handle sz < 8 to avoid modifying out-of-bounds values.
            let s0 = vld1q_u16(src);
            let s1 = vld1q_u16(src.add(1));
            let s2 = vld1q_u16(src.add(2));
            let s3 = vld1q_u16(src.add(3));
            let s4 = vld1q_u16(src.add(4));

            // Make use of the identity:
            // (2*a + 4*b + 4*c + 4*d + 2*e) >> 4 == (a + ((b + c + d) << 1) + e) >> 3
            let t0 = vaddq_u16(s0, s4);
            let mut t1 = vaddq_u16(s1, s2);
            t1 = vaddq_u16(t1, s3);
            t1 = vaddq_u16(t1, t1);
            let sum = vaddq_u16(t0, t1);
            let mut res = vrshrq_n_u16::<3>(sum);

            // Mask off out-of-bounds indices.
            let current_dst = vld1q_u16(dst);
            let mask = vcgtq_u16(vdupq_n_u16(sz as u16), index);
            res = vbslq_u16(mask, res, current_dst);

            vst1q_u16(dst, res);
        }
    }
}

#[no_mangle]
pub unsafe extern "C-unwind" fn av1_highbd_upsample_intra_edge_neon(p: *mut u16, mut sz: i32, bd: i32) {
    if sz == 0 {
        return;
    }

    debug_assert!(sz as usize <= MAX_UPSAMPLE_SZ);

    let mut edge = [0u16; MAX_UPSAMPLE_SZ + 3];
    let mut src = edge.as_ptr();

    // Copy p[-1..(sz-1)] and pad out both ends.
    edge[0] = *p.offset(-1);
    edge[1] = *p.offset(-1);
    core::ptr::copy_nonoverlapping(p, edge.as_mut_ptr().add(2), sz as usize);
    edge[sz as usize + 2] = *p.offset(sz as isize - 1);
    *p.offset(-2) = *p.offset(-1);

    let pixel_val_max = vdupq_n_u16(((1 << bd) - 1) as u16);

    let mut dst = p.offset(-1);

    if bd == 12 {
        loop {
            let s0 = vld1q_u16(src);
            let s1 = vld1q_u16(src.add(1));
            let s2 = vld1q_u16(src.add(2));
            let s3 = vld1q_u16(src.add(3));

            let t0 = vaddq_u16(s1, s2);
            let t1 = vaddq_u16(s0, s3);
            let mut acc0 = vmull_n_u16(vget_low_u16(t0), 9);
            acc0 = vqsubq_u32(acc0, vmovl_u16(vget_low_u16(t1)));
            let mut acc1 = vmull_n_u16(vget_high_u16(t0), 9);
            acc1 = vqsubq_u32(acc1, vmovl_u16(vget_high_u16(t1)));

            let mut res = uint16x8x2_t(vdupq_n_u16(0), vdupq_n_u16(0));
            res.0 = vcombine_u16(vrshrn_n_u32::<4>(acc0), vrshrn_n_u32::<4>(acc1));
            // Clamp pixel values at bitdepth maximum.
            res.0 = vminq_u16(res.0, pixel_val_max);
            res.1 = s2;

            vst2q_u16(dst, res);

            src = src.wrapping_add(8);
            dst = dst.wrapping_add(16);
            sz -= 8;
            if sz <= 0 {
                break;
            }
        }
    } else {
        // Bit depth is 8 or 10.
        loop {
            let s0 = vld1q_u16(src);
            let s1 = vld1q_u16(src.add(1));
            let s2 = vld1q_u16(src.add(2));
            let s3 = vld1q_u16(src.add(3));

            let t0 = vaddq_u16(s0, s3);
            let mut t1 = vaddq_u16(s1, s2);
            t1 = vmulq_n_u16(t1, 9);
            t1 = vqsubq_u16(t1, t0);

            let mut res = uint16x8x2_t(vdupq_n_u16(0), vdupq_n_u16(0));
            res.0 = vrshrq_n_u16::<4>(t1);
            // Clamp pixel values at bitdepth maximum.
            res.0 = vminq_u16(res.0, pixel_val_max);
            res.1 = s2;

            vst2q_u16(dst, res);

            src = src.wrapping_add(8);
            dst = dst.wrapping_add(16);
            sz -= 8;
            if sz <= 0 {
                break;
            }
        }
    }
}
