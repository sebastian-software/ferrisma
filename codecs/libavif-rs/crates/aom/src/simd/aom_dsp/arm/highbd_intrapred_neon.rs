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

// Translated by hand from upstream/aom/aom_dsp/arm/highbd_intrapred_neon.c
// (ADR 0056). Intrinsic sequences, helper structure, and names follow the
// original; only the AOM_ARCH_AARCH64 branches are retained, and the
// `!CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER` branches are taken (the port
// builds the encoder and decoder with CONFIG_REALTIME_ONLY=0).
//
// Translation notes:
// - The function-generating C macros become macro_rules! that receive the
//   generated function and helper names explicitly.
// - `aom_memset16` (aom_mem/aom_mem.h), `get_msb` (aom_ports/bitops.h), and
//   `smooth_weights_u16` (aom_dsp/intrapred_common.h) are header-local in the
//   original and are transcribed below.
// - The variable-length `left_data[bh + 1]` array of the zone 2 predictors
//   becomes a fixed-size array of the same length.
use core::arch::aarch64::*;

use crate::simd::aom_dsp__arm__mem_neon::*;
use crate::simd::aom_dsp__arm__sum_neon::*;
use crate::simd::aom_dsp__arm__transpose_neon::*;

// Transcribed from upstream/aom/aom_dsp/intrapred_common.h.
const SMOOTH_WEIGHT_LOG2_SCALE: i32 = 8;

// Transcribed from upstream/aom/aom_dsp/intrapred_common.h (header-local
// `static const` table).
#[rustfmt::skip]
static smooth_weights_u16: [u16; 124] = [
    // block dimension = 4
    255, 149, 85, 64,
    // block dimension = 8
    255, 197, 146, 105, 73, 50, 37, 32,
    // block dimension = 16
    255, 225, 196, 170, 145, 123, 102, 84, 68, 54, 43, 33, 26, 20, 17, 16,
    // block dimension = 32
    255, 240, 225, 210, 196, 182, 169, 157, 145, 133, 122, 111, 101, 92, 83, 74,
    66, 59, 52, 45, 39, 34, 29, 25, 21, 17, 14, 12, 10, 9, 8, 8,
    // block dimension = 64
    255, 248, 240, 233, 225, 218, 210, 203, 196, 189, 182, 176, 169, 163, 156,
    150, 144, 138, 133, 127, 121, 116, 111, 106, 101, 96, 91, 86, 82, 77, 73, 69,
    65, 61, 57, 54, 50, 47, 44, 41, 38, 35, 32, 29, 27, 25, 22, 20, 18, 16, 15,
    13, 12, 10, 9, 8, 7, 6, 6, 5, 5, 4, 4, 4,
];

// Transcribed from upstream/aom/aom_mem/aom_mem.h.
#[inline(always)]
unsafe fn aom_memset16(dest: *mut u16, val: i32, length: usize) {
    let mut dest16 = dest;
    for _ in 0..length {
        *dest16 = val as u16;
        dest16 = dest16.add(1);
    }
}

// Transcribed from upstream/aom/aom_ports/bitops.h (GNU builtin branch).
#[inline(always)]
fn get_msb(n: u32) -> i32 {
    debug_assert!(n != 0);
    (31 ^ n.leading_zeros()) as i32
}

// -----------------------------------------------------------------------------
// DC

#[inline(always)]
unsafe fn highbd_dc_store_4xh(dst: *mut u16, stride: isize, h: i32, dc: uint16x4_t) {
    for i in 0..h {
        vst1_u16(dst.offset(i as isize * stride), dc);
    }
}

#[inline(always)]
unsafe fn highbd_dc_store_8xh(dst: *mut u16, stride: isize, h: i32, dc: uint16x8_t) {
    for i in 0..h {
        vst1q_u16(dst.offset(i as isize * stride), dc);
    }
}

#[inline(always)]
unsafe fn highbd_dc_store_16xh(dst: *mut u16, stride: isize, h: i32, dc: uint16x8_t) {
    for i in 0..h {
        vst1q_u16(dst.offset(i as isize * stride), dc);
        vst1q_u16(dst.offset(i as isize * stride + 8), dc);
    }
}

#[inline(always)]
unsafe fn highbd_dc_store_32xh(dst: *mut u16, stride: isize, h: i32, dc: uint16x8_t) {
    for i in 0..h {
        vst1q_u16(dst.offset(i as isize * stride), dc);
        vst1q_u16(dst.offset(i as isize * stride + 8), dc);
        vst1q_u16(dst.offset(i as isize * stride + 16), dc);
        vst1q_u16(dst.offset(i as isize * stride + 24), dc);
    }
}

#[inline(always)]
unsafe fn highbd_dc_store_64xh(dst: *mut u16, stride: isize, h: i32, dc: uint16x8_t) {
    for i in 0..h {
        vst1q_u16(dst.offset(i as isize * stride), dc);
        vst1q_u16(dst.offset(i as isize * stride + 8), dc);
        vst1q_u16(dst.offset(i as isize * stride + 16), dc);
        vst1q_u16(dst.offset(i as isize * stride + 24), dc);
        vst1q_u16(dst.offset(i as isize * stride + 32), dc);
        vst1q_u16(dst.offset(i as isize * stride + 40), dc);
        vst1q_u16(dst.offset(i as isize * stride + 48), dc);
        vst1q_u16(dst.offset(i as isize * stride + 56), dc);
    }
}

#[inline(always)]
unsafe fn horizontal_add_and_broadcast_long_u16x8(a: uint16x8_t) -> uint32x4_t {
    // Need to assume input is up to 16 bits wide from dc 64x64 partial sum, so
    // promote first.
    let b = vpaddlq_u16(a);
    let c = vpaddq_u32(b, b);
    vpaddq_u32(c, c)
}

#[inline(always)]
unsafe fn highbd_dc_load_partial_sum_4(left: *const u16) -> uint16x8_t {
    // Nothing to do since sum is already one vector, but saves needing to
    // special case w=4 or h=4 cases. The combine will be zero cost for a sane
    // compiler since vld1 already sets the top half of a vector to zero as part
    // of the operation.
    vcombine_u16(vld1_u16(left), vdup_n_u16(0))
}

#[inline(always)]
unsafe fn highbd_dc_load_partial_sum_8(left: *const u16) -> uint16x8_t {
    // Nothing to do since sum is already one vector, but saves needing to
    // special case w=8 or h=8 cases.
    vld1q_u16(left)
}

#[inline(always)]
unsafe fn highbd_dc_load_partial_sum_16(left: *const u16) -> uint16x8_t {
    let a0 = vld1q_u16(left.add(0)); // up to 12 bits
    let a1 = vld1q_u16(left.add(8));
    vaddq_u16(a0, a1) // up to 13 bits
}

#[inline(always)]
unsafe fn highbd_dc_load_partial_sum_32(left: *const u16) -> uint16x8_t {
    let a0 = vld1q_u16(left.add(0)); // up to 12 bits
    let a1 = vld1q_u16(left.add(8));
    let a2 = vld1q_u16(left.add(16));
    let a3 = vld1q_u16(left.add(24));
    let b0 = vaddq_u16(a0, a1); // up to 13 bits
    let b1 = vaddq_u16(a2, a3);
    vaddq_u16(b0, b1) // up to 14 bits
}

#[inline(always)]
unsafe fn highbd_dc_load_partial_sum_64(left: *const u16) -> uint16x8_t {
    let a0 = vld1q_u16(left.add(0)); // up to 12 bits
    let a1 = vld1q_u16(left.add(8));
    let a2 = vld1q_u16(left.add(16));
    let a3 = vld1q_u16(left.add(24));
    let a4 = vld1q_u16(left.add(32));
    let a5 = vld1q_u16(left.add(40));
    let a6 = vld1q_u16(left.add(48));
    let a7 = vld1q_u16(left.add(56));
    let b0 = vaddq_u16(a0, a1); // up to 13 bits
    let b1 = vaddq_u16(a2, a3);
    let b2 = vaddq_u16(a4, a5);
    let b3 = vaddq_u16(a6, a7);
    let c0 = vaddq_u16(b0, b1); // up to 14 bits
    let c1 = vaddq_u16(b2, b3);
    vaddq_u16(c0, c1) // up to 15 bits
}

// #define HIGHBD_DC_PREDICTOR(w, h, shift)
macro_rules! highbd_dc_predictor {
    ($name:ident, $load_w:ident, $load_h:ident, $store:ident, $h:literal, $shift:literal) => {
        #[no_mangle]
        pub unsafe extern "C-unwind" fn $name(
            dst: *mut u16,
            stride: isize,
            above: *const u16,
            left: *const u16,
            _bd: i32,
        ) {
            let a = $load_w(above);
            let l = $load_h(left);
            let sum = horizontal_add_and_broadcast_long_u16x8(vaddq_u16(a, l));
            let dc0 = vrshrn_n_u32::<$shift>(sum);
            $store(dst, stride, $h, vdupq_lane_u16::<0>(dc0));
        }
    };
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_dc_predictor_4x4_neon(
    dst: *mut u16,
    stride: isize,
    above: *const u16,
    left: *const u16,
    _bd: i32,
) {
    // In the rectangular cases we simply extend the shorter vector to uint16x8
    // in order to accumulate, however in the 4x4 case there is no shorter vector
    // to extend so it is beneficial to do the whole calculation in uint16x4
    // instead.
    let a = vld1_u16(above); // up to 12 bits
    let l = vld1_u16(left);
    let mut sum = vpadd_u16(a, l); // up to 13 bits
    sum = vpadd_u16(sum, sum); // up to 14 bits
    sum = vpadd_u16(sum, sum);
    let dc = vrshr_n_u16::<3>(sum);
    highbd_dc_store_4xh(dst, stride, 4, dc);
}

highbd_dc_predictor!(aom_highbd_dc_predictor_8x8_neon, highbd_dc_load_partial_sum_8, highbd_dc_load_partial_sum_8, highbd_dc_store_8xh, 8, 4);
highbd_dc_predictor!(aom_highbd_dc_predictor_16x16_neon, highbd_dc_load_partial_sum_16, highbd_dc_load_partial_sum_16, highbd_dc_store_16xh, 16, 5);
highbd_dc_predictor!(aom_highbd_dc_predictor_32x32_neon, highbd_dc_load_partial_sum_32, highbd_dc_load_partial_sum_32, highbd_dc_store_32xh, 32, 6);
highbd_dc_predictor!(aom_highbd_dc_predictor_64x64_neon, highbd_dc_load_partial_sum_64, highbd_dc_load_partial_sum_64, highbd_dc_store_64xh, 64, 7);

#[inline(always)]
fn divide_using_multiply_shift(num: i32, shift1: i32, multiplier: i32, shift2: i32) -> i32 {
    let interm = num >> shift1;
    interm.wrapping_mul(multiplier) >> shift2
}

const HIGHBD_DC_MULTIPLIER_1X2: u32 = 0xAAAB;
const HIGHBD_DC_MULTIPLIER_1X4: u32 = 0x6667;
const HIGHBD_DC_SHIFT2: i32 = 17;

#[inline(always)]
fn highbd_dc_predictor_rect(bw: i32, bh: i32, sum: i32, shift1: i32, multiplier: u32) -> i32 {
    divide_using_multiply_shift(sum + ((bw + bh) >> 1), shift1, multiplier as i32, HIGHBD_DC_SHIFT2)
}

// #define HIGHBD_DC_PREDICTOR_RECT(w, h, q, shift, mult)
macro_rules! highbd_dc_predictor_rect {
    ($name:ident, $load_w:ident, $load_h:ident, $store:ident, $dup:ident, $w:literal, $h:literal,
     $shift:literal, $mult:ident) => {
        #[no_mangle]
        pub unsafe extern "C-unwind" fn $name(
            dst: *mut u16,
            stride: isize,
            above: *const u16,
            left: *const u16,
            _bd: i32,
        ) {
            let sum_above = $load_w(above);
            let sum_left = $load_h(left);
            let sum_vec = vaddq_u16(sum_left, sum_above);
            let sum = horizontal_add_u16x8(sum_vec) as i32;
            let dc0 = highbd_dc_predictor_rect($w, $h, sum, $shift, $mult);
            $store(dst, stride, $h, $dup(dc0 as u16));
        }
    };
}

highbd_dc_predictor_rect!(aom_highbd_dc_predictor_4x8_neon, highbd_dc_load_partial_sum_4, highbd_dc_load_partial_sum_8, highbd_dc_store_4xh, vdup_n_u16, 4, 8, 2, HIGHBD_DC_MULTIPLIER_1X2);
highbd_dc_predictor_rect!(aom_highbd_dc_predictor_4x16_neon, highbd_dc_load_partial_sum_4, highbd_dc_load_partial_sum_16, highbd_dc_store_4xh, vdup_n_u16, 4, 16, 2, HIGHBD_DC_MULTIPLIER_1X4);
highbd_dc_predictor_rect!(aom_highbd_dc_predictor_8x4_neon, highbd_dc_load_partial_sum_8, highbd_dc_load_partial_sum_4, highbd_dc_store_8xh, vdupq_n_u16, 8, 4, 2, HIGHBD_DC_MULTIPLIER_1X2);
highbd_dc_predictor_rect!(aom_highbd_dc_predictor_8x16_neon, highbd_dc_load_partial_sum_8, highbd_dc_load_partial_sum_16, highbd_dc_store_8xh, vdupq_n_u16, 8, 16, 3, HIGHBD_DC_MULTIPLIER_1X2);
highbd_dc_predictor_rect!(aom_highbd_dc_predictor_8x32_neon, highbd_dc_load_partial_sum_8, highbd_dc_load_partial_sum_32, highbd_dc_store_8xh, vdupq_n_u16, 8, 32, 3, HIGHBD_DC_MULTIPLIER_1X4);
highbd_dc_predictor_rect!(aom_highbd_dc_predictor_16x4_neon, highbd_dc_load_partial_sum_16, highbd_dc_load_partial_sum_4, highbd_dc_store_16xh, vdupq_n_u16, 16, 4, 2, HIGHBD_DC_MULTIPLIER_1X4);
highbd_dc_predictor_rect!(aom_highbd_dc_predictor_16x8_neon, highbd_dc_load_partial_sum_16, highbd_dc_load_partial_sum_8, highbd_dc_store_16xh, vdupq_n_u16, 16, 8, 3, HIGHBD_DC_MULTIPLIER_1X2);
highbd_dc_predictor_rect!(aom_highbd_dc_predictor_16x32_neon, highbd_dc_load_partial_sum_16, highbd_dc_load_partial_sum_32, highbd_dc_store_16xh, vdupq_n_u16, 16, 32, 4, HIGHBD_DC_MULTIPLIER_1X2);
highbd_dc_predictor_rect!(aom_highbd_dc_predictor_16x64_neon, highbd_dc_load_partial_sum_16, highbd_dc_load_partial_sum_64, highbd_dc_store_16xh, vdupq_n_u16, 16, 64, 4, HIGHBD_DC_MULTIPLIER_1X4);
highbd_dc_predictor_rect!(aom_highbd_dc_predictor_32x8_neon, highbd_dc_load_partial_sum_32, highbd_dc_load_partial_sum_8, highbd_dc_store_32xh, vdupq_n_u16, 32, 8, 3, HIGHBD_DC_MULTIPLIER_1X4);
highbd_dc_predictor_rect!(aom_highbd_dc_predictor_32x16_neon, highbd_dc_load_partial_sum_32, highbd_dc_load_partial_sum_16, highbd_dc_store_32xh, vdupq_n_u16, 32, 16, 4, HIGHBD_DC_MULTIPLIER_1X2);
highbd_dc_predictor_rect!(aom_highbd_dc_predictor_32x64_neon, highbd_dc_load_partial_sum_32, highbd_dc_load_partial_sum_64, highbd_dc_store_32xh, vdupq_n_u16, 32, 64, 5, HIGHBD_DC_MULTIPLIER_1X2);
highbd_dc_predictor_rect!(aom_highbd_dc_predictor_64x16_neon, highbd_dc_load_partial_sum_64, highbd_dc_load_partial_sum_16, highbd_dc_store_64xh, vdupq_n_u16, 64, 16, 4, HIGHBD_DC_MULTIPLIER_1X4);
highbd_dc_predictor_rect!(aom_highbd_dc_predictor_64x32_neon, highbd_dc_load_partial_sum_64, highbd_dc_load_partial_sum_32, highbd_dc_store_64xh, vdupq_n_u16, 64, 32, 5, HIGHBD_DC_MULTIPLIER_1X2);

// -----------------------------------------------------------------------------
// DC_128

// #define HIGHBD_DC_PREDICTOR_128(w, h, q)
macro_rules! highbd_dc_predictor_128 {
    ($name:ident, $store:ident, $dup:ident, $h:literal) => {
        #[no_mangle]
        pub unsafe extern "C-unwind" fn $name(
            dst: *mut u16,
            stride: isize,
            _above: *const u16,
            _left: *const u16,
            bd: i32,
        ) {
            $store(dst, stride, $h, $dup((0x80 << (bd - 8)) as u16));
        }
    };
}

highbd_dc_predictor_128!(aom_highbd_dc_128_predictor_4x4_neon, highbd_dc_store_4xh, vdup_n_u16, 4);
highbd_dc_predictor_128!(aom_highbd_dc_128_predictor_4x8_neon, highbd_dc_store_4xh, vdup_n_u16, 8);
highbd_dc_predictor_128!(aom_highbd_dc_128_predictor_4x16_neon, highbd_dc_store_4xh, vdup_n_u16, 16);
highbd_dc_predictor_128!(aom_highbd_dc_128_predictor_8x4_neon, highbd_dc_store_8xh, vdupq_n_u16, 4);
highbd_dc_predictor_128!(aom_highbd_dc_128_predictor_8x8_neon, highbd_dc_store_8xh, vdupq_n_u16, 8);
highbd_dc_predictor_128!(aom_highbd_dc_128_predictor_8x16_neon, highbd_dc_store_8xh, vdupq_n_u16, 16);
highbd_dc_predictor_128!(aom_highbd_dc_128_predictor_8x32_neon, highbd_dc_store_8xh, vdupq_n_u16, 32);
highbd_dc_predictor_128!(aom_highbd_dc_128_predictor_16x4_neon, highbd_dc_store_16xh, vdupq_n_u16, 4);
highbd_dc_predictor_128!(aom_highbd_dc_128_predictor_16x8_neon, highbd_dc_store_16xh, vdupq_n_u16, 8);
highbd_dc_predictor_128!(aom_highbd_dc_128_predictor_16x16_neon, highbd_dc_store_16xh, vdupq_n_u16, 16);
highbd_dc_predictor_128!(aom_highbd_dc_128_predictor_16x32_neon, highbd_dc_store_16xh, vdupq_n_u16, 32);
highbd_dc_predictor_128!(aom_highbd_dc_128_predictor_16x64_neon, highbd_dc_store_16xh, vdupq_n_u16, 64);
highbd_dc_predictor_128!(aom_highbd_dc_128_predictor_32x8_neon, highbd_dc_store_32xh, vdupq_n_u16, 8);
highbd_dc_predictor_128!(aom_highbd_dc_128_predictor_32x16_neon, highbd_dc_store_32xh, vdupq_n_u16, 16);
highbd_dc_predictor_128!(aom_highbd_dc_128_predictor_32x32_neon, highbd_dc_store_32xh, vdupq_n_u16, 32);
highbd_dc_predictor_128!(aom_highbd_dc_128_predictor_32x64_neon, highbd_dc_store_32xh, vdupq_n_u16, 64);
highbd_dc_predictor_128!(aom_highbd_dc_128_predictor_64x16_neon, highbd_dc_store_64xh, vdupq_n_u16, 16);
highbd_dc_predictor_128!(aom_highbd_dc_128_predictor_64x32_neon, highbd_dc_store_64xh, vdupq_n_u16, 32);
highbd_dc_predictor_128!(aom_highbd_dc_128_predictor_64x64_neon, highbd_dc_store_64xh, vdupq_n_u16, 64);

// -----------------------------------------------------------------------------
// DC_LEFT

#[inline(always)]
unsafe fn highbd_dc_load_sum_4(left: *const u16) -> uint32x4_t {
    let a = vld1_u16(left); // up to 12 bits
    let b = vpadd_u16(a, a); // up to 13 bits
    vcombine_u32(vpaddl_u16(b), vdup_n_u32(0))
}

#[inline(always)]
unsafe fn highbd_dc_load_sum_8(left: *const u16) -> uint32x4_t {
    horizontal_add_and_broadcast_long_u16x8(vld1q_u16(left))
}

#[inline(always)]
unsafe fn highbd_dc_load_sum_16(left: *const u16) -> uint32x4_t {
    horizontal_add_and_broadcast_long_u16x8(highbd_dc_load_partial_sum_16(left))
}

#[inline(always)]
unsafe fn highbd_dc_load_sum_32(left: *const u16) -> uint32x4_t {
    horizontal_add_and_broadcast_long_u16x8(highbd_dc_load_partial_sum_32(left))
}

#[inline(always)]
unsafe fn highbd_dc_load_sum_64(left: *const u16) -> uint32x4_t {
    horizontal_add_and_broadcast_long_u16x8(highbd_dc_load_partial_sum_64(left))
}

// #define DC_PREDICTOR_LEFT(w, h, shift, q)
macro_rules! dc_predictor_left {
    ($name:ident, $load_h:ident, $store:ident, $dup_lane:ident, $h:literal, $shift:literal) => {
        #[no_mangle]
        pub unsafe extern "C-unwind" fn $name(
            dst: *mut u16,
            stride: isize,
            _above: *const u16,
            left: *const u16,
            _bd: i32,
        ) {
            let sum = $load_h(left);
            let dc0 = vrshrn_n_u32::<$shift>(sum);
            $store(dst, stride, $h, $dup_lane::<0>(dc0));
        }
    };
}

dc_predictor_left!(aom_highbd_dc_left_predictor_4x4_neon, highbd_dc_load_sum_4, highbd_dc_store_4xh, vdup_lane_u16, 4, 2);
dc_predictor_left!(aom_highbd_dc_left_predictor_4x8_neon, highbd_dc_load_sum_8, highbd_dc_store_4xh, vdup_lane_u16, 8, 3);
dc_predictor_left!(aom_highbd_dc_left_predictor_4x16_neon, highbd_dc_load_sum_16, highbd_dc_store_4xh, vdup_lane_u16, 16, 4);
dc_predictor_left!(aom_highbd_dc_left_predictor_8x4_neon, highbd_dc_load_sum_4, highbd_dc_store_8xh, vdupq_lane_u16, 4, 2);
dc_predictor_left!(aom_highbd_dc_left_predictor_8x8_neon, highbd_dc_load_sum_8, highbd_dc_store_8xh, vdupq_lane_u16, 8, 3);
dc_predictor_left!(aom_highbd_dc_left_predictor_8x16_neon, highbd_dc_load_sum_16, highbd_dc_store_8xh, vdupq_lane_u16, 16, 4);
dc_predictor_left!(aom_highbd_dc_left_predictor_8x32_neon, highbd_dc_load_sum_32, highbd_dc_store_8xh, vdupq_lane_u16, 32, 5);
dc_predictor_left!(aom_highbd_dc_left_predictor_16x4_neon, highbd_dc_load_sum_4, highbd_dc_store_16xh, vdupq_lane_u16, 4, 2);
dc_predictor_left!(aom_highbd_dc_left_predictor_16x8_neon, highbd_dc_load_sum_8, highbd_dc_store_16xh, vdupq_lane_u16, 8, 3);
dc_predictor_left!(aom_highbd_dc_left_predictor_16x16_neon, highbd_dc_load_sum_16, highbd_dc_store_16xh, vdupq_lane_u16, 16, 4);
dc_predictor_left!(aom_highbd_dc_left_predictor_16x32_neon, highbd_dc_load_sum_32, highbd_dc_store_16xh, vdupq_lane_u16, 32, 5);
dc_predictor_left!(aom_highbd_dc_left_predictor_16x64_neon, highbd_dc_load_sum_64, highbd_dc_store_16xh, vdupq_lane_u16, 64, 6);
dc_predictor_left!(aom_highbd_dc_left_predictor_32x8_neon, highbd_dc_load_sum_8, highbd_dc_store_32xh, vdupq_lane_u16, 8, 3);
dc_predictor_left!(aom_highbd_dc_left_predictor_32x16_neon, highbd_dc_load_sum_16, highbd_dc_store_32xh, vdupq_lane_u16, 16, 4);
dc_predictor_left!(aom_highbd_dc_left_predictor_32x32_neon, highbd_dc_load_sum_32, highbd_dc_store_32xh, vdupq_lane_u16, 32, 5);
dc_predictor_left!(aom_highbd_dc_left_predictor_32x64_neon, highbd_dc_load_sum_64, highbd_dc_store_32xh, vdupq_lane_u16, 64, 6);
dc_predictor_left!(aom_highbd_dc_left_predictor_64x16_neon, highbd_dc_load_sum_16, highbd_dc_store_64xh, vdupq_lane_u16, 16, 4);
dc_predictor_left!(aom_highbd_dc_left_predictor_64x32_neon, highbd_dc_load_sum_32, highbd_dc_store_64xh, vdupq_lane_u16, 32, 5);
dc_predictor_left!(aom_highbd_dc_left_predictor_64x64_neon, highbd_dc_load_sum_64, highbd_dc_store_64xh, vdupq_lane_u16, 64, 6);

// -----------------------------------------------------------------------------
// DC_TOP

// #define DC_PREDICTOR_TOP(w, h, shift, q)
macro_rules! dc_predictor_top {
    ($name:ident, $load_w:ident, $store:ident, $dup_lane:ident, $h:literal, $shift:literal) => {
        #[no_mangle]
        pub unsafe extern "C-unwind" fn $name(
            dst: *mut u16,
            stride: isize,
            above: *const u16,
            _left: *const u16,
            _bd: i32,
        ) {
            let sum = $load_w(above);
            let dc0 = vrshrn_n_u32::<$shift>(sum);
            $store(dst, stride, $h, $dup_lane::<0>(dc0));
        }
    };
}

dc_predictor_top!(aom_highbd_dc_top_predictor_4x4_neon, highbd_dc_load_sum_4, highbd_dc_store_4xh, vdup_lane_u16, 4, 2);
dc_predictor_top!(aom_highbd_dc_top_predictor_4x8_neon, highbd_dc_load_sum_4, highbd_dc_store_4xh, vdup_lane_u16, 8, 2);
dc_predictor_top!(aom_highbd_dc_top_predictor_4x16_neon, highbd_dc_load_sum_4, highbd_dc_store_4xh, vdup_lane_u16, 16, 2);
dc_predictor_top!(aom_highbd_dc_top_predictor_8x4_neon, highbd_dc_load_sum_8, highbd_dc_store_8xh, vdupq_lane_u16, 4, 3);
dc_predictor_top!(aom_highbd_dc_top_predictor_8x8_neon, highbd_dc_load_sum_8, highbd_dc_store_8xh, vdupq_lane_u16, 8, 3);
dc_predictor_top!(aom_highbd_dc_top_predictor_8x16_neon, highbd_dc_load_sum_8, highbd_dc_store_8xh, vdupq_lane_u16, 16, 3);
dc_predictor_top!(aom_highbd_dc_top_predictor_8x32_neon, highbd_dc_load_sum_8, highbd_dc_store_8xh, vdupq_lane_u16, 32, 3);
dc_predictor_top!(aom_highbd_dc_top_predictor_16x4_neon, highbd_dc_load_sum_16, highbd_dc_store_16xh, vdupq_lane_u16, 4, 4);
dc_predictor_top!(aom_highbd_dc_top_predictor_16x8_neon, highbd_dc_load_sum_16, highbd_dc_store_16xh, vdupq_lane_u16, 8, 4);
dc_predictor_top!(aom_highbd_dc_top_predictor_16x16_neon, highbd_dc_load_sum_16, highbd_dc_store_16xh, vdupq_lane_u16, 16, 4);
dc_predictor_top!(aom_highbd_dc_top_predictor_16x32_neon, highbd_dc_load_sum_16, highbd_dc_store_16xh, vdupq_lane_u16, 32, 4);
dc_predictor_top!(aom_highbd_dc_top_predictor_16x64_neon, highbd_dc_load_sum_16, highbd_dc_store_16xh, vdupq_lane_u16, 64, 4);
dc_predictor_top!(aom_highbd_dc_top_predictor_32x8_neon, highbd_dc_load_sum_32, highbd_dc_store_32xh, vdupq_lane_u16, 8, 5);
dc_predictor_top!(aom_highbd_dc_top_predictor_32x16_neon, highbd_dc_load_sum_32, highbd_dc_store_32xh, vdupq_lane_u16, 16, 5);
dc_predictor_top!(aom_highbd_dc_top_predictor_32x32_neon, highbd_dc_load_sum_32, highbd_dc_store_32xh, vdupq_lane_u16, 32, 5);
dc_predictor_top!(aom_highbd_dc_top_predictor_32x64_neon, highbd_dc_load_sum_32, highbd_dc_store_32xh, vdupq_lane_u16, 64, 5);
dc_predictor_top!(aom_highbd_dc_top_predictor_64x16_neon, highbd_dc_load_sum_64, highbd_dc_store_64xh, vdupq_lane_u16, 16, 6);
dc_predictor_top!(aom_highbd_dc_top_predictor_64x32_neon, highbd_dc_load_sum_64, highbd_dc_store_64xh, vdupq_lane_u16, 32, 6);
dc_predictor_top!(aom_highbd_dc_top_predictor_64x64_neon, highbd_dc_load_sum_64, highbd_dc_store_64xh, vdupq_lane_u16, 64, 6);

// -----------------------------------------------------------------------------
// V_PRED

// #define HIGHBD_V_NXM(W, H)
macro_rules! highbd_v_nxm {
    ($name:ident, $vertical:ident, $h:literal) => {
        #[no_mangle]
        pub unsafe extern "C-unwind" fn $name(
            dst: *mut u16,
            stride: isize,
            above: *const u16,
            _left: *const u16,
            _bd: i32,
        ) {
            $vertical(dst, stride, above, $h);
        }
    };
}

#[inline(always)]
unsafe fn load_uint16x8x2(ptr: *const u16) -> uint16x8x2_t {
    // Clang/gcc uses ldp here.
    uint16x8x2_t(vld1q_u16(ptr), vld1q_u16(ptr.add(8)))
}

#[inline(always)]
unsafe fn store_uint16x8x2(ptr: *mut u16, x: uint16x8x2_t) {
    vst1q_u16(ptr, x.0);
    vst1q_u16(ptr.add(8), x.1);
}

#[inline(always)]
unsafe fn vertical4xh_neon(mut dst: *mut u16, stride: isize, above: *const u16, height: i32) {
    let row = vld1_u16(above);
    let mut y = height;
    loop {
        vst1_u16(dst, row);
        vst1_u16(dst.offset(stride), row);
        dst = dst.offset(stride << 1);
        y -= 2;
        if y == 0 {
            break;
        }
    }
}

#[inline(always)]
unsafe fn vertical8xh_neon(mut dst: *mut u16, stride: isize, above: *const u16, height: i32) {
    let row = vld1q_u16(above);
    let mut y = height;
    loop {
        vst1q_u16(dst, row);
        vst1q_u16(dst.offset(stride), row);
        dst = dst.offset(stride << 1);
        y -= 2;
        if y == 0 {
            break;
        }
    }
}

#[inline(always)]
unsafe fn vertical16xh_neon(mut dst: *mut u16, stride: isize, above: *const u16, height: i32) {
    let row = load_uint16x8x2(above);
    let mut y = height;
    loop {
        store_uint16x8x2(dst, row);
        store_uint16x8x2(dst.offset(stride), row);
        dst = dst.offset(stride << 1);
        y -= 2;
        if y == 0 {
            break;
        }
    }
}

#[inline(always)]
unsafe fn load_uint16x8x4(ptr: *const u16) -> uint16x8x4_t {
    // Clang/gcc uses ldp here.
    uint16x8x4_t(vld1q_u16(ptr), vld1q_u16(ptr.add(8)), vld1q_u16(ptr.add(16)), vld1q_u16(ptr.add(24)))
}

#[inline(always)]
unsafe fn store_uint16x8x4(ptr: *mut u16, x: uint16x8x4_t) {
    vst1q_u16(ptr, x.0);
    vst1q_u16(ptr.add(8), x.1);
    vst1q_u16(ptr.add(16), x.2);
    vst1q_u16(ptr.add(24), x.3);
}

#[inline(always)]
unsafe fn vertical32xh_neon(mut dst: *mut u16, stride: isize, above: *const u16, height: i32) {
    let row = load_uint16x8x4(above);
    let mut y = height;
    loop {
        store_uint16x8x4(dst, row);
        store_uint16x8x4(dst.offset(stride), row);
        dst = dst.offset(stride << 1);
        y -= 2;
        if y == 0 {
            break;
        }
    }
}

#[inline(always)]
unsafe fn vertical64xh_neon(mut dst: *mut u16, stride: isize, above: *const u16, height: i32) {
    let mut dst32 = dst.add(32);
    let row = load_uint16x8x4(above);
    let row32 = load_uint16x8x4(above.add(32));
    let mut y = height;
    loop {
        store_uint16x8x4(dst, row);
        store_uint16x8x4(dst32, row32);
        store_uint16x8x4(dst.offset(stride), row);
        store_uint16x8x4(dst32.offset(stride), row32);
        dst = dst.offset(stride << 1);
        dst32 = dst32.offset(stride << 1);
        y -= 2;
        if y == 0 {
            break;
        }
    }
}

highbd_v_nxm!(aom_highbd_v_predictor_4x4_neon, vertical4xh_neon, 4);
highbd_v_nxm!(aom_highbd_v_predictor_4x8_neon, vertical4xh_neon, 8);
highbd_v_nxm!(aom_highbd_v_predictor_4x16_neon, vertical4xh_neon, 16);
highbd_v_nxm!(aom_highbd_v_predictor_8x4_neon, vertical8xh_neon, 4);
highbd_v_nxm!(aom_highbd_v_predictor_8x8_neon, vertical8xh_neon, 8);
highbd_v_nxm!(aom_highbd_v_predictor_8x16_neon, vertical8xh_neon, 16);
highbd_v_nxm!(aom_highbd_v_predictor_8x32_neon, vertical8xh_neon, 32);
highbd_v_nxm!(aom_highbd_v_predictor_16x4_neon, vertical16xh_neon, 4);
highbd_v_nxm!(aom_highbd_v_predictor_16x8_neon, vertical16xh_neon, 8);
highbd_v_nxm!(aom_highbd_v_predictor_16x16_neon, vertical16xh_neon, 16);
highbd_v_nxm!(aom_highbd_v_predictor_16x32_neon, vertical16xh_neon, 32);
highbd_v_nxm!(aom_highbd_v_predictor_16x64_neon, vertical16xh_neon, 64);
highbd_v_nxm!(aom_highbd_v_predictor_32x8_neon, vertical32xh_neon, 8);
highbd_v_nxm!(aom_highbd_v_predictor_32x16_neon, vertical32xh_neon, 16);
highbd_v_nxm!(aom_highbd_v_predictor_32x32_neon, vertical32xh_neon, 32);
highbd_v_nxm!(aom_highbd_v_predictor_32x64_neon, vertical32xh_neon, 64);
highbd_v_nxm!(aom_highbd_v_predictor_64x16_neon, vertical64xh_neon, 16);
highbd_v_nxm!(aom_highbd_v_predictor_64x32_neon, vertical64xh_neon, 32);
highbd_v_nxm!(aom_highbd_v_predictor_64x64_neon, vertical64xh_neon, 64);

// -----------------------------------------------------------------------------
// H_PRED

#[inline(always)]
unsafe fn highbd_h_store_4x4(dst: *mut u16, stride: isize, left: uint16x4_t) {
    vst1_u16(dst.offset(0 * stride), vdup_lane_u16::<0>(left));
    vst1_u16(dst.offset(1 * stride), vdup_lane_u16::<1>(left));
    vst1_u16(dst.offset(2 * stride), vdup_lane_u16::<2>(left));
    vst1_u16(dst.offset(3 * stride), vdup_lane_u16::<3>(left));
}

#[inline(always)]
unsafe fn highbd_h_store_8x4(dst: *mut u16, stride: isize, left: uint16x4_t) {
    vst1q_u16(dst.offset(0 * stride), vdupq_lane_u16::<0>(left));
    vst1q_u16(dst.offset(1 * stride), vdupq_lane_u16::<1>(left));
    vst1q_u16(dst.offset(2 * stride), vdupq_lane_u16::<2>(left));
    vst1q_u16(dst.offset(3 * stride), vdupq_lane_u16::<3>(left));
}

#[inline(always)]
unsafe fn highbd_h_store_16x1(dst: *mut u16, left: uint16x8_t) {
    vst1q_u16(dst.add(0), left);
    vst1q_u16(dst.add(8), left);
}

#[inline(always)]
unsafe fn highbd_h_store_16x4(dst: *mut u16, stride: isize, left: uint16x4_t) {
    highbd_h_store_16x1(dst.offset(0 * stride), vdupq_lane_u16::<0>(left));
    highbd_h_store_16x1(dst.offset(1 * stride), vdupq_lane_u16::<1>(left));
    highbd_h_store_16x1(dst.offset(2 * stride), vdupq_lane_u16::<2>(left));
    highbd_h_store_16x1(dst.offset(3 * stride), vdupq_lane_u16::<3>(left));
}

#[inline(always)]
unsafe fn highbd_h_store_32x1(dst: *mut u16, left: uint16x8_t) {
    vst1q_u16(dst.add(0), left);
    vst1q_u16(dst.add(8), left);
    vst1q_u16(dst.add(16), left);
    vst1q_u16(dst.add(24), left);
}

#[inline(always)]
unsafe fn highbd_h_store_32x4(dst: *mut u16, stride: isize, left: uint16x4_t) {
    highbd_h_store_32x1(dst.offset(0 * stride), vdupq_lane_u16::<0>(left));
    highbd_h_store_32x1(dst.offset(1 * stride), vdupq_lane_u16::<1>(left));
    highbd_h_store_32x1(dst.offset(2 * stride), vdupq_lane_u16::<2>(left));
    highbd_h_store_32x1(dst.offset(3 * stride), vdupq_lane_u16::<3>(left));
}

#[inline(always)]
unsafe fn highbd_h_store_64x1(dst: *mut u16, left: uint16x8_t) {
    vst1q_u16(dst.add(0), left);
    vst1q_u16(dst.add(8), left);
    vst1q_u16(dst.add(16), left);
    vst1q_u16(dst.add(24), left);
    vst1q_u16(dst.add(32), left);
    vst1q_u16(dst.add(40), left);
    vst1q_u16(dst.add(48), left);
    vst1q_u16(dst.add(56), left);
}

#[inline(always)]
unsafe fn highbd_h_store_64x4(dst: *mut u16, stride: isize, left: uint16x4_t) {
    highbd_h_store_64x1(dst.offset(0 * stride), vdupq_lane_u16::<0>(left));
    highbd_h_store_64x1(dst.offset(1 * stride), vdupq_lane_u16::<1>(left));
    highbd_h_store_64x1(dst.offset(2 * stride), vdupq_lane_u16::<2>(left));
    highbd_h_store_64x1(dst.offset(3 * stride), vdupq_lane_u16::<3>(left));
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_h_predictor_4x4_neon(
    dst: *mut u16,
    stride: isize,
    _above: *const u16,
    left: *const u16,
    _bd: i32,
) {
    highbd_h_store_4x4(dst, stride, vld1_u16(left));
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_h_predictor_4x8_neon(
    dst: *mut u16,
    stride: isize,
    _above: *const u16,
    left: *const u16,
    _bd: i32,
) {
    let l = vld1q_u16(left);
    highbd_h_store_4x4(dst.offset(0 * stride), stride, vget_low_u16(l));
    highbd_h_store_4x4(dst.offset(4 * stride), stride, vget_high_u16(l));
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_h_predictor_8x4_neon(
    dst: *mut u16,
    stride: isize,
    _above: *const u16,
    left: *const u16,
    _bd: i32,
) {
    highbd_h_store_8x4(dst, stride, vld1_u16(left));
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_h_predictor_8x8_neon(
    dst: *mut u16,
    stride: isize,
    _above: *const u16,
    left: *const u16,
    _bd: i32,
) {
    let l = vld1q_u16(left);
    highbd_h_store_8x4(dst.offset(0 * stride), stride, vget_low_u16(l));
    highbd_h_store_8x4(dst.offset(4 * stride), stride, vget_high_u16(l));
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_h_predictor_16x4_neon(
    dst: *mut u16,
    stride: isize,
    _above: *const u16,
    left: *const u16,
    _bd: i32,
) {
    highbd_h_store_16x4(dst, stride, vld1_u16(left));
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_h_predictor_16x8_neon(
    dst: *mut u16,
    stride: isize,
    _above: *const u16,
    left: *const u16,
    _bd: i32,
) {
    let l = vld1q_u16(left);
    highbd_h_store_16x4(dst.offset(0 * stride), stride, vget_low_u16(l));
    highbd_h_store_16x4(dst.offset(4 * stride), stride, vget_high_u16(l));
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_h_predictor_32x8_neon(
    dst: *mut u16,
    stride: isize,
    _above: *const u16,
    left: *const u16,
    _bd: i32,
) {
    let l = vld1q_u16(left);
    highbd_h_store_32x4(dst.offset(0 * stride), stride, vget_low_u16(l));
    highbd_h_store_32x4(dst.offset(4 * stride), stride, vget_high_u16(l));
}

// For cases where height >= 16 we use pairs of loads to get LDP instructions.
// #define HIGHBD_H_WXH_LARGE(w, h)
macro_rules! highbd_h_wxh_large {
    ($name:ident, $store:ident, $h:literal) => {
        #[no_mangle]
        pub unsafe extern "C-unwind" fn $name(
            mut dst: *mut u16,
            stride: isize,
            _above: *const u16,
            mut left: *const u16,
            _bd: i32,
        ) {
            for _ in 0..($h) / 16 {
                let l0 = vld1q_u16(left.add(0));
                let l1 = vld1q_u16(left.add(8));
                $store(dst.offset(0 * stride), stride, vget_low_u16(l0));
                $store(dst.offset(4 * stride), stride, vget_high_u16(l0));
                $store(dst.offset(8 * stride), stride, vget_low_u16(l1));
                $store(dst.offset(12 * stride), stride, vget_high_u16(l1));
                left = left.wrapping_add(16);
                dst = dst.wrapping_offset(16 * stride);
            }
        }
    };
}

highbd_h_wxh_large!(aom_highbd_h_predictor_4x16_neon, highbd_h_store_4x4, 16);
highbd_h_wxh_large!(aom_highbd_h_predictor_8x16_neon, highbd_h_store_8x4, 16);
highbd_h_wxh_large!(aom_highbd_h_predictor_8x32_neon, highbd_h_store_8x4, 32);
highbd_h_wxh_large!(aom_highbd_h_predictor_16x16_neon, highbd_h_store_16x4, 16);
highbd_h_wxh_large!(aom_highbd_h_predictor_16x32_neon, highbd_h_store_16x4, 32);
highbd_h_wxh_large!(aom_highbd_h_predictor_16x64_neon, highbd_h_store_16x4, 64);
highbd_h_wxh_large!(aom_highbd_h_predictor_32x16_neon, highbd_h_store_32x4, 16);
highbd_h_wxh_large!(aom_highbd_h_predictor_32x32_neon, highbd_h_store_32x4, 32);
highbd_h_wxh_large!(aom_highbd_h_predictor_32x64_neon, highbd_h_store_32x4, 64);
highbd_h_wxh_large!(aom_highbd_h_predictor_64x16_neon, highbd_h_store_64x4, 16);
highbd_h_wxh_large!(aom_highbd_h_predictor_64x32_neon, highbd_h_store_64x4, 32);
highbd_h_wxh_large!(aom_highbd_h_predictor_64x64_neon, highbd_h_store_64x4, 64);

// -----------------------------------------------------------------------------
// PAETH

#[inline(always)]
unsafe fn highbd_paeth_4or8_x_h_neon(
    mut dest: *mut u16,
    stride: isize,
    top_row: *const u16,
    left_column: *const u16,
    width: i32,
    height: i32,
) {
    let top_left = vdupq_n_u16(*top_row.offset(-1));
    let top_left_x2 = vdupq_n_u16((*top_row.offset(-1) as i32 + *top_row.offset(-1) as i32) as u16);
    let top: uint16x8_t;
    if width == 4 {
        top = vcombine_u16(vld1_u16(top_row), vdup_n_u16(0));
    } else {
        // width == 8
        top = vld1q_u16(top_row);
    }

    for y in 0..height {
        let left = vdupq_n_u16(*left_column.offset(y as isize));

        let left_dist = vabdq_u16(top, top_left);
        let top_dist = vabdq_u16(left, top_left);
        let top_left_dist = vabdq_u16(vaddq_u16(top, left), top_left_x2);

        let left_le_top = vcleq_u16(left_dist, top_dist);
        let left_le_top_left = vcleq_u16(left_dist, top_left_dist);
        let top_le_top_left = vcleq_u16(top_dist, top_left_dist);

        // if (left_dist <= top_dist && left_dist <= top_left_dist)
        let left_mask = vandq_u16(left_le_top, left_le_top_left);
        //   dest[x] = left_column[y];
        // Fill all the unused spaces with 'top'. They will be overwritten when
        // the positions for top_left are known.
        let mut result = vbslq_u16(left_mask, left, top);
        // else if (top_dist <= top_left_dist)
        //   dest[x] = top_row[x];
        // Add these values to the mask. They were already set.
        let left_or_top_mask = vorrq_u16(left_mask, top_le_top_left);
        // else
        //   dest[x] = top_left;
        result = vbslq_u16(left_or_top_mask, result, top_left);

        if width == 4 {
            vst1_u16(dest, vget_low_u16(result));
        } else {
            // width == 8
            vst1q_u16(dest, result);
        }
        dest = dest.wrapping_offset(stride);
    }
}

// #define HIGHBD_PAETH_NXM(W, H) and HIGHBD_PAETH_NXM_WIDE(W, H)
macro_rules! highbd_paeth_nxm {
    ($name:ident, $paeth:ident, $w:literal, $h:literal) => {
        #[no_mangle]
        pub unsafe extern "C-unwind" fn $name(
            dst: *mut u16,
            stride: isize,
            above: *const u16,
            left: *const u16,
            _bd: i32,
        ) {
            $paeth(dst, stride, above, left, $w, $h);
        }
    };
}

highbd_paeth_nxm!(aom_highbd_paeth_predictor_4x4_neon, highbd_paeth_4or8_x_h_neon, 4, 4);
highbd_paeth_nxm!(aom_highbd_paeth_predictor_4x8_neon, highbd_paeth_4or8_x_h_neon, 4, 8);
highbd_paeth_nxm!(aom_highbd_paeth_predictor_4x16_neon, highbd_paeth_4or8_x_h_neon, 4, 16);
highbd_paeth_nxm!(aom_highbd_paeth_predictor_8x4_neon, highbd_paeth_4or8_x_h_neon, 8, 4);
highbd_paeth_nxm!(aom_highbd_paeth_predictor_8x8_neon, highbd_paeth_4or8_x_h_neon, 8, 8);
highbd_paeth_nxm!(aom_highbd_paeth_predictor_8x16_neon, highbd_paeth_4or8_x_h_neon, 8, 16);
highbd_paeth_nxm!(aom_highbd_paeth_predictor_8x32_neon, highbd_paeth_4or8_x_h_neon, 8, 32);

// Select the closest values and collect them.
#[inline(always)]
unsafe fn select_paeth(
    top: uint16x8_t,
    left: uint16x8_t,
    top_left: uint16x8_t,
    left_le_top: uint16x8_t,
    left_le_top_left: uint16x8_t,
    top_le_top_left: uint16x8_t,
) -> uint16x8_t {
    // if (left_dist <= top_dist && left_dist <= top_left_dist)
    let left_mask = vandq_u16(left_le_top, left_le_top_left);
    //   dest[x] = left_column[y];
    // Fill all the unused spaces with 'top'. They will be overwritten when
    // the positions for top_left are known.
    let result = vbslq_u16(left_mask, left, top);
    // else if (top_dist <= top_left_dist)
    //   dest[x] = top_row[x];
    // Add these values to the mask. They were already set.
    let left_or_top_mask = vorrq_u16(left_mask, top_le_top_left);
    // else
    //   dest[x] = top_left;
    vbslq_u16(left_or_top_mask, result, top_left)
}

// #define PAETH_PREDICTOR(num)
macro_rules! paeth_predictor {
    ($num:literal, $top:ident, $left:ident, $top_left:ident, $top_left_x2:ident, $top_dist:ident,
     $dest:ident) => {{
        let left_dist = vabdq_u16($top[$num], $top_left);
        let top_left_dist = vabdq_u16(vaddq_u16($top[$num], $left), $top_left_x2);
        let left_le_top = vcleq_u16(left_dist, $top_dist);
        let left_le_top_left = vcleq_u16(left_dist, top_left_dist);
        let top_le_top_left = vcleq_u16($top_dist, top_left_dist);
        let result = select_paeth($top[$num], $left, $top_left, left_le_top, left_le_top_left, top_le_top_left);
        vst1q_u16($dest.add($num * 8), result);
    }};
}

// #define LOAD_TOP_ROW(num)
macro_rules! load_top_row {
    ($top_row:ident, $num:literal) => {
        vld1q_u16($top_row.add($num * 8))
    };
}

#[inline(always)]
unsafe fn highbd_paeth16_plus_x_h_neon(
    mut dest: *mut u16,
    stride: isize,
    top_row: *const u16,
    left_column: *const u16,
    width: i32,
    height: i32,
) {
    let top_left = vdupq_n_u16(*top_row.offset(-1));
    let top_left_x2 = vdupq_n_u16((*top_row.offset(-1) as i32 + *top_row.offset(-1) as i32) as u16);
    let mut top: [uint16x8_t; 8] = [vdupq_n_u16(0); 8];
    top[0] = load_top_row!(top_row, 0);
    top[1] = load_top_row!(top_row, 1);
    if width > 16 {
        top[2] = load_top_row!(top_row, 2);
        top[3] = load_top_row!(top_row, 3);
        if width == 64 {
            top[4] = load_top_row!(top_row, 4);
            top[5] = load_top_row!(top_row, 5);
            top[6] = load_top_row!(top_row, 6);
            top[7] = load_top_row!(top_row, 7);
        }
    }

    for y in 0..height {
        let left = vdupq_n_u16(*left_column.offset(y as isize));
        let top_dist = vabdq_u16(left, top_left);
        paeth_predictor!(0, top, left, top_left, top_left_x2, top_dist, dest);
        paeth_predictor!(1, top, left, top_left, top_left_x2, top_dist, dest);
        if width > 16 {
            paeth_predictor!(2, top, left, top_left, top_left_x2, top_dist, dest);
            paeth_predictor!(3, top, left, top_left, top_left_x2, top_dist, dest);
            if width == 64 {
                paeth_predictor!(4, top, left, top_left, top_left_x2, top_dist, dest);
                paeth_predictor!(5, top, left, top_left, top_left_x2, top_dist, dest);
                paeth_predictor!(6, top, left, top_left, top_left_x2, top_dist, dest);
                paeth_predictor!(7, top, left, top_left, top_left_x2, top_dist, dest);
            }
        }
        dest = dest.wrapping_offset(stride);
    }
}

highbd_paeth_nxm!(aom_highbd_paeth_predictor_16x4_neon, highbd_paeth16_plus_x_h_neon, 16, 4);
highbd_paeth_nxm!(aom_highbd_paeth_predictor_16x8_neon, highbd_paeth16_plus_x_h_neon, 16, 8);
highbd_paeth_nxm!(aom_highbd_paeth_predictor_16x16_neon, highbd_paeth16_plus_x_h_neon, 16, 16);
highbd_paeth_nxm!(aom_highbd_paeth_predictor_16x32_neon, highbd_paeth16_plus_x_h_neon, 16, 32);
highbd_paeth_nxm!(aom_highbd_paeth_predictor_16x64_neon, highbd_paeth16_plus_x_h_neon, 16, 64);
highbd_paeth_nxm!(aom_highbd_paeth_predictor_32x8_neon, highbd_paeth16_plus_x_h_neon, 32, 8);
highbd_paeth_nxm!(aom_highbd_paeth_predictor_32x16_neon, highbd_paeth16_plus_x_h_neon, 32, 16);
highbd_paeth_nxm!(aom_highbd_paeth_predictor_32x32_neon, highbd_paeth16_plus_x_h_neon, 32, 32);
highbd_paeth_nxm!(aom_highbd_paeth_predictor_32x64_neon, highbd_paeth16_plus_x_h_neon, 32, 64);
highbd_paeth_nxm!(aom_highbd_paeth_predictor_64x16_neon, highbd_paeth16_plus_x_h_neon, 64, 16);
highbd_paeth_nxm!(aom_highbd_paeth_predictor_64x32_neon, highbd_paeth16_plus_x_h_neon, 64, 32);
highbd_paeth_nxm!(aom_highbd_paeth_predictor_64x64_neon, highbd_paeth16_plus_x_h_neon, 64, 64);

// -----------------------------------------------------------------------------
// SMOOTH

// 256 - v = vneg_s8(v)
#[inline(always)]
unsafe fn negate_s8(v: uint16x4_t) -> uint16x4_t {
    vreinterpret_u16_s8(vneg_s8(vreinterpret_s8_u16(v)))
}

#[inline(always)]
unsafe fn highbd_smooth_4xh_neon(
    mut dst: *mut u16,
    stride: isize,
    top_row: *const u16,
    left_column: *const u16,
    height: i32,
) {
    let top_right = *top_row.add(3);
    let bottom_left = *left_column.offset(height as isize - 1);
    let weights_y = smooth_weights_u16.as_ptr().offset(height as isize - 4);

    let top_v = vld1_u16(top_row);
    let bottom_left_v = vdup_n_u16(bottom_left);
    let weights_x_v = vld1_u16(smooth_weights_u16.as_ptr());
    let scaled_weights_x = negate_s8(weights_x_v);
    let weighted_tr = vmull_n_u16(scaled_weights_x, top_right);

    for y in 0..height as isize {
        // Each variable in the running summation is named for the last item to be
        // accumulated.
        let weighted_top = vmlal_n_u16(weighted_tr, top_v, *weights_y.offset(y));
        let weighted_left = vmlal_n_u16(weighted_top, weights_x_v, *left_column.offset(y));
        let weighted_bl = vmlal_n_u16(weighted_left, bottom_left_v, (256 - *weights_y.offset(y) as i32) as u16);

        let pred = vrshrn_n_u32::<{ SMOOTH_WEIGHT_LOG2_SCALE + 1 }>(weighted_bl);
        vst1_u16(dst, pred);
        dst = dst.wrapping_offset(stride);
    }
}

// Common code between 8xH and [16|32|64]xH.
#[inline(always)]
unsafe fn highbd_calculate_pred8(
    dst: *mut u16,
    weighted_corners_low: uint32x4_t,
    weighted_corners_high: uint32x4_t,
    top_vals: uint16x4x2_t,
    weights_x: uint16x4x2_t,
    left_y: u16,
    weight_y: u16,
) {
    // Each variable in the running summation is named for the last item to be
    // accumulated.
    let weighted_top_low = vmlal_n_u16(weighted_corners_low, top_vals.0, weight_y);
    let weighted_edges_low = vmlal_n_u16(weighted_top_low, weights_x.0, left_y);

    let pred_low = vrshrn_n_u32::<{ SMOOTH_WEIGHT_LOG2_SCALE + 1 }>(weighted_edges_low);
    vst1_u16(dst, pred_low);

    let weighted_top_high = vmlal_n_u16(weighted_corners_high, top_vals.1, weight_y);
    let weighted_edges_high = vmlal_n_u16(weighted_top_high, weights_x.1, left_y);

    let pred_high = vrshrn_n_u32::<{ SMOOTH_WEIGHT_LOG2_SCALE + 1 }>(weighted_edges_high);
    vst1_u16(dst.add(4), pred_high);
}

unsafe fn highbd_smooth_8xh_neon(
    mut dst: *mut u16,
    stride: isize,
    top_row: *const u16,
    left_column: *const u16,
    height: i32,
) {
    let top_right = *top_row.add(7);
    let bottom_left = *left_column.offset(height as isize - 1);
    let weights_y = smooth_weights_u16.as_ptr().offset(height as isize - 4);

    let top_vals = uint16x4x2_t(vld1_u16(top_row), vld1_u16(top_row.add(4)));
    let bottom_left_v = vdup_n_u16(bottom_left);
    let weights_x = uint16x4x2_t(
        vld1_u16(smooth_weights_u16.as_ptr().add(4)),
        vld1_u16(smooth_weights_u16.as_ptr().add(8)),
    );
    let weighted_tr_low = vmull_n_u16(negate_s8(weights_x.0), top_right);
    let weighted_tr_high = vmull_n_u16(negate_s8(weights_x.1), top_right);

    for y in 0..height as isize {
        let weighted_bl = vmull_n_u16(bottom_left_v, (256 - *weights_y.offset(y) as i32) as u16);
        let weighted_corners_low = vaddq_u32(weighted_bl, weighted_tr_low);
        let weighted_corners_high = vaddq_u32(weighted_bl, weighted_tr_high);
        highbd_calculate_pred8(
            dst,
            weighted_corners_low,
            weighted_corners_high,
            top_vals,
            weights_x,
            *left_column.offset(y),
            *weights_y.offset(y),
        );
        dst = dst.wrapping_offset(stride);
    }
}

// #define HIGHBD_SMOOTH_NXM(W, H), HIGHBD_SMOOTH_NXM_WIDE(W, H), and the
// SMOOTH_V / SMOOTH_H counterparts: forward to the per-width helper.
macro_rules! highbd_smooth_nxm {
    ($name:ident, $smooth:ident, $h:literal) => {
        #[no_mangle]
        pub unsafe extern "C-unwind" fn $name(
            dst: *mut u16,
            y_stride: isize,
            above: *const u16,
            left: *const u16,
            _bd: i32,
        ) {
            $smooth(dst, y_stride, above, left, $h);
        }
    };
}

highbd_smooth_nxm!(aom_highbd_smooth_predictor_4x4_neon, highbd_smooth_4xh_neon, 4);
highbd_smooth_nxm!(aom_highbd_smooth_predictor_4x8_neon, highbd_smooth_4xh_neon, 8);
highbd_smooth_nxm!(aom_highbd_smooth_predictor_8x4_neon, highbd_smooth_8xh_neon, 4);
highbd_smooth_nxm!(aom_highbd_smooth_predictor_8x8_neon, highbd_smooth_8xh_neon, 8);
highbd_smooth_nxm!(aom_highbd_smooth_predictor_4x16_neon, highbd_smooth_4xh_neon, 16);
highbd_smooth_nxm!(aom_highbd_smooth_predictor_8x16_neon, highbd_smooth_8xh_neon, 16);
highbd_smooth_nxm!(aom_highbd_smooth_predictor_8x32_neon, highbd_smooth_8xh_neon, 32);

// For width 16 and above.
// #define HIGHBD_SMOOTH_PREDICTOR(W)
macro_rules! highbd_smooth_predictor {
    ($fn_name:ident, $w:literal) => {
        unsafe fn $fn_name(
            mut dst: *mut u16,
            stride: isize,
            top_row: *const u16,
            left_column: *const u16,
            height: i32,
        ) {
            let top_right = *top_row.add(($w) - 1);
            let bottom_left = *left_column.offset(height as isize - 1);
            let weights_y = smooth_weights_u16.as_ptr().offset(height as isize - 4);

            /* Precompute weighted values that don't vary with |y|. */
            let mut weighted_tr_low: [uint32x4_t; ($w) >> 3] = [vdupq_n_u32(0); ($w) >> 3];
            let mut weighted_tr_high: [uint32x4_t; ($w) >> 3] = [vdupq_n_u32(0); ($w) >> 3];
            for i in 0..(($w) >> 3) {
                let x = i << 3;
                let weights_x_low = vld1_u16(smooth_weights_u16.as_ptr().add(($w) - 4 + x));
                weighted_tr_low[i] = vmull_n_u16(negate_s8(weights_x_low), top_right);
                let weights_x_high = vld1_u16(smooth_weights_u16.as_ptr().add(($w) + x));
                weighted_tr_high[i] = vmull_n_u16(negate_s8(weights_x_high), top_right);
            }

            let bottom_left_v = vdup_n_u16(bottom_left);
            for y in 0..height as isize {
                let weighted_bl = vmull_n_u16(bottom_left_v, (256 - *weights_y.offset(y) as i32) as u16);
                let mut dst_x = dst;
                for i in 0..(($w) >> 3) {
                    let x = i << 3;
                    let top_vals = uint16x4x2_t(vld1_u16(top_row.add(x)), vld1_u16(top_row.add(x + 4)));
                    let weighted_corners_low = vaddq_u32(weighted_bl, weighted_tr_low[i]);
                    let weighted_corners_high = vaddq_u32(weighted_bl, weighted_tr_high[i]);
                    /* Accumulate weighted edge values and store. */
                    let weights_x = uint16x4x2_t(
                        vld1_u16(smooth_weights_u16.as_ptr().add(($w) - 4 + x)),
                        vld1_u16(smooth_weights_u16.as_ptr().add(($w) + x)),
                    );
                    highbd_calculate_pred8(
                        dst_x,
                        weighted_corners_low,
                        weighted_corners_high,
                        top_vals,
                        weights_x,
                        *left_column.offset(y),
                        *weights_y.offset(y),
                    );
                    dst_x = dst_x.add(8);
                }
                dst = dst.wrapping_offset(stride);
            }
        }
    };
}

highbd_smooth_predictor!(highbd_smooth_16xh_neon, 16);
highbd_smooth_predictor!(highbd_smooth_32xh_neon, 32);
highbd_smooth_predictor!(highbd_smooth_64xh_neon, 64);

highbd_smooth_nxm!(aom_highbd_smooth_predictor_16x4_neon, highbd_smooth_16xh_neon, 4);
highbd_smooth_nxm!(aom_highbd_smooth_predictor_16x8_neon, highbd_smooth_16xh_neon, 8);
highbd_smooth_nxm!(aom_highbd_smooth_predictor_16x16_neon, highbd_smooth_16xh_neon, 16);
highbd_smooth_nxm!(aom_highbd_smooth_predictor_16x32_neon, highbd_smooth_16xh_neon, 32);
highbd_smooth_nxm!(aom_highbd_smooth_predictor_16x64_neon, highbd_smooth_16xh_neon, 64);
highbd_smooth_nxm!(aom_highbd_smooth_predictor_32x8_neon, highbd_smooth_32xh_neon, 8);
highbd_smooth_nxm!(aom_highbd_smooth_predictor_32x16_neon, highbd_smooth_32xh_neon, 16);
highbd_smooth_nxm!(aom_highbd_smooth_predictor_32x32_neon, highbd_smooth_32xh_neon, 32);
highbd_smooth_nxm!(aom_highbd_smooth_predictor_32x64_neon, highbd_smooth_32xh_neon, 64);
highbd_smooth_nxm!(aom_highbd_smooth_predictor_64x16_neon, highbd_smooth_64xh_neon, 16);
highbd_smooth_nxm!(aom_highbd_smooth_predictor_64x32_neon, highbd_smooth_64xh_neon, 32);
highbd_smooth_nxm!(aom_highbd_smooth_predictor_64x64_neon, highbd_smooth_64xh_neon, 64);

unsafe fn highbd_smooth_v_4xh_neon(
    mut dst: *mut u16,
    stride: isize,
    top_row: *const u16,
    left_column: *const u16,
    height: i32,
) {
    let bottom_left = *left_column.offset(height as isize - 1);
    let weights_y = smooth_weights_u16.as_ptr().offset(height as isize - 4);

    let top_v = vld1_u16(top_row);
    let bottom_left_v = vdup_n_u16(bottom_left);

    for y in 0..height as isize {
        let weighted_bl = vmull_n_u16(bottom_left_v, (256 - *weights_y.offset(y) as i32) as u16);
        let weighted_top = vmlal_n_u16(weighted_bl, top_v, *weights_y.offset(y));
        vst1_u16(dst, vrshrn_n_u32::<SMOOTH_WEIGHT_LOG2_SCALE>(weighted_top));

        dst = dst.wrapping_offset(stride);
    }
}

unsafe fn highbd_smooth_v_8xh_neon(
    mut dst: *mut u16,
    stride: isize,
    top_row: *const u16,
    left_column: *const u16,
    height: i32,
) {
    let bottom_left = *left_column.offset(height as isize - 1);
    let weights_y = smooth_weights_u16.as_ptr().offset(height as isize - 4);

    let top_low = vld1_u16(top_row);
    let top_high = vld1_u16(top_row.add(4));
    let bottom_left_v = vdup_n_u16(bottom_left);

    for y in 0..height as isize {
        let weighted_bl = vmull_n_u16(bottom_left_v, (256 - *weights_y.offset(y) as i32) as u16);

        let weighted_top_low = vmlal_n_u16(weighted_bl, top_low, *weights_y.offset(y));
        vst1_u16(dst, vrshrn_n_u32::<SMOOTH_WEIGHT_LOG2_SCALE>(weighted_top_low));

        let weighted_top_high = vmlal_n_u16(weighted_bl, top_high, *weights_y.offset(y));
        vst1_u16(dst.add(4), vrshrn_n_u32::<SMOOTH_WEIGHT_LOG2_SCALE>(weighted_top_high));
        dst = dst.wrapping_offset(stride);
    }
}

highbd_smooth_nxm!(aom_highbd_smooth_v_predictor_4x4_neon, highbd_smooth_v_4xh_neon, 4);
highbd_smooth_nxm!(aom_highbd_smooth_v_predictor_4x8_neon, highbd_smooth_v_4xh_neon, 8);
highbd_smooth_nxm!(aom_highbd_smooth_v_predictor_4x16_neon, highbd_smooth_v_4xh_neon, 16);
highbd_smooth_nxm!(aom_highbd_smooth_v_predictor_8x4_neon, highbd_smooth_v_8xh_neon, 4);
highbd_smooth_nxm!(aom_highbd_smooth_v_predictor_8x8_neon, highbd_smooth_v_8xh_neon, 8);
highbd_smooth_nxm!(aom_highbd_smooth_v_predictor_8x16_neon, highbd_smooth_v_8xh_neon, 16);
highbd_smooth_nxm!(aom_highbd_smooth_v_predictor_8x32_neon, highbd_smooth_v_8xh_neon, 32);

// For width 16 and above.
// #define HIGHBD_SMOOTH_V_PREDICTOR(W)
macro_rules! highbd_smooth_v_predictor {
    ($fn_name:ident, $w:literal) => {
        unsafe fn $fn_name(
            mut dst: *mut u16,
            stride: isize,
            top_row: *const u16,
            left_column: *const u16,
            height: i32,
        ) {
            let bottom_left = *left_column.offset(height as isize - 1);
            let weights_y = smooth_weights_u16.as_ptr().offset(height as isize - 4);

            let mut top_vals: [uint16x4x2_t; ($w) >> 3] = [uint16x4x2_t(vdup_n_u16(0), vdup_n_u16(0)); ($w) >> 3];
            for i in 0..(($w) >> 3) {
                let x = i << 3;
                top_vals[i].0 = vld1_u16(top_row.add(x));
                top_vals[i].1 = vld1_u16(top_row.add(x + 4));
            }

            let bottom_left_v = vdup_n_u16(bottom_left);
            for y in 0..height as isize {
                let weighted_bl = vmull_n_u16(bottom_left_v, (256 - *weights_y.offset(y) as i32) as u16);

                let mut dst_x = dst;
                for i in 0..(($w) >> 3) {
                    let weighted_top_low = vmlal_n_u16(weighted_bl, top_vals[i].0, *weights_y.offset(y));
                    vst1_u16(dst_x, vrshrn_n_u32::<SMOOTH_WEIGHT_LOG2_SCALE>(weighted_top_low));

                    let weighted_top_high = vmlal_n_u16(weighted_bl, top_vals[i].1, *weights_y.offset(y));
                    vst1_u16(dst_x.add(4), vrshrn_n_u32::<SMOOTH_WEIGHT_LOG2_SCALE>(weighted_top_high));
                    dst_x = dst_x.add(8);
                }
                dst = dst.wrapping_offset(stride);
            }
        }
    };
}

highbd_smooth_v_predictor!(highbd_smooth_v_16xh_neon, 16);
highbd_smooth_v_predictor!(highbd_smooth_v_32xh_neon, 32);
highbd_smooth_v_predictor!(highbd_smooth_v_64xh_neon, 64);

highbd_smooth_nxm!(aom_highbd_smooth_v_predictor_16x4_neon, highbd_smooth_v_16xh_neon, 4);
highbd_smooth_nxm!(aom_highbd_smooth_v_predictor_16x8_neon, highbd_smooth_v_16xh_neon, 8);
highbd_smooth_nxm!(aom_highbd_smooth_v_predictor_16x16_neon, highbd_smooth_v_16xh_neon, 16);
highbd_smooth_nxm!(aom_highbd_smooth_v_predictor_16x32_neon, highbd_smooth_v_16xh_neon, 32);
highbd_smooth_nxm!(aom_highbd_smooth_v_predictor_16x64_neon, highbd_smooth_v_16xh_neon, 64);
highbd_smooth_nxm!(aom_highbd_smooth_v_predictor_32x8_neon, highbd_smooth_v_32xh_neon, 8);
highbd_smooth_nxm!(aom_highbd_smooth_v_predictor_32x16_neon, highbd_smooth_v_32xh_neon, 16);
highbd_smooth_nxm!(aom_highbd_smooth_v_predictor_32x32_neon, highbd_smooth_v_32xh_neon, 32);
highbd_smooth_nxm!(aom_highbd_smooth_v_predictor_32x64_neon, highbd_smooth_v_32xh_neon, 64);
highbd_smooth_nxm!(aom_highbd_smooth_v_predictor_64x16_neon, highbd_smooth_v_64xh_neon, 16);
highbd_smooth_nxm!(aom_highbd_smooth_v_predictor_64x32_neon, highbd_smooth_v_64xh_neon, 32);
highbd_smooth_nxm!(aom_highbd_smooth_v_predictor_64x64_neon, highbd_smooth_v_64xh_neon, 64);

#[inline(always)]
unsafe fn highbd_smooth_h_4xh_neon(
    mut dst: *mut u16,
    stride: isize,
    top_row: *const u16,
    left_column: *const u16,
    height: i32,
) {
    let top_right = *top_row.add(3);

    let weights_x = vld1_u16(smooth_weights_u16.as_ptr());
    let scaled_weights_x = negate_s8(weights_x);

    let weighted_tr = vmull_n_u16(scaled_weights_x, top_right);
    for y in 0..height as isize {
        let weighted_left = vmlal_n_u16(weighted_tr, weights_x, *left_column.offset(y));
        vst1_u16(dst, vrshrn_n_u32::<SMOOTH_WEIGHT_LOG2_SCALE>(weighted_left));
        dst = dst.wrapping_offset(stride);
    }
}

#[inline(always)]
unsafe fn highbd_smooth_h_8xh_neon(
    mut dst: *mut u16,
    stride: isize,
    top_row: *const u16,
    left_column: *const u16,
    height: i32,
) {
    let top_right = *top_row.add(7);

    let weights_x = uint16x4x2_t(
        vld1_u16(smooth_weights_u16.as_ptr().add(4)),
        vld1_u16(smooth_weights_u16.as_ptr().add(8)),
    );

    let weighted_tr_low = vmull_n_u16(negate_s8(weights_x.0), top_right);
    let weighted_tr_high = vmull_n_u16(negate_s8(weights_x.1), top_right);

    for y in 0..height as isize {
        let left_y = *left_column.offset(y);
        let weighted_left_low = vmlal_n_u16(weighted_tr_low, weights_x.0, left_y);
        vst1_u16(dst, vrshrn_n_u32::<SMOOTH_WEIGHT_LOG2_SCALE>(weighted_left_low));

        let weighted_left_high = vmlal_n_u16(weighted_tr_high, weights_x.1, left_y);
        vst1_u16(dst.add(4), vrshrn_n_u32::<SMOOTH_WEIGHT_LOG2_SCALE>(weighted_left_high));
        dst = dst.wrapping_offset(stride);
    }
}

highbd_smooth_nxm!(aom_highbd_smooth_h_predictor_4x4_neon, highbd_smooth_h_4xh_neon, 4);
highbd_smooth_nxm!(aom_highbd_smooth_h_predictor_4x8_neon, highbd_smooth_h_4xh_neon, 8);
highbd_smooth_nxm!(aom_highbd_smooth_h_predictor_4x16_neon, highbd_smooth_h_4xh_neon, 16);
highbd_smooth_nxm!(aom_highbd_smooth_h_predictor_8x4_neon, highbd_smooth_h_8xh_neon, 4);
highbd_smooth_nxm!(aom_highbd_smooth_h_predictor_8x8_neon, highbd_smooth_h_8xh_neon, 8);
highbd_smooth_nxm!(aom_highbd_smooth_h_predictor_8x16_neon, highbd_smooth_h_8xh_neon, 16);
highbd_smooth_nxm!(aom_highbd_smooth_h_predictor_8x32_neon, highbd_smooth_h_8xh_neon, 32);

// For width 16 and above.
// #define HIGHBD_SMOOTH_H_PREDICTOR(W)
macro_rules! highbd_smooth_h_predictor {
    ($fn_name:ident, $w:literal) => {
        unsafe fn $fn_name(
            mut dst: *mut u16,
            stride: isize,
            top_row: *const u16,
            left_column: *const u16,
            height: i32,
        ) {
            let top_right = *top_row.add(($w) - 1);

            let mut weights_x_low: [uint16x4_t; ($w) >> 3] = [vdup_n_u16(0); ($w) >> 3];
            let mut weights_x_high: [uint16x4_t; ($w) >> 3] = [vdup_n_u16(0); ($w) >> 3];
            let mut weighted_tr_low: [uint32x4_t; ($w) >> 3] = [vdupq_n_u32(0); ($w) >> 3];
            let mut weighted_tr_high: [uint32x4_t; ($w) >> 3] = [vdupq_n_u32(0); ($w) >> 3];
            for i in 0..(($w) >> 3) {
                let x = i << 3;
                weights_x_low[i] = vld1_u16(smooth_weights_u16.as_ptr().add(($w) - 4 + x));
                weighted_tr_low[i] = vmull_n_u16(negate_s8(weights_x_low[i]), top_right);
                weights_x_high[i] = vld1_u16(smooth_weights_u16.as_ptr().add(($w) + x));
                weighted_tr_high[i] = vmull_n_u16(negate_s8(weights_x_high[i]), top_right);
            }

            for y in 0..height as isize {
                let mut dst_x = dst;
                let left_y = *left_column.offset(y);
                for i in 0..(($w) >> 3) {
                    let weighted_left_low = vmlal_n_u16(weighted_tr_low[i], weights_x_low[i], left_y);
                    vst1_u16(dst_x, vrshrn_n_u32::<SMOOTH_WEIGHT_LOG2_SCALE>(weighted_left_low));

                    let weighted_left_high = vmlal_n_u16(weighted_tr_high[i], weights_x_high[i], left_y);
                    vst1_u16(dst_x.add(4), vrshrn_n_u32::<SMOOTH_WEIGHT_LOG2_SCALE>(weighted_left_high));
                    dst_x = dst_x.add(8);
                }
                dst = dst.wrapping_offset(stride);
            }
        }
    };
}

highbd_smooth_h_predictor!(highbd_smooth_h_16xh_neon, 16);
highbd_smooth_h_predictor!(highbd_smooth_h_32xh_neon, 32);
highbd_smooth_h_predictor!(highbd_smooth_h_64xh_neon, 64);

highbd_smooth_nxm!(aom_highbd_smooth_h_predictor_16x4_neon, highbd_smooth_h_16xh_neon, 4);
highbd_smooth_nxm!(aom_highbd_smooth_h_predictor_16x8_neon, highbd_smooth_h_16xh_neon, 8);
highbd_smooth_nxm!(aom_highbd_smooth_h_predictor_16x16_neon, highbd_smooth_h_16xh_neon, 16);
highbd_smooth_nxm!(aom_highbd_smooth_h_predictor_16x32_neon, highbd_smooth_h_16xh_neon, 32);
highbd_smooth_nxm!(aom_highbd_smooth_h_predictor_16x64_neon, highbd_smooth_h_16xh_neon, 64);
highbd_smooth_nxm!(aom_highbd_smooth_h_predictor_32x8_neon, highbd_smooth_h_32xh_neon, 8);
highbd_smooth_nxm!(aom_highbd_smooth_h_predictor_32x16_neon, highbd_smooth_h_32xh_neon, 16);
highbd_smooth_nxm!(aom_highbd_smooth_h_predictor_32x32_neon, highbd_smooth_h_32xh_neon, 32);
highbd_smooth_nxm!(aom_highbd_smooth_h_predictor_32x64_neon, highbd_smooth_h_32xh_neon, 64);
highbd_smooth_nxm!(aom_highbd_smooth_h_predictor_64x16_neon, highbd_smooth_h_64xh_neon, 16);
highbd_smooth_nxm!(aom_highbd_smooth_h_predictor_64x32_neon, highbd_smooth_h_64xh_neon, 32);
highbd_smooth_nxm!(aom_highbd_smooth_h_predictor_64x64_neon, highbd_smooth_h_64xh_neon, 64);

// -----------------------------------------------------------------------------
// Z1

static iota1_s16: [i16; 9] = [0, 1, 2, 3, 4, 5, 6, 7, 8];
static iota2_s16: [i16; 8] = [0, 2, 4, 6, 8, 10, 12, 14];

#[inline(always)]
unsafe fn highbd_dr_z1_apply_shift_x4(a0: uint16x4_t, a1: uint16x4_t, shift: i32) -> uint16x4_t {
    // The C implementation of the z1 predictor uses (32 - shift) and a right
    // shift by 5, however we instead double shift to avoid an unnecessary right
    // shift by 1.
    let mut res = vmull_n_u16(a1, shift as u16);
    res = vmlal_n_u16(res, a0, (64 - shift) as u16);
    vrshrn_n_u32::<6>(res)
}

#[inline(always)]
unsafe fn highbd_dr_z1_apply_shift_x8(a0: uint16x8_t, a1: uint16x8_t, shift: i32) -> uint16x8_t {
    vcombine_u16(
        highbd_dr_z1_apply_shift_x4(vget_low_u16(a0), vget_low_u16(a1), shift),
        highbd_dr_z1_apply_shift_x4(vget_high_u16(a0), vget_high_u16(a1), shift),
    )
}

#[rustfmt::skip]
static kLoadMaxShuffles: [u8; 128] = [
    14, 15, 14, 15, 14, 15, 14, 15, 14, 15, 14, 15, 14, 15, 14, 15,
    12, 13, 14, 15, 14, 15, 14, 15, 14, 15, 14, 15, 14, 15, 14, 15,
    10, 11, 12, 13, 14, 15, 14, 15, 14, 15, 14, 15, 14, 15, 14, 15,
     8,  9, 10, 11, 12, 13, 14, 15, 14, 15, 14, 15, 14, 15, 14, 15,
     6,  7,  8,  9, 10, 11, 12, 13, 14, 15, 14, 15, 14, 15, 14, 15,
     4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14, 15, 14, 15, 14, 15,
     2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14, 15, 14, 15,
     0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14, 15,
];

#[inline(always)]
unsafe fn zn_load_masked_neon(ptr: *const u16, shuffle_idx: i32) -> uint16x8_t {
    let shuffle = vld1q_u8(kLoadMaxShuffles.as_ptr().add(16 * shuffle_idx as usize));
    let src = vreinterpretq_u8_u16(vld1q_u16(ptr));
    vreinterpretq_u16_u8(vqtbl1q_u8(src, shuffle))
}

unsafe fn highbd_dr_prediction_z1_upsample0_neon(
    mut dst: *mut u16,
    stride: isize,
    bw: i32,
    bh: i32,
    above: *const u16,
    dx: i32,
) {
    debug_assert!(bw % 4 == 0);
    debug_assert!(bh % 4 == 0);
    debug_assert!(dx > 0);

    let max_base_x = (bw + bh) - 1;
    let above_max = *above.offset(max_base_x as isize) as i32;

    let iota1x8 = vld1q_s16(iota1_s16.as_ptr());
    let iota1x4 = vget_low_s16(iota1x8);

    let mut x = dx;
    let mut r = 0;
    loop {
        let base = x >> 6;
        if base >= max_base_x {
            for _ in r..bh {
                aom_memset16(dst, above_max, bw as usize);
                dst = dst.wrapping_offset(stride);
            }
            return;
        }

        // The C implementation of the z1 predictor when not upsampling uses:
        // ((x & 0x3f) >> 1)
        // The right shift is unnecessary here since we instead shift by +1 later,
        // so adjust the mask to 0x3e to ensure we don't consider the extra bit.
        let shift = x & 0x3e;

        if bw == 4 {
            let a0 = vld1_u16(above.offset(base as isize));
            let a1 = vld1_u16(above.offset(base as isize + 1));
            let val = highbd_dr_z1_apply_shift_x4(a0, a1, shift);
            let cmp = vcgt_s16(vdup_n_s16((max_base_x - base) as i16), iota1x4);
            let res = vbsl_u16(cmp, val, vdup_n_u16(above_max as u16));
            vst1_u16(dst, res);
        } else {
            let mut c = 0;
            loop {
                let a0: uint16x8_t;
                let a1: uint16x8_t;
                if base + c >= max_base_x {
                    a1 = vdupq_n_u16(above_max as u16);
                    a0 = a1;
                } else {
                    if base + c + 7 >= max_base_x {
                        let shuffle_idx = max_base_x - base - c;
                        a0 = zn_load_masked_neon(above.offset((max_base_x - 7) as isize), shuffle_idx);
                    } else {
                        a0 = vld1q_u16(above.offset((base + c) as isize));
                    }
                    if base + c + 8 >= max_base_x {
                        let shuffle_idx = max_base_x - base - c - 1;
                        a1 = zn_load_masked_neon(above.offset((max_base_x - 7) as isize), shuffle_idx);
                    } else {
                        a1 = vld1q_u16(above.offset((base + c + 1) as isize));
                    }
                }

                vst1q_u16(dst.offset(c as isize), highbd_dr_z1_apply_shift_x8(a0, a1, shift));
                c += 8;
                if c >= bw {
                    break;
                }
            }
        }

        dst = dst.wrapping_offset(stride);
        x += dx;
        r += 1;
        if r >= bh {
            break;
        }
    }
}

unsafe fn highbd_dr_prediction_z1_upsample1_neon(
    mut dst: *mut u16,
    stride: isize,
    bw: i32,
    bh: i32,
    above: *const u16,
    dx: i32,
) {
    debug_assert!(bw % 4 == 0);
    debug_assert!(bh % 4 == 0);
    debug_assert!(dx > 0);

    let max_base_x = ((bw + bh) - 1) << 1;
    let above_max = *above.offset(max_base_x as isize) as i32;

    let iota2x8 = vld1q_s16(iota2_s16.as_ptr());
    let iota2x4 = vget_low_s16(iota2x8);

    let mut x = dx;
    let mut r = 0;
    loop {
        let base = x >> 5;
        if base >= max_base_x {
            for _ in r..bh {
                aom_memset16(dst, above_max, bw as usize);
                dst = dst.wrapping_offset(stride);
            }
            return;
        }

        // The C implementation of the z1 predictor when upsampling uses:
        // (((x << 1) & 0x3f) >> 1)
        // The right shift is unnecessary here since we instead shift by +1 later,
        // so adjust the mask to 0x3e to ensure we don't consider the extra bit.
        let shift = (x << 1) & 0x3e;

        if bw == 4 {
            let a01 = vld2_u16(above.offset(base as isize));
            let val = highbd_dr_z1_apply_shift_x4(a01.0, a01.1, shift);
            let cmp = vcgt_s16(vdup_n_s16((max_base_x - base) as i16), iota2x4);
            let res = vbsl_u16(cmp, val, vdup_n_u16(above_max as u16));
            vst1_u16(dst, res);
        } else {
            let mut c = 0;
            loop {
                let a01 = vld2q_u16(above.offset((base + 2 * c) as isize));
                let val = highbd_dr_z1_apply_shift_x8(a01.0, a01.1, shift);
                let cmp = vcgtq_s16(vdupq_n_s16((max_base_x - base - 2 * c) as i16), iota2x8);
                let res = vbslq_u16(cmp, val, vdupq_n_u16(above_max as u16));
                vst1q_u16(dst.offset(c as isize), res);
                c += 8;
                if c >= bw {
                    break;
                }
            }
        }

        dst = dst.wrapping_offset(stride);
        x += dx;
        r += 1;
        if r >= bh {
            break;
        }
    }
}

// Directional prediction, zone 1: 0 < angle < 90
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_highbd_dr_prediction_z1_neon(
    dst: *mut u16,
    stride: isize,
    bw: i32,
    bh: i32,
    above: *const u16,
    _left: *const u16,
    upsample_above: i32,
    dx: i32,
    dy: i32,
    _bd: i32,
) {
    debug_assert!(dy == 1);
    let _ = dy;

    if upsample_above != 0 {
        highbd_dr_prediction_z1_upsample1_neon(dst, stride, bw, bh, above, dx);
    } else {
        highbd_dr_prediction_z1_upsample0_neon(dst, stride, bw, bh, above, dx);
    }
}

// -----------------------------------------------------------------------------
// Z2

// Incrementally shift more elements from `above` into the result, merging with
// existing `left` elements.
// X0, X1, X2, X3
// Y0, X0, X1, X2
// Y0, Y1, X0, X1
// Y0, Y1, Y2, X0
// Y0, Y1, Y2, Y3
#[rustfmt::skip]
static z2_merge_shuffles_u16x4: [[u8; 8]; 5] = [
    [  8,  9, 10, 11, 12, 13, 14, 15 ],
    [  0,  1,  8,  9, 10, 11, 12, 13 ],
    [  0,  1,  2,  3,  8,  9, 10, 11 ],
    [  0,  1,  2,  3,  4,  5,  8,  9 ],
    [  0,  1,  2,  3,  4,  5,  6,  7 ],
];

// Incrementally shift more elements from `above` into the result, merging with
// existing `left` elements.
// X0, X1, X2, X3, X4, X5, X6, X7
// Y0, X0, X1, X2, X3, X4, X5, X6
// Y0, Y1, X0, X1, X2, X3, X4, X5
// Y0, Y1, Y2, X0, X1, X2, X3, X4
// Y0, Y1, Y2, Y3, X0, X1, X2, X3
// Y0, Y1, Y2, Y3, Y4, X0, X1, X2
// Y0, Y1, Y2, Y3, Y4, Y5, X0, X1
// Y0, Y1, Y2, Y3, Y4, Y5, Y6, X0
// Y0, Y1, Y2, Y3, Y4, Y5, Y6, Y7
#[rustfmt::skip]
static z2_merge_shuffles_u16x8: [[u8; 16]; 9] = [
    [ 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31 ],
    [  0,  1, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29 ],
    [  0,  1,  2,  3, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27 ],
    [  0,  1,  2,  3,  4,  5, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25 ],
    [  0,  1,  2,  3,  4,  5,  6,  7, 16, 17, 18, 19, 20, 21, 22, 23 ],
    [  0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 16, 17, 18, 19, 20, 21 ],
    [  0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 16, 17, 18, 19 ],
    [  0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 16, 17 ],
    [  0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14, 15 ],
];

#[rustfmt::skip]
static z2_y_iter_masks_u16x4: [[u16; 4]; 5] = [
    [      0,      0,      0,      0 ],
    [ 0xffff,      0,      0,      0 ],
    [ 0xffff, 0xffff,      0,      0 ],
    [ 0xffff, 0xffff, 0xffff,      0 ],
    [ 0xffff, 0xffff, 0xffff, 0xffff ],
];

#[rustfmt::skip]
static z2_y_iter_masks_u16x8: [[u16; 8]; 9] = [
    [      0,      0,      0,      0,      0,      0,      0,      0 ],
    [ 0xffff,      0,      0,      0,      0,      0,      0,      0 ],
    [ 0xffff, 0xffff,      0,      0,      0,      0,      0,      0 ],
    [ 0xffff, 0xffff, 0xffff,      0,      0,      0,      0,      0 ],
    [ 0xffff, 0xffff, 0xffff, 0xffff,      0,      0,      0,      0 ],
    [ 0xffff, 0xffff, 0xffff, 0xffff, 0xffff,      0,      0,      0 ],
    [ 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff,      0,      0 ],
    [ 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff,      0 ],
    [ 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff ],
];

#[inline(always)]
unsafe fn highbd_dr_prediction_z2_tbl_left_x4_from_x8(
    left_data: uint16x8_t,
    indices: int16x4_t,
    base: i32,
    n: i32,
) -> uint16x4_t {
    // Need to adjust indices to operate on 0-based indices rather than
    // `base`-based indices and then adjust from uint16x4 indices to uint8x8
    // indices so we can use a tbl instruction (which only operates on bytes).
    let mut left_indices = vreinterpret_u8_s16(vsub_s16(indices, vdup_n_s16(base as i16)));
    left_indices = vtrn1_u8(left_indices, left_indices);
    left_indices = vadd_u8(left_indices, left_indices);
    left_indices = vadd_u8(left_indices, vreinterpret_u8_u16(vdup_n_u16(0x0100)));
    let ret = vreinterpret_u16_u8(vqtbl1_u8(vreinterpretq_u8_u16(left_data), left_indices));
    vand_u16(ret, vld1_u16(z2_y_iter_masks_u16x4[n as usize].as_ptr()))
}

#[inline(always)]
unsafe fn highbd_dr_prediction_z2_tbl_left_x4_from_x16(
    left_data: uint16x8x2_t,
    indices: int16x4_t,
    base: i32,
    n: i32,
) -> uint16x4_t {
    // Need to adjust indices to operate on 0-based indices rather than
    // `base`-based indices and then adjust from uint16x4 indices to uint8x8
    // indices so we can use a tbl instruction (which only operates on bytes).
    let mut left_indices = vreinterpret_u8_s16(vsub_s16(indices, vdup_n_s16(base as i16)));
    left_indices = vtrn1_u8(left_indices, left_indices);
    left_indices = vadd_u8(left_indices, left_indices);
    left_indices = vadd_u8(left_indices, vreinterpret_u8_u16(vdup_n_u16(0x0100)));
    let data_u8 = uint8x16x2_t(vreinterpretq_u8_u16(left_data.0), vreinterpretq_u8_u16(left_data.1));
    let ret = vreinterpret_u16_u8(vqtbl2_u8(data_u8, left_indices));
    vand_u16(ret, vld1_u16(z2_y_iter_masks_u16x4[n as usize].as_ptr()))
}

#[inline(always)]
unsafe fn highbd_dr_prediction_z2_tbl_left_x8_from_x8(
    left_data: uint16x8_t,
    indices: int16x8_t,
    base: i32,
    n: i32,
) -> uint16x8_t {
    // Need to adjust indices to operate on 0-based indices rather than
    // `base`-based indices and then adjust from uint16x4 indices to uint8x8
    // indices so we can use a tbl instruction (which only operates on bytes).
    let mut left_indices = vreinterpretq_u8_s16(vsubq_s16(indices, vdupq_n_s16(base as i16)));
    left_indices = vtrn1q_u8(left_indices, left_indices);
    left_indices = vaddq_u8(left_indices, left_indices);
    left_indices = vaddq_u8(left_indices, vreinterpretq_u8_u16(vdupq_n_u16(0x0100)));
    let ret = vreinterpretq_u16_u8(vqtbl1q_u8(vreinterpretq_u8_u16(left_data), left_indices));
    vandq_u16(ret, vld1q_u16(z2_y_iter_masks_u16x8[n as usize].as_ptr()))
}

#[inline(always)]
unsafe fn highbd_dr_prediction_z2_tbl_left_x8_from_x16(
    left_data: uint16x8x2_t,
    indices: int16x8_t,
    base: i32,
    n: i32,
) -> uint16x8_t {
    // Need to adjust indices to operate on 0-based indices rather than
    // `base`-based indices and then adjust from uint16x4 indices to uint8x8
    // indices so we can use a tbl instruction (which only operates on bytes).
    let mut left_indices = vreinterpretq_u8_s16(vsubq_s16(indices, vdupq_n_s16(base as i16)));
    left_indices = vtrn1q_u8(left_indices, left_indices);
    left_indices = vaddq_u8(left_indices, left_indices);
    left_indices = vaddq_u8(left_indices, vreinterpretq_u8_u16(vdupq_n_u16(0x0100)));
    let data_u8 = uint8x16x2_t(vreinterpretq_u8_u16(left_data.0), vreinterpretq_u8_u16(left_data.1));
    let ret = vreinterpretq_u16_u8(vqtbl2q_u8(data_u8, left_indices));
    vandq_u16(ret, vld1q_u16(z2_y_iter_masks_u16x8[n as usize].as_ptr()))
}

#[inline(always)]
unsafe fn highbd_dr_prediction_z2_gather_left_x4(left: *const u16, indices: int16x4_t, n: i32) -> uint16x4x2_t {
    debug_assert!(n > 0);
    debug_assert!(n <= 4);
    // Load two elements at a time and then uzp them into separate vectors, to
    // reduce the number of memory accesses.
    let mut ret0_u32 = vdup_n_u32(0);
    let mut ret1_u32 = vdup_n_u32(0);

    // Use a single vget_lane_u64 to minimize vector to general purpose register
    // transfers and then mask off the bits we actually want.
    let indices0123: u64 = vget_lane_u64::<0>(vreinterpret_u64_s16(indices));
    let idx0 = ((indices0123 >> 0) & 0xffff) as i16 as isize;
    let idx1 = ((indices0123 >> 16) & 0xffff) as i16 as isize;
    let idx2 = ((indices0123 >> 32) & 0xffff) as i16 as isize;
    let idx3 = ((indices0123 >> 48) & 0xffff) as i16 as isize;

    // At time of writing both Clang and GCC produced better code with these
    // nested if-statements compared to a switch statement with fallthrough.
    load_unaligned_u32_2x1_lane::<0, u16>(&mut ret0_u32, left.offset(idx0));
    if n > 1 {
        load_unaligned_u32_2x1_lane::<1, u16>(&mut ret0_u32, left.offset(idx1));
        if n > 2 {
            load_unaligned_u32_2x1_lane::<0, u16>(&mut ret1_u32, left.offset(idx2));
            if n > 3 {
                load_unaligned_u32_2x1_lane::<1, u16>(&mut ret1_u32, left.offset(idx3));
            }
        }
    }
    vuzp_u16(vreinterpret_u16_u32(ret0_u32), vreinterpret_u16_u32(ret1_u32))
}

#[inline(always)]
unsafe fn highbd_dr_prediction_z2_gather_left_x8(left: *const u16, indices: int16x8_t, n: i32) -> uint16x8x2_t {
    debug_assert!(n > 0);
    debug_assert!(n <= 8);
    // Load two elements at a time and then uzp them into separate vectors, to
    // reduce the number of memory accesses.
    let mut ret0_u32 = vdupq_n_u32(0);
    let mut ret1_u32 = vdupq_n_u32(0);

    // Use a pair of vget_lane_u64 to minimize vector to general purpose register
    // transfers and then mask off the bits we actually want.
    let indices0123: u64 = vgetq_lane_u64::<0>(vreinterpretq_u64_s16(indices));
    let indices4567: u64 = vgetq_lane_u64::<1>(vreinterpretq_u64_s16(indices));
    let idx0 = ((indices0123 >> 0) & 0xffff) as i16 as isize;
    let idx1 = ((indices0123 >> 16) & 0xffff) as i16 as isize;
    let idx2 = ((indices0123 >> 32) & 0xffff) as i16 as isize;
    let idx3 = ((indices0123 >> 48) & 0xffff) as i16 as isize;
    let idx4 = ((indices4567 >> 0) & 0xffff) as i16 as isize;
    let idx5 = ((indices4567 >> 16) & 0xffff) as i16 as isize;
    let idx6 = ((indices4567 >> 32) & 0xffff) as i16 as isize;
    let idx7 = ((indices4567 >> 48) & 0xffff) as i16 as isize;

    // At time of writing both Clang and GCC produced better code with these
    // nested if-statements compared to a switch statement with fallthrough.
    load_unaligned_u32_4x1_lane::<0, u16>(&mut ret0_u32, left.offset(idx0));
    if n > 1 {
        load_unaligned_u32_4x1_lane::<1, u16>(&mut ret0_u32, left.offset(idx1));
        if n > 2 {
            load_unaligned_u32_4x1_lane::<2, u16>(&mut ret0_u32, left.offset(idx2));
            if n > 3 {
                load_unaligned_u32_4x1_lane::<3, u16>(&mut ret0_u32, left.offset(idx3));
                if n > 4 {
                    load_unaligned_u32_4x1_lane::<0, u16>(&mut ret1_u32, left.offset(idx4));
                    if n > 5 {
                        load_unaligned_u32_4x1_lane::<1, u16>(&mut ret1_u32, left.offset(idx5));
                        if n > 6 {
                            load_unaligned_u32_4x1_lane::<2, u16>(&mut ret1_u32, left.offset(idx6));
                            if n > 7 {
                                load_unaligned_u32_4x1_lane::<3, u16>(&mut ret1_u32, left.offset(idx7));
                            }
                        }
                    }
                }
            }
        }
    }
    vuzpq_u16(vreinterpretq_u16_u32(ret0_u32), vreinterpretq_u16_u32(ret1_u32))
}

#[inline(always)]
unsafe fn highbd_dr_prediction_z2_merge_x4(out_x: uint16x4_t, out_y: uint16x4_t, base_shift: i32) -> uint16x4_t {
    debug_assert!(base_shift >= 0);
    debug_assert!(base_shift <= 4);
    // On AArch64 we can permute the data from the `above` and `left` vectors
    // into a single vector in a single load (of the permute vector) + tbl.
    let out_yx = uint8x8x2_t(vreinterpret_u8_u16(out_y), vreinterpret_u8_u16(out_x));
    vreinterpret_u16_u8(vtbl2_u8(out_yx, vld1_u8(z2_merge_shuffles_u16x4[base_shift as usize].as_ptr())))
}

#[inline(always)]
unsafe fn highbd_dr_prediction_z2_merge_x8(out_x: uint16x8_t, out_y: uint16x8_t, base_shift: i32) -> uint16x8_t {
    debug_assert!(base_shift >= 0);
    debug_assert!(base_shift <= 8);
    // On AArch64 we can permute the data from the `above` and `left` vectors
    // into a single vector in a single load (of the permute vector) + tbl.
    let out_yx = uint8x16x2_t(vreinterpretq_u8_u16(out_y), vreinterpretq_u8_u16(out_x));
    vreinterpretq_u16_u8(vqtbl2q_u8(out_yx, vld1q_u8(z2_merge_shuffles_u16x8[base_shift as usize].as_ptr())))
}

#[inline(always)]
unsafe fn highbd_dr_prediction_z2_apply_shift_x4(a0: uint16x4_t, a1: uint16x4_t, shift: int16x4_t) -> uint16x4_t {
    let mut res = vmull_u16(a1, vreinterpret_u16_s16(shift));
    res = vmlal_u16(res, a0, vsub_u16(vdup_n_u16(32), vreinterpret_u16_s16(shift)));
    vrshrn_n_u32::<5>(res)
}

#[inline(always)]
unsafe fn highbd_dr_prediction_z2_apply_shift_x8(a0: uint16x8_t, a1: uint16x8_t, shift: int16x8_t) -> uint16x8_t {
    vcombine_u16(
        highbd_dr_prediction_z2_apply_shift_x4(vget_low_u16(a0), vget_low_u16(a1), vget_low_s16(shift)),
        highbd_dr_prediction_z2_apply_shift_x4(vget_high_u16(a0), vget_high_u16(a1), vget_high_s16(shift)),
    )
}

#[inline(always)]
#[allow(clippy::too_many_arguments)]
unsafe fn highbd_dr_prediction_z2_step_x4(
    above: *const u16,
    above0: uint16x4_t,
    above1: uint16x4_t,
    left: *const u16,
    dx: i32,
    dy: i32,
    r: i32,
    c: i32,
) -> uint16x4_t {
    let iota = vld1_s16(iota1_s16.as_ptr());

    let x0 = (c << 6) - (r + 1) * dx;
    let y0 = (r << 6) - (c + 1) * dy;

    let x0123 = vadd_s16(vdup_n_s16(x0 as i16), vshl_n_s16::<6>(iota));
    let y0123 = vsub_s16(vdup_n_s16(y0 as i16), vmul_n_s16(iota, dy as i16));
    let shift_x0123 = vshr_n_s16::<1>(vand_s16(x0123, vdup_n_s16(0x3F)));
    let shift_y0123 = vshr_n_s16::<1>(vand_s16(y0123, vdup_n_s16(0x3F)));
    let base_y0123 = vshr_n_s16::<6>(y0123);

    let base_shift = ((((r + 1) * dx) - 1) >> 6) - c;

    // Based on the value of `base_shift` there are three possible cases to
    // compute the result:
    // 1) base_shift <= 0: We can load and operate entirely on data from the
    //                     `above` input vector.
    // 2) base_shift < vl: We can load from `above[-1]` and shift
    //                     `vl - base_shift` elements across to the end of the
    //                     vector, then compute the remainder from `left`.
    // 3) base_shift >= vl: We can load and operate entirely on data from the
    //                      `left` input vector.

    if base_shift <= 0 {
        let base_x = x0 >> 6;
        let a0 = vld1_u16(above.offset(base_x as isize));
        let a1 = vld1_u16(above.offset(base_x as isize + 1));
        highbd_dr_prediction_z2_apply_shift_x4(a0, a1, shift_x0123)
    } else if base_shift < 4 {
        let l01 = highbd_dr_prediction_z2_gather_left_x4(left.add(1), base_y0123, base_shift);
        let out16_y = highbd_dr_prediction_z2_apply_shift_x4(l01.0, l01.1, shift_y0123);

        // No need to reload from above in the loop, just use pre-loaded constants.
        let out16_x = highbd_dr_prediction_z2_apply_shift_x4(above0, above1, shift_x0123);

        highbd_dr_prediction_z2_merge_x4(out16_x, out16_y, base_shift)
    } else {
        let l01 = highbd_dr_prediction_z2_gather_left_x4(left.add(1), base_y0123, 4);
        highbd_dr_prediction_z2_apply_shift_x4(l01.0, l01.1, shift_y0123)
    }
}

#[inline(always)]
#[allow(clippy::too_many_arguments)]
unsafe fn highbd_dr_prediction_z2_step_x8(
    above: *const u16,
    above0: uint16x8_t,
    above1: uint16x8_t,
    left: *const u16,
    dx: i32,
    dy: i32,
    r: i32,
    c: i32,
) -> uint16x8_t {
    let iota = vld1q_s16(iota1_s16.as_ptr());

    let x0 = (c << 6) - (r + 1) * dx;
    let y0 = (r << 6) - (c + 1) * dy;

    let x01234567 = vaddq_s16(vdupq_n_s16(x0 as i16), vshlq_n_s16::<6>(iota));
    let y01234567 = vsubq_s16(vdupq_n_s16(y0 as i16), vmulq_n_s16(iota, dy as i16));
    let shift_x01234567 = vshrq_n_s16::<1>(vandq_s16(x01234567, vdupq_n_s16(0x3F)));
    let shift_y01234567 = vshrq_n_s16::<1>(vandq_s16(y01234567, vdupq_n_s16(0x3F)));
    let base_y01234567 = vshrq_n_s16::<6>(y01234567);

    let base_shift = ((((r + 1) * dx) - 1) >> 6) - c;

    // Based on the value of `base_shift` there are three possible cases to
    // compute the result:
    // 1) base_shift <= 0: We can load and operate entirely on data from the
    //                     `above` input vector.
    // 2) base_shift < vl: We can load from `above[-1]` and shift
    //                     `vl - base_shift` elements across to the end of the
    //                     vector, then compute the remainder from `left`.
    // 3) base_shift >= vl: We can load and operate entirely on data from the
    //                      `left` input vector.

    if base_shift <= 0 {
        let base_x = x0 >> 6;
        let a0 = vld1q_u16(above.offset(base_x as isize));
        let a1 = vld1q_u16(above.offset(base_x as isize + 1));
        highbd_dr_prediction_z2_apply_shift_x8(a0, a1, shift_x01234567)
    } else if base_shift < 8 {
        let l01 = highbd_dr_prediction_z2_gather_left_x8(left.add(1), base_y01234567, base_shift);
        let out16_y = highbd_dr_prediction_z2_apply_shift_x8(l01.0, l01.1, shift_y01234567);

        // No need to reload from above in the loop, just use pre-loaded constants.
        let out16_x = highbd_dr_prediction_z2_apply_shift_x8(above0, above1, shift_x01234567);

        highbd_dr_prediction_z2_merge_x8(out16_x, out16_y, base_shift)
    } else {
        let l01 = highbd_dr_prediction_z2_gather_left_x8(left.add(1), base_y01234567, 8);
        highbd_dr_prediction_z2_apply_shift_x8(l01.0, l01.1, shift_y01234567)
    }
}

// Left array is accessed from -1 through `bh - 1` inclusive.
// Above array is accessed from -1 through `bw - 1` inclusive.
// #define HIGHBD_DR_PREDICTOR_Z2_WXH(bw, bh)
macro_rules! highbd_dr_predictor_z2_wxh {
    ($fn_name:ident, $bw:literal, $bh:literal) => {
        #[allow(clippy::too_many_arguments)]
        unsafe fn $fn_name(
            mut dst: *mut u16,
            stride: isize,
            above: *const u16,
            left: *const u16,
            upsample_above: i32,
            upsample_left: i32,
            dx: i32,
            dy: i32,
            _bd: i32,
        ) {
            debug_assert!(upsample_above == 0);
            debug_assert!(upsample_left == 0);
            let _ = (upsample_above, upsample_left);
            debug_assert!($bw % 4 == 0);
            debug_assert!($bh % 4 == 0);
            debug_assert!(dx > 0);
            debug_assert!(dy > 0);

            let mut left_data: [u16; $bh + 1] = [0; $bh + 1];
            core::ptr::copy_nonoverlapping(left.offset(-1), left_data.as_mut_ptr(), $bh + 1);

            let a0: uint16x8_t;
            let a1: uint16x8_t;
            if $bw == 4 {
                a0 = vcombine_u16(vld1_u16(above.offset(-1)), vdup_n_u16(0));
                a1 = vcombine_u16(vld1_u16(above.offset(0)), vdup_n_u16(0));
            } else {
                a0 = vld1q_u16(above.offset(-1));
                a1 = vld1q_u16(above.offset(0));
            }

            let mut r = 0;
            loop {
                if $bw == 4 {
                    vst1_u16(
                        dst,
                        highbd_dr_prediction_z2_step_x4(
                            above,
                            vget_low_u16(a0),
                            vget_low_u16(a1),
                            left_data.as_ptr(),
                            dx,
                            dy,
                            r,
                            0,
                        ),
                    );
                } else {
                    let mut c = 0;
                    loop {
                        vst1q_u16(
                            dst.offset(c as isize),
                            highbd_dr_prediction_z2_step_x8(above, a0, a1, left_data.as_ptr(), dx, dy, r, c),
                        );
                        c += 8;
                        if c >= $bw {
                            break;
                        }
                    }
                }
                dst = dst.wrapping_offset(stride);
                r += 1;
                if r >= $bh {
                    break;
                }
            }
        }
    };
}

highbd_dr_predictor_z2_wxh!(highbd_dr_prediction_z2_4x16_neon, 4, 16);
highbd_dr_predictor_z2_wxh!(highbd_dr_prediction_z2_8x16_neon, 8, 16);
highbd_dr_predictor_z2_wxh!(highbd_dr_prediction_z2_8x32_neon, 8, 32);
highbd_dr_predictor_z2_wxh!(highbd_dr_prediction_z2_16x4_neon, 16, 4);
highbd_dr_predictor_z2_wxh!(highbd_dr_prediction_z2_16x8_neon, 16, 8);
highbd_dr_predictor_z2_wxh!(highbd_dr_prediction_z2_16x16_neon, 16, 16);
highbd_dr_predictor_z2_wxh!(highbd_dr_prediction_z2_16x32_neon, 16, 32);
highbd_dr_predictor_z2_wxh!(highbd_dr_prediction_z2_16x64_neon, 16, 64);
highbd_dr_predictor_z2_wxh!(highbd_dr_prediction_z2_32x8_neon, 32, 8);
highbd_dr_predictor_z2_wxh!(highbd_dr_prediction_z2_32x16_neon, 32, 16);
highbd_dr_predictor_z2_wxh!(highbd_dr_prediction_z2_32x32_neon, 32, 32);
highbd_dr_predictor_z2_wxh!(highbd_dr_prediction_z2_32x64_neon, 32, 64);
highbd_dr_predictor_z2_wxh!(highbd_dr_prediction_z2_64x16_neon, 64, 16);
highbd_dr_predictor_z2_wxh!(highbd_dr_prediction_z2_64x32_neon, 64, 32);
highbd_dr_predictor_z2_wxh!(highbd_dr_prediction_z2_64x64_neon, 64, 64);

type highbd_dr_prediction_z2_ptr =
    unsafe fn(*mut u16, isize, *const u16, *const u16, i32, i32, i32, i32, i32);

#[allow(clippy::too_many_arguments)]
unsafe fn highbd_dr_prediction_z2_4x4_neon(
    mut dst: *mut u16,
    stride: isize,
    above: *const u16,
    left: *const u16,
    upsample_above: i32,
    upsample_left: i32,
    dx: i32,
    dy: i32,
    _bd: i32,
) {
    debug_assert!(dx > 0);
    debug_assert!(dy > 0);

    let frac_bits_x = 6 - upsample_above;
    let frac_bits_y = 6 - upsample_left;
    let min_base_x = -(1 << (upsample_above + frac_bits_x));

    // if `upsample_left` then we need -2 through 6 inclusive from `left`.
    // else we only need -1 through 3 inclusive.

    let left_data0: uint16x8_t;
    let left_data1: uint16x8_t;
    if upsample_left != 0 {
        left_data0 = vld1q_u16(left.offset(-2));
        left_data1 = vld1q_u16(left.offset(-1));
    } else {
        left_data0 = vcombine_u16(vld1_u16(left.offset(-1)), vdup_n_u16(0));
        left_data1 = vcombine_u16(vld1_u16(left.offset(0)), vdup_n_u16(0));
    }

    let iota0123 = vld1_s16(iota1_s16.as_ptr());
    let iota1234 = vld1_s16(iota1_s16.as_ptr().add(1));

    for r in 0..4 {
        let base_shift = (min_base_x + (r + 1) * dx + 63) >> 6;
        let x0 = (r + 1) * dx;
        let x0123 = vsub_s16(vshl_n_s16::<6>(iota0123), vdup_n_s16(x0 as i16));
        let base_x0 = (-x0) >> frac_bits_x;
        if base_shift <= 0 {
            let a0: uint16x4_t;
            let a1: uint16x4_t;
            let shift_x0123: int16x4_t;
            if upsample_above != 0 {
                let a01 = vld2_u16(above.offset(base_x0 as isize));
                a0 = a01.0;
                a1 = a01.1;
                shift_x0123 = vand_s16(x0123, vdup_n_s16(0x1F));
            } else {
                a0 = vld1_u16(above.offset(base_x0 as isize));
                a1 = vld1_u16(above.offset(base_x0 as isize + 1));
                shift_x0123 = vshr_n_s16::<1>(vand_s16(x0123, vdup_n_s16(0x3F)));
            }
            vst1_u16(dst, highbd_dr_prediction_z2_apply_shift_x4(a0, a1, shift_x0123));
        } else if base_shift < 4 {
            // Calculate Y component from `left`.
            let y_iters = base_shift;
            let y0123 = vsub_s16(vdup_n_s16((r << 6) as i16), vmul_n_s16(iota1234, dy as i16));
            let base_y0123 = vshl_s16(y0123, vdup_n_s16(-frac_bits_y as i16));
            let shift_y0123 =
                vshr_n_s16::<1>(vand_s16(vmul_n_s16(y0123, (1 << upsample_left) as i16), vdup_n_s16(0x3F)));
            let left_data_base = if upsample_left != 0 { -2 } else { -1 };
            let l0 = highbd_dr_prediction_z2_tbl_left_x4_from_x8(left_data0, base_y0123, left_data_base, y_iters);
            let l1 = highbd_dr_prediction_z2_tbl_left_x4_from_x8(left_data1, base_y0123, left_data_base, y_iters);

            let out_y = highbd_dr_prediction_z2_apply_shift_x4(l0, l1, shift_y0123);

            // Calculate X component from `above`.
            let shift_x0123 =
                vshr_n_s16::<1>(vand_s16(vmul_n_s16(x0123, (1 << upsample_above) as i16), vdup_n_s16(0x3F)));
            let a0: uint16x4_t;
            let a1: uint16x4_t;
            if upsample_above != 0 {
                let a01 = vld2_u16(above.offset(if base_x0 % 2 == 0 { -2 } else { -1 }));
                a0 = a01.0;
                a1 = a01.1;
            } else {
                a0 = vld1_u16(above.offset(-1));
                a1 = vld1_u16(above.offset(0));
            }
            let out_x = highbd_dr_prediction_z2_apply_shift_x4(a0, a1, shift_x0123);

            // Combine X and Y vectors.
            let out = highbd_dr_prediction_z2_merge_x4(out_x, out_y, base_shift);
            vst1_u16(dst, out);
        } else {
            let y0123 = vsub_s16(vdup_n_s16((r << 6) as i16), vmul_n_s16(iota1234, dy as i16));
            let base_y0123 = vshl_s16(y0123, vdup_n_s16(-frac_bits_y as i16));
            let shift_y0123 =
                vshr_n_s16::<1>(vand_s16(vmul_n_s16(y0123, (1 << upsample_left) as i16), vdup_n_s16(0x3F)));
            let left_data_base = if upsample_left != 0 { -2 } else { -1 };
            let l0 = highbd_dr_prediction_z2_tbl_left_x4_from_x8(left_data0, base_y0123, left_data_base, 4);
            let l1 = highbd_dr_prediction_z2_tbl_left_x4_from_x8(left_data1, base_y0123, left_data_base, 4);
            vst1_u16(dst, highbd_dr_prediction_z2_apply_shift_x4(l0, l1, shift_y0123));
        }
        dst = dst.wrapping_offset(stride);
    }
}

#[allow(clippy::too_many_arguments)]
unsafe fn highbd_dr_prediction_z2_4x8_neon(
    mut dst: *mut u16,
    stride: isize,
    above: *const u16,
    left: *const u16,
    upsample_above: i32,
    upsample_left: i32,
    dx: i32,
    dy: i32,
    _bd: i32,
) {
    debug_assert!(dx > 0);
    debug_assert!(dy > 0);

    let frac_bits_x = 6 - upsample_above;
    let frac_bits_y = 6 - upsample_left;
    let min_base_x = -(1 << (upsample_above + frac_bits_x));

    // if `upsample_left` then we need -2 through 14 inclusive from `left`.
    // else we only need -1 through 6 inclusive.

    let left_data0: uint16x8x2_t;
    let left_data1: uint16x8x2_t;
    if upsample_left != 0 {
        left_data0 = vld1q_u16_x2(left.offset(-2));
        left_data1 = vld1q_u16_x2(left.offset(-1));
    } else {
        left_data0 = uint16x8x2_t(vld1q_u16(left.offset(-1)), vdupq_n_u16(0));
        left_data1 = uint16x8x2_t(vld1q_u16(left.offset(0)), vdupq_n_u16(0));
    }

    let iota0123 = vld1_s16(iota1_s16.as_ptr());
    let iota1234 = vld1_s16(iota1_s16.as_ptr().add(1));

    for r in 0..8 {
        let base_shift = (min_base_x + (r + 1) * dx + 63) >> 6;
        let x0 = (r + 1) * dx;
        let x0123 = vsub_s16(vshl_n_s16::<6>(iota0123), vdup_n_s16(x0 as i16));
        let base_x0 = (-x0) >> frac_bits_x;
        if base_shift <= 0 {
            let a0: uint16x4_t;
            let a1: uint16x4_t;
            let shift_x0123: int16x4_t;
            if upsample_above != 0 {
                let a01 = vld2_u16(above.offset(base_x0 as isize));
                a0 = a01.0;
                a1 = a01.1;
                shift_x0123 = vand_s16(x0123, vdup_n_s16(0x1F));
            } else {
                a0 = vld1_u16(above.offset(base_x0 as isize));
                a1 = vld1_u16(above.offset(base_x0 as isize + 1));
                shift_x0123 = vand_s16(vshr_n_s16::<1>(x0123), vdup_n_s16(0x1F));
            }
            vst1_u16(dst, highbd_dr_prediction_z2_apply_shift_x4(a0, a1, shift_x0123));
        } else if base_shift < 4 {
            // Calculate Y component from `left`.
            let y_iters = base_shift;
            let y0123 = vsub_s16(vdup_n_s16((r << 6) as i16), vmul_n_s16(iota1234, dy as i16));
            let base_y0123 = vshl_s16(y0123, vdup_n_s16(-frac_bits_y as i16));
            let shift_y0123 =
                vshr_n_s16::<1>(vand_s16(vmul_n_s16(y0123, (1 << upsample_left) as i16), vdup_n_s16(0x3F)));

            let left_data_base = if upsample_left != 0 { -2 } else { -1 };
            let l0 = highbd_dr_prediction_z2_tbl_left_x4_from_x16(left_data0, base_y0123, left_data_base, y_iters);
            let l1 = highbd_dr_prediction_z2_tbl_left_x4_from_x16(left_data1, base_y0123, left_data_base, y_iters);

            let out_y = highbd_dr_prediction_z2_apply_shift_x4(l0, l1, shift_y0123);

            // Calculate X component from `above`.
            let a0: uint16x4_t;
            let a1: uint16x4_t;
            let shift_x0123: int16x4_t;
            if upsample_above != 0 {
                let a01 = vld2_u16(above.offset(if base_x0 % 2 == 0 { -2 } else { -1 }));
                a0 = a01.0;
                a1 = a01.1;
                shift_x0123 = vand_s16(x0123, vdup_n_s16(0x1F));
            } else {
                a0 = vld1_u16(above.offset(-1));
                a1 = vld1_u16(above.offset(0));
                shift_x0123 = vand_s16(vshr_n_s16::<1>(x0123), vdup_n_s16(0x1F));
            }
            let out_x = highbd_dr_prediction_z2_apply_shift_x4(a0, a1, shift_x0123);

            // Combine X and Y vectors.
            let out = highbd_dr_prediction_z2_merge_x4(out_x, out_y, base_shift);
            vst1_u16(dst, out);
        } else {
            let y0123 = vsub_s16(vdup_n_s16((r << 6) as i16), vmul_n_s16(iota1234, dy as i16));
            let base_y0123 = vshl_s16(y0123, vdup_n_s16(-frac_bits_y as i16));
            let shift_y0123 =
                vshr_n_s16::<1>(vand_s16(vmul_n_s16(y0123, (1 << upsample_left) as i16), vdup_n_s16(0x3F)));

            let left_data_base = if upsample_left != 0 { -2 } else { -1 };
            let l0 = highbd_dr_prediction_z2_tbl_left_x4_from_x16(left_data0, base_y0123, left_data_base, 4);
            let l1 = highbd_dr_prediction_z2_tbl_left_x4_from_x16(left_data1, base_y0123, left_data_base, 4);

            vst1_u16(dst, highbd_dr_prediction_z2_apply_shift_x4(l0, l1, shift_y0123));
        }
        dst = dst.wrapping_offset(stride);
    }
}

#[allow(clippy::too_many_arguments)]
unsafe fn highbd_dr_prediction_z2_8x4_neon(
    mut dst: *mut u16,
    stride: isize,
    above: *const u16,
    left: *const u16,
    upsample_above: i32,
    upsample_left: i32,
    dx: i32,
    dy: i32,
    _bd: i32,
) {
    debug_assert!(dx > 0);
    debug_assert!(dy > 0);

    let frac_bits_x = 6 - upsample_above;
    let frac_bits_y = 6 - upsample_left;
    let min_base_x = -(1 << (upsample_above + frac_bits_x));

    // if `upsample_left` then we need -2 through 6 inclusive from `left`.
    // else we only need -1 through 3 inclusive.

    let left_data0: uint16x8_t;
    let left_data1: uint16x8_t;
    if upsample_left != 0 {
        left_data0 = vld1q_u16(left.offset(-2));
        left_data1 = vld1q_u16(left.offset(-1));
    } else {
        left_data0 = vcombine_u16(vld1_u16(left.offset(-1)), vdup_n_u16(0));
        left_data1 = vcombine_u16(vld1_u16(left.offset(0)), vdup_n_u16(0));
    }

    let iota01234567 = vld1q_s16(iota1_s16.as_ptr());
    let iota12345678 = vld1q_s16(iota1_s16.as_ptr().add(1));

    for r in 0..4 {
        let base_shift = (min_base_x + (r + 1) * dx + 63) >> 6;
        let x0 = (r + 1) * dx;
        let x01234567 = vsubq_s16(vshlq_n_s16::<6>(iota01234567), vdupq_n_s16(x0 as i16));
        let base_x0 = (-x0) >> frac_bits_x;
        if base_shift <= 0 {
            let a0: uint16x8_t;
            let a1: uint16x8_t;
            let shift_x01234567: int16x8_t;
            if upsample_above != 0 {
                let a01 = vld2q_u16(above.offset(base_x0 as isize));
                a0 = a01.0;
                a1 = a01.1;
                shift_x01234567 = vandq_s16(x01234567, vdupq_n_s16(0x1F));
            } else {
                a0 = vld1q_u16(above.offset(base_x0 as isize));
                a1 = vld1q_u16(above.offset(base_x0 as isize + 1));
                shift_x01234567 = vandq_s16(vshrq_n_s16::<1>(x01234567), vdupq_n_s16(0x1F));
            }
            vst1q_u16(dst, highbd_dr_prediction_z2_apply_shift_x8(a0, a1, shift_x01234567));
        } else if base_shift < 8 {
            // Calculate Y component from `left`.
            let y_iters = base_shift;
            let y01234567 = vsubq_s16(vdupq_n_s16((r << 6) as i16), vmulq_n_s16(iota12345678, dy as i16));
            let base_y01234567 = vshlq_s16(y01234567, vdupq_n_s16(-frac_bits_y as i16));
            let shift_y01234567 = vshrq_n_s16::<1>(vandq_s16(
                vmulq_n_s16(y01234567, (1 << upsample_left) as i16),
                vdupq_n_s16(0x3F),
            ));

            let left_data_base = if upsample_left != 0 { -2 } else { -1 };
            let l0 = highbd_dr_prediction_z2_tbl_left_x8_from_x8(left_data0, base_y01234567, left_data_base, y_iters);
            let l1 = highbd_dr_prediction_z2_tbl_left_x8_from_x8(left_data1, base_y01234567, left_data_base, y_iters);

            let out_y = highbd_dr_prediction_z2_apply_shift_x8(l0, l1, shift_y01234567);

            // Calculate X component from `above`.
            let a0: uint16x8_t;
            let a1: uint16x8_t;
            let shift_x01234567: int16x8_t;
            if upsample_above != 0 {
                let a01 = vld2q_u16(above.offset(if base_x0 % 2 == 0 { -2 } else { -1 }));
                a0 = a01.0;
                a1 = a01.1;
                shift_x01234567 = vandq_s16(x01234567, vdupq_n_s16(0x1F));
            } else {
                a0 = vld1q_u16(above.offset(-1));
                a1 = vld1q_u16(above.offset(0));
                shift_x01234567 = vandq_s16(vshrq_n_s16::<1>(x01234567), vdupq_n_s16(0x1F));
            }
            let out_x = highbd_dr_prediction_z2_apply_shift_x8(a0, a1, shift_x01234567);

            // Combine X and Y vectors.
            let out = highbd_dr_prediction_z2_merge_x8(out_x, out_y, base_shift);
            vst1q_u16(dst, out);
        } else {
            let y01234567 = vsubq_s16(vdupq_n_s16((r << 6) as i16), vmulq_n_s16(iota12345678, dy as i16));
            let base_y01234567 = vshlq_s16(y01234567, vdupq_n_s16(-frac_bits_y as i16));
            let shift_y01234567 = vshrq_n_s16::<1>(vandq_s16(
                vmulq_n_s16(y01234567, (1 << upsample_left) as i16),
                vdupq_n_s16(0x3F),
            ));

            let left_data_base = if upsample_left != 0 { -2 } else { -1 };
            let l0 = highbd_dr_prediction_z2_tbl_left_x8_from_x8(left_data0, base_y01234567, left_data_base, 8);
            let l1 = highbd_dr_prediction_z2_tbl_left_x8_from_x8(left_data1, base_y01234567, left_data_base, 8);

            vst1q_u16(dst, highbd_dr_prediction_z2_apply_shift_x8(l0, l1, shift_y01234567));
        }
        dst = dst.wrapping_offset(stride);
    }
}

#[allow(clippy::too_many_arguments)]
unsafe fn highbd_dr_prediction_z2_8x8_neon(
    mut dst: *mut u16,
    stride: isize,
    above: *const u16,
    left: *const u16,
    upsample_above: i32,
    upsample_left: i32,
    dx: i32,
    dy: i32,
    _bd: i32,
) {
    debug_assert!(dx > 0);
    debug_assert!(dy > 0);

    let frac_bits_x = 6 - upsample_above;
    let frac_bits_y = 6 - upsample_left;
    let min_base_x = -(1 << (upsample_above + frac_bits_x));

    // if `upsample_left` then we need -2 through 14 inclusive from `left`.
    // else we only need -1 through 6 inclusive.

    let left_data0: uint16x8x2_t;
    let left_data1: uint16x8x2_t;
    if upsample_left != 0 {
        left_data0 = vld1q_u16_x2(left.offset(-2));
        left_data1 = vld1q_u16_x2(left.offset(-1));
    } else {
        left_data0 = uint16x8x2_t(vld1q_u16(left.offset(-1)), vdupq_n_u16(0));
        left_data1 = uint16x8x2_t(vld1q_u16(left.offset(0)), vdupq_n_u16(0));
    }

    let iota01234567 = vld1q_s16(iota1_s16.as_ptr());
    let iota12345678 = vld1q_s16(iota1_s16.as_ptr().add(1));

    for r in 0..8 {
        let base_shift = (min_base_x + (r + 1) * dx + 63) >> 6;
        let x0 = (r + 1) * dx;
        let x01234567 = vsubq_s16(vshlq_n_s16::<6>(iota01234567), vdupq_n_s16(x0 as i16));
        let base_x0 = (-x0) >> frac_bits_x;
        if base_shift <= 0 {
            let a0: uint16x8_t;
            let a1: uint16x8_t;
            let shift_x01234567: int16x8_t;
            if upsample_above != 0 {
                let a01 = vld2q_u16(above.offset(base_x0 as isize));
                a0 = a01.0;
                a1 = a01.1;
                shift_x01234567 = vandq_s16(x01234567, vdupq_n_s16(0x1F));
            } else {
                a0 = vld1q_u16(above.offset(base_x0 as isize));
                a1 = vld1q_u16(above.offset(base_x0 as isize + 1));
                shift_x01234567 = vandq_s16(vshrq_n_s16::<1>(x01234567), vdupq_n_s16(0x1F));
            }
            vst1q_u16(dst, highbd_dr_prediction_z2_apply_shift_x8(a0, a1, shift_x01234567));
        } else if base_shift < 8 {
            // Calculate Y component from `left`.
            let y_iters = base_shift;
            let y01234567 = vsubq_s16(vdupq_n_s16((r << 6) as i16), vmulq_n_s16(iota12345678, dy as i16));
            let base_y01234567 = vshlq_s16(y01234567, vdupq_n_s16(-frac_bits_y as i16));
            let shift_y01234567 = vshrq_n_s16::<1>(vandq_s16(
                vmulq_n_s16(y01234567, (1 << upsample_left) as i16),
                vdupq_n_s16(0x3F),
            ));

            let left_data_base = if upsample_left != 0 { -2 } else { -1 };
            let l0 = highbd_dr_prediction_z2_tbl_left_x8_from_x16(left_data0, base_y01234567, left_data_base, y_iters);
            let l1 = highbd_dr_prediction_z2_tbl_left_x8_from_x16(left_data1, base_y01234567, left_data_base, y_iters);

            let out_y = highbd_dr_prediction_z2_apply_shift_x8(l0, l1, shift_y01234567);

            // Calculate X component from `above`.
            let a0: uint16x8_t;
            let a1: uint16x8_t;
            let shift_x01234567: int16x8_t;
            if upsample_above != 0 {
                let a01 = vld2q_u16(above.offset(if base_x0 % 2 == 0 { -2 } else { -1 }));
                a0 = a01.0;
                a1 = a01.1;
                shift_x01234567 = vandq_s16(x01234567, vdupq_n_s16(0x1F));
            } else {
                a0 = vld1q_u16(above.offset(-1));
                a1 = vld1q_u16(above.offset(0));
                shift_x01234567 = vandq_s16(vshrq_n_s16::<1>(x01234567), vdupq_n_s16(0x1F));
            }
            let out_x = highbd_dr_prediction_z2_apply_shift_x8(a0, a1, shift_x01234567);

            // Combine X and Y vectors.
            let out = highbd_dr_prediction_z2_merge_x8(out_x, out_y, base_shift);
            vst1q_u16(dst, out);
        } else {
            let y01234567 = vsubq_s16(vdupq_n_s16((r << 6) as i16), vmulq_n_s16(iota12345678, dy as i16));
            let base_y01234567 = vshlq_s16(y01234567, vdupq_n_s16(-frac_bits_y as i16));
            let shift_y01234567 = vshrq_n_s16::<1>(vandq_s16(
                vmulq_n_s16(y01234567, (1 << upsample_left) as i16),
                vdupq_n_s16(0x3F),
            ));

            let left_data_base = if upsample_left != 0 { -2 } else { -1 };
            let l0 = highbd_dr_prediction_z2_tbl_left_x8_from_x16(left_data0, base_y01234567, left_data_base, 8);
            let l1 = highbd_dr_prediction_z2_tbl_left_x8_from_x16(left_data1, base_y01234567, left_data_base, 8);

            vst1q_u16(dst, highbd_dr_prediction_z2_apply_shift_x8(l0, l1, shift_y01234567));
        }
        dst = dst.wrapping_offset(stride);
    }
}

#[rustfmt::skip]
static dr_predictor_z2_arr_neon: [[Option<highbd_dr_prediction_z2_ptr>; 7]; 7] = [
    [ None, None, None, None, None, None, None ],
    [ None, None, None, None, None, None, None ],
    [ None, None, Some(highbd_dr_prediction_z2_4x4_neon),
      Some(highbd_dr_prediction_z2_4x8_neon), Some(highbd_dr_prediction_z2_4x16_neon), None,
      None ],
    [ None, None, Some(highbd_dr_prediction_z2_8x4_neon),
      Some(highbd_dr_prediction_z2_8x8_neon), Some(highbd_dr_prediction_z2_8x16_neon),
      Some(highbd_dr_prediction_z2_8x32_neon), None ],
    [ None, None, Some(highbd_dr_prediction_z2_16x4_neon),
      Some(highbd_dr_prediction_z2_16x8_neon), Some(highbd_dr_prediction_z2_16x16_neon),
      Some(highbd_dr_prediction_z2_16x32_neon), Some(highbd_dr_prediction_z2_16x64_neon) ],
    [ None, None, None, Some(highbd_dr_prediction_z2_32x8_neon),
      Some(highbd_dr_prediction_z2_32x16_neon), Some(highbd_dr_prediction_z2_32x32_neon),
      Some(highbd_dr_prediction_z2_32x64_neon) ],
    [ None, None, None, None, Some(highbd_dr_prediction_z2_64x16_neon),
      Some(highbd_dr_prediction_z2_64x32_neon), Some(highbd_dr_prediction_z2_64x64_neon) ],
];

// Directional prediction, zone 2: 90 < angle < 180
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_highbd_dr_prediction_z2_neon(
    dst: *mut u16,
    stride: isize,
    bw: i32,
    bh: i32,
    above: *const u16,
    left: *const u16,
    upsample_above: i32,
    upsample_left: i32,
    dx: i32,
    dy: i32,
    bd: i32,
) {
    let f = dr_predictor_z2_arr_neon[get_msb(bw as u32) as usize][get_msb(bh as u32) as usize];
    debug_assert!(f.is_some());
    let f = f.unwrap_unchecked();
    f(dst, stride, above, left, upsample_above, upsample_left, dx, dy, bd);
}

// -----------------------------------------------------------------------------
// Z3

// Both the lane to the use and the shift amount must be immediates.
// #define HIGHBD_DR_PREDICTOR_Z3_STEP_X4(out, iota, base, in0, in1, s0, s1, lane, shift)
// (`max_base_y` and `left_max` are captured from the caller in the original.)
macro_rules! highbd_dr_predictor_z3_step_x4 {
    ($out:expr, $iota:expr, $base:expr, $in0:expr, $in1:expr, $s0:expr, $s1:expr, $lane:literal, $shift:literal,
     $max_base_y:expr, $left_max:expr) => {{
        let mut val = vmull_lane_u16::<$lane>($in0, $s0);
        val = vmlal_lane_u16::<$lane>(val, $in1, $s1);
        let cmp = vadd_u16($iota, vdup_n_u16($base as u16));
        let res = vrshrn_n_u32::<$shift>(val);
        *($out) = vbsl_u16(vclt_u16(cmp, vdup_n_u16($max_base_y as u16)), res, vdup_n_u16($left_max as u16));
    }};
}

// #define HIGHBD_DR_PREDICTOR_Z3_STEP_X8(out, iota, base, in0, in1, s0, s1, lane, shift)
macro_rules! highbd_dr_predictor_z3_step_x8 {
    ($out:expr, $iota:expr, $base:expr, $in0:expr, $in1:expr, $s0:expr, $s1:expr, $lane:literal, $shift:literal) => {{
        let _ = ($iota, $base);
        let mut val_lo = vmull_lane_u16::<$lane>(vget_low_u16($in0), $s0);
        val_lo = vmlal_lane_u16::<$lane>(val_lo, vget_low_u16($in1), $s1);
        let mut val_hi = vmull_lane_u16::<$lane>(vget_high_u16($in0), $s0);
        val_hi = vmlal_lane_u16::<$lane>(val_hi, vget_high_u16($in1), $s1);
        *($out) = vcombine_u16(vrshrn_n_u32::<$shift>(val_lo), vrshrn_n_u32::<$shift>(val_hi));
    }};
}

#[inline(always)]
unsafe fn z3_load_left_neon(left0: *const u16, ofs: i32, max_ofs: i32) -> uint16x8x2_t {
    let r0: uint16x8_t;
    let r1: uint16x8_t;
    if ofs + 7 >= max_ofs {
        let shuffle_idx = max_ofs - ofs;
        r0 = zn_load_masked_neon(left0.offset((max_ofs - 7) as isize), shuffle_idx);
    } else {
        r0 = vld1q_u16(left0.offset(ofs as isize));
    }
    if ofs + 8 >= max_ofs {
        let shuffle_idx = max_ofs - ofs - 1;
        r1 = zn_load_masked_neon(left0.offset((max_ofs - 7) as isize), shuffle_idx);
    } else {
        r1 = vld1q_u16(left0.offset(ofs as isize + 1));
    }
    uint16x8x2_t(r0, r1)
}

unsafe fn highbd_dr_prediction_z3_upsample0_neon(
    dst: *mut u16,
    stride: isize,
    bw: i32,
    bh: i32,
    left: *const u16,
    dy: i32,
) {
    debug_assert!(bw % 4 == 0);
    debug_assert!(bh % 4 == 0);
    debug_assert!(dy > 0);

    // Factor out left + 1 to give the compiler a better chance of recognising
    // that the offsets used for the loads from left and left + 1 are otherwise
    // identical.
    let left1 = left.add(1);

    let max_base_y = bw + bh - 1;
    let left_max = *left.offset(max_base_y as isize) as i32;
    let frac_bits = 6;

    let iota1x8 = vreinterpretq_u16_s16(vld1q_s16(iota1_s16.as_ptr()));
    let iota1x4 = vget_low_u16(iota1x8);

    // The C implementation of the z3 predictor when not upsampling uses:
    // ((y & 0x3f) >> 1)
    // The right shift is unnecessary here since we instead shift by +1 later,
    // so adjust the mask to 0x3e to ensure we don't consider the extra bit.
    let shift_mask = vdup_n_u16(0x3e);

    if bh == 4 {
        let mut y = dy;
        let mut c = 0;
        loop {
            // Fully unroll the 4x4 block to allow us to use immediate lane-indexed
            // multiply instructions.
            let shifts1 = vand_u16(vmla_n_u16(vdup_n_u16(y as u16), iota1x4, dy as u16), shift_mask);
            let shifts0 = vsub_u16(vdup_n_u16(64), shifts1);
            let base0 = (y + 0 * dy) >> frac_bits;
            let base1 = (y + 1 * dy) >> frac_bits;
            let base2 = (y + 2 * dy) >> frac_bits;
            let base3 = (y + 3 * dy) >> frac_bits;
            let mut out: [uint16x4_t; 4] = [vdup_n_u16(0); 4];
            if base0 >= max_base_y {
                out[0] = vdup_n_u16(left_max as u16);
            } else {
                let l00 = vld1_u16(left.offset(base0 as isize));
                let l01 = vld1_u16(left1.offset(base0 as isize));
                highbd_dr_predictor_z3_step_x4!(
                    &mut out[0], iota1x4, base0, l00, l01, shifts0, shifts1, 0, 6, max_base_y, left_max
                );
            }
            if base1 >= max_base_y {
                out[1] = vdup_n_u16(left_max as u16);
            } else {
                let l10 = vld1_u16(left.offset(base1 as isize));
                let l11 = vld1_u16(left1.offset(base1 as isize));
                highbd_dr_predictor_z3_step_x4!(
                    &mut out[1], iota1x4, base1, l10, l11, shifts0, shifts1, 1, 6, max_base_y, left_max
                );
            }
            if base2 >= max_base_y {
                out[2] = vdup_n_u16(left_max as u16);
            } else {
                let l20 = vld1_u16(left.offset(base2 as isize));
                let l21 = vld1_u16(left1.offset(base2 as isize));
                highbd_dr_predictor_z3_step_x4!(
                    &mut out[2], iota1x4, base2, l20, l21, shifts0, shifts1, 2, 6, max_base_y, left_max
                );
            }
            if base3 >= max_base_y {
                out[3] = vdup_n_u16(left_max as u16);
            } else {
                let l30 = vld1_u16(left.offset(base3 as isize));
                let l31 = vld1_u16(left1.offset(base3 as isize));
                highbd_dr_predictor_z3_step_x4!(
                    &mut out[3], iota1x4, base3, l30, l31, shifts0, shifts1, 3, 6, max_base_y, left_max
                );
            }
            transpose_array_inplace_u16_4x4(out.as_mut_ptr());
            for r2 in 0..4isize {
                vst1_u16(dst.offset(r2 * stride + c as isize), out[r2 as usize]);
            }
            y += 4 * dy;
            c += 4;
            if c >= bw {
                break;
            }
        }
    } else {
        let mut y = dy;
        let mut c = 0;
        loop {
            let mut r = 0;
            loop {
                // Fully unroll the 4x4 block to allow us to use immediate lane-indexed
                // multiply instructions.
                let shifts1 = vand_u16(vmla_n_u16(vdup_n_u16(y as u16), iota1x4, dy as u16), shift_mask);
                let shifts0 = vsub_u16(vdup_n_u16(64), shifts1);
                let base0 = ((y + 0 * dy) >> frac_bits) + r;
                let base1 = ((y + 1 * dy) >> frac_bits) + r;
                let base2 = ((y + 2 * dy) >> frac_bits) + r;
                let base3 = ((y + 3 * dy) >> frac_bits) + r;
                let mut out: [uint16x8_t; 4] = [vdupq_n_u16(0); 4];
                if base0 >= max_base_y {
                    out[0] = vdupq_n_u16(left_max as u16);
                } else {
                    let l0 = z3_load_left_neon(left, base0, max_base_y);
                    highbd_dr_predictor_z3_step_x8!(&mut out[0], iota1x8, base0, l0.0, l0.1, shifts0, shifts1, 0, 6);
                }
                if base1 >= max_base_y {
                    out[1] = vdupq_n_u16(left_max as u16);
                } else {
                    let l1 = z3_load_left_neon(left, base1, max_base_y);
                    highbd_dr_predictor_z3_step_x8!(&mut out[1], iota1x8, base1, l1.0, l1.1, shifts0, shifts1, 1, 6);
                }
                if base2 >= max_base_y {
                    out[2] = vdupq_n_u16(left_max as u16);
                } else {
                    let l2 = z3_load_left_neon(left, base2, max_base_y);
                    highbd_dr_predictor_z3_step_x8!(&mut out[2], iota1x8, base2, l2.0, l2.1, shifts0, shifts1, 2, 6);
                }
                if base3 >= max_base_y {
                    out[3] = vdupq_n_u16(left_max as u16);
                } else {
                    let l3 = z3_load_left_neon(left, base3, max_base_y);
                    highbd_dr_predictor_z3_step_x8!(&mut out[3], iota1x8, base3, l3.0, l3.1, shifts0, shifts1, 3, 6);
                }
                transpose_array_inplace_u16_4x8(out.as_mut_ptr());
                for r2 in 0..4isize {
                    vst1_u16(dst.offset((r as isize + r2) * stride + c as isize), vget_low_u16(out[r2 as usize]));
                }
                for r2 in 0..4isize {
                    vst1_u16(
                        dst.offset((r as isize + r2 + 4) * stride + c as isize),
                        vget_high_u16(out[r2 as usize]),
                    );
                }
                r += 8;
                if r >= bh {
                    break;
                }
            }
            y += 4 * dy;
            c += 4;
            if c >= bw {
                break;
            }
        }
    }
}

unsafe fn highbd_dr_prediction_z3_upsample1_neon(
    dst: *mut u16,
    stride: isize,
    bw: i32,
    bh: i32,
    left: *const u16,
    dy: i32,
) {
    debug_assert!(bw % 4 == 0);
    debug_assert!(bh % 4 == 0);
    debug_assert!(dy > 0);

    let max_base_y = (bw + bh - 1) << 1;
    let left_max = *left.offset(max_base_y as isize) as i32;
    let frac_bits = 5;

    let iota1x4 = vreinterpret_u16_s16(vld1_s16(iota1_s16.as_ptr()));
    let iota2x8 = vreinterpretq_u16_s16(vld1q_s16(iota2_s16.as_ptr()));
    let iota2x4 = vget_low_u16(iota2x8);

    // The C implementation of the z3 predictor when upsampling uses:
    // (((x << 1) & 0x3f) >> 1)
    // The two shifts are unnecessary here since the lowest bit is guaranteed to
    // be zero when the mask is applied, so adjust the mask to 0x1f to avoid
    // needing the shifts at all.
    let shift_mask = vdup_n_u16(0x1F);

    if bh == 4 {
        let mut y = dy;
        let mut c = 0;
        loop {
            // Fully unroll the 4x4 block to allow us to use immediate lane-indexed
            // multiply instructions.
            let shifts1 = vand_u16(vmla_n_u16(vdup_n_u16(y as u16), iota1x4, dy as u16), shift_mask);
            let shifts0 = vsub_u16(vdup_n_u16(32), shifts1);
            let base0 = (y + 0 * dy) >> frac_bits;
            let base1 = (y + 1 * dy) >> frac_bits;
            let base2 = (y + 2 * dy) >> frac_bits;
            let base3 = (y + 3 * dy) >> frac_bits;
            let l0 = vld2_u16(left.offset(base0 as isize));
            let l1 = vld2_u16(left.offset(base1 as isize));
            let l2 = vld2_u16(left.offset(base2 as isize));
            let l3 = vld2_u16(left.offset(base3 as isize));
            let mut out: [uint16x4_t; 4] = [vdup_n_u16(0); 4];
            highbd_dr_predictor_z3_step_x4!(
                &mut out[0], iota2x4, base0, l0.0, l0.1, shifts0, shifts1, 0, 5, max_base_y, left_max
            );
            highbd_dr_predictor_z3_step_x4!(
                &mut out[1], iota2x4, base1, l1.0, l1.1, shifts0, shifts1, 1, 5, max_base_y, left_max
            );
            highbd_dr_predictor_z3_step_x4!(
                &mut out[2], iota2x4, base2, l2.0, l2.1, shifts0, shifts1, 2, 5, max_base_y, left_max
            );
            highbd_dr_predictor_z3_step_x4!(
                &mut out[3], iota2x4, base3, l3.0, l3.1, shifts0, shifts1, 3, 5, max_base_y, left_max
            );
            transpose_array_inplace_u16_4x4(out.as_mut_ptr());
            for r2 in 0..4isize {
                vst1_u16(dst.offset(r2 * stride + c as isize), out[r2 as usize]);
            }
            y += 4 * dy;
            c += 4;
            if c >= bw {
                break;
            }
        }
    } else {
        debug_assert!(bh % 8 == 0);

        let mut y = dy;
        let mut c = 0;
        loop {
            let mut r = 0;
            loop {
                // Fully unroll the 4x8 block to allow us to use immediate lane-indexed
                // multiply instructions.
                let shifts1 = vand_u16(vmla_n_u16(vdup_n_u16(y as u16), iota1x4, dy as u16), shift_mask);
                let shifts0 = vsub_u16(vdup_n_u16(32), shifts1);
                let base0 = ((y + 0 * dy) >> frac_bits) + (r * 2);
                let base1 = ((y + 1 * dy) >> frac_bits) + (r * 2);
                let base2 = ((y + 2 * dy) >> frac_bits) + (r * 2);
                let base3 = ((y + 3 * dy) >> frac_bits) + (r * 2);
                let l0 = vld2q_u16(left.offset(base0 as isize));
                let l1 = vld2q_u16(left.offset(base1 as isize));
                let l2 = vld2q_u16(left.offset(base2 as isize));
                let l3 = vld2q_u16(left.offset(base3 as isize));
                let mut out: [uint16x8_t; 4] = [vdupq_n_u16(0); 4];
                highbd_dr_predictor_z3_step_x8!(&mut out[0], iota2x8, base0, l0.0, l0.1, shifts0, shifts1, 0, 5);
                highbd_dr_predictor_z3_step_x8!(&mut out[1], iota2x8, base1, l1.0, l1.1, shifts0, shifts1, 1, 5);
                highbd_dr_predictor_z3_step_x8!(&mut out[2], iota2x8, base2, l2.0, l2.1, shifts0, shifts1, 2, 5);
                highbd_dr_predictor_z3_step_x8!(&mut out[3], iota2x8, base3, l3.0, l3.1, shifts0, shifts1, 3, 5);
                transpose_array_inplace_u16_4x8(out.as_mut_ptr());
                for r2 in 0..4isize {
                    vst1_u16(dst.offset((r as isize + r2) * stride + c as isize), vget_low_u16(out[r2 as usize]));
                }
                for r2 in 0..4isize {
                    vst1_u16(
                        dst.offset((r as isize + r2 + 4) * stride + c as isize),
                        vget_high_u16(out[r2 as usize]),
                    );
                }
                r += 8;
                if r >= bh {
                    break;
                }
            }
            y += 4 * dy;
            c += 4;
            if c >= bw {
                break;
            }
        }
    }
}

// Directional prediction, zone 3: 180 < angle < 270
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_highbd_dr_prediction_z3_neon(
    dst: *mut u16,
    stride: isize,
    bw: i32,
    bh: i32,
    _above: *const u16,
    left: *const u16,
    upsample_left: i32,
    dx: i32,
    dy: i32,
    _bd: i32,
) {
    debug_assert!(bw % 4 == 0);
    debug_assert!(bh % 4 == 0);
    debug_assert!(dx == 1);
    debug_assert!(dy > 0);
    let _ = dx;

    if upsample_left != 0 {
        highbd_dr_prediction_z3_upsample1_neon(dst, stride, bw, bh, left, dy);
    } else {
        highbd_dr_prediction_z3_upsample0_neon(dst, stride, bw, bh, left, dy);
    }
}
