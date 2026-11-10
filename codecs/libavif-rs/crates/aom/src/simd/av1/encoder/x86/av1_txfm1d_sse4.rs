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

// Translated by hand from upstream/aom/av1/encoder/x86/av1_txfm1d_sse4.h
// (ADR 0056, ADR 0058). Names, order, comments, and intrinsic sequences follow
// the original. The transposes carry `sse2`, the smallest target feature their
// own intrinsics need.
//
// Translation notes:
// - The prototypes of av1_fdct32_sse4_1, av1_fdct64_sse4_1, and
//   av1_idtx32_sse4_1 declare functions defined in av1_fwd_txfm1d_sse4.c; they
//   are re-exported here from that file's translation, so callers that include
//   this header reach them through this module as in C.
// - The four btf_32 macros stay macros (macro_rules, importable as
//   `crate::simd::av1__encoder__x86__av1_txfm1d_sse4::btf_32_sse4_1_type0` and
//   so on) because out0 and out1 are assignment targets. The expansion is that
//   of C: every argument is evaluated where the C macro names it, so an `in1`
//   that aliases `out0` sees the value just assigned, exactly as in C.
// - The run-time `bit` passed to `_mm_srai_epi32` in btf_32_type0_sse4_1_new is
//   translated with the count-register intrinsic
//   (`_mm_sra_epi32(v, _mm_cvtsi32_si128(bit))`), which GCC and Clang emit for a
//   non-constant count and which is bit-identical for every `int` count (see
//   av1/common/x86/av1_txfm_sse4.rs).
// - Array parameters stay raw pointers (`input[i]` -> `*input.offset(i)`).

use core::arch::x86_64::*;

pub use crate::simd::av1__encoder__x86__av1_fwd_txfm1d_sse4::{av1_fdct32_sse4_1, av1_fdct64_sse4_1,
                                                               av1_idtx32_sse4_1};

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn transpose_32_4x4(stride: i32, input: *const __m128i, output: *mut __m128i) {
    let temp0 = _mm_unpacklo_epi32(*input.offset((0 * stride) as isize),
                                   *input.offset((2 * stride) as isize));
    let temp1 = _mm_unpackhi_epi32(*input.offset((0 * stride) as isize),
                                   *input.offset((2 * stride) as isize));
    let temp2 = _mm_unpacklo_epi32(*input.offset((1 * stride) as isize),
                                   *input.offset((3 * stride) as isize));
    let temp3 = _mm_unpackhi_epi32(*input.offset((1 * stride) as isize),
                                   *input.offset((3 * stride) as isize));

    *output.offset((0 * stride) as isize) = _mm_unpacklo_epi32(temp0, temp2);
    *output.offset((1 * stride) as isize) = _mm_unpackhi_epi32(temp0, temp2);
    *output.offset((2 * stride) as isize) = _mm_unpacklo_epi32(temp1, temp3);
    *output.offset((3 * stride) as isize) = _mm_unpackhi_epi32(temp1, temp3);
}

// the entire input block can be represent by a grid of 4x4 blocks
// each 4x4 blocks can be represent by 4 vertical __m128i
// we first transpose each 4x4 block internally
// then transpose the grid
#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn transpose_32(txfm_size: i32, input: *const __m128i, output: *mut __m128i) {
    let num_per_128: i32 = 4;
    let row_size: i32 = txfm_size;
    let col_size: i32 = txfm_size / num_per_128;
    let mut r: i32;
    let mut c: i32;

    // transpose each 4x4 block internally
    r = 0;
    while r < row_size {
        c = 0;
        while c < col_size {
            transpose_32_4x4(col_size, input.offset((r * col_size + c) as isize),
                             output.offset((c * 4 * col_size + r / 4) as isize));
            c += 1;
        }
        r += 4;
    }
}

// out0 = in0*w0 + in1*w1
// out1 = -in1*w0 + in0*w1
macro_rules! btf_32_sse4_1_type0 {
    ($w0:expr, $w1:expr, $in0:expr, $in1:expr, $out0:expr, $out1:expr, $bit:expr) => {{
        let ww0 = ::core::arch::x86_64::_mm_set1_epi32($w0);
        let ww1 = ::core::arch::x86_64::_mm_set1_epi32($w1);
        let in0_w0 = ::core::arch::x86_64::_mm_mullo_epi32($in0, ww0);
        let in1_w1 = ::core::arch::x86_64::_mm_mullo_epi32($in1, ww1);
        $out0 = ::core::arch::x86_64::_mm_add_epi32(in0_w0, in1_w1);
        $out0 = $crate::simd::av1__common__x86__av1_txfm_sse4::av1_round_shift_32_sse4_1(
            $out0, ($bit) as i32);
        let in0_w1 = ::core::arch::x86_64::_mm_mullo_epi32($in0, ww1);
        let in1_w0 = ::core::arch::x86_64::_mm_mullo_epi32($in1, ww0);
        $out1 = ::core::arch::x86_64::_mm_sub_epi32(in0_w1, in1_w0);
        $out1 = $crate::simd::av1__common__x86__av1_txfm_sse4::av1_round_shift_32_sse4_1(
            $out1, ($bit) as i32);
    }};
}
pub(crate) use btf_32_sse4_1_type0;

// out0 = in0*w0 + in1*w1
// out1 = in1*w0 - in0*w1
macro_rules! btf_32_sse4_1_type1 {
    ($w0:expr, $w1:expr, $in0:expr, $in1:expr, $out0:expr, $out1:expr, $bit:expr) => {{
        $crate::simd::av1__encoder__x86__av1_txfm1d_sse4::btf_32_sse4_1_type0!(
            $w1, $w0, $in1, $in0, $out0, $out1, $bit);
    }};
}
pub(crate) use btf_32_sse4_1_type1;

// out0 = in0*w0 + in1*w1
// out1 = -in1*w0 + in0*w1
macro_rules! btf_32_type0_sse4_1_new {
    ($ww0:expr, $ww1:expr, $in0:expr, $in1:expr, $out0:expr, $out1:expr, $r:expr, $bit:expr) => {{
        let in0_w0 = ::core::arch::x86_64::_mm_mullo_epi32($in0, $ww0);
        let in1_w1 = ::core::arch::x86_64::_mm_mullo_epi32($in1, $ww1);
        $out0 = ::core::arch::x86_64::_mm_add_epi32(in0_w0, in1_w1);
        $out0 = ::core::arch::x86_64::_mm_add_epi32($out0, $r);
        // _mm_srai_epi32(out0, bit) with a run-time count.
        $out0 = ::core::arch::x86_64::_mm_sra_epi32(
            $out0, ::core::arch::x86_64::_mm_cvtsi32_si128(($bit) as i32));
        let in0_w1 = ::core::arch::x86_64::_mm_mullo_epi32($in0, $ww1);
        let in1_w0 = ::core::arch::x86_64::_mm_mullo_epi32($in1, $ww0);
        $out1 = ::core::arch::x86_64::_mm_sub_epi32(in0_w1, in1_w0);
        $out1 = ::core::arch::x86_64::_mm_add_epi32($out1, $r);
        // _mm_srai_epi32(out1, bit) with a run-time count.
        $out1 = ::core::arch::x86_64::_mm_sra_epi32(
            $out1, ::core::arch::x86_64::_mm_cvtsi32_si128(($bit) as i32));
    }};
}
pub(crate) use btf_32_type0_sse4_1_new;

// out0 = in0*w0 + in1*w1
// out1 = in1*w0 - in0*w1
macro_rules! btf_32_type1_sse4_1_new {
    ($ww0:expr, $ww1:expr, $in0:expr, $in1:expr, $out0:expr, $out1:expr, $r:expr, $bit:expr) => {{
        $crate::simd::av1__encoder__x86__av1_txfm1d_sse4::btf_32_type0_sse4_1_new!(
            $ww1, $ww0, $in1, $in0, $out0, $out1, $r, $bit);
    }};
}
pub(crate) use btf_32_type1_sse4_1_new;
