/*
 *  Copyright 2011 The LibYuv Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS. All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

// Mechanically translated from upstream/libavif/third_party/libyuv/source/row_common.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C" {
    fn __assert_fail(
        __assertion: *const core::ffi::c_char,
        __file: *const core::ffi::c_char,
        __line: core::ffi::c_uint,
        __function: *const core::ffi::c_char,
    ) -> !;
    fn memcpy(
        __dest: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
}
pub type ptrdiff_t = isize;
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
#[no_mangle]
pub unsafe extern "C" fn CopyRow_C(
    mut src: *const uint8_t,
    mut dst: *mut uint8_t,
    mut count: core::ffi::c_int,
) {
    memcpy(
        dst as *mut core::ffi::c_void,
        src as *const core::ffi::c_void,
        count as size_t,
    );
}
unsafe extern "C" fn HalfRow_C(
    mut src_uv: *const uint8_t,
    mut src_uv_stride: ptrdiff_t,
    mut dst_uv: *mut uint8_t,
    mut width: core::ffi::c_int,
) {
    let mut x: core::ffi::c_int = 0;
    x = 0 as core::ffi::c_int;
    while x < width {
        *dst_uv.offset(x as isize) = (*src_uv.offset(x as isize) as core::ffi::c_int
            + *src_uv.offset((src_uv_stride + x as ptrdiff_t) as isize)
                as core::ffi::c_int + 1 as core::ffi::c_int >> 1 as core::ffi::c_int)
            as uint8_t;
        x += 1;
    }
}
unsafe extern "C" fn HalfRow_16_C(
    mut src_uv: *const uint16_t,
    mut src_uv_stride: ptrdiff_t,
    mut dst_uv: *mut uint16_t,
    mut width: core::ffi::c_int,
) {
    let mut x: core::ffi::c_int = 0;
    x = 0 as core::ffi::c_int;
    while x < width {
        *dst_uv.offset(x as isize) = (*src_uv.offset(x as isize) as core::ffi::c_int
            + *src_uv.offset((src_uv_stride + x as ptrdiff_t) as isize)
                as core::ffi::c_int + 1 as core::ffi::c_int >> 1 as core::ffi::c_int)
            as uint16_t;
        x += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn InterpolateRow_C(
    mut dst_ptr: *mut uint8_t,
    mut src_ptr: *const uint8_t,
    mut src_stride: ptrdiff_t,
    mut width: core::ffi::c_int,
    mut source_y_fraction: core::ffi::c_int,
) {
    let mut y1_fraction: core::ffi::c_int = source_y_fraction;
    let mut y0_fraction: core::ffi::c_int = 256 as core::ffi::c_int - y1_fraction;
    let mut src_ptr1: *const uint8_t = src_ptr.offset(src_stride as isize);
    let mut x: core::ffi::c_int = 0;
    'c_1507: {
        if source_y_fraction >= 0 as core::ffi::c_int {} else {
            __assert_fail(
                b"source_y_fraction >= 0\0" as *const u8 as *const core::ffi::c_char,
                b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/third_party/libyuv/source/row_common.c\0"
                    as *const u8 as *const core::ffi::c_char,
                55 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 71],
                    [core::ffi::c_char; 71],
                >(
                    *b"void InterpolateRow_C(uint8_t *, const uint8_t *, ptrdiff_t, int, int)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    'c_1459: {
        if source_y_fraction < 256 as core::ffi::c_int {} else {
            __assert_fail(
                b"source_y_fraction < 256\0" as *const u8 as *const core::ffi::c_char,
                b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/third_party/libyuv/source/row_common.c\0"
                    as *const u8 as *const core::ffi::c_char,
                56 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 71],
                    [core::ffi::c_char; 71],
                >(
                    *b"void InterpolateRow_C(uint8_t *, const uint8_t *, ptrdiff_t, int, int)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if y1_fraction == 0 as core::ffi::c_int {
        memcpy(
            dst_ptr as *mut core::ffi::c_void,
            src_ptr as *const core::ffi::c_void,
            width as size_t,
        );
        return;
    }
    if y1_fraction == 128 as core::ffi::c_int {
        HalfRow_C(src_ptr, src_stride, dst_ptr, width);
        return;
    }
    x = 0 as core::ffi::c_int;
    while x < width {
        *dst_ptr.offset(0 as core::ffi::c_int as isize) = (*src_ptr
            .offset(0 as core::ffi::c_int as isize) as core::ffi::c_int * y0_fraction
            + *src_ptr1.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                * y1_fraction + 128 as core::ffi::c_int >> 8 as core::ffi::c_int)
            as uint8_t;
        src_ptr = src_ptr.offset(1);
        src_ptr1 = src_ptr1.offset(1);
        dst_ptr = dst_ptr.offset(1);
        x += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn InterpolateRow_16_C(
    mut dst_ptr: *mut uint16_t,
    mut src_ptr: *const uint16_t,
    mut src_stride: ptrdiff_t,
    mut width: core::ffi::c_int,
    mut source_y_fraction: core::ffi::c_int,
) {
    let mut y1_fraction: core::ffi::c_int = source_y_fraction;
    let mut y0_fraction: core::ffi::c_int = 256 as core::ffi::c_int - y1_fraction;
    let mut src_ptr1: *const uint16_t = src_ptr.offset(src_stride as isize);
    let mut x: core::ffi::c_int = 0;
    'c_1766: {
        if source_y_fraction >= 0 as core::ffi::c_int {} else {
            __assert_fail(
                b"source_y_fraction >= 0\0" as *const u8 as *const core::ffi::c_char,
                b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/third_party/libyuv/source/row_common.c\0"
                    as *const u8 as *const core::ffi::c_char,
                86 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 76],
                    [core::ffi::c_char; 76],
                >(
                    *b"void InterpolateRow_16_C(uint16_t *, const uint16_t *, ptrdiff_t, int, int)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    'c_1729: {
        if source_y_fraction < 256 as core::ffi::c_int {} else {
            __assert_fail(
                b"source_y_fraction < 256\0" as *const u8 as *const core::ffi::c_char,
                b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/third_party/libyuv/source/row_common.c\0"
                    as *const u8 as *const core::ffi::c_char,
                87 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 76],
                    [core::ffi::c_char; 76],
                >(
                    *b"void InterpolateRow_16_C(uint16_t *, const uint16_t *, ptrdiff_t, int, int)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if y1_fraction == 0 as core::ffi::c_int {
        memcpy(
            dst_ptr as *mut core::ffi::c_void,
            src_ptr as *const core::ffi::c_void,
            (width * 2 as core::ffi::c_int) as size_t,
        );
        return;
    }
    if y1_fraction == 128 as core::ffi::c_int {
        HalfRow_16_C(src_ptr, src_stride, dst_ptr, width);
        return;
    }
    x = 0 as core::ffi::c_int;
    while x < width {
        *dst_ptr.offset(0 as core::ffi::c_int as isize) = (*src_ptr
            .offset(0 as core::ffi::c_int as isize) as core::ffi::c_int * y0_fraction
            + *src_ptr1.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                * y1_fraction + 128 as core::ffi::c_int >> 8 as core::ffi::c_int)
            as uint16_t;
        src_ptr = src_ptr.offset(1);
        src_ptr1 = src_ptr1.offset(1);
        dst_ptr = dst_ptr.offset(1);
        x += 1;
    }
}
