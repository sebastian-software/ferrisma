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

// Translated by hand from upstream/aom/aom_dsp/x86/highbd_intrapred_sse2.c
// (ADR 0056, ADR 0058). Upstream compiles this file with -msse2.
//
// The file defines the H predictors for every size up to 32x32, the DC_TOP,
// DC_LEFT and DC_128 predictors for the same sizes, and the rectangular V and
// DC predictors. The square V and DC predictors (4x4, 8x8, 16x16, 32x32) live
// only in highbd_intrapred_asm_sse2.asm and are not translated (ADR 0058).
//
// Translation notes:
// - `(void)x;` becomes `let _ = x;`.
// - `uint16_t **dst` parameters of the store helpers become `*mut *mut u16`.
// - `(int16_t)` arguments of `_mm_set1_epi16` become `as i16`.
use core::arch::x86_64::*;

// -----------------------------------------------------------------------------
// H_PRED

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_highbd_h_predictor_4x4_sse2(mut dst: *mut u16, stride: isize,
                                                                above: *const u16, left: *const u16,
                                                                bd: i32) {
    let left_u16 = _mm_loadl_epi64(left as *const __m128i);
    let row0 = _mm_shufflelo_epi16::<0x0>(left_u16);
    let row1 = _mm_shufflelo_epi16::<0x55>(left_u16);
    let row2 = _mm_shufflelo_epi16::<0xaa>(left_u16);
    let row3 = _mm_shufflelo_epi16::<0xff>(left_u16);
    let _ = above;
    let _ = bd;
    _mm_storel_epi64(dst as *mut __m128i, row0);
    dst = dst.offset(stride);
    _mm_storel_epi64(dst as *mut __m128i, row1);
    dst = dst.offset(stride);
    _mm_storel_epi64(dst as *mut __m128i, row2);
    dst = dst.offset(stride);
    _mm_storel_epi64(dst as *mut __m128i, row3);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_highbd_h_predictor_4x8_sse2(mut dst: *mut u16, stride: isize,
                                                                above: *const u16, mut left: *const u16,
                                                                bd: i32) {
    aom_highbd_h_predictor_4x4_sse2(dst, stride, above, left, bd);
    dst = dst.offset(stride << 2);
    left = left.add(4);
    aom_highbd_h_predictor_4x4_sse2(dst, stride, above, left, bd);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_highbd_h_predictor_8x4_sse2(mut dst: *mut u16, stride: isize,
                                                                above: *const u16, left: *const u16,
                                                                bd: i32) {
    let left_u16 = _mm_load_si128(left as *const __m128i);
    let row0 = _mm_shufflelo_epi16::<0x0>(left_u16);
    let row1 = _mm_shufflelo_epi16::<0x55>(left_u16);
    let row2 = _mm_shufflelo_epi16::<0xaa>(left_u16);
    let row3 = _mm_shufflelo_epi16::<0xff>(left_u16);
    let _ = above;
    let _ = bd;
    _mm_store_si128(dst as *mut __m128i, _mm_unpacklo_epi64(row0, row0));
    dst = dst.offset(stride);
    _mm_store_si128(dst as *mut __m128i, _mm_unpacklo_epi64(row1, row1));
    dst = dst.offset(stride);
    _mm_store_si128(dst as *mut __m128i, _mm_unpacklo_epi64(row2, row2));
    dst = dst.offset(stride);
    _mm_store_si128(dst as *mut __m128i, _mm_unpacklo_epi64(row3, row3));
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_highbd_h_predictor_8x8_sse2(mut dst: *mut u16, stride: isize,
                                                                above: *const u16, left: *const u16,
                                                                bd: i32) {
    let left_u16 = _mm_load_si128(left as *const __m128i);
    let row0 = _mm_shufflelo_epi16::<0x0>(left_u16);
    let row1 = _mm_shufflelo_epi16::<0x55>(left_u16);
    let row2 = _mm_shufflelo_epi16::<0xaa>(left_u16);
    let row3 = _mm_shufflelo_epi16::<0xff>(left_u16);
    let row4 = _mm_shufflehi_epi16::<0x0>(left_u16);
    let row5 = _mm_shufflehi_epi16::<0x55>(left_u16);
    let row6 = _mm_shufflehi_epi16::<0xaa>(left_u16);
    let row7 = _mm_shufflehi_epi16::<0xff>(left_u16);
    let _ = above;
    let _ = bd;
    _mm_store_si128(dst as *mut __m128i, _mm_unpacklo_epi64(row0, row0));
    dst = dst.offset(stride);
    _mm_store_si128(dst as *mut __m128i, _mm_unpacklo_epi64(row1, row1));
    dst = dst.offset(stride);
    _mm_store_si128(dst as *mut __m128i, _mm_unpacklo_epi64(row2, row2));
    dst = dst.offset(stride);
    _mm_store_si128(dst as *mut __m128i, _mm_unpacklo_epi64(row3, row3));
    dst = dst.offset(stride);
    _mm_store_si128(dst as *mut __m128i, _mm_unpackhi_epi64(row4, row4));
    dst = dst.offset(stride);
    _mm_store_si128(dst as *mut __m128i, _mm_unpackhi_epi64(row5, row5));
    dst = dst.offset(stride);
    _mm_store_si128(dst as *mut __m128i, _mm_unpackhi_epi64(row6, row6));
    dst = dst.offset(stride);
    _mm_store_si128(dst as *mut __m128i, _mm_unpackhi_epi64(row7, row7));
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_highbd_h_predictor_8x16_sse2(mut dst: *mut u16, stride: isize,
                                                                 above: *const u16, mut left: *const u16,
                                                                 bd: i32) {
    aom_highbd_h_predictor_8x8_sse2(dst, stride, above, left, bd);
    dst = dst.offset(stride << 3);
    left = left.add(8);
    aom_highbd_h_predictor_8x8_sse2(dst, stride, above, left, bd);
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn h_store_16_unpacklo(dst: *mut *mut u16, stride: isize, row: *const __m128i) {
    let val = _mm_unpacklo_epi64(*row, *row);
    _mm_store_si128(*dst as *mut __m128i, val);
    _mm_store_si128((*dst).add(8) as *mut __m128i, val);
    *dst = (*dst).offset(stride);
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn h_store_16_unpackhi(dst: *mut *mut u16, stride: isize, row: *const __m128i) {
    let val = _mm_unpackhi_epi64(*row, *row);
    _mm_store_si128(*dst as *mut __m128i, val);
    _mm_store_si128((*dst).add(8) as *mut __m128i, val);
    *dst = (*dst).offset(stride);
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn h_predictor_16x8(mut dst: *mut u16, stride: isize, left: *const u16) {
    let left_u16 = _mm_load_si128(left as *const __m128i);
    let row0 = _mm_shufflelo_epi16::<0x0>(left_u16);
    let row1 = _mm_shufflelo_epi16::<0x55>(left_u16);
    let row2 = _mm_shufflelo_epi16::<0xaa>(left_u16);
    let row3 = _mm_shufflelo_epi16::<0xff>(left_u16);
    let row4 = _mm_shufflehi_epi16::<0x0>(left_u16);
    let row5 = _mm_shufflehi_epi16::<0x55>(left_u16);
    let row6 = _mm_shufflehi_epi16::<0xaa>(left_u16);
    let row7 = _mm_shufflehi_epi16::<0xff>(left_u16);
    h_store_16_unpacklo(&mut dst, stride, &row0);
    h_store_16_unpacklo(&mut dst, stride, &row1);
    h_store_16_unpacklo(&mut dst, stride, &row2);
    h_store_16_unpacklo(&mut dst, stride, &row3);
    h_store_16_unpackhi(&mut dst, stride, &row4);
    h_store_16_unpackhi(&mut dst, stride, &row5);
    h_store_16_unpackhi(&mut dst, stride, &row6);
    h_store_16_unpackhi(&mut dst, stride, &row7);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_highbd_h_predictor_16x8_sse2(dst: *mut u16, stride: isize,
                                                                 above: *const u16, left: *const u16,
                                                                 bd: i32) {
    let _ = above;
    let _ = bd;
    h_predictor_16x8(dst, stride, left);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_highbd_h_predictor_16x16_sse2(mut dst: *mut u16, stride: isize,
                                                                  above: *const u16, mut left: *const u16,
                                                                  bd: i32) {
    let _ = above;
    let _ = bd;

    let mut i = 0;
    while i < 2 {
        h_predictor_16x8(dst, stride, left);
        dst = dst.offset(stride << 3);
        i += 1;
        left = left.add(8);
    }
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_highbd_h_predictor_16x32_sse2(mut dst: *mut u16, stride: isize,
                                                                  above: *const u16, mut left: *const u16,
                                                                  bd: i32) {
    let _ = above;
    let _ = bd;

    let mut i = 0;
    while i < 4 {
        h_predictor_16x8(dst, stride, left);
        dst = dst.offset(stride << 3);
        i += 1;
        left = left.add(8);
    }
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn h_store_32_unpacklo(dst: *mut *mut u16, stride: isize, row: *const __m128i) {
    let val = _mm_unpacklo_epi64(*row, *row);
    _mm_store_si128(*dst as *mut __m128i, val);
    _mm_store_si128((*dst).add(8) as *mut __m128i, val);
    _mm_store_si128((*dst).add(16) as *mut __m128i, val);
    _mm_store_si128((*dst).add(24) as *mut __m128i, val);
    *dst = (*dst).offset(stride);
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn h_store_32_unpackhi(dst: *mut *mut u16, stride: isize, row: *const __m128i) {
    let val = _mm_unpackhi_epi64(*row, *row);
    _mm_store_si128(*dst as *mut __m128i, val);
    _mm_store_si128((*dst).add(8) as *mut __m128i, val);
    _mm_store_si128((*dst).add(16) as *mut __m128i, val);
    _mm_store_si128((*dst).add(24) as *mut __m128i, val);
    *dst = (*dst).offset(stride);
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn h_predictor_32x8(mut dst: *mut u16, stride: isize, left: *const u16) {
    let left_u16 = _mm_load_si128(left as *const __m128i);
    let row0 = _mm_shufflelo_epi16::<0x0>(left_u16);
    let row1 = _mm_shufflelo_epi16::<0x55>(left_u16);
    let row2 = _mm_shufflelo_epi16::<0xaa>(left_u16);
    let row3 = _mm_shufflelo_epi16::<0xff>(left_u16);
    let row4 = _mm_shufflehi_epi16::<0x0>(left_u16);
    let row5 = _mm_shufflehi_epi16::<0x55>(left_u16);
    let row6 = _mm_shufflehi_epi16::<0xaa>(left_u16);
    let row7 = _mm_shufflehi_epi16::<0xff>(left_u16);
    h_store_32_unpacklo(&mut dst, stride, &row0);
    h_store_32_unpacklo(&mut dst, stride, &row1);
    h_store_32_unpacklo(&mut dst, stride, &row2);
    h_store_32_unpacklo(&mut dst, stride, &row3);
    h_store_32_unpackhi(&mut dst, stride, &row4);
    h_store_32_unpackhi(&mut dst, stride, &row5);
    h_store_32_unpackhi(&mut dst, stride, &row6);
    h_store_32_unpackhi(&mut dst, stride, &row7);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_highbd_h_predictor_32x16_sse2(mut dst: *mut u16, stride: isize,
                                                                  above: *const u16, mut left: *const u16,
                                                                  bd: i32) {
    let _ = above;
    let _ = bd;

    let mut i = 0;
    while i < 2 {
        h_predictor_32x8(dst, stride, left);
        dst = dst.offset(stride << 3);
        i += 1;
        left = left.add(8);
    }
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_highbd_h_predictor_32x32_sse2(mut dst: *mut u16, stride: isize,
                                                                  above: *const u16, mut left: *const u16,
                                                                  bd: i32) {
    let _ = above;
    let _ = bd;

    let mut i = 0;
    while i < 4 {
        h_predictor_32x8(dst, stride, left);
        dst = dst.offset(stride << 3);
        i += 1;
        left = left.add(8);
    }
}

// -----------------------------------------------------------------------------
// DC_TOP, DC_LEFT, DC_128

// 4x4

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn dc_sum_4(ref_: *const u16) -> __m128i {
    let _dcba = _mm_loadl_epi64(ref_ as *const __m128i);
    let _xxdc = _mm_shufflelo_epi16::<0xe>(_dcba);
    let a = _mm_add_epi16(_dcba, _xxdc);
    _mm_add_epi16(a, _mm_shufflelo_epi16::<0x1>(a))
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn dc_store_4x4(mut dst: *mut u16, stride: isize, dc: *const __m128i) {
    let dc_dup = _mm_shufflelo_epi16::<0x0>(*dc);
    let mut i = 0;
    while i < 4 {
        _mm_storel_epi64(dst as *mut __m128i, dc_dup);
        i += 1;
        dst = dst.offset(stride);
    }
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_highbd_dc_left_predictor_4x4_sse2(dst: *mut u16, stride: isize,
                                                                      above: *const u16, left: *const u16,
                                                                      bd: i32) {
    let two = _mm_cvtsi32_si128(2);
    let sum = dc_sum_4(left);
    let dc = _mm_srli_epi16::<2>(_mm_add_epi16(sum, two));
    let _ = above;
    let _ = bd;
    dc_store_4x4(dst, stride, &dc);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_highbd_dc_top_predictor_4x4_sse2(dst: *mut u16, stride: isize,
                                                                     above: *const u16, left: *const u16,
                                                                     bd: i32) {
    let two = _mm_cvtsi32_si128(2);
    let sum = dc_sum_4(above);
    let dc = _mm_srli_epi16::<2>(_mm_add_epi16(sum, two));
    let _ = left;
    let _ = bd;
    dc_store_4x4(dst, stride, &dc);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_highbd_dc_128_predictor_4x4_sse2(dst: *mut u16, stride: isize,
                                                                     above: *const u16, left: *const u16,
                                                                     bd: i32) {
    let dc = _mm_cvtsi32_si128(1 << (bd - 1));
    let dc_dup = _mm_shufflelo_epi16::<0x0>(dc);
    let _ = above;
    let _ = left;
    dc_store_4x4(dst, stride, &dc_dup);
}

// -----------------------------------------------------------------------------
// 4x8

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn dc_store_4x8(mut dst: *mut u16, stride: isize, dc: *const __m128i) {
    let dc_dup = _mm_shufflelo_epi16::<0x0>(*dc);
    let mut i = 0;
    while i < 8 {
        _mm_storel_epi64(dst as *mut __m128i, dc_dup);
        i += 1;
        dst = dst.offset(stride);
    }
}

// Shared with DC 8xh
#[inline]
#[target_feature(enable = "sse2")]
unsafe fn dc_sum_8(ref_: *const u16) -> __m128i {
    let ref_u16 = _mm_load_si128(ref_ as *const __m128i);
    let _dcba = _mm_add_epi16(ref_u16, _mm_srli_si128::<8>(ref_u16));
    let _xxdc = _mm_shufflelo_epi16::<0xe>(_dcba);
    let a = _mm_add_epi16(_dcba, _xxdc);

    _mm_add_epi16(a, _mm_shufflelo_epi16::<0x1>(a))
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_highbd_dc_left_predictor_4x8_sse2(dst: *mut u16, stride: isize,
                                                                      above: *const u16, left: *const u16,
                                                                      bd: i32) {
    let sum = dc_sum_8(left);
    let four = _mm_cvtsi32_si128(4);
    let dc = _mm_srli_epi16::<3>(_mm_add_epi16(sum, four));
    let _ = above;
    let _ = bd;
    dc_store_4x8(dst, stride, &dc);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_highbd_dc_top_predictor_4x8_sse2(dst: *mut u16, stride: isize,
                                                                     above: *const u16, left: *const u16,
                                                                     bd: i32) {
    let two = _mm_cvtsi32_si128(2);
    let sum = dc_sum_4(above);
    let dc = _mm_srli_epi16::<2>(_mm_add_epi16(sum, two));
    let _ = left;
    let _ = bd;
    dc_store_4x8(dst, stride, &dc);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_highbd_dc_128_predictor_4x8_sse2(dst: *mut u16, stride: isize,
                                                                     above: *const u16, left: *const u16,
                                                                     bd: i32) {
    let dc = _mm_cvtsi32_si128(1 << (bd - 1));
    let dc_dup = _mm_shufflelo_epi16::<0x0>(dc);
    let _ = above;
    let _ = left;
    dc_store_4x8(dst, stride, &dc_dup);
}

// -----------------------------------------------------------------------------
// 8xh

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn dc_store_8xh(mut dst: *mut u16, stride: isize, height: i32, dc: *const __m128i) {
    let dc_dup_lo = _mm_shufflelo_epi16::<0>(*dc);
    let dc_dup = _mm_unpacklo_epi64(dc_dup_lo, dc_dup_lo);
    let mut i = 0;
    while i < height {
        _mm_store_si128(dst as *mut __m128i, dc_dup);
        i += 1;
        dst = dst.offset(stride);
    }
}

// -----------------------------------------------------------------------------
// DC_TOP

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn dc_top_predictor_8xh(dst: *mut u16, stride: isize, height: i32, above: *const u16) {
    let four = _mm_cvtsi32_si128(4);
    let sum = dc_sum_8(above);
    let dc = _mm_srli_epi16::<3>(_mm_add_epi16(sum, four));
    dc_store_8xh(dst, stride, height, &dc);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_highbd_dc_top_predictor_8x4_sse2(dst: *mut u16, stride: isize,
                                                                     above: *const u16, left: *const u16,
                                                                     bd: i32) {
    let _ = left;
    let _ = bd;
    dc_top_predictor_8xh(dst, stride, 4, above);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_highbd_dc_top_predictor_8x8_sse2(dst: *mut u16, stride: isize,
                                                                     above: *const u16, left: *const u16,
                                                                     bd: i32) {
    let _ = left;
    let _ = bd;
    dc_top_predictor_8xh(dst, stride, 8, above);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_highbd_dc_top_predictor_8x16_sse2(dst: *mut u16, stride: isize,
                                                                      above: *const u16, left: *const u16,
                                                                      bd: i32) {
    let _ = left;
    let _ = bd;
    dc_top_predictor_8xh(dst, stride, 16, above);
}

// -----------------------------------------------------------------------------
// DC_LEFT

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_highbd_dc_left_predictor_8x4_sse2(dst: *mut u16, stride: isize,
                                                                      above: *const u16, left: *const u16,
                                                                      bd: i32) {
    let two = _mm_cvtsi32_si128(2);
    let sum = dc_sum_4(left);
    let dc = _mm_srli_epi16::<2>(_mm_add_epi16(sum, two));
    let _ = above;
    let _ = bd;
    dc_store_8xh(dst, stride, 4, &dc);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_highbd_dc_left_predictor_8x8_sse2(dst: *mut u16, stride: isize,
                                                                      above: *const u16, left: *const u16,
                                                                      bd: i32) {
    let four = _mm_cvtsi32_si128(4);
    let sum = dc_sum_8(left);
    let dc = _mm_srli_epi16::<3>(_mm_add_epi16(sum, four));
    let _ = above;
    let _ = bd;
    dc_store_8xh(dst, stride, 8, &dc);
}

// Shared with DC 16xh
#[inline]
#[target_feature(enable = "sse2")]
unsafe fn dc_sum_16(ref_: *const u16) -> __m128i {
    let sum_lo = dc_sum_8(ref_);
    let sum_hi = dc_sum_8(ref_.add(8));
    _mm_add_epi16(sum_lo, sum_hi)
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_highbd_dc_left_predictor_8x16_sse2(dst: *mut u16, stride: isize,
                                                                       above: *const u16, left: *const u16,
                                                                       bd: i32) {
    let eight = _mm_cvtsi32_si128(8);
    let sum = dc_sum_16(left);
    let dc = _mm_srli_epi16::<4>(_mm_add_epi16(sum, eight));
    let _ = above;
    let _ = bd;
    dc_store_8xh(dst, stride, 16, &dc);
}

// -----------------------------------------------------------------------------
// DC_128

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn dc_128_predictor_8xh(dst: *mut u16, stride: isize, height: i32, bd: i32) {
    let dc = _mm_cvtsi32_si128(1 << (bd - 1));
    let dc_dup = _mm_shufflelo_epi16::<0x0>(dc);
    dc_store_8xh(dst, stride, height, &dc_dup);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_highbd_dc_128_predictor_8x4_sse2(dst: *mut u16, stride: isize,
                                                                     above: *const u16, left: *const u16,
                                                                     bd: i32) {
    let _ = above;
    let _ = left;
    dc_128_predictor_8xh(dst, stride, 4, bd);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_highbd_dc_128_predictor_8x8_sse2(dst: *mut u16, stride: isize,
                                                                     above: *const u16, left: *const u16,
                                                                     bd: i32) {
    let _ = above;
    let _ = left;
    dc_128_predictor_8xh(dst, stride, 8, bd);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_highbd_dc_128_predictor_8x16_sse2(dst: *mut u16, stride: isize,
                                                                      above: *const u16, left: *const u16,
                                                                      bd: i32) {
    let _ = above;
    let _ = left;
    dc_128_predictor_8xh(dst, stride, 16, bd);
}

// -----------------------------------------------------------------------------
// 16xh

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn dc_store_16xh(mut dst: *mut u16, stride: isize, height: i32, dc: *const __m128i) {
    let dc_dup_lo = _mm_shufflelo_epi16::<0>(*dc);
    let dc_dup = _mm_unpacklo_epi64(dc_dup_lo, dc_dup_lo);
    let mut i = 0;
    while i < height {
        _mm_store_si128(dst as *mut __m128i, dc_dup);
        _mm_store_si128(dst.add(8) as *mut __m128i, dc_dup);
        i += 1;
        dst = dst.offset(stride);
    }
}

// -----------------------------------------------------------------------------
// DC_LEFT

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_highbd_dc_left_predictor_16x8_sse2(dst: *mut u16, stride: isize,
                                                                       above: *const u16, left: *const u16,
                                                                       bd: i32) {
    let four = _mm_cvtsi32_si128(4);
    let sum = dc_sum_8(left);
    let dc = _mm_srli_epi16::<3>(_mm_add_epi16(sum, four));
    let _ = above;
    let _ = bd;
    dc_store_16xh(dst, stride, 8, &dc);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_highbd_dc_left_predictor_16x16_sse2(dst: *mut u16, stride: isize,
                                                                        above: *const u16, left: *const u16,
                                                                        bd: i32) {
    let eight = _mm_cvtsi32_si128(8);
    let sum = dc_sum_16(left);
    let dc = _mm_srli_epi16::<4>(_mm_add_epi16(sum, eight));
    let _ = above;
    let _ = bd;
    dc_store_16xh(dst, stride, 16, &dc);
}

// Shared with 32xh
#[inline]
#[target_feature(enable = "sse2")]
unsafe fn dc_sum_32(ref_: *const u16) -> __m128i {
    let zero = _mm_setzero_si128();
    let sum_a = dc_sum_16(ref_);
    let sum_b = dc_sum_16(ref_.add(16));
    // 12 bit bd will outrange, so expand to 32 bit before adding final total
    _mm_add_epi32(_mm_unpacklo_epi16(sum_a, zero), _mm_unpacklo_epi16(sum_b, zero))
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_highbd_dc_left_predictor_16x32_sse2(dst: *mut u16, stride: isize,
                                                                        above: *const u16, left: *const u16,
                                                                        bd: i32) {
    let sixteen = _mm_cvtsi32_si128(16);
    let sum = dc_sum_32(left);
    let dc = _mm_srli_epi32::<5>(_mm_add_epi32(sum, sixteen));
    let _ = above;
    let _ = bd;
    dc_store_16xh(dst, stride, 32, &dc);
}

// -----------------------------------------------------------------------------
// DC_TOP

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_highbd_dc_top_predictor_16x8_sse2(dst: *mut u16, stride: isize,
                                                                      above: *const u16, left: *const u16,
                                                                      bd: i32) {
    let eight = _mm_cvtsi32_si128(8);
    let sum = dc_sum_16(above);
    let dc = _mm_srli_epi16::<4>(_mm_add_epi16(sum, eight));
    let _ = left;
    let _ = bd;
    dc_store_16xh(dst, stride, 8, &dc);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_highbd_dc_top_predictor_16x16_sse2(dst: *mut u16, stride: isize,
                                                                       above: *const u16, left: *const u16,
                                                                       bd: i32) {
    let eight = _mm_cvtsi32_si128(8);
    let sum = dc_sum_16(above);
    let dc = _mm_srli_epi16::<4>(_mm_add_epi16(sum, eight));
    let _ = left;
    let _ = bd;
    dc_store_16xh(dst, stride, 16, &dc);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_highbd_dc_top_predictor_16x32_sse2(dst: *mut u16, stride: isize,
                                                                       above: *const u16, left: *const u16,
                                                                       bd: i32) {
    let eight = _mm_cvtsi32_si128(8);
    let sum = dc_sum_16(above);
    let dc = _mm_srli_epi16::<4>(_mm_add_epi16(sum, eight));
    let _ = left;
    let _ = bd;
    dc_store_16xh(dst, stride, 32, &dc);
}

// -----------------------------------------------------------------------------
// DC_128

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_highbd_dc_128_predictor_16x8_sse2(dst: *mut u16, stride: isize,
                                                                      above: *const u16, left: *const u16,
                                                                      bd: i32) {
    let dc = _mm_cvtsi32_si128(1 << (bd - 1));
    let dc_dup = _mm_shufflelo_epi16::<0x0>(dc);
    let _ = above;
    let _ = left;
    dc_store_16xh(dst, stride, 8, &dc_dup);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_highbd_dc_128_predictor_16x16_sse2(dst: *mut u16, stride: isize,
                                                                       above: *const u16, left: *const u16,
                                                                       bd: i32) {
    let dc = _mm_cvtsi32_si128(1 << (bd - 1));
    let dc_dup = _mm_shufflelo_epi16::<0x0>(dc);
    let _ = above;
    let _ = left;
    dc_store_16xh(dst, stride, 16, &dc_dup);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_highbd_dc_128_predictor_16x32_sse2(dst: *mut u16, stride: isize,
                                                                       above: *const u16, left: *const u16,
                                                                       bd: i32) {
    let dc = _mm_cvtsi32_si128(1 << (bd - 1));
    let dc_dup = _mm_shufflelo_epi16::<0x0>(dc);
    let _ = above;
    let _ = left;
    dc_store_16xh(dst, stride, 32, &dc_dup);
}

// -----------------------------------------------------------------------------
// 32xh

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn dc_store_32xh(mut dst: *mut u16, stride: isize, height: i32, dc: *const __m128i) {
    let dc_dup_lo = _mm_shufflelo_epi16::<0>(*dc);
    let dc_dup = _mm_unpacklo_epi64(dc_dup_lo, dc_dup_lo);
    let mut i = 0;
    while i < height {
        _mm_store_si128(dst as *mut __m128i, dc_dup);
        _mm_store_si128(dst.add(8) as *mut __m128i, dc_dup);
        _mm_store_si128(dst.add(16) as *mut __m128i, dc_dup);
        _mm_store_si128(dst.add(24) as *mut __m128i, dc_dup);
        i += 1;
        dst = dst.offset(stride);
    }
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_highbd_dc_left_predictor_32x16_sse2(dst: *mut u16, stride: isize,
                                                                        above: *const u16, left: *const u16,
                                                                        bd: i32) {
    let eight = _mm_cvtsi32_si128(8);
    let sum = dc_sum_16(left);
    let dc = _mm_srli_epi16::<4>(_mm_add_epi16(sum, eight));
    let _ = above;
    let _ = bd;
    dc_store_32xh(dst, stride, 16, &dc);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_highbd_dc_left_predictor_32x32_sse2(dst: *mut u16, stride: isize,
                                                                        above: *const u16, left: *const u16,
                                                                        bd: i32) {
    let sixteen = _mm_cvtsi32_si128(16);
    let sum = dc_sum_32(left);
    let dc = _mm_srli_epi32::<5>(_mm_add_epi32(sum, sixteen));
    let _ = above;
    let _ = bd;
    dc_store_32xh(dst, stride, 32, &dc);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_highbd_dc_top_predictor_32x16_sse2(dst: *mut u16, stride: isize,
                                                                       above: *const u16, left: *const u16,
                                                                       bd: i32) {
    let sixteen = _mm_cvtsi32_si128(16);
    let sum = dc_sum_32(above);
    let dc = _mm_srli_epi32::<5>(_mm_add_epi32(sum, sixteen));
    let _ = left;
    let _ = bd;
    dc_store_32xh(dst, stride, 16, &dc);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_highbd_dc_128_predictor_32x16_sse2(dst: *mut u16, stride: isize,
                                                                       above: *const u16, left: *const u16,
                                                                       bd: i32) {
    let dc = _mm_cvtsi32_si128(1 << (bd - 1));
    let dc_dup = _mm_shufflelo_epi16::<0x0>(dc);
    let _ = above;
    let _ = left;
    dc_store_32xh(dst, stride, 16, &dc_dup);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_highbd_dc_top_predictor_32x32_sse2(dst: *mut u16, stride: isize,
                                                                       above: *const u16, left: *const u16,
                                                                       bd: i32) {
    let sixteen = _mm_cvtsi32_si128(16);
    let sum = dc_sum_32(above);
    let dc = _mm_srli_epi32::<5>(_mm_add_epi32(sum, sixteen));
    let _ = left;
    let _ = bd;
    dc_store_32xh(dst, stride, 32, &dc);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_highbd_dc_128_predictor_32x32_sse2(dst: *mut u16, stride: isize,
                                                                       above: *const u16, left: *const u16,
                                                                       bd: i32) {
    let dc = _mm_cvtsi32_si128(1 << (bd - 1));
    let dc_dup = _mm_shufflelo_epi16::<0x0>(dc);
    let _ = above;
    let _ = left;
    dc_store_32xh(dst, stride, 32, &dc_dup);
}

// -----------------------------------------------------------------------------
// V_PRED

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_highbd_v_predictor_4x8_sse2(mut dst: *mut u16, stride: isize,
                                                                above: *const u16, left: *const u16,
                                                                bd: i32) {
    let _ = left;
    let _ = bd;
    let above_u16 = _mm_loadl_epi64(above as *const __m128i);
    let mut i = 0;
    while i < 2 {
        _mm_storel_epi64(dst as *mut __m128i, above_u16);
        _mm_storel_epi64(dst.offset(stride) as *mut __m128i, above_u16);
        _mm_storel_epi64(dst.offset(2 * stride) as *mut __m128i, above_u16);
        _mm_storel_epi64(dst.offset(3 * stride) as *mut __m128i, above_u16);
        dst = dst.offset(stride << 2);
        i += 1;
    }
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_highbd_v_predictor_8x4_sse2(dst: *mut u16, stride: isize,
                                                                above: *const u16, left: *const u16,
                                                                bd: i32) {
    let _ = left;
    let _ = bd;
    let above_u16 = _mm_load_si128(above as *const __m128i);
    _mm_store_si128(dst as *mut __m128i, above_u16);
    _mm_store_si128(dst.offset(stride) as *mut __m128i, above_u16);
    _mm_store_si128(dst.offset(2 * stride) as *mut __m128i, above_u16);
    _mm_store_si128(dst.offset(3 * stride) as *mut __m128i, above_u16);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_highbd_v_predictor_8x16_sse2(mut dst: *mut u16, stride: isize,
                                                                 above: *const u16, left: *const u16,
                                                                 bd: i32) {
    let _ = left;
    let _ = bd;
    let above_u16 = _mm_load_si128(above as *const __m128i);
    let mut i = 0;
    while i < 4 {
        _mm_store_si128(dst as *mut __m128i, above_u16);
        _mm_store_si128(dst.offset(stride) as *mut __m128i, above_u16);
        _mm_store_si128(dst.offset(2 * stride) as *mut __m128i, above_u16);
        _mm_store_si128(dst.offset(3 * stride) as *mut __m128i, above_u16);
        dst = dst.offset(stride << 2);
        i += 1;
    }
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_highbd_v_predictor_16x8_sse2(mut dst: *mut u16, stride: isize,
                                                                 above: *const u16, left: *const u16,
                                                                 bd: i32) {
    let _ = left;
    let _ = bd;
    let above0_u16 = _mm_load_si128(above as *const __m128i);
    let above1_u16 = _mm_load_si128(above.add(8) as *const __m128i);
    let mut i = 0;
    while i < 2 {
        _mm_store_si128(dst as *mut __m128i, above0_u16);
        _mm_store_si128(dst.add(8) as *mut __m128i, above1_u16);
        dst = dst.offset(stride);
        _mm_store_si128(dst as *mut __m128i, above0_u16);
        _mm_store_si128(dst.add(8) as *mut __m128i, above1_u16);
        dst = dst.offset(stride);
        _mm_store_si128(dst as *mut __m128i, above0_u16);
        _mm_store_si128(dst.add(8) as *mut __m128i, above1_u16);
        dst = dst.offset(stride);
        _mm_store_si128(dst as *mut __m128i, above0_u16);
        _mm_store_si128(dst.add(8) as *mut __m128i, above1_u16);
        dst = dst.offset(stride);
        i += 1;
    }
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_highbd_v_predictor_16x32_sse2(mut dst: *mut u16, stride: isize,
                                                                  above: *const u16, left: *const u16,
                                                                  bd: i32) {
    let _ = left;
    let _ = bd;
    let above0_u16 = _mm_load_si128(above as *const __m128i);
    let above1_u16 = _mm_load_si128(above.add(8) as *const __m128i);
    let mut i = 0;
    while i < 8 {
        _mm_store_si128(dst as *mut __m128i, above0_u16);
        _mm_store_si128(dst.add(8) as *mut __m128i, above1_u16);
        dst = dst.offset(stride);
        _mm_store_si128(dst as *mut __m128i, above0_u16);
        _mm_store_si128(dst.add(8) as *mut __m128i, above1_u16);
        dst = dst.offset(stride);
        _mm_store_si128(dst as *mut __m128i, above0_u16);
        _mm_store_si128(dst.add(8) as *mut __m128i, above1_u16);
        dst = dst.offset(stride);
        _mm_store_si128(dst as *mut __m128i, above0_u16);
        _mm_store_si128(dst.add(8) as *mut __m128i, above1_u16);
        dst = dst.offset(stride);
        i += 1;
    }
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_highbd_v_predictor_32x16_sse2(mut dst: *mut u16, stride: isize,
                                                                  above: *const u16, left: *const u16,
                                                                  bd: i32) {
    let _ = left;
    let _ = bd;
    let above0_u16 = _mm_load_si128(above as *const __m128i);
    let above1_u16 = _mm_load_si128(above.add(8) as *const __m128i);
    let above2_u16 = _mm_load_si128(above.add(16) as *const __m128i);
    let above3_u16 = _mm_load_si128(above.add(24) as *const __m128i);
    let mut i = 0;
    while i < 4 {
        _mm_store_si128(dst as *mut __m128i, above0_u16);
        _mm_store_si128(dst.add(8) as *mut __m128i, above1_u16);
        _mm_store_si128(dst.add(16) as *mut __m128i, above2_u16);
        _mm_store_si128(dst.add(24) as *mut __m128i, above3_u16);
        dst = dst.offset(stride);
        _mm_store_si128(dst as *mut __m128i, above0_u16);
        _mm_store_si128(dst.add(8) as *mut __m128i, above1_u16);
        _mm_store_si128(dst.add(16) as *mut __m128i, above2_u16);
        _mm_store_si128(dst.add(24) as *mut __m128i, above3_u16);
        dst = dst.offset(stride);
        _mm_store_si128(dst as *mut __m128i, above0_u16);
        _mm_store_si128(dst.add(8) as *mut __m128i, above1_u16);
        _mm_store_si128(dst.add(16) as *mut __m128i, above2_u16);
        _mm_store_si128(dst.add(24) as *mut __m128i, above3_u16);
        dst = dst.offset(stride);
        _mm_store_si128(dst as *mut __m128i, above0_u16);
        _mm_store_si128(dst.add(8) as *mut __m128i, above1_u16);
        _mm_store_si128(dst.add(16) as *mut __m128i, above2_u16);
        _mm_store_si128(dst.add(24) as *mut __m128i, above3_u16);
        dst = dst.offset(stride);
        i += 1;
    }
}

// -----------------------------------------------------------------------------
// DC_PRED

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_highbd_dc_predictor_4x8_sse2(mut dst: *mut u16, stride: isize,
                                                                 above: *const u16, left: *const u16,
                                                                 bd: i32) {
    let _ = bd;
    let sum_above = dc_sum_4(above);
    let sum_left = dc_sum_8(left);
    let sum = _mm_add_epi16(sum_above, sum_left);
    let mut sum32: u32 = _mm_cvtsi128_si32(sum) as u32;
    sum32 >>= 16;
    sum32 += 6;
    sum32 /= 12;
    let row = _mm_set1_epi16(sum32 as i16);
    let mut i = 0;
    while i < 4 {
        _mm_storel_epi64(dst as *mut __m128i, row);
        dst = dst.offset(stride);
        _mm_storel_epi64(dst as *mut __m128i, row);
        dst = dst.offset(stride);
        i += 1;
    }
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_highbd_dc_predictor_8x4_sse2(mut dst: *mut u16, stride: isize,
                                                                 above: *const u16, left: *const u16,
                                                                 bd: i32) {
    let _ = bd;
    let sum_left = dc_sum_4(left);
    let sum_above = dc_sum_8(above);
    let sum = _mm_add_epi16(sum_above, sum_left);
    let mut sum32: u32 = _mm_cvtsi128_si32(sum) as u32;
    sum32 >>= 16;
    sum32 += 6;
    sum32 /= 12;
    let row = _mm_set1_epi16(sum32 as i16);

    _mm_store_si128(dst as *mut __m128i, row);
    dst = dst.offset(stride);
    _mm_store_si128(dst as *mut __m128i, row);
    dst = dst.offset(stride);
    _mm_store_si128(dst as *mut __m128i, row);
    dst = dst.offset(stride);
    _mm_store_si128(dst as *mut __m128i, row);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_highbd_dc_predictor_8x16_sse2(mut dst: *mut u16, stride: isize,
                                                                  above: *const u16, left: *const u16,
                                                                  bd: i32) {
    let _ = bd;
    let mut sum_left = dc_sum_16(left);
    let mut sum_above = dc_sum_8(above);
    let zero = _mm_setzero_si128();
    sum_left = _mm_unpacklo_epi16(sum_left, zero);
    sum_above = _mm_unpacklo_epi16(sum_above, zero);
    let sum = _mm_add_epi32(sum_left, sum_above);
    let mut sum32: u32 = _mm_cvtsi128_si32(sum) as u32;
    sum32 += 12;
    sum32 /= 24;
    let row = _mm_set1_epi16(sum32 as i16);
    let mut i = 0;
    while i < 4 {
        _mm_store_si128(dst as *mut __m128i, row);
        dst = dst.offset(stride);
        _mm_store_si128(dst as *mut __m128i, row);
        dst = dst.offset(stride);
        _mm_store_si128(dst as *mut __m128i, row);
        dst = dst.offset(stride);
        _mm_store_si128(dst as *mut __m128i, row);
        dst = dst.offset(stride);
        i += 1;
    }
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_highbd_dc_predictor_16x8_sse2(mut dst: *mut u16, stride: isize,
                                                                  above: *const u16, left: *const u16,
                                                                  bd: i32) {
    let _ = bd;
    let mut sum_left = dc_sum_8(left);
    let mut sum_above = dc_sum_16(above);
    let zero = _mm_setzero_si128();
    sum_left = _mm_unpacklo_epi16(sum_left, zero);
    sum_above = _mm_unpacklo_epi16(sum_above, zero);
    let sum = _mm_add_epi32(sum_left, sum_above);
    let mut sum32: u32 = _mm_cvtsi128_si32(sum) as u32;
    sum32 += 12;
    sum32 /= 24;
    let row = _mm_set1_epi16(sum32 as i16);
    let mut i = 0;
    while i < 2 {
        _mm_store_si128(dst as *mut __m128i, row);
        _mm_store_si128(dst.add(8) as *mut __m128i, row);
        dst = dst.offset(stride);
        _mm_store_si128(dst as *mut __m128i, row);
        _mm_store_si128(dst.add(8) as *mut __m128i, row);
        dst = dst.offset(stride);
        _mm_store_si128(dst as *mut __m128i, row);
        _mm_store_si128(dst.add(8) as *mut __m128i, row);
        dst = dst.offset(stride);
        _mm_store_si128(dst as *mut __m128i, row);
        _mm_store_si128(dst.add(8) as *mut __m128i, row);
        dst = dst.offset(stride);
        i += 1;
    }
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_highbd_dc_predictor_16x32_sse2(mut dst: *mut u16, stride: isize,
                                                                   above: *const u16, left: *const u16,
                                                                   bd: i32) {
    let _ = bd;
    let sum_left = dc_sum_32(left);
    let mut sum_above = dc_sum_16(above);
    let zero = _mm_setzero_si128();
    sum_above = _mm_unpacklo_epi16(sum_above, zero);
    let sum = _mm_add_epi32(sum_left, sum_above);
    let mut sum32: u32 = _mm_cvtsi128_si32(sum) as u32;
    sum32 += 24;
    sum32 /= 48;
    let row = _mm_set1_epi16(sum32 as i16);
    let mut i = 0;
    while i < 8 {
        _mm_store_si128(dst as *mut __m128i, row);
        _mm_store_si128(dst.add(8) as *mut __m128i, row);
        dst = dst.offset(stride);
        _mm_store_si128(dst as *mut __m128i, row);
        _mm_store_si128(dst.add(8) as *mut __m128i, row);
        dst = dst.offset(stride);
        _mm_store_si128(dst as *mut __m128i, row);
        _mm_store_si128(dst.add(8) as *mut __m128i, row);
        dst = dst.offset(stride);
        _mm_store_si128(dst as *mut __m128i, row);
        _mm_store_si128(dst.add(8) as *mut __m128i, row);
        dst = dst.offset(stride);
        i += 1;
    }
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_highbd_dc_predictor_32x16_sse2(mut dst: *mut u16, stride: isize,
                                                                   above: *const u16, left: *const u16,
                                                                   bd: i32) {
    let _ = bd;
    let mut sum_left = dc_sum_16(left);
    let sum_above = dc_sum_32(above);
    let zero = _mm_setzero_si128();
    sum_left = _mm_unpacklo_epi16(sum_left, zero);
    let sum = _mm_add_epi32(sum_left, sum_above);
    let mut sum32: u32 = _mm_cvtsi128_si32(sum) as u32;
    sum32 += 24;
    sum32 /= 48;
    let row = _mm_set1_epi16(sum32 as i16);
    let mut i = 0;
    while i < 4 {
        _mm_store_si128(dst as *mut __m128i, row);
        _mm_store_si128(dst.add(8) as *mut __m128i, row);
        _mm_store_si128(dst.add(16) as *mut __m128i, row);
        _mm_store_si128(dst.add(24) as *mut __m128i, row);
        dst = dst.offset(stride);
        _mm_store_si128(dst as *mut __m128i, row);
        _mm_store_si128(dst.add(8) as *mut __m128i, row);
        _mm_store_si128(dst.add(16) as *mut __m128i, row);
        _mm_store_si128(dst.add(24) as *mut __m128i, row);
        dst = dst.offset(stride);
        _mm_store_si128(dst as *mut __m128i, row);
        _mm_store_si128(dst.add(8) as *mut __m128i, row);
        _mm_store_si128(dst.add(16) as *mut __m128i, row);
        _mm_store_si128(dst.add(24) as *mut __m128i, row);
        dst = dst.offset(stride);
        _mm_store_si128(dst as *mut __m128i, row);
        _mm_store_si128(dst.add(8) as *mut __m128i, row);
        _mm_store_si128(dst.add(16) as *mut __m128i, row);
        _mm_store_si128(dst.add(24) as *mut __m128i, row);
        dst = dst.offset(stride);
        i += 1;
    }
}
