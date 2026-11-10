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

/*
 * Sum the square of the difference between every corresponding element of the
 * buffers.
 */

// Mechanically translated from upstream/aom/aom_dsp/sse.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C-unwind" {
}
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __int64_t = i64;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uintptr_t = usize;
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sse_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
) -> int64_t {
    let mut y: core::ffi::c_int = 0;
    let mut x: core::ffi::c_int = 0;
    let mut sse: int64_t = 0 as int64_t;
    y = 0 as core::ffi::c_int;
    while y < height {
        x = 0 as core::ffi::c_int;
        while x < width {
            let diff: int32_t = abs(
                *a.offset(x as isize) as core::ffi::c_int
                    - *b.offset(x as isize) as core::ffi::c_int,
            ) as int32_t;
            sse += (diff * diff) as int64_t;
            x += 1;
        }
        a = a.offset(a_stride as isize);
        b = b.offset(b_stride as isize);
        y += 1;
    }
    return sse;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sse_c(
    mut a8: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b8: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
) -> int64_t {
    let mut y: core::ffi::c_int = 0;
    let mut x: core::ffi::c_int = 0;
    let mut sse: int64_t = 0 as int64_t;
    let mut a: *mut uint16_t = ((a8 as uintptr_t) << 1 as core::ffi::c_int)
        as *mut uint16_t;
    let mut b: *mut uint16_t = ((b8 as uintptr_t) << 1 as core::ffi::c_int)
        as *mut uint16_t;
    y = 0 as core::ffi::c_int;
    while y < height {
        x = 0 as core::ffi::c_int;
        while x < width {
            let diff: int32_t = *a.offset(x as isize) as int32_t
                - *b.offset(x as isize) as int32_t;
            sse += (diff * diff) as int64_t;
            x += 1;
        }
        a = a.offset(a_stride as isize);
        b = b.offset(b_stride as isize);
        y += 1;
    }
    return sse;
}

extern "C" {
    fn abs(__x: core::ffi::c_int) -> core::ffi::c_int;
}
