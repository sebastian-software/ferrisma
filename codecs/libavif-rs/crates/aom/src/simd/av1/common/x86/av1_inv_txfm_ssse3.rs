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

// Translated by hand from upstream/aom/av1/common/x86/av1_inv_txfm_ssse3.h
// (ADR 0056, ADR 0058). Only the items that the high-bitdepth inverse
// transforms (highbd_inv_txfm_sse4.c, highbd_inv_txfm_avx2.c) use are
// translated: the 1D type tables, the eob tables, and the eob helpers. The
// btf_16_* macros, round_shift_16bit_ssse3, and the prototypes of
// av1_inv_txfm_ssse3.c belong to the low-bitdepth transforms and are left out.
//
// Translation notes:
// - The helpers use no intrinsics; they carry `ssse3`, the flag of the
//   header's own translation unit, and inline into SSE4.1 and AVX2 callers.
// - `int *` out-parameters stay raw pointers.
// - The DECLARE_ALIGNED(16, ...) alignment of the eob tables is not needed:
//   they are only read element by element. The pointer table is `static mut`
//   because it holds raw pointers, as in the Neon translation.
#![allow(non_upper_case_globals, dead_code)]

// From av1/common/enums.h.
type TX_SIZE = u8;

// From av1/common/common_data.h.
// Transform block width in pixels
static tx_size_wide: [i32; 19] =
    [4, 8, 16, 32, 64, 4, 8, 8, 16, 16, 32, 32, 64, 4, 16, 8, 32, 16, 64];
// Transform block height in pixels
static tx_size_high: [i32; 19] =
    [4, 8, 16, 32, 64, 8, 4, 16, 8, 32, 16, 64, 32, 16, 4, 32, 8, 64, 16];

// From aom_dsp/aom_dsp_common.h.
#[inline]
fn AOMMIN(x: i32, y: i32) -> i32 {
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
    IDCT_1D as ITX_TYPE_1D,      IADST_1D as ITX_TYPE_1D,     IDCT_1D as ITX_TYPE_1D,
    IADST_1D as ITX_TYPE_1D,     IFLIPADST_1D as ITX_TYPE_1D, IDCT_1D as ITX_TYPE_1D,
    IFLIPADST_1D as ITX_TYPE_1D, IADST_1D as ITX_TYPE_1D,     IFLIPADST_1D as ITX_TYPE_1D,
    IIDENTITY_1D as ITX_TYPE_1D, IDCT_1D as ITX_TYPE_1D,      IIDENTITY_1D as ITX_TYPE_1D,
    IADST_1D as ITX_TYPE_1D,     IIDENTITY_1D as ITX_TYPE_1D, IFLIPADST_1D as ITX_TYPE_1D,
    IIDENTITY_1D as ITX_TYPE_1D,
];

pub static hitx_1d_tab: [ITX_TYPE_1D; 16] = [
    IDCT_1D as ITX_TYPE_1D,      IDCT_1D as ITX_TYPE_1D,      IADST_1D as ITX_TYPE_1D,
    IADST_1D as ITX_TYPE_1D,     IDCT_1D as ITX_TYPE_1D,      IFLIPADST_1D as ITX_TYPE_1D,
    IFLIPADST_1D as ITX_TYPE_1D, IFLIPADST_1D as ITX_TYPE_1D, IADST_1D as ITX_TYPE_1D,
    IIDENTITY_1D as ITX_TYPE_1D, IIDENTITY_1D as ITX_TYPE_1D, IDCT_1D as ITX_TYPE_1D,
    IIDENTITY_1D as ITX_TYPE_1D, IADST_1D as ITX_TYPE_1D,     IIDENTITY_1D as ITX_TYPE_1D,
    IFLIPADST_1D as ITX_TYPE_1D,
];

static av1_eob_to_eobxy_8x8_default: [i16; 8] =
    [0x0707, 0x0707, 0x0707, 0x0707, 0x0707, 0x0707, 0x0707, 0x0707];

static av1_eob_to_eobxy_16x16_default: [i16; 16] = [
    0x0707, 0x0707, 0x0f0f, 0x0f0f, 0x0f0f, 0x0f0f, 0x0f0f, 0x0f0f,
    0x0f0f, 0x0f0f, 0x0f0f, 0x0f0f, 0x0f0f, 0x0f0f, 0x0f0f, 0x0f0f,
];

static av1_eob_to_eobxy_32x32_default: [i16; 32] = [
    0x0707, 0x0f0f, 0x0f0f, 0x0f0f, 0x1f1f, 0x1f1f, 0x1f1f, 0x1f1f,
    0x1f1f, 0x1f1f, 0x1f1f, 0x1f1f, 0x1f1f, 0x1f1f, 0x1f1f, 0x1f1f,
    0x1f1f, 0x1f1f, 0x1f1f, 0x1f1f, 0x1f1f, 0x1f1f, 0x1f1f, 0x1f1f,
    0x1f1f, 0x1f1f, 0x1f1f, 0x1f1f, 0x1f1f, 0x1f1f, 0x1f1f, 0x1f1f,
];

static av1_eob_to_eobxy_8x16_default: [i16; 16] = [
    0x0707, 0x0707, 0x0707, 0x0707, 0x0707, 0x0f07, 0x0f07, 0x0f07,
    0x0f07, 0x0f07, 0x0f07, 0x0f07, 0x0f07, 0x0f07, 0x0f07, 0x0f07,
];

static av1_eob_to_eobxy_16x8_default: [i16; 8] =
    [0x0707, 0x0707, 0x070f, 0x070f, 0x070f, 0x070f, 0x070f, 0x070f];

static av1_eob_to_eobxy_16x32_default: [i16; 32] = [
    0x0707, 0x0707, 0x0f0f, 0x0f0f, 0x0f0f, 0x0f0f, 0x0f0f, 0x0f0f,
    0x0f0f, 0x1f0f, 0x1f0f, 0x1f0f, 0x1f0f, 0x1f0f, 0x1f0f, 0x1f0f,
    0x1f0f, 0x1f0f, 0x1f0f, 0x1f0f, 0x1f0f, 0x1f0f, 0x1f0f, 0x1f0f,
    0x1f0f, 0x1f0f, 0x1f0f, 0x1f0f, 0x1f0f, 0x1f0f, 0x1f0f, 0x1f0f,
];

static av1_eob_to_eobxy_32x16_default: [i16; 16] = [
    0x0707, 0x0f0f, 0x0f0f, 0x0f0f, 0x0f1f, 0x0f1f, 0x0f1f, 0x0f1f,
    0x0f1f, 0x0f1f, 0x0f1f, 0x0f1f, 0x0f1f, 0x0f1f, 0x0f1f, 0x0f1f,
];

static av1_eob_to_eobxy_8x32_default: [i16; 32] = [
    0x0707, 0x0707, 0x0707, 0x0707, 0x0707, 0x0f07, 0x0f07, 0x0f07,
    0x0f07, 0x0f07, 0x0f07, 0x0f07, 0x0f07, 0x1f07, 0x1f07, 0x1f07,
    0x1f07, 0x1f07, 0x1f07, 0x1f07, 0x1f07, 0x1f07, 0x1f07, 0x1f07,
    0x1f07, 0x1f07, 0x1f07, 0x1f07, 0x1f07, 0x1f07, 0x1f07, 0x1f07,
];

static av1_eob_to_eobxy_32x8_default: [i16; 8] =
    [0x0707, 0x070f, 0x070f, 0x071f, 0x071f, 0x071f, 0x071f, 0x071f];

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
    0, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2,
    3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
];

// Transform block width in log2 for eob (size of 64 map to 32)
static tx_size_wide_log2_eob: [i32; 19] = [
    2, 3, 4, 5, 5, 2, 3, 3, 4, 4, 5, 5, 5, 2, 4, 3, 5, 4, 5,
];

#[inline]
#[target_feature(enable = "ssse3")]
pub unsafe fn get_eobx_eoby_scan_default(eobx: *mut i32, eoby: *mut i32, tx_size: TX_SIZE,
                                         eob: i32) {
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
    0,  7,  7,  7,  7,  7,  7,  7,  15, 15, 15, 15, 15, 15, 15, 15,
    31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31,
];

#[inline]
#[target_feature(enable = "ssse3")]
pub unsafe fn get_eobx_eoby_scan_h_identity(eobx: *mut i32, eoby: *mut i32, tx_size: TX_SIZE,
                                            mut eob: i32) {
    eob -= 1;
    let txfm_size_col: i32 = tx_size_wide[tx_size as usize];
    let eobx_max: i32 = AOMMIN(32, txfm_size_col) - 1;
    *eobx = if eob >= eobx_max { eobx_max } else { eob_fill[eob as usize] };
    let temp_eoby: i32 = eob / (eobx_max + 1);
    debug_assert!(temp_eoby < 32);
    *eoby = eob_fill[temp_eoby as usize];
}

#[inline]
#[target_feature(enable = "ssse3")]
pub unsafe fn get_eobx_eoby_scan_v_identity(eobx: *mut i32, eoby: *mut i32, tx_size: TX_SIZE,
                                            mut eob: i32) {
    eob -= 1;
    let txfm_size_row: i32 = tx_size_high[tx_size as usize];
    let eoby_max: i32 = AOMMIN(32, txfm_size_row) - 1;
    *eobx = eob_fill[(eob / (eoby_max + 1)) as usize];
    *eoby = if eob >= eoby_max { eoby_max } else { eob_fill[eob as usize] };
}
