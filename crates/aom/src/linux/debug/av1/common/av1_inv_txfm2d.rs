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

// Mechanically translated from upstream/aom/av1/common/av1_inv_txfm2d.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C-unwind" {
    static av1_txfm_type_ls: [[TXFM_TYPE; 4]; 5];
    static av1_txfm_stage_num_list: [int8_t; 12];
    fn av1_round_shift_array_c(
        arr: *mut int32_t,
        size: core::ffi::c_int,
        bit: core::ffi::c_int,
    );
    fn av1_idct4(
        input: *const int32_t,
        output: *mut int32_t,
        cos_bit: int8_t,
        stage_range: *const int8_t,
    );
    fn av1_idct8(
        input: *const int32_t,
        output: *mut int32_t,
        cos_bit: int8_t,
        stage_range: *const int8_t,
    );
    fn av1_idct16(
        input: *const int32_t,
        output: *mut int32_t,
        cos_bit: int8_t,
        stage_range: *const int8_t,
    );
    fn av1_idct32(
        input: *const int32_t,
        output: *mut int32_t,
        cos_bit: int8_t,
        stage_range: *const int8_t,
    );
    fn av1_idct64(
        input: *const int32_t,
        output: *mut int32_t,
        cos_bit: int8_t,
        stage_range: *const int8_t,
    );
    fn av1_iadst4(
        input: *const int32_t,
        output: *mut int32_t,
        cos_bit: int8_t,
        stage_range: *const int8_t,
    );
    fn av1_iadst8(
        input: *const int32_t,
        output: *mut int32_t,
        cos_bit: int8_t,
        stage_range: *const int8_t,
    );
    fn av1_iadst16(
        input: *const int32_t,
        output: *mut int32_t,
        cos_bit: int8_t,
        stage_range: *const int8_t,
    );
    fn av1_iidentity4_c(
        input: *const int32_t,
        output: *mut int32_t,
        cos_bit: int8_t,
        stage_range: *const int8_t,
    );
    fn av1_iidentity8_c(
        input: *const int32_t,
        output: *mut int32_t,
        cos_bit: int8_t,
        stage_range: *const int8_t,
    );
    fn av1_iidentity16_c(
        input: *const int32_t,
        output: *mut int32_t,
        cos_bit: int8_t,
        stage_range: *const int8_t,
    );
    fn av1_iidentity32_c(
        input: *const int32_t,
        output: *mut int32_t,
        cos_bit: int8_t,
        stage_range: *const int8_t,
    );
}
pub type size_t = usize;
pub type __int8_t = i8;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __int64_t = i64;
pub type int8_t = __int8_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uintptr_t = usize;
pub type tran_high_t = int64_t;
pub type tran_low_t = int32_t;
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
#[inline]
unsafe extern "C-unwind" fn clamp(
    mut value: core::ffi::c_int,
    mut low: core::ffi::c_int,
    mut high: core::ffi::c_int,
) -> core::ffi::c_int {
    return if value < low { low } else if value > high { high } else { value };
}
#[inline]
unsafe extern "C-unwind" fn clamp64(
    mut value: int64_t,
    mut low: int64_t,
    mut high: int64_t,
) -> int64_t {
    return if value < low { low } else if value > high { high } else { value };
}
#[inline]
unsafe extern "C-unwind" fn clip_pixel_highbd(
    mut val: core::ffi::c_int,
    mut bd: core::ffi::c_int,
) -> uint16_t {
    match bd {
        10 => {
            return clamp(val, 0 as core::ffi::c_int, 1023 as core::ffi::c_int)
                as uint16_t;
        }
        12 => {
            return clamp(val, 0 as core::ffi::c_int, 4095 as core::ffi::c_int)
                as uint16_t;
        }
        8 | _ => {
            return clamp(val, 0 as core::ffi::c_int, 255 as core::ffi::c_int) as uint16_t;
        }
    };
}
pub const UNIT_QUANT_SHIFT: core::ffi::c_int = 2 as core::ffi::c_int;
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
static mut NewInvSqrt2: int32_t = 2896 as int32_t;
#[inline]
unsafe extern "C-unwind" fn range_check_value(mut value: int32_t, mut bit: int8_t) -> int32_t {
    return value;
}
#[inline]
unsafe extern "C-unwind" fn round_shift(
    mut value: int64_t,
    mut bit: core::ffi::c_int,
) -> int32_t {
    if bit >= 1 as core::ffi::c_int {} else {
        __assert_fail(
            b"bit >= 1\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/common/av1_txfm.h\0" as *const u8
                as *const core::ffi::c_char,
            119 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 34],
                [core::ffi::c_char; 34],
            >(*b"int32_t round_shift(int64_t, int)\0"))
                .as_ptr(),
        );
    };
    return (value as core::ffi::c_longlong
        + ((1 as core::ffi::c_longlong) << bit - 1 as core::ffi::c_int) >> bit)
        as int32_t;
}
#[inline]
unsafe extern "C-unwind" fn highbd_clip_pixel_add(
    mut dest: uint16_t,
    mut trans: tran_high_t,
    mut bd: core::ffi::c_int,
) -> uint16_t {
    return clip_pixel_highbd(dest as core::ffi::c_int + trans as core::ffi::c_int, bd);
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
            if 0 as core::ffi::c_int != 0 {} else {
                __assert_fail(
                    b"0\0" as *const u8 as *const core::ffi::c_char,
                    b"upstream/aom/av1/common/av1_txfm.h\0" as *const u8
                        as *const core::ffi::c_char,
                    244 as core::ffi::c_uint,
                    (::core::mem::transmute::<
                        [u8; 41],
                        [core::ffi::c_char; 41],
                    >(*b"void get_flip_cfg(TX_TYPE, int *, int *)\0"))
                        .as_ptr(),
                );
            };
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
        if 0 as core::ffi::c_int != 0
            && !(b"Unsupported transform size\0" as *const u8
                as *const core::ffi::c_char)
                .is_null()
        {} else {
            __assert_fail(
                b"0 && \"Unsupported transform size\"\0" as *const u8
                    as *const core::ffi::c_char,
                b"upstream/aom/av1/common/av1_txfm.h\0" as *const u8
                    as *const core::ffi::c_char,
                259 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 36],
                    [core::ffi::c_char; 36],
                >(*b"int get_rect_tx_log_ratio(int, int)\0"))
                    .as_ptr(),
            );
        };
    } else {
        if row == col * 2 as core::ffi::c_int {
            return -(1 as core::ffi::c_int);
        }
        if row == col * 4 as core::ffi::c_int {
            return -(2 as core::ffi::c_int);
        }
        if 0 as core::ffi::c_int != 0
            && !(b"Unsupported transform size\0" as *const u8
                as *const core::ffi::c_char)
                .is_null()
        {} else {
            __assert_fail(
                b"0 && \"Unsupported transform size\"\0" as *const u8
                    as *const core::ffi::c_char,
                b"upstream/aom/av1/common/av1_txfm.h\0" as *const u8
                    as *const core::ffi::c_char,
                263 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 36],
                    [core::ffi::c_char; 36],
                >(*b"int get_rect_tx_log_ratio(int, int)\0"))
                    .as_ptr(),
            );
        };
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
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_highbd_iwht4x4_16_add_c(
    mut input: *const tran_low_t,
    mut dest8: *mut uint8_t,
    mut stride: core::ffi::c_int,
    mut bd: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0;
    let mut output: [tran_low_t; 16] = [0; 16];
    let mut a1: tran_low_t = 0;
    let mut b1: tran_low_t = 0;
    let mut c1: tran_low_t = 0;
    let mut d1: tran_low_t = 0;
    let mut e1: tran_low_t = 0;
    let mut ip: *const tran_low_t = input;
    let mut op: *mut tran_low_t = output.as_mut_ptr();
    let mut dest: *mut uint16_t = ((dest8 as uintptr_t) << 1 as core::ffi::c_int)
        as *mut uint16_t;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        a1 = *ip.offset((4 as core::ffi::c_int * 0 as core::ffi::c_int) as isize)
            >> UNIT_QUANT_SHIFT;
        c1 = *ip.offset((4 as core::ffi::c_int * 1 as core::ffi::c_int) as isize)
            >> UNIT_QUANT_SHIFT;
        d1 = *ip.offset((4 as core::ffi::c_int * 2 as core::ffi::c_int) as isize)
            >> UNIT_QUANT_SHIFT;
        b1 = *ip.offset((4 as core::ffi::c_int * 3 as core::ffi::c_int) as isize)
            >> UNIT_QUANT_SHIFT;
        a1 += c1;
        d1 -= b1;
        e1 = a1 - d1 >> 1 as core::ffi::c_int;
        b1 = e1 - b1;
        c1 = e1 - c1;
        a1 -= b1;
        d1 += c1;
        *op.offset((4 as core::ffi::c_int * 0 as core::ffi::c_int) as isize) = a1;
        *op.offset((4 as core::ffi::c_int * 1 as core::ffi::c_int) as isize) = b1;
        *op.offset((4 as core::ffi::c_int * 2 as core::ffi::c_int) as isize) = c1;
        *op.offset((4 as core::ffi::c_int * 3 as core::ffi::c_int) as isize) = d1;
        ip = ip.offset(1);
        op = op.offset(1);
        i += 1;
    }
    ip = output.as_mut_ptr();
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        a1 = *ip.offset(0 as core::ffi::c_int as isize);
        c1 = *ip.offset(1 as core::ffi::c_int as isize);
        d1 = *ip.offset(2 as core::ffi::c_int as isize);
        b1 = *ip.offset(3 as core::ffi::c_int as isize);
        a1 += c1;
        d1 -= b1;
        e1 = a1 - d1 >> 1 as core::ffi::c_int;
        b1 = e1 - b1;
        c1 = e1 - c1;
        a1 -= b1;
        d1 += c1;
        range_check_value(a1 as int32_t, (bd + 1 as core::ffi::c_int) as int8_t);
        range_check_value(b1 as int32_t, (bd + 1 as core::ffi::c_int) as int8_t);
        range_check_value(c1 as int32_t, (bd + 1 as core::ffi::c_int) as int8_t);
        range_check_value(d1 as int32_t, (bd + 1 as core::ffi::c_int) as int8_t);
        *dest.offset((stride * 0 as core::ffi::c_int) as isize) = highbd_clip_pixel_add(
            *dest.offset((stride * 0 as core::ffi::c_int) as isize),
            a1 as tran_high_t,
            bd,
        );
        *dest.offset((stride * 1 as core::ffi::c_int) as isize) = highbd_clip_pixel_add(
            *dest.offset((stride * 1 as core::ffi::c_int) as isize),
            b1 as tran_high_t,
            bd,
        );
        *dest.offset((stride * 2 as core::ffi::c_int) as isize) = highbd_clip_pixel_add(
            *dest.offset((stride * 2 as core::ffi::c_int) as isize),
            c1 as tran_high_t,
            bd,
        );
        *dest.offset((stride * 3 as core::ffi::c_int) as isize) = highbd_clip_pixel_add(
            *dest.offset((stride * 3 as core::ffi::c_int) as isize),
            d1 as tran_high_t,
            bd,
        );
        ip = ip.offset(4 as core::ffi::c_int as isize);
        dest = dest.offset(1);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_highbd_iwht4x4_1_add_c(
    mut in_0: *const tran_low_t,
    mut dest8: *mut uint8_t,
    mut dest_stride: core::ffi::c_int,
    mut bd: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0;
    let mut a1: tran_low_t = 0;
    let mut e1: tran_low_t = 0;
    let mut tmp: [tran_low_t; 4] = [0; 4];
    let mut ip: *const tran_low_t = in_0;
    let mut op: *mut tran_low_t = tmp.as_mut_ptr();
    let mut dest: *mut uint16_t = ((dest8 as uintptr_t) << 1 as core::ffi::c_int)
        as *mut uint16_t;
    a1 = *ip.offset((0 as core::ffi::c_int * 4 as core::ffi::c_int) as isize)
        >> UNIT_QUANT_SHIFT;
    e1 = a1 >> 1 as core::ffi::c_int;
    a1 -= e1;
    *op.offset(0 as core::ffi::c_int as isize) = a1;
    let fresh0 = &raw mut *op.offset(3 as core::ffi::c_int as isize);
    *fresh0 = e1;
    let fresh1 = &raw mut *op.offset(2 as core::ffi::c_int as isize);
    *fresh1 = *fresh0;
    *op.offset(1 as core::ffi::c_int as isize) = *fresh1;
    ip = tmp.as_mut_ptr();
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        e1 = *ip.offset(0 as core::ffi::c_int as isize) >> 1 as core::ffi::c_int;
        a1 = *ip.offset(0 as core::ffi::c_int as isize) - e1;
        *dest.offset((dest_stride * 0 as core::ffi::c_int) as isize) = highbd_clip_pixel_add(
            *dest.offset((dest_stride * 0 as core::ffi::c_int) as isize),
            a1 as tran_high_t,
            bd,
        );
        *dest.offset((dest_stride * 1 as core::ffi::c_int) as isize) = highbd_clip_pixel_add(
            *dest.offset((dest_stride * 1 as core::ffi::c_int) as isize),
            e1 as tran_high_t,
            bd,
        );
        *dest.offset((dest_stride * 2 as core::ffi::c_int) as isize) = highbd_clip_pixel_add(
            *dest.offset((dest_stride * 2 as core::ffi::c_int) as isize),
            e1 as tran_high_t,
            bd,
        );
        *dest.offset((dest_stride * 3 as core::ffi::c_int) as isize) = highbd_clip_pixel_add(
            *dest.offset((dest_stride * 3 as core::ffi::c_int) as isize),
            e1 as tran_high_t,
            bd,
        );
        ip = ip.offset(1);
        dest = dest.offset(1);
        i += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn inv_txfm_type_to_func(mut txfm_type: TXFM_TYPE) -> TxfmFunc {
    match txfm_type as core::ffi::c_int {
        0 => {
            return Some(
                av1_idct4
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
                av1_idct8
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
                av1_idct16
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
                av1_idct32
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
                av1_idct64
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
                av1_iadst4
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
                av1_iadst8
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
                av1_iadst16
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
                av1_iidentity4_c
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
                av1_iidentity8_c
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
                av1_iidentity16_c
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
                av1_iidentity32_c
                    as unsafe extern "C-unwind" fn(
                        *const int32_t,
                        *mut int32_t,
                        int8_t,
                        *const int8_t,
                    ) -> (),
            );
        }
        _ => {
            if 0 as core::ffi::c_int != 0 {} else {
                __assert_fail(
                    b"0\0" as *const u8 as *const core::ffi::c_char,
                    b"upstream/aom/av1/common/av1_inv_txfm2d.c\0" as *const u8
                        as *const core::ffi::c_char,
                    130 as core::ffi::c_uint,
                    (::core::mem::transmute::<
                        [u8; 42],
                        [core::ffi::c_char; 42],
                    >(*b"TxfmFunc inv_txfm_type_to_func(TXFM_TYPE)\0"))
                        .as_ptr(),
                );
            };
            return None;
        }
    };
}
static mut inv_shift_4x4: [int8_t; 2] = [
    0 as core::ffi::c_int as int8_t,
    -(4 as core::ffi::c_int) as int8_t,
];
static mut inv_shift_8x8: [int8_t; 2] = [
    -(1 as core::ffi::c_int) as int8_t,
    -(4 as core::ffi::c_int) as int8_t,
];
static mut inv_shift_16x16: [int8_t; 2] = [
    -(2 as core::ffi::c_int) as int8_t,
    -(4 as core::ffi::c_int) as int8_t,
];
static mut inv_shift_32x32: [int8_t; 2] = [
    -(2 as core::ffi::c_int) as int8_t,
    -(4 as core::ffi::c_int) as int8_t,
];
static mut inv_shift_64x64: [int8_t; 2] = [
    -(2 as core::ffi::c_int) as int8_t,
    -(4 as core::ffi::c_int) as int8_t,
];
static mut inv_shift_4x8: [int8_t; 2] = [
    0 as core::ffi::c_int as int8_t,
    -(4 as core::ffi::c_int) as int8_t,
];
static mut inv_shift_8x4: [int8_t; 2] = [
    0 as core::ffi::c_int as int8_t,
    -(4 as core::ffi::c_int) as int8_t,
];
static mut inv_shift_8x16: [int8_t; 2] = [
    -(1 as core::ffi::c_int) as int8_t,
    -(4 as core::ffi::c_int) as int8_t,
];
static mut inv_shift_16x8: [int8_t; 2] = [
    -(1 as core::ffi::c_int) as int8_t,
    -(4 as core::ffi::c_int) as int8_t,
];
static mut inv_shift_16x32: [int8_t; 2] = [
    -(1 as core::ffi::c_int) as int8_t,
    -(4 as core::ffi::c_int) as int8_t,
];
static mut inv_shift_32x16: [int8_t; 2] = [
    -(1 as core::ffi::c_int) as int8_t,
    -(4 as core::ffi::c_int) as int8_t,
];
static mut inv_shift_32x64: [int8_t; 2] = [
    -(1 as core::ffi::c_int) as int8_t,
    -(4 as core::ffi::c_int) as int8_t,
];
static mut inv_shift_64x32: [int8_t; 2] = [
    -(1 as core::ffi::c_int) as int8_t,
    -(4 as core::ffi::c_int) as int8_t,
];
static mut inv_shift_4x16: [int8_t; 2] = [
    -(1 as core::ffi::c_int) as int8_t,
    -(4 as core::ffi::c_int) as int8_t,
];
static mut inv_shift_16x4: [int8_t; 2] = [
    -(1 as core::ffi::c_int) as int8_t,
    -(4 as core::ffi::c_int) as int8_t,
];
static mut inv_shift_8x32: [int8_t; 2] = [
    -(2 as core::ffi::c_int) as int8_t,
    -(4 as core::ffi::c_int) as int8_t,
];
static mut inv_shift_32x8: [int8_t; 2] = [
    -(2 as core::ffi::c_int) as int8_t,
    -(4 as core::ffi::c_int) as int8_t,
];
static mut inv_shift_16x64: [int8_t; 2] = [
    -(2 as core::ffi::c_int) as int8_t,
    -(4 as core::ffi::c_int) as int8_t,
];
static mut inv_shift_64x16: [int8_t; 2] = [
    -(2 as core::ffi::c_int) as int8_t,
    -(4 as core::ffi::c_int) as int8_t,
];
#[no_mangle]
pub static mut av1_inv_txfm_shift_ls: [*const int8_t; 19] = unsafe {
    [
        core::ptr::addr_of!(inv_shift_4x4).cast::<int8_t>(),
        core::ptr::addr_of!(inv_shift_8x8).cast::<int8_t>(),
        core::ptr::addr_of!(inv_shift_16x16).cast::<int8_t>(),
        core::ptr::addr_of!(inv_shift_32x32).cast::<int8_t>(),
        core::ptr::addr_of!(inv_shift_64x64).cast::<int8_t>(),
        core::ptr::addr_of!(inv_shift_4x8).cast::<int8_t>(),
        core::ptr::addr_of!(inv_shift_8x4).cast::<int8_t>(),
        core::ptr::addr_of!(inv_shift_8x16).cast::<int8_t>(),
        core::ptr::addr_of!(inv_shift_16x8).cast::<int8_t>(),
        core::ptr::addr_of!(inv_shift_16x32).cast::<int8_t>(),
        core::ptr::addr_of!(inv_shift_32x16).cast::<int8_t>(),
        core::ptr::addr_of!(inv_shift_32x64).cast::<int8_t>(),
        core::ptr::addr_of!(inv_shift_64x32).cast::<int8_t>(),
        core::ptr::addr_of!(inv_shift_4x16).cast::<int8_t>(),
        core::ptr::addr_of!(inv_shift_16x4).cast::<int8_t>(),
        core::ptr::addr_of!(inv_shift_8x32).cast::<int8_t>(),
        core::ptr::addr_of!(inv_shift_32x8).cast::<int8_t>(),
        core::ptr::addr_of!(inv_shift_16x64).cast::<int8_t>(),
        core::ptr::addr_of!(inv_shift_64x16).cast::<int8_t>(),
    ]
};
static mut iadst4_range: [int8_t; 7] = [
    0 as core::ffi::c_int as int8_t,
    1 as core::ffi::c_int as int8_t,
    0 as core::ffi::c_int as int8_t,
    0 as core::ffi::c_int as int8_t,
    0 as core::ffi::c_int as int8_t,
    0 as core::ffi::c_int as int8_t,
    0 as core::ffi::c_int as int8_t,
];
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_get_inv_txfm_cfg(
    mut tx_type: TX_TYPE,
    mut tx_size: TX_SIZE,
    mut cfg: *mut TXFM_2D_FLIP_CFG,
) {
    if !cfg.is_null() {} else {
        __assert_fail(
            b"cfg != NULL\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/common/av1_inv_txfm2d.c\0" as *const u8
                as *const core::ffi::c_char,
            166 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 64],
                [core::ffi::c_char; 64],
            >(*b"void av1_get_inv_txfm_cfg(TX_TYPE, TX_SIZE, TXFM_2D_FLIP_CFG *)\0"))
                .as_ptr(),
        );
    };
    (*cfg).tx_size = tx_size;
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
    set_flip_cfg(tx_type, cfg);
    let tx_type_1d_col: TX_TYPE_1D = vtx_tab[tx_type as usize];
    let tx_type_1d_row: TX_TYPE_1D = htx_tab[tx_type as usize];
    (*cfg).shift = av1_inv_txfm_shift_ls[tx_size as usize];
    let txw_idx: core::ffi::c_int = get_txw_idx(tx_size) as core::ffi::c_int;
    let txh_idx: core::ffi::c_int = get_txh_idx(tx_size) as core::ffi::c_int;
    (*cfg).cos_bit_col = INV_COS_BIT as int8_t;
    (*cfg).cos_bit_row = INV_COS_BIT as int8_t;
    (*cfg).txfm_type_col = av1_txfm_type_ls[txh_idx as usize][tx_type_1d_col as usize];
    if (*cfg).txfm_type_col as core::ffi::c_int == TXFM_TYPE_ADST4 as core::ffi::c_int {
        memcpy(
            ((*cfg).stage_range_col).as_mut_ptr() as *mut core::ffi::c_void,
            core::ptr::addr_of!(iadst4_range).cast::<int8_t>() as *const core::ffi::c_void,
            ::core::mem::size_of::<[int8_t; 7]>() as size_t,
        );
    }
    (*cfg).txfm_type_row = av1_txfm_type_ls[txw_idx as usize][tx_type_1d_row as usize];
    if (*cfg).txfm_type_row as core::ffi::c_int == TXFM_TYPE_ADST4 as core::ffi::c_int {
        memcpy(
            ((*cfg).stage_range_row).as_mut_ptr() as *mut core::ffi::c_void,
            core::ptr::addr_of!(iadst4_range).cast::<int8_t>() as *const core::ffi::c_void,
            ::core::mem::size_of::<[int8_t; 7]>() as size_t,
        );
    }
    (*cfg).stage_num_col = av1_txfm_stage_num_list[(*cfg).txfm_type_col as usize]
        as core::ffi::c_int;
    (*cfg).stage_num_row = av1_txfm_stage_num_list[(*cfg).txfm_type_row as usize]
        as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_gen_inv_stage_range(
    mut stage_range_col: *mut int8_t,
    mut stage_range_row: *mut int8_t,
    mut cfg: *const TXFM_2D_FLIP_CFG,
    mut tx_size: TX_SIZE,
    mut bd: core::ffi::c_int,
) {
    let fwd_shift: core::ffi::c_int = inv_start_range[tx_size as usize]
        as core::ffi::c_int;
    let mut shift: *const int8_t = (*cfg).shift;
    let mut opt_range_row: int8_t = 0;
    let mut opt_range_col: int8_t = 0;
    if bd == 8 as core::ffi::c_int {
        opt_range_row = 16 as int8_t;
        opt_range_col = 16 as int8_t;
    } else if bd == 10 as core::ffi::c_int {
        opt_range_row = 18 as int8_t;
        opt_range_col = 16 as int8_t;
    } else {
        if bd == 12 as core::ffi::c_int {} else {
            __assert_fail(
                b"bd == 12\0" as *const u8 as *const core::ffi::c_char,
                b"upstream/aom/av1/common/av1_inv_txfm2d.c\0" as *const u8
                    as *const core::ffi::c_char,
                203 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 89],
                    [core::ffi::c_char; 89],
                >(
                    *b"void av1_gen_inv_stage_range(int8_t *, int8_t *, const TXFM_2D_FLIP_CFG *, TX_SIZE, int)\0",
                ))
                    .as_ptr(),
            );
        };
        opt_range_row = 20 as int8_t;
        opt_range_col = 18 as int8_t;
    }
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < (*cfg).stage_num_row && i < MAX_TXFM_STAGE_NUM {
        let mut real_range_row: core::ffi::c_int = (*cfg).stage_range_row[i as usize]
            as core::ffi::c_int + fwd_shift + bd + 1 as core::ffi::c_int;
        if (*cfg).txfm_type_row as core::ffi::c_int
            == TXFM_TYPE_ADST4 as core::ffi::c_int && i == 1 as core::ffi::c_int
        {
            *stage_range_row.offset(i as isize) = opt_range_row;
        } else {
            if opt_range_row as core::ffi::c_int >= real_range_row {} else {
                __assert_fail(
                    b"opt_range_row >= real_range_row\0" as *const u8
                        as *const core::ffi::c_char,
                    b"upstream/aom/av1/common/av1_inv_txfm2d.c\0" as *const u8
                        as *const core::ffi::c_char,
                    216 as core::ffi::c_uint,
                    (::core::mem::transmute::<
                        [u8; 89],
                        [core::ffi::c_char; 89],
                    >(
                        *b"void av1_gen_inv_stage_range(int8_t *, int8_t *, const TXFM_2D_FLIP_CFG *, TX_SIZE, int)\0",
                    ))
                        .as_ptr(),
                );
            };
            *stage_range_row.offset(i as isize) = opt_range_row;
        }
        i += 1;
    }
    let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while i_0 < (*cfg).stage_num_col && i_0 < MAX_TXFM_STAGE_NUM {
        let mut real_range_col: core::ffi::c_int = (*cfg).stage_range_col[i_0 as usize]
            as core::ffi::c_int + fwd_shift
            + *shift.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int + bd
            + 1 as core::ffi::c_int;
        if (*cfg).txfm_type_col as core::ffi::c_int
            == TXFM_TYPE_ADST4 as core::ffi::c_int && i_0 == 1 as core::ffi::c_int
        {
            *stage_range_col.offset(i_0 as isize) = opt_range_col;
        } else {
            if opt_range_col as core::ffi::c_int >= real_range_col {} else {
                __assert_fail(
                    b"opt_range_col >= real_range_col\0" as *const u8
                        as *const core::ffi::c_char,
                    b"upstream/aom/av1/common/av1_inv_txfm2d.c\0" as *const u8
                        as *const core::ffi::c_char,
                    230 as core::ffi::c_uint,
                    (::core::mem::transmute::<
                        [u8; 89],
                        [core::ffi::c_char; 89],
                    >(
                        *b"void av1_gen_inv_stage_range(int8_t *, int8_t *, const TXFM_2D_FLIP_CFG *, TX_SIZE, int)\0",
                    ))
                        .as_ptr(),
                );
            };
            *stage_range_col.offset(i_0 as isize) = opt_range_col;
        }
        i_0 += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn inv_txfm2d_add_c(
    mut input: *const int32_t,
    mut output: *mut uint16_t,
    mut stride: core::ffi::c_int,
    mut cfg: *mut TXFM_2D_FLIP_CFG,
    mut txfm_buf: *mut int32_t,
    mut tx_size: TX_SIZE,
    mut bd: core::ffi::c_int,
) {
    let txfm_size_col: core::ffi::c_int = tx_size_wide[(*cfg).tx_size as usize];
    let txfm_size_row: core::ffi::c_int = tx_size_high[(*cfg).tx_size as usize];
    let mut shift: *const int8_t = (*cfg).shift;
    let rect_type: core::ffi::c_int = get_rect_tx_log_ratio(txfm_size_col, txfm_size_row)
        as core::ffi::c_int;
    let mut stage_range_row: [int8_t; 12] = [0; 12];
    let mut stage_range_col: [int8_t; 12] = [0; 12];
    if (*cfg).stage_num_row <= 12 as core::ffi::c_int {} else {
        __assert_fail(
            b"cfg->stage_num_row <= MAX_TXFM_STAGE_NUM\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/common/av1_inv_txfm2d.c\0" as *const u8
                as *const core::ffi::c_char,
            253 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 101],
                [core::ffi::c_char; 101],
            >(
                *b"void inv_txfm2d_add_c(const int32_t *, uint16_t *, int, TXFM_2D_FLIP_CFG *, int32_t *, TX_SIZE, int)\0",
            ))
                .as_ptr(),
        );
    };
    if (*cfg).stage_num_col <= 12 as core::ffi::c_int {} else {
        __assert_fail(
            b"cfg->stage_num_col <= MAX_TXFM_STAGE_NUM\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/common/av1_inv_txfm2d.c\0" as *const u8
                as *const core::ffi::c_char,
            254 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 101],
                [core::ffi::c_char; 101],
            >(
                *b"void inv_txfm2d_add_c(const int32_t *, uint16_t *, int, TXFM_2D_FLIP_CFG *, int32_t *, TX_SIZE, int)\0",
            ))
                .as_ptr(),
        );
    };
    av1_gen_inv_stage_range(
        stage_range_col.as_mut_ptr(),
        stage_range_row.as_mut_ptr(),
        cfg,
        tx_size,
        bd,
    );
    let cos_bit_col: int8_t = (*cfg).cos_bit_col;
    let cos_bit_row: int8_t = (*cfg).cos_bit_row;
    let txfm_func_col: TxfmFunc = inv_txfm_type_to_func((*cfg).txfm_type_col)
        as TxfmFunc;
    let txfm_func_row: TxfmFunc = inv_txfm_type_to_func((*cfg).txfm_type_row)
        as TxfmFunc;
    let buf_offset: core::ffi::c_int = if txfm_size_row > txfm_size_col {
        txfm_size_row
    } else {
        txfm_size_col
    };
    let mut temp_in: *mut int32_t = txfm_buf;
    let mut temp_out: *mut int32_t = temp_in.offset(buf_offset as isize);
    let mut buf: *mut int32_t = temp_out.offset(buf_offset as isize);
    let mut buf_ptr: *mut int32_t = buf;
    let mut c: core::ffi::c_int = 0;
    let mut r: core::ffi::c_int = 0;
    r = 0 as core::ffi::c_int;
    while r < txfm_size_row {
        if abs(rect_type) == 1 as core::ffi::c_int {
            c = 0 as core::ffi::c_int;
            while c < txfm_size_col {
                *temp_in.offset(c as isize) = round_shift(
                    *input.offset((c * txfm_size_row + r) as isize) as int64_t
                        * NewInvSqrt2 as int64_t,
                    NewSqrt2Bits as core::ffi::c_int,
                );
                c += 1;
            }
            clamp_buf(
                temp_in,
                txfm_size_col as int32_t,
                (bd + 8 as core::ffi::c_int) as int8_t,
            );
            txfm_func_row
                .expect(
                    "non-null function pointer",
                )(temp_in, buf_ptr, cos_bit_row, stage_range_row.as_mut_ptr());
        } else {
            c = 0 as core::ffi::c_int;
            while c < txfm_size_col {
                *temp_in.offset(c as isize) = *input
                    .offset((c * txfm_size_row + r) as isize);
                c += 1;
            }
            clamp_buf(
                temp_in,
                txfm_size_col as int32_t,
                (bd + 8 as core::ffi::c_int) as int8_t,
            );
            txfm_func_row
                .expect(
                    "non-null function pointer",
                )(temp_in, buf_ptr, cos_bit_row, stage_range_row.as_mut_ptr());
        }
        av1_round_shift_array_c(
            buf_ptr,
            txfm_size_col,
            -(*shift.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int),
        );
        buf_ptr = buf_ptr.offset(txfm_size_col as isize);
        r += 1;
    }
    c = 0 as core::ffi::c_int;
    while c < txfm_size_col {
        if (*cfg).lr_flip == 0 as core::ffi::c_int {
            r = 0 as core::ffi::c_int;
            while r < txfm_size_row {
                *temp_in.offset(r as isize) = *buf
                    .offset((r * txfm_size_col + c) as isize);
                r += 1;
            }
        } else {
            r = 0 as core::ffi::c_int;
            while r < txfm_size_row {
                *temp_in.offset(r as isize) = *buf
                    .offset(
                        (r * txfm_size_col + (txfm_size_col - c - 1 as core::ffi::c_int))
                            as isize,
                    );
                r += 1;
            }
        }
        clamp_buf(
            temp_in,
            txfm_size_row as int32_t,
            (if bd + 6 as core::ffi::c_int > 16 as core::ffi::c_int {
                bd + 6 as core::ffi::c_int
            } else {
                16 as core::ffi::c_int
            }) as int8_t,
        );
        txfm_func_col
            .expect(
                "non-null function pointer",
            )(temp_in, temp_out, cos_bit_col, stage_range_col.as_mut_ptr());
        av1_round_shift_array_c(
            temp_out,
            txfm_size_row,
            -(*shift.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int),
        );
        if (*cfg).ud_flip == 0 as core::ffi::c_int {
            r = 0 as core::ffi::c_int;
            while r < txfm_size_row {
                *output.offset((r * stride + c) as isize) = highbd_clip_pixel_add(
                    *output.offset((r * stride + c) as isize),
                    *temp_out.offset(r as isize) as tran_high_t,
                    bd,
                );
                r += 1;
            }
        } else {
            r = 0 as core::ffi::c_int;
            while r < txfm_size_row {
                *output.offset((r * stride + c) as isize) = highbd_clip_pixel_add(
                    *output.offset((r * stride + c) as isize),
                    *temp_out
                        .offset((txfm_size_row - r - 1 as core::ffi::c_int) as isize)
                        as tran_high_t,
                    bd,
                );
                r += 1;
            }
        }
        c += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn inv_txfm2d_add_facade(
    mut input: *const int32_t,
    mut output: *mut uint16_t,
    mut stride: core::ffi::c_int,
    mut txfm_buf: *mut int32_t,
    mut tx_type: TX_TYPE,
    mut tx_size: TX_SIZE,
    mut bd: core::ffi::c_int,
) {
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
    av1_get_inv_txfm_cfg(tx_type, tx_size, &raw mut cfg);
    inv_txfm2d_add_c(input, output, stride, &raw mut cfg, txfm_buf, tx_size, bd);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_inv_txfm2d_add_4x8_c(
    mut input: *const int32_t,
    mut output: *mut uint16_t,
    mut stride: core::ffi::c_int,
    mut tx_type: TX_TYPE,
    mut bd: core::ffi::c_int,
) {
    let mut txfm_buf: crate::platform::Aligned32<[core::ffi::c_int; 48]> = crate::platform::Aligned32([0; 48]);
    inv_txfm2d_add_facade(
        input,
        output,
        stride,
        txfm_buf.0.as_mut_ptr() as *mut int32_t,
        tx_type,
        TX_4X8 as core::ffi::c_int as TX_SIZE,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_inv_txfm2d_add_8x4_c(
    mut input: *const int32_t,
    mut output: *mut uint16_t,
    mut stride: core::ffi::c_int,
    mut tx_type: TX_TYPE,
    mut bd: core::ffi::c_int,
) {
    let mut txfm_buf: crate::platform::Aligned32<[core::ffi::c_int; 48]> = crate::platform::Aligned32([0; 48]);
    inv_txfm2d_add_facade(
        input,
        output,
        stride,
        txfm_buf.0.as_mut_ptr() as *mut int32_t,
        tx_type,
        TX_8X4 as core::ffi::c_int as TX_SIZE,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_inv_txfm2d_add_8x16_c(
    mut input: *const int32_t,
    mut output: *mut uint16_t,
    mut stride: core::ffi::c_int,
    mut tx_type: TX_TYPE,
    mut bd: core::ffi::c_int,
) {
    let mut txfm_buf: crate::platform::Aligned32<[core::ffi::c_int; 160]> = crate::platform::Aligned32([0; 160]);
    inv_txfm2d_add_facade(
        input,
        output,
        stride,
        txfm_buf.0.as_mut_ptr() as *mut int32_t,
        tx_type,
        TX_8X16 as core::ffi::c_int as TX_SIZE,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_inv_txfm2d_add_16x8_c(
    mut input: *const int32_t,
    mut output: *mut uint16_t,
    mut stride: core::ffi::c_int,
    mut tx_type: TX_TYPE,
    mut bd: core::ffi::c_int,
) {
    let mut txfm_buf: crate::platform::Aligned32<[core::ffi::c_int; 160]> = crate::platform::Aligned32([0; 160]);
    inv_txfm2d_add_facade(
        input,
        output,
        stride,
        txfm_buf.0.as_mut_ptr() as *mut int32_t,
        tx_type,
        TX_16X8 as core::ffi::c_int as TX_SIZE,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_inv_txfm2d_add_16x32_c(
    mut input: *const int32_t,
    mut output: *mut uint16_t,
    mut stride: core::ffi::c_int,
    mut tx_type: TX_TYPE,
    mut bd: core::ffi::c_int,
) {
    let mut txfm_buf: crate::platform::Aligned32<[core::ffi::c_int; 576]> = crate::platform::Aligned32([0; 576]);
    inv_txfm2d_add_facade(
        input,
        output,
        stride,
        txfm_buf.0.as_mut_ptr() as *mut int32_t,
        tx_type,
        TX_16X32 as core::ffi::c_int as TX_SIZE,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_inv_txfm2d_add_32x16_c(
    mut input: *const int32_t,
    mut output: *mut uint16_t,
    mut stride: core::ffi::c_int,
    mut tx_type: TX_TYPE,
    mut bd: core::ffi::c_int,
) {
    let mut txfm_buf: crate::platform::Aligned32<[core::ffi::c_int; 576]> = crate::platform::Aligned32([0; 576]);
    inv_txfm2d_add_facade(
        input,
        output,
        stride,
        txfm_buf.0.as_mut_ptr() as *mut int32_t,
        tx_type,
        TX_32X16 as core::ffi::c_int as TX_SIZE,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_inv_txfm2d_add_4x4_c(
    mut input: *const int32_t,
    mut output: *mut uint16_t,
    mut stride: core::ffi::c_int,
    mut tx_type: TX_TYPE,
    mut bd: core::ffi::c_int,
) {
    let mut txfm_buf: crate::platform::Aligned32<[core::ffi::c_int; 24]> = crate::platform::Aligned32([0; 24]);
    inv_txfm2d_add_facade(
        input,
        output,
        stride,
        txfm_buf.0.as_mut_ptr() as *mut int32_t,
        tx_type,
        TX_4X4 as core::ffi::c_int as TX_SIZE,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_inv_txfm2d_add_8x8_c(
    mut input: *const int32_t,
    mut output: *mut uint16_t,
    mut stride: core::ffi::c_int,
    mut tx_type: TX_TYPE,
    mut bd: core::ffi::c_int,
) {
    let mut txfm_buf: crate::platform::Aligned32<[core::ffi::c_int; 80]> = crate::platform::Aligned32([0; 80]);
    inv_txfm2d_add_facade(
        input,
        output,
        stride,
        txfm_buf.0.as_mut_ptr() as *mut int32_t,
        tx_type,
        TX_8X8 as core::ffi::c_int as TX_SIZE,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_inv_txfm2d_add_16x16_c(
    mut input: *const int32_t,
    mut output: *mut uint16_t,
    mut stride: core::ffi::c_int,
    mut tx_type: TX_TYPE,
    mut bd: core::ffi::c_int,
) {
    let mut txfm_buf: crate::platform::Aligned32<[core::ffi::c_int; 288]> = crate::platform::Aligned32([0; 288]);
    inv_txfm2d_add_facade(
        input,
        output,
        stride,
        txfm_buf.0.as_mut_ptr() as *mut int32_t,
        tx_type,
        TX_16X16 as core::ffi::c_int as TX_SIZE,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_inv_txfm2d_add_32x32_c(
    mut input: *const int32_t,
    mut output: *mut uint16_t,
    mut stride: core::ffi::c_int,
    mut tx_type: TX_TYPE,
    mut bd: core::ffi::c_int,
) {
    let mut txfm_buf: crate::platform::Aligned32<[core::ffi::c_int; 1088]> = crate::platform::Aligned32([0; 1088]);
    inv_txfm2d_add_facade(
        input,
        output,
        stride,
        txfm_buf.0.as_mut_ptr() as *mut int32_t,
        tx_type,
        TX_32X32 as core::ffi::c_int as TX_SIZE,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_inv_txfm2d_add_64x64_c(
    mut input: *const int32_t,
    mut output: *mut uint16_t,
    mut stride: core::ffi::c_int,
    mut tx_type: TX_TYPE,
    mut bd: core::ffi::c_int,
) {
    let mut mod_input: [int32_t; 4096] = [0; 4096];
    let mut col: core::ffi::c_int = 0 as core::ffi::c_int;
    while col < 32 as core::ffi::c_int {
        memcpy(
            mod_input.as_mut_ptr().offset((col * 64 as core::ffi::c_int) as isize)
                as *mut core::ffi::c_void,
            input.offset((col * 32 as core::ffi::c_int) as isize)
                as *const core::ffi::c_void,
            (32 as size_t).wrapping_mul(::core::mem::size_of::<int32_t>() as size_t),
        );
        memset(
            mod_input
                .as_mut_ptr()
                .offset((col * 64 as core::ffi::c_int) as isize)
                .offset(32 as core::ffi::c_int as isize) as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (32 as size_t).wrapping_mul(::core::mem::size_of::<int32_t>() as size_t),
        );
        col += 1;
    }
    memset(
        mod_input
            .as_mut_ptr()
            .offset((32 as core::ffi::c_int * 64 as core::ffi::c_int) as isize)
            as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ((32 as core::ffi::c_int * 64 as core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<int32_t>() as size_t),
    );
    let mut txfm_buf: crate::platform::Aligned32<[core::ffi::c_int; 4224]> = crate::platform::Aligned32([0; 4224]);
    inv_txfm2d_add_facade(
        mod_input.as_mut_ptr(),
        output,
        stride,
        txfm_buf.0.as_mut_ptr() as *mut int32_t,
        tx_type,
        TX_64X64 as core::ffi::c_int as TX_SIZE,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_inv_txfm2d_add_64x32_c(
    mut input: *const int32_t,
    mut output: *mut uint16_t,
    mut stride: core::ffi::c_int,
    mut tx_type: TX_TYPE,
    mut bd: core::ffi::c_int,
) {
    let mut mod_input: [int32_t; 2048] = [0; 2048];
    memcpy(
        mod_input.as_mut_ptr() as *mut core::ffi::c_void,
        input as *const core::ffi::c_void,
        ((32 as core::ffi::c_int * 32 as core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<int32_t>() as size_t),
    );
    memset(
        mod_input
            .as_mut_ptr()
            .offset((32 as core::ffi::c_int * 32 as core::ffi::c_int) as isize)
            as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ((32 as core::ffi::c_int * 32 as core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<int32_t>() as size_t),
    );
    let mut txfm_buf: crate::platform::Aligned32<[core::ffi::c_int; 2176]> = crate::platform::Aligned32([0; 2176]);
    inv_txfm2d_add_facade(
        mod_input.as_mut_ptr(),
        output,
        stride,
        txfm_buf.0.as_mut_ptr() as *mut int32_t,
        tx_type,
        TX_64X32 as core::ffi::c_int as TX_SIZE,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_inv_txfm2d_add_32x64_c(
    mut input: *const int32_t,
    mut output: *mut uint16_t,
    mut stride: core::ffi::c_int,
    mut tx_type: TX_TYPE,
    mut bd: core::ffi::c_int,
) {
    let mut mod_input: [int32_t; 2048] = [0; 2048];
    let mut col: core::ffi::c_int = 0 as core::ffi::c_int;
    while col < 32 as core::ffi::c_int {
        memcpy(
            mod_input.as_mut_ptr().offset((col * 64 as core::ffi::c_int) as isize)
                as *mut core::ffi::c_void,
            input.offset((col * 32 as core::ffi::c_int) as isize)
                as *const core::ffi::c_void,
            (32 as size_t).wrapping_mul(::core::mem::size_of::<int32_t>() as size_t),
        );
        memset(
            mod_input
                .as_mut_ptr()
                .offset((col * 64 as core::ffi::c_int) as isize)
                .offset(32 as core::ffi::c_int as isize) as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (32 as size_t).wrapping_mul(::core::mem::size_of::<int32_t>() as size_t),
        );
        col += 1;
    }
    let mut txfm_buf: crate::platform::Aligned32<[core::ffi::c_int; 2176]> = crate::platform::Aligned32([0; 2176]);
    inv_txfm2d_add_facade(
        mod_input.as_mut_ptr(),
        output,
        stride,
        txfm_buf.0.as_mut_ptr() as *mut int32_t,
        tx_type,
        TX_32X64 as core::ffi::c_int as TX_SIZE,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_inv_txfm2d_add_16x64_c(
    mut input: *const int32_t,
    mut output: *mut uint16_t,
    mut stride: core::ffi::c_int,
    mut tx_type: TX_TYPE,
    mut bd: core::ffi::c_int,
) {
    let mut mod_input: [int32_t; 1024] = [0; 1024];
    let mut col: core::ffi::c_int = 0 as core::ffi::c_int;
    while col < 16 as core::ffi::c_int {
        memcpy(
            mod_input.as_mut_ptr().offset((col * 64 as core::ffi::c_int) as isize)
                as *mut core::ffi::c_void,
            input.offset((col * 32 as core::ffi::c_int) as isize)
                as *const core::ffi::c_void,
            (32 as size_t).wrapping_mul(::core::mem::size_of::<int32_t>() as size_t),
        );
        memset(
            mod_input
                .as_mut_ptr()
                .offset((col * 64 as core::ffi::c_int) as isize)
                .offset(32 as core::ffi::c_int as isize) as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (32 as size_t).wrapping_mul(::core::mem::size_of::<int32_t>() as size_t),
        );
        col += 1;
    }
    let mut txfm_buf: crate::platform::Aligned32<[core::ffi::c_int; 1152]> = crate::platform::Aligned32([0; 1152]);
    inv_txfm2d_add_facade(
        mod_input.as_mut_ptr(),
        output,
        stride,
        txfm_buf.0.as_mut_ptr() as *mut int32_t,
        tx_type,
        TX_16X64 as core::ffi::c_int as TX_SIZE,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_inv_txfm2d_add_64x16_c(
    mut input: *const int32_t,
    mut output: *mut uint16_t,
    mut stride: core::ffi::c_int,
    mut tx_type: TX_TYPE,
    mut bd: core::ffi::c_int,
) {
    let mut mod_input: [int32_t; 1024] = [0; 1024];
    memcpy(
        mod_input.as_mut_ptr() as *mut core::ffi::c_void,
        input as *const core::ffi::c_void,
        ((16 as core::ffi::c_int * 32 as core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<int32_t>() as size_t),
    );
    memset(
        mod_input
            .as_mut_ptr()
            .offset((16 as core::ffi::c_int * 32 as core::ffi::c_int) as isize)
            as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ((16 as core::ffi::c_int * 32 as core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<int32_t>() as size_t),
    );
    let mut txfm_buf: crate::platform::Aligned32<[core::ffi::c_int; 1152]> = crate::platform::Aligned32([0; 1152]);
    inv_txfm2d_add_facade(
        mod_input.as_mut_ptr(),
        output,
        stride,
        txfm_buf.0.as_mut_ptr() as *mut int32_t,
        tx_type,
        TX_64X16 as core::ffi::c_int as TX_SIZE,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_inv_txfm2d_add_4x16_c(
    mut input: *const int32_t,
    mut output: *mut uint16_t,
    mut stride: core::ffi::c_int,
    mut tx_type: TX_TYPE,
    mut bd: core::ffi::c_int,
) {
    let mut txfm_buf: crate::platform::Aligned32<[core::ffi::c_int; 96]> = crate::platform::Aligned32([0; 96]);
    inv_txfm2d_add_facade(
        input,
        output,
        stride,
        txfm_buf.0.as_mut_ptr() as *mut int32_t,
        tx_type,
        TX_4X16 as core::ffi::c_int as TX_SIZE,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_inv_txfm2d_add_16x4_c(
    mut input: *const int32_t,
    mut output: *mut uint16_t,
    mut stride: core::ffi::c_int,
    mut tx_type: TX_TYPE,
    mut bd: core::ffi::c_int,
) {
    let mut txfm_buf: crate::platform::Aligned32<[core::ffi::c_int; 96]> = crate::platform::Aligned32([0; 96]);
    inv_txfm2d_add_facade(
        input,
        output,
        stride,
        txfm_buf.0.as_mut_ptr() as *mut int32_t,
        tx_type,
        TX_16X4 as core::ffi::c_int as TX_SIZE,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_inv_txfm2d_add_8x32_c(
    mut input: *const int32_t,
    mut output: *mut uint16_t,
    mut stride: core::ffi::c_int,
    mut tx_type: TX_TYPE,
    mut bd: core::ffi::c_int,
) {
    let mut txfm_buf: crate::platform::Aligned32<[core::ffi::c_int; 320]> = crate::platform::Aligned32([0; 320]);
    inv_txfm2d_add_facade(
        input,
        output,
        stride,
        txfm_buf.0.as_mut_ptr() as *mut int32_t,
        tx_type,
        TX_8X32 as core::ffi::c_int as TX_SIZE,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_inv_txfm2d_add_32x8_c(
    mut input: *const int32_t,
    mut output: *mut uint16_t,
    mut stride: core::ffi::c_int,
    mut tx_type: TX_TYPE,
    mut bd: core::ffi::c_int,
) {
    let mut txfm_buf: crate::platform::Aligned32<[core::ffi::c_int; 320]> = crate::platform::Aligned32([0; 320]);
    inv_txfm2d_add_facade(
        input,
        output,
        stride,
        txfm_buf.0.as_mut_ptr() as *mut int32_t,
        tx_type,
        TX_32X8 as core::ffi::c_int as TX_SIZE,
        bd,
    );
}
#[inline]
unsafe extern "C-unwind" fn clamp_value(mut value: int32_t, mut bit: int8_t) -> int32_t {
    if bit as core::ffi::c_int <= 0 as core::ffi::c_int {
        return value;
    }
    let max_value: int64_t = (((1 as core::ffi::c_longlong)
        << bit as core::ffi::c_int - 1 as core::ffi::c_int) - 1 as core::ffi::c_longlong)
        as int64_t;
    let min_value: int64_t = -((1 as core::ffi::c_longlong)
        << bit as core::ffi::c_int - 1 as core::ffi::c_int) as int64_t;
    return clamp64(value as int64_t, min_value, max_value) as int32_t;
}
#[inline]
unsafe extern "C-unwind" fn clamp_buf(
    mut buf: *mut int32_t,
    mut size: int32_t,
    mut bit: int8_t,
) {
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while (i as int32_t) < size {
        *buf.offset(i as isize) = clamp_value(*buf.offset(i as isize), bit);
        i += 1;
    }
}
static mut inv_start_range: [int8_t; 19] = [
    5 as core::ffi::c_int as int8_t,
    6 as core::ffi::c_int as int8_t,
    7 as core::ffi::c_int as int8_t,
    7 as core::ffi::c_int as int8_t,
    7 as core::ffi::c_int as int8_t,
    5 as core::ffi::c_int as int8_t,
    5 as core::ffi::c_int as int8_t,
    6 as core::ffi::c_int as int8_t,
    6 as core::ffi::c_int as int8_t,
    6 as core::ffi::c_int as int8_t,
    6 as core::ffi::c_int as int8_t,
    6 as core::ffi::c_int as int8_t,
    6 as core::ffi::c_int as int8_t,
    6 as core::ffi::c_int as int8_t,
    6 as core::ffi::c_int as int8_t,
    7 as core::ffi::c_int as int8_t,
    7 as core::ffi::c_int as int8_t,
    7 as core::ffi::c_int as int8_t,
    7 as core::ffi::c_int as int8_t,
];
pub const INV_COS_BIT: core::ffi::c_int = 12 as core::ffi::c_int;

extern "C" {
    fn memcpy(
        __dest: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn memset(
        __s: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn __assert_fail(
        __assertion: *const core::ffi::c_char,
        __file: *const core::ffi::c_char,
        __line: core::ffi::c_uint,
        __function: *const core::ffi::c_char,
    ) -> !;
    fn abs(__x: core::ffi::c_int) -> core::ffi::c_int;
}
