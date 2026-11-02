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

// Mechanically translated from upstream/aom/av1/common/av1_inv_txfm1d.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C-unwind" {
    static av1_cospi_arr_data: [[int32_t; 64]; 4];
    static av1_sinpi_arr_data: [[int32_t; 5]; 4];
    fn av1_range_check_buf(
        stage: int32_t,
        input: *const int32_t,
        buf: *const int32_t,
        size: int32_t,
        bit: int8_t,
    );
}
pub type __int8_t = i8;
pub type __int32_t = i32;
pub type __int64_t = i64;
pub type int8_t = __int8_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
#[inline]
unsafe extern "C-unwind" fn clamp64(
    mut value: int64_t,
    mut low: int64_t,
    mut high: int64_t,
) -> int64_t {
    return if value < low { low } else if value > high { high } else { value };
}
static mut cos_bit_min: core::ffi::c_int = 10 as core::ffi::c_int;
pub const NewSqrt2Bits: int32_t = 12 as core::ffi::c_int as int32_t;
static mut NewSqrt2: int32_t = 5793 as int32_t;
#[inline]
unsafe extern "C-unwind" fn cospi_arr(mut n: core::ffi::c_int) -> *const int32_t {
    return (av1_cospi_arr_data[(n - cos_bit_min) as usize]).as_ptr();
}
#[inline]
unsafe extern "C-unwind" fn sinpi_arr(mut n: core::ffi::c_int) -> *const int32_t {
    return (av1_sinpi_arr_data[(n - cos_bit_min) as usize]).as_ptr();
}
#[inline]
unsafe extern "C-unwind" fn range_check_value64(
    mut value: int64_t,
    mut bit: int8_t,
) -> int64_t {
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
unsafe extern "C-unwind" fn half_btf(
    mut w0: int32_t,
    mut in0: int32_t,
    mut w1: int32_t,
    mut in1: int32_t,
    mut bit: core::ffi::c_int,
) -> int32_t {
    let mut result_64: int64_t = (w0 * in0) as int64_t + (w1 * in1) as int64_t;
    let mut intermediate: int64_t = (result_64 as core::ffi::c_longlong
        + ((1 as core::ffi::c_longlong) << bit - 1 as core::ffi::c_int)) as int64_t;
    return (intermediate >> bit) as int32_t;
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
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_idct4(
    mut input: *const int32_t,
    mut output: *mut int32_t,
    mut cos_bit: int8_t,
    mut stage_range: *const int8_t,
) {
    if output != input as *mut int32_t {} else {
        __assert_fail(
            b"output != input\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/common/av1_inv_txfm1d.c\0" as *const u8
                as *const core::ffi::c_char,
            18 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 67],
                [core::ffi::c_char; 67],
            >(*b"void av1_idct4(const int32_t *, int32_t *, int8_t, const int8_t *)\0"))
                .as_ptr(),
        );
    };
    let size: int32_t = 4 as int32_t;
    let mut cospi: *const int32_t = cospi_arr(cos_bit as core::ffi::c_int);
    let mut stage: int32_t = 0 as int32_t;
    let mut bf0: *mut int32_t = 0 as *mut int32_t;
    let mut bf1: *mut int32_t = 0 as *mut int32_t;
    let mut step: [int32_t; 4] = [0; 4];
    stage += 1;
    bf1 = output;
    *bf1.offset(0 as core::ffi::c_int as isize) = *input
        .offset(0 as core::ffi::c_int as isize);
    *bf1.offset(1 as core::ffi::c_int as isize) = *input
        .offset(2 as core::ffi::c_int as isize);
    *bf1.offset(2 as core::ffi::c_int as isize) = *input
        .offset(1 as core::ffi::c_int as isize);
    *bf1.offset(3 as core::ffi::c_int as isize) = *input
        .offset(3 as core::ffi::c_int as isize);
    av1_range_check_buf(stage, input, bf1, size, *stage_range.offset(stage as isize));
    stage += 1;
    bf0 = output;
    bf1 = step.as_mut_ptr();
    *bf1.offset(0 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(0 as core::ffi::c_int as isize),
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(1 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(1 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(0 as core::ffi::c_int as isize),
        -*cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(1 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(2 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(48 as core::ffi::c_int as isize),
        *bf0.offset(2 as core::ffi::c_int as isize),
        -*cospi.offset(16 as core::ffi::c_int as isize),
        *bf0.offset(3 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(3 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(16 as core::ffi::c_int as isize),
        *bf0.offset(2 as core::ffi::c_int as isize),
        *cospi.offset(48 as core::ffi::c_int as isize),
        *bf0.offset(3 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    av1_range_check_buf(stage, input, bf1, size, *stage_range.offset(stage as isize));
    stage += 1;
    bf0 = step.as_mut_ptr();
    bf1 = output;
    *bf1.offset(0 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(0 as core::ffi::c_int as isize)
            + *bf0.offset(3 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(1 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(1 as core::ffi::c_int as isize)
            + *bf0.offset(2 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(2 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(1 as core::ffi::c_int as isize)
            - *bf0.offset(2 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(3 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(0 as core::ffi::c_int as isize)
            - *bf0.offset(3 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_idct8(
    mut input: *const int32_t,
    mut output: *mut int32_t,
    mut cos_bit: int8_t,
    mut stage_range: *const int8_t,
) {
    if output != input as *mut int32_t {} else {
        __assert_fail(
            b"output != input\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/common/av1_inv_txfm1d.c\0" as *const u8
                as *const core::ffi::c_char,
            59 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 67],
                [core::ffi::c_char; 67],
            >(*b"void av1_idct8(const int32_t *, int32_t *, int8_t, const int8_t *)\0"))
                .as_ptr(),
        );
    };
    let size: int32_t = 8 as int32_t;
    let mut cospi: *const int32_t = cospi_arr(cos_bit as core::ffi::c_int);
    let mut stage: int32_t = 0 as int32_t;
    let mut bf0: *mut int32_t = 0 as *mut int32_t;
    let mut bf1: *mut int32_t = 0 as *mut int32_t;
    let mut step: [int32_t; 8] = [0; 8];
    stage += 1;
    bf1 = output;
    *bf1.offset(0 as core::ffi::c_int as isize) = *input
        .offset(0 as core::ffi::c_int as isize);
    *bf1.offset(1 as core::ffi::c_int as isize) = *input
        .offset(4 as core::ffi::c_int as isize);
    *bf1.offset(2 as core::ffi::c_int as isize) = *input
        .offset(2 as core::ffi::c_int as isize);
    *bf1.offset(3 as core::ffi::c_int as isize) = *input
        .offset(6 as core::ffi::c_int as isize);
    *bf1.offset(4 as core::ffi::c_int as isize) = *input
        .offset(1 as core::ffi::c_int as isize);
    *bf1.offset(5 as core::ffi::c_int as isize) = *input
        .offset(5 as core::ffi::c_int as isize);
    *bf1.offset(6 as core::ffi::c_int as isize) = *input
        .offset(3 as core::ffi::c_int as isize);
    *bf1.offset(7 as core::ffi::c_int as isize) = *input
        .offset(7 as core::ffi::c_int as isize);
    av1_range_check_buf(stage, input, bf1, size, *stage_range.offset(stage as isize));
    stage += 1;
    bf0 = output;
    bf1 = step.as_mut_ptr();
    *bf1.offset(0 as core::ffi::c_int as isize) = *bf0
        .offset(0 as core::ffi::c_int as isize);
    *bf1.offset(1 as core::ffi::c_int as isize) = *bf0
        .offset(1 as core::ffi::c_int as isize);
    *bf1.offset(2 as core::ffi::c_int as isize) = *bf0
        .offset(2 as core::ffi::c_int as isize);
    *bf1.offset(3 as core::ffi::c_int as isize) = *bf0
        .offset(3 as core::ffi::c_int as isize);
    *bf1.offset(4 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(56 as core::ffi::c_int as isize),
        *bf0.offset(4 as core::ffi::c_int as isize),
        -*cospi.offset(8 as core::ffi::c_int as isize),
        *bf0.offset(7 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(5 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(24 as core::ffi::c_int as isize),
        *bf0.offset(5 as core::ffi::c_int as isize),
        -*cospi.offset(40 as core::ffi::c_int as isize),
        *bf0.offset(6 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(6 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(40 as core::ffi::c_int as isize),
        *bf0.offset(5 as core::ffi::c_int as isize),
        *cospi.offset(24 as core::ffi::c_int as isize),
        *bf0.offset(6 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(7 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(8 as core::ffi::c_int as isize),
        *bf0.offset(4 as core::ffi::c_int as isize),
        *cospi.offset(56 as core::ffi::c_int as isize),
        *bf0.offset(7 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    av1_range_check_buf(stage, input, bf1, size, *stage_range.offset(stage as isize));
    stage += 1;
    bf0 = step.as_mut_ptr();
    bf1 = output;
    *bf1.offset(0 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(0 as core::ffi::c_int as isize),
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(1 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(1 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(0 as core::ffi::c_int as isize),
        -*cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(1 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(2 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(48 as core::ffi::c_int as isize),
        *bf0.offset(2 as core::ffi::c_int as isize),
        -*cospi.offset(16 as core::ffi::c_int as isize),
        *bf0.offset(3 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(3 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(16 as core::ffi::c_int as isize),
        *bf0.offset(2 as core::ffi::c_int as isize),
        *cospi.offset(48 as core::ffi::c_int as isize),
        *bf0.offset(3 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(4 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(4 as core::ffi::c_int as isize)
            + *bf0.offset(5 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(5 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(4 as core::ffi::c_int as isize)
            - *bf0.offset(5 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(6 as core::ffi::c_int as isize) = clamp_value(
        -*bf0.offset(6 as core::ffi::c_int as isize)
            + *bf0.offset(7 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(7 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(6 as core::ffi::c_int as isize)
            + *bf0.offset(7 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    av1_range_check_buf(stage, input, bf1, size, *stage_range.offset(stage as isize));
    stage += 1;
    bf0 = output;
    bf1 = step.as_mut_ptr();
    *bf1.offset(0 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(0 as core::ffi::c_int as isize)
            + *bf0.offset(3 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(1 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(1 as core::ffi::c_int as isize)
            + *bf0.offset(2 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(2 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(1 as core::ffi::c_int as isize)
            - *bf0.offset(2 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(3 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(0 as core::ffi::c_int as isize)
            - *bf0.offset(3 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(4 as core::ffi::c_int as isize) = *bf0
        .offset(4 as core::ffi::c_int as isize);
    *bf1.offset(5 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(5 as core::ffi::c_int as isize),
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(6 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(6 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(5 as core::ffi::c_int as isize),
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(6 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(7 as core::ffi::c_int as isize) = *bf0
        .offset(7 as core::ffi::c_int as isize);
    av1_range_check_buf(stage, input, bf1, size, *stage_range.offset(stage as isize));
    stage += 1;
    bf0 = step.as_mut_ptr();
    bf1 = output;
    *bf1.offset(0 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(0 as core::ffi::c_int as isize)
            + *bf0.offset(7 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(1 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(1 as core::ffi::c_int as isize)
            + *bf0.offset(6 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(2 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(2 as core::ffi::c_int as isize)
            + *bf0.offset(5 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(3 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(3 as core::ffi::c_int as isize)
            + *bf0.offset(4 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(4 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(3 as core::ffi::c_int as isize)
            - *bf0.offset(4 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(5 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(2 as core::ffi::c_int as isize)
            - *bf0.offset(5 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(6 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(1 as core::ffi::c_int as isize)
            - *bf0.offset(6 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(7 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(0 as core::ffi::c_int as isize)
            - *bf0.offset(7 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_idct16(
    mut input: *const int32_t,
    mut output: *mut int32_t,
    mut cos_bit: int8_t,
    mut stage_range: *const int8_t,
) {
    if output != input as *mut int32_t {} else {
        __assert_fail(
            b"output != input\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/common/av1_inv_txfm1d.c\0" as *const u8
                as *const core::ffi::c_char,
            140 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 68],
                [core::ffi::c_char; 68],
            >(*b"void av1_idct16(const int32_t *, int32_t *, int8_t, const int8_t *)\0"))
                .as_ptr(),
        );
    };
    let size: int32_t = 16 as int32_t;
    let mut cospi: *const int32_t = cospi_arr(cos_bit as core::ffi::c_int);
    let mut stage: int32_t = 0 as int32_t;
    let mut bf0: *mut int32_t = 0 as *mut int32_t;
    let mut bf1: *mut int32_t = 0 as *mut int32_t;
    let mut step: [int32_t; 16] = [0; 16];
    stage += 1;
    bf1 = output;
    *bf1.offset(0 as core::ffi::c_int as isize) = *input
        .offset(0 as core::ffi::c_int as isize);
    *bf1.offset(1 as core::ffi::c_int as isize) = *input
        .offset(8 as core::ffi::c_int as isize);
    *bf1.offset(2 as core::ffi::c_int as isize) = *input
        .offset(4 as core::ffi::c_int as isize);
    *bf1.offset(3 as core::ffi::c_int as isize) = *input
        .offset(12 as core::ffi::c_int as isize);
    *bf1.offset(4 as core::ffi::c_int as isize) = *input
        .offset(2 as core::ffi::c_int as isize);
    *bf1.offset(5 as core::ffi::c_int as isize) = *input
        .offset(10 as core::ffi::c_int as isize);
    *bf1.offset(6 as core::ffi::c_int as isize) = *input
        .offset(6 as core::ffi::c_int as isize);
    *bf1.offset(7 as core::ffi::c_int as isize) = *input
        .offset(14 as core::ffi::c_int as isize);
    *bf1.offset(8 as core::ffi::c_int as isize) = *input
        .offset(1 as core::ffi::c_int as isize);
    *bf1.offset(9 as core::ffi::c_int as isize) = *input
        .offset(9 as core::ffi::c_int as isize);
    *bf1.offset(10 as core::ffi::c_int as isize) = *input
        .offset(5 as core::ffi::c_int as isize);
    *bf1.offset(11 as core::ffi::c_int as isize) = *input
        .offset(13 as core::ffi::c_int as isize);
    *bf1.offset(12 as core::ffi::c_int as isize) = *input
        .offset(3 as core::ffi::c_int as isize);
    *bf1.offset(13 as core::ffi::c_int as isize) = *input
        .offset(11 as core::ffi::c_int as isize);
    *bf1.offset(14 as core::ffi::c_int as isize) = *input
        .offset(7 as core::ffi::c_int as isize);
    *bf1.offset(15 as core::ffi::c_int as isize) = *input
        .offset(15 as core::ffi::c_int as isize);
    av1_range_check_buf(stage, input, bf1, size, *stage_range.offset(stage as isize));
    stage += 1;
    bf0 = output;
    bf1 = step.as_mut_ptr();
    *bf1.offset(0 as core::ffi::c_int as isize) = *bf0
        .offset(0 as core::ffi::c_int as isize);
    *bf1.offset(1 as core::ffi::c_int as isize) = *bf0
        .offset(1 as core::ffi::c_int as isize);
    *bf1.offset(2 as core::ffi::c_int as isize) = *bf0
        .offset(2 as core::ffi::c_int as isize);
    *bf1.offset(3 as core::ffi::c_int as isize) = *bf0
        .offset(3 as core::ffi::c_int as isize);
    *bf1.offset(4 as core::ffi::c_int as isize) = *bf0
        .offset(4 as core::ffi::c_int as isize);
    *bf1.offset(5 as core::ffi::c_int as isize) = *bf0
        .offset(5 as core::ffi::c_int as isize);
    *bf1.offset(6 as core::ffi::c_int as isize) = *bf0
        .offset(6 as core::ffi::c_int as isize);
    *bf1.offset(7 as core::ffi::c_int as isize) = *bf0
        .offset(7 as core::ffi::c_int as isize);
    *bf1.offset(8 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(60 as core::ffi::c_int as isize),
        *bf0.offset(8 as core::ffi::c_int as isize),
        -*cospi.offset(4 as core::ffi::c_int as isize),
        *bf0.offset(15 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(9 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(28 as core::ffi::c_int as isize),
        *bf0.offset(9 as core::ffi::c_int as isize),
        -*cospi.offset(36 as core::ffi::c_int as isize),
        *bf0.offset(14 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(10 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(44 as core::ffi::c_int as isize),
        *bf0.offset(10 as core::ffi::c_int as isize),
        -*cospi.offset(20 as core::ffi::c_int as isize),
        *bf0.offset(13 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(11 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(12 as core::ffi::c_int as isize),
        *bf0.offset(11 as core::ffi::c_int as isize),
        -*cospi.offset(52 as core::ffi::c_int as isize),
        *bf0.offset(12 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(12 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(52 as core::ffi::c_int as isize),
        *bf0.offset(11 as core::ffi::c_int as isize),
        *cospi.offset(12 as core::ffi::c_int as isize),
        *bf0.offset(12 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(13 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(20 as core::ffi::c_int as isize),
        *bf0.offset(10 as core::ffi::c_int as isize),
        *cospi.offset(44 as core::ffi::c_int as isize),
        *bf0.offset(13 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(14 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(36 as core::ffi::c_int as isize),
        *bf0.offset(9 as core::ffi::c_int as isize),
        *cospi.offset(28 as core::ffi::c_int as isize),
        *bf0.offset(14 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(15 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(4 as core::ffi::c_int as isize),
        *bf0.offset(8 as core::ffi::c_int as isize),
        *cospi.offset(60 as core::ffi::c_int as isize),
        *bf0.offset(15 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    av1_range_check_buf(stage, input, bf1, size, *stage_range.offset(stage as isize));
    stage += 1;
    bf0 = step.as_mut_ptr();
    bf1 = output;
    *bf1.offset(0 as core::ffi::c_int as isize) = *bf0
        .offset(0 as core::ffi::c_int as isize);
    *bf1.offset(1 as core::ffi::c_int as isize) = *bf0
        .offset(1 as core::ffi::c_int as isize);
    *bf1.offset(2 as core::ffi::c_int as isize) = *bf0
        .offset(2 as core::ffi::c_int as isize);
    *bf1.offset(3 as core::ffi::c_int as isize) = *bf0
        .offset(3 as core::ffi::c_int as isize);
    *bf1.offset(4 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(56 as core::ffi::c_int as isize),
        *bf0.offset(4 as core::ffi::c_int as isize),
        -*cospi.offset(8 as core::ffi::c_int as isize),
        *bf0.offset(7 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(5 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(24 as core::ffi::c_int as isize),
        *bf0.offset(5 as core::ffi::c_int as isize),
        -*cospi.offset(40 as core::ffi::c_int as isize),
        *bf0.offset(6 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(6 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(40 as core::ffi::c_int as isize),
        *bf0.offset(5 as core::ffi::c_int as isize),
        *cospi.offset(24 as core::ffi::c_int as isize),
        *bf0.offset(6 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(7 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(8 as core::ffi::c_int as isize),
        *bf0.offset(4 as core::ffi::c_int as isize),
        *cospi.offset(56 as core::ffi::c_int as isize),
        *bf0.offset(7 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(8 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(8 as core::ffi::c_int as isize)
            + *bf0.offset(9 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(9 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(8 as core::ffi::c_int as isize)
            - *bf0.offset(9 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(10 as core::ffi::c_int as isize) = clamp_value(
        -*bf0.offset(10 as core::ffi::c_int as isize)
            + *bf0.offset(11 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(11 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(10 as core::ffi::c_int as isize)
            + *bf0.offset(11 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(12 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(12 as core::ffi::c_int as isize)
            + *bf0.offset(13 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(13 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(12 as core::ffi::c_int as isize)
            - *bf0.offset(13 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(14 as core::ffi::c_int as isize) = clamp_value(
        -*bf0.offset(14 as core::ffi::c_int as isize)
            + *bf0.offset(15 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(15 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(14 as core::ffi::c_int as isize)
            + *bf0.offset(15 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    av1_range_check_buf(stage, input, bf1, size, *stage_range.offset(stage as isize));
    stage += 1;
    bf0 = output;
    bf1 = step.as_mut_ptr();
    *bf1.offset(0 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(0 as core::ffi::c_int as isize),
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(1 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(1 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(0 as core::ffi::c_int as isize),
        -*cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(1 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(2 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(48 as core::ffi::c_int as isize),
        *bf0.offset(2 as core::ffi::c_int as isize),
        -*cospi.offset(16 as core::ffi::c_int as isize),
        *bf0.offset(3 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(3 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(16 as core::ffi::c_int as isize),
        *bf0.offset(2 as core::ffi::c_int as isize),
        *cospi.offset(48 as core::ffi::c_int as isize),
        *bf0.offset(3 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(4 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(4 as core::ffi::c_int as isize)
            + *bf0.offset(5 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(5 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(4 as core::ffi::c_int as isize)
            - *bf0.offset(5 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(6 as core::ffi::c_int as isize) = clamp_value(
        -*bf0.offset(6 as core::ffi::c_int as isize)
            + *bf0.offset(7 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(7 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(6 as core::ffi::c_int as isize)
            + *bf0.offset(7 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(8 as core::ffi::c_int as isize) = *bf0
        .offset(8 as core::ffi::c_int as isize);
    *bf1.offset(9 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(16 as core::ffi::c_int as isize),
        *bf0.offset(9 as core::ffi::c_int as isize),
        *cospi.offset(48 as core::ffi::c_int as isize),
        *bf0.offset(14 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(10 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(48 as core::ffi::c_int as isize),
        *bf0.offset(10 as core::ffi::c_int as isize),
        -*cospi.offset(16 as core::ffi::c_int as isize),
        *bf0.offset(13 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(11 as core::ffi::c_int as isize) = *bf0
        .offset(11 as core::ffi::c_int as isize);
    *bf1.offset(12 as core::ffi::c_int as isize) = *bf0
        .offset(12 as core::ffi::c_int as isize);
    *bf1.offset(13 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(16 as core::ffi::c_int as isize),
        *bf0.offset(10 as core::ffi::c_int as isize),
        *cospi.offset(48 as core::ffi::c_int as isize),
        *bf0.offset(13 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(14 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(48 as core::ffi::c_int as isize),
        *bf0.offset(9 as core::ffi::c_int as isize),
        *cospi.offset(16 as core::ffi::c_int as isize),
        *bf0.offset(14 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(15 as core::ffi::c_int as isize) = *bf0
        .offset(15 as core::ffi::c_int as isize);
    av1_range_check_buf(stage, input, bf1, size, *stage_range.offset(stage as isize));
    stage += 1;
    bf0 = step.as_mut_ptr();
    bf1 = output;
    *bf1.offset(0 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(0 as core::ffi::c_int as isize)
            + *bf0.offset(3 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(1 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(1 as core::ffi::c_int as isize)
            + *bf0.offset(2 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(2 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(1 as core::ffi::c_int as isize)
            - *bf0.offset(2 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(3 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(0 as core::ffi::c_int as isize)
            - *bf0.offset(3 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(4 as core::ffi::c_int as isize) = *bf0
        .offset(4 as core::ffi::c_int as isize);
    *bf1.offset(5 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(5 as core::ffi::c_int as isize),
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(6 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(6 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(5 as core::ffi::c_int as isize),
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(6 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(7 as core::ffi::c_int as isize) = *bf0
        .offset(7 as core::ffi::c_int as isize);
    *bf1.offset(8 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(8 as core::ffi::c_int as isize)
            + *bf0.offset(11 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(9 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(9 as core::ffi::c_int as isize)
            + *bf0.offset(10 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(10 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(9 as core::ffi::c_int as isize)
            - *bf0.offset(10 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(11 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(8 as core::ffi::c_int as isize)
            - *bf0.offset(11 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(12 as core::ffi::c_int as isize) = clamp_value(
        -*bf0.offset(12 as core::ffi::c_int as isize)
            + *bf0.offset(15 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(13 as core::ffi::c_int as isize) = clamp_value(
        -*bf0.offset(13 as core::ffi::c_int as isize)
            + *bf0.offset(14 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(14 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(13 as core::ffi::c_int as isize)
            + *bf0.offset(14 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(15 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(12 as core::ffi::c_int as isize)
            + *bf0.offset(15 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    av1_range_check_buf(stage, input, bf1, size, *stage_range.offset(stage as isize));
    stage += 1;
    bf0 = output;
    bf1 = step.as_mut_ptr();
    *bf1.offset(0 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(0 as core::ffi::c_int as isize)
            + *bf0.offset(7 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(1 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(1 as core::ffi::c_int as isize)
            + *bf0.offset(6 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(2 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(2 as core::ffi::c_int as isize)
            + *bf0.offset(5 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(3 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(3 as core::ffi::c_int as isize)
            + *bf0.offset(4 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(4 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(3 as core::ffi::c_int as isize)
            - *bf0.offset(4 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(5 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(2 as core::ffi::c_int as isize)
            - *bf0.offset(5 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(6 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(1 as core::ffi::c_int as isize)
            - *bf0.offset(6 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(7 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(0 as core::ffi::c_int as isize)
            - *bf0.offset(7 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(8 as core::ffi::c_int as isize) = *bf0
        .offset(8 as core::ffi::c_int as isize);
    *bf1.offset(9 as core::ffi::c_int as isize) = *bf0
        .offset(9 as core::ffi::c_int as isize);
    *bf1.offset(10 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(10 as core::ffi::c_int as isize),
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(13 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(11 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(11 as core::ffi::c_int as isize),
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(12 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(12 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(11 as core::ffi::c_int as isize),
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(12 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(13 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(10 as core::ffi::c_int as isize),
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(13 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(14 as core::ffi::c_int as isize) = *bf0
        .offset(14 as core::ffi::c_int as isize);
    *bf1.offset(15 as core::ffi::c_int as isize) = *bf0
        .offset(15 as core::ffi::c_int as isize);
    av1_range_check_buf(stage, input, bf1, size, *stage_range.offset(stage as isize));
    stage += 1;
    bf0 = step.as_mut_ptr();
    bf1 = output;
    *bf1.offset(0 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(0 as core::ffi::c_int as isize)
            + *bf0.offset(15 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(1 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(1 as core::ffi::c_int as isize)
            + *bf0.offset(14 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(2 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(2 as core::ffi::c_int as isize)
            + *bf0.offset(13 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(3 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(3 as core::ffi::c_int as isize)
            + *bf0.offset(12 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(4 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(4 as core::ffi::c_int as isize)
            + *bf0.offset(11 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(5 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(5 as core::ffi::c_int as isize)
            + *bf0.offset(10 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(6 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(6 as core::ffi::c_int as isize)
            + *bf0.offset(9 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(7 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(7 as core::ffi::c_int as isize)
            + *bf0.offset(8 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(8 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(7 as core::ffi::c_int as isize)
            - *bf0.offset(8 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(9 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(6 as core::ffi::c_int as isize)
            - *bf0.offset(9 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(10 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(5 as core::ffi::c_int as isize)
            - *bf0.offset(10 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(11 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(4 as core::ffi::c_int as isize)
            - *bf0.offset(11 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(12 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(3 as core::ffi::c_int as isize)
            - *bf0.offset(12 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(13 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(2 as core::ffi::c_int as isize)
            - *bf0.offset(13 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(14 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(1 as core::ffi::c_int as isize)
            - *bf0.offset(14 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(15 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(0 as core::ffi::c_int as isize)
            - *bf0.offset(15 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_idct32(
    mut input: *const int32_t,
    mut output: *mut int32_t,
    mut cos_bit: int8_t,
    mut stage_range: *const int8_t,
) {
    if output != input as *mut int32_t {} else {
        __assert_fail(
            b"output != input\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/common/av1_inv_txfm1d.c\0" as *const u8
                as *const core::ffi::c_char,
            305 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 68],
                [core::ffi::c_char; 68],
            >(*b"void av1_idct32(const int32_t *, int32_t *, int8_t, const int8_t *)\0"))
                .as_ptr(),
        );
    };
    let size: int32_t = 32 as int32_t;
    let mut cospi: *const int32_t = cospi_arr(cos_bit as core::ffi::c_int);
    let mut stage: int32_t = 0 as int32_t;
    let mut bf0: *mut int32_t = 0 as *mut int32_t;
    let mut bf1: *mut int32_t = 0 as *mut int32_t;
    let mut step: [int32_t; 32] = [0; 32];
    stage += 1;
    bf1 = output;
    *bf1.offset(0 as core::ffi::c_int as isize) = *input
        .offset(0 as core::ffi::c_int as isize);
    *bf1.offset(1 as core::ffi::c_int as isize) = *input
        .offset(16 as core::ffi::c_int as isize);
    *bf1.offset(2 as core::ffi::c_int as isize) = *input
        .offset(8 as core::ffi::c_int as isize);
    *bf1.offset(3 as core::ffi::c_int as isize) = *input
        .offset(24 as core::ffi::c_int as isize);
    *bf1.offset(4 as core::ffi::c_int as isize) = *input
        .offset(4 as core::ffi::c_int as isize);
    *bf1.offset(5 as core::ffi::c_int as isize) = *input
        .offset(20 as core::ffi::c_int as isize);
    *bf1.offset(6 as core::ffi::c_int as isize) = *input
        .offset(12 as core::ffi::c_int as isize);
    *bf1.offset(7 as core::ffi::c_int as isize) = *input
        .offset(28 as core::ffi::c_int as isize);
    *bf1.offset(8 as core::ffi::c_int as isize) = *input
        .offset(2 as core::ffi::c_int as isize);
    *bf1.offset(9 as core::ffi::c_int as isize) = *input
        .offset(18 as core::ffi::c_int as isize);
    *bf1.offset(10 as core::ffi::c_int as isize) = *input
        .offset(10 as core::ffi::c_int as isize);
    *bf1.offset(11 as core::ffi::c_int as isize) = *input
        .offset(26 as core::ffi::c_int as isize);
    *bf1.offset(12 as core::ffi::c_int as isize) = *input
        .offset(6 as core::ffi::c_int as isize);
    *bf1.offset(13 as core::ffi::c_int as isize) = *input
        .offset(22 as core::ffi::c_int as isize);
    *bf1.offset(14 as core::ffi::c_int as isize) = *input
        .offset(14 as core::ffi::c_int as isize);
    *bf1.offset(15 as core::ffi::c_int as isize) = *input
        .offset(30 as core::ffi::c_int as isize);
    *bf1.offset(16 as core::ffi::c_int as isize) = *input
        .offset(1 as core::ffi::c_int as isize);
    *bf1.offset(17 as core::ffi::c_int as isize) = *input
        .offset(17 as core::ffi::c_int as isize);
    *bf1.offset(18 as core::ffi::c_int as isize) = *input
        .offset(9 as core::ffi::c_int as isize);
    *bf1.offset(19 as core::ffi::c_int as isize) = *input
        .offset(25 as core::ffi::c_int as isize);
    *bf1.offset(20 as core::ffi::c_int as isize) = *input
        .offset(5 as core::ffi::c_int as isize);
    *bf1.offset(21 as core::ffi::c_int as isize) = *input
        .offset(21 as core::ffi::c_int as isize);
    *bf1.offset(22 as core::ffi::c_int as isize) = *input
        .offset(13 as core::ffi::c_int as isize);
    *bf1.offset(23 as core::ffi::c_int as isize) = *input
        .offset(29 as core::ffi::c_int as isize);
    *bf1.offset(24 as core::ffi::c_int as isize) = *input
        .offset(3 as core::ffi::c_int as isize);
    *bf1.offset(25 as core::ffi::c_int as isize) = *input
        .offset(19 as core::ffi::c_int as isize);
    *bf1.offset(26 as core::ffi::c_int as isize) = *input
        .offset(11 as core::ffi::c_int as isize);
    *bf1.offset(27 as core::ffi::c_int as isize) = *input
        .offset(27 as core::ffi::c_int as isize);
    *bf1.offset(28 as core::ffi::c_int as isize) = *input
        .offset(7 as core::ffi::c_int as isize);
    *bf1.offset(29 as core::ffi::c_int as isize) = *input
        .offset(23 as core::ffi::c_int as isize);
    *bf1.offset(30 as core::ffi::c_int as isize) = *input
        .offset(15 as core::ffi::c_int as isize);
    *bf1.offset(31 as core::ffi::c_int as isize) = *input
        .offset(31 as core::ffi::c_int as isize);
    av1_range_check_buf(stage, input, bf1, size, *stage_range.offset(stage as isize));
    stage += 1;
    bf0 = output;
    bf1 = step.as_mut_ptr();
    *bf1.offset(0 as core::ffi::c_int as isize) = *bf0
        .offset(0 as core::ffi::c_int as isize);
    *bf1.offset(1 as core::ffi::c_int as isize) = *bf0
        .offset(1 as core::ffi::c_int as isize);
    *bf1.offset(2 as core::ffi::c_int as isize) = *bf0
        .offset(2 as core::ffi::c_int as isize);
    *bf1.offset(3 as core::ffi::c_int as isize) = *bf0
        .offset(3 as core::ffi::c_int as isize);
    *bf1.offset(4 as core::ffi::c_int as isize) = *bf0
        .offset(4 as core::ffi::c_int as isize);
    *bf1.offset(5 as core::ffi::c_int as isize) = *bf0
        .offset(5 as core::ffi::c_int as isize);
    *bf1.offset(6 as core::ffi::c_int as isize) = *bf0
        .offset(6 as core::ffi::c_int as isize);
    *bf1.offset(7 as core::ffi::c_int as isize) = *bf0
        .offset(7 as core::ffi::c_int as isize);
    *bf1.offset(8 as core::ffi::c_int as isize) = *bf0
        .offset(8 as core::ffi::c_int as isize);
    *bf1.offset(9 as core::ffi::c_int as isize) = *bf0
        .offset(9 as core::ffi::c_int as isize);
    *bf1.offset(10 as core::ffi::c_int as isize) = *bf0
        .offset(10 as core::ffi::c_int as isize);
    *bf1.offset(11 as core::ffi::c_int as isize) = *bf0
        .offset(11 as core::ffi::c_int as isize);
    *bf1.offset(12 as core::ffi::c_int as isize) = *bf0
        .offset(12 as core::ffi::c_int as isize);
    *bf1.offset(13 as core::ffi::c_int as isize) = *bf0
        .offset(13 as core::ffi::c_int as isize);
    *bf1.offset(14 as core::ffi::c_int as isize) = *bf0
        .offset(14 as core::ffi::c_int as isize);
    *bf1.offset(15 as core::ffi::c_int as isize) = *bf0
        .offset(15 as core::ffi::c_int as isize);
    *bf1.offset(16 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(62 as core::ffi::c_int as isize),
        *bf0.offset(16 as core::ffi::c_int as isize),
        -*cospi.offset(2 as core::ffi::c_int as isize),
        *bf0.offset(31 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(17 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(30 as core::ffi::c_int as isize),
        *bf0.offset(17 as core::ffi::c_int as isize),
        -*cospi.offset(34 as core::ffi::c_int as isize),
        *bf0.offset(30 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(18 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(46 as core::ffi::c_int as isize),
        *bf0.offset(18 as core::ffi::c_int as isize),
        -*cospi.offset(18 as core::ffi::c_int as isize),
        *bf0.offset(29 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(19 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(14 as core::ffi::c_int as isize),
        *bf0.offset(19 as core::ffi::c_int as isize),
        -*cospi.offset(50 as core::ffi::c_int as isize),
        *bf0.offset(28 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(20 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(54 as core::ffi::c_int as isize),
        *bf0.offset(20 as core::ffi::c_int as isize),
        -*cospi.offset(10 as core::ffi::c_int as isize),
        *bf0.offset(27 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(21 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(22 as core::ffi::c_int as isize),
        *bf0.offset(21 as core::ffi::c_int as isize),
        -*cospi.offset(42 as core::ffi::c_int as isize),
        *bf0.offset(26 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(22 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(38 as core::ffi::c_int as isize),
        *bf0.offset(22 as core::ffi::c_int as isize),
        -*cospi.offset(26 as core::ffi::c_int as isize),
        *bf0.offset(25 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(23 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(6 as core::ffi::c_int as isize),
        *bf0.offset(23 as core::ffi::c_int as isize),
        -*cospi.offset(58 as core::ffi::c_int as isize),
        *bf0.offset(24 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(24 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(58 as core::ffi::c_int as isize),
        *bf0.offset(23 as core::ffi::c_int as isize),
        *cospi.offset(6 as core::ffi::c_int as isize),
        *bf0.offset(24 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(25 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(26 as core::ffi::c_int as isize),
        *bf0.offset(22 as core::ffi::c_int as isize),
        *cospi.offset(38 as core::ffi::c_int as isize),
        *bf0.offset(25 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(26 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(42 as core::ffi::c_int as isize),
        *bf0.offset(21 as core::ffi::c_int as isize),
        *cospi.offset(22 as core::ffi::c_int as isize),
        *bf0.offset(26 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(27 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(10 as core::ffi::c_int as isize),
        *bf0.offset(20 as core::ffi::c_int as isize),
        *cospi.offset(54 as core::ffi::c_int as isize),
        *bf0.offset(27 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(28 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(50 as core::ffi::c_int as isize),
        *bf0.offset(19 as core::ffi::c_int as isize),
        *cospi.offset(14 as core::ffi::c_int as isize),
        *bf0.offset(28 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(29 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(18 as core::ffi::c_int as isize),
        *bf0.offset(18 as core::ffi::c_int as isize),
        *cospi.offset(46 as core::ffi::c_int as isize),
        *bf0.offset(29 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(30 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(34 as core::ffi::c_int as isize),
        *bf0.offset(17 as core::ffi::c_int as isize),
        *cospi.offset(30 as core::ffi::c_int as isize),
        *bf0.offset(30 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(31 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(2 as core::ffi::c_int as isize),
        *bf0.offset(16 as core::ffi::c_int as isize),
        *cospi.offset(62 as core::ffi::c_int as isize),
        *bf0.offset(31 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    av1_range_check_buf(stage, input, bf1, size, *stage_range.offset(stage as isize));
    stage += 1;
    bf0 = step.as_mut_ptr();
    bf1 = output;
    *bf1.offset(0 as core::ffi::c_int as isize) = *bf0
        .offset(0 as core::ffi::c_int as isize);
    *bf1.offset(1 as core::ffi::c_int as isize) = *bf0
        .offset(1 as core::ffi::c_int as isize);
    *bf1.offset(2 as core::ffi::c_int as isize) = *bf0
        .offset(2 as core::ffi::c_int as isize);
    *bf1.offset(3 as core::ffi::c_int as isize) = *bf0
        .offset(3 as core::ffi::c_int as isize);
    *bf1.offset(4 as core::ffi::c_int as isize) = *bf0
        .offset(4 as core::ffi::c_int as isize);
    *bf1.offset(5 as core::ffi::c_int as isize) = *bf0
        .offset(5 as core::ffi::c_int as isize);
    *bf1.offset(6 as core::ffi::c_int as isize) = *bf0
        .offset(6 as core::ffi::c_int as isize);
    *bf1.offset(7 as core::ffi::c_int as isize) = *bf0
        .offset(7 as core::ffi::c_int as isize);
    *bf1.offset(8 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(60 as core::ffi::c_int as isize),
        *bf0.offset(8 as core::ffi::c_int as isize),
        -*cospi.offset(4 as core::ffi::c_int as isize),
        *bf0.offset(15 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(9 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(28 as core::ffi::c_int as isize),
        *bf0.offset(9 as core::ffi::c_int as isize),
        -*cospi.offset(36 as core::ffi::c_int as isize),
        *bf0.offset(14 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(10 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(44 as core::ffi::c_int as isize),
        *bf0.offset(10 as core::ffi::c_int as isize),
        -*cospi.offset(20 as core::ffi::c_int as isize),
        *bf0.offset(13 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(11 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(12 as core::ffi::c_int as isize),
        *bf0.offset(11 as core::ffi::c_int as isize),
        -*cospi.offset(52 as core::ffi::c_int as isize),
        *bf0.offset(12 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(12 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(52 as core::ffi::c_int as isize),
        *bf0.offset(11 as core::ffi::c_int as isize),
        *cospi.offset(12 as core::ffi::c_int as isize),
        *bf0.offset(12 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(13 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(20 as core::ffi::c_int as isize),
        *bf0.offset(10 as core::ffi::c_int as isize),
        *cospi.offset(44 as core::ffi::c_int as isize),
        *bf0.offset(13 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(14 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(36 as core::ffi::c_int as isize),
        *bf0.offset(9 as core::ffi::c_int as isize),
        *cospi.offset(28 as core::ffi::c_int as isize),
        *bf0.offset(14 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(15 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(4 as core::ffi::c_int as isize),
        *bf0.offset(8 as core::ffi::c_int as isize),
        *cospi.offset(60 as core::ffi::c_int as isize),
        *bf0.offset(15 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(16 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(16 as core::ffi::c_int as isize)
            + *bf0.offset(17 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(17 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(16 as core::ffi::c_int as isize)
            - *bf0.offset(17 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(18 as core::ffi::c_int as isize) = clamp_value(
        -*bf0.offset(18 as core::ffi::c_int as isize)
            + *bf0.offset(19 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(19 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(18 as core::ffi::c_int as isize)
            + *bf0.offset(19 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(20 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(20 as core::ffi::c_int as isize)
            + *bf0.offset(21 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(21 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(20 as core::ffi::c_int as isize)
            - *bf0.offset(21 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(22 as core::ffi::c_int as isize) = clamp_value(
        -*bf0.offset(22 as core::ffi::c_int as isize)
            + *bf0.offset(23 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(23 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(22 as core::ffi::c_int as isize)
            + *bf0.offset(23 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(24 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(24 as core::ffi::c_int as isize)
            + *bf0.offset(25 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(25 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(24 as core::ffi::c_int as isize)
            - *bf0.offset(25 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(26 as core::ffi::c_int as isize) = clamp_value(
        -*bf0.offset(26 as core::ffi::c_int as isize)
            + *bf0.offset(27 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(27 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(26 as core::ffi::c_int as isize)
            + *bf0.offset(27 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(28 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(28 as core::ffi::c_int as isize)
            + *bf0.offset(29 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(29 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(28 as core::ffi::c_int as isize)
            - *bf0.offset(29 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(30 as core::ffi::c_int as isize) = clamp_value(
        -*bf0.offset(30 as core::ffi::c_int as isize)
            + *bf0.offset(31 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(31 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(30 as core::ffi::c_int as isize)
            + *bf0.offset(31 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    av1_range_check_buf(stage, input, bf1, size, *stage_range.offset(stage as isize));
    stage += 1;
    bf0 = output;
    bf1 = step.as_mut_ptr();
    *bf1.offset(0 as core::ffi::c_int as isize) = *bf0
        .offset(0 as core::ffi::c_int as isize);
    *bf1.offset(1 as core::ffi::c_int as isize) = *bf0
        .offset(1 as core::ffi::c_int as isize);
    *bf1.offset(2 as core::ffi::c_int as isize) = *bf0
        .offset(2 as core::ffi::c_int as isize);
    *bf1.offset(3 as core::ffi::c_int as isize) = *bf0
        .offset(3 as core::ffi::c_int as isize);
    *bf1.offset(4 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(56 as core::ffi::c_int as isize),
        *bf0.offset(4 as core::ffi::c_int as isize),
        -*cospi.offset(8 as core::ffi::c_int as isize),
        *bf0.offset(7 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(5 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(24 as core::ffi::c_int as isize),
        *bf0.offset(5 as core::ffi::c_int as isize),
        -*cospi.offset(40 as core::ffi::c_int as isize),
        *bf0.offset(6 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(6 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(40 as core::ffi::c_int as isize),
        *bf0.offset(5 as core::ffi::c_int as isize),
        *cospi.offset(24 as core::ffi::c_int as isize),
        *bf0.offset(6 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(7 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(8 as core::ffi::c_int as isize),
        *bf0.offset(4 as core::ffi::c_int as isize),
        *cospi.offset(56 as core::ffi::c_int as isize),
        *bf0.offset(7 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(8 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(8 as core::ffi::c_int as isize)
            + *bf0.offset(9 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(9 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(8 as core::ffi::c_int as isize)
            - *bf0.offset(9 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(10 as core::ffi::c_int as isize) = clamp_value(
        -*bf0.offset(10 as core::ffi::c_int as isize)
            + *bf0.offset(11 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(11 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(10 as core::ffi::c_int as isize)
            + *bf0.offset(11 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(12 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(12 as core::ffi::c_int as isize)
            + *bf0.offset(13 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(13 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(12 as core::ffi::c_int as isize)
            - *bf0.offset(13 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(14 as core::ffi::c_int as isize) = clamp_value(
        -*bf0.offset(14 as core::ffi::c_int as isize)
            + *bf0.offset(15 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(15 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(14 as core::ffi::c_int as isize)
            + *bf0.offset(15 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(16 as core::ffi::c_int as isize) = *bf0
        .offset(16 as core::ffi::c_int as isize);
    *bf1.offset(17 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(8 as core::ffi::c_int as isize),
        *bf0.offset(17 as core::ffi::c_int as isize),
        *cospi.offset(56 as core::ffi::c_int as isize),
        *bf0.offset(30 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(18 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(56 as core::ffi::c_int as isize),
        *bf0.offset(18 as core::ffi::c_int as isize),
        -*cospi.offset(8 as core::ffi::c_int as isize),
        *bf0.offset(29 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(19 as core::ffi::c_int as isize) = *bf0
        .offset(19 as core::ffi::c_int as isize);
    *bf1.offset(20 as core::ffi::c_int as isize) = *bf0
        .offset(20 as core::ffi::c_int as isize);
    *bf1.offset(21 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(40 as core::ffi::c_int as isize),
        *bf0.offset(21 as core::ffi::c_int as isize),
        *cospi.offset(24 as core::ffi::c_int as isize),
        *bf0.offset(26 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(22 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(24 as core::ffi::c_int as isize),
        *bf0.offset(22 as core::ffi::c_int as isize),
        -*cospi.offset(40 as core::ffi::c_int as isize),
        *bf0.offset(25 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(23 as core::ffi::c_int as isize) = *bf0
        .offset(23 as core::ffi::c_int as isize);
    *bf1.offset(24 as core::ffi::c_int as isize) = *bf0
        .offset(24 as core::ffi::c_int as isize);
    *bf1.offset(25 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(40 as core::ffi::c_int as isize),
        *bf0.offset(22 as core::ffi::c_int as isize),
        *cospi.offset(24 as core::ffi::c_int as isize),
        *bf0.offset(25 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(26 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(24 as core::ffi::c_int as isize),
        *bf0.offset(21 as core::ffi::c_int as isize),
        *cospi.offset(40 as core::ffi::c_int as isize),
        *bf0.offset(26 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(27 as core::ffi::c_int as isize) = *bf0
        .offset(27 as core::ffi::c_int as isize);
    *bf1.offset(28 as core::ffi::c_int as isize) = *bf0
        .offset(28 as core::ffi::c_int as isize);
    *bf1.offset(29 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(8 as core::ffi::c_int as isize),
        *bf0.offset(18 as core::ffi::c_int as isize),
        *cospi.offset(56 as core::ffi::c_int as isize),
        *bf0.offset(29 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(30 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(56 as core::ffi::c_int as isize),
        *bf0.offset(17 as core::ffi::c_int as isize),
        *cospi.offset(8 as core::ffi::c_int as isize),
        *bf0.offset(30 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(31 as core::ffi::c_int as isize) = *bf0
        .offset(31 as core::ffi::c_int as isize);
    av1_range_check_buf(stage, input, bf1, size, *stage_range.offset(stage as isize));
    stage += 1;
    bf0 = step.as_mut_ptr();
    bf1 = output;
    *bf1.offset(0 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(0 as core::ffi::c_int as isize),
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(1 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(1 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(0 as core::ffi::c_int as isize),
        -*cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(1 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(2 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(48 as core::ffi::c_int as isize),
        *bf0.offset(2 as core::ffi::c_int as isize),
        -*cospi.offset(16 as core::ffi::c_int as isize),
        *bf0.offset(3 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(3 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(16 as core::ffi::c_int as isize),
        *bf0.offset(2 as core::ffi::c_int as isize),
        *cospi.offset(48 as core::ffi::c_int as isize),
        *bf0.offset(3 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(4 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(4 as core::ffi::c_int as isize)
            + *bf0.offset(5 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(5 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(4 as core::ffi::c_int as isize)
            - *bf0.offset(5 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(6 as core::ffi::c_int as isize) = clamp_value(
        -*bf0.offset(6 as core::ffi::c_int as isize)
            + *bf0.offset(7 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(7 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(6 as core::ffi::c_int as isize)
            + *bf0.offset(7 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(8 as core::ffi::c_int as isize) = *bf0
        .offset(8 as core::ffi::c_int as isize);
    *bf1.offset(9 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(16 as core::ffi::c_int as isize),
        *bf0.offset(9 as core::ffi::c_int as isize),
        *cospi.offset(48 as core::ffi::c_int as isize),
        *bf0.offset(14 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(10 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(48 as core::ffi::c_int as isize),
        *bf0.offset(10 as core::ffi::c_int as isize),
        -*cospi.offset(16 as core::ffi::c_int as isize),
        *bf0.offset(13 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(11 as core::ffi::c_int as isize) = *bf0
        .offset(11 as core::ffi::c_int as isize);
    *bf1.offset(12 as core::ffi::c_int as isize) = *bf0
        .offset(12 as core::ffi::c_int as isize);
    *bf1.offset(13 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(16 as core::ffi::c_int as isize),
        *bf0.offset(10 as core::ffi::c_int as isize),
        *cospi.offset(48 as core::ffi::c_int as isize),
        *bf0.offset(13 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(14 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(48 as core::ffi::c_int as isize),
        *bf0.offset(9 as core::ffi::c_int as isize),
        *cospi.offset(16 as core::ffi::c_int as isize),
        *bf0.offset(14 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(15 as core::ffi::c_int as isize) = *bf0
        .offset(15 as core::ffi::c_int as isize);
    *bf1.offset(16 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(16 as core::ffi::c_int as isize)
            + *bf0.offset(19 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(17 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(17 as core::ffi::c_int as isize)
            + *bf0.offset(18 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(18 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(17 as core::ffi::c_int as isize)
            - *bf0.offset(18 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(19 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(16 as core::ffi::c_int as isize)
            - *bf0.offset(19 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(20 as core::ffi::c_int as isize) = clamp_value(
        -*bf0.offset(20 as core::ffi::c_int as isize)
            + *bf0.offset(23 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(21 as core::ffi::c_int as isize) = clamp_value(
        -*bf0.offset(21 as core::ffi::c_int as isize)
            + *bf0.offset(22 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(22 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(21 as core::ffi::c_int as isize)
            + *bf0.offset(22 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(23 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(20 as core::ffi::c_int as isize)
            + *bf0.offset(23 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(24 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(24 as core::ffi::c_int as isize)
            + *bf0.offset(27 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(25 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(25 as core::ffi::c_int as isize)
            + *bf0.offset(26 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(26 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(25 as core::ffi::c_int as isize)
            - *bf0.offset(26 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(27 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(24 as core::ffi::c_int as isize)
            - *bf0.offset(27 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(28 as core::ffi::c_int as isize) = clamp_value(
        -*bf0.offset(28 as core::ffi::c_int as isize)
            + *bf0.offset(31 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(29 as core::ffi::c_int as isize) = clamp_value(
        -*bf0.offset(29 as core::ffi::c_int as isize)
            + *bf0.offset(30 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(30 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(29 as core::ffi::c_int as isize)
            + *bf0.offset(30 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(31 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(28 as core::ffi::c_int as isize)
            + *bf0.offset(31 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    av1_range_check_buf(stage, input, bf1, size, *stage_range.offset(stage as isize));
    stage += 1;
    bf0 = output;
    bf1 = step.as_mut_ptr();
    *bf1.offset(0 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(0 as core::ffi::c_int as isize)
            + *bf0.offset(3 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(1 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(1 as core::ffi::c_int as isize)
            + *bf0.offset(2 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(2 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(1 as core::ffi::c_int as isize)
            - *bf0.offset(2 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(3 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(0 as core::ffi::c_int as isize)
            - *bf0.offset(3 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(4 as core::ffi::c_int as isize) = *bf0
        .offset(4 as core::ffi::c_int as isize);
    *bf1.offset(5 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(5 as core::ffi::c_int as isize),
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(6 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(6 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(5 as core::ffi::c_int as isize),
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(6 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(7 as core::ffi::c_int as isize) = *bf0
        .offset(7 as core::ffi::c_int as isize);
    *bf1.offset(8 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(8 as core::ffi::c_int as isize)
            + *bf0.offset(11 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(9 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(9 as core::ffi::c_int as isize)
            + *bf0.offset(10 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(10 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(9 as core::ffi::c_int as isize)
            - *bf0.offset(10 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(11 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(8 as core::ffi::c_int as isize)
            - *bf0.offset(11 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(12 as core::ffi::c_int as isize) = clamp_value(
        -*bf0.offset(12 as core::ffi::c_int as isize)
            + *bf0.offset(15 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(13 as core::ffi::c_int as isize) = clamp_value(
        -*bf0.offset(13 as core::ffi::c_int as isize)
            + *bf0.offset(14 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(14 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(13 as core::ffi::c_int as isize)
            + *bf0.offset(14 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(15 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(12 as core::ffi::c_int as isize)
            + *bf0.offset(15 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(16 as core::ffi::c_int as isize) = *bf0
        .offset(16 as core::ffi::c_int as isize);
    *bf1.offset(17 as core::ffi::c_int as isize) = *bf0
        .offset(17 as core::ffi::c_int as isize);
    *bf1.offset(18 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(16 as core::ffi::c_int as isize),
        *bf0.offset(18 as core::ffi::c_int as isize),
        *cospi.offset(48 as core::ffi::c_int as isize),
        *bf0.offset(29 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(19 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(16 as core::ffi::c_int as isize),
        *bf0.offset(19 as core::ffi::c_int as isize),
        *cospi.offset(48 as core::ffi::c_int as isize),
        *bf0.offset(28 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(20 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(48 as core::ffi::c_int as isize),
        *bf0.offset(20 as core::ffi::c_int as isize),
        -*cospi.offset(16 as core::ffi::c_int as isize),
        *bf0.offset(27 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(21 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(48 as core::ffi::c_int as isize),
        *bf0.offset(21 as core::ffi::c_int as isize),
        -*cospi.offset(16 as core::ffi::c_int as isize),
        *bf0.offset(26 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(22 as core::ffi::c_int as isize) = *bf0
        .offset(22 as core::ffi::c_int as isize);
    *bf1.offset(23 as core::ffi::c_int as isize) = *bf0
        .offset(23 as core::ffi::c_int as isize);
    *bf1.offset(24 as core::ffi::c_int as isize) = *bf0
        .offset(24 as core::ffi::c_int as isize);
    *bf1.offset(25 as core::ffi::c_int as isize) = *bf0
        .offset(25 as core::ffi::c_int as isize);
    *bf1.offset(26 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(16 as core::ffi::c_int as isize),
        *bf0.offset(21 as core::ffi::c_int as isize),
        *cospi.offset(48 as core::ffi::c_int as isize),
        *bf0.offset(26 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(27 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(16 as core::ffi::c_int as isize),
        *bf0.offset(20 as core::ffi::c_int as isize),
        *cospi.offset(48 as core::ffi::c_int as isize),
        *bf0.offset(27 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(28 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(48 as core::ffi::c_int as isize),
        *bf0.offset(19 as core::ffi::c_int as isize),
        *cospi.offset(16 as core::ffi::c_int as isize),
        *bf0.offset(28 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(29 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(48 as core::ffi::c_int as isize),
        *bf0.offset(18 as core::ffi::c_int as isize),
        *cospi.offset(16 as core::ffi::c_int as isize),
        *bf0.offset(29 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(30 as core::ffi::c_int as isize) = *bf0
        .offset(30 as core::ffi::c_int as isize);
    *bf1.offset(31 as core::ffi::c_int as isize) = *bf0
        .offset(31 as core::ffi::c_int as isize);
    av1_range_check_buf(stage, input, bf1, size, *stage_range.offset(stage as isize));
    stage += 1;
    bf0 = step.as_mut_ptr();
    bf1 = output;
    *bf1.offset(0 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(0 as core::ffi::c_int as isize)
            + *bf0.offset(7 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(1 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(1 as core::ffi::c_int as isize)
            + *bf0.offset(6 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(2 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(2 as core::ffi::c_int as isize)
            + *bf0.offset(5 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(3 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(3 as core::ffi::c_int as isize)
            + *bf0.offset(4 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(4 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(3 as core::ffi::c_int as isize)
            - *bf0.offset(4 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(5 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(2 as core::ffi::c_int as isize)
            - *bf0.offset(5 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(6 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(1 as core::ffi::c_int as isize)
            - *bf0.offset(6 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(7 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(0 as core::ffi::c_int as isize)
            - *bf0.offset(7 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(8 as core::ffi::c_int as isize) = *bf0
        .offset(8 as core::ffi::c_int as isize);
    *bf1.offset(9 as core::ffi::c_int as isize) = *bf0
        .offset(9 as core::ffi::c_int as isize);
    *bf1.offset(10 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(10 as core::ffi::c_int as isize),
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(13 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(11 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(11 as core::ffi::c_int as isize),
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(12 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(12 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(11 as core::ffi::c_int as isize),
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(12 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(13 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(10 as core::ffi::c_int as isize),
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(13 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(14 as core::ffi::c_int as isize) = *bf0
        .offset(14 as core::ffi::c_int as isize);
    *bf1.offset(15 as core::ffi::c_int as isize) = *bf0
        .offset(15 as core::ffi::c_int as isize);
    *bf1.offset(16 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(16 as core::ffi::c_int as isize)
            + *bf0.offset(23 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(17 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(17 as core::ffi::c_int as isize)
            + *bf0.offset(22 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(18 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(18 as core::ffi::c_int as isize)
            + *bf0.offset(21 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(19 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(19 as core::ffi::c_int as isize)
            + *bf0.offset(20 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(20 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(19 as core::ffi::c_int as isize)
            - *bf0.offset(20 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(21 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(18 as core::ffi::c_int as isize)
            - *bf0.offset(21 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(22 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(17 as core::ffi::c_int as isize)
            - *bf0.offset(22 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(23 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(16 as core::ffi::c_int as isize)
            - *bf0.offset(23 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(24 as core::ffi::c_int as isize) = clamp_value(
        -*bf0.offset(24 as core::ffi::c_int as isize)
            + *bf0.offset(31 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(25 as core::ffi::c_int as isize) = clamp_value(
        -*bf0.offset(25 as core::ffi::c_int as isize)
            + *bf0.offset(30 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(26 as core::ffi::c_int as isize) = clamp_value(
        -*bf0.offset(26 as core::ffi::c_int as isize)
            + *bf0.offset(29 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(27 as core::ffi::c_int as isize) = clamp_value(
        -*bf0.offset(27 as core::ffi::c_int as isize)
            + *bf0.offset(28 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(28 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(27 as core::ffi::c_int as isize)
            + *bf0.offset(28 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(29 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(26 as core::ffi::c_int as isize)
            + *bf0.offset(29 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(30 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(25 as core::ffi::c_int as isize)
            + *bf0.offset(30 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(31 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(24 as core::ffi::c_int as isize)
            + *bf0.offset(31 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    av1_range_check_buf(stage, input, bf1, size, *stage_range.offset(stage as isize));
    stage += 1;
    bf0 = output;
    bf1 = step.as_mut_ptr();
    *bf1.offset(0 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(0 as core::ffi::c_int as isize)
            + *bf0.offset(15 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(1 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(1 as core::ffi::c_int as isize)
            + *bf0.offset(14 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(2 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(2 as core::ffi::c_int as isize)
            + *bf0.offset(13 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(3 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(3 as core::ffi::c_int as isize)
            + *bf0.offset(12 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(4 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(4 as core::ffi::c_int as isize)
            + *bf0.offset(11 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(5 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(5 as core::ffi::c_int as isize)
            + *bf0.offset(10 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(6 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(6 as core::ffi::c_int as isize)
            + *bf0.offset(9 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(7 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(7 as core::ffi::c_int as isize)
            + *bf0.offset(8 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(8 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(7 as core::ffi::c_int as isize)
            - *bf0.offset(8 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(9 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(6 as core::ffi::c_int as isize)
            - *bf0.offset(9 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(10 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(5 as core::ffi::c_int as isize)
            - *bf0.offset(10 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(11 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(4 as core::ffi::c_int as isize)
            - *bf0.offset(11 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(12 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(3 as core::ffi::c_int as isize)
            - *bf0.offset(12 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(13 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(2 as core::ffi::c_int as isize)
            - *bf0.offset(13 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(14 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(1 as core::ffi::c_int as isize)
            - *bf0.offset(14 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(15 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(0 as core::ffi::c_int as isize)
            - *bf0.offset(15 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(16 as core::ffi::c_int as isize) = *bf0
        .offset(16 as core::ffi::c_int as isize);
    *bf1.offset(17 as core::ffi::c_int as isize) = *bf0
        .offset(17 as core::ffi::c_int as isize);
    *bf1.offset(18 as core::ffi::c_int as isize) = *bf0
        .offset(18 as core::ffi::c_int as isize);
    *bf1.offset(19 as core::ffi::c_int as isize) = *bf0
        .offset(19 as core::ffi::c_int as isize);
    *bf1.offset(20 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(20 as core::ffi::c_int as isize),
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(27 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(21 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(21 as core::ffi::c_int as isize),
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(26 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(22 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(22 as core::ffi::c_int as isize),
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(25 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(23 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(23 as core::ffi::c_int as isize),
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(24 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(24 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(23 as core::ffi::c_int as isize),
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(24 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(25 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(22 as core::ffi::c_int as isize),
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(25 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(26 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(21 as core::ffi::c_int as isize),
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(26 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(27 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(20 as core::ffi::c_int as isize),
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(27 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(28 as core::ffi::c_int as isize) = *bf0
        .offset(28 as core::ffi::c_int as isize);
    *bf1.offset(29 as core::ffi::c_int as isize) = *bf0
        .offset(29 as core::ffi::c_int as isize);
    *bf1.offset(30 as core::ffi::c_int as isize) = *bf0
        .offset(30 as core::ffi::c_int as isize);
    *bf1.offset(31 as core::ffi::c_int as isize) = *bf0
        .offset(31 as core::ffi::c_int as isize);
    av1_range_check_buf(stage, input, bf1, size, *stage_range.offset(stage as isize));
    stage += 1;
    bf0 = step.as_mut_ptr();
    bf1 = output;
    *bf1.offset(0 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(0 as core::ffi::c_int as isize)
            + *bf0.offset(31 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(1 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(1 as core::ffi::c_int as isize)
            + *bf0.offset(30 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(2 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(2 as core::ffi::c_int as isize)
            + *bf0.offset(29 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(3 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(3 as core::ffi::c_int as isize)
            + *bf0.offset(28 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(4 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(4 as core::ffi::c_int as isize)
            + *bf0.offset(27 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(5 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(5 as core::ffi::c_int as isize)
            + *bf0.offset(26 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(6 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(6 as core::ffi::c_int as isize)
            + *bf0.offset(25 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(7 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(7 as core::ffi::c_int as isize)
            + *bf0.offset(24 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(8 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(8 as core::ffi::c_int as isize)
            + *bf0.offset(23 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(9 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(9 as core::ffi::c_int as isize)
            + *bf0.offset(22 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(10 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(10 as core::ffi::c_int as isize)
            + *bf0.offset(21 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(11 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(11 as core::ffi::c_int as isize)
            + *bf0.offset(20 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(12 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(12 as core::ffi::c_int as isize)
            + *bf0.offset(19 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(13 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(13 as core::ffi::c_int as isize)
            + *bf0.offset(18 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(14 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(14 as core::ffi::c_int as isize)
            + *bf0.offset(17 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(15 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(15 as core::ffi::c_int as isize)
            + *bf0.offset(16 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(16 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(15 as core::ffi::c_int as isize)
            - *bf0.offset(16 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(17 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(14 as core::ffi::c_int as isize)
            - *bf0.offset(17 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(18 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(13 as core::ffi::c_int as isize)
            - *bf0.offset(18 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(19 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(12 as core::ffi::c_int as isize)
            - *bf0.offset(19 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(20 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(11 as core::ffi::c_int as isize)
            - *bf0.offset(20 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(21 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(10 as core::ffi::c_int as isize)
            - *bf0.offset(21 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(22 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(9 as core::ffi::c_int as isize)
            - *bf0.offset(22 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(23 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(8 as core::ffi::c_int as isize)
            - *bf0.offset(23 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(24 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(7 as core::ffi::c_int as isize)
            - *bf0.offset(24 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(25 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(6 as core::ffi::c_int as isize)
            - *bf0.offset(25 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(26 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(5 as core::ffi::c_int as isize)
            - *bf0.offset(26 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(27 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(4 as core::ffi::c_int as isize)
            - *bf0.offset(27 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(28 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(3 as core::ffi::c_int as isize)
            - *bf0.offset(28 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(29 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(2 as core::ffi::c_int as isize)
            - *bf0.offset(29 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(30 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(1 as core::ffi::c_int as isize)
            - *bf0.offset(30 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(31 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(0 as core::ffi::c_int as isize)
            - *bf0.offset(31 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_iadst4(
    mut input: *const int32_t,
    mut output: *mut int32_t,
    mut cos_bit: int8_t,
    mut stage_range: *const int8_t,
) {
    let mut bit: core::ffi::c_int = cos_bit as core::ffi::c_int;
    let mut sinpi: *const int32_t = sinpi_arr(bit);
    let mut s0: int64_t = 0;
    let mut s1: int64_t = 0;
    let mut s2: int64_t = 0;
    let mut s3: int64_t = 0;
    let mut s4: int64_t = 0;
    let mut s5: int64_t = 0;
    let mut s6: int64_t = 0;
    let mut s7: int64_t = 0;
    let mut x0: int64_t = *input.offset(0 as core::ffi::c_int as isize) as int64_t;
    let mut x1: int64_t = *input.offset(1 as core::ffi::c_int as isize) as int64_t;
    let mut x2: int64_t = *input.offset(2 as core::ffi::c_int as isize) as int64_t;
    let mut x3: int64_t = *input.offset(3 as core::ffi::c_int as isize) as int64_t;
    if x0 | x1 | x2 | x3 == 0 {
        let fresh0 = &raw mut *output.offset(3 as core::ffi::c_int as isize);
        *fresh0 = 0 as core::ffi::c_int as int32_t;
        let fresh1 = &raw mut *output.offset(2 as core::ffi::c_int as isize);
        *fresh1 = *fresh0;
        let fresh2 = &raw mut *output.offset(1 as core::ffi::c_int as isize);
        *fresh2 = *fresh1;
        *output.offset(0 as core::ffi::c_int as isize) = *fresh2;
        return;
    }
    if *sinpi.offset(1 as core::ffi::c_int as isize)
        + *sinpi.offset(2 as core::ffi::c_int as isize)
        == *sinpi.offset(4 as core::ffi::c_int as isize)
    {} else {
        __assert_fail(
            b"sinpi[1] + sinpi[2] == sinpi[4]\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/common/av1_inv_txfm1d.c\0" as *const u8
                as *const core::ffi::c_char,
            672 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 68],
                [core::ffi::c_char; 68],
            >(*b"void av1_iadst4(const int32_t *, int32_t *, int8_t, const int8_t *)\0"))
                .as_ptr(),
        );
    };
    s0 = range_check_value64(
        *sinpi.offset(1 as core::ffi::c_int as isize) as int64_t * x0,
        (*stage_range.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int + bit)
            as int8_t,
    );
    s1 = range_check_value64(
        *sinpi.offset(2 as core::ffi::c_int as isize) as int64_t * x0,
        (*stage_range.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int + bit)
            as int8_t,
    );
    s2 = range_check_value64(
        *sinpi.offset(3 as core::ffi::c_int as isize) as int64_t * x1,
        (*stage_range.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int + bit)
            as int8_t,
    );
    s3 = range_check_value64(
        *sinpi.offset(4 as core::ffi::c_int as isize) as int64_t * x2,
        (*stage_range.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int + bit)
            as int8_t,
    );
    s4 = range_check_value64(
        *sinpi.offset(1 as core::ffi::c_int as isize) as int64_t * x2,
        (*stage_range.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int + bit)
            as int8_t,
    );
    s5 = range_check_value64(
        *sinpi.offset(2 as core::ffi::c_int as isize) as int64_t * x3,
        (*stage_range.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int + bit)
            as int8_t,
    );
    s6 = range_check_value64(
        *sinpi.offset(4 as core::ffi::c_int as isize) as int64_t * x3,
        (*stage_range.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int + bit)
            as int8_t,
    );
    s7 = range_check_value64(
        x0 - x2 + x3,
        *stage_range.offset(2 as core::ffi::c_int as isize),
    );
    s0 = range_check_value64(
        s0 + s3,
        (*stage_range.offset(3 as core::ffi::c_int as isize) as core::ffi::c_int + bit)
            as int8_t,
    );
    s1 = range_check_value64(
        s1 - s4,
        (*stage_range.offset(3 as core::ffi::c_int as isize) as core::ffi::c_int + bit)
            as int8_t,
    );
    s3 = range_check_value64(
        s2,
        (*stage_range.offset(3 as core::ffi::c_int as isize) as core::ffi::c_int + bit)
            as int8_t,
    );
    s2 = range_check_value64(
        *sinpi.offset(3 as core::ffi::c_int as isize) as int64_t * s7,
        (*stage_range.offset(3 as core::ffi::c_int as isize) as core::ffi::c_int + bit)
            as int8_t,
    );
    s0 = range_check_value64(
        s0 + s5,
        (*stage_range.offset(4 as core::ffi::c_int as isize) as core::ffi::c_int + bit)
            as int8_t,
    );
    s1 = range_check_value64(
        s1 - s6,
        (*stage_range.offset(4 as core::ffi::c_int as isize) as core::ffi::c_int + bit)
            as int8_t,
    );
    x0 = range_check_value64(
        s0 + s3,
        (*stage_range.offset(5 as core::ffi::c_int as isize) as core::ffi::c_int + bit)
            as int8_t,
    );
    x1 = range_check_value64(
        s1 + s3,
        (*stage_range.offset(5 as core::ffi::c_int as isize) as core::ffi::c_int + bit)
            as int8_t,
    );
    x2 = range_check_value64(
        s2,
        (*stage_range.offset(5 as core::ffi::c_int as isize) as core::ffi::c_int + bit)
            as int8_t,
    );
    x3 = range_check_value64(
        s0 + s1,
        (*stage_range.offset(5 as core::ffi::c_int as isize) as core::ffi::c_int + bit)
            as int8_t,
    );
    x3 = range_check_value64(
        x3 - s3,
        (*stage_range.offset(6 as core::ffi::c_int as isize) as core::ffi::c_int + bit)
            as int8_t,
    );
    *output.offset(0 as core::ffi::c_int as isize) = round_shift(x0, bit);
    *output.offset(1 as core::ffi::c_int as isize) = round_shift(x1, bit);
    *output.offset(2 as core::ffi::c_int as isize) = round_shift(x2, bit);
    *output.offset(3 as core::ffi::c_int as isize) = round_shift(x3, bit);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_iadst8(
    mut input: *const int32_t,
    mut output: *mut int32_t,
    mut cos_bit: int8_t,
    mut stage_range: *const int8_t,
) {
    if output != input as *mut int32_t {} else {
        __assert_fail(
            b"output != input\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/common/av1_inv_txfm1d.c\0" as *const u8
                as *const core::ffi::c_char,
            715 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 68],
                [core::ffi::c_char; 68],
            >(*b"void av1_iadst8(const int32_t *, int32_t *, int8_t, const int8_t *)\0"))
                .as_ptr(),
        );
    };
    let size: int32_t = 8 as int32_t;
    let mut cospi: *const int32_t = cospi_arr(cos_bit as core::ffi::c_int);
    let mut stage: int32_t = 0 as int32_t;
    let mut bf0: *mut int32_t = 0 as *mut int32_t;
    let mut bf1: *mut int32_t = 0 as *mut int32_t;
    let mut step: [int32_t; 8] = [0; 8];
    stage += 1;
    bf1 = output;
    *bf1.offset(0 as core::ffi::c_int as isize) = *input
        .offset(7 as core::ffi::c_int as isize);
    *bf1.offset(1 as core::ffi::c_int as isize) = *input
        .offset(0 as core::ffi::c_int as isize);
    *bf1.offset(2 as core::ffi::c_int as isize) = *input
        .offset(5 as core::ffi::c_int as isize);
    *bf1.offset(3 as core::ffi::c_int as isize) = *input
        .offset(2 as core::ffi::c_int as isize);
    *bf1.offset(4 as core::ffi::c_int as isize) = *input
        .offset(3 as core::ffi::c_int as isize);
    *bf1.offset(5 as core::ffi::c_int as isize) = *input
        .offset(4 as core::ffi::c_int as isize);
    *bf1.offset(6 as core::ffi::c_int as isize) = *input
        .offset(1 as core::ffi::c_int as isize);
    *bf1.offset(7 as core::ffi::c_int as isize) = *input
        .offset(6 as core::ffi::c_int as isize);
    av1_range_check_buf(stage, input, bf1, size, *stage_range.offset(stage as isize));
    stage += 1;
    bf0 = output;
    bf1 = step.as_mut_ptr();
    *bf1.offset(0 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(4 as core::ffi::c_int as isize),
        *bf0.offset(0 as core::ffi::c_int as isize),
        *cospi.offset(60 as core::ffi::c_int as isize),
        *bf0.offset(1 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(1 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(60 as core::ffi::c_int as isize),
        *bf0.offset(0 as core::ffi::c_int as isize),
        -*cospi.offset(4 as core::ffi::c_int as isize),
        *bf0.offset(1 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(2 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(20 as core::ffi::c_int as isize),
        *bf0.offset(2 as core::ffi::c_int as isize),
        *cospi.offset(44 as core::ffi::c_int as isize),
        *bf0.offset(3 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(3 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(44 as core::ffi::c_int as isize),
        *bf0.offset(2 as core::ffi::c_int as isize),
        -*cospi.offset(20 as core::ffi::c_int as isize),
        *bf0.offset(3 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(4 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(36 as core::ffi::c_int as isize),
        *bf0.offset(4 as core::ffi::c_int as isize),
        *cospi.offset(28 as core::ffi::c_int as isize),
        *bf0.offset(5 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(5 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(28 as core::ffi::c_int as isize),
        *bf0.offset(4 as core::ffi::c_int as isize),
        -*cospi.offset(36 as core::ffi::c_int as isize),
        *bf0.offset(5 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(6 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(52 as core::ffi::c_int as isize),
        *bf0.offset(6 as core::ffi::c_int as isize),
        *cospi.offset(12 as core::ffi::c_int as isize),
        *bf0.offset(7 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(7 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(12 as core::ffi::c_int as isize),
        *bf0.offset(6 as core::ffi::c_int as isize),
        -*cospi.offset(52 as core::ffi::c_int as isize),
        *bf0.offset(7 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    av1_range_check_buf(stage, input, bf1, size, *stage_range.offset(stage as isize));
    stage += 1;
    bf0 = step.as_mut_ptr();
    bf1 = output;
    *bf1.offset(0 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(0 as core::ffi::c_int as isize)
            + *bf0.offset(4 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(1 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(1 as core::ffi::c_int as isize)
            + *bf0.offset(5 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(2 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(2 as core::ffi::c_int as isize)
            + *bf0.offset(6 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(3 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(3 as core::ffi::c_int as isize)
            + *bf0.offset(7 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(4 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(0 as core::ffi::c_int as isize)
            - *bf0.offset(4 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(5 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(1 as core::ffi::c_int as isize)
            - *bf0.offset(5 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(6 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(2 as core::ffi::c_int as isize)
            - *bf0.offset(6 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(7 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(3 as core::ffi::c_int as isize)
            - *bf0.offset(7 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    av1_range_check_buf(stage, input, bf1, size, *stage_range.offset(stage as isize));
    stage += 1;
    bf0 = output;
    bf1 = step.as_mut_ptr();
    *bf1.offset(0 as core::ffi::c_int as isize) = *bf0
        .offset(0 as core::ffi::c_int as isize);
    *bf1.offset(1 as core::ffi::c_int as isize) = *bf0
        .offset(1 as core::ffi::c_int as isize);
    *bf1.offset(2 as core::ffi::c_int as isize) = *bf0
        .offset(2 as core::ffi::c_int as isize);
    *bf1.offset(3 as core::ffi::c_int as isize) = *bf0
        .offset(3 as core::ffi::c_int as isize);
    *bf1.offset(4 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(16 as core::ffi::c_int as isize),
        *bf0.offset(4 as core::ffi::c_int as isize),
        *cospi.offset(48 as core::ffi::c_int as isize),
        *bf0.offset(5 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(5 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(48 as core::ffi::c_int as isize),
        *bf0.offset(4 as core::ffi::c_int as isize),
        -*cospi.offset(16 as core::ffi::c_int as isize),
        *bf0.offset(5 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(6 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(48 as core::ffi::c_int as isize),
        *bf0.offset(6 as core::ffi::c_int as isize),
        *cospi.offset(16 as core::ffi::c_int as isize),
        *bf0.offset(7 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(7 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(16 as core::ffi::c_int as isize),
        *bf0.offset(6 as core::ffi::c_int as isize),
        *cospi.offset(48 as core::ffi::c_int as isize),
        *bf0.offset(7 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    av1_range_check_buf(stage, input, bf1, size, *stage_range.offset(stage as isize));
    stage += 1;
    bf0 = step.as_mut_ptr();
    bf1 = output;
    *bf1.offset(0 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(0 as core::ffi::c_int as isize)
            + *bf0.offset(2 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(1 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(1 as core::ffi::c_int as isize)
            + *bf0.offset(3 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(2 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(0 as core::ffi::c_int as isize)
            - *bf0.offset(2 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(3 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(1 as core::ffi::c_int as isize)
            - *bf0.offset(3 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(4 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(4 as core::ffi::c_int as isize)
            + *bf0.offset(6 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(5 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(5 as core::ffi::c_int as isize)
            + *bf0.offset(7 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(6 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(4 as core::ffi::c_int as isize)
            - *bf0.offset(6 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(7 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(5 as core::ffi::c_int as isize)
            - *bf0.offset(7 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    av1_range_check_buf(stage, input, bf1, size, *stage_range.offset(stage as isize));
    stage += 1;
    bf0 = output;
    bf1 = step.as_mut_ptr();
    *bf1.offset(0 as core::ffi::c_int as isize) = *bf0
        .offset(0 as core::ffi::c_int as isize);
    *bf1.offset(1 as core::ffi::c_int as isize) = *bf0
        .offset(1 as core::ffi::c_int as isize);
    *bf1.offset(2 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(2 as core::ffi::c_int as isize),
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(3 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(3 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(2 as core::ffi::c_int as isize),
        -*cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(3 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(4 as core::ffi::c_int as isize) = *bf0
        .offset(4 as core::ffi::c_int as isize);
    *bf1.offset(5 as core::ffi::c_int as isize) = *bf0
        .offset(5 as core::ffi::c_int as isize);
    *bf1.offset(6 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(6 as core::ffi::c_int as isize),
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(7 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(7 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(6 as core::ffi::c_int as isize),
        -*cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(7 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    av1_range_check_buf(stage, input, bf1, size, *stage_range.offset(stage as isize));
    bf0 = step.as_mut_ptr();
    bf1 = output;
    *bf1.offset(0 as core::ffi::c_int as isize) = *bf0
        .offset(0 as core::ffi::c_int as isize);
    *bf1.offset(1 as core::ffi::c_int as isize) = -*bf0
        .offset(4 as core::ffi::c_int as isize);
    *bf1.offset(2 as core::ffi::c_int as isize) = *bf0
        .offset(6 as core::ffi::c_int as isize);
    *bf1.offset(3 as core::ffi::c_int as isize) = -*bf0
        .offset(2 as core::ffi::c_int as isize);
    *bf1.offset(4 as core::ffi::c_int as isize) = *bf0
        .offset(3 as core::ffi::c_int as isize);
    *bf1.offset(5 as core::ffi::c_int as isize) = -*bf0
        .offset(7 as core::ffi::c_int as isize);
    *bf1.offset(6 as core::ffi::c_int as isize) = *bf0
        .offset(5 as core::ffi::c_int as isize);
    *bf1.offset(7 as core::ffi::c_int as isize) = -*bf0
        .offset(1 as core::ffi::c_int as isize);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_iadst16(
    mut input: *const int32_t,
    mut output: *mut int32_t,
    mut cos_bit: int8_t,
    mut stage_range: *const int8_t,
) {
    if output != input as *mut int32_t {} else {
        __assert_fail(
            b"output != input\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/common/av1_inv_txfm1d.c\0" as *const u8
                as *const core::ffi::c_char,
            823 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 69],
                [core::ffi::c_char; 69],
            >(
                *b"void av1_iadst16(const int32_t *, int32_t *, int8_t, const int8_t *)\0",
            ))
                .as_ptr(),
        );
    };
    let size: int32_t = 16 as int32_t;
    let mut cospi: *const int32_t = cospi_arr(cos_bit as core::ffi::c_int);
    let mut stage: int32_t = 0 as int32_t;
    let mut bf0: *mut int32_t = 0 as *mut int32_t;
    let mut bf1: *mut int32_t = 0 as *mut int32_t;
    let mut step: [int32_t; 16] = [0; 16];
    stage += 1;
    bf1 = output;
    *bf1.offset(0 as core::ffi::c_int as isize) = *input
        .offset(15 as core::ffi::c_int as isize);
    *bf1.offset(1 as core::ffi::c_int as isize) = *input
        .offset(0 as core::ffi::c_int as isize);
    *bf1.offset(2 as core::ffi::c_int as isize) = *input
        .offset(13 as core::ffi::c_int as isize);
    *bf1.offset(3 as core::ffi::c_int as isize) = *input
        .offset(2 as core::ffi::c_int as isize);
    *bf1.offset(4 as core::ffi::c_int as isize) = *input
        .offset(11 as core::ffi::c_int as isize);
    *bf1.offset(5 as core::ffi::c_int as isize) = *input
        .offset(4 as core::ffi::c_int as isize);
    *bf1.offset(6 as core::ffi::c_int as isize) = *input
        .offset(9 as core::ffi::c_int as isize);
    *bf1.offset(7 as core::ffi::c_int as isize) = *input
        .offset(6 as core::ffi::c_int as isize);
    *bf1.offset(8 as core::ffi::c_int as isize) = *input
        .offset(7 as core::ffi::c_int as isize);
    *bf1.offset(9 as core::ffi::c_int as isize) = *input
        .offset(8 as core::ffi::c_int as isize);
    *bf1.offset(10 as core::ffi::c_int as isize) = *input
        .offset(5 as core::ffi::c_int as isize);
    *bf1.offset(11 as core::ffi::c_int as isize) = *input
        .offset(10 as core::ffi::c_int as isize);
    *bf1.offset(12 as core::ffi::c_int as isize) = *input
        .offset(3 as core::ffi::c_int as isize);
    *bf1.offset(13 as core::ffi::c_int as isize) = *input
        .offset(12 as core::ffi::c_int as isize);
    *bf1.offset(14 as core::ffi::c_int as isize) = *input
        .offset(1 as core::ffi::c_int as isize);
    *bf1.offset(15 as core::ffi::c_int as isize) = *input
        .offset(14 as core::ffi::c_int as isize);
    av1_range_check_buf(stage, input, bf1, size, *stage_range.offset(stage as isize));
    stage += 1;
    bf0 = output;
    bf1 = step.as_mut_ptr();
    *bf1.offset(0 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(2 as core::ffi::c_int as isize),
        *bf0.offset(0 as core::ffi::c_int as isize),
        *cospi.offset(62 as core::ffi::c_int as isize),
        *bf0.offset(1 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(1 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(62 as core::ffi::c_int as isize),
        *bf0.offset(0 as core::ffi::c_int as isize),
        -*cospi.offset(2 as core::ffi::c_int as isize),
        *bf0.offset(1 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(2 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(10 as core::ffi::c_int as isize),
        *bf0.offset(2 as core::ffi::c_int as isize),
        *cospi.offset(54 as core::ffi::c_int as isize),
        *bf0.offset(3 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(3 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(54 as core::ffi::c_int as isize),
        *bf0.offset(2 as core::ffi::c_int as isize),
        -*cospi.offset(10 as core::ffi::c_int as isize),
        *bf0.offset(3 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(4 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(18 as core::ffi::c_int as isize),
        *bf0.offset(4 as core::ffi::c_int as isize),
        *cospi.offset(46 as core::ffi::c_int as isize),
        *bf0.offset(5 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(5 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(46 as core::ffi::c_int as isize),
        *bf0.offset(4 as core::ffi::c_int as isize),
        -*cospi.offset(18 as core::ffi::c_int as isize),
        *bf0.offset(5 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(6 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(26 as core::ffi::c_int as isize),
        *bf0.offset(6 as core::ffi::c_int as isize),
        *cospi.offset(38 as core::ffi::c_int as isize),
        *bf0.offset(7 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(7 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(38 as core::ffi::c_int as isize),
        *bf0.offset(6 as core::ffi::c_int as isize),
        -*cospi.offset(26 as core::ffi::c_int as isize),
        *bf0.offset(7 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(8 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(34 as core::ffi::c_int as isize),
        *bf0.offset(8 as core::ffi::c_int as isize),
        *cospi.offset(30 as core::ffi::c_int as isize),
        *bf0.offset(9 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(9 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(30 as core::ffi::c_int as isize),
        *bf0.offset(8 as core::ffi::c_int as isize),
        -*cospi.offset(34 as core::ffi::c_int as isize),
        *bf0.offset(9 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(10 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(42 as core::ffi::c_int as isize),
        *bf0.offset(10 as core::ffi::c_int as isize),
        *cospi.offset(22 as core::ffi::c_int as isize),
        *bf0.offset(11 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(11 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(22 as core::ffi::c_int as isize),
        *bf0.offset(10 as core::ffi::c_int as isize),
        -*cospi.offset(42 as core::ffi::c_int as isize),
        *bf0.offset(11 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(12 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(50 as core::ffi::c_int as isize),
        *bf0.offset(12 as core::ffi::c_int as isize),
        *cospi.offset(14 as core::ffi::c_int as isize),
        *bf0.offset(13 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(13 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(14 as core::ffi::c_int as isize),
        *bf0.offset(12 as core::ffi::c_int as isize),
        -*cospi.offset(50 as core::ffi::c_int as isize),
        *bf0.offset(13 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(14 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(58 as core::ffi::c_int as isize),
        *bf0.offset(14 as core::ffi::c_int as isize),
        *cospi.offset(6 as core::ffi::c_int as isize),
        *bf0.offset(15 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(15 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(6 as core::ffi::c_int as isize),
        *bf0.offset(14 as core::ffi::c_int as isize),
        -*cospi.offset(58 as core::ffi::c_int as isize),
        *bf0.offset(15 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    av1_range_check_buf(stage, input, bf1, size, *stage_range.offset(stage as isize));
    stage += 1;
    bf0 = step.as_mut_ptr();
    bf1 = output;
    *bf1.offset(0 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(0 as core::ffi::c_int as isize)
            + *bf0.offset(8 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(1 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(1 as core::ffi::c_int as isize)
            + *bf0.offset(9 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(2 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(2 as core::ffi::c_int as isize)
            + *bf0.offset(10 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(3 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(3 as core::ffi::c_int as isize)
            + *bf0.offset(11 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(4 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(4 as core::ffi::c_int as isize)
            + *bf0.offset(12 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(5 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(5 as core::ffi::c_int as isize)
            + *bf0.offset(13 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(6 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(6 as core::ffi::c_int as isize)
            + *bf0.offset(14 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(7 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(7 as core::ffi::c_int as isize)
            + *bf0.offset(15 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(8 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(0 as core::ffi::c_int as isize)
            - *bf0.offset(8 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(9 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(1 as core::ffi::c_int as isize)
            - *bf0.offset(9 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(10 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(2 as core::ffi::c_int as isize)
            - *bf0.offset(10 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(11 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(3 as core::ffi::c_int as isize)
            - *bf0.offset(11 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(12 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(4 as core::ffi::c_int as isize)
            - *bf0.offset(12 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(13 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(5 as core::ffi::c_int as isize)
            - *bf0.offset(13 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(14 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(6 as core::ffi::c_int as isize)
            - *bf0.offset(14 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(15 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(7 as core::ffi::c_int as isize)
            - *bf0.offset(15 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    av1_range_check_buf(stage, input, bf1, size, *stage_range.offset(stage as isize));
    stage += 1;
    bf0 = output;
    bf1 = step.as_mut_ptr();
    *bf1.offset(0 as core::ffi::c_int as isize) = *bf0
        .offset(0 as core::ffi::c_int as isize);
    *bf1.offset(1 as core::ffi::c_int as isize) = *bf0
        .offset(1 as core::ffi::c_int as isize);
    *bf1.offset(2 as core::ffi::c_int as isize) = *bf0
        .offset(2 as core::ffi::c_int as isize);
    *bf1.offset(3 as core::ffi::c_int as isize) = *bf0
        .offset(3 as core::ffi::c_int as isize);
    *bf1.offset(4 as core::ffi::c_int as isize) = *bf0
        .offset(4 as core::ffi::c_int as isize);
    *bf1.offset(5 as core::ffi::c_int as isize) = *bf0
        .offset(5 as core::ffi::c_int as isize);
    *bf1.offset(6 as core::ffi::c_int as isize) = *bf0
        .offset(6 as core::ffi::c_int as isize);
    *bf1.offset(7 as core::ffi::c_int as isize) = *bf0
        .offset(7 as core::ffi::c_int as isize);
    *bf1.offset(8 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(8 as core::ffi::c_int as isize),
        *bf0.offset(8 as core::ffi::c_int as isize),
        *cospi.offset(56 as core::ffi::c_int as isize),
        *bf0.offset(9 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(9 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(56 as core::ffi::c_int as isize),
        *bf0.offset(8 as core::ffi::c_int as isize),
        -*cospi.offset(8 as core::ffi::c_int as isize),
        *bf0.offset(9 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(10 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(40 as core::ffi::c_int as isize),
        *bf0.offset(10 as core::ffi::c_int as isize),
        *cospi.offset(24 as core::ffi::c_int as isize),
        *bf0.offset(11 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(11 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(24 as core::ffi::c_int as isize),
        *bf0.offset(10 as core::ffi::c_int as isize),
        -*cospi.offset(40 as core::ffi::c_int as isize),
        *bf0.offset(11 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(12 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(56 as core::ffi::c_int as isize),
        *bf0.offset(12 as core::ffi::c_int as isize),
        *cospi.offset(8 as core::ffi::c_int as isize),
        *bf0.offset(13 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(13 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(8 as core::ffi::c_int as isize),
        *bf0.offset(12 as core::ffi::c_int as isize),
        *cospi.offset(56 as core::ffi::c_int as isize),
        *bf0.offset(13 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(14 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(24 as core::ffi::c_int as isize),
        *bf0.offset(14 as core::ffi::c_int as isize),
        *cospi.offset(40 as core::ffi::c_int as isize),
        *bf0.offset(15 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(15 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(40 as core::ffi::c_int as isize),
        *bf0.offset(14 as core::ffi::c_int as isize),
        *cospi.offset(24 as core::ffi::c_int as isize),
        *bf0.offset(15 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    av1_range_check_buf(stage, input, bf1, size, *stage_range.offset(stage as isize));
    stage += 1;
    bf0 = step.as_mut_ptr();
    bf1 = output;
    *bf1.offset(0 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(0 as core::ffi::c_int as isize)
            + *bf0.offset(4 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(1 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(1 as core::ffi::c_int as isize)
            + *bf0.offset(5 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(2 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(2 as core::ffi::c_int as isize)
            + *bf0.offset(6 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(3 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(3 as core::ffi::c_int as isize)
            + *bf0.offset(7 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(4 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(0 as core::ffi::c_int as isize)
            - *bf0.offset(4 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(5 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(1 as core::ffi::c_int as isize)
            - *bf0.offset(5 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(6 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(2 as core::ffi::c_int as isize)
            - *bf0.offset(6 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(7 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(3 as core::ffi::c_int as isize)
            - *bf0.offset(7 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(8 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(8 as core::ffi::c_int as isize)
            + *bf0.offset(12 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(9 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(9 as core::ffi::c_int as isize)
            + *bf0.offset(13 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(10 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(10 as core::ffi::c_int as isize)
            + *bf0.offset(14 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(11 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(11 as core::ffi::c_int as isize)
            + *bf0.offset(15 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(12 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(8 as core::ffi::c_int as isize)
            - *bf0.offset(12 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(13 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(9 as core::ffi::c_int as isize)
            - *bf0.offset(13 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(14 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(10 as core::ffi::c_int as isize)
            - *bf0.offset(14 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(15 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(11 as core::ffi::c_int as isize)
            - *bf0.offset(15 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    av1_range_check_buf(stage, input, bf1, size, *stage_range.offset(stage as isize));
    stage += 1;
    bf0 = output;
    bf1 = step.as_mut_ptr();
    *bf1.offset(0 as core::ffi::c_int as isize) = *bf0
        .offset(0 as core::ffi::c_int as isize);
    *bf1.offset(1 as core::ffi::c_int as isize) = *bf0
        .offset(1 as core::ffi::c_int as isize);
    *bf1.offset(2 as core::ffi::c_int as isize) = *bf0
        .offset(2 as core::ffi::c_int as isize);
    *bf1.offset(3 as core::ffi::c_int as isize) = *bf0
        .offset(3 as core::ffi::c_int as isize);
    *bf1.offset(4 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(16 as core::ffi::c_int as isize),
        *bf0.offset(4 as core::ffi::c_int as isize),
        *cospi.offset(48 as core::ffi::c_int as isize),
        *bf0.offset(5 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(5 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(48 as core::ffi::c_int as isize),
        *bf0.offset(4 as core::ffi::c_int as isize),
        -*cospi.offset(16 as core::ffi::c_int as isize),
        *bf0.offset(5 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(6 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(48 as core::ffi::c_int as isize),
        *bf0.offset(6 as core::ffi::c_int as isize),
        *cospi.offset(16 as core::ffi::c_int as isize),
        *bf0.offset(7 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(7 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(16 as core::ffi::c_int as isize),
        *bf0.offset(6 as core::ffi::c_int as isize),
        *cospi.offset(48 as core::ffi::c_int as isize),
        *bf0.offset(7 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(8 as core::ffi::c_int as isize) = *bf0
        .offset(8 as core::ffi::c_int as isize);
    *bf1.offset(9 as core::ffi::c_int as isize) = *bf0
        .offset(9 as core::ffi::c_int as isize);
    *bf1.offset(10 as core::ffi::c_int as isize) = *bf0
        .offset(10 as core::ffi::c_int as isize);
    *bf1.offset(11 as core::ffi::c_int as isize) = *bf0
        .offset(11 as core::ffi::c_int as isize);
    *bf1.offset(12 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(16 as core::ffi::c_int as isize),
        *bf0.offset(12 as core::ffi::c_int as isize),
        *cospi.offset(48 as core::ffi::c_int as isize),
        *bf0.offset(13 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(13 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(48 as core::ffi::c_int as isize),
        *bf0.offset(12 as core::ffi::c_int as isize),
        -*cospi.offset(16 as core::ffi::c_int as isize),
        *bf0.offset(13 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(14 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(48 as core::ffi::c_int as isize),
        *bf0.offset(14 as core::ffi::c_int as isize),
        *cospi.offset(16 as core::ffi::c_int as isize),
        *bf0.offset(15 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(15 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(16 as core::ffi::c_int as isize),
        *bf0.offset(14 as core::ffi::c_int as isize),
        *cospi.offset(48 as core::ffi::c_int as isize),
        *bf0.offset(15 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    av1_range_check_buf(stage, input, bf1, size, *stage_range.offset(stage as isize));
    stage += 1;
    bf0 = step.as_mut_ptr();
    bf1 = output;
    *bf1.offset(0 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(0 as core::ffi::c_int as isize)
            + *bf0.offset(2 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(1 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(1 as core::ffi::c_int as isize)
            + *bf0.offset(3 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(2 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(0 as core::ffi::c_int as isize)
            - *bf0.offset(2 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(3 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(1 as core::ffi::c_int as isize)
            - *bf0.offset(3 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(4 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(4 as core::ffi::c_int as isize)
            + *bf0.offset(6 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(5 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(5 as core::ffi::c_int as isize)
            + *bf0.offset(7 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(6 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(4 as core::ffi::c_int as isize)
            - *bf0.offset(6 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(7 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(5 as core::ffi::c_int as isize)
            - *bf0.offset(7 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(8 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(8 as core::ffi::c_int as isize)
            + *bf0.offset(10 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(9 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(9 as core::ffi::c_int as isize)
            + *bf0.offset(11 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(10 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(8 as core::ffi::c_int as isize)
            - *bf0.offset(10 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(11 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(9 as core::ffi::c_int as isize)
            - *bf0.offset(11 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(12 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(12 as core::ffi::c_int as isize)
            + *bf0.offset(14 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(13 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(13 as core::ffi::c_int as isize)
            + *bf0.offset(15 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(14 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(12 as core::ffi::c_int as isize)
            - *bf0.offset(14 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(15 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(13 as core::ffi::c_int as isize)
            - *bf0.offset(15 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    av1_range_check_buf(stage, input, bf1, size, *stage_range.offset(stage as isize));
    stage += 1;
    bf0 = output;
    bf1 = step.as_mut_ptr();
    *bf1.offset(0 as core::ffi::c_int as isize) = *bf0
        .offset(0 as core::ffi::c_int as isize);
    *bf1.offset(1 as core::ffi::c_int as isize) = *bf0
        .offset(1 as core::ffi::c_int as isize);
    *bf1.offset(2 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(2 as core::ffi::c_int as isize),
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(3 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(3 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(2 as core::ffi::c_int as isize),
        -*cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(3 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(4 as core::ffi::c_int as isize) = *bf0
        .offset(4 as core::ffi::c_int as isize);
    *bf1.offset(5 as core::ffi::c_int as isize) = *bf0
        .offset(5 as core::ffi::c_int as isize);
    *bf1.offset(6 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(6 as core::ffi::c_int as isize),
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(7 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(7 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(6 as core::ffi::c_int as isize),
        -*cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(7 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(8 as core::ffi::c_int as isize) = *bf0
        .offset(8 as core::ffi::c_int as isize);
    *bf1.offset(9 as core::ffi::c_int as isize) = *bf0
        .offset(9 as core::ffi::c_int as isize);
    *bf1.offset(10 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(10 as core::ffi::c_int as isize),
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(11 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(11 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(10 as core::ffi::c_int as isize),
        -*cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(11 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(12 as core::ffi::c_int as isize) = *bf0
        .offset(12 as core::ffi::c_int as isize);
    *bf1.offset(13 as core::ffi::c_int as isize) = *bf0
        .offset(13 as core::ffi::c_int as isize);
    *bf1.offset(14 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(14 as core::ffi::c_int as isize),
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(15 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(15 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(14 as core::ffi::c_int as isize),
        -*cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(15 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    av1_range_check_buf(stage, input, bf1, size, *stage_range.offset(stage as isize));
    bf0 = step.as_mut_ptr();
    bf1 = output;
    *bf1.offset(0 as core::ffi::c_int as isize) = *bf0
        .offset(0 as core::ffi::c_int as isize);
    *bf1.offset(1 as core::ffi::c_int as isize) = -*bf0
        .offset(8 as core::ffi::c_int as isize);
    *bf1.offset(2 as core::ffi::c_int as isize) = *bf0
        .offset(12 as core::ffi::c_int as isize);
    *bf1.offset(3 as core::ffi::c_int as isize) = -*bf0
        .offset(4 as core::ffi::c_int as isize);
    *bf1.offset(4 as core::ffi::c_int as isize) = *bf0
        .offset(6 as core::ffi::c_int as isize);
    *bf1.offset(5 as core::ffi::c_int as isize) = -*bf0
        .offset(14 as core::ffi::c_int as isize);
    *bf1.offset(6 as core::ffi::c_int as isize) = *bf0
        .offset(10 as core::ffi::c_int as isize);
    *bf1.offset(7 as core::ffi::c_int as isize) = -*bf0
        .offset(2 as core::ffi::c_int as isize);
    *bf1.offset(8 as core::ffi::c_int as isize) = *bf0
        .offset(3 as core::ffi::c_int as isize);
    *bf1.offset(9 as core::ffi::c_int as isize) = -*bf0
        .offset(11 as core::ffi::c_int as isize);
    *bf1.offset(10 as core::ffi::c_int as isize) = *bf0
        .offset(15 as core::ffi::c_int as isize);
    *bf1.offset(11 as core::ffi::c_int as isize) = -*bf0
        .offset(7 as core::ffi::c_int as isize);
    *bf1.offset(12 as core::ffi::c_int as isize) = *bf0
        .offset(5 as core::ffi::c_int as isize);
    *bf1.offset(13 as core::ffi::c_int as isize) = -*bf0
        .offset(13 as core::ffi::c_int as isize);
    *bf1.offset(14 as core::ffi::c_int as isize) = *bf0
        .offset(9 as core::ffi::c_int as isize);
    *bf1.offset(15 as core::ffi::c_int as isize) = -*bf0
        .offset(1 as core::ffi::c_int as isize);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_iidentity4_c(
    mut input: *const int32_t,
    mut output: *mut int32_t,
    mut cos_bit: int8_t,
    mut stage_range: *const int8_t,
) {
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *output.offset(i as isize) = round_shift(
            NewSqrt2 as int64_t * *input.offset(i as isize) as int64_t,
            NewSqrt2Bits as core::ffi::c_int,
        );
        i += 1;
    }
    if *stage_range.offset(0 as core::ffi::c_int as isize) as int32_t
        + 12 as core::ffi::c_int as int32_t <= 32 as int32_t
    {} else {
        __assert_fail(
            b"stage_range[0] + NewSqrt2Bits <= 32\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/common/av1_inv_txfm1d.c\0" as *const u8
                as *const core::ffi::c_char,
            1036 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 74],
                [core::ffi::c_char; 74],
            >(
                *b"void av1_iidentity4_c(const int32_t *, int32_t *, int8_t, const int8_t *)\0",
            ))
                .as_ptr(),
        );
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_iidentity8_c(
    mut input: *const int32_t,
    mut output: *mut int32_t,
    mut cos_bit: int8_t,
    mut stage_range: *const int8_t,
) {
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < 8 as core::ffi::c_int {
        *output.offset(i as isize) = (*input.offset(i as isize) as int64_t
            * 2 as int64_t) as int32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_iidentity16_c(
    mut input: *const int32_t,
    mut output: *mut int32_t,
    mut cos_bit: int8_t,
    mut stage_range: *const int8_t,
) {
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < 16 as core::ffi::c_int {
        *output.offset(i as isize) = round_shift(
            NewSqrt2 as int64_t * 2 as int64_t * *input.offset(i as isize) as int64_t,
            NewSqrt2Bits as core::ffi::c_int,
        );
        i += 1;
    }
    if *stage_range.offset(0 as core::ffi::c_int as isize) as int32_t
        + 12 as core::ffi::c_int as int32_t <= 32 as int32_t
    {} else {
        __assert_fail(
            b"stage_range[0] + NewSqrt2Bits <= 32\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/common/av1_inv_txfm1d.c\0" as *const u8
                as *const core::ffi::c_char,
            1052 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 75],
                [core::ffi::c_char; 75],
            >(
                *b"void av1_iidentity16_c(const int32_t *, int32_t *, int8_t, const int8_t *)\0",
            ))
                .as_ptr(),
        );
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_iidentity32_c(
    mut input: *const int32_t,
    mut output: *mut int32_t,
    mut cos_bit: int8_t,
    mut stage_range: *const int8_t,
) {
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < 32 as core::ffi::c_int {
        *output.offset(i as isize) = (*input.offset(i as isize) as int64_t
            * 4 as int64_t) as int32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_idct64(
    mut input: *const int32_t,
    mut output: *mut int32_t,
    mut cos_bit: int8_t,
    mut stage_range: *const int8_t,
) {
    if output != input as *mut int32_t {} else {
        __assert_fail(
            b"output != input\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/common/av1_inv_txfm1d.c\0" as *const u8
                as *const core::ffi::c_char,
            1064 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 68],
                [core::ffi::c_char; 68],
            >(*b"void av1_idct64(const int32_t *, int32_t *, int8_t, const int8_t *)\0"))
                .as_ptr(),
        );
    };
    let size: int32_t = 64 as int32_t;
    let mut cospi: *const int32_t = cospi_arr(cos_bit as core::ffi::c_int);
    let mut stage: int32_t = 0 as int32_t;
    let mut bf0: *mut int32_t = 0 as *mut int32_t;
    let mut bf1: *mut int32_t = 0 as *mut int32_t;
    let mut step: [int32_t; 64] = [0; 64];
    stage += 1;
    bf1 = output;
    *bf1.offset(0 as core::ffi::c_int as isize) = *input
        .offset(0 as core::ffi::c_int as isize);
    *bf1.offset(1 as core::ffi::c_int as isize) = *input
        .offset(32 as core::ffi::c_int as isize);
    *bf1.offset(2 as core::ffi::c_int as isize) = *input
        .offset(16 as core::ffi::c_int as isize);
    *bf1.offset(3 as core::ffi::c_int as isize) = *input
        .offset(48 as core::ffi::c_int as isize);
    *bf1.offset(4 as core::ffi::c_int as isize) = *input
        .offset(8 as core::ffi::c_int as isize);
    *bf1.offset(5 as core::ffi::c_int as isize) = *input
        .offset(40 as core::ffi::c_int as isize);
    *bf1.offset(6 as core::ffi::c_int as isize) = *input
        .offset(24 as core::ffi::c_int as isize);
    *bf1.offset(7 as core::ffi::c_int as isize) = *input
        .offset(56 as core::ffi::c_int as isize);
    *bf1.offset(8 as core::ffi::c_int as isize) = *input
        .offset(4 as core::ffi::c_int as isize);
    *bf1.offset(9 as core::ffi::c_int as isize) = *input
        .offset(36 as core::ffi::c_int as isize);
    *bf1.offset(10 as core::ffi::c_int as isize) = *input
        .offset(20 as core::ffi::c_int as isize);
    *bf1.offset(11 as core::ffi::c_int as isize) = *input
        .offset(52 as core::ffi::c_int as isize);
    *bf1.offset(12 as core::ffi::c_int as isize) = *input
        .offset(12 as core::ffi::c_int as isize);
    *bf1.offset(13 as core::ffi::c_int as isize) = *input
        .offset(44 as core::ffi::c_int as isize);
    *bf1.offset(14 as core::ffi::c_int as isize) = *input
        .offset(28 as core::ffi::c_int as isize);
    *bf1.offset(15 as core::ffi::c_int as isize) = *input
        .offset(60 as core::ffi::c_int as isize);
    *bf1.offset(16 as core::ffi::c_int as isize) = *input
        .offset(2 as core::ffi::c_int as isize);
    *bf1.offset(17 as core::ffi::c_int as isize) = *input
        .offset(34 as core::ffi::c_int as isize);
    *bf1.offset(18 as core::ffi::c_int as isize) = *input
        .offset(18 as core::ffi::c_int as isize);
    *bf1.offset(19 as core::ffi::c_int as isize) = *input
        .offset(50 as core::ffi::c_int as isize);
    *bf1.offset(20 as core::ffi::c_int as isize) = *input
        .offset(10 as core::ffi::c_int as isize);
    *bf1.offset(21 as core::ffi::c_int as isize) = *input
        .offset(42 as core::ffi::c_int as isize);
    *bf1.offset(22 as core::ffi::c_int as isize) = *input
        .offset(26 as core::ffi::c_int as isize);
    *bf1.offset(23 as core::ffi::c_int as isize) = *input
        .offset(58 as core::ffi::c_int as isize);
    *bf1.offset(24 as core::ffi::c_int as isize) = *input
        .offset(6 as core::ffi::c_int as isize);
    *bf1.offset(25 as core::ffi::c_int as isize) = *input
        .offset(38 as core::ffi::c_int as isize);
    *bf1.offset(26 as core::ffi::c_int as isize) = *input
        .offset(22 as core::ffi::c_int as isize);
    *bf1.offset(27 as core::ffi::c_int as isize) = *input
        .offset(54 as core::ffi::c_int as isize);
    *bf1.offset(28 as core::ffi::c_int as isize) = *input
        .offset(14 as core::ffi::c_int as isize);
    *bf1.offset(29 as core::ffi::c_int as isize) = *input
        .offset(46 as core::ffi::c_int as isize);
    *bf1.offset(30 as core::ffi::c_int as isize) = *input
        .offset(30 as core::ffi::c_int as isize);
    *bf1.offset(31 as core::ffi::c_int as isize) = *input
        .offset(62 as core::ffi::c_int as isize);
    *bf1.offset(32 as core::ffi::c_int as isize) = *input
        .offset(1 as core::ffi::c_int as isize);
    *bf1.offset(33 as core::ffi::c_int as isize) = *input
        .offset(33 as core::ffi::c_int as isize);
    *bf1.offset(34 as core::ffi::c_int as isize) = *input
        .offset(17 as core::ffi::c_int as isize);
    *bf1.offset(35 as core::ffi::c_int as isize) = *input
        .offset(49 as core::ffi::c_int as isize);
    *bf1.offset(36 as core::ffi::c_int as isize) = *input
        .offset(9 as core::ffi::c_int as isize);
    *bf1.offset(37 as core::ffi::c_int as isize) = *input
        .offset(41 as core::ffi::c_int as isize);
    *bf1.offset(38 as core::ffi::c_int as isize) = *input
        .offset(25 as core::ffi::c_int as isize);
    *bf1.offset(39 as core::ffi::c_int as isize) = *input
        .offset(57 as core::ffi::c_int as isize);
    *bf1.offset(40 as core::ffi::c_int as isize) = *input
        .offset(5 as core::ffi::c_int as isize);
    *bf1.offset(41 as core::ffi::c_int as isize) = *input
        .offset(37 as core::ffi::c_int as isize);
    *bf1.offset(42 as core::ffi::c_int as isize) = *input
        .offset(21 as core::ffi::c_int as isize);
    *bf1.offset(43 as core::ffi::c_int as isize) = *input
        .offset(53 as core::ffi::c_int as isize);
    *bf1.offset(44 as core::ffi::c_int as isize) = *input
        .offset(13 as core::ffi::c_int as isize);
    *bf1.offset(45 as core::ffi::c_int as isize) = *input
        .offset(45 as core::ffi::c_int as isize);
    *bf1.offset(46 as core::ffi::c_int as isize) = *input
        .offset(29 as core::ffi::c_int as isize);
    *bf1.offset(47 as core::ffi::c_int as isize) = *input
        .offset(61 as core::ffi::c_int as isize);
    *bf1.offset(48 as core::ffi::c_int as isize) = *input
        .offset(3 as core::ffi::c_int as isize);
    *bf1.offset(49 as core::ffi::c_int as isize) = *input
        .offset(35 as core::ffi::c_int as isize);
    *bf1.offset(50 as core::ffi::c_int as isize) = *input
        .offset(19 as core::ffi::c_int as isize);
    *bf1.offset(51 as core::ffi::c_int as isize) = *input
        .offset(51 as core::ffi::c_int as isize);
    *bf1.offset(52 as core::ffi::c_int as isize) = *input
        .offset(11 as core::ffi::c_int as isize);
    *bf1.offset(53 as core::ffi::c_int as isize) = *input
        .offset(43 as core::ffi::c_int as isize);
    *bf1.offset(54 as core::ffi::c_int as isize) = *input
        .offset(27 as core::ffi::c_int as isize);
    *bf1.offset(55 as core::ffi::c_int as isize) = *input
        .offset(59 as core::ffi::c_int as isize);
    *bf1.offset(56 as core::ffi::c_int as isize) = *input
        .offset(7 as core::ffi::c_int as isize);
    *bf1.offset(57 as core::ffi::c_int as isize) = *input
        .offset(39 as core::ffi::c_int as isize);
    *bf1.offset(58 as core::ffi::c_int as isize) = *input
        .offset(23 as core::ffi::c_int as isize);
    *bf1.offset(59 as core::ffi::c_int as isize) = *input
        .offset(55 as core::ffi::c_int as isize);
    *bf1.offset(60 as core::ffi::c_int as isize) = *input
        .offset(15 as core::ffi::c_int as isize);
    *bf1.offset(61 as core::ffi::c_int as isize) = *input
        .offset(47 as core::ffi::c_int as isize);
    *bf1.offset(62 as core::ffi::c_int as isize) = *input
        .offset(31 as core::ffi::c_int as isize);
    *bf1.offset(63 as core::ffi::c_int as isize) = *input
        .offset(63 as core::ffi::c_int as isize);
    av1_range_check_buf(stage, input, bf1, size, *stage_range.offset(stage as isize));
    stage += 1;
    bf0 = output;
    bf1 = step.as_mut_ptr();
    *bf1.offset(0 as core::ffi::c_int as isize) = *bf0
        .offset(0 as core::ffi::c_int as isize);
    *bf1.offset(1 as core::ffi::c_int as isize) = *bf0
        .offset(1 as core::ffi::c_int as isize);
    *bf1.offset(2 as core::ffi::c_int as isize) = *bf0
        .offset(2 as core::ffi::c_int as isize);
    *bf1.offset(3 as core::ffi::c_int as isize) = *bf0
        .offset(3 as core::ffi::c_int as isize);
    *bf1.offset(4 as core::ffi::c_int as isize) = *bf0
        .offset(4 as core::ffi::c_int as isize);
    *bf1.offset(5 as core::ffi::c_int as isize) = *bf0
        .offset(5 as core::ffi::c_int as isize);
    *bf1.offset(6 as core::ffi::c_int as isize) = *bf0
        .offset(6 as core::ffi::c_int as isize);
    *bf1.offset(7 as core::ffi::c_int as isize) = *bf0
        .offset(7 as core::ffi::c_int as isize);
    *bf1.offset(8 as core::ffi::c_int as isize) = *bf0
        .offset(8 as core::ffi::c_int as isize);
    *bf1.offset(9 as core::ffi::c_int as isize) = *bf0
        .offset(9 as core::ffi::c_int as isize);
    *bf1.offset(10 as core::ffi::c_int as isize) = *bf0
        .offset(10 as core::ffi::c_int as isize);
    *bf1.offset(11 as core::ffi::c_int as isize) = *bf0
        .offset(11 as core::ffi::c_int as isize);
    *bf1.offset(12 as core::ffi::c_int as isize) = *bf0
        .offset(12 as core::ffi::c_int as isize);
    *bf1.offset(13 as core::ffi::c_int as isize) = *bf0
        .offset(13 as core::ffi::c_int as isize);
    *bf1.offset(14 as core::ffi::c_int as isize) = *bf0
        .offset(14 as core::ffi::c_int as isize);
    *bf1.offset(15 as core::ffi::c_int as isize) = *bf0
        .offset(15 as core::ffi::c_int as isize);
    *bf1.offset(16 as core::ffi::c_int as isize) = *bf0
        .offset(16 as core::ffi::c_int as isize);
    *bf1.offset(17 as core::ffi::c_int as isize) = *bf0
        .offset(17 as core::ffi::c_int as isize);
    *bf1.offset(18 as core::ffi::c_int as isize) = *bf0
        .offset(18 as core::ffi::c_int as isize);
    *bf1.offset(19 as core::ffi::c_int as isize) = *bf0
        .offset(19 as core::ffi::c_int as isize);
    *bf1.offset(20 as core::ffi::c_int as isize) = *bf0
        .offset(20 as core::ffi::c_int as isize);
    *bf1.offset(21 as core::ffi::c_int as isize) = *bf0
        .offset(21 as core::ffi::c_int as isize);
    *bf1.offset(22 as core::ffi::c_int as isize) = *bf0
        .offset(22 as core::ffi::c_int as isize);
    *bf1.offset(23 as core::ffi::c_int as isize) = *bf0
        .offset(23 as core::ffi::c_int as isize);
    *bf1.offset(24 as core::ffi::c_int as isize) = *bf0
        .offset(24 as core::ffi::c_int as isize);
    *bf1.offset(25 as core::ffi::c_int as isize) = *bf0
        .offset(25 as core::ffi::c_int as isize);
    *bf1.offset(26 as core::ffi::c_int as isize) = *bf0
        .offset(26 as core::ffi::c_int as isize);
    *bf1.offset(27 as core::ffi::c_int as isize) = *bf0
        .offset(27 as core::ffi::c_int as isize);
    *bf1.offset(28 as core::ffi::c_int as isize) = *bf0
        .offset(28 as core::ffi::c_int as isize);
    *bf1.offset(29 as core::ffi::c_int as isize) = *bf0
        .offset(29 as core::ffi::c_int as isize);
    *bf1.offset(30 as core::ffi::c_int as isize) = *bf0
        .offset(30 as core::ffi::c_int as isize);
    *bf1.offset(31 as core::ffi::c_int as isize) = *bf0
        .offset(31 as core::ffi::c_int as isize);
    *bf1.offset(32 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(63 as core::ffi::c_int as isize),
        *bf0.offset(32 as core::ffi::c_int as isize),
        -*cospi.offset(1 as core::ffi::c_int as isize),
        *bf0.offset(63 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(33 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(31 as core::ffi::c_int as isize),
        *bf0.offset(33 as core::ffi::c_int as isize),
        -*cospi.offset(33 as core::ffi::c_int as isize),
        *bf0.offset(62 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(34 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(47 as core::ffi::c_int as isize),
        *bf0.offset(34 as core::ffi::c_int as isize),
        -*cospi.offset(17 as core::ffi::c_int as isize),
        *bf0.offset(61 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(35 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(15 as core::ffi::c_int as isize),
        *bf0.offset(35 as core::ffi::c_int as isize),
        -*cospi.offset(49 as core::ffi::c_int as isize),
        *bf0.offset(60 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(36 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(55 as core::ffi::c_int as isize),
        *bf0.offset(36 as core::ffi::c_int as isize),
        -*cospi.offset(9 as core::ffi::c_int as isize),
        *bf0.offset(59 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(37 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(23 as core::ffi::c_int as isize),
        *bf0.offset(37 as core::ffi::c_int as isize),
        -*cospi.offset(41 as core::ffi::c_int as isize),
        *bf0.offset(58 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(38 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(39 as core::ffi::c_int as isize),
        *bf0.offset(38 as core::ffi::c_int as isize),
        -*cospi.offset(25 as core::ffi::c_int as isize),
        *bf0.offset(57 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(39 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(7 as core::ffi::c_int as isize),
        *bf0.offset(39 as core::ffi::c_int as isize),
        -*cospi.offset(57 as core::ffi::c_int as isize),
        *bf0.offset(56 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(40 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(59 as core::ffi::c_int as isize),
        *bf0.offset(40 as core::ffi::c_int as isize),
        -*cospi.offset(5 as core::ffi::c_int as isize),
        *bf0.offset(55 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(41 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(27 as core::ffi::c_int as isize),
        *bf0.offset(41 as core::ffi::c_int as isize),
        -*cospi.offset(37 as core::ffi::c_int as isize),
        *bf0.offset(54 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(42 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(43 as core::ffi::c_int as isize),
        *bf0.offset(42 as core::ffi::c_int as isize),
        -*cospi.offset(21 as core::ffi::c_int as isize),
        *bf0.offset(53 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(43 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(11 as core::ffi::c_int as isize),
        *bf0.offset(43 as core::ffi::c_int as isize),
        -*cospi.offset(53 as core::ffi::c_int as isize),
        *bf0.offset(52 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(44 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(51 as core::ffi::c_int as isize),
        *bf0.offset(44 as core::ffi::c_int as isize),
        -*cospi.offset(13 as core::ffi::c_int as isize),
        *bf0.offset(51 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(45 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(19 as core::ffi::c_int as isize),
        *bf0.offset(45 as core::ffi::c_int as isize),
        -*cospi.offset(45 as core::ffi::c_int as isize),
        *bf0.offset(50 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(46 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(35 as core::ffi::c_int as isize),
        *bf0.offset(46 as core::ffi::c_int as isize),
        -*cospi.offset(29 as core::ffi::c_int as isize),
        *bf0.offset(49 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(47 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(3 as core::ffi::c_int as isize),
        *bf0.offset(47 as core::ffi::c_int as isize),
        -*cospi.offset(61 as core::ffi::c_int as isize),
        *bf0.offset(48 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(48 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(61 as core::ffi::c_int as isize),
        *bf0.offset(47 as core::ffi::c_int as isize),
        *cospi.offset(3 as core::ffi::c_int as isize),
        *bf0.offset(48 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(49 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(29 as core::ffi::c_int as isize),
        *bf0.offset(46 as core::ffi::c_int as isize),
        *cospi.offset(35 as core::ffi::c_int as isize),
        *bf0.offset(49 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(50 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(45 as core::ffi::c_int as isize),
        *bf0.offset(45 as core::ffi::c_int as isize),
        *cospi.offset(19 as core::ffi::c_int as isize),
        *bf0.offset(50 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(51 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(13 as core::ffi::c_int as isize),
        *bf0.offset(44 as core::ffi::c_int as isize),
        *cospi.offset(51 as core::ffi::c_int as isize),
        *bf0.offset(51 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(52 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(53 as core::ffi::c_int as isize),
        *bf0.offset(43 as core::ffi::c_int as isize),
        *cospi.offset(11 as core::ffi::c_int as isize),
        *bf0.offset(52 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(53 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(21 as core::ffi::c_int as isize),
        *bf0.offset(42 as core::ffi::c_int as isize),
        *cospi.offset(43 as core::ffi::c_int as isize),
        *bf0.offset(53 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(54 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(37 as core::ffi::c_int as isize),
        *bf0.offset(41 as core::ffi::c_int as isize),
        *cospi.offset(27 as core::ffi::c_int as isize),
        *bf0.offset(54 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(55 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(5 as core::ffi::c_int as isize),
        *bf0.offset(40 as core::ffi::c_int as isize),
        *cospi.offset(59 as core::ffi::c_int as isize),
        *bf0.offset(55 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(56 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(57 as core::ffi::c_int as isize),
        *bf0.offset(39 as core::ffi::c_int as isize),
        *cospi.offset(7 as core::ffi::c_int as isize),
        *bf0.offset(56 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(57 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(25 as core::ffi::c_int as isize),
        *bf0.offset(38 as core::ffi::c_int as isize),
        *cospi.offset(39 as core::ffi::c_int as isize),
        *bf0.offset(57 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(58 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(41 as core::ffi::c_int as isize),
        *bf0.offset(37 as core::ffi::c_int as isize),
        *cospi.offset(23 as core::ffi::c_int as isize),
        *bf0.offset(58 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(59 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(9 as core::ffi::c_int as isize),
        *bf0.offset(36 as core::ffi::c_int as isize),
        *cospi.offset(55 as core::ffi::c_int as isize),
        *bf0.offset(59 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(60 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(49 as core::ffi::c_int as isize),
        *bf0.offset(35 as core::ffi::c_int as isize),
        *cospi.offset(15 as core::ffi::c_int as isize),
        *bf0.offset(60 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(61 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(17 as core::ffi::c_int as isize),
        *bf0.offset(34 as core::ffi::c_int as isize),
        *cospi.offset(47 as core::ffi::c_int as isize),
        *bf0.offset(61 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(62 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(33 as core::ffi::c_int as isize),
        *bf0.offset(33 as core::ffi::c_int as isize),
        *cospi.offset(31 as core::ffi::c_int as isize),
        *bf0.offset(62 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(63 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(1 as core::ffi::c_int as isize),
        *bf0.offset(32 as core::ffi::c_int as isize),
        *cospi.offset(63 as core::ffi::c_int as isize),
        *bf0.offset(63 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    av1_range_check_buf(stage, input, bf1, size, *stage_range.offset(stage as isize));
    stage += 1;
    bf0 = step.as_mut_ptr();
    bf1 = output;
    *bf1.offset(0 as core::ffi::c_int as isize) = *bf0
        .offset(0 as core::ffi::c_int as isize);
    *bf1.offset(1 as core::ffi::c_int as isize) = *bf0
        .offset(1 as core::ffi::c_int as isize);
    *bf1.offset(2 as core::ffi::c_int as isize) = *bf0
        .offset(2 as core::ffi::c_int as isize);
    *bf1.offset(3 as core::ffi::c_int as isize) = *bf0
        .offset(3 as core::ffi::c_int as isize);
    *bf1.offset(4 as core::ffi::c_int as isize) = *bf0
        .offset(4 as core::ffi::c_int as isize);
    *bf1.offset(5 as core::ffi::c_int as isize) = *bf0
        .offset(5 as core::ffi::c_int as isize);
    *bf1.offset(6 as core::ffi::c_int as isize) = *bf0
        .offset(6 as core::ffi::c_int as isize);
    *bf1.offset(7 as core::ffi::c_int as isize) = *bf0
        .offset(7 as core::ffi::c_int as isize);
    *bf1.offset(8 as core::ffi::c_int as isize) = *bf0
        .offset(8 as core::ffi::c_int as isize);
    *bf1.offset(9 as core::ffi::c_int as isize) = *bf0
        .offset(9 as core::ffi::c_int as isize);
    *bf1.offset(10 as core::ffi::c_int as isize) = *bf0
        .offset(10 as core::ffi::c_int as isize);
    *bf1.offset(11 as core::ffi::c_int as isize) = *bf0
        .offset(11 as core::ffi::c_int as isize);
    *bf1.offset(12 as core::ffi::c_int as isize) = *bf0
        .offset(12 as core::ffi::c_int as isize);
    *bf1.offset(13 as core::ffi::c_int as isize) = *bf0
        .offset(13 as core::ffi::c_int as isize);
    *bf1.offset(14 as core::ffi::c_int as isize) = *bf0
        .offset(14 as core::ffi::c_int as isize);
    *bf1.offset(15 as core::ffi::c_int as isize) = *bf0
        .offset(15 as core::ffi::c_int as isize);
    *bf1.offset(16 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(62 as core::ffi::c_int as isize),
        *bf0.offset(16 as core::ffi::c_int as isize),
        -*cospi.offset(2 as core::ffi::c_int as isize),
        *bf0.offset(31 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(17 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(30 as core::ffi::c_int as isize),
        *bf0.offset(17 as core::ffi::c_int as isize),
        -*cospi.offset(34 as core::ffi::c_int as isize),
        *bf0.offset(30 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(18 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(46 as core::ffi::c_int as isize),
        *bf0.offset(18 as core::ffi::c_int as isize),
        -*cospi.offset(18 as core::ffi::c_int as isize),
        *bf0.offset(29 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(19 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(14 as core::ffi::c_int as isize),
        *bf0.offset(19 as core::ffi::c_int as isize),
        -*cospi.offset(50 as core::ffi::c_int as isize),
        *bf0.offset(28 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(20 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(54 as core::ffi::c_int as isize),
        *bf0.offset(20 as core::ffi::c_int as isize),
        -*cospi.offset(10 as core::ffi::c_int as isize),
        *bf0.offset(27 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(21 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(22 as core::ffi::c_int as isize),
        *bf0.offset(21 as core::ffi::c_int as isize),
        -*cospi.offset(42 as core::ffi::c_int as isize),
        *bf0.offset(26 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(22 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(38 as core::ffi::c_int as isize),
        *bf0.offset(22 as core::ffi::c_int as isize),
        -*cospi.offset(26 as core::ffi::c_int as isize),
        *bf0.offset(25 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(23 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(6 as core::ffi::c_int as isize),
        *bf0.offset(23 as core::ffi::c_int as isize),
        -*cospi.offset(58 as core::ffi::c_int as isize),
        *bf0.offset(24 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(24 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(58 as core::ffi::c_int as isize),
        *bf0.offset(23 as core::ffi::c_int as isize),
        *cospi.offset(6 as core::ffi::c_int as isize),
        *bf0.offset(24 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(25 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(26 as core::ffi::c_int as isize),
        *bf0.offset(22 as core::ffi::c_int as isize),
        *cospi.offset(38 as core::ffi::c_int as isize),
        *bf0.offset(25 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(26 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(42 as core::ffi::c_int as isize),
        *bf0.offset(21 as core::ffi::c_int as isize),
        *cospi.offset(22 as core::ffi::c_int as isize),
        *bf0.offset(26 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(27 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(10 as core::ffi::c_int as isize),
        *bf0.offset(20 as core::ffi::c_int as isize),
        *cospi.offset(54 as core::ffi::c_int as isize),
        *bf0.offset(27 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(28 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(50 as core::ffi::c_int as isize),
        *bf0.offset(19 as core::ffi::c_int as isize),
        *cospi.offset(14 as core::ffi::c_int as isize),
        *bf0.offset(28 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(29 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(18 as core::ffi::c_int as isize),
        *bf0.offset(18 as core::ffi::c_int as isize),
        *cospi.offset(46 as core::ffi::c_int as isize),
        *bf0.offset(29 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(30 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(34 as core::ffi::c_int as isize),
        *bf0.offset(17 as core::ffi::c_int as isize),
        *cospi.offset(30 as core::ffi::c_int as isize),
        *bf0.offset(30 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(31 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(2 as core::ffi::c_int as isize),
        *bf0.offset(16 as core::ffi::c_int as isize),
        *cospi.offset(62 as core::ffi::c_int as isize),
        *bf0.offset(31 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(32 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(32 as core::ffi::c_int as isize)
            + *bf0.offset(33 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(33 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(32 as core::ffi::c_int as isize)
            - *bf0.offset(33 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(34 as core::ffi::c_int as isize) = clamp_value(
        -*bf0.offset(34 as core::ffi::c_int as isize)
            + *bf0.offset(35 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(35 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(34 as core::ffi::c_int as isize)
            + *bf0.offset(35 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(36 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(36 as core::ffi::c_int as isize)
            + *bf0.offset(37 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(37 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(36 as core::ffi::c_int as isize)
            - *bf0.offset(37 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(38 as core::ffi::c_int as isize) = clamp_value(
        -*bf0.offset(38 as core::ffi::c_int as isize)
            + *bf0.offset(39 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(39 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(38 as core::ffi::c_int as isize)
            + *bf0.offset(39 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(40 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(40 as core::ffi::c_int as isize)
            + *bf0.offset(41 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(41 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(40 as core::ffi::c_int as isize)
            - *bf0.offset(41 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(42 as core::ffi::c_int as isize) = clamp_value(
        -*bf0.offset(42 as core::ffi::c_int as isize)
            + *bf0.offset(43 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(43 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(42 as core::ffi::c_int as isize)
            + *bf0.offset(43 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(44 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(44 as core::ffi::c_int as isize)
            + *bf0.offset(45 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(45 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(44 as core::ffi::c_int as isize)
            - *bf0.offset(45 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(46 as core::ffi::c_int as isize) = clamp_value(
        -*bf0.offset(46 as core::ffi::c_int as isize)
            + *bf0.offset(47 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(47 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(46 as core::ffi::c_int as isize)
            + *bf0.offset(47 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(48 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(48 as core::ffi::c_int as isize)
            + *bf0.offset(49 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(49 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(48 as core::ffi::c_int as isize)
            - *bf0.offset(49 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(50 as core::ffi::c_int as isize) = clamp_value(
        -*bf0.offset(50 as core::ffi::c_int as isize)
            + *bf0.offset(51 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(51 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(50 as core::ffi::c_int as isize)
            + *bf0.offset(51 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(52 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(52 as core::ffi::c_int as isize)
            + *bf0.offset(53 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(53 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(52 as core::ffi::c_int as isize)
            - *bf0.offset(53 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(54 as core::ffi::c_int as isize) = clamp_value(
        -*bf0.offset(54 as core::ffi::c_int as isize)
            + *bf0.offset(55 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(55 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(54 as core::ffi::c_int as isize)
            + *bf0.offset(55 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(56 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(56 as core::ffi::c_int as isize)
            + *bf0.offset(57 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(57 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(56 as core::ffi::c_int as isize)
            - *bf0.offset(57 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(58 as core::ffi::c_int as isize) = clamp_value(
        -*bf0.offset(58 as core::ffi::c_int as isize)
            + *bf0.offset(59 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(59 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(58 as core::ffi::c_int as isize)
            + *bf0.offset(59 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(60 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(60 as core::ffi::c_int as isize)
            + *bf0.offset(61 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(61 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(60 as core::ffi::c_int as isize)
            - *bf0.offset(61 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(62 as core::ffi::c_int as isize) = clamp_value(
        -*bf0.offset(62 as core::ffi::c_int as isize)
            + *bf0.offset(63 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(63 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(62 as core::ffi::c_int as isize)
            + *bf0.offset(63 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    av1_range_check_buf(stage, input, bf1, size, *stage_range.offset(stage as isize));
    stage += 1;
    bf0 = output;
    bf1 = step.as_mut_ptr();
    *bf1.offset(0 as core::ffi::c_int as isize) = *bf0
        .offset(0 as core::ffi::c_int as isize);
    *bf1.offset(1 as core::ffi::c_int as isize) = *bf0
        .offset(1 as core::ffi::c_int as isize);
    *bf1.offset(2 as core::ffi::c_int as isize) = *bf0
        .offset(2 as core::ffi::c_int as isize);
    *bf1.offset(3 as core::ffi::c_int as isize) = *bf0
        .offset(3 as core::ffi::c_int as isize);
    *bf1.offset(4 as core::ffi::c_int as isize) = *bf0
        .offset(4 as core::ffi::c_int as isize);
    *bf1.offset(5 as core::ffi::c_int as isize) = *bf0
        .offset(5 as core::ffi::c_int as isize);
    *bf1.offset(6 as core::ffi::c_int as isize) = *bf0
        .offset(6 as core::ffi::c_int as isize);
    *bf1.offset(7 as core::ffi::c_int as isize) = *bf0
        .offset(7 as core::ffi::c_int as isize);
    *bf1.offset(8 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(60 as core::ffi::c_int as isize),
        *bf0.offset(8 as core::ffi::c_int as isize),
        -*cospi.offset(4 as core::ffi::c_int as isize),
        *bf0.offset(15 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(9 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(28 as core::ffi::c_int as isize),
        *bf0.offset(9 as core::ffi::c_int as isize),
        -*cospi.offset(36 as core::ffi::c_int as isize),
        *bf0.offset(14 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(10 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(44 as core::ffi::c_int as isize),
        *bf0.offset(10 as core::ffi::c_int as isize),
        -*cospi.offset(20 as core::ffi::c_int as isize),
        *bf0.offset(13 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(11 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(12 as core::ffi::c_int as isize),
        *bf0.offset(11 as core::ffi::c_int as isize),
        -*cospi.offset(52 as core::ffi::c_int as isize),
        *bf0.offset(12 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(12 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(52 as core::ffi::c_int as isize),
        *bf0.offset(11 as core::ffi::c_int as isize),
        *cospi.offset(12 as core::ffi::c_int as isize),
        *bf0.offset(12 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(13 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(20 as core::ffi::c_int as isize),
        *bf0.offset(10 as core::ffi::c_int as isize),
        *cospi.offset(44 as core::ffi::c_int as isize),
        *bf0.offset(13 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(14 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(36 as core::ffi::c_int as isize),
        *bf0.offset(9 as core::ffi::c_int as isize),
        *cospi.offset(28 as core::ffi::c_int as isize),
        *bf0.offset(14 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(15 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(4 as core::ffi::c_int as isize),
        *bf0.offset(8 as core::ffi::c_int as isize),
        *cospi.offset(60 as core::ffi::c_int as isize),
        *bf0.offset(15 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(16 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(16 as core::ffi::c_int as isize)
            + *bf0.offset(17 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(17 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(16 as core::ffi::c_int as isize)
            - *bf0.offset(17 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(18 as core::ffi::c_int as isize) = clamp_value(
        -*bf0.offset(18 as core::ffi::c_int as isize)
            + *bf0.offset(19 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(19 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(18 as core::ffi::c_int as isize)
            + *bf0.offset(19 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(20 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(20 as core::ffi::c_int as isize)
            + *bf0.offset(21 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(21 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(20 as core::ffi::c_int as isize)
            - *bf0.offset(21 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(22 as core::ffi::c_int as isize) = clamp_value(
        -*bf0.offset(22 as core::ffi::c_int as isize)
            + *bf0.offset(23 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(23 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(22 as core::ffi::c_int as isize)
            + *bf0.offset(23 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(24 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(24 as core::ffi::c_int as isize)
            + *bf0.offset(25 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(25 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(24 as core::ffi::c_int as isize)
            - *bf0.offset(25 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(26 as core::ffi::c_int as isize) = clamp_value(
        -*bf0.offset(26 as core::ffi::c_int as isize)
            + *bf0.offset(27 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(27 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(26 as core::ffi::c_int as isize)
            + *bf0.offset(27 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(28 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(28 as core::ffi::c_int as isize)
            + *bf0.offset(29 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(29 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(28 as core::ffi::c_int as isize)
            - *bf0.offset(29 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(30 as core::ffi::c_int as isize) = clamp_value(
        -*bf0.offset(30 as core::ffi::c_int as isize)
            + *bf0.offset(31 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(31 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(30 as core::ffi::c_int as isize)
            + *bf0.offset(31 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(32 as core::ffi::c_int as isize) = *bf0
        .offset(32 as core::ffi::c_int as isize);
    *bf1.offset(33 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(4 as core::ffi::c_int as isize),
        *bf0.offset(33 as core::ffi::c_int as isize),
        *cospi.offset(60 as core::ffi::c_int as isize),
        *bf0.offset(62 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(34 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(60 as core::ffi::c_int as isize),
        *bf0.offset(34 as core::ffi::c_int as isize),
        -*cospi.offset(4 as core::ffi::c_int as isize),
        *bf0.offset(61 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(35 as core::ffi::c_int as isize) = *bf0
        .offset(35 as core::ffi::c_int as isize);
    *bf1.offset(36 as core::ffi::c_int as isize) = *bf0
        .offset(36 as core::ffi::c_int as isize);
    *bf1.offset(37 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(36 as core::ffi::c_int as isize),
        *bf0.offset(37 as core::ffi::c_int as isize),
        *cospi.offset(28 as core::ffi::c_int as isize),
        *bf0.offset(58 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(38 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(28 as core::ffi::c_int as isize),
        *bf0.offset(38 as core::ffi::c_int as isize),
        -*cospi.offset(36 as core::ffi::c_int as isize),
        *bf0.offset(57 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(39 as core::ffi::c_int as isize) = *bf0
        .offset(39 as core::ffi::c_int as isize);
    *bf1.offset(40 as core::ffi::c_int as isize) = *bf0
        .offset(40 as core::ffi::c_int as isize);
    *bf1.offset(41 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(20 as core::ffi::c_int as isize),
        *bf0.offset(41 as core::ffi::c_int as isize),
        *cospi.offset(44 as core::ffi::c_int as isize),
        *bf0.offset(54 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(42 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(44 as core::ffi::c_int as isize),
        *bf0.offset(42 as core::ffi::c_int as isize),
        -*cospi.offset(20 as core::ffi::c_int as isize),
        *bf0.offset(53 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(43 as core::ffi::c_int as isize) = *bf0
        .offset(43 as core::ffi::c_int as isize);
    *bf1.offset(44 as core::ffi::c_int as isize) = *bf0
        .offset(44 as core::ffi::c_int as isize);
    *bf1.offset(45 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(52 as core::ffi::c_int as isize),
        *bf0.offset(45 as core::ffi::c_int as isize),
        *cospi.offset(12 as core::ffi::c_int as isize),
        *bf0.offset(50 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(46 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(12 as core::ffi::c_int as isize),
        *bf0.offset(46 as core::ffi::c_int as isize),
        -*cospi.offset(52 as core::ffi::c_int as isize),
        *bf0.offset(49 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(47 as core::ffi::c_int as isize) = *bf0
        .offset(47 as core::ffi::c_int as isize);
    *bf1.offset(48 as core::ffi::c_int as isize) = *bf0
        .offset(48 as core::ffi::c_int as isize);
    *bf1.offset(49 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(52 as core::ffi::c_int as isize),
        *bf0.offset(46 as core::ffi::c_int as isize),
        *cospi.offset(12 as core::ffi::c_int as isize),
        *bf0.offset(49 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(50 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(12 as core::ffi::c_int as isize),
        *bf0.offset(45 as core::ffi::c_int as isize),
        *cospi.offset(52 as core::ffi::c_int as isize),
        *bf0.offset(50 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(51 as core::ffi::c_int as isize) = *bf0
        .offset(51 as core::ffi::c_int as isize);
    *bf1.offset(52 as core::ffi::c_int as isize) = *bf0
        .offset(52 as core::ffi::c_int as isize);
    *bf1.offset(53 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(20 as core::ffi::c_int as isize),
        *bf0.offset(42 as core::ffi::c_int as isize),
        *cospi.offset(44 as core::ffi::c_int as isize),
        *bf0.offset(53 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(54 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(44 as core::ffi::c_int as isize),
        *bf0.offset(41 as core::ffi::c_int as isize),
        *cospi.offset(20 as core::ffi::c_int as isize),
        *bf0.offset(54 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(55 as core::ffi::c_int as isize) = *bf0
        .offset(55 as core::ffi::c_int as isize);
    *bf1.offset(56 as core::ffi::c_int as isize) = *bf0
        .offset(56 as core::ffi::c_int as isize);
    *bf1.offset(57 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(36 as core::ffi::c_int as isize),
        *bf0.offset(38 as core::ffi::c_int as isize),
        *cospi.offset(28 as core::ffi::c_int as isize),
        *bf0.offset(57 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(58 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(28 as core::ffi::c_int as isize),
        *bf0.offset(37 as core::ffi::c_int as isize),
        *cospi.offset(36 as core::ffi::c_int as isize),
        *bf0.offset(58 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(59 as core::ffi::c_int as isize) = *bf0
        .offset(59 as core::ffi::c_int as isize);
    *bf1.offset(60 as core::ffi::c_int as isize) = *bf0
        .offset(60 as core::ffi::c_int as isize);
    *bf1.offset(61 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(4 as core::ffi::c_int as isize),
        *bf0.offset(34 as core::ffi::c_int as isize),
        *cospi.offset(60 as core::ffi::c_int as isize),
        *bf0.offset(61 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(62 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(60 as core::ffi::c_int as isize),
        *bf0.offset(33 as core::ffi::c_int as isize),
        *cospi.offset(4 as core::ffi::c_int as isize),
        *bf0.offset(62 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(63 as core::ffi::c_int as isize) = *bf0
        .offset(63 as core::ffi::c_int as isize);
    av1_range_check_buf(stage, input, bf1, size, *stage_range.offset(stage as isize));
    stage += 1;
    bf0 = step.as_mut_ptr();
    bf1 = output;
    *bf1.offset(0 as core::ffi::c_int as isize) = *bf0
        .offset(0 as core::ffi::c_int as isize);
    *bf1.offset(1 as core::ffi::c_int as isize) = *bf0
        .offset(1 as core::ffi::c_int as isize);
    *bf1.offset(2 as core::ffi::c_int as isize) = *bf0
        .offset(2 as core::ffi::c_int as isize);
    *bf1.offset(3 as core::ffi::c_int as isize) = *bf0
        .offset(3 as core::ffi::c_int as isize);
    *bf1.offset(4 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(56 as core::ffi::c_int as isize),
        *bf0.offset(4 as core::ffi::c_int as isize),
        -*cospi.offset(8 as core::ffi::c_int as isize),
        *bf0.offset(7 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(5 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(24 as core::ffi::c_int as isize),
        *bf0.offset(5 as core::ffi::c_int as isize),
        -*cospi.offset(40 as core::ffi::c_int as isize),
        *bf0.offset(6 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(6 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(40 as core::ffi::c_int as isize),
        *bf0.offset(5 as core::ffi::c_int as isize),
        *cospi.offset(24 as core::ffi::c_int as isize),
        *bf0.offset(6 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(7 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(8 as core::ffi::c_int as isize),
        *bf0.offset(4 as core::ffi::c_int as isize),
        *cospi.offset(56 as core::ffi::c_int as isize),
        *bf0.offset(7 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(8 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(8 as core::ffi::c_int as isize)
            + *bf0.offset(9 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(9 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(8 as core::ffi::c_int as isize)
            - *bf0.offset(9 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(10 as core::ffi::c_int as isize) = clamp_value(
        -*bf0.offset(10 as core::ffi::c_int as isize)
            + *bf0.offset(11 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(11 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(10 as core::ffi::c_int as isize)
            + *bf0.offset(11 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(12 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(12 as core::ffi::c_int as isize)
            + *bf0.offset(13 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(13 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(12 as core::ffi::c_int as isize)
            - *bf0.offset(13 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(14 as core::ffi::c_int as isize) = clamp_value(
        -*bf0.offset(14 as core::ffi::c_int as isize)
            + *bf0.offset(15 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(15 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(14 as core::ffi::c_int as isize)
            + *bf0.offset(15 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(16 as core::ffi::c_int as isize) = *bf0
        .offset(16 as core::ffi::c_int as isize);
    *bf1.offset(17 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(8 as core::ffi::c_int as isize),
        *bf0.offset(17 as core::ffi::c_int as isize),
        *cospi.offset(56 as core::ffi::c_int as isize),
        *bf0.offset(30 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(18 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(56 as core::ffi::c_int as isize),
        *bf0.offset(18 as core::ffi::c_int as isize),
        -*cospi.offset(8 as core::ffi::c_int as isize),
        *bf0.offset(29 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(19 as core::ffi::c_int as isize) = *bf0
        .offset(19 as core::ffi::c_int as isize);
    *bf1.offset(20 as core::ffi::c_int as isize) = *bf0
        .offset(20 as core::ffi::c_int as isize);
    *bf1.offset(21 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(40 as core::ffi::c_int as isize),
        *bf0.offset(21 as core::ffi::c_int as isize),
        *cospi.offset(24 as core::ffi::c_int as isize),
        *bf0.offset(26 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(22 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(24 as core::ffi::c_int as isize),
        *bf0.offset(22 as core::ffi::c_int as isize),
        -*cospi.offset(40 as core::ffi::c_int as isize),
        *bf0.offset(25 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(23 as core::ffi::c_int as isize) = *bf0
        .offset(23 as core::ffi::c_int as isize);
    *bf1.offset(24 as core::ffi::c_int as isize) = *bf0
        .offset(24 as core::ffi::c_int as isize);
    *bf1.offset(25 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(40 as core::ffi::c_int as isize),
        *bf0.offset(22 as core::ffi::c_int as isize),
        *cospi.offset(24 as core::ffi::c_int as isize),
        *bf0.offset(25 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(26 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(24 as core::ffi::c_int as isize),
        *bf0.offset(21 as core::ffi::c_int as isize),
        *cospi.offset(40 as core::ffi::c_int as isize),
        *bf0.offset(26 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(27 as core::ffi::c_int as isize) = *bf0
        .offset(27 as core::ffi::c_int as isize);
    *bf1.offset(28 as core::ffi::c_int as isize) = *bf0
        .offset(28 as core::ffi::c_int as isize);
    *bf1.offset(29 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(8 as core::ffi::c_int as isize),
        *bf0.offset(18 as core::ffi::c_int as isize),
        *cospi.offset(56 as core::ffi::c_int as isize),
        *bf0.offset(29 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(30 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(56 as core::ffi::c_int as isize),
        *bf0.offset(17 as core::ffi::c_int as isize),
        *cospi.offset(8 as core::ffi::c_int as isize),
        *bf0.offset(30 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(31 as core::ffi::c_int as isize) = *bf0
        .offset(31 as core::ffi::c_int as isize);
    *bf1.offset(32 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(32 as core::ffi::c_int as isize)
            + *bf0.offset(35 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(33 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(33 as core::ffi::c_int as isize)
            + *bf0.offset(34 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(34 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(33 as core::ffi::c_int as isize)
            - *bf0.offset(34 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(35 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(32 as core::ffi::c_int as isize)
            - *bf0.offset(35 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(36 as core::ffi::c_int as isize) = clamp_value(
        -*bf0.offset(36 as core::ffi::c_int as isize)
            + *bf0.offset(39 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(37 as core::ffi::c_int as isize) = clamp_value(
        -*bf0.offset(37 as core::ffi::c_int as isize)
            + *bf0.offset(38 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(38 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(37 as core::ffi::c_int as isize)
            + *bf0.offset(38 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(39 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(36 as core::ffi::c_int as isize)
            + *bf0.offset(39 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(40 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(40 as core::ffi::c_int as isize)
            + *bf0.offset(43 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(41 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(41 as core::ffi::c_int as isize)
            + *bf0.offset(42 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(42 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(41 as core::ffi::c_int as isize)
            - *bf0.offset(42 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(43 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(40 as core::ffi::c_int as isize)
            - *bf0.offset(43 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(44 as core::ffi::c_int as isize) = clamp_value(
        -*bf0.offset(44 as core::ffi::c_int as isize)
            + *bf0.offset(47 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(45 as core::ffi::c_int as isize) = clamp_value(
        -*bf0.offset(45 as core::ffi::c_int as isize)
            + *bf0.offset(46 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(46 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(45 as core::ffi::c_int as isize)
            + *bf0.offset(46 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(47 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(44 as core::ffi::c_int as isize)
            + *bf0.offset(47 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(48 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(48 as core::ffi::c_int as isize)
            + *bf0.offset(51 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(49 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(49 as core::ffi::c_int as isize)
            + *bf0.offset(50 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(50 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(49 as core::ffi::c_int as isize)
            - *bf0.offset(50 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(51 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(48 as core::ffi::c_int as isize)
            - *bf0.offset(51 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(52 as core::ffi::c_int as isize) = clamp_value(
        -*bf0.offset(52 as core::ffi::c_int as isize)
            + *bf0.offset(55 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(53 as core::ffi::c_int as isize) = clamp_value(
        -*bf0.offset(53 as core::ffi::c_int as isize)
            + *bf0.offset(54 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(54 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(53 as core::ffi::c_int as isize)
            + *bf0.offset(54 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(55 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(52 as core::ffi::c_int as isize)
            + *bf0.offset(55 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(56 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(56 as core::ffi::c_int as isize)
            + *bf0.offset(59 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(57 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(57 as core::ffi::c_int as isize)
            + *bf0.offset(58 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(58 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(57 as core::ffi::c_int as isize)
            - *bf0.offset(58 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(59 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(56 as core::ffi::c_int as isize)
            - *bf0.offset(59 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(60 as core::ffi::c_int as isize) = clamp_value(
        -*bf0.offset(60 as core::ffi::c_int as isize)
            + *bf0.offset(63 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(61 as core::ffi::c_int as isize) = clamp_value(
        -*bf0.offset(61 as core::ffi::c_int as isize)
            + *bf0.offset(62 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(62 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(61 as core::ffi::c_int as isize)
            + *bf0.offset(62 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(63 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(60 as core::ffi::c_int as isize)
            + *bf0.offset(63 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    av1_range_check_buf(stage, input, bf1, size, *stage_range.offset(stage as isize));
    stage += 1;
    bf0 = output;
    bf1 = step.as_mut_ptr();
    *bf1.offset(0 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(0 as core::ffi::c_int as isize),
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(1 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(1 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(0 as core::ffi::c_int as isize),
        -*cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(1 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(2 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(48 as core::ffi::c_int as isize),
        *bf0.offset(2 as core::ffi::c_int as isize),
        -*cospi.offset(16 as core::ffi::c_int as isize),
        *bf0.offset(3 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(3 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(16 as core::ffi::c_int as isize),
        *bf0.offset(2 as core::ffi::c_int as isize),
        *cospi.offset(48 as core::ffi::c_int as isize),
        *bf0.offset(3 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(4 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(4 as core::ffi::c_int as isize)
            + *bf0.offset(5 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(5 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(4 as core::ffi::c_int as isize)
            - *bf0.offset(5 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(6 as core::ffi::c_int as isize) = clamp_value(
        -*bf0.offset(6 as core::ffi::c_int as isize)
            + *bf0.offset(7 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(7 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(6 as core::ffi::c_int as isize)
            + *bf0.offset(7 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(8 as core::ffi::c_int as isize) = *bf0
        .offset(8 as core::ffi::c_int as isize);
    *bf1.offset(9 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(16 as core::ffi::c_int as isize),
        *bf0.offset(9 as core::ffi::c_int as isize),
        *cospi.offset(48 as core::ffi::c_int as isize),
        *bf0.offset(14 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(10 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(48 as core::ffi::c_int as isize),
        *bf0.offset(10 as core::ffi::c_int as isize),
        -*cospi.offset(16 as core::ffi::c_int as isize),
        *bf0.offset(13 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(11 as core::ffi::c_int as isize) = *bf0
        .offset(11 as core::ffi::c_int as isize);
    *bf1.offset(12 as core::ffi::c_int as isize) = *bf0
        .offset(12 as core::ffi::c_int as isize);
    *bf1.offset(13 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(16 as core::ffi::c_int as isize),
        *bf0.offset(10 as core::ffi::c_int as isize),
        *cospi.offset(48 as core::ffi::c_int as isize),
        *bf0.offset(13 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(14 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(48 as core::ffi::c_int as isize),
        *bf0.offset(9 as core::ffi::c_int as isize),
        *cospi.offset(16 as core::ffi::c_int as isize),
        *bf0.offset(14 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(15 as core::ffi::c_int as isize) = *bf0
        .offset(15 as core::ffi::c_int as isize);
    *bf1.offset(16 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(16 as core::ffi::c_int as isize)
            + *bf0.offset(19 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(17 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(17 as core::ffi::c_int as isize)
            + *bf0.offset(18 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(18 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(17 as core::ffi::c_int as isize)
            - *bf0.offset(18 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(19 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(16 as core::ffi::c_int as isize)
            - *bf0.offset(19 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(20 as core::ffi::c_int as isize) = clamp_value(
        -*bf0.offset(20 as core::ffi::c_int as isize)
            + *bf0.offset(23 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(21 as core::ffi::c_int as isize) = clamp_value(
        -*bf0.offset(21 as core::ffi::c_int as isize)
            + *bf0.offset(22 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(22 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(21 as core::ffi::c_int as isize)
            + *bf0.offset(22 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(23 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(20 as core::ffi::c_int as isize)
            + *bf0.offset(23 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(24 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(24 as core::ffi::c_int as isize)
            + *bf0.offset(27 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(25 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(25 as core::ffi::c_int as isize)
            + *bf0.offset(26 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(26 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(25 as core::ffi::c_int as isize)
            - *bf0.offset(26 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(27 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(24 as core::ffi::c_int as isize)
            - *bf0.offset(27 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(28 as core::ffi::c_int as isize) = clamp_value(
        -*bf0.offset(28 as core::ffi::c_int as isize)
            + *bf0.offset(31 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(29 as core::ffi::c_int as isize) = clamp_value(
        -*bf0.offset(29 as core::ffi::c_int as isize)
            + *bf0.offset(30 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(30 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(29 as core::ffi::c_int as isize)
            + *bf0.offset(30 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(31 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(28 as core::ffi::c_int as isize)
            + *bf0.offset(31 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(32 as core::ffi::c_int as isize) = *bf0
        .offset(32 as core::ffi::c_int as isize);
    *bf1.offset(33 as core::ffi::c_int as isize) = *bf0
        .offset(33 as core::ffi::c_int as isize);
    *bf1.offset(34 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(8 as core::ffi::c_int as isize),
        *bf0.offset(34 as core::ffi::c_int as isize),
        *cospi.offset(56 as core::ffi::c_int as isize),
        *bf0.offset(61 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(35 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(8 as core::ffi::c_int as isize),
        *bf0.offset(35 as core::ffi::c_int as isize),
        *cospi.offset(56 as core::ffi::c_int as isize),
        *bf0.offset(60 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(36 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(56 as core::ffi::c_int as isize),
        *bf0.offset(36 as core::ffi::c_int as isize),
        -*cospi.offset(8 as core::ffi::c_int as isize),
        *bf0.offset(59 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(37 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(56 as core::ffi::c_int as isize),
        *bf0.offset(37 as core::ffi::c_int as isize),
        -*cospi.offset(8 as core::ffi::c_int as isize),
        *bf0.offset(58 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(38 as core::ffi::c_int as isize) = *bf0
        .offset(38 as core::ffi::c_int as isize);
    *bf1.offset(39 as core::ffi::c_int as isize) = *bf0
        .offset(39 as core::ffi::c_int as isize);
    *bf1.offset(40 as core::ffi::c_int as isize) = *bf0
        .offset(40 as core::ffi::c_int as isize);
    *bf1.offset(41 as core::ffi::c_int as isize) = *bf0
        .offset(41 as core::ffi::c_int as isize);
    *bf1.offset(42 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(40 as core::ffi::c_int as isize),
        *bf0.offset(42 as core::ffi::c_int as isize),
        *cospi.offset(24 as core::ffi::c_int as isize),
        *bf0.offset(53 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(43 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(40 as core::ffi::c_int as isize),
        *bf0.offset(43 as core::ffi::c_int as isize),
        *cospi.offset(24 as core::ffi::c_int as isize),
        *bf0.offset(52 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(44 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(24 as core::ffi::c_int as isize),
        *bf0.offset(44 as core::ffi::c_int as isize),
        -*cospi.offset(40 as core::ffi::c_int as isize),
        *bf0.offset(51 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(45 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(24 as core::ffi::c_int as isize),
        *bf0.offset(45 as core::ffi::c_int as isize),
        -*cospi.offset(40 as core::ffi::c_int as isize),
        *bf0.offset(50 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(46 as core::ffi::c_int as isize) = *bf0
        .offset(46 as core::ffi::c_int as isize);
    *bf1.offset(47 as core::ffi::c_int as isize) = *bf0
        .offset(47 as core::ffi::c_int as isize);
    *bf1.offset(48 as core::ffi::c_int as isize) = *bf0
        .offset(48 as core::ffi::c_int as isize);
    *bf1.offset(49 as core::ffi::c_int as isize) = *bf0
        .offset(49 as core::ffi::c_int as isize);
    *bf1.offset(50 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(40 as core::ffi::c_int as isize),
        *bf0.offset(45 as core::ffi::c_int as isize),
        *cospi.offset(24 as core::ffi::c_int as isize),
        *bf0.offset(50 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(51 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(40 as core::ffi::c_int as isize),
        *bf0.offset(44 as core::ffi::c_int as isize),
        *cospi.offset(24 as core::ffi::c_int as isize),
        *bf0.offset(51 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(52 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(24 as core::ffi::c_int as isize),
        *bf0.offset(43 as core::ffi::c_int as isize),
        *cospi.offset(40 as core::ffi::c_int as isize),
        *bf0.offset(52 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(53 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(24 as core::ffi::c_int as isize),
        *bf0.offset(42 as core::ffi::c_int as isize),
        *cospi.offset(40 as core::ffi::c_int as isize),
        *bf0.offset(53 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(54 as core::ffi::c_int as isize) = *bf0
        .offset(54 as core::ffi::c_int as isize);
    *bf1.offset(55 as core::ffi::c_int as isize) = *bf0
        .offset(55 as core::ffi::c_int as isize);
    *bf1.offset(56 as core::ffi::c_int as isize) = *bf0
        .offset(56 as core::ffi::c_int as isize);
    *bf1.offset(57 as core::ffi::c_int as isize) = *bf0
        .offset(57 as core::ffi::c_int as isize);
    *bf1.offset(58 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(8 as core::ffi::c_int as isize),
        *bf0.offset(37 as core::ffi::c_int as isize),
        *cospi.offset(56 as core::ffi::c_int as isize),
        *bf0.offset(58 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(59 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(8 as core::ffi::c_int as isize),
        *bf0.offset(36 as core::ffi::c_int as isize),
        *cospi.offset(56 as core::ffi::c_int as isize),
        *bf0.offset(59 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(60 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(56 as core::ffi::c_int as isize),
        *bf0.offset(35 as core::ffi::c_int as isize),
        *cospi.offset(8 as core::ffi::c_int as isize),
        *bf0.offset(60 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(61 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(56 as core::ffi::c_int as isize),
        *bf0.offset(34 as core::ffi::c_int as isize),
        *cospi.offset(8 as core::ffi::c_int as isize),
        *bf0.offset(61 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(62 as core::ffi::c_int as isize) = *bf0
        .offset(62 as core::ffi::c_int as isize);
    *bf1.offset(63 as core::ffi::c_int as isize) = *bf0
        .offset(63 as core::ffi::c_int as isize);
    av1_range_check_buf(stage, input, bf1, size, *stage_range.offset(stage as isize));
    stage += 1;
    bf0 = step.as_mut_ptr();
    bf1 = output;
    *bf1.offset(0 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(0 as core::ffi::c_int as isize)
            + *bf0.offset(3 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(1 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(1 as core::ffi::c_int as isize)
            + *bf0.offset(2 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(2 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(1 as core::ffi::c_int as isize)
            - *bf0.offset(2 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(3 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(0 as core::ffi::c_int as isize)
            - *bf0.offset(3 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(4 as core::ffi::c_int as isize) = *bf0
        .offset(4 as core::ffi::c_int as isize);
    *bf1.offset(5 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(5 as core::ffi::c_int as isize),
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(6 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(6 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(5 as core::ffi::c_int as isize),
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(6 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(7 as core::ffi::c_int as isize) = *bf0
        .offset(7 as core::ffi::c_int as isize);
    *bf1.offset(8 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(8 as core::ffi::c_int as isize)
            + *bf0.offset(11 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(9 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(9 as core::ffi::c_int as isize)
            + *bf0.offset(10 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(10 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(9 as core::ffi::c_int as isize)
            - *bf0.offset(10 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(11 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(8 as core::ffi::c_int as isize)
            - *bf0.offset(11 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(12 as core::ffi::c_int as isize) = clamp_value(
        -*bf0.offset(12 as core::ffi::c_int as isize)
            + *bf0.offset(15 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(13 as core::ffi::c_int as isize) = clamp_value(
        -*bf0.offset(13 as core::ffi::c_int as isize)
            + *bf0.offset(14 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(14 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(13 as core::ffi::c_int as isize)
            + *bf0.offset(14 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(15 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(12 as core::ffi::c_int as isize)
            + *bf0.offset(15 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(16 as core::ffi::c_int as isize) = *bf0
        .offset(16 as core::ffi::c_int as isize);
    *bf1.offset(17 as core::ffi::c_int as isize) = *bf0
        .offset(17 as core::ffi::c_int as isize);
    *bf1.offset(18 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(16 as core::ffi::c_int as isize),
        *bf0.offset(18 as core::ffi::c_int as isize),
        *cospi.offset(48 as core::ffi::c_int as isize),
        *bf0.offset(29 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(19 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(16 as core::ffi::c_int as isize),
        *bf0.offset(19 as core::ffi::c_int as isize),
        *cospi.offset(48 as core::ffi::c_int as isize),
        *bf0.offset(28 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(20 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(48 as core::ffi::c_int as isize),
        *bf0.offset(20 as core::ffi::c_int as isize),
        -*cospi.offset(16 as core::ffi::c_int as isize),
        *bf0.offset(27 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(21 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(48 as core::ffi::c_int as isize),
        *bf0.offset(21 as core::ffi::c_int as isize),
        -*cospi.offset(16 as core::ffi::c_int as isize),
        *bf0.offset(26 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(22 as core::ffi::c_int as isize) = *bf0
        .offset(22 as core::ffi::c_int as isize);
    *bf1.offset(23 as core::ffi::c_int as isize) = *bf0
        .offset(23 as core::ffi::c_int as isize);
    *bf1.offset(24 as core::ffi::c_int as isize) = *bf0
        .offset(24 as core::ffi::c_int as isize);
    *bf1.offset(25 as core::ffi::c_int as isize) = *bf0
        .offset(25 as core::ffi::c_int as isize);
    *bf1.offset(26 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(16 as core::ffi::c_int as isize),
        *bf0.offset(21 as core::ffi::c_int as isize),
        *cospi.offset(48 as core::ffi::c_int as isize),
        *bf0.offset(26 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(27 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(16 as core::ffi::c_int as isize),
        *bf0.offset(20 as core::ffi::c_int as isize),
        *cospi.offset(48 as core::ffi::c_int as isize),
        *bf0.offset(27 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(28 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(48 as core::ffi::c_int as isize),
        *bf0.offset(19 as core::ffi::c_int as isize),
        *cospi.offset(16 as core::ffi::c_int as isize),
        *bf0.offset(28 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(29 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(48 as core::ffi::c_int as isize),
        *bf0.offset(18 as core::ffi::c_int as isize),
        *cospi.offset(16 as core::ffi::c_int as isize),
        *bf0.offset(29 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(30 as core::ffi::c_int as isize) = *bf0
        .offset(30 as core::ffi::c_int as isize);
    *bf1.offset(31 as core::ffi::c_int as isize) = *bf0
        .offset(31 as core::ffi::c_int as isize);
    *bf1.offset(32 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(32 as core::ffi::c_int as isize)
            + *bf0.offset(39 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(33 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(33 as core::ffi::c_int as isize)
            + *bf0.offset(38 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(34 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(34 as core::ffi::c_int as isize)
            + *bf0.offset(37 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(35 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(35 as core::ffi::c_int as isize)
            + *bf0.offset(36 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(36 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(35 as core::ffi::c_int as isize)
            - *bf0.offset(36 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(37 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(34 as core::ffi::c_int as isize)
            - *bf0.offset(37 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(38 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(33 as core::ffi::c_int as isize)
            - *bf0.offset(38 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(39 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(32 as core::ffi::c_int as isize)
            - *bf0.offset(39 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(40 as core::ffi::c_int as isize) = clamp_value(
        -*bf0.offset(40 as core::ffi::c_int as isize)
            + *bf0.offset(47 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(41 as core::ffi::c_int as isize) = clamp_value(
        -*bf0.offset(41 as core::ffi::c_int as isize)
            + *bf0.offset(46 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(42 as core::ffi::c_int as isize) = clamp_value(
        -*bf0.offset(42 as core::ffi::c_int as isize)
            + *bf0.offset(45 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(43 as core::ffi::c_int as isize) = clamp_value(
        -*bf0.offset(43 as core::ffi::c_int as isize)
            + *bf0.offset(44 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(44 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(43 as core::ffi::c_int as isize)
            + *bf0.offset(44 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(45 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(42 as core::ffi::c_int as isize)
            + *bf0.offset(45 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(46 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(41 as core::ffi::c_int as isize)
            + *bf0.offset(46 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(47 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(40 as core::ffi::c_int as isize)
            + *bf0.offset(47 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(48 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(48 as core::ffi::c_int as isize)
            + *bf0.offset(55 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(49 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(49 as core::ffi::c_int as isize)
            + *bf0.offset(54 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(50 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(50 as core::ffi::c_int as isize)
            + *bf0.offset(53 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(51 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(51 as core::ffi::c_int as isize)
            + *bf0.offset(52 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(52 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(51 as core::ffi::c_int as isize)
            - *bf0.offset(52 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(53 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(50 as core::ffi::c_int as isize)
            - *bf0.offset(53 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(54 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(49 as core::ffi::c_int as isize)
            - *bf0.offset(54 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(55 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(48 as core::ffi::c_int as isize)
            - *bf0.offset(55 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(56 as core::ffi::c_int as isize) = clamp_value(
        -*bf0.offset(56 as core::ffi::c_int as isize)
            + *bf0.offset(63 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(57 as core::ffi::c_int as isize) = clamp_value(
        -*bf0.offset(57 as core::ffi::c_int as isize)
            + *bf0.offset(62 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(58 as core::ffi::c_int as isize) = clamp_value(
        -*bf0.offset(58 as core::ffi::c_int as isize)
            + *bf0.offset(61 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(59 as core::ffi::c_int as isize) = clamp_value(
        -*bf0.offset(59 as core::ffi::c_int as isize)
            + *bf0.offset(60 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(60 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(59 as core::ffi::c_int as isize)
            + *bf0.offset(60 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(61 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(58 as core::ffi::c_int as isize)
            + *bf0.offset(61 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(62 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(57 as core::ffi::c_int as isize)
            + *bf0.offset(62 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(63 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(56 as core::ffi::c_int as isize)
            + *bf0.offset(63 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    av1_range_check_buf(stage, input, bf1, size, *stage_range.offset(stage as isize));
    stage += 1;
    bf0 = output;
    bf1 = step.as_mut_ptr();
    *bf1.offset(0 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(0 as core::ffi::c_int as isize)
            + *bf0.offset(7 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(1 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(1 as core::ffi::c_int as isize)
            + *bf0.offset(6 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(2 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(2 as core::ffi::c_int as isize)
            + *bf0.offset(5 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(3 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(3 as core::ffi::c_int as isize)
            + *bf0.offset(4 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(4 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(3 as core::ffi::c_int as isize)
            - *bf0.offset(4 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(5 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(2 as core::ffi::c_int as isize)
            - *bf0.offset(5 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(6 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(1 as core::ffi::c_int as isize)
            - *bf0.offset(6 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(7 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(0 as core::ffi::c_int as isize)
            - *bf0.offset(7 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(8 as core::ffi::c_int as isize) = *bf0
        .offset(8 as core::ffi::c_int as isize);
    *bf1.offset(9 as core::ffi::c_int as isize) = *bf0
        .offset(9 as core::ffi::c_int as isize);
    *bf1.offset(10 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(10 as core::ffi::c_int as isize),
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(13 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(11 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(11 as core::ffi::c_int as isize),
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(12 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(12 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(11 as core::ffi::c_int as isize),
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(12 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(13 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(10 as core::ffi::c_int as isize),
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(13 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(14 as core::ffi::c_int as isize) = *bf0
        .offset(14 as core::ffi::c_int as isize);
    *bf1.offset(15 as core::ffi::c_int as isize) = *bf0
        .offset(15 as core::ffi::c_int as isize);
    *bf1.offset(16 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(16 as core::ffi::c_int as isize)
            + *bf0.offset(23 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(17 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(17 as core::ffi::c_int as isize)
            + *bf0.offset(22 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(18 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(18 as core::ffi::c_int as isize)
            + *bf0.offset(21 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(19 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(19 as core::ffi::c_int as isize)
            + *bf0.offset(20 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(20 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(19 as core::ffi::c_int as isize)
            - *bf0.offset(20 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(21 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(18 as core::ffi::c_int as isize)
            - *bf0.offset(21 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(22 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(17 as core::ffi::c_int as isize)
            - *bf0.offset(22 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(23 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(16 as core::ffi::c_int as isize)
            - *bf0.offset(23 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(24 as core::ffi::c_int as isize) = clamp_value(
        -*bf0.offset(24 as core::ffi::c_int as isize)
            + *bf0.offset(31 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(25 as core::ffi::c_int as isize) = clamp_value(
        -*bf0.offset(25 as core::ffi::c_int as isize)
            + *bf0.offset(30 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(26 as core::ffi::c_int as isize) = clamp_value(
        -*bf0.offset(26 as core::ffi::c_int as isize)
            + *bf0.offset(29 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(27 as core::ffi::c_int as isize) = clamp_value(
        -*bf0.offset(27 as core::ffi::c_int as isize)
            + *bf0.offset(28 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(28 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(27 as core::ffi::c_int as isize)
            + *bf0.offset(28 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(29 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(26 as core::ffi::c_int as isize)
            + *bf0.offset(29 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(30 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(25 as core::ffi::c_int as isize)
            + *bf0.offset(30 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(31 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(24 as core::ffi::c_int as isize)
            + *bf0.offset(31 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(32 as core::ffi::c_int as isize) = *bf0
        .offset(32 as core::ffi::c_int as isize);
    *bf1.offset(33 as core::ffi::c_int as isize) = *bf0
        .offset(33 as core::ffi::c_int as isize);
    *bf1.offset(34 as core::ffi::c_int as isize) = *bf0
        .offset(34 as core::ffi::c_int as isize);
    *bf1.offset(35 as core::ffi::c_int as isize) = *bf0
        .offset(35 as core::ffi::c_int as isize);
    *bf1.offset(36 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(16 as core::ffi::c_int as isize),
        *bf0.offset(36 as core::ffi::c_int as isize),
        *cospi.offset(48 as core::ffi::c_int as isize),
        *bf0.offset(59 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(37 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(16 as core::ffi::c_int as isize),
        *bf0.offset(37 as core::ffi::c_int as isize),
        *cospi.offset(48 as core::ffi::c_int as isize),
        *bf0.offset(58 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(38 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(16 as core::ffi::c_int as isize),
        *bf0.offset(38 as core::ffi::c_int as isize),
        *cospi.offset(48 as core::ffi::c_int as isize),
        *bf0.offset(57 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(39 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(16 as core::ffi::c_int as isize),
        *bf0.offset(39 as core::ffi::c_int as isize),
        *cospi.offset(48 as core::ffi::c_int as isize),
        *bf0.offset(56 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(40 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(48 as core::ffi::c_int as isize),
        *bf0.offset(40 as core::ffi::c_int as isize),
        -*cospi.offset(16 as core::ffi::c_int as isize),
        *bf0.offset(55 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(41 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(48 as core::ffi::c_int as isize),
        *bf0.offset(41 as core::ffi::c_int as isize),
        -*cospi.offset(16 as core::ffi::c_int as isize),
        *bf0.offset(54 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(42 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(48 as core::ffi::c_int as isize),
        *bf0.offset(42 as core::ffi::c_int as isize),
        -*cospi.offset(16 as core::ffi::c_int as isize),
        *bf0.offset(53 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(43 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(48 as core::ffi::c_int as isize),
        *bf0.offset(43 as core::ffi::c_int as isize),
        -*cospi.offset(16 as core::ffi::c_int as isize),
        *bf0.offset(52 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(44 as core::ffi::c_int as isize) = *bf0
        .offset(44 as core::ffi::c_int as isize);
    *bf1.offset(45 as core::ffi::c_int as isize) = *bf0
        .offset(45 as core::ffi::c_int as isize);
    *bf1.offset(46 as core::ffi::c_int as isize) = *bf0
        .offset(46 as core::ffi::c_int as isize);
    *bf1.offset(47 as core::ffi::c_int as isize) = *bf0
        .offset(47 as core::ffi::c_int as isize);
    *bf1.offset(48 as core::ffi::c_int as isize) = *bf0
        .offset(48 as core::ffi::c_int as isize);
    *bf1.offset(49 as core::ffi::c_int as isize) = *bf0
        .offset(49 as core::ffi::c_int as isize);
    *bf1.offset(50 as core::ffi::c_int as isize) = *bf0
        .offset(50 as core::ffi::c_int as isize);
    *bf1.offset(51 as core::ffi::c_int as isize) = *bf0
        .offset(51 as core::ffi::c_int as isize);
    *bf1.offset(52 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(16 as core::ffi::c_int as isize),
        *bf0.offset(43 as core::ffi::c_int as isize),
        *cospi.offset(48 as core::ffi::c_int as isize),
        *bf0.offset(52 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(53 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(16 as core::ffi::c_int as isize),
        *bf0.offset(42 as core::ffi::c_int as isize),
        *cospi.offset(48 as core::ffi::c_int as isize),
        *bf0.offset(53 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(54 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(16 as core::ffi::c_int as isize),
        *bf0.offset(41 as core::ffi::c_int as isize),
        *cospi.offset(48 as core::ffi::c_int as isize),
        *bf0.offset(54 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(55 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(16 as core::ffi::c_int as isize),
        *bf0.offset(40 as core::ffi::c_int as isize),
        *cospi.offset(48 as core::ffi::c_int as isize),
        *bf0.offset(55 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(56 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(48 as core::ffi::c_int as isize),
        *bf0.offset(39 as core::ffi::c_int as isize),
        *cospi.offset(16 as core::ffi::c_int as isize),
        *bf0.offset(56 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(57 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(48 as core::ffi::c_int as isize),
        *bf0.offset(38 as core::ffi::c_int as isize),
        *cospi.offset(16 as core::ffi::c_int as isize),
        *bf0.offset(57 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(58 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(48 as core::ffi::c_int as isize),
        *bf0.offset(37 as core::ffi::c_int as isize),
        *cospi.offset(16 as core::ffi::c_int as isize),
        *bf0.offset(58 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(59 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(48 as core::ffi::c_int as isize),
        *bf0.offset(36 as core::ffi::c_int as isize),
        *cospi.offset(16 as core::ffi::c_int as isize),
        *bf0.offset(59 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(60 as core::ffi::c_int as isize) = *bf0
        .offset(60 as core::ffi::c_int as isize);
    *bf1.offset(61 as core::ffi::c_int as isize) = *bf0
        .offset(61 as core::ffi::c_int as isize);
    *bf1.offset(62 as core::ffi::c_int as isize) = *bf0
        .offset(62 as core::ffi::c_int as isize);
    *bf1.offset(63 as core::ffi::c_int as isize) = *bf0
        .offset(63 as core::ffi::c_int as isize);
    av1_range_check_buf(stage, input, bf1, size, *stage_range.offset(stage as isize));
    stage += 1;
    bf0 = step.as_mut_ptr();
    bf1 = output;
    *bf1.offset(0 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(0 as core::ffi::c_int as isize)
            + *bf0.offset(15 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(1 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(1 as core::ffi::c_int as isize)
            + *bf0.offset(14 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(2 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(2 as core::ffi::c_int as isize)
            + *bf0.offset(13 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(3 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(3 as core::ffi::c_int as isize)
            + *bf0.offset(12 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(4 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(4 as core::ffi::c_int as isize)
            + *bf0.offset(11 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(5 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(5 as core::ffi::c_int as isize)
            + *bf0.offset(10 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(6 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(6 as core::ffi::c_int as isize)
            + *bf0.offset(9 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(7 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(7 as core::ffi::c_int as isize)
            + *bf0.offset(8 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(8 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(7 as core::ffi::c_int as isize)
            - *bf0.offset(8 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(9 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(6 as core::ffi::c_int as isize)
            - *bf0.offset(9 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(10 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(5 as core::ffi::c_int as isize)
            - *bf0.offset(10 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(11 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(4 as core::ffi::c_int as isize)
            - *bf0.offset(11 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(12 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(3 as core::ffi::c_int as isize)
            - *bf0.offset(12 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(13 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(2 as core::ffi::c_int as isize)
            - *bf0.offset(13 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(14 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(1 as core::ffi::c_int as isize)
            - *bf0.offset(14 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(15 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(0 as core::ffi::c_int as isize)
            - *bf0.offset(15 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(16 as core::ffi::c_int as isize) = *bf0
        .offset(16 as core::ffi::c_int as isize);
    *bf1.offset(17 as core::ffi::c_int as isize) = *bf0
        .offset(17 as core::ffi::c_int as isize);
    *bf1.offset(18 as core::ffi::c_int as isize) = *bf0
        .offset(18 as core::ffi::c_int as isize);
    *bf1.offset(19 as core::ffi::c_int as isize) = *bf0
        .offset(19 as core::ffi::c_int as isize);
    *bf1.offset(20 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(20 as core::ffi::c_int as isize),
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(27 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(21 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(21 as core::ffi::c_int as isize),
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(26 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(22 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(22 as core::ffi::c_int as isize),
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(25 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(23 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(23 as core::ffi::c_int as isize),
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(24 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(24 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(23 as core::ffi::c_int as isize),
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(24 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(25 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(22 as core::ffi::c_int as isize),
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(25 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(26 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(21 as core::ffi::c_int as isize),
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(26 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(27 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(20 as core::ffi::c_int as isize),
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(27 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(28 as core::ffi::c_int as isize) = *bf0
        .offset(28 as core::ffi::c_int as isize);
    *bf1.offset(29 as core::ffi::c_int as isize) = *bf0
        .offset(29 as core::ffi::c_int as isize);
    *bf1.offset(30 as core::ffi::c_int as isize) = *bf0
        .offset(30 as core::ffi::c_int as isize);
    *bf1.offset(31 as core::ffi::c_int as isize) = *bf0
        .offset(31 as core::ffi::c_int as isize);
    *bf1.offset(32 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(32 as core::ffi::c_int as isize)
            + *bf0.offset(47 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(33 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(33 as core::ffi::c_int as isize)
            + *bf0.offset(46 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(34 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(34 as core::ffi::c_int as isize)
            + *bf0.offset(45 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(35 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(35 as core::ffi::c_int as isize)
            + *bf0.offset(44 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(36 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(36 as core::ffi::c_int as isize)
            + *bf0.offset(43 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(37 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(37 as core::ffi::c_int as isize)
            + *bf0.offset(42 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(38 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(38 as core::ffi::c_int as isize)
            + *bf0.offset(41 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(39 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(39 as core::ffi::c_int as isize)
            + *bf0.offset(40 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(40 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(39 as core::ffi::c_int as isize)
            - *bf0.offset(40 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(41 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(38 as core::ffi::c_int as isize)
            - *bf0.offset(41 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(42 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(37 as core::ffi::c_int as isize)
            - *bf0.offset(42 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(43 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(36 as core::ffi::c_int as isize)
            - *bf0.offset(43 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(44 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(35 as core::ffi::c_int as isize)
            - *bf0.offset(44 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(45 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(34 as core::ffi::c_int as isize)
            - *bf0.offset(45 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(46 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(33 as core::ffi::c_int as isize)
            - *bf0.offset(46 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(47 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(32 as core::ffi::c_int as isize)
            - *bf0.offset(47 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(48 as core::ffi::c_int as isize) = clamp_value(
        -*bf0.offset(48 as core::ffi::c_int as isize)
            + *bf0.offset(63 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(49 as core::ffi::c_int as isize) = clamp_value(
        -*bf0.offset(49 as core::ffi::c_int as isize)
            + *bf0.offset(62 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(50 as core::ffi::c_int as isize) = clamp_value(
        -*bf0.offset(50 as core::ffi::c_int as isize)
            + *bf0.offset(61 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(51 as core::ffi::c_int as isize) = clamp_value(
        -*bf0.offset(51 as core::ffi::c_int as isize)
            + *bf0.offset(60 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(52 as core::ffi::c_int as isize) = clamp_value(
        -*bf0.offset(52 as core::ffi::c_int as isize)
            + *bf0.offset(59 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(53 as core::ffi::c_int as isize) = clamp_value(
        -*bf0.offset(53 as core::ffi::c_int as isize)
            + *bf0.offset(58 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(54 as core::ffi::c_int as isize) = clamp_value(
        -*bf0.offset(54 as core::ffi::c_int as isize)
            + *bf0.offset(57 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(55 as core::ffi::c_int as isize) = clamp_value(
        -*bf0.offset(55 as core::ffi::c_int as isize)
            + *bf0.offset(56 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(56 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(55 as core::ffi::c_int as isize)
            + *bf0.offset(56 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(57 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(54 as core::ffi::c_int as isize)
            + *bf0.offset(57 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(58 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(53 as core::ffi::c_int as isize)
            + *bf0.offset(58 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(59 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(52 as core::ffi::c_int as isize)
            + *bf0.offset(59 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(60 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(51 as core::ffi::c_int as isize)
            + *bf0.offset(60 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(61 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(50 as core::ffi::c_int as isize)
            + *bf0.offset(61 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(62 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(49 as core::ffi::c_int as isize)
            + *bf0.offset(62 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(63 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(48 as core::ffi::c_int as isize)
            + *bf0.offset(63 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    av1_range_check_buf(stage, input, bf1, size, *stage_range.offset(stage as isize));
    stage += 1;
    bf0 = output;
    bf1 = step.as_mut_ptr();
    *bf1.offset(0 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(0 as core::ffi::c_int as isize)
            + *bf0.offset(31 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(1 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(1 as core::ffi::c_int as isize)
            + *bf0.offset(30 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(2 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(2 as core::ffi::c_int as isize)
            + *bf0.offset(29 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(3 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(3 as core::ffi::c_int as isize)
            + *bf0.offset(28 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(4 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(4 as core::ffi::c_int as isize)
            + *bf0.offset(27 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(5 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(5 as core::ffi::c_int as isize)
            + *bf0.offset(26 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(6 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(6 as core::ffi::c_int as isize)
            + *bf0.offset(25 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(7 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(7 as core::ffi::c_int as isize)
            + *bf0.offset(24 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(8 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(8 as core::ffi::c_int as isize)
            + *bf0.offset(23 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(9 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(9 as core::ffi::c_int as isize)
            + *bf0.offset(22 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(10 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(10 as core::ffi::c_int as isize)
            + *bf0.offset(21 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(11 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(11 as core::ffi::c_int as isize)
            + *bf0.offset(20 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(12 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(12 as core::ffi::c_int as isize)
            + *bf0.offset(19 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(13 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(13 as core::ffi::c_int as isize)
            + *bf0.offset(18 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(14 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(14 as core::ffi::c_int as isize)
            + *bf0.offset(17 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(15 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(15 as core::ffi::c_int as isize)
            + *bf0.offset(16 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(16 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(15 as core::ffi::c_int as isize)
            - *bf0.offset(16 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(17 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(14 as core::ffi::c_int as isize)
            - *bf0.offset(17 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(18 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(13 as core::ffi::c_int as isize)
            - *bf0.offset(18 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(19 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(12 as core::ffi::c_int as isize)
            - *bf0.offset(19 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(20 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(11 as core::ffi::c_int as isize)
            - *bf0.offset(20 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(21 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(10 as core::ffi::c_int as isize)
            - *bf0.offset(21 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(22 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(9 as core::ffi::c_int as isize)
            - *bf0.offset(22 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(23 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(8 as core::ffi::c_int as isize)
            - *bf0.offset(23 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(24 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(7 as core::ffi::c_int as isize)
            - *bf0.offset(24 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(25 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(6 as core::ffi::c_int as isize)
            - *bf0.offset(25 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(26 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(5 as core::ffi::c_int as isize)
            - *bf0.offset(26 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(27 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(4 as core::ffi::c_int as isize)
            - *bf0.offset(27 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(28 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(3 as core::ffi::c_int as isize)
            - *bf0.offset(28 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(29 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(2 as core::ffi::c_int as isize)
            - *bf0.offset(29 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(30 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(1 as core::ffi::c_int as isize)
            - *bf0.offset(30 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(31 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(0 as core::ffi::c_int as isize)
            - *bf0.offset(31 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(32 as core::ffi::c_int as isize) = *bf0
        .offset(32 as core::ffi::c_int as isize);
    *bf1.offset(33 as core::ffi::c_int as isize) = *bf0
        .offset(33 as core::ffi::c_int as isize);
    *bf1.offset(34 as core::ffi::c_int as isize) = *bf0
        .offset(34 as core::ffi::c_int as isize);
    *bf1.offset(35 as core::ffi::c_int as isize) = *bf0
        .offset(35 as core::ffi::c_int as isize);
    *bf1.offset(36 as core::ffi::c_int as isize) = *bf0
        .offset(36 as core::ffi::c_int as isize);
    *bf1.offset(37 as core::ffi::c_int as isize) = *bf0
        .offset(37 as core::ffi::c_int as isize);
    *bf1.offset(38 as core::ffi::c_int as isize) = *bf0
        .offset(38 as core::ffi::c_int as isize);
    *bf1.offset(39 as core::ffi::c_int as isize) = *bf0
        .offset(39 as core::ffi::c_int as isize);
    *bf1.offset(40 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(40 as core::ffi::c_int as isize),
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(55 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(41 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(41 as core::ffi::c_int as isize),
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(54 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(42 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(42 as core::ffi::c_int as isize),
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(53 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(43 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(43 as core::ffi::c_int as isize),
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(52 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(44 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(44 as core::ffi::c_int as isize),
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(51 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(45 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(45 as core::ffi::c_int as isize),
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(50 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(46 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(46 as core::ffi::c_int as isize),
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(49 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(47 as core::ffi::c_int as isize) = half_btf(
        -*cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(47 as core::ffi::c_int as isize),
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(48 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(48 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(47 as core::ffi::c_int as isize),
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(48 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(49 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(46 as core::ffi::c_int as isize),
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(49 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(50 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(45 as core::ffi::c_int as isize),
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(50 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(51 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(44 as core::ffi::c_int as isize),
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(51 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(52 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(43 as core::ffi::c_int as isize),
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(52 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(53 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(42 as core::ffi::c_int as isize),
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(53 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(54 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(41 as core::ffi::c_int as isize),
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(54 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(55 as core::ffi::c_int as isize) = half_btf(
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(40 as core::ffi::c_int as isize),
        *cospi.offset(32 as core::ffi::c_int as isize),
        *bf0.offset(55 as core::ffi::c_int as isize),
        cos_bit as core::ffi::c_int,
    );
    *bf1.offset(56 as core::ffi::c_int as isize) = *bf0
        .offset(56 as core::ffi::c_int as isize);
    *bf1.offset(57 as core::ffi::c_int as isize) = *bf0
        .offset(57 as core::ffi::c_int as isize);
    *bf1.offset(58 as core::ffi::c_int as isize) = *bf0
        .offset(58 as core::ffi::c_int as isize);
    *bf1.offset(59 as core::ffi::c_int as isize) = *bf0
        .offset(59 as core::ffi::c_int as isize);
    *bf1.offset(60 as core::ffi::c_int as isize) = *bf0
        .offset(60 as core::ffi::c_int as isize);
    *bf1.offset(61 as core::ffi::c_int as isize) = *bf0
        .offset(61 as core::ffi::c_int as isize);
    *bf1.offset(62 as core::ffi::c_int as isize) = *bf0
        .offset(62 as core::ffi::c_int as isize);
    *bf1.offset(63 as core::ffi::c_int as isize) = *bf0
        .offset(63 as core::ffi::c_int as isize);
    av1_range_check_buf(stage, input, bf1, size, *stage_range.offset(stage as isize));
    stage += 1;
    bf0 = step.as_mut_ptr();
    bf1 = output;
    *bf1.offset(0 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(0 as core::ffi::c_int as isize)
            + *bf0.offset(63 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(1 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(1 as core::ffi::c_int as isize)
            + *bf0.offset(62 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(2 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(2 as core::ffi::c_int as isize)
            + *bf0.offset(61 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(3 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(3 as core::ffi::c_int as isize)
            + *bf0.offset(60 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(4 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(4 as core::ffi::c_int as isize)
            + *bf0.offset(59 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(5 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(5 as core::ffi::c_int as isize)
            + *bf0.offset(58 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(6 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(6 as core::ffi::c_int as isize)
            + *bf0.offset(57 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(7 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(7 as core::ffi::c_int as isize)
            + *bf0.offset(56 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(8 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(8 as core::ffi::c_int as isize)
            + *bf0.offset(55 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(9 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(9 as core::ffi::c_int as isize)
            + *bf0.offset(54 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(10 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(10 as core::ffi::c_int as isize)
            + *bf0.offset(53 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(11 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(11 as core::ffi::c_int as isize)
            + *bf0.offset(52 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(12 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(12 as core::ffi::c_int as isize)
            + *bf0.offset(51 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(13 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(13 as core::ffi::c_int as isize)
            + *bf0.offset(50 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(14 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(14 as core::ffi::c_int as isize)
            + *bf0.offset(49 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(15 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(15 as core::ffi::c_int as isize)
            + *bf0.offset(48 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(16 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(16 as core::ffi::c_int as isize)
            + *bf0.offset(47 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(17 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(17 as core::ffi::c_int as isize)
            + *bf0.offset(46 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(18 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(18 as core::ffi::c_int as isize)
            + *bf0.offset(45 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(19 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(19 as core::ffi::c_int as isize)
            + *bf0.offset(44 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(20 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(20 as core::ffi::c_int as isize)
            + *bf0.offset(43 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(21 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(21 as core::ffi::c_int as isize)
            + *bf0.offset(42 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(22 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(22 as core::ffi::c_int as isize)
            + *bf0.offset(41 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(23 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(23 as core::ffi::c_int as isize)
            + *bf0.offset(40 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(24 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(24 as core::ffi::c_int as isize)
            + *bf0.offset(39 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(25 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(25 as core::ffi::c_int as isize)
            + *bf0.offset(38 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(26 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(26 as core::ffi::c_int as isize)
            + *bf0.offset(37 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(27 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(27 as core::ffi::c_int as isize)
            + *bf0.offset(36 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(28 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(28 as core::ffi::c_int as isize)
            + *bf0.offset(35 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(29 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(29 as core::ffi::c_int as isize)
            + *bf0.offset(34 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(30 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(30 as core::ffi::c_int as isize)
            + *bf0.offset(33 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(31 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(31 as core::ffi::c_int as isize)
            + *bf0.offset(32 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(32 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(31 as core::ffi::c_int as isize)
            - *bf0.offset(32 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(33 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(30 as core::ffi::c_int as isize)
            - *bf0.offset(33 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(34 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(29 as core::ffi::c_int as isize)
            - *bf0.offset(34 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(35 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(28 as core::ffi::c_int as isize)
            - *bf0.offset(35 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(36 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(27 as core::ffi::c_int as isize)
            - *bf0.offset(36 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(37 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(26 as core::ffi::c_int as isize)
            - *bf0.offset(37 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(38 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(25 as core::ffi::c_int as isize)
            - *bf0.offset(38 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(39 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(24 as core::ffi::c_int as isize)
            - *bf0.offset(39 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(40 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(23 as core::ffi::c_int as isize)
            - *bf0.offset(40 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(41 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(22 as core::ffi::c_int as isize)
            - *bf0.offset(41 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(42 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(21 as core::ffi::c_int as isize)
            - *bf0.offset(42 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(43 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(20 as core::ffi::c_int as isize)
            - *bf0.offset(43 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(44 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(19 as core::ffi::c_int as isize)
            - *bf0.offset(44 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(45 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(18 as core::ffi::c_int as isize)
            - *bf0.offset(45 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(46 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(17 as core::ffi::c_int as isize)
            - *bf0.offset(46 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(47 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(16 as core::ffi::c_int as isize)
            - *bf0.offset(47 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(48 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(15 as core::ffi::c_int as isize)
            - *bf0.offset(48 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(49 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(14 as core::ffi::c_int as isize)
            - *bf0.offset(49 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(50 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(13 as core::ffi::c_int as isize)
            - *bf0.offset(50 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(51 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(12 as core::ffi::c_int as isize)
            - *bf0.offset(51 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(52 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(11 as core::ffi::c_int as isize)
            - *bf0.offset(52 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(53 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(10 as core::ffi::c_int as isize)
            - *bf0.offset(53 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(54 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(9 as core::ffi::c_int as isize)
            - *bf0.offset(54 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(55 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(8 as core::ffi::c_int as isize)
            - *bf0.offset(55 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(56 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(7 as core::ffi::c_int as isize)
            - *bf0.offset(56 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(57 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(6 as core::ffi::c_int as isize)
            - *bf0.offset(57 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(58 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(5 as core::ffi::c_int as isize)
            - *bf0.offset(58 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(59 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(4 as core::ffi::c_int as isize)
            - *bf0.offset(59 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(60 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(3 as core::ffi::c_int as isize)
            - *bf0.offset(60 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(61 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(2 as core::ffi::c_int as isize)
            - *bf0.offset(61 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(62 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(1 as core::ffi::c_int as isize)
            - *bf0.offset(62 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
    *bf1.offset(63 as core::ffi::c_int as isize) = clamp_value(
        *bf0.offset(0 as core::ffi::c_int as isize)
            - *bf0.offset(63 as core::ffi::c_int as isize),
        *stage_range.offset(stage as isize),
    );
}

extern "C" {
    fn __assert_fail(
        __assertion: *const core::ffi::c_char,
        __file: *const core::ffi::c_char,
        __line: core::ffi::c_uint,
        __function: *const core::ffi::c_char,
    ) -> !;
}
