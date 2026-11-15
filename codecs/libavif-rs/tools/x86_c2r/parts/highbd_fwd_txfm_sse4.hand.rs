/*
 * Copyright (c) 2016, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

// Translated from upstream/aom/av1/encoder/x86/highbd_fwd_txfm_sse4.c
// (ADR 0056, ADR 0058). Upstream compiles this file with -msse4.1. Function
// names, helper structure, comments, constants, and intrinsic sequences follow
// the original; every function carries `#[target_feature(enable = "sse4.1")]`.
// CONFIG_REALTIME_ONLY is 0 in the translated configuration, so the
// `#if !CONFIG_REALTIME_ONLY` parts are included.
// The straight-line function bodies were transliterated with statement-
// rewriting scripts (not C2Rust) and then compared with the C function by
// function: identifiers, constants and operators in order (ADR 0058).
//
// Translation notes:
// - av1_fwht4x4_sse4_1 (the lossless Walsh-Hadamard transform) is not
//   translated: av1_fwht4x4 is outside the port's kernel set, as
//   av1_fwht4x4_neon is on Arm, so the function would be an unused export.
// - C arrays become zero-initialized stack arrays that are only accessed
//   through a raw pointer to their first element (`a[i]` -> `*a.add(i)`,
//   `&a[i]` -> `a.add(i)`, `a + i` -> `a.offset(i as isize)`), so the in-place
//   calls (`col_txfm(in, in, ...)`, `transpose_32bit_4x4(cur_in, cur_in)`) keep
//   the C aliasing. The C code never reads an element before writing it.
// - Run-time shift counts (`_mm_srai_epi32(x, bit)`, `_mm_slli_epi32(x, shift)`
//   with an `int` count) are translated with the count-register intrinsics
//   (`_mm_sra_epi32(x, _mm_cvtsi32_si128(bit))`, `_mm_sll_epi32`). GCC and
//   Clang emit exactly these instructions for a non-constant count, the count
//   is zero-extended by MOVD, and counts above 31 saturate identically, so the
//   result is bit-identical for every `int` count. Constant counts keep the
//   immediate intrinsic with a const generic, as do the shuffle immediates.
// - The int8_t entries of av1_fwd_txfm_shift_ls and av1_fwd_cos_bit_col/row
//   are promoted to `int` before any arithmetic (`-shift[1]`), as in C.
// - The C `for` loops whose bound and counter the body does not change become
//   Rust range loops with the same iteration sequence; the do-while loops of
//   convert_8x8_to_16x16 become `loop` with the test at the end.
// - The av1/common/av1_txfm.h, av1/common/enums.h, and
//   av1/common/common_data.h items this file uses are defined locally, as in
//   the Neon translations. The coefficient and shift tables come from the
//   translated C (av1_cospi_arr_data, av1_sinpi_arr_data,
//   av1_fwd_txfm_shift_ls, av1_fwd_cos_bit_col, av1_fwd_cos_bit_row).
// - The dispatch tables hold `fwd_transform_1d_sse4_1` values, an
//   `Option<unsafe fn ..>` alias, so they can keep the C NULL entries. The C
//   code calls the selected entry unconditionally; a NULL entry (a type the
//   size does not support) would crash there and panics here. `assert(0)` in
//   the switch defaults becomes `debug_assert!(false)`.
// - The memset in av1_fwd_txfm2d_64x16_sse4_1 becomes ptr::write_bytes over
//   the same int32_t range.
#![allow(clippy::all)]

use core::arch::x86_64::*;

use crate::av1__common__av1_txfm::{av1_cospi_arr_data, av1_sinpi_arr_data};
use crate::av1__encoder__av1_fwd_txfm2d::{av1_fwd_cos_bit_col, av1_fwd_cos_bit_row,
                                         av1_fwd_txfm_shift_ls};
use crate::simd::aom_dsp__x86__transpose_sse2::transpose_32bit_4x4;
use crate::simd::aom_dsp__x86__txfm_common_sse2::mm_reverse_epi16;
use crate::simd::av1__common__x86__av1_txfm_sse4::av1_round_shift_rect_array_32_sse4_1;
use crate::simd::av1__common__x86__highbd_txfm_utility_sse4::{fwd_transform_1d_sse4_1,
                                                               half_btf_sse4_1, transpose_16x16,
                                                               transpose_8nx8n, transpose_8x8};
use crate::simd::av1__encoder__x86__av1_txfm1d_sse4::{av1_fdct32_sse4_1, av1_fdct64_sse4_1,
                                                       av1_idtx32_sse4_1};

// From aom_dsp/txfm_common.h and av1/common/enums.h. The constants are `int`
// valued as in the C switch statements.
pub type TX_TYPE = u8;
const DCT_DCT: i32 = 0;
const ADST_DCT: i32 = 1;
const DCT_ADST: i32 = 2;
const ADST_ADST: i32 = 3;
const FLIPADST_DCT: i32 = 4;
const DCT_FLIPADST: i32 = 5;
const FLIPADST_FLIPADST: i32 = 6;
const ADST_FLIPADST: i32 = 7;
const FLIPADST_ADST: i32 = 8;
const IDTX: i32 = 9;
const V_DCT: i32 = 10;
const H_DCT: i32 = 11;
const V_ADST: i32 = 12;
const H_ADST: i32 = 13;
const V_FLIPADST: i32 = 14;
const H_FLIPADST: i32 = 15;
const TX_TYPES: usize = 16;

type TX_SIZE = u8;
const TX_4X4: i32 = 0;
const TX_8X8: i32 = 1;
const TX_16X16: i32 = 2;
const TX_4X8: i32 = 5;
const TX_8X4: i32 = 6;
const TX_8X16: i32 = 7;
const TX_16X8: i32 = 8;
const TX_16X32: i32 = 9;
const TX_32X16: i32 = 10;
const TX_32X64: i32 = 11;
const TX_64X32: i32 = 12;
const TX_4X16: i32 = 13;
const TX_16X4: i32 = 14;
const TX_8X32: i32 = 15;
const TX_32X8: i32 = 16;
const TX_16X64: i32 = 17;
const TX_64X16: i32 = 18;

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
unsafe fn sinpi_arr(n: i32) -> *const i32 {
    (&raw const av1_sinpi_arr_data[(n - cos_bit_min) as usize]).cast::<i32>()
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

#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn store_output_w4(out: *mut i32, in_: *const __m128i, stride: i32, out_size: i32) {
    for i in 0..out_size {
        _mm_store_si128(out.offset((i * stride) as isize) as *mut __m128i, *in_.offset(i as isize));
    }
}

// av1_fwht4x4_sse4_1 is not translated (see the translation notes).

#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn load_buffer_4x4(input: *const i16, in_: *mut __m128i, stride: i32, flipud: i32,
                          fliplr: i32, shift: i32) {
    if flipud == 0 {
        *in_.add(0) = _mm_loadl_epi64(input.offset((0 * stride) as isize) as *const __m128i);
        *in_.add(1) = _mm_loadl_epi64(input.offset((1 * stride) as isize) as *const __m128i);
        *in_.add(2) = _mm_loadl_epi64(input.offset((2 * stride) as isize) as *const __m128i);
        *in_.add(3) = _mm_loadl_epi64(input.offset((3 * stride) as isize) as *const __m128i);
    } else {
        *in_.add(0) = _mm_loadl_epi64(input.offset((3 * stride) as isize) as *const __m128i);
        *in_.add(1) = _mm_loadl_epi64(input.offset((2 * stride) as isize) as *const __m128i);
        *in_.add(2) = _mm_loadl_epi64(input.offset((1 * stride) as isize) as *const __m128i);
        *in_.add(3) = _mm_loadl_epi64(input.offset((0 * stride) as isize) as *const __m128i);
    }

    if fliplr != 0 {
        *in_.add(0) = _mm_shufflelo_epi16::<0x1b>(*in_.add(0));
        *in_.add(1) = _mm_shufflelo_epi16::<0x1b>(*in_.add(1));
        *in_.add(2) = _mm_shufflelo_epi16::<0x1b>(*in_.add(2));
        *in_.add(3) = _mm_shufflelo_epi16::<0x1b>(*in_.add(3));
    }

    *in_.add(0) = _mm_cvtepi16_epi32(*in_.add(0));
    *in_.add(1) = _mm_cvtepi16_epi32(*in_.add(1));
    *in_.add(2) = _mm_cvtepi16_epi32(*in_.add(2));
    *in_.add(3) = _mm_cvtepi16_epi32(*in_.add(3));

    *in_.add(0) = _mm_sll_epi32(*in_.add(0), _mm_cvtsi32_si128(shift));
    *in_.add(1) = _mm_sll_epi32(*in_.add(1), _mm_cvtsi32_si128(shift));
    *in_.add(2) = _mm_sll_epi32(*in_.add(2), _mm_cvtsi32_si128(shift));
    *in_.add(3) = _mm_sll_epi32(*in_.add(3), _mm_cvtsi32_si128(shift));
}

// We only use stage-2 bit;
// shift[0] is used in load_buffer_4x4()
// shift[1] is used in txfm_func_col()
// shift[2] is used in txfm_func_row()
#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn fdct4x4_sse4_1(in_: *mut __m128i, out: *mut __m128i, bit: i32, num_col: i32) {
    let cospi: *const i32 = cospi_arr(bit);
    let cospi32: __m128i = _mm_set1_epi32(*cospi.add(32));
    let cospi48: __m128i = _mm_set1_epi32(*cospi.add(48));
    let cospi16: __m128i = _mm_set1_epi32(*cospi.add(16));
    let rnding: __m128i = _mm_set1_epi32(1 << (bit - 1));
    let s0: __m128i;
    let s1: __m128i;
    let s2: __m128i;
    let s3: __m128i;
    let mut u0: __m128i;
    let mut u1: __m128i;
    let mut u2: __m128i;
    let mut u3: __m128i;
    let mut v0: __m128i;
    let mut v1: __m128i;
    let mut v2: __m128i;
    let mut v3: __m128i;

    let mut endidx: i32 = 3 * num_col;
    s0 = _mm_add_epi32(*in_.add(0), *in_.offset(endidx as isize));
    s3 = _mm_sub_epi32(*in_.add(0), *in_.offset(endidx as isize));
    endidx -= num_col;
    s1 = _mm_add_epi32(*in_.offset(num_col as isize), *in_.offset(endidx as isize));
    s2 = _mm_sub_epi32(*in_.offset(num_col as isize), *in_.offset(endidx as isize));

    // btf_32_sse4_1_type0(cospi32, cospi32, s[01], u[02], bit);
    u0 = _mm_mullo_epi32(s0, cospi32);
    u1 = _mm_mullo_epi32(s1, cospi32);
    u2 = _mm_add_epi32(u0, u1);
    v0 = _mm_sub_epi32(u0, u1);

    u3 = _mm_add_epi32(u2, rnding);
    v1 = _mm_add_epi32(v0, rnding);

    u0 = _mm_sra_epi32(u3, _mm_cvtsi32_si128(bit));
    u2 = _mm_sra_epi32(v1, _mm_cvtsi32_si128(bit));

    // btf_32_sse4_1_type1(cospi48, cospi16, s[23], u[13], bit);
    v0 = _mm_mullo_epi32(s2, cospi48);
    v1 = _mm_mullo_epi32(s3, cospi16);
    v2 = _mm_add_epi32(v0, v1);

    v3 = _mm_add_epi32(v2, rnding);
    u1 = _mm_sra_epi32(v3, _mm_cvtsi32_si128(bit));

    v0 = _mm_mullo_epi32(s2, cospi16);
    v1 = _mm_mullo_epi32(s3, cospi48);
    v2 = _mm_sub_epi32(v1, v0);

    v3 = _mm_add_epi32(v2, rnding);
    u3 = _mm_sra_epi32(v3, _mm_cvtsi32_si128(bit));

    // Note: shift[1] and shift[2] are zeros

    *out.add(0) = u0;
    *out.add(1) = u1;
    *out.add(2) = u2;
    *out.add(3) = u3;
}

#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn write_buffer_4x4(res: *mut __m128i, output: *mut i32) {
    _mm_store_si128(output.add(0 * 4) as *mut __m128i, *res.add(0));
    _mm_store_si128(output.add(1 * 4) as *mut __m128i, *res.add(1));
    _mm_store_si128(output.add(2 * 4) as *mut __m128i, *res.add(2));
    _mm_store_si128(output.add(3 * 4) as *mut __m128i, *res.add(3));
}

#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn fadst4x4_sse4_1(in_: *mut __m128i, out: *mut __m128i, bit: i32, num_col: i32) {
    let sinpi: *const i32 = sinpi_arr(bit);
    let rnding: __m128i = _mm_set1_epi32(1 << (bit - 1));
    let sinpi1: __m128i = _mm_set1_epi32(*sinpi.add(1));
    let sinpi2: __m128i = _mm_set1_epi32(*sinpi.add(2));
    let sinpi3: __m128i = _mm_set1_epi32(*sinpi.add(3));
    let sinpi4: __m128i = _mm_set1_epi32(*sinpi.add(4));
    let mut t: __m128i;
    let mut s0: __m128i;
    let mut s1: __m128i;
    let mut s2: __m128i;
    let mut s3: __m128i;
    let s4: __m128i;
    let s5: __m128i;
    let s6: __m128i;
    let s7: __m128i;
    let x0: __m128i;
    let x1: __m128i;
    let x2: __m128i;
    let x3: __m128i;
    let mut u0: __m128i;
    let mut u1: __m128i;
    let mut u2: __m128i;
    let mut u3: __m128i;

    let mut idx: i32 = 0 * num_col;
    s0 = _mm_mullo_epi32(*in_.offset(idx as isize), sinpi1);
    s1 = _mm_mullo_epi32(*in_.offset(idx as isize), sinpi4);
    t = _mm_add_epi32(*in_.offset(idx as isize), *in_.offset((idx + num_col) as isize));
    idx += num_col;
    s2 = _mm_mullo_epi32(*in_.offset(idx as isize), sinpi2);
    s3 = _mm_mullo_epi32(*in_.offset(idx as isize), sinpi1);
    idx += num_col;
    s4 = _mm_mullo_epi32(*in_.offset(idx as isize), sinpi3);
    idx += num_col;
    s5 = _mm_mullo_epi32(*in_.offset(idx as isize), sinpi4);
    s6 = _mm_mullo_epi32(*in_.offset(idx as isize), sinpi2);
    s7 = _mm_sub_epi32(t, *in_.offset(idx as isize));

    t = _mm_add_epi32(s0, s2);
    x0 = _mm_add_epi32(t, s5);
    x1 = _mm_mullo_epi32(s7, sinpi3);
    t = _mm_sub_epi32(s1, s3);
    x2 = _mm_add_epi32(t, s6);
    x3 = s4;

    s0 = _mm_add_epi32(x0, x3);
    s1 = x1;
    s2 = _mm_sub_epi32(x2, x3);
    t = _mm_sub_epi32(x2, x0);
    s3 = _mm_add_epi32(t, x3);

    u0 = _mm_add_epi32(s0, rnding);
    u0 = _mm_sra_epi32(u0, _mm_cvtsi32_si128(bit));

    u1 = _mm_add_epi32(s1, rnding);
    u1 = _mm_sra_epi32(u1, _mm_cvtsi32_si128(bit));

    u2 = _mm_add_epi32(s2, rnding);
    u2 = _mm_sra_epi32(u2, _mm_cvtsi32_si128(bit));

    u3 = _mm_add_epi32(s3, rnding);
    u3 = _mm_sra_epi32(u3, _mm_cvtsi32_si128(bit));

    *out.add(0) = u0;
    *out.add(1) = u1;
    *out.add(2) = u2;
    *out.add(3) = u3;
}

#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn idtx4x4_sse4_1(in_: *mut __m128i, out: *mut __m128i, bit: i32, col_num: i32) {
    let _ = bit;
    let fact: __m128i = _mm_set1_epi32(NewSqrt2);
    let offset: __m128i = _mm_set1_epi32(1 << (NewSqrt2Bits - 1));
    let mut a_low: __m128i;

    for i in 0..4 {
        a_low = _mm_mullo_epi32(*in_.offset((i * col_num) as isize), fact);
        a_low = _mm_add_epi32(a_low, offset);
        *out.offset(i as isize) = _mm_srai_epi32::<NewSqrt2Bits>(a_low);
    }
}

#[no_mangle]
#[target_feature(enable = "sse4.1")]
pub unsafe extern "C-unwind" fn av1_fwd_txfm2d_4x4_sse4_1(input: *const i16, coeff: *mut i32,
                                                          input_stride: i32, tx_type: TX_TYPE,
                                                          bd: i32) {
    let mut in__: [__m128i; 4] = core::mem::zeroed();
    let in_: *mut __m128i = in__.as_mut_ptr();
    let shift: *const i8 = av1_fwd_txfm_shift_ls[TX_4X4 as usize];
    let txw_idx: i32 = get_txw_idx(TX_4X4 as TX_SIZE);
    let txh_idx: i32 = get_txh_idx(TX_4X4 as TX_SIZE);

    match tx_type as i32 {
        DCT_DCT => {
            load_buffer_4x4(input, in_, input_stride, 0, 0, *shift.add(0) as i32);
            fdct4x4_sse4_1(in_, in_, av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32,
                           1);
            transpose_32bit_4x4(in_, in_);
            fdct4x4_sse4_1(in_, in_, av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32,
                           1);
            write_buffer_4x4(in_, coeff);
        }
        ADST_DCT => {
            load_buffer_4x4(input, in_, input_stride, 0, 0, *shift.add(0) as i32);
            fadst4x4_sse4_1(in_, in_,
                            av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32, 1);
            transpose_32bit_4x4(in_, in_);
            fdct4x4_sse4_1(in_, in_, av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32,
                           1);
            write_buffer_4x4(in_, coeff);
        }
        DCT_ADST => {
            load_buffer_4x4(input, in_, input_stride, 0, 0, *shift.add(0) as i32);
            fdct4x4_sse4_1(in_, in_, av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32,
                           1);
            transpose_32bit_4x4(in_, in_);
            fadst4x4_sse4_1(in_, in_,
                            av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32, 1);
            write_buffer_4x4(in_, coeff);
        }
        ADST_ADST => {
            load_buffer_4x4(input, in_, input_stride, 0, 0, *shift.add(0) as i32);
            fadst4x4_sse4_1(in_, in_,
                            av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32, 1);
            transpose_32bit_4x4(in_, in_);
            fadst4x4_sse4_1(in_, in_,
                            av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32, 1);
            write_buffer_4x4(in_, coeff);
        }
        FLIPADST_DCT => {
            load_buffer_4x4(input, in_, input_stride, 1, 0, *shift.add(0) as i32);
            fadst4x4_sse4_1(in_, in_,
                            av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32, 1);
            transpose_32bit_4x4(in_, in_);
            fdct4x4_sse4_1(in_, in_, av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32,
                           1);
            write_buffer_4x4(in_, coeff);
        }
        DCT_FLIPADST => {
            load_buffer_4x4(input, in_, input_stride, 0, 1, *shift.add(0) as i32);
            fdct4x4_sse4_1(in_, in_, av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32,
                           1);
            transpose_32bit_4x4(in_, in_);
            fadst4x4_sse4_1(in_, in_,
                            av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32, 1);
            write_buffer_4x4(in_, coeff);
        }
        FLIPADST_FLIPADST => {
            load_buffer_4x4(input, in_, input_stride, 1, 1, *shift.add(0) as i32);
            fadst4x4_sse4_1(in_, in_,
                            av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32, 1);
            transpose_32bit_4x4(in_, in_);
            fadst4x4_sse4_1(in_, in_,
                            av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32, 1);
            write_buffer_4x4(in_, coeff);
        }
        ADST_FLIPADST => {
            load_buffer_4x4(input, in_, input_stride, 0, 1, *shift.add(0) as i32);
            fadst4x4_sse4_1(in_, in_,
                            av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32, 1);
            transpose_32bit_4x4(in_, in_);
            fadst4x4_sse4_1(in_, in_,
                            av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32, 1);
            write_buffer_4x4(in_, coeff);
        }
        FLIPADST_ADST => {
            load_buffer_4x4(input, in_, input_stride, 1, 0, *shift.add(0) as i32);
            fadst4x4_sse4_1(in_, in_,
                            av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32, 1);
            transpose_32bit_4x4(in_, in_);
            fadst4x4_sse4_1(in_, in_,
                            av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32, 1);
            write_buffer_4x4(in_, coeff);
        }
        IDTX => {
            load_buffer_4x4(input, in_, input_stride, 0, 0, *shift.add(0) as i32);
            idtx4x4_sse4_1(in_, in_, av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32,
                           1);
            transpose_32bit_4x4(in_, in_);
            idtx4x4_sse4_1(in_, in_, av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32,
                           1);
            write_buffer_4x4(in_, coeff);
        }
        V_DCT => {
            load_buffer_4x4(input, in_, input_stride, 0, 0, *shift.add(0) as i32);
            fdct4x4_sse4_1(in_, in_, av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32,
                           1);
            transpose_32bit_4x4(in_, in_);
            idtx4x4_sse4_1(in_, in_, av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32,
                           1);
            write_buffer_4x4(in_, coeff);
        }
        H_DCT => {
            load_buffer_4x4(input, in_, input_stride, 0, 0, *shift.add(0) as i32);
            idtx4x4_sse4_1(in_, in_, av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32,
                           1);
            transpose_32bit_4x4(in_, in_);
            fdct4x4_sse4_1(in_, in_, av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32,
                           1);
            write_buffer_4x4(in_, coeff);
        }
        V_ADST => {
            load_buffer_4x4(input, in_, input_stride, 0, 0, *shift.add(0) as i32);
            fadst4x4_sse4_1(in_, in_,
                            av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32, 1);
            transpose_32bit_4x4(in_, in_);
            idtx4x4_sse4_1(in_, in_, av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32,
                           1);
            write_buffer_4x4(in_, coeff);
        }
        H_ADST => {
            load_buffer_4x4(input, in_, input_stride, 0, 0, *shift.add(0) as i32);
            idtx4x4_sse4_1(in_, in_, av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32,
                           1);
            transpose_32bit_4x4(in_, in_);
            fadst4x4_sse4_1(in_, in_,
                            av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32, 1);
            write_buffer_4x4(in_, coeff);
        }
        V_FLIPADST => {
            load_buffer_4x4(input, in_, input_stride, 1, 0, *shift.add(0) as i32);
            fadst4x4_sse4_1(in_, in_,
                            av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32, 1);
            transpose_32bit_4x4(in_, in_);
            idtx4x4_sse4_1(in_, in_, av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32,
                           1);
            write_buffer_4x4(in_, coeff);
        }
        H_FLIPADST => {
            load_buffer_4x4(input, in_, input_stride, 0, 1, *shift.add(0) as i32);
            idtx4x4_sse4_1(in_, in_, av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32,
                           1);
            transpose_32bit_4x4(in_, in_);
            fadst4x4_sse4_1(in_, in_,
                            av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32, 1);
            write_buffer_4x4(in_, coeff);
        }
        _ => {
            debug_assert!(false);
        }
    }
    let _ = bd;
}

#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn load_buffer_8x8(input: *const i16, in_: *mut __m128i, stride: i32, flipud: i32,
                          fliplr: i32, shift: i32) {
    let mut u: __m128i;
    if flipud == 0 {
        *in_.add(0) = _mm_load_si128(input.offset((0 * stride) as isize) as *const __m128i);
        *in_.add(1) = _mm_load_si128(input.offset((1 * stride) as isize) as *const __m128i);
        *in_.add(2) = _mm_load_si128(input.offset((2 * stride) as isize) as *const __m128i);
        *in_.add(3) = _mm_load_si128(input.offset((3 * stride) as isize) as *const __m128i);
        *in_.add(4) = _mm_load_si128(input.offset((4 * stride) as isize) as *const __m128i);
        *in_.add(5) = _mm_load_si128(input.offset((5 * stride) as isize) as *const __m128i);
        *in_.add(6) = _mm_load_si128(input.offset((6 * stride) as isize) as *const __m128i);
        *in_.add(7) = _mm_load_si128(input.offset((7 * stride) as isize) as *const __m128i);
    } else {
        *in_.add(0) = _mm_load_si128(input.offset((7 * stride) as isize) as *const __m128i);
        *in_.add(1) = _mm_load_si128(input.offset((6 * stride) as isize) as *const __m128i);
        *in_.add(2) = _mm_load_si128(input.offset((5 * stride) as isize) as *const __m128i);
        *in_.add(3) = _mm_load_si128(input.offset((4 * stride) as isize) as *const __m128i);
        *in_.add(4) = _mm_load_si128(input.offset((3 * stride) as isize) as *const __m128i);
        *in_.add(5) = _mm_load_si128(input.offset((2 * stride) as isize) as *const __m128i);
        *in_.add(6) = _mm_load_si128(input.offset((1 * stride) as isize) as *const __m128i);
        *in_.add(7) = _mm_load_si128(input.offset((0 * stride) as isize) as *const __m128i);
    }

    if fliplr != 0 {
        *in_.add(0) = mm_reverse_epi16(*in_.add(0));
        *in_.add(1) = mm_reverse_epi16(*in_.add(1));
        *in_.add(2) = mm_reverse_epi16(*in_.add(2));
        *in_.add(3) = mm_reverse_epi16(*in_.add(3));
        *in_.add(4) = mm_reverse_epi16(*in_.add(4));
        *in_.add(5) = mm_reverse_epi16(*in_.add(5));
        *in_.add(6) = mm_reverse_epi16(*in_.add(6));
        *in_.add(7) = mm_reverse_epi16(*in_.add(7));
    }

    u = _mm_unpackhi_epi64(*in_.add(4), *in_.add(4));
    *in_.add(8) = _mm_cvtepi16_epi32(*in_.add(4));
    *in_.add(9) = _mm_cvtepi16_epi32(u);

    u = _mm_unpackhi_epi64(*in_.add(5), *in_.add(5));
    *in_.add(10) = _mm_cvtepi16_epi32(*in_.add(5));
    *in_.add(11) = _mm_cvtepi16_epi32(u);

    u = _mm_unpackhi_epi64(*in_.add(6), *in_.add(6));
    *in_.add(12) = _mm_cvtepi16_epi32(*in_.add(6));
    *in_.add(13) = _mm_cvtepi16_epi32(u);

    u = _mm_unpackhi_epi64(*in_.add(7), *in_.add(7));
    *in_.add(14) = _mm_cvtepi16_epi32(*in_.add(7));
    *in_.add(15) = _mm_cvtepi16_epi32(u);

    u = _mm_unpackhi_epi64(*in_.add(3), *in_.add(3));
    *in_.add(6) = _mm_cvtepi16_epi32(*in_.add(3));
    *in_.add(7) = _mm_cvtepi16_epi32(u);

    u = _mm_unpackhi_epi64(*in_.add(2), *in_.add(2));
    *in_.add(4) = _mm_cvtepi16_epi32(*in_.add(2));
    *in_.add(5) = _mm_cvtepi16_epi32(u);

    u = _mm_unpackhi_epi64(*in_.add(1), *in_.add(1));
    *in_.add(2) = _mm_cvtepi16_epi32(*in_.add(1));
    *in_.add(3) = _mm_cvtepi16_epi32(u);

    u = _mm_unpackhi_epi64(*in_.add(0), *in_.add(0));
    *in_.add(0) = _mm_cvtepi16_epi32(*in_.add(0));
    *in_.add(1) = _mm_cvtepi16_epi32(u);

    *in_.add(0) = _mm_sll_epi32(*in_.add(0), _mm_cvtsi32_si128(shift));
    *in_.add(1) = _mm_sll_epi32(*in_.add(1), _mm_cvtsi32_si128(shift));
    *in_.add(2) = _mm_sll_epi32(*in_.add(2), _mm_cvtsi32_si128(shift));
    *in_.add(3) = _mm_sll_epi32(*in_.add(3), _mm_cvtsi32_si128(shift));
    *in_.add(4) = _mm_sll_epi32(*in_.add(4), _mm_cvtsi32_si128(shift));
    *in_.add(5) = _mm_sll_epi32(*in_.add(5), _mm_cvtsi32_si128(shift));
    *in_.add(6) = _mm_sll_epi32(*in_.add(6), _mm_cvtsi32_si128(shift));
    *in_.add(7) = _mm_sll_epi32(*in_.add(7), _mm_cvtsi32_si128(shift));

    *in_.add(8) = _mm_sll_epi32(*in_.add(8), _mm_cvtsi32_si128(shift));
    *in_.add(9) = _mm_sll_epi32(*in_.add(9), _mm_cvtsi32_si128(shift));
    *in_.add(10) = _mm_sll_epi32(*in_.add(10), _mm_cvtsi32_si128(shift));
    *in_.add(11) = _mm_sll_epi32(*in_.add(11), _mm_cvtsi32_si128(shift));
    *in_.add(12) = _mm_sll_epi32(*in_.add(12), _mm_cvtsi32_si128(shift));
    *in_.add(13) = _mm_sll_epi32(*in_.add(13), _mm_cvtsi32_si128(shift));
    *in_.add(14) = _mm_sll_epi32(*in_.add(14), _mm_cvtsi32_si128(shift));
    *in_.add(15) = _mm_sll_epi32(*in_.add(15), _mm_cvtsi32_si128(shift));
}

#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn col_txfm_8x8_rounding(in_: *mut __m128i, shift: i32) {
    let rounding: __m128i = _mm_set1_epi32(1 << (shift - 1));

    *in_.add(0) = _mm_add_epi32(*in_.add(0), rounding);
    *in_.add(1) = _mm_add_epi32(*in_.add(1), rounding);
    *in_.add(2) = _mm_add_epi32(*in_.add(2), rounding);
    *in_.add(3) = _mm_add_epi32(*in_.add(3), rounding);
    *in_.add(4) = _mm_add_epi32(*in_.add(4), rounding);
    *in_.add(5) = _mm_add_epi32(*in_.add(5), rounding);
    *in_.add(6) = _mm_add_epi32(*in_.add(6), rounding);
    *in_.add(7) = _mm_add_epi32(*in_.add(7), rounding);
    *in_.add(8) = _mm_add_epi32(*in_.add(8), rounding);
    *in_.add(9) = _mm_add_epi32(*in_.add(9), rounding);
    *in_.add(10) = _mm_add_epi32(*in_.add(10), rounding);
    *in_.add(11) = _mm_add_epi32(*in_.add(11), rounding);
    *in_.add(12) = _mm_add_epi32(*in_.add(12), rounding);
    *in_.add(13) = _mm_add_epi32(*in_.add(13), rounding);
    *in_.add(14) = _mm_add_epi32(*in_.add(14), rounding);
    *in_.add(15) = _mm_add_epi32(*in_.add(15), rounding);

    *in_.add(0) = _mm_sra_epi32(*in_.add(0), _mm_cvtsi32_si128(shift));
    *in_.add(1) = _mm_sra_epi32(*in_.add(1), _mm_cvtsi32_si128(shift));
    *in_.add(2) = _mm_sra_epi32(*in_.add(2), _mm_cvtsi32_si128(shift));
    *in_.add(3) = _mm_sra_epi32(*in_.add(3), _mm_cvtsi32_si128(shift));
    *in_.add(4) = _mm_sra_epi32(*in_.add(4), _mm_cvtsi32_si128(shift));
    *in_.add(5) = _mm_sra_epi32(*in_.add(5), _mm_cvtsi32_si128(shift));
    *in_.add(6) = _mm_sra_epi32(*in_.add(6), _mm_cvtsi32_si128(shift));
    *in_.add(7) = _mm_sra_epi32(*in_.add(7), _mm_cvtsi32_si128(shift));
    *in_.add(8) = _mm_sra_epi32(*in_.add(8), _mm_cvtsi32_si128(shift));
    *in_.add(9) = _mm_sra_epi32(*in_.add(9), _mm_cvtsi32_si128(shift));
    *in_.add(10) = _mm_sra_epi32(*in_.add(10), _mm_cvtsi32_si128(shift));
    *in_.add(11) = _mm_sra_epi32(*in_.add(11), _mm_cvtsi32_si128(shift));
    *in_.add(12) = _mm_sra_epi32(*in_.add(12), _mm_cvtsi32_si128(shift));
    *in_.add(13) = _mm_sra_epi32(*in_.add(13), _mm_cvtsi32_si128(shift));
    *in_.add(14) = _mm_sra_epi32(*in_.add(14), _mm_cvtsi32_si128(shift));
    *in_.add(15) = _mm_sra_epi32(*in_.add(15), _mm_cvtsi32_si128(shift));
}

#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn col_txfm_4x8_rounding(in_: *mut __m128i, shift: i32) {
    let rounding: __m128i = _mm_set1_epi32(1 << (shift - 1));

    *in_.add(0) = _mm_add_epi32(*in_.add(0), rounding);
    *in_.add(1) = _mm_add_epi32(*in_.add(1), rounding);
    *in_.add(2) = _mm_add_epi32(*in_.add(2), rounding);
    *in_.add(3) = _mm_add_epi32(*in_.add(3), rounding);
    *in_.add(4) = _mm_add_epi32(*in_.add(4), rounding);
    *in_.add(5) = _mm_add_epi32(*in_.add(5), rounding);
    *in_.add(6) = _mm_add_epi32(*in_.add(6), rounding);
    *in_.add(7) = _mm_add_epi32(*in_.add(7), rounding);

    *in_.add(0) = _mm_sra_epi32(*in_.add(0), _mm_cvtsi32_si128(shift));
    *in_.add(1) = _mm_sra_epi32(*in_.add(1), _mm_cvtsi32_si128(shift));
    *in_.add(2) = _mm_sra_epi32(*in_.add(2), _mm_cvtsi32_si128(shift));
    *in_.add(3) = _mm_sra_epi32(*in_.add(3), _mm_cvtsi32_si128(shift));
    *in_.add(4) = _mm_sra_epi32(*in_.add(4), _mm_cvtsi32_si128(shift));
    *in_.add(5) = _mm_sra_epi32(*in_.add(5), _mm_cvtsi32_si128(shift));
    *in_.add(6) = _mm_sra_epi32(*in_.add(6), _mm_cvtsi32_si128(shift));
    *in_.add(7) = _mm_sra_epi32(*in_.add(7), _mm_cvtsi32_si128(shift));
}

#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn write_buffer_8x8(res: *const __m128i, output: *mut i32) {
    _mm_store_si128(output.add(0 * 4) as *mut __m128i, *res.add(0));
    _mm_store_si128(output.add(1 * 4) as *mut __m128i, *res.add(1));
    _mm_store_si128(output.add(2 * 4) as *mut __m128i, *res.add(2));
    _mm_store_si128(output.add(3 * 4) as *mut __m128i, *res.add(3));

    _mm_store_si128(output.add(4 * 4) as *mut __m128i, *res.add(4));
    _mm_store_si128(output.add(5 * 4) as *mut __m128i, *res.add(5));
    _mm_store_si128(output.add(6 * 4) as *mut __m128i, *res.add(6));
    _mm_store_si128(output.add(7 * 4) as *mut __m128i, *res.add(7));

    _mm_store_si128(output.add(8 * 4) as *mut __m128i, *res.add(8));
    _mm_store_si128(output.add(9 * 4) as *mut __m128i, *res.add(9));
    _mm_store_si128(output.add(10 * 4) as *mut __m128i, *res.add(10));
    _mm_store_si128(output.add(11 * 4) as *mut __m128i, *res.add(11));

    _mm_store_si128(output.add(12 * 4) as *mut __m128i, *res.add(12));
    _mm_store_si128(output.add(13 * 4) as *mut __m128i, *res.add(13));
    _mm_store_si128(output.add(14 * 4) as *mut __m128i, *res.add(14));
    _mm_store_si128(output.add(15 * 4) as *mut __m128i, *res.add(15));
}

#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn write_buffer_16x8(res: *const __m128i, output: *mut i32, stride: i32) {
    _mm_storeu_si128(output as *mut __m128i, *res.add(0));
    _mm_storeu_si128(output.add(4) as *mut __m128i, *res.add(1));
    _mm_storeu_si128(output.offset(stride as isize) as *mut __m128i, *res.add(2));
    _mm_storeu_si128(output.offset(stride as isize).add(4) as *mut __m128i, *res.add(3));

    _mm_storeu_si128(output.offset((stride * 2) as isize) as *mut __m128i, *res.add(4));
    _mm_storeu_si128(output.offset((stride * 2) as isize).add(4) as *mut __m128i, *res.add(5));
    _mm_storeu_si128(output.offset((stride * 3) as isize) as *mut __m128i, *res.add(6));
    _mm_storeu_si128(output.offset((stride * 3) as isize).add(4) as *mut __m128i, *res.add(7));

    _mm_storeu_si128(output.offset((stride * 4) as isize) as *mut __m128i, *res.add(8));
    _mm_storeu_si128(output.offset((stride * 4) as isize).add(4) as *mut __m128i, *res.add(9));
    _mm_storeu_si128(output.offset((stride * 5) as isize) as *mut __m128i, *res.add(10));
    _mm_storeu_si128(output.offset((stride * 5) as isize).add(4) as *mut __m128i, *res.add(11));

    _mm_storeu_si128(output.offset((stride * 6) as isize) as *mut __m128i, *res.add(12));
    _mm_storeu_si128(output.offset((stride * 6) as isize).add(4) as *mut __m128i, *res.add(13));
    _mm_storeu_si128(output.offset((stride * 7) as isize) as *mut __m128i, *res.add(14));
    _mm_storeu_si128(output.offset((stride * 7) as isize).add(4) as *mut __m128i, *res.add(15));
}

#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn fdct4x8_sse4_1(in_: *mut __m128i, out: *mut __m128i, bit: i32, col_num: i32) {
    let cospi: *const i32 = cospi_arr(bit);
    let cospi32: __m128i = _mm_set1_epi32(*cospi.add(32));
    let cospim32: __m128i = _mm_set1_epi32(-*cospi.add(32));
    let cospi48: __m128i = _mm_set1_epi32(*cospi.add(48));
    let cospi16: __m128i = _mm_set1_epi32(*cospi.add(16));
    let cospi56: __m128i = _mm_set1_epi32(*cospi.add(56));
    let cospi8: __m128i = _mm_set1_epi32(*cospi.add(8));
    let cospi24: __m128i = _mm_set1_epi32(*cospi.add(24));
    let cospi40: __m128i = _mm_set1_epi32(*cospi.add(40));
    let rnding: __m128i = _mm_set1_epi32(1 << (bit - 1));
    let mut u_: [__m128i; 8] = core::mem::zeroed();
    let u: *mut __m128i = u_.as_mut_ptr();
    let mut v_: [__m128i; 8] = core::mem::zeroed();
    let v: *mut __m128i = v_.as_mut_ptr();

    let mut startidx: i32 = 0 * col_num;
    let mut endidx: i32 = 7 * col_num;
    // Even 8 points 0, 2, ..., 14
    // stage 0
    // stage 1
    *u.add(0) = _mm_add_epi32(*in_.offset(startidx as isize), *in_.offset(endidx as isize));
    *v.add(7) = _mm_sub_epi32(*in_.offset(startidx as isize),
                              *in_.offset(endidx as isize));  // *v.add(7)
    startidx += col_num;
    endidx -= col_num;
    *u.add(1) = _mm_add_epi32(*in_.offset(startidx as isize), *in_.offset(endidx as isize));
    *u.add(6) = _mm_sub_epi32(*in_.offset(startidx as isize), *in_.offset(endidx as isize));
    startidx += col_num;
    endidx -= col_num;
    *u.add(2) = _mm_add_epi32(*in_.offset(startidx as isize), *in_.offset(endidx as isize));
    *u.add(5) = _mm_sub_epi32(*in_.offset(startidx as isize), *in_.offset(endidx as isize));
    startidx += col_num;
    endidx -= col_num;
    *u.add(3) = _mm_add_epi32(*in_.offset(startidx as isize), *in_.offset(endidx as isize));
    *v.add(4) = _mm_sub_epi32(*in_.offset(startidx as isize),
                              *in_.offset(endidx as isize));  // *v.add(4)

    // stage 2
    *v.add(0) = _mm_add_epi32(*u.add(0), *u.add(3));
    *v.add(3) = _mm_sub_epi32(*u.add(0), *u.add(3));
    *v.add(1) = _mm_add_epi32(*u.add(1), *u.add(2));
    *v.add(2) = _mm_sub_epi32(*u.add(1), *u.add(2));

    *v.add(5) = _mm_mullo_epi32(*u.add(5), cospim32);
    *v.add(6) = _mm_mullo_epi32(*u.add(6), cospi32);
    *v.add(5) = _mm_add_epi32(*v.add(5), *v.add(6));
    *v.add(5) = _mm_add_epi32(*v.add(5), rnding);
    *v.add(5) = _mm_sra_epi32(*v.add(5), _mm_cvtsi32_si128(bit));

    *u.add(0) = _mm_mullo_epi32(*u.add(5), cospi32);
    *v.add(6) = _mm_mullo_epi32(*u.add(6), cospim32);
    *v.add(6) = _mm_sub_epi32(*u.add(0), *v.add(6));
    *v.add(6) = _mm_add_epi32(*v.add(6), rnding);
    *v.add(6) = _mm_sra_epi32(*v.add(6), _mm_cvtsi32_si128(bit));

    // stage 3
    // type 0
    *v.add(0) = _mm_mullo_epi32(*v.add(0), cospi32);
    *v.add(1) = _mm_mullo_epi32(*v.add(1), cospi32);
    *u.add(0) = _mm_add_epi32(*v.add(0), *v.add(1));
    *u.add(0) = _mm_add_epi32(*u.add(0), rnding);
    *u.add(0) = _mm_sra_epi32(*u.add(0), _mm_cvtsi32_si128(bit));

    *u.add(1) = _mm_sub_epi32(*v.add(0), *v.add(1));
    *u.add(1) = _mm_add_epi32(*u.add(1), rnding);
    *u.add(1) = _mm_sra_epi32(*u.add(1), _mm_cvtsi32_si128(bit));

    // type 1
    *v.add(0) = _mm_mullo_epi32(*v.add(2), cospi48);
    *v.add(1) = _mm_mullo_epi32(*v.add(3), cospi16);
    *u.add(2) = _mm_add_epi32(*v.add(0), *v.add(1));
    *u.add(2) = _mm_add_epi32(*u.add(2), rnding);
    *u.add(2) = _mm_sra_epi32(*u.add(2), _mm_cvtsi32_si128(bit));

    *v.add(0) = _mm_mullo_epi32(*v.add(2), cospi16);
    *v.add(1) = _mm_mullo_epi32(*v.add(3), cospi48);
    *u.add(3) = _mm_sub_epi32(*v.add(1), *v.add(0));
    *u.add(3) = _mm_add_epi32(*u.add(3), rnding);
    *u.add(3) = _mm_sra_epi32(*u.add(3), _mm_cvtsi32_si128(bit));

    *u.add(4) = _mm_add_epi32(*v.add(4), *v.add(5));
    *u.add(5) = _mm_sub_epi32(*v.add(4), *v.add(5));
    *u.add(6) = _mm_sub_epi32(*v.add(7), *v.add(6));
    *u.add(7) = _mm_add_epi32(*v.add(7), *v.add(6));

    // stage 4
    // stage 5
    *v.add(0) = _mm_mullo_epi32(*u.add(4), cospi56);
    *v.add(1) = _mm_mullo_epi32(*u.add(7), cospi8);
    *v.add(0) = _mm_add_epi32(*v.add(0), *v.add(1));
    *v.add(0) = _mm_add_epi32(*v.add(0), rnding);
    *out.offset((1 * col_num) as isize) = _mm_sra_epi32(*v.add(0),
                                                        _mm_cvtsi32_si128(bit));  // buf0[4]

    *v.add(0) = _mm_mullo_epi32(*u.add(4), cospi8);
    *v.add(1) = _mm_mullo_epi32(*u.add(7), cospi56);
    *v.add(0) = _mm_sub_epi32(*v.add(1), *v.add(0));
    *v.add(0) = _mm_add_epi32(*v.add(0), rnding);
    *out.offset((7 * col_num) as isize) = _mm_sra_epi32(*v.add(0),
                                                        _mm_cvtsi32_si128(bit));  // buf0[7]

    *v.add(0) = _mm_mullo_epi32(*u.add(5), cospi24);
    *v.add(1) = _mm_mullo_epi32(*u.add(6), cospi40);
    *v.add(0) = _mm_add_epi32(*v.add(0), *v.add(1));
    *v.add(0) = _mm_add_epi32(*v.add(0), rnding);
    *out.offset((5 * col_num) as isize) = _mm_sra_epi32(*v.add(0),
                                                        _mm_cvtsi32_si128(bit));  // buf0[5]

    *v.add(0) = _mm_mullo_epi32(*u.add(5), cospi40);
    *v.add(1) = _mm_mullo_epi32(*u.add(6), cospi24);
    *v.add(0) = _mm_sub_epi32(*v.add(1), *v.add(0));
    *v.add(0) = _mm_add_epi32(*v.add(0), rnding);
    *out.offset((3 * col_num) as isize) = _mm_sra_epi32(*v.add(0),
                                                        _mm_cvtsi32_si128(bit));  // buf0[6]

    *out.offset((0 * col_num) as isize) = *u.add(0);  // buf0[0]
    *out.offset((4 * col_num) as isize) = *u.add(1);  // buf0[1]
    *out.offset((2 * col_num) as isize) = *u.add(2);  // buf0[2]
    *out.offset((6 * col_num) as isize) = *u.add(3);  // buf0[3]
}

#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn fdct8x8_sse4_1(in_: *mut __m128i, out: *mut __m128i, bit: i32, col_num: i32) {
    fdct4x8_sse4_1(in_, out, bit, col_num);
    fdct4x8_sse4_1(in_.add(1), out.add(1), bit, col_num);
}

#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn fadst8x8_sse4_1(in_: *mut __m128i, out: *mut __m128i, bit: i32, col_num: i32) {
    let cospi: *const i32 = cospi_arr(bit);
    let cospi32: __m128i = _mm_set1_epi32(*cospi.add(32));
    let cospi16: __m128i = _mm_set1_epi32(*cospi.add(16));
    let cospim16: __m128i = _mm_set1_epi32(-*cospi.add(16));
    let cospi48: __m128i = _mm_set1_epi32(*cospi.add(48));
    let cospim48: __m128i = _mm_set1_epi32(-*cospi.add(48));
    let cospi4: __m128i = _mm_set1_epi32(*cospi.add(4));
    let cospim4: __m128i = _mm_set1_epi32(-*cospi.add(4));
    let cospi60: __m128i = _mm_set1_epi32(*cospi.add(60));
    let cospi20: __m128i = _mm_set1_epi32(*cospi.add(20));
    let cospim20: __m128i = _mm_set1_epi32(-*cospi.add(20));
    let cospi44: __m128i = _mm_set1_epi32(*cospi.add(44));
    let cospi28: __m128i = _mm_set1_epi32(*cospi.add(28));
    let cospi36: __m128i = _mm_set1_epi32(*cospi.add(36));
    let cospim36: __m128i = _mm_set1_epi32(-*cospi.add(36));
    let cospi52: __m128i = _mm_set1_epi32(*cospi.add(52));
    let cospim52: __m128i = _mm_set1_epi32(-*cospi.add(52));
    let cospi12: __m128i = _mm_set1_epi32(*cospi.add(12));
    let rnding: __m128i = _mm_set1_epi32(1 << (bit - 1));
    let zero: __m128i = _mm_setzero_si128();
    let mut u0: __m128i;
    let mut u1: __m128i;
    let mut u2: __m128i;
    let mut u3: __m128i;
    let mut u4: __m128i;
    let mut u5: __m128i;
    let mut u6: __m128i;
    let mut u7: __m128i;
    let mut v0: __m128i;
    let mut v1: __m128i;
    let mut v2: __m128i;
    let mut v3: __m128i;
    let mut v4: __m128i;
    let mut v5: __m128i;
    let mut v6: __m128i;
    let mut v7: __m128i;
    let mut x: __m128i;
    let mut y: __m128i;

    // Note:
    //  Even column: 0, 2, ..., 14
    //  Odd column: 1, 3, ..., 15
    //  one even column plus one odd column constructs one row (8 coeffs)
    //  total we have 8 rows (8x8).
    for col in 0..col_num {
        // stage 0
        // stage 1
        u0 = *in_.offset((col_num * 0 + col) as isize);
        u1 = _mm_sub_epi32(zero, *in_.offset((col_num * 7 + col) as isize));
        u2 = _mm_sub_epi32(zero, *in_.offset((col_num * 3 + col) as isize));
        u3 = *in_.offset((col_num * 4 + col) as isize);
        u4 = _mm_sub_epi32(zero, *in_.offset((col_num * 1 + col) as isize));
        u5 = *in_.offset((col_num * 6 + col) as isize);
        u6 = *in_.offset((col_num * 2 + col) as isize);
        u7 = _mm_sub_epi32(zero, *in_.offset((col_num * 5 + col) as isize));

        // stage 2
        v0 = u0;
        v1 = u1;

        x = _mm_mullo_epi32(u2, cospi32);
        y = _mm_mullo_epi32(u3, cospi32);
        v2 = _mm_add_epi32(x, y);
        v2 = _mm_add_epi32(v2, rnding);
        v2 = _mm_sra_epi32(v2, _mm_cvtsi32_si128(bit));

        v3 = _mm_sub_epi32(x, y);
        v3 = _mm_add_epi32(v3, rnding);
        v3 = _mm_sra_epi32(v3, _mm_cvtsi32_si128(bit));

        v4 = u4;
        v5 = u5;

        x = _mm_mullo_epi32(u6, cospi32);
        y = _mm_mullo_epi32(u7, cospi32);
        v6 = _mm_add_epi32(x, y);
        v6 = _mm_add_epi32(v6, rnding);
        v6 = _mm_sra_epi32(v6, _mm_cvtsi32_si128(bit));

        v7 = _mm_sub_epi32(x, y);
        v7 = _mm_add_epi32(v7, rnding);
        v7 = _mm_sra_epi32(v7, _mm_cvtsi32_si128(bit));

        // stage 3
        u0 = _mm_add_epi32(v0, v2);
        u1 = _mm_add_epi32(v1, v3);
        u2 = _mm_sub_epi32(v0, v2);
        u3 = _mm_sub_epi32(v1, v3);
        u4 = _mm_add_epi32(v4, v6);
        u5 = _mm_add_epi32(v5, v7);
        u6 = _mm_sub_epi32(v4, v6);
        u7 = _mm_sub_epi32(v5, v7);

        // stage 4
        v0 = u0;
        v1 = u1;
        v2 = u2;
        v3 = u3;

        x = _mm_mullo_epi32(u4, cospi16);
        y = _mm_mullo_epi32(u5, cospi48);
        v4 = _mm_add_epi32(x, y);
        v4 = _mm_add_epi32(v4, rnding);
        v4 = _mm_sra_epi32(v4, _mm_cvtsi32_si128(bit));

        x = _mm_mullo_epi32(u4, cospi48);
        y = _mm_mullo_epi32(u5, cospim16);
        v5 = _mm_add_epi32(x, y);
        v5 = _mm_add_epi32(v5, rnding);
        v5 = _mm_sra_epi32(v5, _mm_cvtsi32_si128(bit));

        x = _mm_mullo_epi32(u6, cospim48);
        y = _mm_mullo_epi32(u7, cospi16);
        v6 = _mm_add_epi32(x, y);
        v6 = _mm_add_epi32(v6, rnding);
        v6 = _mm_sra_epi32(v6, _mm_cvtsi32_si128(bit));

        x = _mm_mullo_epi32(u6, cospi16);
        y = _mm_mullo_epi32(u7, cospi48);
        v7 = _mm_add_epi32(x, y);
        v7 = _mm_add_epi32(v7, rnding);
        v7 = _mm_sra_epi32(v7, _mm_cvtsi32_si128(bit));

        // stage 5
        u0 = _mm_add_epi32(v0, v4);
        u1 = _mm_add_epi32(v1, v5);
        u2 = _mm_add_epi32(v2, v6);
        u3 = _mm_add_epi32(v3, v7);
        u4 = _mm_sub_epi32(v0, v4);
        u5 = _mm_sub_epi32(v1, v5);
        u6 = _mm_sub_epi32(v2, v6);
        u7 = _mm_sub_epi32(v3, v7);

        // stage 6
        x = _mm_mullo_epi32(u0, cospi4);
        y = _mm_mullo_epi32(u1, cospi60);
        v0 = _mm_add_epi32(x, y);
        v0 = _mm_add_epi32(v0, rnding);
        v0 = _mm_sra_epi32(v0, _mm_cvtsi32_si128(bit));

        x = _mm_mullo_epi32(u0, cospi60);
        y = _mm_mullo_epi32(u1, cospim4);
        v1 = _mm_add_epi32(x, y);
        v1 = _mm_add_epi32(v1, rnding);
        v1 = _mm_sra_epi32(v1, _mm_cvtsi32_si128(bit));

        x = _mm_mullo_epi32(u2, cospi20);
        y = _mm_mullo_epi32(u3, cospi44);
        v2 = _mm_add_epi32(x, y);
        v2 = _mm_add_epi32(v2, rnding);
        v2 = _mm_sra_epi32(v2, _mm_cvtsi32_si128(bit));

        x = _mm_mullo_epi32(u2, cospi44);
        y = _mm_mullo_epi32(u3, cospim20);
        v3 = _mm_add_epi32(x, y);
        v3 = _mm_add_epi32(v3, rnding);
        v3 = _mm_sra_epi32(v3, _mm_cvtsi32_si128(bit));

        x = _mm_mullo_epi32(u4, cospi36);
        y = _mm_mullo_epi32(u5, cospi28);
        v4 = _mm_add_epi32(x, y);
        v4 = _mm_add_epi32(v4, rnding);
        v4 = _mm_sra_epi32(v4, _mm_cvtsi32_si128(bit));

        x = _mm_mullo_epi32(u4, cospi28);
        y = _mm_mullo_epi32(u5, cospim36);
        v5 = _mm_add_epi32(x, y);
        v5 = _mm_add_epi32(v5, rnding);
        v5 = _mm_sra_epi32(v5, _mm_cvtsi32_si128(bit));

        x = _mm_mullo_epi32(u6, cospi52);
        y = _mm_mullo_epi32(u7, cospi12);
        v6 = _mm_add_epi32(x, y);
        v6 = _mm_add_epi32(v6, rnding);
        v6 = _mm_sra_epi32(v6, _mm_cvtsi32_si128(bit));

        x = _mm_mullo_epi32(u6, cospi12);
        y = _mm_mullo_epi32(u7, cospim52);
        v7 = _mm_add_epi32(x, y);
        v7 = _mm_add_epi32(v7, rnding);
        v7 = _mm_sra_epi32(v7, _mm_cvtsi32_si128(bit));

        // stage 7
        *out.offset((col_num * 0 + col) as isize) = v1;
        *out.offset((col_num * 1 + col) as isize) = v6;
        *out.offset((col_num * 2 + col) as isize) = v3;
        *out.offset((col_num * 3 + col) as isize) = v4;
        *out.offset((col_num * 4 + col) as isize) = v5;
        *out.offset((col_num * 5 + col) as isize) = v2;
        *out.offset((col_num * 6 + col) as isize) = v7;
        *out.offset((col_num * 7 + col) as isize) = v0;
    }
}

#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn idtx8x8_sse4_1(in_: *mut __m128i, out: *mut __m128i, bit: i32, col_num: i32) {
    let _ = bit;

    for i in 0..col_num {
        *out.offset((0 + 8 * i) as isize) = _mm_add_epi32(*in_.offset((0 + 8 * i) as isize),
                                                          *in_.offset((0 + 8 * i) as isize));
        *out.offset((1 + 8 * i) as isize) = _mm_add_epi32(*in_.offset((1 + 8 * i) as isize),
                                                          *in_.offset((1 + 8 * i) as isize));
        *out.offset((2 + 8 * i) as isize) = _mm_add_epi32(*in_.offset((2 + 8 * i) as isize),
                                                          *in_.offset((2 + 8 * i) as isize));
        *out.offset((3 + 8 * i) as isize) = _mm_add_epi32(*in_.offset((3 + 8 * i) as isize),
                                                          *in_.offset((3 + 8 * i) as isize));
        *out.offset((4 + 8 * i) as isize) = _mm_add_epi32(*in_.offset((4 + 8 * i) as isize),
                                                          *in_.offset((4 + 8 * i) as isize));
        *out.offset((5 + 8 * i) as isize) = _mm_add_epi32(*in_.offset((5 + 8 * i) as isize),
                                                          *in_.offset((5 + 8 * i) as isize));
        *out.offset((6 + 8 * i) as isize) = _mm_add_epi32(*in_.offset((6 + 8 * i) as isize),
                                                          *in_.offset((6 + 8 * i) as isize));
        *out.offset((7 + 8 * i) as isize) = _mm_add_epi32(*in_.offset((7 + 8 * i) as isize),
                                                          *in_.offset((7 + 8 * i) as isize));
    }
}

#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn idtx32x8_sse4_1(in_: *mut __m128i, out: *mut __m128i, bit: i32, col_num: i32) {
    let _ = bit;
    let _ = col_num;
    for j in 0..2 {
        *out.offset((j + 8 * 0) as isize) = _mm_add_epi32(*in_.offset((j + 8 * 0) as isize),
                                                          *in_.offset((j + 8 * 0) as isize));
        *out.offset((j + 8 * 1) as isize) = _mm_add_epi32(*in_.offset((j + 8 * 1) as isize),
                                                          *in_.offset((j + 8 * 1) as isize));
        *out.offset((j + 8 * 2) as isize) = _mm_add_epi32(*in_.offset((j + 8 * 2) as isize),
                                                          *in_.offset((j + 8 * 2) as isize));
        *out.offset((j + 8 * 3) as isize) = _mm_add_epi32(*in_.offset((j + 8 * 3) as isize),
                                                          *in_.offset((j + 8 * 3) as isize));
        *out.offset((j + 8 * 4) as isize) = _mm_add_epi32(*in_.offset((j + 8 * 4) as isize),
                                                          *in_.offset((j + 8 * 4) as isize));
        *out.offset((j + 8 * 5) as isize) = _mm_add_epi32(*in_.offset((j + 8 * 5) as isize),
                                                          *in_.offset((j + 8 * 5) as isize));
        *out.offset((j + 8 * 6) as isize) = _mm_add_epi32(*in_.offset((j + 8 * 6) as isize),
                                                          *in_.offset((j + 8 * 6) as isize));
        *out.offset((j + 8 * 7) as isize) = _mm_add_epi32(*in_.offset((j + 8 * 7) as isize),
                                                          *in_.offset((j + 8 * 7) as isize));
    }
}

#[no_mangle]
#[target_feature(enable = "sse4.1")]
pub unsafe extern "C-unwind" fn av1_fwd_txfm2d_8x8_sse4_1(input: *const i16, coeff: *mut i32,
                                                          stride: i32, tx_type: TX_TYPE, bd: i32) {
    let mut in__: [__m128i; 16] = core::mem::zeroed();
    let in_: *mut __m128i = in__.as_mut_ptr();
    let mut out_: [__m128i; 16] = core::mem::zeroed();
    let out: *mut __m128i = out_.as_mut_ptr();
    let shift: *const i8 = av1_fwd_txfm_shift_ls[TX_8X8 as usize];
    let txw_idx: i32 = get_txw_idx(TX_8X8 as TX_SIZE);
    let txh_idx: i32 = get_txh_idx(TX_8X8 as TX_SIZE);

    match tx_type as i32 {
        DCT_DCT => {
            load_buffer_8x8(input, in_, stride, 0, 0, *shift.add(0) as i32);
            fdct8x8_sse4_1(in_, out, av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32,
                           2);
            col_txfm_8x8_rounding(out, -(*shift.add(1) as i32));
            transpose_8x8(out, in_);
            fdct8x8_sse4_1(in_, out, av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32,
                           2);
            write_buffer_8x8(out, coeff);
        }
        ADST_DCT => {
            load_buffer_8x8(input, in_, stride, 0, 0, *shift.add(0) as i32);
            fadst8x8_sse4_1(in_, out,
                            av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32, 2);
            col_txfm_8x8_rounding(out, -(*shift.add(1) as i32));
            transpose_8x8(out, in_);
            fdct8x8_sse4_1(in_, out, av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32,
                           2);
            write_buffer_8x8(out, coeff);
        }
        DCT_ADST => {
            load_buffer_8x8(input, in_, stride, 0, 0, *shift.add(0) as i32);
            fdct8x8_sse4_1(in_, out, av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32,
                           2);
            col_txfm_8x8_rounding(out, -(*shift.add(1) as i32));
            transpose_8x8(out, in_);
            fadst8x8_sse4_1(in_, out,
                            av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32, 2);
            write_buffer_8x8(out, coeff);
        }
        ADST_ADST => {
            load_buffer_8x8(input, in_, stride, 0, 0, *shift.add(0) as i32);
            fadst8x8_sse4_1(in_, out,
                            av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32, 2);
            col_txfm_8x8_rounding(out, -(*shift.add(1) as i32));
            transpose_8x8(out, in_);
            fadst8x8_sse4_1(in_, out,
                            av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32, 2);
            write_buffer_8x8(out, coeff);
        }
        FLIPADST_DCT => {
            load_buffer_8x8(input, in_, stride, 1, 0, *shift.add(0) as i32);
            fadst8x8_sse4_1(in_, out,
                            av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32, 2);
            col_txfm_8x8_rounding(out, -(*shift.add(1) as i32));
            transpose_8x8(out, in_);
            fdct8x8_sse4_1(in_, out, av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32,
                           2);
            write_buffer_8x8(out, coeff);
        }
        DCT_FLIPADST => {
            load_buffer_8x8(input, in_, stride, 0, 1, *shift.add(0) as i32);
            fdct8x8_sse4_1(in_, out, av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32,
                           2);
            col_txfm_8x8_rounding(out, -(*shift.add(1) as i32));
            transpose_8x8(out, in_);
            fadst8x8_sse4_1(in_, out,
                            av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32, 2);
            write_buffer_8x8(out, coeff);
        }
        FLIPADST_FLIPADST => {
            load_buffer_8x8(input, in_, stride, 1, 1, *shift.add(0) as i32);
            fadst8x8_sse4_1(in_, out,
                            av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32, 2);
            col_txfm_8x8_rounding(out, -(*shift.add(1) as i32));
            transpose_8x8(out, in_);
            fadst8x8_sse4_1(in_, out,
                            av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32, 2);
            write_buffer_8x8(out, coeff);
        }
        ADST_FLIPADST => {
            load_buffer_8x8(input, in_, stride, 0, 1, *shift.add(0) as i32);
            fadst8x8_sse4_1(in_, out,
                            av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32, 2);
            col_txfm_8x8_rounding(out, -(*shift.add(1) as i32));
            transpose_8x8(out, in_);
            fadst8x8_sse4_1(in_, out,
                            av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32, 2);
            write_buffer_8x8(out, coeff);
        }
        FLIPADST_ADST => {
            load_buffer_8x8(input, in_, stride, 1, 0, *shift.add(0) as i32);
            fadst8x8_sse4_1(in_, out,
                            av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32, 2);
            col_txfm_8x8_rounding(out, -(*shift.add(1) as i32));
            transpose_8x8(out, in_);
            fadst8x8_sse4_1(in_, out,
                            av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32, 2);
            write_buffer_8x8(out, coeff);
        }
        IDTX => {
            load_buffer_8x8(input, in_, stride, 0, 0, *shift.add(0) as i32);
            idtx8x8_sse4_1(in_, out, av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32,
                           2);
            col_txfm_8x8_rounding(out, -(*shift.add(1) as i32));
            transpose_8x8(out, in_);
            idtx8x8_sse4_1(in_, out, av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32,
                           2);
            write_buffer_8x8(out, coeff);
        }
        V_DCT => {
            load_buffer_8x8(input, in_, stride, 0, 0, *shift.add(0) as i32);
            fdct8x8_sse4_1(in_, out, av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32,
                           2);
            col_txfm_8x8_rounding(out, -(*shift.add(1) as i32));
            transpose_8x8(out, in_);
            idtx8x8_sse4_1(in_, out, av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32,
                           2);
            write_buffer_8x8(out, coeff);
        }
        H_DCT => {
            load_buffer_8x8(input, in_, stride, 0, 0, *shift.add(0) as i32);
            idtx8x8_sse4_1(in_, out, av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32,
                           2);
            col_txfm_8x8_rounding(out, -(*shift.add(1) as i32));
            transpose_8x8(out, in_);
            fdct8x8_sse4_1(in_, out, av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32,
                           2);
            write_buffer_8x8(out, coeff);
        }
        V_ADST => {
            load_buffer_8x8(input, in_, stride, 0, 0, *shift.add(0) as i32);
            fadst8x8_sse4_1(in_, out,
                            av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32, 2);
            col_txfm_8x8_rounding(out, -(*shift.add(1) as i32));
            transpose_8x8(out, in_);
            idtx8x8_sse4_1(in_, out, av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32,
                           2);
            write_buffer_8x8(out, coeff);
        }
        H_ADST => {
            load_buffer_8x8(input, in_, stride, 0, 0, *shift.add(0) as i32);
            idtx8x8_sse4_1(in_, out, av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32,
                           2);
            col_txfm_8x8_rounding(out, -(*shift.add(1) as i32));
            transpose_8x8(out, in_);
            fadst8x8_sse4_1(in_, out,
                            av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32, 2);
            write_buffer_8x8(out, coeff);
        }
        V_FLIPADST => {
            load_buffer_8x8(input, in_, stride, 1, 0, *shift.add(0) as i32);
            fadst8x8_sse4_1(in_, out,
                            av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32, 2);
            col_txfm_8x8_rounding(out, -(*shift.add(1) as i32));
            transpose_8x8(out, in_);
            idtx8x8_sse4_1(in_, out, av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32,
                           2);
            write_buffer_8x8(out, coeff);
        }
        H_FLIPADST => {
            load_buffer_8x8(input, in_, stride, 0, 1, *shift.add(0) as i32);
            idtx8x8_sse4_1(in_, out, av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32,
                           2);
            col_txfm_8x8_rounding(out, -(*shift.add(1) as i32));
            transpose_8x8(out, in_);
            fadst8x8_sse4_1(in_, out,
                            av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32, 2);
            write_buffer_8x8(out, coeff);
        }
        _ => {
            debug_assert!(false);
        }
    }
    let _ = bd;
}

// Hybrid Transform 16x16
#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn convert_8x8_to_16x16(in_: *const __m128i, out: *mut __m128i) {
    let mut row_index: i32 = 0;
    let mut dst_index: i32 = 0;
    let mut src_index: i32 = 0;

    // row 0, 1, .., 7
    loop {
        *out.offset(dst_index as isize) = *in_.offset(src_index as isize);
        *out.offset((dst_index + 1) as isize) = *in_.offset((src_index + 1) as isize);
        *out.offset((dst_index + 2) as isize) = *in_.offset((src_index + 16) as isize);
        *out.offset((dst_index + 3) as isize) = *in_.offset((src_index + 17) as isize);
        dst_index += 4;
        src_index += 2;
        row_index += 1;
        if !(row_index < 8) {
            break;
        }
    }

    // row 8, 9, ..., 15
    src_index += 16;
    loop {
        *out.offset(dst_index as isize) = *in_.offset(src_index as isize);
        *out.offset((dst_index + 1) as isize) = *in_.offset((src_index + 1) as isize);
        *out.offset((dst_index + 2) as isize) = *in_.offset((src_index + 16) as isize);
        *out.offset((dst_index + 3) as isize) = *in_.offset((src_index + 17) as isize);
        dst_index += 4;
        src_index += 2;
        row_index += 1;
        if !(row_index < 16) {
            break;
        }
    }
}

#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn load_buffer_16x16(input: *const i16, out: *mut __m128i, stride: i32, flipud: i32,
                            fliplr: i32, shift: i32) {
    let mut in__: [__m128i; 64] = core::mem::zeroed();
    let in_: *mut __m128i = in__.as_mut_ptr();
    // Load 4 8x8 blocks
    let mut topL: *const i16 = input;
    let mut topR: *const i16 = input.add(8);
    let mut botL: *const i16 = input.offset((8 * stride) as isize);
    let mut botR: *const i16 = input.offset((8 * stride) as isize).add(8);

    let mut tmp: *const i16;

    if flipud != 0 {
        // Swap left columns
        tmp = topL;
        topL = botL;
        botL = tmp;
        // Swap right columns
        tmp = topR;
        topR = botR;
        botR = tmp;
    }

    if fliplr != 0 {
        // Swap top rows
        tmp = topL;
        topL = topR;
        topR = tmp;
        // Swap bottom rows
        tmp = botL;
        botL = botR;
        botR = tmp;
    }

    // load first 8 columns
    load_buffer_8x8(topL, in_.add(0), stride, flipud, fliplr, shift);
    load_buffer_8x8(botL, in_.add(32), stride, flipud, fliplr, shift);

    // load second 8 columns
    load_buffer_8x8(topR, in_.add(16), stride, flipud, fliplr, shift);
    load_buffer_8x8(botR, in_.add(48), stride, flipud, fliplr, shift);

    convert_8x8_to_16x16(in_, out);
}

#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn load_buffer_8x16(input: *const i16, out: *mut __m128i, stride: i32, flipud: i32,
                           fliplr: i32, shift: i32) {
    let mut topL: *const i16 = input;
    let mut botL: *const i16 = input.offset((8 * stride) as isize);

    let tmp: *const i16;

    if flipud != 0 {
        tmp = topL;
        topL = botL;
        botL = tmp;
    }

    load_buffer_8x8(topL, out, stride, flipud, fliplr, shift);
    load_buffer_8x8(botL, out.add(16), stride, flipud, fliplr, shift);
}

#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn load_buffer_8x4(input: *const i16, out: *mut __m128i, stride: i32, flipud: i32,
                          fliplr: i32, shift: i32) {
    let mut topL: *const i16 = input;
    let mut topR: *const i16 = input.add(4);

    let tmp: *const i16;

    if fliplr != 0 {
        tmp = topL;
        topL = topR;
        topR = tmp;
    }

    load_buffer_4x4(topL, out, stride, flipud, fliplr, shift);
    load_buffer_4x4(topR, out.add(4), stride, flipud, fliplr, shift);
}

#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn load_buffer_16x4(input: *const i16, out: *mut __m128i, stride: i32, flipud: i32,
                           fliplr: i32, shift: i32) {
    let mut topL: *const i16 = input;
    let mut topR: *const i16 = input.add(8);

    let tmp: *const i16;

    if fliplr != 0 {
        tmp = topL;
        topL = topR;
        topR = tmp;
    }

    load_buffer_8x4(topL, out, stride, flipud, fliplr, shift);
    load_buffer_8x4(topR, out.add(8), stride, flipud, fliplr, shift);
}

#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn load_buffer_4x8(input: *const i16, out: *mut __m128i, stride: i32, flipud: i32,
                          fliplr: i32, shift: i32) {
    let mut topL: *const i16 = input;
    let mut botL: *const i16 = input.offset((4 * stride) as isize);

    let tmp: *const i16;

    if flipud != 0 {
        tmp = topL;
        topL = botL;
        botL = tmp;
    }

    load_buffer_4x4(topL, out, stride, flipud, fliplr, shift);
    load_buffer_4x4(botL, out.add(4), stride, flipud, fliplr, shift);
}

#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn load_buffer_4x16(input: *const i16, out: *mut __m128i, stride: i32, flipud: i32,
                           fliplr: i32, shift: i32) {
    let mut topL: *const i16 = input;
    let mut botL: *const i16 = input.offset((8 * stride) as isize);

    let tmp: *const i16;

    if flipud != 0 {
        tmp = topL;
        topL = botL;
        botL = tmp;
    }
    load_buffer_4x8(topL, out, stride, flipud, fliplr, shift);
    load_buffer_4x8(botL, out.add(8), stride, flipud, fliplr, shift);
}

#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn load_buffer_32x8n(input: *const i16, out: *mut __m128i, stride: i32, flipud: i32,
                            fliplr: i32, shift: i32, height: i32) {
    let mut in_: *const i16 = input;
    let mut output: *mut __m128i = out;
    for col in 0..height {
        in_ = input.offset((col * stride) as isize);
        output = out.offset((col * 8) as isize);
        load_buffer_4x4(in_, output, 4, flipud, fliplr, shift);
        load_buffer_4x4(in_.add(16), output.add(4), 4, flipud, fliplr, shift);
    }
}

#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn fdct16x16_sse4_1(in_: *mut __m128i, out: *mut __m128i, bit: i32, col_num: i32) {
    let cospi: *const i32 = cospi_arr(bit);
    let cospi32: __m128i = _mm_set1_epi32(*cospi.add(32));
    let cospim32: __m128i = _mm_set1_epi32(-*cospi.add(32));
    let cospi48: __m128i = _mm_set1_epi32(*cospi.add(48));
    let cospi16: __m128i = _mm_set1_epi32(*cospi.add(16));
    let cospim48: __m128i = _mm_set1_epi32(-*cospi.add(48));
    let cospim16: __m128i = _mm_set1_epi32(-*cospi.add(16));
    let cospi56: __m128i = _mm_set1_epi32(*cospi.add(56));
    let cospi8: __m128i = _mm_set1_epi32(*cospi.add(8));
    let cospi24: __m128i = _mm_set1_epi32(*cospi.add(24));
    let cospi40: __m128i = _mm_set1_epi32(*cospi.add(40));
    let cospi60: __m128i = _mm_set1_epi32(*cospi.add(60));
    let cospi4: __m128i = _mm_set1_epi32(*cospi.add(4));
    let cospi28: __m128i = _mm_set1_epi32(*cospi.add(28));
    let cospi36: __m128i = _mm_set1_epi32(*cospi.add(36));
    let cospi44: __m128i = _mm_set1_epi32(*cospi.add(44));
    let cospi20: __m128i = _mm_set1_epi32(*cospi.add(20));
    let cospi12: __m128i = _mm_set1_epi32(*cospi.add(12));
    let cospi52: __m128i = _mm_set1_epi32(*cospi.add(52));
    let rnding: __m128i = _mm_set1_epi32(1 << (bit - 1));
    let mut u_: [__m128i; 16] = core::mem::zeroed();
    let u: *mut __m128i = u_.as_mut_ptr();
    let mut v_: [__m128i; 16] = core::mem::zeroed();
    let v: *mut __m128i = v_.as_mut_ptr();
    let mut x: __m128i;

    // Calculate the column 0, 1, 2, 3
    for col in 0..col_num {
        // stage 0
        // stage 1
        *u.add(0) = _mm_add_epi32(*in_.offset((0 * col_num + col) as isize),
                                  *in_.offset((15 * col_num + col) as isize));
        *u.add(15) = _mm_sub_epi32(*in_.offset((0 * col_num + col) as isize),
                                   *in_.offset((15 * col_num + col) as isize));
        *u.add(1) = _mm_add_epi32(*in_.offset((1 * col_num + col) as isize),
                                  *in_.offset((14 * col_num + col) as isize));
        *u.add(14) = _mm_sub_epi32(*in_.offset((1 * col_num + col) as isize),
                                   *in_.offset((14 * col_num + col) as isize));
        *u.add(2) = _mm_add_epi32(*in_.offset((2 * col_num + col) as isize),
                                  *in_.offset((13 * col_num + col) as isize));
        *u.add(13) = _mm_sub_epi32(*in_.offset((2 * col_num + col) as isize),
                                   *in_.offset((13 * col_num + col) as isize));
        *u.add(3) = _mm_add_epi32(*in_.offset((3 * col_num + col) as isize),
                                  *in_.offset((12 * col_num + col) as isize));
        *u.add(12) = _mm_sub_epi32(*in_.offset((3 * col_num + col) as isize),
                                   *in_.offset((12 * col_num + col) as isize));
        *u.add(4) = _mm_add_epi32(*in_.offset((4 * col_num + col) as isize),
                                  *in_.offset((11 * col_num + col) as isize));
        *u.add(11) = _mm_sub_epi32(*in_.offset((4 * col_num + col) as isize),
                                   *in_.offset((11 * col_num + col) as isize));
        *u.add(5) = _mm_add_epi32(*in_.offset((5 * col_num + col) as isize),
                                  *in_.offset((10 * col_num + col) as isize));
        *u.add(10) = _mm_sub_epi32(*in_.offset((5 * col_num + col) as isize),
                                   *in_.offset((10 * col_num + col) as isize));
        *u.add(6) = _mm_add_epi32(*in_.offset((6 * col_num + col) as isize),
                                  *in_.offset((9 * col_num + col) as isize));
        *u.add(9) = _mm_sub_epi32(*in_.offset((6 * col_num + col) as isize),
                                  *in_.offset((9 * col_num + col) as isize));
        *u.add(7) = _mm_add_epi32(*in_.offset((7 * col_num + col) as isize),
                                  *in_.offset((8 * col_num + col) as isize));
        *u.add(8) = _mm_sub_epi32(*in_.offset((7 * col_num + col) as isize),
                                  *in_.offset((8 * col_num + col) as isize));

        // stage 2
        *v.add(0) = _mm_add_epi32(*u.add(0), *u.add(7));
        *v.add(7) = _mm_sub_epi32(*u.add(0), *u.add(7));
        *v.add(1) = _mm_add_epi32(*u.add(1), *u.add(6));
        *v.add(6) = _mm_sub_epi32(*u.add(1), *u.add(6));
        *v.add(2) = _mm_add_epi32(*u.add(2), *u.add(5));
        *v.add(5) = _mm_sub_epi32(*u.add(2), *u.add(5));
        *v.add(3) = _mm_add_epi32(*u.add(3), *u.add(4));
        *v.add(4) = _mm_sub_epi32(*u.add(3), *u.add(4));
        *v.add(8) = *u.add(8);
        *v.add(9) = *u.add(9);

        *v.add(10) = _mm_mullo_epi32(*u.add(10), cospim32);
        x = _mm_mullo_epi32(*u.add(13), cospi32);
        *v.add(10) = _mm_add_epi32(*v.add(10), x);
        *v.add(10) = _mm_add_epi32(*v.add(10), rnding);
        *v.add(10) = _mm_sra_epi32(*v.add(10), _mm_cvtsi32_si128(bit));

        *v.add(13) = _mm_mullo_epi32(*u.add(10), cospi32);
        x = _mm_mullo_epi32(*u.add(13), cospim32);
        *v.add(13) = _mm_sub_epi32(*v.add(13), x);
        *v.add(13) = _mm_add_epi32(*v.add(13), rnding);
        *v.add(13) = _mm_sra_epi32(*v.add(13), _mm_cvtsi32_si128(bit));

        *v.add(11) = _mm_mullo_epi32(*u.add(11), cospim32);
        x = _mm_mullo_epi32(*u.add(12), cospi32);
        *v.add(11) = _mm_add_epi32(*v.add(11), x);
        *v.add(11) = _mm_add_epi32(*v.add(11), rnding);
        *v.add(11) = _mm_sra_epi32(*v.add(11), _mm_cvtsi32_si128(bit));

        *v.add(12) = _mm_mullo_epi32(*u.add(11), cospi32);
        x = _mm_mullo_epi32(*u.add(12), cospim32);
        *v.add(12) = _mm_sub_epi32(*v.add(12), x);
        *v.add(12) = _mm_add_epi32(*v.add(12), rnding);
        *v.add(12) = _mm_sra_epi32(*v.add(12), _mm_cvtsi32_si128(bit));
        *v.add(14) = *u.add(14);
        *v.add(15) = *u.add(15);

        // stage 3
        *u.add(0) = _mm_add_epi32(*v.add(0), *v.add(3));
        *u.add(3) = _mm_sub_epi32(*v.add(0), *v.add(3));
        *u.add(1) = _mm_add_epi32(*v.add(1), *v.add(2));
        *u.add(2) = _mm_sub_epi32(*v.add(1), *v.add(2));
        *u.add(4) = *v.add(4);

        *u.add(5) = _mm_mullo_epi32(*v.add(5), cospim32);
        x = _mm_mullo_epi32(*v.add(6), cospi32);
        *u.add(5) = _mm_add_epi32(*u.add(5), x);
        *u.add(5) = _mm_add_epi32(*u.add(5), rnding);
        *u.add(5) = _mm_sra_epi32(*u.add(5), _mm_cvtsi32_si128(bit));

        *u.add(6) = _mm_mullo_epi32(*v.add(5), cospi32);
        x = _mm_mullo_epi32(*v.add(6), cospim32);
        *u.add(6) = _mm_sub_epi32(*u.add(6), x);
        *u.add(6) = _mm_add_epi32(*u.add(6), rnding);
        *u.add(6) = _mm_sra_epi32(*u.add(6), _mm_cvtsi32_si128(bit));

        *u.add(7) = *v.add(7);
        *u.add(8) = _mm_add_epi32(*v.add(8), *v.add(11));
        *u.add(11) = _mm_sub_epi32(*v.add(8), *v.add(11));
        *u.add(9) = _mm_add_epi32(*v.add(9), *v.add(10));
        *u.add(10) = _mm_sub_epi32(*v.add(9), *v.add(10));
        *u.add(12) = _mm_sub_epi32(*v.add(15), *v.add(12));
        *u.add(15) = _mm_add_epi32(*v.add(15), *v.add(12));
        *u.add(13) = _mm_sub_epi32(*v.add(14), *v.add(13));
        *u.add(14) = _mm_add_epi32(*v.add(14), *v.add(13));

        // stage 4
        *u.add(0) = _mm_mullo_epi32(*u.add(0), cospi32);
        *u.add(1) = _mm_mullo_epi32(*u.add(1), cospi32);
        *v.add(0) = _mm_add_epi32(*u.add(0), *u.add(1));
        *v.add(0) = _mm_add_epi32(*v.add(0), rnding);
        *v.add(0) = _mm_sra_epi32(*v.add(0), _mm_cvtsi32_si128(bit));

        *v.add(1) = _mm_sub_epi32(*u.add(0), *u.add(1));
        *v.add(1) = _mm_add_epi32(*v.add(1), rnding);
        *v.add(1) = _mm_sra_epi32(*v.add(1), _mm_cvtsi32_si128(bit));

        *v.add(2) = _mm_mullo_epi32(*u.add(2), cospi48);
        x = _mm_mullo_epi32(*u.add(3), cospi16);
        *v.add(2) = _mm_add_epi32(*v.add(2), x);
        *v.add(2) = _mm_add_epi32(*v.add(2), rnding);
        *v.add(2) = _mm_sra_epi32(*v.add(2), _mm_cvtsi32_si128(bit));

        *v.add(3) = _mm_mullo_epi32(*u.add(2), cospi16);
        x = _mm_mullo_epi32(*u.add(3), cospi48);
        *v.add(3) = _mm_sub_epi32(x, *v.add(3));
        *v.add(3) = _mm_add_epi32(*v.add(3), rnding);
        *v.add(3) = _mm_sra_epi32(*v.add(3), _mm_cvtsi32_si128(bit));

        *v.add(4) = _mm_add_epi32(*u.add(4), *u.add(5));
        *v.add(5) = _mm_sub_epi32(*u.add(4), *u.add(5));
        *v.add(6) = _mm_sub_epi32(*u.add(7), *u.add(6));
        *v.add(7) = _mm_add_epi32(*u.add(7), *u.add(6));
        *v.add(8) = *u.add(8);

        *v.add(9) = _mm_mullo_epi32(*u.add(9), cospim16);
        x = _mm_mullo_epi32(*u.add(14), cospi48);
        *v.add(9) = _mm_add_epi32(*v.add(9), x);
        *v.add(9) = _mm_add_epi32(*v.add(9), rnding);
        *v.add(9) = _mm_sra_epi32(*v.add(9), _mm_cvtsi32_si128(bit));

        *v.add(14) = _mm_mullo_epi32(*u.add(9), cospi48);
        x = _mm_mullo_epi32(*u.add(14), cospim16);
        *v.add(14) = _mm_sub_epi32(*v.add(14), x);
        *v.add(14) = _mm_add_epi32(*v.add(14), rnding);
        *v.add(14) = _mm_sra_epi32(*v.add(14), _mm_cvtsi32_si128(bit));

        *v.add(10) = _mm_mullo_epi32(*u.add(10), cospim48);
        x = _mm_mullo_epi32(*u.add(13), cospim16);
        *v.add(10) = _mm_add_epi32(*v.add(10), x);
        *v.add(10) = _mm_add_epi32(*v.add(10), rnding);
        *v.add(10) = _mm_sra_epi32(*v.add(10), _mm_cvtsi32_si128(bit));

        *v.add(13) = _mm_mullo_epi32(*u.add(10), cospim16);
        x = _mm_mullo_epi32(*u.add(13), cospim48);
        *v.add(13) = _mm_sub_epi32(*v.add(13), x);
        *v.add(13) = _mm_add_epi32(*v.add(13), rnding);
        *v.add(13) = _mm_sra_epi32(*v.add(13), _mm_cvtsi32_si128(bit));

        *v.add(11) = *u.add(11);
        *v.add(12) = *u.add(12);
        *v.add(15) = *u.add(15);

        // stage 5
        *u.add(0) = *v.add(0);
        *u.add(1) = *v.add(1);
        *u.add(2) = *v.add(2);
        *u.add(3) = *v.add(3);

        *u.add(4) = _mm_mullo_epi32(*v.add(4), cospi56);
        x = _mm_mullo_epi32(*v.add(7), cospi8);
        *u.add(4) = _mm_add_epi32(*u.add(4), x);
        *u.add(4) = _mm_add_epi32(*u.add(4), rnding);
        *u.add(4) = _mm_sra_epi32(*u.add(4), _mm_cvtsi32_si128(bit));

        *u.add(7) = _mm_mullo_epi32(*v.add(4), cospi8);
        x = _mm_mullo_epi32(*v.add(7), cospi56);
        *u.add(7) = _mm_sub_epi32(x, *u.add(7));
        *u.add(7) = _mm_add_epi32(*u.add(7), rnding);
        *u.add(7) = _mm_sra_epi32(*u.add(7), _mm_cvtsi32_si128(bit));

        *u.add(5) = _mm_mullo_epi32(*v.add(5), cospi24);
        x = _mm_mullo_epi32(*v.add(6), cospi40);
        *u.add(5) = _mm_add_epi32(*u.add(5), x);
        *u.add(5) = _mm_add_epi32(*u.add(5), rnding);
        *u.add(5) = _mm_sra_epi32(*u.add(5), _mm_cvtsi32_si128(bit));

        *u.add(6) = _mm_mullo_epi32(*v.add(5), cospi40);
        x = _mm_mullo_epi32(*v.add(6), cospi24);
        *u.add(6) = _mm_sub_epi32(x, *u.add(6));
        *u.add(6) = _mm_add_epi32(*u.add(6), rnding);
        *u.add(6) = _mm_sra_epi32(*u.add(6), _mm_cvtsi32_si128(bit));

        *u.add(8) = _mm_add_epi32(*v.add(8), *v.add(9));
        *u.add(9) = _mm_sub_epi32(*v.add(8), *v.add(9));
        *u.add(10) = _mm_sub_epi32(*v.add(11), *v.add(10));
        *u.add(11) = _mm_add_epi32(*v.add(11), *v.add(10));
        *u.add(12) = _mm_add_epi32(*v.add(12), *v.add(13));
        *u.add(13) = _mm_sub_epi32(*v.add(12), *v.add(13));
        *u.add(14) = _mm_sub_epi32(*v.add(15), *v.add(14));
        *u.add(15) = _mm_add_epi32(*v.add(15), *v.add(14));

        // stage 6
        *v.add(0) = *u.add(0);
        *v.add(1) = *u.add(1);
        *v.add(2) = *u.add(2);
        *v.add(3) = *u.add(3);
        *v.add(4) = *u.add(4);
        *v.add(5) = *u.add(5);
        *v.add(6) = *u.add(6);
        *v.add(7) = *u.add(7);

        *v.add(8) = _mm_mullo_epi32(*u.add(8), cospi60);
        x = _mm_mullo_epi32(*u.add(15), cospi4);
        *v.add(8) = _mm_add_epi32(*v.add(8), x);
        *v.add(8) = _mm_add_epi32(*v.add(8), rnding);
        *v.add(8) = _mm_sra_epi32(*v.add(8), _mm_cvtsi32_si128(bit));

        *v.add(15) = _mm_mullo_epi32(*u.add(8), cospi4);
        x = _mm_mullo_epi32(*u.add(15), cospi60);
        *v.add(15) = _mm_sub_epi32(x, *v.add(15));
        *v.add(15) = _mm_add_epi32(*v.add(15), rnding);
        *v.add(15) = _mm_sra_epi32(*v.add(15), _mm_cvtsi32_si128(bit));

        *v.add(9) = _mm_mullo_epi32(*u.add(9), cospi28);
        x = _mm_mullo_epi32(*u.add(14), cospi36);
        *v.add(9) = _mm_add_epi32(*v.add(9), x);
        *v.add(9) = _mm_add_epi32(*v.add(9), rnding);
        *v.add(9) = _mm_sra_epi32(*v.add(9), _mm_cvtsi32_si128(bit));

        *v.add(14) = _mm_mullo_epi32(*u.add(9), cospi36);
        x = _mm_mullo_epi32(*u.add(14), cospi28);
        *v.add(14) = _mm_sub_epi32(x, *v.add(14));
        *v.add(14) = _mm_add_epi32(*v.add(14), rnding);
        *v.add(14) = _mm_sra_epi32(*v.add(14), _mm_cvtsi32_si128(bit));

        *v.add(10) = _mm_mullo_epi32(*u.add(10), cospi44);
        x = _mm_mullo_epi32(*u.add(13), cospi20);
        *v.add(10) = _mm_add_epi32(*v.add(10), x);
        *v.add(10) = _mm_add_epi32(*v.add(10), rnding);
        *v.add(10) = _mm_sra_epi32(*v.add(10), _mm_cvtsi32_si128(bit));

        *v.add(13) = _mm_mullo_epi32(*u.add(10), cospi20);
        x = _mm_mullo_epi32(*u.add(13), cospi44);
        *v.add(13) = _mm_sub_epi32(x, *v.add(13));
        *v.add(13) = _mm_add_epi32(*v.add(13), rnding);
        *v.add(13) = _mm_sra_epi32(*v.add(13), _mm_cvtsi32_si128(bit));

        *v.add(11) = _mm_mullo_epi32(*u.add(11), cospi12);
        x = _mm_mullo_epi32(*u.add(12), cospi52);
        *v.add(11) = _mm_add_epi32(*v.add(11), x);
        *v.add(11) = _mm_add_epi32(*v.add(11), rnding);
        *v.add(11) = _mm_sra_epi32(*v.add(11), _mm_cvtsi32_si128(bit));

        *v.add(12) = _mm_mullo_epi32(*u.add(11), cospi52);
        x = _mm_mullo_epi32(*u.add(12), cospi12);
        *v.add(12) = _mm_sub_epi32(x, *v.add(12));
        *v.add(12) = _mm_add_epi32(*v.add(12), rnding);
        *v.add(12) = _mm_sra_epi32(*v.add(12), _mm_cvtsi32_si128(bit));

        *out.offset((0 * col_num + col) as isize) = *v.add(0);
        *out.offset((1 * col_num + col) as isize) = *v.add(8);
        *out.offset((2 * col_num + col) as isize) = *v.add(4);
        *out.offset((3 * col_num + col) as isize) = *v.add(12);
        *out.offset((4 * col_num + col) as isize) = *v.add(2);
        *out.offset((5 * col_num + col) as isize) = *v.add(10);
        *out.offset((6 * col_num + col) as isize) = *v.add(6);
        *out.offset((7 * col_num + col) as isize) = *v.add(14);
        *out.offset((8 * col_num + col) as isize) = *v.add(1);
        *out.offset((9 * col_num + col) as isize) = *v.add(9);
        *out.offset((10 * col_num + col) as isize) = *v.add(5);
        *out.offset((11 * col_num + col) as isize) = *v.add(13);
        *out.offset((12 * col_num + col) as isize) = *v.add(3);
        *out.offset((13 * col_num + col) as isize) = *v.add(11);
        *out.offset((14 * col_num + col) as isize) = *v.add(7);
        *out.offset((15 * col_num + col) as isize) = *v.add(15);
    }
}

#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn fadst16x16_sse4_1(in_: *mut __m128i, out: *mut __m128i, bit: i32, num_cols: i32) {
    let cospi: *const i32 = cospi_arr(bit);
    let cospi32: __m128i = _mm_set1_epi32(*cospi.add(32));
    let cospi48: __m128i = _mm_set1_epi32(*cospi.add(48));
    let cospi16: __m128i = _mm_set1_epi32(*cospi.add(16));
    let cospim16: __m128i = _mm_set1_epi32(-*cospi.add(16));
    let cospim48: __m128i = _mm_set1_epi32(-*cospi.add(48));
    let cospi8: __m128i = _mm_set1_epi32(*cospi.add(8));
    let cospi56: __m128i = _mm_set1_epi32(*cospi.add(56));
    let cospim56: __m128i = _mm_set1_epi32(-*cospi.add(56));
    let cospim8: __m128i = _mm_set1_epi32(-*cospi.add(8));
    let cospi24: __m128i = _mm_set1_epi32(*cospi.add(24));
    let cospim24: __m128i = _mm_set1_epi32(-*cospi.add(24));
    let cospim40: __m128i = _mm_set1_epi32(-*cospi.add(40));
    let cospi40: __m128i = _mm_set1_epi32(*cospi.add(40));
    let cospi2: __m128i = _mm_set1_epi32(*cospi.add(2));
    let cospi62: __m128i = _mm_set1_epi32(*cospi.add(62));
    let cospim2: __m128i = _mm_set1_epi32(-*cospi.add(2));
    let cospi10: __m128i = _mm_set1_epi32(*cospi.add(10));
    let cospi54: __m128i = _mm_set1_epi32(*cospi.add(54));
    let cospim10: __m128i = _mm_set1_epi32(-*cospi.add(10));
    let cospi18: __m128i = _mm_set1_epi32(*cospi.add(18));
    let cospi46: __m128i = _mm_set1_epi32(*cospi.add(46));
    let cospim18: __m128i = _mm_set1_epi32(-*cospi.add(18));
    let cospi26: __m128i = _mm_set1_epi32(*cospi.add(26));
    let cospi38: __m128i = _mm_set1_epi32(*cospi.add(38));
    let cospim26: __m128i = _mm_set1_epi32(-*cospi.add(26));
    let cospi34: __m128i = _mm_set1_epi32(*cospi.add(34));
    let cospi30: __m128i = _mm_set1_epi32(*cospi.add(30));
    let cospim34: __m128i = _mm_set1_epi32(-*cospi.add(34));
    let cospi42: __m128i = _mm_set1_epi32(*cospi.add(42));
    let cospi22: __m128i = _mm_set1_epi32(*cospi.add(22));
    let cospim42: __m128i = _mm_set1_epi32(-*cospi.add(42));
    let cospi50: __m128i = _mm_set1_epi32(*cospi.add(50));
    let cospi14: __m128i = _mm_set1_epi32(*cospi.add(14));
    let cospim50: __m128i = _mm_set1_epi32(-*cospi.add(50));
    let cospi58: __m128i = _mm_set1_epi32(*cospi.add(58));
    let cospi6: __m128i = _mm_set1_epi32(*cospi.add(6));
    let cospim58: __m128i = _mm_set1_epi32(-*cospi.add(58));
    let rnding: __m128i = _mm_set1_epi32(1 << (bit - 1));
    let zero: __m128i = _mm_setzero_si128();

    let mut u_: [__m128i; 16] = core::mem::zeroed();

    let u: *mut __m128i = u_.as_mut_ptr();
    let mut v_: [__m128i; 16] = core::mem::zeroed();
    let v: *mut __m128i = v_.as_mut_ptr();
    let mut x: __m128i;
    let mut y: __m128i;

    for col in 0..num_cols {
        // stage 0
        // stage 1
        *u.add(0) = *in_.offset((0 * num_cols + col) as isize);
        *u.add(1) = _mm_sub_epi32(zero, *in_.offset((15 * num_cols + col) as isize));
        *u.add(2) = _mm_sub_epi32(zero, *in_.offset((7 * num_cols + col) as isize));
        *u.add(3) = *in_.offset((8 * num_cols + col) as isize);
        *u.add(4) = _mm_sub_epi32(zero, *in_.offset((3 * num_cols + col) as isize));
        *u.add(5) = *in_.offset((12 * num_cols + col) as isize);
        *u.add(6) = *in_.offset((4 * num_cols + col) as isize);
        *u.add(7) = _mm_sub_epi32(zero, *in_.offset((11 * num_cols + col) as isize));
        *u.add(8) = _mm_sub_epi32(zero, *in_.offset((1 * num_cols + col) as isize));
        *u.add(9) = *in_.offset((14 * num_cols + col) as isize);
        *u.add(10) = *in_.offset((6 * num_cols + col) as isize);
        *u.add(11) = _mm_sub_epi32(zero, *in_.offset((9 * num_cols + col) as isize));
        *u.add(12) = *in_.offset((2 * num_cols + col) as isize);
        *u.add(13) = _mm_sub_epi32(zero, *in_.offset((13 * num_cols + col) as isize));
        *u.add(14) = _mm_sub_epi32(zero, *in_.offset((5 * num_cols + col) as isize));
        *u.add(15) = *in_.offset((10 * num_cols + col) as isize);

        // stage 2
        *v.add(0) = *u.add(0);
        *v.add(1) = *u.add(1);

        x = _mm_mullo_epi32(*u.add(2), cospi32);
        y = _mm_mullo_epi32(*u.add(3), cospi32);
        *v.add(2) = _mm_add_epi32(x, y);
        *v.add(2) = _mm_add_epi32(*v.add(2), rnding);
        *v.add(2) = _mm_sra_epi32(*v.add(2), _mm_cvtsi32_si128(bit));

        *v.add(3) = _mm_sub_epi32(x, y);
        *v.add(3) = _mm_add_epi32(*v.add(3), rnding);
        *v.add(3) = _mm_sra_epi32(*v.add(3), _mm_cvtsi32_si128(bit));

        *v.add(4) = *u.add(4);
        *v.add(5) = *u.add(5);

        x = _mm_mullo_epi32(*u.add(6), cospi32);
        y = _mm_mullo_epi32(*u.add(7), cospi32);
        *v.add(6) = _mm_add_epi32(x, y);
        *v.add(6) = _mm_add_epi32(*v.add(6), rnding);
        *v.add(6) = _mm_sra_epi32(*v.add(6), _mm_cvtsi32_si128(bit));

        *v.add(7) = _mm_sub_epi32(x, y);
        *v.add(7) = _mm_add_epi32(*v.add(7), rnding);
        *v.add(7) = _mm_sra_epi32(*v.add(7), _mm_cvtsi32_si128(bit));

        *v.add(8) = *u.add(8);
        *v.add(9) = *u.add(9);

        x = _mm_mullo_epi32(*u.add(10), cospi32);
        y = _mm_mullo_epi32(*u.add(11), cospi32);
        *v.add(10) = _mm_add_epi32(x, y);
        *v.add(10) = _mm_add_epi32(*v.add(10), rnding);
        *v.add(10) = _mm_sra_epi32(*v.add(10), _mm_cvtsi32_si128(bit));

        *v.add(11) = _mm_sub_epi32(x, y);
        *v.add(11) = _mm_add_epi32(*v.add(11), rnding);
        *v.add(11) = _mm_sra_epi32(*v.add(11), _mm_cvtsi32_si128(bit));

        *v.add(12) = *u.add(12);
        *v.add(13) = *u.add(13);

        x = _mm_mullo_epi32(*u.add(14), cospi32);
        y = _mm_mullo_epi32(*u.add(15), cospi32);
        *v.add(14) = _mm_add_epi32(x, y);
        *v.add(14) = _mm_add_epi32(*v.add(14), rnding);
        *v.add(14) = _mm_sra_epi32(*v.add(14), _mm_cvtsi32_si128(bit));

        *v.add(15) = _mm_sub_epi32(x, y);
        *v.add(15) = _mm_add_epi32(*v.add(15), rnding);
        *v.add(15) = _mm_sra_epi32(*v.add(15), _mm_cvtsi32_si128(bit));

        // stage 3
        *u.add(0) = _mm_add_epi32(*v.add(0), *v.add(2));
        *u.add(1) = _mm_add_epi32(*v.add(1), *v.add(3));
        *u.add(2) = _mm_sub_epi32(*v.add(0), *v.add(2));
        *u.add(3) = _mm_sub_epi32(*v.add(1), *v.add(3));
        *u.add(4) = _mm_add_epi32(*v.add(4), *v.add(6));
        *u.add(5) = _mm_add_epi32(*v.add(5), *v.add(7));
        *u.add(6) = _mm_sub_epi32(*v.add(4), *v.add(6));
        *u.add(7) = _mm_sub_epi32(*v.add(5), *v.add(7));
        *u.add(8) = _mm_add_epi32(*v.add(8), *v.add(10));
        *u.add(9) = _mm_add_epi32(*v.add(9), *v.add(11));
        *u.add(10) = _mm_sub_epi32(*v.add(8), *v.add(10));
        *u.add(11) = _mm_sub_epi32(*v.add(9), *v.add(11));
        *u.add(12) = _mm_add_epi32(*v.add(12), *v.add(14));
        *u.add(13) = _mm_add_epi32(*v.add(13), *v.add(15));
        *u.add(14) = _mm_sub_epi32(*v.add(12), *v.add(14));
        *u.add(15) = _mm_sub_epi32(*v.add(13), *v.add(15));

        // stage 4
        *v.add(0) = *u.add(0);
        *v.add(1) = *u.add(1);
        *v.add(2) = *u.add(2);
        *v.add(3) = *u.add(3);
        *v.add(4) = half_btf_sse4_1(&cospi16, u.add(4), &cospi48, u.add(5), &rnding, bit);
        *v.add(5) = half_btf_sse4_1(&cospi48, u.add(4), &cospim16, u.add(5), &rnding, bit);
        *v.add(6) = half_btf_sse4_1(&cospim48, u.add(6), &cospi16, u.add(7), &rnding, bit);
        *v.add(7) = half_btf_sse4_1(&cospi16, u.add(6), &cospi48, u.add(7), &rnding, bit);
        *v.add(8) = *u.add(8);
        *v.add(9) = *u.add(9);
        *v.add(10) = *u.add(10);
        *v.add(11) = *u.add(11);
        *v.add(12) = half_btf_sse4_1(&cospi16, u.add(12), &cospi48, u.add(13), &rnding, bit);
        *v.add(13) = half_btf_sse4_1(&cospi48, u.add(12), &cospim16, u.add(13), &rnding, bit);
        *v.add(14) = half_btf_sse4_1(&cospim48, u.add(14), &cospi16, u.add(15), &rnding, bit);
        *v.add(15) = half_btf_sse4_1(&cospi16, u.add(14), &cospi48, u.add(15), &rnding, bit);

        // stage 5
        *u.add(0) = _mm_add_epi32(*v.add(0), *v.add(4));
        *u.add(1) = _mm_add_epi32(*v.add(1), *v.add(5));
        *u.add(2) = _mm_add_epi32(*v.add(2), *v.add(6));
        *u.add(3) = _mm_add_epi32(*v.add(3), *v.add(7));
        *u.add(4) = _mm_sub_epi32(*v.add(0), *v.add(4));
        *u.add(5) = _mm_sub_epi32(*v.add(1), *v.add(5));
        *u.add(6) = _mm_sub_epi32(*v.add(2), *v.add(6));
        *u.add(7) = _mm_sub_epi32(*v.add(3), *v.add(7));
        *u.add(8) = _mm_add_epi32(*v.add(8), *v.add(12));
        *u.add(9) = _mm_add_epi32(*v.add(9), *v.add(13));
        *u.add(10) = _mm_add_epi32(*v.add(10), *v.add(14));
        *u.add(11) = _mm_add_epi32(*v.add(11), *v.add(15));
        *u.add(12) = _mm_sub_epi32(*v.add(8), *v.add(12));
        *u.add(13) = _mm_sub_epi32(*v.add(9), *v.add(13));
        *u.add(14) = _mm_sub_epi32(*v.add(10), *v.add(14));
        *u.add(15) = _mm_sub_epi32(*v.add(11), *v.add(15));

        // stage 6
        *v.add(0) = *u.add(0);
        *v.add(1) = *u.add(1);
        *v.add(2) = *u.add(2);
        *v.add(3) = *u.add(3);
        *v.add(4) = *u.add(4);
        *v.add(5) = *u.add(5);
        *v.add(6) = *u.add(6);
        *v.add(7) = *u.add(7);
        *v.add(8) = half_btf_sse4_1(&cospi8, u.add(8), &cospi56, u.add(9), &rnding, bit);
        *v.add(9) = half_btf_sse4_1(&cospi56, u.add(8), &cospim8, u.add(9), &rnding, bit);
        *v.add(10) = half_btf_sse4_1(&cospi40, u.add(10), &cospi24, u.add(11), &rnding, bit);
        *v.add(11) = half_btf_sse4_1(&cospi24, u.add(10), &cospim40, u.add(11), &rnding, bit);
        *v.add(12) = half_btf_sse4_1(&cospim56, u.add(12), &cospi8, u.add(13), &rnding, bit);
        *v.add(13) = half_btf_sse4_1(&cospi8, u.add(12), &cospi56, u.add(13), &rnding, bit);
        *v.add(14) = half_btf_sse4_1(&cospim24, u.add(14), &cospi40, u.add(15), &rnding, bit);
        *v.add(15) = half_btf_sse4_1(&cospi40, u.add(14), &cospi24, u.add(15), &rnding, bit);

        // stage 7
        *u.add(0) = _mm_add_epi32(*v.add(0), *v.add(8));
        *u.add(1) = _mm_add_epi32(*v.add(1), *v.add(9));
        *u.add(2) = _mm_add_epi32(*v.add(2), *v.add(10));
        *u.add(3) = _mm_add_epi32(*v.add(3), *v.add(11));
        *u.add(4) = _mm_add_epi32(*v.add(4), *v.add(12));
        *u.add(5) = _mm_add_epi32(*v.add(5), *v.add(13));
        *u.add(6) = _mm_add_epi32(*v.add(6), *v.add(14));
        *u.add(7) = _mm_add_epi32(*v.add(7), *v.add(15));
        *u.add(8) = _mm_sub_epi32(*v.add(0), *v.add(8));
        *u.add(9) = _mm_sub_epi32(*v.add(1), *v.add(9));
        *u.add(10) = _mm_sub_epi32(*v.add(2), *v.add(10));
        *u.add(11) = _mm_sub_epi32(*v.add(3), *v.add(11));
        *u.add(12) = _mm_sub_epi32(*v.add(4), *v.add(12));
        *u.add(13) = _mm_sub_epi32(*v.add(5), *v.add(13));
        *u.add(14) = _mm_sub_epi32(*v.add(6), *v.add(14));
        *u.add(15) = _mm_sub_epi32(*v.add(7), *v.add(15));

        // stage 8
        *v.add(0) = half_btf_sse4_1(&cospi2, u.add(0), &cospi62, u.add(1), &rnding, bit);
        *v.add(1) = half_btf_sse4_1(&cospi62, u.add(0), &cospim2, u.add(1), &rnding, bit);
        *v.add(2) = half_btf_sse4_1(&cospi10, u.add(2), &cospi54, u.add(3), &rnding, bit);
        *v.add(3) = half_btf_sse4_1(&cospi54, u.add(2), &cospim10, u.add(3), &rnding, bit);
        *v.add(4) = half_btf_sse4_1(&cospi18, u.add(4), &cospi46, u.add(5), &rnding, bit);
        *v.add(5) = half_btf_sse4_1(&cospi46, u.add(4), &cospim18, u.add(5), &rnding, bit);
        *v.add(6) = half_btf_sse4_1(&cospi26, u.add(6), &cospi38, u.add(7), &rnding, bit);
        *v.add(7) = half_btf_sse4_1(&cospi38, u.add(6), &cospim26, u.add(7), &rnding, bit);
        *v.add(8) = half_btf_sse4_1(&cospi34, u.add(8), &cospi30, u.add(9), &rnding, bit);
        *v.add(9) = half_btf_sse4_1(&cospi30, u.add(8), &cospim34, u.add(9), &rnding, bit);
        *v.add(10) = half_btf_sse4_1(&cospi42, u.add(10), &cospi22, u.add(11), &rnding, bit);
        *v.add(11) = half_btf_sse4_1(&cospi22, u.add(10), &cospim42, u.add(11), &rnding, bit);
        *v.add(12) = half_btf_sse4_1(&cospi50, u.add(12), &cospi14, u.add(13), &rnding, bit);
        *v.add(13) = half_btf_sse4_1(&cospi14, u.add(12), &cospim50, u.add(13), &rnding, bit);
        *v.add(14) = half_btf_sse4_1(&cospi58, u.add(14), &cospi6, u.add(15), &rnding, bit);
        *v.add(15) = half_btf_sse4_1(&cospi6, u.add(14), &cospim58, u.add(15), &rnding, bit);

        // stage 9
        *out.offset((0 * num_cols + col) as isize) = *v.add(1);
        *out.offset((1 * num_cols + col) as isize) = *v.add(14);
        *out.offset((2 * num_cols + col) as isize) = *v.add(3);
        *out.offset((3 * num_cols + col) as isize) = *v.add(12);
        *out.offset((4 * num_cols + col) as isize) = *v.add(5);
        *out.offset((5 * num_cols + col) as isize) = *v.add(10);
        *out.offset((6 * num_cols + col) as isize) = *v.add(7);
        *out.offset((7 * num_cols + col) as isize) = *v.add(8);
        *out.offset((8 * num_cols + col) as isize) = *v.add(9);
        *out.offset((9 * num_cols + col) as isize) = *v.add(6);
        *out.offset((10 * num_cols + col) as isize) = *v.add(11);
        *out.offset((11 * num_cols + col) as isize) = *v.add(4);
        *out.offset((12 * num_cols + col) as isize) = *v.add(13);
        *out.offset((13 * num_cols + col) as isize) = *v.add(2);
        *out.offset((14 * num_cols + col) as isize) = *v.add(15);
        *out.offset((15 * num_cols + col) as isize) = *v.add(0);
    }
}

#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn col_txfm_16x16_rounding(in_: *mut __m128i, shift: i32) {
    // Note:
    //  We split 16x16 rounding into 4 sections of 8x8 rounding,
    //  instead of 4 columns
    col_txfm_8x8_rounding(in_.add(0), shift);
    col_txfm_8x8_rounding(in_.add(16), shift);
    col_txfm_8x8_rounding(in_.add(32), shift);
    col_txfm_8x8_rounding(in_.add(48), shift);
}

#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn col_txfm_8x16_rounding(in_: *mut __m128i, shift: i32) {
    col_txfm_8x8_rounding(in_.add(0), shift);
    col_txfm_8x8_rounding(in_.add(16), shift);
}

#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn write_buffer_16x16(in_: *const __m128i, mut output: *mut i32) {
    let size_8x8: i32 = 16 * 4;
    write_buffer_8x8(in_.add(0), output);
    output = output.offset((size_8x8) as isize);
    write_buffer_8x8(in_.add(16), output);
    output = output.offset((size_8x8) as isize);
    write_buffer_8x8(in_.add(32), output);
    output = output.offset((size_8x8) as isize);
    write_buffer_8x8(in_.add(48), output);
}

#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn idtx16x16_sse4_1(in_: *mut __m128i, out: *mut __m128i, bit: i32, col_num: i32) {
    let _ = bit;
    let fact: __m128i = _mm_set1_epi32(2 * NewSqrt2);
    let offset: __m128i = _mm_set1_epi32(1 << (NewSqrt2Bits - 1));
    let mut a_low: __m128i;

    let num_iters: i32 = 16 * col_num;
    for i in 0..num_iters {
        a_low = _mm_mullo_epi32(*in_.offset(i as isize), fact);
        a_low = _mm_add_epi32(a_low, offset);
        *out.offset(i as isize) = _mm_srai_epi32::<NewSqrt2Bits>(a_low);
    }
}

#[no_mangle]
#[target_feature(enable = "sse4.1")]
pub unsafe extern "C-unwind" fn av1_fwd_txfm2d_16x16_sse4_1(input: *const i16, coeff: *mut i32,
                                                            stride: i32, tx_type: TX_TYPE,
                                                            bd: i32) {
    let mut in__: [__m128i; 64] = core::mem::zeroed();
    let in_: *mut __m128i = in__.as_mut_ptr();
    let mut out_: [__m128i; 64] = core::mem::zeroed();
    let out: *mut __m128i = out_.as_mut_ptr();
    let shift: *const i8 = av1_fwd_txfm_shift_ls[TX_16X16 as usize];
    let txw_idx: i32 = get_txw_idx(TX_16X16 as TX_SIZE);
    let txh_idx: i32 = get_txh_idx(TX_16X16 as TX_SIZE);
    let col_num: i32 = 4;
    match tx_type as i32 {
        DCT_DCT => {
            load_buffer_16x16(input, in_, stride, 0, 0, *shift.add(0) as i32);
            fdct16x16_sse4_1(in_, out,
                             av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32,
                             col_num);
            col_txfm_16x16_rounding(out, -(*shift.add(1) as i32));
            transpose_16x16(out, in_);
            fdct16x16_sse4_1(in_, out,
                             av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32,
                             col_num);
            write_buffer_16x16(out, coeff);
        }
        ADST_DCT => {
            load_buffer_16x16(input, in_, stride, 0, 0, *shift.add(0) as i32);
            fadst16x16_sse4_1(in_, out,
                              av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32,
                              col_num);
            col_txfm_16x16_rounding(out, -(*shift.add(1) as i32));
            transpose_16x16(out, in_);
            fdct16x16_sse4_1(in_, out,
                             av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32,
                             col_num);
            write_buffer_16x16(out, coeff);
        }
        DCT_ADST => {
            load_buffer_16x16(input, in_, stride, 0, 0, *shift.add(0) as i32);
            fdct16x16_sse4_1(in_, out,
                             av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32,
                             col_num);
            col_txfm_16x16_rounding(out, -(*shift.add(1) as i32));
            transpose_16x16(out, in_);
            fadst16x16_sse4_1(in_, out,
                              av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32,
                              col_num);
            write_buffer_16x16(out, coeff);
        }
        ADST_ADST => {
            load_buffer_16x16(input, in_, stride, 0, 0, *shift.add(0) as i32);
            fadst16x16_sse4_1(in_, out,
                              av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32,
                              col_num);
            col_txfm_16x16_rounding(out, -(*shift.add(1) as i32));
            transpose_16x16(out, in_);
            fadst16x16_sse4_1(in_, out,
                              av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32,
                              col_num);
            write_buffer_16x16(out, coeff);
        }
        FLIPADST_DCT => {
            load_buffer_16x16(input, in_, stride, 1, 0, *shift.add(0) as i32);
            fadst16x16_sse4_1(in_, out,
                              av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32,
                              col_num);
            col_txfm_16x16_rounding(out, -(*shift.add(1) as i32));
            transpose_16x16(out, in_);
            fdct16x16_sse4_1(in_, out,
                             av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32,
                             col_num);
            write_buffer_16x16(out, coeff);
        }
        DCT_FLIPADST => {
            load_buffer_16x16(input, in_, stride, 0, 1, *shift.add(0) as i32);
            fdct16x16_sse4_1(in_, out,
                             av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32,
                             col_num);
            col_txfm_16x16_rounding(out, -(*shift.add(1) as i32));
            transpose_16x16(out, in_);
            fadst16x16_sse4_1(in_, out,
                              av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32,
                              col_num);
            write_buffer_16x16(out, coeff);
        }
        FLIPADST_FLIPADST => {
            load_buffer_16x16(input, in_, stride, 1, 1, *shift.add(0) as i32);
            fadst16x16_sse4_1(in_, out,
                              av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32,
                              col_num);
            col_txfm_16x16_rounding(out, -(*shift.add(1) as i32));
            transpose_16x16(out, in_);
            fadst16x16_sse4_1(in_, out,
                              av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32,
                              col_num);
            write_buffer_16x16(out, coeff);
        }
        ADST_FLIPADST => {
            load_buffer_16x16(input, in_, stride, 0, 1, *shift.add(0) as i32);
            fadst16x16_sse4_1(in_, out,
                              av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32,
                              col_num);
            col_txfm_16x16_rounding(out, -(*shift.add(1) as i32));
            transpose_16x16(out, in_);
            fadst16x16_sse4_1(in_, out,
                              av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32,
                              col_num);
            write_buffer_16x16(out, coeff);
        }
        FLIPADST_ADST => {
            load_buffer_16x16(input, in_, stride, 1, 0, *shift.add(0) as i32);
            fadst16x16_sse4_1(in_, out,
                              av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32,
                              col_num);
            col_txfm_16x16_rounding(out, -(*shift.add(1) as i32));
            transpose_16x16(out, in_);
            fadst16x16_sse4_1(in_, out,
                              av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32,
                              col_num);
            write_buffer_16x16(out, coeff);
        }
        IDTX => {
            load_buffer_16x16(input, in_, stride, 0, 0, *shift.add(0) as i32);
            idtx16x16_sse4_1(in_, out,
                             av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32,
                             col_num);
            col_txfm_16x16_rounding(out, -(*shift.add(1) as i32));
            transpose_16x16(out, in_);
            idtx16x16_sse4_1(in_, out,
                             av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32,
                             col_num);
            write_buffer_16x16(out, coeff);
        }
        V_DCT => {
            load_buffer_16x16(input, in_, stride, 0, 0, *shift.add(0) as i32);
            fdct16x16_sse4_1(in_, out,
                             av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32,
                             col_num);
            col_txfm_16x16_rounding(out, -(*shift.add(1) as i32));
            transpose_16x16(out, in_);
            idtx16x16_sse4_1(in_, out,
                             av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32,
                             col_num);
            write_buffer_16x16(out, coeff);
        }
        H_DCT => {
            load_buffer_16x16(input, in_, stride, 0, 0, *shift.add(0) as i32);
            idtx16x16_sse4_1(in_, out,
                             av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32,
                             col_num);
            col_txfm_16x16_rounding(out, -(*shift.add(1) as i32));
            transpose_16x16(out, in_);
            fdct16x16_sse4_1(in_, out,
                             av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32,
                             col_num);
            write_buffer_16x16(out, coeff);
        }
        V_ADST => {
            load_buffer_16x16(input, in_, stride, 0, 0, *shift.add(0) as i32);
            fadst16x16_sse4_1(in_, out,
                              av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32,
                              col_num);
            col_txfm_16x16_rounding(out, -(*shift.add(1) as i32));
            transpose_16x16(out, in_);
            idtx16x16_sse4_1(in_, out,
                             av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32,
                             col_num);
            write_buffer_16x16(out, coeff);
        }
        H_ADST => {
            load_buffer_16x16(input, in_, stride, 0, 0, *shift.add(0) as i32);
            idtx16x16_sse4_1(in_, out,
                             av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32,
                             col_num);
            col_txfm_16x16_rounding(out, -(*shift.add(1) as i32));
            transpose_16x16(out, in_);
            fadst16x16_sse4_1(in_, out,
                              av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32,
                              col_num);
            write_buffer_16x16(out, coeff);
        }
        V_FLIPADST => {
            load_buffer_16x16(input, in_, stride, 1, 0, *shift.add(0) as i32);
            fadst16x16_sse4_1(in_, out,
                              av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32,
                              col_num);
            col_txfm_16x16_rounding(out, -(*shift.add(1) as i32));
            transpose_16x16(out, in_);
            idtx16x16_sse4_1(in_, out,
                             av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32,
                             col_num);
            write_buffer_16x16(out, coeff);
        }
        H_FLIPADST => {
            load_buffer_16x16(input, in_, stride, 0, 1, *shift.add(0) as i32);
            idtx16x16_sse4_1(in_, out,
                             av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32,
                             col_num);
            col_txfm_16x16_rounding(out, -(*shift.add(1) as i32));
            transpose_16x16(out, in_);
            fadst16x16_sse4_1(in_, out,
                              av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32,
                              col_num);
            write_buffer_16x16(out, coeff);
        }
        _ => {
            debug_assert!(false);
        }
    }
    let _ = bd;
}

#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn flip_buf_sse4_1(in_: *mut __m128i, out: *mut __m128i, size: i32) {
    for i in (0..size).step_by(2) {
        *in_.offset((30 - i) as isize) = *out.offset(i as isize);
    }
    for i in (1..size).step_by(2) {
        *in_.offset((size - i) as isize) = *out.offset(i as isize);
    }
}

static col_highbd_txfm8x8_arr: [fwd_transform_1d_sse4_1; TX_TYPES] = [
    Some(fdct8x8_sse4_1), // DCT_DCT
    Some(fadst8x8_sse4_1), // ADST_DCT
    Some(fdct8x8_sse4_1), // DCT_ADST
    Some(fadst8x8_sse4_1), // ADST_ADST
    Some(fadst8x8_sse4_1), // FLIPADST_DCT
    Some(fdct8x8_sse4_1), // DCT_FLIPADST
    Some(fadst8x8_sse4_1), // FLIPADST_FLIPADST
    Some(fadst8x8_sse4_1), // ADST_FLIPADST
    Some(fadst8x8_sse4_1), // FLIPADST_ADST
    Some(idtx8x8_sse4_1), // IDTX
    Some(fdct8x8_sse4_1), // V_DCT
    Some(idtx8x8_sse4_1), // H_DCT
    Some(fadst8x8_sse4_1), // V_ADST
    Some(idtx8x8_sse4_1), // H_ADST
    Some(fadst8x8_sse4_1), // V_FLIPADST
    Some(idtx8x8_sse4_1), // H_FLIPADST
];
static row_highbd_txfm32x8_arr: [fwd_transform_1d_sse4_1; TX_TYPES] = [
    Some(fdct8x8_sse4_1), // DCT_DCT
    None, // ADST_DCT
    None, // DCT_ADST
    None, // ADST_ADST
    None, // FLIPADST_DCT
    None, // DCT_FLIPADST
    None, // FLIPADST_FLIPADST
    None, // ADST_FLIPADST
    None, // FLIPADST-ADST
    Some(idtx32x8_sse4_1), // IDTX
    None, // V_DCT
    None, // H_DCT
    None, // V_ADST
    None, // H_ADST
    None, // V_FLIPADST
    None, // H_FLIPADST
];
static col_highbd_txfm4x8_arr: [fwd_transform_1d_sse4_1; TX_TYPES] = [
    Some(fdct4x8_sse4_1), // DCT_DCT
    Some(fadst8x8_sse4_1), // ADST_DCT
    Some(fdct4x8_sse4_1), // DCT_ADST
    Some(fadst8x8_sse4_1), // ADST_ADST
    Some(fadst8x8_sse4_1), // FLIPADST_DCT
    Some(fdct4x8_sse4_1), // DCT_FLIPADST
    Some(fadst8x8_sse4_1), // FLIPADST_FLIPADST
    Some(fadst8x8_sse4_1), // ADST_FLIPADST
    Some(fadst8x8_sse4_1), // FLIPADST_ADST
    Some(idtx8x8_sse4_1), // IDTX
    Some(fdct4x8_sse4_1), // V_DCT
    Some(idtx8x8_sse4_1), // H_DCT
    Some(fadst8x8_sse4_1), // V_ADST
    Some(idtx8x8_sse4_1), // H_ADST
    Some(fadst8x8_sse4_1), // V_FLIPADST
    Some(idtx8x8_sse4_1), // H_FLIPADST
];

static row_highbd_txfm8x16_arr: [fwd_transform_1d_sse4_1; TX_TYPES] = [
    Some(fdct16x16_sse4_1), // DCT_DCT
    Some(fdct16x16_sse4_1), // ADST_DCT
    Some(fadst16x16_sse4_1), // DCT_ADST
    Some(fadst16x16_sse4_1), // ADST_ADST
    Some(fdct16x16_sse4_1), // FLIPADST_DCT
    Some(fadst16x16_sse4_1), // DCT_FLIPADST
    Some(fadst16x16_sse4_1), // FLIPADST_FLIPADST
    Some(fadst16x16_sse4_1), // ADST_FLIPADST
    Some(fadst16x16_sse4_1), // FLIPADST_ADST
    Some(idtx16x16_sse4_1), // IDTX
    Some(idtx16x16_sse4_1), // V_DCT
    Some(fdct16x16_sse4_1), // H_DCT
    Some(idtx16x16_sse4_1), // V_ADST
    Some(fadst16x16_sse4_1), // H_ADST
    Some(idtx16x16_sse4_1), // V_FLIPADST
    Some(fadst16x16_sse4_1), // H_FLIPADST
];

static col_highbd_txfm8x16_arr: [fwd_transform_1d_sse4_1; TX_TYPES] = [
    Some(fdct16x16_sse4_1), // DCT_DCT
    Some(fadst16x16_sse4_1), // ADST_DCT
    Some(fdct16x16_sse4_1), // DCT_ADST
    Some(fadst16x16_sse4_1), // ADST_ADST
    Some(fadst16x16_sse4_1), // FLIPADST_DCT
    Some(fdct16x16_sse4_1), // DCT_FLIPADST
    Some(fadst16x16_sse4_1), // FLIPADST_FLIPADST
    Some(fadst16x16_sse4_1), // ADST_FLIPADST
    Some(fadst16x16_sse4_1), // FLIPADST_ADST
    Some(idtx16x16_sse4_1), // IDTX
    Some(fdct16x16_sse4_1), // V_DCT
    Some(idtx16x16_sse4_1), // H_DCT
    Some(fadst16x16_sse4_1), // V_ADST
    Some(idtx16x16_sse4_1), // H_ADST
    Some(fadst16x16_sse4_1), // V_FLIPADST
    Some(idtx16x16_sse4_1), // H_FLIPADST
];
static row_highbd_txfm8x8_arr: [fwd_transform_1d_sse4_1; TX_TYPES] = [
    Some(fdct8x8_sse4_1), // DCT_DCT
    Some(fdct8x8_sse4_1), // ADST_DCT
    Some(fadst8x8_sse4_1), // DCT_ADST
    Some(fadst8x8_sse4_1), // ADST_ADST
    Some(fdct8x8_sse4_1), // FLIPADST_DCT
    Some(fadst8x8_sse4_1), // DCT_FLIPADST
    Some(fadst8x8_sse4_1), // FLIPADST_FLIPADST
    Some(fadst8x8_sse4_1), // ADST_FLIPADST
    Some(fadst8x8_sse4_1), // FLIPADST_ADST
    Some(idtx8x8_sse4_1), // IDTX
    Some(idtx8x8_sse4_1), // V_DCT
    Some(fdct8x8_sse4_1), // H_DCT
    Some(idtx8x8_sse4_1), // V_ADST
    Some(fadst8x8_sse4_1), // H_ADST
    Some(idtx8x8_sse4_1), // V_FLIPADST
    Some(fadst8x8_sse4_1), // H_FLIPADST
];

static row_highbd_txfm4x8_arr: [fwd_transform_1d_sse4_1; TX_TYPES] = [
    Some(fdct4x8_sse4_1), // DCT_DCT
    Some(fdct4x8_sse4_1), // ADST_DCT
    Some(fadst8x8_sse4_1), // DCT_ADST
    Some(fadst8x8_sse4_1), // ADST_ADST
    Some(fdct4x8_sse4_1), // FLIPADST_DCT
    Some(fadst8x8_sse4_1), // DCT_FLIPADST
    Some(fadst8x8_sse4_1), // FLIPADST_FLIPADST
    Some(fadst8x8_sse4_1), // ADST_FLIPADST
    Some(fadst8x8_sse4_1), // FLIPADST_ADST
    Some(idtx8x8_sse4_1), // IDTX
    Some(idtx8x8_sse4_1), // V_DCT
    Some(fdct4x8_sse4_1), // H_DCT
    Some(idtx8x8_sse4_1), // V_ADST
    Some(fadst8x8_sse4_1), // H_ADST
    Some(idtx8x8_sse4_1), // V_FLIPADST
    Some(fadst8x8_sse4_1), // H_FLIPADST
];

static row_highbd_txfm4x4_arr: [fwd_transform_1d_sse4_1; TX_TYPES] = [
    Some(fdct4x4_sse4_1), // DCT_DCT
    Some(fdct4x4_sse4_1), // ADST_DCT
    Some(fadst4x4_sse4_1), // DCT_ADST
    Some(fadst4x4_sse4_1), // ADST_ADST
    Some(fdct4x4_sse4_1), // FLIPADST_DCT
    Some(fadst4x4_sse4_1), // DCT_FLIPADST
    Some(fadst4x4_sse4_1), // FLIPADST_FLIPADST
    Some(fadst4x4_sse4_1), // ADST_FLIPADST
    Some(fadst4x4_sse4_1), // FLIPADST_ADST
    Some(idtx4x4_sse4_1), // IDTX
    Some(idtx4x4_sse4_1), // V_DCT
    Some(fdct4x4_sse4_1), // H_DCT
    Some(idtx4x4_sse4_1), // V_ADST
    Some(fadst4x4_sse4_1), // H_ADST
    Some(idtx4x4_sse4_1), // V_FLIPADST
    Some(fadst4x4_sse4_1), // H_FLIPADST
];

static col_highbd_txfm4x4_arr: [fwd_transform_1d_sse4_1; TX_TYPES] = [
    Some(fdct4x4_sse4_1), // DCT_DCT
    Some(fadst4x4_sse4_1), // ADST_DCT
    Some(fdct4x4_sse4_1), // DCT_ADST
    Some(fadst4x4_sse4_1), // ADST_ADST
    Some(fadst4x4_sse4_1), // FLIPADST_DCT
    Some(fdct4x4_sse4_1), // DCT_FLIPADST
    Some(fadst4x4_sse4_1), // FLIPADST_FLIPADST
    Some(fadst4x4_sse4_1), // ADST_FLIPADST
    Some(fadst4x4_sse4_1), // FLIPADST_ADST
    Some(idtx4x4_sse4_1), // IDTX
    Some(fdct4x4_sse4_1), // V_DCT
    Some(idtx4x4_sse4_1), // H_DCT
    Some(fadst4x4_sse4_1), // V_ADST
    Some(idtx4x4_sse4_1), // H_ADST
    Some(fadst4x4_sse4_1), // V_FLIPADST
    Some(idtx4x4_sse4_1), // H_FLIPADST
];

static col_highbd_txfm8x32_arr: [fwd_transform_1d_sse4_1; TX_TYPES] = [
    Some(av1_fdct32_sse4_1), // DCT_DCT
    None, // ADST_DCT
    None, // DCT_ADST
    None, // ADST_ADST
    None, // FLIPADST_DCT
    None, // DCT_FLIPADST
    None, // FLIPADST_FLIPADST
    None, // ADST_FLIPADST
    None, // FLIPADST_ADST
    Some(av1_idtx32_sse4_1), // IDTX
    None, // V_DCT
    None, // H_DCT
    None, // V_ADST
    None, // H_ADST
    None, // V_FLIPADST
    None, // H_FLIPADST
];

static row_highbd_txfm8x32_arr: [fwd_transform_1d_sse4_1; TX_TYPES] = [
    Some(fdct16x16_sse4_1), // DCT_DCT
    None, // ADST_DCT
    None, // DCT_ADST
    None, // ADST_ADST
    None, // FLIPADST_DCT
    None, // DCT_FLIPADST
    None, // FLIPADST_FLIPADST
    None, // ADST_FLIPADST
    None, // FLIPADST_ADST
    Some(idtx16x16_sse4_1), // IDTX
    None, // V_DCT
    None, // H_DCT
    None, // V_ADST
    None, // H_ADST
    None, // V_FLIPADST
    None, // H_FLIPADST
];

#[no_mangle]
#[target_feature(enable = "sse4.1")]
pub unsafe extern "C-unwind" fn av1_fwd_txfm2d_16x8_sse4_1(input: *const i16, coeff: *mut i32,
                                                           stride: i32, tx_type: TX_TYPE, bd: i32) {
    let mut in__: [__m128i; 32] = core::mem::zeroed();
    let in_: *mut __m128i = in__.as_mut_ptr();
    let mut out_: [__m128i; 32] = core::mem::zeroed();
    let out: *mut __m128i = out_.as_mut_ptr();
    let shift: *const i8 = av1_fwd_txfm_shift_ls[TX_16X8 as usize];
    let txw_idx: i32 = get_txw_idx(TX_16X8 as TX_SIZE);
    let txh_idx: i32 = get_txh_idx(TX_16X8 as TX_SIZE);
    let col_txfm: fwd_transform_1d_sse4_1 = col_highbd_txfm8x8_arr[tx_type as usize];
    let row_txfm: fwd_transform_1d_sse4_1 = row_highbd_txfm8x16_arr[tx_type as usize];
    let bit: i32 = av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32;
    let mut ud_flip: i32 = 0;
    let mut lr_flip: i32 = 0;
    get_flip_cfg(tx_type, &mut ud_flip, &mut lr_flip);

    for i in 0..2 {
        load_buffer_8x8(input.offset((i * 8) as isize), in_, stride, ud_flip, 0,
                        *shift.add(0) as i32);
        col_txfm.unwrap()(in_, in_, bit, 2);
        col_txfm_8x8_rounding(in_, -(*shift.add(1) as i32));
        transpose_8x8(in_, out.offset((i * 16) as isize));
    }

    if lr_flip != 0 {
        flip_buf_sse4_1(in_, out, 32);
        row_txfm.unwrap()(in_, out, bit, 2);
    } else {
        row_txfm.unwrap()(out, out, bit, 2);
    }

    for i in 0..2 {
        av1_round_shift_rect_array_32_sse4_1(out.offset((i * 16) as isize), in_, 16,
                                             -(*shift.add(2) as i32), NewSqrt2);
        write_buffer_8x8(in_, coeff.offset((i * 64) as isize));
    }
    let _ = bd;
}

#[no_mangle]
#[target_feature(enable = "sse4.1")]
pub unsafe extern "C-unwind" fn av1_fwd_txfm2d_8x16_sse4_1(input: *const i16, coeff: *mut i32,
                                                           stride: i32, tx_type: TX_TYPE, bd: i32) {
    let mut in__: [__m128i; 32] = core::mem::zeroed();
    let in_: *mut __m128i = in__.as_mut_ptr();
    let mut out_: [__m128i; 32] = core::mem::zeroed();
    let out: *mut __m128i = out_.as_mut_ptr();
    let shift: *const i8 = av1_fwd_txfm_shift_ls[TX_8X16 as usize];
    let txw_idx: i32 = get_txw_idx(TX_8X16 as TX_SIZE);
    let txh_idx: i32 = get_txh_idx(TX_8X16 as TX_SIZE);
    let col_txfm: fwd_transform_1d_sse4_1 = col_highbd_txfm8x16_arr[tx_type as usize];
    let row_txfm: fwd_transform_1d_sse4_1 = row_highbd_txfm8x8_arr[tx_type as usize];
    let bit: i32 = av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32;
    let mut ud_flip: i32 = 0;
    let mut lr_flip: i32 = 0;
    get_flip_cfg(tx_type, &mut ud_flip, &mut lr_flip);

    load_buffer_8x16(input, in_, stride, ud_flip, lr_flip, *shift.add(0) as i32);
    col_txfm.unwrap()(in_, in_, bit, 2);
    col_txfm_8x16_rounding(in_, -(*shift.add(1) as i32));
    transpose_8x8(in_, out);
    transpose_8x8(in_.add(16), out.add(16));

    for i in 0..2 {
        row_txfm.unwrap()(out.offset((i * 16) as isize), out, bit, 2);
        av1_round_shift_rect_array_32_sse4_1(out, out, 16, -(*shift.add(2) as i32), NewSqrt2);
        write_buffer_16x8(out, coeff.offset((i * 8) as isize), 16);
    }
    let _ = bd;
}

#[no_mangle]
#[target_feature(enable = "sse4.1")]
pub unsafe extern "C-unwind" fn av1_fwd_txfm2d_4x16_sse4_1(input: *const i16, coeff: *mut i32,
                                                           stride: i32, tx_type: TX_TYPE, bd: i32) {
    let mut in__: [__m128i; 16] = core::mem::zeroed();
    let in_: *mut __m128i = in__.as_mut_ptr();
    let outcoeff128: *mut __m128i = coeff as *mut __m128i;
    let shift: *const i8 = av1_fwd_txfm_shift_ls[TX_4X16 as usize];
    let txw_idx: i32 = get_txw_idx(TX_4X16 as TX_SIZE);
    let txh_idx: i32 = get_txh_idx(TX_4X16 as TX_SIZE);
    let txfm_size_col: i32 = tx_size_wide[TX_4X16 as usize];
    let txfm_size_row: i32 = tx_size_high[TX_4X16 as usize];
    let bitcol: i32 = av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32;
    let bitrow: i32 = av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32;
    let col_txfm: fwd_transform_1d_sse4_1 = col_highbd_txfm8x16_arr[tx_type as usize];
    let row_txfm: fwd_transform_1d_sse4_1 = row_highbd_txfm4x4_arr[tx_type as usize];

    let mut ud_flip: i32 = 0;
    let mut lr_flip: i32 = 0;
    get_flip_cfg(tx_type, &mut ud_flip, &mut lr_flip);
    // col transform
    load_buffer_4x16(input, in_, stride, ud_flip, lr_flip, *shift.add(0) as i32);
    col_txfm.unwrap()(in_, outcoeff128, bitcol, 1);
    col_txfm_8x8_rounding(outcoeff128, -(*shift.add(1) as i32));
    transpose_8nx8n(outcoeff128, in_, txfm_size_col, txfm_size_row);

    // row transform
    for i in 0..4 {
        let mut tmp_: [__m128i; 4] = core::mem::zeroed();
        let tmp: *mut __m128i = tmp_.as_mut_ptr();
        row_txfm.unwrap()(in_.offset(i as isize), tmp, bitrow, txfm_size_row >> 2);
        store_output_w4(coeff.offset((i * 4) as isize), tmp, txfm_size_row, txfm_size_col);
    }
    let _ = bd;
}

#[no_mangle]
#[target_feature(enable = "sse4.1")]
pub unsafe extern "C-unwind" fn av1_fwd_txfm2d_16x4_sse4_1(input: *const i16, coeff: *mut i32,
                                                           stride: i32, tx_type: TX_TYPE, bd: i32) {
    let mut in__: [__m128i; 16] = core::mem::zeroed();
    let in_: *mut __m128i = in__.as_mut_ptr();
    let outcoeff128: *mut __m128i = coeff as *mut __m128i;
    let shift: *const i8 = av1_fwd_txfm_shift_ls[TX_16X4 as usize];
    let txw_idx: i32 = get_txw_idx(TX_16X4 as TX_SIZE);
    let txh_idx: i32 = get_txh_idx(TX_16X4 as TX_SIZE);
    let txfm_size_col: i32 = tx_size_wide[TX_16X4 as usize];
    let txfm_size_row: i32 = tx_size_high[TX_16X4 as usize];
    let bitcol: i32 = av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32;
    let bitrow: i32 = av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32;
    let col_txfm: fwd_transform_1d_sse4_1 = col_highbd_txfm4x4_arr[tx_type as usize];
    let row_txfm: fwd_transform_1d_sse4_1 = row_highbd_txfm8x16_arr[tx_type as usize];
    let mut ud_flip: i32 = 0;
    let mut lr_flip: i32 = 0;
    get_flip_cfg(tx_type, &mut ud_flip, &mut lr_flip);

    // col transform
    load_buffer_16x4(input, in_, stride, ud_flip, lr_flip, *shift.add(0) as i32);

    for i in 0..(txfm_size_col >> 2) {
        let cur_in: *mut __m128i = in_.offset((i * txfm_size_row) as isize);
        col_txfm.unwrap()(cur_in, cur_in, bitcol, 1);
        transpose_32bit_4x4(cur_in, cur_in);
    }
    col_txfm_8x8_rounding(in_, -(*shift.add(1) as i32));

    // row transform
    row_txfm.unwrap()(in_, outcoeff128, bitrow, 1);
    let _ = bd;
}

#[no_mangle]
#[target_feature(enable = "sse4.1")]
pub unsafe extern "C-unwind" fn av1_fwd_txfm2d_16x32_sse4_1(input: *const i16, coeff: *mut i32,
                                                            stride: i32, tx_type: TX_TYPE,
                                                            bd: i32) {
    let mut in__: [__m128i; 128] = core::mem::zeroed();
    let in_: *mut __m128i = in__.as_mut_ptr();
    let outcoef128: *mut __m128i = coeff as *mut __m128i;
    let shift: *const i8 = av1_fwd_txfm_shift_ls[TX_16X32 as usize];
    let txw_idx: i32 = get_txw_idx(TX_16X32 as TX_SIZE);
    let txh_idx: i32 = get_txh_idx(TX_16X32 as TX_SIZE);
    let col_txfm: fwd_transform_1d_sse4_1 = col_highbd_txfm8x32_arr[tx_type as usize];
    let row_txfm: fwd_transform_1d_sse4_1 = row_highbd_txfm8x32_arr[tx_type as usize];
    let bitcol: i32 = av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32;
    let bitrow: i32 = av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32;

    // column transform
    load_buffer_16x16(input, in_, stride, 0, 0, *shift.add(0) as i32);
    load_buffer_16x16(input.offset((16 * stride) as isize), in_.add(64), stride, 0, 0,
                      *shift.add(0) as i32);

    for i in 0..4 {
        col_txfm.unwrap()(in_.offset(i as isize), in_.offset(i as isize), bitcol, 4);
    }
    col_txfm_16x16_rounding(in_.add(0), -(*shift.add(1) as i32));
    col_txfm_16x16_rounding(in_.add(64), -(*shift.add(1) as i32));
    transpose_8nx8n(in_, outcoef128, 16, 32);

    // row transform
    row_txfm.unwrap()(outcoef128, in_, bitrow, 8);
    av1_round_shift_rect_array_32_sse4_1(in_, outcoef128, 128, -(*shift.add(2) as i32), NewSqrt2);
    let _ = bd;
}

#[no_mangle]
#[target_feature(enable = "sse4.1")]
pub unsafe extern "C-unwind" fn av1_fwd_txfm2d_32x64_sse4_1(input: *const i16, coeff: *mut i32,
                                                            stride: i32, tx_type: TX_TYPE,
                                                            bd: i32) {
    let _ = tx_type;
    let mut in__: [__m128i; 512] = core::mem::zeroed();
    let in_: *mut __m128i = in__.as_mut_ptr();
    let outcoef128: *mut __m128i = coeff as *mut __m128i;
    let shift: *const i8 = av1_fwd_txfm_shift_ls[TX_32X64 as usize];
    let txw_idx: i32 = get_txw_idx(TX_32X64 as TX_SIZE);
    let txh_idx: i32 = get_txh_idx(TX_32X64 as TX_SIZE);
    let txfm_size_col: i32 = tx_size_wide[TX_32X64 as usize];
    let txfm_size_row: i32 = tx_size_high[TX_32X64 as usize];
    let bitcol: i32 = av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32;
    let bitrow: i32 = av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32;
    let num_row: i32 = txfm_size_row >> 2;
    let num_col: i32 = txfm_size_col >> 2;

    // column transform
    load_buffer_32x8n(input, in_, stride, 0, 0, *shift.add(0) as i32, txfm_size_row);
    for i in 0..num_col {
        av1_fdct64_sse4_1(in_.offset(i as isize), in_.offset(i as isize), bitcol as i8, num_col,
                          num_col);
    }
    for i in 0..num_col {
        col_txfm_16x16_rounding(in_.offset((i * txfm_size_row) as isize), -(*shift.add(1) as i32));
    }
    transpose_8nx8n(in_, outcoef128, txfm_size_col, txfm_size_row);

    // row transform
    for i in 0..num_row {
        av1_fdct32_sse4_1(outcoef128.offset(i as isize), in_.offset(i as isize), bitrow, num_row);
    }
    for i in 0..txfm_size_col {
        av1_round_shift_rect_array_32_sse4_1(in_.offset((i * 16) as isize),
                                             outcoef128.offset((i * 8) as isize), 8,
                                             -(*shift.add(2) as i32), NewSqrt2);
    }
    let _ = bd;
}

#[no_mangle]
#[target_feature(enable = "sse4.1")]
pub unsafe extern "C-unwind" fn av1_fwd_txfm2d_64x32_sse4_1(input: *const i16, coeff: *mut i32,
                                                            stride: i32, tx_type: TX_TYPE,
                                                            bd: i32) {
    let _ = tx_type;
    let mut in__: [__m128i; 512] = core::mem::zeroed();
    let in_: *mut __m128i = in__.as_mut_ptr();
    let outcoef128: *mut __m128i = coeff as *mut __m128i;
    let shift: *const i8 = av1_fwd_txfm_shift_ls[TX_64X32 as usize];
    let txw_idx: i32 = get_txw_idx(TX_64X32 as TX_SIZE);
    let txh_idx: i32 = get_txh_idx(TX_64X32 as TX_SIZE);
    let txfm_size_col: i32 = tx_size_wide[TX_64X32 as usize];
    let txfm_size_row: i32 = tx_size_high[TX_64X32 as usize];
    let bitcol: i32 = av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32;
    let bitrow: i32 = av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32;
    let num_row: i32 = txfm_size_row >> 2;
    let num_col: i32 = txfm_size_col >> 2;

    // column transform
    for i in 0..32 {
        load_buffer_4x4(input.add(0).offset((i * stride) as isize),
                        in_.add(0).offset((i * 16) as isize), 4, 0, 0, *shift.add(0) as i32);
        load_buffer_4x4(input.add(16).offset((i * stride) as isize),
                        in_.add(4).offset((i * 16) as isize), 4, 0, 0, *shift.add(0) as i32);
        load_buffer_4x4(input.add(32).offset((i * stride) as isize),
                        in_.add(8).offset((i * 16) as isize), 4, 0, 0, *shift.add(0) as i32);
        load_buffer_4x4(input.add(48).offset((i * stride) as isize),
                        in_.add(12).offset((i * 16) as isize), 4, 0, 0, *shift.add(0) as i32);
    }

    for i in 0..num_col {
        av1_fdct32_sse4_1(in_.offset(i as isize), in_.offset(i as isize), bitcol, num_col);
    }

    for i in 0..num_row {
        col_txfm_16x16_rounding(in_.offset((i * txfm_size_col) as isize), -(*shift.add(1) as i32));
    }
    transpose_8nx8n(in_, outcoef128, txfm_size_col, txfm_size_row);

    // row transform
    for i in 0..num_row {
        av1_fdct64_sse4_1(outcoef128.offset(i as isize), in_.offset(i as isize), bitrow as i8,
                          num_row, num_row);
    }
    av1_round_shift_rect_array_32_sse4_1(in_, outcoef128, 512, -(*shift.add(2) as i32), NewSqrt2);
    let _ = bd;
}

#[no_mangle]
#[target_feature(enable = "sse4.1")]
pub unsafe extern "C-unwind" fn av1_fwd_txfm2d_32x16_sse4_1(input: *const i16, coeff: *mut i32,
                                                            stride: i32, tx_type: TX_TYPE,
                                                            bd: i32) {
    let mut in__: [__m128i; 128] = core::mem::zeroed();
    let in_: *mut __m128i = in__.as_mut_ptr();
    let outcoef128: *mut __m128i = coeff as *mut __m128i;
    let shift: *const i8 = av1_fwd_txfm_shift_ls[TX_32X16 as usize];
    let txw_idx: i32 = get_txw_idx(TX_32X16 as TX_SIZE);
    let txh_idx: i32 = get_txh_idx(TX_32X16 as TX_SIZE);
    let col_txfm: fwd_transform_1d_sse4_1 = row_highbd_txfm8x32_arr[tx_type as usize];
    let row_txfm: fwd_transform_1d_sse4_1 = col_highbd_txfm8x32_arr[tx_type as usize];
    let bitcol: i32 = av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32;
    let bitrow: i32 = av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32;

    // column transform
    load_buffer_32x8n(input, in_, stride, 0, 0, *shift.add(0) as i32, 16);
    col_txfm.unwrap()(in_, in_, bitcol, 8);
    col_txfm_16x16_rounding(in_.add(0), -(*shift.add(1) as i32));
    col_txfm_16x16_rounding(in_.add(64), -(*shift.add(1) as i32));
    transpose_8nx8n(in_, outcoef128, 32, 16);

    // row transform
    for i in 0..4 {
        row_txfm.unwrap()(outcoef128.offset(i as isize), in_.offset(i as isize), bitrow, 4);
    }
    av1_round_shift_rect_array_32_sse4_1(in_, outcoef128, 128, -(*shift.add(2) as i32), NewSqrt2);
    let _ = bd;
}

#[no_mangle]
#[target_feature(enable = "sse4.1")]
pub unsafe extern "C-unwind" fn av1_fwd_txfm2d_8x32_sse4_1(input: *const i16, coeff: *mut i32,
                                                           stride: i32, tx_type: TX_TYPE, bd: i32) {
    let mut in__: [__m128i; 64] = core::mem::zeroed();
    let in_: *mut __m128i = in__.as_mut_ptr();
    let outcoef128: *mut __m128i = coeff as *mut __m128i;
    let shift: *const i8 = av1_fwd_txfm_shift_ls[TX_8X32 as usize];
    let txw_idx: i32 = get_txw_idx(TX_8X32 as TX_SIZE);
    let txh_idx: i32 = get_txh_idx(TX_8X32 as TX_SIZE);
    let col_txfm: fwd_transform_1d_sse4_1 = col_highbd_txfm8x32_arr[tx_type as usize];
    let row_txfm: fwd_transform_1d_sse4_1 = row_highbd_txfm32x8_arr[tx_type as usize];
    let bitcol: i32 = av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32;
    let bitrow: i32 = av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32;

    let txfm_size_col: i32 = tx_size_wide[TX_8X32 as usize];
    let txfm_size_row: i32 = tx_size_high[TX_8X32 as usize];
    let num_col: i32 = txfm_size_col >> 2;

    // column transform
    load_buffer_8x16(input, in_, stride, 0, 0, *shift.add(0) as i32);
    load_buffer_8x16(input.offset(((txfm_size_row >> 1) * stride) as isize),
                     in_.offset(txfm_size_row as isize), stride, 0, 0, *shift.add(0) as i32);

    for i in 0..num_col {
        col_txfm.unwrap()(in_.offset(i as isize), in_.offset(i as isize), bitcol, num_col);
    }
    col_txfm_16x16_rounding(in_, -(*shift.add(1) as i32));
    transpose_8nx8n(in_, outcoef128, txfm_size_col, txfm_size_row);

    // row transform
    for i in (0..txfm_size_col).step_by(2) {
        row_txfm.unwrap()(outcoef128.offset(i as isize), outcoef128.offset(i as isize), bitrow,
                          txfm_size_col);
    }
    let _ = bd;
}

#[no_mangle]
#[target_feature(enable = "sse4.1")]
pub unsafe extern "C-unwind" fn av1_fwd_txfm2d_32x8_sse4_1(input: *const i16, coeff: *mut i32,
                                                           stride: i32, tx_type: TX_TYPE, bd: i32) {
    let mut in__: [__m128i; 64] = core::mem::zeroed();
    let in_: *mut __m128i = in__.as_mut_ptr();
    let outcoef128: *mut __m128i = coeff as *mut __m128i;
    let shift: *const i8 = av1_fwd_txfm_shift_ls[TX_32X8 as usize];
    let txw_idx: i32 = get_txw_idx(TX_32X8 as TX_SIZE);
    let txh_idx: i32 = get_txh_idx(TX_32X8 as TX_SIZE);
    let col_txfm: fwd_transform_1d_sse4_1 = row_highbd_txfm32x8_arr[tx_type as usize];
    let row_txfm: fwd_transform_1d_sse4_1 = col_highbd_txfm8x32_arr[tx_type as usize];
    let bitcol: i32 = av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32;
    let bitrow: i32 = av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32;

    let txfm_size_col: i32 = tx_size_wide[TX_32X8 as usize];
    let txfm_size_row: i32 = tx_size_high[TX_32X8 as usize];
    let num_col: i32 = txfm_size_row >> 2;

    // column transform
    load_buffer_32x8n(input, in_, stride, 0, 0, *shift.add(0) as i32, 8);
    for i in (0..txfm_size_row).step_by(2) {
        col_txfm.unwrap()(in_.offset(i as isize), in_.offset(i as isize), bitcol, txfm_size_row);
    }

    col_txfm_16x16_rounding(in_.add(0), -(*shift.add(1) as i32));
    transpose_8nx8n(in_, outcoef128, txfm_size_col, txfm_size_row);

    // row transform
    for i in 0..num_col {
        row_txfm.unwrap()(outcoef128.offset(i as isize), outcoef128.offset(i as isize), bitrow,
                          num_col);
    }
    let _ = bd;
}

#[no_mangle]
#[target_feature(enable = "sse4.1")]
pub unsafe extern "C-unwind" fn av1_fwd_txfm2d_4x8_sse4_1(input: *const i16, coeff: *mut i32,
                                                          stride: i32, tx_type: TX_TYPE, bd: i32) {
    let mut in__: [__m128i; 8] = core::mem::zeroed();
    let in_: *mut __m128i = in__.as_mut_ptr();
    let shift: *const i8 = av1_fwd_txfm_shift_ls[TX_4X8 as usize];
    let txw_idx: i32 = get_txw_idx(TX_4X8 as TX_SIZE);
    let txh_idx: i32 = get_txh_idx(TX_4X8 as TX_SIZE);
    let txfm_size_col: i32 = tx_size_wide[TX_4X8 as usize];
    let txfm_size_row: i32 = tx_size_high[TX_4X8 as usize];
    let bitcol: i32 = av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32;
    let bitrow: i32 = av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32;
    let col_txfm: fwd_transform_1d_sse4_1 = col_highbd_txfm4x8_arr[tx_type as usize];
    let row_txfm: fwd_transform_1d_sse4_1 = row_highbd_txfm4x4_arr[tx_type as usize];

    let mut ud_flip: i32 = 0;
    let mut lr_flip: i32 = 0;
    get_flip_cfg(tx_type, &mut ud_flip, &mut lr_flip);

    load_buffer_4x8(input, in_, stride, ud_flip, lr_flip, *shift.add(0) as i32);
    col_txfm.unwrap()(in_, in_, bitcol, 1);
    col_txfm_4x8_rounding(in_, -(*shift.add(1) as i32));

    for i in 0..2 {
        let cur_in: *mut __m128i = in_.offset((i * 4) as isize);
        transpose_32bit_4x4(cur_in, cur_in);
        row_txfm.unwrap()(cur_in, cur_in, bitrow, 1);
        av1_round_shift_rect_array_32_sse4_1(cur_in, cur_in, txfm_size_col, -(*shift.add(2) as i32),
                                             NewSqrt2);
        store_output_w4(coeff.offset((i * 4) as isize), cur_in, txfm_size_row, 4);
    }
    let _ = bd;
}

#[no_mangle]
#[target_feature(enable = "sse4.1")]
pub unsafe extern "C-unwind" fn av1_fwd_txfm2d_8x4_sse4_1(input: *const i16, coeff: *mut i32,
                                                          stride: i32, tx_type: TX_TYPE, bd: i32) {
    let mut in__: [__m128i; 8] = core::mem::zeroed();
    let in_: *mut __m128i = in__.as_mut_ptr();
    let outcoeff128: *mut __m128i = coeff as *mut __m128i;
    let shift: *const i8 = av1_fwd_txfm_shift_ls[TX_8X4 as usize];
    let txw_idx: i32 = get_txw_idx(TX_8X4 as TX_SIZE);
    let txh_idx: i32 = get_txh_idx(TX_8X4 as TX_SIZE);
    let txfm_size_col: i32 = tx_size_wide[TX_8X4 as usize];
    let txfm_size_row: i32 = tx_size_high[TX_8X4 as usize];
    let bitcol: i32 = av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32;
    let bitrow: i32 = av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32;
    let col_txfm: fwd_transform_1d_sse4_1 = col_highbd_txfm4x4_arr[tx_type as usize];
    let row_txfm: fwd_transform_1d_sse4_1 = row_highbd_txfm4x8_arr[tx_type as usize];
    let mut ud_flip: i32 = 0;
    let mut lr_flip: i32 = 0;
    get_flip_cfg(tx_type, &mut ud_flip, &mut lr_flip);
    // col tranform
    load_buffer_8x4(input, in_, stride, ud_flip, lr_flip, *shift.add(0) as i32);
    for i in 0..2 {
        let cur_in: *mut __m128i = in_.offset((i * txfm_size_row) as isize);
        col_txfm.unwrap()(cur_in, cur_in, bitcol, 1);
        transpose_32bit_4x4(cur_in, cur_in);
    }
    col_txfm_4x8_rounding(in_, -(*shift.add(1) as i32));

    // row tranform
    row_txfm.unwrap()(in_, outcoeff128, bitrow, 1);
    av1_round_shift_rect_array_32_sse4_1(outcoeff128, outcoeff128, txfm_size_col,
                                         -(*shift.add(2) as i32), NewSqrt2);
    let _ = bd;
}

#[no_mangle]
#[target_feature(enable = "sse4.1")]
pub unsafe extern "C-unwind" fn av1_fwd_txfm2d_16x64_sse4_1(input: *const i16, coeff: *mut i32,
                                                            stride: i32, tx_type: TX_TYPE,
                                                            bd: i32) {
    let mut in__: [__m128i; 256] = core::mem::zeroed();
    let in_: *mut __m128i = in__.as_mut_ptr();
    let outcoeff128: *mut __m128i = coeff as *mut __m128i;
    let shift: *const i8 = av1_fwd_txfm_shift_ls[TX_16X64 as usize];
    let txw_idx: i32 = get_txw_idx(TX_16X64 as TX_SIZE);
    let txh_idx: i32 = get_txh_idx(TX_16X64 as TX_SIZE);
    let txfm_size_col: i32 = tx_size_wide[TX_16X64 as usize];
    let txfm_size_row: i32 = tx_size_high[TX_16X64 as usize];
    let bitcol: i32 = av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32;
    let bitrow: i32 = av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32;
    let mut ud_flip: i32 = 0;
    let mut lr_flip: i32 = 0;
    get_flip_cfg(tx_type, &mut ud_flip, &mut lr_flip);
    let num_col: i32 = txfm_size_col >> 2;
    // col tranform
    {
        let mut i: i32 = 0;
        while i < txfm_size_row {
            load_buffer_4x4(input.offset(((i + 0) * stride) as isize),
                            in_.offset(((i + 0) * num_col) as isize), num_col, ud_flip, lr_flip,
                            *shift.add(0) as i32);
            load_buffer_4x4(input.offset(((i + 1) * stride) as isize),
                            in_.offset(((i + 1) * num_col) as isize), num_col, ud_flip, lr_flip,
                            *shift.add(0) as i32);
            load_buffer_4x4(input.offset(((i + 2) * stride) as isize),
                            in_.offset(((i + 2) * num_col) as isize), num_col, ud_flip, lr_flip,
                            *shift.add(0) as i32);
            load_buffer_4x4(input.offset(((i + 3) * stride) as isize),
                            in_.offset(((i + 3) * num_col) as isize), num_col, ud_flip, lr_flip,
                            *shift.add(0) as i32);
            i += num_col;
        }
    }

    for i in 0..num_col {
        av1_fdct64_sse4_1(in_.offset(i as isize), outcoeff128.offset(i as isize), bitcol as i8,
                          num_col, num_col);
    }

    col_txfm_16x16_rounding(outcoeff128, -(*shift.add(1) as i32));
    col_txfm_16x16_rounding(outcoeff128.add(64), -(*shift.add(1) as i32));
    col_txfm_16x16_rounding(outcoeff128.add(128), -(*shift.add(1) as i32));
    col_txfm_16x16_rounding(outcoeff128.add(192), -(*shift.add(1) as i32));

    transpose_8nx8n(outcoeff128, in_, txfm_size_col, 32);
    fdct16x16_sse4_1(in_, outcoeff128, bitrow, 8);
    let _ = bd;
}

#[no_mangle]
#[target_feature(enable = "sse4.1")]
pub unsafe extern "C-unwind" fn av1_fwd_txfm2d_64x16_sse4_1(input: *const i16, coeff: *mut i32,
                                                            stride: i32, tx_type: TX_TYPE,
                                                            bd: i32) {
    let mut in__: [__m128i; 256] = core::mem::zeroed();
    let in_: *mut __m128i = in__.as_mut_ptr();
    let outcoeff128: *mut __m128i = coeff as *mut __m128i;
    let shift: *const i8 = av1_fwd_txfm_shift_ls[TX_64X16 as usize];
    let txw_idx: i32 = get_txw_idx(TX_64X16 as TX_SIZE);
    let txh_idx: i32 = get_txh_idx(TX_64X16 as TX_SIZE);
    let txfm_size_col: i32 = tx_size_wide[TX_64X16 as usize];
    let txfm_size_row: i32 = tx_size_high[TX_64X16 as usize];
    let bitcol: i32 = av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32;
    let bitrow: i32 = av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32;
    let mut ud_flip: i32 = 0;
    let mut lr_flip: i32 = 0;
    get_flip_cfg(tx_type, &mut ud_flip, &mut lr_flip);
    // col tranform
    for i in 0..txfm_size_row {
        load_buffer_4x4(input.add(0).offset((i * stride) as isize),
                        in_.add(0).offset((i * txfm_size_row) as isize), 4, ud_flip, lr_flip,
                        *shift.add(0) as i32);
        load_buffer_4x4(input.add(16).offset((i * stride) as isize),
                        in_.add(4).offset((i * txfm_size_row) as isize), 4, ud_flip, lr_flip,
                        *shift.add(0) as i32);
        load_buffer_4x4(input.add(32).offset((i * stride) as isize),
                        in_.add(8).offset((i * txfm_size_row) as isize), 4, ud_flip, lr_flip,
                        *shift.add(0) as i32);
        load_buffer_4x4(input.add(48).offset((i * stride) as isize),
                        in_.add(12).offset((i * txfm_size_row) as isize), 4, ud_flip, lr_flip,
                        *shift.add(0) as i32);
    }

    fdct16x16_sse4_1(in_, outcoeff128, bitcol, txfm_size_row);
    col_txfm_16x16_rounding(outcoeff128, -(*shift.add(1) as i32));
    col_txfm_16x16_rounding(outcoeff128.add(64), -(*shift.add(1) as i32));
    col_txfm_16x16_rounding(outcoeff128.add(128), -(*shift.add(1) as i32));
    col_txfm_16x16_rounding(outcoeff128.add(192), -(*shift.add(1) as i32));

    transpose_8nx8n(outcoeff128, in_, txfm_size_col, txfm_size_row);
    for i in 0..4 {
        av1_fdct64_sse4_1(in_.offset(i as isize), outcoeff128.offset(i as isize), bitrow as i8, 4,
                          4);
    }
    core::ptr::write_bytes(coeff.offset((txfm_size_row * 32) as isize), 0,
                           (txfm_size_row * 32) as usize);
    let _ = bd;
}
