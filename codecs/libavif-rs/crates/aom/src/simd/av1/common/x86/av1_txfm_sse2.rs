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
#![allow(
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut,
    unused_parens
)]
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

use crate::simd::aom_dsp__x86__txfm_common_sse2::pair_set_epi16;

// From av1/common/av1_txfm.h.
const NewSqrt2Bits: i32 = 12;
// 2^12 * sqrt(2)
const NewSqrt2: i32 = 5793;

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn btf_16_w4_sse2(
    w0: *const __m128i,
    w1: *const __m128i,
    __rounding: __m128i,
    cos_bit: i8,
    in0: *const __m128i,
    in1: *const __m128i,
    out0: *mut __m128i,
    out1: *mut __m128i,
) {
    let t0: __m128i = _mm_unpacklo_epi16(*in0, *in1);
    let u0: __m128i = _mm_madd_epi16(t0, *w0);
    let v0: __m128i = _mm_madd_epi16(t0, *w1);
    let a0: __m128i = _mm_add_epi32(u0, __rounding);
    let b0: __m128i = _mm_add_epi32(v0, __rounding);
    let c0: __m128i = _mm_sra_epi32(a0, _mm_cvtsi32_si128(cos_bit as i32));
    let d0: __m128i = _mm_sra_epi32(b0, _mm_cvtsi32_si128(cos_bit as i32));
    *out0 = _mm_packs_epi32(c0, c0);
    *out1 = _mm_packs_epi32(d0, c0);
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn load_16bit_to_16bit(a: *const i16) -> __m128i {
    return _mm_load_si128(a as *const __m128i);
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn load_32bit_to_16bit(a: *const i32) -> __m128i {
    let a_low: __m128i = _mm_load_si128(a as *const __m128i);
    return _mm_packs_epi32(a_low, *(a.offset(4) as *const __m128i));
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn load_32bit_to_16bit_w4(a: *const i32) -> __m128i {
    let a_low: __m128i = _mm_load_si128(a as *const __m128i);
    return _mm_packs_epi32(a_low, a_low);
}

// Store 4 16 bit values. Sign extend the values.
#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn store_16bit_to_32bit_w4(a: __m128i, b: *mut i32) {
    let a_lo: __m128i = _mm_unpacklo_epi16(a, a);
    let a_1: __m128i = _mm_srai_epi32::<{ 16 }>(a_lo);
    _mm_store_si128(b as *mut __m128i, a_1);
}

// Store 8 16 bit values. Sign extend the values.
#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn store_16bit_to_32bit(a: __m128i, b: *mut i32) {
    let a_lo: __m128i = _mm_unpacklo_epi16(a, a);
    let a_hi: __m128i = _mm_unpackhi_epi16(a, a);
    let a_1: __m128i = _mm_srai_epi32::<{ 16 }>(a_lo);
    let a_2: __m128i = _mm_srai_epi32::<{ 16 }>(a_hi);
    _mm_store_si128(b as *mut __m128i, a_1);
    _mm_store_si128(b.offset(4) as *mut __m128i, a_2);
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn scale_round_sse2(a: __m128i, scale: i32) -> __m128i {
    let scale_rounding: __m128i = pair_set_epi16(scale, 1 << (NewSqrt2Bits - 1));
    let b: __m128i = _mm_madd_epi16(a, scale_rounding);
    return _mm_srai_epi32::<{ NewSqrt2Bits }>(b);
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn store_rect_16bit_to_32bit_w4(a: __m128i, b: *mut i32) {
    let one: __m128i = _mm_set1_epi16(1);
    let a_lo: __m128i = _mm_unpacklo_epi16(a, one);
    let b_lo: __m128i = scale_round_sse2(a_lo, NewSqrt2);
    _mm_store_si128(b as *mut __m128i, b_lo);
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn store_rect_16bit_to_32bit(a: __m128i, b: *mut i32) {
    let one: __m128i = _mm_set1_epi16(1);
    let a_lo: __m128i = _mm_unpacklo_epi16(a, one);
    let a_hi: __m128i = _mm_unpackhi_epi16(a, one);
    let b_lo: __m128i = scale_round_sse2(a_lo, NewSqrt2);
    let b_hi: __m128i = scale_round_sse2(a_hi, NewSqrt2);
    _mm_store_si128(b as *mut __m128i, b_lo);
    _mm_store_si128(b.offset(4) as *mut __m128i, b_hi);
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn load_buffer_16bit_to_16bit_w4(
    in_: *const i16,
    stride: i32,
    out: *mut __m128i,
    out_size: i32,
) {
    for i in 0..out_size {
        *out.offset(i as isize) =
            _mm_loadl_epi64(in_.offset((i * stride) as isize) as *const __m128i);
    }
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn load_buffer_16bit_to_16bit_w4_flip(
    in_: *const i16,
    stride: i32,
    out: *mut __m128i,
    out_size: i32,
) {
    for i in 0..out_size {
        *out.offset((out_size - i - 1) as isize) =
            _mm_loadl_epi64(in_.offset((i * stride) as isize) as *const __m128i);
    }
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn load_buffer_16bit_to_16bit(
    in_: *const i16,
    stride: i32,
    out: *mut __m128i,
    out_size: i32,
) {
    for i in 0..out_size {
        *out.offset(i as isize) = load_16bit_to_16bit(in_.offset((i * stride) as isize));
    }
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn load_buffer_16bit_to_16bit_flip(
    in_: *const i16,
    stride: i32,
    out: *mut __m128i,
    out_size: i32,
) {
    for i in 0..out_size {
        *out.offset((out_size - i - 1) as isize) =
            load_16bit_to_16bit(in_.offset((i * stride) as isize));
    }
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn load_buffer_32bit_to_16bit(
    in_: *const i32,
    stride: i32,
    out: *mut __m128i,
    out_size: i32,
) {
    for i in 0..out_size {
        *out.offset(i as isize) = load_32bit_to_16bit(in_.offset((i * stride) as isize));
    }
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn load_buffer_32bit_to_16bit_w4(
    in_: *const i32,
    stride: i32,
    out: *mut __m128i,
    out_size: i32,
) {
    for i in 0..out_size {
        *out.offset(i as isize) = load_32bit_to_16bit_w4(in_.offset((i * stride) as isize));
    }
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn load_buffer_32bit_to_16bit_flip(
    in_: *const i32,
    stride: i32,
    out: *mut __m128i,
    out_size: i32,
) {
    for i in 0..out_size {
        *out.offset((out_size - i - 1) as isize) =
            load_32bit_to_16bit(in_.offset((i * stride) as isize));
    }
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn store_buffer_16bit_to_32bit_w4(
    in_: *const __m128i,
    out: *mut i32,
    stride: i32,
    out_size: i32,
) {
    for i in 0..out_size {
        store_16bit_to_32bit_w4(*in_.offset(i as isize), out.offset((i * stride) as isize));
    }
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn store_buffer_16bit_to_32bit_w8(
    in_: *const __m128i,
    out: *mut i32,
    stride: i32,
    out_size: i32,
) {
    for i in 0..out_size {
        store_16bit_to_32bit(*in_.offset(i as isize), out.offset((i * stride) as isize));
    }
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn store_rect_buffer_16bit_to_32bit_w4(
    in_: *const __m128i,
    out: *mut i32,
    stride: i32,
    out_size: i32,
) {
    for i in 0..out_size {
        store_rect_16bit_to_32bit_w4(*in_.offset(i as isize), out.offset((i * stride) as isize));
    }
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn store_rect_buffer_16bit_to_32bit_w8(
    in_: *const __m128i,
    out: *mut i32,
    stride: i32,
    out_size: i32,
) {
    for i in 0..out_size {
        store_rect_16bit_to_32bit(*in_.offset(i as isize), out.offset((i * stride) as isize));
    }
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn store_buffer_16bit_to_16bit_8x8(in_: *const __m128i, out: *mut u16, stride: i32) {
    for i in 0..8 {
        _mm_store_si128(
            out.offset((i * stride) as isize) as *mut __m128i,
            *in_.offset(i as isize),
        );
    }
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn round_shift_16bit(in_: *mut __m128i, size: i32, mut bit: i32) {
    if bit < 0 {
        bit = -bit;
        let mut rounding: __m128i = _mm_set1_epi16((1 << (bit - 1)) as i16);
        for i in 0..size {
            *in_.offset(i as isize) = _mm_adds_epi16(*in_.offset(i as isize), rounding);
            *in_.offset(i as isize) =
                _mm_sra_epi16(*in_.offset(i as isize), _mm_cvtsi32_si128(bit));
        }
    } else if bit > 0 {
        for i in 0..size {
            *in_.offset(i as isize) =
                _mm_sll_epi16(*in_.offset(i as isize), _mm_cvtsi32_si128(bit));
        }
    }
}

// AOM_AV1_COMMON_X86_AV1_TXFM_SSE2_H_
