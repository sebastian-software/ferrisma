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

// Mechanically translated from upstream/aom/av1/common/warped_motion.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C-unwind" {
}
pub type __int8_t = i8;
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type __uint64_t = u64;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type uintptr_t = usize;
pub type BLOCK_SIZE = core::ffi::c_uchar;
pub const BLOCK_LARGEST: BLOCK_SIZE = 15;
pub const BLOCK_INVALID: BLOCK_SIZE = 255;
pub const BLOCK_SIZES: BLOCK_SIZE = 16;
pub const BLOCK_SIZES_ALL: BLOCK_SIZE = 22;
pub const BLOCK_64X16: BLOCK_SIZE = 21;
pub const BLOCK_16X64: BLOCK_SIZE = 20;
pub const BLOCK_32X8: BLOCK_SIZE = 19;
pub const BLOCK_8X32: BLOCK_SIZE = 18;
pub const BLOCK_16X4: BLOCK_SIZE = 17;
pub const BLOCK_4X16: BLOCK_SIZE = 16;
pub const BLOCK_128X128: BLOCK_SIZE = 15;
pub const BLOCK_128X64: BLOCK_SIZE = 14;
pub const BLOCK_64X128: BLOCK_SIZE = 13;
pub const BLOCK_64X64: BLOCK_SIZE = 12;
pub const BLOCK_64X32: BLOCK_SIZE = 11;
pub const BLOCK_32X64: BLOCK_SIZE = 10;
pub const BLOCK_32X32: BLOCK_SIZE = 9;
pub const BLOCK_32X16: BLOCK_SIZE = 8;
pub const BLOCK_16X32: BLOCK_SIZE = 7;
pub const BLOCK_16X16: BLOCK_SIZE = 6;
pub const BLOCK_16X8: BLOCK_SIZE = 5;
pub const BLOCK_8X16: BLOCK_SIZE = 4;
pub const BLOCK_8X8: BLOCK_SIZE = 3;
pub const BLOCK_8X4: BLOCK_SIZE = 2;
pub const BLOCK_4X8: BLOCK_SIZE = 1;
pub const BLOCK_4X4: BLOCK_SIZE = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WarpedMotionParams {
    pub wmmat: [int32_t; 6],
    pub alpha: int16_t,
    pub beta: int16_t,
    pub gamma: int16_t,
    pub delta: int16_t,
    pub wmtype: TransformationType,
    pub invalid: int8_t,
}
pub type TransformationType = uint8_t;
pub type CONV_BUF_TYPE = uint16_t;
pub type C2RustUnnamed = core::ffi::c_uint;
pub const TRANS_TYPES: C2RustUnnamed = 4;
pub const AFFINE: C2RustUnnamed = 3;
pub const ROTZOOM: C2RustUnnamed = 2;
pub const TRANSLATION: C2RustUnnamed = 1;
pub const IDENTITY: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ConvolveParams {
    pub do_average: core::ffi::c_int,
    pub dst: *mut CONV_BUF_TYPE,
    pub dst_stride: core::ffi::c_int,
    pub round_0: core::ffi::c_int,
    pub round_1: core::ffi::c_int,
    pub plane: core::ffi::c_int,
    pub is_compound: core::ffi::c_int,
    pub use_dist_wtd_comp_avg: core::ffi::c_int,
    pub fwd_offset: core::ffi::c_int,
    pub bck_offset: core::ffi::c_int,
}
pub type WarpedFilterCoeff = int8_t;
pub const __ASSERT_FUNCTION: [core::ffi::c_char; 26] = unsafe {
    ::core::mem::transmute::<
        [u8; 26],
        [core::ffi::c_char; 26],
    >(*b"int get_msb(unsigned int)\0")
};
pub const INT16_MIN: core::ffi::c_int = -(32767 as core::ffi::c_int)
    - 1 as core::ffi::c_int;
pub const INT16_MAX: core::ffi::c_int = 32767 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn clip_pixel(mut val: core::ffi::c_int) -> uint8_t {
    return (if val > 255 as core::ffi::c_int {
        255 as core::ffi::c_int
    } else if val < 0 as core::ffi::c_int {
        0 as core::ffi::c_int
    } else {
        val
    }) as uint8_t;
}
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
#[inline]
unsafe extern "C-unwind" fn get_msb(mut n: core::ffi::c_uint) -> core::ffi::c_int {
    if n != 0 as core::ffi::c_uint {} else {
        __assert_fail(
            b"n != 0\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/aom_ports/bitops.h\0" as *const u8
                as *const core::ffi::c_char,
            43 as core::ffi::c_uint,
            __ASSERT_FUNCTION.as_ptr(),
        );
    };
    return 31 as core::ffi::c_int ^ n.leading_zeros() as i32;
}
pub const MI_SIZE_LOG2: core::ffi::c_int = 2 as core::ffi::c_int;
pub const MI_SIZE: core::ffi::c_int = (1 as core::ffi::c_int) << MI_SIZE_LOG2;
pub const DIST_PRECISION_BITS: core::ffi::c_int = 4 as core::ffi::c_int;
#[no_mangle]
pub static mut av1_warped_filter: [[WarpedFilterCoeff; 8]; 193] = [
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        127 as core::ffi::c_int as WarpedFilterCoeff,
        1 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        -(1 as core::ffi::c_int) as WarpedFilterCoeff,
        127 as core::ffi::c_int as WarpedFilterCoeff,
        2 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        1 as core::ffi::c_int as WarpedFilterCoeff,
        -(3 as core::ffi::c_int) as WarpedFilterCoeff,
        127 as core::ffi::c_int as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
        -(1 as core::ffi::c_int) as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        1 as core::ffi::c_int as WarpedFilterCoeff,
        -(4 as core::ffi::c_int) as WarpedFilterCoeff,
        126 as core::ffi::c_int as WarpedFilterCoeff,
        6 as core::ffi::c_int as WarpedFilterCoeff,
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
        1 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        1 as core::ffi::c_int as WarpedFilterCoeff,
        -(5 as core::ffi::c_int) as WarpedFilterCoeff,
        126 as core::ffi::c_int as WarpedFilterCoeff,
        8 as core::ffi::c_int as WarpedFilterCoeff,
        -(3 as core::ffi::c_int) as WarpedFilterCoeff,
        1 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        1 as core::ffi::c_int as WarpedFilterCoeff,
        -(6 as core::ffi::c_int) as WarpedFilterCoeff,
        125 as core::ffi::c_int as WarpedFilterCoeff,
        11 as core::ffi::c_int as WarpedFilterCoeff,
        -(4 as core::ffi::c_int) as WarpedFilterCoeff,
        1 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        1 as core::ffi::c_int as WarpedFilterCoeff,
        -(7 as core::ffi::c_int) as WarpedFilterCoeff,
        124 as core::ffi::c_int as WarpedFilterCoeff,
        13 as core::ffi::c_int as WarpedFilterCoeff,
        -(4 as core::ffi::c_int) as WarpedFilterCoeff,
        1 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        2 as core::ffi::c_int as WarpedFilterCoeff,
        -(8 as core::ffi::c_int) as WarpedFilterCoeff,
        123 as core::ffi::c_int as WarpedFilterCoeff,
        15 as core::ffi::c_int as WarpedFilterCoeff,
        -(5 as core::ffi::c_int) as WarpedFilterCoeff,
        1 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        2 as core::ffi::c_int as WarpedFilterCoeff,
        -(9 as core::ffi::c_int) as WarpedFilterCoeff,
        122 as core::ffi::c_int as WarpedFilterCoeff,
        18 as core::ffi::c_int as WarpedFilterCoeff,
        -(6 as core::ffi::c_int) as WarpedFilterCoeff,
        1 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        2 as core::ffi::c_int as WarpedFilterCoeff,
        -(10 as core::ffi::c_int) as WarpedFilterCoeff,
        121 as core::ffi::c_int as WarpedFilterCoeff,
        20 as core::ffi::c_int as WarpedFilterCoeff,
        -(6 as core::ffi::c_int) as WarpedFilterCoeff,
        1 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        2 as core::ffi::c_int as WarpedFilterCoeff,
        -(11 as core::ffi::c_int) as WarpedFilterCoeff,
        120 as core::ffi::c_int as WarpedFilterCoeff,
        22 as core::ffi::c_int as WarpedFilterCoeff,
        -(7 as core::ffi::c_int) as WarpedFilterCoeff,
        2 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        2 as core::ffi::c_int as WarpedFilterCoeff,
        -(12 as core::ffi::c_int) as WarpedFilterCoeff,
        119 as core::ffi::c_int as WarpedFilterCoeff,
        25 as core::ffi::c_int as WarpedFilterCoeff,
        -(8 as core::ffi::c_int) as WarpedFilterCoeff,
        2 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        3 as core::ffi::c_int as WarpedFilterCoeff,
        -(13 as core::ffi::c_int) as WarpedFilterCoeff,
        117 as core::ffi::c_int as WarpedFilterCoeff,
        27 as core::ffi::c_int as WarpedFilterCoeff,
        -(8 as core::ffi::c_int) as WarpedFilterCoeff,
        2 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        3 as core::ffi::c_int as WarpedFilterCoeff,
        -(13 as core::ffi::c_int) as WarpedFilterCoeff,
        116 as core::ffi::c_int as WarpedFilterCoeff,
        29 as core::ffi::c_int as WarpedFilterCoeff,
        -(9 as core::ffi::c_int) as WarpedFilterCoeff,
        2 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        3 as core::ffi::c_int as WarpedFilterCoeff,
        -(14 as core::ffi::c_int) as WarpedFilterCoeff,
        114 as core::ffi::c_int as WarpedFilterCoeff,
        32 as core::ffi::c_int as WarpedFilterCoeff,
        -(10 as core::ffi::c_int) as WarpedFilterCoeff,
        3 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        3 as core::ffi::c_int as WarpedFilterCoeff,
        -(15 as core::ffi::c_int) as WarpedFilterCoeff,
        113 as core::ffi::c_int as WarpedFilterCoeff,
        35 as core::ffi::c_int as WarpedFilterCoeff,
        -(10 as core::ffi::c_int) as WarpedFilterCoeff,
        2 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        3 as core::ffi::c_int as WarpedFilterCoeff,
        -(15 as core::ffi::c_int) as WarpedFilterCoeff,
        111 as core::ffi::c_int as WarpedFilterCoeff,
        37 as core::ffi::c_int as WarpedFilterCoeff,
        -(11 as core::ffi::c_int) as WarpedFilterCoeff,
        3 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        3 as core::ffi::c_int as WarpedFilterCoeff,
        -(16 as core::ffi::c_int) as WarpedFilterCoeff,
        109 as core::ffi::c_int as WarpedFilterCoeff,
        40 as core::ffi::c_int as WarpedFilterCoeff,
        -(11 as core::ffi::c_int) as WarpedFilterCoeff,
        3 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        3 as core::ffi::c_int as WarpedFilterCoeff,
        -(16 as core::ffi::c_int) as WarpedFilterCoeff,
        108 as core::ffi::c_int as WarpedFilterCoeff,
        42 as core::ffi::c_int as WarpedFilterCoeff,
        -(12 as core::ffi::c_int) as WarpedFilterCoeff,
        3 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        4 as core::ffi::c_int as WarpedFilterCoeff,
        -(17 as core::ffi::c_int) as WarpedFilterCoeff,
        106 as core::ffi::c_int as WarpedFilterCoeff,
        45 as core::ffi::c_int as WarpedFilterCoeff,
        -(13 as core::ffi::c_int) as WarpedFilterCoeff,
        3 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        4 as core::ffi::c_int as WarpedFilterCoeff,
        -(17 as core::ffi::c_int) as WarpedFilterCoeff,
        104 as core::ffi::c_int as WarpedFilterCoeff,
        47 as core::ffi::c_int as WarpedFilterCoeff,
        -(13 as core::ffi::c_int) as WarpedFilterCoeff,
        3 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        4 as core::ffi::c_int as WarpedFilterCoeff,
        -(17 as core::ffi::c_int) as WarpedFilterCoeff,
        102 as core::ffi::c_int as WarpedFilterCoeff,
        50 as core::ffi::c_int as WarpedFilterCoeff,
        -(14 as core::ffi::c_int) as WarpedFilterCoeff,
        3 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        4 as core::ffi::c_int as WarpedFilterCoeff,
        -(17 as core::ffi::c_int) as WarpedFilterCoeff,
        100 as core::ffi::c_int as WarpedFilterCoeff,
        52 as core::ffi::c_int as WarpedFilterCoeff,
        -(14 as core::ffi::c_int) as WarpedFilterCoeff,
        3 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        4 as core::ffi::c_int as WarpedFilterCoeff,
        -(18 as core::ffi::c_int) as WarpedFilterCoeff,
        98 as core::ffi::c_int as WarpedFilterCoeff,
        55 as core::ffi::c_int as WarpedFilterCoeff,
        -(15 as core::ffi::c_int) as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        4 as core::ffi::c_int as WarpedFilterCoeff,
        -(18 as core::ffi::c_int) as WarpedFilterCoeff,
        96 as core::ffi::c_int as WarpedFilterCoeff,
        58 as core::ffi::c_int as WarpedFilterCoeff,
        -(15 as core::ffi::c_int) as WarpedFilterCoeff,
        3 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        4 as core::ffi::c_int as WarpedFilterCoeff,
        -(18 as core::ffi::c_int) as WarpedFilterCoeff,
        94 as core::ffi::c_int as WarpedFilterCoeff,
        60 as core::ffi::c_int as WarpedFilterCoeff,
        -(16 as core::ffi::c_int) as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        4 as core::ffi::c_int as WarpedFilterCoeff,
        -(18 as core::ffi::c_int) as WarpedFilterCoeff,
        91 as core::ffi::c_int as WarpedFilterCoeff,
        63 as core::ffi::c_int as WarpedFilterCoeff,
        -(16 as core::ffi::c_int) as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        4 as core::ffi::c_int as WarpedFilterCoeff,
        -(18 as core::ffi::c_int) as WarpedFilterCoeff,
        89 as core::ffi::c_int as WarpedFilterCoeff,
        65 as core::ffi::c_int as WarpedFilterCoeff,
        -(16 as core::ffi::c_int) as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        4 as core::ffi::c_int as WarpedFilterCoeff,
        -(18 as core::ffi::c_int) as WarpedFilterCoeff,
        87 as core::ffi::c_int as WarpedFilterCoeff,
        68 as core::ffi::c_int as WarpedFilterCoeff,
        -(17 as core::ffi::c_int) as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        4 as core::ffi::c_int as WarpedFilterCoeff,
        -(18 as core::ffi::c_int) as WarpedFilterCoeff,
        85 as core::ffi::c_int as WarpedFilterCoeff,
        70 as core::ffi::c_int as WarpedFilterCoeff,
        -(17 as core::ffi::c_int) as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        4 as core::ffi::c_int as WarpedFilterCoeff,
        -(18 as core::ffi::c_int) as WarpedFilterCoeff,
        82 as core::ffi::c_int as WarpedFilterCoeff,
        73 as core::ffi::c_int as WarpedFilterCoeff,
        -(17 as core::ffi::c_int) as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        4 as core::ffi::c_int as WarpedFilterCoeff,
        -(18 as core::ffi::c_int) as WarpedFilterCoeff,
        80 as core::ffi::c_int as WarpedFilterCoeff,
        75 as core::ffi::c_int as WarpedFilterCoeff,
        -(17 as core::ffi::c_int) as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        4 as core::ffi::c_int as WarpedFilterCoeff,
        -(18 as core::ffi::c_int) as WarpedFilterCoeff,
        78 as core::ffi::c_int as WarpedFilterCoeff,
        78 as core::ffi::c_int as WarpedFilterCoeff,
        -(18 as core::ffi::c_int) as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        4 as core::ffi::c_int as WarpedFilterCoeff,
        -(17 as core::ffi::c_int) as WarpedFilterCoeff,
        75 as core::ffi::c_int as WarpedFilterCoeff,
        80 as core::ffi::c_int as WarpedFilterCoeff,
        -(18 as core::ffi::c_int) as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        4 as core::ffi::c_int as WarpedFilterCoeff,
        -(17 as core::ffi::c_int) as WarpedFilterCoeff,
        73 as core::ffi::c_int as WarpedFilterCoeff,
        82 as core::ffi::c_int as WarpedFilterCoeff,
        -(18 as core::ffi::c_int) as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        4 as core::ffi::c_int as WarpedFilterCoeff,
        -(17 as core::ffi::c_int) as WarpedFilterCoeff,
        70 as core::ffi::c_int as WarpedFilterCoeff,
        85 as core::ffi::c_int as WarpedFilterCoeff,
        -(18 as core::ffi::c_int) as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        4 as core::ffi::c_int as WarpedFilterCoeff,
        -(17 as core::ffi::c_int) as WarpedFilterCoeff,
        68 as core::ffi::c_int as WarpedFilterCoeff,
        87 as core::ffi::c_int as WarpedFilterCoeff,
        -(18 as core::ffi::c_int) as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        4 as core::ffi::c_int as WarpedFilterCoeff,
        -(16 as core::ffi::c_int) as WarpedFilterCoeff,
        65 as core::ffi::c_int as WarpedFilterCoeff,
        89 as core::ffi::c_int as WarpedFilterCoeff,
        -(18 as core::ffi::c_int) as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        4 as core::ffi::c_int as WarpedFilterCoeff,
        -(16 as core::ffi::c_int) as WarpedFilterCoeff,
        63 as core::ffi::c_int as WarpedFilterCoeff,
        91 as core::ffi::c_int as WarpedFilterCoeff,
        -(18 as core::ffi::c_int) as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        4 as core::ffi::c_int as WarpedFilterCoeff,
        -(16 as core::ffi::c_int) as WarpedFilterCoeff,
        60 as core::ffi::c_int as WarpedFilterCoeff,
        94 as core::ffi::c_int as WarpedFilterCoeff,
        -(18 as core::ffi::c_int) as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        3 as core::ffi::c_int as WarpedFilterCoeff,
        -(15 as core::ffi::c_int) as WarpedFilterCoeff,
        58 as core::ffi::c_int as WarpedFilterCoeff,
        96 as core::ffi::c_int as WarpedFilterCoeff,
        -(18 as core::ffi::c_int) as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        4 as core::ffi::c_int as WarpedFilterCoeff,
        -(15 as core::ffi::c_int) as WarpedFilterCoeff,
        55 as core::ffi::c_int as WarpedFilterCoeff,
        98 as core::ffi::c_int as WarpedFilterCoeff,
        -(18 as core::ffi::c_int) as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        3 as core::ffi::c_int as WarpedFilterCoeff,
        -(14 as core::ffi::c_int) as WarpedFilterCoeff,
        52 as core::ffi::c_int as WarpedFilterCoeff,
        100 as core::ffi::c_int as WarpedFilterCoeff,
        -(17 as core::ffi::c_int) as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        3 as core::ffi::c_int as WarpedFilterCoeff,
        -(14 as core::ffi::c_int) as WarpedFilterCoeff,
        50 as core::ffi::c_int as WarpedFilterCoeff,
        102 as core::ffi::c_int as WarpedFilterCoeff,
        -(17 as core::ffi::c_int) as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        3 as core::ffi::c_int as WarpedFilterCoeff,
        -(13 as core::ffi::c_int) as WarpedFilterCoeff,
        47 as core::ffi::c_int as WarpedFilterCoeff,
        104 as core::ffi::c_int as WarpedFilterCoeff,
        -(17 as core::ffi::c_int) as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        3 as core::ffi::c_int as WarpedFilterCoeff,
        -(13 as core::ffi::c_int) as WarpedFilterCoeff,
        45 as core::ffi::c_int as WarpedFilterCoeff,
        106 as core::ffi::c_int as WarpedFilterCoeff,
        -(17 as core::ffi::c_int) as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        3 as core::ffi::c_int as WarpedFilterCoeff,
        -(12 as core::ffi::c_int) as WarpedFilterCoeff,
        42 as core::ffi::c_int as WarpedFilterCoeff,
        108 as core::ffi::c_int as WarpedFilterCoeff,
        -(16 as core::ffi::c_int) as WarpedFilterCoeff,
        3 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        3 as core::ffi::c_int as WarpedFilterCoeff,
        -(11 as core::ffi::c_int) as WarpedFilterCoeff,
        40 as core::ffi::c_int as WarpedFilterCoeff,
        109 as core::ffi::c_int as WarpedFilterCoeff,
        -(16 as core::ffi::c_int) as WarpedFilterCoeff,
        3 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        3 as core::ffi::c_int as WarpedFilterCoeff,
        -(11 as core::ffi::c_int) as WarpedFilterCoeff,
        37 as core::ffi::c_int as WarpedFilterCoeff,
        111 as core::ffi::c_int as WarpedFilterCoeff,
        -(15 as core::ffi::c_int) as WarpedFilterCoeff,
        3 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        2 as core::ffi::c_int as WarpedFilterCoeff,
        -(10 as core::ffi::c_int) as WarpedFilterCoeff,
        35 as core::ffi::c_int as WarpedFilterCoeff,
        113 as core::ffi::c_int as WarpedFilterCoeff,
        -(15 as core::ffi::c_int) as WarpedFilterCoeff,
        3 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        3 as core::ffi::c_int as WarpedFilterCoeff,
        -(10 as core::ffi::c_int) as WarpedFilterCoeff,
        32 as core::ffi::c_int as WarpedFilterCoeff,
        114 as core::ffi::c_int as WarpedFilterCoeff,
        -(14 as core::ffi::c_int) as WarpedFilterCoeff,
        3 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        2 as core::ffi::c_int as WarpedFilterCoeff,
        -(9 as core::ffi::c_int) as WarpedFilterCoeff,
        29 as core::ffi::c_int as WarpedFilterCoeff,
        116 as core::ffi::c_int as WarpedFilterCoeff,
        -(13 as core::ffi::c_int) as WarpedFilterCoeff,
        3 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        2 as core::ffi::c_int as WarpedFilterCoeff,
        -(8 as core::ffi::c_int) as WarpedFilterCoeff,
        27 as core::ffi::c_int as WarpedFilterCoeff,
        117 as core::ffi::c_int as WarpedFilterCoeff,
        -(13 as core::ffi::c_int) as WarpedFilterCoeff,
        3 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        2 as core::ffi::c_int as WarpedFilterCoeff,
        -(8 as core::ffi::c_int) as WarpedFilterCoeff,
        25 as core::ffi::c_int as WarpedFilterCoeff,
        119 as core::ffi::c_int as WarpedFilterCoeff,
        -(12 as core::ffi::c_int) as WarpedFilterCoeff,
        2 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        2 as core::ffi::c_int as WarpedFilterCoeff,
        -(7 as core::ffi::c_int) as WarpedFilterCoeff,
        22 as core::ffi::c_int as WarpedFilterCoeff,
        120 as core::ffi::c_int as WarpedFilterCoeff,
        -(11 as core::ffi::c_int) as WarpedFilterCoeff,
        2 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        1 as core::ffi::c_int as WarpedFilterCoeff,
        -(6 as core::ffi::c_int) as WarpedFilterCoeff,
        20 as core::ffi::c_int as WarpedFilterCoeff,
        121 as core::ffi::c_int as WarpedFilterCoeff,
        -(10 as core::ffi::c_int) as WarpedFilterCoeff,
        2 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        1 as core::ffi::c_int as WarpedFilterCoeff,
        -(6 as core::ffi::c_int) as WarpedFilterCoeff,
        18 as core::ffi::c_int as WarpedFilterCoeff,
        122 as core::ffi::c_int as WarpedFilterCoeff,
        -(9 as core::ffi::c_int) as WarpedFilterCoeff,
        2 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        1 as core::ffi::c_int as WarpedFilterCoeff,
        -(5 as core::ffi::c_int) as WarpedFilterCoeff,
        15 as core::ffi::c_int as WarpedFilterCoeff,
        123 as core::ffi::c_int as WarpedFilterCoeff,
        -(8 as core::ffi::c_int) as WarpedFilterCoeff,
        2 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        1 as core::ffi::c_int as WarpedFilterCoeff,
        -(4 as core::ffi::c_int) as WarpedFilterCoeff,
        13 as core::ffi::c_int as WarpedFilterCoeff,
        124 as core::ffi::c_int as WarpedFilterCoeff,
        -(7 as core::ffi::c_int) as WarpedFilterCoeff,
        1 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        1 as core::ffi::c_int as WarpedFilterCoeff,
        -(4 as core::ffi::c_int) as WarpedFilterCoeff,
        11 as core::ffi::c_int as WarpedFilterCoeff,
        125 as core::ffi::c_int as WarpedFilterCoeff,
        -(6 as core::ffi::c_int) as WarpedFilterCoeff,
        1 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        1 as core::ffi::c_int as WarpedFilterCoeff,
        -(3 as core::ffi::c_int) as WarpedFilterCoeff,
        8 as core::ffi::c_int as WarpedFilterCoeff,
        126 as core::ffi::c_int as WarpedFilterCoeff,
        -(5 as core::ffi::c_int) as WarpedFilterCoeff,
        1 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        1 as core::ffi::c_int as WarpedFilterCoeff,
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
        6 as core::ffi::c_int as WarpedFilterCoeff,
        126 as core::ffi::c_int as WarpedFilterCoeff,
        -(4 as core::ffi::c_int) as WarpedFilterCoeff,
        1 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        -(1 as core::ffi::c_int) as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
        127 as core::ffi::c_int as WarpedFilterCoeff,
        -(3 as core::ffi::c_int) as WarpedFilterCoeff,
        1 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        2 as core::ffi::c_int as WarpedFilterCoeff,
        127 as core::ffi::c_int as WarpedFilterCoeff,
        -(1 as core::ffi::c_int) as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        127 as core::ffi::c_int as WarpedFilterCoeff,
        1 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        -(1 as core::ffi::c_int) as WarpedFilterCoeff,
        127 as core::ffi::c_int as WarpedFilterCoeff,
        2 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        1 as core::ffi::c_int as WarpedFilterCoeff,
        -(3 as core::ffi::c_int) as WarpedFilterCoeff,
        127 as core::ffi::c_int as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
        1 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        1 as core::ffi::c_int as WarpedFilterCoeff,
        -(5 as core::ffi::c_int) as WarpedFilterCoeff,
        127 as core::ffi::c_int as WarpedFilterCoeff,
        6 as core::ffi::c_int as WarpedFilterCoeff,
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
        1 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        2 as core::ffi::c_int as WarpedFilterCoeff,
        -(6 as core::ffi::c_int) as WarpedFilterCoeff,
        126 as core::ffi::c_int as WarpedFilterCoeff,
        8 as core::ffi::c_int as WarpedFilterCoeff,
        -(3 as core::ffi::c_int) as WarpedFilterCoeff,
        1 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        -(1 as core::ffi::c_int) as WarpedFilterCoeff,
        2 as core::ffi::c_int as WarpedFilterCoeff,
        -(7 as core::ffi::c_int) as WarpedFilterCoeff,
        126 as core::ffi::c_int as WarpedFilterCoeff,
        11 as core::ffi::c_int as WarpedFilterCoeff,
        -(4 as core::ffi::c_int) as WarpedFilterCoeff,
        2 as core::ffi::c_int as WarpedFilterCoeff,
        -(1 as core::ffi::c_int) as WarpedFilterCoeff,
    ],
    [
        -(1 as core::ffi::c_int) as WarpedFilterCoeff,
        3 as core::ffi::c_int as WarpedFilterCoeff,
        -(8 as core::ffi::c_int) as WarpedFilterCoeff,
        125 as core::ffi::c_int as WarpedFilterCoeff,
        13 as core::ffi::c_int as WarpedFilterCoeff,
        -(5 as core::ffi::c_int) as WarpedFilterCoeff,
        2 as core::ffi::c_int as WarpedFilterCoeff,
        -(1 as core::ffi::c_int) as WarpedFilterCoeff,
    ],
    [
        -(1 as core::ffi::c_int) as WarpedFilterCoeff,
        3 as core::ffi::c_int as WarpedFilterCoeff,
        -(10 as core::ffi::c_int) as WarpedFilterCoeff,
        124 as core::ffi::c_int as WarpedFilterCoeff,
        16 as core::ffi::c_int as WarpedFilterCoeff,
        -(6 as core::ffi::c_int) as WarpedFilterCoeff,
        3 as core::ffi::c_int as WarpedFilterCoeff,
        -(1 as core::ffi::c_int) as WarpedFilterCoeff,
    ],
    [
        -(1 as core::ffi::c_int) as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
        -(11 as core::ffi::c_int) as WarpedFilterCoeff,
        123 as core::ffi::c_int as WarpedFilterCoeff,
        18 as core::ffi::c_int as WarpedFilterCoeff,
        -(7 as core::ffi::c_int) as WarpedFilterCoeff,
        3 as core::ffi::c_int as WarpedFilterCoeff,
        -(1 as core::ffi::c_int) as WarpedFilterCoeff,
    ],
    [
        -(1 as core::ffi::c_int) as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
        -(12 as core::ffi::c_int) as WarpedFilterCoeff,
        122 as core::ffi::c_int as WarpedFilterCoeff,
        20 as core::ffi::c_int as WarpedFilterCoeff,
        -(7 as core::ffi::c_int) as WarpedFilterCoeff,
        3 as core::ffi::c_int as WarpedFilterCoeff,
        -(1 as core::ffi::c_int) as WarpedFilterCoeff,
    ],
    [
        -(1 as core::ffi::c_int) as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
        -(13 as core::ffi::c_int) as WarpedFilterCoeff,
        121 as core::ffi::c_int as WarpedFilterCoeff,
        23 as core::ffi::c_int as WarpedFilterCoeff,
        -(8 as core::ffi::c_int) as WarpedFilterCoeff,
        3 as core::ffi::c_int as WarpedFilterCoeff,
        -(1 as core::ffi::c_int) as WarpedFilterCoeff,
    ],
    [
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
        5 as core::ffi::c_int as WarpedFilterCoeff,
        -(14 as core::ffi::c_int) as WarpedFilterCoeff,
        120 as core::ffi::c_int as WarpedFilterCoeff,
        25 as core::ffi::c_int as WarpedFilterCoeff,
        -(9 as core::ffi::c_int) as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
        -(1 as core::ffi::c_int) as WarpedFilterCoeff,
    ],
    [
        -(1 as core::ffi::c_int) as WarpedFilterCoeff,
        5 as core::ffi::c_int as WarpedFilterCoeff,
        -(15 as core::ffi::c_int) as WarpedFilterCoeff,
        119 as core::ffi::c_int as WarpedFilterCoeff,
        27 as core::ffi::c_int as WarpedFilterCoeff,
        -(10 as core::ffi::c_int) as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
        -(1 as core::ffi::c_int) as WarpedFilterCoeff,
    ],
    [
        -(1 as core::ffi::c_int) as WarpedFilterCoeff,
        5 as core::ffi::c_int as WarpedFilterCoeff,
        -(16 as core::ffi::c_int) as WarpedFilterCoeff,
        118 as core::ffi::c_int as WarpedFilterCoeff,
        30 as core::ffi::c_int as WarpedFilterCoeff,
        -(11 as core::ffi::c_int) as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
        -(1 as core::ffi::c_int) as WarpedFilterCoeff,
    ],
    [
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
        6 as core::ffi::c_int as WarpedFilterCoeff,
        -(17 as core::ffi::c_int) as WarpedFilterCoeff,
        116 as core::ffi::c_int as WarpedFilterCoeff,
        33 as core::ffi::c_int as WarpedFilterCoeff,
        -(12 as core::ffi::c_int) as WarpedFilterCoeff,
        5 as core::ffi::c_int as WarpedFilterCoeff,
        -(1 as core::ffi::c_int) as WarpedFilterCoeff,
    ],
    [
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
        6 as core::ffi::c_int as WarpedFilterCoeff,
        -(17 as core::ffi::c_int) as WarpedFilterCoeff,
        114 as core::ffi::c_int as WarpedFilterCoeff,
        35 as core::ffi::c_int as WarpedFilterCoeff,
        -(12 as core::ffi::c_int) as WarpedFilterCoeff,
        5 as core::ffi::c_int as WarpedFilterCoeff,
        -(1 as core::ffi::c_int) as WarpedFilterCoeff,
    ],
    [
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
        6 as core::ffi::c_int as WarpedFilterCoeff,
        -(18 as core::ffi::c_int) as WarpedFilterCoeff,
        113 as core::ffi::c_int as WarpedFilterCoeff,
        38 as core::ffi::c_int as WarpedFilterCoeff,
        -(13 as core::ffi::c_int) as WarpedFilterCoeff,
        5 as core::ffi::c_int as WarpedFilterCoeff,
        -(1 as core::ffi::c_int) as WarpedFilterCoeff,
    ],
    [
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
        7 as core::ffi::c_int as WarpedFilterCoeff,
        -(19 as core::ffi::c_int) as WarpedFilterCoeff,
        111 as core::ffi::c_int as WarpedFilterCoeff,
        41 as core::ffi::c_int as WarpedFilterCoeff,
        -(14 as core::ffi::c_int) as WarpedFilterCoeff,
        6 as core::ffi::c_int as WarpedFilterCoeff,
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
    ],
    [
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
        7 as core::ffi::c_int as WarpedFilterCoeff,
        -(19 as core::ffi::c_int) as WarpedFilterCoeff,
        110 as core::ffi::c_int as WarpedFilterCoeff,
        43 as core::ffi::c_int as WarpedFilterCoeff,
        -(15 as core::ffi::c_int) as WarpedFilterCoeff,
        6 as core::ffi::c_int as WarpedFilterCoeff,
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
    ],
    [
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
        7 as core::ffi::c_int as WarpedFilterCoeff,
        -(20 as core::ffi::c_int) as WarpedFilterCoeff,
        108 as core::ffi::c_int as WarpedFilterCoeff,
        46 as core::ffi::c_int as WarpedFilterCoeff,
        -(15 as core::ffi::c_int) as WarpedFilterCoeff,
        6 as core::ffi::c_int as WarpedFilterCoeff,
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
    ],
    [
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
        7 as core::ffi::c_int as WarpedFilterCoeff,
        -(20 as core::ffi::c_int) as WarpedFilterCoeff,
        106 as core::ffi::c_int as WarpedFilterCoeff,
        49 as core::ffi::c_int as WarpedFilterCoeff,
        -(16 as core::ffi::c_int) as WarpedFilterCoeff,
        6 as core::ffi::c_int as WarpedFilterCoeff,
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
    ],
    [
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
        7 as core::ffi::c_int as WarpedFilterCoeff,
        -(21 as core::ffi::c_int) as WarpedFilterCoeff,
        104 as core::ffi::c_int as WarpedFilterCoeff,
        51 as core::ffi::c_int as WarpedFilterCoeff,
        -(16 as core::ffi::c_int) as WarpedFilterCoeff,
        7 as core::ffi::c_int as WarpedFilterCoeff,
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
    ],
    [
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
        7 as core::ffi::c_int as WarpedFilterCoeff,
        -(21 as core::ffi::c_int) as WarpedFilterCoeff,
        102 as core::ffi::c_int as WarpedFilterCoeff,
        54 as core::ffi::c_int as WarpedFilterCoeff,
        -(17 as core::ffi::c_int) as WarpedFilterCoeff,
        7 as core::ffi::c_int as WarpedFilterCoeff,
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
    ],
    [
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
        8 as core::ffi::c_int as WarpedFilterCoeff,
        -(21 as core::ffi::c_int) as WarpedFilterCoeff,
        100 as core::ffi::c_int as WarpedFilterCoeff,
        56 as core::ffi::c_int as WarpedFilterCoeff,
        -(18 as core::ffi::c_int) as WarpedFilterCoeff,
        7 as core::ffi::c_int as WarpedFilterCoeff,
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
    ],
    [
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
        8 as core::ffi::c_int as WarpedFilterCoeff,
        -(22 as core::ffi::c_int) as WarpedFilterCoeff,
        98 as core::ffi::c_int as WarpedFilterCoeff,
        59 as core::ffi::c_int as WarpedFilterCoeff,
        -(18 as core::ffi::c_int) as WarpedFilterCoeff,
        7 as core::ffi::c_int as WarpedFilterCoeff,
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
    ],
    [
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
        8 as core::ffi::c_int as WarpedFilterCoeff,
        -(22 as core::ffi::c_int) as WarpedFilterCoeff,
        96 as core::ffi::c_int as WarpedFilterCoeff,
        62 as core::ffi::c_int as WarpedFilterCoeff,
        -(19 as core::ffi::c_int) as WarpedFilterCoeff,
        7 as core::ffi::c_int as WarpedFilterCoeff,
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
    ],
    [
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
        8 as core::ffi::c_int as WarpedFilterCoeff,
        -(22 as core::ffi::c_int) as WarpedFilterCoeff,
        94 as core::ffi::c_int as WarpedFilterCoeff,
        64 as core::ffi::c_int as WarpedFilterCoeff,
        -(19 as core::ffi::c_int) as WarpedFilterCoeff,
        7 as core::ffi::c_int as WarpedFilterCoeff,
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
    ],
    [
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
        8 as core::ffi::c_int as WarpedFilterCoeff,
        -(22 as core::ffi::c_int) as WarpedFilterCoeff,
        91 as core::ffi::c_int as WarpedFilterCoeff,
        67 as core::ffi::c_int as WarpedFilterCoeff,
        -(20 as core::ffi::c_int) as WarpedFilterCoeff,
        8 as core::ffi::c_int as WarpedFilterCoeff,
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
    ],
    [
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
        8 as core::ffi::c_int as WarpedFilterCoeff,
        -(22 as core::ffi::c_int) as WarpedFilterCoeff,
        89 as core::ffi::c_int as WarpedFilterCoeff,
        69 as core::ffi::c_int as WarpedFilterCoeff,
        -(20 as core::ffi::c_int) as WarpedFilterCoeff,
        8 as core::ffi::c_int as WarpedFilterCoeff,
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
    ],
    [
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
        8 as core::ffi::c_int as WarpedFilterCoeff,
        -(22 as core::ffi::c_int) as WarpedFilterCoeff,
        87 as core::ffi::c_int as WarpedFilterCoeff,
        72 as core::ffi::c_int as WarpedFilterCoeff,
        -(21 as core::ffi::c_int) as WarpedFilterCoeff,
        8 as core::ffi::c_int as WarpedFilterCoeff,
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
    ],
    [
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
        8 as core::ffi::c_int as WarpedFilterCoeff,
        -(21 as core::ffi::c_int) as WarpedFilterCoeff,
        84 as core::ffi::c_int as WarpedFilterCoeff,
        74 as core::ffi::c_int as WarpedFilterCoeff,
        -(21 as core::ffi::c_int) as WarpedFilterCoeff,
        8 as core::ffi::c_int as WarpedFilterCoeff,
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
    ],
    [
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
        8 as core::ffi::c_int as WarpedFilterCoeff,
        -(22 as core::ffi::c_int) as WarpedFilterCoeff,
        82 as core::ffi::c_int as WarpedFilterCoeff,
        77 as core::ffi::c_int as WarpedFilterCoeff,
        -(21 as core::ffi::c_int) as WarpedFilterCoeff,
        8 as core::ffi::c_int as WarpedFilterCoeff,
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
    ],
    [
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
        8 as core::ffi::c_int as WarpedFilterCoeff,
        -(21 as core::ffi::c_int) as WarpedFilterCoeff,
        79 as core::ffi::c_int as WarpedFilterCoeff,
        79 as core::ffi::c_int as WarpedFilterCoeff,
        -(21 as core::ffi::c_int) as WarpedFilterCoeff,
        8 as core::ffi::c_int as WarpedFilterCoeff,
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
    ],
    [
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
        8 as core::ffi::c_int as WarpedFilterCoeff,
        -(21 as core::ffi::c_int) as WarpedFilterCoeff,
        77 as core::ffi::c_int as WarpedFilterCoeff,
        82 as core::ffi::c_int as WarpedFilterCoeff,
        -(22 as core::ffi::c_int) as WarpedFilterCoeff,
        8 as core::ffi::c_int as WarpedFilterCoeff,
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
    ],
    [
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
        8 as core::ffi::c_int as WarpedFilterCoeff,
        -(21 as core::ffi::c_int) as WarpedFilterCoeff,
        74 as core::ffi::c_int as WarpedFilterCoeff,
        84 as core::ffi::c_int as WarpedFilterCoeff,
        -(21 as core::ffi::c_int) as WarpedFilterCoeff,
        8 as core::ffi::c_int as WarpedFilterCoeff,
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
    ],
    [
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
        8 as core::ffi::c_int as WarpedFilterCoeff,
        -(21 as core::ffi::c_int) as WarpedFilterCoeff,
        72 as core::ffi::c_int as WarpedFilterCoeff,
        87 as core::ffi::c_int as WarpedFilterCoeff,
        -(22 as core::ffi::c_int) as WarpedFilterCoeff,
        8 as core::ffi::c_int as WarpedFilterCoeff,
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
    ],
    [
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
        8 as core::ffi::c_int as WarpedFilterCoeff,
        -(20 as core::ffi::c_int) as WarpedFilterCoeff,
        69 as core::ffi::c_int as WarpedFilterCoeff,
        89 as core::ffi::c_int as WarpedFilterCoeff,
        -(22 as core::ffi::c_int) as WarpedFilterCoeff,
        8 as core::ffi::c_int as WarpedFilterCoeff,
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
    ],
    [
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
        8 as core::ffi::c_int as WarpedFilterCoeff,
        -(20 as core::ffi::c_int) as WarpedFilterCoeff,
        67 as core::ffi::c_int as WarpedFilterCoeff,
        91 as core::ffi::c_int as WarpedFilterCoeff,
        -(22 as core::ffi::c_int) as WarpedFilterCoeff,
        8 as core::ffi::c_int as WarpedFilterCoeff,
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
    ],
    [
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
        7 as core::ffi::c_int as WarpedFilterCoeff,
        -(19 as core::ffi::c_int) as WarpedFilterCoeff,
        64 as core::ffi::c_int as WarpedFilterCoeff,
        94 as core::ffi::c_int as WarpedFilterCoeff,
        -(22 as core::ffi::c_int) as WarpedFilterCoeff,
        8 as core::ffi::c_int as WarpedFilterCoeff,
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
    ],
    [
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
        7 as core::ffi::c_int as WarpedFilterCoeff,
        -(19 as core::ffi::c_int) as WarpedFilterCoeff,
        62 as core::ffi::c_int as WarpedFilterCoeff,
        96 as core::ffi::c_int as WarpedFilterCoeff,
        -(22 as core::ffi::c_int) as WarpedFilterCoeff,
        8 as core::ffi::c_int as WarpedFilterCoeff,
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
    ],
    [
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
        7 as core::ffi::c_int as WarpedFilterCoeff,
        -(18 as core::ffi::c_int) as WarpedFilterCoeff,
        59 as core::ffi::c_int as WarpedFilterCoeff,
        98 as core::ffi::c_int as WarpedFilterCoeff,
        -(22 as core::ffi::c_int) as WarpedFilterCoeff,
        8 as core::ffi::c_int as WarpedFilterCoeff,
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
    ],
    [
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
        7 as core::ffi::c_int as WarpedFilterCoeff,
        -(18 as core::ffi::c_int) as WarpedFilterCoeff,
        56 as core::ffi::c_int as WarpedFilterCoeff,
        100 as core::ffi::c_int as WarpedFilterCoeff,
        -(21 as core::ffi::c_int) as WarpedFilterCoeff,
        8 as core::ffi::c_int as WarpedFilterCoeff,
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
    ],
    [
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
        7 as core::ffi::c_int as WarpedFilterCoeff,
        -(17 as core::ffi::c_int) as WarpedFilterCoeff,
        54 as core::ffi::c_int as WarpedFilterCoeff,
        102 as core::ffi::c_int as WarpedFilterCoeff,
        -(21 as core::ffi::c_int) as WarpedFilterCoeff,
        7 as core::ffi::c_int as WarpedFilterCoeff,
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
    ],
    [
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
        7 as core::ffi::c_int as WarpedFilterCoeff,
        -(16 as core::ffi::c_int) as WarpedFilterCoeff,
        51 as core::ffi::c_int as WarpedFilterCoeff,
        104 as core::ffi::c_int as WarpedFilterCoeff,
        -(21 as core::ffi::c_int) as WarpedFilterCoeff,
        7 as core::ffi::c_int as WarpedFilterCoeff,
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
    ],
    [
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
        6 as core::ffi::c_int as WarpedFilterCoeff,
        -(16 as core::ffi::c_int) as WarpedFilterCoeff,
        49 as core::ffi::c_int as WarpedFilterCoeff,
        106 as core::ffi::c_int as WarpedFilterCoeff,
        -(20 as core::ffi::c_int) as WarpedFilterCoeff,
        7 as core::ffi::c_int as WarpedFilterCoeff,
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
    ],
    [
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
        6 as core::ffi::c_int as WarpedFilterCoeff,
        -(15 as core::ffi::c_int) as WarpedFilterCoeff,
        46 as core::ffi::c_int as WarpedFilterCoeff,
        108 as core::ffi::c_int as WarpedFilterCoeff,
        -(20 as core::ffi::c_int) as WarpedFilterCoeff,
        7 as core::ffi::c_int as WarpedFilterCoeff,
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
    ],
    [
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
        6 as core::ffi::c_int as WarpedFilterCoeff,
        -(15 as core::ffi::c_int) as WarpedFilterCoeff,
        43 as core::ffi::c_int as WarpedFilterCoeff,
        110 as core::ffi::c_int as WarpedFilterCoeff,
        -(19 as core::ffi::c_int) as WarpedFilterCoeff,
        7 as core::ffi::c_int as WarpedFilterCoeff,
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
    ],
    [
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
        6 as core::ffi::c_int as WarpedFilterCoeff,
        -(14 as core::ffi::c_int) as WarpedFilterCoeff,
        41 as core::ffi::c_int as WarpedFilterCoeff,
        111 as core::ffi::c_int as WarpedFilterCoeff,
        -(19 as core::ffi::c_int) as WarpedFilterCoeff,
        7 as core::ffi::c_int as WarpedFilterCoeff,
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
    ],
    [
        -(1 as core::ffi::c_int) as WarpedFilterCoeff,
        5 as core::ffi::c_int as WarpedFilterCoeff,
        -(13 as core::ffi::c_int) as WarpedFilterCoeff,
        38 as core::ffi::c_int as WarpedFilterCoeff,
        113 as core::ffi::c_int as WarpedFilterCoeff,
        -(18 as core::ffi::c_int) as WarpedFilterCoeff,
        6 as core::ffi::c_int as WarpedFilterCoeff,
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
    ],
    [
        -(1 as core::ffi::c_int) as WarpedFilterCoeff,
        5 as core::ffi::c_int as WarpedFilterCoeff,
        -(12 as core::ffi::c_int) as WarpedFilterCoeff,
        35 as core::ffi::c_int as WarpedFilterCoeff,
        114 as core::ffi::c_int as WarpedFilterCoeff,
        -(17 as core::ffi::c_int) as WarpedFilterCoeff,
        6 as core::ffi::c_int as WarpedFilterCoeff,
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
    ],
    [
        -(1 as core::ffi::c_int) as WarpedFilterCoeff,
        5 as core::ffi::c_int as WarpedFilterCoeff,
        -(12 as core::ffi::c_int) as WarpedFilterCoeff,
        33 as core::ffi::c_int as WarpedFilterCoeff,
        116 as core::ffi::c_int as WarpedFilterCoeff,
        -(17 as core::ffi::c_int) as WarpedFilterCoeff,
        6 as core::ffi::c_int as WarpedFilterCoeff,
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
    ],
    [
        -(1 as core::ffi::c_int) as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
        -(11 as core::ffi::c_int) as WarpedFilterCoeff,
        30 as core::ffi::c_int as WarpedFilterCoeff,
        118 as core::ffi::c_int as WarpedFilterCoeff,
        -(16 as core::ffi::c_int) as WarpedFilterCoeff,
        5 as core::ffi::c_int as WarpedFilterCoeff,
        -(1 as core::ffi::c_int) as WarpedFilterCoeff,
    ],
    [
        -(1 as core::ffi::c_int) as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
        -(10 as core::ffi::c_int) as WarpedFilterCoeff,
        27 as core::ffi::c_int as WarpedFilterCoeff,
        119 as core::ffi::c_int as WarpedFilterCoeff,
        -(15 as core::ffi::c_int) as WarpedFilterCoeff,
        5 as core::ffi::c_int as WarpedFilterCoeff,
        -(1 as core::ffi::c_int) as WarpedFilterCoeff,
    ],
    [
        -(1 as core::ffi::c_int) as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
        -(9 as core::ffi::c_int) as WarpedFilterCoeff,
        25 as core::ffi::c_int as WarpedFilterCoeff,
        120 as core::ffi::c_int as WarpedFilterCoeff,
        -(14 as core::ffi::c_int) as WarpedFilterCoeff,
        5 as core::ffi::c_int as WarpedFilterCoeff,
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
    ],
    [
        -(1 as core::ffi::c_int) as WarpedFilterCoeff,
        3 as core::ffi::c_int as WarpedFilterCoeff,
        -(8 as core::ffi::c_int) as WarpedFilterCoeff,
        23 as core::ffi::c_int as WarpedFilterCoeff,
        121 as core::ffi::c_int as WarpedFilterCoeff,
        -(13 as core::ffi::c_int) as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
        -(1 as core::ffi::c_int) as WarpedFilterCoeff,
    ],
    [
        -(1 as core::ffi::c_int) as WarpedFilterCoeff,
        3 as core::ffi::c_int as WarpedFilterCoeff,
        -(7 as core::ffi::c_int) as WarpedFilterCoeff,
        20 as core::ffi::c_int as WarpedFilterCoeff,
        122 as core::ffi::c_int as WarpedFilterCoeff,
        -(12 as core::ffi::c_int) as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
        -(1 as core::ffi::c_int) as WarpedFilterCoeff,
    ],
    [
        -(1 as core::ffi::c_int) as WarpedFilterCoeff,
        3 as core::ffi::c_int as WarpedFilterCoeff,
        -(7 as core::ffi::c_int) as WarpedFilterCoeff,
        18 as core::ffi::c_int as WarpedFilterCoeff,
        123 as core::ffi::c_int as WarpedFilterCoeff,
        -(11 as core::ffi::c_int) as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
        -(1 as core::ffi::c_int) as WarpedFilterCoeff,
    ],
    [
        -(1 as core::ffi::c_int) as WarpedFilterCoeff,
        3 as core::ffi::c_int as WarpedFilterCoeff,
        -(6 as core::ffi::c_int) as WarpedFilterCoeff,
        16 as core::ffi::c_int as WarpedFilterCoeff,
        124 as core::ffi::c_int as WarpedFilterCoeff,
        -(10 as core::ffi::c_int) as WarpedFilterCoeff,
        3 as core::ffi::c_int as WarpedFilterCoeff,
        -(1 as core::ffi::c_int) as WarpedFilterCoeff,
    ],
    [
        -(1 as core::ffi::c_int) as WarpedFilterCoeff,
        2 as core::ffi::c_int as WarpedFilterCoeff,
        -(5 as core::ffi::c_int) as WarpedFilterCoeff,
        13 as core::ffi::c_int as WarpedFilterCoeff,
        125 as core::ffi::c_int as WarpedFilterCoeff,
        -(8 as core::ffi::c_int) as WarpedFilterCoeff,
        3 as core::ffi::c_int as WarpedFilterCoeff,
        -(1 as core::ffi::c_int) as WarpedFilterCoeff,
    ],
    [
        -(1 as core::ffi::c_int) as WarpedFilterCoeff,
        2 as core::ffi::c_int as WarpedFilterCoeff,
        -(4 as core::ffi::c_int) as WarpedFilterCoeff,
        11 as core::ffi::c_int as WarpedFilterCoeff,
        126 as core::ffi::c_int as WarpedFilterCoeff,
        -(7 as core::ffi::c_int) as WarpedFilterCoeff,
        2 as core::ffi::c_int as WarpedFilterCoeff,
        -(1 as core::ffi::c_int) as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        1 as core::ffi::c_int as WarpedFilterCoeff,
        -(3 as core::ffi::c_int) as WarpedFilterCoeff,
        8 as core::ffi::c_int as WarpedFilterCoeff,
        126 as core::ffi::c_int as WarpedFilterCoeff,
        -(6 as core::ffi::c_int) as WarpedFilterCoeff,
        2 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        1 as core::ffi::c_int as WarpedFilterCoeff,
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
        6 as core::ffi::c_int as WarpedFilterCoeff,
        127 as core::ffi::c_int as WarpedFilterCoeff,
        -(5 as core::ffi::c_int) as WarpedFilterCoeff,
        1 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        1 as core::ffi::c_int as WarpedFilterCoeff,
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
        127 as core::ffi::c_int as WarpedFilterCoeff,
        -(3 as core::ffi::c_int) as WarpedFilterCoeff,
        1 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        2 as core::ffi::c_int as WarpedFilterCoeff,
        127 as core::ffi::c_int as WarpedFilterCoeff,
        -(1 as core::ffi::c_int) as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        1 as core::ffi::c_int as WarpedFilterCoeff,
        127 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        -(1 as core::ffi::c_int) as WarpedFilterCoeff,
        127 as core::ffi::c_int as WarpedFilterCoeff,
        2 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        1 as core::ffi::c_int as WarpedFilterCoeff,
        -(3 as core::ffi::c_int) as WarpedFilterCoeff,
        127 as core::ffi::c_int as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
        -(1 as core::ffi::c_int) as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        1 as core::ffi::c_int as WarpedFilterCoeff,
        -(4 as core::ffi::c_int) as WarpedFilterCoeff,
        126 as core::ffi::c_int as WarpedFilterCoeff,
        6 as core::ffi::c_int as WarpedFilterCoeff,
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
        1 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        1 as core::ffi::c_int as WarpedFilterCoeff,
        -(5 as core::ffi::c_int) as WarpedFilterCoeff,
        126 as core::ffi::c_int as WarpedFilterCoeff,
        8 as core::ffi::c_int as WarpedFilterCoeff,
        -(3 as core::ffi::c_int) as WarpedFilterCoeff,
        1 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        1 as core::ffi::c_int as WarpedFilterCoeff,
        -(6 as core::ffi::c_int) as WarpedFilterCoeff,
        125 as core::ffi::c_int as WarpedFilterCoeff,
        11 as core::ffi::c_int as WarpedFilterCoeff,
        -(4 as core::ffi::c_int) as WarpedFilterCoeff,
        1 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        1 as core::ffi::c_int as WarpedFilterCoeff,
        -(7 as core::ffi::c_int) as WarpedFilterCoeff,
        124 as core::ffi::c_int as WarpedFilterCoeff,
        13 as core::ffi::c_int as WarpedFilterCoeff,
        -(4 as core::ffi::c_int) as WarpedFilterCoeff,
        1 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        2 as core::ffi::c_int as WarpedFilterCoeff,
        -(8 as core::ffi::c_int) as WarpedFilterCoeff,
        123 as core::ffi::c_int as WarpedFilterCoeff,
        15 as core::ffi::c_int as WarpedFilterCoeff,
        -(5 as core::ffi::c_int) as WarpedFilterCoeff,
        1 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        2 as core::ffi::c_int as WarpedFilterCoeff,
        -(9 as core::ffi::c_int) as WarpedFilterCoeff,
        122 as core::ffi::c_int as WarpedFilterCoeff,
        18 as core::ffi::c_int as WarpedFilterCoeff,
        -(6 as core::ffi::c_int) as WarpedFilterCoeff,
        1 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        2 as core::ffi::c_int as WarpedFilterCoeff,
        -(10 as core::ffi::c_int) as WarpedFilterCoeff,
        121 as core::ffi::c_int as WarpedFilterCoeff,
        20 as core::ffi::c_int as WarpedFilterCoeff,
        -(6 as core::ffi::c_int) as WarpedFilterCoeff,
        1 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        2 as core::ffi::c_int as WarpedFilterCoeff,
        -(11 as core::ffi::c_int) as WarpedFilterCoeff,
        120 as core::ffi::c_int as WarpedFilterCoeff,
        22 as core::ffi::c_int as WarpedFilterCoeff,
        -(7 as core::ffi::c_int) as WarpedFilterCoeff,
        2 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        2 as core::ffi::c_int as WarpedFilterCoeff,
        -(12 as core::ffi::c_int) as WarpedFilterCoeff,
        119 as core::ffi::c_int as WarpedFilterCoeff,
        25 as core::ffi::c_int as WarpedFilterCoeff,
        -(8 as core::ffi::c_int) as WarpedFilterCoeff,
        2 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        3 as core::ffi::c_int as WarpedFilterCoeff,
        -(13 as core::ffi::c_int) as WarpedFilterCoeff,
        117 as core::ffi::c_int as WarpedFilterCoeff,
        27 as core::ffi::c_int as WarpedFilterCoeff,
        -(8 as core::ffi::c_int) as WarpedFilterCoeff,
        2 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        3 as core::ffi::c_int as WarpedFilterCoeff,
        -(13 as core::ffi::c_int) as WarpedFilterCoeff,
        116 as core::ffi::c_int as WarpedFilterCoeff,
        29 as core::ffi::c_int as WarpedFilterCoeff,
        -(9 as core::ffi::c_int) as WarpedFilterCoeff,
        2 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        3 as core::ffi::c_int as WarpedFilterCoeff,
        -(14 as core::ffi::c_int) as WarpedFilterCoeff,
        114 as core::ffi::c_int as WarpedFilterCoeff,
        32 as core::ffi::c_int as WarpedFilterCoeff,
        -(10 as core::ffi::c_int) as WarpedFilterCoeff,
        3 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        3 as core::ffi::c_int as WarpedFilterCoeff,
        -(15 as core::ffi::c_int) as WarpedFilterCoeff,
        113 as core::ffi::c_int as WarpedFilterCoeff,
        35 as core::ffi::c_int as WarpedFilterCoeff,
        -(10 as core::ffi::c_int) as WarpedFilterCoeff,
        2 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        3 as core::ffi::c_int as WarpedFilterCoeff,
        -(15 as core::ffi::c_int) as WarpedFilterCoeff,
        111 as core::ffi::c_int as WarpedFilterCoeff,
        37 as core::ffi::c_int as WarpedFilterCoeff,
        -(11 as core::ffi::c_int) as WarpedFilterCoeff,
        3 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        3 as core::ffi::c_int as WarpedFilterCoeff,
        -(16 as core::ffi::c_int) as WarpedFilterCoeff,
        109 as core::ffi::c_int as WarpedFilterCoeff,
        40 as core::ffi::c_int as WarpedFilterCoeff,
        -(11 as core::ffi::c_int) as WarpedFilterCoeff,
        3 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        3 as core::ffi::c_int as WarpedFilterCoeff,
        -(16 as core::ffi::c_int) as WarpedFilterCoeff,
        108 as core::ffi::c_int as WarpedFilterCoeff,
        42 as core::ffi::c_int as WarpedFilterCoeff,
        -(12 as core::ffi::c_int) as WarpedFilterCoeff,
        3 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
        -(17 as core::ffi::c_int) as WarpedFilterCoeff,
        106 as core::ffi::c_int as WarpedFilterCoeff,
        45 as core::ffi::c_int as WarpedFilterCoeff,
        -(13 as core::ffi::c_int) as WarpedFilterCoeff,
        3 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
        -(17 as core::ffi::c_int) as WarpedFilterCoeff,
        104 as core::ffi::c_int as WarpedFilterCoeff,
        47 as core::ffi::c_int as WarpedFilterCoeff,
        -(13 as core::ffi::c_int) as WarpedFilterCoeff,
        3 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
        -(17 as core::ffi::c_int) as WarpedFilterCoeff,
        102 as core::ffi::c_int as WarpedFilterCoeff,
        50 as core::ffi::c_int as WarpedFilterCoeff,
        -(14 as core::ffi::c_int) as WarpedFilterCoeff,
        3 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
        -(17 as core::ffi::c_int) as WarpedFilterCoeff,
        100 as core::ffi::c_int as WarpedFilterCoeff,
        52 as core::ffi::c_int as WarpedFilterCoeff,
        -(14 as core::ffi::c_int) as WarpedFilterCoeff,
        3 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
        -(18 as core::ffi::c_int) as WarpedFilterCoeff,
        98 as core::ffi::c_int as WarpedFilterCoeff,
        55 as core::ffi::c_int as WarpedFilterCoeff,
        -(15 as core::ffi::c_int) as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
        -(18 as core::ffi::c_int) as WarpedFilterCoeff,
        96 as core::ffi::c_int as WarpedFilterCoeff,
        58 as core::ffi::c_int as WarpedFilterCoeff,
        -(15 as core::ffi::c_int) as WarpedFilterCoeff,
        3 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
        -(18 as core::ffi::c_int) as WarpedFilterCoeff,
        94 as core::ffi::c_int as WarpedFilterCoeff,
        60 as core::ffi::c_int as WarpedFilterCoeff,
        -(16 as core::ffi::c_int) as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
        -(18 as core::ffi::c_int) as WarpedFilterCoeff,
        91 as core::ffi::c_int as WarpedFilterCoeff,
        63 as core::ffi::c_int as WarpedFilterCoeff,
        -(16 as core::ffi::c_int) as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
        -(18 as core::ffi::c_int) as WarpedFilterCoeff,
        89 as core::ffi::c_int as WarpedFilterCoeff,
        65 as core::ffi::c_int as WarpedFilterCoeff,
        -(16 as core::ffi::c_int) as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
        -(18 as core::ffi::c_int) as WarpedFilterCoeff,
        87 as core::ffi::c_int as WarpedFilterCoeff,
        68 as core::ffi::c_int as WarpedFilterCoeff,
        -(17 as core::ffi::c_int) as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
        -(18 as core::ffi::c_int) as WarpedFilterCoeff,
        85 as core::ffi::c_int as WarpedFilterCoeff,
        70 as core::ffi::c_int as WarpedFilterCoeff,
        -(17 as core::ffi::c_int) as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
        -(18 as core::ffi::c_int) as WarpedFilterCoeff,
        82 as core::ffi::c_int as WarpedFilterCoeff,
        73 as core::ffi::c_int as WarpedFilterCoeff,
        -(17 as core::ffi::c_int) as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
        -(18 as core::ffi::c_int) as WarpedFilterCoeff,
        80 as core::ffi::c_int as WarpedFilterCoeff,
        75 as core::ffi::c_int as WarpedFilterCoeff,
        -(17 as core::ffi::c_int) as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
        -(18 as core::ffi::c_int) as WarpedFilterCoeff,
        78 as core::ffi::c_int as WarpedFilterCoeff,
        78 as core::ffi::c_int as WarpedFilterCoeff,
        -(18 as core::ffi::c_int) as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
        -(17 as core::ffi::c_int) as WarpedFilterCoeff,
        75 as core::ffi::c_int as WarpedFilterCoeff,
        80 as core::ffi::c_int as WarpedFilterCoeff,
        -(18 as core::ffi::c_int) as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
        -(17 as core::ffi::c_int) as WarpedFilterCoeff,
        73 as core::ffi::c_int as WarpedFilterCoeff,
        82 as core::ffi::c_int as WarpedFilterCoeff,
        -(18 as core::ffi::c_int) as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
        -(17 as core::ffi::c_int) as WarpedFilterCoeff,
        70 as core::ffi::c_int as WarpedFilterCoeff,
        85 as core::ffi::c_int as WarpedFilterCoeff,
        -(18 as core::ffi::c_int) as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
        -(17 as core::ffi::c_int) as WarpedFilterCoeff,
        68 as core::ffi::c_int as WarpedFilterCoeff,
        87 as core::ffi::c_int as WarpedFilterCoeff,
        -(18 as core::ffi::c_int) as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
        -(16 as core::ffi::c_int) as WarpedFilterCoeff,
        65 as core::ffi::c_int as WarpedFilterCoeff,
        89 as core::ffi::c_int as WarpedFilterCoeff,
        -(18 as core::ffi::c_int) as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
        -(16 as core::ffi::c_int) as WarpedFilterCoeff,
        63 as core::ffi::c_int as WarpedFilterCoeff,
        91 as core::ffi::c_int as WarpedFilterCoeff,
        -(18 as core::ffi::c_int) as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
        -(16 as core::ffi::c_int) as WarpedFilterCoeff,
        60 as core::ffi::c_int as WarpedFilterCoeff,
        94 as core::ffi::c_int as WarpedFilterCoeff,
        -(18 as core::ffi::c_int) as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        3 as core::ffi::c_int as WarpedFilterCoeff,
        -(15 as core::ffi::c_int) as WarpedFilterCoeff,
        58 as core::ffi::c_int as WarpedFilterCoeff,
        96 as core::ffi::c_int as WarpedFilterCoeff,
        -(18 as core::ffi::c_int) as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
        -(15 as core::ffi::c_int) as WarpedFilterCoeff,
        55 as core::ffi::c_int as WarpedFilterCoeff,
        98 as core::ffi::c_int as WarpedFilterCoeff,
        -(18 as core::ffi::c_int) as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        3 as core::ffi::c_int as WarpedFilterCoeff,
        -(14 as core::ffi::c_int) as WarpedFilterCoeff,
        52 as core::ffi::c_int as WarpedFilterCoeff,
        100 as core::ffi::c_int as WarpedFilterCoeff,
        -(17 as core::ffi::c_int) as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        3 as core::ffi::c_int as WarpedFilterCoeff,
        -(14 as core::ffi::c_int) as WarpedFilterCoeff,
        50 as core::ffi::c_int as WarpedFilterCoeff,
        102 as core::ffi::c_int as WarpedFilterCoeff,
        -(17 as core::ffi::c_int) as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        3 as core::ffi::c_int as WarpedFilterCoeff,
        -(13 as core::ffi::c_int) as WarpedFilterCoeff,
        47 as core::ffi::c_int as WarpedFilterCoeff,
        104 as core::ffi::c_int as WarpedFilterCoeff,
        -(17 as core::ffi::c_int) as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        3 as core::ffi::c_int as WarpedFilterCoeff,
        -(13 as core::ffi::c_int) as WarpedFilterCoeff,
        45 as core::ffi::c_int as WarpedFilterCoeff,
        106 as core::ffi::c_int as WarpedFilterCoeff,
        -(17 as core::ffi::c_int) as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        3 as core::ffi::c_int as WarpedFilterCoeff,
        -(12 as core::ffi::c_int) as WarpedFilterCoeff,
        42 as core::ffi::c_int as WarpedFilterCoeff,
        108 as core::ffi::c_int as WarpedFilterCoeff,
        -(16 as core::ffi::c_int) as WarpedFilterCoeff,
        3 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        3 as core::ffi::c_int as WarpedFilterCoeff,
        -(11 as core::ffi::c_int) as WarpedFilterCoeff,
        40 as core::ffi::c_int as WarpedFilterCoeff,
        109 as core::ffi::c_int as WarpedFilterCoeff,
        -(16 as core::ffi::c_int) as WarpedFilterCoeff,
        3 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        3 as core::ffi::c_int as WarpedFilterCoeff,
        -(11 as core::ffi::c_int) as WarpedFilterCoeff,
        37 as core::ffi::c_int as WarpedFilterCoeff,
        111 as core::ffi::c_int as WarpedFilterCoeff,
        -(15 as core::ffi::c_int) as WarpedFilterCoeff,
        3 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        2 as core::ffi::c_int as WarpedFilterCoeff,
        -(10 as core::ffi::c_int) as WarpedFilterCoeff,
        35 as core::ffi::c_int as WarpedFilterCoeff,
        113 as core::ffi::c_int as WarpedFilterCoeff,
        -(15 as core::ffi::c_int) as WarpedFilterCoeff,
        3 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        3 as core::ffi::c_int as WarpedFilterCoeff,
        -(10 as core::ffi::c_int) as WarpedFilterCoeff,
        32 as core::ffi::c_int as WarpedFilterCoeff,
        114 as core::ffi::c_int as WarpedFilterCoeff,
        -(14 as core::ffi::c_int) as WarpedFilterCoeff,
        3 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        2 as core::ffi::c_int as WarpedFilterCoeff,
        -(9 as core::ffi::c_int) as WarpedFilterCoeff,
        29 as core::ffi::c_int as WarpedFilterCoeff,
        116 as core::ffi::c_int as WarpedFilterCoeff,
        -(13 as core::ffi::c_int) as WarpedFilterCoeff,
        3 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        2 as core::ffi::c_int as WarpedFilterCoeff,
        -(8 as core::ffi::c_int) as WarpedFilterCoeff,
        27 as core::ffi::c_int as WarpedFilterCoeff,
        117 as core::ffi::c_int as WarpedFilterCoeff,
        -(13 as core::ffi::c_int) as WarpedFilterCoeff,
        3 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        2 as core::ffi::c_int as WarpedFilterCoeff,
        -(8 as core::ffi::c_int) as WarpedFilterCoeff,
        25 as core::ffi::c_int as WarpedFilterCoeff,
        119 as core::ffi::c_int as WarpedFilterCoeff,
        -(12 as core::ffi::c_int) as WarpedFilterCoeff,
        2 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        2 as core::ffi::c_int as WarpedFilterCoeff,
        -(7 as core::ffi::c_int) as WarpedFilterCoeff,
        22 as core::ffi::c_int as WarpedFilterCoeff,
        120 as core::ffi::c_int as WarpedFilterCoeff,
        -(11 as core::ffi::c_int) as WarpedFilterCoeff,
        2 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        1 as core::ffi::c_int as WarpedFilterCoeff,
        -(6 as core::ffi::c_int) as WarpedFilterCoeff,
        20 as core::ffi::c_int as WarpedFilterCoeff,
        121 as core::ffi::c_int as WarpedFilterCoeff,
        -(10 as core::ffi::c_int) as WarpedFilterCoeff,
        2 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        1 as core::ffi::c_int as WarpedFilterCoeff,
        -(6 as core::ffi::c_int) as WarpedFilterCoeff,
        18 as core::ffi::c_int as WarpedFilterCoeff,
        122 as core::ffi::c_int as WarpedFilterCoeff,
        -(9 as core::ffi::c_int) as WarpedFilterCoeff,
        2 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        1 as core::ffi::c_int as WarpedFilterCoeff,
        -(5 as core::ffi::c_int) as WarpedFilterCoeff,
        15 as core::ffi::c_int as WarpedFilterCoeff,
        123 as core::ffi::c_int as WarpedFilterCoeff,
        -(8 as core::ffi::c_int) as WarpedFilterCoeff,
        2 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        1 as core::ffi::c_int as WarpedFilterCoeff,
        -(4 as core::ffi::c_int) as WarpedFilterCoeff,
        13 as core::ffi::c_int as WarpedFilterCoeff,
        124 as core::ffi::c_int as WarpedFilterCoeff,
        -(7 as core::ffi::c_int) as WarpedFilterCoeff,
        1 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        1 as core::ffi::c_int as WarpedFilterCoeff,
        -(4 as core::ffi::c_int) as WarpedFilterCoeff,
        11 as core::ffi::c_int as WarpedFilterCoeff,
        125 as core::ffi::c_int as WarpedFilterCoeff,
        -(6 as core::ffi::c_int) as WarpedFilterCoeff,
        1 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        1 as core::ffi::c_int as WarpedFilterCoeff,
        -(3 as core::ffi::c_int) as WarpedFilterCoeff,
        8 as core::ffi::c_int as WarpedFilterCoeff,
        126 as core::ffi::c_int as WarpedFilterCoeff,
        -(5 as core::ffi::c_int) as WarpedFilterCoeff,
        1 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        1 as core::ffi::c_int as WarpedFilterCoeff,
        -(2 as core::ffi::c_int) as WarpedFilterCoeff,
        6 as core::ffi::c_int as WarpedFilterCoeff,
        126 as core::ffi::c_int as WarpedFilterCoeff,
        -(4 as core::ffi::c_int) as WarpedFilterCoeff,
        1 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        -(1 as core::ffi::c_int) as WarpedFilterCoeff,
        4 as core::ffi::c_int as WarpedFilterCoeff,
        127 as core::ffi::c_int as WarpedFilterCoeff,
        -(3 as core::ffi::c_int) as WarpedFilterCoeff,
        1 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        2 as core::ffi::c_int as WarpedFilterCoeff,
        127 as core::ffi::c_int as WarpedFilterCoeff,
        -(1 as core::ffi::c_int) as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
    [
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
        2 as core::ffi::c_int as WarpedFilterCoeff,
        127 as core::ffi::c_int as WarpedFilterCoeff,
        -(1 as core::ffi::c_int) as WarpedFilterCoeff,
        0 as core::ffi::c_int as WarpedFilterCoeff,
    ],
];
pub const DIV_LUT_PREC_BITS: core::ffi::c_int = 14 as core::ffi::c_int;
pub const DIV_LUT_BITS: core::ffi::c_int = 8 as core::ffi::c_int;
static mut div_lut: [uint16_t; 257] = [
    16384 as core::ffi::c_int as uint16_t,
    16320 as core::ffi::c_int as uint16_t,
    16257 as core::ffi::c_int as uint16_t,
    16194 as core::ffi::c_int as uint16_t,
    16132 as core::ffi::c_int as uint16_t,
    16070 as core::ffi::c_int as uint16_t,
    16009 as core::ffi::c_int as uint16_t,
    15948 as core::ffi::c_int as uint16_t,
    15888 as core::ffi::c_int as uint16_t,
    15828 as core::ffi::c_int as uint16_t,
    15768 as core::ffi::c_int as uint16_t,
    15709 as core::ffi::c_int as uint16_t,
    15650 as core::ffi::c_int as uint16_t,
    15592 as core::ffi::c_int as uint16_t,
    15534 as core::ffi::c_int as uint16_t,
    15477 as core::ffi::c_int as uint16_t,
    15420 as core::ffi::c_int as uint16_t,
    15364 as core::ffi::c_int as uint16_t,
    15308 as core::ffi::c_int as uint16_t,
    15252 as core::ffi::c_int as uint16_t,
    15197 as core::ffi::c_int as uint16_t,
    15142 as core::ffi::c_int as uint16_t,
    15087 as core::ffi::c_int as uint16_t,
    15033 as core::ffi::c_int as uint16_t,
    14980 as core::ffi::c_int as uint16_t,
    14926 as core::ffi::c_int as uint16_t,
    14873 as core::ffi::c_int as uint16_t,
    14821 as core::ffi::c_int as uint16_t,
    14769 as core::ffi::c_int as uint16_t,
    14717 as core::ffi::c_int as uint16_t,
    14665 as core::ffi::c_int as uint16_t,
    14614 as core::ffi::c_int as uint16_t,
    14564 as core::ffi::c_int as uint16_t,
    14513 as core::ffi::c_int as uint16_t,
    14463 as core::ffi::c_int as uint16_t,
    14413 as core::ffi::c_int as uint16_t,
    14364 as core::ffi::c_int as uint16_t,
    14315 as core::ffi::c_int as uint16_t,
    14266 as core::ffi::c_int as uint16_t,
    14218 as core::ffi::c_int as uint16_t,
    14170 as core::ffi::c_int as uint16_t,
    14122 as core::ffi::c_int as uint16_t,
    14075 as core::ffi::c_int as uint16_t,
    14028 as core::ffi::c_int as uint16_t,
    13981 as core::ffi::c_int as uint16_t,
    13935 as core::ffi::c_int as uint16_t,
    13888 as core::ffi::c_int as uint16_t,
    13843 as core::ffi::c_int as uint16_t,
    13797 as core::ffi::c_int as uint16_t,
    13752 as core::ffi::c_int as uint16_t,
    13707 as core::ffi::c_int as uint16_t,
    13662 as core::ffi::c_int as uint16_t,
    13618 as core::ffi::c_int as uint16_t,
    13574 as core::ffi::c_int as uint16_t,
    13530 as core::ffi::c_int as uint16_t,
    13487 as core::ffi::c_int as uint16_t,
    13443 as core::ffi::c_int as uint16_t,
    13400 as core::ffi::c_int as uint16_t,
    13358 as core::ffi::c_int as uint16_t,
    13315 as core::ffi::c_int as uint16_t,
    13273 as core::ffi::c_int as uint16_t,
    13231 as core::ffi::c_int as uint16_t,
    13190 as core::ffi::c_int as uint16_t,
    13148 as core::ffi::c_int as uint16_t,
    13107 as core::ffi::c_int as uint16_t,
    13066 as core::ffi::c_int as uint16_t,
    13026 as core::ffi::c_int as uint16_t,
    12985 as core::ffi::c_int as uint16_t,
    12945 as core::ffi::c_int as uint16_t,
    12906 as core::ffi::c_int as uint16_t,
    12866 as core::ffi::c_int as uint16_t,
    12827 as core::ffi::c_int as uint16_t,
    12788 as core::ffi::c_int as uint16_t,
    12749 as core::ffi::c_int as uint16_t,
    12710 as core::ffi::c_int as uint16_t,
    12672 as core::ffi::c_int as uint16_t,
    12633 as core::ffi::c_int as uint16_t,
    12596 as core::ffi::c_int as uint16_t,
    12558 as core::ffi::c_int as uint16_t,
    12520 as core::ffi::c_int as uint16_t,
    12483 as core::ffi::c_int as uint16_t,
    12446 as core::ffi::c_int as uint16_t,
    12409 as core::ffi::c_int as uint16_t,
    12373 as core::ffi::c_int as uint16_t,
    12336 as core::ffi::c_int as uint16_t,
    12300 as core::ffi::c_int as uint16_t,
    12264 as core::ffi::c_int as uint16_t,
    12228 as core::ffi::c_int as uint16_t,
    12193 as core::ffi::c_int as uint16_t,
    12157 as core::ffi::c_int as uint16_t,
    12122 as core::ffi::c_int as uint16_t,
    12087 as core::ffi::c_int as uint16_t,
    12053 as core::ffi::c_int as uint16_t,
    12018 as core::ffi::c_int as uint16_t,
    11984 as core::ffi::c_int as uint16_t,
    11950 as core::ffi::c_int as uint16_t,
    11916 as core::ffi::c_int as uint16_t,
    11882 as core::ffi::c_int as uint16_t,
    11848 as core::ffi::c_int as uint16_t,
    11815 as core::ffi::c_int as uint16_t,
    11782 as core::ffi::c_int as uint16_t,
    11749 as core::ffi::c_int as uint16_t,
    11716 as core::ffi::c_int as uint16_t,
    11683 as core::ffi::c_int as uint16_t,
    11651 as core::ffi::c_int as uint16_t,
    11619 as core::ffi::c_int as uint16_t,
    11586 as core::ffi::c_int as uint16_t,
    11555 as core::ffi::c_int as uint16_t,
    11523 as core::ffi::c_int as uint16_t,
    11491 as core::ffi::c_int as uint16_t,
    11460 as core::ffi::c_int as uint16_t,
    11429 as core::ffi::c_int as uint16_t,
    11398 as core::ffi::c_int as uint16_t,
    11367 as core::ffi::c_int as uint16_t,
    11336 as core::ffi::c_int as uint16_t,
    11305 as core::ffi::c_int as uint16_t,
    11275 as core::ffi::c_int as uint16_t,
    11245 as core::ffi::c_int as uint16_t,
    11215 as core::ffi::c_int as uint16_t,
    11185 as core::ffi::c_int as uint16_t,
    11155 as core::ffi::c_int as uint16_t,
    11125 as core::ffi::c_int as uint16_t,
    11096 as core::ffi::c_int as uint16_t,
    11067 as core::ffi::c_int as uint16_t,
    11038 as core::ffi::c_int as uint16_t,
    11009 as core::ffi::c_int as uint16_t,
    10980 as core::ffi::c_int as uint16_t,
    10951 as core::ffi::c_int as uint16_t,
    10923 as core::ffi::c_int as uint16_t,
    10894 as core::ffi::c_int as uint16_t,
    10866 as core::ffi::c_int as uint16_t,
    10838 as core::ffi::c_int as uint16_t,
    10810 as core::ffi::c_int as uint16_t,
    10782 as core::ffi::c_int as uint16_t,
    10755 as core::ffi::c_int as uint16_t,
    10727 as core::ffi::c_int as uint16_t,
    10700 as core::ffi::c_int as uint16_t,
    10673 as core::ffi::c_int as uint16_t,
    10645 as core::ffi::c_int as uint16_t,
    10618 as core::ffi::c_int as uint16_t,
    10592 as core::ffi::c_int as uint16_t,
    10565 as core::ffi::c_int as uint16_t,
    10538 as core::ffi::c_int as uint16_t,
    10512 as core::ffi::c_int as uint16_t,
    10486 as core::ffi::c_int as uint16_t,
    10460 as core::ffi::c_int as uint16_t,
    10434 as core::ffi::c_int as uint16_t,
    10408 as core::ffi::c_int as uint16_t,
    10382 as core::ffi::c_int as uint16_t,
    10356 as core::ffi::c_int as uint16_t,
    10331 as core::ffi::c_int as uint16_t,
    10305 as core::ffi::c_int as uint16_t,
    10280 as core::ffi::c_int as uint16_t,
    10255 as core::ffi::c_int as uint16_t,
    10230 as core::ffi::c_int as uint16_t,
    10205 as core::ffi::c_int as uint16_t,
    10180 as core::ffi::c_int as uint16_t,
    10156 as core::ffi::c_int as uint16_t,
    10131 as core::ffi::c_int as uint16_t,
    10107 as core::ffi::c_int as uint16_t,
    10082 as core::ffi::c_int as uint16_t,
    10058 as core::ffi::c_int as uint16_t,
    10034 as core::ffi::c_int as uint16_t,
    10010 as core::ffi::c_int as uint16_t,
    9986 as core::ffi::c_int as uint16_t,
    9963 as core::ffi::c_int as uint16_t,
    9939 as core::ffi::c_int as uint16_t,
    9916 as core::ffi::c_int as uint16_t,
    9892 as core::ffi::c_int as uint16_t,
    9869 as core::ffi::c_int as uint16_t,
    9846 as core::ffi::c_int as uint16_t,
    9823 as core::ffi::c_int as uint16_t,
    9800 as core::ffi::c_int as uint16_t,
    9777 as core::ffi::c_int as uint16_t,
    9754 as core::ffi::c_int as uint16_t,
    9732 as core::ffi::c_int as uint16_t,
    9709 as core::ffi::c_int as uint16_t,
    9687 as core::ffi::c_int as uint16_t,
    9664 as core::ffi::c_int as uint16_t,
    9642 as core::ffi::c_int as uint16_t,
    9620 as core::ffi::c_int as uint16_t,
    9598 as core::ffi::c_int as uint16_t,
    9576 as core::ffi::c_int as uint16_t,
    9554 as core::ffi::c_int as uint16_t,
    9533 as core::ffi::c_int as uint16_t,
    9511 as core::ffi::c_int as uint16_t,
    9489 as core::ffi::c_int as uint16_t,
    9468 as core::ffi::c_int as uint16_t,
    9447 as core::ffi::c_int as uint16_t,
    9425 as core::ffi::c_int as uint16_t,
    9404 as core::ffi::c_int as uint16_t,
    9383 as core::ffi::c_int as uint16_t,
    9362 as core::ffi::c_int as uint16_t,
    9341 as core::ffi::c_int as uint16_t,
    9321 as core::ffi::c_int as uint16_t,
    9300 as core::ffi::c_int as uint16_t,
    9279 as core::ffi::c_int as uint16_t,
    9259 as core::ffi::c_int as uint16_t,
    9239 as core::ffi::c_int as uint16_t,
    9218 as core::ffi::c_int as uint16_t,
    9198 as core::ffi::c_int as uint16_t,
    9178 as core::ffi::c_int as uint16_t,
    9158 as core::ffi::c_int as uint16_t,
    9138 as core::ffi::c_int as uint16_t,
    9118 as core::ffi::c_int as uint16_t,
    9098 as core::ffi::c_int as uint16_t,
    9079 as core::ffi::c_int as uint16_t,
    9059 as core::ffi::c_int as uint16_t,
    9039 as core::ffi::c_int as uint16_t,
    9020 as core::ffi::c_int as uint16_t,
    9001 as core::ffi::c_int as uint16_t,
    8981 as core::ffi::c_int as uint16_t,
    8962 as core::ffi::c_int as uint16_t,
    8943 as core::ffi::c_int as uint16_t,
    8924 as core::ffi::c_int as uint16_t,
    8905 as core::ffi::c_int as uint16_t,
    8886 as core::ffi::c_int as uint16_t,
    8867 as core::ffi::c_int as uint16_t,
    8849 as core::ffi::c_int as uint16_t,
    8830 as core::ffi::c_int as uint16_t,
    8812 as core::ffi::c_int as uint16_t,
    8793 as core::ffi::c_int as uint16_t,
    8775 as core::ffi::c_int as uint16_t,
    8756 as core::ffi::c_int as uint16_t,
    8738 as core::ffi::c_int as uint16_t,
    8720 as core::ffi::c_int as uint16_t,
    8702 as core::ffi::c_int as uint16_t,
    8684 as core::ffi::c_int as uint16_t,
    8666 as core::ffi::c_int as uint16_t,
    8648 as core::ffi::c_int as uint16_t,
    8630 as core::ffi::c_int as uint16_t,
    8613 as core::ffi::c_int as uint16_t,
    8595 as core::ffi::c_int as uint16_t,
    8577 as core::ffi::c_int as uint16_t,
    8560 as core::ffi::c_int as uint16_t,
    8542 as core::ffi::c_int as uint16_t,
    8525 as core::ffi::c_int as uint16_t,
    8508 as core::ffi::c_int as uint16_t,
    8490 as core::ffi::c_int as uint16_t,
    8473 as core::ffi::c_int as uint16_t,
    8456 as core::ffi::c_int as uint16_t,
    8439 as core::ffi::c_int as uint16_t,
    8422 as core::ffi::c_int as uint16_t,
    8405 as core::ffi::c_int as uint16_t,
    8389 as core::ffi::c_int as uint16_t,
    8372 as core::ffi::c_int as uint16_t,
    8355 as core::ffi::c_int as uint16_t,
    8339 as core::ffi::c_int as uint16_t,
    8322 as core::ffi::c_int as uint16_t,
    8306 as core::ffi::c_int as uint16_t,
    8289 as core::ffi::c_int as uint16_t,
    8273 as core::ffi::c_int as uint16_t,
    8257 as core::ffi::c_int as uint16_t,
    8240 as core::ffi::c_int as uint16_t,
    8224 as core::ffi::c_int as uint16_t,
    8208 as core::ffi::c_int as uint16_t,
    8192 as core::ffi::c_int as uint16_t,
];
unsafe extern "C-unwind" fn resolve_divisor_64(
    mut D: uint64_t,
    mut shift: *mut int16_t,
) -> int16_t {
    let mut f: int64_t = 0;
    *shift = (if D >> 32 as core::ffi::c_int != 0 {
        get_msb((D >> 32 as core::ffi::c_int) as core::ffi::c_uint)
            + 32 as core::ffi::c_int
    } else {
        get_msb(D as core::ffi::c_uint)
    }) as int16_t;
    let e: int64_t = D
        .wrapping_sub((1 as core::ffi::c_int as uint64_t) << *shift as core::ffi::c_int)
        as int64_t;
    if *shift as core::ffi::c_int > DIV_LUT_BITS {
        f = e
            + ((1 as core::ffi::c_int as int64_t)
                << *shift as core::ffi::c_int - 8 as core::ffi::c_int
                >> 1 as core::ffi::c_int)
            >> *shift as core::ffi::c_int - 8 as core::ffi::c_int;
    } else {
        f = e << DIV_LUT_BITS - *shift as core::ffi::c_int;
    }
    if f <= ((1 as core::ffi::c_int) << 8 as core::ffi::c_int) as int64_t {} else {
        __assert_fail(
            b"f <= DIV_LUT_NUM\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/common/warped_motion.c\0" as *const u8
                as *const core::ffi::c_char,
            183 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 48],
                [core::ffi::c_char; 48],
            >(*b"int16_t resolve_divisor_64(uint64_t, int16_t *)\0"))
                .as_ptr(),
        );
    };
    *shift = (*shift as core::ffi::c_int + DIV_LUT_PREC_BITS) as int16_t;
    return div_lut[f as usize] as int16_t;
}
unsafe extern "C-unwind" fn resolve_divisor_32(
    mut D: uint32_t,
    mut shift: *mut int16_t,
) -> int16_t {
    let mut f: int32_t = 0;
    *shift = get_msb(D as core::ffi::c_uint) as int16_t;
    let e: int32_t = D
        .wrapping_sub((1 as core::ffi::c_int as uint32_t) << *shift as core::ffi::c_int)
        as int32_t;
    if *shift as core::ffi::c_int > DIV_LUT_BITS {
        f = e
            + ((1 as int32_t) << *shift as core::ffi::c_int - 8 as core::ffi::c_int
                >> 1 as core::ffi::c_int)
            >> *shift as core::ffi::c_int - 8 as core::ffi::c_int;
    } else {
        f = e << DIV_LUT_BITS - *shift as core::ffi::c_int;
    }
    if f <= (1 as int32_t) << 8 as core::ffi::c_int {} else {
        __assert_fail(
            b"f <= DIV_LUT_NUM\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/common/warped_motion.c\0" as *const u8
                as *const core::ffi::c_char,
            199 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 48],
                [core::ffi::c_char; 48],
            >(*b"int16_t resolve_divisor_32(uint32_t, int16_t *)\0"))
                .as_ptr(),
        );
    };
    *shift = (*shift as core::ffi::c_int + DIV_LUT_PREC_BITS) as int16_t;
    return div_lut[f as usize] as int16_t;
}
unsafe extern "C-unwind" fn is_affine_valid(wm: *const WarpedMotionParams) -> core::ffi::c_int {
    let mut mat: *const int32_t = ((*wm).wmmat).as_ptr();
    return (*mat.offset(2 as core::ffi::c_int as isize) > 0 as int32_t)
        as core::ffi::c_int;
}
unsafe extern "C-unwind" fn is_affine_shear_allowed(
    mut alpha: int16_t,
    mut beta: int16_t,
    mut gamma: int16_t,
    mut delta: int16_t,
) -> core::ffi::c_int {
    if 4 as core::ffi::c_int * abs(alpha as core::ffi::c_int)
        + 7 as core::ffi::c_int * abs(beta as core::ffi::c_int)
        >= (1 as core::ffi::c_int) << WARPEDMODEL_PREC_BITS
        || 4 as core::ffi::c_int * abs(gamma as core::ffi::c_int)
            + 4 as core::ffi::c_int * abs(delta as core::ffi::c_int)
            >= (1 as core::ffi::c_int) << WARPEDMODEL_PREC_BITS
    {
        return 0 as core::ffi::c_int
    } else {
        return 1 as core::ffi::c_int
    };
}
unsafe extern "C-unwind" fn check_model_consistency(mut wm: *mut WarpedMotionParams) {
    let mut current_block_12: u64;
    match (*wm).wmtype as core::ffi::c_int {
        0 => {
            if (*wm).wmmat[0 as core::ffi::c_int as usize] == 0 as int32_t {} else {
                __assert_fail(
                    b"wm->wmmat[0] == 0\0" as *const u8 as *const core::ffi::c_char,
                    b"upstream/aom/av1/common/warped_motion.c\0" as *const u8
                        as *const core::ffi::c_char,
                    225 as core::ffi::c_uint,
                    (::core::mem::transmute::<
                        [u8; 51],
                        [core::ffi::c_char; 51],
                    >(*b"void check_model_consistency(WarpedMotionParams *)\0"))
                        .as_ptr(),
                );
            };
            if (*wm).wmmat[1 as core::ffi::c_int as usize] == 0 as int32_t {} else {
                __assert_fail(
                    b"wm->wmmat[1] == 0\0" as *const u8 as *const core::ffi::c_char,
                    b"upstream/aom/av1/common/warped_motion.c\0" as *const u8
                        as *const core::ffi::c_char,
                    226 as core::ffi::c_uint,
                    (::core::mem::transmute::<
                        [u8; 51],
                        [core::ffi::c_char; 51],
                    >(*b"void check_model_consistency(WarpedMotionParams *)\0"))
                        .as_ptr(),
                );
            };
            current_block_12 = 0;
        }
        1 => {
            current_block_12 = 0;
        }
        2 => {
            current_block_12 = 1;
        }
        3 => {
            current_block_12 = 2;
        }
        _ => {
            if 0 as core::ffi::c_int != 0
                && !(b"Bad wmtype\0" as *const u8 as *const core::ffi::c_char).is_null()
            {} else {
                __assert_fail(
                    b"0 && \"Bad wmtype\"\0" as *const u8 as *const core::ffi::c_char,
                    b"upstream/aom/av1/common/warped_motion.c\0" as *const u8
                        as *const core::ffi::c_char,
                    237 as core::ffi::c_uint,
                    (::core::mem::transmute::<
                        [u8; 51],
                        [core::ffi::c_char; 51],
                    >(*b"void check_model_consistency(WarpedMotionParams *)\0"))
                        .as_ptr(),
                );
            };
            current_block_12 = 2;
        }
    }
    match current_block_12 {
        0 => {
            if (*wm).wmmat[2 as core::ffi::c_int as usize]
                == (1 as int32_t) << 16 as core::ffi::c_int
            {} else {
                __assert_fail(
                    b"wm->wmmat[2] == 1 << WARPEDMODEL_PREC_BITS\0" as *const u8
                        as *const core::ffi::c_char,
                    b"upstream/aom/av1/common/warped_motion.c\0" as *const u8
                        as *const core::ffi::c_char,
                    229 as core::ffi::c_uint,
                    (::core::mem::transmute::<
                        [u8; 51],
                        [core::ffi::c_char; 51],
                    >(*b"void check_model_consistency(WarpedMotionParams *)\0"))
                        .as_ptr(),
                );
            };
            if (*wm).wmmat[3 as core::ffi::c_int as usize] == 0 as int32_t {} else {
                __assert_fail(
                    b"wm->wmmat[3] == 0\0" as *const u8 as *const core::ffi::c_char,
                    b"upstream/aom/av1/common/warped_motion.c\0" as *const u8
                        as *const core::ffi::c_char,
                    230 as core::ffi::c_uint,
                    (::core::mem::transmute::<
                        [u8; 51],
                        [core::ffi::c_char; 51],
                    >(*b"void check_model_consistency(WarpedMotionParams *)\0"))
                        .as_ptr(),
                );
            };
            current_block_12 = 1;
        }
        _ => {}
    }
    match current_block_12 {
        1 => {
            if (*wm).wmmat[4 as core::ffi::c_int as usize]
                == -(*wm).wmmat[3 as core::ffi::c_int as usize]
            {} else {
                __assert_fail(
                    b"wm->wmmat[4] == -wm->wmmat[3]\0" as *const u8
                        as *const core::ffi::c_char,
                    b"upstream/aom/av1/common/warped_motion.c\0" as *const u8
                        as *const core::ffi::c_char,
                    233 as core::ffi::c_uint,
                    (::core::mem::transmute::<
                        [u8; 51],
                        [core::ffi::c_char; 51],
                    >(*b"void check_model_consistency(WarpedMotionParams *)\0"))
                        .as_ptr(),
                );
            };
            if (*wm).wmmat[5 as core::ffi::c_int as usize]
                == (*wm).wmmat[2 as core::ffi::c_int as usize]
            {} else {
                __assert_fail(
                    b"wm->wmmat[5] == wm->wmmat[2]\0" as *const u8
                        as *const core::ffi::c_char,
                    b"upstream/aom/av1/common/warped_motion.c\0" as *const u8
                        as *const core::ffi::c_char,
                    234 as core::ffi::c_uint,
                    (::core::mem::transmute::<
                        [u8; 51],
                        [core::ffi::c_char; 51],
                    >(*b"void check_model_consistency(WarpedMotionParams *)\0"))
                        .as_ptr(),
                );
            };
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_get_shear_params(
    mut wm: *mut WarpedMotionParams,
) -> core::ffi::c_int {
    check_model_consistency(wm);
    let mut mat: *const int32_t = ((*wm).wmmat).as_mut_ptr();
    if is_affine_valid(wm) == 0 {
        return 0 as core::ffi::c_int;
    }
    (*wm).alpha = clamp(
        *mat.offset(2 as core::ffi::c_int as isize)
            - ((1 as core::ffi::c_int) << WARPEDMODEL_PREC_BITS),
        INT16_MIN,
        INT16_MAX,
    ) as int16_t;
    (*wm).beta = clamp(*mat.offset(3 as core::ffi::c_int as isize), INT16_MIN, INT16_MAX)
        as int16_t;
    let mut shift: int16_t = 0;
    let mut y: int16_t = (resolve_divisor_32(
        abs(*mat.offset(2 as core::ffi::c_int as isize)) as uint32_t,
        &raw mut shift,
    ) as core::ffi::c_int
        * (if *mat.offset(2 as core::ffi::c_int as isize) < 0 as int32_t {
            -(1 as core::ffi::c_int)
        } else {
            1 as core::ffi::c_int
        })) as int16_t;
    let mut v: int64_t = *mat.offset(4 as core::ffi::c_int as isize) as int64_t
        * ((1 as core::ffi::c_int) << WARPEDMODEL_PREC_BITS) as int64_t * y as int64_t;
    (*wm).gamma = clamp(
        (if v < 0 as int64_t {
            -(-v
                + ((1 as core::ffi::c_int as int64_t) << shift as core::ffi::c_int
                    >> 1 as core::ffi::c_int) >> shift as core::ffi::c_int)
        } else {
            v
                + ((1 as core::ffi::c_int as int64_t) << shift as core::ffi::c_int
                    >> 1 as core::ffi::c_int) >> shift as core::ffi::c_int
        }) as core::ffi::c_int,
        INT16_MIN,
        INT16_MAX,
    ) as int16_t;
    v = *mat.offset(3 as core::ffi::c_int as isize) as int64_t
        * *mat.offset(4 as core::ffi::c_int as isize) as int64_t * y as int64_t;
    (*wm).delta = clamp(
        *mat.offset(5 as core::ffi::c_int as isize)
            - (if v < 0 as int64_t {
                -(-v
                    + ((1 as core::ffi::c_int as int64_t) << shift as core::ffi::c_int
                        >> 1 as core::ffi::c_int) >> shift as core::ffi::c_int)
            } else {
                v
                    + ((1 as core::ffi::c_int as int64_t) << shift as core::ffi::c_int
                        >> 1 as core::ffi::c_int) >> shift as core::ffi::c_int
            }) as core::ffi::c_int - ((1 as core::ffi::c_int) << WARPEDMODEL_PREC_BITS),
        INT16_MIN,
        INT16_MAX,
    ) as int16_t;
    (*wm).alpha = ((if ((*wm).alpha as core::ffi::c_int) < 0 as core::ffi::c_int {
        -(-((*wm).alpha as core::ffi::c_int)
            + ((1 as core::ffi::c_int) << 6 as core::ffi::c_int >> 1 as core::ffi::c_int)
            >> 6 as core::ffi::c_int)
    } else {
        (*wm).alpha as core::ffi::c_int
            + ((1 as core::ffi::c_int) << 6 as core::ffi::c_int >> 1 as core::ffi::c_int)
            >> 6 as core::ffi::c_int
    }) * ((1 as core::ffi::c_int) << WARP_PARAM_REDUCE_BITS)) as int16_t;
    (*wm).beta = ((if ((*wm).beta as core::ffi::c_int) < 0 as core::ffi::c_int {
        -(-((*wm).beta as core::ffi::c_int)
            + ((1 as core::ffi::c_int) << 6 as core::ffi::c_int >> 1 as core::ffi::c_int)
            >> 6 as core::ffi::c_int)
    } else {
        (*wm).beta as core::ffi::c_int
            + ((1 as core::ffi::c_int) << 6 as core::ffi::c_int >> 1 as core::ffi::c_int)
            >> 6 as core::ffi::c_int
    }) * ((1 as core::ffi::c_int) << WARP_PARAM_REDUCE_BITS)) as int16_t;
    (*wm).gamma = ((if ((*wm).gamma as core::ffi::c_int) < 0 as core::ffi::c_int {
        -(-((*wm).gamma as core::ffi::c_int)
            + ((1 as core::ffi::c_int) << 6 as core::ffi::c_int >> 1 as core::ffi::c_int)
            >> 6 as core::ffi::c_int)
    } else {
        (*wm).gamma as core::ffi::c_int
            + ((1 as core::ffi::c_int) << 6 as core::ffi::c_int >> 1 as core::ffi::c_int)
            >> 6 as core::ffi::c_int
    }) * ((1 as core::ffi::c_int) << WARP_PARAM_REDUCE_BITS)) as int16_t;
    (*wm).delta = ((if ((*wm).delta as core::ffi::c_int) < 0 as core::ffi::c_int {
        -(-((*wm).delta as core::ffi::c_int)
            + ((1 as core::ffi::c_int) << 6 as core::ffi::c_int >> 1 as core::ffi::c_int)
            >> 6 as core::ffi::c_int)
    } else {
        (*wm).delta as core::ffi::c_int
            + ((1 as core::ffi::c_int) << 6 as core::ffi::c_int >> 1 as core::ffi::c_int)
            >> 6 as core::ffi::c_int
    }) * ((1 as core::ffi::c_int) << WARP_PARAM_REDUCE_BITS)) as int16_t;
    if is_affine_shear_allowed((*wm).alpha, (*wm).beta, (*wm).gamma, (*wm).delta) == 0 {
        return 0 as core::ffi::c_int;
    }
    return 1 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_highbd_warp_affine_c(
    mut mat: *const int32_t,
    mut ref_0: *const uint16_t,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut stride: core::ffi::c_int,
    mut pred: *mut uint16_t,
    mut p_col: core::ffi::c_int,
    mut p_row: core::ffi::c_int,
    mut p_width: core::ffi::c_int,
    mut p_height: core::ffi::c_int,
    mut p_stride: core::ffi::c_int,
    mut subsampling_x: core::ffi::c_int,
    mut subsampling_y: core::ffi::c_int,
    mut bd: core::ffi::c_int,
    mut conv_params: *mut ConvolveParams,
    mut alpha: int16_t,
    mut beta: int16_t,
    mut gamma: int16_t,
    mut delta: int16_t,
) {
    let mut tmp: [int32_t; 120] = [0; 120];
    let reduce_bits_horiz: core::ffi::c_int = (*conv_params).round_0;
    let reduce_bits_vert: core::ffi::c_int = if (*conv_params).is_compound != 0 {
        (*conv_params).round_1
    } else {
        2 as core::ffi::c_int * FILTER_BITS - reduce_bits_horiz
    };
    let max_bits_horiz: core::ffi::c_int = bd + FILTER_BITS + 1 as core::ffi::c_int
        - reduce_bits_horiz;
    let offset_bits_horiz: core::ffi::c_int = bd + FILTER_BITS - 1 as core::ffi::c_int;
    let offset_bits_vert: core::ffi::c_int = bd + 2 as core::ffi::c_int * FILTER_BITS
        - reduce_bits_horiz;
    let round_bits: core::ffi::c_int = 2 as core::ffi::c_int * FILTER_BITS
        - (*conv_params).round_0 - (*conv_params).round_1;
    let offset_bits: core::ffi::c_int = bd + 2 as core::ffi::c_int * FILTER_BITS
        - (*conv_params).round_0;
    if (*conv_params).is_compound == 0 || !((*conv_params).dst).is_null() {} else {
        __assert_fail(
            b"IMPLIES(conv_params->is_compound, conv_params->dst != NULL)\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/common/warped_motion.c\0" as *const u8
                as *const core::ffi::c_char,
            306 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 186],
                [core::ffi::c_char; 186],
            >(
                *b"void av1_highbd_warp_affine_c(const int32_t *, const uint16_t *, int, int, int, uint16_t *, int, int, int, int, int, int, int, int, ConvolveParams *, int16_t, int16_t, int16_t, int16_t)\0",
            ))
                .as_ptr(),
        );
    };
    if bd + 7 as core::ffi::c_int + 2 as core::ffi::c_int - (*conv_params).round_0
        <= 16 as core::ffi::c_int
    {} else {
        __assert_fail(
            b"bd + FILTER_BITS + 2 - conv_params->round_0 <= 16\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/common/warped_motion.c\0" as *const u8
                as *const core::ffi::c_char,
            310 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 186],
                [core::ffi::c_char; 186],
            >(
                *b"void av1_highbd_warp_affine_c(const int32_t *, const uint16_t *, int, int, int, uint16_t *, int, int, int, int, int, int, int, int, ConvolveParams *, int16_t, int16_t, int16_t, int16_t)\0",
            ))
                .as_ptr(),
        );
    };
    let mut i: core::ffi::c_int = p_row;
    while i < p_row + p_height {
        let mut j: core::ffi::c_int = p_col;
        while j < p_col + p_width {
            let src_x: int32_t = (j as int32_t + 4 as int32_t) << subsampling_x;
            let src_y: int32_t = (i as int32_t + 4 as int32_t) << subsampling_y;
            let dst_x: int64_t = *mat.offset(2 as core::ffi::c_int as isize) as int64_t
                * src_x as int64_t
                + *mat.offset(3 as core::ffi::c_int as isize) as int64_t
                    * src_y as int64_t
                + *mat.offset(0 as core::ffi::c_int as isize) as int64_t;
            let dst_y: int64_t = *mat.offset(4 as core::ffi::c_int as isize) as int64_t
                * src_x as int64_t
                + *mat.offset(5 as core::ffi::c_int as isize) as int64_t
                    * src_y as int64_t
                + *mat.offset(1 as core::ffi::c_int as isize) as int64_t;
            let x4: int64_t = dst_x >> subsampling_x;
            let y4: int64_t = dst_y >> subsampling_y;
            let ix4: int32_t = (x4 >> WARPEDMODEL_PREC_BITS) as int32_t;
            let mut sx4: int32_t = (x4
                & (((1 as core::ffi::c_int) << WARPEDMODEL_PREC_BITS)
                    - 1 as core::ffi::c_int) as int64_t) as int32_t;
            let iy4: int32_t = (y4 >> WARPEDMODEL_PREC_BITS) as int32_t;
            let mut sy4: int32_t = (y4
                & (((1 as core::ffi::c_int) << WARPEDMODEL_PREC_BITS)
                    - 1 as core::ffi::c_int) as int64_t) as int32_t;
            sx4 = (sx4 as core::ffi::c_int
                + (alpha as core::ffi::c_int * -(4 as core::ffi::c_int)
                    + beta as core::ffi::c_int * -(4 as core::ffi::c_int))) as int32_t;
            sy4 = (sy4 as core::ffi::c_int
                + (gamma as core::ffi::c_int * -(4 as core::ffi::c_int)
                    + delta as core::ffi::c_int * -(4 as core::ffi::c_int))) as int32_t;
            sx4 = (sx4 as core::ffi::c_int
                & !(((1 as core::ffi::c_int) << WARP_PARAM_REDUCE_BITS)
                    - 1 as core::ffi::c_int)) as int32_t;
            sy4 = (sy4 as core::ffi::c_int
                & !(((1 as core::ffi::c_int) << WARP_PARAM_REDUCE_BITS)
                    - 1 as core::ffi::c_int)) as int32_t;
            let mut k: core::ffi::c_int = -(7 as core::ffi::c_int);
            while k < 8 as core::ffi::c_int {
                let iy: core::ffi::c_int = clamp(
                    iy4 as core::ffi::c_int + k,
                    0 as core::ffi::c_int,
                    height - 1 as core::ffi::c_int,
                ) as core::ffi::c_int;
                let mut sx: core::ffi::c_int = sx4 as core::ffi::c_int
                    + beta as core::ffi::c_int * (k + 4 as core::ffi::c_int);
                let mut l: core::ffi::c_int = -(4 as core::ffi::c_int);
                while l < 4 as core::ffi::c_int {
                    let mut ix: core::ffi::c_int = ix4 as core::ffi::c_int + l
                        - 3 as core::ffi::c_int;
                    let offs: core::ffi::c_int = (sx
                        + ((1 as core::ffi::c_int)
                            << 16 as core::ffi::c_int - 6 as core::ffi::c_int
                            >> 1 as core::ffi::c_int)
                        >> 16 as core::ffi::c_int - 6 as core::ffi::c_int)
                        + WARPEDPIXEL_PREC_SHIFTS;
                    if offs >= 0 as core::ffi::c_int
                        && offs
                            <= ((1 as core::ffi::c_int) << 6 as core::ffi::c_int)
                                * 3 as core::ffi::c_int
                    {} else {
                        __assert_fail(
                            b"offs >= 0 && offs <= WARPEDPIXEL_PREC_SHIFTS * 3\0"
                                as *const u8 as *const core::ffi::c_char,
                            b"upstream/aom/av1/common/warped_motion.c\0" as *const u8
                                as *const core::ffi::c_char,
                            347 as core::ffi::c_uint,
                            (::core::mem::transmute::<
                                [u8; 186],
                                [core::ffi::c_char; 186],
                            >(
                                *b"void av1_highbd_warp_affine_c(const int32_t *, const uint16_t *, int, int, int, uint16_t *, int, int, int, int, int, int, int, int, ConvolveParams *, int16_t, int16_t, int16_t, int16_t)\0",
                            ))
                                .as_ptr(),
                        );
                    };
                    let mut coeffs: *const WarpedFilterCoeff = (av1_warped_filter[offs
                        as usize])
                        .as_ptr();
                    let mut sum: int32_t = (1 as int32_t) << offset_bits_horiz;
                    let mut m: core::ffi::c_int = 0 as core::ffi::c_int;
                    while m < 8 as core::ffi::c_int {
                        let sample_x: core::ffi::c_int = clamp(
                            ix + m,
                            0 as core::ffi::c_int,
                            width - 1 as core::ffi::c_int,
                        ) as core::ffi::c_int;
                        sum = (sum as core::ffi::c_int
                            + *ref_0.offset((iy * stride + sample_x) as isize)
                                as core::ffi::c_int
                                * *coeffs.offset(m as isize) as core::ffi::c_int)
                            as int32_t;
                        m += 1;
                    }
                    sum = sum
                        + ((1 as int32_t) << reduce_bits_horiz >> 1 as core::ffi::c_int)
                        >> reduce_bits_horiz;
                    if 0 as int32_t <= sum && sum < (1 as int32_t) << max_bits_horiz
                    {} else {
                        __assert_fail(
                            b"0 <= sum && sum < (1 << max_bits_horiz)\0" as *const u8
                                as *const core::ffi::c_char,
                            b"upstream/aom/av1/common/warped_motion.c\0" as *const u8
                                as *const core::ffi::c_char,
                            356 as core::ffi::c_uint,
                            (::core::mem::transmute::<
                                [u8; 186],
                                [core::ffi::c_char; 186],
                            >(
                                *b"void av1_highbd_warp_affine_c(const int32_t *, const uint16_t *, int, int, int, uint16_t *, int, int, int, int, int, int, int, int, ConvolveParams *, int16_t, int16_t, int16_t, int16_t)\0",
                            ))
                                .as_ptr(),
                        );
                    };
                    tmp[((k + 7 as core::ffi::c_int) * 8 as core::ffi::c_int
                        + (l + 4 as core::ffi::c_int)) as usize] = sum;
                    sx += alpha as core::ffi::c_int;
                    l += 1;
                }
                k += 1;
            }
            let mut k_0: core::ffi::c_int = -(4 as core::ffi::c_int);
            while k_0
                < (if (4 as core::ffi::c_int)
                    < p_row + p_height - i - 4 as core::ffi::c_int
                {
                    4 as core::ffi::c_int
                } else {
                    p_row + p_height - i - 4 as core::ffi::c_int
                })
            {
                let mut sy: core::ffi::c_int = sy4 as core::ffi::c_int
                    + delta as core::ffi::c_int * (k_0 + 4 as core::ffi::c_int);
                let mut l_0: core::ffi::c_int = -(4 as core::ffi::c_int);
                while l_0
                    < (if (4 as core::ffi::c_int)
                        < p_col + p_width - j - 4 as core::ffi::c_int
                    {
                        4 as core::ffi::c_int
                    } else {
                        p_col + p_width - j - 4 as core::ffi::c_int
                    })
                {
                    let offs_0: core::ffi::c_int = (sy
                        + ((1 as core::ffi::c_int)
                            << 16 as core::ffi::c_int - 6 as core::ffi::c_int
                            >> 1 as core::ffi::c_int)
                        >> 16 as core::ffi::c_int - 6 as core::ffi::c_int)
                        + WARPEDPIXEL_PREC_SHIFTS;
                    if offs_0 >= 0 as core::ffi::c_int
                        && offs_0
                            <= ((1 as core::ffi::c_int) << 6 as core::ffi::c_int)
                                * 3 as core::ffi::c_int
                    {} else {
                        __assert_fail(
                            b"offs >= 0 && offs <= WARPEDPIXEL_PREC_SHIFTS * 3\0"
                                as *const u8 as *const core::ffi::c_char,
                            b"upstream/aom/av1/common/warped_motion.c\0" as *const u8
                                as *const core::ffi::c_char,
                            368 as core::ffi::c_uint,
                            (::core::mem::transmute::<
                                [u8; 186],
                                [core::ffi::c_char; 186],
                            >(
                                *b"void av1_highbd_warp_affine_c(const int32_t *, const uint16_t *, int, int, int, uint16_t *, int, int, int, int, int, int, int, int, ConvolveParams *, int16_t, int16_t, int16_t, int16_t)\0",
                            ))
                                .as_ptr(),
                        );
                    };
                    let mut coeffs_0: *const WarpedFilterCoeff = (av1_warped_filter[offs_0
                        as usize])
                        .as_ptr();
                    let mut sum_0: int32_t = (1 as int32_t) << offset_bits_vert;
                    let mut m_0: core::ffi::c_int = 0 as core::ffi::c_int;
                    while m_0 < 8 as core::ffi::c_int {
                        sum_0 = (sum_0 as core::ffi::c_int
                            + (tmp[((k_0 + m_0 + 4 as core::ffi::c_int)
                                * 8 as core::ffi::c_int + (l_0 + 4 as core::ffi::c_int))
                                as usize] * *coeffs_0.offset(m_0 as isize) as int32_t)
                                as core::ffi::c_int) as int32_t;
                        m_0 += 1;
                    }
                    if (*conv_params).is_compound != 0 {
                        let mut p: *mut CONV_BUF_TYPE = &raw mut *((*conv_params).dst)
                            .offset(
                                ((i - p_row + k_0 + 4 as core::ffi::c_int)
                                    * (*conv_params).dst_stride
                                    + (j - p_col + l_0 + 4 as core::ffi::c_int)) as isize,
                            ) as *mut CONV_BUF_TYPE;
                        sum_0 = sum_0
                            + ((1 as int32_t) << reduce_bits_vert
                                >> 1 as core::ffi::c_int) >> reduce_bits_vert;
                        if (*conv_params).do_average != 0 {
                            let mut dst16: *mut uint16_t = &raw mut *pred
                                .offset(
                                    ((i - p_row + k_0 + 4 as core::ffi::c_int) * p_stride
                                        + (j - p_col + l_0 + 4 as core::ffi::c_int)) as isize,
                                ) as *mut uint16_t;
                            let mut tmp32: int32_t = *p as int32_t;
                            if (*conv_params).use_dist_wtd_comp_avg != 0 {
                                tmp32 = tmp32 * (*conv_params).fwd_offset as int32_t
                                    + sum_0 * (*conv_params).bck_offset as int32_t;
                                tmp32 = tmp32 >> DIST_PRECISION_BITS;
                            } else {
                                tmp32 += sum_0;
                                tmp32 = tmp32 >> 1 as core::ffi::c_int;
                            }
                            tmp32 = tmp32
                                - ((1 as int32_t) << offset_bits - (*conv_params).round_1)
                                - ((1 as int32_t)
                                    << offset_bits - (*conv_params).round_1
                                        - 1 as core::ffi::c_int);
                            *dst16 = clip_pixel_highbd(
                                tmp32 as core::ffi::c_int
                                    + ((1 as core::ffi::c_int) << round_bits
                                        >> 1 as core::ffi::c_int) >> round_bits,
                                bd,
                            );
                        } else {
                            *p = sum_0 as CONV_BUF_TYPE;
                        }
                    } else {
                        let mut p_0: *mut uint16_t = &raw mut *pred
                            .offset(
                                ((i - p_row + k_0 + 4 as core::ffi::c_int) * p_stride
                                    + (j - p_col + l_0 + 4 as core::ffi::c_int)) as isize,
                            ) as *mut uint16_t;
                        sum_0 = sum_0
                            + ((1 as int32_t) << reduce_bits_vert
                                >> 1 as core::ffi::c_int) >> reduce_bits_vert;
                        if 0 as int32_t <= sum_0
                            && sum_0 < (1 as int32_t) << bd + 2 as core::ffi::c_int
                        {} else {
                            __assert_fail(
                                b"0 <= sum && sum < (1 << (bd + 2))\0" as *const u8
                                    as *const core::ffi::c_char,
                                b"upstream/aom/av1/common/warped_motion.c\0" as *const u8
                                    as *const core::ffi::c_char,
                                405 as core::ffi::c_uint,
                                (::core::mem::transmute::<
                                    [u8; 186],
                                    [core::ffi::c_char; 186],
                                >(
                                    *b"void av1_highbd_warp_affine_c(const int32_t *, const uint16_t *, int, int, int, uint16_t *, int, int, int, int, int, int, int, int, ConvolveParams *, int16_t, int16_t, int16_t, int16_t)\0",
                                ))
                                    .as_ptr(),
                            );
                        };
                        *p_0 = clip_pixel_highbd(
                            sum_0 as core::ffi::c_int
                                - ((1 as core::ffi::c_int) << bd - 1 as core::ffi::c_int)
                                - ((1 as core::ffi::c_int) << bd),
                            bd,
                        );
                    }
                    sy += gamma as core::ffi::c_int;
                    l_0 += 1;
                }
                k_0 += 1;
            }
            j += 8 as core::ffi::c_int;
        }
        i += 8 as core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn highbd_warp_plane(
    mut wm: *mut WarpedMotionParams,
    ref_0: *const uint16_t,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut stride: core::ffi::c_int,
    pred: *mut uint16_t,
    mut p_col: core::ffi::c_int,
    mut p_row: core::ffi::c_int,
    mut p_width: core::ffi::c_int,
    mut p_height: core::ffi::c_int,
    mut p_stride: core::ffi::c_int,
    mut subsampling_x: core::ffi::c_int,
    mut subsampling_y: core::ffi::c_int,
    mut bd: core::ffi::c_int,
    mut conv_params: *mut ConvolveParams,
) {
    let mat: *const int32_t = ((*wm).wmmat).as_mut_ptr();
    let alpha: int16_t = (*wm).alpha;
    let beta: int16_t = (*wm).beta;
    let gamma: int16_t = (*wm).gamma;
    let delta: int16_t = (*wm).delta;
    av1_highbd_warp_affine_c(
        mat,
        ref_0,
        width,
        height,
        stride,
        pred,
        p_col,
        p_row,
        p_width,
        p_height,
        p_stride,
        subsampling_x,
        subsampling_y,
        bd,
        conv_params,
        alpha,
        beta,
        gamma,
        delta,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_warp_affine_c(
    mut mat: *const int32_t,
    mut ref_0: *const uint8_t,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut stride: core::ffi::c_int,
    mut pred: *mut uint8_t,
    mut p_col: core::ffi::c_int,
    mut p_row: core::ffi::c_int,
    mut p_width: core::ffi::c_int,
    mut p_height: core::ffi::c_int,
    mut p_stride: core::ffi::c_int,
    mut subsampling_x: core::ffi::c_int,
    mut subsampling_y: core::ffi::c_int,
    mut conv_params: *mut ConvolveParams,
    mut alpha: int16_t,
    mut beta: int16_t,
    mut gamma: int16_t,
    mut delta: int16_t,
) {
    let mut tmp: [int32_t; 120] = [0; 120];
    let bd: core::ffi::c_int = 8 as core::ffi::c_int;
    let reduce_bits_horiz: core::ffi::c_int = (*conv_params).round_0;
    let reduce_bits_vert: core::ffi::c_int = if (*conv_params).is_compound != 0 {
        (*conv_params).round_1
    } else {
        2 as core::ffi::c_int * FILTER_BITS - reduce_bits_horiz
    };
    let max_bits_horiz: core::ffi::c_int = bd + FILTER_BITS + 1 as core::ffi::c_int
        - reduce_bits_horiz;
    let offset_bits_horiz: core::ffi::c_int = bd + FILTER_BITS - 1 as core::ffi::c_int;
    let offset_bits_vert: core::ffi::c_int = bd + 2 as core::ffi::c_int * FILTER_BITS
        - reduce_bits_horiz;
    let round_bits: core::ffi::c_int = 2 as core::ffi::c_int * FILTER_BITS
        - (*conv_params).round_0 - (*conv_params).round_1;
    let offset_bits: core::ffi::c_int = bd + 2 as core::ffi::c_int * FILTER_BITS
        - (*conv_params).round_0;
    if (*conv_params).is_compound == 0 || !((*conv_params).dst).is_null() {} else {
        __assert_fail(
            b"IMPLIES(conv_params->is_compound, conv_params->dst != NULL)\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/common/warped_motion.c\0" as *const u8
                as *const core::ffi::c_char,
            537 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 172],
                [core::ffi::c_char; 172],
            >(
                *b"void av1_warp_affine_c(const int32_t *, const uint8_t *, int, int, int, uint8_t *, int, int, int, int, int, int, int, ConvolveParams *, int16_t, int16_t, int16_t, int16_t)\0",
            ))
                .as_ptr(),
        );
    };
    if (*conv_params).do_average == 0 || (*conv_params).is_compound != 0 {} else {
        __assert_fail(
            b"IMPLIES(conv_params->do_average, conv_params->is_compound)\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/common/warped_motion.c\0" as *const u8
                as *const core::ffi::c_char,
            538 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 172],
                [core::ffi::c_char; 172],
            >(
                *b"void av1_warp_affine_c(const int32_t *, const uint8_t *, int, int, int, uint8_t *, int, int, int, int, int, int, int, ConvolveParams *, int16_t, int16_t, int16_t, int16_t)\0",
            ))
                .as_ptr(),
        );
    };
    let mut i: core::ffi::c_int = p_row;
    while i < p_row + p_height {
        let mut j: core::ffi::c_int = p_col;
        while j < p_col + p_width {
            let src_x: int32_t = (j as int32_t + 4 as int32_t) << subsampling_x;
            let src_y: int32_t = (i as int32_t + 4 as int32_t) << subsampling_y;
            let dst_x: int64_t = *mat.offset(2 as core::ffi::c_int as isize) as int64_t
                * src_x as int64_t
                + *mat.offset(3 as core::ffi::c_int as isize) as int64_t
                    * src_y as int64_t
                + *mat.offset(0 as core::ffi::c_int as isize) as int64_t;
            let dst_y: int64_t = *mat.offset(4 as core::ffi::c_int as isize) as int64_t
                * src_x as int64_t
                + *mat.offset(5 as core::ffi::c_int as isize) as int64_t
                    * src_y as int64_t
                + *mat.offset(1 as core::ffi::c_int as isize) as int64_t;
            let x4: int64_t = dst_x >> subsampling_x;
            let y4: int64_t = dst_y >> subsampling_y;
            let mut ix4: int32_t = (x4 >> WARPEDMODEL_PREC_BITS) as int32_t;
            let mut sx4: int32_t = (x4
                & (((1 as core::ffi::c_int) << WARPEDMODEL_PREC_BITS)
                    - 1 as core::ffi::c_int) as int64_t) as int32_t;
            let mut iy4: int32_t = (y4 >> WARPEDMODEL_PREC_BITS) as int32_t;
            let mut sy4: int32_t = (y4
                & (((1 as core::ffi::c_int) << WARPEDMODEL_PREC_BITS)
                    - 1 as core::ffi::c_int) as int64_t) as int32_t;
            sx4 = (sx4 as core::ffi::c_int
                + (alpha as core::ffi::c_int * -(4 as core::ffi::c_int)
                    + beta as core::ffi::c_int * -(4 as core::ffi::c_int))) as int32_t;
            sy4 = (sy4 as core::ffi::c_int
                + (gamma as core::ffi::c_int * -(4 as core::ffi::c_int)
                    + delta as core::ffi::c_int * -(4 as core::ffi::c_int))) as int32_t;
            sx4 = (sx4 as core::ffi::c_int
                & !(((1 as core::ffi::c_int) << WARP_PARAM_REDUCE_BITS)
                    - 1 as core::ffi::c_int)) as int32_t;
            sy4 = (sy4 as core::ffi::c_int
                & !(((1 as core::ffi::c_int) << WARP_PARAM_REDUCE_BITS)
                    - 1 as core::ffi::c_int)) as int32_t;
            let mut k: core::ffi::c_int = -(7 as core::ffi::c_int);
            while k < 8 as core::ffi::c_int {
                let iy: core::ffi::c_int = clamp(
                    iy4 as core::ffi::c_int + k,
                    0 as core::ffi::c_int,
                    height - 1 as core::ffi::c_int,
                ) as core::ffi::c_int;
                let mut sx: core::ffi::c_int = sx4 as core::ffi::c_int
                    + beta as core::ffi::c_int * (k + 4 as core::ffi::c_int);
                let mut l: core::ffi::c_int = -(4 as core::ffi::c_int);
                while l < 4 as core::ffi::c_int {
                    let mut ix: core::ffi::c_int = ix4 as core::ffi::c_int + l
                        - 3 as core::ffi::c_int;
                    let offs: core::ffi::c_int = (sx
                        + ((1 as core::ffi::c_int)
                            << 16 as core::ffi::c_int - 6 as core::ffi::c_int
                            >> 1 as core::ffi::c_int)
                        >> 16 as core::ffi::c_int - 6 as core::ffi::c_int)
                        + WARPEDPIXEL_PREC_SHIFTS;
                    if offs >= 0 as core::ffi::c_int
                        && offs
                            <= ((1 as core::ffi::c_int) << 6 as core::ffi::c_int)
                                * 3 as core::ffi::c_int
                    {} else {
                        __assert_fail(
                            b"offs >= 0 && offs <= WARPEDPIXEL_PREC_SHIFTS * 3\0"
                                as *const u8 as *const core::ffi::c_char,
                            b"upstream/aom/av1/common/warped_motion.c\0" as *const u8
                                as *const core::ffi::c_char,
                            578 as core::ffi::c_uint,
                            (::core::mem::transmute::<
                                [u8; 172],
                                [core::ffi::c_char; 172],
                            >(
                                *b"void av1_warp_affine_c(const int32_t *, const uint8_t *, int, int, int, uint8_t *, int, int, int, int, int, int, int, ConvolveParams *, int16_t, int16_t, int16_t, int16_t)\0",
                            ))
                                .as_ptr(),
                        );
                    };
                    let mut coeffs: *const WarpedFilterCoeff = (av1_warped_filter[offs
                        as usize])
                        .as_ptr();
                    let mut sum: int32_t = (1 as int32_t) << offset_bits_horiz;
                    let mut m: core::ffi::c_int = 0 as core::ffi::c_int;
                    while m < 8 as core::ffi::c_int {
                        let sample_x: core::ffi::c_int = clamp(
                            ix + m,
                            0 as core::ffi::c_int,
                            width - 1 as core::ffi::c_int,
                        ) as core::ffi::c_int;
                        sum = (sum as core::ffi::c_int
                            + *ref_0.offset((iy * stride + sample_x) as isize)
                                as core::ffi::c_int
                                * *coeffs.offset(m as isize) as core::ffi::c_int)
                            as int32_t;
                        m += 1;
                    }
                    sum = sum
                        + ((1 as int32_t) << reduce_bits_horiz >> 1 as core::ffi::c_int)
                        >> reduce_bits_horiz;
                    if 0 as int32_t <= sum && sum < (1 as int32_t) << max_bits_horiz
                    {} else {
                        __assert_fail(
                            b"0 <= sum && sum < (1 << max_bits_horiz)\0" as *const u8
                                as *const core::ffi::c_char,
                            b"upstream/aom/av1/common/warped_motion.c\0" as *const u8
                                as *const core::ffi::c_char,
                            589 as core::ffi::c_uint,
                            (::core::mem::transmute::<
                                [u8; 172],
                                [core::ffi::c_char; 172],
                            >(
                                *b"void av1_warp_affine_c(const int32_t *, const uint8_t *, int, int, int, uint8_t *, int, int, int, int, int, int, int, ConvolveParams *, int16_t, int16_t, int16_t, int16_t)\0",
                            ))
                                .as_ptr(),
                        );
                    };
                    tmp[((k + 7 as core::ffi::c_int) * 8 as core::ffi::c_int
                        + (l + 4 as core::ffi::c_int)) as usize] = sum;
                    sx += alpha as core::ffi::c_int;
                    l += 1;
                }
                k += 1;
            }
            let mut k_0: core::ffi::c_int = -(4 as core::ffi::c_int);
            while k_0
                < (if (4 as core::ffi::c_int)
                    < p_row + p_height - i - 4 as core::ffi::c_int
                {
                    4 as core::ffi::c_int
                } else {
                    p_row + p_height - i - 4 as core::ffi::c_int
                })
            {
                let mut sy: core::ffi::c_int = sy4 as core::ffi::c_int
                    + delta as core::ffi::c_int * (k_0 + 4 as core::ffi::c_int);
                let mut l_0: core::ffi::c_int = -(4 as core::ffi::c_int);
                while l_0
                    < (if (4 as core::ffi::c_int)
                        < p_col + p_width - j - 4 as core::ffi::c_int
                    {
                        4 as core::ffi::c_int
                    } else {
                        p_col + p_width - j - 4 as core::ffi::c_int
                    })
                {
                    let offs_0: core::ffi::c_int = (sy
                        + ((1 as core::ffi::c_int)
                            << 16 as core::ffi::c_int - 6 as core::ffi::c_int
                            >> 1 as core::ffi::c_int)
                        >> 16 as core::ffi::c_int - 6 as core::ffi::c_int)
                        + WARPEDPIXEL_PREC_SHIFTS;
                    if offs_0 >= 0 as core::ffi::c_int
                        && offs_0
                            <= ((1 as core::ffi::c_int) << 6 as core::ffi::c_int)
                                * 3 as core::ffi::c_int
                    {} else {
                        __assert_fail(
                            b"offs >= 0 && offs <= WARPEDPIXEL_PREC_SHIFTS * 3\0"
                                as *const u8 as *const core::ffi::c_char,
                            b"upstream/aom/av1/common/warped_motion.c\0" as *const u8
                                as *const core::ffi::c_char,
                            602 as core::ffi::c_uint,
                            (::core::mem::transmute::<
                                [u8; 172],
                                [core::ffi::c_char; 172],
                            >(
                                *b"void av1_warp_affine_c(const int32_t *, const uint8_t *, int, int, int, uint8_t *, int, int, int, int, int, int, int, ConvolveParams *, int16_t, int16_t, int16_t, int16_t)\0",
                            ))
                                .as_ptr(),
                        );
                    };
                    let mut coeffs_0: *const WarpedFilterCoeff = (av1_warped_filter[offs_0
                        as usize])
                        .as_ptr();
                    let mut sum_0: int32_t = (1 as int32_t) << offset_bits_vert;
                    let mut m_0: core::ffi::c_int = 0 as core::ffi::c_int;
                    while m_0 < 8 as core::ffi::c_int {
                        sum_0 = (sum_0 as core::ffi::c_int
                            + (tmp[((k_0 + m_0 + 4 as core::ffi::c_int)
                                * 8 as core::ffi::c_int + (l_0 + 4 as core::ffi::c_int))
                                as usize] * *coeffs_0.offset(m_0 as isize) as int32_t)
                                as core::ffi::c_int) as int32_t;
                        m_0 += 1;
                    }
                    if (*conv_params).is_compound != 0 {
                        let mut p: *mut CONV_BUF_TYPE = &raw mut *((*conv_params).dst)
                            .offset(
                                ((i - p_row + k_0 + 4 as core::ffi::c_int)
                                    * (*conv_params).dst_stride
                                    + (j - p_col + l_0 + 4 as core::ffi::c_int)) as isize,
                            ) as *mut CONV_BUF_TYPE;
                        sum_0 = sum_0
                            + ((1 as int32_t) << reduce_bits_vert
                                >> 1 as core::ffi::c_int) >> reduce_bits_vert;
                        if (*conv_params).do_average != 0 {
                            let mut dst8: *mut uint8_t = &raw mut *pred
                                .offset(
                                    ((i - p_row + k_0 + 4 as core::ffi::c_int) * p_stride
                                        + (j - p_col + l_0 + 4 as core::ffi::c_int)) as isize,
                                ) as *mut uint8_t;
                            let mut tmp32: int32_t = *p as int32_t;
                            if (*conv_params).use_dist_wtd_comp_avg != 0 {
                                tmp32 = tmp32 * (*conv_params).fwd_offset as int32_t
                                    + sum_0 * (*conv_params).bck_offset as int32_t;
                                tmp32 = tmp32 >> DIST_PRECISION_BITS;
                            } else {
                                tmp32 += sum_0;
                                tmp32 = tmp32 >> 1 as core::ffi::c_int;
                            }
                            tmp32 = tmp32
                                - ((1 as int32_t) << offset_bits - (*conv_params).round_1)
                                - ((1 as int32_t)
                                    << offset_bits - (*conv_params).round_1
                                        - 1 as core::ffi::c_int);
                            *dst8 = clip_pixel(
                                tmp32 as core::ffi::c_int
                                    + ((1 as core::ffi::c_int) << round_bits
                                        >> 1 as core::ffi::c_int) >> round_bits,
                            );
                        } else {
                            *p = sum_0 as CONV_BUF_TYPE;
                        }
                    } else {
                        let mut p_0: *mut uint8_t = &raw mut *pred
                            .offset(
                                ((i - p_row + k_0 + 4 as core::ffi::c_int) * p_stride
                                    + (j - p_col + l_0 + 4 as core::ffi::c_int)) as isize,
                            ) as *mut uint8_t;
                        sum_0 = sum_0
                            + ((1 as int32_t) << reduce_bits_vert
                                >> 1 as core::ffi::c_int) >> reduce_bits_vert;
                        if 0 as int32_t <= sum_0
                            && sum_0 < (1 as int32_t) << bd + 2 as core::ffi::c_int
                        {} else {
                            __assert_fail(
                                b"0 <= sum && sum < (1 << (bd + 2))\0" as *const u8
                                    as *const core::ffi::c_char,
                                b"upstream/aom/av1/common/warped_motion.c\0" as *const u8
                                    as *const core::ffi::c_char,
                                638 as core::ffi::c_uint,
                                (::core::mem::transmute::<
                                    [u8; 172],
                                    [core::ffi::c_char; 172],
                                >(
                                    *b"void av1_warp_affine_c(const int32_t *, const uint8_t *, int, int, int, uint8_t *, int, int, int, int, int, int, int, ConvolveParams *, int16_t, int16_t, int16_t, int16_t)\0",
                                ))
                                    .as_ptr(),
                            );
                        };
                        *p_0 = clip_pixel(
                            sum_0 as core::ffi::c_int
                                - ((1 as core::ffi::c_int) << bd - 1 as core::ffi::c_int)
                                - ((1 as core::ffi::c_int) << bd),
                        );
                    }
                    sy += gamma as core::ffi::c_int;
                    l_0 += 1;
                }
                k_0 += 1;
            }
            j += 8 as core::ffi::c_int;
        }
        i += 8 as core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn warp_plane(
    mut wm: *mut WarpedMotionParams,
    ref_0: *const uint8_t,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut stride: core::ffi::c_int,
    mut pred: *mut uint8_t,
    mut p_col: core::ffi::c_int,
    mut p_row: core::ffi::c_int,
    mut p_width: core::ffi::c_int,
    mut p_height: core::ffi::c_int,
    mut p_stride: core::ffi::c_int,
    mut subsampling_x: core::ffi::c_int,
    mut subsampling_y: core::ffi::c_int,
    mut conv_params: *mut ConvolveParams,
) {
    let mat: *const int32_t = ((*wm).wmmat).as_mut_ptr();
    let alpha: int16_t = (*wm).alpha;
    let beta: int16_t = (*wm).beta;
    let gamma: int16_t = (*wm).gamma;
    let delta: int16_t = (*wm).delta;
    av1_warp_affine_c(
        mat,
        ref_0,
        width,
        height,
        stride,
        pred,
        p_col,
        p_row,
        p_width,
        p_height,
        p_stride,
        subsampling_x,
        subsampling_y,
        conv_params,
        alpha,
        beta,
        gamma,
        delta,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_warp_plane(
    mut wm: *mut WarpedMotionParams,
    mut use_hbd: core::ffi::c_int,
    mut bd: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut stride: core::ffi::c_int,
    mut pred: *mut uint8_t,
    mut p_col: core::ffi::c_int,
    mut p_row: core::ffi::c_int,
    mut p_width: core::ffi::c_int,
    mut p_height: core::ffi::c_int,
    mut p_stride: core::ffi::c_int,
    mut subsampling_x: core::ffi::c_int,
    mut subsampling_y: core::ffi::c_int,
    mut conv_params: *mut ConvolveParams,
) {
    if use_hbd != 0 {
        highbd_warp_plane(
            wm,
            ((ref_0 as uintptr_t) << 1 as core::ffi::c_int) as *mut uint16_t,
            width,
            height,
            stride,
            ((pred as uintptr_t) << 1 as core::ffi::c_int) as *mut uint16_t,
            p_col,
            p_row,
            p_width,
            p_height,
            p_stride,
            subsampling_x,
            subsampling_y,
            bd,
            conv_params,
        );
    } else {
        warp_plane(
            wm,
            ref_0,
            width,
            height,
            stride,
            pred,
            p_col,
            p_row,
            p_width,
            p_height,
            p_stride,
            subsampling_x,
            subsampling_y,
            conv_params,
        );
    };
}
pub const LS_MV_MAX: core::ffi::c_int = 256 as core::ffi::c_int;
pub const LS_STEP: core::ffi::c_int = 8 as core::ffi::c_int;
pub const LS_MAT_DOWN_BITS: core::ffi::c_int = 2 as core::ffi::c_int;
unsafe extern "C-unwind" fn get_mult_shift_ndiag(
    mut Px: int64_t,
    mut iDet: int16_t,
    mut shift: core::ffi::c_int,
) -> int32_t {
    let mut v: int64_t = Px * iDet as int64_t;
    return clamp64(
        if v < 0 as int64_t {
            -(-v + ((1 as core::ffi::c_int as int64_t) << shift >> 1 as core::ffi::c_int)
                >> shift)
        } else {
            v + ((1 as core::ffi::c_int as int64_t) << shift >> 1 as core::ffi::c_int)
                >> shift
        },
        (-WARPEDMODEL_NONDIAGAFFINE_CLAMP + 1 as core::ffi::c_int) as int64_t,
        (WARPEDMODEL_NONDIAGAFFINE_CLAMP - 1 as core::ffi::c_int) as int64_t,
    ) as int32_t;
}
unsafe extern "C-unwind" fn get_mult_shift_diag(
    mut Px: int64_t,
    mut iDet: int16_t,
    mut shift: core::ffi::c_int,
) -> int32_t {
    let mut v: int64_t = Px * iDet as int64_t;
    return clamp64(
        if v < 0 as int64_t {
            -(-v + ((1 as core::ffi::c_int as int64_t) << shift >> 1 as core::ffi::c_int)
                >> shift)
        } else {
            v + ((1 as core::ffi::c_int as int64_t) << shift >> 1 as core::ffi::c_int)
                >> shift
        },
        (((1 as core::ffi::c_int) << WARPEDMODEL_PREC_BITS)
            - WARPEDMODEL_NONDIAGAFFINE_CLAMP + 1 as core::ffi::c_int) as int64_t,
        (((1 as core::ffi::c_int) << WARPEDMODEL_PREC_BITS)
            + WARPEDMODEL_NONDIAGAFFINE_CLAMP - 1 as core::ffi::c_int) as int64_t,
    ) as int32_t;
}
unsafe extern "C-unwind" fn find_affine_int(
    mut np: core::ffi::c_int,
    mut pts1: *const core::ffi::c_int,
    mut pts2: *const core::ffi::c_int,
    mut bsize: BLOCK_SIZE,
    mut mvy: core::ffi::c_int,
    mut mvx: core::ffi::c_int,
    mut wm: *mut WarpedMotionParams,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut A: [[int32_t; 2]; 2] = [
        [0 as core::ffi::c_int, 0 as core::ffi::c_int],
        [0 as core::ffi::c_int, 0 as core::ffi::c_int],
    ];
    let mut Bx: [int32_t; 2] = [0 as core::ffi::c_int, 0 as core::ffi::c_int];
    let mut By: [int32_t; 2] = [0 as core::ffi::c_int, 0 as core::ffi::c_int];
    let bw: core::ffi::c_int = block_size_wide[bsize as usize] as core::ffi::c_int;
    let bh: core::ffi::c_int = block_size_high[bsize as usize] as core::ffi::c_int;
    let rsuy: core::ffi::c_int = bh / 2 as core::ffi::c_int - 1 as core::ffi::c_int;
    let rsux: core::ffi::c_int = bw / 2 as core::ffi::c_int - 1 as core::ffi::c_int;
    let suy: core::ffi::c_int = rsuy * 8 as core::ffi::c_int;
    let sux: core::ffi::c_int = rsux * 8 as core::ffi::c_int;
    let duy: core::ffi::c_int = suy + mvy;
    let dux: core::ffi::c_int = sux + mvx;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < np {
        let dx: core::ffi::c_int = *pts2.offset((i * 2 as core::ffi::c_int) as isize)
            - dux;
        let dy: core::ffi::c_int = *pts2
            .offset((i * 2 as core::ffi::c_int + 1 as core::ffi::c_int) as isize) - duy;
        let sx: core::ffi::c_int = *pts1.offset((i * 2 as core::ffi::c_int) as isize)
            - sux;
        let sy: core::ffi::c_int = *pts1
            .offset((i * 2 as core::ffi::c_int + 1 as core::ffi::c_int) as isize) - suy;
        if abs(sx - dx) < LS_MV_MAX && abs(sy - dy) < LS_MV_MAX {
            A[0 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize] = (A[0
                as core::ffi::c_int as usize][0 as core::ffi::c_int as usize]
                as core::ffi::c_int
                + (sx * sx * 4 as core::ffi::c_int + sx * 4 as core::ffi::c_int * LS_STEP
                    + LS_STEP * LS_STEP * 2 as core::ffi::c_int
                    >> 2 as core::ffi::c_int + LS_MAT_DOWN_BITS)) as int32_t;
            A[0 as core::ffi::c_int as usize][1 as core::ffi::c_int as usize] = (A[0
                as core::ffi::c_int as usize][1 as core::ffi::c_int as usize]
                as core::ffi::c_int
                + (sx * sy * 4 as core::ffi::c_int
                    + (sx + sy) * 2 as core::ffi::c_int * LS_STEP + LS_STEP * LS_STEP
                    >> 2 as core::ffi::c_int + LS_MAT_DOWN_BITS)) as int32_t;
            A[1 as core::ffi::c_int as usize][1 as core::ffi::c_int as usize] = (A[1
                as core::ffi::c_int as usize][1 as core::ffi::c_int as usize]
                as core::ffi::c_int
                + (sy * sy * 4 as core::ffi::c_int + sy * 4 as core::ffi::c_int * LS_STEP
                    + LS_STEP * LS_STEP * 2 as core::ffi::c_int
                    >> 2 as core::ffi::c_int + LS_MAT_DOWN_BITS)) as int32_t;
            Bx[0 as core::ffi::c_int as usize] = (Bx[0 as core::ffi::c_int as usize]
                as core::ffi::c_int
                + (sx * dx * 4 as core::ffi::c_int
                    + (sx + dx) * 2 as core::ffi::c_int * LS_STEP
                    + LS_STEP * LS_STEP * 2 as core::ffi::c_int
                    >> 2 as core::ffi::c_int + LS_MAT_DOWN_BITS)) as int32_t;
            Bx[1 as core::ffi::c_int as usize] = (Bx[1 as core::ffi::c_int as usize]
                as core::ffi::c_int
                + (sy * dx * 4 as core::ffi::c_int
                    + (sy + dx) * 2 as core::ffi::c_int * LS_STEP + LS_STEP * LS_STEP
                    >> 2 as core::ffi::c_int + LS_MAT_DOWN_BITS)) as int32_t;
            By[0 as core::ffi::c_int as usize] = (By[0 as core::ffi::c_int as usize]
                as core::ffi::c_int
                + (sx * dy * 4 as core::ffi::c_int
                    + (sx + dy) * 2 as core::ffi::c_int * LS_STEP + LS_STEP * LS_STEP
                    >> 2 as core::ffi::c_int + LS_MAT_DOWN_BITS)) as int32_t;
            By[1 as core::ffi::c_int as usize] = (By[1 as core::ffi::c_int as usize]
                as core::ffi::c_int
                + (sy * dy * 4 as core::ffi::c_int
                    + (sy + dy) * 2 as core::ffi::c_int * LS_STEP
                    + LS_STEP * LS_STEP * 2 as core::ffi::c_int
                    >> 2 as core::ffi::c_int + LS_MAT_DOWN_BITS)) as int32_t;
        }
        i += 1;
    }
    if A[0 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize]
        >= -((1 as int32_t)
            << (7 as core::ffi::c_int + 4 as core::ffi::c_int) * 2 as core::ffi::c_int
                + 3 as core::ffi::c_int - 2 as core::ffi::c_int - 1 as core::ffi::c_int)
        && A[0 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize]
            <= ((1 as int32_t)
                << (7 as core::ffi::c_int + 4 as core::ffi::c_int)
                    * 2 as core::ffi::c_int + 3 as core::ffi::c_int
                    - 2 as core::ffi::c_int - 1 as core::ffi::c_int) - 1 as int32_t
    {} else {
        __assert_fail(
            b"A[0][0] >= LS_MAT_MIN && A[0][0] <= LS_MAT_MAX\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/common/warped_motion.c\0" as *const u8
                as *const core::ffi::c_char,
            855 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 105],
                [core::ffi::c_char; 105],
            >(
                *b"int find_affine_int(int, const int *, const int *, BLOCK_SIZE, int, int, WarpedMotionParams *, int, int)\0",
            ))
                .as_ptr(),
        );
    };
    if A[0 as core::ffi::c_int as usize][1 as core::ffi::c_int as usize]
        >= -((1 as int32_t)
            << (7 as core::ffi::c_int + 4 as core::ffi::c_int) * 2 as core::ffi::c_int
                + 3 as core::ffi::c_int - 2 as core::ffi::c_int - 1 as core::ffi::c_int)
        && A[0 as core::ffi::c_int as usize][1 as core::ffi::c_int as usize]
            <= ((1 as int32_t)
                << (7 as core::ffi::c_int + 4 as core::ffi::c_int)
                    * 2 as core::ffi::c_int + 3 as core::ffi::c_int
                    - 2 as core::ffi::c_int - 1 as core::ffi::c_int) - 1 as int32_t
    {} else {
        __assert_fail(
            b"A[0][1] >= LS_MAT_MIN && A[0][1] <= LS_MAT_MAX\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/common/warped_motion.c\0" as *const u8
                as *const core::ffi::c_char,
            856 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 105],
                [core::ffi::c_char; 105],
            >(
                *b"int find_affine_int(int, const int *, const int *, BLOCK_SIZE, int, int, WarpedMotionParams *, int, int)\0",
            ))
                .as_ptr(),
        );
    };
    if A[1 as core::ffi::c_int as usize][1 as core::ffi::c_int as usize]
        >= -((1 as int32_t)
            << (7 as core::ffi::c_int + 4 as core::ffi::c_int) * 2 as core::ffi::c_int
                + 3 as core::ffi::c_int - 2 as core::ffi::c_int - 1 as core::ffi::c_int)
        && A[1 as core::ffi::c_int as usize][1 as core::ffi::c_int as usize]
            <= ((1 as int32_t)
                << (7 as core::ffi::c_int + 4 as core::ffi::c_int)
                    * 2 as core::ffi::c_int + 3 as core::ffi::c_int
                    - 2 as core::ffi::c_int - 1 as core::ffi::c_int) - 1 as int32_t
    {} else {
        __assert_fail(
            b"A[1][1] >= LS_MAT_MIN && A[1][1] <= LS_MAT_MAX\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/common/warped_motion.c\0" as *const u8
                as *const core::ffi::c_char,
            857 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 105],
                [core::ffi::c_char; 105],
            >(
                *b"int find_affine_int(int, const int *, const int *, BLOCK_SIZE, int, int, WarpedMotionParams *, int, int)\0",
            ))
                .as_ptr(),
        );
    };
    if Bx[0 as core::ffi::c_int as usize]
        >= -((1 as int32_t)
            << (7 as core::ffi::c_int + 4 as core::ffi::c_int) * 2 as core::ffi::c_int
                + 3 as core::ffi::c_int - 2 as core::ffi::c_int - 1 as core::ffi::c_int)
        && Bx[0 as core::ffi::c_int as usize]
            <= ((1 as int32_t)
                << (7 as core::ffi::c_int + 4 as core::ffi::c_int)
                    * 2 as core::ffi::c_int + 3 as core::ffi::c_int
                    - 2 as core::ffi::c_int - 1 as core::ffi::c_int) - 1 as int32_t
    {} else {
        __assert_fail(
            b"Bx[0] >= LS_MAT_MIN && Bx[0] <= LS_MAT_MAX\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/common/warped_motion.c\0" as *const u8
                as *const core::ffi::c_char,
            858 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 105],
                [core::ffi::c_char; 105],
            >(
                *b"int find_affine_int(int, const int *, const int *, BLOCK_SIZE, int, int, WarpedMotionParams *, int, int)\0",
            ))
                .as_ptr(),
        );
    };
    if Bx[1 as core::ffi::c_int as usize]
        >= -((1 as int32_t)
            << (7 as core::ffi::c_int + 4 as core::ffi::c_int) * 2 as core::ffi::c_int
                + 3 as core::ffi::c_int - 2 as core::ffi::c_int - 1 as core::ffi::c_int)
        && Bx[1 as core::ffi::c_int as usize]
            <= ((1 as int32_t)
                << (7 as core::ffi::c_int + 4 as core::ffi::c_int)
                    * 2 as core::ffi::c_int + 3 as core::ffi::c_int
                    - 2 as core::ffi::c_int - 1 as core::ffi::c_int) - 1 as int32_t
    {} else {
        __assert_fail(
            b"Bx[1] >= LS_MAT_MIN && Bx[1] <= LS_MAT_MAX\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/common/warped_motion.c\0" as *const u8
                as *const core::ffi::c_char,
            859 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 105],
                [core::ffi::c_char; 105],
            >(
                *b"int find_affine_int(int, const int *, const int *, BLOCK_SIZE, int, int, WarpedMotionParams *, int, int)\0",
            ))
                .as_ptr(),
        );
    };
    if By[0 as core::ffi::c_int as usize]
        >= -((1 as int32_t)
            << (7 as core::ffi::c_int + 4 as core::ffi::c_int) * 2 as core::ffi::c_int
                + 3 as core::ffi::c_int - 2 as core::ffi::c_int - 1 as core::ffi::c_int)
        && By[0 as core::ffi::c_int as usize]
            <= ((1 as int32_t)
                << (7 as core::ffi::c_int + 4 as core::ffi::c_int)
                    * 2 as core::ffi::c_int + 3 as core::ffi::c_int
                    - 2 as core::ffi::c_int - 1 as core::ffi::c_int) - 1 as int32_t
    {} else {
        __assert_fail(
            b"By[0] >= LS_MAT_MIN && By[0] <= LS_MAT_MAX\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/common/warped_motion.c\0" as *const u8
                as *const core::ffi::c_char,
            860 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 105],
                [core::ffi::c_char; 105],
            >(
                *b"int find_affine_int(int, const int *, const int *, BLOCK_SIZE, int, int, WarpedMotionParams *, int, int)\0",
            ))
                .as_ptr(),
        );
    };
    if By[1 as core::ffi::c_int as usize]
        >= -((1 as int32_t)
            << (7 as core::ffi::c_int + 4 as core::ffi::c_int) * 2 as core::ffi::c_int
                + 3 as core::ffi::c_int - 2 as core::ffi::c_int - 1 as core::ffi::c_int)
        && By[1 as core::ffi::c_int as usize]
            <= ((1 as int32_t)
                << (7 as core::ffi::c_int + 4 as core::ffi::c_int)
                    * 2 as core::ffi::c_int + 3 as core::ffi::c_int
                    - 2 as core::ffi::c_int - 1 as core::ffi::c_int) - 1 as int32_t
    {} else {
        __assert_fail(
            b"By[1] >= LS_MAT_MIN && By[1] <= LS_MAT_MAX\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/common/warped_motion.c\0" as *const u8
                as *const core::ffi::c_char,
            861 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 105],
                [core::ffi::c_char; 105],
            >(
                *b"int find_affine_int(int, const int *, const int *, BLOCK_SIZE, int, int, WarpedMotionParams *, int, int)\0",
            ))
                .as_ptr(),
        );
    };
    let Det: int64_t = A[0 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize]
        as int64_t
        * A[1 as core::ffi::c_int as usize][1 as core::ffi::c_int as usize] as int64_t
        - A[0 as core::ffi::c_int as usize][1 as core::ffi::c_int as usize] as int64_t
            * A[0 as core::ffi::c_int as usize][1 as core::ffi::c_int as usize]
                as int64_t;
    if Det == 0 as int64_t {
        return 1 as core::ffi::c_int;
    }
    let mut shift: int16_t = 0;
    let mut iDet: int16_t = (resolve_divisor_64(
        llabs(Det as core::ffi::c_longlong) as uint64_t,
        &raw mut shift,
    ) as core::ffi::c_int
        * (if Det < 0 as int64_t {
            -(1 as core::ffi::c_int)
        } else {
            1 as core::ffi::c_int
        })) as int16_t;
    shift = (shift as core::ffi::c_int - WARPEDMODEL_PREC_BITS) as int16_t;
    if (shift as core::ffi::c_int) < 0 as core::ffi::c_int {
        iDet = ((iDet as core::ffi::c_int) << -(shift as core::ffi::c_int)) as int16_t;
        shift = 0 as int16_t;
    }
    let mut Px: [int64_t; 2] = [0; 2];
    let mut Py: [int64_t; 2] = [0; 2];
    Px[0 as core::ffi::c_int as usize] = A[1 as core::ffi::c_int
        as usize][1 as core::ffi::c_int as usize] as int64_t
        * Bx[0 as core::ffi::c_int as usize] as int64_t
        - A[0 as core::ffi::c_int as usize][1 as core::ffi::c_int as usize] as int64_t
            * Bx[1 as core::ffi::c_int as usize] as int64_t;
    Px[1 as core::ffi::c_int as usize] = -(A[0 as core::ffi::c_int
        as usize][1 as core::ffi::c_int as usize] as int64_t)
        * Bx[0 as core::ffi::c_int as usize] as int64_t
        + A[0 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize] as int64_t
            * Bx[1 as core::ffi::c_int as usize] as int64_t;
    Py[0 as core::ffi::c_int as usize] = A[1 as core::ffi::c_int
        as usize][1 as core::ffi::c_int as usize] as int64_t
        * By[0 as core::ffi::c_int as usize] as int64_t
        - A[0 as core::ffi::c_int as usize][1 as core::ffi::c_int as usize] as int64_t
            * By[1 as core::ffi::c_int as usize] as int64_t;
    Py[1 as core::ffi::c_int as usize] = -(A[0 as core::ffi::c_int
        as usize][1 as core::ffi::c_int as usize] as int64_t)
        * By[0 as core::ffi::c_int as usize] as int64_t
        + A[0 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize] as int64_t
            * By[1 as core::ffi::c_int as usize] as int64_t;
    (*wm).wmmat[2 as core::ffi::c_int as usize] = get_mult_shift_diag(
        Px[0 as core::ffi::c_int as usize],
        iDet,
        shift as core::ffi::c_int,
    );
    (*wm).wmmat[3 as core::ffi::c_int as usize] = get_mult_shift_ndiag(
        Px[1 as core::ffi::c_int as usize],
        iDet,
        shift as core::ffi::c_int,
    );
    (*wm).wmmat[4 as core::ffi::c_int as usize] = get_mult_shift_ndiag(
        Py[0 as core::ffi::c_int as usize],
        iDet,
        shift as core::ffi::c_int,
    );
    (*wm).wmmat[5 as core::ffi::c_int as usize] = get_mult_shift_diag(
        Py[1 as core::ffi::c_int as usize],
        iDet,
        shift as core::ffi::c_int,
    );
    let isuy: core::ffi::c_int = mi_row * MI_SIZE + rsuy;
    let isux: core::ffi::c_int = mi_col * MI_SIZE + rsux;
    let vx: int32_t = mvx as int32_t
        * ((1 as int32_t) << WARPEDMODEL_PREC_BITS - 3 as core::ffi::c_int)
        - (isux as int32_t
            * ((*wm).wmmat[2 as core::ffi::c_int as usize]
                - ((1 as int32_t) << WARPEDMODEL_PREC_BITS))
            + isuy as int32_t * (*wm).wmmat[3 as core::ffi::c_int as usize]);
    let vy: int32_t = mvy as int32_t
        * ((1 as int32_t) << WARPEDMODEL_PREC_BITS - 3 as core::ffi::c_int)
        - (isux as int32_t * (*wm).wmmat[4 as core::ffi::c_int as usize]
            + isuy as int32_t
                * ((*wm).wmmat[5 as core::ffi::c_int as usize]
                    - ((1 as int32_t) << WARPEDMODEL_PREC_BITS)));
    (*wm).wmmat[0 as core::ffi::c_int as usize] = clamp(
        vx as core::ffi::c_int,
        -WARPEDMODEL_TRANS_CLAMP,
        WARPEDMODEL_TRANS_CLAMP - 1 as core::ffi::c_int,
    ) as int32_t;
    (*wm).wmmat[1 as core::ffi::c_int as usize] = clamp(
        vy as core::ffi::c_int,
        -WARPEDMODEL_TRANS_CLAMP,
        WARPEDMODEL_TRANS_CLAMP - 1 as core::ffi::c_int,
    ) as int32_t;
    return 0 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_find_projection(
    mut np: core::ffi::c_int,
    mut pts1: *const core::ffi::c_int,
    mut pts2: *const core::ffi::c_int,
    mut bsize: BLOCK_SIZE,
    mut mvy: core::ffi::c_int,
    mut mvx: core::ffi::c_int,
    mut wm_params: *mut WarpedMotionParams,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
) -> core::ffi::c_int {
    if (*wm_params).wmtype as core::ffi::c_int == AFFINE as core::ffi::c_int {} else {
        __assert_fail(
            b"wm_params->wmtype == AFFINE\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/common/warped_motion.c\0" as *const u8
                as *const core::ffi::c_char,
            909 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 109],
                [core::ffi::c_char; 109],
            >(
                *b"int av1_find_projection(int, const int *, const int *, BLOCK_SIZE, int, int, WarpedMotionParams *, int, int)\0",
            ))
                .as_ptr(),
        );
    };
    if find_affine_int(np, pts1, pts2, bsize, mvy, mvx, wm_params, mi_row, mi_col) != 0 {
        return 1 as core::ffi::c_int;
    }
    if av1_get_shear_params(wm_params) == 0 {
        return 1 as core::ffi::c_int;
    }
    return 0 as core::ffi::c_int;
}
static mut block_size_wide: [uint8_t; 22] = [
    4 as core::ffi::c_int as uint8_t,
    4 as core::ffi::c_int as uint8_t,
    8 as core::ffi::c_int as uint8_t,
    8 as core::ffi::c_int as uint8_t,
    8 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    32 as core::ffi::c_int as uint8_t,
    32 as core::ffi::c_int as uint8_t,
    32 as core::ffi::c_int as uint8_t,
    64 as core::ffi::c_int as uint8_t,
    64 as core::ffi::c_int as uint8_t,
    64 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    4 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    8 as core::ffi::c_int as uint8_t,
    32 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    64 as core::ffi::c_int as uint8_t,
];
static mut block_size_high: [uint8_t; 22] = [
    4 as core::ffi::c_int as uint8_t,
    8 as core::ffi::c_int as uint8_t,
    4 as core::ffi::c_int as uint8_t,
    8 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    8 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    32 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    32 as core::ffi::c_int as uint8_t,
    64 as core::ffi::c_int as uint8_t,
    32 as core::ffi::c_int as uint8_t,
    64 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    64 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    4 as core::ffi::c_int as uint8_t,
    32 as core::ffi::c_int as uint8_t,
    8 as core::ffi::c_int as uint8_t,
    64 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
];
pub const FILTER_BITS: core::ffi::c_int = 7 as core::ffi::c_int;
pub const WARPEDMODEL_PREC_BITS: core::ffi::c_int = 16 as core::ffi::c_int;
pub const WARPEDMODEL_TRANS_CLAMP: core::ffi::c_int = (128 as core::ffi::c_int)
    << WARPEDMODEL_PREC_BITS;
pub const WARPEDMODEL_NONDIAGAFFINE_CLAMP: core::ffi::c_int = (1 as core::ffi::c_int)
    << WARPEDMODEL_PREC_BITS - 3 as core::ffi::c_int;
pub const WARPEDPIXEL_PREC_BITS: core::ffi::c_int = 6 as core::ffi::c_int;
pub const WARPEDPIXEL_PREC_SHIFTS: core::ffi::c_int = (1 as core::ffi::c_int)
    << WARPEDPIXEL_PREC_BITS;
pub const WARP_PARAM_REDUCE_BITS: core::ffi::c_int = 6 as core::ffi::c_int;

extern "C" {
    fn abs(__x: core::ffi::c_int) -> core::ffi::c_int;
    fn llabs(__x: core::ffi::c_longlong) -> core::ffi::c_longlong;
    fn __assert_fail(
        __assertion: *const core::ffi::c_char,
        __file: *const core::ffi::c_char,
        __line: core::ffi::c_uint,
        __function: *const core::ffi::c_char,
    ) -> !;
}
