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

// Translated from upstream/aom/av1/encoder/x86/av1_fwd_txfm_sse2.h and av1_fwd_txfm_sse2.c
// (ADR 0056, ADR 0058). Upstream compiles the .c file with -msse2. The inline functions,
// tables and functions were
// transliterated with a typing-aware C-subset printer (not C2Rust) and
// reviewed against the C function by function: the same sequence of intrinsic
// and helper calls, constants and operators in order (ADR 0058).
// av1_lowbd_fwd_txfm_sse2 is the RTCD specialization only on 32-bit x86; on x86-64 it is a
// plain function, and the 2D functions are reached from the SSE4.1 and AVX2 entries.
//
// Translation notes:
// - Every function carries `sse2`, the flag upstream compiles the file with.
//   The RTCD specializations are exported under their upstream names.
// - Pointer and array parameters stay raw pointers; local arrays are Rust
//   arrays and start zeroed. DECLARE_ALIGNED arrays keep their alignment
//   through `Aligned16`/`Aligned32`.
// - C integer conversions are explicit (`as`); `assert` becomes
//   `debug_assert!`. Immediate operands become const generics; shift counts
//   that are run-time `int` values use the count-register intrinsics, which is
//   what GCC and Clang emit for a non-constant count.
// - Scalar accesses through cast pointers (`*(int *)p`) are unaligned reads
//   and writes, as x86 compilers emit them.
#![allow(
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut,
    unused_parens
)]
#![allow(unused_variables, dead_code, clippy::all)]

use crate::av1__common__idct::TxfmParam;
use crate::av1__encoder__av1_fwd_txfm2d::*;
use crate::simd::aom_dsp__x86__transpose_sse2::*;
use crate::simd::aom_dsp__x86__txfm_common_sse2::*;
use crate::simd::av1__common__x86__av1_inv_txfm_ssse3::{
    cospi_arr, get_flip_cfg, get_rect_tx_log_ratio, get_txh_idx, get_txw_idx, sinpi_arr,
    tx_size_high, tx_size_wide, NewInvSqrt2, NewSqrt2, NewSqrt2Bits, ADST_ADST, ADST_DCT,
    ADST_FLIPADST, AOMMIN, DCT_ADST, DCT_DCT, DCT_FLIPADST, FLIPADST_ADST, FLIPADST_DCT,
    FLIPADST_FLIPADST, H_ADST, H_DCT, H_FLIPADST, IDTX, TX_16X16, TX_16X32, TX_16X4, TX_16X64,
    TX_16X8, TX_32X16, TX_32X32, TX_32X64, TX_32X8, TX_4X16, TX_4X4, TX_4X8, TX_64X16, TX_64X32,
    TX_64X64, TX_8X16, TX_8X32, TX_8X4, TX_8X8, TX_SIZES, TX_SIZES_ALL, TX_TYPES, V_ADST, V_DCT,
    V_FLIPADST,
};
use crate::simd::av1__common__x86__av1_txfm_sse2::*;
use core::arch::x86_64::*;

// av1/common/enums.h
type TX_SIZE = u8;
type TX_TYPE = u8;

extern "C-unwind" {
    // av1/encoder/hybrid_fwd_txfm.c, declared with this crate's TxfmParam as the Neon
    // translation does.
    fn av1_lowbd_fwd_txfm_c(
        src_diff: *const i16,
        coeff: *mut i32,
        diff_stride: i32,
        txfm_param: *mut TxfmParam,
    );
}

/// transform_1d_sse2 (av1/common/x86/av1_txfm_sse2.h).
pub type transform_1d_sse2 = Option<unsafe fn(*const __m128i, *mut __m128i, i8)>;

// av1/encoder/x86/av1_fwd_txfm_sse2.h

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn fidentity4x4_new_sse2(input: *const __m128i, output: *mut __m128i, cos_bit: i8) {
    let _ = cos_bit;
    let one: __m128i = _mm_set1_epi16(1);
    for i in 0..4 {
        let a: __m128i = _mm_unpacklo_epi16(*input.offset(i as isize), one);
        let b: __m128i = scale_round_sse2(a, NewSqrt2);
        *output.offset(i as isize) = _mm_packs_epi32(b, b);
    }
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn fidentity8x4_new_sse2(input: *const __m128i, output: *mut __m128i, cos_bit: i8) {
    let _ = cos_bit;
    let one: __m128i = _mm_set1_epi16(1);
    for i in 0..4 {
        let a_lo: __m128i = _mm_unpacklo_epi16(*input.offset(i as isize), one);
        let a_hi: __m128i = _mm_unpackhi_epi16(*input.offset(i as isize), one);
        let b_lo: __m128i = scale_round_sse2(a_lo, NewSqrt2);
        let b_hi: __m128i = scale_round_sse2(a_hi, NewSqrt2);
        *output.offset(i as isize) = _mm_packs_epi32(b_lo, b_hi);
    }
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn fidentity8x8_new_sse2(input: *const __m128i, output: *mut __m128i, cos_bit: i8) {
    let _ = cos_bit;
    *output = _mm_adds_epi16(*input, *input);
    *output.offset(1) = _mm_adds_epi16(*input.offset(1), *input.offset(1));
    *output.offset(2) = _mm_adds_epi16(*input.offset(2), *input.offset(2));
    *output.offset(3) = _mm_adds_epi16(*input.offset(3), *input.offset(3));
    *output.offset(4) = _mm_adds_epi16(*input.offset(4), *input.offset(4));
    *output.offset(5) = _mm_adds_epi16(*input.offset(5), *input.offset(5));
    *output.offset(6) = _mm_adds_epi16(*input.offset(6), *input.offset(6));
    *output.offset(7) = _mm_adds_epi16(*input.offset(7), *input.offset(7));
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn fdct8x8_new_sse2(input: *const __m128i, output: *mut __m128i, cos_bit: i8) {
    let mut cospi: *const i32 = cospi_arr(cos_bit as i32);
    let __rounding: __m128i = _mm_set1_epi32(1 << (cos_bit as i32 - 1));
    let cospi_m32_p32: __m128i = pair_set_epi16(-*cospi.offset(32), *cospi.offset(32));
    let cospi_p32_p32: __m128i = pair_set_epi16(*cospi.offset(32), *cospi.offset(32));
    let cospi_p32_m32: __m128i = pair_set_epi16(*cospi.offset(32), -*cospi.offset(32));
    let cospi_p48_p16: __m128i = pair_set_epi16(*cospi.offset(48), *cospi.offset(16));
    let cospi_m16_p48: __m128i = pair_set_epi16(-*cospi.offset(16), *cospi.offset(48));
    let cospi_p56_p08: __m128i = pair_set_epi16(*cospi.offset(56), *cospi.offset(8));
    let cospi_m08_p56: __m128i = pair_set_epi16(-*cospi.offset(8), *cospi.offset(56));
    let cospi_p24_p40: __m128i = pair_set_epi16(*cospi.offset(24), *cospi.offset(40));
    let cospi_m40_p24: __m128i = pair_set_epi16(-*cospi.offset(40), *cospi.offset(24));
    // stage 1
    let mut x1: [__m128i; 8] = core::mem::zeroed();
    x1[0] = _mm_adds_epi16(*input, *input.offset(7));
    x1[7] = _mm_subs_epi16(*input, *input.offset(7));
    x1[1] = _mm_adds_epi16(*input.offset(1), *input.offset(6));
    x1[6] = _mm_subs_epi16(*input.offset(1), *input.offset(6));
    x1[2] = _mm_adds_epi16(*input.offset(2), *input.offset(5));
    x1[5] = _mm_subs_epi16(*input.offset(2), *input.offset(5));
    x1[3] = _mm_adds_epi16(*input.offset(3), *input.offset(4));
    x1[4] = _mm_subs_epi16(*input.offset(3), *input.offset(4));
    // stage 2
    let mut x2: [__m128i; 8] = core::mem::zeroed();
    x2[0] = _mm_adds_epi16(x1[0], x1[3]);
    x2[3] = _mm_subs_epi16(x1[0], x1[3]);
    x2[1] = _mm_adds_epi16(x1[1], x1[2]);
    x2[2] = _mm_subs_epi16(x1[1], x1[2]);
    x2[4] = x1[4];
    btf_16_sse2!(
        cospi_m32_p32,
        cospi_p32_p32,
        x1[5],
        x1[6],
        x2[5],
        x2[6],
        __rounding,
        cos_bit
    );
    x2[7] = x1[7];
    // stage 3
    let mut x3: [__m128i; 8] = core::mem::zeroed();
    btf_16_sse2!(
        cospi_p32_p32,
        cospi_p32_m32,
        x2[0],
        x2[1],
        x3[0],
        x3[1],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p48_p16,
        cospi_m16_p48,
        x2[2],
        x2[3],
        x3[2],
        x3[3],
        __rounding,
        cos_bit
    );
    x3[4] = _mm_adds_epi16(x2[4], x2[5]);
    x3[5] = _mm_subs_epi16(x2[4], x2[5]);
    x3[6] = _mm_subs_epi16(x2[7], x2[6]);
    x3[7] = _mm_adds_epi16(x2[7], x2[6]);
    // stage 4 and 5
    *output = x3[0];
    *output.offset(4) = x3[1];
    *output.offset(2) = x3[2];
    *output.offset(6) = x3[3];
    btf_16_sse2!(
        cospi_p56_p08,
        cospi_m08_p56,
        x3[4],
        x3[7],
        *output.offset(1),
        *output.offset(7),
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p24_p40,
        cospi_m40_p24,
        x3[5],
        x3[6],
        *output.offset(5),
        *output.offset(3),
        __rounding,
        cos_bit
    );
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn fadst8x8_new_sse2(input: *const __m128i, output: *mut __m128i, cos_bit: i8) {
    let mut cospi: *const i32 = cospi_arr(cos_bit as i32);
    let __zero: __m128i = _mm_setzero_si128();
    let __rounding: __m128i = _mm_set1_epi32(1 << (cos_bit as i32 - 1));
    let cospi_p32_p32: __m128i = pair_set_epi16(*cospi.offset(32), *cospi.offset(32));
    let cospi_p32_m32: __m128i = pair_set_epi16(*cospi.offset(32), -*cospi.offset(32));
    let cospi_p16_p48: __m128i = pair_set_epi16(*cospi.offset(16), *cospi.offset(48));
    let cospi_p48_m16: __m128i = pair_set_epi16(*cospi.offset(48), -*cospi.offset(16));
    let cospi_m48_p16: __m128i = pair_set_epi16(-*cospi.offset(48), *cospi.offset(16));
    let cospi_p04_p60: __m128i = pair_set_epi16(*cospi.offset(4), *cospi.offset(60));
    let cospi_p60_m04: __m128i = pair_set_epi16(*cospi.offset(60), -*cospi.offset(4));
    let cospi_p20_p44: __m128i = pair_set_epi16(*cospi.offset(20), *cospi.offset(44));
    let cospi_p44_m20: __m128i = pair_set_epi16(*cospi.offset(44), -*cospi.offset(20));
    let cospi_p36_p28: __m128i = pair_set_epi16(*cospi.offset(36), *cospi.offset(28));
    let cospi_p28_m36: __m128i = pair_set_epi16(*cospi.offset(28), -*cospi.offset(36));
    let cospi_p52_p12: __m128i = pair_set_epi16(*cospi.offset(52), *cospi.offset(12));
    let cospi_p12_m52: __m128i = pair_set_epi16(*cospi.offset(12), -*cospi.offset(52));
    // stage 1
    let mut x1: [__m128i; 8] = core::mem::zeroed();
    x1[0] = *input;
    x1[1] = _mm_subs_epi16(__zero, *input.offset(7));
    x1[2] = _mm_subs_epi16(__zero, *input.offset(3));
    x1[3] = *input.offset(4);
    x1[4] = _mm_subs_epi16(__zero, *input.offset(1));
    x1[5] = *input.offset(6);
    x1[6] = *input.offset(2);
    x1[7] = _mm_subs_epi16(__zero, *input.offset(5));
    // stage 2
    let mut x2: [__m128i; 8] = core::mem::zeroed();
    x2[0] = x1[0];
    x2[1] = x1[1];
    btf_16_sse2!(
        cospi_p32_p32,
        cospi_p32_m32,
        x1[2],
        x1[3],
        x2[2],
        x2[3],
        __rounding,
        cos_bit
    );
    x2[4] = x1[4];
    x2[5] = x1[5];
    btf_16_sse2!(
        cospi_p32_p32,
        cospi_p32_m32,
        x1[6],
        x1[7],
        x2[6],
        x2[7],
        __rounding,
        cos_bit
    );
    // stage 3
    let mut x3: [__m128i; 8] = core::mem::zeroed();
    x3[0] = _mm_adds_epi16(x2[0], x2[2]);
    x3[2] = _mm_subs_epi16(x2[0], x2[2]);
    x3[1] = _mm_adds_epi16(x2[1], x2[3]);
    x3[3] = _mm_subs_epi16(x2[1], x2[3]);
    x3[4] = _mm_adds_epi16(x2[4], x2[6]);
    x3[6] = _mm_subs_epi16(x2[4], x2[6]);
    x3[5] = _mm_adds_epi16(x2[5], x2[7]);
    x3[7] = _mm_subs_epi16(x2[5], x2[7]);
    // stage 4
    let mut x4: [__m128i; 8] = core::mem::zeroed();
    x4[0] = x3[0];
    x4[1] = x3[1];
    x4[2] = x3[2];
    x4[3] = x3[3];
    btf_16_sse2!(
        cospi_p16_p48,
        cospi_p48_m16,
        x3[4],
        x3[5],
        x4[4],
        x4[5],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m48_p16,
        cospi_p16_p48,
        x3[6],
        x3[7],
        x4[6],
        x4[7],
        __rounding,
        cos_bit
    );
    // stage 5, 6 and 7
    *output.offset(7) = _mm_adds_epi16(x4[0], x4[4]);
    *output.offset(3) = _mm_subs_epi16(x4[0], x4[4]);
    *output = _mm_adds_epi16(x4[1], x4[5]);
    *output.offset(4) = _mm_subs_epi16(x4[1], x4[5]);
    *output.offset(5) = _mm_adds_epi16(x4[2], x4[6]);
    *output.offset(1) = _mm_subs_epi16(x4[2], x4[6]);
    *output.offset(2) = _mm_adds_epi16(x4[3], x4[7]);
    *output.offset(6) = _mm_subs_epi16(x4[3], x4[7]);
    btf_16_sse2!(
        cospi_p04_p60,
        cospi_p60_m04,
        *output.offset(7),
        *output,
        *output.offset(7),
        *output,
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p20_p44,
        cospi_p44_m20,
        *output.offset(5),
        *output.offset(2),
        *output.offset(5),
        *output.offset(2),
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p36_p28,
        cospi_p28_m36,
        *output.offset(3),
        *output.offset(4),
        *output.offset(3),
        *output.offset(4),
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p52_p12,
        cospi_p12_m52,
        *output.offset(1),
        *output.offset(6),
        *output.offset(1),
        *output.offset(6),
        __rounding,
        cos_bit
    );
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn fidentity8x16_new_sse2(input: *const __m128i, output: *mut __m128i, cos_bit: i8) {
    let _ = cos_bit;
    let one: __m128i = _mm_set1_epi16(1);
    for i in 0..16 {
        let a_lo: __m128i = _mm_unpacklo_epi16(*input.offset(i as isize), one);
        let a_hi: __m128i = _mm_unpackhi_epi16(*input.offset(i as isize), one);
        let b_lo: __m128i = scale_round_sse2(a_lo, 2 * NewSqrt2);
        let b_hi: __m128i = scale_round_sse2(a_hi, 2 * NewSqrt2);
        *output.offset(i as isize) = _mm_packs_epi32(b_lo, b_hi);
    }
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn fidentity8x32_new_sse2(input: *const __m128i, output: *mut __m128i, cos_bit: i8) {
    let _ = cos_bit;
    for i in 0..32 {
        *output.offset(i as isize) = _mm_slli_epi16::<{ 2 }>(*input.offset(i as isize));
    }
}

pub static col_txfm8x32_arr: [transform_1d_sse2; (TX_TYPES) as usize] = [
    Some(av1_fdct8x32_new_sse2),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(fidentity8x32_new_sse2),
    Some(av1_fdct8x32_new_sse2),
    Some(fidentity8x32_new_sse2),
    None,
    None,
    None,
    None,
];

// DCT_DCT
// ADST_DCT
// DCT_ADST
// ADST_ADST
// FLIPADST_DCT
// DCT_FLIPADST
// FLIPADST_FLIPADST
// ADST_FLIPADST
// FLIPADST_ADST
// IDTX
// V_DCT
// H_DCT
// V_ADST
// H_ADST
// V_FLIPADST
// H_FLIPADST
// AOM_AV1_ENCODER_X86_AV1_FWD_TXFM_SSE2_H_

// ---------------------------------------------------------------------------
// av1/encoder/x86/av1_fwd_txfm_sse2.c

// TODO(linfengz): refine fdct4x8 and fadst4x8 optimization (if possible).
#[inline]
#[target_feature(enable = "sse2")]
unsafe fn fdct4x4_new_sse2(input: *const __m128i, output: *mut __m128i, cos_bit: i8) {
    let mut cospi: *const i32 = cospi_arr(cos_bit as i32);
    let cospi_p32_p32: __m128i = pair_set_epi16(*cospi.offset(32), *cospi.offset(32));
    let cospi_p32_m32: __m128i = pair_set_epi16(*cospi.offset(32), -*cospi.offset(32));
    let cospi_p16_p48: __m128i = pair_set_epi16(*cospi.offset(16), *cospi.offset(48));
    let cospi_p48_m16: __m128i = pair_set_epi16(*cospi.offset(48), -*cospi.offset(16));
    let __rounding: __m128i = _mm_set1_epi32(1 << (cos_bit as i32 - 1));
    let mut u: [__m128i; 4] = core::mem::zeroed();
    let mut v: [__m128i; 4] = core::mem::zeroed();
    u[0] = _mm_unpacklo_epi16(*input, *input.offset(1));
    u[1] = _mm_unpacklo_epi16(*input.offset(3), *input.offset(2));
    v[0] = _mm_add_epi16(u[0], u[1]);
    v[1] = _mm_sub_epi16(u[0], u[1]);
    u[0] = _mm_madd_epi16(v[0], cospi_p32_p32); // 0
    u[1] = _mm_madd_epi16(v[0], cospi_p32_m32); // 2
    u[2] = _mm_madd_epi16(v[1], cospi_p16_p48); // 1
    u[3] = _mm_madd_epi16(v[1], cospi_p48_m16); // 3
    v[0] = _mm_add_epi32(u[0], __rounding);
    v[1] = _mm_add_epi32(u[1], __rounding);
    v[2] = _mm_add_epi32(u[2], __rounding);
    v[3] = _mm_add_epi32(u[3], __rounding);
    u[0] = _mm_sra_epi32(v[0], _mm_cvtsi32_si128(cos_bit as i32));
    u[1] = _mm_sra_epi32(v[1], _mm_cvtsi32_si128(cos_bit as i32));
    u[2] = _mm_sra_epi32(v[2], _mm_cvtsi32_si128(cos_bit as i32));
    u[3] = _mm_sra_epi32(v[3], _mm_cvtsi32_si128(cos_bit as i32));
    *output = _mm_packs_epi32(u[0], u[1]);
    *output.offset(1) = _mm_packs_epi32(u[2], u[3]);
    *output.offset(2) = _mm_srli_si128::<8>(*output);
    *output.offset(3) = _mm_srli_si128::<8>(*output.offset(1));
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn fdct8x4_new_sse2(input: *const __m128i, output: *mut __m128i, cos_bit: i8) {
    let mut cospi: *const i32 = cospi_arr(cos_bit as i32);
    let __rounding: __m128i = _mm_set1_epi32(1 << (cos_bit as i32 - 1));
    let mut cospi_p32_p32: __m128i = pair_set_epi16(*cospi.offset(32), *cospi.offset(32));
    let mut cospi_p32_m32: __m128i = pair_set_epi16(*cospi.offset(32), -*cospi.offset(32));
    let mut cospi_p48_p16: __m128i = pair_set_epi16(*cospi.offset(48), *cospi.offset(16));
    let mut cospi_m16_p48: __m128i = pair_set_epi16(-*cospi.offset(16), *cospi.offset(48));
    // stage 1
    let mut x1: [__m128i; 4] = core::mem::zeroed();
    x1[0] = _mm_adds_epi16(*input, *input.offset(3));
    x1[3] = _mm_subs_epi16(*input, *input.offset(3));
    x1[1] = _mm_adds_epi16(*input.offset(1), *input.offset(2));
    x1[2] = _mm_subs_epi16(*input.offset(1), *input.offset(2));
    // stage 2
    let mut x2: [__m128i; 4] = core::mem::zeroed();
    btf_16_sse2!(
        cospi_p32_p32,
        cospi_p32_m32,
        x1[0],
        x1[1],
        x2[0],
        x2[1],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p48_p16,
        cospi_m16_p48,
        x1[2],
        x1[3],
        x2[2],
        x2[3],
        __rounding,
        cos_bit
    );
    // stage 3
    *output = x2[0];
    *output.offset(1) = x2[2];
    *output.offset(2) = x2[1];
    *output.offset(3) = x2[3];
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn fdct4x8_new_sse2(input: *const __m128i, output: *mut __m128i, cos_bit: i8) {
    let mut cospi: *const i32 = cospi_arr(cos_bit as i32);
    let __rounding: __m128i = _mm_set1_epi32(1 << (cos_bit as i32 - 1));
    let mut cospi_m32_p32: __m128i = pair_set_epi16(-*cospi.offset(32), *cospi.offset(32));
    let mut cospi_p32_p32: __m128i = pair_set_epi16(*cospi.offset(32), *cospi.offset(32));
    let mut cospi_p32_m32: __m128i = pair_set_epi16(*cospi.offset(32), -*cospi.offset(32));
    let mut cospi_p48_p16: __m128i = pair_set_epi16(*cospi.offset(48), *cospi.offset(16));
    let mut cospi_m16_p48: __m128i = pair_set_epi16(-*cospi.offset(16), *cospi.offset(48));
    let mut cospi_p56_p08: __m128i = pair_set_epi16(*cospi.offset(56), *cospi.offset(8));
    let mut cospi_m08_p56: __m128i = pair_set_epi16(-*cospi.offset(8), *cospi.offset(56));
    let mut cospi_p24_p40: __m128i = pair_set_epi16(*cospi.offset(24), *cospi.offset(40));
    let mut cospi_m40_p24: __m128i = pair_set_epi16(-*cospi.offset(40), *cospi.offset(24));
    // stage 1
    let mut x1: [__m128i; 8] = core::mem::zeroed();
    x1[0] = _mm_adds_epi16(*input, *input.offset(7));
    x1[7] = _mm_subs_epi16(*input, *input.offset(7));
    x1[1] = _mm_adds_epi16(*input.offset(1), *input.offset(6));
    x1[6] = _mm_subs_epi16(*input.offset(1), *input.offset(6));
    x1[2] = _mm_adds_epi16(*input.offset(2), *input.offset(5));
    x1[5] = _mm_subs_epi16(*input.offset(2), *input.offset(5));
    x1[3] = _mm_adds_epi16(*input.offset(3), *input.offset(4));
    x1[4] = _mm_subs_epi16(*input.offset(3), *input.offset(4));
    // stage 2
    let mut x2: [__m128i; 8] = core::mem::zeroed();
    x2[0] = _mm_adds_epi16(x1[0], x1[3]);
    x2[3] = _mm_subs_epi16(x1[0], x1[3]);
    x2[1] = _mm_adds_epi16(x1[1], x1[2]);
    x2[2] = _mm_subs_epi16(x1[1], x1[2]);
    x2[4] = x1[4];
    btf_16_w4_sse2(
        &raw mut cospi_m32_p32 as *const __m128i,
        &raw mut cospi_p32_p32 as *const __m128i,
        __rounding,
        cos_bit,
        x1.as_mut_ptr().offset(5) as *const __m128i,
        x1.as_mut_ptr().offset(6) as *const __m128i,
        x2.as_mut_ptr().offset(5),
        x2.as_mut_ptr().offset(6),
    );
    x2[7] = x1[7];
    // stage 3
    let mut x3: [__m128i; 8] = core::mem::zeroed();
    btf_16_w4_sse2(
        &raw mut cospi_p32_p32 as *const __m128i,
        &raw mut cospi_p32_m32 as *const __m128i,
        __rounding,
        cos_bit,
        x2.as_mut_ptr() as *const __m128i,
        x2.as_mut_ptr().offset(1) as *const __m128i,
        x3.as_mut_ptr(),
        x3.as_mut_ptr().offset(1),
    );
    btf_16_w4_sse2(
        &raw mut cospi_p48_p16 as *const __m128i,
        &raw mut cospi_m16_p48 as *const __m128i,
        __rounding,
        cos_bit,
        x2.as_mut_ptr().offset(2) as *const __m128i,
        x2.as_mut_ptr().offset(3) as *const __m128i,
        x3.as_mut_ptr().offset(2),
        x3.as_mut_ptr().offset(3),
    );
    x3[4] = _mm_adds_epi16(x2[4], x2[5]);
    x3[5] = _mm_subs_epi16(x2[4], x2[5]);
    x3[6] = _mm_subs_epi16(x2[7], x2[6]);
    x3[7] = _mm_adds_epi16(x2[7], x2[6]);
    // stage 4
    let mut x4: [__m128i; 8] = core::mem::zeroed();
    x4[0] = x3[0];
    x4[1] = x3[1];
    x4[2] = x3[2];
    x4[3] = x3[3];
    btf_16_w4_sse2(
        &raw mut cospi_p56_p08 as *const __m128i,
        &raw mut cospi_m08_p56 as *const __m128i,
        __rounding,
        cos_bit,
        x3.as_mut_ptr().offset(4) as *const __m128i,
        x3.as_mut_ptr().offset(7) as *const __m128i,
        x4.as_mut_ptr().offset(4),
        x4.as_mut_ptr().offset(7),
    );
    btf_16_w4_sse2(
        &raw mut cospi_p24_p40 as *const __m128i,
        &raw mut cospi_m40_p24 as *const __m128i,
        __rounding,
        cos_bit,
        x3.as_mut_ptr().offset(5) as *const __m128i,
        x3.as_mut_ptr().offset(6) as *const __m128i,
        x4.as_mut_ptr().offset(5),
        x4.as_mut_ptr().offset(6),
    );
    // stage 5
    *output = x4[0];
    *output.offset(1) = x4[4];
    *output.offset(2) = x4[2];
    *output.offset(3) = x4[6];
    *output.offset(4) = x4[1];
    *output.offset(5) = x4[5];
    *output.offset(6) = x4[3];
    *output.offset(7) = x4[7];
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn fdct8x16_new_sse2(input: *const __m128i, output: *mut __m128i, cos_bit: i8) {
    let mut cospi: *const i32 = cospi_arr(cos_bit as i32);
    let __rounding: __m128i = _mm_set1_epi32(1 << (cos_bit as i32 - 1));
    let mut cospi_m32_p32: __m128i = pair_set_epi16(-*cospi.offset(32), *cospi.offset(32));
    let mut cospi_p32_p32: __m128i = pair_set_epi16(*cospi.offset(32), *cospi.offset(32));
    let mut cospi_p32_m32: __m128i = pair_set_epi16(*cospi.offset(32), -*cospi.offset(32));
    let mut cospi_p48_p16: __m128i = pair_set_epi16(*cospi.offset(48), *cospi.offset(16));
    let mut cospi_m16_p48: __m128i = pair_set_epi16(-*cospi.offset(16), *cospi.offset(48));
    let mut cospi_m48_m16: __m128i = pair_set_epi16(-*cospi.offset(48), -*cospi.offset(16));
    let mut cospi_p56_p08: __m128i = pair_set_epi16(*cospi.offset(56), *cospi.offset(8));
    let mut cospi_m08_p56: __m128i = pair_set_epi16(-*cospi.offset(8), *cospi.offset(56));
    let mut cospi_p24_p40: __m128i = pair_set_epi16(*cospi.offset(24), *cospi.offset(40));
    let mut cospi_m40_p24: __m128i = pair_set_epi16(-*cospi.offset(40), *cospi.offset(24));
    let mut cospi_p60_p04: __m128i = pair_set_epi16(*cospi.offset(60), *cospi.offset(4));
    let mut cospi_m04_p60: __m128i = pair_set_epi16(-*cospi.offset(4), *cospi.offset(60));
    let mut cospi_p28_p36: __m128i = pair_set_epi16(*cospi.offset(28), *cospi.offset(36));
    let mut cospi_m36_p28: __m128i = pair_set_epi16(-*cospi.offset(36), *cospi.offset(28));
    let mut cospi_p44_p20: __m128i = pair_set_epi16(*cospi.offset(44), *cospi.offset(20));
    let mut cospi_m20_p44: __m128i = pair_set_epi16(-*cospi.offset(20), *cospi.offset(44));
    let mut cospi_p12_p52: __m128i = pair_set_epi16(*cospi.offset(12), *cospi.offset(52));
    let mut cospi_m52_p12: __m128i = pair_set_epi16(-*cospi.offset(52), *cospi.offset(12));
    // stage 1
    let mut x1: [__m128i; 16] = core::mem::zeroed();
    x1[0] = _mm_adds_epi16(*input, *input.offset(15));
    x1[15] = _mm_subs_epi16(*input, *input.offset(15));
    x1[1] = _mm_adds_epi16(*input.offset(1), *input.offset(14));
    x1[14] = _mm_subs_epi16(*input.offset(1), *input.offset(14));
    x1[2] = _mm_adds_epi16(*input.offset(2), *input.offset(13));
    x1[13] = _mm_subs_epi16(*input.offset(2), *input.offset(13));
    x1[3] = _mm_adds_epi16(*input.offset(3), *input.offset(12));
    x1[12] = _mm_subs_epi16(*input.offset(3), *input.offset(12));
    x1[4] = _mm_adds_epi16(*input.offset(4), *input.offset(11));
    x1[11] = _mm_subs_epi16(*input.offset(4), *input.offset(11));
    x1[5] = _mm_adds_epi16(*input.offset(5), *input.offset(10));
    x1[10] = _mm_subs_epi16(*input.offset(5), *input.offset(10));
    x1[6] = _mm_adds_epi16(*input.offset(6), *input.offset(9));
    x1[9] = _mm_subs_epi16(*input.offset(6), *input.offset(9));
    x1[7] = _mm_adds_epi16(*input.offset(7), *input.offset(8));
    x1[8] = _mm_subs_epi16(*input.offset(7), *input.offset(8));
    // stage 2
    let mut x2: [__m128i; 16] = core::mem::zeroed();
    x2[0] = _mm_adds_epi16(x1[0], x1[7]);
    x2[7] = _mm_subs_epi16(x1[0], x1[7]);
    x2[1] = _mm_adds_epi16(x1[1], x1[6]);
    x2[6] = _mm_subs_epi16(x1[1], x1[6]);
    x2[2] = _mm_adds_epi16(x1[2], x1[5]);
    x2[5] = _mm_subs_epi16(x1[2], x1[5]);
    x2[3] = _mm_adds_epi16(x1[3], x1[4]);
    x2[4] = _mm_subs_epi16(x1[3], x1[4]);
    x2[8] = x1[8];
    x2[9] = x1[9];
    btf_16_sse2!(
        cospi_m32_p32,
        cospi_p32_p32,
        x1[10],
        x1[13],
        x2[10],
        x2[13],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m32_p32,
        cospi_p32_p32,
        x1[11],
        x1[12],
        x2[11],
        x2[12],
        __rounding,
        cos_bit
    );
    x2[14] = x1[14];
    x2[15] = x1[15];
    // stage 3
    let mut x3: [__m128i; 16] = core::mem::zeroed();
    x3[0] = _mm_adds_epi16(x2[0], x2[3]);
    x3[3] = _mm_subs_epi16(x2[0], x2[3]);
    x3[1] = _mm_adds_epi16(x2[1], x2[2]);
    x3[2] = _mm_subs_epi16(x2[1], x2[2]);
    x3[4] = x2[4];
    btf_16_sse2!(
        cospi_m32_p32,
        cospi_p32_p32,
        x2[5],
        x2[6],
        x3[5],
        x3[6],
        __rounding,
        cos_bit
    );
    x3[7] = x2[7];
    x3[8] = _mm_adds_epi16(x2[8], x2[11]);
    x3[11] = _mm_subs_epi16(x2[8], x2[11]);
    x3[9] = _mm_adds_epi16(x2[9], x2[10]);
    x3[10] = _mm_subs_epi16(x2[9], x2[10]);
    x3[12] = _mm_subs_epi16(x2[15], x2[12]);
    x3[15] = _mm_adds_epi16(x2[15], x2[12]);
    x3[13] = _mm_subs_epi16(x2[14], x2[13]);
    x3[14] = _mm_adds_epi16(x2[14], x2[13]);
    // stage 4
    let mut x4: [__m128i; 16] = core::mem::zeroed();
    btf_16_sse2!(
        cospi_p32_p32,
        cospi_p32_m32,
        x3[0],
        x3[1],
        x4[0],
        x4[1],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p48_p16,
        cospi_m16_p48,
        x3[2],
        x3[3],
        x4[2],
        x4[3],
        __rounding,
        cos_bit
    );
    x4[4] = _mm_adds_epi16(x3[4], x3[5]);
    x4[5] = _mm_subs_epi16(x3[4], x3[5]);
    x4[6] = _mm_subs_epi16(x3[7], x3[6]);
    x4[7] = _mm_adds_epi16(x3[7], x3[6]);
    x4[8] = x3[8];
    btf_16_sse2!(
        cospi_m16_p48,
        cospi_p48_p16,
        x3[9],
        x3[14],
        x4[9],
        x4[14],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m48_m16,
        cospi_m16_p48,
        x3[10],
        x3[13],
        x4[10],
        x4[13],
        __rounding,
        cos_bit
    );
    x4[11] = x3[11];
    x4[12] = x3[12];
    x4[15] = x3[15];
    // stage 5
    let mut x5: [__m128i; 16] = core::mem::zeroed();
    x5[0] = x4[0];
    x5[1] = x4[1];
    x5[2] = x4[2];
    x5[3] = x4[3];
    btf_16_sse2!(
        cospi_p56_p08,
        cospi_m08_p56,
        x4[4],
        x4[7],
        x5[4],
        x5[7],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p24_p40,
        cospi_m40_p24,
        x4[5],
        x4[6],
        x5[5],
        x5[6],
        __rounding,
        cos_bit
    );
    x5[8] = _mm_adds_epi16(x4[8], x4[9]);
    x5[9] = _mm_subs_epi16(x4[8], x4[9]);
    x5[10] = _mm_subs_epi16(x4[11], x4[10]);
    x5[11] = _mm_adds_epi16(x4[11], x4[10]);
    x5[12] = _mm_adds_epi16(x4[12], x4[13]);
    x5[13] = _mm_subs_epi16(x4[12], x4[13]);
    x5[14] = _mm_subs_epi16(x4[15], x4[14]);
    x5[15] = _mm_adds_epi16(x4[15], x4[14]);
    // stage 6
    let mut x6: [__m128i; 16] = core::mem::zeroed();
    x6[0] = x5[0];
    x6[1] = x5[1];
    x6[2] = x5[2];
    x6[3] = x5[3];
    x6[4] = x5[4];
    x6[5] = x5[5];
    x6[6] = x5[6];
    x6[7] = x5[7];
    btf_16_sse2!(
        cospi_p60_p04,
        cospi_m04_p60,
        x5[8],
        x5[15],
        x6[8],
        x6[15],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p28_p36,
        cospi_m36_p28,
        x5[9],
        x5[14],
        x6[9],
        x6[14],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p44_p20,
        cospi_m20_p44,
        x5[10],
        x5[13],
        x6[10],
        x6[13],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p12_p52,
        cospi_m52_p12,
        x5[11],
        x5[12],
        x6[11],
        x6[12],
        __rounding,
        cos_bit
    );
    // stage 7
    *output = x6[0];
    *output.offset(1) = x6[8];
    *output.offset(2) = x6[4];
    *output.offset(3) = x6[12];
    *output.offset(4) = x6[2];
    *output.offset(5) = x6[10];
    *output.offset(6) = x6[6];
    *output.offset(7) = x6[14];
    *output.offset(8) = x6[1];
    *output.offset(9) = x6[9];
    *output.offset(10) = x6[5];
    *output.offset(11) = x6[13];
    *output.offset(12) = x6[3];
    *output.offset(13) = x6[11];
    *output.offset(14) = x6[7];
    *output.offset(15) = x6[15];
}

#[target_feature(enable = "sse2")]
pub unsafe fn av1_fdct8x32_new_sse2(input: *const __m128i, output: *mut __m128i, cos_bit: i8) {
    let mut cospi: *const i32 = cospi_arr(cos_bit as i32);
    let __rounding: __m128i = _mm_set1_epi32(1 << (cos_bit as i32 - 1));
    let mut cospi_m32_p32: __m128i = pair_set_epi16(-*cospi.offset(32), *cospi.offset(32));
    let mut cospi_p32_p32: __m128i = pair_set_epi16(*cospi.offset(32), *cospi.offset(32));
    let mut cospi_m16_p48: __m128i = pair_set_epi16(-*cospi.offset(16), *cospi.offset(48));
    let mut cospi_p48_p16: __m128i = pair_set_epi16(*cospi.offset(48), *cospi.offset(16));
    let mut cospi_m48_m16: __m128i = pair_set_epi16(-*cospi.offset(48), -*cospi.offset(16));
    let mut cospi_p32_m32: __m128i = pair_set_epi16(*cospi.offset(32), -*cospi.offset(32));
    let mut cospi_p56_p08: __m128i = pair_set_epi16(*cospi.offset(56), *cospi.offset(8));
    let mut cospi_m08_p56: __m128i = pair_set_epi16(-*cospi.offset(8), *cospi.offset(56));
    let mut cospi_p24_p40: __m128i = pair_set_epi16(*cospi.offset(24), *cospi.offset(40));
    let mut cospi_m40_p24: __m128i = pair_set_epi16(-*cospi.offset(40), *cospi.offset(24));
    let mut cospi_m56_m08: __m128i = pair_set_epi16(-*cospi.offset(56), -*cospi.offset(8));
    let mut cospi_m24_m40: __m128i = pair_set_epi16(-*cospi.offset(24), -*cospi.offset(40));
    let mut cospi_p60_p04: __m128i = pair_set_epi16(*cospi.offset(60), *cospi.offset(4));
    let mut cospi_m04_p60: __m128i = pair_set_epi16(-*cospi.offset(4), *cospi.offset(60));
    let mut cospi_p28_p36: __m128i = pair_set_epi16(*cospi.offset(28), *cospi.offset(36));
    let mut cospi_m36_p28: __m128i = pair_set_epi16(-*cospi.offset(36), *cospi.offset(28));
    let mut cospi_p44_p20: __m128i = pair_set_epi16(*cospi.offset(44), *cospi.offset(20));
    let mut cospi_m20_p44: __m128i = pair_set_epi16(-*cospi.offset(20), *cospi.offset(44));
    let mut cospi_p12_p52: __m128i = pair_set_epi16(*cospi.offset(12), *cospi.offset(52));
    let mut cospi_m52_p12: __m128i = pair_set_epi16(-*cospi.offset(52), *cospi.offset(12));
    let mut cospi_p62_p02: __m128i = pair_set_epi16(*cospi.offset(62), *cospi.offset(2));
    let mut cospi_m02_p62: __m128i = pair_set_epi16(-*cospi.offset(2), *cospi.offset(62));
    let mut cospi_p30_p34: __m128i = pair_set_epi16(*cospi.offset(30), *cospi.offset(34));
    let mut cospi_m34_p30: __m128i = pair_set_epi16(-*cospi.offset(34), *cospi.offset(30));
    let mut cospi_p46_p18: __m128i = pair_set_epi16(*cospi.offset(46), *cospi.offset(18));
    let mut cospi_m18_p46: __m128i = pair_set_epi16(-*cospi.offset(18), *cospi.offset(46));
    let mut cospi_p14_p50: __m128i = pair_set_epi16(*cospi.offset(14), *cospi.offset(50));
    let mut cospi_m50_p14: __m128i = pair_set_epi16(-*cospi.offset(50), *cospi.offset(14));
    let mut cospi_p54_p10: __m128i = pair_set_epi16(*cospi.offset(54), *cospi.offset(10));
    let mut cospi_m10_p54: __m128i = pair_set_epi16(-*cospi.offset(10), *cospi.offset(54));
    let mut cospi_p22_p42: __m128i = pair_set_epi16(*cospi.offset(22), *cospi.offset(42));
    let mut cospi_m42_p22: __m128i = pair_set_epi16(-*cospi.offset(42), *cospi.offset(22));
    let mut cospi_p38_p26: __m128i = pair_set_epi16(*cospi.offset(38), *cospi.offset(26));
    let mut cospi_m26_p38: __m128i = pair_set_epi16(-*cospi.offset(26), *cospi.offset(38));
    let mut cospi_p06_p58: __m128i = pair_set_epi16(*cospi.offset(6), *cospi.offset(58));
    let mut cospi_m58_p06: __m128i = pair_set_epi16(-*cospi.offset(58), *cospi.offset(6));
    // stage 1
    let mut x1: [__m128i; 32] = core::mem::zeroed();
    x1[0] = _mm_adds_epi16(*input, *input.offset(31));
    x1[31] = _mm_subs_epi16(*input, *input.offset(31));
    x1[1] = _mm_adds_epi16(*input.offset(1), *input.offset(30));
    x1[30] = _mm_subs_epi16(*input.offset(1), *input.offset(30));
    x1[2] = _mm_adds_epi16(*input.offset(2), *input.offset(29));
    x1[29] = _mm_subs_epi16(*input.offset(2), *input.offset(29));
    x1[3] = _mm_adds_epi16(*input.offset(3), *input.offset(28));
    x1[28] = _mm_subs_epi16(*input.offset(3), *input.offset(28));
    x1[4] = _mm_adds_epi16(*input.offset(4), *input.offset(27));
    x1[27] = _mm_subs_epi16(*input.offset(4), *input.offset(27));
    x1[5] = _mm_adds_epi16(*input.offset(5), *input.offset(26));
    x1[26] = _mm_subs_epi16(*input.offset(5), *input.offset(26));
    x1[6] = _mm_adds_epi16(*input.offset(6), *input.offset(25));
    x1[25] = _mm_subs_epi16(*input.offset(6), *input.offset(25));
    x1[7] = _mm_adds_epi16(*input.offset(7), *input.offset(24));
    x1[24] = _mm_subs_epi16(*input.offset(7), *input.offset(24));
    x1[8] = _mm_adds_epi16(*input.offset(8), *input.offset(23));
    x1[23] = _mm_subs_epi16(*input.offset(8), *input.offset(23));
    x1[9] = _mm_adds_epi16(*input.offset(9), *input.offset(22));
    x1[22] = _mm_subs_epi16(*input.offset(9), *input.offset(22));
    x1[10] = _mm_adds_epi16(*input.offset(10), *input.offset(21));
    x1[21] = _mm_subs_epi16(*input.offset(10), *input.offset(21));
    x1[11] = _mm_adds_epi16(*input.offset(11), *input.offset(20));
    x1[20] = _mm_subs_epi16(*input.offset(11), *input.offset(20));
    x1[12] = _mm_adds_epi16(*input.offset(12), *input.offset(19));
    x1[19] = _mm_subs_epi16(*input.offset(12), *input.offset(19));
    x1[13] = _mm_adds_epi16(*input.offset(13), *input.offset(18));
    x1[18] = _mm_subs_epi16(*input.offset(13), *input.offset(18));
    x1[14] = _mm_adds_epi16(*input.offset(14), *input.offset(17));
    x1[17] = _mm_subs_epi16(*input.offset(14), *input.offset(17));
    x1[15] = _mm_adds_epi16(*input.offset(15), *input.offset(16));
    x1[16] = _mm_subs_epi16(*input.offset(15), *input.offset(16));
    // stage 2
    let mut x2: [__m128i; 32] = core::mem::zeroed();
    x2[0] = _mm_adds_epi16(x1[0], x1[15]);
    x2[15] = _mm_subs_epi16(x1[0], x1[15]);
    x2[1] = _mm_adds_epi16(x1[1], x1[14]);
    x2[14] = _mm_subs_epi16(x1[1], x1[14]);
    x2[2] = _mm_adds_epi16(x1[2], x1[13]);
    x2[13] = _mm_subs_epi16(x1[2], x1[13]);
    x2[3] = _mm_adds_epi16(x1[3], x1[12]);
    x2[12] = _mm_subs_epi16(x1[3], x1[12]);
    x2[4] = _mm_adds_epi16(x1[4], x1[11]);
    x2[11] = _mm_subs_epi16(x1[4], x1[11]);
    x2[5] = _mm_adds_epi16(x1[5], x1[10]);
    x2[10] = _mm_subs_epi16(x1[5], x1[10]);
    x2[6] = _mm_adds_epi16(x1[6], x1[9]);
    x2[9] = _mm_subs_epi16(x1[6], x1[9]);
    x2[7] = _mm_adds_epi16(x1[7], x1[8]);
    x2[8] = _mm_subs_epi16(x1[7], x1[8]);
    x2[16] = x1[16];
    x2[17] = x1[17];
    x2[18] = x1[18];
    x2[19] = x1[19];
    btf_16_sse2!(
        cospi_m32_p32,
        cospi_p32_p32,
        x1[20],
        x1[27],
        x2[20],
        x2[27],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m32_p32,
        cospi_p32_p32,
        x1[21],
        x1[26],
        x2[21],
        x2[26],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m32_p32,
        cospi_p32_p32,
        x1[22],
        x1[25],
        x2[22],
        x2[25],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m32_p32,
        cospi_p32_p32,
        x1[23],
        x1[24],
        x2[23],
        x2[24],
        __rounding,
        cos_bit
    );
    x2[28] = x1[28];
    x2[29] = x1[29];
    x2[30] = x1[30];
    x2[31] = x1[31];
    // stage 3
    let mut x3: [__m128i; 32] = core::mem::zeroed();
    x3[0] = _mm_adds_epi16(x2[0], x2[7]);
    x3[7] = _mm_subs_epi16(x2[0], x2[7]);
    x3[1] = _mm_adds_epi16(x2[1], x2[6]);
    x3[6] = _mm_subs_epi16(x2[1], x2[6]);
    x3[2] = _mm_adds_epi16(x2[2], x2[5]);
    x3[5] = _mm_subs_epi16(x2[2], x2[5]);
    x3[3] = _mm_adds_epi16(x2[3], x2[4]);
    x3[4] = _mm_subs_epi16(x2[3], x2[4]);
    x3[8] = x2[8];
    x3[9] = x2[9];
    btf_16_sse2!(
        cospi_m32_p32,
        cospi_p32_p32,
        x2[10],
        x2[13],
        x3[10],
        x3[13],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m32_p32,
        cospi_p32_p32,
        x2[11],
        x2[12],
        x3[11],
        x3[12],
        __rounding,
        cos_bit
    );
    x3[14] = x2[14];
    x3[15] = x2[15];
    x3[16] = _mm_adds_epi16(x2[16], x2[23]);
    x3[23] = _mm_subs_epi16(x2[16], x2[23]);
    x3[17] = _mm_adds_epi16(x2[17], x2[22]);
    x3[22] = _mm_subs_epi16(x2[17], x2[22]);
    x3[18] = _mm_adds_epi16(x2[18], x2[21]);
    x3[21] = _mm_subs_epi16(x2[18], x2[21]);
    x3[19] = _mm_adds_epi16(x2[19], x2[20]);
    x3[20] = _mm_subs_epi16(x2[19], x2[20]);
    x3[24] = _mm_subs_epi16(x2[31], x2[24]);
    x3[31] = _mm_adds_epi16(x2[31], x2[24]);
    x3[25] = _mm_subs_epi16(x2[30], x2[25]);
    x3[30] = _mm_adds_epi16(x2[30], x2[25]);
    x3[26] = _mm_subs_epi16(x2[29], x2[26]);
    x3[29] = _mm_adds_epi16(x2[29], x2[26]);
    x3[27] = _mm_subs_epi16(x2[28], x2[27]);
    x3[28] = _mm_adds_epi16(x2[28], x2[27]);
    // stage 4
    let mut x4: [__m128i; 32] = core::mem::zeroed();
    x4[0] = _mm_adds_epi16(x3[0], x3[3]);
    x4[3] = _mm_subs_epi16(x3[0], x3[3]);
    x4[1] = _mm_adds_epi16(x3[1], x3[2]);
    x4[2] = _mm_subs_epi16(x3[1], x3[2]);
    x4[4] = x3[4];
    btf_16_sse2!(
        cospi_m32_p32,
        cospi_p32_p32,
        x3[5],
        x3[6],
        x4[5],
        x4[6],
        __rounding,
        cos_bit
    );
    x4[7] = x3[7];
    x4[8] = _mm_adds_epi16(x3[8], x3[11]);
    x4[11] = _mm_subs_epi16(x3[8], x3[11]);
    x4[9] = _mm_adds_epi16(x3[9], x3[10]);
    x4[10] = _mm_subs_epi16(x3[9], x3[10]);
    x4[12] = _mm_subs_epi16(x3[15], x3[12]);
    x4[15] = _mm_adds_epi16(x3[15], x3[12]);
    x4[13] = _mm_subs_epi16(x3[14], x3[13]);
    x4[14] = _mm_adds_epi16(x3[14], x3[13]);
    x4[16] = x3[16];
    x4[17] = x3[17];
    btf_16_sse2!(
        cospi_m16_p48,
        cospi_p48_p16,
        x3[18],
        x3[29],
        x4[18],
        x4[29],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m16_p48,
        cospi_p48_p16,
        x3[19],
        x3[28],
        x4[19],
        x4[28],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m48_m16,
        cospi_m16_p48,
        x3[20],
        x3[27],
        x4[20],
        x4[27],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m48_m16,
        cospi_m16_p48,
        x3[21],
        x3[26],
        x4[21],
        x4[26],
        __rounding,
        cos_bit
    );
    x4[22] = x3[22];
    x4[23] = x3[23];
    x4[24] = x3[24];
    x4[25] = x3[25];
    x4[30] = x3[30];
    x4[31] = x3[31];
    // stage 5
    let mut x5: [__m128i; 32] = core::mem::zeroed();
    btf_16_sse2!(
        cospi_p32_p32,
        cospi_p32_m32,
        x4[0],
        x4[1],
        x5[0],
        x5[1],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p48_p16,
        cospi_m16_p48,
        x4[2],
        x4[3],
        x5[2],
        x5[3],
        __rounding,
        cos_bit
    );
    x5[4] = _mm_adds_epi16(x4[4], x4[5]);
    x5[5] = _mm_subs_epi16(x4[4], x4[5]);
    x5[6] = _mm_subs_epi16(x4[7], x4[6]);
    x5[7] = _mm_adds_epi16(x4[7], x4[6]);
    x5[8] = x4[8];
    btf_16_sse2!(
        cospi_m16_p48,
        cospi_p48_p16,
        x4[9],
        x4[14],
        x5[9],
        x5[14],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m48_m16,
        cospi_m16_p48,
        x4[10],
        x4[13],
        x5[10],
        x5[13],
        __rounding,
        cos_bit
    );
    x5[11] = x4[11];
    x5[12] = x4[12];
    x5[15] = x4[15];
    x5[16] = _mm_adds_epi16(x4[16], x4[19]);
    x5[19] = _mm_subs_epi16(x4[16], x4[19]);
    x5[17] = _mm_adds_epi16(x4[17], x4[18]);
    x5[18] = _mm_subs_epi16(x4[17], x4[18]);
    x5[20] = _mm_subs_epi16(x4[23], x4[20]);
    x5[23] = _mm_adds_epi16(x4[23], x4[20]);
    x5[21] = _mm_subs_epi16(x4[22], x4[21]);
    x5[22] = _mm_adds_epi16(x4[22], x4[21]);
    x5[24] = _mm_adds_epi16(x4[24], x4[27]);
    x5[27] = _mm_subs_epi16(x4[24], x4[27]);
    x5[25] = _mm_adds_epi16(x4[25], x4[26]);
    x5[26] = _mm_subs_epi16(x4[25], x4[26]);
    x5[28] = _mm_subs_epi16(x4[31], x4[28]);
    x5[31] = _mm_adds_epi16(x4[31], x4[28]);
    x5[29] = _mm_subs_epi16(x4[30], x4[29]);
    x5[30] = _mm_adds_epi16(x4[30], x4[29]);
    // stage 6
    let mut x6: [__m128i; 32] = core::mem::zeroed();
    x6[0] = x5[0];
    x6[1] = x5[1];
    x6[2] = x5[2];
    x6[3] = x5[3];
    btf_16_sse2!(
        cospi_p56_p08,
        cospi_m08_p56,
        x5[4],
        x5[7],
        x6[4],
        x6[7],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p24_p40,
        cospi_m40_p24,
        x5[5],
        x5[6],
        x6[5],
        x6[6],
        __rounding,
        cos_bit
    );
    x6[8] = _mm_adds_epi16(x5[8], x5[9]);
    x6[9] = _mm_subs_epi16(x5[8], x5[9]);
    x6[10] = _mm_subs_epi16(x5[11], x5[10]);
    x6[11] = _mm_adds_epi16(x5[11], x5[10]);
    x6[12] = _mm_adds_epi16(x5[12], x5[13]);
    x6[13] = _mm_subs_epi16(x5[12], x5[13]);
    x6[14] = _mm_subs_epi16(x5[15], x5[14]);
    x6[15] = _mm_adds_epi16(x5[15], x5[14]);
    x6[16] = x5[16];
    btf_16_sse2!(
        cospi_m08_p56,
        cospi_p56_p08,
        x5[17],
        x5[30],
        x6[17],
        x6[30],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m56_m08,
        cospi_m08_p56,
        x5[18],
        x5[29],
        x6[18],
        x6[29],
        __rounding,
        cos_bit
    );
    x6[19] = x5[19];
    x6[20] = x5[20];
    btf_16_sse2!(
        cospi_m40_p24,
        cospi_p24_p40,
        x5[21],
        x5[26],
        x6[21],
        x6[26],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m24_m40,
        cospi_m40_p24,
        x5[22],
        x5[25],
        x6[22],
        x6[25],
        __rounding,
        cos_bit
    );
    x6[23] = x5[23];
    x6[24] = x5[24];
    x6[27] = x5[27];
    x6[28] = x5[28];
    x6[31] = x5[31];
    // stage 7
    let mut x7: [__m128i; 32] = core::mem::zeroed();
    x7[0] = x6[0];
    x7[1] = x6[1];
    x7[2] = x6[2];
    x7[3] = x6[3];
    x7[4] = x6[4];
    x7[5] = x6[5];
    x7[6] = x6[6];
    x7[7] = x6[7];
    btf_16_sse2!(
        cospi_p60_p04,
        cospi_m04_p60,
        x6[8],
        x6[15],
        x7[8],
        x7[15],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p28_p36,
        cospi_m36_p28,
        x6[9],
        x6[14],
        x7[9],
        x7[14],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p44_p20,
        cospi_m20_p44,
        x6[10],
        x6[13],
        x7[10],
        x7[13],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p12_p52,
        cospi_m52_p12,
        x6[11],
        x6[12],
        x7[11],
        x7[12],
        __rounding,
        cos_bit
    );
    x7[16] = _mm_adds_epi16(x6[16], x6[17]);
    x7[17] = _mm_subs_epi16(x6[16], x6[17]);
    x7[18] = _mm_subs_epi16(x6[19], x6[18]);
    x7[19] = _mm_adds_epi16(x6[19], x6[18]);
    x7[20] = _mm_adds_epi16(x6[20], x6[21]);
    x7[21] = _mm_subs_epi16(x6[20], x6[21]);
    x7[22] = _mm_subs_epi16(x6[23], x6[22]);
    x7[23] = _mm_adds_epi16(x6[23], x6[22]);
    x7[24] = _mm_adds_epi16(x6[24], x6[25]);
    x7[25] = _mm_subs_epi16(x6[24], x6[25]);
    x7[26] = _mm_subs_epi16(x6[27], x6[26]);
    x7[27] = _mm_adds_epi16(x6[27], x6[26]);
    x7[28] = _mm_adds_epi16(x6[28], x6[29]);
    x7[29] = _mm_subs_epi16(x6[28], x6[29]);
    x7[30] = _mm_subs_epi16(x6[31], x6[30]);
    x7[31] = _mm_adds_epi16(x6[31], x6[30]);
    // stage 8
    let mut x8: [__m128i; 32] = core::mem::zeroed();
    x8[0] = x7[0];
    x8[1] = x7[1];
    x8[2] = x7[2];
    x8[3] = x7[3];
    x8[4] = x7[4];
    x8[5] = x7[5];
    x8[6] = x7[6];
    x8[7] = x7[7];
    x8[8] = x7[8];
    x8[9] = x7[9];
    x8[10] = x7[10];
    x8[11] = x7[11];
    x8[12] = x7[12];
    x8[13] = x7[13];
    x8[14] = x7[14];
    x8[15] = x7[15];
    btf_16_sse2!(
        cospi_p62_p02,
        cospi_m02_p62,
        x7[16],
        x7[31],
        x8[16],
        x8[31],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p30_p34,
        cospi_m34_p30,
        x7[17],
        x7[30],
        x8[17],
        x8[30],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p46_p18,
        cospi_m18_p46,
        x7[18],
        x7[29],
        x8[18],
        x8[29],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p14_p50,
        cospi_m50_p14,
        x7[19],
        x7[28],
        x8[19],
        x8[28],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p54_p10,
        cospi_m10_p54,
        x7[20],
        x7[27],
        x8[20],
        x8[27],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p22_p42,
        cospi_m42_p22,
        x7[21],
        x7[26],
        x8[21],
        x8[26],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p38_p26,
        cospi_m26_p38,
        x7[22],
        x7[25],
        x8[22],
        x8[25],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p06_p58,
        cospi_m58_p06,
        x7[23],
        x7[24],
        x8[23],
        x8[24],
        __rounding,
        cos_bit
    );
    // stage 9
    *output = x8[0];
    *output.offset(1) = x8[16];
    *output.offset(2) = x8[8];
    *output.offset(3) = x8[24];
    *output.offset(4) = x8[4];
    *output.offset(5) = x8[20];
    *output.offset(6) = x8[12];
    *output.offset(7) = x8[28];
    *output.offset(8) = x8[2];
    *output.offset(9) = x8[18];
    *output.offset(10) = x8[10];
    *output.offset(11) = x8[26];
    *output.offset(12) = x8[6];
    *output.offset(13) = x8[22];
    *output.offset(14) = x8[14];
    *output.offset(15) = x8[30];
    *output.offset(16) = x8[1];
    *output.offset(17) = x8[17];
    *output.offset(18) = x8[9];
    *output.offset(19) = x8[25];
    *output.offset(20) = x8[5];
    *output.offset(21) = x8[21];
    *output.offset(22) = x8[13];
    *output.offset(23) = x8[29];
    *output.offset(24) = x8[3];
    *output.offset(25) = x8[19];
    *output.offset(26) = x8[11];
    *output.offset(27) = x8[27];
    *output.offset(28) = x8[7];
    *output.offset(29) = x8[23];
    *output.offset(30) = x8[15];
    *output.offset(31) = x8[31];
}

#[target_feature(enable = "sse2")]
pub unsafe fn av1_fdct8x64_new_sse2(input: *const __m128i, output: *mut __m128i, cos_bit: i8) {
    let mut cospi: *const i32 = cospi_arr(cos_bit as i32);
    let __rounding: __m128i = _mm_set1_epi32(1 << (cos_bit as i32 - 1));
    let mut cospi_m32_p32: __m128i = pair_set_epi16(-*cospi.offset(32), *cospi.offset(32));
    let mut cospi_p32_p32: __m128i = pair_set_epi16(*cospi.offset(32), *cospi.offset(32));
    let mut cospi_m16_p48: __m128i = pair_set_epi16(-*cospi.offset(16), *cospi.offset(48));
    let mut cospi_p48_p16: __m128i = pair_set_epi16(*cospi.offset(48), *cospi.offset(16));
    let mut cospi_m48_m16: __m128i = pair_set_epi16(-*cospi.offset(48), -*cospi.offset(16));
    let mut cospi_p32_m32: __m128i = pair_set_epi16(*cospi.offset(32), -*cospi.offset(32));
    let mut cospi_m08_p56: __m128i = pair_set_epi16(-*cospi.offset(8), *cospi.offset(56));
    let mut cospi_p56_p08: __m128i = pair_set_epi16(*cospi.offset(56), *cospi.offset(8));
    let mut cospi_m56_m08: __m128i = pair_set_epi16(-*cospi.offset(56), -*cospi.offset(8));
    let mut cospi_m40_p24: __m128i = pair_set_epi16(-*cospi.offset(40), *cospi.offset(24));
    let mut cospi_p24_p40: __m128i = pair_set_epi16(*cospi.offset(24), *cospi.offset(40));
    let mut cospi_m24_m40: __m128i = pair_set_epi16(-*cospi.offset(24), -*cospi.offset(40));
    let mut cospi_p60_p04: __m128i = pair_set_epi16(*cospi.offset(60), *cospi.offset(4));
    let mut cospi_m04_p60: __m128i = pair_set_epi16(-*cospi.offset(4), *cospi.offset(60));
    let mut cospi_p28_p36: __m128i = pair_set_epi16(*cospi.offset(28), *cospi.offset(36));
    let mut cospi_m36_p28: __m128i = pair_set_epi16(-*cospi.offset(36), *cospi.offset(28));
    let mut cospi_p44_p20: __m128i = pair_set_epi16(*cospi.offset(44), *cospi.offset(20));
    let mut cospi_m20_p44: __m128i = pair_set_epi16(-*cospi.offset(20), *cospi.offset(44));
    let mut cospi_p12_p52: __m128i = pair_set_epi16(*cospi.offset(12), *cospi.offset(52));
    let mut cospi_m52_p12: __m128i = pair_set_epi16(-*cospi.offset(52), *cospi.offset(12));
    let mut cospi_m60_m04: __m128i = pair_set_epi16(-*cospi.offset(60), -*cospi.offset(4));
    let mut cospi_m28_m36: __m128i = pair_set_epi16(-*cospi.offset(28), -*cospi.offset(36));
    let mut cospi_m44_m20: __m128i = pair_set_epi16(-*cospi.offset(44), -*cospi.offset(20));
    let mut cospi_m12_m52: __m128i = pair_set_epi16(-*cospi.offset(12), -*cospi.offset(52));
    let mut cospi_p62_p02: __m128i = pair_set_epi16(*cospi.offset(62), *cospi.offset(2));
    let mut cospi_m02_p62: __m128i = pair_set_epi16(-*cospi.offset(2), *cospi.offset(62));
    let mut cospi_p30_p34: __m128i = pair_set_epi16(*cospi.offset(30), *cospi.offset(34));
    let mut cospi_m34_p30: __m128i = pair_set_epi16(-*cospi.offset(34), *cospi.offset(30));
    let mut cospi_p46_p18: __m128i = pair_set_epi16(*cospi.offset(46), *cospi.offset(18));
    let mut cospi_m18_p46: __m128i = pair_set_epi16(-*cospi.offset(18), *cospi.offset(46));
    let mut cospi_p14_p50: __m128i = pair_set_epi16(*cospi.offset(14), *cospi.offset(50));
    let mut cospi_m50_p14: __m128i = pair_set_epi16(-*cospi.offset(50), *cospi.offset(14));
    let mut cospi_p54_p10: __m128i = pair_set_epi16(*cospi.offset(54), *cospi.offset(10));
    let mut cospi_m10_p54: __m128i = pair_set_epi16(-*cospi.offset(10), *cospi.offset(54));
    let mut cospi_p22_p42: __m128i = pair_set_epi16(*cospi.offset(22), *cospi.offset(42));
    let mut cospi_m42_p22: __m128i = pair_set_epi16(-*cospi.offset(42), *cospi.offset(22));
    let mut cospi_p38_p26: __m128i = pair_set_epi16(*cospi.offset(38), *cospi.offset(26));
    let mut cospi_m26_p38: __m128i = pair_set_epi16(-*cospi.offset(26), *cospi.offset(38));
    let mut cospi_p06_p58: __m128i = pair_set_epi16(*cospi.offset(6), *cospi.offset(58));
    let mut cospi_m58_p06: __m128i = pair_set_epi16(-*cospi.offset(58), *cospi.offset(6));
    let mut cospi_p63_p01: __m128i = pair_set_epi16(*cospi.offset(63), *cospi.offset(1));
    let mut cospi_m01_p63: __m128i = pair_set_epi16(-*cospi.offset(1), *cospi.offset(63));
    let mut cospi_p31_p33: __m128i = pair_set_epi16(*cospi.offset(31), *cospi.offset(33));
    let mut cospi_m33_p31: __m128i = pair_set_epi16(-*cospi.offset(33), *cospi.offset(31));
    let mut cospi_p47_p17: __m128i = pair_set_epi16(*cospi.offset(47), *cospi.offset(17));
    let mut cospi_m17_p47: __m128i = pair_set_epi16(-*cospi.offset(17), *cospi.offset(47));
    let mut cospi_p15_p49: __m128i = pair_set_epi16(*cospi.offset(15), *cospi.offset(49));
    let mut cospi_m49_p15: __m128i = pair_set_epi16(-*cospi.offset(49), *cospi.offset(15));
    let mut cospi_p55_p09: __m128i = pair_set_epi16(*cospi.offset(55), *cospi.offset(9));
    let mut cospi_m09_p55: __m128i = pair_set_epi16(-*cospi.offset(9), *cospi.offset(55));
    let mut cospi_p23_p41: __m128i = pair_set_epi16(*cospi.offset(23), *cospi.offset(41));
    let mut cospi_m41_p23: __m128i = pair_set_epi16(-*cospi.offset(41), *cospi.offset(23));
    let mut cospi_p39_p25: __m128i = pair_set_epi16(*cospi.offset(39), *cospi.offset(25));
    let mut cospi_m25_p39: __m128i = pair_set_epi16(-*cospi.offset(25), *cospi.offset(39));
    let mut cospi_p07_p57: __m128i = pair_set_epi16(*cospi.offset(7), *cospi.offset(57));
    let mut cospi_m57_p07: __m128i = pair_set_epi16(-*cospi.offset(57), *cospi.offset(7));
    let mut cospi_p59_p05: __m128i = pair_set_epi16(*cospi.offset(59), *cospi.offset(5));
    let mut cospi_m05_p59: __m128i = pair_set_epi16(-*cospi.offset(5), *cospi.offset(59));
    let mut cospi_p27_p37: __m128i = pair_set_epi16(*cospi.offset(27), *cospi.offset(37));
    let mut cospi_m37_p27: __m128i = pair_set_epi16(-*cospi.offset(37), *cospi.offset(27));
    let mut cospi_p43_p21: __m128i = pair_set_epi16(*cospi.offset(43), *cospi.offset(21));
    let mut cospi_m21_p43: __m128i = pair_set_epi16(-*cospi.offset(21), *cospi.offset(43));
    let mut cospi_p11_p53: __m128i = pair_set_epi16(*cospi.offset(11), *cospi.offset(53));
    let mut cospi_m53_p11: __m128i = pair_set_epi16(-*cospi.offset(53), *cospi.offset(11));
    let mut cospi_p51_p13: __m128i = pair_set_epi16(*cospi.offset(51), *cospi.offset(13));
    let mut cospi_m13_p51: __m128i = pair_set_epi16(-*cospi.offset(13), *cospi.offset(51));
    let mut cospi_p19_p45: __m128i = pair_set_epi16(*cospi.offset(19), *cospi.offset(45));
    let mut cospi_m45_p19: __m128i = pair_set_epi16(-*cospi.offset(45), *cospi.offset(19));
    let mut cospi_p35_p29: __m128i = pair_set_epi16(*cospi.offset(35), *cospi.offset(29));
    let mut cospi_m29_p35: __m128i = pair_set_epi16(-*cospi.offset(29), *cospi.offset(35));
    let mut cospi_p03_p61: __m128i = pair_set_epi16(*cospi.offset(3), *cospi.offset(61));
    let mut cospi_m61_p03: __m128i = pair_set_epi16(-*cospi.offset(61), *cospi.offset(3));
    // stage 1
    let mut x1: [__m128i; 64] = core::mem::zeroed();
    x1[0] = _mm_adds_epi16(*input, *input.offset(63));
    x1[63] = _mm_subs_epi16(*input, *input.offset(63));
    x1[1] = _mm_adds_epi16(*input.offset(1), *input.offset(62));
    x1[62] = _mm_subs_epi16(*input.offset(1), *input.offset(62));
    x1[2] = _mm_adds_epi16(*input.offset(2), *input.offset(61));
    x1[61] = _mm_subs_epi16(*input.offset(2), *input.offset(61));
    x1[3] = _mm_adds_epi16(*input.offset(3), *input.offset(60));
    x1[60] = _mm_subs_epi16(*input.offset(3), *input.offset(60));
    x1[4] = _mm_adds_epi16(*input.offset(4), *input.offset(59));
    x1[59] = _mm_subs_epi16(*input.offset(4), *input.offset(59));
    x1[5] = _mm_adds_epi16(*input.offset(5), *input.offset(58));
    x1[58] = _mm_subs_epi16(*input.offset(5), *input.offset(58));
    x1[6] = _mm_adds_epi16(*input.offset(6), *input.offset(57));
    x1[57] = _mm_subs_epi16(*input.offset(6), *input.offset(57));
    x1[7] = _mm_adds_epi16(*input.offset(7), *input.offset(56));
    x1[56] = _mm_subs_epi16(*input.offset(7), *input.offset(56));
    x1[8] = _mm_adds_epi16(*input.offset(8), *input.offset(55));
    x1[55] = _mm_subs_epi16(*input.offset(8), *input.offset(55));
    x1[9] = _mm_adds_epi16(*input.offset(9), *input.offset(54));
    x1[54] = _mm_subs_epi16(*input.offset(9), *input.offset(54));
    x1[10] = _mm_adds_epi16(*input.offset(10), *input.offset(53));
    x1[53] = _mm_subs_epi16(*input.offset(10), *input.offset(53));
    x1[11] = _mm_adds_epi16(*input.offset(11), *input.offset(52));
    x1[52] = _mm_subs_epi16(*input.offset(11), *input.offset(52));
    x1[12] = _mm_adds_epi16(*input.offset(12), *input.offset(51));
    x1[51] = _mm_subs_epi16(*input.offset(12), *input.offset(51));
    x1[13] = _mm_adds_epi16(*input.offset(13), *input.offset(50));
    x1[50] = _mm_subs_epi16(*input.offset(13), *input.offset(50));
    x1[14] = _mm_adds_epi16(*input.offset(14), *input.offset(49));
    x1[49] = _mm_subs_epi16(*input.offset(14), *input.offset(49));
    x1[15] = _mm_adds_epi16(*input.offset(15), *input.offset(48));
    x1[48] = _mm_subs_epi16(*input.offset(15), *input.offset(48));
    x1[16] = _mm_adds_epi16(*input.offset(16), *input.offset(47));
    x1[47] = _mm_subs_epi16(*input.offset(16), *input.offset(47));
    x1[17] = _mm_adds_epi16(*input.offset(17), *input.offset(46));
    x1[46] = _mm_subs_epi16(*input.offset(17), *input.offset(46));
    x1[18] = _mm_adds_epi16(*input.offset(18), *input.offset(45));
    x1[45] = _mm_subs_epi16(*input.offset(18), *input.offset(45));
    x1[19] = _mm_adds_epi16(*input.offset(19), *input.offset(44));
    x1[44] = _mm_subs_epi16(*input.offset(19), *input.offset(44));
    x1[20] = _mm_adds_epi16(*input.offset(20), *input.offset(43));
    x1[43] = _mm_subs_epi16(*input.offset(20), *input.offset(43));
    x1[21] = _mm_adds_epi16(*input.offset(21), *input.offset(42));
    x1[42] = _mm_subs_epi16(*input.offset(21), *input.offset(42));
    x1[22] = _mm_adds_epi16(*input.offset(22), *input.offset(41));
    x1[41] = _mm_subs_epi16(*input.offset(22), *input.offset(41));
    x1[23] = _mm_adds_epi16(*input.offset(23), *input.offset(40));
    x1[40] = _mm_subs_epi16(*input.offset(23), *input.offset(40));
    x1[24] = _mm_adds_epi16(*input.offset(24), *input.offset(39));
    x1[39] = _mm_subs_epi16(*input.offset(24), *input.offset(39));
    x1[25] = _mm_adds_epi16(*input.offset(25), *input.offset(38));
    x1[38] = _mm_subs_epi16(*input.offset(25), *input.offset(38));
    x1[26] = _mm_adds_epi16(*input.offset(26), *input.offset(37));
    x1[37] = _mm_subs_epi16(*input.offset(26), *input.offset(37));
    x1[27] = _mm_adds_epi16(*input.offset(27), *input.offset(36));
    x1[36] = _mm_subs_epi16(*input.offset(27), *input.offset(36));
    x1[28] = _mm_adds_epi16(*input.offset(28), *input.offset(35));
    x1[35] = _mm_subs_epi16(*input.offset(28), *input.offset(35));
    x1[29] = _mm_adds_epi16(*input.offset(29), *input.offset(34));
    x1[34] = _mm_subs_epi16(*input.offset(29), *input.offset(34));
    x1[30] = _mm_adds_epi16(*input.offset(30), *input.offset(33));
    x1[33] = _mm_subs_epi16(*input.offset(30), *input.offset(33));
    x1[31] = _mm_adds_epi16(*input.offset(31), *input.offset(32));
    x1[32] = _mm_subs_epi16(*input.offset(31), *input.offset(32));
    // stage 2
    let mut x2: [__m128i; 64] = core::mem::zeroed();
    x2[0] = _mm_adds_epi16(x1[0], x1[31]);
    x2[31] = _mm_subs_epi16(x1[0], x1[31]);
    x2[1] = _mm_adds_epi16(x1[1], x1[30]);
    x2[30] = _mm_subs_epi16(x1[1], x1[30]);
    x2[2] = _mm_adds_epi16(x1[2], x1[29]);
    x2[29] = _mm_subs_epi16(x1[2], x1[29]);
    x2[3] = _mm_adds_epi16(x1[3], x1[28]);
    x2[28] = _mm_subs_epi16(x1[3], x1[28]);
    x2[4] = _mm_adds_epi16(x1[4], x1[27]);
    x2[27] = _mm_subs_epi16(x1[4], x1[27]);
    x2[5] = _mm_adds_epi16(x1[5], x1[26]);
    x2[26] = _mm_subs_epi16(x1[5], x1[26]);
    x2[6] = _mm_adds_epi16(x1[6], x1[25]);
    x2[25] = _mm_subs_epi16(x1[6], x1[25]);
    x2[7] = _mm_adds_epi16(x1[7], x1[24]);
    x2[24] = _mm_subs_epi16(x1[7], x1[24]);
    x2[8] = _mm_adds_epi16(x1[8], x1[23]);
    x2[23] = _mm_subs_epi16(x1[8], x1[23]);
    x2[9] = _mm_adds_epi16(x1[9], x1[22]);
    x2[22] = _mm_subs_epi16(x1[9], x1[22]);
    x2[10] = _mm_adds_epi16(x1[10], x1[21]);
    x2[21] = _mm_subs_epi16(x1[10], x1[21]);
    x2[11] = _mm_adds_epi16(x1[11], x1[20]);
    x2[20] = _mm_subs_epi16(x1[11], x1[20]);
    x2[12] = _mm_adds_epi16(x1[12], x1[19]);
    x2[19] = _mm_subs_epi16(x1[12], x1[19]);
    x2[13] = _mm_adds_epi16(x1[13], x1[18]);
    x2[18] = _mm_subs_epi16(x1[13], x1[18]);
    x2[14] = _mm_adds_epi16(x1[14], x1[17]);
    x2[17] = _mm_subs_epi16(x1[14], x1[17]);
    x2[15] = _mm_adds_epi16(x1[15], x1[16]);
    x2[16] = _mm_subs_epi16(x1[15], x1[16]);
    x2[32] = x1[32];
    x2[33] = x1[33];
    x2[34] = x1[34];
    x2[35] = x1[35];
    x2[36] = x1[36];
    x2[37] = x1[37];
    x2[38] = x1[38];
    x2[39] = x1[39];
    btf_16_sse2!(
        cospi_m32_p32,
        cospi_p32_p32,
        x1[40],
        x1[55],
        x2[40],
        x2[55],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m32_p32,
        cospi_p32_p32,
        x1[41],
        x1[54],
        x2[41],
        x2[54],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m32_p32,
        cospi_p32_p32,
        x1[42],
        x1[53],
        x2[42],
        x2[53],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m32_p32,
        cospi_p32_p32,
        x1[43],
        x1[52],
        x2[43],
        x2[52],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m32_p32,
        cospi_p32_p32,
        x1[44],
        x1[51],
        x2[44],
        x2[51],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m32_p32,
        cospi_p32_p32,
        x1[45],
        x1[50],
        x2[45],
        x2[50],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m32_p32,
        cospi_p32_p32,
        x1[46],
        x1[49],
        x2[46],
        x2[49],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m32_p32,
        cospi_p32_p32,
        x1[47],
        x1[48],
        x2[47],
        x2[48],
        __rounding,
        cos_bit
    );
    x2[56] = x1[56];
    x2[57] = x1[57];
    x2[58] = x1[58];
    x2[59] = x1[59];
    x2[60] = x1[60];
    x2[61] = x1[61];
    x2[62] = x1[62];
    x2[63] = x1[63];
    // stage 3
    let mut x3: [__m128i; 64] = core::mem::zeroed();
    x3[0] = _mm_adds_epi16(x2[0], x2[15]);
    x3[15] = _mm_subs_epi16(x2[0], x2[15]);
    x3[1] = _mm_adds_epi16(x2[1], x2[14]);
    x3[14] = _mm_subs_epi16(x2[1], x2[14]);
    x3[2] = _mm_adds_epi16(x2[2], x2[13]);
    x3[13] = _mm_subs_epi16(x2[2], x2[13]);
    x3[3] = _mm_adds_epi16(x2[3], x2[12]);
    x3[12] = _mm_subs_epi16(x2[3], x2[12]);
    x3[4] = _mm_adds_epi16(x2[4], x2[11]);
    x3[11] = _mm_subs_epi16(x2[4], x2[11]);
    x3[5] = _mm_adds_epi16(x2[5], x2[10]);
    x3[10] = _mm_subs_epi16(x2[5], x2[10]);
    x3[6] = _mm_adds_epi16(x2[6], x2[9]);
    x3[9] = _mm_subs_epi16(x2[6], x2[9]);
    x3[7] = _mm_adds_epi16(x2[7], x2[8]);
    x3[8] = _mm_subs_epi16(x2[7], x2[8]);
    x3[16] = x2[16];
    x3[17] = x2[17];
    x3[18] = x2[18];
    x3[19] = x2[19];
    btf_16_sse2!(
        cospi_m32_p32,
        cospi_p32_p32,
        x2[20],
        x2[27],
        x3[20],
        x3[27],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m32_p32,
        cospi_p32_p32,
        x2[21],
        x2[26],
        x3[21],
        x3[26],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m32_p32,
        cospi_p32_p32,
        x2[22],
        x2[25],
        x3[22],
        x3[25],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m32_p32,
        cospi_p32_p32,
        x2[23],
        x2[24],
        x3[23],
        x3[24],
        __rounding,
        cos_bit
    );
    x3[28] = x2[28];
    x3[29] = x2[29];
    x3[30] = x2[30];
    x3[31] = x2[31];
    x3[32] = _mm_adds_epi16(x2[32], x2[47]);
    x3[47] = _mm_subs_epi16(x2[32], x2[47]);
    x3[33] = _mm_adds_epi16(x2[33], x2[46]);
    x3[46] = _mm_subs_epi16(x2[33], x2[46]);
    x3[34] = _mm_adds_epi16(x2[34], x2[45]);
    x3[45] = _mm_subs_epi16(x2[34], x2[45]);
    x3[35] = _mm_adds_epi16(x2[35], x2[44]);
    x3[44] = _mm_subs_epi16(x2[35], x2[44]);
    x3[36] = _mm_adds_epi16(x2[36], x2[43]);
    x3[43] = _mm_subs_epi16(x2[36], x2[43]);
    x3[37] = _mm_adds_epi16(x2[37], x2[42]);
    x3[42] = _mm_subs_epi16(x2[37], x2[42]);
    x3[38] = _mm_adds_epi16(x2[38], x2[41]);
    x3[41] = _mm_subs_epi16(x2[38], x2[41]);
    x3[39] = _mm_adds_epi16(x2[39], x2[40]);
    x3[40] = _mm_subs_epi16(x2[39], x2[40]);
    x3[48] = _mm_subs_epi16(x2[63], x2[48]);
    x3[63] = _mm_adds_epi16(x2[63], x2[48]);
    x3[49] = _mm_subs_epi16(x2[62], x2[49]);
    x3[62] = _mm_adds_epi16(x2[62], x2[49]);
    x3[50] = _mm_subs_epi16(x2[61], x2[50]);
    x3[61] = _mm_adds_epi16(x2[61], x2[50]);
    x3[51] = _mm_subs_epi16(x2[60], x2[51]);
    x3[60] = _mm_adds_epi16(x2[60], x2[51]);
    x3[52] = _mm_subs_epi16(x2[59], x2[52]);
    x3[59] = _mm_adds_epi16(x2[59], x2[52]);
    x3[53] = _mm_subs_epi16(x2[58], x2[53]);
    x3[58] = _mm_adds_epi16(x2[58], x2[53]);
    x3[54] = _mm_subs_epi16(x2[57], x2[54]);
    x3[57] = _mm_adds_epi16(x2[57], x2[54]);
    x3[55] = _mm_subs_epi16(x2[56], x2[55]);
    x3[56] = _mm_adds_epi16(x2[56], x2[55]);
    // stage 4
    let mut x4: [__m128i; 64] = core::mem::zeroed();
    x4[0] = _mm_adds_epi16(x3[0], x3[7]);
    x4[7] = _mm_subs_epi16(x3[0], x3[7]);
    x4[1] = _mm_adds_epi16(x3[1], x3[6]);
    x4[6] = _mm_subs_epi16(x3[1], x3[6]);
    x4[2] = _mm_adds_epi16(x3[2], x3[5]);
    x4[5] = _mm_subs_epi16(x3[2], x3[5]);
    x4[3] = _mm_adds_epi16(x3[3], x3[4]);
    x4[4] = _mm_subs_epi16(x3[3], x3[4]);
    x4[8] = x3[8];
    x4[9] = x3[9];
    btf_16_sse2!(
        cospi_m32_p32,
        cospi_p32_p32,
        x3[10],
        x3[13],
        x4[10],
        x4[13],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m32_p32,
        cospi_p32_p32,
        x3[11],
        x3[12],
        x4[11],
        x4[12],
        __rounding,
        cos_bit
    );
    x4[14] = x3[14];
    x4[15] = x3[15];
    x4[16] = _mm_adds_epi16(x3[16], x3[23]);
    x4[23] = _mm_subs_epi16(x3[16], x3[23]);
    x4[17] = _mm_adds_epi16(x3[17], x3[22]);
    x4[22] = _mm_subs_epi16(x3[17], x3[22]);
    x4[18] = _mm_adds_epi16(x3[18], x3[21]);
    x4[21] = _mm_subs_epi16(x3[18], x3[21]);
    x4[19] = _mm_adds_epi16(x3[19], x3[20]);
    x4[20] = _mm_subs_epi16(x3[19], x3[20]);
    x4[24] = _mm_subs_epi16(x3[31], x3[24]);
    x4[31] = _mm_adds_epi16(x3[31], x3[24]);
    x4[25] = _mm_subs_epi16(x3[30], x3[25]);
    x4[30] = _mm_adds_epi16(x3[30], x3[25]);
    x4[26] = _mm_subs_epi16(x3[29], x3[26]);
    x4[29] = _mm_adds_epi16(x3[29], x3[26]);
    x4[27] = _mm_subs_epi16(x3[28], x3[27]);
    x4[28] = _mm_adds_epi16(x3[28], x3[27]);
    x4[32] = x3[32];
    x4[33] = x3[33];
    x4[34] = x3[34];
    x4[35] = x3[35];
    btf_16_sse2!(
        cospi_m16_p48,
        cospi_p48_p16,
        x3[36],
        x3[59],
        x4[36],
        x4[59],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m16_p48,
        cospi_p48_p16,
        x3[37],
        x3[58],
        x4[37],
        x4[58],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m16_p48,
        cospi_p48_p16,
        x3[38],
        x3[57],
        x4[38],
        x4[57],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m16_p48,
        cospi_p48_p16,
        x3[39],
        x3[56],
        x4[39],
        x4[56],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m48_m16,
        cospi_m16_p48,
        x3[40],
        x3[55],
        x4[40],
        x4[55],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m48_m16,
        cospi_m16_p48,
        x3[41],
        x3[54],
        x4[41],
        x4[54],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m48_m16,
        cospi_m16_p48,
        x3[42],
        x3[53],
        x4[42],
        x4[53],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m48_m16,
        cospi_m16_p48,
        x3[43],
        x3[52],
        x4[43],
        x4[52],
        __rounding,
        cos_bit
    );
    x4[44] = x3[44];
    x4[45] = x3[45];
    x4[46] = x3[46];
    x4[47] = x3[47];
    x4[48] = x3[48];
    x4[49] = x3[49];
    x4[50] = x3[50];
    x4[51] = x3[51];
    x4[60] = x3[60];
    x4[61] = x3[61];
    x4[62] = x3[62];
    x4[63] = x3[63];
    // stage 5
    let mut x5: [__m128i; 64] = core::mem::zeroed();
    x5[0] = _mm_adds_epi16(x4[0], x4[3]);
    x5[3] = _mm_subs_epi16(x4[0], x4[3]);
    x5[1] = _mm_adds_epi16(x4[1], x4[2]);
    x5[2] = _mm_subs_epi16(x4[1], x4[2]);
    x5[4] = x4[4];
    btf_16_sse2!(
        cospi_m32_p32,
        cospi_p32_p32,
        x4[5],
        x4[6],
        x5[5],
        x5[6],
        __rounding,
        cos_bit
    );
    x5[7] = x4[7];
    x5[8] = _mm_adds_epi16(x4[8], x4[11]);
    x5[11] = _mm_subs_epi16(x4[8], x4[11]);
    x5[9] = _mm_adds_epi16(x4[9], x4[10]);
    x5[10] = _mm_subs_epi16(x4[9], x4[10]);
    x5[12] = _mm_subs_epi16(x4[15], x4[12]);
    x5[15] = _mm_adds_epi16(x4[15], x4[12]);
    x5[13] = _mm_subs_epi16(x4[14], x4[13]);
    x5[14] = _mm_adds_epi16(x4[14], x4[13]);
    x5[16] = x4[16];
    x5[17] = x4[17];
    btf_16_sse2!(
        cospi_m16_p48,
        cospi_p48_p16,
        x4[18],
        x4[29],
        x5[18],
        x5[29],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m16_p48,
        cospi_p48_p16,
        x4[19],
        x4[28],
        x5[19],
        x5[28],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m48_m16,
        cospi_m16_p48,
        x4[20],
        x4[27],
        x5[20],
        x5[27],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m48_m16,
        cospi_m16_p48,
        x4[21],
        x4[26],
        x5[21],
        x5[26],
        __rounding,
        cos_bit
    );
    x5[22] = x4[22];
    x5[23] = x4[23];
    x5[24] = x4[24];
    x5[25] = x4[25];
    x5[30] = x4[30];
    x5[31] = x4[31];
    x5[32] = _mm_adds_epi16(x4[32], x4[39]);
    x5[39] = _mm_subs_epi16(x4[32], x4[39]);
    x5[33] = _mm_adds_epi16(x4[33], x4[38]);
    x5[38] = _mm_subs_epi16(x4[33], x4[38]);
    x5[34] = _mm_adds_epi16(x4[34], x4[37]);
    x5[37] = _mm_subs_epi16(x4[34], x4[37]);
    x5[35] = _mm_adds_epi16(x4[35], x4[36]);
    x5[36] = _mm_subs_epi16(x4[35], x4[36]);
    x5[40] = _mm_subs_epi16(x4[47], x4[40]);
    x5[47] = _mm_adds_epi16(x4[47], x4[40]);
    x5[41] = _mm_subs_epi16(x4[46], x4[41]);
    x5[46] = _mm_adds_epi16(x4[46], x4[41]);
    x5[42] = _mm_subs_epi16(x4[45], x4[42]);
    x5[45] = _mm_adds_epi16(x4[45], x4[42]);
    x5[43] = _mm_subs_epi16(x4[44], x4[43]);
    x5[44] = _mm_adds_epi16(x4[44], x4[43]);
    x5[48] = _mm_adds_epi16(x4[48], x4[55]);
    x5[55] = _mm_subs_epi16(x4[48], x4[55]);
    x5[49] = _mm_adds_epi16(x4[49], x4[54]);
    x5[54] = _mm_subs_epi16(x4[49], x4[54]);
    x5[50] = _mm_adds_epi16(x4[50], x4[53]);
    x5[53] = _mm_subs_epi16(x4[50], x4[53]);
    x5[51] = _mm_adds_epi16(x4[51], x4[52]);
    x5[52] = _mm_subs_epi16(x4[51], x4[52]);
    x5[56] = _mm_subs_epi16(x4[63], x4[56]);
    x5[63] = _mm_adds_epi16(x4[63], x4[56]);
    x5[57] = _mm_subs_epi16(x4[62], x4[57]);
    x5[62] = _mm_adds_epi16(x4[62], x4[57]);
    x5[58] = _mm_subs_epi16(x4[61], x4[58]);
    x5[61] = _mm_adds_epi16(x4[61], x4[58]);
    x5[59] = _mm_subs_epi16(x4[60], x4[59]);
    x5[60] = _mm_adds_epi16(x4[60], x4[59]);
    // stage 6
    let mut x6: [__m128i; 64] = core::mem::zeroed();
    btf_16_sse2!(
        cospi_p32_p32,
        cospi_p32_m32,
        x5[0],
        x5[1],
        x6[0],
        x6[1],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p48_p16,
        cospi_m16_p48,
        x5[2],
        x5[3],
        x6[2],
        x6[3],
        __rounding,
        cos_bit
    );
    x6[4] = _mm_adds_epi16(x5[4], x5[5]);
    x6[5] = _mm_subs_epi16(x5[4], x5[5]);
    x6[6] = _mm_subs_epi16(x5[7], x5[6]);
    x6[7] = _mm_adds_epi16(x5[7], x5[6]);
    x6[8] = x5[8];
    btf_16_sse2!(
        cospi_m16_p48,
        cospi_p48_p16,
        x5[9],
        x5[14],
        x6[9],
        x6[14],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m48_m16,
        cospi_m16_p48,
        x5[10],
        x5[13],
        x6[10],
        x6[13],
        __rounding,
        cos_bit
    );
    x6[11] = x5[11];
    x6[12] = x5[12];
    x6[15] = x5[15];
    x6[16] = _mm_adds_epi16(x5[16], x5[19]);
    x6[19] = _mm_subs_epi16(x5[16], x5[19]);
    x6[17] = _mm_adds_epi16(x5[17], x5[18]);
    x6[18] = _mm_subs_epi16(x5[17], x5[18]);
    x6[20] = _mm_subs_epi16(x5[23], x5[20]);
    x6[23] = _mm_adds_epi16(x5[23], x5[20]);
    x6[21] = _mm_subs_epi16(x5[22], x5[21]);
    x6[22] = _mm_adds_epi16(x5[22], x5[21]);
    x6[24] = _mm_adds_epi16(x5[24], x5[27]);
    x6[27] = _mm_subs_epi16(x5[24], x5[27]);
    x6[25] = _mm_adds_epi16(x5[25], x5[26]);
    x6[26] = _mm_subs_epi16(x5[25], x5[26]);
    x6[28] = _mm_subs_epi16(x5[31], x5[28]);
    x6[31] = _mm_adds_epi16(x5[31], x5[28]);
    x6[29] = _mm_subs_epi16(x5[30], x5[29]);
    x6[30] = _mm_adds_epi16(x5[30], x5[29]);
    x6[32] = x5[32];
    x6[33] = x5[33];
    btf_16_sse2!(
        cospi_m08_p56,
        cospi_p56_p08,
        x5[34],
        x5[61],
        x6[34],
        x6[61],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m08_p56,
        cospi_p56_p08,
        x5[35],
        x5[60],
        x6[35],
        x6[60],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m56_m08,
        cospi_m08_p56,
        x5[36],
        x5[59],
        x6[36],
        x6[59],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m56_m08,
        cospi_m08_p56,
        x5[37],
        x5[58],
        x6[37],
        x6[58],
        __rounding,
        cos_bit
    );
    x6[38] = x5[38];
    x6[39] = x5[39];
    x6[40] = x5[40];
    x6[41] = x5[41];
    btf_16_sse2!(
        cospi_m40_p24,
        cospi_p24_p40,
        x5[42],
        x5[53],
        x6[42],
        x6[53],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m40_p24,
        cospi_p24_p40,
        x5[43],
        x5[52],
        x6[43],
        x6[52],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m24_m40,
        cospi_m40_p24,
        x5[44],
        x5[51],
        x6[44],
        x6[51],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m24_m40,
        cospi_m40_p24,
        x5[45],
        x5[50],
        x6[45],
        x6[50],
        __rounding,
        cos_bit
    );
    x6[46] = x5[46];
    x6[47] = x5[47];
    x6[48] = x5[48];
    x6[49] = x5[49];
    x6[54] = x5[54];
    x6[55] = x5[55];
    x6[56] = x5[56];
    x6[57] = x5[57];
    x6[62] = x5[62];
    x6[63] = x5[63];
    // stage 7
    let mut x7: [__m128i; 64] = core::mem::zeroed();
    x7[0] = x6[0];
    x7[1] = x6[1];
    x7[2] = x6[2];
    x7[3] = x6[3];
    btf_16_sse2!(
        cospi_p56_p08,
        cospi_m08_p56,
        x6[4],
        x6[7],
        x7[4],
        x7[7],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p24_p40,
        cospi_m40_p24,
        x6[5],
        x6[6],
        x7[5],
        x7[6],
        __rounding,
        cos_bit
    );
    x7[8] = _mm_adds_epi16(x6[8], x6[9]);
    x7[9] = _mm_subs_epi16(x6[8], x6[9]);
    x7[10] = _mm_subs_epi16(x6[11], x6[10]);
    x7[11] = _mm_adds_epi16(x6[11], x6[10]);
    x7[12] = _mm_adds_epi16(x6[12], x6[13]);
    x7[13] = _mm_subs_epi16(x6[12], x6[13]);
    x7[14] = _mm_subs_epi16(x6[15], x6[14]);
    x7[15] = _mm_adds_epi16(x6[15], x6[14]);
    x7[16] = x6[16];
    btf_16_sse2!(
        cospi_m08_p56,
        cospi_p56_p08,
        x6[17],
        x6[30],
        x7[17],
        x7[30],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m56_m08,
        cospi_m08_p56,
        x6[18],
        x6[29],
        x7[18],
        x7[29],
        __rounding,
        cos_bit
    );
    x7[19] = x6[19];
    x7[20] = x6[20];
    btf_16_sse2!(
        cospi_m40_p24,
        cospi_p24_p40,
        x6[21],
        x6[26],
        x7[21],
        x7[26],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m24_m40,
        cospi_m40_p24,
        x6[22],
        x6[25],
        x7[22],
        x7[25],
        __rounding,
        cos_bit
    );
    x7[23] = x6[23];
    x7[24] = x6[24];
    x7[27] = x6[27];
    x7[28] = x6[28];
    x7[31] = x6[31];
    x7[32] = _mm_adds_epi16(x6[32], x6[35]);
    x7[35] = _mm_subs_epi16(x6[32], x6[35]);
    x7[33] = _mm_adds_epi16(x6[33], x6[34]);
    x7[34] = _mm_subs_epi16(x6[33], x6[34]);
    x7[36] = _mm_subs_epi16(x6[39], x6[36]);
    x7[39] = _mm_adds_epi16(x6[39], x6[36]);
    x7[37] = _mm_subs_epi16(x6[38], x6[37]);
    x7[38] = _mm_adds_epi16(x6[38], x6[37]);
    x7[40] = _mm_adds_epi16(x6[40], x6[43]);
    x7[43] = _mm_subs_epi16(x6[40], x6[43]);
    x7[41] = _mm_adds_epi16(x6[41], x6[42]);
    x7[42] = _mm_subs_epi16(x6[41], x6[42]);
    x7[44] = _mm_subs_epi16(x6[47], x6[44]);
    x7[47] = _mm_adds_epi16(x6[47], x6[44]);
    x7[45] = _mm_subs_epi16(x6[46], x6[45]);
    x7[46] = _mm_adds_epi16(x6[46], x6[45]);
    x7[48] = _mm_adds_epi16(x6[48], x6[51]);
    x7[51] = _mm_subs_epi16(x6[48], x6[51]);
    x7[49] = _mm_adds_epi16(x6[49], x6[50]);
    x7[50] = _mm_subs_epi16(x6[49], x6[50]);
    x7[52] = _mm_subs_epi16(x6[55], x6[52]);
    x7[55] = _mm_adds_epi16(x6[55], x6[52]);
    x7[53] = _mm_subs_epi16(x6[54], x6[53]);
    x7[54] = _mm_adds_epi16(x6[54], x6[53]);
    x7[56] = _mm_adds_epi16(x6[56], x6[59]);
    x7[59] = _mm_subs_epi16(x6[56], x6[59]);
    x7[57] = _mm_adds_epi16(x6[57], x6[58]);
    x7[58] = _mm_subs_epi16(x6[57], x6[58]);
    x7[60] = _mm_subs_epi16(x6[63], x6[60]);
    x7[63] = _mm_adds_epi16(x6[63], x6[60]);
    x7[61] = _mm_subs_epi16(x6[62], x6[61]);
    x7[62] = _mm_adds_epi16(x6[62], x6[61]);
    // stage 8
    let mut x8: [__m128i; 64] = core::mem::zeroed();
    x8[0] = x7[0];
    x8[1] = x7[1];
    x8[2] = x7[2];
    x8[3] = x7[3];
    x8[4] = x7[4];
    x8[5] = x7[5];
    x8[6] = x7[6];
    x8[7] = x7[7];
    btf_16_sse2!(
        cospi_p60_p04,
        cospi_m04_p60,
        x7[8],
        x7[15],
        x8[8],
        x8[15],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p28_p36,
        cospi_m36_p28,
        x7[9],
        x7[14],
        x8[9],
        x8[14],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p44_p20,
        cospi_m20_p44,
        x7[10],
        x7[13],
        x8[10],
        x8[13],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p12_p52,
        cospi_m52_p12,
        x7[11],
        x7[12],
        x8[11],
        x8[12],
        __rounding,
        cos_bit
    );
    x8[16] = _mm_adds_epi16(x7[16], x7[17]);
    x8[17] = _mm_subs_epi16(x7[16], x7[17]);
    x8[18] = _mm_subs_epi16(x7[19], x7[18]);
    x8[19] = _mm_adds_epi16(x7[19], x7[18]);
    x8[20] = _mm_adds_epi16(x7[20], x7[21]);
    x8[21] = _mm_subs_epi16(x7[20], x7[21]);
    x8[22] = _mm_subs_epi16(x7[23], x7[22]);
    x8[23] = _mm_adds_epi16(x7[23], x7[22]);
    x8[24] = _mm_adds_epi16(x7[24], x7[25]);
    x8[25] = _mm_subs_epi16(x7[24], x7[25]);
    x8[26] = _mm_subs_epi16(x7[27], x7[26]);
    x8[27] = _mm_adds_epi16(x7[27], x7[26]);
    x8[28] = _mm_adds_epi16(x7[28], x7[29]);
    x8[29] = _mm_subs_epi16(x7[28], x7[29]);
    x8[30] = _mm_subs_epi16(x7[31], x7[30]);
    x8[31] = _mm_adds_epi16(x7[31], x7[30]);
    x8[32] = x7[32];
    btf_16_sse2!(
        cospi_m04_p60,
        cospi_p60_p04,
        x7[33],
        x7[62],
        x8[33],
        x8[62],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m60_m04,
        cospi_m04_p60,
        x7[34],
        x7[61],
        x8[34],
        x8[61],
        __rounding,
        cos_bit
    );
    x8[35] = x7[35];
    x8[36] = x7[36];
    btf_16_sse2!(
        cospi_m36_p28,
        cospi_p28_p36,
        x7[37],
        x7[58],
        x8[37],
        x8[58],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m28_m36,
        cospi_m36_p28,
        x7[38],
        x7[57],
        x8[38],
        x8[57],
        __rounding,
        cos_bit
    );
    x8[39] = x7[39];
    x8[40] = x7[40];
    btf_16_sse2!(
        cospi_m20_p44,
        cospi_p44_p20,
        x7[41],
        x7[54],
        x8[41],
        x8[54],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m44_m20,
        cospi_m20_p44,
        x7[42],
        x7[53],
        x8[42],
        x8[53],
        __rounding,
        cos_bit
    );
    x8[43] = x7[43];
    x8[44] = x7[44];
    btf_16_sse2!(
        cospi_m52_p12,
        cospi_p12_p52,
        x7[45],
        x7[50],
        x8[45],
        x8[50],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m12_m52,
        cospi_m52_p12,
        x7[46],
        x7[49],
        x8[46],
        x8[49],
        __rounding,
        cos_bit
    );
    x8[47] = x7[47];
    x8[48] = x7[48];
    x8[51] = x7[51];
    x8[52] = x7[52];
    x8[55] = x7[55];
    x8[56] = x7[56];
    x8[59] = x7[59];
    x8[60] = x7[60];
    x8[63] = x7[63];
    // stage 9
    let mut x9: [__m128i; 64] = core::mem::zeroed();
    x9[0] = x8[0];
    x9[1] = x8[1];
    x9[2] = x8[2];
    x9[3] = x8[3];
    x9[4] = x8[4];
    x9[5] = x8[5];
    x9[6] = x8[6];
    x9[7] = x8[7];
    x9[8] = x8[8];
    x9[9] = x8[9];
    x9[10] = x8[10];
    x9[11] = x8[11];
    x9[12] = x8[12];
    x9[13] = x8[13];
    x9[14] = x8[14];
    x9[15] = x8[15];
    btf_16_sse2!(
        cospi_p62_p02,
        cospi_m02_p62,
        x8[16],
        x8[31],
        x9[16],
        x9[31],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p30_p34,
        cospi_m34_p30,
        x8[17],
        x8[30],
        x9[17],
        x9[30],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p46_p18,
        cospi_m18_p46,
        x8[18],
        x8[29],
        x9[18],
        x9[29],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p14_p50,
        cospi_m50_p14,
        x8[19],
        x8[28],
        x9[19],
        x9[28],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p54_p10,
        cospi_m10_p54,
        x8[20],
        x8[27],
        x9[20],
        x9[27],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p22_p42,
        cospi_m42_p22,
        x8[21],
        x8[26],
        x9[21],
        x9[26],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p38_p26,
        cospi_m26_p38,
        x8[22],
        x8[25],
        x9[22],
        x9[25],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p06_p58,
        cospi_m58_p06,
        x8[23],
        x8[24],
        x9[23],
        x9[24],
        __rounding,
        cos_bit
    );
    x9[32] = _mm_adds_epi16(x8[32], x8[33]);
    x9[33] = _mm_subs_epi16(x8[32], x8[33]);
    x9[34] = _mm_subs_epi16(x8[35], x8[34]);
    x9[35] = _mm_adds_epi16(x8[35], x8[34]);
    x9[36] = _mm_adds_epi16(x8[36], x8[37]);
    x9[37] = _mm_subs_epi16(x8[36], x8[37]);
    x9[38] = _mm_subs_epi16(x8[39], x8[38]);
    x9[39] = _mm_adds_epi16(x8[39], x8[38]);
    x9[40] = _mm_adds_epi16(x8[40], x8[41]);
    x9[41] = _mm_subs_epi16(x8[40], x8[41]);
    x9[42] = _mm_subs_epi16(x8[43], x8[42]);
    x9[43] = _mm_adds_epi16(x8[43], x8[42]);
    x9[44] = _mm_adds_epi16(x8[44], x8[45]);
    x9[45] = _mm_subs_epi16(x8[44], x8[45]);
    x9[46] = _mm_subs_epi16(x8[47], x8[46]);
    x9[47] = _mm_adds_epi16(x8[47], x8[46]);
    x9[48] = _mm_adds_epi16(x8[48], x8[49]);
    x9[49] = _mm_subs_epi16(x8[48], x8[49]);
    x9[50] = _mm_subs_epi16(x8[51], x8[50]);
    x9[51] = _mm_adds_epi16(x8[51], x8[50]);
    x9[52] = _mm_adds_epi16(x8[52], x8[53]);
    x9[53] = _mm_subs_epi16(x8[52], x8[53]);
    x9[54] = _mm_subs_epi16(x8[55], x8[54]);
    x9[55] = _mm_adds_epi16(x8[55], x8[54]);
    x9[56] = _mm_adds_epi16(x8[56], x8[57]);
    x9[57] = _mm_subs_epi16(x8[56], x8[57]);
    x9[58] = _mm_subs_epi16(x8[59], x8[58]);
    x9[59] = _mm_adds_epi16(x8[59], x8[58]);
    x9[60] = _mm_adds_epi16(x8[60], x8[61]);
    x9[61] = _mm_subs_epi16(x8[60], x8[61]);
    x9[62] = _mm_subs_epi16(x8[63], x8[62]);
    x9[63] = _mm_adds_epi16(x8[63], x8[62]);
    // stage 10
    let mut x10: [__m128i; 64] = core::mem::zeroed();
    x10[0] = x9[0];
    x10[1] = x9[1];
    x10[2] = x9[2];
    x10[3] = x9[3];
    x10[4] = x9[4];
    x10[5] = x9[5];
    x10[6] = x9[6];
    x10[7] = x9[7];
    x10[8] = x9[8];
    x10[9] = x9[9];
    x10[10] = x9[10];
    x10[11] = x9[11];
    x10[12] = x9[12];
    x10[13] = x9[13];
    x10[14] = x9[14];
    x10[15] = x9[15];
    x10[16] = x9[16];
    x10[17] = x9[17];
    x10[18] = x9[18];
    x10[19] = x9[19];
    x10[20] = x9[20];
    x10[21] = x9[21];
    x10[22] = x9[22];
    x10[23] = x9[23];
    x10[24] = x9[24];
    x10[25] = x9[25];
    x10[26] = x9[26];
    x10[27] = x9[27];
    x10[28] = x9[28];
    x10[29] = x9[29];
    x10[30] = x9[30];
    x10[31] = x9[31];
    btf_16_sse2!(
        cospi_p63_p01,
        cospi_m01_p63,
        x9[32],
        x9[63],
        x10[32],
        x10[63],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p31_p33,
        cospi_m33_p31,
        x9[33],
        x9[62],
        x10[33],
        x10[62],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p47_p17,
        cospi_m17_p47,
        x9[34],
        x9[61],
        x10[34],
        x10[61],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p15_p49,
        cospi_m49_p15,
        x9[35],
        x9[60],
        x10[35],
        x10[60],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p55_p09,
        cospi_m09_p55,
        x9[36],
        x9[59],
        x10[36],
        x10[59],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p23_p41,
        cospi_m41_p23,
        x9[37],
        x9[58],
        x10[37],
        x10[58],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p39_p25,
        cospi_m25_p39,
        x9[38],
        x9[57],
        x10[38],
        x10[57],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p07_p57,
        cospi_m57_p07,
        x9[39],
        x9[56],
        x10[39],
        x10[56],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p59_p05,
        cospi_m05_p59,
        x9[40],
        x9[55],
        x10[40],
        x10[55],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p27_p37,
        cospi_m37_p27,
        x9[41],
        x9[54],
        x10[41],
        x10[54],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p43_p21,
        cospi_m21_p43,
        x9[42],
        x9[53],
        x10[42],
        x10[53],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p11_p53,
        cospi_m53_p11,
        x9[43],
        x9[52],
        x10[43],
        x10[52],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p51_p13,
        cospi_m13_p51,
        x9[44],
        x9[51],
        x10[44],
        x10[51],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p19_p45,
        cospi_m45_p19,
        x9[45],
        x9[50],
        x10[45],
        x10[50],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p35_p29,
        cospi_m29_p35,
        x9[46],
        x9[49],
        x10[46],
        x10[49],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p03_p61,
        cospi_m61_p03,
        x9[47],
        x9[48],
        x10[47],
        x10[48],
        __rounding,
        cos_bit
    );
    // stage 11
    *output = x10[0];
    *output.offset(1) = x10[32];
    *output.offset(2) = x10[16];
    *output.offset(3) = x10[48];
    *output.offset(4) = x10[8];
    *output.offset(5) = x10[40];
    *output.offset(6) = x10[24];
    *output.offset(7) = x10[56];
    *output.offset(8) = x10[4];
    *output.offset(9) = x10[36];
    *output.offset(10) = x10[20];
    *output.offset(11) = x10[52];
    *output.offset(12) = x10[12];
    *output.offset(13) = x10[44];
    *output.offset(14) = x10[28];
    *output.offset(15) = x10[60];
    *output.offset(16) = x10[2];
    *output.offset(17) = x10[34];
    *output.offset(18) = x10[18];
    *output.offset(19) = x10[50];
    *output.offset(20) = x10[10];
    *output.offset(21) = x10[42];
    *output.offset(22) = x10[26];
    *output.offset(23) = x10[58];
    *output.offset(24) = x10[6];
    *output.offset(25) = x10[38];
    *output.offset(26) = x10[22];
    *output.offset(27) = x10[54];
    *output.offset(28) = x10[14];
    *output.offset(29) = x10[46];
    *output.offset(30) = x10[30];
    *output.offset(31) = x10[62];
    *output.offset(32) = x10[1];
    *output.offset(33) = x10[33];
    *output.offset(34) = x10[17];
    *output.offset(35) = x10[49];
    *output.offset(36) = x10[9];
    *output.offset(37) = x10[41];
    *output.offset(38) = x10[25];
    *output.offset(39) = x10[57];
    *output.offset(40) = x10[5];
    *output.offset(41) = x10[37];
    *output.offset(42) = x10[21];
    *output.offset(43) = x10[53];
    *output.offset(44) = x10[13];
    *output.offset(45) = x10[45];
    *output.offset(46) = x10[29];
    *output.offset(47) = x10[61];
    *output.offset(48) = x10[3];
    *output.offset(49) = x10[35];
    *output.offset(50) = x10[19];
    *output.offset(51) = x10[51];
    *output.offset(52) = x10[11];
    *output.offset(53) = x10[43];
    *output.offset(54) = x10[27];
    *output.offset(55) = x10[59];
    *output.offset(56) = x10[7];
    *output.offset(57) = x10[39];
    *output.offset(58) = x10[23];
    *output.offset(59) = x10[55];
    *output.offset(60) = x10[15];
    *output.offset(61) = x10[47];
    *output.offset(62) = x10[31];
    *output.offset(63) = x10[63];
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn fadst4x4_new_sse2(input: *const __m128i, output: *mut __m128i, cos_bit: i8) {
    let mut sinpi: *const i32 = sinpi_arr(cos_bit as i32);
    let sinpi_p01_p02: __m128i = pair_set_epi16(*sinpi.offset(1), *sinpi.offset(2));
    let sinpi_p04_m01: __m128i = pair_set_epi16(*sinpi.offset(4), -*sinpi.offset(1));
    let sinpi_p03_p04: __m128i = pair_set_epi16(*sinpi.offset(3), *sinpi.offset(4));
    let sinpi_m03_p02: __m128i = pair_set_epi16(-*sinpi.offset(3), *sinpi.offset(2));
    let sinpi_p03_p03: __m128i = _mm_set1_epi16(*sinpi.offset(3) as i16);
    let __zero: __m128i = _mm_setzero_si128();
    let __rounding: __m128i = _mm_set1_epi32(1 << (cos_bit as i32 - 1));
    let in7: __m128i = _mm_add_epi16(*input, *input.offset(1));
    let mut u: [__m128i; 8] = core::mem::zeroed();
    let mut v: [__m128i; 8] = core::mem::zeroed();
    u[0] = _mm_unpacklo_epi16(*input, *input.offset(1));
    u[1] = _mm_unpacklo_epi16(*input.offset(2), *input.offset(3));
    u[2] = _mm_unpacklo_epi16(in7, __zero);
    u[3] = _mm_unpacklo_epi16(*input.offset(2), __zero);
    u[4] = _mm_unpacklo_epi16(*input.offset(3), __zero);
    v[0] = _mm_madd_epi16(u[0], sinpi_p01_p02); // s0 + s2
    v[1] = _mm_madd_epi16(u[1], sinpi_p03_p04); // s4 + s5
    v[2] = _mm_madd_epi16(u[2], sinpi_p03_p03); // x1
    v[3] = _mm_madd_epi16(u[0], sinpi_p04_m01); // s1 - s3
    v[4] = _mm_madd_epi16(u[1], sinpi_m03_p02); // -s4 + s6
    v[5] = _mm_madd_epi16(u[3], sinpi_p03_p03); // s4
    v[6] = _mm_madd_epi16(u[4], sinpi_p03_p03);
    u[0] = _mm_add_epi32(v[0], v[1]);
    u[1] = _mm_sub_epi32(v[2], v[6]);
    u[2] = _mm_add_epi32(v[3], v[4]);
    u[3] = _mm_sub_epi32(u[2], u[0]);
    u[4] = _mm_slli_epi32::<{ 2 }>(v[5]);
    u[5] = _mm_sub_epi32(u[4], v[5]);
    u[6] = _mm_add_epi32(u[3], u[5]);
    v[0] = _mm_add_epi32(u[0], __rounding);
    v[1] = _mm_add_epi32(u[1], __rounding);
    v[2] = _mm_add_epi32(u[2], __rounding);
    v[3] = _mm_add_epi32(u[6], __rounding);
    u[0] = _mm_sra_epi32(v[0], _mm_cvtsi32_si128(cos_bit as i32));
    u[1] = _mm_sra_epi32(v[1], _mm_cvtsi32_si128(cos_bit as i32));
    u[2] = _mm_sra_epi32(v[2], _mm_cvtsi32_si128(cos_bit as i32));
    u[3] = _mm_sra_epi32(v[3], _mm_cvtsi32_si128(cos_bit as i32));
    *output = _mm_packs_epi32(u[0], u[2]);
    *output.offset(1) = _mm_packs_epi32(u[1], u[3]);
    *output.offset(2) = _mm_srli_si128::<8>(*output);
    *output.offset(3) = _mm_srli_si128::<8>(*output.offset(1));
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn fadst4x8_new_sse2(input: *const __m128i, output: *mut __m128i, cos_bit: i8) {
    let mut cospi: *const i32 = cospi_arr(cos_bit as i32);
    let __zero: __m128i = _mm_setzero_si128();
    let __rounding: __m128i = _mm_set1_epi32(1 << (cos_bit as i32 - 1));
    let mut cospi_p32_p32: __m128i = pair_set_epi16(*cospi.offset(32), *cospi.offset(32));
    let mut cospi_p32_m32: __m128i = pair_set_epi16(*cospi.offset(32), -*cospi.offset(32));
    let mut cospi_p16_p48: __m128i = pair_set_epi16(*cospi.offset(16), *cospi.offset(48));
    let mut cospi_p48_m16: __m128i = pair_set_epi16(*cospi.offset(48), -*cospi.offset(16));
    let mut cospi_m48_p16: __m128i = pair_set_epi16(-*cospi.offset(48), *cospi.offset(16));
    let mut cospi_p04_p60: __m128i = pair_set_epi16(*cospi.offset(4), *cospi.offset(60));
    let mut cospi_p60_m04: __m128i = pair_set_epi16(*cospi.offset(60), -*cospi.offset(4));
    let mut cospi_p20_p44: __m128i = pair_set_epi16(*cospi.offset(20), *cospi.offset(44));
    let mut cospi_p44_m20: __m128i = pair_set_epi16(*cospi.offset(44), -*cospi.offset(20));
    let mut cospi_p36_p28: __m128i = pair_set_epi16(*cospi.offset(36), *cospi.offset(28));
    let mut cospi_p28_m36: __m128i = pair_set_epi16(*cospi.offset(28), -*cospi.offset(36));
    let mut cospi_p52_p12: __m128i = pair_set_epi16(*cospi.offset(52), *cospi.offset(12));
    let mut cospi_p12_m52: __m128i = pair_set_epi16(*cospi.offset(12), -*cospi.offset(52));
    // stage 1
    let mut x1: [__m128i; 8] = core::mem::zeroed();
    x1[0] = *input;
    x1[1] = _mm_subs_epi16(__zero, *input.offset(7));
    x1[2] = _mm_subs_epi16(__zero, *input.offset(3));
    x1[3] = *input.offset(4);
    x1[4] = _mm_subs_epi16(__zero, *input.offset(1));
    x1[5] = *input.offset(6);
    x1[6] = *input.offset(2);
    x1[7] = _mm_subs_epi16(__zero, *input.offset(5));
    // stage 2
    let mut x2: [__m128i; 8] = core::mem::zeroed();
    x2[0] = x1[0];
    x2[1] = x1[1];
    btf_16_w4_sse2(
        &raw mut cospi_p32_p32 as *const __m128i,
        &raw mut cospi_p32_m32 as *const __m128i,
        __rounding,
        cos_bit,
        x1.as_mut_ptr().offset(2) as *const __m128i,
        x1.as_mut_ptr().offset(3) as *const __m128i,
        x2.as_mut_ptr().offset(2),
        x2.as_mut_ptr().offset(3),
    );
    x2[4] = x1[4];
    x2[5] = x1[5];
    btf_16_w4_sse2(
        &raw mut cospi_p32_p32 as *const __m128i,
        &raw mut cospi_p32_m32 as *const __m128i,
        __rounding,
        cos_bit,
        x1.as_mut_ptr().offset(6) as *const __m128i,
        x1.as_mut_ptr().offset(7) as *const __m128i,
        x2.as_mut_ptr().offset(6),
        x2.as_mut_ptr().offset(7),
    );
    // stage 3
    let mut x3: [__m128i; 8] = core::mem::zeroed();
    x3[0] = _mm_adds_epi16(x2[0], x2[2]);
    x3[2] = _mm_subs_epi16(x2[0], x2[2]);
    x3[1] = _mm_adds_epi16(x2[1], x2[3]);
    x3[3] = _mm_subs_epi16(x2[1], x2[3]);
    x3[4] = _mm_adds_epi16(x2[4], x2[6]);
    x3[6] = _mm_subs_epi16(x2[4], x2[6]);
    x3[5] = _mm_adds_epi16(x2[5], x2[7]);
    x3[7] = _mm_subs_epi16(x2[5], x2[7]);
    // stage 4
    let mut x4: [__m128i; 8] = core::mem::zeroed();
    x4[0] = x3[0];
    x4[1] = x3[1];
    x4[2] = x3[2];
    x4[3] = x3[3];
    btf_16_w4_sse2(
        &raw mut cospi_p16_p48 as *const __m128i,
        &raw mut cospi_p48_m16 as *const __m128i,
        __rounding,
        cos_bit,
        x3.as_mut_ptr().offset(4) as *const __m128i,
        x3.as_mut_ptr().offset(5) as *const __m128i,
        x4.as_mut_ptr().offset(4),
        x4.as_mut_ptr().offset(5),
    );
    btf_16_w4_sse2(
        &raw mut cospi_m48_p16 as *const __m128i,
        &raw mut cospi_p16_p48 as *const __m128i,
        __rounding,
        cos_bit,
        x3.as_mut_ptr().offset(6) as *const __m128i,
        x3.as_mut_ptr().offset(7) as *const __m128i,
        x4.as_mut_ptr().offset(6),
        x4.as_mut_ptr().offset(7),
    );
    // stage 5
    let mut x5: [__m128i; 8] = core::mem::zeroed();
    x5[0] = _mm_adds_epi16(x4[0], x4[4]);
    x5[4] = _mm_subs_epi16(x4[0], x4[4]);
    x5[1] = _mm_adds_epi16(x4[1], x4[5]);
    x5[5] = _mm_subs_epi16(x4[1], x4[5]);
    x5[2] = _mm_adds_epi16(x4[2], x4[6]);
    x5[6] = _mm_subs_epi16(x4[2], x4[6]);
    x5[3] = _mm_adds_epi16(x4[3], x4[7]);
    x5[7] = _mm_subs_epi16(x4[3], x4[7]);
    // stage 6
    let mut x6: [__m128i; 8] = core::mem::zeroed();
    btf_16_w4_sse2(
        &raw mut cospi_p04_p60 as *const __m128i,
        &raw mut cospi_p60_m04 as *const __m128i,
        __rounding,
        cos_bit,
        x5.as_mut_ptr() as *const __m128i,
        x5.as_mut_ptr().offset(1) as *const __m128i,
        x6.as_mut_ptr(),
        x6.as_mut_ptr().offset(1),
    );
    btf_16_w4_sse2(
        &raw mut cospi_p20_p44 as *const __m128i,
        &raw mut cospi_p44_m20 as *const __m128i,
        __rounding,
        cos_bit,
        x5.as_mut_ptr().offset(2) as *const __m128i,
        x5.as_mut_ptr().offset(3) as *const __m128i,
        x6.as_mut_ptr().offset(2),
        x6.as_mut_ptr().offset(3),
    );
    btf_16_w4_sse2(
        &raw mut cospi_p36_p28 as *const __m128i,
        &raw mut cospi_p28_m36 as *const __m128i,
        __rounding,
        cos_bit,
        x5.as_mut_ptr().offset(4) as *const __m128i,
        x5.as_mut_ptr().offset(5) as *const __m128i,
        x6.as_mut_ptr().offset(4),
        x6.as_mut_ptr().offset(5),
    );
    btf_16_w4_sse2(
        &raw mut cospi_p52_p12 as *const __m128i,
        &raw mut cospi_p12_m52 as *const __m128i,
        __rounding,
        cos_bit,
        x5.as_mut_ptr().offset(6) as *const __m128i,
        x5.as_mut_ptr().offset(7) as *const __m128i,
        x6.as_mut_ptr().offset(6),
        x6.as_mut_ptr().offset(7),
    );
    // stage 7
    *output = x6[1];
    *output.offset(1) = x6[6];
    *output.offset(2) = x6[3];
    *output.offset(3) = x6[4];
    *output.offset(4) = x6[5];
    *output.offset(5) = x6[2];
    *output.offset(6) = x6[7];
    *output.offset(7) = x6[0];
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn fadst8x4_new_sse2(input: *const __m128i, output: *mut __m128i, cos_bit: i8) {
    let mut sinpi: *const i32 = sinpi_arr(cos_bit as i32);
    let sinpi_p01_p02: __m128i = pair_set_epi16(*sinpi.offset(1), *sinpi.offset(2));
    let sinpi_p04_m01: __m128i = pair_set_epi16(*sinpi.offset(4), -*sinpi.offset(1));
    let sinpi_p03_p04: __m128i = pair_set_epi16(*sinpi.offset(3), *sinpi.offset(4));
    let sinpi_m03_p02: __m128i = pair_set_epi16(-*sinpi.offset(3), *sinpi.offset(2));
    let sinpi_p03_p03: __m128i = _mm_set1_epi16(*sinpi.offset(3) as i16);
    let __zero: __m128i = _mm_setzero_si128();
    let __rounding: __m128i = _mm_set1_epi32(1 << (cos_bit as i32 - 1));
    let in7: __m128i = _mm_add_epi16(*input, *input.offset(1));
    let mut u_lo: [__m128i; 8] = core::mem::zeroed();
    let mut u_hi: [__m128i; 8] = core::mem::zeroed();
    let mut v_lo: [__m128i; 8] = core::mem::zeroed();
    let mut v_hi: [__m128i; 8] = core::mem::zeroed();
    u_lo[0] = _mm_unpacklo_epi16(*input, *input.offset(1));
    u_hi[0] = _mm_unpackhi_epi16(*input, *input.offset(1));
    u_lo[1] = _mm_unpacklo_epi16(*input.offset(2), *input.offset(3));
    u_hi[1] = _mm_unpackhi_epi16(*input.offset(2), *input.offset(3));
    u_lo[2] = _mm_unpacklo_epi16(in7, __zero);
    u_hi[2] = _mm_unpackhi_epi16(in7, __zero);
    u_lo[3] = _mm_unpacklo_epi16(*input.offset(2), __zero);
    u_hi[3] = _mm_unpackhi_epi16(*input.offset(2), __zero);
    u_lo[4] = _mm_unpacklo_epi16(*input.offset(3), __zero);
    u_hi[4] = _mm_unpackhi_epi16(*input.offset(3), __zero);
    v_lo[0] = _mm_madd_epi16(u_lo[0], sinpi_p01_p02); // s0 + s2
    v_hi[0] = _mm_madd_epi16(u_hi[0], sinpi_p01_p02); // s0 + s2
    v_lo[1] = _mm_madd_epi16(u_lo[1], sinpi_p03_p04); // s4 + s5
    v_hi[1] = _mm_madd_epi16(u_hi[1], sinpi_p03_p04); // s4 + s5
    v_lo[2] = _mm_madd_epi16(u_lo[2], sinpi_p03_p03); // x1
    v_hi[2] = _mm_madd_epi16(u_hi[2], sinpi_p03_p03); // x1
    v_lo[3] = _mm_madd_epi16(u_lo[0], sinpi_p04_m01); // s1 - s3
    v_hi[3] = _mm_madd_epi16(u_hi[0], sinpi_p04_m01); // s1 - s3
    v_lo[4] = _mm_madd_epi16(u_lo[1], sinpi_m03_p02); // -s4 + s6
    v_hi[4] = _mm_madd_epi16(u_hi[1], sinpi_m03_p02); // -s4 + s6
    v_lo[5] = _mm_madd_epi16(u_lo[3], sinpi_p03_p03); // s4
    v_hi[5] = _mm_madd_epi16(u_hi[3], sinpi_p03_p03); // s4
    v_lo[6] = _mm_madd_epi16(u_lo[4], sinpi_p03_p03);
    v_hi[6] = _mm_madd_epi16(u_hi[4], sinpi_p03_p03);
    u_lo[0] = _mm_add_epi32(v_lo[0], v_lo[1]);
    u_hi[0] = _mm_add_epi32(v_hi[0], v_hi[1]);
    u_lo[1] = _mm_sub_epi32(v_lo[2], v_lo[6]);
    u_hi[1] = _mm_sub_epi32(v_hi[2], v_hi[6]);
    u_lo[2] = _mm_add_epi32(v_lo[3], v_lo[4]);
    u_hi[2] = _mm_add_epi32(v_hi[3], v_hi[4]);
    u_lo[3] = _mm_sub_epi32(u_lo[2], u_lo[0]);
    u_hi[3] = _mm_sub_epi32(u_hi[2], u_hi[0]);
    u_lo[4] = _mm_slli_epi32::<{ 2 }>(v_lo[5]);
    u_hi[4] = _mm_slli_epi32::<{ 2 }>(v_hi[5]);
    u_lo[5] = _mm_sub_epi32(u_lo[4], v_lo[5]);
    u_hi[5] = _mm_sub_epi32(u_hi[4], v_hi[5]);
    u_lo[6] = _mm_add_epi32(u_lo[3], u_lo[5]);
    u_hi[6] = _mm_add_epi32(u_hi[3], u_hi[5]);
    v_lo[0] = _mm_add_epi32(u_lo[0], __rounding);
    v_hi[0] = _mm_add_epi32(u_hi[0], __rounding);
    v_lo[1] = _mm_add_epi32(u_lo[1], __rounding);
    v_hi[1] = _mm_add_epi32(u_hi[1], __rounding);
    v_lo[2] = _mm_add_epi32(u_lo[2], __rounding);
    v_hi[2] = _mm_add_epi32(u_hi[2], __rounding);
    v_lo[3] = _mm_add_epi32(u_lo[6], __rounding);
    v_hi[3] = _mm_add_epi32(u_hi[6], __rounding);
    u_lo[0] = _mm_sra_epi32(v_lo[0], _mm_cvtsi32_si128(cos_bit as i32));
    u_hi[0] = _mm_sra_epi32(v_hi[0], _mm_cvtsi32_si128(cos_bit as i32));
    u_lo[1] = _mm_sra_epi32(v_lo[1], _mm_cvtsi32_si128(cos_bit as i32));
    u_hi[1] = _mm_sra_epi32(v_hi[1], _mm_cvtsi32_si128(cos_bit as i32));
    u_lo[2] = _mm_sra_epi32(v_lo[2], _mm_cvtsi32_si128(cos_bit as i32));
    u_hi[2] = _mm_sra_epi32(v_hi[2], _mm_cvtsi32_si128(cos_bit as i32));
    u_lo[3] = _mm_sra_epi32(v_lo[3], _mm_cvtsi32_si128(cos_bit as i32));
    u_hi[3] = _mm_sra_epi32(v_hi[3], _mm_cvtsi32_si128(cos_bit as i32));
    *output = _mm_packs_epi32(u_lo[0], u_hi[0]);
    *output.offset(1) = _mm_packs_epi32(u_lo[1], u_hi[1]);
    *output.offset(2) = _mm_packs_epi32(u_lo[2], u_hi[2]);
    *output.offset(3) = _mm_packs_epi32(u_lo[3], u_hi[3]);
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn fadst8x16_new_sse2(input: *const __m128i, output: *mut __m128i, cos_bit: i8) {
    let mut cospi: *const i32 = cospi_arr(cos_bit as i32);
    let __zero: __m128i = _mm_setzero_si128();
    let __rounding: __m128i = _mm_set1_epi32(1 << (cos_bit as i32 - 1));
    let mut cospi_p32_p32: __m128i = pair_set_epi16(*cospi.offset(32), *cospi.offset(32));
    let mut cospi_p32_m32: __m128i = pair_set_epi16(*cospi.offset(32), -*cospi.offset(32));
    let mut cospi_p16_p48: __m128i = pair_set_epi16(*cospi.offset(16), *cospi.offset(48));
    let mut cospi_p48_m16: __m128i = pair_set_epi16(*cospi.offset(48), -*cospi.offset(16));
    let mut cospi_m48_p16: __m128i = pair_set_epi16(-*cospi.offset(48), *cospi.offset(16));
    let mut cospi_p08_p56: __m128i = pair_set_epi16(*cospi.offset(8), *cospi.offset(56));
    let mut cospi_p56_m08: __m128i = pair_set_epi16(*cospi.offset(56), -*cospi.offset(8));
    let mut cospi_p40_p24: __m128i = pair_set_epi16(*cospi.offset(40), *cospi.offset(24));
    let mut cospi_p24_m40: __m128i = pair_set_epi16(*cospi.offset(24), -*cospi.offset(40));
    let mut cospi_m56_p08: __m128i = pair_set_epi16(-*cospi.offset(56), *cospi.offset(8));
    let mut cospi_m24_p40: __m128i = pair_set_epi16(-*cospi.offset(24), *cospi.offset(40));
    let mut cospi_p02_p62: __m128i = pair_set_epi16(*cospi.offset(2), *cospi.offset(62));
    let mut cospi_p62_m02: __m128i = pair_set_epi16(*cospi.offset(62), -*cospi.offset(2));
    let mut cospi_p10_p54: __m128i = pair_set_epi16(*cospi.offset(10), *cospi.offset(54));
    let mut cospi_p54_m10: __m128i = pair_set_epi16(*cospi.offset(54), -*cospi.offset(10));
    let mut cospi_p18_p46: __m128i = pair_set_epi16(*cospi.offset(18), *cospi.offset(46));
    let mut cospi_p46_m18: __m128i = pair_set_epi16(*cospi.offset(46), -*cospi.offset(18));
    let mut cospi_p26_p38: __m128i = pair_set_epi16(*cospi.offset(26), *cospi.offset(38));
    let mut cospi_p38_m26: __m128i = pair_set_epi16(*cospi.offset(38), -*cospi.offset(26));
    let mut cospi_p34_p30: __m128i = pair_set_epi16(*cospi.offset(34), *cospi.offset(30));
    let mut cospi_p30_m34: __m128i = pair_set_epi16(*cospi.offset(30), -*cospi.offset(34));
    let mut cospi_p42_p22: __m128i = pair_set_epi16(*cospi.offset(42), *cospi.offset(22));
    let mut cospi_p22_m42: __m128i = pair_set_epi16(*cospi.offset(22), -*cospi.offset(42));
    let mut cospi_p50_p14: __m128i = pair_set_epi16(*cospi.offset(50), *cospi.offset(14));
    let mut cospi_p14_m50: __m128i = pair_set_epi16(*cospi.offset(14), -*cospi.offset(50));
    let mut cospi_p58_p06: __m128i = pair_set_epi16(*cospi.offset(58), *cospi.offset(6));
    let mut cospi_p06_m58: __m128i = pair_set_epi16(*cospi.offset(6), -*cospi.offset(58));
    // stage 1
    let mut x1: [__m128i; 16] = core::mem::zeroed();
    x1[0] = *input;
    x1[1] = _mm_subs_epi16(__zero, *input.offset(15));
    x1[2] = _mm_subs_epi16(__zero, *input.offset(7));
    x1[3] = *input.offset(8);
    x1[4] = _mm_subs_epi16(__zero, *input.offset(3));
    x1[5] = *input.offset(12);
    x1[6] = *input.offset(4);
    x1[7] = _mm_subs_epi16(__zero, *input.offset(11));
    x1[8] = _mm_subs_epi16(__zero, *input.offset(1));
    x1[9] = *input.offset(14);
    x1[10] = *input.offset(6);
    x1[11] = _mm_subs_epi16(__zero, *input.offset(9));
    x1[12] = *input.offset(2);
    x1[13] = _mm_subs_epi16(__zero, *input.offset(13));
    x1[14] = _mm_subs_epi16(__zero, *input.offset(5));
    x1[15] = *input.offset(10);
    // stage 2
    let mut x2: [__m128i; 16] = core::mem::zeroed();
    x2[0] = x1[0];
    x2[1] = x1[1];
    btf_16_sse2!(
        cospi_p32_p32,
        cospi_p32_m32,
        x1[2],
        x1[3],
        x2[2],
        x2[3],
        __rounding,
        cos_bit
    );
    x2[4] = x1[4];
    x2[5] = x1[5];
    btf_16_sse2!(
        cospi_p32_p32,
        cospi_p32_m32,
        x1[6],
        x1[7],
        x2[6],
        x2[7],
        __rounding,
        cos_bit
    );
    x2[8] = x1[8];
    x2[9] = x1[9];
    btf_16_sse2!(
        cospi_p32_p32,
        cospi_p32_m32,
        x1[10],
        x1[11],
        x2[10],
        x2[11],
        __rounding,
        cos_bit
    );
    x2[12] = x1[12];
    x2[13] = x1[13];
    btf_16_sse2!(
        cospi_p32_p32,
        cospi_p32_m32,
        x1[14],
        x1[15],
        x2[14],
        x2[15],
        __rounding,
        cos_bit
    );
    // stage 3
    let mut x3: [__m128i; 16] = core::mem::zeroed();
    x3[0] = _mm_adds_epi16(x2[0], x2[2]);
    x3[2] = _mm_subs_epi16(x2[0], x2[2]);
    x3[1] = _mm_adds_epi16(x2[1], x2[3]);
    x3[3] = _mm_subs_epi16(x2[1], x2[3]);
    x3[4] = _mm_adds_epi16(x2[4], x2[6]);
    x3[6] = _mm_subs_epi16(x2[4], x2[6]);
    x3[5] = _mm_adds_epi16(x2[5], x2[7]);
    x3[7] = _mm_subs_epi16(x2[5], x2[7]);
    x3[8] = _mm_adds_epi16(x2[8], x2[10]);
    x3[10] = _mm_subs_epi16(x2[8], x2[10]);
    x3[9] = _mm_adds_epi16(x2[9], x2[11]);
    x3[11] = _mm_subs_epi16(x2[9], x2[11]);
    x3[12] = _mm_adds_epi16(x2[12], x2[14]);
    x3[14] = _mm_subs_epi16(x2[12], x2[14]);
    x3[13] = _mm_adds_epi16(x2[13], x2[15]);
    x3[15] = _mm_subs_epi16(x2[13], x2[15]);
    // stage 4
    let mut x4: [__m128i; 16] = core::mem::zeroed();
    x4[0] = x3[0];
    x4[1] = x3[1];
    x4[2] = x3[2];
    x4[3] = x3[3];
    btf_16_sse2!(
        cospi_p16_p48,
        cospi_p48_m16,
        x3[4],
        x3[5],
        x4[4],
        x4[5],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m48_p16,
        cospi_p16_p48,
        x3[6],
        x3[7],
        x4[6],
        x4[7],
        __rounding,
        cos_bit
    );
    x4[8] = x3[8];
    x4[9] = x3[9];
    x4[10] = x3[10];
    x4[11] = x3[11];
    btf_16_sse2!(
        cospi_p16_p48,
        cospi_p48_m16,
        x3[12],
        x3[13],
        x4[12],
        x4[13],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m48_p16,
        cospi_p16_p48,
        x3[14],
        x3[15],
        x4[14],
        x4[15],
        __rounding,
        cos_bit
    );
    // stage 5
    let mut x5: [__m128i; 16] = core::mem::zeroed();
    x5[0] = _mm_adds_epi16(x4[0], x4[4]);
    x5[4] = _mm_subs_epi16(x4[0], x4[4]);
    x5[1] = _mm_adds_epi16(x4[1], x4[5]);
    x5[5] = _mm_subs_epi16(x4[1], x4[5]);
    x5[2] = _mm_adds_epi16(x4[2], x4[6]);
    x5[6] = _mm_subs_epi16(x4[2], x4[6]);
    x5[3] = _mm_adds_epi16(x4[3], x4[7]);
    x5[7] = _mm_subs_epi16(x4[3], x4[7]);
    x5[8] = _mm_adds_epi16(x4[8], x4[12]);
    x5[12] = _mm_subs_epi16(x4[8], x4[12]);
    x5[9] = _mm_adds_epi16(x4[9], x4[13]);
    x5[13] = _mm_subs_epi16(x4[9], x4[13]);
    x5[10] = _mm_adds_epi16(x4[10], x4[14]);
    x5[14] = _mm_subs_epi16(x4[10], x4[14]);
    x5[11] = _mm_adds_epi16(x4[11], x4[15]);
    x5[15] = _mm_subs_epi16(x4[11], x4[15]);
    // stage 6
    let mut x6: [__m128i; 16] = core::mem::zeroed();
    x6[0] = x5[0];
    x6[1] = x5[1];
    x6[2] = x5[2];
    x6[3] = x5[3];
    x6[4] = x5[4];
    x6[5] = x5[5];
    x6[6] = x5[6];
    x6[7] = x5[7];
    btf_16_sse2!(
        cospi_p08_p56,
        cospi_p56_m08,
        x5[8],
        x5[9],
        x6[8],
        x6[9],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p40_p24,
        cospi_p24_m40,
        x5[10],
        x5[11],
        x6[10],
        x6[11],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m56_p08,
        cospi_p08_p56,
        x5[12],
        x5[13],
        x6[12],
        x6[13],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m24_p40,
        cospi_p40_p24,
        x5[14],
        x5[15],
        x6[14],
        x6[15],
        __rounding,
        cos_bit
    );
    // stage 7
    let mut x7: [__m128i; 16] = core::mem::zeroed();
    x7[0] = _mm_adds_epi16(x6[0], x6[8]);
    x7[8] = _mm_subs_epi16(x6[0], x6[8]);
    x7[1] = _mm_adds_epi16(x6[1], x6[9]);
    x7[9] = _mm_subs_epi16(x6[1], x6[9]);
    x7[2] = _mm_adds_epi16(x6[2], x6[10]);
    x7[10] = _mm_subs_epi16(x6[2], x6[10]);
    x7[3] = _mm_adds_epi16(x6[3], x6[11]);
    x7[11] = _mm_subs_epi16(x6[3], x6[11]);
    x7[4] = _mm_adds_epi16(x6[4], x6[12]);
    x7[12] = _mm_subs_epi16(x6[4], x6[12]);
    x7[5] = _mm_adds_epi16(x6[5], x6[13]);
    x7[13] = _mm_subs_epi16(x6[5], x6[13]);
    x7[6] = _mm_adds_epi16(x6[6], x6[14]);
    x7[14] = _mm_subs_epi16(x6[6], x6[14]);
    x7[7] = _mm_adds_epi16(x6[7], x6[15]);
    x7[15] = _mm_subs_epi16(x6[7], x6[15]);
    // stage 8
    let mut x8: [__m128i; 16] = core::mem::zeroed();
    btf_16_sse2!(
        cospi_p02_p62,
        cospi_p62_m02,
        x7[0],
        x7[1],
        x8[0],
        x8[1],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p10_p54,
        cospi_p54_m10,
        x7[2],
        x7[3],
        x8[2],
        x8[3],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p18_p46,
        cospi_p46_m18,
        x7[4],
        x7[5],
        x8[4],
        x8[5],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p26_p38,
        cospi_p38_m26,
        x7[6],
        x7[7],
        x8[6],
        x8[7],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p34_p30,
        cospi_p30_m34,
        x7[8],
        x7[9],
        x8[8],
        x8[9],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p42_p22,
        cospi_p22_m42,
        x7[10],
        x7[11],
        x8[10],
        x8[11],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p50_p14,
        cospi_p14_m50,
        x7[12],
        x7[13],
        x8[12],
        x8[13],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p58_p06,
        cospi_p06_m58,
        x7[14],
        x7[15],
        x8[14],
        x8[15],
        __rounding,
        cos_bit
    );
    // stage 9
    *output = x8[1];
    *output.offset(1) = x8[14];
    *output.offset(2) = x8[3];
    *output.offset(3) = x8[12];
    *output.offset(4) = x8[5];
    *output.offset(5) = x8[10];
    *output.offset(6) = x8[7];
    *output.offset(7) = x8[8];
    *output.offset(8) = x8[9];
    *output.offset(9) = x8[6];
    *output.offset(10) = x8[11];
    *output.offset(11) = x8[4];
    *output.offset(12) = x8[13];
    *output.offset(13) = x8[2];
    *output.offset(14) = x8[15];
    *output.offset(15) = x8[0];
}

static col_txfm4x4_arr: [transform_1d_sse2; (TX_TYPES) as usize] = [
    Some(fdct4x4_new_sse2),
    Some(fadst4x4_new_sse2),
    Some(fdct4x4_new_sse2),
    Some(fadst4x4_new_sse2),
    Some(fadst4x4_new_sse2),
    Some(fdct4x4_new_sse2),
    Some(fadst4x4_new_sse2),
    Some(fadst4x4_new_sse2),
    Some(fadst4x4_new_sse2),
    Some(fidentity4x4_new_sse2),
    Some(fdct4x4_new_sse2),
    Some(fidentity4x4_new_sse2),
    Some(fadst4x4_new_sse2),
    Some(fidentity4x4_new_sse2),
    Some(fadst4x4_new_sse2),
    Some(fidentity4x4_new_sse2),
];

// DCT_DCT
// ADST_DCT
// DCT_ADST
// ADST_ADST
// FLIPADST_DCT
// DCT_FLIPADST
// FLIPADST_FLIPADST
// ADST_FLIPADST
// FLIPADST_ADST
// IDTX
// V_DCT
// H_DCT
// V_ADST
// H_ADST
// V_FLIPADST
// H_FLIPADST
static row_txfm4x4_arr: [transform_1d_sse2; (TX_TYPES) as usize] = [
    Some(fdct4x4_new_sse2),
    Some(fdct4x4_new_sse2),
    Some(fadst4x4_new_sse2),
    Some(fadst4x4_new_sse2),
    Some(fdct4x4_new_sse2),
    Some(fadst4x4_new_sse2),
    Some(fadst4x4_new_sse2),
    Some(fadst4x4_new_sse2),
    Some(fadst4x4_new_sse2),
    Some(fidentity4x4_new_sse2),
    Some(fidentity4x4_new_sse2),
    Some(fdct4x4_new_sse2),
    Some(fidentity4x4_new_sse2),
    Some(fadst4x4_new_sse2),
    Some(fidentity4x4_new_sse2),
    Some(fadst4x4_new_sse2),
];

// DCT_DCT
// ADST_DCT
// DCT_ADST
// ADST_ADST
// FLIPADST_DCT
// DCT_FLIPADST
// FLIPADST_FLIPADST
// ADST_FLIPADST
// FLIPADST_ADST
// IDTX
// V_DCT
// H_DCT
// V_ADST
// H_ADST
// V_FLIPADST
// H_FLIPADST
static col_txfm4x8_arr: [transform_1d_sse2; (TX_TYPES) as usize] = [
    Some(fdct4x8_new_sse2),
    Some(fadst4x8_new_sse2),
    Some(fdct4x8_new_sse2),
    Some(fadst4x8_new_sse2),
    Some(fadst4x8_new_sse2),
    Some(fdct4x8_new_sse2),
    Some(fadst4x8_new_sse2),
    Some(fadst4x8_new_sse2),
    Some(fadst4x8_new_sse2),
    Some(fidentity8x8_new_sse2),
    Some(fdct4x8_new_sse2),
    Some(fidentity8x8_new_sse2),
    Some(fadst4x8_new_sse2),
    Some(fidentity8x8_new_sse2),
    Some(fadst4x8_new_sse2),
    Some(fidentity8x8_new_sse2),
];

// DCT_DCT
// ADST_DCT
// DCT_ADST
// ADST_ADST
// FLIPADST_DCT
// DCT_FLIPADST
// FLIPADST_FLIPADST
// ADST_FLIPADST
// FLIPADST_ADST
// IDTX
// V_DCT
// H_DCT
// V_ADST
// H_ADST
// V_FLIPADST
// H_FLIPADST
static row_txfm8x4_arr: [transform_1d_sse2; (TX_TYPES) as usize] = [
    Some(fdct8x4_new_sse2),
    Some(fdct8x4_new_sse2),
    Some(fadst8x4_new_sse2),
    Some(fadst8x4_new_sse2),
    Some(fdct8x4_new_sse2),
    Some(fadst8x4_new_sse2),
    Some(fadst8x4_new_sse2),
    Some(fadst8x4_new_sse2),
    Some(fadst8x4_new_sse2),
    Some(fidentity8x4_new_sse2),
    Some(fidentity8x4_new_sse2),
    Some(fdct8x4_new_sse2),
    Some(fidentity8x4_new_sse2),
    Some(fadst8x4_new_sse2),
    Some(fidentity8x4_new_sse2),
    Some(fadst8x4_new_sse2),
];

// DCT_DCT
// ADST_DCT
// DCT_ADST
// ADST_ADST
// FLIPADST_DCT
// DCT_FLIPADST
// FLIPADST_FLIPADST
// ADST_FLIPADST
// FLIPADST_ADST
// IDTX
// V_DCT
// H_DCT
// V_ADST
// H_ADST
// V_FLIPADST
// H_FLIPADST
static col_txfm8x4_arr: [transform_1d_sse2; (TX_TYPES) as usize] = [
    Some(fdct8x4_new_sse2),
    Some(fadst8x4_new_sse2),
    Some(fdct8x4_new_sse2),
    Some(fadst8x4_new_sse2),
    Some(fadst8x4_new_sse2),
    Some(fdct8x4_new_sse2),
    Some(fadst8x4_new_sse2),
    Some(fadst8x4_new_sse2),
    Some(fadst8x4_new_sse2),
    Some(fidentity8x4_new_sse2),
    Some(fdct8x4_new_sse2),
    Some(fidentity8x4_new_sse2),
    Some(fadst8x4_new_sse2),
    Some(fidentity8x4_new_sse2),
    Some(fadst8x4_new_sse2),
    Some(fidentity8x4_new_sse2),
];

// DCT_DCT
// ADST_DCT
// DCT_ADST
// ADST_ADST
// FLIPADST_DCT
// DCT_FLIPADST
// FLIPADST_FLIPADST
// ADST_FLIPADST
// FLIPADST_ADST
// IDTX
// V_DCT
// H_DCT
// V_ADST
// H_ADST
// V_FLIPADST
// H_FLIPADST
static row_txfm4x8_arr: [transform_1d_sse2; (TX_TYPES) as usize] = [
    Some(fdct4x8_new_sse2),
    Some(fdct4x8_new_sse2),
    Some(fadst4x8_new_sse2),
    Some(fadst4x8_new_sse2),
    Some(fdct4x8_new_sse2),
    Some(fadst4x8_new_sse2),
    Some(fadst4x8_new_sse2),
    Some(fadst4x8_new_sse2),
    Some(fadst4x8_new_sse2),
    Some(fidentity8x8_new_sse2),
    Some(fidentity8x8_new_sse2),
    Some(fdct4x8_new_sse2),
    Some(fidentity8x8_new_sse2),
    Some(fadst4x8_new_sse2),
    Some(fidentity8x8_new_sse2),
    Some(fadst4x8_new_sse2),
];

// DCT_DCT
// ADST_DCT
// DCT_ADST
// ADST_ADST
// FLIPADST_DCT
// DCT_FLIPADST
// FLIPADST_FLIPADST
// ADST_FLIPADST
// FLIPADST_ADST
// IDTX
// V_DCT
// H_DCT
// V_ADST
// H_ADST
// V_FLIPADST
// H_FLIPADST
static col_txfm8x8_arr: [transform_1d_sse2; (TX_TYPES) as usize] = [
    Some(fdct8x8_new_sse2),
    Some(fadst8x8_new_sse2),
    Some(fdct8x8_new_sse2),
    Some(fadst8x8_new_sse2),
    Some(fadst8x8_new_sse2),
    Some(fdct8x8_new_sse2),
    Some(fadst8x8_new_sse2),
    Some(fadst8x8_new_sse2),
    Some(fadst8x8_new_sse2),
    Some(fidentity8x8_new_sse2),
    Some(fdct8x8_new_sse2),
    Some(fidentity8x8_new_sse2),
    Some(fadst8x8_new_sse2),
    Some(fidentity8x8_new_sse2),
    Some(fadst8x8_new_sse2),
    Some(fidentity8x8_new_sse2),
];

// DCT_DCT
// ADST_DCT
// DCT_ADST
// ADST_ADST
// FLIPADST_DCT
// DCT_FLIPADST
// FLIPADST_FLIPADST
// ADST_FLIPADST
// FLIPADST_ADST
// IDTX
// V_DCT
// H_DCT
// V_ADST
// H_ADST
// V_FLIPADST
// H_FLIPADST
static row_txfm8x8_arr: [transform_1d_sse2; (TX_TYPES) as usize] = [
    Some(fdct8x8_new_sse2),
    Some(fdct8x8_new_sse2),
    Some(fadst8x8_new_sse2),
    Some(fadst8x8_new_sse2),
    Some(fdct8x8_new_sse2),
    Some(fadst8x8_new_sse2),
    Some(fadst8x8_new_sse2),
    Some(fadst8x8_new_sse2),
    Some(fadst8x8_new_sse2),
    Some(fidentity8x8_new_sse2),
    Some(fidentity8x8_new_sse2),
    Some(fdct8x8_new_sse2),
    Some(fidentity8x8_new_sse2),
    Some(fadst8x8_new_sse2),
    Some(fidentity8x8_new_sse2),
    Some(fadst8x8_new_sse2),
];

// DCT_DCT
// ADST_DCT
// DCT_ADST
// ADST_ADST
// FLIPADST_DCT
// DCT_FLIPADST
// FLIPADST_FLIPADST
// ADST_FLIPADST
// FLIPADST_ADST
// IDTX
// V_DCT
// H_DCT
// V_ADST
// H_ADST
// V_FLIPADST
// H_FLIPADST
static col_txfm8x16_arr: [transform_1d_sse2; (TX_TYPES) as usize] = [
    Some(fdct8x16_new_sse2),
    Some(fadst8x16_new_sse2),
    Some(fdct8x16_new_sse2),
    Some(fadst8x16_new_sse2),
    Some(fadst8x16_new_sse2),
    Some(fdct8x16_new_sse2),
    Some(fadst8x16_new_sse2),
    Some(fadst8x16_new_sse2),
    Some(fadst8x16_new_sse2),
    Some(fidentity8x16_new_sse2),
    Some(fdct8x16_new_sse2),
    Some(fidentity8x16_new_sse2),
    Some(fadst8x16_new_sse2),
    Some(fidentity8x16_new_sse2),
    Some(fadst8x16_new_sse2),
    Some(fidentity8x16_new_sse2),
];

// DCT_DCT
// ADST_DCT
// DCT_ADST
// ADST_ADST
// FLIPADST_DCT
// DCT_FLIPADST
// FLIPADST_FLIPADST
// ADST_FLIPADST
// FLIPADST_ADST
// IDTX
// V_DCT
// H_DCT
// V_ADST
// H_ADST
// V_FLIPADST
// H_FLIPADST
static row_txfm8x16_arr: [transform_1d_sse2; (TX_TYPES) as usize] = [
    Some(fdct8x16_new_sse2),
    Some(fdct8x16_new_sse2),
    Some(fadst8x16_new_sse2),
    Some(fadst8x16_new_sse2),
    Some(fdct8x16_new_sse2),
    Some(fadst8x16_new_sse2),
    Some(fadst8x16_new_sse2),
    Some(fadst8x16_new_sse2),
    Some(fadst8x16_new_sse2),
    Some(fidentity8x16_new_sse2),
    Some(fidentity8x16_new_sse2),
    Some(fdct8x16_new_sse2),
    Some(fidentity8x16_new_sse2),
    Some(fadst8x16_new_sse2),
    Some(fidentity8x16_new_sse2),
    Some(fadst8x16_new_sse2),
];

// DCT_DCT
// ADST_DCT
// DCT_ADST
// ADST_ADST
// FLIPADST_DCT
// DCT_FLIPADST
// FLIPADST_FLIPADST
// ADST_FLIPADST
// FLIPADST_ADST
// IDTX
// V_DCT
// H_DCT
// V_ADST
// H_ADST
// V_FLIPADST
// H_FLIPADST
static row_txfm8x32_arr: [transform_1d_sse2; (TX_TYPES) as usize] = [
    Some(av1_fdct8x32_new_sse2),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(fidentity8x32_new_sse2),
    Some(fidentity8x32_new_sse2),
    Some(av1_fdct8x32_new_sse2),
    None,
    None,
    None,
    None,
];

// DCT_DCT
// ADST_DCT
// DCT_ADST
// ADST_ADST
// FLIPADST_DCT
// DCT_FLIPADST
// FLIPADST_FLIPADST
// ADST_FLIPADST
// FLIPADST_ADST
// IDTX
// V_DCT
// H_DCT
// V_ADST
// H_ADST
// V_FLIPADST
// H_FLIPADST
#[target_feature(enable = "sse2")]
pub unsafe fn av1_lowbd_fwd_txfm2d_4x4_sse2(
    input: *const i16,
    output: *mut i32,
    stride: i32,
    tx_type: u8,
    bd: i32,
) {
    let _ = bd;
    let mut buf0: [__m128i; 4] = core::mem::zeroed();
    let mut buf1: [__m128i; 4] = core::mem::zeroed();
    let mut buf: *mut __m128i;
    let mut shift: *const i8 = av1_fwd_txfm_shift_ls[TX_4X4 as usize];
    let txw_idx: i32 = get_txw_idx(TX_4X4 as u8);
    let txh_idx: i32 = get_txh_idx(TX_4X4 as u8);
    let cos_bit_col: i32 = av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32;
    let cos_bit_row: i32 = av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32;
    let width: i32 = 4;
    let height: i32 = 4;
    let col_txfm: transform_1d_sse2 = col_txfm4x4_arr[tx_type as usize];
    let row_txfm: transform_1d_sse2 = row_txfm4x4_arr[tx_type as usize];
    let mut ud_flip: i32 = 0;
    let mut lr_flip: i32 = 0;
    get_flip_cfg(tx_type, &raw mut ud_flip, &raw mut lr_flip);
    if ud_flip != 0 {
        load_buffer_16bit_to_16bit_w4_flip(input, stride, buf0.as_mut_ptr(), height);
    } else {
        load_buffer_16bit_to_16bit_w4(input, stride, buf0.as_mut_ptr(), height);
    }
    round_shift_16bit(buf0.as_mut_ptr(), height, *shift as i32);
    col_txfm.unwrap()(
        buf0.as_mut_ptr() as *const __m128i,
        buf0.as_mut_ptr(),
        cos_bit_col as i8,
    );
    round_shift_16bit(buf0.as_mut_ptr(), height, *shift.offset(1) as i32);
    transpose_16bit_4x4(buf0.as_mut_ptr() as *const __m128i, buf1.as_mut_ptr());
    if lr_flip != 0 {
        buf = buf0.as_mut_ptr();
        flip_buf_sse2(buf1.as_mut_ptr(), buf, width);
    } else {
        buf = buf1.as_mut_ptr();
    }
    row_txfm.unwrap()(buf as *const __m128i, buf, cos_bit_row as i8);
    round_shift_16bit(buf, width, *shift.offset(2) as i32);
    store_buffer_16bit_to_32bit_w4(buf as *const __m128i, output, height, width);
}

#[target_feature(enable = "sse2")]
pub unsafe fn av1_lowbd_fwd_txfm2d_4x8_sse2(
    input: *const i16,
    output: *mut i32,
    stride: i32,
    tx_type: u8,
    bd: i32,
) {
    let _ = stride;
    let _ = bd;
    let mut buf0: [__m128i; 8] = core::mem::zeroed();
    let mut buf1: [__m128i; 8] = core::mem::zeroed();
    let mut buf: *mut __m128i;
    let mut shift: *const i8 = av1_fwd_txfm_shift_ls[TX_4X8 as usize];
    let txw_idx: i32 = get_txw_idx(TX_4X8 as u8);
    let txh_idx: i32 = get_txh_idx(TX_4X8 as u8);
    let cos_bit_col: i32 = av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32;
    let cos_bit_row: i32 = av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32;
    let width: i32 = 4;
    let height: i32 = 8;
    let col_txfm: transform_1d_sse2 = col_txfm4x8_arr[tx_type as usize];
    let row_txfm: transform_1d_sse2 = row_txfm8x4_arr[tx_type as usize];
    let mut ud_flip: i32 = 0;
    let mut lr_flip: i32 = 0;
    get_flip_cfg(tx_type, &raw mut ud_flip, &raw mut lr_flip);
    if ud_flip != 0 {
        load_buffer_16bit_to_16bit_w4_flip(input, stride, buf0.as_mut_ptr(), height);
    } else {
        load_buffer_16bit_to_16bit_w4(input, stride, buf0.as_mut_ptr(), height);
    }
    round_shift_16bit(buf0.as_mut_ptr(), height, *shift as i32);
    col_txfm.unwrap()(
        buf0.as_mut_ptr() as *const __m128i,
        buf0.as_mut_ptr(),
        cos_bit_col as i8,
    );
    round_shift_16bit(buf0.as_mut_ptr(), height, *shift.offset(1) as i32);
    transpose_16bit_4x8(buf0.as_mut_ptr() as *const __m128i, buf1.as_mut_ptr());
    if lr_flip != 0 {
        buf = buf0.as_mut_ptr();
        flip_buf_sse2(buf1.as_mut_ptr(), buf, width);
    } else {
        buf = buf1.as_mut_ptr();
    }
    row_txfm.unwrap()(buf as *const __m128i, buf, cos_bit_row as i8);
    round_shift_16bit(buf, width, *shift.offset(2) as i32);
    store_rect_buffer_16bit_to_32bit_w8(buf as *const __m128i, output, height, width);
}

#[target_feature(enable = "sse2")]
pub unsafe fn av1_lowbd_fwd_txfm2d_4x16_sse2(
    input: *const i16,
    output: *mut i32,
    stride: i32,
    tx_type: u8,
    bd: i32,
) {
    let _ = bd;
    let mut buf0: [__m128i; 16] = core::mem::zeroed();
    let mut buf1: [__m128i; 16] = core::mem::zeroed();
    let mut shift: *const i8 = av1_fwd_txfm_shift_ls[TX_4X16 as usize];
    let txw_idx: i32 = get_txw_idx(TX_4X16 as u8);
    let txh_idx: i32 = get_txh_idx(TX_4X16 as u8);
    let cos_bit_col: i32 = av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32;
    let cos_bit_row: i32 = av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32;
    let width: i32 = 4;
    let height: i32 = 16;
    let col_txfm: transform_1d_sse2 = col_txfm8x16_arr[tx_type as usize];
    let row_txfm: transform_1d_sse2 = row_txfm8x4_arr[tx_type as usize];
    let mut ud_flip: i32 = 0;
    let mut lr_flip: i32 = 0;
    get_flip_cfg(tx_type, &raw mut ud_flip, &raw mut lr_flip);
    if ud_flip != 0 {
        load_buffer_16bit_to_16bit_w4_flip(input, stride, buf0.as_mut_ptr(), height);
    } else {
        load_buffer_16bit_to_16bit_w4(input, stride, buf0.as_mut_ptr(), height);
    }
    round_shift_16bit(buf0.as_mut_ptr(), height, *shift as i32);
    col_txfm.unwrap()(
        buf0.as_mut_ptr() as *const __m128i,
        buf0.as_mut_ptr(),
        cos_bit_col as i8,
    );
    round_shift_16bit(buf0.as_mut_ptr(), height, *shift.offset(1) as i32);
    transpose_16bit_4x8(buf0.as_mut_ptr() as *const __m128i, buf1.as_mut_ptr());
    transpose_16bit_4x8(
        buf0.as_mut_ptr().offset(8) as *const __m128i,
        buf1.as_mut_ptr().offset(8),
    );
    for i in 0..2 {
        let mut buf: *mut __m128i;
        if lr_flip != 0 {
            buf = buf0.as_mut_ptr();
            flip_buf_sse2(buf1.as_mut_ptr().offset((8 * i) as isize), buf, width);
        } else {
            buf = buf1.as_mut_ptr().offset((8 * i) as isize);
        }
        row_txfm.unwrap()(buf as *const __m128i, buf, cos_bit_row as i8);
        round_shift_16bit(buf, width, *shift.offset(2) as i32);
        store_buffer_16bit_to_32bit_w8(
            buf as *const __m128i,
            output.offset((8 * i) as isize),
            height,
            width,
        );
    }
}

#[target_feature(enable = "sse2")]
pub unsafe fn av1_lowbd_fwd_txfm2d_8x4_sse2(
    input: *const i16,
    output: *mut i32,
    stride: i32,
    tx_type: u8,
    bd: i32,
) {
    let _ = bd;
    let mut buf0: [__m128i; 8] = core::mem::zeroed();
    let mut buf1: [__m128i; 8] = core::mem::zeroed();
    let mut buf: *mut __m128i;
    let mut shift: *const i8 = av1_fwd_txfm_shift_ls[TX_8X4 as usize];
    let txw_idx: i32 = get_txw_idx(TX_8X4 as u8);
    let txh_idx: i32 = get_txh_idx(TX_8X4 as u8);
    let cos_bit_col: i32 = av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32;
    let cos_bit_row: i32 = av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32;
    let width: i32 = 8;
    let height: i32 = 4;
    let col_txfm: transform_1d_sse2 = col_txfm8x4_arr[tx_type as usize];
    let row_txfm: transform_1d_sse2 = row_txfm4x8_arr[tx_type as usize];
    let mut ud_flip: i32 = 0;
    let mut lr_flip: i32 = 0;
    get_flip_cfg(tx_type, &raw mut ud_flip, &raw mut lr_flip);
    if ud_flip != 0 {
        load_buffer_16bit_to_16bit_flip(input, stride, buf0.as_mut_ptr(), height);
    } else {
        load_buffer_16bit_to_16bit(input, stride, buf0.as_mut_ptr(), height);
    }
    round_shift_16bit(buf0.as_mut_ptr(), height, *shift as i32);
    col_txfm.unwrap()(
        buf0.as_mut_ptr() as *const __m128i,
        buf0.as_mut_ptr(),
        cos_bit_col as i8,
    );
    round_shift_16bit(buf0.as_mut_ptr(), height, *shift.offset(1) as i32);
    transpose_16bit_8x8(buf0.as_mut_ptr() as *const __m128i, buf1.as_mut_ptr());
    if lr_flip != 0 {
        buf = buf0.as_mut_ptr();
        flip_buf_sse2(buf1.as_mut_ptr(), buf, width);
    } else {
        buf = buf1.as_mut_ptr();
    }
    row_txfm.unwrap()(buf as *const __m128i, buf, cos_bit_row as i8);
    round_shift_16bit(buf, width, *shift.offset(2) as i32);
    store_rect_buffer_16bit_to_32bit_w4(buf as *const __m128i, output, height, width);
}

#[target_feature(enable = "sse2")]
pub unsafe fn av1_lowbd_fwd_txfm2d_8x8_sse2(
    input: *const i16,
    output: *mut i32,
    stride: i32,
    tx_type: u8,
    bd: i32,
) {
    let _ = bd;
    let mut buf0: [__m128i; 8] = core::mem::zeroed();
    let mut buf1: [__m128i; 8] = core::mem::zeroed();
    let mut buf: *mut __m128i;
    let mut shift: *const i8 = av1_fwd_txfm_shift_ls[TX_8X8 as usize];
    let txw_idx: i32 = get_txw_idx(TX_8X8 as u8);
    let txh_idx: i32 = get_txh_idx(TX_8X8 as u8);
    let cos_bit_col: i32 = av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32;
    let cos_bit_row: i32 = av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32;
    let width: i32 = 8;
    let height: i32 = 8;
    let col_txfm: transform_1d_sse2 = col_txfm8x8_arr[tx_type as usize];
    let row_txfm: transform_1d_sse2 = row_txfm8x8_arr[tx_type as usize];
    let mut ud_flip: i32 = 0;
    let mut lr_flip: i32 = 0;
    get_flip_cfg(tx_type, &raw mut ud_flip, &raw mut lr_flip);
    if ud_flip != 0 {
        load_buffer_16bit_to_16bit_flip(input, stride, buf0.as_mut_ptr(), height);
    } else {
        load_buffer_16bit_to_16bit(input, stride, buf0.as_mut_ptr(), height);
    }
    round_shift_16bit(buf0.as_mut_ptr(), height, *shift as i32);
    col_txfm.unwrap()(
        buf0.as_mut_ptr() as *const __m128i,
        buf0.as_mut_ptr(),
        cos_bit_col as i8,
    );
    round_shift_16bit(buf0.as_mut_ptr(), height, *shift.offset(1) as i32);
    transpose_16bit_8x8(buf0.as_mut_ptr() as *const __m128i, buf1.as_mut_ptr());
    if lr_flip != 0 {
        buf = buf0.as_mut_ptr();
        flip_buf_sse2(buf1.as_mut_ptr(), buf, width);
    } else {
        buf = buf1.as_mut_ptr();
    }
    row_txfm.unwrap()(buf as *const __m128i, buf, cos_bit_row as i8);
    round_shift_16bit(buf, width, *shift.offset(2) as i32);
    store_buffer_16bit_to_32bit_w8(buf as *const __m128i, output, height, width);
}

#[target_feature(enable = "sse2")]
pub unsafe fn av1_lowbd_fwd_txfm2d_8x16_sse2(
    input: *const i16,
    output: *mut i32,
    stride: i32,
    tx_type: u8,
    bd: i32,
) {
    let _ = bd;
    let mut buf0: [__m128i; 16] = core::mem::zeroed();
    let mut buf1: [__m128i; 16] = core::mem::zeroed();
    let mut shift: *const i8 = av1_fwd_txfm_shift_ls[TX_8X16 as usize];
    let txw_idx: i32 = get_txw_idx(TX_8X16 as u8);
    let txh_idx: i32 = get_txh_idx(TX_8X16 as u8);
    let cos_bit_col: i32 = av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32;
    let cos_bit_row: i32 = av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32;
    let width: i32 = 8;
    let height: i32 = 16;
    let col_txfm: transform_1d_sse2 = col_txfm8x16_arr[tx_type as usize];
    let row_txfm: transform_1d_sse2 = row_txfm8x8_arr[tx_type as usize];
    let mut ud_flip: i32 = 0;
    let mut lr_flip: i32 = 0;
    get_flip_cfg(tx_type, &raw mut ud_flip, &raw mut lr_flip);
    if ud_flip != 0 {
        load_buffer_16bit_to_16bit_flip(input, stride, buf0.as_mut_ptr(), height);
    } else {
        load_buffer_16bit_to_16bit(input, stride, buf0.as_mut_ptr(), height);
    }
    round_shift_16bit(buf0.as_mut_ptr(), height, *shift as i32);
    col_txfm.unwrap()(
        buf0.as_mut_ptr() as *const __m128i,
        buf0.as_mut_ptr(),
        cos_bit_col as i8,
    );
    round_shift_16bit(buf0.as_mut_ptr(), height, *shift.offset(1) as i32);
    transpose_16bit_8x8(buf0.as_mut_ptr() as *const __m128i, buf1.as_mut_ptr());
    transpose_16bit_8x8(
        buf0.as_mut_ptr().offset(8) as *const __m128i,
        buf1.as_mut_ptr().offset(8),
    );
    for i in 0..2 {
        let mut buf: *mut __m128i;
        if lr_flip != 0 {
            buf = buf0.as_mut_ptr();
            flip_buf_sse2(buf1.as_mut_ptr().offset((width * i) as isize), buf, width);
        } else {
            buf = buf1.as_mut_ptr().offset((width * i) as isize);
        }
        row_txfm.unwrap()(buf as *const __m128i, buf, cos_bit_row as i8);
        round_shift_16bit(buf, width, *shift.offset(2) as i32);
        store_rect_buffer_16bit_to_32bit_w8(
            buf as *const __m128i,
            output.offset((8 * i) as isize),
            height,
            width,
        );
    }
}

#[target_feature(enable = "sse2")]
pub unsafe fn av1_lowbd_fwd_txfm2d_8x32_sse2(
    input: *const i16,
    output: *mut i32,
    stride: i32,
    tx_type: u8,
    bd: i32,
) {
    let _ = bd;
    let mut buf0: [__m128i; 32] = core::mem::zeroed();
    let mut buf1: [__m128i; 32] = core::mem::zeroed();
    let mut shift: *const i8 = av1_fwd_txfm_shift_ls[TX_8X32 as usize];
    let txw_idx: i32 = get_txw_idx(TX_8X32 as u8);
    let txh_idx: i32 = get_txh_idx(TX_8X32 as u8);
    let cos_bit_col: i32 = av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32;
    let cos_bit_row: i32 = av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32;
    let width: i32 = 8;
    let height: i32 = 32;
    let col_txfm: transform_1d_sse2 = col_txfm8x32_arr[tx_type as usize];
    let row_txfm: transform_1d_sse2 = row_txfm8x8_arr[tx_type as usize];
    let mut ud_flip: i32 = 0;
    let mut lr_flip: i32 = 0;
    get_flip_cfg(tx_type, &raw mut ud_flip, &raw mut lr_flip);
    if ud_flip != 0 {
        load_buffer_16bit_to_16bit_flip(input, stride, buf0.as_mut_ptr(), height);
    } else {
        load_buffer_16bit_to_16bit(input, stride, buf0.as_mut_ptr(), height);
    }
    round_shift_16bit(buf0.as_mut_ptr(), height, *shift as i32);
    col_txfm.unwrap()(
        buf0.as_mut_ptr() as *const __m128i,
        buf0.as_mut_ptr(),
        cos_bit_col as i8,
    );
    round_shift_16bit(buf0.as_mut_ptr(), height, *shift.offset(1) as i32);
    transpose_16bit_8x8(buf0.as_mut_ptr() as *const __m128i, buf1.as_mut_ptr());
    transpose_16bit_8x8(
        buf0.as_mut_ptr().offset(8) as *const __m128i,
        buf1.as_mut_ptr().offset(8),
    );
    transpose_16bit_8x8(
        buf0.as_mut_ptr().offset(16) as *const __m128i,
        buf1.as_mut_ptr().offset(16),
    );
    transpose_16bit_8x8(
        buf0.as_mut_ptr().offset(24) as *const __m128i,
        buf1.as_mut_ptr().offset(24),
    );
    for i in 0..4 {
        let mut buf: *mut __m128i;
        if lr_flip != 0 {
            buf = buf0.as_mut_ptr();
            flip_buf_sse2(buf1.as_mut_ptr().offset((width * i) as isize), buf, width);
        } else {
            buf = buf1.as_mut_ptr().offset((width * i) as isize);
        }
        row_txfm.unwrap()(buf as *const __m128i, buf, cos_bit_row as i8);
        round_shift_16bit(buf, width, *shift.offset(2) as i32);
        store_buffer_16bit_to_32bit_w8(
            buf as *const __m128i,
            output.offset((8 * i) as isize),
            height,
            width,
        );
    }
}

#[target_feature(enable = "sse2")]
pub unsafe fn av1_lowbd_fwd_txfm2d_16x4_sse2(
    input: *const i16,
    output: *mut i32,
    stride: i32,
    tx_type: u8,
    bd: i32,
) {
    let _ = bd;
    let mut buf0: [__m128i; 16] = core::mem::zeroed();
    let mut buf1: [__m128i; 16] = core::mem::zeroed();
    let mut shift: *const i8 = av1_fwd_txfm_shift_ls[TX_16X4 as usize];
    let txw_idx: i32 = get_txw_idx(TX_16X4 as u8);
    let txh_idx: i32 = get_txh_idx(TX_16X4 as u8);
    let cos_bit_col: i32 = av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32;
    let cos_bit_row: i32 = av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32;
    let width: i32 = 16;
    let height: i32 = 4;
    let col_txfm: transform_1d_sse2 = col_txfm8x4_arr[tx_type as usize];
    let row_txfm: transform_1d_sse2 = row_txfm8x16_arr[tx_type as usize];
    let mut buf: *mut __m128i;
    let mut ud_flip: i32 = 0;
    let mut lr_flip: i32 = 0;
    get_flip_cfg(tx_type, &raw mut ud_flip, &raw mut lr_flip);
    for i in 0..2 {
        if ud_flip != 0 {
            load_buffer_16bit_to_16bit_flip(
                input.offset((8 * i) as isize),
                stride,
                buf0.as_mut_ptr(),
                height,
            );
        } else {
            load_buffer_16bit_to_16bit(
                input.offset((8 * i) as isize),
                stride,
                buf0.as_mut_ptr(),
                height,
            );
        }
        round_shift_16bit(buf0.as_mut_ptr(), height, *shift as i32);
        col_txfm.unwrap()(
            buf0.as_mut_ptr() as *const __m128i,
            buf0.as_mut_ptr(),
            cos_bit_col as i8,
        );
        round_shift_16bit(buf0.as_mut_ptr(), height, *shift.offset(1) as i32);
        transpose_16bit_8x4(
            buf0.as_mut_ptr() as *const __m128i,
            buf1.as_mut_ptr().offset((8 * i) as isize),
        );
    }
    if lr_flip != 0 {
        buf = buf0.as_mut_ptr();
        flip_buf_sse2(buf1.as_mut_ptr(), buf, width);
    } else {
        buf = buf1.as_mut_ptr();
    }
    row_txfm.unwrap()(buf as *const __m128i, buf, cos_bit_row as i8);
    round_shift_16bit(buf, width, *shift.offset(2) as i32);
    store_buffer_16bit_to_32bit_w4(buf as *const __m128i, output, height, width);
}

#[target_feature(enable = "sse2")]
pub unsafe fn av1_lowbd_fwd_txfm2d_16x8_sse2(
    input: *const i16,
    output: *mut i32,
    stride: i32,
    tx_type: u8,
    bd: i32,
) {
    let _ = bd;
    let mut buf0: [__m128i; 16] = core::mem::zeroed();
    let mut buf1: [__m128i; 16] = core::mem::zeroed();
    let mut shift: *const i8 = av1_fwd_txfm_shift_ls[TX_16X8 as usize];
    let txw_idx: i32 = get_txw_idx(TX_16X8 as u8);
    let txh_idx: i32 = get_txh_idx(TX_16X8 as u8);
    let cos_bit_col: i32 = av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32;
    let cos_bit_row: i32 = av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32;
    let width: i32 = 16;
    let height: i32 = 8;
    let col_txfm: transform_1d_sse2 = col_txfm8x8_arr[tx_type as usize];
    let row_txfm: transform_1d_sse2 = row_txfm8x16_arr[tx_type as usize];
    let mut buf: *mut __m128i;
    let mut ud_flip: i32 = 0;
    let mut lr_flip: i32 = 0;
    get_flip_cfg(tx_type, &raw mut ud_flip, &raw mut lr_flip);
    for i in 0..2 {
        if ud_flip != 0 {
            load_buffer_16bit_to_16bit_flip(
                input.offset((8 * i) as isize),
                stride,
                buf0.as_mut_ptr(),
                height,
            );
        } else {
            load_buffer_16bit_to_16bit(
                input.offset((8 * i) as isize),
                stride,
                buf0.as_mut_ptr(),
                height,
            );
        }
        round_shift_16bit(buf0.as_mut_ptr(), height, *shift as i32);
        col_txfm.unwrap()(
            buf0.as_mut_ptr() as *const __m128i,
            buf0.as_mut_ptr(),
            cos_bit_col as i8,
        );
        round_shift_16bit(buf0.as_mut_ptr(), height, *shift.offset(1) as i32);
        transpose_16bit_8x8(
            buf0.as_mut_ptr() as *const __m128i,
            buf1.as_mut_ptr().offset((8 * i) as isize),
        );
    }
    if lr_flip != 0 {
        buf = buf0.as_mut_ptr();
        flip_buf_sse2(buf1.as_mut_ptr(), buf, width);
    } else {
        buf = buf1.as_mut_ptr();
    }
    row_txfm.unwrap()(buf as *const __m128i, buf, cos_bit_row as i8);
    round_shift_16bit(buf, width, *shift.offset(2) as i32);
    store_rect_buffer_16bit_to_32bit_w8(buf as *const __m128i, output, height, width);
}

#[target_feature(enable = "sse2")]
pub unsafe fn av1_lowbd_fwd_txfm2d_16x16_sse2(
    input: *const i16,
    output: *mut i32,
    stride: i32,
    tx_type: u8,
    bd: i32,
) {
    let _ = bd;
    let mut buf0: [__m128i; 16] = core::mem::zeroed();
    let mut buf1: [__m128i; 32] = core::mem::zeroed();
    let mut shift: *const i8 = av1_fwd_txfm_shift_ls[TX_16X16 as usize];
    let txw_idx: i32 = get_txw_idx(TX_16X16 as u8);
    let txh_idx: i32 = get_txh_idx(TX_16X16 as u8);
    let cos_bit_col: i32 = av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32;
    let cos_bit_row: i32 = av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32;
    let width: i32 = 16;
    let height: i32 = 16;
    let col_txfm: transform_1d_sse2 = col_txfm8x16_arr[tx_type as usize];
    let row_txfm: transform_1d_sse2 = row_txfm8x16_arr[tx_type as usize];
    let mut ud_flip: i32 = 0;
    let mut lr_flip: i32 = 0;
    get_flip_cfg(tx_type, &raw mut ud_flip, &raw mut lr_flip);
    for i in 0..2 {
        if ud_flip != 0 {
            load_buffer_16bit_to_16bit_flip(
                input.offset((8 * i) as isize),
                stride,
                buf0.as_mut_ptr(),
                height,
            );
        } else {
            load_buffer_16bit_to_16bit(
                input.offset((8 * i) as isize),
                stride,
                buf0.as_mut_ptr(),
                height,
            );
        }
        round_shift_16bit(buf0.as_mut_ptr(), height, *shift as i32);
        col_txfm.unwrap()(
            buf0.as_mut_ptr() as *const __m128i,
            buf0.as_mut_ptr(),
            cos_bit_col as i8,
        );
        round_shift_16bit(buf0.as_mut_ptr(), height, *shift.offset(1) as i32);
        transpose_16bit_8x8(
            buf0.as_mut_ptr() as *const __m128i,
            buf1.as_mut_ptr()
                .offset((0 * width) as isize)
                .offset((8 * i) as isize),
        );
        transpose_16bit_8x8(
            buf0.as_mut_ptr().offset(8) as *const __m128i,
            buf1.as_mut_ptr()
                .offset((1 * width) as isize)
                .offset((8 * i) as isize),
        );
    }
    for i in 0..2 {
        let mut buf: *mut __m128i;
        if lr_flip != 0 {
            buf = buf0.as_mut_ptr();
            flip_buf_sse2(buf1.as_mut_ptr().offset((width * i) as isize), buf, width);
        } else {
            buf = buf1.as_mut_ptr().offset((width * i) as isize);
        }
        row_txfm.unwrap()(buf as *const __m128i, buf, cos_bit_row as i8);
        round_shift_16bit(buf, width, *shift.offset(2) as i32);
        store_buffer_16bit_to_32bit_w8(
            buf as *const __m128i,
            output.offset((8 * i) as isize),
            height,
            width,
        );
    }
}

#[target_feature(enable = "sse2")]
pub unsafe fn av1_lowbd_fwd_txfm2d_16x32_sse2(
    input: *const i16,
    output: *mut i32,
    stride: i32,
    tx_type: u8,
    bd: i32,
) {
    let _ = bd;
    let mut buf0: [__m128i; 32] = core::mem::zeroed();
    let mut buf1: [__m128i; 64] = core::mem::zeroed();
    let mut shift: *const i8 = av1_fwd_txfm_shift_ls[TX_16X32 as usize];
    let txw_idx: i32 = get_txw_idx(TX_16X32 as u8);
    let txh_idx: i32 = get_txh_idx(TX_16X32 as u8);
    let cos_bit_col: i32 = av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32;
    let cos_bit_row: i32 = av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32;
    let width: i32 = 16;
    let height: i32 = 32;
    let col_txfm: transform_1d_sse2 = col_txfm8x32_arr[tx_type as usize];
    let row_txfm: transform_1d_sse2 = row_txfm8x16_arr[tx_type as usize];
    if !col_txfm.is_none() && !row_txfm.is_none() {
        let mut ud_flip: i32 = 0;
        let mut lr_flip: i32 = 0;
        get_flip_cfg(tx_type, &raw mut ud_flip, &raw mut lr_flip);
        for i in 0..2 {
            if ud_flip != 0 {
                load_buffer_16bit_to_16bit_flip(
                    input.offset((8 * i) as isize),
                    stride,
                    buf0.as_mut_ptr(),
                    height,
                );
            } else {
                load_buffer_16bit_to_16bit(
                    input.offset((8 * i) as isize),
                    stride,
                    buf0.as_mut_ptr(),
                    height,
                );
            }
            round_shift_16bit(buf0.as_mut_ptr(), height, *shift as i32);
            col_txfm.unwrap()(
                buf0.as_mut_ptr() as *const __m128i,
                buf0.as_mut_ptr(),
                cos_bit_col as i8,
            );
            round_shift_16bit(buf0.as_mut_ptr(), height, *shift.offset(1) as i32);
            transpose_16bit_8x8(
                buf0.as_mut_ptr().offset((0 * 8) as isize) as *const __m128i,
                buf1.as_mut_ptr()
                    .offset((0 * width) as isize)
                    .offset((8 * i) as isize),
            );
            transpose_16bit_8x8(
                buf0.as_mut_ptr().offset((1 * 8) as isize) as *const __m128i,
                buf1.as_mut_ptr()
                    .offset((1 * width) as isize)
                    .offset((8 * i) as isize),
            );
            transpose_16bit_8x8(
                buf0.as_mut_ptr().offset((2 * 8) as isize) as *const __m128i,
                buf1.as_mut_ptr()
                    .offset((2 * width) as isize)
                    .offset((8 * i) as isize),
            );
            transpose_16bit_8x8(
                buf0.as_mut_ptr().offset((3 * 8) as isize) as *const __m128i,
                buf1.as_mut_ptr()
                    .offset((3 * width) as isize)
                    .offset((8 * i) as isize),
            );
        }
        for i in 0..4 {
            let mut buf: *mut __m128i;
            if lr_flip != 0 {
                buf = buf0.as_mut_ptr();
                flip_buf_sse2(buf1.as_mut_ptr().offset((width * i) as isize), buf, width);
            } else {
                buf = buf1.as_mut_ptr().offset((width * i) as isize);
            }
            row_txfm.unwrap()(buf as *const __m128i, buf, cos_bit_row as i8);
            round_shift_16bit(buf, width, *shift.offset(2) as i32);
            store_rect_buffer_16bit_to_32bit_w8(
                buf as *const __m128i,
                output.offset((8 * i) as isize),
                height,
                width,
            );
        }
    } else {
        av1_fwd_txfm2d_16x32_c(input, output, stride, tx_type, bd);
    }
}

#[target_feature(enable = "sse2")]
pub unsafe fn av1_lowbd_fwd_txfm2d_32x8_sse2(
    input: *const i16,
    output: *mut i32,
    stride: i32,
    tx_type: u8,
    bd: i32,
) {
    let _ = bd;
    let mut buf0: [__m128i; 32] = core::mem::zeroed();
    let mut buf1: [__m128i; 32] = core::mem::zeroed();
    let mut shift: *const i8 = av1_fwd_txfm_shift_ls[TX_32X8 as usize];
    let txw_idx: i32 = get_txw_idx(TX_32X8 as u8);
    let txh_idx: i32 = get_txh_idx(TX_32X8 as u8);
    let cos_bit_col: i32 = av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32;
    let cos_bit_row: i32 = av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32;
    let width: i32 = 32;
    let height: i32 = 8;
    let col_txfm: transform_1d_sse2 = col_txfm8x8_arr[tx_type as usize];
    let row_txfm: transform_1d_sse2 = row_txfm8x32_arr[tx_type as usize];
    if !col_txfm.is_none() && !row_txfm.is_none() {
        let mut ud_flip: i32 = 0;
        let mut lr_flip: i32 = 0;
        get_flip_cfg(tx_type, &raw mut ud_flip, &raw mut lr_flip);
        for i in 0..4 {
            if ud_flip != 0 {
                load_buffer_16bit_to_16bit_flip(
                    input.offset((8 * i) as isize),
                    stride,
                    buf0.as_mut_ptr(),
                    height,
                );
            } else {
                load_buffer_16bit_to_16bit(
                    input.offset((8 * i) as isize),
                    stride,
                    buf0.as_mut_ptr(),
                    height,
                );
            }
            round_shift_16bit(buf0.as_mut_ptr(), height, *shift as i32);
            col_txfm.unwrap()(
                buf0.as_mut_ptr() as *const __m128i,
                buf0.as_mut_ptr(),
                cos_bit_col as i8,
            );
            round_shift_16bit(buf0.as_mut_ptr(), height, *shift.offset(1) as i32);
            transpose_16bit_8x8(
                buf0.as_mut_ptr() as *const __m128i,
                buf1.as_mut_ptr()
                    .offset((0 * width) as isize)
                    .offset((8 * i) as isize),
            );
        }
        for i in 0..1 {
            let mut buf: *mut __m128i;
            if lr_flip != 0 {
                buf = buf0.as_mut_ptr();
                flip_buf_sse2(buf1.as_mut_ptr().offset((width * i) as isize), buf, width);
            } else {
                buf = buf1.as_mut_ptr().offset((width * i) as isize);
            }
            row_txfm.unwrap()(buf as *const __m128i, buf, cos_bit_row as i8);
            round_shift_16bit(buf, width, *shift.offset(2) as i32);
            store_buffer_16bit_to_32bit_w8(
                buf as *const __m128i,
                output.offset((8 * i) as isize),
                height,
                width,
            );
        }
    } else {
        av1_fwd_txfm2d_32x16_c(input, output, stride, tx_type, bd);
    }
}

#[target_feature(enable = "sse2")]
pub unsafe fn av1_lowbd_fwd_txfm2d_32x16_sse2(
    input: *const i16,
    output: *mut i32,
    stride: i32,
    tx_type: u8,
    bd: i32,
) {
    let _ = bd;
    let mut buf0: [__m128i; 32] = core::mem::zeroed();
    let mut buf1: [__m128i; 64] = core::mem::zeroed();
    let mut shift: *const i8 = av1_fwd_txfm_shift_ls[TX_32X16 as usize];
    let txw_idx: i32 = get_txw_idx(TX_32X16 as u8);
    let txh_idx: i32 = get_txh_idx(TX_32X16 as u8);
    let cos_bit_col: i32 = av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32;
    let cos_bit_row: i32 = av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32;
    let width: i32 = 32;
    let height: i32 = 16;
    let col_txfm: transform_1d_sse2 = col_txfm8x16_arr[tx_type as usize];
    let row_txfm: transform_1d_sse2 = row_txfm8x32_arr[tx_type as usize];
    if !col_txfm.is_none() && !row_txfm.is_none() {
        let mut ud_flip: i32 = 0;
        let mut lr_flip: i32 = 0;
        get_flip_cfg(tx_type, &raw mut ud_flip, &raw mut lr_flip);
        for i in 0..4 {
            if ud_flip != 0 {
                load_buffer_16bit_to_16bit_flip(
                    input.offset((8 * i) as isize),
                    stride,
                    buf0.as_mut_ptr(),
                    height,
                );
            } else {
                load_buffer_16bit_to_16bit(
                    input.offset((8 * i) as isize),
                    stride,
                    buf0.as_mut_ptr(),
                    height,
                );
            }
            round_shift_16bit(buf0.as_mut_ptr(), height, *shift as i32);
            col_txfm.unwrap()(
                buf0.as_mut_ptr() as *const __m128i,
                buf0.as_mut_ptr(),
                cos_bit_col as i8,
            );
            round_shift_16bit(buf0.as_mut_ptr(), height, *shift.offset(1) as i32);
            transpose_16bit_8x8(
                buf0.as_mut_ptr() as *const __m128i,
                buf1.as_mut_ptr()
                    .offset((0 * width) as isize)
                    .offset((8 * i) as isize),
            );
            transpose_16bit_8x8(
                buf0.as_mut_ptr().offset(8) as *const __m128i,
                buf1.as_mut_ptr()
                    .offset((1 * width) as isize)
                    .offset((8 * i) as isize),
            );
        }
        for i in 0..2 {
            let mut buf: *mut __m128i;
            if lr_flip != 0 {
                buf = buf0.as_mut_ptr();
                flip_buf_sse2(buf1.as_mut_ptr().offset((width * i) as isize), buf, width);
            } else {
                buf = buf1.as_mut_ptr().offset((width * i) as isize);
            }
            row_txfm.unwrap()(buf as *const __m128i, buf, cos_bit_row as i8);
            round_shift_16bit(buf, width, *shift.offset(2) as i32);
            store_rect_buffer_16bit_to_32bit_w8(
                buf as *const __m128i,
                output.offset((8 * i) as isize),
                height,
                width,
            );
        }
    } else {
        av1_fwd_txfm2d_32x16_c(input, output, stride, tx_type, bd);
    }
}

#[target_feature(enable = "sse2")]
pub unsafe fn av1_lowbd_fwd_txfm2d_32x32_sse2(
    input: *const i16,
    output: *mut i32,
    stride: i32,
    tx_type: u8,
    bd: i32,
) {
    let _ = bd;
    let mut buf0: [__m128i; 32] = core::mem::zeroed();
    let mut buf1: [__m128i; 128] = core::mem::zeroed();
    let mut shift: *const i8 = av1_fwd_txfm_shift_ls[TX_32X32 as usize];
    let txw_idx: i32 = get_txw_idx(TX_32X32 as u8);
    let txh_idx: i32 = get_txh_idx(TX_32X32 as u8);
    let cos_bit_col: i32 = av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32;
    let cos_bit_row: i32 = av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32;
    let width: i32 = 32;
    let height: i32 = 32;
    let col_txfm: transform_1d_sse2 = col_txfm8x32_arr[tx_type as usize];
    let row_txfm: transform_1d_sse2 = row_txfm8x32_arr[tx_type as usize];
    if !col_txfm.is_none() && !row_txfm.is_none() {
        let mut ud_flip: i32 = 0;
        let mut lr_flip: i32 = 0;
        get_flip_cfg(tx_type, &raw mut ud_flip, &raw mut lr_flip);
        for i in 0..4 {
            if ud_flip != 0 {
                load_buffer_16bit_to_16bit_flip(
                    input.offset((8 * i) as isize),
                    stride,
                    buf0.as_mut_ptr(),
                    height,
                );
            } else {
                load_buffer_16bit_to_16bit(
                    input.offset((8 * i) as isize),
                    stride,
                    buf0.as_mut_ptr(),
                    height,
                );
            }
            round_shift_16bit(buf0.as_mut_ptr(), height, *shift as i32);
            col_txfm.unwrap()(
                buf0.as_mut_ptr() as *const __m128i,
                buf0.as_mut_ptr(),
                cos_bit_col as i8,
            );
            round_shift_16bit(buf0.as_mut_ptr(), height, *shift.offset(1) as i32);
            transpose_16bit_8x8(
                buf0.as_mut_ptr().offset((0 * 8) as isize) as *const __m128i,
                buf1.as_mut_ptr()
                    .offset((0 * width) as isize)
                    .offset((8 * i) as isize),
            );
            transpose_16bit_8x8(
                buf0.as_mut_ptr().offset((1 * 8) as isize) as *const __m128i,
                buf1.as_mut_ptr()
                    .offset((1 * width) as isize)
                    .offset((8 * i) as isize),
            );
            transpose_16bit_8x8(
                buf0.as_mut_ptr().offset((2 * 8) as isize) as *const __m128i,
                buf1.as_mut_ptr()
                    .offset((2 * width) as isize)
                    .offset((8 * i) as isize),
            );
            transpose_16bit_8x8(
                buf0.as_mut_ptr().offset((3 * 8) as isize) as *const __m128i,
                buf1.as_mut_ptr()
                    .offset((3 * width) as isize)
                    .offset((8 * i) as isize),
            );
        }
        for i in 0..4 {
            let mut buf: *mut __m128i;
            if lr_flip != 0 {
                buf = buf0.as_mut_ptr();
                flip_buf_sse2(buf1.as_mut_ptr().offset((width * i) as isize), buf, width);
            } else {
                buf = buf1.as_mut_ptr().offset((width * i) as isize);
            }
            row_txfm.unwrap()(buf as *const __m128i, buf, cos_bit_row as i8);
            round_shift_16bit(buf, width, *shift.offset(2) as i32);
            store_buffer_16bit_to_32bit_w8(
                buf as *const __m128i,
                output.offset((8 * i) as isize),
                height,
                width,
            );
        }
    } else {
        av1_fwd_txfm2d_32x32_c(input, output, stride, tx_type, bd);
    }
}

#[target_feature(enable = "sse2")]
pub unsafe fn av1_lowbd_fwd_txfm2d_64x16_sse2(
    input: *const i16,
    output: *mut i32,
    stride: i32,
    tx_type: u8,
    bd: i32,
) {
    let _ = bd;
    let _ = tx_type;
    debug_assert!((tx_type as i32) == DCT_DCT);
    let tx_size: u8 = TX_64X16 as u8;
    let mut buf0: [__m128i; 64] = core::mem::zeroed();
    let mut buf1: [__m128i; 128] = core::mem::zeroed();
    let mut shift: *const i8 = av1_fwd_txfm_shift_ls[tx_size as usize];
    let txw_idx: i32 = get_txw_idx(tx_size);
    let txh_idx: i32 = get_txh_idx(tx_size);
    let cos_bit_col: i32 = av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32;
    let cos_bit_row: i32 = av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32;
    let width: i32 = tx_size_wide[tx_size as usize];
    let height: i32 = tx_size_high[tx_size as usize];
    let col_txfm: transform_1d_sse2 = Some(fdct8x16_new_sse2);
    let row_txfm: transform_1d_sse2 = Some(av1_fdct8x64_new_sse2);
    let width_div8: i32 = width >> 3;
    let height_div8: i32 = height >> 3;
    for i in 0..width_div8 {
        load_buffer_16bit_to_16bit(
            input.offset((8 * i) as isize),
            stride,
            buf0.as_mut_ptr(),
            height,
        );
        round_shift_16bit(buf0.as_mut_ptr(), height, *shift as i32);
        col_txfm.unwrap()(
            buf0.as_mut_ptr() as *const __m128i,
            buf0.as_mut_ptr(),
            cos_bit_col as i8,
        );
        round_shift_16bit(buf0.as_mut_ptr(), height, *shift.offset(1) as i32);
        for j in 0..height_div8 {
            transpose_16bit_8x8(
                buf0.as_mut_ptr().offset((j * 8) as isize) as *const __m128i,
                buf1.as_mut_ptr()
                    .offset((j * width) as isize)
                    .offset((8 * i) as isize),
            );
        }
    }
    for i in 0..height_div8 {
        let mut buf: *mut __m128i = buf1.as_mut_ptr().offset((width * i) as isize);
        row_txfm.unwrap()(buf as *const __m128i, buf, cos_bit_row as i8);
        round_shift_16bit(buf, width, *shift.offset(2) as i32);
        store_buffer_16bit_to_32bit_w8(
            buf as *const __m128i,
            output.offset((8 * i) as isize),
            16,
            32,
        );
    }
    // Zero out the bottom 16x32 area.
    core::ptr::write_bytes(
        output.offset((16 * 32) as isize) as *mut u8,
        0,
        (16 * 32) as usize * core::mem::size_of::<i32>(),
    );
}

#[target_feature(enable = "sse2")]
pub unsafe fn av1_lowbd_fwd_txfm2d_16x64_sse2(
    input: *const i16,
    output: *mut i32,
    stride: i32,
    tx_type: u8,
    bd: i32,
) {
    let _ = bd;
    let _ = tx_type;
    debug_assert!((tx_type as i32) == DCT_DCT);
    let tx_size: u8 = TX_16X64 as u8;
    let mut buf0: [__m128i; 64] = core::mem::zeroed();
    let mut buf1: [__m128i; 128] = core::mem::zeroed();
    let mut shift: *const i8 = av1_fwd_txfm_shift_ls[tx_size as usize];
    let txw_idx: i32 = get_txw_idx(tx_size);
    let txh_idx: i32 = get_txh_idx(tx_size);
    let cos_bit_col: i32 = av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32;
    let cos_bit_row: i32 = av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32;
    let width: i32 = tx_size_wide[tx_size as usize];
    let height: i32 = tx_size_high[tx_size as usize];
    let col_txfm: transform_1d_sse2 = Some(av1_fdct8x64_new_sse2);
    let row_txfm: transform_1d_sse2 = Some(fdct8x16_new_sse2);
    let width_div8: i32 = width >> 3;
    let height_div8: i32 = height >> 3;
    for i in 0..width_div8 {
        load_buffer_16bit_to_16bit(
            input.offset((8 * i) as isize),
            stride,
            buf0.as_mut_ptr(),
            height,
        );
        round_shift_16bit(buf0.as_mut_ptr(), height, *shift as i32);
        col_txfm.unwrap()(
            buf0.as_mut_ptr() as *const __m128i,
            buf0.as_mut_ptr(),
            cos_bit_col as i8,
        );
        round_shift_16bit(buf0.as_mut_ptr(), height, *shift.offset(1) as i32);
        for j in 0..height_div8 {
            transpose_16bit_8x8(
                buf0.as_mut_ptr().offset((j * 8) as isize) as *const __m128i,
                buf1.as_mut_ptr()
                    .offset((j * width) as isize)
                    .offset((8 * i) as isize),
            );
        }
    }
    {
        let mut i: i32 = 0;
        while i < AOMMIN(4, height_div8) {
            let mut buf: *mut __m128i = buf1.as_mut_ptr().offset((width * i) as isize);
            row_txfm.unwrap()(buf as *const __m128i, buf, cos_bit_row as i8);
            round_shift_16bit(buf, width, *shift.offset(2) as i32);
            store_buffer_16bit_to_32bit_w8(
                buf as *const __m128i,
                output.offset((8 * i) as isize),
                32,
                16,
            );
            i += 1;
        }
    }
}

// Include top-level function only for 32-bit x86, to support Valgrind.
// For normal use, we require SSE4.1, so av1_lowbd_fwd_txfm_sse4_1 will be used
// instead of this function. However, 32-bit Valgrind does not support SSE4.1,
// so we include a fallback to SSE2 to improve performance
// 4x4 transform
// 8x8 transform
// 16x16 transform
// 32x32 transform
// 64x64 transform
// 4x8 transform
// 8x4 transform
// 8x16 transform
// 16x8 transform
// 16x32 transform
// 32x16 transform
// 32x64 transform
// 64x32 transform
// 4x16 transform
// 16x4 transform
// 8x32 transform
// 32x8 transform
// 16x64 transform
// 64x16 transform
// AOM_ARCH_X86
