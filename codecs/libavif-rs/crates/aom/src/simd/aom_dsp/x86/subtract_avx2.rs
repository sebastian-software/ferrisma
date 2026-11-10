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

// Translated by hand from upstream/aom/aom_dsp/x86/subtract_avx2.c (ADR 0056,
// ADR 0058). Upstream compiles this file with -mavx2.
//
// For widths other than 16, 32, 64 and 128 the original falls back to
// aom_subtract_block_sse2(), which upstream implements only in NASM
// (aom_dsp/x86/subtract_sse2.asm) and the port therefore does not translate
// (ADR 0058). The translation falls back to aom_subtract_block_c() instead;
// both compute `src - pred` for every sample, so the output is identical.
use core::arch::x86_64::*;

extern "C-unwind" {
    fn aom_subtract_block_c(rows: i32, cols: i32, diff_ptr: *mut i16, diff_stride: isize,
                            src_ptr: *const u8, src_stride: isize, pred_ptr: *const u8,
                            pred_stride: isize);
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn subtract32_avx2(diff_ptr: *mut i16, src_ptr: *const u8, pred_ptr: *const u8) {
    let s = _mm256_lddqu_si256(src_ptr as *const __m256i);
    let p = _mm256_lddqu_si256(pred_ptr as *const __m256i);
    let set_one_minusone = _mm256_set1_epi32(0xff01ff01u32 as i32);
    let mut diff0 = _mm256_unpacklo_epi8(s, p);
    let mut diff1 = _mm256_unpackhi_epi8(s, p);
    diff0 = _mm256_maddubs_epi16(diff0, set_one_minusone);
    diff1 = _mm256_maddubs_epi16(diff1, set_one_minusone);
    _mm256_store_si256(diff_ptr as *mut __m256i,
                       _mm256_permute2x128_si256::<0x20>(diff0, diff1));
    _mm256_store_si256(diff_ptr.add(16) as *mut __m256i,
                       _mm256_permute2x128_si256::<0x31>(diff0, diff1));
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn subtract_block_16xn_avx2(rows: i32, mut diff_ptr: *mut i16, diff_stride: isize,
                                   mut src_ptr: *const u8, src_stride: isize,
                                   mut pred_ptr: *const u8, pred_stride: isize) {
    for _j in 0..rows {
        let s = _mm_lddqu_si128(src_ptr as *const __m128i);
        let p = _mm_lddqu_si128(pred_ptr as *const __m128i);
        let s_0 = _mm256_cvtepu8_epi16(s);
        let p_0 = _mm256_cvtepu8_epi16(p);
        let d_0 = _mm256_sub_epi16(s_0, p_0);
        _mm256_store_si256(diff_ptr as *mut __m256i, d_0);
        src_ptr = src_ptr.offset(src_stride);
        pred_ptr = pred_ptr.offset(pred_stride);
        diff_ptr = diff_ptr.offset(diff_stride);
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn subtract_block_32xn_avx2(rows: i32, mut diff_ptr: *mut i16, diff_stride: isize,
                                   mut src_ptr: *const u8, src_stride: isize,
                                   mut pred_ptr: *const u8, pred_stride: isize) {
    for _j in 0..rows {
        subtract32_avx2(diff_ptr, src_ptr, pred_ptr);
        src_ptr = src_ptr.offset(src_stride);
        pred_ptr = pred_ptr.offset(pred_stride);
        diff_ptr = diff_ptr.offset(diff_stride);
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn subtract_block_64xn_avx2(rows: i32, mut diff_ptr: *mut i16, diff_stride: isize,
                                   mut src_ptr: *const u8, src_stride: isize,
                                   mut pred_ptr: *const u8, pred_stride: isize) {
    for _j in 0..rows {
        subtract32_avx2(diff_ptr, src_ptr, pred_ptr);
        subtract32_avx2(diff_ptr.add(32), src_ptr.add(32), pred_ptr.add(32));
        src_ptr = src_ptr.offset(src_stride);
        pred_ptr = pred_ptr.offset(pred_stride);
        diff_ptr = diff_ptr.offset(diff_stride);
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn subtract_block_128xn_avx2(rows: i32, mut diff_ptr: *mut i16, diff_stride: isize,
                                    mut src_ptr: *const u8, src_stride: isize,
                                    mut pred_ptr: *const u8, pred_stride: isize) {
    for _j in 0..rows {
        subtract32_avx2(diff_ptr, src_ptr, pred_ptr);
        subtract32_avx2(diff_ptr.add(32), src_ptr.add(32), pred_ptr.add(32));
        subtract32_avx2(diff_ptr.add(64), src_ptr.add(64), pred_ptr.add(64));
        subtract32_avx2(diff_ptr.add(96), src_ptr.add(96), pred_ptr.add(96));
        src_ptr = src_ptr.offset(src_stride);
        pred_ptr = pred_ptr.offset(pred_stride);
        diff_ptr = diff_ptr.offset(diff_stride);
    }
}

#[no_mangle]
#[target_feature(enable = "avx2")]
#[allow(clippy::too_many_arguments)]
pub unsafe extern "C-unwind" fn aom_subtract_block_avx2(rows: i32, cols: i32, diff_ptr: *mut i16,
                                                         diff_stride: isize, src_ptr: *const u8,
                                                         src_stride: isize, pred_ptr: *const u8,
                                                         pred_stride: isize) {
    match cols {
        16 => subtract_block_16xn_avx2(rows, diff_ptr, diff_stride, src_ptr, src_stride,
                                       pred_ptr, pred_stride),
        32 => subtract_block_32xn_avx2(rows, diff_ptr, diff_stride, src_ptr, src_stride,
                                       pred_ptr, pred_stride),
        64 => subtract_block_64xn_avx2(rows, diff_ptr, diff_stride, src_ptr, src_stride,
                                       pred_ptr, pred_stride),
        128 => subtract_block_128xn_avx2(rows, diff_ptr, diff_stride, src_ptr, src_stride,
                                         pred_ptr, pred_stride),
        // Upstream: aom_subtract_block_sse2() (NASM only, see above).
        _ => aom_subtract_block_c(rows, cols, diff_ptr, diff_stride, src_ptr, src_stride,
                                  pred_ptr, pred_stride),
    }
}
