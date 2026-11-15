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

// Translated from upstream/aom/av1/common/x86/av1_inv_txfm_avx2.h and
// av1_inv_txfm_avx2.c (ADR 0056, ADR 0058). Upstream compiles the .c file
// with -mavx2. The header macro btf_16_w16_0_avx2 was written by hand with the
// same expansion; the header's inline functions and the .c file were
// transliterated with a typing-aware C-subset printer (not C2Rust) and
// reviewed against the C function by function: the same sequence of
// intrinsic and helper calls, and the same identifiers, constants, and
// operators in order, as ADR 0058 describes for the high-bitdepth transforms.
//
// Translation notes:
// - Every function carries `avx2`. The RTCD specialization
//   av1_inv_txfm_add_avx2 is exported under its upstream name;
//   av1_lowbd_inv_txfm2d_add_avx2, which the original test instantiates, is a
//   public function with the C ABI.
// - The sizes and types that upstream hands to the SSSE3 kernels
//   (av1_lowbd_inv_txfm2d_add_ssse3 and its identity variants, the 8-point
//   1D transforms) call their translations in av1_inv_txfm_ssse3.rs.
// - Array parameters stay raw pointers; local arrays are Rust arrays and
//   start zeroed. C integer conversions are explicit (`as`); `switch` becomes
//   `match`; `assert` becomes `debug_assert!`; calling a NULL transform entry
//   panics through `Option::unwrap`.
// - Immediate operands become const generics. Shift counts that are run-time
//   `int` values use the count-register intrinsics, which is what GCC and
//   Clang emit for a non-constant count.
#![allow(
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut,
    unused_parens
)]
#![allow(unused_variables, dead_code, clippy::all)]

use core::arch::x86_64::*;

use crate::av1__common__av1_inv_txfm2d::av1_inv_txfm_shift_ls;
use crate::av1__common__idct::{av1_inv_txfm_add_c, TxfmParam};
use crate::simd::aom_dsp__x86__transpose_sse2::{
    transpose_16bit_4x4, transpose_16bit_4x8, transpose_16bit_8x4, transpose_16bit_8x8,
};
use crate::simd::aom_dsp__x86__txfm_common_avx2::{
    btf_16_adds_subs_avx2, btf_16_adds_subs_out_avx2, btf_16_w16_avx2, btf_32_add_sub_avx2,
    btf_32_add_sub_out_avx2, flip_buf_avx2, load_32bit_to_16bit_w16_avx2,
    load_buffer_32bit_to_16bit_w16_avx2, pair_set_w16_epi16, round_shift_16bit_w16_avx2,
    transpose_16bit_16x16_avx2, transpose_16bit_16x8_avx2,
};
use crate::simd::aom_dsp__x86__txfm_common_sse2::pair_set_epi16;
use crate::simd::av1__common__x86__av1_inv_txfm_ssse3::*;
use crate::simd::av1__common__x86__av1_txfm_sse2::{
    btf_16_4p_sse2, btf_16_sse2, flip_buf_sse2, load_32bit_to_16bit, load_buffer_32bit_to_16bit,
    round_shift_16bit,
};

// half input is zero
macro_rules! btf_16_w16_0_avx2 {
    ($w0:expr, $w1:expr, $in:expr, $out0:expr, $out1:expr) => {{
        let _w0: __m256i = _mm256_set1_epi16(($w0 * 8) as i16);
        let _w1: __m256i = _mm256_set1_epi16(($w1 * 8) as i16);
        let _in: __m256i = $in;
        $out0 = _mm256_mulhrs_epi16(_in, _w0);
        $out1 = _mm256_mulhrs_epi16(_in, _w1);
    }};
}

pub type transform_1d_avx2 = Option<unsafe fn(*const __m256i, *mut __m256i)>;

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn round_shift_avx2(input: *const __m256i, output: *mut __m256i, size: i32) {
    let scale: __m256i = _mm256_set1_epi16((NewInvSqrt2 * 8) as i16);
    for i in 0..size {
        *output.offset(i as isize) = _mm256_mulhrs_epi16(*input.offset(i as isize), scale);
    }
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn write_recon_w16_avx2(res: __m256i, output: *mut u8) {
    let mut pred: __m128i = _mm_loadu_si128(output as *const __m128i);
    let mut u: __m256i = _mm256_adds_epi16(_mm256_cvtepu8_epi16(pred), res);
    let mut y: __m128i =
        _mm256_castsi256_si128(_mm256_permute4x64_epi64::<168>(_mm256_packus_epi16(u, u)));
    _mm_storeu_si128(output as *mut __m128i, y);
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn lowbd_write_buffer_16xn_avx2(
    in_: *mut __m256i,
    output: *mut u8,
    stride: i32,
    flipud: i32,
    height: i32,
) {
    let mut j: i32 = if flipud != 0 { height - 1 } else { 0 };
    let step: i32 = if flipud != 0 { -1 } else { 1 };
    {
        let mut i: i32 = 0;
        while i < height {
            write_recon_w16_avx2(
                *in_.offset(j as isize),
                output.offset((i * stride) as isize),
            );
            i += 1;
            j += step;
        }
    }
}

// ---------------------------------------------------------------------------
// av1/common/x86/av1_inv_txfm_avx2.c

// TODO(venkatsanampudi@ittiam.com): move this to header file
// Sqrt2, Sqrt2^2, Sqrt2^3, Sqrt2^4, Sqrt2^5
static NewSqrt2list: [i32; (TX_SIZES) as usize] = [5793, 2 * 4096, 2 * 5793, 4 * 4096, 4 * 5793];

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn idct16_stage5_avx2(x1: *mut __m256i, cospi: *const i32, _r: __m256i, cos_bit: i8) {
    let cospi_m32_p32: __m256i =
        pair_set_w16_epi16(-*cospi.offset(32) as i16, *cospi.offset(32) as i16);
    let cospi_p32_p32: __m256i =
        pair_set_w16_epi16(*cospi.offset(32) as i16, *cospi.offset(32) as i16);
    btf_16_adds_subs_avx2(x1, x1.offset(3));
    btf_16_adds_subs_avx2(x1.offset(1), x1.offset(2));
    btf_16_w16_avx2(
        cospi_m32_p32,
        cospi_p32_p32,
        x1.offset(5),
        x1.offset(6),
        _r,
        cos_bit as i32,
    );
    btf_16_adds_subs_avx2(x1.offset(8), x1.offset(11));
    btf_16_adds_subs_avx2(x1.offset(9), x1.offset(10));
    btf_16_adds_subs_avx2(x1.offset(15), x1.offset(12));
    btf_16_adds_subs_avx2(x1.offset(14), x1.offset(13));
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn idct16_stage6_avx2(x: *mut __m256i, cospi: *const i32, _r: __m256i, cos_bit: i8) {
    let cospi_m32_p32: __m256i =
        pair_set_w16_epi16(-*cospi.offset(32) as i16, *cospi.offset(32) as i16);
    let cospi_p32_p32: __m256i =
        pair_set_w16_epi16(*cospi.offset(32) as i16, *cospi.offset(32) as i16);
    btf_16_adds_subs_avx2(x, x.offset(7));
    btf_16_adds_subs_avx2(x.offset(1), x.offset(6));
    btf_16_adds_subs_avx2(x.offset(2), x.offset(5));
    btf_16_adds_subs_avx2(x.offset(3), x.offset(4));
    btf_16_w16_avx2(
        cospi_m32_p32,
        cospi_p32_p32,
        x.offset(10),
        x.offset(13),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m32_p32,
        cospi_p32_p32,
        x.offset(11),
        x.offset(12),
        _r,
        cos_bit as i32,
    );
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn idct16_stage7_avx2(output: *mut __m256i, x1: *mut __m256i) {
    btf_16_adds_subs_out_avx2(output, output.offset(15), *x1, *x1.offset(15));
    btf_16_adds_subs_out_avx2(
        output.offset(1),
        output.offset(14),
        *x1.offset(1),
        *x1.offset(14),
    );
    btf_16_adds_subs_out_avx2(
        output.offset(2),
        output.offset(13),
        *x1.offset(2),
        *x1.offset(13),
    );
    btf_16_adds_subs_out_avx2(
        output.offset(3),
        output.offset(12),
        *x1.offset(3),
        *x1.offset(12),
    );
    btf_16_adds_subs_out_avx2(
        output.offset(4),
        output.offset(11),
        *x1.offset(4),
        *x1.offset(11),
    );
    btf_16_adds_subs_out_avx2(
        output.offset(5),
        output.offset(10),
        *x1.offset(5),
        *x1.offset(10),
    );
    btf_16_adds_subs_out_avx2(
        output.offset(6),
        output.offset(9),
        *x1.offset(6),
        *x1.offset(9),
    );
    btf_16_adds_subs_out_avx2(
        output.offset(7),
        output.offset(8),
        *x1.offset(7),
        *x1.offset(8),
    );
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn idct16_avx2(input: *const __m256i, output: *mut __m256i) {
    let mut cospi: *const i32 = cospi_arr(INV_COS_BIT);
    let _r: __m256i = _mm256_set1_epi32(1 << (INV_COS_BIT - 1));
    let mut cospi_p60_m04: __m256i =
        pair_set_w16_epi16(*cospi.offset(60) as i16, -*cospi.offset(4) as i16);
    let mut cospi_p04_p60: __m256i =
        pair_set_w16_epi16(*cospi.offset(4) as i16, *cospi.offset(60) as i16);
    let mut cospi_p28_m36: __m256i =
        pair_set_w16_epi16(*cospi.offset(28) as i16, -*cospi.offset(36) as i16);
    let mut cospi_p36_p28: __m256i =
        pair_set_w16_epi16(*cospi.offset(36) as i16, *cospi.offset(28) as i16);
    let mut cospi_p44_m20: __m256i =
        pair_set_w16_epi16(*cospi.offset(44) as i16, -*cospi.offset(20) as i16);
    let mut cospi_p20_p44: __m256i =
        pair_set_w16_epi16(*cospi.offset(20) as i16, *cospi.offset(44) as i16);
    let mut cospi_p12_m52: __m256i =
        pair_set_w16_epi16(*cospi.offset(12) as i16, -*cospi.offset(52) as i16);
    let mut cospi_p52_p12: __m256i =
        pair_set_w16_epi16(*cospi.offset(52) as i16, *cospi.offset(12) as i16);
    let mut cospi_p56_m08: __m256i =
        pair_set_w16_epi16(*cospi.offset(56) as i16, -*cospi.offset(8) as i16);
    let mut cospi_p08_p56: __m256i =
        pair_set_w16_epi16(*cospi.offset(8) as i16, *cospi.offset(56) as i16);
    let mut cospi_p24_m40: __m256i =
        pair_set_w16_epi16(*cospi.offset(24) as i16, -*cospi.offset(40) as i16);
    let mut cospi_p40_p24: __m256i =
        pair_set_w16_epi16(*cospi.offset(40) as i16, *cospi.offset(24) as i16);
    let mut cospi_p32_p32: __m256i =
        pair_set_w16_epi16(*cospi.offset(32) as i16, *cospi.offset(32) as i16);
    let mut cospi_p32_m32: __m256i =
        pair_set_w16_epi16(*cospi.offset(32) as i16, -*cospi.offset(32) as i16);
    let mut cospi_p48_m16: __m256i =
        pair_set_w16_epi16(*cospi.offset(48) as i16, -*cospi.offset(16) as i16);
    let mut cospi_p16_p48: __m256i =
        pair_set_w16_epi16(*cospi.offset(16) as i16, *cospi.offset(48) as i16);
    let mut cospi_m16_p48: __m256i =
        pair_set_w16_epi16(-*cospi.offset(16) as i16, *cospi.offset(48) as i16);
    let mut cospi_p48_p16: __m256i =
        pair_set_w16_epi16(*cospi.offset(48) as i16, *cospi.offset(16) as i16);
    let mut cospi_m48_m16: __m256i =
        pair_set_w16_epi16(-*cospi.offset(48) as i16, -*cospi.offset(16) as i16);
    // stage 1
    let mut x1: [__m256i; 16] = core::mem::zeroed();
    x1[0] = *input;
    x1[1] = *input.offset(8);
    x1[2] = *input.offset(4);
    x1[3] = *input.offset(12);
    x1[4] = *input.offset(2);
    x1[5] = *input.offset(10);
    x1[6] = *input.offset(6);
    x1[7] = *input.offset(14);
    x1[8] = *input.offset(1);
    x1[9] = *input.offset(9);
    x1[10] = *input.offset(5);
    x1[11] = *input.offset(13);
    x1[12] = *input.offset(3);
    x1[13] = *input.offset(11);
    x1[14] = *input.offset(7);
    x1[15] = *input.offset(15);
    // stage 2
    btf_16_w16_avx2(
        cospi_p60_m04,
        cospi_p04_p60,
        x1.as_mut_ptr().offset(8),
        x1.as_mut_ptr().offset(15),
        _r,
        INV_COS_BIT,
    );
    btf_16_w16_avx2(
        cospi_p28_m36,
        cospi_p36_p28,
        x1.as_mut_ptr().offset(9),
        x1.as_mut_ptr().offset(14),
        _r,
        INV_COS_BIT,
    );
    btf_16_w16_avx2(
        cospi_p44_m20,
        cospi_p20_p44,
        x1.as_mut_ptr().offset(10),
        x1.as_mut_ptr().offset(13),
        _r,
        INV_COS_BIT,
    );
    btf_16_w16_avx2(
        cospi_p12_m52,
        cospi_p52_p12,
        x1.as_mut_ptr().offset(11),
        x1.as_mut_ptr().offset(12),
        _r,
        INV_COS_BIT,
    );
    // stage 3
    btf_16_w16_avx2(
        cospi_p56_m08,
        cospi_p08_p56,
        x1.as_mut_ptr().offset(4),
        x1.as_mut_ptr().offset(7),
        _r,
        INV_COS_BIT,
    );
    btf_16_w16_avx2(
        cospi_p24_m40,
        cospi_p40_p24,
        x1.as_mut_ptr().offset(5),
        x1.as_mut_ptr().offset(6),
        _r,
        INV_COS_BIT,
    );
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(8), x1.as_mut_ptr().offset(9));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(11), x1.as_mut_ptr().offset(10));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(12), x1.as_mut_ptr().offset(13));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(15), x1.as_mut_ptr().offset(14));
    // stage 4
    btf_16_w16_avx2(
        cospi_p32_p32,
        cospi_p32_m32,
        x1.as_mut_ptr(),
        x1.as_mut_ptr().offset(1),
        _r,
        INV_COS_BIT,
    );
    btf_16_w16_avx2(
        cospi_p48_m16,
        cospi_p16_p48,
        x1.as_mut_ptr().offset(2),
        x1.as_mut_ptr().offset(3),
        _r,
        INV_COS_BIT,
    );
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(4), x1.as_mut_ptr().offset(5));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(7), x1.as_mut_ptr().offset(6));
    btf_16_w16_avx2(
        cospi_m16_p48,
        cospi_p48_p16,
        x1.as_mut_ptr().offset(9),
        x1.as_mut_ptr().offset(14),
        _r,
        INV_COS_BIT,
    );
    btf_16_w16_avx2(
        cospi_m48_m16,
        cospi_m16_p48,
        x1.as_mut_ptr().offset(10),
        x1.as_mut_ptr().offset(13),
        _r,
        INV_COS_BIT,
    );
    idct16_stage5_avx2(x1.as_mut_ptr(), cospi, _r, INV_COS_BIT as i8);
    idct16_stage6_avx2(x1.as_mut_ptr(), cospi, _r, INV_COS_BIT as i8);
    idct16_stage7_avx2(output, x1.as_mut_ptr());
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn idct16_low8_avx2(input: *const __m256i, output: *mut __m256i) {
    let mut cospi: *const i32 = cospi_arr(INV_COS_BIT);
    let _r: __m256i = _mm256_set1_epi32(1 << (INV_COS_BIT - 1));
    let cospi_m16_p48: __m256i =
        pair_set_w16_epi16(-*cospi.offset(16) as i16, *cospi.offset(48) as i16);
    let cospi_p48_p16: __m256i =
        pair_set_w16_epi16(*cospi.offset(48) as i16, *cospi.offset(16) as i16);
    let cospi_m48_m16: __m256i =
        pair_set_w16_epi16(-*cospi.offset(48) as i16, -*cospi.offset(16) as i16);
    // stage 1
    let mut x1: [__m256i; 16] = core::mem::zeroed();
    x1[0] = *input;
    x1[2] = *input.offset(4);
    x1[4] = *input.offset(2);
    x1[6] = *input.offset(6);
    x1[8] = *input.offset(1);
    x1[10] = *input.offset(5);
    x1[12] = *input.offset(3);
    x1[14] = *input.offset(7);
    // stage 2
    btf_16_w16_0_avx2!(*cospi.offset(60), *cospi.offset(4), x1[8], x1[8], x1[15]);
    btf_16_w16_0_avx2!(-*cospi.offset(36), *cospi.offset(28), x1[14], x1[9], x1[14]);
    btf_16_w16_0_avx2!(*cospi.offset(44), *cospi.offset(20), x1[10], x1[10], x1[13]);
    btf_16_w16_0_avx2!(
        -*cospi.offset(52),
        *cospi.offset(12),
        x1[12],
        x1[11],
        x1[12]
    );
    // stage 3
    btf_16_w16_0_avx2!(*cospi.offset(56), *cospi.offset(8), x1[4], x1[4], x1[7]);
    btf_16_w16_0_avx2!(-*cospi.offset(40), *cospi.offset(24), x1[6], x1[5], x1[6]);
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(8), x1.as_mut_ptr().offset(9));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(11), x1.as_mut_ptr().offset(10));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(12), x1.as_mut_ptr().offset(13));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(15), x1.as_mut_ptr().offset(14));
    // stage 4
    btf_16_w16_0_avx2!(*cospi.offset(32), *cospi.offset(32), x1[0], x1[0], x1[1]);
    btf_16_w16_0_avx2!(*cospi.offset(48), *cospi.offset(16), x1[2], x1[2], x1[3]);
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(4), x1.as_mut_ptr().offset(5));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(7), x1.as_mut_ptr().offset(6));
    btf_16_w16_avx2(
        cospi_m16_p48,
        cospi_p48_p16,
        x1.as_mut_ptr().offset(9),
        x1.as_mut_ptr().offset(14),
        _r,
        INV_COS_BIT,
    );
    btf_16_w16_avx2(
        cospi_m48_m16,
        cospi_m16_p48,
        x1.as_mut_ptr().offset(10),
        x1.as_mut_ptr().offset(13),
        _r,
        INV_COS_BIT,
    );
    idct16_stage5_avx2(x1.as_mut_ptr(), cospi, _r, INV_COS_BIT as i8);
    idct16_stage6_avx2(x1.as_mut_ptr(), cospi, _r, INV_COS_BIT as i8);
    idct16_stage7_avx2(output, x1.as_mut_ptr());
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn idct16_low1_avx2(input: *const __m256i, output: *mut __m256i) {
    let mut cospi: *const i32 = cospi_arr(INV_COS_BIT);
    // stage 1
    let mut x1: [__m256i; 2] = core::mem::zeroed();
    x1[0] = *input;
    // stage 2
    // stage 3
    // stage 4
    btf_16_w16_0_avx2!(*cospi.offset(32), *cospi.offset(32), x1[0], x1[0], x1[1]);
    // stage 5
    // stage 6
    *output = x1[0];
    *output.offset(1) = x1[1];
    *output.offset(2) = x1[1];
    *output.offset(3) = x1[0];
    *output.offset(4) = x1[0];
    *output.offset(5) = x1[1];
    *output.offset(6) = x1[1];
    *output.offset(7) = x1[0];
    *output.offset(8) = x1[0];
    *output.offset(9) = x1[1];
    *output.offset(10) = x1[1];
    *output.offset(11) = x1[0];
    *output.offset(12) = x1[0];
    *output.offset(13) = x1[1];
    *output.offset(14) = x1[1];
    *output.offset(15) = x1[0];
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn iadst16_stage3_avx2(x: *mut __m256i) {
    btf_16_adds_subs_avx2(x, x.offset(8));
    btf_16_adds_subs_avx2(x.offset(1), x.offset(9));
    btf_16_adds_subs_avx2(x.offset(2), x.offset(10));
    btf_16_adds_subs_avx2(x.offset(3), x.offset(11));
    btf_16_adds_subs_avx2(x.offset(4), x.offset(12));
    btf_16_adds_subs_avx2(x.offset(5), x.offset(13));
    btf_16_adds_subs_avx2(x.offset(6), x.offset(14));
    btf_16_adds_subs_avx2(x.offset(7), x.offset(15));
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn iadst16_stage4_avx2(x: *mut __m256i, cospi: *const i32, _r: __m256i, cos_bit: i8) {
    let cospi_p08_p56: __m256i =
        pair_set_w16_epi16(*cospi.offset(8) as i16, *cospi.offset(56) as i16);
    let cospi_p56_m08: __m256i =
        pair_set_w16_epi16(*cospi.offset(56) as i16, -*cospi.offset(8) as i16);
    let cospi_p40_p24: __m256i =
        pair_set_w16_epi16(*cospi.offset(40) as i16, *cospi.offset(24) as i16);
    let cospi_p24_m40: __m256i =
        pair_set_w16_epi16(*cospi.offset(24) as i16, -*cospi.offset(40) as i16);
    let cospi_m56_p08: __m256i =
        pair_set_w16_epi16(-*cospi.offset(56) as i16, *cospi.offset(8) as i16);
    let cospi_m24_p40: __m256i =
        pair_set_w16_epi16(-*cospi.offset(24) as i16, *cospi.offset(40) as i16);
    btf_16_w16_avx2(
        cospi_p08_p56,
        cospi_p56_m08,
        x.offset(8),
        x.offset(9),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_p40_p24,
        cospi_p24_m40,
        x.offset(10),
        x.offset(11),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m56_p08,
        cospi_p08_p56,
        x.offset(12),
        x.offset(13),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m24_p40,
        cospi_p40_p24,
        x.offset(14),
        x.offset(15),
        _r,
        cos_bit as i32,
    );
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn iadst16_stage5_avx2(x: *mut __m256i) {
    btf_16_adds_subs_avx2(x, x.offset(4));
    btf_16_adds_subs_avx2(x.offset(1), x.offset(5));
    btf_16_adds_subs_avx2(x.offset(2), x.offset(6));
    btf_16_adds_subs_avx2(x.offset(3), x.offset(7));
    btf_16_adds_subs_avx2(x.offset(8), x.offset(12));
    btf_16_adds_subs_avx2(x.offset(9), x.offset(13));
    btf_16_adds_subs_avx2(x.offset(10), x.offset(14));
    btf_16_adds_subs_avx2(x.offset(11), x.offset(15));
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn iadst16_stage6_avx2(x: *mut __m256i, cospi: *const i32, _r: __m256i, cos_bit: i8) {
    let cospi_p16_p48: __m256i =
        pair_set_w16_epi16(*cospi.offset(16) as i16, *cospi.offset(48) as i16);
    let cospi_p48_m16: __m256i =
        pair_set_w16_epi16(*cospi.offset(48) as i16, -*cospi.offset(16) as i16);
    let cospi_m48_p16: __m256i =
        pair_set_w16_epi16(-*cospi.offset(48) as i16, *cospi.offset(16) as i16);
    btf_16_w16_avx2(
        cospi_p16_p48,
        cospi_p48_m16,
        x.offset(4),
        x.offset(5),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m48_p16,
        cospi_p16_p48,
        x.offset(6),
        x.offset(7),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_p16_p48,
        cospi_p48_m16,
        x.offset(12),
        x.offset(13),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m48_p16,
        cospi_p16_p48,
        x.offset(14),
        x.offset(15),
        _r,
        cos_bit as i32,
    );
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn iadst16_stage7_avx2(x: *mut __m256i) {
    btf_16_adds_subs_avx2(x, x.offset(2));
    btf_16_adds_subs_avx2(x.offset(1), x.offset(3));
    btf_16_adds_subs_avx2(x.offset(4), x.offset(6));
    btf_16_adds_subs_avx2(x.offset(5), x.offset(7));
    btf_16_adds_subs_avx2(x.offset(8), x.offset(10));
    btf_16_adds_subs_avx2(x.offset(9), x.offset(11));
    btf_16_adds_subs_avx2(x.offset(12), x.offset(14));
    btf_16_adds_subs_avx2(x.offset(13), x.offset(15));
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn iadst16_stage8_avx2(x1: *mut __m256i, cospi: *const i32, _r: __m256i, cos_bit: i8) {
    let cospi_p32_p32: __m256i =
        pair_set_w16_epi16(*cospi.offset(32) as i16, *cospi.offset(32) as i16);
    let cospi_p32_m32: __m256i =
        pair_set_w16_epi16(*cospi.offset(32) as i16, -*cospi.offset(32) as i16);
    btf_16_w16_avx2(
        cospi_p32_p32,
        cospi_p32_m32,
        x1.offset(2),
        x1.offset(3),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_p32_p32,
        cospi_p32_m32,
        x1.offset(6),
        x1.offset(7),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_p32_p32,
        cospi_p32_m32,
        x1.offset(10),
        x1.offset(11),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_p32_p32,
        cospi_p32_m32,
        x1.offset(14),
        x1.offset(15),
        _r,
        cos_bit as i32,
    );
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn iadst16_stage9_avx2(output: *mut __m256i, x1: *mut __m256i) {
    let __zero: __m256i = _mm256_setzero_si256();
    *output = *x1;
    *output.offset(1) = _mm256_subs_epi16(__zero, *x1.offset(8));
    *output.offset(2) = *x1.offset(12);
    *output.offset(3) = _mm256_subs_epi16(__zero, *x1.offset(4));
    *output.offset(4) = *x1.offset(6);
    *output.offset(5) = _mm256_subs_epi16(__zero, *x1.offset(14));
    *output.offset(6) = *x1.offset(10);
    *output.offset(7) = _mm256_subs_epi16(__zero, *x1.offset(2));
    *output.offset(8) = *x1.offset(3);
    *output.offset(9) = _mm256_subs_epi16(__zero, *x1.offset(11));
    *output.offset(10) = *x1.offset(15);
    *output.offset(11) = _mm256_subs_epi16(__zero, *x1.offset(7));
    *output.offset(12) = *x1.offset(5);
    *output.offset(13) = _mm256_subs_epi16(__zero, *x1.offset(13));
    *output.offset(14) = *x1.offset(9);
    *output.offset(15) = _mm256_subs_epi16(__zero, *x1.offset(1));
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn iadst16_avx2(input: *const __m256i, output: *mut __m256i) {
    let mut cospi: *const i32 = cospi_arr(INV_COS_BIT);
    let _r: __m256i = _mm256_set1_epi32(1 << (INV_COS_BIT - 1));
    let mut cospi_p02_p62: __m256i =
        pair_set_w16_epi16(*cospi.offset(2) as i16, *cospi.offset(62) as i16);
    let mut cospi_p62_m02: __m256i =
        pair_set_w16_epi16(*cospi.offset(62) as i16, -*cospi.offset(2) as i16);
    let mut cospi_p10_p54: __m256i =
        pair_set_w16_epi16(*cospi.offset(10) as i16, *cospi.offset(54) as i16);
    let mut cospi_p54_m10: __m256i =
        pair_set_w16_epi16(*cospi.offset(54) as i16, -*cospi.offset(10) as i16);
    let mut cospi_p18_p46: __m256i =
        pair_set_w16_epi16(*cospi.offset(18) as i16, *cospi.offset(46) as i16);
    let mut cospi_p46_m18: __m256i =
        pair_set_w16_epi16(*cospi.offset(46) as i16, -*cospi.offset(18) as i16);
    let mut cospi_p26_p38: __m256i =
        pair_set_w16_epi16(*cospi.offset(26) as i16, *cospi.offset(38) as i16);
    let mut cospi_p38_m26: __m256i =
        pair_set_w16_epi16(*cospi.offset(38) as i16, -*cospi.offset(26) as i16);
    let mut cospi_p34_p30: __m256i =
        pair_set_w16_epi16(*cospi.offset(34) as i16, *cospi.offset(30) as i16);
    let mut cospi_p30_m34: __m256i =
        pair_set_w16_epi16(*cospi.offset(30) as i16, -*cospi.offset(34) as i16);
    let mut cospi_p42_p22: __m256i =
        pair_set_w16_epi16(*cospi.offset(42) as i16, *cospi.offset(22) as i16);
    let mut cospi_p22_m42: __m256i =
        pair_set_w16_epi16(*cospi.offset(22) as i16, -*cospi.offset(42) as i16);
    let mut cospi_p50_p14: __m256i =
        pair_set_w16_epi16(*cospi.offset(50) as i16, *cospi.offset(14) as i16);
    let mut cospi_p14_m50: __m256i =
        pair_set_w16_epi16(*cospi.offset(14) as i16, -*cospi.offset(50) as i16);
    let mut cospi_p58_p06: __m256i =
        pair_set_w16_epi16(*cospi.offset(58) as i16, *cospi.offset(6) as i16);
    let mut cospi_p06_m58: __m256i =
        pair_set_w16_epi16(*cospi.offset(6) as i16, -*cospi.offset(58) as i16);
    // stage 1
    let mut x1: [__m256i; 16] = core::mem::zeroed();
    x1[0] = *input.offset(15);
    x1[1] = *input;
    x1[2] = *input.offset(13);
    x1[3] = *input.offset(2);
    x1[4] = *input.offset(11);
    x1[5] = *input.offset(4);
    x1[6] = *input.offset(9);
    x1[7] = *input.offset(6);
    x1[8] = *input.offset(7);
    x1[9] = *input.offset(8);
    x1[10] = *input.offset(5);
    x1[11] = *input.offset(10);
    x1[12] = *input.offset(3);
    x1[13] = *input.offset(12);
    x1[14] = *input.offset(1);
    x1[15] = *input.offset(14);
    // stage 2
    btf_16_w16_avx2(
        cospi_p02_p62,
        cospi_p62_m02,
        x1.as_mut_ptr(),
        x1.as_mut_ptr().offset(1),
        _r,
        INV_COS_BIT,
    );
    btf_16_w16_avx2(
        cospi_p10_p54,
        cospi_p54_m10,
        x1.as_mut_ptr().offset(2),
        x1.as_mut_ptr().offset(3),
        _r,
        INV_COS_BIT,
    );
    btf_16_w16_avx2(
        cospi_p18_p46,
        cospi_p46_m18,
        x1.as_mut_ptr().offset(4),
        x1.as_mut_ptr().offset(5),
        _r,
        INV_COS_BIT,
    );
    btf_16_w16_avx2(
        cospi_p26_p38,
        cospi_p38_m26,
        x1.as_mut_ptr().offset(6),
        x1.as_mut_ptr().offset(7),
        _r,
        INV_COS_BIT,
    );
    btf_16_w16_avx2(
        cospi_p34_p30,
        cospi_p30_m34,
        x1.as_mut_ptr().offset(8),
        x1.as_mut_ptr().offset(9),
        _r,
        INV_COS_BIT,
    );
    btf_16_w16_avx2(
        cospi_p42_p22,
        cospi_p22_m42,
        x1.as_mut_ptr().offset(10),
        x1.as_mut_ptr().offset(11),
        _r,
        INV_COS_BIT,
    );
    btf_16_w16_avx2(
        cospi_p50_p14,
        cospi_p14_m50,
        x1.as_mut_ptr().offset(12),
        x1.as_mut_ptr().offset(13),
        _r,
        INV_COS_BIT,
    );
    btf_16_w16_avx2(
        cospi_p58_p06,
        cospi_p06_m58,
        x1.as_mut_ptr().offset(14),
        x1.as_mut_ptr().offset(15),
        _r,
        INV_COS_BIT,
    );
    iadst16_stage3_avx2(x1.as_mut_ptr());
    iadst16_stage4_avx2(x1.as_mut_ptr(), cospi, _r, INV_COS_BIT as i8);
    iadst16_stage5_avx2(x1.as_mut_ptr());
    iadst16_stage6_avx2(x1.as_mut_ptr(), cospi, _r, INV_COS_BIT as i8);
    iadst16_stage7_avx2(x1.as_mut_ptr());
    iadst16_stage8_avx2(x1.as_mut_ptr(), cospi, _r, INV_COS_BIT as i8);
    iadst16_stage9_avx2(output, x1.as_mut_ptr());
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn iadst16_low8_avx2(input: *const __m256i, output: *mut __m256i) {
    let mut cospi: *const i32 = cospi_arr(INV_COS_BIT);
    let _r: __m256i = _mm256_set1_epi32(1 << (INV_COS_BIT - 1));
    // stage 1
    let mut x1: [__m256i; 16] = core::mem::zeroed();
    x1[1] = *input;
    x1[3] = *input.offset(2);
    x1[5] = *input.offset(4);
    x1[7] = *input.offset(6);
    x1[8] = *input.offset(7);
    x1[10] = *input.offset(5);
    x1[12] = *input.offset(3);
    x1[14] = *input.offset(1);
    // stage 2
    btf_16_w16_0_avx2!(*cospi.offset(62), -*cospi.offset(2), x1[1], x1[0], x1[1]);
    btf_16_w16_0_avx2!(*cospi.offset(54), -*cospi.offset(10), x1[3], x1[2], x1[3]);
    btf_16_w16_0_avx2!(*cospi.offset(46), -*cospi.offset(18), x1[5], x1[4], x1[5]);
    btf_16_w16_0_avx2!(*cospi.offset(38), -*cospi.offset(26), x1[7], x1[6], x1[7]);
    btf_16_w16_0_avx2!(*cospi.offset(34), *cospi.offset(30), x1[8], x1[8], x1[9]);
    btf_16_w16_0_avx2!(*cospi.offset(42), *cospi.offset(22), x1[10], x1[10], x1[11]);
    btf_16_w16_0_avx2!(*cospi.offset(50), *cospi.offset(14), x1[12], x1[12], x1[13]);
    btf_16_w16_0_avx2!(*cospi.offset(58), *cospi.offset(6), x1[14], x1[14], x1[15]);
    iadst16_stage3_avx2(x1.as_mut_ptr());
    iadst16_stage4_avx2(x1.as_mut_ptr(), cospi, _r, INV_COS_BIT as i8);
    iadst16_stage5_avx2(x1.as_mut_ptr());
    iadst16_stage6_avx2(x1.as_mut_ptr(), cospi, _r, INV_COS_BIT as i8);
    iadst16_stage7_avx2(x1.as_mut_ptr());
    iadst16_stage8_avx2(x1.as_mut_ptr(), cospi, _r, INV_COS_BIT as i8);
    iadst16_stage9_avx2(output, x1.as_mut_ptr());
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn iadst16_low1_avx2(input: *const __m256i, output: *mut __m256i) {
    let mut cospi: *const i32 = cospi_arr(INV_COS_BIT);
    let _r: __m256i = _mm256_set1_epi32(1 << (INV_COS_BIT - 1));
    let cospi_p08_p56: __m256i =
        pair_set_w16_epi16(*cospi.offset(8) as i16, *cospi.offset(56) as i16);
    let cospi_p56_m08: __m256i =
        pair_set_w16_epi16(*cospi.offset(56) as i16, -*cospi.offset(8) as i16);
    let cospi_p16_p48: __m256i =
        pair_set_w16_epi16(*cospi.offset(16) as i16, *cospi.offset(48) as i16);
    let cospi_p48_m16: __m256i =
        pair_set_w16_epi16(*cospi.offset(48) as i16, -*cospi.offset(16) as i16);
    // stage 1
    let mut x1: [__m256i; 16] = core::mem::zeroed();
    x1[1] = *input;
    // stage 2
    btf_16_w16_0_avx2!(*cospi.offset(62), -*cospi.offset(2), x1[1], x1[0], x1[1]);
    // stage 3
    x1[8] = x1[0];
    x1[9] = x1[1];
    // stage 4
    btf_16_w16_avx2(
        cospi_p08_p56,
        cospi_p56_m08,
        x1.as_mut_ptr().offset(8),
        x1.as_mut_ptr().offset(9),
        _r,
        INV_COS_BIT,
    );
    // stage 5
    x1[4] = x1[0];
    x1[5] = x1[1];
    x1[12] = x1[8];
    x1[13] = x1[9];
    // stage 6
    btf_16_w16_avx2(
        cospi_p16_p48,
        cospi_p48_m16,
        x1.as_mut_ptr().offset(4),
        x1.as_mut_ptr().offset(5),
        _r,
        INV_COS_BIT,
    );
    btf_16_w16_avx2(
        cospi_p16_p48,
        cospi_p48_m16,
        x1.as_mut_ptr().offset(12),
        x1.as_mut_ptr().offset(13),
        _r,
        INV_COS_BIT,
    );
    // stage 7
    x1[2] = x1[0];
    x1[3] = x1[1];
    x1[6] = x1[4];
    x1[7] = x1[5];
    x1[10] = x1[8];
    x1[11] = x1[9];
    x1[14] = x1[12];
    x1[15] = x1[13];
    iadst16_stage8_avx2(x1.as_mut_ptr(), cospi, _r, INV_COS_BIT as i8);
    iadst16_stage9_avx2(output, x1.as_mut_ptr());
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn idct32_high16_stage3_avx2(x: *mut __m256i) {
    btf_16_adds_subs_avx2(x.offset(16), x.offset(17));
    btf_16_adds_subs_avx2(x.offset(19), x.offset(18));
    btf_16_adds_subs_avx2(x.offset(20), x.offset(21));
    btf_16_adds_subs_avx2(x.offset(23), x.offset(22));
    btf_16_adds_subs_avx2(x.offset(24), x.offset(25));
    btf_16_adds_subs_avx2(x.offset(27), x.offset(26));
    btf_16_adds_subs_avx2(x.offset(28), x.offset(29));
    btf_16_adds_subs_avx2(x.offset(31), x.offset(30));
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn idct32_high16_stage4_avx2(x: *mut __m256i, cospi: *const i32, _r: __m256i, cos_bit: i8) {
    let cospi_m08_p56: __m256i =
        pair_set_w16_epi16(-*cospi.offset(8) as i16, *cospi.offset(56) as i16);
    let cospi_p56_p08: __m256i =
        pair_set_w16_epi16(*cospi.offset(56) as i16, *cospi.offset(8) as i16);
    let cospi_m56_m08: __m256i =
        pair_set_w16_epi16(-*cospi.offset(56) as i16, -*cospi.offset(8) as i16);
    let cospi_m40_p24: __m256i =
        pair_set_w16_epi16(-*cospi.offset(40) as i16, *cospi.offset(24) as i16);
    let cospi_p24_p40: __m256i =
        pair_set_w16_epi16(*cospi.offset(24) as i16, *cospi.offset(40) as i16);
    let cospi_m24_m40: __m256i =
        pair_set_w16_epi16(-*cospi.offset(24) as i16, -*cospi.offset(40) as i16);
    btf_16_w16_avx2(
        cospi_m08_p56,
        cospi_p56_p08,
        x.offset(17),
        x.offset(30),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m56_m08,
        cospi_m08_p56,
        x.offset(18),
        x.offset(29),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m40_p24,
        cospi_p24_p40,
        x.offset(21),
        x.offset(26),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m24_m40,
        cospi_m40_p24,
        x.offset(22),
        x.offset(25),
        _r,
        cos_bit as i32,
    );
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn idct32_high24_stage5_avx2(x: *mut __m256i, cospi: *const i32, _r: __m256i, cos_bit: i8) {
    let cospi_m16_p48: __m256i =
        pair_set_w16_epi16(-*cospi.offset(16) as i16, *cospi.offset(48) as i16);
    let cospi_p48_p16: __m256i =
        pair_set_w16_epi16(*cospi.offset(48) as i16, *cospi.offset(16) as i16);
    let cospi_m48_m16: __m256i =
        pair_set_w16_epi16(-*cospi.offset(48) as i16, -*cospi.offset(16) as i16);
    btf_16_w16_avx2(
        cospi_m16_p48,
        cospi_p48_p16,
        x.offset(9),
        x.offset(14),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m48_m16,
        cospi_m16_p48,
        x.offset(10),
        x.offset(13),
        _r,
        cos_bit as i32,
    );
    btf_16_adds_subs_avx2(x.offset(16), x.offset(19));
    btf_16_adds_subs_avx2(x.offset(17), x.offset(18));
    btf_16_adds_subs_avx2(x.offset(23), x.offset(20));
    btf_16_adds_subs_avx2(x.offset(22), x.offset(21));
    btf_16_adds_subs_avx2(x.offset(24), x.offset(27));
    btf_16_adds_subs_avx2(x.offset(25), x.offset(26));
    btf_16_adds_subs_avx2(x.offset(31), x.offset(28));
    btf_16_adds_subs_avx2(x.offset(30), x.offset(29));
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn idct32_high28_stage6_avx2(x: *mut __m256i, cospi: *const i32, _r: __m256i, cos_bit: i8) {
    let cospi_m32_p32: __m256i =
        pair_set_w16_epi16(-*cospi.offset(32) as i16, *cospi.offset(32) as i16);
    let cospi_p32_p32: __m256i =
        pair_set_w16_epi16(*cospi.offset(32) as i16, *cospi.offset(32) as i16);
    let cospi_m16_p48: __m256i =
        pair_set_w16_epi16(-*cospi.offset(16) as i16, *cospi.offset(48) as i16);
    let cospi_p48_p16: __m256i =
        pair_set_w16_epi16(*cospi.offset(48) as i16, *cospi.offset(16) as i16);
    let cospi_m48_m16: __m256i =
        pair_set_w16_epi16(-*cospi.offset(48) as i16, -*cospi.offset(16) as i16);
    btf_16_w16_avx2(
        cospi_m32_p32,
        cospi_p32_p32,
        x.offset(5),
        x.offset(6),
        _r,
        cos_bit as i32,
    );
    btf_16_adds_subs_avx2(x.offset(8), x.offset(11));
    btf_16_adds_subs_avx2(x.offset(9), x.offset(10));
    btf_16_adds_subs_avx2(x.offset(15), x.offset(12));
    btf_16_adds_subs_avx2(x.offset(14), x.offset(13));
    btf_16_w16_avx2(
        cospi_m16_p48,
        cospi_p48_p16,
        x.offset(18),
        x.offset(29),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m16_p48,
        cospi_p48_p16,
        x.offset(19),
        x.offset(28),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m48_m16,
        cospi_m16_p48,
        x.offset(20),
        x.offset(27),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m48_m16,
        cospi_m16_p48,
        x.offset(21),
        x.offset(26),
        _r,
        cos_bit as i32,
    );
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn idct32_stage7_avx2(x: *mut __m256i, cospi: *const i32, _r: __m256i, cos_bit: i8) {
    let cospi_m32_p32: __m256i =
        pair_set_w16_epi16(-*cospi.offset(32) as i16, *cospi.offset(32) as i16);
    let cospi_p32_p32: __m256i =
        pair_set_w16_epi16(*cospi.offset(32) as i16, *cospi.offset(32) as i16);
    btf_16_adds_subs_avx2(x, x.offset(7));
    btf_16_adds_subs_avx2(x.offset(1), x.offset(6));
    btf_16_adds_subs_avx2(x.offset(2), x.offset(5));
    btf_16_adds_subs_avx2(x.offset(3), x.offset(4));
    btf_16_w16_avx2(
        cospi_m32_p32,
        cospi_p32_p32,
        x.offset(10),
        x.offset(13),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m32_p32,
        cospi_p32_p32,
        x.offset(11),
        x.offset(12),
        _r,
        cos_bit as i32,
    );
    btf_16_adds_subs_avx2(x.offset(16), x.offset(23));
    btf_16_adds_subs_avx2(x.offset(17), x.offset(22));
    btf_16_adds_subs_avx2(x.offset(18), x.offset(21));
    btf_16_adds_subs_avx2(x.offset(19), x.offset(20));
    btf_16_adds_subs_avx2(x.offset(31), x.offset(24));
    btf_16_adds_subs_avx2(x.offset(30), x.offset(25));
    btf_16_adds_subs_avx2(x.offset(29), x.offset(26));
    btf_16_adds_subs_avx2(x.offset(28), x.offset(27));
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn idct32_stage8_avx2(x: *mut __m256i, cospi: *const i32, _r: __m256i, cos_bit: i8) {
    let cospi_m32_p32: __m256i =
        pair_set_w16_epi16(-*cospi.offset(32) as i16, *cospi.offset(32) as i16);
    let cospi_p32_p32: __m256i =
        pair_set_w16_epi16(*cospi.offset(32) as i16, *cospi.offset(32) as i16);
    btf_16_adds_subs_avx2(x, x.offset(15));
    btf_16_adds_subs_avx2(x.offset(1), x.offset(14));
    btf_16_adds_subs_avx2(x.offset(2), x.offset(13));
    btf_16_adds_subs_avx2(x.offset(3), x.offset(12));
    btf_16_adds_subs_avx2(x.offset(4), x.offset(11));
    btf_16_adds_subs_avx2(x.offset(5), x.offset(10));
    btf_16_adds_subs_avx2(x.offset(6), x.offset(9));
    btf_16_adds_subs_avx2(x.offset(7), x.offset(8));
    btf_16_w16_avx2(
        cospi_m32_p32,
        cospi_p32_p32,
        x.offset(20),
        x.offset(27),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m32_p32,
        cospi_p32_p32,
        x.offset(21),
        x.offset(26),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m32_p32,
        cospi_p32_p32,
        x.offset(22),
        x.offset(25),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m32_p32,
        cospi_p32_p32,
        x.offset(23),
        x.offset(24),
        _r,
        cos_bit as i32,
    );
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn idct32_stage9_avx2(output: *mut __m256i, x: *mut __m256i) {
    btf_16_adds_subs_out_avx2(output, output.offset(31), *x, *x.offset(31));
    btf_16_adds_subs_out_avx2(
        output.offset(1),
        output.offset(30),
        *x.offset(1),
        *x.offset(30),
    );
    btf_16_adds_subs_out_avx2(
        output.offset(2),
        output.offset(29),
        *x.offset(2),
        *x.offset(29),
    );
    btf_16_adds_subs_out_avx2(
        output.offset(3),
        output.offset(28),
        *x.offset(3),
        *x.offset(28),
    );
    btf_16_adds_subs_out_avx2(
        output.offset(4),
        output.offset(27),
        *x.offset(4),
        *x.offset(27),
    );
    btf_16_adds_subs_out_avx2(
        output.offset(5),
        output.offset(26),
        *x.offset(5),
        *x.offset(26),
    );
    btf_16_adds_subs_out_avx2(
        output.offset(6),
        output.offset(25),
        *x.offset(6),
        *x.offset(25),
    );
    btf_16_adds_subs_out_avx2(
        output.offset(7),
        output.offset(24),
        *x.offset(7),
        *x.offset(24),
    );
    btf_16_adds_subs_out_avx2(
        output.offset(8),
        output.offset(23),
        *x.offset(8),
        *x.offset(23),
    );
    btf_16_adds_subs_out_avx2(
        output.offset(9),
        output.offset(22),
        *x.offset(9),
        *x.offset(22),
    );
    btf_16_adds_subs_out_avx2(
        output.offset(10),
        output.offset(21),
        *x.offset(10),
        *x.offset(21),
    );
    btf_16_adds_subs_out_avx2(
        output.offset(11),
        output.offset(20),
        *x.offset(11),
        *x.offset(20),
    );
    btf_16_adds_subs_out_avx2(
        output.offset(12),
        output.offset(19),
        *x.offset(12),
        *x.offset(19),
    );
    btf_16_adds_subs_out_avx2(
        output.offset(13),
        output.offset(18),
        *x.offset(13),
        *x.offset(18),
    );
    btf_16_adds_subs_out_avx2(
        output.offset(14),
        output.offset(17),
        *x.offset(14),
        *x.offset(17),
    );
    btf_16_adds_subs_out_avx2(
        output.offset(15),
        output.offset(16),
        *x.offset(15),
        *x.offset(16),
    );
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn idct32_low1_avx2(input: *const __m256i, output: *mut __m256i) {
    let mut cospi: *const i32 = cospi_arr(INV_COS_BIT);
    // stage 1
    let mut x: [__m256i; 2] = core::mem::zeroed();
    x[0] = *input;
    // stage 2
    // stage 3
    // stage 4
    // stage 5
    btf_16_w16_0_avx2!(*cospi.offset(32), *cospi.offset(32), x[0], x[0], x[1]);
    // stage 6
    // stage 7
    // stage 8
    // stage 9
    *output = x[0];
    *output.offset(31) = x[0];
    *output.offset(1) = x[1];
    *output.offset(30) = x[1];
    *output.offset(2) = x[1];
    *output.offset(29) = x[1];
    *output.offset(3) = x[0];
    *output.offset(28) = x[0];
    *output.offset(4) = x[0];
    *output.offset(27) = x[0];
    *output.offset(5) = x[1];
    *output.offset(26) = x[1];
    *output.offset(6) = x[1];
    *output.offset(25) = x[1];
    *output.offset(7) = x[0];
    *output.offset(24) = x[0];
    *output.offset(8) = x[0];
    *output.offset(23) = x[0];
    *output.offset(9) = x[1];
    *output.offset(22) = x[1];
    *output.offset(10) = x[1];
    *output.offset(21) = x[1];
    *output.offset(11) = x[0];
    *output.offset(20) = x[0];
    *output.offset(12) = x[0];
    *output.offset(19) = x[0];
    *output.offset(13) = x[1];
    *output.offset(18) = x[1];
    *output.offset(14) = x[1];
    *output.offset(17) = x[1];
    *output.offset(15) = x[0];
    *output.offset(16) = x[0];
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn idct32_low8_avx2(input: *const __m256i, output: *mut __m256i) {
    let mut cospi: *const i32 = cospi_arr(INV_COS_BIT);
    let _r: __m256i = _mm256_set1_epi32(1 << (INV_COS_BIT - 1));
    // stage 1
    let mut x: [__m256i; 32] = core::mem::zeroed();
    x[0] = *input;
    x[4] = *input.offset(4);
    x[8] = *input.offset(2);
    x[12] = *input.offset(6);
    x[16] = *input.offset(1);
    x[20] = *input.offset(5);
    x[24] = *input.offset(3);
    x[28] = *input.offset(7);
    // stage 2
    btf_16_w16_0_avx2!(*cospi.offset(62), *cospi.offset(2), x[16], x[16], x[31]);
    btf_16_w16_0_avx2!(-*cospi.offset(50), *cospi.offset(14), x[28], x[19], x[28]);
    btf_16_w16_0_avx2!(*cospi.offset(54), *cospi.offset(10), x[20], x[20], x[27]);
    btf_16_w16_0_avx2!(-*cospi.offset(58), *cospi.offset(6), x[24], x[23], x[24]);
    // stage 3
    btf_16_w16_0_avx2!(*cospi.offset(60), *cospi.offset(4), x[8], x[8], x[15]);
    btf_16_w16_0_avx2!(-*cospi.offset(52), *cospi.offset(12), x[12], x[11], x[12]);
    x[17] = x[16];
    x[18] = x[19];
    x[21] = x[20];
    x[22] = x[23];
    x[25] = x[24];
    x[26] = x[27];
    x[29] = x[28];
    x[30] = x[31];
    // stage 4
    btf_16_w16_0_avx2!(*cospi.offset(56), *cospi.offset(8), x[4], x[4], x[7]);
    x[9] = x[8];
    x[10] = x[11];
    x[13] = x[12];
    x[14] = x[15];
    idct32_high16_stage4_avx2(x.as_mut_ptr(), cospi, _r, INV_COS_BIT as i8);
    // stage 5
    btf_16_w16_0_avx2!(*cospi.offset(32), *cospi.offset(32), x[0], x[0], x[1]);
    x[5] = x[4];
    x[6] = x[7];
    idct32_high24_stage5_avx2(x.as_mut_ptr(), cospi, _r, INV_COS_BIT as i8);
    // stage 6
    x[3] = x[0];
    x[2] = x[1];
    idct32_high28_stage6_avx2(x.as_mut_ptr(), cospi, _r, INV_COS_BIT as i8);
    idct32_stage7_avx2(x.as_mut_ptr(), cospi, _r, INV_COS_BIT as i8);
    idct32_stage8_avx2(x.as_mut_ptr(), cospi, _r, INV_COS_BIT as i8);
    idct32_stage9_avx2(output, x.as_mut_ptr());
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn idct32_low16_avx2(input: *const __m256i, output: *mut __m256i) {
    let mut cospi: *const i32 = cospi_arr(INV_COS_BIT);
    let _r: __m256i = _mm256_set1_epi32(1 << (INV_COS_BIT - 1));
    // stage 1
    let mut x: [__m256i; 32] = core::mem::zeroed();
    x[0] = *input;
    x[2] = *input.offset(8);
    x[4] = *input.offset(4);
    x[6] = *input.offset(12);
    x[8] = *input.offset(2);
    x[10] = *input.offset(10);
    x[12] = *input.offset(6);
    x[14] = *input.offset(14);
    x[16] = *input.offset(1);
    x[18] = *input.offset(9);
    x[20] = *input.offset(5);
    x[22] = *input.offset(13);
    x[24] = *input.offset(3);
    x[26] = *input.offset(11);
    x[28] = *input.offset(7);
    x[30] = *input.offset(15);
    // stage 2
    btf_16_w16_0_avx2!(*cospi.offset(62), *cospi.offset(2), x[16], x[16], x[31]);
    btf_16_w16_0_avx2!(-*cospi.offset(34), *cospi.offset(30), x[30], x[17], x[30]);
    btf_16_w16_0_avx2!(*cospi.offset(46), *cospi.offset(18), x[18], x[18], x[29]);
    btf_16_w16_0_avx2!(-*cospi.offset(50), *cospi.offset(14), x[28], x[19], x[28]);
    btf_16_w16_0_avx2!(*cospi.offset(54), *cospi.offset(10), x[20], x[20], x[27]);
    btf_16_w16_0_avx2!(-*cospi.offset(42), *cospi.offset(22), x[26], x[21], x[26]);
    btf_16_w16_0_avx2!(*cospi.offset(38), *cospi.offset(26), x[22], x[22], x[25]);
    btf_16_w16_0_avx2!(-*cospi.offset(58), *cospi.offset(6), x[24], x[23], x[24]);
    // stage 3
    btf_16_w16_0_avx2!(*cospi.offset(60), *cospi.offset(4), x[8], x[8], x[15]);
    btf_16_w16_0_avx2!(-*cospi.offset(36), *cospi.offset(28), x[14], x[9], x[14]);
    btf_16_w16_0_avx2!(*cospi.offset(44), *cospi.offset(20), x[10], x[10], x[13]);
    btf_16_w16_0_avx2!(-*cospi.offset(52), *cospi.offset(12), x[12], x[11], x[12]);
    idct32_high16_stage3_avx2(x.as_mut_ptr());
    // stage 4
    btf_16_w16_0_avx2!(*cospi.offset(56), *cospi.offset(8), x[4], x[4], x[7]);
    btf_16_w16_0_avx2!(-*cospi.offset(40), *cospi.offset(24), x[6], x[5], x[6]);
    btf_16_adds_subs_avx2(x.as_mut_ptr().offset(8), x.as_mut_ptr().offset(9));
    btf_16_adds_subs_avx2(x.as_mut_ptr().offset(11), x.as_mut_ptr().offset(10));
    btf_16_adds_subs_avx2(x.as_mut_ptr().offset(12), x.as_mut_ptr().offset(13));
    btf_16_adds_subs_avx2(x.as_mut_ptr().offset(15), x.as_mut_ptr().offset(14));
    idct32_high16_stage4_avx2(x.as_mut_ptr(), cospi, _r, INV_COS_BIT as i8);
    // stage 5
    btf_16_w16_0_avx2!(*cospi.offset(32), *cospi.offset(32), x[0], x[0], x[1]);
    btf_16_w16_0_avx2!(*cospi.offset(48), *cospi.offset(16), x[2], x[2], x[3]);
    btf_16_adds_subs_avx2(x.as_mut_ptr().offset(4), x.as_mut_ptr().offset(5));
    btf_16_adds_subs_avx2(x.as_mut_ptr().offset(7), x.as_mut_ptr().offset(6));
    idct32_high24_stage5_avx2(x.as_mut_ptr(), cospi, _r, INV_COS_BIT as i8);
    btf_16_adds_subs_avx2(x.as_mut_ptr(), x.as_mut_ptr().offset(3));
    btf_16_adds_subs_avx2(x.as_mut_ptr().offset(1), x.as_mut_ptr().offset(2));
    idct32_high28_stage6_avx2(x.as_mut_ptr(), cospi, _r, INV_COS_BIT as i8);
    idct32_stage7_avx2(x.as_mut_ptr(), cospi, _r, INV_COS_BIT as i8);
    idct32_stage8_avx2(x.as_mut_ptr(), cospi, _r, INV_COS_BIT as i8);
    idct32_stage9_avx2(output, x.as_mut_ptr());
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn idct32_avx2(input: *const __m256i, output: *mut __m256i) {
    let mut cospi: *const i32 = cospi_arr(INV_COS_BIT);
    let _r: __m256i = _mm256_set1_epi32(1 << (INV_COS_BIT - 1));
    let mut cospi_p62_m02: __m256i =
        pair_set_w16_epi16(*cospi.offset(62) as i16, -*cospi.offset(2) as i16);
    let mut cospi_p02_p62: __m256i =
        pair_set_w16_epi16(*cospi.offset(2) as i16, *cospi.offset(62) as i16);
    let mut cospi_p30_m34: __m256i =
        pair_set_w16_epi16(*cospi.offset(30) as i16, -*cospi.offset(34) as i16);
    let mut cospi_p34_p30: __m256i =
        pair_set_w16_epi16(*cospi.offset(34) as i16, *cospi.offset(30) as i16);
    let mut cospi_p46_m18: __m256i =
        pair_set_w16_epi16(*cospi.offset(46) as i16, -*cospi.offset(18) as i16);
    let mut cospi_p18_p46: __m256i =
        pair_set_w16_epi16(*cospi.offset(18) as i16, *cospi.offset(46) as i16);
    let mut cospi_p14_m50: __m256i =
        pair_set_w16_epi16(*cospi.offset(14) as i16, -*cospi.offset(50) as i16);
    let mut cospi_p50_p14: __m256i =
        pair_set_w16_epi16(*cospi.offset(50) as i16, *cospi.offset(14) as i16);
    let mut cospi_p54_m10: __m256i =
        pair_set_w16_epi16(*cospi.offset(54) as i16, -*cospi.offset(10) as i16);
    let mut cospi_p10_p54: __m256i =
        pair_set_w16_epi16(*cospi.offset(10) as i16, *cospi.offset(54) as i16);
    let mut cospi_p22_m42: __m256i =
        pair_set_w16_epi16(*cospi.offset(22) as i16, -*cospi.offset(42) as i16);
    let mut cospi_p42_p22: __m256i =
        pair_set_w16_epi16(*cospi.offset(42) as i16, *cospi.offset(22) as i16);
    let mut cospi_p38_m26: __m256i =
        pair_set_w16_epi16(*cospi.offset(38) as i16, -*cospi.offset(26) as i16);
    let mut cospi_p26_p38: __m256i =
        pair_set_w16_epi16(*cospi.offset(26) as i16, *cospi.offset(38) as i16);
    let mut cospi_p06_m58: __m256i =
        pair_set_w16_epi16(*cospi.offset(6) as i16, -*cospi.offset(58) as i16);
    let mut cospi_p58_p06: __m256i =
        pair_set_w16_epi16(*cospi.offset(58) as i16, *cospi.offset(6) as i16);
    let mut cospi_p60_m04: __m256i =
        pair_set_w16_epi16(*cospi.offset(60) as i16, -*cospi.offset(4) as i16);
    let mut cospi_p04_p60: __m256i =
        pair_set_w16_epi16(*cospi.offset(4) as i16, *cospi.offset(60) as i16);
    let mut cospi_p28_m36: __m256i =
        pair_set_w16_epi16(*cospi.offset(28) as i16, -*cospi.offset(36) as i16);
    let mut cospi_p36_p28: __m256i =
        pair_set_w16_epi16(*cospi.offset(36) as i16, *cospi.offset(28) as i16);
    let mut cospi_p44_m20: __m256i =
        pair_set_w16_epi16(*cospi.offset(44) as i16, -*cospi.offset(20) as i16);
    let mut cospi_p20_p44: __m256i =
        pair_set_w16_epi16(*cospi.offset(20) as i16, *cospi.offset(44) as i16);
    let mut cospi_p12_m52: __m256i =
        pair_set_w16_epi16(*cospi.offset(12) as i16, -*cospi.offset(52) as i16);
    let mut cospi_p52_p12: __m256i =
        pair_set_w16_epi16(*cospi.offset(52) as i16, *cospi.offset(12) as i16);
    let mut cospi_p56_m08: __m256i =
        pair_set_w16_epi16(*cospi.offset(56) as i16, -*cospi.offset(8) as i16);
    let mut cospi_p08_p56: __m256i =
        pair_set_w16_epi16(*cospi.offset(8) as i16, *cospi.offset(56) as i16);
    let mut cospi_p24_m40: __m256i =
        pair_set_w16_epi16(*cospi.offset(24) as i16, -*cospi.offset(40) as i16);
    let mut cospi_p40_p24: __m256i =
        pair_set_w16_epi16(*cospi.offset(40) as i16, *cospi.offset(24) as i16);
    let mut cospi_p32_p32: __m256i =
        pair_set_w16_epi16(*cospi.offset(32) as i16, *cospi.offset(32) as i16);
    let mut cospi_p32_m32: __m256i =
        pair_set_w16_epi16(*cospi.offset(32) as i16, -*cospi.offset(32) as i16);
    let mut cospi_p48_m16: __m256i =
        pair_set_w16_epi16(*cospi.offset(48) as i16, -*cospi.offset(16) as i16);
    let mut cospi_p16_p48: __m256i =
        pair_set_w16_epi16(*cospi.offset(16) as i16, *cospi.offset(48) as i16);
    // stage 1
    let mut x1: [__m256i; 32] = core::mem::zeroed();
    x1[0] = *input;
    x1[1] = *input.offset(16);
    x1[2] = *input.offset(8);
    x1[3] = *input.offset(24);
    x1[4] = *input.offset(4);
    x1[5] = *input.offset(20);
    x1[6] = *input.offset(12);
    x1[7] = *input.offset(28);
    x1[8] = *input.offset(2);
    x1[9] = *input.offset(18);
    x1[10] = *input.offset(10);
    x1[11] = *input.offset(26);
    x1[12] = *input.offset(6);
    x1[13] = *input.offset(22);
    x1[14] = *input.offset(14);
    x1[15] = *input.offset(30);
    x1[16] = *input.offset(1);
    x1[17] = *input.offset(17);
    x1[18] = *input.offset(9);
    x1[19] = *input.offset(25);
    x1[20] = *input.offset(5);
    x1[21] = *input.offset(21);
    x1[22] = *input.offset(13);
    x1[23] = *input.offset(29);
    x1[24] = *input.offset(3);
    x1[25] = *input.offset(19);
    x1[26] = *input.offset(11);
    x1[27] = *input.offset(27);
    x1[28] = *input.offset(7);
    x1[29] = *input.offset(23);
    x1[30] = *input.offset(15);
    x1[31] = *input.offset(31);
    // stage 2
    btf_16_w16_avx2(
        cospi_p62_m02,
        cospi_p02_p62,
        x1.as_mut_ptr().offset(16),
        x1.as_mut_ptr().offset(31),
        _r,
        INV_COS_BIT,
    );
    btf_16_w16_avx2(
        cospi_p30_m34,
        cospi_p34_p30,
        x1.as_mut_ptr().offset(17),
        x1.as_mut_ptr().offset(30),
        _r,
        INV_COS_BIT,
    );
    btf_16_w16_avx2(
        cospi_p46_m18,
        cospi_p18_p46,
        x1.as_mut_ptr().offset(18),
        x1.as_mut_ptr().offset(29),
        _r,
        INV_COS_BIT,
    );
    btf_16_w16_avx2(
        cospi_p14_m50,
        cospi_p50_p14,
        x1.as_mut_ptr().offset(19),
        x1.as_mut_ptr().offset(28),
        _r,
        INV_COS_BIT,
    );
    btf_16_w16_avx2(
        cospi_p54_m10,
        cospi_p10_p54,
        x1.as_mut_ptr().offset(20),
        x1.as_mut_ptr().offset(27),
        _r,
        INV_COS_BIT,
    );
    btf_16_w16_avx2(
        cospi_p22_m42,
        cospi_p42_p22,
        x1.as_mut_ptr().offset(21),
        x1.as_mut_ptr().offset(26),
        _r,
        INV_COS_BIT,
    );
    btf_16_w16_avx2(
        cospi_p38_m26,
        cospi_p26_p38,
        x1.as_mut_ptr().offset(22),
        x1.as_mut_ptr().offset(25),
        _r,
        INV_COS_BIT,
    );
    btf_16_w16_avx2(
        cospi_p06_m58,
        cospi_p58_p06,
        x1.as_mut_ptr().offset(23),
        x1.as_mut_ptr().offset(24),
        _r,
        INV_COS_BIT,
    );
    // stage 3
    btf_16_w16_avx2(
        cospi_p60_m04,
        cospi_p04_p60,
        x1.as_mut_ptr().offset(8),
        x1.as_mut_ptr().offset(15),
        _r,
        INV_COS_BIT,
    );
    btf_16_w16_avx2(
        cospi_p28_m36,
        cospi_p36_p28,
        x1.as_mut_ptr().offset(9),
        x1.as_mut_ptr().offset(14),
        _r,
        INV_COS_BIT,
    );
    btf_16_w16_avx2(
        cospi_p44_m20,
        cospi_p20_p44,
        x1.as_mut_ptr().offset(10),
        x1.as_mut_ptr().offset(13),
        _r,
        INV_COS_BIT,
    );
    btf_16_w16_avx2(
        cospi_p12_m52,
        cospi_p52_p12,
        x1.as_mut_ptr().offset(11),
        x1.as_mut_ptr().offset(12),
        _r,
        INV_COS_BIT,
    );
    idct32_high16_stage3_avx2(x1.as_mut_ptr());
    // stage 4
    btf_16_w16_avx2(
        cospi_p56_m08,
        cospi_p08_p56,
        x1.as_mut_ptr().offset(4),
        x1.as_mut_ptr().offset(7),
        _r,
        INV_COS_BIT,
    );
    btf_16_w16_avx2(
        cospi_p24_m40,
        cospi_p40_p24,
        x1.as_mut_ptr().offset(5),
        x1.as_mut_ptr().offset(6),
        _r,
        INV_COS_BIT,
    );
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(8), x1.as_mut_ptr().offset(9));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(11), x1.as_mut_ptr().offset(10));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(12), x1.as_mut_ptr().offset(13));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(15), x1.as_mut_ptr().offset(14));
    idct32_high16_stage4_avx2(x1.as_mut_ptr(), cospi, _r, INV_COS_BIT as i8);
    // stage 5
    btf_16_w16_avx2(
        cospi_p32_p32,
        cospi_p32_m32,
        x1.as_mut_ptr(),
        x1.as_mut_ptr().offset(1),
        _r,
        INV_COS_BIT,
    );
    btf_16_w16_avx2(
        cospi_p48_m16,
        cospi_p16_p48,
        x1.as_mut_ptr().offset(2),
        x1.as_mut_ptr().offset(3),
        _r,
        INV_COS_BIT,
    );
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(4), x1.as_mut_ptr().offset(5));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(7), x1.as_mut_ptr().offset(6));
    idct32_high24_stage5_avx2(x1.as_mut_ptr(), cospi, _r, INV_COS_BIT as i8);
    // stage 6
    btf_16_adds_subs_avx2(x1.as_mut_ptr(), x1.as_mut_ptr().offset(3));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(1), x1.as_mut_ptr().offset(2));
    idct32_high28_stage6_avx2(x1.as_mut_ptr(), cospi, _r, INV_COS_BIT as i8);
    idct32_stage7_avx2(x1.as_mut_ptr(), cospi, _r, INV_COS_BIT as i8);
    idct32_stage8_avx2(x1.as_mut_ptr(), cospi, _r, INV_COS_BIT as i8);
    idct32_stage9_avx2(output, x1.as_mut_ptr());
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn idct64_stage4_high32_avx2(x: *mut __m256i, cospi: *const i32, _r: __m256i, cos_bit: i8) {
    let _ = cos_bit;
    let cospi_m04_p60: __m256i =
        pair_set_w16_epi16(-*cospi.offset(4) as i16, *cospi.offset(60) as i16);
    let cospi_p60_p04: __m256i =
        pair_set_w16_epi16(*cospi.offset(60) as i16, *cospi.offset(4) as i16);
    let cospi_m60_m04: __m256i =
        pair_set_w16_epi16(-*cospi.offset(60) as i16, -*cospi.offset(4) as i16);
    let cospi_m36_p28: __m256i =
        pair_set_w16_epi16(-*cospi.offset(36) as i16, *cospi.offset(28) as i16);
    let cospi_p28_p36: __m256i =
        pair_set_w16_epi16(*cospi.offset(28) as i16, *cospi.offset(36) as i16);
    let cospi_m28_m36: __m256i =
        pair_set_w16_epi16(-*cospi.offset(28) as i16, -*cospi.offset(36) as i16);
    let cospi_m20_p44: __m256i =
        pair_set_w16_epi16(-*cospi.offset(20) as i16, *cospi.offset(44) as i16);
    let cospi_p44_p20: __m256i =
        pair_set_w16_epi16(*cospi.offset(44) as i16, *cospi.offset(20) as i16);
    let cospi_m44_m20: __m256i =
        pair_set_w16_epi16(-*cospi.offset(44) as i16, -*cospi.offset(20) as i16);
    let cospi_m52_p12: __m256i =
        pair_set_w16_epi16(-*cospi.offset(52) as i16, *cospi.offset(12) as i16);
    let cospi_p12_p52: __m256i =
        pair_set_w16_epi16(*cospi.offset(12) as i16, *cospi.offset(52) as i16);
    let cospi_m12_m52: __m256i =
        pair_set_w16_epi16(-*cospi.offset(12) as i16, -*cospi.offset(52) as i16);
    btf_16_w16_avx2(
        cospi_m04_p60,
        cospi_p60_p04,
        x.offset(33),
        x.offset(62),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m60_m04,
        cospi_m04_p60,
        x.offset(34),
        x.offset(61),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m36_p28,
        cospi_p28_p36,
        x.offset(37),
        x.offset(58),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m28_m36,
        cospi_m36_p28,
        x.offset(38),
        x.offset(57),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m20_p44,
        cospi_p44_p20,
        x.offset(41),
        x.offset(54),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m44_m20,
        cospi_m20_p44,
        x.offset(42),
        x.offset(53),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m52_p12,
        cospi_p12_p52,
        x.offset(45),
        x.offset(50),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m12_m52,
        cospi_m52_p12,
        x.offset(46),
        x.offset(49),
        _r,
        cos_bit as i32,
    );
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn idct64_stage5_high48_avx2(x: *mut __m256i, cospi: *const i32, _r: __m256i, cos_bit: i8) {
    let _ = cos_bit;
    let cospi_m08_p56: __m256i =
        pair_set_w16_epi16(-*cospi.offset(8) as i16, *cospi.offset(56) as i16);
    let cospi_p56_p08: __m256i =
        pair_set_w16_epi16(*cospi.offset(56) as i16, *cospi.offset(8) as i16);
    let cospi_m56_m08: __m256i =
        pair_set_w16_epi16(-*cospi.offset(56) as i16, -*cospi.offset(8) as i16);
    let cospi_m40_p24: __m256i =
        pair_set_w16_epi16(-*cospi.offset(40) as i16, *cospi.offset(24) as i16);
    let cospi_p24_p40: __m256i =
        pair_set_w16_epi16(*cospi.offset(24) as i16, *cospi.offset(40) as i16);
    let cospi_m24_m40: __m256i =
        pair_set_w16_epi16(-*cospi.offset(24) as i16, -*cospi.offset(40) as i16);
    btf_16_w16_avx2(
        cospi_m08_p56,
        cospi_p56_p08,
        x.offset(17),
        x.offset(30),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m56_m08,
        cospi_m08_p56,
        x.offset(18),
        x.offset(29),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m40_p24,
        cospi_p24_p40,
        x.offset(21),
        x.offset(26),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m24_m40,
        cospi_m40_p24,
        x.offset(22),
        x.offset(25),
        _r,
        cos_bit as i32,
    );
    btf_16_adds_subs_avx2(x.offset(32), x.offset(35));
    btf_16_adds_subs_avx2(x.offset(33), x.offset(34));
    btf_16_adds_subs_avx2(x.offset(39), x.offset(36));
    btf_16_adds_subs_avx2(x.offset(38), x.offset(37));
    btf_16_adds_subs_avx2(x.offset(40), x.offset(43));
    btf_16_adds_subs_avx2(x.offset(41), x.offset(42));
    btf_16_adds_subs_avx2(x.offset(47), x.offset(44));
    btf_16_adds_subs_avx2(x.offset(46), x.offset(45));
    btf_16_adds_subs_avx2(x.offset(48), x.offset(51));
    btf_16_adds_subs_avx2(x.offset(49), x.offset(50));
    btf_16_adds_subs_avx2(x.offset(55), x.offset(52));
    btf_16_adds_subs_avx2(x.offset(54), x.offset(53));
    btf_16_adds_subs_avx2(x.offset(56), x.offset(59));
    btf_16_adds_subs_avx2(x.offset(57), x.offset(58));
    btf_16_adds_subs_avx2(x.offset(63), x.offset(60));
    btf_16_adds_subs_avx2(x.offset(62), x.offset(61));
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn idct64_stage6_high32_avx2(x: *mut __m256i, cospi: *const i32, _r: __m256i, cos_bit: i8) {
    let _ = cos_bit;
    let cospi_m08_p56: __m256i =
        pair_set_w16_epi16(-*cospi.offset(8) as i16, *cospi.offset(56) as i16);
    let cospi_p56_p08: __m256i =
        pair_set_w16_epi16(*cospi.offset(56) as i16, *cospi.offset(8) as i16);
    let cospi_m56_m08: __m256i =
        pair_set_w16_epi16(-*cospi.offset(56) as i16, -*cospi.offset(8) as i16);
    let cospi_m40_p24: __m256i =
        pair_set_w16_epi16(-*cospi.offset(40) as i16, *cospi.offset(24) as i16);
    let cospi_p24_p40: __m256i =
        pair_set_w16_epi16(*cospi.offset(24) as i16, *cospi.offset(40) as i16);
    let cospi_m24_m40: __m256i =
        pair_set_w16_epi16(-*cospi.offset(24) as i16, -*cospi.offset(40) as i16);
    btf_16_w16_avx2(
        cospi_m08_p56,
        cospi_p56_p08,
        x.offset(34),
        x.offset(61),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m08_p56,
        cospi_p56_p08,
        x.offset(35),
        x.offset(60),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m56_m08,
        cospi_m08_p56,
        x.offset(36),
        x.offset(59),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m56_m08,
        cospi_m08_p56,
        x.offset(37),
        x.offset(58),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m40_p24,
        cospi_p24_p40,
        x.offset(42),
        x.offset(53),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m40_p24,
        cospi_p24_p40,
        x.offset(43),
        x.offset(52),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m24_m40,
        cospi_m40_p24,
        x.offset(44),
        x.offset(51),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m24_m40,
        cospi_m40_p24,
        x.offset(45),
        x.offset(50),
        _r,
        cos_bit as i32,
    );
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn idct64_stage6_high48_avx2(x: *mut __m256i, cospi: *const i32, _r: __m256i, cos_bit: i8) {
    btf_16_adds_subs_avx2(x.offset(16), x.offset(19));
    btf_16_adds_subs_avx2(x.offset(17), x.offset(18));
    btf_16_adds_subs_avx2(x.offset(23), x.offset(20));
    btf_16_adds_subs_avx2(x.offset(22), x.offset(21));
    btf_16_adds_subs_avx2(x.offset(24), x.offset(27));
    btf_16_adds_subs_avx2(x.offset(25), x.offset(26));
    btf_16_adds_subs_avx2(x.offset(31), x.offset(28));
    btf_16_adds_subs_avx2(x.offset(30), x.offset(29));
    idct64_stage6_high32_avx2(x, cospi, _r, cos_bit);
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn idct64_stage7_high48_avx2(x: *mut __m256i, cospi: *const i32, _r: __m256i, cos_bit: i8) {
    let _ = cos_bit;
    let cospi_m16_p48: __m256i =
        pair_set_w16_epi16(-*cospi.offset(16) as i16, *cospi.offset(48) as i16);
    let cospi_p48_p16: __m256i =
        pair_set_w16_epi16(*cospi.offset(48) as i16, *cospi.offset(16) as i16);
    let cospi_m48_m16: __m256i =
        pair_set_w16_epi16(-*cospi.offset(48) as i16, -*cospi.offset(16) as i16);
    btf_16_w16_avx2(
        cospi_m16_p48,
        cospi_p48_p16,
        x.offset(18),
        x.offset(29),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m16_p48,
        cospi_p48_p16,
        x.offset(19),
        x.offset(28),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m48_m16,
        cospi_m16_p48,
        x.offset(20),
        x.offset(27),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m48_m16,
        cospi_m16_p48,
        x.offset(21),
        x.offset(26),
        _r,
        cos_bit as i32,
    );
    btf_16_adds_subs_avx2(x.offset(32), x.offset(39));
    btf_16_adds_subs_avx2(x.offset(33), x.offset(38));
    btf_16_adds_subs_avx2(x.offset(34), x.offset(37));
    btf_16_adds_subs_avx2(x.offset(35), x.offset(36));
    btf_16_adds_subs_avx2(x.offset(47), x.offset(40));
    btf_16_adds_subs_avx2(x.offset(46), x.offset(41));
    btf_16_adds_subs_avx2(x.offset(45), x.offset(42));
    btf_16_adds_subs_avx2(x.offset(44), x.offset(43));
    btf_16_adds_subs_avx2(x.offset(48), x.offset(55));
    btf_16_adds_subs_avx2(x.offset(49), x.offset(54));
    btf_16_adds_subs_avx2(x.offset(50), x.offset(53));
    btf_16_adds_subs_avx2(x.offset(51), x.offset(52));
    btf_16_adds_subs_avx2(x.offset(63), x.offset(56));
    btf_16_adds_subs_avx2(x.offset(62), x.offset(57));
    btf_16_adds_subs_avx2(x.offset(61), x.offset(58));
    btf_16_adds_subs_avx2(x.offset(60), x.offset(59));
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn idct64_stage8_high48_avx2(x: *mut __m256i, cospi: *const i32, _r: __m256i, cos_bit: i8) {
    let _ = cos_bit;
    let cospi_m16_p48: __m256i =
        pair_set_w16_epi16(-*cospi.offset(16) as i16, *cospi.offset(48) as i16);
    let cospi_p48_p16: __m256i =
        pair_set_w16_epi16(*cospi.offset(48) as i16, *cospi.offset(16) as i16);
    let cospi_m48_m16: __m256i =
        pair_set_w16_epi16(-*cospi.offset(48) as i16, -*cospi.offset(16) as i16);
    btf_16_adds_subs_avx2(x.offset(16), x.offset(23));
    btf_16_adds_subs_avx2(x.offset(17), x.offset(22));
    btf_16_adds_subs_avx2(x.offset(18), x.offset(21));
    btf_16_adds_subs_avx2(x.offset(19), x.offset(20));
    btf_16_adds_subs_avx2(x.offset(31), x.offset(24));
    btf_16_adds_subs_avx2(x.offset(30), x.offset(25));
    btf_16_adds_subs_avx2(x.offset(29), x.offset(26));
    btf_16_adds_subs_avx2(x.offset(28), x.offset(27));
    btf_16_w16_avx2(
        cospi_m16_p48,
        cospi_p48_p16,
        x.offset(36),
        x.offset(59),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m16_p48,
        cospi_p48_p16,
        x.offset(37),
        x.offset(58),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m16_p48,
        cospi_p48_p16,
        x.offset(38),
        x.offset(57),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m16_p48,
        cospi_p48_p16,
        x.offset(39),
        x.offset(56),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m48_m16,
        cospi_m16_p48,
        x.offset(40),
        x.offset(55),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m48_m16,
        cospi_m16_p48,
        x.offset(41),
        x.offset(54),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m48_m16,
        cospi_m16_p48,
        x.offset(42),
        x.offset(53),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m48_m16,
        cospi_m16_p48,
        x.offset(43),
        x.offset(52),
        _r,
        cos_bit as i32,
    );
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn idct64_stage9_avx2(x: *mut __m256i, cospi: *const i32, _r: __m256i, cos_bit: i8) {
    let _ = cos_bit;
    let cospi_m32_p32: __m256i =
        pair_set_w16_epi16(-*cospi.offset(32) as i16, *cospi.offset(32) as i16);
    let cospi_p32_p32: __m256i =
        pair_set_w16_epi16(*cospi.offset(32) as i16, *cospi.offset(32) as i16);
    btf_16_adds_subs_avx2(x, x.offset(15));
    btf_16_adds_subs_avx2(x.offset(1), x.offset(14));
    btf_16_adds_subs_avx2(x.offset(2), x.offset(13));
    btf_16_adds_subs_avx2(x.offset(3), x.offset(12));
    btf_16_adds_subs_avx2(x.offset(4), x.offset(11));
    btf_16_adds_subs_avx2(x.offset(5), x.offset(10));
    btf_16_adds_subs_avx2(x.offset(6), x.offset(9));
    btf_16_adds_subs_avx2(x.offset(7), x.offset(8));
    btf_16_w16_avx2(
        cospi_m32_p32,
        cospi_p32_p32,
        x.offset(20),
        x.offset(27),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m32_p32,
        cospi_p32_p32,
        x.offset(21),
        x.offset(26),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m32_p32,
        cospi_p32_p32,
        x.offset(22),
        x.offset(25),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m32_p32,
        cospi_p32_p32,
        x.offset(23),
        x.offset(24),
        _r,
        cos_bit as i32,
    );
    btf_16_adds_subs_avx2(x.offset(32), x.offset(47));
    btf_16_adds_subs_avx2(x.offset(33), x.offset(46));
    btf_16_adds_subs_avx2(x.offset(34), x.offset(45));
    btf_16_adds_subs_avx2(x.offset(35), x.offset(44));
    btf_16_adds_subs_avx2(x.offset(36), x.offset(43));
    btf_16_adds_subs_avx2(x.offset(37), x.offset(42));
    btf_16_adds_subs_avx2(x.offset(38), x.offset(41));
    btf_16_adds_subs_avx2(x.offset(39), x.offset(40));
    btf_16_adds_subs_avx2(x.offset(63), x.offset(48));
    btf_16_adds_subs_avx2(x.offset(62), x.offset(49));
    btf_16_adds_subs_avx2(x.offset(61), x.offset(50));
    btf_16_adds_subs_avx2(x.offset(60), x.offset(51));
    btf_16_adds_subs_avx2(x.offset(59), x.offset(52));
    btf_16_adds_subs_avx2(x.offset(58), x.offset(53));
    btf_16_adds_subs_avx2(x.offset(57), x.offset(54));
    btf_16_adds_subs_avx2(x.offset(56), x.offset(55));
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn idct64_stage10_avx2(x: *mut __m256i, cospi: *const i32, _r: __m256i, cos_bit: i8) {
    let _ = cos_bit;
    let cospi_m32_p32: __m256i =
        pair_set_w16_epi16(-*cospi.offset(32) as i16, *cospi.offset(32) as i16);
    let cospi_p32_p32: __m256i =
        pair_set_w16_epi16(*cospi.offset(32) as i16, *cospi.offset(32) as i16);
    btf_16_adds_subs_avx2(x, x.offset(31));
    btf_16_adds_subs_avx2(x.offset(1), x.offset(30));
    btf_16_adds_subs_avx2(x.offset(2), x.offset(29));
    btf_16_adds_subs_avx2(x.offset(3), x.offset(28));
    btf_16_adds_subs_avx2(x.offset(4), x.offset(27));
    btf_16_adds_subs_avx2(x.offset(5), x.offset(26));
    btf_16_adds_subs_avx2(x.offset(6), x.offset(25));
    btf_16_adds_subs_avx2(x.offset(7), x.offset(24));
    btf_16_adds_subs_avx2(x.offset(8), x.offset(23));
    btf_16_adds_subs_avx2(x.offset(9), x.offset(22));
    btf_16_adds_subs_avx2(x.offset(10), x.offset(21));
    btf_16_adds_subs_avx2(x.offset(11), x.offset(20));
    btf_16_adds_subs_avx2(x.offset(12), x.offset(19));
    btf_16_adds_subs_avx2(x.offset(13), x.offset(18));
    btf_16_adds_subs_avx2(x.offset(14), x.offset(17));
    btf_16_adds_subs_avx2(x.offset(15), x.offset(16));
    btf_16_w16_avx2(
        cospi_m32_p32,
        cospi_p32_p32,
        x.offset(40),
        x.offset(55),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m32_p32,
        cospi_p32_p32,
        x.offset(41),
        x.offset(54),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m32_p32,
        cospi_p32_p32,
        x.offset(42),
        x.offset(53),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m32_p32,
        cospi_p32_p32,
        x.offset(43),
        x.offset(52),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m32_p32,
        cospi_p32_p32,
        x.offset(44),
        x.offset(51),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m32_p32,
        cospi_p32_p32,
        x.offset(45),
        x.offset(50),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m32_p32,
        cospi_p32_p32,
        x.offset(46),
        x.offset(49),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m32_p32,
        cospi_p32_p32,
        x.offset(47),
        x.offset(48),
        _r,
        cos_bit as i32,
    );
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn idct64_stage11_avx2(output: *mut __m256i, x: *mut __m256i) {
    btf_16_adds_subs_out_avx2(output, output.offset(63), *x, *x.offset(63));
    btf_16_adds_subs_out_avx2(
        output.offset(1),
        output.offset(62),
        *x.offset(1),
        *x.offset(62),
    );
    btf_16_adds_subs_out_avx2(
        output.offset(2),
        output.offset(61),
        *x.offset(2),
        *x.offset(61),
    );
    btf_16_adds_subs_out_avx2(
        output.offset(3),
        output.offset(60),
        *x.offset(3),
        *x.offset(60),
    );
    btf_16_adds_subs_out_avx2(
        output.offset(4),
        output.offset(59),
        *x.offset(4),
        *x.offset(59),
    );
    btf_16_adds_subs_out_avx2(
        output.offset(5),
        output.offset(58),
        *x.offset(5),
        *x.offset(58),
    );
    btf_16_adds_subs_out_avx2(
        output.offset(6),
        output.offset(57),
        *x.offset(6),
        *x.offset(57),
    );
    btf_16_adds_subs_out_avx2(
        output.offset(7),
        output.offset(56),
        *x.offset(7),
        *x.offset(56),
    );
    btf_16_adds_subs_out_avx2(
        output.offset(8),
        output.offset(55),
        *x.offset(8),
        *x.offset(55),
    );
    btf_16_adds_subs_out_avx2(
        output.offset(9),
        output.offset(54),
        *x.offset(9),
        *x.offset(54),
    );
    btf_16_adds_subs_out_avx2(
        output.offset(10),
        output.offset(53),
        *x.offset(10),
        *x.offset(53),
    );
    btf_16_adds_subs_out_avx2(
        output.offset(11),
        output.offset(52),
        *x.offset(11),
        *x.offset(52),
    );
    btf_16_adds_subs_out_avx2(
        output.offset(12),
        output.offset(51),
        *x.offset(12),
        *x.offset(51),
    );
    btf_16_adds_subs_out_avx2(
        output.offset(13),
        output.offset(50),
        *x.offset(13),
        *x.offset(50),
    );
    btf_16_adds_subs_out_avx2(
        output.offset(14),
        output.offset(49),
        *x.offset(14),
        *x.offset(49),
    );
    btf_16_adds_subs_out_avx2(
        output.offset(15),
        output.offset(48),
        *x.offset(15),
        *x.offset(48),
    );
    btf_16_adds_subs_out_avx2(
        output.offset(16),
        output.offset(47),
        *x.offset(16),
        *x.offset(47),
    );
    btf_16_adds_subs_out_avx2(
        output.offset(17),
        output.offset(46),
        *x.offset(17),
        *x.offset(46),
    );
    btf_16_adds_subs_out_avx2(
        output.offset(18),
        output.offset(45),
        *x.offset(18),
        *x.offset(45),
    );
    btf_16_adds_subs_out_avx2(
        output.offset(19),
        output.offset(44),
        *x.offset(19),
        *x.offset(44),
    );
    btf_16_adds_subs_out_avx2(
        output.offset(20),
        output.offset(43),
        *x.offset(20),
        *x.offset(43),
    );
    btf_16_adds_subs_out_avx2(
        output.offset(21),
        output.offset(42),
        *x.offset(21),
        *x.offset(42),
    );
    btf_16_adds_subs_out_avx2(
        output.offset(22),
        output.offset(41),
        *x.offset(22),
        *x.offset(41),
    );
    btf_16_adds_subs_out_avx2(
        output.offset(23),
        output.offset(40),
        *x.offset(23),
        *x.offset(40),
    );
    btf_16_adds_subs_out_avx2(
        output.offset(24),
        output.offset(39),
        *x.offset(24),
        *x.offset(39),
    );
    btf_16_adds_subs_out_avx2(
        output.offset(25),
        output.offset(38),
        *x.offset(25),
        *x.offset(38),
    );
    btf_16_adds_subs_out_avx2(
        output.offset(26),
        output.offset(37),
        *x.offset(26),
        *x.offset(37),
    );
    btf_16_adds_subs_out_avx2(
        output.offset(27),
        output.offset(36),
        *x.offset(27),
        *x.offset(36),
    );
    btf_16_adds_subs_out_avx2(
        output.offset(28),
        output.offset(35),
        *x.offset(28),
        *x.offset(35),
    );
    btf_16_adds_subs_out_avx2(
        output.offset(29),
        output.offset(34),
        *x.offset(29),
        *x.offset(34),
    );
    btf_16_adds_subs_out_avx2(
        output.offset(30),
        output.offset(33),
        *x.offset(30),
        *x.offset(33),
    );
    btf_16_adds_subs_out_avx2(
        output.offset(31),
        output.offset(32),
        *x.offset(31),
        *x.offset(32),
    );
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn idct64_low1_avx2(input: *const __m256i, output: *mut __m256i) {
    let mut cospi: *const i32 = cospi_arr(INV_COS_BIT);
    // stage 1
    let mut x: [__m256i; 32] = core::mem::zeroed();
    x[0] = *input;
    // stage 2
    // stage 3
    // stage 4
    // stage 5
    // stage 6
    btf_16_w16_0_avx2!(*cospi.offset(32), *cospi.offset(32), x[0], x[0], x[1]);
    // stage 7
    // stage 8
    // stage 9
    // stage 10
    // stage 11
    *output = x[0];
    *output.offset(63) = x[0];
    *output.offset(1) = x[1];
    *output.offset(62) = x[1];
    *output.offset(2) = x[1];
    *output.offset(61) = x[1];
    *output.offset(3) = x[0];
    *output.offset(60) = x[0];
    *output.offset(4) = x[0];
    *output.offset(59) = x[0];
    *output.offset(5) = x[1];
    *output.offset(58) = x[1];
    *output.offset(6) = x[1];
    *output.offset(57) = x[1];
    *output.offset(7) = x[0];
    *output.offset(56) = x[0];
    *output.offset(8) = x[0];
    *output.offset(55) = x[0];
    *output.offset(9) = x[1];
    *output.offset(54) = x[1];
    *output.offset(10) = x[1];
    *output.offset(53) = x[1];
    *output.offset(11) = x[0];
    *output.offset(52) = x[0];
    *output.offset(12) = x[0];
    *output.offset(51) = x[0];
    *output.offset(13) = x[1];
    *output.offset(50) = x[1];
    *output.offset(14) = x[1];
    *output.offset(49) = x[1];
    *output.offset(15) = x[0];
    *output.offset(48) = x[0];
    *output.offset(16) = x[0];
    *output.offset(47) = x[0];
    *output.offset(17) = x[1];
    *output.offset(46) = x[1];
    *output.offset(18) = x[1];
    *output.offset(45) = x[1];
    *output.offset(19) = x[0];
    *output.offset(44) = x[0];
    *output.offset(20) = x[0];
    *output.offset(43) = x[0];
    *output.offset(21) = x[1];
    *output.offset(42) = x[1];
    *output.offset(22) = x[1];
    *output.offset(41) = x[1];
    *output.offset(23) = x[0];
    *output.offset(40) = x[0];
    *output.offset(24) = x[0];
    *output.offset(39) = x[0];
    *output.offset(25) = x[1];
    *output.offset(38) = x[1];
    *output.offset(26) = x[1];
    *output.offset(37) = x[1];
    *output.offset(27) = x[0];
    *output.offset(36) = x[0];
    *output.offset(28) = x[0];
    *output.offset(35) = x[0];
    *output.offset(29) = x[1];
    *output.offset(34) = x[1];
    *output.offset(30) = x[1];
    *output.offset(33) = x[1];
    *output.offset(31) = x[0];
    *output.offset(32) = x[0];
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn idct64_low8_avx2(input: *const __m256i, output: *mut __m256i) {
    let mut cospi: *const i32 = cospi_arr(INV_COS_BIT);
    let _r: __m256i = _mm256_set1_epi32(1 << (INV_COS_BIT - 1));
    let cospi_m04_p60: __m256i =
        pair_set_w16_epi16(-*cospi.offset(4) as i16, *cospi.offset(60) as i16);
    let cospi_p60_p04: __m256i =
        pair_set_w16_epi16(*cospi.offset(60) as i16, *cospi.offset(4) as i16);
    let cospi_m36_p28: __m256i =
        pair_set_w16_epi16(-*cospi.offset(36) as i16, *cospi.offset(28) as i16);
    let cospi_m28_m36: __m256i =
        pair_set_w16_epi16(-*cospi.offset(28) as i16, -*cospi.offset(36) as i16);
    let cospi_m20_p44: __m256i =
        pair_set_w16_epi16(-*cospi.offset(20) as i16, *cospi.offset(44) as i16);
    let cospi_p44_p20: __m256i =
        pair_set_w16_epi16(*cospi.offset(44) as i16, *cospi.offset(20) as i16);
    let cospi_m52_p12: __m256i =
        pair_set_w16_epi16(-*cospi.offset(52) as i16, *cospi.offset(12) as i16);
    let cospi_m12_m52: __m256i =
        pair_set_w16_epi16(-*cospi.offset(12) as i16, -*cospi.offset(52) as i16);
    let cospi_m08_p56: __m256i =
        pair_set_w16_epi16(-*cospi.offset(8) as i16, *cospi.offset(56) as i16);
    let cospi_p56_p08: __m256i =
        pair_set_w16_epi16(*cospi.offset(56) as i16, *cospi.offset(8) as i16);
    let cospi_m40_p24: __m256i =
        pair_set_w16_epi16(-*cospi.offset(40) as i16, *cospi.offset(24) as i16);
    let cospi_m24_m40: __m256i =
        pair_set_w16_epi16(-*cospi.offset(24) as i16, -*cospi.offset(40) as i16);
    let cospi_p32_p32: __m256i =
        pair_set_w16_epi16(*cospi.offset(32) as i16, *cospi.offset(32) as i16);
    let cospi_m16_p48: __m256i =
        pair_set_w16_epi16(-*cospi.offset(16) as i16, *cospi.offset(48) as i16);
    let cospi_p48_p16: __m256i =
        pair_set_w16_epi16(*cospi.offset(48) as i16, *cospi.offset(16) as i16);
    let cospi_m32_p32: __m256i =
        pair_set_w16_epi16(-*cospi.offset(32) as i16, *cospi.offset(32) as i16);
    // stage 1
    let mut x: [__m256i; 64] = core::mem::zeroed();
    x[0] = *input;
    x[8] = *input.offset(4);
    x[16] = *input.offset(2);
    x[24] = *input.offset(6);
    x[32] = *input.offset(1);
    x[40] = *input.offset(5);
    x[48] = *input.offset(3);
    x[56] = *input.offset(7);
    // stage 2
    btf_16_w16_0_avx2!(*cospi.offset(63), *cospi.offset(1), x[32], x[32], x[63]);
    btf_16_w16_0_avx2!(-*cospi.offset(57), *cospi.offset(7), x[56], x[39], x[56]);
    btf_16_w16_0_avx2!(*cospi.offset(59), *cospi.offset(5), x[40], x[40], x[55]);
    btf_16_w16_0_avx2!(-*cospi.offset(61), *cospi.offset(3), x[48], x[47], x[48]);
    // stage 3
    btf_16_w16_0_avx2!(*cospi.offset(62), *cospi.offset(2), x[16], x[16], x[31]);
    btf_16_w16_0_avx2!(-*cospi.offset(58), *cospi.offset(6), x[24], x[23], x[24]);
    x[33] = x[32];
    x[38] = x[39];
    x[41] = x[40];
    x[46] = x[47];
    x[49] = x[48];
    x[54] = x[55];
    x[57] = x[56];
    x[62] = x[63];
    // stage 4
    btf_16_w16_0_avx2!(*cospi.offset(60), *cospi.offset(4), x[8], x[8], x[15]);
    x[17] = x[16];
    x[22] = x[23];
    x[25] = x[24];
    x[30] = x[31];
    btf_16_w16_avx2(
        cospi_m04_p60,
        cospi_p60_p04,
        x.as_mut_ptr().offset(33),
        x.as_mut_ptr().offset(62),
        _r,
        INV_COS_BIT,
    );
    btf_16_w16_avx2(
        cospi_m28_m36,
        cospi_m36_p28,
        x.as_mut_ptr().offset(38),
        x.as_mut_ptr().offset(57),
        _r,
        INV_COS_BIT,
    );
    btf_16_w16_avx2(
        cospi_m20_p44,
        cospi_p44_p20,
        x.as_mut_ptr().offset(41),
        x.as_mut_ptr().offset(54),
        _r,
        INV_COS_BIT,
    );
    btf_16_w16_avx2(
        cospi_m12_m52,
        cospi_m52_p12,
        x.as_mut_ptr().offset(46),
        x.as_mut_ptr().offset(49),
        _r,
        INV_COS_BIT,
    );
    // stage 5
    x[9] = x[8];
    x[14] = x[15];
    btf_16_w16_avx2(
        cospi_m08_p56,
        cospi_p56_p08,
        x.as_mut_ptr().offset(17),
        x.as_mut_ptr().offset(30),
        _r,
        INV_COS_BIT,
    );
    btf_16_w16_avx2(
        cospi_m24_m40,
        cospi_m40_p24,
        x.as_mut_ptr().offset(22),
        x.as_mut_ptr().offset(25),
        _r,
        INV_COS_BIT,
    );
    x[35] = x[32];
    x[34] = x[33];
    x[36] = x[39];
    x[37] = x[38];
    x[43] = x[40];
    x[42] = x[41];
    x[44] = x[47];
    x[45] = x[46];
    x[51] = x[48];
    x[50] = x[49];
    x[52] = x[55];
    x[53] = x[54];
    x[59] = x[56];
    x[58] = x[57];
    x[60] = x[63];
    x[61] = x[62];
    // stage 6
    btf_16_w16_0_avx2!(*cospi.offset(32), *cospi.offset(32), x[0], x[0], x[1]);
    btf_16_w16_avx2(
        cospi_m16_p48,
        cospi_p48_p16,
        x.as_mut_ptr().offset(9),
        x.as_mut_ptr().offset(14),
        _r,
        INV_COS_BIT,
    );
    x[19] = x[16];
    x[18] = x[17];
    x[20] = x[23];
    x[21] = x[22];
    x[27] = x[24];
    x[26] = x[25];
    x[28] = x[31];
    x[29] = x[30];
    idct64_stage6_high32_avx2(x.as_mut_ptr(), cospi, _r, INV_COS_BIT as i8);
    // stage 7
    x[3] = x[0];
    x[2] = x[1];
    x[11] = x[8];
    x[10] = x[9];
    x[12] = x[15];
    x[13] = x[14];
    idct64_stage7_high48_avx2(x.as_mut_ptr(), cospi, _r, INV_COS_BIT as i8);
    // stage 8
    x[7] = x[0];
    x[6] = x[1];
    x[5] = x[2];
    x[4] = x[3];
    btf_16_w16_avx2(
        cospi_m32_p32,
        cospi_p32_p32,
        x.as_mut_ptr().offset(10),
        x.as_mut_ptr().offset(13),
        _r,
        INV_COS_BIT,
    );
    btf_16_w16_avx2(
        cospi_m32_p32,
        cospi_p32_p32,
        x.as_mut_ptr().offset(11),
        x.as_mut_ptr().offset(12),
        _r,
        INV_COS_BIT,
    );
    idct64_stage8_high48_avx2(x.as_mut_ptr(), cospi, _r, INV_COS_BIT as i8);
    idct64_stage9_avx2(x.as_mut_ptr(), cospi, _r, INV_COS_BIT as i8);
    idct64_stage10_avx2(x.as_mut_ptr(), cospi, _r, INV_COS_BIT as i8);
    idct64_stage11_avx2(output, x.as_mut_ptr());
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn idct64_low16_avx2(input: *const __m256i, output: *mut __m256i) {
    let mut cospi: *const i32 = cospi_arr(INV_COS_BIT);
    let _r: __m256i = _mm256_set1_epi32(1 << (INV_COS_BIT - 1));
    let cospi_p32_p32: __m256i =
        pair_set_w16_epi16(*cospi.offset(32) as i16, *cospi.offset(32) as i16);
    let cospi_m16_p48: __m256i =
        pair_set_w16_epi16(-*cospi.offset(16) as i16, *cospi.offset(48) as i16);
    let cospi_p48_p16: __m256i =
        pair_set_w16_epi16(*cospi.offset(48) as i16, *cospi.offset(16) as i16);
    let cospi_m48_m16: __m256i =
        pair_set_w16_epi16(-*cospi.offset(48) as i16, -*cospi.offset(16) as i16);
    let cospi_m32_p32: __m256i =
        pair_set_w16_epi16(-*cospi.offset(32) as i16, *cospi.offset(32) as i16);
    // stage 1
    let mut x: [__m256i; 64] = core::mem::zeroed();
    x[0] = *input;
    x[4] = *input.offset(8);
    x[8] = *input.offset(4);
    x[12] = *input.offset(12);
    x[16] = *input.offset(2);
    x[20] = *input.offset(10);
    x[24] = *input.offset(6);
    x[28] = *input.offset(14);
    x[32] = *input.offset(1);
    x[36] = *input.offset(9);
    x[40] = *input.offset(5);
    x[44] = *input.offset(13);
    x[48] = *input.offset(3);
    x[52] = *input.offset(11);
    x[56] = *input.offset(7);
    x[60] = *input.offset(15);
    // stage 2
    btf_16_w16_0_avx2!(*cospi.offset(63), *cospi.offset(1), x[32], x[32], x[63]);
    btf_16_w16_0_avx2!(-*cospi.offset(49), *cospi.offset(15), x[60], x[35], x[60]);
    btf_16_w16_0_avx2!(*cospi.offset(55), *cospi.offset(9), x[36], x[36], x[59]);
    btf_16_w16_0_avx2!(-*cospi.offset(57), *cospi.offset(7), x[56], x[39], x[56]);
    btf_16_w16_0_avx2!(*cospi.offset(59), *cospi.offset(5), x[40], x[40], x[55]);
    btf_16_w16_0_avx2!(-*cospi.offset(53), *cospi.offset(11), x[52], x[43], x[52]);
    btf_16_w16_0_avx2!(*cospi.offset(51), *cospi.offset(13), x[44], x[44], x[51]);
    btf_16_w16_0_avx2!(-*cospi.offset(61), *cospi.offset(3), x[48], x[47], x[48]);
    // stage 3
    btf_16_w16_0_avx2!(*cospi.offset(62), *cospi.offset(2), x[16], x[16], x[31]);
    btf_16_w16_0_avx2!(-*cospi.offset(50), *cospi.offset(14), x[28], x[19], x[28]);
    btf_16_w16_0_avx2!(*cospi.offset(54), *cospi.offset(10), x[20], x[20], x[27]);
    btf_16_w16_0_avx2!(-*cospi.offset(58), *cospi.offset(6), x[24], x[23], x[24]);
    x[33] = x[32];
    x[34] = x[35];
    x[37] = x[36];
    x[38] = x[39];
    x[41] = x[40];
    x[42] = x[43];
    x[45] = x[44];
    x[46] = x[47];
    x[49] = x[48];
    x[50] = x[51];
    x[53] = x[52];
    x[54] = x[55];
    x[57] = x[56];
    x[58] = x[59];
    x[61] = x[60];
    x[62] = x[63];
    // stage 4
    btf_16_w16_0_avx2!(*cospi.offset(60), *cospi.offset(4), x[8], x[8], x[15]);
    btf_16_w16_0_avx2!(-*cospi.offset(52), *cospi.offset(12), x[12], x[11], x[12]);
    x[17] = x[16];
    x[18] = x[19];
    x[21] = x[20];
    x[22] = x[23];
    x[25] = x[24];
    x[26] = x[27];
    x[29] = x[28];
    x[30] = x[31];
    idct64_stage4_high32_avx2(x.as_mut_ptr(), cospi, _r, INV_COS_BIT as i8);
    // stage 5
    btf_16_w16_0_avx2!(*cospi.offset(56), *cospi.offset(8), x[4], x[4], x[7]);
    x[9] = x[8];
    x[10] = x[11];
    x[13] = x[12];
    x[14] = x[15];
    idct64_stage5_high48_avx2(x.as_mut_ptr(), cospi, _r, INV_COS_BIT as i8);
    // stage 6
    btf_16_w16_0_avx2!(*cospi.offset(32), *cospi.offset(32), x[0], x[0], x[1]);
    x[5] = x[4];
    x[6] = x[7];
    btf_16_w16_avx2(
        cospi_m16_p48,
        cospi_p48_p16,
        x.as_mut_ptr().offset(9),
        x.as_mut_ptr().offset(14),
        _r,
        INV_COS_BIT,
    );
    btf_16_w16_avx2(
        cospi_m48_m16,
        cospi_m16_p48,
        x.as_mut_ptr().offset(10),
        x.as_mut_ptr().offset(13),
        _r,
        INV_COS_BIT,
    );
    idct64_stage6_high48_avx2(x.as_mut_ptr(), cospi, _r, INV_COS_BIT as i8);
    // stage 7
    x[3] = x[0];
    x[2] = x[1];
    btf_16_w16_avx2(
        cospi_m32_p32,
        cospi_p32_p32,
        x.as_mut_ptr().offset(5),
        x.as_mut_ptr().offset(6),
        _r,
        INV_COS_BIT,
    );
    btf_16_adds_subs_avx2(x.as_mut_ptr().offset(8), x.as_mut_ptr().offset(11));
    btf_16_adds_subs_avx2(x.as_mut_ptr().offset(9), x.as_mut_ptr().offset(10));
    btf_16_adds_subs_avx2(x.as_mut_ptr().offset(15), x.as_mut_ptr().offset(12));
    btf_16_adds_subs_avx2(x.as_mut_ptr().offset(14), x.as_mut_ptr().offset(13));
    idct64_stage7_high48_avx2(x.as_mut_ptr(), cospi, _r, INV_COS_BIT as i8);
    // stage 8
    btf_16_adds_subs_avx2(x.as_mut_ptr(), x.as_mut_ptr().offset(7));
    btf_16_adds_subs_avx2(x.as_mut_ptr().offset(1), x.as_mut_ptr().offset(6));
    btf_16_adds_subs_avx2(x.as_mut_ptr().offset(2), x.as_mut_ptr().offset(5));
    btf_16_adds_subs_avx2(x.as_mut_ptr().offset(3), x.as_mut_ptr().offset(4));
    btf_16_w16_avx2(
        cospi_m32_p32,
        cospi_p32_p32,
        x.as_mut_ptr().offset(10),
        x.as_mut_ptr().offset(13),
        _r,
        INV_COS_BIT,
    );
    btf_16_w16_avx2(
        cospi_m32_p32,
        cospi_p32_p32,
        x.as_mut_ptr().offset(11),
        x.as_mut_ptr().offset(12),
        _r,
        INV_COS_BIT,
    );
    idct64_stage8_high48_avx2(x.as_mut_ptr(), cospi, _r, INV_COS_BIT as i8);
    idct64_stage9_avx2(x.as_mut_ptr(), cospi, _r, INV_COS_BIT as i8);
    idct64_stage10_avx2(x.as_mut_ptr(), cospi, _r, INV_COS_BIT as i8);
    idct64_stage11_avx2(output, x.as_mut_ptr());
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn idct64_low32_avx2(input: *const __m256i, output: *mut __m256i) {
    let mut cospi: *const i32 = cospi_arr(INV_COS_BIT);
    let _r: __m256i = _mm256_set1_epi32(1 << (INV_COS_BIT - 1));
    let cospi_p32_p32: __m256i =
        pair_set_w16_epi16(*cospi.offset(32) as i16, *cospi.offset(32) as i16);
    let cospi_m16_p48: __m256i =
        pair_set_w16_epi16(-*cospi.offset(16) as i16, *cospi.offset(48) as i16);
    let cospi_p48_p16: __m256i =
        pair_set_w16_epi16(*cospi.offset(48) as i16, *cospi.offset(16) as i16);
    let cospi_m48_m16: __m256i =
        pair_set_w16_epi16(-*cospi.offset(48) as i16, -*cospi.offset(16) as i16);
    let cospi_m32_p32: __m256i =
        pair_set_w16_epi16(-*cospi.offset(32) as i16, *cospi.offset(32) as i16);
    // stage 1
    let mut x: [__m256i; 64] = core::mem::zeroed();
    x[0] = *input;
    x[2] = *input.offset(16);
    x[4] = *input.offset(8);
    x[6] = *input.offset(24);
    x[8] = *input.offset(4);
    x[10] = *input.offset(20);
    x[12] = *input.offset(12);
    x[14] = *input.offset(28);
    x[16] = *input.offset(2);
    x[18] = *input.offset(18);
    x[20] = *input.offset(10);
    x[22] = *input.offset(26);
    x[24] = *input.offset(6);
    x[26] = *input.offset(22);
    x[28] = *input.offset(14);
    x[30] = *input.offset(30);
    x[32] = *input.offset(1);
    x[34] = *input.offset(17);
    x[36] = *input.offset(9);
    x[38] = *input.offset(25);
    x[40] = *input.offset(5);
    x[42] = *input.offset(21);
    x[44] = *input.offset(13);
    x[46] = *input.offset(29);
    x[48] = *input.offset(3);
    x[50] = *input.offset(19);
    x[52] = *input.offset(11);
    x[54] = *input.offset(27);
    x[56] = *input.offset(7);
    x[58] = *input.offset(23);
    x[60] = *input.offset(15);
    x[62] = *input.offset(31);
    // stage 2
    btf_16_w16_0_avx2!(*cospi.offset(63), *cospi.offset(1), x[32], x[32], x[63]);
    btf_16_w16_0_avx2!(-*cospi.offset(33), *cospi.offset(31), x[62], x[33], x[62]);
    btf_16_w16_0_avx2!(*cospi.offset(47), *cospi.offset(17), x[34], x[34], x[61]);
    btf_16_w16_0_avx2!(-*cospi.offset(49), *cospi.offset(15), x[60], x[35], x[60]);
    btf_16_w16_0_avx2!(*cospi.offset(55), *cospi.offset(9), x[36], x[36], x[59]);
    btf_16_w16_0_avx2!(-*cospi.offset(41), *cospi.offset(23), x[58], x[37], x[58]);
    btf_16_w16_0_avx2!(*cospi.offset(39), *cospi.offset(25), x[38], x[38], x[57]);
    btf_16_w16_0_avx2!(-*cospi.offset(57), *cospi.offset(7), x[56], x[39], x[56]);
    btf_16_w16_0_avx2!(*cospi.offset(59), *cospi.offset(5), x[40], x[40], x[55]);
    btf_16_w16_0_avx2!(-*cospi.offset(37), *cospi.offset(27), x[54], x[41], x[54]);
    btf_16_w16_0_avx2!(*cospi.offset(43), *cospi.offset(21), x[42], x[42], x[53]);
    btf_16_w16_0_avx2!(-*cospi.offset(53), *cospi.offset(11), x[52], x[43], x[52]);
    btf_16_w16_0_avx2!(*cospi.offset(51), *cospi.offset(13), x[44], x[44], x[51]);
    btf_16_w16_0_avx2!(-*cospi.offset(45), *cospi.offset(19), x[50], x[45], x[50]);
    btf_16_w16_0_avx2!(*cospi.offset(35), *cospi.offset(29), x[46], x[46], x[49]);
    btf_16_w16_0_avx2!(-*cospi.offset(61), *cospi.offset(3), x[48], x[47], x[48]);
    // stage 3
    btf_16_w16_0_avx2!(*cospi.offset(62), *cospi.offset(2), x[16], x[16], x[31]);
    btf_16_w16_0_avx2!(-*cospi.offset(34), *cospi.offset(30), x[30], x[17], x[30]);
    btf_16_w16_0_avx2!(*cospi.offset(46), *cospi.offset(18), x[18], x[18], x[29]);
    btf_16_w16_0_avx2!(-*cospi.offset(50), *cospi.offset(14), x[28], x[19], x[28]);
    btf_16_w16_0_avx2!(*cospi.offset(54), *cospi.offset(10), x[20], x[20], x[27]);
    btf_16_w16_0_avx2!(-*cospi.offset(42), *cospi.offset(22), x[26], x[21], x[26]);
    btf_16_w16_0_avx2!(*cospi.offset(38), *cospi.offset(26), x[22], x[22], x[25]);
    btf_16_w16_0_avx2!(-*cospi.offset(58), *cospi.offset(6), x[24], x[23], x[24]);
    btf_16_adds_subs_avx2(x.as_mut_ptr().offset(32), x.as_mut_ptr().offset(33));
    btf_16_adds_subs_avx2(x.as_mut_ptr().offset(35), x.as_mut_ptr().offset(34));
    btf_16_adds_subs_avx2(x.as_mut_ptr().offset(36), x.as_mut_ptr().offset(37));
    btf_16_adds_subs_avx2(x.as_mut_ptr().offset(39), x.as_mut_ptr().offset(38));
    btf_16_adds_subs_avx2(x.as_mut_ptr().offset(40), x.as_mut_ptr().offset(41));
    btf_16_adds_subs_avx2(x.as_mut_ptr().offset(43), x.as_mut_ptr().offset(42));
    btf_16_adds_subs_avx2(x.as_mut_ptr().offset(44), x.as_mut_ptr().offset(45));
    btf_16_adds_subs_avx2(x.as_mut_ptr().offset(47), x.as_mut_ptr().offset(46));
    btf_16_adds_subs_avx2(x.as_mut_ptr().offset(48), x.as_mut_ptr().offset(49));
    btf_16_adds_subs_avx2(x.as_mut_ptr().offset(51), x.as_mut_ptr().offset(50));
    btf_16_adds_subs_avx2(x.as_mut_ptr().offset(52), x.as_mut_ptr().offset(53));
    btf_16_adds_subs_avx2(x.as_mut_ptr().offset(55), x.as_mut_ptr().offset(54));
    btf_16_adds_subs_avx2(x.as_mut_ptr().offset(56), x.as_mut_ptr().offset(57));
    btf_16_adds_subs_avx2(x.as_mut_ptr().offset(59), x.as_mut_ptr().offset(58));
    btf_16_adds_subs_avx2(x.as_mut_ptr().offset(60), x.as_mut_ptr().offset(61));
    btf_16_adds_subs_avx2(x.as_mut_ptr().offset(63), x.as_mut_ptr().offset(62));
    // stage 4
    btf_16_w16_0_avx2!(*cospi.offset(60), *cospi.offset(4), x[8], x[8], x[15]);
    btf_16_w16_0_avx2!(-*cospi.offset(36), *cospi.offset(28), x[14], x[9], x[14]);
    btf_16_w16_0_avx2!(*cospi.offset(44), *cospi.offset(20), x[10], x[10], x[13]);
    btf_16_w16_0_avx2!(-*cospi.offset(52), *cospi.offset(12), x[12], x[11], x[12]);
    btf_16_adds_subs_avx2(x.as_mut_ptr().offset(16), x.as_mut_ptr().offset(17));
    btf_16_adds_subs_avx2(x.as_mut_ptr().offset(19), x.as_mut_ptr().offset(18));
    btf_16_adds_subs_avx2(x.as_mut_ptr().offset(20), x.as_mut_ptr().offset(21));
    btf_16_adds_subs_avx2(x.as_mut_ptr().offset(23), x.as_mut_ptr().offset(22));
    btf_16_adds_subs_avx2(x.as_mut_ptr().offset(24), x.as_mut_ptr().offset(25));
    btf_16_adds_subs_avx2(x.as_mut_ptr().offset(27), x.as_mut_ptr().offset(26));
    btf_16_adds_subs_avx2(x.as_mut_ptr().offset(28), x.as_mut_ptr().offset(29));
    btf_16_adds_subs_avx2(x.as_mut_ptr().offset(31), x.as_mut_ptr().offset(30));
    idct64_stage4_high32_avx2(x.as_mut_ptr(), cospi, _r, INV_COS_BIT as i8);
    // stage 5
    btf_16_w16_0_avx2!(*cospi.offset(56), *cospi.offset(8), x[4], x[4], x[7]);
    btf_16_w16_0_avx2!(-*cospi.offset(40), *cospi.offset(24), x[6], x[5], x[6]);
    btf_16_adds_subs_avx2(x.as_mut_ptr().offset(8), x.as_mut_ptr().offset(9));
    btf_16_adds_subs_avx2(x.as_mut_ptr().offset(11), x.as_mut_ptr().offset(10));
    btf_16_adds_subs_avx2(x.as_mut_ptr().offset(12), x.as_mut_ptr().offset(13));
    btf_16_adds_subs_avx2(x.as_mut_ptr().offset(15), x.as_mut_ptr().offset(14));
    idct64_stage5_high48_avx2(x.as_mut_ptr(), cospi, _r, INV_COS_BIT as i8);
    // stage 6
    btf_16_w16_0_avx2!(*cospi.offset(32), *cospi.offset(32), x[0], x[0], x[1]);
    btf_16_w16_0_avx2!(*cospi.offset(48), *cospi.offset(16), x[2], x[2], x[3]);
    btf_16_adds_subs_avx2(x.as_mut_ptr().offset(4), x.as_mut_ptr().offset(5));
    btf_16_adds_subs_avx2(x.as_mut_ptr().offset(7), x.as_mut_ptr().offset(6));
    btf_16_w16_avx2(
        cospi_m16_p48,
        cospi_p48_p16,
        x.as_mut_ptr().offset(9),
        x.as_mut_ptr().offset(14),
        _r,
        INV_COS_BIT,
    );
    btf_16_w16_avx2(
        cospi_m48_m16,
        cospi_m16_p48,
        x.as_mut_ptr().offset(10),
        x.as_mut_ptr().offset(13),
        _r,
        INV_COS_BIT,
    );
    idct64_stage6_high48_avx2(x.as_mut_ptr(), cospi, _r, INV_COS_BIT as i8);
    // stage 7
    btf_16_adds_subs_avx2(x.as_mut_ptr(), x.as_mut_ptr().offset(3));
    btf_16_adds_subs_avx2(x.as_mut_ptr().offset(1), x.as_mut_ptr().offset(2));
    btf_16_w16_avx2(
        cospi_m32_p32,
        cospi_p32_p32,
        x.as_mut_ptr().offset(5),
        x.as_mut_ptr().offset(6),
        _r,
        INV_COS_BIT,
    );
    btf_16_adds_subs_avx2(x.as_mut_ptr().offset(8), x.as_mut_ptr().offset(11));
    btf_16_adds_subs_avx2(x.as_mut_ptr().offset(9), x.as_mut_ptr().offset(10));
    btf_16_adds_subs_avx2(x.as_mut_ptr().offset(15), x.as_mut_ptr().offset(12));
    btf_16_adds_subs_avx2(x.as_mut_ptr().offset(14), x.as_mut_ptr().offset(13));
    idct64_stage7_high48_avx2(x.as_mut_ptr(), cospi, _r, INV_COS_BIT as i8);
    // stage 8
    btf_16_adds_subs_avx2(x.as_mut_ptr(), x.as_mut_ptr().offset(7));
    btf_16_adds_subs_avx2(x.as_mut_ptr().offset(1), x.as_mut_ptr().offset(6));
    btf_16_adds_subs_avx2(x.as_mut_ptr().offset(2), x.as_mut_ptr().offset(5));
    btf_16_adds_subs_avx2(x.as_mut_ptr().offset(3), x.as_mut_ptr().offset(4));
    btf_16_w16_avx2(
        cospi_m32_p32,
        cospi_p32_p32,
        x.as_mut_ptr().offset(10),
        x.as_mut_ptr().offset(13),
        _r,
        INV_COS_BIT,
    );
    btf_16_w16_avx2(
        cospi_m32_p32,
        cospi_p32_p32,
        x.as_mut_ptr().offset(11),
        x.as_mut_ptr().offset(12),
        _r,
        INV_COS_BIT,
    );
    idct64_stage8_high48_avx2(x.as_mut_ptr(), cospi, _r, INV_COS_BIT as i8);
    // stage 9~11
    idct64_stage9_avx2(x.as_mut_ptr(), cospi, _r, INV_COS_BIT as i8);
    idct64_stage10_avx2(x.as_mut_ptr(), cospi, _r, INV_COS_BIT as i8);
    idct64_stage11_avx2(output, x.as_mut_ptr());
}

// 1D functions process 16 pixels at one time.
static lowbd_txfm_all_1d_zeros_w16_arr: [[[transform_1d_avx2; 4]; (ITX_TYPES_1D) as usize];
    (TX_SIZES) as usize] = [
    [
        [None, None, None, None],
        [None, None, None, None],
        [None, None, None, None],
    ],
    [
        [None, None, None, None],
        [None, None, None, None],
        [None, None, None, None],
    ],
    [
        [
            Some(idct16_low1_avx2),
            Some(idct16_low8_avx2),
            Some(idct16_avx2),
            None,
        ],
        [
            Some(iadst16_low1_avx2),
            Some(iadst16_low8_avx2),
            Some(iadst16_avx2),
            None,
        ],
        [None, None, None, None],
    ],
    [
        [
            Some(idct32_low1_avx2),
            Some(idct32_low8_avx2),
            Some(idct32_low16_avx2),
            Some(idct32_avx2),
        ],
        [None, None, None, None],
        [None, None, None, None],
    ],
    [
        [
            Some(idct64_low1_avx2),
            Some(idct64_low8_avx2),
            Some(idct64_low16_avx2),
            Some(idct64_low32_avx2),
        ],
        [None, None, None, None],
        [None, None, None, None],
    ],
];

// only process w >= 16 h >= 16
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn lowbd_inv_txfm2d_add_no_identity_avx2(
    input: *const i32,
    output: *mut u8,
    stride: i32,
    tx_type: u8,
    tx_size: u8,
    eob: i32,
) {
    let mut buf1: [__m256i; (64 * 16) as usize] = core::mem::zeroed();
    let mut eobx: i32 = 0;
    let mut eoby: i32 = 0;
    get_eobx_eoby_scan_default(&raw mut eobx, &raw mut eoby, tx_size, eob);
    let mut shift: *const i8 = av1_inv_txfm_shift_ls[tx_size as usize];
    let txw_idx: i32 = get_txw_idx(tx_size);
    let txh_idx: i32 = get_txh_idx(tx_size);
    let txfm_size_col: i32 = tx_size_wide[tx_size as usize];
    let txfm_size_row: i32 = tx_size_high[tx_size as usize];
    let buf_size_w_div16: i32 = txfm_size_col >> 4;
    let buf_size_nonzero_w: i32 = ((eobx + 16) >> 4) << 4;
    let buf_size_nonzero_h_div16: i32 = (eoby + 16) >> 4;
    let input_stride: i32 = AOMMIN(32, txfm_size_row);
    let rect_type: i32 = get_rect_tx_log_ratio(txfm_size_col, txfm_size_row);
    let fun_idx_x: i32 = lowbd_txfm_all_1d_zeros_idx[eobx as usize];
    let fun_idx_y: i32 = lowbd_txfm_all_1d_zeros_idx[eoby as usize];
    let row_txfm: transform_1d_avx2 = lowbd_txfm_all_1d_zeros_w16_arr[txw_idx as usize]
        [hitx_1d_tab[tx_type as usize] as usize][fun_idx_x as usize];
    let col_txfm: transform_1d_avx2 = lowbd_txfm_all_1d_zeros_w16_arr[txh_idx as usize]
        [vitx_1d_tab[tx_type as usize] as usize][fun_idx_y as usize];
    debug_assert!(!col_txfm.is_none());
    debug_assert!(!row_txfm.is_none());
    let mut ud_flip: i32 = 0;
    let mut lr_flip: i32 = 0;
    get_flip_cfg(tx_type, &raw mut ud_flip, &raw mut lr_flip);
    let scale0: __m256i = _mm256_set1_epi16((1 << (15 + *shift as i32)) as i16);
    for i in 0..buf_size_nonzero_h_div16 {
        let mut buf0: [__m256i; 64] = core::mem::zeroed();
        load_buffer_32bit_to_16bit_w16_avx2(
            input.offset((16 * i) as isize),
            input_stride,
            buf0.as_mut_ptr(),
            buf_size_nonzero_w,
        );
        if rect_type == 1 || rect_type == -1 {
            round_shift_avx2(
                buf0.as_mut_ptr() as *const __m256i,
                buf0.as_mut_ptr(),
                buf_size_nonzero_w,
            );
        } // rect special code
        row_txfm.unwrap()(buf0.as_mut_ptr() as *const __m256i, buf0.as_mut_ptr());
        for j in 0..txfm_size_col {
            buf0[j as usize] = _mm256_mulhrs_epi16(buf0[j as usize], scale0);
        }
        let mut buf1_cur: *mut __m256i = buf1.as_mut_ptr().offset((i << 4) as isize);
        if lr_flip != 0 {
            for j in 0..buf_size_w_div16 {
                let mut temp: [__m256i; 16] = core::mem::zeroed();
                flip_buf_avx2(
                    buf0.as_mut_ptr().offset((16 * j) as isize),
                    temp.as_mut_ptr(),
                    16,
                );
                let mut offset: i32 = txfm_size_row * (buf_size_w_div16 - 1 - j);
                transpose_16bit_16x16_avx2(
                    temp.as_mut_ptr() as *const __m256i,
                    buf1_cur.offset(offset as isize),
                );
            }
        } else {
            for j in 0..buf_size_w_div16 {
                transpose_16bit_16x16_avx2(
                    buf0.as_mut_ptr().offset((16 * j) as isize) as *const __m256i,
                    buf1_cur.offset((txfm_size_row * j) as isize),
                );
            }
        }
    }
    let scale1: __m256i = _mm256_set1_epi16((1 << (15 + *shift.offset(1) as i32)) as i16);
    for i in 0..buf_size_w_div16 {
        let mut buf1_cur: *mut __m256i = buf1.as_mut_ptr().offset((i * txfm_size_row) as isize);
        col_txfm.unwrap()(buf1_cur as *const __m256i, buf1_cur);
        for j in 0..txfm_size_row {
            *buf1_cur.offset(j as isize) =
                _mm256_mulhrs_epi16(*buf1_cur.offset(j as isize), scale1);
        }
    }
    for i in 0..buf_size_w_div16 {
        lowbd_write_buffer_16xn_avx2(
            buf1.as_mut_ptr().offset((i * txfm_size_row) as isize),
            output.offset((16 * i) as isize),
            stride,
            ud_flip,
            txfm_size_row,
        );
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn iidentity_row_16xn_avx2(
    out: *mut __m256i,
    input: *const i32,
    stride: i32,
    shift: i32,
    height: i32,
    txw_idx: i32,
    rect_type: i32,
) {
    let mut input_row: *const i32 = input;
    let scale: __m256i = _mm256_set1_epi16(NewSqrt2list[txw_idx as usize] as i16);
    let _r: __m256i =
        _mm256_set1_epi16(((1 << (NewSqrt2Bits - 1)) + (1 << (NewSqrt2Bits - shift - 1))) as i16);
    let one: __m256i = _mm256_set1_epi16(1);
    let scale__r: __m256i = _mm256_unpacklo_epi16(scale, _r);
    if rect_type != 1 && rect_type != -1 {
        for i in 0..height {
            let src: __m256i = load_32bit_to_16bit_w16_avx2(input_row);
            input_row = input_row.offset(stride as isize);
            let mut lo: __m256i = _mm256_unpacklo_epi16(src, one);
            let mut hi: __m256i = _mm256_unpackhi_epi16(src, one);
            lo = _mm256_madd_epi16(lo, scale__r);
            hi = _mm256_madd_epi16(hi, scale__r);
            lo = _mm256_sra_epi32(lo, _mm_cvtsi32_si128(NewSqrt2Bits - shift));
            hi = _mm256_sra_epi32(hi, _mm_cvtsi32_si128(NewSqrt2Bits - shift));
            *out.offset(i as isize) = _mm256_packs_epi32(lo, hi);
        }
    } else {
        let rect_scale: __m256i = _mm256_set1_epi16((NewInvSqrt2 << (15 - NewSqrt2Bits)) as i16);
        for i in 0..height {
            let mut src: __m256i = load_32bit_to_16bit_w16_avx2(input_row);
            src = _mm256_mulhrs_epi16(src, rect_scale);
            input_row = input_row.offset(stride as isize);
            let mut lo: __m256i = _mm256_unpacklo_epi16(src, one);
            let mut hi: __m256i = _mm256_unpackhi_epi16(src, one);
            lo = _mm256_madd_epi16(lo, scale__r);
            hi = _mm256_madd_epi16(hi, scale__r);
            lo = _mm256_sra_epi32(lo, _mm_cvtsi32_si128(NewSqrt2Bits - shift));
            hi = _mm256_sra_epi32(hi, _mm_cvtsi32_si128(NewSqrt2Bits - shift));
            *out.offset(i as isize) = _mm256_packs_epi32(lo, hi);
        }
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn iidentity_col_16xn_avx2(
    mut output: *mut u8,
    stride: i32,
    buf: *mut __m256i,
    shift: i32,
    height: i32,
    txh_idx: i32,
) {
    let scale: __m256i = _mm256_set1_epi16(NewSqrt2list[txh_idx as usize] as i16);
    let scale__r: __m256i = _mm256_set1_epi16((1 << (NewSqrt2Bits - 1)) as i16);
    let shift__r: __m256i = _mm256_set1_epi32(1 << (-shift - 1));
    let one: __m256i = _mm256_set1_epi16(1);
    let scale_coeff: __m256i = _mm256_unpacklo_epi16(scale, scale__r);
    for h in 0..height {
        let mut lo: __m256i = _mm256_unpacklo_epi16(*buf.offset(h as isize), one);
        let mut hi: __m256i = _mm256_unpackhi_epi16(*buf.offset(h as isize), one);
        lo = _mm256_madd_epi16(lo, scale_coeff);
        hi = _mm256_madd_epi16(hi, scale_coeff);
        lo = _mm256_srai_epi32::<{ NewSqrt2Bits }>(lo);
        hi = _mm256_srai_epi32::<{ NewSqrt2Bits }>(hi);
        lo = _mm256_add_epi32(lo, shift__r);
        hi = _mm256_add_epi32(hi, shift__r);
        lo = _mm256_sra_epi32(lo, _mm_cvtsi32_si128(-shift));
        hi = _mm256_sra_epi32(hi, _mm_cvtsi32_si128(-shift));
        let x: __m256i = _mm256_packs_epi32(lo, hi);
        write_recon_w16_avx2(x, output);
        output = output.offset(stride as isize);
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn lowbd_inv_txfm2d_add_idtx_avx2(
    input: *const i32,
    output: *mut u8,
    stride: i32,
    tx_size: u8,
    eob: i32,
) {
    let _ = eob;
    let mut shift: *const i8 = av1_inv_txfm_shift_ls[tx_size as usize];
    let txw_idx: i32 = get_txw_idx(tx_size);
    let txh_idx: i32 = get_txh_idx(tx_size);
    let txfm_size_col: i32 = tx_size_wide[tx_size as usize];
    let txfm_size_row: i32 = tx_size_high[tx_size as usize];
    let col_max: i32 = AOMMIN(32, txfm_size_col);
    let row_max: i32 = AOMMIN(32, txfm_size_row);
    let input_stride: i32 = row_max;
    let rect_type: i32 = get_rect_tx_log_ratio(txfm_size_col, txfm_size_row);
    let mut buf: [__m256i; 32] = core::mem::zeroed();
    for i in 0..(col_max >> 4) {
        for j in 0..(row_max >> 4) {
            iidentity_row_16xn_avx2(
                buf.as_mut_ptr(),
                input
                    .offset((j * 16) as isize)
                    .offset((i * 16 * input_stride) as isize),
                row_max,
                *shift as i32,
                16,
                txw_idx,
                rect_type,
            );
            transpose_16bit_16x16_avx2(buf.as_mut_ptr() as *const __m256i, buf.as_mut_ptr());
            iidentity_col_16xn_avx2(
                output
                    .offset((i * 16) as isize)
                    .offset((j * 16 * stride) as isize),
                stride,
                buf.as_mut_ptr(),
                *shift.offset(1) as i32,
                16,
                txh_idx,
            );
        }
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn lowbd_inv_txfm2d_add_h_identity_avx2(
    input: *const i32,
    output: *mut u8,
    stride: i32,
    tx_type: u8,
    tx_size: u8,
    eob: i32,
) {
    let mut eobx: i32 = 0;
    let mut eoby: i32 = 0;
    get_eobx_eoby_scan_h_identity(&raw mut eobx, &raw mut eoby, tx_size, eob);
    let mut shift: *const i8 = av1_inv_txfm_shift_ls[tx_size as usize];
    let txw_idx: i32 = get_txw_idx(tx_size);
    let txh_idx: i32 = get_txh_idx(tx_size);
    let txfm_size_col: i32 = tx_size_wide[tx_size as usize];
    let txfm_size_row: i32 = tx_size_high[tx_size as usize];
    let txfm_size_row_notzero: i32 = AOMMIN(32, txfm_size_row);
    let input_stride: i32 = txfm_size_row_notzero;
    let buf_size_w_div16: i32 = (eobx + 16) >> 4;
    let buf_size_h_div16: i32 = (eoby + 16) >> 4;
    let rect_type: i32 = get_rect_tx_log_ratio(txfm_size_col, txfm_size_row);
    let fun_idx_y: i32 = lowbd_txfm_all_1d_zeros_idx[eoby as usize];
    let col_txfm: transform_1d_avx2 = lowbd_txfm_all_1d_zeros_w16_arr[txh_idx as usize]
        [vitx_1d_tab[tx_type as usize] as usize][fun_idx_y as usize];
    debug_assert!(!col_txfm.is_none());
    let mut ud_flip: i32 = 0;
    let mut lr_flip: i32 = 0;
    get_flip_cfg(tx_type, &raw mut ud_flip, &raw mut lr_flip);
    for i in 0..buf_size_w_div16 {
        let mut buf0: [__m256i; 64] = core::mem::zeroed();
        for j in 0..buf_size_h_div16 {
            let mut buf0_cur: *mut __m256i = buf0.as_mut_ptr().offset((j * 16) as isize);
            let mut input_cur: *const i32 = input
                .offset((i * 16 * input_stride) as isize)
                .offset((j * 16) as isize);
            iidentity_row_16xn_avx2(
                buf0_cur,
                input_cur,
                input_stride,
                *shift as i32,
                16,
                txw_idx,
                rect_type,
            );
            transpose_16bit_16x16_avx2(buf0_cur as *const __m256i, buf0_cur);
        }
        col_txfm.unwrap()(buf0.as_mut_ptr() as *const __m256i, buf0.as_mut_ptr());
        let mut mshift: __m256i = _mm256_set1_epi16((1 << (15 + *shift.offset(1) as i32)) as i16);
        let mut k: i32 = if ud_flip != 0 { txfm_size_row - 1 } else { 0 };
        let step: i32 = if ud_flip != 0 { -1 } else { 1 };
        {
            let mut j: i32 = 0;
            while j < txfm_size_row {
                let mut res: __m256i = _mm256_mulhrs_epi16(buf0[k as usize], mshift);
                write_recon_w16_avx2(
                    res,
                    output
                        .offset((i << 4) as isize)
                        .offset((j * stride) as isize),
                );
                j += 1;
                k += step;
            }
        }
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn lowbd_inv_txfm2d_add_v_identity_avx2(
    input: *const i32,
    output: *mut u8,
    stride: i32,
    tx_type: u8,
    tx_size: u8,
    eob: i32,
) {
    let mut buf1: [__m256i; 64] = core::mem::zeroed();
    let mut eobx: i32 = 0;
    let mut eoby: i32 = 0;
    get_eobx_eoby_scan_v_identity(&raw mut eobx, &raw mut eoby, tx_size, eob);
    let mut shift: *const i8 = av1_inv_txfm_shift_ls[tx_size as usize];
    let txw_idx: i32 = get_txw_idx(tx_size);
    let txh_idx: i32 = get_txh_idx(tx_size);
    let txfm_size_col: i32 = tx_size_wide[tx_size as usize];
    let txfm_size_row: i32 = tx_size_high[tx_size as usize];
    let buf_size_w_div16: i32 = txfm_size_col >> 4;
    let buf_size_h_div16: i32 = (eoby + 16) >> 4;
    let buf_size_nonzero_w: i32 = ((eobx + 8) >> 3) << 3;
    let input_stride: i32 = AOMMIN(32, txfm_size_row);
    let rect_type: i32 = get_rect_tx_log_ratio(txfm_size_col, txfm_size_row);
    let fun_idx_x: i32 = lowbd_txfm_all_1d_zeros_idx[eobx as usize];
    let row_txfm: transform_1d_avx2 = lowbd_txfm_all_1d_zeros_w16_arr[txw_idx as usize]
        [hitx_1d_tab[tx_type as usize] as usize][fun_idx_x as usize];
    debug_assert!(!row_txfm.is_none());
    let mut ud_flip: i32 = 0;
    let mut lr_flip: i32 = 0;
    get_flip_cfg(tx_type, &raw mut ud_flip, &raw mut lr_flip);
    for i in 0..buf_size_h_div16 {
        let mut buf0: [__m256i; 64] = core::mem::zeroed();
        load_buffer_32bit_to_16bit_w16_avx2(
            input.offset((i * 16) as isize),
            input_stride,
            buf0.as_mut_ptr(),
            buf_size_nonzero_w,
        );
        if rect_type == 1 || rect_type == -1 {
            round_shift_avx2(
                buf0.as_mut_ptr() as *const __m256i,
                buf0.as_mut_ptr(),
                buf_size_nonzero_w,
            );
        } // rect special code
        row_txfm.unwrap()(buf0.as_mut_ptr() as *const __m256i, buf0.as_mut_ptr());
        round_shift_16bit_w16_avx2(buf0.as_mut_ptr(), txfm_size_col, *shift as i32);
        let mut _buf1: *mut __m256i = buf1.as_mut_ptr();
        if lr_flip != 0 {
            for j in 0..buf_size_w_div16 {
                let mut temp: [__m256i; 16] = core::mem::zeroed();
                flip_buf_avx2(
                    buf0.as_mut_ptr().offset((16 * j) as isize),
                    temp.as_mut_ptr(),
                    16,
                );
                transpose_16bit_16x16_avx2(
                    temp.as_mut_ptr() as *const __m256i,
                    _buf1.offset((16 * (buf_size_w_div16 - 1 - j)) as isize),
                );
            }
        } else {
            for j in 0..buf_size_w_div16 {
                transpose_16bit_16x16_avx2(
                    buf0.as_mut_ptr().offset((16 * j) as isize) as *const __m256i,
                    _buf1.offset((16 * j) as isize),
                );
            }
        }
        for j in 0..buf_size_w_div16 {
            iidentity_col_16xn_avx2(
                output
                    .offset((i * 16 * stride) as isize)
                    .offset((j * 16) as isize),
                stride,
                buf1.as_mut_ptr().offset((j * 16) as isize),
                *shift.offset(1) as i32,
                16,
                txh_idx,
            );
        }
    }
}

static lowbd_txfm_all_1d_zeros_8x8_arr: [[transform_1d_ssse3; 2]; 2] = [
    [Some(av1_idct8_low1_ssse3), Some(av1_idct8_sse2)],
    [Some(av1_iadst8_low1_ssse3), Some(av1_iadst8_sse2)],
];

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn load_buffer_avx2(in_: *const i32, stride: i32, out: *mut __m128i) {
    let a: __m256i = _mm256_load_si256(in_ as *const __m256i);
    let b: __m256i = _mm256_load_si256(in_.offset((stride * 1) as isize) as *const __m256i);
    let c: __m256i = _mm256_load_si256(in_.offset((stride * 2) as isize) as *const __m256i);
    let d: __m256i = _mm256_load_si256(in_.offset((stride * 3) as isize) as *const __m256i);
    let e: __m256i = _mm256_load_si256(in_.offset((stride * 4) as isize) as *const __m256i);
    let f: __m256i = _mm256_load_si256(in_.offset((stride * 5) as isize) as *const __m256i);
    let g: __m256i = _mm256_load_si256(in_.offset((stride * 6) as isize) as *const __m256i);
    let h: __m256i = _mm256_load_si256(in_.offset((stride * 7) as isize) as *const __m256i);
    // a0 a1 a2 a3 b0 b1 b2 b3 a4 a5 a6 a7 b4 b5 b6 b7
    let ab_16bit: __m256i = _mm256_packs_epi32(a, b);
    // c0 c1 c2 c3 d0 d1 d2 d3 c4 c5 c6 c7 d4 d5 d6 d7
    let cd_16bit: __m256i = _mm256_packs_epi32(c, d);
    // e0 e1 e2 e3 f0 f1 f2 f3 e4 e5 e6 e7 f4 f5 f6 f7
    let ef_16bit: __m256i = _mm256_packs_epi32(e, f);
    // g0 g1 g2 g3 h0 h1 h2 h3 g4 g5 g6 g7 h4 h5 h6 h7
    let gh_16bit: __m256i = _mm256_packs_epi32(g, h);
    // a0 a1 a2 a3 a4 a5 a6 a7 b0 b1 b2 b3 b4 b5 b6 b7
    let ab: __m256i = _mm256_permute4x64_epi64::<0xd8>(ab_16bit);
    // c0 c1 c2 c3 c4 c5 c6 c7 d0 d1 d2 d3 d4 d5 d6 d7
    let cd: __m256i = _mm256_permute4x64_epi64::<0xd8>(cd_16bit);
    // e0 e1 e2 e3 e4 e5 e6 e7 f0 f1 f2 f3 f4 f5 f6 f7
    let ef: __m256i = _mm256_permute4x64_epi64::<0xd8>(ef_16bit);
    // g0 g1 g2 g3 g4 g5 g6 g7 h0 h1 h2 h3 h4 h5 h6 h7
    let gh: __m256i = _mm256_permute4x64_epi64::<0xd8>(gh_16bit);
    *out = _mm256_castsi256_si128(ab);
    *out.offset(1) = _mm256_extractf128_si256::<1>(ab);
    *out.offset(2) = _mm256_castsi256_si128(cd);
    *out.offset(3) = _mm256_extractf128_si256::<1>(cd);
    *out.offset(4) = _mm256_castsi256_si128(ef);
    *out.offset(5) = _mm256_extractf128_si256::<1>(ef);
    *out.offset(6) = _mm256_castsi256_si128(gh);
    *out.offset(7) = _mm256_extractf128_si256::<1>(gh);
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn round_and_transpose_avx2(
    in_: *const __m128i,
    out: *mut __m128i,
    bit: i32,
    lr_flip: *mut i32,
) {
    let mut buf_temp: [__m256i; 4] = core::mem::zeroed();
    let scale: __m256i = _mm256_set1_epi16((1 << (15 + bit)) as i16);
    let mut j: i32 = if *lr_flip != 0 { 7 } else { 0 };
    let step: i32 = if *lr_flip != 0 { -1 } else { 1 };
    // 70 71 72 73 74 75 76 77 | 30 31 32 33 34 35 36 37
    buf_temp[0] = _mm256_inserti128_si256::<1>(
        _mm256_castsi128_si256(*in_.offset(j as isize)),
        *in_.offset((j + 4 * step) as isize),
    );
    j += step;
    // 60 61 62 63 64 65 66 67 | 20 21 22 23 24 25 26 27
    buf_temp[1] = _mm256_inserti128_si256::<1>(
        _mm256_castsi128_si256(*in_.offset(j as isize)),
        *in_.offset((j + 4 * step) as isize),
    );
    j += step;
    // 50 51 52 53 54 55 56 57 | 10 11 12 13 14 15 16 17
    buf_temp[2] = _mm256_inserti128_si256::<1>(
        _mm256_castsi128_si256(*in_.offset(j as isize)),
        *in_.offset((j + 4 * step) as isize),
    );
    j += step;
    // 40 41 42 43 44 45 46 47 | 00 01 02 03 04 05 06 07
    buf_temp[3] = _mm256_inserti128_si256::<1>(
        _mm256_castsi128_si256(*in_.offset(j as isize)),
        *in_.offset((j + 4 * step) as isize),
    );
    // 70 71 72 73 74 75 76 77 | 30 31 32 33 34 35 36 37
    buf_temp[0] = _mm256_mulhrs_epi16(buf_temp[0], scale);
    // 60 61 62 63 64 65 66 67 | 20 21 22 23 24 25 26 27
    buf_temp[1] = _mm256_mulhrs_epi16(buf_temp[1], scale);
    // 50 51 52 53 54 55 56 57 | 10 11 12 13 14 15 16 17
    buf_temp[2] = _mm256_mulhrs_epi16(buf_temp[2], scale);
    // 40 41 42 43 44 45 46 47 | 00 01 02 03 04 05 06 07
    buf_temp[3] = _mm256_mulhrs_epi16(buf_temp[3], scale);
    // 70 60 71 61 72 62 73 63 | 30 20 31 21 32 22 33 23
    let unpcklo0: __m256i = _mm256_unpacklo_epi16(buf_temp[0], buf_temp[1]);
    // 74 64 75 65 76 66 77 67 | 34 24 35 25 36 26 37 27
    let unpckhi0: __m256i = _mm256_unpackhi_epi16(buf_temp[0], buf_temp[1]);
    // 50 40 51 41 52 42 53 43 | 10 00 11 01 12 02 13 03
    let unpcklo1: __m256i = _mm256_unpacklo_epi16(buf_temp[2], buf_temp[3]);
    // 54 44 55 45 56 46 57 47 | 14 04 15 05 16 06 17 07
    let unpckhi1: __m256i = _mm256_unpackhi_epi16(buf_temp[2], buf_temp[3]);
    // 70 60 50 40 71 61 51 41 | 30 20 10 00 31 21 11 01
    let unpcklo00: __m256i = _mm256_unpacklo_epi32(unpcklo0, unpcklo1);
    // 72 62 52 42 73 63 53 43 | 32 22 12 02 33 23 13 03
    let unpckhi00: __m256i = _mm256_unpackhi_epi32(unpcklo0, unpcklo1);
    // 74 64 54 44 75 65 55 45 | 34 24 14 04 35 25 15 05
    let unpcklo01: __m256i = _mm256_unpacklo_epi32(unpckhi0, unpckhi1);
    // 76 66 56 46 77 67 57 47 | 36 26 16 06 37 27 17 07
    let unpckhi01: __m256i = _mm256_unpackhi_epi32(unpckhi0, unpckhi1);
    // 70 60 50 40 30 20 10 00 | 71 61 51 41 31 21 11 01
    let reg_00: __m256i = _mm256_permute4x64_epi64::<0xd8>(unpcklo00);
    // 72 62 52 42 32 22 12 02 | 73 63 53 43 33 23 13 03
    let reg_01: __m256i = _mm256_permute4x64_epi64::<0xd8>(unpckhi00);
    // 74 64 54 44 34 24 14 04 | 75 65 55 45 35 25 15 05
    let reg_10: __m256i = _mm256_permute4x64_epi64::<0xd8>(unpcklo01);
    // 76 66 56 46 36 26 16 06 | 77 67 57 47 37 27 17 07
    let reg_11: __m256i = _mm256_permute4x64_epi64::<0xd8>(unpckhi01);
    // 70 60 50 40 30 20 10 00
    *out = _mm256_castsi256_si128(reg_00);
    // 71 61 51 41 31 21 11 01
    *out.offset(1) = _mm256_extracti128_si256::<1>(reg_00);
    // 72 62 52 42 32 22 12 02
    *out.offset(2) = _mm256_castsi256_si128(reg_01);
    // 73 63 53 43 33 23 13 03
    *out.offset(3) = _mm256_extracti128_si256::<1>(reg_01);
    // 74 64 54 44 34 24 14 04
    *out.offset(4) = _mm256_castsi256_si128(reg_10);
    // 75 65 55 45 35 25 15 05
    *out.offset(5) = _mm256_extracti128_si256::<1>(reg_10);
    // 76 66 56 46 36 26 16 06
    *out.offset(6) = _mm256_castsi256_si128(reg_11);
    // 77 67 57 47 37 27 17 07
    *out.offset(7) = _mm256_extracti128_si256::<1>(reg_11);
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn round_shift_lowbd_write_buffer_avx2(
    in_: *mut __m128i,
    bit: i32,
    output: *mut u8,
    stride: i32,
    flipud: i32,
) {
    let mut in_256: [__m256i; 4] = core::mem::zeroed();
    let mut v_256: [__m256i; 4] = core::mem::zeroed();
    let mut j: i32 = if flipud != 0 { 7 } else { 0 };
    let step: i32 = if flipud != 0 { -1 } else { 1 };
    let scale: __m256i = _mm256_set1_epi16((1 << (15 + bit)) as i16);
    let zero: __m256i = _mm256_setzero_si256();
    // in[0], in[1]
    in_256[0] = _mm256_inserti128_si256::<1>(
        _mm256_castsi128_si256(*in_.offset(j as isize)),
        *in_.offset((j + step) as isize),
    );
    j += 2 * step;
    // in[2], in[3]
    in_256[1] = _mm256_inserti128_si256::<1>(
        _mm256_castsi128_si256(*in_.offset(j as isize)),
        *in_.offset((j + step) as isize),
    );
    j += 2 * step;
    // in[4], in[5]
    in_256[2] = _mm256_inserti128_si256::<1>(
        _mm256_castsi128_si256(*in_.offset(j as isize)),
        *in_.offset((j + step) as isize),
    );
    j += 2 * step;
    // in[6], in[7]
    in_256[3] = _mm256_inserti128_si256::<1>(
        _mm256_castsi128_si256(*in_.offset(j as isize)),
        *in_.offset((j + step) as isize),
    );
    // i00 i01 i02 i03 i04 i05 i06 i07 i10 i11 i12 i13 i14 i15 i16 i17
    in_256[0] = _mm256_mulhrs_epi16(in_256[0], scale);
    // i20 i21 i22 i23 i24 i25 i26 i27 i30 i31 i32 i33 i34 i35 i36 i37
    in_256[1] = _mm256_mulhrs_epi16(in_256[1], scale);
    // i40 i41 i42 i43 i44 i45 i46 i47 i50 i51 i52 i53 i54 i55 i56 i57
    in_256[2] = _mm256_mulhrs_epi16(in_256[2], scale);
    // i60 i61 i62 i63 i64 i65 i66 i67 i70 i71 i72 i73 i74 i75 i76 i77
    in_256[3] = _mm256_mulhrs_epi16(in_256[3], scale);
    let v0: __m128i = _mm_loadl_epi64(output as *const __m128i);
    let v1: __m128i = _mm_loadl_epi64(output.offset(stride as isize) as *const __m128i);
    let v2: __m128i = _mm_loadl_epi64(output.offset((2 * stride) as isize) as *const __m128i);
    let v3: __m128i = _mm_loadl_epi64(output.offset((3 * stride) as isize) as *const __m128i);
    let v4: __m128i = _mm_loadl_epi64(output.offset((4 * stride) as isize) as *const __m128i);
    let v5: __m128i = _mm_loadl_epi64(output.offset((5 * stride) as isize) as *const __m128i);
    let v6: __m128i = _mm_loadl_epi64(output.offset((6 * stride) as isize) as *const __m128i);
    let v7: __m128i = _mm_loadl_epi64(output.offset((7 * stride) as isize) as *const __m128i);
    v_256[0] = _mm256_inserti128_si256::<1>(_mm256_castsi128_si256(v0), v1);
    v_256[1] = _mm256_inserti128_si256::<1>(_mm256_castsi128_si256(v2), v3);
    v_256[2] = _mm256_inserti128_si256::<1>(_mm256_castsi128_si256(v4), v5);
    v_256[3] = _mm256_inserti128_si256::<1>(_mm256_castsi128_si256(v6), v7);
    let unpcklo0: __m256i = _mm256_unpacklo_epi8(v_256[0], zero);
    let unpcklo1: __m256i = _mm256_unpacklo_epi8(v_256[1], zero);
    let unpcklo2: __m256i = _mm256_unpacklo_epi8(v_256[2], zero);
    let unpcklo3: __m256i = _mm256_unpacklo_epi8(v_256[3], zero);
    // 00 01 10 11
    let x0: __m256i = _mm256_adds_epi16(in_256[0], unpcklo0);
    // 20 21 30 31
    let x1: __m256i = _mm256_adds_epi16(in_256[1], unpcklo1);
    // 40 41 50 51
    let x2: __m256i = _mm256_adds_epi16(in_256[2], unpcklo2);
    // 60 61 70 71
    let x3: __m256i = _mm256_adds_epi16(in_256[3], unpcklo3);
    // 00 01 20 21 10 11 30 31
    let res_0123: __m256i = _mm256_packus_epi16(x0, x1);
    // 40 41 60 61 50 51 70 71
    let res_4567: __m256i = _mm256_packus_epi16(x2, x3);
    // 00 01 20 21
    let res_02: __m128i = _mm256_castsi256_si128(res_0123);
    // 10 11 30 31
    let res_13: __m128i = _mm256_extracti128_si256::<1>(res_0123);
    // 40 41 60 61
    let res_46: __m128i = _mm256_castsi256_si128(res_4567);
    // 50 51 70 71
    let res_57: __m128i = _mm256_extracti128_si256::<1>(res_4567);
    // 00 01
    _mm_storel_epi64(output as *mut __m128i, res_02);
    // 10 11
    _mm_storel_epi64(output.offset(stride as isize) as *mut __m128i, res_13);
    // 20 21
    _mm_storel_epi64(
        output.offset((2 * stride) as isize) as *mut __m128i,
        _mm_unpackhi_epi64(res_02, res_02),
    );
    // 30 31
    _mm_storel_epi64(
        output.offset((3 * stride) as isize) as *mut __m128i,
        _mm_unpackhi_epi64(res_13, res_13),
    );
    // 40 41
    _mm_storel_epi64(output.offset((4 * stride) as isize) as *mut __m128i, res_46);
    // 50 51
    _mm_storel_epi64(output.offset((5 * stride) as isize) as *mut __m128i, res_57);
    // 60 61
    _mm_storel_epi64(
        output.offset((6 * stride) as isize) as *mut __m128i,
        _mm_unpackhi_epi64(res_46, res_46),
    );
    // 70 71
    _mm_storel_epi64(
        output.offset((7 * stride) as isize) as *mut __m128i,
        _mm_unpackhi_epi64(res_57, res_57),
    );
}

// AVX2 implementation has the advantage when combined multiple operations
// together.
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn lowbd_inv_txfm2d_8x8_no_identity_avx2(
    input: *const i32,
    output: *mut u8,
    stride: i32,
    tx_type: u8,
    tx_size: u8,
    eob: i32,
) {
    let mut buf1: [__m128i; 8] = core::mem::zeroed();
    let input_stride: i32 = 8;
    let mut shift: *const i8 = av1_inv_txfm_shift_ls[tx_size as usize];
    debug_assert!((hitx_1d_tab[tx_type as usize] as i32) < 2);
    debug_assert!((vitx_1d_tab[tx_type as usize] as i32) < 2);
    let row_txfm: transform_1d_ssse3 = lowbd_txfm_all_1d_zeros_8x8_arr
        [hitx_1d_tab[tx_type as usize] as usize][(eob != 1) as i32 as usize];
    let col_txfm: transform_1d_ssse3 = lowbd_txfm_all_1d_zeros_8x8_arr
        [vitx_1d_tab[tx_type as usize] as usize][(eob != 1) as i32 as usize];
    debug_assert!(!col_txfm.is_none());
    debug_assert!(!row_txfm.is_none());
    let mut ud_flip: i32 = 0;
    let mut lr_flip: i32 = 0;
    get_flip_cfg(tx_type, &raw mut ud_flip, &raw mut lr_flip);
    let mut buf0: [__m128i; 8] = core::mem::zeroed();
    let mut buf0_cur: *mut __m128i = buf0.as_mut_ptr();
    load_buffer_avx2(input, input_stride, buf0_cur);
    row_txfm.unwrap()(buf0.as_mut_ptr() as *const __m128i, buf0.as_mut_ptr());
    debug_assert!((*shift as i32) < 0);
    let mut _buf1: *mut __m128i = buf1.as_mut_ptr();
    round_and_transpose_avx2(
        buf0.as_mut_ptr() as *const __m128i,
        _buf1,
        *shift as i32,
        &raw mut lr_flip,
    );
    debug_assert!((*shift.offset(1) as i32) < 0);
    col_txfm.unwrap()(buf1.as_mut_ptr() as *const __m128i, buf1.as_mut_ptr());
    round_shift_lowbd_write_buffer_avx2(
        buf1.as_mut_ptr(),
        *shift.offset(1) as i32,
        output,
        stride,
        ud_flip,
    );
}

// AVX2 implementation of 8x8 inverse transform. Observed that coding AVX2 for
// tx_type with identity in either of the direction has no advantage.
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn lowbd_inv_txfm2d_add_8x8_avx2(
    input: *const i32,
    output: *mut u8,
    stride: i32,
    tx_type: u8,
    tx_size: u8,
    eob: i32,
) {
    match tx_type as i32 {
        IDTX => {
            av1_lowbd_inv_txfm2d_add_idtx_ssse3(input, output, stride, tx_size);
        }
        V_DCT | V_ADST | V_FLIPADST => {
            av1_lowbd_inv_txfm2d_add_h_identity_ssse3(input, output, stride, tx_type, tx_size, eob);
        }
        H_DCT | H_ADST | H_FLIPADST => {
            av1_lowbd_inv_txfm2d_add_v_identity_ssse3(input, output, stride, tx_type, tx_size, eob);
        }
        _ => {
            lowbd_inv_txfm2d_8x8_no_identity_avx2(input, output, stride, tx_type, tx_size, eob);
        }
    }
}

// for 32x32,32x64,64x32,64x64,16x32,32x16,64x16,16x64
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn lowbd_inv_txfm2d_add_universe_avx2(
    input: *const i32,
    output: *mut u8,
    stride: i32,
    tx_type: u8,
    tx_size: u8,
    eob: i32,
) {
    let _ = eob;
    match tx_type as i32 {
        DCT_DCT | ADST_DCT | DCT_ADST | ADST_ADST | FLIPADST_DCT | DCT_FLIPADST
        | FLIPADST_FLIPADST | ADST_FLIPADST | FLIPADST_ADST => {
            // ADST in vertical, DCT in horizontal
            // DCT  in vertical, ADST in horizontal
            // ADST in both directions
            lowbd_inv_txfm2d_add_no_identity_avx2(input, output, stride, tx_type, tx_size, eob);
        }
        IDTX => {
            lowbd_inv_txfm2d_add_idtx_avx2(input, output, stride, tx_size, eob);
        }
        V_DCT | V_ADST | V_FLIPADST => {
            lowbd_inv_txfm2d_add_h_identity_avx2(input, output, stride, tx_type, tx_size, eob);
        }
        H_DCT | H_ADST | H_FLIPADST => {
            lowbd_inv_txfm2d_add_v_identity_avx2(input, output, stride, tx_type, tx_size, eob);
        }
        _ => {
            av1_lowbd_inv_txfm2d_add_ssse3(input, output, stride, tx_type, tx_size, eob);
        }
    }
}

#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn av1_lowbd_inv_txfm2d_add_avx2(
    input: *const i32,
    output: *mut u8,
    stride: i32,
    tx_type: u8,
    tx_size: u8,
    eob: i32,
) {
    match tx_size as i32 {
        TX_4X4 | TX_4X8 | TX_8X4 | TX_8X16 | TX_16X8 | TX_4X16 | TX_16X4 | TX_8X32 | TX_32X8 => {
            av1_lowbd_inv_txfm2d_add_ssse3(input, output, stride, tx_type, tx_size, eob);
        }
        TX_8X8 => {
            lowbd_inv_txfm2d_add_8x8_avx2(input, output, stride, tx_type, tx_size, eob);
        }
        _ => {
            debug_assert!((tx_size as i32) < TX_SIZES_ALL);
            lowbd_inv_txfm2d_add_universe_avx2(input, output, stride, tx_type, tx_size, eob);
        }
    }
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn av1_inv_txfm_add_avx2(
    dqcoeff: *const i32,
    dst: *mut u8,
    stride: i32,
    txfm_param: *const TxfmParam,
) {
    let tx_type: u8 = (*txfm_param).tx_type;
    if !((*txfm_param).lossless != 0) {
        av1_lowbd_inv_txfm2d_add_avx2(
            dqcoeff,
            dst,
            stride,
            tx_type,
            (*txfm_param).tx_size,
            (*txfm_param).eob,
        );
    } else {
        av1_inv_txfm_add_c(dqcoeff, dst, stride, txfm_param);
    }
}
