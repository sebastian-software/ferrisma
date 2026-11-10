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

// Mechanically translated from upstream/aom/aom_dsp/sad.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C-unwind" {
    fn aom_comp_avg_pred_c(
        comp_pred: *mut uint8_t,
        pred: *const uint8_t,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
        ref_0: *const uint8_t,
        ref_stride: core::ffi::c_int,
    );
    fn aom_highbd_comp_avg_pred_c(
        comp_pred8: *mut uint8_t,
        pred8: *const uint8_t,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
        ref8: *const uint8_t,
        ref_stride: core::ffi::c_int,
    );
}
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uintptr_t = usize;
#[inline]
unsafe extern "C-unwind" fn sad(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
) -> core::ffi::c_uint {
    let mut y: core::ffi::c_int = 0;
    let mut x: core::ffi::c_int = 0;
    let mut sad_0: core::ffi::c_uint = 0 as core::ffi::c_uint;
    y = 0 as core::ffi::c_int;
    while y < height {
        x = 0 as core::ffi::c_int;
        while x < width {
            sad_0 = sad_0
                .wrapping_add(
                    abs(
                        *a.offset(x as isize) as core::ffi::c_int
                            - *b.offset(x as isize) as core::ffi::c_int,
                    ) as core::ffi::c_uint,
                );
            x += 1;
        }
        a = a.offset(a_stride as isize);
        b = b.offset(b_stride as isize);
        y += 1;
    }
    return sad_0;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad128x128_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return sad(
        src,
        src_stride,
        ref_0,
        ref_stride,
        128 as core::ffi::c_int,
        128 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad128x128_avg_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
) -> core::ffi::c_uint {
    let mut comp_pred: [uint8_t; 16384] = [0; 16384];
    aom_comp_avg_pred_c(
        comp_pred.as_mut_ptr(),
        second_pred,
        128 as core::ffi::c_int,
        128 as core::ffi::c_int,
        ref_0,
        ref_stride,
    );
    return sad(
        src,
        src_stride,
        comp_pred.as_mut_ptr(),
        128 as core::ffi::c_int,
        128 as core::ffi::c_int,
        128 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad_skip_128x128_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return (2 as core::ffi::c_uint)
        .wrapping_mul(
            sad(
                src,
                2 as core::ffi::c_int * src_stride,
                ref_0,
                2 as core::ffi::c_int * ref_stride,
                128 as core::ffi::c_int,
                128 as core::ffi::c_int / 2 as core::ffi::c_int,
            ),
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad_skip_128x128x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = (2 as core::ffi::c_uint)
            .wrapping_mul(
                sad(
                    src,
                    2 as core::ffi::c_int * src_stride,
                    *ref_array.offset(i as isize),
                    2 as core::ffi::c_int * ref_stride,
                    128 as core::ffi::c_int,
                    128 as core::ffi::c_int / 2 as core::ffi::c_int,
                ),
            ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad128x128x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = aom_sad128x128_c(
            src,
            src_stride,
            *ref_array.offset(i as isize),
            ref_stride,
        ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad128x128x3d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    aom_sad128x128x4d_c(src, src_stride, ref_array, ref_stride, sad_array);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad_skip_128x64_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return (2 as core::ffi::c_uint)
        .wrapping_mul(
            sad(
                src,
                2 as core::ffi::c_int * src_stride,
                ref_0,
                2 as core::ffi::c_int * ref_stride,
                128 as core::ffi::c_int,
                64 as core::ffi::c_int / 2 as core::ffi::c_int,
            ),
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad128x64_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return sad(
        src,
        src_stride,
        ref_0,
        ref_stride,
        128 as core::ffi::c_int,
        64 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad128x64_avg_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
) -> core::ffi::c_uint {
    let mut comp_pred: [uint8_t; 8192] = [0; 8192];
    aom_comp_avg_pred_c(
        comp_pred.as_mut_ptr(),
        second_pred,
        128 as core::ffi::c_int,
        64 as core::ffi::c_int,
        ref_0,
        ref_stride,
    );
    return sad(
        src,
        src_stride,
        comp_pred.as_mut_ptr(),
        128 as core::ffi::c_int,
        128 as core::ffi::c_int,
        64 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad128x64x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = aom_sad128x64_c(
            src,
            src_stride,
            *ref_array.offset(i as isize),
            ref_stride,
        ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad_skip_128x64x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = (2 as core::ffi::c_uint)
            .wrapping_mul(
                sad(
                    src,
                    2 as core::ffi::c_int * src_stride,
                    *ref_array.offset(i as isize),
                    2 as core::ffi::c_int * ref_stride,
                    128 as core::ffi::c_int,
                    64 as core::ffi::c_int / 2 as core::ffi::c_int,
                ),
            ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad128x64x3d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    aom_sad128x64x4d_c(src, src_stride, ref_array, ref_stride, sad_array);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad_skip_64x128_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return (2 as core::ffi::c_uint)
        .wrapping_mul(
            sad(
                src,
                2 as core::ffi::c_int * src_stride,
                ref_0,
                2 as core::ffi::c_int * ref_stride,
                64 as core::ffi::c_int,
                128 as core::ffi::c_int / 2 as core::ffi::c_int,
            ),
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad64x128_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return sad(
        src,
        src_stride,
        ref_0,
        ref_stride,
        64 as core::ffi::c_int,
        128 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad64x128_avg_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
) -> core::ffi::c_uint {
    let mut comp_pred: [uint8_t; 8192] = [0; 8192];
    aom_comp_avg_pred_c(
        comp_pred.as_mut_ptr(),
        second_pred,
        64 as core::ffi::c_int,
        128 as core::ffi::c_int,
        ref_0,
        ref_stride,
    );
    return sad(
        src,
        src_stride,
        comp_pred.as_mut_ptr(),
        64 as core::ffi::c_int,
        64 as core::ffi::c_int,
        128 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad_skip_64x128x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = (2 as core::ffi::c_uint)
            .wrapping_mul(
                sad(
                    src,
                    2 as core::ffi::c_int * src_stride,
                    *ref_array.offset(i as isize),
                    2 as core::ffi::c_int * ref_stride,
                    64 as core::ffi::c_int,
                    128 as core::ffi::c_int / 2 as core::ffi::c_int,
                ),
            ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad64x128x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = aom_sad64x128_c(
            src,
            src_stride,
            *ref_array.offset(i as isize),
            ref_stride,
        ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad64x128x3d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    aom_sad64x128x4d_c(src, src_stride, ref_array, ref_stride, sad_array);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad64x64_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return sad(
        src,
        src_stride,
        ref_0,
        ref_stride,
        64 as core::ffi::c_int,
        64 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad_skip_64x64_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return (2 as core::ffi::c_uint)
        .wrapping_mul(
            sad(
                src,
                2 as core::ffi::c_int * src_stride,
                ref_0,
                2 as core::ffi::c_int * ref_stride,
                64 as core::ffi::c_int,
                64 as core::ffi::c_int / 2 as core::ffi::c_int,
            ),
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad64x64_avg_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
) -> core::ffi::c_uint {
    let mut comp_pred: [uint8_t; 4096] = [0; 4096];
    aom_comp_avg_pred_c(
        comp_pred.as_mut_ptr(),
        second_pred,
        64 as core::ffi::c_int,
        64 as core::ffi::c_int,
        ref_0,
        ref_stride,
    );
    return sad(
        src,
        src_stride,
        comp_pred.as_mut_ptr(),
        64 as core::ffi::c_int,
        64 as core::ffi::c_int,
        64 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad_skip_64x64x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = (2 as core::ffi::c_uint)
            .wrapping_mul(
                sad(
                    src,
                    2 as core::ffi::c_int * src_stride,
                    *ref_array.offset(i as isize),
                    2 as core::ffi::c_int * ref_stride,
                    64 as core::ffi::c_int,
                    64 as core::ffi::c_int / 2 as core::ffi::c_int,
                ),
            ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad64x64x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = aom_sad64x64_c(
            src,
            src_stride,
            *ref_array.offset(i as isize),
            ref_stride,
        ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad64x64x3d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    aom_sad64x64x4d_c(src, src_stride, ref_array, ref_stride, sad_array);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad_skip_64x32_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return (2 as core::ffi::c_uint)
        .wrapping_mul(
            sad(
                src,
                2 as core::ffi::c_int * src_stride,
                ref_0,
                2 as core::ffi::c_int * ref_stride,
                64 as core::ffi::c_int,
                32 as core::ffi::c_int / 2 as core::ffi::c_int,
            ),
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad64x32_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return sad(
        src,
        src_stride,
        ref_0,
        ref_stride,
        64 as core::ffi::c_int,
        32 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad64x32_avg_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
) -> core::ffi::c_uint {
    let mut comp_pred: [uint8_t; 2048] = [0; 2048];
    aom_comp_avg_pred_c(
        comp_pred.as_mut_ptr(),
        second_pred,
        64 as core::ffi::c_int,
        32 as core::ffi::c_int,
        ref_0,
        ref_stride,
    );
    return sad(
        src,
        src_stride,
        comp_pred.as_mut_ptr(),
        64 as core::ffi::c_int,
        64 as core::ffi::c_int,
        32 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad_skip_64x32x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = (2 as core::ffi::c_uint)
            .wrapping_mul(
                sad(
                    src,
                    2 as core::ffi::c_int * src_stride,
                    *ref_array.offset(i as isize),
                    2 as core::ffi::c_int * ref_stride,
                    64 as core::ffi::c_int,
                    32 as core::ffi::c_int / 2 as core::ffi::c_int,
                ),
            ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad64x32x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = aom_sad64x32_c(
            src,
            src_stride,
            *ref_array.offset(i as isize),
            ref_stride,
        ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad64x32x3d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    aom_sad64x32x4d_c(src, src_stride, ref_array, ref_stride, sad_array);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad32x64_avg_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
) -> core::ffi::c_uint {
    let mut comp_pred: [uint8_t; 2048] = [0; 2048];
    aom_comp_avg_pred_c(
        comp_pred.as_mut_ptr(),
        second_pred,
        32 as core::ffi::c_int,
        64 as core::ffi::c_int,
        ref_0,
        ref_stride,
    );
    return sad(
        src,
        src_stride,
        comp_pred.as_mut_ptr(),
        32 as core::ffi::c_int,
        32 as core::ffi::c_int,
        64 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad32x64_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return sad(
        src,
        src_stride,
        ref_0,
        ref_stride,
        32 as core::ffi::c_int,
        64 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad_skip_32x64_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return (2 as core::ffi::c_uint)
        .wrapping_mul(
            sad(
                src,
                2 as core::ffi::c_int * src_stride,
                ref_0,
                2 as core::ffi::c_int * ref_stride,
                32 as core::ffi::c_int,
                64 as core::ffi::c_int / 2 as core::ffi::c_int,
            ),
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad32x64x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = aom_sad32x64_c(
            src,
            src_stride,
            *ref_array.offset(i as isize),
            ref_stride,
        ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad_skip_32x64x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = (2 as core::ffi::c_uint)
            .wrapping_mul(
                sad(
                    src,
                    2 as core::ffi::c_int * src_stride,
                    *ref_array.offset(i as isize),
                    2 as core::ffi::c_int * ref_stride,
                    32 as core::ffi::c_int,
                    64 as core::ffi::c_int / 2 as core::ffi::c_int,
                ),
            ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad32x64x3d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    aom_sad32x64x4d_c(src, src_stride, ref_array, ref_stride, sad_array);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad_skip_32x32_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return (2 as core::ffi::c_uint)
        .wrapping_mul(
            sad(
                src,
                2 as core::ffi::c_int * src_stride,
                ref_0,
                2 as core::ffi::c_int * ref_stride,
                32 as core::ffi::c_int,
                32 as core::ffi::c_int / 2 as core::ffi::c_int,
            ),
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad32x32_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return sad(
        src,
        src_stride,
        ref_0,
        ref_stride,
        32 as core::ffi::c_int,
        32 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad32x32_avg_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
) -> core::ffi::c_uint {
    let mut comp_pred: [uint8_t; 1024] = [0; 1024];
    aom_comp_avg_pred_c(
        comp_pred.as_mut_ptr(),
        second_pred,
        32 as core::ffi::c_int,
        32 as core::ffi::c_int,
        ref_0,
        ref_stride,
    );
    return sad(
        src,
        src_stride,
        comp_pred.as_mut_ptr(),
        32 as core::ffi::c_int,
        32 as core::ffi::c_int,
        32 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad_skip_32x32x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = (2 as core::ffi::c_uint)
            .wrapping_mul(
                sad(
                    src,
                    2 as core::ffi::c_int * src_stride,
                    *ref_array.offset(i as isize),
                    2 as core::ffi::c_int * ref_stride,
                    32 as core::ffi::c_int,
                    32 as core::ffi::c_int / 2 as core::ffi::c_int,
                ),
            ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad32x32x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = aom_sad32x32_c(
            src,
            src_stride,
            *ref_array.offset(i as isize),
            ref_stride,
        ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad32x32x3d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    aom_sad32x32x4d_c(src, src_stride, ref_array, ref_stride, sad_array);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad_skip_32x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return (2 as core::ffi::c_uint)
        .wrapping_mul(
            sad(
                src,
                2 as core::ffi::c_int * src_stride,
                ref_0,
                2 as core::ffi::c_int * ref_stride,
                32 as core::ffi::c_int,
                16 as core::ffi::c_int / 2 as core::ffi::c_int,
            ),
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad32x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return sad(
        src,
        src_stride,
        ref_0,
        ref_stride,
        32 as core::ffi::c_int,
        16 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad32x16_avg_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
) -> core::ffi::c_uint {
    let mut comp_pred: [uint8_t; 512] = [0; 512];
    aom_comp_avg_pred_c(
        comp_pred.as_mut_ptr(),
        second_pred,
        32 as core::ffi::c_int,
        16 as core::ffi::c_int,
        ref_0,
        ref_stride,
    );
    return sad(
        src,
        src_stride,
        comp_pred.as_mut_ptr(),
        32 as core::ffi::c_int,
        32 as core::ffi::c_int,
        16 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad_skip_32x16x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = (2 as core::ffi::c_uint)
            .wrapping_mul(
                sad(
                    src,
                    2 as core::ffi::c_int * src_stride,
                    *ref_array.offset(i as isize),
                    2 as core::ffi::c_int * ref_stride,
                    32 as core::ffi::c_int,
                    16 as core::ffi::c_int / 2 as core::ffi::c_int,
                ),
            ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad32x16x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = aom_sad32x16_c(
            src,
            src_stride,
            *ref_array.offset(i as isize),
            ref_stride,
        ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad32x16x3d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    aom_sad32x16x4d_c(src, src_stride, ref_array, ref_stride, sad_array);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad16x32_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return sad(
        src,
        src_stride,
        ref_0,
        ref_stride,
        16 as core::ffi::c_int,
        32 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad_skip_16x32_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return (2 as core::ffi::c_uint)
        .wrapping_mul(
            sad(
                src,
                2 as core::ffi::c_int * src_stride,
                ref_0,
                2 as core::ffi::c_int * ref_stride,
                16 as core::ffi::c_int,
                32 as core::ffi::c_int / 2 as core::ffi::c_int,
            ),
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad16x32_avg_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
) -> core::ffi::c_uint {
    let mut comp_pred: [uint8_t; 512] = [0; 512];
    aom_comp_avg_pred_c(
        comp_pred.as_mut_ptr(),
        second_pred,
        16 as core::ffi::c_int,
        32 as core::ffi::c_int,
        ref_0,
        ref_stride,
    );
    return sad(
        src,
        src_stride,
        comp_pred.as_mut_ptr(),
        16 as core::ffi::c_int,
        16 as core::ffi::c_int,
        32 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad16x32x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = aom_sad16x32_c(
            src,
            src_stride,
            *ref_array.offset(i as isize),
            ref_stride,
        ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad_skip_16x32x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = (2 as core::ffi::c_uint)
            .wrapping_mul(
                sad(
                    src,
                    2 as core::ffi::c_int * src_stride,
                    *ref_array.offset(i as isize),
                    2 as core::ffi::c_int * ref_stride,
                    16 as core::ffi::c_int,
                    32 as core::ffi::c_int / 2 as core::ffi::c_int,
                ),
            ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad16x32x3d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    aom_sad16x32x4d_c(src, src_stride, ref_array, ref_stride, sad_array);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad16x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return sad(
        src,
        src_stride,
        ref_0,
        ref_stride,
        16 as core::ffi::c_int,
        16 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad16x16_avg_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
) -> core::ffi::c_uint {
    let mut comp_pred: [uint8_t; 256] = [0; 256];
    aom_comp_avg_pred_c(
        comp_pred.as_mut_ptr(),
        second_pred,
        16 as core::ffi::c_int,
        16 as core::ffi::c_int,
        ref_0,
        ref_stride,
    );
    return sad(
        src,
        src_stride,
        comp_pred.as_mut_ptr(),
        16 as core::ffi::c_int,
        16 as core::ffi::c_int,
        16 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad_skip_16x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return (2 as core::ffi::c_uint)
        .wrapping_mul(
            sad(
                src,
                2 as core::ffi::c_int * src_stride,
                ref_0,
                2 as core::ffi::c_int * ref_stride,
                16 as core::ffi::c_int,
                16 as core::ffi::c_int / 2 as core::ffi::c_int,
            ),
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad16x16x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = aom_sad16x16_c(
            src,
            src_stride,
            *ref_array.offset(i as isize),
            ref_stride,
        ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad_skip_16x16x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = (2 as core::ffi::c_uint)
            .wrapping_mul(
                sad(
                    src,
                    2 as core::ffi::c_int * src_stride,
                    *ref_array.offset(i as isize),
                    2 as core::ffi::c_int * ref_stride,
                    16 as core::ffi::c_int,
                    16 as core::ffi::c_int / 2 as core::ffi::c_int,
                ),
            ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad16x16x3d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    aom_sad16x16x4d_c(src, src_stride, ref_array, ref_stride, sad_array);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad16x8_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return sad(
        src,
        src_stride,
        ref_0,
        ref_stride,
        16 as core::ffi::c_int,
        8 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad16x8_avg_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
) -> core::ffi::c_uint {
    let mut comp_pred: [uint8_t; 128] = [0; 128];
    aom_comp_avg_pred_c(
        comp_pred.as_mut_ptr(),
        second_pred,
        16 as core::ffi::c_int,
        8 as core::ffi::c_int,
        ref_0,
        ref_stride,
    );
    return sad(
        src,
        src_stride,
        comp_pred.as_mut_ptr(),
        16 as core::ffi::c_int,
        16 as core::ffi::c_int,
        8 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad16x8x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = aom_sad16x8_c(
            src,
            src_stride,
            *ref_array.offset(i as isize),
            ref_stride,
        ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad16x8x3d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    aom_sad16x8x4d_c(src, src_stride, ref_array, ref_stride, sad_array);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad_skip_8x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return (2 as core::ffi::c_uint)
        .wrapping_mul(
            sad(
                src,
                2 as core::ffi::c_int * src_stride,
                ref_0,
                2 as core::ffi::c_int * ref_stride,
                8 as core::ffi::c_int,
                16 as core::ffi::c_int / 2 as core::ffi::c_int,
            ),
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad8x16_avg_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
) -> core::ffi::c_uint {
    let mut comp_pred: [uint8_t; 128] = [0; 128];
    aom_comp_avg_pred_c(
        comp_pred.as_mut_ptr(),
        second_pred,
        8 as core::ffi::c_int,
        16 as core::ffi::c_int,
        ref_0,
        ref_stride,
    );
    return sad(
        src,
        src_stride,
        comp_pred.as_mut_ptr(),
        8 as core::ffi::c_int,
        8 as core::ffi::c_int,
        16 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad8x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return sad(
        src,
        src_stride,
        ref_0,
        ref_stride,
        8 as core::ffi::c_int,
        16 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad8x16x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = aom_sad8x16_c(
            src,
            src_stride,
            *ref_array.offset(i as isize),
            ref_stride,
        ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad_skip_8x16x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = (2 as core::ffi::c_uint)
            .wrapping_mul(
                sad(
                    src,
                    2 as core::ffi::c_int * src_stride,
                    *ref_array.offset(i as isize),
                    2 as core::ffi::c_int * ref_stride,
                    8 as core::ffi::c_int,
                    16 as core::ffi::c_int / 2 as core::ffi::c_int,
                ),
            ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad8x16x3d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    aom_sad8x16x4d_c(src, src_stride, ref_array, ref_stride, sad_array);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad8x8_avg_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
) -> core::ffi::c_uint {
    let mut comp_pred: [uint8_t; 64] = [0; 64];
    aom_comp_avg_pred_c(
        comp_pred.as_mut_ptr(),
        second_pred,
        8 as core::ffi::c_int,
        8 as core::ffi::c_int,
        ref_0,
        ref_stride,
    );
    return sad(
        src,
        src_stride,
        comp_pred.as_mut_ptr(),
        8 as core::ffi::c_int,
        8 as core::ffi::c_int,
        8 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad8x8_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return sad(
        src,
        src_stride,
        ref_0,
        ref_stride,
        8 as core::ffi::c_int,
        8 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad8x8x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = aom_sad8x8_c(
            src,
            src_stride,
            *ref_array.offset(i as isize),
            ref_stride,
        ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad8x8x3d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    aom_sad8x8x4d_c(src, src_stride, ref_array, ref_stride, sad_array);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad8x4_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return sad(
        src,
        src_stride,
        ref_0,
        ref_stride,
        8 as core::ffi::c_int,
        4 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad8x4x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = aom_sad8x4_c(
            src,
            src_stride,
            *ref_array.offset(i as isize),
            ref_stride,
        ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad8x4x3d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    aom_sad8x4x4d_c(src, src_stride, ref_array, ref_stride, sad_array);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad4x8_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return sad(
        src,
        src_stride,
        ref_0,
        ref_stride,
        4 as core::ffi::c_int,
        8 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad4x8x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = aom_sad4x8_c(
            src,
            src_stride,
            *ref_array.offset(i as isize),
            ref_stride,
        ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad4x8x3d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    aom_sad4x8x4d_c(src, src_stride, ref_array, ref_stride, sad_array);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad4x4_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return sad(
        src,
        src_stride,
        ref_0,
        ref_stride,
        4 as core::ffi::c_int,
        4 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad4x4x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = aom_sad4x4_c(
            src,
            src_stride,
            *ref_array.offset(i as isize),
            ref_stride,
        ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad4x4x3d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    aom_sad4x4x4d_c(src, src_stride, ref_array, ref_stride, sad_array);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad4x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return sad(
        src,
        src_stride,
        ref_0,
        ref_stride,
        4 as core::ffi::c_int,
        16 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad_skip_4x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return (2 as core::ffi::c_uint)
        .wrapping_mul(
            sad(
                src,
                2 as core::ffi::c_int * src_stride,
                ref_0,
                2 as core::ffi::c_int * ref_stride,
                4 as core::ffi::c_int,
                16 as core::ffi::c_int / 2 as core::ffi::c_int,
            ),
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad_skip_4x16x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = (2 as core::ffi::c_uint)
            .wrapping_mul(
                sad(
                    src,
                    2 as core::ffi::c_int * src_stride,
                    *ref_array.offset(i as isize),
                    2 as core::ffi::c_int * ref_stride,
                    4 as core::ffi::c_int,
                    16 as core::ffi::c_int / 2 as core::ffi::c_int,
                ),
            ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad4x16x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = aom_sad4x16_c(
            src,
            src_stride,
            *ref_array.offset(i as isize),
            ref_stride,
        ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad16x4_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return sad(
        src,
        src_stride,
        ref_0,
        ref_stride,
        16 as core::ffi::c_int,
        4 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad16x4x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = aom_sad16x4_c(
            src,
            src_stride,
            *ref_array.offset(i as isize),
            ref_stride,
        ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad_skip_8x32_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return (2 as core::ffi::c_uint)
        .wrapping_mul(
            sad(
                src,
                2 as core::ffi::c_int * src_stride,
                ref_0,
                2 as core::ffi::c_int * ref_stride,
                8 as core::ffi::c_int,
                32 as core::ffi::c_int / 2 as core::ffi::c_int,
            ),
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad8x32_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return sad(
        src,
        src_stride,
        ref_0,
        ref_stride,
        8 as core::ffi::c_int,
        32 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad8x32_avg_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
) -> core::ffi::c_uint {
    let mut comp_pred: [uint8_t; 256] = [0; 256];
    aom_comp_avg_pred_c(
        comp_pred.as_mut_ptr(),
        second_pred,
        8 as core::ffi::c_int,
        32 as core::ffi::c_int,
        ref_0,
        ref_stride,
    );
    return sad(
        src,
        src_stride,
        comp_pred.as_mut_ptr(),
        8 as core::ffi::c_int,
        8 as core::ffi::c_int,
        32 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad_skip_8x32x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = (2 as core::ffi::c_uint)
            .wrapping_mul(
                sad(
                    src,
                    2 as core::ffi::c_int * src_stride,
                    *ref_array.offset(i as isize),
                    2 as core::ffi::c_int * ref_stride,
                    8 as core::ffi::c_int,
                    32 as core::ffi::c_int / 2 as core::ffi::c_int,
                ),
            ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad8x32x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = aom_sad8x32_c(
            src,
            src_stride,
            *ref_array.offset(i as isize),
            ref_stride,
        ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad32x8_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return sad(
        src,
        src_stride,
        ref_0,
        ref_stride,
        32 as core::ffi::c_int,
        8 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad32x8_avg_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
) -> core::ffi::c_uint {
    let mut comp_pred: [uint8_t; 256] = [0; 256];
    aom_comp_avg_pred_c(
        comp_pred.as_mut_ptr(),
        second_pred,
        32 as core::ffi::c_int,
        8 as core::ffi::c_int,
        ref_0,
        ref_stride,
    );
    return sad(
        src,
        src_stride,
        comp_pred.as_mut_ptr(),
        32 as core::ffi::c_int,
        32 as core::ffi::c_int,
        8 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad32x8x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = aom_sad32x8_c(
            src,
            src_stride,
            *ref_array.offset(i as isize),
            ref_stride,
        ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad_skip_16x64_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return (2 as core::ffi::c_uint)
        .wrapping_mul(
            sad(
                src,
                2 as core::ffi::c_int * src_stride,
                ref_0,
                2 as core::ffi::c_int * ref_stride,
                16 as core::ffi::c_int,
                64 as core::ffi::c_int / 2 as core::ffi::c_int,
            ),
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad16x64_avg_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
) -> core::ffi::c_uint {
    let mut comp_pred: [uint8_t; 1024] = [0; 1024];
    aom_comp_avg_pred_c(
        comp_pred.as_mut_ptr(),
        second_pred,
        16 as core::ffi::c_int,
        64 as core::ffi::c_int,
        ref_0,
        ref_stride,
    );
    return sad(
        src,
        src_stride,
        comp_pred.as_mut_ptr(),
        16 as core::ffi::c_int,
        16 as core::ffi::c_int,
        64 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad16x64_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return sad(
        src,
        src_stride,
        ref_0,
        ref_stride,
        16 as core::ffi::c_int,
        64 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad_skip_16x64x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = (2 as core::ffi::c_uint)
            .wrapping_mul(
                sad(
                    src,
                    2 as core::ffi::c_int * src_stride,
                    *ref_array.offset(i as isize),
                    2 as core::ffi::c_int * ref_stride,
                    16 as core::ffi::c_int,
                    64 as core::ffi::c_int / 2 as core::ffi::c_int,
                ),
            ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad16x64x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = aom_sad16x64_c(
            src,
            src_stride,
            *ref_array.offset(i as isize),
            ref_stride,
        ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad64x16_avg_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
) -> core::ffi::c_uint {
    let mut comp_pred: [uint8_t; 1024] = [0; 1024];
    aom_comp_avg_pred_c(
        comp_pred.as_mut_ptr(),
        second_pred,
        64 as core::ffi::c_int,
        16 as core::ffi::c_int,
        ref_0,
        ref_stride,
    );
    return sad(
        src,
        src_stride,
        comp_pred.as_mut_ptr(),
        64 as core::ffi::c_int,
        64 as core::ffi::c_int,
        16 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad_skip_64x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return (2 as core::ffi::c_uint)
        .wrapping_mul(
            sad(
                src,
                2 as core::ffi::c_int * src_stride,
                ref_0,
                2 as core::ffi::c_int * ref_stride,
                64 as core::ffi::c_int,
                16 as core::ffi::c_int / 2 as core::ffi::c_int,
            ),
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad64x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return sad(
        src,
        src_stride,
        ref_0,
        ref_stride,
        64 as core::ffi::c_int,
        16 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad_skip_64x16x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = (2 as core::ffi::c_uint)
            .wrapping_mul(
                sad(
                    src,
                    2 as core::ffi::c_int * src_stride,
                    *ref_array.offset(i as isize),
                    2 as core::ffi::c_int * ref_stride,
                    64 as core::ffi::c_int,
                    16 as core::ffi::c_int / 2 as core::ffi::c_int,
                ),
            ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad64x16x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = aom_sad64x16_c(
            src,
            src_stride,
            *ref_array.offset(i as isize),
            ref_stride,
        ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad4x16x3d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    aom_sad4x16x4d_c(src, src_stride, ref_array, ref_stride, sad_array);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad16x4x3d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    aom_sad16x4x4d_c(src, src_stride, ref_array, ref_stride, sad_array);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad8x32x3d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    aom_sad8x32x4d_c(src, src_stride, ref_array, ref_stride, sad_array);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad32x8x3d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    aom_sad32x8x4d_c(src, src_stride, ref_array, ref_stride, sad_array);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad16x64x3d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    aom_sad16x64x4d_c(src, src_stride, ref_array, ref_stride, sad_array);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sad64x16x3d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    aom_sad64x16x4d_c(src, src_stride, ref_array, ref_stride, sad_array);
}
#[inline]
unsafe extern "C-unwind" fn highbd_sad(
    mut a8: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b8: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
) -> core::ffi::c_uint {
    let mut y: core::ffi::c_int = 0;
    let mut x: core::ffi::c_int = 0;
    let mut sad_0: core::ffi::c_uint = 0 as core::ffi::c_uint;
    let mut a: *const uint16_t = ((a8 as uintptr_t) << 1 as core::ffi::c_int)
        as *mut uint16_t;
    let mut b: *const uint16_t = ((b8 as uintptr_t) << 1 as core::ffi::c_int)
        as *mut uint16_t;
    y = 0 as core::ffi::c_int;
    while y < height {
        x = 0 as core::ffi::c_int;
        while x < width {
            sad_0 = sad_0
                .wrapping_add(
                    abs(
                        *a.offset(x as isize) as core::ffi::c_int
                            - *b.offset(x as isize) as core::ffi::c_int,
                    ) as core::ffi::c_uint,
                );
            x += 1;
        }
        a = a.offset(a_stride as isize);
        b = b.offset(b_stride as isize);
        y += 1;
    }
    return sad_0;
}
#[inline]
unsafe extern "C-unwind" fn highbd_sadb(
    mut a8: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b8: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
) -> core::ffi::c_uint {
    let mut y: core::ffi::c_int = 0;
    let mut x: core::ffi::c_int = 0;
    let mut sad_0: core::ffi::c_uint = 0 as core::ffi::c_uint;
    let mut a: *const uint16_t = ((a8 as uintptr_t) << 1 as core::ffi::c_int)
        as *mut uint16_t;
    let mut b: *const uint16_t = ((b8 as uintptr_t) << 1 as core::ffi::c_int)
        as *mut uint16_t;
    y = 0 as core::ffi::c_int;
    while y < height {
        x = 0 as core::ffi::c_int;
        while x < width {
            sad_0 = sad_0
                .wrapping_add(
                    abs(
                        *a.offset(x as isize) as core::ffi::c_int
                            - *b.offset(x as isize) as core::ffi::c_int,
                    ) as core::ffi::c_uint,
                );
            x += 1;
        }
        a = a.offset(a_stride as isize);
        b = b.offset(b_stride as isize);
        y += 1;
    }
    return sad_0;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad_skip_128x128_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return (2 as core::ffi::c_uint)
        .wrapping_mul(
            highbd_sad(
                src,
                2 as core::ffi::c_int * src_stride,
                ref_0,
                2 as core::ffi::c_int * ref_stride,
                128 as core::ffi::c_int,
                128 as core::ffi::c_int / 2 as core::ffi::c_int,
            ),
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad128x128_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return highbd_sad(
        src,
        src_stride,
        ref_0,
        ref_stride,
        128 as core::ffi::c_int,
        128 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad128x128_avg_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
) -> core::ffi::c_uint {
    let mut comp_pred: [uint16_t; 16384] = [0; 16384];
    let comp_pred8: *mut uint8_t = (comp_pred.as_mut_ptr() as uintptr_t
        >> 1 as core::ffi::c_int) as *mut uint8_t;
    aom_highbd_comp_avg_pred_c(
        comp_pred8,
        second_pred,
        128 as core::ffi::c_int,
        128 as core::ffi::c_int,
        ref_0,
        ref_stride,
    );
    return highbd_sadb(
        src,
        src_stride,
        comp_pred8,
        128 as core::ffi::c_int,
        128 as core::ffi::c_int,
        128 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad128x128x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = aom_highbd_sad128x128_c(
            src,
            src_stride,
            *ref_array.offset(i as isize),
            ref_stride,
        ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad_skip_128x128x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = (2 as core::ffi::c_uint)
            .wrapping_mul(
                highbd_sad(
                    src,
                    2 as core::ffi::c_int * src_stride,
                    *ref_array.offset(i as isize),
                    2 as core::ffi::c_int * ref_stride,
                    128 as core::ffi::c_int,
                    128 as core::ffi::c_int / 2 as core::ffi::c_int,
                ),
            ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad128x128x3d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    aom_highbd_sad128x128x4d_c(src, src_stride, ref_array, ref_stride, sad_array);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad128x64_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return highbd_sad(
        src,
        src_stride,
        ref_0,
        ref_stride,
        128 as core::ffi::c_int,
        64 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad128x64_avg_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
) -> core::ffi::c_uint {
    let mut comp_pred: [uint16_t; 8192] = [0; 8192];
    let comp_pred8: *mut uint8_t = (comp_pred.as_mut_ptr() as uintptr_t
        >> 1 as core::ffi::c_int) as *mut uint8_t;
    aom_highbd_comp_avg_pred_c(
        comp_pred8,
        second_pred,
        128 as core::ffi::c_int,
        64 as core::ffi::c_int,
        ref_0,
        ref_stride,
    );
    return highbd_sadb(
        src,
        src_stride,
        comp_pred8,
        128 as core::ffi::c_int,
        128 as core::ffi::c_int,
        64 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad_skip_128x64_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return (2 as core::ffi::c_uint)
        .wrapping_mul(
            highbd_sad(
                src,
                2 as core::ffi::c_int * src_stride,
                ref_0,
                2 as core::ffi::c_int * ref_stride,
                128 as core::ffi::c_int,
                64 as core::ffi::c_int / 2 as core::ffi::c_int,
            ),
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad128x64x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = aom_highbd_sad128x64_c(
            src,
            src_stride,
            *ref_array.offset(i as isize),
            ref_stride,
        ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad_skip_128x64x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = (2 as core::ffi::c_uint)
            .wrapping_mul(
                highbd_sad(
                    src,
                    2 as core::ffi::c_int * src_stride,
                    *ref_array.offset(i as isize),
                    2 as core::ffi::c_int * ref_stride,
                    128 as core::ffi::c_int,
                    64 as core::ffi::c_int / 2 as core::ffi::c_int,
                ),
            ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad128x64x3d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    aom_highbd_sad128x64x4d_c(src, src_stride, ref_array, ref_stride, sad_array);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad64x128_avg_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
) -> core::ffi::c_uint {
    let mut comp_pred: [uint16_t; 8192] = [0; 8192];
    let comp_pred8: *mut uint8_t = (comp_pred.as_mut_ptr() as uintptr_t
        >> 1 as core::ffi::c_int) as *mut uint8_t;
    aom_highbd_comp_avg_pred_c(
        comp_pred8,
        second_pred,
        64 as core::ffi::c_int,
        128 as core::ffi::c_int,
        ref_0,
        ref_stride,
    );
    return highbd_sadb(
        src,
        src_stride,
        comp_pred8,
        64 as core::ffi::c_int,
        64 as core::ffi::c_int,
        128 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad64x128_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return highbd_sad(
        src,
        src_stride,
        ref_0,
        ref_stride,
        64 as core::ffi::c_int,
        128 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad_skip_64x128_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return (2 as core::ffi::c_uint)
        .wrapping_mul(
            highbd_sad(
                src,
                2 as core::ffi::c_int * src_stride,
                ref_0,
                2 as core::ffi::c_int * ref_stride,
                64 as core::ffi::c_int,
                128 as core::ffi::c_int / 2 as core::ffi::c_int,
            ),
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad64x128x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = aom_highbd_sad64x128_c(
            src,
            src_stride,
            *ref_array.offset(i as isize),
            ref_stride,
        ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad_skip_64x128x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = (2 as core::ffi::c_uint)
            .wrapping_mul(
                highbd_sad(
                    src,
                    2 as core::ffi::c_int * src_stride,
                    *ref_array.offset(i as isize),
                    2 as core::ffi::c_int * ref_stride,
                    64 as core::ffi::c_int,
                    128 as core::ffi::c_int / 2 as core::ffi::c_int,
                ),
            ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad64x128x3d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    aom_highbd_sad64x128x4d_c(src, src_stride, ref_array, ref_stride, sad_array);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad_skip_64x64_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return (2 as core::ffi::c_uint)
        .wrapping_mul(
            highbd_sad(
                src,
                2 as core::ffi::c_int * src_stride,
                ref_0,
                2 as core::ffi::c_int * ref_stride,
                64 as core::ffi::c_int,
                64 as core::ffi::c_int / 2 as core::ffi::c_int,
            ),
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad64x64_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return highbd_sad(
        src,
        src_stride,
        ref_0,
        ref_stride,
        64 as core::ffi::c_int,
        64 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad64x64_avg_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
) -> core::ffi::c_uint {
    let mut comp_pred: [uint16_t; 4096] = [0; 4096];
    let comp_pred8: *mut uint8_t = (comp_pred.as_mut_ptr() as uintptr_t
        >> 1 as core::ffi::c_int) as *mut uint8_t;
    aom_highbd_comp_avg_pred_c(
        comp_pred8,
        second_pred,
        64 as core::ffi::c_int,
        64 as core::ffi::c_int,
        ref_0,
        ref_stride,
    );
    return highbd_sadb(
        src,
        src_stride,
        comp_pred8,
        64 as core::ffi::c_int,
        64 as core::ffi::c_int,
        64 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad64x64x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = aom_highbd_sad64x64_c(
            src,
            src_stride,
            *ref_array.offset(i as isize),
            ref_stride,
        ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad_skip_64x64x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = (2 as core::ffi::c_uint)
            .wrapping_mul(
                highbd_sad(
                    src,
                    2 as core::ffi::c_int * src_stride,
                    *ref_array.offset(i as isize),
                    2 as core::ffi::c_int * ref_stride,
                    64 as core::ffi::c_int,
                    64 as core::ffi::c_int / 2 as core::ffi::c_int,
                ),
            ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad64x64x3d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    aom_highbd_sad64x64x4d_c(src, src_stride, ref_array, ref_stride, sad_array);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad_skip_64x32_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return (2 as core::ffi::c_uint)
        .wrapping_mul(
            highbd_sad(
                src,
                2 as core::ffi::c_int * src_stride,
                ref_0,
                2 as core::ffi::c_int * ref_stride,
                64 as core::ffi::c_int,
                32 as core::ffi::c_int / 2 as core::ffi::c_int,
            ),
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad64x32_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return highbd_sad(
        src,
        src_stride,
        ref_0,
        ref_stride,
        64 as core::ffi::c_int,
        32 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad64x32_avg_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
) -> core::ffi::c_uint {
    let mut comp_pred: [uint16_t; 2048] = [0; 2048];
    let comp_pred8: *mut uint8_t = (comp_pred.as_mut_ptr() as uintptr_t
        >> 1 as core::ffi::c_int) as *mut uint8_t;
    aom_highbd_comp_avg_pred_c(
        comp_pred8,
        second_pred,
        64 as core::ffi::c_int,
        32 as core::ffi::c_int,
        ref_0,
        ref_stride,
    );
    return highbd_sadb(
        src,
        src_stride,
        comp_pred8,
        64 as core::ffi::c_int,
        64 as core::ffi::c_int,
        32 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad_skip_64x32x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = (2 as core::ffi::c_uint)
            .wrapping_mul(
                highbd_sad(
                    src,
                    2 as core::ffi::c_int * src_stride,
                    *ref_array.offset(i as isize),
                    2 as core::ffi::c_int * ref_stride,
                    64 as core::ffi::c_int,
                    32 as core::ffi::c_int / 2 as core::ffi::c_int,
                ),
            ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad64x32x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = aom_highbd_sad64x32_c(
            src,
            src_stride,
            *ref_array.offset(i as isize),
            ref_stride,
        ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad64x32x3d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    aom_highbd_sad64x32x4d_c(src, src_stride, ref_array, ref_stride, sad_array);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad32x64_avg_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
) -> core::ffi::c_uint {
    let mut comp_pred: [uint16_t; 2048] = [0; 2048];
    let comp_pred8: *mut uint8_t = (comp_pred.as_mut_ptr() as uintptr_t
        >> 1 as core::ffi::c_int) as *mut uint8_t;
    aom_highbd_comp_avg_pred_c(
        comp_pred8,
        second_pred,
        32 as core::ffi::c_int,
        64 as core::ffi::c_int,
        ref_0,
        ref_stride,
    );
    return highbd_sadb(
        src,
        src_stride,
        comp_pred8,
        32 as core::ffi::c_int,
        32 as core::ffi::c_int,
        64 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad_skip_32x64_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return (2 as core::ffi::c_uint)
        .wrapping_mul(
            highbd_sad(
                src,
                2 as core::ffi::c_int * src_stride,
                ref_0,
                2 as core::ffi::c_int * ref_stride,
                32 as core::ffi::c_int,
                64 as core::ffi::c_int / 2 as core::ffi::c_int,
            ),
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad32x64_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return highbd_sad(
        src,
        src_stride,
        ref_0,
        ref_stride,
        32 as core::ffi::c_int,
        64 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad32x64x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = aom_highbd_sad32x64_c(
            src,
            src_stride,
            *ref_array.offset(i as isize),
            ref_stride,
        ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad_skip_32x64x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = (2 as core::ffi::c_uint)
            .wrapping_mul(
                highbd_sad(
                    src,
                    2 as core::ffi::c_int * src_stride,
                    *ref_array.offset(i as isize),
                    2 as core::ffi::c_int * ref_stride,
                    32 as core::ffi::c_int,
                    64 as core::ffi::c_int / 2 as core::ffi::c_int,
                ),
            ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad32x64x3d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    aom_highbd_sad32x64x4d_c(src, src_stride, ref_array, ref_stride, sad_array);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad32x32_avg_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
) -> core::ffi::c_uint {
    let mut comp_pred: [uint16_t; 1024] = [0; 1024];
    let comp_pred8: *mut uint8_t = (comp_pred.as_mut_ptr() as uintptr_t
        >> 1 as core::ffi::c_int) as *mut uint8_t;
    aom_highbd_comp_avg_pred_c(
        comp_pred8,
        second_pred,
        32 as core::ffi::c_int,
        32 as core::ffi::c_int,
        ref_0,
        ref_stride,
    );
    return highbd_sadb(
        src,
        src_stride,
        comp_pred8,
        32 as core::ffi::c_int,
        32 as core::ffi::c_int,
        32 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad32x32_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return highbd_sad(
        src,
        src_stride,
        ref_0,
        ref_stride,
        32 as core::ffi::c_int,
        32 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad_skip_32x32_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return (2 as core::ffi::c_uint)
        .wrapping_mul(
            highbd_sad(
                src,
                2 as core::ffi::c_int * src_stride,
                ref_0,
                2 as core::ffi::c_int * ref_stride,
                32 as core::ffi::c_int,
                32 as core::ffi::c_int / 2 as core::ffi::c_int,
            ),
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad32x32x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = aom_highbd_sad32x32_c(
            src,
            src_stride,
            *ref_array.offset(i as isize),
            ref_stride,
        ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad_skip_32x32x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = (2 as core::ffi::c_uint)
            .wrapping_mul(
                highbd_sad(
                    src,
                    2 as core::ffi::c_int * src_stride,
                    *ref_array.offset(i as isize),
                    2 as core::ffi::c_int * ref_stride,
                    32 as core::ffi::c_int,
                    32 as core::ffi::c_int / 2 as core::ffi::c_int,
                ),
            ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad32x32x3d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    aom_highbd_sad32x32x4d_c(src, src_stride, ref_array, ref_stride, sad_array);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad_skip_32x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return (2 as core::ffi::c_uint)
        .wrapping_mul(
            highbd_sad(
                src,
                2 as core::ffi::c_int * src_stride,
                ref_0,
                2 as core::ffi::c_int * ref_stride,
                32 as core::ffi::c_int,
                16 as core::ffi::c_int / 2 as core::ffi::c_int,
            ),
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad32x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return highbd_sad(
        src,
        src_stride,
        ref_0,
        ref_stride,
        32 as core::ffi::c_int,
        16 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad32x16_avg_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
) -> core::ffi::c_uint {
    let mut comp_pred: [uint16_t; 512] = [0; 512];
    let comp_pred8: *mut uint8_t = (comp_pred.as_mut_ptr() as uintptr_t
        >> 1 as core::ffi::c_int) as *mut uint8_t;
    aom_highbd_comp_avg_pred_c(
        comp_pred8,
        second_pred,
        32 as core::ffi::c_int,
        16 as core::ffi::c_int,
        ref_0,
        ref_stride,
    );
    return highbd_sadb(
        src,
        src_stride,
        comp_pred8,
        32 as core::ffi::c_int,
        32 as core::ffi::c_int,
        16 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad_skip_32x16x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = (2 as core::ffi::c_uint)
            .wrapping_mul(
                highbd_sad(
                    src,
                    2 as core::ffi::c_int * src_stride,
                    *ref_array.offset(i as isize),
                    2 as core::ffi::c_int * ref_stride,
                    32 as core::ffi::c_int,
                    16 as core::ffi::c_int / 2 as core::ffi::c_int,
                ),
            ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad32x16x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = aom_highbd_sad32x16_c(
            src,
            src_stride,
            *ref_array.offset(i as isize),
            ref_stride,
        ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad32x16x3d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    aom_highbd_sad32x16x4d_c(src, src_stride, ref_array, ref_stride, sad_array);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad16x32_avg_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
) -> core::ffi::c_uint {
    let mut comp_pred: [uint16_t; 512] = [0; 512];
    let comp_pred8: *mut uint8_t = (comp_pred.as_mut_ptr() as uintptr_t
        >> 1 as core::ffi::c_int) as *mut uint8_t;
    aom_highbd_comp_avg_pred_c(
        comp_pred8,
        second_pred,
        16 as core::ffi::c_int,
        32 as core::ffi::c_int,
        ref_0,
        ref_stride,
    );
    return highbd_sadb(
        src,
        src_stride,
        comp_pred8,
        16 as core::ffi::c_int,
        16 as core::ffi::c_int,
        32 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad_skip_16x32_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return (2 as core::ffi::c_uint)
        .wrapping_mul(
            highbd_sad(
                src,
                2 as core::ffi::c_int * src_stride,
                ref_0,
                2 as core::ffi::c_int * ref_stride,
                16 as core::ffi::c_int,
                32 as core::ffi::c_int / 2 as core::ffi::c_int,
            ),
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad16x32_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return highbd_sad(
        src,
        src_stride,
        ref_0,
        ref_stride,
        16 as core::ffi::c_int,
        32 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad16x32x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = aom_highbd_sad16x32_c(
            src,
            src_stride,
            *ref_array.offset(i as isize),
            ref_stride,
        ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad_skip_16x32x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = (2 as core::ffi::c_uint)
            .wrapping_mul(
                highbd_sad(
                    src,
                    2 as core::ffi::c_int * src_stride,
                    *ref_array.offset(i as isize),
                    2 as core::ffi::c_int * ref_stride,
                    16 as core::ffi::c_int,
                    32 as core::ffi::c_int / 2 as core::ffi::c_int,
                ),
            ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad16x32x3d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    aom_highbd_sad16x32x4d_c(src, src_stride, ref_array, ref_stride, sad_array);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad_skip_16x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return (2 as core::ffi::c_uint)
        .wrapping_mul(
            highbd_sad(
                src,
                2 as core::ffi::c_int * src_stride,
                ref_0,
                2 as core::ffi::c_int * ref_stride,
                16 as core::ffi::c_int,
                16 as core::ffi::c_int / 2 as core::ffi::c_int,
            ),
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad16x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return highbd_sad(
        src,
        src_stride,
        ref_0,
        ref_stride,
        16 as core::ffi::c_int,
        16 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad16x16_avg_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
) -> core::ffi::c_uint {
    let mut comp_pred: [uint16_t; 256] = [0; 256];
    let comp_pred8: *mut uint8_t = (comp_pred.as_mut_ptr() as uintptr_t
        >> 1 as core::ffi::c_int) as *mut uint8_t;
    aom_highbd_comp_avg_pred_c(
        comp_pred8,
        second_pred,
        16 as core::ffi::c_int,
        16 as core::ffi::c_int,
        ref_0,
        ref_stride,
    );
    return highbd_sadb(
        src,
        src_stride,
        comp_pred8,
        16 as core::ffi::c_int,
        16 as core::ffi::c_int,
        16 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad_skip_16x16x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = (2 as core::ffi::c_uint)
            .wrapping_mul(
                highbd_sad(
                    src,
                    2 as core::ffi::c_int * src_stride,
                    *ref_array.offset(i as isize),
                    2 as core::ffi::c_int * ref_stride,
                    16 as core::ffi::c_int,
                    16 as core::ffi::c_int / 2 as core::ffi::c_int,
                ),
            ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad16x16x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = aom_highbd_sad16x16_c(
            src,
            src_stride,
            *ref_array.offset(i as isize),
            ref_stride,
        ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad16x16x3d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    aom_highbd_sad16x16x4d_c(src, src_stride, ref_array, ref_stride, sad_array);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad16x8_avg_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
) -> core::ffi::c_uint {
    let mut comp_pred: [uint16_t; 128] = [0; 128];
    let comp_pred8: *mut uint8_t = (comp_pred.as_mut_ptr() as uintptr_t
        >> 1 as core::ffi::c_int) as *mut uint8_t;
    aom_highbd_comp_avg_pred_c(
        comp_pred8,
        second_pred,
        16 as core::ffi::c_int,
        8 as core::ffi::c_int,
        ref_0,
        ref_stride,
    );
    return highbd_sadb(
        src,
        src_stride,
        comp_pred8,
        16 as core::ffi::c_int,
        16 as core::ffi::c_int,
        8 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad16x8_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return highbd_sad(
        src,
        src_stride,
        ref_0,
        ref_stride,
        16 as core::ffi::c_int,
        8 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad16x8x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = aom_highbd_sad16x8_c(
            src,
            src_stride,
            *ref_array.offset(i as isize),
            ref_stride,
        ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad16x8x3d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    aom_highbd_sad16x8x4d_c(src, src_stride, ref_array, ref_stride, sad_array);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad8x16_avg_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
) -> core::ffi::c_uint {
    let mut comp_pred: [uint16_t; 128] = [0; 128];
    let comp_pred8: *mut uint8_t = (comp_pred.as_mut_ptr() as uintptr_t
        >> 1 as core::ffi::c_int) as *mut uint8_t;
    aom_highbd_comp_avg_pred_c(
        comp_pred8,
        second_pred,
        8 as core::ffi::c_int,
        16 as core::ffi::c_int,
        ref_0,
        ref_stride,
    );
    return highbd_sadb(
        src,
        src_stride,
        comp_pred8,
        8 as core::ffi::c_int,
        8 as core::ffi::c_int,
        16 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad8x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return highbd_sad(
        src,
        src_stride,
        ref_0,
        ref_stride,
        8 as core::ffi::c_int,
        16 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad_skip_8x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return (2 as core::ffi::c_uint)
        .wrapping_mul(
            highbd_sad(
                src,
                2 as core::ffi::c_int * src_stride,
                ref_0,
                2 as core::ffi::c_int * ref_stride,
                8 as core::ffi::c_int,
                16 as core::ffi::c_int / 2 as core::ffi::c_int,
            ),
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad8x16x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = aom_highbd_sad8x16_c(
            src,
            src_stride,
            *ref_array.offset(i as isize),
            ref_stride,
        ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad_skip_8x16x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = (2 as core::ffi::c_uint)
            .wrapping_mul(
                highbd_sad(
                    src,
                    2 as core::ffi::c_int * src_stride,
                    *ref_array.offset(i as isize),
                    2 as core::ffi::c_int * ref_stride,
                    8 as core::ffi::c_int,
                    16 as core::ffi::c_int / 2 as core::ffi::c_int,
                ),
            ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad8x16x3d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    aom_highbd_sad8x16x4d_c(src, src_stride, ref_array, ref_stride, sad_array);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad8x8_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return highbd_sad(
        src,
        src_stride,
        ref_0,
        ref_stride,
        8 as core::ffi::c_int,
        8 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad8x8_avg_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
) -> core::ffi::c_uint {
    let mut comp_pred: [uint16_t; 64] = [0; 64];
    let comp_pred8: *mut uint8_t = (comp_pred.as_mut_ptr() as uintptr_t
        >> 1 as core::ffi::c_int) as *mut uint8_t;
    aom_highbd_comp_avg_pred_c(
        comp_pred8,
        second_pred,
        8 as core::ffi::c_int,
        8 as core::ffi::c_int,
        ref_0,
        ref_stride,
    );
    return highbd_sadb(
        src,
        src_stride,
        comp_pred8,
        8 as core::ffi::c_int,
        8 as core::ffi::c_int,
        8 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad8x8x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = aom_highbd_sad8x8_c(
            src,
            src_stride,
            *ref_array.offset(i as isize),
            ref_stride,
        ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad8x8x3d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    aom_highbd_sad8x8x4d_c(src, src_stride, ref_array, ref_stride, sad_array);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad8x4_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return highbd_sad(
        src,
        src_stride,
        ref_0,
        ref_stride,
        8 as core::ffi::c_int,
        4 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad8x4x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = aom_highbd_sad8x4_c(
            src,
            src_stride,
            *ref_array.offset(i as isize),
            ref_stride,
        ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad8x4x3d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    aom_highbd_sad8x4x4d_c(src, src_stride, ref_array, ref_stride, sad_array);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad4x8_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return highbd_sad(
        src,
        src_stride,
        ref_0,
        ref_stride,
        4 as core::ffi::c_int,
        8 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad4x8x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = aom_highbd_sad4x8_c(
            src,
            src_stride,
            *ref_array.offset(i as isize),
            ref_stride,
        ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad4x8x3d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    aom_highbd_sad4x8x4d_c(src, src_stride, ref_array, ref_stride, sad_array);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad4x4_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return highbd_sad(
        src,
        src_stride,
        ref_0,
        ref_stride,
        4 as core::ffi::c_int,
        4 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad4x4x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = aom_highbd_sad4x4_c(
            src,
            src_stride,
            *ref_array.offset(i as isize),
            ref_stride,
        ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad4x4x3d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    aom_highbd_sad4x4x4d_c(src, src_stride, ref_array, ref_stride, sad_array);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad4x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return highbd_sad(
        src,
        src_stride,
        ref_0,
        ref_stride,
        4 as core::ffi::c_int,
        16 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad_skip_4x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return (2 as core::ffi::c_uint)
        .wrapping_mul(
            highbd_sad(
                src,
                2 as core::ffi::c_int * src_stride,
                ref_0,
                2 as core::ffi::c_int * ref_stride,
                4 as core::ffi::c_int,
                16 as core::ffi::c_int / 2 as core::ffi::c_int,
            ),
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad4x16x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = aom_highbd_sad4x16_c(
            src,
            src_stride,
            *ref_array.offset(i as isize),
            ref_stride,
        ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad_skip_4x16x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = (2 as core::ffi::c_uint)
            .wrapping_mul(
                highbd_sad(
                    src,
                    2 as core::ffi::c_int * src_stride,
                    *ref_array.offset(i as isize),
                    2 as core::ffi::c_int * ref_stride,
                    4 as core::ffi::c_int,
                    16 as core::ffi::c_int / 2 as core::ffi::c_int,
                ),
            ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad16x4_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return highbd_sad(
        src,
        src_stride,
        ref_0,
        ref_stride,
        16 as core::ffi::c_int,
        4 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad16x4x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = aom_highbd_sad16x4_c(
            src,
            src_stride,
            *ref_array.offset(i as isize),
            ref_stride,
        ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad_skip_8x32_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return (2 as core::ffi::c_uint)
        .wrapping_mul(
            highbd_sad(
                src,
                2 as core::ffi::c_int * src_stride,
                ref_0,
                2 as core::ffi::c_int * ref_stride,
                8 as core::ffi::c_int,
                32 as core::ffi::c_int / 2 as core::ffi::c_int,
            ),
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad8x32_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return highbd_sad(
        src,
        src_stride,
        ref_0,
        ref_stride,
        8 as core::ffi::c_int,
        32 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad8x32_avg_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
) -> core::ffi::c_uint {
    let mut comp_pred: [uint16_t; 256] = [0; 256];
    let comp_pred8: *mut uint8_t = (comp_pred.as_mut_ptr() as uintptr_t
        >> 1 as core::ffi::c_int) as *mut uint8_t;
    aom_highbd_comp_avg_pred_c(
        comp_pred8,
        second_pred,
        8 as core::ffi::c_int,
        32 as core::ffi::c_int,
        ref_0,
        ref_stride,
    );
    return highbd_sadb(
        src,
        src_stride,
        comp_pred8,
        8 as core::ffi::c_int,
        8 as core::ffi::c_int,
        32 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad_skip_8x32x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = (2 as core::ffi::c_uint)
            .wrapping_mul(
                highbd_sad(
                    src,
                    2 as core::ffi::c_int * src_stride,
                    *ref_array.offset(i as isize),
                    2 as core::ffi::c_int * ref_stride,
                    8 as core::ffi::c_int,
                    32 as core::ffi::c_int / 2 as core::ffi::c_int,
                ),
            ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad8x32x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = aom_highbd_sad8x32_c(
            src,
            src_stride,
            *ref_array.offset(i as isize),
            ref_stride,
        ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad32x8_avg_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
) -> core::ffi::c_uint {
    let mut comp_pred: [uint16_t; 256] = [0; 256];
    let comp_pred8: *mut uint8_t = (comp_pred.as_mut_ptr() as uintptr_t
        >> 1 as core::ffi::c_int) as *mut uint8_t;
    aom_highbd_comp_avg_pred_c(
        comp_pred8,
        second_pred,
        32 as core::ffi::c_int,
        8 as core::ffi::c_int,
        ref_0,
        ref_stride,
    );
    return highbd_sadb(
        src,
        src_stride,
        comp_pred8,
        32 as core::ffi::c_int,
        32 as core::ffi::c_int,
        8 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad32x8_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return highbd_sad(
        src,
        src_stride,
        ref_0,
        ref_stride,
        32 as core::ffi::c_int,
        8 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad32x8x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = aom_highbd_sad32x8_c(
            src,
            src_stride,
            *ref_array.offset(i as isize),
            ref_stride,
        ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad_skip_16x64_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return (2 as core::ffi::c_uint)
        .wrapping_mul(
            highbd_sad(
                src,
                2 as core::ffi::c_int * src_stride,
                ref_0,
                2 as core::ffi::c_int * ref_stride,
                16 as core::ffi::c_int,
                64 as core::ffi::c_int / 2 as core::ffi::c_int,
            ),
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad16x64_avg_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
) -> core::ffi::c_uint {
    let mut comp_pred: [uint16_t; 1024] = [0; 1024];
    let comp_pred8: *mut uint8_t = (comp_pred.as_mut_ptr() as uintptr_t
        >> 1 as core::ffi::c_int) as *mut uint8_t;
    aom_highbd_comp_avg_pred_c(
        comp_pred8,
        second_pred,
        16 as core::ffi::c_int,
        64 as core::ffi::c_int,
        ref_0,
        ref_stride,
    );
    return highbd_sadb(
        src,
        src_stride,
        comp_pred8,
        16 as core::ffi::c_int,
        16 as core::ffi::c_int,
        64 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad16x64_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return highbd_sad(
        src,
        src_stride,
        ref_0,
        ref_stride,
        16 as core::ffi::c_int,
        64 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad16x64x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = aom_highbd_sad16x64_c(
            src,
            src_stride,
            *ref_array.offset(i as isize),
            ref_stride,
        ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad_skip_16x64x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = (2 as core::ffi::c_uint)
            .wrapping_mul(
                highbd_sad(
                    src,
                    2 as core::ffi::c_int * src_stride,
                    *ref_array.offset(i as isize),
                    2 as core::ffi::c_int * ref_stride,
                    16 as core::ffi::c_int,
                    64 as core::ffi::c_int / 2 as core::ffi::c_int,
                ),
            ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad64x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return highbd_sad(
        src,
        src_stride,
        ref_0,
        ref_stride,
        64 as core::ffi::c_int,
        16 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad64x16_avg_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
) -> core::ffi::c_uint {
    let mut comp_pred: [uint16_t; 1024] = [0; 1024];
    let comp_pred8: *mut uint8_t = (comp_pred.as_mut_ptr() as uintptr_t
        >> 1 as core::ffi::c_int) as *mut uint8_t;
    aom_highbd_comp_avg_pred_c(
        comp_pred8,
        second_pred,
        64 as core::ffi::c_int,
        16 as core::ffi::c_int,
        ref_0,
        ref_stride,
    );
    return highbd_sadb(
        src,
        src_stride,
        comp_pred8,
        64 as core::ffi::c_int,
        64 as core::ffi::c_int,
        16 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad_skip_64x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) -> core::ffi::c_uint {
    return (2 as core::ffi::c_uint)
        .wrapping_mul(
            highbd_sad(
                src,
                2 as core::ffi::c_int * src_stride,
                ref_0,
                2 as core::ffi::c_int * ref_stride,
                64 as core::ffi::c_int,
                16 as core::ffi::c_int / 2 as core::ffi::c_int,
            ),
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad64x16x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = aom_highbd_sad64x16_c(
            src,
            src_stride,
            *ref_array.offset(i as isize),
            ref_stride,
        ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad_skip_64x16x4d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *sad_array.offset(i as isize) = (2 as core::ffi::c_uint)
            .wrapping_mul(
                highbd_sad(
                    src,
                    2 as core::ffi::c_int * src_stride,
                    *ref_array.offset(i as isize),
                    2 as core::ffi::c_int * ref_stride,
                    64 as core::ffi::c_int,
                    16 as core::ffi::c_int / 2 as core::ffi::c_int,
                ),
            ) as uint32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad4x16x3d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    aom_highbd_sad4x16x4d_c(src, src_stride, ref_array, ref_stride, sad_array);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad16x4x3d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    aom_highbd_sad16x4x4d_c(src, src_stride, ref_array, ref_stride, sad_array);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad8x32x3d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    aom_highbd_sad8x32x4d_c(src, src_stride, ref_array, ref_stride, sad_array);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad32x8x3d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    aom_highbd_sad32x8x4d_c(src, src_stride, ref_array, ref_stride, sad_array);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad16x64x3d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    aom_highbd_sad16x64x4d_c(src, src_stride, ref_array, ref_stride, sad_array);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sad64x16x3d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_array: *const *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sad_array: *mut uint32_t,
) {
    aom_highbd_sad64x16x4d_c(src, src_stride, ref_array, ref_stride, sad_array);
}

extern "C" {
    fn abs(__x: core::ffi::c_int) -> core::ffi::c_int;
}
