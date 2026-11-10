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

// Mechanically translated from upstream/aom/aom_dsp/blend_a64_vmask.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C-unwind" {
}
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uintptr_t = usize;
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_blend_a64_vmask_c(
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
    if !(src0 == dst as *const uint8_t) || src0_stride == dst_stride {} else {
        __assert_fail(
            b"IMPLIES(src0 == dst, src0_stride == dst_stride)\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/aom_dsp/blend_a64_vmask.c\0" as *const u8
                as *const core::ffi::c_char,
            27 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 129],
                [core::ffi::c_char; 129],
            >(
                *b"void aom_blend_a64_vmask_c(uint8_t *, uint32_t, const uint8_t *, uint32_t, const uint8_t *, uint32_t, const uint8_t *, int, int)\0",
            ))
                .as_ptr(),
        );
    };
    if !(src1 == dst as *const uint8_t) || src1_stride == dst_stride {} else {
        __assert_fail(
            b"IMPLIES(src1 == dst, src1_stride == dst_stride)\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/aom_dsp/blend_a64_vmask.c\0" as *const u8
                as *const core::ffi::c_char,
            28 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 129],
                [core::ffi::c_char; 129],
            >(
                *b"void aom_blend_a64_vmask_c(uint8_t *, uint32_t, const uint8_t *, uint32_t, const uint8_t *, uint32_t, const uint8_t *, int, int)\0",
            ))
                .as_ptr(),
        );
    };
    if h >= 1 as core::ffi::c_int {} else {
        __assert_fail(
            b"h >= 1\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/aom_dsp/blend_a64_vmask.c\0" as *const u8
                as *const core::ffi::c_char,
            30 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 129],
                [core::ffi::c_char; 129],
            >(
                *b"void aom_blend_a64_vmask_c(uint8_t *, uint32_t, const uint8_t *, uint32_t, const uint8_t *, uint32_t, const uint8_t *, int, int)\0",
            ))
                .as_ptr(),
        );
    };
    if w >= 1 as core::ffi::c_int {} else {
        __assert_fail(
            b"w >= 1\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/aom_dsp/blend_a64_vmask.c\0" as *const u8
                as *const core::ffi::c_char,
            31 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 129],
                [core::ffi::c_char; 129],
            >(
                *b"void aom_blend_a64_vmask_c(uint8_t *, uint32_t, const uint8_t *, uint32_t, const uint8_t *, uint32_t, const uint8_t *, int, int)\0",
            ))
                .as_ptr(),
        );
    };
    if h & h - 1 as core::ffi::c_int == 0 as core::ffi::c_int {} else {
        __assert_fail(
            b"IS_POWER_OF_TWO(h)\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/aom_dsp/blend_a64_vmask.c\0" as *const u8
                as *const core::ffi::c_char,
            32 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 129],
                [core::ffi::c_char; 129],
            >(
                *b"void aom_blend_a64_vmask_c(uint8_t *, uint32_t, const uint8_t *, uint32_t, const uint8_t *, uint32_t, const uint8_t *, int, int)\0",
            ))
                .as_ptr(),
        );
    };
    if w & w - 1 as core::ffi::c_int == 0 as core::ffi::c_int {} else {
        __assert_fail(
            b"IS_POWER_OF_TWO(w)\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/aom_dsp/blend_a64_vmask.c\0" as *const u8
                as *const core::ffi::c_char,
            33 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 129],
                [core::ffi::c_char; 129],
            >(
                *b"void aom_blend_a64_vmask_c(uint8_t *, uint32_t, const uint8_t *, uint32_t, const uint8_t *, uint32_t, const uint8_t *, int, int)\0",
            ))
                .as_ptr(),
        );
    };
    i = 0 as core::ffi::c_int;
    while i < h {
        let m: core::ffi::c_int = *mask.offset(i as isize) as core::ffi::c_int;
        j = 0 as core::ffi::c_int;
        while j < w {
            *dst
                .offset(
                    (i as uint32_t).wrapping_mul(dst_stride).wrapping_add(j as uint32_t)
                        as isize,
                ) = (m
                * *src0
                    .offset(
                        (i as uint32_t)
                            .wrapping_mul(src0_stride)
                            .wrapping_add(j as uint32_t) as isize,
                    ) as core::ffi::c_int
                + (((1 as core::ffi::c_int) << 6 as core::ffi::c_int) - m)
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
pub unsafe extern "C-unwind" fn aom_highbd_blend_a64_vmask_c(
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
    if !(src0 == dst as *const uint16_t) || src0_stride == dst_stride {} else {
        __assert_fail(
            b"IMPLIES(src0 == dst, src0_stride == dst_stride)\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/aom_dsp/blend_a64_vmask.c\0" as *const u8
                as *const core::ffi::c_char,
            55 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 141],
                [core::ffi::c_char; 141],
            >(
                *b"void aom_highbd_blend_a64_vmask_c(uint8_t *, uint32_t, const uint8_t *, uint32_t, const uint8_t *, uint32_t, const uint8_t *, int, int, int)\0",
            ))
                .as_ptr(),
        );
    };
    if !(src1 == dst as *const uint16_t) || src1_stride == dst_stride {} else {
        __assert_fail(
            b"IMPLIES(src1 == dst, src1_stride == dst_stride)\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/aom_dsp/blend_a64_vmask.c\0" as *const u8
                as *const core::ffi::c_char,
            56 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 141],
                [core::ffi::c_char; 141],
            >(
                *b"void aom_highbd_blend_a64_vmask_c(uint8_t *, uint32_t, const uint8_t *, uint32_t, const uint8_t *, uint32_t, const uint8_t *, int, int, int)\0",
            ))
                .as_ptr(),
        );
    };
    if h >= 1 as core::ffi::c_int {} else {
        __assert_fail(
            b"h >= 1\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/aom_dsp/blend_a64_vmask.c\0" as *const u8
                as *const core::ffi::c_char,
            58 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 141],
                [core::ffi::c_char; 141],
            >(
                *b"void aom_highbd_blend_a64_vmask_c(uint8_t *, uint32_t, const uint8_t *, uint32_t, const uint8_t *, uint32_t, const uint8_t *, int, int, int)\0",
            ))
                .as_ptr(),
        );
    };
    if w >= 1 as core::ffi::c_int {} else {
        __assert_fail(
            b"w >= 1\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/aom_dsp/blend_a64_vmask.c\0" as *const u8
                as *const core::ffi::c_char,
            59 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 141],
                [core::ffi::c_char; 141],
            >(
                *b"void aom_highbd_blend_a64_vmask_c(uint8_t *, uint32_t, const uint8_t *, uint32_t, const uint8_t *, uint32_t, const uint8_t *, int, int, int)\0",
            ))
                .as_ptr(),
        );
    };
    if h & h - 1 as core::ffi::c_int == 0 as core::ffi::c_int {} else {
        __assert_fail(
            b"IS_POWER_OF_TWO(h)\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/aom_dsp/blend_a64_vmask.c\0" as *const u8
                as *const core::ffi::c_char,
            60 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 141],
                [core::ffi::c_char; 141],
            >(
                *b"void aom_highbd_blend_a64_vmask_c(uint8_t *, uint32_t, const uint8_t *, uint32_t, const uint8_t *, uint32_t, const uint8_t *, int, int, int)\0",
            ))
                .as_ptr(),
        );
    };
    if w & w - 1 as core::ffi::c_int == 0 as core::ffi::c_int {} else {
        __assert_fail(
            b"IS_POWER_OF_TWO(w)\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/aom_dsp/blend_a64_vmask.c\0" as *const u8
                as *const core::ffi::c_char,
            61 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 141],
                [core::ffi::c_char; 141],
            >(
                *b"void aom_highbd_blend_a64_vmask_c(uint8_t *, uint32_t, const uint8_t *, uint32_t, const uint8_t *, uint32_t, const uint8_t *, int, int, int)\0",
            ))
                .as_ptr(),
        );
    };
    if bd == 8 as core::ffi::c_int || bd == 10 as core::ffi::c_int
        || bd == 12 as core::ffi::c_int
    {} else {
        __assert_fail(
            b"bd == 8 || bd == 10 || bd == 12\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/aom_dsp/blend_a64_vmask.c\0" as *const u8
                as *const core::ffi::c_char,
            63 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 141],
                [core::ffi::c_char; 141],
            >(
                *b"void aom_highbd_blend_a64_vmask_c(uint8_t *, uint32_t, const uint8_t *, uint32_t, const uint8_t *, uint32_t, const uint8_t *, int, int, int)\0",
            ))
                .as_ptr(),
        );
    };
    i = 0 as core::ffi::c_int;
    while i < h {
        let m: core::ffi::c_int = *mask.offset(i as isize) as core::ffi::c_int;
        j = 0 as core::ffi::c_int;
        while j < w {
            *dst
                .offset(
                    (i as uint32_t).wrapping_mul(dst_stride).wrapping_add(j as uint32_t)
                        as isize,
                ) = (m
                * *src0
                    .offset(
                        (i as uint32_t)
                            .wrapping_mul(src0_stride)
                            .wrapping_add(j as uint32_t) as isize,
                    ) as core::ffi::c_int
                + (((1 as core::ffi::c_int) << 6 as core::ffi::c_int) - m)
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

extern "C" {
    fn __assert_fail(
        __assertion: *const core::ffi::c_char,
        __file: *const core::ffi::c_char,
        __line: core::ffi::c_uint,
        __function: *const core::ffi::c_char,
    ) -> !;
}
