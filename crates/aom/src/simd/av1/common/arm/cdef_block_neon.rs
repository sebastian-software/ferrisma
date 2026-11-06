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

// Translated by hand from upstream/aom/av1/common/arm/cdef_block_neon.c
// (ADR 0056). Intrinsic sequences, helper structure, and names follow the
// original; only the AOM_ARCH_AARCH64 branches are retained. Pointer
// increments that carry a loop to its next row use wrapping_offset, because
// the C original advances past the last row after the final iteration.
use core::arch::aarch64::*;
use core::ffi::c_void;
use core::ptr::addr_of;

use crate::av1__common__cdef_block::{cdef_directions, cdef_pri_taps, cdef_sec_taps};
use crate::simd::aom_dsp__arm__mem_neon::*;
use crate::simd::aom_dsp__arm__sum_neon::*;

// av1/common/cdef_block.h (MAX_SB_SIZE_LOG2 = 7 from av1/common/enums.h):
// CDEF_BSTRIDE = ALIGN_POWER_OF_TWO((1 << MAX_SB_SIZE_LOG2) + 2 * CDEF_HBORDER, 3)
const CDEF_BSTRIDE: isize = 144;
// av1/common/cdef_block.h
const CDEF_VERY_LARGE: u16 = 0x4000;

// aom_ports/bitops.h get_msb(): the index of the most significant set bit.
#[inline(always)]
fn get_msb(n: u32) -> i32 {
    debug_assert!(n != 0);
    (31 ^ n.leading_zeros()) as i32
}

// aom_ports/bitops.h aom_clzll()
#[inline(always)]
fn aom_clzll(n: u64) -> i32 {
    debug_assert!(n != 0);
    n.leading_zeros() as i32
}

// AOMMAX(a, b) from aom_dsp/aom_dsp_common.h
#[inline(always)]
fn aommax(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

/// `cdef_directions[i][j]` from av1/common/cdef_block.c (a pointer two
/// entries into cdef_directions_padded, so `i` may range over -2..=9).
#[inline(always)]
unsafe fn cdef_dir(i: i32, j: usize) -> isize {
    (*cdef_directions.offset(i as isize))[j] as isize
}

#[inline(always)]
unsafe fn cdef_pri_taps_row(i: i32) -> *const i32 {
    (*addr_of!(cdef_pri_taps))[i as usize].as_ptr()
}

#[inline(always)]
unsafe fn cdef_sec_taps_ptr() -> *const i32 {
    (*addr_of!(cdef_sec_taps)).as_ptr()
}

#[no_mangle]
pub unsafe extern "C-unwind" fn cdef_copy_rect8_8bit_to_16bit_neon(
    mut dst: *mut u16,
    dstride: i32,
    mut src: *const u8,
    sstride: i32,
    width: i32,
    mut height: i32,
) {
    loop {
        let src_ptr = src;
        let dst_ptr = dst;

        let mut w: i32 = 0;
        while width - w >= 16 {
            let row = vld1q_u8(src_ptr.offset(w as isize));
            let row_u16 = uint8x16x2_t(row, vdupq_n_u8(0));
            vst2q_u8(dst_ptr.offset(w as isize) as *mut u8, row_u16);

            w += 16;
        }
        if width - w >= 8 {
            let row = vld1_u8(src_ptr.offset(w as isize));
            vst1q_u16(dst_ptr.offset(w as isize), vmovl_u8(row));
            w += 8;
        }
        if width - w == 4 {
            for i in w..w + 4 {
                *dst_ptr.offset(i as isize) = *src_ptr.offset(i as isize) as u16;
            }
        }

        src = src.wrapping_offset(sstride as isize);
        dst = dst.wrapping_offset(dstride as isize);
        height -= 1;
        if height == 0 {
            break;
        }
    }
}

// CONFIG_AV1_HIGHBITDEPTH
#[no_mangle]
pub unsafe extern "C-unwind" fn cdef_copy_rect8_16bit_to_16bit_neon(
    mut dst: *mut u16,
    dstride: i32,
    mut src: *const u16,
    sstride: i32,
    width: i32,
    mut height: i32,
) {
    loop {
        let src_ptr = src;
        let dst_ptr = dst;

        let mut w: i32 = 0;
        while width - w >= 8 {
            let row = vld1q_u16(src_ptr.offset(w as isize));
            vst1q_u16(dst_ptr.offset(w as isize), row);

            w += 8;
        }
        if width - w == 4 {
            let row = vld1_u16(src_ptr.offset(w as isize));
            vst1_u16(dst_ptr.offset(w as isize), row);
        }

        src = src.wrapping_offset(sstride as isize);
        dst = dst.wrapping_offset(dstride as isize);
        height -= 1;
        if height == 0 {
            break;
        }
    }
}

// partial A is a 16-bit vector of the form:
// [x8 x7 x6 x5 x4 x3 x2 x1] and partial B has the form:
// [0  y1 y2 y3 y4 y5 y6 y7].
// This function computes (x1^2+y1^2)*C1 + (x2^2+y2^2)*C2 + ...
// (x7^2+y2^7)*C7 + (x8^2+0^2)*C8 where the C1..C8 constants are in const1
// and const2.
#[inline(always)]
unsafe fn fold_mul_and_sum_neon(
    partiala: int16x8_t,
    mut partialb: int16x8_t,
    const1: uint32x4_t,
    const2: uint32x4_t,
) -> uint32x4_t {
    // Reverse partial B.
    // pattern = { 12 13 10 11 8 9 6 7 4 5 2 3 0 1 14 15 }.
    let pattern = vreinterpretq_u8_u64(vcombine_u64(
        vcreate_u64((0x07060908u64) << 32 | 0x0b0a0d0c),
        vcreate_u64((0x0f0e0100u64) << 32 | 0x03020504),
    ));

    partialb = vreinterpretq_s16_s8(vqtbl1q_s8(vreinterpretq_s8_s16(partialb), pattern));

    // Square and add the corresponding x and y values.
    let mut cost_lo = vmull_s16(vget_low_s16(partiala), vget_low_s16(partiala));
    cost_lo = vmlal_s16(cost_lo, vget_low_s16(partialb), vget_low_s16(partialb));
    let mut cost_hi = vmull_s16(vget_high_s16(partiala), vget_high_s16(partiala));
    cost_hi = vmlal_s16(cost_hi, vget_high_s16(partialb), vget_high_s16(partialb));

    // Multiply by constant.
    let mut cost = vmulq_u32(vreinterpretq_u32_s32(cost_lo), const1);
    cost = vmlaq_u32(cost, vreinterpretq_u32_s32(cost_hi), const2);
    cost
}

// This function computes the cost along directions 4, 5, 6, 7. (4 is diagonal
// down-right, 6 is vertical).
//
// For each direction the lines are shifted so that we can perform a
// basic sum on each vector element. For example, direction 5 is "south by
// southeast", so we need to add the pixels along each line i below:
//
// 0  1 2 3 4 5 6 7
// 0  1 2 3 4 5 6 7
// 8  0 1 2 3 4 5 6
// 8  0 1 2 3 4 5 6
// 9  8 0 1 2 3 4 5
// 9  8 0 1 2 3 4 5
// 10 9 8 0 1 2 3 4
// 10 9 8 0 1 2 3 4
//
// For this to fit nicely in vectors, the lines need to be shifted like so:
//        0 1 2 3 4 5 6 7
//        0 1 2 3 4 5 6 7
//      8 0 1 2 3 4 5 6
//      8 0 1 2 3 4 5 6
//    9 8 0 1 2 3 4 5
//    9 8 0 1 2 3 4 5
// 10 9 8 0 1 2 3 4
// 10 9 8 0 1 2 3 4
//
// In this configuration we can now perform SIMD additions to get the cost
// along direction 5. Since this won't fit into a single 128-bit vector, we use
// two of them to compute each half of the new configuration, and pad the empty
// spaces with zeros. Similar shifting is done for other directions, except
// direction 6 which is straightforward as it's the vertical direction.
#[inline(always)]
unsafe fn compute_vert_directions_neon(lines: *const int16x8_t, cost: *mut u32) -> uint32x4_t {
    let lines = core::slice::from_raw_parts(lines, 8);
    let zero = vdupq_n_s16(0);

    // Partial sums for lines 0 and 1.
    let mut partial4a = vextq_s16::<1>(zero, lines[0]);
    partial4a = vaddq_s16(partial4a, vextq_s16::<2>(zero, lines[1]));
    let mut partial4b = vextq_s16::<1>(lines[0], zero);
    partial4b = vaddq_s16(partial4b, vextq_s16::<2>(lines[1], zero));
    let mut tmp = vaddq_s16(lines[0], lines[1]);
    let mut partial5a = vextq_s16::<3>(zero, tmp);
    let mut partial5b = vextq_s16::<3>(tmp, zero);
    let mut partial7a = vextq_s16::<6>(zero, tmp);
    let mut partial7b = vextq_s16::<6>(tmp, zero);
    let mut partial6 = tmp;

    // Partial sums for lines 2 and 3.
    partial4a = vaddq_s16(partial4a, vextq_s16::<3>(zero, lines[2]));
    partial4a = vaddq_s16(partial4a, vextq_s16::<4>(zero, lines[3]));
    partial4b = vaddq_s16(partial4b, vextq_s16::<3>(lines[2], zero));
    partial4b = vaddq_s16(partial4b, vextq_s16::<4>(lines[3], zero));
    tmp = vaddq_s16(lines[2], lines[3]);
    partial5a = vaddq_s16(partial5a, vextq_s16::<4>(zero, tmp));
    partial5b = vaddq_s16(partial5b, vextq_s16::<4>(tmp, zero));
    partial7a = vaddq_s16(partial7a, vextq_s16::<5>(zero, tmp));
    partial7b = vaddq_s16(partial7b, vextq_s16::<5>(tmp, zero));
    partial6 = vaddq_s16(partial6, tmp);

    // Partial sums for lines 4 and 5.
    partial4a = vaddq_s16(partial4a, vextq_s16::<5>(zero, lines[4]));
    partial4a = vaddq_s16(partial4a, vextq_s16::<6>(zero, lines[5]));
    partial4b = vaddq_s16(partial4b, vextq_s16::<5>(lines[4], zero));
    partial4b = vaddq_s16(partial4b, vextq_s16::<6>(lines[5], zero));
    tmp = vaddq_s16(lines[4], lines[5]);
    partial5a = vaddq_s16(partial5a, vextq_s16::<5>(zero, tmp));
    partial5b = vaddq_s16(partial5b, vextq_s16::<5>(tmp, zero));
    partial7a = vaddq_s16(partial7a, vextq_s16::<4>(zero, tmp));
    partial7b = vaddq_s16(partial7b, vextq_s16::<4>(tmp, zero));
    partial6 = vaddq_s16(partial6, tmp);

    // Partial sums for lines 6 and 7.
    partial4a = vaddq_s16(partial4a, vextq_s16::<7>(zero, lines[6]));
    partial4a = vaddq_s16(partial4a, lines[7]);
    partial4b = vaddq_s16(partial4b, vextq_s16::<7>(lines[6], zero));
    tmp = vaddq_s16(lines[6], lines[7]);
    partial5a = vaddq_s16(partial5a, vextq_s16::<6>(zero, tmp));
    partial5b = vaddq_s16(partial5b, vextq_s16::<6>(tmp, zero));
    partial7a = vaddq_s16(partial7a, vextq_s16::<3>(zero, tmp));
    partial7b = vaddq_s16(partial7b, vextq_s16::<3>(tmp, zero));
    partial6 = vaddq_s16(partial6, tmp);

    let const0 = vreinterpretq_u32_u64(vcombine_u64(
        vcreate_u64((420u64) << 32 | 840),
        vcreate_u64((210u64) << 32 | 280),
    ));
    let const1 = vreinterpretq_u32_u64(vcombine_u64(
        vcreate_u64((140u64) << 32 | 168),
        vcreate_u64((105u64) << 32 | 120),
    ));
    let const2 = vreinterpretq_u32_u64(vcombine_u64(
        vcreate_u64(0),
        vcreate_u64((210u64) << 32 | 420),
    ));
    let const3 = vreinterpretq_u32_u64(vcombine_u64(
        vcreate_u64((105u64) << 32 | 140),
        vcreate_u64((105u64) << 32 | 105),
    ));

    // Compute costs in terms of partial sums.
    let mut partial6_s32 = vmull_s16(vget_low_s16(partial6), vget_low_s16(partial6));
    partial6_s32 = vmlal_s16(partial6_s32, vget_high_s16(partial6), vget_high_s16(partial6));

    let mut costs: [uint32x4_t; 4] = [
        fold_mul_and_sum_neon(partial4a, partial4b, const0, const1),
        fold_mul_and_sum_neon(partial5a, partial5b, const2, const3),
        vmulq_n_u32(vreinterpretq_u32_s32(partial6_s32), 105),
        fold_mul_and_sum_neon(partial7a, partial7b, const2, const3),
    ];

    costs[0] = horizontal_add_4d_u32x4(costs.as_ptr());
    vst1q_u32(cost, costs[0]);
    costs[0]
}

#[inline(always)]
unsafe fn fold_mul_and_sum_pairwise_neon(
    partiala: int16x8_t,
    partialb: int16x8_t,
    mut partialc: int16x8_t,
    const0: uint32x4_t,
) -> uint32x4_t {
    // Reverse partial c.
    // pattern = { 10 11 8 9 6 7 4 5 2 3 0 1 12 13 14 15 }.
    let pattern = vreinterpretq_u8_u64(vcombine_u64(
        vcreate_u64((0x05040706u64) << 32 | 0x09080b0a),
        vcreate_u64((0x0f0e0d0cu64) << 32 | 0x01000302),
    ));

    partialc = vreinterpretq_s16_s8(vqtbl1q_s8(vreinterpretq_s8_s16(partialc), pattern));

    let mut partiala_s32 = vpaddlq_s16(partiala);
    let mut partialb_s32 = vpaddlq_s16(partialb);
    let mut partialc_s32 = vpaddlq_s16(partialc);

    partiala_s32 = vmulq_s32(partiala_s32, partiala_s32);
    partialb_s32 = vmulq_s32(partialb_s32, partialb_s32);
    partialc_s32 = vmulq_s32(partialc_s32, partialc_s32);

    partiala_s32 = vaddq_s32(partiala_s32, partialc_s32);

    let mut cost = vmulq_n_u32(vreinterpretq_u32_s32(partialb_s32), 105);
    cost = vmlaq_u32(cost, vreinterpretq_u32_s32(partiala_s32), const0);
    cost
}

// This function computes the cost along directions 0, 1, 2, 3. (0 means
// 45-degree up-right, 2 is horizontal).
//
// For direction 1 and 3 ("east northeast" and "east southeast") the shifted
// lines need three vectors instead of two. For direction 1 for example, we need
// to compute the sums along the line i below:
// 0 0 1 1 2 2 3  3
// 1 1 2 2 3 3 4  4
// 2 2 3 3 4 4 5  5
// 3 3 4 4 5 5 6  6
// 4 4 5 5 6 6 7  7
// 5 5 6 6 7 7 8  8
// 6 6 7 7 8 8 9  9
// 7 7 8 8 9 9 10 10
//
// Which means we need the following configuration:
// 0 0 1 1 2 2 3 3
//     1 1 2 2 3 3 4 4
//         2 2 3 3 4 4 5 5
//             3 3 4 4 5 5 6 6
//                 4 4 5 5 6 6 7 7
//                     5 5 6 6 7 7 8 8
//                         6 6 7 7 8 8 9 9
//                             7 7 8 8 9 9 10 10
//
// Three vectors are needed to compute this, as well as some extra pairwise
// additions.
unsafe fn compute_horiz_directions_neon(lines: *const int16x8_t, cost: *mut u32) -> uint32x4_t {
    let lines = core::slice::from_raw_parts(lines, 8);
    let zero = vdupq_n_s16(0);

    // Compute diagonal directions (1, 2, 3).
    // Partial sums for lines 0 and 1.
    let mut partial0a = lines[0];
    partial0a = vaddq_s16(partial0a, vextq_s16::<7>(zero, lines[1]));
    let mut partial0b = vextq_s16::<7>(lines[1], zero);
    let mut partial1a = vaddq_s16(lines[0], vextq_s16::<6>(zero, lines[1]));
    let mut partial1b = vextq_s16::<6>(lines[1], zero);
    let mut partial3a = vextq_s16::<2>(lines[0], zero);
    partial3a = vaddq_s16(partial3a, vextq_s16::<4>(lines[1], zero));
    let mut partial3b = vextq_s16::<2>(zero, lines[0]);
    partial3b = vaddq_s16(partial3b, vextq_s16::<4>(zero, lines[1]));

    // Partial sums for lines 2 and 3.
    partial0a = vaddq_s16(partial0a, vextq_s16::<6>(zero, lines[2]));
    partial0a = vaddq_s16(partial0a, vextq_s16::<5>(zero, lines[3]));
    partial0b = vaddq_s16(partial0b, vextq_s16::<6>(lines[2], zero));
    partial0b = vaddq_s16(partial0b, vextq_s16::<5>(lines[3], zero));
    partial1a = vaddq_s16(partial1a, vextq_s16::<4>(zero, lines[2]));
    partial1a = vaddq_s16(partial1a, vextq_s16::<2>(zero, lines[3]));
    partial1b = vaddq_s16(partial1b, vextq_s16::<4>(lines[2], zero));
    partial1b = vaddq_s16(partial1b, vextq_s16::<2>(lines[3], zero));
    partial3a = vaddq_s16(partial3a, vextq_s16::<6>(lines[2], zero));
    partial3b = vaddq_s16(partial3b, vextq_s16::<6>(zero, lines[2]));
    partial3b = vaddq_s16(partial3b, lines[3]);

    // Partial sums for lines 4 and 5.
    partial0a = vaddq_s16(partial0a, vextq_s16::<4>(zero, lines[4]));
    partial0a = vaddq_s16(partial0a, vextq_s16::<3>(zero, lines[5]));
    partial0b = vaddq_s16(partial0b, vextq_s16::<4>(lines[4], zero));
    partial0b = vaddq_s16(partial0b, vextq_s16::<3>(lines[5], zero));
    partial1b = vaddq_s16(partial1b, lines[4]);
    partial1b = vaddq_s16(partial1b, vextq_s16::<6>(zero, lines[5]));
    let mut partial1c = vextq_s16::<6>(lines[5], zero);
    partial3b = vaddq_s16(partial3b, vextq_s16::<2>(lines[4], zero));
    partial3b = vaddq_s16(partial3b, vextq_s16::<4>(lines[5], zero));
    let mut partial3c = vextq_s16::<2>(zero, lines[4]);
    partial3c = vaddq_s16(partial3c, vextq_s16::<4>(zero, lines[5]));

    // Partial sums for lines 6 and 7.
    partial0a = vaddq_s16(partial0a, vextq_s16::<2>(zero, lines[6]));
    partial0a = vaddq_s16(partial0a, vextq_s16::<1>(zero, lines[7]));
    partial0b = vaddq_s16(partial0b, vextq_s16::<2>(lines[6], zero));
    partial0b = vaddq_s16(partial0b, vextq_s16::<1>(lines[7], zero));
    partial1b = vaddq_s16(partial1b, vextq_s16::<4>(zero, lines[6]));
    partial1b = vaddq_s16(partial1b, vextq_s16::<2>(zero, lines[7]));
    partial1c = vaddq_s16(partial1c, vextq_s16::<4>(lines[6], zero));
    partial1c = vaddq_s16(partial1c, vextq_s16::<2>(lines[7], zero));
    partial3b = vaddq_s16(partial3b, vextq_s16::<6>(lines[6], zero));
    partial3c = vaddq_s16(partial3c, vextq_s16::<6>(zero, lines[6]));
    partial3c = vaddq_s16(partial3c, lines[7]);

    // Special case for direction 2 as it's just a sum along each line.
    let lines03: [int16x8_t; 4] = [lines[0], lines[1], lines[2], lines[3]];
    let lines47: [int16x8_t; 4] = [lines[4], lines[5], lines[6], lines[7]];
    let partial2a = horizontal_add_4d_s16x8(lines03.as_ptr());
    let partial2b = horizontal_add_4d_s16x8(lines47.as_ptr());

    let partial2a_u32 = vreinterpretq_u32_s32(vmulq_s32(partial2a, partial2a));
    let partial2b_u32 = vreinterpretq_u32_s32(vmulq_s32(partial2b, partial2b));

    let const0 = vreinterpretq_u32_u64(vcombine_u64(
        vcreate_u64((420u64) << 32 | 840),
        vcreate_u64((210u64) << 32 | 280),
    ));
    let const1 = vreinterpretq_u32_u64(vcombine_u64(
        vcreate_u64((140u64) << 32 | 168),
        vcreate_u64((105u64) << 32 | 120),
    ));
    let const2 = vreinterpretq_u32_u64(vcombine_u64(
        vcreate_u64((210u64) << 32 | 420),
        vcreate_u64((105u64) << 32 | 140),
    ));

    let mut costs: [uint32x4_t; 4] = [
        fold_mul_and_sum_neon(partial0a, partial0b, const0, const1),
        fold_mul_and_sum_pairwise_neon(partial1a, partial1b, partial1c, const2),
        vaddq_u32(partial2a_u32, partial2b_u32),
        fold_mul_and_sum_pairwise_neon(partial3c, partial3b, partial3a, const2),
    ];
    costs[2] = vmulq_n_u32(costs[2], 105);

    costs[0] = horizontal_add_4d_u32x4(costs.as_ptr());
    vst1q_u32(cost, costs[0]);
    costs[0]
}

#[no_mangle]
pub unsafe extern "C-unwind" fn cdef_find_dir_neon(
    img: *const u16,
    stride: i32,
    var: *mut i32,
    coeff_shift: i32,
) -> i32 {
    let mut cost: [u32; 8] = [0; 8];
    let best_cost: u32;
    let best_dir: i32;
    let mut lines: [int16x8_t; 8] = [vdupq_n_s16(0); 8];
    for i in 0..8 {
        let s = vld1q_u16(img.offset((i * stride) as isize));
        lines[i as usize] = vreinterpretq_s16_u16(vsubq_u16(
            vshlq_u16(s, vdupq_n_s16((-coeff_shift) as i16)),
            vdupq_n_u16(128),
        ));
    }

    // Compute "mostly vertical" directions.
    let cost47 = compute_vert_directions_neon(lines.as_ptr(), cost.as_mut_ptr().add(4));

    // Compute "mostly horizontal" directions.
    let cost03 = compute_horiz_directions_neon(lines.as_ptr(), cost.as_mut_ptr());

    // Find max cost as well as its index to get best_dir.
    // The max cost needs to be propagated in the whole vector to find its
    // position in the original cost vectors cost03 and cost47.
    let cost07 = vmaxq_u32(cost03, cost47);
    best_cost = vmaxvq_u32(cost07);
    let max_cost = vdupq_n_u32(best_cost);
    let costs = uint8x16x2_t(
        vreinterpretq_u8_u32(vceqq_u32(max_cost, cost03)),
        vreinterpretq_u8_u32(vceqq_u32(max_cost, cost47)),
    );
    // idx = { 28, 24, 20, 16, 12, 8, 4, 0 };
    let idx = vreinterpret_u8_u64(vcreate_u64(0x0004080c1014181c));
    // Get the lowest 8 bit of each 32-bit elements and reverse them.
    let tbl = vqtbl2_u8(costs, idx);
    let a: u64 = vget_lane_u64::<0>(vreinterpret_u64_u8(tbl));
    best_dir = aom_clzll(a) >> 3;

    // Difference between the optimal variance and the variance along the
    // orthogonal direction. Again, the sum(x^2) terms cancel out.
    *var = best_cost.wrapping_sub(cost[((best_dir + 4) & 7) as usize]) as i32;
    // We'd normally divide by 840, but dividing by 1024 is close enough
    // for what we're going to do with this.
    *var >>= 10;
    best_dir
}

#[no_mangle]
pub unsafe extern "C-unwind" fn cdef_find_dir_dual_neon(
    img1: *const u16,
    img2: *const u16,
    stride: i32,
    var_out_1st: *mut i32,
    var_out_2nd: *mut i32,
    coeff_shift: i32,
    out_dir_1st_8x8: *mut i32,
    out_dir_2nd_8x8: *mut i32,
) {
    // The original calls the RTCD symbol cdef_find_dir, which the AArch64
    // dispatch resolves to cdef_find_dir_neon whenever this function runs.
    // Process first 8x8.
    *out_dir_1st_8x8 = cdef_find_dir_neon(img1, stride, var_out_1st, coeff_shift);

    // Process second 8x8.
    *out_dir_2nd_8x8 = cdef_find_dir_neon(img2, stride, var_out_2nd, coeff_shift);
}

// sign(a-b) * min(abs(a-b), max(0, threshold - (abs(a-b) >> adjdamp)))
#[inline(always)]
unsafe fn constrain16(a: uint16x8_t, b: uint16x8_t, threshold: u32, adjdamp: i32) -> int16x8_t {
    let diff = vabdq_u16(a, b);
    let a_gt_b = vcgtq_u16(a, b);
    let s = vqsubq_u16(vdupq_n_u16(threshold as u16), vshlq_u16(diff, vdupq_n_s16((-adjdamp) as i16)));
    let clip = vreinterpretq_s16_u16(vminq_u16(diff, s));
    vbslq_s16(a_gt_b, clip, vnegq_s16(clip))
}

#[inline(always)]
unsafe fn primary_filter(
    s: uint16x8_t,
    tap: *const uint16x8_t,
    pri_taps: *const i32,
    pri_strength: i32,
    pri_damping: i32,
    sum: *mut int16x8_t,
) {
    // Near taps
    let mut n0 = constrain16(*tap.add(0), s, pri_strength as u32, pri_damping);
    let n1 = constrain16(*tap.add(1), s, pri_strength as u32, pri_damping);
    // sum += pri_taps[0] * (n0 + n1)
    n0 = vaddq_s16(n0, n1);
    *sum = vmlaq_n_s16(*sum, n0, *pri_taps.add(0) as i16);

    // Far taps
    let mut f0 = constrain16(*tap.add(2), s, pri_strength as u32, pri_damping);
    let f1 = constrain16(*tap.add(3), s, pri_strength as u32, pri_damping);
    // sum += pri_taps[1] * (f0 + f1)
    f0 = vaddq_s16(f0, f1);
    *sum = vmlaq_n_s16(*sum, f0, *pri_taps.add(1) as i16);
}

#[inline(always)]
unsafe fn secondary_filter(
    s: uint16x8_t,
    tap: *const uint16x8_t,
    sec_taps: *const i32,
    sec_strength: i32,
    sec_damping: i32,
    sum: *mut int16x8_t,
) {
    // Near taps
    let mut s0 = constrain16(*tap.add(0), s, sec_strength as u32, sec_damping);
    let mut s1 = constrain16(*tap.add(1), s, sec_strength as u32, sec_damping);
    let mut s2 = constrain16(*tap.add(2), s, sec_strength as u32, sec_damping);
    let mut s3 = constrain16(*tap.add(3), s, sec_strength as u32, sec_damping);

    // sum += sec_taps[0] * (p0 + p1 + p2 + p3)
    s0 = vaddq_s16(s0, s1);
    s2 = vaddq_s16(s2, s3);
    s0 = vaddq_s16(s0, s2);
    *sum = vmlaq_n_s16(*sum, s0, *sec_taps.add(0) as i16);

    // Far taps
    s0 = constrain16(*tap.add(4), s, sec_strength as u32, sec_damping);
    s1 = constrain16(*tap.add(5), s, sec_strength as u32, sec_damping);
    s2 = constrain16(*tap.add(6), s, sec_strength as u32, sec_damping);
    s3 = constrain16(*tap.add(7), s, sec_strength as u32, sec_damping);

    // sum += sec_taps[1] * (p0 + p1 + p2 + p3)
    s0 = vaddq_s16(s0, s1);
    s2 = vaddq_s16(s2, s3);
    s0 = vaddq_s16(s0, s2);
    *sum = vmlaq_n_s16(*sum, s0, *sec_taps.add(1) as i16);
}

#[no_mangle]
pub unsafe extern "C-unwind" fn cdef_filter_8_0_neon(
    dest: *mut c_void,
    dstride: i32,
    mut in_: *const u16,
    pri_strength: i32,
    sec_strength: i32,
    dir: i32,
    mut pri_damping: i32,
    mut sec_damping: i32,
    coeff_shift: i32,
    block_width: i32,
    block_height: i32,
) {
    let mut max: uint16x8_t;
    let mut min: uint16x8_t;
    let cdef_large_value_mask = vdupq_n_u16(!CDEF_VERY_LARGE);
    let po1 = cdef_dir(dir, 0);
    let po2 = cdef_dir(dir, 1);
    let s1o1 = cdef_dir(dir + 2, 0);
    let s1o2 = cdef_dir(dir + 2, 1);
    let s2o1 = cdef_dir(dir - 2, 0);
    let s2o2 = cdef_dir(dir - 2, 1);
    let pri_taps = cdef_pri_taps_row((pri_strength >> coeff_shift) & 1);
    let sec_taps = cdef_sec_taps_ptr();

    if pri_strength != 0 {
        pri_damping = aommax(0, pri_damping - get_msb(pri_strength as u32));
    }
    if sec_strength != 0 {
        sec_damping = aommax(0, sec_damping - get_msb(sec_strength as u32));
    }

    if block_width == 8 {
        let mut dst8 = dest as *mut u8;

        let mut h = block_height;
        loop {
            let mut sum = vdupq_n_s16(0);
            let s = vld1q_u16(in_);
            min = s;
            max = s;

            let mut pri_src: [uint16x8_t; 4] = [vdupq_n_u16(0); 4];

            // Primary near taps
            pri_src[0] = vld1q_u16(in_.offset(po1));
            pri_src[1] = vld1q_u16(in_.offset(-po1));

            // Primary far taps
            pri_src[2] = vld1q_u16(in_.offset(po2));
            pri_src[3] = vld1q_u16(in_.offset(-po2));

            primary_filter(s, pri_src.as_ptr(), pri_taps, pri_strength, pri_damping, &mut sum);

            // The source is 16 bits, however, we only really care about the lower
            // 8 bits.  The upper 8 bits contain the "large" flag.  After the final
            // primary max has been calculated, zero out the upper 8 bits.  Use this
            // to find the "16 bit" max.
            let mut pri_max0 = vmaxq_u8(vreinterpretq_u8_u16(pri_src[0]), vreinterpretq_u8_u16(pri_src[1]));
            let pri_max1 = vmaxq_u8(vreinterpretq_u8_u16(pri_src[2]), vreinterpretq_u8_u16(pri_src[3]));
            pri_max0 = vmaxq_u8(pri_max0, pri_max1);
            max = vmaxq_u16(max, vandq_u16(vreinterpretq_u16_u8(pri_max0), cdef_large_value_mask));

            let mut pri_min0 = vminq_u16(pri_src[0], pri_src[1]);
            let pri_min1 = vminq_u16(pri_src[2], pri_src[3]);
            pri_min0 = vminq_u16(pri_min0, pri_min1);
            min = vminq_u16(min, pri_min0);

            let mut sec_src: [uint16x8_t; 8] = [vdupq_n_u16(0); 8];

            // Secondary near taps
            sec_src[0] = vld1q_u16(in_.offset(s1o1));
            sec_src[1] = vld1q_u16(in_.offset(-s1o1));
            sec_src[2] = vld1q_u16(in_.offset(s2o1));
            sec_src[3] = vld1q_u16(in_.offset(-s2o1));

            // Secondary far taps
            sec_src[4] = vld1q_u16(in_.offset(s1o2));
            sec_src[5] = vld1q_u16(in_.offset(-s1o2));
            sec_src[6] = vld1q_u16(in_.offset(s2o2));
            sec_src[7] = vld1q_u16(in_.offset(-s2o2));

            secondary_filter(s, sec_src.as_ptr(), sec_taps, sec_strength, sec_damping, &mut sum);

            // The source is 16 bits, however, we only really care about the lower
            // 8 bits.  The upper 8 bits contain the "large" flag.  After the final
            // primary max has been calculated, zero out the upper 8 bits.  Use this
            // to find the "16 bit" max.
            let mut sec_max0 = vmaxq_u8(vreinterpretq_u8_u16(sec_src[0]), vreinterpretq_u8_u16(sec_src[1]));
            let sec_max1 = vmaxq_u8(vreinterpretq_u8_u16(sec_src[2]), vreinterpretq_u8_u16(sec_src[3]));
            let mut sec_max2 = vmaxq_u8(vreinterpretq_u8_u16(sec_src[4]), vreinterpretq_u8_u16(sec_src[5]));
            let sec_max3 = vmaxq_u8(vreinterpretq_u8_u16(sec_src[6]), vreinterpretq_u8_u16(sec_src[7]));
            sec_max0 = vmaxq_u8(sec_max0, sec_max1);
            sec_max2 = vmaxq_u8(sec_max2, sec_max3);
            sec_max0 = vmaxq_u8(sec_max0, sec_max2);
            max = vmaxq_u16(max, vandq_u16(vreinterpretq_u16_u8(sec_max0), cdef_large_value_mask));

            let mut sec_min0 = vminq_u16(sec_src[0], sec_src[1]);
            let sec_min1 = vminq_u16(sec_src[2], sec_src[3]);
            let mut sec_min2 = vminq_u16(sec_src[4], sec_src[5]);
            let sec_min3 = vminq_u16(sec_src[6], sec_src[7]);
            sec_min0 = vminq_u16(sec_min0, sec_min1);
            sec_min2 = vminq_u16(sec_min2, sec_min3);
            sec_min0 = vminq_u16(sec_min0, sec_min2);
            min = vminq_u16(min, sec_min0);

            // res = s + ((sum - (sum < 0) + 8) >> 4)
            sum = vaddq_s16(sum, vreinterpretq_s16_u16(vcltq_s16(sum, vdupq_n_s16(0))));
            let mut res_s16 = vrsraq_n_s16::<4>(vreinterpretq_s16_u16(s), sum);

            res_s16 = vminq_s16(vmaxq_s16(res_s16, vreinterpretq_s16_u16(min)), vreinterpretq_s16_u16(max));

            let res_u8 = vqmovun_s16(res_s16);
            vst1_u8(dst8, res_u8);

            in_ = in_.wrapping_offset(CDEF_BSTRIDE);
            dst8 = dst8.wrapping_offset(dstride as isize);
            h -= 1;
            if h == 0 {
                break;
            }
        }
    } else {
        let mut dst8 = dest as *mut u8;

        let mut h = block_height;
        loop {
            let mut sum = vdupq_n_s16(0);
            let s = load_unaligned_u16_4x2(in_, CDEF_BSTRIDE);
            min = s;
            max = s;

            let mut pri_src: [uint16x8_t; 4] = [vdupq_n_u16(0); 4];

            // Primary near taps
            pri_src[0] = load_unaligned_u16_4x2(in_.offset(po1), CDEF_BSTRIDE);
            pri_src[1] = load_unaligned_u16_4x2(in_.offset(-po1), CDEF_BSTRIDE);

            // Primary far taps
            pri_src[2] = load_unaligned_u16_4x2(in_.offset(po2), CDEF_BSTRIDE);
            pri_src[3] = load_unaligned_u16_4x2(in_.offset(-po2), CDEF_BSTRIDE);

            primary_filter(s, pri_src.as_ptr(), pri_taps, pri_strength, pri_damping, &mut sum);

            // The source is 16 bits, however, we only really care about the lower
            // 8 bits.  The upper 8 bits contain the "large" flag.  After the final
            // primary max has been calculated, zero out the upper 8 bits.  Use this
            // to find the "16 bit" max.
            let mut pri_max0 = vmaxq_u8(vreinterpretq_u8_u16(pri_src[0]), vreinterpretq_u8_u16(pri_src[1]));
            let pri_max1 = vmaxq_u8(vreinterpretq_u8_u16(pri_src[2]), vreinterpretq_u8_u16(pri_src[3]));
            pri_max0 = vmaxq_u8(pri_max0, pri_max1);
            max = vmaxq_u16(max, vandq_u16(vreinterpretq_u16_u8(pri_max0), cdef_large_value_mask));

            let mut pri_min1 = vminq_u16(pri_src[0], pri_src[1]);
            let pri_min2 = vminq_u16(pri_src[2], pri_src[3]);
            pri_min1 = vminq_u16(pri_min1, pri_min2);
            min = vminq_u16(min, pri_min1);

            let mut sec_src: [uint16x8_t; 8] = [vdupq_n_u16(0); 8];

            // Secondary near taps
            sec_src[0] = load_unaligned_u16_4x2(in_.offset(s1o1), CDEF_BSTRIDE);
            sec_src[1] = load_unaligned_u16_4x2(in_.offset(-s1o1), CDEF_BSTRIDE);
            sec_src[2] = load_unaligned_u16_4x2(in_.offset(s2o1), CDEF_BSTRIDE);
            sec_src[3] = load_unaligned_u16_4x2(in_.offset(-s2o1), CDEF_BSTRIDE);

            // Secondary far taps
            sec_src[4] = load_unaligned_u16_4x2(in_.offset(s1o2), CDEF_BSTRIDE);
            sec_src[5] = load_unaligned_u16_4x2(in_.offset(-s1o2), CDEF_BSTRIDE);
            sec_src[6] = load_unaligned_u16_4x2(in_.offset(s2o2), CDEF_BSTRIDE);
            sec_src[7] = load_unaligned_u16_4x2(in_.offset(-s2o2), CDEF_BSTRIDE);

            secondary_filter(s, sec_src.as_ptr(), sec_taps, sec_strength, sec_damping, &mut sum);

            // The source is 16 bits, however, we only really care about the lower
            // 8 bits.  The upper 8 bits contain the "large" flag.  After the final
            // primary max has been calculated, zero out the upper 8 bits.  Use this
            // to find the "16 bit" max.
            let mut sec_max0 = vmaxq_u8(vreinterpretq_u8_u16(sec_src[0]), vreinterpretq_u8_u16(sec_src[1]));
            let sec_max1 = vmaxq_u8(vreinterpretq_u8_u16(sec_src[2]), vreinterpretq_u8_u16(sec_src[3]));
            let mut sec_max2 = vmaxq_u8(vreinterpretq_u8_u16(sec_src[4]), vreinterpretq_u8_u16(sec_src[5]));
            let sec_max3 = vmaxq_u8(vreinterpretq_u8_u16(sec_src[6]), vreinterpretq_u8_u16(sec_src[7]));
            sec_max0 = vmaxq_u8(sec_max0, sec_max1);
            sec_max2 = vmaxq_u8(sec_max2, sec_max3);
            sec_max0 = vmaxq_u8(sec_max0, sec_max2);
            max = vmaxq_u16(max, vandq_u16(vreinterpretq_u16_u8(sec_max0), cdef_large_value_mask));

            let mut sec_min0 = vminq_u16(sec_src[0], sec_src[1]);
            let sec_min1 = vminq_u16(sec_src[2], sec_src[3]);
            let mut sec_min2 = vminq_u16(sec_src[4], sec_src[5]);
            let sec_min3 = vminq_u16(sec_src[6], sec_src[7]);
            sec_min0 = vminq_u16(sec_min0, sec_min1);
            sec_min2 = vminq_u16(sec_min2, sec_min3);
            sec_min0 = vminq_u16(sec_min0, sec_min2);
            min = vminq_u16(min, sec_min0);

            // res = s + ((sum - (sum < 0) + 8) >> 4)
            sum = vaddq_s16(sum, vreinterpretq_s16_u16(vcltq_s16(sum, vdupq_n_s16(0))));
            let mut res_s16 = vrsraq_n_s16::<4>(vreinterpretq_s16_u16(s), sum);

            res_s16 = vminq_s16(vmaxq_s16(res_s16, vreinterpretq_s16_u16(min)), vreinterpretq_s16_u16(max));

            let res_u8 = vqmovun_s16(res_s16);
            store_u8x4_strided_x2(dst8, dstride as isize, res_u8);

            in_ = in_.wrapping_offset(2 * CDEF_BSTRIDE);
            dst8 = dst8.wrapping_offset(2 * dstride as isize);
            h -= 2;
            if h == 0 {
                break;
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C-unwind" fn cdef_filter_8_1_neon(
    dest: *mut c_void,
    dstride: i32,
    mut in_: *const u16,
    pri_strength: i32,
    _sec_strength: i32,
    dir: i32,
    mut pri_damping: i32,
    _sec_damping: i32,
    coeff_shift: i32,
    block_width: i32,
    block_height: i32,
) {
    let po1 = cdef_dir(dir, 0);
    let po2 = cdef_dir(dir, 1);
    let pri_taps = cdef_pri_taps_row((pri_strength >> coeff_shift) & 1);

    if pri_strength != 0 {
        pri_damping = aommax(0, pri_damping - get_msb(pri_strength as u32));
    }

    if block_width == 8 {
        let mut dst8 = dest as *mut u8;

        let mut h = block_height;
        loop {
            let mut sum = vdupq_n_s16(0);
            let s = vld1q_u16(in_);

            let mut tap: [uint16x8_t; 4] = [vdupq_n_u16(0); 4];

            // Primary near taps
            tap[0] = vld1q_u16(in_.offset(po1));
            tap[1] = vld1q_u16(in_.offset(-po1));

            // Primary far taps
            tap[2] = vld1q_u16(in_.offset(po2));
            tap[3] = vld1q_u16(in_.offset(-po2));

            primary_filter(s, tap.as_ptr(), pri_taps, pri_strength, pri_damping, &mut sum);

            // res = s + ((sum - (sum < 0) + 8) >> 4)
            sum = vaddq_s16(sum, vreinterpretq_s16_u16(vcltq_s16(sum, vdupq_n_s16(0))));
            let res_s16 = vrsraq_n_s16::<4>(vreinterpretq_s16_u16(s), sum);

            let res_u8 = vqmovun_s16(res_s16);
            vst1_u8(dst8, res_u8);

            in_ = in_.wrapping_offset(CDEF_BSTRIDE);
            dst8 = dst8.wrapping_offset(dstride as isize);
            h -= 1;
            if h == 0 {
                break;
            }
        }
    } else {
        let mut dst8 = dest as *mut u8;

        let mut h = block_height;
        loop {
            let mut sum = vdupq_n_s16(0);
            let s = load_unaligned_u16_4x2(in_, CDEF_BSTRIDE);

            let mut pri_src: [uint16x8_t; 4] = [vdupq_n_u16(0); 4];

            // Primary near taps
            pri_src[0] = load_unaligned_u16_4x2(in_.offset(po1), CDEF_BSTRIDE);
            pri_src[1] = load_unaligned_u16_4x2(in_.offset(-po1), CDEF_BSTRIDE);

            // Primary far taps
            pri_src[2] = load_unaligned_u16_4x2(in_.offset(po2), CDEF_BSTRIDE);
            pri_src[3] = load_unaligned_u16_4x2(in_.offset(-po2), CDEF_BSTRIDE);

            primary_filter(s, pri_src.as_ptr(), pri_taps, pri_strength, pri_damping, &mut sum);

            // res = s + ((sum - (sum < 0) + 8) >> 4)
            sum = vaddq_s16(sum, vreinterpretq_s16_u16(vcltq_s16(sum, vdupq_n_s16(0))));
            let res_s16 = vrsraq_n_s16::<4>(vreinterpretq_s16_u16(s), sum);

            let res_u8 = vqmovun_s16(res_s16);
            store_u8x4_strided_x2(dst8, dstride as isize, res_u8);

            in_ = in_.wrapping_offset(2 * CDEF_BSTRIDE);
            dst8 = dst8.wrapping_offset(2 * dstride as isize);
            h -= 2;
            if h == 0 {
                break;
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C-unwind" fn cdef_filter_8_2_neon(
    dest: *mut c_void,
    dstride: i32,
    mut in_: *const u16,
    _pri_strength: i32,
    sec_strength: i32,
    dir: i32,
    _pri_damping: i32,
    mut sec_damping: i32,
    _coeff_shift: i32,
    block_width: i32,
    block_height: i32,
) {
    let s1o1 = cdef_dir(dir + 2, 0);
    let s1o2 = cdef_dir(dir + 2, 1);
    let s2o1 = cdef_dir(dir - 2, 0);
    let s2o2 = cdef_dir(dir - 2, 1);
    let sec_taps = cdef_sec_taps_ptr();

    if sec_strength != 0 {
        sec_damping = aommax(0, sec_damping - get_msb(sec_strength as u32));
    }

    if block_width == 8 {
        let mut dst8 = dest as *mut u8;

        let mut h = block_height;
        loop {
            let mut sum = vdupq_n_s16(0);
            let s = vld1q_u16(in_);

            let mut sec_src: [uint16x8_t; 8] = [vdupq_n_u16(0); 8];

            // Secondary near taps
            sec_src[0] = vld1q_u16(in_.offset(s1o1));
            sec_src[1] = vld1q_u16(in_.offset(-s1o1));
            sec_src[2] = vld1q_u16(in_.offset(s2o1));
            sec_src[3] = vld1q_u16(in_.offset(-s2o1));

            // Secondary far taps
            sec_src[4] = vld1q_u16(in_.offset(s1o2));
            sec_src[5] = vld1q_u16(in_.offset(-s1o2));
            sec_src[6] = vld1q_u16(in_.offset(s2o2));
            sec_src[7] = vld1q_u16(in_.offset(-s2o2));

            secondary_filter(s, sec_src.as_ptr(), sec_taps, sec_strength, sec_damping, &mut sum);

            // res = s + ((sum - (sum < 0) + 8) >> 4)
            sum = vaddq_s16(sum, vreinterpretq_s16_u16(vcltq_s16(sum, vdupq_n_s16(0))));
            let res_s16 = vrsraq_n_s16::<4>(vreinterpretq_s16_u16(s), sum);

            let res_u8 = vqmovun_s16(res_s16);
            vst1_u8(dst8, res_u8);

            in_ = in_.wrapping_offset(CDEF_BSTRIDE);
            dst8 = dst8.wrapping_offset(dstride as isize);
            h -= 1;
            if h == 0 {
                break;
            }
        }
    } else {
        let mut dst8 = dest as *mut u8;

        let mut h = block_height;
        loop {
            let mut sum = vdupq_n_s16(0);
            let s = load_unaligned_u16_4x2(in_, CDEF_BSTRIDE);

            let mut sec_src: [uint16x8_t; 8] = [vdupq_n_u16(0); 8];

            // Secondary near taps
            sec_src[0] = load_unaligned_u16_4x2(in_.offset(s1o1), CDEF_BSTRIDE);
            sec_src[1] = load_unaligned_u16_4x2(in_.offset(-s1o1), CDEF_BSTRIDE);
            sec_src[2] = load_unaligned_u16_4x2(in_.offset(s2o1), CDEF_BSTRIDE);
            sec_src[3] = load_unaligned_u16_4x2(in_.offset(-s2o1), CDEF_BSTRIDE);

            // Secondary far taps
            sec_src[4] = load_unaligned_u16_4x2(in_.offset(s1o2), CDEF_BSTRIDE);
            sec_src[5] = load_unaligned_u16_4x2(in_.offset(-s1o2), CDEF_BSTRIDE);
            sec_src[6] = load_unaligned_u16_4x2(in_.offset(s2o2), CDEF_BSTRIDE);
            sec_src[7] = load_unaligned_u16_4x2(in_.offset(-s2o2), CDEF_BSTRIDE);

            secondary_filter(s, sec_src.as_ptr(), sec_taps, sec_strength, sec_damping, &mut sum);

            // res = s + ((sum - (sum < 0) + 8) >> 4)
            sum = vaddq_s16(sum, vreinterpretq_s16_u16(vcltq_s16(sum, vdupq_n_s16(0))));
            let res_s16 = vrsraq_n_s16::<4>(vreinterpretq_s16_u16(s), sum);

            let res_u8 = vqmovun_s16(res_s16);
            store_u8x4_strided_x2(dst8, dstride as isize, res_u8);

            in_ = in_.wrapping_offset(2 * CDEF_BSTRIDE);
            dst8 = dst8.wrapping_offset(2 * dstride as isize);
            h -= 2;
            if h == 0 {
                break;
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C-unwind" fn cdef_filter_8_3_neon(
    dest: *mut c_void,
    dstride: i32,
    mut in_: *const u16,
    _pri_strength: i32,
    _sec_strength: i32,
    _dir: i32,
    _pri_damping: i32,
    _sec_damping: i32,
    _coeff_shift: i32,
    block_width: i32,
    block_height: i32,
) {
    if block_width == 8 {
        let mut dst8 = dest as *mut u8;

        let mut h = block_height;
        loop {
            let s = vld1q_u16(in_);
            let res = vqmovn_u16(s);
            vst1_u8(dst8, res);

            in_ = in_.wrapping_offset(CDEF_BSTRIDE);
            dst8 = dst8.wrapping_offset(dstride as isize);
            h -= 1;
            if h == 0 {
                break;
            }
        }
    } else {
        let mut dst8 = dest as *mut u8;

        let mut h = block_height;
        loop {
            let s = load_unaligned_u16_4x2(in_, CDEF_BSTRIDE);
            let res = vqmovn_u16(s);
            store_u8x4_strided_x2(dst8, dstride as isize, res);

            in_ = in_.wrapping_offset(2 * CDEF_BSTRIDE);
            dst8 = dst8.wrapping_offset(2 * dstride as isize);
            h -= 2;
            if h == 0 {
                break;
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C-unwind" fn cdef_filter_16_0_neon(
    dest: *mut c_void,
    dstride: i32,
    mut in_: *const u16,
    pri_strength: i32,
    sec_strength: i32,
    dir: i32,
    mut pri_damping: i32,
    mut sec_damping: i32,
    coeff_shift: i32,
    block_width: i32,
    block_height: i32,
) {
    let mut max: uint16x8_t;
    let mut min: uint16x8_t;
    let cdef_large_value_mask = vdupq_n_u16(!CDEF_VERY_LARGE);
    let po1 = cdef_dir(dir, 0);
    let po2 = cdef_dir(dir, 1);
    let s1o1 = cdef_dir(dir + 2, 0);
    let s1o2 = cdef_dir(dir + 2, 1);
    let s2o1 = cdef_dir(dir - 2, 0);
    let s2o2 = cdef_dir(dir - 2, 1);
    let pri_taps = cdef_pri_taps_row((pri_strength >> coeff_shift) & 1);
    let sec_taps = cdef_sec_taps_ptr();

    if pri_strength != 0 {
        pri_damping = aommax(0, pri_damping - get_msb(pri_strength as u32));
    }
    if sec_strength != 0 {
        sec_damping = aommax(0, sec_damping - get_msb(sec_strength as u32));
    }

    if block_width == 8 {
        let mut dst16 = dest as *mut u16;

        let mut h = block_height;
        loop {
            let mut sum = vdupq_n_s16(0);
            let s = vld1q_u16(in_);
            min = s;
            max = s;

            let mut pri_src: [uint16x8_t; 4] = [vdupq_n_u16(0); 4];

            // Primary near taps
            pri_src[0] = vld1q_u16(in_.offset(po1));
            pri_src[1] = vld1q_u16(in_.offset(-po1));

            // Primary far taps
            pri_src[2] = vld1q_u16(in_.offset(po2));
            pri_src[3] = vld1q_u16(in_.offset(-po2));

            primary_filter(s, pri_src.as_ptr(), pri_taps, pri_strength, pri_damping, &mut sum);

            let mut pri_min0 = vminq_u16(pri_src[0], pri_src[1]);
            let pri_min1 = vminq_u16(pri_src[2], pri_src[3]);
            pri_min0 = vminq_u16(pri_min0, pri_min1);
            min = vminq_u16(min, pri_min0);

            /* Convert CDEF_VERY_LARGE to 0 before calculating max. */
            pri_src[0] = vandq_u16(pri_src[0], cdef_large_value_mask);
            pri_src[1] = vandq_u16(pri_src[1], cdef_large_value_mask);
            pri_src[2] = vandq_u16(pri_src[2], cdef_large_value_mask);
            pri_src[3] = vandq_u16(pri_src[3], cdef_large_value_mask);

            let mut pri_max0 = vmaxq_u16(pri_src[0], pri_src[1]);
            let pri_max1 = vmaxq_u16(pri_src[2], pri_src[3]);
            pri_max0 = vmaxq_u16(pri_max0, pri_max1);
            max = vmaxq_u16(max, pri_max0);

            let mut sec_src: [uint16x8_t; 8] = [vdupq_n_u16(0); 8];

            // Secondary near taps
            sec_src[0] = vld1q_u16(in_.offset(s1o1));
            sec_src[1] = vld1q_u16(in_.offset(-s1o1));
            sec_src[2] = vld1q_u16(in_.offset(s2o1));
            sec_src[3] = vld1q_u16(in_.offset(-s2o1));

            // Secondary far taps
            sec_src[4] = vld1q_u16(in_.offset(s1o2));
            sec_src[5] = vld1q_u16(in_.offset(-s1o2));
            sec_src[6] = vld1q_u16(in_.offset(s2o2));
            sec_src[7] = vld1q_u16(in_.offset(-s2o2));

            secondary_filter(s, sec_src.as_ptr(), sec_taps, sec_strength, sec_damping, &mut sum);

            let mut sec_min0 = vminq_u16(sec_src[0], sec_src[1]);
            let sec_min1 = vminq_u16(sec_src[2], sec_src[3]);
            let mut sec_min2 = vminq_u16(sec_src[4], sec_src[5]);
            let sec_min3 = vminq_u16(sec_src[6], sec_src[7]);
            sec_min0 = vminq_u16(sec_min0, sec_min1);
            sec_min2 = vminq_u16(sec_min2, sec_min3);
            sec_min0 = vminq_u16(sec_min0, sec_min2);
            min = vminq_u16(min, sec_min0);

            /* Convert CDEF_VERY_LARGE to 0 before calculating max. */
            sec_src[0] = vandq_u16(sec_src[0], cdef_large_value_mask);
            sec_src[1] = vandq_u16(sec_src[1], cdef_large_value_mask);
            sec_src[2] = vandq_u16(sec_src[2], cdef_large_value_mask);
            sec_src[3] = vandq_u16(sec_src[3], cdef_large_value_mask);
            sec_src[4] = vandq_u16(sec_src[4], cdef_large_value_mask);
            sec_src[5] = vandq_u16(sec_src[5], cdef_large_value_mask);
            sec_src[6] = vandq_u16(sec_src[6], cdef_large_value_mask);
            sec_src[7] = vandq_u16(sec_src[7], cdef_large_value_mask);

            let mut sec_max0 = vmaxq_u16(sec_src[0], sec_src[1]);
            let sec_max1 = vmaxq_u16(sec_src[2], sec_src[3]);
            let mut sec_max2 = vmaxq_u16(sec_src[4], sec_src[5]);
            let sec_max3 = vmaxq_u16(sec_src[6], sec_src[7]);
            sec_max0 = vmaxq_u16(sec_max0, sec_max1);
            sec_max2 = vmaxq_u16(sec_max2, sec_max3);
            sec_max0 = vmaxq_u16(sec_max0, sec_max2);
            max = vmaxq_u16(max, sec_max0);

            // res = s + ((sum - (sum < 0) + 8) >> 4)
            sum = vaddq_s16(sum, vreinterpretq_s16_u16(vcltq_s16(sum, vdupq_n_s16(0))));
            let mut res = vrsraq_n_s16::<4>(vreinterpretq_s16_u16(s), sum);

            res = vminq_s16(vmaxq_s16(res, vreinterpretq_s16_u16(min)), vreinterpretq_s16_u16(max));

            vst1q_u16(dst16, vreinterpretq_u16_s16(res));

            in_ = in_.wrapping_offset(CDEF_BSTRIDE);
            dst16 = dst16.wrapping_offset(dstride as isize);
            h -= 1;
            if h == 0 {
                break;
            }
        }
    } else {
        let mut dst16 = dest as *mut u16;

        let mut h = block_height;
        loop {
            let mut sum = vdupq_n_s16(0);
            let s = load_unaligned_u16_4x2(in_, CDEF_BSTRIDE);
            min = s;
            max = s;

            let mut pri_src: [uint16x8_t; 4] = [vdupq_n_u16(0); 4];

            // Primary near taps
            pri_src[0] = load_unaligned_u16_4x2(in_.offset(po1), CDEF_BSTRIDE);
            pri_src[1] = load_unaligned_u16_4x2(in_.offset(-po1), CDEF_BSTRIDE);

            // Primary far taps
            pri_src[2] = load_unaligned_u16_4x2(in_.offset(po2), CDEF_BSTRIDE);
            pri_src[3] = load_unaligned_u16_4x2(in_.offset(-po2), CDEF_BSTRIDE);

            primary_filter(s, pri_src.as_ptr(), pri_taps, pri_strength, pri_damping, &mut sum);

            let mut pri_min1 = vminq_u16(pri_src[0], pri_src[1]);
            let pri_min2 = vminq_u16(pri_src[2], pri_src[3]);
            pri_min1 = vminq_u16(pri_min1, pri_min2);
            min = vminq_u16(min, pri_min1);

            /* Convert CDEF_VERY_LARGE to 0 before calculating max. */
            pri_src[0] = vandq_u16(pri_src[0], cdef_large_value_mask);
            pri_src[1] = vandq_u16(pri_src[1], cdef_large_value_mask);
            pri_src[2] = vandq_u16(pri_src[2], cdef_large_value_mask);
            pri_src[3] = vandq_u16(pri_src[3], cdef_large_value_mask);
            let mut pri_max0 = vmaxq_u16(pri_src[0], pri_src[1]);
            let pri_max1 = vmaxq_u16(pri_src[2], pri_src[3]);
            pri_max0 = vmaxq_u16(pri_max0, pri_max1);
            max = vmaxq_u16(max, pri_max0);

            let mut sec_src: [uint16x8_t; 8] = [vdupq_n_u16(0); 8];

            // Secondary near taps
            sec_src[0] = load_unaligned_u16_4x2(in_.offset(s1o1), CDEF_BSTRIDE);
            sec_src[1] = load_unaligned_u16_4x2(in_.offset(-s1o1), CDEF_BSTRIDE);
            sec_src[2] = load_unaligned_u16_4x2(in_.offset(s2o1), CDEF_BSTRIDE);
            sec_src[3] = load_unaligned_u16_4x2(in_.offset(-s2o1), CDEF_BSTRIDE);

            // Secondary far taps
            sec_src[4] = load_unaligned_u16_4x2(in_.offset(s1o2), CDEF_BSTRIDE);
            sec_src[5] = load_unaligned_u16_4x2(in_.offset(-s1o2), CDEF_BSTRIDE);
            sec_src[6] = load_unaligned_u16_4x2(in_.offset(s2o2), CDEF_BSTRIDE);
            sec_src[7] = load_unaligned_u16_4x2(in_.offset(-s2o2), CDEF_BSTRIDE);

            secondary_filter(s, sec_src.as_ptr(), sec_taps, sec_strength, sec_damping, &mut sum);

            let mut sec_min0 = vminq_u16(sec_src[0], sec_src[1]);
            let sec_min1 = vminq_u16(sec_src[2], sec_src[3]);
            let mut sec_min2 = vminq_u16(sec_src[4], sec_src[5]);
            let sec_min3 = vminq_u16(sec_src[6], sec_src[7]);
            sec_min0 = vminq_u16(sec_min0, sec_min1);
            sec_min2 = vminq_u16(sec_min2, sec_min3);
            sec_min0 = vminq_u16(sec_min0, sec_min2);
            min = vminq_u16(min, sec_min0);

            /* Convert CDEF_VERY_LARGE to 0 before calculating max. */
            sec_src[0] = vandq_u16(sec_src[0], cdef_large_value_mask);
            sec_src[1] = vandq_u16(sec_src[1], cdef_large_value_mask);
            sec_src[2] = vandq_u16(sec_src[2], cdef_large_value_mask);
            sec_src[3] = vandq_u16(sec_src[3], cdef_large_value_mask);
            sec_src[4] = vandq_u16(sec_src[4], cdef_large_value_mask);
            sec_src[5] = vandq_u16(sec_src[5], cdef_large_value_mask);
            sec_src[6] = vandq_u16(sec_src[6], cdef_large_value_mask);
            sec_src[7] = vandq_u16(sec_src[7], cdef_large_value_mask);

            let mut sec_max0 = vmaxq_u16(sec_src[0], sec_src[1]);
            let sec_max1 = vmaxq_u16(sec_src[2], sec_src[3]);
            let mut sec_max2 = vmaxq_u16(sec_src[4], sec_src[5]);
            let sec_max3 = vmaxq_u16(sec_src[6], sec_src[7]);
            sec_max0 = vmaxq_u16(sec_max0, sec_max1);
            sec_max2 = vmaxq_u16(sec_max2, sec_max3);
            sec_max0 = vmaxq_u16(sec_max0, sec_max2);
            max = vmaxq_u16(max, sec_max0);

            // res = s + ((sum - (sum < 0) + 8) >> 4)
            sum = vaddq_s16(sum, vreinterpretq_s16_u16(vcltq_s16(sum, vdupq_n_s16(0))));
            let mut res = vrsraq_n_s16::<4>(vreinterpretq_s16_u16(s), sum);

            res = vminq_s16(vmaxq_s16(res, vreinterpretq_s16_u16(min)), vreinterpretq_s16_u16(max));

            store_u16x4_strided_x2(dst16, dstride as isize, vreinterpretq_u16_s16(res));

            in_ = in_.wrapping_offset(2 * CDEF_BSTRIDE);
            dst16 = dst16.wrapping_offset(2 * dstride as isize);
            h -= 2;
            if h == 0 {
                break;
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C-unwind" fn cdef_filter_16_1_neon(
    dest: *mut c_void,
    dstride: i32,
    mut in_: *const u16,
    pri_strength: i32,
    _sec_strength: i32,
    dir: i32,
    mut pri_damping: i32,
    _sec_damping: i32,
    coeff_shift: i32,
    block_width: i32,
    block_height: i32,
) {
    let po1 = cdef_dir(dir, 0);
    let po2 = cdef_dir(dir, 1);
    let pri_taps = cdef_pri_taps_row((pri_strength >> coeff_shift) & 1);

    if pri_strength != 0 {
        pri_damping = aommax(0, pri_damping - get_msb(pri_strength as u32));
    }

    if block_width == 8 {
        let mut dst16 = dest as *mut u16;

        let mut h = block_height;
        loop {
            let mut sum = vdupq_n_s16(0);
            let s = vld1q_u16(in_);

            let mut tap: [uint16x8_t; 4] = [vdupq_n_u16(0); 4];

            // Primary near taps
            tap[0] = vld1q_u16(in_.offset(po1));
            tap[1] = vld1q_u16(in_.offset(-po1));

            // Primary far taps
            tap[2] = vld1q_u16(in_.offset(po2));
            tap[3] = vld1q_u16(in_.offset(-po2));

            primary_filter(s, tap.as_ptr(), pri_taps, pri_strength, pri_damping, &mut sum);

            // res = s + ((sum - (sum < 0) + 8) >> 4)
            sum = vaddq_s16(sum, vreinterpretq_s16_u16(vcltq_s16(sum, vdupq_n_s16(0))));
            let res = vrsraq_n_s16::<4>(vreinterpretq_s16_u16(s), sum);

            vst1q_u16(dst16, vreinterpretq_u16_s16(res));

            in_ = in_.wrapping_offset(CDEF_BSTRIDE);
            dst16 = dst16.wrapping_offset(dstride as isize);
            h -= 1;
            if h == 0 {
                break;
            }
        }
    } else {
        let mut dst16 = dest as *mut u16;

        let mut h = block_height;
        loop {
            let mut sum = vdupq_n_s16(0);
            let s = load_unaligned_u16_4x2(in_, CDEF_BSTRIDE);

            let mut pri_src: [uint16x8_t; 4] = [vdupq_n_u16(0); 4];

            // Primary near taps
            pri_src[0] = load_unaligned_u16_4x2(in_.offset(po1), CDEF_BSTRIDE);
            pri_src[1] = load_unaligned_u16_4x2(in_.offset(-po1), CDEF_BSTRIDE);

            // Primary far taps
            pri_src[2] = load_unaligned_u16_4x2(in_.offset(po2), CDEF_BSTRIDE);
            pri_src[3] = load_unaligned_u16_4x2(in_.offset(-po2), CDEF_BSTRIDE);

            primary_filter(s, pri_src.as_ptr(), pri_taps, pri_strength, pri_damping, &mut sum);

            // res = s + ((sum - (sum < 0) + 8) >> 4)
            sum = vaddq_s16(sum, vreinterpretq_s16_u16(vcltq_s16(sum, vdupq_n_s16(0))));
            let res = vrsraq_n_s16::<4>(vreinterpretq_s16_u16(s), sum);

            store_u16x4_strided_x2(dst16, dstride as isize, vreinterpretq_u16_s16(res));

            in_ = in_.wrapping_offset(2 * CDEF_BSTRIDE);
            dst16 = dst16.wrapping_offset(2 * dstride as isize);
            h -= 2;
            if h == 0 {
                break;
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C-unwind" fn cdef_filter_16_2_neon(
    dest: *mut c_void,
    dstride: i32,
    mut in_: *const u16,
    _pri_strength: i32,
    sec_strength: i32,
    dir: i32,
    _pri_damping: i32,
    mut sec_damping: i32,
    _coeff_shift: i32,
    block_width: i32,
    block_height: i32,
) {
    let s1o1 = cdef_dir(dir + 2, 0);
    let s1o2 = cdef_dir(dir + 2, 1);
    let s2o1 = cdef_dir(dir - 2, 0);
    let s2o2 = cdef_dir(dir - 2, 1);
    let sec_taps = cdef_sec_taps_ptr();

    if sec_strength != 0 {
        sec_damping = aommax(0, sec_damping - get_msb(sec_strength as u32));
    }

    if block_width == 8 {
        let mut dst16 = dest as *mut u16;

        let mut h = block_height;
        loop {
            let mut sum = vdupq_n_s16(0);
            let s = vld1q_u16(in_);

            let mut sec_src: [uint16x8_t; 8] = [vdupq_n_u16(0); 8];

            // Secondary near taps
            sec_src[0] = vld1q_u16(in_.offset(s1o1));
            sec_src[1] = vld1q_u16(in_.offset(-s1o1));
            sec_src[2] = vld1q_u16(in_.offset(s2o1));
            sec_src[3] = vld1q_u16(in_.offset(-s2o1));

            // Secondary far taps
            sec_src[4] = vld1q_u16(in_.offset(s1o2));
            sec_src[5] = vld1q_u16(in_.offset(-s1o2));
            sec_src[6] = vld1q_u16(in_.offset(s2o2));
            sec_src[7] = vld1q_u16(in_.offset(-s2o2));

            secondary_filter(s, sec_src.as_ptr(), sec_taps, sec_strength, sec_damping, &mut sum);

            // res = s + ((sum - (sum < 0) + 8) >> 4)
            sum = vaddq_s16(sum, vreinterpretq_s16_u16(vcltq_s16(sum, vdupq_n_s16(0))));
            let res = vrsraq_n_s16::<4>(vreinterpretq_s16_u16(s), sum);

            vst1q_u16(dst16, vreinterpretq_u16_s16(res));

            in_ = in_.wrapping_offset(CDEF_BSTRIDE);
            dst16 = dst16.wrapping_offset(dstride as isize);
            h -= 1;
            if h == 0 {
                break;
            }
        }
    } else {
        let mut dst16 = dest as *mut u16;

        let mut h = block_height;
        loop {
            let mut sum = vdupq_n_s16(0);
            let s = load_unaligned_u16_4x2(in_, CDEF_BSTRIDE);

            let mut sec_src: [uint16x8_t; 8] = [vdupq_n_u16(0); 8];

            // Secondary near taps
            sec_src[0] = load_unaligned_u16_4x2(in_.offset(s1o1), CDEF_BSTRIDE);
            sec_src[1] = load_unaligned_u16_4x2(in_.offset(-s1o1), CDEF_BSTRIDE);
            sec_src[2] = load_unaligned_u16_4x2(in_.offset(s2o1), CDEF_BSTRIDE);
            sec_src[3] = load_unaligned_u16_4x2(in_.offset(-s2o1), CDEF_BSTRIDE);

            // Secondary far taps
            sec_src[4] = load_unaligned_u16_4x2(in_.offset(s1o2), CDEF_BSTRIDE);
            sec_src[5] = load_unaligned_u16_4x2(in_.offset(-s1o2), CDEF_BSTRIDE);
            sec_src[6] = load_unaligned_u16_4x2(in_.offset(s2o2), CDEF_BSTRIDE);
            sec_src[7] = load_unaligned_u16_4x2(in_.offset(-s2o2), CDEF_BSTRIDE);

            secondary_filter(s, sec_src.as_ptr(), sec_taps, sec_strength, sec_damping, &mut sum);

            // res = s + ((sum - (sum < 0) + 8) >> 4)
            sum = vaddq_s16(sum, vreinterpretq_s16_u16(vcltq_s16(sum, vdupq_n_s16(0))));
            let res = vrsraq_n_s16::<4>(vreinterpretq_s16_u16(s), sum);

            store_u16x4_strided_x2(dst16, dstride as isize, vreinterpretq_u16_s16(res));

            in_ = in_.wrapping_offset(2 * CDEF_BSTRIDE);
            dst16 = dst16.wrapping_offset(2 * dstride as isize);
            h -= 2;
            if h == 0 {
                break;
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C-unwind" fn cdef_filter_16_3_neon(
    dest: *mut c_void,
    dstride: i32,
    mut in_: *const u16,
    _pri_strength: i32,
    _sec_strength: i32,
    _dir: i32,
    _pri_damping: i32,
    _sec_damping: i32,
    _coeff_shift: i32,
    block_width: i32,
    block_height: i32,
) {
    if block_width == 8 {
        let mut dst16 = dest as *mut u16;

        let mut h = block_height;
        loop {
            let s = vld1q_u16(in_);
            vst1q_u16(dst16, s);

            in_ = in_.wrapping_offset(CDEF_BSTRIDE);
            dst16 = dst16.wrapping_offset(dstride as isize);
            h -= 1;
            if h == 0 {
                break;
            }
        }
    } else {
        let mut dst16 = dest as *mut u16;

        let mut h = block_height;
        loop {
            let s = load_unaligned_u16_4x2(in_, CDEF_BSTRIDE);
            store_u16x4_strided_x2(dst16, dstride as isize, s);

            in_ = in_.wrapping_offset(2 * CDEF_BSTRIDE);
            dst16 = dst16.wrapping_offset(2 * dstride as isize);
            h -= 2;
            if h == 0 {
                break;
            }
        }
    }
}
