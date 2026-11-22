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

// Mechanically translated from upstream/aom/aom_dsp/sum_squares.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
pub type int16_t = i16;
pub type int64_t = i64;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type uintptr_t = usize;
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sum_squares_2d_i16_c(
    mut src: *const int16_t,
    mut src_stride: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
) -> uint64_t {
    let mut r: core::ffi::c_int = 0;
    let mut c: core::ffi::c_int = 0;
    let mut ss: uint64_t = 0 as uint64_t;
    r = 0 as core::ffi::c_int;
    while r < height {
        c = 0 as core::ffi::c_int;
        while c < width {
            let v: int16_t = *src.offset(c as isize);
            ss = ss
                .wrapping_add(
                    (v as core::ffi::c_int * v as core::ffi::c_int) as uint64_t,
                );
            c += 1;
        }
        src = src.offset(src_stride as isize);
        r += 1;
    }
    return ss;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sum_squares_i16_c(
    mut src: *const int16_t,
    mut n: uint32_t,
) -> uint64_t {
    let mut ss: uint64_t = 0 as uint64_t;
    loop {
        let fresh0 = src;
        src = src.offset(1);
        let v: int16_t = *fresh0;
        ss = ss
            .wrapping_add((v as core::ffi::c_int * v as core::ffi::c_int) as uint64_t);
        n = n.wrapping_sub(1);
        if !(n != 0) {
            break;
        }
    }
    return ss;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_var_2d_u8_c(
    mut src: *mut uint8_t,
    mut src_stride: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
) -> uint64_t {
    let mut r: core::ffi::c_int = 0;
    let mut c: core::ffi::c_int = 0;
    let mut ss: uint64_t = 0 as uint64_t;
    let mut s: uint64_t = 0 as uint64_t;
    r = 0 as core::ffi::c_int;
    while r < height {
        c = 0 as core::ffi::c_int;
        while c < width {
            let v: uint8_t = *src.offset(c as isize);
            ss = ss
                .wrapping_add(
                    (v as core::ffi::c_int * v as core::ffi::c_int) as uint64_t,
                );
            s = s.wrapping_add(v as uint64_t);
            c += 1;
        }
        src = src.offset(src_stride as isize);
        r += 1;
    }
    return ss.wrapping_sub(s.wrapping_mul(s).wrapping_div((width * height) as uint64_t));
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_var_2d_u16_c(
    mut src: *mut uint8_t,
    mut src_stride: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
) -> uint64_t {
    let mut srcp: *mut uint16_t = ((src as uintptr_t) << 1 as core::ffi::c_int)
        as *mut uint16_t;
    let mut r: core::ffi::c_int = 0;
    let mut c: core::ffi::c_int = 0;
    let mut ss: uint64_t = 0 as uint64_t;
    let mut s: uint64_t = 0 as uint64_t;
    r = 0 as core::ffi::c_int;
    while r < height {
        c = 0 as core::ffi::c_int;
        while c < width {
            let v: uint16_t = *srcp.offset(c as isize);
            ss = ss
                .wrapping_add(
                    (v as core::ffi::c_int * v as core::ffi::c_int) as uint64_t,
                );
            s = s.wrapping_add(v as uint64_t);
            c += 1;
        }
        srcp = srcp.offset(src_stride as isize);
        r += 1;
    }
    return ss.wrapping_sub(s.wrapping_mul(s).wrapping_div((width * height) as uint64_t));
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sum_sse_2d_i16_c(
    mut src: *const int16_t,
    mut src_stride: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut sum: *mut core::ffi::c_int,
) -> uint64_t {
    let mut r: core::ffi::c_int = 0;
    let mut c: core::ffi::c_int = 0;
    let mut srcp: *mut int16_t = src as *mut int16_t;
    let mut ss: int64_t = 0 as int64_t;
    r = 0 as core::ffi::c_int;
    while r < height {
        c = 0 as core::ffi::c_int;
        while c < width {
            let v: int16_t = *srcp.offset(c as isize);
            ss += (v as core::ffi::c_int * v as core::ffi::c_int) as int64_t;
            *sum += v as core::ffi::c_int;
            c += 1;
        }
        srcp = srcp.offset(src_stride as isize);
        r += 1;
    }
    return ss as uint64_t;
}
