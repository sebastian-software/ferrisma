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

// Translated by hand from upstream/aom/av1/common/arm/selfguided_neon.c
// (ADR 0056). Intrinsic sequences, helper structure, and names follow the
// original; the CONFIG_AV1_HIGHBITDEPTH branches are retained (the port's
// configuration enables high bit depth). C locals that the original leaves
// uninitialized start zeroed here, including the `dgd16_` stack buffers. The
// heap scratch buffers keep the original aom_memalign/aom_free calls. The
// output pointers derived from `dst8` are advanced with wrapping arithmetic:
// for the unused bit-depth branch they are CONVERT_TO_SHORTPTR/BYTEPTR values
// that the original computes but never dereferences.
#![allow(non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![allow(unused_variables, dead_code, clippy::all)]

use core::arch::aarch64::*;
use core::ffi::c_void;
use core::ptr;

use crate::aom_mem__aom_mem::{aom_free, aom_memalign};
use crate::av1__common__restoration::{
    av1_decode_xq, av1_one_by_x, av1_sgr_params, av1_x_by_xplus1, sgr_params_type,
};
use crate::simd::aom_dsp__arm__mem_neon::*;
use crate::simd::aom_dsp__arm__transpose_neon::*;

// av1/common/restoration.h
const RESTORATION_PROC_UNIT_SIZE: i32 = 64;
const RESTORATION_UNIT_OFFSET: i32 = 8;
const SGRPROJ_BORDER_VERT: i32 = 3;
const SGRPROJ_BORDER_HORZ: i32 = 3;
const RESTORATION_BORDER_VERT: i32 = SGRPROJ_BORDER_VERT;
const RESTORATION_BORDER_HORZ: i32 = SGRPROJ_BORDER_HORZ;
const RESTORATION_PADDING: i32 = 20;
const RESTORATION_PROC_UNIT_PELS: i32 = (RESTORATION_PROC_UNIT_SIZE
    + RESTORATION_BORDER_HORZ * 2
    + RESTORATION_PADDING)
    * (RESTORATION_PROC_UNIT_SIZE + RESTORATION_BORDER_VERT * 2 + RESTORATION_PADDING);
const RESTORATION_UNITSIZE_MAX: i32 = 256;
const RESTORATION_UNITPELS_HORZ_MAX: i32 =
    RESTORATION_UNITSIZE_MAX * 3 / 2 + 2 * RESTORATION_BORDER_HORZ + 16;
const RESTORATION_UNITPELS_VERT_MAX: i32 =
    RESTORATION_UNITSIZE_MAX * 3 / 2 + 2 * RESTORATION_BORDER_VERT + RESTORATION_UNIT_OFFSET;
const RESTORATION_UNITPELS_MAX: i32 = RESTORATION_UNITPELS_HORZ_MAX * RESTORATION_UNITPELS_VERT_MAX;
const SGRPROJ_PRJ_BITS: i32 = 7;
const SGRPROJ_RST_BITS: i32 = 4;
const SGRPROJ_SGR_BITS: i32 = 8;
const SGRPROJ_SGR: i32 = 1 << SGRPROJ_SGR_BITS;
const MAX_RADIUS: i32 = 2;
const SGRPROJ_MTABLE_BITS: i32 = 20;
const SGRPROJ_RECIP_BITS: i32 = 12;

// aom_ports/mem.h
#[inline(always)]
fn CONVERT_TO_SHORTPTR(x: *const u8) -> *mut u16 {
    ((x as usize) << 1) as *mut u16
}

// Constants used for right shift in final_filter calculation.
const NB_EVEN: i32 = 5;
const NB_ODD: i32 = 4;

#[inline(always)]
unsafe fn calc_ab_fast_internal_common(
    mut s0: uint32x4_t,
    mut s1: uint32x4_t,
    mut s2: uint32x4_t,
    mut s3: uint32x4_t,
    mut s4: uint32x4_t,
    mut s5: uint32x4_t,
    mut s6: uint32x4_t,
    mut s7: uint32x4_t,
    sr4: int32x4_t,
    sr5: int32x4_t,
    sr6: int32x4_t,
    sr7: int32x4_t,
    const_n_val: uint32x4_t,
    s_vec: uint32x4_t,
    const_val: uint32x4_t,
    one_by_n_minus_1_vec: uint32x4_t,
    sgrproj_sgr: uint16x4_t,
    src1: *mut i32,
    dst_A16: *mut u16,
    src2: *mut i32,
    buf_stride: i32,
) {
    let mut q0: uint32x4_t;
    let mut q1: uint32x4_t;
    let mut q2: uint32x4_t;
    let mut q3: uint32x4_t;
    let mut p0: uint32x4_t;
    let mut p1: uint32x4_t;
    let mut p2: uint32x4_t;
    let mut p3: uint32x4_t;
    let mut d0: uint16x4_t = core::mem::zeroed();
    let mut d1: uint16x4_t = core::mem::zeroed();
    let mut d2: uint16x4_t = core::mem::zeroed();
    let mut d3: uint16x4_t = core::mem::zeroed();

    s0 = vmulq_u32(s0, const_n_val);
    s1 = vmulq_u32(s1, const_n_val);
    s2 = vmulq_u32(s2, const_n_val);
    s3 = vmulq_u32(s3, const_n_val);

    q0 = vmulq_u32(s4, s4);
    q1 = vmulq_u32(s5, s5);
    q2 = vmulq_u32(s6, s6);
    q3 = vmulq_u32(s7, s7);

    p0 = vcleq_u32(q0, s0);
    p1 = vcleq_u32(q1, s1);
    p2 = vcleq_u32(q2, s2);
    p3 = vcleq_u32(q3, s3);

    q0 = vsubq_u32(s0, q0);
    q1 = vsubq_u32(s1, q1);
    q2 = vsubq_u32(s2, q2);
    q3 = vsubq_u32(s3, q3);

    p0 = vandq_u32(p0, q0);
    p1 = vandq_u32(p1, q1);
    p2 = vandq_u32(p2, q2);
    p3 = vandq_u32(p3, q3);

    p0 = vmulq_u32(p0, s_vec);
    p1 = vmulq_u32(p1, s_vec);
    p2 = vmulq_u32(p2, s_vec);
    p3 = vmulq_u32(p3, s_vec);

    p0 = vrshrq_n_u32::<SGRPROJ_MTABLE_BITS>(p0);
    p1 = vrshrq_n_u32::<SGRPROJ_MTABLE_BITS>(p1);
    p2 = vrshrq_n_u32::<SGRPROJ_MTABLE_BITS>(p2);
    p3 = vrshrq_n_u32::<SGRPROJ_MTABLE_BITS>(p3);

    p0 = vminq_u32(p0, const_val);
    p1 = vminq_u32(p1, const_val);
    p2 = vminq_u32(p2, const_val);
    p3 = vminq_u32(p3, const_val);

    {
        store_u32_4x4(src1 as *mut u32, buf_stride as isize, p0, p1, p2, p3);

        for x in 0..4 {
            for y in 0..4 {
                *dst_A16.offset((x * buf_stride + y) as isize) =
                    av1_x_by_xplus1[*src1.offset((x * buf_stride + y) as isize) as usize] as u16;
            }
        }
        load_u16_4x4(dst_A16, buf_stride as isize, &mut d0, &mut d1, &mut d2, &mut d3);
    }
    p0 = vsubl_u16(sgrproj_sgr, d0);
    p1 = vsubl_u16(sgrproj_sgr, d1);
    p2 = vsubl_u16(sgrproj_sgr, d2);
    p3 = vsubl_u16(sgrproj_sgr, d3);

    s4 = vmulq_u32(vreinterpretq_u32_s32(sr4), one_by_n_minus_1_vec);
    s5 = vmulq_u32(vreinterpretq_u32_s32(sr5), one_by_n_minus_1_vec);
    s6 = vmulq_u32(vreinterpretq_u32_s32(sr6), one_by_n_minus_1_vec);
    s7 = vmulq_u32(vreinterpretq_u32_s32(sr7), one_by_n_minus_1_vec);

    s4 = vmulq_u32(s4, p0);
    s5 = vmulq_u32(s5, p1);
    s6 = vmulq_u32(s6, p2);
    s7 = vmulq_u32(s7, p3);

    p0 = vrshrq_n_u32::<SGRPROJ_RECIP_BITS>(s4);
    p1 = vrshrq_n_u32::<SGRPROJ_RECIP_BITS>(s5);
    p2 = vrshrq_n_u32::<SGRPROJ_RECIP_BITS>(s6);
    p3 = vrshrq_n_u32::<SGRPROJ_RECIP_BITS>(s7);

    store_s32_4x4(
        src2,
        buf_stride as isize,
        vreinterpretq_s32_u32(p0),
        vreinterpretq_s32_u32(p1),
        vreinterpretq_s32_u32(p2),
        vreinterpretq_s32_u32(p3),
    );
}

#[inline(always)]
unsafe fn calc_ab_internal_common(
    mut s0: uint32x4_t,
    mut s1: uint32x4_t,
    mut s2: uint32x4_t,
    mut s3: uint32x4_t,
    mut s4: uint32x4_t,
    mut s5: uint32x4_t,
    mut s6: uint32x4_t,
    mut s7: uint32x4_t,
    s16_0: uint16x8_t,
    s16_1: uint16x8_t,
    s16_2: uint16x8_t,
    s16_3: uint16x8_t,
    mut s16_4: uint16x8_t,
    mut s16_5: uint16x8_t,
    mut s16_6: uint16x8_t,
    mut s16_7: uint16x8_t,
    const_n_val: uint32x4_t,
    s_vec: uint32x4_t,
    const_val: uint32x4_t,
    one_by_n_minus_1_vec: uint16x4_t,
    sgrproj_sgr: uint16x8_t,
    src1: *mut i32,
    dst_A16: *mut u16,
    dst2: *mut i32,
    buf_stride: i32,
) {
    let d0: uint16x4_t;
    let d1: uint16x4_t;
    let d2: uint16x4_t;
    let d3: uint16x4_t;
    let d4: uint16x4_t;
    let d5: uint16x4_t;
    let d6: uint16x4_t;
    let d7: uint16x4_t;
    let mut q0: uint32x4_t;
    let mut q1: uint32x4_t;
    let mut q2: uint32x4_t;
    let mut q3: uint32x4_t;
    let mut q4: uint32x4_t;
    let mut q5: uint32x4_t;
    let mut q6: uint32x4_t;
    let mut q7: uint32x4_t;
    let mut p0: uint32x4_t;
    let mut p1: uint32x4_t;
    let mut p2: uint32x4_t;
    let mut p3: uint32x4_t;
    let mut p4: uint32x4_t;
    let mut p5: uint32x4_t;
    let mut p6: uint32x4_t;
    let mut p7: uint32x4_t;

    s0 = vmulq_u32(s0, const_n_val);
    s1 = vmulq_u32(s1, const_n_val);
    s2 = vmulq_u32(s2, const_n_val);
    s3 = vmulq_u32(s3, const_n_val);
    s4 = vmulq_u32(s4, const_n_val);
    s5 = vmulq_u32(s5, const_n_val);
    s6 = vmulq_u32(s6, const_n_val);
    s7 = vmulq_u32(s7, const_n_val);

    d0 = vget_low_u16(s16_4);
    d1 = vget_low_u16(s16_5);
    d2 = vget_low_u16(s16_6);
    d3 = vget_low_u16(s16_7);
    d4 = vget_high_u16(s16_4);
    d5 = vget_high_u16(s16_5);
    d6 = vget_high_u16(s16_6);
    d7 = vget_high_u16(s16_7);

    q0 = vmull_u16(d0, d0);
    q1 = vmull_u16(d1, d1);
    q2 = vmull_u16(d2, d2);
    q3 = vmull_u16(d3, d3);
    q4 = vmull_u16(d4, d4);
    q5 = vmull_u16(d5, d5);
    q6 = vmull_u16(d6, d6);
    q7 = vmull_u16(d7, d7);

    p0 = vcleq_u32(q0, s0);
    p1 = vcleq_u32(q1, s1);
    p2 = vcleq_u32(q2, s2);
    p3 = vcleq_u32(q3, s3);
    p4 = vcleq_u32(q4, s4);
    p5 = vcleq_u32(q5, s5);
    p6 = vcleq_u32(q6, s6);
    p7 = vcleq_u32(q7, s7);

    q0 = vsubq_u32(s0, q0);
    q1 = vsubq_u32(s1, q1);
    q2 = vsubq_u32(s2, q2);
    q3 = vsubq_u32(s3, q3);
    q4 = vsubq_u32(s4, q4);
    q5 = vsubq_u32(s5, q5);
    q6 = vsubq_u32(s6, q6);
    q7 = vsubq_u32(s7, q7);

    p0 = vandq_u32(p0, q0);
    p1 = vandq_u32(p1, q1);
    p2 = vandq_u32(p2, q2);
    p3 = vandq_u32(p3, q3);
    p4 = vandq_u32(p4, q4);
    p5 = vandq_u32(p5, q5);
    p6 = vandq_u32(p6, q6);
    p7 = vandq_u32(p7, q7);

    p0 = vmulq_u32(p0, s_vec);
    p1 = vmulq_u32(p1, s_vec);
    p2 = vmulq_u32(p2, s_vec);
    p3 = vmulq_u32(p3, s_vec);
    p4 = vmulq_u32(p4, s_vec);
    p5 = vmulq_u32(p5, s_vec);
    p6 = vmulq_u32(p6, s_vec);
    p7 = vmulq_u32(p7, s_vec);

    p0 = vrshrq_n_u32::<SGRPROJ_MTABLE_BITS>(p0);
    p1 = vrshrq_n_u32::<SGRPROJ_MTABLE_BITS>(p1);
    p2 = vrshrq_n_u32::<SGRPROJ_MTABLE_BITS>(p2);
    p3 = vrshrq_n_u32::<SGRPROJ_MTABLE_BITS>(p3);
    p4 = vrshrq_n_u32::<SGRPROJ_MTABLE_BITS>(p4);
    p5 = vrshrq_n_u32::<SGRPROJ_MTABLE_BITS>(p5);
    p6 = vrshrq_n_u32::<SGRPROJ_MTABLE_BITS>(p6);
    p7 = vrshrq_n_u32::<SGRPROJ_MTABLE_BITS>(p7);

    p0 = vminq_u32(p0, const_val);
    p1 = vminq_u32(p1, const_val);
    p2 = vminq_u32(p2, const_val);
    p3 = vminq_u32(p3, const_val);
    p4 = vminq_u32(p4, const_val);
    p5 = vminq_u32(p5, const_val);
    p6 = vminq_u32(p6, const_val);
    p7 = vminq_u32(p7, const_val);

    {
        store_u32_4x4(src1 as *mut u32, buf_stride as isize, p0, p1, p2, p3);
        store_u32_4x4((src1 as *mut u32).offset(4), buf_stride as isize, p4, p5, p6, p7);

        for x in 0..4 {
            for y in 0..8 {
                *dst_A16.offset((x * buf_stride + y) as isize) =
                    av1_x_by_xplus1[*src1.offset((x * buf_stride + y) as isize) as usize] as u16;
            }
        }
        load_u16_8x4(dst_A16, buf_stride as isize, &mut s16_4, &mut s16_5, &mut s16_6, &mut s16_7);
    }

    s16_4 = vsubq_u16(sgrproj_sgr, s16_4);
    s16_5 = vsubq_u16(sgrproj_sgr, s16_5);
    s16_6 = vsubq_u16(sgrproj_sgr, s16_6);
    s16_7 = vsubq_u16(sgrproj_sgr, s16_7);

    s0 = vmull_u16(vget_low_u16(s16_0), one_by_n_minus_1_vec);
    s1 = vmull_u16(vget_low_u16(s16_1), one_by_n_minus_1_vec);
    s2 = vmull_u16(vget_low_u16(s16_2), one_by_n_minus_1_vec);
    s3 = vmull_u16(vget_low_u16(s16_3), one_by_n_minus_1_vec);
    s4 = vmull_u16(vget_high_u16(s16_0), one_by_n_minus_1_vec);
    s5 = vmull_u16(vget_high_u16(s16_1), one_by_n_minus_1_vec);
    s6 = vmull_u16(vget_high_u16(s16_2), one_by_n_minus_1_vec);
    s7 = vmull_u16(vget_high_u16(s16_3), one_by_n_minus_1_vec);

    s0 = vmulq_u32(s0, vmovl_u16(vget_low_u16(s16_4)));
    s1 = vmulq_u32(s1, vmovl_u16(vget_low_u16(s16_5)));
    s2 = vmulq_u32(s2, vmovl_u16(vget_low_u16(s16_6)));
    s3 = vmulq_u32(s3, vmovl_u16(vget_low_u16(s16_7)));
    s4 = vmulq_u32(s4, vmovl_u16(vget_high_u16(s16_4)));
    s5 = vmulq_u32(s5, vmovl_u16(vget_high_u16(s16_5)));
    s6 = vmulq_u32(s6, vmovl_u16(vget_high_u16(s16_6)));
    s7 = vmulq_u32(s7, vmovl_u16(vget_high_u16(s16_7)));

    p0 = vrshrq_n_u32::<SGRPROJ_RECIP_BITS>(s0);
    p1 = vrshrq_n_u32::<SGRPROJ_RECIP_BITS>(s1);
    p2 = vrshrq_n_u32::<SGRPROJ_RECIP_BITS>(s2);
    p3 = vrshrq_n_u32::<SGRPROJ_RECIP_BITS>(s3);
    p4 = vrshrq_n_u32::<SGRPROJ_RECIP_BITS>(s4);
    p5 = vrshrq_n_u32::<SGRPROJ_RECIP_BITS>(s5);
    p6 = vrshrq_n_u32::<SGRPROJ_RECIP_BITS>(s6);
    p7 = vrshrq_n_u32::<SGRPROJ_RECIP_BITS>(s7);

    store_s32_4x4(
        dst2,
        buf_stride as isize,
        vreinterpretq_s32_u32(p0),
        vreinterpretq_s32_u32(p1),
        vreinterpretq_s32_u32(p2),
        vreinterpretq_s32_u32(p3),
    );
    store_s32_4x4(
        dst2.offset(4),
        buf_stride as isize,
        vreinterpretq_s32_u32(p4),
        vreinterpretq_s32_u32(p5),
        vreinterpretq_s32_u32(p6),
        vreinterpretq_s32_u32(p7),
    );
}

#[inline(always)]
unsafe fn boxsum2_square_sum_calc(
    t1: int16x4_t,
    t2: int16x4_t,
    t3: int16x4_t,
    t4: int16x4_t,
    t5: int16x4_t,
    t6: int16x4_t,
    t7: int16x4_t,
    t8: int16x4_t,
    t9: int16x4_t,
    t10: int16x4_t,
    t11: int16x4_t,
    r0: *mut int32x4_t,
    r1: *mut int32x4_t,
    r2: *mut int32x4_t,
    r3: *mut int32x4_t,
) {
    let d1: int32x4_t = vmull_s16(t1, t1);
    let d2: int32x4_t = vmull_s16(t2, t2);
    let d3: int32x4_t = vmull_s16(t3, t3);
    let d4: int32x4_t = vmull_s16(t4, t4);
    let d5: int32x4_t = vmull_s16(t5, t5);
    let d6: int32x4_t = vmull_s16(t6, t6);
    let d7: int32x4_t = vmull_s16(t7, t7);
    let d8: int32x4_t = vmull_s16(t8, t8);
    let d9: int32x4_t = vmull_s16(t9, t9);
    let d10: int32x4_t = vmull_s16(t10, t10);
    let d11: int32x4_t = vmull_s16(t11, t11);

    let r12: int32x4_t = vaddq_s32(d1, d2);
    let r34: int32x4_t = vaddq_s32(d3, d4);
    let r67: int32x4_t = vaddq_s32(d6, d7);
    let r89: int32x4_t = vaddq_s32(d8, d9);
    let r1011: int32x4_t = vaddq_s32(d10, d11);
    let r345: int32x4_t = vaddq_s32(r34, d5);
    let r6789: int32x4_t = vaddq_s32(r67, r89);
    let r789: int32x4_t = vsubq_s32(r6789, d6);
    *r0 = vaddq_s32(r12, r345);
    *r1 = vaddq_s32(r67, r345);
    *r2 = vaddq_s32(d5, r6789);
    *r3 = vaddq_s32(r789, r1011);
}

#[inline(always)]
unsafe fn boxsum2(
    src: *mut i16,
    src_stride: i32,
    dst16: *mut i16,
    dst32: *mut i32,
    dst2: *mut i32,
    dst_stride: i32,
    width: i32,
    height: i32,
) {
    debug_assert!(width > 2 * SGRPROJ_BORDER_HORZ);
    debug_assert!(height > 2 * SGRPROJ_BORDER_VERT);

    let mut dst1_16_ptr: *mut i16;
    let mut src_ptr: *mut i16;
    let mut dst2_ptr: *mut i32;
    let mut h: i32;
    let mut w: i32;
    let mut count: i32 = 0;
    let dst_stride_2: i32 = dst_stride << 1;
    let dst_stride_8: i32 = dst_stride << 3;

    dst1_16_ptr = dst16;
    dst2_ptr = dst2;
    src_ptr = src;
    w = width;
    {
        let mut t1: int16x8_t = core::mem::zeroed();
        let mut t2: int16x8_t = core::mem::zeroed();
        let mut t3: int16x8_t = core::mem::zeroed();
        let mut t4: int16x8_t = core::mem::zeroed();
        let mut t5: int16x8_t = core::mem::zeroed();
        let mut t6: int16x8_t = core::mem::zeroed();
        let mut t7: int16x8_t = core::mem::zeroed();
        let mut t8: int16x8_t = core::mem::zeroed();
        let mut t9: int16x8_t = core::mem::zeroed();
        let mut t10: int16x8_t = core::mem::zeroed();
        let mut t11: int16x8_t = core::mem::zeroed();
        let mut t12: int16x8_t = core::mem::zeroed();

        let mut r12345: int32x4_t = core::mem::zeroed();
        let mut r56789: int32x4_t = core::mem::zeroed();
        let mut r34567: int32x4_t = core::mem::zeroed();
        let mut r7891011: int32x4_t = core::mem::zeroed();

        loop {
            h = height;
            dst1_16_ptr = dst16.offset((count << 3) as isize);
            dst2_ptr = dst2.offset((count << 3) as isize);
            src_ptr = src.offset((count << 3) as isize);

            dst1_16_ptr = dst1_16_ptr.offset(dst_stride_2 as isize);
            dst2_ptr = dst2_ptr.offset(dst_stride_2 as isize);
            loop {
                load_s16_8x4(src_ptr, src_stride as isize, &mut t1, &mut t2, &mut t3, &mut t4);
                src_ptr = src_ptr.offset((4 * src_stride) as isize);
                load_s16_8x4(src_ptr, src_stride as isize, &mut t5, &mut t6, &mut t7, &mut t8);
                src_ptr = src_ptr.offset((4 * src_stride) as isize);
                load_s16_8x4(src_ptr, src_stride as isize, &mut t9, &mut t10, &mut t11, &mut t12);

                let q12: int16x8_t = vaddq_s16(t1, t2);
                let q34: int16x8_t = vaddq_s16(t3, t4);
                let q67: int16x8_t = vaddq_s16(t6, t7);
                let q89: int16x8_t = vaddq_s16(t8, t9);
                let q1011: int16x8_t = vaddq_s16(t10, t11);
                let q345: int16x8_t = vaddq_s16(q34, t5);
                let q6789: int16x8_t = vaddq_s16(q67, q89);
                let q789: int16x8_t = vaddq_s16(q89, t7);
                let q12345: int16x8_t = vaddq_s16(q12, q345);
                let q34567: int16x8_t = vaddq_s16(q67, q345);
                let q56789: int16x8_t = vaddq_s16(t5, q6789);
                let q7891011: int16x8_t = vaddq_s16(q789, q1011);

                store_s16_8x4(dst1_16_ptr, dst_stride_2 as isize, q12345, q34567, q56789, q7891011);
                dst1_16_ptr = dst1_16_ptr.offset(dst_stride_8 as isize);

                boxsum2_square_sum_calc(
                    vget_low_s16(t1),
                    vget_low_s16(t2),
                    vget_low_s16(t3),
                    vget_low_s16(t4),
                    vget_low_s16(t5),
                    vget_low_s16(t6),
                    vget_low_s16(t7),
                    vget_low_s16(t8),
                    vget_low_s16(t9),
                    vget_low_s16(t10),
                    vget_low_s16(t11),
                    &mut r12345,
                    &mut r34567,
                    &mut r56789,
                    &mut r7891011,
                );

                store_s32_4x4(dst2_ptr, dst_stride_2 as isize, r12345, r34567, r56789, r7891011);

                boxsum2_square_sum_calc(
                    vget_high_s16(t1),
                    vget_high_s16(t2),
                    vget_high_s16(t3),
                    vget_high_s16(t4),
                    vget_high_s16(t5),
                    vget_high_s16(t6),
                    vget_high_s16(t7),
                    vget_high_s16(t8),
                    vget_high_s16(t9),
                    vget_high_s16(t10),
                    vget_high_s16(t11),
                    &mut r12345,
                    &mut r34567,
                    &mut r56789,
                    &mut r7891011,
                );

                store_s32_4x4(
                    dst2_ptr.offset(4),
                    dst_stride_2 as isize,
                    r12345,
                    r34567,
                    r56789,
                    r7891011,
                );
                dst2_ptr = dst2_ptr.offset(dst_stride_8 as isize);
                h -= 8;
                if !(h > 0) {
                    break;
                }
            }
            w -= 8;
            count += 1;
            if !(w > 0) {
                break;
            }
        }

        // memset needed for row pixels as 2nd stage of boxsum filter uses
        // first 2 rows of dst16, dst2 buffer which is not filled in first stage.
        for x in 0..2 {
            ptr::write_bytes(dst16.offset((x * dst_stride) as isize), 0, (width + 4) as usize);
            ptr::write_bytes(dst2.offset((x * dst_stride) as isize), 0, (width + 4) as usize);
        }

        // memset needed for extra columns as 2nd stage of boxsum filter uses
        // last 2 columns of dst16, dst2 buffer which is not filled in first stage.
        for x in 2..(height + 2) {
            let dst_offset: i32 = x * dst_stride + width + 2;
            ptr::write_bytes(dst16.offset(dst_offset as isize), 0, 3);
            ptr::write_bytes(dst2.offset(dst_offset as isize), 0, 3);
        }
    }

    {
        let mut s1: int16x4_t = core::mem::zeroed();
        let mut s2: int16x4_t = core::mem::zeroed();
        let mut s3: int16x4_t = core::mem::zeroed();
        let mut s4: int16x4_t = core::mem::zeroed();
        let mut s5: int16x4_t = core::mem::zeroed();
        let mut s6: int16x4_t = core::mem::zeroed();
        let mut s7: int16x4_t = core::mem::zeroed();
        let mut s8: int16x4_t = core::mem::zeroed();
        let mut d1: int32x4_t = core::mem::zeroed();
        let mut d2: int32x4_t = core::mem::zeroed();
        let mut d3: int32x4_t = core::mem::zeroed();
        let mut d4: int32x4_t = core::mem::zeroed();
        let mut d5: int32x4_t = core::mem::zeroed();
        let mut d6: int32x4_t = core::mem::zeroed();
        let mut d7: int32x4_t = core::mem::zeroed();
        let mut d8: int32x4_t = core::mem::zeroed();

        let mut src2_ptr: *mut i32;
        let mut dst1_32_ptr: *mut i32;
        let mut src1_ptr: *mut i16;
        count = 0;
        h = height;
        loop {
            dst1_32_ptr = dst32.offset((count * dst_stride_8 + dst_stride_2) as isize);
            dst2_ptr = dst2.offset((count * dst_stride_8 + dst_stride_2) as isize);
            src1_ptr = dst16.offset((count * dst_stride_8 + dst_stride_2) as isize);
            src2_ptr = dst2.offset((count * dst_stride_8 + dst_stride_2) as isize);
            w = width;

            dst1_32_ptr = dst1_32_ptr.offset(2);
            dst2_ptr = dst2_ptr.offset(2);
            load_s16_4x4(src1_ptr, dst_stride_2 as isize, &mut s1, &mut s2, &mut s3, &mut s4);
            transpose_elems_inplace_s16_4x4(&mut s1, &mut s2, &mut s3, &mut s4);
            load_s32_4x4(src2_ptr, dst_stride_2 as isize, &mut d1, &mut d2, &mut d3, &mut d4);
            transpose_elems_inplace_s32_4x4(&mut d1, &mut d2, &mut d3, &mut d4);
            loop {
                src1_ptr = src1_ptr.offset(4);
                src2_ptr = src2_ptr.offset(4);
                load_s16_4x4(src1_ptr, dst_stride_2 as isize, &mut s5, &mut s6, &mut s7, &mut s8);
                transpose_elems_inplace_s16_4x4(&mut s5, &mut s6, &mut s7, &mut s8);
                load_s32_4x4(src2_ptr, dst_stride_2 as isize, &mut d5, &mut d6, &mut d7, &mut d8);
                transpose_elems_inplace_s32_4x4(&mut d5, &mut d6, &mut d7, &mut d8);
                let q23: int32x4_t = vaddl_s16(s2, s3);
                let q45: int32x4_t = vaddl_s16(s4, s5);
                let q67: int32x4_t = vaddl_s16(s6, s7);
                let q2345: int32x4_t = vaddq_s32(q23, q45);
                let q4567: int32x4_t = vaddq_s32(q45, q67);
                let mut q12345: int32x4_t = vaddq_s32(vmovl_s16(s1), q2345);
                let mut q23456: int32x4_t = vaddq_s32(q2345, vmovl_s16(s6));
                let mut q34567: int32x4_t = vaddq_s32(q4567, vmovl_s16(s3));
                let mut q45678: int32x4_t = vaddq_s32(q4567, vmovl_s16(s8));

                transpose_elems_inplace_s32_4x4(&mut q12345, &mut q23456, &mut q34567, &mut q45678);
                store_s32_4x4(dst1_32_ptr, dst_stride_2 as isize, q12345, q23456, q34567, q45678);
                dst1_32_ptr = dst1_32_ptr.offset(4);
                s1 = s5;
                s2 = s6;
                s3 = s7;
                s4 = s8;

                let r23: int32x4_t = vaddq_s32(d2, d3);
                let r45: int32x4_t = vaddq_s32(d4, d5);
                let r67: int32x4_t = vaddq_s32(d6, d7);
                let r2345: int32x4_t = vaddq_s32(r23, r45);
                let r4567: int32x4_t = vaddq_s32(r45, r67);
                let mut r12345: int32x4_t = vaddq_s32(d1, r2345);
                let mut r23456: int32x4_t = vaddq_s32(r2345, d6);
                let mut r34567: int32x4_t = vaddq_s32(r4567, d3);
                let mut r45678: int32x4_t = vaddq_s32(r4567, d8);

                transpose_elems_inplace_s32_4x4(&mut r12345, &mut r23456, &mut r34567, &mut r45678);
                store_s32_4x4(dst2_ptr, dst_stride_2 as isize, r12345, r23456, r34567, r45678);
                dst2_ptr = dst2_ptr.offset(4);
                d1 = d5;
                d2 = d6;
                d3 = d7;
                d4 = d8;
                w -= 4;
                if !(w > 0) {
                    break;
                }
            }
            h -= 8;
            count += 1;
            if !(h > 0) {
                break;
            }
        }
    }
}

#[inline(always)]
unsafe fn calc_ab_internal_lbd(
    A: *mut i32,
    A16: *mut u16,
    B16: *mut u16,
    B: *mut i32,
    buf_stride: i32,
    width: i32,
    height: i32,
    r: i32,
    s: i32,
    ht_inc: i32,
) {
    let mut src1: *mut i32;
    let mut dst2: *mut i32;
    let mut count: i32 = 0;
    let mut dst_A16: *mut u16;
    let mut src2: *mut u16;
    let n: u32 = ((2 * r + 1) * (2 * r + 1)) as u32;
    let const_n_val: uint32x4_t = vdupq_n_u32(n);
    let sgrproj_sgr: uint16x8_t = vdupq_n_u16(SGRPROJ_SGR as u16);
    let one_by_n_minus_1_vec: uint16x4_t = vdup_n_u16(av1_one_by_x[(n - 1) as usize] as u16);
    let const_val: uint32x4_t = vdupq_n_u32(255);

    let mut s16_0: uint16x8_t = core::mem::zeroed();
    let mut s16_1: uint16x8_t = core::mem::zeroed();
    let mut s16_2: uint16x8_t = core::mem::zeroed();
    let mut s16_3: uint16x8_t = core::mem::zeroed();
    let mut s16_4: uint16x8_t;
    let mut s16_5: uint16x8_t;
    let mut s16_6: uint16x8_t;
    let mut s16_7: uint16x8_t;

    let mut s0: uint32x4_t = core::mem::zeroed();
    let mut s1: uint32x4_t = core::mem::zeroed();
    let mut s2: uint32x4_t = core::mem::zeroed();
    let mut s3: uint32x4_t = core::mem::zeroed();
    let mut s4: uint32x4_t = core::mem::zeroed();
    let mut s5: uint32x4_t = core::mem::zeroed();
    let mut s6: uint32x4_t = core::mem::zeroed();
    let mut s7: uint32x4_t = core::mem::zeroed();

    let s_vec: uint32x4_t = vdupq_n_u32(s as u32);
    let mut w: i32;
    let mut h: i32 = height;

    loop {
        dst_A16 = A16.offset(((count << 2) * buf_stride) as isize);
        src1 = A.offset(((count << 2) * buf_stride) as isize);
        src2 = B16.offset(((count << 2) * buf_stride) as isize);
        dst2 = B.offset(((count << 2) * buf_stride) as isize);
        w = width;
        loop {
            load_u32_4x4(src1 as *mut u32, buf_stride as isize, &mut s0, &mut s1, &mut s2, &mut s3);
            load_u32_4x4(
                (src1 as *mut u32).offset(4),
                buf_stride as isize,
                &mut s4,
                &mut s5,
                &mut s6,
                &mut s7,
            );
            load_u16_8x4(src2, buf_stride as isize, &mut s16_0, &mut s16_1, &mut s16_2, &mut s16_3);

            s16_4 = s16_0;
            s16_5 = s16_1;
            s16_6 = s16_2;
            s16_7 = s16_3;

            calc_ab_internal_common(
                s0,
                s1,
                s2,
                s3,
                s4,
                s5,
                s6,
                s7,
                s16_0,
                s16_1,
                s16_2,
                s16_3,
                s16_4,
                s16_5,
                s16_6,
                s16_7,
                const_n_val,
                s_vec,
                const_val,
                one_by_n_minus_1_vec,
                sgrproj_sgr,
                src1,
                dst_A16,
                dst2,
                buf_stride,
            );

            w -= 8;
            dst2 = dst2.offset(8);
            src1 = src1.offset(8);
            src2 = src2.offset(8);
            dst_A16 = dst_A16.offset(8);
            if !(w > 0) {
                break;
            }
        }
        count += 1;
        h -= ht_inc * 4;
        if !(h > 0) {
            break;
        }
    }
}

#[inline(always)]
unsafe fn calc_ab_internal_hbd(
    A: *mut i32,
    A16: *mut u16,
    B16: *mut u16,
    B: *mut i32,
    buf_stride: i32,
    width: i32,
    height: i32,
    bit_depth: i32,
    r: i32,
    s: i32,
    ht_inc: i32,
) {
    let mut src1: *mut i32;
    let mut dst2: *mut i32;
    let mut count: i32 = 0;
    let mut dst_A16: *mut u16;
    let mut src2: *mut u16;
    let n: u32 = ((2 * r + 1) * (2 * r + 1)) as u32;
    let bd_min_2_vec: int16x8_t = vdupq_n_s16(-(bit_depth - 8) as i16);
    let bd_min_1_vec: int32x4_t = vdupq_n_s32(-((bit_depth - 8) << 1));
    let const_n_val: uint32x4_t = vdupq_n_u32(n);
    let sgrproj_sgr: uint16x8_t = vdupq_n_u16(SGRPROJ_SGR as u16);
    let one_by_n_minus_1_vec: uint16x4_t = vdup_n_u16(av1_one_by_x[(n - 1) as usize] as u16);
    let const_val: uint32x4_t = vdupq_n_u32(255);

    let mut sr0: int32x4_t = core::mem::zeroed();
    let mut sr1: int32x4_t = core::mem::zeroed();
    let mut sr2: int32x4_t = core::mem::zeroed();
    let mut sr3: int32x4_t = core::mem::zeroed();
    let mut sr4: int32x4_t = core::mem::zeroed();
    let mut sr5: int32x4_t = core::mem::zeroed();
    let mut sr6: int32x4_t = core::mem::zeroed();
    let mut sr7: int32x4_t = core::mem::zeroed();
    let mut s16_0: uint16x8_t = core::mem::zeroed();
    let mut s16_1: uint16x8_t = core::mem::zeroed();
    let mut s16_2: uint16x8_t = core::mem::zeroed();
    let mut s16_3: uint16x8_t = core::mem::zeroed();
    let mut s16_4: uint16x8_t;
    let mut s16_5: uint16x8_t;
    let mut s16_6: uint16x8_t;
    let mut s16_7: uint16x8_t;
    let mut s0: uint32x4_t;
    let mut s1: uint32x4_t;
    let mut s2: uint32x4_t;
    let mut s3: uint32x4_t;
    let mut s4: uint32x4_t;
    let mut s5: uint32x4_t;
    let mut s6: uint32x4_t;
    let mut s7: uint32x4_t;

    let s_vec: uint32x4_t = vdupq_n_u32(s as u32);
    let mut w: i32;
    let mut h: i32 = height;

    loop {
        src1 = A.offset(((count << 2) * buf_stride) as isize);
        src2 = B16.offset(((count << 2) * buf_stride) as isize);
        dst2 = B.offset(((count << 2) * buf_stride) as isize);
        dst_A16 = A16.offset(((count << 2) * buf_stride) as isize);
        w = width;
        loop {
            load_s32_4x4(src1, buf_stride as isize, &mut sr0, &mut sr1, &mut sr2, &mut sr3);
            load_s32_4x4(src1.offset(4), buf_stride as isize, &mut sr4, &mut sr5, &mut sr6, &mut sr7);
            load_u16_8x4(src2, buf_stride as isize, &mut s16_0, &mut s16_1, &mut s16_2, &mut s16_3);

            s0 = vrshlq_u32(vreinterpretq_u32_s32(sr0), bd_min_1_vec);
            s1 = vrshlq_u32(vreinterpretq_u32_s32(sr1), bd_min_1_vec);
            s2 = vrshlq_u32(vreinterpretq_u32_s32(sr2), bd_min_1_vec);
            s3 = vrshlq_u32(vreinterpretq_u32_s32(sr3), bd_min_1_vec);
            s4 = vrshlq_u32(vreinterpretq_u32_s32(sr4), bd_min_1_vec);
            s5 = vrshlq_u32(vreinterpretq_u32_s32(sr5), bd_min_1_vec);
            s6 = vrshlq_u32(vreinterpretq_u32_s32(sr6), bd_min_1_vec);
            s7 = vrshlq_u32(vreinterpretq_u32_s32(sr7), bd_min_1_vec);

            s16_4 = vrshlq_u16(s16_0, bd_min_2_vec);
            s16_5 = vrshlq_u16(s16_1, bd_min_2_vec);
            s16_6 = vrshlq_u16(s16_2, bd_min_2_vec);
            s16_7 = vrshlq_u16(s16_3, bd_min_2_vec);

            calc_ab_internal_common(
                s0,
                s1,
                s2,
                s3,
                s4,
                s5,
                s6,
                s7,
                s16_0,
                s16_1,
                s16_2,
                s16_3,
                s16_4,
                s16_5,
                s16_6,
                s16_7,
                const_n_val,
                s_vec,
                const_val,
                one_by_n_minus_1_vec,
                sgrproj_sgr,
                src1,
                dst_A16,
                dst2,
                buf_stride,
            );

            w -= 8;
            dst2 = dst2.offset(8);
            src1 = src1.offset(8);
            src2 = src2.offset(8);
            dst_A16 = dst_A16.offset(8);
            if !(w > 0) {
                break;
            }
        }
        count += 1;
        h -= ht_inc * 4;
        if !(h > 0) {
            break;
        }
    }
}

#[inline(always)]
unsafe fn calc_ab_fast_internal_lbd(
    A: *mut i32,
    A16: *mut u16,
    B: *mut i32,
    buf_stride: i32,
    width: i32,
    height: i32,
    r: i32,
    s: i32,
    ht_inc: i32,
) {
    let mut src1: *mut i32;
    let mut src2: *mut i32;
    let mut count: i32 = 0;
    let mut dst_A16: *mut u16;
    let n: u32 = ((2 * r + 1) * (2 * r + 1)) as u32;
    let const_n_val: uint32x4_t = vdupq_n_u32(n);
    let sgrproj_sgr: uint16x4_t = vdup_n_u16(SGRPROJ_SGR as u16);
    let one_by_n_minus_1_vec: uint32x4_t = vdupq_n_u32(av1_one_by_x[(n - 1) as usize] as u32);
    let const_val: uint32x4_t = vdupq_n_u32(255);

    let mut sr0: int32x4_t = core::mem::zeroed();
    let mut sr1: int32x4_t = core::mem::zeroed();
    let mut sr2: int32x4_t = core::mem::zeroed();
    let mut sr3: int32x4_t = core::mem::zeroed();
    let mut sr4: int32x4_t = core::mem::zeroed();
    let mut sr5: int32x4_t = core::mem::zeroed();
    let mut sr6: int32x4_t = core::mem::zeroed();
    let mut sr7: int32x4_t = core::mem::zeroed();
    let mut s0: uint32x4_t;
    let mut s1: uint32x4_t;
    let mut s2: uint32x4_t;
    let mut s3: uint32x4_t;
    let mut s4: uint32x4_t;
    let mut s5: uint32x4_t;
    let mut s6: uint32x4_t;
    let mut s7: uint32x4_t;

    let s_vec: uint32x4_t = vdupq_n_u32(s as u32);
    let mut w: i32;
    let mut h: i32 = height;

    loop {
        src1 = A.offset(((count << 2) * buf_stride) as isize);
        src2 = B.offset(((count << 2) * buf_stride) as isize);
        dst_A16 = A16.offset(((count << 2) * buf_stride) as isize);
        w = width;
        loop {
            load_s32_4x4(src1, buf_stride as isize, &mut sr0, &mut sr1, &mut sr2, &mut sr3);
            load_s32_4x4(src2, buf_stride as isize, &mut sr4, &mut sr5, &mut sr6, &mut sr7);

            s0 = vreinterpretq_u32_s32(sr0);
            s1 = vreinterpretq_u32_s32(sr1);
            s2 = vreinterpretq_u32_s32(sr2);
            s3 = vreinterpretq_u32_s32(sr3);
            s4 = vreinterpretq_u32_s32(sr4);
            s5 = vreinterpretq_u32_s32(sr5);
            s6 = vreinterpretq_u32_s32(sr6);
            s7 = vreinterpretq_u32_s32(sr7);

            calc_ab_fast_internal_common(
                s0,
                s1,
                s2,
                s3,
                s4,
                s5,
                s6,
                s7,
                sr4,
                sr5,
                sr6,
                sr7,
                const_n_val,
                s_vec,
                const_val,
                one_by_n_minus_1_vec,
                sgrproj_sgr,
                src1,
                dst_A16,
                src2,
                buf_stride,
            );

            w -= 4;
            src1 = src1.offset(4);
            src2 = src2.offset(4);
            dst_A16 = dst_A16.offset(4);
            if !(w > 0) {
                break;
            }
        }
        count += 1;
        h -= ht_inc * 4;
        if !(h > 0) {
            break;
        }
    }
}

#[inline(always)]
unsafe fn calc_ab_fast_internal_hbd(
    A: *mut i32,
    A16: *mut u16,
    B: *mut i32,
    buf_stride: i32,
    width: i32,
    height: i32,
    bit_depth: i32,
    r: i32,
    s: i32,
    ht_inc: i32,
) {
    let mut src1: *mut i32;
    let mut src2: *mut i32;
    let mut count: i32 = 0;
    let mut dst_A16: *mut u16;
    let n: u32 = ((2 * r + 1) * (2 * r + 1)) as u32;
    let bd_min_2_vec: int32x4_t = vdupq_n_s32(-(bit_depth - 8));
    let bd_min_1_vec: int32x4_t = vdupq_n_s32(-((bit_depth - 8) << 1));
    let const_n_val: uint32x4_t = vdupq_n_u32(n);
    let sgrproj_sgr: uint16x4_t = vdup_n_u16(SGRPROJ_SGR as u16);
    let one_by_n_minus_1_vec: uint32x4_t = vdupq_n_u32(av1_one_by_x[(n - 1) as usize] as u32);
    let const_val: uint32x4_t = vdupq_n_u32(255);

    let mut sr0: int32x4_t = core::mem::zeroed();
    let mut sr1: int32x4_t = core::mem::zeroed();
    let mut sr2: int32x4_t = core::mem::zeroed();
    let mut sr3: int32x4_t = core::mem::zeroed();
    let mut sr4: int32x4_t = core::mem::zeroed();
    let mut sr5: int32x4_t = core::mem::zeroed();
    let mut sr6: int32x4_t = core::mem::zeroed();
    let mut sr7: int32x4_t = core::mem::zeroed();
    let mut s0: uint32x4_t;
    let mut s1: uint32x4_t;
    let mut s2: uint32x4_t;
    let mut s3: uint32x4_t;
    let mut s4: uint32x4_t;
    let mut s5: uint32x4_t;
    let mut s6: uint32x4_t;
    let mut s7: uint32x4_t;

    let s_vec: uint32x4_t = vdupq_n_u32(s as u32);
    let mut w: i32;
    let mut h: i32 = height;

    loop {
        src1 = A.offset(((count << 2) * buf_stride) as isize);
        src2 = B.offset(((count << 2) * buf_stride) as isize);
        dst_A16 = A16.offset(((count << 2) * buf_stride) as isize);
        w = width;
        loop {
            load_s32_4x4(src1, buf_stride as isize, &mut sr0, &mut sr1, &mut sr2, &mut sr3);
            load_s32_4x4(src2, buf_stride as isize, &mut sr4, &mut sr5, &mut sr6, &mut sr7);

            s0 = vrshlq_u32(vreinterpretq_u32_s32(sr0), bd_min_1_vec);
            s1 = vrshlq_u32(vreinterpretq_u32_s32(sr1), bd_min_1_vec);
            s2 = vrshlq_u32(vreinterpretq_u32_s32(sr2), bd_min_1_vec);
            s3 = vrshlq_u32(vreinterpretq_u32_s32(sr3), bd_min_1_vec);
            s4 = vrshlq_u32(vreinterpretq_u32_s32(sr4), bd_min_2_vec);
            s5 = vrshlq_u32(vreinterpretq_u32_s32(sr5), bd_min_2_vec);
            s6 = vrshlq_u32(vreinterpretq_u32_s32(sr6), bd_min_2_vec);
            s7 = vrshlq_u32(vreinterpretq_u32_s32(sr7), bd_min_2_vec);

            calc_ab_fast_internal_common(
                s0,
                s1,
                s2,
                s3,
                s4,
                s5,
                s6,
                s7,
                sr4,
                sr5,
                sr6,
                sr7,
                const_n_val,
                s_vec,
                const_val,
                one_by_n_minus_1_vec,
                sgrproj_sgr,
                src1,
                dst_A16,
                src2,
                buf_stride,
            );

            w -= 4;
            src1 = src1.offset(4);
            src2 = src2.offset(4);
            dst_A16 = dst_A16.offset(4);
            if !(w > 0) {
                break;
            }
        }
        count += 1;
        h -= ht_inc * 4;
        if !(h > 0) {
            break;
        }
    }
}

#[inline(always)]
unsafe fn boxsum1(
    src: *mut i16,
    src_stride: i32,
    dst1: *mut u16,
    dst2: *mut i32,
    dst_stride: i32,
    width: i32,
    height: i32,
) {
    debug_assert!(width > 2 * SGRPROJ_BORDER_HORZ);
    debug_assert!(height > 2 * SGRPROJ_BORDER_VERT);

    let mut src_ptr: *mut i16;
    let mut dst2_ptr: *mut i32;
    let mut dst1_ptr: *mut u16;
    let mut h: i32;
    let mut w: i32;
    let mut count: i32 = 0;

    w = width;
    {
        let mut s1: int16x8_t = core::mem::zeroed();
        let mut s2: int16x8_t = core::mem::zeroed();
        let mut s3: int16x8_t = core::mem::zeroed();
        let mut s4: int16x8_t = core::mem::zeroed();
        let mut s5: int16x8_t = core::mem::zeroed();
        let mut s6: int16x8_t = core::mem::zeroed();
        let mut s7: int16x8_t = core::mem::zeroed();
        let mut s8: int16x8_t = core::mem::zeroed();
        let mut q23: int16x8_t;
        let mut q34: int16x8_t;
        let mut q56: int16x8_t;
        let mut q234: int16x8_t;
        let mut q345: int16x8_t;
        let mut q456: int16x8_t;
        let mut q567: int16x8_t;
        let mut r23: int32x4_t;
        let mut r56: int32x4_t;
        let mut r345: int32x4_t;
        let mut r456: int32x4_t;
        let mut r567: int32x4_t;
        let mut r78: int32x4_t;
        let mut r678: int32x4_t;
        let mut r4_low: int32x4_t;
        let mut r4_high: int32x4_t;
        let mut r34_low: int32x4_t;
        let mut r34_high: int32x4_t;
        let mut r234_low: int32x4_t;
        let mut r234_high: int32x4_t;
        let mut r2: int32x4_t;
        let mut r3: int32x4_t;
        let mut r5: int32x4_t;
        let mut r6: int32x4_t;
        let mut r7: int32x4_t;
        let mut r8: int32x4_t;
        let mut q678: int16x8_t;
        let mut q78: int16x8_t;

        loop {
            dst1_ptr = dst1.offset((count << 3) as isize);
            dst2_ptr = dst2.offset((count << 3) as isize);
            src_ptr = src.offset((count << 3) as isize);
            h = height;

            load_s16_8x4(src_ptr, src_stride as isize, &mut s1, &mut s2, &mut s3, &mut s4);
            src_ptr = src_ptr.offset((4 * src_stride) as isize);

            q23 = vaddq_s16(s2, s3);
            q234 = vaddq_s16(q23, s4);
            q34 = vaddq_s16(s3, s4);
            dst1_ptr = dst1_ptr.offset((dst_stride << 1) as isize);

            r2 = vmull_s16(vget_low_s16(s2), vget_low_s16(s2));
            r3 = vmull_s16(vget_low_s16(s3), vget_low_s16(s3));
            r4_low = vmull_s16(vget_low_s16(s4), vget_low_s16(s4));
            r23 = vaddq_s32(r2, r3);
            r234_low = vaddq_s32(r23, r4_low);
            r34_low = vaddq_s32(r3, r4_low);

            r2 = vmull_s16(vget_high_s16(s2), vget_high_s16(s2));
            r3 = vmull_s16(vget_high_s16(s3), vget_high_s16(s3));
            r4_high = vmull_s16(vget_high_s16(s4), vget_high_s16(s4));
            r23 = vaddq_s32(r2, r3);
            r234_high = vaddq_s32(r23, r4_high);
            r34_high = vaddq_s32(r3, r4_high);

            dst2_ptr = dst2_ptr.offset((dst_stride << 1) as isize);

            loop {
                load_s16_8x4(src_ptr, src_stride as isize, &mut s5, &mut s6, &mut s7, &mut s8);
                src_ptr = src_ptr.offset((4 * src_stride) as isize);

                q345 = vaddq_s16(s5, q34);
                q56 = vaddq_s16(s5, s6);
                q456 = vaddq_s16(s4, q56);
                q567 = vaddq_s16(s7, q56);
                q78 = vaddq_s16(s7, s8);
                q678 = vaddq_s16(s6, q78);

                store_s16_8x4(dst1_ptr as *mut i16, dst_stride as isize, q234, q345, q456, q567);
                dst1_ptr = dst1_ptr.offset((dst_stride << 2) as isize);

                s4 = s8;
                q34 = q78;
                q234 = q678;

                r5 = vmull_s16(vget_low_s16(s5), vget_low_s16(s5));
                r6 = vmull_s16(vget_low_s16(s6), vget_low_s16(s6));
                r7 = vmull_s16(vget_low_s16(s7), vget_low_s16(s7));
                r8 = vmull_s16(vget_low_s16(s8), vget_low_s16(s8));

                r345 = vaddq_s32(r5, r34_low);
                r56 = vaddq_s32(r5, r6);
                r456 = vaddq_s32(r4_low, r56);
                r567 = vaddq_s32(r7, r56);
                r78 = vaddq_s32(r7, r8);
                r678 = vaddq_s32(r6, r78);
                store_s32_4x4(dst2_ptr, dst_stride as isize, r234_low, r345, r456, r567);

                r4_low = r8;
                r34_low = r78;
                r234_low = r678;

                r5 = vmull_s16(vget_high_s16(s5), vget_high_s16(s5));
                r6 = vmull_s16(vget_high_s16(s6), vget_high_s16(s6));
                r7 = vmull_s16(vget_high_s16(s7), vget_high_s16(s7));
                r8 = vmull_s16(vget_high_s16(s8), vget_high_s16(s8));

                r345 = vaddq_s32(r5, r34_high);
                r56 = vaddq_s32(r5, r6);
                r456 = vaddq_s32(r4_high, r56);
                r567 = vaddq_s32(r7, r56);
                r78 = vaddq_s32(r7, r8);
                r678 = vaddq_s32(r6, r78);
                store_s32_4x4(dst2_ptr.offset(4), dst_stride as isize, r234_high, r345, r456, r567);
                dst2_ptr = dst2_ptr.offset((dst_stride << 2) as isize);

                r4_high = r8;
                r34_high = r78;
                r234_high = r678;

                h -= 4;
                if !(h > 0) {
                    break;
                }
            }
            w -= 8;
            count += 1;
            if !(w > 0) {
                break;
            }
        }

        // memset needed for row pixels as 2nd stage of boxsum filter uses
        // first 2 rows of dst1, dst2 buffer which is not filled in first stage.
        for x in 0..2 {
            ptr::write_bytes(dst1.offset((x * dst_stride) as isize), 0, (width + 4) as usize);
            ptr::write_bytes(dst2.offset((x * dst_stride) as isize), 0, (width + 4) as usize);
        }

        // memset needed for extra columns as 2nd stage of boxsum filter uses
        // last 2 columns of dst1, dst2 buffer which is not filled in first stage.
        for x in 2..(height + 2) {
            let dst_offset: i32 = x * dst_stride + width + 2;
            ptr::write_bytes(dst1.offset(dst_offset as isize), 0, 3);
            ptr::write_bytes(dst2.offset(dst_offset as isize), 0, 3);
        }
    }

    {
        let mut d1: int16x4_t = core::mem::zeroed();
        let mut d2: int16x4_t = core::mem::zeroed();
        let mut d3: int16x4_t = core::mem::zeroed();
        let mut d4: int16x4_t = core::mem::zeroed();
        let mut d5: int16x4_t = core::mem::zeroed();
        let mut d6: int16x4_t = core::mem::zeroed();
        let mut d7: int16x4_t = core::mem::zeroed();
        let mut d8: int16x4_t = core::mem::zeroed();
        let mut q23: int16x4_t;
        let mut q34: int16x4_t;
        let mut q56: int16x4_t;
        let mut q234: int16x4_t;
        let mut q345: int16x4_t;
        let mut q456: int16x4_t;
        let mut q567: int16x4_t;
        let mut r23: int32x4_t;
        let mut r56: int32x4_t;
        let mut r234: int32x4_t;
        let mut r345: int32x4_t;
        let mut r456: int32x4_t;
        let mut r567: int32x4_t;
        let mut r34: int32x4_t;
        let mut r78: int32x4_t;
        let mut r678: int32x4_t;
        let mut r1: int32x4_t = core::mem::zeroed();
        let mut r2: int32x4_t = core::mem::zeroed();
        let mut r3: int32x4_t = core::mem::zeroed();
        let mut r4: int32x4_t = core::mem::zeroed();
        let mut r5: int32x4_t = core::mem::zeroed();
        let mut r6: int32x4_t = core::mem::zeroed();
        let mut r7: int32x4_t = core::mem::zeroed();
        let mut r8: int32x4_t = core::mem::zeroed();
        let mut q678: int16x4_t;
        let mut q78: int16x4_t;

        let mut src2_ptr: *mut i32;
        let mut src1_ptr: *mut u16;
        count = 0;
        h = height;
        w = width;
        loop {
            dst1_ptr = dst1.offset(((count << 2) * dst_stride) as isize);
            dst2_ptr = dst2.offset(((count << 2) * dst_stride) as isize);
            src1_ptr = dst1.offset(((count << 2) * dst_stride) as isize);
            src2_ptr = dst2.offset(((count << 2) * dst_stride) as isize);
            w = width;

            load_s16_4x4(src1_ptr as *const i16, dst_stride as isize, &mut d1, &mut d2, &mut d3, &mut d4);
            transpose_elems_inplace_s16_4x4(&mut d1, &mut d2, &mut d3, &mut d4);
            load_s32_4x4(src2_ptr, dst_stride as isize, &mut r1, &mut r2, &mut r3, &mut r4);
            transpose_elems_inplace_s32_4x4(&mut r1, &mut r2, &mut r3, &mut r4);
            src1_ptr = src1_ptr.offset(4);
            src2_ptr = src2_ptr.offset(4);

            q23 = vadd_s16(d2, d3);
            q234 = vadd_s16(q23, d4);
            q34 = vadd_s16(d3, d4);
            dst1_ptr = dst1_ptr.offset(2);
            r23 = vaddq_s32(r2, r3);
            r234 = vaddq_s32(r23, r4);
            r34 = vaddq_s32(r3, r4);
            dst2_ptr = dst2_ptr.offset(2);

            loop {
                load_s16_4x4(src1_ptr as *const i16, dst_stride as isize, &mut d5, &mut d6, &mut d7, &mut d8);
                transpose_elems_inplace_s16_4x4(&mut d5, &mut d6, &mut d7, &mut d8);
                load_s32_4x4(src2_ptr, dst_stride as isize, &mut r5, &mut r6, &mut r7, &mut r8);
                transpose_elems_inplace_s32_4x4(&mut r5, &mut r6, &mut r7, &mut r8);
                src1_ptr = src1_ptr.offset(4);
                src2_ptr = src2_ptr.offset(4);

                q345 = vadd_s16(d5, q34);
                q56 = vadd_s16(d5, d6);
                q456 = vadd_s16(d4, q56);
                q567 = vadd_s16(d7, q56);
                q78 = vadd_s16(d7, d8);
                q678 = vadd_s16(d6, q78);
                transpose_elems_inplace_s16_4x4(&mut q234, &mut q345, &mut q456, &mut q567);
                store_s16_4x4(dst1_ptr as *mut i16, dst_stride as isize, q234, q345, q456, q567);
                dst1_ptr = dst1_ptr.offset(4);

                d4 = d8;
                q34 = q78;
                q234 = q678;

                r345 = vaddq_s32(r5, r34);
                r56 = vaddq_s32(r5, r6);
                r456 = vaddq_s32(r4, r56);
                r567 = vaddq_s32(r7, r56);
                r78 = vaddq_s32(r7, r8);
                r678 = vaddq_s32(r6, r78);
                transpose_elems_inplace_s32_4x4(&mut r234, &mut r345, &mut r456, &mut r567);
                store_s32_4x4(dst2_ptr, dst_stride as isize, r234, r345, r456, r567);
                dst2_ptr = dst2_ptr.offset(4);

                r4 = r8;
                r34 = r78;
                r234 = r678;
                w -= 4;
                if !(w > 0) {
                    break;
                }
            }
            h -= 4;
            count += 1;
            if !(h > 0) {
                break;
            }
        }
    }
}

#[inline(always)]
unsafe fn cross_sum_inp_s32(buf: *mut i32, buf_stride: i32) -> int32x4_t {
    let bs: isize = buf_stride as isize;
    let xtl: int32x4_t = vld1q_s32(buf.offset(-bs - 1));
    let xt: int32x4_t = vld1q_s32(buf.offset(-bs));
    let xtr: int32x4_t = vld1q_s32(buf.offset(-bs + 1));
    let xl: int32x4_t = vld1q_s32(buf.offset(-1));
    let x: int32x4_t = vld1q_s32(buf);
    let xr: int32x4_t = vld1q_s32(buf.offset(1));
    let xbl: int32x4_t = vld1q_s32(buf.offset(bs - 1));
    let xb: int32x4_t = vld1q_s32(buf.offset(bs));
    let xbr: int32x4_t = vld1q_s32(buf.offset(bs + 1));

    let fours: int32x4_t = vaddq_s32(xl, vaddq_s32(xt, vaddq_s32(xr, vaddq_s32(xb, x))));
    let threes: int32x4_t = vaddq_s32(xtl, vaddq_s32(xtr, vaddq_s32(xbr, xbl)));
    let res: int32x4_t = vsubq_s32(vshlq_n_s32::<2>(vaddq_s32(fours, threes)), threes);
    res
}

#[inline(always)]
unsafe fn cross_sum_inp_u16(buf: *mut u16, buf_stride: i32, a0: *mut int32x4_t, a1: *mut int32x4_t) {
    let bs: isize = buf_stride as isize;
    let mut xtl: uint16x8_t = vld1q_u16(buf.offset(-bs - 1));
    let mut xt: uint16x8_t = vld1q_u16(buf.offset(-bs));
    let xtr: uint16x8_t = vld1q_u16(buf.offset(-bs + 1));
    let mut xl: uint16x8_t = vld1q_u16(buf.offset(-1));
    let x: uint16x8_t = vld1q_u16(buf);
    let xr: uint16x8_t = vld1q_u16(buf.offset(1));
    let mut xbl: uint16x8_t = vld1q_u16(buf.offset(bs - 1));
    let mut xb: uint16x8_t = vld1q_u16(buf.offset(bs));
    let xbr: uint16x8_t = vld1q_u16(buf.offset(bs + 1));

    xb = vaddq_u16(xb, x);
    xt = vaddq_u16(xt, xr);
    xl = vaddq_u16(xl, xb);
    xl = vaddq_u16(xl, xt);

    let r0: uint16x8_t = vshlq_n_u16::<2>(xl);

    xbl = vaddq_u16(xbl, xbr);
    xtl = vaddq_u16(xtl, xtr);
    xtl = vaddq_u16(xtl, xbl);

    let mut r1: uint16x8_t = vshlq_n_u16::<2>(xtl);
    r1 = vsubq_u16(r1, xtl);

    *a0 = vreinterpretq_s32_u32(vaddq_u32(vmovl_u16(vget_low_u16(r0)), vmovl_u16(vget_low_u16(r1))));
    *a1 =
        vreinterpretq_s32_u32(vaddq_u32(vmovl_u16(vget_high_u16(r0)), vmovl_u16(vget_high_u16(r1))));
}

#[inline(always)]
unsafe fn cross_sum_fast_even_row(buf: *mut i32, buf_stride: i32) -> int32x4_t {
    let bs: isize = buf_stride as isize;
    let xtl: int32x4_t = vld1q_s32(buf.offset(-bs - 1));
    let xt: int32x4_t = vld1q_s32(buf.offset(-bs));
    let xtr: int32x4_t = vld1q_s32(buf.offset(-bs + 1));
    let xbl: int32x4_t = vld1q_s32(buf.offset(bs - 1));
    let xb: int32x4_t = vld1q_s32(buf.offset(bs));
    let xbr: int32x4_t = vld1q_s32(buf.offset(bs + 1));

    let fives: int32x4_t = vaddq_s32(xtl, vaddq_s32(xtr, vaddq_s32(xbr, xbl)));
    let sixes: int32x4_t = vaddq_s32(xt, xb);
    let fives_plus_sixes: int32x4_t = vaddq_s32(fives, sixes);

    vaddq_s32(vaddq_s32(vshlq_n_s32::<2>(fives_plus_sixes), fives_plus_sixes), sixes)
}

#[inline(always)]
unsafe fn cross_sum_fast_even_row_inp16(
    buf: *mut u16,
    buf_stride: i32,
    a0: *mut int32x4_t,
    a1: *mut int32x4_t,
) {
    let bs: isize = buf_stride as isize;
    let mut xtl: uint16x8_t = vld1q_u16(buf.offset(-bs - 1));
    let xt: uint16x8_t = vld1q_u16(buf.offset(-bs));
    let mut xtr: uint16x8_t = vld1q_u16(buf.offset(-bs + 1));
    let xbl: uint16x8_t = vld1q_u16(buf.offset(bs - 1));
    let mut xb: uint16x8_t = vld1q_u16(buf.offset(bs));
    let mut xbr: uint16x8_t = vld1q_u16(buf.offset(bs + 1));

    xbr = vaddq_u16(xbr, xbl);
    xtr = vaddq_u16(xtr, xtl);
    xbr = vaddq_u16(xbr, xtr);
    xtl = vshlq_n_u16::<2>(xbr);
    xbr = vaddq_u16(xtl, xbr);

    xb = vaddq_u16(xb, xt);
    let xb0: uint16x8_t = vshlq_n_u16::<1>(xb);
    xb = vshlq_n_u16::<2>(xb);
    xb = vaddq_u16(xb, xb0);

    *a0 = vreinterpretq_s32_u32(vaddq_u32(vmovl_u16(vget_low_u16(xbr)), vmovl_u16(vget_low_u16(xb))));
    *a1 =
        vreinterpretq_s32_u32(vaddq_u32(vmovl_u16(vget_high_u16(xbr)), vmovl_u16(vget_high_u16(xb))));
}

#[inline(always)]
unsafe fn cross_sum_fast_odd_row(buf: *mut i32) -> int32x4_t {
    let xl: int32x4_t = vld1q_s32(buf.offset(-1));
    let x: int32x4_t = vld1q_s32(buf);
    let xr: int32x4_t = vld1q_s32(buf.offset(1));
    let fives: int32x4_t = vaddq_s32(xl, xr);
    let sixes: int32x4_t = x;
    let fives_plus_sixes: int32x4_t = vaddq_s32(fives, sixes);

    vaddq_s32(vaddq_s32(vshlq_n_s32::<2>(fives_plus_sixes), fives_plus_sixes), sixes)
}

#[inline(always)]
unsafe fn cross_sum_fast_odd_row_inp16(buf: *mut u16, a0: *mut int32x4_t, a1: *mut int32x4_t) {
    let mut xl: uint16x8_t = vld1q_u16(buf.offset(-1));
    let mut x: uint16x8_t = vld1q_u16(buf);
    let xr: uint16x8_t = vld1q_u16(buf.offset(1));
    xl = vaddq_u16(xl, xr);
    let mut x0: uint16x8_t = vshlq_n_u16::<2>(xl);
    xl = vaddq_u16(xl, x0);

    x0 = vshlq_n_u16::<1>(x);
    x = vshlq_n_u16::<2>(x);
    x = vaddq_u16(x, x0);

    *a0 = vreinterpretq_s32_u32(vaddq_u32(vmovl_u16(vget_low_u16(xl)), vmovl_u16(vget_low_u16(x))));
    *a1 = vreinterpretq_s32_u32(vaddq_u32(vmovl_u16(vget_high_u16(xl)), vmovl_u16(vget_high_u16(x))));
}

unsafe fn final_filter_fast_internal(
    A: *mut u16,
    B: *mut i32,
    buf_stride: i32,
    src: *mut i16,
    src_stride: i32,
    dst: *mut i32,
    dst_stride: i32,
    width: i32,
    height: i32,
) {
    let mut s0: int16x8_t;
    let mut B_tmp: *mut i32;
    let mut dst_ptr: *mut i32;
    let mut A_tmp: *mut u16;
    let mut src_ptr: *mut i16;
    let mut a_res0: int32x4_t = core::mem::zeroed();
    let mut a_res1: int32x4_t = core::mem::zeroed();
    let mut b_res0: int32x4_t;
    let mut b_res1: int32x4_t;
    let mut w: i32;
    let mut h: i32;
    let mut count: i32 = 0;
    debug_assert!(SGRPROJ_SGR_BITS == 8);
    debug_assert!(SGRPROJ_RST_BITS == 4);

    A_tmp = A;
    B_tmp = B;
    src_ptr = src;
    dst_ptr = dst;
    h = height;
    loop {
        A_tmp = A.offset((count * buf_stride) as isize);
        B_tmp = B.offset((count * buf_stride) as isize);
        src_ptr = src.offset((count * src_stride) as isize);
        dst_ptr = dst.offset((count * dst_stride) as isize);
        w = width;
        if count & 1 == 0 {
            loop {
                s0 = vld1q_s16(src_ptr);
                cross_sum_fast_even_row_inp16(A_tmp, buf_stride, &mut a_res0, &mut a_res1);
                a_res0 = vmulq_s32(vmovl_s16(vget_low_s16(s0)), a_res0);
                a_res1 = vmulq_s32(vmovl_s16(vget_high_s16(s0)), a_res1);

                b_res0 = cross_sum_fast_even_row(B_tmp, buf_stride);
                b_res1 = cross_sum_fast_even_row(B_tmp.offset(4), buf_stride);
                a_res0 = vaddq_s32(a_res0, b_res0);
                a_res1 = vaddq_s32(a_res1, b_res1);

                a_res0 = vrshrq_n_s32::<{ SGRPROJ_SGR_BITS + NB_EVEN - SGRPROJ_RST_BITS }>(a_res0);
                a_res1 = vrshrq_n_s32::<{ SGRPROJ_SGR_BITS + NB_EVEN - SGRPROJ_RST_BITS }>(a_res1);

                vst1q_s32(dst_ptr, a_res0);
                vst1q_s32(dst_ptr.offset(4), a_res1);

                A_tmp = A_tmp.offset(8);
                B_tmp = B_tmp.offset(8);
                src_ptr = src_ptr.offset(8);
                dst_ptr = dst_ptr.offset(8);
                w -= 8;
                if !(w > 0) {
                    break;
                }
            }
        } else {
            loop {
                s0 = vld1q_s16(src_ptr);
                cross_sum_fast_odd_row_inp16(A_tmp, &mut a_res0, &mut a_res1);
                a_res0 = vmulq_s32(vmovl_s16(vget_low_s16(s0)), a_res0);
                a_res1 = vmulq_s32(vmovl_s16(vget_high_s16(s0)), a_res1);

                b_res0 = cross_sum_fast_odd_row(B_tmp);
                b_res1 = cross_sum_fast_odd_row(B_tmp.offset(4));
                a_res0 = vaddq_s32(a_res0, b_res0);
                a_res1 = vaddq_s32(a_res1, b_res1);

                a_res0 = vrshrq_n_s32::<{ SGRPROJ_SGR_BITS + NB_ODD - SGRPROJ_RST_BITS }>(a_res0);
                a_res1 = vrshrq_n_s32::<{ SGRPROJ_SGR_BITS + NB_ODD - SGRPROJ_RST_BITS }>(a_res1);

                vst1q_s32(dst_ptr, a_res0);
                vst1q_s32(dst_ptr.offset(4), a_res1);

                A_tmp = A_tmp.offset(8);
                B_tmp = B_tmp.offset(8);
                src_ptr = src_ptr.offset(8);
                dst_ptr = dst_ptr.offset(8);
                w -= 8;
                if !(w > 0) {
                    break;
                }
            }
        }
        count += 1;
        h -= 1;
        if !(h > 0) {
            break;
        }
    }
}

unsafe fn final_filter_internal(
    A: *mut u16,
    B: *mut i32,
    buf_stride: i32,
    src: *mut i16,
    src_stride: i32,
    dst: *mut i32,
    dst_stride: i32,
    width: i32,
    height: i32,
) {
    let mut s0: int16x8_t;
    let mut B_tmp: *mut i32;
    let mut dst_ptr: *mut i32;
    let mut A_tmp: *mut u16;
    let mut src_ptr: *mut i16;
    let mut a_res0: int32x4_t = core::mem::zeroed();
    let mut a_res1: int32x4_t = core::mem::zeroed();
    let mut b_res0: int32x4_t;
    let mut b_res1: int32x4_t;
    let mut w: i32;
    let mut h: i32;
    let mut count: i32 = 0;

    debug_assert!(SGRPROJ_SGR_BITS == 8);
    debug_assert!(SGRPROJ_RST_BITS == 4);
    h = height;

    loop {
        A_tmp = A.offset((count * buf_stride) as isize);
        B_tmp = B.offset((count * buf_stride) as isize);
        src_ptr = src.offset((count * src_stride) as isize);
        dst_ptr = dst.offset((count * dst_stride) as isize);
        w = width;
        loop {
            s0 = vld1q_s16(src_ptr);
            cross_sum_inp_u16(A_tmp, buf_stride, &mut a_res0, &mut a_res1);
            a_res0 = vmulq_s32(vmovl_s16(vget_low_s16(s0)), a_res0);
            a_res1 = vmulq_s32(vmovl_s16(vget_high_s16(s0)), a_res1);

            b_res0 = cross_sum_inp_s32(B_tmp, buf_stride);
            b_res1 = cross_sum_inp_s32(B_tmp.offset(4), buf_stride);
            a_res0 = vaddq_s32(a_res0, b_res0);
            a_res1 = vaddq_s32(a_res1, b_res1);

            a_res0 = vrshrq_n_s32::<{ SGRPROJ_SGR_BITS + NB_EVEN - SGRPROJ_RST_BITS }>(a_res0);
            a_res1 = vrshrq_n_s32::<{ SGRPROJ_SGR_BITS + NB_EVEN - SGRPROJ_RST_BITS }>(a_res1);
            vst1q_s32(dst_ptr, a_res0);
            vst1q_s32(dst_ptr.offset(4), a_res1);

            A_tmp = A_tmp.offset(8);
            B_tmp = B_tmp.offset(8);
            src_ptr = src_ptr.offset(8);
            dst_ptr = dst_ptr.offset(8);
            w -= 8;
            if !(w > 0) {
                break;
            }
        }
        count += 1;
        h -= 1;
        if !(h > 0) {
            break;
        }
    }
}

#[inline(always)]
unsafe fn restoration_fast_internal(
    dgd16: *mut u16,
    width: i32,
    height: i32,
    dgd_stride: i32,
    dst: *mut i32,
    dst_stride: i32,
    bit_depth: i32,
    sgr_params_idx: i32,
    radius_idx: i32,
) -> i32 {
    let params: *const sgr_params_type =
        (&raw const av1_sgr_params).cast::<sgr_params_type>().offset(sgr_params_idx as isize);
    let r: i32 = (*params).r[radius_idx as usize];
    let width_ext: i32 = width + 2 * SGRPROJ_BORDER_HORZ;
    let height_ext: i32 = height + 2 * SGRPROJ_BORDER_VERT;
    let buf_stride: i32 = ((width_ext + 3) & !3) + 16;

    let buf_size: usize = 3 * core::mem::size_of::<i32>() * RESTORATION_PROC_UNIT_PELS as usize;
    let buf: *mut i32 = aom_memalign(8, buf_size as _) as *mut i32;
    if buf.is_null() {
        return -1;
    }

    let mut square_sum_buf: *mut i32 = buf;
    let mut sum_buf: *mut i32 = square_sum_buf.offset(RESTORATION_PROC_UNIT_PELS as isize);
    let mut tmp16_buf: *mut u16 = sum_buf.offset(RESTORATION_PROC_UNIT_PELS as isize) as *mut u16;
    debug_assert!(
        sum_buf.offset(RESTORATION_PROC_UNIT_PELS as isize) as *mut u8
            <= (buf as *mut u8).add(buf_size),
        "Allocated buffer is too small. Resize the buffer."
    );

    debug_assert!(r <= MAX_RADIUS, "Need MAX_RADIUS >= r");
    debug_assert!(
        r <= SGRPROJ_BORDER_VERT - 1 && r <= SGRPROJ_BORDER_HORZ - 1,
        "Need SGRPROJ_BORDER_* >= r+1"
    );

    debug_assert!(radius_idx == 0);
    debug_assert!(r == 2);

    // input(dgd16) is 16bit.
    // sum of pixels 1st stage output will be in 16bit(tmp16_buf). End output is
    // kept in 32bit [sum_buf]. sum of squares output is kept in 32bit
    // buffer(square_sum_buf).
    boxsum2(
        dgd16.offset((-dgd_stride * SGRPROJ_BORDER_VERT - SGRPROJ_BORDER_HORZ) as isize) as *mut i16,
        dgd_stride,
        tmp16_buf as *mut i16,
        sum_buf,
        square_sum_buf,
        buf_stride,
        width_ext,
        height_ext,
    );

    square_sum_buf = square_sum_buf.offset((SGRPROJ_BORDER_VERT * buf_stride + SGRPROJ_BORDER_HORZ) as isize);
    sum_buf = sum_buf.offset((SGRPROJ_BORDER_VERT * buf_stride + SGRPROJ_BORDER_HORZ) as isize);
    tmp16_buf = tmp16_buf.offset((SGRPROJ_BORDER_VERT * buf_stride + SGRPROJ_BORDER_HORZ) as isize);

    // Calculation of a, b. a output is in 16bit tmp_buf which is in range of
    // [1, 256] for all bit depths. b output is kept in 32bit buffer.

    if bit_depth > 8 {
        calc_ab_fast_internal_hbd(
            square_sum_buf.offset((-buf_stride - 1) as isize),
            tmp16_buf.offset((-buf_stride - 1) as isize),
            sum_buf.offset((-buf_stride - 1) as isize),
            buf_stride * 2,
            width + 2,
            height + 2,
            bit_depth,
            r,
            (*params).s[radius_idx as usize],
            2,
        );
    } else {
        calc_ab_fast_internal_lbd(
            square_sum_buf.offset((-buf_stride - 1) as isize),
            tmp16_buf.offset((-buf_stride - 1) as isize),
            sum_buf.offset((-buf_stride - 1) as isize),
            buf_stride * 2,
            width + 2,
            height + 2,
            r,
            (*params).s[radius_idx as usize],
            2,
        );
    }
    final_filter_fast_internal(
        tmp16_buf,
        sum_buf,
        buf_stride,
        dgd16 as *mut i16,
        dgd_stride,
        dst,
        dst_stride,
        width,
        height,
    );
    aom_free(buf as *mut c_void);
    0
}

#[inline(always)]
unsafe fn restoration_internal(
    dgd16: *mut u16,
    width: i32,
    height: i32,
    dgd_stride: i32,
    dst: *mut i32,
    dst_stride: i32,
    bit_depth: i32,
    sgr_params_idx: i32,
    radius_idx: i32,
) -> i32 {
    let params: *const sgr_params_type =
        (&raw const av1_sgr_params).cast::<sgr_params_type>().offset(sgr_params_idx as isize);
    let r: i32 = (*params).r[radius_idx as usize];
    let width_ext: i32 = width + 2 * SGRPROJ_BORDER_HORZ;
    let height_ext: i32 = height + 2 * SGRPROJ_BORDER_VERT;
    let buf_stride: i32 = ((width_ext + 3) & !3) + 16;

    let buf_size: usize = 3 * core::mem::size_of::<i32>() * RESTORATION_PROC_UNIT_PELS as usize;
    let buf: *mut i32 = aom_memalign(8, buf_size as _) as *mut i32;
    if buf.is_null() {
        return -1;
    }

    let mut square_sum_buf: *mut i32 = buf;
    let mut B: *mut i32 = square_sum_buf.offset(RESTORATION_PROC_UNIT_PELS as isize);
    let mut A16: *mut u16 = B.offset(RESTORATION_PROC_UNIT_PELS as isize) as *mut u16;
    let mut sum_buf: *mut u16 = A16.offset(RESTORATION_PROC_UNIT_PELS as isize);

    debug_assert!(
        sum_buf.offset(RESTORATION_PROC_UNIT_PELS as isize) as *mut u8
            <= (buf as *mut u8).add(buf_size),
        "Allocated buffer is too small. Resize the buffer."
    );

    debug_assert!(r <= MAX_RADIUS, "Need MAX_RADIUS >= r");
    debug_assert!(
        r <= SGRPROJ_BORDER_VERT - 1 && r <= SGRPROJ_BORDER_HORZ - 1,
        "Need SGRPROJ_BORDER_* >= r+1"
    );

    debug_assert!(radius_idx == 1);
    debug_assert!(r == 1);

    // input(dgd16) is 16bit.
    // sum of pixels output will be in 16bit(sum_buf).
    // sum of squares output is kept in 32bit buffer(square_sum_buf).
    boxsum1(
        dgd16.offset((-dgd_stride * SGRPROJ_BORDER_VERT - SGRPROJ_BORDER_HORZ) as isize) as *mut i16,
        dgd_stride,
        sum_buf,
        square_sum_buf,
        buf_stride,
        width_ext,
        height_ext,
    );

    square_sum_buf = square_sum_buf.offset((SGRPROJ_BORDER_VERT * buf_stride + SGRPROJ_BORDER_HORZ) as isize);
    B = B.offset((SGRPROJ_BORDER_VERT * buf_stride + SGRPROJ_BORDER_HORZ) as isize);
    A16 = A16.offset((SGRPROJ_BORDER_VERT * buf_stride + SGRPROJ_BORDER_HORZ) as isize);
    sum_buf = sum_buf.offset((SGRPROJ_BORDER_VERT * buf_stride + SGRPROJ_BORDER_HORZ) as isize);

    // Calculation of a, b. a output is in 16bit tmp_buf which is in range of
    // [1, 256] for all bit depths. b output is kept in 32bit buffer.
    if bit_depth > 8 {
        calc_ab_internal_hbd(
            square_sum_buf.offset((-buf_stride - 1) as isize),
            A16.offset((-buf_stride - 1) as isize),
            sum_buf.offset((-buf_stride - 1) as isize),
            B.offset((-buf_stride - 1) as isize),
            buf_stride,
            width + 2,
            height + 2,
            bit_depth,
            r,
            (*params).s[radius_idx as usize],
            1,
        );
    } else {
        calc_ab_internal_lbd(
            square_sum_buf.offset((-buf_stride - 1) as isize),
            A16.offset((-buf_stride - 1) as isize),
            sum_buf.offset((-buf_stride - 1) as isize),
            B.offset((-buf_stride - 1) as isize),
            buf_stride,
            width + 2,
            height + 2,
            r,
            (*params).s[radius_idx as usize],
            1,
        );
    }
    final_filter_internal(A16, B, buf_stride, dgd16 as *mut i16, dgd_stride, dst, dst_stride, width, height);
    aom_free(buf as *mut c_void);
    0
}

#[inline(always)]
unsafe fn src_convert_u8_to_u16(
    src: *const u8,
    src_stride: i32,
    dst: *mut u16,
    dst_stride: i32,
    width: i32,
    height: i32,
) {
    let mut src_ptr: *const u8;
    let mut dst_ptr: *mut u16;
    let mut h: i32;
    let mut w: i32;
    let mut count: i32 = 0;

    let mut t1: uint8x8_t = core::mem::zeroed();
    let mut t2: uint8x8_t = core::mem::zeroed();
    let mut t3: uint8x8_t = core::mem::zeroed();
    let mut t4: uint8x8_t = core::mem::zeroed();
    let mut s1: uint16x8_t;
    let mut s2: uint16x8_t;
    let mut s3: uint16x8_t;
    let mut s4: uint16x8_t;
    h = height;
    loop {
        src_ptr = src.offset(((count << 2) * src_stride) as isize);
        dst_ptr = dst.offset(((count << 2) * dst_stride) as isize);
        w = width;
        if w >= 7 {
            loop {
                load_u8_8x4(src_ptr, src_stride as isize, &mut t1, &mut t2, &mut t3, &mut t4);
                s1 = vmovl_u8(t1);
                s2 = vmovl_u8(t2);
                s3 = vmovl_u8(t3);
                s4 = vmovl_u8(t4);
                store_u16_8x4(dst_ptr, dst_stride as isize, s1, s2, s3, s4);

                src_ptr = src_ptr.offset(8);
                dst_ptr = dst_ptr.offset(8);
                w -= 8;
                if !(w > 7) {
                    break;
                }
            }
        }

        for y in 0..w {
            *dst_ptr.offset(y as isize) = *src_ptr.offset(y as isize) as u16;
            *dst_ptr.offset((y + 1 * dst_stride) as isize) =
                *src_ptr.offset((y + 1 * src_stride) as isize) as u16;
            *dst_ptr.offset((y + 2 * dst_stride) as isize) =
                *src_ptr.offset((y + 2 * src_stride) as isize) as u16;
            *dst_ptr.offset((y + 3 * dst_stride) as isize) =
                *src_ptr.offset((y + 3 * src_stride) as isize) as u16;
        }
        count += 1;
        h -= 4;
        if !(h > 3) {
            break;
        }
    }

    src_ptr = src.offset(((count << 2) * src_stride) as isize);
    dst_ptr = dst.offset(((count << 2) * dst_stride) as isize);
    for x in 0..h {
        for y in 0..width {
            *dst_ptr.offset((y + x * dst_stride) as isize) =
                *src_ptr.offset((y + x * src_stride) as isize) as u16;
        }
    }

    // memset uninitialized rows of src buffer as they are needed for the
    // boxsum filter calculation.
    for x in height..(height + 5) {
        ptr::write_bytes(dst.offset((x * dst_stride) as isize), 0, (width + 2) as usize);
    }
}

#[inline(always)]
unsafe fn src_convert_hbd_copy(
    src: *const u16,
    src_stride: i32,
    dst: *mut u16,
    dst_stride: i32,
    width: i32,
    height: i32,
) {
    let mut src_ptr: *const u16;
    let mut dst_ptr: *mut u16;
    let mut h: i32;
    let mut w: i32;
    let mut count: i32 = 0;
    let mut s1: uint16x8_t = core::mem::zeroed();
    let mut s2: uint16x8_t = core::mem::zeroed();
    let mut s3: uint16x8_t = core::mem::zeroed();
    let mut s4: uint16x8_t = core::mem::zeroed();

    h = height;
    loop {
        src_ptr = src.offset(((count << 2) * src_stride) as isize);
        dst_ptr = dst.offset(((count << 2) * dst_stride) as isize);
        w = width;
        loop {
            load_u16_8x4(src_ptr, src_stride as isize, &mut s1, &mut s2, &mut s3, &mut s4);
            store_u16_8x4(dst_ptr, dst_stride as isize, s1, s2, s3, s4);
            src_ptr = src_ptr.offset(8);
            dst_ptr = dst_ptr.offset(8);
            w -= 8;
            if !(w > 7) {
                break;
            }
        }

        for y in 0..w {
            *dst_ptr.offset(y as isize) = *src_ptr.offset(y as isize);
            *dst_ptr.offset((y + 1 * dst_stride) as isize) =
                *src_ptr.offset((y + 1 * src_stride) as isize);
            *dst_ptr.offset((y + 2 * dst_stride) as isize) =
                *src_ptr.offset((y + 2 * src_stride) as isize);
            *dst_ptr.offset((y + 3 * dst_stride) as isize) =
                *src_ptr.offset((y + 3 * src_stride) as isize);
        }
        count += 1;
        h -= 4;
        if !(h > 3) {
            break;
        }
    }

    src_ptr = src.offset(((count << 2) * src_stride) as isize);
    dst_ptr = dst.offset(((count << 2) * dst_stride) as isize);

    for x in 0..h {
        ptr::copy_nonoverlapping(
            src_ptr.offset((x * src_stride) as isize),
            dst_ptr.offset((x * dst_stride) as isize),
            width as usize,
        );
    }
    // memset uninitialized rows of src buffer as they are needed for the
    // boxsum filter calculation.
    for x in height..(height + 5) {
        ptr::write_bytes(dst.offset((x * dst_stride) as isize), 0, (width + 2) as usize);
    }
}

#[no_mangle]
pub unsafe extern "C-unwind" fn av1_selfguided_restoration_neon(
    dat8: *const u8,
    width: i32,
    height: i32,
    stride: i32,
    flt0: *mut i32,
    flt1: *mut i32,
    flt_stride: i32,
    sgr_params_idx: i32,
    bit_depth: i32,
    highbd: i32,
) -> i32 {
    let params: *const sgr_params_type =
        (&raw const av1_sgr_params).cast::<sgr_params_type>().offset(sgr_params_idx as isize);
    debug_assert!(!((*params).r[0] == 0 && (*params).r[1] == 0));

    let mut dgd16_: [u16; RESTORATION_PROC_UNIT_PELS as usize] =
        [0; RESTORATION_PROC_UNIT_PELS as usize];
    let dgd16_stride: i32 = width + 2 * SGRPROJ_BORDER_HORZ;
    let dgd16: *mut u16 = dgd16_
        .as_mut_ptr()
        .offset((dgd16_stride * SGRPROJ_BORDER_VERT + SGRPROJ_BORDER_HORZ) as isize);
    let width_ext: i32 = width + 2 * SGRPROJ_BORDER_HORZ;
    let height_ext: i32 = height + 2 * SGRPROJ_BORDER_VERT;
    let dgd_stride: i32 = stride;

    if highbd != 0 {
        let dgd16_tmp: *const u16 = CONVERT_TO_SHORTPTR(dat8);
        src_convert_hbd_copy(
            dgd16_tmp.offset((-SGRPROJ_BORDER_VERT * dgd_stride - SGRPROJ_BORDER_HORZ) as isize),
            dgd_stride,
            dgd16.offset((-SGRPROJ_BORDER_VERT * dgd16_stride - SGRPROJ_BORDER_HORZ) as isize),
            dgd16_stride,
            width_ext,
            height_ext,
        );
    } else {
        src_convert_u8_to_u16(
            dat8.offset((-SGRPROJ_BORDER_VERT * dgd_stride - SGRPROJ_BORDER_HORZ) as isize),
            dgd_stride,
            dgd16.offset((-SGRPROJ_BORDER_VERT * dgd16_stride - SGRPROJ_BORDER_HORZ) as isize),
            dgd16_stride,
            width_ext,
            height_ext,
        );
    }

    if (*params).r[0] > 0 {
        let ret: i32 = restoration_fast_internal(
            dgd16,
            width,
            height,
            dgd16_stride,
            flt0,
            flt_stride,
            bit_depth,
            sgr_params_idx,
            0,
        );
        if ret != 0 {
            return ret;
        }
    }
    if (*params).r[1] > 0 {
        let ret: i32 = restoration_internal(
            dgd16,
            width,
            height,
            dgd16_stride,
            flt1,
            flt_stride,
            bit_depth,
            sgr_params_idx,
            1,
        );
        if ret != 0 {
            return ret;
        }
    }
    0
}

#[no_mangle]
pub unsafe extern "C-unwind" fn av1_apply_selfguided_restoration_neon(
    dat8: *const u8,
    width: i32,
    height: i32,
    stride: i32,
    eps: i32,
    xqd: *const i32,
    dst8: *mut u8,
    dst_stride: i32,
    tmpbuf: *mut i32,
    bit_depth: i32,
    highbd: i32,
) -> i32 {
    let mut flt0: *mut i32 = tmpbuf;
    let mut flt1: *mut i32 = flt0.offset(RESTORATION_UNITPELS_MAX as isize);
    debug_assert!(width * height <= RESTORATION_UNITPELS_MAX);
    let mut dgd16_: [u16; RESTORATION_PROC_UNIT_PELS as usize] =
        [0; RESTORATION_PROC_UNIT_PELS as usize];
    let dgd16_stride: i32 = width + 2 * SGRPROJ_BORDER_HORZ;
    let dgd16: *mut u16 = dgd16_
        .as_mut_ptr()
        .offset((dgd16_stride * SGRPROJ_BORDER_VERT + SGRPROJ_BORDER_HORZ) as isize);
    let width_ext: i32 = width + 2 * SGRPROJ_BORDER_HORZ;
    let height_ext: i32 = height + 2 * SGRPROJ_BORDER_VERT;
    let dgd_stride: i32 = stride;
    let params: *const sgr_params_type =
        (&raw const av1_sgr_params).cast::<sgr_params_type>().offset(eps as isize);
    let mut xq: [i32; 2] = [0; 2];

    debug_assert!(!((*params).r[0] == 0 && (*params).r[1] == 0));

    if highbd != 0 {
        let dgd16_tmp: *const u16 = CONVERT_TO_SHORTPTR(dat8);
        src_convert_hbd_copy(
            dgd16_tmp.offset((-SGRPROJ_BORDER_VERT * dgd_stride - SGRPROJ_BORDER_HORZ) as isize),
            dgd_stride,
            dgd16.offset((-SGRPROJ_BORDER_VERT * dgd16_stride - SGRPROJ_BORDER_HORZ) as isize),
            dgd16_stride,
            width_ext,
            height_ext,
        );
    } else {
        src_convert_u8_to_u16(
            dat8.offset((-SGRPROJ_BORDER_VERT * dgd_stride - SGRPROJ_BORDER_HORZ) as isize),
            dgd_stride,
            dgd16.offset((-SGRPROJ_BORDER_VERT * dgd16_stride - SGRPROJ_BORDER_HORZ) as isize),
            dgd16_stride,
            width_ext,
            height_ext,
        );
    }
    if (*params).r[0] > 0 {
        let ret: i32 = restoration_fast_internal(
            dgd16,
            width,
            height,
            dgd16_stride,
            flt0,
            width,
            bit_depth,
            eps,
            0,
        );
        if ret != 0 {
            return ret;
        }
    }
    if (*params).r[1] > 0 {
        let ret: i32 =
            restoration_internal(dgd16, width, height, dgd16_stride, flt1, width, bit_depth, eps, 1);
        if ret != 0 {
            return ret;
        }
    }

    av1_decode_xq(xqd, xq.as_mut_ptr(), params as *const _);

    {
        let mut src_ptr: *mut i16;
        let mut dst_ptr: *mut u8;
        let dst16: *mut u16 = CONVERT_TO_SHORTPTR(dst8);
        let mut dst16_ptr: *mut u16;
        let mut d0: int16x4_t;
        let mut d4: int16x4_t;
        let mut r0: int16x8_t;
        let mut s0: int16x8_t;
        let mut r4: uint16x8_t;
        let mut u0: int32x4_t;
        let mut u4: int32x4_t;
        let mut v0: int32x4_t;
        let mut v4: int32x4_t;
        let mut f00: int32x4_t;
        let mut f10: int32x4_t;
        let mut t0: uint8x8_t;
        let mut count: i32 = 0;
        let mut w: i32 = width;
        let mut h: i32 = height;
        let mut rc: i32 = 0;

        let xq0_vec: int32x4_t = vdupq_n_s32(xq[0]);
        let xq1_vec: int32x4_t = vdupq_n_s32(xq[1]);
        let zero: int16x8_t = vdupq_n_s16(0);
        let max: uint16x8_t = vdupq_n_u16(((1 << bit_depth) - 1) as u16);
        src_ptr = dgd16 as *mut i16;
        loop {
            w = width;
            count = 0;
            dst_ptr = dst8.wrapping_offset((rc * dst_stride) as isize);
            dst16_ptr = dst16.wrapping_offset((rc * dst_stride) as isize);
            loop {
                s0 = vld1q_s16(src_ptr.offset(count as isize));

                u0 = vshll_n_s16::<SGRPROJ_RST_BITS>(vget_low_s16(s0));
                u4 = vshll_n_s16::<SGRPROJ_RST_BITS>(vget_high_s16(s0));

                v0 = vshlq_n_s32::<SGRPROJ_PRJ_BITS>(u0);
                v4 = vshlq_n_s32::<SGRPROJ_PRJ_BITS>(u4);

                if (*params).r[0] > 0 {
                    f00 = vld1q_s32(flt0.offset(count as isize));
                    f10 = vld1q_s32(flt0.offset((count + 4) as isize));

                    f00 = vsubq_s32(f00, u0);
                    f10 = vsubq_s32(f10, u4);

                    v0 = vmlaq_s32(v0, xq0_vec, f00);
                    v4 = vmlaq_s32(v4, xq0_vec, f10);
                }

                if (*params).r[1] > 0 {
                    f00 = vld1q_s32(flt1.offset(count as isize));
                    f10 = vld1q_s32(flt1.offset((count + 4) as isize));

                    f00 = vsubq_s32(f00, u0);
                    f10 = vsubq_s32(f10, u4);

                    v0 = vmlaq_s32(v0, xq1_vec, f00);
                    v4 = vmlaq_s32(v4, xq1_vec, f10);
                }

                d0 = vqrshrn_n_s32::<{ SGRPROJ_PRJ_BITS + SGRPROJ_RST_BITS }>(v0);
                d4 = vqrshrn_n_s32::<{ SGRPROJ_PRJ_BITS + SGRPROJ_RST_BITS }>(v4);

                r0 = vcombine_s16(d0, d4);

                r4 = vreinterpretq_u16_s16(vmaxq_s16(r0, zero));

                if highbd != 0 {
                    r4 = vminq_u16(r4, max);
                    vst1q_u16(dst16_ptr, r4);
                    dst16_ptr = dst16_ptr.wrapping_offset(8);
                } else {
                    t0 = vqmovn_u16(r4);
                    vst1_u8(dst_ptr, t0);
                    dst_ptr = dst_ptr.wrapping_offset(8);
                }
                w -= 8;
                count += 8;
                if !(w > 0) {
                    break;
                }
            }

            src_ptr = src_ptr.offset(dgd16_stride as isize);
            flt1 = flt1.offset(width as isize);
            flt0 = flt0.offset(width as isize);
            rc += 1;
            h -= 1;
            if !(h > 0) {
                break;
            }
        }
    }
    0
}
