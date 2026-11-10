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

// Mechanically translated from upstream/aom/aom_dsp/blend_a64_hmask.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uintptr_t = usize;
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_blend_a64_hmask_c(
    mut dst: *mut uint8_t,
    mut dst_stride: uint32_t,
    mut src0: *const uint8_t,
    mut src0_stride: uint32_t,
    mut src1: *const uint8_t,
    mut src1_stride: uint32_t,
    mut mask: *const uint8_t,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0;
    let mut j: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < h {
        j = 0 as core::ffi::c_int;
        while j < w {
            *dst
                .offset(
                    (i as uint32_t).wrapping_mul(dst_stride).wrapping_add(j as uint32_t)
                        as isize,
                ) = (*mask.offset(j as isize) as core::ffi::c_int
                * *src0
                    .offset(
                        (i as uint32_t)
                            .wrapping_mul(src0_stride)
                            .wrapping_add(j as uint32_t) as isize,
                    ) as core::ffi::c_int
                + (((1 as core::ffi::c_int) << 6 as core::ffi::c_int)
                    - *mask.offset(j as isize) as core::ffi::c_int)
                    * *src1
                        .offset(
                            (i as uint32_t)
                                .wrapping_mul(src1_stride)
                                .wrapping_add(j as uint32_t) as isize,
                        ) as core::ffi::c_int
                + ((1 as core::ffi::c_int) << 6 as core::ffi::c_int
                    >> 1 as core::ffi::c_int) >> 6 as core::ffi::c_int) as uint8_t;
            j += 1;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_blend_a64_hmask_c(
    mut dst_8: *mut uint8_t,
    mut dst_stride: uint32_t,
    mut src0_8: *const uint8_t,
    mut src0_stride: uint32_t,
    mut src1_8: *const uint8_t,
    mut src1_stride: uint32_t,
    mut mask: *const uint8_t,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut bd: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0;
    let mut j: core::ffi::c_int = 0;
    let mut dst: *mut uint16_t = ((dst_8 as uintptr_t) << 1 as core::ffi::c_int)
        as *mut uint16_t;
    let mut src0: *const uint16_t = ((src0_8 as uintptr_t) << 1 as core::ffi::c_int)
        as *mut uint16_t;
    let mut src1: *const uint16_t = ((src1_8 as uintptr_t) << 1 as core::ffi::c_int)
        as *mut uint16_t;
    i = 0 as core::ffi::c_int;
    while i < h {
        j = 0 as core::ffi::c_int;
        while j < w {
            *dst
                .offset(
                    (i as uint32_t).wrapping_mul(dst_stride).wrapping_add(j as uint32_t)
                        as isize,
                ) = (*mask.offset(j as isize) as core::ffi::c_int
                * *src0
                    .offset(
                        (i as uint32_t)
                            .wrapping_mul(src0_stride)
                            .wrapping_add(j as uint32_t) as isize,
                    ) as core::ffi::c_int
                + (((1 as core::ffi::c_int) << 6 as core::ffi::c_int)
                    - *mask.offset(j as isize) as core::ffi::c_int)
                    * *src1
                        .offset(
                            (i as uint32_t)
                                .wrapping_mul(src1_stride)
                                .wrapping_add(j as uint32_t) as isize,
                        ) as core::ffi::c_int
                + ((1 as core::ffi::c_int) << 6 as core::ffi::c_int
                    >> 1 as core::ffi::c_int) >> 6 as core::ffi::c_int) as uint16_t;
            j += 1;
        }
        i += 1;
    }
}
