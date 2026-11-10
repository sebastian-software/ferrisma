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

// Translated from upstream/aom/av1/encoder/x86/av1_fwd_txfm1d_sse4.c
// (ADR 0056, ADR 0058). Upstream compiles this file with -msse4.1. Names,
// order, comments, and intrinsic sequences follow the original.
// The straight-line function bodies were transliterated with statement-
// rewriting scripts (not C2Rust) and then compared with the C function by
// function: identifiers, constants and operators in order (ADR 0058).
//
// Translation notes:
// - The three functions are external in C but are not RTCD functions; they are
//   called only by the other hand-translated forward transforms, so they are
//   plain `pub unsafe fn` items with the `sse4.1` target feature of this file.
// - C arrays become zero-initialized stack arrays that are only accessed
//   through a raw pointer to their first element (`buf0[i]` -> `*buf0.add(i)`),
//   and the array parameters stay raw pointers
//   (`input[startidx]` -> `*input.offset(startidx as isize)`). The C code never
//   reads a local element before writing it.
// - The btf_32_* macros come from av1_txfm1d_sse4.rs and are invoked with the
//   C arguments in the C order. The int8_t `cos_bit` of av1_fdct64_sse4_1 is
//   widened with `as i32` where C promotes it to int (cospi_arr, the rounding
//   constant, and inside the macros).
// - av1_idtx32_sse4_1 shifts by the constant 2, so it keeps the immediate
//   intrinsic (`_mm_slli_epi32::<2>`).
#![allow(clippy::all)]

use core::arch::x86_64::*;

use crate::av1__common__av1_txfm::av1_cospi_arr_data;
use crate::simd::av1__encoder__x86__av1_txfm1d_sse4::{btf_32_sse4_1_type0, btf_32_sse4_1_type1,
                                                      btf_32_type0_sse4_1_new,
                                                      btf_32_type1_sse4_1_new};

// From av1/common/av1_txfm.h.
const cos_bit_min: i32 = 10;

// From av1/common/av1_txfm.h.
#[inline]
unsafe fn cospi_arr(n: i32) -> *const i32 {
    (&raw const av1_cospi_arr_data[(n - cos_bit_min) as usize]).cast::<i32>()
}

#[target_feature(enable = "sse4.1")]
pub unsafe fn av1_fdct32_sse4_1(input: *mut __m128i, output: *mut __m128i, cos_bit: i32,
                                stride: i32) {
    let mut buf0_: [__m128i; 32] = core::mem::zeroed();
    let buf0: *mut __m128i = buf0_.as_mut_ptr();
    let mut buf1_: [__m128i; 32] = core::mem::zeroed();
    let buf1: *mut __m128i = buf1_.as_mut_ptr();
    let mut cospi: *const i32;

    let mut startidx: i32 = 0 * stride;
    let mut endidx: i32 = 31 * stride;
    // stage 0
    // stage 1
    *buf1.add(0) = _mm_add_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    *buf1.add(31) = _mm_sub_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    startidx += stride;
    endidx -= stride;
    *buf1.add(1) = _mm_add_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    *buf1.add(30) = _mm_sub_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    startidx += stride;
    endidx -= stride;
    *buf1.add(2) = _mm_add_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    *buf1.add(29) = _mm_sub_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    startidx += stride;
    endidx -= stride;
    *buf1.add(3) = _mm_add_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    *buf1.add(28) = _mm_sub_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    startidx += stride;
    endidx -= stride;
    *buf1.add(4) = _mm_add_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    *buf1.add(27) = _mm_sub_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    startidx += stride;
    endidx -= stride;
    *buf1.add(5) = _mm_add_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    *buf1.add(26) = _mm_sub_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    startidx += stride;
    endidx -= stride;
    *buf1.add(6) = _mm_add_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    *buf1.add(25) = _mm_sub_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    startidx += stride;
    endidx -= stride;
    *buf1.add(7) = _mm_add_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    *buf1.add(24) = _mm_sub_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    startidx += stride;
    endidx -= stride;
    *buf1.add(8) = _mm_add_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    *buf1.add(23) = _mm_sub_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    startidx += stride;
    endidx -= stride;
    *buf1.add(9) = _mm_add_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    *buf1.add(22) = _mm_sub_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    startidx += stride;
    endidx -= stride;
    *buf1.add(10) = _mm_add_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    *buf1.add(21) = _mm_sub_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    startidx += stride;
    endidx -= stride;
    *buf1.add(11) = _mm_add_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    *buf1.add(20) = _mm_sub_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    startidx += stride;
    endidx -= stride;
    *buf1.add(12) = _mm_add_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    *buf1.add(19) = _mm_sub_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    startidx += stride;
    endidx -= stride;
    *buf1.add(13) = _mm_add_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    *buf1.add(18) = _mm_sub_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    startidx += stride;
    endidx -= stride;
    *buf1.add(14) = _mm_add_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    *buf1.add(17) = _mm_sub_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    startidx += stride;
    endidx -= stride;
    *buf1.add(15) = _mm_add_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    *buf1.add(16) = _mm_sub_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));

    // stage 2
    cospi = cospi_arr(cos_bit);
    *buf0.add(0) = _mm_add_epi32(*buf1.add(0), *buf1.add(15));
    *buf0.add(15) = _mm_sub_epi32(*buf1.add(0), *buf1.add(15));
    *buf0.add(1) = _mm_add_epi32(*buf1.add(1), *buf1.add(14));
    *buf0.add(14) = _mm_sub_epi32(*buf1.add(1), *buf1.add(14));
    *buf0.add(2) = _mm_add_epi32(*buf1.add(2), *buf1.add(13));
    *buf0.add(13) = _mm_sub_epi32(*buf1.add(2), *buf1.add(13));
    *buf0.add(3) = _mm_add_epi32(*buf1.add(3), *buf1.add(12));
    *buf0.add(12) = _mm_sub_epi32(*buf1.add(3), *buf1.add(12));
    *buf0.add(4) = _mm_add_epi32(*buf1.add(4), *buf1.add(11));
    *buf0.add(11) = _mm_sub_epi32(*buf1.add(4), *buf1.add(11));
    *buf0.add(5) = _mm_add_epi32(*buf1.add(5), *buf1.add(10));
    *buf0.add(10) = _mm_sub_epi32(*buf1.add(5), *buf1.add(10));
    *buf0.add(6) = _mm_add_epi32(*buf1.add(6), *buf1.add(9));
    *buf0.add(9) = _mm_sub_epi32(*buf1.add(6), *buf1.add(9));
    *buf0.add(7) = _mm_add_epi32(*buf1.add(7), *buf1.add(8));
    *buf0.add(8) = _mm_sub_epi32(*buf1.add(7), *buf1.add(8));
    *buf0.add(16) = *buf1.add(16);
    *buf0.add(17) = *buf1.add(17);
    *buf0.add(18) = *buf1.add(18);
    *buf0.add(19) = *buf1.add(19);
    btf_32_sse4_1_type0!(-*cospi.add(32), *cospi.add(32), *buf1.add(20), *buf1.add(27),
                         *buf0.add(20), *buf0.add(27), cos_bit);
    btf_32_sse4_1_type0!(-*cospi.add(32), *cospi.add(32), *buf1.add(21), *buf1.add(26),
                         *buf0.add(21), *buf0.add(26), cos_bit);
    btf_32_sse4_1_type0!(-*cospi.add(32), *cospi.add(32), *buf1.add(22), *buf1.add(25),
                         *buf0.add(22), *buf0.add(25), cos_bit);
    btf_32_sse4_1_type0!(-*cospi.add(32), *cospi.add(32), *buf1.add(23), *buf1.add(24),
                         *buf0.add(23), *buf0.add(24), cos_bit);
    *buf0.add(28) = *buf1.add(28);
    *buf0.add(29) = *buf1.add(29);
    *buf0.add(30) = *buf1.add(30);
    *buf0.add(31) = *buf1.add(31);

    // stage 3
    cospi = cospi_arr(cos_bit);
    *buf1.add(0) = _mm_add_epi32(*buf0.add(0), *buf0.add(7));
    *buf1.add(7) = _mm_sub_epi32(*buf0.add(0), *buf0.add(7));
    *buf1.add(1) = _mm_add_epi32(*buf0.add(1), *buf0.add(6));
    *buf1.add(6) = _mm_sub_epi32(*buf0.add(1), *buf0.add(6));
    *buf1.add(2) = _mm_add_epi32(*buf0.add(2), *buf0.add(5));
    *buf1.add(5) = _mm_sub_epi32(*buf0.add(2), *buf0.add(5));
    *buf1.add(3) = _mm_add_epi32(*buf0.add(3), *buf0.add(4));
    *buf1.add(4) = _mm_sub_epi32(*buf0.add(3), *buf0.add(4));
    *buf1.add(8) = *buf0.add(8);
    *buf1.add(9) = *buf0.add(9);
    btf_32_sse4_1_type0!(-*cospi.add(32), *cospi.add(32), *buf0.add(10), *buf0.add(13),
                         *buf1.add(10), *buf1.add(13), cos_bit);
    btf_32_sse4_1_type0!(-*cospi.add(32), *cospi.add(32), *buf0.add(11), *buf0.add(12),
                         *buf1.add(11), *buf1.add(12), cos_bit);
    *buf1.add(14) = *buf0.add(14);
    *buf1.add(15) = *buf0.add(15);
    *buf1.add(16) = _mm_add_epi32(*buf0.add(16), *buf0.add(23));
    *buf1.add(23) = _mm_sub_epi32(*buf0.add(16), *buf0.add(23));
    *buf1.add(17) = _mm_add_epi32(*buf0.add(17), *buf0.add(22));
    *buf1.add(22) = _mm_sub_epi32(*buf0.add(17), *buf0.add(22));
    *buf1.add(18) = _mm_add_epi32(*buf0.add(18), *buf0.add(21));
    *buf1.add(21) = _mm_sub_epi32(*buf0.add(18), *buf0.add(21));
    *buf1.add(19) = _mm_add_epi32(*buf0.add(19), *buf0.add(20));
    *buf1.add(20) = _mm_sub_epi32(*buf0.add(19), *buf0.add(20));
    *buf1.add(24) = _mm_sub_epi32(*buf0.add(31), *buf0.add(24));
    *buf1.add(31) = _mm_add_epi32(*buf0.add(31), *buf0.add(24));
    *buf1.add(25) = _mm_sub_epi32(*buf0.add(30), *buf0.add(25));
    *buf1.add(30) = _mm_add_epi32(*buf0.add(30), *buf0.add(25));
    *buf1.add(26) = _mm_sub_epi32(*buf0.add(29), *buf0.add(26));
    *buf1.add(29) = _mm_add_epi32(*buf0.add(29), *buf0.add(26));
    *buf1.add(27) = _mm_sub_epi32(*buf0.add(28), *buf0.add(27));
    *buf1.add(28) = _mm_add_epi32(*buf0.add(28), *buf0.add(27));

    // stage 4
    cospi = cospi_arr(cos_bit);
    *buf0.add(0) = _mm_add_epi32(*buf1.add(0), *buf1.add(3));
    *buf0.add(3) = _mm_sub_epi32(*buf1.add(0), *buf1.add(3));
    *buf0.add(1) = _mm_add_epi32(*buf1.add(1), *buf1.add(2));
    *buf0.add(2) = _mm_sub_epi32(*buf1.add(1), *buf1.add(2));
    *buf0.add(4) = *buf1.add(4);
    btf_32_sse4_1_type0!(-*cospi.add(32), *cospi.add(32), *buf1.add(5), *buf1.add(6), *buf0.add(5),
                         *buf0.add(6), cos_bit);
    *buf0.add(7) = *buf1.add(7);
    *buf0.add(8) = _mm_add_epi32(*buf1.add(8), *buf1.add(11));
    *buf0.add(11) = _mm_sub_epi32(*buf1.add(8), *buf1.add(11));
    *buf0.add(9) = _mm_add_epi32(*buf1.add(9), *buf1.add(10));
    *buf0.add(10) = _mm_sub_epi32(*buf1.add(9), *buf1.add(10));
    *buf0.add(12) = _mm_sub_epi32(*buf1.add(15), *buf1.add(12));
    *buf0.add(15) = _mm_add_epi32(*buf1.add(15), *buf1.add(12));
    *buf0.add(13) = _mm_sub_epi32(*buf1.add(14), *buf1.add(13));
    *buf0.add(14) = _mm_add_epi32(*buf1.add(14), *buf1.add(13));
    *buf0.add(16) = *buf1.add(16);
    *buf0.add(17) = *buf1.add(17);
    btf_32_sse4_1_type0!(-*cospi.add(16), *cospi.add(48), *buf1.add(18), *buf1.add(29),
                         *buf0.add(18), *buf0.add(29), cos_bit);
    btf_32_sse4_1_type0!(-*cospi.add(16), *cospi.add(48), *buf1.add(19), *buf1.add(28),
                         *buf0.add(19), *buf0.add(28), cos_bit);
    btf_32_sse4_1_type0!(-*cospi.add(48), -*cospi.add(16), *buf1.add(20), *buf1.add(27),
                         *buf0.add(20), *buf0.add(27), cos_bit);
    btf_32_sse4_1_type0!(-*cospi.add(48), -*cospi.add(16), *buf1.add(21), *buf1.add(26),
                         *buf0.add(21), *buf0.add(26), cos_bit);
    *buf0.add(22) = *buf1.add(22);
    *buf0.add(23) = *buf1.add(23);
    *buf0.add(24) = *buf1.add(24);
    *buf0.add(25) = *buf1.add(25);
    *buf0.add(30) = *buf1.add(30);
    *buf0.add(31) = *buf1.add(31);

    // stage 5
    cospi = cospi_arr(cos_bit);
    btf_32_sse4_1_type0!(*cospi.add(32), *cospi.add(32), *buf0.add(0), *buf0.add(1), *buf1.add(0),
                         *buf1.add(1), cos_bit);
    btf_32_sse4_1_type1!(*cospi.add(48), *cospi.add(16), *buf0.add(2), *buf0.add(3), *buf1.add(2),
                         *buf1.add(3), cos_bit);
    *buf1.add(4) = _mm_add_epi32(*buf0.add(4), *buf0.add(5));
    *buf1.add(5) = _mm_sub_epi32(*buf0.add(4), *buf0.add(5));
    *buf1.add(6) = _mm_sub_epi32(*buf0.add(7), *buf0.add(6));
    *buf1.add(7) = _mm_add_epi32(*buf0.add(7), *buf0.add(6));
    *buf1.add(8) = *buf0.add(8);
    btf_32_sse4_1_type0!(-*cospi.add(16), *cospi.add(48), *buf0.add(9), *buf0.add(14), *buf1.add(9),
                         *buf1.add(14), cos_bit);
    btf_32_sse4_1_type0!(-*cospi.add(48), -*cospi.add(16), *buf0.add(10), *buf0.add(13),
                         *buf1.add(10), *buf1.add(13), cos_bit);
    *buf1.add(11) = *buf0.add(11);
    *buf1.add(12) = *buf0.add(12);
    *buf1.add(15) = *buf0.add(15);
    *buf1.add(16) = _mm_add_epi32(*buf0.add(16), *buf0.add(19));
    *buf1.add(19) = _mm_sub_epi32(*buf0.add(16), *buf0.add(19));
    *buf1.add(17) = _mm_add_epi32(*buf0.add(17), *buf0.add(18));
    *buf1.add(18) = _mm_sub_epi32(*buf0.add(17), *buf0.add(18));
    *buf1.add(20) = _mm_sub_epi32(*buf0.add(23), *buf0.add(20));
    *buf1.add(23) = _mm_add_epi32(*buf0.add(23), *buf0.add(20));
    *buf1.add(21) = _mm_sub_epi32(*buf0.add(22), *buf0.add(21));
    *buf1.add(22) = _mm_add_epi32(*buf0.add(22), *buf0.add(21));
    *buf1.add(24) = _mm_add_epi32(*buf0.add(24), *buf0.add(27));
    *buf1.add(27) = _mm_sub_epi32(*buf0.add(24), *buf0.add(27));
    *buf1.add(25) = _mm_add_epi32(*buf0.add(25), *buf0.add(26));
    *buf1.add(26) = _mm_sub_epi32(*buf0.add(25), *buf0.add(26));
    *buf1.add(28) = _mm_sub_epi32(*buf0.add(31), *buf0.add(28));
    *buf1.add(31) = _mm_add_epi32(*buf0.add(31), *buf0.add(28));
    *buf1.add(29) = _mm_sub_epi32(*buf0.add(30), *buf0.add(29));
    *buf1.add(30) = _mm_add_epi32(*buf0.add(30), *buf0.add(29));

    // stage 6
    cospi = cospi_arr(cos_bit);
    *buf0.add(0) = *buf1.add(0);
    *buf0.add(1) = *buf1.add(1);
    *buf0.add(2) = *buf1.add(2);
    *buf0.add(3) = *buf1.add(3);
    btf_32_sse4_1_type1!(*cospi.add(56), *cospi.add(8), *buf1.add(4), *buf1.add(7), *buf0.add(4),
                         *buf0.add(7), cos_bit);
    btf_32_sse4_1_type1!(*cospi.add(24), *cospi.add(40), *buf1.add(5), *buf1.add(6), *buf0.add(5),
                         *buf0.add(6), cos_bit);
    *buf0.add(8) = _mm_add_epi32(*buf1.add(8), *buf1.add(9));
    *buf0.add(9) = _mm_sub_epi32(*buf1.add(8), *buf1.add(9));
    *buf0.add(10) = _mm_sub_epi32(*buf1.add(11), *buf1.add(10));
    *buf0.add(11) = _mm_add_epi32(*buf1.add(11), *buf1.add(10));
    *buf0.add(12) = _mm_add_epi32(*buf1.add(12), *buf1.add(13));
    *buf0.add(13) = _mm_sub_epi32(*buf1.add(12), *buf1.add(13));
    *buf0.add(14) = _mm_sub_epi32(*buf1.add(15), *buf1.add(14));
    *buf0.add(15) = _mm_add_epi32(*buf1.add(15), *buf1.add(14));
    *buf0.add(16) = *buf1.add(16);
    btf_32_sse4_1_type0!(-*cospi.add(8), *cospi.add(56), *buf1.add(17), *buf1.add(30),
                         *buf0.add(17), *buf0.add(30), cos_bit);
    btf_32_sse4_1_type0!(-*cospi.add(56), -*cospi.add(8), *buf1.add(18), *buf1.add(29),
                         *buf0.add(18), *buf0.add(29), cos_bit);
    *buf0.add(19) = *buf1.add(19);
    *buf0.add(20) = *buf1.add(20);
    btf_32_sse4_1_type0!(-*cospi.add(40), *cospi.add(24), *buf1.add(21), *buf1.add(26),
                         *buf0.add(21), *buf0.add(26), cos_bit);
    btf_32_sse4_1_type0!(-*cospi.add(24), -*cospi.add(40), *buf1.add(22), *buf1.add(25),
                         *buf0.add(22), *buf0.add(25), cos_bit);
    *buf0.add(23) = *buf1.add(23);
    *buf0.add(24) = *buf1.add(24);
    *buf0.add(27) = *buf1.add(27);
    *buf0.add(28) = *buf1.add(28);
    *buf0.add(31) = *buf1.add(31);

    // stage 7
    cospi = cospi_arr(cos_bit);
    *buf1.add(0) = *buf0.add(0);
    *buf1.add(1) = *buf0.add(1);
    *buf1.add(2) = *buf0.add(2);
    *buf1.add(3) = *buf0.add(3);
    *buf1.add(4) = *buf0.add(4);
    *buf1.add(5) = *buf0.add(5);
    *buf1.add(6) = *buf0.add(6);
    *buf1.add(7) = *buf0.add(7);
    btf_32_sse4_1_type1!(*cospi.add(60), *cospi.add(4), *buf0.add(8), *buf0.add(15), *buf1.add(8),
                         *buf1.add(15), cos_bit);
    btf_32_sse4_1_type1!(*cospi.add(28), *cospi.add(36), *buf0.add(9), *buf0.add(14), *buf1.add(9),
                         *buf1.add(14), cos_bit);
    btf_32_sse4_1_type1!(*cospi.add(44), *cospi.add(20), *buf0.add(10), *buf0.add(13),
                         *buf1.add(10), *buf1.add(13), cos_bit);
    btf_32_sse4_1_type1!(*cospi.add(12), *cospi.add(52), *buf0.add(11), *buf0.add(12),
                         *buf1.add(11), *buf1.add(12), cos_bit);
    *buf1.add(16) = _mm_add_epi32(*buf0.add(16), *buf0.add(17));
    *buf1.add(17) = _mm_sub_epi32(*buf0.add(16), *buf0.add(17));
    *buf1.add(18) = _mm_sub_epi32(*buf0.add(19), *buf0.add(18));
    *buf1.add(19) = _mm_add_epi32(*buf0.add(19), *buf0.add(18));
    *buf1.add(20) = _mm_add_epi32(*buf0.add(20), *buf0.add(21));
    *buf1.add(21) = _mm_sub_epi32(*buf0.add(20), *buf0.add(21));
    *buf1.add(22) = _mm_sub_epi32(*buf0.add(23), *buf0.add(22));
    *buf1.add(23) = _mm_add_epi32(*buf0.add(23), *buf0.add(22));
    *buf1.add(24) = _mm_add_epi32(*buf0.add(24), *buf0.add(25));
    *buf1.add(25) = _mm_sub_epi32(*buf0.add(24), *buf0.add(25));
    *buf1.add(26) = _mm_sub_epi32(*buf0.add(27), *buf0.add(26));
    *buf1.add(27) = _mm_add_epi32(*buf0.add(27), *buf0.add(26));
    *buf1.add(28) = _mm_add_epi32(*buf0.add(28), *buf0.add(29));
    *buf1.add(29) = _mm_sub_epi32(*buf0.add(28), *buf0.add(29));
    *buf1.add(30) = _mm_sub_epi32(*buf0.add(31), *buf0.add(30));
    *buf1.add(31) = _mm_add_epi32(*buf0.add(31), *buf0.add(30));

    // stage 8
    cospi = cospi_arr(cos_bit);
    *buf0.add(0) = *buf1.add(0);
    *buf0.add(1) = *buf1.add(1);
    *buf0.add(2) = *buf1.add(2);
    *buf0.add(3) = *buf1.add(3);
    *buf0.add(4) = *buf1.add(4);
    *buf0.add(5) = *buf1.add(5);
    *buf0.add(6) = *buf1.add(6);
    *buf0.add(7) = *buf1.add(7);
    *buf0.add(8) = *buf1.add(8);
    *buf0.add(9) = *buf1.add(9);
    *buf0.add(10) = *buf1.add(10);
    *buf0.add(11) = *buf1.add(11);
    *buf0.add(12) = *buf1.add(12);
    *buf0.add(13) = *buf1.add(13);
    *buf0.add(14) = *buf1.add(14);
    *buf0.add(15) = *buf1.add(15);
    btf_32_sse4_1_type1!(*cospi.add(62), *cospi.add(2), *buf1.add(16), *buf1.add(31), *buf0.add(16),
                         *buf0.add(31), cos_bit);
    btf_32_sse4_1_type1!(*cospi.add(30), *cospi.add(34), *buf1.add(17), *buf1.add(30),
                         *buf0.add(17), *buf0.add(30), cos_bit);
    btf_32_sse4_1_type1!(*cospi.add(46), *cospi.add(18), *buf1.add(18), *buf1.add(29),
                         *buf0.add(18), *buf0.add(29), cos_bit);
    btf_32_sse4_1_type1!(*cospi.add(14), *cospi.add(50), *buf1.add(19), *buf1.add(28),
                         *buf0.add(19), *buf0.add(28), cos_bit);
    btf_32_sse4_1_type1!(*cospi.add(54), *cospi.add(10), *buf1.add(20), *buf1.add(27),
                         *buf0.add(20), *buf0.add(27), cos_bit);
    btf_32_sse4_1_type1!(*cospi.add(22), *cospi.add(42), *buf1.add(21), *buf1.add(26),
                         *buf0.add(21), *buf0.add(26), cos_bit);
    btf_32_sse4_1_type1!(*cospi.add(38), *cospi.add(26), *buf1.add(22), *buf1.add(25),
                         *buf0.add(22), *buf0.add(25), cos_bit);
    btf_32_sse4_1_type1!(*cospi.add(6), *cospi.add(58), *buf1.add(23), *buf1.add(24), *buf0.add(23),
                         *buf0.add(24), cos_bit);

    startidx = 0 * stride;
    endidx = 31 * stride;
    // stage 9
    *output.offset(startidx as isize) = *buf0.add(0);
    *output.offset(endidx as isize) = *buf0.add(31);
    startidx += stride;
    endidx -= stride;
    *output.offset(startidx as isize) = *buf0.add(16);
    *output.offset(endidx as isize) = *buf0.add(15);
    startidx += stride;
    endidx -= stride;
    *output.offset(startidx as isize) = *buf0.add(8);
    *output.offset(endidx as isize) = *buf0.add(23);
    startidx += stride;
    endidx -= stride;
    *output.offset(startidx as isize) = *buf0.add(24);
    *output.offset(endidx as isize) = *buf0.add(7);
    startidx += stride;
    endidx -= stride;
    *output.offset(startidx as isize) = *buf0.add(4);
    *output.offset(endidx as isize) = *buf0.add(27);
    startidx += stride;
    endidx -= stride;
    *output.offset(startidx as isize) = *buf0.add(20);
    *output.offset(endidx as isize) = *buf0.add(11);
    startidx += stride;
    endidx -= stride;
    *output.offset(startidx as isize) = *buf0.add(12);
    *output.offset(endidx as isize) = *buf0.add(19);
    startidx += stride;
    endidx -= stride;
    *output.offset(startidx as isize) = *buf0.add(28);
    *output.offset(endidx as isize) = *buf0.add(3);
    startidx += stride;
    endidx -= stride;
    *output.offset(startidx as isize) = *buf0.add(2);
    *output.offset(endidx as isize) = *buf0.add(29);
    startidx += stride;
    endidx -= stride;
    *output.offset(startidx as isize) = *buf0.add(18);
    *output.offset(endidx as isize) = *buf0.add(13);
    startidx += stride;
    endidx -= stride;
    *output.offset(startidx as isize) = *buf0.add(10);
    *output.offset(endidx as isize) = *buf0.add(21);
    startidx += stride;
    endidx -= stride;
    *output.offset(startidx as isize) = *buf0.add(26);
    *output.offset(endidx as isize) = *buf0.add(5);
    startidx += stride;
    endidx -= stride;
    *output.offset(startidx as isize) = *buf0.add(6);
    *output.offset(endidx as isize) = *buf0.add(25);
    startidx += stride;
    endidx -= stride;
    *output.offset(startidx as isize) = *buf0.add(22);
    *output.offset(endidx as isize) = *buf0.add(9);
    startidx += stride;
    endidx -= stride;
    *output.offset(startidx as isize) = *buf0.add(14);
    *output.offset(endidx as isize) = *buf0.add(17);
    startidx += stride;
    endidx -= stride;
    *output.offset(startidx as isize) = *buf0.add(30);
    *output.offset(endidx as isize) = *buf0.add(1);
}

#[target_feature(enable = "sse4.1")]
pub unsafe fn av1_fdct64_sse4_1(input: *mut __m128i, output: *mut __m128i, cos_bit: i8,
                                instride: i32, outstride: i32) {
    let cospi: *const i32 = cospi_arr(cos_bit as i32);
    let __rounding: __m128i = _mm_set1_epi32(1 << (cos_bit as i32 - 1));

    let cospi_m32: __m128i = _mm_set1_epi32(-*cospi.add(32));
    let cospi_p32: __m128i = _mm_set1_epi32(*cospi.add(32));
    let cospi_m16: __m128i = _mm_set1_epi32(-*cospi.add(16));
    let cospi_p48: __m128i = _mm_set1_epi32(*cospi.add(48));
    let cospi_m48: __m128i = _mm_set1_epi32(-*cospi.add(48));
    let cospi_p16: __m128i = _mm_set1_epi32(*cospi.add(16));
    let cospi_m08: __m128i = _mm_set1_epi32(-*cospi.add(8));
    let cospi_p56: __m128i = _mm_set1_epi32(*cospi.add(56));
    let cospi_m56: __m128i = _mm_set1_epi32(-*cospi.add(56));
    let cospi_m40: __m128i = _mm_set1_epi32(-*cospi.add(40));
    let cospi_p24: __m128i = _mm_set1_epi32(*cospi.add(24));
    let cospi_m24: __m128i = _mm_set1_epi32(-*cospi.add(24));
    let cospi_p08: __m128i = _mm_set1_epi32(*cospi.add(8));
    let cospi_p40: __m128i = _mm_set1_epi32(*cospi.add(40));
    let cospi_p60: __m128i = _mm_set1_epi32(*cospi.add(60));
    let cospi_p04: __m128i = _mm_set1_epi32(*cospi.add(4));
    let cospi_p28: __m128i = _mm_set1_epi32(*cospi.add(28));
    let cospi_p36: __m128i = _mm_set1_epi32(*cospi.add(36));
    let cospi_p44: __m128i = _mm_set1_epi32(*cospi.add(44));
    let cospi_p20: __m128i = _mm_set1_epi32(*cospi.add(20));
    let cospi_p12: __m128i = _mm_set1_epi32(*cospi.add(12));
    let cospi_p52: __m128i = _mm_set1_epi32(*cospi.add(52));
    let cospi_m04: __m128i = _mm_set1_epi32(-*cospi.add(4));
    let cospi_m60: __m128i = _mm_set1_epi32(-*cospi.add(60));
    let cospi_m36: __m128i = _mm_set1_epi32(-*cospi.add(36));
    let cospi_m28: __m128i = _mm_set1_epi32(-*cospi.add(28));
    let cospi_m20: __m128i = _mm_set1_epi32(-*cospi.add(20));
    let cospi_m44: __m128i = _mm_set1_epi32(-*cospi.add(44));
    let cospi_m52: __m128i = _mm_set1_epi32(-*cospi.add(52));
    let cospi_m12: __m128i = _mm_set1_epi32(-*cospi.add(12));
    let cospi_p62: __m128i = _mm_set1_epi32(*cospi.add(62));
    let cospi_p02: __m128i = _mm_set1_epi32(*cospi.add(2));
    let cospi_p30: __m128i = _mm_set1_epi32(*cospi.add(30));
    let cospi_p34: __m128i = _mm_set1_epi32(*cospi.add(34));
    let cospi_p46: __m128i = _mm_set1_epi32(*cospi.add(46));
    let cospi_p18: __m128i = _mm_set1_epi32(*cospi.add(18));
    let cospi_p14: __m128i = _mm_set1_epi32(*cospi.add(14));
    let cospi_p50: __m128i = _mm_set1_epi32(*cospi.add(50));
    let cospi_p54: __m128i = _mm_set1_epi32(*cospi.add(54));
    let cospi_p10: __m128i = _mm_set1_epi32(*cospi.add(10));
    let cospi_p22: __m128i = _mm_set1_epi32(*cospi.add(22));
    let cospi_p42: __m128i = _mm_set1_epi32(*cospi.add(42));
    let cospi_p38: __m128i = _mm_set1_epi32(*cospi.add(38));
    let cospi_p26: __m128i = _mm_set1_epi32(*cospi.add(26));
    let cospi_p06: __m128i = _mm_set1_epi32(*cospi.add(6));
    let cospi_p58: __m128i = _mm_set1_epi32(*cospi.add(58));
    let cospi_p63: __m128i = _mm_set1_epi32(*cospi.add(63));
    let cospi_p01: __m128i = _mm_set1_epi32(*cospi.add(1));
    let cospi_p31: __m128i = _mm_set1_epi32(*cospi.add(31));
    let cospi_p33: __m128i = _mm_set1_epi32(*cospi.add(33));
    let cospi_p47: __m128i = _mm_set1_epi32(*cospi.add(47));
    let cospi_p17: __m128i = _mm_set1_epi32(*cospi.add(17));
    let cospi_p15: __m128i = _mm_set1_epi32(*cospi.add(15));
    let cospi_p49: __m128i = _mm_set1_epi32(*cospi.add(49));
    let cospi_p55: __m128i = _mm_set1_epi32(*cospi.add(55));
    let cospi_p09: __m128i = _mm_set1_epi32(*cospi.add(9));
    let cospi_p23: __m128i = _mm_set1_epi32(*cospi.add(23));
    let cospi_p41: __m128i = _mm_set1_epi32(*cospi.add(41));
    let cospi_p39: __m128i = _mm_set1_epi32(*cospi.add(39));
    let cospi_p25: __m128i = _mm_set1_epi32(*cospi.add(25));
    let cospi_p07: __m128i = _mm_set1_epi32(*cospi.add(7));
    let cospi_p57: __m128i = _mm_set1_epi32(*cospi.add(57));
    let cospi_p59: __m128i = _mm_set1_epi32(*cospi.add(59));
    let cospi_p05: __m128i = _mm_set1_epi32(*cospi.add(5));
    let cospi_p27: __m128i = _mm_set1_epi32(*cospi.add(27));
    let cospi_p37: __m128i = _mm_set1_epi32(*cospi.add(37));
    let cospi_p43: __m128i = _mm_set1_epi32(*cospi.add(43));
    let cospi_p21: __m128i = _mm_set1_epi32(*cospi.add(21));
    let cospi_p11: __m128i = _mm_set1_epi32(*cospi.add(11));
    let cospi_p53: __m128i = _mm_set1_epi32(*cospi.add(53));
    let cospi_p51: __m128i = _mm_set1_epi32(*cospi.add(51));
    let cospi_p13: __m128i = _mm_set1_epi32(*cospi.add(13));
    let cospi_p19: __m128i = _mm_set1_epi32(*cospi.add(19));
    let cospi_p45: __m128i = _mm_set1_epi32(*cospi.add(45));
    let cospi_p35: __m128i = _mm_set1_epi32(*cospi.add(35));
    let cospi_p29: __m128i = _mm_set1_epi32(*cospi.add(29));
    let cospi_p03: __m128i = _mm_set1_epi32(*cospi.add(3));
    let cospi_p61: __m128i = _mm_set1_epi32(*cospi.add(61));

    let mut startidx: i32 = 0 * instride;
    let mut endidx: i32 = 63 * instride;
    // stage 1
    let mut x1_: [__m128i; 64] = core::mem::zeroed();
    let x1: *mut __m128i = x1_.as_mut_ptr();
    *x1.add(0) = _mm_add_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    *x1.add(63) = _mm_sub_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *x1.add(1) = _mm_add_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    *x1.add(62) = _mm_sub_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *x1.add(2) = _mm_add_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    *x1.add(61) = _mm_sub_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *x1.add(3) = _mm_add_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    *x1.add(60) = _mm_sub_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *x1.add(4) = _mm_add_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    *x1.add(59) = _mm_sub_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *x1.add(5) = _mm_add_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    *x1.add(58) = _mm_sub_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *x1.add(6) = _mm_add_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    *x1.add(57) = _mm_sub_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *x1.add(7) = _mm_add_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    *x1.add(56) = _mm_sub_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *x1.add(8) = _mm_add_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    *x1.add(55) = _mm_sub_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *x1.add(9) = _mm_add_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    *x1.add(54) = _mm_sub_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *x1.add(10) = _mm_add_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    *x1.add(53) = _mm_sub_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *x1.add(11) = _mm_add_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    *x1.add(52) = _mm_sub_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *x1.add(12) = _mm_add_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    *x1.add(51) = _mm_sub_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *x1.add(13) = _mm_add_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    *x1.add(50) = _mm_sub_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *x1.add(14) = _mm_add_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    *x1.add(49) = _mm_sub_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *x1.add(15) = _mm_add_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    *x1.add(48) = _mm_sub_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *x1.add(16) = _mm_add_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    *x1.add(47) = _mm_sub_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *x1.add(17) = _mm_add_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    *x1.add(46) = _mm_sub_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *x1.add(18) = _mm_add_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    *x1.add(45) = _mm_sub_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *x1.add(19) = _mm_add_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    *x1.add(44) = _mm_sub_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *x1.add(20) = _mm_add_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    *x1.add(43) = _mm_sub_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *x1.add(21) = _mm_add_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    *x1.add(42) = _mm_sub_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *x1.add(22) = _mm_add_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    *x1.add(41) = _mm_sub_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *x1.add(23) = _mm_add_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    *x1.add(40) = _mm_sub_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *x1.add(24) = _mm_add_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    *x1.add(39) = _mm_sub_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *x1.add(25) = _mm_add_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    *x1.add(38) = _mm_sub_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *x1.add(26) = _mm_add_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    *x1.add(37) = _mm_sub_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *x1.add(27) = _mm_add_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    *x1.add(36) = _mm_sub_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *x1.add(28) = _mm_add_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    *x1.add(35) = _mm_sub_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *x1.add(29) = _mm_add_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    *x1.add(34) = _mm_sub_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *x1.add(30) = _mm_add_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    *x1.add(33) = _mm_sub_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *x1.add(31) = _mm_add_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    *x1.add(32) = _mm_sub_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));

    // stage 2
    let mut x2_: [__m128i; 64] = core::mem::zeroed();
    let x2: *mut __m128i = x2_.as_mut_ptr();
    *x2.add(0) = _mm_add_epi32(*x1.add(0), *x1.add(31));
    *x2.add(31) = _mm_sub_epi32(*x1.add(0), *x1.add(31));
    *x2.add(1) = _mm_add_epi32(*x1.add(1), *x1.add(30));
    *x2.add(30) = _mm_sub_epi32(*x1.add(1), *x1.add(30));
    *x2.add(2) = _mm_add_epi32(*x1.add(2), *x1.add(29));
    *x2.add(29) = _mm_sub_epi32(*x1.add(2), *x1.add(29));
    *x2.add(3) = _mm_add_epi32(*x1.add(3), *x1.add(28));
    *x2.add(28) = _mm_sub_epi32(*x1.add(3), *x1.add(28));
    *x2.add(4) = _mm_add_epi32(*x1.add(4), *x1.add(27));
    *x2.add(27) = _mm_sub_epi32(*x1.add(4), *x1.add(27));
    *x2.add(5) = _mm_add_epi32(*x1.add(5), *x1.add(26));
    *x2.add(26) = _mm_sub_epi32(*x1.add(5), *x1.add(26));
    *x2.add(6) = _mm_add_epi32(*x1.add(6), *x1.add(25));
    *x2.add(25) = _mm_sub_epi32(*x1.add(6), *x1.add(25));
    *x2.add(7) = _mm_add_epi32(*x1.add(7), *x1.add(24));
    *x2.add(24) = _mm_sub_epi32(*x1.add(7), *x1.add(24));
    *x2.add(8) = _mm_add_epi32(*x1.add(8), *x1.add(23));
    *x2.add(23) = _mm_sub_epi32(*x1.add(8), *x1.add(23));
    *x2.add(9) = _mm_add_epi32(*x1.add(9), *x1.add(22));
    *x2.add(22) = _mm_sub_epi32(*x1.add(9), *x1.add(22));
    *x2.add(10) = _mm_add_epi32(*x1.add(10), *x1.add(21));
    *x2.add(21) = _mm_sub_epi32(*x1.add(10), *x1.add(21));
    *x2.add(11) = _mm_add_epi32(*x1.add(11), *x1.add(20));
    *x2.add(20) = _mm_sub_epi32(*x1.add(11), *x1.add(20));
    *x2.add(12) = _mm_add_epi32(*x1.add(12), *x1.add(19));
    *x2.add(19) = _mm_sub_epi32(*x1.add(12), *x1.add(19));
    *x2.add(13) = _mm_add_epi32(*x1.add(13), *x1.add(18));
    *x2.add(18) = _mm_sub_epi32(*x1.add(13), *x1.add(18));
    *x2.add(14) = _mm_add_epi32(*x1.add(14), *x1.add(17));
    *x2.add(17) = _mm_sub_epi32(*x1.add(14), *x1.add(17));
    *x2.add(15) = _mm_add_epi32(*x1.add(15), *x1.add(16));
    *x2.add(16) = _mm_sub_epi32(*x1.add(15), *x1.add(16));
    *x2.add(32) = *x1.add(32);
    *x2.add(33) = *x1.add(33);
    *x2.add(34) = *x1.add(34);
    *x2.add(35) = *x1.add(35);
    *x2.add(36) = *x1.add(36);
    *x2.add(37) = *x1.add(37);
    *x2.add(38) = *x1.add(38);
    *x2.add(39) = *x1.add(39);
    btf_32_type0_sse4_1_new!(cospi_m32, cospi_p32, *x1.add(40), *x1.add(55), *x2.add(40),
                             *x2.add(55), __rounding, cos_bit);
    btf_32_type0_sse4_1_new!(cospi_m32, cospi_p32, *x1.add(41), *x1.add(54), *x2.add(41),
                             *x2.add(54), __rounding, cos_bit);
    btf_32_type0_sse4_1_new!(cospi_m32, cospi_p32, *x1.add(42), *x1.add(53), *x2.add(42),
                             *x2.add(53), __rounding, cos_bit);
    btf_32_type0_sse4_1_new!(cospi_m32, cospi_p32, *x1.add(43), *x1.add(52), *x2.add(43),
                             *x2.add(52), __rounding, cos_bit);
    btf_32_type0_sse4_1_new!(cospi_m32, cospi_p32, *x1.add(44), *x1.add(51), *x2.add(44),
                             *x2.add(51), __rounding, cos_bit);
    btf_32_type0_sse4_1_new!(cospi_m32, cospi_p32, *x1.add(45), *x1.add(50), *x2.add(45),
                             *x2.add(50), __rounding, cos_bit);
    btf_32_type0_sse4_1_new!(cospi_m32, cospi_p32, *x1.add(46), *x1.add(49), *x2.add(46),
                             *x2.add(49), __rounding, cos_bit);
    btf_32_type0_sse4_1_new!(cospi_m32, cospi_p32, *x1.add(47), *x1.add(48), *x2.add(47),
                             *x2.add(48), __rounding, cos_bit);
    *x2.add(56) = *x1.add(56);
    *x2.add(57) = *x1.add(57);
    *x2.add(58) = *x1.add(58);
    *x2.add(59) = *x1.add(59);
    *x2.add(60) = *x1.add(60);
    *x2.add(61) = *x1.add(61);
    *x2.add(62) = *x1.add(62);
    *x2.add(63) = *x1.add(63);

    // stage 3
    let mut x3_: [__m128i; 64] = core::mem::zeroed();
    let x3: *mut __m128i = x3_.as_mut_ptr();
    *x3.add(0) = _mm_add_epi32(*x2.add(0), *x2.add(15));
    *x3.add(15) = _mm_sub_epi32(*x2.add(0), *x2.add(15));
    *x3.add(1) = _mm_add_epi32(*x2.add(1), *x2.add(14));
    *x3.add(14) = _mm_sub_epi32(*x2.add(1), *x2.add(14));
    *x3.add(2) = _mm_add_epi32(*x2.add(2), *x2.add(13));
    *x3.add(13) = _mm_sub_epi32(*x2.add(2), *x2.add(13));
    *x3.add(3) = _mm_add_epi32(*x2.add(3), *x2.add(12));
    *x3.add(12) = _mm_sub_epi32(*x2.add(3), *x2.add(12));
    *x3.add(4) = _mm_add_epi32(*x2.add(4), *x2.add(11));
    *x3.add(11) = _mm_sub_epi32(*x2.add(4), *x2.add(11));
    *x3.add(5) = _mm_add_epi32(*x2.add(5), *x2.add(10));
    *x3.add(10) = _mm_sub_epi32(*x2.add(5), *x2.add(10));
    *x3.add(6) = _mm_add_epi32(*x2.add(6), *x2.add(9));
    *x3.add(9) = _mm_sub_epi32(*x2.add(6), *x2.add(9));
    *x3.add(7) = _mm_add_epi32(*x2.add(7), *x2.add(8));
    *x3.add(8) = _mm_sub_epi32(*x2.add(7), *x2.add(8));
    *x3.add(16) = *x2.add(16);
    *x3.add(17) = *x2.add(17);
    *x3.add(18) = *x2.add(18);
    *x3.add(19) = *x2.add(19);
    btf_32_type0_sse4_1_new!(cospi_m32, cospi_p32, *x2.add(20), *x2.add(27), *x3.add(20),
                             *x3.add(27), __rounding, cos_bit);
    btf_32_type0_sse4_1_new!(cospi_m32, cospi_p32, *x2.add(21), *x2.add(26), *x3.add(21),
                             *x3.add(26), __rounding, cos_bit);
    btf_32_type0_sse4_1_new!(cospi_m32, cospi_p32, *x2.add(22), *x2.add(25), *x3.add(22),
                             *x3.add(25), __rounding, cos_bit);
    btf_32_type0_sse4_1_new!(cospi_m32, cospi_p32, *x2.add(23), *x2.add(24), *x3.add(23),
                             *x3.add(24), __rounding, cos_bit);
    *x3.add(28) = *x2.add(28);
    *x3.add(29) = *x2.add(29);
    *x3.add(30) = *x2.add(30);
    *x3.add(31) = *x2.add(31);
    *x3.add(32) = _mm_add_epi32(*x2.add(32), *x2.add(47));
    *x3.add(47) = _mm_sub_epi32(*x2.add(32), *x2.add(47));
    *x3.add(33) = _mm_add_epi32(*x2.add(33), *x2.add(46));
    *x3.add(46) = _mm_sub_epi32(*x2.add(33), *x2.add(46));
    *x3.add(34) = _mm_add_epi32(*x2.add(34), *x2.add(45));
    *x3.add(45) = _mm_sub_epi32(*x2.add(34), *x2.add(45));
    *x3.add(35) = _mm_add_epi32(*x2.add(35), *x2.add(44));
    *x3.add(44) = _mm_sub_epi32(*x2.add(35), *x2.add(44));
    *x3.add(36) = _mm_add_epi32(*x2.add(36), *x2.add(43));
    *x3.add(43) = _mm_sub_epi32(*x2.add(36), *x2.add(43));
    *x3.add(37) = _mm_add_epi32(*x2.add(37), *x2.add(42));
    *x3.add(42) = _mm_sub_epi32(*x2.add(37), *x2.add(42));
    *x3.add(38) = _mm_add_epi32(*x2.add(38), *x2.add(41));
    *x3.add(41) = _mm_sub_epi32(*x2.add(38), *x2.add(41));
    *x3.add(39) = _mm_add_epi32(*x2.add(39), *x2.add(40));
    *x3.add(40) = _mm_sub_epi32(*x2.add(39), *x2.add(40));
    *x3.add(48) = _mm_sub_epi32(*x2.add(63), *x2.add(48));
    *x3.add(63) = _mm_add_epi32(*x2.add(63), *x2.add(48));
    *x3.add(49) = _mm_sub_epi32(*x2.add(62), *x2.add(49));
    *x3.add(62) = _mm_add_epi32(*x2.add(62), *x2.add(49));
    *x3.add(50) = _mm_sub_epi32(*x2.add(61), *x2.add(50));
    *x3.add(61) = _mm_add_epi32(*x2.add(61), *x2.add(50));
    *x3.add(51) = _mm_sub_epi32(*x2.add(60), *x2.add(51));
    *x3.add(60) = _mm_add_epi32(*x2.add(60), *x2.add(51));
    *x3.add(52) = _mm_sub_epi32(*x2.add(59), *x2.add(52));
    *x3.add(59) = _mm_add_epi32(*x2.add(59), *x2.add(52));
    *x3.add(53) = _mm_sub_epi32(*x2.add(58), *x2.add(53));
    *x3.add(58) = _mm_add_epi32(*x2.add(58), *x2.add(53));
    *x3.add(54) = _mm_sub_epi32(*x2.add(57), *x2.add(54));
    *x3.add(57) = _mm_add_epi32(*x2.add(57), *x2.add(54));
    *x3.add(55) = _mm_sub_epi32(*x2.add(56), *x2.add(55));
    *x3.add(56) = _mm_add_epi32(*x2.add(56), *x2.add(55));

    // stage 4
    let mut x4_: [__m128i; 64] = core::mem::zeroed();
    let x4: *mut __m128i = x4_.as_mut_ptr();
    *x4.add(0) = _mm_add_epi32(*x3.add(0), *x3.add(7));
    *x4.add(7) = _mm_sub_epi32(*x3.add(0), *x3.add(7));
    *x4.add(1) = _mm_add_epi32(*x3.add(1), *x3.add(6));
    *x4.add(6) = _mm_sub_epi32(*x3.add(1), *x3.add(6));
    *x4.add(2) = _mm_add_epi32(*x3.add(2), *x3.add(5));
    *x4.add(5) = _mm_sub_epi32(*x3.add(2), *x3.add(5));
    *x4.add(3) = _mm_add_epi32(*x3.add(3), *x3.add(4));
    *x4.add(4) = _mm_sub_epi32(*x3.add(3), *x3.add(4));
    *x4.add(8) = *x3.add(8);
    *x4.add(9) = *x3.add(9);
    btf_32_type0_sse4_1_new!(cospi_m32, cospi_p32, *x3.add(10), *x3.add(13), *x4.add(10),
                             *x4.add(13), __rounding, cos_bit);
    btf_32_type0_sse4_1_new!(cospi_m32, cospi_p32, *x3.add(11), *x3.add(12), *x4.add(11),
                             *x4.add(12), __rounding, cos_bit);
    *x4.add(14) = *x3.add(14);
    *x4.add(15) = *x3.add(15);
    *x4.add(16) = _mm_add_epi32(*x3.add(16), *x3.add(23));
    *x4.add(23) = _mm_sub_epi32(*x3.add(16), *x3.add(23));
    *x4.add(17) = _mm_add_epi32(*x3.add(17), *x3.add(22));
    *x4.add(22) = _mm_sub_epi32(*x3.add(17), *x3.add(22));
    *x4.add(18) = _mm_add_epi32(*x3.add(18), *x3.add(21));
    *x4.add(21) = _mm_sub_epi32(*x3.add(18), *x3.add(21));
    *x4.add(19) = _mm_add_epi32(*x3.add(19), *x3.add(20));
    *x4.add(20) = _mm_sub_epi32(*x3.add(19), *x3.add(20));
    *x4.add(24) = _mm_sub_epi32(*x3.add(31), *x3.add(24));
    *x4.add(31) = _mm_add_epi32(*x3.add(31), *x3.add(24));
    *x4.add(25) = _mm_sub_epi32(*x3.add(30), *x3.add(25));
    *x4.add(30) = _mm_add_epi32(*x3.add(30), *x3.add(25));
    *x4.add(26) = _mm_sub_epi32(*x3.add(29), *x3.add(26));
    *x4.add(29) = _mm_add_epi32(*x3.add(29), *x3.add(26));
    *x4.add(27) = _mm_sub_epi32(*x3.add(28), *x3.add(27));
    *x4.add(28) = _mm_add_epi32(*x3.add(28), *x3.add(27));
    *x4.add(32) = *x3.add(32);
    *x4.add(33) = *x3.add(33);
    *x4.add(34) = *x3.add(34);
    *x4.add(35) = *x3.add(35);
    btf_32_type0_sse4_1_new!(cospi_m16, cospi_p48, *x3.add(36), *x3.add(59), *x4.add(36),
                             *x4.add(59), __rounding, cos_bit);
    btf_32_type0_sse4_1_new!(cospi_m16, cospi_p48, *x3.add(37), *x3.add(58), *x4.add(37),
                             *x4.add(58), __rounding, cos_bit);
    btf_32_type0_sse4_1_new!(cospi_m16, cospi_p48, *x3.add(38), *x3.add(57), *x4.add(38),
                             *x4.add(57), __rounding, cos_bit);
    btf_32_type0_sse4_1_new!(cospi_m16, cospi_p48, *x3.add(39), *x3.add(56), *x4.add(39),
                             *x4.add(56), __rounding, cos_bit);
    btf_32_type0_sse4_1_new!(cospi_m48, cospi_m16, *x3.add(40), *x3.add(55), *x4.add(40),
                             *x4.add(55), __rounding, cos_bit);
    btf_32_type0_sse4_1_new!(cospi_m48, cospi_m16, *x3.add(41), *x3.add(54), *x4.add(41),
                             *x4.add(54), __rounding, cos_bit);
    btf_32_type0_sse4_1_new!(cospi_m48, cospi_m16, *x3.add(42), *x3.add(53), *x4.add(42),
                             *x4.add(53), __rounding, cos_bit);
    btf_32_type0_sse4_1_new!(cospi_m48, cospi_m16, *x3.add(43), *x3.add(52), *x4.add(43),
                             *x4.add(52), __rounding, cos_bit);
    *x4.add(44) = *x3.add(44);
    *x4.add(45) = *x3.add(45);
    *x4.add(46) = *x3.add(46);
    *x4.add(47) = *x3.add(47);
    *x4.add(48) = *x3.add(48);
    *x4.add(49) = *x3.add(49);
    *x4.add(50) = *x3.add(50);
    *x4.add(51) = *x3.add(51);
    *x4.add(60) = *x3.add(60);
    *x4.add(61) = *x3.add(61);
    *x4.add(62) = *x3.add(62);
    *x4.add(63) = *x3.add(63);

    // stage 5
    let mut x5_: [__m128i; 64] = core::mem::zeroed();
    let x5: *mut __m128i = x5_.as_mut_ptr();
    *x5.add(0) = _mm_add_epi32(*x4.add(0), *x4.add(3));
    *x5.add(3) = _mm_sub_epi32(*x4.add(0), *x4.add(3));
    *x5.add(1) = _mm_add_epi32(*x4.add(1), *x4.add(2));
    *x5.add(2) = _mm_sub_epi32(*x4.add(1), *x4.add(2));
    *x5.add(4) = *x4.add(4);
    btf_32_type0_sse4_1_new!(cospi_m32, cospi_p32, *x4.add(5), *x4.add(6), *x5.add(5), *x5.add(6),
                             __rounding, cos_bit);
    *x5.add(7) = *x4.add(7);
    *x5.add(8) = _mm_add_epi32(*x4.add(8), *x4.add(11));
    *x5.add(11) = _mm_sub_epi32(*x4.add(8), *x4.add(11));
    *x5.add(9) = _mm_add_epi32(*x4.add(9), *x4.add(10));
    *x5.add(10) = _mm_sub_epi32(*x4.add(9), *x4.add(10));
    *x5.add(12) = _mm_sub_epi32(*x4.add(15), *x4.add(12));
    *x5.add(15) = _mm_add_epi32(*x4.add(15), *x4.add(12));
    *x5.add(13) = _mm_sub_epi32(*x4.add(14), *x4.add(13));
    *x5.add(14) = _mm_add_epi32(*x4.add(14), *x4.add(13));
    *x5.add(16) = *x4.add(16);
    *x5.add(17) = *x4.add(17);
    btf_32_type0_sse4_1_new!(cospi_m16, cospi_p48, *x4.add(18), *x4.add(29), *x5.add(18),
                             *x5.add(29), __rounding, cos_bit);
    btf_32_type0_sse4_1_new!(cospi_m16, cospi_p48, *x4.add(19), *x4.add(28), *x5.add(19),
                             *x5.add(28), __rounding, cos_bit);
    btf_32_type0_sse4_1_new!(cospi_m48, cospi_m16, *x4.add(20), *x4.add(27), *x5.add(20),
                             *x5.add(27), __rounding, cos_bit);
    btf_32_type0_sse4_1_new!(cospi_m48, cospi_m16, *x4.add(21), *x4.add(26), *x5.add(21),
                             *x5.add(26), __rounding, cos_bit);
    *x5.add(22) = *x4.add(22);
    *x5.add(23) = *x4.add(23);
    *x5.add(24) = *x4.add(24);
    *x5.add(25) = *x4.add(25);
    *x5.add(30) = *x4.add(30);
    *x5.add(31) = *x4.add(31);
    *x5.add(32) = _mm_add_epi32(*x4.add(32), *x4.add(39));
    *x5.add(39) = _mm_sub_epi32(*x4.add(32), *x4.add(39));
    *x5.add(33) = _mm_add_epi32(*x4.add(33), *x4.add(38));
    *x5.add(38) = _mm_sub_epi32(*x4.add(33), *x4.add(38));
    *x5.add(34) = _mm_add_epi32(*x4.add(34), *x4.add(37));
    *x5.add(37) = _mm_sub_epi32(*x4.add(34), *x4.add(37));
    *x5.add(35) = _mm_add_epi32(*x4.add(35), *x4.add(36));
    *x5.add(36) = _mm_sub_epi32(*x4.add(35), *x4.add(36));
    *x5.add(40) = _mm_sub_epi32(*x4.add(47), *x4.add(40));
    *x5.add(47) = _mm_add_epi32(*x4.add(47), *x4.add(40));
    *x5.add(41) = _mm_sub_epi32(*x4.add(46), *x4.add(41));
    *x5.add(46) = _mm_add_epi32(*x4.add(46), *x4.add(41));
    *x5.add(42) = _mm_sub_epi32(*x4.add(45), *x4.add(42));
    *x5.add(45) = _mm_add_epi32(*x4.add(45), *x4.add(42));
    *x5.add(43) = _mm_sub_epi32(*x4.add(44), *x4.add(43));
    *x5.add(44) = _mm_add_epi32(*x4.add(44), *x4.add(43));
    *x5.add(48) = _mm_add_epi32(*x4.add(48), *x4.add(55));
    *x5.add(55) = _mm_sub_epi32(*x4.add(48), *x4.add(55));
    *x5.add(49) = _mm_add_epi32(*x4.add(49), *x4.add(54));
    *x5.add(54) = _mm_sub_epi32(*x4.add(49), *x4.add(54));
    *x5.add(50) = _mm_add_epi32(*x4.add(50), *x4.add(53));
    *x5.add(53) = _mm_sub_epi32(*x4.add(50), *x4.add(53));
    *x5.add(51) = _mm_add_epi32(*x4.add(51), *x4.add(52));
    *x5.add(52) = _mm_sub_epi32(*x4.add(51), *x4.add(52));
    *x5.add(56) = _mm_sub_epi32(*x4.add(63), *x4.add(56));
    *x5.add(63) = _mm_add_epi32(*x4.add(63), *x4.add(56));
    *x5.add(57) = _mm_sub_epi32(*x4.add(62), *x4.add(57));
    *x5.add(62) = _mm_add_epi32(*x4.add(62), *x4.add(57));
    *x5.add(58) = _mm_sub_epi32(*x4.add(61), *x4.add(58));
    *x5.add(61) = _mm_add_epi32(*x4.add(61), *x4.add(58));
    *x5.add(59) = _mm_sub_epi32(*x4.add(60), *x4.add(59));
    *x5.add(60) = _mm_add_epi32(*x4.add(60), *x4.add(59));

    // stage 6
    let mut x6_: [__m128i; 64] = core::mem::zeroed();
    let x6: *mut __m128i = x6_.as_mut_ptr();
    btf_32_type0_sse4_1_new!(cospi_p32, cospi_p32, *x5.add(0), *x5.add(1), *x6.add(0), *x6.add(1),
                             __rounding, cos_bit);
    btf_32_type1_sse4_1_new!(cospi_p48, cospi_p16, *x5.add(2), *x5.add(3), *x6.add(2), *x6.add(3),
                             __rounding, cos_bit);
    *x6.add(4) = _mm_add_epi32(*x5.add(4), *x5.add(5));
    *x6.add(5) = _mm_sub_epi32(*x5.add(4), *x5.add(5));
    *x6.add(6) = _mm_sub_epi32(*x5.add(7), *x5.add(6));
    *x6.add(7) = _mm_add_epi32(*x5.add(7), *x5.add(6));
    *x6.add(8) = *x5.add(8);
    btf_32_type0_sse4_1_new!(cospi_m16, cospi_p48, *x5.add(9), *x5.add(14), *x6.add(9), *x6.add(14),
                             __rounding, cos_bit);
    btf_32_type0_sse4_1_new!(cospi_m48, cospi_m16, *x5.add(10), *x5.add(13), *x6.add(10),
                             *x6.add(13), __rounding, cos_bit);
    *x6.add(11) = *x5.add(11);
    *x6.add(12) = *x5.add(12);
    *x6.add(15) = *x5.add(15);
    *x6.add(16) = _mm_add_epi32(*x5.add(16), *x5.add(19));
    *x6.add(19) = _mm_sub_epi32(*x5.add(16), *x5.add(19));
    *x6.add(17) = _mm_add_epi32(*x5.add(17), *x5.add(18));
    *x6.add(18) = _mm_sub_epi32(*x5.add(17), *x5.add(18));
    *x6.add(20) = _mm_sub_epi32(*x5.add(23), *x5.add(20));
    *x6.add(23) = _mm_add_epi32(*x5.add(23), *x5.add(20));
    *x6.add(21) = _mm_sub_epi32(*x5.add(22), *x5.add(21));
    *x6.add(22) = _mm_add_epi32(*x5.add(22), *x5.add(21));
    *x6.add(24) = _mm_add_epi32(*x5.add(24), *x5.add(27));
    *x6.add(27) = _mm_sub_epi32(*x5.add(24), *x5.add(27));
    *x6.add(25) = _mm_add_epi32(*x5.add(25), *x5.add(26));
    *x6.add(26) = _mm_sub_epi32(*x5.add(25), *x5.add(26));
    *x6.add(28) = _mm_sub_epi32(*x5.add(31), *x5.add(28));
    *x6.add(31) = _mm_add_epi32(*x5.add(31), *x5.add(28));
    *x6.add(29) = _mm_sub_epi32(*x5.add(30), *x5.add(29));
    *x6.add(30) = _mm_add_epi32(*x5.add(30), *x5.add(29));
    *x6.add(32) = *x5.add(32);
    *x6.add(33) = *x5.add(33);
    btf_32_type0_sse4_1_new!(cospi_m08, cospi_p56, *x5.add(34), *x5.add(61), *x6.add(34),
                             *x6.add(61), __rounding, cos_bit);
    btf_32_type0_sse4_1_new!(cospi_m08, cospi_p56, *x5.add(35), *x5.add(60), *x6.add(35),
                             *x6.add(60), __rounding, cos_bit);
    btf_32_type0_sse4_1_new!(cospi_m56, cospi_m08, *x5.add(36), *x5.add(59), *x6.add(36),
                             *x6.add(59), __rounding, cos_bit);
    btf_32_type0_sse4_1_new!(cospi_m56, cospi_m08, *x5.add(37), *x5.add(58), *x6.add(37),
                             *x6.add(58), __rounding, cos_bit);
    *x6.add(38) = *x5.add(38);
    *x6.add(39) = *x5.add(39);
    *x6.add(40) = *x5.add(40);
    *x6.add(41) = *x5.add(41);
    btf_32_type0_sse4_1_new!(cospi_m40, cospi_p24, *x5.add(42), *x5.add(53), *x6.add(42),
                             *x6.add(53), __rounding, cos_bit);
    btf_32_type0_sse4_1_new!(cospi_m40, cospi_p24, *x5.add(43), *x5.add(52), *x6.add(43),
                             *x6.add(52), __rounding, cos_bit);
    btf_32_type0_sse4_1_new!(cospi_m24, cospi_m40, *x5.add(44), *x5.add(51), *x6.add(44),
                             *x6.add(51), __rounding, cos_bit);
    btf_32_type0_sse4_1_new!(cospi_m24, cospi_m40, *x5.add(45), *x5.add(50), *x6.add(45),
                             *x6.add(50), __rounding, cos_bit);
    *x6.add(46) = *x5.add(46);
    *x6.add(47) = *x5.add(47);
    *x6.add(48) = *x5.add(48);
    *x6.add(49) = *x5.add(49);
    *x6.add(54) = *x5.add(54);
    *x6.add(55) = *x5.add(55);
    *x6.add(56) = *x5.add(56);
    *x6.add(57) = *x5.add(57);
    *x6.add(62) = *x5.add(62);
    *x6.add(63) = *x5.add(63);

    // stage 7
    let mut x7_: [__m128i; 64] = core::mem::zeroed();
    let x7: *mut __m128i = x7_.as_mut_ptr();
    *x7.add(0) = *x6.add(0);
    *x7.add(1) = *x6.add(1);
    *x7.add(2) = *x6.add(2);
    *x7.add(3) = *x6.add(3);
    btf_32_type1_sse4_1_new!(cospi_p56, cospi_p08, *x6.add(4), *x6.add(7), *x7.add(4), *x7.add(7),
                             __rounding, cos_bit);
    btf_32_type1_sse4_1_new!(cospi_p24, cospi_p40, *x6.add(5), *x6.add(6), *x7.add(5), *x7.add(6),
                             __rounding, cos_bit);
    *x7.add(8) = _mm_add_epi32(*x6.add(8), *x6.add(9));
    *x7.add(9) = _mm_sub_epi32(*x6.add(8), *x6.add(9));
    *x7.add(10) = _mm_sub_epi32(*x6.add(11), *x6.add(10));
    *x7.add(11) = _mm_add_epi32(*x6.add(11), *x6.add(10));
    *x7.add(12) = _mm_add_epi32(*x6.add(12), *x6.add(13));
    *x7.add(13) = _mm_sub_epi32(*x6.add(12), *x6.add(13));
    *x7.add(14) = _mm_sub_epi32(*x6.add(15), *x6.add(14));
    *x7.add(15) = _mm_add_epi32(*x6.add(15), *x6.add(14));
    *x7.add(16) = *x6.add(16);
    btf_32_type0_sse4_1_new!(cospi_m08, cospi_p56, *x6.add(17), *x6.add(30), *x7.add(17),
                             *x7.add(30), __rounding, cos_bit);
    btf_32_type0_sse4_1_new!(cospi_m56, cospi_m08, *x6.add(18), *x6.add(29), *x7.add(18),
                             *x7.add(29), __rounding, cos_bit);
    *x7.add(19) = *x6.add(19);
    *x7.add(20) = *x6.add(20);
    btf_32_type0_sse4_1_new!(cospi_m40, cospi_p24, *x6.add(21), *x6.add(26), *x7.add(21),
                             *x7.add(26), __rounding, cos_bit);
    btf_32_type0_sse4_1_new!(cospi_m24, cospi_m40, *x6.add(22), *x6.add(25), *x7.add(22),
                             *x7.add(25), __rounding, cos_bit);
    *x7.add(23) = *x6.add(23);
    *x7.add(24) = *x6.add(24);
    *x7.add(27) = *x6.add(27);
    *x7.add(28) = *x6.add(28);
    *x7.add(31) = *x6.add(31);
    *x7.add(32) = _mm_add_epi32(*x6.add(32), *x6.add(35));
    *x7.add(35) = _mm_sub_epi32(*x6.add(32), *x6.add(35));
    *x7.add(33) = _mm_add_epi32(*x6.add(33), *x6.add(34));
    *x7.add(34) = _mm_sub_epi32(*x6.add(33), *x6.add(34));
    *x7.add(36) = _mm_sub_epi32(*x6.add(39), *x6.add(36));
    *x7.add(39) = _mm_add_epi32(*x6.add(39), *x6.add(36));
    *x7.add(37) = _mm_sub_epi32(*x6.add(38), *x6.add(37));
    *x7.add(38) = _mm_add_epi32(*x6.add(38), *x6.add(37));
    *x7.add(40) = _mm_add_epi32(*x6.add(40), *x6.add(43));
    *x7.add(43) = _mm_sub_epi32(*x6.add(40), *x6.add(43));
    *x7.add(41) = _mm_add_epi32(*x6.add(41), *x6.add(42));
    *x7.add(42) = _mm_sub_epi32(*x6.add(41), *x6.add(42));
    *x7.add(44) = _mm_sub_epi32(*x6.add(47), *x6.add(44));
    *x7.add(47) = _mm_add_epi32(*x6.add(47), *x6.add(44));
    *x7.add(45) = _mm_sub_epi32(*x6.add(46), *x6.add(45));
    *x7.add(46) = _mm_add_epi32(*x6.add(46), *x6.add(45));
    *x7.add(48) = _mm_add_epi32(*x6.add(48), *x6.add(51));
    *x7.add(51) = _mm_sub_epi32(*x6.add(48), *x6.add(51));
    *x7.add(49) = _mm_add_epi32(*x6.add(49), *x6.add(50));
    *x7.add(50) = _mm_sub_epi32(*x6.add(49), *x6.add(50));
    *x7.add(52) = _mm_sub_epi32(*x6.add(55), *x6.add(52));
    *x7.add(55) = _mm_add_epi32(*x6.add(55), *x6.add(52));
    *x7.add(53) = _mm_sub_epi32(*x6.add(54), *x6.add(53));
    *x7.add(54) = _mm_add_epi32(*x6.add(54), *x6.add(53));
    *x7.add(56) = _mm_add_epi32(*x6.add(56), *x6.add(59));
    *x7.add(59) = _mm_sub_epi32(*x6.add(56), *x6.add(59));
    *x7.add(57) = _mm_add_epi32(*x6.add(57), *x6.add(58));
    *x7.add(58) = _mm_sub_epi32(*x6.add(57), *x6.add(58));
    *x7.add(60) = _mm_sub_epi32(*x6.add(63), *x6.add(60));
    *x7.add(63) = _mm_add_epi32(*x6.add(63), *x6.add(60));
    *x7.add(61) = _mm_sub_epi32(*x6.add(62), *x6.add(61));
    *x7.add(62) = _mm_add_epi32(*x6.add(62), *x6.add(61));

    // stage 8
    let mut x8_: [__m128i; 64] = core::mem::zeroed();
    let x8: *mut __m128i = x8_.as_mut_ptr();
    *x8.add(0) = *x7.add(0);
    *x8.add(1) = *x7.add(1);
    *x8.add(2) = *x7.add(2);
    *x8.add(3) = *x7.add(3);
    *x8.add(4) = *x7.add(4);
    *x8.add(5) = *x7.add(5);
    *x8.add(6) = *x7.add(6);
    *x8.add(7) = *x7.add(7);
    btf_32_type1_sse4_1_new!(cospi_p60, cospi_p04, *x7.add(8), *x7.add(15), *x8.add(8), *x8.add(15),
                             __rounding, cos_bit);
    btf_32_type1_sse4_1_new!(cospi_p28, cospi_p36, *x7.add(9), *x7.add(14), *x8.add(9), *x8.add(14),
                             __rounding, cos_bit);
    btf_32_type1_sse4_1_new!(cospi_p44, cospi_p20, *x7.add(10), *x7.add(13), *x8.add(10),
                             *x8.add(13), __rounding, cos_bit);
    btf_32_type1_sse4_1_new!(cospi_p12, cospi_p52, *x7.add(11), *x7.add(12), *x8.add(11),
                             *x8.add(12), __rounding, cos_bit);
    *x8.add(16) = _mm_add_epi32(*x7.add(16), *x7.add(17));
    *x8.add(17) = _mm_sub_epi32(*x7.add(16), *x7.add(17));
    *x8.add(18) = _mm_sub_epi32(*x7.add(19), *x7.add(18));
    *x8.add(19) = _mm_add_epi32(*x7.add(19), *x7.add(18));
    *x8.add(20) = _mm_add_epi32(*x7.add(20), *x7.add(21));
    *x8.add(21) = _mm_sub_epi32(*x7.add(20), *x7.add(21));
    *x8.add(22) = _mm_sub_epi32(*x7.add(23), *x7.add(22));
    *x8.add(23) = _mm_add_epi32(*x7.add(23), *x7.add(22));
    *x8.add(24) = _mm_add_epi32(*x7.add(24), *x7.add(25));
    *x8.add(25) = _mm_sub_epi32(*x7.add(24), *x7.add(25));
    *x8.add(26) = _mm_sub_epi32(*x7.add(27), *x7.add(26));
    *x8.add(27) = _mm_add_epi32(*x7.add(27), *x7.add(26));
    *x8.add(28) = _mm_add_epi32(*x7.add(28), *x7.add(29));
    *x8.add(29) = _mm_sub_epi32(*x7.add(28), *x7.add(29));
    *x8.add(30) = _mm_sub_epi32(*x7.add(31), *x7.add(30));
    *x8.add(31) = _mm_add_epi32(*x7.add(31), *x7.add(30));
    *x8.add(32) = *x7.add(32);
    btf_32_type0_sse4_1_new!(cospi_m04, cospi_p60, *x7.add(33), *x7.add(62), *x8.add(33),
                             *x8.add(62), __rounding, cos_bit);
    btf_32_type0_sse4_1_new!(cospi_m60, cospi_m04, *x7.add(34), *x7.add(61), *x8.add(34),
                             *x8.add(61), __rounding, cos_bit);
    *x8.add(35) = *x7.add(35);
    *x8.add(36) = *x7.add(36);
    btf_32_type0_sse4_1_new!(cospi_m36, cospi_p28, *x7.add(37), *x7.add(58), *x8.add(37),
                             *x8.add(58), __rounding, cos_bit);
    btf_32_type0_sse4_1_new!(cospi_m28, cospi_m36, *x7.add(38), *x7.add(57), *x8.add(38),
                             *x8.add(57), __rounding, cos_bit);
    *x8.add(39) = *x7.add(39);
    *x8.add(40) = *x7.add(40);
    btf_32_type0_sse4_1_new!(cospi_m20, cospi_p44, *x7.add(41), *x7.add(54), *x8.add(41),
                             *x8.add(54), __rounding, cos_bit);
    btf_32_type0_sse4_1_new!(cospi_m44, cospi_m20, *x7.add(42), *x7.add(53), *x8.add(42),
                             *x8.add(53), __rounding, cos_bit);
    *x8.add(43) = *x7.add(43);
    *x8.add(44) = *x7.add(44);
    btf_32_type0_sse4_1_new!(cospi_m52, cospi_p12, *x7.add(45), *x7.add(50), *x8.add(45),
                             *x8.add(50), __rounding, cos_bit);
    btf_32_type0_sse4_1_new!(cospi_m12, cospi_m52, *x7.add(46), *x7.add(49), *x8.add(46),
                             *x8.add(49), __rounding, cos_bit);
    *x8.add(47) = *x7.add(47);
    *x8.add(48) = *x7.add(48);
    *x8.add(51) = *x7.add(51);
    *x8.add(52) = *x7.add(52);
    *x8.add(55) = *x7.add(55);
    *x8.add(56) = *x7.add(56);
    *x8.add(59) = *x7.add(59);
    *x8.add(60) = *x7.add(60);
    *x8.add(63) = *x7.add(63);

    // stage 9
    let mut x9_: [__m128i; 64] = core::mem::zeroed();
    let x9: *mut __m128i = x9_.as_mut_ptr();
    *x9.add(0) = *x8.add(0);
    *x9.add(1) = *x8.add(1);
    *x9.add(2) = *x8.add(2);
    *x9.add(3) = *x8.add(3);
    *x9.add(4) = *x8.add(4);
    *x9.add(5) = *x8.add(5);
    *x9.add(6) = *x8.add(6);
    *x9.add(7) = *x8.add(7);
    *x9.add(8) = *x8.add(8);
    *x9.add(9) = *x8.add(9);
    *x9.add(10) = *x8.add(10);
    *x9.add(11) = *x8.add(11);
    *x9.add(12) = *x8.add(12);
    *x9.add(13) = *x8.add(13);
    *x9.add(14) = *x8.add(14);
    *x9.add(15) = *x8.add(15);
    btf_32_type1_sse4_1_new!(cospi_p62, cospi_p02, *x8.add(16), *x8.add(31), *x9.add(16),
                             *x9.add(31), __rounding, cos_bit);
    btf_32_type1_sse4_1_new!(cospi_p30, cospi_p34, *x8.add(17), *x8.add(30), *x9.add(17),
                             *x9.add(30), __rounding, cos_bit);
    btf_32_type1_sse4_1_new!(cospi_p46, cospi_p18, *x8.add(18), *x8.add(29), *x9.add(18),
                             *x9.add(29), __rounding, cos_bit);
    btf_32_type1_sse4_1_new!(cospi_p14, cospi_p50, *x8.add(19), *x8.add(28), *x9.add(19),
                             *x9.add(28), __rounding, cos_bit);
    btf_32_type1_sse4_1_new!(cospi_p54, cospi_p10, *x8.add(20), *x8.add(27), *x9.add(20),
                             *x9.add(27), __rounding, cos_bit);
    btf_32_type1_sse4_1_new!(cospi_p22, cospi_p42, *x8.add(21), *x8.add(26), *x9.add(21),
                             *x9.add(26), __rounding, cos_bit);
    btf_32_type1_sse4_1_new!(cospi_p38, cospi_p26, *x8.add(22), *x8.add(25), *x9.add(22),
                             *x9.add(25), __rounding, cos_bit);
    btf_32_type1_sse4_1_new!(cospi_p06, cospi_p58, *x8.add(23), *x8.add(24), *x9.add(23),
                             *x9.add(24), __rounding, cos_bit);
    *x9.add(32) = _mm_add_epi32(*x8.add(32), *x8.add(33));
    *x9.add(33) = _mm_sub_epi32(*x8.add(32), *x8.add(33));
    *x9.add(34) = _mm_sub_epi32(*x8.add(35), *x8.add(34));
    *x9.add(35) = _mm_add_epi32(*x8.add(35), *x8.add(34));
    *x9.add(36) = _mm_add_epi32(*x8.add(36), *x8.add(37));
    *x9.add(37) = _mm_sub_epi32(*x8.add(36), *x8.add(37));
    *x9.add(38) = _mm_sub_epi32(*x8.add(39), *x8.add(38));
    *x9.add(39) = _mm_add_epi32(*x8.add(39), *x8.add(38));
    *x9.add(40) = _mm_add_epi32(*x8.add(40), *x8.add(41));
    *x9.add(41) = _mm_sub_epi32(*x8.add(40), *x8.add(41));
    *x9.add(42) = _mm_sub_epi32(*x8.add(43), *x8.add(42));
    *x9.add(43) = _mm_add_epi32(*x8.add(43), *x8.add(42));
    *x9.add(44) = _mm_add_epi32(*x8.add(44), *x8.add(45));
    *x9.add(45) = _mm_sub_epi32(*x8.add(44), *x8.add(45));
    *x9.add(46) = _mm_sub_epi32(*x8.add(47), *x8.add(46));
    *x9.add(47) = _mm_add_epi32(*x8.add(47), *x8.add(46));
    *x9.add(48) = _mm_add_epi32(*x8.add(48), *x8.add(49));
    *x9.add(49) = _mm_sub_epi32(*x8.add(48), *x8.add(49));
    *x9.add(50) = _mm_sub_epi32(*x8.add(51), *x8.add(50));
    *x9.add(51) = _mm_add_epi32(*x8.add(51), *x8.add(50));
    *x9.add(52) = _mm_add_epi32(*x8.add(52), *x8.add(53));
    *x9.add(53) = _mm_sub_epi32(*x8.add(52), *x8.add(53));
    *x9.add(54) = _mm_sub_epi32(*x8.add(55), *x8.add(54));
    *x9.add(55) = _mm_add_epi32(*x8.add(55), *x8.add(54));
    *x9.add(56) = _mm_add_epi32(*x8.add(56), *x8.add(57));
    *x9.add(57) = _mm_sub_epi32(*x8.add(56), *x8.add(57));
    *x9.add(58) = _mm_sub_epi32(*x8.add(59), *x8.add(58));
    *x9.add(59) = _mm_add_epi32(*x8.add(59), *x8.add(58));
    *x9.add(60) = _mm_add_epi32(*x8.add(60), *x8.add(61));
    *x9.add(61) = _mm_sub_epi32(*x8.add(60), *x8.add(61));
    *x9.add(62) = _mm_sub_epi32(*x8.add(63), *x8.add(62));
    *x9.add(63) = _mm_add_epi32(*x8.add(63), *x8.add(62));

    // stage 10
    let mut x10_: [__m128i; 64] = core::mem::zeroed();
    let x10: *mut __m128i = x10_.as_mut_ptr();
    *x10.add(0) = *x9.add(0);
    *x10.add(1) = *x9.add(1);
    *x10.add(2) = *x9.add(2);
    *x10.add(3) = *x9.add(3);
    *x10.add(4) = *x9.add(4);
    *x10.add(5) = *x9.add(5);
    *x10.add(6) = *x9.add(6);
    *x10.add(7) = *x9.add(7);
    *x10.add(8) = *x9.add(8);
    *x10.add(9) = *x9.add(9);
    *x10.add(10) = *x9.add(10);
    *x10.add(11) = *x9.add(11);
    *x10.add(12) = *x9.add(12);
    *x10.add(13) = *x9.add(13);
    *x10.add(14) = *x9.add(14);
    *x10.add(15) = *x9.add(15);
    *x10.add(16) = *x9.add(16);
    *x10.add(17) = *x9.add(17);
    *x10.add(18) = *x9.add(18);
    *x10.add(19) = *x9.add(19);
    *x10.add(20) = *x9.add(20);
    *x10.add(21) = *x9.add(21);
    *x10.add(22) = *x9.add(22);
    *x10.add(23) = *x9.add(23);
    *x10.add(24) = *x9.add(24);
    *x10.add(25) = *x9.add(25);
    *x10.add(26) = *x9.add(26);
    *x10.add(27) = *x9.add(27);
    *x10.add(28) = *x9.add(28);
    *x10.add(29) = *x9.add(29);
    *x10.add(30) = *x9.add(30);
    *x10.add(31) = *x9.add(31);
    btf_32_type1_sse4_1_new!(cospi_p63, cospi_p01, *x9.add(32), *x9.add(63), *x10.add(32),
                             *x10.add(63), __rounding, cos_bit);
    btf_32_type1_sse4_1_new!(cospi_p31, cospi_p33, *x9.add(33), *x9.add(62), *x10.add(33),
                             *x10.add(62), __rounding, cos_bit);
    btf_32_type1_sse4_1_new!(cospi_p47, cospi_p17, *x9.add(34), *x9.add(61), *x10.add(34),
                             *x10.add(61), __rounding, cos_bit);
    btf_32_type1_sse4_1_new!(cospi_p15, cospi_p49, *x9.add(35), *x9.add(60), *x10.add(35),
                             *x10.add(60), __rounding, cos_bit);
    btf_32_type1_sse4_1_new!(cospi_p55, cospi_p09, *x9.add(36), *x9.add(59), *x10.add(36),
                             *x10.add(59), __rounding, cos_bit);
    btf_32_type1_sse4_1_new!(cospi_p23, cospi_p41, *x9.add(37), *x9.add(58), *x10.add(37),
                             *x10.add(58), __rounding, cos_bit);
    btf_32_type1_sse4_1_new!(cospi_p39, cospi_p25, *x9.add(38), *x9.add(57), *x10.add(38),
                             *x10.add(57), __rounding, cos_bit);
    btf_32_type1_sse4_1_new!(cospi_p07, cospi_p57, *x9.add(39), *x9.add(56), *x10.add(39),
                             *x10.add(56), __rounding, cos_bit);
    btf_32_type1_sse4_1_new!(cospi_p59, cospi_p05, *x9.add(40), *x9.add(55), *x10.add(40),
                             *x10.add(55), __rounding, cos_bit);
    btf_32_type1_sse4_1_new!(cospi_p27, cospi_p37, *x9.add(41), *x9.add(54), *x10.add(41),
                             *x10.add(54), __rounding, cos_bit);
    btf_32_type1_sse4_1_new!(cospi_p43, cospi_p21, *x9.add(42), *x9.add(53), *x10.add(42),
                             *x10.add(53), __rounding, cos_bit);
    btf_32_type1_sse4_1_new!(cospi_p11, cospi_p53, *x9.add(43), *x9.add(52), *x10.add(43),
                             *x10.add(52), __rounding, cos_bit);
    btf_32_type1_sse4_1_new!(cospi_p51, cospi_p13, *x9.add(44), *x9.add(51), *x10.add(44),
                             *x10.add(51), __rounding, cos_bit);
    btf_32_type1_sse4_1_new!(cospi_p19, cospi_p45, *x9.add(45), *x9.add(50), *x10.add(45),
                             *x10.add(50), __rounding, cos_bit);
    btf_32_type1_sse4_1_new!(cospi_p35, cospi_p29, *x9.add(46), *x9.add(49), *x10.add(46),
                             *x10.add(49), __rounding, cos_bit);
    btf_32_type1_sse4_1_new!(cospi_p03, cospi_p61, *x9.add(47), *x9.add(48), *x10.add(47),
                             *x10.add(48), __rounding, cos_bit);

    startidx = 0 * outstride;
    endidx = 63 * outstride;
    // stage 11
    *output.offset(startidx as isize) = *x10.add(0);
    *output.offset(endidx as isize) = *x10.add(63);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *x10.add(32);
    *output.offset(endidx as isize) = *x10.add(31);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *x10.add(16);
    *output.offset(endidx as isize) = *x10.add(47);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *x10.add(48);
    *output.offset(endidx as isize) = *x10.add(15);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *x10.add(8);
    *output.offset(endidx as isize) = *x10.add(55);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *x10.add(40);
    *output.offset(endidx as isize) = *x10.add(23);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *x10.add(24);
    *output.offset(endidx as isize) = *x10.add(39);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *x10.add(56);
    *output.offset(endidx as isize) = *x10.add(7);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *x10.add(4);
    *output.offset(endidx as isize) = *x10.add(59);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *x10.add(36);
    *output.offset(endidx as isize) = *x10.add(27);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *x10.add(20);
    *output.offset(endidx as isize) = *x10.add(43);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *x10.add(52);
    *output.offset(endidx as isize) = *x10.add(11);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *x10.add(12);
    *output.offset(endidx as isize) = *x10.add(51);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *x10.add(44);
    *output.offset(endidx as isize) = *x10.add(19);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *x10.add(28);
    *output.offset(endidx as isize) = *x10.add(35);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *x10.add(60);
    *output.offset(endidx as isize) = *x10.add(3);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *x10.add(2);
    *output.offset(endidx as isize) = *x10.add(61);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *x10.add(34);
    *output.offset(endidx as isize) = *x10.add(29);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *x10.add(18);
    *output.offset(endidx as isize) = *x10.add(45);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *x10.add(50);
    *output.offset(endidx as isize) = *x10.add(13);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *x10.add(10);
    *output.offset(endidx as isize) = *x10.add(53);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *x10.add(42);
    *output.offset(endidx as isize) = *x10.add(21);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *x10.add(26);
    *output.offset(endidx as isize) = *x10.add(37);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *x10.add(58);
    *output.offset(endidx as isize) = *x10.add(5);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *x10.add(6);
    *output.offset(endidx as isize) = *x10.add(57);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *x10.add(38);
    *output.offset(endidx as isize) = *x10.add(25);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *x10.add(22);
    *output.offset(endidx as isize) = *x10.add(41);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *x10.add(54);
    *output.offset(endidx as isize) = *x10.add(9);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *x10.add(14);
    *output.offset(endidx as isize) = *x10.add(49);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *x10.add(46);
    *output.offset(endidx as isize) = *x10.add(17);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *x10.add(30);
    *output.offset(endidx as isize) = *x10.add(33);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *x10.add(62);
    *output.offset(endidx as isize) = *x10.add(1);
}

#[target_feature(enable = "sse4.1")]
pub unsafe fn av1_idtx32_sse4_1(input: *mut __m128i, output: *mut __m128i, cos_bit: i32,
                                col_num: i32) {
    let _ = cos_bit;
    let mut i: i32 = 0;
    while i < 32 {
        *output.offset((i * col_num) as isize) =
            _mm_slli_epi32::<2>(*input.offset((i * col_num) as isize));
        i += 1;
    }
}
