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

// Mechanically translated from upstream/aom/aom_dsp/fwd_txfm.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C-unwind" {
}
pub type __int16_t = i16;
pub type __int32_t = i32;
pub type __int64_t = i64;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type tran_high_t = int64_t;
pub type tran_low_t = int32_t;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
static mut cospi_8_64: tran_high_t = 15137 as tran_high_t;
static mut cospi_16_64: tran_high_t = 11585 as tran_high_t;
static mut cospi_24_64: tran_high_t = 6270 as tran_high_t;
#[inline]
unsafe extern "C-unwind" fn fdct_round_shift(mut input: tran_high_t) -> tran_high_t {
    let mut rv: tran_high_t = input
        + ((1 as core::ffi::c_int) << 14 as core::ffi::c_int >> 1 as core::ffi::c_int)
            as tran_high_t >> 14 as core::ffi::c_int;
    return rv;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_fdct4x4_c(
    mut input: *const int16_t,
    mut output: *mut tran_low_t,
    mut stride: core::ffi::c_int,
) {
    let mut intermediate: [tran_low_t; 16] = [0; 16];
    let mut in_low: *const tran_low_t = 0 as *const tran_low_t;
    let mut out: *mut tran_low_t = intermediate.as_mut_ptr();
    let mut pass: core::ffi::c_int = 0 as core::ffi::c_int;
    while pass < 2 as core::ffi::c_int {
        let mut in_high: [tran_high_t; 4] = [0; 4];
        let mut step: [tran_high_t; 4] = [0; 4];
        let mut temp: [tran_low_t; 4] = [0; 4];
        let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
        while i < 4 as core::ffi::c_int {
            if pass == 0 as core::ffi::c_int {
                in_high[0 as core::ffi::c_int as usize] = (*input
                    .offset((0 as core::ffi::c_int * stride) as isize)
                    as core::ffi::c_int * 16 as core::ffi::c_int) as tran_high_t;
                in_high[1 as core::ffi::c_int as usize] = (*input
                    .offset((1 as core::ffi::c_int * stride) as isize)
                    as core::ffi::c_int * 16 as core::ffi::c_int) as tran_high_t;
                in_high[2 as core::ffi::c_int as usize] = (*input
                    .offset((2 as core::ffi::c_int * stride) as isize)
                    as core::ffi::c_int * 16 as core::ffi::c_int) as tran_high_t;
                in_high[3 as core::ffi::c_int as usize] = (*input
                    .offset((3 as core::ffi::c_int * stride) as isize)
                    as core::ffi::c_int * 16 as core::ffi::c_int) as tran_high_t;
                if i == 0 as core::ffi::c_int
                    && in_high[0 as core::ffi::c_int as usize] != 0
                {
                    in_high[0 as core::ffi::c_int as usize] += 1;
                }
                input = input.offset(1);
            } else {
                if !in_low.is_null() {} else {
                    __assert_fail(
                        b"in_low != NULL\0" as *const u8 as *const core::ffi::c_char,
                        b"upstream/aom/aom_dsp/fwd_txfm.c\0" as *const u8
                            as *const core::ffi::c_char,
                        41 as core::ffi::c_uint,
                        (::core::mem::transmute::<
                            [u8; 55],
                            [core::ffi::c_char; 55],
                        >(*b"void aom_fdct4x4_c(const int16_t *, tran_low_t *, int)\0"))
                            .as_ptr(),
                    );
                };
                in_high[0 as core::ffi::c_int as usize] = *in_low
                    .offset((0 as core::ffi::c_int * 4 as core::ffi::c_int) as isize)
                    as tran_high_t;
                in_high[1 as core::ffi::c_int as usize] = *in_low
                    .offset((1 as core::ffi::c_int * 4 as core::ffi::c_int) as isize)
                    as tran_high_t;
                in_high[2 as core::ffi::c_int as usize] = *in_low
                    .offset((2 as core::ffi::c_int * 4 as core::ffi::c_int) as isize)
                    as tran_high_t;
                in_high[3 as core::ffi::c_int as usize] = *in_low
                    .offset((3 as core::ffi::c_int * 4 as core::ffi::c_int) as isize)
                    as tran_high_t;
                in_low = in_low.offset(1);
            }
            step[0 as core::ffi::c_int as usize] = in_high[0 as core::ffi::c_int
                as usize] + in_high[3 as core::ffi::c_int as usize];
            step[1 as core::ffi::c_int as usize] = in_high[1 as core::ffi::c_int
                as usize] + in_high[2 as core::ffi::c_int as usize];
            step[2 as core::ffi::c_int as usize] = in_high[1 as core::ffi::c_int
                as usize] - in_high[2 as core::ffi::c_int as usize];
            step[3 as core::ffi::c_int as usize] = in_high[0 as core::ffi::c_int
                as usize] - in_high[3 as core::ffi::c_int as usize];
            temp[0 as core::ffi::c_int as usize] = fdct_round_shift(
                (step[0 as core::ffi::c_int as usize]
                    + step[1 as core::ffi::c_int as usize]) * cospi_16_64,
            ) as tran_low_t;
            temp[2 as core::ffi::c_int as usize] = fdct_round_shift(
                (step[0 as core::ffi::c_int as usize]
                    - step[1 as core::ffi::c_int as usize]) * cospi_16_64,
            ) as tran_low_t;
            temp[1 as core::ffi::c_int as usize] = fdct_round_shift(
                step[2 as core::ffi::c_int as usize] * cospi_24_64
                    + step[3 as core::ffi::c_int as usize] * cospi_8_64,
            ) as tran_low_t;
            temp[3 as core::ffi::c_int as usize] = fdct_round_shift(
                -step[2 as core::ffi::c_int as usize] * cospi_8_64
                    + step[3 as core::ffi::c_int as usize] * cospi_24_64,
            ) as tran_low_t;
            if pass == 0 as core::ffi::c_int {
                *out.offset(0 as core::ffi::c_int as isize) = temp[0 as core::ffi::c_int
                    as usize];
                *out.offset(1 as core::ffi::c_int as isize) = temp[1 as core::ffi::c_int
                    as usize];
                *out.offset(2 as core::ffi::c_int as isize) = temp[2 as core::ffi::c_int
                    as usize];
                *out.offset(3 as core::ffi::c_int as isize) = temp[3 as core::ffi::c_int
                    as usize];
                out = out.offset(4 as core::ffi::c_int as isize);
            } else {
                *out.offset((0 as core::ffi::c_int * 4 as core::ffi::c_int) as isize) = temp[0
                    as core::ffi::c_int as usize];
                *out.offset((1 as core::ffi::c_int * 4 as core::ffi::c_int) as isize) = temp[1
                    as core::ffi::c_int as usize];
                *out.offset((2 as core::ffi::c_int * 4 as core::ffi::c_int) as isize) = temp[2
                    as core::ffi::c_int as usize];
                *out.offset((3 as core::ffi::c_int * 4 as core::ffi::c_int) as isize) = temp[3
                    as core::ffi::c_int as usize];
                out = out.offset(1);
            }
            i += 1;
        }
        in_low = intermediate.as_mut_ptr();
        out = output;
        pass += 1;
    }
    let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while i_0 < 4 as core::ffi::c_int {
        let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
        while j < 4 as core::ffi::c_int {
            *output.offset((j + i_0 * 4 as core::ffi::c_int) as isize) = *output
                .offset((j + i_0 * 4 as core::ffi::c_int) as isize) + 1 as tran_low_t
                >> 2 as core::ffi::c_int;
            j += 1;
        }
        i_0 += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_fdct4x4_lp_c(
    mut input: *const int16_t,
    mut output: *mut int16_t,
    mut stride: core::ffi::c_int,
) {
    let mut intermediate: [int16_t; 16] = [0; 16];
    let mut in_low: *const int16_t = 0 as *const int16_t;
    let mut out: *mut int16_t = intermediate.as_mut_ptr();
    let mut pass: core::ffi::c_int = 0 as core::ffi::c_int;
    while pass < 2 as core::ffi::c_int {
        let mut in_high: [int32_t; 4] = [0; 4];
        let mut step: [int32_t; 4] = [0; 4];
        let mut temp: [int16_t; 4] = [0; 4];
        let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
        while i < 4 as core::ffi::c_int {
            if pass == 0 as core::ffi::c_int {
                in_high[0 as core::ffi::c_int as usize] = (*input
                    .offset((0 as core::ffi::c_int * stride) as isize)
                    as core::ffi::c_int * 16 as core::ffi::c_int) as int32_t;
                in_high[1 as core::ffi::c_int as usize] = (*input
                    .offset((1 as core::ffi::c_int * stride) as isize)
                    as core::ffi::c_int * 16 as core::ffi::c_int) as int32_t;
                in_high[2 as core::ffi::c_int as usize] = (*input
                    .offset((2 as core::ffi::c_int * stride) as isize)
                    as core::ffi::c_int * 16 as core::ffi::c_int) as int32_t;
                in_high[3 as core::ffi::c_int as usize] = (*input
                    .offset((3 as core::ffi::c_int * stride) as isize)
                    as core::ffi::c_int * 16 as core::ffi::c_int) as int32_t;
                input = input.offset(1);
                if i == 0 as core::ffi::c_int
                    && in_high[0 as core::ffi::c_int as usize] != 0
                {
                    in_high[0 as core::ffi::c_int as usize] += 1;
                }
            } else {
                if !in_low.is_null() {} else {
                    __assert_fail(
                        b"in_low != NULL\0" as *const u8 as *const core::ffi::c_char,
                        b"upstream/aom/aom_dsp/fwd_txfm.c\0" as *const u8
                            as *const core::ffi::c_char,
                        110 as core::ffi::c_uint,
                        (::core::mem::transmute::<
                            [u8; 55],
                            [core::ffi::c_char; 55],
                        >(*b"void aom_fdct4x4_lp_c(const int16_t *, int16_t *, int)\0"))
                            .as_ptr(),
                    );
                };
                in_high[0 as core::ffi::c_int as usize] = *in_low
                    .offset((0 as core::ffi::c_int * 4 as core::ffi::c_int) as isize)
                    as int32_t;
                in_high[1 as core::ffi::c_int as usize] = *in_low
                    .offset((1 as core::ffi::c_int * 4 as core::ffi::c_int) as isize)
                    as int32_t;
                in_high[2 as core::ffi::c_int as usize] = *in_low
                    .offset((2 as core::ffi::c_int * 4 as core::ffi::c_int) as isize)
                    as int32_t;
                in_high[3 as core::ffi::c_int as usize] = *in_low
                    .offset((3 as core::ffi::c_int * 4 as core::ffi::c_int) as isize)
                    as int32_t;
                in_low = in_low.offset(1);
            }
            step[0 as core::ffi::c_int as usize] = in_high[0 as core::ffi::c_int
                as usize] + in_high[3 as core::ffi::c_int as usize];
            step[1 as core::ffi::c_int as usize] = in_high[1 as core::ffi::c_int
                as usize] + in_high[2 as core::ffi::c_int as usize];
            step[2 as core::ffi::c_int as usize] = in_high[1 as core::ffi::c_int
                as usize] - in_high[2 as core::ffi::c_int as usize];
            step[3 as core::ffi::c_int as usize] = in_high[0 as core::ffi::c_int
                as usize] - in_high[3 as core::ffi::c_int as usize];
            temp[0 as core::ffi::c_int as usize] = fdct_round_shift(
                (step[0 as core::ffi::c_int as usize]
                    + step[1 as core::ffi::c_int as usize]) as tran_high_t * cospi_16_64,
            ) as int16_t;
            temp[2 as core::ffi::c_int as usize] = fdct_round_shift(
                (step[0 as core::ffi::c_int as usize]
                    - step[1 as core::ffi::c_int as usize]) as tran_high_t * cospi_16_64,
            ) as int16_t;
            temp[1 as core::ffi::c_int as usize] = fdct_round_shift(
                step[2 as core::ffi::c_int as usize] as tran_high_t * cospi_24_64
                    + step[3 as core::ffi::c_int as usize] as tran_high_t * cospi_8_64,
            ) as int16_t;
            temp[3 as core::ffi::c_int as usize] = fdct_round_shift(
                -step[2 as core::ffi::c_int as usize] as tran_high_t * cospi_8_64
                    + step[3 as core::ffi::c_int as usize] as tran_high_t * cospi_24_64,
            ) as int16_t;
            if pass == 0 as core::ffi::c_int {
                *out.offset(0 as core::ffi::c_int as isize) = temp[0 as core::ffi::c_int
                    as usize];
                *out.offset(1 as core::ffi::c_int as isize) = temp[1 as core::ffi::c_int
                    as usize];
                *out.offset(2 as core::ffi::c_int as isize) = temp[2 as core::ffi::c_int
                    as usize];
                *out.offset(3 as core::ffi::c_int as isize) = temp[3 as core::ffi::c_int
                    as usize];
                out = out.offset(4 as core::ffi::c_int as isize);
            } else {
                *out.offset((0 as core::ffi::c_int * 4 as core::ffi::c_int) as isize) = temp[0
                    as core::ffi::c_int as usize];
                *out.offset((1 as core::ffi::c_int * 4 as core::ffi::c_int) as isize) = temp[1
                    as core::ffi::c_int as usize];
                *out.offset((2 as core::ffi::c_int * 4 as core::ffi::c_int) as isize) = temp[2
                    as core::ffi::c_int as usize];
                *out.offset((3 as core::ffi::c_int * 4 as core::ffi::c_int) as isize) = temp[3
                    as core::ffi::c_int as usize];
                out = out.offset(1);
            }
            i += 1;
        }
        in_low = intermediate.as_mut_ptr();
        out = output;
        pass += 1;
    }
    let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while i_0 < 4 as core::ffi::c_int {
        let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
        while j < 4 as core::ffi::c_int {
            *output.offset((j + i_0 * 4 as core::ffi::c_int) as isize) = (*output
                .offset((j + i_0 * 4 as core::ffi::c_int) as isize) as core::ffi::c_int
                + 1 as core::ffi::c_int >> 2 as core::ffi::c_int) as int16_t;
            j += 1;
        }
        i_0 += 1;
    }
}

extern "C" {
    fn __assert_fail(
        __assertion: *const core::ffi::c_char,
        __file: *const core::ffi::c_char,
        __line: core::ffi::c_uint,
        __function: *const core::ffi::c_char,
    ) -> !;
}
