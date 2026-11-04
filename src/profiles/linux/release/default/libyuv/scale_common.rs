/*
 *  Copyright 2013 The LibYuv Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS. All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

// Mechanically translated from upstream/libavif/third_party/libyuv/source/scale_common.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C" {
    fn InterpolateRow_C(
        dst_ptr: *mut uint8_t,
        src_ptr: *const uint8_t,
        src_stride: ptrdiff_t,
        width: core::ffi::c_int,
        source_y_fraction: core::ffi::c_int,
    );
    fn InterpolateRow_16_C(
        dst_ptr: *mut uint16_t,
        src_ptr: *const uint16_t,
        src_stride: ptrdiff_t,
        width: core::ffi::c_int,
        source_y_fraction: core::ffi::c_int,
    );
}
pub type ptrdiff_t = isize;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type FilterMode = core::ffi::c_uint;
pub const kFilterBox: FilterMode = 3;
pub const kFilterBilinear: FilterMode = 2;
pub const kFilterLinear: FilterMode = 1;
pub const kFilterNone: FilterMode = 0;
#[inline]
unsafe extern "C" fn Abs(mut v: core::ffi::c_int) -> core::ffi::c_int {
    return if v >= 0 as core::ffi::c_int { v } else { -v };
}
#[no_mangle]
pub unsafe extern "C" fn ScaleRowUp2_Linear_C(
    mut src_ptr: *const uint8_t,
    mut dst_ptr: *mut uint8_t,
    mut dst_width: core::ffi::c_int,
) {
    let mut src_width: core::ffi::c_int = dst_width >> 1 as core::ffi::c_int;
    let mut x: core::ffi::c_int = 0;
    x = 0 as core::ffi::c_int;
    while x < src_width {
        *dst_ptr.offset((2 as core::ffi::c_int * x + 0 as core::ffi::c_int) as isize) = (*src_ptr
            .offset((x + 0 as core::ffi::c_int) as isize) as core::ffi::c_int
            * 3 as core::ffi::c_int
            + *src_ptr.offset((x + 1 as core::ffi::c_int) as isize) as core::ffi::c_int
                * 1 as core::ffi::c_int + 2 as core::ffi::c_int >> 2 as core::ffi::c_int)
            as uint8_t;
        *dst_ptr.offset((2 as core::ffi::c_int * x + 1 as core::ffi::c_int) as isize) = (*src_ptr
            .offset((x + 0 as core::ffi::c_int) as isize) as core::ffi::c_int
            * 1 as core::ffi::c_int
            + *src_ptr.offset((x + 1 as core::ffi::c_int) as isize) as core::ffi::c_int
                * 3 as core::ffi::c_int + 2 as core::ffi::c_int >> 2 as core::ffi::c_int)
            as uint8_t;
        x += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ScaleRowUp2_Bilinear_C(
    mut src_ptr: *const uint8_t,
    mut src_stride: ptrdiff_t,
    mut dst_ptr: *mut uint8_t,
    mut dst_stride: ptrdiff_t,
    mut dst_width: core::ffi::c_int,
) {
    let mut s: *const uint8_t = src_ptr;
    let mut t: *const uint8_t = src_ptr.offset(src_stride as isize);
    let mut d: *mut uint8_t = dst_ptr;
    let mut e: *mut uint8_t = dst_ptr.offset(dst_stride as isize);
    let mut src_width: core::ffi::c_int = dst_width >> 1 as core::ffi::c_int;
    let mut x: core::ffi::c_int = 0;
    x = 0 as core::ffi::c_int;
    while x < src_width {
        *d.offset((2 as core::ffi::c_int * x + 0 as core::ffi::c_int) as isize) = (*s
            .offset((x + 0 as core::ffi::c_int) as isize) as core::ffi::c_int
            * 9 as core::ffi::c_int
            + *s.offset((x + 1 as core::ffi::c_int) as isize) as core::ffi::c_int
                * 3 as core::ffi::c_int
            + *t.offset((x + 0 as core::ffi::c_int) as isize) as core::ffi::c_int
                * 3 as core::ffi::c_int
            + *t.offset((x + 1 as core::ffi::c_int) as isize) as core::ffi::c_int
                * 1 as core::ffi::c_int + 8 as core::ffi::c_int >> 4 as core::ffi::c_int)
            as uint8_t;
        *d.offset((2 as core::ffi::c_int * x + 1 as core::ffi::c_int) as isize) = (*s
            .offset((x + 0 as core::ffi::c_int) as isize) as core::ffi::c_int
            * 3 as core::ffi::c_int
            + *s.offset((x + 1 as core::ffi::c_int) as isize) as core::ffi::c_int
                * 9 as core::ffi::c_int
            + *t.offset((x + 0 as core::ffi::c_int) as isize) as core::ffi::c_int
                * 1 as core::ffi::c_int
            + *t.offset((x + 1 as core::ffi::c_int) as isize) as core::ffi::c_int
                * 3 as core::ffi::c_int + 8 as core::ffi::c_int >> 4 as core::ffi::c_int)
            as uint8_t;
        *e.offset((2 as core::ffi::c_int * x + 0 as core::ffi::c_int) as isize) = (*s
            .offset((x + 0 as core::ffi::c_int) as isize) as core::ffi::c_int
            * 3 as core::ffi::c_int
            + *s.offset((x + 1 as core::ffi::c_int) as isize) as core::ffi::c_int
                * 1 as core::ffi::c_int
            + *t.offset((x + 0 as core::ffi::c_int) as isize) as core::ffi::c_int
                * 9 as core::ffi::c_int
            + *t.offset((x + 1 as core::ffi::c_int) as isize) as core::ffi::c_int
                * 3 as core::ffi::c_int + 8 as core::ffi::c_int >> 4 as core::ffi::c_int)
            as uint8_t;
        *e.offset((2 as core::ffi::c_int * x + 1 as core::ffi::c_int) as isize) = (*s
            .offset((x + 0 as core::ffi::c_int) as isize) as core::ffi::c_int
            * 1 as core::ffi::c_int
            + *s.offset((x + 1 as core::ffi::c_int) as isize) as core::ffi::c_int
                * 3 as core::ffi::c_int
            + *t.offset((x + 0 as core::ffi::c_int) as isize) as core::ffi::c_int
                * 3 as core::ffi::c_int
            + *t.offset((x + 1 as core::ffi::c_int) as isize) as core::ffi::c_int
                * 9 as core::ffi::c_int + 8 as core::ffi::c_int >> 4 as core::ffi::c_int)
            as uint8_t;
        x += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ScaleRowUp2_Linear_16_C(
    mut src_ptr: *const uint16_t,
    mut dst_ptr: *mut uint16_t,
    mut dst_width: core::ffi::c_int,
) {
    let mut src_width: core::ffi::c_int = dst_width >> 1 as core::ffi::c_int;
    let mut x: core::ffi::c_int = 0;
    x = 0 as core::ffi::c_int;
    while x < src_width {
        *dst_ptr.offset((2 as core::ffi::c_int * x + 0 as core::ffi::c_int) as isize) = (*src_ptr
            .offset((x + 0 as core::ffi::c_int) as isize) as core::ffi::c_int
            * 3 as core::ffi::c_int
            + *src_ptr.offset((x + 1 as core::ffi::c_int) as isize) as core::ffi::c_int
                * 1 as core::ffi::c_int + 2 as core::ffi::c_int >> 2 as core::ffi::c_int)
            as uint16_t;
        *dst_ptr.offset((2 as core::ffi::c_int * x + 1 as core::ffi::c_int) as isize) = (*src_ptr
            .offset((x + 0 as core::ffi::c_int) as isize) as core::ffi::c_int
            * 1 as core::ffi::c_int
            + *src_ptr.offset((x + 1 as core::ffi::c_int) as isize) as core::ffi::c_int
                * 3 as core::ffi::c_int + 2 as core::ffi::c_int >> 2 as core::ffi::c_int)
            as uint16_t;
        x += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ScaleRowUp2_Bilinear_16_C(
    mut src_ptr: *const uint16_t,
    mut src_stride: ptrdiff_t,
    mut dst_ptr: *mut uint16_t,
    mut dst_stride: ptrdiff_t,
    mut dst_width: core::ffi::c_int,
) {
    let mut s: *const uint16_t = src_ptr;
    let mut t: *const uint16_t = src_ptr.offset(src_stride as isize);
    let mut d: *mut uint16_t = dst_ptr;
    let mut e: *mut uint16_t = dst_ptr.offset(dst_stride as isize);
    let mut src_width: core::ffi::c_int = dst_width >> 1 as core::ffi::c_int;
    let mut x: core::ffi::c_int = 0;
    x = 0 as core::ffi::c_int;
    while x < src_width {
        *d.offset((2 as core::ffi::c_int * x + 0 as core::ffi::c_int) as isize) = (*s
            .offset((x + 0 as core::ffi::c_int) as isize) as core::ffi::c_int
            * 9 as core::ffi::c_int
            + *s.offset((x + 1 as core::ffi::c_int) as isize) as core::ffi::c_int
                * 3 as core::ffi::c_int
            + *t.offset((x + 0 as core::ffi::c_int) as isize) as core::ffi::c_int
                * 3 as core::ffi::c_int
            + *t.offset((x + 1 as core::ffi::c_int) as isize) as core::ffi::c_int
                * 1 as core::ffi::c_int + 8 as core::ffi::c_int >> 4 as core::ffi::c_int)
            as uint16_t;
        *d.offset((2 as core::ffi::c_int * x + 1 as core::ffi::c_int) as isize) = (*s
            .offset((x + 0 as core::ffi::c_int) as isize) as core::ffi::c_int
            * 3 as core::ffi::c_int
            + *s.offset((x + 1 as core::ffi::c_int) as isize) as core::ffi::c_int
                * 9 as core::ffi::c_int
            + *t.offset((x + 0 as core::ffi::c_int) as isize) as core::ffi::c_int
                * 1 as core::ffi::c_int
            + *t.offset((x + 1 as core::ffi::c_int) as isize) as core::ffi::c_int
                * 3 as core::ffi::c_int + 8 as core::ffi::c_int >> 4 as core::ffi::c_int)
            as uint16_t;
        *e.offset((2 as core::ffi::c_int * x + 0 as core::ffi::c_int) as isize) = (*s
            .offset((x + 0 as core::ffi::c_int) as isize) as core::ffi::c_int
            * 3 as core::ffi::c_int
            + *s.offset((x + 1 as core::ffi::c_int) as isize) as core::ffi::c_int
                * 1 as core::ffi::c_int
            + *t.offset((x + 0 as core::ffi::c_int) as isize) as core::ffi::c_int
                * 9 as core::ffi::c_int
            + *t.offset((x + 1 as core::ffi::c_int) as isize) as core::ffi::c_int
                * 3 as core::ffi::c_int + 8 as core::ffi::c_int >> 4 as core::ffi::c_int)
            as uint16_t;
        *e.offset((2 as core::ffi::c_int * x + 1 as core::ffi::c_int) as isize) = (*s
            .offset((x + 0 as core::ffi::c_int) as isize) as core::ffi::c_int
            * 1 as core::ffi::c_int
            + *s.offset((x + 1 as core::ffi::c_int) as isize) as core::ffi::c_int
                * 3 as core::ffi::c_int
            + *t.offset((x + 0 as core::ffi::c_int) as isize) as core::ffi::c_int
                * 3 as core::ffi::c_int
            + *t.offset((x + 1 as core::ffi::c_int) as isize) as core::ffi::c_int
                * 9 as core::ffi::c_int + 8 as core::ffi::c_int >> 4 as core::ffi::c_int)
            as uint16_t;
        x += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ScaleCols_C(
    mut dst_ptr: *mut uint8_t,
    mut src_ptr: *const uint8_t,
    mut dst_width: core::ffi::c_int,
    mut x: core::ffi::c_int,
    mut dx: core::ffi::c_int,
) {
    let mut j: core::ffi::c_int = 0;
    j = 0 as core::ffi::c_int;
    while j < dst_width - 1 as core::ffi::c_int {
        *dst_ptr.offset(0 as core::ffi::c_int as isize) = *src_ptr
            .offset((x >> 16 as core::ffi::c_int) as isize);
        x += dx;
        *dst_ptr.offset(1 as core::ffi::c_int as isize) = *src_ptr
            .offset((x >> 16 as core::ffi::c_int) as isize);
        x += dx;
        dst_ptr = dst_ptr.offset(2 as core::ffi::c_int as isize);
        j += 2 as core::ffi::c_int;
    }
    if dst_width & 1 as core::ffi::c_int != 0 {
        *dst_ptr.offset(0 as core::ffi::c_int as isize) = *src_ptr
            .offset((x >> 16 as core::ffi::c_int) as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ScaleCols_16_C(
    mut dst_ptr: *mut uint16_t,
    mut src_ptr: *const uint16_t,
    mut dst_width: core::ffi::c_int,
    mut x: core::ffi::c_int,
    mut dx: core::ffi::c_int,
) {
    let mut j: core::ffi::c_int = 0;
    j = 0 as core::ffi::c_int;
    while j < dst_width - 1 as core::ffi::c_int {
        *dst_ptr.offset(0 as core::ffi::c_int as isize) = *src_ptr
            .offset((x >> 16 as core::ffi::c_int) as isize);
        x += dx;
        *dst_ptr.offset(1 as core::ffi::c_int as isize) = *src_ptr
            .offset((x >> 16 as core::ffi::c_int) as isize);
        x += dx;
        dst_ptr = dst_ptr.offset(2 as core::ffi::c_int as isize);
        j += 2 as core::ffi::c_int;
    }
    if dst_width & 1 as core::ffi::c_int != 0 {
        *dst_ptr.offset(0 as core::ffi::c_int as isize) = *src_ptr
            .offset((x >> 16 as core::ffi::c_int) as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ScaleColsUp2_C(
    mut dst_ptr: *mut uint8_t,
    mut src_ptr: *const uint8_t,
    mut dst_width: core::ffi::c_int,
    mut x: core::ffi::c_int,
    mut dx: core::ffi::c_int,
) {
    let mut j: core::ffi::c_int = 0;
    j = 0 as core::ffi::c_int;
    while j < dst_width - 1 as core::ffi::c_int {
        let ref mut fresh0 = *dst_ptr.offset(0 as core::ffi::c_int as isize);
        *fresh0 = *src_ptr.offset(0 as core::ffi::c_int as isize);
        *dst_ptr.offset(1 as core::ffi::c_int as isize) = *fresh0;
        src_ptr = src_ptr.offset(1 as core::ffi::c_int as isize);
        dst_ptr = dst_ptr.offset(2 as core::ffi::c_int as isize);
        j += 2 as core::ffi::c_int;
    }
    if dst_width & 1 as core::ffi::c_int != 0 {
        *dst_ptr.offset(0 as core::ffi::c_int as isize) = *src_ptr
            .offset(0 as core::ffi::c_int as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ScaleColsUp2_16_C(
    mut dst_ptr: *mut uint16_t,
    mut src_ptr: *const uint16_t,
    mut dst_width: core::ffi::c_int,
    mut x: core::ffi::c_int,
    mut dx: core::ffi::c_int,
) {
    let mut j: core::ffi::c_int = 0;
    j = 0 as core::ffi::c_int;
    while j < dst_width - 1 as core::ffi::c_int {
        let ref mut fresh1 = *dst_ptr.offset(0 as core::ffi::c_int as isize);
        *fresh1 = *src_ptr.offset(0 as core::ffi::c_int as isize);
        *dst_ptr.offset(1 as core::ffi::c_int as isize) = *fresh1;
        src_ptr = src_ptr.offset(1 as core::ffi::c_int as isize);
        dst_ptr = dst_ptr.offset(2 as core::ffi::c_int as isize);
        j += 2 as core::ffi::c_int;
    }
    if dst_width & 1 as core::ffi::c_int != 0 {
        *dst_ptr.offset(0 as core::ffi::c_int as isize) = *src_ptr
            .offset(0 as core::ffi::c_int as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ScaleFilterCols_C(
    mut dst_ptr: *mut uint8_t,
    mut src_ptr: *const uint8_t,
    mut dst_width: core::ffi::c_int,
    mut x: core::ffi::c_int,
    mut dx: core::ffi::c_int,
) {
    let mut j: core::ffi::c_int = 0;
    j = 0 as core::ffi::c_int;
    while j < dst_width - 1 as core::ffi::c_int {
        let mut xi: core::ffi::c_int = x >> 16 as core::ffi::c_int;
        let mut a: core::ffi::c_int = *src_ptr.offset(xi as isize) as core::ffi::c_int;
        let mut b: core::ffi::c_int = *src_ptr
            .offset((xi + 1 as core::ffi::c_int) as isize) as core::ffi::c_int;
        *dst_ptr.offset(0 as core::ffi::c_int as isize) = (a
            + (((x & 0xffff as core::ffi::c_int) >> 9 as core::ffi::c_int) * (b - a)
                + 0x40 as core::ffi::c_int >> 7 as core::ffi::c_int)) as uint8_t;
        x += dx;
        xi = x >> 16 as core::ffi::c_int;
        a = *src_ptr.offset(xi as isize) as core::ffi::c_int;
        b = *src_ptr.offset((xi + 1 as core::ffi::c_int) as isize) as core::ffi::c_int;
        *dst_ptr.offset(1 as core::ffi::c_int as isize) = (a
            + (((x & 0xffff as core::ffi::c_int) >> 9 as core::ffi::c_int) * (b - a)
                + 0x40 as core::ffi::c_int >> 7 as core::ffi::c_int)) as uint8_t;
        x += dx;
        dst_ptr = dst_ptr.offset(2 as core::ffi::c_int as isize);
        j += 2 as core::ffi::c_int;
    }
    if dst_width & 1 as core::ffi::c_int != 0 {
        let mut xi_0: core::ffi::c_int = x >> 16 as core::ffi::c_int;
        let mut a_0: core::ffi::c_int = *src_ptr.offset(xi_0 as isize)
            as core::ffi::c_int;
        let mut b_0: core::ffi::c_int = *src_ptr
            .offset((xi_0 + 1 as core::ffi::c_int) as isize) as core::ffi::c_int;
        *dst_ptr.offset(0 as core::ffi::c_int as isize) = (a_0
            + (((x & 0xffff as core::ffi::c_int) >> 9 as core::ffi::c_int) * (b_0 - a_0)
                + 0x40 as core::ffi::c_int >> 7 as core::ffi::c_int)) as uint8_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ScaleFilterCols64_C(
    mut dst_ptr: *mut uint8_t,
    mut src_ptr: *const uint8_t,
    mut dst_width: core::ffi::c_int,
    mut x32: core::ffi::c_int,
    mut dx: core::ffi::c_int,
) {
    let mut x: int64_t = x32 as int64_t;
    let mut j: core::ffi::c_int = 0;
    j = 0 as core::ffi::c_int;
    while j < dst_width - 1 as core::ffi::c_int {
        let mut xi: int64_t = x >> 16 as core::ffi::c_int;
        let mut a: core::ffi::c_int = *src_ptr.offset(xi as isize) as core::ffi::c_int;
        let mut b: core::ffi::c_int = *src_ptr.offset((xi + 1 as int64_t) as isize)
            as core::ffi::c_int;
        *dst_ptr.offset(0 as core::ffi::c_int as isize) = (a
            + (((x & 0xffff as int64_t) >> 9 as core::ffi::c_int) as core::ffi::c_int
                * (b - a) + 0x40 as core::ffi::c_int >> 7 as core::ffi::c_int))
            as uint8_t;
        x += dx as int64_t;
        xi = x >> 16 as core::ffi::c_int;
        a = *src_ptr.offset(xi as isize) as core::ffi::c_int;
        b = *src_ptr.offset((xi + 1 as int64_t) as isize) as core::ffi::c_int;
        *dst_ptr.offset(1 as core::ffi::c_int as isize) = (a
            + (((x & 0xffff as int64_t) >> 9 as core::ffi::c_int) as core::ffi::c_int
                * (b - a) + 0x40 as core::ffi::c_int >> 7 as core::ffi::c_int))
            as uint8_t;
        x += dx as int64_t;
        dst_ptr = dst_ptr.offset(2 as core::ffi::c_int as isize);
        j += 2 as core::ffi::c_int;
    }
    if dst_width & 1 as core::ffi::c_int != 0 {
        let mut xi_0: int64_t = x >> 16 as core::ffi::c_int;
        let mut a_0: core::ffi::c_int = *src_ptr.offset(xi_0 as isize)
            as core::ffi::c_int;
        let mut b_0: core::ffi::c_int = *src_ptr.offset((xi_0 + 1 as int64_t) as isize)
            as core::ffi::c_int;
        *dst_ptr.offset(0 as core::ffi::c_int as isize) = (a_0
            + (((x & 0xffff as int64_t) >> 9 as core::ffi::c_int) as core::ffi::c_int
                * (b_0 - a_0) + 0x40 as core::ffi::c_int >> 7 as core::ffi::c_int))
            as uint8_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ScaleFilterCols_16_C(
    mut dst_ptr: *mut uint16_t,
    mut src_ptr: *const uint16_t,
    mut dst_width: core::ffi::c_int,
    mut x: core::ffi::c_int,
    mut dx: core::ffi::c_int,
) {
    let mut j: core::ffi::c_int = 0;
    j = 0 as core::ffi::c_int;
    while j < dst_width - 1 as core::ffi::c_int {
        let mut xi: core::ffi::c_int = x >> 16 as core::ffi::c_int;
        let mut a: core::ffi::c_int = *src_ptr.offset(xi as isize) as core::ffi::c_int;
        let mut b: core::ffi::c_int = *src_ptr
            .offset((xi + 1 as core::ffi::c_int) as isize) as core::ffi::c_int;
        *dst_ptr.offset(0 as core::ffi::c_int as isize) = (a
            + ((x & 0xffff as core::ffi::c_int) as int64_t
                * (b as int64_t - a as int64_t) + 0x8000 as int64_t
                >> 16 as core::ffi::c_int) as core::ffi::c_int) as uint16_t;
        x += dx;
        xi = x >> 16 as core::ffi::c_int;
        a = *src_ptr.offset(xi as isize) as core::ffi::c_int;
        b = *src_ptr.offset((xi + 1 as core::ffi::c_int) as isize) as core::ffi::c_int;
        *dst_ptr.offset(1 as core::ffi::c_int as isize) = (a
            + ((x & 0xffff as core::ffi::c_int) as int64_t
                * (b as int64_t - a as int64_t) + 0x8000 as int64_t
                >> 16 as core::ffi::c_int) as core::ffi::c_int) as uint16_t;
        x += dx;
        dst_ptr = dst_ptr.offset(2 as core::ffi::c_int as isize);
        j += 2 as core::ffi::c_int;
    }
    if dst_width & 1 as core::ffi::c_int != 0 {
        let mut xi_0: core::ffi::c_int = x >> 16 as core::ffi::c_int;
        let mut a_0: core::ffi::c_int = *src_ptr.offset(xi_0 as isize)
            as core::ffi::c_int;
        let mut b_0: core::ffi::c_int = *src_ptr
            .offset((xi_0 + 1 as core::ffi::c_int) as isize) as core::ffi::c_int;
        *dst_ptr.offset(0 as core::ffi::c_int as isize) = (a_0
            + ((x & 0xffff as core::ffi::c_int) as int64_t
                * (b_0 as int64_t - a_0 as int64_t) + 0x8000 as int64_t
                >> 16 as core::ffi::c_int) as core::ffi::c_int) as uint16_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ScaleFilterCols64_16_C(
    mut dst_ptr: *mut uint16_t,
    mut src_ptr: *const uint16_t,
    mut dst_width: core::ffi::c_int,
    mut x32: core::ffi::c_int,
    mut dx: core::ffi::c_int,
) {
    let mut x: int64_t = x32 as int64_t;
    let mut j: core::ffi::c_int = 0;
    j = 0 as core::ffi::c_int;
    while j < dst_width - 1 as core::ffi::c_int {
        let mut xi: int64_t = x >> 16 as core::ffi::c_int;
        let mut a: core::ffi::c_int = *src_ptr.offset(xi as isize) as core::ffi::c_int;
        let mut b: core::ffi::c_int = *src_ptr.offset((xi + 1 as int64_t) as isize)
            as core::ffi::c_int;
        *dst_ptr.offset(0 as core::ffi::c_int as isize) = (a
            + ((x & 0xffff as int64_t) * (b as int64_t - a as int64_t)
                + 0x8000 as int64_t >> 16 as core::ffi::c_int) as core::ffi::c_int)
            as uint16_t;
        x += dx as int64_t;
        xi = x >> 16 as core::ffi::c_int;
        a = *src_ptr.offset(xi as isize) as core::ffi::c_int;
        b = *src_ptr.offset((xi + 1 as int64_t) as isize) as core::ffi::c_int;
        *dst_ptr.offset(1 as core::ffi::c_int as isize) = (a
            + ((x & 0xffff as int64_t) * (b as int64_t - a as int64_t)
                + 0x8000 as int64_t >> 16 as core::ffi::c_int) as core::ffi::c_int)
            as uint16_t;
        x += dx as int64_t;
        dst_ptr = dst_ptr.offset(2 as core::ffi::c_int as isize);
        j += 2 as core::ffi::c_int;
    }
    if dst_width & 1 as core::ffi::c_int != 0 {
        let mut xi_0: int64_t = x >> 16 as core::ffi::c_int;
        let mut a_0: core::ffi::c_int = *src_ptr.offset(xi_0 as isize)
            as core::ffi::c_int;
        let mut b_0: core::ffi::c_int = *src_ptr.offset((xi_0 + 1 as int64_t) as isize)
            as core::ffi::c_int;
        *dst_ptr.offset(0 as core::ffi::c_int as isize) = (a_0
            + ((x & 0xffff as int64_t) * (b_0 as int64_t - a_0 as int64_t)
                + 0x8000 as int64_t >> 16 as core::ffi::c_int) as core::ffi::c_int)
            as uint16_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ScaleAddRow_C(
    mut src_ptr: *const uint8_t,
    mut dst_ptr: *mut uint16_t,
    mut src_width: core::ffi::c_int,
) {
    let mut x: core::ffi::c_int = 0;
    x = 0 as core::ffi::c_int;
    while x < src_width - 1 as core::ffi::c_int {
        let ref mut fresh2 = *dst_ptr.offset(0 as core::ffi::c_int as isize);
        *fresh2 = (*fresh2 as core::ffi::c_int
            + *src_ptr.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int)
            as uint16_t;
        let ref mut fresh3 = *dst_ptr.offset(1 as core::ffi::c_int as isize);
        *fresh3 = (*fresh3 as core::ffi::c_int
            + *src_ptr.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int)
            as uint16_t;
        src_ptr = src_ptr.offset(2 as core::ffi::c_int as isize);
        dst_ptr = dst_ptr.offset(2 as core::ffi::c_int as isize);
        x += 2 as core::ffi::c_int;
    }
    if src_width & 1 as core::ffi::c_int != 0 {
        let ref mut fresh4 = *dst_ptr.offset(0 as core::ffi::c_int as isize);
        *fresh4 = (*fresh4 as core::ffi::c_int
            + *src_ptr.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int)
            as uint16_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ScaleAddRow_16_C(
    mut src_ptr: *const uint16_t,
    mut dst_ptr: *mut uint32_t,
    mut src_width: core::ffi::c_int,
) {
    let mut x: core::ffi::c_int = 0;
    x = 0 as core::ffi::c_int;
    while x < src_width - 1 as core::ffi::c_int {
        let ref mut fresh5 = *dst_ptr.offset(0 as core::ffi::c_int as isize);
        *fresh5 = (*fresh5)
            .wrapping_add(*src_ptr.offset(0 as core::ffi::c_int as isize) as uint32_t);
        let ref mut fresh6 = *dst_ptr.offset(1 as core::ffi::c_int as isize);
        *fresh6 = (*fresh6)
            .wrapping_add(*src_ptr.offset(1 as core::ffi::c_int as isize) as uint32_t);
        src_ptr = src_ptr.offset(2 as core::ffi::c_int as isize);
        dst_ptr = dst_ptr.offset(2 as core::ffi::c_int as isize);
        x += 2 as core::ffi::c_int;
    }
    if src_width & 1 as core::ffi::c_int != 0 {
        let ref mut fresh7 = *dst_ptr.offset(0 as core::ffi::c_int as isize);
        *fresh7 = (*fresh7)
            .wrapping_add(*src_ptr.offset(0 as core::ffi::c_int as isize) as uint32_t);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ScalePlaneVertical(
    mut src_height: core::ffi::c_int,
    mut dst_width: core::ffi::c_int,
    mut dst_height: core::ffi::c_int,
    mut src_stride: core::ffi::c_int,
    mut dst_stride: core::ffi::c_int,
    mut src_argb: *const uint8_t,
    mut dst_argb: *mut uint8_t,
    mut x: core::ffi::c_int,
    mut y: core::ffi::c_int,
    mut dy: core::ffi::c_int,
    mut bpp: core::ffi::c_int,
    mut filtering: FilterMode,
) {
    let mut dst_width_bytes: core::ffi::c_int = dst_width * bpp;
    let mut InterpolateRow: Option<
        unsafe extern "C" fn(
            *mut uint8_t,
            *const uint8_t,
            ptrdiff_t,
            core::ffi::c_int,
            core::ffi::c_int,
        ) -> (),
    > = Some(
        InterpolateRow_C
            as unsafe extern "C" fn(
                *mut uint8_t,
                *const uint8_t,
                ptrdiff_t,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    );
    let max_y: core::ffi::c_int = if src_height > 1 as core::ffi::c_int {
        ((src_height - 1 as core::ffi::c_int) << 16 as core::ffi::c_int)
            - 1 as core::ffi::c_int
    } else {
        0 as core::ffi::c_int
    };
    let mut j: core::ffi::c_int = 0;
    src_argb = src_argb.offset(((x >> 16 as core::ffi::c_int) * bpp) as isize);
    j = 0 as core::ffi::c_int;
    while j < dst_height {
        let mut yi: core::ffi::c_int = 0;
        let mut yf: core::ffi::c_int = 0;
        if y > max_y {
            y = max_y;
        }
        yi = y >> 16 as core::ffi::c_int;
        yf = if filtering as core::ffi::c_uint != 0 {
            y >> 8 as core::ffi::c_int & 255 as core::ffi::c_int
        } else {
            0 as core::ffi::c_int
        };
        InterpolateRow
            .expect(
                "non-null function pointer",
            )(
            dst_argb,
            src_argb.offset((yi * src_stride) as isize),
            src_stride as ptrdiff_t,
            dst_width_bytes,
            yf,
        );
        dst_argb = dst_argb.offset(dst_stride as isize);
        y += dy;
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ScalePlaneVertical_16(
    mut src_height: core::ffi::c_int,
    mut dst_width: core::ffi::c_int,
    mut dst_height: core::ffi::c_int,
    mut src_stride: core::ffi::c_int,
    mut dst_stride: core::ffi::c_int,
    mut src_argb: *const uint16_t,
    mut dst_argb: *mut uint16_t,
    mut x: core::ffi::c_int,
    mut y: core::ffi::c_int,
    mut dy: core::ffi::c_int,
    mut wpp: core::ffi::c_int,
    mut filtering: FilterMode,
) {
    let mut dst_width_words: core::ffi::c_int = dst_width * wpp;
    let mut InterpolateRow: Option<
        unsafe extern "C" fn(
            *mut uint16_t,
            *const uint16_t,
            ptrdiff_t,
            core::ffi::c_int,
            core::ffi::c_int,
        ) -> (),
    > = Some(
        InterpolateRow_16_C
            as unsafe extern "C" fn(
                *mut uint16_t,
                *const uint16_t,
                ptrdiff_t,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    );
    let max_y: core::ffi::c_int = if src_height > 1 as core::ffi::c_int {
        ((src_height - 1 as core::ffi::c_int) << 16 as core::ffi::c_int)
            - 1 as core::ffi::c_int
    } else {
        0 as core::ffi::c_int
    };
    let mut j: core::ffi::c_int = 0;
    src_argb = src_argb.offset(((x >> 16 as core::ffi::c_int) * wpp) as isize);
    j = 0 as core::ffi::c_int;
    while j < dst_height {
        let mut yi: core::ffi::c_int = 0;
        let mut yf: core::ffi::c_int = 0;
        if y > max_y {
            y = max_y;
        }
        yi = y >> 16 as core::ffi::c_int;
        yf = if filtering as core::ffi::c_uint != 0 {
            y >> 8 as core::ffi::c_int & 255 as core::ffi::c_int
        } else {
            0 as core::ffi::c_int
        };
        InterpolateRow
            .expect(
                "non-null function pointer",
            )(
            dst_argb,
            src_argb.offset((yi * src_stride) as isize),
            src_stride as ptrdiff_t,
            dst_width_words,
            yf,
        );
        dst_argb = dst_argb.offset(dst_stride as isize);
        y += dy;
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ScaleFilterReduce(
    mut src_width: core::ffi::c_int,
    mut src_height: core::ffi::c_int,
    mut dst_width: core::ffi::c_int,
    mut dst_height: core::ffi::c_int,
    mut filtering: FilterMode,
) -> FilterMode {
    if src_width < 0 as core::ffi::c_int {
        src_width = -src_width;
    }
    if src_height < 0 as core::ffi::c_int {
        src_height = -src_height;
    }
    if filtering as core::ffi::c_uint
        == kFilterBox as core::ffi::c_int as core::ffi::c_uint
    {
        if dst_width * 2 as core::ffi::c_int >= src_width
            || dst_height * 2 as core::ffi::c_int >= src_height
        {
            filtering = kFilterBilinear;
        }
    }
    if filtering as core::ffi::c_uint
        == kFilterBilinear as core::ffi::c_int as core::ffi::c_uint
    {
        if src_height == 1 as core::ffi::c_int {
            filtering = kFilterLinear;
        }
        if dst_height == src_height || dst_height * 3 as core::ffi::c_int == src_height {
            filtering = kFilterLinear;
        }
        if src_width == 1 as core::ffi::c_int {
            filtering = kFilterNone;
        }
    }
    if filtering as core::ffi::c_uint
        == kFilterLinear as core::ffi::c_int as core::ffi::c_uint
    {
        if src_width == 1 as core::ffi::c_int {
            filtering = kFilterNone;
        }
        if dst_width == src_width || dst_width * 3 as core::ffi::c_int == src_width {
            filtering = kFilterNone;
        }
    }
    return filtering;
}
#[no_mangle]
pub unsafe extern "C" fn FixedDiv_C(
    mut num: core::ffi::c_int,
    mut div: core::ffi::c_int,
) -> core::ffi::c_int {
    return (((num as int64_t) << 16 as core::ffi::c_int) / div as int64_t)
        as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn FixedDiv1_C(
    mut num: core::ffi::c_int,
    mut div: core::ffi::c_int,
) -> core::ffi::c_int {
    return ((((num as int64_t) << 16 as core::ffi::c_int) - 0x10001 as int64_t)
        / (div - 1 as core::ffi::c_int) as int64_t) as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ScaleSlope(
    mut src_width: core::ffi::c_int,
    mut src_height: core::ffi::c_int,
    mut dst_width: core::ffi::c_int,
    mut dst_height: core::ffi::c_int,
    mut filtering: FilterMode,
    mut x: *mut core::ffi::c_int,
    mut y: *mut core::ffi::c_int,
    mut dx: *mut core::ffi::c_int,
    mut dy: *mut core::ffi::c_int,
) {
    if dst_width == 1 as core::ffi::c_int && src_width >= 32768 as core::ffi::c_int {
        dst_width = src_width;
    }
    if dst_height == 1 as core::ffi::c_int && src_height >= 32768 as core::ffi::c_int {
        dst_height = src_height;
    }
    if filtering as core::ffi::c_uint
        == kFilterBox as core::ffi::c_int as core::ffi::c_uint
    {
        *dx = FixedDiv_C(Abs(src_width), dst_width);
        *dy = FixedDiv_C(src_height, dst_height);
        *x = 0 as core::ffi::c_int;
        *y = 0 as core::ffi::c_int;
    } else if filtering as core::ffi::c_uint
        == kFilterBilinear as core::ffi::c_int as core::ffi::c_uint
    {
        if dst_width <= Abs(src_width) {
            *dx = FixedDiv_C(Abs(src_width), dst_width);
            *x = if *dx < 0 as core::ffi::c_int {
                -((-*dx >> 1 as core::ffi::c_int) + -(32768 as core::ffi::c_int))
            } else {
                (*dx >> 1 as core::ffi::c_int) + -(32768 as core::ffi::c_int)
            };
        } else if src_width > 1 as core::ffi::c_int && dst_width > 1 as core::ffi::c_int
        {
            *dx = FixedDiv1_C(Abs(src_width), dst_width);
            *x = 0 as core::ffi::c_int;
        }
        if dst_height <= src_height {
            *dy = FixedDiv_C(src_height, dst_height);
            *y = if *dy < 0 as core::ffi::c_int {
                -((-*dy >> 1 as core::ffi::c_int) + -(32768 as core::ffi::c_int))
            } else {
                (*dy >> 1 as core::ffi::c_int) + -(32768 as core::ffi::c_int)
            };
        } else if src_height > 1 as core::ffi::c_int
            && dst_height > 1 as core::ffi::c_int
        {
            *dy = FixedDiv1_C(src_height, dst_height);
            *y = 0 as core::ffi::c_int;
        }
    } else if filtering as core::ffi::c_uint
        == kFilterLinear as core::ffi::c_int as core::ffi::c_uint
    {
        if dst_width <= Abs(src_width) {
            *dx = FixedDiv_C(Abs(src_width), dst_width);
            *x = if *dx < 0 as core::ffi::c_int {
                -((-*dx >> 1 as core::ffi::c_int) + -(32768 as core::ffi::c_int))
            } else {
                (*dx >> 1 as core::ffi::c_int) + -(32768 as core::ffi::c_int)
            };
        } else if src_width > 1 as core::ffi::c_int && dst_width > 1 as core::ffi::c_int
        {
            *dx = FixedDiv1_C(Abs(src_width), dst_width);
            *x = 0 as core::ffi::c_int;
        }
        *dy = FixedDiv_C(src_height, dst_height);
        *y = *dy >> 1 as core::ffi::c_int;
    } else {
        *dx = FixedDiv_C(Abs(src_width), dst_width);
        *dy = FixedDiv_C(src_height, dst_height);
        *x = if *dx < 0 as core::ffi::c_int {
            -((-*dx >> 1 as core::ffi::c_int) + 0 as core::ffi::c_int)
        } else {
            (*dx >> 1 as core::ffi::c_int) + 0 as core::ffi::c_int
        };
        *y = if *dy < 0 as core::ffi::c_int {
            -((-*dy >> 1 as core::ffi::c_int) + 0 as core::ffi::c_int)
        } else {
            (*dy >> 1 as core::ffi::c_int) + 0 as core::ffi::c_int
        };
    }
    if src_width < 0 as core::ffi::c_int {
        *x += (dst_width - 1 as core::ffi::c_int) * *dx;
        *dx = -*dx;
    }
}
