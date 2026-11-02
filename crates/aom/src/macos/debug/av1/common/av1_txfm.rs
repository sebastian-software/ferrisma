/*
 * Copyright (c) 2017, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

// Mechanically translated from upstream/aom/av1/common/av1_txfm.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C-unwind" {
}
pub type int8_t = i8;
pub type int32_t = i32;
pub type int64_t = i64;
pub type uint8_t = u8;
pub type C2RustUnnamed = core::ffi::c_uint;
pub const TXFM_TYPE_INVALID: C2RustUnnamed = 13;
pub const TXFM_TYPES: C2RustUnnamed = 12;
pub const TXFM_TYPE_IDENTITY32: C2RustUnnamed = 11;
pub const TXFM_TYPE_IDENTITY16: C2RustUnnamed = 10;
pub const TXFM_TYPE_IDENTITY8: C2RustUnnamed = 9;
pub const TXFM_TYPE_IDENTITY4: C2RustUnnamed = 8;
pub const TXFM_TYPE_ADST16: C2RustUnnamed = 7;
pub const TXFM_TYPE_ADST8: C2RustUnnamed = 6;
pub const TXFM_TYPE_ADST4: C2RustUnnamed = 5;
pub const TXFM_TYPE_DCT64: C2RustUnnamed = 4;
pub const TXFM_TYPE_DCT32: C2RustUnnamed = 3;
pub const TXFM_TYPE_DCT16: C2RustUnnamed = 2;
pub const TXFM_TYPE_DCT8: C2RustUnnamed = 1;
pub const TXFM_TYPE_DCT4: C2RustUnnamed = 0;
pub type TXFM_TYPE = uint8_t;
pub const INT32_MAX: core::ffi::c_int = 2147483647 as core::ffi::c_int;
pub const INT32_MIN: core::ffi::c_int = -INT32_MAX - 1 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn clamp64(
    mut value: int64_t,
    mut low: int64_t,
    mut high: int64_t,
) -> int64_t {
    return if value < low { low } else if value > high { high } else { value };
}
#[inline]
unsafe extern "C-unwind" fn round_shift(
    mut value: int64_t,
    mut bit: core::ffi::c_int,
) -> int32_t {
    if !(bit >= 1 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 12],
                [core::ffi::c_char; 12],
            >(*b"round_shift\0"))
                .as_ptr(),
            b"av1_txfm.h\0" as *const u8 as *const core::ffi::c_char,
            119 as core::ffi::c_int,
            b"bit >= 1\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    return (value + ((1 as int64_t) << bit - 1 as core::ffi::c_int) >> bit) as int32_t;
}
#[no_mangle]
pub static mut av1_cospi_arr_data: [[int32_t; 64]; 4] = [
    [
        1024 as core::ffi::c_int,
        1024 as core::ffi::c_int,
        1023 as core::ffi::c_int,
        1021 as core::ffi::c_int,
        1019 as core::ffi::c_int,
        1016 as core::ffi::c_int,
        1013 as core::ffi::c_int,
        1009 as core::ffi::c_int,
        1004 as core::ffi::c_int,
        999 as core::ffi::c_int,
        993 as core::ffi::c_int,
        987 as core::ffi::c_int,
        980 as core::ffi::c_int,
        972 as core::ffi::c_int,
        964 as core::ffi::c_int,
        955 as core::ffi::c_int,
        946 as core::ffi::c_int,
        936 as core::ffi::c_int,
        926 as core::ffi::c_int,
        915 as core::ffi::c_int,
        903 as core::ffi::c_int,
        891 as core::ffi::c_int,
        878 as core::ffi::c_int,
        865 as core::ffi::c_int,
        851 as core::ffi::c_int,
        837 as core::ffi::c_int,
        822 as core::ffi::c_int,
        807 as core::ffi::c_int,
        792 as core::ffi::c_int,
        775 as core::ffi::c_int,
        759 as core::ffi::c_int,
        742 as core::ffi::c_int,
        724 as core::ffi::c_int,
        706 as core::ffi::c_int,
        688 as core::ffi::c_int,
        669 as core::ffi::c_int,
        650 as core::ffi::c_int,
        630 as core::ffi::c_int,
        610 as core::ffi::c_int,
        590 as core::ffi::c_int,
        569 as core::ffi::c_int,
        548 as core::ffi::c_int,
        526 as core::ffi::c_int,
        505 as core::ffi::c_int,
        483 as core::ffi::c_int,
        460 as core::ffi::c_int,
        438 as core::ffi::c_int,
        415 as core::ffi::c_int,
        392 as core::ffi::c_int,
        369 as core::ffi::c_int,
        345 as core::ffi::c_int,
        321 as core::ffi::c_int,
        297 as core::ffi::c_int,
        273 as core::ffi::c_int,
        249 as core::ffi::c_int,
        224 as core::ffi::c_int,
        200 as core::ffi::c_int,
        175 as core::ffi::c_int,
        150 as core::ffi::c_int,
        125 as core::ffi::c_int,
        100 as core::ffi::c_int,
        75 as core::ffi::c_int,
        50 as core::ffi::c_int,
        25 as core::ffi::c_int,
    ],
    [
        2048 as core::ffi::c_int,
        2047 as core::ffi::c_int,
        2046 as core::ffi::c_int,
        2042 as core::ffi::c_int,
        2038 as core::ffi::c_int,
        2033 as core::ffi::c_int,
        2026 as core::ffi::c_int,
        2018 as core::ffi::c_int,
        2009 as core::ffi::c_int,
        1998 as core::ffi::c_int,
        1987 as core::ffi::c_int,
        1974 as core::ffi::c_int,
        1960 as core::ffi::c_int,
        1945 as core::ffi::c_int,
        1928 as core::ffi::c_int,
        1911 as core::ffi::c_int,
        1892 as core::ffi::c_int,
        1872 as core::ffi::c_int,
        1851 as core::ffi::c_int,
        1829 as core::ffi::c_int,
        1806 as core::ffi::c_int,
        1782 as core::ffi::c_int,
        1757 as core::ffi::c_int,
        1730 as core::ffi::c_int,
        1703 as core::ffi::c_int,
        1674 as core::ffi::c_int,
        1645 as core::ffi::c_int,
        1615 as core::ffi::c_int,
        1583 as core::ffi::c_int,
        1551 as core::ffi::c_int,
        1517 as core::ffi::c_int,
        1483 as core::ffi::c_int,
        1448 as core::ffi::c_int,
        1412 as core::ffi::c_int,
        1375 as core::ffi::c_int,
        1338 as core::ffi::c_int,
        1299 as core::ffi::c_int,
        1260 as core::ffi::c_int,
        1220 as core::ffi::c_int,
        1179 as core::ffi::c_int,
        1138 as core::ffi::c_int,
        1096 as core::ffi::c_int,
        1053 as core::ffi::c_int,
        1009 as core::ffi::c_int,
        965 as core::ffi::c_int,
        921 as core::ffi::c_int,
        876 as core::ffi::c_int,
        830 as core::ffi::c_int,
        784 as core::ffi::c_int,
        737 as core::ffi::c_int,
        690 as core::ffi::c_int,
        642 as core::ffi::c_int,
        595 as core::ffi::c_int,
        546 as core::ffi::c_int,
        498 as core::ffi::c_int,
        449 as core::ffi::c_int,
        400 as core::ffi::c_int,
        350 as core::ffi::c_int,
        301 as core::ffi::c_int,
        251 as core::ffi::c_int,
        201 as core::ffi::c_int,
        151 as core::ffi::c_int,
        100 as core::ffi::c_int,
        50 as core::ffi::c_int,
    ],
    [
        4096 as core::ffi::c_int,
        4095 as core::ffi::c_int,
        4091 as core::ffi::c_int,
        4085 as core::ffi::c_int,
        4076 as core::ffi::c_int,
        4065 as core::ffi::c_int,
        4052 as core::ffi::c_int,
        4036 as core::ffi::c_int,
        4017 as core::ffi::c_int,
        3996 as core::ffi::c_int,
        3973 as core::ffi::c_int,
        3948 as core::ffi::c_int,
        3920 as core::ffi::c_int,
        3889 as core::ffi::c_int,
        3857 as core::ffi::c_int,
        3822 as core::ffi::c_int,
        3784 as core::ffi::c_int,
        3745 as core::ffi::c_int,
        3703 as core::ffi::c_int,
        3659 as core::ffi::c_int,
        3612 as core::ffi::c_int,
        3564 as core::ffi::c_int,
        3513 as core::ffi::c_int,
        3461 as core::ffi::c_int,
        3406 as core::ffi::c_int,
        3349 as core::ffi::c_int,
        3290 as core::ffi::c_int,
        3229 as core::ffi::c_int,
        3166 as core::ffi::c_int,
        3102 as core::ffi::c_int,
        3035 as core::ffi::c_int,
        2967 as core::ffi::c_int,
        2896 as core::ffi::c_int,
        2824 as core::ffi::c_int,
        2751 as core::ffi::c_int,
        2675 as core::ffi::c_int,
        2598 as core::ffi::c_int,
        2520 as core::ffi::c_int,
        2440 as core::ffi::c_int,
        2359 as core::ffi::c_int,
        2276 as core::ffi::c_int,
        2191 as core::ffi::c_int,
        2106 as core::ffi::c_int,
        2019 as core::ffi::c_int,
        1931 as core::ffi::c_int,
        1842 as core::ffi::c_int,
        1751 as core::ffi::c_int,
        1660 as core::ffi::c_int,
        1567 as core::ffi::c_int,
        1474 as core::ffi::c_int,
        1380 as core::ffi::c_int,
        1285 as core::ffi::c_int,
        1189 as core::ffi::c_int,
        1092 as core::ffi::c_int,
        995 as core::ffi::c_int,
        897 as core::ffi::c_int,
        799 as core::ffi::c_int,
        700 as core::ffi::c_int,
        601 as core::ffi::c_int,
        501 as core::ffi::c_int,
        401 as core::ffi::c_int,
        301 as core::ffi::c_int,
        201 as core::ffi::c_int,
        101 as core::ffi::c_int,
    ],
    [
        8192 as core::ffi::c_int,
        8190 as core::ffi::c_int,
        8182 as core::ffi::c_int,
        8170 as core::ffi::c_int,
        8153 as core::ffi::c_int,
        8130 as core::ffi::c_int,
        8103 as core::ffi::c_int,
        8071 as core::ffi::c_int,
        8035 as core::ffi::c_int,
        7993 as core::ffi::c_int,
        7946 as core::ffi::c_int,
        7895 as core::ffi::c_int,
        7839 as core::ffi::c_int,
        7779 as core::ffi::c_int,
        7713 as core::ffi::c_int,
        7643 as core::ffi::c_int,
        7568 as core::ffi::c_int,
        7489 as core::ffi::c_int,
        7405 as core::ffi::c_int,
        7317 as core::ffi::c_int,
        7225 as core::ffi::c_int,
        7128 as core::ffi::c_int,
        7027 as core::ffi::c_int,
        6921 as core::ffi::c_int,
        6811 as core::ffi::c_int,
        6698 as core::ffi::c_int,
        6580 as core::ffi::c_int,
        6458 as core::ffi::c_int,
        6333 as core::ffi::c_int,
        6203 as core::ffi::c_int,
        6070 as core::ffi::c_int,
        5933 as core::ffi::c_int,
        5793 as core::ffi::c_int,
        5649 as core::ffi::c_int,
        5501 as core::ffi::c_int,
        5351 as core::ffi::c_int,
        5197 as core::ffi::c_int,
        5040 as core::ffi::c_int,
        4880 as core::ffi::c_int,
        4717 as core::ffi::c_int,
        4551 as core::ffi::c_int,
        4383 as core::ffi::c_int,
        4212 as core::ffi::c_int,
        4038 as core::ffi::c_int,
        3862 as core::ffi::c_int,
        3683 as core::ffi::c_int,
        3503 as core::ffi::c_int,
        3320 as core::ffi::c_int,
        3135 as core::ffi::c_int,
        2948 as core::ffi::c_int,
        2760 as core::ffi::c_int,
        2570 as core::ffi::c_int,
        2378 as core::ffi::c_int,
        2185 as core::ffi::c_int,
        1990 as core::ffi::c_int,
        1795 as core::ffi::c_int,
        1598 as core::ffi::c_int,
        1401 as core::ffi::c_int,
        1202 as core::ffi::c_int,
        1003 as core::ffi::c_int,
        803 as core::ffi::c_int,
        603 as core::ffi::c_int,
        402 as core::ffi::c_int,
        201 as core::ffi::c_int,
    ],
];
#[no_mangle]
pub static mut av1_sinpi_arr_data: [[int32_t; 5]; 4] = [
    [
        0 as core::ffi::c_int,
        330 as core::ffi::c_int,
        621 as core::ffi::c_int,
        836 as core::ffi::c_int,
        951 as core::ffi::c_int,
    ],
    [
        0 as core::ffi::c_int,
        660 as core::ffi::c_int,
        1241 as core::ffi::c_int,
        1672 as core::ffi::c_int,
        1901 as core::ffi::c_int,
    ],
    [
        0 as core::ffi::c_int,
        1321 as core::ffi::c_int,
        2482 as core::ffi::c_int,
        3344 as core::ffi::c_int,
        3803 as core::ffi::c_int,
    ],
    [
        0 as core::ffi::c_int,
        2642 as core::ffi::c_int,
        4964 as core::ffi::c_int,
        6689 as core::ffi::c_int,
        7606 as core::ffi::c_int,
    ],
];
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_round_shift_array_c(
    mut arr: *mut int32_t,
    mut size: core::ffi::c_int,
    mut bit: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0;
    if bit == 0 as core::ffi::c_int {
        return
    } else if bit > 0 as core::ffi::c_int {
        i = 0 as core::ffi::c_int;
        while i < size {
            *arr.offset(i as isize) = round_shift(
                *arr.offset(i as isize) as int64_t,
                bit,
            );
            i += 1;
        }
    } else {
        i = 0 as core::ffi::c_int;
        while i < size {
            *arr.offset(i as isize) = clamp64(
                ((1 as core::ffi::c_int as int64_t) << -bit)
                    * *arr.offset(i as isize) as int64_t,
                INT32_MIN as int64_t,
                INT32_MAX as int64_t,
            ) as int32_t;
            i += 1;
        }
    };
}
#[no_mangle]
pub static mut av1_txfm_type_ls: [[TXFM_TYPE; 4]; 5] = [
    [
        TXFM_TYPE_DCT4 as core::ffi::c_int as TXFM_TYPE,
        TXFM_TYPE_ADST4 as core::ffi::c_int as TXFM_TYPE,
        TXFM_TYPE_ADST4 as core::ffi::c_int as TXFM_TYPE,
        TXFM_TYPE_IDENTITY4 as core::ffi::c_int as TXFM_TYPE,
    ],
    [
        TXFM_TYPE_DCT8 as core::ffi::c_int as TXFM_TYPE,
        TXFM_TYPE_ADST8 as core::ffi::c_int as TXFM_TYPE,
        TXFM_TYPE_ADST8 as core::ffi::c_int as TXFM_TYPE,
        TXFM_TYPE_IDENTITY8 as core::ffi::c_int as TXFM_TYPE,
    ],
    [
        TXFM_TYPE_DCT16 as core::ffi::c_int as TXFM_TYPE,
        TXFM_TYPE_ADST16 as core::ffi::c_int as TXFM_TYPE,
        TXFM_TYPE_ADST16 as core::ffi::c_int as TXFM_TYPE,
        TXFM_TYPE_IDENTITY16 as core::ffi::c_int as TXFM_TYPE,
    ],
    [
        TXFM_TYPE_DCT32 as core::ffi::c_int as TXFM_TYPE,
        TXFM_TYPE_INVALID as core::ffi::c_int as TXFM_TYPE,
        TXFM_TYPE_INVALID as core::ffi::c_int as TXFM_TYPE,
        TXFM_TYPE_IDENTITY32 as core::ffi::c_int as TXFM_TYPE,
    ],
    [
        TXFM_TYPE_DCT64 as core::ffi::c_int as TXFM_TYPE,
        TXFM_TYPE_INVALID as core::ffi::c_int as TXFM_TYPE,
        TXFM_TYPE_INVALID as core::ffi::c_int as TXFM_TYPE,
        TXFM_TYPE_INVALID as core::ffi::c_int as TXFM_TYPE,
    ],
];
#[no_mangle]
pub static mut av1_txfm_stage_num_list: [int8_t; 12] = [
    4 as core::ffi::c_int as int8_t,
    6 as core::ffi::c_int as int8_t,
    8 as core::ffi::c_int as int8_t,
    10 as core::ffi::c_int as int8_t,
    12 as core::ffi::c_int as int8_t,
    7 as core::ffi::c_int as int8_t,
    8 as core::ffi::c_int as int8_t,
    10 as core::ffi::c_int as int8_t,
    1 as core::ffi::c_int as int8_t,
    1 as core::ffi::c_int as int8_t,
    1 as core::ffi::c_int as int8_t,
    1 as core::ffi::c_int as int8_t,
];
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_range_check_buf(
    mut stage: int32_t,
    mut input: *const int32_t,
    mut buf: *const int32_t,
    mut size: int32_t,
    mut bit: int8_t,
) {}
