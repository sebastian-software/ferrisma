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

// Mechanically translated from upstream/aom/av1/encoder/av1_fwd_txfm2d.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C-unwind" {
    static av1_txfm_type_ls: [[TXFM_TYPE; 4]; 5];
    static av1_txfm_stage_num_list: [int8_t; 12];
    static mut av1_round_shift_array: Option<unsafe extern "C-unwind" fn(*mut int32_t, core::ffi::c_int, core::ffi::c_int) -> ()>;
    fn av1_fdct4(
        input: *const int32_t,
        output: *mut int32_t,
        cos_bit: int8_t,
        stage_range: *const int8_t,
    );
    fn av1_fdct8(
        input: *const int32_t,
        output: *mut int32_t,
        cos_bit: int8_t,
        stage_range: *const int8_t,
    );
    fn av1_fdct16(
        input: *const int32_t,
        output: *mut int32_t,
        cos_bit: int8_t,
        stage_range: *const int8_t,
    );
    fn av1_fdct32(
        input: *const int32_t,
        output: *mut int32_t,
        cos_bit: int8_t,
        stage_range: *const int8_t,
    );
    fn av1_fdct64(
        input: *const int32_t,
        output: *mut int32_t,
        cos_bit: int8_t,
        stage_range: *const int8_t,
    );
    fn av1_fadst4(
        input: *const int32_t,
        output: *mut int32_t,
        cos_bit: int8_t,
        stage_range: *const int8_t,
    );
    fn av1_fadst8(
        input: *const int32_t,
        output: *mut int32_t,
        cos_bit: int8_t,
        stage_range: *const int8_t,
    );
    fn av1_fadst16(
        input: *const int32_t,
        output: *mut int32_t,
        cos_bit: int8_t,
        stage_range: *const int8_t,
    );
    fn av1_fidentity4_c(
        input: *const int32_t,
        output: *mut int32_t,
        cos_bit: int8_t,
        stage_range: *const int8_t,
    );
    fn av1_fidentity8_c(
        input: *const int32_t,
        output: *mut int32_t,
        cos_bit: int8_t,
        stage_range: *const int8_t,
    );
    fn av1_fidentity16_c(
        input: *const int32_t,
        output: *mut int32_t,
        cos_bit: int8_t,
        stage_range: *const int8_t,
    );
    fn av1_fidentity32_c(
        input: *const int32_t,
        output: *mut int32_t,
        cos_bit: int8_t,
        stage_range: *const int8_t,
    );
}
pub type size_t = usize;
pub type int8_t = i8;
pub type int16_t = i16;
pub type int32_t = i32;
pub type int64_t = i64;
pub type uint8_t = u8;
pub type C2RustUnnamed = core::ffi::c_uint;
pub const TX_INVALID: C2RustUnnamed = 255;
pub const TX_SIZES_LARGEST: C2RustUnnamed = 4;
pub const TX_SIZES: C2RustUnnamed = 5;
pub const TX_SIZES_ALL: C2RustUnnamed = 19;
pub const TX_64X16: C2RustUnnamed = 18;
pub const TX_16X64: C2RustUnnamed = 17;
pub const TX_32X8: C2RustUnnamed = 16;
pub const TX_8X32: C2RustUnnamed = 15;
pub const TX_16X4: C2RustUnnamed = 14;
pub const TX_4X16: C2RustUnnamed = 13;
pub const TX_64X32: C2RustUnnamed = 12;
pub const TX_32X64: C2RustUnnamed = 11;
pub const TX_32X16: C2RustUnnamed = 10;
pub const TX_16X32: C2RustUnnamed = 9;
pub const TX_16X8: C2RustUnnamed = 8;
pub const TX_8X16: C2RustUnnamed = 7;
pub const TX_8X4: C2RustUnnamed = 6;
pub const TX_4X8: C2RustUnnamed = 5;
pub const TX_64X64: C2RustUnnamed = 4;
pub const TX_32X32: C2RustUnnamed = 3;
pub const TX_16X16: C2RustUnnamed = 2;
pub const TX_8X8: C2RustUnnamed = 1;
pub const TX_4X4: C2RustUnnamed = 0;
pub type TX_SIZE = uint8_t;
pub type C2RustUnnamed_0 = core::ffi::c_uint;
pub const TX_TYPE_INVALID: C2RustUnnamed_0 = 255;
pub const DCT_ADST_TX_MASK: C2RustUnnamed_0 = 15;
pub const TX_TYPES: C2RustUnnamed_0 = 16;
pub const H_FLIPADST: C2RustUnnamed_0 = 15;
pub const V_FLIPADST: C2RustUnnamed_0 = 14;
pub const H_ADST: C2RustUnnamed_0 = 13;
pub const V_ADST: C2RustUnnamed_0 = 12;
pub const H_DCT: C2RustUnnamed_0 = 11;
pub const V_DCT: C2RustUnnamed_0 = 10;
pub const IDTX: C2RustUnnamed_0 = 9;
pub const FLIPADST_ADST: C2RustUnnamed_0 = 8;
pub const ADST_FLIPADST: C2RustUnnamed_0 = 7;
pub const FLIPADST_FLIPADST: C2RustUnnamed_0 = 6;
pub const DCT_FLIPADST: C2RustUnnamed_0 = 5;
pub const FLIPADST_DCT: C2RustUnnamed_0 = 4;
pub const ADST_ADST: C2RustUnnamed_0 = 3;
pub const DCT_ADST: C2RustUnnamed_0 = 2;
pub const ADST_DCT: C2RustUnnamed_0 = 1;
pub const DCT_DCT: C2RustUnnamed_0 = 0;
pub type TX_TYPE = uint8_t;
pub type C2RustUnnamed_1 = core::ffi::c_uint;
pub const TX_TYPES_1D: C2RustUnnamed_1 = 4;
pub const IDTX_1D: C2RustUnnamed_1 = 3;
pub const FLIPADST_1D: C2RustUnnamed_1 = 2;
pub const ADST_1D: C2RustUnnamed_1 = 1;
pub const DCT_1D: C2RustUnnamed_1 = 0;
pub type TX_TYPE_1D = uint8_t;
pub type TxfmFunc = Option<
    unsafe extern "C-unwind" fn(*const int32_t, *mut int32_t, int8_t, *const int8_t) -> (),
>;
pub type C2RustUnnamed_2 = core::ffi::c_uint;
pub const TXFM_TYPE_INVALID: C2RustUnnamed_2 = 13;
pub const TXFM_TYPES: C2RustUnnamed_2 = 12;
pub const TXFM_TYPE_IDENTITY32: C2RustUnnamed_2 = 11;
pub const TXFM_TYPE_IDENTITY16: C2RustUnnamed_2 = 10;
pub const TXFM_TYPE_IDENTITY8: C2RustUnnamed_2 = 9;
pub const TXFM_TYPE_IDENTITY4: C2RustUnnamed_2 = 8;
pub const TXFM_TYPE_ADST16: C2RustUnnamed_2 = 7;
pub const TXFM_TYPE_ADST8: C2RustUnnamed_2 = 6;
pub const TXFM_TYPE_ADST4: C2RustUnnamed_2 = 5;
pub const TXFM_TYPE_DCT64: C2RustUnnamed_2 = 4;
pub const TXFM_TYPE_DCT32: C2RustUnnamed_2 = 3;
pub const TXFM_TYPE_DCT16: C2RustUnnamed_2 = 2;
pub const TXFM_TYPE_DCT8: C2RustUnnamed_2 = 1;
pub const TXFM_TYPE_DCT4: C2RustUnnamed_2 = 0;
pub type TXFM_TYPE = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TXFM_2D_FLIP_CFG {
    pub tx_size: TX_SIZE,
    pub ud_flip: core::ffi::c_int,
    pub lr_flip: core::ffi::c_int,
    pub shift: *const int8_t,
    pub cos_bit_col: int8_t,
    pub cos_bit_row: int8_t,
    pub stage_range_col: [int8_t; 12],
    pub stage_range_row: [int8_t; 12],
    pub txfm_type_col: TXFM_TYPE,
    pub txfm_type_row: TXFM_TYPE,
    pub stage_num_col: core::ffi::c_int,
    pub stage_num_row: core::ffi::c_int,
}
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
static mut vtx_tab: [TX_TYPE_1D; 16] = [
    DCT_1D as core::ffi::c_int as TX_TYPE_1D,
    ADST_1D as core::ffi::c_int as TX_TYPE_1D,
    DCT_1D as core::ffi::c_int as TX_TYPE_1D,
    ADST_1D as core::ffi::c_int as TX_TYPE_1D,
    FLIPADST_1D as core::ffi::c_int as TX_TYPE_1D,
    DCT_1D as core::ffi::c_int as TX_TYPE_1D,
    FLIPADST_1D as core::ffi::c_int as TX_TYPE_1D,
    ADST_1D as core::ffi::c_int as TX_TYPE_1D,
    FLIPADST_1D as core::ffi::c_int as TX_TYPE_1D,
    IDTX_1D as core::ffi::c_int as TX_TYPE_1D,
    DCT_1D as core::ffi::c_int as TX_TYPE_1D,
    IDTX_1D as core::ffi::c_int as TX_TYPE_1D,
    ADST_1D as core::ffi::c_int as TX_TYPE_1D,
    IDTX_1D as core::ffi::c_int as TX_TYPE_1D,
    FLIPADST_1D as core::ffi::c_int as TX_TYPE_1D,
    IDTX_1D as core::ffi::c_int as TX_TYPE_1D,
];
static mut htx_tab: [TX_TYPE_1D; 16] = [
    DCT_1D as core::ffi::c_int as TX_TYPE_1D,
    DCT_1D as core::ffi::c_int as TX_TYPE_1D,
    ADST_1D as core::ffi::c_int as TX_TYPE_1D,
    ADST_1D as core::ffi::c_int as TX_TYPE_1D,
    DCT_1D as core::ffi::c_int as TX_TYPE_1D,
    FLIPADST_1D as core::ffi::c_int as TX_TYPE_1D,
    FLIPADST_1D as core::ffi::c_int as TX_TYPE_1D,
    FLIPADST_1D as core::ffi::c_int as TX_TYPE_1D,
    ADST_1D as core::ffi::c_int as TX_TYPE_1D,
    IDTX_1D as core::ffi::c_int as TX_TYPE_1D,
    IDTX_1D as core::ffi::c_int as TX_TYPE_1D,
    DCT_1D as core::ffi::c_int as TX_TYPE_1D,
    IDTX_1D as core::ffi::c_int as TX_TYPE_1D,
    ADST_1D as core::ffi::c_int as TX_TYPE_1D,
    IDTX_1D as core::ffi::c_int as TX_TYPE_1D,
    FLIPADST_1D as core::ffi::c_int as TX_TYPE_1D,
];
static mut tx_size_wide: [core::ffi::c_int; 19] = [
    4 as core::ffi::c_int,
    8 as core::ffi::c_int,
    16 as core::ffi::c_int,
    32 as core::ffi::c_int,
    64 as core::ffi::c_int,
    4 as core::ffi::c_int,
    8 as core::ffi::c_int,
    8 as core::ffi::c_int,
    16 as core::ffi::c_int,
    16 as core::ffi::c_int,
    32 as core::ffi::c_int,
    32 as core::ffi::c_int,
    64 as core::ffi::c_int,
    4 as core::ffi::c_int,
    16 as core::ffi::c_int,
    8 as core::ffi::c_int,
    32 as core::ffi::c_int,
    16 as core::ffi::c_int,
    64 as core::ffi::c_int,
];
static mut tx_size_high: [core::ffi::c_int; 19] = [
    4 as core::ffi::c_int,
    8 as core::ffi::c_int,
    16 as core::ffi::c_int,
    32 as core::ffi::c_int,
    64 as core::ffi::c_int,
    8 as core::ffi::c_int,
    4 as core::ffi::c_int,
    16 as core::ffi::c_int,
    8 as core::ffi::c_int,
    32 as core::ffi::c_int,
    16 as core::ffi::c_int,
    64 as core::ffi::c_int,
    32 as core::ffi::c_int,
    16 as core::ffi::c_int,
    4 as core::ffi::c_int,
    32 as core::ffi::c_int,
    8 as core::ffi::c_int,
    64 as core::ffi::c_int,
    16 as core::ffi::c_int,
];
static mut tx_size_wide_log2: [core::ffi::c_int; 19] = [
    2 as core::ffi::c_int,
    3 as core::ffi::c_int,
    4 as core::ffi::c_int,
    5 as core::ffi::c_int,
    6 as core::ffi::c_int,
    2 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    5 as core::ffi::c_int,
    5 as core::ffi::c_int,
    6 as core::ffi::c_int,
    2 as core::ffi::c_int,
    4 as core::ffi::c_int,
    3 as core::ffi::c_int,
    5 as core::ffi::c_int,
    4 as core::ffi::c_int,
    6 as core::ffi::c_int,
];
static mut tx_size_high_log2: [core::ffi::c_int; 19] = [
    2 as core::ffi::c_int,
    3 as core::ffi::c_int,
    4 as core::ffi::c_int,
    5 as core::ffi::c_int,
    6 as core::ffi::c_int,
    3 as core::ffi::c_int,
    2 as core::ffi::c_int,
    4 as core::ffi::c_int,
    3 as core::ffi::c_int,
    5 as core::ffi::c_int,
    4 as core::ffi::c_int,
    6 as core::ffi::c_int,
    5 as core::ffi::c_int,
    4 as core::ffi::c_int,
    2 as core::ffi::c_int,
    5 as core::ffi::c_int,
    3 as core::ffi::c_int,
    6 as core::ffi::c_int,
    4 as core::ffi::c_int,
];
pub const MAX_TXFM_STAGE_NUM: core::ffi::c_int = 12 as core::ffi::c_int;
pub const NewSqrt2Bits: int32_t = 12 as core::ffi::c_int as int32_t;
static mut NewSqrt2: int32_t = 5793 as int32_t;
#[inline]
unsafe extern "C-unwind" fn round_shift(
    mut value: int64_t,
    mut bit: core::ffi::c_int,
) -> int32_t {
    return (value + ((1 as int64_t) << bit - 1 as core::ffi::c_int) >> bit) as int32_t;
}
#[inline]
unsafe extern "C-unwind" fn get_flip_cfg(
    mut tx_type: TX_TYPE,
    mut ud_flip: *mut core::ffi::c_int,
    mut lr_flip: *mut core::ffi::c_int,
) {
    match tx_type as core::ffi::c_int {
        0 | 1 | 2 | 3 => {
            *ud_flip = 0 as core::ffi::c_int;
            *lr_flip = 0 as core::ffi::c_int;
        }
        9 | 10 | 11 | 12 | 13 => {
            *ud_flip = 0 as core::ffi::c_int;
            *lr_flip = 0 as core::ffi::c_int;
        }
        4 | 8 | 14 => {
            *ud_flip = 1 as core::ffi::c_int;
            *lr_flip = 0 as core::ffi::c_int;
        }
        5 | 7 | 15 => {
            *ud_flip = 0 as core::ffi::c_int;
            *lr_flip = 1 as core::ffi::c_int;
        }
        6 => {
            *ud_flip = 1 as core::ffi::c_int;
            *lr_flip = 1 as core::ffi::c_int;
        }
        _ => {
            *ud_flip = 0 as core::ffi::c_int;
            *lr_flip = 0 as core::ffi::c_int;
        }
    };
}
#[inline]
unsafe extern "C-unwind" fn set_flip_cfg(mut tx_type: TX_TYPE, mut cfg: *mut TXFM_2D_FLIP_CFG) {
    get_flip_cfg(tx_type, &raw mut (*cfg).ud_flip, &raw mut (*cfg).lr_flip);
}
#[inline]
unsafe extern "C-unwind" fn get_rect_tx_log_ratio(
    mut col: core::ffi::c_int,
    mut row: core::ffi::c_int,
) -> core::ffi::c_int {
    if col == row {
        return 0 as core::ffi::c_int;
    }
    if col > row {
        if col == row * 2 as core::ffi::c_int {
            return 1 as core::ffi::c_int;
        }
        if col == row * 4 as core::ffi::c_int {
            return 2 as core::ffi::c_int;
        }
    } else {
        if row == col * 2 as core::ffi::c_int {
            return -(1 as core::ffi::c_int);
        }
        if row == col * 4 as core::ffi::c_int {
            return -(2 as core::ffi::c_int);
        }
    }
    return 0 as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn get_txw_idx(mut tx_size: TX_SIZE) -> core::ffi::c_int {
    return tx_size_wide_log2[tx_size as usize]
        - tx_size_wide_log2[0 as core::ffi::c_int as usize];
}
#[inline]
unsafe extern "C-unwind" fn get_txh_idx(mut tx_size: TX_SIZE) -> core::ffi::c_int {
    return tx_size_high_log2[tx_size as usize]
        - tx_size_high_log2[0 as core::ffi::c_int as usize];
}
#[inline]
unsafe extern "C-unwind" fn fwd_txfm_type_to_func(mut txfm_type: TXFM_TYPE) -> TxfmFunc {
    match txfm_type as core::ffi::c_int {
        0 => {
            return Some(
                av1_fdct4
                    as unsafe extern "C-unwind" fn(
                        *const int32_t,
                        *mut int32_t,
                        int8_t,
                        *const int8_t,
                    ) -> (),
            );
        }
        1 => {
            return Some(
                av1_fdct8
                    as unsafe extern "C-unwind" fn(
                        *const int32_t,
                        *mut int32_t,
                        int8_t,
                        *const int8_t,
                    ) -> (),
            );
        }
        2 => {
            return Some(
                av1_fdct16
                    as unsafe extern "C-unwind" fn(
                        *const int32_t,
                        *mut int32_t,
                        int8_t,
                        *const int8_t,
                    ) -> (),
            );
        }
        3 => {
            return Some(
                av1_fdct32
                    as unsafe extern "C-unwind" fn(
                        *const int32_t,
                        *mut int32_t,
                        int8_t,
                        *const int8_t,
                    ) -> (),
            );
        }
        4 => {
            return Some(
                av1_fdct64
                    as unsafe extern "C-unwind" fn(
                        *const int32_t,
                        *mut int32_t,
                        int8_t,
                        *const int8_t,
                    ) -> (),
            );
        }
        5 => {
            return Some(
                av1_fadst4
                    as unsafe extern "C-unwind" fn(
                        *const int32_t,
                        *mut int32_t,
                        int8_t,
                        *const int8_t,
                    ) -> (),
            );
        }
        6 => {
            return Some(
                av1_fadst8
                    as unsafe extern "C-unwind" fn(
                        *const int32_t,
                        *mut int32_t,
                        int8_t,
                        *const int8_t,
                    ) -> (),
            );
        }
        7 => {
            return Some(
                av1_fadst16
                    as unsafe extern "C-unwind" fn(
                        *const int32_t,
                        *mut int32_t,
                        int8_t,
                        *const int8_t,
                    ) -> (),
            );
        }
        8 => {
            return Some(
                av1_fidentity4_c
                    as unsafe extern "C-unwind" fn(
                        *const int32_t,
                        *mut int32_t,
                        int8_t,
                        *const int8_t,
                    ) -> (),
            );
        }
        9 => {
            return Some(
                av1_fidentity8_c
                    as unsafe extern "C-unwind" fn(
                        *const int32_t,
                        *mut int32_t,
                        int8_t,
                        *const int8_t,
                    ) -> (),
            );
        }
        10 => {
            return Some(
                av1_fidentity16_c
                    as unsafe extern "C-unwind" fn(
                        *const int32_t,
                        *mut int32_t,
                        int8_t,
                        *const int8_t,
                    ) -> (),
            );
        }
        11 => {
            return Some(
                av1_fidentity32_c
                    as unsafe extern "C-unwind" fn(
                        *const int32_t,
                        *mut int32_t,
                        int8_t,
                        *const int8_t,
                    ) -> (),
            );
        }
        _ => return None,
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_gen_fwd_stage_range(
    mut stage_range_col: *mut int8_t,
    mut stage_range_row: *mut int8_t,
    mut cfg: *const TXFM_2D_FLIP_CFG,
    mut bd: core::ffi::c_int,
) {
    let mut shift: *const int8_t = (*cfg).shift;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < (*cfg).stage_num_col && i < MAX_TXFM_STAGE_NUM {
        *stage_range_col.offset(i as isize) = ((*cfg).stage_range_col[i as usize]
            as core::ffi::c_int
            + *shift.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int + bd
            + 1 as core::ffi::c_int) as int8_t;
        i += 1;
    }
    let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while i_0 < (*cfg).stage_num_row && i_0 < MAX_TXFM_STAGE_NUM {
        *stage_range_row.offset(i_0 as isize) = ((*cfg).stage_range_row[i_0 as usize]
            as core::ffi::c_int
            + *shift.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
            + *shift.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int + bd
            + 1 as core::ffi::c_int) as int8_t;
        i_0 += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn fwd_txfm2d_c(
    mut input: *const int16_t,
    mut output: *mut int32_t,
    stride: core::ffi::c_int,
    mut cfg: *const TXFM_2D_FLIP_CFG,
    mut buf: *mut int32_t,
    mut bd: core::ffi::c_int,
) {
    let mut c: core::ffi::c_int = 0;
    let mut r: core::ffi::c_int = 0;
    let txfm_size_col: core::ffi::c_int = tx_size_wide[(*cfg).tx_size as usize];
    let txfm_size_row: core::ffi::c_int = tx_size_high[(*cfg).tx_size as usize];
    let mut shift: *const int8_t = (*cfg).shift;
    let rect_type: core::ffi::c_int = get_rect_tx_log_ratio(txfm_size_col, txfm_size_row)
        as core::ffi::c_int;
    let mut stage_range_col: [int8_t; 12] = [0; 12];
    let mut stage_range_row: [int8_t; 12] = [0; 12];
    av1_gen_fwd_stage_range(
        stage_range_col.as_mut_ptr(),
        stage_range_row.as_mut_ptr(),
        cfg,
        bd,
    );
    let cos_bit_col: int8_t = (*cfg).cos_bit_col;
    let cos_bit_row: int8_t = (*cfg).cos_bit_row;
    let txfm_func_col: TxfmFunc = fwd_txfm_type_to_func((*cfg).txfm_type_col)
        as TxfmFunc;
    let txfm_func_row: TxfmFunc = fwd_txfm_type_to_func((*cfg).txfm_type_row)
        as TxfmFunc;
    let mut temp_in: *mut int32_t = output;
    let mut temp_out: *mut int32_t = output.offset(txfm_size_row as isize);
    c = 0 as core::ffi::c_int;
    while c < txfm_size_col {
        if (*cfg).ud_flip == 0 as core::ffi::c_int {
            r = 0 as core::ffi::c_int;
            while r < txfm_size_row {
                *temp_in.offset(r as isize) = *input.offset((r * stride + c) as isize)
                    as int32_t;
                r += 1;
            }
        } else {
            r = 0 as core::ffi::c_int;
            while r < txfm_size_row {
                *temp_in.offset(r as isize) = *input
                    .offset(
                        ((txfm_size_row - r - 1 as core::ffi::c_int) * stride + c)
                            as isize,
                    ) as int32_t;
                r += 1;
            }
        }
        av1_round_shift_array.expect("non-null function pointer")(
            temp_in,
            txfm_size_row,
            -(*shift.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int),
        );
        txfm_func_col
            .expect(
                "non-null function pointer",
            )(temp_in, temp_out, cos_bit_col, stage_range_col.as_mut_ptr());
        av1_round_shift_array.expect("non-null function pointer")(
            temp_out,
            txfm_size_row,
            -(*shift.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int),
        );
        if (*cfg).lr_flip == 0 as core::ffi::c_int {
            r = 0 as core::ffi::c_int;
            while r < txfm_size_row {
                *buf.offset((r * txfm_size_col + c) as isize) = *temp_out
                    .offset(r as isize);
                r += 1;
            }
        } else {
            r = 0 as core::ffi::c_int;
            while r < txfm_size_row {
                *buf
                    .offset(
                        (r * txfm_size_col + (txfm_size_col - c - 1 as core::ffi::c_int))
                            as isize,
                    ) = *temp_out.offset(r as isize);
                r += 1;
            }
        }
        c += 1;
    }
    let mut row_buffer: crate::platform::Aligned16<[int32_t; 64]> = crate::platform::Aligned16([0; 64]);
    r = 0 as core::ffi::c_int;
    while r < txfm_size_row {
        txfm_func_row
            .expect(
                "non-null function pointer",
            )(
            buf.offset((r * txfm_size_col) as isize),
            row_buffer.0.as_mut_ptr(),
            cos_bit_row,
            stage_range_row.as_mut_ptr(),
        );
        av1_round_shift_array.expect("non-null function pointer")(
            row_buffer.0.as_mut_ptr(),
            txfm_size_col,
            -(*shift.offset(2 as core::ffi::c_int as isize) as core::ffi::c_int),
        );
        if abs(rect_type) == 1 as core::ffi::c_int {
            c = 0 as core::ffi::c_int;
            while c < txfm_size_col {
                row_buffer.0[c as usize] = round_shift(
                    row_buffer.0[c as usize] as int64_t * NewSqrt2 as int64_t,
                    NewSqrt2Bits as core::ffi::c_int,
                );
                c += 1;
            }
        }
        c = 0 as core::ffi::c_int;
        while c < txfm_size_col {
            *output.offset((c * txfm_size_row + r) as isize) = row_buffer.0[c as usize];
            c += 1;
        }
        r += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_fwd_txfm2d_4x8_c(
    mut input: *const int16_t,
    mut output: *mut int32_t,
    mut stride: core::ffi::c_int,
    mut tx_type: TX_TYPE,
    mut bd: core::ffi::c_int,
) {
    let mut txfm_buf: crate::platform::Aligned32<[int32_t; 32]> = crate::platform::Aligned32([0; 32]);
    let mut cfg: TXFM_2D_FLIP_CFG = TXFM_2D_FLIP_CFG {
        tx_size: 0,
        ud_flip: 0,
        lr_flip: 0,
        shift: 0 as *const int8_t,
        cos_bit_col: 0,
        cos_bit_row: 0,
        stage_range_col: [0; 12],
        stage_range_row: [0; 12],
        txfm_type_col: 0,
        txfm_type_row: 0,
        stage_num_col: 0,
        stage_num_row: 0,
    };
    av1_get_fwd_txfm_cfg(tx_type, TX_4X8 as core::ffi::c_int as TX_SIZE, &raw mut cfg);
    fwd_txfm2d_c(input, output, stride, &raw mut cfg, txfm_buf.0.as_mut_ptr(), bd);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_fwd_txfm2d_8x4_c(
    mut input: *const int16_t,
    mut output: *mut int32_t,
    mut stride: core::ffi::c_int,
    mut tx_type: TX_TYPE,
    mut bd: core::ffi::c_int,
) {
    let mut txfm_buf: [int32_t; 32] = [0; 32];
    let mut cfg: TXFM_2D_FLIP_CFG = TXFM_2D_FLIP_CFG {
        tx_size: 0,
        ud_flip: 0,
        lr_flip: 0,
        shift: 0 as *const int8_t,
        cos_bit_col: 0,
        cos_bit_row: 0,
        stage_range_col: [0; 12],
        stage_range_row: [0; 12],
        txfm_type_col: 0,
        txfm_type_row: 0,
        stage_num_col: 0,
        stage_num_row: 0,
    };
    av1_get_fwd_txfm_cfg(tx_type, TX_8X4 as core::ffi::c_int as TX_SIZE, &raw mut cfg);
    fwd_txfm2d_c(input, output, stride, &raw mut cfg, txfm_buf.as_mut_ptr(), bd);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_fwd_txfm2d_8x16_c(
    mut input: *const int16_t,
    mut output: *mut int32_t,
    mut stride: core::ffi::c_int,
    mut tx_type: TX_TYPE,
    mut bd: core::ffi::c_int,
) {
    let mut txfm_buf: crate::platform::Aligned32<[int32_t; 128]> = crate::platform::Aligned32([0; 128]);
    let mut cfg: TXFM_2D_FLIP_CFG = TXFM_2D_FLIP_CFG {
        tx_size: 0,
        ud_flip: 0,
        lr_flip: 0,
        shift: 0 as *const int8_t,
        cos_bit_col: 0,
        cos_bit_row: 0,
        stage_range_col: [0; 12],
        stage_range_row: [0; 12],
        txfm_type_col: 0,
        txfm_type_row: 0,
        stage_num_col: 0,
        stage_num_row: 0,
    };
    av1_get_fwd_txfm_cfg(tx_type, TX_8X16 as core::ffi::c_int as TX_SIZE, &raw mut cfg);
    fwd_txfm2d_c(input, output, stride, &raw mut cfg, txfm_buf.0.as_mut_ptr(), bd);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_fwd_txfm2d_16x8_c(
    mut input: *const int16_t,
    mut output: *mut int32_t,
    mut stride: core::ffi::c_int,
    mut tx_type: TX_TYPE,
    mut bd: core::ffi::c_int,
) {
    let mut txfm_buf: [int32_t; 128] = [0; 128];
    let mut cfg: TXFM_2D_FLIP_CFG = TXFM_2D_FLIP_CFG {
        tx_size: 0,
        ud_flip: 0,
        lr_flip: 0,
        shift: 0 as *const int8_t,
        cos_bit_col: 0,
        cos_bit_row: 0,
        stage_range_col: [0; 12],
        stage_range_row: [0; 12],
        txfm_type_col: 0,
        txfm_type_row: 0,
        stage_num_col: 0,
        stage_num_row: 0,
    };
    av1_get_fwd_txfm_cfg(tx_type, TX_16X8 as core::ffi::c_int as TX_SIZE, &raw mut cfg);
    fwd_txfm2d_c(input, output, stride, &raw mut cfg, txfm_buf.as_mut_ptr(), bd);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_fwd_txfm2d_16x32_c(
    mut input: *const int16_t,
    mut output: *mut int32_t,
    mut stride: core::ffi::c_int,
    mut tx_type: TX_TYPE,
    mut bd: core::ffi::c_int,
) {
    let mut txfm_buf: crate::platform::Aligned32<[int32_t; 512]> = crate::platform::Aligned32([0; 512]);
    let mut cfg: TXFM_2D_FLIP_CFG = TXFM_2D_FLIP_CFG {
        tx_size: 0,
        ud_flip: 0,
        lr_flip: 0,
        shift: 0 as *const int8_t,
        cos_bit_col: 0,
        cos_bit_row: 0,
        stage_range_col: [0; 12],
        stage_range_row: [0; 12],
        txfm_type_col: 0,
        txfm_type_row: 0,
        stage_num_col: 0,
        stage_num_row: 0,
    };
    av1_get_fwd_txfm_cfg(tx_type, TX_16X32 as core::ffi::c_int as TX_SIZE, &raw mut cfg);
    fwd_txfm2d_c(input, output, stride, &raw mut cfg, txfm_buf.0.as_mut_ptr(), bd);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_fwd_txfm2d_32x16_c(
    mut input: *const int16_t,
    mut output: *mut int32_t,
    mut stride: core::ffi::c_int,
    mut tx_type: TX_TYPE,
    mut bd: core::ffi::c_int,
) {
    let mut txfm_buf: [int32_t; 512] = [0; 512];
    let mut cfg: TXFM_2D_FLIP_CFG = TXFM_2D_FLIP_CFG {
        tx_size: 0,
        ud_flip: 0,
        lr_flip: 0,
        shift: 0 as *const int8_t,
        cos_bit_col: 0,
        cos_bit_row: 0,
        stage_range_col: [0; 12],
        stage_range_row: [0; 12],
        txfm_type_col: 0,
        txfm_type_row: 0,
        stage_num_col: 0,
        stage_num_row: 0,
    };
    av1_get_fwd_txfm_cfg(tx_type, TX_32X16 as core::ffi::c_int as TX_SIZE, &raw mut cfg);
    fwd_txfm2d_c(input, output, stride, &raw mut cfg, txfm_buf.as_mut_ptr(), bd);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_fwd_txfm2d_4x16_c(
    mut input: *const int16_t,
    mut output: *mut int32_t,
    mut stride: core::ffi::c_int,
    mut tx_type: TX_TYPE,
    mut bd: core::ffi::c_int,
) {
    let mut txfm_buf: crate::platform::Aligned32<[int32_t; 64]> = crate::platform::Aligned32([0; 64]);
    let mut cfg: TXFM_2D_FLIP_CFG = TXFM_2D_FLIP_CFG {
        tx_size: 0,
        ud_flip: 0,
        lr_flip: 0,
        shift: 0 as *const int8_t,
        cos_bit_col: 0,
        cos_bit_row: 0,
        stage_range_col: [0; 12],
        stage_range_row: [0; 12],
        txfm_type_col: 0,
        txfm_type_row: 0,
        stage_num_col: 0,
        stage_num_row: 0,
    };
    av1_get_fwd_txfm_cfg(tx_type, TX_4X16 as core::ffi::c_int as TX_SIZE, &raw mut cfg);
    fwd_txfm2d_c(input, output, stride, &raw mut cfg, txfm_buf.0.as_mut_ptr(), bd);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_fwd_txfm2d_16x4_c(
    mut input: *const int16_t,
    mut output: *mut int32_t,
    mut stride: core::ffi::c_int,
    mut tx_type: TX_TYPE,
    mut bd: core::ffi::c_int,
) {
    let mut txfm_buf: [int32_t; 64] = [0; 64];
    let mut cfg: TXFM_2D_FLIP_CFG = TXFM_2D_FLIP_CFG {
        tx_size: 0,
        ud_flip: 0,
        lr_flip: 0,
        shift: 0 as *const int8_t,
        cos_bit_col: 0,
        cos_bit_row: 0,
        stage_range_col: [0; 12],
        stage_range_row: [0; 12],
        txfm_type_col: 0,
        txfm_type_row: 0,
        stage_num_col: 0,
        stage_num_row: 0,
    };
    av1_get_fwd_txfm_cfg(tx_type, TX_16X4 as core::ffi::c_int as TX_SIZE, &raw mut cfg);
    fwd_txfm2d_c(input, output, stride, &raw mut cfg, txfm_buf.as_mut_ptr(), bd);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_fwd_txfm2d_8x32_c(
    mut input: *const int16_t,
    mut output: *mut int32_t,
    mut stride: core::ffi::c_int,
    mut tx_type: TX_TYPE,
    mut bd: core::ffi::c_int,
) {
    let mut txfm_buf: crate::platform::Aligned32<[int32_t; 256]> = crate::platform::Aligned32([0; 256]);
    let mut cfg: TXFM_2D_FLIP_CFG = TXFM_2D_FLIP_CFG {
        tx_size: 0,
        ud_flip: 0,
        lr_flip: 0,
        shift: 0 as *const int8_t,
        cos_bit_col: 0,
        cos_bit_row: 0,
        stage_range_col: [0; 12],
        stage_range_row: [0; 12],
        txfm_type_col: 0,
        txfm_type_row: 0,
        stage_num_col: 0,
        stage_num_row: 0,
    };
    av1_get_fwd_txfm_cfg(tx_type, TX_8X32 as core::ffi::c_int as TX_SIZE, &raw mut cfg);
    fwd_txfm2d_c(input, output, stride, &raw mut cfg, txfm_buf.0.as_mut_ptr(), bd);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_fwd_txfm2d_32x8_c(
    mut input: *const int16_t,
    mut output: *mut int32_t,
    mut stride: core::ffi::c_int,
    mut tx_type: TX_TYPE,
    mut bd: core::ffi::c_int,
) {
    let mut txfm_buf: [int32_t; 256] = [0; 256];
    let mut cfg: TXFM_2D_FLIP_CFG = TXFM_2D_FLIP_CFG {
        tx_size: 0,
        ud_flip: 0,
        lr_flip: 0,
        shift: 0 as *const int8_t,
        cos_bit_col: 0,
        cos_bit_row: 0,
        stage_range_col: [0; 12],
        stage_range_row: [0; 12],
        txfm_type_col: 0,
        txfm_type_row: 0,
        stage_num_col: 0,
        stage_num_row: 0,
    };
    av1_get_fwd_txfm_cfg(tx_type, TX_32X8 as core::ffi::c_int as TX_SIZE, &raw mut cfg);
    fwd_txfm2d_c(input, output, stride, &raw mut cfg, txfm_buf.as_mut_ptr(), bd);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_fwd_txfm2d_4x4_c(
    mut input: *const int16_t,
    mut output: *mut int32_t,
    mut stride: core::ffi::c_int,
    mut tx_type: TX_TYPE,
    mut bd: core::ffi::c_int,
) {
    let mut txfm_buf: [int32_t; 16] = [0; 16];
    let mut cfg: TXFM_2D_FLIP_CFG = TXFM_2D_FLIP_CFG {
        tx_size: 0,
        ud_flip: 0,
        lr_flip: 0,
        shift: 0 as *const int8_t,
        cos_bit_col: 0,
        cos_bit_row: 0,
        stage_range_col: [0; 12],
        stage_range_row: [0; 12],
        txfm_type_col: 0,
        txfm_type_row: 0,
        stage_num_col: 0,
        stage_num_row: 0,
    };
    av1_get_fwd_txfm_cfg(tx_type, TX_4X4 as core::ffi::c_int as TX_SIZE, &raw mut cfg);
    fwd_txfm2d_c(input, output, stride, &raw mut cfg, txfm_buf.as_mut_ptr(), bd);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_fwd_txfm2d_8x8_c(
    mut input: *const int16_t,
    mut output: *mut int32_t,
    mut stride: core::ffi::c_int,
    mut tx_type: TX_TYPE,
    mut bd: core::ffi::c_int,
) {
    let mut txfm_buf: [int32_t; 64] = [0; 64];
    let mut cfg: TXFM_2D_FLIP_CFG = TXFM_2D_FLIP_CFG {
        tx_size: 0,
        ud_flip: 0,
        lr_flip: 0,
        shift: 0 as *const int8_t,
        cos_bit_col: 0,
        cos_bit_row: 0,
        stage_range_col: [0; 12],
        stage_range_row: [0; 12],
        txfm_type_col: 0,
        txfm_type_row: 0,
        stage_num_col: 0,
        stage_num_row: 0,
    };
    av1_get_fwd_txfm_cfg(tx_type, TX_8X8 as core::ffi::c_int as TX_SIZE, &raw mut cfg);
    fwd_txfm2d_c(input, output, stride, &raw mut cfg, txfm_buf.as_mut_ptr(), bd);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_fwd_txfm2d_16x16_c(
    mut input: *const int16_t,
    mut output: *mut int32_t,
    mut stride: core::ffi::c_int,
    mut tx_type: TX_TYPE,
    mut bd: core::ffi::c_int,
) {
    let mut txfm_buf: [int32_t; 256] = [0; 256];
    let mut cfg: TXFM_2D_FLIP_CFG = TXFM_2D_FLIP_CFG {
        tx_size: 0,
        ud_flip: 0,
        lr_flip: 0,
        shift: 0 as *const int8_t,
        cos_bit_col: 0,
        cos_bit_row: 0,
        stage_range_col: [0; 12],
        stage_range_row: [0; 12],
        txfm_type_col: 0,
        txfm_type_row: 0,
        stage_num_col: 0,
        stage_num_row: 0,
    };
    av1_get_fwd_txfm_cfg(tx_type, TX_16X16 as core::ffi::c_int as TX_SIZE, &raw mut cfg);
    fwd_txfm2d_c(input, output, stride, &raw mut cfg, txfm_buf.as_mut_ptr(), bd);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_fwd_txfm2d_32x32_c(
    mut input: *const int16_t,
    mut output: *mut int32_t,
    mut stride: core::ffi::c_int,
    mut tx_type: TX_TYPE,
    mut bd: core::ffi::c_int,
) {
    let mut txfm_buf: [int32_t; 1024] = [0; 1024];
    let mut cfg: TXFM_2D_FLIP_CFG = TXFM_2D_FLIP_CFG {
        tx_size: 0,
        ud_flip: 0,
        lr_flip: 0,
        shift: 0 as *const int8_t,
        cos_bit_col: 0,
        cos_bit_row: 0,
        stage_range_col: [0; 12],
        stage_range_row: [0; 12],
        txfm_type_col: 0,
        txfm_type_row: 0,
        stage_num_col: 0,
        stage_num_row: 0,
    };
    av1_get_fwd_txfm_cfg(tx_type, TX_32X32 as core::ffi::c_int as TX_SIZE, &raw mut cfg);
    fwd_txfm2d_c(input, output, stride, &raw mut cfg, txfm_buf.as_mut_ptr(), bd);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_fwd_txfm2d_64x64_c(
    mut input: *const int16_t,
    mut output: *mut int32_t,
    mut stride: core::ffi::c_int,
    mut tx_type: TX_TYPE,
    mut bd: core::ffi::c_int,
) {
    let mut txfm_buf: [int32_t; 4096] = [0; 4096];
    let mut cfg: TXFM_2D_FLIP_CFG = TXFM_2D_FLIP_CFG {
        tx_size: 0,
        ud_flip: 0,
        lr_flip: 0,
        shift: 0 as *const int8_t,
        cos_bit_col: 0,
        cos_bit_row: 0,
        stage_range_col: [0; 12],
        stage_range_row: [0; 12],
        txfm_type_col: 0,
        txfm_type_row: 0,
        stage_num_col: 0,
        stage_num_row: 0,
    };
    av1_get_fwd_txfm_cfg(tx_type, TX_64X64 as core::ffi::c_int as TX_SIZE, &raw mut cfg);
    fwd_txfm2d_c(input, output, stride, &raw mut cfg, txfm_buf.as_mut_ptr(), bd);
    let mut col: core::ffi::c_int = 0 as core::ffi::c_int;
    while col < 32 as core::ffi::c_int {
        memset(
            output
                .offset((col * 64 as core::ffi::c_int) as isize)
                .offset(32 as core::ffi::c_int as isize) as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (32 as size_t).wrapping_mul(::core::mem::size_of::<int32_t>() as size_t),
        );
        col += 1;
    }
    memset(
        output.offset((32 as core::ffi::c_int * 64 as core::ffi::c_int) as isize)
            as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ((32 as core::ffi::c_int * 64 as core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<int32_t>() as size_t),
    );
    let mut col_0: core::ffi::c_int = 1 as core::ffi::c_int;
    while col_0 < 32 as core::ffi::c_int {
        memcpy(
            output.offset((col_0 * 32 as core::ffi::c_int) as isize)
                as *mut core::ffi::c_void,
            output.offset((col_0 * 64 as core::ffi::c_int) as isize)
                as *const core::ffi::c_void,
            (32 as size_t).wrapping_mul(::core::mem::size_of::<int32_t>() as size_t),
        );
        col_0 += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_fwd_txfm2d_32x64_c(
    mut input: *const int16_t,
    mut output: *mut int32_t,
    mut stride: core::ffi::c_int,
    mut tx_type: TX_TYPE,
    mut bd: core::ffi::c_int,
) {
    let mut txfm_buf: crate::platform::Aligned32<[int32_t; 2048]> = crate::platform::Aligned32([0; 2048]);
    let mut cfg: TXFM_2D_FLIP_CFG = TXFM_2D_FLIP_CFG {
        tx_size: 0,
        ud_flip: 0,
        lr_flip: 0,
        shift: 0 as *const int8_t,
        cos_bit_col: 0,
        cos_bit_row: 0,
        stage_range_col: [0; 12],
        stage_range_row: [0; 12],
        txfm_type_col: 0,
        txfm_type_row: 0,
        stage_num_col: 0,
        stage_num_row: 0,
    };
    av1_get_fwd_txfm_cfg(tx_type, TX_32X64 as core::ffi::c_int as TX_SIZE, &raw mut cfg);
    fwd_txfm2d_c(input, output, stride, &raw mut cfg, txfm_buf.0.as_mut_ptr(), bd);
    let mut col: core::ffi::c_int = 0 as core::ffi::c_int;
    while col < 32 as core::ffi::c_int {
        memset(
            output
                .offset((col * 64 as core::ffi::c_int) as isize)
                .offset(32 as core::ffi::c_int as isize) as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (32 as size_t).wrapping_mul(::core::mem::size_of::<int32_t>() as size_t),
        );
        col += 1;
    }
    let mut col_0: core::ffi::c_int = 1 as core::ffi::c_int;
    while col_0 < 32 as core::ffi::c_int {
        memcpy(
            output.offset((col_0 * 32 as core::ffi::c_int) as isize)
                as *mut core::ffi::c_void,
            output.offset((col_0 * 64 as core::ffi::c_int) as isize)
                as *const core::ffi::c_void,
            (32 as size_t).wrapping_mul(::core::mem::size_of::<int32_t>() as size_t),
        );
        col_0 += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_fwd_txfm2d_64x32_c(
    mut input: *const int16_t,
    mut output: *mut int32_t,
    mut stride: core::ffi::c_int,
    mut tx_type: TX_TYPE,
    mut bd: core::ffi::c_int,
) {
    let mut txfm_buf: [int32_t; 2048] = [0; 2048];
    let mut cfg: TXFM_2D_FLIP_CFG = TXFM_2D_FLIP_CFG {
        tx_size: 0,
        ud_flip: 0,
        lr_flip: 0,
        shift: 0 as *const int8_t,
        cos_bit_col: 0,
        cos_bit_row: 0,
        stage_range_col: [0; 12],
        stage_range_row: [0; 12],
        txfm_type_col: 0,
        txfm_type_row: 0,
        stage_num_col: 0,
        stage_num_row: 0,
    };
    av1_get_fwd_txfm_cfg(tx_type, TX_64X32 as core::ffi::c_int as TX_SIZE, &raw mut cfg);
    fwd_txfm2d_c(input, output, stride, &raw mut cfg, txfm_buf.as_mut_ptr(), bd);
    memset(
        output.offset((32 as core::ffi::c_int * 32 as core::ffi::c_int) as isize)
            as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ((32 as core::ffi::c_int * 32 as core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<int32_t>() as size_t),
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_fwd_txfm2d_16x64_c(
    mut input: *const int16_t,
    mut output: *mut int32_t,
    mut stride: core::ffi::c_int,
    mut tx_type: TX_TYPE,
    mut bd: core::ffi::c_int,
) {
    let mut txfm_buf: crate::platform::Aligned32<[int32_t; 1024]> = crate::platform::Aligned32([0; 1024]);
    let mut cfg: TXFM_2D_FLIP_CFG = TXFM_2D_FLIP_CFG {
        tx_size: 0,
        ud_flip: 0,
        lr_flip: 0,
        shift: 0 as *const int8_t,
        cos_bit_col: 0,
        cos_bit_row: 0,
        stage_range_col: [0; 12],
        stage_range_row: [0; 12],
        txfm_type_col: 0,
        txfm_type_row: 0,
        stage_num_col: 0,
        stage_num_row: 0,
    };
    av1_get_fwd_txfm_cfg(tx_type, TX_16X64 as core::ffi::c_int as TX_SIZE, &raw mut cfg);
    fwd_txfm2d_c(input, output, stride, &raw mut cfg, txfm_buf.0.as_mut_ptr(), bd);
    let mut row: core::ffi::c_int = 0 as core::ffi::c_int;
    while row < 16 as core::ffi::c_int {
        memset(
            output
                .offset((row * 64 as core::ffi::c_int) as isize)
                .offset(32 as core::ffi::c_int as isize) as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (32 as size_t).wrapping_mul(::core::mem::size_of::<int32_t>() as size_t),
        );
        row += 1;
    }
    let mut row_0: core::ffi::c_int = 1 as core::ffi::c_int;
    while row_0 < 16 as core::ffi::c_int {
        memcpy(
            output.offset((row_0 * 32 as core::ffi::c_int) as isize)
                as *mut core::ffi::c_void,
            output.offset((row_0 * 64 as core::ffi::c_int) as isize)
                as *const core::ffi::c_void,
            (32 as size_t).wrapping_mul(::core::mem::size_of::<int32_t>() as size_t),
        );
        row_0 += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_fwd_txfm2d_64x16_c(
    mut input: *const int16_t,
    mut output: *mut int32_t,
    mut stride: core::ffi::c_int,
    mut tx_type: TX_TYPE,
    mut bd: core::ffi::c_int,
) {
    let mut txfm_buf: [int32_t; 1024] = [0; 1024];
    let mut cfg: TXFM_2D_FLIP_CFG = TXFM_2D_FLIP_CFG {
        tx_size: 0,
        ud_flip: 0,
        lr_flip: 0,
        shift: 0 as *const int8_t,
        cos_bit_col: 0,
        cos_bit_row: 0,
        stage_range_col: [0; 12],
        stage_range_row: [0; 12],
        txfm_type_col: 0,
        txfm_type_row: 0,
        stage_num_col: 0,
        stage_num_row: 0,
    };
    av1_get_fwd_txfm_cfg(tx_type, TX_64X16 as core::ffi::c_int as TX_SIZE, &raw mut cfg);
    fwd_txfm2d_c(input, output, stride, &raw mut cfg, txfm_buf.as_mut_ptr(), bd);
    memset(
        output.offset((16 as core::ffi::c_int * 32 as core::ffi::c_int) as isize)
            as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ((16 as core::ffi::c_int * 32 as core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<int32_t>() as size_t),
    );
}
static mut fwd_shift_4x4: [int8_t; 3] = [
    2 as core::ffi::c_int as int8_t,
    0 as core::ffi::c_int as int8_t,
    0 as core::ffi::c_int as int8_t,
];
static mut fwd_shift_8x8: [int8_t; 3] = [
    2 as core::ffi::c_int as int8_t,
    -(1 as core::ffi::c_int) as int8_t,
    0 as core::ffi::c_int as int8_t,
];
static mut fwd_shift_16x16: [int8_t; 3] = [
    2 as core::ffi::c_int as int8_t,
    -(2 as core::ffi::c_int) as int8_t,
    0 as core::ffi::c_int as int8_t,
];
static mut fwd_shift_32x32: [int8_t; 3] = [
    2 as core::ffi::c_int as int8_t,
    -(4 as core::ffi::c_int) as int8_t,
    0 as core::ffi::c_int as int8_t,
];
static mut fwd_shift_64x64: [int8_t; 3] = [
    0 as core::ffi::c_int as int8_t,
    -(2 as core::ffi::c_int) as int8_t,
    -(2 as core::ffi::c_int) as int8_t,
];
static mut fwd_shift_4x8: [int8_t; 3] = [
    2 as core::ffi::c_int as int8_t,
    -(1 as core::ffi::c_int) as int8_t,
    0 as core::ffi::c_int as int8_t,
];
static mut fwd_shift_8x4: [int8_t; 3] = [
    2 as core::ffi::c_int as int8_t,
    -(1 as core::ffi::c_int) as int8_t,
    0 as core::ffi::c_int as int8_t,
];
static mut fwd_shift_8x16: [int8_t; 3] = [
    2 as core::ffi::c_int as int8_t,
    -(2 as core::ffi::c_int) as int8_t,
    0 as core::ffi::c_int as int8_t,
];
static mut fwd_shift_16x8: [int8_t; 3] = [
    2 as core::ffi::c_int as int8_t,
    -(2 as core::ffi::c_int) as int8_t,
    0 as core::ffi::c_int as int8_t,
];
static mut fwd_shift_16x32: [int8_t; 3] = [
    2 as core::ffi::c_int as int8_t,
    -(4 as core::ffi::c_int) as int8_t,
    0 as core::ffi::c_int as int8_t,
];
static mut fwd_shift_32x16: [int8_t; 3] = [
    2 as core::ffi::c_int as int8_t,
    -(4 as core::ffi::c_int) as int8_t,
    0 as core::ffi::c_int as int8_t,
];
static mut fwd_shift_32x64: [int8_t; 3] = [
    0 as core::ffi::c_int as int8_t,
    -(2 as core::ffi::c_int) as int8_t,
    -(2 as core::ffi::c_int) as int8_t,
];
static mut fwd_shift_64x32: [int8_t; 3] = [
    2 as core::ffi::c_int as int8_t,
    -(4 as core::ffi::c_int) as int8_t,
    -(2 as core::ffi::c_int) as int8_t,
];
static mut fwd_shift_4x16: [int8_t; 3] = [
    2 as core::ffi::c_int as int8_t,
    -(1 as core::ffi::c_int) as int8_t,
    0 as core::ffi::c_int as int8_t,
];
static mut fwd_shift_16x4: [int8_t; 3] = [
    2 as core::ffi::c_int as int8_t,
    -(1 as core::ffi::c_int) as int8_t,
    0 as core::ffi::c_int as int8_t,
];
static mut fwd_shift_8x32: [int8_t; 3] = [
    2 as core::ffi::c_int as int8_t,
    -(2 as core::ffi::c_int) as int8_t,
    0 as core::ffi::c_int as int8_t,
];
static mut fwd_shift_32x8: [int8_t; 3] = [
    2 as core::ffi::c_int as int8_t,
    -(2 as core::ffi::c_int) as int8_t,
    0 as core::ffi::c_int as int8_t,
];
static mut fwd_shift_16x64: [int8_t; 3] = [
    0 as core::ffi::c_int as int8_t,
    -(2 as core::ffi::c_int) as int8_t,
    0 as core::ffi::c_int as int8_t,
];
static mut fwd_shift_64x16: [int8_t; 3] = [
    2 as core::ffi::c_int as int8_t,
    -(4 as core::ffi::c_int) as int8_t,
    0 as core::ffi::c_int as int8_t,
];
#[no_mangle]
pub static mut av1_fwd_txfm_shift_ls: [*const int8_t; 19] = unsafe {
    [
        core::ptr::addr_of!(fwd_shift_4x4).cast::<int8_t>(),
        core::ptr::addr_of!(fwd_shift_8x8).cast::<int8_t>(),
        core::ptr::addr_of!(fwd_shift_16x16).cast::<int8_t>(),
        core::ptr::addr_of!(fwd_shift_32x32).cast::<int8_t>(),
        core::ptr::addr_of!(fwd_shift_64x64).cast::<int8_t>(),
        core::ptr::addr_of!(fwd_shift_4x8).cast::<int8_t>(),
        core::ptr::addr_of!(fwd_shift_8x4).cast::<int8_t>(),
        core::ptr::addr_of!(fwd_shift_8x16).cast::<int8_t>(),
        core::ptr::addr_of!(fwd_shift_16x8).cast::<int8_t>(),
        core::ptr::addr_of!(fwd_shift_16x32).cast::<int8_t>(),
        core::ptr::addr_of!(fwd_shift_32x16).cast::<int8_t>(),
        core::ptr::addr_of!(fwd_shift_32x64).cast::<int8_t>(),
        core::ptr::addr_of!(fwd_shift_64x32).cast::<int8_t>(),
        core::ptr::addr_of!(fwd_shift_4x16).cast::<int8_t>(),
        core::ptr::addr_of!(fwd_shift_16x4).cast::<int8_t>(),
        core::ptr::addr_of!(fwd_shift_8x32).cast::<int8_t>(),
        core::ptr::addr_of!(fwd_shift_32x8).cast::<int8_t>(),
        core::ptr::addr_of!(fwd_shift_16x64).cast::<int8_t>(),
        core::ptr::addr_of!(fwd_shift_64x16).cast::<int8_t>(),
    ]
};
#[no_mangle]
pub static mut av1_fwd_cos_bit_col: [[int8_t; 5]; 5] = [
    [
        13 as core::ffi::c_int as int8_t,
        13 as core::ffi::c_int as int8_t,
        13 as core::ffi::c_int as int8_t,
        0 as core::ffi::c_int as int8_t,
        0 as core::ffi::c_int as int8_t,
    ],
    [
        13 as core::ffi::c_int as int8_t,
        13 as core::ffi::c_int as int8_t,
        13 as core::ffi::c_int as int8_t,
        12 as core::ffi::c_int as int8_t,
        0 as core::ffi::c_int as int8_t,
    ],
    [
        13 as core::ffi::c_int as int8_t,
        13 as core::ffi::c_int as int8_t,
        13 as core::ffi::c_int as int8_t,
        12 as core::ffi::c_int as int8_t,
        13 as core::ffi::c_int as int8_t,
    ],
    [
        0 as core::ffi::c_int as int8_t,
        13 as core::ffi::c_int as int8_t,
        13 as core::ffi::c_int as int8_t,
        12 as core::ffi::c_int as int8_t,
        13 as core::ffi::c_int as int8_t,
    ],
    [
        0 as core::ffi::c_int as int8_t,
        0 as core::ffi::c_int as int8_t,
        13 as core::ffi::c_int as int8_t,
        12 as core::ffi::c_int as int8_t,
        13 as core::ffi::c_int as int8_t,
    ],
];
#[no_mangle]
pub static mut av1_fwd_cos_bit_row: [[int8_t; 5]; 5] = [
    [
        13 as core::ffi::c_int as int8_t,
        13 as core::ffi::c_int as int8_t,
        12 as core::ffi::c_int as int8_t,
        0 as core::ffi::c_int as int8_t,
        0 as core::ffi::c_int as int8_t,
    ],
    [
        13 as core::ffi::c_int as int8_t,
        13 as core::ffi::c_int as int8_t,
        13 as core::ffi::c_int as int8_t,
        12 as core::ffi::c_int as int8_t,
        0 as core::ffi::c_int as int8_t,
    ],
    [
        13 as core::ffi::c_int as int8_t,
        13 as core::ffi::c_int as int8_t,
        12 as core::ffi::c_int as int8_t,
        13 as core::ffi::c_int as int8_t,
        12 as core::ffi::c_int as int8_t,
    ],
    [
        0 as core::ffi::c_int as int8_t,
        12 as core::ffi::c_int as int8_t,
        13 as core::ffi::c_int as int8_t,
        12 as core::ffi::c_int as int8_t,
        11 as core::ffi::c_int as int8_t,
    ],
    [
        0 as core::ffi::c_int as int8_t,
        0 as core::ffi::c_int as int8_t,
        12 as core::ffi::c_int as int8_t,
        11 as core::ffi::c_int as int8_t,
        10 as core::ffi::c_int as int8_t,
    ],
];
static mut fdct4_range_mult2: [int8_t; 4] = [
    0 as core::ffi::c_int as int8_t,
    2 as core::ffi::c_int as int8_t,
    3 as core::ffi::c_int as int8_t,
    3 as core::ffi::c_int as int8_t,
];
static mut fdct8_range_mult2: [int8_t; 6] = [
    0 as core::ffi::c_int as int8_t,
    2 as core::ffi::c_int as int8_t,
    4 as core::ffi::c_int as int8_t,
    5 as core::ffi::c_int as int8_t,
    5 as core::ffi::c_int as int8_t,
    5 as core::ffi::c_int as int8_t,
];
static mut fdct16_range_mult2: [int8_t; 8] = [
    0 as core::ffi::c_int as int8_t,
    2 as core::ffi::c_int as int8_t,
    4 as core::ffi::c_int as int8_t,
    6 as core::ffi::c_int as int8_t,
    7 as core::ffi::c_int as int8_t,
    7 as core::ffi::c_int as int8_t,
    7 as core::ffi::c_int as int8_t,
    7 as core::ffi::c_int as int8_t,
];
static mut fdct32_range_mult2: [int8_t; 10] = [
    0 as core::ffi::c_int as int8_t,
    2 as core::ffi::c_int as int8_t,
    4 as core::ffi::c_int as int8_t,
    6 as core::ffi::c_int as int8_t,
    8 as core::ffi::c_int as int8_t,
    9 as core::ffi::c_int as int8_t,
    9 as core::ffi::c_int as int8_t,
    9 as core::ffi::c_int as int8_t,
    9 as core::ffi::c_int as int8_t,
    9 as core::ffi::c_int as int8_t,
];
static mut fdct64_range_mult2: [int8_t; 12] = [
    0 as core::ffi::c_int as int8_t,
    2 as core::ffi::c_int as int8_t,
    4 as core::ffi::c_int as int8_t,
    6 as core::ffi::c_int as int8_t,
    8 as core::ffi::c_int as int8_t,
    10 as core::ffi::c_int as int8_t,
    11 as core::ffi::c_int as int8_t,
    11 as core::ffi::c_int as int8_t,
    11 as core::ffi::c_int as int8_t,
    11 as core::ffi::c_int as int8_t,
    11 as core::ffi::c_int as int8_t,
    11 as core::ffi::c_int as int8_t,
];
static mut fadst4_range_mult2: [int8_t; 7] = [
    0 as core::ffi::c_int as int8_t,
    2 as core::ffi::c_int as int8_t,
    4 as core::ffi::c_int as int8_t,
    3 as core::ffi::c_int as int8_t,
    3 as core::ffi::c_int as int8_t,
    3 as core::ffi::c_int as int8_t,
    3 as core::ffi::c_int as int8_t,
];
static mut fadst8_range_mult2: [int8_t; 8] = [
    0 as core::ffi::c_int as int8_t,
    0 as core::ffi::c_int as int8_t,
    1 as core::ffi::c_int as int8_t,
    3 as core::ffi::c_int as int8_t,
    3 as core::ffi::c_int as int8_t,
    5 as core::ffi::c_int as int8_t,
    5 as core::ffi::c_int as int8_t,
    5 as core::ffi::c_int as int8_t,
];
static mut fadst16_range_mult2: [int8_t; 10] = [
    0 as core::ffi::c_int as int8_t,
    0 as core::ffi::c_int as int8_t,
    1 as core::ffi::c_int as int8_t,
    3 as core::ffi::c_int as int8_t,
    3 as core::ffi::c_int as int8_t,
    5 as core::ffi::c_int as int8_t,
    5 as core::ffi::c_int as int8_t,
    7 as core::ffi::c_int as int8_t,
    7 as core::ffi::c_int as int8_t,
    7 as core::ffi::c_int as int8_t,
];
static mut fidtx4_range_mult2: [int8_t; 1] = [1 as core::ffi::c_int as int8_t];
static mut fidtx8_range_mult2: [int8_t; 1] = [2 as core::ffi::c_int as int8_t];
static mut fidtx16_range_mult2: [int8_t; 1] = [3 as core::ffi::c_int as int8_t];
static mut fidtx32_range_mult2: [int8_t; 1] = [4 as core::ffi::c_int as int8_t];
static mut fwd_txfm_range_mult2_list: [*const int8_t; 12] = unsafe {
    [
        core::ptr::addr_of!(fdct4_range_mult2).cast::<int8_t>(),
        core::ptr::addr_of!(fdct8_range_mult2).cast::<int8_t>(),
        core::ptr::addr_of!(fdct16_range_mult2).cast::<int8_t>(),
        core::ptr::addr_of!(fdct32_range_mult2).cast::<int8_t>(),
        core::ptr::addr_of!(fdct64_range_mult2).cast::<int8_t>(),
        core::ptr::addr_of!(fadst4_range_mult2).cast::<int8_t>(),
        core::ptr::addr_of!(fadst8_range_mult2).cast::<int8_t>(),
        core::ptr::addr_of!(fadst16_range_mult2).cast::<int8_t>(),
        core::ptr::addr_of!(fidtx4_range_mult2).cast::<int8_t>(),
        core::ptr::addr_of!(fidtx8_range_mult2).cast::<int8_t>(),
        core::ptr::addr_of!(fidtx16_range_mult2).cast::<int8_t>(),
        core::ptr::addr_of!(fidtx32_range_mult2).cast::<int8_t>(),
    ]
};
#[inline]
unsafe extern "C-unwind" fn set_fwd_txfm_non_scale_range(mut cfg: *mut TXFM_2D_FLIP_CFG) {
    memset(
        &raw mut (*cfg).stage_range_col as *mut [int8_t; 12] as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<[int8_t; 12]>() as size_t,
    );
    memset(
        &raw mut (*cfg).stage_range_row as *mut [int8_t; 12] as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<[int8_t; 12]>() as size_t,
    );
    let range_mult2_col: *const int8_t = fwd_txfm_range_mult2_list[(*cfg).txfm_type_col
        as usize];
    let stage_num_col: core::ffi::c_int = (*cfg).stage_num_col;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < stage_num_col && i < MAX_TXFM_STAGE_NUM {
        (*cfg).stage_range_col[i as usize] = (*range_mult2_col.offset(i as isize)
            as core::ffi::c_int + 1 as core::ffi::c_int >> 1 as core::ffi::c_int)
            as int8_t;
        i += 1;
    }
    let range_mult2_row: *const int8_t = fwd_txfm_range_mult2_list[(*cfg).txfm_type_row
        as usize];
    let stage_num_row: core::ffi::c_int = (*cfg).stage_num_row;
    let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while i_0 < stage_num_row && i_0 < MAX_TXFM_STAGE_NUM {
        (*cfg).stage_range_row[i_0 as usize] = (*range_mult2_col
            .offset((stage_num_col - 1 as core::ffi::c_int) as isize) as core::ffi::c_int
            + *range_mult2_row.offset(i_0 as isize) as core::ffi::c_int
            + 1 as core::ffi::c_int >> 1 as core::ffi::c_int) as int8_t;
        i_0 += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_get_fwd_txfm_cfg(
    mut tx_type: TX_TYPE,
    mut tx_size: TX_SIZE,
    mut cfg: *mut TXFM_2D_FLIP_CFG,
) {
    (*cfg).tx_size = tx_size;
    set_flip_cfg(tx_type, cfg);
    let tx_type_1d_col: TX_TYPE_1D = vtx_tab[tx_type as usize];
    let tx_type_1d_row: TX_TYPE_1D = htx_tab[tx_type as usize];
    let txw_idx: core::ffi::c_int = get_txw_idx(tx_size) as core::ffi::c_int;
    let txh_idx: core::ffi::c_int = get_txh_idx(tx_size) as core::ffi::c_int;
    (*cfg).shift = av1_fwd_txfm_shift_ls[tx_size as usize];
    (*cfg).cos_bit_col = av1_fwd_cos_bit_col[txw_idx as usize][txh_idx as usize];
    (*cfg).cos_bit_row = av1_fwd_cos_bit_row[txw_idx as usize][txh_idx as usize];
    (*cfg).txfm_type_col = av1_txfm_type_ls[txh_idx as usize][tx_type_1d_col as usize];
    (*cfg).txfm_type_row = av1_txfm_type_ls[txw_idx as usize][tx_type_1d_row as usize];
    (*cfg).stage_num_col = av1_txfm_stage_num_list[(*cfg).txfm_type_col as usize]
        as core::ffi::c_int;
    (*cfg).stage_num_row = av1_txfm_stage_num_list[(*cfg).txfm_type_row as usize]
        as core::ffi::c_int;
    set_fwd_txfm_non_scale_range(cfg);
}

extern "C" {
    fn memcpy(
        __dst: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn memset(
        __b: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __len: size_t,
    ) -> *mut core::ffi::c_void;
    fn abs(_: core::ffi::c_int) -> core::ffi::c_int;
}
