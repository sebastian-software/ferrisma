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

// Translated from the high-bitdepth part of
// upstream/aom/av1/encoder/x86/av1_fwd_txfm2d_sse4.c (ADR 0056, ADR 0058).
// Upstream compiles this file with -msse4.1. Function names, helper structure,
// and intrinsic sequences follow the original; every function carries
// `#[target_feature(enable = "sse4.1")]`.
// The straight-line function bodies were transliterated with statement-
// rewriting scripts (not C2Rust) and then compared with the C function by
// function: identifiers, constants and operators in order (ADR 0058).
//
// Translation notes:
// - Only av1_fwd_txfm2d_32x32_sse4_1 and av1_fwd_txfm2d_64x64_sse4_1 and the
//   helpers they use are translated. The rest of the file (the low-bitdepth
//   lowbd_fwd_txfm2d_*_sse4_1 functions and the RTCD function
//   av1_lowbd_fwd_txfm_sse4_1, which depend on av1_fwd_txfm_sse2.c) is outside
//   the port's kernel set, as av1_lowbd_fwd_txfm_neon is on Arm.
// - TxfmFuncSSE2 becomes an `Option<unsafe fn ..>` alias; fwd_txfm_type_to_func
//   returns None where C returns NULL after `assert(0)`, and the caller's
//   unconditional call becomes `.unwrap()`.
// - TXFM_2D_FLIP_CFG and av1_get_fwd_txfm_cfg come from the translated C
//   (av1/encoder/av1_fwd_txfm2d.c). The uninitialized C `cfg` and
//   DECLARE_ALIGNED buffers are zero-initialized; av1_get_fwd_txfm_cfg fills
//   every field read here, and the buffers are written before they are read.
// - The int8_t shift entries are promoted to `int` before negation, as in C.
// - Array parameters stay raw pointers, so the in-place
//   av1_round_shift_array_32_sse4_1(out_128, out_128, ...) keeps the C
//   aliasing.
#![allow(clippy::all)]

use core::arch::x86_64::*;

use crate::av1__encoder__av1_fwd_txfm2d::{av1_get_fwd_txfm_cfg, TXFM_2D_FLIP_CFG};
use crate::simd::av1__common__x86__av1_txfm_sse4::av1_round_shift_array_32_sse4_1;
use crate::simd::av1__encoder__x86__av1_txfm1d_sse4::{
    av1_fdct32_sse4_1, av1_fdct64_sse4_1, av1_idtx32_sse4_1, transpose_32,
};

// From aom_dsp/txfm_common.h and av1/common/enums.h.
pub type TX_TYPE = u8;
type TX_SIZE = u8;
const TX_32X32: TX_SIZE = 3;
const TX_64X64: TX_SIZE = 4;
const TX_SIZES: TX_SIZE = 5;

// From av1/common/av1_txfm.h.
type TXFM_TYPE = u8;
const TXFM_TYPE_DCT32: TXFM_TYPE = 3;
const TXFM_TYPE_DCT64: TXFM_TYPE = 4;
const TXFM_TYPE_IDENTITY32: TXFM_TYPE = 11;

// From av1/common/common_data.h.
const tx_size_wide: [i32; 19] = [
    4, 8, 16, 32, 64, 4, 8, 8, 16, 16, 32, 32, 64, 4, 16, 8, 32, 16, 64,
];

#[repr(C, align(16))]
struct Aligned16<T>(T);

#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn int16_array_with_stride_to_int32_array_without_stride(
    input: *const i16,
    stride: i32,
    output: *mut i32,
    txfm1d_size: i32,
) {
    let mut r: i32;
    let mut c: i32;
    r = 0;
    while r < txfm1d_size {
        c = 0;
        while c < txfm1d_size {
            *output.offset((r * txfm1d_size + c) as isize) =
                *input.offset((r * stride + c) as isize) as i32;
            c += 1;
        }
        r += 1;
    }
}

type TxfmFuncSSE2 = Option<unsafe fn(*mut __m128i, *mut __m128i, i8, *const i8)>;

#[target_feature(enable = "sse4.1")]
unsafe fn fdct32_sse4_1(
    input: *mut __m128i,
    output: *mut __m128i,
    cos_bit: i8,
    stage_range: *const i8,
) {
    let txfm_size: i32 = 32;
    let num_per_128: i32 = 4;
    let col_num: i32 = txfm_size / num_per_128;
    let mut col: i32;
    let _ = stage_range;
    col = 0;
    while col < col_num {
        av1_fdct32_sse4_1(
            input.offset(col as isize),
            output.offset(col as isize),
            cos_bit as i32,
            col_num,
        );
        col += 1;
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn fdct64_new_sse4_1(
    input: *mut __m128i,
    output: *mut __m128i,
    cos_bit: i8,
    stage_range: *const i8,
) {
    let txfm_size: i32 = 64;
    let num_per_128: i32 = 4;
    let col_num: i32 = txfm_size / num_per_128;
    let _ = stage_range;
    let mut col: i32 = 0;
    while col < col_num {
        av1_fdct64_sse4_1(
            input.offset(col as isize),
            output.offset(col as isize),
            cos_bit,
            col_num,
            col_num,
        );
        col += 1;
    }
}
#[target_feature(enable = "sse4.1")]
unsafe fn idtx32x32_sse4_1(
    input: *mut __m128i,
    output: *mut __m128i,
    cos_bit: i8,
    stage_range: *const i8,
) {
    let _ = stage_range;

    let mut i: i32 = 0;
    while i < 8 {
        av1_idtx32_sse4_1(
            input.offset((i * 32) as isize),
            output.offset((i * 32) as isize),
            cos_bit as i32,
            1,
        );
        i += 1;
    }
}

#[inline]
fn fwd_txfm_type_to_func(txfm_type: TXFM_TYPE) -> TxfmFuncSSE2 {
    match txfm_type {
        TXFM_TYPE_DCT32 => return Some(fdct32_sse4_1),
        TXFM_TYPE_DCT64 => return Some(fdct64_new_sse4_1),
        TXFM_TYPE_IDENTITY32 => return Some(idtx32x32_sse4_1),
        _ => debug_assert!(false),
    }
    None
}

#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn fwd_txfm2d_sse4_1(
    input: *const i16,
    output: *mut i32,
    stride: i32,
    cfg: *const TXFM_2D_FLIP_CFG,
    txfm_buf: *mut i32,
) {
    // TODO(sarahparker) This does not currently support rectangular transforms
    // and will break without splitting txfm_size out into row and col size.
    // Rectangular transforms use c code only, so it should be ok for now.
    // It will be corrected when there are sse implementations for rectangular
    // transforms.
    debug_assert!((*cfg).tx_size < TX_SIZES);
    let txfm_size: i32 = tx_size_wide[(*cfg).tx_size as usize];
    let shift: *const i8 = (*cfg).shift;
    let stage_range_col: *const i8 = (&raw const (*cfg).stage_range_col).cast::<i8>();
    let stage_range_row: *const i8 = (&raw const (*cfg).stage_range_row).cast::<i8>();
    let cos_bit_col: i8 = (*cfg).cos_bit_col;
    let cos_bit_row: i8 = (*cfg).cos_bit_row;
    let txfm_func_col: TxfmFuncSSE2 = fwd_txfm_type_to_func((*cfg).txfm_type_col);
    let txfm_func_row: TxfmFuncSSE2 = fwd_txfm_type_to_func((*cfg).txfm_type_row);

    let buf_128: *mut __m128i = txfm_buf as *mut __m128i;
    let out_128: *mut __m128i = output as *mut __m128i;
    let num_per_128: i32 = 4;
    let txfm2d_size_128: i32 = txfm_size * txfm_size / num_per_128;

    int16_array_with_stride_to_int32_array_without_stride(input, stride, txfm_buf, txfm_size);
    av1_round_shift_array_32_sse4_1(buf_128, out_128, txfm2d_size_128, -(*shift.add(0) as i32));
    txfm_func_col.unwrap()(out_128, buf_128, cos_bit_col, stage_range_col);
    av1_round_shift_array_32_sse4_1(buf_128, out_128, txfm2d_size_128, -(*shift.add(1) as i32));
    transpose_32(txfm_size, out_128, buf_128);
    txfm_func_row.unwrap()(buf_128, out_128, cos_bit_row, stage_range_row);
    av1_round_shift_array_32_sse4_1(out_128, out_128, txfm2d_size_128, -(*shift.add(2) as i32));
}

#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn fwd_txfm2d_64x64_sse4_1(
    input: *const i16,
    output: *mut i32,
    stride: i32,
    cfg: *const TXFM_2D_FLIP_CFG,
    txfm_buf: *mut i32,
) {
    debug_assert!((*cfg).tx_size < TX_SIZES);
    let txfm_size: i32 = tx_size_wide[(*cfg).tx_size as usize];
    let shift: *const i8 = (*cfg).shift;
    let stage_range_col: *const i8 = (&raw const (*cfg).stage_range_col).cast::<i8>();
    let cos_bit_col: i8 = (*cfg).cos_bit_col;
    let cos_bit_row: i8 = (*cfg).cos_bit_row;
    let txfm_func_col: TxfmFuncSSE2 = fwd_txfm_type_to_func((*cfg).txfm_type_col);
    let buf_128: *mut __m128i = txfm_buf as *mut __m128i;
    let out_128: *mut __m128i = output as *mut __m128i;

    let num_per_128: i32 = 4;
    let mut txfm2d_size_128: i32 = txfm_size * txfm_size / num_per_128;
    let col_num: i32 = txfm_size / num_per_128;

    int16_array_with_stride_to_int32_array_without_stride(input, stride, output, txfm_size);
    /*col wise transform*/
    txfm_func_col.unwrap()(out_128, buf_128, cos_bit_col, stage_range_col);
    av1_round_shift_array_32_sse4_1(buf_128, out_128, txfm2d_size_128, -(*shift.add(1) as i32));
    transpose_32(txfm_size, out_128, buf_128);

    /*row wise transform*/
    let mut col: i32 = 0;
    while col < (col_num >> 1) {
        av1_fdct64_sse4_1(
            buf_128.offset(col as isize),
            out_128.offset(col as isize),
            cos_bit_row,
            col_num,
            col_num >> 1,
        );
        col += 1;
    }

    txfm2d_size_128 = (col_num >> 1) * (txfm_size >> 1);
    av1_round_shift_array_32_sse4_1(out_128, out_128, txfm2d_size_128, -(*shift.add(2) as i32));
}

#[no_mangle]
#[target_feature(enable = "sse4.1")]
pub unsafe extern "C-unwind" fn av1_fwd_txfm2d_32x32_sse4_1(
    input: *const i16,
    output: *mut i32,
    stride: i32,
    tx_type: TX_TYPE,
    bd: i32,
) {
    let mut txfm_buf: Aligned16<[i32; 1024]> = Aligned16([0; 1024]);
    let mut cfg: TXFM_2D_FLIP_CFG = core::mem::zeroed();
    av1_get_fwd_txfm_cfg(tx_type, TX_32X32, &raw mut cfg);
    let _ = bd;
    fwd_txfm2d_sse4_1(
        input,
        output,
        stride,
        &raw const cfg,
        txfm_buf.0.as_mut_ptr(),
    );
}

#[no_mangle]
#[target_feature(enable = "sse4.1")]
pub unsafe extern "C-unwind" fn av1_fwd_txfm2d_64x64_sse4_1(
    input: *const i16,
    output: *mut i32,
    stride: i32,
    tx_type: TX_TYPE,
    bd: i32,
) {
    let mut txfm_buf: Aligned16<[i32; 4096]> = Aligned16([0; 4096]);
    let mut cfg: TXFM_2D_FLIP_CFG = core::mem::zeroed();
    av1_get_fwd_txfm_cfg(tx_type, TX_64X64, &raw mut cfg);
    let _ = bd;
    fwd_txfm2d_64x64_sse4_1(
        input,
        output,
        stride,
        &raw const cfg,
        txfm_buf.0.as_mut_ptr(),
    );
}

// ---------------------------------------------------------------------------
// The low-bitdepth functions of av1_fwd_txfm2d_sse4.c (lowbd_fwd_txfm2d_*_sse4_1 and
// av1_lowbd_fwd_txfm_sse4_1), transliterated with a typing-aware C-subset printer (not C2Rust) and
// reviewed against the C function by function: the same sequence of intrinsic
// and helper calls, constants and operators in order (ADR 0058).

use crate::av1__common__idct::TxfmParam;
use crate::av1__encoder__av1_fwd_txfm2d::{
    av1_fwd_cos_bit_col, av1_fwd_cos_bit_row, av1_fwd_txfm_shift_ls,
};
use crate::simd::aom_dsp__x86__transpose_sse2::*;
use crate::simd::av1__common__x86__av1_inv_txfm_ssse3::{
    get_flip_cfg, get_rect_tx_log_ratio, get_txh_idx, get_txw_idx, tx_size_high, NewSqrt2, AOMMIN,
    DCT_DCT, TX_32X64, TX_4X4, TX_64X32, TX_SIZES_ALL,
};
use crate::simd::av1__common__x86__av1_txfm_sse2::*;
use crate::simd::av1__common__x86__av1_txfm_sse4::av1_round_shift_rect_array_32_sse4_1;
use crate::simd::av1__encoder__x86__av1_fwd_txfm_sse2::*;

type FwdTxfm2dFunc = Option<unsafe fn(*const i16, *mut i32, i32, u8, i32)>;

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

#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn store_output_32bit_w8(
    out: *mut i32,
    in1: *const __m128i,
    in2: *const __m128i,
    stride: i32,
    out_size: i32,
) {
    for i in 0..out_size {
        _mm_store_si128(
            out.offset((stride * i) as isize) as *mut __m128i,
            *in1.offset(i as isize),
        );
        _mm_store_si128(
            out.offset((stride * i) as isize).offset(4) as *mut __m128i,
            *in2.offset(i as isize),
        );
    }
}

#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn lowbd_fwd_txfm2d_64x64_sse4_1(
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
    let mut buf0: [__m128i; 64] = core::mem::zeroed();
    let mut buf1: [__m128i; 512] = core::mem::zeroed();
    let mut shift: *const i8 = av1_fwd_txfm_shift_ls[tx_size as usize];
    let txw_idx: i32 = get_txw_idx(tx_size);
    let txh_idx: i32 = get_txh_idx(tx_size);
    let cos_bit_col: i32 = av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32;
    let cos_bit_row: i32 = av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32;
    let width: i32 = tx_size_wide[tx_size as usize];
    let height: i32 = tx_size_high[tx_size as usize];
    let col_txfm: transform_1d_sse2 = Some(av1_fdct8x64_new_sse2);
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
        {
            let mut j: i32 = 0;
            while j < AOMMIN(4, height_div8) {
                transpose_16bit_8x8(
                    buf0.as_mut_ptr().offset((j * 8) as isize) as *const __m128i,
                    buf1.as_mut_ptr()
                        .offset((j * width) as isize)
                        .offset((8 * i) as isize),
                );
                j += 1;
            }
        }
    }
    {
        let mut i: i32 = 0;
        while i < AOMMIN(4, height_div8) {
            let mut bufA: [__m128i; 64] = core::mem::zeroed();
            let mut bufB: [__m128i; 64] = core::mem::zeroed();
            let mut buf: *mut __m128i = buf1.as_mut_ptr().offset((width * i) as isize);
            for j in 0..width {
                bufA[j as usize] = _mm_cvtepi16_epi32(*buf.offset(j as isize));
                bufB[j as usize] = _mm_cvtepi16_epi32(_mm_unpackhi_epi64(
                    *buf.offset(j as isize),
                    *buf.offset(j as isize),
                ));
            }
            av1_fdct64_sse4_1(
                bufA.as_mut_ptr(),
                bufA.as_mut_ptr(),
                cos_bit_row as i8,
                1,
                1,
            );
            av1_fdct64_sse4_1(
                bufB.as_mut_ptr(),
                bufB.as_mut_ptr(),
                cos_bit_row as i8,
                1,
                1,
            );
            av1_round_shift_array_32_sse4_1(
                bufA.as_mut_ptr() as *const __m128i,
                bufA.as_mut_ptr(),
                32,
                -(*shift.offset(2) as i32),
            );
            av1_round_shift_array_32_sse4_1(
                bufB.as_mut_ptr() as *const __m128i,
                bufB.as_mut_ptr(),
                32,
                -(*shift.offset(2) as i32),
            );
            store_output_32bit_w8(
                output.offset((i * 8) as isize),
                bufA.as_mut_ptr() as *const __m128i,
                bufB.as_mut_ptr() as *const __m128i,
                32,
                32,
            );
            i += 1;
        }
    }
}

#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn lowbd_fwd_txfm2d_64x32_sse4_1(
    input: *const i16,
    output: *mut i32,
    stride: i32,
    tx_type: u8,
    bd: i32,
) {
    let _ = bd;
    let tx_size: u8 = TX_64X32 as u8;
    let mut buf0: [__m128i; 64] = core::mem::zeroed();
    let mut buf1: [__m128i; 256] = core::mem::zeroed();
    let mut shift: *const i8 = av1_fwd_txfm_shift_ls[tx_size as usize];
    let txw_idx: i32 = get_txw_idx(tx_size);
    let txh_idx: i32 = get_txh_idx(tx_size);
    let cos_bit_col: i32 = av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32;
    let cos_bit_row: i32 = av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32;
    let width: i32 = tx_size_wide[tx_size as usize];
    let height: i32 = tx_size_high[tx_size as usize];
    let col_txfm: transform_1d_sse2 = col_txfm8x32_arr[tx_type as usize];
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
        {
            let mut j: i32 = 0;
            while j < AOMMIN(4, height_div8) {
                transpose_16bit_8x8(
                    buf0.as_mut_ptr().offset((j * 8) as isize) as *const __m128i,
                    buf1.as_mut_ptr()
                        .offset((j * width) as isize)
                        .offset((8 * i) as isize),
                );
                j += 1;
            }
        }
    }
    debug_assert!((tx_type as i32) == DCT_DCT);
    {
        let mut i: i32 = 0;
        while i < AOMMIN(4, height_div8) {
            let mut bufA: [__m128i; 64] = core::mem::zeroed();
            let mut bufB: [__m128i; 64] = core::mem::zeroed();
            let mut buf: *mut __m128i = buf1.as_mut_ptr().offset((width * i) as isize);
            for j in 0..width {
                bufA[j as usize] = _mm_cvtepi16_epi32(*buf.offset(j as isize));
                bufB[j as usize] = _mm_cvtepi16_epi32(_mm_unpackhi_epi64(
                    *buf.offset(j as isize),
                    *buf.offset(j as isize),
                ));
            }
            av1_fdct64_sse4_1(
                bufA.as_mut_ptr(),
                bufA.as_mut_ptr(),
                cos_bit_row as i8,
                1,
                1,
            );
            av1_fdct64_sse4_1(
                bufB.as_mut_ptr(),
                bufB.as_mut_ptr(),
                cos_bit_row as i8,
                1,
                1,
            );
            av1_round_shift_rect_array_32_sse4_1(
                bufA.as_mut_ptr() as *const __m128i,
                bufA.as_mut_ptr(),
                32,
                -(*shift.offset(2) as i32),
                NewSqrt2,
            );
            av1_round_shift_rect_array_32_sse4_1(
                bufB.as_mut_ptr() as *const __m128i,
                bufB.as_mut_ptr(),
                32,
                -(*shift.offset(2) as i32),
                NewSqrt2,
            );
            store_output_32bit_w8(
                output.offset((i * 8) as isize),
                bufA.as_mut_ptr() as *const __m128i,
                bufB.as_mut_ptr() as *const __m128i,
                32,
                32,
            );
            i += 1;
        }
    }
}

#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn lowbd_fwd_txfm2d_32x64_sse4_1(
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
    let mut buf0: [__m128i; 64] = core::mem::zeroed();
    let mut buf1: [__m128i; 256] = core::mem::zeroed();
    let mut shift: *const i8 = av1_fwd_txfm_shift_ls[tx_size as usize];
    let txw_idx: i32 = get_txw_idx(tx_size);
    let txh_idx: i32 = get_txh_idx(tx_size);
    let cos_bit_col: i32 = av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize] as i32;
    let cos_bit_row: i32 = av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize] as i32;
    let width: i32 = tx_size_wide[tx_size as usize];
    let height: i32 = tx_size_high[tx_size as usize];
    let col_txfm: transform_1d_sse2 = Some(av1_fdct8x64_new_sse2);
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
        {
            let mut j: i32 = 0;
            while j < AOMMIN(4, height_div8) {
                transpose_16bit_8x8(
                    buf0.as_mut_ptr().offset((j * 8) as isize) as *const __m128i,
                    buf1.as_mut_ptr()
                        .offset((j * width) as isize)
                        .offset((8 * i) as isize),
                );
                j += 1;
            }
        }
    }
    {
        let mut i: i32 = 0;
        while i < AOMMIN(4, height_div8) {
            let mut bufA: [__m128i; 32] = core::mem::zeroed();
            let mut bufB: [__m128i; 32] = core::mem::zeroed();
            let mut buf: *mut __m128i = buf1.as_mut_ptr().offset((width * i) as isize);
            for j in 0..width {
                bufA[j as usize] = _mm_cvtepi16_epi32(*buf.offset(j as isize));
                bufB[j as usize] = _mm_cvtepi16_epi32(_mm_unpackhi_epi64(
                    *buf.offset(j as isize),
                    *buf.offset(j as isize),
                ));
            }
            av1_fdct32_sse4_1(bufA.as_mut_ptr(), bufA.as_mut_ptr(), cos_bit_row, 1);
            av1_fdct32_sse4_1(bufB.as_mut_ptr(), bufB.as_mut_ptr(), cos_bit_row, 1);
            av1_round_shift_rect_array_32_sse4_1(
                bufA.as_mut_ptr() as *const __m128i,
                bufA.as_mut_ptr(),
                32,
                -(*shift.offset(2) as i32),
                NewSqrt2,
            );
            av1_round_shift_rect_array_32_sse4_1(
                bufB.as_mut_ptr() as *const __m128i,
                bufB.as_mut_ptr(),
                32,
                -(*shift.offset(2) as i32),
                NewSqrt2,
            );
            store_output_32bit_w8(
                output.offset((i * 8) as isize),
                bufA.as_mut_ptr() as *const __m128i,
                bufB.as_mut_ptr() as *const __m128i,
                32,
                32,
            );
            i += 1;
        }
    }
}

static fwd_txfm2d_func_ls: [FwdTxfm2dFunc; (TX_SIZES_ALL) as usize] = [
    Some(av1_lowbd_fwd_txfm2d_4x4_sse2),
    Some(av1_lowbd_fwd_txfm2d_8x8_sse2),
    Some(av1_lowbd_fwd_txfm2d_16x16_sse2),
    Some(av1_lowbd_fwd_txfm2d_32x32_sse2),
    Some(lowbd_fwd_txfm2d_64x64_sse4_1),
    Some(av1_lowbd_fwd_txfm2d_4x8_sse2),
    Some(av1_lowbd_fwd_txfm2d_8x4_sse2),
    Some(av1_lowbd_fwd_txfm2d_8x16_sse2),
    Some(av1_lowbd_fwd_txfm2d_16x8_sse2),
    Some(av1_lowbd_fwd_txfm2d_16x32_sse2),
    Some(av1_lowbd_fwd_txfm2d_32x16_sse2),
    Some(lowbd_fwd_txfm2d_32x64_sse4_1),
    Some(lowbd_fwd_txfm2d_64x32_sse4_1),
    Some(av1_lowbd_fwd_txfm2d_4x16_sse2),
    Some(av1_lowbd_fwd_txfm2d_16x4_sse2),
    Some(av1_lowbd_fwd_txfm2d_8x32_sse2),
    Some(av1_lowbd_fwd_txfm2d_32x8_sse2),
    Some(av1_lowbd_fwd_txfm2d_16x64_sse2),
    Some(av1_lowbd_fwd_txfm2d_64x16_sse2),
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
#[target_feature(enable = "sse4.1")]
pub unsafe extern "C-unwind" fn av1_lowbd_fwd_txfm_sse4_1(
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
