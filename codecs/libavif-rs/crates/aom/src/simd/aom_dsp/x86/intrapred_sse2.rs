/*
 * Copyright (c) 2017, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

// Translated from upstream/aom/aom_dsp/x86/intrapred_sse2.c (ADR 0056, ADR 0058).
// Upstream compiles it with -msse2. The functions were
// transliterated with a typing-aware C-subset printer (not C2Rust) and
// reviewed against the C function by function: the same sequence of intrinsic
// and helper calls, constants and operators in order (ADR 0058).
//
// Translation notes:
// - Every function carries `sse2`, the flag upstream compiles the file with.
//   The RTCD specializations are exported under their upstream names.
// - Pointer and array parameters stay raw pointers; local arrays are Rust
//   arrays and start zeroed. DECLARE_ALIGNED arrays keep their alignment
//   through `Aligned16`/`Aligned32`.
// - C integer conversions are explicit (`as`); `assert` becomes
//   `debug_assert!`. Immediate operands become const generics; shift counts
//   that are run-time `int` values use the count-register intrinsics, which is
//   what GCC and Clang emit for a non-constant count.
// - Scalar accesses through cast pointers (`*(int *)p`) are unaligned reads
//   and writes, as x86 compilers emit them.
#![allow(
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut,
    unused_parens
)]
#![allow(unused_variables, dead_code, clippy::all)]

use crate::simd::aom_dsp__x86__intrapred_utils::*;
use crate::simd::aom_dsp__x86__intrapred_x86::*;
use crate::simd::aom_dsp__x86__lpf_common_sse2::*;
use crate::simd::aom_dsp__x86__synonyms::*;
use core::arch::x86_64::*;

const SMOOTH_WEIGHT_LOG2_SCALE: i32 = 8;

const DC_MULTIPLIER_1X2: i32 = 0x5556;
const DC_MULTIPLIER_1X4: i32 = 0x3334;
const DC_SHIFT2: i32 = 16;

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn dc_store_4xh(dc: u32, height: i32, mut dst: *mut u8, stride: isize) {
    for i in (0..height).step_by(2) {
        core::ptr::write_unaligned(dst as *mut u32, dc);
        dst = dst.offset(stride);
        core::ptr::write_unaligned(dst as *mut u32, dc);
        dst = dst.offset(stride);
    }
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn dc_store_8xh(row: *const __m128i, height: i32, mut dst: *mut u8, stride: isize) {
    let mut i: i32 = 0;
    {
        i = 0;
        while i < height {
            _mm_storel_epi64(dst as *mut __m128i, *row);
            dst = dst.offset(stride);
            i += 1;
        }
    }
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn dc_store_16xh(row: *const __m128i, height: i32, mut dst: *mut u8, stride: isize) {
    let mut i: i32 = 0;
    {
        i = 0;
        while i < height {
            _mm_store_si128(dst as *mut __m128i, *row);
            dst = dst.offset(stride);
            i += 1;
        }
    }
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn dc_store_32xh(row: *const __m128i, height: i32, mut dst: *mut u8, stride: isize) {
    let mut i: i32 = 0;
    {
        i = 0;
        while i < height {
            _mm_store_si128(dst as *mut __m128i, *row);
            _mm_store_si128(dst.offset(16) as *mut __m128i, *row);
            dst = dst.offset(stride);
            i += 1;
        }
    }
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn dc_store_64xh(row: *const __m128i, height: i32, mut dst: *mut u8, stride: isize) {
    for i in 0..height {
        _mm_store_si128(dst as *mut __m128i, *row);
        _mm_store_si128(dst.offset(16) as *mut __m128i, *row);
        _mm_store_si128(dst.offset(32) as *mut __m128i, *row);
        _mm_store_si128(dst.offset(48) as *mut __m128i, *row);
        dst = dst.offset(stride);
    }
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn dc_sum_4(ref_: *const u8) -> __m128i {
    let mut x: __m128i = _mm_loadl_epi64(ref_ as *const __m128i);
    let zero: __m128i = _mm_setzero_si128();
    x = _mm_unpacklo_epi8(x, zero);
    return _mm_sad_epu8(x, zero);
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn dc_sum_8(ref_: *const u8) -> __m128i {
    let mut x: __m128i = _mm_loadl_epi64(ref_ as *const __m128i);
    let zero: __m128i = _mm_setzero_si128();
    return _mm_sad_epu8(x, zero);
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn dc_sum_64(ref_: *const u8) -> __m128i {
    let mut x0: __m128i = _mm_load_si128(ref_ as *const __m128i);
    let mut x1: __m128i = _mm_load_si128(ref_.offset(16) as *const __m128i);
    let mut x2: __m128i = _mm_load_si128(ref_.offset(32) as *const __m128i);
    let mut x3: __m128i = _mm_load_si128(ref_.offset(48) as *const __m128i);
    let zero: __m128i = _mm_setzero_si128();
    x0 = _mm_sad_epu8(x0, zero);
    x1 = _mm_sad_epu8(x1, zero);
    x2 = _mm_sad_epu8(x2, zero);
    x3 = _mm_sad_epu8(x3, zero);
    x0 = _mm_add_epi16(x0, x1);
    x2 = _mm_add_epi16(x2, x3);
    x0 = _mm_add_epi16(x0, x2);
    let high: __m128i = _mm_unpackhi_epi64(x0, x0);
    return _mm_add_epi16(x0, high);
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn divide_using_multiply_shift(num: i32, shift1: i32, multiplier: i32) -> i32 {
    let interm: i32 = num >> shift1;
    return (interm * multiplier) >> DC_SHIFT2;
}

// -----------------------------------------------------------------------------
// DC_PRED
#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_dc_predictor_4x8_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let sum_left: __m128i = dc_sum_8(left);
    let mut sum_above: __m128i = dc_sum_4(above);
    sum_above = _mm_add_epi16(sum_left, sum_above);
    let mut sum: u32 = _mm_cvtsi128_si32(sum_above) as u32;
    sum += 6;
    sum = divide_using_multiply_shift(sum as i32, 2, DC_MULTIPLIER_1X2) as u32;
    let row: __m128i = _mm_set1_epi8(sum as i8);
    let pred: u32 = _mm_cvtsi128_si32(row) as u32;
    dc_store_4xh(pred, 8, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_dc_predictor_4x16_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let sum_left: __m128i = dc_sum_16_sse2(left);
    let mut sum_above: __m128i = dc_sum_4(above);
    sum_above = _mm_add_epi16(sum_left, sum_above);
    let mut sum: u32 = _mm_cvtsi128_si32(sum_above) as u32;
    sum += 10;
    sum = divide_using_multiply_shift(sum as i32, 2, DC_MULTIPLIER_1X4) as u32;
    let row: __m128i = _mm_set1_epi8(sum as i8);
    let pred: u32 = _mm_cvtsi128_si32(row) as u32;
    dc_store_4xh(pred, 16, dst, stride);
}

// !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER
#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_dc_predictor_8x4_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let sum_left: __m128i = dc_sum_4(left);
    let mut sum_above: __m128i = dc_sum_8(above);
    sum_above = _mm_add_epi16(sum_above, sum_left);
    let mut sum: u32 = _mm_cvtsi128_si32(sum_above) as u32;
    sum += 6;
    sum = divide_using_multiply_shift(sum as i32, 2, DC_MULTIPLIER_1X2) as u32;
    let row: __m128i = _mm_set1_epi8(sum as i8);
    dc_store_8xh(&raw const row, 4, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_dc_predictor_8x16_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let sum_left: __m128i = dc_sum_16_sse2(left);
    let mut sum_above: __m128i = dc_sum_8(above);
    sum_above = _mm_add_epi16(sum_above, sum_left);
    let mut sum: u32 = _mm_cvtsi128_si32(sum_above) as u32;
    sum += 12;
    sum = divide_using_multiply_shift(sum as i32, 3, DC_MULTIPLIER_1X2) as u32;
    let row: __m128i = _mm_set1_epi8(sum as i8);
    dc_store_8xh(&raw const row, 16, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_dc_predictor_8x32_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let sum_left: __m128i = dc_sum_32_sse2(left);
    let mut sum_above: __m128i = dc_sum_8(above);
    sum_above = _mm_add_epi16(sum_above, sum_left);
    let mut sum: u32 = _mm_cvtsi128_si32(sum_above) as u32;
    sum += 20;
    sum = divide_using_multiply_shift(sum as i32, 3, DC_MULTIPLIER_1X4) as u32;
    let row: __m128i = _mm_set1_epi8(sum as i8);
    dc_store_8xh(&raw const row, 32, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_dc_predictor_16x4_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let sum_left: __m128i = dc_sum_4(left);
    let mut sum_above: __m128i = dc_sum_16_sse2(above);
    sum_above = _mm_add_epi16(sum_above, sum_left);
    let mut sum: u32 = _mm_cvtsi128_si32(sum_above) as u32;
    sum += 10;
    sum = divide_using_multiply_shift(sum as i32, 2, DC_MULTIPLIER_1X4) as u32;
    let row: __m128i = _mm_set1_epi8(sum as i8);
    dc_store_16xh(&raw const row, 4, dst, stride);
}

// !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER
#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_dc_predictor_16x8_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let sum_left: __m128i = dc_sum_8(left);
    let mut sum_above: __m128i = dc_sum_16_sse2(above);
    sum_above = _mm_add_epi16(sum_above, sum_left);
    let mut sum: u32 = _mm_cvtsi128_si32(sum_above) as u32;
    sum += 12;
    sum = divide_using_multiply_shift(sum as i32, 3, DC_MULTIPLIER_1X2) as u32;
    let row: __m128i = _mm_set1_epi8(sum as i8);
    dc_store_16xh(&raw const row, 8, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_dc_predictor_16x32_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let sum_left: __m128i = dc_sum_32_sse2(left);
    let mut sum_above: __m128i = dc_sum_16_sse2(above);
    sum_above = _mm_add_epi16(sum_left, sum_above);
    let mut sum: u32 = _mm_cvtsi128_si32(sum_above) as u32;
    sum += 24;
    sum = divide_using_multiply_shift(sum as i32, 4, DC_MULTIPLIER_1X2) as u32;
    let row: __m128i = _mm_set1_epi8(sum as i8);
    dc_store_16xh(&raw const row, 32, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_dc_predictor_16x64_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let sum_left: __m128i = dc_sum_64(left);
    let mut sum_above: __m128i = dc_sum_16_sse2(above);
    sum_above = _mm_add_epi16(sum_left, sum_above);
    let mut sum: u32 = _mm_cvtsi128_si32(sum_above) as u32;
    sum += 40;
    sum = divide_using_multiply_shift(sum as i32, 4, DC_MULTIPLIER_1X4) as u32;
    let row: __m128i = _mm_set1_epi8(sum as i8);
    dc_store_16xh(&raw const row, 64, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_dc_predictor_32x8_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let mut sum_above: __m128i = dc_sum_32_sse2(above);
    let sum_left: __m128i = dc_sum_8(left);
    sum_above = _mm_add_epi16(sum_above, sum_left);
    let mut sum: u32 = _mm_cvtsi128_si32(sum_above) as u32;
    sum += 20;
    sum = divide_using_multiply_shift(sum as i32, 3, DC_MULTIPLIER_1X4) as u32;
    let row: __m128i = _mm_set1_epi8(sum as i8);
    dc_store_32xh(&raw const row, 8, dst, stride);
}

// !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER
#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_dc_predictor_32x16_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let mut sum_above: __m128i = dc_sum_32_sse2(above);
    let sum_left: __m128i = dc_sum_16_sse2(left);
    sum_above = _mm_add_epi16(sum_above, sum_left);
    let mut sum: u32 = _mm_cvtsi128_si32(sum_above) as u32;
    sum += 24;
    sum = divide_using_multiply_shift(sum as i32, 4, DC_MULTIPLIER_1X2) as u32;
    let row: __m128i = _mm_set1_epi8(sum as i8);
    dc_store_32xh(&raw const row, 16, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_dc_predictor_32x64_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let mut sum_above: __m128i = dc_sum_32_sse2(above);
    let sum_left: __m128i = dc_sum_64(left);
    sum_above = _mm_add_epi16(sum_above, sum_left);
    let mut sum: u32 = _mm_cvtsi128_si32(sum_above) as u32;
    sum += 48;
    sum = divide_using_multiply_shift(sum as i32, 5, DC_MULTIPLIER_1X2) as u32;
    let row: __m128i = _mm_set1_epi8(sum as i8);
    dc_store_32xh(&raw const row, 64, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_dc_predictor_64x64_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let mut sum_above: __m128i = dc_sum_64(above);
    let sum_left: __m128i = dc_sum_64(left);
    sum_above = _mm_add_epi16(sum_above, sum_left);
    let mut sum: u32 = _mm_cvtsi128_si32(sum_above) as u32;
    sum += 64;
    sum /= 128;
    let row: __m128i = _mm_set1_epi8(sum as i8);
    dc_store_64xh(&raw const row, 64, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_dc_predictor_64x32_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let mut sum_above: __m128i = dc_sum_64(above);
    let sum_left: __m128i = dc_sum_32_sse2(left);
    sum_above = _mm_add_epi16(sum_above, sum_left);
    let mut sum: u32 = _mm_cvtsi128_si32(sum_above) as u32;
    sum += 48;
    sum = divide_using_multiply_shift(sum as i32, 5, DC_MULTIPLIER_1X2) as u32;
    let row: __m128i = _mm_set1_epi8(sum as i8);
    dc_store_64xh(&raw const row, 32, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_dc_predictor_64x16_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let mut sum_above: __m128i = dc_sum_64(above);
    let sum_left: __m128i = dc_sum_16_sse2(left);
    sum_above = _mm_add_epi16(sum_above, sum_left);
    let mut sum: u32 = _mm_cvtsi128_si32(sum_above) as u32;
    sum += 40;
    sum = divide_using_multiply_shift(sum as i32, 4, DC_MULTIPLIER_1X4) as u32;
    let row: __m128i = _mm_set1_epi8(sum as i8);
    dc_store_64xh(&raw const row, 16, dst, stride);
}

// !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER
// -----------------------------------------------------------------------------
// DC_TOP
#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_dc_top_predictor_4x8_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let _ = left;
    let mut sum_above: __m128i = dc_sum_4(above);
    let two: __m128i = _mm_set1_epi16(2);
    sum_above = _mm_add_epi16(sum_above, two);
    sum_above = _mm_srai_epi16::<{ 2 }>(sum_above);
    sum_above = _mm_shufflelo_epi16::<0>(sum_above);
    sum_above = _mm_packus_epi16(sum_above, sum_above);
    let pred: u32 = _mm_cvtsi128_si32(sum_above) as u32;
    dc_store_4xh(pred, 8, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_dc_top_predictor_4x16_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let _ = left;
    let mut sum_above: __m128i = dc_sum_4(above);
    let two: __m128i = _mm_set1_epi16(2);
    sum_above = _mm_add_epi16(sum_above, two);
    sum_above = _mm_srai_epi16::<{ 2 }>(sum_above);
    sum_above = _mm_shufflelo_epi16::<0>(sum_above);
    sum_above = _mm_packus_epi16(sum_above, sum_above);
    let pred: u32 = _mm_cvtsi128_si32(sum_above) as u32;
    dc_store_4xh(pred, 16, dst, stride);
}

// !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER
#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_dc_top_predictor_8x4_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let _ = left;
    let mut sum_above: __m128i = dc_sum_8(above);
    let four: __m128i = _mm_set1_epi16(4);
    sum_above = _mm_add_epi16(sum_above, four);
    sum_above = _mm_srai_epi16::<{ 3 }>(sum_above);
    sum_above = _mm_unpacklo_epi8(sum_above, sum_above);
    let row: __m128i = _mm_shufflelo_epi16::<0>(sum_above);
    dc_store_8xh(&raw const row, 4, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_dc_top_predictor_8x16_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let _ = left;
    let mut sum_above: __m128i = dc_sum_8(above);
    let four: __m128i = _mm_set1_epi16(4);
    sum_above = _mm_add_epi16(sum_above, four);
    sum_above = _mm_srai_epi16::<{ 3 }>(sum_above);
    sum_above = _mm_unpacklo_epi8(sum_above, sum_above);
    let row: __m128i = _mm_shufflelo_epi16::<0>(sum_above);
    dc_store_8xh(&raw const row, 16, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_dc_top_predictor_8x32_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let _ = left;
    let mut sum_above: __m128i = dc_sum_8(above);
    let four: __m128i = _mm_set1_epi16(4);
    sum_above = _mm_add_epi16(sum_above, four);
    sum_above = _mm_srai_epi16::<{ 3 }>(sum_above);
    sum_above = _mm_unpacklo_epi8(sum_above, sum_above);
    let row: __m128i = _mm_shufflelo_epi16::<0>(sum_above);
    dc_store_8xh(&raw const row, 32, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_dc_top_predictor_16x4_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let _ = left;
    let mut sum_above: __m128i = dc_sum_16_sse2(above);
    let eight: __m128i = _mm_set1_epi16(8);
    sum_above = _mm_add_epi16(sum_above, eight);
    sum_above = _mm_srai_epi16::<{ 4 }>(sum_above);
    sum_above = _mm_unpacklo_epi8(sum_above, sum_above);
    sum_above = _mm_shufflelo_epi16::<0>(sum_above);
    let row: __m128i = _mm_unpacklo_epi64(sum_above, sum_above);
    dc_store_16xh(&raw const row, 4, dst, stride);
}

// !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER
#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_dc_top_predictor_16x8_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let _ = left;
    let mut sum_above: __m128i = dc_sum_16_sse2(above);
    let eight: __m128i = _mm_set1_epi16(8);
    sum_above = _mm_add_epi16(sum_above, eight);
    sum_above = _mm_srai_epi16::<{ 4 }>(sum_above);
    sum_above = _mm_unpacklo_epi8(sum_above, sum_above);
    sum_above = _mm_shufflelo_epi16::<0>(sum_above);
    let row: __m128i = _mm_unpacklo_epi64(sum_above, sum_above);
    dc_store_16xh(&raw const row, 8, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_dc_top_predictor_16x32_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let _ = left;
    let mut sum_above: __m128i = dc_sum_16_sse2(above);
    let eight: __m128i = _mm_set1_epi16(8);
    sum_above = _mm_add_epi16(sum_above, eight);
    sum_above = _mm_srai_epi16::<{ 4 }>(sum_above);
    sum_above = _mm_unpacklo_epi8(sum_above, sum_above);
    sum_above = _mm_shufflelo_epi16::<0>(sum_above);
    let row: __m128i = _mm_unpacklo_epi64(sum_above, sum_above);
    dc_store_16xh(&raw const row, 32, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_dc_top_predictor_16x64_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let _ = left;
    let mut sum_above: __m128i = dc_sum_16_sse2(above);
    let eight: __m128i = _mm_set1_epi16(8);
    sum_above = _mm_add_epi16(sum_above, eight);
    sum_above = _mm_srai_epi16::<{ 4 }>(sum_above);
    sum_above = _mm_unpacklo_epi8(sum_above, sum_above);
    sum_above = _mm_shufflelo_epi16::<0>(sum_above);
    let row: __m128i = _mm_unpacklo_epi64(sum_above, sum_above);
    dc_store_16xh(&raw const row, 64, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_dc_top_predictor_32x8_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let _ = left;
    let mut sum_above: __m128i = dc_sum_32_sse2(above);
    let sixteen: __m128i = _mm_set1_epi16(16);
    sum_above = _mm_add_epi16(sum_above, sixteen);
    sum_above = _mm_srai_epi16::<{ 5 }>(sum_above);
    sum_above = _mm_unpacklo_epi8(sum_above, sum_above);
    sum_above = _mm_shufflelo_epi16::<0>(sum_above);
    let row: __m128i = _mm_unpacklo_epi64(sum_above, sum_above);
    dc_store_32xh(&raw const row, 8, dst, stride);
}

// !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER
#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_dc_top_predictor_32x16_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let _ = left;
    let mut sum_above: __m128i = dc_sum_32_sse2(above);
    let sixteen: __m128i = _mm_set1_epi16(16);
    sum_above = _mm_add_epi16(sum_above, sixteen);
    sum_above = _mm_srai_epi16::<{ 5 }>(sum_above);
    sum_above = _mm_unpacklo_epi8(sum_above, sum_above);
    sum_above = _mm_shufflelo_epi16::<0>(sum_above);
    let row: __m128i = _mm_unpacklo_epi64(sum_above, sum_above);
    dc_store_32xh(&raw const row, 16, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_dc_top_predictor_32x64_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let _ = left;
    let mut sum_above: __m128i = dc_sum_32_sse2(above);
    let sixteen: __m128i = _mm_set1_epi16(16);
    sum_above = _mm_add_epi16(sum_above, sixteen);
    sum_above = _mm_srai_epi16::<{ 5 }>(sum_above);
    sum_above = _mm_unpacklo_epi8(sum_above, sum_above);
    sum_above = _mm_shufflelo_epi16::<0>(sum_above);
    let row: __m128i = _mm_unpacklo_epi64(sum_above, sum_above);
    dc_store_32xh(&raw const row, 64, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_dc_top_predictor_64x64_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let _ = left;
    let mut sum_above: __m128i = dc_sum_64(above);
    let thirtytwo: __m128i = _mm_set1_epi16(32);
    sum_above = _mm_add_epi16(sum_above, thirtytwo);
    sum_above = _mm_srai_epi16::<{ 6 }>(sum_above);
    sum_above = _mm_unpacklo_epi8(sum_above, sum_above);
    sum_above = _mm_shufflelo_epi16::<0>(sum_above);
    let row: __m128i = _mm_unpacklo_epi64(sum_above, sum_above);
    dc_store_64xh(&raw const row, 64, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_dc_top_predictor_64x32_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let _ = left;
    let mut sum_above: __m128i = dc_sum_64(above);
    let thirtytwo: __m128i = _mm_set1_epi16(32);
    sum_above = _mm_add_epi16(sum_above, thirtytwo);
    sum_above = _mm_srai_epi16::<{ 6 }>(sum_above);
    sum_above = _mm_unpacklo_epi8(sum_above, sum_above);
    sum_above = _mm_shufflelo_epi16::<0>(sum_above);
    let row: __m128i = _mm_unpacklo_epi64(sum_above, sum_above);
    dc_store_64xh(&raw const row, 32, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_dc_top_predictor_64x16_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let _ = left;
    let mut sum_above: __m128i = dc_sum_64(above);
    let thirtytwo: __m128i = _mm_set1_epi16(32);
    sum_above = _mm_add_epi16(sum_above, thirtytwo);
    sum_above = _mm_srai_epi16::<{ 6 }>(sum_above);
    sum_above = _mm_unpacklo_epi8(sum_above, sum_above);
    sum_above = _mm_shufflelo_epi16::<0>(sum_above);
    let row: __m128i = _mm_unpacklo_epi64(sum_above, sum_above);
    dc_store_64xh(&raw const row, 16, dst, stride);
}

// !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER
// -----------------------------------------------------------------------------
// DC_LEFT
#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_dc_left_predictor_4x8_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let _ = above;
    let mut sum_left: __m128i = dc_sum_8(left);
    let four: __m128i = _mm_set1_epi16(4);
    sum_left = _mm_add_epi16(sum_left, four);
    sum_left = _mm_srai_epi16::<{ 3 }>(sum_left);
    sum_left = _mm_shufflelo_epi16::<0>(sum_left);
    sum_left = _mm_packus_epi16(sum_left, sum_left);
    let pred: u32 = _mm_cvtsi128_si32(sum_left) as u32;
    dc_store_4xh(pred, 8, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_dc_left_predictor_4x16_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let _ = above;
    let mut sum_left: __m128i = dc_sum_16_sse2(left);
    let eight: __m128i = _mm_set1_epi16(8);
    sum_left = _mm_add_epi16(sum_left, eight);
    sum_left = _mm_srai_epi16::<{ 4 }>(sum_left);
    sum_left = _mm_shufflelo_epi16::<0>(sum_left);
    sum_left = _mm_packus_epi16(sum_left, sum_left);
    let pred: u32 = _mm_cvtsi128_si32(sum_left) as u32;
    dc_store_4xh(pred, 16, dst, stride);
}

// !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER
#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_dc_left_predictor_8x4_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let _ = above;
    let mut sum_left: __m128i = dc_sum_4(left);
    let two: __m128i = _mm_set1_epi16(2);
    sum_left = _mm_add_epi16(sum_left, two);
    sum_left = _mm_srai_epi16::<{ 2 }>(sum_left);
    sum_left = _mm_unpacklo_epi8(sum_left, sum_left);
    let row: __m128i = _mm_shufflelo_epi16::<0>(sum_left);
    dc_store_8xh(&raw const row, 4, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_dc_left_predictor_8x16_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let _ = above;
    let mut sum_left: __m128i = dc_sum_16_sse2(left);
    let eight: __m128i = _mm_set1_epi16(8);
    sum_left = _mm_add_epi16(sum_left, eight);
    sum_left = _mm_srai_epi16::<{ 4 }>(sum_left);
    sum_left = _mm_unpacklo_epi8(sum_left, sum_left);
    let row: __m128i = _mm_shufflelo_epi16::<0>(sum_left);
    dc_store_8xh(&raw const row, 16, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_dc_left_predictor_8x32_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let _ = above;
    let mut sum_left: __m128i = dc_sum_32_sse2(left);
    let sixteen: __m128i = _mm_set1_epi16(16);
    sum_left = _mm_add_epi16(sum_left, sixteen);
    sum_left = _mm_srai_epi16::<{ 5 }>(sum_left);
    sum_left = _mm_unpacklo_epi8(sum_left, sum_left);
    let row: __m128i = _mm_shufflelo_epi16::<0>(sum_left);
    dc_store_8xh(&raw const row, 32, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_dc_left_predictor_16x4_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let _ = above;
    let mut sum_left: __m128i = dc_sum_4(left);
    let two: __m128i = _mm_set1_epi16(2);
    sum_left = _mm_add_epi16(sum_left, two);
    sum_left = _mm_srai_epi16::<{ 2 }>(sum_left);
    sum_left = _mm_unpacklo_epi8(sum_left, sum_left);
    sum_left = _mm_shufflelo_epi16::<0>(sum_left);
    let row: __m128i = _mm_unpacklo_epi64(sum_left, sum_left);
    dc_store_16xh(&raw const row, 4, dst, stride);
}

// !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER
#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_dc_left_predictor_16x8_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let _ = above;
    let mut sum_left: __m128i = dc_sum_8(left);
    let four: __m128i = _mm_set1_epi16(4);
    sum_left = _mm_add_epi16(sum_left, four);
    sum_left = _mm_srai_epi16::<{ 3 }>(sum_left);
    sum_left = _mm_unpacklo_epi8(sum_left, sum_left);
    sum_left = _mm_shufflelo_epi16::<0>(sum_left);
    let row: __m128i = _mm_unpacklo_epi64(sum_left, sum_left);
    dc_store_16xh(&raw const row, 8, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_dc_left_predictor_16x32_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let _ = above;
    let mut sum_left: __m128i = dc_sum_32_sse2(left);
    let sixteen: __m128i = _mm_set1_epi16(16);
    sum_left = _mm_add_epi16(sum_left, sixteen);
    sum_left = _mm_srai_epi16::<{ 5 }>(sum_left);
    sum_left = _mm_unpacklo_epi8(sum_left, sum_left);
    sum_left = _mm_shufflelo_epi16::<0>(sum_left);
    let row: __m128i = _mm_unpacklo_epi64(sum_left, sum_left);
    dc_store_16xh(&raw const row, 32, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_dc_left_predictor_16x64_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let _ = above;
    let mut sum_left: __m128i = dc_sum_64(left);
    let thirtytwo: __m128i = _mm_set1_epi16(32);
    sum_left = _mm_add_epi16(sum_left, thirtytwo);
    sum_left = _mm_srai_epi16::<{ 6 }>(sum_left);
    sum_left = _mm_unpacklo_epi8(sum_left, sum_left);
    sum_left = _mm_shufflelo_epi16::<0>(sum_left);
    let row: __m128i = _mm_unpacklo_epi64(sum_left, sum_left);
    dc_store_16xh(&raw const row, 64, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_dc_left_predictor_32x8_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let _ = above;
    let mut sum_left: __m128i = dc_sum_8(left);
    let four: __m128i = _mm_set1_epi16(4);
    sum_left = _mm_add_epi16(sum_left, four);
    sum_left = _mm_srai_epi16::<{ 3 }>(sum_left);
    sum_left = _mm_unpacklo_epi8(sum_left, sum_left);
    sum_left = _mm_shufflelo_epi16::<0>(sum_left);
    let row: __m128i = _mm_unpacklo_epi64(sum_left, sum_left);
    dc_store_32xh(&raw const row, 8, dst, stride);
}

// !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER
#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_dc_left_predictor_32x16_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let _ = above;
    let mut sum_left: __m128i = dc_sum_16_sse2(left);
    let eight: __m128i = _mm_set1_epi16(8);
    sum_left = _mm_add_epi16(sum_left, eight);
    sum_left = _mm_srai_epi16::<{ 4 }>(sum_left);
    sum_left = _mm_unpacklo_epi8(sum_left, sum_left);
    sum_left = _mm_shufflelo_epi16::<0>(sum_left);
    let row: __m128i = _mm_unpacklo_epi64(sum_left, sum_left);
    dc_store_32xh(&raw const row, 16, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_dc_left_predictor_32x64_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let _ = above;
    let mut sum_left: __m128i = dc_sum_64(left);
    let thirtytwo: __m128i = _mm_set1_epi16(32);
    sum_left = _mm_add_epi16(sum_left, thirtytwo);
    sum_left = _mm_srai_epi16::<{ 6 }>(sum_left);
    sum_left = _mm_unpacklo_epi8(sum_left, sum_left);
    sum_left = _mm_shufflelo_epi16::<0>(sum_left);
    let row: __m128i = _mm_unpacklo_epi64(sum_left, sum_left);
    dc_store_32xh(&raw const row, 64, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_dc_left_predictor_64x64_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let _ = above;
    let mut sum_left: __m128i = dc_sum_64(left);
    let thirtytwo: __m128i = _mm_set1_epi16(32);
    sum_left = _mm_add_epi16(sum_left, thirtytwo);
    sum_left = _mm_srai_epi16::<{ 6 }>(sum_left);
    sum_left = _mm_unpacklo_epi8(sum_left, sum_left);
    sum_left = _mm_shufflelo_epi16::<0>(sum_left);
    let row: __m128i = _mm_unpacklo_epi64(sum_left, sum_left);
    dc_store_64xh(&raw const row, 64, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_dc_left_predictor_64x32_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let _ = above;
    let mut sum_left: __m128i = dc_sum_32_sse2(left);
    let sixteen: __m128i = _mm_set1_epi16(16);
    sum_left = _mm_add_epi16(sum_left, sixteen);
    sum_left = _mm_srai_epi16::<{ 5 }>(sum_left);
    sum_left = _mm_unpacklo_epi8(sum_left, sum_left);
    sum_left = _mm_shufflelo_epi16::<0>(sum_left);
    let row: __m128i = _mm_unpacklo_epi64(sum_left, sum_left);
    dc_store_64xh(&raw const row, 32, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_dc_left_predictor_64x16_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let _ = above;
    let mut sum_left: __m128i = dc_sum_16_sse2(left);
    let eight: __m128i = _mm_set1_epi16(8);
    sum_left = _mm_add_epi16(sum_left, eight);
    sum_left = _mm_srai_epi16::<{ 4 }>(sum_left);
    sum_left = _mm_unpacklo_epi8(sum_left, sum_left);
    sum_left = _mm_shufflelo_epi16::<0>(sum_left);
    let row: __m128i = _mm_unpacklo_epi64(sum_left, sum_left);
    dc_store_64xh(&raw const row, 16, dst, stride);
}

// !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER
// -----------------------------------------------------------------------------
// DC_128
#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_dc_128_predictor_4x8_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let _ = above;
    let _ = left;
    let pred: u32 = 0x80808080;
    dc_store_4xh(pred, 8, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_dc_128_predictor_4x16_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let _ = above;
    let _ = left;
    let pred: u32 = 0x80808080;
    dc_store_4xh(pred, 16, dst, stride);
}

// !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER
#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_dc_128_predictor_8x4_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let _ = above;
    let _ = left;
    let row: __m128i = _mm_set1_epi8(128_i32 as i8);
    dc_store_8xh(&raw const row, 4, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_dc_128_predictor_8x16_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let _ = above;
    let _ = left;
    let row: __m128i = _mm_set1_epi8(128_i32 as i8);
    dc_store_8xh(&raw const row, 16, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_dc_128_predictor_8x32_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let _ = above;
    let _ = left;
    let row: __m128i = _mm_set1_epi8(128_i32 as i8);
    dc_store_8xh(&raw const row, 32, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_dc_128_predictor_16x4_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let _ = above;
    let _ = left;
    let row: __m128i = _mm_set1_epi8(128_i32 as i8);
    dc_store_16xh(&raw const row, 4, dst, stride);
}

// !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER
#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_dc_128_predictor_16x8_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let _ = above;
    let _ = left;
    let row: __m128i = _mm_set1_epi8(128_i32 as i8);
    dc_store_16xh(&raw const row, 8, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_dc_128_predictor_16x32_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let _ = above;
    let _ = left;
    let row: __m128i = _mm_set1_epi8(128_i32 as i8);
    dc_store_16xh(&raw const row, 32, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_dc_128_predictor_16x64_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let _ = above;
    let _ = left;
    let row: __m128i = _mm_set1_epi8(128_i32 as i8);
    dc_store_16xh(&raw const row, 64, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_dc_128_predictor_32x8_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let _ = above;
    let _ = left;
    let row: __m128i = _mm_set1_epi8(128_i32 as i8);
    dc_store_32xh(&raw const row, 8, dst, stride);
}

// !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER
#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_dc_128_predictor_32x16_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let _ = above;
    let _ = left;
    let row: __m128i = _mm_set1_epi8(128_i32 as i8);
    dc_store_32xh(&raw const row, 16, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_dc_128_predictor_32x64_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let _ = above;
    let _ = left;
    let row: __m128i = _mm_set1_epi8(128_i32 as i8);
    dc_store_32xh(&raw const row, 64, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_dc_128_predictor_64x64_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let _ = above;
    let _ = left;
    let row: __m128i = _mm_set1_epi8(128_i32 as i8);
    dc_store_64xh(&raw const row, 64, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_dc_128_predictor_64x32_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let _ = above;
    let _ = left;
    let row: __m128i = _mm_set1_epi8(128_i32 as i8);
    dc_store_64xh(&raw const row, 32, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_dc_128_predictor_64x16_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let _ = above;
    let _ = left;
    let row: __m128i = _mm_set1_epi8(128_i32 as i8);
    dc_store_64xh(&raw const row, 16, dst, stride);
}

// !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER
// -----------------------------------------------------------------------------
// V_PRED
#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_v_predictor_4x8_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let pred: u32 = core::ptr::read_unaligned(above as *mut u32);
    let _ = left;
    dc_store_4xh(pred, 8, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_v_predictor_4x16_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let pred: u32 = core::ptr::read_unaligned(above as *mut u32);
    let _ = left;
    dc_store_4xh(pred, 16, dst, stride);
}

// !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER
#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_v_predictor_8x4_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let row: __m128i = _mm_loadl_epi64(above as *const __m128i);
    let _ = left;
    dc_store_8xh(&raw const row, 4, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_v_predictor_8x16_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let row: __m128i = _mm_loadl_epi64(above as *const __m128i);
    let _ = left;
    dc_store_8xh(&raw const row, 16, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_v_predictor_8x32_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let row: __m128i = _mm_loadl_epi64(above as *const __m128i);
    let _ = left;
    dc_store_8xh(&raw const row, 32, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_v_predictor_16x4_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let row: __m128i = _mm_load_si128(above as *const __m128i);
    let _ = left;
    dc_store_16xh(&raw const row, 4, dst, stride);
}

// !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER
#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_v_predictor_16x8_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let row: __m128i = _mm_load_si128(above as *const __m128i);
    let _ = left;
    dc_store_16xh(&raw const row, 8, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_v_predictor_16x32_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let row: __m128i = _mm_load_si128(above as *const __m128i);
    let _ = left;
    dc_store_16xh(&raw const row, 32, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_v_predictor_16x64_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let row: __m128i = _mm_load_si128(above as *const __m128i);
    let _ = left;
    dc_store_16xh(&raw const row, 64, dst, stride);
}

// !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER
#[inline]
#[target_feature(enable = "sse2")]
unsafe fn v_predictor_32xh(mut dst: *mut u8, stride: isize, above: *const u8, height: i32) {
    let row0: __m128i = _mm_load_si128(above as *const __m128i);
    let row1: __m128i = _mm_load_si128(above.offset(16) as *const __m128i);
    for i in 0..height {
        _mm_store_si128(dst as *mut __m128i, row0);
        _mm_store_si128(dst.offset(16) as *mut __m128i, row1);
        dst = dst.offset(stride);
    }
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_v_predictor_32x8_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let _ = left;
    v_predictor_32xh(dst, stride, above, 8);
}

// !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER
#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_v_predictor_32x16_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let _ = left;
    v_predictor_32xh(dst, stride, above, 16);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_v_predictor_32x64_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let _ = left;
    v_predictor_32xh(dst, stride, above, 64);
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn v_predictor_64xh(mut dst: *mut u8, stride: isize, above: *const u8, height: i32) {
    let row0: __m128i = _mm_load_si128(above as *const __m128i);
    let row1: __m128i = _mm_load_si128(above.offset(16) as *const __m128i);
    let row2: __m128i = _mm_load_si128(above.offset(32) as *const __m128i);
    let row3: __m128i = _mm_load_si128(above.offset(48) as *const __m128i);
    for i in 0..height {
        _mm_store_si128(dst as *mut __m128i, row0);
        _mm_store_si128(dst.offset(16) as *mut __m128i, row1);
        _mm_store_si128(dst.offset(32) as *mut __m128i, row2);
        _mm_store_si128(dst.offset(48) as *mut __m128i, row3);
        dst = dst.offset(stride);
    }
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_v_predictor_64x64_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let _ = left;
    v_predictor_64xh(dst, stride, above, 64);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_v_predictor_64x32_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let _ = left;
    v_predictor_64xh(dst, stride, above, 32);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_v_predictor_64x16_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let _ = left;
    v_predictor_64xh(dst, stride, above, 16);
}

// !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER
// -----------------------------------------------------------------------------
// H_PRED
#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_h_predictor_4x8_sse2(
    mut dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let _ = above;
    let mut left_col: __m128i = _mm_loadl_epi64(left as *const __m128i);
    left_col = _mm_unpacklo_epi8(left_col, left_col);
    let mut row0: __m128i = _mm_shufflelo_epi16::<0>(left_col);
    let mut row1: __m128i = _mm_shufflelo_epi16::<0x55>(left_col);
    let mut row2: __m128i = _mm_shufflelo_epi16::<0xaa>(left_col);
    let mut row3: __m128i = _mm_shufflelo_epi16::<0xff>(left_col);
    core::ptr::write_unaligned(dst as *mut i32, _mm_cvtsi128_si32(row0));
    dst = dst.offset(stride);
    core::ptr::write_unaligned(dst as *mut i32, _mm_cvtsi128_si32(row1));
    dst = dst.offset(stride);
    core::ptr::write_unaligned(dst as *mut i32, _mm_cvtsi128_si32(row2));
    dst = dst.offset(stride);
    core::ptr::write_unaligned(dst as *mut i32, _mm_cvtsi128_si32(row3));
    dst = dst.offset(stride);
    left_col = _mm_unpackhi_epi64(left_col, left_col);
    row0 = _mm_shufflelo_epi16::<0>(left_col);
    row1 = _mm_shufflelo_epi16::<0x55>(left_col);
    row2 = _mm_shufflelo_epi16::<0xaa>(left_col);
    row3 = _mm_shufflelo_epi16::<0xff>(left_col);
    core::ptr::write_unaligned(dst as *mut i32, _mm_cvtsi128_si32(row0));
    dst = dst.offset(stride);
    core::ptr::write_unaligned(dst as *mut i32, _mm_cvtsi128_si32(row1));
    dst = dst.offset(stride);
    core::ptr::write_unaligned(dst as *mut i32, _mm_cvtsi128_si32(row2));
    dst = dst.offset(stride);
    core::ptr::write_unaligned(dst as *mut i32, _mm_cvtsi128_si32(row3));
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_h_predictor_4x16_sse2(
    mut dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let _ = above;
    let left_col: __m128i = _mm_load_si128(left as *const __m128i);
    let mut left_col_low: __m128i = _mm_unpacklo_epi8(left_col, left_col);
    let mut left_col_high: __m128i = _mm_unpackhi_epi8(left_col, left_col);
    let mut row0: __m128i = _mm_shufflelo_epi16::<0>(left_col_low);
    let mut row1: __m128i = _mm_shufflelo_epi16::<0x55>(left_col_low);
    let mut row2: __m128i = _mm_shufflelo_epi16::<0xaa>(left_col_low);
    let mut row3: __m128i = _mm_shufflelo_epi16::<0xff>(left_col_low);
    core::ptr::write_unaligned(dst as *mut i32, _mm_cvtsi128_si32(row0));
    dst = dst.offset(stride);
    core::ptr::write_unaligned(dst as *mut i32, _mm_cvtsi128_si32(row1));
    dst = dst.offset(stride);
    core::ptr::write_unaligned(dst as *mut i32, _mm_cvtsi128_si32(row2));
    dst = dst.offset(stride);
    core::ptr::write_unaligned(dst as *mut i32, _mm_cvtsi128_si32(row3));
    dst = dst.offset(stride);
    left_col_low = _mm_unpackhi_epi64(left_col_low, left_col_low);
    row0 = _mm_shufflelo_epi16::<0>(left_col_low);
    row1 = _mm_shufflelo_epi16::<0x55>(left_col_low);
    row2 = _mm_shufflelo_epi16::<0xaa>(left_col_low);
    row3 = _mm_shufflelo_epi16::<0xff>(left_col_low);
    core::ptr::write_unaligned(dst as *mut i32, _mm_cvtsi128_si32(row0));
    dst = dst.offset(stride);
    core::ptr::write_unaligned(dst as *mut i32, _mm_cvtsi128_si32(row1));
    dst = dst.offset(stride);
    core::ptr::write_unaligned(dst as *mut i32, _mm_cvtsi128_si32(row2));
    dst = dst.offset(stride);
    core::ptr::write_unaligned(dst as *mut i32, _mm_cvtsi128_si32(row3));
    dst = dst.offset(stride);
    row0 = _mm_shufflelo_epi16::<0>(left_col_high);
    row1 = _mm_shufflelo_epi16::<0x55>(left_col_high);
    row2 = _mm_shufflelo_epi16::<0xaa>(left_col_high);
    row3 = _mm_shufflelo_epi16::<0xff>(left_col_high);
    core::ptr::write_unaligned(dst as *mut i32, _mm_cvtsi128_si32(row0));
    dst = dst.offset(stride);
    core::ptr::write_unaligned(dst as *mut i32, _mm_cvtsi128_si32(row1));
    dst = dst.offset(stride);
    core::ptr::write_unaligned(dst as *mut i32, _mm_cvtsi128_si32(row2));
    dst = dst.offset(stride);
    core::ptr::write_unaligned(dst as *mut i32, _mm_cvtsi128_si32(row3));
    dst = dst.offset(stride);
    left_col_high = _mm_unpackhi_epi64(left_col_high, left_col_high);
    row0 = _mm_shufflelo_epi16::<0>(left_col_high);
    row1 = _mm_shufflelo_epi16::<0x55>(left_col_high);
    row2 = _mm_shufflelo_epi16::<0xaa>(left_col_high);
    row3 = _mm_shufflelo_epi16::<0xff>(left_col_high);
    core::ptr::write_unaligned(dst as *mut i32, _mm_cvtsi128_si32(row0));
    dst = dst.offset(stride);
    core::ptr::write_unaligned(dst as *mut i32, _mm_cvtsi128_si32(row1));
    dst = dst.offset(stride);
    core::ptr::write_unaligned(dst as *mut i32, _mm_cvtsi128_si32(row2));
    dst = dst.offset(stride);
    core::ptr::write_unaligned(dst as *mut i32, _mm_cvtsi128_si32(row3));
}

// !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER
#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_h_predictor_8x4_sse2(
    mut dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let _ = above;
    let mut left_col: __m128i = _mm_loadl_epi64(left as *const __m128i);
    left_col = _mm_unpacklo_epi8(left_col, left_col);
    let mut row0: __m128i = _mm_shufflelo_epi16::<0>(left_col);
    let mut row1: __m128i = _mm_shufflelo_epi16::<0x55>(left_col);
    let mut row2: __m128i = _mm_shufflelo_epi16::<0xaa>(left_col);
    let mut row3: __m128i = _mm_shufflelo_epi16::<0xff>(left_col);
    _mm_storel_epi64(dst as *mut __m128i, row0);
    dst = dst.offset(stride);
    _mm_storel_epi64(dst as *mut __m128i, row1);
    dst = dst.offset(stride);
    _mm_storel_epi64(dst as *mut __m128i, row2);
    dst = dst.offset(stride);
    _mm_storel_epi64(dst as *mut __m128i, row3);
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn h_predictor_8x16xc(
    mut dst: *mut u8,
    stride: isize,
    above: *const u8,
    mut left: *const u8,
    count: i32,
) {
    let _ = above;
    for i in 0..count {
        let left_col: __m128i = _mm_load_si128(left as *const __m128i);
        let mut left_col_low: __m128i = _mm_unpacklo_epi8(left_col, left_col);
        let mut left_col_high: __m128i = _mm_unpackhi_epi8(left_col, left_col);
        let mut row0: __m128i = _mm_shufflelo_epi16::<0>(left_col_low);
        let mut row1: __m128i = _mm_shufflelo_epi16::<0x55>(left_col_low);
        let mut row2: __m128i = _mm_shufflelo_epi16::<0xaa>(left_col_low);
        let mut row3: __m128i = _mm_shufflelo_epi16::<0xff>(left_col_low);
        _mm_storel_epi64(dst as *mut __m128i, row0);
        dst = dst.offset(stride);
        _mm_storel_epi64(dst as *mut __m128i, row1);
        dst = dst.offset(stride);
        _mm_storel_epi64(dst as *mut __m128i, row2);
        dst = dst.offset(stride);
        _mm_storel_epi64(dst as *mut __m128i, row3);
        dst = dst.offset(stride);
        left_col_low = _mm_unpackhi_epi64(left_col_low, left_col_low);
        row0 = _mm_shufflelo_epi16::<0>(left_col_low);
        row1 = _mm_shufflelo_epi16::<0x55>(left_col_low);
        row2 = _mm_shufflelo_epi16::<0xaa>(left_col_low);
        row3 = _mm_shufflelo_epi16::<0xff>(left_col_low);
        _mm_storel_epi64(dst as *mut __m128i, row0);
        dst = dst.offset(stride);
        _mm_storel_epi64(dst as *mut __m128i, row1);
        dst = dst.offset(stride);
        _mm_storel_epi64(dst as *mut __m128i, row2);
        dst = dst.offset(stride);
        _mm_storel_epi64(dst as *mut __m128i, row3);
        dst = dst.offset(stride);
        row0 = _mm_shufflelo_epi16::<0>(left_col_high);
        row1 = _mm_shufflelo_epi16::<0x55>(left_col_high);
        row2 = _mm_shufflelo_epi16::<0xaa>(left_col_high);
        row3 = _mm_shufflelo_epi16::<0xff>(left_col_high);
        _mm_storel_epi64(dst as *mut __m128i, row0);
        dst = dst.offset(stride);
        _mm_storel_epi64(dst as *mut __m128i, row1);
        dst = dst.offset(stride);
        _mm_storel_epi64(dst as *mut __m128i, row2);
        dst = dst.offset(stride);
        _mm_storel_epi64(dst as *mut __m128i, row3);
        dst = dst.offset(stride);
        left_col_high = _mm_unpackhi_epi64(left_col_high, left_col_high);
        row0 = _mm_shufflelo_epi16::<0>(left_col_high);
        row1 = _mm_shufflelo_epi16::<0x55>(left_col_high);
        row2 = _mm_shufflelo_epi16::<0xaa>(left_col_high);
        row3 = _mm_shufflelo_epi16::<0xff>(left_col_high);
        _mm_storel_epi64(dst as *mut __m128i, row0);
        dst = dst.offset(stride);
        _mm_storel_epi64(dst as *mut __m128i, row1);
        dst = dst.offset(stride);
        _mm_storel_epi64(dst as *mut __m128i, row2);
        dst = dst.offset(stride);
        _mm_storel_epi64(dst as *mut __m128i, row3);
        dst = dst.offset(stride);
        left = left.offset(16);
    }
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_h_predictor_8x16_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    h_predictor_8x16xc(dst, stride, above, left, 1);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_h_predictor_8x32_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    h_predictor_8x16xc(dst, stride, above, left, 2);
}

// !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER
#[inline]
#[target_feature(enable = "sse2")]
unsafe fn h_pred_store_16xh(row: *const __m128i, h: i32, mut dst: *mut u8, stride: isize) {
    let mut i: i32 = 0;
    {
        i = 0;
        while i < h {
            _mm_store_si128(dst as *mut __m128i, *row.offset(i as isize));
            dst = dst.offset(stride);
            i += 1;
        }
    }
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn repeat_low_4pixels(x: *const __m128i, row: *mut __m128i) {
    let u0: __m128i = _mm_shufflelo_epi16::<0>(*x);
    let u1: __m128i = _mm_shufflelo_epi16::<0x55>(*x);
    let u2: __m128i = _mm_shufflelo_epi16::<0xaa>(*x);
    let u3: __m128i = _mm_shufflelo_epi16::<0xff>(*x);
    *row = _mm_unpacklo_epi64(u0, u0);
    *row.offset(1) = _mm_unpacklo_epi64(u1, u1);
    *row.offset(2) = _mm_unpacklo_epi64(u2, u2);
    *row.offset(3) = _mm_unpacklo_epi64(u3, u3);
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn repeat_high_4pixels(x: *const __m128i, row: *mut __m128i) {
    let u0: __m128i = _mm_shufflehi_epi16::<0>(*x);
    let u1: __m128i = _mm_shufflehi_epi16::<0x55>(*x);
    let u2: __m128i = _mm_shufflehi_epi16::<0xaa>(*x);
    let u3: __m128i = _mm_shufflehi_epi16::<0xff>(*x);
    *row = _mm_unpackhi_epi64(u0, u0);
    *row.offset(1) = _mm_unpackhi_epi64(u1, u1);
    *row.offset(2) = _mm_unpackhi_epi64(u2, u2);
    *row.offset(3) = _mm_unpackhi_epi64(u3, u3);
}

// Process 16x8, first 4 rows
// Use first 8 bytes of left register: xxxxxxxx33221100
#[inline]
#[target_feature(enable = "sse2")]
unsafe fn h_prediction_16x8_1(left: *const __m128i, dst: *mut u8, stride: isize) {
    let mut row: [__m128i; 4] = core::mem::zeroed();
    repeat_low_4pixels(left, row.as_mut_ptr());
    h_pred_store_16xh(row.as_mut_ptr() as *const __m128i, 4, dst, stride);
}

// Process 16x8, second 4 rows
// Use second 8 bytes of left register: 77665544xxxxxxxx
#[inline]
#[target_feature(enable = "sse2")]
unsafe fn h_prediction_16x8_2(left: *const __m128i, dst: *mut u8, stride: isize) {
    let mut row: [__m128i; 4] = core::mem::zeroed();
    repeat_high_4pixels(left, row.as_mut_ptr());
    h_pred_store_16xh(row.as_mut_ptr() as *const __m128i, 4, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_h_predictor_16x4_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let _ = above;
    let left_col: __m128i = _mm_loadl_epi64(left as *const __m128i);
    let left_col_8p: __m128i = _mm_unpacklo_epi8(left_col, left_col);
    h_prediction_16x8_1(&raw const left_col_8p, dst, stride);
}

// !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER
#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_h_predictor_16x8_sse2(
    mut dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let _ = above;
    let left_col: __m128i = _mm_loadl_epi64(left as *const __m128i);
    let left_col_8p: __m128i = _mm_unpacklo_epi8(left_col, left_col);
    h_prediction_16x8_1(&raw const left_col_8p, dst, stride);
    dst = dst.offset(stride << 2);
    h_prediction_16x8_2(&raw const left_col_8p, dst, stride);
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn h_predictor_16xh(mut dst: *mut u8, stride: isize, mut left: *const u8, count: i32) {
    let mut i: i32 = 0;
    loop {
        let left_col: __m128i = _mm_load_si128(left as *const __m128i);
        let left_col_8p_lo: __m128i = _mm_unpacklo_epi8(left_col, left_col);
        h_prediction_16x8_1(&raw const left_col_8p_lo, dst, stride);
        dst = dst.offset(stride << 2);
        h_prediction_16x8_2(&raw const left_col_8p_lo, dst, stride);
        dst = dst.offset(stride << 2);
        let left_col_8p_hi: __m128i = _mm_unpackhi_epi8(left_col, left_col);
        h_prediction_16x8_1(&raw const left_col_8p_hi, dst, stride);
        dst = dst.offset(stride << 2);
        h_prediction_16x8_2(&raw const left_col_8p_hi, dst, stride);
        dst = dst.offset(stride << 2);
        left = left.offset(16);
        i += 1;
        if !(i < count) {
            break;
        }
    }
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_h_predictor_16x32_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let _ = above;
    h_predictor_16xh(dst, stride, left, 2);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_h_predictor_16x64_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let _ = above;
    h_predictor_16xh(dst, stride, left, 4);
}

// !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER
#[inline]
#[target_feature(enable = "sse2")]
unsafe fn h_pred_store_32xh(row: *const __m128i, h: i32, mut dst: *mut u8, stride: isize) {
    let mut i: i32 = 0;
    {
        i = 0;
        while i < h {
            _mm_store_si128(dst as *mut __m128i, *row.offset(i as isize));
            _mm_store_si128(dst.offset(16) as *mut __m128i, *row.offset(i as isize));
            dst = dst.offset(stride);
            i += 1;
        }
    }
}

// Process 32x8, first 4 rows
// Use first 8 bytes of left register: xxxxxxxx33221100
#[inline]
#[target_feature(enable = "sse2")]
unsafe fn h_prediction_32x8_1(left: *const __m128i, dst: *mut u8, stride: isize) {
    let mut row: [__m128i; 4] = core::mem::zeroed();
    repeat_low_4pixels(left, row.as_mut_ptr());
    h_pred_store_32xh(row.as_mut_ptr() as *const __m128i, 4, dst, stride);
}

// Process 32x8, second 4 rows
// Use second 8 bytes of left register: 77665544xxxxxxxx
#[inline]
#[target_feature(enable = "sse2")]
unsafe fn h_prediction_32x8_2(left: *const __m128i, dst: *mut u8, stride: isize) {
    let mut row: [__m128i; 4] = core::mem::zeroed();
    repeat_high_4pixels(left, row.as_mut_ptr());
    h_pred_store_32xh(row.as_mut_ptr() as *const __m128i, 4, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_h_predictor_32x8_sse2(
    mut dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let mut left_col: __m128i;
    let mut left_col_8p: __m128i = core::mem::zeroed();
    let _ = above;
    left_col = _mm_load_si128(left as *const __m128i);
    left_col_8p = _mm_unpacklo_epi8(left_col, left_col);
    h_prediction_32x8_1(&raw mut left_col_8p as *const __m128i, dst, stride);
    dst = dst.offset(stride << 2);
    h_prediction_32x8_2(&raw mut left_col_8p as *const __m128i, dst, stride);
}

// !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER
#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_h_predictor_32x16_sse2(
    mut dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let mut left_col: __m128i;
    let mut left_col_8p: __m128i = core::mem::zeroed();
    let _ = above;
    left_col = _mm_load_si128(left as *const __m128i);
    left_col_8p = _mm_unpacklo_epi8(left_col, left_col);
    h_prediction_32x8_1(&raw mut left_col_8p as *const __m128i, dst, stride);
    dst = dst.offset(stride << 2);
    h_prediction_32x8_2(&raw mut left_col_8p as *const __m128i, dst, stride);
    dst = dst.offset(stride << 2);
    left_col_8p = _mm_unpackhi_epi8(left_col, left_col);
    h_prediction_32x8_1(&raw mut left_col_8p as *const __m128i, dst, stride);
    dst = dst.offset(stride << 2);
    h_prediction_32x8_2(&raw mut left_col_8p as *const __m128i, dst, stride);
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn h_predictor_32xh(mut dst: *mut u8, stride: isize, mut left: *const u8, height: i32) {
    let mut i: i32 = height >> 2;
    loop {
        let mut left4: __m128i = _mm_cvtsi32_si128(core::ptr::read_unaligned(left as *mut i32));
        left4 = _mm_unpacklo_epi8(left4, left4);
        left4 = _mm_unpacklo_epi8(left4, left4);
        let r0: __m128i = _mm_shuffle_epi32::<0x0>(left4);
        let r1: __m128i = _mm_shuffle_epi32::<0x55>(left4);
        _mm_store_si128(dst as *mut __m128i, r0);
        _mm_store_si128(dst.offset(16) as *mut __m128i, r0);
        _mm_store_si128(dst.offset(stride) as *mut __m128i, r1);
        _mm_store_si128(dst.offset(stride).offset(16) as *mut __m128i, r1);
        let r2: __m128i = _mm_shuffle_epi32::<0xaa>(left4);
        let r3: __m128i = _mm_shuffle_epi32::<0xff>(left4);
        _mm_store_si128(dst.offset(stride * 2) as *mut __m128i, r2);
        _mm_store_si128(dst.offset(stride * 2).offset(16) as *mut __m128i, r2);
        _mm_store_si128(dst.offset(stride * 3) as *mut __m128i, r3);
        _mm_store_si128(dst.offset(stride * 3).offset(16) as *mut __m128i, r3);
        left = left.offset(4);
        dst = dst.offset(stride * 4);
        i -= 1;
        if !(i != 0) {
            break;
        }
    }
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_h_predictor_32x64_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let _ = above;
    h_predictor_32xh(dst, stride, left, 64);
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn h_predictor_64xh(mut dst: *mut u8, stride: isize, mut left: *const u8, height: i32) {
    let mut i: i32 = height >> 2;
    loop {
        let mut left4: __m128i = _mm_cvtsi32_si128(core::ptr::read_unaligned(left as *mut i32));
        left4 = _mm_unpacklo_epi8(left4, left4);
        left4 = _mm_unpacklo_epi8(left4, left4);
        let r0: __m128i = _mm_shuffle_epi32::<0x0>(left4);
        let r1: __m128i = _mm_shuffle_epi32::<0x55>(left4);
        _mm_store_si128(dst as *mut __m128i, r0);
        _mm_store_si128(dst.offset(16) as *mut __m128i, r0);
        _mm_store_si128(dst.offset(32) as *mut __m128i, r0);
        _mm_store_si128(dst.offset(48) as *mut __m128i, r0);
        _mm_store_si128(dst.offset(stride) as *mut __m128i, r1);
        _mm_store_si128(dst.offset(stride).offset(16) as *mut __m128i, r1);
        _mm_store_si128(dst.offset(stride).offset(32) as *mut __m128i, r1);
        _mm_store_si128(dst.offset(stride).offset(48) as *mut __m128i, r1);
        let r2: __m128i = _mm_shuffle_epi32::<0xaa>(left4);
        let r3: __m128i = _mm_shuffle_epi32::<0xff>(left4);
        _mm_store_si128(dst.offset(stride * 2) as *mut __m128i, r2);
        _mm_store_si128(dst.offset(stride * 2).offset(16) as *mut __m128i, r2);
        _mm_store_si128(dst.offset(stride * 2).offset(32) as *mut __m128i, r2);
        _mm_store_si128(dst.offset(stride * 2).offset(48) as *mut __m128i, r2);
        _mm_store_si128(dst.offset(stride * 3) as *mut __m128i, r3);
        _mm_store_si128(dst.offset(stride * 3).offset(16) as *mut __m128i, r3);
        _mm_store_si128(dst.offset(stride * 3).offset(32) as *mut __m128i, r3);
        _mm_store_si128(dst.offset(stride * 3).offset(48) as *mut __m128i, r3);
        left = left.offset(4);
        dst = dst.offset(stride * 4);
        i -= 1;
        if !(i != 0) {
            break;
        }
    }
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_h_predictor_64x64_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let _ = above;
    h_predictor_64xh(dst, stride, left, 64);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_h_predictor_64x32_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let _ = above;
    h_predictor_64xh(dst, stride, left, 32);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_h_predictor_64x16_sse2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let _ = above;
    h_predictor_64xh(dst, stride, left, 16);
}

// !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER
