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

// Translated by hand from upstream/aom/aom_dsp/x86/txfm_common_avx2.h (ADR
// 0056, ADR 0058). Names, order, comments, and intrinsic sequences follow the
// original. Every helper carries the smallest target feature its own
// intrinsics need: `avx` for helpers that only use AVX loads, stores, sets,
// and 128-bit lane inserts (flip_buf_avx2, which uses no intrinsic, also gets
// `avx` because it moves 256-bit vectors), `avx2` for everything else.
//
// Translation notes:
// - Pointer and array parameters stay raw pointers (`T *` -> `*mut T`,
//   `const T *` -> `*const T`), so in-place calls keep the C aliasing. Local
//   arrays are zero-initialized; the C code never reads an element before
//   writing it.
// - The run-time shift counts of btf_16_w16_avx2, round_shift_16bit_w16_avx2,
//   round_shift_32_avx2, round_shift_array_32_avx2, and
//   round_shift_rect_array_32_avx2 (`_mm256_srai_epi16/32`,
//   `_mm256_slli_epi16/32` with an `int` count) are translated with the
//   count-register intrinsics (`_mm256_sra_epi32(v, _mm_cvtsi32_si128(n))`,
//   ...). GCC and Clang emit exactly these instructions for a non-constant
//   count, the count is zero-extended by MOVD in both, and counts above the
//   lane width saturate identically, so the result is bit-identical for every
//   `int` count. The callers pass table-driven `cos_bit` and shift values, so a
//   const-generic signature is not possible. Constant counts keep the
//   immediate intrinsic.
// - load_32bit_to_16bit_w16_avx2 dereferences `(const __m256i *)(a + 8)` in C,
//   which VEX encodings fold into an unaligned memory operand; it is read with
//   ptr::read_unaligned, which gives the same value for every address the C
//   accepts.
// - transpose_16bit_16x16_avx2's LOADL/LOADR macros become local macro_rules
//   with the same expansion.

use core::arch::x86_64::*;
use core::ptr;

// From av1/common/av1_txfm.h.
const NewSqrt2Bits: i32 = 12;
// 2^12 * sqrt(2)
const NewSqrt2: i32 = 5793;

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn pair_set_w16_epi16(a: i16, b: i16) -> __m256i {
    _mm256_set1_epi32((((a as u16) as u32) | (((b as u16) as u32) << 16)) as i32)
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn btf_16_w16_avx2(w0: __m256i, w1: __m256i, in0: *mut __m256i, in1: *mut __m256i,
                              _r: __m256i, cos_bit: i32) {
    let t0 = _mm256_unpacklo_epi16(*in0, *in1);
    let t1 = _mm256_unpackhi_epi16(*in0, *in1);
    let u0 = _mm256_madd_epi16(t0, w0);
    let u1 = _mm256_madd_epi16(t1, w0);
    let v0 = _mm256_madd_epi16(t0, w1);
    let v1 = _mm256_madd_epi16(t1, w1);

    let a0 = _mm256_add_epi32(u0, _r);
    let a1 = _mm256_add_epi32(u1, _r);
    let b0 = _mm256_add_epi32(v0, _r);
    let b1 = _mm256_add_epi32(v1, _r);

    // _mm256_srai_epi32(x, cos_bit) with a run-time count.
    let c0 = _mm256_sra_epi32(a0, _mm_cvtsi32_si128(cos_bit));
    let c1 = _mm256_sra_epi32(a1, _mm_cvtsi32_si128(cos_bit));
    let d0 = _mm256_sra_epi32(b0, _mm_cvtsi32_si128(cos_bit));
    let d1 = _mm256_sra_epi32(b1, _mm_cvtsi32_si128(cos_bit));

    *in0 = _mm256_packs_epi32(c0, c1);
    *in1 = _mm256_packs_epi32(d0, d1);
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn btf_16_adds_subs_avx2(in0: *mut __m256i, in1: *mut __m256i) {
    let _in0 = *in0;
    let _in1 = *in1;
    *in0 = _mm256_adds_epi16(_in0, _in1);
    *in1 = _mm256_subs_epi16(_in0, _in1);
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn btf_32_add_sub_avx2(in0: *mut __m256i, in1: *mut __m256i) {
    let _in0 = *in0;
    let _in1 = *in1;
    *in0 = _mm256_add_epi32(_in0, _in1);
    *in1 = _mm256_sub_epi32(_in0, _in1);
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn btf_16_adds_subs_out_avx2(out0: *mut __m256i, out1: *mut __m256i, in0: __m256i,
                                        in1: __m256i) {
    let _in0 = in0;
    let _in1 = in1;
    *out0 = _mm256_adds_epi16(_in0, _in1);
    *out1 = _mm256_subs_epi16(_in0, _in1);
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn btf_32_add_sub_out_avx2(out0: *mut __m256i, out1: *mut __m256i, in0: __m256i,
                                      in1: __m256i) {
    let _in0 = in0;
    let _in1 = in1;
    *out0 = _mm256_add_epi32(_in0, _in1);
    *out1 = _mm256_sub_epi32(_in0, _in1);
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn load_16bit_to_16bit_avx2(a: *const i16) -> __m256i {
    _mm256_load_si256(a as *const __m256i)
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn load_buffer_16bit_to_16bit_avx2(in_: *const i16, stride: i32, out: *mut __m256i,
                                              out_size: i32) {
    let mut i: i32 = 0;
    while i < out_size {
        *out.offset(i as isize) = load_16bit_to_16bit_avx2(in_.offset((i * stride) as isize));
        i += 1;
    }
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn load_buffer_16bit_to_16bit_flip_avx2(in_: *const i16, stride: i32,
                                                   out: *mut __m256i, out_size: i32) {
    let mut i: i32 = 0;
    while i < out_size {
        *out.offset((out_size - i - 1) as isize) =
            load_16bit_to_16bit_avx2(in_.offset((i * stride) as isize));
        i += 1;
    }
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn load_32bit_to_16bit_w16_avx2(a: *const i32) -> __m256i {
    let a_low = _mm256_lddqu_si256(a as *const __m256i);
    let b = _mm256_packs_epi32(a_low, ptr::read_unaligned(a.add(8) as *const __m256i));
    _mm256_permute4x64_epi64::<0xD8>(b)
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn load_buffer_32bit_to_16bit_w16_avx2(in_: *const i32, stride: i32,
                                                  out: *mut __m256i, out_size: i32) {
    let mut i: i32 = 0;
    while i < out_size {
        *out.offset(i as isize) = load_32bit_to_16bit_w16_avx2(in_.offset((i * stride) as isize));
        i += 1;
    }
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn transpose2_8x8_avx2(in_: *const __m256i, out: *mut __m256i) {
    let mut t = [_mm256_setzero_si256(); 16];
    let mut u = [_mm256_setzero_si256(); 16];
    // (1st, 2nd) ==> (lo, hi)
    //   (0, 1)   ==>  (0, 1)
    //   (2, 3)   ==>  (2, 3)
    //   (4, 5)   ==>  (4, 5)
    //   (6, 7)   ==>  (6, 7)
    for i in 0..4usize {
        t[2 * i] = _mm256_unpacklo_epi16(*in_.add(2 * i), *in_.add(2 * i + 1));
        t[2 * i + 1] = _mm256_unpackhi_epi16(*in_.add(2 * i), *in_.add(2 * i + 1));
    }

    // (1st, 2nd) ==> (lo, hi)
    //   (0, 2)   ==>  (0, 2)
    //   (1, 3)   ==>  (1, 3)
    //   (4, 6)   ==>  (4, 6)
    //   (5, 7)   ==>  (5, 7)
    for i in 0..2usize {
        u[i] = _mm256_unpacklo_epi32(t[i], t[i + 2]);
        u[i + 2] = _mm256_unpackhi_epi32(t[i], t[i + 2]);

        u[i + 4] = _mm256_unpacklo_epi32(t[i + 4], t[i + 6]);
        u[i + 6] = _mm256_unpackhi_epi32(t[i + 4], t[i + 6]);
    }

    // (1st, 2nd) ==> (lo, hi)
    //   (0, 4)   ==>  (0, 1)
    //   (1, 5)   ==>  (4, 5)
    //   (2, 6)   ==>  (2, 3)
    //   (3, 7)   ==>  (6, 7)
    for i in 0..2usize {
        *out.add(2 * i) = _mm256_unpacklo_epi64(u[2 * i], u[2 * i + 4]);
        *out.add(2 * i + 1) = _mm256_unpackhi_epi64(u[2 * i], u[2 * i + 4]);

        *out.add(2 * i + 4) = _mm256_unpacklo_epi64(u[2 * i + 1], u[2 * i + 5]);
        *out.add(2 * i + 5) = _mm256_unpackhi_epi64(u[2 * i + 1], u[2 * i + 5]);
    }
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn transpose_16bit_16x16_avx2(in_: *const __m256i, out: *mut __m256i) {
    let mut t = [_mm256_setzero_si256(); 16];

    macro_rules! LOADL {
        ($idx:expr) => {
            t[$idx] = _mm256_castsi128_si256(_mm_load_si128(in_.add($idx) as *const __m128i));
            t[$idx] = _mm256_inserti128_si256::<1>(
                t[$idx], _mm_load_si128(in_.add($idx + 8) as *const __m128i));
        };
    }

    macro_rules! LOADR {
        ($idx:expr) => {
            t[8 + $idx] =
                _mm256_castsi128_si256(_mm_load_si128((in_.add($idx) as *const __m128i).add(1)));
            t[8 + $idx] = _mm256_inserti128_si256::<1>(
                t[8 + $idx], _mm_load_si128((in_.add($idx + 8) as *const __m128i).add(1)));
        };
    }

    // load left 8x16
    LOADL!(0);
    LOADL!(1);
    LOADL!(2);
    LOADL!(3);
    LOADL!(4);
    LOADL!(5);
    LOADL!(6);
    LOADL!(7);

    // load right 8x16
    LOADR!(0);
    LOADR!(1);
    LOADR!(2);
    LOADR!(3);
    LOADR!(4);
    LOADR!(5);
    LOADR!(6);
    LOADR!(7);

    // get the top 16x8 result
    transpose2_8x8_avx2(t.as_ptr(), out);
    // get the bottom 16x8 result
    transpose2_8x8_avx2(t.as_ptr().add(8), out.add(8));
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn transpose_16bit_16x8_avx2(in_: *const __m256i, out: *mut __m256i) {
    let a0 = _mm256_unpacklo_epi16(*in_.add(0), *in_.add(1));
    let a1 = _mm256_unpacklo_epi16(*in_.add(2), *in_.add(3));
    let a2 = _mm256_unpacklo_epi16(*in_.add(4), *in_.add(5));
    let a3 = _mm256_unpacklo_epi16(*in_.add(6), *in_.add(7));
    let a4 = _mm256_unpackhi_epi16(*in_.add(0), *in_.add(1));
    let a5 = _mm256_unpackhi_epi16(*in_.add(2), *in_.add(3));
    let a6 = _mm256_unpackhi_epi16(*in_.add(4), *in_.add(5));
    let a7 = _mm256_unpackhi_epi16(*in_.add(6), *in_.add(7));

    let b0 = _mm256_unpacklo_epi32(a0, a1);
    let b1 = _mm256_unpacklo_epi32(a2, a3);
    let b2 = _mm256_unpacklo_epi32(a4, a5);
    let b3 = _mm256_unpacklo_epi32(a6, a7);
    let b4 = _mm256_unpackhi_epi32(a0, a1);
    let b5 = _mm256_unpackhi_epi32(a2, a3);
    let b6 = _mm256_unpackhi_epi32(a4, a5);
    let b7 = _mm256_unpackhi_epi32(a6, a7);

    *out.add(0) = _mm256_unpacklo_epi64(b0, b1);
    *out.add(1) = _mm256_unpackhi_epi64(b0, b1);
    *out.add(2) = _mm256_unpacklo_epi64(b4, b5);
    *out.add(3) = _mm256_unpackhi_epi64(b4, b5);
    *out.add(4) = _mm256_unpacklo_epi64(b2, b3);
    *out.add(5) = _mm256_unpackhi_epi64(b2, b3);
    *out.add(6) = _mm256_unpacklo_epi64(b6, b7);
    *out.add(7) = _mm256_unpackhi_epi64(b6, b7);
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn flip_buf_avx2(in_: *mut __m256i, out: *mut __m256i, size: i32) {
    let mut i: i32 = 0;
    while i < size {
        *out.offset((size - i - 1) as isize) = *in_.offset(i as isize);
        i += 1;
    }
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn round_shift_16bit_w16_avx2(in_: *mut __m256i, size: i32, mut bit: i32) {
    if bit < 0 {
        bit = -bit;
        let round = _mm256_set1_epi16((1 << (bit - 1)) as i16);
        let mut i: i32 = 0;
        while i < size {
            *in_.offset(i as isize) = _mm256_adds_epi16(*in_.offset(i as isize), round);
            // _mm256_srai_epi16(in[i], bit) with a run-time count.
            *in_.offset(i as isize) =
                _mm256_sra_epi16(*in_.offset(i as isize), _mm_cvtsi32_si128(bit));
            i += 1;
        }
    } else if bit > 0 {
        let mut i: i32 = 0;
        while i < size {
            // _mm256_slli_epi16(in[i], bit) with a run-time count.
            *in_.offset(i as isize) =
                _mm256_sll_epi16(*in_.offset(i as isize), _mm_cvtsi32_si128(bit));
            i += 1;
        }
    }
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn round_shift_32_avx2(vec: __m256i, bit: i32) -> __m256i {
    let tmp: __m256i;
    let round: __m256i;
    round = _mm256_set1_epi32(1 << (bit - 1));
    tmp = _mm256_add_epi32(vec, round);
    // _mm256_srai_epi32(tmp, bit) with a run-time count.
    _mm256_sra_epi32(tmp, _mm_cvtsi32_si128(bit))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn round_shift_array_32_avx2(input: *mut __m256i, output: *mut __m256i, size: i32,
                                        bit: i32) {
    if bit > 0 {
        let mut i: i32;
        i = 0;
        while i < size {
            *output.offset(i as isize) = round_shift_32_avx2(*input.offset(i as isize), bit);
            i += 1;
        }
    } else {
        let mut i: i32;
        i = 0;
        while i < size {
            // _mm256_slli_epi32(input[i], -bit) with a run-time count.
            *output.offset(i as isize) =
                _mm256_sll_epi32(*input.offset(i as isize), _mm_cvtsi32_si128(-bit));
            i += 1;
        }
    }
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn round_shift_rect_array_32_avx2(input: *mut __m256i, output: *mut __m256i,
                                             size: i32, bit: i32, val: i32) {
    let sqrt2 = _mm256_set1_epi32(val);
    if bit > 0 {
        let mut i: i32;
        i = 0;
        while i < size {
            let r0 = round_shift_32_avx2(*input.offset(i as isize), bit);
            let r1 = _mm256_mullo_epi32(sqrt2, r0);
            *output.offset(i as isize) = round_shift_32_avx2(r1, NewSqrt2Bits);
            i += 1;
        }
    } else {
        let mut i: i32;
        i = 0;
        while i < size {
            // _mm256_slli_epi32(input[i], -bit) with a run-time count.
            let r0 = _mm256_sll_epi32(*input.offset(i as isize), _mm_cvtsi32_si128(-bit));
            let r1 = _mm256_mullo_epi32(sqrt2, r0);
            *output.offset(i as isize) = round_shift_32_avx2(r1, NewSqrt2Bits);
            i += 1;
        }
    }
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn scale_round_avx2(a: __m256i, scale: i32) -> __m256i {
    let scale_rounding = pair_set_w16_epi16(scale as i16, (1 << (NewSqrt2Bits - 1)) as i16);
    let b = _mm256_madd_epi16(a, scale_rounding);
    _mm256_srai_epi32::<NewSqrt2Bits>(b)
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn store_rect_16bit_to_32bit_w8_avx2(a: __m256i, b: *mut i32) {
    let one = _mm256_set1_epi16(1);
    let a_lo = _mm256_unpacklo_epi16(a, one);
    let a_hi = _mm256_unpackhi_epi16(a, one);
    let b_lo = scale_round_avx2(a_lo, NewSqrt2);
    let b_hi = scale_round_avx2(a_hi, NewSqrt2);
    let temp = _mm256_permute2f128_si256::<0x31>(b_lo, b_hi);
    _mm_store_si128(b as *mut __m128i, _mm256_castsi256_si128(b_lo));
    _mm_store_si128(b.add(4) as *mut __m128i, _mm256_castsi256_si128(b_hi));
    _mm256_store_si256(b.add(64) as *mut __m256i, temp);
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn store_rect_buffer_16bit_to_32bit_w8_avx2(in_: *const __m256i, out: *mut i32,
                                                       stride: i32, out_size: i32) {
    let mut i: i32 = 0;
    while i < out_size {
        store_rect_16bit_to_32bit_w8_avx2(*in_.offset(i as isize),
                                          out.offset((i * stride) as isize));
        i += 1;
    }
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn pack_reg(in1: *const __m128i, in2: *const __m128i, out: *mut __m256i) {
    *out.add(0) = _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(*in1.add(0)), *in2.add(0));
    *out.add(1) = _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(*in1.add(1)), *in2.add(1));
    *out.add(2) = _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(*in1.add(2)), *in2.add(2));
    *out.add(3) = _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(*in1.add(3)), *in2.add(3));
    *out.add(4) = _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(*in1.add(4)), *in2.add(4));
    *out.add(5) = _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(*in1.add(5)), *in2.add(5));
    *out.add(6) = _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(*in1.add(6)), *in2.add(6));
    *out.add(7) = _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(*in1.add(7)), *in2.add(7));
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn extract_reg(in_: *const __m256i, out1: *mut __m128i) {
    *out1.add(0) = _mm256_castsi256_si128(*in_.add(0));
    *out1.add(1) = _mm256_castsi256_si128(*in_.add(1));
    *out1.add(2) = _mm256_castsi256_si128(*in_.add(2));
    *out1.add(3) = _mm256_castsi256_si128(*in_.add(3));
    *out1.add(4) = _mm256_castsi256_si128(*in_.add(4));
    *out1.add(5) = _mm256_castsi256_si128(*in_.add(5));
    *out1.add(6) = _mm256_castsi256_si128(*in_.add(6));
    *out1.add(7) = _mm256_castsi256_si128(*in_.add(7));

    *out1.add(8) = _mm256_extracti128_si256::<0x01>(*in_.add(0));
    *out1.add(9) = _mm256_extracti128_si256::<0x01>(*in_.add(1));
    *out1.add(10) = _mm256_extracti128_si256::<0x01>(*in_.add(2));
    *out1.add(11) = _mm256_extracti128_si256::<0x01>(*in_.add(3));
    *out1.add(12) = _mm256_extracti128_si256::<0x01>(*in_.add(4));
    *out1.add(13) = _mm256_extracti128_si256::<0x01>(*in_.add(5));
    *out1.add(14) = _mm256_extracti128_si256::<0x01>(*in_.add(6));
    *out1.add(15) = _mm256_extracti128_si256::<0x01>(*in_.add(7));
}
