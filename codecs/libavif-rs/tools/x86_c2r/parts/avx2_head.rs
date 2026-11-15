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
#![allow(non_snake_case, non_upper_case_globals, unused_assignments, unused_mut, unused_parens)]
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

