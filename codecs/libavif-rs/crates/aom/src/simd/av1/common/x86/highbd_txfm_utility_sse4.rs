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

// Translated by hand from upstream/aom/av1/common/x86/highbd_txfm_utility_sse4.h
// (ADR 0056, ADR 0058). Names, order, and intrinsic sequences follow the
// original. Every helper carries the smallest target feature its own
// intrinsics need: `sse2` for the transposes, `sse4.1` for the half
// butterflies (`_mm_mullo_epi32`).
//
// Translation notes:
// - TRANSPOSE_4X4 stays a macro (macro_rules, importable as
//   `crate::simd::av1__common__x86__highbd_txfm_utility_sse4::TRANSPOSE_4X4`)
//   because its last four arguments are assignment targets. It evaluates its
//   arguments exactly as the C macro does (x0..x3 twice each, before any
//   output is written).
// - Array parameters stay raw pointers (`in[i]` -> `*in_.add(i)`).
// - half_btf_sse4_1 and half_btf_0_sse4_1 pass their run-time `bit` to
//   `_mm_srai_epi32`. GCC and Clang lower a non-constant count to PSRAD xmm,
//   xmm with the count zero-extended by MOVD, so it is translated with the
//   count-register intrinsic `_mm_sra_epi32(x, _mm_cvtsi32_si128(bit))`, which
//   is bit-identical for every `int` count. The callers pass table-driven
//   `cos_bit` values, so a const-generic signature is not possible.
// - The function-pointer typedefs become `Option<unsafe fn(..)>` aliases, as in
//   the Neon translations, so dispatch tables can hold NULL entries.
// - The prototype of av1_highbd_inv_txfm2d_add_universe_sse4_1 declares a
//   function defined in highbd_inv_txfm_sse4.c; it belongs to that file's
//   translation and is not repeated here.

use core::arch::x86_64::*;

macro_rules! TRANSPOSE_4X4 {
    ($x0:expr, $x1:expr, $x2:expr, $x3:expr, $y0:expr, $y1:expr, $y2:expr, $y3:expr) => {{
        let u0: ::core::arch::x86_64::__m128i;
        let u1: ::core::arch::x86_64::__m128i;
        let u2: ::core::arch::x86_64::__m128i;
        let u3: ::core::arch::x86_64::__m128i;
        u0 = ::core::arch::x86_64::_mm_unpacklo_epi32($x0, $x1);
        u1 = ::core::arch::x86_64::_mm_unpackhi_epi32($x0, $x1);
        u2 = ::core::arch::x86_64::_mm_unpacklo_epi32($x2, $x3);
        u3 = ::core::arch::x86_64::_mm_unpackhi_epi32($x2, $x3);
        $y0 = ::core::arch::x86_64::_mm_unpacklo_epi64(u0, u2);
        $y1 = ::core::arch::x86_64::_mm_unpackhi_epi64(u0, u2);
        $y2 = ::core::arch::x86_64::_mm_unpacklo_epi64(u1, u3);
        $y3 = ::core::arch::x86_64::_mm_unpackhi_epi64(u1, u3);
    }};
}
// Unused until the SSE4.1 transform translations import it.
#[allow(unused_imports)]
pub(crate) use TRANSPOSE_4X4;

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn transpose_8x8(in_: *const __m128i, out: *mut __m128i) {
    TRANSPOSE_4X4!(*in_.add(0), *in_.add(2), *in_.add(4), *in_.add(6), *out.add(0), *out.add(2),
                   *out.add(4), *out.add(6));
    TRANSPOSE_4X4!(*in_.add(1), *in_.add(3), *in_.add(5), *in_.add(7), *out.add(8), *out.add(10),
                   *out.add(12), *out.add(14));
    TRANSPOSE_4X4!(*in_.add(8), *in_.add(10), *in_.add(12), *in_.add(14), *out.add(1),
                   *out.add(3), *out.add(5), *out.add(7));
    TRANSPOSE_4X4!(*in_.add(9), *in_.add(11), *in_.add(13), *in_.add(15), *out.add(9),
                   *out.add(11), *out.add(13), *out.add(15));
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn transpose_16x16(in_: *const __m128i, out: *mut __m128i) {
    // Upper left 8x8
    TRANSPOSE_4X4!(*in_.add(0), *in_.add(4), *in_.add(8), *in_.add(12), *out.add(0), *out.add(4),
                   *out.add(8), *out.add(12));
    TRANSPOSE_4X4!(*in_.add(1), *in_.add(5), *in_.add(9), *in_.add(13), *out.add(16),
                   *out.add(20), *out.add(24), *out.add(28));
    TRANSPOSE_4X4!(*in_.add(16), *in_.add(20), *in_.add(24), *in_.add(28), *out.add(1),
                   *out.add(5), *out.add(9), *out.add(13));
    TRANSPOSE_4X4!(*in_.add(17), *in_.add(21), *in_.add(25), *in_.add(29), *out.add(17),
                   *out.add(21), *out.add(25), *out.add(29));

    // Upper right 8x8
    TRANSPOSE_4X4!(*in_.add(2), *in_.add(6), *in_.add(10), *in_.add(14), *out.add(32),
                   *out.add(36), *out.add(40), *out.add(44));
    TRANSPOSE_4X4!(*in_.add(3), *in_.add(7), *in_.add(11), *in_.add(15), *out.add(48),
                   *out.add(52), *out.add(56), *out.add(60));
    TRANSPOSE_4X4!(*in_.add(18), *in_.add(22), *in_.add(26), *in_.add(30), *out.add(33),
                   *out.add(37), *out.add(41), *out.add(45));
    TRANSPOSE_4X4!(*in_.add(19), *in_.add(23), *in_.add(27), *in_.add(31), *out.add(49),
                   *out.add(53), *out.add(57), *out.add(61));

    // Lower left 8x8
    TRANSPOSE_4X4!(*in_.add(32), *in_.add(36), *in_.add(40), *in_.add(44), *out.add(2),
                   *out.add(6), *out.add(10), *out.add(14));
    TRANSPOSE_4X4!(*in_.add(33), *in_.add(37), *in_.add(41), *in_.add(45), *out.add(18),
                   *out.add(22), *out.add(26), *out.add(30));
    TRANSPOSE_4X4!(*in_.add(48), *in_.add(52), *in_.add(56), *in_.add(60), *out.add(3),
                   *out.add(7), *out.add(11), *out.add(15));
    TRANSPOSE_4X4!(*in_.add(49), *in_.add(53), *in_.add(57), *in_.add(61), *out.add(19),
                   *out.add(23), *out.add(27), *out.add(31));
    // Lower right 8x8
    TRANSPOSE_4X4!(*in_.add(34), *in_.add(38), *in_.add(42), *in_.add(46), *out.add(34),
                   *out.add(38), *out.add(42), *out.add(46));
    TRANSPOSE_4X4!(*in_.add(35), *in_.add(39), *in_.add(43), *in_.add(47), *out.add(50),
                   *out.add(54), *out.add(58), *out.add(62));
    TRANSPOSE_4X4!(*in_.add(50), *in_.add(54), *in_.add(58), *in_.add(62), *out.add(35),
                   *out.add(39), *out.add(43), *out.add(47));
    TRANSPOSE_4X4!(*in_.add(51), *in_.add(55), *in_.add(59), *in_.add(63), *out.add(51),
                   *out.add(55), *out.add(59), *out.add(63));
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn transpose_8nx8n(input: *const __m128i, output: *mut __m128i, width: i32,
                              height: i32) {
    let numcol = height >> 2;
    let numrow = width >> 2;
    let mut j: i32 = 0;
    while j < numrow {
        let mut i: i32 = 0;
        while i < numcol {
            TRANSPOSE_4X4!(*input.offset((i * width + j + (numrow * 0)) as isize),
                           *input.offset((i * width + j + (numrow * 1)) as isize),
                           *input.offset((i * width + j + (numrow * 2)) as isize),
                           *input.offset((i * width + j + (numrow * 3)) as isize),
                           *output.offset((j * height + i + (numcol * 0)) as isize),
                           *output.offset((j * height + i + (numcol * 1)) as isize),
                           *output.offset((j * height + i + (numcol * 2)) as isize),
                           *output.offset((j * height + i + (numcol * 3)) as isize));
            i += 1;
        }
        j += 1;
    }
}

// Note:
//  rounding = 1 << (bit - 1)
#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn half_btf_sse4_1(w0: *const __m128i, n0: *const __m128i, w1: *const __m128i,
                              n1: *const __m128i, rounding: *const __m128i, bit: i32) -> __m128i {
    let mut x: __m128i;
    let y: __m128i;

    x = _mm_mullo_epi32(*w0, *n0);
    y = _mm_mullo_epi32(*w1, *n1);
    x = _mm_add_epi32(x, y);
    x = _mm_add_epi32(x, *rounding);
    // _mm_srai_epi32(x, bit) with a run-time count.
    x = _mm_sra_epi32(x, _mm_cvtsi32_si128(bit));
    x
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn half_btf_0_sse4_1(w0: *const __m128i, n0: *const __m128i, rounding: *const __m128i,
                                bit: i32) -> __m128i {
    let mut x: __m128i;

    x = _mm_mullo_epi32(*w0, *n0);
    x = _mm_add_epi32(x, *rounding);
    // _mm_srai_epi32(x, bit) with a run-time count.
    x = _mm_sra_epi32(x, _mm_cvtsi32_si128(bit));
    x
}

pub type transform_1d_sse4_1 =
    Option<unsafe fn(*mut __m128i, *mut __m128i, i32, i32, i32, i32)>;

pub type fwd_transform_1d_sse4_1 = Option<unsafe fn(*mut __m128i, *mut __m128i, i32, i32)>;
