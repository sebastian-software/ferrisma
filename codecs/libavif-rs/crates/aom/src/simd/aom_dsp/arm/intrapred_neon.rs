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

// Translated by hand from upstream/aom/aom_dsp/arm/intrapred_neon.c
// (ADR 0056). Intrinsic sequences, helper structure, and names follow the
// original; only the AOM_ARCH_AARCH64 branches are retained, and the
// `!CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER` branches are taken (the port
// builds the encoder and decoder with CONFIG_REALTIME_ONLY=0).
//
// Translation notes:
// - The function-generating C macros become macro_rules! that receive the
//   generated function and helper names explicitly.
// - `smooth_weights` (aom_dsp/intrapred_common.h) is header-local in the
//   original and is transcribed below.
// - C locals that the original leaves uninitialized start zeroed here.
#![allow(non_snake_case, non_upper_case_globals, unused_assignments, unused_mut, unused_parens)]
#![allow(unused_variables, dead_code, clippy::all)]

use core::arch::aarch64::*;

use crate::simd::aom_dsp__arm__mem_neon::*;
use crate::simd::aom_dsp__arm__sum_neon::*;
use crate::simd::aom_dsp__arm__transpose_neon::*;

//------------------------------------------------------------------------------
// DC 4x4

#[inline(always)]
unsafe fn dc_load_sum_4(in_: *const u8) -> uint16x8_t {
    let a = load_u8_4x1(in_);
    let p0 = vpaddl_u8(a);
    let p1 = vpadd_u16(p0, p0);
    vcombine_u16(p1, vdup_n_u16(0))
}

#[inline(always)]
unsafe fn dc_store_4xh(dst: *mut u8, stride: isize, h: i32, dc: uint8x8_t) {
    for i in 0..h {
        store_u8_4x1(dst.offset(i as isize * stride), dc);
    }
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_predictor_4x4_neon(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let sum_top = dc_load_sum_4(above);
    let sum_left = dc_load_sum_4(left);
    let sum = vaddq_u16(sum_left, sum_top);
    let dc0 = vrshrn_n_u16::<3>(sum);
    dc_store_4xh(dst, stride, 4, vdup_lane_u8::<0>(dc0));
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_left_predictor_4x4_neon(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let sum_left = dc_load_sum_4(left);
    let dc0 = vrshrn_n_u16::<2>(sum_left);
    dc_store_4xh(dst, stride, 4, vdup_lane_u8::<0>(dc0));
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_top_predictor_4x4_neon(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let sum_top = dc_load_sum_4(above);
    let dc0 = vrshrn_n_u16::<2>(sum_top);
    dc_store_4xh(dst, stride, 4, vdup_lane_u8::<0>(dc0));
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_128_predictor_4x4_neon(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let dc0 = vdup_n_u8(0x80);
    dc_store_4xh(dst, stride, 4, dc0);
}

//------------------------------------------------------------------------------
// DC 8x8

#[inline(always)]
unsafe fn dc_load_sum_8(in_: *const u8) -> uint16x8_t {
    // This isn't used in the case where we want to load both above and left
    // vectors, since we want to avoid performing the reduction twice.
    let a = vld1_u8(in_);
    let p0 = vpaddl_u8(a);
    let p1 = vpadd_u16(p0, p0);
    let p2 = vpadd_u16(p1, p1);
    vcombine_u16(p2, vdup_n_u16(0))
}

#[inline(always)]
unsafe fn horizontal_add_and_broadcast_u16x8(a: uint16x8_t) -> uint16x8_t {
    // On AArch64 we could also use vdupq_n_u16(vaddvq_u16(a)) here to save an
    // instruction, however the addv instruction is usually slightly more
    // expensive than a pairwise addition, so the need for immediately
    // broadcasting the result again seems to negate any benefit.
    let b = vpaddq_u16(a, a);
    let c = vpaddq_u16(b, b);
    vpaddq_u16(c, c)
}

#[inline(always)]
unsafe fn dc_store_8xh(dst: *mut u8, stride: isize, h: i32, dc: uint8x8_t) {
    for i in 0..h {
        vst1_u8(dst.offset(i as isize * stride), dc);
    }
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_predictor_8x8_neon(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let sum_top = vld1_u8(above);
    let sum_left = vld1_u8(left);
    let mut sum = vaddl_u8(sum_left, sum_top);
    sum = horizontal_add_and_broadcast_u16x8(sum);
    let dc0 = vrshrn_n_u16::<4>(sum);
    dc_store_8xh(dst, stride, 8, vdup_lane_u8::<0>(dc0));
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_left_predictor_8x8_neon(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let sum_left = dc_load_sum_8(left);
    let dc0 = vrshrn_n_u16::<3>(sum_left);
    dc_store_8xh(dst, stride, 8, vdup_lane_u8::<0>(dc0));
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_top_predictor_8x8_neon(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let sum_top = dc_load_sum_8(above);
    let dc0 = vrshrn_n_u16::<3>(sum_top);
    dc_store_8xh(dst, stride, 8, vdup_lane_u8::<0>(dc0));
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_128_predictor_8x8_neon(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let dc0 = vdup_n_u8(0x80);
    dc_store_8xh(dst, stride, 8, dc0);
}

//------------------------------------------------------------------------------
// DC 16x16

#[inline(always)]
unsafe fn dc_load_partial_sum_16(in_: *const u8) -> uint16x8_t {
    let a = vld1q_u8(in_);
    // delay the remainder of the reduction until
    // horizontal_add_and_broadcast_u16x8, since we want to do it once rather
    // than twice in the case we are loading both above and left.
    vpaddlq_u8(a)
}

#[inline(always)]
unsafe fn dc_load_sum_16(in_: *const u8) -> uint16x8_t {
    horizontal_add_and_broadcast_u16x8(dc_load_partial_sum_16(in_))
}

#[inline(always)]
unsafe fn dc_store_16xh(dst: *mut u8, stride: isize, h: i32, dc: uint8x16_t) {
    for i in 0..h {
        vst1q_u8(dst.offset(i as isize * stride), dc);
    }
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_predictor_16x16_neon(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let sum_top = dc_load_partial_sum_16(above);
    let sum_left = dc_load_partial_sum_16(left);
    let mut sum = vaddq_u16(sum_left, sum_top);
    sum = horizontal_add_and_broadcast_u16x8(sum);
    let dc0 = vrshrn_n_u16::<5>(sum);
    dc_store_16xh(dst, stride, 16, vdupq_lane_u8::<0>(dc0));
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_left_predictor_16x16_neon(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let sum_left = dc_load_sum_16(left);
    let dc0 = vrshrn_n_u16::<4>(sum_left);
    dc_store_16xh(dst, stride, 16, vdupq_lane_u8::<0>(dc0));
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_top_predictor_16x16_neon(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let sum_top = dc_load_sum_16(above);
    let dc0 = vrshrn_n_u16::<4>(sum_top);
    dc_store_16xh(dst, stride, 16, vdupq_lane_u8::<0>(dc0));
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_128_predictor_16x16_neon(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let dc0 = vdupq_n_u8(0x80);
    dc_store_16xh(dst, stride, 16, dc0);
}

//------------------------------------------------------------------------------
// DC 32x32

#[inline(always)]
unsafe fn dc_load_partial_sum_32(in_: *const u8) -> uint16x8_t {
    let a0 = vld1q_u8(in_);
    let a1 = vld1q_u8(in_.add(16));
    // delay the remainder of the reduction until
    // horizontal_add_and_broadcast_u16x8, since we want to do it once rather
    // than twice in the case we are loading both above and left.
    vpadalq_u8(vpaddlq_u8(a0), a1)
}

#[inline(always)]
unsafe fn dc_load_sum_32(in_: *const u8) -> uint16x8_t {
    horizontal_add_and_broadcast_u16x8(dc_load_partial_sum_32(in_))
}

#[inline(always)]
unsafe fn dc_store_32xh(dst: *mut u8, stride: isize, h: i32, dc: uint8x16_t) {
    for i in 0..h {
        vst1q_u8(dst.offset(i as isize * stride), dc);
        vst1q_u8(dst.offset(i as isize * stride + 16), dc);
    }
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_predictor_32x32_neon(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let sum_top = dc_load_partial_sum_32(above);
    let sum_left = dc_load_partial_sum_32(left);
    let mut sum = vaddq_u16(sum_left, sum_top);
    sum = horizontal_add_and_broadcast_u16x8(sum);
    let dc0 = vrshrn_n_u16::<6>(sum);
    dc_store_32xh(dst, stride, 32, vdupq_lane_u8::<0>(dc0));
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_left_predictor_32x32_neon(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let sum_left = dc_load_sum_32(left);
    let dc0 = vrshrn_n_u16::<5>(sum_left);
    dc_store_32xh(dst, stride, 32, vdupq_lane_u8::<0>(dc0));
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_top_predictor_32x32_neon(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let sum_top = dc_load_sum_32(above);
    let dc0 = vrshrn_n_u16::<5>(sum_top);
    dc_store_32xh(dst, stride, 32, vdupq_lane_u8::<0>(dc0));
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_128_predictor_32x32_neon(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let dc0 = vdupq_n_u8(0x80);
    dc_store_32xh(dst, stride, 32, dc0);
}

//------------------------------------------------------------------------------
// DC 64x64

#[inline(always)]
unsafe fn dc_load_partial_sum_64(in_: *const u8) -> uint16x8_t {
    let a0 = vld1q_u8(in_);
    let a1 = vld1q_u8(in_.add(16));
    let a2 = vld1q_u8(in_.add(32));
    let a3 = vld1q_u8(in_.add(48));
    let p01 = vpadalq_u8(vpaddlq_u8(a0), a1);
    let p23 = vpadalq_u8(vpaddlq_u8(a2), a3);
    // delay the remainder of the reduction until
    // horizontal_add_and_broadcast_u16x8, since we want to do it once rather
    // than twice in the case we are loading both above and left.
    vaddq_u16(p01, p23)
}

#[inline(always)]
unsafe fn dc_load_sum_64(in_: *const u8) -> uint16x8_t {
    horizontal_add_and_broadcast_u16x8(dc_load_partial_sum_64(in_))
}

#[inline(always)]
unsafe fn dc_store_64xh(dst: *mut u8, stride: isize, h: i32, dc: uint8x16_t) {
    for i in 0..h {
        vst1q_u8(dst.offset(i as isize * stride), dc);
        vst1q_u8(dst.offset(i as isize * stride + 16), dc);
        vst1q_u8(dst.offset(i as isize * stride + 32), dc);
        vst1q_u8(dst.offset(i as isize * stride + 48), dc);
    }
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_predictor_64x64_neon(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let sum_top = dc_load_partial_sum_64(above);
    let sum_left = dc_load_partial_sum_64(left);
    let mut sum = vaddq_u16(sum_left, sum_top);
    sum = horizontal_add_and_broadcast_u16x8(sum);
    let dc0 = vrshrn_n_u16::<7>(sum);
    dc_store_64xh(dst, stride, 64, vdupq_lane_u8::<0>(dc0));
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_left_predictor_64x64_neon(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let sum_left = dc_load_sum_64(left);
    let dc0 = vrshrn_n_u16::<6>(sum_left);
    dc_store_64xh(dst, stride, 64, vdupq_lane_u8::<0>(dc0));
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_top_predictor_64x64_neon(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let sum_top = dc_load_sum_64(above);
    let dc0 = vrshrn_n_u16::<6>(sum_top);
    dc_store_64xh(dst, stride, 64, vdupq_lane_u8::<0>(dc0));
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_128_predictor_64x64_neon(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let dc0 = vdupq_n_u8(0x80);
    dc_store_64xh(dst, stride, 64, dc0);
}

//------------------------------------------------------------------------------
// DC rectangular cases

const DC_MULTIPLIER_1X2: i32 = 0x5556;
const DC_MULTIPLIER_1X4: i32 = 0x3334;

const DC_SHIFT2: i32 = 16;

#[inline(always)]
fn divide_using_multiply_shift(num: i32, shift1: i32, multiplier: i32, shift2: i32) -> i32 {
    let interm = num >> shift1;
    interm * multiplier >> shift2
}

#[inline(always)]
fn calculate_dc_from_sum(bw: i32, bh: i32, sum: u32, shift1: i32, multiplier: i32) -> i32 {
    let expected_dc = divide_using_multiply_shift(
        (sum as i32) + ((bw + bh) >> 1),
        shift1,
        multiplier,
        DC_SHIFT2,
    );
    debug_assert!(expected_dc < (1 << 8));
    expected_dc
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_predictor_4x8_neon(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let a = load_u8_4x1(above);
    let l = vld1_u8(left);
    let sum: u32 = horizontal_add_u16x8(vaddl_u8(a, l));
    let dc: u32 = calculate_dc_from_sum(4, 8, sum, 2, DC_MULTIPLIER_1X2) as u32;
    dc_store_4xh(dst, stride, 8, vdup_n_u8(dc as u8));
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_predictor_8x4_neon(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let a = vld1_u8(above);
    let l = load_u8_4x1(left);
    let sum: u32 = horizontal_add_u16x8(vaddl_u8(a, l));
    let dc: u32 = calculate_dc_from_sum(8, 4, sum, 2, DC_MULTIPLIER_1X2) as u32;
    dc_store_8xh(dst, stride, 4, vdup_n_u8(dc as u8));
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_predictor_4x16_neon(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let a = load_u8_4x1(above);
    let l = vld1q_u8(left);
    let sum_al = vaddw_u8(vpaddlq_u8(l), a);
    let sum: u32 = horizontal_add_u16x8(sum_al);
    let dc: u32 = calculate_dc_from_sum(4, 16, sum, 2, DC_MULTIPLIER_1X4) as u32;
    dc_store_4xh(dst, stride, 16, vdup_n_u8(dc as u8));
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_predictor_16x4_neon(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let a = vld1q_u8(above);
    let l = load_u8_4x1(left);
    let sum_al = vaddw_u8(vpaddlq_u8(a), l);
    let sum: u32 = horizontal_add_u16x8(sum_al);
    let dc: u32 = calculate_dc_from_sum(16, 4, sum, 2, DC_MULTIPLIER_1X4) as u32;
    dc_store_16xh(dst, stride, 4, vdupq_n_u8(dc as u8));
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_predictor_8x16_neon(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let a = vld1_u8(above);
    let l = vld1q_u8(left);
    let sum_al = vaddw_u8(vpaddlq_u8(l), a);
    let sum: u32 = horizontal_add_u16x8(sum_al);
    let dc: u32 = calculate_dc_from_sum(8, 16, sum, 3, DC_MULTIPLIER_1X2) as u32;
    dc_store_8xh(dst, stride, 16, vdup_n_u8(dc as u8));
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_predictor_16x8_neon(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let a = vld1q_u8(above);
    let l = vld1_u8(left);
    let sum_al = vaddw_u8(vpaddlq_u8(a), l);
    let sum: u32 = horizontal_add_u16x8(sum_al);
    let dc: u32 = calculate_dc_from_sum(16, 8, sum, 3, DC_MULTIPLIER_1X2) as u32;
    dc_store_16xh(dst, stride, 8, vdupq_n_u8(dc as u8));
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_predictor_8x32_neon(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let a = vld1_u8(above);
    let sum_left = dc_load_partial_sum_32(left);
    let sum_al = vaddw_u8(sum_left, a);
    let sum: u32 = horizontal_add_u16x8(sum_al);
    let dc: u32 = calculate_dc_from_sum(8, 32, sum, 3, DC_MULTIPLIER_1X4) as u32;
    dc_store_8xh(dst, stride, 32, vdup_n_u8(dc as u8));
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_predictor_32x8_neon(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let sum_top = dc_load_partial_sum_32(above);
    let l = vld1_u8(left);
    let sum_al = vaddw_u8(sum_top, l);
    let sum: u32 = horizontal_add_u16x8(sum_al);
    let dc: u32 = calculate_dc_from_sum(32, 8, sum, 3, DC_MULTIPLIER_1X4) as u32;
    dc_store_32xh(dst, stride, 8, vdupq_n_u8(dc as u8));
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_predictor_16x32_neon(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let sum_above = dc_load_partial_sum_16(above);
    let sum_left = dc_load_partial_sum_32(left);
    let sum_al = vaddq_u16(sum_left, sum_above);
    let sum: u32 = horizontal_add_u16x8(sum_al);
    let dc: u32 = calculate_dc_from_sum(16, 32, sum, 4, DC_MULTIPLIER_1X2) as u32;
    dc_store_16xh(dst, stride, 32, vdupq_n_u8(dc as u8));
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_predictor_32x16_neon(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let sum_above = dc_load_partial_sum_32(above);
    let sum_left = dc_load_partial_sum_16(left);
    let sum_al = vaddq_u16(sum_left, sum_above);
    let sum: u32 = horizontal_add_u16x8(sum_al);
    let dc: u32 = calculate_dc_from_sum(32, 16, sum, 4, DC_MULTIPLIER_1X2) as u32;
    dc_store_32xh(dst, stride, 16, vdupq_n_u8(dc as u8));
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_predictor_16x64_neon(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let sum_above = dc_load_partial_sum_16(above);
    let sum_left = dc_load_partial_sum_64(left);
    let sum_al = vaddq_u16(sum_left, sum_above);
    let sum: u32 = horizontal_add_u16x8(sum_al);
    let dc: u32 = calculate_dc_from_sum(16, 64, sum, 4, DC_MULTIPLIER_1X4) as u32;
    dc_store_16xh(dst, stride, 64, vdupq_n_u8(dc as u8));
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_predictor_64x16_neon(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let sum_above = dc_load_partial_sum_64(above);
    let sum_left = dc_load_partial_sum_16(left);
    let sum_al = vaddq_u16(sum_above, sum_left);
    let sum: u32 = horizontal_add_u16x8(sum_al);
    let dc: u32 = calculate_dc_from_sum(64, 16, sum, 4, DC_MULTIPLIER_1X4) as u32;
    dc_store_64xh(dst, stride, 16, vdupq_n_u8(dc as u8));
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_predictor_32x64_neon(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let sum_above = dc_load_partial_sum_32(above);
    let sum_left = dc_load_partial_sum_64(left);
    let sum_al = vaddq_u16(sum_above, sum_left);
    let sum: u32 = horizontal_add_u16x8(sum_al);
    let dc: u32 = calculate_dc_from_sum(32, 64, sum, 5, DC_MULTIPLIER_1X2) as u32;
    dc_store_32xh(dst, stride, 64, vdupq_n_u8(dc as u8));
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_dc_predictor_64x32_neon(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let sum_above = dc_load_partial_sum_64(above);
    let sum_left = dc_load_partial_sum_32(left);
    let sum_al = vaddq_u16(sum_above, sum_left);
    let sum: u32 = horizontal_add_u16x8(sum_al);
    let dc: u32 = calculate_dc_from_sum(64, 32, sum, 5, DC_MULTIPLIER_1X2) as u32;
    dc_store_64xh(dst, stride, 32, vdupq_n_u8(dc as u8));
}

// #define DC_PREDICTOR_128(w, h, q)
macro_rules! dc_predictor_128 {
    ($name:ident, $store:ident, $h:literal, $dup:ident) => {
        #[no_mangle]
        pub unsafe extern "C-unwind" fn $name(
            dst: *mut u8,
            stride: isize,
            above: *const u8,
            left: *const u8,
        ) {
            $store(dst, stride, ($h), $dup(0x80));
        }
    };
}

dc_predictor_128!(aom_dc_128_predictor_4x16_neon, dc_store_4xh, 16, vdup_n_u8);
dc_predictor_128!(aom_dc_128_predictor_8x32_neon, dc_store_8xh, 32, vdup_n_u8);
dc_predictor_128!(aom_dc_128_predictor_16x4_neon, dc_store_16xh, 4, vdupq_n_u8);
dc_predictor_128!(aom_dc_128_predictor_16x64_neon, dc_store_16xh, 64, vdupq_n_u8);
dc_predictor_128!(aom_dc_128_predictor_32x8_neon, dc_store_32xh, 8, vdupq_n_u8);
dc_predictor_128!(aom_dc_128_predictor_64x16_neon, dc_store_64xh, 16, vdupq_n_u8);
dc_predictor_128!(aom_dc_128_predictor_4x8_neon, dc_store_4xh, 8, vdup_n_u8);
dc_predictor_128!(aom_dc_128_predictor_8x4_neon, dc_store_8xh, 4, vdup_n_u8);
dc_predictor_128!(aom_dc_128_predictor_8x16_neon, dc_store_8xh, 16, vdup_n_u8);
dc_predictor_128!(aom_dc_128_predictor_16x8_neon, dc_store_16xh, 8, vdupq_n_u8);
dc_predictor_128!(aom_dc_128_predictor_16x32_neon, dc_store_16xh, 32, vdupq_n_u8);
dc_predictor_128!(aom_dc_128_predictor_32x16_neon, dc_store_32xh, 16, vdupq_n_u8);
dc_predictor_128!(aom_dc_128_predictor_32x64_neon, dc_store_32xh, 64, vdupq_n_u8);
dc_predictor_128!(aom_dc_128_predictor_64x32_neon, dc_store_64xh, 32, vdupq_n_u8);

// #define DC_PREDICTOR_LEFT(w, h, shift, q)
macro_rules! dc_predictor_left {
    ($name:ident, $load:ident, $store:ident, $h:literal, $shift:literal, $dup:ident) => {
        #[no_mangle]
        pub unsafe extern "C-unwind" fn $name(
            dst: *mut u8,
            stride: isize,
            above: *const u8,
            left: *const u8,
        ) {
            let sum = $load(left);
            let dc0 = vrshrn_n_u16::<$shift>(sum);
            $store(dst, stride, ($h), $dup::<0>(dc0));
        }
    };
}

dc_predictor_left!(aom_dc_left_predictor_4x8_neon, dc_load_sum_8, dc_store_4xh, 8, 3, vdup_lane_u8);
dc_predictor_left!(aom_dc_left_predictor_8x4_neon, dc_load_sum_4, dc_store_8xh, 4, 2, vdup_lane_u8);
dc_predictor_left!(aom_dc_left_predictor_8x16_neon, dc_load_sum_16, dc_store_8xh, 16, 4, vdup_lane_u8);
dc_predictor_left!(aom_dc_left_predictor_16x8_neon, dc_load_sum_8, dc_store_16xh, 8, 3, vdupq_lane_u8);
dc_predictor_left!(aom_dc_left_predictor_16x32_neon, dc_load_sum_32, dc_store_16xh, 32, 5, vdupq_lane_u8);
dc_predictor_left!(aom_dc_left_predictor_32x16_neon, dc_load_sum_16, dc_store_32xh, 16, 4, vdupq_lane_u8);
dc_predictor_left!(aom_dc_left_predictor_32x64_neon, dc_load_sum_64, dc_store_32xh, 64, 6, vdupq_lane_u8);
dc_predictor_left!(aom_dc_left_predictor_64x32_neon, dc_load_sum_32, dc_store_64xh, 32, 5, vdupq_lane_u8);
dc_predictor_left!(aom_dc_left_predictor_4x16_neon, dc_load_sum_16, dc_store_4xh, 16, 4, vdup_lane_u8);
dc_predictor_left!(aom_dc_left_predictor_16x4_neon, dc_load_sum_4, dc_store_16xh, 4, 2, vdupq_lane_u8);
dc_predictor_left!(aom_dc_left_predictor_8x32_neon, dc_load_sum_32, dc_store_8xh, 32, 5, vdup_lane_u8);
dc_predictor_left!(aom_dc_left_predictor_32x8_neon, dc_load_sum_8, dc_store_32xh, 8, 3, vdupq_lane_u8);
dc_predictor_left!(aom_dc_left_predictor_16x64_neon, dc_load_sum_64, dc_store_16xh, 64, 6, vdupq_lane_u8);
dc_predictor_left!(aom_dc_left_predictor_64x16_neon, dc_load_sum_16, dc_store_64xh, 16, 4, vdupq_lane_u8);

// #define DC_PREDICTOR_TOP(w, h, shift, q)
macro_rules! dc_predictor_top {
    ($name:ident, $load:ident, $store:ident, $h:literal, $shift:literal, $dup:ident) => {
        #[no_mangle]
        pub unsafe extern "C-unwind" fn $name(
            dst: *mut u8,
            stride: isize,
            above: *const u8,
            left: *const u8,
        ) {
            let sum = $load(above);
            let dc0 = vrshrn_n_u16::<$shift>(sum);
            $store(dst, stride, ($h), $dup::<0>(dc0));
        }
    };
}

dc_predictor_top!(aom_dc_top_predictor_8x32_neon, dc_load_sum_8, dc_store_8xh, 32, 3, vdup_lane_u8);
dc_predictor_top!(aom_dc_top_predictor_4x16_neon, dc_load_sum_4, dc_store_4xh, 16, 2, vdup_lane_u8);
dc_predictor_top!(aom_dc_top_predictor_16x4_neon, dc_load_sum_16, dc_store_16xh, 4, 4, vdupq_lane_u8);
dc_predictor_top!(aom_dc_top_predictor_16x64_neon, dc_load_sum_16, dc_store_16xh, 64, 4, vdupq_lane_u8);
dc_predictor_top!(aom_dc_top_predictor_32x8_neon, dc_load_sum_32, dc_store_32xh, 8, 5, vdupq_lane_u8);
dc_predictor_top!(aom_dc_top_predictor_64x16_neon, dc_load_sum_64, dc_store_64xh, 16, 6, vdupq_lane_u8);
dc_predictor_top!(aom_dc_top_predictor_4x8_neon, dc_load_sum_4, dc_store_4xh, 8, 2, vdup_lane_u8);
dc_predictor_top!(aom_dc_top_predictor_8x4_neon, dc_load_sum_8, dc_store_8xh, 4, 3, vdup_lane_u8);
dc_predictor_top!(aom_dc_top_predictor_8x16_neon, dc_load_sum_8, dc_store_8xh, 16, 3, vdup_lane_u8);
dc_predictor_top!(aom_dc_top_predictor_16x8_neon, dc_load_sum_16, dc_store_16xh, 8, 4, vdupq_lane_u8);
dc_predictor_top!(aom_dc_top_predictor_16x32_neon, dc_load_sum_16, dc_store_16xh, 32, 4, vdupq_lane_u8);
dc_predictor_top!(aom_dc_top_predictor_32x16_neon, dc_load_sum_32, dc_store_32xh, 16, 5, vdupq_lane_u8);
dc_predictor_top!(aom_dc_top_predictor_32x64_neon, dc_load_sum_32, dc_store_32xh, 64, 5, vdupq_lane_u8);
dc_predictor_top!(aom_dc_top_predictor_64x32_neon, dc_load_sum_64, dc_store_64xh, 32, 6, vdupq_lane_u8);

// -----------------------------------------------------------------------------

// Defines one exported `void name(uint8_t *dst, ptrdiff_t stride, const
// uint8_t *above, const uint8_t *left)` predictor with the given body.
macro_rules! intra_pred_fn {
    ($name:ident, |$dst:ident, $stride:ident, $above:ident, $left:ident| $body:block) => {
        #[no_mangle]
        pub unsafe extern "C-unwind" fn $name(
            mut $dst: *mut u8,
            $stride: isize,
            mut $above: *const u8,
            mut $left: *const u8,
        ) $body
    };
}

#[inline(always)]
unsafe fn v_store_4xh(dst: *mut u8, stride: isize, h: i32, d0: uint8x8_t) {
    for i in 0..h {
        store_u8_4x1(dst.offset(i as isize * stride), d0);
    }
}

#[inline(always)]
unsafe fn v_store_8xh(dst: *mut u8, stride: isize, h: i32, d0: uint8x8_t) {
    for i in 0..h {
        vst1_u8(dst.offset(i as isize * stride), d0);
    }
}

#[inline(always)]
unsafe fn v_store_16xh(dst: *mut u8, stride: isize, h: i32, d0: uint8x16_t) {
    for i in 0..h {
        vst1q_u8(dst.offset(i as isize * stride), d0);
    }
}

#[inline(always)]
unsafe fn v_store_32xh(mut dst: *mut u8, stride: isize, h: i32, d0: uint8x16_t, d1: uint8x16_t) {
    for _i in 0..h {
        vst1q_u8(dst.add(0), d0);
        vst1q_u8(dst.add(16), d1);
        dst = dst.offset(stride);
    }
}

#[inline(always)]
unsafe fn v_store_64xh(
    mut dst: *mut u8,
    stride: isize,
    h: i32,
    d0: uint8x16_t,
    d1: uint8x16_t,
    d2: uint8x16_t,
    d3: uint8x16_t,
) {
    for _i in 0..h {
        vst1q_u8(dst.add(0), d0);
        vst1q_u8(dst.add(16), d1);
        vst1q_u8(dst.add(32), d2);
        vst1q_u8(dst.add(48), d3);
        dst = dst.offset(stride);
    }
}

intra_pred_fn!(aom_v_predictor_4x4_neon, |dst, stride, above, left| {
    v_store_4xh(dst, stride, 4, load_u8_4x1(above));
});

intra_pred_fn!(aom_v_predictor_8x8_neon, |dst, stride, above, left| {
    v_store_8xh(dst, stride, 8, vld1_u8(above));
});

intra_pred_fn!(aom_v_predictor_16x16_neon, |dst, stride, above, left| {
    v_store_16xh(dst, stride, 16, vld1q_u8(above));
});

intra_pred_fn!(aom_v_predictor_32x32_neon, |dst, stride, above, left| {
    let d0 = vld1q_u8(above);
    let d1 = vld1q_u8(above.add(16));
    v_store_32xh(dst, stride, 32, d0, d1);
});

intra_pred_fn!(aom_v_predictor_4x8_neon, |dst, stride, above, left| {
    v_store_4xh(dst, stride, 8, load_u8_4x1(above));
});

intra_pred_fn!(aom_v_predictor_4x16_neon, |dst, stride, above, left| {
    v_store_4xh(dst, stride, 16, load_u8_4x1(above));
});

intra_pred_fn!(aom_v_predictor_8x4_neon, |dst, stride, above, left| {
    v_store_8xh(dst, stride, 4, vld1_u8(above));
});

intra_pred_fn!(aom_v_predictor_8x16_neon, |dst, stride, above, left| {
    v_store_8xh(dst, stride, 16, vld1_u8(above));
});

intra_pred_fn!(aom_v_predictor_8x32_neon, |dst, stride, above, left| {
    v_store_8xh(dst, stride, 32, vld1_u8(above));
});

intra_pred_fn!(aom_v_predictor_16x4_neon, |dst, stride, above, left| {
    v_store_16xh(dst, stride, 4, vld1q_u8(above));
});

intra_pred_fn!(aom_v_predictor_16x8_neon, |dst, stride, above, left| {
    v_store_16xh(dst, stride, 8, vld1q_u8(above));
});

intra_pred_fn!(aom_v_predictor_16x32_neon, |dst, stride, above, left| {
    v_store_16xh(dst, stride, 32, vld1q_u8(above));
});

intra_pred_fn!(aom_v_predictor_16x64_neon, |dst, stride, above, left| {
    v_store_16xh(dst, stride, 64, vld1q_u8(above));
});

intra_pred_fn!(aom_v_predictor_32x8_neon, |dst, stride, above, left| {
    let d0 = vld1q_u8(above);
    let d1 = vld1q_u8(above.add(16));
    v_store_32xh(dst, stride, 8, d0, d1);
});

intra_pred_fn!(aom_v_predictor_32x16_neon, |dst, stride, above, left| {
    let d0 = vld1q_u8(above);
    let d1 = vld1q_u8(above.add(16));
    v_store_32xh(dst, stride, 16, d0, d1);
});

intra_pred_fn!(aom_v_predictor_32x64_neon, |dst, stride, above, left| {
    let d0 = vld1q_u8(above);
    let d1 = vld1q_u8(above.add(16));
    v_store_32xh(dst, stride, 64, d0, d1);
});

intra_pred_fn!(aom_v_predictor_64x16_neon, |dst, stride, above, left| {
    let d0 = vld1q_u8(above);
    let d1 = vld1q_u8(above.add(16));
    let d2 = vld1q_u8(above.add(32));
    let d3 = vld1q_u8(above.add(48));
    v_store_64xh(dst, stride, 16, d0, d1, d2, d3);
});

intra_pred_fn!(aom_v_predictor_64x32_neon, |dst, stride, above, left| {
    let d0 = vld1q_u8(above);
    let d1 = vld1q_u8(above.add(16));
    let d2 = vld1q_u8(above.add(32));
    let d3 = vld1q_u8(above.add(48));
    v_store_64xh(dst, stride, 32, d0, d1, d2, d3);
});

intra_pred_fn!(aom_v_predictor_64x64_neon, |dst, stride, above, left| {
    let d0 = vld1q_u8(above);
    let d1 = vld1q_u8(above.add(16));
    let d2 = vld1q_u8(above.add(32));
    let d3 = vld1q_u8(above.add(48));
    v_store_64xh(dst, stride, 64, d0, d1, d2, d3);
});

// -----------------------------------------------------------------------------

#[inline(always)]
unsafe fn h_store_4x8(dst: *mut u8, stride: isize, d0: uint8x8_t) {
    store_u8_4x1(dst.offset(0 * stride), vdup_lane_u8::<0>(d0));
    store_u8_4x1(dst.offset(1 * stride), vdup_lane_u8::<1>(d0));
    store_u8_4x1(dst.offset(2 * stride), vdup_lane_u8::<2>(d0));
    store_u8_4x1(dst.offset(3 * stride), vdup_lane_u8::<3>(d0));
    store_u8_4x1(dst.offset(4 * stride), vdup_lane_u8::<4>(d0));
    store_u8_4x1(dst.offset(5 * stride), vdup_lane_u8::<5>(d0));
    store_u8_4x1(dst.offset(6 * stride), vdup_lane_u8::<6>(d0));
    store_u8_4x1(dst.offset(7 * stride), vdup_lane_u8::<7>(d0));
}

#[inline(always)]
unsafe fn h_store_8x8(dst: *mut u8, stride: isize, d0: uint8x8_t) {
    vst1_u8(dst.offset(0 * stride), vdup_lane_u8::<0>(d0));
    vst1_u8(dst.offset(1 * stride), vdup_lane_u8::<1>(d0));
    vst1_u8(dst.offset(2 * stride), vdup_lane_u8::<2>(d0));
    vst1_u8(dst.offset(3 * stride), vdup_lane_u8::<3>(d0));
    vst1_u8(dst.offset(4 * stride), vdup_lane_u8::<4>(d0));
    vst1_u8(dst.offset(5 * stride), vdup_lane_u8::<5>(d0));
    vst1_u8(dst.offset(6 * stride), vdup_lane_u8::<6>(d0));
    vst1_u8(dst.offset(7 * stride), vdup_lane_u8::<7>(d0));
}

#[inline(always)]
unsafe fn h_store_16x8(dst: *mut u8, stride: isize, d0: uint8x8_t) {
    vst1q_u8(dst.offset(0 * stride), vdupq_lane_u8::<0>(d0));
    vst1q_u8(dst.offset(1 * stride), vdupq_lane_u8::<1>(d0));
    vst1q_u8(dst.offset(2 * stride), vdupq_lane_u8::<2>(d0));
    vst1q_u8(dst.offset(3 * stride), vdupq_lane_u8::<3>(d0));
    vst1q_u8(dst.offset(4 * stride), vdupq_lane_u8::<4>(d0));
    vst1q_u8(dst.offset(5 * stride), vdupq_lane_u8::<5>(d0));
    vst1q_u8(dst.offset(6 * stride), vdupq_lane_u8::<6>(d0));
    vst1q_u8(dst.offset(7 * stride), vdupq_lane_u8::<7>(d0));
}

// One row of h_store_32x8 (two 16-byte stores of lane `$lane`) or
// h_store_64x8 (four stores), in the original's order.
macro_rules! h_store_row {
    ($dst:ident, $d0:ident, $lane:literal, 2) => {
        vst1q_u8($dst.add(0), vdupq_lane_u8::<$lane>($d0));
        vst1q_u8($dst.add(16), vdupq_lane_u8::<$lane>($d0));
    };
    ($dst:ident, $d0:ident, $lane:literal, 4) => {
        vst1q_u8($dst.add(0), vdupq_lane_u8::<$lane>($d0));
        vst1q_u8($dst.add(16), vdupq_lane_u8::<$lane>($d0));
        vst1q_u8($dst.add(32), vdupq_lane_u8::<$lane>($d0));
        vst1q_u8($dst.add(48), vdupq_lane_u8::<$lane>($d0));
    };
}

#[inline(always)]
unsafe fn h_store_32x8(mut dst: *mut u8, stride: isize, d0: uint8x8_t) {
    h_store_row!(dst, d0, 0, 2);
    dst = dst.offset(stride);
    h_store_row!(dst, d0, 1, 2);
    dst = dst.offset(stride);
    h_store_row!(dst, d0, 2, 2);
    dst = dst.offset(stride);
    h_store_row!(dst, d0, 3, 2);
    dst = dst.offset(stride);
    h_store_row!(dst, d0, 4, 2);
    dst = dst.offset(stride);
    h_store_row!(dst, d0, 5, 2);
    dst = dst.offset(stride);
    h_store_row!(dst, d0, 6, 2);
    dst = dst.offset(stride);
    h_store_row!(dst, d0, 7, 2);
}

#[inline(always)]
unsafe fn h_store_64x8(mut dst: *mut u8, stride: isize, d0: uint8x8_t) {
    h_store_row!(dst, d0, 0, 4);
    dst = dst.offset(stride);
    h_store_row!(dst, d0, 1, 4);
    dst = dst.offset(stride);
    h_store_row!(dst, d0, 2, 4);
    dst = dst.offset(stride);
    h_store_row!(dst, d0, 3, 4);
    dst = dst.offset(stride);
    h_store_row!(dst, d0, 4, 4);
    dst = dst.offset(stride);
    h_store_row!(dst, d0, 5, 4);
    dst = dst.offset(stride);
    h_store_row!(dst, d0, 6, 4);
    dst = dst.offset(stride);
    h_store_row!(dst, d0, 7, 4);
}

intra_pred_fn!(aom_h_predictor_4x4_neon, |dst, stride, above, left| {
    let d0 = load_u8_4x1(left);
    store_u8_4x1(dst.offset(0 * stride), vdup_lane_u8::<0>(d0));
    store_u8_4x1(dst.offset(1 * stride), vdup_lane_u8::<1>(d0));
    store_u8_4x1(dst.offset(2 * stride), vdup_lane_u8::<2>(d0));
    store_u8_4x1(dst.offset(3 * stride), vdup_lane_u8::<3>(d0));
});

intra_pred_fn!(aom_h_predictor_8x8_neon, |dst, stride, above, left| {
    let d0 = vld1_u8(left);
    h_store_8x8(dst, stride, d0);
});

intra_pred_fn!(aom_h_predictor_16x16_neon, |dst, stride, above, left| {
    let d0 = vld1q_u8(left);
    h_store_16x8(dst, stride, vget_low_u8(d0));
    h_store_16x8(dst.offset(8 * stride), stride, vget_high_u8(d0));
});

intra_pred_fn!(aom_h_predictor_32x32_neon, |dst, stride, above, left| {
    let d0 = vld1q_u8(left);
    let d1 = vld1q_u8(left.add(16));
    h_store_32x8(dst.offset(0 * stride), stride, vget_low_u8(d0));
    h_store_32x8(dst.offset(8 * stride), stride, vget_high_u8(d0));
    h_store_32x8(dst.offset(16 * stride), stride, vget_low_u8(d1));
    h_store_32x8(dst.offset(24 * stride), stride, vget_high_u8(d1));
});

intra_pred_fn!(aom_h_predictor_4x8_neon, |dst, stride, above, left| {
    let d0 = vld1_u8(left);
    h_store_4x8(dst, stride, d0);
});

intra_pred_fn!(aom_h_predictor_4x16_neon, |dst, stride, above, left| {
    let d0 = vld1q_u8(left);
    h_store_4x8(dst.offset(0 * stride), stride, vget_low_u8(d0));
    h_store_4x8(dst.offset(8 * stride), stride, vget_high_u8(d0));
});

intra_pred_fn!(aom_h_predictor_8x4_neon, |dst, stride, above, left| {
    let d0 = load_u8_4x1(left);
    vst1_u8(dst.offset(0 * stride), vdup_lane_u8::<0>(d0));
    vst1_u8(dst.offset(1 * stride), vdup_lane_u8::<1>(d0));
    vst1_u8(dst.offset(2 * stride), vdup_lane_u8::<2>(d0));
    vst1_u8(dst.offset(3 * stride), vdup_lane_u8::<3>(d0));
});

intra_pred_fn!(aom_h_predictor_8x16_neon, |dst, stride, above, left| {
    let d0 = vld1q_u8(left);
    h_store_8x8(dst.offset(0 * stride), stride, vget_low_u8(d0));
    h_store_8x8(dst.offset(8 * stride), stride, vget_high_u8(d0));
});

intra_pred_fn!(aom_h_predictor_8x32_neon, |dst, stride, above, left| {
    let d0 = vld1q_u8(left);
    let d1 = vld1q_u8(left.add(16));
    h_store_8x8(dst.offset(0 * stride), stride, vget_low_u8(d0));
    h_store_8x8(dst.offset(8 * stride), stride, vget_high_u8(d0));
    h_store_8x8(dst.offset(16 * stride), stride, vget_low_u8(d1));
    h_store_8x8(dst.offset(24 * stride), stride, vget_high_u8(d1));
});

intra_pred_fn!(aom_h_predictor_16x4_neon, |dst, stride, above, left| {
    let d0 = load_u8_4x1(left);
    vst1q_u8(dst.offset(0 * stride), vdupq_lane_u8::<0>(d0));
    vst1q_u8(dst.offset(1 * stride), vdupq_lane_u8::<1>(d0));
    vst1q_u8(dst.offset(2 * stride), vdupq_lane_u8::<2>(d0));
    vst1q_u8(dst.offset(3 * stride), vdupq_lane_u8::<3>(d0));
});

intra_pred_fn!(aom_h_predictor_16x8_neon, |dst, stride, above, left| {
    let d0 = vld1_u8(left);
    h_store_16x8(dst, stride, d0);
});

intra_pred_fn!(aom_h_predictor_16x32_neon, |dst, stride, above, left| {
    let d0 = vld1q_u8(left);
    let d1 = vld1q_u8(left.add(16));
    h_store_16x8(dst.offset(0 * stride), stride, vget_low_u8(d0));
    h_store_16x8(dst.offset(8 * stride), stride, vget_high_u8(d0));
    h_store_16x8(dst.offset(16 * stride), stride, vget_low_u8(d1));
    h_store_16x8(dst.offset(24 * stride), stride, vget_high_u8(d1));
});

intra_pred_fn!(aom_h_predictor_16x64_neon, |dst, stride, above, left| {
    let d0 = vld1q_u8(left);
    let d1 = vld1q_u8(left.add(16));
    let d2 = vld1q_u8(left.add(32));
    let d3 = vld1q_u8(left.add(48));
    h_store_16x8(dst.offset(0 * stride), stride, vget_low_u8(d0));
    h_store_16x8(dst.offset(8 * stride), stride, vget_high_u8(d0));
    h_store_16x8(dst.offset(16 * stride), stride, vget_low_u8(d1));
    h_store_16x8(dst.offset(24 * stride), stride, vget_high_u8(d1));
    h_store_16x8(dst.offset(32 * stride), stride, vget_low_u8(d2));
    h_store_16x8(dst.offset(40 * stride), stride, vget_high_u8(d2));
    h_store_16x8(dst.offset(48 * stride), stride, vget_low_u8(d3));
    h_store_16x8(dst.offset(56 * stride), stride, vget_high_u8(d3));
});

intra_pred_fn!(aom_h_predictor_32x8_neon, |dst, stride, above, left| {
    let d0 = vld1_u8(left);
    h_store_32x8(dst, stride, d0);
});

intra_pred_fn!(aom_h_predictor_32x16_neon, |dst, stride, above, left| {
    let d0 = vld1q_u8(left);
    h_store_32x8(dst.offset(0 * stride), stride, vget_low_u8(d0));
    h_store_32x8(dst.offset(8 * stride), stride, vget_high_u8(d0));
});

intra_pred_fn!(aom_h_predictor_32x64_neon, |dst, stride, above, left| {
    let d0 = vld1q_u8(left.add(0));
    let d1 = vld1q_u8(left.add(16));
    let d2 = vld1q_u8(left.add(32));
    let d3 = vld1q_u8(left.add(48));
    h_store_32x8(dst.offset(0 * stride), stride, vget_low_u8(d0));
    h_store_32x8(dst.offset(8 * stride), stride, vget_high_u8(d0));
    h_store_32x8(dst.offset(16 * stride), stride, vget_low_u8(d1));
    h_store_32x8(dst.offset(24 * stride), stride, vget_high_u8(d1));
    h_store_32x8(dst.offset(32 * stride), stride, vget_low_u8(d2));
    h_store_32x8(dst.offset(40 * stride), stride, vget_high_u8(d2));
    h_store_32x8(dst.offset(48 * stride), stride, vget_low_u8(d3));
    h_store_32x8(dst.offset(56 * stride), stride, vget_high_u8(d3));
});

intra_pred_fn!(aom_h_predictor_64x16_neon, |dst, stride, above, left| {
    let d0 = vld1q_u8(left);
    h_store_64x8(dst.offset(0 * stride), stride, vget_low_u8(d0));
    h_store_64x8(dst.offset(8 * stride), stride, vget_high_u8(d0));
});

intra_pred_fn!(aom_h_predictor_64x32_neon, |dst, stride, above, left| {
    for _i in 0..2 {
        let d0 = vld1q_u8(left);
        h_store_64x8(dst.offset(0 * stride), stride, vget_low_u8(d0));
        h_store_64x8(dst.offset(8 * stride), stride, vget_high_u8(d0));
        left = left.add(16);
        dst = dst.offset(16 * stride);
    }
});

intra_pred_fn!(aom_h_predictor_64x64_neon, |dst, stride, above, left| {
    for _i in 0..4 {
        let d0 = vld1q_u8(left);
        h_store_64x8(dst.offset(0 * stride), stride, vget_low_u8(d0));
        h_store_64x8(dst.offset(8 * stride), stride, vget_high_u8(d0));
        left = left.add(16);
        dst = dst.offset(16 * stride);
    }
});

// SMOOTH_PRED

// Transcribed from upstream/aom/aom_dsp/intrapred_common.h.
const SMOOTH_WEIGHT_LOG2_SCALE: i32 = 8;

// Transcribed from upstream/aom/aom_dsp/intrapred_common.h (header-local
// `static const` table).
#[rustfmt::skip]
static smooth_weights: [u8; 124] = [
    // bs = 4
    255, 149, 85, 64,
    // bs = 8
    255, 197, 146, 105, 73, 50, 37, 32,
    // bs = 16
    255, 225, 196, 170, 145, 123, 102, 84, 68, 54, 43, 33, 26, 20, 17, 16,
    // bs = 32
    255, 240, 225, 210, 196, 182, 169, 157, 145, 133, 122, 111, 101, 92, 83, 74,
    66, 59, 52, 45, 39, 34, 29, 25, 21, 17, 14, 12, 10, 9, 8, 8,
    // bs = 64
    255, 248, 240, 233, 225, 218, 210, 203, 196, 189, 182, 176, 169, 163, 156,
    150, 144, 138, 133, 127, 121, 116, 111, 106, 101, 96, 91, 86, 82, 77, 73, 69,
    65, 61, 57, 54, 50, 47, 44, 41, 38, 35, 32, 29, 27, 25, 22, 20, 18, 16, 15,
    13, 12, 10, 9, 8, 7, 6, 6, 5, 5, 4, 4, 4,
];

#[inline(always)]
unsafe fn smooth_weights_ptr() -> *const u8 {
    smooth_weights.as_ptr()
}

// 256 - v = vneg_s8(v)
#[inline(always)]
unsafe fn negate_s8(v: uint8x8_t) -> uint8x8_t {
    vreinterpret_u8_s8(vneg_s8(vreinterpret_s8_u8(v)))
}

unsafe fn smooth_4xh_neon(
    mut dst: *mut u8,
    stride: isize,
    top_row: *const u8,
    left_column: *const u8,
    height: i32,
) {
    let top_right: u8 = *top_row.add(3);
    let bottom_left: u8 = *left_column.offset((height - 1) as isize);
    let weights_y: *const u8 = smooth_weights_ptr().offset((height - 4) as isize);

    let mut top_v = load_u8_4x1(top_row);
    let top_right_v = vdup_n_u8(top_right);
    let bottom_left_v = vdup_n_u8(bottom_left);
    let mut weights_x_v = load_u8_4x1(smooth_weights_ptr());
    let scaled_weights_x = negate_s8(weights_x_v);
    let weighted_tr = vmull_u8(scaled_weights_x, top_right_v);

    debug_assert!(height > 0);
    let mut y: i32 = 0;
    loop {
        let left_v = vdup_n_u8(*left_column.offset(y as isize));
        let weights_y_v = vdup_n_u8(*weights_y.offset(y as isize));
        let scaled_weights_y = negate_s8(weights_y_v);
        let weighted_bl = vmull_u8(scaled_weights_y, bottom_left_v);
        let weighted_top_bl = vmlal_u8(weighted_bl, weights_y_v, top_v);
        let weighted_left_tr = vmlal_u8(weighted_tr, weights_x_v, left_v);
        // Maximum value of each parameter: 0xFF00
        let avg = vhaddq_u16(weighted_top_bl, weighted_left_tr);
        let result = vrshrn_n_u16::<SMOOTH_WEIGHT_LOG2_SCALE>(avg);

        vst1_lane_u32::<0>(dst as *mut u32, vreinterpret_u32_u8(result));
        dst = dst.offset(stride);
        y += 1;
        if y == height {
            break;
        }
    }
}

#[inline(always)]
unsafe fn calculate_pred(weighted_top_bl: uint16x8_t, weighted_left_tr: uint16x8_t) -> uint8x8_t {
    // Maximum value of each parameter: 0xFF00
    let avg = vhaddq_u16(weighted_top_bl, weighted_left_tr);
    vrshrn_n_u16::<SMOOTH_WEIGHT_LOG2_SCALE>(avg)
}

#[inline(always)]
unsafe fn calculate_weights_and_pred(
    top: uint8x8_t,
    left: uint8x8_t,
    weighted_tr: uint16x8_t,
    bottom_left: uint8x8_t,
    weights_x: uint8x8_t,
    scaled_weights_y: uint8x8_t,
    weights_y: uint8x8_t,
) -> uint8x8_t {
    let weighted_top = vmull_u8(weights_y, top);
    let weighted_top_bl = vmlal_u8(weighted_top, scaled_weights_y, bottom_left);
    let weighted_left_tr = vmlal_u8(weighted_tr, weights_x, left);
    calculate_pred(weighted_top_bl, weighted_left_tr)
}

unsafe fn smooth_8xh_neon(
    mut dst: *mut u8,
    stride: isize,
    top_row: *const u8,
    left_column: *const u8,
    height: i32,
) {
    let top_right: u8 = *top_row.add(7);
    let bottom_left: u8 = *left_column.offset((height - 1) as isize);
    let weights_y: *const u8 = smooth_weights_ptr().offset((height - 4) as isize);

    let top_v = vld1_u8(top_row);
    let top_right_v = vdup_n_u8(top_right);
    let bottom_left_v = vdup_n_u8(bottom_left);
    let weights_x_v = vld1_u8(smooth_weights_ptr().add(4));
    let scaled_weights_x = negate_s8(weights_x_v);
    let weighted_tr = vmull_u8(scaled_weights_x, top_right_v);

    debug_assert!(height > 0);
    let mut y: i32 = 0;
    loop {
        let left_v = vdup_n_u8(*left_column.offset(y as isize));
        let weights_y_v = vdup_n_u8(*weights_y.offset(y as isize));
        let scaled_weights_y = negate_s8(weights_y_v);
        let result = calculate_weights_and_pred(
            top_v,
            left_v,
            weighted_tr,
            bottom_left_v,
            weights_x_v,
            scaled_weights_y,
            weights_y_v,
        );

        vst1_u8(dst, result);
        dst = dst.offset(stride);
        y += 1;
        if y == height {
            break;
        }
    }
}

// #define SMOOTH_NXM(W, H) / SMOOTH_NXM_WIDE(W, H) and the SMOOTH_V / SMOOTH_H
// counterparts: forward to the per-width helper with the block height.
macro_rules! smooth_nxm {
    ($name:ident, $helper:ident, $h:literal) => {
        #[no_mangle]
        pub unsafe extern "C-unwind" fn $name(
            dst: *mut u8,
            y_stride: isize,
            above: *const u8,
            left: *const u8,
        ) {
            $helper(dst, y_stride, above, left, $h);
        }
    };
}

smooth_nxm!(aom_smooth_predictor_4x4_neon, smooth_4xh_neon, 4);
smooth_nxm!(aom_smooth_predictor_4x8_neon, smooth_4xh_neon, 8);
smooth_nxm!(aom_smooth_predictor_8x4_neon, smooth_8xh_neon, 4);
smooth_nxm!(aom_smooth_predictor_8x8_neon, smooth_8xh_neon, 8);
smooth_nxm!(aom_smooth_predictor_8x16_neon, smooth_8xh_neon, 16);
smooth_nxm!(aom_smooth_predictor_4x16_neon, smooth_4xh_neon, 16);
smooth_nxm!(aom_smooth_predictor_8x32_neon, smooth_8xh_neon, 32);

#[inline(always)]
unsafe fn calculate_weights_and_predq(
    top: uint8x16_t,
    left: uint8x8_t,
    top_right: uint8x8_t,
    weights_y: uint8x8_t,
    weights_x: uint8x16_t,
    scaled_weights_x: uint8x16_t,
    weighted_bl: uint16x8_t,
) -> uint8x16_t {
    let weighted_top_bl_low = vmlal_u8(weighted_bl, weights_y, vget_low_u8(top));
    let weighted_left_low = vmull_u8(vget_low_u8(weights_x), left);
    let weighted_left_tr_low = vmlal_u8(weighted_left_low, vget_low_u8(scaled_weights_x), top_right);
    let result_low = calculate_pred(weighted_top_bl_low, weighted_left_tr_low);

    let weighted_top_bl_high = vmlal_u8(weighted_bl, weights_y, vget_high_u8(top));
    let weighted_left_high = vmull_u8(vget_high_u8(weights_x), left);
    let weighted_left_tr_high =
        vmlal_u8(weighted_left_high, vget_high_u8(scaled_weights_x), top_right);
    let result_high = calculate_pred(weighted_top_bl_high, weighted_left_tr_high);

    vcombine_u8(result_low, result_high)
}

// 256 - v = vneg_s8(v)
#[inline(always)]
unsafe fn negate_s8q(v: uint8x16_t) -> uint8x16_t {
    vreinterpretq_u8_s8(vnegq_s8(vreinterpretq_s8_u8(v)))
}

// For width 16 and above.
// #define SMOOTH_PREDICTOR(W)
macro_rules! smooth_predictor {
    ($name:ident, $W:literal) => {
        unsafe fn $name(
            mut dst: *mut u8,
            stride: isize,
            top_row: *const u8,
            left_column: *const u8,
            height: i32,
        ) {
            let top_right: u8 = *top_row.add(($W) - 1);
            let bottom_left: u8 = *left_column.offset((height - 1) as isize);
            let weights_y: *const u8 = smooth_weights_ptr().offset((height - 4) as isize);

            let mut top_v: [uint8x16_t; 4] = core::mem::zeroed();
            top_v[0] = vld1q_u8(top_row);
            if ($W) > 16 {
                top_v[1] = vld1q_u8(top_row.add(16));
                if ($W) == 64 {
                    top_v[2] = vld1q_u8(top_row.add(32));
                    top_v[3] = vld1q_u8(top_row.add(48));
                }
            }

            let top_right_v = vdup_n_u8(top_right);
            let bottom_left_v = vdup_n_u8(bottom_left);

            let mut weights_x_v: [uint8x16_t; 4] = core::mem::zeroed();
            weights_x_v[0] = vld1q_u8(smooth_weights_ptr().add(($W) - 4));
            if ($W) > 16 {
                weights_x_v[1] = vld1q_u8(smooth_weights_ptr().add(($W) + 16 - 4));
                if ($W) == 64 {
                    weights_x_v[2] = vld1q_u8(smooth_weights_ptr().add(($W) + 32 - 4));
                    weights_x_v[3] = vld1q_u8(smooth_weights_ptr().add(($W) + 48 - 4));
                }
            }

            let mut scaled_weights_x: [uint8x16_t; 4] = core::mem::zeroed();
            scaled_weights_x[0] = negate_s8q(weights_x_v[0]);
            if ($W) > 16 {
                scaled_weights_x[1] = negate_s8q(weights_x_v[1]);
                if ($W) == 64 {
                    scaled_weights_x[2] = negate_s8q(weights_x_v[2]);
                    scaled_weights_x[3] = negate_s8q(weights_x_v[3]);
                }
            }

            for y in 0..height {
                let left_v = vdup_n_u8(*left_column.offset(y as isize));
                let weights_y_v = vdup_n_u8(*weights_y.offset(y as isize));
                let scaled_weights_y = negate_s8(weights_y_v);
                let weighted_bl = vmull_u8(scaled_weights_y, bottom_left_v);

                vst1q_u8(
                    dst,
                    calculate_weights_and_predq(
                        top_v[0],
                        left_v,
                        top_right_v,
                        weights_y_v,
                        weights_x_v[0],
                        scaled_weights_x[0],
                        weighted_bl,
                    ),
                );

                if ($W) > 16 {
                    vst1q_u8(
                        dst.add(16),
                        calculate_weights_and_predq(
                            top_v[1],
                            left_v,
                            top_right_v,
                            weights_y_v,
                            weights_x_v[1],
                            scaled_weights_x[1],
                            weighted_bl,
                        ),
                    );
                    if ($W) == 64 {
                        vst1q_u8(
                            dst.add(32),
                            calculate_weights_and_predq(
                                top_v[2],
                                left_v,
                                top_right_v,
                                weights_y_v,
                                weights_x_v[2],
                                scaled_weights_x[2],
                                weighted_bl,
                            ),
                        );
                        vst1q_u8(
                            dst.add(48),
                            calculate_weights_and_predq(
                                top_v[3],
                                left_v,
                                top_right_v,
                                weights_y_v,
                                weights_x_v[3],
                                scaled_weights_x[3],
                                weighted_bl,
                            ),
                        );
                    }
                }

                dst = dst.offset(stride);
            }
        }
    };
}

smooth_predictor!(smooth_16xh_neon, 16);
smooth_predictor!(smooth_32xh_neon, 32);
smooth_predictor!(smooth_64xh_neon, 64);

smooth_nxm!(aom_smooth_predictor_16x4_neon, smooth_16xh_neon, 4);
smooth_nxm!(aom_smooth_predictor_16x64_neon, smooth_16xh_neon, 64);
smooth_nxm!(aom_smooth_predictor_32x8_neon, smooth_32xh_neon, 8);
smooth_nxm!(aom_smooth_predictor_64x16_neon, smooth_64xh_neon, 16);
smooth_nxm!(aom_smooth_predictor_16x8_neon, smooth_16xh_neon, 8);
smooth_nxm!(aom_smooth_predictor_16x16_neon, smooth_16xh_neon, 16);
smooth_nxm!(aom_smooth_predictor_16x32_neon, smooth_16xh_neon, 32);
smooth_nxm!(aom_smooth_predictor_32x16_neon, smooth_32xh_neon, 16);
smooth_nxm!(aom_smooth_predictor_32x32_neon, smooth_32xh_neon, 32);
smooth_nxm!(aom_smooth_predictor_32x64_neon, smooth_32xh_neon, 64);
smooth_nxm!(aom_smooth_predictor_64x32_neon, smooth_64xh_neon, 32);
smooth_nxm!(aom_smooth_predictor_64x64_neon, smooth_64xh_neon, 64);

// -----------------------------------------------------------------------------
// SMOOTH_V_PRED

// For widths 4 and 8.
// #define SMOOTH_V_PREDICTOR(W)
macro_rules! smooth_v_predictor_narrow {
    ($name:ident, $W:literal) => {
        unsafe fn $name(
            mut dst: *mut u8,
            stride: isize,
            top_row: *const u8,
            left_column: *const u8,
            height: i32,
        ) {
            let bottom_left: u8 = *left_column.offset((height - 1) as isize);
            let weights_y: *const u8 = smooth_weights_ptr().offset((height - 4) as isize);

            let top_v: uint8x8_t;
            if ($W) == 4 {
                top_v = load_u8_4x1(top_row);
            } else {
                /* width == 8 */
                top_v = vld1_u8(top_row);
            }

            let bottom_left_v = vdup_n_u8(bottom_left);

            debug_assert!(height > 0);
            let mut y: i32 = 0;
            loop {
                let weights_y_v = vdup_n_u8(*weights_y.offset(y as isize));
                let scaled_weights_y = negate_s8(weights_y_v);

                let weighted_top = vmull_u8(weights_y_v, top_v);
                let weighted_top_bl = vmlal_u8(weighted_top, scaled_weights_y, bottom_left_v);
                let pred = vrshrn_n_u16::<SMOOTH_WEIGHT_LOG2_SCALE>(weighted_top_bl);

                if ($W) == 4 {
                    vst1_lane_u32::<0>(dst as *mut u32, vreinterpret_u32_u8(pred));
                } else {
                    /* width == 8 */
                    vst1_u8(dst, pred);
                }
                dst = dst.offset(stride);
                y += 1;
                if y == height {
                    break;
                }
            }
        }
    };
}

smooth_v_predictor_narrow!(smooth_v_4xh_neon, 4);
smooth_v_predictor_narrow!(smooth_v_8xh_neon, 8);

smooth_nxm!(aom_smooth_v_predictor_4x16_neon, smooth_v_4xh_neon, 16);
smooth_nxm!(aom_smooth_v_predictor_8x32_neon, smooth_v_8xh_neon, 32);
smooth_nxm!(aom_smooth_v_predictor_4x4_neon, smooth_v_4xh_neon, 4);
smooth_nxm!(aom_smooth_v_predictor_4x8_neon, smooth_v_4xh_neon, 8);
smooth_nxm!(aom_smooth_v_predictor_8x4_neon, smooth_v_8xh_neon, 4);
smooth_nxm!(aom_smooth_v_predictor_8x8_neon, smooth_v_8xh_neon, 8);
smooth_nxm!(aom_smooth_v_predictor_8x16_neon, smooth_v_8xh_neon, 16);

#[inline(always)]
unsafe fn calculate_vertical_weights_and_pred(
    top: uint8x16_t,
    weights_y: uint8x8_t,
    weighted_bl: uint16x8_t,
) -> uint8x16_t {
    let pred_low = vmlal_u8(weighted_bl, weights_y, vget_low_u8(top));
    let pred_high = vmlal_u8(weighted_bl, weights_y, vget_high_u8(top));
    let pred_scaled_low = vrshrn_n_u16::<SMOOTH_WEIGHT_LOG2_SCALE>(pred_low);
    let pred_scaled_high = vrshrn_n_u16::<SMOOTH_WEIGHT_LOG2_SCALE>(pred_high);
    vcombine_u8(pred_scaled_low, pred_scaled_high)
}

// For width 16 and above.
// #define SMOOTH_V_PREDICTOR(W)
macro_rules! smooth_v_predictor {
    ($name:ident, $W:literal) => {
        unsafe fn $name(
            mut dst: *mut u8,
            stride: isize,
            top_row: *const u8,
            left_column: *const u8,
            height: i32,
        ) {
            let bottom_left: u8 = *left_column.offset((height - 1) as isize);
            let weights_y: *const u8 = smooth_weights_ptr().offset((height - 4) as isize);

            let mut top_v: [uint8x16_t; 4] = core::mem::zeroed();
            top_v[0] = vld1q_u8(top_row);
            if ($W) > 16 {
                top_v[1] = vld1q_u8(top_row.add(16));
                if ($W) == 64 {
                    top_v[2] = vld1q_u8(top_row.add(32));
                    top_v[3] = vld1q_u8(top_row.add(48));
                }
            }

            let bottom_left_v = vdup_n_u8(bottom_left);

            debug_assert!(height > 0);
            let mut y: i32 = 0;
            loop {
                let weights_y_v = vdup_n_u8(*weights_y.offset(y as isize));
                let scaled_weights_y = negate_s8(weights_y_v);
                let weighted_bl = vmull_u8(scaled_weights_y, bottom_left_v);

                let pred_0 = calculate_vertical_weights_and_pred(top_v[0], weights_y_v, weighted_bl);
                vst1q_u8(dst, pred_0);

                if ($W) > 16 {
                    let pred_1 =
                        calculate_vertical_weights_and_pred(top_v[1], weights_y_v, weighted_bl);
                    vst1q_u8(dst.add(16), pred_1);

                    if ($W) == 64 {
                        let pred_2 =
                            calculate_vertical_weights_and_pred(top_v[2], weights_y_v, weighted_bl);
                        vst1q_u8(dst.add(32), pred_2);

                        let pred_3 =
                            calculate_vertical_weights_and_pred(top_v[3], weights_y_v, weighted_bl);
                        vst1q_u8(dst.add(48), pred_3);
                    }
                }

                dst = dst.offset(stride);
                y += 1;
                if y == height {
                    break;
                }
            }
        }
    };
}

smooth_v_predictor!(smooth_v_16xh_neon, 16);
smooth_v_predictor!(smooth_v_32xh_neon, 32);
smooth_v_predictor!(smooth_v_64xh_neon, 64);

smooth_nxm!(aom_smooth_v_predictor_16x4_neon, smooth_v_16xh_neon, 4);
smooth_nxm!(aom_smooth_v_predictor_32x8_neon, smooth_v_32xh_neon, 8);
smooth_nxm!(aom_smooth_v_predictor_64x16_neon, smooth_v_64xh_neon, 16);
smooth_nxm!(aom_smooth_v_predictor_16x64_neon, smooth_v_16xh_neon, 64);
smooth_nxm!(aom_smooth_v_predictor_16x8_neon, smooth_v_16xh_neon, 8);
smooth_nxm!(aom_smooth_v_predictor_16x16_neon, smooth_v_16xh_neon, 16);
smooth_nxm!(aom_smooth_v_predictor_16x32_neon, smooth_v_16xh_neon, 32);
smooth_nxm!(aom_smooth_v_predictor_32x16_neon, smooth_v_32xh_neon, 16);
smooth_nxm!(aom_smooth_v_predictor_32x32_neon, smooth_v_32xh_neon, 32);
smooth_nxm!(aom_smooth_v_predictor_32x64_neon, smooth_v_32xh_neon, 64);
smooth_nxm!(aom_smooth_v_predictor_64x32_neon, smooth_v_64xh_neon, 32);
smooth_nxm!(aom_smooth_v_predictor_64x64_neon, smooth_v_64xh_neon, 64);

// -----------------------------------------------------------------------------
// SMOOTH_H_PRED

// For widths 4 and 8.
// #define SMOOTH_H_PREDICTOR(W)
macro_rules! smooth_h_predictor_narrow {
    ($name:ident, $W:literal) => {
        unsafe fn $name(
            mut dst: *mut u8,
            stride: isize,
            top_row: *const u8,
            left_column: *const u8,
            height: i32,
        ) {
            let top_right: u8 = *top_row.add(($W) - 1);

            let top_right_v = vdup_n_u8(top_right);
            /* Over-reads for 4xN but still within the array. */
            let weights_x = vld1_u8(smooth_weights_ptr().add(($W) - 4));
            let scaled_weights_x = negate_s8(weights_x);
            let weighted_tr = vmull_u8(scaled_weights_x, top_right_v);

            debug_assert!(height > 0);
            let mut y: i32 = 0;
            loop {
                let left_v = vdup_n_u8(*left_column.offset(y as isize));
                let weighted_left_tr = vmlal_u8(weighted_tr, weights_x, left_v);
                let pred = vrshrn_n_u16::<SMOOTH_WEIGHT_LOG2_SCALE>(weighted_left_tr);

                if ($W) == 4 {
                    vst1_lane_u32::<0>(dst as *mut u32, vreinterpret_u32_u8(pred));
                } else {
                    /* width == 8 */
                    vst1_u8(dst, pred);
                }
                dst = dst.offset(stride);
                y += 1;
                if y == height {
                    break;
                }
            }
        }
    };
}

smooth_h_predictor_narrow!(smooth_h_4xh_neon, 4);
smooth_h_predictor_narrow!(smooth_h_8xh_neon, 8);

smooth_nxm!(aom_smooth_h_predictor_4x16_neon, smooth_h_4xh_neon, 16);
smooth_nxm!(aom_smooth_h_predictor_8x32_neon, smooth_h_8xh_neon, 32);
smooth_nxm!(aom_smooth_h_predictor_4x4_neon, smooth_h_4xh_neon, 4);
smooth_nxm!(aom_smooth_h_predictor_4x8_neon, smooth_h_4xh_neon, 8);
smooth_nxm!(aom_smooth_h_predictor_8x4_neon, smooth_h_8xh_neon, 4);
smooth_nxm!(aom_smooth_h_predictor_8x8_neon, smooth_h_8xh_neon, 8);
smooth_nxm!(aom_smooth_h_predictor_8x16_neon, smooth_h_8xh_neon, 16);

#[inline(always)]
unsafe fn calculate_horizontal_weights_and_pred(
    left: uint8x8_t,
    top_right: uint8x8_t,
    weights_x: uint8x16_t,
    scaled_weights_x: uint8x16_t,
) -> uint8x16_t {
    let weighted_left_low = vmull_u8(vget_low_u8(weights_x), left);
    let weighted_left_tr_low = vmlal_u8(weighted_left_low, vget_low_u8(scaled_weights_x), top_right);
    let pred_scaled_low = vrshrn_n_u16::<SMOOTH_WEIGHT_LOG2_SCALE>(weighted_left_tr_low);

    let weighted_left_high = vmull_u8(vget_high_u8(weights_x), left);
    let weighted_left_tr_high =
        vmlal_u8(weighted_left_high, vget_high_u8(scaled_weights_x), top_right);
    let pred_scaled_high = vrshrn_n_u16::<SMOOTH_WEIGHT_LOG2_SCALE>(weighted_left_tr_high);

    vcombine_u8(pred_scaled_low, pred_scaled_high)
}

// For width 16 and above.
// #define SMOOTH_H_PREDICTOR(W)
macro_rules! smooth_h_predictor {
    ($name:ident, $W:literal) => {
        unsafe fn $name(
            mut dst: *mut u8,
            stride: isize,
            top_row: *const u8,
            left_column: *const u8,
            height: i32,
        ) {
            let top_right: u8 = *top_row.add(($W) - 1);

            let top_right_v = vdup_n_u8(top_right);

            let mut weights_x: [uint8x16_t; 4] = core::mem::zeroed();
            weights_x[0] = vld1q_u8(smooth_weights_ptr().add(($W) - 4));
            if ($W) > 16 {
                weights_x[1] = vld1q_u8(smooth_weights_ptr().add(($W) + 16 - 4));
                if ($W) == 64 {
                    weights_x[2] = vld1q_u8(smooth_weights_ptr().add(($W) + 32 - 4));
                    weights_x[3] = vld1q_u8(smooth_weights_ptr().add(($W) + 48 - 4));
                }
            }

            let mut scaled_weights_x: [uint8x16_t; 4] = core::mem::zeroed();
            scaled_weights_x[0] = negate_s8q(weights_x[0]);
            if ($W) > 16 {
                scaled_weights_x[1] = negate_s8q(weights_x[1]);
                if ($W) == 64 {
                    scaled_weights_x[2] = negate_s8q(weights_x[2]);
                    scaled_weights_x[3] = negate_s8q(weights_x[3]);
                }
            }

            debug_assert!(height > 0);
            let mut y: i32 = 0;
            loop {
                let left_v = vdup_n_u8(*left_column.offset(y as isize));

                let pred_0 = calculate_horizontal_weights_and_pred(
                    left_v,
                    top_right_v,
                    weights_x[0],
                    scaled_weights_x[0],
                );
                vst1q_u8(dst, pred_0);

                if ($W) > 16 {
                    let pred_1 = calculate_horizontal_weights_and_pred(
                        left_v,
                        top_right_v,
                        weights_x[1],
                        scaled_weights_x[1],
                    );
                    vst1q_u8(dst.add(16), pred_1);

                    if ($W) == 64 {
                        let pred_2 = calculate_horizontal_weights_and_pred(
                            left_v,
                            top_right_v,
                            weights_x[2],
                            scaled_weights_x[2],
                        );
                        vst1q_u8(dst.add(32), pred_2);

                        let pred_3 = calculate_horizontal_weights_and_pred(
                            left_v,
                            top_right_v,
                            weights_x[3],
                            scaled_weights_x[3],
                        );
                        vst1q_u8(dst.add(48), pred_3);
                    }
                }
                dst = dst.offset(stride);
                y += 1;
                if y == height {
                    break;
                }
            }
        }
    };
}

smooth_h_predictor!(smooth_h_16xh_neon, 16);
smooth_h_predictor!(smooth_h_32xh_neon, 32);
smooth_h_predictor!(smooth_h_64xh_neon, 64);

smooth_nxm!(aom_smooth_h_predictor_16x4_neon, smooth_h_16xh_neon, 4);
smooth_nxm!(aom_smooth_h_predictor_16x64_neon, smooth_h_16xh_neon, 64);
smooth_nxm!(aom_smooth_h_predictor_32x8_neon, smooth_h_32xh_neon, 8);
smooth_nxm!(aom_smooth_h_predictor_64x16_neon, smooth_h_64xh_neon, 16);
smooth_nxm!(aom_smooth_h_predictor_16x8_neon, smooth_h_16xh_neon, 8);
smooth_nxm!(aom_smooth_h_predictor_16x16_neon, smooth_h_16xh_neon, 16);
smooth_nxm!(aom_smooth_h_predictor_16x32_neon, smooth_h_16xh_neon, 32);
smooth_nxm!(aom_smooth_h_predictor_32x16_neon, smooth_h_32xh_neon, 16);
smooth_nxm!(aom_smooth_h_predictor_32x32_neon, smooth_h_32xh_neon, 32);
smooth_nxm!(aom_smooth_h_predictor_32x64_neon, smooth_h_32xh_neon, 64);
smooth_nxm!(aom_smooth_h_predictor_64x32_neon, smooth_h_64xh_neon, 32);
smooth_nxm!(aom_smooth_h_predictor_64x64_neon, smooth_h_64xh_neon, 64);

// -----------------------------------------------------------------------------
// PAETH

#[inline(always)]
unsafe fn paeth_4or8_x_h_neon(
    mut dest: *mut u8,
    stride: isize,
    top_row: *const u8,
    left_column: *const u8,
    width: i32,
    height: i32,
) {
    let top_left = vdup_n_u8(*top_row.offset(-1));
    let top_left_x2 = vdupq_n_u16((*top_row.offset(-1) as i32 + *top_row.offset(-1) as i32) as u16);
    let top: uint8x8_t;
    if width == 4 {
        top = load_u8_4x1(top_row);
    } else {
        // width == 8
        top = vld1_u8(top_row);
    }

    debug_assert!(height > 0);
    let mut y: i32 = 0;
    loop {
        let left = vdup_n_u8(*left_column.offset(y as isize));

        let left_dist = vabd_u8(top, top_left);
        let top_dist = vabd_u8(left, top_left);
        let top_left_dist = vabdq_u16(vaddl_u8(top, left), top_left_x2);

        let left_le_top = vcle_u8(left_dist, top_dist);
        let left_le_top_left = vmovn_u16(vcleq_u16(vmovl_u8(left_dist), top_left_dist));
        let top_le_top_left = vmovn_u16(vcleq_u16(vmovl_u8(top_dist), top_left_dist));

        // if (left_dist <= top_dist && left_dist <= top_left_dist)
        let left_mask = vand_u8(left_le_top, left_le_top_left);
        //   dest[x] = left_column[y];
        // Fill all the unused spaces with 'top'. They will be overwritten when
        // the positions for top_left are known.
        let mut result = vbsl_u8(left_mask, left, top);
        // else if (top_dist <= top_left_dist)
        //   dest[x] = top_row[x];
        // Add these values to the mask. They were already set.
        let left_or_top_mask = vorr_u8(left_mask, top_le_top_left);
        // else
        //   dest[x] = top_left;
        result = vbsl_u8(left_or_top_mask, result, top_left);

        if width == 4 {
            store_u8_4x1(dest, result);
        } else {
            // width == 8
            vst1_u8(dest, result);
        }
        dest = dest.offset(stride);
        y += 1;
        if y == height {
            break;
        }
    }
}

// #define PAETH_NXM(W, H) and PAETH_NXM_WIDE(W, H)
macro_rules! paeth_nxm {
    ($name:ident, $helper:ident, $w:literal, $h:literal) => {
        #[no_mangle]
        pub unsafe extern "C-unwind" fn $name(
            dst: *mut u8,
            stride: isize,
            above: *const u8,
            left: *const u8,
        ) {
            $helper(dst, stride, above, left, $w, $h);
        }
    };
}

paeth_nxm!(aom_paeth_predictor_4x16_neon, paeth_4or8_x_h_neon, 4, 16);
paeth_nxm!(aom_paeth_predictor_8x32_neon, paeth_4or8_x_h_neon, 8, 32);
paeth_nxm!(aom_paeth_predictor_4x4_neon, paeth_4or8_x_h_neon, 4, 4);
paeth_nxm!(aom_paeth_predictor_4x8_neon, paeth_4or8_x_h_neon, 4, 8);
paeth_nxm!(aom_paeth_predictor_8x4_neon, paeth_4or8_x_h_neon, 8, 4);
paeth_nxm!(aom_paeth_predictor_8x8_neon, paeth_4or8_x_h_neon, 8, 8);
paeth_nxm!(aom_paeth_predictor_8x16_neon, paeth_4or8_x_h_neon, 8, 16);

// Calculate X distance <= TopLeft distance and pack the resulting mask into
// uint8x8_t.
#[inline(always)]
unsafe fn x_le_top_left(
    x_dist: uint8x16_t,
    top_left_dist_low: uint16x8_t,
    top_left_dist_high: uint16x8_t,
) -> uint8x16_t {
    let top_left_dist = vcombine_u8(vqmovn_u16(top_left_dist_low), vqmovn_u16(top_left_dist_high));
    vcleq_u8(x_dist, top_left_dist)
}

// Select the closest values and collect them.
#[inline(always)]
unsafe fn select_paeth(
    top: uint8x16_t,
    left: uint8x16_t,
    top_left: uint8x16_t,
    left_le_top: uint8x16_t,
    left_le_top_left: uint8x16_t,
    top_le_top_left: uint8x16_t,
) -> uint8x16_t {
    // if (left_dist <= top_dist && left_dist <= top_left_dist)
    let left_mask = vandq_u8(left_le_top, left_le_top_left);
    //   dest[x] = left_column[y];
    // Fill all the unused spaces with 'top'. They will be overwritten when
    // the positions for top_left are known.
    let result = vbslq_u8(left_mask, left, top);
    // else if (top_dist <= top_left_dist)
    //   dest[x] = top_row[x];
    // Add these values to the mask. They were already set.
    let left_or_top_mask = vorrq_u8(left_mask, top_le_top_left);
    // else
    //   dest[x] = top_left;
    vbslq_u8(left_or_top_mask, result, top_left)
}

// The numbered TOP_LEFT_DIST(num), LEFT_LE_TOP_LEFT(num) and
// TOP_LE_TOP_LEFT(num) steps of one 16-wide column group, followed by
// select_paeth, in the original's order.
macro_rules! paeth16_column {
    ($top:expr, $left:ident, $top_left:ident, $top_left_x2:ident, $top_dist:ident) => {{
        let left_n_dist = vabdq_u8($top, $top_left);
        // TOP_LEFT_DIST(num)
        let top_left_n_dist_low =
            vabdq_u16(vaddl_u8(vget_low_u8($top), vget_low_u8($left)), $top_left_x2);
        let top_left_n_dist_high =
            vabdq_u16(vaddl_u8(vget_high_u8($top), vget_low_u8($left)), $top_left_x2);
        let left_n_le_top = vcleq_u8(left_n_dist, $top_dist);
        // LEFT_LE_TOP_LEFT(num)
        let left_le_top_left_n = x_le_top_left(left_n_dist, top_left_n_dist_low, top_left_n_dist_high);
        // TOP_LE_TOP_LEFT(num)
        let top_le_top_left_n = x_le_top_left($top_dist, top_left_n_dist_low, top_left_n_dist_high);

        select_paeth($top, $left, $top_left, left_n_le_top, left_le_top_left_n, top_le_top_left_n)
    }};
}

#[inline(always)]
unsafe fn paeth16_plus_x_h_neon(
    mut dest: *mut u8,
    stride: isize,
    top_row: *const u8,
    left_column: *const u8,
    width: i32,
    height: i32,
) {
    let top_left = vdupq_n_u8(*top_row.offset(-1));
    let top_left_x2 = vdupq_n_u16((*top_row.offset(-1) as i32 + *top_row.offset(-1) as i32) as u16);
    let mut top: [uint8x16_t; 4] = core::mem::zeroed();
    top[0] = vld1q_u8(top_row);
    if width > 16 {
        top[1] = vld1q_u8(top_row.add(16));
        if width == 64 {
            top[2] = vld1q_u8(top_row.add(32));
            top[3] = vld1q_u8(top_row.add(48));
        }
    }

    debug_assert!(height > 0);
    let mut y: i32 = 0;
    loop {
        let left = vdupq_n_u8(*left_column.offset(y as isize));

        let top_dist = vabdq_u8(left, top_left);

        let result_0 = paeth16_column!(top[0], left, top_left, top_left_x2, top_dist);
        vst1q_u8(dest, result_0);

        if width > 16 {
            let result_1 = paeth16_column!(top[1], left, top_left, top_left_x2, top_dist);
            vst1q_u8(dest.add(16), result_1);

            if width == 64 {
                let result_2 = paeth16_column!(top[2], left, top_left, top_left_x2, top_dist);
                vst1q_u8(dest.add(32), result_2);

                let result_3 = paeth16_column!(top[3], left, top_left, top_left_x2, top_dist);
                vst1q_u8(dest.add(48), result_3);
            }
        }

        dest = dest.offset(stride);
        y += 1;
        if y == height {
            break;
        }
    }
}

paeth_nxm!(aom_paeth_predictor_16x4_neon, paeth16_plus_x_h_neon, 16, 4);
paeth_nxm!(aom_paeth_predictor_16x64_neon, paeth16_plus_x_h_neon, 16, 64);
paeth_nxm!(aom_paeth_predictor_32x8_neon, paeth16_plus_x_h_neon, 32, 8);
paeth_nxm!(aom_paeth_predictor_64x16_neon, paeth16_plus_x_h_neon, 64, 16);
paeth_nxm!(aom_paeth_predictor_16x8_neon, paeth16_plus_x_h_neon, 16, 8);
paeth_nxm!(aom_paeth_predictor_16x16_neon, paeth16_plus_x_h_neon, 16, 16);
paeth_nxm!(aom_paeth_predictor_16x32_neon, paeth16_plus_x_h_neon, 16, 32);
paeth_nxm!(aom_paeth_predictor_32x16_neon, paeth16_plus_x_h_neon, 32, 16);
paeth_nxm!(aom_paeth_predictor_32x32_neon, paeth16_plus_x_h_neon, 32, 32);
paeth_nxm!(aom_paeth_predictor_32x64_neon, paeth16_plus_x_h_neon, 32, 64);
paeth_nxm!(aom_paeth_predictor_64x32_neon, paeth16_plus_x_h_neon, 64, 32);
paeth_nxm!(aom_paeth_predictor_64x64_neon, paeth16_plus_x_h_neon, 64, 64);

/* ---------------------P R E D I C T I O N   Z 1--------------------------- */

// Low bit depth functions
// BaseMask[n] holds n leading 0xff bytes followed by zeros; the 33x32 table is
// generated here with the same contents as the original's literal.
const fn base_mask_table() -> [[u8; 32]; 33] {
    let mut t = [[0u8; 32]; 33];
    let mut i = 0;
    while i < 33 {
        let mut j = 0;
        while j < i {
            t[i][j] = 0xff;
            j += 1;
        }
        i += 1;
    }
    t
}

#[repr(C, align(32))]
struct Align32<T>(T);

#[repr(C, align(16))]
struct Align16<T>(T);

static BaseMask: Align32<[[u8; 32]; 33]> = Align32(base_mask_table());

#[inline(always)]
unsafe fn base_mask(n: i32) -> *const u8 {
    BaseMask.0[n as usize].as_ptr()
}

#[inline(always)]
unsafe fn dr_prediction_z1_WxH_internal_neon_small(
    W: i32,
    H: i32,
    dst: *mut uint8x8_t,
    above: *const u8,
    upsample_above: i32,
    dx: i32,
) {
    let frac_bits = 6 - upsample_above;
    let max_base_x = ((W + H) - 1) << upsample_above;

    debug_assert!(dx > 0);
    //   above0 * (32 - shift) + above1 * shift
    // = (above1 - above0) * shift + above0 * 32

    let a_mbase_x = vdup_n_u8(*above.offset(max_base_x as isize));

    let mut x = dx;
    for r in 0..H {
        let base = x >> frac_bits;
        let mut base_max_diff = (max_base_x - base) >> upsample_above;
        if base_max_diff <= 0 {
            for i in r..H {
                *dst.offset(i as isize) = a_mbase_x; // save 4 values
            }
            return;
        }

        if base_max_diff > W {
            base_max_diff = W;
        }

        let mut a01_128: uint8x8x2_t = core::mem::zeroed();
        let shift: uint16x8_t;
        if upsample_above != 0 {
            a01_128 = vld2_u8(above.offset(base as isize));
            shift = vdupq_n_u16((x & 0x1f) as u16); // ((x << upsample_above) & 0x3f) >> 1
        } else {
            a01_128.0 = vld1_u8(above.offset(base as isize));
            a01_128.1 = vld1_u8(above.offset(base as isize + 1));
            shift = vdupq_n_u16(((x & 0x3f) >> 1) as u16);
        }
        let diff = vsubl_u8(a01_128.1, a01_128.0);
        let a32 = vshll_n_u8::<5>(a01_128.0);
        let res = vmlaq_u16(a32, diff, shift);

        let mask = vld1_u8(base_mask(base_max_diff));
        *dst.offset(r as isize) = vbsl_u8(mask, vrshrn_n_u16::<5>(res), a_mbase_x);

        x += dx;
    }
}

unsafe fn dr_prediction_z1_4xH_neon(
    H: i32,
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    upsample_above: i32,
    dx: i32,
) {
    let mut dstvec: [uint8x8_t; 16] = core::mem::zeroed();

    dr_prediction_z1_WxH_internal_neon_small(4, H, dstvec.as_mut_ptr(), above, upsample_above, dx);
    for i in 0..H {
        vst1_lane_u32::<0>(
            dst.offset(stride * i as isize) as *mut u32,
            vreinterpret_u32_u8(dstvec[i as usize]),
        );
    }
}

unsafe fn dr_prediction_z1_8xH_neon(
    H: i32,
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    upsample_above: i32,
    dx: i32,
) {
    let mut dstvec: [uint8x8_t; 32] = core::mem::zeroed();

    dr_prediction_z1_WxH_internal_neon_small(8, H, dstvec.as_mut_ptr(), above, upsample_above, dx);
    for i in 0..H {
        vst1_u8(dst.offset(stride * i as isize), dstvec[i as usize]);
    }
}

#[inline(always)]
unsafe fn dr_prediction_z1_WxH_internal_neon_large(
    W: i32,
    H: i32,
    dst: *mut uint8x16_t,
    above: *const u8,
    dx: i32,
) {
    // Here upsample_above is 0 by design of av1_use_intra_edge_upsample defined
    // in av1/common/reconintra.h.
    let frac_bits = 6;
    let max_base_x = (W + H) - 1;

    debug_assert!(dx > 0);
    //   above0 * (32 - shift) + above1 * shift
    // = (above1 - above0) * shift + above0 * 32

    let a_mbase_x = vdupq_n_u8(*above.offset(max_base_x as isize));

    let mut x = dx;
    for r in 0..H {
        let base = x >> frac_bits;
        let mut base_max_diff = max_base_x - base;
        if base_max_diff <= 0 {
            for i in r..H {
                *dst.offset(i as isize) = a_mbase_x; // save 4 values
            }
            return;
        }

        if base_max_diff > W {
            base_max_diff = W;
        }

        let a0_128 = vld1q_u8(above.offset(base as isize));
        let a1_128 = vld1q_u8(above.offset(base as isize + 1));
        let shift = vdupq_n_u16(((x & 0x3f) >> 1) as u16);

        let diff_lo = vsubl_u8(vget_low_u8(a1_128), vget_low_u8(a0_128));
        let diff_hi = vsubl_u8(vget_high_u8(a1_128), vget_high_u8(a0_128));
        let a32_lo = vshll_n_u8::<5>(vget_low_u8(a0_128));
        let a32_hi = vshll_n_u8::<5>(vget_high_u8(a0_128));
        let res_lo = vmlaq_u16(a32_lo, diff_lo, shift);
        let res_hi = vmlaq_u16(a32_hi, diff_hi, shift);
        let v_temp = vcombine_u8(vrshrn_n_u16::<5>(res_lo), vrshrn_n_u16::<5>(res_hi));

        let mask = vld1q_u8(base_mask(base_max_diff));
        *dst.offset(r as isize) = vbslq_u8(mask, v_temp, a_mbase_x);

        x += dx;
    }
}

unsafe fn dr_prediction_z1_16xH_neon(H: i32, dst: *mut u8, stride: isize, above: *const u8, dx: i32) {
    let mut dstvec: [uint8x16_t; 64] = core::mem::zeroed();

    dr_prediction_z1_WxH_internal_neon_large(16, H, dstvec.as_mut_ptr(), above, dx);
    for i in 0..H {
        vst1q_u8(dst.offset(stride * i as isize), dstvec[i as usize]);
    }
}

#[inline(always)]
unsafe fn dr_prediction_z1_32xH_internal_neon(
    H: i32,
    dstvec: *mut uint8x16x2_t,
    above: *const u8,
    dx: i32,
) {
    // Here upsample_above is 0 by design of av1_use_intra_edge_upsample defined
    // in av1/common/reconintra.h.
    let frac_bits = 6;
    let max_base_x = (32 + H) - 1;

    //   above0 * (32 - shift) + above1 * shift
    // = (above1 - above0) * shift + above0 * 32

    let a_mbase_x = vdupq_n_u8(*above.offset(max_base_x as isize));

    let mut x = dx;
    for r in 0..H {
        let base = x >> frac_bits;
        let mut base_max_diff = max_base_x - base;
        if base_max_diff <= 0 {
            for i in r..H {
                (*dstvec.offset(i as isize)).0 = a_mbase_x; // save 32 values
                (*dstvec.offset(i as isize)).1 = a_mbase_x;
            }
            return;
        }
        if base_max_diff > 32 {
            base_max_diff = 32;
        }

        let shift = vdupq_n_u16(((x & 0x3f) >> 1) as u16);

        let mut res16: [uint8x16_t; 2] = core::mem::zeroed();
        let mut j = 0;
        let mut jj = 0;
        while j < 32 {
            let mdiff = base_max_diff - j;
            if mdiff <= 0 {
                res16[jj] = a_mbase_x;
            } else {
                let a0_128 = vld1q_u8(above.offset((base + j) as isize));
                let a1_128 = vld1q_u8(above.offset((base + j) as isize + 1));

                let diff_lo = vsubl_u8(vget_low_u8(a1_128), vget_low_u8(a0_128));
                let diff_hi = vsubl_u8(vget_high_u8(a1_128), vget_high_u8(a0_128));
                let a32_lo = vshll_n_u8::<5>(vget_low_u8(a0_128));
                let a32_hi = vshll_n_u8::<5>(vget_high_u8(a0_128));
                let res_lo = vmlaq_u16(a32_lo, diff_lo, shift);
                let res_hi = vmlaq_u16(a32_hi, diff_hi, shift);

                res16[jj] = vcombine_u8(vrshrn_n_u16::<5>(res_lo), vrshrn_n_u16::<5>(res_hi));
            }
            j += 16;
            jj += 1;
        }

        let mask_lo = vld1q_u8(base_mask(base_max_diff));
        let mask_hi = vld1q_u8(base_mask(base_max_diff).add(16));
        (*dstvec.offset(r as isize)).0 = vbslq_u8(mask_lo, res16[0], a_mbase_x);
        (*dstvec.offset(r as isize)).1 = vbslq_u8(mask_hi, res16[1], a_mbase_x);
        x += dx;
    }
}

unsafe fn dr_prediction_z1_32xH_neon(H: i32, dst: *mut u8, stride: isize, above: *const u8, dx: i32) {
    let mut dstvec: [uint8x16x2_t; 64] = core::mem::zeroed();

    dr_prediction_z1_32xH_internal_neon(H, dstvec.as_mut_ptr(), above, dx);
    for i in 0..H {
        vst1q_u8(dst.offset(stride * i as isize), dstvec[i as usize].0);
        vst1q_u8(dst.offset(stride * i as isize + 16), dstvec[i as usize].1);
    }
}

#[rustfmt::skip]
static kLoadMaxShuffles: [u8; 256] = [
  15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
  14, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
  13, 14, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
  12, 13, 14, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
  11, 12, 13, 14, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
  10, 11, 12, 13, 14, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
   9, 10, 11, 12, 13, 14, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
   8,  9, 10, 11, 12, 13, 14, 15, 15, 15, 15, 15, 15, 15, 15, 15,
   7,  8,  9, 10, 11, 12, 13, 14, 15, 15, 15, 15, 15, 15, 15, 15,
   6,  7,  8,  9, 10, 11, 12, 13, 14, 15, 15, 15, 15, 15, 15, 15,
   5,  6,  7,  8,  9, 10, 11, 12, 13, 14, 15, 15, 15, 15, 15, 15,
   4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14, 15, 15, 15, 15, 15,
   3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14, 15, 15, 15, 15,
   2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14, 15, 15, 15,
   1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14, 15, 15,
   0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14, 15,
];

#[inline(always)]
unsafe fn z1_load_masked_neon(ptr: *const u8, shuffle_idx: i32) -> uint8x16_t {
    let shuffle = vld1q_u8(kLoadMaxShuffles.as_ptr().offset(16 * shuffle_idx as isize));
    let src = vld1q_u8(ptr);
    vqtbl1q_u8(src, shuffle)
}

unsafe fn dr_prediction_z1_64xH_neon(H: i32, mut dst: *mut u8, stride: isize, above: *const u8, dx: i32) {
    let frac_bits = 6;
    let max_base_x = (64 + H) - 1;

    //   above0 * (32 - shift) + above1 * shift
    // = (above1 - above0) * shift + above0 * 32

    let a_mbase_x = vdupq_n_u8(*above.offset(max_base_x as isize));

    let mut x = dx;
    let mut r = 0;
    while r < H {
        let base = x >> frac_bits;
        if base >= max_base_x {
            for _i in r..H {
                vst1q_u8(dst, a_mbase_x);
                vst1q_u8(dst.add(16), a_mbase_x);
                vst1q_u8(dst.add(32), a_mbase_x);
                vst1q_u8(dst.add(48), a_mbase_x);
                dst = dst.offset(stride);
            }
            return;
        }

        let shift = vdupq_n_u16(((x & 0x3f) >> 1) as u16);

        let mut j = 0;
        while j < 64 {
            if base + j >= max_base_x {
                vst1q_u8(dst.offset(j as isize), a_mbase_x);
            } else {
                let a0_128: uint8x16_t;
                let a1_128: uint8x16_t;
                if base + j + 15 >= max_base_x {
                    let shuffle_idx = max_base_x - base - j;
                    a0_128 = z1_load_masked_neon(above.offset((max_base_x - 15) as isize), shuffle_idx);
                } else {
                    a0_128 = vld1q_u8(above.offset((base + j) as isize));
                }
                if base + j + 16 >= max_base_x {
                    let shuffle_idx = max_base_x - base - j - 1;
                    a1_128 = z1_load_masked_neon(above.offset((max_base_x - 15) as isize), shuffle_idx);
                } else {
                    a1_128 = vld1q_u8(above.offset((base + j) as isize + 1));
                }

                let diff_lo = vsubl_u8(vget_low_u8(a1_128), vget_low_u8(a0_128));
                let diff_hi = vsubl_u8(vget_high_u8(a1_128), vget_high_u8(a0_128));
                let a32_lo = vshll_n_u8::<5>(vget_low_u8(a0_128));
                let a32_hi = vshll_n_u8::<5>(vget_high_u8(a0_128));
                let res_lo = vmlaq_u16(a32_lo, diff_lo, shift);
                let res_hi = vmlaq_u16(a32_hi, diff_hi, shift);
                vst1q_u8(
                    dst.offset(j as isize),
                    vcombine_u8(vrshrn_n_u16::<5>(res_lo), vrshrn_n_u16::<5>(res_hi)),
                );
            }
            j += 16;
        }
        x += dx;
        r += 1;
        dst = dst.offset(stride);
    }
}

// Directional prediction, zone 1: 0 < angle < 90
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_dr_prediction_z1_neon(
    dst: *mut u8,
    stride: isize,
    bw: i32,
    bh: i32,
    above: *const u8,
    left: *const u8,
    upsample_above: i32,
    dx: i32,
    dy: i32,
) {
    match bw {
        4 => dr_prediction_z1_4xH_neon(bh, dst, stride, above, upsample_above, dx),
        8 => dr_prediction_z1_8xH_neon(bh, dst, stride, above, upsample_above, dx),
        16 => dr_prediction_z1_16xH_neon(bh, dst, stride, above, dx),
        32 => dr_prediction_z1_32xH_neon(bh, dst, stride, above, dx),
        64 => dr_prediction_z1_64xH_neon(bh, dst, stride, above, dx),
        _ => {}
    }
}

/* ---------------------P R E D I C T I O N   Z 2--------------------------- */

#[inline(always)]
unsafe fn dr_prediction_z2_4xH_above_neon(
    above: *const u8,
    upsample_above: i32,
    dx: i32,
    base_x: i32,
    y: i32,
    a0_x: *mut uint8x8_t,
    a1_x: *mut uint8x8_t,
    shift0: *mut uint16x4_t,
) {
    let x = -y * dx;

    if upsample_above != 0 {
        // Cannot use LD2 here since we only want to load eight bytes, but LD2 can
        // only load either 16 or 32.
        let v_tmp = vld1_u8(above.offset(base_x as isize));
        *a0_x = vuzp_u8(v_tmp, vdup_n_u8(0)).0;
        *a1_x = vuzp_u8(v_tmp, vdup_n_u8(0)).1;
        *shift0 = vdup_n_u16((x & 0x1f) as u16); // ((x << upsample_above) & 0x3f) >> 1
    } else {
        *a0_x = load_unaligned_u8_4x1(above.offset(base_x as isize));
        *a1_x = load_unaligned_u8_4x1(above.offset(base_x as isize + 1));
        *shift0 = vdup_n_u16(((x & 0x3f) >> 1) as u16);
    }
}

#[inline(always)]
unsafe fn dr_prediction_z2_4xH_left_neon(
    left_vals: uint8x16x2_t,
    upsample_left: i32,
    dy: i32,
    r: i32,
    frac_bits_y: i32,
    a0_y: *mut uint16x4_t,
    a1_y: *mut uint16x4_t,
    shift1: *mut uint16x4_t,
) {
    let dy64 = vdup_n_s16(dy as i16);
    let v_1234 = vcreate_s16(0x0004000300020001);
    let v_frac_bits_y = vdup_n_s16((-frac_bits_y) as i16);
    let v_r6 = vdup_n_s16((r << 6) as i16);
    let y_c64 = vmls_s16(v_r6, v_1234, dy64);
    let base_y_c64 = vshl_s16(y_c64, v_frac_bits_y);

    let left_idx0 = vreinterpret_u8_s16(vadd_s16(base_y_c64, vdup_n_s16(2))); // [0, 16]
    let left_idx1 = vreinterpret_u8_s16(vadd_s16(base_y_c64, vdup_n_s16(3))); // [1, 17]

    *a0_y = vreinterpret_u16_u8(vqtbl2_u8(left_vals, left_idx0));
    *a1_y = vreinterpret_u16_u8(vqtbl2_u8(left_vals, left_idx1));

    if upsample_left != 0 {
        *shift1 = vand_u16(vreinterpret_u16_s16(y_c64), vdup_n_u16(0x1f)); // ((y << upsample_left) & 0x3f) >> 1
    } else {
        *shift1 = vand_u16(vshr_n_u16::<1>(vreinterpret_u16_s16(y_c64)), vdup_n_u16(0x1f));
    }
}

#[inline(always)]
unsafe fn dr_prediction_z2_8xH_above_neon(
    above: *const u8,
    upsample_above: i32,
    dx: i32,
    base_x: i32,
    y: i32,
) -> uint8x8_t {
    let x = -y * dx;

    let shift0: uint16x8_t;
    let a0_x0: uint8x8_t;
    let a1_x0: uint8x8_t;
    if upsample_above != 0 {
        let v_tmp = vld2_u8(above.offset(base_x as isize));
        a0_x0 = v_tmp.0;
        a1_x0 = v_tmp.1;
        shift0 = vdupq_n_u16((x & 0x1f) as u16); // ((x << upsample_above) & 0x3f) >> 1
    } else {
        a0_x0 = vld1_u8(above.offset(base_x as isize));
        a1_x0 = vld1_u8(above.offset(base_x as isize + 1));
        shift0 = vdupq_n_u16(((x & 0x3f) >> 1) as u16);
    }

    let diff0 = vsubl_u8(a1_x0, a0_x0); // a[x+1] - a[x]
    let a32 = vshll_n_u8::<5>(a0_x0); // a[x] * 32
    let res = vmlaq_u16(a32, diff0, shift0);

    vrshrn_n_u16::<5>(res)
}

#[inline(always)]
unsafe fn dr_prediction_z2_8xH_left_neon(
    left_vals: uint8x16x3_t,
    upsample_left: i32,
    dy: i32,
    r: i32,
    frac_bits_y: i32,
) -> uint8x8_t {
    let v_r6 = vdupq_n_s16((r << 6) as i16);
    let dy128 = vdupq_n_s16(dy as i16);
    let v_frac_bits_y = vdupq_n_s16((-frac_bits_y) as i16);

    let c1234 = vcombine_u16(vcreate_u16(0x0004000300020001), vcreate_u16(0x0008000700060005));
    let y_c128 = vmlsq_s16(v_r6, vreinterpretq_s16_u16(c1234), dy128);
    let base_y_c128 = vshlq_s16(y_c128, v_frac_bits_y);

    let left_idx0 = vreinterpretq_u8_s16(vaddq_s16(base_y_c128, vdupq_n_s16(2))); // [0, 33]
    let left_idx1 = vreinterpretq_u8_s16(vaddq_s16(base_y_c128, vdupq_n_s16(3))); // [1, 34]
    let left_idx01 = vuzp1q_u8(left_idx0, left_idx1);

    let a01_x = vqtbl3q_u8(left_vals, left_idx01);
    let a0_x1 = vget_low_u8(a01_x);
    let a1_x1 = vget_high_u8(a01_x);

    let shift1: uint16x8_t;
    if upsample_left != 0 {
        shift1 = vandq_u16(vreinterpretq_u16_s16(y_c128), vdupq_n_u16(0x1f)); // ((y << upsample_left) & 0x3f) >> 1
    } else {
        shift1 = vshrq_n_u16::<1>(vandq_u16(vreinterpretq_u16_s16(y_c128), vdupq_n_u16(0x3f)));
    }

    let diff1 = vsubl_u8(a1_x1, a0_x1);
    let a32 = vshll_n_u8::<5>(a0_x1);
    let res = vmlaq_u16(a32, diff1, shift1);

    vrshrn_n_u16::<5>(res)
}

#[inline(always)]
unsafe fn dr_prediction_z2_WxH_above_neon(above: *const u8, dx: i32, base_x: i32, y: i32, j: i32) -> uint8x16_t {
    let x = -y * dx;
    let a0_x128 = vld1q_u8(above.offset((base_x + j) as isize));
    let a1_x128 = vld1q_u8(above.offset((base_x + j) as isize + 1));
    let shift = vdupq_n_u16(((x & 0x3f) >> 1) as u16);

    // a[x+1] - a[x]
    let diff0 = vsubl_u8(vget_low_u8(a1_x128), vget_low_u8(a0_x128));
    let diff1 = vsubl_u8(vget_high_u8(a1_x128), vget_high_u8(a0_x128));

    // a[x] * 32
    let a32_0 = vshll_n_u8::<5>(vget_low_u8(a0_x128));
    let a32_1 = vshll_n_u8::<5>(vget_high_u8(a0_x128));
    let res0 = vmlaq_u16(a32_0, diff0, shift);
    let res1 = vmlaq_u16(a32_1, diff1, shift);

    vcombine_u8(vrshrn_n_u16::<5>(res0), vrshrn_n_u16::<5>(res1))
}

#[inline(always)]
unsafe fn dr_prediction_z2_WxH_left_neon(
    left_vals0: uint8x16x4_t,
    left_vals1: uint8x16x4_t,
    dy: i32,
    r: i32,
    j: i32,
) -> uint8x16_t {
    // here upsample_above and upsample_left are 0 by design of
    // av1_use_intra_edge_upsample
    let dy256 = vdupq_n_s16(dy as i16);
    let j256 = vdupq_n_u16(j as u16);

    let c1234 = uint16x8x2_t(
        vcombine_u16(vcreate_u16(0x0004000300020001), vcreate_u16(0x0008000700060005)),
        vcombine_u16(vcreate_u16(0x000C000B000A0009), vcreate_u16(0x0010000F000E000D)),
    );

    let v_r6 = vdupq_n_s16((r << 6) as i16);

    let c256_0 = vreinterpretq_s16_u16(vaddq_u16(j256, c1234.0));
    let c256_1 = vreinterpretq_s16_u16(vaddq_u16(j256, c1234.1));

    let y_c256_lo = vmlsq_s16(v_r6, c256_0, dy256);
    let y_c256_hi = vmlsq_s16(v_r6, c256_1, dy256);

    let base_y_c256_lo = vshrq_n_s16::<6>(y_c256_lo);
    let base_y_c256_hi = vshrq_n_s16::<6>(y_c256_hi);

    // Values in left_idx{0,1} range from 0 through 63 inclusive.
    let left_idx0 = vreinterpretq_u8_s16(vaddq_s16(base_y_c256_lo, vdupq_n_s16(1)));
    let left_idx1 = vreinterpretq_u8_s16(vaddq_s16(base_y_c256_hi, vdupq_n_s16(1)));
    let left_idx01 = vuzp1q_u8(left_idx0, left_idx1);

    let a0_y01 = vqtbl4q_u8(left_vals0, left_idx01);
    let a1_y01 = vqtbl4q_u8(left_vals1, left_idx01);

    let a0_y0 = vget_low_u8(a0_y01);
    let a0_y1 = vget_high_u8(a0_y01);
    let a1_y0 = vget_low_u8(a1_y01);
    let a1_y1 = vget_high_u8(a1_y01);

    let shifty_lo = vshrq_n_u16::<1>(vandq_u16(vreinterpretq_u16_s16(y_c256_lo), vdupq_n_u16(0x3f)));
    let shifty_hi = vshrq_n_u16::<1>(vandq_u16(vreinterpretq_u16_s16(y_c256_hi), vdupq_n_u16(0x3f)));

    // a[x+1] - a[x]
    let diff_lo = vsubl_u8(a1_y0, a0_y0);
    let diff_hi = vsubl_u8(a1_y1, a0_y1);
    // a[x] * 32 + 16
    let a32_lo = vshll_n_u8::<5>(a0_y0);
    let a32_hi = vshll_n_u8::<5>(a0_y1);

    let res0 = vmlaq_u16(a32_lo, diff_lo, shifty_lo);
    let res1 = vmlaq_u16(a32_hi, diff_hi, shifty_hi);

    vcombine_u8(vrshrn_n_u16::<5>(res0), vrshrn_n_u16::<5>(res1))
}

unsafe fn dr_prediction_z2_4xH_neon(
    H: i32,
    mut dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
    upsample_above: i32,
    upsample_left: i32,
    dx: i32,
    dy: i32,
) {
    let min_base_x = -(1 << upsample_above);
    let frac_bits_x = 6 - upsample_above;
    let frac_bits_y = 6 - upsample_left;

    debug_assert!(dx > 0);
    //   above0 * (32 - shift) + above1 * shift
    // = (above1 - above0) * shift + above0 * 32

    // Use ext rather than loading left + 14 directly to avoid over-read.
    let left_m2 = vld1q_u8(left.offset(-2));
    let left_0 = vld1q_u8(left);
    let left_14 = vextq_u8::<14>(left_0, left_0);
    let left_vals = uint8x16x2_t(left_m2, left_14);

    for r in 0..H {
        let y = r + 1;
        let base_x = (-y * dx) >> frac_bits_x;
        let base_min_diff = (min_base_x - ((-y * dx) >> frac_bits_x) + upsample_above) >> upsample_above;

        if base_min_diff <= 0 {
            let mut a0_x_u8: uint8x8_t = core::mem::zeroed();
            let mut a1_x_u8: uint8x8_t = core::mem::zeroed();
            let mut shift0: uint16x4_t = core::mem::zeroed();
            dr_prediction_z2_4xH_above_neon(
                above,
                upsample_above,
                dx,
                base_x,
                y,
                &raw mut a0_x_u8,
                &raw mut a1_x_u8,
                &raw mut shift0,
            );
            let a0_x = a0_x_u8;
            let a1_x = a1_x_u8;

            let diff = vsubl_u8(a1_x, a0_x); // a[x+1] - a[x]
            let a32 = vshll_n_u8::<5>(a0_x);
            let res = vmlaq_u16(a32, diff, vcombine_u16(shift0, vdup_n_u16(0)));
            let resx = vrshrn_n_u16::<5>(res);
            vst1_lane_u32::<0>(dst as *mut u32, vreinterpret_u32_u8(resx));
        } else if base_min_diff < 4 {
            let mut a0_x_u8: uint8x8_t = core::mem::zeroed();
            let mut a1_x_u8: uint8x8_t = core::mem::zeroed();
            let mut shift0: uint16x4_t = core::mem::zeroed();
            dr_prediction_z2_4xH_above_neon(
                above,
                upsample_above,
                dx,
                base_x,
                y,
                &raw mut a0_x_u8,
                &raw mut a1_x_u8,
                &raw mut shift0,
            );
            let mut a0_x = vmovl_u8(a0_x_u8);
            let mut a1_x = vmovl_u8(a1_x_u8);

            let mut a0_y: uint16x4_t = core::mem::zeroed();
            let mut a1_y: uint16x4_t = core::mem::zeroed();
            let mut shift1: uint16x4_t = core::mem::zeroed();
            dr_prediction_z2_4xH_left_neon(
                left_vals,
                upsample_left,
                dy,
                r,
                frac_bits_y,
                &raw mut a0_y,
                &raw mut a1_y,
                &raw mut shift1,
            );
            a0_x = vcombine_u16(vget_low_u16(a0_x), a0_y);
            a1_x = vcombine_u16(vget_low_u16(a1_x), a1_y);

            let shift = vcombine_u16(shift0, shift1);
            let diff = vsubq_u16(a1_x, a0_x); // a[x+1] - a[x]
            let a32 = vshlq_n_u16::<5>(a0_x);
            let res = vmlaq_u16(a32, diff, shift);
            let resx = vrshrn_n_u16::<5>(res);
            let resy = vext_u8::<4>(resx, vdup_n_u8(0));

            let mask = vld1_u8(base_mask(base_min_diff));
            let v_resxy = vbsl_u8(mask, resy, resx);
            vst1_lane_u32::<0>(dst as *mut u32, vreinterpret_u32_u8(v_resxy));
        } else {
            let mut a0_y: uint16x4_t = core::mem::zeroed();
            let mut a1_y: uint16x4_t = core::mem::zeroed();
            let mut shift1: uint16x4_t = core::mem::zeroed();
            dr_prediction_z2_4xH_left_neon(
                left_vals,
                upsample_left,
                dy,
                r,
                frac_bits_y,
                &raw mut a0_y,
                &raw mut a1_y,
                &raw mut shift1,
            );
            let diff = vsub_u16(a1_y, a0_y); // a[x+1] - a[x]
            let a32 = vshl_n_u16::<5>(a0_y);
            let res = vmla_u16(a32, diff, shift1);
            let resy = vrshrn_n_u16::<5>(vcombine_u16(res, vdup_n_u16(0)));

            vst1_lane_u32::<0>(dst as *mut u32, vreinterpret_u32_u8(resy));
        }

        dst = dst.offset(stride);
    }
}

unsafe fn dr_prediction_z2_8xH_neon(
    H: i32,
    mut dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
    upsample_above: i32,
    upsample_left: i32,
    dx: i32,
    dy: i32,
) {
    let min_base_x = -(1 << upsample_above);
    let frac_bits_x = 6 - upsample_above;
    let frac_bits_y = 6 - upsample_left;

    //   above0 * (32 - shift) + above1 * shift
    // = (above1 - above0) * shift + above0 * 32

    // Use ext rather than loading left + 30 directly to avoid over-read.
    let left_m2 = vld1q_u8(left.offset(-2));
    let left_0 = vld1q_u8(left.add(0));
    let left_16 = vld1q_u8(left.add(16));
    let left_14 = vextq_u8::<14>(left_0, left_16);
    let left_30 = vextq_u8::<14>(left_16, left_16);
    let left_vals = uint8x16x3_t(left_m2, left_14, left_30);

    for r in 0..H {
        let y = r + 1;
        let base_x = (-y * dx) >> frac_bits_x;
        let base_min_diff = (min_base_x - base_x + upsample_above) >> upsample_above;

        if base_min_diff <= 0 {
            let resx = dr_prediction_z2_8xH_above_neon(above, upsample_above, dx, base_x, y);
            vst1_u8(dst, resx);
        } else if base_min_diff < 8 {
            let resx = dr_prediction_z2_8xH_above_neon(above, upsample_above, dx, base_x, y);
            let resy = dr_prediction_z2_8xH_left_neon(left_vals, upsample_left, dy, r, frac_bits_y);
            let mask = vld1_u8(base_mask(base_min_diff));
            let resxy = vbsl_u8(mask, resy, resx);
            vst1_u8(dst, resxy);
        } else {
            let resy = dr_prediction_z2_8xH_left_neon(left_vals, upsample_left, dy, r, frac_bits_y);
            vst1_u8(dst, resy);
        }

        dst = dst.offset(stride);
    }
}

unsafe fn dr_prediction_z2_WxH_neon(
    W: i32,
    H: i32,
    mut dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
    dx: i32,
    dy: i32,
) {
    // here upsample_above and upsample_left are 0 by design of
    // av1_use_intra_edge_upsample
    let min_base_x = -1;

    let left_m1 = vld1q_u8(left.offset(-1));
    let left_0 = vld1q_u8(left.add(0));
    let left_16 = vld1q_u8(left.add(16));
    let left_32 = vld1q_u8(left.add(32));
    let left_48 = vld1q_u8(left.add(48));
    let left_15 = vextq_u8::<15>(left_0, left_16);
    let left_31 = vextq_u8::<15>(left_16, left_32);
    let left_47 = vextq_u8::<15>(left_32, left_48);
    let left_vals0 = uint8x16x4_t(left_m1, left_15, left_31, left_47);
    let left_vals1 = uint8x16x4_t(left_0, left_16, left_32, left_48);

    for r in 0..H {
        let y = r + 1;
        let base_x = (-y * dx) >> 6;
        let mut j = 0;
        while j < W {
            let base_min_diff = min_base_x - base_x - j;

            if base_min_diff <= 0 {
                let resx = dr_prediction_z2_WxH_above_neon(above, dx, base_x, y, j);
                vst1q_u8(dst.offset(j as isize), resx);
            } else if base_min_diff < 16 {
                let resx = dr_prediction_z2_WxH_above_neon(above, dx, base_x, y, j);
                let resy = dr_prediction_z2_WxH_left_neon(left_vals0, left_vals1, dy, r, j);
                let mask = vld1q_u8(base_mask(base_min_diff));
                let resxy = vbslq_u8(mask, resy, resx);
                vst1q_u8(dst.offset(j as isize), resxy);
            } else {
                let resy = dr_prediction_z2_WxH_left_neon(left_vals0, left_vals1, dy, r, j);
                vst1q_u8(dst.offset(j as isize), resy);
            }
            j += 16;
        } // for j
        dst = dst.offset(stride);
    }
}

// Directional prediction, zone 2: 90 < angle < 180
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_dr_prediction_z2_neon(
    dst: *mut u8,
    stride: isize,
    bw: i32,
    bh: i32,
    above: *const u8,
    left: *const u8,
    upsample_above: i32,
    upsample_left: i32,
    dx: i32,
    dy: i32,
) {
    debug_assert!(dx > 0);
    debug_assert!(dy > 0);

    match bw {
        4 => dr_prediction_z2_4xH_neon(bh, dst, stride, above, left, upsample_above, upsample_left, dx, dy),
        8 => dr_prediction_z2_8xH_neon(bh, dst, stride, above, left, upsample_above, upsample_left, dx, dy),
        _ => dr_prediction_z2_WxH_neon(bw, bh, dst, stride, above, left, dx, dy),
    }
}

/* ---------------------P R E D I C T I O N   Z 3--------------------------- */

// Transcribed from upstream/aom/aom_dsp/arm/reinterpret_neon.h
// (REINTERPRET_NEON(u, 8, 8, 16, 4, 2, ) and REINTERPRET_NEON(u, 8, 16, 16, 8,
// 2, q)).
#[inline(always)]
unsafe fn aom_reinterpret_u8_u16_x2(src: uint16x4x2_t) -> uint8x8x2_t {
    uint8x8x2_t(vreinterpret_u8_u16(src.0), vreinterpret_u8_u16(src.1))
}

#[inline(always)]
unsafe fn aom_reinterpretq_u8_u16_x2(src: uint16x8x2_t) -> uint8x16x2_t {
    uint8x16x2_t(vreinterpretq_u8_u16(src.0), vreinterpretq_u8_u16(src.1))
}

// Transcribed from upstream/aom/aom_ports/bitops.h (GNU builtin branch).
#[inline(always)]
fn get_msb(n: u32) -> i32 {
    debug_assert!(n != 0);
    (31 ^ n.leading_zeros()) as i32
}

#[inline(always)]
unsafe fn z3_transpose_arrays_u8_16x4(x: *const uint8x16_t, d: *mut uint8x16x2_t) {
    let w0 = vzipq_u8(*x.add(0), *x.add(1));
    let w1 = vzipq_u8(*x.add(2), *x.add(3));

    *d.add(0) =
        aom_reinterpretq_u8_u16_x2(vzipq_u16(vreinterpretq_u16_u8(w0.0), vreinterpretq_u16_u8(w1.0)));
    *d.add(1) =
        aom_reinterpretq_u8_u16_x2(vzipq_u16(vreinterpretq_u16_u8(w0.1), vreinterpretq_u16_u8(w1.1)));
}

#[inline(always)]
unsafe fn z3_transpose_arrays_u8_4x4(x: *const uint8x8_t, d: *mut uint8x8x2_t) {
    let w0 = vzip_u8(*x.add(0), *x.add(1));
    let w1 = vzip_u8(*x.add(2), *x.add(3));

    *d = aom_reinterpret_u8_u16_x2(vzip_u16(vreinterpret_u16_u8(w0.0), vreinterpret_u16_u8(w1.0)));
}

#[inline(always)]
unsafe fn z3_transpose_arrays_u8_8x4(x: *const uint8x8_t, d: *mut uint8x8x2_t) {
    let w0 = vzip_u8(*x.add(0), *x.add(1));
    let w1 = vzip_u8(*x.add(2), *x.add(3));

    *d.add(0) = aom_reinterpret_u8_u16_x2(vzip_u16(vreinterpret_u16_u8(w0.0), vreinterpret_u16_u8(w1.0)));
    *d.add(1) = aom_reinterpret_u8_u16_x2(vzip_u16(vreinterpret_u16_u8(w0.1), vreinterpret_u16_u8(w1.1)));
}

unsafe fn z3_transpose_arrays_u8_16x16(src: *const u8, pitchSrc: isize, dst: *mut u8, pitchDst: isize) {
    // The same as the normal transposes in transpose_neon.h, but with a stride
    // between consecutive vectors of elements.
    let mut r: [uint8x16_t; 16] = core::mem::zeroed();
    let mut d: [uint8x16_t; 16] = core::mem::zeroed();
    for i in 0..16 {
        r[i] = vld1q_u8(src.offset(i as isize * pitchSrc));
    }
    transpose_arrays_u8_16x16(r.as_ptr(), d.as_mut_ptr());
    for i in 0..16 {
        vst1q_u8(dst.offset(i as isize * pitchDst), d[i]);
    }
}

unsafe fn z3_transpose_arrays_u8_16nx16n(
    src: *const u8,
    pitchSrc: isize,
    dst: *mut u8,
    pitchDst: isize,
    width: i32,
    height: i32,
) {
    let mut j = 0;
    while j < height {
        let mut i = 0;
        while i < width {
            z3_transpose_arrays_u8_16x16(
                src.offset(i as isize * pitchSrc + j as isize),
                pitchSrc,
                dst.offset(j as isize * pitchDst + i as isize),
                pitchDst,
            );
            i += 16;
        }
        j += 16;
    }
}

unsafe extern "C-unwind" fn dr_prediction_z3_4x4_neon(
    dst: *mut u8,
    stride: isize,
    left: *const u8,
    upsample_left: i32,
    dy: i32,
) {
    let mut dstvec: [uint8x8_t; 4] = core::mem::zeroed();
    let mut dest: uint8x8x2_t = core::mem::zeroed();

    dr_prediction_z1_WxH_internal_neon_small(4, 4, dstvec.as_mut_ptr(), left, upsample_left, dy);
    z3_transpose_arrays_u8_4x4(dstvec.as_ptr(), &raw mut dest);
    store_u8x4_strided_x2(dst.offset(stride * 0), stride, dest.0);
    store_u8x4_strided_x2(dst.offset(stride * 2), stride, dest.1);
}

unsafe extern "C-unwind" fn dr_prediction_z3_8x8_neon(
    dst: *mut u8,
    stride: isize,
    left: *const u8,
    upsample_left: i32,
    dy: i32,
) {
    let mut dstvec: [uint8x8_t; 8] = core::mem::zeroed();
    let mut d: [uint8x8_t; 8] = core::mem::zeroed();

    dr_prediction_z1_WxH_internal_neon_small(8, 8, dstvec.as_mut_ptr(), left, upsample_left, dy);
    transpose_arrays_u8_8x8(dstvec.as_ptr(), d.as_mut_ptr());
    store_u8_8x8(dst, stride, d[0], d[1], d[2], d[3], d[4], d[5], d[6], d[7]);
}

unsafe extern "C-unwind" fn dr_prediction_z3_4x8_neon(
    dst: *mut u8,
    stride: isize,
    left: *const u8,
    upsample_left: i32,
    dy: i32,
) {
    let mut dstvec: [uint8x8_t; 4] = core::mem::zeroed();
    let mut d: [uint8x8x2_t; 2] = core::mem::zeroed();

    dr_prediction_z1_WxH_internal_neon_small(8, 4, dstvec.as_mut_ptr(), left, upsample_left, dy);
    z3_transpose_arrays_u8_8x4(dstvec.as_ptr(), d.as_mut_ptr());
    store_u8x4_strided_x2(dst.offset(stride * 0), stride, d[0].0);
    store_u8x4_strided_x2(dst.offset(stride * 2), stride, d[0].1);
    store_u8x4_strided_x2(dst.offset(stride * 4), stride, d[1].0);
    store_u8x4_strided_x2(dst.offset(stride * 6), stride, d[1].1);
}

unsafe extern "C-unwind" fn dr_prediction_z3_8x4_neon(
    dst: *mut u8,
    stride: isize,
    left: *const u8,
    upsample_left: i32,
    dy: i32,
) {
    let mut dstvec: [uint8x8_t; 8] = core::mem::zeroed();
    let mut d: [uint8x8_t; 8] = core::mem::zeroed();

    dr_prediction_z1_WxH_internal_neon_small(4, 8, dstvec.as_mut_ptr(), left, upsample_left, dy);
    transpose_arrays_u8_8x8(dstvec.as_ptr(), d.as_mut_ptr());
    store_u8_8x4(dst, stride, d[0], d[1], d[2], d[3]);
}

unsafe extern "C-unwind" fn dr_prediction_z3_8x16_neon(
    dst: *mut u8,
    stride: isize,
    left: *const u8,
    upsample_left: i32,
    dy: i32,
) {
    let mut dstvec: [uint8x16_t; 8] = core::mem::zeroed();
    let mut d: [uint8x8_t; 16] = core::mem::zeroed();

    dr_prediction_z1_WxH_internal_neon_large(16, 8, dstvec.as_mut_ptr(), left, dy);
    transpose_arrays_u8_16x8(dstvec.as_ptr(), d.as_mut_ptr());
    for i in 0..16 {
        vst1_u8(dst.offset(i as isize * stride), d[i]);
    }
}

unsafe extern "C-unwind" fn dr_prediction_z3_16x8_neon(
    dst: *mut u8,
    stride: isize,
    left: *const u8,
    upsample_left: i32,
    dy: i32,
) {
    let mut dstvec: [uint8x8_t; 16] = core::mem::zeroed();
    let mut d: [uint8x16_t; 8] = core::mem::zeroed();

    dr_prediction_z1_WxH_internal_neon_small(8, 16, dstvec.as_mut_ptr(), left, upsample_left, dy);
    transpose_arrays_u8_8x16(dstvec.as_ptr(), d.as_mut_ptr());
    for i in 0..8 {
        vst1q_u8(dst.offset(i as isize * stride), d[i]);
    }
}

unsafe extern "C-unwind" fn dr_prediction_z3_4x16_neon(
    dst: *mut u8,
    stride: isize,
    left: *const u8,
    upsample_left: i32,
    dy: i32,
) {
    let mut dstvec: [uint8x16_t; 4] = core::mem::zeroed();
    let mut d: [uint8x16x2_t; 2] = core::mem::zeroed();

    dr_prediction_z1_WxH_internal_neon_large(16, 4, dstvec.as_mut_ptr(), left, dy);
    z3_transpose_arrays_u8_16x4(dstvec.as_ptr(), d.as_mut_ptr());
    store_u8x4_strided_x4(dst.offset(stride * 0), stride, d[0].0);
    store_u8x4_strided_x4(dst.offset(stride * 4), stride, d[0].1);
    store_u8x4_strided_x4(dst.offset(stride * 8), stride, d[1].0);
    store_u8x4_strided_x4(dst.offset(stride * 12), stride, d[1].1);
}

unsafe extern "C-unwind" fn dr_prediction_z3_16x4_neon(
    dst: *mut u8,
    stride: isize,
    left: *const u8,
    upsample_left: i32,
    dy: i32,
) {
    let mut dstvec: [uint8x8_t; 16] = core::mem::zeroed();
    let mut d: [uint8x16_t; 8] = core::mem::zeroed();

    dr_prediction_z1_WxH_internal_neon_small(4, 16, dstvec.as_mut_ptr(), left, upsample_left, dy);
    transpose_arrays_u8_8x16(dstvec.as_ptr(), d.as_mut_ptr());
    for i in 0..4 {
        vst1q_u8(dst.offset(i as isize * stride), d[i]);
    }
}

unsafe extern "C-unwind" fn dr_prediction_z3_8x32_neon(
    dst: *mut u8,
    stride: isize,
    left: *const u8,
    upsample_left: i32,
    dy: i32,
) {
    let mut dstvec: [uint8x16x2_t; 16] = core::mem::zeroed();
    let mut d: [uint8x16_t; 32] = core::mem::zeroed();
    let v_zero = vdupq_n_u8(0);

    dr_prediction_z1_32xH_internal_neon(8, dstvec.as_mut_ptr(), left, dy);
    for i in 8..16 {
        dstvec[i].0 = v_zero;
        dstvec[i].1 = v_zero;
    }
    transpose_arrays_u8_32x16(dstvec.as_ptr(), d.as_mut_ptr());
    for i in 0..32 {
        vst1_u8(dst.offset(i as isize * stride), vget_low_u8(d[i]));
    }
}

unsafe extern "C-unwind" fn dr_prediction_z3_32x8_neon(
    dst: *mut u8,
    stride: isize,
    left: *const u8,
    upsample_left: i32,
    dy: i32,
) {
    let mut dstvec: [uint8x8_t; 32] = core::mem::zeroed();
    let mut d: [uint8x16_t; 16] = core::mem::zeroed();

    dr_prediction_z1_WxH_internal_neon_small(8, 32, dstvec.as_mut_ptr(), left, upsample_left, dy);
    transpose_arrays_u8_8x16(dstvec.as_ptr(), d.as_mut_ptr());
    transpose_arrays_u8_8x16(dstvec.as_ptr().add(16), d.as_mut_ptr().add(8));
    for i in 0..8 {
        vst1q_u8(dst.offset(i as isize * stride), d[i]);
        vst1q_u8(dst.offset(i as isize * stride + 16), d[i + 8]);
    }
}

unsafe extern "C-unwind" fn dr_prediction_z3_16x16_neon(
    dst: *mut u8,
    stride: isize,
    left: *const u8,
    upsample_left: i32,
    dy: i32,
) {
    let mut dstvec: [uint8x16_t; 16] = core::mem::zeroed();
    let mut d: [uint8x16_t; 16] = core::mem::zeroed();

    dr_prediction_z1_WxH_internal_neon_large(16, 16, dstvec.as_mut_ptr(), left, dy);
    transpose_arrays_u8_16x16(dstvec.as_ptr(), d.as_mut_ptr());
    for i in 0..16 {
        vst1q_u8(dst.offset(i as isize * stride), d[i]);
    }
}

unsafe extern "C-unwind" fn dr_prediction_z3_32x32_neon(
    dst: *mut u8,
    stride: isize,
    left: *const u8,
    upsample_left: i32,
    dy: i32,
) {
    let mut dstvec: [uint8x16x2_t; 32] = core::mem::zeroed();
    let mut d: [uint8x16_t; 64] = core::mem::zeroed();

    dr_prediction_z1_32xH_internal_neon(32, dstvec.as_mut_ptr(), left, dy);
    transpose_arrays_u8_32x16(dstvec.as_ptr(), d.as_mut_ptr());
    transpose_arrays_u8_32x16(dstvec.as_ptr().add(16), d.as_mut_ptr().add(32));
    for i in 0..32 {
        vst1q_u8(dst.offset(i as isize * stride), d[i]);
        vst1q_u8(dst.offset(i as isize * stride + 16), d[i + 32]);
    }
}

unsafe extern "C-unwind" fn dr_prediction_z3_64x64_neon(
    dst: *mut u8,
    stride: isize,
    left: *const u8,
    upsample_left: i32,
    dy: i32,
) {
    let mut dstT = Align16([0u8; 64 * 64]);

    dr_prediction_z1_64xH_neon(64, dstT.0.as_mut_ptr(), 64, left, dy);
    z3_transpose_arrays_u8_16nx16n(dstT.0.as_ptr(), 64, dst, stride, 64, 64);
}

unsafe extern "C-unwind" fn dr_prediction_z3_16x32_neon(
    dst: *mut u8,
    stride: isize,
    left: *const u8,
    upsample_left: i32,
    dy: i32,
) {
    let mut dstvec: [uint8x16x2_t; 16] = core::mem::zeroed();
    let mut d: [uint8x16_t; 32] = core::mem::zeroed();

    dr_prediction_z1_32xH_internal_neon(16, dstvec.as_mut_ptr(), left, dy);
    transpose_arrays_u8_32x16(dstvec.as_ptr(), d.as_mut_ptr());
    for i in 0..16 {
        vst1q_u8(dst.offset(2 * i as isize * stride), d[2 * i + 0]);
        vst1q_u8(dst.offset((2 * i + 1) as isize * stride), d[2 * i + 1]);
    }
}

unsafe extern "C-unwind" fn dr_prediction_z3_32x16_neon(
    dst: *mut u8,
    stride: isize,
    left: *const u8,
    upsample_left: i32,
    dy: i32,
) {
    let mut dstvec: [uint8x16_t; 32] = core::mem::zeroed();

    dr_prediction_z1_WxH_internal_neon_large(16, 32, dstvec.as_mut_ptr(), left, dy);
    let mut i = 0;
    while i < 32 {
        let mut d: [uint8x16_t; 16] = core::mem::zeroed();
        transpose_arrays_u8_16x16(dstvec.as_ptr().add(i), d.as_mut_ptr());
        for j in 0..16 {
            vst1q_u8(dst.offset(j as isize * stride + i as isize), d[j]);
        }
        i += 16;
    }
}

unsafe extern "C-unwind" fn dr_prediction_z3_32x64_neon(
    dst: *mut u8,
    stride: isize,
    left: *const u8,
    upsample_left: i32,
    dy: i32,
) {
    let mut dstT = [0u8; 64 * 32];

    dr_prediction_z1_64xH_neon(32, dstT.as_mut_ptr(), 64, left, dy);
    z3_transpose_arrays_u8_16nx16n(dstT.as_ptr(), 64, dst, stride, 32, 64);
}

unsafe extern "C-unwind" fn dr_prediction_z3_64x32_neon(
    dst: *mut u8,
    stride: isize,
    left: *const u8,
    upsample_left: i32,
    dy: i32,
) {
    let mut dstT = [0u8; 32 * 64];

    dr_prediction_z1_32xH_neon(64, dstT.as_mut_ptr(), 32, left, dy);
    z3_transpose_arrays_u8_16nx16n(dstT.as_ptr(), 32, dst, stride, 64, 32);
}

unsafe extern "C-unwind" fn dr_prediction_z3_16x64_neon(
    dst: *mut u8,
    stride: isize,
    left: *const u8,
    upsample_left: i32,
    dy: i32,
) {
    let mut dstT = [0u8; 64 * 16];

    dr_prediction_z1_64xH_neon(16, dstT.as_mut_ptr(), 64, left, dy);
    z3_transpose_arrays_u8_16nx16n(dstT.as_ptr(), 64, dst, stride, 16, 64);
}

unsafe extern "C-unwind" fn dr_prediction_z3_64x16_neon(
    dst: *mut u8,
    stride: isize,
    left: *const u8,
    upsample_left: i32,
    dy: i32,
) {
    let mut dstvec: [uint8x16_t; 64] = core::mem::zeroed();

    dr_prediction_z1_WxH_internal_neon_large(16, 64, dstvec.as_mut_ptr(), left, dy);
    let mut i = 0;
    while i < 64 {
        let mut d: [uint8x16_t; 16] = core::mem::zeroed();
        transpose_arrays_u8_16x16(dstvec.as_ptr().add(i), d.as_mut_ptr());
        for j in 0..16 {
            vst1q_u8(dst.offset(j as isize * stride + i as isize), d[j]);
        }
        i += 16;
    }
}

type dr_prediction_z3_fn = Option<unsafe extern "C-unwind" fn(*mut u8, isize, *const u8, i32, i32)>;

static dr_prediction_z3_arr: [[dr_prediction_z3_fn; 7]; 7] = [
    [None, None, None, None, None, None, None],
    [None, None, None, None, None, None, None],
    [
        None,
        None,
        Some(dr_prediction_z3_4x4_neon),
        Some(dr_prediction_z3_4x8_neon),
        Some(dr_prediction_z3_4x16_neon),
        None,
        None,
    ],
    [
        None,
        None,
        Some(dr_prediction_z3_8x4_neon),
        Some(dr_prediction_z3_8x8_neon),
        Some(dr_prediction_z3_8x16_neon),
        Some(dr_prediction_z3_8x32_neon),
        None,
    ],
    [
        None,
        None,
        Some(dr_prediction_z3_16x4_neon),
        Some(dr_prediction_z3_16x8_neon),
        Some(dr_prediction_z3_16x16_neon),
        Some(dr_prediction_z3_16x32_neon),
        Some(dr_prediction_z3_16x64_neon),
    ],
    [
        None,
        None,
        None,
        Some(dr_prediction_z3_32x8_neon),
        Some(dr_prediction_z3_32x16_neon),
        Some(dr_prediction_z3_32x32_neon),
        Some(dr_prediction_z3_32x64_neon),
    ],
    [
        None,
        None,
        None,
        None,
        Some(dr_prediction_z3_64x16_neon),
        Some(dr_prediction_z3_64x32_neon),
        Some(dr_prediction_z3_64x64_neon),
    ],
];

#[no_mangle]
pub unsafe extern "C-unwind" fn av1_dr_prediction_z3_neon(
    dst: *mut u8,
    stride: isize,
    bw: i32,
    bh: i32,
    above: *const u8,
    left: *const u8,
    upsample_left: i32,
    dx: i32,
    dy: i32,
) {
    debug_assert!(dx == 1);
    debug_assert!(dy > 0);

    let f = dr_prediction_z3_arr[get_msb(bw as u32) as usize][get_msb(bh as u32) as usize];
    debug_assert!(f.is_some());
    f.unwrap()(dst, stride, left, upsample_left, dy);
}

// -----------------------------------------------------------------------------
