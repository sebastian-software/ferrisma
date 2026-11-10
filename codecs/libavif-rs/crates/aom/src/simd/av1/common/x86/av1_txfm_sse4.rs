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

// Translated by hand from upstream/aom/av1/common/x86/av1_txfm_sse4.h and
// upstream/aom/av1/common/x86/av1_txfm_sse4.c (ADR 0056, ADR 0058). Names,
// order, and intrinsic sequences follow the original.
//
// Translation notes:
// - The header helpers carry the smallest target feature their own intrinsics
//   need: `sse2` for av1_round_shift_32_sse4_1 and
//   av1_round_shift_array_32_sse4_1, `sse4.1` for
//   av1_round_shift_rect_array_32_sse4_1 (`_mm_mullo_epi32`). The RTCD
//   function av1_round_shift_array_sse4_1 from av1_txfm_sse4.c carries
//   `sse4.1`, the flag upstream compiles that file with.
// - The run-time `bit` passed to `_mm_srai_epi32` and `_mm_slli_epi32` is
//   translated with the count-register intrinsics
//   (`_mm_sra_epi32(v, _mm_cvtsi32_si128(bit))`, `_mm_sll_epi32`). GCC and
//   Clang emit exactly these instructions for a non-constant count, the count
//   is zero-extended by MOVD in both, and counts above 31 saturate identically,
//   so the result is bit-identical for every `int` count. The callers pass
//   table-driven shift values, so a const-generic signature is not possible.
// - Array parameters stay raw pointers (`input[i]` -> `*input.offset(i)`), so
//   the in-place call in av1_round_shift_array_sse4_1 keeps the C aliasing.

use core::arch::x86_64::*;

// From av1/common/av1_txfm.h.
const NewSqrt2Bits: i32 = 12;

// ---- av1/common/x86/av1_txfm_sse4.h ----

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn av1_round_shift_32_sse4_1(vec: __m128i, bit: i32) -> __m128i {
    let tmp: __m128i;
    let round: __m128i;
    round = _mm_set1_epi32(1 << (bit - 1));
    tmp = _mm_add_epi32(vec, round);
    // _mm_srai_epi32(tmp, bit) with a run-time count.
    _mm_sra_epi32(tmp, _mm_cvtsi32_si128(bit))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn av1_round_shift_array_32_sse4_1(input: *const __m128i, output: *mut __m128i,
                                              size: i32, bit: i32) {
    if bit > 0 {
        let mut i: i32;
        i = 0;
        while i < size {
            *output.offset(i as isize) = av1_round_shift_32_sse4_1(*input.offset(i as isize), bit);
            i += 1;
        }
    } else {
        let mut i: i32;
        i = 0;
        while i < size {
            // _mm_slli_epi32(input[i], -bit) with a run-time count.
            *output.offset(i as isize) =
                _mm_sll_epi32(*input.offset(i as isize), _mm_cvtsi32_si128(-bit));
            i += 1;
        }
    }
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn av1_round_shift_rect_array_32_sse4_1(input: *const __m128i, output: *mut __m128i,
                                                   size: i32, bit: i32, val: i32) {
    let sqrt2 = _mm_set1_epi32(val);
    if bit > 0 {
        let mut i: i32;
        i = 0;
        while i < size {
            let r0 = av1_round_shift_32_sse4_1(*input.offset(i as isize), bit);
            let r1 = _mm_mullo_epi32(sqrt2, r0);
            *output.offset(i as isize) = av1_round_shift_32_sse4_1(r1, NewSqrt2Bits);
            i += 1;
        }
    } else {
        let mut i: i32;
        i = 0;
        while i < size {
            // _mm_slli_epi32(input[i], -bit) with a run-time count.
            let r0 = _mm_sll_epi32(*input.offset(i as isize), _mm_cvtsi32_si128(-bit));
            let r1 = _mm_mullo_epi32(sqrt2, r0);
            *output.offset(i as isize) = av1_round_shift_32_sse4_1(r1, NewSqrt2Bits);
            i += 1;
        }
    }
}

// ---- av1/common/x86/av1_txfm_sse4.c ----

// This function assumes `arr` is 16-byte aligned.
#[no_mangle]
#[target_feature(enable = "sse4.1")]
pub unsafe extern "C-unwind" fn av1_round_shift_array_sse4_1(arr: *mut i32, size: i32, bit: i32) {
    let vec: *mut __m128i = arr as *mut __m128i;
    let vec_size = size >> 2;
    av1_round_shift_array_32_sse4_1(vec, vec, vec_size, bit);
}
