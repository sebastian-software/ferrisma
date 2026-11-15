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

// Translated from upstream/aom/av1/common/x86/av1_txfm_sse2.h (ADR 0056,
// ADR 0058). Names, order, comments and intrinsic sequences follow the
// original. flip_buf_sse2 was translated by hand for the high-bitdepth
// inverse transforms; the other inline functions, which the 8-bit inverse
// transforms (av1_inv_txfm_ssse3.c, av1_inv_txfm_avx2.c) use, were
// transliterated with a typing-aware C-subset printer (not C2Rust) and
// reviewed against the C function by function, as ADR 0058 describes for the
// high-bitdepth transforms.
//
// Translation notes:
// - Every function carries `sse2`, the flag of the header's intrinsics.
// - Array parameters stay raw pointers (`out[i]` -> `*out.offset(i)`).
// - `*(const __m128i *)p` stays a dereference of an aligned `*const __m128i`,
//   as in the C (an SSE2 build requires the alignment too).
// - Shift counts that are run-time `int` values (`_mm_srai_epi32(x, cos_bit)`)
//   use the count-register intrinsics (`_mm_sra_epi32(x, _mm_cvtsi32_si128(n))`),
//   which is what GCC and Clang emit for a non-constant count.
// - The statement macros btf_16_4p_sse2 and btf_16_sse2 become macro_rules
//   with the same expansion. The C macros read `__rounding` and `cos_bit` from
//   the caller's scope; the Rust macros take them as two trailing arguments.
#![allow(non_snake_case, non_upper_case_globals, unused_assignments, unused_mut, unused_parens)]
#![allow(unused_variables, dead_code, clippy::all)]

use core::arch::x86_64::*;

macro_rules! btf_16_4p_sse2 {
    ($w0:expr, $w1:expr, $in0:expr, $in1:expr, $out0:expr, $out1:expr, $rounding:expr, $cos_bit:expr) => {{
        let t0: __m128i = _mm_unpacklo_epi16($in0, $in1);
        let u0: __m128i = _mm_madd_epi16(t0, $w0);
        let v0: __m128i = _mm_madd_epi16(t0, $w1);

        let a0: __m128i = _mm_add_epi32(u0, $rounding);
        let b0: __m128i = _mm_add_epi32(v0, $rounding);

        let c0: __m128i = _mm_sra_epi32(a0, _mm_cvtsi32_si128($cos_bit as i32));
        let d0: __m128i = _mm_sra_epi32(b0, _mm_cvtsi32_si128($cos_bit as i32));

        $out0 = _mm_packs_epi32(c0, c0);
        $out1 = _mm_packs_epi32(d0, d0);
    }};
}
pub(crate) use btf_16_4p_sse2;

macro_rules! btf_16_sse2 {
    ($w0:expr, $w1:expr, $in0:expr, $in1:expr, $out0:expr, $out1:expr, $rounding:expr, $cos_bit:expr) => {{
        let t0: __m128i = _mm_unpacklo_epi16($in0, $in1);
        let t1: __m128i = _mm_unpackhi_epi16($in0, $in1);
        let u0: __m128i = _mm_madd_epi16(t0, $w0);
        let u1: __m128i = _mm_madd_epi16(t1, $w0);
        let v0: __m128i = _mm_madd_epi16(t0, $w1);
        let v1: __m128i = _mm_madd_epi16(t1, $w1);

        let a0: __m128i = _mm_add_epi32(u0, $rounding);
        let a1: __m128i = _mm_add_epi32(u1, $rounding);
        let b0: __m128i = _mm_add_epi32(v0, $rounding);
        let b1: __m128i = _mm_add_epi32(v1, $rounding);

        let c0: __m128i = _mm_sra_epi32(a0, _mm_cvtsi32_si128($cos_bit as i32));
        let c1: __m128i = _mm_sra_epi32(a1, _mm_cvtsi32_si128($cos_bit as i32));
        let d0: __m128i = _mm_sra_epi32(b0, _mm_cvtsi32_si128($cos_bit as i32));
        let d1: __m128i = _mm_sra_epi32(b1, _mm_cvtsi32_si128($cos_bit as i32));

        $out0 = _mm_packs_epi32(c0, c1);
        $out1 = _mm_packs_epi32(d0, d1);
    }};
}
pub(crate) use btf_16_sse2;

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn flip_buf_sse2(in_: *mut __m128i, out: *mut __m128i, size: i32) {
    for i in 0..size {
        *out.offset((size - i - 1) as isize) = *in_.offset(i as isize);
    }
}

