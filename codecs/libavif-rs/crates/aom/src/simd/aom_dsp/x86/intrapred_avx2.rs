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

// Translated by hand from upstream/aom/aom_dsp/x86/intrapred_avx2.c (ADR 0056,
// ADR 0058). Upstream compiles this file with -mavx2.
//
// The high-bitdepth directional predictors (av1_highbd_dr_prediction_z1_avx2,
// av1_highbd_dr_prediction_z2_avx2 and av1_highbd_dr_prediction_z3_avx2, with
// the CONFIG_AV1_HIGHBITDEPTH tables and transposes and the static helpers they
// call) were translated by hand first. The low-bitdepth DC, V, H, Paeth, smooth
// and directional predictors that follow them were
// transliterated with a typing-aware C-subset printer (not C2Rust) and
// reviewed against the C function by function: the same sequence of intrinsic
// and helper calls, constants and operators in order (ADR 0058), with the notes of the other x86 files below. The
// `!CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER` branches are taken (the port
// builds the encoder and decoder with CONFIG_REALTIME_ONLY=0).
//
// The high-bitdepth predictors use no helper from intrapred_x86.h or
// intrapred_utils.h (the tables and transposes there are low-bitdepth); the
// 8-bit ones use them from crate::simd::aom_dsp__x86__intrapred_x86 and
// crate::simd::aom_dsp__x86__intrapred_utils. The transposes from
// lpf_common_sse2.h are in crate::simd::aom_dsp__x86__lpf_common_sse2.
//
// Translation notes:
// - Every function carries #[target_feature(enable = "avx2")], the flag of
//   this file, including the static inline transposes defined here.
//   AOM_FORCE_INLINE and `static inline` become #[inline].
// - The tables keep their DECLARE_ALIGNED alignment through Aligned16 /
//   Aligned32 wrappers, and `*(__m128i *)Table[i]` / `*(__m256i *)Table[i]`
//   stay aligned vector reads of the same rows. The aligned local arrays
//   (`base_y_c`, `dstT`) use the same wrappers.
// - Immediate shift counts and lane indices become const generics. Where C
//   passes a run-time `int` count (upsample_above, upsample_left, frac_bits_y
//   of the Nx4 and Nx8 zone 2 predictors) to _mm*_slli_epi* / _mm*_srai_epi*,
//   GCC and Clang lower it to the count-register form of the same instruction,
//   and the translation calls that form directly
//   (`_mm256_sll_epi16(v, _mm_cvtsi32_si128(n))`, ...). Both forms behave the
//   same for every count. `frac_bits_y` of the HxW zone 2 predictors is a
//   `const int` in C and stays an immediate.
// - The `int` arguments of _mm*_set*_epi16 are converted to `short` in C;
//   the translation spells that conversion `as i16` (modular, as GCC and
//   Clang define it). uint16_t edge pixels promoted to `int` for
//   _mm_setr_epi32 become `as i32`.
// - Local vector arrays that C leaves uninitialized are zero-initialized;
//   every element the C reads is written first, so the results are unchanged.
// - Array parameters stay raw pointers (`dst[i]` -> `*dst.add(i)`), and
//   `above + base` style addresses with negative offsets use `offset`.
#![allow(
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut,
    unused_parens
)]
#![allow(unused_variables, dead_code, clippy::all)]

use core::arch::x86_64::*;

use crate::simd::aom_dsp__x86__intrapred_utils::{
    transpose, transpose16x16_sse2, transpose4x16_sse2, transpose_TX_16X16, EvenOddMaskx,
    LoadMaskx, LoadMaskz2,
};
use crate::simd::aom_dsp__x86__intrapred_x86::*;
use crate::simd::aom_dsp__x86__lpf_common_sse2::*;
use crate::simd::aom_dsp__x86__synonyms::*;

#[repr(C, align(16))]
struct Aligned16<T>(T);

#[repr(C, align(32))]
struct Aligned32<T>(T);

// CONFIG_AV1_HIGHBITDEPTH
#[rustfmt::skip]
static HighbdLoadMaskx: Aligned16<[[u8; 16]; 8]> = Aligned16([
    [ 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15 ],
    [ 0, 1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13 ],
    [ 0, 1, 0, 1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11 ],
    [ 0, 1, 0, 1, 0, 1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9 ],
    [ 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 2, 3, 4, 5, 6, 7 ],
    [ 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 2, 3, 4, 5 ],
    [ 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 2, 3 ],
    [ 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1 ],
]);

#[rustfmt::skip]
static HighbdEvenOddMaskx4: Aligned16<[[u8; 16]; 4]> = Aligned16([
    [ 0, 1, 4, 5, 8, 9, 12, 13, 2, 3, 6, 7, 10, 11, 14, 15 ],
    [ 0, 1, 2, 3, 6, 7, 10, 11, 14, 15, 4, 5, 8, 9, 12, 13 ],
    [ 0, 1, 0, 1, 4, 5, 8, 9, 12, 13, 0, 1, 6, 7, 10, 11 ],
    [ 0, 1, 0, 1, 0, 1, 6, 7, 10, 11, 14, 15, 0, 1, 8, 9 ],
]);

#[rustfmt::skip]
static HighbdEvenOddMaskx: Aligned16<[[u8; 32]; 8]> = Aligned16([
    [ 0, 1, 4, 5, 8,  9,  12, 13, 16, 17, 20, 21, 24, 25, 28, 29,
      2, 3, 6, 7, 10, 11, 14, 15, 18, 19, 22, 23, 26, 27, 30, 31 ],
    [ 0, 1, 2, 3, 6, 7, 10, 11, 14, 15, 18, 19, 22, 23, 26, 27,
      0, 1, 4, 5, 8, 9, 12, 13, 16, 17, 20, 21, 24, 25, 28, 29 ],
    [ 0, 1, 0, 1, 4, 5, 8,  9,  12, 13, 16, 17, 20, 21, 24, 25,
      0, 1, 0, 1, 6, 7, 10, 11, 14, 15, 18, 19, 22, 23, 26, 27 ],
    [ 0, 1, 0, 1, 0, 1, 6, 7, 10, 11, 14, 15, 18, 19, 22, 23,
      0, 1, 0, 1, 0, 1, 8, 9, 12, 13, 16, 17, 20, 21, 24, 25 ],
    [ 0, 1, 0, 1, 0, 1, 0, 1, 8,  9,  12, 13, 16, 17, 20, 21,
      0, 1, 0, 1, 0, 1, 0, 1, 10, 11, 14, 15, 18, 19, 22, 23 ],
    [ 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 10, 11, 14, 15, 18, 19,
      0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 12, 13, 16, 17, 20, 21 ],
    [ 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 12, 13, 16, 17,
      0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 14, 15, 18, 19 ],
    [ 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 14, 15,
      0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 16, 17 ],
]);

#[rustfmt::skip]
static HighbdBaseMask: Aligned32<[[u16; 16]; 17]> = Aligned32([
    [ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
    [ 0xffff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
    [ 0xffff, 0xffff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
    [ 0xffff, 0xffff, 0xffff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
    [ 0xffff, 0xffff, 0xffff, 0xffff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
    [ 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
    [ 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0 ],
    [ 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0, 0, 0, 0, 0, 0, 0,
      0, 0 ],
    [ 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0, 0, 0, 0,
      0, 0, 0, 0 ],
    [ 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0,
      0, 0, 0, 0, 0, 0 ],
    [ 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff,
      0xffff, 0, 0, 0, 0, 0, 0 ],
    [ 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff,
      0xffff, 0xffff, 0, 0, 0, 0, 0 ],
    [ 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff,
      0xffff, 0xffff, 0xffff, 0, 0, 0, 0 ],
    [ 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff,
      0xffff, 0xffff, 0xffff, 0xffff, 0, 0, 0 ],
    [ 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff,
      0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0, 0 ],
    [ 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff,
      0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0 ],
    [ 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff,
      0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff ],
]);

// `*(__m128i *)HighbdLoadMaskx[i]` and the like: aligned reads of a table row.
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn load_mask128(row: *const u8) -> __m128i {
    *(row as *const __m128i)
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn base_mask128(base_min_diff: i32) -> __m128i {
    *(HighbdBaseMask.0[base_min_diff as usize].as_ptr() as *const __m128i)
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn base_mask256(base_min_diff: i32) -> __m256i {
    *(HighbdBaseMask.0[base_min_diff as usize].as_ptr() as *const __m256i)
}

// !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn highbd_transpose16x4_8x8_sse2(x: *const __m128i, d: *mut __m128i) {
    let mut r0: __m128i;
    let mut r1: __m128i;
    let mut r2: __m128i;
    let mut r3: __m128i;
    let mut r4: __m128i;
    let mut r5: __m128i;
    let mut r6: __m128i;
    let mut r7: __m128i;
    let r8: __m128i;
    let r9: __m128i;
    let r10: __m128i;
    let r11: __m128i;
    let r12: __m128i;
    let r13: __m128i;
    let r14: __m128i;
    let r15: __m128i;

    r0 = _mm_unpacklo_epi16(*x.add(0), *x.add(1));
    r1 = _mm_unpacklo_epi16(*x.add(2), *x.add(3));
    r2 = _mm_unpacklo_epi16(*x.add(4), *x.add(5));
    r3 = _mm_unpacklo_epi16(*x.add(6), *x.add(7));

    r4 = _mm_unpacklo_epi16(*x.add(8), *x.add(9));
    r5 = _mm_unpacklo_epi16(*x.add(10), *x.add(11));
    r6 = _mm_unpacklo_epi16(*x.add(12), *x.add(13));
    r7 = _mm_unpacklo_epi16(*x.add(14), *x.add(15));

    r8 = _mm_unpacklo_epi32(r0, r1);
    r9 = _mm_unpackhi_epi32(r0, r1);
    r10 = _mm_unpacklo_epi32(r2, r3);
    r11 = _mm_unpackhi_epi32(r2, r3);

    r12 = _mm_unpacklo_epi32(r4, r5);
    r13 = _mm_unpackhi_epi32(r4, r5);
    r14 = _mm_unpacklo_epi32(r6, r7);
    r15 = _mm_unpackhi_epi32(r6, r7);

    r0 = _mm_unpacklo_epi64(r8, r9);
    r1 = _mm_unpackhi_epi64(r8, r9);
    r2 = _mm_unpacklo_epi64(r10, r11);
    r3 = _mm_unpackhi_epi64(r10, r11);

    r4 = _mm_unpacklo_epi64(r12, r13);
    r5 = _mm_unpackhi_epi64(r12, r13);
    r6 = _mm_unpacklo_epi64(r14, r15);
    r7 = _mm_unpackhi_epi64(r14, r15);

    *d.add(0) = _mm_unpacklo_epi64(r0, r2);
    *d.add(1) = _mm_unpacklo_epi64(r4, r6);
    *d.add(2) = _mm_unpacklo_epi64(r1, r3);
    *d.add(3) = _mm_unpacklo_epi64(r5, r7);

    *d.add(4) = _mm_unpackhi_epi64(r0, r2);
    *d.add(5) = _mm_unpackhi_epi64(r4, r6);
    *d.add(6) = _mm_unpackhi_epi64(r1, r3);
    *d.add(7) = _mm_unpackhi_epi64(r5, r7);
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn highbd_transpose4x16_avx2(x: *const __m256i, d: *mut __m256i) {
    let w0: __m256i;
    let w1: __m256i;
    let w2: __m256i;
    let w3: __m256i;
    let mut ww0: __m256i;
    let mut ww1: __m256i;

    w0 = _mm256_unpacklo_epi16(*x.add(0), *x.add(1)); // 00 10 01 11 02 12 03 13
    w1 = _mm256_unpacklo_epi16(*x.add(2), *x.add(3)); // 20 30 21 31 22 32 23 33
    w2 = _mm256_unpackhi_epi16(*x.add(0), *x.add(1)); // 40 50 41 51 42 52 43 53
    w3 = _mm256_unpackhi_epi16(*x.add(2), *x.add(3)); // 60 70 61 71 62 72 63 73

    ww0 = _mm256_unpacklo_epi32(w0, w1); // 00 10 20 30 01 11 21 31
    ww1 = _mm256_unpacklo_epi32(w2, w3); // 40 50 60 70 41 51 61 71

    *d.add(0) = _mm256_unpacklo_epi64(ww0, ww1); // 00 10 20 30 40 50 60 70
    *d.add(1) = _mm256_unpackhi_epi64(ww0, ww1); // 01 11 21 31 41 51 61 71

    ww0 = _mm256_unpackhi_epi32(w0, w1); // 02 12 22 32 03 13 23 33
    ww1 = _mm256_unpackhi_epi32(w2, w3); // 42 52 62 72 43 53 63 73

    *d.add(2) = _mm256_unpacklo_epi64(ww0, ww1); // 02 12 22 32 42 52 62 72
    *d.add(3) = _mm256_unpackhi_epi64(ww0, ww1); // 03 13 23 33 43 53 63 73
}
// !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn highbd_transpose8x16_16x8_avx2(x: *const __m256i, d: *mut __m256i) {
    let mut w0: __m256i;
    let mut w1: __m256i;
    let mut w2: __m256i;
    let mut w3: __m256i;
    let mut ww0: __m256i;
    let mut ww1: __m256i;

    w0 = _mm256_unpacklo_epi16(*x.add(0), *x.add(1)); // 00 10 01 11 02 12 03 13
    w1 = _mm256_unpacklo_epi16(*x.add(2), *x.add(3)); // 20 30 21 31 22 32 23 33
    w2 = _mm256_unpacklo_epi16(*x.add(4), *x.add(5)); // 40 50 41 51 42 52 43 53
    w3 = _mm256_unpacklo_epi16(*x.add(6), *x.add(7)); // 60 70 61 71 62 72 63 73

    ww0 = _mm256_unpacklo_epi32(w0, w1); // 00 10 20 30 01 11 21 31
    ww1 = _mm256_unpacklo_epi32(w2, w3); // 40 50 60 70 41 51 61 71

    *d.add(0) = _mm256_unpacklo_epi64(ww0, ww1); // 00 10 20 30 40 50 60 70
    *d.add(1) = _mm256_unpackhi_epi64(ww0, ww1); // 01 11 21 31 41 51 61 71

    ww0 = _mm256_unpackhi_epi32(w0, w1); // 02 12 22 32 03 13 23 33
    ww1 = _mm256_unpackhi_epi32(w2, w3); // 42 52 62 72 43 53 63 73

    *d.add(2) = _mm256_unpacklo_epi64(ww0, ww1); // 02 12 22 32 42 52 62 72
    *d.add(3) = _mm256_unpackhi_epi64(ww0, ww1); // 03 13 23 33 43 53 63 73

    w0 = _mm256_unpackhi_epi16(*x.add(0), *x.add(1)); // 04 14 05 15 06 16 07 17
    w1 = _mm256_unpackhi_epi16(*x.add(2), *x.add(3)); // 24 34 25 35 26 36 27 37
    w2 = _mm256_unpackhi_epi16(*x.add(4), *x.add(5)); // 44 54 45 55 46 56 47 57
    w3 = _mm256_unpackhi_epi16(*x.add(6), *x.add(7)); // 64 74 65 75 66 76 67 77

    ww0 = _mm256_unpacklo_epi32(w0, w1); // 04 14 24 34 05 15 25 35
    ww1 = _mm256_unpacklo_epi32(w2, w3); // 44 54 64 74 45 55 65 75

    *d.add(4) = _mm256_unpacklo_epi64(ww0, ww1); // 04 14 24 34 44 54 64 74
    *d.add(5) = _mm256_unpackhi_epi64(ww0, ww1); // 05 15 25 35 45 55 65 75

    ww0 = _mm256_unpackhi_epi32(w0, w1); // 06 16 26 36 07 17 27 37
    ww1 = _mm256_unpackhi_epi32(w2, w3); // 46 56 66 76 47 57 67 77

    *d.add(6) = _mm256_unpacklo_epi64(ww0, ww1); // 06 16 26 36 46 56 66 76
    *d.add(7) = _mm256_unpackhi_epi64(ww0, ww1); // 07 17 27 37 47 57 67 77
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn highbd_transpose16x16_avx2(x: *const __m256i, d: *mut __m256i) {
    let mut w0: __m256i;
    let mut w1: __m256i;
    let mut w2: __m256i;
    let mut w3: __m256i;
    let mut ww0: __m256i;
    let mut ww1: __m256i;
    let mut dd: [__m256i; 16] = core::mem::zeroed();
    w0 = _mm256_unpacklo_epi16(*x.add(0), *x.add(1));
    w1 = _mm256_unpacklo_epi16(*x.add(2), *x.add(3));
    w2 = _mm256_unpacklo_epi16(*x.add(4), *x.add(5));
    w3 = _mm256_unpacklo_epi16(*x.add(6), *x.add(7));

    ww0 = _mm256_unpacklo_epi32(w0, w1); //
    ww1 = _mm256_unpacklo_epi32(w2, w3); //

    dd[0] = _mm256_unpacklo_epi64(ww0, ww1);
    dd[1] = _mm256_unpackhi_epi64(ww0, ww1);

    ww0 = _mm256_unpackhi_epi32(w0, w1); //
    ww1 = _mm256_unpackhi_epi32(w2, w3); //

    dd[2] = _mm256_unpacklo_epi64(ww0, ww1);
    dd[3] = _mm256_unpackhi_epi64(ww0, ww1);

    w0 = _mm256_unpackhi_epi16(*x.add(0), *x.add(1));
    w1 = _mm256_unpackhi_epi16(*x.add(2), *x.add(3));
    w2 = _mm256_unpackhi_epi16(*x.add(4), *x.add(5));
    w3 = _mm256_unpackhi_epi16(*x.add(6), *x.add(7));

    ww0 = _mm256_unpacklo_epi32(w0, w1); //
    ww1 = _mm256_unpacklo_epi32(w2, w3); //

    dd[4] = _mm256_unpacklo_epi64(ww0, ww1);
    dd[5] = _mm256_unpackhi_epi64(ww0, ww1);

    ww0 = _mm256_unpackhi_epi32(w0, w1); //
    ww1 = _mm256_unpackhi_epi32(w2, w3); //

    dd[6] = _mm256_unpacklo_epi64(ww0, ww1);
    dd[7] = _mm256_unpackhi_epi64(ww0, ww1);

    w0 = _mm256_unpacklo_epi16(*x.add(8), *x.add(9));
    w1 = _mm256_unpacklo_epi16(*x.add(10), *x.add(11));
    w2 = _mm256_unpacklo_epi16(*x.add(12), *x.add(13));
    w3 = _mm256_unpacklo_epi16(*x.add(14), *x.add(15));

    ww0 = _mm256_unpacklo_epi32(w0, w1);
    ww1 = _mm256_unpacklo_epi32(w2, w3);

    dd[8] = _mm256_unpacklo_epi64(ww0, ww1);
    dd[9] = _mm256_unpackhi_epi64(ww0, ww1);

    ww0 = _mm256_unpackhi_epi32(w0, w1);
    ww1 = _mm256_unpackhi_epi32(w2, w3);

    dd[10] = _mm256_unpacklo_epi64(ww0, ww1);
    dd[11] = _mm256_unpackhi_epi64(ww0, ww1);

    w0 = _mm256_unpackhi_epi16(*x.add(8), *x.add(9));
    w1 = _mm256_unpackhi_epi16(*x.add(10), *x.add(11));
    w2 = _mm256_unpackhi_epi16(*x.add(12), *x.add(13));
    w3 = _mm256_unpackhi_epi16(*x.add(14), *x.add(15));

    ww0 = _mm256_unpacklo_epi32(w0, w1);
    ww1 = _mm256_unpacklo_epi32(w2, w3);

    dd[12] = _mm256_unpacklo_epi64(ww0, ww1);
    dd[13] = _mm256_unpackhi_epi64(ww0, ww1);

    ww0 = _mm256_unpackhi_epi32(w0, w1);
    ww1 = _mm256_unpackhi_epi32(w2, w3);

    dd[14] = _mm256_unpacklo_epi64(ww0, ww1);
    dd[15] = _mm256_unpackhi_epi64(ww0, ww1);

    for i in 0..8 {
        *d.add(i) = _mm256_insertf128_si256::<1>(dd[i], _mm256_castsi256_si128(dd[i + 8]));
        *d.add(i + 8) =
            _mm256_insertf128_si256::<0>(dd[i + 8], _mm256_extracti128_si256::<1>(dd[i]));
    }
}
// CONFIG_AV1_HIGHBITDEPTH

// CONFIG_AV1_HIGHBITDEPTH

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn highbd_dr_prediction_z1_4xN_internal_avx2(
    N: i32,
    dst: *mut __m128i,
    above: *const u16,
    upsample_above: i32,
    dx: i32,
) {
    let frac_bits = 6 - upsample_above;
    let max_base_x = ((N + 4) - 1) << upsample_above;

    debug_assert!(dx > 0);
    // pre-filter above pixels
    // store in temp buffers:
    //   above[x] * 32 + 16
    //   above[x+1] - above[x]
    // final pixels will be calculated as:
    //   (above[x] * 32 + 16 + (above[x+1] - above[x]) * shift) >> 5
    let mut a0: __m256i;
    let mut a1: __m256i;
    let mut a32: __m256i;
    let a16: __m256i;
    let mut diff: __m256i;
    let c3f: __m256i;
    let a_mbase_x: __m128i;
    let max_base_x128: __m128i;
    let mut base_inc128: __m128i;
    let mut mask128: __m128i;
    let mut a0_128: __m128i;
    let mut a1_128: __m128i;
    a16 = _mm256_set1_epi16(16);
    a_mbase_x = _mm_set1_epi16(*above.offset(max_base_x as isize) as i16);
    max_base_x128 = _mm_set1_epi16(max_base_x as i16);
    c3f = _mm256_set1_epi16(0x3f);

    let mut x = dx;
    for r in 0..N {
        let b: __m256i;
        let mut res: __m256i;
        let shift: __m256i;
        let res1: __m128i;

        let base = x >> frac_bits;
        if base >= max_base_x {
            for i in r..N {
                *dst.add(i as usize) = a_mbase_x; // save 4 values
            }
            return;
        }

        a0_128 = _mm_loadu_si128(above.offset(base as isize) as *const __m128i);
        a1_128 = _mm_loadu_si128(above.offset(base as isize + 1) as *const __m128i);

        if upsample_above != 0 {
            a0_128 = _mm_shuffle_epi8(a0_128, load_mask128(HighbdEvenOddMaskx4.0[0].as_ptr()));
            a1_128 = _mm_srli_si128::<8>(a0_128);

            base_inc128 = _mm_setr_epi16(
                base as i16,
                (base + 2) as i16,
                (base + 4) as i16,
                (base + 6) as i16,
                (base + 8) as i16,
                (base + 10) as i16,
                (base + 12) as i16,
                (base + 14) as i16,
            );
            shift = _mm256_srli_epi16::<1>(_mm256_and_si256(
                _mm256_sll_epi16(
                    _mm256_set1_epi16(x as i16),
                    _mm_cvtsi32_si128(upsample_above),
                ),
                _mm256_set1_epi16(0x3f),
            ));
        } else {
            base_inc128 = _mm_setr_epi16(
                base as i16,
                (base + 1) as i16,
                (base + 2) as i16,
                (base + 3) as i16,
                (base + 4) as i16,
                (base + 5) as i16,
                (base + 6) as i16,
                (base + 7) as i16,
            );
            shift = _mm256_srli_epi16::<1>(_mm256_and_si256(_mm256_set1_epi16(x as i16), c3f));
        }
        a0 = _mm256_castsi128_si256(a0_128);
        a1 = _mm256_castsi128_si256(a1_128);
        diff = _mm256_sub_epi16(a1, a0); // a[x+1] - a[x]
        a32 = _mm256_slli_epi16::<5>(a0); // a[x] * 32
        a32 = _mm256_add_epi16(a32, a16); // a[x] * 32 + 16

        b = _mm256_mullo_epi16(diff, shift);
        res = _mm256_add_epi16(a32, b);
        res = _mm256_srli_epi16::<5>(res);
        res1 = _mm256_castsi256_si128(res);

        mask128 = _mm_cmpgt_epi16(max_base_x128, base_inc128);
        *dst.add(r as usize) = _mm_blendv_epi8(a_mbase_x, res1, mask128);
        x += dx;
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn highbd_dr_prediction_32bit_z1_4xN_internal_avx2(
    N: i32,
    dst: *mut __m128i,
    above: *const u16,
    upsample_above: i32,
    dx: i32,
) {
    let frac_bits = 6 - upsample_above;
    let max_base_x = ((N + 4) - 1) << upsample_above;

    debug_assert!(dx > 0);
    // pre-filter above pixels
    // store in temp buffers:
    //   above[x] * 32 + 16
    //   above[x+1] - above[x]
    // final pixels will be calculated as:
    //   (above[x] * 32 + 16 + (above[x+1] - above[x]) * shift) >> 5
    let mut a0: __m256i;
    let mut a1: __m256i;
    let mut a32: __m256i;
    let a16: __m256i;
    let mut diff: __m256i;
    let a_mbase_x: __m128i;
    let max_base_x128: __m128i;
    let mut base_inc128: __m128i;
    let mut mask128: __m128i;

    a16 = _mm256_set1_epi32(16);
    a_mbase_x = _mm_set1_epi16(*above.offset(max_base_x as isize) as i16);
    max_base_x128 = _mm_set1_epi32(max_base_x);

    let mut x = dx;
    for r in 0..N {
        let b: __m256i;
        let mut res: __m256i;
        let shift: __m256i;
        let mut res1: __m128i;

        let base = x >> frac_bits;
        if base >= max_base_x {
            for i in r..N {
                *dst.add(i as usize) = a_mbase_x; // save 4 values
            }
            return;
        }

        a0 = _mm256_cvtepu16_epi32(_mm_loadu_si128(
            above.offset(base as isize) as *const __m128i
        ));
        a1 = _mm256_cvtepu16_epi32(_mm_loadu_si128(
            above.offset(base as isize + 1) as *const __m128i
        ));

        if upsample_above != 0 {
            a0 = _mm256_permutevar8x32_epi32(a0, _mm256_set_epi32(7, 5, 3, 1, 6, 4, 2, 0));
            a1 = _mm256_castsi128_si256(_mm256_extracti128_si256::<1>(a0));
            base_inc128 = _mm_setr_epi32(base, base + 2, base + 4, base + 6);
            shift = _mm256_srli_epi32::<1>(_mm256_and_si256(
                _mm256_sll_epi32(_mm256_set1_epi32(x), _mm_cvtsi32_si128(upsample_above)),
                _mm256_set1_epi32(0x3f),
            ));
        } else {
            base_inc128 = _mm_setr_epi32(base, base + 1, base + 2, base + 3);
            shift = _mm256_srli_epi32::<1>(_mm256_and_si256(
                _mm256_set1_epi32(x),
                _mm256_set1_epi32(0x3f),
            ));
        }

        diff = _mm256_sub_epi32(a1, a0); // a[x+1] - a[x]
        a32 = _mm256_slli_epi32::<5>(a0); // a[x] * 32
        a32 = _mm256_add_epi32(a32, a16); // a[x] * 32 + 16

        b = _mm256_mullo_epi32(diff, shift);
        res = _mm256_add_epi32(a32, b);
        res = _mm256_srli_epi32::<5>(res);

        res1 = _mm256_castsi256_si128(res);
        res1 = _mm_packus_epi32(res1, res1);

        mask128 = _mm_cmpgt_epi32(max_base_x128, base_inc128);
        mask128 = _mm_packs_epi32(mask128, mask128); // goto 16 bit
        *dst.add(r as usize) = _mm_blendv_epi8(a_mbase_x, res1, mask128);
        x += dx;
    }
}

#[target_feature(enable = "avx2")]
unsafe fn highbd_dr_prediction_z1_4xN_avx2(
    N: i32,
    dst: *mut u16,
    stride: isize,
    above: *const u16,
    upsample_above: i32,
    dx: i32,
    bd: i32,
) {
    let mut dstvec: [__m128i; 16] = core::mem::zeroed();
    if bd < 12 {
        highbd_dr_prediction_z1_4xN_internal_avx2(
            N,
            dstvec.as_mut_ptr(),
            above,
            upsample_above,
            dx,
        );
    } else {
        highbd_dr_prediction_32bit_z1_4xN_internal_avx2(
            N,
            dstvec.as_mut_ptr(),
            above,
            upsample_above,
            dx,
        );
    }
    for i in 0..N {
        _mm_storel_epi64(
            dst.offset(stride * i as isize) as *mut __m128i,
            dstvec[i as usize],
        );
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn highbd_dr_prediction_32bit_z1_8xN_internal_avx2(
    N: i32,
    dst: *mut __m128i,
    above: *const u16,
    upsample_above: i32,
    dx: i32,
) {
    let frac_bits = 6 - upsample_above;
    let max_base_x = ((8 + N) - 1) << upsample_above;

    debug_assert!(dx > 0);
    // pre-filter above pixels
    // store in temp buffers:
    //   above[x] * 32 + 16
    //   above[x+1] - above[x]
    // final pixels will be calculated as:
    //   (above[x] * 32 + 16 + (above[x+1] - above[x]) * shift) >> 5
    let mut a0: __m256i;
    let mut a1: __m256i;
    let mut a0_1: __m256i;
    let mut a1_1: __m256i;
    let mut a32: __m256i;
    let a16: __m256i;
    let a_mbase_x: __m256i;
    let mut diff: __m256i;
    let max_base_x256: __m256i;
    let mut base_inc256: __m256i;
    let mut mask256: __m256i;

    a16 = _mm256_set1_epi32(16);
    a_mbase_x = _mm256_set1_epi16(*above.offset(max_base_x as isize) as i16);
    max_base_x256 = _mm256_set1_epi32(max_base_x);

    let mut x = dx;
    for r in 0..N {
        let b: __m256i;
        let mut res: __m256i;
        let mut res1: __m256i;
        let shift: __m256i;

        let base = x >> frac_bits;
        if base >= max_base_x {
            for i in r..N {
                *dst.add(i as usize) = _mm256_castsi256_si128(a_mbase_x); // save 8 values
            }
            return;
        }

        a0 = _mm256_cvtepu16_epi32(_mm_loadu_si128(
            above.offset(base as isize) as *const __m128i
        ));
        a1 = _mm256_cvtepu16_epi32(_mm_loadu_si128(
            above.offset(base as isize + 1) as *const __m128i
        ));

        if upsample_above != 0 {
            a0 = _mm256_permutevar8x32_epi32(a0, _mm256_set_epi32(7, 5, 3, 1, 6, 4, 2, 0));
            a1 = _mm256_castsi128_si256(_mm256_extracti128_si256::<1>(a0));

            a0_1 = _mm256_cvtepu16_epi32(_mm_loadu_si128(
                above.offset(base as isize + 8) as *const __m128i
            ));
            a0_1 = _mm256_permutevar8x32_epi32(a0_1, _mm256_set_epi32(7, 5, 3, 1, 6, 4, 2, 0));
            a1_1 = _mm256_castsi128_si256(_mm256_extracti128_si256::<1>(a0_1));

            a0 = _mm256_inserti128_si256::<1>(a0, _mm256_castsi256_si128(a0_1));
            a1 = _mm256_inserti128_si256::<1>(a1, _mm256_castsi256_si128(a1_1));
            base_inc256 = _mm256_setr_epi32(
                base,
                base + 2,
                base + 4,
                base + 6,
                base + 8,
                base + 10,
                base + 12,
                base + 14,
            );
            shift = _mm256_srli_epi32::<1>(_mm256_and_si256(
                _mm256_sll_epi32(_mm256_set1_epi32(x), _mm_cvtsi32_si128(upsample_above)),
                _mm256_set1_epi32(0x3f),
            ));
        } else {
            base_inc256 = _mm256_setr_epi32(
                base,
                base + 1,
                base + 2,
                base + 3,
                base + 4,
                base + 5,
                base + 6,
                base + 7,
            );
            shift = _mm256_srli_epi32::<1>(_mm256_and_si256(
                _mm256_set1_epi32(x),
                _mm256_set1_epi32(0x3f),
            ));
        }

        diff = _mm256_sub_epi32(a1, a0); // a[x+1] - a[x]
        a32 = _mm256_slli_epi32::<5>(a0); // a[x] * 32
        a32 = _mm256_add_epi32(a32, a16); // a[x] * 32 + 16

        b = _mm256_mullo_epi32(diff, shift);
        res = _mm256_add_epi32(a32, b);
        res = _mm256_srli_epi32::<5>(res);

        res1 = _mm256_packus_epi32(
            res,
            _mm256_castsi128_si256(_mm256_extracti128_si256::<1>(res)),
        );

        mask256 = _mm256_cmpgt_epi32(max_base_x256, base_inc256);
        mask256 = _mm256_packs_epi32(
            mask256,
            _mm256_castsi128_si256(_mm256_extracti128_si256::<1>(mask256)),
        ); // goto 16 bit
        res1 = _mm256_blendv_epi8(a_mbase_x, res1, mask256);
        *dst.add(r as usize) = _mm256_castsi256_si128(res1);
        x += dx;
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn highbd_dr_prediction_z1_8xN_internal_avx2(
    N: i32,
    dst: *mut __m128i,
    above: *const u16,
    upsample_above: i32,
    dx: i32,
) {
    let frac_bits = 6 - upsample_above;
    let max_base_x = ((8 + N) - 1) << upsample_above;

    debug_assert!(dx > 0);
    // pre-filter above pixels
    // store in temp buffers:
    //   above[x] * 32 + 16
    //   above[x+1] - above[x]
    // final pixels will be calculated as:
    //   (above[x] * 32 + 16 + (above[x+1] - above[x]) * shift) >> 5
    let mut a0: __m256i;
    let mut a1: __m256i;
    let mut a32: __m256i;
    let a16: __m256i;
    let c3f: __m256i;
    let a_mbase_x: __m256i;
    let mut diff: __m256i;
    let max_base_x256: __m256i;
    let mut base_inc256: __m256i;
    let mut mask256: __m256i;
    let mut a0_x128: __m128i;
    let mut a1_x128: __m128i;

    a16 = _mm256_set1_epi16(16);
    a_mbase_x = _mm256_set1_epi16(*above.offset(max_base_x as isize) as i16);
    max_base_x256 = _mm256_set1_epi16(max_base_x as i16);
    c3f = _mm256_set1_epi16(0x3f);

    let mut x = dx;
    for r in 0..N {
        let b: __m256i;
        let mut res: __m256i;
        let res1: __m256i;
        let shift: __m256i;

        let base = x >> frac_bits;
        if base >= max_base_x {
            for i in r..N {
                *dst.add(i as usize) = _mm256_castsi256_si128(a_mbase_x); // save 8 values
            }
            return;
        }

        a0_x128 = _mm_loadu_si128(above.offset(base as isize) as *const __m128i);
        if upsample_above != 0 {
            let mut mask: __m128i;
            let atmp0: __m128i;
            let atmp1: __m128i;
            let atmp2: __m128i;
            let atmp3: __m128i;
            a1_x128 = _mm_loadu_si128(above.offset(base as isize + 8) as *const __m128i);
            atmp0 = _mm_shuffle_epi8(a0_x128, load_mask128(HighbdEvenOddMaskx.0[0].as_ptr()));
            atmp1 = _mm_shuffle_epi8(a1_x128, load_mask128(HighbdEvenOddMaskx.0[0].as_ptr()));
            atmp2 = _mm_shuffle_epi8(
                a0_x128,
                load_mask128(HighbdEvenOddMaskx.0[0].as_ptr().add(16)),
            );
            atmp3 = _mm_shuffle_epi8(
                a1_x128,
                load_mask128(HighbdEvenOddMaskx.0[0].as_ptr().add(16)),
            );
            mask = _mm_cmpgt_epi8(
                load_mask128(HighbdEvenOddMaskx.0[0].as_ptr()),
                _mm_set1_epi8(15),
            );
            a0_x128 = _mm_blendv_epi8(atmp0, atmp1, mask);
            mask = _mm_cmpgt_epi8(
                load_mask128(HighbdEvenOddMaskx.0[0].as_ptr().add(16)),
                _mm_set1_epi8(15),
            );
            a1_x128 = _mm_blendv_epi8(atmp2, atmp3, mask);

            base_inc256 = _mm256_setr_epi16(
                base as i16,
                (base + 2) as i16,
                (base + 4) as i16,
                (base + 6) as i16,
                (base + 8) as i16,
                (base + 10) as i16,
                (base + 12) as i16,
                (base + 14) as i16,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            );
            shift = _mm256_srli_epi16::<1>(_mm256_and_si256(
                _mm256_sll_epi16(
                    _mm256_set1_epi16(x as i16),
                    _mm_cvtsi32_si128(upsample_above),
                ),
                c3f,
            ));
        } else {
            a1_x128 = _mm_loadu_si128(above.offset(base as isize + 1) as *const __m128i);
            base_inc256 = _mm256_setr_epi16(
                base as i16,
                (base + 1) as i16,
                (base + 2) as i16,
                (base + 3) as i16,
                (base + 4) as i16,
                (base + 5) as i16,
                (base + 6) as i16,
                (base + 7) as i16,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            );
            shift = _mm256_srli_epi16::<1>(_mm256_and_si256(_mm256_set1_epi16(x as i16), c3f));
        }
        a0 = _mm256_castsi128_si256(a0_x128);
        a1 = _mm256_castsi128_si256(a1_x128);

        diff = _mm256_sub_epi16(a1, a0); // a[x+1] - a[x]
        a32 = _mm256_slli_epi16::<5>(a0); // a[x] * 32
        a32 = _mm256_add_epi16(a32, a16); // a[x] * 32 + 16

        b = _mm256_mullo_epi16(diff, shift);
        res = _mm256_add_epi16(a32, b);
        res = _mm256_srli_epi16::<5>(res);

        mask256 = _mm256_cmpgt_epi16(max_base_x256, base_inc256);
        res1 = _mm256_blendv_epi8(a_mbase_x, res, mask256);
        *dst.add(r as usize) = _mm256_castsi256_si128(res1);
        x += dx;
    }
}

#[target_feature(enable = "avx2")]
unsafe fn highbd_dr_prediction_z1_8xN_avx2(
    N: i32,
    dst: *mut u16,
    stride: isize,
    above: *const u16,
    upsample_above: i32,
    dx: i32,
    bd: i32,
) {
    let mut dstvec: [__m128i; 32] = core::mem::zeroed();
    if bd < 12 {
        highbd_dr_prediction_z1_8xN_internal_avx2(
            N,
            dstvec.as_mut_ptr(),
            above,
            upsample_above,
            dx,
        );
    } else {
        highbd_dr_prediction_32bit_z1_8xN_internal_avx2(
            N,
            dstvec.as_mut_ptr(),
            above,
            upsample_above,
            dx,
        );
    }
    for i in 0..N {
        _mm_storeu_si128(
            dst.offset(stride * i as isize) as *mut __m128i,
            dstvec[i as usize],
        );
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn highbd_dr_prediction_32bit_z1_16xN_internal_avx2(
    N: i32,
    dstvec: *mut __m256i,
    above: *const u16,
    upsample_above: i32,
    dx: i32,
) {
    // here upsample_above is 0 by design of av1_use_intra_edge_upsample
    let _ = upsample_above;
    let frac_bits = 6;
    let max_base_x = (16 + N) - 1;

    // pre-filter above pixels
    // store in temp buffers:
    //   above[x] * 32 + 16
    //   above[x+1] - above[x]
    // final pixels will be calculated as:
    //   (above[x] * 32 + 16 + (above[x+1] - above[x]) * shift) >> 5
    let mut a0: __m256i;
    let mut a0_1: __m256i;
    let mut a1: __m256i;
    let mut a1_1: __m256i;
    let mut a32: __m256i;
    let a16: __m256i;
    let a_mbase_x: __m256i;
    let mut diff: __m256i;
    let max_base_x256: __m256i;
    let mut base_inc256: __m256i;
    let mut mask256: __m256i;

    a16 = _mm256_set1_epi32(16);
    a_mbase_x = _mm256_set1_epi16(*above.offset(max_base_x as isize) as i16);
    max_base_x256 = _mm256_set1_epi16(max_base_x as i16);

    let mut x = dx;
    for r in 0..N {
        let mut b: __m256i;
        let mut res: [__m256i; 2] = core::mem::zeroed();
        let res1: __m256i;

        let base = x >> frac_bits;
        if base >= max_base_x {
            for i in r..N {
                *dstvec.add(i as usize) = a_mbase_x; // save 16 values
            }
            return;
        }
        let shift = _mm256_srli_epi32::<1>(_mm256_and_si256(
            _mm256_set1_epi32(x),
            _mm256_set1_epi32(0x3f),
        ));

        a0 = _mm256_cvtepu16_epi32(_mm_loadu_si128(
            above.offset(base as isize) as *const __m128i
        ));
        a1 = _mm256_cvtepu16_epi32(_mm_loadu_si128(
            above.offset(base as isize + 1) as *const __m128i
        ));

        diff = _mm256_sub_epi32(a1, a0); // a[x+1] - a[x]
        a32 = _mm256_slli_epi32::<5>(a0); // a[x] * 32
        a32 = _mm256_add_epi32(a32, a16); // a[x] * 32 + 16
        b = _mm256_mullo_epi32(diff, shift);

        res[0] = _mm256_add_epi32(a32, b);
        res[0] = _mm256_srli_epi32::<5>(res[0]);
        res[0] = _mm256_packus_epi32(
            res[0],
            _mm256_castsi128_si256(_mm256_extracti128_si256::<1>(res[0])),
        );

        let mdif = max_base_x - base;
        if mdif > 8 {
            a0_1 = _mm256_cvtepu16_epi32(_mm_loadu_si128(
                above.offset(base as isize + 8) as *const __m128i
            ));
            a1_1 = _mm256_cvtepu16_epi32(_mm_loadu_si128(
                above.offset(base as isize + 9) as *const __m128i
            ));

            diff = _mm256_sub_epi32(a1_1, a0_1); // a[x+1] - a[x]
            a32 = _mm256_slli_epi32::<5>(a0_1); // a[x] * 32
            a32 = _mm256_add_epi32(a32, a16); // a[x] * 32 + 16
            b = _mm256_mullo_epi32(diff, shift);

            res[1] = _mm256_add_epi32(a32, b);
            res[1] = _mm256_srli_epi32::<5>(res[1]);
            res[1] = _mm256_packus_epi32(
                res[1],
                _mm256_castsi128_si256(_mm256_extracti128_si256::<1>(res[1])),
            );
        } else {
            res[1] = a_mbase_x;
        }
        res1 = _mm256_inserti128_si256::<1>(res[0], _mm256_castsi256_si128(res[1])); // 16 16bit values

        base_inc256 = _mm256_setr_epi16(
            base as i16,
            (base + 1) as i16,
            (base + 2) as i16,
            (base + 3) as i16,
            (base + 4) as i16,
            (base + 5) as i16,
            (base + 6) as i16,
            (base + 7) as i16,
            (base + 8) as i16,
            (base + 9) as i16,
            (base + 10) as i16,
            (base + 11) as i16,
            (base + 12) as i16,
            (base + 13) as i16,
            (base + 14) as i16,
            (base + 15) as i16,
        );
        mask256 = _mm256_cmpgt_epi16(max_base_x256, base_inc256);
        *dstvec.add(r as usize) = _mm256_blendv_epi8(a_mbase_x, res1, mask256);
        x += dx;
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn highbd_dr_prediction_z1_16xN_internal_avx2(
    N: i32,
    dstvec: *mut __m256i,
    above: *const u16,
    upsample_above: i32,
    dx: i32,
) {
    // here upsample_above is 0 by design of av1_use_intra_edge_upsample
    let _ = upsample_above;
    let frac_bits = 6;
    let max_base_x = (16 + N) - 1;

    // pre-filter above pixels
    // store in temp buffers:
    //   above[x] * 32 + 16
    //   above[x+1] - above[x]
    // final pixels will be calculated as:
    //   (above[x] * 32 + 16 + (above[x+1] - above[x]) * shift) >> 5
    let mut a0: __m256i;
    let mut a1: __m256i;
    let mut a32: __m256i;
    let a16: __m256i;
    let c3f: __m256i;
    let a_mbase_x: __m256i;
    let mut diff: __m256i;
    let max_base_x256: __m256i;
    let mut base_inc256: __m256i;
    let mut mask256: __m256i;

    a16 = _mm256_set1_epi16(16);
    a_mbase_x = _mm256_set1_epi16(*above.offset(max_base_x as isize) as i16);
    max_base_x256 = _mm256_set1_epi16(max_base_x as i16);
    c3f = _mm256_set1_epi16(0x3f);

    let mut x = dx;
    for r in 0..N {
        let b: __m256i;
        let mut res: __m256i;

        let base = x >> frac_bits;
        if base >= max_base_x {
            for i in r..N {
                *dstvec.add(i as usize) = a_mbase_x; // save 16 values
            }
            return;
        }
        let shift = _mm256_srli_epi16::<1>(_mm256_and_si256(_mm256_set1_epi16(x as i16), c3f));

        a0 = _mm256_loadu_si256(above.offset(base as isize) as *const __m256i);
        a1 = _mm256_loadu_si256(above.offset(base as isize + 1) as *const __m256i);

        diff = _mm256_sub_epi16(a1, a0); // a[x+1] - a[x]
        a32 = _mm256_slli_epi16::<5>(a0); // a[x] * 32
        a32 = _mm256_add_epi16(a32, a16); // a[x] * 32 + 16
        b = _mm256_mullo_epi16(diff, shift);

        res = _mm256_add_epi16(a32, b);
        res = _mm256_srli_epi16::<5>(res); // 16 16bit values

        base_inc256 = _mm256_setr_epi16(
            base as i16,
            (base + 1) as i16,
            (base + 2) as i16,
            (base + 3) as i16,
            (base + 4) as i16,
            (base + 5) as i16,
            (base + 6) as i16,
            (base + 7) as i16,
            (base + 8) as i16,
            (base + 9) as i16,
            (base + 10) as i16,
            (base + 11) as i16,
            (base + 12) as i16,
            (base + 13) as i16,
            (base + 14) as i16,
            (base + 15) as i16,
        );
        mask256 = _mm256_cmpgt_epi16(max_base_x256, base_inc256);
        *dstvec.add(r as usize) = _mm256_blendv_epi8(a_mbase_x, res, mask256);
        x += dx;
    }
}

#[target_feature(enable = "avx2")]
unsafe fn highbd_dr_prediction_z1_16xN_avx2(
    N: i32,
    dst: *mut u16,
    stride: isize,
    above: *const u16,
    upsample_above: i32,
    dx: i32,
    bd: i32,
) {
    let mut dstvec: [__m256i; 64] = core::mem::zeroed();
    if bd < 12 {
        highbd_dr_prediction_z1_16xN_internal_avx2(
            N,
            dstvec.as_mut_ptr(),
            above,
            upsample_above,
            dx,
        );
    } else {
        highbd_dr_prediction_32bit_z1_16xN_internal_avx2(
            N,
            dstvec.as_mut_ptr(),
            above,
            upsample_above,
            dx,
        );
    }
    for i in 0..N {
        _mm256_storeu_si256(
            dst.offset(stride * i as isize) as *mut __m256i,
            dstvec[i as usize],
        );
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn highbd_dr_prediction_32bit_z1_32xN_internal_avx2(
    N: i32,
    dstvec: *mut __m256i,
    above: *const u16,
    upsample_above: i32,
    dx: i32,
) {
    // here upsample_above is 0 by design of av1_use_intra_edge_upsample
    let _ = upsample_above;
    let frac_bits = 6;
    let max_base_x = (32 + N) - 1;

    // pre-filter above pixels
    // store in temp buffers:
    //   above[x] * 32 + 16
    //   above[x+1] - above[x]
    // final pixels will be calculated as:
    //   (above[x] * 32 + 16 + (above[x+1] - above[x]) * shift) >> 5
    let mut a0: __m256i;
    let mut a0_1: __m256i;
    let mut a1: __m256i;
    let mut a1_1: __m256i;
    let mut a32: __m256i;
    let a16: __m256i;
    let c3f: __m256i;
    let a_mbase_x: __m256i;
    let mut diff: __m256i;
    let max_base_x256: __m256i;
    let mut base_inc256: __m256i;
    let mut mask256: __m256i;

    a16 = _mm256_set1_epi32(16);
    a_mbase_x = _mm256_set1_epi16(*above.offset(max_base_x as isize) as i16);
    max_base_x256 = _mm256_set1_epi16(max_base_x as i16);
    c3f = _mm256_set1_epi16(0x3f);

    let mut x = dx;
    for r in 0..N {
        let mut b: __m256i;
        let mut res: [__m256i; 2] = core::mem::zeroed();
        let mut res1: __m256i;

        let base = x >> frac_bits;
        if base >= max_base_x {
            for i in r..N {
                *dstvec.add(i as usize) = a_mbase_x; // save 32 values
                *dstvec.add((i + N) as usize) = a_mbase_x;
            }
            return;
        }

        let shift = _mm256_srli_epi32::<1>(_mm256_and_si256(_mm256_set1_epi32(x), c3f));

        let mut j = 0;
        while j < 32 {
            let mdif = max_base_x - (base + j);
            if mdif <= 0 {
                res1 = a_mbase_x;
            } else {
                a0 = _mm256_cvtepu16_epi32(_mm_loadu_si128(
                    above.offset((base + j) as isize) as *const __m128i
                ));
                a1 = _mm256_cvtepu16_epi32(_mm_loadu_si128(
                    above.offset((base + 1 + j) as isize) as *const __m128i
                ));

                diff = _mm256_sub_epi32(a1, a0); // a[x+1] - a[x]
                a32 = _mm256_slli_epi32::<5>(a0); // a[x] * 32
                a32 = _mm256_add_epi32(a32, a16); // a[x] * 32 + 16
                b = _mm256_mullo_epi32(diff, shift);

                res[0] = _mm256_add_epi32(a32, b);
                res[0] = _mm256_srli_epi32::<5>(res[0]);
                res[0] = _mm256_packus_epi32(
                    res[0],
                    _mm256_castsi128_si256(_mm256_extracti128_si256::<1>(res[0])),
                );
                if mdif > 8 {
                    a0_1 = _mm256_cvtepu16_epi32(_mm_loadu_si128(
                        above.offset((base + 8 + j) as isize) as *const __m128i,
                    ));
                    a1_1 = _mm256_cvtepu16_epi32(_mm_loadu_si128(
                        above.offset((base + 9 + j) as isize) as *const __m128i,
                    ));

                    diff = _mm256_sub_epi32(a1_1, a0_1); // a[x+1] - a[x]
                    a32 = _mm256_slli_epi32::<5>(a0_1); // a[x] * 32
                    a32 = _mm256_add_epi32(a32, a16); // a[x] * 32 + 16
                    b = _mm256_mullo_epi32(diff, shift);

                    res[1] = _mm256_add_epi32(a32, b);
                    res[1] = _mm256_srli_epi32::<5>(res[1]);
                    res[1] = _mm256_packus_epi32(
                        res[1],
                        _mm256_castsi128_si256(_mm256_extracti128_si256::<1>(res[1])),
                    );
                } else {
                    res[1] = a_mbase_x;
                }
                res1 = _mm256_inserti128_si256::<1>(res[0], _mm256_castsi256_si128(res[1])); // 16 16bit values
                base_inc256 = _mm256_setr_epi16(
                    (base + j) as i16,
                    (base + j + 1) as i16,
                    (base + j + 2) as i16,
                    (base + j + 3) as i16,
                    (base + j + 4) as i16,
                    (base + j + 5) as i16,
                    (base + j + 6) as i16,
                    (base + j + 7) as i16,
                    (base + j + 8) as i16,
                    (base + j + 9) as i16,
                    (base + j + 10) as i16,
                    (base + j + 11) as i16,
                    (base + j + 12) as i16,
                    (base + j + 13) as i16,
                    (base + j + 14) as i16,
                    (base + j + 15) as i16,
                );

                mask256 = _mm256_cmpgt_epi16(max_base_x256, base_inc256);
                res1 = _mm256_blendv_epi8(a_mbase_x, res1, mask256);
            }
            if j == 0 {
                *dstvec.add(r as usize) = res1;
            } else {
                *dstvec.add((r + N) as usize) = res1;
            }
            j += 16;
        }
        x += dx;
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn highbd_dr_prediction_z1_32xN_internal_avx2(
    N: i32,
    dstvec: *mut __m256i,
    above: *const u16,
    upsample_above: i32,
    dx: i32,
) {
    // here upsample_above is 0 by design of av1_use_intra_edge_upsample
    let _ = upsample_above;
    let frac_bits = 6;
    let max_base_x = (32 + N) - 1;

    // pre-filter above pixels
    // store in temp buffers:
    //   above[x] * 32 + 16
    //   above[x+1] - above[x]
    // final pixels will be calculated as:
    //   (above[x] * 32 + 16 + (above[x+1] - above[x]) * shift) >> 5
    let mut a0: __m256i;
    let mut a1: __m256i;
    let mut a32: __m256i;
    let a16: __m256i;
    let c3f: __m256i;
    let a_mbase_x: __m256i;
    let mut diff: __m256i;
    let max_base_x256: __m256i;
    let mut base_inc256: __m256i;
    let mut mask256: __m256i;

    a16 = _mm256_set1_epi16(16);
    a_mbase_x = _mm256_set1_epi16(*above.offset(max_base_x as isize) as i16);
    max_base_x256 = _mm256_set1_epi16(max_base_x as i16);
    c3f = _mm256_set1_epi16(0x3f);

    let mut x = dx;
    for r in 0..N {
        let mut b: __m256i;
        let mut res: __m256i;

        let base = x >> frac_bits;
        if base >= max_base_x {
            for i in r..N {
                *dstvec.add(i as usize) = a_mbase_x; // save 32 values
                *dstvec.add((i + N) as usize) = a_mbase_x;
            }
            return;
        }

        let shift = _mm256_srli_epi16::<1>(_mm256_and_si256(_mm256_set1_epi16(x as i16), c3f));

        let mut j = 0;
        while j < 32 {
            let mdif = max_base_x - (base + j);
            if mdif <= 0 {
                res = a_mbase_x;
            } else {
                a0 = _mm256_loadu_si256(above.offset((base + j) as isize) as *const __m256i);
                a1 = _mm256_loadu_si256(above.offset((base + 1 + j) as isize) as *const __m256i);

                diff = _mm256_sub_epi16(a1, a0); // a[x+1] - a[x]
                a32 = _mm256_slli_epi16::<5>(a0); // a[x] * 32
                a32 = _mm256_add_epi16(a32, a16); // a[x] * 32 + 16
                b = _mm256_mullo_epi16(diff, shift);

                res = _mm256_add_epi16(a32, b);
                res = _mm256_srli_epi16::<5>(res);

                base_inc256 = _mm256_setr_epi16(
                    (base + j) as i16,
                    (base + j + 1) as i16,
                    (base + j + 2) as i16,
                    (base + j + 3) as i16,
                    (base + j + 4) as i16,
                    (base + j + 5) as i16,
                    (base + j + 6) as i16,
                    (base + j + 7) as i16,
                    (base + j + 8) as i16,
                    (base + j + 9) as i16,
                    (base + j + 10) as i16,
                    (base + j + 11) as i16,
                    (base + j + 12) as i16,
                    (base + j + 13) as i16,
                    (base + j + 14) as i16,
                    (base + j + 15) as i16,
                );

                mask256 = _mm256_cmpgt_epi16(max_base_x256, base_inc256);
                res = _mm256_blendv_epi8(a_mbase_x, res, mask256);
            }
            if j == 0 {
                *dstvec.add(r as usize) = res;
            } else {
                *dstvec.add((r + N) as usize) = res;
            }
            j += 16;
        }
        x += dx;
    }
}

#[target_feature(enable = "avx2")]
unsafe fn highbd_dr_prediction_z1_32xN_avx2(
    N: i32,
    dst: *mut u16,
    stride: isize,
    above: *const u16,
    upsample_above: i32,
    dx: i32,
    bd: i32,
) {
    let mut dstvec: [__m256i; 128] = core::mem::zeroed();
    if bd < 12 {
        highbd_dr_prediction_z1_32xN_internal_avx2(
            N,
            dstvec.as_mut_ptr(),
            above,
            upsample_above,
            dx,
        );
    } else {
        highbd_dr_prediction_32bit_z1_32xN_internal_avx2(
            N,
            dstvec.as_mut_ptr(),
            above,
            upsample_above,
            dx,
        );
    }
    for i in 0..N {
        _mm256_storeu_si256(
            dst.offset(stride * i as isize) as *mut __m256i,
            dstvec[i as usize],
        );
        _mm256_storeu_si256(
            dst.offset(stride * i as isize + 16) as *mut __m256i,
            dstvec[(i + N) as usize],
        );
    }
}

#[target_feature(enable = "avx2")]
unsafe fn highbd_dr_prediction_32bit_z1_64xN_avx2(
    N: i32,
    mut dst: *mut u16,
    stride: isize,
    above: *const u16,
    upsample_above: i32,
    dx: i32,
) {
    // here upsample_above is 0 by design of av1_use_intra_edge_upsample
    let _ = upsample_above;
    let frac_bits = 6;
    let max_base_x = (64 + N) - 1;

    // pre-filter above pixels
    // store in temp buffers:
    //   above[x] * 32 + 16
    //   above[x+1] - above[x]
    // final pixels will be calculated as:
    //   (above[x] * 32 + 16 + (above[x+1] - above[x]) * shift) >> 5
    let mut a0: __m256i;
    let mut a0_1: __m256i;
    let mut a1: __m256i;
    let mut a1_1: __m256i;
    let mut a32: __m256i;
    let a16: __m256i;
    let a_mbase_x: __m256i;
    let mut diff: __m256i;
    let max_base_x256: __m256i;
    let mut base_inc256: __m256i;
    let mut mask256: __m256i;

    a16 = _mm256_set1_epi32(16);
    a_mbase_x = _mm256_set1_epi16(*above.offset(max_base_x as isize) as i16);
    max_base_x256 = _mm256_set1_epi16(max_base_x as i16);

    let mut x = dx;
    for r in 0..N {
        let mut b: __m256i;
        let mut res: [__m256i; 2] = core::mem::zeroed();
        let mut res1: __m256i;

        let base = x >> frac_bits;
        if base >= max_base_x {
            for _i in r..N {
                _mm256_storeu_si256(dst as *mut __m256i, a_mbase_x); // save 32 values
                _mm256_storeu_si256(dst.add(16) as *mut __m256i, a_mbase_x);
                _mm256_storeu_si256(dst.add(32) as *mut __m256i, a_mbase_x);
                _mm256_storeu_si256(dst.add(48) as *mut __m256i, a_mbase_x);
                dst = dst.offset(stride);
            }
            return;
        }

        let shift = _mm256_srli_epi32::<1>(_mm256_and_si256(
            _mm256_set1_epi32(x),
            _mm256_set1_epi32(0x3f),
        ));

        let mut a0_128: __m128i;
        let mut a0_1_128: __m128i;
        let mut a1_128: __m128i;
        let mut a1_1_128: __m128i;
        let mut j = 0;
        while j < 64 {
            let mdif = max_base_x - (base + j);
            if mdif <= 0 {
                _mm256_storeu_si256(dst.add(j as usize) as *mut __m256i, a_mbase_x);
            } else {
                a0_128 = _mm_loadu_si128(above.offset((base + j) as isize) as *const __m128i);
                a1_128 = _mm_loadu_si128(above.offset((base + 1 + j) as isize) as *const __m128i);
                a0 = _mm256_cvtepu16_epi32(a0_128);
                a1 = _mm256_cvtepu16_epi32(a1_128);

                diff = _mm256_sub_epi32(a1, a0); // a[x+1] - a[x]
                a32 = _mm256_slli_epi32::<5>(a0); // a[x] * 32
                a32 = _mm256_add_epi32(a32, a16); // a[x] * 32 + 16
                b = _mm256_mullo_epi32(diff, shift);

                res[0] = _mm256_add_epi32(a32, b);
                res[0] = _mm256_srli_epi32::<5>(res[0]);
                res[0] = _mm256_packus_epi32(
                    res[0],
                    _mm256_castsi128_si256(_mm256_extracti128_si256::<1>(res[0])),
                );
                if mdif > 8 {
                    a0_1_128 =
                        _mm_loadu_si128(above.offset((base + 8 + j) as isize) as *const __m128i);
                    a1_1_128 =
                        _mm_loadu_si128(above.offset((base + 9 + j) as isize) as *const __m128i);
                    a0_1 = _mm256_cvtepu16_epi32(a0_1_128);
                    a1_1 = _mm256_cvtepu16_epi32(a1_1_128);

                    diff = _mm256_sub_epi32(a1_1, a0_1); // a[x+1] - a[x]
                    a32 = _mm256_slli_epi32::<5>(a0_1); // a[x] * 32
                    a32 = _mm256_add_epi32(a32, a16); // a[x] * 32 + 16
                    b = _mm256_mullo_epi32(diff, shift);

                    res[1] = _mm256_add_epi32(a32, b);
                    res[1] = _mm256_srli_epi32::<5>(res[1]);
                    res[1] = _mm256_packus_epi32(
                        res[1],
                        _mm256_castsi128_si256(_mm256_extracti128_si256::<1>(res[1])),
                    );
                } else {
                    res[1] = a_mbase_x;
                }
                res1 = _mm256_inserti128_si256::<1>(res[0], _mm256_castsi256_si128(res[1])); // 16 16bit values
                base_inc256 = _mm256_setr_epi16(
                    (base + j) as i16,
                    (base + j + 1) as i16,
                    (base + j + 2) as i16,
                    (base + j + 3) as i16,
                    (base + j + 4) as i16,
                    (base + j + 5) as i16,
                    (base + j + 6) as i16,
                    (base + j + 7) as i16,
                    (base + j + 8) as i16,
                    (base + j + 9) as i16,
                    (base + j + 10) as i16,
                    (base + j + 11) as i16,
                    (base + j + 12) as i16,
                    (base + j + 13) as i16,
                    (base + j + 14) as i16,
                    (base + j + 15) as i16,
                );

                mask256 = _mm256_cmpgt_epi16(max_base_x256, base_inc256);
                res1 = _mm256_blendv_epi8(a_mbase_x, res1, mask256);
                _mm256_storeu_si256(dst.add(j as usize) as *mut __m256i, res1);
            }
            j += 16;
        }
        x += dx;
        dst = dst.offset(stride);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn highbd_dr_prediction_z1_64xN_avx2(
    N: i32,
    mut dst: *mut u16,
    stride: isize,
    above: *const u16,
    upsample_above: i32,
    dx: i32,
) {
    // here upsample_above is 0 by design of av1_use_intra_edge_upsample
    let _ = upsample_above;
    let frac_bits = 6;
    let max_base_x = (64 + N) - 1;

    // pre-filter above pixels
    // store in temp buffers:
    //   above[x] * 32 + 16
    //   above[x+1] - above[x]
    // final pixels will be calculated as:
    //   (above[x] * 32 + 16 + (above[x+1] - above[x]) * shift) >> 5
    let mut a0: __m256i;
    let mut a1: __m256i;
    let mut a32: __m256i;
    let a16: __m256i;
    let c3f: __m256i;
    let a_mbase_x: __m256i;
    let mut diff: __m256i;
    let max_base_x256: __m256i;
    let mut base_inc256: __m256i;
    let mut mask256: __m256i;

    a16 = _mm256_set1_epi16(16);
    a_mbase_x = _mm256_set1_epi16(*above.offset(max_base_x as isize) as i16);
    max_base_x256 = _mm256_set1_epi16(max_base_x as i16);
    c3f = _mm256_set1_epi16(0x3f);

    let mut x = dx;
    for r in 0..N {
        let mut b: __m256i;
        let mut res: __m256i;

        let base = x >> frac_bits;
        if base >= max_base_x {
            for _i in r..N {
                _mm256_storeu_si256(dst as *mut __m256i, a_mbase_x); // save 32 values
                _mm256_storeu_si256(dst.add(16) as *mut __m256i, a_mbase_x);
                _mm256_storeu_si256(dst.add(32) as *mut __m256i, a_mbase_x);
                _mm256_storeu_si256(dst.add(48) as *mut __m256i, a_mbase_x);
                dst = dst.offset(stride);
            }
            return;
        }

        let shift = _mm256_srli_epi16::<1>(_mm256_and_si256(_mm256_set1_epi16(x as i16), c3f));

        let mut j = 0;
        while j < 64 {
            let mdif = max_base_x - (base + j);
            if mdif <= 0 {
                _mm256_storeu_si256(dst.add(j as usize) as *mut __m256i, a_mbase_x);
            } else {
                a0 = _mm256_loadu_si256(above.offset((base + j) as isize) as *const __m256i);
                a1 = _mm256_loadu_si256(above.offset((base + 1 + j) as isize) as *const __m256i);

                diff = _mm256_sub_epi16(a1, a0); // a[x+1] - a[x]
                a32 = _mm256_slli_epi16::<5>(a0); // a[x] * 32
                a32 = _mm256_add_epi16(a32, a16); // a[x] * 32 + 16
                b = _mm256_mullo_epi16(diff, shift);

                res = _mm256_add_epi16(a32, b);
                res = _mm256_srli_epi16::<5>(res);

                base_inc256 = _mm256_setr_epi16(
                    (base + j) as i16,
                    (base + j + 1) as i16,
                    (base + j + 2) as i16,
                    (base + j + 3) as i16,
                    (base + j + 4) as i16,
                    (base + j + 5) as i16,
                    (base + j + 6) as i16,
                    (base + j + 7) as i16,
                    (base + j + 8) as i16,
                    (base + j + 9) as i16,
                    (base + j + 10) as i16,
                    (base + j + 11) as i16,
                    (base + j + 12) as i16,
                    (base + j + 13) as i16,
                    (base + j + 14) as i16,
                    (base + j + 15) as i16,
                );

                mask256 = _mm256_cmpgt_epi16(max_base_x256, base_inc256);
                res = _mm256_blendv_epi8(a_mbase_x, res, mask256);
                _mm256_storeu_si256(dst.add(j as usize) as *mut __m256i, res); // 16 16bit values
            }
            j += 16;
        }
        x += dx;
        dst = dst.offset(stride);
    }
}

// Directional prediction, zone 1: 0 < angle < 90
#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn av1_highbd_dr_prediction_z1_avx2(
    dst: *mut u16,
    stride: isize,
    bw: i32,
    bh: i32,
    above: *const u16,
    left: *const u16,
    upsample_above: i32,
    dx: i32,
    dy: i32,
    bd: i32,
) {
    let _ = left;
    let _ = dy;

    match bw {
        4 => highbd_dr_prediction_z1_4xN_avx2(bh, dst, stride, above, upsample_above, dx, bd),
        8 => highbd_dr_prediction_z1_8xN_avx2(bh, dst, stride, above, upsample_above, dx, bd),
        16 => highbd_dr_prediction_z1_16xN_avx2(bh, dst, stride, above, upsample_above, dx, bd),
        32 => highbd_dr_prediction_z1_32xN_avx2(bh, dst, stride, above, upsample_above, dx, bd),
        64 => {
            if bd < 12 {
                highbd_dr_prediction_z1_64xN_avx2(bh, dst, stride, above, upsample_above, dx);
            } else {
                highbd_dr_prediction_32bit_z1_64xN_avx2(bh, dst, stride, above, upsample_above, dx);
            }
        }
        _ => {}
    }
}

#[target_feature(enable = "avx2")]
unsafe fn highbd_transpose_TX_16X16(
    src: *const u16,
    pitchSrc: isize,
    dst: *mut u16,
    pitchDst: isize,
) {
    let mut r: [__m256i; 16] = core::mem::zeroed();
    let mut d: [__m256i; 16] = core::mem::zeroed();
    for j in 0..16 {
        r[j] = _mm256_loadu_si256(src.offset(j as isize * pitchSrc) as *const __m256i);
    }
    highbd_transpose16x16_avx2(r.as_ptr(), d.as_mut_ptr());
    for j in 0..16 {
        _mm256_storeu_si256(dst.offset(j as isize * pitchDst) as *mut __m256i, d[j]);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn highbd_transpose(
    src: *const u16,
    pitchSrc: isize,
    dst: *mut u16,
    pitchDst: isize,
    width: i32,
    height: i32,
) {
    let mut j = 0;
    while j < height {
        let mut i = 0;
        while i < width {
            highbd_transpose_TX_16X16(
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

#[target_feature(enable = "avx2")]
unsafe fn highbd_dr_prediction_32bit_z2_Nx4_avx2(
    N: i32,
    mut dst: *mut u16,
    stride: isize,
    above: *const u16,
    left: *const u16,
    upsample_above: i32,
    upsample_left: i32,
    dx: i32,
    dy: i32,
) {
    let min_base_x = -(1 << upsample_above);
    let min_base_y = -(1 << upsample_left);
    let frac_bits_x = 6 - upsample_above;
    let frac_bits_y = 6 - upsample_left;

    debug_assert!(dx > 0);
    // pre-filter above pixels
    // store in temp buffers:
    //   above[x] * 32 + 16
    //   above[x+1] - above[x]
    // final pixels will be calculated as:
    //   (above[x] * 32 + 16 + (above[x+1] - above[x]) * shift) >> 5
    let mut a0_x: __m256i;
    let mut a1_x: __m256i;
    let mut a32: __m256i;
    let a16: __m256i;
    let mut diff: __m256i;
    let c3f: __m128i;
    let min_base_y128: __m128i;

    a16 = _mm256_set1_epi32(16);
    c3f = _mm_set1_epi32(0x3f);
    min_base_y128 = _mm_set1_epi32(min_base_y);

    for r in 0..N {
        let b: __m256i;
        let mut res: __m256i;
        let mut shift: __m256i;
        let mut resx: __m128i;
        let mut resy: __m128i;
        let resxy: __m128i;
        let mut a0_x128: __m128i;
        let a1_x128: __m128i;
        let y = r + 1;
        let base_x = (-y * dx) >> frac_bits_x;
        let mut base_shift = 0;
        if base_x < (min_base_x - 1) {
            base_shift = (min_base_x - base_x - 1) >> upsample_above;
        }
        let mut base_min_diff = (min_base_x - base_x + upsample_above) >> upsample_above;
        if base_min_diff > 4 {
            base_min_diff = 4;
        } else {
            if base_min_diff < 0 {
                base_min_diff = 0;
            }
        }

        if base_shift > 3 {
            a0_x = _mm256_setzero_si256();
            a1_x = _mm256_setzero_si256();
            shift = _mm256_setzero_si256();
        } else {
            a0_x128 =
                _mm_loadu_si128(above.offset((base_x + base_shift) as isize) as *const __m128i);
            if upsample_above != 0 {
                a0_x128 = _mm_shuffle_epi8(
                    a0_x128,
                    load_mask128(HighbdEvenOddMaskx4.0[base_shift as usize].as_ptr()),
                );
                a1_x128 = _mm_srli_si128::<8>(a0_x128);

                shift = _mm256_castsi128_si256(_mm_srli_epi32::<1>(_mm_and_si128(
                    _mm_sll_epi32(
                        _mm_setr_epi32(
                            -y * dx,
                            (1 << 6) - y * dx,
                            (2 << 6) - y * dx,
                            (3 << 6) - y * dx,
                        ),
                        _mm_cvtsi32_si128(upsample_above),
                    ),
                    c3f,
                )));
            } else {
                a0_x128 = _mm_shuffle_epi8(
                    a0_x128,
                    load_mask128(HighbdLoadMaskx.0[base_shift as usize].as_ptr()),
                );
                a1_x128 = _mm_srli_si128::<2>(a0_x128);

                shift = _mm256_castsi128_si256(_mm_srli_epi32::<1>(_mm_and_si128(
                    _mm_setr_epi32(
                        -y * dx,
                        (1 << 6) - y * dx,
                        (2 << 6) - y * dx,
                        (3 << 6) - y * dx,
                    ),
                    c3f,
                )));
            }
            a0_x = _mm256_cvtepu16_epi32(a0_x128);
            a1_x = _mm256_cvtepu16_epi32(a1_x128);
        }
        // y calc
        let a0_y: __m128i;
        let a1_y: __m128i;
        let shifty: __m128i;
        if base_x < min_base_x {
            let r6: __m128i;
            let c1234: __m128i;
            let dy128: __m128i;
            let y_c128: __m128i;
            let mut base_y_c128: __m128i;
            let mask128: __m128i;
            let mut base_y_c: Aligned32<[i32; 4]> = core::mem::zeroed();
            r6 = _mm_set1_epi32(r << 6);
            dy128 = _mm_set1_epi32(dy);
            c1234 = _mm_setr_epi32(1, 2, 3, 4);
            y_c128 = _mm_sub_epi32(r6, _mm_mullo_epi32(c1234, dy128));
            base_y_c128 = _mm_sra_epi32(y_c128, _mm_cvtsi32_si128(frac_bits_y));
            mask128 = _mm_cmpgt_epi32(min_base_y128, base_y_c128);
            base_y_c128 = _mm_andnot_si128(mask128, base_y_c128);
            _mm_store_si128(base_y_c.0.as_mut_ptr() as *mut __m128i, base_y_c128);
            let base_y_c = &base_y_c.0;

            a0_y = _mm_setr_epi32(
                *left.offset(base_y_c[0] as isize) as i32,
                *left.offset(base_y_c[1] as isize) as i32,
                *left.offset(base_y_c[2] as isize) as i32,
                *left.offset(base_y_c[3] as isize) as i32,
            );
            a1_y = _mm_setr_epi32(
                *left.offset(base_y_c[0] as isize + 1) as i32,
                *left.offset(base_y_c[1] as isize + 1) as i32,
                *left.offset(base_y_c[2] as isize + 1) as i32,
                *left.offset(base_y_c[3] as isize + 1) as i32,
            );

            if upsample_left != 0 {
                shifty = _mm_srli_epi32::<1>(_mm_and_si128(
                    _mm_sll_epi32(y_c128, _mm_cvtsi32_si128(upsample_left)),
                    c3f,
                ));
            } else {
                shifty = _mm_srli_epi32::<1>(_mm_and_si128(y_c128, c3f));
            }
            a0_x = _mm256_inserti128_si256::<1>(a0_x, a0_y);
            a1_x = _mm256_inserti128_si256::<1>(a1_x, a1_y);
            shift = _mm256_inserti128_si256::<1>(shift, shifty);
        }

        diff = _mm256_sub_epi32(a1_x, a0_x); // a[x+1] - a[x]
        a32 = _mm256_slli_epi32::<5>(a0_x); // a[x] * 32
        a32 = _mm256_add_epi32(a32, a16); // a[x] * 32 + 16

        b = _mm256_mullo_epi32(diff, shift);
        res = _mm256_add_epi32(a32, b);
        res = _mm256_srli_epi32::<5>(res);

        resx = _mm256_castsi256_si128(res);
        resx = _mm_packus_epi32(resx, resx);

        resy = _mm256_extracti128_si256::<1>(res);
        resy = _mm_packus_epi32(resy, resy);

        resxy = _mm_blendv_epi8(resx, resy, base_mask128(base_min_diff));
        _mm_storel_epi64(dst as *mut __m128i, resxy);
        dst = dst.offset(stride);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn highbd_dr_prediction_z2_Nx4_avx2(
    N: i32,
    mut dst: *mut u16,
    stride: isize,
    above: *const u16,
    left: *const u16,
    upsample_above: i32,
    upsample_left: i32,
    dx: i32,
    dy: i32,
) {
    let min_base_x = -(1 << upsample_above);
    let min_base_y = -(1 << upsample_left);
    let frac_bits_x = 6 - upsample_above;
    let frac_bits_y = 6 - upsample_left;

    debug_assert!(dx > 0);
    // pre-filter above pixels
    // store in temp buffers:
    //   above[x] * 32 + 16
    //   above[x+1] - above[x]
    // final pixels will be calculated as:
    //   (above[x] * 32 + 16 + (above[x+1] - above[x]) * shift) >> 5
    let mut a0_x: __m256i;
    let mut a1_x: __m256i;
    let mut a32: __m256i;
    let a16: __m256i;
    let mut diff: __m256i;
    let c3f: __m128i;
    let min_base_y128: __m128i;

    a16 = _mm256_set1_epi16(16);
    c3f = _mm_set1_epi16(0x3f);
    min_base_y128 = _mm_set1_epi16(min_base_y as i16);

    for r in 0..N {
        let b: __m256i;
        let mut res: __m256i;
        let mut shift: __m256i;
        let resx: __m128i;
        let resy: __m128i;
        let resxy: __m128i;
        let mut a0_x128: __m128i;
        let a1_x128: __m128i;
        let y = r + 1;
        let base_x = (-y * dx) >> frac_bits_x;
        let mut base_shift = 0;
        if base_x < (min_base_x - 1) {
            base_shift = (min_base_x - base_x - 1) >> upsample_above;
        }
        let mut base_min_diff = (min_base_x - base_x + upsample_above) >> upsample_above;
        if base_min_diff > 4 {
            base_min_diff = 4;
        } else {
            if base_min_diff < 0 {
                base_min_diff = 0;
            }
        }

        if base_shift > 3 {
            a0_x = _mm256_setzero_si256();
            a1_x = _mm256_setzero_si256();
            shift = _mm256_setzero_si256();
        } else {
            a0_x128 =
                _mm_loadu_si128(above.offset((base_x + base_shift) as isize) as *const __m128i);
            if upsample_above != 0 {
                a0_x128 = _mm_shuffle_epi8(
                    a0_x128,
                    load_mask128(HighbdEvenOddMaskx4.0[base_shift as usize].as_ptr()),
                );
                a1_x128 = _mm_srli_si128::<8>(a0_x128);

                shift = _mm256_castsi128_si256(_mm_srli_epi16::<1>(_mm_and_si128(
                    _mm_sll_epi16(
                        _mm_setr_epi16(
                            (-y * dx) as i16,
                            ((1 << 6) - y * dx) as i16,
                            ((2 << 6) - y * dx) as i16,
                            ((3 << 6) - y * dx) as i16,
                            0,
                            0,
                            0,
                            0,
                        ),
                        _mm_cvtsi32_si128(upsample_above),
                    ),
                    c3f,
                )));
            } else {
                a0_x128 = _mm_shuffle_epi8(
                    a0_x128,
                    load_mask128(HighbdLoadMaskx.0[base_shift as usize].as_ptr()),
                );
                a1_x128 = _mm_srli_si128::<2>(a0_x128);

                shift = _mm256_castsi128_si256(_mm_srli_epi16::<1>(_mm_and_si128(
                    _mm_setr_epi16(
                        (-y * dx) as i16,
                        ((1 << 6) - y * dx) as i16,
                        ((2 << 6) - y * dx) as i16,
                        ((3 << 6) - y * dx) as i16,
                        0,
                        0,
                        0,
                        0,
                    ),
                    c3f,
                )));
            }
            a0_x = _mm256_castsi128_si256(a0_x128);
            a1_x = _mm256_castsi128_si256(a1_x128);
        }
        // y calc
        let a0_y: __m128i;
        let a1_y: __m128i;
        let shifty: __m128i;
        if base_x < min_base_x {
            let r6: __m128i;
            let c1234: __m128i;
            let dy128: __m128i;
            let y_c128: __m128i;
            let mut base_y_c128: __m128i;
            let mask128: __m128i;
            let mut base_y_c: Aligned32<[i16; 8]> = core::mem::zeroed();
            r6 = _mm_set1_epi16((r << 6) as i16);
            dy128 = _mm_set1_epi16(dy as i16);
            c1234 = _mm_setr_epi16(1, 2, 3, 4, 0, 0, 0, 0);
            y_c128 = _mm_sub_epi16(r6, _mm_mullo_epi16(c1234, dy128));
            base_y_c128 = _mm_sra_epi16(y_c128, _mm_cvtsi32_si128(frac_bits_y));
            mask128 = _mm_cmpgt_epi16(min_base_y128, base_y_c128);
            base_y_c128 = _mm_andnot_si128(mask128, base_y_c128);
            _mm_store_si128(base_y_c.0.as_mut_ptr() as *mut __m128i, base_y_c128);
            let base_y_c = &base_y_c.0;

            a0_y = _mm_setr_epi16(
                *left.offset(base_y_c[0] as isize) as i16,
                *left.offset(base_y_c[1] as isize) as i16,
                *left.offset(base_y_c[2] as isize) as i16,
                *left.offset(base_y_c[3] as isize) as i16,
                0,
                0,
                0,
                0,
            );
            a1_y = _mm_setr_epi16(
                *left.offset(base_y_c[0] as isize + 1) as i16,
                *left.offset(base_y_c[1] as isize + 1) as i16,
                *left.offset(base_y_c[2] as isize + 1) as i16,
                *left.offset(base_y_c[3] as isize + 1) as i16,
                0,
                0,
                0,
                0,
            );

            if upsample_left != 0 {
                shifty = _mm_srli_epi16::<1>(_mm_and_si128(
                    _mm_sll_epi16(y_c128, _mm_cvtsi32_si128(upsample_left)),
                    c3f,
                ));
            } else {
                shifty = _mm_srli_epi16::<1>(_mm_and_si128(y_c128, c3f));
            }
            a0_x = _mm256_inserti128_si256::<1>(a0_x, a0_y);
            a1_x = _mm256_inserti128_si256::<1>(a1_x, a1_y);
            shift = _mm256_inserti128_si256::<1>(shift, shifty);
        }

        diff = _mm256_sub_epi16(a1_x, a0_x); // a[x+1] - a[x]
        a32 = _mm256_slli_epi16::<5>(a0_x); // a[x] * 32
        a32 = _mm256_add_epi16(a32, a16); // a[x] * 32 + 16

        b = _mm256_mullo_epi16(diff, shift);
        res = _mm256_add_epi16(a32, b);
        res = _mm256_srli_epi16::<5>(res);

        resx = _mm256_castsi256_si128(res);
        resy = _mm256_extracti128_si256::<1>(res);
        resxy = _mm_blendv_epi8(resx, resy, base_mask128(base_min_diff));
        _mm_storel_epi64(dst as *mut __m128i, resxy);
        dst = dst.offset(stride);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn highbd_dr_prediction_32bit_z2_Nx8_avx2(
    N: i32,
    mut dst: *mut u16,
    stride: isize,
    above: *const u16,
    left: *const u16,
    upsample_above: i32,
    upsample_left: i32,
    dx: i32,
    dy: i32,
) {
    let min_base_x = -(1 << upsample_above);
    let min_base_y = -(1 << upsample_left);
    let frac_bits_x = 6 - upsample_above;
    let frac_bits_y = 6 - upsample_left;

    // pre-filter above pixels
    // store in temp buffers:
    //   above[x] * 32 + 16
    //   above[x+1] - above[x]
    // final pixels will be calculated as:
    //   (above[x] * 32 + 16 + (above[x+1] - above[x]) * shift) >> 5
    let mut a0_x: __m256i;
    let mut a1_x: __m256i;
    let mut a0_y: __m256i;
    let mut a1_y: __m256i;
    let mut a32: __m256i;
    let a16: __m256i;
    let c3f: __m256i;
    let min_base_y256: __m256i;
    let mut diff: __m256i;
    let mut a0_x128: __m128i;
    let mut a1_x128: __m128i;

    a16 = _mm256_set1_epi32(16);
    c3f = _mm256_set1_epi32(0x3f);
    min_base_y256 = _mm256_set1_epi32(min_base_y);

    for r in 0..N {
        let mut b: __m256i;
        let mut res: __m256i;
        let mut shift: __m256i;
        let resx: __m128i;
        let resy: __m128i;
        let resxy: __m128i;
        let y = r + 1;
        let base_x = (-y * dx) >> frac_bits_x;
        let mut base_shift = 0;
        if base_x < (min_base_x - 1) {
            base_shift = (min_base_x - base_x - 1) >> upsample_above;
        }
        let mut base_min_diff = (min_base_x - base_x + upsample_above) >> upsample_above;
        if base_min_diff > 8 {
            base_min_diff = 8;
        } else {
            if base_min_diff < 0 {
                base_min_diff = 0;
            }
        }

        if base_shift > 7 {
            resx = _mm_setzero_si128();
        } else {
            a0_x128 =
                _mm_loadu_si128(above.offset((base_x + base_shift) as isize) as *const __m128i);
            if upsample_above != 0 {
                let mut mask: __m128i;
                let atmp0: __m128i;
                let atmp1: __m128i;
                let atmp2: __m128i;
                let atmp3: __m128i;
                let mask_row = HighbdEvenOddMaskx.0[base_shift as usize].as_ptr();
                a1_x128 = _mm_loadu_si128(
                    above.offset((base_x + 8 + base_shift) as isize) as *const __m128i
                );
                atmp0 = _mm_shuffle_epi8(a0_x128, load_mask128(mask_row));
                atmp1 = _mm_shuffle_epi8(a1_x128, load_mask128(mask_row));
                atmp2 = _mm_shuffle_epi8(a0_x128, load_mask128(mask_row.add(16)));
                atmp3 = _mm_shuffle_epi8(a1_x128, load_mask128(mask_row.add(16)));
                mask = _mm_cmpgt_epi8(load_mask128(mask_row), _mm_set1_epi8(15));
                a0_x128 = _mm_blendv_epi8(atmp0, atmp1, mask);
                mask = _mm_cmpgt_epi8(load_mask128(mask_row.add(16)), _mm_set1_epi8(15));
                a1_x128 = _mm_blendv_epi8(atmp2, atmp3, mask);
                shift = _mm256_srli_epi32::<1>(_mm256_and_si256(
                    _mm256_sll_epi32(
                        _mm256_setr_epi32(
                            -y * dx,
                            (1 << 6) - y * dx,
                            (2 << 6) - y * dx,
                            (3 << 6) - y * dx,
                            (4 << 6) - y * dx,
                            (5 << 6) - y * dx,
                            (6 << 6) - y * dx,
                            (7 << 6) - y * dx,
                        ),
                        _mm_cvtsi32_si128(upsample_above),
                    ),
                    c3f,
                ));
            } else {
                a1_x128 = _mm_loadu_si128(
                    above.offset((base_x + 1 + base_shift) as isize) as *const __m128i
                );
                a0_x128 = _mm_shuffle_epi8(
                    a0_x128,
                    load_mask128(HighbdLoadMaskx.0[base_shift as usize].as_ptr()),
                );
                a1_x128 = _mm_shuffle_epi8(
                    a1_x128,
                    load_mask128(HighbdLoadMaskx.0[base_shift as usize].as_ptr()),
                );

                shift = _mm256_srli_epi32::<1>(_mm256_and_si256(
                    _mm256_setr_epi32(
                        -y * dx,
                        (1 << 6) - y * dx,
                        (2 << 6) - y * dx,
                        (3 << 6) - y * dx,
                        (4 << 6) - y * dx,
                        (5 << 6) - y * dx,
                        (6 << 6) - y * dx,
                        (7 << 6) - y * dx,
                    ),
                    c3f,
                ));
            }
            a0_x = _mm256_cvtepu16_epi32(a0_x128);
            a1_x = _mm256_cvtepu16_epi32(a1_x128);

            diff = _mm256_sub_epi32(a1_x, a0_x); // a[x+1] - a[x]
            a32 = _mm256_slli_epi32::<5>(a0_x); // a[x] * 32
            a32 = _mm256_add_epi32(a32, a16); // a[x] * 32 + 16

            b = _mm256_mullo_epi32(diff, shift);
            res = _mm256_add_epi32(a32, b);
            res = _mm256_srli_epi32::<5>(res);

            resx = _mm256_castsi256_si128(_mm256_packus_epi32(
                res,
                _mm256_castsi128_si256(_mm256_extracti128_si256::<1>(res)),
            ));
        }
        // y calc
        if base_x < min_base_x {
            let mut base_y_c: Aligned32<[i32; 8]> = core::mem::zeroed();
            let r6: __m256i;
            let c256: __m256i;
            let dy256: __m256i;
            let y_c256: __m256i;
            let mut base_y_c256: __m256i;
            let mask256: __m256i;
            r6 = _mm256_set1_epi32(r << 6);
            dy256 = _mm256_set1_epi32(dy);
            c256 = _mm256_setr_epi32(1, 2, 3, 4, 5, 6, 7, 8);
            y_c256 = _mm256_sub_epi32(r6, _mm256_mullo_epi32(c256, dy256));
            base_y_c256 = _mm256_sra_epi32(y_c256, _mm_cvtsi32_si128(frac_bits_y));
            mask256 = _mm256_cmpgt_epi32(min_base_y256, base_y_c256);
            base_y_c256 = _mm256_andnot_si256(mask256, base_y_c256);
            _mm256_store_si256(base_y_c.0.as_mut_ptr() as *mut __m256i, base_y_c256);
            let base_y_c = &base_y_c.0;

            a0_y = _mm256_cvtepu16_epi32(_mm_setr_epi16(
                *left.offset(base_y_c[0] as isize) as i16,
                *left.offset(base_y_c[1] as isize) as i16,
                *left.offset(base_y_c[2] as isize) as i16,
                *left.offset(base_y_c[3] as isize) as i16,
                *left.offset(base_y_c[4] as isize) as i16,
                *left.offset(base_y_c[5] as isize) as i16,
                *left.offset(base_y_c[6] as isize) as i16,
                *left.offset(base_y_c[7] as isize) as i16,
            ));
            a1_y = _mm256_cvtepu16_epi32(_mm_setr_epi16(
                *left.offset(base_y_c[0] as isize + 1) as i16,
                *left.offset(base_y_c[1] as isize + 1) as i16,
                *left.offset(base_y_c[2] as isize + 1) as i16,
                *left.offset(base_y_c[3] as isize + 1) as i16,
                *left.offset(base_y_c[4] as isize + 1) as i16,
                *left.offset(base_y_c[5] as isize + 1) as i16,
                *left.offset(base_y_c[6] as isize + 1) as i16,
                *left.offset(base_y_c[7] as isize + 1) as i16,
            ));

            if upsample_left != 0 {
                shift = _mm256_srli_epi32::<1>(_mm256_and_si256(
                    _mm256_sll_epi32(y_c256, _mm_cvtsi32_si128(upsample_left)),
                    c3f,
                ));
            } else {
                shift = _mm256_srli_epi32::<1>(_mm256_and_si256(y_c256, c3f));
            }
            diff = _mm256_sub_epi32(a1_y, a0_y); // a[x+1] - a[x]
            a32 = _mm256_slli_epi32::<5>(a0_y); // a[x] * 32
            a32 = _mm256_add_epi32(a32, a16); // a[x] * 32 + 16

            b = _mm256_mullo_epi32(diff, shift);
            res = _mm256_add_epi32(a32, b);
            res = _mm256_srli_epi32::<5>(res);

            resy = _mm256_castsi256_si128(_mm256_packus_epi32(
                res,
                _mm256_castsi128_si256(_mm256_extracti128_si256::<1>(res)),
            ));
        } else {
            resy = resx;
        }
        resxy = _mm_blendv_epi8(resx, resy, base_mask128(base_min_diff));
        _mm_storeu_si128(dst as *mut __m128i, resxy);
        dst = dst.offset(stride);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn highbd_dr_prediction_z2_Nx8_avx2(
    N: i32,
    mut dst: *mut u16,
    stride: isize,
    above: *const u16,
    left: *const u16,
    upsample_above: i32,
    upsample_left: i32,
    dx: i32,
    dy: i32,
) {
    let min_base_x = -(1 << upsample_above);
    let min_base_y = -(1 << upsample_left);
    let frac_bits_x = 6 - upsample_above;
    let frac_bits_y = 6 - upsample_left;

    // pre-filter above pixels
    // store in temp buffers:
    //   above[x] * 32 + 16
    //   above[x+1] - above[x]
    // final pixels will be calculated as:
    //   (above[x] * 32 + 16 + (above[x+1] - above[x]) * shift) >> 5
    let c3f: __m128i;
    let min_base_y128: __m128i;
    let mut a0_x: __m256i;
    let mut a1_x: __m256i;
    let mut diff: __m256i;
    let mut a32: __m256i;
    let a16: __m256i;
    let mut a0_x128: __m128i;
    let mut a1_x128: __m128i;

    a16 = _mm256_set1_epi16(16);
    c3f = _mm_set1_epi16(0x3f);
    min_base_y128 = _mm_set1_epi16(min_base_y as i16);

    for r in 0..N {
        let b: __m256i;
        let mut res: __m256i;
        let mut shift: __m256i;
        let resx: __m128i;
        let resy: __m128i;
        let resxy: __m128i;
        let y = r + 1;
        let base_x = (-y * dx) >> frac_bits_x;
        let mut base_shift = 0;
        if base_x < (min_base_x - 1) {
            base_shift = (min_base_x - base_x - 1) >> upsample_above;
        }
        let mut base_min_diff = (min_base_x - base_x + upsample_above) >> upsample_above;
        if base_min_diff > 8 {
            base_min_diff = 8;
        } else {
            if base_min_diff < 0 {
                base_min_diff = 0;
            }
        }

        if base_shift > 7 {
            a0_x = _mm256_setzero_si256();
            a1_x = _mm256_setzero_si256();
            shift = _mm256_setzero_si256();
        } else {
            a0_x128 =
                _mm_loadu_si128(above.offset((base_x + base_shift) as isize) as *const __m128i);
            if upsample_above != 0 {
                let mut mask: __m128i;
                let atmp0: __m128i;
                let atmp1: __m128i;
                let atmp2: __m128i;
                let atmp3: __m128i;
                let mask_row = HighbdEvenOddMaskx.0[base_shift as usize].as_ptr();
                a1_x128 = _mm_loadu_si128(
                    above.offset((base_x + 8 + base_shift) as isize) as *const __m128i
                );
                atmp0 = _mm_shuffle_epi8(a0_x128, load_mask128(mask_row));
                atmp1 = _mm_shuffle_epi8(a1_x128, load_mask128(mask_row));
                atmp2 = _mm_shuffle_epi8(a0_x128, load_mask128(mask_row.add(16)));
                atmp3 = _mm_shuffle_epi8(a1_x128, load_mask128(mask_row.add(16)));
                mask = _mm_cmpgt_epi8(load_mask128(mask_row), _mm_set1_epi8(15));
                a0_x128 = _mm_blendv_epi8(atmp0, atmp1, mask);
                mask = _mm_cmpgt_epi8(load_mask128(mask_row.add(16)), _mm_set1_epi8(15));
                a1_x128 = _mm_blendv_epi8(atmp2, atmp3, mask);

                shift = _mm256_castsi128_si256(_mm_srli_epi16::<1>(_mm_and_si128(
                    _mm_sll_epi16(
                        _mm_setr_epi16(
                            (-y * dx) as i16,
                            ((1 << 6) - y * dx) as i16,
                            ((2 << 6) - y * dx) as i16,
                            ((3 << 6) - y * dx) as i16,
                            ((4 << 6) - y * dx) as i16,
                            ((5 << 6) - y * dx) as i16,
                            ((6 << 6) - y * dx) as i16,
                            ((7 << 6) - y * dx) as i16,
                        ),
                        _mm_cvtsi32_si128(upsample_above),
                    ),
                    c3f,
                )));
            } else {
                a1_x128 = _mm_loadu_si128(
                    above.offset((base_x + 1 + base_shift) as isize) as *const __m128i
                );
                a0_x128 = _mm_shuffle_epi8(
                    a0_x128,
                    load_mask128(HighbdLoadMaskx.0[base_shift as usize].as_ptr()),
                );
                a1_x128 = _mm_shuffle_epi8(
                    a1_x128,
                    load_mask128(HighbdLoadMaskx.0[base_shift as usize].as_ptr()),
                );

                shift = _mm256_castsi128_si256(_mm_srli_epi16::<1>(_mm_and_si128(
                    _mm_setr_epi16(
                        (-y * dx) as i16,
                        ((1 << 6) - y * dx) as i16,
                        ((2 << 6) - y * dx) as i16,
                        ((3 << 6) - y * dx) as i16,
                        ((4 << 6) - y * dx) as i16,
                        ((5 << 6) - y * dx) as i16,
                        ((6 << 6) - y * dx) as i16,
                        ((7 << 6) - y * dx) as i16,
                    ),
                    c3f,
                )));
            }
            a0_x = _mm256_castsi128_si256(a0_x128);
            a1_x = _mm256_castsi128_si256(a1_x128);
        }

        // y calc
        let a0_y: __m128i;
        let a1_y: __m128i;
        let shifty: __m128i;
        if base_x < min_base_x {
            let mut base_y_c: Aligned32<[i16; 8]> = core::mem::zeroed();
            let r6: __m128i;
            let c1234: __m128i;
            let dy128: __m128i;
            let y_c128: __m128i;
            let mut base_y_c128: __m128i;
            let mask128: __m128i;
            r6 = _mm_set1_epi16((r << 6) as i16);
            dy128 = _mm_set1_epi16(dy as i16);
            c1234 = _mm_setr_epi16(1, 2, 3, 4, 5, 6, 7, 8);
            y_c128 = _mm_sub_epi16(r6, _mm_mullo_epi16(c1234, dy128));
            base_y_c128 = _mm_sra_epi16(y_c128, _mm_cvtsi32_si128(frac_bits_y));
            mask128 = _mm_cmpgt_epi16(min_base_y128, base_y_c128);
            base_y_c128 = _mm_andnot_si128(mask128, base_y_c128);
            _mm_store_si128(base_y_c.0.as_mut_ptr() as *mut __m128i, base_y_c128);
            let base_y_c = &base_y_c.0;

            a0_y = _mm_setr_epi16(
                *left.offset(base_y_c[0] as isize) as i16,
                *left.offset(base_y_c[1] as isize) as i16,
                *left.offset(base_y_c[2] as isize) as i16,
                *left.offset(base_y_c[3] as isize) as i16,
                *left.offset(base_y_c[4] as isize) as i16,
                *left.offset(base_y_c[5] as isize) as i16,
                *left.offset(base_y_c[6] as isize) as i16,
                *left.offset(base_y_c[7] as isize) as i16,
            );
            a1_y = _mm_setr_epi16(
                *left.offset(base_y_c[0] as isize + 1) as i16,
                *left.offset(base_y_c[1] as isize + 1) as i16,
                *left.offset(base_y_c[2] as isize + 1) as i16,
                *left.offset(base_y_c[3] as isize + 1) as i16,
                *left.offset(base_y_c[4] as isize + 1) as i16,
                *left.offset(base_y_c[5] as isize + 1) as i16,
                *left.offset(base_y_c[6] as isize + 1) as i16,
                *left.offset(base_y_c[7] as isize + 1) as i16,
            );

            if upsample_left != 0 {
                shifty = _mm_srli_epi16::<1>(_mm_and_si128(
                    _mm_sll_epi16(y_c128, _mm_cvtsi32_si128(upsample_left)),
                    c3f,
                ));
            } else {
                shifty = _mm_srli_epi16::<1>(_mm_and_si128(y_c128, c3f));
            }
            a0_x = _mm256_inserti128_si256::<1>(a0_x, a0_y);
            a1_x = _mm256_inserti128_si256::<1>(a1_x, a1_y);
            shift = _mm256_inserti128_si256::<1>(shift, shifty);
        }

        diff = _mm256_sub_epi16(a1_x, a0_x); // a[x+1] - a[x]
        a32 = _mm256_slli_epi16::<5>(a0_x); // a[x] * 32
        a32 = _mm256_add_epi16(a32, a16); // a[x] * 32 + 16

        b = _mm256_mullo_epi16(diff, shift);
        res = _mm256_add_epi16(a32, b);
        res = _mm256_srli_epi16::<5>(res);

        resx = _mm256_castsi256_si128(res);
        resy = _mm256_extracti128_si256::<1>(res);

        resxy = _mm_blendv_epi8(resx, resy, base_mask128(base_min_diff));
        _mm_storeu_si128(dst as *mut __m128i, resxy);
        dst = dst.offset(stride);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn highbd_dr_prediction_32bit_z2_HxW_avx2(
    H: i32,
    W: i32,
    mut dst: *mut u16,
    stride: isize,
    above: *const u16,
    left: *const u16,
    upsample_above: i32,
    upsample_left: i32,
    dx: i32,
    dy: i32,
) {
    // here upsample_above and upsample_left are 0 by design of
    // av1_use_intra_edge_upsample
    let min_base_x = -1;
    let min_base_y = -1;
    let _ = upsample_above;
    let _ = upsample_left;
    let frac_bits_x = 6;
    const frac_bits_y: i32 = 6;

    // pre-filter above pixels
    // store in temp buffers:
    //   above[x] * 32 + 16
    //   above[x+1] - above[x]
    // final pixels will be calculated as:
    //   (above[x] * 32 + 16 + (above[x+1] - above[x]) * shift) >> 5
    let mut a0_x: __m256i;
    let mut a1_x: __m256i;
    let mut a0_y: __m256i;
    let mut a1_y: __m256i;
    let mut a32: __m256i;
    let mut a0_1_x: __m256i;
    let mut a1_1_x: __m256i;
    let a16: __m256i;
    let c1: __m256i;
    let mut diff: __m256i;
    let min_base_y256: __m256i;
    let c3f: __m256i;
    let dy256: __m256i;
    let c1234: __m256i;
    let c0123: __m256i;
    let c8: __m256i;
    let mut a0_x128: __m128i;
    let mut a1_x128: __m128i;
    let mut a0_1_x128: __m128i;
    let mut a1_1_x128: __m128i;
    let mut base_y_c: Aligned32<[i32; 16]> = core::mem::zeroed();

    a16 = _mm256_set1_epi32(16);
    c1 = _mm256_srli_epi32::<4>(a16);
    c8 = _mm256_srli_epi32::<1>(a16);
    min_base_y256 = _mm256_set1_epi32(min_base_y);
    c3f = _mm256_set1_epi32(0x3f);
    dy256 = _mm256_set1_epi32(dy);
    c0123 = _mm256_setr_epi32(0, 1, 2, 3, 4, 5, 6, 7);
    c1234 = _mm256_add_epi32(c0123, c1);

    for r in 0..H {
        let mut b: __m256i;
        let mut res: __m256i;
        let mut shift: __m256i;
        let mut ydx: __m256i;
        let mut resx: [__m256i; 2] = core::mem::zeroed();
        let mut resy: [__m256i; 2] = core::mem::zeroed();
        let mut resxy: __m256i;
        let mut j256: __m256i;
        let mut r6: __m256i;
        let mut j = 0;
        while j < W {
            j256 = _mm256_set1_epi32(j);
            let y = r + 1;
            ydx = _mm256_set1_epi32(y * dx);

            let base_x = ((j << 6) - y * dx) >> frac_bits_x;
            let mut base_shift = 0;
            if (base_x) < (min_base_x - 1) {
                base_shift = min_base_x - base_x - 1;
            }
            let mut base_min_diff = min_base_x - base_x;
            if base_min_diff > 16 {
                base_min_diff = 16;
            } else {
                if base_min_diff < 0 {
                    base_min_diff = 0;
                }
            }

            if base_shift > 7 {
                resx[0] = _mm256_setzero_si256();
            } else {
                a0_x128 =
                    _mm_loadu_si128(above.offset((base_x + base_shift) as isize) as *const __m128i);
                a1_x128 = _mm_loadu_si128(
                    above.offset((base_x + base_shift + 1) as isize) as *const __m128i
                );
                a0_x128 = _mm_shuffle_epi8(
                    a0_x128,
                    load_mask128(HighbdLoadMaskx.0[base_shift as usize].as_ptr()),
                );
                a1_x128 = _mm_shuffle_epi8(
                    a1_x128,
                    load_mask128(HighbdLoadMaskx.0[base_shift as usize].as_ptr()),
                );

                a0_x = _mm256_cvtepu16_epi32(a0_x128);
                a1_x = _mm256_cvtepu16_epi32(a1_x128);

                r6 = _mm256_slli_epi32::<6>(_mm256_add_epi32(c0123, j256));
                shift = _mm256_srli_epi32::<1>(_mm256_and_si256(_mm256_sub_epi32(r6, ydx), c3f));

                diff = _mm256_sub_epi32(a1_x, a0_x); // a[x+1] - a[x]
                a32 = _mm256_slli_epi32::<5>(a0_x); // a[x] * 32
                a32 = _mm256_add_epi32(a32, a16); // a[x] * 32 + 16

                b = _mm256_mullo_epi32(diff, shift);
                res = _mm256_add_epi32(a32, b);
                res = _mm256_srli_epi32::<5>(res);

                resx[0] = _mm256_packus_epi32(
                    res,
                    _mm256_castsi128_si256(_mm256_extracti128_si256::<1>(res)),
                );
            }
            let mut base_shift8 = 0;
            if (base_x + 8) < (min_base_x - 1) {
                base_shift8 = min_base_x - (base_x + 8) - 1;
            }
            if base_shift8 > 7 {
                resx[1] = _mm256_setzero_si256();
            } else {
                a0_1_x128 = _mm_loadu_si128(
                    above.offset((base_x + base_shift8 + 8) as isize) as *const __m128i
                );
                a1_1_x128 = _mm_loadu_si128(
                    above.offset((base_x + base_shift8 + 9) as isize) as *const __m128i
                );
                a0_1_x128 = _mm_shuffle_epi8(
                    a0_1_x128,
                    load_mask128(HighbdLoadMaskx.0[base_shift8 as usize].as_ptr()),
                );
                a1_1_x128 = _mm_shuffle_epi8(
                    a1_1_x128,
                    load_mask128(HighbdLoadMaskx.0[base_shift8 as usize].as_ptr()),
                );

                a0_1_x = _mm256_cvtepu16_epi32(a0_1_x128);
                a1_1_x = _mm256_cvtepu16_epi32(a1_1_x128);

                r6 = _mm256_slli_epi32::<6>(_mm256_add_epi32(c0123, _mm256_add_epi32(j256, c8)));
                shift = _mm256_srli_epi32::<1>(_mm256_and_si256(_mm256_sub_epi32(r6, ydx), c3f));

                diff = _mm256_sub_epi32(a1_1_x, a0_1_x); // a[x+1] - a[x]
                a32 = _mm256_slli_epi32::<5>(a0_1_x); // a[x] * 32
                a32 = _mm256_add_epi32(a32, a16); // a[x] * 32 + 16
                b = _mm256_mullo_epi32(diff, shift);

                resx[1] = _mm256_add_epi32(a32, b);
                resx[1] = _mm256_srli_epi32::<5>(resx[1]);
                resx[1] = _mm256_packus_epi32(
                    resx[1],
                    _mm256_castsi128_si256(_mm256_extracti128_si256::<1>(resx[1])),
                );
            }
            resx[0] = _mm256_inserti128_si256::<1>(resx[0], _mm256_castsi256_si128(resx[1])); // 16 16bit values

            // y calc
            resy[0] = _mm256_setzero_si256();
            if base_x < min_base_x {
                let mut c256: __m256i;
                let y_c256: __m256i;
                let y_c_1_256: __m256i;
                let mut base_y_c256: __m256i;
                let mut mask256: __m256i;
                r6 = _mm256_set1_epi32(r << 6);
                c256 = _mm256_add_epi32(j256, c1234);
                y_c256 = _mm256_sub_epi32(r6, _mm256_mullo_epi32(c256, dy256));
                base_y_c256 = _mm256_srai_epi32::<frac_bits_y>(y_c256);
                mask256 = _mm256_cmpgt_epi32(min_base_y256, base_y_c256);
                base_y_c256 = _mm256_andnot_si256(mask256, base_y_c256);
                _mm256_store_si256(base_y_c.0.as_mut_ptr() as *mut __m256i, base_y_c256);
                c256 = _mm256_add_epi32(c256, c8);
                y_c_1_256 = _mm256_sub_epi32(r6, _mm256_mullo_epi32(c256, dy256));
                base_y_c256 = _mm256_srai_epi32::<frac_bits_y>(y_c_1_256);
                mask256 = _mm256_cmpgt_epi32(min_base_y256, base_y_c256);
                base_y_c256 = _mm256_andnot_si256(mask256, base_y_c256);
                _mm256_store_si256(base_y_c.0.as_mut_ptr().add(8) as *mut __m256i, base_y_c256);
                let bc = &base_y_c.0;

                a0_y = _mm256_cvtepu16_epi32(_mm_setr_epi16(
                    *left.offset(bc[0] as isize) as i16,
                    *left.offset(bc[1] as isize) as i16,
                    *left.offset(bc[2] as isize) as i16,
                    *left.offset(bc[3] as isize) as i16,
                    *left.offset(bc[4] as isize) as i16,
                    *left.offset(bc[5] as isize) as i16,
                    *left.offset(bc[6] as isize) as i16,
                    *left.offset(bc[7] as isize) as i16,
                ));
                a1_y = _mm256_cvtepu16_epi32(_mm_setr_epi16(
                    *left.offset(bc[0] as isize + 1) as i16,
                    *left.offset(bc[1] as isize + 1) as i16,
                    *left.offset(bc[2] as isize + 1) as i16,
                    *left.offset(bc[3] as isize + 1) as i16,
                    *left.offset(bc[4] as isize + 1) as i16,
                    *left.offset(bc[5] as isize + 1) as i16,
                    *left.offset(bc[6] as isize + 1) as i16,
                    *left.offset(bc[7] as isize + 1) as i16,
                ));

                shift = _mm256_srli_epi32::<1>(_mm256_and_si256(y_c256, c3f));

                diff = _mm256_sub_epi32(a1_y, a0_y); // a[x+1] - a[x]
                a32 = _mm256_slli_epi32::<5>(a0_y); // a[x] * 32
                a32 = _mm256_add_epi32(a32, a16); // a[x] * 32 + 16

                b = _mm256_mullo_epi32(diff, shift);
                res = _mm256_add_epi32(a32, b);
                res = _mm256_srli_epi32::<5>(res);

                resy[0] = _mm256_packus_epi32(
                    res,
                    _mm256_castsi128_si256(_mm256_extracti128_si256::<1>(res)),
                );

                a0_y = _mm256_cvtepu16_epi32(_mm_setr_epi16(
                    *left.offset(bc[8] as isize) as i16,
                    *left.offset(bc[9] as isize) as i16,
                    *left.offset(bc[10] as isize) as i16,
                    *left.offset(bc[11] as isize) as i16,
                    *left.offset(bc[12] as isize) as i16,
                    *left.offset(bc[13] as isize) as i16,
                    *left.offset(bc[14] as isize) as i16,
                    *left.offset(bc[15] as isize) as i16,
                ));
                a1_y = _mm256_cvtepu16_epi32(_mm_setr_epi16(
                    *left.offset(bc[8] as isize + 1) as i16,
                    *left.offset(bc[9] as isize + 1) as i16,
                    *left.offset(bc[10] as isize + 1) as i16,
                    *left.offset(bc[11] as isize + 1) as i16,
                    *left.offset(bc[12] as isize + 1) as i16,
                    *left.offset(bc[13] as isize + 1) as i16,
                    *left.offset(bc[14] as isize + 1) as i16,
                    *left.offset(bc[15] as isize + 1) as i16,
                ));
                shift = _mm256_srli_epi32::<1>(_mm256_and_si256(y_c_1_256, c3f));

                diff = _mm256_sub_epi32(a1_y, a0_y); // a[x+1] - a[x]
                a32 = _mm256_slli_epi32::<5>(a0_y); // a[x] * 32
                a32 = _mm256_add_epi32(a32, a16); // a[x] * 32 + 16

                b = _mm256_mullo_epi32(diff, shift);
                res = _mm256_add_epi32(a32, b);
                res = _mm256_srli_epi32::<5>(res);

                resy[1] = _mm256_packus_epi32(
                    res,
                    _mm256_castsi128_si256(_mm256_extracti128_si256::<1>(res)),
                );

                resy[0] = _mm256_inserti128_si256::<1>(resy[0], _mm256_castsi256_si128(resy[1]));
                // 16 16bit values
            }

            resxy = _mm256_blendv_epi8(resx[0], resy[0], base_mask256(base_min_diff));
            _mm256_storeu_si256(dst.add(j as usize) as *mut __m256i, resxy);
            j += 16;
        } // for j
        dst = dst.offset(stride);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn highbd_dr_prediction_z2_HxW_avx2(
    H: i32,
    W: i32,
    mut dst: *mut u16,
    stride: isize,
    above: *const u16,
    left: *const u16,
    upsample_above: i32,
    upsample_left: i32,
    dx: i32,
    dy: i32,
) {
    // here upsample_above and upsample_left are 0 by design of
    // av1_use_intra_edge_upsample
    let min_base_x = -1;
    let min_base_y = -1;
    let _ = upsample_above;
    let _ = upsample_left;
    let frac_bits_x = 6;
    const frac_bits_y: i32 = 6;

    // pre-filter above pixels
    // store in temp buffers:
    //   above[x] * 32 + 16
    //   above[x+1] - above[x]
    // final pixels will be calculated as:
    //   (above[x] * 32 + 16 + (above[x+1] - above[x]) * shift) >> 5
    let mut a0_x: __m256i;
    let mut a1_x: __m256i;
    let mut a32: __m256i;
    let a16: __m256i;
    let c3f: __m256i;
    let c1: __m256i;
    let mut diff: __m256i;
    let min_base_y256: __m256i;
    let dy256: __m256i;
    let c1234: __m256i;
    let c0123: __m256i;
    let mut base_y_c: Aligned32<[i16; 16]> = core::mem::zeroed();

    a16 = _mm256_set1_epi16(16);
    c1 = _mm256_srli_epi16::<4>(a16);
    min_base_y256 = _mm256_set1_epi16(min_base_y as i16);
    c3f = _mm256_set1_epi16(0x3f);
    dy256 = _mm256_set1_epi16(dy as i16);
    c0123 = _mm256_setr_epi16(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
    c1234 = _mm256_add_epi16(c0123, c1);

    for r in 0..H {
        let mut b: __m256i;
        let mut res: __m256i;
        let mut shift: __m256i;
        let mut resx: __m256i;
        let mut resy: __m256i;
        let ydx: __m256i;
        let mut resxy: __m256i;
        let mut j256: __m256i;
        let mut r6: __m256i;
        let mut a0_x128: __m128i;
        let mut a1_x128: __m128i;
        let mut a0_1_x128: __m128i;
        let mut a1_1_x128: __m128i;
        let y = r + 1;
        ydx = _mm256_set1_epi16((y * dx) as i16);

        let mut j = 0;
        while j < W {
            j256 = _mm256_set1_epi16(j as i16);
            let base_x = ((j << 6) - y * dx) >> frac_bits_x;
            let mut base_shift = 0;
            if (base_x) < (min_base_x - 1) {
                base_shift = min_base_x - (base_x) - 1;
            }
            let mut base_min_diff = min_base_x - base_x;
            if base_min_diff > 16 {
                base_min_diff = 16;
            } else {
                if base_min_diff < 0 {
                    base_min_diff = 0;
                }
            }

            if base_shift < 8 {
                a0_x128 =
                    _mm_loadu_si128(above.offset((base_x + base_shift) as isize) as *const __m128i);
                a1_x128 = _mm_loadu_si128(
                    above.offset((base_x + base_shift + 1) as isize) as *const __m128i
                );
                a0_x128 = _mm_shuffle_epi8(
                    a0_x128,
                    load_mask128(HighbdLoadMaskx.0[base_shift as usize].as_ptr()),
                );
                a1_x128 = _mm_shuffle_epi8(
                    a1_x128,
                    load_mask128(HighbdLoadMaskx.0[base_shift as usize].as_ptr()),
                );

                a0_x = _mm256_castsi128_si256(a0_x128);
                a1_x = _mm256_castsi128_si256(a1_x128);
            } else {
                a0_x = _mm256_setzero_si256();
                a1_x = _mm256_setzero_si256();
            }

            let mut base_shift1 = 0;
            if base_shift > 8 {
                base_shift1 = base_shift - 8;
            }
            if base_shift1 < 8 {
                a0_1_x128 = _mm_loadu_si128(
                    above.offset((base_x + base_shift1 + 8) as isize) as *const __m128i
                );
                a1_1_x128 = _mm_loadu_si128(
                    above.offset((base_x + base_shift1 + 9) as isize) as *const __m128i
                );
                a0_1_x128 = _mm_shuffle_epi8(
                    a0_1_x128,
                    load_mask128(HighbdLoadMaskx.0[base_shift1 as usize].as_ptr()),
                );
                a1_1_x128 = _mm_shuffle_epi8(
                    a1_1_x128,
                    load_mask128(HighbdLoadMaskx.0[base_shift1 as usize].as_ptr()),
                );

                a0_x = _mm256_inserti128_si256::<1>(a0_x, a0_1_x128);
                a1_x = _mm256_inserti128_si256::<1>(a1_x, a1_1_x128);
            }
            r6 = _mm256_slli_epi16::<6>(_mm256_add_epi16(c0123, j256));
            shift = _mm256_srli_epi16::<1>(_mm256_and_si256(_mm256_sub_epi16(r6, ydx), c3f));

            diff = _mm256_sub_epi16(a1_x, a0_x); // a[x+1] - a[x]
            a32 = _mm256_slli_epi16::<5>(a0_x); // a[x] * 32
            a32 = _mm256_add_epi16(a32, a16); // a[x] * 32 + 16

            b = _mm256_mullo_epi16(diff, shift);
            res = _mm256_add_epi16(a32, b);
            resx = _mm256_srli_epi16::<5>(res); // 16 16-bit values

            // y calc
            resy = _mm256_setzero_si256();
            let a0_y: __m256i;
            let a1_y: __m256i;
            let shifty: __m256i;
            if base_x < min_base_x {
                let c256: __m256i;
                let y_c256: __m256i;
                let mut base_y_c256: __m256i;
                let mask256: __m256i;
                let mul16: __m256i;
                r6 = _mm256_set1_epi16((r << 6) as i16);
                c256 = _mm256_add_epi16(j256, c1234);
                mul16 = _mm256_min_epu16(
                    _mm256_mullo_epi16(c256, dy256),
                    _mm256_srli_epi16::<1>(min_base_y256),
                );
                y_c256 = _mm256_sub_epi16(r6, mul16);
                base_y_c256 = _mm256_srai_epi16::<frac_bits_y>(y_c256);
                mask256 = _mm256_cmpgt_epi16(min_base_y256, base_y_c256);
                base_y_c256 = _mm256_andnot_si256(mask256, base_y_c256);
                _mm256_store_si256(base_y_c.0.as_mut_ptr() as *mut __m256i, base_y_c256);
                {
                    let bc = &base_y_c.0;
                    a0_y = _mm256_setr_epi16(
                        *left.offset(bc[0] as isize) as i16,
                        *left.offset(bc[1] as isize) as i16,
                        *left.offset(bc[2] as isize) as i16,
                        *left.offset(bc[3] as isize) as i16,
                        *left.offset(bc[4] as isize) as i16,
                        *left.offset(bc[5] as isize) as i16,
                        *left.offset(bc[6] as isize) as i16,
                        *left.offset(bc[7] as isize) as i16,
                        *left.offset(bc[8] as isize) as i16,
                        *left.offset(bc[9] as isize) as i16,
                        *left.offset(bc[10] as isize) as i16,
                        *left.offset(bc[11] as isize) as i16,
                        *left.offset(bc[12] as isize) as i16,
                        *left.offset(bc[13] as isize) as i16,
                        *left.offset(bc[14] as isize) as i16,
                        *left.offset(bc[15] as isize) as i16,
                    );
                }
                base_y_c256 = _mm256_add_epi16(base_y_c256, c1);
                _mm256_store_si256(base_y_c.0.as_mut_ptr() as *mut __m256i, base_y_c256);
                {
                    let bc = &base_y_c.0;
                    a1_y = _mm256_setr_epi16(
                        *left.offset(bc[0] as isize) as i16,
                        *left.offset(bc[1] as isize) as i16,
                        *left.offset(bc[2] as isize) as i16,
                        *left.offset(bc[3] as isize) as i16,
                        *left.offset(bc[4] as isize) as i16,
                        *left.offset(bc[5] as isize) as i16,
                        *left.offset(bc[6] as isize) as i16,
                        *left.offset(bc[7] as isize) as i16,
                        *left.offset(bc[8] as isize) as i16,
                        *left.offset(bc[9] as isize) as i16,
                        *left.offset(bc[10] as isize) as i16,
                        *left.offset(bc[11] as isize) as i16,
                        *left.offset(bc[12] as isize) as i16,
                        *left.offset(bc[13] as isize) as i16,
                        *left.offset(bc[14] as isize) as i16,
                        *left.offset(bc[15] as isize) as i16,
                    );
                }

                shifty = _mm256_srli_epi16::<1>(_mm256_and_si256(y_c256, c3f));

                diff = _mm256_sub_epi16(a1_y, a0_y); // a[x+1] - a[x]
                a32 = _mm256_slli_epi16::<5>(a0_y); // a[x] * 32
                a32 = _mm256_add_epi16(a32, a16); // a[x] * 32 + 16

                b = _mm256_mullo_epi16(diff, shifty);
                res = _mm256_add_epi16(a32, b);
                resy = _mm256_srli_epi16::<5>(res);
            }

            resxy = _mm256_blendv_epi8(resx, resy, base_mask256(base_min_diff));
            _mm256_storeu_si256(dst.add(j as usize) as *mut __m256i, resxy);
            j += 16;
        } // for j
        dst = dst.offset(stride);
    }
}

// Directional prediction, zone 2: 90 < angle < 180
#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn av1_highbd_dr_prediction_z2_avx2(
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
    let _ = bd;
    debug_assert!(dx > 0);
    debug_assert!(dy > 0);
    match bw {
        4 => {
            if bd < 12 {
                highbd_dr_prediction_z2_Nx4_avx2(
                    bh,
                    dst,
                    stride,
                    above,
                    left,
                    upsample_above,
                    upsample_left,
                    dx,
                    dy,
                );
            } else {
                highbd_dr_prediction_32bit_z2_Nx4_avx2(
                    bh,
                    dst,
                    stride,
                    above,
                    left,
                    upsample_above,
                    upsample_left,
                    dx,
                    dy,
                );
            }
        }
        8 => {
            if bd < 12 {
                highbd_dr_prediction_z2_Nx8_avx2(
                    bh,
                    dst,
                    stride,
                    above,
                    left,
                    upsample_above,
                    upsample_left,
                    dx,
                    dy,
                );
            } else {
                highbd_dr_prediction_32bit_z2_Nx8_avx2(
                    bh,
                    dst,
                    stride,
                    above,
                    left,
                    upsample_above,
                    upsample_left,
                    dx,
                    dy,
                );
            }
        }
        _ => {
            if bd < 12 {
                highbd_dr_prediction_z2_HxW_avx2(
                    bh,
                    bw,
                    dst,
                    stride,
                    above,
                    left,
                    upsample_above,
                    upsample_left,
                    dx,
                    dy,
                );
            } else {
                highbd_dr_prediction_32bit_z2_HxW_avx2(
                    bh,
                    bw,
                    dst,
                    stride,
                    above,
                    left,
                    upsample_above,
                    upsample_left,
                    dx,
                    dy,
                );
            }
        }
    }
}

//  Directional prediction, zone 3 functions
#[target_feature(enable = "avx2")]
unsafe fn highbd_dr_prediction_z3_4x4_avx2(
    dst: *mut u16,
    stride: isize,
    left: *const u16,
    upsample_left: i32,
    dy: i32,
    bd: i32,
) {
    let mut dstvec: [__m128i; 4] = core::mem::zeroed();
    let mut d: [__m128i; 4] = core::mem::zeroed();
    if bd < 12 {
        highbd_dr_prediction_z1_4xN_internal_avx2(4, dstvec.as_mut_ptr(), left, upsample_left, dy);
    } else {
        highbd_dr_prediction_32bit_z1_4xN_internal_avx2(
            4,
            dstvec.as_mut_ptr(),
            left,
            upsample_left,
            dy,
        );
    }
    let v = dstvec.as_ptr();
    let o = d.as_mut_ptr();
    highbd_transpose4x8_8x4_low_sse2(
        v.add(0),
        v.add(1),
        v.add(2),
        v.add(3),
        o.add(0),
        o.add(1),
        o.add(2),
        o.add(3),
    );
    _mm_storel_epi64(dst.offset(0 * stride) as *mut __m128i, d[0]);
    _mm_storel_epi64(dst.offset(1 * stride) as *mut __m128i, d[1]);
    _mm_storel_epi64(dst.offset(2 * stride) as *mut __m128i, d[2]);
    _mm_storel_epi64(dst.offset(3 * stride) as *mut __m128i, d[3]);
}

#[target_feature(enable = "avx2")]
unsafe fn highbd_dr_prediction_z3_8x8_avx2(
    dst: *mut u16,
    stride: isize,
    left: *const u16,
    upsample_left: i32,
    dy: i32,
    bd: i32,
) {
    let mut dstvec: [__m128i; 8] = core::mem::zeroed();
    let mut d: [__m128i; 8] = core::mem::zeroed();
    if bd < 12 {
        highbd_dr_prediction_z1_8xN_internal_avx2(8, dstvec.as_mut_ptr(), left, upsample_left, dy);
    } else {
        highbd_dr_prediction_32bit_z1_8xN_internal_avx2(
            8,
            dstvec.as_mut_ptr(),
            left,
            upsample_left,
            dy,
        );
    }
    let v = dstvec.as_ptr();
    let o = d.as_mut_ptr();
    highbd_transpose8x8_sse2(
        v.add(0),
        v.add(1),
        v.add(2),
        v.add(3),
        v.add(4),
        v.add(5),
        v.add(6),
        v.add(7),
        o.add(0),
        o.add(1),
        o.add(2),
        o.add(3),
        o.add(4),
        o.add(5),
        o.add(6),
        o.add(7),
    );
    for i in 0..8 {
        _mm_storeu_si128(dst.offset(i as isize * stride) as *mut __m128i, d[i]);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn highbd_dr_prediction_z3_4x8_avx2(
    dst: *mut u16,
    stride: isize,
    left: *const u16,
    upsample_left: i32,
    dy: i32,
    bd: i32,
) {
    let mut dstvec: [__m128i; 4] = core::mem::zeroed();
    let mut d: [__m128i; 8] = core::mem::zeroed();
    if bd < 12 {
        highbd_dr_prediction_z1_8xN_internal_avx2(4, dstvec.as_mut_ptr(), left, upsample_left, dy);
    } else {
        highbd_dr_prediction_32bit_z1_8xN_internal_avx2(
            4,
            dstvec.as_mut_ptr(),
            left,
            upsample_left,
            dy,
        );
    }

    let v = dstvec.as_ptr();
    let o = d.as_mut_ptr();
    highbd_transpose4x8_8x4_sse2(
        v.add(0),
        v.add(1),
        v.add(2),
        v.add(3),
        o.add(0),
        o.add(1),
        o.add(2),
        o.add(3),
        o.add(4),
        o.add(5),
        o.add(6),
        o.add(7),
    );
    for i in 0..8 {
        _mm_storel_epi64(dst.offset(i as isize * stride) as *mut __m128i, d[i]);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn highbd_dr_prediction_z3_8x4_avx2(
    dst: *mut u16,
    stride: isize,
    left: *const u16,
    upsample_left: i32,
    dy: i32,
    bd: i32,
) {
    let mut dstvec: [__m128i; 8] = core::mem::zeroed();
    let mut d: [__m128i; 4] = core::mem::zeroed();
    if bd < 12 {
        highbd_dr_prediction_z1_4xN_internal_avx2(8, dstvec.as_mut_ptr(), left, upsample_left, dy);
    } else {
        highbd_dr_prediction_32bit_z1_4xN_internal_avx2(
            8,
            dstvec.as_mut_ptr(),
            left,
            upsample_left,
            dy,
        );
    }

    let v = dstvec.as_ptr();
    let o = d.as_mut_ptr();
    highbd_transpose8x8_low_sse2(
        v.add(0),
        v.add(1),
        v.add(2),
        v.add(3),
        v.add(4),
        v.add(5),
        v.add(6),
        v.add(7),
        o.add(0),
        o.add(1),
        o.add(2),
        o.add(3),
    );
    _mm_storeu_si128(dst.offset(0 * stride) as *mut __m128i, d[0]);
    _mm_storeu_si128(dst.offset(1 * stride) as *mut __m128i, d[1]);
    _mm_storeu_si128(dst.offset(2 * stride) as *mut __m128i, d[2]);
    _mm_storeu_si128(dst.offset(3 * stride) as *mut __m128i, d[3]);
}

#[target_feature(enable = "avx2")]
unsafe fn highbd_dr_prediction_z3_8x16_avx2(
    dst: *mut u16,
    stride: isize,
    left: *const u16,
    upsample_left: i32,
    dy: i32,
    bd: i32,
) {
    let mut dstvec: [__m256i; 8] = core::mem::zeroed();
    let mut d: [__m256i; 8] = core::mem::zeroed();
    if bd < 12 {
        highbd_dr_prediction_z1_16xN_internal_avx2(8, dstvec.as_mut_ptr(), left, upsample_left, dy);
    } else {
        highbd_dr_prediction_32bit_z1_16xN_internal_avx2(
            8,
            dstvec.as_mut_ptr(),
            left,
            upsample_left,
            dy,
        );
    }
    highbd_transpose8x16_16x8_avx2(dstvec.as_ptr(), d.as_mut_ptr());
    for i in 0..8 {
        _mm_storeu_si128(
            dst.offset(i as isize * stride) as *mut __m128i,
            _mm256_castsi256_si128(d[i]),
        );
    }
    for i in 8..16 {
        _mm_storeu_si128(
            dst.offset(i as isize * stride) as *mut __m128i,
            _mm256_extracti128_si256::<1>(d[i - 8]),
        );
    }
}

#[target_feature(enable = "avx2")]
unsafe fn highbd_dr_prediction_z3_16x8_avx2(
    dst: *mut u16,
    stride: isize,
    left: *const u16,
    upsample_left: i32,
    dy: i32,
    bd: i32,
) {
    let mut dstvec: [__m128i; 16] = core::mem::zeroed();
    let mut d: [__m128i; 16] = core::mem::zeroed();
    if bd < 12 {
        highbd_dr_prediction_z1_8xN_internal_avx2(16, dstvec.as_mut_ptr(), left, upsample_left, dy);
    } else {
        highbd_dr_prediction_32bit_z1_8xN_internal_avx2(
            16,
            dstvec.as_mut_ptr(),
            left,
            upsample_left,
            dy,
        );
    }
    let v = dstvec.as_ptr();
    let o = d.as_mut_ptr();
    let mut i = 0;
    while i < 16 {
        highbd_transpose8x8_sse2(
            v.add(0 + i),
            v.add(1 + i),
            v.add(2 + i),
            v.add(3 + i),
            v.add(4 + i),
            v.add(5 + i),
            v.add(6 + i),
            v.add(7 + i),
            o.add(0 + i),
            o.add(1 + i),
            o.add(2 + i),
            o.add(3 + i),
            o.add(4 + i),
            o.add(5 + i),
            o.add(6 + i),
            o.add(7 + i),
        );
        i += 8;
    }
    for i in 0..8 {
        _mm_storeu_si128(dst.offset(i as isize * stride) as *mut __m128i, d[i]);
        _mm_storeu_si128(
            dst.offset(i as isize * stride + 8) as *mut __m128i,
            d[i + 8],
        );
    }
}

// !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER
#[target_feature(enable = "avx2")]
unsafe fn highbd_dr_prediction_z3_4x16_avx2(
    dst: *mut u16,
    stride: isize,
    left: *const u16,
    upsample_left: i32,
    dy: i32,
    bd: i32,
) {
    let mut dstvec: [__m256i; 4] = core::mem::zeroed();
    let mut d: [__m256i; 4] = core::mem::zeroed();
    let mut d1: __m256i;
    if bd < 12 {
        highbd_dr_prediction_z1_16xN_internal_avx2(4, dstvec.as_mut_ptr(), left, upsample_left, dy);
    } else {
        highbd_dr_prediction_32bit_z1_16xN_internal_avx2(
            4,
            dstvec.as_mut_ptr(),
            left,
            upsample_left,
            dy,
        );
    }
    highbd_transpose4x16_avx2(dstvec.as_ptr(), d.as_mut_ptr());
    for i in 0..4 {
        _mm_storel_epi64(
            dst.offset(i as isize * stride) as *mut __m128i,
            _mm256_castsi256_si128(d[i]),
        );
        d1 = _mm256_bsrli_epi128::<8>(d[i]);
        _mm_storel_epi64(
            dst.offset((i as isize + 4) * stride) as *mut __m128i,
            _mm256_castsi256_si128(d1),
        );
        _mm_storel_epi64(
            dst.offset((i as isize + 8) * stride) as *mut __m128i,
            _mm256_extracti128_si256::<1>(d[i]),
        );
        _mm_storel_epi64(
            dst.offset((i as isize + 12) * stride) as *mut __m128i,
            _mm256_extracti128_si256::<1>(d1),
        );
    }
}

#[target_feature(enable = "avx2")]
unsafe fn highbd_dr_prediction_z3_16x4_avx2(
    dst: *mut u16,
    stride: isize,
    left: *const u16,
    upsample_left: i32,
    dy: i32,
    bd: i32,
) {
    let mut dstvec: [__m128i; 16] = core::mem::zeroed();
    let mut d: [__m128i; 8] = core::mem::zeroed();
    if bd < 12 {
        highbd_dr_prediction_z1_4xN_internal_avx2(16, dstvec.as_mut_ptr(), left, upsample_left, dy);
    } else {
        highbd_dr_prediction_32bit_z1_4xN_internal_avx2(
            16,
            dstvec.as_mut_ptr(),
            left,
            upsample_left,
            dy,
        );
    }
    highbd_transpose16x4_8x8_sse2(dstvec.as_ptr(), d.as_mut_ptr());

    _mm_storeu_si128(dst.offset(0 * stride) as *mut __m128i, d[0]);
    _mm_storeu_si128(dst.offset(0 * stride + 8) as *mut __m128i, d[1]);
    _mm_storeu_si128(dst.offset(1 * stride) as *mut __m128i, d[2]);
    _mm_storeu_si128(dst.offset(1 * stride + 8) as *mut __m128i, d[3]);
    _mm_storeu_si128(dst.offset(2 * stride) as *mut __m128i, d[4]);
    _mm_storeu_si128(dst.offset(2 * stride + 8) as *mut __m128i, d[5]);
    _mm_storeu_si128(dst.offset(3 * stride) as *mut __m128i, d[6]);
    _mm_storeu_si128(dst.offset(3 * stride + 8) as *mut __m128i, d[7]);
}

#[target_feature(enable = "avx2")]
unsafe fn highbd_dr_prediction_z3_8x32_avx2(
    dst: *mut u16,
    stride: isize,
    left: *const u16,
    upsample_left: i32,
    dy: i32,
    bd: i32,
) {
    let mut dstvec: [__m256i; 16] = core::mem::zeroed();
    let mut d: [__m256i; 16] = core::mem::zeroed();
    if bd < 12 {
        highbd_dr_prediction_z1_32xN_internal_avx2(8, dstvec.as_mut_ptr(), left, upsample_left, dy);
    } else {
        highbd_dr_prediction_32bit_z1_32xN_internal_avx2(
            8,
            dstvec.as_mut_ptr(),
            left,
            upsample_left,
            dy,
        );
    }

    let mut i = 0;
    while i < 16 {
        highbd_transpose8x16_16x8_avx2(dstvec.as_ptr().add(i), d.as_mut_ptr().add(i));
        i += 8;
    }

    for i in 0..8 {
        _mm_storeu_si128(
            dst.offset(i as isize * stride) as *mut __m128i,
            _mm256_castsi256_si128(d[i]),
        );
    }
    for i in 0..8 {
        _mm_storeu_si128(
            dst.offset((i as isize + 8) * stride) as *mut __m128i,
            _mm256_extracti128_si256::<1>(d[i]),
        );
    }
    for i in 8..16 {
        _mm_storeu_si128(
            dst.offset((i as isize + 8) * stride) as *mut __m128i,
            _mm256_castsi256_si128(d[i]),
        );
    }
    for i in 8..16 {
        _mm_storeu_si128(
            dst.offset((i as isize + 16) * stride) as *mut __m128i,
            _mm256_extracti128_si256::<1>(d[i]),
        );
    }
}

#[target_feature(enable = "avx2")]
unsafe fn highbd_dr_prediction_z3_32x8_avx2(
    dst: *mut u16,
    stride: isize,
    left: *const u16,
    upsample_left: i32,
    dy: i32,
    bd: i32,
) {
    let mut dstvec: [__m128i; 32] = core::mem::zeroed();
    let mut d: [__m128i; 32] = core::mem::zeroed();
    if bd < 12 {
        highbd_dr_prediction_z1_8xN_internal_avx2(32, dstvec.as_mut_ptr(), left, upsample_left, dy);
    } else {
        highbd_dr_prediction_32bit_z1_8xN_internal_avx2(
            32,
            dstvec.as_mut_ptr(),
            left,
            upsample_left,
            dy,
        );
    }

    let v = dstvec.as_ptr();
    let o = d.as_mut_ptr();
    let mut i = 0;
    while i < 32 {
        highbd_transpose8x8_sse2(
            v.add(0 + i),
            v.add(1 + i),
            v.add(2 + i),
            v.add(3 + i),
            v.add(4 + i),
            v.add(5 + i),
            v.add(6 + i),
            v.add(7 + i),
            o.add(0 + i),
            o.add(1 + i),
            o.add(2 + i),
            o.add(3 + i),
            o.add(4 + i),
            o.add(5 + i),
            o.add(6 + i),
            o.add(7 + i),
        );
        i += 8;
    }
    for i in 0..8 {
        _mm_storeu_si128(dst.offset(i as isize * stride) as *mut __m128i, d[i]);
        _mm_storeu_si128(
            dst.offset(i as isize * stride + 8) as *mut __m128i,
            d[i + 8],
        );
        _mm_storeu_si128(
            dst.offset(i as isize * stride + 16) as *mut __m128i,
            d[i + 16],
        );
        _mm_storeu_si128(
            dst.offset(i as isize * stride + 24) as *mut __m128i,
            d[i + 24],
        );
    }
}
// !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER

#[target_feature(enable = "avx2")]
unsafe fn highbd_dr_prediction_z3_16x16_avx2(
    dst: *mut u16,
    stride: isize,
    left: *const u16,
    upsample_left: i32,
    dy: i32,
    bd: i32,
) {
    let mut dstvec: [__m256i; 16] = core::mem::zeroed();
    let mut d: [__m256i; 16] = core::mem::zeroed();
    if bd < 12 {
        highbd_dr_prediction_z1_16xN_internal_avx2(
            16,
            dstvec.as_mut_ptr(),
            left,
            upsample_left,
            dy,
        );
    } else {
        highbd_dr_prediction_32bit_z1_16xN_internal_avx2(
            16,
            dstvec.as_mut_ptr(),
            left,
            upsample_left,
            dy,
        );
    }

    highbd_transpose16x16_avx2(dstvec.as_ptr(), d.as_mut_ptr());

    for i in 0..16 {
        _mm256_storeu_si256(dst.offset(i as isize * stride) as *mut __m256i, d[i]);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn highbd_dr_prediction_z3_32x32_avx2(
    dst: *mut u16,
    stride: isize,
    left: *const u16,
    upsample_left: i32,
    dy: i32,
    bd: i32,
) {
    let mut dstvec: [__m256i; 64] = core::mem::zeroed();
    let mut d: [__m256i; 16] = core::mem::zeroed();
    if bd < 12 {
        highbd_dr_prediction_z1_32xN_internal_avx2(
            32,
            dstvec.as_mut_ptr(),
            left,
            upsample_left,
            dy,
        );
    } else {
        highbd_dr_prediction_32bit_z1_32xN_internal_avx2(
            32,
            dstvec.as_mut_ptr(),
            left,
            upsample_left,
            dy,
        );
    }
    highbd_transpose16x16_avx2(dstvec.as_ptr(), d.as_mut_ptr());
    for j in 0..16 {
        _mm256_storeu_si256(dst.offset(j as isize * stride) as *mut __m256i, d[j]);
    }
    highbd_transpose16x16_avx2(dstvec.as_ptr().add(16), d.as_mut_ptr());
    for j in 0..16 {
        _mm256_storeu_si256(dst.offset(j as isize * stride + 16) as *mut __m256i, d[j]);
    }
    highbd_transpose16x16_avx2(dstvec.as_ptr().add(32), d.as_mut_ptr());
    for j in 0..16 {
        _mm256_storeu_si256(dst.offset((j as isize + 16) * stride) as *mut __m256i, d[j]);
    }
    highbd_transpose16x16_avx2(dstvec.as_ptr().add(48), d.as_mut_ptr());
    for j in 0..16 {
        _mm256_storeu_si256(
            dst.offset((j as isize + 16) * stride + 16) as *mut __m256i,
            d[j],
        );
    }
}

#[target_feature(enable = "avx2")]
unsafe fn highbd_dr_prediction_z3_64x64_avx2(
    dst: *mut u16,
    stride: isize,
    left: *const u16,
    upsample_left: i32,
    dy: i32,
    bd: i32,
) {
    let mut dstT: Aligned16<[u16; 64 * 64]> = core::mem::zeroed();
    if bd < 12 {
        highbd_dr_prediction_z1_64xN_avx2(64, dstT.0.as_mut_ptr(), 64, left, upsample_left, dy);
    } else {
        highbd_dr_prediction_32bit_z1_64xN_avx2(
            64,
            dstT.0.as_mut_ptr(),
            64,
            left,
            upsample_left,
            dy,
        );
    }
    highbd_transpose(dstT.0.as_ptr(), 64, dst, stride, 64, 64);
}

#[target_feature(enable = "avx2")]
unsafe fn highbd_dr_prediction_z3_16x32_avx2(
    dst: *mut u16,
    stride: isize,
    left: *const u16,
    upsample_left: i32,
    dy: i32,
    bd: i32,
) {
    let mut dstvec: [__m256i; 32] = core::mem::zeroed();
    let mut d: [__m256i; 32] = core::mem::zeroed();
    if bd < 12 {
        highbd_dr_prediction_z1_32xN_internal_avx2(
            16,
            dstvec.as_mut_ptr(),
            left,
            upsample_left,
            dy,
        );
    } else {
        highbd_dr_prediction_32bit_z1_32xN_internal_avx2(
            16,
            dstvec.as_mut_ptr(),
            left,
            upsample_left,
            dy,
        );
    }
    let mut i = 0;
    while i < 32 {
        highbd_transpose8x16_16x8_avx2(dstvec.as_ptr().add(i), d.as_mut_ptr().add(i));
        i += 8;
    }
    // store
    let mut j = 0;
    while j < 32 {
        for i in 0..8 {
            _mm_storeu_si128(
                dst.offset((i + j) as isize * stride) as *mut __m128i,
                _mm256_castsi256_si128(d[i + j]),
            );
        }
        for i in 0..8 {
            _mm_storeu_si128(
                dst.offset((i + j) as isize * stride + 8) as *mut __m128i,
                _mm256_castsi256_si128(d[(i + j) + 8]),
            );
        }
        for i in 8..16 {
            _mm256_storeu_si256(
                dst.offset((i + j) as isize * stride) as *mut __m256i,
                _mm256_inserti128_si256::<0>(
                    d[i + j],
                    _mm256_extracti128_si256::<1>(d[(i + j) - 8]),
                ),
            );
        }
        j += 16;
    }
}

#[target_feature(enable = "avx2")]
unsafe fn highbd_dr_prediction_z3_32x16_avx2(
    dst: *mut u16,
    stride: isize,
    left: *const u16,
    upsample_left: i32,
    dy: i32,
    bd: i32,
) {
    let mut dstvec: [__m256i; 32] = core::mem::zeroed();
    let mut d: [__m256i; 16] = core::mem::zeroed();
    if bd < 12 {
        highbd_dr_prediction_z1_16xN_internal_avx2(
            32,
            dstvec.as_mut_ptr(),
            left,
            upsample_left,
            dy,
        );
    } else {
        highbd_dr_prediction_32bit_z1_16xN_internal_avx2(
            32,
            dstvec.as_mut_ptr(),
            left,
            upsample_left,
            dy,
        );
    }
    let mut i = 0;
    while i < 32 {
        highbd_transpose16x16_avx2(dstvec.as_ptr().add(i), d.as_mut_ptr());
        for j in 0..16 {
            _mm256_storeu_si256(
                dst.offset(j as isize * stride + i as isize) as *mut __m256i,
                d[j],
            );
        }
        i += 16;
    }
}

#[target_feature(enable = "avx2")]
unsafe fn highbd_dr_prediction_z3_32x64_avx2(
    dst: *mut u16,
    stride: isize,
    left: *const u16,
    upsample_left: i32,
    dy: i32,
    bd: i32,
) {
    let mut dstT: [u16; 64 * 32] = core::mem::zeroed();
    if bd < 12 {
        highbd_dr_prediction_z1_64xN_avx2(32, dstT.as_mut_ptr(), 64, left, upsample_left, dy);
    } else {
        highbd_dr_prediction_32bit_z1_64xN_avx2(32, dstT.as_mut_ptr(), 64, left, upsample_left, dy);
    }
    highbd_transpose(dstT.as_ptr(), 64, dst, stride, 32, 64);
}

#[target_feature(enable = "avx2")]
unsafe fn highbd_dr_prediction_z3_64x32_avx2(
    dst: *mut u16,
    stride: isize,
    left: *const u16,
    upsample_left: i32,
    dy: i32,
    bd: i32,
) {
    let mut dstT: Aligned16<[u16; 32 * 64]> = core::mem::zeroed();
    highbd_dr_prediction_z1_32xN_avx2(64, dstT.0.as_mut_ptr(), 32, left, upsample_left, dy, bd);
    highbd_transpose(dstT.0.as_ptr(), 32, dst, stride, 64, 32);
}

// !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER
#[target_feature(enable = "avx2")]
unsafe fn highbd_dr_prediction_z3_16x64_avx2(
    dst: *mut u16,
    stride: isize,
    left: *const u16,
    upsample_left: i32,
    dy: i32,
    bd: i32,
) {
    let mut dstT: Aligned16<[u16; 64 * 16]> = core::mem::zeroed();
    if bd < 12 {
        highbd_dr_prediction_z1_64xN_avx2(16, dstT.0.as_mut_ptr(), 64, left, upsample_left, dy);
    } else {
        highbd_dr_prediction_32bit_z1_64xN_avx2(
            16,
            dstT.0.as_mut_ptr(),
            64,
            left,
            upsample_left,
            dy,
        );
    }
    highbd_transpose(dstT.0.as_ptr(), 64, dst, stride, 16, 64);
}

#[target_feature(enable = "avx2")]
unsafe fn highbd_dr_prediction_z3_64x16_avx2(
    dst: *mut u16,
    stride: isize,
    left: *const u16,
    upsample_left: i32,
    dy: i32,
    bd: i32,
) {
    let mut dstvec: [__m256i; 64] = core::mem::zeroed();
    let mut d: [__m256i; 16] = core::mem::zeroed();
    if bd < 12 {
        highbd_dr_prediction_z1_16xN_internal_avx2(
            64,
            dstvec.as_mut_ptr(),
            left,
            upsample_left,
            dy,
        );
    } else {
        highbd_dr_prediction_32bit_z1_16xN_internal_avx2(
            64,
            dstvec.as_mut_ptr(),
            left,
            upsample_left,
            dy,
        );
    }
    let mut i = 0;
    while i < 64 {
        highbd_transpose16x16_avx2(dstvec.as_ptr().add(i), d.as_mut_ptr());
        for j in 0..16 {
            _mm256_storeu_si256(
                dst.offset(j as isize * stride + i as isize) as *mut __m256i,
                d[j],
            );
        }
        i += 16;
    }
}
// !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn av1_highbd_dr_prediction_z3_avx2(
    dst: *mut u16,
    stride: isize,
    bw: i32,
    bh: i32,
    above: *const u16,
    left: *const u16,
    upsample_left: i32,
    dx: i32,
    dy: i32,
    bd: i32,
) {
    let _ = above;
    let _ = dx;

    debug_assert!(dx == 1);
    debug_assert!(dy > 0);
    if bw == bh {
        match bw {
            4 => highbd_dr_prediction_z3_4x4_avx2(dst, stride, left, upsample_left, dy, bd),
            8 => highbd_dr_prediction_z3_8x8_avx2(dst, stride, left, upsample_left, dy, bd),
            16 => highbd_dr_prediction_z3_16x16_avx2(dst, stride, left, upsample_left, dy, bd),
            32 => highbd_dr_prediction_z3_32x32_avx2(dst, stride, left, upsample_left, dy, bd),
            64 => highbd_dr_prediction_z3_64x64_avx2(dst, stride, left, upsample_left, dy, bd),
            _ => {}
        }
    } else {
        if bw < bh {
            if bw + bw == bh {
                match bw {
                    4 => highbd_dr_prediction_z3_4x8_avx2(dst, stride, left, upsample_left, dy, bd),
                    8 => {
                        highbd_dr_prediction_z3_8x16_avx2(dst, stride, left, upsample_left, dy, bd)
                    }
                    16 => {
                        highbd_dr_prediction_z3_16x32_avx2(dst, stride, left, upsample_left, dy, bd)
                    }
                    32 => {
                        highbd_dr_prediction_z3_32x64_avx2(dst, stride, left, upsample_left, dy, bd)
                    }
                    _ => {}
                }
            } else {
                match bw {
                    // !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER
                    4 => {
                        highbd_dr_prediction_z3_4x16_avx2(dst, stride, left, upsample_left, dy, bd)
                    }
                    8 => {
                        highbd_dr_prediction_z3_8x32_avx2(dst, stride, left, upsample_left, dy, bd)
                    }
                    16 => {
                        highbd_dr_prediction_z3_16x64_avx2(dst, stride, left, upsample_left, dy, bd)
                    }
                    _ => {}
                }
            }
        } else {
            if bh + bh == bw {
                match bh {
                    4 => highbd_dr_prediction_z3_8x4_avx2(dst, stride, left, upsample_left, dy, bd),
                    8 => {
                        highbd_dr_prediction_z3_16x8_avx2(dst, stride, left, upsample_left, dy, bd)
                    }
                    16 => {
                        highbd_dr_prediction_z3_32x16_avx2(dst, stride, left, upsample_left, dy, bd)
                    }
                    32 => {
                        highbd_dr_prediction_z3_64x32_avx2(dst, stride, left, upsample_left, dy, bd)
                    }
                    _ => {}
                }
            } else {
                match bh {
                    // !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER
                    4 => {
                        highbd_dr_prediction_z3_16x4_avx2(dst, stride, left, upsample_left, dy, bd)
                    }
                    8 => {
                        highbd_dr_prediction_z3_32x8_avx2(dst, stride, left, upsample_left, dy, bd)
                    }
                    16 => {
                        highbd_dr_prediction_z3_64x16_avx2(dst, stride, left, upsample_left, dy, bd)
                    }
                    _ => {}
                }
            }
        }
    }
}

// From aom_dsp/intrapred_common.h.
const SMOOTH_WEIGHT_LOG2_SCALE: i32 = 8;

// Weights are quadratic from '1' to '1 / block_size', scaled by
// 2^SMOOTH_WEIGHT_LOG2_SCALE.
// Note these arrays are aligned to ensure NEON loads using a cast to uint32_t*
// have sufficient alignment. Using 8 preserves the potential for an alignment
// hint in load_weight_w8(). For that case, this could be increased to 16 to
// allow an aligned load in x86.
static smooth_weights: Aligned8Table<[u8; 124]> = Aligned8Table([
    255, 149, 85, 64, 255, 197, 146, 105, 73, 50, 37, 32, 255, 225, 196, 170, 145, 123, 102, 84,
    68, 54, 43, 33, 26, 20, 17, 16, 255, 240, 225, 210, 196, 182, 169, 157, 145, 133, 122, 111,
    101, 92, 83, 74, 66, 59, 52, 45, 39, 34, 29, 25, 21, 17, 14, 12, 10, 9, 8, 8, 255, 248, 240,
    233, 225, 218, 210, 203, 196, 189, 182, 176, 169, 163, 156, 150, 144, 138, 133, 127, 121, 116,
    111, 106, 101, 96, 91, 86, 82, 77, 73, 69, 65, 61, 57, 54, 50, 47, 44, 41, 38, 35, 32, 29, 27,
    25, 22, 20, 18, 16, 15, 13, 12, 10, 9, 8, 7, 6, 6, 5, 5, 4, 4, 4,
]);

// bs = 4
// bs = 8
// bs = 16
// bs = 32
// bs = 64
static smooth_weights_u16: Aligned8Table<[u16; 124]> = Aligned8Table([
    255, 149, 85, 64, 255, 197, 146, 105, 73, 50, 37, 32, 255, 225, 196, 170, 145, 123, 102, 84,
    68, 54, 43, 33, 26, 20, 17, 16, 255, 240, 225, 210, 196, 182, 169, 157, 145, 133, 122, 111,
    101, 92, 83, 74, 66, 59, 52, 45, 39, 34, 29, 25, 21, 17, 14, 12, 10, 9, 8, 8, 255, 248, 240,
    233, 225, 218, 210, 203, 196, 189, 182, 176, 169, 163, 156, 150, 144, 138, 133, 127, 121, 116,
    111, 106, 101, 96, 91, 86, 82, 77, 73, 69, 65, 61, 57, 54, 50, 47, 44, 41, 38, 35, 32, 29, 27,
    25, 22, 20, 18, 16, 15, 13, 12, 10, 9, 8, 7, 6, 6, 5, 5, 4, 4, 4,
]);

// block dimension = 4
// block dimension = 8
// block dimension = 16
// block dimension = 32
// block dimension = 64
// AOM_AOM_DSP_INTRAPRED_COMMON_H_

#[repr(C, align(8))]
struct Aligned8Table<T>(T);

// ---------------------------------------------------------------------------
// The 8-bit predictors of aom_dsp/x86/intrapred_avx2.c.

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn dc_sum_64(ref_: *const u8) -> __m256i {
    let x0: __m256i = _mm256_loadu_si256(ref_ as *const __m256i);
    let x1: __m256i = _mm256_loadu_si256(ref_.offset(32) as *const __m256i);
    let zero: __m256i = _mm256_setzero_si256();
    let mut y0: __m256i = _mm256_sad_epu8(x0, zero);
    let mut y1: __m256i = _mm256_sad_epu8(x1, zero);
    y0 = _mm256_add_epi64(y0, y1);
    let mut u0: __m256i = _mm256_permute2x128_si256::<1>(y0, y0);
    y0 = _mm256_add_epi64(u0, y0);
    u0 = _mm256_unpackhi_epi64(y0, y0);
    return _mm256_add_epi16(y0, u0);
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn dc_sum_32(ref_: *const u8) -> __m256i {
    let x: __m256i = _mm256_loadu_si256(ref_ as *const __m256i);
    let zero: __m256i = _mm256_setzero_si256();
    let mut y: __m256i = _mm256_sad_epu8(x, zero);
    let mut u: __m256i = _mm256_permute2x128_si256::<1>(y, y);
    y = _mm256_add_epi64(u, y);
    u = _mm256_unpackhi_epi64(y, y);
    return _mm256_add_epi16(y, u);
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn row_store_32xh(r: *const __m256i, height: i32, mut dst: *mut u8, stride: isize) {
    for i in 0..height {
        _mm256_storeu_si256(dst as *mut __m256i, *r);
        dst = dst.offset(stride);
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn row_store_32x2xh(
    r0: *const __m256i,
    r1: *const __m256i,
    height: i32,
    mut dst: *mut u8,
    stride: isize,
) {
    for i in 0..height {
        _mm256_storeu_si256(dst as *mut __m256i, *r0);
        _mm256_storeu_si256(dst.offset(32) as *mut __m256i, *r1);
        dst = dst.offset(stride);
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn row_store_64xh(r: *const __m256i, height: i32, mut dst: *mut u8, stride: isize) {
    for i in 0..height {
        _mm256_storeu_si256(dst as *mut __m256i, *r);
        _mm256_storeu_si256(dst.offset(32) as *mut __m256i, *r);
        dst = dst.offset(stride);
    }
}

//
//
//
//
//
//
//
//
// CONFIG_AV1_HIGHBITDEPTH
#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_dc_predictor_32x32_avx2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let sum_above: __m256i = dc_sum_32(above);
    let mut sum_left: __m256i = dc_sum_32(left);
    sum_left = _mm256_add_epi16(sum_left, sum_above);
    let thirtytwo: __m256i = _mm256_set1_epi16(32);
    sum_left = _mm256_add_epi16(sum_left, thirtytwo);
    sum_left = _mm256_srai_epi16::<{ 6 }>(sum_left);
    let zero: __m256i = _mm256_setzero_si256();
    let mut row: __m256i = _mm256_shuffle_epi8(sum_left, zero);
    row_store_32xh(&raw mut row as *const __m256i, 32, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_dc_top_predictor_32x32_avx2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let mut sum: __m256i = dc_sum_32(above);
    let _ = left;
    let sixteen: __m256i = _mm256_set1_epi16(16);
    sum = _mm256_add_epi16(sum, sixteen);
    sum = _mm256_srai_epi16::<{ 5 }>(sum);
    let zero: __m256i = _mm256_setzero_si256();
    let mut row: __m256i = _mm256_shuffle_epi8(sum, zero);
    row_store_32xh(&raw mut row as *const __m256i, 32, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_dc_left_predictor_32x32_avx2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let mut sum: __m256i = dc_sum_32(left);
    let _ = above;
    let sixteen: __m256i = _mm256_set1_epi16(16);
    sum = _mm256_add_epi16(sum, sixteen);
    sum = _mm256_srai_epi16::<{ 5 }>(sum);
    let zero: __m256i = _mm256_setzero_si256();
    let mut row: __m256i = _mm256_shuffle_epi8(sum, zero);
    row_store_32xh(&raw mut row as *const __m256i, 32, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_dc_128_predictor_32x32_avx2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let _ = above;
    let _ = left;
    let row: __m256i = _mm256_set1_epi8(0x80_i32 as i8);
    row_store_32xh(&raw const row, 32, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_v_predictor_32x32_avx2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let row: __m256i = _mm256_loadu_si256(above as *const __m256i);
    let _ = left;
    row_store_32xh(&raw const row, 32, dst, stride);
}

// There are 32 rows togeter. This function does line:
// 0,1,2,3, and 16,17,18,19. The next call would do
// 4,5,6,7, and 20,21,22,23. So 4 times of calling
// would finish 32 rows.
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn h_predictor_32x8line(row: *const __m256i, mut dst: *mut u8, stride: isize) {
    let mut t: [__m256i; 4] = core::mem::zeroed();
    let mut m: __m256i = _mm256_setzero_si256();
    let inc: __m256i = _mm256_set1_epi8(4);
    let mut i: i32 = 0;
    {
        i = 0;
        while i < 4 {
            t[i as usize] = _mm256_shuffle_epi8(*row, m);
            let mut r0: __m256i = _mm256_permute2x128_si256::<0>(t[i as usize], t[i as usize]);
            let mut r1: __m256i = _mm256_permute2x128_si256::<0x11>(t[i as usize], t[i as usize]);
            _mm256_storeu_si256(dst as *mut __m256i, r0);
            _mm256_storeu_si256(dst.offset(stride << 4) as *mut __m256i, r1);
            dst = dst.offset(stride);
            m = _mm256_add_epi8(m, inc);
            i += 1;
        }
    }
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_h_predictor_32x32_avx2(
    mut dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let _ = above;
    let left_col: __m256i = _mm256_loadu_si256(left as *const __m256i);
    let mut u: __m256i = _mm256_unpacklo_epi8(left_col, left_col);
    let mut v: __m256i = _mm256_unpacklo_epi8(u, u);
    h_predictor_32x8line(&raw mut v as *const __m256i, dst, stride);
    dst = dst.offset(stride << 2);
    v = _mm256_unpackhi_epi8(u, u);
    h_predictor_32x8line(&raw mut v as *const __m256i, dst, stride);
    dst = dst.offset(stride << 2);
    u = _mm256_unpackhi_epi8(left_col, left_col);
    v = _mm256_unpacklo_epi8(u, u);
    h_predictor_32x8line(&raw mut v as *const __m256i, dst, stride);
    dst = dst.offset(stride << 2);
    v = _mm256_unpackhi_epi8(u, u);
    h_predictor_32x8line(&raw mut v as *const __m256i, dst, stride);
}

// -----------------------------------------------------------------------------
// Rectangle
#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_dc_predictor_32x16_avx2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let top_sum: __m128i = dc_sum_32_sse2(above);
    let mut left_sum: __m128i = dc_sum_16_sse2(left);
    left_sum = _mm_add_epi16(top_sum, left_sum);
    let mut sum: u16 = _mm_cvtsi128_si32(left_sum) as u16;
    sum += 24;
    sum /= 48;
    let row: __m256i = _mm256_set1_epi8(sum as i8);
    row_store_32xh(&raw const row, 16, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_dc_predictor_32x64_avx2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let sum_above: __m256i = dc_sum_32(above);
    let mut sum_left: __m256i = dc_sum_64(left);
    sum_left = _mm256_add_epi16(sum_left, sum_above);
    let mut sum: u16 = _mm_cvtsi128_si32(_mm256_castsi256_si128(sum_left)) as u16;
    sum += 48;
    sum /= 96;
    let row: __m256i = _mm256_set1_epi8(sum as i8);
    row_store_32xh(&raw const row, 64, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_dc_predictor_64x64_avx2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let sum_above: __m256i = dc_sum_64(above);
    let mut sum_left: __m256i = dc_sum_64(left);
    sum_left = _mm256_add_epi16(sum_left, sum_above);
    let mut sum: u16 = _mm_cvtsi128_si32(_mm256_castsi256_si128(sum_left)) as u16;
    sum += 64;
    sum /= 128;
    let row: __m256i = _mm256_set1_epi8(sum as i8);
    row_store_64xh(&raw const row, 64, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_dc_predictor_64x32_avx2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let sum_above: __m256i = dc_sum_64(above);
    let mut sum_left: __m256i = dc_sum_32(left);
    sum_left = _mm256_add_epi16(sum_left, sum_above);
    let mut sum: u16 = _mm_cvtsi128_si32(_mm256_castsi256_si128(sum_left)) as u16;
    sum += 48;
    sum /= 96;
    let row: __m256i = _mm256_set1_epi8(sum as i8);
    row_store_64xh(&raw const row, 32, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_dc_predictor_64x16_avx2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let sum_above: __m256i = dc_sum_64(above);
    let mut sum_left: __m256i = _mm256_castsi128_si256(dc_sum_16_sse2(left));
    sum_left = _mm256_add_epi16(sum_left, sum_above);
    let mut sum: u16 = _mm_cvtsi128_si32(_mm256_castsi256_si128(sum_left)) as u16;
    sum += 40;
    sum /= 80;
    let row: __m256i = _mm256_set1_epi8(sum as i8);
    row_store_64xh(&raw const row, 16, dst, stride);
}

// !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER
#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_dc_top_predictor_32x16_avx2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let mut sum: __m256i = dc_sum_32(above);
    let _ = left;
    let sixteen: __m256i = _mm256_set1_epi16(16);
    sum = _mm256_add_epi16(sum, sixteen);
    sum = _mm256_srai_epi16::<{ 5 }>(sum);
    let zero: __m256i = _mm256_setzero_si256();
    let mut row: __m256i = _mm256_shuffle_epi8(sum, zero);
    row_store_32xh(&raw mut row as *const __m256i, 16, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_dc_top_predictor_32x64_avx2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let mut sum: __m256i = dc_sum_32(above);
    let _ = left;
    let sixteen: __m256i = _mm256_set1_epi16(16);
    sum = _mm256_add_epi16(sum, sixteen);
    sum = _mm256_srai_epi16::<{ 5 }>(sum);
    let zero: __m256i = _mm256_setzero_si256();
    let mut row: __m256i = _mm256_shuffle_epi8(sum, zero);
    row_store_32xh(&raw mut row as *const __m256i, 64, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_dc_top_predictor_64x64_avx2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let mut sum: __m256i = dc_sum_64(above);
    let _ = left;
    let thirtytwo: __m256i = _mm256_set1_epi16(32);
    sum = _mm256_add_epi16(sum, thirtytwo);
    sum = _mm256_srai_epi16::<{ 6 }>(sum);
    let zero: __m256i = _mm256_setzero_si256();
    let mut row: __m256i = _mm256_shuffle_epi8(sum, zero);
    row_store_64xh(&raw mut row as *const __m256i, 64, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_dc_top_predictor_64x32_avx2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let mut sum: __m256i = dc_sum_64(above);
    let _ = left;
    let thirtytwo: __m256i = _mm256_set1_epi16(32);
    sum = _mm256_add_epi16(sum, thirtytwo);
    sum = _mm256_srai_epi16::<{ 6 }>(sum);
    let zero: __m256i = _mm256_setzero_si256();
    let mut row: __m256i = _mm256_shuffle_epi8(sum, zero);
    row_store_64xh(&raw mut row as *const __m256i, 32, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_dc_top_predictor_64x16_avx2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let mut sum: __m256i = dc_sum_64(above);
    let _ = left;
    let thirtytwo: __m256i = _mm256_set1_epi16(32);
    sum = _mm256_add_epi16(sum, thirtytwo);
    sum = _mm256_srai_epi16::<{ 6 }>(sum);
    let zero: __m256i = _mm256_setzero_si256();
    let mut row: __m256i = _mm256_shuffle_epi8(sum, zero);
    row_store_64xh(&raw mut row as *const __m256i, 16, dst, stride);
}

// !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER
#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_dc_left_predictor_32x16_avx2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let mut sum: __m128i = dc_sum_16_sse2(left);
    let _ = above;
    let eight: __m128i = _mm_set1_epi16(8);
    sum = _mm_add_epi16(sum, eight);
    sum = _mm_srai_epi16::<{ 4 }>(sum);
    let zero: __m128i = _mm_setzero_si128();
    let r: __m128i = _mm_shuffle_epi8(sum, zero);
    let row: __m256i = _mm256_inserti128_si256::<1>(_mm256_castsi128_si256(r), r);
    row_store_32xh(&raw const row, 16, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_dc_left_predictor_32x64_avx2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let mut sum: __m256i = dc_sum_64(left);
    let _ = above;
    let thirtytwo: __m256i = _mm256_set1_epi16(32);
    sum = _mm256_add_epi16(sum, thirtytwo);
    sum = _mm256_srai_epi16::<{ 6 }>(sum);
    let zero: __m256i = _mm256_setzero_si256();
    let mut row: __m256i = _mm256_shuffle_epi8(sum, zero);
    row_store_32xh(&raw mut row as *const __m256i, 64, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_dc_left_predictor_64x64_avx2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let mut sum: __m256i = dc_sum_64(left);
    let _ = above;
    let thirtytwo: __m256i = _mm256_set1_epi16(32);
    sum = _mm256_add_epi16(sum, thirtytwo);
    sum = _mm256_srai_epi16::<{ 6 }>(sum);
    let zero: __m256i = _mm256_setzero_si256();
    let mut row: __m256i = _mm256_shuffle_epi8(sum, zero);
    row_store_64xh(&raw mut row as *const __m256i, 64, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_dc_left_predictor_64x32_avx2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let mut sum: __m256i = dc_sum_32(left);
    let _ = above;
    let sixteen: __m256i = _mm256_set1_epi16(16);
    sum = _mm256_add_epi16(sum, sixteen);
    sum = _mm256_srai_epi16::<{ 5 }>(sum);
    let zero: __m256i = _mm256_setzero_si256();
    let mut row: __m256i = _mm256_shuffle_epi8(sum, zero);
    row_store_64xh(&raw mut row as *const __m256i, 32, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_dc_left_predictor_64x16_avx2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let mut sum: __m128i = dc_sum_16_sse2(left);
    let _ = above;
    let eight: __m128i = _mm_set1_epi16(8);
    sum = _mm_add_epi16(sum, eight);
    sum = _mm_srai_epi16::<{ 4 }>(sum);
    let zero: __m128i = _mm_setzero_si128();
    let r: __m128i = _mm_shuffle_epi8(sum, zero);
    let row: __m256i = _mm256_inserti128_si256::<1>(_mm256_castsi128_si256(r), r);
    row_store_64xh(&raw const row, 16, dst, stride);
}

// !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER
#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_dc_128_predictor_32x16_avx2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let _ = above;
    let _ = left;
    let row: __m256i = _mm256_set1_epi8(0x80_i32 as i8);
    row_store_32xh(&raw const row, 16, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_dc_128_predictor_32x64_avx2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let _ = above;
    let _ = left;
    let row: __m256i = _mm256_set1_epi8(0x80_i32 as i8);
    row_store_32xh(&raw const row, 64, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_dc_128_predictor_64x64_avx2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let _ = above;
    let _ = left;
    let row: __m256i = _mm256_set1_epi8(0x80_i32 as i8);
    row_store_64xh(&raw const row, 64, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_dc_128_predictor_64x32_avx2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let _ = above;
    let _ = left;
    let row: __m256i = _mm256_set1_epi8(0x80_i32 as i8);
    row_store_64xh(&raw const row, 32, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_dc_128_predictor_64x16_avx2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let _ = above;
    let _ = left;
    let row: __m256i = _mm256_set1_epi8(0x80_i32 as i8);
    row_store_64xh(&raw const row, 16, dst, stride);
}

// !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER
#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_v_predictor_32x16_avx2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let row: __m256i = _mm256_loadu_si256(above as *const __m256i);
    let _ = left;
    row_store_32xh(&raw const row, 16, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_v_predictor_32x64_avx2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let row: __m256i = _mm256_loadu_si256(above as *const __m256i);
    let _ = left;
    row_store_32xh(&raw const row, 64, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_v_predictor_64x64_avx2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let row0: __m256i = _mm256_loadu_si256(above as *const __m256i);
    let row1: __m256i = _mm256_loadu_si256(above.offset(32) as *const __m256i);
    let _ = left;
    row_store_32x2xh(&raw const row0, &raw const row1, 64, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_v_predictor_64x32_avx2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let row0: __m256i = _mm256_loadu_si256(above as *const __m256i);
    let row1: __m256i = _mm256_loadu_si256(above.offset(32) as *const __m256i);
    let _ = left;
    row_store_32x2xh(&raw const row0, &raw const row1, 32, dst, stride);
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_v_predictor_64x16_avx2(
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let row0: __m256i = _mm256_loadu_si256(above as *const __m256i);
    let row1: __m256i = _mm256_loadu_si256(above.offset(32) as *const __m256i);
    let _ = left;
    row_store_32x2xh(&raw const row0, &raw const row1, 16, dst, stride);
}

// !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER
// -----------------------------------------------------------------------------
// PAETH_PRED
// Return 16 16-bit pixels in one row (__m256i)
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn paeth_pred(
    left: *const __m256i,
    top: *const __m256i,
    topleft: *const __m256i,
) -> __m256i {
    let base: __m256i = _mm256_sub_epi16(_mm256_add_epi16(*top, *left), *topleft);
    let mut pl: __m256i = _mm256_abs_epi16(_mm256_sub_epi16(base, *left));
    let mut pt: __m256i = _mm256_abs_epi16(_mm256_sub_epi16(base, *top));
    let mut ptl: __m256i = _mm256_abs_epi16(_mm256_sub_epi16(base, *topleft));
    let mut mask1: __m256i = _mm256_cmpgt_epi16(pl, pt);
    mask1 = _mm256_or_si256(mask1, _mm256_cmpgt_epi16(pl, ptl));
    let mut mask2: __m256i = _mm256_cmpgt_epi16(pt, ptl);
    pl = _mm256_andnot_si256(mask1, *left);
    ptl = _mm256_and_si256(mask2, *topleft);
    pt = _mm256_andnot_si256(mask2, *top);
    pt = _mm256_or_si256(pt, ptl);
    pt = _mm256_and_si256(mask1, pt);
    return _mm256_or_si256(pt, pl);
}

// Return 16 8-bit pixels in one row (__m128i)
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn paeth_16x1_pred(
    left: *const __m256i,
    top: *const __m256i,
    topleft: *const __m256i,
) -> __m128i {
    let p0: __m256i = paeth_pred(left, top, topleft);
    let p1: __m256i = _mm256_permute4x64_epi64::<0xe>(p0);
    let p: __m256i = _mm256_packus_epi16(p0, p1);
    return _mm256_castsi256_si128(p);
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn get_top_vector(above: *const u8) -> __m256i {
    let x: __m128i = _mm_load_si128(above as *const __m128i);
    let zero: __m128i = _mm_setzero_si128();
    let t0: __m128i = _mm_unpacklo_epi8(x, zero);
    let t1: __m128i = _mm_unpackhi_epi8(x, zero);
    return _mm256_inserti128_si256::<1>(_mm256_castsi128_si256(t0), t1);
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_paeth_predictor_16x8_avx2(
    mut dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let mut x: __m128i = _mm_loadl_epi64(left as *const __m128i);
    let l: __m256i = _mm256_inserti128_si256::<1>(_mm256_castsi128_si256(x), x);
    let tl16: __m256i = _mm256_set1_epi16(*above.offset(-1) as i16);
    let mut rep: __m256i = _mm256_set1_epi16(0x8000_i32 as i16);
    let one: __m256i = _mm256_set1_epi16(1);
    let top: __m256i = get_top_vector(above);
    let mut i: i32 = 0;
    {
        i = 0;
        while i < 8 {
            let l16: __m256i = _mm256_shuffle_epi8(l, rep);
            let row: __m128i = paeth_16x1_pred(&raw const l16, &raw const top, &raw const tl16);
            _mm_store_si128(dst as *mut __m128i, row);
            dst = dst.offset(stride);
            rep = _mm256_add_epi16(rep, one);
            i += 1;
        }
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn get_left_vector(left: *const u8) -> __m256i {
    let x: __m128i = _mm_load_si128(left as *const __m128i);
    return _mm256_inserti128_si256::<1>(_mm256_castsi128_si256(x), x);
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_paeth_predictor_16x16_avx2(
    mut dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let l: __m256i = get_left_vector(left);
    let tl16: __m256i = _mm256_set1_epi16(*above.offset(-1) as i16);
    let mut rep: __m256i = _mm256_set1_epi16(0x8000_i32 as i16);
    let one: __m256i = _mm256_set1_epi16(1);
    let top: __m256i = get_top_vector(above);
    let mut i: i32 = 0;
    {
        i = 0;
        while i < 16 {
            let l16: __m256i = _mm256_shuffle_epi8(l, rep);
            let row: __m128i = paeth_16x1_pred(&raw const l16, &raw const top, &raw const tl16);
            _mm_store_si128(dst as *mut __m128i, row);
            dst = dst.offset(stride);
            rep = _mm256_add_epi16(rep, one);
            i += 1;
        }
    }
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_paeth_predictor_16x32_avx2(
    mut dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let mut l: __m256i = get_left_vector(left);
    let tl16: __m256i = _mm256_set1_epi16(*above.offset(-1) as i16);
    let mut rep: __m256i = _mm256_set1_epi16(0x8000_i32 as i16);
    let one: __m256i = _mm256_set1_epi16(1);
    let top: __m256i = get_top_vector(above);
    let mut i: i32 = 0;
    {
        i = 0;
        while i < 16 {
            let l16: __m256i = _mm256_shuffle_epi8(l, rep);
            let row: __m128i = paeth_16x1_pred(&raw const l16, &raw const top, &raw const tl16);
            _mm_store_si128(dst as *mut __m128i, row);
            dst = dst.offset(stride);
            rep = _mm256_add_epi16(rep, one);
            i += 1;
        }
    }
    l = get_left_vector(left.offset(16));
    rep = _mm256_set1_epi16(0x8000_i32 as i16);
    {
        i = 0;
        while i < 16 {
            let l16: __m256i = _mm256_shuffle_epi8(l, rep);
            let row: __m128i = paeth_16x1_pred(&raw const l16, &raw const top, &raw const tl16);
            _mm_store_si128(dst as *mut __m128i, row);
            dst = dst.offset(stride);
            rep = _mm256_add_epi16(rep, one);
            i += 1;
        }
    }
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_paeth_predictor_16x64_avx2(
    mut dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let tl16: __m256i = _mm256_set1_epi16(*above.offset(-1) as i16);
    let one: __m256i = _mm256_set1_epi16(1);
    let top: __m256i = get_top_vector(above);
    for j in 0..4 {
        let l: __m256i = get_left_vector(left.offset((j * 16) as isize));
        let mut rep: __m256i = _mm256_set1_epi16(0x8000_i32 as i16);
        for i in 0..16 {
            let l16: __m256i = _mm256_shuffle_epi8(l, rep);
            let row: __m128i = paeth_16x1_pred(&raw const l16, &raw const top, &raw const tl16);
            _mm_store_si128(dst as *mut __m128i, row);
            dst = dst.offset(stride);
            rep = _mm256_add_epi16(rep, one);
        }
    }
}

// !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER
// Return 32 8-bit pixels in one row (__m256i)
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn paeth_32x1_pred(
    left: *const __m256i,
    top0: *const __m256i,
    top1: *const __m256i,
    topleft: *const __m256i,
) -> __m256i {
    let mut p0: __m256i = paeth_pred(left, top0, topleft);
    let mut p1: __m256i = _mm256_permute4x64_epi64::<0xe>(p0);
    let x0: __m256i = _mm256_packus_epi16(p0, p1);
    p0 = paeth_pred(left, top1, topleft);
    p1 = _mm256_permute4x64_epi64::<0xe>(p0);
    let x1: __m256i = _mm256_packus_epi16(p0, p1);
    return _mm256_permute2x128_si256::<0x20>(x0, x1);
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_paeth_predictor_32x16_avx2(
    mut dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let l: __m256i = get_left_vector(left);
    let t0: __m256i = get_top_vector(above);
    let t1: __m256i = get_top_vector(above.offset(16));
    let tl: __m256i = _mm256_set1_epi16(*above.offset(-1) as i16);
    let mut rep: __m256i = _mm256_set1_epi16(0x8000_i32 as i16);
    let one: __m256i = _mm256_set1_epi16(1);
    let mut i: i32 = 0;
    {
        i = 0;
        while i < 16 {
            let l16: __m256i = _mm256_shuffle_epi8(l, rep);
            let r: __m256i =
                paeth_32x1_pred(&raw const l16, &raw const t0, &raw const t1, &raw const tl);
            _mm256_storeu_si256(dst as *mut __m256i, r);
            dst = dst.offset(stride);
            rep = _mm256_add_epi16(rep, one);
            i += 1;
        }
    }
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_paeth_predictor_32x32_avx2(
    mut dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let mut l: __m256i = get_left_vector(left);
    let t0: __m256i = get_top_vector(above);
    let t1: __m256i = get_top_vector(above.offset(16));
    let tl: __m256i = _mm256_set1_epi16(*above.offset(-1) as i16);
    let mut rep: __m256i = _mm256_set1_epi16(0x8000_i32 as i16);
    let one: __m256i = _mm256_set1_epi16(1);
    let mut i: i32 = 0;
    {
        i = 0;
        while i < 16 {
            let l16: __m256i = _mm256_shuffle_epi8(l, rep);
            let r0: __m128i = paeth_16x1_pred(&raw const l16, &raw const t0, &raw const tl);
            let r1: __m128i = paeth_16x1_pred(&raw const l16, &raw const t1, &raw const tl);
            _mm_store_si128(dst as *mut __m128i, r0);
            _mm_store_si128(dst.offset(16) as *mut __m128i, r1);
            dst = dst.offset(stride);
            rep = _mm256_add_epi16(rep, one);
            i += 1;
        }
    }
    l = get_left_vector(left.offset(16));
    rep = _mm256_set1_epi16(0x8000_i32 as i16);
    {
        i = 0;
        while i < 16 {
            let l16: __m256i = _mm256_shuffle_epi8(l, rep);
            let r0: __m128i = paeth_16x1_pred(&raw const l16, &raw const t0, &raw const tl);
            let r1: __m128i = paeth_16x1_pred(&raw const l16, &raw const t1, &raw const tl);
            _mm_store_si128(dst as *mut __m128i, r0);
            _mm_store_si128(dst.offset(16) as *mut __m128i, r1);
            dst = dst.offset(stride);
            rep = _mm256_add_epi16(rep, one);
            i += 1;
        }
    }
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_paeth_predictor_32x64_avx2(
    mut dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let t0: __m256i = get_top_vector(above);
    let t1: __m256i = get_top_vector(above.offset(16));
    let tl: __m256i = _mm256_set1_epi16(*above.offset(-1) as i16);
    let one: __m256i = _mm256_set1_epi16(1);
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    {
        j = 0;
        while j < 4 {
            let l: __m256i = get_left_vector(left.offset((j * 16) as isize));
            let mut rep: __m256i = _mm256_set1_epi16(0x8000_i32 as i16);
            {
                i = 0;
                while i < 16 {
                    let l16: __m256i = _mm256_shuffle_epi8(l, rep);
                    let r0: __m128i = paeth_16x1_pred(&raw const l16, &raw const t0, &raw const tl);
                    let r1: __m128i = paeth_16x1_pred(&raw const l16, &raw const t1, &raw const tl);
                    _mm_store_si128(dst as *mut __m128i, r0);
                    _mm_store_si128(dst.offset(16) as *mut __m128i, r1);
                    dst = dst.offset(stride);
                    rep = _mm256_add_epi16(rep, one);
                    i += 1;
                }
            }
            j += 1;
        }
    }
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_paeth_predictor_64x32_avx2(
    mut dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let t0: __m256i = get_top_vector(above);
    let t1: __m256i = get_top_vector(above.offset(16));
    let t2: __m256i = get_top_vector(above.offset(32));
    let t3: __m256i = get_top_vector(above.offset(48));
    let tl: __m256i = _mm256_set1_epi16(*above.offset(-1) as i16);
    let one: __m256i = _mm256_set1_epi16(1);
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    {
        j = 0;
        while j < 2 {
            let l: __m256i = get_left_vector(left.offset((j * 16) as isize));
            let mut rep: __m256i = _mm256_set1_epi16(0x8000_i32 as i16);
            {
                i = 0;
                while i < 16 {
                    let l16: __m256i = _mm256_shuffle_epi8(l, rep);
                    let r0: __m128i = paeth_16x1_pred(&raw const l16, &raw const t0, &raw const tl);
                    let r1: __m128i = paeth_16x1_pred(&raw const l16, &raw const t1, &raw const tl);
                    let r2: __m128i = paeth_16x1_pred(&raw const l16, &raw const t2, &raw const tl);
                    let r3: __m128i = paeth_16x1_pred(&raw const l16, &raw const t3, &raw const tl);
                    _mm_store_si128(dst as *mut __m128i, r0);
                    _mm_store_si128(dst.offset(16) as *mut __m128i, r1);
                    _mm_store_si128(dst.offset(32) as *mut __m128i, r2);
                    _mm_store_si128(dst.offset(48) as *mut __m128i, r3);
                    dst = dst.offset(stride);
                    rep = _mm256_add_epi16(rep, one);
                    i += 1;
                }
            }
            j += 1;
        }
    }
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_paeth_predictor_64x64_avx2(
    mut dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let t0: __m256i = get_top_vector(above);
    let t1: __m256i = get_top_vector(above.offset(16));
    let t2: __m256i = get_top_vector(above.offset(32));
    let t3: __m256i = get_top_vector(above.offset(48));
    let tl: __m256i = _mm256_set1_epi16(*above.offset(-1) as i16);
    let one: __m256i = _mm256_set1_epi16(1);
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    {
        j = 0;
        while j < 4 {
            let l: __m256i = get_left_vector(left.offset((j * 16) as isize));
            let mut rep: __m256i = _mm256_set1_epi16(0x8000_i32 as i16);
            {
                i = 0;
                while i < 16 {
                    let l16: __m256i = _mm256_shuffle_epi8(l, rep);
                    let r0: __m128i = paeth_16x1_pred(&raw const l16, &raw const t0, &raw const tl);
                    let r1: __m128i = paeth_16x1_pred(&raw const l16, &raw const t1, &raw const tl);
                    let r2: __m128i = paeth_16x1_pred(&raw const l16, &raw const t2, &raw const tl);
                    let r3: __m128i = paeth_16x1_pred(&raw const l16, &raw const t3, &raw const tl);
                    _mm_store_si128(dst as *mut __m128i, r0);
                    _mm_store_si128(dst.offset(16) as *mut __m128i, r1);
                    _mm_store_si128(dst.offset(32) as *mut __m128i, r2);
                    _mm_store_si128(dst.offset(48) as *mut __m128i, r3);
                    dst = dst.offset(stride);
                    rep = _mm256_add_epi16(rep, one);
                    i += 1;
                }
            }
            j += 1;
        }
    }
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_paeth_predictor_64x16_avx2(
    mut dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
) {
    let t0: __m256i = get_top_vector(above);
    let t1: __m256i = get_top_vector(above.offset(16));
    let t2: __m256i = get_top_vector(above.offset(32));
    let t3: __m256i = get_top_vector(above.offset(48));
    let tl: __m256i = _mm256_set1_epi16(*above.offset(-1) as i16);
    let one: __m256i = _mm256_set1_epi16(1);
    let mut i: i32 = 0;
    let l: __m256i = get_left_vector(left);
    let mut rep: __m256i = _mm256_set1_epi16(0x8000_i32 as i16);
    {
        i = 0;
        while i < 16 {
            let l16: __m256i = _mm256_shuffle_epi8(l, rep);
            let r0: __m128i = paeth_16x1_pred(&raw const l16, &raw const t0, &raw const tl);
            let r1: __m128i = paeth_16x1_pred(&raw const l16, &raw const t1, &raw const tl);
            let r2: __m128i = paeth_16x1_pred(&raw const l16, &raw const t2, &raw const tl);
            let r3: __m128i = paeth_16x1_pred(&raw const l16, &raw const t3, &raw const tl);
            _mm_store_si128(dst as *mut __m128i, r0);
            _mm_store_si128(dst.offset(16) as *mut __m128i, r1);
            _mm_store_si128(dst.offset(32) as *mut __m128i, r2);
            _mm_store_si128(dst.offset(48) as *mut __m128i, r3);
            dst = dst.offset(stride);
            rep = _mm256_add_epi16(rep, one);
            i += 1;
        }
    }
}

// !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER
// !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER
// CONFIG_AV1_HIGHBITDEPTH
// Low bit depth functions
static BaseMask: Aligned32<[[u8; 32]; 33]> = Aligned32([
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ],
    [
        0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0,
    ],
    [
        0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ],
    [
        0xff, 0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ],
    [
        0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0,
    ],
    [
        0xff, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0,
    ],
    [
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0, 0,
    ],
    [
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0,
    ],
    [
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0,
    ],
    [
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0, 0,
        0,
    ],
    [
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0, 0,
    ],
    [
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0,
    ],
    [
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff,
    ],
]);

// clang-format on
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn dr_prediction_z1_HxW_internal_avx2(
    H: i32,
    W: i32,
    dst: *mut __m128i,
    above: *const u8,
    upsample_above: i32,
    dx: i32,
) {
    let frac_bits: i32 = 6 - upsample_above;
    let max_base_x: i32 = (W + H - 1) << upsample_above;
    debug_assert!(dx > 0);
    // pre-filter above pixels
    // store in temp buffers:
    //   above[x] * 32 + 16
    //   above[x+1] - above[x]
    // final pixels will be calculated as:
    //   (above[x] * 32 + 16 + (above[x+1] - above[x]) * shift) >> 5
    let mut a0: __m256i;
    let mut a1: __m256i;
    let mut a32: __m256i;
    let mut a16: __m256i;
    let mut diff: __m256i;
    let mut c3f: __m256i;
    let mut a_mbase_x: __m128i;
    a16 = _mm256_set1_epi16(16);
    a_mbase_x = _mm_set1_epi8(*above.offset(max_base_x as isize) as i8);
    c3f = _mm256_set1_epi16(0x3f);
    let mut x: i32 = dx;
    for r in 0..W {
        let mut b: __m256i;
        let mut res: __m256i;
        let mut shift: __m256i;
        let mut res1: __m128i;
        let mut a0_128: __m128i;
        let mut a1_128: __m128i;
        let mut base: i32 = x >> frac_bits;
        let mut base_max_diff: i32 = (max_base_x - base) >> upsample_above;
        if base_max_diff <= 0 {
            for i in r..W {
                *dst.offset(i as isize) = a_mbase_x;
            } // save 4 values
            return;
        }
        if base_max_diff > H {
            base_max_diff = H;
        }
        a0_128 = _mm_loadu_si128(above.offset(base as isize) as *mut __m128i as *const __m128i);
        a1_128 = _mm_loadu_si128(
            above.offset(base as isize).offset(1) as *mut __m128i as *const __m128i
        );
        if upsample_above != 0 {
            a0_128 = _mm_shuffle_epi8(a0_128, *(EvenOddMaskx.0[0].as_ptr() as *mut __m128i));
            a1_128 = _mm_srli_si128::<8>(a0_128);
            shift = _mm256_srli_epi16::<{ 1 }>(_mm256_and_si256(
                _mm256_sll_epi16(
                    _mm256_set1_epi16(x as i16),
                    _mm_cvtsi32_si128(upsample_above),
                ),
                c3f,
            ));
        } else {
            shift = _mm256_srli_epi16::<{ 1 }>(_mm256_and_si256(_mm256_set1_epi16(x as i16), c3f));
        }
        a0 = _mm256_cvtepu8_epi16(a0_128);
        a1 = _mm256_cvtepu8_epi16(a1_128);
        diff = _mm256_sub_epi16(a1, a0); // a[x+1] - a[x]
        a32 = _mm256_slli_epi16::<{ 5 }>(a0); // a[x] * 32
        a32 = _mm256_add_epi16(a32, a16); // a[x] * 32 + 16
        b = _mm256_mullo_epi16(diff, shift);
        res = _mm256_add_epi16(a32, b);
        res = _mm256_srli_epi16::<{ 5 }>(res);
        res = _mm256_packus_epi16(
            res,
            _mm256_castsi128_si256(_mm256_extracti128_si256::<1>(res)),
        ); // goto 8 bit
        res1 = _mm256_castsi256_si128(res); // 16 8bit values
        *dst.offset(r as isize) = _mm_blendv_epi8(
            a_mbase_x,
            res1,
            *(BaseMask.0[base_max_diff as usize].as_ptr() as *mut __m128i),
        );
        x += dx;
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn dr_prediction_z1_4xN_avx2(
    N: i32,
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    upsample_above: i32,
    dx: i32,
) {
    let mut dstvec: [__m128i; 16] = core::mem::zeroed();
    dr_prediction_z1_HxW_internal_avx2(4, N, dstvec.as_mut_ptr(), above, upsample_above, dx);
    for i in 0..N {
        core::ptr::write_unaligned(
            dst.offset(stride * i as isize) as *mut i32,
            _mm_cvtsi128_si32(dstvec[i as usize]),
        );
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn dr_prediction_z1_8xN_avx2(
    N: i32,
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    upsample_above: i32,
    dx: i32,
) {
    let mut dstvec: [__m128i; 32] = core::mem::zeroed();
    dr_prediction_z1_HxW_internal_avx2(8, N, dstvec.as_mut_ptr(), above, upsample_above, dx);
    for i in 0..N {
        _mm_storel_epi64(
            dst.offset(stride * i as isize) as *mut __m128i,
            dstvec[i as usize],
        );
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn dr_prediction_z1_16xN_avx2(
    N: i32,
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    upsample_above: i32,
    dx: i32,
) {
    let mut dstvec: [__m128i; 64] = core::mem::zeroed();
    dr_prediction_z1_HxW_internal_avx2(16, N, dstvec.as_mut_ptr(), above, upsample_above, dx);
    for i in 0..N {
        _mm_storeu_si128(
            dst.offset(stride * i as isize) as *mut __m128i,
            dstvec[i as usize],
        );
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn dr_prediction_z1_32xN_internal_avx2(
    N: i32,
    dstvec: *mut __m256i,
    above: *const u8,
    upsample_above: i32,
    dx: i32,
) {
    // here upsample_above is 0 by design of av1_use_intra_edge_upsample
    let _ = upsample_above;
    let frac_bits: i32 = 6;
    let max_base_x: i32 = 32 + N - 1;
    // pre-filter above pixels
    // store in temp buffers:
    //   above[x] * 32 + 16
    //   above[x+1] - above[x]
    // final pixels will be calculated as:
    //   (above[x] * 32 + 16 + (above[x+1] - above[x]) * shift) >> 5
    let mut a0: __m256i;
    let mut a1: __m256i;
    let mut a32: __m256i;
    let mut a16: __m256i;
    let mut a_mbase_x: __m256i;
    let mut diff: __m256i;
    let mut c3f: __m256i;
    a16 = _mm256_set1_epi16(16);
    a_mbase_x = _mm256_set1_epi8(*above.offset(max_base_x as isize) as i8);
    c3f = _mm256_set1_epi16(0x3f);
    let mut x: i32 = dx;
    for r in 0..N {
        let mut b: __m256i;
        let mut res: __m256i;
        let mut res16: [__m256i; 2] = core::mem::zeroed();
        let mut a0_128: __m128i;
        let mut a1_128: __m128i;
        let mut base: i32 = x >> frac_bits;
        let mut base_max_diff: i32 = max_base_x - base;
        if base_max_diff <= 0 {
            for i in r..N {
                *dstvec.offset(i as isize) = a_mbase_x;
            } // save 32 values
            return;
        }
        if base_max_diff > 32 {
            base_max_diff = 32;
        }
        let mut shift: __m256i =
            _mm256_srli_epi16::<{ 1 }>(_mm256_and_si256(_mm256_set1_epi16(x as i16), c3f));
        {
            let mut j: i32 = 0;
            let mut jj: i32 = 0;
            while j < 32 {
                let mut mdiff: i32 = base_max_diff - j;
                if mdiff <= 0 {
                    res16[jj as usize] = a_mbase_x;
                } else {
                    a0_128 = _mm_loadu_si128(above.offset(base as isize).offset(j as isize)
                        as *mut __m128i
                        as *const __m128i);
                    a1_128 =
                        _mm_loadu_si128(above.offset(base as isize).offset(j as isize).offset(1)
                            as *mut __m128i
                            as *const __m128i);
                    a0 = _mm256_cvtepu8_epi16(a0_128);
                    a1 = _mm256_cvtepu8_epi16(a1_128);
                    diff = _mm256_sub_epi16(a1, a0); // a[x+1] - a[x]
                    a32 = _mm256_slli_epi16::<{ 5 }>(a0); // a[x] * 32
                    a32 = _mm256_add_epi16(a32, a16); // a[x] * 32 + 16
                    b = _mm256_mullo_epi16(diff, shift);
                    res = _mm256_add_epi16(a32, b);
                    res = _mm256_srli_epi16::<{ 5 }>(res);
                    res16[jj as usize] = _mm256_packus_epi16(
                        res,
                        _mm256_castsi128_si256(_mm256_extracti128_si256::<1>(res)),
                    );
                }
                j += 16;
                jj += 1;
            }
        } // 16 8bit values
        res16[1] = _mm256_inserti128_si256::<1>(res16[0], _mm256_castsi256_si128(res16[1])); // 32 8bit values
        *dstvec.offset(r as isize) = _mm256_blendv_epi8(
            a_mbase_x,
            res16[1],
            *(BaseMask.0[base_max_diff as usize].as_ptr() as *mut __m256i),
        ); // 32 8bit values
        x += dx;
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn dr_prediction_z1_32xN_avx2(
    N: i32,
    dst: *mut u8,
    stride: isize,
    above: *const u8,
    upsample_above: i32,
    dx: i32,
) {
    let mut dstvec: [__m256i; 64] = core::mem::zeroed();
    dr_prediction_z1_32xN_internal_avx2(N, dstvec.as_mut_ptr(), above, upsample_above, dx);
    for i in 0..N {
        _mm256_storeu_si256(
            dst.offset(stride * i as isize) as *mut __m256i,
            dstvec[i as usize],
        );
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn dr_prediction_z1_64xN_avx2(
    N: i32,
    mut dst: *mut u8,
    stride: isize,
    above: *const u8,
    upsample_above: i32,
    dx: i32,
) {
    // here upsample_above is 0 by design of av1_use_intra_edge_upsample
    let _ = upsample_above;
    let frac_bits: i32 = 6;
    let max_base_x: i32 = 64 + N - 1;
    // pre-filter above pixels
    // store in temp buffers:
    //   above[x] * 32 + 16
    //   above[x+1] - above[x]
    // final pixels will be calculated as:
    //   (above[x] * 32 + 16 + (above[x+1] - above[x]) * shift) >> 5
    let mut a0: __m256i;
    let mut a1: __m256i;
    let mut a32: __m256i;
    let mut a16: __m256i;
    let mut a_mbase_x: __m256i;
    let mut diff: __m256i;
    let mut c3f: __m256i;
    let mut max_base_x128: __m128i;
    let mut base_inc128: __m128i;
    let mut mask128: __m128i;
    a16 = _mm256_set1_epi16(16);
    a_mbase_x = _mm256_set1_epi8(*above.offset(max_base_x as isize) as i8);
    max_base_x128 = _mm_set1_epi8(max_base_x as i8);
    c3f = _mm256_set1_epi16(0x3f);
    let mut x: i32 = dx;
    {
        let mut r: i32 = 0;
        while r < N {
            let mut b: __m256i;
            let mut res: __m256i;
            let mut base: i32 = x >> frac_bits;
            if base >= max_base_x {
                for i in r..N {
                    _mm256_storeu_si256(dst as *mut __m256i, a_mbase_x); // save 32 values
                    _mm256_storeu_si256(dst.offset(32) as *mut __m256i, a_mbase_x);
                    dst = dst.offset(stride);
                }
                return;
            }
            let mut shift: __m256i =
                _mm256_srli_epi16::<{ 1 }>(_mm256_and_si256(_mm256_set1_epi16(x as i16), c3f));
            let mut a0_128: __m128i;
            let mut a1_128: __m128i;
            let mut res128: __m128i;
            for j in (0..64).step_by(16) {
                let mut mdif: i32 = max_base_x - (base + j);
                if mdif <= 0 {
                    _mm_storeu_si128(
                        dst.offset(j as isize) as *mut __m128i,
                        _mm256_castsi256_si128(a_mbase_x),
                    );
                } else {
                    a0_128 = _mm_loadu_si128(above.offset(base as isize).offset(j as isize)
                        as *mut __m128i
                        as *const __m128i);
                    a1_128 =
                        _mm_loadu_si128(above.offset(base as isize).offset(1).offset(j as isize)
                            as *mut __m128i
                            as *const __m128i);
                    a0 = _mm256_cvtepu8_epi16(a0_128);
                    a1 = _mm256_cvtepu8_epi16(a1_128);
                    diff = _mm256_sub_epi16(a1, a0); // a[x+1] - a[x]
                    a32 = _mm256_slli_epi16::<{ 5 }>(a0); // a[x] * 32
                    a32 = _mm256_add_epi16(a32, a16); // a[x] * 32 + 16
                    b = _mm256_mullo_epi16(diff, shift);
                    res = _mm256_add_epi16(a32, b);
                    res = _mm256_srli_epi16::<{ 5 }>(res);
                    res = _mm256_packus_epi16(
                        res,
                        _mm256_castsi128_si256(_mm256_extracti128_si256::<1>(res)),
                    ); // 16 8bit values
                    base_inc128 = _mm_setr_epi8(
                        (base + j) as i8,
                        (base + j + 1) as i8,
                        (base + j + 2) as i8,
                        (base + j + 3) as i8,
                        (base + j + 4) as i8,
                        (base + j + 5) as i8,
                        (base + j + 6) as i8,
                        (base + j + 7) as i8,
                        (base + j + 8) as i8,
                        (base + j + 9) as i8,
                        (base + j + 10) as i8,
                        (base + j + 11) as i8,
                        (base + j + 12) as i8,
                        (base + j + 13) as i8,
                        (base + j + 14) as i8,
                        (base + j + 15) as i8,
                    );
                    mask128 = _mm_cmpgt_epi8(
                        _mm_subs_epu8(max_base_x128, base_inc128),
                        _mm_setzero_si128(),
                    );
                    res128 = _mm_blendv_epi8(
                        _mm256_castsi256_si128(a_mbase_x),
                        _mm256_castsi256_si128(res),
                        mask128,
                    );
                    _mm_storeu_si128(dst.offset(j as isize) as *mut __m128i, res128);
                }
            }
            x += dx;
            r += 1;
            dst = dst.offset(stride);
        }
    }
}

// Directional prediction, zone 1: 0 < angle < 90
#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn av1_dr_prediction_z1_avx2(
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
    let _ = left;
    let _ = dy;
    match bw {
        4 => {
            dr_prediction_z1_4xN_avx2(bh, dst, stride, above, upsample_above, dx);
        }
        8 => {
            dr_prediction_z1_8xN_avx2(bh, dst, stride, above, upsample_above, dx);
        }
        16 => {
            dr_prediction_z1_16xN_avx2(bh, dst, stride, above, upsample_above, dx);
        }
        32 => {
            dr_prediction_z1_32xN_avx2(bh, dst, stride, above, upsample_above, dx);
        }
        64 => {
            dr_prediction_z1_64xN_avx2(bh, dst, stride, above, upsample_above, dx);
        }
        _ => {}
    }
    return;
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn dr_prediction_z2_Nx4_avx2(
    N: i32,
    mut dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
    upsample_above: i32,
    upsample_left: i32,
    dx: i32,
    dy: i32,
) {
    let min_base_x: i32 = -(1 << upsample_above);
    let min_base_y: i32 = -(1 << upsample_left);
    let frac_bits_x: i32 = 6 - upsample_above;
    let frac_bits_y: i32 = 6 - upsample_left;
    debug_assert!(dx > 0);
    // pre-filter above pixels
    // store in temp buffers:
    //   above[x] * 32 + 16
    //   above[x+1] - above[x]
    // final pixels will be calculated as:
    //   (above[x] * 32 + 16 + (above[x+1] - above[x]) * shift) >> 5
    let mut a0_x: __m128i;
    let mut a1_x: __m128i;
    let mut a32: __m128i;
    let mut a16: __m128i;
    let mut diff: __m128i;
    let mut c3f: __m128i;
    let mut min_base_y128: __m128i;
    let mut c1234: __m128i;
    let mut dy128: __m128i;
    a16 = _mm_set1_epi16(16);
    c3f = _mm_set1_epi16(0x3f);
    min_base_y128 = _mm_set1_epi16(min_base_y as i16);
    c1234 = _mm_setr_epi16(0, 1, 2, 3, 4, 0, 0, 0);
    dy128 = _mm_set1_epi16(dy as i16);
    for r in 0..N {
        let mut b: __m128i;
        let mut res: __m128i;
        let mut shift: __m128i;
        let mut r6: __m128i;
        let mut ydx: __m128i;
        let mut resx: __m128i;
        let mut resy: __m128i;
        let mut resxy: __m128i;
        let mut a0_x128: __m128i;
        let mut a1_x128: __m128i;
        let mut y: i32 = r + 1;
        let mut base_x: i32 = (-y * dx) >> frac_bits_x;
        let mut base_shift: i32 = 0;
        if base_x < min_base_x - 1 {
            base_shift = (min_base_x - base_x - 1) >> upsample_above;
        }
        let mut base_min_diff: i32 = (min_base_x - base_x + upsample_above) >> upsample_above;
        if base_min_diff > 4 {
            base_min_diff = 4;
        } else {
            if base_min_diff < 0 {
                base_min_diff = 0;
            }
        }
        if base_shift > 3 {
            a0_x = _mm_setzero_si128();
            a1_x = _mm_setzero_si128();
            shift = _mm_setzero_si128();
        } else {
            a0_x128 = _mm_loadu_si128(above.offset(base_x as isize).offset(base_shift as isize)
                as *mut __m128i as *const __m128i);
            ydx = _mm_set1_epi16((y * dx) as i16);
            r6 = _mm_slli_epi16::<{ 6 }>(c1234);
            if upsample_above != 0 {
                a0_x128 = _mm_shuffle_epi8(
                    a0_x128,
                    *(EvenOddMaskx.0[base_shift as usize].as_ptr() as *mut __m128i),
                );
                a1_x128 = _mm_srli_si128::<8>(a0_x128);
                shift = _mm_srli_epi16::<{ 1 }>(_mm_and_si128(
                    _mm_sll_epi16(_mm_sub_epi16(r6, ydx), _mm_cvtsi32_si128(upsample_above)),
                    c3f,
                ));
            } else {
                a0_x128 = _mm_shuffle_epi8(
                    a0_x128,
                    *(LoadMaskx.0[base_shift as usize].as_ptr() as *mut __m128i),
                );
                a1_x128 = _mm_srli_si128::<1>(a0_x128);
                shift = _mm_srli_epi16::<{ 1 }>(_mm_and_si128(_mm_sub_epi16(r6, ydx), c3f));
            }
            a0_x = _mm_cvtepu8_epi16(a0_x128);
            a1_x = _mm_cvtepu8_epi16(a1_x128);
        }
        // y calc
        let mut a0_y: __m128i;
        let mut a1_y: __m128i;
        let mut shifty: __m128i;
        if base_x < min_base_x {
            let mut base_y_c: Aligned32<[i16; 8]> = core::mem::zeroed();
            let mut y_c128: __m128i;
            let mut base_y_c128: __m128i;
            let mut mask128: __m128i;
            let mut c1234_: __m128i;
            c1234_ = _mm_srli_si128::<2>(c1234);
            r6 = _mm_set1_epi16((r << 6) as i16);
            y_c128 = _mm_sub_epi16(r6, _mm_mullo_epi16(c1234_, dy128));
            base_y_c128 = _mm_sra_epi16(y_c128, _mm_cvtsi32_si128(frac_bits_y));
            mask128 = _mm_cmpgt_epi16(min_base_y128, base_y_c128);
            base_y_c128 = _mm_andnot_si128(mask128, base_y_c128);
            _mm_store_si128(base_y_c.0.as_mut_ptr() as *mut __m128i, base_y_c128);
            a0_y = _mm_setr_epi16(
                *left.offset(base_y_c.0[0] as isize) as i16,
                *left.offset(base_y_c.0[1] as isize) as i16,
                *left.offset(base_y_c.0[2] as isize) as i16,
                *left.offset(base_y_c.0[3] as isize) as i16,
                0,
                0,
                0,
                0,
            );
            base_y_c128 = _mm_add_epi16(base_y_c128, _mm_srli_epi16::<{ 4 }>(a16));
            _mm_store_si128(base_y_c.0.as_mut_ptr() as *mut __m128i, base_y_c128);
            a1_y = _mm_setr_epi16(
                *left.offset(base_y_c.0[0] as isize) as i16,
                *left.offset(base_y_c.0[1] as isize) as i16,
                *left.offset(base_y_c.0[2] as isize) as i16,
                *left.offset(base_y_c.0[3] as isize) as i16,
                0,
                0,
                0,
                0,
            );
            if upsample_left != 0 {
                shifty = _mm_srli_epi16::<{ 1 }>(_mm_and_si128(
                    _mm_sll_epi16(y_c128, _mm_cvtsi32_si128(upsample_left)),
                    c3f,
                ));
            } else {
                shifty = _mm_srli_epi16::<{ 1 }>(_mm_and_si128(y_c128, c3f));
            }
            a0_x = _mm_unpacklo_epi64(a0_x, a0_y);
            a1_x = _mm_unpacklo_epi64(a1_x, a1_y);
            shift = _mm_unpacklo_epi64(shift, shifty);
        }
        diff = _mm_sub_epi16(a1_x, a0_x); // a[x+1] - a[x]
        a32 = _mm_slli_epi16::<{ 5 }>(a0_x); // a[x] * 32
        a32 = _mm_add_epi16(a32, a16); // a[x] * 32 + 16
        b = _mm_mullo_epi16(diff, shift);
        res = _mm_add_epi16(a32, b);
        res = _mm_srli_epi16::<{ 5 }>(res);
        resx = _mm_packus_epi16(res, res);
        resy = _mm_srli_si128::<4>(resx);
        resxy = _mm_blendv_epi8(
            resx,
            resy,
            *(BaseMask.0[base_min_diff as usize].as_ptr() as *mut __m128i),
        );
        core::ptr::write_unaligned(dst as *mut i32, _mm_cvtsi128_si32(resxy));
        dst = dst.offset(stride);
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn dr_prediction_z2_Nx8_avx2(
    N: i32,
    mut dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
    upsample_above: i32,
    upsample_left: i32,
    dx: i32,
    dy: i32,
) {
    let min_base_x: i32 = -(1 << upsample_above);
    let min_base_y: i32 = -(1 << upsample_left);
    let frac_bits_x: i32 = 6 - upsample_above;
    let frac_bits_y: i32 = 6 - upsample_left;
    // pre-filter above pixels
    // store in temp buffers:
    //   above[x] * 32 + 16
    //   above[x+1] - above[x]
    // final pixels will be calculated as:
    //   (above[x] * 32 + 16 + (above[x+1] - above[x]) * shift) >> 5
    let mut diff: __m256i;
    let mut a32: __m256i;
    let mut a16: __m256i;
    let mut a0_x: __m256i;
    let mut a1_x: __m256i;
    let mut a0_x128: __m128i;
    let mut a1_x128: __m128i;
    let mut min_base_y128: __m128i;
    let mut c3f: __m128i;
    let mut c1234: __m128i;
    let mut dy128: __m128i;
    a16 = _mm256_set1_epi16(16);
    c3f = _mm_set1_epi16(0x3f);
    min_base_y128 = _mm_set1_epi16(min_base_y as i16);
    dy128 = _mm_set1_epi16(dy as i16);
    c1234 = _mm_setr_epi16(1, 2, 3, 4, 5, 6, 7, 8);
    for r in 0..N {
        let mut b: __m256i;
        let mut res: __m256i;
        let mut shift: __m256i;
        let mut resx: __m128i;
        let mut resy: __m128i;
        let mut resxy: __m128i;
        let mut r6: __m128i;
        let mut ydx: __m128i;
        let mut y: i32 = r + 1;
        let mut base_x: i32 = (-y * dx) >> frac_bits_x;
        let mut base_shift: i32 = 0;
        if base_x < min_base_x - 1 {
            base_shift = (min_base_x - base_x - 1) >> upsample_above;
        }
        let mut base_min_diff: i32 = (min_base_x - base_x + upsample_above) >> upsample_above;
        if base_min_diff > 8 {
            base_min_diff = 8;
        } else {
            if base_min_diff < 0 {
                base_min_diff = 0;
            }
        }
        if base_shift > 7 {
            a0_x = _mm256_setzero_si256();
            a1_x = _mm256_setzero_si256();
            shift = _mm256_setzero_si256();
        } else {
            a0_x128 = _mm_loadu_si128(above.offset(base_x as isize).offset(base_shift as isize)
                as *mut __m128i as *const __m128i);
            ydx = _mm_set1_epi16((y * dx) as i16);
            r6 = _mm_slli_epi16::<{ 6 }>(_mm_srli_si128::<2>(c1234));
            if upsample_above != 0 {
                a0_x128 = _mm_shuffle_epi8(
                    a0_x128,
                    *(EvenOddMaskx.0[base_shift as usize].as_ptr() as *mut __m128i),
                );
                a1_x128 = _mm_srli_si128::<8>(a0_x128);
                shift = _mm256_castsi128_si256(_mm_srli_epi16::<{ 1 }>(_mm_and_si128(
                    _mm_sll_epi16(_mm_sub_epi16(r6, ydx), _mm_cvtsi32_si128(upsample_above)),
                    c3f,
                )));
            } else {
                a1_x128 = _mm_srli_si128::<1>(a0_x128);
                a0_x128 = _mm_shuffle_epi8(
                    a0_x128,
                    *(LoadMaskx.0[base_shift as usize].as_ptr() as *mut __m128i),
                );
                a1_x128 = _mm_shuffle_epi8(
                    a1_x128,
                    *(LoadMaskx.0[base_shift as usize].as_ptr() as *mut __m128i),
                );
                shift = _mm256_castsi128_si256(_mm_srli_epi16::<{ 1 }>(_mm_and_si128(
                    _mm_sub_epi16(r6, ydx),
                    c3f,
                )));
            }
            a0_x = _mm256_castsi128_si256(_mm_cvtepu8_epi16(a0_x128));
            a1_x = _mm256_castsi128_si256(_mm_cvtepu8_epi16(a1_x128));
        }
        // y calc
        let mut a0_y: __m128i;
        let mut a1_y: __m128i;
        let mut shifty: __m128i;
        if base_x < min_base_x {
            let mut base_y_c: Aligned32<[i16; 16]> = core::mem::zeroed();
            let mut y_c128: __m128i;
            let mut base_y_c128: __m128i;
            let mut mask128: __m128i;
            r6 = _mm_set1_epi16((r << 6) as i16);
            y_c128 = _mm_sub_epi16(r6, _mm_mullo_epi16(c1234, dy128));
            base_y_c128 = _mm_sra_epi16(y_c128, _mm_cvtsi32_si128(frac_bits_y));
            mask128 = _mm_cmpgt_epi16(min_base_y128, base_y_c128);
            base_y_c128 = _mm_andnot_si128(mask128, base_y_c128);
            _mm_store_si128(base_y_c.0.as_mut_ptr() as *mut __m128i, base_y_c128);
            a0_y = _mm_setr_epi16(
                *left.offset(base_y_c.0[0] as isize) as i16,
                *left.offset(base_y_c.0[1] as isize) as i16,
                *left.offset(base_y_c.0[2] as isize) as i16,
                *left.offset(base_y_c.0[3] as isize) as i16,
                *left.offset(base_y_c.0[4] as isize) as i16,
                *left.offset(base_y_c.0[5] as isize) as i16,
                *left.offset(base_y_c.0[6] as isize) as i16,
                *left.offset(base_y_c.0[7] as isize) as i16,
            );
            base_y_c128 = _mm_add_epi16(
                base_y_c128,
                _mm_srli_epi16::<{ 4 }>(_mm256_castsi256_si128(a16)),
            );
            _mm_store_si128(base_y_c.0.as_mut_ptr() as *mut __m128i, base_y_c128);
            a1_y = _mm_setr_epi16(
                *left.offset(base_y_c.0[0] as isize) as i16,
                *left.offset(base_y_c.0[1] as isize) as i16,
                *left.offset(base_y_c.0[2] as isize) as i16,
                *left.offset(base_y_c.0[3] as isize) as i16,
                *left.offset(base_y_c.0[4] as isize) as i16,
                *left.offset(base_y_c.0[5] as isize) as i16,
                *left.offset(base_y_c.0[6] as isize) as i16,
                *left.offset(base_y_c.0[7] as isize) as i16,
            );
            if upsample_left != 0 {
                shifty = _mm_srli_epi16::<{ 1 }>(_mm_and_si128(
                    _mm_sll_epi16(y_c128, _mm_cvtsi32_si128(upsample_left)),
                    c3f,
                ));
            } else {
                shifty = _mm_srli_epi16::<{ 1 }>(_mm_and_si128(y_c128, c3f));
            }
            a0_x = _mm256_inserti128_si256::<1>(a0_x, a0_y);
            a1_x = _mm256_inserti128_si256::<1>(a1_x, a1_y);
            shift = _mm256_inserti128_si256::<1>(shift, shifty);
        }
        diff = _mm256_sub_epi16(a1_x, a0_x); // a[x+1] - a[x]
        a32 = _mm256_slli_epi16::<{ 5 }>(a0_x); // a[x] * 32
        a32 = _mm256_add_epi16(a32, a16); // a[x] * 32 + 16
        b = _mm256_mullo_epi16(diff, shift);
        res = _mm256_add_epi16(a32, b);
        res = _mm256_srli_epi16::<{ 5 }>(res);
        resx = _mm_packus_epi16(_mm256_castsi256_si128(res), _mm256_castsi256_si128(res));
        resy = _mm256_extracti128_si256::<1>(res);
        resy = _mm_packus_epi16(resy, resy);
        resxy = _mm_blendv_epi8(
            resx,
            resy,
            *(BaseMask.0[base_min_diff as usize].as_ptr() as *mut __m128i),
        );
        _mm_storel_epi64(dst as *mut __m128i, resxy);
        dst = dst.offset(stride);
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn dr_prediction_z2_HxW_avx2(
    H: i32,
    W: i32,
    mut dst: *mut u8,
    stride: isize,
    above: *const u8,
    left: *const u8,
    upsample_above: i32,
    upsample_left: i32,
    dx: i32,
    dy: i32,
) {
    // here upsample_above and upsample_left are 0 by design of
    // av1_use_intra_edge_upsample
    let min_base_x: i32 = -1;
    let min_base_y: i32 = -1;
    let _ = upsample_above;
    let _ = upsample_left;
    let frac_bits_x: i32 = 6;
    let frac_bits_y: i32 = 6;
    let mut a0_x: __m256i;
    let mut a1_x: __m256i;
    let mut a0_y: __m256i;
    let mut a1_y: __m256i;
    let mut a32: __m256i;
    let mut a16: __m256i;
    let mut c1234: __m256i;
    let mut c0123: __m256i;
    let mut diff: __m256i;
    let mut min_base_y256: __m256i;
    let mut c3f: __m256i;
    let mut shifty: __m256i;
    let mut dy256: __m256i;
    let mut c1: __m256i;
    let mut a0_x128: __m128i;
    let mut a1_x128: __m128i;
    let mut base_y_c: Aligned32<[i16; 16]> = core::mem::zeroed();
    a16 = _mm256_set1_epi16(16);
    c1 = _mm256_srli_epi16::<{ 4 }>(a16);
    min_base_y256 = _mm256_set1_epi16(min_base_y as i16);
    c3f = _mm256_set1_epi16(0x3f);
    dy256 = _mm256_set1_epi16(dy as i16);
    c0123 = _mm256_setr_epi16(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
    c1234 = _mm256_add_epi16(c0123, c1);
    for r in 0..H {
        let mut b: __m256i;
        let mut res: __m256i;
        let mut shift: __m256i;
        let mut j256: __m256i;
        let mut r6: __m256i;
        let mut ydx: __m256i;
        let mut resx: __m128i;
        let mut resy: __m128i;
        let mut resxy: __m128i;
        let mut y: i32 = r + 1;
        ydx = _mm256_set1_epi16((y * dx) as i16);
        let mut base_x: i32 = (-y * dx) >> frac_bits_x;
        for j in (0..W).step_by(16) {
            j256 = _mm256_set1_epi16(j as i16);
            let mut base_shift: i32 = 0;
            if base_x + j < min_base_x - 1 {
                base_shift = min_base_x - (base_x + j) - 1;
            }
            let mut base_min_diff: i32 = min_base_x - base_x - j;
            if base_min_diff > 16 {
                base_min_diff = 16;
            } else {
                if base_min_diff < 0 {
                    base_min_diff = 0;
                }
            }
            if base_shift < 16 {
                a0_x128 = _mm_loadu_si128(
                    above
                        .offset(base_x as isize)
                        .offset(base_shift as isize)
                        .offset(j as isize) as *mut __m128i as *const __m128i,
                );
                a1_x128 = _mm_loadu_si128(
                    above
                        .offset(base_x as isize)
                        .offset(base_shift as isize)
                        .offset(1)
                        .offset(j as isize) as *mut __m128i as *const __m128i,
                );
                a0_x128 = _mm_shuffle_epi8(
                    a0_x128,
                    *(LoadMaskx.0[base_shift as usize].as_ptr() as *mut __m128i),
                );
                a1_x128 = _mm_shuffle_epi8(
                    a1_x128,
                    *(LoadMaskx.0[base_shift as usize].as_ptr() as *mut __m128i),
                );
                a0_x = _mm256_cvtepu8_epi16(a0_x128);
                a1_x = _mm256_cvtepu8_epi16(a1_x128);
                r6 = _mm256_slli_epi16::<{ 6 }>(_mm256_add_epi16(c0123, j256));
                shift =
                    _mm256_srli_epi16::<{ 1 }>(_mm256_and_si256(_mm256_sub_epi16(r6, ydx), c3f));
                diff = _mm256_sub_epi16(a1_x, a0_x); // a[x+1] - a[x]
                a32 = _mm256_slli_epi16::<{ 5 }>(a0_x); // a[x] * 32
                a32 = _mm256_add_epi16(a32, a16); // a[x] * 32 + 16
                b = _mm256_mullo_epi16(diff, shift);
                res = _mm256_add_epi16(a32, b);
                res = _mm256_srli_epi16::<{ 5 }>(res); // 16 16-bit values
                resx = _mm256_castsi256_si128(_mm256_packus_epi16(
                    res,
                    _mm256_castsi128_si256(_mm256_extracti128_si256::<1>(res)),
                ));
            } else {
                resx = _mm_setzero_si128();
            }
            // y calc
            if base_x < min_base_x {
                let mut c256: __m256i;
                let mut y_c256: __m256i;
                let mut base_y_c256: __m256i;
                let mut mask256: __m256i;
                let mut mul16: __m256i;
                r6 = _mm256_set1_epi16((r << 6) as i16);
                c256 = _mm256_add_epi16(j256, c1234);
                mul16 = _mm256_min_epu16(
                    _mm256_mullo_epi16(c256, dy256),
                    _mm256_srli_epi16::<{ 1 }>(min_base_y256),
                );
                y_c256 = _mm256_sub_epi16(r6, mul16);
                base_y_c256 = _mm256_sra_epi16(y_c256, _mm_cvtsi32_si128(frac_bits_y));
                mask256 = _mm256_cmpgt_epi16(min_base_y256, base_y_c256);
                base_y_c256 = _mm256_blendv_epi8(base_y_c256, min_base_y256, mask256);
                let mut min_y: i16 =
                    _mm_extract_epi16::<7>(_mm256_extracti128_si256::<1>(base_y_c256)) as i16;
                let mut max_y: i16 =
                    _mm_extract_epi16::<0>(_mm256_castsi256_si128(base_y_c256)) as i16;
                let mut offset_diff: i16 = (max_y as i32 - min_y as i32) as i16;
                if (offset_diff as i32) < 16 {
                    let mut min_y256: __m256i = _mm256_set1_epi16(min_y);
                    let mut base_y_offset: __m256i = _mm256_sub_epi16(base_y_c256, min_y256);
                    let mut base_y_offset128: __m128i = _mm_packs_epi16(
                        _mm256_extracti128_si256::<0>(base_y_offset),
                        _mm256_extracti128_si256::<1>(base_y_offset),
                    );
                    let mut a0_y128: __m128i = _mm_maskload_epi32(
                        left.offset(min_y as isize) as *mut i32 as *const i32,
                        *(LoadMaskz2.0[(offset_diff as i32 / 4) as usize].as_ptr() as *mut __m128i),
                    );
                    let mut a1_y128: __m128i = _mm_maskload_epi32(
                        left.offset(min_y as isize).offset(1) as *mut i32 as *const i32,
                        *(LoadMaskz2.0[(offset_diff as i32 / 4) as usize].as_ptr() as *mut __m128i),
                    );
                    a0_y128 = _mm_shuffle_epi8(a0_y128, base_y_offset128);
                    a1_y128 = _mm_shuffle_epi8(a1_y128, base_y_offset128);
                    a0_y = _mm256_cvtepu8_epi16(a0_y128);
                    a1_y = _mm256_cvtepu8_epi16(a1_y128);
                } else {
                    base_y_c256 = _mm256_andnot_si256(mask256, base_y_c256);
                    _mm256_store_si256(base_y_c.0.as_mut_ptr() as *mut __m256i, base_y_c256);
                    a0_y = _mm256_setr_epi16(
                        *left.offset(base_y_c.0[0] as isize) as i16,
                        *left.offset(base_y_c.0[1] as isize) as i16,
                        *left.offset(base_y_c.0[2] as isize) as i16,
                        *left.offset(base_y_c.0[3] as isize) as i16,
                        *left.offset(base_y_c.0[4] as isize) as i16,
                        *left.offset(base_y_c.0[5] as isize) as i16,
                        *left.offset(base_y_c.0[6] as isize) as i16,
                        *left.offset(base_y_c.0[7] as isize) as i16,
                        *left.offset(base_y_c.0[8] as isize) as i16,
                        *left.offset(base_y_c.0[9] as isize) as i16,
                        *left.offset(base_y_c.0[10] as isize) as i16,
                        *left.offset(base_y_c.0[11] as isize) as i16,
                        *left.offset(base_y_c.0[12] as isize) as i16,
                        *left.offset(base_y_c.0[13] as isize) as i16,
                        *left.offset(base_y_c.0[14] as isize) as i16,
                        *left.offset(base_y_c.0[15] as isize) as i16,
                    );
                    base_y_c256 = _mm256_add_epi16(base_y_c256, c1);
                    _mm256_store_si256(base_y_c.0.as_mut_ptr() as *mut __m256i, base_y_c256);
                    a1_y = _mm256_setr_epi16(
                        *left.offset(base_y_c.0[0] as isize) as i16,
                        *left.offset(base_y_c.0[1] as isize) as i16,
                        *left.offset(base_y_c.0[2] as isize) as i16,
                        *left.offset(base_y_c.0[3] as isize) as i16,
                        *left.offset(base_y_c.0[4] as isize) as i16,
                        *left.offset(base_y_c.0[5] as isize) as i16,
                        *left.offset(base_y_c.0[6] as isize) as i16,
                        *left.offset(base_y_c.0[7] as isize) as i16,
                        *left.offset(base_y_c.0[8] as isize) as i16,
                        *left.offset(base_y_c.0[9] as isize) as i16,
                        *left.offset(base_y_c.0[10] as isize) as i16,
                        *left.offset(base_y_c.0[11] as isize) as i16,
                        *left.offset(base_y_c.0[12] as isize) as i16,
                        *left.offset(base_y_c.0[13] as isize) as i16,
                        *left.offset(base_y_c.0[14] as isize) as i16,
                        *left.offset(base_y_c.0[15] as isize) as i16,
                    );
                }
                shifty = _mm256_srli_epi16::<{ 1 }>(_mm256_and_si256(y_c256, c3f));
                diff = _mm256_sub_epi16(a1_y, a0_y); // a[x+1] - a[x]
                a32 = _mm256_slli_epi16::<{ 5 }>(a0_y); // a[x] * 32
                a32 = _mm256_add_epi16(a32, a16); // a[x] * 32 + 16
                b = _mm256_mullo_epi16(diff, shifty);
                res = _mm256_add_epi16(a32, b);
                res = _mm256_srli_epi16::<{ 5 }>(res); // 16 16-bit values
                resy = _mm256_castsi256_si128(_mm256_packus_epi16(
                    res,
                    _mm256_castsi128_si256(_mm256_extracti128_si256::<1>(res)),
                ));
            } else {
                resy = _mm_setzero_si128();
            }
            resxy = _mm_blendv_epi8(
                resx,
                resy,
                *(BaseMask.0[base_min_diff as usize].as_ptr() as *mut __m128i),
            );
            _mm_storeu_si128(dst.offset(j as isize) as *mut __m128i, resxy);
        } // for j
        dst = dst.offset(stride);
    }
}

// Directional prediction, zone 2: 90 < angle < 180
#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn av1_dr_prediction_z2_avx2(
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
        4 => {
            dr_prediction_z2_Nx4_avx2(
                bh,
                dst,
                stride,
                above,
                left,
                upsample_above,
                upsample_left,
                dx,
                dy,
            );
        }
        8 => {
            dr_prediction_z2_Nx8_avx2(
                bh,
                dst,
                stride,
                above,
                left,
                upsample_above,
                upsample_left,
                dx,
                dy,
            );
        }
        _ => {
            dr_prediction_z2_HxW_avx2(
                bh,
                bw,
                dst,
                stride,
                above,
                left,
                upsample_above,
                upsample_left,
                dx,
                dy,
            );
        }
    }
    return;
}

// z3 functions
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn transpose16x32_avx2(x: *mut __m256i, d: *mut __m256i) {
    let mut w0: __m256i;
    let mut w1: __m256i;
    let mut w2: __m256i;
    let mut w3: __m256i;
    let mut w4: __m256i;
    let mut w5: __m256i;
    let mut w6: __m256i;
    let mut w7: __m256i;
    let mut w8: __m256i;
    let mut w9: __m256i;
    let mut w10: __m256i;
    let mut w11: __m256i;
    let mut w12: __m256i;
    let mut w13: __m256i;
    let mut w14: __m256i;
    let mut w15: __m256i;
    w0 = _mm256_unpacklo_epi8(*x, *x.offset(1));
    w1 = _mm256_unpacklo_epi8(*x.offset(2), *x.offset(3));
    w2 = _mm256_unpacklo_epi8(*x.offset(4), *x.offset(5));
    w3 = _mm256_unpacklo_epi8(*x.offset(6), *x.offset(7));
    w8 = _mm256_unpacklo_epi8(*x.offset(8), *x.offset(9));
    w9 = _mm256_unpacklo_epi8(*x.offset(10), *x.offset(11));
    w10 = _mm256_unpacklo_epi8(*x.offset(12), *x.offset(13));
    w11 = _mm256_unpacklo_epi8(*x.offset(14), *x.offset(15));
    w4 = _mm256_unpacklo_epi16(w0, w1);
    w5 = _mm256_unpacklo_epi16(w2, w3);
    w12 = _mm256_unpacklo_epi16(w8, w9);
    w13 = _mm256_unpacklo_epi16(w10, w11);
    w6 = _mm256_unpacklo_epi32(w4, w5);
    w7 = _mm256_unpackhi_epi32(w4, w5);
    w14 = _mm256_unpacklo_epi32(w12, w13);
    w15 = _mm256_unpackhi_epi32(w12, w13);
    // Store first 4-line result
    *d = _mm256_unpacklo_epi64(w6, w14);
    *d.offset(1) = _mm256_unpackhi_epi64(w6, w14);
    *d.offset(2) = _mm256_unpacklo_epi64(w7, w15);
    *d.offset(3) = _mm256_unpackhi_epi64(w7, w15);
    w4 = _mm256_unpackhi_epi16(w0, w1);
    w5 = _mm256_unpackhi_epi16(w2, w3);
    w12 = _mm256_unpackhi_epi16(w8, w9);
    w13 = _mm256_unpackhi_epi16(w10, w11);
    w6 = _mm256_unpacklo_epi32(w4, w5);
    w7 = _mm256_unpackhi_epi32(w4, w5);
    w14 = _mm256_unpacklo_epi32(w12, w13);
    w15 = _mm256_unpackhi_epi32(w12, w13);
    // Store second 4-line result
    *d.offset(4) = _mm256_unpacklo_epi64(w6, w14);
    *d.offset(5) = _mm256_unpackhi_epi64(w6, w14);
    *d.offset(6) = _mm256_unpacklo_epi64(w7, w15);
    *d.offset(7) = _mm256_unpackhi_epi64(w7, w15);
    // upper half
    w0 = _mm256_unpackhi_epi8(*x, *x.offset(1));
    w1 = _mm256_unpackhi_epi8(*x.offset(2), *x.offset(3));
    w2 = _mm256_unpackhi_epi8(*x.offset(4), *x.offset(5));
    w3 = _mm256_unpackhi_epi8(*x.offset(6), *x.offset(7));
    w8 = _mm256_unpackhi_epi8(*x.offset(8), *x.offset(9));
    w9 = _mm256_unpackhi_epi8(*x.offset(10), *x.offset(11));
    w10 = _mm256_unpackhi_epi8(*x.offset(12), *x.offset(13));
    w11 = _mm256_unpackhi_epi8(*x.offset(14), *x.offset(15));
    w4 = _mm256_unpacklo_epi16(w0, w1);
    w5 = _mm256_unpacklo_epi16(w2, w3);
    w12 = _mm256_unpacklo_epi16(w8, w9);
    w13 = _mm256_unpacklo_epi16(w10, w11);
    w6 = _mm256_unpacklo_epi32(w4, w5);
    w7 = _mm256_unpackhi_epi32(w4, w5);
    w14 = _mm256_unpacklo_epi32(w12, w13);
    w15 = _mm256_unpackhi_epi32(w12, w13);
    // Store first 4-line result
    *d.offset(8) = _mm256_unpacklo_epi64(w6, w14);
    *d.offset(9) = _mm256_unpackhi_epi64(w6, w14);
    *d.offset(10) = _mm256_unpacklo_epi64(w7, w15);
    *d.offset(11) = _mm256_unpackhi_epi64(w7, w15);
    w4 = _mm256_unpackhi_epi16(w0, w1);
    w5 = _mm256_unpackhi_epi16(w2, w3);
    w12 = _mm256_unpackhi_epi16(w8, w9);
    w13 = _mm256_unpackhi_epi16(w10, w11);
    w6 = _mm256_unpacklo_epi32(w4, w5);
    w7 = _mm256_unpackhi_epi32(w4, w5);
    w14 = _mm256_unpacklo_epi32(w12, w13);
    w15 = _mm256_unpackhi_epi32(w12, w13);
    // Store second 4-line result
    *d.offset(12) = _mm256_unpacklo_epi64(w6, w14);
    *d.offset(13) = _mm256_unpackhi_epi64(w6, w14);
    *d.offset(14) = _mm256_unpacklo_epi64(w7, w15);
    *d.offset(15) = _mm256_unpackhi_epi64(w7, w15);
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn dr_prediction_z3_4x4_avx2(
    dst: *mut u8,
    stride: isize,
    left: *const u8,
    upsample_left: i32,
    dy: i32,
) {
    let mut dstvec: [__m128i; 4] = core::mem::zeroed();
    let mut d: [__m128i; 4] = core::mem::zeroed();
    dr_prediction_z1_HxW_internal_avx2(4, 4, dstvec.as_mut_ptr(), left, upsample_left, dy);
    transpose4x8_8x4_low_sse2(
        dstvec.as_mut_ptr(),
        dstvec.as_mut_ptr().offset(1),
        dstvec.as_mut_ptr().offset(2),
        dstvec.as_mut_ptr().offset(3),
        d.as_mut_ptr(),
        d.as_mut_ptr().offset(1),
        d.as_mut_ptr().offset(2),
        d.as_mut_ptr().offset(3),
    );
    core::ptr::write_unaligned(dst.offset(stride * 0) as *mut i32, _mm_cvtsi128_si32(d[0]));
    core::ptr::write_unaligned(dst.offset(stride * 1) as *mut i32, _mm_cvtsi128_si32(d[1]));
    core::ptr::write_unaligned(dst.offset(stride * 2) as *mut i32, _mm_cvtsi128_si32(d[2]));
    core::ptr::write_unaligned(dst.offset(stride * 3) as *mut i32, _mm_cvtsi128_si32(d[3]));
    return;
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn dr_prediction_z3_8x8_avx2(
    dst: *mut u8,
    stride: isize,
    left: *const u8,
    upsample_left: i32,
    dy: i32,
) {
    let mut dstvec: [__m128i; 8] = core::mem::zeroed();
    let mut d: [__m128i; 8] = core::mem::zeroed();
    dr_prediction_z1_HxW_internal_avx2(8, 8, dstvec.as_mut_ptr(), left, upsample_left, dy);
    transpose8x8_sse2(
        dstvec.as_mut_ptr(),
        dstvec.as_mut_ptr().offset(1),
        dstvec.as_mut_ptr().offset(2),
        dstvec.as_mut_ptr().offset(3),
        dstvec.as_mut_ptr().offset(4),
        dstvec.as_mut_ptr().offset(5),
        dstvec.as_mut_ptr().offset(6),
        dstvec.as_mut_ptr().offset(7),
        d.as_mut_ptr(),
        d.as_mut_ptr().offset(1),
        d.as_mut_ptr().offset(2),
        d.as_mut_ptr().offset(3),
    );
    _mm_storel_epi64(dst.offset(0 * stride) as *mut __m128i, d[0]);
    _mm_storel_epi64(
        dst.offset(1 * stride) as *mut __m128i,
        _mm_srli_si128::<8>(d[0]),
    );
    _mm_storel_epi64(dst.offset(2 * stride) as *mut __m128i, d[1]);
    _mm_storel_epi64(
        dst.offset(3 * stride) as *mut __m128i,
        _mm_srli_si128::<8>(d[1]),
    );
    _mm_storel_epi64(dst.offset(4 * stride) as *mut __m128i, d[2]);
    _mm_storel_epi64(
        dst.offset(5 * stride) as *mut __m128i,
        _mm_srli_si128::<8>(d[2]),
    );
    _mm_storel_epi64(dst.offset(6 * stride) as *mut __m128i, d[3]);
    _mm_storel_epi64(
        dst.offset(7 * stride) as *mut __m128i,
        _mm_srli_si128::<8>(d[3]),
    );
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn dr_prediction_z3_4x8_avx2(
    dst: *mut u8,
    stride: isize,
    left: *const u8,
    upsample_left: i32,
    dy: i32,
) {
    let mut dstvec: [__m128i; 4] = core::mem::zeroed();
    let mut d: [__m128i; 8] = core::mem::zeroed();
    dr_prediction_z1_HxW_internal_avx2(8, 4, dstvec.as_mut_ptr(), left, upsample_left, dy);
    transpose4x8_8x4_sse2(
        dstvec.as_mut_ptr(),
        dstvec.as_mut_ptr().offset(1),
        dstvec.as_mut_ptr().offset(2),
        dstvec.as_mut_ptr().offset(3),
        d.as_mut_ptr(),
        d.as_mut_ptr().offset(1),
        d.as_mut_ptr().offset(2),
        d.as_mut_ptr().offset(3),
        d.as_mut_ptr().offset(4),
        d.as_mut_ptr().offset(5),
        d.as_mut_ptr().offset(6),
        d.as_mut_ptr().offset(7),
    );
    for i in 0..8 {
        core::ptr::write_unaligned(
            dst.offset(stride * i as isize) as *mut i32,
            _mm_cvtsi128_si32(d[i as usize]),
        );
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn dr_prediction_z3_8x4_avx2(
    dst: *mut u8,
    stride: isize,
    left: *const u8,
    upsample_left: i32,
    dy: i32,
) {
    let mut dstvec: [__m128i; 8] = core::mem::zeroed();
    let mut d: [__m128i; 4] = core::mem::zeroed();
    dr_prediction_z1_HxW_internal_avx2(4, 8, dstvec.as_mut_ptr(), left, upsample_left, dy);
    transpose8x8_low_sse2(
        dstvec.as_mut_ptr(),
        dstvec.as_mut_ptr().offset(1),
        dstvec.as_mut_ptr().offset(2),
        dstvec.as_mut_ptr().offset(3),
        dstvec.as_mut_ptr().offset(4),
        dstvec.as_mut_ptr().offset(5),
        dstvec.as_mut_ptr().offset(6),
        dstvec.as_mut_ptr().offset(7),
        d.as_mut_ptr(),
        d.as_mut_ptr().offset(1),
        d.as_mut_ptr().offset(2),
        d.as_mut_ptr().offset(3),
    );
    _mm_storel_epi64(dst.offset(0 * stride) as *mut __m128i, d[0]);
    _mm_storel_epi64(dst.offset(1 * stride) as *mut __m128i, d[1]);
    _mm_storel_epi64(dst.offset(2 * stride) as *mut __m128i, d[2]);
    _mm_storel_epi64(dst.offset(3 * stride) as *mut __m128i, d[3]);
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn dr_prediction_z3_8x16_avx2(
    dst: *mut u8,
    stride: isize,
    left: *const u8,
    upsample_left: i32,
    dy: i32,
) {
    let mut dstvec: [__m128i; 8] = core::mem::zeroed();
    let mut d: [__m128i; 8] = core::mem::zeroed();
    dr_prediction_z1_HxW_internal_avx2(16, 8, dstvec.as_mut_ptr(), left, upsample_left, dy);
    transpose8x16_16x8_sse2(
        dstvec.as_mut_ptr(),
        dstvec.as_mut_ptr().offset(1),
        dstvec.as_mut_ptr().offset(2),
        dstvec.as_mut_ptr().offset(3),
        dstvec.as_mut_ptr().offset(4),
        dstvec.as_mut_ptr().offset(5),
        dstvec.as_mut_ptr().offset(6),
        dstvec.as_mut_ptr().offset(7),
        d.as_mut_ptr(),
        d.as_mut_ptr().offset(1),
        d.as_mut_ptr().offset(2),
        d.as_mut_ptr().offset(3),
        d.as_mut_ptr().offset(4),
        d.as_mut_ptr().offset(5),
        d.as_mut_ptr().offset(6),
        d.as_mut_ptr().offset(7),
    );
    for i in 0..8 {
        _mm_storel_epi64(
            dst.offset(i as isize * stride) as *mut __m128i,
            d[i as usize],
        );
        _mm_storel_epi64(
            dst.offset((i + 8) as isize * stride) as *mut __m128i,
            _mm_srli_si128::<8>(d[i as usize]),
        );
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn dr_prediction_z3_16x8_avx2(
    dst: *mut u8,
    stride: isize,
    left: *const u8,
    upsample_left: i32,
    dy: i32,
) {
    let mut dstvec: [__m128i; 16] = core::mem::zeroed();
    let mut d: [__m128i; 16] = core::mem::zeroed();
    dr_prediction_z1_HxW_internal_avx2(8, 16, dstvec.as_mut_ptr(), left, upsample_left, dy);
    transpose16x8_8x16_sse2(
        dstvec.as_mut_ptr(),
        dstvec.as_mut_ptr().offset(1),
        dstvec.as_mut_ptr().offset(2),
        dstvec.as_mut_ptr().offset(3),
        dstvec.as_mut_ptr().offset(4),
        dstvec.as_mut_ptr().offset(5),
        dstvec.as_mut_ptr().offset(6),
        dstvec.as_mut_ptr().offset(7),
        dstvec.as_mut_ptr().offset(8),
        dstvec.as_mut_ptr().offset(9),
        dstvec.as_mut_ptr().offset(10),
        dstvec.as_mut_ptr().offset(11),
        dstvec.as_mut_ptr().offset(12),
        dstvec.as_mut_ptr().offset(13),
        dstvec.as_mut_ptr().offset(14),
        dstvec.as_mut_ptr().offset(15),
        d.as_mut_ptr(),
        d.as_mut_ptr().offset(1),
        d.as_mut_ptr().offset(2),
        d.as_mut_ptr().offset(3),
        d.as_mut_ptr().offset(4),
        d.as_mut_ptr().offset(5),
        d.as_mut_ptr().offset(6),
        d.as_mut_ptr().offset(7),
    );
    for i in 0..8 {
        _mm_storeu_si128(
            dst.offset(i as isize * stride) as *mut __m128i,
            d[i as usize],
        );
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn dr_prediction_z3_4x16_avx2(
    dst: *mut u8,
    stride: isize,
    left: *const u8,
    upsample_left: i32,
    dy: i32,
) {
    let mut dstvec: [__m128i; 4] = core::mem::zeroed();
    let mut d: [__m128i; 16] = core::mem::zeroed();
    dr_prediction_z1_HxW_internal_avx2(16, 4, dstvec.as_mut_ptr(), left, upsample_left, dy);
    transpose4x16_sse2(dstvec.as_mut_ptr(), d.as_mut_ptr());
    for i in 0..16 {
        core::ptr::write_unaligned(
            dst.offset(stride * i as isize) as *mut i32,
            _mm_cvtsi128_si32(d[i as usize]),
        );
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn dr_prediction_z3_16x4_avx2(
    dst: *mut u8,
    stride: isize,
    left: *const u8,
    upsample_left: i32,
    dy: i32,
) {
    let mut dstvec: [__m128i; 16] = core::mem::zeroed();
    let mut d: [__m128i; 8] = core::mem::zeroed();
    dr_prediction_z1_HxW_internal_avx2(4, 16, dstvec.as_mut_ptr(), left, upsample_left, dy);
    for i in 4..8 {
        d[i as usize] = _mm_setzero_si128();
    }
    transpose16x8_8x16_sse2(
        dstvec.as_mut_ptr(),
        dstvec.as_mut_ptr().offset(1),
        dstvec.as_mut_ptr().offset(2),
        dstvec.as_mut_ptr().offset(3),
        dstvec.as_mut_ptr().offset(4),
        dstvec.as_mut_ptr().offset(5),
        dstvec.as_mut_ptr().offset(6),
        dstvec.as_mut_ptr().offset(7),
        dstvec.as_mut_ptr().offset(8),
        dstvec.as_mut_ptr().offset(9),
        dstvec.as_mut_ptr().offset(10),
        dstvec.as_mut_ptr().offset(11),
        dstvec.as_mut_ptr().offset(12),
        dstvec.as_mut_ptr().offset(13),
        dstvec.as_mut_ptr().offset(14),
        dstvec.as_mut_ptr().offset(15),
        d.as_mut_ptr(),
        d.as_mut_ptr().offset(1),
        d.as_mut_ptr().offset(2),
        d.as_mut_ptr().offset(3),
        d.as_mut_ptr().offset(4),
        d.as_mut_ptr().offset(5),
        d.as_mut_ptr().offset(6),
        d.as_mut_ptr().offset(7),
    );
    for i in 0..4 {
        _mm_storeu_si128(
            dst.offset(i as isize * stride) as *mut __m128i,
            d[i as usize],
        );
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn dr_prediction_z3_8x32_avx2(
    dst: *mut u8,
    stride: isize,
    left: *const u8,
    upsample_left: i32,
    dy: i32,
) {
    let mut dstvec: [__m256i; 16] = core::mem::zeroed();
    let mut d: [__m256i; 16] = core::mem::zeroed();
    dr_prediction_z1_32xN_internal_avx2(8, dstvec.as_mut_ptr(), left, upsample_left, dy);
    for i in 8..16 {
        dstvec[i as usize] = _mm256_setzero_si256();
    }
    transpose16x32_avx2(dstvec.as_mut_ptr(), d.as_mut_ptr());
    for i in 0..16 {
        _mm_storel_epi64(
            dst.offset(i as isize * stride) as *mut __m128i,
            _mm256_castsi256_si128(d[i as usize]),
        );
    }
    for i in 0..16 {
        _mm_storel_epi64(
            dst.offset((i + 16) as isize * stride) as *mut __m128i,
            _mm256_extracti128_si256::<1>(d[i as usize]),
        );
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn dr_prediction_z3_32x8_avx2(
    dst: *mut u8,
    stride: isize,
    left: *const u8,
    upsample_left: i32,
    dy: i32,
) {
    let mut dstvec: [__m128i; 32] = core::mem::zeroed();
    let mut d: [__m128i; 16] = core::mem::zeroed();
    dr_prediction_z1_HxW_internal_avx2(8, 32, dstvec.as_mut_ptr(), left, upsample_left, dy);
    transpose16x8_8x16_sse2(
        dstvec.as_mut_ptr(),
        dstvec.as_mut_ptr().offset(1),
        dstvec.as_mut_ptr().offset(2),
        dstvec.as_mut_ptr().offset(3),
        dstvec.as_mut_ptr().offset(4),
        dstvec.as_mut_ptr().offset(5),
        dstvec.as_mut_ptr().offset(6),
        dstvec.as_mut_ptr().offset(7),
        dstvec.as_mut_ptr().offset(8),
        dstvec.as_mut_ptr().offset(9),
        dstvec.as_mut_ptr().offset(10),
        dstvec.as_mut_ptr().offset(11),
        dstvec.as_mut_ptr().offset(12),
        dstvec.as_mut_ptr().offset(13),
        dstvec.as_mut_ptr().offset(14),
        dstvec.as_mut_ptr().offset(15),
        d.as_mut_ptr(),
        d.as_mut_ptr().offset(1),
        d.as_mut_ptr().offset(2),
        d.as_mut_ptr().offset(3),
        d.as_mut_ptr().offset(4),
        d.as_mut_ptr().offset(5),
        d.as_mut_ptr().offset(6),
        d.as_mut_ptr().offset(7),
    );
    transpose16x8_8x16_sse2(
        dstvec.as_mut_ptr().offset((0 + 16) as isize),
        dstvec.as_mut_ptr().offset((1 + 16) as isize),
        dstvec.as_mut_ptr().offset((2 + 16) as isize),
        dstvec.as_mut_ptr().offset((3 + 16) as isize),
        dstvec.as_mut_ptr().offset((4 + 16) as isize),
        dstvec.as_mut_ptr().offset((5 + 16) as isize),
        dstvec.as_mut_ptr().offset((6 + 16) as isize),
        dstvec.as_mut_ptr().offset((7 + 16) as isize),
        dstvec.as_mut_ptr().offset((8 + 16) as isize),
        dstvec.as_mut_ptr().offset((9 + 16) as isize),
        dstvec.as_mut_ptr().offset((10 + 16) as isize),
        dstvec.as_mut_ptr().offset((11 + 16) as isize),
        dstvec.as_mut_ptr().offset((12 + 16) as isize),
        dstvec.as_mut_ptr().offset((13 + 16) as isize),
        dstvec.as_mut_ptr().offset((14 + 16) as isize),
        dstvec.as_mut_ptr().offset((15 + 16) as isize),
        d.as_mut_ptr().offset((0 + 8) as isize),
        d.as_mut_ptr().offset((1 + 8) as isize),
        d.as_mut_ptr().offset((2 + 8) as isize),
        d.as_mut_ptr().offset((3 + 8) as isize),
        d.as_mut_ptr().offset((4 + 8) as isize),
        d.as_mut_ptr().offset((5 + 8) as isize),
        d.as_mut_ptr().offset((6 + 8) as isize),
        d.as_mut_ptr().offset((7 + 8) as isize),
    );
    for i in 0..8 {
        _mm_storeu_si128(
            dst.offset(i as isize * stride) as *mut __m128i,
            d[i as usize],
        );
        _mm_storeu_si128(
            dst.offset(i as isize * stride).offset(16) as *mut __m128i,
            d[(i + 8) as usize],
        );
    }
}

// !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn dr_prediction_z3_16x16_avx2(
    dst: *mut u8,
    stride: isize,
    left: *const u8,
    upsample_left: i32,
    dy: i32,
) {
    let mut dstvec: [__m128i; 16] = core::mem::zeroed();
    let mut d: [__m128i; 16] = core::mem::zeroed();
    dr_prediction_z1_HxW_internal_avx2(16, 16, dstvec.as_mut_ptr(), left, upsample_left, dy);
    transpose16x16_sse2(dstvec.as_mut_ptr(), d.as_mut_ptr());
    for i in 0..16 {
        _mm_storeu_si128(
            dst.offset(i as isize * stride) as *mut __m128i,
            d[i as usize],
        );
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn dr_prediction_z3_32x32_avx2(
    dst: *mut u8,
    stride: isize,
    left: *const u8,
    upsample_left: i32,
    dy: i32,
) {
    let mut dstvec: [__m256i; 32] = core::mem::zeroed();
    let mut d: [__m256i; 32] = core::mem::zeroed();
    dr_prediction_z1_32xN_internal_avx2(32, dstvec.as_mut_ptr(), left, upsample_left, dy);
    transpose16x32_avx2(dstvec.as_mut_ptr(), d.as_mut_ptr());
    transpose16x32_avx2(dstvec.as_mut_ptr().offset(16), d.as_mut_ptr().offset(16));
    for j in 0..16 {
        _mm_storeu_si128(
            dst.offset(j as isize * stride) as *mut __m128i,
            _mm256_castsi256_si128(d[j as usize]),
        );
        _mm_storeu_si128(
            dst.offset(j as isize * stride).offset(16) as *mut __m128i,
            _mm256_castsi256_si128(d[(j + 16) as usize]),
        );
    }
    for j in 0..16 {
        _mm_storeu_si128(
            dst.offset((j + 16) as isize * stride) as *mut __m128i,
            _mm256_extracti128_si256::<1>(d[j as usize]),
        );
        _mm_storeu_si128(
            dst.offset((j + 16) as isize * stride).offset(16) as *mut __m128i,
            _mm256_extracti128_si256::<1>(d[(j + 16) as usize]),
        );
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn dr_prediction_z3_64x64_avx2(
    dst: *mut u8,
    stride: isize,
    left: *const u8,
    upsample_left: i32,
    dy: i32,
) {
    let mut dstT: Aligned16<[u8; (64 * 64) as usize]> = core::mem::zeroed();
    dr_prediction_z1_64xN_avx2(64, dstT.0.as_mut_ptr(), 64, left, upsample_left, dy);
    transpose(dstT.0.as_mut_ptr() as *const u8, 64, dst, stride, 64, 64);
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn dr_prediction_z3_16x32_avx2(
    dst: *mut u8,
    stride: isize,
    left: *const u8,
    upsample_left: i32,
    dy: i32,
) {
    let mut dstvec: [__m256i; 16] = core::mem::zeroed();
    let mut d: [__m256i; 16] = core::mem::zeroed();
    dr_prediction_z1_32xN_internal_avx2(16, dstvec.as_mut_ptr(), left, upsample_left, dy);
    transpose16x32_avx2(dstvec.as_mut_ptr(), d.as_mut_ptr());
    // store
    for j in 0..16 {
        _mm_storeu_si128(
            dst.offset(j as isize * stride) as *mut __m128i,
            _mm256_castsi256_si128(d[j as usize]),
        );
        _mm_storeu_si128(
            dst.offset((j + 16) as isize * stride) as *mut __m128i,
            _mm256_extracti128_si256::<1>(d[j as usize]),
        );
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn dr_prediction_z3_32x16_avx2(
    dst: *mut u8,
    stride: isize,
    left: *const u8,
    upsample_left: i32,
    dy: i32,
) {
    let mut dstvec: [__m128i; 32] = core::mem::zeroed();
    let mut d: [__m128i; 16] = core::mem::zeroed();
    dr_prediction_z1_HxW_internal_avx2(16, 32, dstvec.as_mut_ptr(), left, upsample_left, dy);
    for i in (0..32).step_by(16) {
        transpose16x16_sse2(dstvec.as_mut_ptr().offset(i as isize), d.as_mut_ptr());
        for j in 0..16 {
            _mm_storeu_si128(
                dst.offset(j as isize * stride).offset(i as isize) as *mut __m128i,
                d[j as usize],
            );
        }
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn dr_prediction_z3_32x64_avx2(
    dst: *mut u8,
    stride: isize,
    left: *const u8,
    upsample_left: i32,
    dy: i32,
) {
    let mut dstT: Aligned16<[u8; (64 * 32) as usize]> = core::mem::zeroed();
    dr_prediction_z1_64xN_avx2(32, dstT.0.as_mut_ptr(), 64, left, upsample_left, dy);
    transpose(dstT.0.as_mut_ptr() as *const u8, 64, dst, stride, 32, 64);
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn dr_prediction_z3_64x32_avx2(
    dst: *mut u8,
    stride: isize,
    left: *const u8,
    upsample_left: i32,
    dy: i32,
) {
    let mut dstT: Aligned16<[u8; (32 * 64) as usize]> = core::mem::zeroed();
    dr_prediction_z1_32xN_avx2(64, dstT.0.as_mut_ptr(), 32, left, upsample_left, dy);
    transpose(dstT.0.as_mut_ptr() as *const u8, 32, dst, stride, 64, 32);
    return;
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn dr_prediction_z3_16x64_avx2(
    dst: *mut u8,
    stride: isize,
    left: *const u8,
    upsample_left: i32,
    dy: i32,
) {
    let mut dstT: Aligned16<[u8; (64 * 16) as usize]> = core::mem::zeroed();
    dr_prediction_z1_64xN_avx2(16, dstT.0.as_mut_ptr(), 64, left, upsample_left, dy);
    transpose(dstT.0.as_mut_ptr() as *const u8, 64, dst, stride, 16, 64);
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn dr_prediction_z3_64x16_avx2(
    dst: *mut u8,
    stride: isize,
    left: *const u8,
    upsample_left: i32,
    dy: i32,
) {
    let mut dstvec: [__m128i; 64] = core::mem::zeroed();
    let mut d: [__m128i; 16] = core::mem::zeroed();
    dr_prediction_z1_HxW_internal_avx2(16, 64, dstvec.as_mut_ptr(), left, upsample_left, dy);
    for i in (0..64).step_by(16) {
        transpose16x16_sse2(dstvec.as_mut_ptr().offset(i as isize), d.as_mut_ptr());
        for j in 0..16 {
            _mm_storeu_si128(
                dst.offset(j as isize * stride).offset(i as isize) as *mut __m128i,
                d[j as usize],
            );
        }
    }
}

// !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER
#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn av1_dr_prediction_z3_avx2(
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
    let _ = above;
    let _ = dx;
    debug_assert!(dx == 1);
    debug_assert!(dy > 0);
    if bw == bh {
        match bw {
            4 => {
                dr_prediction_z3_4x4_avx2(dst, stride, left, upsample_left, dy);
            }
            8 => {
                dr_prediction_z3_8x8_avx2(dst, stride, left, upsample_left, dy);
            }
            16 => {
                dr_prediction_z3_16x16_avx2(dst, stride, left, upsample_left, dy);
            }
            32 => {
                dr_prediction_z3_32x32_avx2(dst, stride, left, upsample_left, dy);
            }
            64 => {
                dr_prediction_z3_64x64_avx2(dst, stride, left, upsample_left, dy);
            }
            _ => {}
        }
    } else {
        if bw < bh {
            if bw + bw == bh {
                match bw {
                    4 => {
                        dr_prediction_z3_4x8_avx2(dst, stride, left, upsample_left, dy);
                    }
                    8 => {
                        dr_prediction_z3_8x16_avx2(dst, stride, left, upsample_left, dy);
                    }
                    16 => {
                        dr_prediction_z3_16x32_avx2(dst, stride, left, upsample_left, dy);
                    }
                    32 => {
                        dr_prediction_z3_32x64_avx2(dst, stride, left, upsample_left, dy);
                    }
                    _ => {}
                }
            } else {
                match bw {
                    4 => {
                        dr_prediction_z3_4x16_avx2(dst, stride, left, upsample_left, dy);
                    }
                    8 => {
                        dr_prediction_z3_8x32_avx2(dst, stride, left, upsample_left, dy);
                    }
                    16 => {
                        dr_prediction_z3_16x64_avx2(dst, stride, left, upsample_left, dy);
                    }
                    _ => {}
                }
            }
        } else {
            // !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER
            if bh + bh == bw {
                match bh {
                    4 => {
                        dr_prediction_z3_8x4_avx2(dst, stride, left, upsample_left, dy);
                    }
                    8 => {
                        dr_prediction_z3_16x8_avx2(dst, stride, left, upsample_left, dy);
                    }
                    16 => {
                        dr_prediction_z3_32x16_avx2(dst, stride, left, upsample_left, dy);
                    }
                    32 => {
                        dr_prediction_z3_64x32_avx2(dst, stride, left, upsample_left, dy);
                    }
                    _ => {}
                }
            } else {
                match bh {
                    4 => {
                        dr_prediction_z3_16x4_avx2(dst, stride, left, upsample_left, dy);
                    }
                    8 => {
                        dr_prediction_z3_32x8_avx2(dst, stride, left, upsample_left, dy);
                    }
                    16 => {
                        dr_prediction_z3_64x16_avx2(dst, stride, left, upsample_left, dy);
                    }
                    _ => {}
                }
            }
        }
    }
}

// !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER
