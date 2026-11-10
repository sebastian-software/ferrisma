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

// Mechanically translated from upstream/aom/aom_dsp/subtract.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
pub type ptrdiff_t = isize;
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type int16_t = __int16_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uintptr_t = usize;
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_subtract_block_c(
    mut rows: core::ffi::c_int,
    mut cols: core::ffi::c_int,
    mut diff: *mut int16_t,
    mut diff_stride: ptrdiff_t,
    mut src: *const uint8_t,
    mut src_stride: ptrdiff_t,
    mut pred: *const uint8_t,
    mut pred_stride: ptrdiff_t,
) {
    let mut r: core::ffi::c_int = 0;
    let mut c: core::ffi::c_int = 0;
    r = 0 as core::ffi::c_int;
    while r < rows {
        c = 0 as core::ffi::c_int;
        while c < cols {
            *diff.offset(c as isize) = (*src.offset(c as isize) as core::ffi::c_int
                - *pred.offset(c as isize) as core::ffi::c_int) as int16_t;
            c += 1;
        }
        diff = diff.offset(diff_stride as isize);
        pred = pred.offset(pred_stride as isize);
        src = src.offset(src_stride as isize);
        r += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_subtract_block_c(
    mut rows: core::ffi::c_int,
    mut cols: core::ffi::c_int,
    mut diff: *mut int16_t,
    mut diff_stride: ptrdiff_t,
    mut src8: *const uint8_t,
    mut src_stride: ptrdiff_t,
    mut pred8: *const uint8_t,
    mut pred_stride: ptrdiff_t,
) {
    let mut r: core::ffi::c_int = 0;
    let mut c: core::ffi::c_int = 0;
    let mut src: *mut uint16_t = ((src8 as uintptr_t) << 1 as core::ffi::c_int)
        as *mut uint16_t;
    let mut pred: *mut uint16_t = ((pred8 as uintptr_t) << 1 as core::ffi::c_int)
        as *mut uint16_t;
    r = 0 as core::ffi::c_int;
    while r < rows {
        c = 0 as core::ffi::c_int;
        while c < cols {
            *diff.offset(c as isize) = (*src.offset(c as isize) as core::ffi::c_int
                - *pred.offset(c as isize) as core::ffi::c_int) as int16_t;
            c += 1;
        }
        diff = diff.offset(diff_stride as isize);
        pred = pred.offset(pred_stride as isize);
        src = src.offset(src_stride as isize);
        r += 1;
    }
}
