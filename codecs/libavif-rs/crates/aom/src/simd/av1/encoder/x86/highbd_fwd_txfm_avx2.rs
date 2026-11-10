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

// Translated from upstream/aom/av1/encoder/x86/highbd_fwd_txfm_avx2.c
// (ADR 0056, ADR 0058). Upstream compiles this file with -mavx2. Function
// names, helper structure, comments, constants, and intrinsic sequences follow
// the original; every function carries `#[target_feature(enable = "avx2")]`.
// The straight-line function bodies were transliterated with statement-
// rewriting scripts (not C2Rust) and then compared with the C function by
// function: identifiers, constants and operators in order (ADR 0058).
//
// Translation notes:
// - C arrays become zero-initialized stack arrays that are only accessed
//   through a raw pointer to their first element (`a[i]` -> `*a.add(i)` or
//   `*a.offset(i as isize)`, `&a[i]` -> `a.add(i)`), so the in-place calls
//   (`col_txfm(&buf0[i], &buf0[i], ...)`) keep the C aliasing. The C code never
//   reads an element before writing it.
// - Run-time shift counts (`_mm256_srai_epi32(x, bit)`,
//   `_mm256_slli_epi32(x, shift)` with an `int` or `int8_t` count) are
//   translated with the count-register intrinsics
//   (`_mm256_sra_epi32(x, _mm_cvtsi32_si128(bit as i32))`,
//   `_mm256_sll_epi32`). GCC and Clang emit exactly these instructions for a
//   non-constant count, the count is the C integer promotion of the argument
//   zero-extended by MOVD, and counts above 31 saturate identically, so the
//   result is bit-identical for every count. Constant counts keep the
//   immediate intrinsic with a const generic, as do the immediates of
//   `_mm256_permute2f128_si256` and `_mm256_extractf128_si256`.
// - The int8_t entries of av1_fwd_txfm_shift_ls are promoted to `int` before
//   any arithmetic (`-shift[1]`), as in C (`*shift.add(1) as i32`).
// - btf_32_avx2_type0 and btf_32_type0_avx2_new stay macros (macro_rules)
//   because out0 and out1 are assignment targets; the expansion evaluates every
//   argument where the C macro names it.
// - The av1/common/av1_txfm.h, av1/common/enums.h, and
//   av1/common/common_data.h items this file uses are defined locally, as in
//   the Neon translations. The coefficient and shift tables come from the
//   translated C (av1_cospi_arr_data, av1_fwd_txfm_shift_ls,
//   av1_fwd_cos_bit_col, av1_fwd_cos_bit_row).
// - transform_1d_avx2 becomes an `Option<unsafe fn ..>` alias so the 32x32
//   tables can hold NULL entries. The C code calls the selected entry
//   unconditionally; a NULL entry (a type the size does not support) would
//   crash there and panics here.
#![allow(clippy::all)]

use core::arch::x86_64::*;

use crate::av1__common__av1_txfm::av1_cospi_arr_data;
use crate::av1__encoder__av1_fwd_txfm2d::{av1_fwd_cos_bit_col, av1_fwd_cos_bit_row,
                                         av1_fwd_txfm_shift_ls};
use crate::simd::aom_dsp__x86__txfm_common_avx2::round_shift_rect_array_32_avx2;
use crate::simd::aom_dsp__x86__txfm_common_sse2::mm_reverse_epi16;

// From aom_dsp/txfm_common.h and av1/common/enums.h.
pub type TX_TYPE = u8;
const DCT_DCT: TX_TYPE = 0;
const ADST_DCT: TX_TYPE = 1;
const DCT_ADST: TX_TYPE = 2;
const ADST_ADST: TX_TYPE = 3;
const FLIPADST_DCT: TX_TYPE = 4;
const DCT_FLIPADST: TX_TYPE = 5;
const FLIPADST_FLIPADST: TX_TYPE = 6;
const ADST_FLIPADST: TX_TYPE = 7;
const FLIPADST_ADST: TX_TYPE = 8;
const IDTX: TX_TYPE = 9;
const V_DCT: TX_TYPE = 10;
const H_DCT: TX_TYPE = 11;
const V_ADST: TX_TYPE = 12;
const H_ADST: TX_TYPE = 13;
const V_FLIPADST: TX_TYPE = 14;
const H_FLIPADST: TX_TYPE = 15;
const TX_TYPES: usize = 16;

type TX_SIZE = u8;
const TX_8X8: TX_SIZE = 1;
const TX_16X16: TX_SIZE = 2;
const TX_32X32: TX_SIZE = 3;
const TX_64X64: TX_SIZE = 4;
const TX_8X16: TX_SIZE = 7;
const TX_16X8: TX_SIZE = 8;

// From av1/common/common_data.h.
const tx_size_wide: [i32; 19] =
    [4, 8, 16, 32, 64, 4, 8, 8, 16, 16, 32, 32, 64, 4, 16, 8, 32, 16, 64];
const tx_size_high: [i32; 19] =
    [4, 8, 16, 32, 64, 8, 4, 16, 8, 32, 16, 64, 32, 16, 4, 32, 8, 64, 16];
const tx_size_wide_log2: [i32; 19] = [2, 3, 4, 5, 6, 2, 3, 3, 4, 4, 5, 5, 6, 2, 4, 3, 5, 4, 6];
const tx_size_high_log2: [i32; 19] = [2, 3, 4, 5, 6, 3, 2, 4, 3, 5, 4, 6, 5, 4, 2, 5, 3, 6, 4];

// From av1/common/av1_txfm.h.
const cos_bit_min: i32 = 10;
const NewSqrt2Bits: i32 = 12;
// 2^12 * sqrt(2)
const NewSqrt2: i32 = 5793;

// From av1/common/av1_txfm.h.
#[inline]
unsafe fn cospi_arr(n: i32) -> *const i32 {
    (&raw const av1_cospi_arr_data[(n - cos_bit_min) as usize]).cast::<i32>()
}

// From av1/common/av1_txfm.h.
#[inline]
fn get_txw_idx(tx_size: TX_SIZE) -> i32 {
    tx_size_wide_log2[tx_size as usize] - tx_size_wide_log2[0]
}

// From av1/common/av1_txfm.h.
#[inline]
fn get_txh_idx(tx_size: TX_SIZE) -> i32 {
    tx_size_high_log2[tx_size as usize] - tx_size_high_log2[0]
}

// From av1/common/av1_txfm.h.
#[inline]
unsafe fn get_flip_cfg(tx_type: TX_TYPE, ud_flip: *mut i32, lr_flip: *mut i32) {
    match tx_type {
        DCT_DCT | ADST_DCT | DCT_ADST | ADST_ADST => {
            *ud_flip = 0;
            *lr_flip = 0;
        }
        IDTX | V_DCT | H_DCT | V_ADST | H_ADST => {
            *ud_flip = 0;
            *lr_flip = 0;
        }
        FLIPADST_DCT | FLIPADST_ADST | V_FLIPADST => {
            *ud_flip = 1;
            *lr_flip = 0;
        }
        DCT_FLIPADST | ADST_FLIPADST | H_FLIPADST => {
            *ud_flip = 0;
            *lr_flip = 1;
        }
        FLIPADST_FLIPADST => {
            *ud_flip = 1;
            *lr_flip = 1;
        }
        _ => {
            *ud_flip = 0;
            *lr_flip = 0;
            debug_assert!(false);
        }
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn load_buffer_8x8_avx2(input: *const i16, out: *mut __m256i, stride: i32, flipud: i32,
                               fliplr: i32, shift: i32) {
    let mut out1_: [__m128i; 8] = core::mem::zeroed();
    let out1 = out1_.as_mut_ptr();
    if flipud == 0 {
        *out1.add(0) = _mm_load_si128(input.offset((0 * stride) as isize) as *const __m128i);
        *out1.add(1) = _mm_load_si128(input.offset((1 * stride) as isize) as *const __m128i);
        *out1.add(2) = _mm_load_si128(input.offset((2 * stride) as isize) as *const __m128i);
        *out1.add(3) = _mm_load_si128(input.offset((3 * stride) as isize) as *const __m128i);
        *out1.add(4) = _mm_load_si128(input.offset((4 * stride) as isize) as *const __m128i);
        *out1.add(5) = _mm_load_si128(input.offset((5 * stride) as isize) as *const __m128i);
        *out1.add(6) = _mm_load_si128(input.offset((6 * stride) as isize) as *const __m128i);
        *out1.add(7) = _mm_load_si128(input.offset((7 * stride) as isize) as *const __m128i);
    } else {
        *out1.add(7) = _mm_load_si128(input.offset((0 * stride) as isize) as *const __m128i);
        *out1.add(6) = _mm_load_si128(input.offset((1 * stride) as isize) as *const __m128i);
        *out1.add(5) = _mm_load_si128(input.offset((2 * stride) as isize) as *const __m128i);
        *out1.add(4) = _mm_load_si128(input.offset((3 * stride) as isize) as *const __m128i);
        *out1.add(3) = _mm_load_si128(input.offset((4 * stride) as isize) as *const __m128i);
        *out1.add(2) = _mm_load_si128(input.offset((5 * stride) as isize) as *const __m128i);
        *out1.add(1) = _mm_load_si128(input.offset((6 * stride) as isize) as *const __m128i);
        *out1.add(0) = _mm_load_si128(input.offset((7 * stride) as isize) as *const __m128i);
    }
    if fliplr == 0 {
        *out.add(0) = _mm256_cvtepi16_epi32(*out1.add(0));
        *out.add(1) = _mm256_cvtepi16_epi32(*out1.add(1));
        *out.add(2) = _mm256_cvtepi16_epi32(*out1.add(2));
        *out.add(3) = _mm256_cvtepi16_epi32(*out1.add(3));
        *out.add(4) = _mm256_cvtepi16_epi32(*out1.add(4));
        *out.add(5) = _mm256_cvtepi16_epi32(*out1.add(5));
        *out.add(6) = _mm256_cvtepi16_epi32(*out1.add(6));
        *out.add(7) = _mm256_cvtepi16_epi32(*out1.add(7));
    } else {
        *out.add(0) = _mm256_cvtepi16_epi32(mm_reverse_epi16(*out1.add(0)));
        *out.add(1) = _mm256_cvtepi16_epi32(mm_reverse_epi16(*out1.add(1)));
        *out.add(2) = _mm256_cvtepi16_epi32(mm_reverse_epi16(*out1.add(2)));
        *out.add(3) = _mm256_cvtepi16_epi32(mm_reverse_epi16(*out1.add(3)));
        *out.add(4) = _mm256_cvtepi16_epi32(mm_reverse_epi16(*out1.add(4)));
        *out.add(5) = _mm256_cvtepi16_epi32(mm_reverse_epi16(*out1.add(5)));
        *out.add(6) = _mm256_cvtepi16_epi32(mm_reverse_epi16(*out1.add(6)));
        *out.add(7) = _mm256_cvtepi16_epi32(mm_reverse_epi16(*out1.add(7)));
    }
    *out.add(0) = _mm256_sll_epi32(*out.add(0), _mm_cvtsi32_si128(shift as i32));
    *out.add(1) = _mm256_sll_epi32(*out.add(1), _mm_cvtsi32_si128(shift as i32));
    *out.add(2) = _mm256_sll_epi32(*out.add(2), _mm_cvtsi32_si128(shift as i32));
    *out.add(3) = _mm256_sll_epi32(*out.add(3), _mm_cvtsi32_si128(shift as i32));
    *out.add(4) = _mm256_sll_epi32(*out.add(4), _mm_cvtsi32_si128(shift as i32));
    *out.add(5) = _mm256_sll_epi32(*out.add(5), _mm_cvtsi32_si128(shift as i32));
    *out.add(6) = _mm256_sll_epi32(*out.add(6), _mm_cvtsi32_si128(shift as i32));
    *out.add(7) = _mm256_sll_epi32(*out.add(7), _mm_cvtsi32_si128(shift as i32));
}
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn col_txfm_8x8_rounding(in_: *mut __m256i, shift: i32) {
    let rounding: __m256i = _mm256_set1_epi32(1 << (shift - 1));

    *in_.add(0) = _mm256_add_epi32(*in_.add(0), rounding);
    *in_.add(1) = _mm256_add_epi32(*in_.add(1), rounding);
    *in_.add(2) = _mm256_add_epi32(*in_.add(2), rounding);
    *in_.add(3) = _mm256_add_epi32(*in_.add(3), rounding);
    *in_.add(4) = _mm256_add_epi32(*in_.add(4), rounding);
    *in_.add(5) = _mm256_add_epi32(*in_.add(5), rounding);
    *in_.add(6) = _mm256_add_epi32(*in_.add(6), rounding);
    *in_.add(7) = _mm256_add_epi32(*in_.add(7), rounding);

    *in_.add(0) = _mm256_sra_epi32(*in_.add(0), _mm_cvtsi32_si128(shift as i32));
    *in_.add(1) = _mm256_sra_epi32(*in_.add(1), _mm_cvtsi32_si128(shift as i32));
    *in_.add(2) = _mm256_sra_epi32(*in_.add(2), _mm_cvtsi32_si128(shift as i32));
    *in_.add(3) = _mm256_sra_epi32(*in_.add(3), _mm_cvtsi32_si128(shift as i32));
    *in_.add(4) = _mm256_sra_epi32(*in_.add(4), _mm_cvtsi32_si128(shift as i32));
    *in_.add(5) = _mm256_sra_epi32(*in_.add(5), _mm_cvtsi32_si128(shift as i32));
    *in_.add(6) = _mm256_sra_epi32(*in_.add(6), _mm_cvtsi32_si128(shift as i32));
    *in_.add(7) = _mm256_sra_epi32(*in_.add(7), _mm_cvtsi32_si128(shift as i32));
}
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn load_buffer_8x16_avx2(input: *const i16, out: *mut __m256i, stride: i32, flipud: i32,
                                fliplr: i32, shift: i32) {
    let mut topL: *const i16 = input;
    let mut botL: *const i16 = input.offset((8 * stride) as isize);

    let tmp: *const i16;

    if flipud != 0 {
        tmp = topL;
        topL = botL;
        botL = tmp;
    }
    load_buffer_8x8_avx2(topL, out, stride, flipud, fliplr, shift);
    load_buffer_8x8_avx2(botL, out.add(8), stride, flipud, fliplr, shift);
}
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn load_buffer_16xn_avx2(input: *const i16, out: *mut __m256i, stride: i32, height: i32,
                                outstride: i32, flipud: i32, fliplr: i32) {
    let mut out1_: [__m256i; 64] = core::mem::zeroed();
    let out1 = out1_.as_mut_ptr();
    if flipud == 0 {
        let mut i: i32 = 0;
        while i < height {
            *out1.offset(i as isize) =
                _mm256_loadu_si256(input.offset((i * stride) as isize) as *const __m256i);
            i += 1;
        }
    } else {
        let mut i: i32 = 0;
        while i < height {
            *out1.offset(((height - 1) - i) as isize) =
                _mm256_loadu_si256(input.offset((i * stride) as isize) as *const __m256i);
            i += 1;
        }
    }
    if fliplr == 0 {
        let mut i: i32 = 0;
        while i < height {
            *out.offset((i * outstride) as isize) =
                _mm256_cvtepi16_epi32(_mm256_castsi256_si128(*out1.offset(i as isize)));
            *out.offset((i * outstride + 1) as isize) =
                _mm256_cvtepi16_epi32(_mm256_extractf128_si256::<1>(*out1.offset(i as isize)));
            i += 1;
        }
    } else {
        let mut i: i32 = 0;
        while i < height {
            *out.offset((i * outstride + 1) as isize) = _mm256_cvtepi16_epi32(
                mm_reverse_epi16(_mm256_castsi256_si128(*out1.offset(i as isize))));
            *out.offset((i * outstride + 0) as isize) = _mm256_cvtepi16_epi32(
                mm_reverse_epi16(_mm256_extractf128_si256::<1>(*out1.offset(i as isize))));
            i += 1;
        }
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn fwd_txfm_transpose_8x8_avx2(in_: *const __m256i, out: *mut __m256i, instride: i32,
                                      outstride: i32) {
    let u0: __m256i;
    let u1: __m256i;
    let u2: __m256i;
    let u3: __m256i;
    let u4: __m256i;
    let u5: __m256i;
    let u6: __m256i;
    let u7: __m256i;
    let mut x0: __m256i;
    let mut x1: __m256i;

    u0 = _mm256_unpacklo_epi32(*in_.offset((0 * instride) as isize),
                               *in_.offset((1 * instride) as isize));
    u1 = _mm256_unpackhi_epi32(*in_.offset((0 * instride) as isize),
                               *in_.offset((1 * instride) as isize));

    u2 = _mm256_unpacklo_epi32(*in_.offset((2 * instride) as isize),
                               *in_.offset((3 * instride) as isize));
    u3 = _mm256_unpackhi_epi32(*in_.offset((2 * instride) as isize),
                               *in_.offset((3 * instride) as isize));

    u4 = _mm256_unpacklo_epi32(*in_.offset((4 * instride) as isize),
                               *in_.offset((5 * instride) as isize));
    u5 = _mm256_unpackhi_epi32(*in_.offset((4 * instride) as isize),
                               *in_.offset((5 * instride) as isize));

    u6 = _mm256_unpacklo_epi32(*in_.offset((6 * instride) as isize),
                               *in_.offset((7 * instride) as isize));
    u7 = _mm256_unpackhi_epi32(*in_.offset((6 * instride) as isize),
                               *in_.offset((7 * instride) as isize));

    x0 = _mm256_unpacklo_epi64(u0, u2);
    x1 = _mm256_unpacklo_epi64(u4, u6);
    *out.offset((0 * outstride) as isize) = _mm256_permute2f128_si256::<0x20>(x0, x1);
    *out.offset((4 * outstride) as isize) = _mm256_permute2f128_si256::<0x31>(x0, x1);

    x0 = _mm256_unpackhi_epi64(u0, u2);
    x1 = _mm256_unpackhi_epi64(u4, u6);
    *out.offset((1 * outstride) as isize) = _mm256_permute2f128_si256::<0x20>(x0, x1);
    *out.offset((5 * outstride) as isize) = _mm256_permute2f128_si256::<0x31>(x0, x1);

    x0 = _mm256_unpacklo_epi64(u1, u3);
    x1 = _mm256_unpacklo_epi64(u5, u7);
    *out.offset((2 * outstride) as isize) = _mm256_permute2f128_si256::<0x20>(x0, x1);
    *out.offset((6 * outstride) as isize) = _mm256_permute2f128_si256::<0x31>(x0, x1);

    x0 = _mm256_unpackhi_epi64(u1, u3);
    x1 = _mm256_unpackhi_epi64(u5, u7);
    *out.offset((3 * outstride) as isize) = _mm256_permute2f128_si256::<0x20>(x0, x1);
    *out.offset((7 * outstride) as isize) = _mm256_permute2f128_si256::<0x31>(x0, x1);
}
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn round_shift_32_8xn_avx2(in_: *mut __m256i, size: i32, mut bit: i32, stride: i32) {
    if bit < 0 {
        bit = -bit;
        let round: __m256i = _mm256_set1_epi32(1 << (bit - 1));
        let mut i: i32 = 0;
        while i < size {
            *in_.offset((stride * i) as isize) =
                _mm256_add_epi32(*in_.offset((stride * i) as isize), round);
            // _mm256_srai_epi32(in[stride * i], bit) with a run-time count.
            *in_.offset((stride * i) as isize) =
                _mm256_sra_epi32(*in_.offset((stride * i) as isize), _mm_cvtsi32_si128(bit));
            i += 1;
        }
    } else if bit > 0 {
        let mut i: i32 = 0;
        while i < size {
            // _mm256_slli_epi32(in[stride * i], bit) with a run-time count.
            *in_.offset((stride * i) as isize) =
                _mm256_sll_epi32(*in_.offset((stride * i) as isize), _mm_cvtsi32_si128(bit));
            i += 1;
        }
    }
}
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn store_buffer_avx2(in_: *const __m256i, mut out: *mut i32, stride: i32, out_size: i32) {
    let mut i: i32 = 0;
    while i < out_size {
        _mm256_store_si256(out as *mut __m256i, *in_.offset(i as isize));
        out = out.offset(stride as isize);
        i += 1;
    }
}
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn fwd_txfm_transpose_16x16_avx2(in_: *const __m256i, out: *mut __m256i) {
    fwd_txfm_transpose_8x8_avx2(in_.add(0), out.add(0), 2, 2);
    fwd_txfm_transpose_8x8_avx2(in_.add(1), out.add(16), 2, 2);
    fwd_txfm_transpose_8x8_avx2(in_.add(16), out.add(1), 2, 2);
    fwd_txfm_transpose_8x8_avx2(in_.add(17), out.add(17), 2, 2);
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn av1_half_btf_avx2(w0: *const __m256i, n0: *const __m256i, w1: *const __m256i,
                            n1: *const __m256i, rounding: *const __m256i, bit: i32) -> __m256i {
    let mut x: __m256i;
    let y: __m256i;

    x = _mm256_mullo_epi32(*w0, *n0);
    y = _mm256_mullo_epi32(*w1, *n1);
    x = _mm256_add_epi32(x, y);
    x = _mm256_add_epi32(x, *rounding);
    // _mm256_srai_epi32(x, bit) with a run-time count.
    x = _mm256_sra_epi32(x, _mm_cvtsi32_si128(bit));
    x
}
macro_rules! btf_32_avx2_type0 {
    ($w0:expr, $w1:expr, $in0:expr, $in1:expr, $out0:expr, $out1:expr, $bit:expr) => {{
        let ww0: __m256i = _mm256_set1_epi32($w0);
        let ww1: __m256i = _mm256_set1_epi32($w1);
        let in0_w0: __m256i = _mm256_mullo_epi32($in0, ww0);
        let in1_w1: __m256i = _mm256_mullo_epi32($in1, ww1);
        $out0 = _mm256_add_epi32(in0_w0, in1_w1);
        round_shift_32_8xn_avx2(&raw mut $out0, 1, -(($bit) as i32), 1);
        let in0_w1: __m256i = _mm256_mullo_epi32($in0, ww1);
        let in1_w0: __m256i = _mm256_mullo_epi32($in1, ww0);
        $out1 = _mm256_sub_epi32(in0_w1, in1_w0);
        round_shift_32_8xn_avx2(&raw mut $out1, 1, -(($bit) as i32), 1);
    }};
}

macro_rules! btf_32_type0_avx2_new {
    ($ww0:expr, $ww1:expr, $in0:expr, $in1:expr, $out0:expr, $out1:expr, $r:expr, $bit:expr) => {{
        let in0_w0: __m256i = _mm256_mullo_epi32($in0, $ww0);
        let in1_w1: __m256i = _mm256_mullo_epi32($in1, $ww1);
        $out0 = _mm256_add_epi32(in0_w0, in1_w1);
        $out0 = _mm256_add_epi32($out0, $r);
        // _mm256_srai_epi32(out0, bit) with a run-time count.
        $out0 = _mm256_sra_epi32($out0, _mm_cvtsi32_si128(($bit) as i32));
        let in0_w1: __m256i = _mm256_mullo_epi32($in0, $ww1);
        let in1_w0: __m256i = _mm256_mullo_epi32($in1, $ww0);
        $out1 = _mm256_sub_epi32(in0_w1, in1_w0);
        $out1 = _mm256_add_epi32($out1, $r);
        // _mm256_srai_epi32(out1, bit) with a run-time count.
        $out1 = _mm256_sra_epi32($out1, _mm_cvtsi32_si128(($bit) as i32));
    }};
}

type transform_1d_avx2 = Option<unsafe fn(*mut __m256i, *mut __m256i, i8, i32, i32)>;
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn fdct8_avx2(in_: *mut __m256i, out: *mut __m256i, bit: i8, col_num: i32,
                     outstride: i32) {
    let cospi: *const i32 = cospi_arr(bit as i32);
    let cospi32: __m256i = _mm256_set1_epi32(*cospi.add(32));
    let cospim32: __m256i = _mm256_set1_epi32(-*cospi.add(32));
    let cospi48: __m256i = _mm256_set1_epi32(*cospi.add(48));
    let cospi16: __m256i = _mm256_set1_epi32(*cospi.add(16));
    let cospi56: __m256i = _mm256_set1_epi32(*cospi.add(56));
    let cospi8: __m256i = _mm256_set1_epi32(*cospi.add(8));
    let cospi24: __m256i = _mm256_set1_epi32(*cospi.add(24));
    let cospi40: __m256i = _mm256_set1_epi32(*cospi.add(40));
    let rnding: __m256i = _mm256_set1_epi32(1 << (bit - 1));
    let mut u_: [__m256i; 8] = core::mem::zeroed();
    let mut v_: [__m256i; 8] = core::mem::zeroed();
    let u = u_.as_mut_ptr();
    let v = v_.as_mut_ptr();
    let mut col: i32 = 0;
    while col < col_num {
        *u.add(0) = _mm256_add_epi32(*in_.offset((0 * col_num + col) as isize),
                                     *in_.offset((7 * col_num + col) as isize));
        *v.add(7) = _mm256_sub_epi32(*in_.offset((0 * col_num + col) as isize),
                                     *in_.offset((7 * col_num + col) as isize));
        *u.add(1) = _mm256_add_epi32(*in_.offset((1 * col_num + col) as isize),
                                     *in_.offset((6 * col_num + col) as isize));
        *u.add(6) = _mm256_sub_epi32(*in_.offset((1 * col_num + col) as isize),
                                     *in_.offset((6 * col_num + col) as isize));
        *u.add(2) = _mm256_add_epi32(*in_.offset((2 * col_num + col) as isize),
                                     *in_.offset((5 * col_num + col) as isize));
        *u.add(5) = _mm256_sub_epi32(*in_.offset((2 * col_num + col) as isize),
                                     *in_.offset((5 * col_num + col) as isize));
        *u.add(3) = _mm256_add_epi32(*in_.offset((3 * col_num + col) as isize),
                                     *in_.offset((4 * col_num + col) as isize));
        *v.add(4) = _mm256_sub_epi32(*in_.offset((3 * col_num + col) as isize),
                                     *in_.offset((4 * col_num + col) as isize));
        *v.add(0) = _mm256_add_epi32(*u.add(0), *u.add(3));
        *v.add(3) = _mm256_sub_epi32(*u.add(0), *u.add(3));
        *v.add(1) = _mm256_add_epi32(*u.add(1), *u.add(2));
        *v.add(2) = _mm256_sub_epi32(*u.add(1), *u.add(2));

        *v.add(5) = _mm256_mullo_epi32(*u.add(5), cospim32);
        *v.add(6) = _mm256_mullo_epi32(*u.add(6), cospi32);
        *v.add(5) = _mm256_add_epi32(*v.add(5), *v.add(6));
        *v.add(5) = _mm256_add_epi32(*v.add(5), rnding);
        *v.add(5) = _mm256_sra_epi32(*v.add(5), _mm_cvtsi32_si128(bit as i32));

        *u.add(0) = _mm256_mullo_epi32(*u.add(5), cospi32);
        *v.add(6) = _mm256_mullo_epi32(*u.add(6), cospim32);
        *v.add(6) = _mm256_sub_epi32(*u.add(0), *v.add(6));
        *v.add(6) = _mm256_add_epi32(*v.add(6), rnding);
        *v.add(6) = _mm256_sra_epi32(*v.add(6), _mm_cvtsi32_si128(bit as i32));

        // stage 3
        // type 0
        *v.add(0) = _mm256_mullo_epi32(*v.add(0), cospi32);
        *v.add(1) = _mm256_mullo_epi32(*v.add(1), cospi32);
        *u.add(0) = _mm256_add_epi32(*v.add(0), *v.add(1));
        *u.add(0) = _mm256_add_epi32(*u.add(0), rnding);
        *u.add(0) = _mm256_sra_epi32(*u.add(0), _mm_cvtsi32_si128(bit as i32));

        *u.add(1) = _mm256_sub_epi32(*v.add(0), *v.add(1));
        *u.add(1) = _mm256_add_epi32(*u.add(1), rnding);
        *u.add(1) = _mm256_sra_epi32(*u.add(1), _mm_cvtsi32_si128(bit as i32));

        // type 1
        *v.add(0) = _mm256_mullo_epi32(*v.add(2), cospi48);
        *v.add(1) = _mm256_mullo_epi32(*v.add(3), cospi16);
        *u.add(2) = _mm256_add_epi32(*v.add(0), *v.add(1));
        *u.add(2) = _mm256_add_epi32(*u.add(2), rnding);
        *u.add(2) = _mm256_sra_epi32(*u.add(2), _mm_cvtsi32_si128(bit as i32));

        *v.add(0) = _mm256_mullo_epi32(*v.add(2), cospi16);
        *v.add(1) = _mm256_mullo_epi32(*v.add(3), cospi48);
        *u.add(3) = _mm256_sub_epi32(*v.add(1), *v.add(0));
        *u.add(3) = _mm256_add_epi32(*u.add(3), rnding);
        *u.add(3) = _mm256_sra_epi32(*u.add(3), _mm_cvtsi32_si128(bit as i32));

        *u.add(4) = _mm256_add_epi32(*v.add(4), *v.add(5));
        *u.add(5) = _mm256_sub_epi32(*v.add(4), *v.add(5));
        *u.add(6) = _mm256_sub_epi32(*v.add(7), *v.add(6));
        *u.add(7) = _mm256_add_epi32(*v.add(7), *v.add(6));

        // stage 4
        // stage 5
        *v.add(0) = _mm256_mullo_epi32(*u.add(4), cospi56);
        *v.add(1) = _mm256_mullo_epi32(*u.add(7), cospi8);
        *v.add(0) = _mm256_add_epi32(*v.add(0), *v.add(1));
        *v.add(0) = _mm256_add_epi32(*v.add(0), rnding);
        *out.offset((1 * outstride + col) as isize) =
            _mm256_sra_epi32(*v.add(0), _mm_cvtsi32_si128(bit as i32));  // buf0[4]

        *v.add(0) = _mm256_mullo_epi32(*u.add(4), cospi8);
        *v.add(1) = _mm256_mullo_epi32(*u.add(7), cospi56);
        *v.add(0) = _mm256_sub_epi32(*v.add(1), *v.add(0));
        *v.add(0) = _mm256_add_epi32(*v.add(0), rnding);
        *out.offset((7 * outstride + col) as isize) =
            _mm256_sra_epi32(*v.add(0), _mm_cvtsi32_si128(bit as i32));  // buf0[7]

        *v.add(0) = _mm256_mullo_epi32(*u.add(5), cospi24);
        *v.add(1) = _mm256_mullo_epi32(*u.add(6), cospi40);
        *v.add(0) = _mm256_add_epi32(*v.add(0), *v.add(1));
        *v.add(0) = _mm256_add_epi32(*v.add(0), rnding);
        *out.offset((5 * outstride + col) as isize) =
            _mm256_sra_epi32(*v.add(0), _mm_cvtsi32_si128(bit as i32));  // buf0[5]

        *v.add(0) = _mm256_mullo_epi32(*u.add(5), cospi40);
        *v.add(1) = _mm256_mullo_epi32(*u.add(6), cospi24);
        *v.add(0) = _mm256_sub_epi32(*v.add(1), *v.add(0));
        *v.add(0) = _mm256_add_epi32(*v.add(0), rnding);
        *out.offset((3 * outstride + col) as isize) =
            _mm256_sra_epi32(*v.add(0), _mm_cvtsi32_si128(bit as i32));  // buf0[6]

        *out.offset((0 * outstride + col) as isize) = *u.add(0);  // buf0[0]
        *out.offset((4 * outstride + col) as isize) = *u.add(1);  // buf0[1]
        *out.offset((2 * outstride + col) as isize) = *u.add(2);  // buf0[2]
        *out.offset((6 * outstride + col) as isize) = *u.add(3);  // buf0[3]
        col += 1;
    }
}
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn fadst8_avx2(in_: *mut __m256i, out: *mut __m256i, bit: i8, col_num: i32,
                      outstirde: i32) {
    let _ = col_num;
    let cospi: *const i32 = cospi_arr(bit as i32);
    let cospi32: __m256i = _mm256_set1_epi32(*cospi.add(32));
    let cospi16: __m256i = _mm256_set1_epi32(*cospi.add(16));
    let cospim16: __m256i = _mm256_set1_epi32(-*cospi.add(16));
    let cospi48: __m256i = _mm256_set1_epi32(*cospi.add(48));
    let cospim48: __m256i = _mm256_set1_epi32(-*cospi.add(48));
    let cospi4: __m256i = _mm256_set1_epi32(*cospi.add(4));
    let cospim4: __m256i = _mm256_set1_epi32(-*cospi.add(4));
    let cospi60: __m256i = _mm256_set1_epi32(*cospi.add(60));
    let cospi20: __m256i = _mm256_set1_epi32(*cospi.add(20));
    let cospim20: __m256i = _mm256_set1_epi32(-*cospi.add(20));
    let cospi44: __m256i = _mm256_set1_epi32(*cospi.add(44));
    let cospi28: __m256i = _mm256_set1_epi32(*cospi.add(28));
    let cospi36: __m256i = _mm256_set1_epi32(*cospi.add(36));
    let cospim36: __m256i = _mm256_set1_epi32(-*cospi.add(36));
    let cospi52: __m256i = _mm256_set1_epi32(*cospi.add(52));
    let cospim52: __m256i = _mm256_set1_epi32(-*cospi.add(52));
    let cospi12: __m256i = _mm256_set1_epi32(*cospi.add(12));
    let rnding: __m256i = _mm256_set1_epi32(1 << (bit - 1));
    let zero: __m256i = _mm256_setzero_si256();
    let mut u0: __m256i;
    let mut u1: __m256i;
    let mut u2: __m256i;
    let mut u3: __m256i;
    let mut u4: __m256i;
    let mut u5: __m256i;
    let mut u6: __m256i;
    let mut u7: __m256i;
    let mut v0: __m256i;
    let mut v1: __m256i;
    let mut v2: __m256i;
    let mut v3: __m256i;
    let mut v4: __m256i;
    let mut v5: __m256i;
    let mut v6: __m256i;
    let mut v7: __m256i;
    let mut x: __m256i;
    let mut y: __m256i;
    let mut col: i32 = 0;
    while col < col_num {
        u0 = *in_.offset((0 * col_num + col) as isize);
        u1 = _mm256_sub_epi32(zero, *in_.offset((7 * col_num + col) as isize));
        u2 = _mm256_sub_epi32(zero, *in_.offset((3 * col_num + col) as isize));
        u3 = *in_.offset((4 * col_num + col) as isize);
        u4 = _mm256_sub_epi32(zero, *in_.offset((1 * col_num + col) as isize));
        u5 = *in_.offset((6 * col_num + col) as isize);
        u6 = *in_.offset((2 * col_num + col) as isize);
        u7 = _mm256_sub_epi32(zero, *in_.offset((5 * col_num + col) as isize));

        // stage 2
        v0 = u0;
        v1 = u1;

        x = _mm256_mullo_epi32(u2, cospi32);
        y = _mm256_mullo_epi32(u3, cospi32);
        v2 = _mm256_add_epi32(x, y);
        v2 = _mm256_add_epi32(v2, rnding);
        v2 = _mm256_sra_epi32(v2, _mm_cvtsi32_si128(bit as i32));

        v3 = _mm256_sub_epi32(x, y);
        v3 = _mm256_add_epi32(v3, rnding);
        v3 = _mm256_sra_epi32(v3, _mm_cvtsi32_si128(bit as i32));

        v4 = u4;
        v5 = u5;

        x = _mm256_mullo_epi32(u6, cospi32);
        y = _mm256_mullo_epi32(u7, cospi32);
        v6 = _mm256_add_epi32(x, y);
        v6 = _mm256_add_epi32(v6, rnding);
        v6 = _mm256_sra_epi32(v6, _mm_cvtsi32_si128(bit as i32));

        v7 = _mm256_sub_epi32(x, y);
        v7 = _mm256_add_epi32(v7, rnding);
        v7 = _mm256_sra_epi32(v7, _mm_cvtsi32_si128(bit as i32));

        // stage 3
        u0 = _mm256_add_epi32(v0, v2);
        u1 = _mm256_add_epi32(v1, v3);
        u2 = _mm256_sub_epi32(v0, v2);
        u3 = _mm256_sub_epi32(v1, v3);
        u4 = _mm256_add_epi32(v4, v6);
        u5 = _mm256_add_epi32(v5, v7);
        u6 = _mm256_sub_epi32(v4, v6);
        u7 = _mm256_sub_epi32(v5, v7);

        // stage 4
        v0 = u0;
        v1 = u1;
        v2 = u2;
        v3 = u3;

        x = _mm256_mullo_epi32(u4, cospi16);
        y = _mm256_mullo_epi32(u5, cospi48);
        v4 = _mm256_add_epi32(x, y);
        v4 = _mm256_add_epi32(v4, rnding);
        v4 = _mm256_sra_epi32(v4, _mm_cvtsi32_si128(bit as i32));

        x = _mm256_mullo_epi32(u4, cospi48);
        y = _mm256_mullo_epi32(u5, cospim16);
        v5 = _mm256_add_epi32(x, y);
        v5 = _mm256_add_epi32(v5, rnding);
        v5 = _mm256_sra_epi32(v5, _mm_cvtsi32_si128(bit as i32));

        x = _mm256_mullo_epi32(u6, cospim48);
        y = _mm256_mullo_epi32(u7, cospi16);
        v6 = _mm256_add_epi32(x, y);
        v6 = _mm256_add_epi32(v6, rnding);
        v6 = _mm256_sra_epi32(v6, _mm_cvtsi32_si128(bit as i32));

        x = _mm256_mullo_epi32(u6, cospi16);
        y = _mm256_mullo_epi32(u7, cospi48);
        v7 = _mm256_add_epi32(x, y);
        v7 = _mm256_add_epi32(v7, rnding);
        v7 = _mm256_sra_epi32(v7, _mm_cvtsi32_si128(bit as i32));

        // stage 5
        u0 = _mm256_add_epi32(v0, v4);
        u1 = _mm256_add_epi32(v1, v5);
        u2 = _mm256_add_epi32(v2, v6);
        u3 = _mm256_add_epi32(v3, v7);
        u4 = _mm256_sub_epi32(v0, v4);
        u5 = _mm256_sub_epi32(v1, v5);
        u6 = _mm256_sub_epi32(v2, v6);
        u7 = _mm256_sub_epi32(v3, v7);

        // stage 6
        x = _mm256_mullo_epi32(u0, cospi4);
        y = _mm256_mullo_epi32(u1, cospi60);
        v0 = _mm256_add_epi32(x, y);
        v0 = _mm256_add_epi32(v0, rnding);
        v0 = _mm256_sra_epi32(v0, _mm_cvtsi32_si128(bit as i32));

        x = _mm256_mullo_epi32(u0, cospi60);
        y = _mm256_mullo_epi32(u1, cospim4);
        v1 = _mm256_add_epi32(x, y);
        v1 = _mm256_add_epi32(v1, rnding);
        v1 = _mm256_sra_epi32(v1, _mm_cvtsi32_si128(bit as i32));

        x = _mm256_mullo_epi32(u2, cospi20);
        y = _mm256_mullo_epi32(u3, cospi44);
        v2 = _mm256_add_epi32(x, y);
        v2 = _mm256_add_epi32(v2, rnding);
        v2 = _mm256_sra_epi32(v2, _mm_cvtsi32_si128(bit as i32));

        x = _mm256_mullo_epi32(u2, cospi44);
        y = _mm256_mullo_epi32(u3, cospim20);
        v3 = _mm256_add_epi32(x, y);
        v3 = _mm256_add_epi32(v3, rnding);
        v3 = _mm256_sra_epi32(v3, _mm_cvtsi32_si128(bit as i32));

        x = _mm256_mullo_epi32(u4, cospi36);
        y = _mm256_mullo_epi32(u5, cospi28);
        v4 = _mm256_add_epi32(x, y);
        v4 = _mm256_add_epi32(v4, rnding);
        v4 = _mm256_sra_epi32(v4, _mm_cvtsi32_si128(bit as i32));

        x = _mm256_mullo_epi32(u4, cospi28);
        y = _mm256_mullo_epi32(u5, cospim36);
        v5 = _mm256_add_epi32(x, y);
        v5 = _mm256_add_epi32(v5, rnding);
        v5 = _mm256_sra_epi32(v5, _mm_cvtsi32_si128(bit as i32));

        x = _mm256_mullo_epi32(u6, cospi52);
        y = _mm256_mullo_epi32(u7, cospi12);
        v6 = _mm256_add_epi32(x, y);
        v6 = _mm256_add_epi32(v6, rnding);
        v6 = _mm256_sra_epi32(v6, _mm_cvtsi32_si128(bit as i32));

        x = _mm256_mullo_epi32(u6, cospi12);
        y = _mm256_mullo_epi32(u7, cospim52);
        v7 = _mm256_add_epi32(x, y);
        v7 = _mm256_add_epi32(v7, rnding);
        v7 = _mm256_sra_epi32(v7, _mm_cvtsi32_si128(bit as i32));

        // stage 7
        *out.offset((0 * outstirde + col) as isize) = v1;
        *out.offset((1 * outstirde + col) as isize) = v6;
        *out.offset((2 * outstirde + col) as isize) = v3;
        *out.offset((3 * outstirde + col) as isize) = v4;
        *out.offset((4 * outstirde + col) as isize) = v5;
        *out.offset((5 * outstirde + col) as isize) = v2;
        *out.offset((6 * outstirde + col) as isize) = v7;
        *out.offset((7 * outstirde + col) as isize) = v0;
        col += 1;
    }
}
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn idtx8_avx2(in_: *mut __m256i, out: *mut __m256i, bit: i8, col_num: i32,
                     outstride: i32) {
    let _ = bit;
    let _ = outstride;
    let num_iters: i32 = 8 * col_num;
    let mut i: i32 = 0;
    while i < num_iters {
        *out.offset(i as isize) = _mm256_add_epi32(*in_.offset(i as isize),
                                                   *in_.offset(i as isize));
        *out.offset((i + 1) as isize) = _mm256_add_epi32(*in_.offset((i + 1) as isize),
                                                         *in_.offset((i + 1) as isize));
        *out.offset((i + 2) as isize) = _mm256_add_epi32(*in_.offset((i + 2) as isize),
                                                         *in_.offset((i + 2) as isize));
        *out.offset((i + 3) as isize) = _mm256_add_epi32(*in_.offset((i + 3) as isize),
                                                         *in_.offset((i + 3) as isize));
        *out.offset((i + 4) as isize) = _mm256_add_epi32(*in_.offset((i + 4) as isize),
                                                         *in_.offset((i + 4) as isize));
        *out.offset((i + 5) as isize) = _mm256_add_epi32(*in_.offset((i + 5) as isize),
                                                         *in_.offset((i + 5) as isize));
        *out.offset((i + 6) as isize) = _mm256_add_epi32(*in_.offset((i + 6) as isize),
                                                         *in_.offset((i + 6) as isize));
        *out.offset((i + 7) as isize) = _mm256_add_epi32(*in_.offset((i + 7) as isize),
                                                         *in_.offset((i + 7) as isize));
        i += 8;
    }
}
#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn av1_fwd_txfm2d_8x8_avx2(input: *const i16, coeff: *mut i32,
                                                        stride: i32, tx_type: TX_TYPE, bd: i32) {
    let mut in__: [__m256i; 8] = core::mem::zeroed();
    let mut out_: [__m256i; 8] = core::mem::zeroed();
    let in_ = in__.as_mut_ptr();
    let out = out_.as_mut_ptr();
    let tx_size: TX_SIZE = TX_8X8;
    let shift: *const i8 = av1_fwd_txfm_shift_ls[tx_size as usize];
    let txw_idx: i32 = get_txw_idx(tx_size);
    let txh_idx: i32 = get_txh_idx(tx_size);
    let width: i32 = tx_size_wide[tx_size as usize];
    let width_div8: i32 = width >> 3;

    match tx_type {
        DCT_DCT => {
            load_buffer_8x8_avx2(input, in_, stride, 0, 0, *shift.add(0) as i32);
            fdct8_avx2(in_, out, av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize],
                       width_div8, width_div8);
            col_txfm_8x8_rounding(out, -(*shift.add(1) as i32));
            fwd_txfm_transpose_8x8_avx2(out, in_, width_div8, width_div8);
            fdct8_avx2(in_, out, av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize],
                       width_div8, width_div8);
            store_buffer_avx2(out, coeff, 8, 8);
        }
        ADST_DCT => {
            load_buffer_8x8_avx2(input, in_, stride, 0, 0, *shift.add(0) as i32);
            fadst8_avx2(in_, out, av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize],
                        width_div8, width_div8);
            col_txfm_8x8_rounding(out, -(*shift.add(1) as i32));
            fwd_txfm_transpose_8x8_avx2(out, in_, width_div8, width_div8);
            fdct8_avx2(in_, out, av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize],
                       width_div8, width_div8);
            store_buffer_avx2(out, coeff, 8, 8);
        }
        DCT_ADST => {
            load_buffer_8x8_avx2(input, in_, stride, 0, 0, *shift.add(0) as i32);
            fdct8_avx2(in_, out, av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize],
                       width_div8, width_div8);
            col_txfm_8x8_rounding(out, -(*shift.add(1) as i32));
            fwd_txfm_transpose_8x8_avx2(out, in_, width_div8, width_div8);
            fadst8_avx2(in_, out, av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize],
                        width_div8, width_div8);
            store_buffer_avx2(out, coeff, 8, 8);
        }
        ADST_ADST => {
            load_buffer_8x8_avx2(input, in_, stride, 0, 0, *shift.add(0) as i32);
            fadst8_avx2(in_, out, av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize],
                        width_div8, width_div8);
            col_txfm_8x8_rounding(out, -(*shift.add(1) as i32));
            fwd_txfm_transpose_8x8_avx2(out, in_, width_div8, width_div8);
            fadst8_avx2(in_, out, av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize],
                        width_div8, width_div8);
            store_buffer_avx2(out, coeff, 8, 8);
        }
        FLIPADST_DCT => {
            load_buffer_8x8_avx2(input, in_, stride, 1, 0, *shift.add(0) as i32);
            fadst8_avx2(in_, out, av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize],
                        width_div8, width_div8);
            col_txfm_8x8_rounding(out, -(*shift.add(1) as i32));
            fwd_txfm_transpose_8x8_avx2(out, in_, width_div8, width_div8);
            fdct8_avx2(in_, out, av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize],
                       width_div8, width_div8);
            store_buffer_avx2(out, coeff, 8, 8);
        }
        DCT_FLIPADST => {
            load_buffer_8x8_avx2(input, in_, stride, 0, 1, *shift.add(0) as i32);
            fdct8_avx2(in_, out, av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize],
                       width_div8, width_div8);
            col_txfm_8x8_rounding(out, -(*shift.add(1) as i32));
            fwd_txfm_transpose_8x8_avx2(out, in_, width_div8, width_div8);
            fadst8_avx2(in_, out, av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize],
                        width_div8, width_div8);
            store_buffer_avx2(out, coeff, 8, 8);
        }
        FLIPADST_FLIPADST => {
            load_buffer_8x8_avx2(input, in_, stride, 1, 1, *shift.add(0) as i32);
            fadst8_avx2(in_, out, av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize],
                        width_div8, width_div8);
            col_txfm_8x8_rounding(out, -(*shift.add(1) as i32));
            fwd_txfm_transpose_8x8_avx2(out, in_, width_div8, width_div8);
            fadst8_avx2(in_, out, av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize],
                        width_div8, width_div8);
            store_buffer_avx2(out, coeff, 8, 8);
        }
        ADST_FLIPADST => {
            load_buffer_8x8_avx2(input, in_, stride, 0, 1, *shift.add(0) as i32);
            fadst8_avx2(in_, out, av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize],
                        width_div8, width_div8);
            col_txfm_8x8_rounding(out, -(*shift.add(1) as i32));
            fwd_txfm_transpose_8x8_avx2(out, in_, width_div8, width_div8);
            fadst8_avx2(in_, out, av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize],
                        width_div8, width_div8);
            store_buffer_avx2(out, coeff, 8, 8);
        }
        FLIPADST_ADST => {
            load_buffer_8x8_avx2(input, in_, stride, 1, 0, *shift.add(0) as i32);
            fadst8_avx2(in_, out, av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize],
                        width_div8, width_div8);
            col_txfm_8x8_rounding(out, -(*shift.add(1) as i32));
            fwd_txfm_transpose_8x8_avx2(out, in_, width_div8, width_div8);
            fadst8_avx2(in_, out, av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize],
                        width_div8, width_div8);
            store_buffer_avx2(out, coeff, 8, 8);
        }
        IDTX => {
            load_buffer_8x8_avx2(input, in_, stride, 0, 0, *shift.add(0) as i32);
            idtx8_avx2(in_, out, av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize],
                       width_div8, width_div8);
            col_txfm_8x8_rounding(out, -(*shift.add(1) as i32));
            fwd_txfm_transpose_8x8_avx2(out, in_, width_div8, width_div8);
            idtx8_avx2(in_, out, av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize],
                       width_div8, width_div8);
            store_buffer_avx2(out, coeff, 8, 8);
        }
        V_DCT => {
            load_buffer_8x8_avx2(input, in_, stride, 0, 0, *shift.add(0) as i32);
            fdct8_avx2(in_, out, av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize],
                       width_div8, width_div8);
            col_txfm_8x8_rounding(out, -(*shift.add(1) as i32));
            fwd_txfm_transpose_8x8_avx2(out, in_, width_div8, width_div8);
            idtx8_avx2(in_, out, av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize],
                       width_div8, width_div8);
            store_buffer_avx2(out, coeff, 8, 8);
        }
        H_DCT => {
            load_buffer_8x8_avx2(input, in_, stride, 0, 0, *shift.add(0) as i32);
            idtx8_avx2(in_, out, av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize],
                       width_div8, width_div8);
            col_txfm_8x8_rounding(out, -(*shift.add(1) as i32));
            fwd_txfm_transpose_8x8_avx2(out, in_, width_div8, width_div8);
            fdct8_avx2(in_, out, av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize],
                       width_div8, width_div8);
            store_buffer_avx2(out, coeff, 8, 8);
        }
        V_ADST => {
            load_buffer_8x8_avx2(input, in_, stride, 0, 0, *shift.add(0) as i32);
            fadst8_avx2(in_, out, av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize],
                        width_div8, width_div8);
            col_txfm_8x8_rounding(out, -(*shift.add(1) as i32));
            fwd_txfm_transpose_8x8_avx2(out, in_, width_div8, width_div8);
            idtx8_avx2(in_, out, av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize],
                       width_div8, width_div8);
            store_buffer_avx2(out, coeff, 8, 8);
        }
        H_ADST => {
            load_buffer_8x8_avx2(input, in_, stride, 0, 0, *shift.add(0) as i32);
            idtx8_avx2(in_, out, av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize],
                       width_div8, width_div8);
            col_txfm_8x8_rounding(out, -(*shift.add(1) as i32));
            fwd_txfm_transpose_8x8_avx2(out, in_, width_div8, width_div8);
            fadst8_avx2(in_, out, av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize],
                        width_div8, width_div8);
            store_buffer_avx2(out, coeff, 8, 8);
        }
        V_FLIPADST => {
            load_buffer_8x8_avx2(input, in_, stride, 1, 0, *shift.add(0) as i32);
            fadst8_avx2(in_, out, av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize],
                        width_div8, width_div8);
            col_txfm_8x8_rounding(out, -(*shift.add(1) as i32));
            fwd_txfm_transpose_8x8_avx2(out, in_, width_div8, width_div8);
            idtx8_avx2(in_, out, av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize],
                       width_div8, width_div8);
            store_buffer_avx2(out, coeff, 8, 8);
        }
        H_FLIPADST => {
            load_buffer_8x8_avx2(input, in_, stride, 0, 1, *shift.add(0) as i32);
            idtx8_avx2(in_, out, av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize],
                       width_div8, width_div8);
            col_txfm_8x8_rounding(out, -(*shift.add(1) as i32));
            fwd_txfm_transpose_8x8_avx2(out, in_, width_div8, width_div8);
            fadst8_avx2(in_, out, av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize],
                        width_div8, width_div8);
            store_buffer_avx2(out, coeff, 8, 8);
        }
        _ => debug_assert!(false),
    }
    let _ = bd;
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn fdct16_avx2(in_: *mut __m256i, out: *mut __m256i, bit: i8, col_num: i32,
                      outstride: i32) {
    let cospi: *const i32 = cospi_arr(bit as i32);
    let cospi32: __m256i = _mm256_set1_epi32(*cospi.add(32));
    let cospim32: __m256i = _mm256_set1_epi32(-*cospi.add(32));
    let cospi48: __m256i = _mm256_set1_epi32(*cospi.add(48));
    let cospi16: __m256i = _mm256_set1_epi32(*cospi.add(16));
    let cospim48: __m256i = _mm256_set1_epi32(-*cospi.add(48));
    let cospim16: __m256i = _mm256_set1_epi32(-*cospi.add(16));
    let cospi56: __m256i = _mm256_set1_epi32(*cospi.add(56));
    let cospi8: __m256i = _mm256_set1_epi32(*cospi.add(8));
    let cospi24: __m256i = _mm256_set1_epi32(*cospi.add(24));
    let cospi40: __m256i = _mm256_set1_epi32(*cospi.add(40));
    let cospi60: __m256i = _mm256_set1_epi32(*cospi.add(60));
    let cospi4: __m256i = _mm256_set1_epi32(*cospi.add(4));
    let cospi28: __m256i = _mm256_set1_epi32(*cospi.add(28));
    let cospi36: __m256i = _mm256_set1_epi32(*cospi.add(36));
    let cospi44: __m256i = _mm256_set1_epi32(*cospi.add(44));
    let cospi20: __m256i = _mm256_set1_epi32(*cospi.add(20));
    let cospi12: __m256i = _mm256_set1_epi32(*cospi.add(12));
    let cospi52: __m256i = _mm256_set1_epi32(*cospi.add(52));
    let rnding: __m256i = _mm256_set1_epi32(1 << (bit - 1));
    let mut u_: [__m256i; 16] = core::mem::zeroed();
    let mut v_: [__m256i; 16] = core::mem::zeroed();
    let u = u_.as_mut_ptr();
    let v = v_.as_mut_ptr();
    let mut x: __m256i;
    let mut col: i32;

    // Calculate the column 0, 1, 2, 3
    col = 0;
    while col < col_num {
        // stage 0
        // stage 1
        *u.add(0) = _mm256_add_epi32(*in_.offset((0 * col_num + col) as isize),
                                     *in_.offset((15 * col_num + col) as isize));
        *u.add(15) = _mm256_sub_epi32(*in_.offset((0 * col_num + col) as isize),
                                      *in_.offset((15 * col_num + col) as isize));
        *u.add(1) = _mm256_add_epi32(*in_.offset((1 * col_num + col) as isize),
                                     *in_.offset((14 * col_num + col) as isize));
        *u.add(14) = _mm256_sub_epi32(*in_.offset((1 * col_num + col) as isize),
                                      *in_.offset((14 * col_num + col) as isize));
        *u.add(2) = _mm256_add_epi32(*in_.offset((2 * col_num + col) as isize),
                                     *in_.offset((13 * col_num + col) as isize));
        *u.add(13) = _mm256_sub_epi32(*in_.offset((2 * col_num + col) as isize),
                                      *in_.offset((13 * col_num + col) as isize));
        *u.add(3) = _mm256_add_epi32(*in_.offset((3 * col_num + col) as isize),
                                     *in_.offset((12 * col_num + col) as isize));
        *u.add(12) = _mm256_sub_epi32(*in_.offset((3 * col_num + col) as isize),
                                      *in_.offset((12 * col_num + col) as isize));
        *u.add(4) = _mm256_add_epi32(*in_.offset((4 * col_num + col) as isize),
                                     *in_.offset((11 * col_num + col) as isize));
        *u.add(11) = _mm256_sub_epi32(*in_.offset((4 * col_num + col) as isize),
                                      *in_.offset((11 * col_num + col) as isize));
        *u.add(5) = _mm256_add_epi32(*in_.offset((5 * col_num + col) as isize),
                                     *in_.offset((10 * col_num + col) as isize));
        *u.add(10) = _mm256_sub_epi32(*in_.offset((5 * col_num + col) as isize),
                                      *in_.offset((10 * col_num + col) as isize));
        *u.add(6) = _mm256_add_epi32(*in_.offset((6 * col_num + col) as isize),
                                     *in_.offset((9 * col_num + col) as isize));
        *u.add(9) = _mm256_sub_epi32(*in_.offset((6 * col_num + col) as isize),
                                     *in_.offset((9 * col_num + col) as isize));
        *u.add(7) = _mm256_add_epi32(*in_.offset((7 * col_num + col) as isize),
                                     *in_.offset((8 * col_num + col) as isize));
        *u.add(8) = _mm256_sub_epi32(*in_.offset((7 * col_num + col) as isize),
                                     *in_.offset((8 * col_num + col) as isize));

        // stage 2
        *v.add(0) = _mm256_add_epi32(*u.add(0), *u.add(7));
        *v.add(7) = _mm256_sub_epi32(*u.add(0), *u.add(7));
        *v.add(1) = _mm256_add_epi32(*u.add(1), *u.add(6));
        *v.add(6) = _mm256_sub_epi32(*u.add(1), *u.add(6));
        *v.add(2) = _mm256_add_epi32(*u.add(2), *u.add(5));
        *v.add(5) = _mm256_sub_epi32(*u.add(2), *u.add(5));
        *v.add(3) = _mm256_add_epi32(*u.add(3), *u.add(4));
        *v.add(4) = _mm256_sub_epi32(*u.add(3), *u.add(4));
        *v.add(8) = *u.add(8);
        *v.add(9) = *u.add(9);

        *v.add(10) = _mm256_mullo_epi32(*u.add(10), cospim32);
        x = _mm256_mullo_epi32(*u.add(13), cospi32);
        *v.add(10) = _mm256_add_epi32(*v.add(10), x);
        *v.add(10) = _mm256_add_epi32(*v.add(10), rnding);
        *v.add(10) = _mm256_sra_epi32(*v.add(10), _mm_cvtsi32_si128(bit as i32));

        *v.add(13) = _mm256_mullo_epi32(*u.add(10), cospi32);
        x = _mm256_mullo_epi32(*u.add(13), cospim32);
        *v.add(13) = _mm256_sub_epi32(*v.add(13), x);
        *v.add(13) = _mm256_add_epi32(*v.add(13), rnding);
        *v.add(13) = _mm256_sra_epi32(*v.add(13), _mm_cvtsi32_si128(bit as i32));

        *v.add(11) = _mm256_mullo_epi32(*u.add(11), cospim32);
        x = _mm256_mullo_epi32(*u.add(12), cospi32);
        *v.add(11) = _mm256_add_epi32(*v.add(11), x);
        *v.add(11) = _mm256_add_epi32(*v.add(11), rnding);
        *v.add(11) = _mm256_sra_epi32(*v.add(11), _mm_cvtsi32_si128(bit as i32));

        *v.add(12) = _mm256_mullo_epi32(*u.add(11), cospi32);
        x = _mm256_mullo_epi32(*u.add(12), cospim32);
        *v.add(12) = _mm256_sub_epi32(*v.add(12), x);
        *v.add(12) = _mm256_add_epi32(*v.add(12), rnding);
        *v.add(12) = _mm256_sra_epi32(*v.add(12), _mm_cvtsi32_si128(bit as i32));
        *v.add(14) = *u.add(14);
        *v.add(15) = *u.add(15);

        // stage 3
        *u.add(0) = _mm256_add_epi32(*v.add(0), *v.add(3));
        *u.add(3) = _mm256_sub_epi32(*v.add(0), *v.add(3));
        *u.add(1) = _mm256_add_epi32(*v.add(1), *v.add(2));
        *u.add(2) = _mm256_sub_epi32(*v.add(1), *v.add(2));
        *u.add(4) = *v.add(4);

        *u.add(5) = _mm256_mullo_epi32(*v.add(5), cospim32);
        x = _mm256_mullo_epi32(*v.add(6), cospi32);
        *u.add(5) = _mm256_add_epi32(*u.add(5), x);
        *u.add(5) = _mm256_add_epi32(*u.add(5), rnding);
        *u.add(5) = _mm256_sra_epi32(*u.add(5), _mm_cvtsi32_si128(bit as i32));

        *u.add(6) = _mm256_mullo_epi32(*v.add(5), cospi32);
        x = _mm256_mullo_epi32(*v.add(6), cospim32);
        *u.add(6) = _mm256_sub_epi32(*u.add(6), x);
        *u.add(6) = _mm256_add_epi32(*u.add(6), rnding);
        *u.add(6) = _mm256_sra_epi32(*u.add(6), _mm_cvtsi32_si128(bit as i32));

        *u.add(7) = *v.add(7);
        *u.add(8) = _mm256_add_epi32(*v.add(8), *v.add(11));
        *u.add(11) = _mm256_sub_epi32(*v.add(8), *v.add(11));
        *u.add(9) = _mm256_add_epi32(*v.add(9), *v.add(10));
        *u.add(10) = _mm256_sub_epi32(*v.add(9), *v.add(10));
        *u.add(12) = _mm256_sub_epi32(*v.add(15), *v.add(12));
        *u.add(15) = _mm256_add_epi32(*v.add(15), *v.add(12));
        *u.add(13) = _mm256_sub_epi32(*v.add(14), *v.add(13));
        *u.add(14) = _mm256_add_epi32(*v.add(14), *v.add(13));

        // stage 4
        *u.add(0) = _mm256_mullo_epi32(*u.add(0), cospi32);
        *u.add(1) = _mm256_mullo_epi32(*u.add(1), cospi32);
        *v.add(0) = _mm256_add_epi32(*u.add(0), *u.add(1));
        *v.add(0) = _mm256_add_epi32(*v.add(0), rnding);
        *v.add(0) = _mm256_sra_epi32(*v.add(0), _mm_cvtsi32_si128(bit as i32));

        *v.add(1) = _mm256_sub_epi32(*u.add(0), *u.add(1));
        *v.add(1) = _mm256_add_epi32(*v.add(1), rnding);
        *v.add(1) = _mm256_sra_epi32(*v.add(1), _mm_cvtsi32_si128(bit as i32));

        *v.add(2) = _mm256_mullo_epi32(*u.add(2), cospi48);
        x = _mm256_mullo_epi32(*u.add(3), cospi16);
        *v.add(2) = _mm256_add_epi32(*v.add(2), x);
        *v.add(2) = _mm256_add_epi32(*v.add(2), rnding);
        *v.add(2) = _mm256_sra_epi32(*v.add(2), _mm_cvtsi32_si128(bit as i32));

        *v.add(3) = _mm256_mullo_epi32(*u.add(2), cospi16);
        x = _mm256_mullo_epi32(*u.add(3), cospi48);
        *v.add(3) = _mm256_sub_epi32(x, *v.add(3));
        *v.add(3) = _mm256_add_epi32(*v.add(3), rnding);
        *v.add(3) = _mm256_sra_epi32(*v.add(3), _mm_cvtsi32_si128(bit as i32));

        *v.add(4) = _mm256_add_epi32(*u.add(4), *u.add(5));
        *v.add(5) = _mm256_sub_epi32(*u.add(4), *u.add(5));
        *v.add(6) = _mm256_sub_epi32(*u.add(7), *u.add(6));
        *v.add(7) = _mm256_add_epi32(*u.add(7), *u.add(6));
        *v.add(8) = *u.add(8);

        *v.add(9) = _mm256_mullo_epi32(*u.add(9), cospim16);
        x = _mm256_mullo_epi32(*u.add(14), cospi48);
        *v.add(9) = _mm256_add_epi32(*v.add(9), x);
        *v.add(9) = _mm256_add_epi32(*v.add(9), rnding);
        *v.add(9) = _mm256_sra_epi32(*v.add(9), _mm_cvtsi32_si128(bit as i32));

        *v.add(14) = _mm256_mullo_epi32(*u.add(9), cospi48);
        x = _mm256_mullo_epi32(*u.add(14), cospim16);
        *v.add(14) = _mm256_sub_epi32(*v.add(14), x);
        *v.add(14) = _mm256_add_epi32(*v.add(14), rnding);
        *v.add(14) = _mm256_sra_epi32(*v.add(14), _mm_cvtsi32_si128(bit as i32));

        *v.add(10) = _mm256_mullo_epi32(*u.add(10), cospim48);
        x = _mm256_mullo_epi32(*u.add(13), cospim16);
        *v.add(10) = _mm256_add_epi32(*v.add(10), x);
        *v.add(10) = _mm256_add_epi32(*v.add(10), rnding);
        *v.add(10) = _mm256_sra_epi32(*v.add(10), _mm_cvtsi32_si128(bit as i32));

        *v.add(13) = _mm256_mullo_epi32(*u.add(10), cospim16);
        x = _mm256_mullo_epi32(*u.add(13), cospim48);
        *v.add(13) = _mm256_sub_epi32(*v.add(13), x);
        *v.add(13) = _mm256_add_epi32(*v.add(13), rnding);
        *v.add(13) = _mm256_sra_epi32(*v.add(13), _mm_cvtsi32_si128(bit as i32));

        *v.add(11) = *u.add(11);
        *v.add(12) = *u.add(12);
        *v.add(15) = *u.add(15);

        // stage 5
        *u.add(0) = *v.add(0);
        *u.add(1) = *v.add(1);
        *u.add(2) = *v.add(2);
        *u.add(3) = *v.add(3);

        *u.add(4) = _mm256_mullo_epi32(*v.add(4), cospi56);
        x = _mm256_mullo_epi32(*v.add(7), cospi8);
        *u.add(4) = _mm256_add_epi32(*u.add(4), x);
        *u.add(4) = _mm256_add_epi32(*u.add(4), rnding);
        *u.add(4) = _mm256_sra_epi32(*u.add(4), _mm_cvtsi32_si128(bit as i32));

        *u.add(7) = _mm256_mullo_epi32(*v.add(4), cospi8);
        x = _mm256_mullo_epi32(*v.add(7), cospi56);
        *u.add(7) = _mm256_sub_epi32(x, *u.add(7));
        *u.add(7) = _mm256_add_epi32(*u.add(7), rnding);
        *u.add(7) = _mm256_sra_epi32(*u.add(7), _mm_cvtsi32_si128(bit as i32));

        *u.add(5) = _mm256_mullo_epi32(*v.add(5), cospi24);
        x = _mm256_mullo_epi32(*v.add(6), cospi40);
        *u.add(5) = _mm256_add_epi32(*u.add(5), x);
        *u.add(5) = _mm256_add_epi32(*u.add(5), rnding);
        *u.add(5) = _mm256_sra_epi32(*u.add(5), _mm_cvtsi32_si128(bit as i32));

        *u.add(6) = _mm256_mullo_epi32(*v.add(5), cospi40);
        x = _mm256_mullo_epi32(*v.add(6), cospi24);
        *u.add(6) = _mm256_sub_epi32(x, *u.add(6));
        *u.add(6) = _mm256_add_epi32(*u.add(6), rnding);
        *u.add(6) = _mm256_sra_epi32(*u.add(6), _mm_cvtsi32_si128(bit as i32));

        *u.add(8) = _mm256_add_epi32(*v.add(8), *v.add(9));
        *u.add(9) = _mm256_sub_epi32(*v.add(8), *v.add(9));
        *u.add(10) = _mm256_sub_epi32(*v.add(11), *v.add(10));
        *u.add(11) = _mm256_add_epi32(*v.add(11), *v.add(10));
        *u.add(12) = _mm256_add_epi32(*v.add(12), *v.add(13));
        *u.add(13) = _mm256_sub_epi32(*v.add(12), *v.add(13));
        *u.add(14) = _mm256_sub_epi32(*v.add(15), *v.add(14));
        *u.add(15) = _mm256_add_epi32(*v.add(15), *v.add(14));

        // stage 6
        *v.add(0) = *u.add(0);
        *v.add(1) = *u.add(1);
        *v.add(2) = *u.add(2);
        *v.add(3) = *u.add(3);
        *v.add(4) = *u.add(4);
        *v.add(5) = *u.add(5);
        *v.add(6) = *u.add(6);
        *v.add(7) = *u.add(7);

        *v.add(8) = _mm256_mullo_epi32(*u.add(8), cospi60);
        x = _mm256_mullo_epi32(*u.add(15), cospi4);
        *v.add(8) = _mm256_add_epi32(*v.add(8), x);
        *v.add(8) = _mm256_add_epi32(*v.add(8), rnding);
        *v.add(8) = _mm256_sra_epi32(*v.add(8), _mm_cvtsi32_si128(bit as i32));

        *v.add(15) = _mm256_mullo_epi32(*u.add(8), cospi4);
        x = _mm256_mullo_epi32(*u.add(15), cospi60);
        *v.add(15) = _mm256_sub_epi32(x, *v.add(15));
        *v.add(15) = _mm256_add_epi32(*v.add(15), rnding);
        *v.add(15) = _mm256_sra_epi32(*v.add(15), _mm_cvtsi32_si128(bit as i32));

        *v.add(9) = _mm256_mullo_epi32(*u.add(9), cospi28);
        x = _mm256_mullo_epi32(*u.add(14), cospi36);
        *v.add(9) = _mm256_add_epi32(*v.add(9), x);
        *v.add(9) = _mm256_add_epi32(*v.add(9), rnding);
        *v.add(9) = _mm256_sra_epi32(*v.add(9), _mm_cvtsi32_si128(bit as i32));

        *v.add(14) = _mm256_mullo_epi32(*u.add(9), cospi36);
        x = _mm256_mullo_epi32(*u.add(14), cospi28);
        *v.add(14) = _mm256_sub_epi32(x, *v.add(14));
        *v.add(14) = _mm256_add_epi32(*v.add(14), rnding);
        *v.add(14) = _mm256_sra_epi32(*v.add(14), _mm_cvtsi32_si128(bit as i32));

        *v.add(10) = _mm256_mullo_epi32(*u.add(10), cospi44);
        x = _mm256_mullo_epi32(*u.add(13), cospi20);
        *v.add(10) = _mm256_add_epi32(*v.add(10), x);
        *v.add(10) = _mm256_add_epi32(*v.add(10), rnding);
        *v.add(10) = _mm256_sra_epi32(*v.add(10), _mm_cvtsi32_si128(bit as i32));

        *v.add(13) = _mm256_mullo_epi32(*u.add(10), cospi20);
        x = _mm256_mullo_epi32(*u.add(13), cospi44);
        *v.add(13) = _mm256_sub_epi32(x, *v.add(13));
        *v.add(13) = _mm256_add_epi32(*v.add(13), rnding);
        *v.add(13) = _mm256_sra_epi32(*v.add(13), _mm_cvtsi32_si128(bit as i32));

        *v.add(11) = _mm256_mullo_epi32(*u.add(11), cospi12);
        x = _mm256_mullo_epi32(*u.add(12), cospi52);
        *v.add(11) = _mm256_add_epi32(*v.add(11), x);
        *v.add(11) = _mm256_add_epi32(*v.add(11), rnding);
        *v.add(11) = _mm256_sra_epi32(*v.add(11), _mm_cvtsi32_si128(bit as i32));

        *v.add(12) = _mm256_mullo_epi32(*u.add(11), cospi52);
        x = _mm256_mullo_epi32(*u.add(12), cospi12);
        *v.add(12) = _mm256_sub_epi32(x, *v.add(12));
        *v.add(12) = _mm256_add_epi32(*v.add(12), rnding);
        *v.add(12) = _mm256_sra_epi32(*v.add(12), _mm_cvtsi32_si128(bit as i32));

        *out.offset((0 * outstride + col) as isize) = *v.add(0);
        *out.offset((1 * outstride + col) as isize) = *v.add(8);
        *out.offset((2 * outstride + col) as isize) = *v.add(4);
        *out.offset((3 * outstride + col) as isize) = *v.add(12);
        *out.offset((4 * outstride + col) as isize) = *v.add(2);
        *out.offset((5 * outstride + col) as isize) = *v.add(10);
        *out.offset((6 * outstride + col) as isize) = *v.add(6);
        *out.offset((7 * outstride + col) as isize) = *v.add(14);
        *out.offset((8 * outstride + col) as isize) = *v.add(1);
        *out.offset((9 * outstride + col) as isize) = *v.add(9);
        *out.offset((10 * outstride + col) as isize) = *v.add(5);
        *out.offset((11 * outstride + col) as isize) = *v.add(13);
        *out.offset((12 * outstride + col) as isize) = *v.add(3);
        *out.offset((13 * outstride + col) as isize) = *v.add(11);
        *out.offset((14 * outstride + col) as isize) = *v.add(7);
        *out.offset((15 * outstride + col) as isize) = *v.add(15);
        col += 1;
    }
}
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn fadst16_avx2(in_: *mut __m256i, out: *mut __m256i, bit: i8, num_cols: i32,
                       outstride: i32) {
    let cospi: *const i32 = cospi_arr(bit as i32);
    let cospi32: __m256i = _mm256_set1_epi32(*cospi.add(32));
    let cospi48: __m256i = _mm256_set1_epi32(*cospi.add(48));
    let cospi16: __m256i = _mm256_set1_epi32(*cospi.add(16));
    let cospim16: __m256i = _mm256_set1_epi32(-*cospi.add(16));
    let cospim48: __m256i = _mm256_set1_epi32(-*cospi.add(48));
    let cospi8: __m256i = _mm256_set1_epi32(*cospi.add(8));
    let cospi56: __m256i = _mm256_set1_epi32(*cospi.add(56));
    let cospim56: __m256i = _mm256_set1_epi32(-*cospi.add(56));
    let cospim8: __m256i = _mm256_set1_epi32(-*cospi.add(8));
    let cospi24: __m256i = _mm256_set1_epi32(*cospi.add(24));
    let cospim24: __m256i = _mm256_set1_epi32(-*cospi.add(24));
    let cospim40: __m256i = _mm256_set1_epi32(-*cospi.add(40));
    let cospi40: __m256i = _mm256_set1_epi32(*cospi.add(40));
    let cospi2: __m256i = _mm256_set1_epi32(*cospi.add(2));
    let cospi62: __m256i = _mm256_set1_epi32(*cospi.add(62));
    let cospim2: __m256i = _mm256_set1_epi32(-*cospi.add(2));
    let cospi10: __m256i = _mm256_set1_epi32(*cospi.add(10));
    let cospi54: __m256i = _mm256_set1_epi32(*cospi.add(54));
    let cospim10: __m256i = _mm256_set1_epi32(-*cospi.add(10));
    let cospi18: __m256i = _mm256_set1_epi32(*cospi.add(18));
    let cospi46: __m256i = _mm256_set1_epi32(*cospi.add(46));
    let cospim18: __m256i = _mm256_set1_epi32(-*cospi.add(18));
    let cospi26: __m256i = _mm256_set1_epi32(*cospi.add(26));
    let cospi38: __m256i = _mm256_set1_epi32(*cospi.add(38));
    let cospim26: __m256i = _mm256_set1_epi32(-*cospi.add(26));
    let cospi34: __m256i = _mm256_set1_epi32(*cospi.add(34));
    let cospi30: __m256i = _mm256_set1_epi32(*cospi.add(30));
    let cospim34: __m256i = _mm256_set1_epi32(-*cospi.add(34));
    let cospi42: __m256i = _mm256_set1_epi32(*cospi.add(42));
    let cospi22: __m256i = _mm256_set1_epi32(*cospi.add(22));
    let cospim42: __m256i = _mm256_set1_epi32(-*cospi.add(42));
    let cospi50: __m256i = _mm256_set1_epi32(*cospi.add(50));
    let cospi14: __m256i = _mm256_set1_epi32(*cospi.add(14));
    let cospim50: __m256i = _mm256_set1_epi32(-*cospi.add(50));
    let cospi58: __m256i = _mm256_set1_epi32(*cospi.add(58));
    let cospi6: __m256i = _mm256_set1_epi32(*cospi.add(6));
    let cospim58: __m256i = _mm256_set1_epi32(-*cospi.add(58));
    let rnding: __m256i = _mm256_set1_epi32(1 << (bit - 1));
    let zero: __m256i = _mm256_setzero_si256();

    let mut u_: [__m256i; 16] = core::mem::zeroed();
    let mut v_: [__m256i; 16] = core::mem::zeroed();
    let u = u_.as_mut_ptr();
    let v = v_.as_mut_ptr();
    let mut x: __m256i;
    let mut y: __m256i;
    let mut col: i32;

    col = 0;
    while col < num_cols {
        // stage 0
        // stage 1
        *u.add(0) = *in_.offset((0 * num_cols + col) as isize);
        *u.add(1) = _mm256_sub_epi32(zero, *in_.offset((15 * num_cols + col) as isize));
        *u.add(2) = _mm256_sub_epi32(zero, *in_.offset((7 * num_cols + col) as isize));
        *u.add(3) = *in_.offset((8 * num_cols + col) as isize);
        *u.add(4) = _mm256_sub_epi32(zero, *in_.offset((3 * num_cols + col) as isize));
        *u.add(5) = *in_.offset((12 * num_cols + col) as isize);
        *u.add(6) = *in_.offset((4 * num_cols + col) as isize);
        *u.add(7) = _mm256_sub_epi32(zero, *in_.offset((11 * num_cols + col) as isize));
        *u.add(8) = _mm256_sub_epi32(zero, *in_.offset((1 * num_cols + col) as isize));
        *u.add(9) = *in_.offset((14 * num_cols + col) as isize);
        *u.add(10) = *in_.offset((6 * num_cols + col) as isize);
        *u.add(11) = _mm256_sub_epi32(zero, *in_.offset((9 * num_cols + col) as isize));
        *u.add(12) = *in_.offset((2 * num_cols + col) as isize);
        *u.add(13) = _mm256_sub_epi32(zero, *in_.offset((13 * num_cols + col) as isize));
        *u.add(14) = _mm256_sub_epi32(zero, *in_.offset((5 * num_cols + col) as isize));
        *u.add(15) = *in_.offset((10 * num_cols + col) as isize);

        // stage 2
        *v.add(0) = *u.add(0);
        *v.add(1) = *u.add(1);

        x = _mm256_mullo_epi32(*u.add(2), cospi32);
        y = _mm256_mullo_epi32(*u.add(3), cospi32);
        *v.add(2) = _mm256_add_epi32(x, y);
        *v.add(2) = _mm256_add_epi32(*v.add(2), rnding);
        *v.add(2) = _mm256_sra_epi32(*v.add(2), _mm_cvtsi32_si128(bit as i32));

        *v.add(3) = _mm256_sub_epi32(x, y);
        *v.add(3) = _mm256_add_epi32(*v.add(3), rnding);
        *v.add(3) = _mm256_sra_epi32(*v.add(3), _mm_cvtsi32_si128(bit as i32));

        *v.add(4) = *u.add(4);
        *v.add(5) = *u.add(5);

        x = _mm256_mullo_epi32(*u.add(6), cospi32);
        y = _mm256_mullo_epi32(*u.add(7), cospi32);
        *v.add(6) = _mm256_add_epi32(x, y);
        *v.add(6) = _mm256_add_epi32(*v.add(6), rnding);
        *v.add(6) = _mm256_sra_epi32(*v.add(6), _mm_cvtsi32_si128(bit as i32));

        *v.add(7) = _mm256_sub_epi32(x, y);
        *v.add(7) = _mm256_add_epi32(*v.add(7), rnding);
        *v.add(7) = _mm256_sra_epi32(*v.add(7), _mm_cvtsi32_si128(bit as i32));

        *v.add(8) = *u.add(8);
        *v.add(9) = *u.add(9);

        x = _mm256_mullo_epi32(*u.add(10), cospi32);
        y = _mm256_mullo_epi32(*u.add(11), cospi32);
        *v.add(10) = _mm256_add_epi32(x, y);
        *v.add(10) = _mm256_add_epi32(*v.add(10), rnding);
        *v.add(10) = _mm256_sra_epi32(*v.add(10), _mm_cvtsi32_si128(bit as i32));

        *v.add(11) = _mm256_sub_epi32(x, y);
        *v.add(11) = _mm256_add_epi32(*v.add(11), rnding);
        *v.add(11) = _mm256_sra_epi32(*v.add(11), _mm_cvtsi32_si128(bit as i32));

        *v.add(12) = *u.add(12);
        *v.add(13) = *u.add(13);

        x = _mm256_mullo_epi32(*u.add(14), cospi32);
        y = _mm256_mullo_epi32(*u.add(15), cospi32);
        *v.add(14) = _mm256_add_epi32(x, y);
        *v.add(14) = _mm256_add_epi32(*v.add(14), rnding);
        *v.add(14) = _mm256_sra_epi32(*v.add(14), _mm_cvtsi32_si128(bit as i32));

        *v.add(15) = _mm256_sub_epi32(x, y);
        *v.add(15) = _mm256_add_epi32(*v.add(15), rnding);
        *v.add(15) = _mm256_sra_epi32(*v.add(15), _mm_cvtsi32_si128(bit as i32));

        // stage 3
        *u.add(0) = _mm256_add_epi32(*v.add(0), *v.add(2));
        *u.add(1) = _mm256_add_epi32(*v.add(1), *v.add(3));
        *u.add(2) = _mm256_sub_epi32(*v.add(0), *v.add(2));
        *u.add(3) = _mm256_sub_epi32(*v.add(1), *v.add(3));
        *u.add(4) = _mm256_add_epi32(*v.add(4), *v.add(6));
        *u.add(5) = _mm256_add_epi32(*v.add(5), *v.add(7));
        *u.add(6) = _mm256_sub_epi32(*v.add(4), *v.add(6));
        *u.add(7) = _mm256_sub_epi32(*v.add(5), *v.add(7));
        *u.add(8) = _mm256_add_epi32(*v.add(8), *v.add(10));
        *u.add(9) = _mm256_add_epi32(*v.add(9), *v.add(11));
        *u.add(10) = _mm256_sub_epi32(*v.add(8), *v.add(10));
        *u.add(11) = _mm256_sub_epi32(*v.add(9), *v.add(11));
        *u.add(12) = _mm256_add_epi32(*v.add(12), *v.add(14));
        *u.add(13) = _mm256_add_epi32(*v.add(13), *v.add(15));
        *u.add(14) = _mm256_sub_epi32(*v.add(12), *v.add(14));
        *u.add(15) = _mm256_sub_epi32(*v.add(13), *v.add(15));

        // stage 4
        *v.add(0) = *u.add(0);
        *v.add(1) = *u.add(1);
        *v.add(2) = *u.add(2);
        *v.add(3) = *u.add(3);
        *v.add(4) = av1_half_btf_avx2(&cospi16, u.add(4), &cospi48, u.add(5), &rnding, bit as i32);
        *v.add(5) = av1_half_btf_avx2(&cospi48, u.add(4), &cospim16, u.add(5), &rnding, bit as i32);
        *v.add(6) = av1_half_btf_avx2(&cospim48, u.add(6), &cospi16, u.add(7), &rnding, bit as i32);
        *v.add(7) = av1_half_btf_avx2(&cospi16, u.add(6), &cospi48, u.add(7), &rnding, bit as i32);
        *v.add(8) = *u.add(8);
        *v.add(9) = *u.add(9);
        *v.add(10) = *u.add(10);
        *v.add(11) = *u.add(11);
        *v.add(12) = av1_half_btf_avx2(&cospi16, u.add(12), &cospi48, u.add(13), &rnding,
                                       bit as i32);
        *v.add(13) = av1_half_btf_avx2(&cospi48, u.add(12), &cospim16, u.add(13), &rnding,
                                       bit as i32);
        *v.add(14) = av1_half_btf_avx2(&cospim48, u.add(14), &cospi16, u.add(15), &rnding,
                                       bit as i32);
        *v.add(15) = av1_half_btf_avx2(&cospi16, u.add(14), &cospi48, u.add(15), &rnding,
                                       bit as i32);

        // stage 5
        *u.add(0) = _mm256_add_epi32(*v.add(0), *v.add(4));
        *u.add(1) = _mm256_add_epi32(*v.add(1), *v.add(5));
        *u.add(2) = _mm256_add_epi32(*v.add(2), *v.add(6));
        *u.add(3) = _mm256_add_epi32(*v.add(3), *v.add(7));
        *u.add(4) = _mm256_sub_epi32(*v.add(0), *v.add(4));
        *u.add(5) = _mm256_sub_epi32(*v.add(1), *v.add(5));
        *u.add(6) = _mm256_sub_epi32(*v.add(2), *v.add(6));
        *u.add(7) = _mm256_sub_epi32(*v.add(3), *v.add(7));
        *u.add(8) = _mm256_add_epi32(*v.add(8), *v.add(12));
        *u.add(9) = _mm256_add_epi32(*v.add(9), *v.add(13));
        *u.add(10) = _mm256_add_epi32(*v.add(10), *v.add(14));
        *u.add(11) = _mm256_add_epi32(*v.add(11), *v.add(15));
        *u.add(12) = _mm256_sub_epi32(*v.add(8), *v.add(12));
        *u.add(13) = _mm256_sub_epi32(*v.add(9), *v.add(13));
        *u.add(14) = _mm256_sub_epi32(*v.add(10), *v.add(14));
        *u.add(15) = _mm256_sub_epi32(*v.add(11), *v.add(15));

        // stage 6
        *v.add(0) = *u.add(0);
        *v.add(1) = *u.add(1);
        *v.add(2) = *u.add(2);
        *v.add(3) = *u.add(3);
        *v.add(4) = *u.add(4);
        *v.add(5) = *u.add(5);
        *v.add(6) = *u.add(6);
        *v.add(7) = *u.add(7);
        *v.add(8) = av1_half_btf_avx2(&cospi8, u.add(8), &cospi56, u.add(9), &rnding, bit as i32);
        *v.add(9) = av1_half_btf_avx2(&cospi56, u.add(8), &cospim8, u.add(9), &rnding, bit as i32);
        *v.add(10) = av1_half_btf_avx2(&cospi40, u.add(10), &cospi24, u.add(11), &rnding,
                                       bit as i32);
        *v.add(11) = av1_half_btf_avx2(&cospi24, u.add(10), &cospim40, u.add(11), &rnding,
                                       bit as i32);
        *v.add(12) = av1_half_btf_avx2(&cospim56, u.add(12), &cospi8, u.add(13), &rnding,
                                       bit as i32);
        *v.add(13) = av1_half_btf_avx2(&cospi8, u.add(12), &cospi56, u.add(13), &rnding,
                                       bit as i32);
        *v.add(14) = av1_half_btf_avx2(&cospim24, u.add(14), &cospi40, u.add(15), &rnding,
                                       bit as i32);
        *v.add(15) = av1_half_btf_avx2(&cospi40, u.add(14), &cospi24, u.add(15), &rnding,
                                       bit as i32);

        // stage 7
        *u.add(0) = _mm256_add_epi32(*v.add(0), *v.add(8));
        *u.add(1) = _mm256_add_epi32(*v.add(1), *v.add(9));
        *u.add(2) = _mm256_add_epi32(*v.add(2), *v.add(10));
        *u.add(3) = _mm256_add_epi32(*v.add(3), *v.add(11));
        *u.add(4) = _mm256_add_epi32(*v.add(4), *v.add(12));
        *u.add(5) = _mm256_add_epi32(*v.add(5), *v.add(13));
        *u.add(6) = _mm256_add_epi32(*v.add(6), *v.add(14));
        *u.add(7) = _mm256_add_epi32(*v.add(7), *v.add(15));
        *u.add(8) = _mm256_sub_epi32(*v.add(0), *v.add(8));
        *u.add(9) = _mm256_sub_epi32(*v.add(1), *v.add(9));
        *u.add(10) = _mm256_sub_epi32(*v.add(2), *v.add(10));
        *u.add(11) = _mm256_sub_epi32(*v.add(3), *v.add(11));
        *u.add(12) = _mm256_sub_epi32(*v.add(4), *v.add(12));
        *u.add(13) = _mm256_sub_epi32(*v.add(5), *v.add(13));
        *u.add(14) = _mm256_sub_epi32(*v.add(6), *v.add(14));
        *u.add(15) = _mm256_sub_epi32(*v.add(7), *v.add(15));

        // stage 8
        *v.add(0) = av1_half_btf_avx2(&cospi2, u.add(0), &cospi62, u.add(1), &rnding, bit as i32);
        *v.add(1) = av1_half_btf_avx2(&cospi62, u.add(0), &cospim2, u.add(1), &rnding, bit as i32);
        *v.add(2) = av1_half_btf_avx2(&cospi10, u.add(2), &cospi54, u.add(3), &rnding, bit as i32);
        *v.add(3) = av1_half_btf_avx2(&cospi54, u.add(2), &cospim10, u.add(3), &rnding, bit as i32);
        *v.add(4) = av1_half_btf_avx2(&cospi18, u.add(4), &cospi46, u.add(5), &rnding, bit as i32);
        *v.add(5) = av1_half_btf_avx2(&cospi46, u.add(4), &cospim18, u.add(5), &rnding, bit as i32);
        *v.add(6) = av1_half_btf_avx2(&cospi26, u.add(6), &cospi38, u.add(7), &rnding, bit as i32);
        *v.add(7) = av1_half_btf_avx2(&cospi38, u.add(6), &cospim26, u.add(7), &rnding, bit as i32);
        *v.add(8) = av1_half_btf_avx2(&cospi34, u.add(8), &cospi30, u.add(9), &rnding, bit as i32);
        *v.add(9) = av1_half_btf_avx2(&cospi30, u.add(8), &cospim34, u.add(9), &rnding, bit as i32);
        *v.add(10) = av1_half_btf_avx2(&cospi42, u.add(10), &cospi22, u.add(11), &rnding,
                                       bit as i32);
        *v.add(11) = av1_half_btf_avx2(&cospi22, u.add(10), &cospim42, u.add(11), &rnding,
                                       bit as i32);
        *v.add(12) = av1_half_btf_avx2(&cospi50, u.add(12), &cospi14, u.add(13), &rnding,
                                       bit as i32);
        *v.add(13) = av1_half_btf_avx2(&cospi14, u.add(12), &cospim50, u.add(13), &rnding,
                                       bit as i32);
        *v.add(14) = av1_half_btf_avx2(&cospi58, u.add(14), &cospi6, u.add(15), &rnding,
                                       bit as i32);
        *v.add(15) = av1_half_btf_avx2(&cospi6, u.add(14), &cospim58, u.add(15), &rnding,
                                       bit as i32);

        // stage 9
        *out.offset((0 * outstride + col) as isize) = *v.add(1);
        *out.offset((1 * outstride + col) as isize) = *v.add(14);
        *out.offset((2 * outstride + col) as isize) = *v.add(3);
        *out.offset((3 * outstride + col) as isize) = *v.add(12);
        *out.offset((4 * outstride + col) as isize) = *v.add(5);
        *out.offset((5 * outstride + col) as isize) = *v.add(10);
        *out.offset((6 * outstride + col) as isize) = *v.add(7);
        *out.offset((7 * outstride + col) as isize) = *v.add(8);
        *out.offset((8 * outstride + col) as isize) = *v.add(9);
        *out.offset((9 * outstride + col) as isize) = *v.add(6);
        *out.offset((10 * outstride + col) as isize) = *v.add(11);
        *out.offset((11 * outstride + col) as isize) = *v.add(4);
        *out.offset((12 * outstride + col) as isize) = *v.add(13);
        *out.offset((13 * outstride + col) as isize) = *v.add(2);
        *out.offset((14 * outstride + col) as isize) = *v.add(15);
        *out.offset((15 * outstride + col) as isize) = *v.add(0);
        col += 1;
    }
}
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn idtx16_avx2(in_: *mut __m256i, out: *mut __m256i, bit: i8, col_num: i32,
                      outstride: i32) {
    let _ = bit;
    let _ = outstride;
    let fact: __m256i = _mm256_set1_epi32(2 * NewSqrt2);
    let offset: __m256i = _mm256_set1_epi32(1 << (NewSqrt2Bits - 1));
    let mut a_low: __m256i;

    let num_iters: i32 = 16 * col_num;
    let mut i: i32 = 0;
    while i < num_iters {
        a_low = _mm256_mullo_epi32(*in_.offset(i as isize), fact);
        a_low = _mm256_add_epi32(a_low, offset);
        *out.offset(i as isize) = _mm256_srai_epi32::<NewSqrt2Bits>(a_low);
        i += 1;
    }
}
static col_highbd_txfm8x16_arr: [transform_1d_avx2; TX_TYPES] = [
    Some(fdct16_avx2),  // DCT_DCT
    Some(fadst16_avx2), // ADST_DCT
    Some(fdct16_avx2),  // DCT_ADST
    Some(fadst16_avx2), // ADST_ADST
    Some(fadst16_avx2), // FLIPADST_DCT
    Some(fdct16_avx2),  // DCT_FLIPADST
    Some(fadst16_avx2), // FLIPADST_FLIPADST
    Some(fadst16_avx2), // ADST_FLIPADST
    Some(fadst16_avx2), // FLIPADST_ADST
    Some(idtx16_avx2),  // IDTX
    Some(fdct16_avx2),  // V_DCT
    Some(idtx16_avx2),  // H_DCT
    Some(fadst16_avx2), // V_ADST
    Some(idtx16_avx2),  // H_ADST
    Some(fadst16_avx2), // V_FLIPADST
    Some(idtx16_avx2),  // H_FLIPADST
];
static row_highbd_txfm8x8_arr: [transform_1d_avx2; TX_TYPES] = [
    Some(fdct8_avx2),  // DCT_DCT
    Some(fdct8_avx2),  // ADST_DCT
    Some(fadst8_avx2), // DCT_ADST
    Some(fadst8_avx2), // ADST_ADST
    Some(fdct8_avx2),  // FLIPADST_DCT
    Some(fadst8_avx2), // DCT_FLIPADST
    Some(fadst8_avx2), // FLIPADST_FLIPADST
    Some(fadst8_avx2), // ADST_FLIPADST
    Some(fadst8_avx2), // FLIPADST_ADST
    Some(idtx8_avx2),  // IDTX
    Some(idtx8_avx2),  // V_DCT
    Some(fdct8_avx2),  // H_DCT
    Some(idtx8_avx2),  // V_ADST
    Some(fadst8_avx2), // H_ADST
    Some(idtx8_avx2),  // V_FLIPADST
    Some(fadst8_avx2), // H_FLIPADST
];
#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn av1_fwd_txfm2d_8x16_avx2(input: *const i16, coeff: *mut i32,
                                                         stride: i32, tx_type: TX_TYPE,
                                                         bd: i32) {
    let mut in__: [__m256i; 16] = core::mem::zeroed();
    let mut out_: [__m256i; 16] = core::mem::zeroed();
    let in_ = in__.as_mut_ptr();
    let out = out_.as_mut_ptr();
    let shift: *const i8 = av1_fwd_txfm_shift_ls[TX_8X16 as usize];
    let txw_idx: i32 = get_txw_idx(TX_8X16);
    let txh_idx: i32 = get_txh_idx(TX_8X16);
    let col_txfm = col_highbd_txfm8x16_arr[tx_type as usize].unwrap();
    let row_txfm = row_highbd_txfm8x8_arr[tx_type as usize].unwrap();
    let bit: i8 = av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize];
    let mut ud_flip: i32 = 0;
    let mut lr_flip: i32 = 0;
    get_flip_cfg(tx_type, &mut ud_flip, &mut lr_flip);

    load_buffer_8x16_avx2(input, in_, stride, ud_flip, lr_flip, *shift.add(0) as i32);
    col_txfm(in_, out, bit, 1, 1);
    col_txfm_8x8_rounding(out, -(*shift.add(1) as i32));
    col_txfm_8x8_rounding(out.add(8), -(*shift.add(1) as i32));
    fwd_txfm_transpose_8x8_avx2(out, in_, 1, 2);
    fwd_txfm_transpose_8x8_avx2(out.add(8), in_.add(1), 1, 2);
    row_txfm(in_, out, bit, 2, 2);
    round_shift_rect_array_32_avx2(out, in_, 16, -(*shift.add(2) as i32), NewSqrt2);
    store_buffer_avx2(in_, coeff, 8, 16);
    let _ = bd;
}
static col_highbd_txfm8x8_arr: [transform_1d_avx2; TX_TYPES] = [
    Some(fdct8_avx2),  // DCT_DCT
    Some(fadst8_avx2), // ADST_DCT
    Some(fdct8_avx2),  // DCT_ADST
    Some(fadst8_avx2), // ADST_ADST
    Some(fadst8_avx2), // FLIPADST_DCT
    Some(fdct8_avx2),  // DCT_FLIPADST
    Some(fadst8_avx2), // FLIPADST_FLIPADST
    Some(fadst8_avx2), // ADST_FLIPADST
    Some(fadst8_avx2), // FLIPADST_ADST
    Some(idtx8_avx2),  // IDTX
    Some(fdct8_avx2),  // V_DCT
    Some(idtx8_avx2),  // H_DCT
    Some(fadst8_avx2), // V_ADST
    Some(idtx8_avx2),  // H_ADST
    Some(fadst8_avx2), // V_FLIPADST
    Some(idtx8_avx2),  // H_FLIPADST
];
static row_highbd_txfm8x16_arr: [transform_1d_avx2; TX_TYPES] = [
    Some(fdct16_avx2),  // DCT_DCT
    Some(fdct16_avx2),  // ADST_DCT
    Some(fadst16_avx2), // DCT_ADST
    Some(fadst16_avx2), // ADST_ADST
    Some(fdct16_avx2),  // FLIPADST_DCT
    Some(fadst16_avx2), // DCT_FLIPADST
    Some(fadst16_avx2), // FLIPADST_FLIPADST
    Some(fadst16_avx2), // ADST_FLIPADST
    Some(fadst16_avx2), // FLIPADST_ADST
    Some(idtx16_avx2),  // IDTX
    Some(idtx16_avx2),  // V_DCT
    Some(fdct16_avx2),  // H_DCT
    Some(idtx16_avx2),  // V_ADST
    Some(fadst16_avx2), // H_ADST
    Some(idtx16_avx2),  // V_FLIPADST
    Some(fadst16_avx2), // H_FLIPADST
];
#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn av1_fwd_txfm2d_16x8_avx2(input: *const i16, coeff: *mut i32,
                                                         stride: i32, tx_type: TX_TYPE,
                                                         bd: i32) {
    let mut in__: [__m256i; 16] = core::mem::zeroed();
    let mut out_: [__m256i; 16] = core::mem::zeroed();
    let in_ = in__.as_mut_ptr();
    let out = out_.as_mut_ptr();
    let shift: *const i8 = av1_fwd_txfm_shift_ls[TX_16X8 as usize];
    let txw_idx: i32 = get_txw_idx(TX_16X8);
    let txh_idx: i32 = get_txh_idx(TX_16X8);
    let col_txfm = col_highbd_txfm8x8_arr[tx_type as usize].unwrap();
    let row_txfm = row_highbd_txfm8x16_arr[tx_type as usize].unwrap();
    let bit: i8 = av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize];
    let mut ud_flip: i32 = 0;
    let mut lr_flip: i32 = 0;
    get_flip_cfg(tx_type, &mut ud_flip, &mut lr_flip);

    load_buffer_16xn_avx2(input, in_, stride, 8, 2, ud_flip, lr_flip);
    round_shift_32_8xn_avx2(in_, 16, *shift.add(0) as i32, 1);
    col_txfm(in_, out, bit, 2, 2);
    round_shift_32_8xn_avx2(out, 16, *shift.add(1) as i32, 1);
    fwd_txfm_transpose_8x8_avx2(out, in_, 2, 1);
    fwd_txfm_transpose_8x8_avx2(out.add(1), in_.add(8), 2, 1);
    row_txfm(in_, out, bit, 1, 1);
    round_shift_rect_array_32_avx2(out, out, 16, -(*shift.add(2) as i32), NewSqrt2);
    store_buffer_avx2(out, coeff, 8, 16);
    let _ = bd;
}
#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn av1_fwd_txfm2d_16x16_avx2(input: *const i16, coeff: *mut i32,
                                                          stride: i32, tx_type: TX_TYPE,
                                                          bd: i32) {
    let mut in__: [__m256i; 32] = core::mem::zeroed();
    let mut out_: [__m256i; 32] = core::mem::zeroed();
    let in_ = in__.as_mut_ptr();
    let out = out_.as_mut_ptr();
    let tx_size: TX_SIZE = TX_16X16;
    let shift: *const i8 = av1_fwd_txfm_shift_ls[tx_size as usize];
    let txw_idx: i32 = get_txw_idx(tx_size);
    let txh_idx: i32 = get_txh_idx(tx_size);
    let width: i32 = tx_size_wide[tx_size as usize];
    let height: i32 = tx_size_high[tx_size as usize];
    let width_div8: i32 = width >> 3;
    let width_div16: i32 = width >> 4;
    let size: i32 = height << 1;
    match tx_type {
        DCT_DCT => {
            load_buffer_16xn_avx2(input, in_, stride, height, width_div8, 0, 0);
            round_shift_32_8xn_avx2(in_, size, *shift.add(0) as i32, width_div16);
            fdct16_avx2(in_, out, av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize],
                        width_div8, width_div8);
            round_shift_32_8xn_avx2(out, size, *shift.add(1) as i32, width_div16);
            fwd_txfm_transpose_16x16_avx2(out, in_);
            fdct16_avx2(in_, out, av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize],
                        width_div8, width_div8);
            store_buffer_avx2(out, coeff, 8, 32);
        }
        ADST_DCT => {
            load_buffer_16xn_avx2(input, in_, stride, height, width_div8, 0, 0);
            round_shift_32_8xn_avx2(in_, size, *shift.add(0) as i32, width_div16);
            fadst16_avx2(in_, out, av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize],
                         width_div8, width_div8);
            round_shift_32_8xn_avx2(out, size, *shift.add(1) as i32, width_div16);
            fwd_txfm_transpose_16x16_avx2(out, in_);
            fdct16_avx2(in_, out, av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize],
                        width_div8, width_div8);
            store_buffer_avx2(out, coeff, 8, 32);
        }
        DCT_ADST => {
            load_buffer_16xn_avx2(input, in_, stride, height, width_div8, 0, 0);
            round_shift_32_8xn_avx2(in_, size, *shift.add(0) as i32, width_div16);
            fdct16_avx2(in_, out, av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize],
                        width_div8, width_div8);
            round_shift_32_8xn_avx2(out, size, *shift.add(1) as i32, width_div16);
            fwd_txfm_transpose_16x16_avx2(out, in_);
            fadst16_avx2(in_, out, av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize],
                         width_div8, width_div8);
            store_buffer_avx2(out, coeff, 8, 32);
        }
        ADST_ADST => {
            load_buffer_16xn_avx2(input, in_, stride, height, width_div8, 0, 0);
            round_shift_32_8xn_avx2(in_, size, *shift.add(0) as i32, width_div16);
            fadst16_avx2(in_, out, av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize],
                         width_div8, width_div8);
            round_shift_32_8xn_avx2(out, size, *shift.add(1) as i32, width_div16);
            fwd_txfm_transpose_16x16_avx2(out, in_);
            fadst16_avx2(in_, out, av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize],
                         width_div8, width_div8);
            store_buffer_avx2(out, coeff, 8, 32);
        }
        FLIPADST_DCT => {
            load_buffer_16xn_avx2(input, in_, stride, height, width_div8, 1, 0);
            round_shift_32_8xn_avx2(in_, size, *shift.add(0) as i32, width_div16);
            fadst16_avx2(in_, out, av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize],
                         width_div8, width_div8);
            round_shift_32_8xn_avx2(out, size, *shift.add(1) as i32, width_div16);
            fwd_txfm_transpose_16x16_avx2(out, in_);
            fdct16_avx2(in_, out, av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize],
                        width_div8, width_div8);
            store_buffer_avx2(out, coeff, 8, 32);
        }
        DCT_FLIPADST => {
            load_buffer_16xn_avx2(input, in_, stride, height, width_div8, 0, 1);
            round_shift_32_8xn_avx2(in_, size, *shift.add(0) as i32, width_div16);
            fdct16_avx2(in_, out, av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize],
                        width_div8, width_div8);
            round_shift_32_8xn_avx2(out, size, *shift.add(1) as i32, width_div16);
            fwd_txfm_transpose_16x16_avx2(out, in_);
            fadst16_avx2(in_, out, av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize],
                         width_div8, width_div8);
            store_buffer_avx2(out, coeff, 8, 32);
        }
        FLIPADST_FLIPADST => {
            load_buffer_16xn_avx2(input, in_, stride, height, width_div8, 1, 1);
            round_shift_32_8xn_avx2(in_, size, *shift.add(0) as i32, width_div16);
            fadst16_avx2(in_, out, av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize],
                         width_div8, width_div8);
            round_shift_32_8xn_avx2(out, size, *shift.add(1) as i32, width_div16);
            fwd_txfm_transpose_16x16_avx2(out, in_);
            fadst16_avx2(in_, out, av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize],
                         width_div8, width_div8);
            store_buffer_avx2(out, coeff, 8, 32);
        }
        ADST_FLIPADST => {
            load_buffer_16xn_avx2(input, in_, stride, height, width_div8, 0, 1);
            round_shift_32_8xn_avx2(in_, size, *shift.add(0) as i32, width_div16);
            fadst16_avx2(in_, out, av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize],
                         width_div8, width_div8);
            round_shift_32_8xn_avx2(out, size, *shift.add(1) as i32, width_div16);
            fwd_txfm_transpose_16x16_avx2(out, in_);
            fadst16_avx2(in_, out, av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize],
                         width_div8, width_div8);
            store_buffer_avx2(out, coeff, 8, 32);
        }
        FLIPADST_ADST => {
            load_buffer_16xn_avx2(input, in_, stride, height, width_div8, 1, 0);
            round_shift_32_8xn_avx2(in_, size, *shift.add(0) as i32, width_div16);
            fadst16_avx2(in_, out, av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize],
                         width_div8, width_div8);
            round_shift_32_8xn_avx2(out, size, *shift.add(1) as i32, width_div16);
            fwd_txfm_transpose_16x16_avx2(out, in_);
            fadst16_avx2(in_, out, av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize],
                         width_div8, width_div8);
            store_buffer_avx2(out, coeff, 8, 32);
        }
        IDTX => {
            load_buffer_16xn_avx2(input, in_, stride, height, width_div8, 0, 0);
            round_shift_32_8xn_avx2(in_, size, *shift.add(0) as i32, width_div16);
            idtx16_avx2(in_, out, av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize],
                        width_div8, width_div8);
            round_shift_32_8xn_avx2(out, size, *shift.add(1) as i32, width_div16);
            fwd_txfm_transpose_16x16_avx2(out, in_);
            idtx16_avx2(in_, out, av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize],
                        width_div8, width_div8);
            store_buffer_avx2(out, coeff, 8, 32);
        }
        V_DCT => {
            load_buffer_16xn_avx2(input, in_, stride, height, width_div8, 0, 0);
            round_shift_32_8xn_avx2(in_, size, *shift.add(0) as i32, width_div16);
            fdct16_avx2(in_, out, av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize],
                        width_div8, width_div8);
            round_shift_32_8xn_avx2(out, size, *shift.add(1) as i32, width_div16);
            fwd_txfm_transpose_16x16_avx2(out, in_);
            idtx16_avx2(in_, out, av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize],
                        width_div8, width_div8);
            store_buffer_avx2(out, coeff, 8, 32);
        }
        H_DCT => {
            load_buffer_16xn_avx2(input, in_, stride, height, width_div8, 0, 0);
            round_shift_32_8xn_avx2(in_, size, *shift.add(0) as i32, width_div16);
            idtx16_avx2(in_, out, av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize],
                        width_div8, width_div8);
            round_shift_32_8xn_avx2(out, size, *shift.add(1) as i32, width_div16);
            fwd_txfm_transpose_16x16_avx2(out, in_);
            fdct16_avx2(in_, out, av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize],
                        width_div8, width_div8);
            store_buffer_avx2(out, coeff, 8, 32);
        }
        V_ADST => {
            load_buffer_16xn_avx2(input, in_, stride, height, width_div8, 0, 0);
            round_shift_32_8xn_avx2(in_, size, *shift.add(0) as i32, width_div16);
            fadst16_avx2(in_, out, av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize],
                         width_div8, width_div8);
            round_shift_32_8xn_avx2(out, size, *shift.add(1) as i32, width_div16);
            fwd_txfm_transpose_16x16_avx2(out, in_);
            idtx16_avx2(in_, out, av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize],
                        width_div8, width_div8);
            store_buffer_avx2(out, coeff, 8, 32);
        }
        H_ADST => {
            load_buffer_16xn_avx2(input, in_, stride, height, width_div8, 0, 0);
            round_shift_32_8xn_avx2(in_, size, *shift.add(0) as i32, width_div16);
            idtx16_avx2(in_, out, av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize],
                        width_div8, width_div8);
            round_shift_32_8xn_avx2(out, size, *shift.add(1) as i32, width_div16);
            fwd_txfm_transpose_16x16_avx2(out, in_);
            fadst16_avx2(in_, out, av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize],
                         width_div8, width_div8);
            store_buffer_avx2(out, coeff, 8, 32);
        }
        V_FLIPADST => {
            load_buffer_16xn_avx2(input, in_, stride, height, width_div8, 1, 0);
            round_shift_32_8xn_avx2(in_, size, *shift.add(0) as i32, width_div16);
            fadst16_avx2(in_, out, av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize],
                         width_div8, width_div8);
            round_shift_32_8xn_avx2(out, size, *shift.add(1) as i32, width_div16);
            fwd_txfm_transpose_16x16_avx2(out, in_);
            idtx16_avx2(in_, out, av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize],
                        width_div8, width_div8);
            store_buffer_avx2(out, coeff, 8, 32);
        }
        H_FLIPADST => {
            load_buffer_16xn_avx2(input, in_, stride, height, width_div8, 0, 1);
            round_shift_32_8xn_avx2(in_, size, *shift.add(0) as i32, width_div16);
            idtx16_avx2(in_, out, av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize],
                        width_div8, width_div8);
            round_shift_32_8xn_avx2(out, size, *shift.add(1) as i32, width_div16);
            fwd_txfm_transpose_16x16_avx2(out, in_);
            fadst16_avx2(in_, out, av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize],
                         width_div8, width_div8);
            store_buffer_avx2(out, coeff, 8, 32);
        }
        _ => debug_assert!(false),
    }
    let _ = bd;
}
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn fdct32_avx2(input: *mut __m256i, output: *mut __m256i, cos_bit: i8, instride: i32,
                      outstride: i32) {
    let mut buf0_: [__m256i; 32] = core::mem::zeroed();
    let mut buf1_: [__m256i; 32] = core::mem::zeroed();
    let buf0 = buf0_.as_mut_ptr();
    let buf1 = buf1_.as_mut_ptr();
    let mut cospi: *const i32;
    let mut startidx: i32 = 0 * instride;
    let mut endidx: i32 = 31 * instride;
    // stage 0
    // stage 1
    *buf1.add(0) = _mm256_add_epi32(*input.offset(startidx as isize),
                                    *input.offset(endidx as isize));
    *buf1.add(31) = _mm256_sub_epi32(*input.offset(startidx as isize),
                                     *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *buf1.add(1) = _mm256_add_epi32(*input.offset(startidx as isize),
                                    *input.offset(endidx as isize));
    *buf1.add(30) = _mm256_sub_epi32(*input.offset(startidx as isize),
                                     *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *buf1.add(2) = _mm256_add_epi32(*input.offset(startidx as isize),
                                    *input.offset(endidx as isize));
    *buf1.add(29) = _mm256_sub_epi32(*input.offset(startidx as isize),
                                     *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *buf1.add(3) = _mm256_add_epi32(*input.offset(startidx as isize),
                                    *input.offset(endidx as isize));
    *buf1.add(28) = _mm256_sub_epi32(*input.offset(startidx as isize),
                                     *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *buf1.add(4) = _mm256_add_epi32(*input.offset(startidx as isize),
                                    *input.offset(endidx as isize));
    *buf1.add(27) = _mm256_sub_epi32(*input.offset(startidx as isize),
                                     *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *buf1.add(5) = _mm256_add_epi32(*input.offset(startidx as isize),
                                    *input.offset(endidx as isize));
    *buf1.add(26) = _mm256_sub_epi32(*input.offset(startidx as isize),
                                     *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *buf1.add(6) = _mm256_add_epi32(*input.offset(startidx as isize),
                                    *input.offset(endidx as isize));
    *buf1.add(25) = _mm256_sub_epi32(*input.offset(startidx as isize),
                                     *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *buf1.add(7) = _mm256_add_epi32(*input.offset(startidx as isize),
                                    *input.offset(endidx as isize));
    *buf1.add(24) = _mm256_sub_epi32(*input.offset(startidx as isize),
                                     *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *buf1.add(8) = _mm256_add_epi32(*input.offset(startidx as isize),
                                    *input.offset(endidx as isize));
    *buf1.add(23) = _mm256_sub_epi32(*input.offset(startidx as isize),
                                     *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *buf1.add(9) = _mm256_add_epi32(*input.offset(startidx as isize),
                                    *input.offset(endidx as isize));
    *buf1.add(22) = _mm256_sub_epi32(*input.offset(startidx as isize),
                                     *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *buf1.add(10) = _mm256_add_epi32(*input.offset(startidx as isize),
                                     *input.offset(endidx as isize));
    *buf1.add(21) = _mm256_sub_epi32(*input.offset(startidx as isize),
                                     *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *buf1.add(11) = _mm256_add_epi32(*input.offset(startidx as isize),
                                     *input.offset(endidx as isize));
    *buf1.add(20) = _mm256_sub_epi32(*input.offset(startidx as isize),
                                     *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *buf1.add(12) = _mm256_add_epi32(*input.offset(startidx as isize),
                                     *input.offset(endidx as isize));
    *buf1.add(19) = _mm256_sub_epi32(*input.offset(startidx as isize),
                                     *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *buf1.add(13) = _mm256_add_epi32(*input.offset(startidx as isize),
                                     *input.offset(endidx as isize));
    *buf1.add(18) = _mm256_sub_epi32(*input.offset(startidx as isize),
                                     *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *buf1.add(14) = _mm256_add_epi32(*input.offset(startidx as isize),
                                     *input.offset(endidx as isize));
    *buf1.add(17) = _mm256_sub_epi32(*input.offset(startidx as isize),
                                     *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *buf1.add(15) = _mm256_add_epi32(*input.offset(startidx as isize),
                                     *input.offset(endidx as isize));
    *buf1.add(16) = _mm256_sub_epi32(*input.offset(startidx as isize),
                                     *input.offset(endidx as isize));

    // stage 2
    cospi = cospi_arr(cos_bit as i32);
    *buf0.add(0) = _mm256_add_epi32(*buf1.add(0), *buf1.add(15));
    *buf0.add(15) = _mm256_sub_epi32(*buf1.add(0), *buf1.add(15));
    *buf0.add(1) = _mm256_add_epi32(*buf1.add(1), *buf1.add(14));
    *buf0.add(14) = _mm256_sub_epi32(*buf1.add(1), *buf1.add(14));
    *buf0.add(2) = _mm256_add_epi32(*buf1.add(2), *buf1.add(13));
    *buf0.add(13) = _mm256_sub_epi32(*buf1.add(2), *buf1.add(13));
    *buf0.add(3) = _mm256_add_epi32(*buf1.add(3), *buf1.add(12));
    *buf0.add(12) = _mm256_sub_epi32(*buf1.add(3), *buf1.add(12));
    *buf0.add(4) = _mm256_add_epi32(*buf1.add(4), *buf1.add(11));
    *buf0.add(11) = _mm256_sub_epi32(*buf1.add(4), *buf1.add(11));
    *buf0.add(5) = _mm256_add_epi32(*buf1.add(5), *buf1.add(10));
    *buf0.add(10) = _mm256_sub_epi32(*buf1.add(5), *buf1.add(10));
    *buf0.add(6) = _mm256_add_epi32(*buf1.add(6), *buf1.add(9));
    *buf0.add(9) = _mm256_sub_epi32(*buf1.add(6), *buf1.add(9));
    *buf0.add(7) = _mm256_add_epi32(*buf1.add(7), *buf1.add(8));
    *buf0.add(8) = _mm256_sub_epi32(*buf1.add(7), *buf1.add(8));
    *buf0.add(16) = *buf1.add(16);
    *buf0.add(17) = *buf1.add(17);
    *buf0.add(18) = *buf1.add(18);
    *buf0.add(19) = *buf1.add(19);
    btf_32_avx2_type0!(-*cospi.add(32), *cospi.add(32), *buf1.add(20), *buf1.add(27), *buf0.add(20),
                       *buf0.add(27), cos_bit);
    btf_32_avx2_type0!(-*cospi.add(32), *cospi.add(32), *buf1.add(21), *buf1.add(26), *buf0.add(21),
                       *buf0.add(26), cos_bit);
    btf_32_avx2_type0!(-*cospi.add(32), *cospi.add(32), *buf1.add(22), *buf1.add(25), *buf0.add(22),
                       *buf0.add(25), cos_bit);
    btf_32_avx2_type0!(-*cospi.add(32), *cospi.add(32), *buf1.add(23), *buf1.add(24), *buf0.add(23),
                       *buf0.add(24), cos_bit);
    *buf0.add(28) = *buf1.add(28);
    *buf0.add(29) = *buf1.add(29);
    *buf0.add(30) = *buf1.add(30);
    *buf0.add(31) = *buf1.add(31);

    // stage 3
    cospi = cospi_arr(cos_bit as i32);
    *buf1.add(0) = _mm256_add_epi32(*buf0.add(0), *buf0.add(7));
    *buf1.add(7) = _mm256_sub_epi32(*buf0.add(0), *buf0.add(7));
    *buf1.add(1) = _mm256_add_epi32(*buf0.add(1), *buf0.add(6));
    *buf1.add(6) = _mm256_sub_epi32(*buf0.add(1), *buf0.add(6));
    *buf1.add(2) = _mm256_add_epi32(*buf0.add(2), *buf0.add(5));
    *buf1.add(5) = _mm256_sub_epi32(*buf0.add(2), *buf0.add(5));
    *buf1.add(3) = _mm256_add_epi32(*buf0.add(3), *buf0.add(4));
    *buf1.add(4) = _mm256_sub_epi32(*buf0.add(3), *buf0.add(4));
    *buf1.add(8) = *buf0.add(8);
    *buf1.add(9) = *buf0.add(9);
    btf_32_avx2_type0!(-*cospi.add(32), *cospi.add(32), *buf0.add(10), *buf0.add(13), *buf1.add(10),
                       *buf1.add(13), cos_bit);
    btf_32_avx2_type0!(-*cospi.add(32), *cospi.add(32), *buf0.add(11), *buf0.add(12), *buf1.add(11),
                       *buf1.add(12), cos_bit);
    *buf1.add(14) = *buf0.add(14);
    *buf1.add(15) = *buf0.add(15);
    *buf1.add(16) = _mm256_add_epi32(*buf0.add(16), *buf0.add(23));
    *buf1.add(23) = _mm256_sub_epi32(*buf0.add(16), *buf0.add(23));
    *buf1.add(17) = _mm256_add_epi32(*buf0.add(17), *buf0.add(22));
    *buf1.add(22) = _mm256_sub_epi32(*buf0.add(17), *buf0.add(22));
    *buf1.add(18) = _mm256_add_epi32(*buf0.add(18), *buf0.add(21));
    *buf1.add(21) = _mm256_sub_epi32(*buf0.add(18), *buf0.add(21));
    *buf1.add(19) = _mm256_add_epi32(*buf0.add(19), *buf0.add(20));
    *buf1.add(20) = _mm256_sub_epi32(*buf0.add(19), *buf0.add(20));
    *buf1.add(24) = _mm256_sub_epi32(*buf0.add(31), *buf0.add(24));
    *buf1.add(31) = _mm256_add_epi32(*buf0.add(31), *buf0.add(24));
    *buf1.add(25) = _mm256_sub_epi32(*buf0.add(30), *buf0.add(25));
    *buf1.add(30) = _mm256_add_epi32(*buf0.add(30), *buf0.add(25));
    *buf1.add(26) = _mm256_sub_epi32(*buf0.add(29), *buf0.add(26));
    *buf1.add(29) = _mm256_add_epi32(*buf0.add(29), *buf0.add(26));
    *buf1.add(27) = _mm256_sub_epi32(*buf0.add(28), *buf0.add(27));
    *buf1.add(28) = _mm256_add_epi32(*buf0.add(28), *buf0.add(27));

    // stage 4
    cospi = cospi_arr(cos_bit as i32);
    *buf0.add(0) = _mm256_add_epi32(*buf1.add(0), *buf1.add(3));
    *buf0.add(3) = _mm256_sub_epi32(*buf1.add(0), *buf1.add(3));
    *buf0.add(1) = _mm256_add_epi32(*buf1.add(1), *buf1.add(2));
    *buf0.add(2) = _mm256_sub_epi32(*buf1.add(1), *buf1.add(2));
    *buf0.add(4) = *buf1.add(4);
    btf_32_avx2_type0!(-*cospi.add(32), *cospi.add(32), *buf1.add(5), *buf1.add(6), *buf0.add(5),
                       *buf0.add(6), cos_bit);
    *buf0.add(7) = *buf1.add(7);
    *buf0.add(8) = _mm256_add_epi32(*buf1.add(8), *buf1.add(11));
    *buf0.add(11) = _mm256_sub_epi32(*buf1.add(8), *buf1.add(11));
    *buf0.add(9) = _mm256_add_epi32(*buf1.add(9), *buf1.add(10));
    *buf0.add(10) = _mm256_sub_epi32(*buf1.add(9), *buf1.add(10));
    *buf0.add(12) = _mm256_sub_epi32(*buf1.add(15), *buf1.add(12));
    *buf0.add(15) = _mm256_add_epi32(*buf1.add(15), *buf1.add(12));
    *buf0.add(13) = _mm256_sub_epi32(*buf1.add(14), *buf1.add(13));
    *buf0.add(14) = _mm256_add_epi32(*buf1.add(14), *buf1.add(13));
    *buf0.add(16) = *buf1.add(16);
    *buf0.add(17) = *buf1.add(17);
    btf_32_avx2_type0!(-*cospi.add(16), *cospi.add(48), *buf1.add(18), *buf1.add(29), *buf0.add(18),
                       *buf0.add(29), cos_bit);
    btf_32_avx2_type0!(-*cospi.add(16), *cospi.add(48), *buf1.add(19), *buf1.add(28), *buf0.add(19),
                       *buf0.add(28), cos_bit);
    btf_32_avx2_type0!(-*cospi.add(48), -*cospi.add(16), *buf1.add(20), *buf1.add(27),
                       *buf0.add(20), *buf0.add(27), cos_bit);
    btf_32_avx2_type0!(-*cospi.add(48), -*cospi.add(16), *buf1.add(21), *buf1.add(26),
                       *buf0.add(21), *buf0.add(26), cos_bit);
    *buf0.add(22) = *buf1.add(22);
    *buf0.add(23) = *buf1.add(23);
    *buf0.add(24) = *buf1.add(24);
    *buf0.add(25) = *buf1.add(25);
    *buf0.add(30) = *buf1.add(30);
    *buf0.add(31) = *buf1.add(31);

    // stage 5
    cospi = cospi_arr(cos_bit as i32);
    btf_32_avx2_type0!(*cospi.add(32), *cospi.add(32), *buf0.add(0), *buf0.add(1), *buf1.add(0),
                       *buf1.add(1), cos_bit);
    btf_32_avx2_type0!(*cospi.add(16), *cospi.add(48), *buf0.add(3), *buf0.add(2), *buf1.add(2),
                       *buf1.add(3), cos_bit);
    *buf1.add(4) = _mm256_add_epi32(*buf0.add(4), *buf0.add(5));
    *buf1.add(5) = _mm256_sub_epi32(*buf0.add(4), *buf0.add(5));
    *buf1.add(6) = _mm256_sub_epi32(*buf0.add(7), *buf0.add(6));
    *buf1.add(7) = _mm256_add_epi32(*buf0.add(7), *buf0.add(6));
    *buf1.add(8) = *buf0.add(8);
    btf_32_avx2_type0!(-*cospi.add(16), *cospi.add(48), *buf0.add(9), *buf0.add(14), *buf1.add(9),
                       *buf1.add(14), cos_bit);
    btf_32_avx2_type0!(-*cospi.add(48), -*cospi.add(16), *buf0.add(10), *buf0.add(13),
                       *buf1.add(10), *buf1.add(13), cos_bit);
    *buf1.add(11) = *buf0.add(11);
    *buf1.add(12) = *buf0.add(12);
    *buf1.add(15) = *buf0.add(15);
    *buf1.add(16) = _mm256_add_epi32(*buf0.add(16), *buf0.add(19));
    *buf1.add(19) = _mm256_sub_epi32(*buf0.add(16), *buf0.add(19));
    *buf1.add(17) = _mm256_add_epi32(*buf0.add(17), *buf0.add(18));
    *buf1.add(18) = _mm256_sub_epi32(*buf0.add(17), *buf0.add(18));
    *buf1.add(20) = _mm256_sub_epi32(*buf0.add(23), *buf0.add(20));
    *buf1.add(23) = _mm256_add_epi32(*buf0.add(23), *buf0.add(20));
    *buf1.add(21) = _mm256_sub_epi32(*buf0.add(22), *buf0.add(21));
    *buf1.add(22) = _mm256_add_epi32(*buf0.add(22), *buf0.add(21));
    *buf1.add(24) = _mm256_add_epi32(*buf0.add(24), *buf0.add(27));
    *buf1.add(27) = _mm256_sub_epi32(*buf0.add(24), *buf0.add(27));
    *buf1.add(25) = _mm256_add_epi32(*buf0.add(25), *buf0.add(26));
    *buf1.add(26) = _mm256_sub_epi32(*buf0.add(25), *buf0.add(26));
    *buf1.add(28) = _mm256_sub_epi32(*buf0.add(31), *buf0.add(28));
    *buf1.add(31) = _mm256_add_epi32(*buf0.add(31), *buf0.add(28));
    *buf1.add(29) = _mm256_sub_epi32(*buf0.add(30), *buf0.add(29));
    *buf1.add(30) = _mm256_add_epi32(*buf0.add(30), *buf0.add(29));

    // stage 6
    cospi = cospi_arr(cos_bit as i32);
    *buf0.add(0) = *buf1.add(0);
    *buf0.add(1) = *buf1.add(1);
    *buf0.add(2) = *buf1.add(2);
    *buf0.add(3) = *buf1.add(3);
    btf_32_avx2_type0!(*cospi.add(8), *cospi.add(56), *buf1.add(7), *buf1.add(4), *buf0.add(4),
                       *buf0.add(7), cos_bit);
    btf_32_avx2_type0!(*cospi.add(40), *cospi.add(24), *buf1.add(6), *buf1.add(5), *buf0.add(5),
                       *buf0.add(6), cos_bit);
    *buf0.add(8) = _mm256_add_epi32(*buf1.add(8), *buf1.add(9));
    *buf0.add(9) = _mm256_sub_epi32(*buf1.add(8), *buf1.add(9));
    *buf0.add(10) = _mm256_sub_epi32(*buf1.add(11), *buf1.add(10));
    *buf0.add(11) = _mm256_add_epi32(*buf1.add(11), *buf1.add(10));
    *buf0.add(12) = _mm256_add_epi32(*buf1.add(12), *buf1.add(13));
    *buf0.add(13) = _mm256_sub_epi32(*buf1.add(12), *buf1.add(13));
    *buf0.add(14) = _mm256_sub_epi32(*buf1.add(15), *buf1.add(14));
    *buf0.add(15) = _mm256_add_epi32(*buf1.add(15), *buf1.add(14));
    *buf0.add(16) = *buf1.add(16);
    btf_32_avx2_type0!(-*cospi.add(8), *cospi.add(56), *buf1.add(17), *buf1.add(30), *buf0.add(17),
                       *buf0.add(30), cos_bit);
    btf_32_avx2_type0!(-*cospi.add(56), -*cospi.add(8), *buf1.add(18), *buf1.add(29), *buf0.add(18),
                       *buf0.add(29), cos_bit);
    *buf0.add(19) = *buf1.add(19);
    *buf0.add(20) = *buf1.add(20);
    btf_32_avx2_type0!(-*cospi.add(40), *cospi.add(24), *buf1.add(21), *buf1.add(26), *buf0.add(21),
                       *buf0.add(26), cos_bit);
    btf_32_avx2_type0!(-*cospi.add(24), -*cospi.add(40), *buf1.add(22), *buf1.add(25),
                       *buf0.add(22), *buf0.add(25), cos_bit);
    *buf0.add(23) = *buf1.add(23);
    *buf0.add(24) = *buf1.add(24);
    *buf0.add(27) = *buf1.add(27);
    *buf0.add(28) = *buf1.add(28);
    *buf0.add(31) = *buf1.add(31);

    // stage 7
    cospi = cospi_arr(cos_bit as i32);
    *buf1.add(0) = *buf0.add(0);
    *buf1.add(1) = *buf0.add(1);
    *buf1.add(2) = *buf0.add(2);
    *buf1.add(3) = *buf0.add(3);
    *buf1.add(4) = *buf0.add(4);
    *buf1.add(5) = *buf0.add(5);
    *buf1.add(6) = *buf0.add(6);
    *buf1.add(7) = *buf0.add(7);
    btf_32_avx2_type0!(*cospi.add(4), *cospi.add(60), *buf0.add(15), *buf0.add(8), *buf1.add(8),
                       *buf1.add(15), cos_bit);
    btf_32_avx2_type0!(*cospi.add(36), *cospi.add(28), *buf0.add(14), *buf0.add(9), *buf1.add(9),
                       *buf1.add(14), cos_bit);
    btf_32_avx2_type0!(*cospi.add(20), *cospi.add(44), *buf0.add(13), *buf0.add(10), *buf1.add(10),
                       *buf1.add(13), cos_bit);
    btf_32_avx2_type0!(*cospi.add(52), *cospi.add(12), *buf0.add(12), *buf0.add(11), *buf1.add(11),
                       *buf1.add(12), cos_bit);
    *buf1.add(16) = _mm256_add_epi32(*buf0.add(16), *buf0.add(17));
    *buf1.add(17) = _mm256_sub_epi32(*buf0.add(16), *buf0.add(17));
    *buf1.add(18) = _mm256_sub_epi32(*buf0.add(19), *buf0.add(18));
    *buf1.add(19) = _mm256_add_epi32(*buf0.add(19), *buf0.add(18));
    *buf1.add(20) = _mm256_add_epi32(*buf0.add(20), *buf0.add(21));
    *buf1.add(21) = _mm256_sub_epi32(*buf0.add(20), *buf0.add(21));
    *buf1.add(22) = _mm256_sub_epi32(*buf0.add(23), *buf0.add(22));
    *buf1.add(23) = _mm256_add_epi32(*buf0.add(23), *buf0.add(22));
    *buf1.add(24) = _mm256_add_epi32(*buf0.add(24), *buf0.add(25));
    *buf1.add(25) = _mm256_sub_epi32(*buf0.add(24), *buf0.add(25));
    *buf1.add(26) = _mm256_sub_epi32(*buf0.add(27), *buf0.add(26));
    *buf1.add(27) = _mm256_add_epi32(*buf0.add(27), *buf0.add(26));
    *buf1.add(28) = _mm256_add_epi32(*buf0.add(28), *buf0.add(29));
    *buf1.add(29) = _mm256_sub_epi32(*buf0.add(28), *buf0.add(29));
    *buf1.add(30) = _mm256_sub_epi32(*buf0.add(31), *buf0.add(30));
    *buf1.add(31) = _mm256_add_epi32(*buf0.add(31), *buf0.add(30));

    // stage 8
    cospi = cospi_arr(cos_bit as i32);
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
    btf_32_avx2_type0!(*cospi.add(2), *cospi.add(62), *buf1.add(31), *buf1.add(16), *buf0.add(16),
                       *buf0.add(31), cos_bit);
    btf_32_avx2_type0!(*cospi.add(34), *cospi.add(30), *buf1.add(30), *buf1.add(17), *buf0.add(17),
                       *buf0.add(30), cos_bit);
    btf_32_avx2_type0!(*cospi.add(18), *cospi.add(46), *buf1.add(29), *buf1.add(18), *buf0.add(18),
                       *buf0.add(29), cos_bit);
    btf_32_avx2_type0!(*cospi.add(50), *cospi.add(14), *buf1.add(28), *buf1.add(19), *buf0.add(19),
                       *buf0.add(28), cos_bit);
    btf_32_avx2_type0!(*cospi.add(10), *cospi.add(54), *buf1.add(27), *buf1.add(20), *buf0.add(20),
                       *buf0.add(27), cos_bit);
    btf_32_avx2_type0!(*cospi.add(42), *cospi.add(22), *buf1.add(26), *buf1.add(21), *buf0.add(21),
                       *buf0.add(26), cos_bit);
    btf_32_avx2_type0!(*cospi.add(26), *cospi.add(38), *buf1.add(25), *buf1.add(22), *buf0.add(22),
                       *buf0.add(25), cos_bit);
    btf_32_avx2_type0!(*cospi.add(58), *cospi.add(6), *buf1.add(24), *buf1.add(23), *buf0.add(23),
                       *buf0.add(24), cos_bit);

    startidx = 0 * outstride;
    endidx = 31 * outstride;
    // stage 9
    *output.offset(startidx as isize) = *buf0.add(0);
    *output.offset(endidx as isize) = *buf0.add(31);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *buf0.add(16);
    *output.offset(endidx as isize) = *buf0.add(15);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *buf0.add(8);
    *output.offset(endidx as isize) = *buf0.add(23);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *buf0.add(24);
    *output.offset(endidx as isize) = *buf0.add(7);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *buf0.add(4);
    *output.offset(endidx as isize) = *buf0.add(27);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *buf0.add(20);
    *output.offset(endidx as isize) = *buf0.add(11);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *buf0.add(12);
    *output.offset(endidx as isize) = *buf0.add(19);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *buf0.add(28);
    *output.offset(endidx as isize) = *buf0.add(3);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *buf0.add(2);
    *output.offset(endidx as isize) = *buf0.add(29);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *buf0.add(18);
    *output.offset(endidx as isize) = *buf0.add(13);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *buf0.add(10);
    *output.offset(endidx as isize) = *buf0.add(21);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *buf0.add(26);
    *output.offset(endidx as isize) = *buf0.add(5);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *buf0.add(6);
    *output.offset(endidx as isize) = *buf0.add(25);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *buf0.add(22);
    *output.offset(endidx as isize) = *buf0.add(9);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *buf0.add(14);
    *output.offset(endidx as isize) = *buf0.add(17);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *buf0.add(30);
    *output.offset(endidx as isize) = *buf0.add(1);
}
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn idtx32x32_avx2(input: *mut __m256i, output: *mut __m256i, cos_bit: i8, instride: i32,
                         outstride: i32) {
    let _ = cos_bit;
    let mut i: i32 = 0;
    while i < 32 {
        *output.offset((i * outstride) as isize) =
            _mm256_slli_epi32::<2>(*input.offset((i * instride) as isize));
        *output.offset(((i + 1) * outstride) as isize) =
            _mm256_slli_epi32::<2>(*input.offset(((i + 1) * instride) as isize));
        *output.offset(((i + 2) * outstride) as isize) =
            _mm256_slli_epi32::<2>(*input.offset(((i + 2) * instride) as isize));
        *output.offset(((i + 3) * outstride) as isize) =
            _mm256_slli_epi32::<2>(*input.offset(((i + 3) * instride) as isize));
        *output.offset(((i + 4) * outstride) as isize) =
            _mm256_slli_epi32::<2>(*input.offset(((i + 4) * instride) as isize));
        *output.offset(((i + 5) * outstride) as isize) =
            _mm256_slli_epi32::<2>(*input.offset(((i + 5) * instride) as isize));
        *output.offset(((i + 6) * outstride) as isize) =
            _mm256_slli_epi32::<2>(*input.offset(((i + 6) * instride) as isize));
        *output.offset(((i + 7) * outstride) as isize) =
            _mm256_slli_epi32::<2>(*input.offset(((i + 7) * instride) as isize));
        i += 8;
    }
}
static col_txfm8x32_arr: [transform_1d_avx2; TX_TYPES] = [
    Some(fdct32_avx2),    // DCT_DCT
    None,                 // ADST_DCT
    None,                 // DCT_ADST
    None,                 // ADST_ADST
    None,                 // FLIPADST_DCT
    None,                 // DCT_FLIPADST
    None,                 // FLIPADST_FLIPADST
    None,                 // ADST_FLIPADST
    None,                 // FLIPADST_ADST
    Some(idtx32x32_avx2), // IDTX
    None,                 // V_DCT
    None,                 // H_DCT
    None,                 // V_ADST
    None,                 // H_ADST
    None,                 // V_FLIPADST
    None,                 // H_FLIPADST
];
static row_txfm8x32_arr: [transform_1d_avx2; TX_TYPES] = [
    Some(fdct32_avx2),    // DCT_DCT
    None,                 // ADST_DCT
    None,                 // DCT_ADST
    None,                 // ADST_ADST
    None,                 // FLIPADST_DCT
    None,                 // DCT_FLIPADST
    None,                 // FLIPADST_FLIPADST
    None,                 // ADST_FLIPADST
    None,                 // FLIPADST_ADST
    Some(idtx32x32_avx2), // IDTX
    None,                 // V_DCT
    None,                 // H_DCT
    None,                 // V_ADST
    None,                 // H_ADST
    None,                 // V_FLIPADST
    None,                 // H_FLIPADST
];
#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn av1_fwd_txfm2d_32x32_avx2(input: *const i16, output: *mut i32,
                                                          stride: i32, tx_type: TX_TYPE,
                                                          bd: i32) {
    let _ = bd;
    let mut buf0_: [__m256i; 128] = core::mem::zeroed();
    let mut buf1_: [__m256i; 128] = core::mem::zeroed();
    let buf0 = buf0_.as_mut_ptr();
    let buf1 = buf1_.as_mut_ptr();
    let tx_size: TX_SIZE = TX_32X32;
    let shift: *const i8 = av1_fwd_txfm_shift_ls[tx_size as usize];
    let txw_idx: i32 = get_txw_idx(tx_size);
    let txh_idx: i32 = get_txh_idx(tx_size);
    let cos_bit_col: i32 = av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32;
    let cos_bit_row: i32 = av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32;
    let width: i32 = tx_size_wide[tx_size as usize];
    let height: i32 = tx_size_high[tx_size as usize];
    let col_txfm = col_txfm8x32_arr[tx_type as usize].unwrap();
    let row_txfm = row_txfm8x32_arr[tx_type as usize].unwrap();
    let mut r: i32;
    let mut c: i32;
    let width_div16: i32 = width >> 4;
    let width_div8: i32 = width >> 3;

    let mut i: i32 = 0;
    while i < width_div16 {
        load_buffer_16xn_avx2(input.offset((i << 4) as isize), buf0.offset((i << 1) as isize),
                              stride, height, width_div8, 0, 0);
        round_shift_32_8xn_avx2(buf0.offset((i << 1) as isize), height, *shift.add(0) as i32,
                                width_div8);
        round_shift_32_8xn_avx2(buf0.offset(((i << 1) + 1) as isize), height,
                                *shift.add(0) as i32, width_div8);
        col_txfm(buf0.offset((i << 1) as isize), buf0.offset((i << 1) as isize),
                 cos_bit_col as i8, width_div8, width_div8);
        col_txfm(buf0.offset(((i << 1) + 1) as isize), buf0.offset(((i << 1) + 1) as isize),
                 cos_bit_col as i8, width_div8, width_div8);
        round_shift_32_8xn_avx2(buf0.offset((i << 1) as isize), height, *shift.add(1) as i32,
                                width_div8);
        round_shift_32_8xn_avx2(buf0.offset(((i << 1) + 1) as isize), height,
                                *shift.add(1) as i32, width_div8);
        i += 1;
    }

    r = 0;
    while r < height {
        c = 0;
        while c < width_div8 {
            fwd_txfm_transpose_8x8_avx2(buf0.offset((r * width_div8 + c) as isize),
                                        buf1.offset((c * 8 * width_div8 + (r >> 3)) as isize),
                                        width_div8, width_div8);
            c += 1;
        }
        r += 8;
    }

    let mut i: i32 = 0;
    while i < width_div16 {
        row_txfm(buf1.offset((i << 1) as isize), buf1.offset((i << 1) as isize),
                 cos_bit_row as i8, width_div8, width_div8);
        row_txfm(buf1.offset(((i << 1) + 1) as isize), buf1.offset(((i << 1) + 1) as isize),
                 cos_bit_row as i8, width_div8, width_div8);
        round_shift_32_8xn_avx2(buf1.offset((i << 1) as isize), height, *shift.add(2) as i32,
                                width_div8);
        round_shift_32_8xn_avx2(buf1.offset(((i << 1) + 1) as isize), height,
                                *shift.add(2) as i32, width_div8);
        i += 1;
    }

    store_buffer_avx2(buf1, output, 8, 128);
}
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn fdct64_stage2_avx2(x1: *mut __m256i, x2: *mut __m256i, cospi_m32: *mut __m256i,
                             cospi_p32: *mut __m256i, __rounding: *const __m256i, cos_bit: i8) {
    *x2.add(0) = _mm256_add_epi32(*x1.add(0), *x1.add(31));
    *x2.add(31) = _mm256_sub_epi32(*x1.add(0), *x1.add(31));
    *x2.add(1) = _mm256_add_epi32(*x1.add(1), *x1.add(30));
    *x2.add(30) = _mm256_sub_epi32(*x1.add(1), *x1.add(30));
    *x2.add(2) = _mm256_add_epi32(*x1.add(2), *x1.add(29));
    *x2.add(29) = _mm256_sub_epi32(*x1.add(2), *x1.add(29));
    *x2.add(3) = _mm256_add_epi32(*x1.add(3), *x1.add(28));
    *x2.add(28) = _mm256_sub_epi32(*x1.add(3), *x1.add(28));
    *x2.add(4) = _mm256_add_epi32(*x1.add(4), *x1.add(27));
    *x2.add(27) = _mm256_sub_epi32(*x1.add(4), *x1.add(27));
    *x2.add(5) = _mm256_add_epi32(*x1.add(5), *x1.add(26));
    *x2.add(26) = _mm256_sub_epi32(*x1.add(5), *x1.add(26));
    *x2.add(6) = _mm256_add_epi32(*x1.add(6), *x1.add(25));
    *x2.add(25) = _mm256_sub_epi32(*x1.add(6), *x1.add(25));
    *x2.add(7) = _mm256_add_epi32(*x1.add(7), *x1.add(24));
    *x2.add(24) = _mm256_sub_epi32(*x1.add(7), *x1.add(24));
    *x2.add(8) = _mm256_add_epi32(*x1.add(8), *x1.add(23));
    *x2.add(23) = _mm256_sub_epi32(*x1.add(8), *x1.add(23));
    *x2.add(9) = _mm256_add_epi32(*x1.add(9), *x1.add(22));
    *x2.add(22) = _mm256_sub_epi32(*x1.add(9), *x1.add(22));
    *x2.add(10) = _mm256_add_epi32(*x1.add(10), *x1.add(21));
    *x2.add(21) = _mm256_sub_epi32(*x1.add(10), *x1.add(21));
    *x2.add(11) = _mm256_add_epi32(*x1.add(11), *x1.add(20));
    *x2.add(20) = _mm256_sub_epi32(*x1.add(11), *x1.add(20));
    *x2.add(12) = _mm256_add_epi32(*x1.add(12), *x1.add(19));
    *x2.add(19) = _mm256_sub_epi32(*x1.add(12), *x1.add(19));
    *x2.add(13) = _mm256_add_epi32(*x1.add(13), *x1.add(18));
    *x2.add(18) = _mm256_sub_epi32(*x1.add(13), *x1.add(18));
    *x2.add(14) = _mm256_add_epi32(*x1.add(14), *x1.add(17));
    *x2.add(17) = _mm256_sub_epi32(*x1.add(14), *x1.add(17));
    *x2.add(15) = _mm256_add_epi32(*x1.add(15), *x1.add(16));
    *x2.add(16) = _mm256_sub_epi32(*x1.add(15), *x1.add(16));
    *x2.add(32) = *x1.add(32);
    *x2.add(33) = *x1.add(33);
    *x2.add(34) = *x1.add(34);
    *x2.add(35) = *x1.add(35);
    *x2.add(36) = *x1.add(36);
    *x2.add(37) = *x1.add(37);
    *x2.add(38) = *x1.add(38);
    *x2.add(39) = *x1.add(39);
    btf_32_type0_avx2_new!(*cospi_m32, *cospi_p32, *x1.add(40), *x1.add(55), *x2.add(40),
                           *x2.add(55), *__rounding, cos_bit);
    btf_32_type0_avx2_new!(*cospi_m32, *cospi_p32, *x1.add(41), *x1.add(54), *x2.add(41),
                           *x2.add(54), *__rounding, cos_bit);
    btf_32_type0_avx2_new!(*cospi_m32, *cospi_p32, *x1.add(42), *x1.add(53), *x2.add(42),
                           *x2.add(53), *__rounding, cos_bit);
    btf_32_type0_avx2_new!(*cospi_m32, *cospi_p32, *x1.add(43), *x1.add(52), *x2.add(43),
                           *x2.add(52), *__rounding, cos_bit);
    btf_32_type0_avx2_new!(*cospi_m32, *cospi_p32, *x1.add(44), *x1.add(51), *x2.add(44),
                           *x2.add(51), *__rounding, cos_bit);
    btf_32_type0_avx2_new!(*cospi_m32, *cospi_p32, *x1.add(45), *x1.add(50), *x2.add(45),
                           *x2.add(50), *__rounding, cos_bit);
    btf_32_type0_avx2_new!(*cospi_m32, *cospi_p32, *x1.add(46), *x1.add(49), *x2.add(46),
                           *x2.add(49), *__rounding, cos_bit);
    btf_32_type0_avx2_new!(*cospi_m32, *cospi_p32, *x1.add(47), *x1.add(48), *x2.add(47),
                           *x2.add(48), *__rounding, cos_bit);
    *x2.add(56) = *x1.add(56);
    *x2.add(57) = *x1.add(57);
    *x2.add(58) = *x1.add(58);
    *x2.add(59) = *x1.add(59);
    *x2.add(60) = *x1.add(60);
    *x2.add(61) = *x1.add(61);
    *x2.add(62) = *x1.add(62);
    *x2.add(63) = *x1.add(63);
}
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn fdct64_stage3_avx2(x2: *mut __m256i, x3: *mut __m256i, cospi_m32: *mut __m256i,
                             cospi_p32: *mut __m256i, __rounding: *const __m256i, cos_bit: i8) {
    *x3.add(0) = _mm256_add_epi32(*x2.add(0), *x2.add(15));
    *x3.add(15) = _mm256_sub_epi32(*x2.add(0), *x2.add(15));
    *x3.add(1) = _mm256_add_epi32(*x2.add(1), *x2.add(14));
    *x3.add(14) = _mm256_sub_epi32(*x2.add(1), *x2.add(14));
    *x3.add(2) = _mm256_add_epi32(*x2.add(2), *x2.add(13));
    *x3.add(13) = _mm256_sub_epi32(*x2.add(2), *x2.add(13));
    *x3.add(3) = _mm256_add_epi32(*x2.add(3), *x2.add(12));
    *x3.add(12) = _mm256_sub_epi32(*x2.add(3), *x2.add(12));
    *x3.add(4) = _mm256_add_epi32(*x2.add(4), *x2.add(11));
    *x3.add(11) = _mm256_sub_epi32(*x2.add(4), *x2.add(11));
    *x3.add(5) = _mm256_add_epi32(*x2.add(5), *x2.add(10));
    *x3.add(10) = _mm256_sub_epi32(*x2.add(5), *x2.add(10));
    *x3.add(6) = _mm256_add_epi32(*x2.add(6), *x2.add(9));
    *x3.add(9) = _mm256_sub_epi32(*x2.add(6), *x2.add(9));
    *x3.add(7) = _mm256_add_epi32(*x2.add(7), *x2.add(8));
    *x3.add(8) = _mm256_sub_epi32(*x2.add(7), *x2.add(8));
    *x3.add(16) = *x2.add(16);
    *x3.add(17) = *x2.add(17);
    *x3.add(18) = *x2.add(18);
    *x3.add(19) = *x2.add(19);
    btf_32_type0_avx2_new!(*cospi_m32, *cospi_p32, *x2.add(20), *x2.add(27), *x3.add(20),
                           *x3.add(27), *__rounding, cos_bit);
    btf_32_type0_avx2_new!(*cospi_m32, *cospi_p32, *x2.add(21), *x2.add(26), *x3.add(21),
                           *x3.add(26), *__rounding, cos_bit);
    btf_32_type0_avx2_new!(*cospi_m32, *cospi_p32, *x2.add(22), *x2.add(25), *x3.add(22),
                           *x3.add(25), *__rounding, cos_bit);
    btf_32_type0_avx2_new!(*cospi_m32, *cospi_p32, *x2.add(23), *x2.add(24), *x3.add(23),
                           *x3.add(24), *__rounding, cos_bit);
    *x3.add(28) = *x2.add(28);
    *x3.add(29) = *x2.add(29);
    *x3.add(30) = *x2.add(30);
    *x3.add(31) = *x2.add(31);
    *x3.add(32) = _mm256_add_epi32(*x2.add(32), *x2.add(47));
    *x3.add(47) = _mm256_sub_epi32(*x2.add(32), *x2.add(47));
    *x3.add(33) = _mm256_add_epi32(*x2.add(33), *x2.add(46));
    *x3.add(46) = _mm256_sub_epi32(*x2.add(33), *x2.add(46));
    *x3.add(34) = _mm256_add_epi32(*x2.add(34), *x2.add(45));
    *x3.add(45) = _mm256_sub_epi32(*x2.add(34), *x2.add(45));
    *x3.add(35) = _mm256_add_epi32(*x2.add(35), *x2.add(44));
    *x3.add(44) = _mm256_sub_epi32(*x2.add(35), *x2.add(44));
    *x3.add(36) = _mm256_add_epi32(*x2.add(36), *x2.add(43));
    *x3.add(43) = _mm256_sub_epi32(*x2.add(36), *x2.add(43));
    *x3.add(37) = _mm256_add_epi32(*x2.add(37), *x2.add(42));
    *x3.add(42) = _mm256_sub_epi32(*x2.add(37), *x2.add(42));
    *x3.add(38) = _mm256_add_epi32(*x2.add(38), *x2.add(41));
    *x3.add(41) = _mm256_sub_epi32(*x2.add(38), *x2.add(41));
    *x3.add(39) = _mm256_add_epi32(*x2.add(39), *x2.add(40));
    *x3.add(40) = _mm256_sub_epi32(*x2.add(39), *x2.add(40));
    *x3.add(48) = _mm256_sub_epi32(*x2.add(63), *x2.add(48));
    *x3.add(63) = _mm256_add_epi32(*x2.add(63), *x2.add(48));
    *x3.add(49) = _mm256_sub_epi32(*x2.add(62), *x2.add(49));
    *x3.add(62) = _mm256_add_epi32(*x2.add(62), *x2.add(49));
    *x3.add(50) = _mm256_sub_epi32(*x2.add(61), *x2.add(50));
    *x3.add(61) = _mm256_add_epi32(*x2.add(61), *x2.add(50));
    *x3.add(51) = _mm256_sub_epi32(*x2.add(60), *x2.add(51));
    *x3.add(60) = _mm256_add_epi32(*x2.add(60), *x2.add(51));
    *x3.add(52) = _mm256_sub_epi32(*x2.add(59), *x2.add(52));
    *x3.add(59) = _mm256_add_epi32(*x2.add(59), *x2.add(52));
    *x3.add(53) = _mm256_sub_epi32(*x2.add(58), *x2.add(53));
    *x3.add(58) = _mm256_add_epi32(*x2.add(58), *x2.add(53));
    *x3.add(54) = _mm256_sub_epi32(*x2.add(57), *x2.add(54));
    *x3.add(57) = _mm256_add_epi32(*x2.add(57), *x2.add(54));
    *x3.add(55) = _mm256_sub_epi32(*x2.add(56), *x2.add(55));
    *x3.add(56) = _mm256_add_epi32(*x2.add(56), *x2.add(55));
}
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn fdct64_stage4_avx2(x3: *mut __m256i, x4: *mut __m256i, cospi_m32: *mut __m256i,
                             cospi_p32: *mut __m256i, cospi_m16: *mut __m256i,
                             cospi_p48: *mut __m256i, cospi_m48: *mut __m256i,
                             __rounding: *const __m256i, cos_bit: i8) {
    *x4.add(0) = _mm256_add_epi32(*x3.add(0), *x3.add(7));
    *x4.add(7) = _mm256_sub_epi32(*x3.add(0), *x3.add(7));
    *x4.add(1) = _mm256_add_epi32(*x3.add(1), *x3.add(6));
    *x4.add(6) = _mm256_sub_epi32(*x3.add(1), *x3.add(6));
    *x4.add(2) = _mm256_add_epi32(*x3.add(2), *x3.add(5));
    *x4.add(5) = _mm256_sub_epi32(*x3.add(2), *x3.add(5));
    *x4.add(3) = _mm256_add_epi32(*x3.add(3), *x3.add(4));
    *x4.add(4) = _mm256_sub_epi32(*x3.add(3), *x3.add(4));
    *x4.add(8) = *x3.add(8);
    *x4.add(9) = *x3.add(9);
    btf_32_type0_avx2_new!(*cospi_m32, *cospi_p32, *x3.add(10), *x3.add(13), *x4.add(10),
                           *x4.add(13), *__rounding, cos_bit);
    btf_32_type0_avx2_new!(*cospi_m32, *cospi_p32, *x3.add(11), *x3.add(12), *x4.add(11),
                           *x4.add(12), *__rounding, cos_bit);
    *x4.add(14) = *x3.add(14);
    *x4.add(15) = *x3.add(15);
    *x4.add(16) = _mm256_add_epi32(*x3.add(16), *x3.add(23));
    *x4.add(23) = _mm256_sub_epi32(*x3.add(16), *x3.add(23));
    *x4.add(17) = _mm256_add_epi32(*x3.add(17), *x3.add(22));
    *x4.add(22) = _mm256_sub_epi32(*x3.add(17), *x3.add(22));
    *x4.add(18) = _mm256_add_epi32(*x3.add(18), *x3.add(21));
    *x4.add(21) = _mm256_sub_epi32(*x3.add(18), *x3.add(21));
    *x4.add(19) = _mm256_add_epi32(*x3.add(19), *x3.add(20));
    *x4.add(20) = _mm256_sub_epi32(*x3.add(19), *x3.add(20));
    *x4.add(24) = _mm256_sub_epi32(*x3.add(31), *x3.add(24));
    *x4.add(31) = _mm256_add_epi32(*x3.add(31), *x3.add(24));
    *x4.add(25) = _mm256_sub_epi32(*x3.add(30), *x3.add(25));
    *x4.add(30) = _mm256_add_epi32(*x3.add(30), *x3.add(25));
    *x4.add(26) = _mm256_sub_epi32(*x3.add(29), *x3.add(26));
    *x4.add(29) = _mm256_add_epi32(*x3.add(29), *x3.add(26));
    *x4.add(27) = _mm256_sub_epi32(*x3.add(28), *x3.add(27));
    *x4.add(28) = _mm256_add_epi32(*x3.add(28), *x3.add(27));
    *x4.add(32) = *x3.add(32);
    *x4.add(33) = *x3.add(33);
    *x4.add(34) = *x3.add(34);
    *x4.add(35) = *x3.add(35);
    btf_32_type0_avx2_new!(*cospi_m16, *cospi_p48, *x3.add(36), *x3.add(59), *x4.add(36),
                           *x4.add(59), *__rounding, cos_bit);
    btf_32_type0_avx2_new!(*cospi_m16, *cospi_p48, *x3.add(37), *x3.add(58), *x4.add(37),
                           *x4.add(58), *__rounding, cos_bit);
    btf_32_type0_avx2_new!(*cospi_m16, *cospi_p48, *x3.add(38), *x3.add(57), *x4.add(38),
                           *x4.add(57), *__rounding, cos_bit);
    btf_32_type0_avx2_new!(*cospi_m16, *cospi_p48, *x3.add(39), *x3.add(56), *x4.add(39),
                           *x4.add(56), *__rounding, cos_bit);
    btf_32_type0_avx2_new!(*cospi_m48, *cospi_m16, *x3.add(40), *x3.add(55), *x4.add(40),
                           *x4.add(55), *__rounding, cos_bit);
    btf_32_type0_avx2_new!(*cospi_m48, *cospi_m16, *x3.add(41), *x3.add(54), *x4.add(41),
                           *x4.add(54), *__rounding, cos_bit);
    btf_32_type0_avx2_new!(*cospi_m48, *cospi_m16, *x3.add(42), *x3.add(53), *x4.add(42),
                           *x4.add(53), *__rounding, cos_bit);
    btf_32_type0_avx2_new!(*cospi_m48, *cospi_m16, *x3.add(43), *x3.add(52), *x4.add(43),
                           *x4.add(52), *__rounding, cos_bit);
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
}
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn fdct64_stage5_avx2(x4: *mut __m256i, x5: *mut __m256i, cospi_m32: *mut __m256i,
                             cospi_p32: *mut __m256i, cospi_m16: *mut __m256i,
                             cospi_p48: *mut __m256i, cospi_m48: *mut __m256i,
                             __rounding: *const __m256i, cos_bit: i8) {
    *x5.add(0) = _mm256_add_epi32(*x4.add(0), *x4.add(3));
    *x5.add(3) = _mm256_sub_epi32(*x4.add(0), *x4.add(3));
    *x5.add(1) = _mm256_add_epi32(*x4.add(1), *x4.add(2));
    *x5.add(2) = _mm256_sub_epi32(*x4.add(1), *x4.add(2));
    *x5.add(4) = *x4.add(4);
    btf_32_type0_avx2_new!(*cospi_m32, *cospi_p32, *x4.add(5), *x4.add(6), *x5.add(5), *x5.add(6),
                           *__rounding, cos_bit);
    *x5.add(7) = *x4.add(7);
    *x5.add(8) = _mm256_add_epi32(*x4.add(8), *x4.add(11));
    *x5.add(11) = _mm256_sub_epi32(*x4.add(8), *x4.add(11));
    *x5.add(9) = _mm256_add_epi32(*x4.add(9), *x4.add(10));
    *x5.add(10) = _mm256_sub_epi32(*x4.add(9), *x4.add(10));
    *x5.add(12) = _mm256_sub_epi32(*x4.add(15), *x4.add(12));
    *x5.add(15) = _mm256_add_epi32(*x4.add(15), *x4.add(12));
    *x5.add(13) = _mm256_sub_epi32(*x4.add(14), *x4.add(13));
    *x5.add(14) = _mm256_add_epi32(*x4.add(14), *x4.add(13));
    *x5.add(16) = *x4.add(16);
    *x5.add(17) = *x4.add(17);
    btf_32_type0_avx2_new!(*cospi_m16, *cospi_p48, *x4.add(18), *x4.add(29), *x5.add(18),
                           *x5.add(29), *__rounding, cos_bit);
    btf_32_type0_avx2_new!(*cospi_m16, *cospi_p48, *x4.add(19), *x4.add(28), *x5.add(19),
                           *x5.add(28), *__rounding, cos_bit);
    btf_32_type0_avx2_new!(*cospi_m48, *cospi_m16, *x4.add(20), *x4.add(27), *x5.add(20),
                           *x5.add(27), *__rounding, cos_bit);
    btf_32_type0_avx2_new!(*cospi_m48, *cospi_m16, *x4.add(21), *x4.add(26), *x5.add(21),
                           *x5.add(26), *__rounding, cos_bit);
    *x5.add(22) = *x4.add(22);
    *x5.add(23) = *x4.add(23);
    *x5.add(24) = *x4.add(24);
    *x5.add(25) = *x4.add(25);
    *x5.add(30) = *x4.add(30);
    *x5.add(31) = *x4.add(31);
    *x5.add(32) = _mm256_add_epi32(*x4.add(32), *x4.add(39));
    *x5.add(39) = _mm256_sub_epi32(*x4.add(32), *x4.add(39));
    *x5.add(33) = _mm256_add_epi32(*x4.add(33), *x4.add(38));
    *x5.add(38) = _mm256_sub_epi32(*x4.add(33), *x4.add(38));
    *x5.add(34) = _mm256_add_epi32(*x4.add(34), *x4.add(37));
    *x5.add(37) = _mm256_sub_epi32(*x4.add(34), *x4.add(37));
    *x5.add(35) = _mm256_add_epi32(*x4.add(35), *x4.add(36));
    *x5.add(36) = _mm256_sub_epi32(*x4.add(35), *x4.add(36));
    *x5.add(40) = _mm256_sub_epi32(*x4.add(47), *x4.add(40));
    *x5.add(47) = _mm256_add_epi32(*x4.add(47), *x4.add(40));
    *x5.add(41) = _mm256_sub_epi32(*x4.add(46), *x4.add(41));
    *x5.add(46) = _mm256_add_epi32(*x4.add(46), *x4.add(41));
    *x5.add(42) = _mm256_sub_epi32(*x4.add(45), *x4.add(42));
    *x5.add(45) = _mm256_add_epi32(*x4.add(45), *x4.add(42));
    *x5.add(43) = _mm256_sub_epi32(*x4.add(44), *x4.add(43));
    *x5.add(44) = _mm256_add_epi32(*x4.add(44), *x4.add(43));
    *x5.add(48) = _mm256_add_epi32(*x4.add(48), *x4.add(55));
    *x5.add(55) = _mm256_sub_epi32(*x4.add(48), *x4.add(55));
    *x5.add(49) = _mm256_add_epi32(*x4.add(49), *x4.add(54));
    *x5.add(54) = _mm256_sub_epi32(*x4.add(49), *x4.add(54));
    *x5.add(50) = _mm256_add_epi32(*x4.add(50), *x4.add(53));
    *x5.add(53) = _mm256_sub_epi32(*x4.add(50), *x4.add(53));
    *x5.add(51) = _mm256_add_epi32(*x4.add(51), *x4.add(52));
    *x5.add(52) = _mm256_sub_epi32(*x4.add(51), *x4.add(52));
    *x5.add(56) = _mm256_sub_epi32(*x4.add(63), *x4.add(56));
    *x5.add(63) = _mm256_add_epi32(*x4.add(63), *x4.add(56));
    *x5.add(57) = _mm256_sub_epi32(*x4.add(62), *x4.add(57));
    *x5.add(62) = _mm256_add_epi32(*x4.add(62), *x4.add(57));
    *x5.add(58) = _mm256_sub_epi32(*x4.add(61), *x4.add(58));
    *x5.add(61) = _mm256_add_epi32(*x4.add(61), *x4.add(58));
    *x5.add(59) = _mm256_sub_epi32(*x4.add(60), *x4.add(59));
    *x5.add(60) = _mm256_add_epi32(*x4.add(60), *x4.add(59));
}
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn fdct64_stage6_avx2(x5: *mut __m256i, x6: *mut __m256i, cospi_p16: *mut __m256i,
                             cospi_p32: *mut __m256i, cospi_m16: *mut __m256i,
                             cospi_p48: *mut __m256i, cospi_m48: *mut __m256i,
                             cospi_m08: *mut __m256i, cospi_p56: *mut __m256i,
                             cospi_m56: *mut __m256i, cospi_m40: *mut __m256i,
                             cospi_p24: *mut __m256i, cospi_m24: *mut __m256i,
                             __rounding: *const __m256i, cos_bit: i8) {
    btf_32_type0_avx2_new!(*cospi_p32, *cospi_p32, *x5.add(0), *x5.add(1), *x6.add(0), *x6.add(1),
                           *__rounding, cos_bit);
    btf_32_type0_avx2_new!(*cospi_p16, *cospi_p48, *x5.add(3), *x5.add(2), *x6.add(2), *x6.add(3),
                           *__rounding, cos_bit);
    *x6.add(4) = _mm256_add_epi32(*x5.add(4), *x5.add(5));
    *x6.add(5) = _mm256_sub_epi32(*x5.add(4), *x5.add(5));
    *x6.add(6) = _mm256_sub_epi32(*x5.add(7), *x5.add(6));
    *x6.add(7) = _mm256_add_epi32(*x5.add(7), *x5.add(6));
    *x6.add(8) = *x5.add(8);
    btf_32_type0_avx2_new!(*cospi_m16, *cospi_p48, *x5.add(9), *x5.add(14), *x6.add(9), *x6.add(14),
                           *__rounding, cos_bit);
    btf_32_type0_avx2_new!(*cospi_m48, *cospi_m16, *x5.add(10), *x5.add(13), *x6.add(10),
                           *x6.add(13), *__rounding, cos_bit);
    *x6.add(11) = *x5.add(11);
    *x6.add(12) = *x5.add(12);
    *x6.add(15) = *x5.add(15);
    *x6.add(16) = _mm256_add_epi32(*x5.add(16), *x5.add(19));
    *x6.add(19) = _mm256_sub_epi32(*x5.add(16), *x5.add(19));
    *x6.add(17) = _mm256_add_epi32(*x5.add(17), *x5.add(18));
    *x6.add(18) = _mm256_sub_epi32(*x5.add(17), *x5.add(18));
    *x6.add(20) = _mm256_sub_epi32(*x5.add(23), *x5.add(20));
    *x6.add(23) = _mm256_add_epi32(*x5.add(23), *x5.add(20));
    *x6.add(21) = _mm256_sub_epi32(*x5.add(22), *x5.add(21));
    *x6.add(22) = _mm256_add_epi32(*x5.add(22), *x5.add(21));
    *x6.add(24) = _mm256_add_epi32(*x5.add(24), *x5.add(27));
    *x6.add(27) = _mm256_sub_epi32(*x5.add(24), *x5.add(27));
    *x6.add(25) = _mm256_add_epi32(*x5.add(25), *x5.add(26));
    *x6.add(26) = _mm256_sub_epi32(*x5.add(25), *x5.add(26));
    *x6.add(28) = _mm256_sub_epi32(*x5.add(31), *x5.add(28));
    *x6.add(31) = _mm256_add_epi32(*x5.add(31), *x5.add(28));
    *x6.add(29) = _mm256_sub_epi32(*x5.add(30), *x5.add(29));
    *x6.add(30) = _mm256_add_epi32(*x5.add(30), *x5.add(29));
    *x6.add(32) = *x5.add(32);
    *x6.add(33) = *x5.add(33);
    btf_32_type0_avx2_new!(*cospi_m08, *cospi_p56, *x5.add(34), *x5.add(61), *x6.add(34),
                           *x6.add(61), *__rounding, cos_bit);
    btf_32_type0_avx2_new!(*cospi_m08, *cospi_p56, *x5.add(35), *x5.add(60), *x6.add(35),
                           *x6.add(60), *__rounding, cos_bit);
    btf_32_type0_avx2_new!(*cospi_m56, *cospi_m08, *x5.add(36), *x5.add(59), *x6.add(36),
                           *x6.add(59), *__rounding, cos_bit);
    btf_32_type0_avx2_new!(*cospi_m56, *cospi_m08, *x5.add(37), *x5.add(58), *x6.add(37),
                           *x6.add(58), *__rounding, cos_bit);
    *x6.add(38) = *x5.add(38);
    *x6.add(39) = *x5.add(39);
    *x6.add(40) = *x5.add(40);
    *x6.add(41) = *x5.add(41);
    btf_32_type0_avx2_new!(*cospi_m40, *cospi_p24, *x5.add(42), *x5.add(53), *x6.add(42),
                           *x6.add(53), *__rounding, cos_bit);
    btf_32_type0_avx2_new!(*cospi_m40, *cospi_p24, *x5.add(43), *x5.add(52), *x6.add(43),
                           *x6.add(52), *__rounding, cos_bit);
    btf_32_type0_avx2_new!(*cospi_m24, *cospi_m40, *x5.add(44), *x5.add(51), *x6.add(44),
                           *x6.add(51), *__rounding, cos_bit);
    btf_32_type0_avx2_new!(*cospi_m24, *cospi_m40, *x5.add(45), *x5.add(50), *x6.add(45),
                           *x6.add(50), *__rounding, cos_bit);
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
}
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn fdct64_stage7_avx2(x6: *mut __m256i, x7: *mut __m256i, cospi_p08: *mut __m256i,
                             cospi_p56: *mut __m256i, cospi_p40: *mut __m256i,
                             cospi_p24: *mut __m256i, cospi_m08: *mut __m256i,
                             cospi_m56: *mut __m256i, cospi_m40: *mut __m256i,
                             cospi_m24: *mut __m256i, __rounding: *const __m256i, cos_bit: i8) {
    *x7.add(0) = *x6.add(0);
    *x7.add(1) = *x6.add(1);
    *x7.add(2) = *x6.add(2);
    *x7.add(3) = *x6.add(3);
    btf_32_type0_avx2_new!(*cospi_p08, *cospi_p56, *x6.add(7), *x6.add(4), *x7.add(4), *x7.add(7),
                           *__rounding, cos_bit);
    btf_32_type0_avx2_new!(*cospi_p40, *cospi_p24, *x6.add(6), *x6.add(5), *x7.add(5), *x7.add(6),
                           *__rounding, cos_bit);
    *x7.add(8) = _mm256_add_epi32(*x6.add(8), *x6.add(9));
    *x7.add(9) = _mm256_sub_epi32(*x6.add(8), *x6.add(9));
    *x7.add(10) = _mm256_sub_epi32(*x6.add(11), *x6.add(10));
    *x7.add(11) = _mm256_add_epi32(*x6.add(11), *x6.add(10));
    *x7.add(12) = _mm256_add_epi32(*x6.add(12), *x6.add(13));
    *x7.add(13) = _mm256_sub_epi32(*x6.add(12), *x6.add(13));
    *x7.add(14) = _mm256_sub_epi32(*x6.add(15), *x6.add(14));
    *x7.add(15) = _mm256_add_epi32(*x6.add(15), *x6.add(14));
    *x7.add(16) = *x6.add(16);
    btf_32_type0_avx2_new!(*cospi_m08, *cospi_p56, *x6.add(17), *x6.add(30), *x7.add(17),
                           *x7.add(30), *__rounding, cos_bit);
    btf_32_type0_avx2_new!(*cospi_m56, *cospi_m08, *x6.add(18), *x6.add(29), *x7.add(18),
                           *x7.add(29), *__rounding, cos_bit);
    *x7.add(19) = *x6.add(19);
    *x7.add(20) = *x6.add(20);
    btf_32_type0_avx2_new!(*cospi_m40, *cospi_p24, *x6.add(21), *x6.add(26), *x7.add(21),
                           *x7.add(26), *__rounding, cos_bit);
    btf_32_type0_avx2_new!(*cospi_m24, *cospi_m40, *x6.add(22), *x6.add(25), *x7.add(22),
                           *x7.add(25), *__rounding, cos_bit);
    *x7.add(23) = *x6.add(23);
    *x7.add(24) = *x6.add(24);
    *x7.add(27) = *x6.add(27);
    *x7.add(28) = *x6.add(28);
    *x7.add(31) = *x6.add(31);
    *x7.add(32) = _mm256_add_epi32(*x6.add(32), *x6.add(35));
    *x7.add(35) = _mm256_sub_epi32(*x6.add(32), *x6.add(35));
    *x7.add(33) = _mm256_add_epi32(*x6.add(33), *x6.add(34));
    *x7.add(34) = _mm256_sub_epi32(*x6.add(33), *x6.add(34));
    *x7.add(36) = _mm256_sub_epi32(*x6.add(39), *x6.add(36));
    *x7.add(39) = _mm256_add_epi32(*x6.add(39), *x6.add(36));
    *x7.add(37) = _mm256_sub_epi32(*x6.add(38), *x6.add(37));
    *x7.add(38) = _mm256_add_epi32(*x6.add(38), *x6.add(37));
    *x7.add(40) = _mm256_add_epi32(*x6.add(40), *x6.add(43));
    *x7.add(43) = _mm256_sub_epi32(*x6.add(40), *x6.add(43));
    *x7.add(41) = _mm256_add_epi32(*x6.add(41), *x6.add(42));
    *x7.add(42) = _mm256_sub_epi32(*x6.add(41), *x6.add(42));
    *x7.add(44) = _mm256_sub_epi32(*x6.add(47), *x6.add(44));
    *x7.add(47) = _mm256_add_epi32(*x6.add(47), *x6.add(44));
    *x7.add(45) = _mm256_sub_epi32(*x6.add(46), *x6.add(45));
    *x7.add(46) = _mm256_add_epi32(*x6.add(46), *x6.add(45));
    *x7.add(48) = _mm256_add_epi32(*x6.add(48), *x6.add(51));
    *x7.add(51) = _mm256_sub_epi32(*x6.add(48), *x6.add(51));
    *x7.add(49) = _mm256_add_epi32(*x6.add(49), *x6.add(50));
    *x7.add(50) = _mm256_sub_epi32(*x6.add(49), *x6.add(50));
    *x7.add(52) = _mm256_sub_epi32(*x6.add(55), *x6.add(52));
    *x7.add(55) = _mm256_add_epi32(*x6.add(55), *x6.add(52));
    *x7.add(53) = _mm256_sub_epi32(*x6.add(54), *x6.add(53));
    *x7.add(54) = _mm256_add_epi32(*x6.add(54), *x6.add(53));
    *x7.add(56) = _mm256_add_epi32(*x6.add(56), *x6.add(59));
    *x7.add(59) = _mm256_sub_epi32(*x6.add(56), *x6.add(59));
    *x7.add(57) = _mm256_add_epi32(*x6.add(57), *x6.add(58));
    *x7.add(58) = _mm256_sub_epi32(*x6.add(57), *x6.add(58));
    *x7.add(60) = _mm256_sub_epi32(*x6.add(63), *x6.add(60));
    *x7.add(63) = _mm256_add_epi32(*x6.add(63), *x6.add(60));
    *x7.add(61) = _mm256_sub_epi32(*x6.add(62), *x6.add(61));
    *x7.add(62) = _mm256_add_epi32(*x6.add(62), *x6.add(61));
}
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn fdct64_stage8_avx2(x7: *mut __m256i, x8: *mut __m256i, cospi: *const i32,
                             __rounding: *const __m256i, cos_bit: i8) {
    let mut cospi_p60: __m256i = _mm256_set1_epi32(*cospi.add(60));
    let mut cospi_p04: __m256i = _mm256_set1_epi32(*cospi.add(4));
    let mut cospi_p28: __m256i = _mm256_set1_epi32(*cospi.add(28));
    let mut cospi_p36: __m256i = _mm256_set1_epi32(*cospi.add(36));
    let mut cospi_p44: __m256i = _mm256_set1_epi32(*cospi.add(44));
    let mut cospi_p20: __m256i = _mm256_set1_epi32(*cospi.add(20));
    let mut cospi_p12: __m256i = _mm256_set1_epi32(*cospi.add(12));
    let mut cospi_p52: __m256i = _mm256_set1_epi32(*cospi.add(52));
    let mut cospi_m04: __m256i = _mm256_set1_epi32(-*cospi.add(4));
    let mut cospi_m60: __m256i = _mm256_set1_epi32(-*cospi.add(60));
    let mut cospi_m36: __m256i = _mm256_set1_epi32(-*cospi.add(36));
    let mut cospi_m28: __m256i = _mm256_set1_epi32(-*cospi.add(28));
    let mut cospi_m20: __m256i = _mm256_set1_epi32(-*cospi.add(20));
    let mut cospi_m44: __m256i = _mm256_set1_epi32(-*cospi.add(44));
    let mut cospi_m52: __m256i = _mm256_set1_epi32(-*cospi.add(52));
    let mut cospi_m12: __m256i = _mm256_set1_epi32(-*cospi.add(12));

    *x8.add(0) = *x7.add(0);
    *x8.add(1) = *x7.add(1);
    *x8.add(2) = *x7.add(2);
    *x8.add(3) = *x7.add(3);
    *x8.add(4) = *x7.add(4);
    *x8.add(5) = *x7.add(5);
    *x8.add(6) = *x7.add(6);
    *x8.add(7) = *x7.add(7);

    btf_32_type0_avx2_new!(cospi_p04, cospi_p60, *x7.add(15), *x7.add(8), *x8.add(8), *x8.add(15),
                           *__rounding, cos_bit);
    btf_32_type0_avx2_new!(cospi_p36, cospi_p28, *x7.add(14), *x7.add(9), *x8.add(9), *x8.add(14),
                           *__rounding, cos_bit);
    btf_32_type0_avx2_new!(cospi_p20, cospi_p44, *x7.add(13), *x7.add(10), *x8.add(10), *x8.add(13),
                           *__rounding, cos_bit);
    btf_32_type0_avx2_new!(cospi_p52, cospi_p12, *x7.add(12), *x7.add(11), *x8.add(11), *x8.add(12),
                           *__rounding, cos_bit);
    *x8.add(16) = _mm256_add_epi32(*x7.add(16), *x7.add(17));
    *x8.add(17) = _mm256_sub_epi32(*x7.add(16), *x7.add(17));
    *x8.add(18) = _mm256_sub_epi32(*x7.add(19), *x7.add(18));
    *x8.add(19) = _mm256_add_epi32(*x7.add(19), *x7.add(18));
    *x8.add(20) = _mm256_add_epi32(*x7.add(20), *x7.add(21));
    *x8.add(21) = _mm256_sub_epi32(*x7.add(20), *x7.add(21));
    *x8.add(22) = _mm256_sub_epi32(*x7.add(23), *x7.add(22));
    *x8.add(23) = _mm256_add_epi32(*x7.add(23), *x7.add(22));
    *x8.add(24) = _mm256_add_epi32(*x7.add(24), *x7.add(25));
    *x8.add(25) = _mm256_sub_epi32(*x7.add(24), *x7.add(25));
    *x8.add(26) = _mm256_sub_epi32(*x7.add(27), *x7.add(26));
    *x8.add(27) = _mm256_add_epi32(*x7.add(27), *x7.add(26));
    *x8.add(28) = _mm256_add_epi32(*x7.add(28), *x7.add(29));
    *x8.add(29) = _mm256_sub_epi32(*x7.add(28), *x7.add(29));
    *x8.add(30) = _mm256_sub_epi32(*x7.add(31), *x7.add(30));
    *x8.add(31) = _mm256_add_epi32(*x7.add(31), *x7.add(30));
    *x8.add(32) = *x7.add(32);
    btf_32_type0_avx2_new!(cospi_m04, cospi_p60, *x7.add(33), *x7.add(62), *x8.add(33), *x8.add(62),
                           *__rounding, cos_bit);
    btf_32_type0_avx2_new!(cospi_m60, cospi_m04, *x7.add(34), *x7.add(61), *x8.add(34), *x8.add(61),
                           *__rounding, cos_bit);
    *x8.add(35) = *x7.add(35);
    *x8.add(36) = *x7.add(36);
    btf_32_type0_avx2_new!(cospi_m36, cospi_p28, *x7.add(37), *x7.add(58), *x8.add(37), *x8.add(58),
                           *__rounding, cos_bit);
    btf_32_type0_avx2_new!(cospi_m28, cospi_m36, *x7.add(38), *x7.add(57), *x8.add(38), *x8.add(57),
                           *__rounding, cos_bit);
    *x8.add(39) = *x7.add(39);
    *x8.add(40) = *x7.add(40);
    btf_32_type0_avx2_new!(cospi_m20, cospi_p44, *x7.add(41), *x7.add(54), *x8.add(41), *x8.add(54),
                           *__rounding, cos_bit);
    btf_32_type0_avx2_new!(cospi_m44, cospi_m20, *x7.add(42), *x7.add(53), *x8.add(42), *x8.add(53),
                           *__rounding, cos_bit);
    *x8.add(43) = *x7.add(43);
    *x8.add(44) = *x7.add(44);
    btf_32_type0_avx2_new!(cospi_m52, cospi_p12, *x7.add(45), *x7.add(50), *x8.add(45), *x8.add(50),
                           *__rounding, cos_bit);
    btf_32_type0_avx2_new!(cospi_m12, cospi_m52, *x7.add(46), *x7.add(49), *x8.add(46), *x8.add(49),
                           *__rounding, cos_bit);
    *x8.add(47) = *x7.add(47);
    *x8.add(48) = *x7.add(48);
    *x8.add(51) = *x7.add(51);
    *x8.add(52) = *x7.add(52);
    *x8.add(55) = *x7.add(55);
    *x8.add(56) = *x7.add(56);
    *x8.add(59) = *x7.add(59);
    *x8.add(60) = *x7.add(60);
    *x8.add(63) = *x7.add(63);
}
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn fdct64_stage9_avx2(x8: *mut __m256i, x9: *mut __m256i, cospi: *const i32,
                             __rounding: *const __m256i, cos_bit: i8) {
    let mut cospi_p62: __m256i = _mm256_set1_epi32(*cospi.add(62));
    let mut cospi_p02: __m256i = _mm256_set1_epi32(*cospi.add(2));
    let mut cospi_p30: __m256i = _mm256_set1_epi32(*cospi.add(30));
    let mut cospi_p34: __m256i = _mm256_set1_epi32(*cospi.add(34));
    let mut cospi_p46: __m256i = _mm256_set1_epi32(*cospi.add(46));
    let mut cospi_p18: __m256i = _mm256_set1_epi32(*cospi.add(18));
    let mut cospi_p14: __m256i = _mm256_set1_epi32(*cospi.add(14));
    let mut cospi_p50: __m256i = _mm256_set1_epi32(*cospi.add(50));
    let mut cospi_p54: __m256i = _mm256_set1_epi32(*cospi.add(54));
    let mut cospi_p10: __m256i = _mm256_set1_epi32(*cospi.add(10));
    let mut cospi_p22: __m256i = _mm256_set1_epi32(*cospi.add(22));
    let mut cospi_p42: __m256i = _mm256_set1_epi32(*cospi.add(42));
    let mut cospi_p38: __m256i = _mm256_set1_epi32(*cospi.add(38));
    let mut cospi_p26: __m256i = _mm256_set1_epi32(*cospi.add(26));
    let mut cospi_p06: __m256i = _mm256_set1_epi32(*cospi.add(6));
    let mut cospi_p58: __m256i = _mm256_set1_epi32(*cospi.add(58));

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
    btf_32_type0_avx2_new!(cospi_p02, cospi_p62, *x8.add(31), *x8.add(16), *x9.add(16), *x9.add(31),
                           *__rounding, cos_bit);
    btf_32_type0_avx2_new!(cospi_p34, cospi_p30, *x8.add(30), *x8.add(17), *x9.add(17), *x9.add(30),
                           *__rounding, cos_bit);
    btf_32_type0_avx2_new!(cospi_p18, cospi_p46, *x8.add(29), *x8.add(18), *x9.add(18), *x9.add(29),
                           *__rounding, cos_bit);
    btf_32_type0_avx2_new!(cospi_p50, cospi_p14, *x8.add(28), *x8.add(19), *x9.add(19), *x9.add(28),
                           *__rounding, cos_bit);
    btf_32_type0_avx2_new!(cospi_p10, cospi_p54, *x8.add(27), *x8.add(20), *x9.add(20), *x9.add(27),
                           *__rounding, cos_bit);
    btf_32_type0_avx2_new!(cospi_p42, cospi_p22, *x8.add(26), *x8.add(21), *x9.add(21), *x9.add(26),
                           *__rounding, cos_bit);
    btf_32_type0_avx2_new!(cospi_p26, cospi_p38, *x8.add(25), *x8.add(22), *x9.add(22), *x9.add(25),
                           *__rounding, cos_bit);
    btf_32_type0_avx2_new!(cospi_p58, cospi_p06, *x8.add(24), *x8.add(23), *x9.add(23), *x9.add(24),
                           *__rounding, cos_bit);
    *x9.add(32) = _mm256_add_epi32(*x8.add(32), *x8.add(33));
    *x9.add(33) = _mm256_sub_epi32(*x8.add(32), *x8.add(33));
    *x9.add(34) = _mm256_sub_epi32(*x8.add(35), *x8.add(34));
    *x9.add(35) = _mm256_add_epi32(*x8.add(35), *x8.add(34));
    *x9.add(36) = _mm256_add_epi32(*x8.add(36), *x8.add(37));
    *x9.add(37) = _mm256_sub_epi32(*x8.add(36), *x8.add(37));
    *x9.add(38) = _mm256_sub_epi32(*x8.add(39), *x8.add(38));
    *x9.add(39) = _mm256_add_epi32(*x8.add(39), *x8.add(38));
    *x9.add(40) = _mm256_add_epi32(*x8.add(40), *x8.add(41));
    *x9.add(41) = _mm256_sub_epi32(*x8.add(40), *x8.add(41));
    *x9.add(42) = _mm256_sub_epi32(*x8.add(43), *x8.add(42));
    *x9.add(43) = _mm256_add_epi32(*x8.add(43), *x8.add(42));
    *x9.add(44) = _mm256_add_epi32(*x8.add(44), *x8.add(45));
    *x9.add(45) = _mm256_sub_epi32(*x8.add(44), *x8.add(45));
    *x9.add(46) = _mm256_sub_epi32(*x8.add(47), *x8.add(46));
    *x9.add(47) = _mm256_add_epi32(*x8.add(47), *x8.add(46));
    *x9.add(48) = _mm256_add_epi32(*x8.add(48), *x8.add(49));
    *x9.add(49) = _mm256_sub_epi32(*x8.add(48), *x8.add(49));
    *x9.add(50) = _mm256_sub_epi32(*x8.add(51), *x8.add(50));
    *x9.add(51) = _mm256_add_epi32(*x8.add(51), *x8.add(50));
    *x9.add(52) = _mm256_add_epi32(*x8.add(52), *x8.add(53));
    *x9.add(53) = _mm256_sub_epi32(*x8.add(52), *x8.add(53));
    *x9.add(54) = _mm256_sub_epi32(*x8.add(55), *x8.add(54));
    *x9.add(55) = _mm256_add_epi32(*x8.add(55), *x8.add(54));
    *x9.add(56) = _mm256_add_epi32(*x8.add(56), *x8.add(57));
    *x9.add(57) = _mm256_sub_epi32(*x8.add(56), *x8.add(57));
    *x9.add(58) = _mm256_sub_epi32(*x8.add(59), *x8.add(58));
    *x9.add(59) = _mm256_add_epi32(*x8.add(59), *x8.add(58));
    *x9.add(60) = _mm256_add_epi32(*x8.add(60), *x8.add(61));
    *x9.add(61) = _mm256_sub_epi32(*x8.add(60), *x8.add(61));
    *x9.add(62) = _mm256_sub_epi32(*x8.add(63), *x8.add(62));
    *x9.add(63) = _mm256_add_epi32(*x8.add(63), *x8.add(62));
}
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn fdct64_stage10_avx2(x9: *mut __m256i, x10: *mut __m256i, cospi: *const i32,
                              __rounding: *const __m256i, cos_bit: i8) {
    let mut cospi_p63: __m256i = _mm256_set1_epi32(*cospi.add(63));
    let mut cospi_p01: __m256i = _mm256_set1_epi32(*cospi.add(1));
    let mut cospi_p31: __m256i = _mm256_set1_epi32(*cospi.add(31));
    let mut cospi_p33: __m256i = _mm256_set1_epi32(*cospi.add(33));
    let mut cospi_p47: __m256i = _mm256_set1_epi32(*cospi.add(47));
    let mut cospi_p17: __m256i = _mm256_set1_epi32(*cospi.add(17));
    let mut cospi_p15: __m256i = _mm256_set1_epi32(*cospi.add(15));
    let mut cospi_p49: __m256i = _mm256_set1_epi32(*cospi.add(49));
    let mut cospi_p55: __m256i = _mm256_set1_epi32(*cospi.add(55));
    let mut cospi_p09: __m256i = _mm256_set1_epi32(*cospi.add(9));
    let mut cospi_p23: __m256i = _mm256_set1_epi32(*cospi.add(23));
    let mut cospi_p41: __m256i = _mm256_set1_epi32(*cospi.add(41));
    let mut cospi_p39: __m256i = _mm256_set1_epi32(*cospi.add(39));
    let mut cospi_p25: __m256i = _mm256_set1_epi32(*cospi.add(25));
    let mut cospi_p07: __m256i = _mm256_set1_epi32(*cospi.add(7));
    let mut cospi_p57: __m256i = _mm256_set1_epi32(*cospi.add(57));
    let mut cospi_p59: __m256i = _mm256_set1_epi32(*cospi.add(59));
    let mut cospi_p05: __m256i = _mm256_set1_epi32(*cospi.add(5));
    let mut cospi_p27: __m256i = _mm256_set1_epi32(*cospi.add(27));
    let mut cospi_p37: __m256i = _mm256_set1_epi32(*cospi.add(37));
    let mut cospi_p43: __m256i = _mm256_set1_epi32(*cospi.add(43));
    let mut cospi_p21: __m256i = _mm256_set1_epi32(*cospi.add(21));
    let mut cospi_p11: __m256i = _mm256_set1_epi32(*cospi.add(11));
    let mut cospi_p53: __m256i = _mm256_set1_epi32(*cospi.add(53));
    let mut cospi_p51: __m256i = _mm256_set1_epi32(*cospi.add(51));
    let mut cospi_p13: __m256i = _mm256_set1_epi32(*cospi.add(13));
    let mut cospi_p19: __m256i = _mm256_set1_epi32(*cospi.add(19));
    let mut cospi_p45: __m256i = _mm256_set1_epi32(*cospi.add(45));
    let mut cospi_p35: __m256i = _mm256_set1_epi32(*cospi.add(35));
    let mut cospi_p29: __m256i = _mm256_set1_epi32(*cospi.add(29));
    let mut cospi_p03: __m256i = _mm256_set1_epi32(*cospi.add(3));
    let mut cospi_p61: __m256i = _mm256_set1_epi32(*cospi.add(61));

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
    btf_32_type0_avx2_new!(cospi_p01, cospi_p63, *x9.add(63), *x9.add(32), *x10.add(32),
                           *x10.add(63), *__rounding, cos_bit);
    btf_32_type0_avx2_new!(cospi_p33, cospi_p31, *x9.add(62), *x9.add(33), *x10.add(33),
                           *x10.add(62), *__rounding, cos_bit);
    btf_32_type0_avx2_new!(cospi_p17, cospi_p47, *x9.add(61), *x9.add(34), *x10.add(34),
                           *x10.add(61), *__rounding, cos_bit);
    btf_32_type0_avx2_new!(cospi_p49, cospi_p15, *x9.add(60), *x9.add(35), *x10.add(35),
                           *x10.add(60), *__rounding, cos_bit);
    btf_32_type0_avx2_new!(cospi_p09, cospi_p55, *x9.add(59), *x9.add(36), *x10.add(36),
                           *x10.add(59), *__rounding, cos_bit);
    btf_32_type0_avx2_new!(cospi_p41, cospi_p23, *x9.add(58), *x9.add(37), *x10.add(37),
                           *x10.add(58), *__rounding, cos_bit);
    btf_32_type0_avx2_new!(cospi_p25, cospi_p39, *x9.add(57), *x9.add(38), *x10.add(38),
                           *x10.add(57), *__rounding, cos_bit);
    btf_32_type0_avx2_new!(cospi_p57, cospi_p07, *x9.add(56), *x9.add(39), *x10.add(39),
                           *x10.add(56), *__rounding, cos_bit);
    btf_32_type0_avx2_new!(cospi_p05, cospi_p59, *x9.add(55), *x9.add(40), *x10.add(40),
                           *x10.add(55), *__rounding, cos_bit);
    btf_32_type0_avx2_new!(cospi_p37, cospi_p27, *x9.add(54), *x9.add(41), *x10.add(41),
                           *x10.add(54), *__rounding, cos_bit);
    btf_32_type0_avx2_new!(cospi_p21, cospi_p43, *x9.add(53), *x9.add(42), *x10.add(42),
                           *x10.add(53), *__rounding, cos_bit);
    btf_32_type0_avx2_new!(cospi_p53, cospi_p11, *x9.add(52), *x9.add(43), *x10.add(43),
                           *x10.add(52), *__rounding, cos_bit);
    btf_32_type0_avx2_new!(cospi_p13, cospi_p51, *x9.add(51), *x9.add(44), *x10.add(44),
                           *x10.add(51), *__rounding, cos_bit);
    btf_32_type0_avx2_new!(cospi_p45, cospi_p19, *x9.add(50), *x9.add(45), *x10.add(45),
                           *x10.add(50), *__rounding, cos_bit);
    btf_32_type0_avx2_new!(cospi_p29, cospi_p35, *x9.add(49), *x9.add(46), *x10.add(46),
                           *x10.add(49), *__rounding, cos_bit);
    btf_32_type0_avx2_new!(cospi_p61, cospi_p03, *x9.add(48), *x9.add(47), *x10.add(47),
                           *x10.add(48), *__rounding, cos_bit);
}
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn fdct64_avx2(input: *mut __m256i, output: *mut __m256i, cos_bit: i8, instride: i32,
                      outstride: i32) {
    let cospi: *const i32 = cospi_arr(cos_bit as i32);
    let __rounding: __m256i = _mm256_set1_epi32(1 << (cos_bit - 1));
    let mut cospi_m32: __m256i = _mm256_set1_epi32(-*cospi.add(32));
    let mut cospi_p32: __m256i = _mm256_set1_epi32(*cospi.add(32));
    let mut cospi_m16: __m256i = _mm256_set1_epi32(-*cospi.add(16));
    let mut cospi_p48: __m256i = _mm256_set1_epi32(*cospi.add(48));
    let mut cospi_m48: __m256i = _mm256_set1_epi32(-*cospi.add(48));
    let mut cospi_p16: __m256i = _mm256_set1_epi32(*cospi.add(16));
    let mut cospi_m08: __m256i = _mm256_set1_epi32(-*cospi.add(8));
    let mut cospi_p56: __m256i = _mm256_set1_epi32(*cospi.add(56));
    let mut cospi_m56: __m256i = _mm256_set1_epi32(-*cospi.add(56));
    let mut cospi_m40: __m256i = _mm256_set1_epi32(-*cospi.add(40));
    let mut cospi_p24: __m256i = _mm256_set1_epi32(*cospi.add(24));
    let mut cospi_m24: __m256i = _mm256_set1_epi32(-*cospi.add(24));
    let mut cospi_p08: __m256i = _mm256_set1_epi32(*cospi.add(8));
    let mut cospi_p40: __m256i = _mm256_set1_epi32(*cospi.add(40));

    let mut startidx: i32 = 0 * instride;
    let mut endidx: i32 = 63 * instride;
    // stage 1
    let mut x1_: [__m256i; 64] = core::mem::zeroed();
    let x1 = x1_.as_mut_ptr();
    *x1.add(0) = _mm256_add_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    *x1.add(63) = _mm256_sub_epi32(*input.offset(startidx as isize),
                                   *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *x1.add(1) = _mm256_add_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    *x1.add(62) = _mm256_sub_epi32(*input.offset(startidx as isize),
                                   *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *x1.add(2) = _mm256_add_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    *x1.add(61) = _mm256_sub_epi32(*input.offset(startidx as isize),
                                   *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *x1.add(3) = _mm256_add_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    *x1.add(60) = _mm256_sub_epi32(*input.offset(startidx as isize),
                                   *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *x1.add(4) = _mm256_add_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    *x1.add(59) = _mm256_sub_epi32(*input.offset(startidx as isize),
                                   *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *x1.add(5) = _mm256_add_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    *x1.add(58) = _mm256_sub_epi32(*input.offset(startidx as isize),
                                   *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *x1.add(6) = _mm256_add_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    *x1.add(57) = _mm256_sub_epi32(*input.offset(startidx as isize),
                                   *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *x1.add(7) = _mm256_add_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    *x1.add(56) = _mm256_sub_epi32(*input.offset(startidx as isize),
                                   *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *x1.add(8) = _mm256_add_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    *x1.add(55) = _mm256_sub_epi32(*input.offset(startidx as isize),
                                   *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *x1.add(9) = _mm256_add_epi32(*input.offset(startidx as isize), *input.offset(endidx as isize));
    *x1.add(54) = _mm256_sub_epi32(*input.offset(startidx as isize),
                                   *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *x1.add(10) = _mm256_add_epi32(*input.offset(startidx as isize),
                                   *input.offset(endidx as isize));
    *x1.add(53) = _mm256_sub_epi32(*input.offset(startidx as isize),
                                   *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *x1.add(11) = _mm256_add_epi32(*input.offset(startidx as isize),
                                   *input.offset(endidx as isize));
    *x1.add(52) = _mm256_sub_epi32(*input.offset(startidx as isize),
                                   *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *x1.add(12) = _mm256_add_epi32(*input.offset(startidx as isize),
                                   *input.offset(endidx as isize));
    *x1.add(51) = _mm256_sub_epi32(*input.offset(startidx as isize),
                                   *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *x1.add(13) = _mm256_add_epi32(*input.offset(startidx as isize),
                                   *input.offset(endidx as isize));
    *x1.add(50) = _mm256_sub_epi32(*input.offset(startidx as isize),
                                   *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *x1.add(14) = _mm256_add_epi32(*input.offset(startidx as isize),
                                   *input.offset(endidx as isize));
    *x1.add(49) = _mm256_sub_epi32(*input.offset(startidx as isize),
                                   *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *x1.add(15) = _mm256_add_epi32(*input.offset(startidx as isize),
                                   *input.offset(endidx as isize));
    *x1.add(48) = _mm256_sub_epi32(*input.offset(startidx as isize),
                                   *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *x1.add(16) = _mm256_add_epi32(*input.offset(startidx as isize),
                                   *input.offset(endidx as isize));
    *x1.add(47) = _mm256_sub_epi32(*input.offset(startidx as isize),
                                   *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *x1.add(17) = _mm256_add_epi32(*input.offset(startidx as isize),
                                   *input.offset(endidx as isize));
    *x1.add(46) = _mm256_sub_epi32(*input.offset(startidx as isize),
                                   *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *x1.add(18) = _mm256_add_epi32(*input.offset(startidx as isize),
                                   *input.offset(endidx as isize));
    *x1.add(45) = _mm256_sub_epi32(*input.offset(startidx as isize),
                                   *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *x1.add(19) = _mm256_add_epi32(*input.offset(startidx as isize),
                                   *input.offset(endidx as isize));
    *x1.add(44) = _mm256_sub_epi32(*input.offset(startidx as isize),
                                   *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *x1.add(20) = _mm256_add_epi32(*input.offset(startidx as isize),
                                   *input.offset(endidx as isize));
    *x1.add(43) = _mm256_sub_epi32(*input.offset(startidx as isize),
                                   *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *x1.add(21) = _mm256_add_epi32(*input.offset(startidx as isize),
                                   *input.offset(endidx as isize));
    *x1.add(42) = _mm256_sub_epi32(*input.offset(startidx as isize),
                                   *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *x1.add(22) = _mm256_add_epi32(*input.offset(startidx as isize),
                                   *input.offset(endidx as isize));
    *x1.add(41) = _mm256_sub_epi32(*input.offset(startidx as isize),
                                   *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *x1.add(23) = _mm256_add_epi32(*input.offset(startidx as isize),
                                   *input.offset(endidx as isize));
    *x1.add(40) = _mm256_sub_epi32(*input.offset(startidx as isize),
                                   *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *x1.add(24) = _mm256_add_epi32(*input.offset(startidx as isize),
                                   *input.offset(endidx as isize));
    *x1.add(39) = _mm256_sub_epi32(*input.offset(startidx as isize),
                                   *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *x1.add(25) = _mm256_add_epi32(*input.offset(startidx as isize),
                                   *input.offset(endidx as isize));
    *x1.add(38) = _mm256_sub_epi32(*input.offset(startidx as isize),
                                   *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *x1.add(26) = _mm256_add_epi32(*input.offset(startidx as isize),
                                   *input.offset(endidx as isize));
    *x1.add(37) = _mm256_sub_epi32(*input.offset(startidx as isize),
                                   *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *x1.add(27) = _mm256_add_epi32(*input.offset(startidx as isize),
                                   *input.offset(endidx as isize));
    *x1.add(36) = _mm256_sub_epi32(*input.offset(startidx as isize),
                                   *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *x1.add(28) = _mm256_add_epi32(*input.offset(startidx as isize),
                                   *input.offset(endidx as isize));
    *x1.add(35) = _mm256_sub_epi32(*input.offset(startidx as isize),
                                   *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *x1.add(29) = _mm256_add_epi32(*input.offset(startidx as isize),
                                   *input.offset(endidx as isize));
    *x1.add(34) = _mm256_sub_epi32(*input.offset(startidx as isize),
                                   *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *x1.add(30) = _mm256_add_epi32(*input.offset(startidx as isize),
                                   *input.offset(endidx as isize));
    *x1.add(33) = _mm256_sub_epi32(*input.offset(startidx as isize),
                                   *input.offset(endidx as isize));
    startidx += instride;
    endidx -= instride;
    *x1.add(31) = _mm256_add_epi32(*input.offset(startidx as isize),
                                   *input.offset(endidx as isize));
    *x1.add(32) = _mm256_sub_epi32(*input.offset(startidx as isize),
                                   *input.offset(endidx as isize));

    // stage 2
    let mut x2_: [__m256i; 64] = core::mem::zeroed();
    let x2 = x2_.as_mut_ptr();
    fdct64_stage2_avx2(x1, x2, &raw mut cospi_m32, &raw mut cospi_p32, &__rounding, cos_bit);
    // stage 3
    fdct64_stage3_avx2(x2, x1, &raw mut cospi_m32, &raw mut cospi_p32, &__rounding, cos_bit);
    // stage 4
    fdct64_stage4_avx2(x1, x2, &raw mut cospi_m32, &raw mut cospi_p32, &raw mut cospi_m16,
                       &raw mut cospi_p48, &raw mut cospi_m48, &__rounding, cos_bit);
    // stage 5
    fdct64_stage5_avx2(x2, x1, &raw mut cospi_m32, &raw mut cospi_p32, &raw mut cospi_m16,
                       &raw mut cospi_p48, &raw mut cospi_m48, &__rounding, cos_bit);
    // stage 6
    fdct64_stage6_avx2(x1, x2, &raw mut cospi_p16, &raw mut cospi_p32, &raw mut cospi_m16,
                       &raw mut cospi_p48, &raw mut cospi_m48, &raw mut cospi_m08,
                       &raw mut cospi_p56, &raw mut cospi_m56, &raw mut cospi_m40,
                       &raw mut cospi_p24, &raw mut cospi_m24, &__rounding, cos_bit);
    // stage 7
    fdct64_stage7_avx2(x2, x1, &raw mut cospi_p08, &raw mut cospi_p56, &raw mut cospi_p40,
                       &raw mut cospi_p24, &raw mut cospi_m08, &raw mut cospi_m56,
                       &raw mut cospi_m40, &raw mut cospi_m24, &__rounding, cos_bit);
    // stage 8
    fdct64_stage8_avx2(x1, x2, cospi, &__rounding, cos_bit);
    // stage 9
    fdct64_stage9_avx2(x2, x1, cospi, &__rounding, cos_bit);
    // stage 10
    fdct64_stage10_avx2(x1, x2, cospi, &__rounding, cos_bit);

    startidx = 0 * outstride;
    endidx = 63 * outstride;

    // stage 11
    *output.offset(startidx as isize) = *x2.add(0);
    *output.offset(endidx as isize) = *x2.add(63);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *x2.add(32);
    *output.offset(endidx as isize) = *x2.add(31);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *x2.add(16);
    *output.offset(endidx as isize) = *x2.add(47);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *x2.add(48);
    *output.offset(endidx as isize) = *x2.add(15);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *x2.add(8);
    *output.offset(endidx as isize) = *x2.add(55);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *x2.add(40);
    *output.offset(endidx as isize) = *x2.add(23);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *x2.add(24);
    *output.offset(endidx as isize) = *x2.add(39);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *x2.add(56);
    *output.offset(endidx as isize) = *x2.add(7);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *x2.add(4);
    *output.offset(endidx as isize) = *x2.add(59);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *x2.add(36);
    *output.offset(endidx as isize) = *x2.add(27);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *x2.add(20);
    *output.offset(endidx as isize) = *x2.add(43);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *x2.add(52);
    *output.offset(endidx as isize) = *x2.add(11);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *x2.add(12);
    *output.offset(endidx as isize) = *x2.add(51);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *x2.add(44);
    *output.offset(endidx as isize) = *x2.add(19);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *x2.add(28);
    *output.offset(endidx as isize) = *x2.add(35);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *x2.add(60);
    *output.offset(endidx as isize) = *x2.add(3);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *x2.add(2);
    *output.offset(endidx as isize) = *x2.add(61);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *x2.add(34);
    *output.offset(endidx as isize) = *x2.add(29);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *x2.add(18);
    *output.offset(endidx as isize) = *x2.add(45);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *x2.add(50);
    *output.offset(endidx as isize) = *x2.add(13);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *x2.add(10);
    *output.offset(endidx as isize) = *x2.add(53);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *x2.add(42);
    *output.offset(endidx as isize) = *x2.add(21);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *x2.add(26);
    *output.offset(endidx as isize) = *x2.add(37);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *x2.add(58);
    *output.offset(endidx as isize) = *x2.add(5);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *x2.add(6);
    *output.offset(endidx as isize) = *x2.add(57);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *x2.add(38);
    *output.offset(endidx as isize) = *x2.add(25);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *x2.add(22);
    *output.offset(endidx as isize) = *x2.add(41);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *x2.add(54);
    *output.offset(endidx as isize) = *x2.add(9);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *x2.add(14);
    *output.offset(endidx as isize) = *x2.add(49);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *x2.add(46);
    *output.offset(endidx as isize) = *x2.add(17);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *x2.add(30);
    *output.offset(endidx as isize) = *x2.add(33);
    startidx += outstride;
    endidx -= outstride;
    *output.offset(startidx as isize) = *x2.add(62);
    *output.offset(endidx as isize) = *x2.add(1);
}
#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn av1_fwd_txfm2d_64x64_avx2(input: *const i16, output: *mut i32,
                                                          stride: i32, tx_type: TX_TYPE,
                                                          bd: i32) {
    let _ = bd;
    let _ = tx_type;
    debug_assert!(tx_type == DCT_DCT);
    let tx_size: TX_SIZE = TX_64X64;
    let mut buf0_: [__m256i; 512] = core::mem::zeroed();
    let mut buf1_: [__m256i; 512] = core::mem::zeroed();
    let buf0 = buf0_.as_mut_ptr();
    let buf1 = buf1_.as_mut_ptr();
    let shift: *const i8 = av1_fwd_txfm_shift_ls[tx_size as usize];
    let txw_idx: i32 = get_txw_idx(tx_size);
    let txh_idx: i32 = get_txh_idx(tx_size);
    let cos_bit_col: i32 = av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32;
    let cos_bit_row: i32 = av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32;
    let width: i32 = tx_size_wide[tx_size as usize];
    let height: i32 = tx_size_high[tx_size as usize];
    let col_txfm: unsafe fn(*mut __m256i, *mut __m256i, i8, i32, i32) = fdct64_avx2;
    let row_txfm: unsafe fn(*mut __m256i, *mut __m256i, i8, i32, i32) = fdct64_avx2;
    let width_div16: i32 = width >> 4;
    let width_div8: i32 = width >> 3;
    let mut r: i32;
    let mut c: i32;
    let mut i: i32 = 0;
    while i < width_div16 {
        load_buffer_16xn_avx2(input.offset((i << 4) as isize), buf0.offset((i << 1) as isize),
                              stride, height, width_div8, 0, 0);
        round_shift_32_8xn_avx2(buf0.offset((i << 1) as isize), height, *shift.add(0) as i32,
                                width_div8);
        round_shift_32_8xn_avx2(buf0.offset(((i << 1) + 1) as isize), height,
                                *shift.add(0) as i32, width_div8);
        col_txfm(buf0.offset((i << 1) as isize), buf0.offset((i << 1) as isize),
                 cos_bit_col as i8, width_div8, width_div8);
        col_txfm(buf0.offset(((i << 1) + 1) as isize), buf0.offset(((i << 1) + 1) as isize),
                 cos_bit_col as i8, width_div8, width_div8);
        round_shift_32_8xn_avx2(buf0.offset((i << 1) as isize), height, *shift.add(1) as i32,
                                width_div8);
        round_shift_32_8xn_avx2(buf0.offset(((i << 1) + 1) as isize), height,
                                *shift.add(1) as i32, width_div8);
        i += 1;
    }

    r = 0;
    while r < height {
        c = 0;
        while c < width_div8 {
            fwd_txfm_transpose_8x8_avx2(buf0.offset((r * width_div8 + c) as isize),
                                        buf1.offset((c * 8 * width_div8 + (r >> 3)) as isize),
                                        width_div8, width_div8);
            c += 1;
        }
        r += 8;
    }

    let mut i: i32 = 0;
    while i < 2 {
        row_txfm(buf1.offset((i << 1) as isize), buf0.offset((i << 1) as isize),
                 cos_bit_row as i8, width_div8, width_div16);
        row_txfm(buf1.offset(((i << 1) + 1) as isize), buf0.offset(((i << 1) + 1) as isize),
                 cos_bit_row as i8, width_div8, width_div16);
        round_shift_32_8xn_avx2(buf0.offset((i << 1) as isize), height >> 1,
                                *shift.add(2) as i32, width_div16);
        round_shift_32_8xn_avx2(buf0.offset(((i << 1) + 1) as isize), height >> 1,
                                *shift.add(2) as i32, width_div16);
        i += 1;
    }

    store_buffer_avx2(buf0, output, 8, 128);
}
