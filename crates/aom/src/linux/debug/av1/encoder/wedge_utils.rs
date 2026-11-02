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

// Mechanically translated from upstream/aom/av1/encoder/wedge_utils.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
pub type __int8_t = i8;
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __int32_t = i32;
pub type __int64_t = i64;
pub type __uint64_t = u64;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint64_t = __uint64_t;
pub const INT16_MIN: core::ffi::c_int = -(32767 as core::ffi::c_int)
    - 1 as core::ffi::c_int;
pub const INT16_MAX: core::ffi::c_int = 32767 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn clamp(
    mut value: core::ffi::c_int,
    mut low: core::ffi::c_int,
    mut high: core::ffi::c_int,
) -> core::ffi::c_int {
    return if value < low { low } else if value > high { high } else { value };
}
pub const MAX_MASK_VALUE: core::ffi::c_int = (1 as core::ffi::c_int)
    << WEDGE_WEIGHT_BITS;
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_wedge_sse_from_residuals_c(
    mut r1: *const int16_t,
    mut d: *const int16_t,
    mut m: *const uint8_t,
    mut N: core::ffi::c_int,
) -> uint64_t {
    let mut csse: uint64_t = 0 as uint64_t;
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < N {
        let mut t: int32_t = MAX_MASK_VALUE as int32_t
            * *r1.offset(i as isize) as int32_t
            + *m.offset(i as isize) as int32_t * *d.offset(i as isize) as int32_t;
        t = clamp(t as core::ffi::c_int, INT16_MIN, INT16_MAX) as int32_t;
        csse = csse.wrapping_add((t * t) as uint64_t);
        i += 1;
    }
    return csse
        .wrapping_add(
            ((1 as core::ffi::c_int) << 2 as core::ffi::c_int * 6 as core::ffi::c_int
                >> 1 as core::ffi::c_int) as uint64_t,
        ) >> 2 as core::ffi::c_int * 6 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_wedge_sign_from_residuals_c(
    mut ds: *const int16_t,
    mut m: *const uint8_t,
    mut N: core::ffi::c_int,
    mut limit: int64_t,
) -> int8_t {
    let mut acc: int64_t = 0 as int64_t;
    loop {
        let fresh0 = ds;
        ds = ds.offset(1);
        let fresh1 = m;
        m = m.offset(1);
        acc += (*fresh0 as core::ffi::c_int * *fresh1 as core::ffi::c_int) as int64_t;
        N -= 1;
        if !(N != 0) {
            break;
        }
    }
    return (acc > limit) as core::ffi::c_int as int8_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_wedge_compute_delta_squares_c(
    mut d: *mut int16_t,
    mut a: *const int16_t,
    mut b: *const int16_t,
    mut N: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < N {
        *d.offset(i as isize) = clamp(
            *a.offset(i as isize) as core::ffi::c_int
                * *a.offset(i as isize) as core::ffi::c_int
                - *b.offset(i as isize) as core::ffi::c_int
                    * *b.offset(i as isize) as core::ffi::c_int,
            INT16_MIN,
            INT16_MAX,
        ) as int16_t;
        i += 1;
    }
}
pub const WEDGE_WEIGHT_BITS: core::ffi::c_int = 6 as core::ffi::c_int;
