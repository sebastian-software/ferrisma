/*
 *  Copyright 2015 The LibYuv Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS. All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

// Mechanically translated from upstream/libavif/third_party/libyuv/source/scale_any.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C" {
    fn ScaleRowUp2_Linear_C(
        src_ptr: *const uint8_t,
        dst_ptr: *mut uint8_t,
        dst_width: core::ffi::c_int,
    );
    fn ScaleRowUp2_Bilinear_C(
        src_ptr: *const uint8_t,
        src_stride: ptrdiff_t,
        dst_ptr: *mut uint8_t,
        dst_stride: ptrdiff_t,
        dst_width: core::ffi::c_int,
    );
    fn ScaleRowUp2_Linear_16_C(
        src_ptr: *const uint16_t,
        dst_ptr: *mut uint16_t,
        dst_width: core::ffi::c_int,
    );
    fn ScaleRowUp2_Bilinear_16_C(
        src_ptr: *const uint16_t,
        src_stride: ptrdiff_t,
        dst_ptr: *mut uint16_t,
        dst_stride: ptrdiff_t,
        dst_width: core::ffi::c_int,
    );
}
pub type ptrdiff_t = isize;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
#[no_mangle]
pub unsafe extern "C" fn ScaleRowUp2_Linear_Any_C(
    mut src_ptr: *const uint8_t,
    mut dst_ptr: *mut uint8_t,
    mut dst_width: core::ffi::c_int,
) {
    let mut work_width: core::ffi::c_int = dst_width - 1 as core::ffi::c_int
        & !(1 as core::ffi::c_int);
    let mut r: core::ffi::c_int = work_width & 0 as core::ffi::c_int;
    let mut n: core::ffi::c_int = work_width & !(0 as core::ffi::c_int);
    *dst_ptr.offset(0 as core::ffi::c_int as isize) = *src_ptr
        .offset(0 as core::ffi::c_int as isize);
    if work_width > 0 as core::ffi::c_int {
        if n != 0 as core::ffi::c_int {
            ScaleRowUp2_Linear_C(
                src_ptr,
                dst_ptr.offset(1 as core::ffi::c_int as isize),
                n,
            );
        }
        ScaleRowUp2_Linear_C(
            src_ptr.offset((n / 2 as core::ffi::c_int) as isize),
            dst_ptr.offset(n as isize).offset(1 as core::ffi::c_int as isize),
            r,
        );
    }
    *dst_ptr.offset((dst_width - 1 as core::ffi::c_int) as isize) = *src_ptr
        .offset(((dst_width - 1 as core::ffi::c_int) / 2 as core::ffi::c_int) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn ScaleRowUp2_Linear_16_Any_C(
    mut src_ptr: *const uint16_t,
    mut dst_ptr: *mut uint16_t,
    mut dst_width: core::ffi::c_int,
) {
    let mut work_width: core::ffi::c_int = dst_width - 1 as core::ffi::c_int
        & !(1 as core::ffi::c_int);
    let mut r: core::ffi::c_int = work_width & 0 as core::ffi::c_int;
    let mut n: core::ffi::c_int = work_width & !(0 as core::ffi::c_int);
    *dst_ptr.offset(0 as core::ffi::c_int as isize) = *src_ptr
        .offset(0 as core::ffi::c_int as isize);
    if work_width > 0 as core::ffi::c_int {
        if n != 0 as core::ffi::c_int {
            ScaleRowUp2_Linear_16_C(
                src_ptr,
                dst_ptr.offset(1 as core::ffi::c_int as isize),
                n,
            );
        }
        ScaleRowUp2_Linear_16_C(
            src_ptr.offset((n / 2 as core::ffi::c_int) as isize),
            dst_ptr.offset(n as isize).offset(1 as core::ffi::c_int as isize),
            r,
        );
    }
    *dst_ptr.offset((dst_width - 1 as core::ffi::c_int) as isize) = *src_ptr
        .offset(((dst_width - 1 as core::ffi::c_int) / 2 as core::ffi::c_int) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn ScaleRowUp2_Bilinear_Any_C(
    mut src_ptr: *const uint8_t,
    mut src_stride: ptrdiff_t,
    mut dst_ptr: *mut uint8_t,
    mut dst_stride: ptrdiff_t,
    mut dst_width: core::ffi::c_int,
) {
    let mut work_width: core::ffi::c_int = dst_width - 1 as core::ffi::c_int
        & !(1 as core::ffi::c_int);
    let mut r: core::ffi::c_int = work_width & 0 as core::ffi::c_int;
    let mut n: core::ffi::c_int = work_width & !(0 as core::ffi::c_int);
    let mut sa: *const uint8_t = src_ptr;
    let mut sb: *const uint8_t = src_ptr.offset(src_stride as isize);
    let mut da: *mut uint8_t = dst_ptr;
    let mut db: *mut uint8_t = dst_ptr.offset(dst_stride as isize);
    *da.offset(0 as core::ffi::c_int as isize) = (3 as core::ffi::c_int
        * *sa.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
        + *sb.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
        + 2 as core::ffi::c_int >> 2 as core::ffi::c_int) as uint8_t;
    *db.offset(0 as core::ffi::c_int as isize) = (*sa
        .offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
        + 3 as core::ffi::c_int
            * *sb.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
        + 2 as core::ffi::c_int >> 2 as core::ffi::c_int) as uint8_t;
    if work_width > 0 as core::ffi::c_int {
        if n != 0 as core::ffi::c_int {
            ScaleRowUp2_Bilinear_C(
                sa,
                sb.offset_from(sa) as ptrdiff_t,
                da.offset(1 as core::ffi::c_int as isize),
                db.offset_from(da) as ptrdiff_t,
                n,
            );
        }
        ScaleRowUp2_Bilinear_C(
            sa.offset((n / 2 as core::ffi::c_int) as isize),
            sb.offset_from(sa) as ptrdiff_t,
            da.offset(n as isize).offset(1 as core::ffi::c_int as isize),
            db.offset_from(da) as ptrdiff_t,
            r,
        );
    }
    *da.offset((dst_width - 1 as core::ffi::c_int) as isize) = (3 as core::ffi::c_int
        * *sa
            .offset(
                ((dst_width - 1 as core::ffi::c_int) / 2 as core::ffi::c_int) as isize,
            ) as core::ffi::c_int
        + *sb
            .offset(
                ((dst_width - 1 as core::ffi::c_int) / 2 as core::ffi::c_int) as isize,
            ) as core::ffi::c_int + 2 as core::ffi::c_int >> 2 as core::ffi::c_int)
        as uint8_t;
    *db.offset((dst_width - 1 as core::ffi::c_int) as isize) = (*sa
        .offset(((dst_width - 1 as core::ffi::c_int) / 2 as core::ffi::c_int) as isize)
        as core::ffi::c_int
        + 3 as core::ffi::c_int
            * *sb
                .offset(
                    ((dst_width - 1 as core::ffi::c_int) / 2 as core::ffi::c_int)
                        as isize,
                ) as core::ffi::c_int + 2 as core::ffi::c_int >> 2 as core::ffi::c_int)
        as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn ScaleRowUp2_Bilinear_16_Any_C(
    mut src_ptr: *const uint16_t,
    mut src_stride: ptrdiff_t,
    mut dst_ptr: *mut uint16_t,
    mut dst_stride: ptrdiff_t,
    mut dst_width: core::ffi::c_int,
) {
    let mut work_width: core::ffi::c_int = dst_width - 1 as core::ffi::c_int
        & !(1 as core::ffi::c_int);
    let mut r: core::ffi::c_int = work_width & 0 as core::ffi::c_int;
    let mut n: core::ffi::c_int = work_width & !(0 as core::ffi::c_int);
    let mut sa: *const uint16_t = src_ptr;
    let mut sb: *const uint16_t = src_ptr.offset(src_stride as isize);
    let mut da: *mut uint16_t = dst_ptr;
    let mut db: *mut uint16_t = dst_ptr.offset(dst_stride as isize);
    *da.offset(0 as core::ffi::c_int as isize) = (3 as core::ffi::c_int
        * *sa.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
        + *sb.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
        + 2 as core::ffi::c_int >> 2 as core::ffi::c_int) as uint16_t;
    *db.offset(0 as core::ffi::c_int as isize) = (*sa
        .offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
        + 3 as core::ffi::c_int
            * *sb.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
        + 2 as core::ffi::c_int >> 2 as core::ffi::c_int) as uint16_t;
    if work_width > 0 as core::ffi::c_int {
        if n != 0 as core::ffi::c_int {
            ScaleRowUp2_Bilinear_16_C(
                sa,
                sb.offset_from(sa) as ptrdiff_t,
                da.offset(1 as core::ffi::c_int as isize),
                db.offset_from(da) as ptrdiff_t,
                n,
            );
        }
        ScaleRowUp2_Bilinear_16_C(
            sa.offset((n / 2 as core::ffi::c_int) as isize),
            sb.offset_from(sa) as ptrdiff_t,
            da.offset(n as isize).offset(1 as core::ffi::c_int as isize),
            db.offset_from(da) as ptrdiff_t,
            r,
        );
    }
    *da.offset((dst_width - 1 as core::ffi::c_int) as isize) = (3 as core::ffi::c_int
        * *sa
            .offset(
                ((dst_width - 1 as core::ffi::c_int) / 2 as core::ffi::c_int) as isize,
            ) as core::ffi::c_int
        + *sb
            .offset(
                ((dst_width - 1 as core::ffi::c_int) / 2 as core::ffi::c_int) as isize,
            ) as core::ffi::c_int + 2 as core::ffi::c_int >> 2 as core::ffi::c_int)
        as uint16_t;
    *db.offset((dst_width - 1 as core::ffi::c_int) as isize) = (*sa
        .offset(((dst_width - 1 as core::ffi::c_int) / 2 as core::ffi::c_int) as isize)
        as core::ffi::c_int
        + 3 as core::ffi::c_int
            * *sb
                .offset(
                    ((dst_width - 1 as core::ffi::c_int) / 2 as core::ffi::c_int)
                        as isize,
                ) as core::ffi::c_int + 2 as core::ffi::c_int >> 2 as core::ffi::c_int)
        as uint16_t;
}
