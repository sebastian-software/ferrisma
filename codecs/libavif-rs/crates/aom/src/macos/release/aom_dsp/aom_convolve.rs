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

// Mechanically translated from upstream/aom/aom_dsp/aom_convolve.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C-unwind" {
}
pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub type int16_t = i16;
pub type intptr_t = isize;
pub type uintptr_t = usize;
pub type ptrdiff_t = isize;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type InterpKernel = [int16_t; 8];
#[inline]
unsafe extern "C-unwind" fn clip_pixel(mut val: core::ffi::c_int) -> uint8_t {
    return (if val > 255 as core::ffi::c_int {
        255 as core::ffi::c_int
    } else if val < 0 as core::ffi::c_int {
        0 as core::ffi::c_int
    } else {
        val
    }) as uint8_t;
}
#[inline]
unsafe extern "C-unwind" fn clamp(
    mut value: core::ffi::c_int,
    mut low: core::ffi::c_int,
    mut high: core::ffi::c_int,
) -> core::ffi::c_int {
    return if value < low { low } else if value > high { high } else { value };
}
#[inline]
unsafe extern "C-unwind" fn clip_pixel_highbd(
    mut val: core::ffi::c_int,
    mut bd: core::ffi::c_int,
) -> uint16_t {
    match bd {
        10 => {
            return clamp(val, 0 as core::ffi::c_int, 1023 as core::ffi::c_int)
                as uint16_t;
        }
        12 => {
            return clamp(val, 0 as core::ffi::c_int, 4095 as core::ffi::c_int)
                as uint16_t;
        }
        8 | _ => {
            return clamp(val, 0 as core::ffi::c_int, 255 as core::ffi::c_int) as uint16_t;
        }
    };
}
pub const SUBPEL_BITS: core::ffi::c_int = 4 as core::ffi::c_int;
pub const SUBPEL_MASK: core::ffi::c_int = ((1 as core::ffi::c_int) << SUBPEL_BITS)
    - 1 as core::ffi::c_int;
pub const SUBPEL_TAPS: core::ffi::c_int = 8 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn horz_scalar_product(
    mut a: *const uint8_t,
    mut b: *const int16_t,
) -> core::ffi::c_int {
    let mut sum: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut k: core::ffi::c_int = 0 as core::ffi::c_int;
    while k < SUBPEL_TAPS {
        sum
            += *a.offset(k as isize) as core::ffi::c_int
                * *b.offset(k as isize) as core::ffi::c_int;
        k += 1;
    }
    return sum;
}
#[inline]
unsafe extern "C-unwind" fn vert_scalar_product(
    mut a: *const uint8_t,
    mut a_stride: ptrdiff_t,
    mut b: *const int16_t,
) -> core::ffi::c_int {
    let mut sum: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut k: core::ffi::c_int = 0 as core::ffi::c_int;
    while k < SUBPEL_TAPS {
        sum
            += *a.offset((k as ptrdiff_t * a_stride) as isize) as core::ffi::c_int
                * *b.offset(k as isize) as core::ffi::c_int;
        k += 1;
    }
    return sum;
}
unsafe extern "C-unwind" fn convolve_horiz(
    mut src: *const uint8_t,
    mut src_stride: ptrdiff_t,
    mut dst: *mut uint8_t,
    mut dst_stride: ptrdiff_t,
    mut x_filters: *const InterpKernel,
    mut x0_q4: core::ffi::c_int,
    mut x_step_q4: core::ffi::c_int,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
) {
    src = src
        .offset(
            -((SUBPEL_TAPS / 2 as core::ffi::c_int - 1 as core::ffi::c_int) as isize),
        );
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    while y < h {
        let mut x_q4: core::ffi::c_int = x0_q4;
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < w {
            let src_x: *const uint8_t = &*src.offset((x_q4 >> SUBPEL_BITS) as isize)
                as *const uint8_t;
            let x_filter: *const int16_t = (*x_filters
                .offset((x_q4 & SUBPEL_MASK) as isize))
                .as_ptr();
            let sum: core::ffi::c_int = horz_scalar_product(src_x, x_filter)
                as core::ffi::c_int;
            *dst.offset(x as isize) = clip_pixel(
                sum
                    + ((1 as core::ffi::c_int) << 7 as core::ffi::c_int
                        >> 1 as core::ffi::c_int) >> 7 as core::ffi::c_int,
            );
            x_q4 += x_step_q4;
            x += 1;
        }
        src = src.offset(src_stride as isize);
        dst = dst.offset(dst_stride as isize);
        y += 1;
    }
}
unsafe extern "C-unwind" fn convolve_vert(
    mut src: *const uint8_t,
    mut src_stride: ptrdiff_t,
    mut dst: *mut uint8_t,
    mut dst_stride: ptrdiff_t,
    mut y_filters: *const InterpKernel,
    mut y0_q4: core::ffi::c_int,
    mut y_step_q4: core::ffi::c_int,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
) {
    src = src
        .offset(
            -((src_stride
                * (SUBPEL_TAPS / 2 as core::ffi::c_int - 1 as core::ffi::c_int)
                    as ptrdiff_t) as isize),
        );
    let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
    while x < w {
        let mut y_q4: core::ffi::c_int = y0_q4;
        let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
        while y < h {
            let mut src_y: *const core::ffi::c_uchar = &*src
                .offset(((y_q4 >> SUBPEL_BITS) as ptrdiff_t * src_stride) as isize)
                as *const core::ffi::c_uchar;
            let y_filter: *const int16_t = (*y_filters
                .offset((y_q4 & SUBPEL_MASK) as isize))
                .as_ptr();
            let sum: core::ffi::c_int = vert_scalar_product(
                src_y as *const uint8_t,
                src_stride,
                y_filter,
            ) as core::ffi::c_int;
            *dst.offset((y as ptrdiff_t * dst_stride) as isize) = clip_pixel(
                sum
                    + ((1 as core::ffi::c_int) << 7 as core::ffi::c_int
                        >> 1 as core::ffi::c_int) >> 7 as core::ffi::c_int,
            );
            y_q4 += y_step_q4;
            y += 1;
        }
        src = src.offset(1);
        dst = dst.offset(1);
        x += 1;
    }
}
unsafe extern "C-unwind" fn get_filter_base(mut filter: *const int16_t) -> *const InterpKernel {
    return (filter as intptr_t & !(0xff as core::ffi::c_int as intptr_t))
        as *const InterpKernel;
}
unsafe extern "C-unwind" fn get_filter_offset(
    mut f: *const int16_t,
    mut base: *const InterpKernel,
) -> core::ffi::c_int {
    return (f as intptr_t as *const InterpKernel).offset_from(base) as core::ffi::c_long
        as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_convolve8_horiz_c(
    mut src: *const uint8_t,
    mut src_stride: ptrdiff_t,
    mut dst: *mut uint8_t,
    mut dst_stride: ptrdiff_t,
    mut filter_x: *const int16_t,
    mut x_step_q4: core::ffi::c_int,
    mut filter_y: *const int16_t,
    mut y_step_q4: core::ffi::c_int,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
) {
    let filters_x: *const InterpKernel = get_filter_base(filter_x)
        as *const InterpKernel;
    let x0_q4: core::ffi::c_int = get_filter_offset(filter_x, filters_x)
        as core::ffi::c_int;
    convolve_horiz(src, src_stride, dst, dst_stride, filters_x, x0_q4, x_step_q4, w, h);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_convolve8_vert_c(
    mut src: *const uint8_t,
    mut src_stride: ptrdiff_t,
    mut dst: *mut uint8_t,
    mut dst_stride: ptrdiff_t,
    mut filter_x: *const int16_t,
    mut x_step_q4: core::ffi::c_int,
    mut filter_y: *const int16_t,
    mut y_step_q4: core::ffi::c_int,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
) {
    let filters_y: *const InterpKernel = get_filter_base(filter_y)
        as *const InterpKernel;
    let y0_q4: core::ffi::c_int = get_filter_offset(filter_y, filters_y)
        as core::ffi::c_int;
    convolve_vert(src, src_stride, dst, dst_stride, filters_y, y0_q4, y_step_q4, w, h);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_scaled_2d_c(
    mut src: *const uint8_t,
    mut src_stride: ptrdiff_t,
    mut dst: *mut uint8_t,
    mut dst_stride: ptrdiff_t,
    mut filter: *const InterpKernel,
    mut x0_q4: core::ffi::c_int,
    mut x_step_q4: core::ffi::c_int,
    mut y0_q4: core::ffi::c_int,
    mut y_step_q4: core::ffi::c_int,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
) {
    let mut temp: [uint8_t; 8640] = [0; 8640];
    let intermediate_height: core::ffi::c_int = ((h - 1 as core::ffi::c_int) * y_step_q4
        + y0_q4 >> SUBPEL_BITS) + SUBPEL_TAPS;
    convolve_horiz(
        src
            .offset(
                -((src_stride
                    * (SUBPEL_TAPS / 2 as core::ffi::c_int - 1 as core::ffi::c_int)
                        as ptrdiff_t) as isize),
            ),
        src_stride,
        temp.as_mut_ptr(),
        64 as ptrdiff_t,
        filter,
        x0_q4,
        x_step_q4,
        w,
        intermediate_height,
    );
    convolve_vert(
        temp
            .as_mut_ptr()
            .offset(
                (64 as core::ffi::c_int
                    * (SUBPEL_TAPS / 2 as core::ffi::c_int - 1 as core::ffi::c_int))
                    as isize,
            ),
        64 as ptrdiff_t,
        dst,
        dst_stride,
        filter,
        y0_q4,
        y_step_q4,
        w,
        h,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_convolve_copy_c(
    mut src: *const uint8_t,
    mut src_stride: ptrdiff_t,
    mut dst: *mut uint8_t,
    mut dst_stride: ptrdiff_t,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
) {
    let mut r: core::ffi::c_int = h;
    while r > 0 as core::ffi::c_int {
        memmove(
            dst as *mut core::ffi::c_void,
            src as *const core::ffi::c_void,
            w as size_t,
        );
        src = src.offset(src_stride as isize);
        dst = dst.offset(dst_stride as isize);
        r -= 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn highbd_vert_scalar_product(
    mut a: *const uint16_t,
    mut a_stride: ptrdiff_t,
    mut b: *const int16_t,
) -> core::ffi::c_int {
    let mut sum: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut k: core::ffi::c_int = 0 as core::ffi::c_int;
    while k < SUBPEL_TAPS {
        sum
            += *a.offset((k as ptrdiff_t * a_stride) as isize) as core::ffi::c_int
                * *b.offset(k as isize) as core::ffi::c_int;
        k += 1;
    }
    return sum;
}
#[inline]
unsafe extern "C-unwind" fn highbd_horz_scalar_product(
    mut a: *const uint16_t,
    mut b: *const int16_t,
) -> core::ffi::c_int {
    let mut sum: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut k: core::ffi::c_int = 0 as core::ffi::c_int;
    while k < SUBPEL_TAPS {
        sum
            += *a.offset(k as isize) as core::ffi::c_int
                * *b.offset(k as isize) as core::ffi::c_int;
        k += 1;
    }
    return sum;
}
unsafe extern "C-unwind" fn highbd_convolve_horiz(
    mut src8: *const uint8_t,
    mut src_stride: ptrdiff_t,
    mut dst8: *mut uint8_t,
    mut dst_stride: ptrdiff_t,
    mut x_filters: *const InterpKernel,
    mut x0_q4: core::ffi::c_int,
    mut x_step_q4: core::ffi::c_int,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut bd: core::ffi::c_int,
) {
    let mut src: *mut uint16_t = ((src8 as uintptr_t) << 1 as core::ffi::c_int)
        as *mut uint16_t;
    let mut dst: *mut uint16_t = ((dst8 as uintptr_t) << 1 as core::ffi::c_int)
        as *mut uint16_t;
    src = src
        .offset(
            -((SUBPEL_TAPS / 2 as core::ffi::c_int - 1 as core::ffi::c_int) as isize),
        );
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    while y < h {
        let mut x_q4: core::ffi::c_int = x0_q4;
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < w {
            let src_x: *const uint16_t = &raw mut *src.offset((x_q4 >> SUBPEL_BITS) as isize)
                as *mut uint16_t;
            let x_filter: *const int16_t = (*x_filters
                .offset((x_q4 & SUBPEL_MASK) as isize))
                .as_ptr();
            let sum: core::ffi::c_int = highbd_horz_scalar_product(src_x, x_filter)
                as core::ffi::c_int;
            *dst.offset(x as isize) = clip_pixel_highbd(
                sum
                    + ((1 as core::ffi::c_int) << 7 as core::ffi::c_int
                        >> 1 as core::ffi::c_int) >> 7 as core::ffi::c_int,
                bd,
            );
            x_q4 += x_step_q4;
            x += 1;
        }
        src = src.offset(src_stride as isize);
        dst = dst.offset(dst_stride as isize);
        y += 1;
    }
}
unsafe extern "C-unwind" fn highbd_convolve_vert(
    mut src8: *const uint8_t,
    mut src_stride: ptrdiff_t,
    mut dst8: *mut uint8_t,
    mut dst_stride: ptrdiff_t,
    mut y_filters: *const InterpKernel,
    mut y0_q4: core::ffi::c_int,
    mut y_step_q4: core::ffi::c_int,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut bd: core::ffi::c_int,
) {
    let mut src: *mut uint16_t = ((src8 as uintptr_t) << 1 as core::ffi::c_int)
        as *mut uint16_t;
    let mut dst: *mut uint16_t = ((dst8 as uintptr_t) << 1 as core::ffi::c_int)
        as *mut uint16_t;
    src = src
        .offset(
            -((src_stride
                * (SUBPEL_TAPS / 2 as core::ffi::c_int - 1 as core::ffi::c_int)
                    as ptrdiff_t) as isize),
        );
    let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
    while x < w {
        let mut y_q4: core::ffi::c_int = y0_q4;
        let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
        while y < h {
            let mut src_y: *const uint16_t = &raw mut *src
                .offset(((y_q4 >> SUBPEL_BITS) as ptrdiff_t * src_stride) as isize)
                as *mut uint16_t;
            let y_filter: *const int16_t = (*y_filters
                .offset((y_q4 & SUBPEL_MASK) as isize))
                .as_ptr();
            let sum: core::ffi::c_int = highbd_vert_scalar_product(
                src_y,
                src_stride,
                y_filter,
            ) as core::ffi::c_int;
            *dst.offset((y as ptrdiff_t * dst_stride) as isize) = clip_pixel_highbd(
                sum
                    + ((1 as core::ffi::c_int) << 7 as core::ffi::c_int
                        >> 1 as core::ffi::c_int) >> 7 as core::ffi::c_int,
                bd,
            );
            y_q4 += y_step_q4;
            y += 1;
        }
        src = src.offset(1);
        dst = dst.offset(1);
        x += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_convolve8_horiz_c(
    mut src: *const uint8_t,
    mut src_stride: ptrdiff_t,
    mut dst: *mut uint8_t,
    mut dst_stride: ptrdiff_t,
    mut filter_x: *const int16_t,
    mut x_step_q4: core::ffi::c_int,
    mut filter_y: *const int16_t,
    mut y_step_q4: core::ffi::c_int,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut bd: core::ffi::c_int,
) {
    let filters_x: *const InterpKernel = get_filter_base(filter_x)
        as *const InterpKernel;
    let x0_q4: core::ffi::c_int = get_filter_offset(filter_x, filters_x)
        as core::ffi::c_int;
    highbd_convolve_horiz(
        src,
        src_stride,
        dst,
        dst_stride,
        filters_x,
        x0_q4,
        x_step_q4,
        w,
        h,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_convolve8_vert_c(
    mut src: *const uint8_t,
    mut src_stride: ptrdiff_t,
    mut dst: *mut uint8_t,
    mut dst_stride: ptrdiff_t,
    mut filter_x: *const int16_t,
    mut x_step_q4: core::ffi::c_int,
    mut filter_y: *const int16_t,
    mut y_step_q4: core::ffi::c_int,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut bd: core::ffi::c_int,
) {
    let filters_y: *const InterpKernel = get_filter_base(filter_y)
        as *const InterpKernel;
    let y0_q4: core::ffi::c_int = get_filter_offset(filter_y, filters_y)
        as core::ffi::c_int;
    highbd_convolve_vert(
        src,
        src_stride,
        dst,
        dst_stride,
        filters_y,
        y0_q4,
        y_step_q4,
        w,
        h,
        bd,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_convolve_copy_c(
    mut src: *const uint16_t,
    mut src_stride: ptrdiff_t,
    mut dst: *mut uint16_t,
    mut dst_stride: ptrdiff_t,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
) {
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    while y < h {
        memmove(
            dst as *mut core::ffi::c_void,
            src as *const core::ffi::c_void,
            (w as size_t).wrapping_mul(::core::mem::size_of::<uint16_t>() as size_t),
        );
        src = src.offset(src_stride as isize);
        dst = dst.offset(dst_stride as isize);
        y += 1;
    }
}

extern "C" {
    fn memmove(
        __dst: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __len: size_t,
    ) -> *mut core::ffi::c_void;
}
