/*
 *  Copyright 2011 The LibYuv Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS. All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

// Mechanically translated from upstream/libavif/third_party/libyuv/source/planar_functions.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C" {
    fn CopyRow_C(src: *const uint8_t, dst: *mut uint8_t, count: core::ffi::c_int);
}
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
#[no_mangle]
pub unsafe extern "C" fn CopyPlane(
    mut src_y: *const uint8_t,
    mut src_stride_y: core::ffi::c_int,
    mut dst_y: *mut uint8_t,
    mut dst_stride_y: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
) {
    let mut y: core::ffi::c_int = 0;
    let mut CopyRow: Option<
        unsafe extern "C" fn(*const uint8_t, *mut uint8_t, core::ffi::c_int) -> (),
    > = Some(
        CopyRow_C
            as unsafe extern "C" fn(*const uint8_t, *mut uint8_t, core::ffi::c_int) -> (),
    );
    if width <= 0 as core::ffi::c_int || height == 0 as core::ffi::c_int {
        return;
    }
    if height < 0 as core::ffi::c_int {
        height = -height;
        dst_y = dst_y.offset(((height - 1 as core::ffi::c_int) * dst_stride_y) as isize);
        dst_stride_y = -dst_stride_y;
    }
    if src_stride_y == width && dst_stride_y == width {
        width *= height;
        height = 1 as core::ffi::c_int;
        dst_stride_y = 0 as core::ffi::c_int;
        src_stride_y = dst_stride_y;
    }
    if src_y == dst_y as *const uint8_t && src_stride_y == dst_stride_y {
        return;
    }
    y = 0 as core::ffi::c_int;
    while y < height {
        CopyRow.expect("non-null function pointer")(src_y, dst_y, width);
        src_y = src_y.offset(src_stride_y as isize);
        dst_y = dst_y.offset(dst_stride_y as isize);
        y += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CopyPlane_16(
    mut src_y: *const uint16_t,
    mut src_stride_y: core::ffi::c_int,
    mut dst_y: *mut uint16_t,
    mut dst_stride_y: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
) {
    CopyPlane(
        src_y as *const uint8_t,
        src_stride_y * 2 as core::ffi::c_int,
        dst_y as *mut uint8_t,
        dst_stride_y * 2 as core::ffi::c_int,
        width * 2 as core::ffi::c_int,
        height,
    );
}
