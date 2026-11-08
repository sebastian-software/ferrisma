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

// Translated by hand from upstream/aom/aom_dsp/x86/transpose_sse2.h (ADR 0056,
// ADR 0058). Names, order, comments, and intrinsic sequences follow the
// original; every helper carries the smallest target feature its own
// intrinsics need (SSE2). Array parameters stay raw pointers (`in[i]` ->
// `*in_.add(i)`, `in + i` -> `in_.add(i)`), so the in-place calls of
// transpose_16bit_16x16 keep the C aliasing: each 8x8 transpose reads all of
// its input rows before it writes any output row.

use core::arch::x86_64::*;

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn transpose_8bit_4x4(in_: *const __m128i) -> __m128i {
    // Unpack 8 bit elements. Goes from:
    // in[0]: 00 01 02 03
    // in[1]: 10 11 12 13
    // in[2]: 20 21 22 23
    // in[3]: 30 31 32 33
    // to:
    // a0:    00 10 01 11  02 12 03 13
    // a1:    20 30 21 31  22 32 23 33
    let a0 = _mm_unpacklo_epi8(*in_.add(0), *in_.add(1));
    let a1 = _mm_unpacklo_epi8(*in_.add(2), *in_.add(3));

    // Unpack 16 bit elements resulting in:
    // 00 10 20 30  01 11 21 31  02 12 22 32  03 13 23 33
    _mm_unpacklo_epi16(a0, a1)
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn transpose_8bit_8x8(in_: *const __m128i, out: *mut __m128i) {
    // Unpack 8 bit elements. Goes from:
    // in[0]: 00 01 02 03 04 05 06 07
    // in[1]: 10 11 12 13 14 15 16 17
    // in[2]: 20 21 22 23 24 25 26 27
    // in[3]: 30 31 32 33 34 35 36 37
    // in[4]: 40 41 42 43 44 45 46 47
    // in[5]: 50 51 52 53 54 55 56 57
    // in[6]: 60 61 62 63 64 65 66 67
    // in[7]: 70 71 72 73 74 75 76 77
    // to:
    // a0:    00 10 01 11 02 12 03 13  04 14 05 15 06 16 07 17
    // a1:    20 30 21 31 22 32 23 33  24 34 25 35 26 36 27 37
    // a2:    40 50 41 51 42 52 43 53  44 54 45 55 46 56 47 57
    // a3:    60 70 61 71 62 72 63 73  64 74 65 75 66 76 67 77
    let a0 = _mm_unpacklo_epi8(*in_.add(0), *in_.add(1));
    let a1 = _mm_unpacklo_epi8(*in_.add(2), *in_.add(3));
    let a2 = _mm_unpacklo_epi8(*in_.add(4), *in_.add(5));
    let a3 = _mm_unpacklo_epi8(*in_.add(6), *in_.add(7));

    // Unpack 16 bit elements resulting in:
    // b0: 00 10 20 30 01 11 21 31  02 12 22 32 03 13 23 33
    // b1: 40 50 60 70 41 51 61 71  42 52 62 72 43 53 63 73
    // b2: 04 14 24 34 05 15 25 35  06 16 26 36 07 17 27 37
    // b3: 44 54 64 74 45 55 65 75  46 56 66 76 47 57 67 77
    let b0 = _mm_unpacklo_epi16(a0, a1);
    let b1 = _mm_unpackhi_epi16(a0, a1);
    let b2 = _mm_unpacklo_epi16(a2, a3);
    let b3 = _mm_unpackhi_epi16(a2, a3);

    // Unpack 32 bit elements resulting in:
    // c0: 00 10 20 30 40 50 60 70  01 11 21 31 41 51 61 71
    // c1: 02 12 22 32 42 52 62 72  03 13 23 33 43 53 63 73
    // c2: 04 14 24 34 44 54 64 74  05 15 25 35 45 55 65 75
    // c3: 06 16 26 36 46 56 66 76  07 17 27 37 47 57 67 77
    let c0 = _mm_unpacklo_epi32(b0, b2);
    let c1 = _mm_unpackhi_epi32(b0, b2);
    let c2 = _mm_unpacklo_epi32(b1, b3);
    let c3 = _mm_unpackhi_epi32(b1, b3);

    // Unpack 64 bit elements resulting in:
    // out[0]: 00 10 20 30 40 50 60 70
    // out[1]: 01 11 21 31 41 51 61 71
    // out[2]: 02 12 22 32 42 52 62 72
    // out[3]: 03 13 23 33 43 53 63 73
    // out[4]: 04 14 24 34 44 54 64 74
    // out[5]: 05 15 25 35 45 55 65 75
    // out[6]: 06 16 26 36 46 56 66 76
    // out[7]: 07 17 27 37 47 57 67 77
    *out.add(0) = _mm_unpacklo_epi64(c0, c0);
    *out.add(1) = _mm_unpackhi_epi64(c0, c0);
    *out.add(2) = _mm_unpacklo_epi64(c1, c1);
    *out.add(3) = _mm_unpackhi_epi64(c1, c1);
    *out.add(4) = _mm_unpacklo_epi64(c2, c2);
    *out.add(5) = _mm_unpackhi_epi64(c2, c2);
    *out.add(6) = _mm_unpacklo_epi64(c3, c3);
    *out.add(7) = _mm_unpackhi_epi64(c3, c3);
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn transpose_16bit_4x4(in_: *const __m128i, out: *mut __m128i) {
    // Unpack 16 bit elements. Goes from:
    // in[0]: 00 01 02 03  XX XX XX XX
    // in[1]: 10 11 12 13  XX XX XX XX
    // in[2]: 20 21 22 23  XX XX XX XX
    // in[3]: 30 31 32 33  XX XX XX XX
    // to:
    // a0:    00 10 01 11  02 12 03 13
    // a1:    20 30 21 31  22 32 23 33
    let a0 = _mm_unpacklo_epi16(*in_.add(0), *in_.add(1));
    let a1 = _mm_unpacklo_epi16(*in_.add(2), *in_.add(3));

    // Unpack 32 bit elements resulting in:
    // out[0]: 00 10 20 30  01 11 21 31
    // out[1]: 01 11 21 31  __ __ __ __
    // out[2]: 02 12 22 32  03 13 23 33
    // out[3]: 03 13 23 33  __ __ __ __
    //
    // Note: The high 64 bits of the output registers are shown for informational
    // purposes only. Callers should only use the low 64 bits of the output
    // registers. "__" indicates zeros.
    *out.add(0) = _mm_unpacklo_epi32(a0, a1);
    *out.add(1) = _mm_srli_si128::<8>(*out.add(0));
    *out.add(2) = _mm_unpackhi_epi32(a0, a1);
    *out.add(3) = _mm_srli_si128::<8>(*out.add(2));
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn transpose_16bit_4x8(in_: *const __m128i, out: *mut __m128i) {
    // Unpack 16 bit elements. Goes from:
    // in[0]: 00 01 02 03  XX XX XX XX
    // in[1]: 10 11 12 13  XX XX XX XX
    // in[2]: 20 21 22 23  XX XX XX XX
    // in[3]: 30 31 32 33  XX XX XX XX
    // in[4]: 40 41 42 43  XX XX XX XX
    // in[5]: 50 51 52 53  XX XX XX XX
    // in[6]: 60 61 62 63  XX XX XX XX
    // in[7]: 70 71 72 73  XX XX XX XX
    // to:
    // a0:    00 10 01 11  02 12 03 13
    // a1:    20 30 21 31  22 32 23 33
    // a2:    40 50 41 51  42 52 43 53
    // a3:    60 70 61 71  62 72 63 73
    let a0 = _mm_unpacklo_epi16(*in_.add(0), *in_.add(1));
    let a1 = _mm_unpacklo_epi16(*in_.add(2), *in_.add(3));
    let a2 = _mm_unpacklo_epi16(*in_.add(4), *in_.add(5));
    let a3 = _mm_unpacklo_epi16(*in_.add(6), *in_.add(7));

    // Unpack 32 bit elements resulting in:
    // b0: 00 10 20 30  01 11 21 31
    // b1: 40 50 60 70  41 51 61 71
    // b2: 02 12 22 32  03 13 23 33
    // b3: 42 52 62 72  43 53 63 73
    let b0 = _mm_unpacklo_epi32(a0, a1);
    let b1 = _mm_unpacklo_epi32(a2, a3);
    let b2 = _mm_unpackhi_epi32(a0, a1);
    let b3 = _mm_unpackhi_epi32(a2, a3);

    // Unpack 64 bit elements resulting in:
    // out[0]: 00 10 20 30  40 50 60 70
    // out[1]: 01 11 21 31  41 51 61 71
    // out[2]: 02 12 22 32  42 52 62 72
    // out[3]: 03 13 23 33  43 53 63 73
    *out.add(0) = _mm_unpacklo_epi64(b0, b1);
    *out.add(1) = _mm_unpackhi_epi64(b0, b1);
    *out.add(2) = _mm_unpacklo_epi64(b2, b3);
    *out.add(3) = _mm_unpackhi_epi64(b2, b3);
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn transpose_16bit_8x4(in_: *const __m128i, out: *mut __m128i) {
    // Unpack 16 bit elements. Goes from:
    // in[0]: 00 01 02 03  04 05 06 07
    // in[1]: 10 11 12 13  14 15 16 17
    // in[2]: 20 21 22 23  24 25 26 27
    // in[3]: 30 31 32 33  34 35 36 37

    // to:
    // a0:    00 10 01 11  02 12 03 13
    // a1:    20 30 21 31  22 32 23 33
    // a4:    04 14 05 15  06 16 07 17
    // a5:    24 34 25 35  26 36 27 37
    let a0 = _mm_unpacklo_epi16(*in_.add(0), *in_.add(1));
    let a1 = _mm_unpacklo_epi16(*in_.add(2), *in_.add(3));
    let a4 = _mm_unpackhi_epi16(*in_.add(0), *in_.add(1));
    let a5 = _mm_unpackhi_epi16(*in_.add(2), *in_.add(3));

    // Unpack 32 bit elements resulting in:
    // b0: 00 10 20 30  01 11 21 31
    // b2: 04 14 24 34  05 15 25 35
    // b4: 02 12 22 32  03 13 23 33
    // b6: 06 16 26 36  07 17 27 37
    let b0 = _mm_unpacklo_epi32(a0, a1);
    let b2 = _mm_unpacklo_epi32(a4, a5);
    let b4 = _mm_unpackhi_epi32(a0, a1);
    let b6 = _mm_unpackhi_epi32(a4, a5);

    // Unpack 64 bit elements resulting in:
    // out[0]: 00 10 20 30  XX XX XX XX
    // out[1]: 01 11 21 31  XX XX XX XX
    // out[2]: 02 12 22 32  XX XX XX XX
    // out[3]: 03 13 23 33  XX XX XX XX
    // out[4]: 04 14 24 34  XX XX XX XX
    // out[5]: 05 15 25 35  XX XX XX XX
    // out[6]: 06 16 26 36  XX XX XX XX
    // out[7]: 07 17 27 37  XX XX XX XX
    let zeros = _mm_setzero_si128();
    *out.add(0) = _mm_unpacklo_epi64(b0, zeros);
    *out.add(1) = _mm_unpackhi_epi64(b0, zeros);
    *out.add(2) = _mm_unpacklo_epi64(b4, zeros);
    *out.add(3) = _mm_unpackhi_epi64(b4, zeros);
    *out.add(4) = _mm_unpacklo_epi64(b2, zeros);
    *out.add(5) = _mm_unpackhi_epi64(b2, zeros);
    *out.add(6) = _mm_unpacklo_epi64(b6, zeros);
    *out.add(7) = _mm_unpackhi_epi64(b6, zeros);
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn transpose_16bit_8x8(in_: *const __m128i, out: *mut __m128i) {
    // Unpack 16 bit elements. Goes from:
    // in[0]: 00 01 02 03  04 05 06 07
    // in[1]: 10 11 12 13  14 15 16 17
    // in[2]: 20 21 22 23  24 25 26 27
    // in[3]: 30 31 32 33  34 35 36 37
    // in[4]: 40 41 42 43  44 45 46 47
    // in[5]: 50 51 52 53  54 55 56 57
    // in[6]: 60 61 62 63  64 65 66 67
    // in[7]: 70 71 72 73  74 75 76 77
    // to:
    // a0:    00 10 01 11  02 12 03 13
    // a1:    20 30 21 31  22 32 23 33
    // a2:    40 50 41 51  42 52 43 53
    // a3:    60 70 61 71  62 72 63 73
    // a4:    04 14 05 15  06 16 07 17
    // a5:    24 34 25 35  26 36 27 37
    // a6:    44 54 45 55  46 56 47 57
    // a7:    64 74 65 75  66 76 67 77
    let a0 = _mm_unpacklo_epi16(*in_.add(0), *in_.add(1));
    let a1 = _mm_unpacklo_epi16(*in_.add(2), *in_.add(3));
    let a2 = _mm_unpacklo_epi16(*in_.add(4), *in_.add(5));
    let a3 = _mm_unpacklo_epi16(*in_.add(6), *in_.add(7));
    let a4 = _mm_unpackhi_epi16(*in_.add(0), *in_.add(1));
    let a5 = _mm_unpackhi_epi16(*in_.add(2), *in_.add(3));
    let a6 = _mm_unpackhi_epi16(*in_.add(4), *in_.add(5));
    let a7 = _mm_unpackhi_epi16(*in_.add(6), *in_.add(7));

    // Unpack 32 bit elements resulting in:
    // b0: 00 10 20 30  01 11 21 31
    // b1: 40 50 60 70  41 51 61 71
    // b2: 04 14 24 34  05 15 25 35
    // b3: 44 54 64 74  45 55 65 75
    // b4: 02 12 22 32  03 13 23 33
    // b5: 42 52 62 72  43 53 63 73
    // b6: 06 16 26 36  07 17 27 37
    // b7: 46 56 66 76  47 57 67 77
    let b0 = _mm_unpacklo_epi32(a0, a1);
    let b1 = _mm_unpacklo_epi32(a2, a3);
    let b2 = _mm_unpacklo_epi32(a4, a5);
    let b3 = _mm_unpacklo_epi32(a6, a7);
    let b4 = _mm_unpackhi_epi32(a0, a1);
    let b5 = _mm_unpackhi_epi32(a2, a3);
    let b6 = _mm_unpackhi_epi32(a4, a5);
    let b7 = _mm_unpackhi_epi32(a6, a7);

    // Unpack 64 bit elements resulting in:
    // out[0]: 00 10 20 30  40 50 60 70
    // out[1]: 01 11 21 31  41 51 61 71
    // out[2]: 02 12 22 32  42 52 62 72
    // out[3]: 03 13 23 33  43 53 63 73
    // out[4]: 04 14 24 34  44 54 64 74
    // out[5]: 05 15 25 35  45 55 65 75
    // out[6]: 06 16 26 36  46 56 66 76
    // out[7]: 07 17 27 37  47 57 67 77
    *out.add(0) = _mm_unpacklo_epi64(b0, b1);
    *out.add(1) = _mm_unpackhi_epi64(b0, b1);
    *out.add(2) = _mm_unpacklo_epi64(b4, b5);
    *out.add(3) = _mm_unpackhi_epi64(b4, b5);
    *out.add(4) = _mm_unpacklo_epi64(b2, b3);
    *out.add(5) = _mm_unpackhi_epi64(b2, b3);
    *out.add(6) = _mm_unpacklo_epi64(b6, b7);
    *out.add(7) = _mm_unpackhi_epi64(b6, b7);
}

// Transpose in-place
#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn transpose_16bit_16x16(left: *mut __m128i, right: *mut __m128i) {
    let mut tbuf = [_mm_setzero_si128(); 8];
    transpose_16bit_8x8(left, left);
    transpose_16bit_8x8(right, tbuf.as_mut_ptr());
    transpose_16bit_8x8(left.add(8), right);
    transpose_16bit_8x8(right.add(8), right.add(8));

    *left.add(8) = tbuf[0];
    *left.add(9) = tbuf[1];
    *left.add(10) = tbuf[2];
    *left.add(11) = tbuf[3];
    *left.add(12) = tbuf[4];
    *left.add(13) = tbuf[5];
    *left.add(14) = tbuf[6];
    *left.add(15) = tbuf[7];
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn transpose_32bit_4x4(in_: *const __m128i, out: *mut __m128i) {
    // Unpack 32 bit elements. Goes from:
    // in[0]: 00 01 02 03
    // in[1]: 10 11 12 13
    // in[2]: 20 21 22 23
    // in[3]: 30 31 32 33
    // to:
    // a0:    00 10 01 11
    // a1:    20 30 21 31
    // a2:    02 12 03 13
    // a3:    22 32 23 33

    let a0 = _mm_unpacklo_epi32(*in_.add(0), *in_.add(1));
    let a1 = _mm_unpacklo_epi32(*in_.add(2), *in_.add(3));
    let a2 = _mm_unpackhi_epi32(*in_.add(0), *in_.add(1));
    let a3 = _mm_unpackhi_epi32(*in_.add(2), *in_.add(3));

    // Unpack 64 bit elements resulting in:
    // out[0]: 00 10 20 30
    // out[1]: 01 11 21 31
    // out[2]: 02 12 22 32
    // out[3]: 03 13 23 33
    *out.add(0) = _mm_unpacklo_epi64(a0, a1);
    *out.add(1) = _mm_unpackhi_epi64(a0, a1);
    *out.add(2) = _mm_unpacklo_epi64(a2, a3);
    *out.add(3) = _mm_unpackhi_epi64(a2, a3);
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn transpose_32bit_4x4x2(in_: *const __m128i, out: *mut __m128i) {
    // Unpack 32 bit elements. Goes from:
    // in[0]: 00 01 02 03
    // in[1]: 10 11 12 13
    // in[2]: 20 21 22 23
    // in[3]: 30 31 32 33
    // in[4]: 04 05 06 07
    // in[5]: 14 15 16 17
    // in[6]: 24 25 26 27
    // in[7]: 34 35 36 37
    // to:
    // a0:    00 10 01 11
    // a1:    20 30 21 31
    // a2:    02 12 03 13
    // a3:    22 32 23 33
    // a4:    04 14 05 15
    // a5:    24 34 25 35
    // a6:    06 16 07 17
    // a7:    26 36 27 37
    let a0 = _mm_unpacklo_epi32(*in_.add(0), *in_.add(1));
    let a1 = _mm_unpacklo_epi32(*in_.add(2), *in_.add(3));
    let a2 = _mm_unpackhi_epi32(*in_.add(0), *in_.add(1));
    let a3 = _mm_unpackhi_epi32(*in_.add(2), *in_.add(3));
    let a4 = _mm_unpacklo_epi32(*in_.add(4), *in_.add(5));
    let a5 = _mm_unpacklo_epi32(*in_.add(6), *in_.add(7));
    let a6 = _mm_unpackhi_epi32(*in_.add(4), *in_.add(5));
    let a7 = _mm_unpackhi_epi32(*in_.add(6), *in_.add(7));

    // Unpack 64 bit elements resulting in:
    // out[0]: 00 10 20 30
    // out[1]: 01 11 21 31
    // out[2]: 02 12 22 32
    // out[3]: 03 13 23 33
    // out[4]: 04 14 24 34
    // out[5]: 05 15 25 35
    // out[6]: 06 16 26 36
    // out[7]: 07 17 27 37
    *out.add(0) = _mm_unpacklo_epi64(a0, a1);
    *out.add(1) = _mm_unpackhi_epi64(a0, a1);
    *out.add(2) = _mm_unpacklo_epi64(a2, a3);
    *out.add(3) = _mm_unpackhi_epi64(a2, a3);
    *out.add(4) = _mm_unpacklo_epi64(a4, a5);
    *out.add(5) = _mm_unpackhi_epi64(a4, a5);
    *out.add(6) = _mm_unpacklo_epi64(a6, a7);
    *out.add(7) = _mm_unpackhi_epi64(a6, a7);
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn transpose_32bit_8x4(in_: *const __m128i, out: *mut __m128i) {
    // Unpack 32 bit elements. Goes from:
    // in[0]: 00 01 02 03
    // in[1]: 04 05 06 07
    // in[2]: 10 11 12 13
    // in[3]: 14 15 16 17
    // in[4]: 20 21 22 23
    // in[5]: 24 25 26 27
    // in[6]: 30 31 32 33
    // in[7]: 34 35 36 37
    // to:
    // a0: 00 10 01 11
    // a1: 20 30 21 31
    // a2: 02 12 03 13
    // a3: 22 32 23 33
    // a4: 04 14 05 15
    // a5: 24 34 25 35
    // a6: 06 16 07 17
    // a7: 26 36 27 37
    let a0 = _mm_unpacklo_epi32(*in_.add(0), *in_.add(2));
    let a1 = _mm_unpacklo_epi32(*in_.add(4), *in_.add(6));
    let a2 = _mm_unpackhi_epi32(*in_.add(0), *in_.add(2));
    let a3 = _mm_unpackhi_epi32(*in_.add(4), *in_.add(6));
    let a4 = _mm_unpacklo_epi32(*in_.add(1), *in_.add(3));
    let a5 = _mm_unpacklo_epi32(*in_.add(5), *in_.add(7));
    let a6 = _mm_unpackhi_epi32(*in_.add(1), *in_.add(3));
    let a7 = _mm_unpackhi_epi32(*in_.add(5), *in_.add(7));

    // Unpack 64 bit elements resulting in:
    // out[0]: 00 10 20 30
    // out[1]: 01 11 21 31
    // out[2]: 02 12 22 32
    // out[3]: 03 13 23 33
    // out[4]: 04 14 24 34
    // out[5]: 05 15 25 35
    // out[6]: 06 16 26 36
    // out[7]: 07 17 27 37
    *out.add(0) = _mm_unpacklo_epi64(a0, a1);
    *out.add(1) = _mm_unpackhi_epi64(a0, a1);
    *out.add(2) = _mm_unpacklo_epi64(a2, a3);
    *out.add(3) = _mm_unpackhi_epi64(a2, a3);
    *out.add(4) = _mm_unpacklo_epi64(a4, a5);
    *out.add(5) = _mm_unpackhi_epi64(a4, a5);
    *out.add(6) = _mm_unpacklo_epi64(a6, a7);
    *out.add(7) = _mm_unpackhi_epi64(a6, a7);
}
