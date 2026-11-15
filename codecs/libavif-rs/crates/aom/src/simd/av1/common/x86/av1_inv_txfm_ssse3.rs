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

// Translated from upstream/aom/av1/common/x86/av1_inv_txfm_ssse3.h and
// av1_inv_txfm_ssse3.c (ADR 0056, ADR 0058). Upstream compiles the .c file
// with -mssse3. The header items that the high-bitdepth inverse transforms
// use (the 1D type tables, the eob tables, and the eob helpers) were
// translated by hand first. The header's butterfly macros and
// round_shift_16bit_ssse3 were written by hand with the same expansion, and
// the .c file was transliterated with a typing-aware C-subset printer (not
// C2Rust) and reviewed against the C function by function: the same sequence
// of intrinsic and helper calls, and the same identifiers, constants, and
// operators in order, as ADR 0058 describes for the high-bitdepth transforms.
//
// Translation notes:
// - Every function of the .c file carries `ssse3`. The RTCD specialization
//   av1_inv_txfm_add_ssse3 is exported under its upstream name; the other
//   non-static functions, which av1_inv_txfm_avx2.c calls, are public Rust
//   functions.
// - The eob helpers use no intrinsics; they carry `ssse3`, the flag of the
//   header's own translation unit, and inline into SSE4.1 and AVX2 callers.
// - `int *` out-parameters and array parameters stay raw pointers
//   (`in[i]` -> `*in_.offset(i)`), so in-place calls keep the C aliasing.
//   Local arrays are Rust arrays (`x[i]` -> `x[i as usize]`) and start zeroed.
// - C integer conversions are explicit (`as`); `switch` becomes `match` on
//   the `int`-promoted value; `assert` becomes `debug_assert!`. Calling a NULL
//   `transform_1d_ssse3` entry, which C leaves undefined, panics through
//   `Option::unwrap`.
// - Immediate operands become const generics. Shift counts that are run-time
//   `int` values use the count-register intrinsics, which is what GCC and
//   Clang emit for a non-constant count.
// - The butterfly macros read `__rounding` and `cos_bit` from the caller's
//   scope in C; the Rust macros take them as trailing arguments.
// - The DECLARE_ALIGNED(16, ...) alignment of the eob tables is not needed:
//   they are only read element by element. The pointer table is `static mut`
//   because it holds raw pointers, as in the Neon translation.
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
use crate::av1__common__av1_txfm::{av1_cospi_arr_data, av1_sinpi_arr_data};
use crate::av1__common__idct::{av1_inv_txfm_add_c, TxfmParam};
use crate::simd::aom_dsp__x86__transpose_sse2::{
    transpose_16bit_4x4, transpose_16bit_4x8, transpose_16bit_8x4, transpose_16bit_8x8,
};
use crate::simd::aom_dsp__x86__txfm_common_sse2::pair_set_epi16;
use crate::simd::av1__common__x86__av1_txfm_sse2::{
    btf_16_4p_sse2, btf_16_sse2, flip_buf_sse2, load_32bit_to_16bit, load_buffer_32bit_to_16bit,
    load_buffer_32bit_to_16bit_w4, round_shift_16bit,
};

// From av1/common/enums.h.
type TX_SIZE = u8;

// From av1/common/common_data.h.
// Transform block width in pixels
pub static tx_size_wide: [i32; 19] = [
    4, 8, 16, 32, 64, 4, 8, 8, 16, 16, 32, 32, 64, 4, 16, 8, 32, 16, 64,
];
// Transform block height in pixels
pub static tx_size_high: [i32; 19] = [
    4, 8, 16, 32, 64, 8, 4, 16, 8, 32, 16, 64, 32, 16, 4, 32, 8, 64, 16,
];

// From aom_dsp/aom_dsp_common.h.
#[inline]
pub fn AOMMIN(x: i32, y: i32) -> i32 {
    if x < y {
        x
    } else {
        y
    }
}

// 1D itx types
pub const IDCT_1D: i32 = 0;
pub const IADST_1D: i32 = 1;
pub const IFLIPADST_1D: i32 = IADST_1D;
pub const IIDENTITY_1D: i32 = 2;
pub const ITX_TYPES_1D: i32 = 3;
// `enum { ... } UENUM1BYTE(ITX_TYPE_1D)` is one byte wide.
pub type ITX_TYPE_1D = u8;

pub static vitx_1d_tab: [ITX_TYPE_1D; 16] = [
    IDCT_1D as ITX_TYPE_1D,
    IADST_1D as ITX_TYPE_1D,
    IDCT_1D as ITX_TYPE_1D,
    IADST_1D as ITX_TYPE_1D,
    IFLIPADST_1D as ITX_TYPE_1D,
    IDCT_1D as ITX_TYPE_1D,
    IFLIPADST_1D as ITX_TYPE_1D,
    IADST_1D as ITX_TYPE_1D,
    IFLIPADST_1D as ITX_TYPE_1D,
    IIDENTITY_1D as ITX_TYPE_1D,
    IDCT_1D as ITX_TYPE_1D,
    IIDENTITY_1D as ITX_TYPE_1D,
    IADST_1D as ITX_TYPE_1D,
    IIDENTITY_1D as ITX_TYPE_1D,
    IFLIPADST_1D as ITX_TYPE_1D,
    IIDENTITY_1D as ITX_TYPE_1D,
];

pub static hitx_1d_tab: [ITX_TYPE_1D; 16] = [
    IDCT_1D as ITX_TYPE_1D,
    IDCT_1D as ITX_TYPE_1D,
    IADST_1D as ITX_TYPE_1D,
    IADST_1D as ITX_TYPE_1D,
    IDCT_1D as ITX_TYPE_1D,
    IFLIPADST_1D as ITX_TYPE_1D,
    IFLIPADST_1D as ITX_TYPE_1D,
    IFLIPADST_1D as ITX_TYPE_1D,
    IADST_1D as ITX_TYPE_1D,
    IIDENTITY_1D as ITX_TYPE_1D,
    IIDENTITY_1D as ITX_TYPE_1D,
    IDCT_1D as ITX_TYPE_1D,
    IIDENTITY_1D as ITX_TYPE_1D,
    IADST_1D as ITX_TYPE_1D,
    IIDENTITY_1D as ITX_TYPE_1D,
    IFLIPADST_1D as ITX_TYPE_1D,
];

static av1_eob_to_eobxy_8x8_default: [i16; 8] = [
    0x0707, 0x0707, 0x0707, 0x0707, 0x0707, 0x0707, 0x0707, 0x0707,
];

static av1_eob_to_eobxy_16x16_default: [i16; 16] = [
    0x0707, 0x0707, 0x0f0f, 0x0f0f, 0x0f0f, 0x0f0f, 0x0f0f, 0x0f0f, 0x0f0f, 0x0f0f, 0x0f0f, 0x0f0f,
    0x0f0f, 0x0f0f, 0x0f0f, 0x0f0f,
];

static av1_eob_to_eobxy_32x32_default: [i16; 32] = [
    0x0707, 0x0f0f, 0x0f0f, 0x0f0f, 0x1f1f, 0x1f1f, 0x1f1f, 0x1f1f, 0x1f1f, 0x1f1f, 0x1f1f, 0x1f1f,
    0x1f1f, 0x1f1f, 0x1f1f, 0x1f1f, 0x1f1f, 0x1f1f, 0x1f1f, 0x1f1f, 0x1f1f, 0x1f1f, 0x1f1f, 0x1f1f,
    0x1f1f, 0x1f1f, 0x1f1f, 0x1f1f, 0x1f1f, 0x1f1f, 0x1f1f, 0x1f1f,
];

static av1_eob_to_eobxy_8x16_default: [i16; 16] = [
    0x0707, 0x0707, 0x0707, 0x0707, 0x0707, 0x0f07, 0x0f07, 0x0f07, 0x0f07, 0x0f07, 0x0f07, 0x0f07,
    0x0f07, 0x0f07, 0x0f07, 0x0f07,
];

static av1_eob_to_eobxy_16x8_default: [i16; 8] = [
    0x0707, 0x0707, 0x070f, 0x070f, 0x070f, 0x070f, 0x070f, 0x070f,
];

static av1_eob_to_eobxy_16x32_default: [i16; 32] = [
    0x0707, 0x0707, 0x0f0f, 0x0f0f, 0x0f0f, 0x0f0f, 0x0f0f, 0x0f0f, 0x0f0f, 0x1f0f, 0x1f0f, 0x1f0f,
    0x1f0f, 0x1f0f, 0x1f0f, 0x1f0f, 0x1f0f, 0x1f0f, 0x1f0f, 0x1f0f, 0x1f0f, 0x1f0f, 0x1f0f, 0x1f0f,
    0x1f0f, 0x1f0f, 0x1f0f, 0x1f0f, 0x1f0f, 0x1f0f, 0x1f0f, 0x1f0f,
];

static av1_eob_to_eobxy_32x16_default: [i16; 16] = [
    0x0707, 0x0f0f, 0x0f0f, 0x0f0f, 0x0f1f, 0x0f1f, 0x0f1f, 0x0f1f, 0x0f1f, 0x0f1f, 0x0f1f, 0x0f1f,
    0x0f1f, 0x0f1f, 0x0f1f, 0x0f1f,
];

static av1_eob_to_eobxy_8x32_default: [i16; 32] = [
    0x0707, 0x0707, 0x0707, 0x0707, 0x0707, 0x0f07, 0x0f07, 0x0f07, 0x0f07, 0x0f07, 0x0f07, 0x0f07,
    0x0f07, 0x1f07, 0x1f07, 0x1f07, 0x1f07, 0x1f07, 0x1f07, 0x1f07, 0x1f07, 0x1f07, 0x1f07, 0x1f07,
    0x1f07, 0x1f07, 0x1f07, 0x1f07, 0x1f07, 0x1f07, 0x1f07, 0x1f07,
];

static av1_eob_to_eobxy_32x8_default: [i16; 8] = [
    0x0707, 0x070f, 0x070f, 0x071f, 0x071f, 0x071f, 0x071f, 0x071f,
];

static mut av1_eob_to_eobxy_default: [*const i16; 19] = [
    core::ptr::null(),
    av1_eob_to_eobxy_8x8_default.as_ptr(),
    av1_eob_to_eobxy_16x16_default.as_ptr(),
    av1_eob_to_eobxy_32x32_default.as_ptr(),
    av1_eob_to_eobxy_32x32_default.as_ptr(),
    core::ptr::null(),
    core::ptr::null(),
    av1_eob_to_eobxy_8x16_default.as_ptr(),
    av1_eob_to_eobxy_16x8_default.as_ptr(),
    av1_eob_to_eobxy_16x32_default.as_ptr(),
    av1_eob_to_eobxy_32x16_default.as_ptr(),
    av1_eob_to_eobxy_32x32_default.as_ptr(),
    av1_eob_to_eobxy_32x32_default.as_ptr(),
    core::ptr::null(),
    core::ptr::null(),
    av1_eob_to_eobxy_8x32_default.as_ptr(),
    av1_eob_to_eobxy_32x8_default.as_ptr(),
    av1_eob_to_eobxy_16x32_default.as_ptr(),
    av1_eob_to_eobxy_32x16_default.as_ptr(),
];

pub static lowbd_txfm_all_1d_zeros_idx: [i32; 32] = [
    0, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
];

// Transform block width in log2 for eob (size of 64 map to 32)
static tx_size_wide_log2_eob: [i32; 19] = [2, 3, 4, 5, 5, 2, 3, 3, 4, 4, 5, 5, 5, 2, 4, 3, 5, 4, 5];

#[inline]
#[target_feature(enable = "ssse3")]
pub unsafe fn get_eobx_eoby_scan_default(
    eobx: *mut i32,
    eoby: *mut i32,
    tx_size: TX_SIZE,
    eob: i32,
) {
    if eob == 1 {
        *eobx = 0;
        *eoby = 0;
        return;
    }

    let tx_w_log2: i32 = tx_size_wide_log2_eob[tx_size as usize];
    let eob_row: i32 = (eob - 1) >> tx_w_log2;
    let eobxy: i32 = *av1_eob_to_eobxy_default[tx_size as usize].offset(eob_row as isize) as i32;
    *eobx = eobxy & 0xFF;
    *eoby = eobxy >> 8;
}

static eob_fill: [i32; 32] = [
    0, 7, 7, 7, 7, 7, 7, 7, 15, 15, 15, 15, 15, 15, 15, 15, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31,
    31, 31, 31, 31, 31, 31,
];

#[inline]
#[target_feature(enable = "ssse3")]
pub unsafe fn get_eobx_eoby_scan_h_identity(
    eobx: *mut i32,
    eoby: *mut i32,
    tx_size: TX_SIZE,
    mut eob: i32,
) {
    eob -= 1;
    let txfm_size_col: i32 = tx_size_wide[tx_size as usize];
    let eobx_max: i32 = AOMMIN(32, txfm_size_col) - 1;
    *eobx = if eob >= eobx_max {
        eobx_max
    } else {
        eob_fill[eob as usize]
    };
    let temp_eoby: i32 = eob / (eobx_max + 1);
    debug_assert!(temp_eoby < 32);
    *eoby = eob_fill[temp_eoby as usize];
}

#[inline]
#[target_feature(enable = "ssse3")]
pub unsafe fn get_eobx_eoby_scan_v_identity(
    eobx: *mut i32,
    eoby: *mut i32,
    tx_size: TX_SIZE,
    mut eob: i32,
) {
    eob -= 1;
    let txfm_size_row: i32 = tx_size_high[tx_size as usize];
    let eoby_max: i32 = AOMMIN(32, txfm_size_row) - 1;
    *eobx = eob_fill[(eob / (eoby_max + 1)) as usize];
    *eoby = if eob >= eoby_max {
        eoby_max
    } else {
        eob_fill[eob as usize]
    };
}

// From av1/common/enums.h.
type TX_TYPE = u8;

pub const TX_4X4: i32 = 0;
pub const TX_8X8: i32 = 1;
pub const TX_16X16: i32 = 2;
pub const TX_32X32: i32 = 3;
pub const TX_64X64: i32 = 4;
pub const TX_4X8: i32 = 5;
pub const TX_8X4: i32 = 6;
pub const TX_8X16: i32 = 7;
pub const TX_16X8: i32 = 8;
pub const TX_16X32: i32 = 9;
pub const TX_32X16: i32 = 10;
pub const TX_32X64: i32 = 11;
pub const TX_64X32: i32 = 12;
pub const TX_4X16: i32 = 13;
pub const TX_16X4: i32 = 14;
pub const TX_8X32: i32 = 15;
pub const TX_32X8: i32 = 16;
pub const TX_16X64: i32 = 17;
pub const TX_64X16: i32 = 18;
pub const TX_SIZES_ALL: i32 = 19;
pub const TX_SIZES: i32 = TX_4X8;

pub const DCT_DCT: i32 = 0;
pub const ADST_DCT: i32 = 1;
pub const DCT_ADST: i32 = 2;
pub const ADST_ADST: i32 = 3;
pub const FLIPADST_DCT: i32 = 4;
pub const DCT_FLIPADST: i32 = 5;
pub const FLIPADST_FLIPADST: i32 = 6;
pub const ADST_FLIPADST: i32 = 7;
pub const FLIPADST_ADST: i32 = 8;
pub const IDTX: i32 = 9;
pub const V_DCT: i32 = 10;
pub const H_DCT: i32 = 11;
pub const V_ADST: i32 = 12;
pub const H_ADST: i32 = 13;
pub const V_FLIPADST: i32 = 14;
pub const H_FLIPADST: i32 = 15;
pub const TX_TYPES: i32 = 16;

// From av1/common/av1_inv_txfm1d_cfg.h.
pub const INV_COS_BIT: i32 = 12;

// From av1/common/av1_txfm.h.
const cos_bit_min: i32 = 10;
pub const NewSqrt2Bits: i32 = 12;
// 2^12 * sqrt(2)
pub const NewSqrt2: i32 = 5793;
// 2^12 / sqrt(2)
pub const NewInvSqrt2: i32 = 2896;

#[inline]
pub unsafe fn cospi_arr(n: i32) -> *const i32 {
    (&raw const av1_cospi_arr_data[(n - cos_bit_min) as usize]).cast::<i32>()
}

#[inline]
pub unsafe fn sinpi_arr(n: i32) -> *const i32 {
    (&raw const av1_sinpi_arr_data[(n - cos_bit_min) as usize]).cast::<i32>()
}

#[inline]
pub unsafe fn get_flip_cfg(tx_type: TX_TYPE, ud_flip: *mut i32, lr_flip: *mut i32) {
    match tx_type as i32 {
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

// Utility function that returns the log of the ratio of the col and row
// sizes.
#[inline]
pub fn get_rect_tx_log_ratio(col: i32, row: i32) -> i32 {
    if col == row {
        return 0;
    }
    if col > row {
        if col == row * 2 {
            return 1;
        }
        if col == row * 4 {
            return 2;
        }
        debug_assert!(false, "Unsupported transform size");
    } else {
        if row == col * 2 {
            return -1;
        }
        if row == col * 4 {
            return -2;
        }
        debug_assert!(false, "Unsupported transform size");
    }
    0 // Invalid
}

// From av1/common/common_data.h.
// Transform block width in log2
static tx_size_wide_log2: [i32; 19] = [2, 3, 4, 5, 6, 2, 3, 3, 4, 4, 5, 5, 6, 2, 4, 3, 5, 4, 6];
// Transform block height in log2
static tx_size_high_log2: [i32; 19] = [2, 3, 4, 5, 6, 3, 2, 4, 3, 5, 4, 6, 5, 4, 2, 5, 3, 6, 4];

#[inline]
pub fn get_txw_idx(tx_size: TX_SIZE) -> i32 {
    tx_size_wide_log2[tx_size as usize] - tx_size_wide_log2[0]
}

#[inline]
pub fn get_txh_idx(tx_size: TX_SIZE) -> i32 {
    tx_size_high_log2[tx_size as usize] - tx_size_high_log2[0]
}

// From av1/common/x86/av1_inv_txfm_ssse3.h.
macro_rules! btf_16_ssse3 {
    ($w0:expr, $w1:expr, $in:expr, $out0:expr, $out1:expr) => {{
        let _w0: __m128i = _mm_set1_epi16(($w0 * 8) as i16);
        let _w1: __m128i = _mm_set1_epi16(($w1 * 8) as i16);
        let _in: __m128i = $in;
        $out0 = _mm_mulhrs_epi16(_in, _w0);
        $out1 = _mm_mulhrs_epi16(_in, _w1);
    }};
}
pub(crate) use btf_16_ssse3;

macro_rules! btf_16_adds_subs_sse2 {
    ($in0:expr, $in1:expr) => {{
        let _in0: __m128i = $in0;
        let _in1: __m128i = $in1;
        $in0 = _mm_adds_epi16(_in0, _in1);
        $in1 = _mm_subs_epi16(_in0, _in1);
    }};
}
pub(crate) use btf_16_adds_subs_sse2;

macro_rules! btf_16_subs_adds_sse2 {
    ($in0:expr, $in1:expr) => {{
        let _in0: __m128i = $in0;
        let _in1: __m128i = $in1;
        $in1 = _mm_subs_epi16(_in0, _in1);
        $in0 = _mm_adds_epi16(_in0, _in1);
    }};
}
pub(crate) use btf_16_subs_adds_sse2;

macro_rules! btf_16_adds_subs_out_sse2 {
    ($out0:expr, $out1:expr, $in0:expr, $in1:expr) => {{
        let _in0: __m128i = $in0;
        let _in1: __m128i = $in1;
        $out0 = _mm_adds_epi16(_in0, _in1);
        $out1 = _mm_subs_epi16(_in0, _in1);
    }};
}
pub(crate) use btf_16_adds_subs_out_sse2;

#[inline]
#[target_feature(enable = "ssse3")]
pub unsafe fn round_shift_16bit_ssse3(in_: *mut __m128i, size: i32, bit: i32) {
    if bit < 0 {
        let scale: __m128i = _mm_set1_epi16((1 << (15 + bit)) as i16);
        for i in 0..size {
            *in_.offset(i as isize) = _mm_mulhrs_epi16(*in_.offset(i as isize), scale);
        }
    } else if bit > 0 {
        for i in 0..size {
            *in_.offset(i as isize) =
                _mm_sll_epi16(*in_.offset(i as isize), _mm_cvtsi32_si128(bit));
        }
    }
}

pub type transform_1d_ssse3 = Option<unsafe fn(*const __m128i, *mut __m128i)>;

// ---------------------------------------------------------------------------
// av1/common/x86/av1_inv_txfm_ssse3.c

// TODO(venkatsanampudi@ittiam.com): move this to header file
// Sqrt2, Sqrt2^2, Sqrt2^3, Sqrt2^4, Sqrt2^5
static NewSqrt2list: [i32; (TX_SIZES) as usize] = [5793, 2 * 4096, 2 * 5793, 4 * 4096, 4 * 5793];

// TODO(binpengsmail@gmail.com): replace some for loop with do {} while
#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn idct4_sse2(input: *const __m128i, output: *mut __m128i) {
    let cos_bit: i8 = INV_COS_BIT as i8;
    let mut cospi: *const i32 = cospi_arr(INV_COS_BIT);
    let __rounding: __m128i = _mm_set1_epi32(1 << (INV_COS_BIT - 1));
    let cospi_p32_p32: __m128i = pair_set_epi16(*cospi.offset(32), *cospi.offset(32));
    let cospi_p32_m32: __m128i = pair_set_epi16(*cospi.offset(32), -*cospi.offset(32));
    let cospi_p48_m16: __m128i = pair_set_epi16(*cospi.offset(48), -*cospi.offset(16));
    let cospi_p16_p48: __m128i = pair_set_epi16(*cospi.offset(16), *cospi.offset(48));
    // stage 1
    let mut x: [__m128i; 4] = core::mem::zeroed();
    x[0] = *input;
    x[1] = *input.offset(2);
    x[2] = *input.offset(1);
    x[3] = *input.offset(3);
    // stage 2
    btf_16_sse2!(
        cospi_p32_p32,
        cospi_p32_m32,
        x[0],
        x[1],
        x[0],
        x[1],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p48_m16,
        cospi_p16_p48,
        x[2],
        x[3],
        x[2],
        x[3],
        __rounding,
        cos_bit
    );
    // stage 3
    btf_16_adds_subs_out_sse2!(*output, *output.offset(3), x[0], x[3]);
    btf_16_adds_subs_out_sse2!(*output.offset(1), *output.offset(2), x[1], x[2]);
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn idct4_w4_sse2(input: *const __m128i, output: *mut __m128i) {
    let cos_bit: i8 = INV_COS_BIT as i8;
    let mut cospi: *const i32 = cospi_arr(INV_COS_BIT);
    let __rounding: __m128i = _mm_set1_epi32(1 << (INV_COS_BIT - 1));
    let cospi_p32_p32: __m128i = pair_set_epi16(*cospi.offset(32), *cospi.offset(32));
    let cospi_p32_m32: __m128i = pair_set_epi16(*cospi.offset(32), -*cospi.offset(32));
    let cospi_p48_m16: __m128i = pair_set_epi16(*cospi.offset(48), -*cospi.offset(16));
    let cospi_p16_p48: __m128i = pair_set_epi16(*cospi.offset(16), *cospi.offset(48));
    // stage 1
    let mut x: [__m128i; 4] = core::mem::zeroed();
    x[0] = *input;
    x[1] = *input.offset(2);
    x[2] = *input.offset(1);
    x[3] = *input.offset(3);
    // stage 2
    btf_16_4p_sse2!(
        cospi_p32_p32,
        cospi_p32_m32,
        x[0],
        x[1],
        x[0],
        x[1],
        __rounding,
        cos_bit
    );
    btf_16_4p_sse2!(
        cospi_p48_m16,
        cospi_p16_p48,
        x[2],
        x[3],
        x[2],
        x[3],
        __rounding,
        cos_bit
    );
    // stage 3
    btf_16_adds_subs_out_sse2!(*output, *output.offset(3), x[0], x[3]);
    btf_16_adds_subs_out_sse2!(*output.offset(1), *output.offset(2), x[1], x[2]);
}

#[target_feature(enable = "ssse3")]
pub unsafe fn av1_idct8_low1_ssse3(input: *const __m128i, output: *mut __m128i) {
    let mut cospi: *const i32 = cospi_arr(INV_COS_BIT);
    // stage 1
    let mut x: [__m128i; 2] = core::mem::zeroed();
    x[0] = *input;
    // stage 2
    // stage 3
    btf_16_ssse3!(*cospi.offset(32), *cospi.offset(32), x[0], x[0], x[1]);
    // stage 4
    // stage 5
    *output = x[0];
    *output.offset(7) = x[0];
    *output.offset(1) = x[1];
    *output.offset(6) = x[1];
    *output.offset(2) = x[1];
    *output.offset(5) = x[1];
    *output.offset(3) = x[0];
    *output.offset(4) = x[0];
}

#[target_feature(enable = "ssse3")]
pub unsafe fn av1_idct8_sse2(input: *const __m128i, output: *mut __m128i) {
    let cos_bit: i8 = INV_COS_BIT as i8;
    let mut cospi: *const i32 = cospi_arr(INV_COS_BIT);
    let __rounding: __m128i = _mm_set1_epi32(1 << (INV_COS_BIT - 1));
    let cospi_p56_m08: __m128i = pair_set_epi16(*cospi.offset(56), -*cospi.offset(8));
    let cospi_p08_p56: __m128i = pair_set_epi16(*cospi.offset(8), *cospi.offset(56));
    let cospi_p24_m40: __m128i = pair_set_epi16(*cospi.offset(24), -*cospi.offset(40));
    let cospi_p40_p24: __m128i = pair_set_epi16(*cospi.offset(40), *cospi.offset(24));
    let cospi_p32_p32: __m128i = pair_set_epi16(*cospi.offset(32), *cospi.offset(32));
    let cospi_p32_m32: __m128i = pair_set_epi16(*cospi.offset(32), -*cospi.offset(32));
    let cospi_p48_m16: __m128i = pair_set_epi16(*cospi.offset(48), -*cospi.offset(16));
    let cospi_p16_p48: __m128i = pair_set_epi16(*cospi.offset(16), *cospi.offset(48));
    let cospi_m32_p32: __m128i = pair_set_epi16(-*cospi.offset(32), *cospi.offset(32));
    // stage 1
    let mut x: [__m128i; 8] = core::mem::zeroed();
    x[0] = *input;
    x[1] = *input.offset(4);
    x[2] = *input.offset(2);
    x[3] = *input.offset(6);
    x[4] = *input.offset(1);
    x[5] = *input.offset(5);
    x[6] = *input.offset(3);
    x[7] = *input.offset(7);
    // stage 2
    btf_16_sse2!(
        cospi_p56_m08,
        cospi_p08_p56,
        x[4],
        x[7],
        x[4],
        x[7],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p24_m40,
        cospi_p40_p24,
        x[5],
        x[6],
        x[5],
        x[6],
        __rounding,
        cos_bit
    );
    // stage 3
    btf_16_sse2!(
        cospi_p32_p32,
        cospi_p32_m32,
        x[0],
        x[1],
        x[0],
        x[1],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p48_m16,
        cospi_p16_p48,
        x[2],
        x[3],
        x[2],
        x[3],
        __rounding,
        cos_bit
    );
    btf_16_adds_subs_sse2!(x[4], x[5]);
    btf_16_subs_adds_sse2!(x[7], x[6]);
    // stage 4
    btf_16_adds_subs_sse2!(x[0], x[3]);
    btf_16_adds_subs_sse2!(x[1], x[2]);
    btf_16_sse2!(
        cospi_m32_p32,
        cospi_p32_p32,
        x[5],
        x[6],
        x[5],
        x[6],
        __rounding,
        cos_bit
    );
    // stage 5
    btf_16_adds_subs_out_sse2!(*output, *output.offset(7), x[0], x[7]);
    btf_16_adds_subs_out_sse2!(*output.offset(1), *output.offset(6), x[1], x[6]);
    btf_16_adds_subs_out_sse2!(*output.offset(2), *output.offset(5), x[2], x[5]);
    btf_16_adds_subs_out_sse2!(*output.offset(3), *output.offset(4), x[3], x[4]);
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn idct8_w4_sse2(input: *const __m128i, output: *mut __m128i) {
    let cos_bit: i8 = INV_COS_BIT as i8;
    let mut cospi: *const i32 = cospi_arr(INV_COS_BIT);
    let __rounding: __m128i = _mm_set1_epi32(1 << (INV_COS_BIT - 1));
    let cospi_p56_m08: __m128i = pair_set_epi16(*cospi.offset(56), -*cospi.offset(8));
    let cospi_p08_p56: __m128i = pair_set_epi16(*cospi.offset(8), *cospi.offset(56));
    let cospi_p24_m40: __m128i = pair_set_epi16(*cospi.offset(24), -*cospi.offset(40));
    let cospi_p40_p24: __m128i = pair_set_epi16(*cospi.offset(40), *cospi.offset(24));
    let cospi_p32_p32: __m128i = pair_set_epi16(*cospi.offset(32), *cospi.offset(32));
    let cospi_p32_m32: __m128i = pair_set_epi16(*cospi.offset(32), -*cospi.offset(32));
    let cospi_p48_m16: __m128i = pair_set_epi16(*cospi.offset(48), -*cospi.offset(16));
    let cospi_p16_p48: __m128i = pair_set_epi16(*cospi.offset(16), *cospi.offset(48));
    let cospi_m32_p32: __m128i = pair_set_epi16(-*cospi.offset(32), *cospi.offset(32));
    // stage 1
    let mut x: [__m128i; 8] = core::mem::zeroed();
    x[0] = *input;
    x[1] = *input.offset(4);
    x[2] = *input.offset(2);
    x[3] = *input.offset(6);
    x[4] = *input.offset(1);
    x[5] = *input.offset(5);
    x[6] = *input.offset(3);
    x[7] = *input.offset(7);
    // stage 2
    btf_16_4p_sse2!(
        cospi_p56_m08,
        cospi_p08_p56,
        x[4],
        x[7],
        x[4],
        x[7],
        __rounding,
        cos_bit
    );
    btf_16_4p_sse2!(
        cospi_p24_m40,
        cospi_p40_p24,
        x[5],
        x[6],
        x[5],
        x[6],
        __rounding,
        cos_bit
    );
    // stage 3
    btf_16_4p_sse2!(
        cospi_p32_p32,
        cospi_p32_m32,
        x[0],
        x[1],
        x[0],
        x[1],
        __rounding,
        cos_bit
    );
    btf_16_4p_sse2!(
        cospi_p48_m16,
        cospi_p16_p48,
        x[2],
        x[3],
        x[2],
        x[3],
        __rounding,
        cos_bit
    );
    btf_16_adds_subs_sse2!(x[4], x[5]);
    btf_16_subs_adds_sse2!(x[7], x[6]);
    // stage 4
    btf_16_adds_subs_sse2!(x[0], x[3]);
    btf_16_adds_subs_sse2!(x[1], x[2]);
    btf_16_4p_sse2!(
        cospi_m32_p32,
        cospi_p32_p32,
        x[5],
        x[6],
        x[5],
        x[6],
        __rounding,
        cos_bit
    );
    // stage 5
    btf_16_adds_subs_out_sse2!(*output, *output.offset(7), x[0], x[7]);
    btf_16_adds_subs_out_sse2!(*output.offset(1), *output.offset(6), x[1], x[6]);
    btf_16_adds_subs_out_sse2!(*output.offset(2), *output.offset(5), x[2], x[5]);
    btf_16_adds_subs_out_sse2!(*output.offset(3), *output.offset(4), x[3], x[4]);
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn idct16_stage5_sse2(x: *mut __m128i, cospi: *const i32, __rounding: __m128i, cos_bit: i8) {
    let cospi_m32_p32: __m128i = pair_set_epi16(-*cospi.offset(32), *cospi.offset(32));
    let cospi_p32_p32: __m128i = pair_set_epi16(*cospi.offset(32), *cospi.offset(32));
    btf_16_adds_subs_sse2!(*x, *x.offset(3));
    btf_16_adds_subs_sse2!(*x.offset(1), *x.offset(2));
    btf_16_sse2!(
        cospi_m32_p32,
        cospi_p32_p32,
        *x.offset(5),
        *x.offset(6),
        *x.offset(5),
        *x.offset(6),
        __rounding,
        cos_bit
    );
    btf_16_adds_subs_sse2!(*x.offset(8), *x.offset(11));
    btf_16_adds_subs_sse2!(*x.offset(9), *x.offset(10));
    btf_16_subs_adds_sse2!(*x.offset(15), *x.offset(12));
    btf_16_subs_adds_sse2!(*x.offset(14), *x.offset(13));
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn idct16_stage6_sse2(x: *mut __m128i, cospi: *const i32, __rounding: __m128i, cos_bit: i8) {
    let cospi_m32_p32: __m128i = pair_set_epi16(-*cospi.offset(32), *cospi.offset(32));
    let cospi_p32_p32: __m128i = pair_set_epi16(*cospi.offset(32), *cospi.offset(32));
    btf_16_adds_subs_sse2!(*x, *x.offset(7));
    btf_16_adds_subs_sse2!(*x.offset(1), *x.offset(6));
    btf_16_adds_subs_sse2!(*x.offset(2), *x.offset(5));
    btf_16_adds_subs_sse2!(*x.offset(3), *x.offset(4));
    btf_16_sse2!(
        cospi_m32_p32,
        cospi_p32_p32,
        *x.offset(10),
        *x.offset(13),
        *x.offset(10),
        *x.offset(13),
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m32_p32,
        cospi_p32_p32,
        *x.offset(11),
        *x.offset(12),
        *x.offset(11),
        *x.offset(12),
        __rounding,
        cos_bit
    );
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn idct16_stage7_sse2(output: *mut __m128i, x: *mut __m128i) {
    btf_16_adds_subs_out_sse2!(*output, *output.offset(15), *x, *x.offset(15));
    btf_16_adds_subs_out_sse2!(
        *output.offset(1),
        *output.offset(14),
        *x.offset(1),
        *x.offset(14)
    );
    btf_16_adds_subs_out_sse2!(
        *output.offset(2),
        *output.offset(13),
        *x.offset(2),
        *x.offset(13)
    );
    btf_16_adds_subs_out_sse2!(
        *output.offset(3),
        *output.offset(12),
        *x.offset(3),
        *x.offset(12)
    );
    btf_16_adds_subs_out_sse2!(
        *output.offset(4),
        *output.offset(11),
        *x.offset(4),
        *x.offset(11)
    );
    btf_16_adds_subs_out_sse2!(
        *output.offset(5),
        *output.offset(10),
        *x.offset(5),
        *x.offset(10)
    );
    btf_16_adds_subs_out_sse2!(
        *output.offset(6),
        *output.offset(9),
        *x.offset(6),
        *x.offset(9)
    );
    btf_16_adds_subs_out_sse2!(
        *output.offset(7),
        *output.offset(8),
        *x.offset(7),
        *x.offset(8)
    );
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn idct16_low1_ssse3(input: *const __m128i, output: *mut __m128i) {
    let mut cospi: *const i32 = cospi_arr(INV_COS_BIT);
    // stage 1
    let mut x: [__m128i; 2] = core::mem::zeroed();
    x[0] = *input;
    // stage 2
    // stage 3
    // stage 4
    btf_16_ssse3!(*cospi.offset(32), *cospi.offset(32), x[0], x[0], x[1]);
    // stage 5
    // stage 6
    // stage 7
    *output = x[0];
    *output.offset(15) = x[0];
    *output.offset(1) = x[1];
    *output.offset(14) = x[1];
    *output.offset(2) = x[1];
    *output.offset(13) = x[1];
    *output.offset(3) = x[0];
    *output.offset(12) = x[0];
    *output.offset(4) = x[0];
    *output.offset(11) = x[0];
    *output.offset(5) = x[1];
    *output.offset(10) = x[1];
    *output.offset(6) = x[1];
    *output.offset(9) = x[1];
    *output.offset(7) = x[0];
    *output.offset(8) = x[0];
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn idct16_low8_ssse3(input: *const __m128i, output: *mut __m128i) {
    let cos_bit: i8 = INV_COS_BIT as i8;
    let mut cospi: *const i32 = cospi_arr(INV_COS_BIT);
    let __rounding: __m128i = _mm_set1_epi32(1 << (INV_COS_BIT - 1));
    let cospi_m16_p48: __m128i = pair_set_epi16(-*cospi.offset(16), *cospi.offset(48));
    let cospi_p48_p16: __m128i = pair_set_epi16(*cospi.offset(48), *cospi.offset(16));
    let cospi_m48_m16: __m128i = pair_set_epi16(-*cospi.offset(48), -*cospi.offset(16));
    // stage 1
    let mut x: [__m128i; 16] = core::mem::zeroed();
    x[0] = *input;
    x[2] = *input.offset(4);
    x[4] = *input.offset(2);
    x[6] = *input.offset(6);
    x[8] = *input.offset(1);
    x[10] = *input.offset(5);
    x[12] = *input.offset(3);
    x[14] = *input.offset(7);
    // stage 2
    btf_16_ssse3!(*cospi.offset(60), *cospi.offset(4), x[8], x[8], x[15]);
    btf_16_ssse3!(-*cospi.offset(36), *cospi.offset(28), x[14], x[9], x[14]);
    btf_16_ssse3!(*cospi.offset(44), *cospi.offset(20), x[10], x[10], x[13]);
    btf_16_ssse3!(-*cospi.offset(52), *cospi.offset(12), x[12], x[11], x[12]);
    // stage 3
    btf_16_ssse3!(*cospi.offset(56), *cospi.offset(8), x[4], x[4], x[7]);
    btf_16_ssse3!(-*cospi.offset(40), *cospi.offset(24), x[6], x[5], x[6]);
    btf_16_adds_subs_sse2!(x[8], x[9]);
    btf_16_subs_adds_sse2!(x[11], x[10]);
    btf_16_adds_subs_sse2!(x[12], x[13]);
    btf_16_subs_adds_sse2!(x[15], x[14]);
    // stage 4
    btf_16_ssse3!(*cospi.offset(32), *cospi.offset(32), x[0], x[0], x[1]);
    btf_16_ssse3!(*cospi.offset(48), *cospi.offset(16), x[2], x[2], x[3]);
    btf_16_adds_subs_sse2!(x[4], x[5]);
    btf_16_subs_adds_sse2!(x[7], x[6]);
    btf_16_sse2!(
        cospi_m16_p48,
        cospi_p48_p16,
        x[9],
        x[14],
        x[9],
        x[14],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m48_m16,
        cospi_m16_p48,
        x[10],
        x[13],
        x[10],
        x[13],
        __rounding,
        cos_bit
    );
    idct16_stage5_sse2(x.as_mut_ptr(), cospi, __rounding, cos_bit);
    idct16_stage6_sse2(x.as_mut_ptr(), cospi, __rounding, cos_bit);
    idct16_stage7_sse2(output, x.as_mut_ptr());
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn idct16_sse2(input: *const __m128i, output: *mut __m128i) {
    let cos_bit: i8 = INV_COS_BIT as i8;
    let mut cospi: *const i32 = cospi_arr(INV_COS_BIT);
    let __rounding: __m128i = _mm_set1_epi32(1 << (INV_COS_BIT - 1));
    let cospi_p60_m04: __m128i = pair_set_epi16(*cospi.offset(60), -*cospi.offset(4));
    let cospi_p04_p60: __m128i = pair_set_epi16(*cospi.offset(4), *cospi.offset(60));
    let cospi_p28_m36: __m128i = pair_set_epi16(*cospi.offset(28), -*cospi.offset(36));
    let cospi_p36_p28: __m128i = pair_set_epi16(*cospi.offset(36), *cospi.offset(28));
    let cospi_p44_m20: __m128i = pair_set_epi16(*cospi.offset(44), -*cospi.offset(20));
    let cospi_p20_p44: __m128i = pair_set_epi16(*cospi.offset(20), *cospi.offset(44));
    let cospi_p12_m52: __m128i = pair_set_epi16(*cospi.offset(12), -*cospi.offset(52));
    let cospi_p52_p12: __m128i = pair_set_epi16(*cospi.offset(52), *cospi.offset(12));
    let cospi_p56_m08: __m128i = pair_set_epi16(*cospi.offset(56), -*cospi.offset(8));
    let cospi_p08_p56: __m128i = pair_set_epi16(*cospi.offset(8), *cospi.offset(56));
    let cospi_p24_m40: __m128i = pair_set_epi16(*cospi.offset(24), -*cospi.offset(40));
    let cospi_p40_p24: __m128i = pair_set_epi16(*cospi.offset(40), *cospi.offset(24));
    let cospi_p32_p32: __m128i = pair_set_epi16(*cospi.offset(32), *cospi.offset(32));
    let cospi_p32_m32: __m128i = pair_set_epi16(*cospi.offset(32), -*cospi.offset(32));
    let cospi_p48_m16: __m128i = pair_set_epi16(*cospi.offset(48), -*cospi.offset(16));
    let cospi_p16_p48: __m128i = pair_set_epi16(*cospi.offset(16), *cospi.offset(48));
    let cospi_m16_p48: __m128i = pair_set_epi16(-*cospi.offset(16), *cospi.offset(48));
    let cospi_p48_p16: __m128i = pair_set_epi16(*cospi.offset(48), *cospi.offset(16));
    let cospi_m48_m16: __m128i = pair_set_epi16(-*cospi.offset(48), -*cospi.offset(16));
    // stage 1
    let mut x: [__m128i; 16] = core::mem::zeroed();
    x[0] = *input;
    x[1] = *input.offset(8);
    x[2] = *input.offset(4);
    x[3] = *input.offset(12);
    x[4] = *input.offset(2);
    x[5] = *input.offset(10);
    x[6] = *input.offset(6);
    x[7] = *input.offset(14);
    x[8] = *input.offset(1);
    x[9] = *input.offset(9);
    x[10] = *input.offset(5);
    x[11] = *input.offset(13);
    x[12] = *input.offset(3);
    x[13] = *input.offset(11);
    x[14] = *input.offset(7);
    x[15] = *input.offset(15);
    // stage 2
    btf_16_sse2!(
        cospi_p60_m04,
        cospi_p04_p60,
        x[8],
        x[15],
        x[8],
        x[15],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p28_m36,
        cospi_p36_p28,
        x[9],
        x[14],
        x[9],
        x[14],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p44_m20,
        cospi_p20_p44,
        x[10],
        x[13],
        x[10],
        x[13],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p12_m52,
        cospi_p52_p12,
        x[11],
        x[12],
        x[11],
        x[12],
        __rounding,
        cos_bit
    );
    // stage 3
    btf_16_sse2!(
        cospi_p56_m08,
        cospi_p08_p56,
        x[4],
        x[7],
        x[4],
        x[7],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p24_m40,
        cospi_p40_p24,
        x[5],
        x[6],
        x[5],
        x[6],
        __rounding,
        cos_bit
    );
    btf_16_adds_subs_sse2!(x[8], x[9]);
    btf_16_subs_adds_sse2!(x[11], x[10]);
    btf_16_adds_subs_sse2!(x[12], x[13]);
    btf_16_subs_adds_sse2!(x[15], x[14]);
    // stage 4
    btf_16_sse2!(
        cospi_p32_p32,
        cospi_p32_m32,
        x[0],
        x[1],
        x[0],
        x[1],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p48_m16,
        cospi_p16_p48,
        x[2],
        x[3],
        x[2],
        x[3],
        __rounding,
        cos_bit
    );
    btf_16_adds_subs_sse2!(x[4], x[5]);
    btf_16_subs_adds_sse2!(x[7], x[6]);
    btf_16_sse2!(
        cospi_m16_p48,
        cospi_p48_p16,
        x[9],
        x[14],
        x[9],
        x[14],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m48_m16,
        cospi_m16_p48,
        x[10],
        x[13],
        x[10],
        x[13],
        __rounding,
        cos_bit
    );
    // stage 5~7
    idct16_stage5_sse2(x.as_mut_ptr(), cospi, __rounding, cos_bit);
    idct16_stage6_sse2(x.as_mut_ptr(), cospi, __rounding, cos_bit);
    idct16_stage7_sse2(output, x.as_mut_ptr());
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn idct16_w4_sse2(input: *const __m128i, output: *mut __m128i) {
    let cos_bit: i8 = INV_COS_BIT as i8;
    let mut cospi: *const i32 = cospi_arr(INV_COS_BIT);
    let __rounding: __m128i = _mm_set1_epi32(1 << (INV_COS_BIT - 1));
    let cospi_p60_m04: __m128i = pair_set_epi16(*cospi.offset(60), -*cospi.offset(4));
    let cospi_p04_p60: __m128i = pair_set_epi16(*cospi.offset(4), *cospi.offset(60));
    let cospi_p28_m36: __m128i = pair_set_epi16(*cospi.offset(28), -*cospi.offset(36));
    let cospi_p36_p28: __m128i = pair_set_epi16(*cospi.offset(36), *cospi.offset(28));
    let cospi_p44_m20: __m128i = pair_set_epi16(*cospi.offset(44), -*cospi.offset(20));
    let cospi_p20_p44: __m128i = pair_set_epi16(*cospi.offset(20), *cospi.offset(44));
    let cospi_p12_m52: __m128i = pair_set_epi16(*cospi.offset(12), -*cospi.offset(52));
    let cospi_p52_p12: __m128i = pair_set_epi16(*cospi.offset(52), *cospi.offset(12));
    let cospi_p56_m08: __m128i = pair_set_epi16(*cospi.offset(56), -*cospi.offset(8));
    let cospi_p08_p56: __m128i = pair_set_epi16(*cospi.offset(8), *cospi.offset(56));
    let cospi_p24_m40: __m128i = pair_set_epi16(*cospi.offset(24), -*cospi.offset(40));
    let cospi_p40_p24: __m128i = pair_set_epi16(*cospi.offset(40), *cospi.offset(24));
    let cospi_p32_p32: __m128i = pair_set_epi16(*cospi.offset(32), *cospi.offset(32));
    let cospi_p32_m32: __m128i = pair_set_epi16(*cospi.offset(32), -*cospi.offset(32));
    let cospi_p48_m16: __m128i = pair_set_epi16(*cospi.offset(48), -*cospi.offset(16));
    let cospi_p16_p48: __m128i = pair_set_epi16(*cospi.offset(16), *cospi.offset(48));
    let cospi_m16_p48: __m128i = pair_set_epi16(-*cospi.offset(16), *cospi.offset(48));
    let cospi_p48_p16: __m128i = pair_set_epi16(*cospi.offset(48), *cospi.offset(16));
    let cospi_m48_m16: __m128i = pair_set_epi16(-*cospi.offset(48), -*cospi.offset(16));
    let cospi_m32_p32: __m128i = pair_set_epi16(-*cospi.offset(32), *cospi.offset(32));
    // stage 1
    let mut x: [__m128i; 16] = core::mem::zeroed();
    x[0] = *input;
    x[1] = *input.offset(8);
    x[2] = *input.offset(4);
    x[3] = *input.offset(12);
    x[4] = *input.offset(2);
    x[5] = *input.offset(10);
    x[6] = *input.offset(6);
    x[7] = *input.offset(14);
    x[8] = *input.offset(1);
    x[9] = *input.offset(9);
    x[10] = *input.offset(5);
    x[11] = *input.offset(13);
    x[12] = *input.offset(3);
    x[13] = *input.offset(11);
    x[14] = *input.offset(7);
    x[15] = *input.offset(15);
    // stage 2
    btf_16_4p_sse2!(
        cospi_p60_m04,
        cospi_p04_p60,
        x[8],
        x[15],
        x[8],
        x[15],
        __rounding,
        cos_bit
    );
    btf_16_4p_sse2!(
        cospi_p28_m36,
        cospi_p36_p28,
        x[9],
        x[14],
        x[9],
        x[14],
        __rounding,
        cos_bit
    );
    btf_16_4p_sse2!(
        cospi_p44_m20,
        cospi_p20_p44,
        x[10],
        x[13],
        x[10],
        x[13],
        __rounding,
        cos_bit
    );
    btf_16_4p_sse2!(
        cospi_p12_m52,
        cospi_p52_p12,
        x[11],
        x[12],
        x[11],
        x[12],
        __rounding,
        cos_bit
    );
    // stage 3
    btf_16_4p_sse2!(
        cospi_p56_m08,
        cospi_p08_p56,
        x[4],
        x[7],
        x[4],
        x[7],
        __rounding,
        cos_bit
    );
    btf_16_4p_sse2!(
        cospi_p24_m40,
        cospi_p40_p24,
        x[5],
        x[6],
        x[5],
        x[6],
        __rounding,
        cos_bit
    );
    btf_16_adds_subs_sse2!(x[8], x[9]);
    btf_16_subs_adds_sse2!(x[11], x[10]);
    btf_16_adds_subs_sse2!(x[12], x[13]);
    btf_16_subs_adds_sse2!(x[15], x[14]);
    // stage 4
    btf_16_4p_sse2!(
        cospi_p32_p32,
        cospi_p32_m32,
        x[0],
        x[1],
        x[0],
        x[1],
        __rounding,
        cos_bit
    );
    btf_16_4p_sse2!(
        cospi_p48_m16,
        cospi_p16_p48,
        x[2],
        x[3],
        x[2],
        x[3],
        __rounding,
        cos_bit
    );
    btf_16_adds_subs_sse2!(x[4], x[5]);
    btf_16_subs_adds_sse2!(x[7], x[6]);
    btf_16_4p_sse2!(
        cospi_m16_p48,
        cospi_p48_p16,
        x[9],
        x[14],
        x[9],
        x[14],
        __rounding,
        cos_bit
    );
    btf_16_4p_sse2!(
        cospi_m48_m16,
        cospi_m16_p48,
        x[10],
        x[13],
        x[10],
        x[13],
        __rounding,
        cos_bit
    );
    // stage 5
    btf_16_adds_subs_sse2!(x[0], x[3]);
    btf_16_adds_subs_sse2!(x[1], x[2]);
    btf_16_4p_sse2!(
        cospi_m32_p32,
        cospi_p32_p32,
        x[5],
        x[6],
        x[5],
        x[6],
        __rounding,
        cos_bit
    );
    btf_16_adds_subs_sse2!(x[8], x[11]);
    btf_16_adds_subs_sse2!(x[9], x[10]);
    btf_16_subs_adds_sse2!(x[15], x[12]);
    btf_16_subs_adds_sse2!(x[14], x[13]);
    // stage 6
    btf_16_adds_subs_sse2!(x[0], x[7]);
    btf_16_adds_subs_sse2!(x[1], x[6]);
    btf_16_adds_subs_sse2!(x[2], x[5]);
    btf_16_adds_subs_sse2!(x[3], x[4]);
    btf_16_4p_sse2!(
        cospi_m32_p32,
        cospi_p32_p32,
        x[10],
        x[13],
        x[10],
        x[13],
        __rounding,
        cos_bit
    );
    btf_16_4p_sse2!(
        cospi_m32_p32,
        cospi_p32_p32,
        x[11],
        x[12],
        x[11],
        x[12],
        __rounding,
        cos_bit
    );
    // stage 7
    idct16_stage7_sse2(output, x.as_mut_ptr());
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn idct32_high16_stage3_sse2(x: *mut __m128i) {
    btf_16_adds_subs_sse2!(*x.offset(16), *x.offset(17));
    btf_16_subs_adds_sse2!(*x.offset(19), *x.offset(18));
    btf_16_adds_subs_sse2!(*x.offset(20), *x.offset(21));
    btf_16_subs_adds_sse2!(*x.offset(23), *x.offset(22));
    btf_16_adds_subs_sse2!(*x.offset(24), *x.offset(25));
    btf_16_subs_adds_sse2!(*x.offset(27), *x.offset(26));
    btf_16_adds_subs_sse2!(*x.offset(28), *x.offset(29));
    btf_16_subs_adds_sse2!(*x.offset(31), *x.offset(30));
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn idct32_high16_stage4_sse2(
    x: *mut __m128i,
    cospi: *const i32,
    __rounding: __m128i,
    cos_bit: i8,
) {
    let cospi_m08_p56: __m128i = pair_set_epi16(-*cospi.offset(8), *cospi.offset(56));
    let cospi_p56_p08: __m128i = pair_set_epi16(*cospi.offset(56), *cospi.offset(8));
    let cospi_m56_m08: __m128i = pair_set_epi16(-*cospi.offset(56), -*cospi.offset(8));
    let cospi_m40_p24: __m128i = pair_set_epi16(-*cospi.offset(40), *cospi.offset(24));
    let cospi_p24_p40: __m128i = pair_set_epi16(*cospi.offset(24), *cospi.offset(40));
    let cospi_m24_m40: __m128i = pair_set_epi16(-*cospi.offset(24), -*cospi.offset(40));
    btf_16_sse2!(
        cospi_m08_p56,
        cospi_p56_p08,
        *x.offset(17),
        *x.offset(30),
        *x.offset(17),
        *x.offset(30),
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m56_m08,
        cospi_m08_p56,
        *x.offset(18),
        *x.offset(29),
        *x.offset(18),
        *x.offset(29),
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m40_p24,
        cospi_p24_p40,
        *x.offset(21),
        *x.offset(26),
        *x.offset(21),
        *x.offset(26),
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m24_m40,
        cospi_m40_p24,
        *x.offset(22),
        *x.offset(25),
        *x.offset(22),
        *x.offset(25),
        __rounding,
        cos_bit
    );
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn idct32_high24_stage5_sse2(
    x: *mut __m128i,
    cospi: *const i32,
    __rounding: __m128i,
    cos_bit: i8,
) {
    let cospi_m16_p48: __m128i = pair_set_epi16(-*cospi.offset(16), *cospi.offset(48));
    let cospi_p48_p16: __m128i = pair_set_epi16(*cospi.offset(48), *cospi.offset(16));
    let cospi_m48_m16: __m128i = pair_set_epi16(-*cospi.offset(48), -*cospi.offset(16));
    btf_16_sse2!(
        cospi_m16_p48,
        cospi_p48_p16,
        *x.offset(9),
        *x.offset(14),
        *x.offset(9),
        *x.offset(14),
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m48_m16,
        cospi_m16_p48,
        *x.offset(10),
        *x.offset(13),
        *x.offset(10),
        *x.offset(13),
        __rounding,
        cos_bit
    );
    btf_16_adds_subs_sse2!(*x.offset(16), *x.offset(19));
    btf_16_adds_subs_sse2!(*x.offset(17), *x.offset(18));
    btf_16_subs_adds_sse2!(*x.offset(23), *x.offset(20));
    btf_16_subs_adds_sse2!(*x.offset(22), *x.offset(21));
    btf_16_adds_subs_sse2!(*x.offset(24), *x.offset(27));
    btf_16_adds_subs_sse2!(*x.offset(25), *x.offset(26));
    btf_16_subs_adds_sse2!(*x.offset(31), *x.offset(28));
    btf_16_subs_adds_sse2!(*x.offset(30), *x.offset(29));
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn idct32_high28_stage6_sse2(
    x: *mut __m128i,
    cospi: *const i32,
    __rounding: __m128i,
    cos_bit: i8,
) {
    let cospi_m32_p32: __m128i = pair_set_epi16(-*cospi.offset(32), *cospi.offset(32));
    let cospi_p32_p32: __m128i = pair_set_epi16(*cospi.offset(32), *cospi.offset(32));
    let cospi_m16_p48: __m128i = pair_set_epi16(-*cospi.offset(16), *cospi.offset(48));
    let cospi_p48_p16: __m128i = pair_set_epi16(*cospi.offset(48), *cospi.offset(16));
    let cospi_m48_m16: __m128i = pair_set_epi16(-*cospi.offset(48), -*cospi.offset(16));
    btf_16_sse2!(
        cospi_m32_p32,
        cospi_p32_p32,
        *x.offset(5),
        *x.offset(6),
        *x.offset(5),
        *x.offset(6),
        __rounding,
        cos_bit
    );
    btf_16_adds_subs_sse2!(*x.offset(8), *x.offset(11));
    btf_16_adds_subs_sse2!(*x.offset(9), *x.offset(10));
    btf_16_subs_adds_sse2!(*x.offset(15), *x.offset(12));
    btf_16_subs_adds_sse2!(*x.offset(14), *x.offset(13));
    btf_16_sse2!(
        cospi_m16_p48,
        cospi_p48_p16,
        *x.offset(18),
        *x.offset(29),
        *x.offset(18),
        *x.offset(29),
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m16_p48,
        cospi_p48_p16,
        *x.offset(19),
        *x.offset(28),
        *x.offset(19),
        *x.offset(28),
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m48_m16,
        cospi_m16_p48,
        *x.offset(20),
        *x.offset(27),
        *x.offset(20),
        *x.offset(27),
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m48_m16,
        cospi_m16_p48,
        *x.offset(21),
        *x.offset(26),
        *x.offset(21),
        *x.offset(26),
        __rounding,
        cos_bit
    );
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn idct32_stage7_sse2(x: *mut __m128i, cospi: *const i32, __rounding: __m128i, cos_bit: i8) {
    let cospi_m32_p32: __m128i = pair_set_epi16(-*cospi.offset(32), *cospi.offset(32));
    let cospi_p32_p32: __m128i = pair_set_epi16(*cospi.offset(32), *cospi.offset(32));
    btf_16_adds_subs_sse2!(*x, *x.offset(7));
    btf_16_adds_subs_sse2!(*x.offset(1), *x.offset(6));
    btf_16_adds_subs_sse2!(*x.offset(2), *x.offset(5));
    btf_16_adds_subs_sse2!(*x.offset(3), *x.offset(4));
    btf_16_sse2!(
        cospi_m32_p32,
        cospi_p32_p32,
        *x.offset(10),
        *x.offset(13),
        *x.offset(10),
        *x.offset(13),
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m32_p32,
        cospi_p32_p32,
        *x.offset(11),
        *x.offset(12),
        *x.offset(11),
        *x.offset(12),
        __rounding,
        cos_bit
    );
    btf_16_adds_subs_sse2!(*x.offset(16), *x.offset(23));
    btf_16_adds_subs_sse2!(*x.offset(17), *x.offset(22));
    btf_16_adds_subs_sse2!(*x.offset(18), *x.offset(21));
    btf_16_adds_subs_sse2!(*x.offset(19), *x.offset(20));
    btf_16_subs_adds_sse2!(*x.offset(31), *x.offset(24));
    btf_16_subs_adds_sse2!(*x.offset(30), *x.offset(25));
    btf_16_subs_adds_sse2!(*x.offset(29), *x.offset(26));
    btf_16_subs_adds_sse2!(*x.offset(28), *x.offset(27));
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn idct32_stage8_sse2(x: *mut __m128i, cospi: *const i32, __rounding: __m128i, cos_bit: i8) {
    let cospi_m32_p32: __m128i = pair_set_epi16(-*cospi.offset(32), *cospi.offset(32));
    let cospi_p32_p32: __m128i = pair_set_epi16(*cospi.offset(32), *cospi.offset(32));
    btf_16_adds_subs_sse2!(*x, *x.offset(15));
    btf_16_adds_subs_sse2!(*x.offset(1), *x.offset(14));
    btf_16_adds_subs_sse2!(*x.offset(2), *x.offset(13));
    btf_16_adds_subs_sse2!(*x.offset(3), *x.offset(12));
    btf_16_adds_subs_sse2!(*x.offset(4), *x.offset(11));
    btf_16_adds_subs_sse2!(*x.offset(5), *x.offset(10));
    btf_16_adds_subs_sse2!(*x.offset(6), *x.offset(9));
    btf_16_adds_subs_sse2!(*x.offset(7), *x.offset(8));
    btf_16_sse2!(
        cospi_m32_p32,
        cospi_p32_p32,
        *x.offset(20),
        *x.offset(27),
        *x.offset(20),
        *x.offset(27),
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m32_p32,
        cospi_p32_p32,
        *x.offset(21),
        *x.offset(26),
        *x.offset(21),
        *x.offset(26),
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m32_p32,
        cospi_p32_p32,
        *x.offset(22),
        *x.offset(25),
        *x.offset(22),
        *x.offset(25),
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m32_p32,
        cospi_p32_p32,
        *x.offset(23),
        *x.offset(24),
        *x.offset(23),
        *x.offset(24),
        __rounding,
        cos_bit
    );
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn idct32_stage9_sse2(output: *mut __m128i, x: *mut __m128i) {
    btf_16_adds_subs_out_sse2!(*output, *output.offset(31), *x, *x.offset(31));
    btf_16_adds_subs_out_sse2!(
        *output.offset(1),
        *output.offset(30),
        *x.offset(1),
        *x.offset(30)
    );
    btf_16_adds_subs_out_sse2!(
        *output.offset(2),
        *output.offset(29),
        *x.offset(2),
        *x.offset(29)
    );
    btf_16_adds_subs_out_sse2!(
        *output.offset(3),
        *output.offset(28),
        *x.offset(3),
        *x.offset(28)
    );
    btf_16_adds_subs_out_sse2!(
        *output.offset(4),
        *output.offset(27),
        *x.offset(4),
        *x.offset(27)
    );
    btf_16_adds_subs_out_sse2!(
        *output.offset(5),
        *output.offset(26),
        *x.offset(5),
        *x.offset(26)
    );
    btf_16_adds_subs_out_sse2!(
        *output.offset(6),
        *output.offset(25),
        *x.offset(6),
        *x.offset(25)
    );
    btf_16_adds_subs_out_sse2!(
        *output.offset(7),
        *output.offset(24),
        *x.offset(7),
        *x.offset(24)
    );
    btf_16_adds_subs_out_sse2!(
        *output.offset(8),
        *output.offset(23),
        *x.offset(8),
        *x.offset(23)
    );
    btf_16_adds_subs_out_sse2!(
        *output.offset(9),
        *output.offset(22),
        *x.offset(9),
        *x.offset(22)
    );
    btf_16_adds_subs_out_sse2!(
        *output.offset(10),
        *output.offset(21),
        *x.offset(10),
        *x.offset(21)
    );
    btf_16_adds_subs_out_sse2!(
        *output.offset(11),
        *output.offset(20),
        *x.offset(11),
        *x.offset(20)
    );
    btf_16_adds_subs_out_sse2!(
        *output.offset(12),
        *output.offset(19),
        *x.offset(12),
        *x.offset(19)
    );
    btf_16_adds_subs_out_sse2!(
        *output.offset(13),
        *output.offset(18),
        *x.offset(13),
        *x.offset(18)
    );
    btf_16_adds_subs_out_sse2!(
        *output.offset(14),
        *output.offset(17),
        *x.offset(14),
        *x.offset(17)
    );
    btf_16_adds_subs_out_sse2!(
        *output.offset(15),
        *output.offset(16),
        *x.offset(15),
        *x.offset(16)
    );
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn idct32_low1_ssse3(input: *const __m128i, output: *mut __m128i) {
    let mut cospi: *const i32 = cospi_arr(INV_COS_BIT);
    // stage 1
    let mut x: [__m128i; 2] = core::mem::zeroed();
    x[0] = *input;
    // stage 2
    // stage 3
    // stage 4
    // stage 5
    btf_16_ssse3!(*cospi.offset(32), *cospi.offset(32), x[0], x[0], x[1]);
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
#[target_feature(enable = "ssse3")]
unsafe fn idct32_low8_ssse3(input: *const __m128i, output: *mut __m128i) {
    let cos_bit: i8 = INV_COS_BIT as i8;
    let mut cospi: *const i32 = cospi_arr(INV_COS_BIT);
    let __rounding: __m128i = _mm_set1_epi32(1 << (INV_COS_BIT - 1));
    // stage 1
    let mut x: [__m128i; 32] = core::mem::zeroed();
    x[0] = *input;
    x[4] = *input.offset(4);
    x[8] = *input.offset(2);
    x[12] = *input.offset(6);
    x[16] = *input.offset(1);
    x[20] = *input.offset(5);
    x[24] = *input.offset(3);
    x[28] = *input.offset(7);
    // stage 2
    btf_16_ssse3!(*cospi.offset(62), *cospi.offset(2), x[16], x[16], x[31]);
    btf_16_ssse3!(-*cospi.offset(50), *cospi.offset(14), x[28], x[19], x[28]);
    btf_16_ssse3!(*cospi.offset(54), *cospi.offset(10), x[20], x[20], x[27]);
    btf_16_ssse3!(-*cospi.offset(58), *cospi.offset(6), x[24], x[23], x[24]);
    // stage 3
    btf_16_ssse3!(*cospi.offset(60), *cospi.offset(4), x[8], x[8], x[15]);
    btf_16_ssse3!(-*cospi.offset(52), *cospi.offset(12), x[12], x[11], x[12]);
    x[17] = x[16];
    x[18] = x[19];
    x[21] = x[20];
    x[22] = x[23];
    x[25] = x[24];
    x[26] = x[27];
    x[29] = x[28];
    x[30] = x[31];
    // stage 4
    btf_16_ssse3!(*cospi.offset(56), *cospi.offset(8), x[4], x[4], x[7]);
    x[9] = x[8];
    x[10] = x[11];
    x[13] = x[12];
    x[14] = x[15];
    idct32_high16_stage4_sse2(x.as_mut_ptr(), cospi, __rounding, cos_bit);
    // stage 5
    btf_16_ssse3!(*cospi.offset(32), *cospi.offset(32), x[0], x[0], x[1]);
    x[5] = x[4];
    x[6] = x[7];
    idct32_high24_stage5_sse2(x.as_mut_ptr(), cospi, __rounding, cos_bit);
    // stage 6
    x[3] = x[0];
    x[2] = x[1];
    idct32_high28_stage6_sse2(x.as_mut_ptr(), cospi, __rounding, cos_bit);
    idct32_stage7_sse2(x.as_mut_ptr(), cospi, __rounding, cos_bit);
    idct32_stage8_sse2(x.as_mut_ptr(), cospi, __rounding, cos_bit);
    idct32_stage9_sse2(output, x.as_mut_ptr());
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn idct32_low16_ssse3(input: *const __m128i, output: *mut __m128i) {
    let cos_bit: i8 = INV_COS_BIT as i8;
    let mut cospi: *const i32 = cospi_arr(INV_COS_BIT);
    let __rounding: __m128i = _mm_set1_epi32(1 << (INV_COS_BIT - 1));
    // stage 1
    let mut x: [__m128i; 32] = core::mem::zeroed();
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
    btf_16_ssse3!(*cospi.offset(62), *cospi.offset(2), x[16], x[16], x[31]);
    btf_16_ssse3!(-*cospi.offset(34), *cospi.offset(30), x[30], x[17], x[30]);
    btf_16_ssse3!(*cospi.offset(46), *cospi.offset(18), x[18], x[18], x[29]);
    btf_16_ssse3!(-*cospi.offset(50), *cospi.offset(14), x[28], x[19], x[28]);
    btf_16_ssse3!(*cospi.offset(54), *cospi.offset(10), x[20], x[20], x[27]);
    btf_16_ssse3!(-*cospi.offset(42), *cospi.offset(22), x[26], x[21], x[26]);
    btf_16_ssse3!(*cospi.offset(38), *cospi.offset(26), x[22], x[22], x[25]);
    btf_16_ssse3!(-*cospi.offset(58), *cospi.offset(6), x[24], x[23], x[24]);
    // stage 3
    btf_16_ssse3!(*cospi.offset(60), *cospi.offset(4), x[8], x[8], x[15]);
    btf_16_ssse3!(-*cospi.offset(36), *cospi.offset(28), x[14], x[9], x[14]);
    btf_16_ssse3!(*cospi.offset(44), *cospi.offset(20), x[10], x[10], x[13]);
    btf_16_ssse3!(-*cospi.offset(52), *cospi.offset(12), x[12], x[11], x[12]);
    idct32_high16_stage3_sse2(x.as_mut_ptr());
    // stage 4
    btf_16_ssse3!(*cospi.offset(56), *cospi.offset(8), x[4], x[4], x[7]);
    btf_16_ssse3!(-*cospi.offset(40), *cospi.offset(24), x[6], x[5], x[6]);
    btf_16_adds_subs_sse2!(x[8], x[9]);
    btf_16_subs_adds_sse2!(x[11], x[10]);
    btf_16_adds_subs_sse2!(x[12], x[13]);
    btf_16_subs_adds_sse2!(x[15], x[14]);
    idct32_high16_stage4_sse2(x.as_mut_ptr(), cospi, __rounding, cos_bit);
    // stage 5
    btf_16_ssse3!(*cospi.offset(32), *cospi.offset(32), x[0], x[0], x[1]);
    btf_16_ssse3!(*cospi.offset(48), *cospi.offset(16), x[2], x[2], x[3]);
    btf_16_adds_subs_sse2!(x[4], x[5]);
    btf_16_subs_adds_sse2!(x[7], x[6]);
    idct32_high24_stage5_sse2(x.as_mut_ptr(), cospi, __rounding, cos_bit);
    btf_16_adds_subs_sse2!(x[0], x[3]);
    btf_16_adds_subs_sse2!(x[1], x[2]);
    idct32_high28_stage6_sse2(x.as_mut_ptr(), cospi, __rounding, cos_bit);
    idct32_stage7_sse2(x.as_mut_ptr(), cospi, __rounding, cos_bit);
    idct32_stage8_sse2(x.as_mut_ptr(), cospi, __rounding, cos_bit);
    idct32_stage9_sse2(output, x.as_mut_ptr());
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn idct32_sse2(input: *const __m128i, output: *mut __m128i) {
    let cos_bit: i8 = INV_COS_BIT as i8;
    let mut cospi: *const i32 = cospi_arr(INV_COS_BIT);
    let __rounding: __m128i = _mm_set1_epi32(1 << (INV_COS_BIT - 1));
    let cospi_p62_m02: __m128i = pair_set_epi16(*cospi.offset(62), -*cospi.offset(2));
    let cospi_p02_p62: __m128i = pair_set_epi16(*cospi.offset(2), *cospi.offset(62));
    let cospi_p30_m34: __m128i = pair_set_epi16(*cospi.offset(30), -*cospi.offset(34));
    let cospi_p34_p30: __m128i = pair_set_epi16(*cospi.offset(34), *cospi.offset(30));
    let cospi_p46_m18: __m128i = pair_set_epi16(*cospi.offset(46), -*cospi.offset(18));
    let cospi_p18_p46: __m128i = pair_set_epi16(*cospi.offset(18), *cospi.offset(46));
    let cospi_p14_m50: __m128i = pair_set_epi16(*cospi.offset(14), -*cospi.offset(50));
    let cospi_p50_p14: __m128i = pair_set_epi16(*cospi.offset(50), *cospi.offset(14));
    let cospi_p54_m10: __m128i = pair_set_epi16(*cospi.offset(54), -*cospi.offset(10));
    let cospi_p10_p54: __m128i = pair_set_epi16(*cospi.offset(10), *cospi.offset(54));
    let cospi_p22_m42: __m128i = pair_set_epi16(*cospi.offset(22), -*cospi.offset(42));
    let cospi_p42_p22: __m128i = pair_set_epi16(*cospi.offset(42), *cospi.offset(22));
    let cospi_p38_m26: __m128i = pair_set_epi16(*cospi.offset(38), -*cospi.offset(26));
    let cospi_p26_p38: __m128i = pair_set_epi16(*cospi.offset(26), *cospi.offset(38));
    let cospi_p06_m58: __m128i = pair_set_epi16(*cospi.offset(6), -*cospi.offset(58));
    let cospi_p58_p06: __m128i = pair_set_epi16(*cospi.offset(58), *cospi.offset(6));
    let cospi_p60_m04: __m128i = pair_set_epi16(*cospi.offset(60), -*cospi.offset(4));
    let cospi_p04_p60: __m128i = pair_set_epi16(*cospi.offset(4), *cospi.offset(60));
    let cospi_p28_m36: __m128i = pair_set_epi16(*cospi.offset(28), -*cospi.offset(36));
    let cospi_p36_p28: __m128i = pair_set_epi16(*cospi.offset(36), *cospi.offset(28));
    let cospi_p44_m20: __m128i = pair_set_epi16(*cospi.offset(44), -*cospi.offset(20));
    let cospi_p20_p44: __m128i = pair_set_epi16(*cospi.offset(20), *cospi.offset(44));
    let cospi_p12_m52: __m128i = pair_set_epi16(*cospi.offset(12), -*cospi.offset(52));
    let cospi_p52_p12: __m128i = pair_set_epi16(*cospi.offset(52), *cospi.offset(12));
    let cospi_p56_m08: __m128i = pair_set_epi16(*cospi.offset(56), -*cospi.offset(8));
    let cospi_p08_p56: __m128i = pair_set_epi16(*cospi.offset(8), *cospi.offset(56));
    let cospi_p24_m40: __m128i = pair_set_epi16(*cospi.offset(24), -*cospi.offset(40));
    let cospi_p40_p24: __m128i = pair_set_epi16(*cospi.offset(40), *cospi.offset(24));
    let cospi_p32_p32: __m128i = pair_set_epi16(*cospi.offset(32), *cospi.offset(32));
    let cospi_p32_m32: __m128i = pair_set_epi16(*cospi.offset(32), -*cospi.offset(32));
    let cospi_p48_m16: __m128i = pair_set_epi16(*cospi.offset(48), -*cospi.offset(16));
    let cospi_p16_p48: __m128i = pair_set_epi16(*cospi.offset(16), *cospi.offset(48));
    // stage 1
    let mut x: [__m128i; 32] = core::mem::zeroed();
    x[0] = *input;
    x[1] = *input.offset(16);
    x[2] = *input.offset(8);
    x[3] = *input.offset(24);
    x[4] = *input.offset(4);
    x[5] = *input.offset(20);
    x[6] = *input.offset(12);
    x[7] = *input.offset(28);
    x[8] = *input.offset(2);
    x[9] = *input.offset(18);
    x[10] = *input.offset(10);
    x[11] = *input.offset(26);
    x[12] = *input.offset(6);
    x[13] = *input.offset(22);
    x[14] = *input.offset(14);
    x[15] = *input.offset(30);
    x[16] = *input.offset(1);
    x[17] = *input.offset(17);
    x[18] = *input.offset(9);
    x[19] = *input.offset(25);
    x[20] = *input.offset(5);
    x[21] = *input.offset(21);
    x[22] = *input.offset(13);
    x[23] = *input.offset(29);
    x[24] = *input.offset(3);
    x[25] = *input.offset(19);
    x[26] = *input.offset(11);
    x[27] = *input.offset(27);
    x[28] = *input.offset(7);
    x[29] = *input.offset(23);
    x[30] = *input.offset(15);
    x[31] = *input.offset(31);
    // stage 2
    btf_16_sse2!(
        cospi_p62_m02,
        cospi_p02_p62,
        x[16],
        x[31],
        x[16],
        x[31],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p30_m34,
        cospi_p34_p30,
        x[17],
        x[30],
        x[17],
        x[30],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p46_m18,
        cospi_p18_p46,
        x[18],
        x[29],
        x[18],
        x[29],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p14_m50,
        cospi_p50_p14,
        x[19],
        x[28],
        x[19],
        x[28],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p54_m10,
        cospi_p10_p54,
        x[20],
        x[27],
        x[20],
        x[27],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p22_m42,
        cospi_p42_p22,
        x[21],
        x[26],
        x[21],
        x[26],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p38_m26,
        cospi_p26_p38,
        x[22],
        x[25],
        x[22],
        x[25],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p06_m58,
        cospi_p58_p06,
        x[23],
        x[24],
        x[23],
        x[24],
        __rounding,
        cos_bit
    );
    // stage 3
    btf_16_sse2!(
        cospi_p60_m04,
        cospi_p04_p60,
        x[8],
        x[15],
        x[8],
        x[15],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p28_m36,
        cospi_p36_p28,
        x[9],
        x[14],
        x[9],
        x[14],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p44_m20,
        cospi_p20_p44,
        x[10],
        x[13],
        x[10],
        x[13],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p12_m52,
        cospi_p52_p12,
        x[11],
        x[12],
        x[11],
        x[12],
        __rounding,
        cos_bit
    );
    idct32_high16_stage3_sse2(x.as_mut_ptr());
    // stage 4
    btf_16_sse2!(
        cospi_p56_m08,
        cospi_p08_p56,
        x[4],
        x[7],
        x[4],
        x[7],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p24_m40,
        cospi_p40_p24,
        x[5],
        x[6],
        x[5],
        x[6],
        __rounding,
        cos_bit
    );
    btf_16_adds_subs_sse2!(x[8], x[9]);
    btf_16_subs_adds_sse2!(x[11], x[10]);
    btf_16_adds_subs_sse2!(x[12], x[13]);
    btf_16_subs_adds_sse2!(x[15], x[14]);
    idct32_high16_stage4_sse2(x.as_mut_ptr(), cospi, __rounding, cos_bit);
    // stage 5
    btf_16_sse2!(
        cospi_p32_p32,
        cospi_p32_m32,
        x[0],
        x[1],
        x[0],
        x[1],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p48_m16,
        cospi_p16_p48,
        x[2],
        x[3],
        x[2],
        x[3],
        __rounding,
        cos_bit
    );
    btf_16_adds_subs_sse2!(x[4], x[5]);
    btf_16_adds_subs_sse2!(x[7], x[6]);
    idct32_high24_stage5_sse2(x.as_mut_ptr(), cospi, __rounding, cos_bit);
    // stage 6
    btf_16_adds_subs_sse2!(x[0], x[3]);
    btf_16_adds_subs_sse2!(x[1], x[2]);
    idct32_high28_stage6_sse2(x.as_mut_ptr(), cospi, __rounding, cos_bit);
    // stage 7~8
    idct32_stage7_sse2(x.as_mut_ptr(), cospi, __rounding, cos_bit);
    idct32_stage8_sse2(x.as_mut_ptr(), cospi, __rounding, cos_bit);
    idct32_stage9_sse2(output, x.as_mut_ptr());
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn idct64_stage4_high32_sse2(
    x: *mut __m128i,
    cospi: *const i32,
    __rounding: __m128i,
    cos_bit: i8,
) {
    let cospi_m04_p60: __m128i = pair_set_epi16(-*cospi.offset(4), *cospi.offset(60));
    let cospi_p60_p04: __m128i = pair_set_epi16(*cospi.offset(60), *cospi.offset(4));
    let cospi_m60_m04: __m128i = pair_set_epi16(-*cospi.offset(60), -*cospi.offset(4));
    let cospi_m36_p28: __m128i = pair_set_epi16(-*cospi.offset(36), *cospi.offset(28));
    let cospi_p28_p36: __m128i = pair_set_epi16(*cospi.offset(28), *cospi.offset(36));
    let cospi_m28_m36: __m128i = pair_set_epi16(-*cospi.offset(28), -*cospi.offset(36));
    let cospi_m20_p44: __m128i = pair_set_epi16(-*cospi.offset(20), *cospi.offset(44));
    let cospi_p44_p20: __m128i = pair_set_epi16(*cospi.offset(44), *cospi.offset(20));
    let cospi_m44_m20: __m128i = pair_set_epi16(-*cospi.offset(44), -*cospi.offset(20));
    let cospi_m52_p12: __m128i = pair_set_epi16(-*cospi.offset(52), *cospi.offset(12));
    let cospi_p12_p52: __m128i = pair_set_epi16(*cospi.offset(12), *cospi.offset(52));
    let cospi_m12_m52: __m128i = pair_set_epi16(-*cospi.offset(12), -*cospi.offset(52));
    btf_16_sse2!(
        cospi_m04_p60,
        cospi_p60_p04,
        *x.offset(33),
        *x.offset(62),
        *x.offset(33),
        *x.offset(62),
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m60_m04,
        cospi_m04_p60,
        *x.offset(34),
        *x.offset(61),
        *x.offset(34),
        *x.offset(61),
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m36_p28,
        cospi_p28_p36,
        *x.offset(37),
        *x.offset(58),
        *x.offset(37),
        *x.offset(58),
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m28_m36,
        cospi_m36_p28,
        *x.offset(38),
        *x.offset(57),
        *x.offset(38),
        *x.offset(57),
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m20_p44,
        cospi_p44_p20,
        *x.offset(41),
        *x.offset(54),
        *x.offset(41),
        *x.offset(54),
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m44_m20,
        cospi_m20_p44,
        *x.offset(42),
        *x.offset(53),
        *x.offset(42),
        *x.offset(53),
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m52_p12,
        cospi_p12_p52,
        *x.offset(45),
        *x.offset(50),
        *x.offset(45),
        *x.offset(50),
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m12_m52,
        cospi_m52_p12,
        *x.offset(46),
        *x.offset(49),
        *x.offset(46),
        *x.offset(49),
        __rounding,
        cos_bit
    );
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn idct64_stage5_high48_sse2(
    x: *mut __m128i,
    cospi: *const i32,
    __rounding: __m128i,
    cos_bit: i8,
) {
    let cospi_m08_p56: __m128i = pair_set_epi16(-*cospi.offset(8), *cospi.offset(56));
    let cospi_p56_p08: __m128i = pair_set_epi16(*cospi.offset(56), *cospi.offset(8));
    let cospi_m56_m08: __m128i = pair_set_epi16(-*cospi.offset(56), -*cospi.offset(8));
    let cospi_m40_p24: __m128i = pair_set_epi16(-*cospi.offset(40), *cospi.offset(24));
    let cospi_p24_p40: __m128i = pair_set_epi16(*cospi.offset(24), *cospi.offset(40));
    let cospi_m24_m40: __m128i = pair_set_epi16(-*cospi.offset(24), -*cospi.offset(40));
    btf_16_sse2!(
        cospi_m08_p56,
        cospi_p56_p08,
        *x.offset(17),
        *x.offset(30),
        *x.offset(17),
        *x.offset(30),
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m56_m08,
        cospi_m08_p56,
        *x.offset(18),
        *x.offset(29),
        *x.offset(18),
        *x.offset(29),
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m40_p24,
        cospi_p24_p40,
        *x.offset(21),
        *x.offset(26),
        *x.offset(21),
        *x.offset(26),
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m24_m40,
        cospi_m40_p24,
        *x.offset(22),
        *x.offset(25),
        *x.offset(22),
        *x.offset(25),
        __rounding,
        cos_bit
    );
    btf_16_adds_subs_sse2!(*x.offset(32), *x.offset(35));
    btf_16_adds_subs_sse2!(*x.offset(33), *x.offset(34));
    btf_16_subs_adds_sse2!(*x.offset(39), *x.offset(36));
    btf_16_subs_adds_sse2!(*x.offset(38), *x.offset(37));
    btf_16_adds_subs_sse2!(*x.offset(40), *x.offset(43));
    btf_16_adds_subs_sse2!(*x.offset(41), *x.offset(42));
    btf_16_subs_adds_sse2!(*x.offset(47), *x.offset(44));
    btf_16_subs_adds_sse2!(*x.offset(46), *x.offset(45));
    btf_16_adds_subs_sse2!(*x.offset(48), *x.offset(51));
    btf_16_adds_subs_sse2!(*x.offset(49), *x.offset(50));
    btf_16_subs_adds_sse2!(*x.offset(55), *x.offset(52));
    btf_16_subs_adds_sse2!(*x.offset(54), *x.offset(53));
    btf_16_adds_subs_sse2!(*x.offset(56), *x.offset(59));
    btf_16_adds_subs_sse2!(*x.offset(57), *x.offset(58));
    btf_16_subs_adds_sse2!(*x.offset(63), *x.offset(60));
    btf_16_subs_adds_sse2!(*x.offset(62), *x.offset(61));
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn idct64_stage6_high32_sse2(
    x: *mut __m128i,
    cospi: *const i32,
    __rounding: __m128i,
    cos_bit: i8,
) {
    let cospi_m08_p56: __m128i = pair_set_epi16(-*cospi.offset(8), *cospi.offset(56));
    let cospi_p56_p08: __m128i = pair_set_epi16(*cospi.offset(56), *cospi.offset(8));
    let cospi_m56_m08: __m128i = pair_set_epi16(-*cospi.offset(56), -*cospi.offset(8));
    let cospi_m40_p24: __m128i = pair_set_epi16(-*cospi.offset(40), *cospi.offset(24));
    let cospi_p24_p40: __m128i = pair_set_epi16(*cospi.offset(24), *cospi.offset(40));
    let cospi_m24_m40: __m128i = pair_set_epi16(-*cospi.offset(24), -*cospi.offset(40));
    btf_16_sse2!(
        cospi_m08_p56,
        cospi_p56_p08,
        *x.offset(34),
        *x.offset(61),
        *x.offset(34),
        *x.offset(61),
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m08_p56,
        cospi_p56_p08,
        *x.offset(35),
        *x.offset(60),
        *x.offset(35),
        *x.offset(60),
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m56_m08,
        cospi_m08_p56,
        *x.offset(36),
        *x.offset(59),
        *x.offset(36),
        *x.offset(59),
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m56_m08,
        cospi_m08_p56,
        *x.offset(37),
        *x.offset(58),
        *x.offset(37),
        *x.offset(58),
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m40_p24,
        cospi_p24_p40,
        *x.offset(42),
        *x.offset(53),
        *x.offset(42),
        *x.offset(53),
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m40_p24,
        cospi_p24_p40,
        *x.offset(43),
        *x.offset(52),
        *x.offset(43),
        *x.offset(52),
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m24_m40,
        cospi_m40_p24,
        *x.offset(44),
        *x.offset(51),
        *x.offset(44),
        *x.offset(51),
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m24_m40,
        cospi_m40_p24,
        *x.offset(45),
        *x.offset(50),
        *x.offset(45),
        *x.offset(50),
        __rounding,
        cos_bit
    );
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn idct64_stage6_high48_sse2(
    x: *mut __m128i,
    cospi: *const i32,
    __rounding: __m128i,
    cos_bit: i8,
) {
    btf_16_adds_subs_sse2!(*x.offset(16), *x.offset(19));
    btf_16_adds_subs_sse2!(*x.offset(17), *x.offset(18));
    btf_16_subs_adds_sse2!(*x.offset(23), *x.offset(20));
    btf_16_subs_adds_sse2!(*x.offset(22), *x.offset(21));
    btf_16_adds_subs_sse2!(*x.offset(24), *x.offset(27));
    btf_16_adds_subs_sse2!(*x.offset(25), *x.offset(26));
    btf_16_subs_adds_sse2!(*x.offset(31), *x.offset(28));
    btf_16_subs_adds_sse2!(*x.offset(30), *x.offset(29));
    idct64_stage6_high32_sse2(x, cospi, __rounding, cos_bit);
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn idct64_stage7_high48_sse2(
    x: *mut __m128i,
    cospi: *const i32,
    __rounding: __m128i,
    cos_bit: i8,
) {
    let cospi_m16_p48: __m128i = pair_set_epi16(-*cospi.offset(16), *cospi.offset(48));
    let cospi_p48_p16: __m128i = pair_set_epi16(*cospi.offset(48), *cospi.offset(16));
    let cospi_m48_m16: __m128i = pair_set_epi16(-*cospi.offset(48), -*cospi.offset(16));
    btf_16_sse2!(
        cospi_m16_p48,
        cospi_p48_p16,
        *x.offset(18),
        *x.offset(29),
        *x.offset(18),
        *x.offset(29),
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m16_p48,
        cospi_p48_p16,
        *x.offset(19),
        *x.offset(28),
        *x.offset(19),
        *x.offset(28),
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m48_m16,
        cospi_m16_p48,
        *x.offset(20),
        *x.offset(27),
        *x.offset(20),
        *x.offset(27),
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m48_m16,
        cospi_m16_p48,
        *x.offset(21),
        *x.offset(26),
        *x.offset(21),
        *x.offset(26),
        __rounding,
        cos_bit
    );
    btf_16_adds_subs_sse2!(*x.offset(32), *x.offset(39));
    btf_16_adds_subs_sse2!(*x.offset(33), *x.offset(38));
    btf_16_adds_subs_sse2!(*x.offset(34), *x.offset(37));
    btf_16_adds_subs_sse2!(*x.offset(35), *x.offset(36));
    btf_16_subs_adds_sse2!(*x.offset(47), *x.offset(40));
    btf_16_subs_adds_sse2!(*x.offset(46), *x.offset(41));
    btf_16_subs_adds_sse2!(*x.offset(45), *x.offset(42));
    btf_16_subs_adds_sse2!(*x.offset(44), *x.offset(43));
    btf_16_adds_subs_sse2!(*x.offset(48), *x.offset(55));
    btf_16_adds_subs_sse2!(*x.offset(49), *x.offset(54));
    btf_16_adds_subs_sse2!(*x.offset(50), *x.offset(53));
    btf_16_adds_subs_sse2!(*x.offset(51), *x.offset(52));
    btf_16_subs_adds_sse2!(*x.offset(63), *x.offset(56));
    btf_16_subs_adds_sse2!(*x.offset(62), *x.offset(57));
    btf_16_subs_adds_sse2!(*x.offset(61), *x.offset(58));
    btf_16_subs_adds_sse2!(*x.offset(60), *x.offset(59));
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn idct64_stage8_high48_sse2(
    x: *mut __m128i,
    cospi: *const i32,
    __rounding: __m128i,
    cos_bit: i8,
) {
    let cospi_m16_p48: __m128i = pair_set_epi16(-*cospi.offset(16), *cospi.offset(48));
    let cospi_p48_p16: __m128i = pair_set_epi16(*cospi.offset(48), *cospi.offset(16));
    let cospi_m48_m16: __m128i = pair_set_epi16(-*cospi.offset(48), -*cospi.offset(16));
    btf_16_adds_subs_sse2!(*x.offset(16), *x.offset(23));
    btf_16_adds_subs_sse2!(*x.offset(17), *x.offset(22));
    btf_16_adds_subs_sse2!(*x.offset(18), *x.offset(21));
    btf_16_adds_subs_sse2!(*x.offset(19), *x.offset(20));
    btf_16_subs_adds_sse2!(*x.offset(31), *x.offset(24));
    btf_16_subs_adds_sse2!(*x.offset(30), *x.offset(25));
    btf_16_subs_adds_sse2!(*x.offset(29), *x.offset(26));
    btf_16_subs_adds_sse2!(*x.offset(28), *x.offset(27));
    btf_16_sse2!(
        cospi_m16_p48,
        cospi_p48_p16,
        *x.offset(36),
        *x.offset(59),
        *x.offset(36),
        *x.offset(59),
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m16_p48,
        cospi_p48_p16,
        *x.offset(37),
        *x.offset(58),
        *x.offset(37),
        *x.offset(58),
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m16_p48,
        cospi_p48_p16,
        *x.offset(38),
        *x.offset(57),
        *x.offset(38),
        *x.offset(57),
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m16_p48,
        cospi_p48_p16,
        *x.offset(39),
        *x.offset(56),
        *x.offset(39),
        *x.offset(56),
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m48_m16,
        cospi_m16_p48,
        *x.offset(40),
        *x.offset(55),
        *x.offset(40),
        *x.offset(55),
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m48_m16,
        cospi_m16_p48,
        *x.offset(41),
        *x.offset(54),
        *x.offset(41),
        *x.offset(54),
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m48_m16,
        cospi_m16_p48,
        *x.offset(42),
        *x.offset(53),
        *x.offset(42),
        *x.offset(53),
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m48_m16,
        cospi_m16_p48,
        *x.offset(43),
        *x.offset(52),
        *x.offset(43),
        *x.offset(52),
        __rounding,
        cos_bit
    );
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn idct64_stage9_sse2(x: *mut __m128i, cospi: *const i32, __rounding: __m128i, cos_bit: i8) {
    let cospi_m32_p32: __m128i = pair_set_epi16(-*cospi.offset(32), *cospi.offset(32));
    let cospi_p32_p32: __m128i = pair_set_epi16(*cospi.offset(32), *cospi.offset(32));
    btf_16_adds_subs_sse2!(*x, *x.offset(15));
    btf_16_adds_subs_sse2!(*x.offset(1), *x.offset(14));
    btf_16_adds_subs_sse2!(*x.offset(2), *x.offset(13));
    btf_16_adds_subs_sse2!(*x.offset(3), *x.offset(12));
    btf_16_adds_subs_sse2!(*x.offset(4), *x.offset(11));
    btf_16_adds_subs_sse2!(*x.offset(5), *x.offset(10));
    btf_16_adds_subs_sse2!(*x.offset(6), *x.offset(9));
    btf_16_adds_subs_sse2!(*x.offset(7), *x.offset(8));
    btf_16_sse2!(
        cospi_m32_p32,
        cospi_p32_p32,
        *x.offset(20),
        *x.offset(27),
        *x.offset(20),
        *x.offset(27),
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m32_p32,
        cospi_p32_p32,
        *x.offset(21),
        *x.offset(26),
        *x.offset(21),
        *x.offset(26),
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m32_p32,
        cospi_p32_p32,
        *x.offset(22),
        *x.offset(25),
        *x.offset(22),
        *x.offset(25),
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m32_p32,
        cospi_p32_p32,
        *x.offset(23),
        *x.offset(24),
        *x.offset(23),
        *x.offset(24),
        __rounding,
        cos_bit
    );
    btf_16_adds_subs_sse2!(*x.offset(32), *x.offset(47));
    btf_16_adds_subs_sse2!(*x.offset(33), *x.offset(46));
    btf_16_adds_subs_sse2!(*x.offset(34), *x.offset(45));
    btf_16_adds_subs_sse2!(*x.offset(35), *x.offset(44));
    btf_16_adds_subs_sse2!(*x.offset(36), *x.offset(43));
    btf_16_adds_subs_sse2!(*x.offset(37), *x.offset(42));
    btf_16_adds_subs_sse2!(*x.offset(38), *x.offset(41));
    btf_16_adds_subs_sse2!(*x.offset(39), *x.offset(40));
    btf_16_subs_adds_sse2!(*x.offset(63), *x.offset(48));
    btf_16_subs_adds_sse2!(*x.offset(62), *x.offset(49));
    btf_16_subs_adds_sse2!(*x.offset(61), *x.offset(50));
    btf_16_subs_adds_sse2!(*x.offset(60), *x.offset(51));
    btf_16_subs_adds_sse2!(*x.offset(59), *x.offset(52));
    btf_16_subs_adds_sse2!(*x.offset(58), *x.offset(53));
    btf_16_subs_adds_sse2!(*x.offset(57), *x.offset(54));
    btf_16_subs_adds_sse2!(*x.offset(56), *x.offset(55));
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn idct64_stage10_sse2(
    x: *mut __m128i,
    cospi: *const i32,
    __rounding: __m128i,
    cos_bit: i8,
) {
    let cospi_m32_p32: __m128i = pair_set_epi16(-*cospi.offset(32), *cospi.offset(32));
    let cospi_p32_p32: __m128i = pair_set_epi16(*cospi.offset(32), *cospi.offset(32));
    btf_16_adds_subs_sse2!(*x, *x.offset(31));
    btf_16_adds_subs_sse2!(*x.offset(1), *x.offset(30));
    btf_16_adds_subs_sse2!(*x.offset(2), *x.offset(29));
    btf_16_adds_subs_sse2!(*x.offset(3), *x.offset(28));
    btf_16_adds_subs_sse2!(*x.offset(4), *x.offset(27));
    btf_16_adds_subs_sse2!(*x.offset(5), *x.offset(26));
    btf_16_adds_subs_sse2!(*x.offset(6), *x.offset(25));
    btf_16_adds_subs_sse2!(*x.offset(7), *x.offset(24));
    btf_16_adds_subs_sse2!(*x.offset(8), *x.offset(23));
    btf_16_adds_subs_sse2!(*x.offset(9), *x.offset(22));
    btf_16_adds_subs_sse2!(*x.offset(10), *x.offset(21));
    btf_16_adds_subs_sse2!(*x.offset(11), *x.offset(20));
    btf_16_adds_subs_sse2!(*x.offset(12), *x.offset(19));
    btf_16_adds_subs_sse2!(*x.offset(13), *x.offset(18));
    btf_16_adds_subs_sse2!(*x.offset(14), *x.offset(17));
    btf_16_adds_subs_sse2!(*x.offset(15), *x.offset(16));
    btf_16_sse2!(
        cospi_m32_p32,
        cospi_p32_p32,
        *x.offset(40),
        *x.offset(55),
        *x.offset(40),
        *x.offset(55),
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m32_p32,
        cospi_p32_p32,
        *x.offset(41),
        *x.offset(54),
        *x.offset(41),
        *x.offset(54),
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m32_p32,
        cospi_p32_p32,
        *x.offset(42),
        *x.offset(53),
        *x.offset(42),
        *x.offset(53),
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m32_p32,
        cospi_p32_p32,
        *x.offset(43),
        *x.offset(52),
        *x.offset(43),
        *x.offset(52),
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m32_p32,
        cospi_p32_p32,
        *x.offset(44),
        *x.offset(51),
        *x.offset(44),
        *x.offset(51),
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m32_p32,
        cospi_p32_p32,
        *x.offset(45),
        *x.offset(50),
        *x.offset(45),
        *x.offset(50),
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m32_p32,
        cospi_p32_p32,
        *x.offset(46),
        *x.offset(49),
        *x.offset(46),
        *x.offset(49),
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m32_p32,
        cospi_p32_p32,
        *x.offset(47),
        *x.offset(48),
        *x.offset(47),
        *x.offset(48),
        __rounding,
        cos_bit
    );
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn idct64_stage11_sse2(output: *mut __m128i, x: *mut __m128i) {
    btf_16_adds_subs_out_sse2!(*output, *output.offset(63), *x, *x.offset(63));
    btf_16_adds_subs_out_sse2!(
        *output.offset(1),
        *output.offset(62),
        *x.offset(1),
        *x.offset(62)
    );
    btf_16_adds_subs_out_sse2!(
        *output.offset(2),
        *output.offset(61),
        *x.offset(2),
        *x.offset(61)
    );
    btf_16_adds_subs_out_sse2!(
        *output.offset(3),
        *output.offset(60),
        *x.offset(3),
        *x.offset(60)
    );
    btf_16_adds_subs_out_sse2!(
        *output.offset(4),
        *output.offset(59),
        *x.offset(4),
        *x.offset(59)
    );
    btf_16_adds_subs_out_sse2!(
        *output.offset(5),
        *output.offset(58),
        *x.offset(5),
        *x.offset(58)
    );
    btf_16_adds_subs_out_sse2!(
        *output.offset(6),
        *output.offset(57),
        *x.offset(6),
        *x.offset(57)
    );
    btf_16_adds_subs_out_sse2!(
        *output.offset(7),
        *output.offset(56),
        *x.offset(7),
        *x.offset(56)
    );
    btf_16_adds_subs_out_sse2!(
        *output.offset(8),
        *output.offset(55),
        *x.offset(8),
        *x.offset(55)
    );
    btf_16_adds_subs_out_sse2!(
        *output.offset(9),
        *output.offset(54),
        *x.offset(9),
        *x.offset(54)
    );
    btf_16_adds_subs_out_sse2!(
        *output.offset(10),
        *output.offset(53),
        *x.offset(10),
        *x.offset(53)
    );
    btf_16_adds_subs_out_sse2!(
        *output.offset(11),
        *output.offset(52),
        *x.offset(11),
        *x.offset(52)
    );
    btf_16_adds_subs_out_sse2!(
        *output.offset(12),
        *output.offset(51),
        *x.offset(12),
        *x.offset(51)
    );
    btf_16_adds_subs_out_sse2!(
        *output.offset(13),
        *output.offset(50),
        *x.offset(13),
        *x.offset(50)
    );
    btf_16_adds_subs_out_sse2!(
        *output.offset(14),
        *output.offset(49),
        *x.offset(14),
        *x.offset(49)
    );
    btf_16_adds_subs_out_sse2!(
        *output.offset(15),
        *output.offset(48),
        *x.offset(15),
        *x.offset(48)
    );
    btf_16_adds_subs_out_sse2!(
        *output.offset(16),
        *output.offset(47),
        *x.offset(16),
        *x.offset(47)
    );
    btf_16_adds_subs_out_sse2!(
        *output.offset(17),
        *output.offset(46),
        *x.offset(17),
        *x.offset(46)
    );
    btf_16_adds_subs_out_sse2!(
        *output.offset(18),
        *output.offset(45),
        *x.offset(18),
        *x.offset(45)
    );
    btf_16_adds_subs_out_sse2!(
        *output.offset(19),
        *output.offset(44),
        *x.offset(19),
        *x.offset(44)
    );
    btf_16_adds_subs_out_sse2!(
        *output.offset(20),
        *output.offset(43),
        *x.offset(20),
        *x.offset(43)
    );
    btf_16_adds_subs_out_sse2!(
        *output.offset(21),
        *output.offset(42),
        *x.offset(21),
        *x.offset(42)
    );
    btf_16_adds_subs_out_sse2!(
        *output.offset(22),
        *output.offset(41),
        *x.offset(22),
        *x.offset(41)
    );
    btf_16_adds_subs_out_sse2!(
        *output.offset(23),
        *output.offset(40),
        *x.offset(23),
        *x.offset(40)
    );
    btf_16_adds_subs_out_sse2!(
        *output.offset(24),
        *output.offset(39),
        *x.offset(24),
        *x.offset(39)
    );
    btf_16_adds_subs_out_sse2!(
        *output.offset(25),
        *output.offset(38),
        *x.offset(25),
        *x.offset(38)
    );
    btf_16_adds_subs_out_sse2!(
        *output.offset(26),
        *output.offset(37),
        *x.offset(26),
        *x.offset(37)
    );
    btf_16_adds_subs_out_sse2!(
        *output.offset(27),
        *output.offset(36),
        *x.offset(27),
        *x.offset(36)
    );
    btf_16_adds_subs_out_sse2!(
        *output.offset(28),
        *output.offset(35),
        *x.offset(28),
        *x.offset(35)
    );
    btf_16_adds_subs_out_sse2!(
        *output.offset(29),
        *output.offset(34),
        *x.offset(29),
        *x.offset(34)
    );
    btf_16_adds_subs_out_sse2!(
        *output.offset(30),
        *output.offset(33),
        *x.offset(30),
        *x.offset(33)
    );
    btf_16_adds_subs_out_sse2!(
        *output.offset(31),
        *output.offset(32),
        *x.offset(31),
        *x.offset(32)
    );
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn idct64_low1_ssse3(input: *const __m128i, output: *mut __m128i) {
    let mut cospi: *const i32 = cospi_arr(INV_COS_BIT);
    // stage 1
    let mut x: [__m128i; 32] = core::mem::zeroed();
    x[0] = *input;
    // stage 2
    // stage 3
    // stage 4
    // stage 5
    // stage 6
    btf_16_ssse3!(*cospi.offset(32), *cospi.offset(32), x[0], x[0], x[1]);
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
#[target_feature(enable = "ssse3")]
unsafe fn idct64_low8_ssse3(input: *const __m128i, output: *mut __m128i) {
    let cos_bit: i8 = INV_COS_BIT as i8;
    let mut cospi: *const i32 = cospi_arr(INV_COS_BIT);
    let __rounding: __m128i = _mm_set1_epi32(1 << (INV_COS_BIT - 1));
    let cospi_m04_p60: __m128i = pair_set_epi16(-*cospi.offset(4), *cospi.offset(60));
    let cospi_p60_p04: __m128i = pair_set_epi16(*cospi.offset(60), *cospi.offset(4));
    let cospi_m36_p28: __m128i = pair_set_epi16(-*cospi.offset(36), *cospi.offset(28));
    let cospi_m28_m36: __m128i = pair_set_epi16(-*cospi.offset(28), -*cospi.offset(36));
    let cospi_m20_p44: __m128i = pair_set_epi16(-*cospi.offset(20), *cospi.offset(44));
    let cospi_p44_p20: __m128i = pair_set_epi16(*cospi.offset(44), *cospi.offset(20));
    let cospi_m52_p12: __m128i = pair_set_epi16(-*cospi.offset(52), *cospi.offset(12));
    let cospi_m12_m52: __m128i = pair_set_epi16(-*cospi.offset(12), -*cospi.offset(52));
    let cospi_m08_p56: __m128i = pair_set_epi16(-*cospi.offset(8), *cospi.offset(56));
    let cospi_p56_p08: __m128i = pair_set_epi16(*cospi.offset(56), *cospi.offset(8));
    let cospi_m40_p24: __m128i = pair_set_epi16(-*cospi.offset(40), *cospi.offset(24));
    let cospi_m24_m40: __m128i = pair_set_epi16(-*cospi.offset(24), -*cospi.offset(40));
    let cospi_p32_p32: __m128i = pair_set_epi16(*cospi.offset(32), *cospi.offset(32));
    let cospi_m16_p48: __m128i = pair_set_epi16(-*cospi.offset(16), *cospi.offset(48));
    let cospi_p48_p16: __m128i = pair_set_epi16(*cospi.offset(48), *cospi.offset(16));
    let cospi_m32_p32: __m128i = pair_set_epi16(-*cospi.offset(32), *cospi.offset(32));
    // stage 1
    let mut x: [__m128i; 64] = core::mem::zeroed();
    x[0] = *input;
    x[8] = *input.offset(4);
    x[16] = *input.offset(2);
    x[24] = *input.offset(6);
    x[32] = *input.offset(1);
    x[40] = *input.offset(5);
    x[48] = *input.offset(3);
    x[56] = *input.offset(7);
    // stage 2
    btf_16_ssse3!(*cospi.offset(63), *cospi.offset(1), x[32], x[32], x[63]);
    btf_16_ssse3!(-*cospi.offset(57), *cospi.offset(7), x[56], x[39], x[56]);
    btf_16_ssse3!(*cospi.offset(59), *cospi.offset(5), x[40], x[40], x[55]);
    btf_16_ssse3!(-*cospi.offset(61), *cospi.offset(3), x[48], x[47], x[48]);
    // stage 3
    btf_16_ssse3!(*cospi.offset(62), *cospi.offset(2), x[16], x[16], x[31]);
    btf_16_ssse3!(-*cospi.offset(58), *cospi.offset(6), x[24], x[23], x[24]);
    x[33] = x[32];
    x[38] = x[39];
    x[41] = x[40];
    x[46] = x[47];
    x[49] = x[48];
    x[54] = x[55];
    x[57] = x[56];
    x[62] = x[63];
    // stage 4
    btf_16_ssse3!(*cospi.offset(60), *cospi.offset(4), x[8], x[8], x[15]);
    x[17] = x[16];
    x[22] = x[23];
    x[25] = x[24];
    x[30] = x[31];
    btf_16_sse2!(
        cospi_m04_p60,
        cospi_p60_p04,
        x[33],
        x[62],
        x[33],
        x[62],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m28_m36,
        cospi_m36_p28,
        x[38],
        x[57],
        x[38],
        x[57],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m20_p44,
        cospi_p44_p20,
        x[41],
        x[54],
        x[41],
        x[54],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m12_m52,
        cospi_m52_p12,
        x[46],
        x[49],
        x[46],
        x[49],
        __rounding,
        cos_bit
    );
    // stage 5
    x[9] = x[8];
    x[14] = x[15];
    btf_16_sse2!(
        cospi_m08_p56,
        cospi_p56_p08,
        x[17],
        x[30],
        x[17],
        x[30],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m24_m40,
        cospi_m40_p24,
        x[22],
        x[25],
        x[22],
        x[25],
        __rounding,
        cos_bit
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
    btf_16_ssse3!(*cospi.offset(32), *cospi.offset(32), x[0], x[0], x[1]);
    btf_16_sse2!(
        cospi_m16_p48,
        cospi_p48_p16,
        x[9],
        x[14],
        x[9],
        x[14],
        __rounding,
        cos_bit
    );
    x[19] = x[16];
    x[18] = x[17];
    x[20] = x[23];
    x[21] = x[22];
    x[27] = x[24];
    x[26] = x[25];
    x[28] = x[31];
    x[29] = x[30];
    idct64_stage6_high32_sse2(x.as_mut_ptr(), cospi, __rounding, cos_bit);
    // stage 7
    x[3] = x[0];
    x[2] = x[1];
    x[11] = x[8];
    x[10] = x[9];
    x[12] = x[15];
    x[13] = x[14];
    idct64_stage7_high48_sse2(x.as_mut_ptr(), cospi, __rounding, cos_bit);
    // stage 8
    x[7] = x[0];
    x[6] = x[1];
    x[5] = x[2];
    x[4] = x[3];
    btf_16_sse2!(
        cospi_m32_p32,
        cospi_p32_p32,
        x[10],
        x[13],
        x[10],
        x[13],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m32_p32,
        cospi_p32_p32,
        x[11],
        x[12],
        x[11],
        x[12],
        __rounding,
        cos_bit
    );
    idct64_stage8_high48_sse2(x.as_mut_ptr(), cospi, __rounding, cos_bit);
    idct64_stage9_sse2(x.as_mut_ptr(), cospi, __rounding, cos_bit);
    idct64_stage10_sse2(x.as_mut_ptr(), cospi, __rounding, cos_bit);
    idct64_stage11_sse2(output, x.as_mut_ptr());
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn idct64_low16_ssse3(input: *const __m128i, output: *mut __m128i) {
    let cos_bit: i8 = INV_COS_BIT as i8;
    let mut cospi: *const i32 = cospi_arr(INV_COS_BIT);
    let __rounding: __m128i = _mm_set1_epi32(1 << (INV_COS_BIT - 1));
    let cospi_p32_p32: __m128i = pair_set_epi16(*cospi.offset(32), *cospi.offset(32));
    let cospi_m16_p48: __m128i = pair_set_epi16(-*cospi.offset(16), *cospi.offset(48));
    let cospi_p48_p16: __m128i = pair_set_epi16(*cospi.offset(48), *cospi.offset(16));
    let cospi_m48_m16: __m128i = pair_set_epi16(-*cospi.offset(48), -*cospi.offset(16));
    let cospi_m32_p32: __m128i = pair_set_epi16(-*cospi.offset(32), *cospi.offset(32));
    // stage 1
    let mut x: [__m128i; 64] = core::mem::zeroed();
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
    btf_16_ssse3!(*cospi.offset(63), *cospi.offset(1), x[32], x[32], x[63]);
    btf_16_ssse3!(-*cospi.offset(49), *cospi.offset(15), x[60], x[35], x[60]);
    btf_16_ssse3!(*cospi.offset(55), *cospi.offset(9), x[36], x[36], x[59]);
    btf_16_ssse3!(-*cospi.offset(57), *cospi.offset(7), x[56], x[39], x[56]);
    btf_16_ssse3!(*cospi.offset(59), *cospi.offset(5), x[40], x[40], x[55]);
    btf_16_ssse3!(-*cospi.offset(53), *cospi.offset(11), x[52], x[43], x[52]);
    btf_16_ssse3!(*cospi.offset(51), *cospi.offset(13), x[44], x[44], x[51]);
    btf_16_ssse3!(-*cospi.offset(61), *cospi.offset(3), x[48], x[47], x[48]);
    // stage 3
    btf_16_ssse3!(*cospi.offset(62), *cospi.offset(2), x[16], x[16], x[31]);
    btf_16_ssse3!(-*cospi.offset(50), *cospi.offset(14), x[28], x[19], x[28]);
    btf_16_ssse3!(*cospi.offset(54), *cospi.offset(10), x[20], x[20], x[27]);
    btf_16_ssse3!(-*cospi.offset(58), *cospi.offset(6), x[24], x[23], x[24]);
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
    btf_16_ssse3!(*cospi.offset(60), *cospi.offset(4), x[8], x[8], x[15]);
    btf_16_ssse3!(-*cospi.offset(52), *cospi.offset(12), x[12], x[11], x[12]);
    x[17] = x[16];
    x[18] = x[19];
    x[21] = x[20];
    x[22] = x[23];
    x[25] = x[24];
    x[26] = x[27];
    x[29] = x[28];
    x[30] = x[31];
    idct64_stage4_high32_sse2(x.as_mut_ptr(), cospi, __rounding, cos_bit);
    // stage 5
    btf_16_ssse3!(*cospi.offset(56), *cospi.offset(8), x[4], x[4], x[7]);
    x[9] = x[8];
    x[10] = x[11];
    x[13] = x[12];
    x[14] = x[15];
    idct64_stage5_high48_sse2(x.as_mut_ptr(), cospi, __rounding, cos_bit);
    // stage 6
    btf_16_ssse3!(*cospi.offset(32), *cospi.offset(32), x[0], x[0], x[1]);
    x[5] = x[4];
    x[6] = x[7];
    btf_16_sse2!(
        cospi_m16_p48,
        cospi_p48_p16,
        x[9],
        x[14],
        x[9],
        x[14],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m48_m16,
        cospi_m16_p48,
        x[10],
        x[13],
        x[10],
        x[13],
        __rounding,
        cos_bit
    );
    idct64_stage6_high48_sse2(x.as_mut_ptr(), cospi, __rounding, cos_bit);
    // stage 7
    x[3] = x[0];
    x[2] = x[1];
    btf_16_sse2!(
        cospi_m32_p32,
        cospi_p32_p32,
        x[5],
        x[6],
        x[5],
        x[6],
        __rounding,
        cos_bit
    );
    btf_16_adds_subs_sse2!(x[8], x[11]);
    btf_16_adds_subs_sse2!(x[9], x[10]);
    btf_16_subs_adds_sse2!(x[15], x[12]);
    btf_16_subs_adds_sse2!(x[14], x[13]);
    idct64_stage7_high48_sse2(x.as_mut_ptr(), cospi, __rounding, cos_bit);
    // stage 8
    btf_16_adds_subs_sse2!(x[0], x[7]);
    btf_16_adds_subs_sse2!(x[1], x[6]);
    btf_16_adds_subs_sse2!(x[2], x[5]);
    btf_16_adds_subs_sse2!(x[3], x[4]);
    btf_16_sse2!(
        cospi_m32_p32,
        cospi_p32_p32,
        x[10],
        x[13],
        x[10],
        x[13],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m32_p32,
        cospi_p32_p32,
        x[11],
        x[12],
        x[11],
        x[12],
        __rounding,
        cos_bit
    );
    idct64_stage8_high48_sse2(x.as_mut_ptr(), cospi, __rounding, cos_bit);
    idct64_stage9_sse2(x.as_mut_ptr(), cospi, __rounding, cos_bit);
    idct64_stage10_sse2(x.as_mut_ptr(), cospi, __rounding, cos_bit);
    idct64_stage11_sse2(output, x.as_mut_ptr());
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn idct64_low32_ssse3(input: *const __m128i, output: *mut __m128i) {
    let cos_bit: i8 = INV_COS_BIT as i8;
    let mut cospi: *const i32 = cospi_arr(INV_COS_BIT);
    let __rounding: __m128i = _mm_set1_epi32(1 << (INV_COS_BIT - 1));
    let cospi_p32_p32: __m128i = pair_set_epi16(*cospi.offset(32), *cospi.offset(32));
    let cospi_m16_p48: __m128i = pair_set_epi16(-*cospi.offset(16), *cospi.offset(48));
    let cospi_p48_p16: __m128i = pair_set_epi16(*cospi.offset(48), *cospi.offset(16));
    let cospi_m48_m16: __m128i = pair_set_epi16(-*cospi.offset(48), -*cospi.offset(16));
    let cospi_m32_p32: __m128i = pair_set_epi16(-*cospi.offset(32), *cospi.offset(32));
    // stage 1
    let mut x: [__m128i; 64] = core::mem::zeroed();
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
    btf_16_ssse3!(*cospi.offset(63), *cospi.offset(1), x[32], x[32], x[63]);
    btf_16_ssse3!(-*cospi.offset(33), *cospi.offset(31), x[62], x[33], x[62]);
    btf_16_ssse3!(*cospi.offset(47), *cospi.offset(17), x[34], x[34], x[61]);
    btf_16_ssse3!(-*cospi.offset(49), *cospi.offset(15), x[60], x[35], x[60]);
    btf_16_ssse3!(*cospi.offset(55), *cospi.offset(9), x[36], x[36], x[59]);
    btf_16_ssse3!(-*cospi.offset(41), *cospi.offset(23), x[58], x[37], x[58]);
    btf_16_ssse3!(*cospi.offset(39), *cospi.offset(25), x[38], x[38], x[57]);
    btf_16_ssse3!(-*cospi.offset(57), *cospi.offset(7), x[56], x[39], x[56]);
    btf_16_ssse3!(*cospi.offset(59), *cospi.offset(5), x[40], x[40], x[55]);
    btf_16_ssse3!(-*cospi.offset(37), *cospi.offset(27), x[54], x[41], x[54]);
    btf_16_ssse3!(*cospi.offset(43), *cospi.offset(21), x[42], x[42], x[53]);
    btf_16_ssse3!(-*cospi.offset(53), *cospi.offset(11), x[52], x[43], x[52]);
    btf_16_ssse3!(*cospi.offset(51), *cospi.offset(13), x[44], x[44], x[51]);
    btf_16_ssse3!(-*cospi.offset(45), *cospi.offset(19), x[50], x[45], x[50]);
    btf_16_ssse3!(*cospi.offset(35), *cospi.offset(29), x[46], x[46], x[49]);
    btf_16_ssse3!(-*cospi.offset(61), *cospi.offset(3), x[48], x[47], x[48]);
    // stage 3
    btf_16_ssse3!(*cospi.offset(62), *cospi.offset(2), x[16], x[16], x[31]);
    btf_16_ssse3!(-*cospi.offset(34), *cospi.offset(30), x[30], x[17], x[30]);
    btf_16_ssse3!(*cospi.offset(46), *cospi.offset(18), x[18], x[18], x[29]);
    btf_16_ssse3!(-*cospi.offset(50), *cospi.offset(14), x[28], x[19], x[28]);
    btf_16_ssse3!(*cospi.offset(54), *cospi.offset(10), x[20], x[20], x[27]);
    btf_16_ssse3!(-*cospi.offset(42), *cospi.offset(22), x[26], x[21], x[26]);
    btf_16_ssse3!(*cospi.offset(38), *cospi.offset(26), x[22], x[22], x[25]);
    btf_16_ssse3!(-*cospi.offset(58), *cospi.offset(6), x[24], x[23], x[24]);
    btf_16_adds_subs_sse2!(x[32], x[33]);
    btf_16_subs_adds_sse2!(x[35], x[34]);
    btf_16_adds_subs_sse2!(x[36], x[37]);
    btf_16_subs_adds_sse2!(x[39], x[38]);
    btf_16_adds_subs_sse2!(x[40], x[41]);
    btf_16_subs_adds_sse2!(x[43], x[42]);
    btf_16_adds_subs_sse2!(x[44], x[45]);
    btf_16_subs_adds_sse2!(x[47], x[46]);
    btf_16_adds_subs_sse2!(x[48], x[49]);
    btf_16_subs_adds_sse2!(x[51], x[50]);
    btf_16_adds_subs_sse2!(x[52], x[53]);
    btf_16_subs_adds_sse2!(x[55], x[54]);
    btf_16_adds_subs_sse2!(x[56], x[57]);
    btf_16_subs_adds_sse2!(x[59], x[58]);
    btf_16_adds_subs_sse2!(x[60], x[61]);
    btf_16_subs_adds_sse2!(x[63], x[62]);
    // stage 4
    btf_16_ssse3!(*cospi.offset(60), *cospi.offset(4), x[8], x[8], x[15]);
    btf_16_ssse3!(-*cospi.offset(36), *cospi.offset(28), x[14], x[9], x[14]);
    btf_16_ssse3!(*cospi.offset(44), *cospi.offset(20), x[10], x[10], x[13]);
    btf_16_ssse3!(-*cospi.offset(52), *cospi.offset(12), x[12], x[11], x[12]);
    btf_16_adds_subs_sse2!(x[16], x[17]);
    btf_16_subs_adds_sse2!(x[19], x[18]);
    btf_16_adds_subs_sse2!(x[20], x[21]);
    btf_16_subs_adds_sse2!(x[23], x[22]);
    btf_16_adds_subs_sse2!(x[24], x[25]);
    btf_16_subs_adds_sse2!(x[27], x[26]);
    btf_16_adds_subs_sse2!(x[28], x[29]);
    btf_16_subs_adds_sse2!(x[31], x[30]);
    idct64_stage4_high32_sse2(x.as_mut_ptr(), cospi, __rounding, cos_bit);
    // stage 5
    btf_16_ssse3!(*cospi.offset(56), *cospi.offset(8), x[4], x[4], x[7]);
    btf_16_ssse3!(-*cospi.offset(40), *cospi.offset(24), x[6], x[5], x[6]);
    btf_16_adds_subs_sse2!(x[8], x[9]);
    btf_16_subs_adds_sse2!(x[11], x[10]);
    btf_16_adds_subs_sse2!(x[12], x[13]);
    btf_16_subs_adds_sse2!(x[15], x[14]);
    idct64_stage5_high48_sse2(x.as_mut_ptr(), cospi, __rounding, cos_bit);
    // stage 6
    btf_16_ssse3!(*cospi.offset(32), *cospi.offset(32), x[0], x[0], x[1]);
    btf_16_ssse3!(*cospi.offset(48), *cospi.offset(16), x[2], x[2], x[3]);
    btf_16_adds_subs_sse2!(x[4], x[5]);
    btf_16_subs_adds_sse2!(x[7], x[6]);
    btf_16_sse2!(
        cospi_m16_p48,
        cospi_p48_p16,
        x[9],
        x[14],
        x[9],
        x[14],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m48_m16,
        cospi_m16_p48,
        x[10],
        x[13],
        x[10],
        x[13],
        __rounding,
        cos_bit
    );
    idct64_stage6_high48_sse2(x.as_mut_ptr(), cospi, __rounding, cos_bit);
    // stage 7
    btf_16_adds_subs_sse2!(x[0], x[3]);
    btf_16_adds_subs_sse2!(x[1], x[2]);
    btf_16_sse2!(
        cospi_m32_p32,
        cospi_p32_p32,
        x[5],
        x[6],
        x[5],
        x[6],
        __rounding,
        cos_bit
    );
    btf_16_adds_subs_sse2!(x[8], x[11]);
    btf_16_adds_subs_sse2!(x[9], x[10]);
    btf_16_subs_adds_sse2!(x[15], x[12]);
    btf_16_subs_adds_sse2!(x[14], x[13]);
    idct64_stage7_high48_sse2(x.as_mut_ptr(), cospi, __rounding, cos_bit);
    // stage 8
    btf_16_adds_subs_sse2!(x[0], x[7]);
    btf_16_adds_subs_sse2!(x[1], x[6]);
    btf_16_adds_subs_sse2!(x[2], x[5]);
    btf_16_adds_subs_sse2!(x[3], x[4]);
    btf_16_sse2!(
        cospi_m32_p32,
        cospi_p32_p32,
        x[10],
        x[13],
        x[10],
        x[13],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m32_p32,
        cospi_p32_p32,
        x[11],
        x[12],
        x[11],
        x[12],
        __rounding,
        cos_bit
    );
    idct64_stage8_high48_sse2(x.as_mut_ptr(), cospi, __rounding, cos_bit);
    // stage 9~11
    idct64_stage9_sse2(x.as_mut_ptr(), cospi, __rounding, cos_bit);
    idct64_stage10_sse2(x.as_mut_ptr(), cospi, __rounding, cos_bit);
    idct64_stage11_sse2(output, x.as_mut_ptr());
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn iadst4_sse2(input: *const __m128i, output: *mut __m128i) {
    let mut sinpi: *const i32 = sinpi_arr(INV_COS_BIT);
    let sinpi_p01_p04: __m128i = pair_set_epi16(*sinpi.offset(1), *sinpi.offset(4));
    let sinpi_p02_m01: __m128i = pair_set_epi16(*sinpi.offset(2), -*sinpi.offset(1));
    let sinpi_p03_p02: __m128i = pair_set_epi16(*sinpi.offset(3), *sinpi.offset(2));
    let sinpi_p03_m04: __m128i = pair_set_epi16(*sinpi.offset(3), -*sinpi.offset(4));
    let sinpi_p03_m03: __m128i = pair_set_epi16(*sinpi.offset(3), -*sinpi.offset(3));
    let sinpi_0_p03: __m128i = pair_set_epi16(0, *sinpi.offset(3));
    let sinpi_p04_p02: __m128i = pair_set_epi16(*sinpi.offset(4), *sinpi.offset(2));
    let sinpi_m03_m01: __m128i = pair_set_epi16(-*sinpi.offset(3), -*sinpi.offset(1));
    let mut x0: [__m128i; 4] = core::mem::zeroed();
    x0[0] = *input;
    x0[1] = *input.offset(1);
    x0[2] = *input.offset(2);
    x0[3] = *input.offset(3);
    let mut u: [__m128i; 4] = core::mem::zeroed();
    u[0] = _mm_unpacklo_epi16(x0[0], x0[2]);
    u[1] = _mm_unpackhi_epi16(x0[0], x0[2]);
    u[2] = _mm_unpacklo_epi16(x0[1], x0[3]);
    u[3] = _mm_unpackhi_epi16(x0[1], x0[3]);
    let mut x1: [__m128i; 16] = core::mem::zeroed();
    x1[0] = _mm_madd_epi16(u[0], sinpi_p01_p04); // x0*sin1 + x2*sin4
    x1[1] = _mm_madd_epi16(u[1], sinpi_p01_p04);
    x1[2] = _mm_madd_epi16(u[0], sinpi_p02_m01); // x0*sin2 - x2*sin1
    x1[3] = _mm_madd_epi16(u[1], sinpi_p02_m01);
    x1[4] = _mm_madd_epi16(u[2], sinpi_p03_p02); // x1*sin3 + x3*sin2
    x1[5] = _mm_madd_epi16(u[3], sinpi_p03_p02);
    x1[6] = _mm_madd_epi16(u[2], sinpi_p03_m04); // x1*sin3 - x3*sin4
    x1[7] = _mm_madd_epi16(u[3], sinpi_p03_m04);
    x1[8] = _mm_madd_epi16(u[0], sinpi_p03_m03); // x0*sin3 - x2*sin3
    x1[9] = _mm_madd_epi16(u[1], sinpi_p03_m03);
    x1[10] = _mm_madd_epi16(u[2], sinpi_0_p03); // x2*sin3
    x1[11] = _mm_madd_epi16(u[3], sinpi_0_p03);
    x1[12] = _mm_madd_epi16(u[0], sinpi_p04_p02); // x0*sin4 + x2*sin2
    x1[13] = _mm_madd_epi16(u[1], sinpi_p04_p02);
    x1[14] = _mm_madd_epi16(u[2], sinpi_m03_m01); // -x1*sin3 - x3*sin1
    x1[15] = _mm_madd_epi16(u[3], sinpi_m03_m01);
    let mut x2: [__m128i; 8] = core::mem::zeroed();
    x2[0] = _mm_add_epi32(x1[0], x1[4]); // x0*sin1 +x2*sin4 +x1*sin3 +x3*sin2
    x2[1] = _mm_add_epi32(x1[1], x1[5]);
    x2[2] = _mm_add_epi32(x1[2], x1[6]); // x0*sin2 -x2*sin1 +x1*sin3 -x3*sin4
    x2[3] = _mm_add_epi32(x1[3], x1[7]);
    x2[4] = _mm_add_epi32(x1[8], x1[10]); // x0*sin3 -x2*sin3 +x3*sin3
    x2[5] = _mm_add_epi32(x1[9], x1[11]);
    x2[6] = _mm_add_epi32(x1[12], x1[14]); // x0*sin1 +x2*sin4 +x0*sin2 -x2*sin1
    x2[7] = _mm_add_epi32(x1[13], x1[15]);
    let rounding: __m128i = _mm_set1_epi32(1 << (INV_COS_BIT - 1));
    for i in 0..4 {
        let mut out0: __m128i = _mm_add_epi32(x2[(2 * i) as usize], rounding);
        let mut out1: __m128i = _mm_add_epi32(x2[(2 * i + 1) as usize], rounding);
        out0 = _mm_srai_epi32::<{ INV_COS_BIT }>(out0);
        out1 = _mm_srai_epi32::<{ INV_COS_BIT }>(out1);
        *output.offset(i as isize) = _mm_packs_epi32(out0, out1);
    }
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn iadst4_w4_sse2(input: *const __m128i, output: *mut __m128i) {
    let mut sinpi: *const i32 = sinpi_arr(INV_COS_BIT);
    let sinpi_p01_p04: __m128i = pair_set_epi16(*sinpi.offset(1), *sinpi.offset(4));
    let sinpi_p02_m01: __m128i = pair_set_epi16(*sinpi.offset(2), -*sinpi.offset(1));
    let sinpi_p03_p02: __m128i = pair_set_epi16(*sinpi.offset(3), *sinpi.offset(2));
    let sinpi_p03_m04: __m128i = pair_set_epi16(*sinpi.offset(3), -*sinpi.offset(4));
    let sinpi_p03_m03: __m128i = pair_set_epi16(*sinpi.offset(3), -*sinpi.offset(3));
    let sinpi_0_p03: __m128i = pair_set_epi16(0, *sinpi.offset(3));
    let sinpi_p04_p02: __m128i = pair_set_epi16(*sinpi.offset(4), *sinpi.offset(2));
    let sinpi_m03_m01: __m128i = pair_set_epi16(-*sinpi.offset(3), -*sinpi.offset(1));
    let mut x0: [__m128i; 4] = core::mem::zeroed();
    x0[0] = *input;
    x0[1] = *input.offset(1);
    x0[2] = *input.offset(2);
    x0[3] = *input.offset(3);
    let mut u: [__m128i; 2] = core::mem::zeroed();
    u[0] = _mm_unpacklo_epi16(x0[0], x0[2]);
    u[1] = _mm_unpacklo_epi16(x0[1], x0[3]);
    let mut x1: [__m128i; 8] = core::mem::zeroed();
    x1[0] = _mm_madd_epi16(u[0], sinpi_p01_p04); // x0*sin1 + x2*sin4
    x1[1] = _mm_madd_epi16(u[0], sinpi_p02_m01); // x0*sin2 - x2*sin1
    x1[2] = _mm_madd_epi16(u[1], sinpi_p03_p02); // x1*sin3 + x3*sin2
    x1[3] = _mm_madd_epi16(u[1], sinpi_p03_m04); // x1*sin3 - x3*sin4
    x1[4] = _mm_madd_epi16(u[0], sinpi_p03_m03); // x0*sin3 - x2*sin3
    x1[5] = _mm_madd_epi16(u[1], sinpi_0_p03); // x2*sin3
    x1[6] = _mm_madd_epi16(u[0], sinpi_p04_p02); // x0*sin4 + x2*sin2
    x1[7] = _mm_madd_epi16(u[1], sinpi_m03_m01); // -x1*sin3 - x3*sin1
    let mut x2: [__m128i; 4] = core::mem::zeroed();
    x2[0] = _mm_add_epi32(x1[0], x1[2]); // x0*sin1 + x2*sin4 + x1*sin3 + x3*sin2
    x2[1] = _mm_add_epi32(x1[1], x1[3]); // x0*sin2 - x2*sin1 + x1*sin3 - x3*sin4
    x2[2] = _mm_add_epi32(x1[4], x1[5]); // x0*sin3 - x2*sin3 + x3*sin3
    x2[3] = _mm_add_epi32(x1[6], x1[7]); // x0*sin4 + x2*sin2 - x1*sin3 - x3*sin1
    let rounding: __m128i = _mm_set1_epi32(1 << (INV_COS_BIT - 1));
    for i in 0..4 {
        let mut out0: __m128i = _mm_add_epi32(x2[i as usize], rounding);
        out0 = _mm_srai_epi32::<{ INV_COS_BIT }>(out0);
        *output.offset(i as isize) = _mm_packs_epi32(out0, out0);
    }
}

#[target_feature(enable = "ssse3")]
pub unsafe fn av1_iadst8_low1_ssse3(input: *const __m128i, output: *mut __m128i) {
    let cos_bit: i8 = INV_COS_BIT as i8;
    let mut cospi: *const i32 = cospi_arr(INV_COS_BIT);
    let __zero: __m128i = _mm_setzero_si128();
    let __rounding: __m128i = _mm_set1_epi32(1 << (INV_COS_BIT - 1));
    let cospi_p16_p48: __m128i = pair_set_epi16(*cospi.offset(16), *cospi.offset(48));
    let cospi_p48_m16: __m128i = pair_set_epi16(*cospi.offset(48), -*cospi.offset(16));
    let cospi_p32_p32: __m128i = pair_set_epi16(*cospi.offset(32), *cospi.offset(32));
    let cospi_p32_m32: __m128i = pair_set_epi16(*cospi.offset(32), -*cospi.offset(32));
    // stage 1
    let mut x: [__m128i; 8] = core::mem::zeroed();
    x[1] = *input;
    // stage 2
    btf_16_ssse3!(*cospi.offset(60), -*cospi.offset(4), x[1], x[0], x[1]);
    // stage 3
    x[4] = x[0];
    x[5] = x[1];
    // stage 4
    btf_16_sse2!(
        cospi_p16_p48,
        cospi_p48_m16,
        x[4],
        x[5],
        x[4],
        x[5],
        __rounding,
        cos_bit
    );
    // stage 5
    x[2] = x[0];
    x[3] = x[1];
    x[6] = x[4];
    x[7] = x[5];
    // stage 6
    btf_16_sse2!(
        cospi_p32_p32,
        cospi_p32_m32,
        x[2],
        x[3],
        x[2],
        x[3],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p32_p32,
        cospi_p32_m32,
        x[6],
        x[7],
        x[6],
        x[7],
        __rounding,
        cos_bit
    );
    // stage 7
    *output = x[0];
    *output.offset(1) = _mm_subs_epi16(__zero, x[4]);
    *output.offset(2) = x[6];
    *output.offset(3) = _mm_subs_epi16(__zero, x[2]);
    *output.offset(4) = x[3];
    *output.offset(5) = _mm_subs_epi16(__zero, x[7]);
    *output.offset(6) = x[5];
    *output.offset(7) = _mm_subs_epi16(__zero, x[1]);
}

#[target_feature(enable = "ssse3")]
pub unsafe fn av1_iadst8_sse2(input: *const __m128i, output: *mut __m128i) {
    let cos_bit: i8 = INV_COS_BIT as i8;
    let mut cospi: *const i32 = cospi_arr(INV_COS_BIT);
    let __zero: __m128i = _mm_setzero_si128();
    let __rounding: __m128i = _mm_set1_epi32(1 << (INV_COS_BIT - 1));
    let cospi_p04_p60: __m128i = pair_set_epi16(*cospi.offset(4), *cospi.offset(60));
    let cospi_p60_m04: __m128i = pair_set_epi16(*cospi.offset(60), -*cospi.offset(4));
    let cospi_p20_p44: __m128i = pair_set_epi16(*cospi.offset(20), *cospi.offset(44));
    let cospi_p44_m20: __m128i = pair_set_epi16(*cospi.offset(44), -*cospi.offset(20));
    let cospi_p36_p28: __m128i = pair_set_epi16(*cospi.offset(36), *cospi.offset(28));
    let cospi_p28_m36: __m128i = pair_set_epi16(*cospi.offset(28), -*cospi.offset(36));
    let cospi_p52_p12: __m128i = pair_set_epi16(*cospi.offset(52), *cospi.offset(12));
    let cospi_p12_m52: __m128i = pair_set_epi16(*cospi.offset(12), -*cospi.offset(52));
    let cospi_p16_p48: __m128i = pair_set_epi16(*cospi.offset(16), *cospi.offset(48));
    let cospi_p48_m16: __m128i = pair_set_epi16(*cospi.offset(48), -*cospi.offset(16));
    let cospi_m48_p16: __m128i = pair_set_epi16(-*cospi.offset(48), *cospi.offset(16));
    let cospi_p32_p32: __m128i = pair_set_epi16(*cospi.offset(32), *cospi.offset(32));
    let cospi_p32_m32: __m128i = pair_set_epi16(*cospi.offset(32), -*cospi.offset(32));
    // stage 1
    let mut x: [__m128i; 8] = core::mem::zeroed();
    x[0] = *input.offset(7);
    x[1] = *input;
    x[2] = *input.offset(5);
    x[3] = *input.offset(2);
    x[4] = *input.offset(3);
    x[5] = *input.offset(4);
    x[6] = *input.offset(1);
    x[7] = *input.offset(6);
    // stage 2
    btf_16_sse2!(
        cospi_p04_p60,
        cospi_p60_m04,
        x[0],
        x[1],
        x[0],
        x[1],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p20_p44,
        cospi_p44_m20,
        x[2],
        x[3],
        x[2],
        x[3],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p36_p28,
        cospi_p28_m36,
        x[4],
        x[5],
        x[4],
        x[5],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p52_p12,
        cospi_p12_m52,
        x[6],
        x[7],
        x[6],
        x[7],
        __rounding,
        cos_bit
    );
    // stage 3
    btf_16_adds_subs_sse2!(x[0], x[4]);
    btf_16_adds_subs_sse2!(x[1], x[5]);
    btf_16_adds_subs_sse2!(x[2], x[6]);
    btf_16_adds_subs_sse2!(x[3], x[7]);
    // stage 4
    btf_16_sse2!(
        cospi_p16_p48,
        cospi_p48_m16,
        x[4],
        x[5],
        x[4],
        x[5],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m48_p16,
        cospi_p16_p48,
        x[6],
        x[7],
        x[6],
        x[7],
        __rounding,
        cos_bit
    );
    // stage 5
    btf_16_adds_subs_sse2!(x[0], x[2]);
    btf_16_adds_subs_sse2!(x[1], x[3]);
    btf_16_adds_subs_sse2!(x[4], x[6]);
    btf_16_adds_subs_sse2!(x[5], x[7]);
    // stage 6
    btf_16_sse2!(
        cospi_p32_p32,
        cospi_p32_m32,
        x[2],
        x[3],
        x[2],
        x[3],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p32_p32,
        cospi_p32_m32,
        x[6],
        x[7],
        x[6],
        x[7],
        __rounding,
        cos_bit
    );
    // stage 7
    *output = x[0];
    *output.offset(1) = _mm_subs_epi16(__zero, x[4]);
    *output.offset(2) = x[6];
    *output.offset(3) = _mm_subs_epi16(__zero, x[2]);
    *output.offset(4) = x[3];
    *output.offset(5) = _mm_subs_epi16(__zero, x[7]);
    *output.offset(6) = x[5];
    *output.offset(7) = _mm_subs_epi16(__zero, x[1]);
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn iadst8_w4_sse2(input: *const __m128i, output: *mut __m128i) {
    let cos_bit: i8 = INV_COS_BIT as i8;
    let mut cospi: *const i32 = cospi_arr(INV_COS_BIT);
    let __zero: __m128i = _mm_setzero_si128();
    let __rounding: __m128i = _mm_set1_epi32(1 << (INV_COS_BIT - 1));
    let cospi_p04_p60: __m128i = pair_set_epi16(*cospi.offset(4), *cospi.offset(60));
    let cospi_p60_m04: __m128i = pair_set_epi16(*cospi.offset(60), -*cospi.offset(4));
    let cospi_p20_p44: __m128i = pair_set_epi16(*cospi.offset(20), *cospi.offset(44));
    let cospi_p44_m20: __m128i = pair_set_epi16(*cospi.offset(44), -*cospi.offset(20));
    let cospi_p36_p28: __m128i = pair_set_epi16(*cospi.offset(36), *cospi.offset(28));
    let cospi_p28_m36: __m128i = pair_set_epi16(*cospi.offset(28), -*cospi.offset(36));
    let cospi_p52_p12: __m128i = pair_set_epi16(*cospi.offset(52), *cospi.offset(12));
    let cospi_p12_m52: __m128i = pair_set_epi16(*cospi.offset(12), -*cospi.offset(52));
    let cospi_p16_p48: __m128i = pair_set_epi16(*cospi.offset(16), *cospi.offset(48));
    let cospi_p48_m16: __m128i = pair_set_epi16(*cospi.offset(48), -*cospi.offset(16));
    let cospi_m48_p16: __m128i = pair_set_epi16(-*cospi.offset(48), *cospi.offset(16));
    let cospi_p32_p32: __m128i = pair_set_epi16(*cospi.offset(32), *cospi.offset(32));
    let cospi_p32_m32: __m128i = pair_set_epi16(*cospi.offset(32), -*cospi.offset(32));
    // stage 1
    let mut x: [__m128i; 8] = core::mem::zeroed();
    x[0] = *input.offset(7);
    x[1] = *input;
    x[2] = *input.offset(5);
    x[3] = *input.offset(2);
    x[4] = *input.offset(3);
    x[5] = *input.offset(4);
    x[6] = *input.offset(1);
    x[7] = *input.offset(6);
    // stage 2
    btf_16_4p_sse2!(
        cospi_p04_p60,
        cospi_p60_m04,
        x[0],
        x[1],
        x[0],
        x[1],
        __rounding,
        cos_bit
    );
    btf_16_4p_sse2!(
        cospi_p20_p44,
        cospi_p44_m20,
        x[2],
        x[3],
        x[2],
        x[3],
        __rounding,
        cos_bit
    );
    btf_16_4p_sse2!(
        cospi_p36_p28,
        cospi_p28_m36,
        x[4],
        x[5],
        x[4],
        x[5],
        __rounding,
        cos_bit
    );
    btf_16_4p_sse2!(
        cospi_p52_p12,
        cospi_p12_m52,
        x[6],
        x[7],
        x[6],
        x[7],
        __rounding,
        cos_bit
    );
    // stage 3
    btf_16_adds_subs_sse2!(x[0], x[4]);
    btf_16_adds_subs_sse2!(x[1], x[5]);
    btf_16_adds_subs_sse2!(x[2], x[6]);
    btf_16_adds_subs_sse2!(x[3], x[7]);
    // stage 4
    btf_16_4p_sse2!(
        cospi_p16_p48,
        cospi_p48_m16,
        x[4],
        x[5],
        x[4],
        x[5],
        __rounding,
        cos_bit
    );
    btf_16_4p_sse2!(
        cospi_m48_p16,
        cospi_p16_p48,
        x[6],
        x[7],
        x[6],
        x[7],
        __rounding,
        cos_bit
    );
    // stage 5
    btf_16_adds_subs_sse2!(x[0], x[2]);
    btf_16_adds_subs_sse2!(x[1], x[3]);
    btf_16_adds_subs_sse2!(x[4], x[6]);
    btf_16_adds_subs_sse2!(x[5], x[7]);
    // stage 6
    btf_16_4p_sse2!(
        cospi_p32_p32,
        cospi_p32_m32,
        x[2],
        x[3],
        x[2],
        x[3],
        __rounding,
        cos_bit
    );
    btf_16_4p_sse2!(
        cospi_p32_p32,
        cospi_p32_m32,
        x[6],
        x[7],
        x[6],
        x[7],
        __rounding,
        cos_bit
    );
    // stage 7
    *output = x[0];
    *output.offset(1) = _mm_subs_epi16(__zero, x[4]);
    *output.offset(2) = x[6];
    *output.offset(3) = _mm_subs_epi16(__zero, x[2]);
    *output.offset(4) = x[3];
    *output.offset(5) = _mm_subs_epi16(__zero, x[7]);
    *output.offset(6) = x[5];
    *output.offset(7) = _mm_subs_epi16(__zero, x[1]);
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn iadst16_stage3_ssse3(x: *mut __m128i) {
    btf_16_adds_subs_sse2!(*x, *x.offset(8));
    btf_16_adds_subs_sse2!(*x.offset(1), *x.offset(9));
    btf_16_adds_subs_sse2!(*x.offset(2), *x.offset(10));
    btf_16_adds_subs_sse2!(*x.offset(3), *x.offset(11));
    btf_16_adds_subs_sse2!(*x.offset(4), *x.offset(12));
    btf_16_adds_subs_sse2!(*x.offset(5), *x.offset(13));
    btf_16_adds_subs_sse2!(*x.offset(6), *x.offset(14));
    btf_16_adds_subs_sse2!(*x.offset(7), *x.offset(15));
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn iadst16_stage4_ssse3(
    x: *mut __m128i,
    cospi: *const i32,
    __rounding: __m128i,
    cos_bit: i8,
) {
    let cospi_p08_p56: __m128i = pair_set_epi16(*cospi.offset(8), *cospi.offset(56));
    let cospi_p56_m08: __m128i = pair_set_epi16(*cospi.offset(56), -*cospi.offset(8));
    let cospi_p40_p24: __m128i = pair_set_epi16(*cospi.offset(40), *cospi.offset(24));
    let cospi_p24_m40: __m128i = pair_set_epi16(*cospi.offset(24), -*cospi.offset(40));
    let cospi_m56_p08: __m128i = pair_set_epi16(-*cospi.offset(56), *cospi.offset(8));
    let cospi_m24_p40: __m128i = pair_set_epi16(-*cospi.offset(24), *cospi.offset(40));
    btf_16_sse2!(
        cospi_p08_p56,
        cospi_p56_m08,
        *x.offset(8),
        *x.offset(9),
        *x.offset(8),
        *x.offset(9),
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p40_p24,
        cospi_p24_m40,
        *x.offset(10),
        *x.offset(11),
        *x.offset(10),
        *x.offset(11),
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m56_p08,
        cospi_p08_p56,
        *x.offset(12),
        *x.offset(13),
        *x.offset(12),
        *x.offset(13),
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m24_p40,
        cospi_p40_p24,
        *x.offset(14),
        *x.offset(15),
        *x.offset(14),
        *x.offset(15),
        __rounding,
        cos_bit
    );
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn iadst16_stage5_ssse3(x: *mut __m128i) {
    btf_16_adds_subs_sse2!(*x, *x.offset(4));
    btf_16_adds_subs_sse2!(*x.offset(1), *x.offset(5));
    btf_16_adds_subs_sse2!(*x.offset(2), *x.offset(6));
    btf_16_adds_subs_sse2!(*x.offset(3), *x.offset(7));
    btf_16_adds_subs_sse2!(*x.offset(8), *x.offset(12));
    btf_16_adds_subs_sse2!(*x.offset(9), *x.offset(13));
    btf_16_adds_subs_sse2!(*x.offset(10), *x.offset(14));
    btf_16_adds_subs_sse2!(*x.offset(11), *x.offset(15));
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn iadst16_stage6_ssse3(
    x: *mut __m128i,
    cospi: *const i32,
    __rounding: __m128i,
    cos_bit: i8,
) {
    let cospi_p16_p48: __m128i = pair_set_epi16(*cospi.offset(16), *cospi.offset(48));
    let cospi_p48_m16: __m128i = pair_set_epi16(*cospi.offset(48), -*cospi.offset(16));
    let cospi_m48_p16: __m128i = pair_set_epi16(-*cospi.offset(48), *cospi.offset(16));
    btf_16_sse2!(
        cospi_p16_p48,
        cospi_p48_m16,
        *x.offset(4),
        *x.offset(5),
        *x.offset(4),
        *x.offset(5),
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m48_p16,
        cospi_p16_p48,
        *x.offset(6),
        *x.offset(7),
        *x.offset(6),
        *x.offset(7),
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p16_p48,
        cospi_p48_m16,
        *x.offset(12),
        *x.offset(13),
        *x.offset(12),
        *x.offset(13),
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_m48_p16,
        cospi_p16_p48,
        *x.offset(14),
        *x.offset(15),
        *x.offset(14),
        *x.offset(15),
        __rounding,
        cos_bit
    );
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn iadst16_stage7_ssse3(x: *mut __m128i) {
    btf_16_adds_subs_sse2!(*x, *x.offset(2));
    btf_16_adds_subs_sse2!(*x.offset(1), *x.offset(3));
    btf_16_adds_subs_sse2!(*x.offset(4), *x.offset(6));
    btf_16_adds_subs_sse2!(*x.offset(5), *x.offset(7));
    btf_16_adds_subs_sse2!(*x.offset(8), *x.offset(10));
    btf_16_adds_subs_sse2!(*x.offset(9), *x.offset(11));
    btf_16_adds_subs_sse2!(*x.offset(12), *x.offset(14));
    btf_16_adds_subs_sse2!(*x.offset(13), *x.offset(15));
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn iadst16_stage8_ssse3(
    x: *mut __m128i,
    cospi: *const i32,
    __rounding: __m128i,
    cos_bit: i8,
) {
    let cospi_p32_p32: __m128i = pair_set_epi16(*cospi.offset(32), *cospi.offset(32));
    let cospi_p32_m32: __m128i = pair_set_epi16(*cospi.offset(32), -*cospi.offset(32));
    btf_16_sse2!(
        cospi_p32_p32,
        cospi_p32_m32,
        *x.offset(2),
        *x.offset(3),
        *x.offset(2),
        *x.offset(3),
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p32_p32,
        cospi_p32_m32,
        *x.offset(6),
        *x.offset(7),
        *x.offset(6),
        *x.offset(7),
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p32_p32,
        cospi_p32_m32,
        *x.offset(10),
        *x.offset(11),
        *x.offset(10),
        *x.offset(11),
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p32_p32,
        cospi_p32_m32,
        *x.offset(14),
        *x.offset(15),
        *x.offset(14),
        *x.offset(15),
        __rounding,
        cos_bit
    );
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn iadst16_stage9_ssse3(output: *mut __m128i, x: *mut __m128i) {
    let __zero: __m128i = _mm_setzero_si128();
    *output = *x;
    *output.offset(1) = _mm_subs_epi16(__zero, *x.offset(8));
    *output.offset(2) = *x.offset(12);
    *output.offset(3) = _mm_subs_epi16(__zero, *x.offset(4));
    *output.offset(4) = *x.offset(6);
    *output.offset(5) = _mm_subs_epi16(__zero, *x.offset(14));
    *output.offset(6) = *x.offset(10);
    *output.offset(7) = _mm_subs_epi16(__zero, *x.offset(2));
    *output.offset(8) = *x.offset(3);
    *output.offset(9) = _mm_subs_epi16(__zero, *x.offset(11));
    *output.offset(10) = *x.offset(15);
    *output.offset(11) = _mm_subs_epi16(__zero, *x.offset(7));
    *output.offset(12) = *x.offset(5);
    *output.offset(13) = _mm_subs_epi16(__zero, *x.offset(13));
    *output.offset(14) = *x.offset(9);
    *output.offset(15) = _mm_subs_epi16(__zero, *x.offset(1));
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn iadst16_low1_ssse3(input: *const __m128i, output: *mut __m128i) {
    let cos_bit: i8 = INV_COS_BIT as i8;
    let mut cospi: *const i32 = cospi_arr(INV_COS_BIT);
    let __rounding: __m128i = _mm_set1_epi32(1 << (INV_COS_BIT - 1));
    let cospi_p08_p56: __m128i = pair_set_epi16(*cospi.offset(8), *cospi.offset(56));
    let cospi_p56_m08: __m128i = pair_set_epi16(*cospi.offset(56), -*cospi.offset(8));
    let cospi_p16_p48: __m128i = pair_set_epi16(*cospi.offset(16), *cospi.offset(48));
    let cospi_p48_m16: __m128i = pair_set_epi16(*cospi.offset(48), -*cospi.offset(16));
    // stage 1
    let mut x: [__m128i; 16] = core::mem::zeroed();
    x[1] = *input;
    // stage 2
    btf_16_ssse3!(*cospi.offset(62), -*cospi.offset(2), x[1], x[0], x[1]);
    // stage 3
    x[8] = x[0];
    x[9] = x[1];
    // stage 4
    btf_16_sse2!(
        cospi_p08_p56,
        cospi_p56_m08,
        x[8],
        x[9],
        x[8],
        x[9],
        __rounding,
        cos_bit
    );
    // stage 5
    x[4] = x[0];
    x[5] = x[1];
    x[12] = x[8];
    x[13] = x[9];
    // stage 6
    btf_16_sse2!(
        cospi_p16_p48,
        cospi_p48_m16,
        x[4],
        x[5],
        x[4],
        x[5],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p16_p48,
        cospi_p48_m16,
        x[12],
        x[13],
        x[12],
        x[13],
        __rounding,
        cos_bit
    );
    // stage 7
    x[2] = x[0];
    x[3] = x[1];
    x[6] = x[4];
    x[7] = x[5];
    x[10] = x[8];
    x[11] = x[9];
    x[14] = x[12];
    x[15] = x[13];
    iadst16_stage8_ssse3(x.as_mut_ptr(), cospi, __rounding, cos_bit);
    iadst16_stage9_ssse3(output, x.as_mut_ptr());
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn iadst16_low8_ssse3(input: *const __m128i, output: *mut __m128i) {
    let cos_bit: i8 = INV_COS_BIT as i8;
    let mut cospi: *const i32 = cospi_arr(INV_COS_BIT);
    let __rounding: __m128i = _mm_set1_epi32(1 << (INV_COS_BIT - 1));
    // stage 1
    let mut x: [__m128i; 16] = core::mem::zeroed();
    x[1] = *input;
    x[3] = *input.offset(2);
    x[5] = *input.offset(4);
    x[7] = *input.offset(6);
    x[8] = *input.offset(7);
    x[10] = *input.offset(5);
    x[12] = *input.offset(3);
    x[14] = *input.offset(1);
    // stage 2
    btf_16_ssse3!(*cospi.offset(62), -*cospi.offset(2), x[1], x[0], x[1]);
    btf_16_ssse3!(*cospi.offset(54), -*cospi.offset(10), x[3], x[2], x[3]);
    btf_16_ssse3!(*cospi.offset(46), -*cospi.offset(18), x[5], x[4], x[5]);
    btf_16_ssse3!(*cospi.offset(38), -*cospi.offset(26), x[7], x[6], x[7]);
    btf_16_ssse3!(*cospi.offset(34), *cospi.offset(30), x[8], x[8], x[9]);
    btf_16_ssse3!(*cospi.offset(42), *cospi.offset(22), x[10], x[10], x[11]);
    btf_16_ssse3!(*cospi.offset(50), *cospi.offset(14), x[12], x[12], x[13]);
    btf_16_ssse3!(*cospi.offset(58), *cospi.offset(6), x[14], x[14], x[15]);
    // stage 3
    iadst16_stage3_ssse3(x.as_mut_ptr());
    iadst16_stage4_ssse3(x.as_mut_ptr(), cospi, __rounding, cos_bit);
    iadst16_stage5_ssse3(x.as_mut_ptr());
    iadst16_stage6_ssse3(x.as_mut_ptr(), cospi, __rounding, cos_bit);
    iadst16_stage7_ssse3(x.as_mut_ptr());
    iadst16_stage8_ssse3(x.as_mut_ptr(), cospi, __rounding, cos_bit);
    iadst16_stage9_ssse3(output, x.as_mut_ptr());
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn iadst16_sse2(input: *const __m128i, output: *mut __m128i) {
    let cos_bit: i8 = INV_COS_BIT as i8;
    let mut cospi: *const i32 = cospi_arr(INV_COS_BIT);
    let __rounding: __m128i = _mm_set1_epi32(1 << (INV_COS_BIT - 1));
    let cospi_p02_p62: __m128i = pair_set_epi16(*cospi.offset(2), *cospi.offset(62));
    let cospi_p62_m02: __m128i = pair_set_epi16(*cospi.offset(62), -*cospi.offset(2));
    let cospi_p10_p54: __m128i = pair_set_epi16(*cospi.offset(10), *cospi.offset(54));
    let cospi_p54_m10: __m128i = pair_set_epi16(*cospi.offset(54), -*cospi.offset(10));
    let cospi_p18_p46: __m128i = pair_set_epi16(*cospi.offset(18), *cospi.offset(46));
    let cospi_p46_m18: __m128i = pair_set_epi16(*cospi.offset(46), -*cospi.offset(18));
    let cospi_p26_p38: __m128i = pair_set_epi16(*cospi.offset(26), *cospi.offset(38));
    let cospi_p38_m26: __m128i = pair_set_epi16(*cospi.offset(38), -*cospi.offset(26));
    let cospi_p34_p30: __m128i = pair_set_epi16(*cospi.offset(34), *cospi.offset(30));
    let cospi_p30_m34: __m128i = pair_set_epi16(*cospi.offset(30), -*cospi.offset(34));
    let cospi_p42_p22: __m128i = pair_set_epi16(*cospi.offset(42), *cospi.offset(22));
    let cospi_p22_m42: __m128i = pair_set_epi16(*cospi.offset(22), -*cospi.offset(42));
    let cospi_p50_p14: __m128i = pair_set_epi16(*cospi.offset(50), *cospi.offset(14));
    let cospi_p14_m50: __m128i = pair_set_epi16(*cospi.offset(14), -*cospi.offset(50));
    let cospi_p58_p06: __m128i = pair_set_epi16(*cospi.offset(58), *cospi.offset(6));
    let cospi_p06_m58: __m128i = pair_set_epi16(*cospi.offset(6), -*cospi.offset(58));
    // stage 1
    let mut x: [__m128i; 16] = core::mem::zeroed();
    x[0] = *input.offset(15);
    x[1] = *input;
    x[2] = *input.offset(13);
    x[3] = *input.offset(2);
    x[4] = *input.offset(11);
    x[5] = *input.offset(4);
    x[6] = *input.offset(9);
    x[7] = *input.offset(6);
    x[8] = *input.offset(7);
    x[9] = *input.offset(8);
    x[10] = *input.offset(5);
    x[11] = *input.offset(10);
    x[12] = *input.offset(3);
    x[13] = *input.offset(12);
    x[14] = *input.offset(1);
    x[15] = *input.offset(14);
    // stage 2
    btf_16_sse2!(
        cospi_p02_p62,
        cospi_p62_m02,
        x[0],
        x[1],
        x[0],
        x[1],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p10_p54,
        cospi_p54_m10,
        x[2],
        x[3],
        x[2],
        x[3],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p18_p46,
        cospi_p46_m18,
        x[4],
        x[5],
        x[4],
        x[5],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p26_p38,
        cospi_p38_m26,
        x[6],
        x[7],
        x[6],
        x[7],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p34_p30,
        cospi_p30_m34,
        x[8],
        x[9],
        x[8],
        x[9],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p42_p22,
        cospi_p22_m42,
        x[10],
        x[11],
        x[10],
        x[11],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p50_p14,
        cospi_p14_m50,
        x[12],
        x[13],
        x[12],
        x[13],
        __rounding,
        cos_bit
    );
    btf_16_sse2!(
        cospi_p58_p06,
        cospi_p06_m58,
        x[14],
        x[15],
        x[14],
        x[15],
        __rounding,
        cos_bit
    );
    // stage 3~9
    iadst16_stage3_ssse3(x.as_mut_ptr());
    iadst16_stage4_ssse3(x.as_mut_ptr(), cospi, __rounding, cos_bit);
    iadst16_stage5_ssse3(x.as_mut_ptr());
    iadst16_stage6_ssse3(x.as_mut_ptr(), cospi, __rounding, cos_bit);
    iadst16_stage7_ssse3(x.as_mut_ptr());
    iadst16_stage8_ssse3(x.as_mut_ptr(), cospi, __rounding, cos_bit);
    iadst16_stage9_ssse3(output, x.as_mut_ptr());
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn iadst16_w4_sse2(input: *const __m128i, output: *mut __m128i) {
    let cos_bit: i8 = INV_COS_BIT as i8;
    let mut cospi: *const i32 = cospi_arr(INV_COS_BIT);
    let __rounding: __m128i = _mm_set1_epi32(1 << (INV_COS_BIT - 1));
    let cospi_p02_p62: __m128i = pair_set_epi16(*cospi.offset(2), *cospi.offset(62));
    let cospi_p62_m02: __m128i = pair_set_epi16(*cospi.offset(62), -*cospi.offset(2));
    let cospi_p10_p54: __m128i = pair_set_epi16(*cospi.offset(10), *cospi.offset(54));
    let cospi_p54_m10: __m128i = pair_set_epi16(*cospi.offset(54), -*cospi.offset(10));
    let cospi_p18_p46: __m128i = pair_set_epi16(*cospi.offset(18), *cospi.offset(46));
    let cospi_p46_m18: __m128i = pair_set_epi16(*cospi.offset(46), -*cospi.offset(18));
    let cospi_p26_p38: __m128i = pair_set_epi16(*cospi.offset(26), *cospi.offset(38));
    let cospi_p38_m26: __m128i = pair_set_epi16(*cospi.offset(38), -*cospi.offset(26));
    let cospi_p34_p30: __m128i = pair_set_epi16(*cospi.offset(34), *cospi.offset(30));
    let cospi_p30_m34: __m128i = pair_set_epi16(*cospi.offset(30), -*cospi.offset(34));
    let cospi_p42_p22: __m128i = pair_set_epi16(*cospi.offset(42), *cospi.offset(22));
    let cospi_p22_m42: __m128i = pair_set_epi16(*cospi.offset(22), -*cospi.offset(42));
    let cospi_p50_p14: __m128i = pair_set_epi16(*cospi.offset(50), *cospi.offset(14));
    let cospi_p14_m50: __m128i = pair_set_epi16(*cospi.offset(14), -*cospi.offset(50));
    let cospi_p58_p06: __m128i = pair_set_epi16(*cospi.offset(58), *cospi.offset(6));
    let cospi_p06_m58: __m128i = pair_set_epi16(*cospi.offset(6), -*cospi.offset(58));
    let cospi_p08_p56: __m128i = pair_set_epi16(*cospi.offset(8), *cospi.offset(56));
    let cospi_p56_m08: __m128i = pair_set_epi16(*cospi.offset(56), -*cospi.offset(8));
    let cospi_p40_p24: __m128i = pair_set_epi16(*cospi.offset(40), *cospi.offset(24));
    let cospi_p24_m40: __m128i = pair_set_epi16(*cospi.offset(24), -*cospi.offset(40));
    let cospi_m56_p08: __m128i = pair_set_epi16(-*cospi.offset(56), *cospi.offset(8));
    let cospi_m24_p40: __m128i = pair_set_epi16(-*cospi.offset(24), *cospi.offset(40));
    let cospi_p16_p48: __m128i = pair_set_epi16(*cospi.offset(16), *cospi.offset(48));
    let cospi_p48_m16: __m128i = pair_set_epi16(*cospi.offset(48), -*cospi.offset(16));
    let cospi_m48_p16: __m128i = pair_set_epi16(-*cospi.offset(48), *cospi.offset(16));
    let cospi_p32_p32: __m128i = pair_set_epi16(*cospi.offset(32), *cospi.offset(32));
    let cospi_p32_m32: __m128i = pair_set_epi16(*cospi.offset(32), -*cospi.offset(32));
    // stage 1
    let mut x: [__m128i; 16] = core::mem::zeroed();
    x[0] = *input.offset(15);
    x[1] = *input;
    x[2] = *input.offset(13);
    x[3] = *input.offset(2);
    x[4] = *input.offset(11);
    x[5] = *input.offset(4);
    x[6] = *input.offset(9);
    x[7] = *input.offset(6);
    x[8] = *input.offset(7);
    x[9] = *input.offset(8);
    x[10] = *input.offset(5);
    x[11] = *input.offset(10);
    x[12] = *input.offset(3);
    x[13] = *input.offset(12);
    x[14] = *input.offset(1);
    x[15] = *input.offset(14);
    // stage 2
    btf_16_4p_sse2!(
        cospi_p02_p62,
        cospi_p62_m02,
        x[0],
        x[1],
        x[0],
        x[1],
        __rounding,
        cos_bit
    );
    btf_16_4p_sse2!(
        cospi_p10_p54,
        cospi_p54_m10,
        x[2],
        x[3],
        x[2],
        x[3],
        __rounding,
        cos_bit
    );
    btf_16_4p_sse2!(
        cospi_p18_p46,
        cospi_p46_m18,
        x[4],
        x[5],
        x[4],
        x[5],
        __rounding,
        cos_bit
    );
    btf_16_4p_sse2!(
        cospi_p26_p38,
        cospi_p38_m26,
        x[6],
        x[7],
        x[6],
        x[7],
        __rounding,
        cos_bit
    );
    btf_16_4p_sse2!(
        cospi_p34_p30,
        cospi_p30_m34,
        x[8],
        x[9],
        x[8],
        x[9],
        __rounding,
        cos_bit
    );
    btf_16_4p_sse2!(
        cospi_p42_p22,
        cospi_p22_m42,
        x[10],
        x[11],
        x[10],
        x[11],
        __rounding,
        cos_bit
    );
    btf_16_4p_sse2!(
        cospi_p50_p14,
        cospi_p14_m50,
        x[12],
        x[13],
        x[12],
        x[13],
        __rounding,
        cos_bit
    );
    btf_16_4p_sse2!(
        cospi_p58_p06,
        cospi_p06_m58,
        x[14],
        x[15],
        x[14],
        x[15],
        __rounding,
        cos_bit
    );
    // stage 3
    iadst16_stage3_ssse3(x.as_mut_ptr());
    // stage 4
    btf_16_4p_sse2!(
        cospi_p08_p56,
        cospi_p56_m08,
        x[8],
        x[9],
        x[8],
        x[9],
        __rounding,
        cos_bit
    );
    btf_16_4p_sse2!(
        cospi_p40_p24,
        cospi_p24_m40,
        x[10],
        x[11],
        x[10],
        x[11],
        __rounding,
        cos_bit
    );
    btf_16_4p_sse2!(
        cospi_m56_p08,
        cospi_p08_p56,
        x[12],
        x[13],
        x[12],
        x[13],
        __rounding,
        cos_bit
    );
    btf_16_4p_sse2!(
        cospi_m24_p40,
        cospi_p40_p24,
        x[14],
        x[15],
        x[14],
        x[15],
        __rounding,
        cos_bit
    );
    // stage 5
    iadst16_stage5_ssse3(x.as_mut_ptr());
    // stage 6
    btf_16_4p_sse2!(
        cospi_p16_p48,
        cospi_p48_m16,
        x[4],
        x[5],
        x[4],
        x[5],
        __rounding,
        cos_bit
    );
    btf_16_4p_sse2!(
        cospi_m48_p16,
        cospi_p16_p48,
        x[6],
        x[7],
        x[6],
        x[7],
        __rounding,
        cos_bit
    );
    btf_16_4p_sse2!(
        cospi_p16_p48,
        cospi_p48_m16,
        x[12],
        x[13],
        x[12],
        x[13],
        __rounding,
        cos_bit
    );
    btf_16_4p_sse2!(
        cospi_m48_p16,
        cospi_p16_p48,
        x[14],
        x[15],
        x[14],
        x[15],
        __rounding,
        cos_bit
    );
    // stage 7
    iadst16_stage7_ssse3(x.as_mut_ptr());
    // stage 8
    btf_16_4p_sse2!(
        cospi_p32_p32,
        cospi_p32_m32,
        x[2],
        x[3],
        x[2],
        x[3],
        __rounding,
        cos_bit
    );
    btf_16_4p_sse2!(
        cospi_p32_p32,
        cospi_p32_m32,
        x[6],
        x[7],
        x[6],
        x[7],
        __rounding,
        cos_bit
    );
    btf_16_4p_sse2!(
        cospi_p32_p32,
        cospi_p32_m32,
        x[10],
        x[11],
        x[10],
        x[11],
        __rounding,
        cos_bit
    );
    btf_16_4p_sse2!(
        cospi_p32_p32,
        cospi_p32_m32,
        x[14],
        x[15],
        x[14],
        x[15],
        __rounding,
        cos_bit
    );
    // stage 9
    iadst16_stage9_ssse3(output, x.as_mut_ptr());
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn iidentity4_ssse3(input: *const __m128i, output: *mut __m128i) {
    let scale_fractional: i16 = (NewSqrt2 - (1 << NewSqrt2Bits)) as i16;
    let scale: __m128i = _mm_set1_epi16(((scale_fractional as i32) << (15 - NewSqrt2Bits)) as i16);
    for i in 0..4 {
        let mut x: __m128i = _mm_mulhrs_epi16(*input.offset(i as isize), scale);
        *output.offset(i as isize) = _mm_adds_epi16(x, *input.offset(i as isize));
    }
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn iidentity8_sse2(input: *const __m128i, output: *mut __m128i) {
    for i in 0..8 {
        *output.offset(i as isize) =
            _mm_adds_epi16(*input.offset(i as isize), *input.offset(i as isize));
    }
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn iidentity16_ssse3(input: *const __m128i, output: *mut __m128i) {
    let scale_fractional: i16 = (2 * (NewSqrt2 - (1 << NewSqrt2Bits))) as i16;
    let scale: __m128i = _mm_set1_epi16(((scale_fractional as i32) << (15 - NewSqrt2Bits)) as i16);
    for i in 0..16 {
        let mut x: __m128i = _mm_mulhrs_epi16(*input.offset(i as isize), scale);
        let mut srcx2: __m128i =
            _mm_adds_epi16(*input.offset(i as isize), *input.offset(i as isize));
        *output.offset(i as isize) = _mm_adds_epi16(x, srcx2);
    }
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn lowbd_get_recon_8x8_sse2(pred: __m128i, res: __m128i) -> __m128i {
    let zero: __m128i = _mm_setzero_si128();
    let mut x0: __m128i = _mm_adds_epi16(res, _mm_unpacklo_epi8(pred, zero));
    return _mm_packus_epi16(x0, x0);
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn lowbd_write_buffer_4xn_sse2(
    in_: *mut __m128i,
    output: *mut u8,
    stride: i32,
    flipud: i32,
    height: i32,
) {
    let mut j: i32 = if flipud != 0 { height - 1 } else { 0 };
    let step: i32 = if flipud != 0 { -1 } else { 1 };
    let zero: __m128i = _mm_setzero_si128();
    {
        let mut i: i32 = 0;
        while i < height {
            let v: __m128i = _mm_cvtsi32_si128(core::ptr::read_unaligned(
                output.offset((i * stride) as isize) as *mut i32,
            ));
            let mut u: __m128i =
                _mm_adds_epi16(*in_.offset(j as isize), _mm_unpacklo_epi8(v, zero));
            u = _mm_packus_epi16(u, zero);
            core::ptr::write_unaligned(
                output.offset((i * stride) as isize) as *mut i32,
                _mm_cvtsi128_si32(u),
            );
            i += 1;
            j += step;
        }
    }
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn lowbd_write_buffer_8xn_sse2(
    in_: *mut __m128i,
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
            let v: __m128i =
                _mm_loadl_epi64(output.offset((i * stride) as isize) as *const __m128i);
            let u: __m128i = lowbd_get_recon_8x8_sse2(v, *in_.offset(j as isize));
            _mm_storel_epi64(output.offset((i * stride) as isize) as *mut __m128i, u);
            i += 1;
            j += step;
        }
    }
}

// 1D functions process process 8 pixels at one time.
static lowbd_txfm_all_1d_w8_arr: [[transform_1d_ssse3; (ITX_TYPES_1D) as usize];
    (TX_SIZES) as usize] = [
    [Some(idct4_sse2), Some(iadst4_sse2), Some(iidentity4_ssse3)],
    [
        Some(av1_idct8_sse2),
        Some(av1_iadst8_sse2),
        Some(iidentity8_sse2),
    ],
    [
        Some(idct16_sse2),
        Some(iadst16_sse2),
        Some(iidentity16_ssse3),
    ],
    [Some(idct32_sse2), None, None],
    [Some(idct64_low32_ssse3), None, None],
];

// functions for blocks with eob at DC and within
// topleft 8x8, 16x16, 32x32 corner
static lowbd_txfm_all_1d_zeros_w8_arr: [[[transform_1d_ssse3; 4]; (ITX_TYPES_1D) as usize];
    (TX_SIZES) as usize] = [
    [
        [Some(idct4_sse2), Some(idct4_sse2), None, None],
        [Some(iadst4_sse2), Some(iadst4_sse2), None, None],
        [Some(iidentity4_ssse3), Some(iidentity4_ssse3), None, None],
    ],
    [
        [Some(av1_idct8_low1_ssse3), Some(av1_idct8_sse2), None, None],
        [
            Some(av1_iadst8_low1_ssse3),
            Some(av1_iadst8_sse2),
            None,
            None,
        ],
        [Some(iidentity8_sse2), Some(iidentity8_sse2), None, None],
    ],
    [
        [
            Some(idct16_low1_ssse3),
            Some(idct16_low8_ssse3),
            Some(idct16_sse2),
            None,
        ],
        [
            Some(iadst16_low1_ssse3),
            Some(iadst16_low8_ssse3),
            Some(iadst16_sse2),
            None,
        ],
        [None, None, None, None],
    ],
    [
        [
            Some(idct32_low1_ssse3),
            Some(idct32_low8_ssse3),
            Some(idct32_low16_ssse3),
            Some(idct32_sse2),
        ],
        [None, None, None, None],
        [None, None, None, None],
    ],
    [
        [
            Some(idct64_low1_ssse3),
            Some(idct64_low8_ssse3),
            Some(idct64_low16_ssse3),
            Some(idct64_low32_ssse3),
        ],
        [None, None, None, None],
        [None, None, None, None],
    ],
];

// 1D functions process process 4 pixels at one time.
// used in 4x4, 4x8, 4x16, 8x4, 16x4
static lowbd_txfm_all_1d_w4_arr: [[transform_1d_ssse3; (ITX_TYPES_1D) as usize];
    (TX_SIZES) as usize] = [
    [
        Some(idct4_w4_sse2),
        Some(iadst4_w4_sse2),
        Some(iidentity4_ssse3),
    ],
    [
        Some(idct8_w4_sse2),
        Some(iadst8_w4_sse2),
        Some(iidentity8_sse2),
    ],
    [
        Some(idct16_w4_sse2),
        Some(iadst16_w4_sse2),
        Some(iidentity16_ssse3),
    ],
    [None, None, None],
    [None, None, None],
];

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn iidentity_row_8xn_ssse3(
    out: *mut __m128i,
    input: *const i32,
    stride: i32,
    shift: i32,
    height: i32,
    txw_idx: i32,
    rect_type: i32,
) {
    let mut input_row: *const i32 = input;
    let scale: __m128i = _mm_set1_epi16(NewSqrt2list[txw_idx as usize] as i16);
    let rounding: __m128i =
        _mm_set1_epi16(((1 << (NewSqrt2Bits - 1)) + (1 << (NewSqrt2Bits - shift - 1))) as i16);
    let one: __m128i = _mm_set1_epi16(1);
    let scale_rounding: __m128i = _mm_unpacklo_epi16(scale, rounding);
    if rect_type != 1 && rect_type != -1 {
        for i in 0..height {
            let src: __m128i = load_32bit_to_16bit(input_row);
            input_row = input_row.offset(stride as isize);
            let mut lo: __m128i = _mm_unpacklo_epi16(src, one);
            let mut hi: __m128i = _mm_unpackhi_epi16(src, one);
            lo = _mm_madd_epi16(lo, scale_rounding);
            hi = _mm_madd_epi16(hi, scale_rounding);
            lo = _mm_sra_epi32(lo, _mm_cvtsi32_si128(NewSqrt2Bits - shift));
            hi = _mm_sra_epi32(hi, _mm_cvtsi32_si128(NewSqrt2Bits - shift));
            *out.offset(i as isize) = _mm_packs_epi32(lo, hi);
        }
    } else {
        let rect_scale: __m128i = _mm_set1_epi16((NewInvSqrt2 << (15 - NewSqrt2Bits)) as i16);
        for i in 0..height {
            let mut src: __m128i = load_32bit_to_16bit(input_row);
            src = _mm_mulhrs_epi16(src, rect_scale);
            input_row = input_row.offset(stride as isize);
            let mut lo: __m128i = _mm_unpacklo_epi16(src, one);
            let mut hi: __m128i = _mm_unpackhi_epi16(src, one);
            lo = _mm_madd_epi16(lo, scale_rounding);
            hi = _mm_madd_epi16(hi, scale_rounding);
            lo = _mm_sra_epi32(lo, _mm_cvtsi32_si128(NewSqrt2Bits - shift));
            hi = _mm_sra_epi32(hi, _mm_cvtsi32_si128(NewSqrt2Bits - shift));
            *out.offset(i as isize) = _mm_packs_epi32(lo, hi);
        }
    }
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn iidentity_col_8xn_ssse3(
    mut output: *mut u8,
    stride: i32,
    buf: *mut __m128i,
    shift: i32,
    height: i32,
    txh_idx: i32,
) {
    let scale: __m128i = _mm_set1_epi16(NewSqrt2list[txh_idx as usize] as i16);
    let scale_rounding: __m128i = _mm_set1_epi16((1 << (NewSqrt2Bits - 1)) as i16);
    let shift_rounding: __m128i = _mm_set1_epi32(1 << (-shift - 1));
    let one: __m128i = _mm_set1_epi16(1);
    let scale_coeff: __m128i = _mm_unpacklo_epi16(scale, scale_rounding);
    let zero: __m128i = _mm_setzero_si128();
    for h in 0..height {
        let mut lo: __m128i = _mm_unpacklo_epi16(*buf.offset(h as isize), one);
        let mut hi: __m128i = _mm_unpackhi_epi16(*buf.offset(h as isize), one);
        lo = _mm_madd_epi16(lo, scale_coeff);
        hi = _mm_madd_epi16(hi, scale_coeff);
        lo = _mm_srai_epi32::<{ NewSqrt2Bits }>(lo);
        hi = _mm_srai_epi32::<{ NewSqrt2Bits }>(hi);
        lo = _mm_add_epi32(lo, shift_rounding);
        hi = _mm_add_epi32(hi, shift_rounding);
        lo = _mm_sra_epi32(lo, _mm_cvtsi32_si128(-shift));
        hi = _mm_sra_epi32(hi, _mm_cvtsi32_si128(-shift));
        let mut x: __m128i = _mm_packs_epi32(lo, hi);
        let pred: __m128i = _mm_loadl_epi64(output as *const __m128i);
        x = _mm_adds_epi16(x, _mm_unpacklo_epi8(pred, zero));
        let u: __m128i = _mm_packus_epi16(x, x);
        _mm_storel_epi64(output as *mut __m128i, u);
        output = output.offset(stride as isize);
    }
}

#[target_feature(enable = "ssse3")]
pub unsafe fn av1_lowbd_inv_txfm2d_add_idtx_ssse3(
    input: *const i32,
    output: *mut u8,
    stride: i32,
    tx_size: u8,
) {
    let mut shift: *const i8 = av1_inv_txfm_shift_ls[tx_size as usize];
    let txw_idx: i32 = get_txw_idx(tx_size);
    let txh_idx: i32 = get_txh_idx(tx_size);
    let txfm_size_col: i32 = tx_size_wide[tx_size as usize];
    let txfm_size_row: i32 = tx_size_high[tx_size as usize];
    let col_max: i32 = AOMMIN(32, txfm_size_col);
    let row_max: i32 = AOMMIN(32, txfm_size_row);
    let input_stride: i32 = row_max;
    let rect_type: i32 = get_rect_tx_log_ratio(txfm_size_col, txfm_size_row);
    for i in 0..(col_max >> 3) {
        for j in 0..(row_max >> 3) {
            let mut buf: [__m128i; 8] = core::mem::zeroed();
            iidentity_row_8xn_ssse3(
                buf.as_mut_ptr(),
                input
                    .offset((j * 8) as isize)
                    .offset((i * 8 * input_stride) as isize),
                row_max,
                *shift as i32,
                8,
                txw_idx,
                rect_type,
            );
            transpose_16bit_8x8(buf.as_mut_ptr() as *const __m128i, buf.as_mut_ptr());
            iidentity_col_8xn_ssse3(
                output
                    .offset((i * 8) as isize)
                    .offset((j * 8 * stride) as isize),
                stride,
                buf.as_mut_ptr(),
                *shift.offset(1) as i32,
                8,
                txh_idx,
            );
        }
    }
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn lowbd_inv_txfm2d_add_4x4_ssse3(
    input: *const i32,
    output: *mut u8,
    stride: i32,
    tx_type: u8,
    tx_size_: u8,
    eob: i32,
) {
    let _ = tx_size_;
    let _ = eob;
    let mut buf: [__m128i; 4] = core::mem::zeroed();
    let tx_size: u8 = TX_4X4 as u8;
    let mut shift: *const i8 = av1_inv_txfm_shift_ls[tx_size as usize];
    let txw_idx: i32 = get_txw_idx(tx_size);
    let txh_idx: i32 = get_txh_idx(tx_size);
    let txfm_size_col: i32 = tx_size_wide[tx_size as usize];
    let txfm_size_row: i32 = tx_size_high[tx_size as usize];
    let row_txfm: transform_1d_ssse3 =
        lowbd_txfm_all_1d_w4_arr[txw_idx as usize][hitx_1d_tab[tx_type as usize] as usize];
    let col_txfm: transform_1d_ssse3 =
        lowbd_txfm_all_1d_w4_arr[txh_idx as usize][vitx_1d_tab[tx_type as usize] as usize];
    let mut ud_flip: i32 = 0;
    let mut lr_flip: i32 = 0;
    get_flip_cfg(tx_type, &raw mut ud_flip, &raw mut lr_flip);
    load_buffer_32bit_to_16bit_w4(input, txfm_size_row, buf.as_mut_ptr(), txfm_size_col);
    row_txfm.unwrap()(buf.as_mut_ptr() as *const __m128i, buf.as_mut_ptr());
    if lr_flip != 0 {
        let mut temp: [__m128i; 4] = core::mem::zeroed();
        flip_buf_sse2(buf.as_mut_ptr(), temp.as_mut_ptr(), txfm_size_col);
        transpose_16bit_4x4(temp.as_mut_ptr() as *const __m128i, buf.as_mut_ptr());
    } else {
        transpose_16bit_4x4(buf.as_mut_ptr() as *const __m128i, buf.as_mut_ptr());
    }
    col_txfm.unwrap()(buf.as_mut_ptr() as *const __m128i, buf.as_mut_ptr());
    round_shift_16bit_ssse3(buf.as_mut_ptr(), txfm_size_row, *shift.offset(1) as i32);
    lowbd_write_buffer_4xn_sse2(buf.as_mut_ptr(), output, stride, ud_flip, txfm_size_row);
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn lowbd_get_recon_16x16_sse2(pred: __m128i, res0: __m128i, res1: __m128i) -> __m128i {
    let zero: __m128i = _mm_setzero_si128();
    let mut x0: __m128i = _mm_unpacklo_epi8(pred, zero);
    let mut x1: __m128i = _mm_unpackhi_epi8(pred, zero);
    x0 = _mm_adds_epi16(res0, x0);
    x1 = _mm_adds_epi16(res1, x1);
    return _mm_packus_epi16(x0, x1);
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn lowbd_write_buffer_16xn_sse2(
    in_: *mut __m128i,
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
            let mut v: __m128i =
                _mm_loadu_si128(output.offset((i * stride) as isize) as *const __m128i);
            let mut u: __m128i = lowbd_get_recon_16x16_sse2(
                v,
                *in_.offset(j as isize),
                *in_.offset((j + height) as isize),
            );
            _mm_storeu_si128(output.offset((i * stride) as isize) as *mut __m128i, u);
            i += 1;
            j += step;
        }
    }
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn round_shift_ssse3(input: *const __m128i, output: *mut __m128i, size: i32) {
    let scale: __m128i = _mm_set1_epi16((NewInvSqrt2 * 8) as i16);
    for i in 0..size {
        *output.offset(i as isize) = _mm_mulhrs_epi16(*input.offset(i as isize), scale);
    }
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn lowbd_inv_txfm2d_add_no_identity_ssse3(
    input: *const i32,
    output: *mut u8,
    stride: i32,
    tx_type: u8,
    tx_size: u8,
    eob: i32,
) {
    let mut buf1: [__m128i; (64 * 8) as usize] = core::mem::zeroed();
    let mut eobx: i32 = 0;
    let mut eoby: i32 = 0;
    get_eobx_eoby_scan_default(&raw mut eobx, &raw mut eoby, tx_size, eob);
    let mut shift: *const i8 = av1_inv_txfm_shift_ls[tx_size as usize];
    let txw_idx: i32 = get_txw_idx(tx_size);
    let txh_idx: i32 = get_txh_idx(tx_size);
    let txfm_size_col: i32 = tx_size_wide[tx_size as usize];
    let txfm_size_row: i32 = tx_size_high[tx_size as usize];
    let buf_size_w_div8: i32 = txfm_size_col >> 3;
    let buf_size_nonzero_w: i32 = ((eobx + 8) >> 3) << 3;
    let buf_size_nonzero_h_div8: i32 = (eoby + 8) >> 3;
    let input_stride: i32 = AOMMIN(32, txfm_size_row);
    let rect_type: i32 = get_rect_tx_log_ratio(txfm_size_col, txfm_size_row);
    let fun_idx_x: i32 = lowbd_txfm_all_1d_zeros_idx[eobx as usize];
    let fun_idx_y: i32 = lowbd_txfm_all_1d_zeros_idx[eoby as usize];
    let row_txfm: transform_1d_ssse3 = lowbd_txfm_all_1d_zeros_w8_arr[txw_idx as usize]
        [hitx_1d_tab[tx_type as usize] as usize][fun_idx_x as usize];
    let col_txfm: transform_1d_ssse3 = lowbd_txfm_all_1d_zeros_w8_arr[txh_idx as usize]
        [vitx_1d_tab[tx_type as usize] as usize][fun_idx_y as usize];
    debug_assert!(!col_txfm.is_none());
    debug_assert!(!row_txfm.is_none());
    let mut ud_flip: i32 = 0;
    let mut lr_flip: i32 = 0;
    get_flip_cfg(tx_type, &raw mut ud_flip, &raw mut lr_flip);
    for i in 0..buf_size_nonzero_h_div8 {
        let mut buf0: [__m128i; 64] = core::mem::zeroed();
        load_buffer_32bit_to_16bit(
            input.offset((8 * i) as isize),
            input_stride,
            buf0.as_mut_ptr(),
            buf_size_nonzero_w,
        );
        if rect_type == 1 || rect_type == -1 {
            round_shift_ssse3(
                buf0.as_mut_ptr() as *const __m128i,
                buf0.as_mut_ptr(),
                buf_size_nonzero_w,
            );
        } // rect special code
        row_txfm.unwrap()(buf0.as_mut_ptr() as *const __m128i, buf0.as_mut_ptr());
        round_shift_16bit_ssse3(buf0.as_mut_ptr(), txfm_size_col, *shift as i32);
        let mut _buf1: *mut __m128i = buf1.as_mut_ptr().offset((i * 8) as isize);
        if lr_flip != 0 {
            for j in 0..buf_size_w_div8 {
                let mut temp: [__m128i; 8] = core::mem::zeroed();
                flip_buf_sse2(
                    buf0.as_mut_ptr().offset((8 * j) as isize),
                    temp.as_mut_ptr(),
                    8,
                );
                transpose_16bit_8x8(
                    temp.as_mut_ptr() as *const __m128i,
                    _buf1.offset((txfm_size_row * (buf_size_w_div8 - 1 - j)) as isize),
                );
            }
        } else {
            for j in 0..buf_size_w_div8 {
                transpose_16bit_8x8(
                    buf0.as_mut_ptr().offset((8 * j) as isize) as *const __m128i,
                    _buf1.offset((txfm_size_row * j) as isize),
                );
            }
        }
    }
    for i in 0..buf_size_w_div8 {
        col_txfm.unwrap()(
            buf1.as_mut_ptr().offset((i * txfm_size_row) as isize) as *const __m128i,
            buf1.as_mut_ptr().offset((i * txfm_size_row) as isize),
        );
        round_shift_16bit_ssse3(
            buf1.as_mut_ptr().offset((i * txfm_size_row) as isize),
            txfm_size_row,
            *shift.offset(1) as i32,
        );
    }
    if txfm_size_col >= 16 {
        for i in 0..(txfm_size_col >> 4) {
            lowbd_write_buffer_16xn_sse2(
                buf1.as_mut_ptr().offset((i * txfm_size_row * 2) as isize),
                output.offset((16 * i) as isize),
                stride,
                ud_flip,
                txfm_size_row,
            );
        }
    } else if txfm_size_col == 8 {
        lowbd_write_buffer_8xn_sse2(buf1.as_mut_ptr(), output, stride, ud_flip, txfm_size_row);
    }
}

#[target_feature(enable = "ssse3")]
pub unsafe fn av1_lowbd_inv_txfm2d_add_h_identity_ssse3(
    input: *const i32,
    output: *mut u8,
    stride: i32,
    tx_type: u8,
    tx_size: u8,
    eob: i32,
) {
    let mut shift: *const i8 = av1_inv_txfm_shift_ls[tx_size as usize];
    let mut eobx: i32 = 0;
    let mut eoby: i32 = 0;
    get_eobx_eoby_scan_h_identity(&raw mut eobx, &raw mut eoby, tx_size, eob);
    let txw_idx: i32 = get_txw_idx(tx_size);
    let txh_idx: i32 = get_txh_idx(tx_size);
    let txfm_size_col: i32 = tx_size_wide[tx_size as usize];
    let txfm_size_row: i32 = tx_size_high[tx_size as usize];
    let buf_size_w_div8: i32 = (eobx + 8) >> 3;
    let buf_size_h_div8: i32 = (eoby + 8) >> 3;
    let input_stride: i32 = AOMMIN(32, txfm_size_row);
    let rect_type: i32 = get_rect_tx_log_ratio(txfm_size_col, txfm_size_row);
    let fun_idx: i32 = lowbd_txfm_all_1d_zeros_idx[eoby as usize];
    debug_assert!(fun_idx < 5);
    let col_txfm: transform_1d_ssse3 = lowbd_txfm_all_1d_zeros_w8_arr[txh_idx as usize]
        [vitx_1d_tab[tx_type as usize] as usize][fun_idx as usize];
    debug_assert!(!col_txfm.is_none());
    let mut ud_flip: i32 = 0;
    let mut lr_flip: i32 = 0;
    get_flip_cfg(tx_type, &raw mut ud_flip, &raw mut lr_flip);
    for i in 0..buf_size_w_div8 {
        let mut buf0: [__m128i; 64] = core::mem::zeroed();
        for j in 0..buf_size_h_div8 {
            let mut buf0_cur: *mut __m128i = buf0.as_mut_ptr().offset((j * 8) as isize);
            let mut input_cur: *const i32 = input
                .offset((i * 8 * input_stride) as isize)
                .offset((j * 8) as isize);
            iidentity_row_8xn_ssse3(
                buf0_cur,
                input_cur,
                input_stride,
                *shift as i32,
                8,
                txw_idx,
                rect_type,
            );
            transpose_16bit_8x8(buf0_cur as *const __m128i, buf0_cur);
        }
        col_txfm.unwrap()(buf0.as_mut_ptr() as *const __m128i, buf0.as_mut_ptr());
        let mut mshift: __m128i = _mm_set1_epi16((1 << (15 + *shift.offset(1) as i32)) as i16);
        let mut k: i32 = if ud_flip != 0 { txfm_size_row - 1 } else { 0 };
        let step: i32 = if ud_flip != 0 { -1 } else { 1 };
        let mut out: *mut u8 = output.offset((8 * i) as isize);
        {
            let mut j: i32 = 0;
            while j < txfm_size_row {
                let v: __m128i = _mm_loadl_epi64(out as *const __m128i);
                let mut res: __m128i = _mm_mulhrs_epi16(buf0[k as usize], mshift);
                let u: __m128i = lowbd_get_recon_8x8_sse2(v, res);
                _mm_storel_epi64(out as *mut __m128i, u);
                out = out.offset(stride as isize);
                j += 1;
                k += step;
            }
        }
    }
}

#[target_feature(enable = "ssse3")]
pub unsafe fn av1_lowbd_inv_txfm2d_add_v_identity_ssse3(
    input: *const i32,
    output: *mut u8,
    stride: i32,
    tx_type: u8,
    tx_size: u8,
    eob: i32,
) {
    let mut buf1: [__m128i; 64] = core::mem::zeroed();
    let mut eobx: i32 = 0;
    let mut eoby: i32 = 0;
    get_eobx_eoby_scan_v_identity(&raw mut eobx, &raw mut eoby, tx_size, eob);
    let mut shift: *const i8 = av1_inv_txfm_shift_ls[tx_size as usize];
    let txw_idx: i32 = get_txw_idx(tx_size);
    let txh_idx: i32 = get_txh_idx(tx_size);
    let txfm_size_col: i32 = tx_size_wide[tx_size as usize];
    let txfm_size_row: i32 = tx_size_high[tx_size as usize];
    let buf_size_w_div8: i32 = txfm_size_col >> 3;
    let buf_size_nonzero_w: i32 = ((eobx + 8) >> 3) << 3;
    let buf_size_h_div8: i32 = (eoby + 8) >> 3;
    let input_stride: i32 = AOMMIN(32, txfm_size_row);
    let rect_type: i32 = get_rect_tx_log_ratio(txfm_size_col, txfm_size_row);
    let fun_idx: i32 = lowbd_txfm_all_1d_zeros_idx[eobx as usize];
    let row_txfm: transform_1d_ssse3 = lowbd_txfm_all_1d_zeros_w8_arr[txw_idx as usize]
        [hitx_1d_tab[tx_type as usize] as usize][fun_idx as usize];
    debug_assert!(!row_txfm.is_none());
    let mut ud_flip: i32 = 0;
    let mut lr_flip: i32 = 0;
    get_flip_cfg(tx_type, &raw mut ud_flip, &raw mut lr_flip);
    for i in 0..buf_size_h_div8 {
        let mut buf0: [__m128i; 64] = core::mem::zeroed();
        load_buffer_32bit_to_16bit(
            input.offset((i * 8) as isize),
            input_stride,
            buf0.as_mut_ptr(),
            buf_size_nonzero_w,
        );
        if rect_type == 1 || rect_type == -1 {
            round_shift_ssse3(
                buf0.as_mut_ptr() as *const __m128i,
                buf0.as_mut_ptr(),
                buf_size_nonzero_w,
            );
        } // rect special code
        row_txfm.unwrap()(buf0.as_mut_ptr() as *const __m128i, buf0.as_mut_ptr());
        round_shift_16bit_ssse3(buf0.as_mut_ptr(), txfm_size_col, *shift as i32);
        let mut _buf1: *mut __m128i = buf1.as_mut_ptr();
        if lr_flip != 0 {
            for j in 0..buf_size_w_div8 {
                let mut temp: [__m128i; 8] = core::mem::zeroed();
                flip_buf_sse2(
                    buf0.as_mut_ptr().offset((8 * j) as isize),
                    temp.as_mut_ptr(),
                    8,
                );
                transpose_16bit_8x8(
                    temp.as_mut_ptr() as *const __m128i,
                    _buf1.offset((8 * (buf_size_w_div8 - 1 - j)) as isize),
                );
            }
        } else {
            for j in 0..buf_size_w_div8 {
                transpose_16bit_8x8(
                    buf0.as_mut_ptr().offset((8 * j) as isize) as *const __m128i,
                    _buf1.offset((8 * j) as isize),
                );
            }
        }
        for j in 0..buf_size_w_div8 {
            iidentity_col_8xn_ssse3(
                output
                    .offset((i * 8 * stride) as isize)
                    .offset((j * 8) as isize),
                stride,
                buf1.as_mut_ptr().offset((j * 8) as isize),
                *shift.offset(1) as i32,
                8,
                txh_idx,
            );
        }
    }
}

// for 32x32,32x64,64x32,64x64,32x8,8x32,16x32,32x16,64x16,16x64
#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn lowbd_inv_txfm2d_add_universe_ssse3(
    input: *const i32,
    output: *mut u8,
    stride: i32,
    tx_type: u8,
    tx_size: u8,
    eob: i32,
) {
    match tx_type as i32 {
        DCT_DCT => {
            lowbd_inv_txfm2d_add_no_identity_ssse3(input, output, stride, tx_type, tx_size, eob);
        }
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
            lowbd_inv_txfm2d_add_no_identity_ssse3(input, output, stride, tx_type, tx_size, eob);
        }
    }
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn lowbd_inv_txfm2d_add_4x8_ssse3(
    input: *const i32,
    output: *mut u8,
    stride: i32,
    tx_type: u8,
    tx_size_: u8,
    eob: i32,
) {
    let _ = tx_size_;
    let _ = eob;
    let mut buf: [__m128i; 8] = core::mem::zeroed();
    let tx_size: u8 = TX_4X8 as u8;
    let mut shift: *const i8 = av1_inv_txfm_shift_ls[tx_size as usize];
    let txw_idx: i32 = get_txw_idx(tx_size);
    let txh_idx: i32 = get_txh_idx(tx_size);
    let txfm_size_col: i32 = tx_size_wide[tx_size as usize];
    let txfm_size_row: i32 = tx_size_high[tx_size as usize];
    let row_txfm: transform_1d_ssse3 =
        lowbd_txfm_all_1d_w8_arr[txw_idx as usize][hitx_1d_tab[tx_type as usize] as usize];
    let col_txfm: transform_1d_ssse3 =
        lowbd_txfm_all_1d_w4_arr[txh_idx as usize][vitx_1d_tab[tx_type as usize] as usize];
    let mut ud_flip: i32 = 0;
    let mut lr_flip: i32 = 0;
    get_flip_cfg(tx_type, &raw mut ud_flip, &raw mut lr_flip);
    load_buffer_32bit_to_16bit(input, txfm_size_row, buf.as_mut_ptr(), txfm_size_col);
    round_shift_ssse3(
        buf.as_mut_ptr() as *const __m128i,
        buf.as_mut_ptr(),
        txfm_size_col,
    ); // rect special code
    row_txfm.unwrap()(buf.as_mut_ptr() as *const __m128i, buf.as_mut_ptr());
    // round_shift_16bit_ssse3(buf, txfm_size_col, shift[0]);// shift[0] is 0
    if lr_flip != 0 {
        let mut temp: [__m128i; 4] = core::mem::zeroed();
        flip_buf_sse2(buf.as_mut_ptr(), temp.as_mut_ptr(), txfm_size_col);
        transpose_16bit_8x4(temp.as_mut_ptr() as *const __m128i, buf.as_mut_ptr());
    } else {
        transpose_16bit_8x4(buf.as_mut_ptr() as *const __m128i, buf.as_mut_ptr());
    }
    col_txfm.unwrap()(buf.as_mut_ptr() as *const __m128i, buf.as_mut_ptr());
    round_shift_16bit_ssse3(buf.as_mut_ptr(), txfm_size_row, *shift.offset(1) as i32);
    lowbd_write_buffer_4xn_sse2(buf.as_mut_ptr(), output, stride, ud_flip, txfm_size_row);
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn lowbd_inv_txfm2d_add_8x4_ssse3(
    input: *const i32,
    output: *mut u8,
    stride: i32,
    tx_type: u8,
    tx_size_: u8,
    eob: i32,
) {
    let _ = tx_size_;
    let _ = eob;
    let mut buf: [__m128i; 8] = core::mem::zeroed();
    let tx_size: u8 = TX_8X4 as u8;
    let mut shift: *const i8 = av1_inv_txfm_shift_ls[tx_size as usize];
    let txw_idx: i32 = get_txw_idx(tx_size);
    let txh_idx: i32 = get_txh_idx(tx_size);
    let txfm_size_col: i32 = tx_size_wide[tx_size as usize];
    let txfm_size_row: i32 = tx_size_high[tx_size as usize];
    let row_txfm: transform_1d_ssse3 =
        lowbd_txfm_all_1d_w4_arr[txw_idx as usize][hitx_1d_tab[tx_type as usize] as usize];
    let col_txfm: transform_1d_ssse3 =
        lowbd_txfm_all_1d_w8_arr[txh_idx as usize][vitx_1d_tab[tx_type as usize] as usize];
    let mut ud_flip: i32 = 0;
    let mut lr_flip: i32 = 0;
    get_flip_cfg(tx_type, &raw mut ud_flip, &raw mut lr_flip);
    load_buffer_32bit_to_16bit_w4(input, txfm_size_row, buf.as_mut_ptr(), txfm_size_col);
    round_shift_ssse3(
        buf.as_mut_ptr() as *const __m128i,
        buf.as_mut_ptr(),
        txfm_size_col,
    ); // rect special code
    row_txfm.unwrap()(buf.as_mut_ptr() as *const __m128i, buf.as_mut_ptr());
    // round_shift_16bit_ssse3(buf, txfm_size_col, shift[0]); // shift[0] is 0
    if lr_flip != 0 {
        let mut temp: [__m128i; 8] = core::mem::zeroed();
        flip_buf_sse2(buf.as_mut_ptr(), temp.as_mut_ptr(), txfm_size_col);
        transpose_16bit_4x8(temp.as_mut_ptr() as *const __m128i, buf.as_mut_ptr());
    } else {
        transpose_16bit_4x8(buf.as_mut_ptr() as *const __m128i, buf.as_mut_ptr());
    }
    col_txfm.unwrap()(buf.as_mut_ptr() as *const __m128i, buf.as_mut_ptr());
    round_shift_16bit_ssse3(buf.as_mut_ptr(), txfm_size_row, *shift.offset(1) as i32);
    lowbd_write_buffer_8xn_sse2(buf.as_mut_ptr(), output, stride, ud_flip, txfm_size_row);
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn lowbd_inv_txfm2d_add_4x16_ssse3(
    input: *const i32,
    output: *mut u8,
    stride: i32,
    tx_type: u8,
    tx_size_: u8,
    eob: i32,
) {
    let _ = tx_size_;
    let _ = eob;
    let mut buf: [__m128i; 16] = core::mem::zeroed();
    let tx_size: u8 = TX_4X16 as u8;
    let mut shift: *const i8 = av1_inv_txfm_shift_ls[tx_size as usize];
    let txw_idx: i32 = get_txw_idx(tx_size);
    let txh_idx: i32 = get_txh_idx(tx_size);
    let txfm_size_col: i32 = tx_size_wide[tx_size as usize];
    let txfm_size_row: i32 = tx_size_high[tx_size as usize];
    let row_txfm: transform_1d_ssse3 =
        lowbd_txfm_all_1d_w8_arr[txw_idx as usize][hitx_1d_tab[tx_type as usize] as usize];
    let col_txfm: transform_1d_ssse3 =
        lowbd_txfm_all_1d_w4_arr[txh_idx as usize][vitx_1d_tab[tx_type as usize] as usize];
    let mut ud_flip: i32 = 0;
    let mut lr_flip: i32 = 0;
    get_flip_cfg(tx_type, &raw mut ud_flip, &raw mut lr_flip);
    let row_one_loop: i32 = 8;
    for i in 0..2 {
        let mut input_cur: *const i32 = input.offset((i * row_one_loop) as isize);
        let mut buf_cur: *mut __m128i = buf.as_mut_ptr().offset((i * row_one_loop) as isize);
        load_buffer_32bit_to_16bit(input_cur, txfm_size_row, buf_cur, txfm_size_col);
        if row_txfm == Some(iidentity4_ssse3) {
            let scale: __m128i = pair_set_epi16(NewSqrt2, 3 << (NewSqrt2Bits - 1));
            let ones: __m128i = _mm_set1_epi16(1);
            for j in 0..4 {
                let buf_lo: __m128i = _mm_unpacklo_epi16(*buf_cur.offset(j as isize), ones);
                let buf_hi: __m128i = _mm_unpackhi_epi16(*buf_cur.offset(j as isize), ones);
                let buf_32_lo: __m128i =
                    _mm_srai_epi32::<{ NewSqrt2Bits + 1 }>(_mm_madd_epi16(buf_lo, scale));
                let buf_32_hi: __m128i =
                    _mm_srai_epi32::<{ NewSqrt2Bits + 1 }>(_mm_madd_epi16(buf_hi, scale));
                *buf_cur.offset(j as isize) = _mm_packs_epi32(buf_32_lo, buf_32_hi);
            }
        } else {
            row_txfm.unwrap()(buf_cur as *const __m128i, buf_cur);
            round_shift_16bit_ssse3(buf_cur, row_one_loop, *shift as i32);
        }
        if lr_flip != 0 {
            let mut temp: [__m128i; 8] = core::mem::zeroed();
            flip_buf_sse2(buf_cur, temp.as_mut_ptr(), txfm_size_col);
            transpose_16bit_8x4(temp.as_mut_ptr() as *const __m128i, buf_cur);
        } else {
            transpose_16bit_8x4(buf_cur as *const __m128i, buf_cur);
        }
    }
    col_txfm.unwrap()(buf.as_mut_ptr() as *const __m128i, buf.as_mut_ptr());
    round_shift_16bit_ssse3(buf.as_mut_ptr(), txfm_size_row, *shift.offset(1) as i32);
    lowbd_write_buffer_4xn_sse2(buf.as_mut_ptr(), output, stride, ud_flip, txfm_size_row);
}

#[inline]
#[target_feature(enable = "ssse3")]
unsafe fn lowbd_inv_txfm2d_add_16x4_ssse3(
    input: *const i32,
    output: *mut u8,
    stride: i32,
    tx_type: u8,
    tx_size_: u8,
    eob: i32,
) {
    let _ = tx_size_;
    let _ = eob;
    let mut buf: [__m128i; 16] = core::mem::zeroed();
    let tx_size: u8 = TX_16X4 as u8;
    let mut shift: *const i8 = av1_inv_txfm_shift_ls[tx_size as usize];
    let txw_idx: i32 = get_txw_idx(tx_size);
    let txh_idx: i32 = get_txh_idx(tx_size);
    let txfm_size_col: i32 = tx_size_wide[tx_size as usize];
    let txfm_size_row: i32 = tx_size_high[tx_size as usize];
    let buf_size_w_div8: i32 = txfm_size_col >> 3;
    let row_txfm: transform_1d_ssse3 =
        lowbd_txfm_all_1d_w4_arr[txw_idx as usize][hitx_1d_tab[tx_type as usize] as usize];
    let col_txfm: transform_1d_ssse3 =
        lowbd_txfm_all_1d_w8_arr[txh_idx as usize][vitx_1d_tab[tx_type as usize] as usize];
    let mut ud_flip: i32 = 0;
    let mut lr_flip: i32 = 0;
    get_flip_cfg(tx_type, &raw mut ud_flip, &raw mut lr_flip);
    let row_one_loop: i32 = 8;
    load_buffer_32bit_to_16bit_w4(input, txfm_size_row, buf.as_mut_ptr(), txfm_size_col);
    if row_txfm == Some(iidentity16_ssse3) {
        let scale: __m128i = pair_set_epi16(2 * NewSqrt2, 3 << (NewSqrt2Bits - 1));
        let ones: __m128i = _mm_set1_epi16(1);
        for j in 0..16 {
            let buf_lo: __m128i = _mm_unpacklo_epi16(buf[j as usize], ones);
            let buf_hi: __m128i = _mm_unpackhi_epi16(buf[j as usize], ones);
            let buf_32_lo: __m128i =
                _mm_srai_epi32::<{ NewSqrt2Bits + 1 }>(_mm_madd_epi16(buf_lo, scale));
            let buf_32_hi: __m128i =
                _mm_srai_epi32::<{ NewSqrt2Bits + 1 }>(_mm_madd_epi16(buf_hi, scale));
            buf[j as usize] = _mm_packs_epi32(buf_32_lo, buf_32_hi);
        }
    } else {
        row_txfm.unwrap()(buf.as_mut_ptr() as *const __m128i, buf.as_mut_ptr());
        round_shift_16bit_ssse3(buf.as_mut_ptr(), txfm_size_col, *shift as i32);
    }
    if lr_flip != 0 {
        let mut temp: [__m128i; 16] = core::mem::zeroed();
        flip_buf_sse2(buf.as_mut_ptr(), temp.as_mut_ptr(), 16);
        transpose_16bit_4x8(temp.as_mut_ptr() as *const __m128i, buf.as_mut_ptr());
        transpose_16bit_4x8(
            temp.as_mut_ptr().offset(8) as *const __m128i,
            buf.as_mut_ptr().offset(8),
        );
    } else {
        transpose_16bit_4x8(buf.as_mut_ptr() as *const __m128i, buf.as_mut_ptr());
        transpose_16bit_4x8(
            buf.as_mut_ptr().offset(row_one_loop as isize) as *const __m128i,
            buf.as_mut_ptr().offset(row_one_loop as isize),
        );
    }
    for i in 0..buf_size_w_div8 {
        col_txfm.unwrap()(
            buf.as_mut_ptr().offset((i * row_one_loop) as isize) as *const __m128i,
            buf.as_mut_ptr().offset((i * row_one_loop) as isize),
        );
        round_shift_16bit_ssse3(
            buf.as_mut_ptr().offset((i * row_one_loop) as isize),
            txfm_size_row,
            *shift.offset(1) as i32,
        );
    }
    lowbd_write_buffer_8xn_sse2(buf.as_mut_ptr(), output, stride, ud_flip, 4);
    lowbd_write_buffer_8xn_sse2(
        buf.as_mut_ptr().offset(8),
        output.offset(8),
        stride,
        ud_flip,
        4,
    );
}

#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn av1_lowbd_inv_txfm2d_add_ssse3(
    input: *const i32,
    output: *mut u8,
    stride: i32,
    tx_type: u8,
    tx_size: u8,
    eob: i32,
) {
    match tx_size as i32 {
        TX_4X4 => {
            lowbd_inv_txfm2d_add_4x4_ssse3(input, output, stride, tx_type, tx_size, eob);
        }
        TX_4X8 => {
            lowbd_inv_txfm2d_add_4x8_ssse3(input, output, stride, tx_type, tx_size, eob);
        }
        TX_8X4 => {
            lowbd_inv_txfm2d_add_8x4_ssse3(input, output, stride, tx_type, tx_size, eob);
        }
        TX_4X16 => {
            lowbd_inv_txfm2d_add_4x16_ssse3(input, output, stride, tx_type, tx_size, eob);
        }
        TX_16X4 => {
            lowbd_inv_txfm2d_add_16x4_ssse3(input, output, stride, tx_type, tx_size, eob);
        }
        _ => {
            lowbd_inv_txfm2d_add_universe_ssse3(input, output, stride, tx_type, tx_size, eob);
        }
    }
}

#[no_mangle]
#[target_feature(enable = "ssse3")]
pub unsafe extern "C-unwind" fn av1_inv_txfm_add_ssse3(
    dqcoeff: *const i32,
    dst: *mut u8,
    stride: i32,
    txfm_param: *const TxfmParam,
) {
    if !((*txfm_param).lossless != 0) {
        let tx_type: u8 = (*txfm_param).tx_type;
        av1_lowbd_inv_txfm2d_add_ssse3(
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
