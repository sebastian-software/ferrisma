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

// Translated by hand from upstream/aom/aom_dsp/x86/txfm_common_sse2.h (ADR
// 0056, ADR 0058). Names, order, and intrinsic sequences follow the original;
// every helper carries the smallest target feature its own intrinsics need
// (SSE2).
//
// The function-like macros pair_set_epi16 and octa_set_epi16 become functions
// with `i32` parameters (the type of the int expressions every caller passes);
// the casts inside are those of the macros, so a narrower argument widened
// with `as i32` produces the same lanes.

use core::arch::x86_64::*;

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn pair_set_epi16(a: i32, b: i32) -> __m128i {
    _mm_set1_epi32((((a as u16) as u32) | (((b as u16) as u32) << 16)) as i32)
}

// Reverse the 8 16 bit words in __m128i
#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn mm_reverse_epi16(x: __m128i) -> __m128i {
    let a = _mm_shufflelo_epi16::<0x1b>(x);
    let b = _mm_shufflehi_epi16::<0x1b>(a);
    _mm_shuffle_epi32::<0x4e>(b)
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn octa_set_epi16(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32,
                             h: i32) -> __m128i {
    _mm_setr_epi16(a as i16, b as i16, c as i16, d as i16, e as i16, f as i16, g as i16,
                   h as i16)
}
