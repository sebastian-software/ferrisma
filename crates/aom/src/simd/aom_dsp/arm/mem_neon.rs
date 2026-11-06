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

// Translated by hand from upstream/aom/aom_dsp/arm/mem_neon.h (ADR 0056).
// Function names, order, and intrinsic sequences follow the original; only the
// AOM_ARCH_AARCH64 branches are retained.
//
// Translation notes:
// - The vld1*_x{2,3,4} / vst1*_x{2,4} fallbacks for deficient 32-bit Clang and
//   old GCC are omitted: core::arch::aarch64 provides those intrinsics.
// - Output parameters stay raw pointers (`&mut v` coerces at call sites).
// - Pointer increments after the last access of a sequence (dead in the C
//   original) are dropped, so no pointer is advanced past its allocation.
// - memcpy-based unaligned scalar accesses use ptr::read_unaligned /
//   ptr::write_unaligned. core::arch's vld1{q}_lane_u32 dereferences its
//   pointer as an aligned `*const u32`, while the C helpers pass arbitrary
//   byte addresses; those lane loads go through `vld1_lane_u32_unaligned` /
//   `vld1q_lane_u32_unaligned`, which insert a read_unaligned value into the
//   lane (the same LD1 lane semantics without Rust's alignment requirement).
// - The macros load_unaligned_u32_{2,4}x1_lane become const-generic functions
//   that update `*v` in place; the #undef'd store_*_lane macros become private
//   const-generic functions.
#![allow(clippy::too_many_arguments)]

use core::arch::aarch64::*;
use core::ptr;

use crate::simd::types::tran_low_t;

/// LD1 (single lane) from a possibly unaligned address; see the header note.
#[inline(always)]
pub unsafe fn vld1_lane_u32_unaligned<const LANE: i32>(p: *const u32, v: uint32x2_t) -> uint32x2_t {
    vset_lane_u32::<LANE>(ptr::read_unaligned(p), v)
}

/// LD1 (single lane) from a possibly unaligned address; see the header note.
#[inline(always)]
pub unsafe fn vld1q_lane_u32_unaligned<const LANE: i32>(p: *const u32, v: uint32x4_t) -> uint32x4_t {
    vsetq_lane_u32::<LANE>(ptr::read_unaligned(p), v)
}


#[inline(always)]
pub unsafe fn store_u8_8x2(mut s: *mut u8, p: isize, s0: uint8x8_t, s1: uint8x8_t) {
    vst1_u8(s, s0);
    s = s.offset(p);
    vst1_u8(s, s1);
}

#[inline(always)]
pub unsafe fn load_u8_8x2(s: *const u8, p: isize) -> uint8x16_t {
    vcombine_u8(vld1_u8(s), vld1_u8(s.offset(p)))
}

// Load four bytes into the low half of a uint8x8_t, zero the upper half.
#[inline(always)]
pub unsafe fn load_u8_4x1(p: *const u8) -> uint8x8_t {
    let mut ret = vdup_n_u8(0);
    ret = vreinterpret_u8_u32(vld1_lane_u32_unaligned::<0>(p as *const u32, vreinterpret_u32_u8(ret)));
    ret
}

#[inline(always)]
pub unsafe fn load_u8_4x2(mut p: *const u8, stride: isize) -> uint8x8_t {
    let mut ret = vdup_n_u8(0);
    ret = vreinterpret_u8_u32(vld1_lane_u32_unaligned::<0>(p as *const u32, vreinterpret_u32_u8(ret)));
    p = p.offset(stride);
    ret = vreinterpret_u8_u32(vld1_lane_u32_unaligned::<1>(p as *const u32, vreinterpret_u32_u8(ret)));
    ret
}

#[inline(always)]
pub unsafe fn load_u16_2x2(mut p: *const u16, stride: isize) -> uint16x4_t {
    let mut ret = vdup_n_u16(0);
    ret = vreinterpret_u16_u32(vld1_lane_u32_unaligned::<0>(p as *const u32, vreinterpret_u32_u16(ret)));
    p = p.offset(stride);
    ret = vreinterpret_u16_u32(vld1_lane_u32_unaligned::<1>(p as *const u32, vreinterpret_u32_u16(ret)));
    ret
}

#[inline(always)]
pub unsafe fn load_u8_8x8(mut s: *const u8, p: isize, s0: *mut uint8x8_t, s1: *mut uint8x8_t, s2: *mut uint8x8_t, s3: *mut uint8x8_t, s4: *mut uint8x8_t, s5: *mut uint8x8_t, s6: *mut uint8x8_t, s7: *mut uint8x8_t) {
    *s0 = vld1_u8(s);
    s = s.offset(p);
    *s1 = vld1_u8(s);
    s = s.offset(p);
    *s2 = vld1_u8(s);
    s = s.offset(p);
    *s3 = vld1_u8(s);
    s = s.offset(p);
    *s4 = vld1_u8(s);
    s = s.offset(p);
    *s5 = vld1_u8(s);
    s = s.offset(p);
    *s6 = vld1_u8(s);
    s = s.offset(p);
    *s7 = vld1_u8(s);
}

#[inline(always)]
pub unsafe fn load_u8_8x7(mut s: *const u8, p: isize, s0: *mut uint8x8_t, s1: *mut uint8x8_t, s2: *mut uint8x8_t, s3: *mut uint8x8_t, s4: *mut uint8x8_t, s5: *mut uint8x8_t, s6: *mut uint8x8_t) {
    *s0 = vld1_u8(s);
    s = s.offset(p);
    *s1 = vld1_u8(s);
    s = s.offset(p);
    *s2 = vld1_u8(s);
    s = s.offset(p);
    *s3 = vld1_u8(s);
    s = s.offset(p);
    *s4 = vld1_u8(s);
    s = s.offset(p);
    *s5 = vld1_u8(s);
    s = s.offset(p);
    *s6 = vld1_u8(s);
}

#[inline(always)]
pub unsafe fn load_u8_8x6(mut s: *const u8, p: isize, s0: *mut uint8x8_t, s1: *mut uint8x8_t, s2: *mut uint8x8_t, s3: *mut uint8x8_t, s4: *mut uint8x8_t, s5: *mut uint8x8_t) {
    *s0 = vld1_u8(s);
    s = s.offset(p);
    *s1 = vld1_u8(s);
    s = s.offset(p);
    *s2 = vld1_u8(s);
    s = s.offset(p);
    *s3 = vld1_u8(s);
    s = s.offset(p);
    *s4 = vld1_u8(s);
    s = s.offset(p);
    *s5 = vld1_u8(s);
}

#[inline(always)]
pub unsafe fn load_u8_8x4(mut s: *const u8, p: isize, s0: *mut uint8x8_t, s1: *mut uint8x8_t, s2: *mut uint8x8_t, s3: *mut uint8x8_t) {
    *s0 = vld1_u8(s);
    s = s.offset(p);
    *s1 = vld1_u8(s);
    s = s.offset(p);
    *s2 = vld1_u8(s);
    s = s.offset(p);
    *s3 = vld1_u8(s);
}

#[inline(always)]
pub unsafe fn load_u8_8x3(mut s: *const u8, p: isize, s0: *mut uint8x8_t, s1: *mut uint8x8_t, s2: *mut uint8x8_t) {
    *s0 = vld1_u8(s);
    s = s.offset(p);
    *s1 = vld1_u8(s);
    s = s.offset(p);
    *s2 = vld1_u8(s);
}

#[inline(always)]
pub unsafe fn load_u16_4x4(mut s: *const u16, p: isize, s0: *mut uint16x4_t, s1: *mut uint16x4_t, s2: *mut uint16x4_t, s3: *mut uint16x4_t) {
    *s0 = vld1_u16(s);
    s = s.offset(p);
    *s1 = vld1_u16(s);
    s = s.offset(p);
    *s2 = vld1_u16(s);
    s = s.offset(p);
    *s3 = vld1_u16(s);
}

#[inline(always)]
pub unsafe fn load_u16_4x6(mut s: *const u16, p: isize, s0: *mut uint16x4_t, s1: *mut uint16x4_t, s2: *mut uint16x4_t, s3: *mut uint16x4_t, s4: *mut uint16x4_t, s5: *mut uint16x4_t) {
    *s0 = vld1_u16(s);
    s = s.offset(p);
    *s1 = vld1_u16(s);
    s = s.offset(p);
    *s2 = vld1_u16(s);
    s = s.offset(p);
    *s3 = vld1_u16(s);
    s = s.offset(p);
    *s4 = vld1_u16(s);
    s = s.offset(p);
    *s5 = vld1_u16(s);
}

#[inline(always)]
pub unsafe fn load_u16_4x7(mut s: *const u16, p: isize, s0: *mut uint16x4_t, s1: *mut uint16x4_t, s2: *mut uint16x4_t, s3: *mut uint16x4_t, s4: *mut uint16x4_t, s5: *mut uint16x4_t, s6: *mut uint16x4_t) {
    *s0 = vld1_u16(s);
    s = s.offset(p);
    *s1 = vld1_u16(s);
    s = s.offset(p);
    *s2 = vld1_u16(s);
    s = s.offset(p);
    *s3 = vld1_u16(s);
    s = s.offset(p);
    *s4 = vld1_u16(s);
    s = s.offset(p);
    *s5 = vld1_u16(s);
    s = s.offset(p);
    *s6 = vld1_u16(s);
}

#[inline(always)]
pub unsafe fn load_u16_4x8(mut s: *const u16, p: isize, s0: *mut uint16x4_t, s1: *mut uint16x4_t, s2: *mut uint16x4_t, s3: *mut uint16x4_t, s4: *mut uint16x4_t, s5: *mut uint16x4_t, s6: *mut uint16x4_t, s7: *mut uint16x4_t) {
    *s0 = vld1_u16(s);
    s = s.offset(p);
    *s1 = vld1_u16(s);
    s = s.offset(p);
    *s2 = vld1_u16(s);
    s = s.offset(p);
    *s3 = vld1_u16(s);
    s = s.offset(p);
    *s4 = vld1_u16(s);
    s = s.offset(p);
    *s5 = vld1_u16(s);
    s = s.offset(p);
    *s6 = vld1_u16(s);
    s = s.offset(p);
    *s7 = vld1_u16(s);
}

#[inline(always)]
pub unsafe fn load_u16_4x14(mut s: *const u16, p: isize, s0: *mut uint16x4_t, s1: *mut uint16x4_t, s2: *mut uint16x4_t, s3: *mut uint16x4_t, s4: *mut uint16x4_t, s5: *mut uint16x4_t, s6: *mut uint16x4_t, s7: *mut uint16x4_t, s8: *mut uint16x4_t, s9: *mut uint16x4_t, s10: *mut uint16x4_t, s11: *mut uint16x4_t, s12: *mut uint16x4_t, s13: *mut uint16x4_t) {
    *s0 = vld1_u16(s);
    s = s.offset(p);
    *s1 = vld1_u16(s);
    s = s.offset(p);
    *s2 = vld1_u16(s);
    s = s.offset(p);
    *s3 = vld1_u16(s);
    s = s.offset(p);
    *s4 = vld1_u16(s);
    s = s.offset(p);
    *s5 = vld1_u16(s);
    s = s.offset(p);
    *s6 = vld1_u16(s);
    s = s.offset(p);
    *s7 = vld1_u16(s);
    s = s.offset(p);
    *s8 = vld1_u16(s);
    s = s.offset(p);
    *s9 = vld1_u16(s);
    s = s.offset(p);
    *s10 = vld1_u16(s);
    s = s.offset(p);
    *s11 = vld1_u16(s);
    s = s.offset(p);
    *s12 = vld1_u16(s);
    s = s.offset(p);
    *s13 = vld1_u16(s);
}

#[inline(always)]
pub unsafe fn load_s16_8x2(mut s: *const i16, p: isize, s0: *mut int16x8_t, s1: *mut int16x8_t) {
    *s0 = vld1q_s16(s);
    s = s.offset(p);
    *s1 = vld1q_s16(s);
}

#[inline(always)]
pub unsafe fn load_u16_8x2(mut s: *const u16, p: isize, s0: *mut uint16x8_t, s1: *mut uint16x8_t) {
    *s0 = vld1q_u16(s);
    s = s.offset(p);
    *s1 = vld1q_u16(s);
}

#[inline(always)]
pub unsafe fn load_u16_8x3(mut s: *const u16, p: isize, s0: *mut uint16x8_t, s1: *mut uint16x8_t, s2: *mut uint16x8_t) {
    *s0 = vld1q_u16(s);
    s = s.offset(p);
    *s1 = vld1q_u16(s);
    s = s.offset(p);
    *s2 = vld1q_u16(s);
}

#[inline(always)]
pub unsafe fn load_u16_8x4(mut s: *const u16, p: isize, s0: *mut uint16x8_t, s1: *mut uint16x8_t, s2: *mut uint16x8_t, s3: *mut uint16x8_t) {
    *s0 = vld1q_u16(s);
    s = s.offset(p);
    *s1 = vld1q_u16(s);
    s = s.offset(p);
    *s2 = vld1q_u16(s);
    s = s.offset(p);
    *s3 = vld1q_u16(s);
}

#[inline(always)]
pub unsafe fn load_s16_4x12(mut s: *const i16, p: isize, s0: *mut int16x4_t, s1: *mut int16x4_t, s2: *mut int16x4_t, s3: *mut int16x4_t, s4: *mut int16x4_t, s5: *mut int16x4_t, s6: *mut int16x4_t, s7: *mut int16x4_t, s8: *mut int16x4_t, s9: *mut int16x4_t, s10: *mut int16x4_t, s11: *mut int16x4_t) {
    *s0 = vld1_s16(s);
    s = s.offset(p);
    *s1 = vld1_s16(s);
    s = s.offset(p);
    *s2 = vld1_s16(s);
    s = s.offset(p);
    *s3 = vld1_s16(s);
    s = s.offset(p);
    *s4 = vld1_s16(s);
    s = s.offset(p);
    *s5 = vld1_s16(s);
    s = s.offset(p);
    *s6 = vld1_s16(s);
    s = s.offset(p);
    *s7 = vld1_s16(s);
    s = s.offset(p);
    *s8 = vld1_s16(s);
    s = s.offset(p);
    *s9 = vld1_s16(s);
    s = s.offset(p);
    *s10 = vld1_s16(s);
    s = s.offset(p);
    *s11 = vld1_s16(s);
}

#[inline(always)]
pub unsafe fn load_s16_4x11(mut s: *const i16, p: isize, s0: *mut int16x4_t, s1: *mut int16x4_t, s2: *mut int16x4_t, s3: *mut int16x4_t, s4: *mut int16x4_t, s5: *mut int16x4_t, s6: *mut int16x4_t, s7: *mut int16x4_t, s8: *mut int16x4_t, s9: *mut int16x4_t, s10: *mut int16x4_t) {
    *s0 = vld1_s16(s);
    s = s.offset(p);
    *s1 = vld1_s16(s);
    s = s.offset(p);
    *s2 = vld1_s16(s);
    s = s.offset(p);
    *s3 = vld1_s16(s);
    s = s.offset(p);
    *s4 = vld1_s16(s);
    s = s.offset(p);
    *s5 = vld1_s16(s);
    s = s.offset(p);
    *s6 = vld1_s16(s);
    s = s.offset(p);
    *s7 = vld1_s16(s);
    s = s.offset(p);
    *s8 = vld1_s16(s);
    s = s.offset(p);
    *s9 = vld1_s16(s);
    s = s.offset(p);
    *s10 = vld1_s16(s);
}

#[inline(always)]
pub unsafe fn load_u16_4x11(mut s: *const u16, p: isize, s0: *mut uint16x4_t, s1: *mut uint16x4_t, s2: *mut uint16x4_t, s3: *mut uint16x4_t, s4: *mut uint16x4_t, s5: *mut uint16x4_t, s6: *mut uint16x4_t, s7: *mut uint16x4_t, s8: *mut uint16x4_t, s9: *mut uint16x4_t, s10: *mut uint16x4_t) {
    *s0 = vld1_u16(s);
    s = s.offset(p);
    *s1 = vld1_u16(s);
    s = s.offset(p);
    *s2 = vld1_u16(s);
    s = s.offset(p);
    *s3 = vld1_u16(s);
    s = s.offset(p);
    *s4 = vld1_u16(s);
    s = s.offset(p);
    *s5 = vld1_u16(s);
    s = s.offset(p);
    *s6 = vld1_u16(s);
    s = s.offset(p);
    *s7 = vld1_u16(s);
    s = s.offset(p);
    *s8 = vld1_u16(s);
    s = s.offset(p);
    *s9 = vld1_u16(s);
    s = s.offset(p);
    *s10 = vld1_u16(s);
}

#[inline(always)]
pub unsafe fn load_s16_4x8(mut s: *const i16, p: isize, s0: *mut int16x4_t, s1: *mut int16x4_t, s2: *mut int16x4_t, s3: *mut int16x4_t, s4: *mut int16x4_t, s5: *mut int16x4_t, s6: *mut int16x4_t, s7: *mut int16x4_t) {
    *s0 = vld1_s16(s);
    s = s.offset(p);
    *s1 = vld1_s16(s);
    s = s.offset(p);
    *s2 = vld1_s16(s);
    s = s.offset(p);
    *s3 = vld1_s16(s);
    s = s.offset(p);
    *s4 = vld1_s16(s);
    s = s.offset(p);
    *s5 = vld1_s16(s);
    s = s.offset(p);
    *s6 = vld1_s16(s);
    s = s.offset(p);
    *s7 = vld1_s16(s);
}

#[inline(always)]
pub unsafe fn load_s16_4x7(mut s: *const i16, p: isize, s0: *mut int16x4_t, s1: *mut int16x4_t, s2: *mut int16x4_t, s3: *mut int16x4_t, s4: *mut int16x4_t, s5: *mut int16x4_t, s6: *mut int16x4_t) {
    *s0 = vld1_s16(s);
    s = s.offset(p);
    *s1 = vld1_s16(s);
    s = s.offset(p);
    *s2 = vld1_s16(s);
    s = s.offset(p);
    *s3 = vld1_s16(s);
    s = s.offset(p);
    *s4 = vld1_s16(s);
    s = s.offset(p);
    *s5 = vld1_s16(s);
    s = s.offset(p);
    *s6 = vld1_s16(s);
}

#[inline(always)]
pub unsafe fn load_s16_4x6(mut s: *const i16, p: isize, s0: *mut int16x4_t, s1: *mut int16x4_t, s2: *mut int16x4_t, s3: *mut int16x4_t, s4: *mut int16x4_t, s5: *mut int16x4_t) {
    *s0 = vld1_s16(s);
    s = s.offset(p);
    *s1 = vld1_s16(s);
    s = s.offset(p);
    *s2 = vld1_s16(s);
    s = s.offset(p);
    *s3 = vld1_s16(s);
    s = s.offset(p);
    *s4 = vld1_s16(s);
    s = s.offset(p);
    *s5 = vld1_s16(s);
}

#[inline(always)]
pub unsafe fn load_s16_4x5(mut s: *const i16, p: isize, s0: *mut int16x4_t, s1: *mut int16x4_t, s2: *mut int16x4_t, s3: *mut int16x4_t, s4: *mut int16x4_t) {
    *s0 = vld1_s16(s);
    s = s.offset(p);
    *s1 = vld1_s16(s);
    s = s.offset(p);
    *s2 = vld1_s16(s);
    s = s.offset(p);
    *s3 = vld1_s16(s);
    s = s.offset(p);
    *s4 = vld1_s16(s);
}

#[inline(always)]
pub unsafe fn load_u16_4x5(mut s: *const u16, p: isize, s0: *mut uint16x4_t, s1: *mut uint16x4_t, s2: *mut uint16x4_t, s3: *mut uint16x4_t, s4: *mut uint16x4_t) {
    *s0 = vld1_u16(s);
    s = s.offset(p);
    *s1 = vld1_u16(s);
    s = s.offset(p);
    *s2 = vld1_u16(s);
    s = s.offset(p);
    *s3 = vld1_u16(s);
    s = s.offset(p);
    *s4 = vld1_u16(s);
}

#[inline(always)]
pub unsafe fn load_u8_8x5(mut s: *const u8, p: isize, s0: *mut uint8x8_t, s1: *mut uint8x8_t, s2: *mut uint8x8_t, s3: *mut uint8x8_t, s4: *mut uint8x8_t) {
    *s0 = vld1_u8(s);
    s = s.offset(p);
    *s1 = vld1_u8(s);
    s = s.offset(p);
    *s2 = vld1_u8(s);
    s = s.offset(p);
    *s3 = vld1_u8(s);
    s = s.offset(p);
    *s4 = vld1_u8(s);
}

#[inline(always)]
pub unsafe fn load_u16_8x5(mut s: *const u16, p: isize, s0: *mut uint16x8_t, s1: *mut uint16x8_t, s2: *mut uint16x8_t, s3: *mut uint16x8_t, s4: *mut uint16x8_t) {
    *s0 = vld1q_u16(s);
    s = s.offset(p);
    *s1 = vld1q_u16(s);
    s = s.offset(p);
    *s2 = vld1q_u16(s);
    s = s.offset(p);
    *s3 = vld1q_u16(s);
    s = s.offset(p);
    *s4 = vld1q_u16(s);
}

#[inline(always)]
pub unsafe fn load_s16_4x4(mut s: *const i16, p: isize, s0: *mut int16x4_t, s1: *mut int16x4_t, s2: *mut int16x4_t, s3: *mut int16x4_t) {
    *s0 = vld1_s16(s);
    s = s.offset(p);
    *s1 = vld1_s16(s);
    s = s.offset(p);
    *s2 = vld1_s16(s);
    s = s.offset(p);
    *s3 = vld1_s16(s);
}

#[inline(always)]
pub unsafe fn load_s16_4x3(mut s: *const i16, p: isize, s0: *mut int16x4_t, s1: *mut int16x4_t, s2: *mut int16x4_t) {
    *s0 = vld1_s16(s);
    s = s.offset(p);
    *s1 = vld1_s16(s);
    s = s.offset(p);
    *s2 = vld1_s16(s);
}

#[inline(always)]
pub unsafe fn store_u8_8x8(mut s: *mut u8, p: isize, s0: uint8x8_t, s1: uint8x8_t, s2: uint8x8_t, s3: uint8x8_t, s4: uint8x8_t, s5: uint8x8_t, s6: uint8x8_t, s7: uint8x8_t) {
    vst1_u8(s, s0);
    s = s.offset(p);
    vst1_u8(s, s1);
    s = s.offset(p);
    vst1_u8(s, s2);
    s = s.offset(p);
    vst1_u8(s, s3);
    s = s.offset(p);
    vst1_u8(s, s4);
    s = s.offset(p);
    vst1_u8(s, s5);
    s = s.offset(p);
    vst1_u8(s, s6);
    s = s.offset(p);
    vst1_u8(s, s7);
}

#[inline(always)]
pub unsafe fn store_u8_8x4(mut s: *mut u8, p: isize, s0: uint8x8_t, s1: uint8x8_t, s2: uint8x8_t, s3: uint8x8_t) {
    vst1_u8(s, s0);
    s = s.offset(p);
    vst1_u8(s, s1);
    s = s.offset(p);
    vst1_u8(s, s2);
    s = s.offset(p);
    vst1_u8(s, s3);
}

#[inline(always)]
pub unsafe fn store_u8_16x4(mut s: *mut u8, p: isize, s0: uint8x16_t, s1: uint8x16_t, s2: uint8x16_t, s3: uint8x16_t) {
    vst1q_u8(s, s0);
    s = s.offset(p);
    vst1q_u8(s, s1);
    s = s.offset(p);
    vst1q_u8(s, s2);
    s = s.offset(p);
    vst1q_u8(s, s3);
}

#[inline(always)]
pub unsafe fn store_u16_8x8(mut s: *mut u16, dst_stride: isize, s0: uint16x8_t, s1: uint16x8_t, s2: uint16x8_t, s3: uint16x8_t, s4: uint16x8_t, s5: uint16x8_t, s6: uint16x8_t, s7: uint16x8_t) {
    vst1q_u16(s, s0);
    s = s.offset(dst_stride);
    vst1q_u16(s, s1);
    s = s.offset(dst_stride);
    vst1q_u16(s, s2);
    s = s.offset(dst_stride);
    vst1q_u16(s, s3);
    s = s.offset(dst_stride);
    vst1q_u16(s, s4);
    s = s.offset(dst_stride);
    vst1q_u16(s, s5);
    s = s.offset(dst_stride);
    vst1q_u16(s, s6);
    s = s.offset(dst_stride);
    vst1q_u16(s, s7);
}

#[inline(always)]
pub unsafe fn store_u16_4x3(mut s: *mut u16, dst_stride: isize, s0: uint16x4_t, s1: uint16x4_t, s2: uint16x4_t) {
    vst1_u16(s, s0);
    s = s.offset(dst_stride);
    vst1_u16(s, s1);
    s = s.offset(dst_stride);
    vst1_u16(s, s2);
}

#[inline(always)]
pub unsafe fn store_u16_4x4(mut s: *mut u16, dst_stride: isize, s0: uint16x4_t, s1: uint16x4_t, s2: uint16x4_t, s3: uint16x4_t) {
    vst1_u16(s, s0);
    s = s.offset(dst_stride);
    vst1_u16(s, s1);
    s = s.offset(dst_stride);
    vst1_u16(s, s2);
    s = s.offset(dst_stride);
    vst1_u16(s, s3);
}

#[inline(always)]
pub unsafe fn store_u16_4x6(mut s: *mut u16, dst_stride: isize, s0: uint16x4_t, s1: uint16x4_t, s2: uint16x4_t, s3: uint16x4_t, s4: uint16x4_t, s5: uint16x4_t) {
    vst1_u16(s, s0);
    s = s.offset(dst_stride);
    vst1_u16(s, s1);
    s = s.offset(dst_stride);
    vst1_u16(s, s2);
    s = s.offset(dst_stride);
    vst1_u16(s, s3);
    s = s.offset(dst_stride);
    vst1_u16(s, s4);
    s = s.offset(dst_stride);
    vst1_u16(s, s5);
}

#[inline(always)]
pub unsafe fn store_u16_4x12(mut s: *mut u16, dst_stride: isize, s0: uint16x4_t, s1: uint16x4_t, s2: uint16x4_t, s3: uint16x4_t, s4: uint16x4_t, s5: uint16x4_t, s6: uint16x4_t, s7: uint16x4_t, s8: uint16x4_t, s9: uint16x4_t, s10: uint16x4_t, s11: uint16x4_t) {
    vst1_u16(s, s0);
    s = s.offset(dst_stride);
    vst1_u16(s, s1);
    s = s.offset(dst_stride);
    vst1_u16(s, s2);
    s = s.offset(dst_stride);
    vst1_u16(s, s3);
    s = s.offset(dst_stride);
    vst1_u16(s, s4);
    s = s.offset(dst_stride);
    vst1_u16(s, s5);
    s = s.offset(dst_stride);
    vst1_u16(s, s6);
    s = s.offset(dst_stride);
    vst1_u16(s, s7);
    s = s.offset(dst_stride);
    vst1_u16(s, s8);
    s = s.offset(dst_stride);
    vst1_u16(s, s9);
    s = s.offset(dst_stride);
    vst1_u16(s, s10);
    s = s.offset(dst_stride);
    vst1_u16(s, s11);
}

#[inline(always)]
pub unsafe fn store_u16_8x2(mut s: *mut u16, dst_stride: isize, s0: uint16x8_t, s1: uint16x8_t) {
    vst1q_u16(s, s0);
    s = s.offset(dst_stride);
    vst1q_u16(s, s1);
}

#[inline(always)]
pub unsafe fn store_u16_8x3(mut s: *mut u16, dst_stride: isize, s0: uint16x8_t, s1: uint16x8_t, s2: uint16x8_t) {
    vst1q_u16(s, s0);
    s = s.offset(dst_stride);
    vst1q_u16(s, s1);
    s = s.offset(dst_stride);
    vst1q_u16(s, s2);
}

#[inline(always)]
pub unsafe fn store_u16_8x4(mut s: *mut u16, dst_stride: isize, s0: uint16x8_t, s1: uint16x8_t, s2: uint16x8_t, s3: uint16x8_t) {
    vst1q_u16(s, s0);
    s = s.offset(dst_stride);
    vst1q_u16(s, s1);
    s = s.offset(dst_stride);
    vst1q_u16(s, s2);
    s = s.offset(dst_stride);
    vst1q_u16(s, s3);
}

#[inline(always)]
pub unsafe fn store_s16_8x8(mut s: *mut i16, dst_stride: isize, s0: int16x8_t, s1: int16x8_t, s2: int16x8_t, s3: int16x8_t, s4: int16x8_t, s5: int16x8_t, s6: int16x8_t, s7: int16x8_t) {
    vst1q_s16(s, s0);
    s = s.offset(dst_stride);
    vst1q_s16(s, s1);
    s = s.offset(dst_stride);
    vst1q_s16(s, s2);
    s = s.offset(dst_stride);
    vst1q_s16(s, s3);
    s = s.offset(dst_stride);
    vst1q_s16(s, s4);
    s = s.offset(dst_stride);
    vst1q_s16(s, s5);
    s = s.offset(dst_stride);
    vst1q_s16(s, s6);
    s = s.offset(dst_stride);
    vst1q_s16(s, s7);
}

#[inline(always)]
pub unsafe fn store_s16_4x4(mut s: *mut i16, dst_stride: isize, s0: int16x4_t, s1: int16x4_t, s2: int16x4_t, s3: int16x4_t) {
    vst1_s16(s, s0);
    s = s.offset(dst_stride);
    vst1_s16(s, s1);
    s = s.offset(dst_stride);
    vst1_s16(s, s2);
    s = s.offset(dst_stride);
    vst1_s16(s, s3);
}

#[inline(always)]
pub unsafe fn store_s16_4x8(mut s: *mut i16, dst_stride: isize, s0: int16x4_t, s1: int16x4_t, s2: int16x4_t, s3: int16x4_t, s4: int16x4_t, s5: int16x4_t, s6: int16x4_t, s7: int16x4_t) {
    vst1_s16(s, s0);
    s = s.offset(dst_stride);
    vst1_s16(s, s1);
    s = s.offset(dst_stride);
    vst1_s16(s, s2);
    s = s.offset(dst_stride);
    vst1_s16(s, s3);
    s = s.offset(dst_stride);
    vst1_s16(s, s4);
    s = s.offset(dst_stride);
    vst1_s16(s, s5);
    s = s.offset(dst_stride);
    vst1_s16(s, s6);
    s = s.offset(dst_stride);
    vst1_s16(s, s7);
}

#[inline(always)]
pub unsafe fn store_s16_8x4(mut s: *mut i16, dst_stride: isize, s0: int16x8_t, s1: int16x8_t, s2: int16x8_t, s3: int16x8_t) {
    vst1q_s16(s, s0);
    s = s.offset(dst_stride);
    vst1q_s16(s, s1);
    s = s.offset(dst_stride);
    vst1q_s16(s, s2);
    s = s.offset(dst_stride);
    vst1q_s16(s, s3);
}

#[inline(always)]
pub unsafe fn store_s16_8x2(mut s: *mut i16, dst_stride: isize, s0: int16x8_t, s1: int16x8_t) {
    vst1q_s16(s, s0);
    s = s.offset(dst_stride);
    vst1q_s16(s, s1);
}

#[inline(always)]
pub unsafe fn load_u8_8x11(mut s: *const u8, p: isize, s0: *mut uint8x8_t, s1: *mut uint8x8_t, s2: *mut uint8x8_t, s3: *mut uint8x8_t, s4: *mut uint8x8_t, s5: *mut uint8x8_t, s6: *mut uint8x8_t, s7: *mut uint8x8_t, s8: *mut uint8x8_t, s9: *mut uint8x8_t, s10: *mut uint8x8_t) {
    *s0 = vld1_u8(s);
    s = s.offset(p);
    *s1 = vld1_u8(s);
    s = s.offset(p);
    *s2 = vld1_u8(s);
    s = s.offset(p);
    *s3 = vld1_u8(s);
    s = s.offset(p);
    *s4 = vld1_u8(s);
    s = s.offset(p);
    *s5 = vld1_u8(s);
    s = s.offset(p);
    *s6 = vld1_u8(s);
    s = s.offset(p);
    *s7 = vld1_u8(s);
    s = s.offset(p);
    *s8 = vld1_u8(s);
    s = s.offset(p);
    *s9 = vld1_u8(s);
    s = s.offset(p);
    *s10 = vld1_u8(s);
}

#[inline(always)]
pub unsafe fn load_s16_8x10(mut s: *const i16, p: isize, s0: *mut int16x8_t, s1: *mut int16x8_t, s2: *mut int16x8_t, s3: *mut int16x8_t, s4: *mut int16x8_t, s5: *mut int16x8_t, s6: *mut int16x8_t, s7: *mut int16x8_t, s8: *mut int16x8_t, s9: *mut int16x8_t) {
    *s0 = vld1q_s16(s);
    s = s.offset(p);
    *s1 = vld1q_s16(s);
    s = s.offset(p);
    *s2 = vld1q_s16(s);
    s = s.offset(p);
    *s3 = vld1q_s16(s);
    s = s.offset(p);
    *s4 = vld1q_s16(s);
    s = s.offset(p);
    *s5 = vld1q_s16(s);
    s = s.offset(p);
    *s6 = vld1q_s16(s);
    s = s.offset(p);
    *s7 = vld1q_s16(s);
    s = s.offset(p);
    *s8 = vld1q_s16(s);
    s = s.offset(p);
    *s9 = vld1q_s16(s);
}

#[inline(always)]
pub unsafe fn load_s16_8x11(mut s: *const i16, p: isize, s0: *mut int16x8_t, s1: *mut int16x8_t, s2: *mut int16x8_t, s3: *mut int16x8_t, s4: *mut int16x8_t, s5: *mut int16x8_t, s6: *mut int16x8_t, s7: *mut int16x8_t, s8: *mut int16x8_t, s9: *mut int16x8_t, s10: *mut int16x8_t) {
    *s0 = vld1q_s16(s);
    s = s.offset(p);
    *s1 = vld1q_s16(s);
    s = s.offset(p);
    *s2 = vld1q_s16(s);
    s = s.offset(p);
    *s3 = vld1q_s16(s);
    s = s.offset(p);
    *s4 = vld1q_s16(s);
    s = s.offset(p);
    *s5 = vld1q_s16(s);
    s = s.offset(p);
    *s6 = vld1q_s16(s);
    s = s.offset(p);
    *s7 = vld1q_s16(s);
    s = s.offset(p);
    *s8 = vld1q_s16(s);
    s = s.offset(p);
    *s9 = vld1q_s16(s);
    s = s.offset(p);
    *s10 = vld1q_s16(s);
}

#[inline(always)]
pub unsafe fn load_s16_8x12(mut s: *const i16, p: isize, s0: *mut int16x8_t, s1: *mut int16x8_t, s2: *mut int16x8_t, s3: *mut int16x8_t, s4: *mut int16x8_t, s5: *mut int16x8_t, s6: *mut int16x8_t, s7: *mut int16x8_t, s8: *mut int16x8_t, s9: *mut int16x8_t, s10: *mut int16x8_t, s11: *mut int16x8_t) {
    *s0 = vld1q_s16(s);
    s = s.offset(p);
    *s1 = vld1q_s16(s);
    s = s.offset(p);
    *s2 = vld1q_s16(s);
    s = s.offset(p);
    *s3 = vld1q_s16(s);
    s = s.offset(p);
    *s4 = vld1q_s16(s);
    s = s.offset(p);
    *s5 = vld1q_s16(s);
    s = s.offset(p);
    *s6 = vld1q_s16(s);
    s = s.offset(p);
    *s7 = vld1q_s16(s);
    s = s.offset(p);
    *s8 = vld1q_s16(s);
    s = s.offset(p);
    *s9 = vld1q_s16(s);
    s = s.offset(p);
    *s10 = vld1q_s16(s);
    s = s.offset(p);
    *s11 = vld1q_s16(s);
}

#[inline(always)]
pub unsafe fn load_u16_8x11(mut s: *const u16, p: isize, s0: *mut uint16x8_t, s1: *mut uint16x8_t, s2: *mut uint16x8_t, s3: *mut uint16x8_t, s4: *mut uint16x8_t, s5: *mut uint16x8_t, s6: *mut uint16x8_t, s7: *mut uint16x8_t, s8: *mut uint16x8_t, s9: *mut uint16x8_t, s10: *mut uint16x8_t) {
    *s0 = vld1q_u16(s);
    s = s.offset(p);
    *s1 = vld1q_u16(s);
    s = s.offset(p);
    *s2 = vld1q_u16(s);
    s = s.offset(p);
    *s3 = vld1q_u16(s);
    s = s.offset(p);
    *s4 = vld1q_u16(s);
    s = s.offset(p);
    *s5 = vld1q_u16(s);
    s = s.offset(p);
    *s6 = vld1q_u16(s);
    s = s.offset(p);
    *s7 = vld1q_u16(s);
    s = s.offset(p);
    *s8 = vld1q_u16(s);
    s = s.offset(p);
    *s9 = vld1q_u16(s);
    s = s.offset(p);
    *s10 = vld1q_u16(s);
}

#[inline(always)]
pub unsafe fn load_s16_8x8(mut s: *const i16, p: isize, s0: *mut int16x8_t, s1: *mut int16x8_t, s2: *mut int16x8_t, s3: *mut int16x8_t, s4: *mut int16x8_t, s5: *mut int16x8_t, s6: *mut int16x8_t, s7: *mut int16x8_t) {
    *s0 = vld1q_s16(s);
    s = s.offset(p);
    *s1 = vld1q_s16(s);
    s = s.offset(p);
    *s2 = vld1q_s16(s);
    s = s.offset(p);
    *s3 = vld1q_s16(s);
    s = s.offset(p);
    *s4 = vld1q_s16(s);
    s = s.offset(p);
    *s5 = vld1q_s16(s);
    s = s.offset(p);
    *s6 = vld1q_s16(s);
    s = s.offset(p);
    *s7 = vld1q_s16(s);
}

#[inline(always)]
pub unsafe fn load_u16_8x7(mut s: *const u16, p: isize, s0: *mut uint16x8_t, s1: *mut uint16x8_t, s2: *mut uint16x8_t, s3: *mut uint16x8_t, s4: *mut uint16x8_t, s5: *mut uint16x8_t, s6: *mut uint16x8_t) {
    *s0 = vld1q_u16(s);
    s = s.offset(p);
    *s1 = vld1q_u16(s);
    s = s.offset(p);
    *s2 = vld1q_u16(s);
    s = s.offset(p);
    *s3 = vld1q_u16(s);
    s = s.offset(p);
    *s4 = vld1q_u16(s);
    s = s.offset(p);
    *s5 = vld1q_u16(s);
    s = s.offset(p);
    *s6 = vld1q_u16(s);
}

#[inline(always)]
pub unsafe fn load_s16_8x7(mut s: *const i16, p: isize, s0: *mut int16x8_t, s1: *mut int16x8_t, s2: *mut int16x8_t, s3: *mut int16x8_t, s4: *mut int16x8_t, s5: *mut int16x8_t, s6: *mut int16x8_t) {
    *s0 = vld1q_s16(s);
    s = s.offset(p);
    *s1 = vld1q_s16(s);
    s = s.offset(p);
    *s2 = vld1q_s16(s);
    s = s.offset(p);
    *s3 = vld1q_s16(s);
    s = s.offset(p);
    *s4 = vld1q_s16(s);
    s = s.offset(p);
    *s5 = vld1q_s16(s);
    s = s.offset(p);
    *s6 = vld1q_s16(s);
}

#[inline(always)]
pub unsafe fn load_s16_8x6(mut s: *const i16, p: isize, s0: *mut int16x8_t, s1: *mut int16x8_t, s2: *mut int16x8_t, s3: *mut int16x8_t, s4: *mut int16x8_t, s5: *mut int16x8_t) {
    *s0 = vld1q_s16(s);
    s = s.offset(p);
    *s1 = vld1q_s16(s);
    s = s.offset(p);
    *s2 = vld1q_s16(s);
    s = s.offset(p);
    *s3 = vld1q_s16(s);
    s = s.offset(p);
    *s4 = vld1q_s16(s);
    s = s.offset(p);
    *s5 = vld1q_s16(s);
}

#[inline(always)]
pub unsafe fn load_s16_8x5(mut s: *const i16, p: isize, s0: *mut int16x8_t, s1: *mut int16x8_t, s2: *mut int16x8_t, s3: *mut int16x8_t, s4: *mut int16x8_t) {
    *s0 = vld1q_s16(s);
    s = s.offset(p);
    *s1 = vld1q_s16(s);
    s = s.offset(p);
    *s2 = vld1q_s16(s);
    s = s.offset(p);
    *s3 = vld1q_s16(s);
    s = s.offset(p);
    *s4 = vld1q_s16(s);
}

#[inline(always)]
pub unsafe fn load_s16_8x4(mut s: *const i16, p: isize, s0: *mut int16x8_t, s1: *mut int16x8_t, s2: *mut int16x8_t, s3: *mut int16x8_t) {
    *s0 = vld1q_s16(s);
    s = s.offset(p);
    *s1 = vld1q_s16(s);
    s = s.offset(p);
    *s2 = vld1q_s16(s);
    s = s.offset(p);
    *s3 = vld1q_s16(s);
}

#[inline(always)]
pub unsafe fn load_s16_8x3(mut s: *const i16, p: isize, s0: *mut int16x8_t, s1: *mut int16x8_t, s2: *mut int16x8_t) {
    *s0 = vld1q_s16(s);
    s = s.offset(p);
    *s1 = vld1q_s16(s);
    s = s.offset(p);
    *s2 = vld1q_s16(s);
}

// #define load_unaligned_u32_2x1_lane(v, p, lane) (AOM_ARCH_AARCH64 branch)
#[inline(always)]
pub unsafe fn load_unaligned_u32_2x1_lane<const LANE: i32, T>(v: *mut uint32x2_t, p: *const T) {
    *v = vld1_lane_u32_unaligned::<LANE>(p as *const u32, *v);
}

// #define load_unaligned_u32_4x1_lane(v, p, lane) (AOM_ARCH_AARCH64 branch)
#[inline(always)]
pub unsafe fn load_unaligned_u32_4x1_lane<const LANE: i32, T>(v: *mut uint32x4_t, p: *const T) {
    *v = vld1q_lane_u32_unaligned::<LANE>(p as *const u32, *v);
}

// Load 2 sets of 4 bytes when alignment is not guaranteed.
#[inline(always)]
pub unsafe fn load_unaligned_u8(mut buf: *const u8, stride: isize) -> uint8x8_t {
    let mut a: u32 = ptr::read_unaligned(buf as *const u32);
    buf = buf.offset(stride);
    let mut a_u32 = vdup_n_u32(a);
    a = ptr::read_unaligned(buf as *const u32);
    a_u32 = vset_lane_u32::<1>(a, a_u32);
    vreinterpret_u8_u32(a_u32)
}

// Load 4 sets of 4 bytes when alignment is not guaranteed.
#[inline(always)]
pub unsafe fn load_unaligned_u8q(mut buf: *const u8, stride: isize) -> uint8x16_t {
    let mut a: u32;
    let mut a_u32: uint32x4_t;
    if stride == 4 {
        return vld1q_u8(buf);
    }
    a = ptr::read_unaligned(buf as *const u32);
    buf = buf.offset(stride);
    a_u32 = vdupq_n_u32(a);
    a = ptr::read_unaligned(buf as *const u32);
    buf = buf.offset(stride);
    a_u32 = vsetq_lane_u32::<1>(a, a_u32);
    a = ptr::read_unaligned(buf as *const u32);
    buf = buf.offset(stride);
    a_u32 = vsetq_lane_u32::<2>(a, a_u32);
    a = ptr::read_unaligned(buf as *const u32);
    a_u32 = vsetq_lane_u32::<3>(a, a_u32);
    vreinterpretq_u8_u32(a_u32)
}

#[inline(always)]
pub unsafe fn load_unaligned_u8_2x2(mut buf: *const u8, stride: isize) -> uint8x8_t {
    let mut a: u16;
    let mut a_u16: uint16x4_t;

    a = ptr::read_unaligned(buf as *const u16);
    buf = buf.offset(stride);
    a_u16 = vdup_n_u16(a);
    a = ptr::read_unaligned(buf as *const u16);
    a_u16 = vset_lane_u16::<1>(a, a_u16);
    vreinterpret_u8_u16(a_u16)
}

#[inline(always)]
pub unsafe fn load_unaligned_u8_4x1(buf: *const u8) -> uint8x8_t {
    let a: u32;
    let mut a_u32: uint32x2_t;

    a = ptr::read_unaligned(buf as *const u32);
    a_u32 = vdup_n_u32(0);
    a_u32 = vset_lane_u32::<0>(a, a_u32);
    vreinterpret_u8_u32(a_u32)
}

#[inline(always)]
pub unsafe fn load_unaligned_dup_u8_4x2(buf: *const u8) -> uint8x8_t {
    let a: u32;
    let a_u32: uint32x2_t;

    a = ptr::read_unaligned(buf as *const u32);
    a_u32 = vdup_n_u32(a);
    vreinterpret_u8_u32(a_u32)
}

#[inline(always)]
pub unsafe fn load_unaligned_dup_u8_2x4(buf: *const u8) -> uint8x8_t {
    let a: u16;
    let a_u32: uint16x4_t;

    a = ptr::read_unaligned(buf as *const u16);
    a_u32 = vdup_n_u16(a);
    vreinterpret_u8_u16(a_u32)
}

#[inline(always)]
pub unsafe fn load_unaligned_u8_4x2(mut buf: *const u8, stride: isize) -> uint8x8_t {
    let mut a: u32;
    let mut a_u32: uint32x2_t;

    a = ptr::read_unaligned(buf as *const u32);
    buf = buf.offset(stride);
    a_u32 = vdup_n_u32(a);
    a = ptr::read_unaligned(buf as *const u32);
    a_u32 = vset_lane_u32::<1>(a, a_u32);
    vreinterpret_u8_u32(a_u32)
}

#[inline(always)]
pub unsafe fn load_unaligned_u8_4x4(mut buf: *const u8, stride: isize, tu0: *mut uint8x8_t,
                                    tu1: *mut uint8x8_t) {
    *tu0 = load_unaligned_u8_4x2(buf, stride);
    buf = buf.offset(2 * stride);
    *tu1 = load_unaligned_u8_4x2(buf, stride);
}

#[inline(always)]
pub unsafe fn load_unaligned_u8_3x8(mut buf: *const u8, stride: isize, tu0: *mut uint8x8_t,
                                    tu1: *mut uint8x8_t, tu2: *mut uint8x8_t) {
    load_unaligned_u8_4x4(buf, stride, tu0, tu1);
    buf = buf.offset(4 * stride);
    *tu2 = load_unaligned_u8_4x2(buf, stride);
}

#[inline(always)]
pub unsafe fn load_unaligned_u8_4x8(mut buf: *const u8, stride: isize, tu0: *mut uint8x8_t,
                                    tu1: *mut uint8x8_t, tu2: *mut uint8x8_t,
                                    tu3: *mut uint8x8_t) {
    load_unaligned_u8_4x4(buf, stride, tu0, tu1);
    buf = buf.offset(4 * stride);
    load_unaligned_u8_4x4(buf, stride, tu2, tu3);
}

#[inline(always)]
pub unsafe fn load_u8_16x8(mut s: *const u8, p: isize, s0: *mut uint8x16_t, s1: *mut uint8x16_t, s2: *mut uint8x16_t, s3: *mut uint8x16_t, s4: *mut uint8x16_t, s5: *mut uint8x16_t, s6: *mut uint8x16_t, s7: *mut uint8x16_t) {
    *s0 = vld1q_u8(s);
    s = s.offset(p);
    *s1 = vld1q_u8(s);
    s = s.offset(p);
    *s2 = vld1q_u8(s);
    s = s.offset(p);
    *s3 = vld1q_u8(s);
    s = s.offset(p);
    *s4 = vld1q_u8(s);
    s = s.offset(p);
    *s5 = vld1q_u8(s);
    s = s.offset(p);
    *s6 = vld1q_u8(s);
    s = s.offset(p);
    *s7 = vld1q_u8(s);
}

#[inline(always)]
pub unsafe fn load_u8_16x5(mut s: *const u8, p: isize, s0: *mut uint8x16_t, s1: *mut uint8x16_t, s2: *mut uint8x16_t, s3: *mut uint8x16_t, s4: *mut uint8x16_t) {
    *s0 = vld1q_u8(s);
    s = s.offset(p);
    *s1 = vld1q_u8(s);
    s = s.offset(p);
    *s2 = vld1q_u8(s);
    s = s.offset(p);
    *s3 = vld1q_u8(s);
    s = s.offset(p);
    *s4 = vld1q_u8(s);
}

#[inline(always)]
pub unsafe fn load_u8_16x4(mut s: *const u8, p: isize, s0: *mut uint8x16_t, s1: *mut uint8x16_t, s2: *mut uint8x16_t, s3: *mut uint8x16_t) {
    *s0 = vld1q_u8(s);
    s = s.offset(p);
    *s1 = vld1q_u8(s);
    s = s.offset(p);
    *s2 = vld1q_u8(s);
    s = s.offset(p);
    *s3 = vld1q_u8(s);
}

#[inline(always)]
pub unsafe fn load_u8_16x3(mut s: *const u8, p: isize, s0: *mut uint8x16_t, s1: *mut uint8x16_t, s2: *mut uint8x16_t) {
    *s0 = vld1q_u8(s);
    s = s.offset(p);
    *s1 = vld1q_u8(s);
    s = s.offset(p);
    *s2 = vld1q_u8(s);
}

#[inline(always)]
pub unsafe fn load_u16_8x8(mut s: *const u16, p: isize, s0: *mut uint16x8_t, s1: *mut uint16x8_t, s2: *mut uint16x8_t, s3: *mut uint16x8_t, s4: *mut uint16x8_t, s5: *mut uint16x8_t, s6: *mut uint16x8_t, s7: *mut uint16x8_t) {
    *s0 = vld1q_u16(s);
    s = s.offset(p);
    *s1 = vld1q_u16(s);
    s = s.offset(p);
    *s2 = vld1q_u16(s);
    s = s.offset(p);
    *s3 = vld1q_u16(s);
    s = s.offset(p);
    *s4 = vld1q_u16(s);
    s = s.offset(p);
    *s5 = vld1q_u16(s);
    s = s.offset(p);
    *s6 = vld1q_u16(s);
    s = s.offset(p);
    *s7 = vld1q_u16(s);
}

#[inline(always)]
pub unsafe fn load_u16_16x4(mut s: *const u16, p: isize, s0: *mut uint16x8_t,
                            s1: *mut uint16x8_t, s2: *mut uint16x8_t, s3: *mut uint16x8_t,
                            s4: *mut uint16x8_t, s5: *mut uint16x8_t, s6: *mut uint16x8_t,
                            s7: *mut uint16x8_t) {
    *s0 = vld1q_u16(s);
    *s1 = vld1q_u16(s.add(8));
    s = s.offset(p);
    *s2 = vld1q_u16(s);
    *s3 = vld1q_u16(s.add(8));
    s = s.offset(p);
    *s4 = vld1q_u16(s);
    *s5 = vld1q_u16(s.add(8));
    s = s.offset(p);
    *s6 = vld1q_u16(s);
    *s7 = vld1q_u16(s.add(8));
}

#[inline(always)]
pub unsafe fn load_unaligned_u16_2x2(mut buf: *const u16, stride: isize) -> uint16x4_t {
    let mut a: u32;
    let mut a_u32: uint32x2_t;

    a = ptr::read_unaligned(buf as *const u32);
    buf = buf.offset(stride);
    a_u32 = vdup_n_u32(a);
    a = ptr::read_unaligned(buf as *const u32);
    a_u32 = vset_lane_u32::<1>(a, a_u32);
    vreinterpret_u16_u32(a_u32)
}

#[inline(always)]
pub unsafe fn load_unaligned_u16_4x1(buf: *const u16) -> uint16x4_t {
    let a: u64;
    let mut a_u64 = vdup_n_u64(0);
    a = ptr::read_unaligned(buf as *const u64);
    a_u64 = vset_lane_u64::<0>(a, a_u64);
    vreinterpret_u16_u64(a_u64)
}

#[inline(always)]
pub unsafe fn load_unaligned_u16_4x2(mut buf: *const u16, stride: isize) -> uint16x8_t {
    let mut a: u64;
    let mut a_u64: uint64x2_t;

    a = ptr::read_unaligned(buf as *const u64);
    buf = buf.offset(stride);
    a_u64 = vdupq_n_u64(0);
    a_u64 = vsetq_lane_u64::<0>(a, a_u64);
    a = ptr::read_unaligned(buf as *const u64);
    a_u64 = vsetq_lane_u64::<1>(a, a_u64);
    vreinterpretq_u16_u64(a_u64)
}

#[inline(always)]
pub unsafe fn load_unaligned_s16_4x2(mut buf: *const i16, stride: isize) -> int16x8_t {
    let mut a: i64;
    let mut a_s64: int64x2_t;
    a = ptr::read_unaligned(buf as *const i64);
    buf = buf.offset(stride);
    a_s64 = vdupq_n_s64(0);
    a_s64 = vsetq_lane_s64::<0>(a, a_s64);
    a = ptr::read_unaligned(buf as *const i64);
    a_s64 = vsetq_lane_s64::<1>(a, a_s64);
    vreinterpretq_s16_s64(a_s64)
}

#[inline(always)]
pub unsafe fn load_unaligned_u16_4x4(mut buf: *const u16, stride: isize, tu0: *mut uint16x8_t,
                                     tu1: *mut uint16x8_t) {
    *tu0 = load_unaligned_u16_4x2(buf, stride);
    buf = buf.offset(2 * stride);
    *tu1 = load_unaligned_u16_4x2(buf, stride);
}

#[inline(always)]
pub unsafe fn load_s32_4x4(mut s: *mut i32, p: isize, s1: *mut int32x4_t, s2: *mut int32x4_t, s3: *mut int32x4_t, s4: *mut int32x4_t) {
    *s1 = vld1q_s32(s);
    s = s.offset(p);
    *s2 = vld1q_s32(s);
    s = s.offset(p);
    *s3 = vld1q_s32(s);
    s = s.offset(p);
    *s4 = vld1q_s32(s);
}

#[inline(always)]
pub unsafe fn store_s32_4x4(mut s: *mut i32, p: isize, s1: int32x4_t, s2: int32x4_t, s3: int32x4_t, s4: int32x4_t) {
    vst1q_s32(s, s1);
    s = s.offset(p);
    vst1q_s32(s, s2);
    s = s.offset(p);
    vst1q_s32(s, s3);
    s = s.offset(p);
    vst1q_s32(s, s4);
}

#[inline(always)]
pub unsafe fn load_u32_4x4(mut s: *mut u32, p: isize, s1: *mut uint32x4_t, s2: *mut uint32x4_t, s3: *mut uint32x4_t, s4: *mut uint32x4_t) {
    *s1 = vld1q_u32(s);
    s = s.offset(p);
    *s2 = vld1q_u32(s);
    s = s.offset(p);
    *s3 = vld1q_u32(s);
    s = s.offset(p);
    *s4 = vld1q_u32(s);
}

#[inline(always)]
pub unsafe fn store_u32_4x4(mut s: *mut u32, p: isize, s1: uint32x4_t, s2: uint32x4_t, s3: uint32x4_t, s4: uint32x4_t) {
    vst1q_u32(s, s1);
    s = s.offset(p);
    vst1q_u32(s, s2);
    s = s.offset(p);
    vst1q_u32(s, s3);
    s = s.offset(p);
    vst1q_u32(s, s4);
}

#[inline(always)]
pub unsafe fn load_tran_low_to_s16q(buf: *const tran_low_t) -> int16x8_t {
    let v0 = vld1q_s32(buf);
    let v1 = vld1q_s32(buf.add(4));
    let s0 = vmovn_s32(v0);
    let s1 = vmovn_s32(v1);
    vcombine_s16(s0, s1)
}

#[inline(always)]
pub unsafe fn store_s16q_to_tran_low(buf: *mut tran_low_t, a: int16x8_t) {
    let v0 = vmovl_s16(vget_low_s16(a));
    let v1 = vmovl_s16(vget_high_s16(a));
    vst1q_s32(buf, v0);
    vst1q_s32(buf.add(4), v1);
}

#[inline(always)]
pub unsafe fn store_s16_to_tran_low(buf: *mut tran_low_t, a: int16x4_t) {
    let v0 = vmovl_s16(a);
    vst1q_s32(buf, v0);
}

#[inline(always)]
pub unsafe fn load_u8_gather_s16_x8(src: *const u8, indices: int16x8_t) -> uint8x8_t {
    // Recent Clang and GCC versions correctly identify that this zero-broadcast
    // is redundant. Alternatively we could load and broadcast the zeroth element
    // and then replace the other lanes, however this is slower than loading a
    // single element without broadcast on some micro-architectures.
    let mut ret = vdup_n_u8(0);
    ret = vld1_lane_u8::<0>(src.offset(vget_lane_s16::<0>(vget_low_s16(indices)) as isize), ret);
    ret = vld1_lane_u8::<1>(src.offset(vget_lane_s16::<1>(vget_low_s16(indices)) as isize), ret);
    ret = vld1_lane_u8::<2>(src.offset(vget_lane_s16::<2>(vget_low_s16(indices)) as isize), ret);
    ret = vld1_lane_u8::<3>(src.offset(vget_lane_s16::<3>(vget_low_s16(indices)) as isize), ret);
    ret = vld1_lane_u8::<4>(src.offset(vget_lane_s16::<0>(vget_high_s16(indices)) as isize), ret);
    ret = vld1_lane_u8::<5>(src.offset(vget_lane_s16::<1>(vget_high_s16(indices)) as isize), ret);
    ret = vld1_lane_u8::<6>(src.offset(vget_lane_s16::<2>(vget_high_s16(indices)) as isize), ret);
    ret = vld1_lane_u8::<7>(src.offset(vget_lane_s16::<3>(vget_high_s16(indices)) as isize), ret);
    ret
}

// The `lane` parameter here must be an immediate.
// #define store_u8_2x1_lane(dst, src, lane)
#[inline(always)]
unsafe fn store_u8_2x1_lane<const LANE: i32>(dst: *mut u8, src: uint8x8_t) {
    let a: u16 = vget_lane_u16::<LANE>(vreinterpret_u16_u8(src));
    ptr::write_unaligned(dst as *mut u16, a);
}

// #define store_u8_4x1_lane(dst, src, lane)
#[inline(always)]
unsafe fn store_u8_4x1_lane<const LANE: i32>(dst: *mut u8, src: uint8x8_t) {
    let a: u32 = vget_lane_u32::<LANE>(vreinterpret_u32_u8(src));
    ptr::write_unaligned(dst as *mut u32, a);
}

// #define store_u16_2x1_lane(dst, src, lane)
#[inline(always)]
unsafe fn store_u16_2x1_lane<const LANE: i32>(dst: *mut u16, src: uint16x4_t) {
    let a: u32 = vget_lane_u32::<LANE>(vreinterpret_u32_u16(src));
    ptr::write_unaligned(dst as *mut u32, a);
}

// #define store_u16_4x1_lane(dst, src, lane)
#[inline(always)]
unsafe fn store_u16_4x1_lane<const LANE: i32>(dst: *mut u16, src: uint16x8_t) {
    let a: u64 = vgetq_lane_u64::<LANE>(vreinterpretq_u64_u16(src));
    ptr::write_unaligned(dst as *mut u64, a);
}

// #define store_s16_4x1_lane(dst, src, lane)
#[inline(always)]
unsafe fn store_s16_4x1_lane<const LANE: i32>(dst: *mut i16, src: int16x8_t) {
    let a: i64 = vgetq_lane_s64::<LANE>(vreinterpretq_s64_s16(src));
    ptr::write_unaligned(dst as *mut i64, a);
}

// Store the low 16-bits from a single vector.
#[inline(always)]
pub unsafe fn store_u8_2x1(dst: *mut u8, src: uint8x8_t) {
    store_u8_2x1_lane::<0>(dst, src);
}

// Store the low 32-bits from a single vector.
#[inline(always)]
pub unsafe fn store_u8_4x1(dst: *mut u8, src: uint8x8_t) {
    store_u8_4x1_lane::<0>(dst, src);
}

// Store two blocks of 16-bits from a single vector.
#[inline(always)]
pub unsafe fn store_u8x2_strided_x2(mut dst: *mut u8, dst_stride: isize, src: uint8x8_t) {
    store_u8_2x1_lane::<0>(dst, src);
    dst = dst.offset(dst_stride);
    store_u8_2x1_lane::<1>(dst, src);
}

#[inline(always)]
pub unsafe fn store_u8x2_strided_x4(mut dst: *mut u8, dst_stride: isize, src: uint8x8_t) {
    store_u8_2x1_lane::<0>(dst, src);
    dst = dst.offset(dst_stride);
    store_u8_2x1_lane::<1>(dst, src);
    dst = dst.offset(dst_stride);
    store_u8_2x1_lane::<2>(dst, src);
    dst = dst.offset(dst_stride);
    store_u8_2x1_lane::<3>(dst, src);
}

// Store two blocks of 32-bits from a single vector.
#[inline(always)]
pub unsafe fn store_u8x4_strided_x2(mut dst: *mut u8, stride: isize, src: uint8x8_t) {
    store_u8_4x1_lane::<0>(dst, src);
    dst = dst.offset(stride);
    store_u8_4x1_lane::<1>(dst, src);
}

// Store four blocks of 32-bits from a single vector.
#[inline(always)]
pub unsafe fn store_u8x4_strided_x4(mut dst: *mut u8, stride: isize, src: uint8x16_t) {
    store_u8_4x1_lane::<0>(dst, vget_low_u8(src));
    dst = dst.offset(stride);
    store_u8_4x1_lane::<1>(dst, vget_low_u8(src));
    dst = dst.offset(stride);
    store_u8_4x1_lane::<0>(dst, vget_high_u8(src));
    dst = dst.offset(stride);
    store_u8_4x1_lane::<1>(dst, vget_high_u8(src));
}

// Store the low 32-bits from a single vector.
#[inline(always)]
pub unsafe fn store_u16_2x1(dst: *mut u16, src: uint16x4_t) {
    store_u16_2x1_lane::<0>(dst, src);
}

// Store two blocks of 32-bits from a single vector.
#[inline(always)]
pub unsafe fn store_u16x2_strided_x2(mut dst: *mut u16, dst_stride: isize, src: uint16x4_t) {
    store_u16_2x1_lane::<0>(dst, src);
    dst = dst.offset(dst_stride);
    store_u16_2x1_lane::<1>(dst, src);
}

// Store two blocks of 64-bits from a single vector.
#[inline(always)]
pub unsafe fn store_u16x4_strided_x2(mut dst: *mut u16, dst_stride: isize, src: uint16x8_t) {
    store_u16_4x1_lane::<0>(dst, src);
    dst = dst.offset(dst_stride);
    store_u16_4x1_lane::<1>(dst, src);
}

// Store two blocks of 64-bits from a single vector.
#[inline(always)]
pub unsafe fn store_s16x4_strided_x2(mut dst: *mut i16, dst_stride: isize, src: int16x8_t) {
    store_s16_4x1_lane::<0>(dst, src);
    dst = dst.offset(dst_stride);
    store_s16_4x1_lane::<1>(dst, src);
}
