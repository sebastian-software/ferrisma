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

// Translated by hand from upstream/aom/aom_dsp/x86/highbd_subtract_sse2.c
// (ADR 0056, ADR 0058). Upstream compiles this file with -msse2.
use core::arch::x86_64::*;

type SubtractWxHFuncType = unsafe fn(diff: *mut i16, diff_stride: isize, src: *const u16,
                                     src_stride: isize, pred: *const u16, pred_stride: isize);

// aom_ports/mem.h
#[inline]
fn CONVERT_TO_SHORTPTR(x: *const u8) -> *mut u16 {
    ((x as usize) << 1) as *mut u16
}

#[target_feature(enable = "sse2")]
unsafe fn subtract_4x4(diff: *mut i16, diff_stride: isize, src: *const u16, src_stride: isize,
                       pred: *const u16, pred_stride: isize) {
    let u0: __m128i;
    let u1: __m128i;
    let u2: __m128i;
    let u3: __m128i;
    let v0: __m128i;
    let v1: __m128i;
    let v2: __m128i;
    let v3: __m128i;
    let x0: __m128i;
    let x1: __m128i;
    let x2: __m128i;
    let x3: __m128i;
    let mut store_diff = diff.offset(0 * diff_stride) as *mut i64;

    u0 = _mm_loadl_epi64(src.offset(0 * src_stride) as *const __m128i);
    u1 = _mm_loadl_epi64(src.offset(1 * src_stride) as *const __m128i);
    u2 = _mm_loadl_epi64(src.offset(2 * src_stride) as *const __m128i);
    u3 = _mm_loadl_epi64(src.offset(3 * src_stride) as *const __m128i);

    v0 = _mm_loadl_epi64(pred.offset(0 * pred_stride) as *const __m128i);
    v1 = _mm_loadl_epi64(pred.offset(1 * pred_stride) as *const __m128i);
    v2 = _mm_loadl_epi64(pred.offset(2 * pred_stride) as *const __m128i);
    v3 = _mm_loadl_epi64(pred.offset(3 * pred_stride) as *const __m128i);

    x0 = _mm_sub_epi16(u0, v0);
    x1 = _mm_sub_epi16(u1, v1);
    x2 = _mm_sub_epi16(u2, v2);
    x3 = _mm_sub_epi16(u3, v3);

    _mm_storel_epi64(store_diff as *mut __m128i, x0);
    store_diff = diff.offset(1 * diff_stride) as *mut i64;
    _mm_storel_epi64(store_diff as *mut __m128i, x1);
    store_diff = diff.offset(2 * diff_stride) as *mut i64;
    _mm_storel_epi64(store_diff as *mut __m128i, x2);
    store_diff = diff.offset(3 * diff_stride) as *mut i64;
    _mm_storel_epi64(store_diff as *mut __m128i, x3);
}

#[target_feature(enable = "sse2")]
unsafe fn subtract_4x8(diff: *mut i16, diff_stride: isize, src: *const u16, src_stride: isize,
                       pred: *const u16, pred_stride: isize) {
    let u0: __m128i;
    let u1: __m128i;
    let u2: __m128i;
    let u3: __m128i;
    let u4: __m128i;
    let u5: __m128i;
    let u6: __m128i;
    let u7: __m128i;
    let v0: __m128i;
    let v1: __m128i;
    let v2: __m128i;
    let v3: __m128i;
    let v4: __m128i;
    let v5: __m128i;
    let v6: __m128i;
    let v7: __m128i;
    let x0: __m128i;
    let x1: __m128i;
    let x2: __m128i;
    let x3: __m128i;
    let x4: __m128i;
    let x5: __m128i;
    let x6: __m128i;
    let x7: __m128i;
    let mut store_diff = diff.offset(0 * diff_stride) as *mut i64;

    u0 = _mm_loadl_epi64(src.offset(0 * src_stride) as *const __m128i);
    u1 = _mm_loadl_epi64(src.offset(1 * src_stride) as *const __m128i);
    u2 = _mm_loadl_epi64(src.offset(2 * src_stride) as *const __m128i);
    u3 = _mm_loadl_epi64(src.offset(3 * src_stride) as *const __m128i);
    u4 = _mm_loadl_epi64(src.offset(4 * src_stride) as *const __m128i);
    u5 = _mm_loadl_epi64(src.offset(5 * src_stride) as *const __m128i);
    u6 = _mm_loadl_epi64(src.offset(6 * src_stride) as *const __m128i);
    u7 = _mm_loadl_epi64(src.offset(7 * src_stride) as *const __m128i);

    v0 = _mm_loadl_epi64(pred.offset(0 * pred_stride) as *const __m128i);
    v1 = _mm_loadl_epi64(pred.offset(1 * pred_stride) as *const __m128i);
    v2 = _mm_loadl_epi64(pred.offset(2 * pred_stride) as *const __m128i);
    v3 = _mm_loadl_epi64(pred.offset(3 * pred_stride) as *const __m128i);
    v4 = _mm_loadl_epi64(pred.offset(4 * pred_stride) as *const __m128i);
    v5 = _mm_loadl_epi64(pred.offset(5 * pred_stride) as *const __m128i);
    v6 = _mm_loadl_epi64(pred.offset(6 * pred_stride) as *const __m128i);
    v7 = _mm_loadl_epi64(pred.offset(7 * pred_stride) as *const __m128i);

    x0 = _mm_sub_epi16(u0, v0);
    x1 = _mm_sub_epi16(u1, v1);
    x2 = _mm_sub_epi16(u2, v2);
    x3 = _mm_sub_epi16(u3, v3);
    x4 = _mm_sub_epi16(u4, v4);
    x5 = _mm_sub_epi16(u5, v5);
    x6 = _mm_sub_epi16(u6, v6);
    x7 = _mm_sub_epi16(u7, v7);

    _mm_storel_epi64(store_diff as *mut __m128i, x0);
    store_diff = diff.offset(1 * diff_stride) as *mut i64;
    _mm_storel_epi64(store_diff as *mut __m128i, x1);
    store_diff = diff.offset(2 * diff_stride) as *mut i64;
    _mm_storel_epi64(store_diff as *mut __m128i, x2);
    store_diff = diff.offset(3 * diff_stride) as *mut i64;
    _mm_storel_epi64(store_diff as *mut __m128i, x3);
    store_diff = diff.offset(4 * diff_stride) as *mut i64;
    _mm_storel_epi64(store_diff as *mut __m128i, x4);
    store_diff = diff.offset(5 * diff_stride) as *mut i64;
    _mm_storel_epi64(store_diff as *mut __m128i, x5);
    store_diff = diff.offset(6 * diff_stride) as *mut i64;
    _mm_storel_epi64(store_diff as *mut __m128i, x6);
    store_diff = diff.offset(7 * diff_stride) as *mut i64;
    _mm_storel_epi64(store_diff as *mut __m128i, x7);
}

#[target_feature(enable = "sse2")]
unsafe fn subtract_8x4(diff: *mut i16, diff_stride: isize, src: *const u16, src_stride: isize,
                       pred: *const u16, pred_stride: isize) {
    let u0: __m128i;
    let u1: __m128i;
    let u2: __m128i;
    let u3: __m128i;
    let v0: __m128i;
    let v1: __m128i;
    let v2: __m128i;
    let v3: __m128i;
    let x0: __m128i;
    let x1: __m128i;
    let x2: __m128i;
    let x3: __m128i;

    u0 = _mm_loadu_si128(src.offset(0 * src_stride) as *const __m128i);
    u1 = _mm_loadu_si128(src.offset(1 * src_stride) as *const __m128i);
    u2 = _mm_loadu_si128(src.offset(2 * src_stride) as *const __m128i);
    u3 = _mm_loadu_si128(src.offset(3 * src_stride) as *const __m128i);

    v0 = _mm_loadu_si128(pred.offset(0 * pred_stride) as *const __m128i);
    v1 = _mm_loadu_si128(pred.offset(1 * pred_stride) as *const __m128i);
    v2 = _mm_loadu_si128(pred.offset(2 * pred_stride) as *const __m128i);
    v3 = _mm_loadu_si128(pred.offset(3 * pred_stride) as *const __m128i);

    x0 = _mm_sub_epi16(u0, v0);
    x1 = _mm_sub_epi16(u1, v1);
    x2 = _mm_sub_epi16(u2, v2);
    x3 = _mm_sub_epi16(u3, v3);

    _mm_storeu_si128(diff.offset(0 * diff_stride) as *mut __m128i, x0);
    _mm_storeu_si128(diff.offset(1 * diff_stride) as *mut __m128i, x1);
    _mm_storeu_si128(diff.offset(2 * diff_stride) as *mut __m128i, x2);
    _mm_storeu_si128(diff.offset(3 * diff_stride) as *mut __m128i, x3);
}

#[target_feature(enable = "sse2")]
unsafe fn subtract_8x8(diff: *mut i16, diff_stride: isize, src: *const u16, src_stride: isize,
                       pred: *const u16, pred_stride: isize) {
    let u0: __m128i;
    let u1: __m128i;
    let u2: __m128i;
    let u3: __m128i;
    let u4: __m128i;
    let u5: __m128i;
    let u6: __m128i;
    let u7: __m128i;
    let v0: __m128i;
    let v1: __m128i;
    let v2: __m128i;
    let v3: __m128i;
    let v4: __m128i;
    let v5: __m128i;
    let v6: __m128i;
    let v7: __m128i;
    let x0: __m128i;
    let x1: __m128i;
    let x2: __m128i;
    let x3: __m128i;
    let x4: __m128i;
    let x5: __m128i;
    let x6: __m128i;
    let x7: __m128i;

    u0 = _mm_loadu_si128(src.offset(0 * src_stride) as *const __m128i);
    u1 = _mm_loadu_si128(src.offset(1 * src_stride) as *const __m128i);
    u2 = _mm_loadu_si128(src.offset(2 * src_stride) as *const __m128i);
    u3 = _mm_loadu_si128(src.offset(3 * src_stride) as *const __m128i);
    u4 = _mm_loadu_si128(src.offset(4 * src_stride) as *const __m128i);
    u5 = _mm_loadu_si128(src.offset(5 * src_stride) as *const __m128i);
    u6 = _mm_loadu_si128(src.offset(6 * src_stride) as *const __m128i);
    u7 = _mm_loadu_si128(src.offset(7 * src_stride) as *const __m128i);

    v0 = _mm_loadu_si128(pred.offset(0 * pred_stride) as *const __m128i);
    v1 = _mm_loadu_si128(pred.offset(1 * pred_stride) as *const __m128i);
    v2 = _mm_loadu_si128(pred.offset(2 * pred_stride) as *const __m128i);
    v3 = _mm_loadu_si128(pred.offset(3 * pred_stride) as *const __m128i);
    v4 = _mm_loadu_si128(pred.offset(4 * pred_stride) as *const __m128i);
    v5 = _mm_loadu_si128(pred.offset(5 * pred_stride) as *const __m128i);
    v6 = _mm_loadu_si128(pred.offset(6 * pred_stride) as *const __m128i);
    v7 = _mm_loadu_si128(pred.offset(7 * pred_stride) as *const __m128i);

    x0 = _mm_sub_epi16(u0, v0);
    x1 = _mm_sub_epi16(u1, v1);
    x2 = _mm_sub_epi16(u2, v2);
    x3 = _mm_sub_epi16(u3, v3);
    x4 = _mm_sub_epi16(u4, v4);
    x5 = _mm_sub_epi16(u5, v5);
    x6 = _mm_sub_epi16(u6, v6);
    x7 = _mm_sub_epi16(u7, v7);

    _mm_storeu_si128(diff.offset(0 * diff_stride) as *mut __m128i, x0);
    _mm_storeu_si128(diff.offset(1 * diff_stride) as *mut __m128i, x1);
    _mm_storeu_si128(diff.offset(2 * diff_stride) as *mut __m128i, x2);
    _mm_storeu_si128(diff.offset(3 * diff_stride) as *mut __m128i, x3);
    _mm_storeu_si128(diff.offset(4 * diff_stride) as *mut __m128i, x4);
    _mm_storeu_si128(diff.offset(5 * diff_stride) as *mut __m128i, x5);
    _mm_storeu_si128(diff.offset(6 * diff_stride) as *mut __m128i, x6);
    _mm_storeu_si128(diff.offset(7 * diff_stride) as *mut __m128i, x7);
}

// SUBTRACT_FUN(size) { STACK_V(h, fun); } and SUBTRACT_FUN(size) { STACK_H(w, fun); }
macro_rules! SUBTRACT_FUN {
    ($name:ident { STACK_V($h:literal, $fun:ident) }) => {
        #[target_feature(enable = "sse2")]
        unsafe fn $name(diff: *mut i16, diff_stride: isize, src: *const u16, src_stride: isize,
                        pred: *const u16, pred_stride: isize) {
            $fun(diff, diff_stride, src, src_stride, pred, pred_stride);
            $fun(diff.offset(diff_stride * $h), diff_stride, src.offset(src_stride * $h),
                 src_stride, pred.offset(pred_stride * $h), pred_stride);
        }
    };
    ($name:ident { STACK_H($w:literal, $fun:ident) }) => {
        #[target_feature(enable = "sse2")]
        unsafe fn $name(diff: *mut i16, diff_stride: isize, src: *const u16, src_stride: isize,
                        pred: *const u16, pred_stride: isize) {
            $fun(diff, diff_stride, src, src_stride, pred, pred_stride);
            $fun(diff.add($w), diff_stride, src.add($w), src_stride, pred.add($w), pred_stride);
        }
    };
}

SUBTRACT_FUN!(subtract_8x16 { STACK_V(8, subtract_8x8) });
SUBTRACT_FUN!(subtract_16x8 { STACK_H(8, subtract_8x8) });
SUBTRACT_FUN!(subtract_16x16 { STACK_V(8, subtract_16x8) });
SUBTRACT_FUN!(subtract_16x32 { STACK_V(16, subtract_16x16) });
SUBTRACT_FUN!(subtract_32x16 { STACK_H(16, subtract_16x16) });
SUBTRACT_FUN!(subtract_32x32 { STACK_V(16, subtract_32x16) });
SUBTRACT_FUN!(subtract_32x64 { STACK_V(32, subtract_32x32) });
SUBTRACT_FUN!(subtract_64x32 { STACK_H(32, subtract_32x32) });
SUBTRACT_FUN!(subtract_64x64 { STACK_V(32, subtract_64x32) });
SUBTRACT_FUN!(subtract_64x128 { STACK_V(64, subtract_64x64) });
SUBTRACT_FUN!(subtract_128x64 { STACK_H(64, subtract_64x64) });
SUBTRACT_FUN!(subtract_128x128 { STACK_V(64, subtract_128x64) });
SUBTRACT_FUN!(subtract_4x16 { STACK_V(8, subtract_4x8) });
SUBTRACT_FUN!(subtract_16x4 { STACK_H(8, subtract_8x4) });
SUBTRACT_FUN!(subtract_8x32 { STACK_V(16, subtract_8x16) });
SUBTRACT_FUN!(subtract_32x8 { STACK_H(16, subtract_16x8) });
SUBTRACT_FUN!(subtract_16x64 { STACK_V(32, subtract_16x32) });
SUBTRACT_FUN!(subtract_64x16 { STACK_H(32, subtract_32x16) });

#[target_feature(enable = "sse2")]
unsafe fn getSubtractFunc(rows: i32, cols: i32) -> Option<SubtractWxHFuncType> {
    if rows == 4 {
        if cols == 4 { return Some(subtract_4x4); }
        if cols == 8 { return Some(subtract_8x4); }
        if cols == 16 { return Some(subtract_16x4); }
    }
    if rows == 8 {
        if cols == 4 { return Some(subtract_4x8); }
        if cols == 8 { return Some(subtract_8x8); }
        if cols == 16 { return Some(subtract_16x8); }
        if cols == 32 { return Some(subtract_32x8); }
    }
    if rows == 16 {
        if cols == 4 { return Some(subtract_4x16); }
        if cols == 8 { return Some(subtract_8x16); }
        if cols == 16 { return Some(subtract_16x16); }
        if cols == 32 { return Some(subtract_32x16); }
        if cols == 64 { return Some(subtract_64x16); }
    }
    if rows == 32 {
        if cols == 8 { return Some(subtract_8x32); }
        if cols == 16 { return Some(subtract_16x32); }
        if cols == 32 { return Some(subtract_32x32); }
        if cols == 64 { return Some(subtract_64x32); }
    }
    if rows == 64 {
        if cols == 16 { return Some(subtract_16x64); }
        if cols == 32 { return Some(subtract_32x64); }
        if cols == 64 { return Some(subtract_64x64); }
        if cols == 128 { return Some(subtract_128x64); }
    }
    if rows == 128 {
        if cols == 64 { return Some(subtract_64x128); }
        if cols == 128 { return Some(subtract_128x128); }
    }
    debug_assert!(false);
    None
}

#[no_mangle]
#[target_feature(enable = "sse2")]
#[allow(clippy::too_many_arguments)]
pub unsafe extern "C-unwind" fn aom_highbd_subtract_block_sse2(rows: i32, cols: i32,
                                                                diff: *mut i16,
                                                                diff_stride: isize,
                                                                src8: *const u8,
                                                                src_stride: isize,
                                                                pred8: *const u8,
                                                                pred_stride: isize) {
    let src: *mut u16 = CONVERT_TO_SHORTPTR(src8);
    let pred: *mut u16 = CONVERT_TO_SHORTPTR(pred8);
    let func: SubtractWxHFuncType;

    // The original calls through the NULL pointer that getSubtractFunc()
    // returns for an unsupported size; the translation panics instead.
    func = getSubtractFunc(rows, cols).expect("unsupported block size");
    func(diff, diff_stride, src, src_stride, pred, pred_stride);
}
