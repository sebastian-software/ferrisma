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
// Only the high-bitdepth directional predictors are translated:
// av1_highbd_dr_prediction_z1_avx2, av1_highbd_dr_prediction_z2_avx2 and
// av1_highbd_dr_prediction_z3_avx2, with the CONFIG_AV1_HIGHBITDEPTH tables
// and transposes and the static helpers they call. The low-bitdepth DC, V, H,
// Paeth and directional predictors of the same file are not translated. The
// `!CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER` branches are taken (the port
// builds the encoder and decoder with CONFIG_REALTIME_ONLY=0).
//
// The predictors use no helper from intrapred_x86.h or intrapred_utils.h (the
// tables and transposes there are low-bitdepth); the high-bitdepth transposes
// they call from lpf_common_sse2.h are in crate::simd::aom_dsp__x86__lpf_common_sse2.
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
use core::arch::x86_64::*;

use crate::simd::aom_dsp__x86__lpf_common_sse2::*;

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
        *d.add(i + 8) = _mm256_insertf128_si256::<0>(dd[i + 8], _mm256_extracti128_si256::<1>(dd[i]));
    }
}
// CONFIG_AV1_HIGHBITDEPTH

// CONFIG_AV1_HIGHBITDEPTH

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn highbd_dr_prediction_z1_4xN_internal_avx2(N: i32, dst: *mut __m128i, above: *const u16,
                                                    upsample_above: i32, dx: i32) {
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

            base_inc128 = _mm_setr_epi16(base as i16, (base + 2) as i16, (base + 4) as i16,
                                         (base + 6) as i16, (base + 8) as i16, (base + 10) as i16,
                                         (base + 12) as i16, (base + 14) as i16);
            shift = _mm256_srli_epi16::<1>(_mm256_and_si256(
                _mm256_sll_epi16(_mm256_set1_epi16(x as i16), _mm_cvtsi32_si128(upsample_above)),
                _mm256_set1_epi16(0x3f)));
        } else {
            base_inc128 = _mm_setr_epi16(base as i16, (base + 1) as i16, (base + 2) as i16,
                                         (base + 3) as i16, (base + 4) as i16, (base + 5) as i16,
                                         (base + 6) as i16, (base + 7) as i16);
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
unsafe fn highbd_dr_prediction_32bit_z1_4xN_internal_avx2(N: i32, dst: *mut __m128i,
                                                          above: *const u16, upsample_above: i32,
                                                          dx: i32) {
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

        a0 = _mm256_cvtepu16_epi32(_mm_loadu_si128(above.offset(base as isize) as *const __m128i));
        a1 = _mm256_cvtepu16_epi32(_mm_loadu_si128(above.offset(base as isize + 1) as *const __m128i));

        if upsample_above != 0 {
            a0 = _mm256_permutevar8x32_epi32(a0, _mm256_set_epi32(7, 5, 3, 1, 6, 4, 2, 0));
            a1 = _mm256_castsi128_si256(_mm256_extracti128_si256::<1>(a0));
            base_inc128 = _mm_setr_epi32(base, base + 2, base + 4, base + 6);
            shift = _mm256_srli_epi32::<1>(_mm256_and_si256(
                _mm256_sll_epi32(_mm256_set1_epi32(x), _mm_cvtsi32_si128(upsample_above)),
                _mm256_set1_epi32(0x3f)));
        } else {
            base_inc128 = _mm_setr_epi32(base, base + 1, base + 2, base + 3);
            shift = _mm256_srli_epi32::<1>(_mm256_and_si256(_mm256_set1_epi32(x),
                                                            _mm256_set1_epi32(0x3f)));
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
unsafe fn highbd_dr_prediction_z1_4xN_avx2(N: i32, dst: *mut u16, stride: isize, above: *const u16,
                                           upsample_above: i32, dx: i32, bd: i32) {
    let mut dstvec: [__m128i; 16] = core::mem::zeroed();
    if bd < 12 {
        highbd_dr_prediction_z1_4xN_internal_avx2(N, dstvec.as_mut_ptr(), above, upsample_above, dx);
    } else {
        highbd_dr_prediction_32bit_z1_4xN_internal_avx2(N, dstvec.as_mut_ptr(), above,
                                                        upsample_above, dx);
    }
    for i in 0..N {
        _mm_storel_epi64(dst.offset(stride * i as isize) as *mut __m128i, dstvec[i as usize]);
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn highbd_dr_prediction_32bit_z1_8xN_internal_avx2(N: i32, dst: *mut __m128i,
                                                          above: *const u16, upsample_above: i32,
                                                          dx: i32) {
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

        a0 = _mm256_cvtepu16_epi32(_mm_loadu_si128(above.offset(base as isize) as *const __m128i));
        a1 = _mm256_cvtepu16_epi32(_mm_loadu_si128(above.offset(base as isize + 1) as *const __m128i));

        if upsample_above != 0 {
            a0 = _mm256_permutevar8x32_epi32(a0, _mm256_set_epi32(7, 5, 3, 1, 6, 4, 2, 0));
            a1 = _mm256_castsi128_si256(_mm256_extracti128_si256::<1>(a0));

            a0_1 = _mm256_cvtepu16_epi32(_mm_loadu_si128(above.offset(base as isize + 8)
                                                         as *const __m128i));
            a0_1 = _mm256_permutevar8x32_epi32(a0_1, _mm256_set_epi32(7, 5, 3, 1, 6, 4, 2, 0));
            a1_1 = _mm256_castsi128_si256(_mm256_extracti128_si256::<1>(a0_1));

            a0 = _mm256_inserti128_si256::<1>(a0, _mm256_castsi256_si128(a0_1));
            a1 = _mm256_inserti128_si256::<1>(a1, _mm256_castsi256_si128(a1_1));
            base_inc256 = _mm256_setr_epi32(base, base + 2, base + 4, base + 6, base + 8, base + 10,
                                            base + 12, base + 14);
            shift = _mm256_srli_epi32::<1>(_mm256_and_si256(
                _mm256_sll_epi32(_mm256_set1_epi32(x), _mm_cvtsi32_si128(upsample_above)),
                _mm256_set1_epi32(0x3f)));
        } else {
            base_inc256 = _mm256_setr_epi32(base, base + 1, base + 2, base + 3, base + 4, base + 5,
                                            base + 6, base + 7);
            shift = _mm256_srli_epi32::<1>(_mm256_and_si256(_mm256_set1_epi32(x),
                                                            _mm256_set1_epi32(0x3f)));
        }

        diff = _mm256_sub_epi32(a1, a0); // a[x+1] - a[x]
        a32 = _mm256_slli_epi32::<5>(a0); // a[x] * 32
        a32 = _mm256_add_epi32(a32, a16); // a[x] * 32 + 16

        b = _mm256_mullo_epi32(diff, shift);
        res = _mm256_add_epi32(a32, b);
        res = _mm256_srli_epi32::<5>(res);

        res1 = _mm256_packus_epi32(res, _mm256_castsi128_si256(_mm256_extracti128_si256::<1>(res)));

        mask256 = _mm256_cmpgt_epi32(max_base_x256, base_inc256);
        mask256 = _mm256_packs_epi32(
            mask256, _mm256_castsi128_si256(_mm256_extracti128_si256::<1>(mask256))); // goto 16 bit
        res1 = _mm256_blendv_epi8(a_mbase_x, res1, mask256);
        *dst.add(r as usize) = _mm256_castsi256_si128(res1);
        x += dx;
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn highbd_dr_prediction_z1_8xN_internal_avx2(N: i32, dst: *mut __m128i, above: *const u16,
                                                    upsample_above: i32, dx: i32) {
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
            atmp2 = _mm_shuffle_epi8(a0_x128, load_mask128(HighbdEvenOddMaskx.0[0].as_ptr().add(16)));
            atmp3 = _mm_shuffle_epi8(a1_x128, load_mask128(HighbdEvenOddMaskx.0[0].as_ptr().add(16)));
            mask = _mm_cmpgt_epi8(load_mask128(HighbdEvenOddMaskx.0[0].as_ptr()), _mm_set1_epi8(15));
            a0_x128 = _mm_blendv_epi8(atmp0, atmp1, mask);
            mask = _mm_cmpgt_epi8(load_mask128(HighbdEvenOddMaskx.0[0].as_ptr().add(16)),
                                  _mm_set1_epi8(15));
            a1_x128 = _mm_blendv_epi8(atmp2, atmp3, mask);

            base_inc256 = _mm256_setr_epi16(base as i16, (base + 2) as i16, (base + 4) as i16,
                                            (base + 6) as i16, (base + 8) as i16, (base + 10) as i16,
                                            (base + 12) as i16, (base + 14) as i16, 0, 0, 0, 0, 0, 0,
                                            0, 0);
            shift = _mm256_srli_epi16::<1>(_mm256_and_si256(
                _mm256_sll_epi16(_mm256_set1_epi16(x as i16), _mm_cvtsi32_si128(upsample_above)),
                c3f));
        } else {
            a1_x128 = _mm_loadu_si128(above.offset(base as isize + 1) as *const __m128i);
            base_inc256 = _mm256_setr_epi16(base as i16, (base + 1) as i16, (base + 2) as i16,
                                            (base + 3) as i16, (base + 4) as i16, (base + 5) as i16,
                                            (base + 6) as i16, (base + 7) as i16, 0, 0, 0, 0, 0, 0,
                                            0, 0);
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
unsafe fn highbd_dr_prediction_z1_8xN_avx2(N: i32, dst: *mut u16, stride: isize, above: *const u16,
                                           upsample_above: i32, dx: i32, bd: i32) {
    let mut dstvec: [__m128i; 32] = core::mem::zeroed();
    if bd < 12 {
        highbd_dr_prediction_z1_8xN_internal_avx2(N, dstvec.as_mut_ptr(), above, upsample_above, dx);
    } else {
        highbd_dr_prediction_32bit_z1_8xN_internal_avx2(N, dstvec.as_mut_ptr(), above,
                                                        upsample_above, dx);
    }
    for i in 0..N {
        _mm_storeu_si128(dst.offset(stride * i as isize) as *mut __m128i, dstvec[i as usize]);
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn highbd_dr_prediction_32bit_z1_16xN_internal_avx2(N: i32, dstvec: *mut __m256i,
                                                           above: *const u16, upsample_above: i32,
                                                           dx: i32) {
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
        let shift = _mm256_srli_epi32::<1>(_mm256_and_si256(_mm256_set1_epi32(x),
                                                            _mm256_set1_epi32(0x3f)));

        a0 = _mm256_cvtepu16_epi32(_mm_loadu_si128(above.offset(base as isize) as *const __m128i));
        a1 = _mm256_cvtepu16_epi32(_mm_loadu_si128(above.offset(base as isize + 1) as *const __m128i));

        diff = _mm256_sub_epi32(a1, a0); // a[x+1] - a[x]
        a32 = _mm256_slli_epi32::<5>(a0); // a[x] * 32
        a32 = _mm256_add_epi32(a32, a16); // a[x] * 32 + 16
        b = _mm256_mullo_epi32(diff, shift);

        res[0] = _mm256_add_epi32(a32, b);
        res[0] = _mm256_srli_epi32::<5>(res[0]);
        res[0] = _mm256_packus_epi32(res[0],
                                     _mm256_castsi128_si256(_mm256_extracti128_si256::<1>(res[0])));

        let mdif = max_base_x - base;
        if mdif > 8 {
            a0_1 = _mm256_cvtepu16_epi32(_mm_loadu_si128(above.offset(base as isize + 8)
                                                         as *const __m128i));
            a1_1 = _mm256_cvtepu16_epi32(_mm_loadu_si128(above.offset(base as isize + 9)
                                                         as *const __m128i));

            diff = _mm256_sub_epi32(a1_1, a0_1); // a[x+1] - a[x]
            a32 = _mm256_slli_epi32::<5>(a0_1); // a[x] * 32
            a32 = _mm256_add_epi32(a32, a16); // a[x] * 32 + 16
            b = _mm256_mullo_epi32(diff, shift);

            res[1] = _mm256_add_epi32(a32, b);
            res[1] = _mm256_srli_epi32::<5>(res[1]);
            res[1] = _mm256_packus_epi32(res[1],
                                         _mm256_castsi128_si256(_mm256_extracti128_si256::<1>(res[1])));
        } else {
            res[1] = a_mbase_x;
        }
        res1 = _mm256_inserti128_si256::<1>(res[0], _mm256_castsi256_si128(res[1])); // 16 16bit values

        base_inc256 = _mm256_setr_epi16(base as i16, (base + 1) as i16, (base + 2) as i16,
                                        (base + 3) as i16, (base + 4) as i16, (base + 5) as i16,
                                        (base + 6) as i16, (base + 7) as i16, (base + 8) as i16,
                                        (base + 9) as i16, (base + 10) as i16, (base + 11) as i16,
                                        (base + 12) as i16, (base + 13) as i16, (base + 14) as i16,
                                        (base + 15) as i16);
        mask256 = _mm256_cmpgt_epi16(max_base_x256, base_inc256);
        *dstvec.add(r as usize) = _mm256_blendv_epi8(a_mbase_x, res1, mask256);
        x += dx;
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn highbd_dr_prediction_z1_16xN_internal_avx2(N: i32, dstvec: *mut __m256i,
                                                     above: *const u16, upsample_above: i32,
                                                     dx: i32) {
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

        base_inc256 = _mm256_setr_epi16(base as i16, (base + 1) as i16, (base + 2) as i16,
                                        (base + 3) as i16, (base + 4) as i16, (base + 5) as i16,
                                        (base + 6) as i16, (base + 7) as i16, (base + 8) as i16,
                                        (base + 9) as i16, (base + 10) as i16, (base + 11) as i16,
                                        (base + 12) as i16, (base + 13) as i16, (base + 14) as i16,
                                        (base + 15) as i16);
        mask256 = _mm256_cmpgt_epi16(max_base_x256, base_inc256);
        *dstvec.add(r as usize) = _mm256_blendv_epi8(a_mbase_x, res, mask256);
        x += dx;
    }
}

#[target_feature(enable = "avx2")]
unsafe fn highbd_dr_prediction_z1_16xN_avx2(N: i32, dst: *mut u16, stride: isize, above: *const u16,
                                            upsample_above: i32, dx: i32, bd: i32) {
    let mut dstvec: [__m256i; 64] = core::mem::zeroed();
    if bd < 12 {
        highbd_dr_prediction_z1_16xN_internal_avx2(N, dstvec.as_mut_ptr(), above, upsample_above, dx);
    } else {
        highbd_dr_prediction_32bit_z1_16xN_internal_avx2(N, dstvec.as_mut_ptr(), above,
                                                         upsample_above, dx);
    }
    for i in 0..N {
        _mm256_storeu_si256(dst.offset(stride * i as isize) as *mut __m256i, dstvec[i as usize]);
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn highbd_dr_prediction_32bit_z1_32xN_internal_avx2(N: i32, dstvec: *mut __m256i,
                                                           above: *const u16, upsample_above: i32,
                                                           dx: i32) {
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
                a0 = _mm256_cvtepu16_epi32(_mm_loadu_si128(above.offset((base + j) as isize)
                                                           as *const __m128i));
                a1 = _mm256_cvtepu16_epi32(_mm_loadu_si128(above.offset((base + 1 + j) as isize)
                                                           as *const __m128i));

                diff = _mm256_sub_epi32(a1, a0); // a[x+1] - a[x]
                a32 = _mm256_slli_epi32::<5>(a0); // a[x] * 32
                a32 = _mm256_add_epi32(a32, a16); // a[x] * 32 + 16
                b = _mm256_mullo_epi32(diff, shift);

                res[0] = _mm256_add_epi32(a32, b);
                res[0] = _mm256_srli_epi32::<5>(res[0]);
                res[0] = _mm256_packus_epi32(
                    res[0], _mm256_castsi128_si256(_mm256_extracti128_si256::<1>(res[0])));
                if mdif > 8 {
                    a0_1 = _mm256_cvtepu16_epi32(_mm_loadu_si128(above.offset((base + 8 + j) as isize)
                                                                 as *const __m128i));
                    a1_1 = _mm256_cvtepu16_epi32(_mm_loadu_si128(above.offset((base + 9 + j) as isize)
                                                                 as *const __m128i));

                    diff = _mm256_sub_epi32(a1_1, a0_1); // a[x+1] - a[x]
                    a32 = _mm256_slli_epi32::<5>(a0_1); // a[x] * 32
                    a32 = _mm256_add_epi32(a32, a16); // a[x] * 32 + 16
                    b = _mm256_mullo_epi32(diff, shift);

                    res[1] = _mm256_add_epi32(a32, b);
                    res[1] = _mm256_srli_epi32::<5>(res[1]);
                    res[1] = _mm256_packus_epi32(
                        res[1], _mm256_castsi128_si256(_mm256_extracti128_si256::<1>(res[1])));
                } else {
                    res[1] = a_mbase_x;
                }
                res1 = _mm256_inserti128_si256::<1>(res[0], _mm256_castsi256_si128(res[1])); // 16 16bit values
                base_inc256 = _mm256_setr_epi16(
                    (base + j) as i16, (base + j + 1) as i16, (base + j + 2) as i16,
                    (base + j + 3) as i16, (base + j + 4) as i16, (base + j + 5) as i16,
                    (base + j + 6) as i16, (base + j + 7) as i16, (base + j + 8) as i16,
                    (base + j + 9) as i16, (base + j + 10) as i16, (base + j + 11) as i16,
                    (base + j + 12) as i16, (base + j + 13) as i16, (base + j + 14) as i16,
                    (base + j + 15) as i16);

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
unsafe fn highbd_dr_prediction_z1_32xN_internal_avx2(N: i32, dstvec: *mut __m256i,
                                                     above: *const u16, upsample_above: i32,
                                                     dx: i32) {
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
                    (base + j) as i16, (base + j + 1) as i16, (base + j + 2) as i16,
                    (base + j + 3) as i16, (base + j + 4) as i16, (base + j + 5) as i16,
                    (base + j + 6) as i16, (base + j + 7) as i16, (base + j + 8) as i16,
                    (base + j + 9) as i16, (base + j + 10) as i16, (base + j + 11) as i16,
                    (base + j + 12) as i16, (base + j + 13) as i16, (base + j + 14) as i16,
                    (base + j + 15) as i16);

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
unsafe fn highbd_dr_prediction_z1_32xN_avx2(N: i32, dst: *mut u16, stride: isize, above: *const u16,
                                            upsample_above: i32, dx: i32, bd: i32) {
    let mut dstvec: [__m256i; 128] = core::mem::zeroed();
    if bd < 12 {
        highbd_dr_prediction_z1_32xN_internal_avx2(N, dstvec.as_mut_ptr(), above, upsample_above, dx);
    } else {
        highbd_dr_prediction_32bit_z1_32xN_internal_avx2(N, dstvec.as_mut_ptr(), above,
                                                         upsample_above, dx);
    }
    for i in 0..N {
        _mm256_storeu_si256(dst.offset(stride * i as isize) as *mut __m256i, dstvec[i as usize]);
        _mm256_storeu_si256(dst.offset(stride * i as isize + 16) as *mut __m256i,
                            dstvec[(i + N) as usize]);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn highbd_dr_prediction_32bit_z1_64xN_avx2(N: i32, mut dst: *mut u16, stride: isize,
                                                  above: *const u16, upsample_above: i32, dx: i32) {
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

        let shift = _mm256_srli_epi32::<1>(_mm256_and_si256(_mm256_set1_epi32(x),
                                                            _mm256_set1_epi32(0x3f)));

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
                    res[0], _mm256_castsi128_si256(_mm256_extracti128_si256::<1>(res[0])));
                if mdif > 8 {
                    a0_1_128 = _mm_loadu_si128(above.offset((base + 8 + j) as isize) as *const __m128i);
                    a1_1_128 = _mm_loadu_si128(above.offset((base + 9 + j) as isize) as *const __m128i);
                    a0_1 = _mm256_cvtepu16_epi32(a0_1_128);
                    a1_1 = _mm256_cvtepu16_epi32(a1_1_128);

                    diff = _mm256_sub_epi32(a1_1, a0_1); // a[x+1] - a[x]
                    a32 = _mm256_slli_epi32::<5>(a0_1); // a[x] * 32
                    a32 = _mm256_add_epi32(a32, a16); // a[x] * 32 + 16
                    b = _mm256_mullo_epi32(diff, shift);

                    res[1] = _mm256_add_epi32(a32, b);
                    res[1] = _mm256_srli_epi32::<5>(res[1]);
                    res[1] = _mm256_packus_epi32(
                        res[1], _mm256_castsi128_si256(_mm256_extracti128_si256::<1>(res[1])));
                } else {
                    res[1] = a_mbase_x;
                }
                res1 = _mm256_inserti128_si256::<1>(res[0], _mm256_castsi256_si128(res[1])); // 16 16bit values
                base_inc256 = _mm256_setr_epi16(
                    (base + j) as i16, (base + j + 1) as i16, (base + j + 2) as i16,
                    (base + j + 3) as i16, (base + j + 4) as i16, (base + j + 5) as i16,
                    (base + j + 6) as i16, (base + j + 7) as i16, (base + j + 8) as i16,
                    (base + j + 9) as i16, (base + j + 10) as i16, (base + j + 11) as i16,
                    (base + j + 12) as i16, (base + j + 13) as i16, (base + j + 14) as i16,
                    (base + j + 15) as i16);

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
unsafe fn highbd_dr_prediction_z1_64xN_avx2(N: i32, mut dst: *mut u16, stride: isize,
                                            above: *const u16, upsample_above: i32, dx: i32) {
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
                    (base + j) as i16, (base + j + 1) as i16, (base + j + 2) as i16,
                    (base + j + 3) as i16, (base + j + 4) as i16, (base + j + 5) as i16,
                    (base + j + 6) as i16, (base + j + 7) as i16, (base + j + 8) as i16,
                    (base + j + 9) as i16, (base + j + 10) as i16, (base + j + 11) as i16,
                    (base + j + 12) as i16, (base + j + 13) as i16, (base + j + 14) as i16,
                    (base + j + 15) as i16);

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
pub unsafe extern "C-unwind" fn av1_highbd_dr_prediction_z1_avx2(dst: *mut u16, stride: isize, bw: i32,
                                                                 bh: i32, above: *const u16,
                                                                 left: *const u16,
                                                                 upsample_above: i32, dx: i32,
                                                                 dy: i32, bd: i32) {
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
unsafe fn highbd_transpose_TX_16X16(src: *const u16, pitchSrc: isize, dst: *mut u16,
                                    pitchDst: isize) {
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
unsafe fn highbd_transpose(src: *const u16, pitchSrc: isize, dst: *mut u16, pitchDst: isize,
                           width: i32, height: i32) {
    let mut j = 0;
    while j < height {
        let mut i = 0;
        while i < width {
            highbd_transpose_TX_16X16(src.offset(i as isize * pitchSrc + j as isize), pitchSrc,
                                      dst.offset(j as isize * pitchDst + i as isize), pitchDst);
            i += 16;
        }
        j += 16;
    }
}

#[target_feature(enable = "avx2")]
unsafe fn highbd_dr_prediction_32bit_z2_Nx4_avx2(N: i32, mut dst: *mut u16, stride: isize,
                                                 above: *const u16, left: *const u16,
                                                 upsample_above: i32, upsample_left: i32, dx: i32,
                                                 dy: i32) {
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
            a0_x128 = _mm_loadu_si128(above.offset((base_x + base_shift) as isize) as *const __m128i);
            if upsample_above != 0 {
                a0_x128 = _mm_shuffle_epi8(
                    a0_x128, load_mask128(HighbdEvenOddMaskx4.0[base_shift as usize].as_ptr()));
                a1_x128 = _mm_srli_si128::<8>(a0_x128);

                shift = _mm256_castsi128_si256(_mm_srli_epi32::<1>(_mm_and_si128(
                    _mm_sll_epi32(_mm_setr_epi32(-y * dx, (1 << 6) - y * dx, (2 << 6) - y * dx,
                                                 (3 << 6) - y * dx),
                                  _mm_cvtsi32_si128(upsample_above)),
                    c3f)));
            } else {
                a0_x128 = _mm_shuffle_epi8(a0_x128,
                                           load_mask128(HighbdLoadMaskx.0[base_shift as usize].as_ptr()));
                a1_x128 = _mm_srli_si128::<2>(a0_x128);

                shift = _mm256_castsi128_si256(_mm_srli_epi32::<1>(_mm_and_si128(
                    _mm_setr_epi32(-y * dx, (1 << 6) - y * dx, (2 << 6) - y * dx, (3 << 6) - y * dx),
                    c3f)));
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

            a0_y = _mm_setr_epi32(*left.offset(base_y_c[0] as isize) as i32,
                                  *left.offset(base_y_c[1] as isize) as i32,
                                  *left.offset(base_y_c[2] as isize) as i32,
                                  *left.offset(base_y_c[3] as isize) as i32);
            a1_y = _mm_setr_epi32(*left.offset(base_y_c[0] as isize + 1) as i32,
                                  *left.offset(base_y_c[1] as isize + 1) as i32,
                                  *left.offset(base_y_c[2] as isize + 1) as i32,
                                  *left.offset(base_y_c[3] as isize + 1) as i32);

            if upsample_left != 0 {
                shifty = _mm_srli_epi32::<1>(_mm_and_si128(
                    _mm_sll_epi32(y_c128, _mm_cvtsi32_si128(upsample_left)), c3f));
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
unsafe fn highbd_dr_prediction_z2_Nx4_avx2(N: i32, mut dst: *mut u16, stride: isize,
                                           above: *const u16, left: *const u16, upsample_above: i32,
                                           upsample_left: i32, dx: i32, dy: i32) {
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
            a0_x128 = _mm_loadu_si128(above.offset((base_x + base_shift) as isize) as *const __m128i);
            if upsample_above != 0 {
                a0_x128 = _mm_shuffle_epi8(
                    a0_x128, load_mask128(HighbdEvenOddMaskx4.0[base_shift as usize].as_ptr()));
                a1_x128 = _mm_srli_si128::<8>(a0_x128);

                shift = _mm256_castsi128_si256(_mm_srli_epi16::<1>(_mm_and_si128(
                    _mm_sll_epi16(_mm_setr_epi16((-y * dx) as i16, ((1 << 6) - y * dx) as i16,
                                                 ((2 << 6) - y * dx) as i16,
                                                 ((3 << 6) - y * dx) as i16, 0, 0, 0, 0),
                                  _mm_cvtsi32_si128(upsample_above)),
                    c3f)));
            } else {
                a0_x128 = _mm_shuffle_epi8(a0_x128,
                                           load_mask128(HighbdLoadMaskx.0[base_shift as usize].as_ptr()));
                a1_x128 = _mm_srli_si128::<2>(a0_x128);

                shift = _mm256_castsi128_si256(_mm_srli_epi16::<1>(_mm_and_si128(
                    _mm_setr_epi16((-y * dx) as i16, ((1 << 6) - y * dx) as i16,
                                   ((2 << 6) - y * dx) as i16, ((3 << 6) - y * dx) as i16, 0, 0, 0,
                                   0),
                    c3f)));
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

            a0_y = _mm_setr_epi16(*left.offset(base_y_c[0] as isize) as i16,
                                  *left.offset(base_y_c[1] as isize) as i16,
                                  *left.offset(base_y_c[2] as isize) as i16,
                                  *left.offset(base_y_c[3] as isize) as i16, 0, 0, 0, 0);
            a1_y = _mm_setr_epi16(*left.offset(base_y_c[0] as isize + 1) as i16,
                                  *left.offset(base_y_c[1] as isize + 1) as i16,
                                  *left.offset(base_y_c[2] as isize + 1) as i16,
                                  *left.offset(base_y_c[3] as isize + 1) as i16, 0, 0, 0, 0);

            if upsample_left != 0 {
                shifty = _mm_srli_epi16::<1>(_mm_and_si128(
                    _mm_sll_epi16(y_c128, _mm_cvtsi32_si128(upsample_left)), c3f));
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
unsafe fn highbd_dr_prediction_32bit_z2_Nx8_avx2(N: i32, mut dst: *mut u16, stride: isize,
                                                 above: *const u16, left: *const u16,
                                                 upsample_above: i32, upsample_left: i32, dx: i32,
                                                 dy: i32) {
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
            a0_x128 = _mm_loadu_si128(above.offset((base_x + base_shift) as isize) as *const __m128i);
            if upsample_above != 0 {
                let mut mask: __m128i;
                let atmp0: __m128i;
                let atmp1: __m128i;
                let atmp2: __m128i;
                let atmp3: __m128i;
                let mask_row = HighbdEvenOddMaskx.0[base_shift as usize].as_ptr();
                a1_x128 = _mm_loadu_si128(above.offset((base_x + 8 + base_shift) as isize)
                                          as *const __m128i);
                atmp0 = _mm_shuffle_epi8(a0_x128, load_mask128(mask_row));
                atmp1 = _mm_shuffle_epi8(a1_x128, load_mask128(mask_row));
                atmp2 = _mm_shuffle_epi8(a0_x128, load_mask128(mask_row.add(16)));
                atmp3 = _mm_shuffle_epi8(a1_x128, load_mask128(mask_row.add(16)));
                mask = _mm_cmpgt_epi8(load_mask128(mask_row), _mm_set1_epi8(15));
                a0_x128 = _mm_blendv_epi8(atmp0, atmp1, mask);
                mask = _mm_cmpgt_epi8(load_mask128(mask_row.add(16)), _mm_set1_epi8(15));
                a1_x128 = _mm_blendv_epi8(atmp2, atmp3, mask);
                shift = _mm256_srli_epi32::<1>(_mm256_and_si256(
                    _mm256_sll_epi32(_mm256_setr_epi32(-y * dx, (1 << 6) - y * dx,
                                                       (2 << 6) - y * dx, (3 << 6) - y * dx,
                                                       (4 << 6) - y * dx, (5 << 6) - y * dx,
                                                       (6 << 6) - y * dx, (7 << 6) - y * dx),
                                     _mm_cvtsi32_si128(upsample_above)),
                    c3f));
            } else {
                a1_x128 = _mm_loadu_si128(above.offset((base_x + 1 + base_shift) as isize)
                                          as *const __m128i);
                a0_x128 = _mm_shuffle_epi8(a0_x128,
                                           load_mask128(HighbdLoadMaskx.0[base_shift as usize].as_ptr()));
                a1_x128 = _mm_shuffle_epi8(a1_x128,
                                           load_mask128(HighbdLoadMaskx.0[base_shift as usize].as_ptr()));

                shift = _mm256_srli_epi32::<1>(_mm256_and_si256(
                    _mm256_setr_epi32(-y * dx, (1 << 6) - y * dx, (2 << 6) - y * dx,
                                      (3 << 6) - y * dx, (4 << 6) - y * dx, (5 << 6) - y * dx,
                                      (6 << 6) - y * dx, (7 << 6) - y * dx),
                    c3f));
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
                res, _mm256_castsi128_si256(_mm256_extracti128_si256::<1>(res))));
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
                *left.offset(base_y_c[0] as isize) as i16, *left.offset(base_y_c[1] as isize) as i16,
                *left.offset(base_y_c[2] as isize) as i16, *left.offset(base_y_c[3] as isize) as i16,
                *left.offset(base_y_c[4] as isize) as i16, *left.offset(base_y_c[5] as isize) as i16,
                *left.offset(base_y_c[6] as isize) as i16, *left.offset(base_y_c[7] as isize) as i16));
            a1_y = _mm256_cvtepu16_epi32(_mm_setr_epi16(
                *left.offset(base_y_c[0] as isize + 1) as i16,
                *left.offset(base_y_c[1] as isize + 1) as i16,
                *left.offset(base_y_c[2] as isize + 1) as i16,
                *left.offset(base_y_c[3] as isize + 1) as i16,
                *left.offset(base_y_c[4] as isize + 1) as i16,
                *left.offset(base_y_c[5] as isize + 1) as i16,
                *left.offset(base_y_c[6] as isize + 1) as i16,
                *left.offset(base_y_c[7] as isize + 1) as i16));

            if upsample_left != 0 {
                shift = _mm256_srli_epi32::<1>(_mm256_and_si256(
                    _mm256_sll_epi32(y_c256, _mm_cvtsi32_si128(upsample_left)), c3f));
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
                res, _mm256_castsi128_si256(_mm256_extracti128_si256::<1>(res))));
        } else {
            resy = resx;
        }
        resxy = _mm_blendv_epi8(resx, resy, base_mask128(base_min_diff));
        _mm_storeu_si128(dst as *mut __m128i, resxy);
        dst = dst.offset(stride);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn highbd_dr_prediction_z2_Nx8_avx2(N: i32, mut dst: *mut u16, stride: isize,
                                           above: *const u16, left: *const u16, upsample_above: i32,
                                           upsample_left: i32, dx: i32, dy: i32) {
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
            a0_x128 = _mm_loadu_si128(above.offset((base_x + base_shift) as isize) as *const __m128i);
            if upsample_above != 0 {
                let mut mask: __m128i;
                let atmp0: __m128i;
                let atmp1: __m128i;
                let atmp2: __m128i;
                let atmp3: __m128i;
                let mask_row = HighbdEvenOddMaskx.0[base_shift as usize].as_ptr();
                a1_x128 = _mm_loadu_si128(above.offset((base_x + 8 + base_shift) as isize)
                                          as *const __m128i);
                atmp0 = _mm_shuffle_epi8(a0_x128, load_mask128(mask_row));
                atmp1 = _mm_shuffle_epi8(a1_x128, load_mask128(mask_row));
                atmp2 = _mm_shuffle_epi8(a0_x128, load_mask128(mask_row.add(16)));
                atmp3 = _mm_shuffle_epi8(a1_x128, load_mask128(mask_row.add(16)));
                mask = _mm_cmpgt_epi8(load_mask128(mask_row), _mm_set1_epi8(15));
                a0_x128 = _mm_blendv_epi8(atmp0, atmp1, mask);
                mask = _mm_cmpgt_epi8(load_mask128(mask_row.add(16)), _mm_set1_epi8(15));
                a1_x128 = _mm_blendv_epi8(atmp2, atmp3, mask);

                shift = _mm256_castsi128_si256(_mm_srli_epi16::<1>(_mm_and_si128(
                    _mm_sll_epi16(_mm_setr_epi16((-y * dx) as i16, ((1 << 6) - y * dx) as i16,
                                                 ((2 << 6) - y * dx) as i16,
                                                 ((3 << 6) - y * dx) as i16,
                                                 ((4 << 6) - y * dx) as i16,
                                                 ((5 << 6) - y * dx) as i16,
                                                 ((6 << 6) - y * dx) as i16,
                                                 ((7 << 6) - y * dx) as i16),
                                  _mm_cvtsi32_si128(upsample_above)),
                    c3f)));
            } else {
                a1_x128 = _mm_loadu_si128(above.offset((base_x + 1 + base_shift) as isize)
                                          as *const __m128i);
                a0_x128 = _mm_shuffle_epi8(a0_x128,
                                           load_mask128(HighbdLoadMaskx.0[base_shift as usize].as_ptr()));
                a1_x128 = _mm_shuffle_epi8(a1_x128,
                                           load_mask128(HighbdLoadMaskx.0[base_shift as usize].as_ptr()));

                shift = _mm256_castsi128_si256(_mm_srli_epi16::<1>(_mm_and_si128(
                    _mm_setr_epi16((-y * dx) as i16, ((1 << 6) - y * dx) as i16,
                                   ((2 << 6) - y * dx) as i16, ((3 << 6) - y * dx) as i16,
                                   ((4 << 6) - y * dx) as i16, ((5 << 6) - y * dx) as i16,
                                   ((6 << 6) - y * dx) as i16, ((7 << 6) - y * dx) as i16),
                    c3f)));
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
                *left.offset(base_y_c[0] as isize) as i16, *left.offset(base_y_c[1] as isize) as i16,
                *left.offset(base_y_c[2] as isize) as i16, *left.offset(base_y_c[3] as isize) as i16,
                *left.offset(base_y_c[4] as isize) as i16, *left.offset(base_y_c[5] as isize) as i16,
                *left.offset(base_y_c[6] as isize) as i16, *left.offset(base_y_c[7] as isize) as i16);
            a1_y = _mm_setr_epi16(*left.offset(base_y_c[0] as isize + 1) as i16,
                                  *left.offset(base_y_c[1] as isize + 1) as i16,
                                  *left.offset(base_y_c[2] as isize + 1) as i16,
                                  *left.offset(base_y_c[3] as isize + 1) as i16,
                                  *left.offset(base_y_c[4] as isize + 1) as i16,
                                  *left.offset(base_y_c[5] as isize + 1) as i16,
                                  *left.offset(base_y_c[6] as isize + 1) as i16,
                                  *left.offset(base_y_c[7] as isize + 1) as i16);

            if upsample_left != 0 {
                shifty = _mm_srli_epi16::<1>(_mm_and_si128(
                    _mm_sll_epi16(y_c128, _mm_cvtsi32_si128(upsample_left)), c3f));
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
unsafe fn highbd_dr_prediction_32bit_z2_HxW_avx2(H: i32, W: i32, mut dst: *mut u16, stride: isize,
                                                 above: *const u16, left: *const u16,
                                                 upsample_above: i32, upsample_left: i32, dx: i32,
                                                 dy: i32) {
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
                a0_x128 = _mm_loadu_si128(above.offset((base_x + base_shift) as isize) as *const __m128i);
                a1_x128 = _mm_loadu_si128(above.offset((base_x + base_shift + 1) as isize)
                                          as *const __m128i);
                a0_x128 = _mm_shuffle_epi8(a0_x128,
                                           load_mask128(HighbdLoadMaskx.0[base_shift as usize].as_ptr()));
                a1_x128 = _mm_shuffle_epi8(a1_x128,
                                           load_mask128(HighbdLoadMaskx.0[base_shift as usize].as_ptr()));

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
                    res, _mm256_castsi128_si256(_mm256_extracti128_si256::<1>(res)));
            }
            let mut base_shift8 = 0;
            if (base_x + 8) < (min_base_x - 1) {
                base_shift8 = min_base_x - (base_x + 8) - 1;
            }
            if base_shift8 > 7 {
                resx[1] = _mm256_setzero_si256();
            } else {
                a0_1_x128 = _mm_loadu_si128(above.offset((base_x + base_shift8 + 8) as isize)
                                            as *const __m128i);
                a1_1_x128 = _mm_loadu_si128(above.offset((base_x + base_shift8 + 9) as isize)
                                            as *const __m128i);
                a0_1_x128 = _mm_shuffle_epi8(a0_1_x128,
                                             load_mask128(HighbdLoadMaskx.0[base_shift8 as usize].as_ptr()));
                a1_1_x128 = _mm_shuffle_epi8(a1_1_x128,
                                             load_mask128(HighbdLoadMaskx.0[base_shift8 as usize].as_ptr()));

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
                    resx[1], _mm256_castsi128_si256(_mm256_extracti128_si256::<1>(resx[1])));
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
                    *left.offset(bc[0] as isize) as i16, *left.offset(bc[1] as isize) as i16,
                    *left.offset(bc[2] as isize) as i16, *left.offset(bc[3] as isize) as i16,
                    *left.offset(bc[4] as isize) as i16, *left.offset(bc[5] as isize) as i16,
                    *left.offset(bc[6] as isize) as i16, *left.offset(bc[7] as isize) as i16));
                a1_y = _mm256_cvtepu16_epi32(_mm_setr_epi16(
                    *left.offset(bc[0] as isize + 1) as i16, *left.offset(bc[1] as isize + 1) as i16,
                    *left.offset(bc[2] as isize + 1) as i16, *left.offset(bc[3] as isize + 1) as i16,
                    *left.offset(bc[4] as isize + 1) as i16, *left.offset(bc[5] as isize + 1) as i16,
                    *left.offset(bc[6] as isize + 1) as i16, *left.offset(bc[7] as isize + 1) as i16));

                shift = _mm256_srli_epi32::<1>(_mm256_and_si256(y_c256, c3f));

                diff = _mm256_sub_epi32(a1_y, a0_y); // a[x+1] - a[x]
                a32 = _mm256_slli_epi32::<5>(a0_y); // a[x] * 32
                a32 = _mm256_add_epi32(a32, a16); // a[x] * 32 + 16

                b = _mm256_mullo_epi32(diff, shift);
                res = _mm256_add_epi32(a32, b);
                res = _mm256_srli_epi32::<5>(res);

                resy[0] = _mm256_packus_epi32(
                    res, _mm256_castsi128_si256(_mm256_extracti128_si256::<1>(res)));

                a0_y = _mm256_cvtepu16_epi32(_mm_setr_epi16(
                    *left.offset(bc[8] as isize) as i16, *left.offset(bc[9] as isize) as i16,
                    *left.offset(bc[10] as isize) as i16, *left.offset(bc[11] as isize) as i16,
                    *left.offset(bc[12] as isize) as i16, *left.offset(bc[13] as isize) as i16,
                    *left.offset(bc[14] as isize) as i16, *left.offset(bc[15] as isize) as i16));
                a1_y = _mm256_cvtepu16_epi32(_mm_setr_epi16(
                    *left.offset(bc[8] as isize + 1) as i16, *left.offset(bc[9] as isize + 1) as i16,
                    *left.offset(bc[10] as isize + 1) as i16, *left.offset(bc[11] as isize + 1) as i16,
                    *left.offset(bc[12] as isize + 1) as i16, *left.offset(bc[13] as isize + 1) as i16,
                    *left.offset(bc[14] as isize + 1) as i16, *left.offset(bc[15] as isize + 1) as i16));
                shift = _mm256_srli_epi32::<1>(_mm256_and_si256(y_c_1_256, c3f));

                diff = _mm256_sub_epi32(a1_y, a0_y); // a[x+1] - a[x]
                a32 = _mm256_slli_epi32::<5>(a0_y); // a[x] * 32
                a32 = _mm256_add_epi32(a32, a16); // a[x] * 32 + 16

                b = _mm256_mullo_epi32(diff, shift);
                res = _mm256_add_epi32(a32, b);
                res = _mm256_srli_epi32::<5>(res);

                resy[1] = _mm256_packus_epi32(
                    res, _mm256_castsi128_si256(_mm256_extracti128_si256::<1>(res)));

                resy[0] = _mm256_inserti128_si256::<1>(resy[0], _mm256_castsi256_si128(resy[1])); // 16 16bit values
            }

            resxy = _mm256_blendv_epi8(resx[0], resy[0], base_mask256(base_min_diff));
            _mm256_storeu_si256(dst.add(j as usize) as *mut __m256i, resxy);
            j += 16;
        } // for j
        dst = dst.offset(stride);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn highbd_dr_prediction_z2_HxW_avx2(H: i32, W: i32, mut dst: *mut u16, stride: isize,
                                           above: *const u16, left: *const u16, upsample_above: i32,
                                           upsample_left: i32, dx: i32, dy: i32) {
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
                a0_x128 = _mm_loadu_si128(above.offset((base_x + base_shift) as isize) as *const __m128i);
                a1_x128 = _mm_loadu_si128(above.offset((base_x + base_shift + 1) as isize)
                                          as *const __m128i);
                a0_x128 = _mm_shuffle_epi8(a0_x128,
                                           load_mask128(HighbdLoadMaskx.0[base_shift as usize].as_ptr()));
                a1_x128 = _mm_shuffle_epi8(a1_x128,
                                           load_mask128(HighbdLoadMaskx.0[base_shift as usize].as_ptr()));

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
                a0_1_x128 = _mm_loadu_si128(above.offset((base_x + base_shift1 + 8) as isize)
                                            as *const __m128i);
                a1_1_x128 = _mm_loadu_si128(above.offset((base_x + base_shift1 + 9) as isize)
                                            as *const __m128i);
                a0_1_x128 = _mm_shuffle_epi8(a0_1_x128,
                                             load_mask128(HighbdLoadMaskx.0[base_shift1 as usize].as_ptr()));
                a1_1_x128 = _mm_shuffle_epi8(a1_1_x128,
                                             load_mask128(HighbdLoadMaskx.0[base_shift1 as usize].as_ptr()));

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
                mul16 = _mm256_min_epu16(_mm256_mullo_epi16(c256, dy256),
                                         _mm256_srli_epi16::<1>(min_base_y256));
                y_c256 = _mm256_sub_epi16(r6, mul16);
                base_y_c256 = _mm256_srai_epi16::<frac_bits_y>(y_c256);
                mask256 = _mm256_cmpgt_epi16(min_base_y256, base_y_c256);
                base_y_c256 = _mm256_andnot_si256(mask256, base_y_c256);
                _mm256_store_si256(base_y_c.0.as_mut_ptr() as *mut __m256i, base_y_c256);
                {
                    let bc = &base_y_c.0;
                    a0_y = _mm256_setr_epi16(
                        *left.offset(bc[0] as isize) as i16, *left.offset(bc[1] as isize) as i16,
                        *left.offset(bc[2] as isize) as i16, *left.offset(bc[3] as isize) as i16,
                        *left.offset(bc[4] as isize) as i16, *left.offset(bc[5] as isize) as i16,
                        *left.offset(bc[6] as isize) as i16, *left.offset(bc[7] as isize) as i16,
                        *left.offset(bc[8] as isize) as i16, *left.offset(bc[9] as isize) as i16,
                        *left.offset(bc[10] as isize) as i16, *left.offset(bc[11] as isize) as i16,
                        *left.offset(bc[12] as isize) as i16, *left.offset(bc[13] as isize) as i16,
                        *left.offset(bc[14] as isize) as i16, *left.offset(bc[15] as isize) as i16);
                }
                base_y_c256 = _mm256_add_epi16(base_y_c256, c1);
                _mm256_store_si256(base_y_c.0.as_mut_ptr() as *mut __m256i, base_y_c256);
                {
                    let bc = &base_y_c.0;
                    a1_y = _mm256_setr_epi16(
                        *left.offset(bc[0] as isize) as i16, *left.offset(bc[1] as isize) as i16,
                        *left.offset(bc[2] as isize) as i16, *left.offset(bc[3] as isize) as i16,
                        *left.offset(bc[4] as isize) as i16, *left.offset(bc[5] as isize) as i16,
                        *left.offset(bc[6] as isize) as i16, *left.offset(bc[7] as isize) as i16,
                        *left.offset(bc[8] as isize) as i16, *left.offset(bc[9] as isize) as i16,
                        *left.offset(bc[10] as isize) as i16, *left.offset(bc[11] as isize) as i16,
                        *left.offset(bc[12] as isize) as i16, *left.offset(bc[13] as isize) as i16,
                        *left.offset(bc[14] as isize) as i16, *left.offset(bc[15] as isize) as i16);
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
pub unsafe extern "C-unwind" fn av1_highbd_dr_prediction_z2_avx2(dst: *mut u16, stride: isize, bw: i32,
                                                                 bh: i32, above: *const u16,
                                                                 left: *const u16,
                                                                 upsample_above: i32,
                                                                 upsample_left: i32, dx: i32,
                                                                 dy: i32, bd: i32) {
    let _ = bd;
    debug_assert!(dx > 0);
    debug_assert!(dy > 0);
    match bw {
        4 => {
            if bd < 12 {
                highbd_dr_prediction_z2_Nx4_avx2(bh, dst, stride, above, left, upsample_above,
                                                 upsample_left, dx, dy);
            } else {
                highbd_dr_prediction_32bit_z2_Nx4_avx2(bh, dst, stride, above, left, upsample_above,
                                                       upsample_left, dx, dy);
            }
        }
        8 => {
            if bd < 12 {
                highbd_dr_prediction_z2_Nx8_avx2(bh, dst, stride, above, left, upsample_above,
                                                 upsample_left, dx, dy);
            } else {
                highbd_dr_prediction_32bit_z2_Nx8_avx2(bh, dst, stride, above, left, upsample_above,
                                                       upsample_left, dx, dy);
            }
        }
        _ => {
            if bd < 12 {
                highbd_dr_prediction_z2_HxW_avx2(bh, bw, dst, stride, above, left, upsample_above,
                                                 upsample_left, dx, dy);
            } else {
                highbd_dr_prediction_32bit_z2_HxW_avx2(bh, bw, dst, stride, above, left,
                                                       upsample_above, upsample_left, dx, dy);
            }
        }
    }
}

//  Directional prediction, zone 3 functions
#[target_feature(enable = "avx2")]
unsafe fn highbd_dr_prediction_z3_4x4_avx2(dst: *mut u16, stride: isize, left: *const u16,
                                           upsample_left: i32, dy: i32, bd: i32) {
    let mut dstvec: [__m128i; 4] = core::mem::zeroed();
    let mut d: [__m128i; 4] = core::mem::zeroed();
    if bd < 12 {
        highbd_dr_prediction_z1_4xN_internal_avx2(4, dstvec.as_mut_ptr(), left, upsample_left, dy);
    } else {
        highbd_dr_prediction_32bit_z1_4xN_internal_avx2(4, dstvec.as_mut_ptr(), left, upsample_left,
                                                        dy);
    }
    let v = dstvec.as_ptr();
    let o = d.as_mut_ptr();
    highbd_transpose4x8_8x4_low_sse2(v.add(0), v.add(1), v.add(2), v.add(3), o.add(0), o.add(1),
                                     o.add(2), o.add(3));
    _mm_storel_epi64(dst.offset(0 * stride) as *mut __m128i, d[0]);
    _mm_storel_epi64(dst.offset(1 * stride) as *mut __m128i, d[1]);
    _mm_storel_epi64(dst.offset(2 * stride) as *mut __m128i, d[2]);
    _mm_storel_epi64(dst.offset(3 * stride) as *mut __m128i, d[3]);
}

#[target_feature(enable = "avx2")]
unsafe fn highbd_dr_prediction_z3_8x8_avx2(dst: *mut u16, stride: isize, left: *const u16,
                                           upsample_left: i32, dy: i32, bd: i32) {
    let mut dstvec: [__m128i; 8] = core::mem::zeroed();
    let mut d: [__m128i; 8] = core::mem::zeroed();
    if bd < 12 {
        highbd_dr_prediction_z1_8xN_internal_avx2(8, dstvec.as_mut_ptr(), left, upsample_left, dy);
    } else {
        highbd_dr_prediction_32bit_z1_8xN_internal_avx2(8, dstvec.as_mut_ptr(), left, upsample_left,
                                                        dy);
    }
    let v = dstvec.as_ptr();
    let o = d.as_mut_ptr();
    highbd_transpose8x8_sse2(v.add(0), v.add(1), v.add(2), v.add(3), v.add(4), v.add(5), v.add(6),
                             v.add(7), o.add(0), o.add(1), o.add(2), o.add(3), o.add(4), o.add(5),
                             o.add(6), o.add(7));
    for i in 0..8 {
        _mm_storeu_si128(dst.offset(i as isize * stride) as *mut __m128i, d[i]);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn highbd_dr_prediction_z3_4x8_avx2(dst: *mut u16, stride: isize, left: *const u16,
                                           upsample_left: i32, dy: i32, bd: i32) {
    let mut dstvec: [__m128i; 4] = core::mem::zeroed();
    let mut d: [__m128i; 8] = core::mem::zeroed();
    if bd < 12 {
        highbd_dr_prediction_z1_8xN_internal_avx2(4, dstvec.as_mut_ptr(), left, upsample_left, dy);
    } else {
        highbd_dr_prediction_32bit_z1_8xN_internal_avx2(4, dstvec.as_mut_ptr(), left, upsample_left,
                                                        dy);
    }

    let v = dstvec.as_ptr();
    let o = d.as_mut_ptr();
    highbd_transpose4x8_8x4_sse2(v.add(0), v.add(1), v.add(2), v.add(3), o.add(0), o.add(1),
                                 o.add(2), o.add(3), o.add(4), o.add(5), o.add(6), o.add(7));
    for i in 0..8 {
        _mm_storel_epi64(dst.offset(i as isize * stride) as *mut __m128i, d[i]);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn highbd_dr_prediction_z3_8x4_avx2(dst: *mut u16, stride: isize, left: *const u16,
                                           upsample_left: i32, dy: i32, bd: i32) {
    let mut dstvec: [__m128i; 8] = core::mem::zeroed();
    let mut d: [__m128i; 4] = core::mem::zeroed();
    if bd < 12 {
        highbd_dr_prediction_z1_4xN_internal_avx2(8, dstvec.as_mut_ptr(), left, upsample_left, dy);
    } else {
        highbd_dr_prediction_32bit_z1_4xN_internal_avx2(8, dstvec.as_mut_ptr(), left, upsample_left,
                                                        dy);
    }

    let v = dstvec.as_ptr();
    let o = d.as_mut_ptr();
    highbd_transpose8x8_low_sse2(v.add(0), v.add(1), v.add(2), v.add(3), v.add(4), v.add(5),
                                 v.add(6), v.add(7), o.add(0), o.add(1), o.add(2), o.add(3));
    _mm_storeu_si128(dst.offset(0 * stride) as *mut __m128i, d[0]);
    _mm_storeu_si128(dst.offset(1 * stride) as *mut __m128i, d[1]);
    _mm_storeu_si128(dst.offset(2 * stride) as *mut __m128i, d[2]);
    _mm_storeu_si128(dst.offset(3 * stride) as *mut __m128i, d[3]);
}

#[target_feature(enable = "avx2")]
unsafe fn highbd_dr_prediction_z3_8x16_avx2(dst: *mut u16, stride: isize, left: *const u16,
                                            upsample_left: i32, dy: i32, bd: i32) {
    let mut dstvec: [__m256i; 8] = core::mem::zeroed();
    let mut d: [__m256i; 8] = core::mem::zeroed();
    if bd < 12 {
        highbd_dr_prediction_z1_16xN_internal_avx2(8, dstvec.as_mut_ptr(), left, upsample_left, dy);
    } else {
        highbd_dr_prediction_32bit_z1_16xN_internal_avx2(8, dstvec.as_mut_ptr(), left, upsample_left,
                                                         dy);
    }
    highbd_transpose8x16_16x8_avx2(dstvec.as_ptr(), d.as_mut_ptr());
    for i in 0..8 {
        _mm_storeu_si128(dst.offset(i as isize * stride) as *mut __m128i,
                         _mm256_castsi256_si128(d[i]));
    }
    for i in 8..16 {
        _mm_storeu_si128(dst.offset(i as isize * stride) as *mut __m128i,
                         _mm256_extracti128_si256::<1>(d[i - 8]));
    }
}

#[target_feature(enable = "avx2")]
unsafe fn highbd_dr_prediction_z3_16x8_avx2(dst: *mut u16, stride: isize, left: *const u16,
                                            upsample_left: i32, dy: i32, bd: i32) {
    let mut dstvec: [__m128i; 16] = core::mem::zeroed();
    let mut d: [__m128i; 16] = core::mem::zeroed();
    if bd < 12 {
        highbd_dr_prediction_z1_8xN_internal_avx2(16, dstvec.as_mut_ptr(), left, upsample_left, dy);
    } else {
        highbd_dr_prediction_32bit_z1_8xN_internal_avx2(16, dstvec.as_mut_ptr(), left, upsample_left,
                                                        dy);
    }
    let v = dstvec.as_ptr();
    let o = d.as_mut_ptr();
    let mut i = 0;
    while i < 16 {
        highbd_transpose8x8_sse2(v.add(0 + i), v.add(1 + i), v.add(2 + i), v.add(3 + i),
                                 v.add(4 + i), v.add(5 + i), v.add(6 + i), v.add(7 + i),
                                 o.add(0 + i), o.add(1 + i), o.add(2 + i), o.add(3 + i),
                                 o.add(4 + i), o.add(5 + i), o.add(6 + i), o.add(7 + i));
        i += 8;
    }
    for i in 0..8 {
        _mm_storeu_si128(dst.offset(i as isize * stride) as *mut __m128i, d[i]);
        _mm_storeu_si128(dst.offset(i as isize * stride + 8) as *mut __m128i, d[i + 8]);
    }
}

// !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER
#[target_feature(enable = "avx2")]
unsafe fn highbd_dr_prediction_z3_4x16_avx2(dst: *mut u16, stride: isize, left: *const u16,
                                            upsample_left: i32, dy: i32, bd: i32) {
    let mut dstvec: [__m256i; 4] = core::mem::zeroed();
    let mut d: [__m256i; 4] = core::mem::zeroed();
    let mut d1: __m256i;
    if bd < 12 {
        highbd_dr_prediction_z1_16xN_internal_avx2(4, dstvec.as_mut_ptr(), left, upsample_left, dy);
    } else {
        highbd_dr_prediction_32bit_z1_16xN_internal_avx2(4, dstvec.as_mut_ptr(), left, upsample_left,
                                                         dy);
    }
    highbd_transpose4x16_avx2(dstvec.as_ptr(), d.as_mut_ptr());
    for i in 0..4 {
        _mm_storel_epi64(dst.offset(i as isize * stride) as *mut __m128i,
                         _mm256_castsi256_si128(d[i]));
        d1 = _mm256_bsrli_epi128::<8>(d[i]);
        _mm_storel_epi64(dst.offset((i as isize + 4) * stride) as *mut __m128i,
                         _mm256_castsi256_si128(d1));
        _mm_storel_epi64(dst.offset((i as isize + 8) * stride) as *mut __m128i,
                         _mm256_extracti128_si256::<1>(d[i]));
        _mm_storel_epi64(dst.offset((i as isize + 12) * stride) as *mut __m128i,
                         _mm256_extracti128_si256::<1>(d1));
    }
}

#[target_feature(enable = "avx2")]
unsafe fn highbd_dr_prediction_z3_16x4_avx2(dst: *mut u16, stride: isize, left: *const u16,
                                            upsample_left: i32, dy: i32, bd: i32) {
    let mut dstvec: [__m128i; 16] = core::mem::zeroed();
    let mut d: [__m128i; 8] = core::mem::zeroed();
    if bd < 12 {
        highbd_dr_prediction_z1_4xN_internal_avx2(16, dstvec.as_mut_ptr(), left, upsample_left, dy);
    } else {
        highbd_dr_prediction_32bit_z1_4xN_internal_avx2(16, dstvec.as_mut_ptr(), left, upsample_left,
                                                        dy);
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
unsafe fn highbd_dr_prediction_z3_8x32_avx2(dst: *mut u16, stride: isize, left: *const u16,
                                            upsample_left: i32, dy: i32, bd: i32) {
    let mut dstvec: [__m256i; 16] = core::mem::zeroed();
    let mut d: [__m256i; 16] = core::mem::zeroed();
    if bd < 12 {
        highbd_dr_prediction_z1_32xN_internal_avx2(8, dstvec.as_mut_ptr(), left, upsample_left, dy);
    } else {
        highbd_dr_prediction_32bit_z1_32xN_internal_avx2(8, dstvec.as_mut_ptr(), left, upsample_left,
                                                         dy);
    }

    let mut i = 0;
    while i < 16 {
        highbd_transpose8x16_16x8_avx2(dstvec.as_ptr().add(i), d.as_mut_ptr().add(i));
        i += 8;
    }

    for i in 0..8 {
        _mm_storeu_si128(dst.offset(i as isize * stride) as *mut __m128i,
                         _mm256_castsi256_si128(d[i]));
    }
    for i in 0..8 {
        _mm_storeu_si128(dst.offset((i as isize + 8) * stride) as *mut __m128i,
                         _mm256_extracti128_si256::<1>(d[i]));
    }
    for i in 8..16 {
        _mm_storeu_si128(dst.offset((i as isize + 8) * stride) as *mut __m128i,
                         _mm256_castsi256_si128(d[i]));
    }
    for i in 8..16 {
        _mm_storeu_si128(dst.offset((i as isize + 16) * stride) as *mut __m128i,
                         _mm256_extracti128_si256::<1>(d[i]));
    }
}

#[target_feature(enable = "avx2")]
unsafe fn highbd_dr_prediction_z3_32x8_avx2(dst: *mut u16, stride: isize, left: *const u16,
                                            upsample_left: i32, dy: i32, bd: i32) {
    let mut dstvec: [__m128i; 32] = core::mem::zeroed();
    let mut d: [__m128i; 32] = core::mem::zeroed();
    if bd < 12 {
        highbd_dr_prediction_z1_8xN_internal_avx2(32, dstvec.as_mut_ptr(), left, upsample_left, dy);
    } else {
        highbd_dr_prediction_32bit_z1_8xN_internal_avx2(32, dstvec.as_mut_ptr(), left, upsample_left,
                                                        dy);
    }

    let v = dstvec.as_ptr();
    let o = d.as_mut_ptr();
    let mut i = 0;
    while i < 32 {
        highbd_transpose8x8_sse2(v.add(0 + i), v.add(1 + i), v.add(2 + i), v.add(3 + i),
                                 v.add(4 + i), v.add(5 + i), v.add(6 + i), v.add(7 + i),
                                 o.add(0 + i), o.add(1 + i), o.add(2 + i), o.add(3 + i),
                                 o.add(4 + i), o.add(5 + i), o.add(6 + i), o.add(7 + i));
        i += 8;
    }
    for i in 0..8 {
        _mm_storeu_si128(dst.offset(i as isize * stride) as *mut __m128i, d[i]);
        _mm_storeu_si128(dst.offset(i as isize * stride + 8) as *mut __m128i, d[i + 8]);
        _mm_storeu_si128(dst.offset(i as isize * stride + 16) as *mut __m128i, d[i + 16]);
        _mm_storeu_si128(dst.offset(i as isize * stride + 24) as *mut __m128i, d[i + 24]);
    }
}
// !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER

#[target_feature(enable = "avx2")]
unsafe fn highbd_dr_prediction_z3_16x16_avx2(dst: *mut u16, stride: isize, left: *const u16,
                                             upsample_left: i32, dy: i32, bd: i32) {
    let mut dstvec: [__m256i; 16] = core::mem::zeroed();
    let mut d: [__m256i; 16] = core::mem::zeroed();
    if bd < 12 {
        highbd_dr_prediction_z1_16xN_internal_avx2(16, dstvec.as_mut_ptr(), left, upsample_left, dy);
    } else {
        highbd_dr_prediction_32bit_z1_16xN_internal_avx2(16, dstvec.as_mut_ptr(), left,
                                                         upsample_left, dy);
    }

    highbd_transpose16x16_avx2(dstvec.as_ptr(), d.as_mut_ptr());

    for i in 0..16 {
        _mm256_storeu_si256(dst.offset(i as isize * stride) as *mut __m256i, d[i]);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn highbd_dr_prediction_z3_32x32_avx2(dst: *mut u16, stride: isize, left: *const u16,
                                             upsample_left: i32, dy: i32, bd: i32) {
    let mut dstvec: [__m256i; 64] = core::mem::zeroed();
    let mut d: [__m256i; 16] = core::mem::zeroed();
    if bd < 12 {
        highbd_dr_prediction_z1_32xN_internal_avx2(32, dstvec.as_mut_ptr(), left, upsample_left, dy);
    } else {
        highbd_dr_prediction_32bit_z1_32xN_internal_avx2(32, dstvec.as_mut_ptr(), left,
                                                         upsample_left, dy);
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
        _mm256_storeu_si256(dst.offset((j as isize + 16) * stride + 16) as *mut __m256i, d[j]);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn highbd_dr_prediction_z3_64x64_avx2(dst: *mut u16, stride: isize, left: *const u16,
                                             upsample_left: i32, dy: i32, bd: i32) {
    let mut dstT: Aligned16<[u16; 64 * 64]> = core::mem::zeroed();
    if bd < 12 {
        highbd_dr_prediction_z1_64xN_avx2(64, dstT.0.as_mut_ptr(), 64, left, upsample_left, dy);
    } else {
        highbd_dr_prediction_32bit_z1_64xN_avx2(64, dstT.0.as_mut_ptr(), 64, left, upsample_left, dy);
    }
    highbd_transpose(dstT.0.as_ptr(), 64, dst, stride, 64, 64);
}

#[target_feature(enable = "avx2")]
unsafe fn highbd_dr_prediction_z3_16x32_avx2(dst: *mut u16, stride: isize, left: *const u16,
                                             upsample_left: i32, dy: i32, bd: i32) {
    let mut dstvec: [__m256i; 32] = core::mem::zeroed();
    let mut d: [__m256i; 32] = core::mem::zeroed();
    if bd < 12 {
        highbd_dr_prediction_z1_32xN_internal_avx2(16, dstvec.as_mut_ptr(), left, upsample_left, dy);
    } else {
        highbd_dr_prediction_32bit_z1_32xN_internal_avx2(16, dstvec.as_mut_ptr(), left,
                                                         upsample_left, dy);
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
            _mm_storeu_si128(dst.offset((i + j) as isize * stride) as *mut __m128i,
                             _mm256_castsi256_si128(d[i + j]));
        }
        for i in 0..8 {
            _mm_storeu_si128(dst.offset((i + j) as isize * stride + 8) as *mut __m128i,
                             _mm256_castsi256_si128(d[(i + j) + 8]));
        }
        for i in 8..16 {
            _mm256_storeu_si256(dst.offset((i + j) as isize * stride) as *mut __m256i,
                                _mm256_inserti128_si256::<0>(
                                    d[i + j], _mm256_extracti128_si256::<1>(d[(i + j) - 8])));
        }
        j += 16;
    }
}

#[target_feature(enable = "avx2")]
unsafe fn highbd_dr_prediction_z3_32x16_avx2(dst: *mut u16, stride: isize, left: *const u16,
                                             upsample_left: i32, dy: i32, bd: i32) {
    let mut dstvec: [__m256i; 32] = core::mem::zeroed();
    let mut d: [__m256i; 16] = core::mem::zeroed();
    if bd < 12 {
        highbd_dr_prediction_z1_16xN_internal_avx2(32, dstvec.as_mut_ptr(), left, upsample_left, dy);
    } else {
        highbd_dr_prediction_32bit_z1_16xN_internal_avx2(32, dstvec.as_mut_ptr(), left,
                                                         upsample_left, dy);
    }
    let mut i = 0;
    while i < 32 {
        highbd_transpose16x16_avx2(dstvec.as_ptr().add(i), d.as_mut_ptr());
        for j in 0..16 {
            _mm256_storeu_si256(dst.offset(j as isize * stride + i as isize) as *mut __m256i, d[j]);
        }
        i += 16;
    }
}

#[target_feature(enable = "avx2")]
unsafe fn highbd_dr_prediction_z3_32x64_avx2(dst: *mut u16, stride: isize, left: *const u16,
                                             upsample_left: i32, dy: i32, bd: i32) {
    let mut dstT: [u16; 64 * 32] = core::mem::zeroed();
    if bd < 12 {
        highbd_dr_prediction_z1_64xN_avx2(32, dstT.as_mut_ptr(), 64, left, upsample_left, dy);
    } else {
        highbd_dr_prediction_32bit_z1_64xN_avx2(32, dstT.as_mut_ptr(), 64, left, upsample_left, dy);
    }
    highbd_transpose(dstT.as_ptr(), 64, dst, stride, 32, 64);
}

#[target_feature(enable = "avx2")]
unsafe fn highbd_dr_prediction_z3_64x32_avx2(dst: *mut u16, stride: isize, left: *const u16,
                                             upsample_left: i32, dy: i32, bd: i32) {
    let mut dstT: Aligned16<[u16; 32 * 64]> = core::mem::zeroed();
    highbd_dr_prediction_z1_32xN_avx2(64, dstT.0.as_mut_ptr(), 32, left, upsample_left, dy, bd);
    highbd_transpose(dstT.0.as_ptr(), 32, dst, stride, 64, 32);
}

// !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER
#[target_feature(enable = "avx2")]
unsafe fn highbd_dr_prediction_z3_16x64_avx2(dst: *mut u16, stride: isize, left: *const u16,
                                             upsample_left: i32, dy: i32, bd: i32) {
    let mut dstT: Aligned16<[u16; 64 * 16]> = core::mem::zeroed();
    if bd < 12 {
        highbd_dr_prediction_z1_64xN_avx2(16, dstT.0.as_mut_ptr(), 64, left, upsample_left, dy);
    } else {
        highbd_dr_prediction_32bit_z1_64xN_avx2(16, dstT.0.as_mut_ptr(), 64, left, upsample_left, dy);
    }
    highbd_transpose(dstT.0.as_ptr(), 64, dst, stride, 16, 64);
}

#[target_feature(enable = "avx2")]
unsafe fn highbd_dr_prediction_z3_64x16_avx2(dst: *mut u16, stride: isize, left: *const u16,
                                             upsample_left: i32, dy: i32, bd: i32) {
    let mut dstvec: [__m256i; 64] = core::mem::zeroed();
    let mut d: [__m256i; 16] = core::mem::zeroed();
    if bd < 12 {
        highbd_dr_prediction_z1_16xN_internal_avx2(64, dstvec.as_mut_ptr(), left, upsample_left, dy);
    } else {
        highbd_dr_prediction_32bit_z1_16xN_internal_avx2(64, dstvec.as_mut_ptr(), left,
                                                         upsample_left, dy);
    }
    let mut i = 0;
    while i < 64 {
        highbd_transpose16x16_avx2(dstvec.as_ptr().add(i), d.as_mut_ptr());
        for j in 0..16 {
            _mm256_storeu_si256(dst.offset(j as isize * stride + i as isize) as *mut __m256i, d[j]);
        }
        i += 16;
    }
}
// !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn av1_highbd_dr_prediction_z3_avx2(dst: *mut u16, stride: isize, bw: i32,
                                                                 bh: i32, above: *const u16,
                                                                 left: *const u16,
                                                                 upsample_left: i32, dx: i32,
                                                                 dy: i32, bd: i32) {
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
                    8 => highbd_dr_prediction_z3_8x16_avx2(dst, stride, left, upsample_left, dy, bd),
                    16 => highbd_dr_prediction_z3_16x32_avx2(dst, stride, left, upsample_left, dy, bd),
                    32 => highbd_dr_prediction_z3_32x64_avx2(dst, stride, left, upsample_left, dy, bd),
                    _ => {}
                }
            } else {
                match bw {
                    // !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER
                    4 => highbd_dr_prediction_z3_4x16_avx2(dst, stride, left, upsample_left, dy, bd),
                    8 => highbd_dr_prediction_z3_8x32_avx2(dst, stride, left, upsample_left, dy, bd),
                    16 => highbd_dr_prediction_z3_16x64_avx2(dst, stride, left, upsample_left, dy, bd),
                    _ => {}
                }
            }
        } else {
            if bh + bh == bw {
                match bh {
                    4 => highbd_dr_prediction_z3_8x4_avx2(dst, stride, left, upsample_left, dy, bd),
                    8 => highbd_dr_prediction_z3_16x8_avx2(dst, stride, left, upsample_left, dy, bd),
                    16 => highbd_dr_prediction_z3_32x16_avx2(dst, stride, left, upsample_left, dy, bd),
                    32 => highbd_dr_prediction_z3_64x32_avx2(dst, stride, left, upsample_left, dy, bd),
                    _ => {}
                }
            } else {
                match bh {
                    // !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER
                    4 => highbd_dr_prediction_z3_16x4_avx2(dst, stride, left, upsample_left, dy, bd),
                    8 => highbd_dr_prediction_z3_32x8_avx2(dst, stride, left, upsample_left, dy, bd),
                    16 => highbd_dr_prediction_z3_64x16_avx2(dst, stride, left, upsample_left, dy, bd),
                    _ => {}
                }
            }
        }
    }
}
