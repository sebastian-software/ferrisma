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

// Translated by hand from upstream/aom/av1/common/arm/av1_inv_txfm_neon.c and
// the static items of av1_inv_txfm_neon.h (ADR 0056). Intrinsic sequences,
// helper structure, and names follow the original; only the AOM_ARCH_AARCH64
// branches are retained. C locals that the original leaves uninitialized
// start zeroed here. The original calls the RTCD function
// av1_round_shift_array; the port does not dispatch it, so the translated
// av1_round_shift_array_c is called, as the port's RTCD header resolves it.
#![allow(non_snake_case, non_upper_case_globals, unused_assignments, unused_mut, unused_parens)]
#![allow(unused_variables, dead_code, clippy::all)]

use core::arch::aarch64::*;

use crate::av1__common__av1_inv_txfm1d::{
    av1_iadst16, av1_iadst4, av1_iadst8, av1_idct16, av1_idct32, av1_idct4, av1_idct64,
    av1_idct8, av1_iidentity16_c, av1_iidentity4_c, av1_iidentity8_c,
};
use crate::av1__common__av1_inv_txfm2d::av1_inv_txfm_shift_ls;
use crate::av1__common__av1_txfm::{av1_cospi_arr_data, av1_round_shift_array_c};
use crate::av1__common__idct::{av1_inv_txfm_add_c, TxfmParam};
use crate::simd::aom_dsp__arm__transpose_neon::transpose_arrays_s16_8x8;
use crate::simd::types::tran_low_t;

// ---------------------------------------------------------------------------
// Header items used by the original translation unit. Header-local (`static`)
// tables, inline functions, and macros are transcribed here; exported tables
// and functions come from the translated modules imported above.

// av1/common/enums.h
type TX_TYPE = u8;
type TX_SIZE = u8;

const TX_4X4: i32 = 0;
const TX_4X8: i32 = 5;
const TX_8X4: i32 = 6;
const TX_4X16: i32 = 13;
const TX_16X4: i32 = 14;
const TX_SIZES: usize = 5;

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

// av1/common/av1_inv_txfm1d_cfg.h
const INV_COS_BIT: i32 = 12;

// av1/common/av1_txfm.h
const cos_bit_min: i32 = 10;
const NewSqrt2Bits: i32 = 12;
// 2^12 / sqrt(2)
const NewInvSqrt2: i32 = 2896;
const MAX_TXFM_STAGE_NUM: usize = 12;

#[inline(always)]
unsafe fn cospi_arr(n: i32) -> *const i32 {
    (&raw const av1_cospi_arr_data[(n - cos_bit_min) as usize]).cast::<i32>()
}

#[inline(always)]
fn round_shift(value: i64, bit: i32) -> i32 {
    debug_assert!(bit >= 1);
    ((value + (1i64 << (bit - 1))) >> bit) as i32
}

#[inline(always)]
fn get_flip_cfg(tx_type: TX_TYPE, ud_flip: *mut i32, lr_flip: *mut i32) {
    let (ud, lr) = match tx_type as i32 {
        DCT_DCT | ADST_DCT | DCT_ADST | ADST_ADST => (0, 0),
        IDTX | V_DCT | H_DCT | V_ADST | H_ADST => (0, 0),
        FLIPADST_DCT | FLIPADST_ADST | V_FLIPADST => (1, 0),
        DCT_FLIPADST | ADST_FLIPADST | H_FLIPADST => (0, 1),
        FLIPADST_FLIPADST => (1, 1),
        _ => {
            debug_assert!(false);
            (0, 0)
        }
    };
    // SAFETY: the callers pass pointers to their own locals.
    unsafe {
        *ud_flip = ud;
        *lr_flip = lr;
    }
}

// Utility function that returns the log of the ratio of the col and row
// sizes.
#[inline(always)]
fn get_rect_tx_log_ratio(col: i32, row: i32) -> i32 {
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

#[inline(always)]
fn get_txw_idx(tx_size: TX_SIZE) -> i32 {
    tx_size_wide_log2[tx_size as usize] - tx_size_wide_log2[0]
}

#[inline(always)]
fn get_txh_idx(tx_size: TX_SIZE) -> i32 {
    tx_size_high_log2[tx_size as usize] - tx_size_high_log2[0]
}

// av1/common/av1_inv_txfm1d.h
#[inline(always)]
fn clamp_value(value: i32, bit: i8) -> i32 {
    if bit as i32 <= 0 {
        return value; // Do nothing for invalid clamp bit.
    }
    let max_value: i64 = (1i64 << (bit as i32 - 1)) - 1;
    let min_value: i64 = -(1i64 << (bit as i32 - 1));
    clamp64(value as i64, min_value, max_value) as i32
}

#[inline(always)]
unsafe fn clamp_buf(buf: *mut i32, size: i32, bit: i8) {
    for i in 0..size {
        *buf.offset(i as isize) = clamp_value(*buf.offset(i as isize), bit);
    }
}

// av1/common/common_data.h
// Transform block width in pixels
static tx_size_wide: [i32; 19] =
    [4, 8, 16, 32, 64, 4, 8, 8, 16, 16, 32, 32, 64, 4, 16, 8, 32, 16, 64];
// Transform block height in pixels
static tx_size_high: [i32; 19] =
    [4, 8, 16, 32, 64, 8, 4, 16, 8, 32, 16, 64, 32, 16, 4, 32, 8, 64, 16];
// Transform block width in log2
static tx_size_wide_log2: [i32; 19] = [2, 3, 4, 5, 6, 2, 3, 3, 4, 4, 5, 5, 6, 2, 4, 3, 5, 4, 6];
// Transform block height in log2
static tx_size_high_log2: [i32; 19] = [2, 3, 4, 5, 6, 3, 2, 4, 3, 5, 4, 6, 5, 4, 2, 5, 3, 6, 4];

// aom_dsp/aom_dsp_common.h
#[inline(always)]
fn AOMMIN(x: i32, y: i32) -> i32 {
    if x < y {
        x
    } else {
        y
    }
}

#[inline(always)]
fn AOMMAX(x: i32, y: i32) -> i32 {
    if x > y {
        x
    } else {
        y
    }
}

#[inline(always)]
fn clamp64(value: i64, low: i64, high: i64) -> i64 {
    if value < low {
        low
    } else if value > high {
        high
    } else {
        value
    }
}

#[inline(always)]
fn clip_pixel(val: i32) -> u8 {
    (if val > 255 {
        255
    } else if val < 0 {
        0
    } else {
        val
    }) as u8
}

// av1/common/arm/av1_inv_txfm_neon.h
type transform_1d_neon = Option<unsafe extern "C-unwind" fn(*const i32, *mut i32, i8, *const i8)>;
type transform_neon = Option<unsafe fn(*mut int16x8_t, *mut int16x8_t, i8)>;

// The DECLARE_ALIGNED(16, ...) alignment of these tables is not needed:
// they are only read element by element.
static av1_eob_to_eobxy_8x8_default: [i16; 8] =
    [0x0707, 0x0707, 0x0707, 0x0707, 0x0707, 0x0707, 0x0707, 0x0707];

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

static av1_eob_to_eobxy_16x8_default: [i16; 8] =
    [0x0707, 0x0707, 0x070f, 0x070f, 0x070f, 0x070f, 0x070f, 0x070f];

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

static av1_eob_to_eobxy_32x8_default: [i16; 8] =
    [0x0707, 0x070f, 0x070f, 0x071f, 0x071f, 0x071f, 0x071f, 0x071f];

// DECLARE_ALIGNED(16, ...) in the original; `static mut` because the
// table holds raw pointers.
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

static lowbd_txfm_all_1d_zeros_idx: [i32; 32] = [
    0, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
];

// Transform block width in log2 for eob (size of 64 map to 32)
static tx_size_wide_log2_eob: [i32; 19] = [2, 3, 4, 5, 5, 2, 3, 3, 4, 4, 5, 5, 5, 2, 4, 3, 5, 4, 5];

static eob_fill: [i32; 32] = [
    0, 7, 7, 7, 7, 7, 7, 7, 15, 15, 15, 15, 15, 15, 15, 15, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31,
    31, 31, 31, 31, 31, 31,
];

#[inline(always)]
unsafe fn get_eobx_eoby_scan_default(
    mut eobx: *mut i32,
    mut eoby: *mut i32,
    mut tx_size: TX_SIZE,
    mut eob: i32,
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

#[inline(always)]
unsafe fn get_eobx_eoby_scan_v_identity(
    mut eobx: *mut i32,
    mut eoby: *mut i32,
    mut tx_size: TX_SIZE,
    mut eob: i32,
) {
    eob -= 1;
    let txfm_size_row: i32 = tx_size_high[tx_size as usize];
    let eoby_max: i32 = AOMMIN(32, txfm_size_row) - 1;
    *eobx = eob / (eoby_max + 1);
    *eoby = if eob >= eoby_max { eoby_max } else { eob_fill[eob as usize] };
}

#[inline(always)]
unsafe fn get_eobx_eoby_scan_h_identity(
    mut eobx: *mut i32,
    mut eoby: *mut i32,
    mut tx_size: TX_SIZE,
    mut eob: i32,
) {
    eob -= 1;
    let txfm_size_col: i32 = tx_size_wide[tx_size as usize];
    let eobx_max: i32 = AOMMIN(32, txfm_size_col) - 1;
    *eobx = if eob >= eobx_max { eobx_max } else { eob_fill[eob as usize] };
    let temp_eoby: i32 = eob / (eobx_max + 1);
    debug_assert!(temp_eoby < 32);
    *eoby = eob_fill[temp_eoby as usize];
}

// ---------------------------------------------------------------------------

// 1D itx types
type ITX_TYPE_1D = u8;
const IDCT_1D: ITX_TYPE_1D = 0;
const IADST_1D: ITX_TYPE_1D = 1;
const IFLIPADST_1D: ITX_TYPE_1D = IADST_1D;
const IIDENTITY_1D: ITX_TYPE_1D = 2;
const ITX_TYPES_1D: usize = 3;

static vitx_1d_tab: [ITX_TYPE_1D; TX_TYPES] = [
    IDCT_1D,
    IADST_1D,
    IDCT_1D,
    IADST_1D,
    IFLIPADST_1D,
    IDCT_1D,
    IFLIPADST_1D,
    IADST_1D,
    IFLIPADST_1D,
    IIDENTITY_1D,
    IDCT_1D,
    IIDENTITY_1D,
    IADST_1D,
    IIDENTITY_1D,
    IFLIPADST_1D,
    IIDENTITY_1D,
];

static hitx_1d_tab: [ITX_TYPE_1D; TX_TYPES] = [
    IDCT_1D,
    IDCT_1D,
    IADST_1D,
    IADST_1D,
    IDCT_1D,
    IFLIPADST_1D,
    IFLIPADST_1D,
    IFLIPADST_1D,
    IADST_1D,
    IIDENTITY_1D,
    IIDENTITY_1D,
    IDCT_1D,
    IIDENTITY_1D,
    IADST_1D,
    IIDENTITY_1D,
    IFLIPADST_1D,
];

// 1D functions
static lowbd_txfm_all_1d_arr: [[transform_1d_neon; ITX_TYPES_1D]; TX_SIZES] = [
    [Some(av1_idct4), Some(av1_iadst4), Some(av1_iidentity4_c)],
    [Some(av1_idct8), Some(av1_iadst8), Some(av1_iidentity8_c)],
    [Some(av1_idct16), Some(av1_iadst16), Some(av1_iidentity16_c)],
    [Some(av1_idct32), None, None],
    [Some(av1_idct64), None, None],
];


#[inline(always)]
unsafe fn lowbd_add_flip_buffer_8xn_neon(mut in_: *mut int16x8_t, mut output: *mut u8, mut stride: i32, mut flipud: i32, mut height: i32) {
    let mut j: i32 = if flipud != 0 { height - 1 } else { 0 };
    let step: i32 = if flipud != 0 { -1 } else { 1 };
    let mut temp_output: int16x8_t = core::mem::zeroed();
    let mut i: i32 = 0;
    while i < height {
        temp_output = vreinterpretq_s16_u16(vmovl_u8(vld1_u8(output)));
        temp_output = vaddq_s16(temp_output, (*in_.offset((j) as isize)));
        vst1_u8(output, vqmovun_s16(temp_output));
        output = output.offset(stride as isize);
        i += 1;
        j += step;
    }
}

#[inline(always)]
unsafe fn lowbd_get_recon_16x16_neon(mut pred: uint8x16_t, mut res0: int16x8_t, mut res1: int16x8_t) -> uint8x16_t {
    let mut temp_output: [int16x8_t; 2] = core::mem::zeroed();
    let mut temp_output_8q: uint8x16_t = core::mem::zeroed();
    temp_output[0] = vreinterpretq_s16_u16(vmovl_u8(vget_low_u8(pred)));
    temp_output[0] = vaddq_s16(temp_output[0], res0);
    temp_output[1] = vreinterpretq_s16_u16(vmovl_u8(vget_high_u8(pred)));
    temp_output[1] = vaddq_s16(temp_output[1], res1);
    temp_output_8q = vcombine_u8(vqmovun_s16(temp_output[0]), vqmovun_s16(temp_output[1]));
    temp_output_8q
}

#[inline(always)]
unsafe fn lowbd_add_flip_buffer_16xn_neon(mut in_: *mut int16x8_t, mut output: *mut u8, mut stride: i32, mut flipud: i32, mut height: i32) {
    let mut temp_output_8q: uint8x16_t = core::mem::zeroed();
    let mut j: i32 = if flipud != 0 { height - 1 } else { 0 };
    let step: i32 = if flipud != 0 { -1 } else { 1 };
    let mut i: i32 = 0;
    while i < height {
        temp_output_8q = vld1q_u8(output.offset((i * stride) as isize));
        temp_output_8q = lowbd_get_recon_16x16_neon(temp_output_8q, (*in_.offset((j) as isize)), (*in_.offset((j + height) as isize)));
        vst1q_u8(output.offset((i * stride) as isize), temp_output_8q);
        i += 1;
        j += step;
    }
}

#[inline(always)]
unsafe fn lowbd_inv_txfm2d_memset_neon(mut a: *mut int16x8_t, mut size: i32, mut value: i32) {
    for i in 0..size {
        *a.offset((i) as isize) = vdupq_n_s16((value as i16));
    }
}

#[inline(always)]
unsafe fn btf_16_lane_0_1_neon(mut in0: int16x8_t, mut in1: int16x8_t, mut c: int16x4_t, mut t0: *mut int16x8_t, mut t1: *mut int16x8_t) {
    let mut s0: [int32x4_t; 2] = core::mem::zeroed();
    let mut s1: [int32x4_t; 2] = core::mem::zeroed();
    let mut v0: [int16x4_t; 2] = core::mem::zeroed();
    let mut v1: [int16x4_t; 2] = core::mem::zeroed();

    s0[0] = vmull_lane_s16(vget_low_s16(in0), c, 0);
    s0[1] = vmull_lane_s16(vget_high_s16(in0), c, 0);
    s1[0] = vmull_lane_s16(vget_low_s16(in0), c, 1);
    s1[1] = vmull_lane_s16(vget_high_s16(in0), c, 1);

    s0[0] = vmlal_lane_s16(s0[0], vget_low_s16(in1), c, 1);
    s0[1] = vmlal_lane_s16(s0[1], vget_high_s16(in1), c, 1);
    s1[0] = vmlsl_lane_s16(s1[0], vget_low_s16(in1), c, 0);
    s1[1] = vmlsl_lane_s16(s1[1], vget_high_s16(in1), c, 0);

    v0[0] = vrshrn_n_s32(s0[0], INV_COS_BIT);
    v0[1] = vrshrn_n_s32(s0[1], INV_COS_BIT);
    v1[0] = vrshrn_n_s32(s1[0], INV_COS_BIT);
    v1[1] = vrshrn_n_s32(s1[1], INV_COS_BIT);

    *t0 = vcombine_s16(v0[0], v0[1]);
    *t1 = vcombine_s16(v1[0], v1[1]);
}

#[inline(always)]
unsafe fn btf_16_lane_1_0_neon(mut in0: int16x8_t, mut in1: int16x8_t, mut c: int16x4_t, mut t0: *mut int16x8_t, mut t1: *mut int16x8_t) {
    let mut s0: [int32x4_t; 2] = core::mem::zeroed();
    let mut s1: [int32x4_t; 2] = core::mem::zeroed();
    let mut v0: [int16x4_t; 2] = core::mem::zeroed();
    let mut v1: [int16x4_t; 2] = core::mem::zeroed();

    s0[0] = vmull_lane_s16(vget_low_s16(in0), c, 1);
    s0[1] = vmull_lane_s16(vget_high_s16(in0), c, 1);
    s1[0] = vmull_lane_s16(vget_low_s16(in0), c, 0);
    s1[1] = vmull_lane_s16(vget_high_s16(in0), c, 0);

    s0[0] = vmlal_lane_s16(s0[0], vget_low_s16(in1), c, 0);
    s0[1] = vmlal_lane_s16(s0[1], vget_high_s16(in1), c, 0);
    s1[0] = vmlsl_lane_s16(s1[0], vget_low_s16(in1), c, 1);
    s1[1] = vmlsl_lane_s16(s1[1], vget_high_s16(in1), c, 1);

    v0[0] = vrshrn_n_s32(s0[0], INV_COS_BIT);
    v0[1] = vrshrn_n_s32(s0[1], INV_COS_BIT);
    v1[0] = vrshrn_n_s32(s1[0], INV_COS_BIT);
    v1[1] = vrshrn_n_s32(s1[1], INV_COS_BIT);

    *t0 = vcombine_s16(v0[0], v0[1]);
    *t1 = vcombine_s16(v1[0], v1[1]);
}

#[inline(always)]
unsafe fn btf_16_lane_2_3_neon(mut in0: int16x8_t, mut in1: int16x8_t, mut c: int16x4_t, mut t0: *mut int16x8_t, mut t1: *mut int16x8_t) {
    let mut s0: [int32x4_t; 2] = core::mem::zeroed();
    let mut s1: [int32x4_t; 2] = core::mem::zeroed();
    let mut v0: [int16x4_t; 2] = core::mem::zeroed();
    let mut v1: [int16x4_t; 2] = core::mem::zeroed();

    s0[0] = vmull_lane_s16(vget_low_s16(in0), c, 2);
    s0[1] = vmull_lane_s16(vget_high_s16(in0), c, 2);
    s1[0] = vmull_lane_s16(vget_low_s16(in0), c, 3);
    s1[1] = vmull_lane_s16(vget_high_s16(in0), c, 3);

    s0[0] = vmlal_lane_s16(s0[0], vget_low_s16(in1), c, 3);
    s0[1] = vmlal_lane_s16(s0[1], vget_high_s16(in1), c, 3);
    s1[0] = vmlsl_lane_s16(s1[0], vget_low_s16(in1), c, 2);
    s1[1] = vmlsl_lane_s16(s1[1], vget_high_s16(in1), c, 2);

    v0[0] = vrshrn_n_s32(s0[0], INV_COS_BIT);
    v0[1] = vrshrn_n_s32(s0[1], INV_COS_BIT);
    v1[0] = vrshrn_n_s32(s1[0], INV_COS_BIT);
    v1[1] = vrshrn_n_s32(s1[1], INV_COS_BIT);

    *t0 = vcombine_s16(v0[0], v0[1]);
    *t1 = vcombine_s16(v1[0], v1[1]);
}

#[inline(always)]
unsafe fn btf_16_neon(mut in0: int16x8_t, mut coef1: i16, mut coef2: i16, mut t0: *mut int16x8_t, mut t1: *mut int16x8_t) {
    let mut s0_l: int32x4_t = core::mem::zeroed();
    let mut s0_h: int32x4_t = core::mem::zeroed();
    let mut s1_l: int32x4_t = core::mem::zeroed();
    let mut s1_h: int32x4_t = core::mem::zeroed();
    let mut v0: [int16x4_t; 2] = core::mem::zeroed();
    let mut v1: [int16x4_t; 2] = core::mem::zeroed();

    s0_l = vmull_n_s16(vget_low_s16(in0), coef1);
    s0_h = vmull_n_s16(vget_high_s16(in0), coef1);
    s1_l = vmull_n_s16(vget_low_s16(in0), coef2);
    s1_h = vmull_n_s16(vget_high_s16(in0), coef2);

    v0[0] = vrshrn_n_s32(s0_l, INV_COS_BIT);
    v0[1] = vrshrn_n_s32(s0_h, INV_COS_BIT);
    v1[0] = vrshrn_n_s32(s1_l, INV_COS_BIT);
    v1[1] = vrshrn_n_s32(s1_h, INV_COS_BIT);

    *t0 = vcombine_s16(v0[0], v0[1]);
    *t1 = vcombine_s16(v1[0], v1[1]);
}

#[inline(always)]
unsafe fn btf_16_lane_3_2_neon(mut in0: int16x8_t, mut in1: int16x8_t, mut c: int16x4_t, mut t0: *mut int16x8_t, mut t1: *mut int16x8_t) {
    let mut s0: [int32x4_t; 2] = core::mem::zeroed();
    let mut s1: [int32x4_t; 2] = core::mem::zeroed();
    let mut v0: [int16x4_t; 2] = core::mem::zeroed();
    let mut v1: [int16x4_t; 2] = core::mem::zeroed();

    s0[0] = vmull_lane_s16(vget_low_s16(in0), c, 3);
    s0[1] = vmull_lane_s16(vget_high_s16(in0), c, 3);
    s1[0] = vmull_lane_s16(vget_low_s16(in0), c, 2);
    s1[1] = vmull_lane_s16(vget_high_s16(in0), c, 2);

    s0[0] = vmlal_lane_s16(s0[0], vget_low_s16(in1), c, 2);
    s0[1] = vmlal_lane_s16(s0[1], vget_high_s16(in1), c, 2);
    s1[0] = vmlsl_lane_s16(s1[0], vget_low_s16(in1), c, 3);
    s1[1] = vmlsl_lane_s16(s1[1], vget_high_s16(in1), c, 3);

    v0[0] = vrshrn_n_s32(s0[0], INV_COS_BIT);
    v0[1] = vrshrn_n_s32(s0[1], INV_COS_BIT);
    v1[0] = vrshrn_n_s32(s1[0], INV_COS_BIT);
    v1[1] = vrshrn_n_s32(s1[1], INV_COS_BIT);

    *t0 = vcombine_s16(v0[0], v0[1]);
    *t1 = vcombine_s16(v1[0], v1[1]);
}

#[inline(always)]
unsafe fn btf_16_half_neon(mut x: *mut int16x8_t, mut c: int16x4_t) {
    let mut t0: [int32x4_t; 2] = core::mem::zeroed();
    let mut t1: [int32x4_t; 2] = core::mem::zeroed();
    let mut v0: [int16x4_t; 2] = core::mem::zeroed();
    let mut v1: [int16x4_t; 2] = core::mem::zeroed();

    // Don't add/sub before multiply, which will overflow in iadst8.
    let x0_lo: int32x4_t = vmull_lane_s16(vget_low_s16((*x.add(0))), c, 0);
    let x0_hi: int32x4_t = vmull_lane_s16(vget_high_s16((*x.add(0))), c, 0);
    let x1_lo: int32x4_t = vmull_lane_s16(vget_low_s16((*x.add(1))), c, 0);
    let x1_hi: int32x4_t = vmull_lane_s16(vget_high_s16((*x.add(1))), c, 0);

    t0[0] = vaddq_s32(x0_lo, x1_lo);
    t0[1] = vaddq_s32(x0_hi, x1_hi);
    t1[0] = vsubq_s32(x0_lo, x1_lo);
    t1[1] = vsubq_s32(x0_hi, x1_hi);

    v0[0] = vrshrn_n_s32(t0[0], INV_COS_BIT);
    v0[1] = vrshrn_n_s32(t0[1], INV_COS_BIT);
    v1[0] = vrshrn_n_s32(t1[0], INV_COS_BIT);
    v1[1] = vrshrn_n_s32(t1[1], INV_COS_BIT);

    *x.add(0) = vcombine_s16(v0[0], v0[1]);
    *x.add(1) = vcombine_s16(v1[0], v1[1]);
}

#[inline(always)]
unsafe fn set_s16x4_neon(mut c0: i16, mut c1: i16, mut c2: i16, mut c3: i16) -> int16x4_t {
    let mut val: int16x4_t = vdup_n_s16(c0);
    val = vset_lane_s16(c1, val, 1);
    val = vset_lane_s16(c2, val, 2);
    val = vset_lane_s16(c3, val, 3);
    val
}

#[inline(always)]
unsafe fn iadst8_neon(mut in_: *mut int16x8_t, mut out: *mut int16x8_t, mut cos_bit: i8) {
    let cospi: *const i32 = cospi_arr(cos_bit as i32);

    let c0: int16x4_t = set_s16x4_neon(((*cospi.add(4) as i16) as i16), ((*cospi.add(60) as i16) as i16), ((*cospi.add(20) as i16) as i16), ((*cospi.add(44) as i16) as i16));
    let c1: int16x4_t = set_s16x4_neon(((*cospi.add(36) as i16) as i16), ((*cospi.add(28) as i16) as i16), ((*cospi.add(52) as i16) as i16), ((*cospi.add(12) as i16) as i16));
    let c2: int16x4_t = set_s16x4_neon(((*cospi.add(32) as i16) as i16), ((*cospi.add(32) as i16) as i16), ((*cospi.add(16) as i16) as i16), ((*cospi.add(48) as i16) as i16));

    let mut x: [int16x8_t; 8] = core::mem::zeroed();
    let mut s0: int16x8_t = core::mem::zeroed();
    let mut s1: int16x8_t = core::mem::zeroed();
    let mut s2: int16x8_t = core::mem::zeroed();
    let mut s3: int16x8_t = core::mem::zeroed();
    let mut s4: int16x8_t = core::mem::zeroed();
    let mut s5: int16x8_t = core::mem::zeroed();
    let mut s6: int16x8_t = core::mem::zeroed();
    let mut s7: int16x8_t = core::mem::zeroed();

    // Stage 1
    x[0] = (*in_.add(7));
    x[1] = (*in_.add(0));
    x[2] = (*in_.add(5));
    x[3] = (*in_.add(2));
    x[4] = (*in_.add(3));
    x[5] = (*in_.add(4));
    x[6] = (*in_.add(1));
    x[7] = (*in_.add(6));

    // Stage 2
    btf_16_lane_0_1_neon(x[0], x[1], c0, &raw mut s0, &raw mut s1);
    btf_16_lane_2_3_neon(x[2], x[3], c0, &raw mut s2, &raw mut s3);
    btf_16_lane_0_1_neon(x[4], x[5], c1, &raw mut s4, &raw mut s5);
    btf_16_lane_2_3_neon(x[6], x[7], c1, &raw mut s6, &raw mut s7);

    // Stage 3
    x[0] = vqaddq_s16(s0, s4);
    x[1] = vqaddq_s16(s1, s5);
    x[2] = vqaddq_s16(s2, s6);
    x[3] = vqaddq_s16(s3, s7);
    x[4] = vqsubq_s16(s0, s4);
    x[5] = vqsubq_s16(s1, s5);
    x[6] = vqsubq_s16(s2, s6);
    x[7] = vqsubq_s16(s3, s7);

    // Stage 4
    s0 = x[0];
    s1 = x[1];
    s2 = x[2];
    s3 = x[3];
    btf_16_lane_2_3_neon(x[4], x[5], c2, &raw mut s4, &raw mut s5);
    btf_16_lane_3_2_neon(x[7], x[6], c2, &raw mut s7, &raw mut s6);

    // Stage 5
    x[0] = vqaddq_s16(s0, s2);
    x[1] = vqaddq_s16(s1, s3);
    x[2] = vqsubq_s16(s0, s2);
    x[3] = vqsubq_s16(s1, s3);
    x[4] = vqaddq_s16(s4, s6);
    x[5] = vqaddq_s16(s5, s7);
    x[6] = vqsubq_s16(s4, s6);
    x[7] = vqsubq_s16(s5, s7);

    // stage 6
    btf_16_half_neon(x.as_mut_ptr().add(2), c2);
    btf_16_half_neon(x.as_mut_ptr().add(6), c2);

    // Stage 7
    *out.add(0) = x[0];
    *out.add(1) = vqnegq_s16(x[4]);
    *out.add(2) = x[6];
    *out.add(3) = vqnegq_s16(x[2]);
    *out.add(4) = x[3];
    *out.add(5) = vqnegq_s16(x[7]);
    *out.add(6) = x[5];
    *out.add(7) = vqnegq_s16(x[1]);
}

#[inline(always)]
unsafe fn iadst8_low1_neon(mut in_: *mut int16x8_t, mut out: *mut int16x8_t, mut cos_bit: i8) {
    let cospi: *const i32 = cospi_arr(cos_bit as i32);
    let c2: int16x4_t = set_s16x4_neon(((*cospi.add(32) as i16) as i16), ((*cospi.add(32) as i16) as i16), ((*cospi.add(16) as i16) as i16), ((*cospi.add(48) as i16) as i16));

    let mut x: [int16x8_t; 8] = core::mem::zeroed();
    let mut s0: int16x8_t = core::mem::zeroed();
    let mut s1: int16x8_t = core::mem::zeroed();
    let mut s4: int16x8_t = core::mem::zeroed();
    let mut s5: int16x8_t = core::mem::zeroed();

    // Stage 1
    x[1] = (*in_.add(0));

    // Stage 2

    btf_16_neon(x[1], (*cospi.add(60) as i16), -(*cospi.add(4) as i16), &raw mut s0, &raw mut s1);

    // Stage 3
    x[0] = s0;
    x[1] = s1;
    x[4] = s0;
    x[5] = s1;

    // Stage 4
    s0 = x[0];
    s1 = x[1];
    btf_16_lane_2_3_neon(x[4], x[5], c2, &raw mut s4, &raw mut s5);

    // Stage 5
    x[0] = s0;
    x[1] = s1;
    x[2] = s0;
    x[3] = s1;
    x[4] = s4;
    x[5] = s5;
    x[6] = s4;
    x[7] = s5;

    // stage 6
    btf_16_half_neon(x.as_mut_ptr().add(2), c2);
    btf_16_half_neon(x.as_mut_ptr().add(6), c2);

    // Stage 7
    *out.add(0) = x[0];
    *out.add(1) = vqnegq_s16(x[4]);
    *out.add(2) = x[6];
    *out.add(3) = vqnegq_s16(x[2]);
    *out.add(4) = x[3];
    *out.add(5) = vqnegq_s16(x[7]);
    *out.add(6) = x[5];
    *out.add(7) = vqnegq_s16(x[1]);
}

#[inline(always)]
unsafe fn idct8_neon(mut in_: *mut int16x8_t, mut out: *mut int16x8_t, mut cos_bit: i8) {
    let cospi: *const i32 = cospi_arr(cos_bit as i32);
    let mut step1: [int16x8_t; 8] = core::mem::zeroed();
    let mut step2: [int16x8_t; 8] = core::mem::zeroed();
    let c0: int16x4_t = set_s16x4_neon(((*cospi.add(8) as i16) as i16), ((*cospi.add(56) as i16) as i16), ((*cospi.add(40) as i16) as i16), ((*cospi.add(24) as i16) as i16));
    let c1: int16x4_t = set_s16x4_neon(((*cospi.add(32) as i16) as i16), ((*cospi.add(32) as i16) as i16), ((*cospi.add(16) as i16) as i16), ((*cospi.add(48) as i16) as i16));

    // stage 2
    btf_16_lane_0_1_neon((*in_.add(1)), (*in_.add(7)), c0, &raw mut step1[7], &raw mut step1[4]);
    btf_16_lane_2_3_neon((*in_.add(5)), (*in_.add(3)), c0, &raw mut step1[6], &raw mut step1[5]);

    // stage 3
    btf_16_lane_0_1_neon((*in_.add(0)), (*in_.add(4)), c1, &raw mut step2[0], &raw mut step2[1]);
    btf_16_lane_2_3_neon((*in_.add(2)), (*in_.add(6)), c1, &raw mut step2[3], &raw mut step2[2]);
    step2[4] = vqaddq_s16(step1[4], step1[5]);
    step2[5] = vqsubq_s16(step1[4], step1[5]);
    step2[6] = vqsubq_s16(step1[7], step1[6]);
    step2[7] = vqaddq_s16(step1[7], step1[6]);

    // stage 4
    step1[0] = vqaddq_s16(step2[0], step2[3]);
    step1[1] = vqaddq_s16(step2[1], step2[2]);
    step1[2] = vqsubq_s16(step2[1], step2[2]);
    step1[3] = vqsubq_s16(step2[0], step2[3]);
    btf_16_lane_0_1_neon(step2[6], step2[5], c1, &raw mut step1[6], &raw mut step1[5]);

    // stage 5
    *out.add(0) = vqaddq_s16(step1[0], step2[7]);
    *out.add(1) = vqaddq_s16(step1[1], step1[6]);
    *out.add(2) = vqaddq_s16(step1[2], step1[5]);
    *out.add(3) = vqaddq_s16(step1[3], step2[4]);
    *out.add(4) = vqsubq_s16(step1[3], step2[4]);
    *out.add(5) = vqsubq_s16(step1[2], step1[5]);
    *out.add(6) = vqsubq_s16(step1[1], step1[6]);
    *out.add(7) = vqsubq_s16(step1[0], step2[7]);
}

#[inline(always)]
unsafe fn idct8_low1_neon(mut in_: *mut int16x8_t, mut out: *mut int16x8_t, mut cos_bit: i8) {
    let cospi: *const i32 = cospi_arr(cos_bit as i32);
    let mut step1: int16x8_t = core::mem::zeroed();
    let mut t32: [int32x4_t; 2] = core::mem::zeroed();

    // stage 1
    // stage 2
    // stage 3
    t32[0] = vmull_n_s16(vget_low_s16((*in_.add(0))), ((*cospi.add(32) as i16) as i16));
    t32[1] = vmull_n_s16(vget_high_s16((*in_.add(0))), ((*cospi.add(32) as i16) as i16));

    step1 = vcombine_s16(vrshrn_n_s32(t32[0], INV_COS_BIT), vrshrn_n_s32(t32[1], INV_COS_BIT));

    // stage 4
    // stage 5
    *out.add(0) = step1;
    *out.add(1) = step1;
    *out.add(2) = step1;
    *out.add(3) = step1;
    *out.add(4) = step1;
    *out.add(5) = step1;
    *out.add(6) = step1;
    *out.add(7) = step1;
}

#[inline(always)]
unsafe fn round_shift_array_16_neon(mut arr: *mut int16x8_t, mut size: i32, mut bit: i32) {
    debug_assert!(size % 4 == 0);
    if bit == 0 {
        return;
    }
    let dup_bits_n_16x8: int16x8_t = vdupq_n_s16(((-bit) as i16));
    for i in 0..size {
        *arr.offset((i) as isize) = vrshlq_s16((*arr.offset((i) as isize)), dup_bits_n_16x8);
    }
}

#[inline(always)]
unsafe fn flip_buf_ud_neon(mut input: *mut int16x8_t, mut size: i32) {
    let mut temp: [int16x8_t; 8] = core::mem::zeroed();
    for i in 0..size {
        temp[(i) as usize] = (*input.offset((size - 1 - i) as isize));
    }
    for i in 0..size {
        *input.offset((i) as isize) = temp[(i) as usize];
    }
}

#[inline(always)]
unsafe fn load_buffer_32bit_to_16bit_neon(mut input: *const i32, mut stride: i32, mut a: *mut int16x8_t, mut out_size: i32) {
    for i in 0..out_size {
        *a.offset((i) as isize) = vcombine_s16(vmovn_s32(vld1q_s32(input)), vmovn_s32(vld1q_s32(input.add(4))));
        input = input.offset(stride as isize);
    }
}

static sqrt_2_list: [i16; TX_SIZES] = [5793, 2 * 4096, 2 * 5793, 4 * 4096, 4 * 5793];

#[inline(always)]
unsafe fn identity_txfm_round_neon(mut input: *mut int16x8_t, mut output: *mut int16x8_t, mut txw_idx: i32, mut size: i8, mut bit: i32) {
    let dup_bits_n_32x4: int32x4_t = vdupq_n_s32(-bit);
    let scale: int16x4_t = vdup_n_s16(sqrt_2_list[txw_idx as usize]);
    let mut low_i16: int16x4_t = core::mem::zeroed();
    let mut high_i16: int16x4_t = core::mem::zeroed();
    let mut low_i32: int32x4_t = core::mem::zeroed();
    let mut high_i32: int32x4_t = core::mem::zeroed();
    for i in 0..size as i32 {
        let temp_out_low: int32x4_t = vmull_s16(vget_low_s16((*input.offset((i) as isize))), scale);
        let temp_out_high: int32x4_t = vmull_s16(vget_high_s16((*input.offset((i) as isize))), scale);
        low_i32 = vrshlq_s32(vrshrq_n_s32(temp_out_low, 12), dup_bits_n_32x4);
        high_i32 = vrshlq_s32(vrshrq_n_s32(temp_out_high, 12), dup_bits_n_32x4);
        low_i16 = vqmovn_s32(low_i32);
        high_i16 = vqmovn_s32(high_i32);
        *output.offset((i) as isize) = vcombine_s16(low_i16, high_i16);
    }
}

#[inline(always)]
unsafe fn round_shift_for_rect(mut input: *mut int16x8_t, mut output: *mut int16x8_t, mut size: i32) {
    let mut out_low: int32x4_t = core::mem::zeroed();
    let mut out_high: int32x4_t = core::mem::zeroed();
    let mut low: int16x4_t = core::mem::zeroed();
    let mut high: int16x4_t = core::mem::zeroed();

    for z in 0..size {
        out_low = vmull_n_s16(vget_low_s16((*input.offset((z) as isize))), (NewInvSqrt2 as i16));
        out_high = vmull_n_s16(vget_high_s16((*input.offset((z) as isize))), (NewInvSqrt2 as i16));

        low = vqrshrn_n_s32(out_low, NewSqrt2Bits);
        high = vqrshrn_n_s32(out_high, NewSqrt2Bits);

        *output.offset((z) as isize) = vcombine_s16(low, high);
    }
}

#[inline(always)]
unsafe fn idct16_low1_neon(mut in_: *mut int16x8_t, mut out: *mut int16x8_t, mut cos_bit: i8) {
    let cospi: *const i32 = cospi_arr(cos_bit as i32);
    let mut step1: int16x8_t = core::mem::zeroed();
    let mut t32: [int32x4_t; 2] = core::mem::zeroed();

    // stage 4

    t32[0] = vmull_n_s16(vget_low_s16((*in_.add(0))), (*cospi.add(32) as i16));
    t32[1] = vmull_n_s16(vget_high_s16((*in_.add(0))), (*cospi.add(32) as i16));
    step1 = vcombine_s16(vrshrn_n_s32(t32[0], INV_COS_BIT), vrshrn_n_s32(t32[1], INV_COS_BIT));

    // stage 6
    // stage 7
    *out.add(0) = step1;
    *out.add(1) = step1;
    *out.add(2) = step1;
    *out.add(3) = step1;
    *out.add(4) = step1;
    *out.add(5) = step1;
    *out.add(6) = step1;
    *out.add(7) = step1;
    *out.add(8) = step1;
    *out.add(9) = step1;
    *out.add(10) = step1;
    *out.add(11) = step1;
    *out.add(12) = step1;
    *out.add(13) = step1;
    *out.add(14) = step1;
    *out.add(15) = step1;
}

#[inline(always)]
unsafe fn idct16_neon(mut in_: *mut int16x8_t, mut out: *mut int16x8_t, mut cos_bit: i8) {
    let cospi: *const i32 = cospi_arr(cos_bit as i32);
    let mut step1: [int16x8_t; 16] = core::mem::zeroed();
    let mut step2: [int16x8_t; 16] = core::mem::zeroed();

    let c0: int16x4_t = set_s16x4_neon(((*cospi.add(4) as i16) as i16), ((*cospi.add(60) as i16) as i16), ((*cospi.add(36) as i16) as i16), ((*cospi.add(28) as i16) as i16));
    let c1: int16x4_t = set_s16x4_neon(((*cospi.add(20) as i16) as i16), ((*cospi.add(44) as i16) as i16), ((*cospi.add(52) as i16) as i16), ((*cospi.add(12) as i16) as i16));
    let c2: int16x4_t = set_s16x4_neon(((*cospi.add(8) as i16) as i16), ((*cospi.add(56) as i16) as i16), ((*cospi.add(40) as i16) as i16), ((*cospi.add(24) as i16) as i16));
    let c3: int16x4_t = set_s16x4_neon(((*cospi.add(32) as i16) as i16), ((*cospi.add(32) as i16) as i16), ((*cospi.add(16) as i16) as i16), ((*cospi.add(48) as i16) as i16));
    let c4: int16x4_t = set_s16x4_neon(((-(*cospi.add(32) as i16)) as i16), ((-(*cospi.add(32) as i16)) as i16), ((-(*cospi.add(16) as i16)) as i16), ((-(*cospi.add(48) as i16)) as i16));
    // stage 2

    btf_16_lane_0_1_neon((*in_.add(1)), (*in_.add(15)), c0, &raw mut step2[15], &raw mut step2[8]);
    btf_16_lane_2_3_neon((*in_.add(9)), (*in_.add(7)), c0, &raw mut step2[14], &raw mut step2[9]);
    btf_16_lane_0_1_neon((*in_.add(5)), (*in_.add(11)), c1, &raw mut step2[13], &raw mut step2[10]);
    btf_16_lane_2_3_neon((*in_.add(13)), (*in_.add(3)), c1, &raw mut step2[12], &raw mut step2[11]);

    step2[0] = (*in_.add(0));
    step2[1] = (*in_.add(8));
    step2[2] = (*in_.add(4));
    step2[3] = (*in_.add(12));
    step2[4] = (*in_.add(2));
    step2[5] = (*in_.add(10));
    step2[6] = (*in_.add(6));
    step2[7] = (*in_.add(14));

    // stage 3

    btf_16_lane_0_1_neon(step2[4], step2[7], c2, &raw mut step1[7], &raw mut step1[4]);
    btf_16_lane_2_3_neon(step2[5], step2[6], c2, &raw mut step1[6], &raw mut step1[5]);

    step1[0] = step2[0];
    step1[1] = step2[1];
    step1[2] = step2[2];
    step1[3] = step2[3];
    step1[8] = vqaddq_s16(step2[8], step2[9]);
    step1[9] = vqsubq_s16(step2[8], step2[9]);
    step1[10] = vqsubq_s16(step2[11], step2[10]);
    step1[11] = vqaddq_s16(step2[11], step2[10]);
    step1[12] = vqaddq_s16(step2[12], step2[13]);
    step1[13] = vqsubq_s16(step2[12], step2[13]);
    step1[14] = vqsubq_s16(step2[15], step2[14]);
    step1[15] = vqaddq_s16(step2[15], step2[14]);

    // stage 4

    btf_16_lane_0_1_neon(step1[0], step1[1], c3, &raw mut step2[0], &raw mut step2[1]);
    btf_16_lane_2_3_neon(step1[2], step1[3], c3, &raw mut step2[3], &raw mut step2[2]);
    btf_16_lane_2_3_neon(step1[14], step1[9], c3, &raw mut step2[14], &raw mut step2[9]);
    btf_16_lane_3_2_neon(step1[10], step1[13], c4, &raw mut step2[10], &raw mut step2[13]);

    step2[4] = vqaddq_s16(step1[4], step1[5]);
    step2[5] = vqsubq_s16(step1[4], step1[5]);
    step2[6] = vqsubq_s16(step1[7], step1[6]);
    step2[7] = vqaddq_s16(step1[7], step1[6]);
    step2[8] = step1[8];
    step2[11] = step1[11];
    step2[12] = step1[12];
    step2[15] = step1[15];

    // stage 5

    btf_16_lane_0_1_neon(step2[6], step2[5], c3, &raw mut step1[6], &raw mut step1[5]);

    step1[0] = vqaddq_s16(step2[0], step2[3]);
    step1[1] = vqaddq_s16(step2[1], step2[2]);
    step1[2] = vqsubq_s16(step2[1], step2[2]);
    step1[3] = vqsubq_s16(step2[0], step2[3]);
    step1[4] = step2[4];
    step1[7] = step2[7];
    step1[8] = vqaddq_s16(step2[8], step2[11]);
    step1[9] = vqaddq_s16(step2[9], step2[10]);
    step1[10] = vqsubq_s16(step2[9], step2[10]);
    step1[11] = vqsubq_s16(step2[8], step2[11]);
    step1[12] = vqsubq_s16(step2[15], step2[12]);
    step1[13] = vqsubq_s16(step2[14], step2[13]);
    step1[14] = vqaddq_s16(step2[14], step2[13]);
    step1[15] = vqaddq_s16(step2[15], step2[12]);

    // stage 6

    btf_16_lane_0_1_neon(step1[13], step1[10], c3, &raw mut step2[13], &raw mut step2[10]);
    btf_16_lane_0_1_neon(step1[12], step1[11], c3, &raw mut step2[12], &raw mut step2[11]);

    step2[0] = vqaddq_s16(step1[0], step1[7]);
    step2[1] = vqaddq_s16(step1[1], step1[6]);
    step2[2] = vqaddq_s16(step1[2], step1[5]);
    step2[3] = vqaddq_s16(step1[3], step1[4]);
    step2[4] = vqsubq_s16(step1[3], step1[4]);
    step2[5] = vqsubq_s16(step1[2], step1[5]);
    step2[6] = vqsubq_s16(step1[1], step1[6]);
    step2[7] = vqsubq_s16(step1[0], step1[7]);
    step2[8] = step1[8];
    step2[9] = step1[9];
    step2[14] = step1[14];
    step2[15] = step1[15];

    // stage 7
    *out.add(0) = vqaddq_s16(step2[0], step2[15]);
    *out.add(1) = vqaddq_s16(step2[1], step2[14]);
    *out.add(2) = vqaddq_s16(step2[2], step2[13]);
    *out.add(3) = vqaddq_s16(step2[3], step2[12]);
    *out.add(4) = vqaddq_s16(step2[4], step2[11]);
    *out.add(5) = vqaddq_s16(step2[5], step2[10]);
    *out.add(6) = vqaddq_s16(step2[6], step2[9]);
    *out.add(7) = vqaddq_s16(step2[7], step2[8]);
    *out.add(8) = vqsubq_s16(step2[7], step2[8]);
    *out.add(9) = vqsubq_s16(step2[6], step2[9]);
    *out.add(10) = vqsubq_s16(step2[5], step2[10]);
    *out.add(11) = vqsubq_s16(step2[4], step2[11]);
    *out.add(12) = vqsubq_s16(step2[3], step2[12]);
    *out.add(13) = vqsubq_s16(step2[2], step2[13]);
    *out.add(14) = vqsubq_s16(step2[1], step2[14]);
    *out.add(15) = vqsubq_s16(step2[0], step2[15]);
}

#[inline(always)]
unsafe fn idct16_low8_neon(mut in_: *mut int16x8_t, mut out: *mut int16x8_t, mut cos_bit: i8) {
    let cospi: *const i32 = cospi_arr(cos_bit as i32);
    let mut step1: [int16x8_t; 16] = core::mem::zeroed();
    let mut step2: [int16x8_t; 16] = core::mem::zeroed();
    let c0: int16x4_t = set_s16x4_neon(((*cospi.add(32) as i16) as i16), ((*cospi.add(32) as i16) as i16), ((*cospi.add(16) as i16) as i16), ((*cospi.add(48) as i16) as i16));
    let c1: int16x4_t = set_s16x4_neon(((-(*cospi.add(32) as i16)) as i16), ((-(*cospi.add(32) as i16)) as i16), ((-(*cospi.add(16) as i16)) as i16), ((-(*cospi.add(48) as i16)) as i16));

    // stage 1
    // stage 2

    step2[0] = (*in_.add(0));
    step2[2] = (*in_.add(4));
    step2[4] = (*in_.add(2));
    step2[6] = (*in_.add(6));

    btf_16_neon((*in_.add(1)), (*cospi.add(60) as i16), (*cospi.add(4) as i16), &raw mut step2[8], &raw mut step2[15]);
    btf_16_neon((*in_.add(7)), -(*cospi.add(36) as i16), (*cospi.add(28) as i16), &raw mut step2[9], &raw mut step2[14]);
    btf_16_neon((*in_.add(5)), (*cospi.add(44) as i16), (*cospi.add(20) as i16), &raw mut step2[10], &raw mut step2[13]);
    btf_16_neon((*in_.add(3)), -(*cospi.add(52) as i16), (*cospi.add(12) as i16), &raw mut step2[11], &raw mut step2[12]);

    // stage 3

    btf_16_neon(step2[4], (*cospi.add(56) as i16), (*cospi.add(8) as i16), &raw mut step1[4], &raw mut step1[7]);
    btf_16_neon(step2[6], -(*cospi.add(40) as i16), (*cospi.add(24) as i16), &raw mut step1[5], &raw mut step1[6]);

    step1[0] = step2[0];
    step1[2] = step2[2];
    step1[8] = vqaddq_s16(step2[8], step2[9]);
    step1[9] = vqsubq_s16(step2[8], step2[9]);
    step1[10] = vqsubq_s16(step2[11], step2[10]);
    step1[11] = vqaddq_s16(step2[11], step2[10]);
    step1[12] = vqaddq_s16(step2[12], step2[13]);
    step1[13] = vqsubq_s16(step2[12], step2[13]);
    step1[14] = vqsubq_s16(step2[15], step2[14]);
    step1[15] = vqaddq_s16(step2[15], step2[14]);

    // stage 4

    btf_16_neon(step1[0], (*cospi.add(32) as i16), (*cospi.add(32) as i16), &raw mut step2[0], &raw mut step2[1]);
    btf_16_neon(step1[2], (*cospi.add(48) as i16), (*cospi.add(16) as i16), &raw mut step2[2], &raw mut step2[3]);
    btf_16_lane_2_3_neon(step1[14], step1[9], c0, &raw mut step2[14], &raw mut step2[9]);
    btf_16_lane_3_2_neon(step1[10], step1[13], c1, &raw mut step2[10], &raw mut step2[13]);

    step2[4] = vqaddq_s16(step1[4], step1[5]);
    step2[5] = vqsubq_s16(step1[4], step1[5]);
    step2[6] = vqsubq_s16(step1[7], step1[6]);
    step2[7] = vqaddq_s16(step1[7], step1[6]);
    step2[8] = step1[8];
    step2[11] = step1[11];
    step2[12] = step1[12];
    step2[15] = step1[15];

    // stage 5

    btf_16_lane_0_1_neon(step2[6], step2[5], c0, &raw mut step1[6], &raw mut step1[5]);
    step1[0] = vqaddq_s16(step2[0], step2[3]);
    step1[1] = vqaddq_s16(step2[1], step2[2]);
    step1[2] = vqsubq_s16(step2[1], step2[2]);
    step1[3] = vqsubq_s16(step2[0], step2[3]);
    step1[4] = step2[4];
    step1[7] = step2[7];
    step1[8] = vqaddq_s16(step2[8], step2[11]);
    step1[9] = vqaddq_s16(step2[9], step2[10]);
    step1[10] = vqsubq_s16(step2[9], step2[10]);
    step1[11] = vqsubq_s16(step2[8], step2[11]);
    step1[12] = vqsubq_s16(step2[15], step2[12]);
    step1[13] = vqsubq_s16(step2[14], step2[13]);
    step1[14] = vqaddq_s16(step2[14], step2[13]);
    step1[15] = vqaddq_s16(step2[15], step2[12]);

    // stage 6
    btf_16_lane_0_1_neon(step1[13], step1[10], c0, &raw mut step2[13], &raw mut step2[10]);
    btf_16_lane_0_1_neon(step1[12], step1[11], c0, &raw mut step2[12], &raw mut step2[11]);

    step2[0] = vqaddq_s16(step1[0], step1[7]);
    step2[1] = vqaddq_s16(step1[1], step1[6]);
    step2[2] = vqaddq_s16(step1[2], step1[5]);
    step2[3] = vqaddq_s16(step1[3], step1[4]);
    step2[4] = vqsubq_s16(step1[3], step1[4]);
    step2[5] = vqsubq_s16(step1[2], step1[5]);
    step2[6] = vqsubq_s16(step1[1], step1[6]);
    step2[7] = vqsubq_s16(step1[0], step1[7]);
    step2[8] = step1[8];
    step2[9] = step1[9];
    step2[14] = step1[14];
    step2[15] = step1[15];

    // stage 7

    *out.add(0) = vqaddq_s16(step2[0], step2[15]);
    *out.add(1) = vqaddq_s16(step2[1], step2[14]);
    *out.add(2) = vqaddq_s16(step2[2], step2[13]);
    *out.add(3) = vqaddq_s16(step2[3], step2[12]);
    *out.add(4) = vqaddq_s16(step2[4], step2[11]);
    *out.add(5) = vqaddq_s16(step2[5], step2[10]);
    *out.add(6) = vqaddq_s16(step2[6], step2[9]);
    *out.add(7) = vqaddq_s16(step2[7], step2[8]);
    *out.add(8) = vqsubq_s16(step2[7], step2[8]);
    *out.add(9) = vqsubq_s16(step2[6], step2[9]);
    *out.add(10) = vqsubq_s16(step2[5], step2[10]);
    *out.add(11) = vqsubq_s16(step2[4], step2[11]);
    *out.add(12) = vqsubq_s16(step2[3], step2[12]);
    *out.add(13) = vqsubq_s16(step2[2], step2[13]);
    *out.add(14) = vqsubq_s16(step2[1], step2[14]);
    *out.add(15) = vqsubq_s16(step2[0], step2[15]);
}

#[inline(always)]
unsafe fn iadst16_neon(mut in_: *mut int16x8_t, mut out: *mut int16x8_t, mut cos_bit: i8) {
    let cospi: *const i32 = cospi_arr(cos_bit as i32);

    let c0: int16x4_t = set_s16x4_neon(((*cospi.add(2) as i16) as i16), ((*cospi.add(62) as i16) as i16), ((*cospi.add(10) as i16) as i16), ((*cospi.add(54) as i16) as i16));
    let c1: int16x4_t = set_s16x4_neon(((*cospi.add(18) as i16) as i16), ((*cospi.add(46) as i16) as i16), ((*cospi.add(26) as i16) as i16), ((*cospi.add(38) as i16) as i16));
    let c2: int16x4_t = set_s16x4_neon(((*cospi.add(34) as i16) as i16), ((*cospi.add(30) as i16) as i16), ((*cospi.add(42) as i16) as i16), ((*cospi.add(22) as i16) as i16));
    let c3: int16x4_t = set_s16x4_neon(((*cospi.add(50) as i16) as i16), ((*cospi.add(14) as i16) as i16), ((*cospi.add(58) as i16) as i16), ((*cospi.add(6) as i16) as i16));
    let c4: int16x4_t = set_s16x4_neon(((*cospi.add(8) as i16) as i16), ((*cospi.add(56) as i16) as i16), ((*cospi.add(40) as i16) as i16), ((*cospi.add(24) as i16) as i16));
    let c5: int16x4_t = set_s16x4_neon(((*cospi.add(32) as i16) as i16), ((*cospi.add(32) as i16) as i16), ((*cospi.add(16) as i16) as i16), ((*cospi.add(48) as i16) as i16));

    let mut x: [int16x8_t; 16] = core::mem::zeroed();
    let mut t: [int16x8_t; 14] = core::mem::zeroed();
    let mut s0: int16x8_t = core::mem::zeroed();
    let mut s1: int16x8_t = core::mem::zeroed();
    let mut s2: int16x8_t = core::mem::zeroed();
    let mut s3: int16x8_t = core::mem::zeroed();
    let mut s4: int16x8_t = core::mem::zeroed();
    let mut s5: int16x8_t = core::mem::zeroed();
    let mut s6: int16x8_t = core::mem::zeroed();
    let mut s7: int16x8_t = core::mem::zeroed();
    let mut s8: int16x8_t = core::mem::zeroed();
    let mut s9: int16x8_t = core::mem::zeroed();
    let mut s10: int16x8_t = core::mem::zeroed();
    let mut s11: int16x8_t = core::mem::zeroed();
    let mut s12: int16x8_t = core::mem::zeroed();
    let mut s13: int16x8_t = core::mem::zeroed();
    let mut s14: int16x8_t = core::mem::zeroed();
    let mut s15: int16x8_t = core::mem::zeroed();

    // Stage 1
    x[0] = (*in_.add(15));
    x[1] = (*in_.add(0));
    x[2] = (*in_.add(13));
    x[3] = (*in_.add(2));
    x[4] = (*in_.add(11));
    x[5] = (*in_.add(4));
    x[6] = (*in_.add(9));
    x[7] = (*in_.add(6));
    x[8] = (*in_.add(7));
    x[9] = (*in_.add(8));
    x[10] = (*in_.add(5));
    x[11] = (*in_.add(10));
    x[12] = (*in_.add(3));
    x[13] = (*in_.add(12));
    x[14] = (*in_.add(1));
    x[15] = (*in_.add(14));

    // Stage 2
    btf_16_lane_0_1_neon(x[0], x[1], c0, &raw mut s0, &raw mut s1);
    btf_16_lane_2_3_neon(x[2], x[3], c0, &raw mut s2, &raw mut s3);
    btf_16_lane_0_1_neon(x[4], x[5], c1, &raw mut s4, &raw mut s5);
    btf_16_lane_2_3_neon(x[6], x[7], c1, &raw mut s6, &raw mut s7);
    btf_16_lane_0_1_neon(x[8], x[9], c2, &raw mut s8, &raw mut s9);
    btf_16_lane_2_3_neon(x[10], x[11], c2, &raw mut s10, &raw mut s11);
    btf_16_lane_0_1_neon(x[12], x[13], c3, &raw mut s12, &raw mut s13);
    btf_16_lane_2_3_neon(x[14], x[15], c3, &raw mut s14, &raw mut s15);

    // Stage 3
    x[0] = vqaddq_s16(s0, s8);
    x[1] = vqaddq_s16(s1, s9);
    x[2] = vqaddq_s16(s2, s10);
    x[3] = vqaddq_s16(s3, s11);
    x[4] = vqaddq_s16(s4, s12);
    x[5] = vqaddq_s16(s5, s13);
    x[6] = vqaddq_s16(s6, s14);
    x[7] = vqaddq_s16(s7, s15);
    x[8] = vqsubq_s16(s0, s8);
    x[9] = vqsubq_s16(s1, s9);
    x[10] = vqsubq_s16(s2, s10);
    x[11] = vqsubq_s16(s3, s11);
    x[12] = vqsubq_s16(s4, s12);
    x[13] = vqsubq_s16(s5, s13);
    x[14] = vqsubq_s16(s6, s14);
    x[15] = vqsubq_s16(s7, s15);

    // Stage 4
    t[0] = x[0];
    t[1] = x[1];
    t[2] = x[2];
    t[3] = x[3];
    t[4] = x[4];
    t[5] = x[5];
    t[6] = x[6];
    t[7] = x[7];
    btf_16_lane_0_1_neon(x[8], x[9], c4, &raw mut s8, &raw mut s9);
    btf_16_lane_2_3_neon(x[10], x[11], c4, &raw mut s10, &raw mut s11);
    btf_16_lane_1_0_neon(x[13], x[12], c4, &raw mut s13, &raw mut s12);
    btf_16_lane_3_2_neon(x[15], x[14], c4, &raw mut s15, &raw mut s14);

    // Stage 5
    x[0] = vqaddq_s16(t[0], t[4]);
    x[1] = vqaddq_s16(t[1], t[5]);
    x[2] = vqaddq_s16(t[2], t[6]);
    x[3] = vqaddq_s16(t[3], t[7]);
    x[4] = vqsubq_s16(t[0], t[4]);
    x[5] = vqsubq_s16(t[1], t[5]);
    x[6] = vqsubq_s16(t[2], t[6]);
    x[7] = vqsubq_s16(t[3], t[7]);
    x[8] = vqaddq_s16(s8, s12);
    x[9] = vqaddq_s16(s9, s13);
    x[10] = vqaddq_s16(s10, s14);
    x[11] = vqaddq_s16(s11, s15);
    x[12] = vqsubq_s16(s8, s12);
    x[13] = vqsubq_s16(s9, s13);
    x[14] = vqsubq_s16(s10, s14);
    x[15] = vqsubq_s16(s11, s15);

    // stage 6
    t[0] = x[0];
    t[1] = x[1];
    t[2] = x[2];
    t[3] = x[3];
    btf_16_lane_2_3_neon(x[4], x[5], c5, &raw mut s4, &raw mut s5);
    btf_16_lane_3_2_neon(x[7], x[6], c5, &raw mut s7, &raw mut s6);
    t[8] = x[8];
    t[9] = x[9];
    t[10] = x[10];
    t[11] = x[11];
    btf_16_lane_2_3_neon(x[12], x[13], c5, &raw mut s12, &raw mut s13);
    btf_16_lane_3_2_neon(x[15], x[14], c5, &raw mut s15, &raw mut s14);

    // Stage 7
    x[0] = vqaddq_s16(t[0], t[2]);
    x[1] = vqaddq_s16(t[1], t[3]);
    x[2] = vqsubq_s16(t[0], t[2]);
    x[3] = vqsubq_s16(t[1], t[3]);
    x[4] = vqaddq_s16(s4, s6);
    x[5] = vqaddq_s16(s5, s7);
    x[6] = vqsubq_s16(s4, s6);
    x[7] = vqsubq_s16(s5, s7);
    x[8] = vqaddq_s16(t[8], t[10]);
    x[9] = vqaddq_s16(t[9], t[11]);
    x[10] = vqsubq_s16(t[8], t[10]);
    x[11] = vqsubq_s16(t[9], t[11]);
    x[12] = vqaddq_s16(s12, s14);
    x[13] = vqaddq_s16(s13, s15);
    x[14] = vqsubq_s16(s12, s14);
    x[15] = vqsubq_s16(s13, s15);

    // Stage 8
    btf_16_half_neon(x.as_mut_ptr().add(2), c5);
    btf_16_half_neon(x.as_mut_ptr().add(6), c5);
    btf_16_half_neon(x.as_mut_ptr().add(10), c5);
    btf_16_half_neon(x.as_mut_ptr().add(14), c5);

    // Stage 9
    *out.add(0) = x[0];
    *out.add(1) = vqnegq_s16(x[8]);
    *out.add(2) = x[12];
    *out.add(3) = vqnegq_s16(x[4]);
    *out.add(4) = x[6];
    *out.add(5) = vqnegq_s16(x[14]);
    *out.add(6) = x[10];
    *out.add(7) = vqnegq_s16(x[2]);
    *out.add(8) = x[3];
    *out.add(9) = vqnegq_s16(x[11]);
    *out.add(10) = x[15];
    *out.add(11) = vqnegq_s16(x[7]);
    *out.add(12) = x[5];
    *out.add(13) = vqnegq_s16(x[13]);
    *out.add(14) = x[9];
    *out.add(15) = vqnegq_s16(x[1]);
}

#[inline(always)]
unsafe fn iadst16_low1_neon(mut in_: *mut int16x8_t, mut out: *mut int16x8_t, mut cos_bit: i8) {
    let cospi: *const i32 = cospi_arr(cos_bit as i32);
    let c0: int16x4_t = set_s16x4_neon(((*cospi.add(8) as i16) as i16), ((*cospi.add(56) as i16) as i16), ((*cospi.add(40) as i16) as i16), ((*cospi.add(24) as i16) as i16));
    let c1: int16x4_t = set_s16x4_neon(((*cospi.add(32) as i16) as i16), ((*cospi.add(32) as i16) as i16), ((*cospi.add(16) as i16) as i16), ((*cospi.add(48) as i16) as i16));

    let mut x: [int16x8_t; 16] = core::mem::zeroed();
    let mut t: [int16x8_t; 10] = core::mem::zeroed();
    let mut s0: int16x8_t = core::mem::zeroed();
    let mut s1: int16x8_t = core::mem::zeroed();
    let mut s4: int16x8_t = core::mem::zeroed();
    let mut s5: int16x8_t = core::mem::zeroed();
    let mut s8: int16x8_t = core::mem::zeroed();
    let mut s9: int16x8_t = core::mem::zeroed();
    let mut s12: int16x8_t = core::mem::zeroed();
    let mut s13: int16x8_t = core::mem::zeroed();

    // Stage 1
    x[1] = (*in_.add(0));

    // Stage 2
    btf_16_neon(x[1], (*cospi.add(62) as i16), -(*cospi.add(2) as i16), &raw mut s0, &raw mut s1);

    // Stage 3
    x[0] = s0;
    x[1] = s1;
    x[8] = s0;
    x[9] = s1;

    // Stage 4
    t[0] = x[0];
    t[1] = x[1];
    btf_16_lane_0_1_neon(x[8], x[9], c0, &raw mut s8, &raw mut s9);

    // Stage 5
    x[0] = t[0];
    x[1] = t[1];
    x[4] = t[0];
    x[5] = t[1];
    x[8] = s8;
    x[9] = s9;
    x[12] = s8;
    x[13] = s9;

    // stage 6
    t[0] = x[0];
    t[1] = x[1];
    btf_16_lane_2_3_neon(x[4], x[5], c1, &raw mut s4, &raw mut s5);
    t[8] = x[8];
    t[9] = x[9];
    btf_16_lane_2_3_neon(x[12], x[13], c1, &raw mut s12, &raw mut s13);

    // Stage 7
    x[0] = t[0];
    x[1] = t[1];
    x[2] = t[0];
    x[3] = t[1];
    x[4] = s4;
    x[5] = s5;
    x[6] = s4;
    x[7] = s5;
    x[8] = t[8];
    x[9] = t[9];
    x[10] = t[8];
    x[11] = t[9];
    x[12] = s12;
    x[13] = s13;
    x[14] = s12;
    x[15] = s13;

    // Stage 8
    btf_16_half_neon(x.as_mut_ptr().add(2), c1);
    btf_16_half_neon(x.as_mut_ptr().add(6), c1);
    btf_16_half_neon(x.as_mut_ptr().add(10), c1);
    btf_16_half_neon(x.as_mut_ptr().add(14), c1);

    // Stage 9
    *out.add(0) = x[0];
    *out.add(1) = vqnegq_s16(x[8]);
    *out.add(2) = x[12];
    *out.add(3) = vqnegq_s16(x[4]);
    *out.add(4) = x[6];
    *out.add(5) = vqnegq_s16(x[14]);
    *out.add(6) = x[10];
    *out.add(7) = vqnegq_s16(x[2]);
    *out.add(8) = x[3];
    *out.add(9) = vqnegq_s16(x[11]);
    *out.add(10) = x[15];
    *out.add(11) = vqnegq_s16(x[7]);
    *out.add(12) = x[5];
    *out.add(13) = vqnegq_s16(x[13]);
    *out.add(14) = x[9];
    *out.add(15) = vqnegq_s16(x[1]);
}

#[inline(always)]
unsafe fn iadst16_low8_neon(mut in_: *mut int16x8_t, mut out: *mut int16x8_t, mut cos_bit: i8) {
    let cospi: *const i32 = cospi_arr(cos_bit as i32);

    let c0: int16x4_t = set_s16x4_neon(((*cospi.add(8) as i16) as i16), ((*cospi.add(56) as i16) as i16), ((*cospi.add(40) as i16) as i16), ((*cospi.add(24) as i16) as i16));
    let c1: int16x4_t = set_s16x4_neon(((*cospi.add(32) as i16) as i16), ((*cospi.add(32) as i16) as i16), ((*cospi.add(16) as i16) as i16), ((*cospi.add(48) as i16) as i16));

    let mut x: [int16x8_t; 16] = core::mem::zeroed();
    let mut t: [int16x8_t; 14] = core::mem::zeroed();
    let mut s0: int16x8_t = core::mem::zeroed();
    let mut s1: int16x8_t = core::mem::zeroed();
    let mut s2: int16x8_t = core::mem::zeroed();
    let mut s3: int16x8_t = core::mem::zeroed();
    let mut s4: int16x8_t = core::mem::zeroed();
    let mut s5: int16x8_t = core::mem::zeroed();
    let mut s6: int16x8_t = core::mem::zeroed();
    let mut s7: int16x8_t = core::mem::zeroed();
    let mut s8: int16x8_t = core::mem::zeroed();
    let mut s9: int16x8_t = core::mem::zeroed();
    let mut s10: int16x8_t = core::mem::zeroed();
    let mut s11: int16x8_t = core::mem::zeroed();
    let mut s12: int16x8_t = core::mem::zeroed();
    let mut s13: int16x8_t = core::mem::zeroed();
    let mut s14: int16x8_t = core::mem::zeroed();
    let mut s15: int16x8_t = core::mem::zeroed();

    // Stage 1
    x[1] = (*in_.add(0));
    x[3] = (*in_.add(2));
    x[5] = (*in_.add(4));
    x[7] = (*in_.add(6));
    x[8] = (*in_.add(7));
    x[10] = (*in_.add(5));
    x[12] = (*in_.add(3));
    x[14] = (*in_.add(1));

    // Stage 2
    btf_16_neon(x[1], (*cospi.add(62) as i16), -(*cospi.add(2) as i16), &raw mut s0, &raw mut s1);
    btf_16_neon(x[3], (*cospi.add(54) as i16), -(*cospi.add(10) as i16), &raw mut s2, &raw mut s3);
    btf_16_neon(x[5], (*cospi.add(46) as i16), -(*cospi.add(18) as i16), &raw mut s4, &raw mut s5);
    btf_16_neon(x[7], (*cospi.add(38) as i16), -(*cospi.add(26) as i16), &raw mut s6, &raw mut s7);

    btf_16_neon(x[8], (*cospi.add(34) as i16), (*cospi.add(30) as i16), &raw mut s8, &raw mut s9);
    btf_16_neon(x[10], (*cospi.add(42) as i16), (*cospi.add(22) as i16), &raw mut s10, &raw mut s11);
    btf_16_neon(x[12], (*cospi.add(50) as i16), (*cospi.add(14) as i16), &raw mut s12, &raw mut s13);
    btf_16_neon(x[14], (*cospi.add(58) as i16), (*cospi.add(6) as i16), &raw mut s14, &raw mut s15);

    // Stage 3
    x[0] = vqaddq_s16(s0, s8);
    x[1] = vqaddq_s16(s1, s9);
    x[2] = vqaddq_s16(s2, s10);
    x[3] = vqaddq_s16(s3, s11);
    x[4] = vqaddq_s16(s4, s12);
    x[5] = vqaddq_s16(s5, s13);
    x[6] = vqaddq_s16(s6, s14);
    x[7] = vqaddq_s16(s7, s15);
    x[8] = vqsubq_s16(s0, s8);
    x[9] = vqsubq_s16(s1, s9);
    x[10] = vqsubq_s16(s2, s10);
    x[11] = vqsubq_s16(s3, s11);
    x[12] = vqsubq_s16(s4, s12);
    x[13] = vqsubq_s16(s5, s13);
    x[14] = vqsubq_s16(s6, s14);
    x[15] = vqsubq_s16(s7, s15);

    // Stage 4
    t[0] = x[0];
    t[1] = x[1];
    t[2] = x[2];
    t[3] = x[3];
    t[4] = x[4];
    t[5] = x[5];
    t[6] = x[6];
    t[7] = x[7];
    btf_16_lane_0_1_neon(x[8], x[9], c0, &raw mut s8, &raw mut s9);
    btf_16_lane_2_3_neon(x[10], x[11], c0, &raw mut s10, &raw mut s11);
    btf_16_lane_1_0_neon(x[13], x[12], c0, &raw mut s13, &raw mut s12);
    btf_16_lane_3_2_neon(x[15], x[14], c0, &raw mut s15, &raw mut s14);

    // Stage 5
    x[0] = vqaddq_s16(t[0], t[4]);
    x[1] = vqaddq_s16(t[1], t[5]);
    x[2] = vqaddq_s16(t[2], t[6]);
    x[3] = vqaddq_s16(t[3], t[7]);
    x[4] = vqsubq_s16(t[0], t[4]);
    x[5] = vqsubq_s16(t[1], t[5]);
    x[6] = vqsubq_s16(t[2], t[6]);
    x[7] = vqsubq_s16(t[3], t[7]);
    x[8] = vqaddq_s16(s8, s12);
    x[9] = vqaddq_s16(s9, s13);
    x[10] = vqaddq_s16(s10, s14);
    x[11] = vqaddq_s16(s11, s15);
    x[12] = vqsubq_s16(s8, s12);
    x[13] = vqsubq_s16(s9, s13);
    x[14] = vqsubq_s16(s10, s14);
    x[15] = vqsubq_s16(s11, s15);

    // stage 6
    t[0] = x[0];
    t[1] = x[1];
    t[2] = x[2];
    t[3] = x[3];
    btf_16_lane_2_3_neon(x[4], x[5], c1, &raw mut s4, &raw mut s5);
    btf_16_lane_3_2_neon(x[7], x[6], c1, &raw mut s7, &raw mut s6);
    t[8] = x[8];
    t[9] = x[9];
    t[10] = x[10];
    t[11] = x[11];
    btf_16_lane_2_3_neon(x[12], x[13], c1, &raw mut s12, &raw mut s13);
    btf_16_lane_3_2_neon(x[15], x[14], c1, &raw mut s15, &raw mut s14);

    // Stage 7
    x[0] = vqaddq_s16(t[0], t[2]);
    x[1] = vqaddq_s16(t[1], t[3]);
    x[2] = vqsubq_s16(t[0], t[2]);
    x[3] = vqsubq_s16(t[1], t[3]);
    x[4] = vqaddq_s16(s4, s6);
    x[5] = vqaddq_s16(s5, s7);
    x[6] = vqsubq_s16(s4, s6);
    x[7] = vqsubq_s16(s5, s7);
    x[8] = vqaddq_s16(t[8], t[10]);
    x[9] = vqaddq_s16(t[9], t[11]);
    x[10] = vqsubq_s16(t[8], t[10]);
    x[11] = vqsubq_s16(t[9], t[11]);
    x[12] = vqaddq_s16(s12, s14);
    x[13] = vqaddq_s16(s13, s15);
    x[14] = vqsubq_s16(s12, s14);
    x[15] = vqsubq_s16(s13, s15);

    // Stage 8
    btf_16_half_neon(x.as_mut_ptr().add(2), c1);
    btf_16_half_neon(x.as_mut_ptr().add(6), c1);
    btf_16_half_neon(x.as_mut_ptr().add(10), c1);
    btf_16_half_neon(x.as_mut_ptr().add(14), c1);

    // Stage 9
    *out.add(0) = x[0];
    *out.add(1) = vqnegq_s16(x[8]);
    *out.add(2) = x[12];
    *out.add(3) = vqnegq_s16(x[4]);
    *out.add(4) = x[6];
    *out.add(5) = vqnegq_s16(x[14]);
    *out.add(6) = x[10];
    *out.add(7) = vqnegq_s16(x[2]);
    *out.add(8) = x[3];
    *out.add(9) = vqnegq_s16(x[11]);
    *out.add(10) = x[15];
    *out.add(11) = vqnegq_s16(x[7]);
    *out.add(12) = x[5];
    *out.add(13) = vqnegq_s16(x[13]);
    *out.add(14) = x[9];
    *out.add(15) = vqnegq_s16(x[1]);
}

#[inline(always)]
unsafe fn idct32_neon(mut in_: *mut int16x8_t, mut out: *mut int16x8_t, mut cos_bit: i8) {
    let cospi: *const i32 = cospi_arr(cos_bit as i32);
    let mut step1: [int16x8_t; 32] = core::mem::zeroed();
    let mut step2: [int16x8_t; 32] = core::mem::zeroed();

    let c0: int16x4_t = set_s16x4_neon(((*cospi.add(2) as i16) as i16), ((*cospi.add(62) as i16) as i16), ((*cospi.add(34) as i16) as i16), ((*cospi.add(30) as i16) as i16));
    let c1: int16x4_t = set_s16x4_neon(((*cospi.add(18) as i16) as i16), ((*cospi.add(46) as i16) as i16), ((*cospi.add(50) as i16) as i16), ((*cospi.add(14) as i16) as i16));
    let c2: int16x4_t = set_s16x4_neon(((*cospi.add(10) as i16) as i16), ((*cospi.add(54) as i16) as i16), ((*cospi.add(42) as i16) as i16), ((*cospi.add(22) as i16) as i16));
    let c3: int16x4_t = set_s16x4_neon(((*cospi.add(26) as i16) as i16), ((*cospi.add(38) as i16) as i16), ((*cospi.add(58) as i16) as i16), ((*cospi.add(6) as i16) as i16));
    let c4: int16x4_t = set_s16x4_neon(((*cospi.add(4) as i16) as i16), ((*cospi.add(60) as i16) as i16), ((*cospi.add(36) as i16) as i16), ((*cospi.add(28) as i16) as i16));
    let c5: int16x4_t = set_s16x4_neon(((*cospi.add(20) as i16) as i16), ((*cospi.add(44) as i16) as i16), ((*cospi.add(52) as i16) as i16), ((*cospi.add(12) as i16) as i16));
    let c6: int16x4_t = set_s16x4_neon(((*cospi.add(8) as i16) as i16), ((*cospi.add(56) as i16) as i16), ((*cospi.add(40) as i16) as i16), ((*cospi.add(24) as i16) as i16));
    let c7: int16x4_t = set_s16x4_neon(((*cospi.add(32) as i16) as i16), ((*cospi.add(32) as i16) as i16), ((*cospi.add(16) as i16) as i16), ((*cospi.add(48) as i16) as i16));
    let c8: int16x4_t = set_s16x4_neon(((-(*cospi.add(8) as i16)) as i16), ((-(*cospi.add(56) as i16)) as i16), ((-(*cospi.add(40) as i16)) as i16), ((-(*cospi.add(24) as i16)) as i16));
    let c9: int16x4_t = set_s16x4_neon(((-(*cospi.add(32) as i16)) as i16), ((-(*cospi.add(32) as i16)) as i16), ((-(*cospi.add(16) as i16)) as i16), ((-(*cospi.add(48) as i16)) as i16));

    // stage 2

    btf_16_lane_0_1_neon((*in_.add(1)), (*in_.add(31)), c0, &raw mut step2[31], &raw mut step2[16]);
    btf_16_lane_2_3_neon((*in_.add(17)), (*in_.add(15)), c0, &raw mut step2[30], &raw mut step2[17]);
    btf_16_lane_0_1_neon((*in_.add(9)), (*in_.add(23)), c1, &raw mut step2[29], &raw mut step2[18]);
    btf_16_lane_2_3_neon((*in_.add(25)), (*in_.add(7)), c1, &raw mut step2[28], &raw mut step2[19]);
    btf_16_lane_0_1_neon((*in_.add(5)), (*in_.add(27)), c2, &raw mut step2[27], &raw mut step2[20]);
    btf_16_lane_2_3_neon((*in_.add(21)), (*in_.add(11)), c2, &raw mut step2[26], &raw mut step2[21]);
    btf_16_lane_0_1_neon((*in_.add(13)), (*in_.add(19)), c3, &raw mut step2[25], &raw mut step2[22]);
    btf_16_lane_2_3_neon((*in_.add(29)), (*in_.add(3)), c3, &raw mut step2[24], &raw mut step2[23]);

    step2[0] = (*in_.add(0));
    step2[1] = (*in_.add(16));
    step2[2] = (*in_.add(8));
    step2[3] = (*in_.add(24));
    step2[4] = (*in_.add(4));
    step2[5] = (*in_.add(20));
    step2[6] = (*in_.add(12));
    step2[7] = (*in_.add(28));
    step2[8] = (*in_.add(2));
    step2[9] = (*in_.add(18));
    step2[10] = (*in_.add(10));
    step2[11] = (*in_.add(26));
    step2[12] = (*in_.add(6));
    step2[13] = (*in_.add(22));
    step2[14] = (*in_.add(14));
    step2[15] = (*in_.add(30));

    // stage 3

    btf_16_lane_0_1_neon(step2[8], step2[15], c4, &raw mut step1[15], &raw mut step1[8]);
    btf_16_lane_2_3_neon(step2[9], step2[14], c4, &raw mut step1[14], &raw mut step1[9]);
    btf_16_lane_0_1_neon(step2[10], step2[13], c5, &raw mut step1[13], &raw mut step1[10]);
    btf_16_lane_2_3_neon(step2[11], step2[12], c5, &raw mut step1[12], &raw mut step1[11]);

    step1[0] = step2[0];
    step1[1] = step2[1];
    step1[2] = step2[2];
    step1[3] = step2[3];
    step1[4] = step2[4];
    step1[5] = step2[5];
    step1[6] = step2[6];
    step1[7] = step2[7];

    step1[16] = vqaddq_s16(step2[16], step2[17]);
    step1[17] = vqsubq_s16(step2[16], step2[17]);
    step1[18] = vqsubq_s16(step2[19], step2[18]);
    step1[19] = vqaddq_s16(step2[19], step2[18]);
    step1[20] = vqaddq_s16(step2[20], step2[21]);
    step1[21] = vqsubq_s16(step2[20], step2[21]);
    step1[22] = vqsubq_s16(step2[23], step2[22]);
    step1[23] = vqaddq_s16(step2[23], step2[22]);
    step1[24] = vqaddq_s16(step2[24], step2[25]);
    step1[25] = vqsubq_s16(step2[24], step2[25]);
    step1[26] = vqsubq_s16(step2[27], step2[26]);
    step1[27] = vqaddq_s16(step2[27], step2[26]);
    step1[28] = vqaddq_s16(step2[28], step2[29]);
    step1[29] = vqsubq_s16(step2[28], step2[29]);
    step1[30] = vqsubq_s16(step2[31], step2[30]);
    step1[31] = vqaddq_s16(step2[31], step2[30]);

    // stage 4

    btf_16_lane_0_1_neon(step1[4], step1[7], c6, &raw mut step2[7], &raw mut step2[4]);
    btf_16_lane_2_3_neon(step1[5], step1[6], c6, &raw mut step2[6], &raw mut step2[5]);
    btf_16_lane_0_1_neon(step1[30], step1[17], c6, &raw mut step2[30], &raw mut step2[17]);
    btf_16_lane_1_0_neon(step1[18], step1[29], c8, &raw mut step2[18], &raw mut step2[29]);
    btf_16_lane_2_3_neon(step1[26], step1[21], c6, &raw mut step2[26], &raw mut step2[21]);
    btf_16_lane_3_2_neon(step1[22], step1[25], c8, &raw mut step2[22], &raw mut step2[25]);

    step2[0] = step1[0];
    step2[1] = step1[1];
    step2[2] = step1[2];
    step2[3] = step1[3];
    step2[8] = vqaddq_s16(step1[8], step1[9]);
    step2[9] = vqsubq_s16(step1[8], step1[9]);
    step2[10] = vqsubq_s16(step1[11], step1[10]);
    step2[11] = vqaddq_s16(step1[11], step1[10]);
    step2[12] = vqaddq_s16(step1[12], step1[13]);
    step2[13] = vqsubq_s16(step1[12], step1[13]);
    step2[14] = vqsubq_s16(step1[15], step1[14]);
    step2[15] = vqaddq_s16(step1[15], step1[14]);
    step2[16] = step1[16];
    step2[19] = step1[19];
    step2[20] = step1[20];
    step2[23] = step1[23];
    step2[24] = step1[24];
    step2[27] = step1[27];
    step2[28] = step1[28];
    step2[31] = step1[31];

    // stage 5

    btf_16_lane_0_1_neon(step2[0], step2[1], c7, &raw mut step1[0], &raw mut step1[1]);
    btf_16_lane_2_3_neon(step2[2], step2[3], c7, &raw mut step1[3], &raw mut step1[2]);
    btf_16_lane_2_3_neon(step2[14], step2[9], c7, &raw mut step1[14], &raw mut step1[9]);
    btf_16_lane_3_2_neon(step2[10], step2[13], c9, &raw mut step1[10], &raw mut step1[13]);

    step1[4] = vqaddq_s16(step2[4], step2[5]);
    step1[5] = vqsubq_s16(step2[4], step2[5]);
    step1[6] = vqsubq_s16(step2[7], step2[6]);
    step1[7] = vqaddq_s16(step2[7], step2[6]);
    step1[8] = step2[8];
    step1[11] = step2[11];
    step1[12] = step2[12];
    step1[15] = step2[15];
    step1[16] = vqaddq_s16(step2[16], step2[19]);
    step1[17] = vqaddq_s16(step2[17], step2[18]);
    step1[18] = vqsubq_s16(step2[17], step2[18]);
    step1[19] = vqsubq_s16(step2[16], step2[19]);
    step1[20] = vqsubq_s16(step2[23], step2[20]);
    step1[21] = vqsubq_s16(step2[22], step2[21]);
    step1[22] = vqaddq_s16(step2[22], step2[21]);
    step1[23] = vqaddq_s16(step2[23], step2[20]);
    step1[24] = vqaddq_s16(step2[24], step2[27]);
    step1[25] = vqaddq_s16(step2[25], step2[26]);
    step1[26] = vqsubq_s16(step2[25], step2[26]);
    step1[27] = vqsubq_s16(step2[24], step2[27]);
    step1[28] = vqsubq_s16(step2[31], step2[28]);
    step1[29] = vqsubq_s16(step2[30], step2[29]);
    step1[30] = vqaddq_s16(step2[30], step2[29]);
    step1[31] = vqaddq_s16(step2[31], step2[28]);

    // stage 6

    btf_16_lane_0_1_neon(step1[6], step1[5], c7, &raw mut step2[6], &raw mut step2[5]);
    btf_16_lane_2_3_neon(step1[29], step1[18], c7, &raw mut step2[29], &raw mut step2[18]);
    btf_16_lane_2_3_neon(step1[28], step1[19], c7, &raw mut step2[28], &raw mut step2[19]);
    btf_16_lane_3_2_neon(step1[20], step1[27], c9, &raw mut step2[20], &raw mut step2[27]);
    btf_16_lane_3_2_neon(step1[21], step1[26], c9, &raw mut step2[21], &raw mut step2[26]);

    step2[0] = vqaddq_s16(step1[0], step1[3]);
    step2[1] = vqaddq_s16(step1[1], step1[2]);
    step2[2] = vqsubq_s16(step1[1], step1[2]);
    step2[3] = vqsubq_s16(step1[0], step1[3]);
    step2[4] = step1[4];
    step2[7] = step1[7];
    step2[8] = vqaddq_s16(step1[8], step1[11]);
    step2[9] = vqaddq_s16(step1[9], step1[10]);
    step2[10] = vqsubq_s16(step1[9], step1[10]);
    step2[11] = vqsubq_s16(step1[8], step1[11]);
    step2[12] = vqsubq_s16(step1[15], step1[12]);
    step2[13] = vqsubq_s16(step1[14], step1[13]);
    step2[14] = vqaddq_s16(step1[14], step1[13]);
    step2[15] = vqaddq_s16(step1[15], step1[12]);
    step2[16] = step1[16];
    step2[17] = step1[17];
    step2[22] = step1[22];
    step2[23] = step1[23];
    step2[24] = step1[24];
    step2[25] = step1[25];
    step2[30] = step1[30];
    step2[31] = step1[31];

    // stage 7

    btf_16_lane_0_1_neon(step2[13], step2[10], c7, &raw mut step1[13], &raw mut step1[10]);
    btf_16_lane_0_1_neon(step2[12], step2[11], c7, &raw mut step1[12], &raw mut step1[11]);

    step1[0] = vqaddq_s16(step2[0], step2[7]);
    step1[1] = vqaddq_s16(step2[1], step2[6]);
    step1[2] = vqaddq_s16(step2[2], step2[5]);
    step1[3] = vqaddq_s16(step2[3], step2[4]);
    step1[4] = vqsubq_s16(step2[3], step2[4]);
    step1[5] = vqsubq_s16(step2[2], step2[5]);
    step1[6] = vqsubq_s16(step2[1], step2[6]);
    step1[7] = vqsubq_s16(step2[0], step2[7]);
    step1[8] = step2[8];
    step1[9] = step2[9];
    step1[14] = step2[14];
    step1[15] = step2[15];
    step1[16] = vqaddq_s16(step2[16], step2[23]);
    step1[17] = vqaddq_s16(step2[17], step2[22]);
    step1[18] = vqaddq_s16(step2[18], step2[21]);
    step1[19] = vqaddq_s16(step2[19], step2[20]);
    step1[20] = vqsubq_s16(step2[19], step2[20]);
    step1[21] = vqsubq_s16(step2[18], step2[21]);
    step1[22] = vqsubq_s16(step2[17], step2[22]);
    step1[23] = vqsubq_s16(step2[16], step2[23]);
    step1[24] = vqsubq_s16(step2[31], step2[24]);
    step1[25] = vqsubq_s16(step2[30], step2[25]);
    step1[26] = vqsubq_s16(step2[29], step2[26]);
    step1[27] = vqsubq_s16(step2[28], step2[27]);
    step1[28] = vqaddq_s16(step2[27], step2[28]);
    step1[29] = vqaddq_s16(step2[26], step2[29]);
    step1[30] = vqaddq_s16(step2[25], step2[30]);
    step1[31] = vqaddq_s16(step2[24], step2[31]);

    // stage 8

    btf_16_lane_0_1_neon(step1[27], step1[20], c7, &raw mut step2[27], &raw mut step2[20]);
    btf_16_lane_0_1_neon(step1[26], step1[21], c7, &raw mut step2[26], &raw mut step2[21]);
    btf_16_lane_0_1_neon(step1[25], step1[22], c7, &raw mut step2[25], &raw mut step2[22]);
    btf_16_lane_0_1_neon(step1[24], step1[23], c7, &raw mut step2[24], &raw mut step2[23]);

    step2[0] = vqaddq_s16(step1[0], step1[15]);
    step2[1] = vqaddq_s16(step1[1], step1[14]);
    step2[2] = vqaddq_s16(step1[2], step1[13]);
    step2[3] = vqaddq_s16(step1[3], step1[12]);
    step2[4] = vqaddq_s16(step1[4], step1[11]);
    step2[5] = vqaddq_s16(step1[5], step1[10]);
    step2[6] = vqaddq_s16(step1[6], step1[9]);
    step2[7] = vqaddq_s16(step1[7], step1[8]);
    step2[8] = vqsubq_s16(step1[7], step1[8]);
    step2[9] = vqsubq_s16(step1[6], step1[9]);
    step2[10] = vqsubq_s16(step1[5], step1[10]);
    step2[11] = vqsubq_s16(step1[4], step1[11]);
    step2[12] = vqsubq_s16(step1[3], step1[12]);
    step2[13] = vqsubq_s16(step1[2], step1[13]);
    step2[14] = vqsubq_s16(step1[1], step1[14]);
    step2[15] = vqsubq_s16(step1[0], step1[15]);
    step2[16] = step1[16];
    step2[17] = step1[17];
    step2[18] = step1[18];
    step2[19] = step1[19];
    step2[28] = step1[28];
    step2[29] = step1[29];
    step2[30] = step1[30];
    step2[31] = step1[31];

    // stage 9

    *out.add(0) = vqaddq_s16(step2[0], step2[31]);
    *out.add(1) = vqaddq_s16(step2[1], step2[30]);
    *out.add(2) = vqaddq_s16(step2[2], step2[29]);
    *out.add(3) = vqaddq_s16(step2[3], step2[28]);
    *out.add(4) = vqaddq_s16(step2[4], step2[27]);
    *out.add(5) = vqaddq_s16(step2[5], step2[26]);
    *out.add(6) = vqaddq_s16(step2[6], step2[25]);
    *out.add(7) = vqaddq_s16(step2[7], step2[24]);
    *out.add(8) = vqaddq_s16(step2[8], step2[23]);
    *out.add(9) = vqaddq_s16(step2[9], step2[22]);
    *out.add(10) = vqaddq_s16(step2[10], step2[21]);
    *out.add(11) = vqaddq_s16(step2[11], step2[20]);
    *out.add(12) = vqaddq_s16(step2[12], step2[19]);
    *out.add(13) = vqaddq_s16(step2[13], step2[18]);
    *out.add(14) = vqaddq_s16(step2[14], step2[17]);
    *out.add(15) = vqaddq_s16(step2[15], step2[16]);
    *out.add(16) = vqsubq_s16(step2[15], step2[16]);
    *out.add(17) = vqsubq_s16(step2[14], step2[17]);
    *out.add(18) = vqsubq_s16(step2[13], step2[18]);
    *out.add(19) = vqsubq_s16(step2[12], step2[19]);
    *out.add(20) = vqsubq_s16(step2[11], step2[20]);
    *out.add(21) = vqsubq_s16(step2[10], step2[21]);
    *out.add(22) = vqsubq_s16(step2[9], step2[22]);
    *out.add(23) = vqsubq_s16(step2[8], step2[23]);
    *out.add(24) = vqsubq_s16(step2[7], step2[24]);
    *out.add(25) = vqsubq_s16(step2[6], step2[25]);
    *out.add(26) = vqsubq_s16(step2[5], step2[26]);
    *out.add(27) = vqsubq_s16(step2[4], step2[27]);
    *out.add(28) = vqsubq_s16(step2[3], step2[28]);
    *out.add(29) = vqsubq_s16(step2[2], step2[29]);
    *out.add(30) = vqsubq_s16(step2[1], step2[30]);
    *out.add(31) = vqsubq_s16(step2[0], step2[31]);
}

#[inline(always)]
unsafe fn idct32_low1_neon(mut in_: *mut int16x8_t, mut out: *mut int16x8_t, mut cos_bit: i8) {
    let cospi: *const i32 = cospi_arr(cos_bit as i32);
    let mut step1: int16x8_t = core::mem::zeroed();
    let mut t32: [int32x4_t; 2] = core::mem::zeroed();

    // stage 1
    // stage 2
    // stage 3
    // stage 4
    // stage 5

    t32[0] = vmull_n_s16(vget_low_s16((*in_.add(0))), (*cospi.add(32) as i16));
    t32[1] = vmull_n_s16(vget_high_s16((*in_.add(0))), (*cospi.add(32) as i16));
    step1 = vcombine_s16(vrshrn_n_s32(t32[0], INV_COS_BIT), vrshrn_n_s32(t32[1], INV_COS_BIT));

    // stage 6
    // stage 7
    // stage 8
    // stage 9

    *out.add(0) = step1;
    *out.add(1) = step1;
    *out.add(2) = step1;
    *out.add(3) = step1;
    *out.add(4) = step1;
    *out.add(5) = step1;
    *out.add(6) = step1;
    *out.add(7) = step1;
    *out.add(8) = step1;
    *out.add(9) = step1;
    *out.add(10) = step1;
    *out.add(11) = step1;
    *out.add(12) = step1;
    *out.add(13) = step1;
    *out.add(14) = step1;
    *out.add(15) = step1;
    *out.add(16) = step1;
    *out.add(17) = step1;
    *out.add(18) = step1;
    *out.add(19) = step1;
    *out.add(20) = step1;
    *out.add(21) = step1;
    *out.add(22) = step1;
    *out.add(23) = step1;
    *out.add(24) = step1;
    *out.add(25) = step1;
    *out.add(26) = step1;
    *out.add(27) = step1;
    *out.add(28) = step1;
    *out.add(29) = step1;
    *out.add(30) = step1;
    *out.add(31) = step1;
}

#[inline(always)]
unsafe fn idct32_low8_neon(mut in_: *mut int16x8_t, mut out: *mut int16x8_t, mut cos_bit: i8) {
    let cospi: *const i32 = cospi_arr(cos_bit as i32);
    let mut step1: [int16x8_t; 32] = core::mem::zeroed();
    let mut step2: [int16x8_t; 32] = core::mem::zeroed();
    let mut t32: [int32x4_t; 16] = core::mem::zeroed();
    let c0: int16x4_t = set_s16x4_neon(((*cospi.add(8) as i16) as i16), ((*cospi.add(56) as i16) as i16), ((*cospi.add(40) as i16) as i16), ((*cospi.add(24) as i16) as i16));
    let c1: int16x4_t = set_s16x4_neon(((*cospi.add(32) as i16) as i16), ((*cospi.add(32) as i16) as i16), ((*cospi.add(16) as i16) as i16), (*cospi.add(48) as i16));
    let c2: int16x4_t = set_s16x4_neon(((-(*cospi.add(8) as i16)) as i16), ((-(*cospi.add(56) as i16)) as i16), ((-(*cospi.add(40) as i16)) as i16), ((-(*cospi.add(24) as i16)) as i16));
    let c3: int16x4_t = set_s16x4_neon(((-(*cospi.add(32) as i16)) as i16), ((-(*cospi.add(32) as i16)) as i16), ((-(*cospi.add(16) as i16)) as i16), ((-(*cospi.add(48) as i16)) as i16));
    // stage 1
    // stage 2

    step2[0] = (*in_.add(0));
    step2[4] = (*in_.add(4));
    step2[8] = (*in_.add(2));
    step2[12] = (*in_.add(6));

    btf_16_neon((*in_.add(1)), (*cospi.add(62) as i16), (*cospi.add(2) as i16), &raw mut step2[16], &raw mut step2[31]);
    btf_16_neon((*in_.add(7)), -(*cospi.add(50) as i16), (*cospi.add(14) as i16), &raw mut step2[19], &raw mut step2[28]);
    btf_16_neon((*in_.add(5)), (*cospi.add(54) as i16), (*cospi.add(10) as i16), &raw mut step2[20], &raw mut step2[27]);
    btf_16_neon((*in_.add(3)), -(*cospi.add(58) as i16), (*cospi.add(6) as i16), &raw mut step2[23], &raw mut step2[24]);

    // stage 3
    step1[0] = step2[0];
    step1[4] = step2[4];

    btf_16_neon(step2[8], (*cospi.add(60) as i16), (*cospi.add(4) as i16), &raw mut step1[8], &raw mut step1[15]);
    btf_16_neon(step2[12], -(*cospi.add(52) as i16), (*cospi.add(12) as i16), &raw mut step1[11], &raw mut step1[12]);

    step1[16] = step2[16];
    step1[17] = step2[16];
    step1[18] = step2[19];
    step1[19] = step2[19];
    step1[20] = step2[20];
    step1[21] = step2[20];
    step1[22] = step2[23];
    step1[23] = step2[23];
    step1[24] = step2[24];
    step1[25] = step2[24];
    step1[26] = step2[27];
    step1[27] = step2[27];
    step1[28] = step2[28];
    step1[29] = step2[28];
    step1[30] = step2[31];
    step1[31] = step2[31];

    // stage 4

    btf_16_neon(step1[4], (*cospi.add(56) as i16), (*cospi.add(8) as i16), &raw mut step2[4], &raw mut step2[7]);
    btf_16_lane_0_1_neon(step1[30], step1[17], c0, &raw mut step2[30], &raw mut step2[17]);
    btf_16_lane_1_0_neon(step1[18], step1[29], c2, &raw mut step2[18], &raw mut step2[29]);
    btf_16_lane_2_3_neon(step1[26], step1[21], c0, &raw mut step2[26], &raw mut step2[21]);
    btf_16_lane_3_2_neon(step1[22], step1[25], c2, &raw mut step2[22], &raw mut step2[25]);

    step2[0] = step1[0];
    step2[8] = step1[8];
    step2[9] = step1[8];
    step2[10] = step1[11];
    step2[11] = step1[11];
    step2[12] = step1[12];
    step2[13] = step1[12];
    step2[14] = step1[15];
    step2[15] = step1[15];
    step2[16] = step1[16];
    step2[19] = step1[19];
    step2[20] = step1[20];
    step2[23] = step1[23];
    step2[24] = step1[24];
    step2[27] = step1[27];
    step2[28] = step1[28];
    step2[31] = step1[31];

    // stage 5

    t32[0] = vmull_n_s16(vget_low_s16(step2[0]), (*cospi.add(32) as i16));
    t32[1] = vmull_n_s16(vget_high_s16(step2[0]), (*cospi.add(32) as i16));
    step1[0] = vcombine_s16(vrshrn_n_s32(t32[0], INV_COS_BIT), vrshrn_n_s32(t32[1], INV_COS_BIT));

    btf_16_lane_2_3_neon(step2[14], step2[9], c1, &raw mut step1[14], &raw mut step1[9]);
    btf_16_lane_3_2_neon(step2[10], step2[13], c3, &raw mut step1[10], &raw mut step1[13]);

    step1[4] = step2[4];
    step1[5] = step2[4];
    step1[6] = step2[7];
    step1[7] = step2[7];
    step1[8] = step2[8];
    step1[11] = step2[11];
    step1[12] = step2[12];
    step1[15] = step2[15];
    step1[16] = vqaddq_s16(step2[16], step2[19]);
    step1[17] = vqaddq_s16(step2[17], step2[18]);
    step1[18] = vqsubq_s16(step2[17], step2[18]);
    step1[19] = vqsubq_s16(step2[16], step2[19]);
    step1[20] = vqsubq_s16(step2[23], step2[20]);
    step1[21] = vqsubq_s16(step2[22], step2[21]);
    step1[22] = vqaddq_s16(step2[22], step2[21]);
    step1[23] = vqaddq_s16(step2[23], step2[20]);
    step1[24] = vqaddq_s16(step2[24], step2[27]);
    step1[25] = vqaddq_s16(step2[25], step2[26]);
    step1[26] = vqsubq_s16(step2[25], step2[26]);
    step1[27] = vqsubq_s16(step2[24], step2[27]);
    step1[28] = vqsubq_s16(step2[31], step2[28]);
    step1[29] = vqsubq_s16(step2[30], step2[29]);
    step1[30] = vqaddq_s16(step2[30], step2[29]);
    step1[31] = vqaddq_s16(step2[31], step2[28]);

    // stage 6

    btf_16_lane_0_1_neon(step1[6], step1[5], c1, &raw mut step2[6], &raw mut step2[5]);
    btf_16_lane_2_3_neon(step1[29], step1[18], c1, &raw mut step2[29], &raw mut step2[18]);
    btf_16_lane_2_3_neon(step1[28], step1[19], c1, &raw mut step2[28], &raw mut step2[19]);
    btf_16_lane_3_2_neon(step1[20], step1[27], c3, &raw mut step2[20], &raw mut step2[27]);
    btf_16_lane_3_2_neon(step1[21], step1[26], c3, &raw mut step2[21], &raw mut step2[26]);

    step2[0] = step1[0];
    step2[1] = step1[0];
    step2[2] = step1[0];
    step2[3] = step1[0];
    step2[4] = step1[4];
    step2[7] = step1[7];
    step2[8] = vqaddq_s16(step1[8], step1[11]);
    step2[9] = vqaddq_s16(step1[9], step1[10]);
    step2[10] = vqsubq_s16(step1[9], step1[10]);
    step2[11] = vqsubq_s16(step1[8], step1[11]);
    step2[12] = vqsubq_s16(step1[15], step1[12]);
    step2[13] = vqsubq_s16(step1[14], step1[13]);
    step2[14] = vqaddq_s16(step1[14], step1[13]);
    step2[15] = vqaddq_s16(step1[15], step1[12]);
    step2[16] = step1[16];
    step2[17] = step1[17];
    step2[22] = step1[22];
    step2[23] = step1[23];
    step2[24] = step1[24];
    step2[25] = step1[25];
    step2[30] = step1[30];
    step2[31] = step1[31];

    // stage 7

    btf_16_lane_0_1_neon(step2[13], step2[10], c1, &raw mut step1[13], &raw mut step1[10]);
    btf_16_lane_0_1_neon(step2[12], step2[11], c1, &raw mut step1[12], &raw mut step1[11]);

    step1[0] = vqaddq_s16(step2[0], step2[7]);
    step1[1] = vqaddq_s16(step2[1], step2[6]);
    step1[2] = vqaddq_s16(step2[2], step2[5]);
    step1[3] = vqaddq_s16(step2[3], step2[4]);
    step1[4] = vqsubq_s16(step2[3], step2[4]);
    step1[5] = vqsubq_s16(step2[2], step2[5]);
    step1[6] = vqsubq_s16(step2[1], step2[6]);
    step1[7] = vqsubq_s16(step2[0], step2[7]);
    step1[8] = step2[8];
    step1[9] = step2[9];
    step1[14] = step2[14];
    step1[15] = step2[15];
    step1[16] = vqaddq_s16(step2[16], step2[23]);
    step1[17] = vqaddq_s16(step2[17], step2[22]);
    step1[18] = vqaddq_s16(step2[18], step2[21]);
    step1[19] = vqaddq_s16(step2[19], step2[20]);
    step1[20] = vqsubq_s16(step2[19], step2[20]);
    step1[21] = vqsubq_s16(step2[18], step2[21]);
    step1[22] = vqsubq_s16(step2[17], step2[22]);
    step1[23] = vqsubq_s16(step2[16], step2[23]);
    step1[24] = vqsubq_s16(step2[31], step2[24]);
    step1[25] = vqsubq_s16(step2[30], step2[25]);
    step1[26] = vqsubq_s16(step2[29], step2[26]);
    step1[27] = vqsubq_s16(step2[28], step2[27]);
    step1[28] = vqaddq_s16(step2[27], step2[28]);
    step1[29] = vqaddq_s16(step2[26], step2[29]);
    step1[30] = vqaddq_s16(step2[25], step2[30]);
    step1[31] = vqaddq_s16(step2[24], step2[31]);

    // stage 8

    btf_16_lane_0_1_neon(step1[27], step1[20], c1, &raw mut step2[27], &raw mut step2[20]);
    btf_16_lane_0_1_neon(step1[26], step1[21], c1, &raw mut step2[26], &raw mut step2[21]);
    btf_16_lane_0_1_neon(step1[25], step1[22], c1, &raw mut step2[25], &raw mut step2[22]);
    btf_16_lane_0_1_neon(step1[24], step1[23], c1, &raw mut step2[24], &raw mut step2[23]);

    step2[0] = vqaddq_s16(step1[0], step1[15]);
    step2[1] = vqaddq_s16(step1[1], step1[14]);
    step2[2] = vqaddq_s16(step1[2], step1[13]);
    step2[3] = vqaddq_s16(step1[3], step1[12]);
    step2[4] = vqaddq_s16(step1[4], step1[11]);
    step2[5] = vqaddq_s16(step1[5], step1[10]);
    step2[6] = vqaddq_s16(step1[6], step1[9]);
    step2[7] = vqaddq_s16(step1[7], step1[8]);
    step2[8] = vqsubq_s16(step1[7], step1[8]);
    step2[9] = vqsubq_s16(step1[6], step1[9]);
    step2[10] = vqsubq_s16(step1[5], step1[10]);
    step2[11] = vqsubq_s16(step1[4], step1[11]);
    step2[12] = vqsubq_s16(step1[3], step1[12]);
    step2[13] = vqsubq_s16(step1[2], step1[13]);
    step2[14] = vqsubq_s16(step1[1], step1[14]);
    step2[15] = vqsubq_s16(step1[0], step1[15]);
    step2[16] = step1[16];
    step2[17] = step1[17];
    step2[18] = step1[18];
    step2[19] = step1[19];
    step2[28] = step1[28];
    step2[29] = step1[29];
    step2[30] = step1[30];
    step2[31] = step1[31];

    // stage 9

    *out.add(0) = vqaddq_s16(step2[0], step2[31]);
    *out.add(1) = vqaddq_s16(step2[1], step2[30]);
    *out.add(2) = vqaddq_s16(step2[2], step2[29]);
    *out.add(3) = vqaddq_s16(step2[3], step2[28]);
    *out.add(4) = vqaddq_s16(step2[4], step2[27]);
    *out.add(5) = vqaddq_s16(step2[5], step2[26]);
    *out.add(6) = vqaddq_s16(step2[6], step2[25]);
    *out.add(7) = vqaddq_s16(step2[7], step2[24]);
    *out.add(8) = vqaddq_s16(step2[8], step2[23]);
    *out.add(9) = vqaddq_s16(step2[9], step2[22]);
    *out.add(10) = vqaddq_s16(step2[10], step2[21]);
    *out.add(11) = vqaddq_s16(step2[11], step2[20]);
    *out.add(12) = vqaddq_s16(step2[12], step2[19]);
    *out.add(13) = vqaddq_s16(step2[13], step2[18]);
    *out.add(14) = vqaddq_s16(step2[14], step2[17]);
    *out.add(15) = vqaddq_s16(step2[15], step2[16]);
    *out.add(16) = vqsubq_s16(step2[15], step2[16]);
    *out.add(17) = vqsubq_s16(step2[14], step2[17]);
    *out.add(18) = vqsubq_s16(step2[13], step2[18]);
    *out.add(19) = vqsubq_s16(step2[12], step2[19]);
    *out.add(20) = vqsubq_s16(step2[11], step2[20]);
    *out.add(21) = vqsubq_s16(step2[10], step2[21]);
    *out.add(22) = vqsubq_s16(step2[9], step2[22]);
    *out.add(23) = vqsubq_s16(step2[8], step2[23]);
    *out.add(24) = vqsubq_s16(step2[7], step2[24]);
    *out.add(25) = vqsubq_s16(step2[6], step2[25]);
    *out.add(26) = vqsubq_s16(step2[5], step2[26]);
    *out.add(27) = vqsubq_s16(step2[4], step2[27]);
    *out.add(28) = vqsubq_s16(step2[3], step2[28]);
    *out.add(29) = vqsubq_s16(step2[2], step2[29]);
    *out.add(30) = vqsubq_s16(step2[1], step2[30]);
    *out.add(31) = vqsubq_s16(step2[0], step2[31]);
}

#[inline(always)]
unsafe fn idct32_low16_neon(mut in_: *mut int16x8_t, mut out: *mut int16x8_t, mut cos_bit: i8) {
    let cospi: *const i32 = cospi_arr(cos_bit as i32);
    let mut step1: [int16x8_t; 32] = core::mem::zeroed();
    let mut step2: [int16x8_t; 32] = core::mem::zeroed();
    let mut t32: [int32x4_t; 16] = core::mem::zeroed();
    let c0: int16x4_t = set_s16x4_neon(((*cospi.add(8) as i16) as i16), ((*cospi.add(56) as i16) as i16), ((*cospi.add(40) as i16) as i16), ((*cospi.add(24) as i16) as i16));
    let c1: int16x4_t = set_s16x4_neon(((*cospi.add(32) as i16) as i16), ((*cospi.add(32) as i16) as i16), ((*cospi.add(16) as i16) as i16), ((*cospi.add(48) as i16) as i16));
    let c2: int16x4_t = set_s16x4_neon(((-(*cospi.add(8) as i16)) as i16), ((-(*cospi.add(56) as i16)) as i16), ((-(*cospi.add(40) as i16)) as i16), ((-(*cospi.add(24) as i16)) as i16));
    let c3: int16x4_t = set_s16x4_neon(((-(*cospi.add(32) as i16)) as i16), ((-(*cospi.add(32) as i16)) as i16), ((-(*cospi.add(16) as i16)) as i16), ((-(*cospi.add(48) as i16)) as i16));

    // stage 1
    // stage 2

    btf_16_neon((*in_.add(1)), (*cospi.add(62) as i16), (*cospi.add(2) as i16), &raw mut step2[16], &raw mut step2[31]);
    btf_16_neon((*in_.add(15)), -(*cospi.add(34) as i16), (*cospi.add(30) as i16), &raw mut step2[17], &raw mut step2[30]);
    btf_16_neon((*in_.add(9)), (*cospi.add(46) as i16), (*cospi.add(18) as i16), &raw mut step2[18], &raw mut step2[29]);
    btf_16_neon((*in_.add(7)), -(*cospi.add(50) as i16), (*cospi.add(14) as i16), &raw mut step2[19], &raw mut step2[28]);
    btf_16_neon((*in_.add(5)), (*cospi.add(54) as i16), (*cospi.add(10) as i16), &raw mut step2[20], &raw mut step2[27]);
    btf_16_neon((*in_.add(11)), -(*cospi.add(42) as i16), (*cospi.add(22) as i16), &raw mut step2[21], &raw mut step2[26]);
    btf_16_neon((*in_.add(13)), (*cospi.add(38) as i16), (*cospi.add(26) as i16), &raw mut step2[22], &raw mut step2[25]);
    btf_16_neon((*in_.add(3)), -(*cospi.add(58) as i16), (*cospi.add(6) as i16), &raw mut step2[23], &raw mut step2[24]);

    step2[0] = (*in_.add(0));
    step2[2] = (*in_.add(8));
    step2[4] = (*in_.add(4));
    step2[6] = (*in_.add(12));
    step2[8] = (*in_.add(2));
    step2[10] = (*in_.add(10));
    step2[12] = (*in_.add(6));
    step2[14] = (*in_.add(14));

    // stage 3

    btf_16_neon(step2[8], (*cospi.add(60) as i16), (*cospi.add(4) as i16), &raw mut step1[8], &raw mut step1[15]);
    btf_16_neon(step2[14], -(*cospi.add(36) as i16), (*cospi.add(28) as i16), &raw mut step1[9], &raw mut step1[14]);
    btf_16_neon(step2[10], (*cospi.add(44) as i16), (*cospi.add(20) as i16), &raw mut step1[10], &raw mut step1[13]);
    btf_16_neon(step2[12], -(*cospi.add(52) as i16), (*cospi.add(12) as i16), &raw mut step1[11], &raw mut step1[12]);

    step1[0] = step2[0];
    step1[2] = step2[2];
    step1[4] = step2[4];
    step1[6] = step2[6];
    step1[16] = vqaddq_s16(step2[16], step2[17]);
    step1[17] = vqsubq_s16(step2[16], step2[17]);
    step1[18] = vqsubq_s16(step2[19], step2[18]);
    step1[19] = vqaddq_s16(step2[19], step2[18]);
    step1[20] = vqaddq_s16(step2[20], step2[21]);
    step1[21] = vqsubq_s16(step2[20], step2[21]);
    step1[22] = vqsubq_s16(step2[23], step2[22]);
    step1[23] = vqaddq_s16(step2[23], step2[22]);
    step1[24] = vqaddq_s16(step2[24], step2[25]);
    step1[25] = vqsubq_s16(step2[24], step2[25]);
    step1[26] = vqsubq_s16(step2[27], step2[26]);
    step1[27] = vqaddq_s16(step2[27], step2[26]);
    step1[28] = vqaddq_s16(step2[28], step2[29]);
    step1[29] = vqsubq_s16(step2[28], step2[29]);
    step1[30] = vqsubq_s16(step2[31], step2[30]);
    step1[31] = vqaddq_s16(step2[31], step2[30]);

    // stage 4

    btf_16_neon(step1[4], (*cospi.add(56) as i16), (*cospi.add(8) as i16), &raw mut step2[4], &raw mut step2[7]);
    btf_16_neon(step1[6], -(*cospi.add(40) as i16), (*cospi.add(24) as i16), &raw mut step2[5], &raw mut step2[6]);
    btf_16_lane_0_1_neon(step1[30], step1[17], c0, &raw mut step2[30], &raw mut step2[17]);
    btf_16_lane_1_0_neon(step1[18], step1[29], c2, &raw mut step2[18], &raw mut step2[29]);
    btf_16_lane_2_3_neon(step1[26], step1[21], c0, &raw mut step2[26], &raw mut step2[21]);
    btf_16_lane_3_2_neon(step1[22], step1[25], c2, &raw mut step2[22], &raw mut step2[25]);

    step2[0] = step1[0];
    step2[2] = step1[2];
    step2[8] = vqaddq_s16(step1[8], step1[9]);
    step2[9] = vqsubq_s16(step1[8], step1[9]);
    step2[10] = vqsubq_s16(step1[11], step1[10]);
    step2[11] = vqaddq_s16(step1[11], step1[10]);
    step2[12] = vqaddq_s16(step1[12], step1[13]);
    step2[13] = vqsubq_s16(step1[12], step1[13]);
    step2[14] = vqsubq_s16(step1[15], step1[14]);
    step2[15] = vqaddq_s16(step1[15], step1[14]);
    step2[16] = step1[16];
    step2[19] = step1[19];
    step2[20] = step1[20];
    step2[23] = step1[23];
    step2[24] = step1[24];
    step2[27] = step1[27];
    step2[28] = step1[28];
    step2[31] = step1[31];

    // stage 5

    t32[0] = vmull_n_s16(vget_low_s16(step2[0]), (*cospi.add(32) as i16));
    t32[1] = vmull_n_s16(vget_high_s16(step2[0]), (*cospi.add(32) as i16));

    step1[0] = vcombine_s16(vrshrn_n_s32(t32[0], INV_COS_BIT), vrshrn_n_s32(t32[1], INV_COS_BIT));

    btf_16_neon(step2[2], (*cospi.add(48) as i16), (*cospi.add(16) as i16), &raw mut step1[2], &raw mut step1[3]);
    btf_16_lane_2_3_neon(step2[14], step2[9], c1, &raw mut step1[14], &raw mut step1[9]);
    btf_16_lane_3_2_neon(step2[10], step2[13], c3, &raw mut step1[10], &raw mut step1[13]);

    step1[4] = vqaddq_s16(step2[4], step2[5]);
    step1[5] = vqsubq_s16(step2[4], step2[5]);
    step1[6] = vqsubq_s16(step2[7], step2[6]);
    step1[7] = vqaddq_s16(step2[7], step2[6]);
    step1[8] = step2[8];
    step1[11] = step2[11];
    step1[12] = step2[12];
    step1[15] = step2[15];
    step1[16] = vqaddq_s16(step2[16], step2[19]);
    step1[17] = vqaddq_s16(step2[17], step2[18]);
    step1[18] = vqsubq_s16(step2[17], step2[18]);
    step1[19] = vqsubq_s16(step2[16], step2[19]);
    step1[20] = vqsubq_s16(step2[23], step2[20]);
    step1[21] = vqsubq_s16(step2[22], step2[21]);
    step1[22] = vqaddq_s16(step2[22], step2[21]);
    step1[23] = vqaddq_s16(step2[23], step2[20]);
    step1[24] = vqaddq_s16(step2[24], step2[27]);
    step1[25] = vqaddq_s16(step2[25], step2[26]);
    step1[26] = vqsubq_s16(step2[25], step2[26]);
    step1[27] = vqsubq_s16(step2[24], step2[27]);
    step1[28] = vqsubq_s16(step2[31], step2[28]);
    step1[29] = vqsubq_s16(step2[30], step2[29]);
    step1[30] = vqaddq_s16(step2[30], step2[29]);
    step1[31] = vqaddq_s16(step2[31], step2[28]);

    // stage 6

    btf_16_lane_0_1_neon(step1[6], step1[5], c1, &raw mut step2[6], &raw mut step2[5]);
    btf_16_lane_2_3_neon(step1[29], step1[18], c1, &raw mut step2[29], &raw mut step2[18]);
    btf_16_lane_2_3_neon(step1[28], step1[19], c1, &raw mut step2[28], &raw mut step2[19]);
    btf_16_lane_3_2_neon(step1[20], step1[27], c3, &raw mut step2[20], &raw mut step2[27]);
    btf_16_lane_3_2_neon(step1[21], step1[26], c3, &raw mut step2[21], &raw mut step2[26]);

    step2[0] = vqaddq_s16(step1[0], step1[3]);
    step2[1] = vqaddq_s16(step1[0], step1[2]);
    step2[2] = vqsubq_s16(step1[0], step1[2]);
    step2[3] = vqsubq_s16(step1[0], step1[3]);
    step2[4] = step1[4];
    step2[7] = step1[7];
    step2[8] = vqaddq_s16(step1[8], step1[11]);
    step2[9] = vqaddq_s16(step1[9], step1[10]);
    step2[10] = vqsubq_s16(step1[9], step1[10]);
    step2[11] = vqsubq_s16(step1[8], step1[11]);
    step2[12] = vqsubq_s16(step1[15], step1[12]);
    step2[13] = vqsubq_s16(step1[14], step1[13]);
    step2[14] = vqaddq_s16(step1[14], step1[13]);
    step2[15] = vqaddq_s16(step1[15], step1[12]);
    step2[16] = step1[16];
    step2[17] = step1[17];
    step2[22] = step1[22];
    step2[23] = step1[23];
    step2[24] = step1[24];
    step2[25] = step1[25];
    step2[30] = step1[30];
    step2[31] = step1[31];

    // stage 7

    btf_16_lane_0_1_neon(step2[13], step2[10], c1, &raw mut step1[13], &raw mut step1[10]);
    btf_16_lane_0_1_neon(step2[12], step2[11], c1, &raw mut step1[12], &raw mut step1[11]);

    step1[0] = vqaddq_s16(step2[0], step2[7]);
    step1[1] = vqaddq_s16(step2[1], step2[6]);
    step1[2] = vqaddq_s16(step2[2], step2[5]);
    step1[3] = vqaddq_s16(step2[3], step2[4]);
    step1[4] = vqsubq_s16(step2[3], step2[4]);
    step1[5] = vqsubq_s16(step2[2], step2[5]);
    step1[6] = vqsubq_s16(step2[1], step2[6]);
    step1[7] = vqsubq_s16(step2[0], step2[7]);
    step1[8] = step2[8];
    step1[9] = step2[9];
    step1[14] = step2[14];
    step1[15] = step2[15];
    step1[16] = vqaddq_s16(step2[16], step2[23]);
    step1[17] = vqaddq_s16(step2[17], step2[22]);
    step1[18] = vqaddq_s16(step2[18], step2[21]);
    step1[19] = vqaddq_s16(step2[19], step2[20]);
    step1[20] = vqsubq_s16(step2[19], step2[20]);
    step1[21] = vqsubq_s16(step2[18], step2[21]);
    step1[22] = vqsubq_s16(step2[17], step2[22]);
    step1[23] = vqsubq_s16(step2[16], step2[23]);
    step1[24] = vqsubq_s16(step2[31], step2[24]);
    step1[25] = vqsubq_s16(step2[30], step2[25]);
    step1[26] = vqsubq_s16(step2[29], step2[26]);
    step1[27] = vqsubq_s16(step2[28], step2[27]);
    step1[28] = vqaddq_s16(step2[27], step2[28]);
    step1[29] = vqaddq_s16(step2[26], step2[29]);
    step1[30] = vqaddq_s16(step2[25], step2[30]);
    step1[31] = vqaddq_s16(step2[24], step2[31]);

    // stage 8

    btf_16_lane_0_1_neon(step1[27], step1[20], c1, &raw mut step2[27], &raw mut step2[20]);
    btf_16_lane_0_1_neon(step1[26], step1[21], c1, &raw mut step2[26], &raw mut step2[21]);
    btf_16_lane_0_1_neon(step1[25], step1[22], c1, &raw mut step2[25], &raw mut step2[22]);
    btf_16_lane_0_1_neon(step1[24], step1[23], c1, &raw mut step2[24], &raw mut step2[23]);

    step2[0] = vqaddq_s16(step1[0], step1[15]);
    step2[1] = vqaddq_s16(step1[1], step1[14]);
    step2[2] = vqaddq_s16(step1[2], step1[13]);
    step2[3] = vqaddq_s16(step1[3], step1[12]);
    step2[4] = vqaddq_s16(step1[4], step1[11]);
    step2[5] = vqaddq_s16(step1[5], step1[10]);
    step2[6] = vqaddq_s16(step1[6], step1[9]);
    step2[7] = vqaddq_s16(step1[7], step1[8]);
    step2[8] = vqsubq_s16(step1[7], step1[8]);
    step2[9] = vqsubq_s16(step1[6], step1[9]);
    step2[10] = vqsubq_s16(step1[5], step1[10]);
    step2[11] = vqsubq_s16(step1[4], step1[11]);
    step2[12] = vqsubq_s16(step1[3], step1[12]);
    step2[13] = vqsubq_s16(step1[2], step1[13]);
    step2[14] = vqsubq_s16(step1[1], step1[14]);
    step2[15] = vqsubq_s16(step1[0], step1[15]);
    step2[16] = step1[16];
    step2[17] = step1[17];
    step2[18] = step1[18];
    step2[19] = step1[19];
    step2[28] = step1[28];
    step2[29] = step1[29];
    step2[30] = step1[30];
    step2[31] = step1[31];

    // stage 9

    *out.add(0) = vqaddq_s16(step2[0], step2[31]);
    *out.add(1) = vqaddq_s16(step2[1], step2[30]);
    *out.add(2) = vqaddq_s16(step2[2], step2[29]);
    *out.add(3) = vqaddq_s16(step2[3], step2[28]);
    *out.add(4) = vqaddq_s16(step2[4], step2[27]);
    *out.add(5) = vqaddq_s16(step2[5], step2[26]);
    *out.add(6) = vqaddq_s16(step2[6], step2[25]);
    *out.add(7) = vqaddq_s16(step2[7], step2[24]);
    *out.add(8) = vqaddq_s16(step2[8], step2[23]);
    *out.add(9) = vqaddq_s16(step2[9], step2[22]);
    *out.add(10) = vqaddq_s16(step2[10], step2[21]);
    *out.add(11) = vqaddq_s16(step2[11], step2[20]);
    *out.add(12) = vqaddq_s16(step2[12], step2[19]);
    *out.add(13) = vqaddq_s16(step2[13], step2[18]);
    *out.add(14) = vqaddq_s16(step2[14], step2[17]);
    *out.add(15) = vqaddq_s16(step2[15], step2[16]);
    *out.add(16) = vqsubq_s16(step2[15], step2[16]);
    *out.add(17) = vqsubq_s16(step2[14], step2[17]);
    *out.add(18) = vqsubq_s16(step2[13], step2[18]);
    *out.add(19) = vqsubq_s16(step2[12], step2[19]);
    *out.add(20) = vqsubq_s16(step2[11], step2[20]);
    *out.add(21) = vqsubq_s16(step2[10], step2[21]);
    *out.add(22) = vqsubq_s16(step2[9], step2[22]);
    *out.add(23) = vqsubq_s16(step2[8], step2[23]);
    *out.add(24) = vqsubq_s16(step2[7], step2[24]);
    *out.add(25) = vqsubq_s16(step2[6], step2[25]);
    *out.add(26) = vqsubq_s16(step2[5], step2[26]);
    *out.add(27) = vqsubq_s16(step2[4], step2[27]);
    *out.add(28) = vqsubq_s16(step2[3], step2[28]);
    *out.add(29) = vqsubq_s16(step2[2], step2[29]);
    *out.add(30) = vqsubq_s16(step2[1], step2[30]);
    *out.add(31) = vqsubq_s16(step2[0], step2[31]);
}

#[inline(always)]
unsafe fn idct64_stage9_neon(mut step2: *mut int16x8_t, mut step1: *mut int16x8_t, mut cos_bit: i8) {
    let cospi: *const i32 = cospi_arr(cos_bit as i32);
    let c3: int16x4_t = set_s16x4_neon(((*cospi.add(32) as i16) as i16), ((*cospi.add(32) as i16) as i16), ((*cospi.add(16) as i16) as i16), ((*cospi.add(48) as i16) as i16));

    btf_16_lane_0_1_neon((*step2.add(27)), (*step2.add(20)), c3, step1.add(27), step1.add(20));
    btf_16_lane_0_1_neon((*step2.add(26)), (*step2.add(21)), c3, step1.add(26), step1.add(21));
    btf_16_lane_0_1_neon((*step2.add(25)), (*step2.add(22)), c3, step1.add(25), step1.add(22));
    btf_16_lane_0_1_neon((*step2.add(24)), (*step2.add(23)), c3, step1.add(24), step1.add(23));

    *step1.add(0) = vqaddq_s16((*step2.add(0)), (*step2.add(15)));
    *step1.add(1) = vqaddq_s16((*step2.add(1)), (*step2.add(14)));
    *step1.add(2) = vqaddq_s16((*step2.add(2)), (*step2.add(13)));
    *step1.add(3) = vqaddq_s16((*step2.add(3)), (*step2.add(12)));
    *step1.add(4) = vqaddq_s16((*step2.add(4)), (*step2.add(11)));
    *step1.add(5) = vqaddq_s16((*step2.add(5)), (*step2.add(10)));
    *step1.add(6) = vqaddq_s16((*step2.add(6)), (*step2.add(9)));
    *step1.add(7) = vqaddq_s16((*step2.add(7)), (*step2.add(8)));
    *step1.add(8) = vqsubq_s16((*step2.add(7)), (*step2.add(8)));
    *step1.add(9) = vqsubq_s16((*step2.add(6)), (*step2.add(9)));
    *step1.add(10) = vqsubq_s16((*step2.add(5)), (*step2.add(10)));
    *step1.add(11) = vqsubq_s16((*step2.add(4)), (*step2.add(11)));
    *step1.add(12) = vqsubq_s16((*step2.add(3)), (*step2.add(12)));
    *step1.add(13) = vqsubq_s16((*step2.add(2)), (*step2.add(13)));
    *step1.add(14) = vqsubq_s16((*step2.add(1)), (*step2.add(14)));
    *step1.add(15) = vqsubq_s16((*step2.add(0)), (*step2.add(15)));
    *step1.add(16) = (*step2.add(16));
    *step1.add(17) = (*step2.add(17));
    *step1.add(18) = (*step2.add(18));
    *step1.add(19) = (*step2.add(19));
    *step1.add(28) = (*step2.add(28));
    *step1.add(29) = (*step2.add(29));
    *step1.add(30) = (*step2.add(30));
    *step1.add(31) = (*step2.add(31));
    *step1.add(32) = vqaddq_s16((*step2.add(32)), (*step2.add(47)));
    *step1.add(33) = vqaddq_s16((*step2.add(33)), (*step2.add(46)));
    *step1.add(34) = vqaddq_s16((*step2.add(34)), (*step2.add(45)));
    *step1.add(35) = vqaddq_s16((*step2.add(35)), (*step2.add(44)));
    *step1.add(36) = vqaddq_s16((*step2.add(36)), (*step2.add(43)));
    *step1.add(37) = vqaddq_s16((*step2.add(37)), (*step2.add(42)));
    *step1.add(38) = vqaddq_s16((*step2.add(38)), (*step2.add(41)));
    *step1.add(39) = vqaddq_s16((*step2.add(39)), (*step2.add(40)));
    *step1.add(40) = vqsubq_s16((*step2.add(39)), (*step2.add(40)));
    *step1.add(41) = vqsubq_s16((*step2.add(38)), (*step2.add(41)));
    *step1.add(42) = vqsubq_s16((*step2.add(37)), (*step2.add(42)));
    *step1.add(43) = vqsubq_s16((*step2.add(36)), (*step2.add(43)));
    *step1.add(44) = vqsubq_s16((*step2.add(35)), (*step2.add(44)));
    *step1.add(45) = vqsubq_s16((*step2.add(34)), (*step2.add(45)));
    *step1.add(46) = vqsubq_s16((*step2.add(33)), (*step2.add(46)));
    *step1.add(47) = vqsubq_s16((*step2.add(32)), (*step2.add(47)));
    *step1.add(48) = vqsubq_s16((*step2.add(63)), (*step2.add(48)));
    *step1.add(49) = vqsubq_s16((*step2.add(62)), (*step2.add(49)));
    *step1.add(50) = vqsubq_s16((*step2.add(61)), (*step2.add(50)));
    *step1.add(51) = vqsubq_s16((*step2.add(60)), (*step2.add(51)));
    *step1.add(52) = vqsubq_s16((*step2.add(59)), (*step2.add(52)));
    *step1.add(53) = vqsubq_s16((*step2.add(58)), (*step2.add(53)));
    *step1.add(54) = vqsubq_s16((*step2.add(57)), (*step2.add(54)));
    *step1.add(55) = vqsubq_s16((*step2.add(56)), (*step2.add(55)));
    *step1.add(56) = vqaddq_s16((*step2.add(56)), (*step2.add(55)));
    *step1.add(57) = vqaddq_s16((*step2.add(57)), (*step2.add(54)));
    *step1.add(58) = vqaddq_s16((*step2.add(58)), (*step2.add(53)));
    *step1.add(59) = vqaddq_s16((*step2.add(59)), (*step2.add(52)));
    *step1.add(60) = vqaddq_s16((*step2.add(60)), (*step2.add(51)));
    *step1.add(61) = vqaddq_s16((*step2.add(61)), (*step2.add(50)));
    *step1.add(62) = vqaddq_s16((*step2.add(62)), (*step2.add(49)));
    *step1.add(63) = vqaddq_s16((*step2.add(63)), (*step2.add(48)));
}

#[inline(always)]
unsafe fn idct64_stage10_neon(mut step1: *mut int16x8_t, mut step2: *mut int16x8_t, mut cos_bit: i8) {
    let cospi: *const i32 = cospi_arr(cos_bit as i32);
    let c3: int16x4_t = set_s16x4_neon(((*cospi.add(32) as i16) as i16), ((*cospi.add(32) as i16) as i16), ((*cospi.add(16) as i16) as i16), ((*cospi.add(48) as i16) as i16));

    btf_16_lane_0_1_neon((*step1.add(55)), (*step1.add(40)), c3, step2.add(55), step2.add(40));
    btf_16_lane_0_1_neon((*step1.add(54)), (*step1.add(41)), c3, step2.add(54), step2.add(41));
    btf_16_lane_0_1_neon((*step1.add(53)), (*step1.add(42)), c3, step2.add(53), step2.add(42));
    btf_16_lane_0_1_neon((*step1.add(52)), (*step1.add(43)), c3, step2.add(52), step2.add(43));
    btf_16_lane_0_1_neon((*step1.add(51)), (*step1.add(44)), c3, step2.add(51), step2.add(44));
    btf_16_lane_0_1_neon((*step1.add(50)), (*step1.add(45)), c3, step2.add(50), step2.add(45));
    btf_16_lane_0_1_neon((*step1.add(49)), (*step1.add(46)), c3, step2.add(49), step2.add(46));
    btf_16_lane_0_1_neon((*step1.add(48)), (*step1.add(47)), c3, step2.add(48), step2.add(47));

    *step2.add(0) = vqaddq_s16((*step1.add(0)), (*step1.add(31)));
    *step2.add(1) = vqaddq_s16((*step1.add(1)), (*step1.add(30)));
    *step2.add(2) = vqaddq_s16((*step1.add(2)), (*step1.add(29)));
    *step2.add(3) = vqaddq_s16((*step1.add(3)), (*step1.add(28)));
    *step2.add(4) = vqaddq_s16((*step1.add(4)), (*step1.add(27)));
    *step2.add(5) = vqaddq_s16((*step1.add(5)), (*step1.add(26)));
    *step2.add(6) = vqaddq_s16((*step1.add(6)), (*step1.add(25)));
    *step2.add(7) = vqaddq_s16((*step1.add(7)), (*step1.add(24)));
    *step2.add(8) = vqaddq_s16((*step1.add(8)), (*step1.add(23)));
    *step2.add(9) = vqaddq_s16((*step1.add(9)), (*step1.add(22)));
    *step2.add(10) = vqaddq_s16((*step1.add(10)), (*step1.add(21)));
    *step2.add(11) = vqaddq_s16((*step1.add(11)), (*step1.add(20)));
    *step2.add(12) = vqaddq_s16((*step1.add(12)), (*step1.add(19)));
    *step2.add(13) = vqaddq_s16((*step1.add(13)), (*step1.add(18)));
    *step2.add(14) = vqaddq_s16((*step1.add(14)), (*step1.add(17)));
    *step2.add(15) = vqaddq_s16((*step1.add(15)), (*step1.add(16)));
    *step2.add(16) = vqsubq_s16((*step1.add(15)), (*step1.add(16)));
    *step2.add(17) = vqsubq_s16((*step1.add(14)), (*step1.add(17)));
    *step2.add(18) = vqsubq_s16((*step1.add(13)), (*step1.add(18)));
    *step2.add(19) = vqsubq_s16((*step1.add(12)), (*step1.add(19)));
    *step2.add(20) = vqsubq_s16((*step1.add(11)), (*step1.add(20)));
    *step2.add(21) = vqsubq_s16((*step1.add(10)), (*step1.add(21)));
    *step2.add(22) = vqsubq_s16((*step1.add(9)), (*step1.add(22)));
    *step2.add(23) = vqsubq_s16((*step1.add(8)), (*step1.add(23)));
    *step2.add(24) = vqsubq_s16((*step1.add(7)), (*step1.add(24)));
    *step2.add(25) = vqsubq_s16((*step1.add(6)), (*step1.add(25)));
    *step2.add(26) = vqsubq_s16((*step1.add(5)), (*step1.add(26)));
    *step2.add(27) = vqsubq_s16((*step1.add(4)), (*step1.add(27)));
    *step2.add(28) = vqsubq_s16((*step1.add(3)), (*step1.add(28)));
    *step2.add(29) = vqsubq_s16((*step1.add(2)), (*step1.add(29)));
    *step2.add(30) = vqsubq_s16((*step1.add(1)), (*step1.add(30)));
    *step2.add(31) = vqsubq_s16((*step1.add(0)), (*step1.add(31)));
    *step2.add(32) = (*step1.add(32));
    *step2.add(33) = (*step1.add(33));
    *step2.add(34) = (*step1.add(34));
    *step2.add(35) = (*step1.add(35));
    *step2.add(36) = (*step1.add(36));
    *step2.add(37) = (*step1.add(37));
    *step2.add(38) = (*step1.add(38));
    *step2.add(39) = (*step1.add(39));
    *step2.add(56) = (*step1.add(56));
    *step2.add(57) = (*step1.add(57));
    *step2.add(58) = (*step1.add(58));
    *step2.add(59) = (*step1.add(59));
    *step2.add(60) = (*step1.add(60));
    *step2.add(61) = (*step1.add(61));
    *step2.add(62) = (*step1.add(62));
    *step2.add(63) = (*step1.add(63));
}

#[inline(always)]
unsafe fn idct64_low32_neon(mut in_: *mut int16x8_t, mut out: *mut int16x8_t, mut cos_bit: i8) {
    let cospi: *const i32 = cospi_arr(cos_bit as i32);
    let mut step2: [int16x8_t; 64] = core::mem::zeroed();
    let mut step1: [int16x8_t; 64] = core::mem::zeroed();
    let c0: int16x4_t = set_s16x4_neon(((*cospi.add(4) as i16) as i16), ((*cospi.add(60) as i16) as i16), ((*cospi.add(36) as i16) as i16), ((*cospi.add(28) as i16) as i16));
    let c1: int16x4_t = set_s16x4_neon(((*cospi.add(20) as i16) as i16), ((*cospi.add(44) as i16) as i16), ((*cospi.add(52) as i16) as i16), ((*cospi.add(12) as i16) as i16));
    let c2: int16x4_t = set_s16x4_neon(((*cospi.add(8) as i16) as i16), ((*cospi.add(56) as i16) as i16), ((*cospi.add(40) as i16) as i16), ((*cospi.add(24) as i16) as i16));
    let c3: int16x4_t = set_s16x4_neon(((*cospi.add(32) as i16) as i16), ((*cospi.add(32) as i16) as i16), ((*cospi.add(16) as i16) as i16), ((*cospi.add(48) as i16) as i16));
    let c4: int16x4_t = set_s16x4_neon(((-(*cospi.add(4) as i16)) as i16), ((-(*cospi.add(60) as i16)) as i16), ((-(*cospi.add(36) as i16)) as i16), ((-(*cospi.add(28) as i16)) as i16));
    let c5: int16x4_t = set_s16x4_neon(((-(*cospi.add(20) as i16)) as i16), ((-(*cospi.add(44) as i16)) as i16), ((-(*cospi.add(52) as i16)) as i16), ((-(*cospi.add(12) as i16)) as i16));
    let c6: int16x4_t = set_s16x4_neon(((-(*cospi.add(8) as i16)) as i16), ((-(*cospi.add(56) as i16)) as i16), ((-(*cospi.add(40) as i16)) as i16), ((-(*cospi.add(24) as i16)) as i16));
    let c7: int16x4_t = set_s16x4_neon(((-(*cospi.add(32) as i16)) as i16), ((-(*cospi.add(32) as i16)) as i16), ((-(*cospi.add(16) as i16)) as i16), ((-(*cospi.add(48) as i16)) as i16));

    // stage 1
    // stage 2

    step2[0] = (*in_.add(0));
    step2[2] = (*in_.add(16));
    step2[4] = (*in_.add(8));
    step2[6] = (*in_.add(24));
    step2[8] = (*in_.add(4));
    step2[10] = (*in_.add(20));
    step2[12] = (*in_.add(12));
    step2[14] = (*in_.add(28));
    step2[16] = (*in_.add(2));
    step2[18] = (*in_.add(18));
    step2[20] = (*in_.add(10));
    step2[22] = (*in_.add(26));
    step2[24] = (*in_.add(6));
    step2[26] = (*in_.add(22));
    step2[28] = (*in_.add(14));
    step2[30] = (*in_.add(30));

    btf_16_neon((*in_.add(1)), (*cospi.add(63) as i16), (*cospi.add(1) as i16), &raw mut step2[32], &raw mut step2[63]);
    btf_16_neon((*in_.add(31)), -(*cospi.add(33) as i16), (*cospi.add(31) as i16), &raw mut step2[33], &raw mut step2[62]);
    btf_16_neon((*in_.add(17)), (*cospi.add(47) as i16), (*cospi.add(17) as i16), &raw mut step2[34], &raw mut step2[61]);
    btf_16_neon((*in_.add(15)), -(*cospi.add(49) as i16), (*cospi.add(15) as i16), &raw mut step2[35], &raw mut step2[60]);
    btf_16_neon((*in_.add(9)), (*cospi.add(55) as i16), (*cospi.add(9) as i16), &raw mut step2[36], &raw mut step2[59]);
    btf_16_neon((*in_.add(23)), -(*cospi.add(41) as i16), (*cospi.add(23) as i16), &raw mut step2[37], &raw mut step2[58]);
    btf_16_neon((*in_.add(25)), (*cospi.add(39) as i16), (*cospi.add(25) as i16), &raw mut step2[38], &raw mut step2[57]);
    btf_16_neon((*in_.add(7)), -(*cospi.add(57) as i16), (*cospi.add(7) as i16), &raw mut step2[39], &raw mut step2[56]);
    btf_16_neon((*in_.add(5)), (*cospi.add(59) as i16), (*cospi.add(5) as i16), &raw mut step2[40], &raw mut step2[55]);
    btf_16_neon((*in_.add(27)), -(*cospi.add(37) as i16), (*cospi.add(27) as i16), &raw mut step2[41], &raw mut step2[54]);
    btf_16_neon((*in_.add(21)), (*cospi.add(43) as i16), (*cospi.add(21) as i16), &raw mut step2[42], &raw mut step2[53]);
    btf_16_neon((*in_.add(11)), -(*cospi.add(53) as i16), (*cospi.add(11) as i16), &raw mut step2[43], &raw mut step2[52]);
    btf_16_neon((*in_.add(13)), (*cospi.add(51) as i16), (*cospi.add(13) as i16), &raw mut step2[44], &raw mut step2[51]);
    btf_16_neon((*in_.add(19)), -(*cospi.add(45) as i16), (*cospi.add(19) as i16), &raw mut step2[45], &raw mut step2[50]);
    btf_16_neon((*in_.add(29)), (*cospi.add(35) as i16), (*cospi.add(29) as i16), &raw mut step2[46], &raw mut step2[49]);
    btf_16_neon((*in_.add(3)), -(*cospi.add(61) as i16), (*cospi.add(3) as i16), &raw mut step2[47], &raw mut step2[48]);

    // stage 3

    step1[0] = step2[0];
    step1[2] = step2[2];
    step1[4] = step2[4];
    step1[6] = step2[6];
    step1[8] = step2[8];
    step1[10] = step2[10];
    step1[12] = step2[12];
    step1[14] = step2[14];

    btf_16_neon(step2[16], (*cospi.add(62) as i16), (*cospi.add(2) as i16), &raw mut step1[16], &raw mut step1[31]);
    btf_16_neon(step2[30], -(*cospi.add(34) as i16), (*cospi.add(30) as i16), &raw mut step1[17], &raw mut step1[30]);
    btf_16_neon(step2[18], (*cospi.add(46) as i16), (*cospi.add(18) as i16), &raw mut step1[18], &raw mut step1[29]);
    btf_16_neon(step2[28], -(*cospi.add(50) as i16), (*cospi.add(14) as i16), &raw mut step1[19], &raw mut step1[28]);
    btf_16_neon(step2[20], (*cospi.add(54) as i16), (*cospi.add(10) as i16), &raw mut step1[20], &raw mut step1[27]);
    btf_16_neon(step2[26], -(*cospi.add(42) as i16), (*cospi.add(22) as i16), &raw mut step1[21], &raw mut step1[26]);
    btf_16_neon(step2[22], (*cospi.add(38) as i16), (*cospi.add(26) as i16), &raw mut step1[22], &raw mut step1[25]);
    btf_16_neon(step2[24], -(*cospi.add(58) as i16), (*cospi.add(6) as i16), &raw mut step1[23], &raw mut step1[24]);

    step1[32] = vqaddq_s16(step2[32], step2[33]);
    step1[33] = vqsubq_s16(step2[32], step2[33]);
    step1[34] = vqsubq_s16(step2[35], step2[34]);
    step1[35] = vqaddq_s16(step2[35], step2[34]);
    step1[36] = vqaddq_s16(step2[36], step2[37]);
    step1[37] = vqsubq_s16(step2[36], step2[37]);
    step1[38] = vqsubq_s16(step2[39], step2[38]);
    step1[39] = vqaddq_s16(step2[39], step2[38]);
    step1[40] = vqaddq_s16(step2[40], step2[41]);
    step1[41] = vqsubq_s16(step2[40], step2[41]);
    step1[42] = vqsubq_s16(step2[43], step2[42]);
    step1[43] = vqaddq_s16(step2[43], step2[42]);
    step1[44] = vqaddq_s16(step2[44], step2[45]);
    step1[45] = vqsubq_s16(step2[44], step2[45]);
    step1[46] = vqsubq_s16(step2[47], step2[46]);
    step1[47] = vqaddq_s16(step2[47], step2[46]);
    step1[48] = vqaddq_s16(step2[48], step2[49]);
    step1[49] = vqsubq_s16(step2[48], step2[49]);
    step1[50] = vqsubq_s16(step2[51], step2[50]);
    step1[51] = vqaddq_s16(step2[51], step2[50]);
    step1[52] = vqaddq_s16(step2[52], step2[53]);
    step1[53] = vqsubq_s16(step2[52], step2[53]);
    step1[54] = vqsubq_s16(step2[55], step2[54]);
    step1[55] = vqaddq_s16(step2[55], step2[54]);
    step1[56] = vqaddq_s16(step2[56], step2[57]);
    step1[57] = vqsubq_s16(step2[56], step2[57]);
    step1[58] = vqsubq_s16(step2[59], step2[58]);
    step1[59] = vqaddq_s16(step2[59], step2[58]);
    step1[60] = vqaddq_s16(step2[60], step2[61]);
    step1[61] = vqsubq_s16(step2[60], step2[61]);
    step1[62] = vqsubq_s16(step2[63], step2[62]);
    step1[63] = vqaddq_s16(step2[63], step2[62]);

    // stage 4

    step2[0] = step1[0];
    step2[2] = step1[2];
    step2[4] = step1[4];
    step2[6] = step1[6];

    btf_16_neon(step1[8], (*cospi.add(60) as i16), (*cospi.add(4) as i16), &raw mut step2[8], &raw mut step2[15]);
    btf_16_neon(step1[14], -(*cospi.add(36) as i16), (*cospi.add(28) as i16), &raw mut step2[9], &raw mut step2[14]);
    btf_16_neon(step1[10], (*cospi.add(44) as i16), (*cospi.add(20) as i16), &raw mut step2[10], &raw mut step2[13]);
    btf_16_neon(step1[12], -(*cospi.add(52) as i16), (*cospi.add(12) as i16), &raw mut step2[11], &raw mut step2[12]);
    btf_16_lane_0_1_neon(step1[62], step1[33], c0, &raw mut step2[62], &raw mut step2[33]);
    btf_16_lane_1_0_neon(step1[34], step1[61], c4, &raw mut step2[34], &raw mut step2[61]);
    btf_16_lane_2_3_neon(step1[58], step1[37], c0, &raw mut step2[58], &raw mut step2[37]);
    btf_16_lane_3_2_neon(step1[38], step1[57], c4, &raw mut step2[38], &raw mut step2[57]);
    btf_16_lane_0_1_neon(step1[54], step1[41], c1, &raw mut step2[54], &raw mut step2[41]);
    btf_16_lane_1_0_neon(step1[42], step1[53], c5, &raw mut step2[42], &raw mut step2[53]);
    btf_16_lane_2_3_neon(step1[50], step1[45], c1, &raw mut step2[50], &raw mut step2[45]);
    btf_16_lane_3_2_neon(step1[46], step1[49], c5, &raw mut step2[46], &raw mut step2[49]);

    step2[16] = vqaddq_s16(step1[16], step1[17]);
    step2[17] = vqsubq_s16(step1[16], step1[17]);
    step2[18] = vqsubq_s16(step1[19], step1[18]);
    step2[19] = vqaddq_s16(step1[19], step1[18]);
    step2[20] = vqaddq_s16(step1[20], step1[21]);
    step2[21] = vqsubq_s16(step1[20], step1[21]);
    step2[22] = vqsubq_s16(step1[23], step1[22]);
    step2[23] = vqaddq_s16(step1[23], step1[22]);
    step2[24] = vqaddq_s16(step1[24], step1[25]);
    step2[25] = vqsubq_s16(step1[24], step1[25]);
    step2[26] = vqsubq_s16(step1[27], step1[26]);
    step2[27] = vqaddq_s16(step1[27], step1[26]);
    step2[28] = vqaddq_s16(step1[28], step1[29]);
    step2[29] = vqsubq_s16(step1[28], step1[29]);
    step2[30] = vqsubq_s16(step1[31], step1[30]);
    step2[31] = vqaddq_s16(step1[31], step1[30]);
    step2[32] = step1[32];
    step2[35] = step1[35];
    step2[36] = step1[36];
    step2[39] = step1[39];
    step2[40] = step1[40];
    step2[43] = step1[43];
    step2[44] = step1[44];
    step2[47] = step1[47];
    step2[48] = step1[48];
    step2[51] = step1[51];
    step2[52] = step1[52];
    step2[55] = step1[55];
    step2[56] = step1[56];
    step2[59] = step1[59];
    step2[60] = step1[60];
    step2[63] = step1[63];

    // stage 5

    step1[0] = step2[0];
    step1[2] = step2[2];

    btf_16_neon(step2[4], (*cospi.add(56) as i16), (*cospi.add(8) as i16), &raw mut step1[4], &raw mut step1[7]);
    btf_16_neon(step2[6], -(*cospi.add(40) as i16), (*cospi.add(24) as i16), &raw mut step1[5], &raw mut step1[6]);
    btf_16_lane_0_1_neon(step2[30], step2[17], c2, &raw mut step1[30], &raw mut step1[17]);
    btf_16_lane_1_0_neon(step2[18], step2[29], c6, &raw mut step1[18], &raw mut step1[29]);
    btf_16_lane_2_3_neon(step2[26], step2[21], c2, &raw mut step1[26], &raw mut step1[21]);
    btf_16_lane_3_2_neon(step2[22], step2[25], c6, &raw mut step1[22], &raw mut step1[25]);

    step1[8] = vqaddq_s16(step2[8], step2[9]);
    step1[9] = vqsubq_s16(step2[8], step2[9]);
    step1[10] = vqsubq_s16(step2[11], step2[10]);
    step1[11] = vqaddq_s16(step2[11], step2[10]);
    step1[12] = vqaddq_s16(step2[12], step2[13]);
    step1[13] = vqsubq_s16(step2[12], step2[13]);
    step1[14] = vqsubq_s16(step2[15], step2[14]);
    step1[15] = vqaddq_s16(step2[15], step2[14]);
    step1[16] = step2[16];
    step1[19] = step2[19];
    step1[20] = step2[20];
    step1[23] = step2[23];
    step1[24] = step2[24];
    step1[27] = step2[27];
    step1[28] = step2[28];
    step1[31] = step2[31];
    step1[32] = vqaddq_s16(step2[32], step2[35]);
    step1[33] = vqaddq_s16(step2[33], step2[34]);
    step1[34] = vqsubq_s16(step2[33], step2[34]);
    step1[35] = vqsubq_s16(step2[32], step2[35]);
    step1[36] = vqsubq_s16(step2[39], step2[36]);
    step1[37] = vqsubq_s16(step2[38], step2[37]);
    step1[38] = vqaddq_s16(step2[38], step2[37]);
    step1[39] = vqaddq_s16(step2[39], step2[36]);
    step1[40] = vqaddq_s16(step2[40], step2[43]);
    step1[41] = vqaddq_s16(step2[41], step2[42]);
    step1[42] = vqsubq_s16(step2[41], step2[42]);
    step1[43] = vqsubq_s16(step2[40], step2[43]);
    step1[44] = vqsubq_s16(step2[47], step2[44]);
    step1[45] = vqsubq_s16(step2[46], step2[45]);
    step1[46] = vqaddq_s16(step2[46], step2[45]);
    step1[47] = vqaddq_s16(step2[47], step2[44]);
    step1[48] = vqaddq_s16(step2[48], step2[51]);
    step1[49] = vqaddq_s16(step2[49], step2[50]);
    step1[50] = vqsubq_s16(step2[49], step2[50]);
    step1[51] = vqsubq_s16(step2[48], step2[51]);
    step1[52] = vqsubq_s16(step2[55], step2[52]);
    step1[53] = vqsubq_s16(step2[54], step2[53]);
    step1[54] = vqaddq_s16(step2[54], step2[53]);
    step1[55] = vqaddq_s16(step2[55], step2[52]);
    step1[56] = vqaddq_s16(step2[56], step2[59]);
    step1[57] = vqaddq_s16(step2[57], step2[58]);
    step1[58] = vqsubq_s16(step2[57], step2[58]);
    step1[59] = vqsubq_s16(step2[56], step2[59]);
    step1[60] = vqsubq_s16(step2[63], step2[60]);
    step1[61] = vqsubq_s16(step2[62], step2[61]);
    step1[62] = vqaddq_s16(step2[62], step2[61]);
    step1[63] = vqaddq_s16(step2[63], step2[60]);

    // stage 6

    btf_16_neon(step1[0], (*cospi.add(32) as i16), (*cospi.add(32) as i16), &raw mut step2[0], &raw mut step2[1]);
    btf_16_neon(step1[2], (*cospi.add(48) as i16), (*cospi.add(16) as i16), &raw mut step2[2], &raw mut step2[3]);
    btf_16_lane_2_3_neon(step1[14], step1[9], c3, &raw mut step2[14], &raw mut step2[9]);
    btf_16_lane_3_2_neon(step1[10], step1[13], c7, &raw mut step2[10], &raw mut step2[13]);
    btf_16_lane_0_1_neon(step1[61], step1[34], c2, &raw mut step2[61], &raw mut step2[34]);
    btf_16_lane_0_1_neon(step1[60], step1[35], c2, &raw mut step2[60], &raw mut step2[35]);
    btf_16_lane_1_0_neon(step1[36], step1[59], c6, &raw mut step2[36], &raw mut step2[59]);
    btf_16_lane_1_0_neon(step1[37], step1[58], c6, &raw mut step2[37], &raw mut step2[58]);
    btf_16_lane_2_3_neon(step1[53], step1[42], c2, &raw mut step2[53], &raw mut step2[42]);
    btf_16_lane_2_3_neon(step1[52], step1[43], c2, &raw mut step2[52], &raw mut step2[43]);
    btf_16_lane_3_2_neon(step1[44], step1[51], c6, &raw mut step2[44], &raw mut step2[51]);
    btf_16_lane_3_2_neon(step1[45], step1[50], c6, &raw mut step2[45], &raw mut step2[50]);

    step2[4] = vqaddq_s16(step1[4], step1[5]);
    step2[5] = vqsubq_s16(step1[4], step1[5]);
    step2[6] = vqsubq_s16(step1[7], step1[6]);
    step2[7] = vqaddq_s16(step1[7], step1[6]);
    step2[8] = step1[8];
    step2[11] = step1[11];
    step2[12] = step1[12];
    step2[15] = step1[15];
    step2[16] = vqaddq_s16(step1[16], step1[19]);
    step2[17] = vqaddq_s16(step1[17], step1[18]);
    step2[18] = vqsubq_s16(step1[17], step1[18]);
    step2[19] = vqsubq_s16(step1[16], step1[19]);
    step2[20] = vqsubq_s16(step1[23], step1[20]);
    step2[21] = vqsubq_s16(step1[22], step1[21]);
    step2[22] = vqaddq_s16(step1[22], step1[21]);
    step2[23] = vqaddq_s16(step1[23], step1[20]);
    step2[24] = vqaddq_s16(step1[24], step1[27]);
    step2[25] = vqaddq_s16(step1[25], step1[26]);
    step2[26] = vqsubq_s16(step1[25], step1[26]);
    step2[27] = vqsubq_s16(step1[24], step1[27]);
    step2[28] = vqsubq_s16(step1[31], step1[28]);
    step2[29] = vqsubq_s16(step1[30], step1[29]);
    step2[30] = vqaddq_s16(step1[30], step1[29]);
    step2[31] = vqaddq_s16(step1[31], step1[28]);
    step2[32] = step1[32];
    step2[33] = step1[33];
    step2[38] = step1[38];
    step2[39] = step1[39];
    step2[40] = step1[40];
    step2[41] = step1[41];
    step2[46] = step1[46];
    step2[47] = step1[47];
    step2[48] = step1[48];
    step2[49] = step1[49];
    step2[54] = step1[54];
    step2[55] = step1[55];
    step2[56] = step1[56];
    step2[57] = step1[57];
    step2[62] = step1[62];
    step2[63] = step1[63];

    // stage 7

    btf_16_lane_0_1_neon(step2[6], step2[5], c3, &raw mut step1[6], &raw mut step1[5]);
    btf_16_lane_2_3_neon(step2[29], step2[18], c3, &raw mut step1[29], &raw mut step1[18]);
    btf_16_lane_2_3_neon(step2[28], step2[19], c3, &raw mut step1[28], &raw mut step1[19]);
    btf_16_lane_3_2_neon(step2[20], step2[27], c7, &raw mut step1[20], &raw mut step1[27]);
    btf_16_lane_3_2_neon(step2[21], step2[26], c7, &raw mut step1[21], &raw mut step1[26]);

    step1[0] = vqaddq_s16(step2[0], step2[3]);
    step1[1] = vqaddq_s16(step2[1], step2[2]);
    step1[2] = vqsubq_s16(step2[1], step2[2]);
    step1[3] = vqsubq_s16(step2[0], step2[3]);
    step1[4] = step2[4];
    step1[7] = step2[7];
    step1[8] = vqaddq_s16(step2[8], step2[11]);
    step1[9] = vqaddq_s16(step2[9], step2[10]);
    step1[10] = vqsubq_s16(step2[9], step2[10]);
    step1[11] = vqsubq_s16(step2[8], step2[11]);
    step1[12] = vqsubq_s16(step2[15], step2[12]);
    step1[13] = vqsubq_s16(step2[14], step2[13]);
    step1[14] = vqaddq_s16(step2[14], step2[13]);
    step1[15] = vqaddq_s16(step2[15], step2[12]);
    step1[16] = step2[16];
    step1[17] = step2[17];
    step1[22] = step2[22];
    step1[23] = step2[23];
    step1[24] = step2[24];
    step1[25] = step2[25];
    step1[30] = step2[30];
    step1[31] = step2[31];
    step1[32] = vqaddq_s16(step2[32], step2[39]);
    step1[33] = vqaddq_s16(step2[33], step2[38]);
    step1[34] = vqaddq_s16(step2[34], step2[37]);
    step1[35] = vqaddq_s16(step2[35], step2[36]);
    step1[36] = vqsubq_s16(step2[35], step2[36]);
    step1[37] = vqsubq_s16(step2[34], step2[37]);
    step1[38] = vqsubq_s16(step2[33], step2[38]);
    step1[39] = vqsubq_s16(step2[32], step2[39]);
    step1[40] = vqsubq_s16(step2[47], step2[40]);
    step1[41] = vqsubq_s16(step2[46], step2[41]);
    step1[42] = vqsubq_s16(step2[45], step2[42]);
    step1[43] = vqsubq_s16(step2[44], step2[43]);
    step1[44] = vqaddq_s16(step2[43], step2[44]);
    step1[45] = vqaddq_s16(step2[42], step2[45]);
    step1[46] = vqaddq_s16(step2[41], step2[46]);
    step1[47] = vqaddq_s16(step2[40], step2[47]);
    step1[48] = vqaddq_s16(step2[48], step2[55]);
    step1[49] = vqaddq_s16(step2[49], step2[54]);
    step1[50] = vqaddq_s16(step2[50], step2[53]);
    step1[51] = vqaddq_s16(step2[51], step2[52]);
    step1[52] = vqsubq_s16(step2[51], step2[52]);
    step1[53] = vqsubq_s16(step2[50], step2[53]);
    step1[54] = vqsubq_s16(step2[49], step2[54]);
    step1[55] = vqsubq_s16(step2[48], step2[55]);
    step1[56] = vqsubq_s16(step2[63], step2[56]);
    step1[57] = vqsubq_s16(step2[62], step2[57]);
    step1[58] = vqsubq_s16(step2[61], step2[58]);
    step1[59] = vqsubq_s16(step2[60], step2[59]);
    step1[60] = vqaddq_s16(step2[59], step2[60]);
    step1[61] = vqaddq_s16(step2[58], step2[61]);
    step1[62] = vqaddq_s16(step2[57], step2[62]);
    step1[63] = vqaddq_s16(step2[56], step2[63]);

    // stage 8

    btf_16_lane_0_1_neon(step1[13], step1[10], c3, &raw mut step2[13], &raw mut step2[10]);
    btf_16_lane_0_1_neon(step1[12], step1[11], c3, &raw mut step2[12], &raw mut step2[11]);
    btf_16_lane_2_3_neon(step1[59], step1[36], c3, &raw mut step2[59], &raw mut step2[36]);
    btf_16_lane_2_3_neon(step1[58], step1[37], c3, &raw mut step2[58], &raw mut step2[37]);
    btf_16_lane_2_3_neon(step1[57], step1[38], c3, &raw mut step2[57], &raw mut step2[38]);
    btf_16_lane_2_3_neon(step1[56], step1[39], c3, &raw mut step2[56], &raw mut step2[39]);
    btf_16_lane_3_2_neon(step1[40], step1[55], c7, &raw mut step2[40], &raw mut step2[55]);
    btf_16_lane_3_2_neon(step1[41], step1[54], c7, &raw mut step2[41], &raw mut step2[54]);
    btf_16_lane_3_2_neon(step1[42], step1[53], c7, &raw mut step2[42], &raw mut step2[53]);
    btf_16_lane_3_2_neon(step1[43], step1[52], c7, &raw mut step2[43], &raw mut step2[52]);

    step2[0] = vqaddq_s16(step1[0], step1[7]);
    step2[1] = vqaddq_s16(step1[1], step1[6]);
    step2[2] = vqaddq_s16(step1[2], step1[5]);
    step2[3] = vqaddq_s16(step1[3], step1[4]);
    step2[4] = vqsubq_s16(step1[3], step1[4]);
    step2[5] = vqsubq_s16(step1[2], step1[5]);
    step2[6] = vqsubq_s16(step1[1], step1[6]);
    step2[7] = vqsubq_s16(step1[0], step1[7]);
    step2[8] = step1[8];
    step2[9] = step1[9];
    step2[14] = step1[14];
    step2[15] = step1[15];
    step2[16] = vqaddq_s16(step1[16], step1[23]);
    step2[17] = vqaddq_s16(step1[17], step1[22]);
    step2[18] = vqaddq_s16(step1[18], step1[21]);
    step2[19] = vqaddq_s16(step1[19], step1[20]);
    step2[20] = vqsubq_s16(step1[19], step1[20]);
    step2[21] = vqsubq_s16(step1[18], step1[21]);
    step2[22] = vqsubq_s16(step1[17], step1[22]);
    step2[23] = vqsubq_s16(step1[16], step1[23]);
    step2[24] = vqsubq_s16(step1[31], step1[24]);
    step2[25] = vqsubq_s16(step1[30], step1[25]);
    step2[26] = vqsubq_s16(step1[29], step1[26]);
    step2[27] = vqsubq_s16(step1[28], step1[27]);
    step2[28] = vqaddq_s16(step1[28], step1[27]);
    step2[29] = vqaddq_s16(step1[29], step1[26]);
    step2[30] = vqaddq_s16(step1[30], step1[25]);
    step2[31] = vqaddq_s16(step1[31], step1[24]);
    step2[32] = step1[32];
    step2[33] = step1[33];
    step2[34] = step1[34];
    step2[35] = step1[35];
    step2[44] = step1[44];
    step2[45] = step1[45];
    step2[46] = step1[46];
    step2[47] = step1[47];
    step2[48] = step1[48];
    step2[49] = step1[49];
    step2[50] = step1[50];
    step2[51] = step1[51];
    step2[60] = step1[60];
    step2[61] = step1[61];
    step2[62] = step1[62];
    step2[63] = step1[63];

    // stage 9
    idct64_stage9_neon(step2.as_mut_ptr(), step1.as_mut_ptr(), cos_bit);

    // stage 10
    idct64_stage10_neon(step1.as_mut_ptr(), step2.as_mut_ptr(), cos_bit);

    // stage 11

    *out.add(0) = vqaddq_s16(step2[0], step2[63]);
    *out.add(1) = vqaddq_s16(step2[1], step2[62]);
    *out.add(2) = vqaddq_s16(step2[2], step2[61]);
    *out.add(3) = vqaddq_s16(step2[3], step2[60]);
    *out.add(4) = vqaddq_s16(step2[4], step2[59]);
    *out.add(5) = vqaddq_s16(step2[5], step2[58]);
    *out.add(6) = vqaddq_s16(step2[6], step2[57]);
    *out.add(7) = vqaddq_s16(step2[7], step2[56]);
    *out.add(8) = vqaddq_s16(step2[8], step2[55]);
    *out.add(9) = vqaddq_s16(step2[9], step2[54]);
    *out.add(10) = vqaddq_s16(step2[10], step2[53]);
    *out.add(11) = vqaddq_s16(step2[11], step2[52]);
    *out.add(12) = vqaddq_s16(step2[12], step2[51]);
    *out.add(13) = vqaddq_s16(step2[13], step2[50]);
    *out.add(14) = vqaddq_s16(step2[14], step2[49]);
    *out.add(15) = vqaddq_s16(step2[15], step2[48]);
    *out.add(16) = vqaddq_s16(step2[16], step2[47]);
    *out.add(17) = vqaddq_s16(step2[17], step2[46]);
    *out.add(18) = vqaddq_s16(step2[18], step2[45]);
    *out.add(19) = vqaddq_s16(step2[19], step2[44]);
    *out.add(20) = vqaddq_s16(step2[20], step2[43]);
    *out.add(21) = vqaddq_s16(step2[21], step2[42]);
    *out.add(22) = vqaddq_s16(step2[22], step2[41]);
    *out.add(23) = vqaddq_s16(step2[23], step2[40]);
    *out.add(24) = vqaddq_s16(step2[24], step2[39]);
    *out.add(25) = vqaddq_s16(step2[25], step2[38]);
    *out.add(26) = vqaddq_s16(step2[26], step2[37]);
    *out.add(27) = vqaddq_s16(step2[27], step2[36]);
    *out.add(28) = vqaddq_s16(step2[28], step2[35]);
    *out.add(29) = vqaddq_s16(step2[29], step2[34]);
    *out.add(30) = vqaddq_s16(step2[30], step2[33]);
    *out.add(31) = vqaddq_s16(step2[31], step2[32]);
    *out.add(32) = vqsubq_s16(step2[31], step2[32]);
    *out.add(33) = vqsubq_s16(step2[30], step2[33]);
    *out.add(34) = vqsubq_s16(step2[29], step2[34]);
    *out.add(35) = vqsubq_s16(step2[28], step2[35]);
    *out.add(36) = vqsubq_s16(step2[27], step2[36]);
    *out.add(37) = vqsubq_s16(step2[26], step2[37]);
    *out.add(38) = vqsubq_s16(step2[25], step2[38]);
    *out.add(39) = vqsubq_s16(step2[24], step2[39]);
    *out.add(40) = vqsubq_s16(step2[23], step2[40]);
    *out.add(41) = vqsubq_s16(step2[22], step2[41]);
    *out.add(42) = vqsubq_s16(step2[21], step2[42]);
    *out.add(43) = vqsubq_s16(step2[20], step2[43]);
    *out.add(44) = vqsubq_s16(step2[19], step2[44]);
    *out.add(45) = vqsubq_s16(step2[18], step2[45]);
    *out.add(46) = vqsubq_s16(step2[17], step2[46]);
    *out.add(47) = vqsubq_s16(step2[16], step2[47]);
    *out.add(48) = vqsubq_s16(step2[15], step2[48]);
    *out.add(49) = vqsubq_s16(step2[14], step2[49]);
    *out.add(50) = vqsubq_s16(step2[13], step2[50]);
    *out.add(51) = vqsubq_s16(step2[12], step2[51]);
    *out.add(52) = vqsubq_s16(step2[11], step2[52]);
    *out.add(53) = vqsubq_s16(step2[10], step2[53]);
    *out.add(54) = vqsubq_s16(step2[9], step2[54]);
    *out.add(55) = vqsubq_s16(step2[8], step2[55]);
    *out.add(56) = vqsubq_s16(step2[7], step2[56]);
    *out.add(57) = vqsubq_s16(step2[6], step2[57]);
    *out.add(58) = vqsubq_s16(step2[5], step2[58]);
    *out.add(59) = vqsubq_s16(step2[4], step2[59]);
    *out.add(60) = vqsubq_s16(step2[3], step2[60]);
    *out.add(61) = vqsubq_s16(step2[2], step2[61]);
    *out.add(62) = vqsubq_s16(step2[1], step2[62]);
    *out.add(63) = vqsubq_s16(step2[0], step2[63]);
}

#[inline(always)]
unsafe fn idct64_low1_neon(mut input: *mut int16x8_t, mut out: *mut int16x8_t, mut cos_bit: i8) {
    let cospi: *const i32 = cospi_arr(cos_bit as i32);
    let mut step1: int16x8_t = core::mem::zeroed();
    let mut t32: [int32x4_t; 2] = core::mem::zeroed();

    // stage 1
    // stage 2
    // stage 3
    // stage 4
    // stage 5
    // stage 6

    t32[0] = vmull_n_s16(vget_low_s16((*input.add(0))), (*cospi.add(32) as i16));
    t32[1] = vmull_n_s16(vget_high_s16((*input.add(0))), (*cospi.add(32) as i16));

    step1 = vcombine_s16(vrshrn_n_s32(t32[0], INV_COS_BIT), vrshrn_n_s32(t32[1], INV_COS_BIT));
    // stage 7
    // stage 8
    // stage 9
    // stage 10
    // stage 11
    *out.add(0) = step1;
    *out.add(1) = step1;
    *out.add(2) = step1;
    *out.add(3) = step1;
    *out.add(4) = step1;
    *out.add(5) = step1;
    *out.add(6) = step1;
    *out.add(7) = step1;
    *out.add(8) = step1;
    *out.add(9) = step1;
    *out.add(10) = step1;
    *out.add(11) = step1;
    *out.add(12) = step1;
    *out.add(13) = step1;
    *out.add(14) = step1;
    *out.add(15) = step1;
    *out.add(16) = step1;
    *out.add(17) = step1;
    *out.add(18) = step1;
    *out.add(19) = step1;
    *out.add(20) = step1;
    *out.add(21) = step1;
    *out.add(22) = step1;
    *out.add(23) = step1;
    *out.add(24) = step1;
    *out.add(25) = step1;
    *out.add(26) = step1;
    *out.add(27) = step1;
    *out.add(28) = step1;
    *out.add(29) = step1;
    *out.add(30) = step1;
    *out.add(31) = step1;
    *out.add(32) = step1;
    *out.add(33) = step1;
    *out.add(34) = step1;
    *out.add(35) = step1;
    *out.add(36) = step1;
    *out.add(37) = step1;
    *out.add(38) = step1;
    *out.add(39) = step1;
    *out.add(40) = step1;
    *out.add(41) = step1;
    *out.add(42) = step1;
    *out.add(43) = step1;
    *out.add(44) = step1;
    *out.add(45) = step1;
    *out.add(46) = step1;
    *out.add(47) = step1;
    *out.add(48) = step1;
    *out.add(49) = step1;
    *out.add(50) = step1;
    *out.add(51) = step1;
    *out.add(52) = step1;
    *out.add(53) = step1;
    *out.add(54) = step1;
    *out.add(55) = step1;
    *out.add(56) = step1;
    *out.add(57) = step1;
    *out.add(58) = step1;
    *out.add(59) = step1;
    *out.add(60) = step1;
    *out.add(61) = step1;
    *out.add(62) = step1;
    *out.add(63) = step1;
}

#[inline(always)]
unsafe fn idct64_low8_neon(mut in_: *mut int16x8_t, mut out: *mut int16x8_t, mut cos_bit: i8) {
    let cospi: *const i32 = cospi_arr(cos_bit as i32);
    let mut step2: [int16x8_t; 64] = core::mem::zeroed();
    let mut step1: [int16x8_t; 64] = core::mem::zeroed();

    let c0: int16x4_t = set_s16x4_neon(((*cospi.add(4) as i16) as i16), ((*cospi.add(60) as i16) as i16), ((*cospi.add(36) as i16) as i16), ((*cospi.add(28) as i16) as i16));
    let c1: int16x4_t = set_s16x4_neon(((*cospi.add(20) as i16) as i16), ((*cospi.add(44) as i16) as i16), ((*cospi.add(52) as i16) as i16), ((*cospi.add(12) as i16) as i16));
    let c2: int16x4_t = set_s16x4_neon(((*cospi.add(8) as i16) as i16), ((*cospi.add(56) as i16) as i16), ((*cospi.add(40) as i16) as i16), ((*cospi.add(24) as i16) as i16));
    let c3: int16x4_t = set_s16x4_neon(((*cospi.add(32) as i16) as i16), ((*cospi.add(32) as i16) as i16), ((*cospi.add(16) as i16) as i16), ((*cospi.add(48) as i16) as i16));
    let c4: int16x4_t = set_s16x4_neon(((-(*cospi.add(36) as i16)) as i16), ((-(*cospi.add(28) as i16)) as i16), ((-(*cospi.add(52) as i16)) as i16), ((-(*cospi.add(12) as i16)) as i16));
    let c5: int16x4_t = set_s16x4_neon(((-(*cospi.add(8) as i16)) as i16), ((-(*cospi.add(56) as i16)) as i16), ((-(*cospi.add(40) as i16)) as i16), ((-(*cospi.add(24) as i16)) as i16));
    let c6: int16x4_t = set_s16x4_neon(((-(*cospi.add(32) as i16)) as i16), ((-(*cospi.add(32) as i16)) as i16), ((-(*cospi.add(16) as i16)) as i16), ((-(*cospi.add(48) as i16)) as i16));

    // stage 1
    // stage 2

    step2[0] = (*in_.add(0));
    step2[8] = (*in_.add(4));
    step2[16] = (*in_.add(2));
    step2[24] = (*in_.add(6));

    btf_16_neon((*in_.add(1)), (*cospi.add(63) as i16), (*cospi.add(1) as i16), &raw mut step2[32], &raw mut step2[63]);
    btf_16_neon((*in_.add(7)), -(*cospi.add(57) as i16), (*cospi.add(7) as i16), &raw mut step2[39], &raw mut step2[56]);
    btf_16_neon((*in_.add(5)), (*cospi.add(59) as i16), (*cospi.add(5) as i16), &raw mut step2[40], &raw mut step2[55]);
    btf_16_neon((*in_.add(3)), -(*cospi.add(61) as i16), (*cospi.add(3) as i16), &raw mut step2[47], &raw mut step2[48]);

    // stage 3

    step1[0] = step2[0];
    step1[8] = step2[8];

    btf_16_neon(step2[16], (*cospi.add(62) as i16), (*cospi.add(2) as i16), &raw mut step1[16], &raw mut step1[31]);
    btf_16_neon(step2[24], -(*cospi.add(58) as i16), (*cospi.add(6) as i16), &raw mut step1[23], &raw mut step1[24]);

    step1[32] = step2[32];
    step1[33] = step2[32];
    step1[38] = step2[39];
    step1[39] = step2[39];
    step1[40] = step2[40];
    step1[41] = step2[40];
    step1[46] = step2[47];
    step1[47] = step2[47];
    step1[48] = step2[48];
    step1[49] = step2[48];
    step1[54] = step2[55];
    step1[55] = step2[55];
    step1[56] = step2[56];
    step1[57] = step2[56];
    step1[62] = step2[63];
    step1[63] = step2[63];

    // stage 4

    step2[0] = step1[0];

    btf_16_neon(step1[8], (*cospi.add(60) as i16), (*cospi.add(4) as i16), &raw mut step2[8], &raw mut step2[15]);
    btf_16_lane_0_1_neon(step1[62], step1[33], c0, &raw mut step2[62], &raw mut step2[33]);
    btf_16_lane_1_0_neon(step1[38], step1[57], c4, &raw mut step2[38], &raw mut step2[57]);
    btf_16_lane_0_1_neon(step1[54], step1[41], c1, &raw mut step2[54], &raw mut step2[41]);
    btf_16_lane_3_2_neon(step1[46], step1[49], c4, &raw mut step2[46], &raw mut step2[49]);

    step2[16] = step1[16];
    step2[17] = step1[16];
    step2[22] = step1[23];
    step2[23] = step1[23];
    step2[24] = step1[24];
    step2[25] = step1[24];
    step2[30] = step1[31];
    step2[31] = step1[31];
    step2[32] = step1[32];
    step2[39] = step1[39];
    step2[40] = step1[40];
    step2[47] = step1[47];
    step2[48] = step1[48];
    step2[55] = step1[55];
    step2[56] = step1[56];
    step2[63] = step1[63];

    // stage 5

    step1[0] = step2[0];

    btf_16_lane_0_1_neon(step2[30], step2[17], c2, &raw mut step1[30], &raw mut step1[17]);
    btf_16_lane_3_2_neon(step2[22], step2[25], c5, &raw mut step1[22], &raw mut step1[25]);

    step1[8] = step2[8];
    step1[9] = step2[8];
    step1[14] = step2[15];
    step1[15] = step2[15];

    step1[16] = step2[16];
    step1[23] = step2[23];
    step1[24] = step2[24];
    step1[31] = step2[31];
    step1[32] = step2[32];
    step1[33] = step2[33];
    step1[34] = step2[33];
    step1[35] = step2[32];
    step1[36] = step2[39];
    step1[37] = step2[38];
    step1[38] = step2[38];
    step1[39] = step2[39];
    step1[40] = step2[40];
    step1[41] = step2[41];
    step1[42] = step2[41];
    step1[43] = step2[40];
    step1[44] = step2[47];
    step1[45] = step2[46];
    step1[46] = step2[46];
    step1[47] = step2[47];
    step1[48] = step2[48];
    step1[49] = step2[49];
    step1[50] = step2[49];
    step1[51] = step2[48];
    step1[52] = step2[55];
    step1[53] = step2[54];
    step1[54] = step2[54];
    step1[55] = step2[55];
    step1[56] = step2[56];
    step1[57] = step2[57];
    step1[58] = step2[57];
    step1[59] = step2[56];
    step1[60] = step2[63];
    step1[61] = step2[62];
    step1[62] = step2[62];
    step1[63] = step2[63];

    // stage 6

    btf_16_neon(step1[0], (*cospi.add(32) as i16), (*cospi.add(32) as i16), &raw mut step2[0], &raw mut step2[1]);
    btf_16_lane_2_3_neon(step1[14], step1[9], c3, &raw mut step2[14], &raw mut step2[9]);
    btf_16_lane_0_1_neon(step1[61], step1[34], c2, &raw mut step2[61], &raw mut step2[34]);
    btf_16_lane_0_1_neon(step1[60], step1[35], c2, &raw mut step2[60], &raw mut step2[35]);
    btf_16_lane_1_0_neon(step1[36], step1[59], c5, &raw mut step2[36], &raw mut step2[59]);
    btf_16_lane_1_0_neon(step1[37], step1[58], c5, &raw mut step2[37], &raw mut step2[58]);
    btf_16_lane_2_3_neon(step1[53], step1[42], c2, &raw mut step2[53], &raw mut step2[42]);
    btf_16_lane_2_3_neon(step1[52], step1[43], c2, &raw mut step2[52], &raw mut step2[43]);
    btf_16_lane_3_2_neon(step1[44], step1[51], c5, &raw mut step2[44], &raw mut step2[51]);
    btf_16_lane_3_2_neon(step1[45], step1[50], c5, &raw mut step2[45], &raw mut step2[50]);

    step2[8] = step1[8];
    step2[15] = step1[15];
    step2[16] = step1[16];
    step2[17] = step1[17];
    step2[18] = step1[17];
    step2[19] = step1[16];
    step2[20] = step1[23];
    step2[21] = step1[22];
    step2[22] = step1[22];
    step2[23] = step1[23];
    step2[24] = step1[24];
    step2[25] = step1[25];
    step2[26] = step1[25];
    step2[27] = step1[24];
    step2[28] = step1[31];
    step2[29] = step1[30];
    step2[30] = step1[30];
    step2[31] = step1[31];
    step2[32] = step1[32];
    step2[33] = step1[33];
    step2[38] = step1[38];
    step2[39] = step1[39];
    step2[40] = step1[40];
    step2[41] = step1[41];
    step2[46] = step1[46];
    step2[47] = step1[47];
    step2[48] = step1[48];
    step2[49] = step1[49];
    step2[54] = step1[54];
    step2[55] = step1[55];
    step2[56] = step1[56];
    step2[57] = step1[57];
    step2[62] = step1[62];
    step2[63] = step1[63];

    // stage 7

    btf_16_lane_2_3_neon(step2[29], step2[18], c3, &raw mut step1[29], &raw mut step1[18]);
    btf_16_lane_2_3_neon(step2[28], step2[19], c3, &raw mut step1[28], &raw mut step1[19]);
    btf_16_lane_3_2_neon(step2[20], step2[27], c6, &raw mut step1[20], &raw mut step1[27]);
    btf_16_lane_3_2_neon(step2[21], step2[26], c6, &raw mut step1[21], &raw mut step1[26]);

    step1[0] = step2[0];
    step1[1] = step2[1];
    step1[2] = step2[1];
    step1[3] = step2[0];
    step1[8] = step2[8];
    step1[9] = step2[9];
    step1[10] = step2[9];
    step1[11] = step2[8];
    step1[12] = step2[15];
    step1[13] = step2[14];
    step1[14] = step2[14];
    step1[15] = step2[15];
    step1[16] = step2[16];
    step1[17] = step2[17];
    step1[22] = step2[22];
    step1[23] = step2[23];
    step1[24] = step2[24];
    step1[25] = step2[25];
    step1[30] = step2[30];
    step1[31] = step2[31];
    step1[32] = vqaddq_s16(step2[32], step2[39]);
    step1[33] = vqaddq_s16(step2[33], step2[38]);
    step1[34] = vqaddq_s16(step2[34], step2[37]);
    step1[35] = vqaddq_s16(step2[35], step2[36]);
    step1[36] = vqsubq_s16(step2[35], step2[36]);
    step1[37] = vqsubq_s16(step2[34], step2[37]);
    step1[38] = vqsubq_s16(step2[33], step2[38]);
    step1[39] = vqsubq_s16(step2[32], step2[39]);
    step1[40] = vqsubq_s16(step2[47], step2[40]);
    step1[41] = vqsubq_s16(step2[46], step2[41]);
    step1[42] = vqsubq_s16(step2[45], step2[42]);
    step1[43] = vqsubq_s16(step2[44], step2[43]);
    step1[44] = vqaddq_s16(step2[43], step2[44]);
    step1[45] = vqaddq_s16(step2[42], step2[45]);
    step1[46] = vqaddq_s16(step2[41], step2[46]);
    step1[47] = vqaddq_s16(step2[40], step2[47]);
    step1[48] = vqaddq_s16(step2[48], step2[55]);
    step1[49] = vqaddq_s16(step2[49], step2[54]);
    step1[50] = vqaddq_s16(step2[50], step2[53]);
    step1[51] = vqaddq_s16(step2[51], step2[52]);
    step1[52] = vqsubq_s16(step2[51], step2[52]);
    step1[53] = vqsubq_s16(step2[50], step2[53]);
    step1[54] = vqsubq_s16(step2[49], step2[54]);
    step1[55] = vqsubq_s16(step2[48], step2[55]);
    step1[56] = vqsubq_s16(step2[63], step2[56]);
    step1[57] = vqsubq_s16(step2[62], step2[57]);
    step1[58] = vqsubq_s16(step2[61], step2[58]);
    step1[59] = vqsubq_s16(step2[60], step2[59]);
    step1[60] = vqaddq_s16(step2[59], step2[60]);
    step1[61] = vqaddq_s16(step2[58], step2[61]);
    step1[62] = vqaddq_s16(step2[57], step2[62]);
    step1[63] = vqaddq_s16(step2[56], step2[63]);

    // stage 8

    btf_16_lane_0_1_neon(step1[13], step1[10], c3, &raw mut step2[13], &raw mut step2[10]);
    btf_16_lane_0_1_neon(step1[12], step1[11], c3, &raw mut step2[12], &raw mut step2[11]);
    btf_16_lane_2_3_neon(step1[59], step1[36], c3, &raw mut step2[59], &raw mut step2[36]);
    btf_16_lane_2_3_neon(step1[58], step1[37], c3, &raw mut step2[58], &raw mut step2[37]);
    btf_16_lane_2_3_neon(step1[57], step1[38], c3, &raw mut step2[57], &raw mut step2[38]);
    btf_16_lane_2_3_neon(step1[56], step1[39], c3, &raw mut step2[56], &raw mut step2[39]);
    btf_16_lane_3_2_neon(step1[40], step1[55], c6, &raw mut step2[40], &raw mut step2[55]);
    btf_16_lane_3_2_neon(step1[41], step1[54], c6, &raw mut step2[41], &raw mut step2[54]);
    btf_16_lane_3_2_neon(step1[42], step1[53], c6, &raw mut step2[42], &raw mut step2[53]);
    btf_16_lane_3_2_neon(step1[43], step1[52], c6, &raw mut step2[43], &raw mut step2[52]);

    step2[0] = step1[0];
    step2[1] = step1[1];
    step2[2] = step1[2];
    step2[3] = step1[3];
    step2[4] = step1[3];
    step2[5] = step1[2];
    step2[6] = step1[1];
    step2[7] = step1[0];
    step2[8] = step1[8];
    step2[9] = step1[9];
    step2[14] = step1[14];
    step2[15] = step1[15];
    step2[16] = vqaddq_s16(step1[16], step1[23]);
    step2[17] = vqaddq_s16(step1[17], step1[22]);
    step2[18] = vqaddq_s16(step1[18], step1[21]);
    step2[19] = vqaddq_s16(step1[19], step1[20]);
    step2[20] = vqsubq_s16(step1[19], step1[20]);
    step2[21] = vqsubq_s16(step1[18], step1[21]);
    step2[22] = vqsubq_s16(step1[17], step1[22]);
    step2[23] = vqsubq_s16(step1[16], step1[23]);
    step2[24] = vqsubq_s16(step1[31], step1[24]);
    step2[25] = vqsubq_s16(step1[30], step1[25]);
    step2[26] = vqsubq_s16(step1[29], step1[26]);
    step2[27] = vqsubq_s16(step1[28], step1[27]);
    step2[28] = vqaddq_s16(step1[28], step1[27]);
    step2[29] = vqaddq_s16(step1[29], step1[26]);
    step2[30] = vqaddq_s16(step1[30], step1[25]);
    step2[31] = vqaddq_s16(step1[31], step1[24]);
    step2[32] = step1[32];
    step2[33] = step1[33];
    step2[34] = step1[34];
    step2[35] = step1[35];
    step2[44] = step1[44];
    step2[45] = step1[45];
    step2[46] = step1[46];
    step2[47] = step1[47];
    step2[48] = step1[48];
    step2[49] = step1[49];
    step2[50] = step1[50];
    step2[51] = step1[51];
    step2[60] = step1[60];
    step2[61] = step1[61];
    step2[62] = step1[62];
    step2[63] = step1[63];

    // stage 9
    idct64_stage9_neon(step2.as_mut_ptr(), step1.as_mut_ptr(), cos_bit);

    // stage 10
    idct64_stage10_neon(step1.as_mut_ptr(), step2.as_mut_ptr(), cos_bit);

    // stage 11

    *out.add(0) = vqaddq_s16(step2[0], step2[63]);
    *out.add(1) = vqaddq_s16(step2[1], step2[62]);
    *out.add(2) = vqaddq_s16(step2[2], step2[61]);
    *out.add(3) = vqaddq_s16(step2[3], step2[60]);
    *out.add(4) = vqaddq_s16(step2[4], step2[59]);
    *out.add(5) = vqaddq_s16(step2[5], step2[58]);
    *out.add(6) = vqaddq_s16(step2[6], step2[57]);
    *out.add(7) = vqaddq_s16(step2[7], step2[56]);
    *out.add(8) = vqaddq_s16(step2[8], step2[55]);
    *out.add(9) = vqaddq_s16(step2[9], step2[54]);
    *out.add(10) = vqaddq_s16(step2[10], step2[53]);
    *out.add(11) = vqaddq_s16(step2[11], step2[52]);
    *out.add(12) = vqaddq_s16(step2[12], step2[51]);
    *out.add(13) = vqaddq_s16(step2[13], step2[50]);
    *out.add(14) = vqaddq_s16(step2[14], step2[49]);
    *out.add(15) = vqaddq_s16(step2[15], step2[48]);
    *out.add(16) = vqaddq_s16(step2[16], step2[47]);
    *out.add(17) = vqaddq_s16(step2[17], step2[46]);
    *out.add(18) = vqaddq_s16(step2[18], step2[45]);
    *out.add(19) = vqaddq_s16(step2[19], step2[44]);
    *out.add(20) = vqaddq_s16(step2[20], step2[43]);
    *out.add(21) = vqaddq_s16(step2[21], step2[42]);
    *out.add(22) = vqaddq_s16(step2[22], step2[41]);
    *out.add(23) = vqaddq_s16(step2[23], step2[40]);
    *out.add(24) = vqaddq_s16(step2[24], step2[39]);
    *out.add(25) = vqaddq_s16(step2[25], step2[38]);
    *out.add(26) = vqaddq_s16(step2[26], step2[37]);
    *out.add(27) = vqaddq_s16(step2[27], step2[36]);
    *out.add(28) = vqaddq_s16(step2[28], step2[35]);
    *out.add(29) = vqaddq_s16(step2[29], step2[34]);
    *out.add(30) = vqaddq_s16(step2[30], step2[33]);
    *out.add(31) = vqaddq_s16(step2[31], step2[32]);
    *out.add(32) = vqsubq_s16(step2[31], step2[32]);
    *out.add(33) = vqsubq_s16(step2[30], step2[33]);
    *out.add(34) = vqsubq_s16(step2[29], step2[34]);
    *out.add(35) = vqsubq_s16(step2[28], step2[35]);
    *out.add(36) = vqsubq_s16(step2[27], step2[36]);
    *out.add(37) = vqsubq_s16(step2[26], step2[37]);
    *out.add(38) = vqsubq_s16(step2[25], step2[38]);
    *out.add(39) = vqsubq_s16(step2[24], step2[39]);
    *out.add(40) = vqsubq_s16(step2[23], step2[40]);
    *out.add(41) = vqsubq_s16(step2[22], step2[41]);
    *out.add(42) = vqsubq_s16(step2[21], step2[42]);
    *out.add(43) = vqsubq_s16(step2[20], step2[43]);
    *out.add(44) = vqsubq_s16(step2[19], step2[44]);
    *out.add(45) = vqsubq_s16(step2[18], step2[45]);
    *out.add(46) = vqsubq_s16(step2[17], step2[46]);
    *out.add(47) = vqsubq_s16(step2[16], step2[47]);
    *out.add(48) = vqsubq_s16(step2[15], step2[48]);
    *out.add(49) = vqsubq_s16(step2[14], step2[49]);
    *out.add(50) = vqsubq_s16(step2[13], step2[50]);
    *out.add(51) = vqsubq_s16(step2[12], step2[51]);
    *out.add(52) = vqsubq_s16(step2[11], step2[52]);
    *out.add(53) = vqsubq_s16(step2[10], step2[53]);
    *out.add(54) = vqsubq_s16(step2[9], step2[54]);
    *out.add(55) = vqsubq_s16(step2[8], step2[55]);
    *out.add(56) = vqsubq_s16(step2[7], step2[56]);
    *out.add(57) = vqsubq_s16(step2[6], step2[57]);
    *out.add(58) = vqsubq_s16(step2[5], step2[58]);
    *out.add(59) = vqsubq_s16(step2[4], step2[59]);
    *out.add(60) = vqsubq_s16(step2[3], step2[60]);
    *out.add(61) = vqsubq_s16(step2[2], step2[61]);
    *out.add(62) = vqsubq_s16(step2[1], step2[62]);
    *out.add(63) = vqsubq_s16(step2[0], step2[63]);
}

#[inline(always)]
unsafe fn idct64_low16_neon(mut in_: *mut int16x8_t, mut out: *mut int16x8_t, mut cos_bit: i8) {
    let cospi: *const i32 = cospi_arr(cos_bit as i32);
    let mut step2: [int16x8_t; 64] = core::mem::zeroed();
    let mut step1: [int16x8_t; 64] = core::mem::zeroed();

    let c0: int16x4_t = set_s16x4_neon(((*cospi.add(4) as i16) as i16), ((*cospi.add(60) as i16) as i16), ((*cospi.add(36) as i16) as i16), ((*cospi.add(28) as i16) as i16));
    let c1: int16x4_t = set_s16x4_neon(((*cospi.add(20) as i16) as i16), ((*cospi.add(44) as i16) as i16), ((*cospi.add(52) as i16) as i16), ((*cospi.add(12) as i16) as i16));
    let c2: int16x4_t = set_s16x4_neon(((*cospi.add(8) as i16) as i16), ((*cospi.add(56) as i16) as i16), ((*cospi.add(40) as i16) as i16), ((*cospi.add(24) as i16) as i16));
    let c3: int16x4_t = set_s16x4_neon(((*cospi.add(32) as i16) as i16), ((*cospi.add(32) as i16) as i16), ((*cospi.add(16) as i16) as i16), ((*cospi.add(48) as i16) as i16));
    let c4: int16x4_t = set_s16x4_neon(((-(*cospi.add(4) as i16)) as i16), ((-(*cospi.add(60) as i16)) as i16), ((-(*cospi.add(36) as i16)) as i16), ((-(*cospi.add(28) as i16)) as i16));
    let c5: int16x4_t = set_s16x4_neon(((-(*cospi.add(20) as i16)) as i16), ((-(*cospi.add(44) as i16)) as i16), ((-(*cospi.add(52) as i16)) as i16), ((-(*cospi.add(12) as i16)) as i16));
    let c6: int16x4_t = set_s16x4_neon(((-(*cospi.add(8) as i16)) as i16), ((-(*cospi.add(56) as i16)) as i16), ((-(*cospi.add(40) as i16)) as i16), ((-(*cospi.add(24) as i16)) as i16));
    let c7: int16x4_t = set_s16x4_neon(((-(*cospi.add(32) as i16)) as i16), ((-(*cospi.add(32) as i16)) as i16), ((-(*cospi.add(16) as i16)) as i16), ((-(*cospi.add(48) as i16)) as i16));

    // stage 1
    // stage 2

    step2[0] = (*in_.add(0));
    step2[4] = (*in_.add(8));
    step2[8] = (*in_.add(4));
    step2[12] = (*in_.add(12));
    step2[16] = (*in_.add(2));
    step2[20] = (*in_.add(10));
    step2[24] = (*in_.add(6));
    step2[28] = (*in_.add(14));

    btf_16_neon((*in_.add(1)), (*cospi.add(63) as i16), (*cospi.add(1) as i16), &raw mut step2[32], &raw mut step2[63]);
    btf_16_neon((*in_.add(15)), -(*cospi.add(49) as i16), (*cospi.add(15) as i16), &raw mut step2[35], &raw mut step2[60]);
    btf_16_neon((*in_.add(9)), (*cospi.add(55) as i16), (*cospi.add(9) as i16), &raw mut step2[36], &raw mut step2[59]);
    btf_16_neon((*in_.add(7)), -(*cospi.add(57) as i16), (*cospi.add(7) as i16), &raw mut step2[39], &raw mut step2[56]);
    btf_16_neon((*in_.add(5)), (*cospi.add(59) as i16), (*cospi.add(5) as i16), &raw mut step2[40], &raw mut step2[55]);
    btf_16_neon((*in_.add(11)), -(*cospi.add(53) as i16), (*cospi.add(11) as i16), &raw mut step2[43], &raw mut step2[52]);
    btf_16_neon((*in_.add(13)), (*cospi.add(51) as i16), (*cospi.add(13) as i16), &raw mut step2[44], &raw mut step2[51]);
    btf_16_neon((*in_.add(3)), -(*cospi.add(61) as i16), (*cospi.add(3) as i16), &raw mut step2[47], &raw mut step2[48]);

    // stage 3

    step1[0] = step2[0];
    step1[4] = step2[4];
    step1[8] = step2[8];
    step1[12] = step2[12];

    btf_16_neon(step2[16], (*cospi.add(62) as i16), (*cospi.add(2) as i16), &raw mut step1[16], &raw mut step1[31]);
    btf_16_neon(step2[20], (*cospi.add(54) as i16), (*cospi.add(10) as i16), &raw mut step1[20], &raw mut step1[27]);
    btf_16_neon(step2[24], -(*cospi.add(58) as i16), (*cospi.add(6) as i16), &raw mut step1[23], &raw mut step1[24]);
    btf_16_neon(step2[28], -(*cospi.add(50) as i16), (*cospi.add(14) as i16), &raw mut step1[19], &raw mut step1[28]);

    step1[32] = step2[32];
    step1[33] = step2[32];
    step1[34] = step2[35];
    step1[35] = step2[35];
    step1[36] = step2[36];
    step1[37] = step2[36];
    step1[38] = step2[39];
    step1[39] = step2[39];
    step1[40] = step2[40];
    step1[41] = step2[40];
    step1[42] = step2[43];
    step1[43] = step2[43];
    step1[44] = step2[44];
    step1[45] = step2[44];
    step1[46] = step2[47];
    step1[47] = step2[47];
    step1[48] = step2[48];
    step1[49] = step2[48];
    step1[50] = step2[51];
    step1[51] = step2[51];
    step1[52] = step2[52];
    step1[53] = step2[52];
    step1[54] = step2[55];
    step1[55] = step2[55];
    step1[56] = step2[56];
    step1[57] = step2[56];
    step1[58] = step2[59];
    step1[59] = step2[59];
    step1[60] = step2[60];
    step1[61] = step2[60];
    step1[62] = step2[63];
    step1[63] = step2[63];

    // stage 4

    step2[0] = step1[0];
    step2[4] = step1[4];

    btf_16_neon(step1[8], (*cospi.add(60) as i16), (*cospi.add(4) as i16), &raw mut step2[8], &raw mut step2[15]);
    btf_16_neon(step1[12], -(*cospi.add(52) as i16), (*cospi.add(12) as i16), &raw mut step2[11], &raw mut step2[12]);
    btf_16_lane_0_1_neon(step1[62], step1[33], c0, &raw mut step2[62], &raw mut step2[33]);
    btf_16_lane_1_0_neon(step1[34], step1[61], c4, &raw mut step2[34], &raw mut step2[61]);
    btf_16_lane_2_3_neon(step1[58], step1[37], c0, &raw mut step2[58], &raw mut step2[37]);
    btf_16_lane_3_2_neon(step1[38], step1[57], c4, &raw mut step2[38], &raw mut step2[57]);
    btf_16_lane_0_1_neon(step1[54], step1[41], c1, &raw mut step2[54], &raw mut step2[41]);
    btf_16_lane_1_0_neon(step1[42], step1[53], c5, &raw mut step2[42], &raw mut step2[53]);
    btf_16_lane_2_3_neon(step1[50], step1[45], c1, &raw mut step2[50], &raw mut step2[45]);
    btf_16_lane_3_2_neon(step1[46], step1[49], c5, &raw mut step2[46], &raw mut step2[49]);

    step2[16] = step1[16];
    step2[17] = step1[16];
    step2[18] = step1[19];
    step2[19] = step1[19];
    step2[20] = step1[20];
    step2[21] = step1[20];
    step2[22] = step1[23];
    step2[23] = step1[23];
    step2[24] = step1[24];
    step2[25] = step1[24];
    step2[26] = step1[27];
    step2[27] = step1[27];
    step2[28] = step1[28];
    step2[29] = step1[28];
    step2[30] = step1[31];
    step2[31] = step1[31];
    step2[32] = step1[32];
    step2[35] = step1[35];
    step2[36] = step1[36];
    step2[39] = step1[39];
    step2[40] = step1[40];
    step2[43] = step1[43];
    step2[44] = step1[44];
    step2[47] = step1[47];
    step2[48] = step1[48];
    step2[51] = step1[51];
    step2[52] = step1[52];
    step2[55] = step1[55];
    step2[56] = step1[56];
    step2[59] = step1[59];
    step2[60] = step1[60];
    step2[63] = step1[63];

    // stage 5

    step1[0] = step2[0];

    btf_16_neon(step2[4], (*cospi.add(56) as i16), (*cospi.add(8) as i16), &raw mut step1[4], &raw mut step1[7]);
    btf_16_lane_0_1_neon(step2[30], step2[17], c2, &raw mut step1[30], &raw mut step1[17]);
    btf_16_lane_1_0_neon(step2[18], step2[29], c6, &raw mut step1[18], &raw mut step1[29]);
    btf_16_lane_2_3_neon(step2[26], step2[21], c2, &raw mut step1[26], &raw mut step1[21]);
    btf_16_lane_3_2_neon(step2[22], step2[25], c6, &raw mut step1[22], &raw mut step1[25]);

    step1[8] = step2[8];
    step1[9] = step2[8];
    step1[10] = step2[11];
    step1[11] = step2[11];
    step1[12] = step2[12];
    step1[13] = step2[12];
    step1[14] = step2[15];
    step1[15] = step2[15];
    step1[16] = step2[16];
    step1[19] = step2[19];
    step1[20] = step2[20];
    step1[23] = step2[23];
    step1[24] = step2[24];
    step1[27] = step2[27];
    step1[28] = step2[28];
    step1[31] = step2[31];
    step1[32] = vqaddq_s16(step2[32], step2[35]);
    step1[33] = vqaddq_s16(step2[33], step2[34]);
    step1[34] = vqsubq_s16(step2[33], step2[34]);
    step1[35] = vqsubq_s16(step2[32], step2[35]);
    step1[36] = vqsubq_s16(step2[39], step2[36]);
    step1[37] = vqsubq_s16(step2[38], step2[37]);
    step1[38] = vqaddq_s16(step2[38], step2[37]);
    step1[39] = vqaddq_s16(step2[39], step2[36]);
    step1[40] = vqaddq_s16(step2[40], step2[43]);
    step1[41] = vqaddq_s16(step2[41], step2[42]);
    step1[42] = vqsubq_s16(step2[41], step2[42]);
    step1[43] = vqsubq_s16(step2[40], step2[43]);
    step1[44] = vqsubq_s16(step2[47], step2[44]);
    step1[45] = vqsubq_s16(step2[46], step2[45]);
    step1[46] = vqaddq_s16(step2[46], step2[45]);
    step1[47] = vqaddq_s16(step2[47], step2[44]);
    step1[48] = vqaddq_s16(step2[48], step2[51]);
    step1[49] = vqaddq_s16(step2[49], step2[50]);
    step1[50] = vqsubq_s16(step2[49], step2[50]);
    step1[51] = vqsubq_s16(step2[48], step2[51]);
    step1[52] = vqsubq_s16(step2[55], step2[52]);
    step1[53] = vqsubq_s16(step2[54], step2[53]);
    step1[54] = vqaddq_s16(step2[54], step2[53]);
    step1[55] = vqaddq_s16(step2[55], step2[52]);
    step1[56] = vqaddq_s16(step2[56], step2[59]);
    step1[57] = vqaddq_s16(step2[57], step2[58]);
    step1[58] = vqsubq_s16(step2[57], step2[58]);
    step1[59] = vqsubq_s16(step2[56], step2[59]);
    step1[60] = vqsubq_s16(step2[63], step2[60]);
    step1[61] = vqsubq_s16(step2[62], step2[61]);
    step1[62] = vqaddq_s16(step2[62], step2[61]);
    step1[63] = vqaddq_s16(step2[63], step2[60]);

    // stage 6

    btf_16_neon(step1[0], (*cospi.add(32) as i16), (*cospi.add(32) as i16), &raw mut step2[0], &raw mut step2[1]);
    btf_16_lane_2_3_neon(step1[14], step1[9], c3, &raw mut step2[14], &raw mut step2[9]);
    btf_16_lane_3_2_neon(step1[10], step1[13], c7, &raw mut step2[10], &raw mut step2[13]);
    btf_16_lane_0_1_neon(step1[61], step1[34], c2, &raw mut step2[61], &raw mut step2[34]);
    btf_16_lane_0_1_neon(step1[60], step1[35], c2, &raw mut step2[60], &raw mut step2[35]);
    btf_16_lane_1_0_neon(step1[36], step1[59], c6, &raw mut step2[36], &raw mut step2[59]);
    btf_16_lane_1_0_neon(step1[37], step1[58], c6, &raw mut step2[37], &raw mut step2[58]);
    btf_16_lane_2_3_neon(step1[53], step1[42], c2, &raw mut step2[53], &raw mut step2[42]);
    btf_16_lane_2_3_neon(step1[52], step1[43], c2, &raw mut step2[52], &raw mut step2[43]);
    btf_16_lane_3_2_neon(step1[44], step1[51], c6, &raw mut step2[44], &raw mut step2[51]);
    btf_16_lane_3_2_neon(step1[45], step1[50], c6, &raw mut step2[45], &raw mut step2[50]);

    step2[4] = step1[4];
    step2[5] = step1[4];
    step2[6] = step1[7];
    step2[7] = step1[7];
    step2[8] = step1[8];
    step2[11] = step1[11];
    step2[12] = step1[12];
    step2[15] = step1[15];
    step2[16] = vqaddq_s16(step1[16], step1[19]);
    step2[17] = vqaddq_s16(step1[17], step1[18]);
    step2[18] = vqsubq_s16(step1[17], step1[18]);
    step2[19] = vqsubq_s16(step1[16], step1[19]);
    step2[20] = vqsubq_s16(step1[23], step1[20]);
    step2[21] = vqsubq_s16(step1[22], step1[21]);
    step2[22] = vqaddq_s16(step1[22], step1[21]);
    step2[23] = vqaddq_s16(step1[23], step1[20]);
    step2[24] = vqaddq_s16(step1[24], step1[27]);
    step2[25] = vqaddq_s16(step1[25], step1[26]);
    step2[26] = vqsubq_s16(step1[25], step1[26]);
    step2[27] = vqsubq_s16(step1[24], step1[27]);
    step2[28] = vqsubq_s16(step1[31], step1[28]);
    step2[29] = vqsubq_s16(step1[30], step1[29]);
    step2[30] = vqaddq_s16(step1[30], step1[29]);
    step2[31] = vqaddq_s16(step1[31], step1[28]);
    step2[32] = step1[32];
    step2[33] = step1[33];
    step2[38] = step1[38];
    step2[39] = step1[39];
    step2[40] = step1[40];
    step2[41] = step1[41];
    step2[46] = step1[46];
    step2[47] = step1[47];
    step2[48] = step1[48];
    step2[49] = step1[49];
    step2[54] = step1[54];
    step2[55] = step1[55];
    step2[56] = step1[56];
    step2[57] = step1[57];
    step2[62] = step1[62];
    step2[63] = step1[63];

    // stage 7

    btf_16_lane_0_1_neon(step2[6], step2[5], c3, &raw mut step1[6], &raw mut step1[5]);
    btf_16_lane_2_3_neon(step2[29], step2[18], c3, &raw mut step1[29], &raw mut step1[18]);
    btf_16_lane_2_3_neon(step2[28], step2[19], c3, &raw mut step1[28], &raw mut step1[19]);
    btf_16_lane_3_2_neon(step2[20], step2[27], c7, &raw mut step1[20], &raw mut step1[27]);
    btf_16_lane_3_2_neon(step2[21], step2[26], c7, &raw mut step1[21], &raw mut step1[26]);

    step1[0] = step2[0];
    step1[1] = step2[1];
    step1[2] = step2[1];
    step1[3] = step2[0];
    step1[4] = step2[4];
    step1[7] = step2[7];
    step1[8] = vqaddq_s16(step2[8], step2[11]);
    step1[9] = vqaddq_s16(step2[9], step2[10]);
    step1[10] = vqsubq_s16(step2[9], step2[10]);
    step1[11] = vqsubq_s16(step2[8], step2[11]);
    step1[12] = vqsubq_s16(step2[15], step2[12]);
    step1[13] = vqsubq_s16(step2[14], step2[13]);
    step1[14] = vqaddq_s16(step2[14], step2[13]);
    step1[15] = vqaddq_s16(step2[15], step2[12]);
    step1[16] = step2[16];
    step1[17] = step2[17];
    step1[22] = step2[22];
    step1[23] = step2[23];
    step1[24] = step2[24];
    step1[25] = step2[25];
    step1[30] = step2[30];
    step1[31] = step2[31];
    step1[32] = vqaddq_s16(step2[32], step2[39]);
    step1[33] = vqaddq_s16(step2[33], step2[38]);
    step1[34] = vqaddq_s16(step2[34], step2[37]);
    step1[35] = vqaddq_s16(step2[35], step2[36]);
    step1[36] = vqsubq_s16(step2[35], step2[36]);
    step1[37] = vqsubq_s16(step2[34], step2[37]);
    step1[38] = vqsubq_s16(step2[33], step2[38]);
    step1[39] = vqsubq_s16(step2[32], step2[39]);
    step1[40] = vqsubq_s16(step2[47], step2[40]);
    step1[41] = vqsubq_s16(step2[46], step2[41]);
    step1[42] = vqsubq_s16(step2[45], step2[42]);
    step1[43] = vqsubq_s16(step2[44], step2[43]);
    step1[44] = vqaddq_s16(step2[43], step2[44]);
    step1[45] = vqaddq_s16(step2[42], step2[45]);
    step1[46] = vqaddq_s16(step2[41], step2[46]);
    step1[47] = vqaddq_s16(step2[40], step2[47]);
    step1[48] = vqaddq_s16(step2[48], step2[55]);
    step1[49] = vqaddq_s16(step2[49], step2[54]);
    step1[50] = vqaddq_s16(step2[50], step2[53]);
    step1[51] = vqaddq_s16(step2[51], step2[52]);
    step1[52] = vqsubq_s16(step2[51], step2[52]);
    step1[53] = vqsubq_s16(step2[50], step2[53]);
    step1[54] = vqsubq_s16(step2[49], step2[54]);
    step1[55] = vqsubq_s16(step2[48], step2[55]);
    step1[56] = vqsubq_s16(step2[63], step2[56]);
    step1[57] = vqsubq_s16(step2[62], step2[57]);
    step1[58] = vqsubq_s16(step2[61], step2[58]);
    step1[59] = vqsubq_s16(step2[60], step2[59]);
    step1[60] = vqaddq_s16(step2[59], step2[60]);
    step1[61] = vqaddq_s16(step2[58], step2[61]);
    step1[62] = vqaddq_s16(step2[57], step2[62]);
    step1[63] = vqaddq_s16(step2[56], step2[63]);

    // stage 8

    btf_16_lane_0_1_neon(step1[13], step1[10], c3, &raw mut step2[13], &raw mut step2[10]);
    btf_16_lane_0_1_neon(step1[12], step1[11], c3, &raw mut step2[12], &raw mut step2[11]);
    btf_16_lane_2_3_neon(step1[59], step1[36], c3, &raw mut step2[59], &raw mut step2[36]);
    btf_16_lane_2_3_neon(step1[58], step1[37], c3, &raw mut step2[58], &raw mut step2[37]);
    btf_16_lane_2_3_neon(step1[57], step1[38], c3, &raw mut step2[57], &raw mut step2[38]);
    btf_16_lane_2_3_neon(step1[56], step1[39], c3, &raw mut step2[56], &raw mut step2[39]);
    btf_16_lane_3_2_neon(step1[40], step1[55], c7, &raw mut step2[40], &raw mut step2[55]);
    btf_16_lane_3_2_neon(step1[41], step1[54], c7, &raw mut step2[41], &raw mut step2[54]);
    btf_16_lane_3_2_neon(step1[42], step1[53], c7, &raw mut step2[42], &raw mut step2[53]);
    btf_16_lane_3_2_neon(step1[43], step1[52], c7, &raw mut step2[43], &raw mut step2[52]);

    step2[0] = vqaddq_s16(step1[0], step1[7]);
    step2[1] = vqaddq_s16(step1[1], step1[6]);
    step2[2] = vqaddq_s16(step1[2], step1[5]);
    step2[3] = vqaddq_s16(step1[3], step1[4]);
    step2[4] = vqsubq_s16(step1[3], step1[4]);
    step2[5] = vqsubq_s16(step1[2], step1[5]);
    step2[6] = vqsubq_s16(step1[1], step1[6]);
    step2[7] = vqsubq_s16(step1[0], step1[7]);
    step2[8] = step1[8];
    step2[9] = step1[9];
    step2[14] = step1[14];
    step2[15] = step1[15];
    step2[16] = vqaddq_s16(step1[16], step1[23]);
    step2[17] = vqaddq_s16(step1[17], step1[22]);
    step2[18] = vqaddq_s16(step1[18], step1[21]);
    step2[19] = vqaddq_s16(step1[19], step1[20]);
    step2[20] = vqsubq_s16(step1[19], step1[20]);
    step2[21] = vqsubq_s16(step1[18], step1[21]);
    step2[22] = vqsubq_s16(step1[17], step1[22]);
    step2[23] = vqsubq_s16(step1[16], step1[23]);
    step2[24] = vqsubq_s16(step1[31], step1[24]);
    step2[25] = vqsubq_s16(step1[30], step1[25]);
    step2[26] = vqsubq_s16(step1[29], step1[26]);
    step2[27] = vqsubq_s16(step1[28], step1[27]);
    step2[28] = vqaddq_s16(step1[28], step1[27]);
    step2[29] = vqaddq_s16(step1[29], step1[26]);
    step2[30] = vqaddq_s16(step1[30], step1[25]);
    step2[31] = vqaddq_s16(step1[31], step1[24]);
    step2[32] = step1[32];
    step2[33] = step1[33];
    step2[34] = step1[34];
    step2[35] = step1[35];
    step2[44] = step1[44];
    step2[45] = step1[45];
    step2[46] = step1[46];
    step2[47] = step1[47];
    step2[48] = step1[48];
    step2[49] = step1[49];
    step2[50] = step1[50];
    step2[51] = step1[51];
    step2[60] = step1[60];
    step2[61] = step1[61];
    step2[62] = step1[62];
    step2[63] = step1[63];

    // stage 9
    idct64_stage9_neon(step2.as_mut_ptr(), step1.as_mut_ptr(), cos_bit);

    // stage 10
    idct64_stage10_neon(step1.as_mut_ptr(), step2.as_mut_ptr(), cos_bit);

    // stage 11

    *out.add(0) = vqaddq_s16(step2[0], step2[63]);
    *out.add(1) = vqaddq_s16(step2[1], step2[62]);
    *out.add(2) = vqaddq_s16(step2[2], step2[61]);
    *out.add(3) = vqaddq_s16(step2[3], step2[60]);
    *out.add(4) = vqaddq_s16(step2[4], step2[59]);
    *out.add(5) = vqaddq_s16(step2[5], step2[58]);
    *out.add(6) = vqaddq_s16(step2[6], step2[57]);
    *out.add(7) = vqaddq_s16(step2[7], step2[56]);
    *out.add(8) = vqaddq_s16(step2[8], step2[55]);
    *out.add(9) = vqaddq_s16(step2[9], step2[54]);
    *out.add(10) = vqaddq_s16(step2[10], step2[53]);
    *out.add(11) = vqaddq_s16(step2[11], step2[52]);
    *out.add(12) = vqaddq_s16(step2[12], step2[51]);
    *out.add(13) = vqaddq_s16(step2[13], step2[50]);
    *out.add(14) = vqaddq_s16(step2[14], step2[49]);
    *out.add(15) = vqaddq_s16(step2[15], step2[48]);
    *out.add(16) = vqaddq_s16(step2[16], step2[47]);
    *out.add(17) = vqaddq_s16(step2[17], step2[46]);
    *out.add(18) = vqaddq_s16(step2[18], step2[45]);
    *out.add(19) = vqaddq_s16(step2[19], step2[44]);
    *out.add(20) = vqaddq_s16(step2[20], step2[43]);
    *out.add(21) = vqaddq_s16(step2[21], step2[42]);
    *out.add(22) = vqaddq_s16(step2[22], step2[41]);
    *out.add(23) = vqaddq_s16(step2[23], step2[40]);
    *out.add(24) = vqaddq_s16(step2[24], step2[39]);
    *out.add(25) = vqaddq_s16(step2[25], step2[38]);
    *out.add(26) = vqaddq_s16(step2[26], step2[37]);
    *out.add(27) = vqaddq_s16(step2[27], step2[36]);
    *out.add(28) = vqaddq_s16(step2[28], step2[35]);
    *out.add(29) = vqaddq_s16(step2[29], step2[34]);
    *out.add(30) = vqaddq_s16(step2[30], step2[33]);
    *out.add(31) = vqaddq_s16(step2[31], step2[32]);
    *out.add(32) = vqsubq_s16(step2[31], step2[32]);
    *out.add(33) = vqsubq_s16(step2[30], step2[33]);
    *out.add(34) = vqsubq_s16(step2[29], step2[34]);
    *out.add(35) = vqsubq_s16(step2[28], step2[35]);
    *out.add(36) = vqsubq_s16(step2[27], step2[36]);
    *out.add(37) = vqsubq_s16(step2[26], step2[37]);
    *out.add(38) = vqsubq_s16(step2[25], step2[38]);
    *out.add(39) = vqsubq_s16(step2[24], step2[39]);
    *out.add(40) = vqsubq_s16(step2[23], step2[40]);
    *out.add(41) = vqsubq_s16(step2[22], step2[41]);
    *out.add(42) = vqsubq_s16(step2[21], step2[42]);
    *out.add(43) = vqsubq_s16(step2[20], step2[43]);
    *out.add(44) = vqsubq_s16(step2[19], step2[44]);
    *out.add(45) = vqsubq_s16(step2[18], step2[45]);
    *out.add(46) = vqsubq_s16(step2[17], step2[46]);
    *out.add(47) = vqsubq_s16(step2[16], step2[47]);
    *out.add(48) = vqsubq_s16(step2[15], step2[48]);
    *out.add(49) = vqsubq_s16(step2[14], step2[49]);
    *out.add(50) = vqsubq_s16(step2[13], step2[50]);
    *out.add(51) = vqsubq_s16(step2[12], step2[51]);
    *out.add(52) = vqsubq_s16(step2[11], step2[52]);
    *out.add(53) = vqsubq_s16(step2[10], step2[53]);
    *out.add(54) = vqsubq_s16(step2[9], step2[54]);
    *out.add(55) = vqsubq_s16(step2[8], step2[55]);
    *out.add(56) = vqsubq_s16(step2[7], step2[56]);
    *out.add(57) = vqsubq_s16(step2[6], step2[57]);
    *out.add(58) = vqsubq_s16(step2[5], step2[58]);
    *out.add(59) = vqsubq_s16(step2[4], step2[59]);
    *out.add(60) = vqsubq_s16(step2[3], step2[60]);
    *out.add(61) = vqsubq_s16(step2[2], step2[61]);
    *out.add(62) = vqsubq_s16(step2[1], step2[62]);
    *out.add(63) = vqsubq_s16(step2[0], step2[63]);
}



// Functions for blocks with eob at DC and within
// topleft 8x8, 16x16, 32x32 corner
static lowbd_txfm_all_1d_zeros_w_arr: [[[transform_neon; 4]; ITX_TYPES_1D]; TX_SIZES] = [
    [[None, None, None, None], [None, None, None, None], [None, None, None, None]],
    [
        [Some(idct8_low1_neon), Some(idct8_neon), None, None],
        [Some(iadst8_low1_neon), Some(iadst8_neon), None, None],
        [None, None, None, None],
    ],
    [
        [Some(idct16_low1_neon), Some(idct16_low8_neon), Some(idct16_neon), None],
        [Some(iadst16_low1_neon), Some(iadst16_low8_neon), Some(iadst16_neon), None],
        [None, None, None, None],
    ],
    [
        [Some(idct32_low1_neon), Some(idct32_low8_neon), Some(idct32_low16_neon), Some(idct32_neon)],
        [None, None, None, None],
        [None, None, None, None],
    ],
    [
        [Some(idct64_low1_neon), Some(idct64_low8_neon), Some(idct64_low16_neon), Some(idct64_low32_neon)],
        [None, None, None, None],
        [None, None, None, None],
    ],
];

#[inline(always)]
unsafe fn lowbd_inv_txfm2d_add_idtx_neon(
    mut input: *const i32,
    mut output: *mut u8,
    mut stride: i32,
    mut tx_type: TX_TYPE,
    mut tx_size: TX_SIZE,
    mut eob: i32,
) {
    let _ = tx_type;
    let mut a: [int16x8_t; 32 * 4] = core::mem::zeroed();
    let mut b: [int16x8_t; 32 * 4] = core::mem::zeroed();
    let mut eobx: i32 = 0;
    let mut eoby: i32 = 0;
    get_eobx_eoby_scan_default(&raw mut eobx, &raw mut eoby, tx_size, eob);
    let shift: *const i8 = av1_inv_txfm_shift_ls[tx_size as usize];
    let txw_idx: i32 = get_txw_idx(tx_size);
    let txh_idx: i32 = get_txh_idx(tx_size);
    let txfm_size_col: i32 = tx_size_wide[tx_size as usize];
    let txfm_size_row: i32 = tx_size_high[tx_size as usize];
    lowbd_inv_txfm2d_memset_neon(&raw mut a[0], (txfm_size_col * (txfm_size_row) >> 3), 0);
    lowbd_inv_txfm2d_memset_neon(&raw mut b[0], (txfm_size_col * (txfm_size_row) >> 3), 0);
    let buf_size_w_div8: i32 = txfm_size_col >> 3;
    let rect_type: i32 = get_rect_tx_log_ratio(txfm_size_col, txfm_size_row);
    let buf_size_nonzero_h_div8: i32 = (eoby + 8) >> 3;
    let buf_size_nonzero_w: i32 = (eobx + 8) >> 3 << 3;
    let input_stride: i32 = txfm_size_row;
    let mut temp_b: i32 = 0;

    for i in 0..buf_size_nonzero_h_div8 {
        let cur_a: *mut int16x8_t = &raw mut a[(i * txfm_size_col) as usize];
        load_buffer_32bit_to_16bit_neon(input, input_stride, cur_a, buf_size_nonzero_w);
        input = input.add(8);
        if rect_type.abs() == 1 {
            round_shift_for_rect(cur_a, cur_a, buf_size_nonzero_w);
        }
        identity_txfm_round_neon(cur_a, cur_a, txw_idx, buf_size_nonzero_w as i8, -(*shift as i32));
        for j in 0..buf_size_w_div8 {
            transpose_arrays_s16_8x8(
                cur_a.offset((j * 8) as isize),
                &raw mut b[(temp_b + txfm_size_row * j) as usize],
            );
        }
        temp_b += 8;
    }
    for j in 0..buf_size_w_div8 {
        identity_txfm_round_neon(
            &raw mut b[(j * txfm_size_row) as usize],
            &raw mut b[(j * txfm_size_row) as usize],
            txh_idx,
            txfm_size_row as i8,
            -(*shift.add(1) as i32),
        );
    }
    if txfm_size_col >= 16 {
        for i in 0..(txfm_size_col >> 4) {
            lowbd_add_flip_buffer_16xn_neon(
                &raw mut b[(i * txfm_size_row * 2) as usize],
                output.offset((16 * i) as isize),
                stride,
                0,
                txfm_size_row,
            );
        }
    } else if txfm_size_col == 8 {
        lowbd_add_flip_buffer_8xn_neon(b.as_mut_ptr(), output, stride, 0, txfm_size_row);
    }
}

#[inline(always)]
unsafe fn lowbd_inv_txfm2d_add_v_identity_neon(
    mut input: *const i32,
    mut output: *mut u8,
    mut stride: i32,
    mut tx_type: TX_TYPE,
    mut tx_size: TX_SIZE,
    mut eob: i32,
) {
    let mut a: [int16x8_t; 16 * 2] = core::mem::zeroed();
    let mut b: [int16x8_t; 16 * 2] = core::mem::zeroed();
    let mut eobx: i32 = 0;
    let mut eoby: i32 = 0;
    let mut ud_flip: i32 = 0;
    let mut lr_flip: i32 = 0;
    get_eobx_eoby_scan_v_identity(&raw mut eobx, &raw mut eoby, tx_size, eob);
    let shift: *const i8 = av1_inv_txfm_shift_ls[tx_size as usize];
    let txw_idx: i32 = get_txw_idx(tx_size);
    let txh_idx: i32 = get_txh_idx(tx_size);
    let txfm_size_col: i32 = tx_size_wide[tx_size as usize];
    let txfm_size_row: i32 = tx_size_high[tx_size as usize];
    lowbd_inv_txfm2d_memset_neon(&raw mut b[0], (txfm_size_col * (txfm_size_row) >> 3), 0);
    let rect_type: i32 = get_rect_tx_log_ratio(txfm_size_col, txfm_size_row);
    let buf_size_w_div8: i32 = txfm_size_col >> 3;
    let buf_size_nonzero_h_div8: i32 = (eoby + 8) >> 3;
    let buf_size_nonzero_w: i32 = (eobx + 8) >> 3 << 3;
    let input_stride: i32 = txfm_size_row;
    let fun_idx_x: i32 = lowbd_txfm_all_1d_zeros_idx[eobx as usize];
    let mut temp_b: i32 = 0;
    let row_txfm: transform_neon = lowbd_txfm_all_1d_zeros_w_arr[txw_idx as usize]
        [hitx_1d_tab[tx_type as usize] as usize][fun_idx_x as usize];

    debug_assert!(row_txfm.is_some());

    get_flip_cfg(tx_type, &raw mut ud_flip, &raw mut lr_flip);

    for i in 0..buf_size_nonzero_h_div8 {
        let cur_a: *mut int16x8_t = &raw mut a[(i * txfm_size_col) as usize];
        load_buffer_32bit_to_16bit_neon(input, input_stride, cur_a, buf_size_nonzero_w);
        input = input.add(8);
        if rect_type.abs() == 1 {
            round_shift_for_rect(cur_a, cur_a, buf_size_nonzero_w);
        }
        row_txfm.unwrap()(cur_a, cur_a, INV_COS_BIT as i8);
        round_shift_array_16_neon(cur_a, txfm_size_col, -(*shift as i32));
        if lr_flip == 1 {
            for j in 0..buf_size_w_div8 {
                flip_buf_ud_neon(cur_a.offset((j * 8) as isize), 8);
                transpose_arrays_s16_8x8(
                    cur_a.offset((j * 8) as isize),
                    &raw mut b[(temp_b + txfm_size_row * (buf_size_w_div8 - 1 - j)) as usize],
                );
            }
            temp_b += 8;
        } else {
            for j in 0..buf_size_w_div8 {
                transpose_arrays_s16_8x8(
                    cur_a.offset((j * 8) as isize),
                    &raw mut b[(temp_b + txfm_size_row * j) as usize],
                );
            }
            temp_b += 8;
        }
    }
    for j in 0..buf_size_w_div8 {
        identity_txfm_round_neon(
            &raw mut b[(j * txfm_size_row) as usize],
            &raw mut b[(j * txfm_size_row) as usize],
            txh_idx,
            txfm_size_row as i8,
            -(*shift.add(1) as i32),
        );
    }
    if txfm_size_col >= 16 {
        for i in 0..(txfm_size_col >> 4) {
            lowbd_add_flip_buffer_16xn_neon(
                &raw mut b[(i * txfm_size_row * 2) as usize],
                output.offset((16 * i) as isize),
                stride,
                0,
                txfm_size_row,
            );
        }
    } else if txfm_size_col == 8 {
        lowbd_add_flip_buffer_8xn_neon(b.as_mut_ptr(), output, stride, 0, txfm_size_row);
    }
}

#[inline(always)]
unsafe fn lowbd_inv_txfm2d_add_h_identity_neon(
    mut input: *const i32,
    mut output: *mut u8,
    mut stride: i32,
    mut tx_type: TX_TYPE,
    mut tx_size: TX_SIZE,
    mut eob: i32,
) {
    let mut a: [int16x8_t; 16 * 2] = core::mem::zeroed();
    let mut b: [int16x8_t; 16 * 2] = core::mem::zeroed();
    let mut eobx: i32 = 0;
    let mut eoby: i32 = 0;
    let mut ud_flip: i32 = 0;
    let mut lr_flip: i32 = 0;
    get_eobx_eoby_scan_h_identity(&raw mut eobx, &raw mut eoby, tx_size, eob);
    let shift: *const i8 = av1_inv_txfm_shift_ls[tx_size as usize];
    let txw_idx: i32 = get_txw_idx(tx_size);
    let txh_idx: i32 = get_txh_idx(tx_size);
    let txfm_size_col: i32 = tx_size_wide[tx_size as usize];
    let txfm_size_row: i32 = tx_size_high[tx_size as usize];
    lowbd_inv_txfm2d_memset_neon(&raw mut a[0], (txfm_size_col * (txfm_size_row) >> 3), 0);
    let buf_size_w_div8: i32 = txfm_size_col >> 3;
    let rect_type: i32 = get_rect_tx_log_ratio(txfm_size_col, txfm_size_row);
    let buf_size_nonzero_h_div8: i32 = (eoby + 8) >> 3;
    let buf_size_nonzero_w: i32 = (eobx + 8) >> 3 << 3;
    let input_stride: i32 = txfm_size_row;
    let fun_idx_y: i32 = lowbd_txfm_all_1d_zeros_idx[eoby as usize];
    let mut temp_b: i32 = 0;
    let col_txfm: transform_neon = lowbd_txfm_all_1d_zeros_w_arr[txh_idx as usize]
        [vitx_1d_tab[tx_type as usize] as usize][fun_idx_y as usize];

    debug_assert!(col_txfm.is_some());

    get_flip_cfg(tx_type, &raw mut ud_flip, &raw mut lr_flip);

    for i in 0..buf_size_nonzero_h_div8 {
        let cur_a: *mut int16x8_t = &raw mut a[(i * txfm_size_col) as usize];
        load_buffer_32bit_to_16bit_neon(input, input_stride, cur_a, buf_size_nonzero_w);
        input = input.add(8);
        if rect_type.abs() == 1 {
            round_shift_for_rect(cur_a, cur_a, buf_size_nonzero_w);
        }
        identity_txfm_round_neon(cur_a, cur_a, txw_idx, buf_size_nonzero_w as i8, -(*shift as i32));
        for j in 0..buf_size_w_div8 {
            transpose_arrays_s16_8x8(
                cur_a.offset((j * 8) as isize),
                &raw mut b[(temp_b + txfm_size_row * j) as usize],
            );
        }
        temp_b += 8;
    }
    for j in 0..buf_size_w_div8 {
        col_txfm.unwrap()(
            &raw mut b[(j * txfm_size_row) as usize],
            &raw mut b[(j * txfm_size_row) as usize],
            INV_COS_BIT as i8,
        );
        round_shift_array_16_neon(
            &raw mut b[(j * txfm_size_row) as usize],
            txfm_size_row,
            -(*shift.add(1) as i32),
        );
    }
    if txfm_size_col >= 16 {
        for i in 0..(txfm_size_col >> 4) {
            lowbd_add_flip_buffer_16xn_neon(
                &raw mut b[(i * txfm_size_row * 2) as usize],
                output.offset((16 * i) as isize),
                stride,
                ud_flip,
                txfm_size_row,
            );
        }
    } else if txfm_size_col == 8 {
        lowbd_add_flip_buffer_8xn_neon(b.as_mut_ptr(), output, stride, ud_flip, txfm_size_row);
    }
}

// The original repeats this body for TX_4X4, TX_4X8, TX_8X4, TX_4X16 and
// TX_16X4; they differ only in the transform size, the buffer extent, the
// stage_range initializer (all 16 in the used entries), and whether the row
// input is scaled by NewInvSqrt2 (4x8, 8x4) or the row output is round-shifted
// by shift[0] (4x16, 16x4). The macro keeps each instance's statement order.
macro_rules! lowbd_inv_txfm2d_add_small_neon {
    ($name:ident, $tx_size:expr, $buf_len:expr, rect_scale: $rect:literal, row_shift: $row_shift:literal) => {
        #[inline(always)]
        unsafe fn $name(
            mut input: *const i32,
            mut output: *mut u8,
            mut stride: i32,
            mut tx_type: TX_TYPE,
            mut eob: i32,
        ) {
            let _ = eob;
            let tx_size: TX_SIZE = $tx_size as TX_SIZE;
            let mut txfm_buf: crate::platform::Aligned32<[i32; $buf_len]> =
                crate::platform::Aligned32([0; $buf_len]);
            let temp_in: *mut i32 = txfm_buf.0.as_mut_ptr();

            let shift: *const i8 = av1_inv_txfm_shift_ls[tx_size as usize];
            let txw_idx: i32 = get_txw_idx(tx_size);
            let txh_idx: i32 = get_txh_idx(tx_size);
            let txfm_size_col: i32 = tx_size_wide[tx_size as usize];
            let txfm_size_row: i32 = tx_size_high[tx_size as usize];
            let buf_offset: i32 = AOMMAX(txfm_size_row, txfm_size_col);
            let temp_out: *mut i32 = temp_in.offset(buf_offset as isize);
            let buf: *mut i32 = temp_out.offset(buf_offset as isize);
            let mut buf_ptr: *mut i32 = buf;
            let mut stage_range: [i8; MAX_TXFM_STAGE_NUM] = [0; MAX_TXFM_STAGE_NUM];
            // The initializers list 7 (4x4), 8 (4x8, 8x4) or 10 (4x16, 16x4)
            // entries of 16; the transforms used here read fewer stages.
            let stage_count: usize = match tx_size as i32 {
                TX_4X4 => 7,
                TX_4X8 | TX_8X4 => 8,
                _ => 10,
            };
            for s in 0..stage_count {
                stage_range[s] = 16;
            }
            let mut r: i32 = 0;
            let row_txfm: transform_1d_neon =
                lowbd_txfm_all_1d_arr[txw_idx as usize][hitx_1d_tab[tx_type as usize] as usize];
            let col_txfm: transform_1d_neon =
                lowbd_txfm_all_1d_arr[txh_idx as usize][vitx_1d_tab[tx_type as usize] as usize];

            let mut ud_flip: i32 = 0;
            let mut lr_flip: i32 = 0;
            get_flip_cfg(tx_type, &raw mut ud_flip, &raw mut lr_flip);

            for i in 0..txfm_size_row {
                for c in 0..txfm_size_col {
                    if $rect {
                        *temp_in.offset(c as isize) = round_shift(
                            *input.offset((c * txfm_size_row) as isize) as i64 * NewInvSqrt2 as i64,
                            NewSqrt2Bits,
                        );
                    } else {
                        *temp_in.offset(c as isize) = *input.offset((c * txfm_size_row) as isize);
                    }
                }
                row_txfm.unwrap()(temp_in, buf_ptr, INV_COS_BIT as i8, stage_range.as_ptr());
                if $row_shift {
                    av1_round_shift_array_c(buf_ptr, txfm_size_col, -(*shift as i32));
                }
                input = input.add(1);
                buf_ptr = buf_ptr.offset(txfm_size_col as isize);
            }

            for c in 0..txfm_size_col {
                if lr_flip == 0 {
                    r = 0;
                    while r < txfm_size_row {
                        *temp_in.offset(r as isize) = *buf.offset((r * txfm_size_col + c) as isize);
                        r += 1;
                    }
                } else {
                    // flip left right
                    r = 0;
                    while r < txfm_size_row {
                        *temp_in.offset(r as isize) =
                            *buf.offset((r * txfm_size_col + (txfm_size_col - c - 1)) as isize);
                        r += 1;
                    }
                }
                clamp_buf(temp_in, txfm_size_row, 16);
                col_txfm.unwrap()(temp_in, temp_out, INV_COS_BIT as i8, stage_range.as_ptr());
                av1_round_shift_array_c(temp_out, txfm_size_row, -(*shift.add(1) as i32));

                if ud_flip == 0 {
                    r = 0;
                    while r < txfm_size_row {
                        *output.offset((r * stride + c) as isize) = clip_pixel(
                            *output.offset((r * stride + c) as isize) as i32
                                + *temp_out.offset(r as isize),
                        );
                        r += 1;
                    }
                } else {
                    // flip upside down
                    r = 0;
                    while r < txfm_size_row {
                        *output.offset((r * stride + c) as isize) = clip_pixel(
                            *output.offset((r * stride + c) as isize) as i32
                                + *temp_out.offset((txfm_size_row - r - 1) as isize),
                        );
                        r += 1;
                    }
                }
            }
        }
    };
}

lowbd_inv_txfm2d_add_small_neon!(lowbd_inv_txfm2d_add_4x4_neon, TX_4X4, 4 * 4 + 8 + 8,
                                 rect_scale: false, row_shift: false);
lowbd_inv_txfm2d_add_small_neon!(lowbd_inv_txfm2d_add_4x8_neon, TX_4X8, 4 * 8 + 8 + 8,
                                 rect_scale: true, row_shift: false);
lowbd_inv_txfm2d_add_small_neon!(lowbd_inv_txfm2d_add_8x4_neon, TX_8X4, 8 * 4 + 8 + 8,
                                 rect_scale: true, row_shift: false);
lowbd_inv_txfm2d_add_small_neon!(lowbd_inv_txfm2d_add_4x16_neon, TX_4X16, 4 * 16 + 16 + 16,
                                 rect_scale: false, row_shift: true);
lowbd_inv_txfm2d_add_small_neon!(lowbd_inv_txfm2d_add_16x4_neon, TX_16X4, 16 * 4 + 16 + 16,
                                 rect_scale: false, row_shift: true);

#[inline(always)]
unsafe fn lowbd_inv_txfm2d_add_no_identity_neon(
    mut input: *const i32,
    mut output: *mut u8,
    mut stride: i32,
    mut tx_type: TX_TYPE,
    mut tx_size: TX_SIZE,
    mut eob: i32,
) {
    let mut a: [int16x8_t; 64 * 8] = core::mem::zeroed();
    let mut b: [int16x8_t; 64 * 8] = core::mem::zeroed();
    let mut eobx: i32 = 0;
    let mut eoby: i32 = 0;
    let mut ud_flip: i32 = 0;
    let mut lr_flip: i32 = 0;
    get_eobx_eoby_scan_default(&raw mut eobx, &raw mut eoby, tx_size, eob);
    let shift: *const i8 = av1_inv_txfm_shift_ls[tx_size as usize];
    let txw_idx: i32 = get_txw_idx(tx_size);
    let txh_idx: i32 = get_txh_idx(tx_size);
    let txfm_size_col: i32 = tx_size_wide[tx_size as usize];
    let txfm_size_row: i32 = tx_size_high[tx_size as usize];
    let rect_type: i32 = get_rect_tx_log_ratio(txfm_size_col, txfm_size_row);
    let buf_size_w_div8: i32 = txfm_size_col >> 3;
    let buf_size_nonzero_h_div8: i32 = (eoby + 8) >> 3;
    let buf_size_nonzero_w: i32 = (eobx + 8) >> 3 << 3;
    let input_stride: i32 = AOMMIN(32, txfm_size_row);
    let fun_idx_x: i32 = lowbd_txfm_all_1d_zeros_idx[eobx as usize];
    let fun_idx_y: i32 = lowbd_txfm_all_1d_zeros_idx[eoby as usize];
    let mut temp_b: i32 = 0;

    let row_txfm: transform_neon = lowbd_txfm_all_1d_zeros_w_arr[txw_idx as usize]
        [hitx_1d_tab[tx_type as usize] as usize][fun_idx_x as usize];
    let col_txfm: transform_neon = lowbd_txfm_all_1d_zeros_w_arr[txh_idx as usize]
        [vitx_1d_tab[tx_type as usize] as usize][fun_idx_y as usize];

    debug_assert!(col_txfm.is_some());
    debug_assert!(row_txfm.is_some());

    get_flip_cfg(tx_type, &raw mut ud_flip, &raw mut lr_flip);

    for i in 0..buf_size_nonzero_h_div8 {
        let cur_a: *mut int16x8_t = &raw mut a[(i * txfm_size_col) as usize];
        load_buffer_32bit_to_16bit_neon(input, input_stride, cur_a, buf_size_nonzero_w);
        input = input.add(8);
        if rect_type.abs() == 1 {
            round_shift_for_rect(cur_a, cur_a, buf_size_nonzero_w);
        }
        row_txfm.unwrap()(cur_a, cur_a, INV_COS_BIT as i8);
        round_shift_array_16_neon(cur_a, txfm_size_col, -(*shift as i32));
        if lr_flip == 1 {
            for j in 0..buf_size_w_div8 {
                flip_buf_ud_neon(cur_a.offset((j * 8) as isize), 8);
                transpose_arrays_s16_8x8(
                    cur_a.offset((j * 8) as isize),
                    &raw mut b[(temp_b + txfm_size_row * (buf_size_w_div8 - 1 - j)) as usize],
                );
            }
            temp_b += 8;
        } else {
            for j in 0..buf_size_w_div8 {
                transpose_arrays_s16_8x8(
                    cur_a.offset((j * 8) as isize),
                    &raw mut b[(temp_b + txfm_size_row * j) as usize],
                );
            }
            temp_b += 8;
        }
    }
    for j in 0..buf_size_w_div8 {
        col_txfm.unwrap()(
            &raw mut b[(j * txfm_size_row) as usize],
            &raw mut b[(j * txfm_size_row) as usize],
            INV_COS_BIT as i8,
        );
        round_shift_array_16_neon(
            &raw mut b[(j * txfm_size_row) as usize],
            txfm_size_row,
            -(*shift.add(1) as i32),
        );
    }

    if txfm_size_col >= 16 {
        for i in 0..(txfm_size_col >> 4) {
            lowbd_add_flip_buffer_16xn_neon(
                &raw mut b[(i * txfm_size_row * 2) as usize],
                output.offset((16 * i) as isize),
                stride,
                ud_flip,
                txfm_size_row,
            );
        }
    } else if txfm_size_col == 8 {
        lowbd_add_flip_buffer_8xn_neon(b.as_mut_ptr(), output, stride, ud_flip, txfm_size_row);
    }
}

#[inline(always)]
unsafe fn lowbd_inv_txfm2d_add_universe_neon(
    mut input: *const i32,
    mut output: *mut u8,
    mut stride: i32,
    mut tx_type: TX_TYPE,
    mut tx_size: TX_SIZE,
    mut eob: i32,
) {
    match tx_type as i32 {
        IDTX => {
            lowbd_inv_txfm2d_add_idtx_neon(input, output, stride, tx_type, tx_size, eob);
        }

        H_DCT | H_ADST | H_FLIPADST => {
            lowbd_inv_txfm2d_add_v_identity_neon(input, output, stride, tx_type, tx_size, eob);
        }

        V_DCT | V_ADST | V_FLIPADST => {
            lowbd_inv_txfm2d_add_h_identity_neon(input, output, stride, tx_type, tx_size, eob);
        }

        _ => {
            lowbd_inv_txfm2d_add_no_identity_neon(input, output, stride, tx_type, tx_size, eob);
        }
    }
}

#[no_mangle]
pub unsafe extern "C-unwind" fn av1_lowbd_inv_txfm2d_add_neon(
    mut input: *const i32,
    mut output: *mut u8,
    mut stride: i32,
    mut tx_type: TX_TYPE,
    mut tx_size: TX_SIZE,
    mut eob: i32,
) {
    match tx_size as i32 {
        TX_4X4 => {
            lowbd_inv_txfm2d_add_4x4_neon(input, output, stride, tx_type, eob);
        }

        TX_4X8 => {
            lowbd_inv_txfm2d_add_4x8_neon(input, output, stride, tx_type, eob);
        }

        TX_8X4 => {
            lowbd_inv_txfm2d_add_8x4_neon(input, output, stride, tx_type, eob);
        }

        TX_4X16 => {
            lowbd_inv_txfm2d_add_4x16_neon(input, output, stride, tx_type, eob);
        }

        TX_16X4 => {
            lowbd_inv_txfm2d_add_16x4_neon(input, output, stride, tx_type, eob);
        }

        _ => {
            lowbd_inv_txfm2d_add_universe_neon(input, output, stride, tx_type, tx_size, eob);
        }
    }
}

#[no_mangle]
pub unsafe extern "C-unwind" fn av1_inv_txfm_add_neon(
    mut dqcoeff: *const tran_low_t,
    mut dst: *mut u8,
    mut stride: i32,
    mut txfm_param: *const TxfmParam,
) {
    if (*txfm_param).lossless == 0 {
        // The original's GCC 14 / pre-Armv7 C fallback does not apply to
        // AArch64.
        let tx_type: TX_TYPE = (*txfm_param).tx_type;
        av1_lowbd_inv_txfm2d_add_neon(
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
