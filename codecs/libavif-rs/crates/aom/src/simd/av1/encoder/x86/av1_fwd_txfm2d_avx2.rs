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

// Translated from upstream/aom/av1/encoder/x86/av1_fwd_txfm_avx2.h and av1_fwd_txfm2d_avx2.c
// (ADR 0056, ADR 0058). Upstream compiles the .c file with -mavx2. The functions were
// transliterated with a typing-aware C-subset printer (not C2Rust) and
// reviewed against the C function by function: the same sequence of intrinsic
// and helper calls, constants and operators in order (ADR 0058).
// TRANSPOSE_8X8_AVX2() reads b0..b3 and writes c0..c3 of its caller; the Rust macro takes
// them as arguments, and its two call sites pass them by name.
//
// Translation notes:
// - Every function carries `avx2`, the flag upstream compiles the file with.
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
use crate::simd::aom_dsp__x86__txfm_common_avx2::*;
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
use crate::simd::av1__encoder__x86__av1_fwd_txfm_sse2::*;
use crate::simd::av1__encoder__x86__av1_txfm1d_sse4::*;
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

type transform_1d_avx2 = Option<unsafe fn(*const __m256i, *mut __m256i, i8)>;
type FwdTxfm2dFunc = Option<unsafe fn(*const i16, *mut i32, i32, u8, i32)>;

// TRANSPOSE_8X8_AVX2() of av1_fwd_txfm2d_avx2.c: the C macro reads b0..b3 and
// writes c0..c3 of its caller.
macro_rules! TRANSPOSE_8X8_AVX2 {
    ($b0:expr, $b1:expr, $b2:expr, $b3:expr, $c0:expr, $c1:expr, $c2:expr, $c3:expr) => {{
        // aa0:    00 10 01 11  02 12 03 13 | 40 50 41 51  42 52 43 53
        // aa1:    04 14 05 15  06 16 07 17 | 44 54 45 55  46 56 47 57
        // aa2:    20 30 21 31  22 32 23 33 | 60 70 61 71  62 72 63 73
        // aa3:    24 34 25 35  26 36 27 37 | 64 74 65 75  66 76 67 77
        let aa0: __m256i = _mm256_unpacklo_epi16($b0, $b1);
        let aa1: __m256i = _mm256_unpackhi_epi16($b0, $b1);
        let aa2: __m256i = _mm256_unpacklo_epi16($b2, $b3);
        let aa3: __m256i = _mm256_unpackhi_epi16($b2, $b3);
        // Unpack 32 bit elements resulting in:
        // bb0: 00 10 20 30  01 11 21 31 | 40 50 60 70  41 51 61 71
        // bb1: 02 12 22 32  03 13 23 33 | 42 52 62 72  43 53 63 73
        // bb2: 04 14 24 34  05 15 25 35 | 44 54 64 74  45 55 65 75
        // bb2: 06 16 26 36  07 17 27 37 | 46 56 66 76  47 57 67 77
        let bb0: __m256i = _mm256_unpacklo_epi32(aa0, aa2);
        let bb1: __m256i = _mm256_unpackhi_epi32(aa0, aa2);
        let bb2: __m256i = _mm256_unpacklo_epi32(aa1, aa3);
        let bb3: __m256i = _mm256_unpackhi_epi32(aa1, aa3);
        // bb0: 00 10 20 30 40 50 60 70| 01 11 21 31 41 51 61 71
        // bb1: 02 12 22 32 42 52 62 72| 03 13 23 33 43 53 63 73
        // bb2: 04 14 24 34 44 54 64 74| 05 15 25 35 45 55 65 75
        // bb2: 06 16 26 36 46 56 66 76| 07 17 27 37 47 57 67 77
        $c0 = _mm256_permute4x64_epi64::<0xd8>(bb0);
        $c1 = _mm256_permute4x64_epi64::<0xd8>(bb1);
        $c2 = _mm256_permute4x64_epi64::<0xd8>(bb2);
        $c3 = _mm256_permute4x64_epi64::<0xd8>(bb3);
    }};
}

// av1/encoder/x86/av1_fwd_txfm_avx2.h

// out0 = in0*w0 + in1*w1
// out1 = -in1*w0 + in0*w1
#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn btf_32_avx2_type0(
    w0: i32,
    w1: i32,
    in0: *mut __m256i,
    in1: *mut __m256i,
    _r: __m256i,
    cos_bit: i32,
) {
    let mut _in0: __m256i = *in0;
    let mut _in1: __m256i = *in1;
    let ww0: __m256i = _mm256_set1_epi32(w0);
    let ww1: __m256i = _mm256_set1_epi32(w1);
    let in0_w0: __m256i = _mm256_mullo_epi32(_in0, ww0);
    let in1_w1: __m256i = _mm256_mullo_epi32(_in1, ww1);
    let mut temp0: __m256i = _mm256_add_epi32(in0_w0, in1_w1);
    temp0 = _mm256_add_epi32(temp0, _r);
    *in0 = _mm256_sra_epi32(temp0, _mm_cvtsi32_si128(cos_bit));
    let in0_w1: __m256i = _mm256_mullo_epi32(_in0, ww1);
    let in1_w0: __m256i = _mm256_mullo_epi32(_in1, ww0);
    let mut temp1: __m256i = _mm256_sub_epi32(in0_w1, in1_w0);
    temp1 = _mm256_add_epi32(temp1, _r);
    *in1 = _mm256_sra_epi32(temp1, _mm_cvtsi32_si128(cos_bit));
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn btf_32_avx2_type1(
    w0: i32,
    w1: i32,
    in0: *mut __m256i,
    in1: *mut __m256i,
    _r: __m256i,
    cos_bit: i32,
) {
    let mut _in0: __m256i = *in0;
    let mut _in1: __m256i = *in1;
    let ww0: __m256i = _mm256_set1_epi32(w0);
    let ww1: __m256i = _mm256_set1_epi32(w1);
    let in0_w0: __m256i = _mm256_mullo_epi32(_in0, ww0);
    let in1_w1: __m256i = _mm256_mullo_epi32(_in1, ww1);
    let mut temp0: __m256i = _mm256_add_epi32(in0_w0, in1_w1);
    temp0 = _mm256_add_epi32(temp0, _r);
    *in0 = _mm256_sra_epi32(temp0, _mm_cvtsi32_si128(cos_bit));
    let in0_w1: __m256i = _mm256_mullo_epi32(_in0, ww1);
    let in1_w0: __m256i = _mm256_mullo_epi32(_in1, ww0);
    let mut temp1: __m256i = _mm256_sub_epi32(in1_w0, in0_w1);
    temp1 = _mm256_add_epi32(temp1, _r);
    *in1 = _mm256_sra_epi32(temp1, _mm_cvtsi32_si128(cos_bit));
}

// out0 = in0*w0 + in1*w1
// out1 = -in1*w0 + in0*w1
#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn btf_32_avx2_type0_new(
    ww0: __m256i,
    ww1: __m256i,
    in0: *mut __m256i,
    in1: *mut __m256i,
    _r: __m256i,
    cos_bit: i32,
) {
    let mut _in0: __m256i = *in0;
    let mut _in1: __m256i = *in1;
    let in0_w0: __m256i = _mm256_mullo_epi32(_in0, ww0);
    let in1_w1: __m256i = _mm256_mullo_epi32(_in1, ww1);
    let mut temp0: __m256i = _mm256_add_epi32(in0_w0, in1_w1);
    temp0 = _mm256_add_epi32(temp0, _r);
    *in0 = _mm256_sra_epi32(temp0, _mm_cvtsi32_si128(cos_bit));
    let in0_w1: __m256i = _mm256_mullo_epi32(_in0, ww1);
    let in1_w0: __m256i = _mm256_mullo_epi32(_in1, ww0);
    let mut temp1: __m256i = _mm256_sub_epi32(in0_w1, in1_w0);
    temp1 = _mm256_add_epi32(temp1, _r);
    *in1 = _mm256_sra_epi32(temp1, _mm_cvtsi32_si128(cos_bit));
}

// out0 = in0*w0 + in1*w1
// out1 = in1*w0 - in0*w1
#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn btf_32_avx2_type1_new(
    ww0: __m256i,
    ww1: __m256i,
    in0: *mut __m256i,
    in1: *mut __m256i,
    _r: __m256i,
    cos_bit: i32,
) {
    let mut _in0: __m256i = *in0;
    let mut _in1: __m256i = *in1;
    let in0_w0: __m256i = _mm256_mullo_epi32(_in0, ww0);
    let in1_w1: __m256i = _mm256_mullo_epi32(_in1, ww1);
    let mut temp0: __m256i = _mm256_add_epi32(in0_w0, in1_w1);
    temp0 = _mm256_add_epi32(temp0, _r);
    *in0 = _mm256_sra_epi32(temp0, _mm_cvtsi32_si128(cos_bit));
    let in0_w1: __m256i = _mm256_mullo_epi32(_in0, ww1);
    let in1_w0: __m256i = _mm256_mullo_epi32(_in1, ww0);
    let mut temp1: __m256i = _mm256_sub_epi32(in1_w0, in0_w1);
    temp1 = _mm256_add_epi32(temp1, _r);
    *in1 = _mm256_sra_epi32(temp1, _mm_cvtsi32_si128(cos_bit));
}

// AOM_AV1_ENCODER_X86_AV1_FWD_TXFM_AVX2_H_

// ---------------------------------------------------------------------------
// av1/encoder/x86/av1_fwd_txfm2d_avx2.c

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn fdct16x16_new_avx2(input: *const __m256i, output: *mut __m256i, cos_bit: i8) {
    let mut cospi: *const i32 = cospi_arr(cos_bit as i32);
    let _r: __m256i = _mm256_set1_epi32(1 << (cos_bit as i32 - 1));
    let mut cospi_m32_p32: __m256i =
        pair_set_w16_epi16(-*cospi.offset(32) as i16, *cospi.offset(32) as i16);
    let mut cospi_p32_p32: __m256i =
        pair_set_w16_epi16(*cospi.offset(32) as i16, *cospi.offset(32) as i16);
    let mut cospi_p32_m32: __m256i =
        pair_set_w16_epi16(*cospi.offset(32) as i16, -*cospi.offset(32) as i16);
    let mut cospi_p48_p16: __m256i =
        pair_set_w16_epi16(*cospi.offset(48) as i16, *cospi.offset(16) as i16);
    let mut cospi_m16_p48: __m256i =
        pair_set_w16_epi16(-*cospi.offset(16) as i16, *cospi.offset(48) as i16);
    let mut cospi_m48_m16: __m256i =
        pair_set_w16_epi16(-*cospi.offset(48) as i16, -*cospi.offset(16) as i16);
    let mut cospi_p56_p08: __m256i =
        pair_set_w16_epi16(*cospi.offset(56) as i16, *cospi.offset(8) as i16);
    let mut cospi_m08_p56: __m256i =
        pair_set_w16_epi16(-*cospi.offset(8) as i16, *cospi.offset(56) as i16);
    let mut cospi_p24_p40: __m256i =
        pair_set_w16_epi16(*cospi.offset(24) as i16, *cospi.offset(40) as i16);
    let mut cospi_m40_p24: __m256i =
        pair_set_w16_epi16(-*cospi.offset(40) as i16, *cospi.offset(24) as i16);
    let mut cospi_p60_p04: __m256i =
        pair_set_w16_epi16(*cospi.offset(60) as i16, *cospi.offset(4) as i16);
    let mut cospi_m04_p60: __m256i =
        pair_set_w16_epi16(-*cospi.offset(4) as i16, *cospi.offset(60) as i16);
    let mut cospi_p28_p36: __m256i =
        pair_set_w16_epi16(*cospi.offset(28) as i16, *cospi.offset(36) as i16);
    let mut cospi_m36_p28: __m256i =
        pair_set_w16_epi16(-*cospi.offset(36) as i16, *cospi.offset(28) as i16);
    let mut cospi_p44_p20: __m256i =
        pair_set_w16_epi16(*cospi.offset(44) as i16, *cospi.offset(20) as i16);
    let mut cospi_m20_p44: __m256i =
        pair_set_w16_epi16(-*cospi.offset(20) as i16, *cospi.offset(44) as i16);
    let mut cospi_p12_p52: __m256i =
        pair_set_w16_epi16(*cospi.offset(12) as i16, *cospi.offset(52) as i16);
    let mut cospi_m52_p12: __m256i =
        pair_set_w16_epi16(-*cospi.offset(52) as i16, *cospi.offset(12) as i16);
    // stage 1
    let mut x1: [__m256i; 16] = core::mem::zeroed();
    btf_16_adds_subs_out_avx2(
        x1.as_mut_ptr(),
        x1.as_mut_ptr().offset(15),
        *input,
        *input.offset(15),
    );
    btf_16_adds_subs_out_avx2(
        x1.as_mut_ptr().offset(1),
        x1.as_mut_ptr().offset(14),
        *input.offset(1),
        *input.offset(14),
    );
    btf_16_adds_subs_out_avx2(
        x1.as_mut_ptr().offset(2),
        x1.as_mut_ptr().offset(13),
        *input.offset(2),
        *input.offset(13),
    );
    btf_16_adds_subs_out_avx2(
        x1.as_mut_ptr().offset(3),
        x1.as_mut_ptr().offset(12),
        *input.offset(3),
        *input.offset(12),
    );
    btf_16_adds_subs_out_avx2(
        x1.as_mut_ptr().offset(4),
        x1.as_mut_ptr().offset(11),
        *input.offset(4),
        *input.offset(11),
    );
    btf_16_adds_subs_out_avx2(
        x1.as_mut_ptr().offset(5),
        x1.as_mut_ptr().offset(10),
        *input.offset(5),
        *input.offset(10),
    );
    btf_16_adds_subs_out_avx2(
        x1.as_mut_ptr().offset(6),
        x1.as_mut_ptr().offset(9),
        *input.offset(6),
        *input.offset(9),
    );
    btf_16_adds_subs_out_avx2(
        x1.as_mut_ptr().offset(7),
        x1.as_mut_ptr().offset(8),
        *input.offset(7),
        *input.offset(8),
    );
    // stage 2
    btf_16_adds_subs_avx2(x1.as_mut_ptr(), x1.as_mut_ptr().offset(7));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(1), x1.as_mut_ptr().offset(6));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(2), x1.as_mut_ptr().offset(5));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(3), x1.as_mut_ptr().offset(4));
    btf_16_w16_avx2(
        cospi_m32_p32,
        cospi_p32_p32,
        x1.as_mut_ptr().offset(10),
        x1.as_mut_ptr().offset(13),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m32_p32,
        cospi_p32_p32,
        x1.as_mut_ptr().offset(11),
        x1.as_mut_ptr().offset(12),
        _r,
        cos_bit as i32,
    );
    // stage 3
    btf_16_adds_subs_avx2(x1.as_mut_ptr(), x1.as_mut_ptr().offset(3));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(1), x1.as_mut_ptr().offset(2));
    btf_16_w16_avx2(
        cospi_m32_p32,
        cospi_p32_p32,
        x1.as_mut_ptr().offset(5),
        x1.as_mut_ptr().offset(6),
        _r,
        cos_bit as i32,
    );
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(8), x1.as_mut_ptr().offset(11));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(9), x1.as_mut_ptr().offset(10));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(15), x1.as_mut_ptr().offset(12));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(14), x1.as_mut_ptr().offset(13));
    // stage 4
    btf_16_w16_avx2(
        cospi_p32_p32,
        cospi_p32_m32,
        x1.as_mut_ptr(),
        x1.as_mut_ptr().offset(1),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_p48_p16,
        cospi_m16_p48,
        x1.as_mut_ptr().offset(2),
        x1.as_mut_ptr().offset(3),
        _r,
        cos_bit as i32,
    );
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(4), x1.as_mut_ptr().offset(5));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(7), x1.as_mut_ptr().offset(6));
    btf_16_w16_avx2(
        cospi_m16_p48,
        cospi_p48_p16,
        x1.as_mut_ptr().offset(9),
        x1.as_mut_ptr().offset(14),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m48_m16,
        cospi_m16_p48,
        x1.as_mut_ptr().offset(10),
        x1.as_mut_ptr().offset(13),
        _r,
        cos_bit as i32,
    );
    // stage 5
    btf_16_w16_avx2(
        cospi_p56_p08,
        cospi_m08_p56,
        x1.as_mut_ptr().offset(4),
        x1.as_mut_ptr().offset(7),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_p24_p40,
        cospi_m40_p24,
        x1.as_mut_ptr().offset(5),
        x1.as_mut_ptr().offset(6),
        _r,
        cos_bit as i32,
    );
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(8), x1.as_mut_ptr().offset(9));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(11), x1.as_mut_ptr().offset(10));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(12), x1.as_mut_ptr().offset(13));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(15), x1.as_mut_ptr().offset(14));
    // stage 6
    btf_16_w16_avx2(
        cospi_p60_p04,
        cospi_m04_p60,
        x1.as_mut_ptr().offset(8),
        x1.as_mut_ptr().offset(15),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_p28_p36,
        cospi_m36_p28,
        x1.as_mut_ptr().offset(9),
        x1.as_mut_ptr().offset(14),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_p44_p20,
        cospi_m20_p44,
        x1.as_mut_ptr().offset(10),
        x1.as_mut_ptr().offset(13),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_p12_p52,
        cospi_m52_p12,
        x1.as_mut_ptr().offset(11),
        x1.as_mut_ptr().offset(12),
        _r,
        cos_bit as i32,
    );
    // stage 7
    *output = x1[0];
    *output.offset(1) = x1[8];
    *output.offset(2) = x1[4];
    *output.offset(3) = x1[12];
    *output.offset(4) = x1[2];
    *output.offset(5) = x1[10];
    *output.offset(6) = x1[6];
    *output.offset(7) = x1[14];
    *output.offset(8) = x1[1];
    *output.offset(9) = x1[9];
    *output.offset(10) = x1[5];
    *output.offset(11) = x1[13];
    *output.offset(12) = x1[3];
    *output.offset(13) = x1[11];
    *output.offset(14) = x1[7];
    *output.offset(15) = x1[15];
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn fdct16x32_avx2(input: *const __m256i, output: *mut __m256i, cos_bit: i8) {
    let mut cospi: *const i32 = cospi_arr(cos_bit as i32);
    let _r: __m256i = _mm256_set1_epi32(1 << (cos_bit as i32 - 1));
    let mut cospi_m32_p32: __m256i =
        pair_set_w16_epi16(-*cospi.offset(32) as i16, *cospi.offset(32) as i16);
    let mut cospi_p32_p32: __m256i =
        pair_set_w16_epi16(*cospi.offset(32) as i16, *cospi.offset(32) as i16);
    let mut cospi_m16_p48: __m256i =
        pair_set_w16_epi16(-*cospi.offset(16) as i16, *cospi.offset(48) as i16);
    let mut cospi_p48_p16: __m256i =
        pair_set_w16_epi16(*cospi.offset(48) as i16, *cospi.offset(16) as i16);
    let mut cospi_m48_m16: __m256i =
        pair_set_w16_epi16(-*cospi.offset(48) as i16, -*cospi.offset(16) as i16);
    let mut cospi_p32_m32: __m256i =
        pair_set_w16_epi16(*cospi.offset(32) as i16, -*cospi.offset(32) as i16);
    let mut cospi_p56_p08: __m256i =
        pair_set_w16_epi16(*cospi.offset(56) as i16, *cospi.offset(8) as i16);
    let mut cospi_m08_p56: __m256i =
        pair_set_w16_epi16(-*cospi.offset(8) as i16, *cospi.offset(56) as i16);
    let mut cospi_p24_p40: __m256i =
        pair_set_w16_epi16(*cospi.offset(24) as i16, *cospi.offset(40) as i16);
    let mut cospi_m40_p24: __m256i =
        pair_set_w16_epi16(-*cospi.offset(40) as i16, *cospi.offset(24) as i16);
    let mut cospi_m56_m08: __m256i =
        pair_set_w16_epi16(-*cospi.offset(56) as i16, -*cospi.offset(8) as i16);
    let mut cospi_m24_m40: __m256i =
        pair_set_w16_epi16(-*cospi.offset(24) as i16, -*cospi.offset(40) as i16);
    let mut cospi_p60_p04: __m256i =
        pair_set_w16_epi16(*cospi.offset(60) as i16, *cospi.offset(4) as i16);
    let mut cospi_m04_p60: __m256i =
        pair_set_w16_epi16(-*cospi.offset(4) as i16, *cospi.offset(60) as i16);
    let mut cospi_p28_p36: __m256i =
        pair_set_w16_epi16(*cospi.offset(28) as i16, *cospi.offset(36) as i16);
    let mut cospi_m36_p28: __m256i =
        pair_set_w16_epi16(-*cospi.offset(36) as i16, *cospi.offset(28) as i16);
    let mut cospi_p44_p20: __m256i =
        pair_set_w16_epi16(*cospi.offset(44) as i16, *cospi.offset(20) as i16);
    let mut cospi_m20_p44: __m256i =
        pair_set_w16_epi16(-*cospi.offset(20) as i16, *cospi.offset(44) as i16);
    let mut cospi_p12_p52: __m256i =
        pair_set_w16_epi16(*cospi.offset(12) as i16, *cospi.offset(52) as i16);
    let mut cospi_m52_p12: __m256i =
        pair_set_w16_epi16(-*cospi.offset(52) as i16, *cospi.offset(12) as i16);
    let mut cospi_p62_p02: __m256i =
        pair_set_w16_epi16(*cospi.offset(62) as i16, *cospi.offset(2) as i16);
    let mut cospi_m02_p62: __m256i =
        pair_set_w16_epi16(-*cospi.offset(2) as i16, *cospi.offset(62) as i16);
    let mut cospi_p30_p34: __m256i =
        pair_set_w16_epi16(*cospi.offset(30) as i16, *cospi.offset(34) as i16);
    let mut cospi_m34_p30: __m256i =
        pair_set_w16_epi16(-*cospi.offset(34) as i16, *cospi.offset(30) as i16);
    let mut cospi_p46_p18: __m256i =
        pair_set_w16_epi16(*cospi.offset(46) as i16, *cospi.offset(18) as i16);
    let mut cospi_m18_p46: __m256i =
        pair_set_w16_epi16(-*cospi.offset(18) as i16, *cospi.offset(46) as i16);
    let mut cospi_p14_p50: __m256i =
        pair_set_w16_epi16(*cospi.offset(14) as i16, *cospi.offset(50) as i16);
    let mut cospi_m50_p14: __m256i =
        pair_set_w16_epi16(-*cospi.offset(50) as i16, *cospi.offset(14) as i16);
    let mut cospi_p54_p10: __m256i =
        pair_set_w16_epi16(*cospi.offset(54) as i16, *cospi.offset(10) as i16);
    let mut cospi_m10_p54: __m256i =
        pair_set_w16_epi16(-*cospi.offset(10) as i16, *cospi.offset(54) as i16);
    let mut cospi_p22_p42: __m256i =
        pair_set_w16_epi16(*cospi.offset(22) as i16, *cospi.offset(42) as i16);
    let mut cospi_m42_p22: __m256i =
        pair_set_w16_epi16(-*cospi.offset(42) as i16, *cospi.offset(22) as i16);
    let mut cospi_p38_p26: __m256i =
        pair_set_w16_epi16(*cospi.offset(38) as i16, *cospi.offset(26) as i16);
    let mut cospi_m26_p38: __m256i =
        pair_set_w16_epi16(-*cospi.offset(26) as i16, *cospi.offset(38) as i16);
    let mut cospi_p06_p58: __m256i =
        pair_set_w16_epi16(*cospi.offset(6) as i16, *cospi.offset(58) as i16);
    let mut cospi_m58_p06: __m256i =
        pair_set_w16_epi16(-*cospi.offset(58) as i16, *cospi.offset(6) as i16);
    // stage 1
    let mut x1: [__m256i; 32] = core::mem::zeroed();
    btf_16_adds_subs_out_avx2(
        x1.as_mut_ptr(),
        x1.as_mut_ptr().offset(31),
        *input,
        *input.offset(31),
    );
    btf_16_adds_subs_out_avx2(
        x1.as_mut_ptr().offset(1),
        x1.as_mut_ptr().offset(30),
        *input.offset(1),
        *input.offset(30),
    );
    btf_16_adds_subs_out_avx2(
        x1.as_mut_ptr().offset(2),
        x1.as_mut_ptr().offset(29),
        *input.offset(2),
        *input.offset(29),
    );
    btf_16_adds_subs_out_avx2(
        x1.as_mut_ptr().offset(3),
        x1.as_mut_ptr().offset(28),
        *input.offset(3),
        *input.offset(28),
    );
    btf_16_adds_subs_out_avx2(
        x1.as_mut_ptr().offset(4),
        x1.as_mut_ptr().offset(27),
        *input.offset(4),
        *input.offset(27),
    );
    btf_16_adds_subs_out_avx2(
        x1.as_mut_ptr().offset(5),
        x1.as_mut_ptr().offset(26),
        *input.offset(5),
        *input.offset(26),
    );
    btf_16_adds_subs_out_avx2(
        x1.as_mut_ptr().offset(6),
        x1.as_mut_ptr().offset(25),
        *input.offset(6),
        *input.offset(25),
    );
    btf_16_adds_subs_out_avx2(
        x1.as_mut_ptr().offset(7),
        x1.as_mut_ptr().offset(24),
        *input.offset(7),
        *input.offset(24),
    );
    btf_16_adds_subs_out_avx2(
        x1.as_mut_ptr().offset(8),
        x1.as_mut_ptr().offset(23),
        *input.offset(8),
        *input.offset(23),
    );
    btf_16_adds_subs_out_avx2(
        x1.as_mut_ptr().offset(9),
        x1.as_mut_ptr().offset(22),
        *input.offset(9),
        *input.offset(22),
    );
    btf_16_adds_subs_out_avx2(
        x1.as_mut_ptr().offset(10),
        x1.as_mut_ptr().offset(21),
        *input.offset(10),
        *input.offset(21),
    );
    btf_16_adds_subs_out_avx2(
        x1.as_mut_ptr().offset(11),
        x1.as_mut_ptr().offset(20),
        *input.offset(11),
        *input.offset(20),
    );
    btf_16_adds_subs_out_avx2(
        x1.as_mut_ptr().offset(12),
        x1.as_mut_ptr().offset(19),
        *input.offset(12),
        *input.offset(19),
    );
    btf_16_adds_subs_out_avx2(
        x1.as_mut_ptr().offset(13),
        x1.as_mut_ptr().offset(18),
        *input.offset(13),
        *input.offset(18),
    );
    btf_16_adds_subs_out_avx2(
        x1.as_mut_ptr().offset(14),
        x1.as_mut_ptr().offset(17),
        *input.offset(14),
        *input.offset(17),
    );
    btf_16_adds_subs_out_avx2(
        x1.as_mut_ptr().offset(15),
        x1.as_mut_ptr().offset(16),
        *input.offset(15),
        *input.offset(16),
    );
    // stage 2
    btf_16_adds_subs_avx2(x1.as_mut_ptr(), x1.as_mut_ptr().offset(15));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(1), x1.as_mut_ptr().offset(14));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(2), x1.as_mut_ptr().offset(13));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(3), x1.as_mut_ptr().offset(12));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(4), x1.as_mut_ptr().offset(11));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(5), x1.as_mut_ptr().offset(10));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(6), x1.as_mut_ptr().offset(9));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(7), x1.as_mut_ptr().offset(8));
    btf_16_w16_avx2(
        cospi_m32_p32,
        cospi_p32_p32,
        x1.as_mut_ptr().offset(20),
        x1.as_mut_ptr().offset(27),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m32_p32,
        cospi_p32_p32,
        x1.as_mut_ptr().offset(21),
        x1.as_mut_ptr().offset(26),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m32_p32,
        cospi_p32_p32,
        x1.as_mut_ptr().offset(22),
        x1.as_mut_ptr().offset(25),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m32_p32,
        cospi_p32_p32,
        x1.as_mut_ptr().offset(23),
        x1.as_mut_ptr().offset(24),
        _r,
        cos_bit as i32,
    );
    // stage 3
    btf_16_adds_subs_avx2(x1.as_mut_ptr(), x1.as_mut_ptr().offset(7));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(1), x1.as_mut_ptr().offset(6));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(2), x1.as_mut_ptr().offset(5));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(3), x1.as_mut_ptr().offset(4));
    btf_16_w16_avx2(
        cospi_m32_p32,
        cospi_p32_p32,
        x1.as_mut_ptr().offset(10),
        x1.as_mut_ptr().offset(13),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m32_p32,
        cospi_p32_p32,
        x1.as_mut_ptr().offset(11),
        x1.as_mut_ptr().offset(12),
        _r,
        cos_bit as i32,
    );
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(16), x1.as_mut_ptr().offset(23));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(17), x1.as_mut_ptr().offset(22));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(18), x1.as_mut_ptr().offset(21));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(19), x1.as_mut_ptr().offset(20));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(31), x1.as_mut_ptr().offset(24));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(30), x1.as_mut_ptr().offset(25));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(29), x1.as_mut_ptr().offset(26));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(28), x1.as_mut_ptr().offset(27));
    // stage 4
    btf_16_adds_subs_avx2(x1.as_mut_ptr(), x1.as_mut_ptr().offset(3));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(1), x1.as_mut_ptr().offset(2));
    btf_16_w16_avx2(
        cospi_m32_p32,
        cospi_p32_p32,
        x1.as_mut_ptr().offset(5),
        x1.as_mut_ptr().offset(6),
        _r,
        cos_bit as i32,
    );
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(8), x1.as_mut_ptr().offset(11));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(9), x1.as_mut_ptr().offset(10));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(15), x1.as_mut_ptr().offset(12));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(14), x1.as_mut_ptr().offset(13));
    btf_16_w16_avx2(
        cospi_m16_p48,
        cospi_p48_p16,
        x1.as_mut_ptr().offset(18),
        x1.as_mut_ptr().offset(29),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m16_p48,
        cospi_p48_p16,
        x1.as_mut_ptr().offset(19),
        x1.as_mut_ptr().offset(28),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m48_m16,
        cospi_m16_p48,
        x1.as_mut_ptr().offset(20),
        x1.as_mut_ptr().offset(27),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m48_m16,
        cospi_m16_p48,
        x1.as_mut_ptr().offset(21),
        x1.as_mut_ptr().offset(26),
        _r,
        cos_bit as i32,
    );
    // stage 5
    btf_16_w16_avx2(
        cospi_p32_p32,
        cospi_p32_m32,
        x1.as_mut_ptr(),
        x1.as_mut_ptr().offset(1),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_p48_p16,
        cospi_m16_p48,
        x1.as_mut_ptr().offset(2),
        x1.as_mut_ptr().offset(3),
        _r,
        cos_bit as i32,
    );
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(4), x1.as_mut_ptr().offset(5));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(7), x1.as_mut_ptr().offset(6));
    btf_16_w16_avx2(
        cospi_m16_p48,
        cospi_p48_p16,
        x1.as_mut_ptr().offset(9),
        x1.as_mut_ptr().offset(14),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m48_m16,
        cospi_m16_p48,
        x1.as_mut_ptr().offset(10),
        x1.as_mut_ptr().offset(13),
        _r,
        cos_bit as i32,
    );
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(16), x1.as_mut_ptr().offset(19));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(17), x1.as_mut_ptr().offset(18));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(23), x1.as_mut_ptr().offset(20));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(22), x1.as_mut_ptr().offset(21));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(24), x1.as_mut_ptr().offset(27));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(25), x1.as_mut_ptr().offset(26));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(31), x1.as_mut_ptr().offset(28));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(30), x1.as_mut_ptr().offset(29));
    // stage 6
    btf_16_w16_avx2(
        cospi_p56_p08,
        cospi_m08_p56,
        x1.as_mut_ptr().offset(4),
        x1.as_mut_ptr().offset(7),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_p24_p40,
        cospi_m40_p24,
        x1.as_mut_ptr().offset(5),
        x1.as_mut_ptr().offset(6),
        _r,
        cos_bit as i32,
    );
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(8), x1.as_mut_ptr().offset(9));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(11), x1.as_mut_ptr().offset(10));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(12), x1.as_mut_ptr().offset(13));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(15), x1.as_mut_ptr().offset(14));
    btf_16_w16_avx2(
        cospi_m08_p56,
        cospi_p56_p08,
        x1.as_mut_ptr().offset(17),
        x1.as_mut_ptr().offset(30),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m56_m08,
        cospi_m08_p56,
        x1.as_mut_ptr().offset(18),
        x1.as_mut_ptr().offset(29),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m40_p24,
        cospi_p24_p40,
        x1.as_mut_ptr().offset(21),
        x1.as_mut_ptr().offset(26),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m24_m40,
        cospi_m40_p24,
        x1.as_mut_ptr().offset(22),
        x1.as_mut_ptr().offset(25),
        _r,
        cos_bit as i32,
    );
    // stage 7
    btf_16_w16_avx2(
        cospi_p60_p04,
        cospi_m04_p60,
        x1.as_mut_ptr().offset(8),
        x1.as_mut_ptr().offset(15),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_p28_p36,
        cospi_m36_p28,
        x1.as_mut_ptr().offset(9),
        x1.as_mut_ptr().offset(14),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_p44_p20,
        cospi_m20_p44,
        x1.as_mut_ptr().offset(10),
        x1.as_mut_ptr().offset(13),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_p12_p52,
        cospi_m52_p12,
        x1.as_mut_ptr().offset(11),
        x1.as_mut_ptr().offset(12),
        _r,
        cos_bit as i32,
    );
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(16), x1.as_mut_ptr().offset(17));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(19), x1.as_mut_ptr().offset(18));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(20), x1.as_mut_ptr().offset(21));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(23), x1.as_mut_ptr().offset(22));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(24), x1.as_mut_ptr().offset(25));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(27), x1.as_mut_ptr().offset(26));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(28), x1.as_mut_ptr().offset(29));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(31), x1.as_mut_ptr().offset(30));
    // stage 8
    btf_16_w16_avx2(
        cospi_p62_p02,
        cospi_m02_p62,
        x1.as_mut_ptr().offset(16),
        x1.as_mut_ptr().offset(31),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_p30_p34,
        cospi_m34_p30,
        x1.as_mut_ptr().offset(17),
        x1.as_mut_ptr().offset(30),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_p46_p18,
        cospi_m18_p46,
        x1.as_mut_ptr().offset(18),
        x1.as_mut_ptr().offset(29),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_p14_p50,
        cospi_m50_p14,
        x1.as_mut_ptr().offset(19),
        x1.as_mut_ptr().offset(28),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_p54_p10,
        cospi_m10_p54,
        x1.as_mut_ptr().offset(20),
        x1.as_mut_ptr().offset(27),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_p22_p42,
        cospi_m42_p22,
        x1.as_mut_ptr().offset(21),
        x1.as_mut_ptr().offset(26),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_p38_p26,
        cospi_m26_p38,
        x1.as_mut_ptr().offset(22),
        x1.as_mut_ptr().offset(25),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_p06_p58,
        cospi_m58_p06,
        x1.as_mut_ptr().offset(23),
        x1.as_mut_ptr().offset(24),
        _r,
        cos_bit as i32,
    );
    // stage 9
    *output = x1[0];
    *output.offset(1) = x1[16];
    *output.offset(2) = x1[8];
    *output.offset(3) = x1[24];
    *output.offset(4) = x1[4];
    *output.offset(5) = x1[20];
    *output.offset(6) = x1[12];
    *output.offset(7) = x1[28];
    *output.offset(8) = x1[2];
    *output.offset(9) = x1[18];
    *output.offset(10) = x1[10];
    *output.offset(11) = x1[26];
    *output.offset(12) = x1[6];
    *output.offset(13) = x1[22];
    *output.offset(14) = x1[14];
    *output.offset(15) = x1[30];
    *output.offset(16) = x1[1];
    *output.offset(17) = x1[17];
    *output.offset(18) = x1[9];
    *output.offset(19) = x1[25];
    *output.offset(20) = x1[5];
    *output.offset(21) = x1[21];
    *output.offset(22) = x1[13];
    *output.offset(23) = x1[29];
    *output.offset(24) = x1[3];
    *output.offset(25) = x1[19];
    *output.offset(26) = x1[11];
    *output.offset(27) = x1[27];
    *output.offset(28) = x1[7];
    *output.offset(29) = x1[23];
    *output.offset(30) = x1[15];
    *output.offset(31) = x1[31];
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn fdct16x64_new_avx2(input: *const __m256i, output: *mut __m256i, cos_bit: i8) {
    let mut cospi: *const i32 = cospi_arr(cos_bit as i32);
    let _r: __m256i = _mm256_set1_epi32(1 << (cos_bit as i32 - 1));
    let mut cospi_m32_p32: __m256i =
        pair_set_w16_epi16(-*cospi.offset(32) as i16, *cospi.offset(32) as i16);
    let mut cospi_p32_p32: __m256i =
        pair_set_w16_epi16(*cospi.offset(32) as i16, *cospi.offset(32) as i16);
    let mut cospi_m16_p48: __m256i =
        pair_set_w16_epi16(-*cospi.offset(16) as i16, *cospi.offset(48) as i16);
    let mut cospi_p48_p16: __m256i =
        pair_set_w16_epi16(*cospi.offset(48) as i16, *cospi.offset(16) as i16);
    let mut cospi_m48_m16: __m256i =
        pair_set_w16_epi16(-*cospi.offset(48) as i16, -*cospi.offset(16) as i16);
    let mut cospi_p32_m32: __m256i =
        pair_set_w16_epi16(*cospi.offset(32) as i16, -*cospi.offset(32) as i16);
    let mut cospi_m08_p56: __m256i =
        pair_set_w16_epi16(-*cospi.offset(8) as i16, *cospi.offset(56) as i16);
    let mut cospi_p56_p08: __m256i =
        pair_set_w16_epi16(*cospi.offset(56) as i16, *cospi.offset(8) as i16);
    let mut cospi_m56_m08: __m256i =
        pair_set_w16_epi16(-*cospi.offset(56) as i16, -*cospi.offset(8) as i16);
    let mut cospi_m40_p24: __m256i =
        pair_set_w16_epi16(-*cospi.offset(40) as i16, *cospi.offset(24) as i16);
    let mut cospi_p24_p40: __m256i =
        pair_set_w16_epi16(*cospi.offset(24) as i16, *cospi.offset(40) as i16);
    let mut cospi_m24_m40: __m256i =
        pair_set_w16_epi16(-*cospi.offset(24) as i16, -*cospi.offset(40) as i16);
    let mut cospi_p60_p04: __m256i =
        pair_set_w16_epi16(*cospi.offset(60) as i16, *cospi.offset(4) as i16);
    let mut cospi_m04_p60: __m256i =
        pair_set_w16_epi16(-*cospi.offset(4) as i16, *cospi.offset(60) as i16);
    let mut cospi_p28_p36: __m256i =
        pair_set_w16_epi16(*cospi.offset(28) as i16, *cospi.offset(36) as i16);
    let mut cospi_m36_p28: __m256i =
        pair_set_w16_epi16(-*cospi.offset(36) as i16, *cospi.offset(28) as i16);
    let mut cospi_p44_p20: __m256i =
        pair_set_w16_epi16(*cospi.offset(44) as i16, *cospi.offset(20) as i16);
    let mut cospi_m20_p44: __m256i =
        pair_set_w16_epi16(-*cospi.offset(20) as i16, *cospi.offset(44) as i16);
    let mut cospi_p12_p52: __m256i =
        pair_set_w16_epi16(*cospi.offset(12) as i16, *cospi.offset(52) as i16);
    let mut cospi_m52_p12: __m256i =
        pair_set_w16_epi16(-*cospi.offset(52) as i16, *cospi.offset(12) as i16);
    let mut cospi_m60_m04: __m256i =
        pair_set_w16_epi16(-*cospi.offset(60) as i16, -*cospi.offset(4) as i16);
    let mut cospi_m28_m36: __m256i =
        pair_set_w16_epi16(-*cospi.offset(28) as i16, -*cospi.offset(36) as i16);
    let mut cospi_m44_m20: __m256i =
        pair_set_w16_epi16(-*cospi.offset(44) as i16, -*cospi.offset(20) as i16);
    let mut cospi_m12_m52: __m256i =
        pair_set_w16_epi16(-*cospi.offset(12) as i16, -*cospi.offset(52) as i16);
    let mut cospi_p62_p02: __m256i =
        pair_set_w16_epi16(*cospi.offset(62) as i16, *cospi.offset(2) as i16);
    let mut cospi_m02_p62: __m256i =
        pair_set_w16_epi16(-*cospi.offset(2) as i16, *cospi.offset(62) as i16);
    let mut cospi_p30_p34: __m256i =
        pair_set_w16_epi16(*cospi.offset(30) as i16, *cospi.offset(34) as i16);
    let mut cospi_m34_p30: __m256i =
        pair_set_w16_epi16(-*cospi.offset(34) as i16, *cospi.offset(30) as i16);
    let mut cospi_p46_p18: __m256i =
        pair_set_w16_epi16(*cospi.offset(46) as i16, *cospi.offset(18) as i16);
    let mut cospi_m18_p46: __m256i =
        pair_set_w16_epi16(-*cospi.offset(18) as i16, *cospi.offset(46) as i16);
    let mut cospi_p14_p50: __m256i =
        pair_set_w16_epi16(*cospi.offset(14) as i16, *cospi.offset(50) as i16);
    let mut cospi_m50_p14: __m256i =
        pair_set_w16_epi16(-*cospi.offset(50) as i16, *cospi.offset(14) as i16);
    let mut cospi_p54_p10: __m256i =
        pair_set_w16_epi16(*cospi.offset(54) as i16, *cospi.offset(10) as i16);
    let mut cospi_m10_p54: __m256i =
        pair_set_w16_epi16(-*cospi.offset(10) as i16, *cospi.offset(54) as i16);
    let mut cospi_p22_p42: __m256i =
        pair_set_w16_epi16(*cospi.offset(22) as i16, *cospi.offset(42) as i16);
    let mut cospi_m42_p22: __m256i =
        pair_set_w16_epi16(-*cospi.offset(42) as i16, *cospi.offset(22) as i16);
    let mut cospi_p38_p26: __m256i =
        pair_set_w16_epi16(*cospi.offset(38) as i16, *cospi.offset(26) as i16);
    let mut cospi_m26_p38: __m256i =
        pair_set_w16_epi16(-*cospi.offset(26) as i16, *cospi.offset(38) as i16);
    let mut cospi_p06_p58: __m256i =
        pair_set_w16_epi16(*cospi.offset(6) as i16, *cospi.offset(58) as i16);
    let mut cospi_m58_p06: __m256i =
        pair_set_w16_epi16(-*cospi.offset(58) as i16, *cospi.offset(6) as i16);
    let mut cospi_p63_p01: __m256i =
        pair_set_w16_epi16(*cospi.offset(63) as i16, *cospi.offset(1) as i16);
    let mut cospi_m01_p63: __m256i =
        pair_set_w16_epi16(-*cospi.offset(1) as i16, *cospi.offset(63) as i16);
    let mut cospi_p31_p33: __m256i =
        pair_set_w16_epi16(*cospi.offset(31) as i16, *cospi.offset(33) as i16);
    let mut cospi_m33_p31: __m256i =
        pair_set_w16_epi16(-*cospi.offset(33) as i16, *cospi.offset(31) as i16);
    let mut cospi_p47_p17: __m256i =
        pair_set_w16_epi16(*cospi.offset(47) as i16, *cospi.offset(17) as i16);
    let mut cospi_m17_p47: __m256i =
        pair_set_w16_epi16(-*cospi.offset(17) as i16, *cospi.offset(47) as i16);
    let mut cospi_p15_p49: __m256i =
        pair_set_w16_epi16(*cospi.offset(15) as i16, *cospi.offset(49) as i16);
    let mut cospi_m49_p15: __m256i =
        pair_set_w16_epi16(-*cospi.offset(49) as i16, *cospi.offset(15) as i16);
    let mut cospi_p55_p09: __m256i =
        pair_set_w16_epi16(*cospi.offset(55) as i16, *cospi.offset(9) as i16);
    let mut cospi_m09_p55: __m256i =
        pair_set_w16_epi16(-*cospi.offset(9) as i16, *cospi.offset(55) as i16);
    let mut cospi_p23_p41: __m256i =
        pair_set_w16_epi16(*cospi.offset(23) as i16, *cospi.offset(41) as i16);
    let mut cospi_m41_p23: __m256i =
        pair_set_w16_epi16(-*cospi.offset(41) as i16, *cospi.offset(23) as i16);
    let mut cospi_p39_p25: __m256i =
        pair_set_w16_epi16(*cospi.offset(39) as i16, *cospi.offset(25) as i16);
    let mut cospi_m25_p39: __m256i =
        pair_set_w16_epi16(-*cospi.offset(25) as i16, *cospi.offset(39) as i16);
    let mut cospi_p07_p57: __m256i =
        pair_set_w16_epi16(*cospi.offset(7) as i16, *cospi.offset(57) as i16);
    let mut cospi_m57_p07: __m256i =
        pair_set_w16_epi16(-*cospi.offset(57) as i16, *cospi.offset(7) as i16);
    let mut cospi_p59_p05: __m256i =
        pair_set_w16_epi16(*cospi.offset(59) as i16, *cospi.offset(5) as i16);
    let mut cospi_m05_p59: __m256i =
        pair_set_w16_epi16(-*cospi.offset(5) as i16, *cospi.offset(59) as i16);
    let mut cospi_p27_p37: __m256i =
        pair_set_w16_epi16(*cospi.offset(27) as i16, *cospi.offset(37) as i16);
    let mut cospi_m37_p27: __m256i =
        pair_set_w16_epi16(-*cospi.offset(37) as i16, *cospi.offset(27) as i16);
    let mut cospi_p43_p21: __m256i =
        pair_set_w16_epi16(*cospi.offset(43) as i16, *cospi.offset(21) as i16);
    let mut cospi_m21_p43: __m256i =
        pair_set_w16_epi16(-*cospi.offset(21) as i16, *cospi.offset(43) as i16);
    let mut cospi_p11_p53: __m256i =
        pair_set_w16_epi16(*cospi.offset(11) as i16, *cospi.offset(53) as i16);
    let mut cospi_m53_p11: __m256i =
        pair_set_w16_epi16(-*cospi.offset(53) as i16, *cospi.offset(11) as i16);
    let mut cospi_p51_p13: __m256i =
        pair_set_w16_epi16(*cospi.offset(51) as i16, *cospi.offset(13) as i16);
    let mut cospi_m13_p51: __m256i =
        pair_set_w16_epi16(-*cospi.offset(13) as i16, *cospi.offset(51) as i16);
    let mut cospi_p19_p45: __m256i =
        pair_set_w16_epi16(*cospi.offset(19) as i16, *cospi.offset(45) as i16);
    let mut cospi_m45_p19: __m256i =
        pair_set_w16_epi16(-*cospi.offset(45) as i16, *cospi.offset(19) as i16);
    let mut cospi_p35_p29: __m256i =
        pair_set_w16_epi16(*cospi.offset(35) as i16, *cospi.offset(29) as i16);
    let mut cospi_m29_p35: __m256i =
        pair_set_w16_epi16(-*cospi.offset(29) as i16, *cospi.offset(35) as i16);
    let mut cospi_p03_p61: __m256i =
        pair_set_w16_epi16(*cospi.offset(3) as i16, *cospi.offset(61) as i16);
    let mut cospi_m61_p03: __m256i =
        pair_set_w16_epi16(-*cospi.offset(61) as i16, *cospi.offset(3) as i16);
    // stage 1
    let mut x1: [__m256i; 64] = core::mem::zeroed();
    btf_16_adds_subs_out_avx2(
        x1.as_mut_ptr(),
        x1.as_mut_ptr().offset(63),
        *input,
        *input.offset(63),
    );
    btf_16_adds_subs_out_avx2(
        x1.as_mut_ptr().offset(1),
        x1.as_mut_ptr().offset(62),
        *input.offset(1),
        *input.offset(62),
    );
    btf_16_adds_subs_out_avx2(
        x1.as_mut_ptr().offset(2),
        x1.as_mut_ptr().offset(61),
        *input.offset(2),
        *input.offset(61),
    );
    btf_16_adds_subs_out_avx2(
        x1.as_mut_ptr().offset(3),
        x1.as_mut_ptr().offset(60),
        *input.offset(3),
        *input.offset(60),
    );
    btf_16_adds_subs_out_avx2(
        x1.as_mut_ptr().offset(4),
        x1.as_mut_ptr().offset(59),
        *input.offset(4),
        *input.offset(59),
    );
    btf_16_adds_subs_out_avx2(
        x1.as_mut_ptr().offset(5),
        x1.as_mut_ptr().offset(58),
        *input.offset(5),
        *input.offset(58),
    );
    btf_16_adds_subs_out_avx2(
        x1.as_mut_ptr().offset(6),
        x1.as_mut_ptr().offset(57),
        *input.offset(6),
        *input.offset(57),
    );
    btf_16_adds_subs_out_avx2(
        x1.as_mut_ptr().offset(7),
        x1.as_mut_ptr().offset(56),
        *input.offset(7),
        *input.offset(56),
    );
    btf_16_adds_subs_out_avx2(
        x1.as_mut_ptr().offset(8),
        x1.as_mut_ptr().offset(55),
        *input.offset(8),
        *input.offset(55),
    );
    btf_16_adds_subs_out_avx2(
        x1.as_mut_ptr().offset(9),
        x1.as_mut_ptr().offset(54),
        *input.offset(9),
        *input.offset(54),
    );
    btf_16_adds_subs_out_avx2(
        x1.as_mut_ptr().offset(10),
        x1.as_mut_ptr().offset(53),
        *input.offset(10),
        *input.offset(53),
    );
    btf_16_adds_subs_out_avx2(
        x1.as_mut_ptr().offset(11),
        x1.as_mut_ptr().offset(52),
        *input.offset(11),
        *input.offset(52),
    );
    btf_16_adds_subs_out_avx2(
        x1.as_mut_ptr().offset(12),
        x1.as_mut_ptr().offset(51),
        *input.offset(12),
        *input.offset(51),
    );
    btf_16_adds_subs_out_avx2(
        x1.as_mut_ptr().offset(13),
        x1.as_mut_ptr().offset(50),
        *input.offset(13),
        *input.offset(50),
    );
    btf_16_adds_subs_out_avx2(
        x1.as_mut_ptr().offset(14),
        x1.as_mut_ptr().offset(49),
        *input.offset(14),
        *input.offset(49),
    );
    btf_16_adds_subs_out_avx2(
        x1.as_mut_ptr().offset(15),
        x1.as_mut_ptr().offset(48),
        *input.offset(15),
        *input.offset(48),
    );
    btf_16_adds_subs_out_avx2(
        x1.as_mut_ptr().offset(16),
        x1.as_mut_ptr().offset(47),
        *input.offset(16),
        *input.offset(47),
    );
    btf_16_adds_subs_out_avx2(
        x1.as_mut_ptr().offset(17),
        x1.as_mut_ptr().offset(46),
        *input.offset(17),
        *input.offset(46),
    );
    btf_16_adds_subs_out_avx2(
        x1.as_mut_ptr().offset(18),
        x1.as_mut_ptr().offset(45),
        *input.offset(18),
        *input.offset(45),
    );
    btf_16_adds_subs_out_avx2(
        x1.as_mut_ptr().offset(19),
        x1.as_mut_ptr().offset(44),
        *input.offset(19),
        *input.offset(44),
    );
    btf_16_adds_subs_out_avx2(
        x1.as_mut_ptr().offset(20),
        x1.as_mut_ptr().offset(43),
        *input.offset(20),
        *input.offset(43),
    );
    btf_16_adds_subs_out_avx2(
        x1.as_mut_ptr().offset(21),
        x1.as_mut_ptr().offset(42),
        *input.offset(21),
        *input.offset(42),
    );
    btf_16_adds_subs_out_avx2(
        x1.as_mut_ptr().offset(22),
        x1.as_mut_ptr().offset(41),
        *input.offset(22),
        *input.offset(41),
    );
    btf_16_adds_subs_out_avx2(
        x1.as_mut_ptr().offset(23),
        x1.as_mut_ptr().offset(40),
        *input.offset(23),
        *input.offset(40),
    );
    btf_16_adds_subs_out_avx2(
        x1.as_mut_ptr().offset(24),
        x1.as_mut_ptr().offset(39),
        *input.offset(24),
        *input.offset(39),
    );
    btf_16_adds_subs_out_avx2(
        x1.as_mut_ptr().offset(25),
        x1.as_mut_ptr().offset(38),
        *input.offset(25),
        *input.offset(38),
    );
    btf_16_adds_subs_out_avx2(
        x1.as_mut_ptr().offset(26),
        x1.as_mut_ptr().offset(37),
        *input.offset(26),
        *input.offset(37),
    );
    btf_16_adds_subs_out_avx2(
        x1.as_mut_ptr().offset(27),
        x1.as_mut_ptr().offset(36),
        *input.offset(27),
        *input.offset(36),
    );
    btf_16_adds_subs_out_avx2(
        x1.as_mut_ptr().offset(28),
        x1.as_mut_ptr().offset(35),
        *input.offset(28),
        *input.offset(35),
    );
    btf_16_adds_subs_out_avx2(
        x1.as_mut_ptr().offset(29),
        x1.as_mut_ptr().offset(34),
        *input.offset(29),
        *input.offset(34),
    );
    btf_16_adds_subs_out_avx2(
        x1.as_mut_ptr().offset(30),
        x1.as_mut_ptr().offset(33),
        *input.offset(30),
        *input.offset(33),
    );
    btf_16_adds_subs_out_avx2(
        x1.as_mut_ptr().offset(31),
        x1.as_mut_ptr().offset(32),
        *input.offset(31),
        *input.offset(32),
    );
    // stage 2
    btf_16_adds_subs_avx2(x1.as_mut_ptr(), x1.as_mut_ptr().offset(31));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(1), x1.as_mut_ptr().offset(30));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(2), x1.as_mut_ptr().offset(29));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(3), x1.as_mut_ptr().offset(28));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(4), x1.as_mut_ptr().offset(27));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(5), x1.as_mut_ptr().offset(26));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(6), x1.as_mut_ptr().offset(25));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(7), x1.as_mut_ptr().offset(24));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(8), x1.as_mut_ptr().offset(23));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(9), x1.as_mut_ptr().offset(22));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(10), x1.as_mut_ptr().offset(21));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(11), x1.as_mut_ptr().offset(20));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(12), x1.as_mut_ptr().offset(19));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(13), x1.as_mut_ptr().offset(18));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(14), x1.as_mut_ptr().offset(17));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(15), x1.as_mut_ptr().offset(16));
    btf_16_w16_avx2(
        cospi_m32_p32,
        cospi_p32_p32,
        x1.as_mut_ptr().offset(40),
        x1.as_mut_ptr().offset(55),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m32_p32,
        cospi_p32_p32,
        x1.as_mut_ptr().offset(41),
        x1.as_mut_ptr().offset(54),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m32_p32,
        cospi_p32_p32,
        x1.as_mut_ptr().offset(42),
        x1.as_mut_ptr().offset(53),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m32_p32,
        cospi_p32_p32,
        x1.as_mut_ptr().offset(43),
        x1.as_mut_ptr().offset(52),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m32_p32,
        cospi_p32_p32,
        x1.as_mut_ptr().offset(44),
        x1.as_mut_ptr().offset(51),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m32_p32,
        cospi_p32_p32,
        x1.as_mut_ptr().offset(45),
        x1.as_mut_ptr().offset(50),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m32_p32,
        cospi_p32_p32,
        x1.as_mut_ptr().offset(46),
        x1.as_mut_ptr().offset(49),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m32_p32,
        cospi_p32_p32,
        x1.as_mut_ptr().offset(47),
        x1.as_mut_ptr().offset(48),
        _r,
        cos_bit as i32,
    );
    // stage 3
    btf_16_adds_subs_avx2(x1.as_mut_ptr(), x1.as_mut_ptr().offset(15));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(1), x1.as_mut_ptr().offset(14));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(2), x1.as_mut_ptr().offset(13));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(3), x1.as_mut_ptr().offset(12));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(4), x1.as_mut_ptr().offset(11));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(5), x1.as_mut_ptr().offset(10));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(6), x1.as_mut_ptr().offset(9));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(7), x1.as_mut_ptr().offset(8));
    btf_16_w16_avx2(
        cospi_m32_p32,
        cospi_p32_p32,
        x1.as_mut_ptr().offset(20),
        x1.as_mut_ptr().offset(27),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m32_p32,
        cospi_p32_p32,
        x1.as_mut_ptr().offset(21),
        x1.as_mut_ptr().offset(26),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m32_p32,
        cospi_p32_p32,
        x1.as_mut_ptr().offset(22),
        x1.as_mut_ptr().offset(25),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m32_p32,
        cospi_p32_p32,
        x1.as_mut_ptr().offset(23),
        x1.as_mut_ptr().offset(24),
        _r,
        cos_bit as i32,
    );
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(32), x1.as_mut_ptr().offset(47));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(33), x1.as_mut_ptr().offset(46));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(34), x1.as_mut_ptr().offset(45));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(35), x1.as_mut_ptr().offset(44));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(36), x1.as_mut_ptr().offset(43));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(37), x1.as_mut_ptr().offset(42));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(38), x1.as_mut_ptr().offset(41));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(39), x1.as_mut_ptr().offset(40));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(63), x1.as_mut_ptr().offset(48));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(62), x1.as_mut_ptr().offset(49));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(61), x1.as_mut_ptr().offset(50));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(60), x1.as_mut_ptr().offset(51));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(59), x1.as_mut_ptr().offset(52));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(58), x1.as_mut_ptr().offset(53));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(57), x1.as_mut_ptr().offset(54));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(56), x1.as_mut_ptr().offset(55));
    // stage 4
    btf_16_adds_subs_avx2(x1.as_mut_ptr(), x1.as_mut_ptr().offset(7));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(1), x1.as_mut_ptr().offset(6));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(2), x1.as_mut_ptr().offset(5));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(3), x1.as_mut_ptr().offset(4));
    btf_16_w16_avx2(
        cospi_m32_p32,
        cospi_p32_p32,
        x1.as_mut_ptr().offset(10),
        x1.as_mut_ptr().offset(13),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m32_p32,
        cospi_p32_p32,
        x1.as_mut_ptr().offset(11),
        x1.as_mut_ptr().offset(12),
        _r,
        cos_bit as i32,
    );
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(16), x1.as_mut_ptr().offset(23));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(17), x1.as_mut_ptr().offset(22));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(18), x1.as_mut_ptr().offset(21));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(19), x1.as_mut_ptr().offset(20));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(31), x1.as_mut_ptr().offset(24));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(30), x1.as_mut_ptr().offset(25));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(29), x1.as_mut_ptr().offset(26));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(28), x1.as_mut_ptr().offset(27));
    btf_16_w16_avx2(
        cospi_m16_p48,
        cospi_p48_p16,
        x1.as_mut_ptr().offset(36),
        x1.as_mut_ptr().offset(59),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m16_p48,
        cospi_p48_p16,
        x1.as_mut_ptr().offset(37),
        x1.as_mut_ptr().offset(58),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m16_p48,
        cospi_p48_p16,
        x1.as_mut_ptr().offset(38),
        x1.as_mut_ptr().offset(57),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m16_p48,
        cospi_p48_p16,
        x1.as_mut_ptr().offset(39),
        x1.as_mut_ptr().offset(56),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m48_m16,
        cospi_m16_p48,
        x1.as_mut_ptr().offset(40),
        x1.as_mut_ptr().offset(55),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m48_m16,
        cospi_m16_p48,
        x1.as_mut_ptr().offset(41),
        x1.as_mut_ptr().offset(54),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m48_m16,
        cospi_m16_p48,
        x1.as_mut_ptr().offset(42),
        x1.as_mut_ptr().offset(53),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m48_m16,
        cospi_m16_p48,
        x1.as_mut_ptr().offset(43),
        x1.as_mut_ptr().offset(52),
        _r,
        cos_bit as i32,
    );
    // stage 5
    btf_16_adds_subs_avx2(x1.as_mut_ptr(), x1.as_mut_ptr().offset(3));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(1), x1.as_mut_ptr().offset(2));
    btf_16_w16_avx2(
        cospi_m32_p32,
        cospi_p32_p32,
        x1.as_mut_ptr().offset(5),
        x1.as_mut_ptr().offset(6),
        _r,
        cos_bit as i32,
    );
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(8), x1.as_mut_ptr().offset(11));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(9), x1.as_mut_ptr().offset(10));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(15), x1.as_mut_ptr().offset(12));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(14), x1.as_mut_ptr().offset(13));
    btf_16_w16_avx2(
        cospi_m16_p48,
        cospi_p48_p16,
        x1.as_mut_ptr().offset(18),
        x1.as_mut_ptr().offset(29),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m16_p48,
        cospi_p48_p16,
        x1.as_mut_ptr().offset(19),
        x1.as_mut_ptr().offset(28),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m48_m16,
        cospi_m16_p48,
        x1.as_mut_ptr().offset(20),
        x1.as_mut_ptr().offset(27),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m48_m16,
        cospi_m16_p48,
        x1.as_mut_ptr().offset(21),
        x1.as_mut_ptr().offset(26),
        _r,
        cos_bit as i32,
    );
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(32), x1.as_mut_ptr().offset(39));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(33), x1.as_mut_ptr().offset(38));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(34), x1.as_mut_ptr().offset(37));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(35), x1.as_mut_ptr().offset(36));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(47), x1.as_mut_ptr().offset(40));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(46), x1.as_mut_ptr().offset(41));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(45), x1.as_mut_ptr().offset(42));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(44), x1.as_mut_ptr().offset(43));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(48), x1.as_mut_ptr().offset(55));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(49), x1.as_mut_ptr().offset(54));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(50), x1.as_mut_ptr().offset(53));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(51), x1.as_mut_ptr().offset(52));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(63), x1.as_mut_ptr().offset(56));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(62), x1.as_mut_ptr().offset(57));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(61), x1.as_mut_ptr().offset(58));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(60), x1.as_mut_ptr().offset(59));
    // stage 6
    btf_16_w16_avx2(
        cospi_p32_p32,
        cospi_p32_m32,
        x1.as_mut_ptr(),
        x1.as_mut_ptr().offset(1),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_p48_p16,
        cospi_m16_p48,
        x1.as_mut_ptr().offset(2),
        x1.as_mut_ptr().offset(3),
        _r,
        cos_bit as i32,
    );
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(4), x1.as_mut_ptr().offset(5));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(7), x1.as_mut_ptr().offset(6));
    btf_16_w16_avx2(
        cospi_m16_p48,
        cospi_p48_p16,
        x1.as_mut_ptr().offset(9),
        x1.as_mut_ptr().offset(14),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m48_m16,
        cospi_m16_p48,
        x1.as_mut_ptr().offset(10),
        x1.as_mut_ptr().offset(13),
        _r,
        cos_bit as i32,
    );
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(16), x1.as_mut_ptr().offset(19));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(17), x1.as_mut_ptr().offset(18));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(23), x1.as_mut_ptr().offset(20));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(22), x1.as_mut_ptr().offset(21));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(24), x1.as_mut_ptr().offset(27));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(25), x1.as_mut_ptr().offset(26));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(31), x1.as_mut_ptr().offset(28));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(30), x1.as_mut_ptr().offset(29));
    btf_16_w16_avx2(
        cospi_m08_p56,
        cospi_p56_p08,
        x1.as_mut_ptr().offset(34),
        x1.as_mut_ptr().offset(61),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m08_p56,
        cospi_p56_p08,
        x1.as_mut_ptr().offset(35),
        x1.as_mut_ptr().offset(60),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m56_m08,
        cospi_m08_p56,
        x1.as_mut_ptr().offset(36),
        x1.as_mut_ptr().offset(59),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m56_m08,
        cospi_m08_p56,
        x1.as_mut_ptr().offset(37),
        x1.as_mut_ptr().offset(58),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m40_p24,
        cospi_p24_p40,
        x1.as_mut_ptr().offset(42),
        x1.as_mut_ptr().offset(53),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m40_p24,
        cospi_p24_p40,
        x1.as_mut_ptr().offset(43),
        x1.as_mut_ptr().offset(52),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m24_m40,
        cospi_m40_p24,
        x1.as_mut_ptr().offset(44),
        x1.as_mut_ptr().offset(51),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m24_m40,
        cospi_m40_p24,
        x1.as_mut_ptr().offset(45),
        x1.as_mut_ptr().offset(50),
        _r,
        cos_bit as i32,
    );
    // stage 7
    btf_16_w16_avx2(
        cospi_p56_p08,
        cospi_m08_p56,
        x1.as_mut_ptr().offset(4),
        x1.as_mut_ptr().offset(7),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_p24_p40,
        cospi_m40_p24,
        x1.as_mut_ptr().offset(5),
        x1.as_mut_ptr().offset(6),
        _r,
        cos_bit as i32,
    );
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(8), x1.as_mut_ptr().offset(9));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(11), x1.as_mut_ptr().offset(10));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(12), x1.as_mut_ptr().offset(13));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(15), x1.as_mut_ptr().offset(14));
    btf_16_w16_avx2(
        cospi_m08_p56,
        cospi_p56_p08,
        x1.as_mut_ptr().offset(17),
        x1.as_mut_ptr().offset(30),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m56_m08,
        cospi_m08_p56,
        x1.as_mut_ptr().offset(18),
        x1.as_mut_ptr().offset(29),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m40_p24,
        cospi_p24_p40,
        x1.as_mut_ptr().offset(21),
        x1.as_mut_ptr().offset(26),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m24_m40,
        cospi_m40_p24,
        x1.as_mut_ptr().offset(22),
        x1.as_mut_ptr().offset(25),
        _r,
        cos_bit as i32,
    );
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(32), x1.as_mut_ptr().offset(35));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(33), x1.as_mut_ptr().offset(34));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(39), x1.as_mut_ptr().offset(36));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(38), x1.as_mut_ptr().offset(37));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(40), x1.as_mut_ptr().offset(43));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(41), x1.as_mut_ptr().offset(42));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(47), x1.as_mut_ptr().offset(44));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(46), x1.as_mut_ptr().offset(45));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(48), x1.as_mut_ptr().offset(51));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(49), x1.as_mut_ptr().offset(50));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(55), x1.as_mut_ptr().offset(52));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(54), x1.as_mut_ptr().offset(53));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(56), x1.as_mut_ptr().offset(59));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(57), x1.as_mut_ptr().offset(58));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(63), x1.as_mut_ptr().offset(60));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(62), x1.as_mut_ptr().offset(61));
    // stage 8
    btf_16_w16_avx2(
        cospi_p60_p04,
        cospi_m04_p60,
        x1.as_mut_ptr().offset(8),
        x1.as_mut_ptr().offset(15),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_p28_p36,
        cospi_m36_p28,
        x1.as_mut_ptr().offset(9),
        x1.as_mut_ptr().offset(14),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_p44_p20,
        cospi_m20_p44,
        x1.as_mut_ptr().offset(10),
        x1.as_mut_ptr().offset(13),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_p12_p52,
        cospi_m52_p12,
        x1.as_mut_ptr().offset(11),
        x1.as_mut_ptr().offset(12),
        _r,
        cos_bit as i32,
    );
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(16), x1.as_mut_ptr().offset(17));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(19), x1.as_mut_ptr().offset(18));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(20), x1.as_mut_ptr().offset(21));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(23), x1.as_mut_ptr().offset(22));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(24), x1.as_mut_ptr().offset(25));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(27), x1.as_mut_ptr().offset(26));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(28), x1.as_mut_ptr().offset(29));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(31), x1.as_mut_ptr().offset(30));
    btf_16_w16_avx2(
        cospi_m04_p60,
        cospi_p60_p04,
        x1.as_mut_ptr().offset(33),
        x1.as_mut_ptr().offset(62),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m60_m04,
        cospi_m04_p60,
        x1.as_mut_ptr().offset(34),
        x1.as_mut_ptr().offset(61),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m36_p28,
        cospi_p28_p36,
        x1.as_mut_ptr().offset(37),
        x1.as_mut_ptr().offset(58),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m28_m36,
        cospi_m36_p28,
        x1.as_mut_ptr().offset(38),
        x1.as_mut_ptr().offset(57),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m20_p44,
        cospi_p44_p20,
        x1.as_mut_ptr().offset(41),
        x1.as_mut_ptr().offset(54),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m44_m20,
        cospi_m20_p44,
        x1.as_mut_ptr().offset(42),
        x1.as_mut_ptr().offset(53),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m52_p12,
        cospi_p12_p52,
        x1.as_mut_ptr().offset(45),
        x1.as_mut_ptr().offset(50),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m12_m52,
        cospi_m52_p12,
        x1.as_mut_ptr().offset(46),
        x1.as_mut_ptr().offset(49),
        _r,
        cos_bit as i32,
    );
    // stage 9
    btf_16_w16_avx2(
        cospi_p62_p02,
        cospi_m02_p62,
        x1.as_mut_ptr().offset(16),
        x1.as_mut_ptr().offset(31),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_p30_p34,
        cospi_m34_p30,
        x1.as_mut_ptr().offset(17),
        x1.as_mut_ptr().offset(30),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_p46_p18,
        cospi_m18_p46,
        x1.as_mut_ptr().offset(18),
        x1.as_mut_ptr().offset(29),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_p14_p50,
        cospi_m50_p14,
        x1.as_mut_ptr().offset(19),
        x1.as_mut_ptr().offset(28),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_p54_p10,
        cospi_m10_p54,
        x1.as_mut_ptr().offset(20),
        x1.as_mut_ptr().offset(27),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_p22_p42,
        cospi_m42_p22,
        x1.as_mut_ptr().offset(21),
        x1.as_mut_ptr().offset(26),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_p38_p26,
        cospi_m26_p38,
        x1.as_mut_ptr().offset(22),
        x1.as_mut_ptr().offset(25),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_p06_p58,
        cospi_m58_p06,
        x1.as_mut_ptr().offset(23),
        x1.as_mut_ptr().offset(24),
        _r,
        cos_bit as i32,
    );
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(32), x1.as_mut_ptr().offset(33));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(35), x1.as_mut_ptr().offset(34));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(36), x1.as_mut_ptr().offset(37));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(39), x1.as_mut_ptr().offset(38));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(40), x1.as_mut_ptr().offset(41));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(43), x1.as_mut_ptr().offset(42));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(44), x1.as_mut_ptr().offset(45));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(47), x1.as_mut_ptr().offset(46));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(48), x1.as_mut_ptr().offset(49));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(51), x1.as_mut_ptr().offset(50));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(52), x1.as_mut_ptr().offset(53));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(55), x1.as_mut_ptr().offset(54));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(56), x1.as_mut_ptr().offset(57));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(59), x1.as_mut_ptr().offset(58));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(60), x1.as_mut_ptr().offset(61));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(63), x1.as_mut_ptr().offset(62));
    // stage 10
    btf_16_w16_avx2(
        cospi_p63_p01,
        cospi_m01_p63,
        x1.as_mut_ptr().offset(32),
        x1.as_mut_ptr().offset(63),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_p31_p33,
        cospi_m33_p31,
        x1.as_mut_ptr().offset(33),
        x1.as_mut_ptr().offset(62),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_p47_p17,
        cospi_m17_p47,
        x1.as_mut_ptr().offset(34),
        x1.as_mut_ptr().offset(61),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_p15_p49,
        cospi_m49_p15,
        x1.as_mut_ptr().offset(35),
        x1.as_mut_ptr().offset(60),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_p55_p09,
        cospi_m09_p55,
        x1.as_mut_ptr().offset(36),
        x1.as_mut_ptr().offset(59),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_p23_p41,
        cospi_m41_p23,
        x1.as_mut_ptr().offset(37),
        x1.as_mut_ptr().offset(58),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_p39_p25,
        cospi_m25_p39,
        x1.as_mut_ptr().offset(38),
        x1.as_mut_ptr().offset(57),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_p07_p57,
        cospi_m57_p07,
        x1.as_mut_ptr().offset(39),
        x1.as_mut_ptr().offset(56),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_p59_p05,
        cospi_m05_p59,
        x1.as_mut_ptr().offset(40),
        x1.as_mut_ptr().offset(55),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_p27_p37,
        cospi_m37_p27,
        x1.as_mut_ptr().offset(41),
        x1.as_mut_ptr().offset(54),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_p43_p21,
        cospi_m21_p43,
        x1.as_mut_ptr().offset(42),
        x1.as_mut_ptr().offset(53),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_p11_p53,
        cospi_m53_p11,
        x1.as_mut_ptr().offset(43),
        x1.as_mut_ptr().offset(52),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_p51_p13,
        cospi_m13_p51,
        x1.as_mut_ptr().offset(44),
        x1.as_mut_ptr().offset(51),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_p19_p45,
        cospi_m45_p19,
        x1.as_mut_ptr().offset(45),
        x1.as_mut_ptr().offset(50),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_p35_p29,
        cospi_m29_p35,
        x1.as_mut_ptr().offset(46),
        x1.as_mut_ptr().offset(49),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_p03_p61,
        cospi_m61_p03,
        x1.as_mut_ptr().offset(47),
        x1.as_mut_ptr().offset(48),
        _r,
        cos_bit as i32,
    );
    // stage 11
    *output = x1[0];
    *output.offset(1) = x1[32];
    *output.offset(2) = x1[16];
    *output.offset(3) = x1[48];
    *output.offset(4) = x1[8];
    *output.offset(5) = x1[40];
    *output.offset(6) = x1[24];
    *output.offset(7) = x1[56];
    *output.offset(8) = x1[4];
    *output.offset(9) = x1[36];
    *output.offset(10) = x1[20];
    *output.offset(11) = x1[52];
    *output.offset(12) = x1[12];
    *output.offset(13) = x1[44];
    *output.offset(14) = x1[28];
    *output.offset(15) = x1[60];
    *output.offset(16) = x1[2];
    *output.offset(17) = x1[34];
    *output.offset(18) = x1[18];
    *output.offset(19) = x1[50];
    *output.offset(20) = x1[10];
    *output.offset(21) = x1[42];
    *output.offset(22) = x1[26];
    *output.offset(23) = x1[58];
    *output.offset(24) = x1[6];
    *output.offset(25) = x1[38];
    *output.offset(26) = x1[22];
    *output.offset(27) = x1[54];
    *output.offset(28) = x1[14];
    *output.offset(29) = x1[46];
    *output.offset(30) = x1[30];
    *output.offset(31) = x1[62];
    *output.offset(32) = x1[1];
    *output.offset(33) = x1[33];
    *output.offset(34) = x1[17];
    *output.offset(35) = x1[49];
    *output.offset(36) = x1[9];
    *output.offset(37) = x1[41];
    *output.offset(38) = x1[25];
    *output.offset(39) = x1[57];
    *output.offset(40) = x1[5];
    *output.offset(41) = x1[37];
    *output.offset(42) = x1[21];
    *output.offset(43) = x1[53];
    *output.offset(44) = x1[13];
    *output.offset(45) = x1[45];
    *output.offset(46) = x1[29];
    *output.offset(47) = x1[61];
    *output.offset(48) = x1[3];
    *output.offset(49) = x1[35];
    *output.offset(50) = x1[19];
    *output.offset(51) = x1[51];
    *output.offset(52) = x1[11];
    *output.offset(53) = x1[43];
    *output.offset(54) = x1[27];
    *output.offset(55) = x1[59];
    *output.offset(56) = x1[7];
    *output.offset(57) = x1[39];
    *output.offset(58) = x1[23];
    *output.offset(59) = x1[55];
    *output.offset(60) = x1[15];
    *output.offset(61) = x1[47];
    *output.offset(62) = x1[31];
    *output.offset(63) = x1[63];
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn fdct32_avx2(input: *const __m256i, output: *mut __m256i, cos_bit: i8) {
    let mut x1: [__m256i; 32] = core::mem::zeroed();
    let mut cospi: *const i32 = cospi_arr(cos_bit as i32);
    let _r: __m256i = _mm256_set1_epi32(1 << (cos_bit as i32 - 1));
    // stage 0
    // stage 1
    btf_32_add_sub_out_avx2(
        x1.as_mut_ptr(),
        x1.as_mut_ptr().offset(31),
        *input,
        *input.offset(31),
    );
    btf_32_add_sub_out_avx2(
        x1.as_mut_ptr().offset(1),
        x1.as_mut_ptr().offset(30),
        *input.offset(1),
        *input.offset(30),
    );
    btf_32_add_sub_out_avx2(
        x1.as_mut_ptr().offset(2),
        x1.as_mut_ptr().offset(29),
        *input.offset(2),
        *input.offset(29),
    );
    btf_32_add_sub_out_avx2(
        x1.as_mut_ptr().offset(3),
        x1.as_mut_ptr().offset(28),
        *input.offset(3),
        *input.offset(28),
    );
    btf_32_add_sub_out_avx2(
        x1.as_mut_ptr().offset(4),
        x1.as_mut_ptr().offset(27),
        *input.offset(4),
        *input.offset(27),
    );
    btf_32_add_sub_out_avx2(
        x1.as_mut_ptr().offset(5),
        x1.as_mut_ptr().offset(26),
        *input.offset(5),
        *input.offset(26),
    );
    btf_32_add_sub_out_avx2(
        x1.as_mut_ptr().offset(6),
        x1.as_mut_ptr().offset(25),
        *input.offset(6),
        *input.offset(25),
    );
    btf_32_add_sub_out_avx2(
        x1.as_mut_ptr().offset(7),
        x1.as_mut_ptr().offset(24),
        *input.offset(7),
        *input.offset(24),
    );
    btf_32_add_sub_out_avx2(
        x1.as_mut_ptr().offset(8),
        x1.as_mut_ptr().offset(23),
        *input.offset(8),
        *input.offset(23),
    );
    btf_32_add_sub_out_avx2(
        x1.as_mut_ptr().offset(9),
        x1.as_mut_ptr().offset(22),
        *input.offset(9),
        *input.offset(22),
    );
    btf_32_add_sub_out_avx2(
        x1.as_mut_ptr().offset(10),
        x1.as_mut_ptr().offset(21),
        *input.offset(10),
        *input.offset(21),
    );
    btf_32_add_sub_out_avx2(
        x1.as_mut_ptr().offset(11),
        x1.as_mut_ptr().offset(20),
        *input.offset(11),
        *input.offset(20),
    );
    btf_32_add_sub_out_avx2(
        x1.as_mut_ptr().offset(12),
        x1.as_mut_ptr().offset(19),
        *input.offset(12),
        *input.offset(19),
    );
    btf_32_add_sub_out_avx2(
        x1.as_mut_ptr().offset(13),
        x1.as_mut_ptr().offset(18),
        *input.offset(13),
        *input.offset(18),
    );
    btf_32_add_sub_out_avx2(
        x1.as_mut_ptr().offset(14),
        x1.as_mut_ptr().offset(17),
        *input.offset(14),
        *input.offset(17),
    );
    btf_32_add_sub_out_avx2(
        x1.as_mut_ptr().offset(15),
        x1.as_mut_ptr().offset(16),
        *input.offset(15),
        *input.offset(16),
    );
    // stage 2
    btf_32_add_sub_avx2(x1.as_mut_ptr(), x1.as_mut_ptr().offset(15));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(1), x1.as_mut_ptr().offset(14));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(2), x1.as_mut_ptr().offset(13));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(3), x1.as_mut_ptr().offset(12));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(4), x1.as_mut_ptr().offset(11));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(5), x1.as_mut_ptr().offset(10));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(6), x1.as_mut_ptr().offset(9));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(7), x1.as_mut_ptr().offset(8));
    btf_32_avx2_type0(
        -*cospi.offset(32),
        *cospi.offset(32),
        x1.as_mut_ptr().offset(20),
        x1.as_mut_ptr().offset(27),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type0(
        -*cospi.offset(32),
        *cospi.offset(32),
        x1.as_mut_ptr().offset(21),
        x1.as_mut_ptr().offset(26),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type0(
        -*cospi.offset(32),
        *cospi.offset(32),
        x1.as_mut_ptr().offset(22),
        x1.as_mut_ptr().offset(25),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type0(
        -*cospi.offset(32),
        *cospi.offset(32),
        x1.as_mut_ptr().offset(23),
        x1.as_mut_ptr().offset(24),
        _r,
        cos_bit as i32,
    );
    // stage 3
    btf_32_add_sub_avx2(x1.as_mut_ptr(), x1.as_mut_ptr().offset(7));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(1), x1.as_mut_ptr().offset(6));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(2), x1.as_mut_ptr().offset(5));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(3), x1.as_mut_ptr().offset(4));
    btf_32_avx2_type0(
        -*cospi.offset(32),
        *cospi.offset(32),
        x1.as_mut_ptr().offset(10),
        x1.as_mut_ptr().offset(13),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type0(
        -*cospi.offset(32),
        *cospi.offset(32),
        x1.as_mut_ptr().offset(11),
        x1.as_mut_ptr().offset(12),
        _r,
        cos_bit as i32,
    );
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(16), x1.as_mut_ptr().offset(23));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(17), x1.as_mut_ptr().offset(22));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(18), x1.as_mut_ptr().offset(21));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(19), x1.as_mut_ptr().offset(20));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(31), x1.as_mut_ptr().offset(24));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(30), x1.as_mut_ptr().offset(25));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(29), x1.as_mut_ptr().offset(26));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(28), x1.as_mut_ptr().offset(27));
    // stage 4
    btf_32_add_sub_avx2(x1.as_mut_ptr(), x1.as_mut_ptr().offset(3));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(1), x1.as_mut_ptr().offset(2));
    btf_32_avx2_type0(
        -*cospi.offset(32),
        *cospi.offset(32),
        x1.as_mut_ptr().offset(5),
        x1.as_mut_ptr().offset(6),
        _r,
        cos_bit as i32,
    );
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(8), x1.as_mut_ptr().offset(11));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(9), x1.as_mut_ptr().offset(10));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(15), x1.as_mut_ptr().offset(12));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(14), x1.as_mut_ptr().offset(13));
    btf_32_avx2_type0(
        -*cospi.offset(16),
        *cospi.offset(48),
        x1.as_mut_ptr().offset(18),
        x1.as_mut_ptr().offset(29),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type0(
        -*cospi.offset(16),
        *cospi.offset(48),
        x1.as_mut_ptr().offset(19),
        x1.as_mut_ptr().offset(28),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type0(
        -*cospi.offset(48),
        -*cospi.offset(16),
        x1.as_mut_ptr().offset(20),
        x1.as_mut_ptr().offset(27),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type0(
        -*cospi.offset(48),
        -*cospi.offset(16),
        x1.as_mut_ptr().offset(21),
        x1.as_mut_ptr().offset(26),
        _r,
        cos_bit as i32,
    );
    // stage 5
    btf_32_avx2_type0(
        *cospi.offset(32),
        *cospi.offset(32),
        x1.as_mut_ptr(),
        x1.as_mut_ptr().offset(1),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type1(
        *cospi.offset(48),
        *cospi.offset(16),
        x1.as_mut_ptr().offset(2),
        x1.as_mut_ptr().offset(3),
        _r,
        cos_bit as i32,
    );
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(4), x1.as_mut_ptr().offset(5));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(7), x1.as_mut_ptr().offset(6));
    btf_32_avx2_type0(
        -*cospi.offset(16),
        *cospi.offset(48),
        x1.as_mut_ptr().offset(9),
        x1.as_mut_ptr().offset(14),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type0(
        -*cospi.offset(48),
        -*cospi.offset(16),
        x1.as_mut_ptr().offset(10),
        x1.as_mut_ptr().offset(13),
        _r,
        cos_bit as i32,
    );
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(16), x1.as_mut_ptr().offset(19));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(17), x1.as_mut_ptr().offset(18));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(23), x1.as_mut_ptr().offset(20));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(22), x1.as_mut_ptr().offset(21));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(24), x1.as_mut_ptr().offset(27));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(25), x1.as_mut_ptr().offset(26));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(31), x1.as_mut_ptr().offset(28));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(30), x1.as_mut_ptr().offset(29));
    // stage 6
    btf_32_avx2_type1(
        *cospi.offset(56),
        *cospi.offset(8),
        x1.as_mut_ptr().offset(4),
        x1.as_mut_ptr().offset(7),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type1(
        *cospi.offset(24),
        *cospi.offset(40),
        x1.as_mut_ptr().offset(5),
        x1.as_mut_ptr().offset(6),
        _r,
        cos_bit as i32,
    );
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(8), x1.as_mut_ptr().offset(9));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(11), x1.as_mut_ptr().offset(10));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(12), x1.as_mut_ptr().offset(13));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(15), x1.as_mut_ptr().offset(14));
    btf_32_avx2_type0(
        -*cospi.offset(8),
        *cospi.offset(56),
        x1.as_mut_ptr().offset(17),
        x1.as_mut_ptr().offset(30),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type0(
        -*cospi.offset(56),
        -*cospi.offset(8),
        x1.as_mut_ptr().offset(18),
        x1.as_mut_ptr().offset(29),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type0(
        -*cospi.offset(40),
        *cospi.offset(24),
        x1.as_mut_ptr().offset(21),
        x1.as_mut_ptr().offset(26),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type0(
        -*cospi.offset(24),
        -*cospi.offset(40),
        x1.as_mut_ptr().offset(22),
        x1.as_mut_ptr().offset(25),
        _r,
        cos_bit as i32,
    );
    // stage 7
    btf_32_avx2_type1(
        *cospi.offset(60),
        *cospi.offset(4),
        x1.as_mut_ptr().offset(8),
        x1.as_mut_ptr().offset(15),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type1(
        *cospi.offset(28),
        *cospi.offset(36),
        x1.as_mut_ptr().offset(9),
        x1.as_mut_ptr().offset(14),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type1(
        *cospi.offset(44),
        *cospi.offset(20),
        x1.as_mut_ptr().offset(10),
        x1.as_mut_ptr().offset(13),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type1(
        *cospi.offset(12),
        *cospi.offset(52),
        x1.as_mut_ptr().offset(11),
        x1.as_mut_ptr().offset(12),
        _r,
        cos_bit as i32,
    );
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(16), x1.as_mut_ptr().offset(17));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(19), x1.as_mut_ptr().offset(18));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(20), x1.as_mut_ptr().offset(21));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(23), x1.as_mut_ptr().offset(22));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(24), x1.as_mut_ptr().offset(25));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(27), x1.as_mut_ptr().offset(26));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(28), x1.as_mut_ptr().offset(29));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(31), x1.as_mut_ptr().offset(30));
    // stage 8
    btf_32_avx2_type1(
        *cospi.offset(62),
        *cospi.offset(2),
        x1.as_mut_ptr().offset(16),
        x1.as_mut_ptr().offset(31),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type1(
        *cospi.offset(30),
        *cospi.offset(34),
        x1.as_mut_ptr().offset(17),
        x1.as_mut_ptr().offset(30),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type1(
        *cospi.offset(46),
        *cospi.offset(18),
        x1.as_mut_ptr().offset(18),
        x1.as_mut_ptr().offset(29),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type1(
        *cospi.offset(14),
        *cospi.offset(50),
        x1.as_mut_ptr().offset(19),
        x1.as_mut_ptr().offset(28),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type1(
        *cospi.offset(54),
        *cospi.offset(10),
        x1.as_mut_ptr().offset(20),
        x1.as_mut_ptr().offset(27),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type1(
        *cospi.offset(22),
        *cospi.offset(42),
        x1.as_mut_ptr().offset(21),
        x1.as_mut_ptr().offset(26),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type1(
        *cospi.offset(38),
        *cospi.offset(26),
        x1.as_mut_ptr().offset(22),
        x1.as_mut_ptr().offset(25),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type1(
        *cospi.offset(6),
        *cospi.offset(58),
        x1.as_mut_ptr().offset(23),
        x1.as_mut_ptr().offset(24),
        _r,
        cos_bit as i32,
    );
    // stage 9
    *output = x1[0];
    *output.offset(1) = x1[16];
    *output.offset(2) = x1[8];
    *output.offset(3) = x1[24];
    *output.offset(4) = x1[4];
    *output.offset(5) = x1[20];
    *output.offset(6) = x1[12];
    *output.offset(7) = x1[28];
    *output.offset(8) = x1[2];
    *output.offset(9) = x1[18];
    *output.offset(10) = x1[10];
    *output.offset(11) = x1[26];
    *output.offset(12) = x1[6];
    *output.offset(13) = x1[22];
    *output.offset(14) = x1[14];
    *output.offset(15) = x1[30];
    *output.offset(16) = x1[1];
    *output.offset(17) = x1[17];
    *output.offset(18) = x1[9];
    *output.offset(19) = x1[25];
    *output.offset(20) = x1[5];
    *output.offset(21) = x1[21];
    *output.offset(22) = x1[13];
    *output.offset(23) = x1[29];
    *output.offset(24) = x1[3];
    *output.offset(25) = x1[19];
    *output.offset(26) = x1[11];
    *output.offset(27) = x1[27];
    *output.offset(28) = x1[7];
    *output.offset(29) = x1[23];
    *output.offset(30) = x1[15];
    *output.offset(31) = x1[31];
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn fdct64_new_avx2(input: *const __m256i, output: *mut __m256i, cos_bit: i8) {
    let mut cospi: *const i32 = cospi_arr(cos_bit as i32);
    let _r: __m256i = _mm256_set1_epi32(1 << (cos_bit as i32 - 1));
    let mut cospi_m32: __m256i = _mm256_set1_epi32(-*cospi.offset(32));
    let mut cospi_p32: __m256i = _mm256_set1_epi32(*cospi.offset(32));
    let mut cospi_m16: __m256i = _mm256_set1_epi32(-*cospi.offset(16));
    let mut cospi_p48: __m256i = _mm256_set1_epi32(*cospi.offset(48));
    let mut cospi_m48: __m256i = _mm256_set1_epi32(-*cospi.offset(48));
    let mut cospi_p16: __m256i = _mm256_set1_epi32(*cospi.offset(16));
    let mut cospi_m08: __m256i = _mm256_set1_epi32(-*cospi.offset(8));
    let mut cospi_p56: __m256i = _mm256_set1_epi32(*cospi.offset(56));
    let mut cospi_m56: __m256i = _mm256_set1_epi32(-*cospi.offset(56));
    let mut cospi_m40: __m256i = _mm256_set1_epi32(-*cospi.offset(40));
    let mut cospi_p24: __m256i = _mm256_set1_epi32(*cospi.offset(24));
    let mut cospi_m24: __m256i = _mm256_set1_epi32(-*cospi.offset(24));
    let mut cospi_p08: __m256i = _mm256_set1_epi32(*cospi.offset(8));
    let mut cospi_p40: __m256i = _mm256_set1_epi32(*cospi.offset(40));
    let mut cospi_p60: __m256i = _mm256_set1_epi32(*cospi.offset(60));
    let mut cospi_p04: __m256i = _mm256_set1_epi32(*cospi.offset(4));
    let mut cospi_p28: __m256i = _mm256_set1_epi32(*cospi.offset(28));
    let mut cospi_p36: __m256i = _mm256_set1_epi32(*cospi.offset(36));
    let mut cospi_p44: __m256i = _mm256_set1_epi32(*cospi.offset(44));
    let mut cospi_p20: __m256i = _mm256_set1_epi32(*cospi.offset(20));
    let mut cospi_p12: __m256i = _mm256_set1_epi32(*cospi.offset(12));
    let mut cospi_p52: __m256i = _mm256_set1_epi32(*cospi.offset(52));
    let mut cospi_m04: __m256i = _mm256_set1_epi32(-*cospi.offset(4));
    let mut cospi_m60: __m256i = _mm256_set1_epi32(-*cospi.offset(60));
    let mut cospi_m36: __m256i = _mm256_set1_epi32(-*cospi.offset(36));
    let mut cospi_m28: __m256i = _mm256_set1_epi32(-*cospi.offset(28));
    let mut cospi_m20: __m256i = _mm256_set1_epi32(-*cospi.offset(20));
    let mut cospi_m44: __m256i = _mm256_set1_epi32(-*cospi.offset(44));
    let mut cospi_m52: __m256i = _mm256_set1_epi32(-*cospi.offset(52));
    let mut cospi_m12: __m256i = _mm256_set1_epi32(-*cospi.offset(12));
    let mut cospi_p62: __m256i = _mm256_set1_epi32(*cospi.offset(62));
    let mut cospi_p02: __m256i = _mm256_set1_epi32(*cospi.offset(2));
    let mut cospi_p30: __m256i = _mm256_set1_epi32(*cospi.offset(30));
    let mut cospi_p34: __m256i = _mm256_set1_epi32(*cospi.offset(34));
    let mut cospi_p46: __m256i = _mm256_set1_epi32(*cospi.offset(46));
    let mut cospi_p18: __m256i = _mm256_set1_epi32(*cospi.offset(18));
    let mut cospi_p14: __m256i = _mm256_set1_epi32(*cospi.offset(14));
    let mut cospi_p50: __m256i = _mm256_set1_epi32(*cospi.offset(50));
    let mut cospi_p54: __m256i = _mm256_set1_epi32(*cospi.offset(54));
    let mut cospi_p10: __m256i = _mm256_set1_epi32(*cospi.offset(10));
    let mut cospi_p22: __m256i = _mm256_set1_epi32(*cospi.offset(22));
    let mut cospi_p42: __m256i = _mm256_set1_epi32(*cospi.offset(42));
    let mut cospi_p38: __m256i = _mm256_set1_epi32(*cospi.offset(38));
    let mut cospi_p26: __m256i = _mm256_set1_epi32(*cospi.offset(26));
    let mut cospi_p06: __m256i = _mm256_set1_epi32(*cospi.offset(6));
    let mut cospi_p58: __m256i = _mm256_set1_epi32(*cospi.offset(58));
    let mut cospi_p63: __m256i = _mm256_set1_epi32(*cospi.offset(63));
    let mut cospi_p01: __m256i = _mm256_set1_epi32(*cospi.offset(1));
    let mut cospi_p31: __m256i = _mm256_set1_epi32(*cospi.offset(31));
    let mut cospi_p33: __m256i = _mm256_set1_epi32(*cospi.offset(33));
    let mut cospi_p47: __m256i = _mm256_set1_epi32(*cospi.offset(47));
    let mut cospi_p17: __m256i = _mm256_set1_epi32(*cospi.offset(17));
    let mut cospi_p15: __m256i = _mm256_set1_epi32(*cospi.offset(15));
    let mut cospi_p49: __m256i = _mm256_set1_epi32(*cospi.offset(49));
    let mut cospi_p55: __m256i = _mm256_set1_epi32(*cospi.offset(55));
    let mut cospi_p09: __m256i = _mm256_set1_epi32(*cospi.offset(9));
    let mut cospi_p23: __m256i = _mm256_set1_epi32(*cospi.offset(23));
    let mut cospi_p41: __m256i = _mm256_set1_epi32(*cospi.offset(41));
    let mut cospi_p39: __m256i = _mm256_set1_epi32(*cospi.offset(39));
    let mut cospi_p25: __m256i = _mm256_set1_epi32(*cospi.offset(25));
    let mut cospi_p07: __m256i = _mm256_set1_epi32(*cospi.offset(7));
    let mut cospi_p57: __m256i = _mm256_set1_epi32(*cospi.offset(57));
    let mut cospi_p59: __m256i = _mm256_set1_epi32(*cospi.offset(59));
    let mut cospi_p05: __m256i = _mm256_set1_epi32(*cospi.offset(5));
    let mut cospi_p27: __m256i = _mm256_set1_epi32(*cospi.offset(27));
    let mut cospi_p37: __m256i = _mm256_set1_epi32(*cospi.offset(37));
    let mut cospi_p43: __m256i = _mm256_set1_epi32(*cospi.offset(43));
    let mut cospi_p21: __m256i = _mm256_set1_epi32(*cospi.offset(21));
    let mut cospi_p11: __m256i = _mm256_set1_epi32(*cospi.offset(11));
    let mut cospi_p53: __m256i = _mm256_set1_epi32(*cospi.offset(53));
    let mut cospi_p51: __m256i = _mm256_set1_epi32(*cospi.offset(51));
    let mut cospi_p13: __m256i = _mm256_set1_epi32(*cospi.offset(13));
    let mut cospi_p19: __m256i = _mm256_set1_epi32(*cospi.offset(19));
    let mut cospi_p45: __m256i = _mm256_set1_epi32(*cospi.offset(45));
    let mut cospi_p35: __m256i = _mm256_set1_epi32(*cospi.offset(35));
    let mut cospi_p29: __m256i = _mm256_set1_epi32(*cospi.offset(29));
    let mut cospi_p03: __m256i = _mm256_set1_epi32(*cospi.offset(3));
    let mut cospi_p61: __m256i = _mm256_set1_epi32(*cospi.offset(61));
    // stage 1
    let mut x1: [__m256i; 64] = core::mem::zeroed();
    btf_32_add_sub_out_avx2(
        x1.as_mut_ptr(),
        x1.as_mut_ptr().offset(63),
        *input,
        *input.offset(63),
    );
    btf_32_add_sub_out_avx2(
        x1.as_mut_ptr().offset(1),
        x1.as_mut_ptr().offset(62),
        *input.offset(1),
        *input.offset(62),
    );
    btf_32_add_sub_out_avx2(
        x1.as_mut_ptr().offset(2),
        x1.as_mut_ptr().offset(61),
        *input.offset(2),
        *input.offset(61),
    );
    btf_32_add_sub_out_avx2(
        x1.as_mut_ptr().offset(3),
        x1.as_mut_ptr().offset(60),
        *input.offset(3),
        *input.offset(60),
    );
    btf_32_add_sub_out_avx2(
        x1.as_mut_ptr().offset(4),
        x1.as_mut_ptr().offset(59),
        *input.offset(4),
        *input.offset(59),
    );
    btf_32_add_sub_out_avx2(
        x1.as_mut_ptr().offset(5),
        x1.as_mut_ptr().offset(58),
        *input.offset(5),
        *input.offset(58),
    );
    btf_32_add_sub_out_avx2(
        x1.as_mut_ptr().offset(6),
        x1.as_mut_ptr().offset(57),
        *input.offset(6),
        *input.offset(57),
    );
    btf_32_add_sub_out_avx2(
        x1.as_mut_ptr().offset(7),
        x1.as_mut_ptr().offset(56),
        *input.offset(7),
        *input.offset(56),
    );
    btf_32_add_sub_out_avx2(
        x1.as_mut_ptr().offset(8),
        x1.as_mut_ptr().offset(55),
        *input.offset(8),
        *input.offset(55),
    );
    btf_32_add_sub_out_avx2(
        x1.as_mut_ptr().offset(9),
        x1.as_mut_ptr().offset(54),
        *input.offset(9),
        *input.offset(54),
    );
    btf_32_add_sub_out_avx2(
        x1.as_mut_ptr().offset(10),
        x1.as_mut_ptr().offset(53),
        *input.offset(10),
        *input.offset(53),
    );
    btf_32_add_sub_out_avx2(
        x1.as_mut_ptr().offset(11),
        x1.as_mut_ptr().offset(52),
        *input.offset(11),
        *input.offset(52),
    );
    btf_32_add_sub_out_avx2(
        x1.as_mut_ptr().offset(12),
        x1.as_mut_ptr().offset(51),
        *input.offset(12),
        *input.offset(51),
    );
    btf_32_add_sub_out_avx2(
        x1.as_mut_ptr().offset(13),
        x1.as_mut_ptr().offset(50),
        *input.offset(13),
        *input.offset(50),
    );
    btf_32_add_sub_out_avx2(
        x1.as_mut_ptr().offset(14),
        x1.as_mut_ptr().offset(49),
        *input.offset(14),
        *input.offset(49),
    );
    btf_32_add_sub_out_avx2(
        x1.as_mut_ptr().offset(15),
        x1.as_mut_ptr().offset(48),
        *input.offset(15),
        *input.offset(48),
    );
    btf_32_add_sub_out_avx2(
        x1.as_mut_ptr().offset(16),
        x1.as_mut_ptr().offset(47),
        *input.offset(16),
        *input.offset(47),
    );
    btf_32_add_sub_out_avx2(
        x1.as_mut_ptr().offset(17),
        x1.as_mut_ptr().offset(46),
        *input.offset(17),
        *input.offset(46),
    );
    btf_32_add_sub_out_avx2(
        x1.as_mut_ptr().offset(18),
        x1.as_mut_ptr().offset(45),
        *input.offset(18),
        *input.offset(45),
    );
    btf_32_add_sub_out_avx2(
        x1.as_mut_ptr().offset(19),
        x1.as_mut_ptr().offset(44),
        *input.offset(19),
        *input.offset(44),
    );
    btf_32_add_sub_out_avx2(
        x1.as_mut_ptr().offset(20),
        x1.as_mut_ptr().offset(43),
        *input.offset(20),
        *input.offset(43),
    );
    btf_32_add_sub_out_avx2(
        x1.as_mut_ptr().offset(21),
        x1.as_mut_ptr().offset(42),
        *input.offset(21),
        *input.offset(42),
    );
    btf_32_add_sub_out_avx2(
        x1.as_mut_ptr().offset(22),
        x1.as_mut_ptr().offset(41),
        *input.offset(22),
        *input.offset(41),
    );
    btf_32_add_sub_out_avx2(
        x1.as_mut_ptr().offset(23),
        x1.as_mut_ptr().offset(40),
        *input.offset(23),
        *input.offset(40),
    );
    btf_32_add_sub_out_avx2(
        x1.as_mut_ptr().offset(24),
        x1.as_mut_ptr().offset(39),
        *input.offset(24),
        *input.offset(39),
    );
    btf_32_add_sub_out_avx2(
        x1.as_mut_ptr().offset(25),
        x1.as_mut_ptr().offset(38),
        *input.offset(25),
        *input.offset(38),
    );
    btf_32_add_sub_out_avx2(
        x1.as_mut_ptr().offset(26),
        x1.as_mut_ptr().offset(37),
        *input.offset(26),
        *input.offset(37),
    );
    btf_32_add_sub_out_avx2(
        x1.as_mut_ptr().offset(27),
        x1.as_mut_ptr().offset(36),
        *input.offset(27),
        *input.offset(36),
    );
    btf_32_add_sub_out_avx2(
        x1.as_mut_ptr().offset(28),
        x1.as_mut_ptr().offset(35),
        *input.offset(28),
        *input.offset(35),
    );
    btf_32_add_sub_out_avx2(
        x1.as_mut_ptr().offset(29),
        x1.as_mut_ptr().offset(34),
        *input.offset(29),
        *input.offset(34),
    );
    btf_32_add_sub_out_avx2(
        x1.as_mut_ptr().offset(30),
        x1.as_mut_ptr().offset(33),
        *input.offset(30),
        *input.offset(33),
    );
    btf_32_add_sub_out_avx2(
        x1.as_mut_ptr().offset(31),
        x1.as_mut_ptr().offset(32),
        *input.offset(31),
        *input.offset(32),
    );
    // stage 2
    btf_32_add_sub_avx2(x1.as_mut_ptr(), x1.as_mut_ptr().offset(31));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(1), x1.as_mut_ptr().offset(30));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(2), x1.as_mut_ptr().offset(29));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(3), x1.as_mut_ptr().offset(28));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(4), x1.as_mut_ptr().offset(27));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(5), x1.as_mut_ptr().offset(26));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(6), x1.as_mut_ptr().offset(25));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(7), x1.as_mut_ptr().offset(24));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(8), x1.as_mut_ptr().offset(23));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(9), x1.as_mut_ptr().offset(22));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(10), x1.as_mut_ptr().offset(21));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(11), x1.as_mut_ptr().offset(20));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(12), x1.as_mut_ptr().offset(19));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(13), x1.as_mut_ptr().offset(18));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(14), x1.as_mut_ptr().offset(17));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(15), x1.as_mut_ptr().offset(16));
    btf_32_avx2_type0_new(
        cospi_m32,
        cospi_p32,
        x1.as_mut_ptr().offset(40),
        x1.as_mut_ptr().offset(55),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type0_new(
        cospi_m32,
        cospi_p32,
        x1.as_mut_ptr().offset(41),
        x1.as_mut_ptr().offset(54),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type0_new(
        cospi_m32,
        cospi_p32,
        x1.as_mut_ptr().offset(42),
        x1.as_mut_ptr().offset(53),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type0_new(
        cospi_m32,
        cospi_p32,
        x1.as_mut_ptr().offset(43),
        x1.as_mut_ptr().offset(52),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type0_new(
        cospi_m32,
        cospi_p32,
        x1.as_mut_ptr().offset(44),
        x1.as_mut_ptr().offset(51),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type0_new(
        cospi_m32,
        cospi_p32,
        x1.as_mut_ptr().offset(45),
        x1.as_mut_ptr().offset(50),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type0_new(
        cospi_m32,
        cospi_p32,
        x1.as_mut_ptr().offset(46),
        x1.as_mut_ptr().offset(49),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type0_new(
        cospi_m32,
        cospi_p32,
        x1.as_mut_ptr().offset(47),
        x1.as_mut_ptr().offset(48),
        _r,
        cos_bit as i32,
    );
    // stage 3
    btf_32_add_sub_avx2(x1.as_mut_ptr(), x1.as_mut_ptr().offset(15));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(1), x1.as_mut_ptr().offset(14));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(2), x1.as_mut_ptr().offset(13));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(3), x1.as_mut_ptr().offset(12));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(4), x1.as_mut_ptr().offset(11));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(5), x1.as_mut_ptr().offset(10));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(6), x1.as_mut_ptr().offset(9));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(7), x1.as_mut_ptr().offset(8));
    btf_32_avx2_type0_new(
        cospi_m32,
        cospi_p32,
        x1.as_mut_ptr().offset(20),
        x1.as_mut_ptr().offset(27),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type0_new(
        cospi_m32,
        cospi_p32,
        x1.as_mut_ptr().offset(21),
        x1.as_mut_ptr().offset(26),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type0_new(
        cospi_m32,
        cospi_p32,
        x1.as_mut_ptr().offset(22),
        x1.as_mut_ptr().offset(25),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type0_new(
        cospi_m32,
        cospi_p32,
        x1.as_mut_ptr().offset(23),
        x1.as_mut_ptr().offset(24),
        _r,
        cos_bit as i32,
    );
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(32), x1.as_mut_ptr().offset(47));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(33), x1.as_mut_ptr().offset(46));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(34), x1.as_mut_ptr().offset(45));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(35), x1.as_mut_ptr().offset(44));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(36), x1.as_mut_ptr().offset(43));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(37), x1.as_mut_ptr().offset(42));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(38), x1.as_mut_ptr().offset(41));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(39), x1.as_mut_ptr().offset(40));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(63), x1.as_mut_ptr().offset(48));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(62), x1.as_mut_ptr().offset(49));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(61), x1.as_mut_ptr().offset(50));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(60), x1.as_mut_ptr().offset(51));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(59), x1.as_mut_ptr().offset(52));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(58), x1.as_mut_ptr().offset(53));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(57), x1.as_mut_ptr().offset(54));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(56), x1.as_mut_ptr().offset(55));
    // stage 4
    btf_32_add_sub_avx2(x1.as_mut_ptr(), x1.as_mut_ptr().offset(7));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(1), x1.as_mut_ptr().offset(6));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(2), x1.as_mut_ptr().offset(5));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(3), x1.as_mut_ptr().offset(4));
    btf_32_avx2_type0_new(
        cospi_m32,
        cospi_p32,
        x1.as_mut_ptr().offset(10),
        x1.as_mut_ptr().offset(13),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type0_new(
        cospi_m32,
        cospi_p32,
        x1.as_mut_ptr().offset(11),
        x1.as_mut_ptr().offset(12),
        _r,
        cos_bit as i32,
    );
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(16), x1.as_mut_ptr().offset(23));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(17), x1.as_mut_ptr().offset(22));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(18), x1.as_mut_ptr().offset(21));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(19), x1.as_mut_ptr().offset(20));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(31), x1.as_mut_ptr().offset(24));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(30), x1.as_mut_ptr().offset(25));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(29), x1.as_mut_ptr().offset(26));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(28), x1.as_mut_ptr().offset(27));
    btf_32_avx2_type0_new(
        cospi_m16,
        cospi_p48,
        x1.as_mut_ptr().offset(36),
        x1.as_mut_ptr().offset(59),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type0_new(
        cospi_m16,
        cospi_p48,
        x1.as_mut_ptr().offset(37),
        x1.as_mut_ptr().offset(58),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type0_new(
        cospi_m16,
        cospi_p48,
        x1.as_mut_ptr().offset(38),
        x1.as_mut_ptr().offset(57),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type0_new(
        cospi_m16,
        cospi_p48,
        x1.as_mut_ptr().offset(39),
        x1.as_mut_ptr().offset(56),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type0_new(
        cospi_m48,
        cospi_m16,
        x1.as_mut_ptr().offset(40),
        x1.as_mut_ptr().offset(55),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type0_new(
        cospi_m48,
        cospi_m16,
        x1.as_mut_ptr().offset(41),
        x1.as_mut_ptr().offset(54),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type0_new(
        cospi_m48,
        cospi_m16,
        x1.as_mut_ptr().offset(42),
        x1.as_mut_ptr().offset(53),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type0_new(
        cospi_m48,
        cospi_m16,
        x1.as_mut_ptr().offset(43),
        x1.as_mut_ptr().offset(52),
        _r,
        cos_bit as i32,
    );
    // stage 5
    btf_32_add_sub_avx2(x1.as_mut_ptr(), x1.as_mut_ptr().offset(3));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(1), x1.as_mut_ptr().offset(2));
    btf_32_avx2_type0_new(
        cospi_m32,
        cospi_p32,
        x1.as_mut_ptr().offset(5),
        x1.as_mut_ptr().offset(6),
        _r,
        cos_bit as i32,
    );
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(8), x1.as_mut_ptr().offset(11));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(9), x1.as_mut_ptr().offset(10));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(15), x1.as_mut_ptr().offset(12));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(14), x1.as_mut_ptr().offset(13));
    btf_32_avx2_type0_new(
        cospi_m16,
        cospi_p48,
        x1.as_mut_ptr().offset(18),
        x1.as_mut_ptr().offset(29),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type0_new(
        cospi_m16,
        cospi_p48,
        x1.as_mut_ptr().offset(19),
        x1.as_mut_ptr().offset(28),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type0_new(
        cospi_m48,
        cospi_m16,
        x1.as_mut_ptr().offset(20),
        x1.as_mut_ptr().offset(27),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type0_new(
        cospi_m48,
        cospi_m16,
        x1.as_mut_ptr().offset(21),
        x1.as_mut_ptr().offset(26),
        _r,
        cos_bit as i32,
    );
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(32), x1.as_mut_ptr().offset(39));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(33), x1.as_mut_ptr().offset(38));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(34), x1.as_mut_ptr().offset(37));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(35), x1.as_mut_ptr().offset(36));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(47), x1.as_mut_ptr().offset(40));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(46), x1.as_mut_ptr().offset(41));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(45), x1.as_mut_ptr().offset(42));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(44), x1.as_mut_ptr().offset(43));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(48), x1.as_mut_ptr().offset(55));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(49), x1.as_mut_ptr().offset(54));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(50), x1.as_mut_ptr().offset(53));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(51), x1.as_mut_ptr().offset(52));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(63), x1.as_mut_ptr().offset(56));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(62), x1.as_mut_ptr().offset(57));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(61), x1.as_mut_ptr().offset(58));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(60), x1.as_mut_ptr().offset(59));
    // stage 6
    btf_32_avx2_type0_new(
        cospi_p32,
        cospi_p32,
        x1.as_mut_ptr(),
        x1.as_mut_ptr().offset(1),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type1_new(
        cospi_p48,
        cospi_p16,
        x1.as_mut_ptr().offset(2),
        x1.as_mut_ptr().offset(3),
        _r,
        cos_bit as i32,
    );
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(4), x1.as_mut_ptr().offset(5));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(7), x1.as_mut_ptr().offset(6));
    btf_32_avx2_type0_new(
        cospi_m16,
        cospi_p48,
        x1.as_mut_ptr().offset(9),
        x1.as_mut_ptr().offset(14),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type0_new(
        cospi_m48,
        cospi_m16,
        x1.as_mut_ptr().offset(10),
        x1.as_mut_ptr().offset(13),
        _r,
        cos_bit as i32,
    );
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(16), x1.as_mut_ptr().offset(19));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(17), x1.as_mut_ptr().offset(18));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(23), x1.as_mut_ptr().offset(20));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(22), x1.as_mut_ptr().offset(21));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(24), x1.as_mut_ptr().offset(27));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(25), x1.as_mut_ptr().offset(26));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(31), x1.as_mut_ptr().offset(28));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(30), x1.as_mut_ptr().offset(29));
    btf_32_avx2_type0_new(
        cospi_m08,
        cospi_p56,
        x1.as_mut_ptr().offset(34),
        x1.as_mut_ptr().offset(61),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type0_new(
        cospi_m08,
        cospi_p56,
        x1.as_mut_ptr().offset(35),
        x1.as_mut_ptr().offset(60),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type0_new(
        cospi_m56,
        cospi_m08,
        x1.as_mut_ptr().offset(36),
        x1.as_mut_ptr().offset(59),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type0_new(
        cospi_m56,
        cospi_m08,
        x1.as_mut_ptr().offset(37),
        x1.as_mut_ptr().offset(58),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type0_new(
        cospi_m40,
        cospi_p24,
        x1.as_mut_ptr().offset(42),
        x1.as_mut_ptr().offset(53),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type0_new(
        cospi_m40,
        cospi_p24,
        x1.as_mut_ptr().offset(43),
        x1.as_mut_ptr().offset(52),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type0_new(
        cospi_m24,
        cospi_m40,
        x1.as_mut_ptr().offset(44),
        x1.as_mut_ptr().offset(51),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type0_new(
        cospi_m24,
        cospi_m40,
        x1.as_mut_ptr().offset(45),
        x1.as_mut_ptr().offset(50),
        _r,
        cos_bit as i32,
    );
    // stage 7
    btf_32_avx2_type1_new(
        cospi_p56,
        cospi_p08,
        x1.as_mut_ptr().offset(4),
        x1.as_mut_ptr().offset(7),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type1_new(
        cospi_p24,
        cospi_p40,
        x1.as_mut_ptr().offset(5),
        x1.as_mut_ptr().offset(6),
        _r,
        cos_bit as i32,
    );
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(8), x1.as_mut_ptr().offset(9));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(11), x1.as_mut_ptr().offset(10));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(12), x1.as_mut_ptr().offset(13));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(15), x1.as_mut_ptr().offset(14));
    btf_32_avx2_type0_new(
        cospi_m08,
        cospi_p56,
        x1.as_mut_ptr().offset(17),
        x1.as_mut_ptr().offset(30),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type0_new(
        cospi_m56,
        cospi_m08,
        x1.as_mut_ptr().offset(18),
        x1.as_mut_ptr().offset(29),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type0_new(
        cospi_m40,
        cospi_p24,
        x1.as_mut_ptr().offset(21),
        x1.as_mut_ptr().offset(26),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type0_new(
        cospi_m24,
        cospi_m40,
        x1.as_mut_ptr().offset(22),
        x1.as_mut_ptr().offset(25),
        _r,
        cos_bit as i32,
    );
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(32), x1.as_mut_ptr().offset(35));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(33), x1.as_mut_ptr().offset(34));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(39), x1.as_mut_ptr().offset(36));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(38), x1.as_mut_ptr().offset(37));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(40), x1.as_mut_ptr().offset(43));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(41), x1.as_mut_ptr().offset(42));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(47), x1.as_mut_ptr().offset(44));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(46), x1.as_mut_ptr().offset(45));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(48), x1.as_mut_ptr().offset(51));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(49), x1.as_mut_ptr().offset(50));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(55), x1.as_mut_ptr().offset(52));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(54), x1.as_mut_ptr().offset(53));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(56), x1.as_mut_ptr().offset(59));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(57), x1.as_mut_ptr().offset(58));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(63), x1.as_mut_ptr().offset(60));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(62), x1.as_mut_ptr().offset(61));
    // stage 8
    btf_32_avx2_type1_new(
        cospi_p60,
        cospi_p04,
        x1.as_mut_ptr().offset(8),
        x1.as_mut_ptr().offset(15),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type1_new(
        cospi_p28,
        cospi_p36,
        x1.as_mut_ptr().offset(9),
        x1.as_mut_ptr().offset(14),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type1_new(
        cospi_p44,
        cospi_p20,
        x1.as_mut_ptr().offset(10),
        x1.as_mut_ptr().offset(13),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type1_new(
        cospi_p12,
        cospi_p52,
        x1.as_mut_ptr().offset(11),
        x1.as_mut_ptr().offset(12),
        _r,
        cos_bit as i32,
    );
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(16), x1.as_mut_ptr().offset(17));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(19), x1.as_mut_ptr().offset(18));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(20), x1.as_mut_ptr().offset(21));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(23), x1.as_mut_ptr().offset(22));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(24), x1.as_mut_ptr().offset(25));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(27), x1.as_mut_ptr().offset(26));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(28), x1.as_mut_ptr().offset(29));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(31), x1.as_mut_ptr().offset(30));
    btf_32_avx2_type0_new(
        cospi_m04,
        cospi_p60,
        x1.as_mut_ptr().offset(33),
        x1.as_mut_ptr().offset(62),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type0_new(
        cospi_m60,
        cospi_m04,
        x1.as_mut_ptr().offset(34),
        x1.as_mut_ptr().offset(61),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type0_new(
        cospi_m36,
        cospi_p28,
        x1.as_mut_ptr().offset(37),
        x1.as_mut_ptr().offset(58),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type0_new(
        cospi_m28,
        cospi_m36,
        x1.as_mut_ptr().offset(38),
        x1.as_mut_ptr().offset(57),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type0_new(
        cospi_m20,
        cospi_p44,
        x1.as_mut_ptr().offset(41),
        x1.as_mut_ptr().offset(54),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type0_new(
        cospi_m44,
        cospi_m20,
        x1.as_mut_ptr().offset(42),
        x1.as_mut_ptr().offset(53),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type0_new(
        cospi_m52,
        cospi_p12,
        x1.as_mut_ptr().offset(45),
        x1.as_mut_ptr().offset(50),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type0_new(
        cospi_m12,
        cospi_m52,
        x1.as_mut_ptr().offset(46),
        x1.as_mut_ptr().offset(49),
        _r,
        cos_bit as i32,
    );
    // stage 9
    btf_32_avx2_type1_new(
        cospi_p62,
        cospi_p02,
        x1.as_mut_ptr().offset(16),
        x1.as_mut_ptr().offset(31),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type1_new(
        cospi_p30,
        cospi_p34,
        x1.as_mut_ptr().offset(17),
        x1.as_mut_ptr().offset(30),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type1_new(
        cospi_p46,
        cospi_p18,
        x1.as_mut_ptr().offset(18),
        x1.as_mut_ptr().offset(29),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type1_new(
        cospi_p14,
        cospi_p50,
        x1.as_mut_ptr().offset(19),
        x1.as_mut_ptr().offset(28),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type1_new(
        cospi_p54,
        cospi_p10,
        x1.as_mut_ptr().offset(20),
        x1.as_mut_ptr().offset(27),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type1_new(
        cospi_p22,
        cospi_p42,
        x1.as_mut_ptr().offset(21),
        x1.as_mut_ptr().offset(26),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type1_new(
        cospi_p38,
        cospi_p26,
        x1.as_mut_ptr().offset(22),
        x1.as_mut_ptr().offset(25),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type1_new(
        cospi_p06,
        cospi_p58,
        x1.as_mut_ptr().offset(23),
        x1.as_mut_ptr().offset(24),
        _r,
        cos_bit as i32,
    );
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(32), x1.as_mut_ptr().offset(33));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(35), x1.as_mut_ptr().offset(34));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(36), x1.as_mut_ptr().offset(37));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(39), x1.as_mut_ptr().offset(38));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(40), x1.as_mut_ptr().offset(41));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(43), x1.as_mut_ptr().offset(42));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(44), x1.as_mut_ptr().offset(45));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(47), x1.as_mut_ptr().offset(46));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(48), x1.as_mut_ptr().offset(49));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(51), x1.as_mut_ptr().offset(50));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(52), x1.as_mut_ptr().offset(53));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(55), x1.as_mut_ptr().offset(54));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(56), x1.as_mut_ptr().offset(57));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(59), x1.as_mut_ptr().offset(58));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(60), x1.as_mut_ptr().offset(61));
    btf_32_add_sub_avx2(x1.as_mut_ptr().offset(63), x1.as_mut_ptr().offset(62));
    // stage 10
    btf_32_avx2_type1_new(
        cospi_p63,
        cospi_p01,
        x1.as_mut_ptr().offset(32),
        x1.as_mut_ptr().offset(63),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type1_new(
        cospi_p31,
        cospi_p33,
        x1.as_mut_ptr().offset(33),
        x1.as_mut_ptr().offset(62),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type1_new(
        cospi_p47,
        cospi_p17,
        x1.as_mut_ptr().offset(34),
        x1.as_mut_ptr().offset(61),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type1_new(
        cospi_p15,
        cospi_p49,
        x1.as_mut_ptr().offset(35),
        x1.as_mut_ptr().offset(60),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type1_new(
        cospi_p55,
        cospi_p09,
        x1.as_mut_ptr().offset(36),
        x1.as_mut_ptr().offset(59),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type1_new(
        cospi_p23,
        cospi_p41,
        x1.as_mut_ptr().offset(37),
        x1.as_mut_ptr().offset(58),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type1_new(
        cospi_p39,
        cospi_p25,
        x1.as_mut_ptr().offset(38),
        x1.as_mut_ptr().offset(57),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type1_new(
        cospi_p07,
        cospi_p57,
        x1.as_mut_ptr().offset(39),
        x1.as_mut_ptr().offset(56),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type1_new(
        cospi_p59,
        cospi_p05,
        x1.as_mut_ptr().offset(40),
        x1.as_mut_ptr().offset(55),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type1_new(
        cospi_p27,
        cospi_p37,
        x1.as_mut_ptr().offset(41),
        x1.as_mut_ptr().offset(54),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type1_new(
        cospi_p43,
        cospi_p21,
        x1.as_mut_ptr().offset(42),
        x1.as_mut_ptr().offset(53),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type1_new(
        cospi_p11,
        cospi_p53,
        x1.as_mut_ptr().offset(43),
        x1.as_mut_ptr().offset(52),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type1_new(
        cospi_p51,
        cospi_p13,
        x1.as_mut_ptr().offset(44),
        x1.as_mut_ptr().offset(51),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type1_new(
        cospi_p19,
        cospi_p45,
        x1.as_mut_ptr().offset(45),
        x1.as_mut_ptr().offset(50),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type1_new(
        cospi_p35,
        cospi_p29,
        x1.as_mut_ptr().offset(46),
        x1.as_mut_ptr().offset(49),
        _r,
        cos_bit as i32,
    );
    btf_32_avx2_type1_new(
        cospi_p03,
        cospi_p61,
        x1.as_mut_ptr().offset(47),
        x1.as_mut_ptr().offset(48),
        _r,
        cos_bit as i32,
    );
    // stage 11
    *output = x1[0];
    *output.offset(1) = x1[32];
    *output.offset(2) = x1[16];
    *output.offset(3) = x1[48];
    *output.offset(4) = x1[8];
    *output.offset(5) = x1[40];
    *output.offset(6) = x1[24];
    *output.offset(7) = x1[56];
    *output.offset(8) = x1[4];
    *output.offset(9) = x1[36];
    *output.offset(10) = x1[20];
    *output.offset(11) = x1[52];
    *output.offset(12) = x1[12];
    *output.offset(13) = x1[44];
    *output.offset(14) = x1[28];
    *output.offset(15) = x1[60];
    *output.offset(16) = x1[2];
    *output.offset(17) = x1[34];
    *output.offset(18) = x1[18];
    *output.offset(19) = x1[50];
    *output.offset(20) = x1[10];
    *output.offset(21) = x1[42];
    *output.offset(22) = x1[26];
    *output.offset(23) = x1[58];
    *output.offset(24) = x1[6];
    *output.offset(25) = x1[38];
    *output.offset(26) = x1[22];
    *output.offset(27) = x1[54];
    *output.offset(28) = x1[14];
    *output.offset(29) = x1[46];
    *output.offset(30) = x1[30];
    *output.offset(31) = x1[62];
    *output.offset(32) = x1[1];
    *output.offset(33) = x1[33];
    *output.offset(34) = x1[17];
    *output.offset(35) = x1[49];
    *output.offset(36) = x1[9];
    *output.offset(37) = x1[41];
    *output.offset(38) = x1[25];
    *output.offset(39) = x1[57];
    *output.offset(40) = x1[5];
    *output.offset(41) = x1[37];
    *output.offset(42) = x1[21];
    *output.offset(43) = x1[53];
    *output.offset(44) = x1[13];
    *output.offset(45) = x1[45];
    *output.offset(46) = x1[29];
    *output.offset(47) = x1[61];
    *output.offset(48) = x1[3];
    *output.offset(49) = x1[35];
    *output.offset(50) = x1[19];
    *output.offset(51) = x1[51];
    *output.offset(52) = x1[11];
    *output.offset(53) = x1[43];
    *output.offset(54) = x1[27];
    *output.offset(55) = x1[59];
    *output.offset(56) = x1[7];
    *output.offset(57) = x1[39];
    *output.offset(58) = x1[23];
    *output.offset(59) = x1[55];
    *output.offset(60) = x1[15];
    *output.offset(61) = x1[47];
    *output.offset(62) = x1[31];
    *output.offset(63) = x1[63];
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn fadst16x16_new_avx2(input: *const __m256i, output: *mut __m256i, cos_bit: i8) {
    let mut cospi: *const i32 = cospi_arr(cos_bit as i32);
    let __zero: __m256i = _mm256_setzero_si256();
    let _r: __m256i = _mm256_set1_epi32(1 << (cos_bit as i32 - 1));
    let mut cospi_p32_p32: __m256i =
        pair_set_w16_epi16(*cospi.offset(32) as i16, *cospi.offset(32) as i16);
    let mut cospi_p32_m32: __m256i =
        pair_set_w16_epi16(*cospi.offset(32) as i16, -*cospi.offset(32) as i16);
    let mut cospi_p16_p48: __m256i =
        pair_set_w16_epi16(*cospi.offset(16) as i16, *cospi.offset(48) as i16);
    let mut cospi_p48_m16: __m256i =
        pair_set_w16_epi16(*cospi.offset(48) as i16, -*cospi.offset(16) as i16);
    let mut cospi_m48_p16: __m256i =
        pair_set_w16_epi16(-*cospi.offset(48) as i16, *cospi.offset(16) as i16);
    let mut cospi_p08_p56: __m256i =
        pair_set_w16_epi16(*cospi.offset(8) as i16, *cospi.offset(56) as i16);
    let mut cospi_p56_m08: __m256i =
        pair_set_w16_epi16(*cospi.offset(56) as i16, -*cospi.offset(8) as i16);
    let mut cospi_p40_p24: __m256i =
        pair_set_w16_epi16(*cospi.offset(40) as i16, *cospi.offset(24) as i16);
    let mut cospi_p24_m40: __m256i =
        pair_set_w16_epi16(*cospi.offset(24) as i16, -*cospi.offset(40) as i16);
    let mut cospi_m56_p08: __m256i =
        pair_set_w16_epi16(-*cospi.offset(56) as i16, *cospi.offset(8) as i16);
    let mut cospi_m24_p40: __m256i =
        pair_set_w16_epi16(-*cospi.offset(24) as i16, *cospi.offset(40) as i16);
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
    x1[0] = *input;
    x1[1] = _mm256_subs_epi16(__zero, *input.offset(15));
    x1[2] = _mm256_subs_epi16(__zero, *input.offset(7));
    x1[3] = *input.offset(8);
    x1[4] = _mm256_subs_epi16(__zero, *input.offset(3));
    x1[5] = *input.offset(12);
    x1[6] = *input.offset(4);
    x1[7] = _mm256_subs_epi16(__zero, *input.offset(11));
    x1[8] = _mm256_subs_epi16(__zero, *input.offset(1));
    x1[9] = *input.offset(14);
    x1[10] = *input.offset(6);
    x1[11] = _mm256_subs_epi16(__zero, *input.offset(9));
    x1[12] = *input.offset(2);
    x1[13] = _mm256_subs_epi16(__zero, *input.offset(13));
    x1[14] = _mm256_subs_epi16(__zero, *input.offset(5));
    x1[15] = *input.offset(10);
    // stage 2
    btf_16_w16_avx2(
        cospi_p32_p32,
        cospi_p32_m32,
        x1.as_mut_ptr().offset(2),
        x1.as_mut_ptr().offset(3),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_p32_p32,
        cospi_p32_m32,
        x1.as_mut_ptr().offset(6),
        x1.as_mut_ptr().offset(7),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_p32_p32,
        cospi_p32_m32,
        x1.as_mut_ptr().offset(10),
        x1.as_mut_ptr().offset(11),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_p32_p32,
        cospi_p32_m32,
        x1.as_mut_ptr().offset(14),
        x1.as_mut_ptr().offset(15),
        _r,
        cos_bit as i32,
    );
    // stage 3
    btf_16_adds_subs_avx2(x1.as_mut_ptr(), x1.as_mut_ptr().offset(2));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(1), x1.as_mut_ptr().offset(3));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(4), x1.as_mut_ptr().offset(6));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(5), x1.as_mut_ptr().offset(7));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(8), x1.as_mut_ptr().offset(10));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(9), x1.as_mut_ptr().offset(11));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(12), x1.as_mut_ptr().offset(14));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(13), x1.as_mut_ptr().offset(15));
    // stage 4
    btf_16_w16_avx2(
        cospi_p16_p48,
        cospi_p48_m16,
        x1.as_mut_ptr().offset(4),
        x1.as_mut_ptr().offset(5),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m48_p16,
        cospi_p16_p48,
        x1.as_mut_ptr().offset(6),
        x1.as_mut_ptr().offset(7),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_p16_p48,
        cospi_p48_m16,
        x1.as_mut_ptr().offset(12),
        x1.as_mut_ptr().offset(13),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m48_p16,
        cospi_p16_p48,
        x1.as_mut_ptr().offset(14),
        x1.as_mut_ptr().offset(15),
        _r,
        cos_bit as i32,
    );
    // stage 5
    btf_16_adds_subs_avx2(x1.as_mut_ptr(), x1.as_mut_ptr().offset(4));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(1), x1.as_mut_ptr().offset(5));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(2), x1.as_mut_ptr().offset(6));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(3), x1.as_mut_ptr().offset(7));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(8), x1.as_mut_ptr().offset(12));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(9), x1.as_mut_ptr().offset(13));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(10), x1.as_mut_ptr().offset(14));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(11), x1.as_mut_ptr().offset(15));
    // stage 6
    btf_16_w16_avx2(
        cospi_p08_p56,
        cospi_p56_m08,
        x1.as_mut_ptr().offset(8),
        x1.as_mut_ptr().offset(9),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_p40_p24,
        cospi_p24_m40,
        x1.as_mut_ptr().offset(10),
        x1.as_mut_ptr().offset(11),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m56_p08,
        cospi_p08_p56,
        x1.as_mut_ptr().offset(12),
        x1.as_mut_ptr().offset(13),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_m24_p40,
        cospi_p40_p24,
        x1.as_mut_ptr().offset(14),
        x1.as_mut_ptr().offset(15),
        _r,
        cos_bit as i32,
    );
    // stage 7
    btf_16_adds_subs_avx2(x1.as_mut_ptr(), x1.as_mut_ptr().offset(8));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(1), x1.as_mut_ptr().offset(9));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(2), x1.as_mut_ptr().offset(10));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(3), x1.as_mut_ptr().offset(11));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(4), x1.as_mut_ptr().offset(12));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(5), x1.as_mut_ptr().offset(13));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(6), x1.as_mut_ptr().offset(14));
    btf_16_adds_subs_avx2(x1.as_mut_ptr().offset(7), x1.as_mut_ptr().offset(15));
    // stage 8
    btf_16_w16_avx2(
        cospi_p02_p62,
        cospi_p62_m02,
        x1.as_mut_ptr(),
        x1.as_mut_ptr().offset(1),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_p10_p54,
        cospi_p54_m10,
        x1.as_mut_ptr().offset(2),
        x1.as_mut_ptr().offset(3),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_p18_p46,
        cospi_p46_m18,
        x1.as_mut_ptr().offset(4),
        x1.as_mut_ptr().offset(5),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_p26_p38,
        cospi_p38_m26,
        x1.as_mut_ptr().offset(6),
        x1.as_mut_ptr().offset(7),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_p34_p30,
        cospi_p30_m34,
        x1.as_mut_ptr().offset(8),
        x1.as_mut_ptr().offset(9),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_p42_p22,
        cospi_p22_m42,
        x1.as_mut_ptr().offset(10),
        x1.as_mut_ptr().offset(11),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_p50_p14,
        cospi_p14_m50,
        x1.as_mut_ptr().offset(12),
        x1.as_mut_ptr().offset(13),
        _r,
        cos_bit as i32,
    );
    btf_16_w16_avx2(
        cospi_p58_p06,
        cospi_p06_m58,
        x1.as_mut_ptr().offset(14),
        x1.as_mut_ptr().offset(15),
        _r,
        cos_bit as i32,
    );
    // stage 9
    *output = x1[1];
    *output.offset(1) = x1[14];
    *output.offset(2) = x1[3];
    *output.offset(3) = x1[12];
    *output.offset(4) = x1[5];
    *output.offset(5) = x1[10];
    *output.offset(6) = x1[7];
    *output.offset(7) = x1[8];
    *output.offset(8) = x1[9];
    *output.offset(9) = x1[6];
    *output.offset(10) = x1[11];
    *output.offset(11) = x1[4];
    *output.offset(12) = x1[13];
    *output.offset(13) = x1[2];
    *output.offset(14) = x1[15];
    *output.offset(15) = x1[0];
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn fidentity16x16_new_avx2(input: *const __m256i, output: *mut __m256i, cos_bit: i8) {
    let _ = cos_bit;
    let one: __m256i = _mm256_set1_epi16(1);
    for i in 0..16 {
        let a_lo: __m256i = _mm256_unpacklo_epi16(*input.offset(i as isize), one);
        let a_hi: __m256i = _mm256_unpackhi_epi16(*input.offset(i as isize), one);
        let b_lo: __m256i = scale_round_avx2(a_lo, 2 * NewSqrt2);
        let b_hi: __m256i = scale_round_avx2(a_hi, 2 * NewSqrt2);
        *output.offset(i as isize) = _mm256_packs_epi32(b_lo, b_hi);
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn fidentity16x32_avx2(input: *const __m256i, output: *mut __m256i, cos_bit: i8) {
    let _ = cos_bit;
    for i in 0..32 {
        *output.offset(i as isize) = _mm256_slli_epi16::<{ 2 }>(*input.offset(i as isize));
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn store_output_32bit_w16(
    out: *mut i32,
    in1: *const __m256i,
    in2: *const __m256i,
    stride: i32,
    out_size: i32,
) {
    for i in 0..out_size {
        _mm256_store_si256(
            out.offset((stride * i) as isize) as *mut __m256i,
            *in1.offset(i as isize),
        );
        _mm256_store_si256(
            out.offset((stride * i) as isize).offset(8) as *mut __m256i,
            *in2.offset(i as isize),
        );
    }
}

// Store 8 16 bit values. Sign extend the values.
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn store_buffer_16bit_to_32bit_w16_avx2(
    in_: *const __m256i,
    mut out: *mut i32,
    stride: i32,
    out_size: i32,
) {
    for i in 0..out_size {
        _mm256_store_si256(
            out as *mut __m256i,
            _mm256_cvtepi16_epi32(_mm256_castsi256_si128(*in_.offset(i as isize))),
        );
        _mm256_store_si256(
            out.offset(8) as *mut __m256i,
            _mm256_cvtepi16_epi32(_mm256_extracti128_si256::<1>(*in_.offset(i as isize))),
        );
        out = out.offset(stride as isize);
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn store_rect_16bit_to_32bit_avx2(a: __m256i, b: *mut i32) {
    let one: __m256i = _mm256_set1_epi16(1);
    let a_reoder: __m256i = _mm256_permute4x64_epi64::<0xd8>(a);
    let a_lo: __m256i = _mm256_unpacklo_epi16(a_reoder, one);
    let a_hi: __m256i = _mm256_unpackhi_epi16(a_reoder, one);
    let b_lo: __m256i = scale_round_avx2(a_lo, NewSqrt2);
    let b_hi: __m256i = scale_round_avx2(a_hi, NewSqrt2);
    _mm256_store_si256(b as *mut __m256i, b_lo);
    _mm256_store_si256(b.offset(8) as *mut __m256i, b_hi);
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn store_rect_buffer_16bit_to_32bit_w16_avx2(
    in_: *const __m256i,
    out: *mut i32,
    stride: i32,
    out_size: i32,
) {
    for i in 0..out_size {
        store_rect_16bit_to_32bit_avx2(*in_.offset(i as isize), out.offset((i * stride) as isize));
    }
}

static col_txfm16x32_arr: [transform_1d_avx2; (TX_TYPES) as usize] = [
    Some(fdct16x32_avx2),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(fidentity16x32_avx2),
    Some(fdct16x32_avx2),
    Some(fidentity16x32_avx2),
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
static row_txfm16x32_arr: [transform_1d_avx2; (TX_TYPES) as usize] = [
    Some(fdct16x32_avx2),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(fidentity16x32_avx2),
    Some(fidentity16x32_avx2),
    Some(fdct16x32_avx2),
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
static col_txfm16x16_arr: [transform_1d_avx2; (TX_TYPES) as usize] = [
    Some(fdct16x16_new_avx2),
    Some(fadst16x16_new_avx2),
    Some(fdct16x16_new_avx2),
    Some(fadst16x16_new_avx2),
    Some(fadst16x16_new_avx2),
    Some(fdct16x16_new_avx2),
    Some(fadst16x16_new_avx2),
    Some(fadst16x16_new_avx2),
    Some(fadst16x16_new_avx2),
    Some(fidentity16x16_new_avx2),
    Some(fdct16x16_new_avx2),
    Some(fidentity16x16_new_avx2),
    Some(fadst16x16_new_avx2),
    Some(fidentity16x16_new_avx2),
    Some(fadst16x16_new_avx2),
    Some(fidentity16x16_new_avx2),
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
static row_txfm16x16_arr: [transform_1d_avx2; (TX_TYPES) as usize] = [
    Some(fdct16x16_new_avx2),
    Some(fdct16x16_new_avx2),
    Some(fadst16x16_new_avx2),
    Some(fadst16x16_new_avx2),
    Some(fdct16x16_new_avx2),
    Some(fadst16x16_new_avx2),
    Some(fadst16x16_new_avx2),
    Some(fadst16x16_new_avx2),
    Some(fadst16x16_new_avx2),
    Some(fidentity16x16_new_avx2),
    Some(fidentity16x16_new_avx2),
    Some(fdct16x16_new_avx2),
    Some(fidentity16x16_new_avx2),
    Some(fadst16x16_new_avx2),
    Some(fidentity16x16_new_avx2),
    Some(fadst16x16_new_avx2),
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
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn load_buffer_and_round_shift(in_: *const i16, stride: i32, out: *mut __m128i, bit: i32) {
    *out = _mm_load_si128(in_.offset((0 * stride) as isize) as *const __m128i);
    *out.offset(1) = _mm_load_si128(in_.offset((1 * stride) as isize) as *const __m128i);
    *out.offset(2) = _mm_load_si128(in_.offset((2 * stride) as isize) as *const __m128i);
    *out.offset(3) = _mm_load_si128(in_.offset((3 * stride) as isize) as *const __m128i);
    *out.offset(4) = _mm_load_si128(in_.offset((4 * stride) as isize) as *const __m128i);
    *out.offset(5) = _mm_load_si128(in_.offset((5 * stride) as isize) as *const __m128i);
    *out.offset(6) = _mm_load_si128(in_.offset((6 * stride) as isize) as *const __m128i);
    *out.offset(7) = _mm_load_si128(in_.offset((7 * stride) as isize) as *const __m128i);
    *out = _mm_sll_epi16(*out, _mm_cvtsi32_si128(bit));
    *out.offset(1) = _mm_sll_epi16(*out.offset(1), _mm_cvtsi32_si128(bit));
    *out.offset(2) = _mm_sll_epi16(*out.offset(2), _mm_cvtsi32_si128(bit));
    *out.offset(3) = _mm_sll_epi16(*out.offset(3), _mm_cvtsi32_si128(bit));
    *out.offset(4) = _mm_sll_epi16(*out.offset(4), _mm_cvtsi32_si128(bit));
    *out.offset(5) = _mm_sll_epi16(*out.offset(5), _mm_cvtsi32_si128(bit));
    *out.offset(6) = _mm_sll_epi16(*out.offset(6), _mm_cvtsi32_si128(bit));
    *out.offset(7) = _mm_sll_epi16(*out.offset(7), _mm_cvtsi32_si128(bit));
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn load_buffer_and_flip_round_shift(
    in_: *const i16,
    stride: i32,
    out: *mut __m128i,
    bit: i32,
) {
    *out.offset(7) = load_16bit_to_16bit(in_.offset((0 * stride) as isize));
    *out.offset(6) = load_16bit_to_16bit(in_.offset((1 * stride) as isize));
    *out.offset(5) = load_16bit_to_16bit(in_.offset((2 * stride) as isize));
    *out.offset(4) = load_16bit_to_16bit(in_.offset((3 * stride) as isize));
    *out.offset(3) = load_16bit_to_16bit(in_.offset((4 * stride) as isize));
    *out.offset(2) = load_16bit_to_16bit(in_.offset((5 * stride) as isize));
    *out.offset(1) = load_16bit_to_16bit(in_.offset((6 * stride) as isize));
    *out = load_16bit_to_16bit(in_.offset((7 * stride) as isize));
    *out.offset(7) = _mm_sll_epi16(*out.offset(7), _mm_cvtsi32_si128(bit));
    *out.offset(6) = _mm_sll_epi16(*out.offset(6), _mm_cvtsi32_si128(bit));
    *out.offset(5) = _mm_sll_epi16(*out.offset(5), _mm_cvtsi32_si128(bit));
    *out.offset(4) = _mm_sll_epi16(*out.offset(4), _mm_cvtsi32_si128(bit));
    *out.offset(3) = _mm_sll_epi16(*out.offset(3), _mm_cvtsi32_si128(bit));
    *out.offset(2) = _mm_sll_epi16(*out.offset(2), _mm_cvtsi32_si128(bit));
    *out.offset(1) = _mm_sll_epi16(*out.offset(1), _mm_cvtsi32_si128(bit));
    *out = _mm_sll_epi16(*out, _mm_cvtsi32_si128(bit));
}

// aa0:    00 10 01 11  02 12 03 13 | 40 50 41 51  42 52 43 53
// aa1:    04 14 05 15  06 16 07 17 | 44 54 45 55  46 56 47 57
// aa2:    20 30 21 31  22 32 23 33 | 60 70 61 71  62 72 63 73
// aa3:    24 34 25 35  26 36 27 37 | 64 74 65 75  66 76 67 77
// Unpack 32 bit elements resulting in:
// bb0: 00 10 20 30  01 11 21 31 | 40 50 60 70  41 51 61 71
// bb1: 02 12 22 32  03 13 23 33 | 42 52 62 72  43 53 63 73
// bb2: 04 14 24 34  05 15 25 35 | 44 54 64 74  45 55 65 75
// bb2: 06 16 26 36  07 17 27 37 | 46 56 66 76  47 57 67 77
// bb0: 00 10 20 30 40 50 60 70| 01 11 21 31 41 51 61 71
// bb1: 02 12 22 32 42 52 62 72| 03 13 23 33 43 53 63 73
// bb2: 04 14 24 34 44 54 64 74| 05 15 25 35 45 55 65 75
// bb2: 06 16 26 36 46 56 66 76| 07 17 27 37 47 57 67 77
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn transpose_round_shift_flip_8x8(in_: *mut __m128i, out: *mut __m128i, mut bit: i32) {
    let mut c0: __m256i;
    let mut c1: __m256i;
    let mut c2: __m256i;
    let mut c3: __m256i;
    bit = -bit;
    let rounding: __m256i = _mm256_set1_epi16((1 << (bit - 1)) as i16);
    let s04: __m256i = _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(*in_), *in_.offset(4));
    let s15: __m256i =
        _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(*in_.offset(1)), *in_.offset(5));
    let s26: __m256i =
        _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(*in_.offset(2)), *in_.offset(6));
    let s37: __m256i =
        _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(*in_.offset(3)), *in_.offset(7));
    let a0: __m256i = _mm256_adds_epi16(s04, rounding);
    let a1: __m256i = _mm256_adds_epi16(s15, rounding);
    let a2: __m256i = _mm256_adds_epi16(s26, rounding);
    let a3: __m256i = _mm256_adds_epi16(s37, rounding);
    // b0: 00 01 02 03  04 05 06 07 | 40 41 42 43  44 45 46 47
    // b1: 10 11 12 13  14 15 16 17 | 50 51 52 53  54 55 56 57
    // b2: 20 21 22 23  24 25 26 27 | 60 61 62 63  64 65 66 67
    // b3: 30 31 32 33  34 35 36 37 | 70 71 72 73  74 75 76 77
    let b0: __m256i = _mm256_sra_epi16(a0, _mm_cvtsi32_si128(bit));
    let b1: __m256i = _mm256_sra_epi16(a1, _mm_cvtsi32_si128(bit));
    let b2: __m256i = _mm256_sra_epi16(a2, _mm_cvtsi32_si128(bit));
    let b3: __m256i = _mm256_sra_epi16(a3, _mm_cvtsi32_si128(bit));
    TRANSPOSE_8X8_AVX2!(b0, b1, b2, b3, c0, c1, c2, c3);
    // Unpack 64 bit elements resulting in:
    // out[7]: 00 10 20 30  40 50 60 70
    // out[6]: 01 11 21 31  41 51 61 71
    // out[5]: 02 12 22 32  42 52 62 72
    // out[4]: 03 13 23 33  43 53 63 73
    // out[3]: 04 14 24 34  44 54 64 74
    // out[2]: 05 15 25 35  45 55 65 75
    // out[1]: 06 16 26 36  46 56 66 76
    // out[0]: 07 17 27 37  47 57 67 77
    *out.offset(7) = _mm256_castsi256_si128(c0);
    *out.offset(6) = _mm256_extractf128_si256::<1>(c0);
    *out.offset(5) = _mm256_castsi256_si128(c1);
    *out.offset(4) = _mm256_extractf128_si256::<1>(c1);
    *out.offset(3) = _mm256_castsi256_si128(c2);
    *out.offset(2) = _mm256_extractf128_si256::<1>(c2);
    *out.offset(1) = _mm256_castsi256_si128(c3);
    *out = _mm256_extractf128_si256::<1>(c3);
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn transpose_round_shift_8x8(in_: *mut __m128i, out: *mut __m128i, mut bit: i32) {
    let mut c0: __m256i;
    let mut c1: __m256i;
    let mut c2: __m256i;
    let mut c3: __m256i;
    bit = -bit;
    let rounding: __m256i = _mm256_set1_epi16((1 << (bit - 1)) as i16);
    let s04: __m256i = _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(*in_), *in_.offset(4));
    let s15: __m256i =
        _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(*in_.offset(1)), *in_.offset(5));
    let s26: __m256i =
        _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(*in_.offset(2)), *in_.offset(6));
    let s37: __m256i =
        _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(*in_.offset(3)), *in_.offset(7));
    let a0: __m256i = _mm256_adds_epi16(s04, rounding);
    let a1: __m256i = _mm256_adds_epi16(s15, rounding);
    let a2: __m256i = _mm256_adds_epi16(s26, rounding);
    let a3: __m256i = _mm256_adds_epi16(s37, rounding);
    // b0: 00 01 02 03  04 05 06 07 | 40 41 42 43  44 45 46 47
    // b1: 10 11 12 13  14 15 16 17 | 50 51 52 53  54 55 56 57
    // b2: 20 21 22 23  24 25 26 27 | 60 61 62 63  64 65 66 67
    // b3: 30 31 32 33  34 35 36 37 | 70 71 72 73  74 75 76 77
    let b0: __m256i = _mm256_sra_epi16(a0, _mm_cvtsi32_si128(bit));
    let b1: __m256i = _mm256_sra_epi16(a1, _mm_cvtsi32_si128(bit));
    let b2: __m256i = _mm256_sra_epi16(a2, _mm_cvtsi32_si128(bit));
    let b3: __m256i = _mm256_sra_epi16(a3, _mm_cvtsi32_si128(bit));
    TRANSPOSE_8X8_AVX2!(b0, b1, b2, b3, c0, c1, c2, c3);
    // Unpack 64 bit elements resulting in:
    // out[7]: 00 10 20 30  40 50 60 70
    // out[6]: 01 11 21 31  41 51 61 71
    // out[5]: 02 12 22 32  42 52 62 72
    // out[4]: 03 13 23 33  43 53 63 73
    // out[3]: 04 14 24 34  44 54 64 74
    // out[2]: 05 15 25 35  45 55 65 75
    // out[1]: 06 16 26 36  46 56 66 76
    // out[0]: 07 17 27 37  47 57 67 77
    *out = _mm256_castsi256_si128(c0);
    *out.offset(1) = _mm256_extractf128_si256::<1>(c0);
    *out.offset(2) = _mm256_castsi256_si128(c1);
    *out.offset(3) = _mm256_extractf128_si256::<1>(c1);
    *out.offset(4) = _mm256_castsi256_si128(c2);
    *out.offset(5) = _mm256_extractf128_si256::<1>(c2);
    *out.offset(6) = _mm256_castsi256_si128(c3);
    *out.offset(7) = _mm256_extractf128_si256::<1>(c3);
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn store_buffer_16bit_to_32bit_w8_avx2(
    in_: *const __m128i,
    out: *mut i32,
    stride: i32,
    out_size: i32,
) {
    for i in 0..out_size {
        _mm256_store_si256(
            out.offset((i * stride) as isize) as *mut __m256i,
            _mm256_cvtepi16_epi32(*in_.offset(i as isize)),
        );
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn av1_lowbd_fwd_txfm2d_8x8_avx2(
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
    let col_txfm: transform_1d_sse2 = col_txfm8x8_arr[tx_type as usize];
    let row_txfm: transform_1d_sse2 = row_txfm8x8_arr[tx_type as usize];
    let mut ud_flip: i32 = 0;
    let mut lr_flip: i32 = 0;
    get_flip_cfg(tx_type, &raw mut ud_flip, &raw mut lr_flip);
    // Condition to check shift bit is avoided while round shifting, by assuming
    // that shift[0] will always be positive.
    debug_assert!((*shift as i32) > 0);
    if ud_flip != 0 {
        load_buffer_and_flip_round_shift(input, stride, buf0.as_mut_ptr(), *shift as i32);
    } else {
        load_buffer_and_round_shift(input, stride, buf0.as_mut_ptr(), *shift as i32);
    }
    col_txfm.unwrap()(
        buf0.as_mut_ptr() as *const __m128i,
        buf0.as_mut_ptr(),
        cos_bit_col as i8,
    );
    // Condition to check shift bit is avoided while round shifting, by assuming
    // that shift[1] will always be negative.
    debug_assert!((*shift.offset(1) as i32) < 0);
    if lr_flip != 0 {
        transpose_round_shift_flip_8x8(
            buf0.as_mut_ptr(),
            buf1.as_mut_ptr(),
            *shift.offset(1) as i32,
        );
    } else {
        transpose_round_shift_8x8(
            buf0.as_mut_ptr(),
            buf1.as_mut_ptr(),
            *shift.offset(1) as i32,
        );
    }
    buf = buf1.as_mut_ptr();
    row_txfm.unwrap()(buf as *const __m128i, buf, cos_bit_row as i8);
    // Round and shift operation is avoided here as the shift bit is assumed to be
    // zero always.
    debug_assert!((*shift.offset(2) as i32) == 0);
    store_buffer_16bit_to_32bit_w8_avx2(buf as *const __m128i, output, 8, 8);
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn lowbd_fwd_txfm2d_16x16_avx2(
    input: *const i16,
    output: *mut i32,
    stride: i32,
    tx_type: u8,
    bd: i32,
) {
    let _ = bd;
    let tx_size: u8 = TX_16X16 as u8;
    let mut buf0: [__m256i; 16] = core::mem::zeroed();
    let mut buf1: [__m256i; 16] = core::mem::zeroed();
    let mut shift: *const i8 = av1_fwd_txfm_shift_ls[tx_size as usize];
    let txw_idx: i32 = get_txw_idx(tx_size);
    let txh_idx: i32 = get_txh_idx(tx_size);
    let cos_bit_col: i32 = av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32;
    let cos_bit_row: i32 = av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32;
    let width: i32 = tx_size_wide[tx_size as usize];
    let height: i32 = tx_size_high[tx_size as usize];
    let col_txfm: transform_1d_avx2 = col_txfm16x16_arr[tx_type as usize];
    let row_txfm: transform_1d_avx2 = row_txfm16x16_arr[tx_type as usize];
    let mut ud_flip: i32 = 0;
    let mut lr_flip: i32 = 0;
    get_flip_cfg(tx_type, &raw mut ud_flip, &raw mut lr_flip);
    let i: i32 = 0;
    if ud_flip != 0 {
        load_buffer_16bit_to_16bit_flip_avx2(
            input.offset((16 * i) as isize),
            stride,
            buf0.as_mut_ptr(),
            height,
        );
    } else {
        load_buffer_16bit_to_16bit_avx2(
            input.offset((16 * i) as isize),
            stride,
            buf0.as_mut_ptr(),
            height,
        );
    }
    round_shift_16bit_w16_avx2(buf0.as_mut_ptr(), height, *shift as i32);
    col_txfm.unwrap()(
        buf0.as_mut_ptr() as *const __m256i,
        buf0.as_mut_ptr(),
        cos_bit_col as i8,
    );
    round_shift_16bit_w16_avx2(buf0.as_mut_ptr(), height, *shift.offset(1) as i32);
    transpose_16bit_16x16_avx2(
        buf0.as_mut_ptr() as *const __m256i,
        buf1.as_mut_ptr()
            .offset((0 * width) as isize)
            .offset((16 * i) as isize),
    );
    let mut buf: *mut __m256i;
    if lr_flip != 0 {
        buf = buf0.as_mut_ptr();
        flip_buf_avx2(buf1.as_mut_ptr().offset((width * i) as isize), buf, width);
    } else {
        buf = buf1.as_mut_ptr().offset((width * i) as isize);
    }
    row_txfm.unwrap()(buf as *const __m256i, buf, cos_bit_row as i8);
    round_shift_16bit_w16_avx2(buf, width, *shift.offset(2) as i32);
    store_buffer_16bit_to_32bit_w16_avx2(
        buf as *const __m256i,
        output.offset((i * 16) as isize),
        height,
        width,
    );
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn lowbd_fwd_txfm2d_32x32_avx2(
    input: *const i16,
    output: *mut i32,
    stride: i32,
    tx_type: u8,
    bd: i32,
) {
    let _ = bd;
    let tx_size: u8 = TX_32X32 as u8;
    let mut buf0: [__m256i; 32] = core::mem::zeroed();
    let mut buf1: [__m256i; 128] = core::mem::zeroed();
    let mut shift: *const i8 = av1_fwd_txfm_shift_ls[tx_size as usize];
    let txw_idx: i32 = get_txw_idx(tx_size);
    let txh_idx: i32 = get_txh_idx(tx_size);
    let cos_bit_col: i32 = av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32;
    let cos_bit_row: i32 = av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32;
    let width: i32 = tx_size_wide[tx_size as usize];
    let height: i32 = tx_size_high[tx_size as usize];
    let col_txfm: transform_1d_avx2 = col_txfm16x32_arr[tx_type as usize];
    let row_txfm: transform_1d_avx2 = row_txfm16x32_arr[tx_type as usize];
    let mut ud_flip: i32 = 0;
    let mut lr_flip: i32 = 0;
    get_flip_cfg(tx_type, &raw mut ud_flip, &raw mut lr_flip);
    for i in 0..2 {
        if ud_flip != 0 {
            load_buffer_16bit_to_16bit_flip_avx2(
                input.offset((16 * i) as isize),
                stride,
                buf0.as_mut_ptr(),
                height,
            );
        } else {
            load_buffer_16bit_to_16bit_avx2(
                input.offset((16 * i) as isize),
                stride,
                buf0.as_mut_ptr(),
                height,
            );
        }
        round_shift_16bit_w16_avx2(buf0.as_mut_ptr(), height, *shift as i32);
        col_txfm.unwrap()(
            buf0.as_mut_ptr() as *const __m256i,
            buf0.as_mut_ptr(),
            cos_bit_col as i8,
        );
        round_shift_16bit_w16_avx2(buf0.as_mut_ptr(), height, *shift.offset(1) as i32);
        transpose_16bit_16x16_avx2(
            buf0.as_mut_ptr().offset((0 * 16) as isize) as *const __m256i,
            buf1.as_mut_ptr()
                .offset((0 * width) as isize)
                .offset((16 * i) as isize),
        );
        transpose_16bit_16x16_avx2(
            buf0.as_mut_ptr().offset((1 * 16) as isize) as *const __m256i,
            buf1.as_mut_ptr()
                .offset((1 * width) as isize)
                .offset((16 * i) as isize),
        );
    }
    for i in 0..2 {
        let mut buf: *mut __m256i;
        if lr_flip != 0 {
            buf = buf0.as_mut_ptr();
            flip_buf_avx2(buf1.as_mut_ptr().offset((width * i) as isize), buf, width);
        } else {
            buf = buf1.as_mut_ptr().offset((width * i) as isize);
        }
        row_txfm.unwrap()(buf as *const __m256i, buf, cos_bit_row as i8);
        round_shift_16bit_w16_avx2(buf, width, *shift.offset(2) as i32);
        store_buffer_16bit_to_32bit_w16_avx2(
            buf as *const __m256i,
            output.offset((i * 16) as isize),
            height,
            width,
        );
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn lowbd_fwd_txfm2d_64x64_avx2(
    input: *const i16,
    output: *mut i32,
    stride: i32,
    tx_type: u8,
    bd: i32,
) {
    let _ = bd;
    let _ = tx_type;
    debug_assert!((tx_type as i32) == DCT_DCT);
    let tx_size: u8 = TX_64X64 as u8;
    let mut buf0: [__m256i; 64] = core::mem::zeroed();
    let mut buf1: [__m256i; 256] = core::mem::zeroed();
    let mut shift: *const i8 = av1_fwd_txfm_shift_ls[tx_size as usize];
    let txw_idx: i32 = get_txw_idx(tx_size);
    let txh_idx: i32 = get_txh_idx(tx_size);
    let cos_bit_col: i32 = av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32;
    let cos_bit_row: i32 = av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32;
    let width: i32 = tx_size_wide[tx_size as usize];
    let height: i32 = tx_size_high[tx_size as usize];
    let col_txfm: transform_1d_avx2 = Some(fdct16x64_new_avx2);
    let width_div16: i32 = width >> 4;
    let height_div16: i32 = height >> 4;
    for i in 0..width_div16 {
        load_buffer_16bit_to_16bit_avx2(
            input.offset((16 * i) as isize),
            stride,
            buf0.as_mut_ptr(),
            height,
        );
        round_shift_16bit_w16_avx2(buf0.as_mut_ptr(), height, *shift as i32);
        col_txfm.unwrap()(
            buf0.as_mut_ptr() as *const __m256i,
            buf0.as_mut_ptr(),
            cos_bit_col as i8,
        );
        round_shift_16bit_w16_avx2(buf0.as_mut_ptr(), height, *shift.offset(1) as i32);
        {
            let mut j: i32 = 0;
            while j < AOMMIN(2, height_div16) {
                transpose_16bit_16x16_avx2(
                    buf0.as_mut_ptr().offset((j * 16) as isize) as *const __m256i,
                    buf1.as_mut_ptr()
                        .offset((j * width) as isize)
                        .offset((16 * i) as isize),
                );
                j += 1;
            }
        }
    }
    {
        let mut i: i32 = 0;
        while i < AOMMIN(2, height_div16) {
            let mut bufA: [__m256i; 64] = core::mem::zeroed();
            let mut bufB: [__m256i; 64] = core::mem::zeroed();
            let mut buf: *mut __m128i =
                buf1.as_mut_ptr().offset((width * i) as isize) as *mut __m128i;
            for j in 0..width {
                bufA[j as usize] = _mm256_cvtepi16_epi32(*buf.offset((j * 2) as isize));
                bufB[j as usize] = _mm256_cvtepi16_epi32(*buf.offset((j * 2 + 1) as isize));
            }
            fdct64_new_avx2(
                bufA.as_mut_ptr() as *const __m256i,
                bufA.as_mut_ptr(),
                cos_bit_row as i8,
            );
            fdct64_new_avx2(
                bufB.as_mut_ptr() as *const __m256i,
                bufB.as_mut_ptr(),
                cos_bit_row as i8,
            );
            round_shift_array_32_avx2(
                bufA.as_mut_ptr(),
                bufA.as_mut_ptr(),
                32,
                -(*shift.offset(2) as i32),
            );
            round_shift_array_32_avx2(
                bufB.as_mut_ptr(),
                bufB.as_mut_ptr(),
                32,
                -(*shift.offset(2) as i32),
            );
            store_output_32bit_w16(
                output.offset((i * 16) as isize),
                bufA.as_mut_ptr() as *const __m256i,
                bufB.as_mut_ptr() as *const __m256i,
                32,
                32,
            );
            i += 1;
        }
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn lowbd_fwd_txfm2d_16x32_avx2(
    input: *const i16,
    output: *mut i32,
    stride: i32,
    tx_type: u8,
    bd: i32,
) {
    let _ = bd;
    let tx_size: u8 = TX_16X32 as u8;
    let mut buf0: [__m256i; 32] = core::mem::zeroed();
    let mut buf1: [__m256i; 32] = core::mem::zeroed();
    let mut shift: *const i8 = av1_fwd_txfm_shift_ls[tx_size as usize];
    let txw_idx: i32 = get_txw_idx(tx_size);
    let txh_idx: i32 = get_txh_idx(tx_size);
    let cos_bit_col: i32 = av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32;
    let cos_bit_row: i32 = av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32;
    let width: i32 = tx_size_wide[tx_size as usize];
    let height: i32 = tx_size_high[tx_size as usize];
    let col_txfm: transform_1d_avx2 = col_txfm16x32_arr[tx_type as usize];
    let row_txfm: transform_1d_avx2 = row_txfm16x16_arr[tx_type as usize];
    let mut ud_flip: i32 = 0;
    let mut lr_flip: i32 = 0;
    get_flip_cfg(tx_type, &raw mut ud_flip, &raw mut lr_flip);
    if ud_flip != 0 {
        load_buffer_16bit_to_16bit_flip_avx2(input, stride, buf0.as_mut_ptr(), height);
    } else {
        load_buffer_16bit_to_16bit_avx2(input, stride, buf0.as_mut_ptr(), height);
    }
    round_shift_16bit_w16_avx2(buf0.as_mut_ptr(), height, *shift as i32);
    col_txfm.unwrap()(
        buf0.as_mut_ptr() as *const __m256i,
        buf0.as_mut_ptr(),
        cos_bit_col as i8,
    );
    round_shift_16bit_w16_avx2(buf0.as_mut_ptr(), height, *shift.offset(1) as i32);
    transpose_16bit_16x16_avx2(buf0.as_mut_ptr() as *const __m256i, buf1.as_mut_ptr());
    transpose_16bit_16x16_avx2(
        buf0.as_mut_ptr().offset(16) as *const __m256i,
        buf1.as_mut_ptr().offset(16),
    );
    for i in 0..2 {
        let mut buf: *mut __m256i;
        if lr_flip != 0 {
            buf = buf0.as_mut_ptr();
            flip_buf_avx2(buf1.as_mut_ptr().offset((width * i) as isize), buf, width);
        } else {
            buf = buf1.as_mut_ptr().offset((width * i) as isize);
        }
        row_txfm.unwrap()(buf as *const __m256i, buf, cos_bit_row as i8);
        round_shift_16bit_w16_avx2(buf, width, *shift.offset(2) as i32);
        store_rect_buffer_16bit_to_32bit_w16_avx2(
            buf as *const __m256i,
            output.offset((i * 16) as isize),
            height,
            width,
        );
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn lowbd_fwd_txfm2d_32x16_avx2(
    input: *const i16,
    output: *mut i32,
    stride: i32,
    tx_type: u8,
    bd: i32,
) {
    let _ = bd;
    let mut buf0: [__m256i; 32] = core::mem::zeroed();
    let mut buf1: [__m256i; 64] = core::mem::zeroed();
    let mut shift: *const i8 = av1_fwd_txfm_shift_ls[TX_32X16 as usize];
    let txw_idx: i32 = get_txw_idx(TX_32X16 as u8);
    let txh_idx: i32 = get_txh_idx(TX_32X16 as u8);
    let cos_bit_col: i32 = av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32;
    let cos_bit_row: i32 = av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32;
    let width: i32 = 32;
    let height: i32 = 16;
    let col_txfm: transform_1d_avx2 = col_txfm16x16_arr[tx_type as usize];
    let row_txfm: transform_1d_avx2 = row_txfm16x32_arr[tx_type as usize];
    let mut ud_flip: i32 = 0;
    let mut lr_flip: i32 = 0;
    get_flip_cfg(tx_type, &raw mut ud_flip, &raw mut lr_flip);
    for i in 0..2 {
        if ud_flip != 0 {
            load_buffer_16bit_to_16bit_flip_avx2(
                input.offset((16 * i) as isize),
                stride,
                buf0.as_mut_ptr(),
                height,
            );
        } else {
            load_buffer_16bit_to_16bit_avx2(
                input.offset((16 * i) as isize),
                stride,
                buf0.as_mut_ptr(),
                height,
            );
        }
        round_shift_16bit_w16_avx2(buf0.as_mut_ptr(), height, *shift as i32);
        col_txfm.unwrap()(
            buf0.as_mut_ptr() as *const __m256i,
            buf0.as_mut_ptr(),
            cos_bit_col as i8,
        );
        round_shift_16bit_w16_avx2(buf0.as_mut_ptr(), height, *shift.offset(1) as i32);
        transpose_16bit_16x16_avx2(
            buf0.as_mut_ptr() as *const __m256i,
            buf1.as_mut_ptr()
                .offset((0 * width) as isize)
                .offset((16 * i) as isize),
        );
    }
    let mut buf: *mut __m256i;
    if lr_flip != 0 {
        buf = buf0.as_mut_ptr();
        flip_buf_avx2(buf1.as_mut_ptr(), buf, width);
    } else {
        buf = buf1.as_mut_ptr();
    }
    row_txfm.unwrap()(buf as *const __m256i, buf, cos_bit_row as i8);
    round_shift_16bit_w16_avx2(buf, width, *shift.offset(2) as i32);
    store_rect_buffer_16bit_to_32bit_w16_avx2(buf as *const __m256i, output, height, width);
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn lowbd_fwd_txfm2d_64x32_avx2(
    input: *const i16,
    output: *mut i32,
    stride: i32,
    tx_type: u8,
    bd: i32,
) {
    let _ = bd;
    let tx_size: u8 = TX_64X32 as u8;
    let mut buf0: [__m256i; 64] = core::mem::zeroed();
    let mut buf1: [__m256i; 256] = core::mem::zeroed();
    let mut shift: *const i8 = av1_fwd_txfm_shift_ls[tx_size as usize];
    let txw_idx: i32 = get_txw_idx(tx_size);
    let txh_idx: i32 = get_txh_idx(tx_size);
    let cos_bit_col: i32 = av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32;
    let cos_bit_row: i32 = av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32;
    let width: i32 = tx_size_wide[tx_size as usize];
    let height: i32 = tx_size_high[tx_size as usize];
    let col_txfm: transform_1d_avx2 = col_txfm16x32_arr[tx_type as usize];
    let width_div16: i32 = width >> 4;
    let height_div16: i32 = height >> 4;
    for i in 0..width_div16 {
        load_buffer_16bit_to_16bit_avx2(
            input.offset((16 * i) as isize),
            stride,
            buf0.as_mut_ptr(),
            height,
        );
        round_shift_16bit_w16_avx2(buf0.as_mut_ptr(), height, *shift as i32);
        col_txfm.unwrap()(
            buf0.as_mut_ptr() as *const __m256i,
            buf0.as_mut_ptr(),
            cos_bit_col as i8,
        );
        round_shift_16bit_w16_avx2(buf0.as_mut_ptr(), height, *shift.offset(1) as i32);
        {
            let mut j: i32 = 0;
            while j < AOMMIN(4, height_div16) {
                transpose_16bit_16x16_avx2(
                    buf0.as_mut_ptr().offset((j * 16) as isize) as *const __m256i,
                    buf1.as_mut_ptr()
                        .offset((j * width) as isize)
                        .offset((16 * i) as isize),
                );
                j += 1;
            }
        }
    }
    debug_assert!((tx_type as i32) == DCT_DCT);
    {
        let mut i: i32 = 0;
        while i < AOMMIN(2, height_div16) {
            let mut bufA: [__m256i; 64] = core::mem::zeroed();
            let mut bufB: [__m256i; 64] = core::mem::zeroed();
            let mut buf: *mut __m128i =
                buf1.as_mut_ptr().offset((width * i) as isize) as *mut __m128i;
            for j in 0..width {
                bufA[j as usize] = _mm256_cvtepi16_epi32(*buf.offset((j * 2) as isize));
                bufB[j as usize] = _mm256_cvtepi16_epi32(*buf.offset((j * 2 + 1) as isize));
            }
            fdct64_new_avx2(
                bufA.as_mut_ptr() as *const __m256i,
                bufA.as_mut_ptr(),
                cos_bit_row as i8,
            );
            fdct64_new_avx2(
                bufB.as_mut_ptr() as *const __m256i,
                bufB.as_mut_ptr(),
                cos_bit_row as i8,
            );
            round_shift_rect_array_32_avx2(
                bufA.as_mut_ptr(),
                bufA.as_mut_ptr(),
                32,
                -(*shift.offset(2) as i32),
                NewSqrt2,
            );
            round_shift_rect_array_32_avx2(
                bufB.as_mut_ptr(),
                bufB.as_mut_ptr(),
                32,
                -(*shift.offset(2) as i32),
                NewSqrt2,
            );
            store_output_32bit_w16(
                output.offset((i * 16) as isize),
                bufA.as_mut_ptr() as *const __m256i,
                bufB.as_mut_ptr() as *const __m256i,
                32,
                32,
            );
            i += 1;
        }
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn lowbd_fwd_txfm2d_32x64_avx2(
    input: *const i16,
    output: *mut i32,
    stride: i32,
    tx_type: u8,
    bd: i32,
) {
    let _ = bd;
    let _ = tx_type;
    debug_assert!((tx_type as i32) == DCT_DCT);
    let tx_size: u8 = TX_32X64 as u8;
    let mut buf0: [__m256i; 64] = core::mem::zeroed();
    let mut buf1: [__m256i; 256] = core::mem::zeroed();
    let mut shift: *const i8 = av1_fwd_txfm_shift_ls[tx_size as usize];
    let txw_idx: i32 = get_txw_idx(tx_size);
    let txh_idx: i32 = get_txh_idx(tx_size);
    let cos_bit_col: i32 = av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32;
    let cos_bit_row: i32 = av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32;
    let width: i32 = tx_size_wide[tx_size as usize];
    let height: i32 = tx_size_high[tx_size as usize];
    let col_txfm: transform_1d_avx2 = Some(fdct16x64_new_avx2);
    let width_div16: i32 = width >> 4;
    let height_div16: i32 = height >> 4;
    for i in 0..width_div16 {
        load_buffer_16bit_to_16bit_avx2(
            input.offset((16 * i) as isize),
            stride,
            buf0.as_mut_ptr(),
            height,
        );
        round_shift_16bit_w16_avx2(buf0.as_mut_ptr(), height, *shift as i32);
        col_txfm.unwrap()(
            buf0.as_mut_ptr() as *const __m256i,
            buf0.as_mut_ptr(),
            cos_bit_col as i8,
        );
        round_shift_16bit_w16_avx2(buf0.as_mut_ptr(), height, *shift.offset(1) as i32);
        {
            let mut j: i32 = 0;
            while j < AOMMIN(2, height_div16) {
                transpose_16bit_16x16_avx2(
                    buf0.as_mut_ptr().offset((j * 16) as isize) as *const __m256i,
                    buf1.as_mut_ptr()
                        .offset((j * width) as isize)
                        .offset((16 * i) as isize),
                );
                j += 1;
            }
        }
    }
    {
        let mut i: i32 = 0;
        while i < AOMMIN(2, height_div16) {
            let mut bufA: [__m256i; 32] = core::mem::zeroed();
            let mut bufB: [__m256i; 32] = core::mem::zeroed();
            let mut buf: *mut __m128i =
                buf1.as_mut_ptr().offset((width * i) as isize) as *mut __m128i;
            for j in 0..width {
                bufA[j as usize] = _mm256_cvtepi16_epi32(*buf.offset((j * 2) as isize));
                bufB[j as usize] = _mm256_cvtepi16_epi32(*buf.offset((j * 2 + 1) as isize));
            }
            fdct32_avx2(
                bufA.as_mut_ptr() as *const __m256i,
                bufA.as_mut_ptr(),
                cos_bit_row as i8,
            );
            fdct32_avx2(
                bufB.as_mut_ptr() as *const __m256i,
                bufB.as_mut_ptr(),
                cos_bit_row as i8,
            );
            round_shift_rect_array_32_avx2(
                bufA.as_mut_ptr(),
                bufA.as_mut_ptr(),
                32,
                -(*shift.offset(2) as i32),
                NewSqrt2,
            );
            round_shift_rect_array_32_avx2(
                bufB.as_mut_ptr(),
                bufB.as_mut_ptr(),
                32,
                -(*shift.offset(2) as i32),
                NewSqrt2,
            );
            store_output_32bit_w16(
                output.offset((i * 16) as isize),
                bufA.as_mut_ptr() as *const __m256i,
                bufB.as_mut_ptr() as *const __m256i,
                32,
                32,
            );
            i += 1;
        }
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn lowbd_fwd_txfm2d_16x64_avx2(
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
    let mut buf0: [__m256i; 64] = core::mem::zeroed();
    let mut buf1: [__m256i; 64] = core::mem::zeroed();
    let mut shift: *const i8 = av1_fwd_txfm_shift_ls[tx_size as usize];
    let txw_idx: i32 = get_txw_idx(tx_size);
    let txh_idx: i32 = get_txh_idx(tx_size);
    let cos_bit_col: i32 = av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32;
    let cos_bit_row: i32 = av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32;
    let width: i32 = tx_size_wide[tx_size as usize];
    let height: i32 = tx_size_high[tx_size as usize];
    let col_txfm: transform_1d_avx2 = Some(fdct16x64_new_avx2);
    let row_txfm: transform_1d_avx2 = Some(fdct16x16_new_avx2);
    let width_div16: i32 = width >> 4;
    let height_div16: i32 = height >> 4;
    for i in 0..width_div16 {
        load_buffer_16bit_to_16bit_avx2(
            input.offset((16 * i) as isize),
            stride,
            buf0.as_mut_ptr(),
            height,
        );
        round_shift_16bit_w16_avx2(buf0.as_mut_ptr(), height, *shift as i32);
        col_txfm.unwrap()(
            buf0.as_mut_ptr() as *const __m256i,
            buf0.as_mut_ptr(),
            cos_bit_col as i8,
        );
        round_shift_16bit_w16_avx2(buf0.as_mut_ptr(), height, *shift.offset(1) as i32);
        for j in 0..height_div16 {
            transpose_16bit_16x16_avx2(
                buf0.as_mut_ptr().offset((j * 16) as isize) as *const __m256i,
                buf1.as_mut_ptr()
                    .offset((j * width) as isize)
                    .offset((16 * i) as isize),
            );
        }
    }
    {
        let mut i: i32 = 0;
        while i < AOMMIN(2, height_div16) {
            let mut buf: *mut __m256i = buf1.as_mut_ptr().offset((width * i) as isize);
            row_txfm.unwrap()(buf as *const __m256i, buf, cos_bit_row as i8);
            round_shift_16bit_w16_avx2(buf, width, *shift.offset(2) as i32);
            store_buffer_16bit_to_32bit_w16_avx2(
                buf as *const __m256i,
                output.offset((width * i) as isize),
                32,
                width,
            );
            i += 1;
        }
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn lowbd_fwd_txfm2d_64x16_avx2(
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
    let mut buf0: [__m256i; 64] = core::mem::zeroed();
    let mut buf1: [__m256i; 64] = core::mem::zeroed();
    let mut shift: *const i8 = av1_fwd_txfm_shift_ls[tx_size as usize];
    let txw_idx: i32 = get_txw_idx(tx_size);
    let txh_idx: i32 = get_txh_idx(tx_size);
    let cos_bit_col: i32 = av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32;
    let cos_bit_row: i32 = av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32;
    let width: i32 = tx_size_wide[tx_size as usize];
    let height: i32 = tx_size_high[tx_size as usize];
    let col_txfm: transform_1d_avx2 = Some(fdct16x16_new_avx2);
    let row_txfm: transform_1d_avx2 = Some(fdct16x64_new_avx2);
    let width_div16: i32 = width >> 4;
    let height_div16: i32 = height >> 4;
    for i in 0..width_div16 {
        load_buffer_16bit_to_16bit_avx2(
            input.offset((16 * i) as isize),
            stride,
            buf0.as_mut_ptr(),
            height,
        );
        round_shift_16bit_w16_avx2(buf0.as_mut_ptr(), height, *shift as i32);
        col_txfm.unwrap()(
            buf0.as_mut_ptr() as *const __m256i,
            buf0.as_mut_ptr(),
            cos_bit_col as i8,
        );
        round_shift_16bit_w16_avx2(buf0.as_mut_ptr(), height, *shift.offset(1) as i32);
        for j in 0..height_div16 {
            transpose_16bit_16x16_avx2(
                buf0.as_mut_ptr().offset((j * 16) as isize) as *const __m256i,
                buf1.as_mut_ptr()
                    .offset((j * width) as isize)
                    .offset((16 * i) as isize),
            );
        }
    }
    for i in 0..height_div16 {
        let mut buf: *mut __m256i = buf1.as_mut_ptr().offset((width * i) as isize);
        row_txfm.unwrap()(buf as *const __m256i, buf, cos_bit_row as i8);
        round_shift_16bit_w16_avx2(buf, width, *shift.offset(2) as i32);
        store_buffer_16bit_to_32bit_w16_avx2(
            buf as *const __m256i,
            output.offset((16 * i) as isize),
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

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn btf_16_avx2(
    w0: *mut __m256i,
    w1: *mut __m256i,
    in0: *mut __m256i,
    in1: *mut __m256i,
    out0: *mut __m128i,
    out1: *mut __m128i,
    out2: *mut __m128i,
    out3: *mut __m128i,
    __rounding: *const __m256i,
    cos_bit: *mut i8,
) {
    let mut t0: __m256i = _mm256_unpacklo_epi16(*in0, *in1);
    let mut t1: __m256i = _mm256_unpackhi_epi16(*in0, *in1);
    let mut u0: __m256i = _mm256_madd_epi16(t0, *w0);
    let mut u1: __m256i = _mm256_madd_epi16(t1, *w0);
    let mut v0: __m256i = _mm256_madd_epi16(t0, *w1);
    let mut v1: __m256i = _mm256_madd_epi16(t1, *w1);
    let mut a0: __m256i = _mm256_add_epi32(u0, *__rounding);
    let mut a1: __m256i = _mm256_add_epi32(u1, *__rounding);
    let mut b0: __m256i = _mm256_add_epi32(v0, *__rounding);
    let mut b1: __m256i = _mm256_add_epi32(v1, *__rounding);
    let mut c0: __m256i = _mm256_sra_epi32(a0, _mm_cvtsi32_si128(*cos_bit as i32));
    let mut c1: __m256i = _mm256_sra_epi32(a1, _mm_cvtsi32_si128(*cos_bit as i32));
    let mut d0: __m256i = _mm256_sra_epi32(b0, _mm_cvtsi32_si128(*cos_bit as i32));
    let mut d1: __m256i = _mm256_sra_epi32(b1, _mm_cvtsi32_si128(*cos_bit as i32));
    let mut temp0: __m256i = _mm256_packs_epi32(c0, c1);
    let mut temp1: __m256i = _mm256_packs_epi32(d0, d1);
    *out0 = _mm256_castsi256_si128(temp0);
    *out1 = _mm256_castsi256_si128(temp1);
    *out2 = _mm256_extracti128_si256::<0x01>(temp0);
    *out3 = _mm256_extracti128_si256::<0x01>(temp1);
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn fdct8x8_new_avx2(input: *const __m256i, output: *mut __m256i, cos_bit: i8) {
    let mut cospi: *const i32 = cospi_arr(cos_bit as i32);
    let __rounding: __m256i = _mm256_set1_epi32(1 << (cos_bit as i32 - 1));
    let mut cospi_m32_p32: __m256i =
        pair_set_w16_epi16(-*cospi.offset(32) as i16, *cospi.offset(32) as i16);
    let mut cospi_p32_p32: __m256i =
        pair_set_w16_epi16(*cospi.offset(32) as i16, *cospi.offset(32) as i16);
    let mut cospi_p32_m32: __m256i =
        pair_set_w16_epi16(*cospi.offset(32) as i16, -*cospi.offset(32) as i16);
    let mut cospi_p48_p16: __m256i =
        pair_set_w16_epi16(*cospi.offset(48) as i16, *cospi.offset(16) as i16);
    let mut cospi_m16_p48: __m256i =
        pair_set_w16_epi16(-*cospi.offset(16) as i16, *cospi.offset(48) as i16);
    let mut cospi_p56_p08: __m256i =
        pair_set_w16_epi16(*cospi.offset(56) as i16, *cospi.offset(8) as i16);
    let mut cospi_m08_p56: __m256i =
        pair_set_w16_epi16(-*cospi.offset(8) as i16, *cospi.offset(56) as i16);
    let mut cospi_p24_p40: __m256i =
        pair_set_w16_epi16(*cospi.offset(24) as i16, *cospi.offset(40) as i16);
    let mut cospi_m40_p24: __m256i =
        pair_set_w16_epi16(-*cospi.offset(40) as i16, *cospi.offset(24) as i16);
    // stage 1
    let mut x1: [__m256i; 8] = core::mem::zeroed();
    x1[0] = _mm256_adds_epi16(*input, *input.offset(7));
    x1[7] = _mm256_subs_epi16(*input, *input.offset(7));
    x1[1] = _mm256_adds_epi16(*input.offset(1), *input.offset(6));
    x1[6] = _mm256_subs_epi16(*input.offset(1), *input.offset(6));
    x1[2] = _mm256_adds_epi16(*input.offset(2), *input.offset(5));
    x1[5] = _mm256_subs_epi16(*input.offset(2), *input.offset(5));
    x1[3] = _mm256_adds_epi16(*input.offset(3), *input.offset(4));
    x1[4] = _mm256_subs_epi16(*input.offset(3), *input.offset(4));
    // stage 2
    let mut x2: [__m256i; 8] = core::mem::zeroed();
    x2[0] = _mm256_adds_epi16(x1[0], x1[3]);
    x2[3] = _mm256_subs_epi16(x1[0], x1[3]);
    x2[1] = _mm256_adds_epi16(x1[1], x1[2]);
    x2[2] = _mm256_subs_epi16(x1[1], x1[2]);
    x2[4] = x1[4];
    btf_16_w16_avx2(
        cospi_m32_p32,
        cospi_p32_p32,
        x1.as_mut_ptr().offset(5),
        x1.as_mut_ptr().offset(6),
        __rounding,
        cos_bit as i32,
    );
    x2[5] = x1[5];
    x2[6] = x1[6];
    x2[7] = x1[7];
    // stage 3
    let mut x3: [__m256i; 8] = core::mem::zeroed();
    btf_16_w16_avx2(
        cospi_p32_p32,
        cospi_p32_m32,
        x2.as_mut_ptr(),
        x2.as_mut_ptr().offset(1),
        __rounding,
        cos_bit as i32,
    );
    x3[0] = x2[0];
    x3[1] = x2[1];
    btf_16_w16_avx2(
        cospi_p48_p16,
        cospi_m16_p48,
        x2.as_mut_ptr().offset(2),
        x2.as_mut_ptr().offset(3),
        __rounding,
        cos_bit as i32,
    );
    x3[2] = x2[2];
    x3[3] = x2[3];
    x3[4] = _mm256_adds_epi16(x2[4], x2[5]);
    x3[5] = _mm256_subs_epi16(x2[4], x2[5]);
    x3[6] = _mm256_subs_epi16(x2[7], x2[6]);
    x3[7] = _mm256_adds_epi16(x2[7], x2[6]);
    // stage 4
    let mut x4: [__m256i; 8] = core::mem::zeroed();
    x4[0] = x3[0];
    x4[1] = x3[1];
    x4[2] = x3[2];
    x4[3] = x3[3];
    btf_16_w16_avx2(
        cospi_p56_p08,
        cospi_m08_p56,
        x3.as_mut_ptr().offset(4),
        x3.as_mut_ptr().offset(7),
        __rounding,
        cos_bit as i32,
    );
    x4[4] = x3[4];
    x4[7] = x3[7];
    btf_16_w16_avx2(
        cospi_p24_p40,
        cospi_m40_p24,
        x3.as_mut_ptr().offset(5),
        x3.as_mut_ptr().offset(6),
        __rounding,
        cos_bit as i32,
    );
    x4[5] = x3[5];
    x4[6] = x3[6];
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
#[target_feature(enable = "avx2")]
unsafe fn fadst8x8_new_avx2(input: *const __m256i, output: *mut __m256i, cos_bit: i8) {
    let mut cospi: *const i32 = cospi_arr(cos_bit as i32);
    let __zero: __m256i = _mm256_setzero_si256();
    let __rounding: __m256i = _mm256_set1_epi32(1 << (cos_bit as i32 - 1));
    let mut cospi_p32_p32: __m256i =
        pair_set_w16_epi16(*cospi.offset(32) as i16, *cospi.offset(32) as i16);
    let mut cospi_p32_m32: __m256i =
        pair_set_w16_epi16(*cospi.offset(32) as i16, -*cospi.offset(32) as i16);
    let mut cospi_p16_p48: __m256i =
        pair_set_w16_epi16(*cospi.offset(16) as i16, *cospi.offset(48) as i16);
    let mut cospi_p48_m16: __m256i =
        pair_set_w16_epi16(*cospi.offset(48) as i16, -*cospi.offset(16) as i16);
    let mut cospi_m48_p16: __m256i =
        pair_set_w16_epi16(-*cospi.offset(48) as i16, *cospi.offset(16) as i16);
    let mut cospi_p04_p60: __m256i =
        pair_set_w16_epi16(*cospi.offset(4) as i16, *cospi.offset(60) as i16);
    let mut cospi_p60_m04: __m256i =
        pair_set_w16_epi16(*cospi.offset(60) as i16, -*cospi.offset(4) as i16);
    let mut cospi_p20_p44: __m256i =
        pair_set_w16_epi16(*cospi.offset(20) as i16, *cospi.offset(44) as i16);
    let mut cospi_p44_m20: __m256i =
        pair_set_w16_epi16(*cospi.offset(44) as i16, -*cospi.offset(20) as i16);
    let mut cospi_p36_p28: __m256i =
        pair_set_w16_epi16(*cospi.offset(36) as i16, *cospi.offset(28) as i16);
    let mut cospi_p28_m36: __m256i =
        pair_set_w16_epi16(*cospi.offset(28) as i16, -*cospi.offset(36) as i16);
    let mut cospi_p52_p12: __m256i =
        pair_set_w16_epi16(*cospi.offset(52) as i16, *cospi.offset(12) as i16);
    let mut cospi_p12_m52: __m256i =
        pair_set_w16_epi16(*cospi.offset(12) as i16, -*cospi.offset(52) as i16);
    // stage 1
    let mut x1: [__m256i; 8] = core::mem::zeroed();
    x1[0] = *input;
    x1[1] = _mm256_subs_epi16(__zero, *input.offset(7));
    x1[2] = _mm256_subs_epi16(__zero, *input.offset(3));
    x1[3] = *input.offset(4);
    x1[4] = _mm256_subs_epi16(__zero, *input.offset(1));
    x1[5] = *input.offset(6);
    x1[6] = *input.offset(2);
    x1[7] = _mm256_subs_epi16(__zero, *input.offset(5));
    // stage 2
    let mut x2: [__m256i; 8] = core::mem::zeroed();
    x2[0] = x1[0];
    x2[1] = x1[1];
    btf_16_w16_avx2(
        cospi_p32_p32,
        cospi_p32_m32,
        x1.as_mut_ptr().offset(2),
        x1.as_mut_ptr().offset(3),
        __rounding,
        cos_bit as i32,
    );
    x2[2] = x1[2];
    x2[3] = x1[3];
    x2[4] = x1[4];
    x2[5] = x1[5];
    btf_16_w16_avx2(
        cospi_p32_p32,
        cospi_p32_m32,
        x1.as_mut_ptr().offset(6),
        x1.as_mut_ptr().offset(7),
        __rounding,
        cos_bit as i32,
    );
    x2[6] = x1[6];
    x2[7] = x1[7];
    // stage 3
    let mut x3: [__m256i; 8] = core::mem::zeroed();
    x3[0] = _mm256_adds_epi16(x2[0], x2[2]);
    x3[2] = _mm256_subs_epi16(x2[0], x2[2]);
    x3[1] = _mm256_adds_epi16(x2[1], x2[3]);
    x3[3] = _mm256_subs_epi16(x2[1], x2[3]);
    x3[4] = _mm256_adds_epi16(x2[4], x2[6]);
    x3[6] = _mm256_subs_epi16(x2[4], x2[6]);
    x3[5] = _mm256_adds_epi16(x2[5], x2[7]);
    x3[7] = _mm256_subs_epi16(x2[5], x2[7]);
    // stage 4
    let mut x4: [__m256i; 8] = core::mem::zeroed();
    x4[0] = x3[0];
    x4[1] = x3[1];
    x4[2] = x3[2];
    x4[3] = x3[3];
    btf_16_w16_avx2(
        cospi_p16_p48,
        cospi_p48_m16,
        x3.as_mut_ptr().offset(4),
        x3.as_mut_ptr().offset(5),
        __rounding,
        cos_bit as i32,
    );
    x4[4] = x3[4];
    x4[5] = x3[5];
    btf_16_w16_avx2(
        cospi_m48_p16,
        cospi_p16_p48,
        x3.as_mut_ptr().offset(6),
        x3.as_mut_ptr().offset(7),
        __rounding,
        cos_bit as i32,
    );
    x4[6] = x3[6];
    x4[7] = x3[7];
    // stage 5
    let mut x5: [__m256i; 8] = core::mem::zeroed();
    x5[0] = _mm256_adds_epi16(x4[0], x4[4]);
    x5[4] = _mm256_subs_epi16(x4[0], x4[4]);
    x5[1] = _mm256_adds_epi16(x4[1], x4[5]);
    x5[5] = _mm256_subs_epi16(x4[1], x4[5]);
    x5[2] = _mm256_adds_epi16(x4[2], x4[6]);
    x5[6] = _mm256_subs_epi16(x4[2], x4[6]);
    x5[3] = _mm256_adds_epi16(x4[3], x4[7]);
    x5[7] = _mm256_subs_epi16(x4[3], x4[7]);
    // stage 6
    let mut x6: [__m256i; 8] = core::mem::zeroed();
    btf_16_w16_avx2(
        cospi_p04_p60,
        cospi_p60_m04,
        x5.as_mut_ptr(),
        x5.as_mut_ptr().offset(1),
        __rounding,
        cos_bit as i32,
    );
    x6[0] = x5[0];
    x6[1] = x5[1];
    btf_16_w16_avx2(
        cospi_p20_p44,
        cospi_p44_m20,
        x5.as_mut_ptr().offset(2),
        x5.as_mut_ptr().offset(3),
        __rounding,
        cos_bit as i32,
    );
    x6[2] = x5[2];
    x6[3] = x5[3];
    btf_16_w16_avx2(
        cospi_p36_p28,
        cospi_p28_m36,
        x5.as_mut_ptr().offset(4),
        x5.as_mut_ptr().offset(5),
        __rounding,
        cos_bit as i32,
    );
    x6[4] = x5[4];
    x6[5] = x5[5];
    btf_16_w16_avx2(
        cospi_p52_p12,
        cospi_p12_m52,
        x5.as_mut_ptr().offset(6),
        x5.as_mut_ptr().offset(7),
        __rounding,
        cos_bit as i32,
    );
    x6[6] = x5[6];
    x6[7] = x5[7];
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
#[target_feature(enable = "avx2")]
unsafe fn fidentity8x8_new_avx2(input: *const __m256i, output: *mut __m256i, cos_bit: i8) {
    let _ = cos_bit;
    *output = _mm256_adds_epi16(*input, *input);
    *output.offset(1) = _mm256_adds_epi16(*input.offset(1), *input.offset(1));
    *output.offset(2) = _mm256_adds_epi16(*input.offset(2), *input.offset(2));
    *output.offset(3) = _mm256_adds_epi16(*input.offset(3), *input.offset(3));
    *output.offset(4) = _mm256_adds_epi16(*input.offset(4), *input.offset(4));
    *output.offset(5) = _mm256_adds_epi16(*input.offset(5), *input.offset(5));
    *output.offset(6) = _mm256_adds_epi16(*input.offset(6), *input.offset(6));
    *output.offset(7) = _mm256_adds_epi16(*input.offset(7), *input.offset(7));
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn fdct8x16_new_avx2(input: *const __m128i, output: *mut __m128i, mut cos_bit: i8) {
    let mut cospi: *const i32 = cospi_arr(cos_bit as i32);
    let __rounding_256: __m256i = _mm256_set1_epi32(1 << (cos_bit as i32 - 1));
    let __rounding: __m128i = _mm_set1_epi32(1 << (cos_bit as i32 - 1));
    let mut temp0: __m128i = core::mem::zeroed();
    let mut temp1: __m128i = core::mem::zeroed();
    let mut temp2: __m128i = core::mem::zeroed();
    let mut temp3: __m128i = core::mem::zeroed();
    let mut in0: __m256i = core::mem::zeroed();
    let mut in1: __m256i = core::mem::zeroed();
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
    let mut cospi_arr: [__m256i; 12] = core::mem::zeroed();
    cospi_arr[0] =
        _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(cospi_m32_p32), cospi_m32_p32);
    cospi_arr[1] =
        _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(cospi_p32_p32), cospi_p32_p32);
    cospi_arr[2] =
        _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(cospi_p32_p32), cospi_p48_p16);
    cospi_arr[3] =
        _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(cospi_p32_m32), cospi_m16_p48);
    cospi_arr[4] =
        _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(cospi_m16_p48), cospi_m48_m16);
    cospi_arr[5] =
        _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(cospi_p48_p16), cospi_m16_p48);
    cospi_arr[6] =
        _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(cospi_p56_p08), cospi_p24_p40);
    cospi_arr[7] =
        _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(cospi_m08_p56), cospi_m40_p24);
    cospi_arr[8] =
        _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(cospi_p60_p04), cospi_p28_p36);
    cospi_arr[9] =
        _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(cospi_m04_p60), cospi_m36_p28);
    cospi_arr[10] =
        _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(cospi_p44_p20), cospi_p12_p52);
    cospi_arr[11] =
        _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(cospi_m20_p44), cospi_m52_p12);
    let mut x: [__m256i; 8] = core::mem::zeroed();
    x[0] = _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(*input), *input.offset(1));
    x[1] = _mm256_insertf128_si256::<0x1>(
        _mm256_castsi128_si256(*input.offset(15)),
        *input.offset(14),
    );
    x[2] =
        _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(*input.offset(2)), *input.offset(3));
    x[3] = _mm256_insertf128_si256::<0x1>(
        _mm256_castsi128_si256(*input.offset(13)),
        *input.offset(12),
    );
    x[4] =
        _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(*input.offset(5)), *input.offset(4));
    x[5] = _mm256_insertf128_si256::<0x1>(
        _mm256_castsi128_si256(*input.offset(10)),
        *input.offset(11),
    );
    x[6] =
        _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(*input.offset(7)), *input.offset(6));
    x[7] =
        _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(*input.offset(8)), *input.offset(9));
    // stage 1
    let mut x1: [__m256i; 8] = core::mem::zeroed();
    x1[0] = _mm256_adds_epi16(x[0], x[1]);
    x1[7] = _mm256_subs_epi16(x[0], x[1]);
    x1[1] = _mm256_adds_epi16(x[2], x[3]);
    x1[6] = _mm256_subs_epi16(x[2], x[3]);
    x1[2] = _mm256_adds_epi16(x[4], x[5]);
    x1[5] = _mm256_subs_epi16(x[4], x[5]);
    x1[3] = _mm256_adds_epi16(x[6], x[7]);
    x1[4] = _mm256_subs_epi16(x[6], x[7]);
    // stage 2
    let mut x2: [__m256i; 8] = core::mem::zeroed();
    x2[0] = _mm256_adds_epi16(x1[0], x1[3]);
    x2[7] = _mm256_subs_epi16(x1[0], x1[3]);
    x2[1] = _mm256_adds_epi16(x1[1], x1[2]);
    x2[6] = _mm256_subs_epi16(x1[1], x1[2]);
    x2[2] = x1[4];
    x2[3] = x1[7];
    btf_16_avx2(
        cospi_arr.as_mut_ptr(),
        cospi_arr.as_mut_ptr().offset(1),
        x1.as_mut_ptr().offset(5),
        x1.as_mut_ptr().offset(6),
        &raw mut temp0,
        &raw mut temp1,
        &raw mut temp2,
        &raw mut temp3,
        &raw const __rounding_256,
        &raw mut cos_bit,
    );
    x2[4] = _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(temp2), temp0);
    x2[5] = _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(temp3), temp1);
    // stage 3
    let mut x3: [__m256i; 8] = core::mem::zeroed();
    x2[1] = _mm256_permute4x64_epi64::<0x4e>(x2[1]);
    x3[0] = _mm256_adds_epi16(x2[0], x2[1]);
    x3[1] = _mm256_subs_epi16(x2[0], x2[1]);
    x3[2] = _mm256_blend_epi32::<0xf0>(x2[7], x2[6]);
    btf_16_sse2!(
        cospi_m32_p32,
        cospi_p32_p32,
        _mm256_castsi256_si128(x2[6]),
        _mm256_extractf128_si256::<0x01>(x2[7]),
        temp0,
        temp1,
        __rounding,
        cos_bit
    );
    x3[7] = _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(temp1), temp0);
    x3[3] = _mm256_adds_epi16(x2[2], x2[4]);
    x3[4] = _mm256_subs_epi16(x2[2], x2[4]);
    x3[5] = _mm256_adds_epi16(x2[3], x2[5]);
    x3[6] = _mm256_subs_epi16(x2[3], x2[5]);
    // stage 4
    let mut x4: [__m256i; 8] = core::mem::zeroed();
    x4[0] = _mm256_blend_epi32::<0xf0>(x3[0], x3[1]);
    x4[1] = _mm256_permute2f128_si256::<0x21>(x3[0], x3[1]);
    btf_16_avx2(
        cospi_arr.as_mut_ptr().offset(2),
        cospi_arr.as_mut_ptr().offset(3),
        x4.as_mut_ptr(),
        x4.as_mut_ptr().offset(1),
        output,
        output.offset(8),
        output.offset(4),
        output.offset(12),
        &raw const __rounding_256,
        &raw mut cos_bit,
    );
    x4[2] = _mm256_adds_epi16(x3[2], x3[7]);
    x4[3] = _mm256_subs_epi16(x3[2], x3[7]);
    x4[4] = _mm256_permute2f128_si256::<0x20>(x3[3], x3[4]);
    x4[5] = _mm256_permute2f128_si256::<0x20>(x3[6], x3[5]);
    in0 = _mm256_permute2f128_si256::<0x31>(x3[3], x3[4]);
    in1 = _mm256_permute2f128_si256::<0x31>(x3[5], x3[6]);
    btf_16_avx2(
        cospi_arr.as_mut_ptr().offset(4),
        cospi_arr.as_mut_ptr().offset(5),
        &raw mut in0,
        &raw mut in1,
        &raw mut temp0,
        &raw mut temp1,
        &raw mut temp2,
        &raw mut temp3,
        &raw const __rounding_256,
        &raw mut cos_bit,
    );
    x4[6] = _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(temp0), temp2);
    x4[7] = _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(temp3), temp1);
    // stage 5
    let mut x5: [__m256i; 4] = core::mem::zeroed();
    in0 = _mm256_permute2f128_si256::<0x31>(x4[2], x4[3]);
    in1 = _mm256_permute2f128_si256::<0x20>(x4[2], x4[3]);
    btf_16_avx2(
        cospi_arr.as_mut_ptr().offset(6),
        cospi_arr.as_mut_ptr().offset(7),
        &raw mut in0,
        &raw mut in1,
        output.offset(2),
        output.offset(14),
        output.offset(10),
        output.offset(6),
        &raw const __rounding_256,
        &raw mut cos_bit,
    );
    x5[0] = _mm256_adds_epi16(x4[4], x4[6]);
    x5[1] = _mm256_subs_epi16(x4[4], x4[6]);
    x5[2] = _mm256_adds_epi16(x4[5], x4[7]);
    x5[3] = _mm256_subs_epi16(x4[5], x4[7]);
    // stage 6
    in0 = _mm256_permute2f128_si256::<0x20>(x5[0], x5[1]);
    in1 = _mm256_permute2f128_si256::<0x31>(x5[2], x5[3]);
    btf_16_avx2(
        cospi_arr.as_mut_ptr().offset(8),
        cospi_arr.as_mut_ptr().offset(9),
        &raw mut in0,
        &raw mut in1,
        output.offset(1),
        output.offset(15),
        output.offset(9),
        output.offset(7),
        &raw const __rounding_256,
        &raw mut cos_bit,
    );
    in0 = _mm256_permute2f128_si256::<0x31>(x5[1], x5[0]);
    in1 = _mm256_permute2f128_si256::<0x20>(x5[3], x5[2]);
    btf_16_avx2(
        cospi_arr.as_mut_ptr().offset(10),
        cospi_arr.as_mut_ptr().offset(11),
        &raw mut in0,
        &raw mut in1,
        output.offset(5),
        output.offset(11),
        output.offset(13),
        output.offset(3),
        &raw const __rounding_256,
        &raw mut cos_bit,
    );
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn fadst8x16_new_avx2(input: *const __m128i, output: *mut __m128i, mut cos_bit: i8) {
    let mut cospi: *const i32 = cospi_arr(cos_bit as i32);
    let __zero: __m256i = _mm256_setzero_si256();
    let __rounding_256: __m256i = _mm256_set1_epi32(1 << (cos_bit as i32 - 1));
    let mut in0: __m256i = core::mem::zeroed();
    let mut in1: __m256i = core::mem::zeroed();
    let mut temp0: __m128i = core::mem::zeroed();
    let mut temp1: __m128i = core::mem::zeroed();
    let mut temp2: __m128i = core::mem::zeroed();
    let mut temp3: __m128i = core::mem::zeroed();
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
    let mut cospi_arr: [__m256i; 20] = core::mem::zeroed();
    cospi_arr[0] =
        _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(cospi_p32_p32), cospi_p32_p32);
    cospi_arr[1] =
        _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(cospi_p32_m32), cospi_p32_m32);
    cospi_arr[2] =
        _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(cospi_p32_p32), cospi_p32_p32);
    cospi_arr[3] =
        _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(cospi_p32_m32), cospi_p32_m32);
    cospi_arr[4] =
        _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(cospi_p16_p48), cospi_m48_p16);
    cospi_arr[5] =
        _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(cospi_p48_m16), cospi_p16_p48);
    cospi_arr[6] =
        _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(cospi_p16_p48), cospi_m48_p16);
    cospi_arr[7] =
        _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(cospi_p48_m16), cospi_p16_p48);
    cospi_arr[8] =
        _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(cospi_p08_p56), cospi_p40_p24);
    cospi_arr[9] =
        _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(cospi_p56_m08), cospi_p24_m40);
    cospi_arr[10] =
        _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(cospi_m56_p08), cospi_m24_p40);
    cospi_arr[11] =
        _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(cospi_p08_p56), cospi_p40_p24);
    cospi_arr[12] =
        _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(cospi_p02_p62), cospi_p10_p54);
    cospi_arr[13] =
        _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(cospi_p62_m02), cospi_p54_m10);
    cospi_arr[14] =
        _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(cospi_p18_p46), cospi_p26_p38);
    cospi_arr[15] =
        _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(cospi_p46_m18), cospi_p38_m26);
    cospi_arr[16] =
        _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(cospi_p34_p30), cospi_p42_p22);
    cospi_arr[17] =
        _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(cospi_p30_m34), cospi_p22_m42);
    cospi_arr[18] =
        _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(cospi_p50_p14), cospi_p58_p06);
    cospi_arr[19] =
        _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(cospi_p14_m50), cospi_p06_m58);
    let mut x: [__m256i; 8] = core::mem::zeroed();
    x[0] = _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(*input), *input.offset(4));
    x[1] =
        _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(*input.offset(2)), *input.offset(6));
    x[2] =
        _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(*input.offset(8)), *input.offset(12));
    x[3] = _mm256_insertf128_si256::<0x1>(
        _mm256_castsi128_si256(*input.offset(10)),
        *input.offset(14),
    );
    x[4] =
        _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(*input.offset(1)), *input.offset(9));
    x[5] =
        _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(*input.offset(3)), *input.offset(11));
    x[6] =
        _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(*input.offset(5)), *input.offset(13));
    x[7] =
        _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(*input.offset(7)), *input.offset(15));
    // stage 1
    let mut x1: [__m256i; 8] = core::mem::zeroed();
    x1[0] = x[0];
    x1[1] = _mm256_subs_epi16(__zero, x[7]);
    x1[2] = x[2];
    x1[3] = _mm256_subs_epi16(__zero, x[5]);
    x1[4] = _mm256_subs_epi16(__zero, x[4]);
    x1[5] = x[3];
    x1[6] = _mm256_subs_epi16(__zero, x[6]);
    x1[7] = x[1];
    // stage 2
    let mut x2: [__m256i; 8] = core::mem::zeroed();
    x2[0] = _mm256_blend_epi32::<0xf0>(x1[0], x1[1]);
    x2[3] = _mm256_blend_epi32::<0xf0>(x1[3], x1[2]);
    x2[4] = _mm256_blend_epi32::<0xf0>(x1[4], x1[5]);
    x2[7] = _mm256_blend_epi32::<0xf0>(x1[7], x1[6]);
    in0 = _mm256_blend_epi32::<0xf0>(x1[1], x1[0]);
    in1 = _mm256_blend_epi32::<0xf0>(x1[2], x1[3]);
    btf_16_avx2(
        cospi_arr.as_mut_ptr(),
        cospi_arr.as_mut_ptr().offset(1),
        &raw mut in0,
        &raw mut in1,
        &raw mut temp0,
        &raw mut temp1,
        &raw mut temp2,
        &raw mut temp3,
        &raw const __rounding_256,
        &raw mut cos_bit,
    );
    x2[1] = _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(temp0), temp1);
    x2[2] = _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(temp2), temp3);
    in0 = _mm256_permute2f128_si256::<0x21>(x1[7], x1[6]);
    in1 = _mm256_permute2f128_si256::<0x21>(x1[4], x1[5]);
    btf_16_avx2(
        cospi_arr.as_mut_ptr().offset(2),
        cospi_arr.as_mut_ptr().offset(3),
        &raw mut in0,
        &raw mut in1,
        &raw mut temp0,
        &raw mut temp1,
        &raw mut temp2,
        &raw mut temp3,
        &raw const __rounding_256,
        &raw mut cos_bit,
    );
    x2[5] = _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(temp0), temp1);
    x2[6] = _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(temp2), temp3);
    // stage 3
    let mut x3: [__m256i; 8] = core::mem::zeroed();
    x3[0] = _mm256_adds_epi16(x2[0], x2[1]);
    x3[1] = _mm256_subs_epi16(x2[0], x2[1]);
    x3[2] = _mm256_adds_epi16(x2[3], x2[2]);
    x3[3] = _mm256_subs_epi16(x2[3], x2[2]);
    x3[4] = _mm256_adds_epi16(x2[4], x2[5]);
    x3[5] = _mm256_subs_epi16(x2[4], x2[5]);
    x3[6] = _mm256_adds_epi16(x2[7], x2[6]);
    x3[7] = _mm256_subs_epi16(x2[7], x2[6]);
    // stage 4
    let mut x4: [__m256i; 8] = core::mem::zeroed();
    x4[0] = x3[0];
    x4[1] = x3[1];
    x4[4] = x3[4];
    x4[5] = x3[5];
    in0 = _mm256_permute2f128_si256::<0x20>(x3[2], x3[3]);
    in1 = _mm256_permute2f128_si256::<0x31>(x3[2], x3[3]);
    btf_16_avx2(
        cospi_arr.as_mut_ptr().offset(4),
        cospi_arr.as_mut_ptr().offset(5),
        &raw mut in0,
        &raw mut in1,
        &raw mut temp0,
        &raw mut temp1,
        &raw mut temp2,
        &raw mut temp3,
        &raw const __rounding_256,
        &raw mut cos_bit,
    );
    x4[2] = _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(temp0), temp1);
    x4[3] = _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(temp2), temp3);
    in0 = _mm256_permute2f128_si256::<0x20>(x3[6], x3[7]);
    in1 = _mm256_permute2f128_si256::<0x31>(x3[6], x3[7]);
    btf_16_avx2(
        cospi_arr.as_mut_ptr().offset(6),
        cospi_arr.as_mut_ptr().offset(7),
        &raw mut in0,
        &raw mut in1,
        &raw mut temp0,
        &raw mut temp1,
        &raw mut temp2,
        &raw mut temp3,
        &raw const __rounding_256,
        &raw mut cos_bit,
    );
    x4[6] = _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(temp0), temp1);
    x4[7] = _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(temp2), temp3);
    // stage 5
    let mut x5: [__m256i; 8] = core::mem::zeroed();
    x5[0] = _mm256_adds_epi16(x4[0], x4[2]);
    x5[1] = _mm256_subs_epi16(x4[0], x4[2]);
    x5[2] = _mm256_adds_epi16(x4[1], x4[3]);
    x5[3] = _mm256_subs_epi16(x4[1], x4[3]);
    x5[4] = _mm256_adds_epi16(x4[4], x4[6]);
    x5[5] = _mm256_subs_epi16(x4[4], x4[6]);
    x5[6] = _mm256_adds_epi16(x4[5], x4[7]);
    x5[7] = _mm256_subs_epi16(x4[5], x4[7]);
    // stage 6
    let mut x6: [__m256i; 8] = core::mem::zeroed();
    x6[0] = x5[0];
    x6[1] = x5[2];
    x6[2] = x5[1];
    x6[3] = x5[3];
    in0 = _mm256_permute2f128_si256::<0x20>(x5[4], x5[6]);
    in1 = _mm256_permute2f128_si256::<0x31>(x5[4], x5[6]);
    btf_16_avx2(
        cospi_arr.as_mut_ptr().offset(8),
        cospi_arr.as_mut_ptr().offset(9),
        &raw mut in0,
        &raw mut in1,
        &raw mut temp0,
        &raw mut temp1,
        &raw mut temp2,
        &raw mut temp3,
        &raw const __rounding_256,
        &raw mut cos_bit,
    );
    x6[4] = _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(temp0), temp1);
    x6[5] = _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(temp2), temp3);
    in0 = _mm256_permute2f128_si256::<0x20>(x5[5], x5[7]);
    in1 = _mm256_permute2f128_si256::<0x31>(x5[5], x5[7]);
    btf_16_avx2(
        cospi_arr.as_mut_ptr().offset(10),
        cospi_arr.as_mut_ptr().offset(11),
        &raw mut in0,
        &raw mut in1,
        &raw mut temp0,
        &raw mut temp1,
        &raw mut temp2,
        &raw mut temp3,
        &raw const __rounding_256,
        &raw mut cos_bit,
    );
    x6[6] = _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(temp0), temp1);
    x6[7] = _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(temp2), temp3);
    // stage 7
    let mut x7: [__m256i; 8] = core::mem::zeroed();
    x7[0] = _mm256_adds_epi16(x6[0], x6[4]);
    x7[1] = _mm256_subs_epi16(x6[0], x6[4]);
    x7[2] = _mm256_adds_epi16(x6[1], x6[5]);
    x7[3] = _mm256_subs_epi16(x6[1], x6[5]);
    x7[4] = _mm256_adds_epi16(x6[2], x6[6]);
    x7[5] = _mm256_subs_epi16(x6[2], x6[6]);
    x7[6] = _mm256_adds_epi16(x6[3], x6[7]);
    x7[7] = _mm256_subs_epi16(x6[3], x6[7]);
    // stage 8
    in0 = _mm256_permute2f128_si256::<0x20>(x7[0], x7[2]);
    in1 = _mm256_permute2f128_si256::<0x31>(x7[0], x7[2]);
    btf_16_avx2(
        cospi_arr.as_mut_ptr().offset(12),
        cospi_arr.as_mut_ptr().offset(13),
        &raw mut in0,
        &raw mut in1,
        output.offset(15),
        output,
        output.offset(13),
        output.offset(2),
        &raw const __rounding_256,
        &raw mut cos_bit,
    );
    in0 = _mm256_permute2f128_si256::<0x20>(x7[4], x7[6]);
    in1 = _mm256_permute2f128_si256::<0x31>(x7[4], x7[6]);
    btf_16_avx2(
        cospi_arr.as_mut_ptr().offset(14),
        cospi_arr.as_mut_ptr().offset(15),
        &raw mut in0,
        &raw mut in1,
        output.offset(11),
        output.offset(4),
        output.offset(9),
        output.offset(6),
        &raw const __rounding_256,
        &raw mut cos_bit,
    );
    in0 = _mm256_permute2f128_si256::<0x20>(x7[1], x7[3]);
    in1 = _mm256_permute2f128_si256::<0x31>(x7[1], x7[3]);
    btf_16_avx2(
        cospi_arr.as_mut_ptr().offset(16),
        cospi_arr.as_mut_ptr().offset(17),
        &raw mut in0,
        &raw mut in1,
        output.offset(7),
        output.offset(8),
        output.offset(5),
        output.offset(10),
        &raw const __rounding_256,
        &raw mut cos_bit,
    );
    in0 = _mm256_permute2f128_si256::<0x20>(x7[5], x7[7]);
    in1 = _mm256_permute2f128_si256::<0x31>(x7[5], x7[7]);
    btf_16_avx2(
        cospi_arr.as_mut_ptr().offset(18),
        cospi_arr.as_mut_ptr().offset(19),
        &raw mut in0,
        &raw mut in1,
        output.offset(3),
        output.offset(12),
        output.offset(1),
        output.offset(14),
        &raw const __rounding_256,
        &raw mut cos_bit,
    );
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn fidentity8x16_new_avx2(input: *const __m128i, output: *mut __m128i, cos_bit: i8) {
    let _ = cos_bit;
    let one: __m256i = _mm256_set1_epi16(1);
    let mut temp: __m256i;
    for i in (0..16).step_by(2) {
        temp = _mm256_insertf128_si256::<0x1>(
            _mm256_castsi128_si256(*input.offset(i as isize)),
            *input.offset((i + 1) as isize),
        );
        let a_lo: __m256i = _mm256_unpacklo_epi16(temp, one);
        let a_hi: __m256i = _mm256_unpackhi_epi16(temp, one);
        let b_lo: __m256i = scale_round_avx2(a_lo, 2 * NewSqrt2);
        let b_hi: __m256i = scale_round_avx2(a_hi, 2 * NewSqrt2);
        temp = _mm256_packs_epi32(b_lo, b_hi);
        *output.offset(i as isize) = _mm256_castsi256_si128(temp);
        *output.offset((i + 1) as isize) = _mm256_extractf128_si256::<0x1>(temp);
    }
}

static row_txfm8x16_arr: [transform_1d_avx2; (TX_TYPES) as usize] = [
    Some(fdct8x8_new_avx2),
    Some(fdct8x8_new_avx2),
    Some(fadst8x8_new_avx2),
    Some(fadst8x8_new_avx2),
    Some(fdct8x8_new_avx2),
    Some(fadst8x8_new_avx2),
    Some(fadst8x8_new_avx2),
    Some(fadst8x8_new_avx2),
    Some(fadst8x8_new_avx2),
    Some(fidentity8x8_new_avx2),
    Some(fidentity8x8_new_avx2),
    Some(fdct8x8_new_avx2),
    Some(fidentity8x8_new_avx2),
    Some(fadst8x8_new_avx2),
    Some(fidentity8x8_new_avx2),
    Some(fadst8x8_new_avx2),
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
    Some(fdct8x16_new_avx2),
    Some(fadst8x16_new_avx2),
    Some(fdct8x16_new_avx2),
    Some(fadst8x16_new_avx2),
    Some(fadst8x16_new_avx2),
    Some(fdct8x16_new_avx2),
    Some(fadst8x16_new_avx2),
    Some(fadst8x16_new_avx2),
    Some(fadst8x16_new_avx2),
    Some(fidentity8x16_new_avx2),
    Some(fdct8x16_new_avx2),
    Some(fidentity8x16_new_avx2),
    Some(fadst8x16_new_avx2),
    Some(fidentity8x16_new_avx2),
    Some(fadst8x16_new_avx2),
    Some(fidentity8x16_new_avx2),
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
static col_txfm16x8_arr: [transform_1d_avx2; (TX_TYPES) as usize] = [
    Some(fdct8x8_new_avx2),
    Some(fadst8x8_new_avx2),
    Some(fdct8x8_new_avx2),
    Some(fadst8x8_new_avx2),
    Some(fadst8x8_new_avx2),
    Some(fdct8x8_new_avx2),
    Some(fadst8x8_new_avx2),
    Some(fadst8x8_new_avx2),
    Some(fadst8x8_new_avx2),
    Some(fidentity8x8_new_avx2),
    Some(fdct8x8_new_avx2),
    Some(fidentity8x8_new_avx2),
    Some(fadst8x8_new_avx2),
    Some(fidentity8x8_new_avx2),
    Some(fadst8x8_new_avx2),
    Some(fidentity8x8_new_avx2),
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
static row_txfm16x8_arr: [transform_1d_sse2; (TX_TYPES) as usize] = [
    Some(fdct8x16_new_avx2),
    Some(fdct8x16_new_avx2),
    Some(fadst8x16_new_avx2),
    Some(fadst8x16_new_avx2),
    Some(fdct8x16_new_avx2),
    Some(fadst8x16_new_avx2),
    Some(fadst8x16_new_avx2),
    Some(fadst8x16_new_avx2),
    Some(fadst8x16_new_avx2),
    Some(fidentity8x16_new_avx2),
    Some(fidentity8x16_new_avx2),
    Some(fdct8x16_new_avx2),
    Some(fidentity8x16_new_avx2),
    Some(fadst8x16_new_avx2),
    Some(fidentity8x16_new_avx2),
    Some(fadst8x16_new_avx2),
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
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn lowbd_fwd_txfm2d_8x16_avx2(
    input: *const i16,
    output: *mut i32,
    stride: i32,
    tx_type: u8,
    bd: i32,
) {
    let _ = bd;
    let mut buf0: [__m128i; 16] = core::mem::zeroed();
    let mut buf1: [__m128i; 16] = core::mem::zeroed();
    let mut buf2: [__m256i; 8] = core::mem::zeroed();
    let mut shift: *const i8 = av1_fwd_txfm_shift_ls[TX_8X16 as usize];
    let txw_idx: i32 = get_txw_idx(TX_8X16 as u8);
    let txh_idx: i32 = get_txh_idx(TX_8X16 as u8);
    let cos_bit_col: i32 = av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32;
    let cos_bit_row: i32 = av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32;
    let width: i32 = 8;
    let height: i32 = 16;
    let col_txfm: transform_1d_sse2 = col_txfm8x16_arr[tx_type as usize];
    let row_txfm: transform_1d_avx2 = row_txfm8x16_arr[tx_type as usize];
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
    let mut bufl: *mut __m128i;
    let mut bufu: *mut __m128i;
    if lr_flip != 0 {
        bufl = buf0.as_mut_ptr();
        bufu = buf0.as_mut_ptr().offset(8);
        flip_buf_sse2(buf1.as_mut_ptr().offset((width * 0) as isize), bufl, width);
        flip_buf_sse2(buf1.as_mut_ptr().offset((width * 1) as isize), bufu, width);
    } else {
        bufl = buf1.as_mut_ptr().offset((width * 0) as isize);
        bufu = buf1.as_mut_ptr().offset((width * 1) as isize);
    }
    pack_reg(
        bufl as *const __m128i,
        bufu as *const __m128i,
        buf2.as_mut_ptr(),
    );
    row_txfm.unwrap()(
        buf2.as_mut_ptr() as *const __m256i,
        buf2.as_mut_ptr(),
        cos_bit_row as i8,
    );
    round_shift_16bit_w16_avx2(buf2.as_mut_ptr(), width, *shift.offset(2) as i32);
    store_rect_buffer_16bit_to_32bit_w16_avx2(
        buf2.as_mut_ptr() as *const __m256i,
        output,
        height,
        width,
    );
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn lowbd_fwd_txfm2d_16x8_avx2(
    input: *const i16,
    output: *mut i32,
    stride: i32,
    tx_type: u8,
    bd: i32,
) {
    let _ = bd;
    let mut buf0: [__m128i; 16] = core::mem::zeroed();
    let mut buf1: [__m128i; 16] = core::mem::zeroed();
    let mut buf2: [__m256i; 8] = core::mem::zeroed();
    let mut shift: *const i8 = av1_fwd_txfm_shift_ls[TX_16X8 as usize];
    let txw_idx: i32 = get_txw_idx(TX_16X8 as u8);
    let txh_idx: i32 = get_txh_idx(TX_16X8 as u8);
    let cos_bit_col: i32 = av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32;
    let cos_bit_row: i32 = av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32;
    let width: i32 = 16;
    let height: i32 = 8;
    let col_txfm: transform_1d_avx2 = col_txfm16x8_arr[tx_type as usize];
    let row_txfm: transform_1d_sse2 = row_txfm16x8_arr[tx_type as usize];
    let mut buf: *mut __m128i;
    let mut ud_flip: i32 = 0;
    let mut lr_flip: i32 = 0;
    get_flip_cfg(tx_type, &raw mut ud_flip, &raw mut lr_flip);
    if ud_flip != 0 {
        load_buffer_16bit_to_16bit_flip(
            input.offset((8 * 0) as isize),
            stride,
            buf0.as_mut_ptr(),
            height,
        );
        load_buffer_16bit_to_16bit_flip(
            input.offset((8 * 1) as isize),
            stride,
            buf0.as_mut_ptr().offset(8),
            height,
        );
    } else {
        load_buffer_16bit_to_16bit(
            input.offset((8 * 0) as isize),
            stride,
            buf0.as_mut_ptr(),
            height,
        );
        load_buffer_16bit_to_16bit(
            input.offset((8 * 1) as isize),
            stride,
            buf0.as_mut_ptr().offset(8),
            height,
        );
    }
    pack_reg(
        buf0.as_mut_ptr() as *const __m128i,
        buf0.as_mut_ptr().offset(8) as *const __m128i,
        buf2.as_mut_ptr(),
    );
    round_shift_16bit_w16_avx2(buf2.as_mut_ptr(), height, *shift as i32);
    col_txfm.unwrap()(
        buf2.as_mut_ptr() as *const __m256i,
        buf2.as_mut_ptr(),
        cos_bit_col as i8,
    );
    round_shift_16bit_w16_avx2(buf2.as_mut_ptr(), height, *shift.offset(1) as i32);
    transpose_16bit_16x8_avx2(buf2.as_mut_ptr() as *const __m256i, buf2.as_mut_ptr());
    extract_reg(buf2.as_mut_ptr() as *const __m256i, buf1.as_mut_ptr());
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

static fwd_txfm2d_func_ls: [FwdTxfm2dFunc; (TX_SIZES_ALL) as usize] = [
    Some(av1_lowbd_fwd_txfm2d_4x4_sse2),
    Some(av1_lowbd_fwd_txfm2d_8x8_avx2),
    Some(lowbd_fwd_txfm2d_16x16_avx2),
    Some(lowbd_fwd_txfm2d_32x32_avx2),
    Some(lowbd_fwd_txfm2d_64x64_avx2),
    Some(av1_lowbd_fwd_txfm2d_4x8_sse2),
    Some(av1_lowbd_fwd_txfm2d_8x4_sse2),
    Some(lowbd_fwd_txfm2d_8x16_avx2),
    Some(lowbd_fwd_txfm2d_16x8_avx2),
    Some(lowbd_fwd_txfm2d_16x32_avx2),
    Some(lowbd_fwd_txfm2d_32x16_avx2),
    Some(lowbd_fwd_txfm2d_32x64_avx2),
    Some(lowbd_fwd_txfm2d_64x32_avx2),
    Some(av1_lowbd_fwd_txfm2d_4x16_sse2),
    Some(av1_lowbd_fwd_txfm2d_16x4_sse2),
    Some(av1_lowbd_fwd_txfm2d_8x32_sse2),
    Some(av1_lowbd_fwd_txfm2d_32x8_sse2),
    Some(lowbd_fwd_txfm2d_16x64_avx2),
    Some(lowbd_fwd_txfm2d_64x16_avx2),
];

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
#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn av1_lowbd_fwd_txfm_avx2(
    src_diff: *const i16,
    coeff: *mut i32,
    diff_stride: i32,
    txfm_param: *mut TxfmParam,
) {
    let mut fwd_txfm2d_func: FwdTxfm2dFunc = fwd_txfm2d_func_ls[(*txfm_param).tx_size as usize];
    if (*txfm_param).lossless != 0 && ((*txfm_param).tx_size as i32) == TX_4X4 {
        av1_lowbd_fwd_txfm_c(src_diff, coeff, diff_stride, txfm_param);
    } else {
        fwd_txfm2d_func.unwrap()(
            src_diff,
            coeff,
            diff_stride,
            (*txfm_param).tx_type,
            (*txfm_param).bd,
        );
    }
}
