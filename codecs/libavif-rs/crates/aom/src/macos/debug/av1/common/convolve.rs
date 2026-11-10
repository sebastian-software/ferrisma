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

// Mechanically translated from upstream/aom/av1/common/convolve.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C-unwind" {
    fn aom_convolve_copy_c(
        src: *const uint8_t,
        src_stride: ptrdiff_t,
        dst: *mut uint8_t,
        dst_stride: ptrdiff_t,
        w: core::ffi::c_int,
        h: core::ffi::c_int,
    );
    fn aom_highbd_convolve_copy_c(
        src: *const uint16_t,
        src_stride: ptrdiff_t,
        dst: *mut uint16_t,
        dst_stride: ptrdiff_t,
        w: core::ffi::c_int,
        h: core::ffi::c_int,
    );
}
pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub type int16_t = i16;
pub type int32_t = i32;
pub type intptr_t = isize;
pub type uintptr_t = usize;
pub type ptrdiff_t = isize;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type InterpKernel = [int16_t; 8];
pub type InterpFilter = core::ffi::c_uchar;
pub const INTERP_INVALID: InterpFilter = 255;
pub const EXTRA_FILTERS: InterpFilter = 2;
pub const SWITCHABLE: InterpFilter = 4;
pub const SWITCHABLE_FILTERS: InterpFilter = 3;
pub const INTERP_FILTERS_ALL: InterpFilter = 5;
pub const MULTITAP_SHARP2: InterpFilter = 4;
pub const BILINEAR: InterpFilter = 3;
pub const MULTITAP_SHARP: InterpFilter = 2;
pub const EIGHTTAP_SMOOTH: InterpFilter = 1;
pub const EIGHTTAP_REGULAR: InterpFilter = 0;
pub type CONV_BUF_TYPE = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct InterpFilterParams {
    pub filter_ptr: *const int16_t,
    pub taps: uint16_t,
    pub interp_filter: InterpFilter,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ConvolveParams {
    pub do_average: core::ffi::c_int,
    pub dst: *mut CONV_BUF_TYPE,
    pub dst_stride: core::ffi::c_int,
    pub round_0: core::ffi::c_int,
    pub round_1: core::ffi::c_int,
    pub plane: core::ffi::c_int,
    pub is_compound: core::ffi::c_int,
    pub use_dist_wtd_comp_avg: core::ffi::c_int,
    pub fwd_offset: core::ffi::c_int,
    pub bck_offset: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WienerConvolveParams {
    pub round_0: core::ffi::c_int,
    pub round_1: core::ffi::c_int,
}
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
pub const MAX_SB_SIZE_LOG2: core::ffi::c_int = 7 as core::ffi::c_int;
pub const MAX_SB_SIZE: core::ffi::c_int = (1 as core::ffi::c_int) << MAX_SB_SIZE_LOG2;
pub const DIST_PRECISION_BITS: core::ffi::c_int = 4 as core::ffi::c_int;
pub const FILTER_BITS: core::ffi::c_int = 7 as core::ffi::c_int;
pub const SUBPEL_BITS: core::ffi::c_int = 4 as core::ffi::c_int;
pub const SUBPEL_MASK: core::ffi::c_int = ((1 as core::ffi::c_int) << SUBPEL_BITS)
    - 1 as core::ffi::c_int;
pub const SUBPEL_TAPS: core::ffi::c_int = 8 as core::ffi::c_int;
pub const SCALE_SUBPEL_BITS: core::ffi::c_int = 10 as core::ffi::c_int;
pub const SCALE_SUBPEL_SHIFTS: core::ffi::c_int = (1 as core::ffi::c_int)
    << SCALE_SUBPEL_BITS;
pub const SCALE_SUBPEL_MASK: core::ffi::c_int = SCALE_SUBPEL_SHIFTS
    - 1 as core::ffi::c_int;
pub const SCALE_EXTRA_BITS: core::ffi::c_int = SCALE_SUBPEL_BITS - SUBPEL_BITS;
pub const RS_SUBPEL_BITS: core::ffi::c_int = 6 as core::ffi::c_int;
pub const RS_SCALE_SUBPEL_BITS: core::ffi::c_int = 14 as core::ffi::c_int;
pub const RS_SCALE_SUBPEL_MASK: core::ffi::c_int = ((1 as core::ffi::c_int)
    << RS_SCALE_SUBPEL_BITS) - 1 as core::ffi::c_int;
pub const RS_SCALE_EXTRA_BITS: core::ffi::c_int = RS_SCALE_SUBPEL_BITS - RS_SUBPEL_BITS;
#[inline]
unsafe extern "C-unwind" fn av1_get_interp_filter_subpel_kernel(
    filter_params: *const InterpFilterParams,
    subpel: core::ffi::c_int,
) -> *const int16_t {
    return ((*filter_params).filter_ptr)
        .offset(((*filter_params).taps as core::ffi::c_int * subpel) as isize);
}
pub const UPSCALE_NORMATIVE_TAPS: core::ffi::c_int = 8 as core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_convolve_horiz_rs_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut dst: *mut uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut x_filters: *const int16_t,
    mut x0_qn: core::ffi::c_int,
    mut x_step_qn: core::ffi::c_int,
) {
    src = src
        .offset(
            -((UPSCALE_NORMATIVE_TAPS / 2 as core::ffi::c_int - 1 as core::ffi::c_int)
                as isize),
        );
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    while y < h {
        let mut x_qn: core::ffi::c_int = x0_qn;
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < w {
            let src_x: *const uint8_t = &*src
                .offset((x_qn >> RS_SCALE_SUBPEL_BITS) as isize) as *const uint8_t;
            let x_filter_idx: core::ffi::c_int = (x_qn & RS_SCALE_SUBPEL_MASK)
                >> RS_SCALE_EXTRA_BITS;
            if !(x_filter_idx
                <= ((1 as core::ffi::c_int) << 6 as core::ffi::c_int)
                    - 1 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long
                != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 24],
                        [core::ffi::c_char; 24],
                    >(*b"av1_convolve_horiz_rs_c\0"))
                        .as_ptr(),
                    b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
                    37 as core::ffi::c_int,
                    b"x_filter_idx <= RS_SUBPEL_MASK\0" as *const u8
                        as *const core::ffi::c_char,
                );
            } else {};
            let x_filter: *const int16_t = &*x_filters
                .offset((x_filter_idx * UPSCALE_NORMATIVE_TAPS) as isize)
                as *const int16_t;
            let mut sum: core::ffi::c_int = 0 as core::ffi::c_int;
            let mut k: core::ffi::c_int = 0 as core::ffi::c_int;
            while k < UPSCALE_NORMATIVE_TAPS {
                sum
                    += *src_x.offset(k as isize) as core::ffi::c_int
                        * *x_filter.offset(k as isize) as core::ffi::c_int;
                k += 1;
            }
            *dst.offset(x as isize) = clip_pixel(
                sum
                    + ((1 as core::ffi::c_int) << 7 as core::ffi::c_int
                        >> 1 as core::ffi::c_int) >> 7 as core::ffi::c_int,
            );
            x_qn += x_step_qn;
            x += 1;
        }
        src = src.offset(src_stride as isize);
        dst = dst.offset(dst_stride as isize);
        y += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_highbd_convolve_horiz_rs_c(
    mut src: *const uint16_t,
    mut src_stride: core::ffi::c_int,
    mut dst: *mut uint16_t,
    mut dst_stride: core::ffi::c_int,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut x_filters: *const int16_t,
    mut x0_qn: core::ffi::c_int,
    mut x_step_qn: core::ffi::c_int,
    mut bd: core::ffi::c_int,
) {
    src = src
        .offset(
            -((UPSCALE_NORMATIVE_TAPS / 2 as core::ffi::c_int - 1 as core::ffi::c_int)
                as isize),
        );
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    while y < h {
        let mut x_qn: core::ffi::c_int = x0_qn;
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < w {
            let src_x: *const uint16_t = &*src
                .offset((x_qn >> RS_SCALE_SUBPEL_BITS) as isize) as *const uint16_t;
            let x_filter_idx: core::ffi::c_int = (x_qn & RS_SCALE_SUBPEL_MASK)
                >> RS_SCALE_EXTRA_BITS;
            if !(x_filter_idx
                <= ((1 as core::ffi::c_int) << 6 as core::ffi::c_int)
                    - 1 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long
                != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 31],
                        [core::ffi::c_char; 31],
                    >(*b"av1_highbd_convolve_horiz_rs_c\0"))
                        .as_ptr(),
                    b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
                    63 as core::ffi::c_int,
                    b"x_filter_idx <= RS_SUBPEL_MASK\0" as *const u8
                        as *const core::ffi::c_char,
                );
            } else {};
            let x_filter: *const int16_t = &*x_filters
                .offset((x_filter_idx * UPSCALE_NORMATIVE_TAPS) as isize)
                as *const int16_t;
            let mut sum: core::ffi::c_int = 0 as core::ffi::c_int;
            let mut k: core::ffi::c_int = 0 as core::ffi::c_int;
            while k < UPSCALE_NORMATIVE_TAPS {
                sum
                    += *src_x.offset(k as isize) as core::ffi::c_int
                        * *x_filter.offset(k as isize) as core::ffi::c_int;
                k += 1;
            }
            *dst.offset(x as isize) = clip_pixel_highbd(
                sum
                    + ((1 as core::ffi::c_int) << 7 as core::ffi::c_int
                        >> 1 as core::ffi::c_int) >> 7 as core::ffi::c_int,
                bd,
            );
            x_qn += x_step_qn;
            x += 1;
        }
        src = src.offset(src_stride as isize);
        dst = dst.offset(dst_stride as isize);
        y += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_convolve_2d_sr_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut dst: *mut uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut filter_params_x: *const InterpFilterParams,
    mut filter_params_y: *const InterpFilterParams,
    subpel_x_qn: core::ffi::c_int,
    subpel_y_qn: core::ffi::c_int,
    mut conv_params: *mut ConvolveParams,
) {
    let mut im_block: [int16_t; 17792] = [0; 17792];
    let mut im_h: core::ffi::c_int = h + (*filter_params_y).taps as core::ffi::c_int
        - 1 as core::ffi::c_int;
    let mut im_stride: core::ffi::c_int = w;
    if !(w <= (1 as core::ffi::c_int) << 7 as core::ffi::c_int
        && h <= (1 as core::ffi::c_int) << 7 as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"av1_convolve_2d_sr_c\0"))
                .as_ptr(),
            b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
            87 as core::ffi::c_int,
            b"w <= MAX_SB_SIZE && h <= MAX_SB_SIZE\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    let fo_vert: core::ffi::c_int = (*filter_params_y).taps as core::ffi::c_int
        / 2 as core::ffi::c_int - 1 as core::ffi::c_int;
    let fo_horiz: core::ffi::c_int = (*filter_params_x).taps as core::ffi::c_int
        / 2 as core::ffi::c_int - 1 as core::ffi::c_int;
    let bd: core::ffi::c_int = 8 as core::ffi::c_int;
    let bits: core::ffi::c_int = FILTER_BITS * 2 as core::ffi::c_int
        - (*conv_params).round_0 - (*conv_params).round_1;
    let mut src_horiz: *const uint8_t = src.offset(-((fo_vert * src_stride) as isize));
    let mut x_filter: *const int16_t = av1_get_interp_filter_subpel_kernel(
        filter_params_x,
        subpel_x_qn & SUBPEL_MASK,
    );
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    while y < im_h {
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < w {
            let mut sum: int32_t = (1 as int32_t)
                << bd + FILTER_BITS - 1 as core::ffi::c_int;
            let mut k: core::ffi::c_int = 0 as core::ffi::c_int;
            while k < (*filter_params_x).taps as core::ffi::c_int {
                sum = (sum as core::ffi::c_int
                    + *x_filter.offset(k as isize) as core::ffi::c_int
                        * *src_horiz.offset((y * src_stride + x - fo_horiz + k) as isize)
                            as core::ffi::c_int) as int32_t;
                k += 1;
            }
            if !((*filter_params_x).taps as core::ffi::c_int > 8 as core::ffi::c_int
                || 0 as int32_t <= sum
                    && sum
                        < (1 as int32_t)
                            << bd + 7 as core::ffi::c_int + 1 as core::ffi::c_int)
                as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 21],
                        [core::ffi::c_char; 21],
                    >(*b"av1_convolve_2d_sr_c\0"))
                        .as_ptr(),
                    b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
                    110 as core::ffi::c_int,
                    b"filter_params_x->taps > 8 || (0 <= sum && sum < (1 << (bd + FILTER_BITS + 1)))\0"
                        as *const u8 as *const core::ffi::c_char,
                );
            } else {};
            im_block[(y * im_stride + x) as usize] = (sum
                + ((1 as int32_t) << (*conv_params).round_0 >> 1 as core::ffi::c_int)
                >> (*conv_params).round_0) as int16_t;
            x += 1;
        }
        y += 1;
    }
    let mut src_vert: *mut int16_t = im_block
        .as_mut_ptr()
        .offset((fo_vert * im_stride) as isize);
    let mut y_filter: *const int16_t = av1_get_interp_filter_subpel_kernel(
        filter_params_y,
        subpel_y_qn & SUBPEL_MASK,
    );
    let offset_bits: core::ffi::c_int = bd + 2 as core::ffi::c_int * FILTER_BITS
        - (*conv_params).round_0;
    let mut y_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while y_0 < h {
        let mut x_0: core::ffi::c_int = 0 as core::ffi::c_int;
        while x_0 < w {
            let mut sum_0: int32_t = (1 as int32_t) << offset_bits;
            let mut k_0: core::ffi::c_int = 0 as core::ffi::c_int;
            while k_0 < (*filter_params_y).taps as core::ffi::c_int {
                sum_0 = (sum_0 as core::ffi::c_int
                    + *y_filter.offset(k_0 as isize) as core::ffi::c_int
                        * *src_vert
                            .offset(((y_0 - fo_vert + k_0) * im_stride + x_0) as isize)
                            as core::ffi::c_int) as int32_t;
                k_0 += 1;
            }
            if !((*filter_params_y).taps as core::ffi::c_int > 8 as core::ffi::c_int
                || 0 as int32_t <= sum_0
                    && sum_0 < (1 as int32_t) << offset_bits + 2 as core::ffi::c_int)
                as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 21],
                        [core::ffi::c_char; 21],
                    >(*b"av1_convolve_2d_sr_c\0"))
                        .as_ptr(),
                    b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
                    128 as core::ffi::c_int,
                    b"filter_params_y->taps > 8 || (0 <= sum && sum < (1 << (offset_bits + 2)))\0"
                        as *const u8 as *const core::ffi::c_char,
                );
            } else {};
            let mut res: int16_t = ((sum_0
                + ((1 as int32_t) << (*conv_params).round_1 >> 1 as core::ffi::c_int)
                >> (*conv_params).round_1)
                - (((1 as int32_t) << offset_bits - (*conv_params).round_1)
                    + ((1 as int32_t)
                        << offset_bits - (*conv_params).round_1
                            - 1 as core::ffi::c_int))) as int16_t;
            *dst.offset((y_0 * dst_stride + x_0) as isize) = clip_pixel(
                res as core::ffi::c_int
                    + ((1 as core::ffi::c_int) << bits >> 1 as core::ffi::c_int) >> bits,
            );
            x_0 += 1;
        }
        y_0 += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_convolve_y_sr_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut dst: *mut uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut filter_params_y: *const InterpFilterParams,
    subpel_y_qn: core::ffi::c_int,
) {
    let fo_vert: core::ffi::c_int = (*filter_params_y).taps as core::ffi::c_int
        / 2 as core::ffi::c_int - 1 as core::ffi::c_int;
    let mut y_filter: *const int16_t = av1_get_interp_filter_subpel_kernel(
        filter_params_y,
        subpel_y_qn & SUBPEL_MASK,
    );
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    while y < h {
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < w {
            let mut res: int32_t = 0 as int32_t;
            let mut k: core::ffi::c_int = 0 as core::ffi::c_int;
            while k < (*filter_params_y).taps as core::ffi::c_int {
                res = (res as core::ffi::c_int
                    + *y_filter.offset(k as isize) as core::ffi::c_int
                        * *src.offset(((y - fo_vert + k) * src_stride + x) as isize)
                            as core::ffi::c_int) as int32_t;
                k += 1;
            }
            *dst.offset((y * dst_stride + x) as isize) = clip_pixel(
                res as core::ffi::c_int
                    + ((1 as core::ffi::c_int) << 7 as core::ffi::c_int
                        >> 1 as core::ffi::c_int) >> 7 as core::ffi::c_int,
            );
            x += 1;
        }
        y += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_convolve_x_sr_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut dst: *mut uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut filter_params_x: *const InterpFilterParams,
    subpel_x_qn: core::ffi::c_int,
    mut conv_params: *mut ConvolveParams,
) {
    let fo_horiz: core::ffi::c_int = (*filter_params_x).taps as core::ffi::c_int
        / 2 as core::ffi::c_int - 1 as core::ffi::c_int;
    let bits: core::ffi::c_int = FILTER_BITS - (*conv_params).round_0;
    if !(bits >= 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 20],
                [core::ffi::c_char; 20],
            >(*b"av1_convolve_x_sr_c\0"))
                .as_ptr(),
            b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
            165 as core::ffi::c_int,
            b"bits >= 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !(7 as core::ffi::c_int - (*conv_params).round_1 >= 0 as core::ffi::c_int
        || (*conv_params).round_0 + (*conv_params).round_1
            == 2 as core::ffi::c_int * 7 as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 20],
                [core::ffi::c_char; 20],
            >(*b"av1_convolve_x_sr_c\0"))
                .as_ptr(),
            b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
            167 as core::ffi::c_int,
            b"(FILTER_BITS - conv_params->round_1) >= 0 || ((conv_params->round_0 + conv_params->round_1) == 2 * FILTER_BITS)\0"
                as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let mut x_filter: *const int16_t = av1_get_interp_filter_subpel_kernel(
        filter_params_x,
        subpel_x_qn & SUBPEL_MASK,
    );
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    while y < h {
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < w {
            let mut res: int32_t = 0 as int32_t;
            let mut k: core::ffi::c_int = 0 as core::ffi::c_int;
            while k < (*filter_params_x).taps as core::ffi::c_int {
                res = (res as core::ffi::c_int
                    + *x_filter.offset(k as isize) as core::ffi::c_int
                        * *src.offset((y * src_stride + x - fo_horiz + k) as isize)
                            as core::ffi::c_int) as int32_t;
                k += 1;
            }
            res = res
                + ((1 as int32_t) << (*conv_params).round_0 >> 1 as core::ffi::c_int)
                >> (*conv_params).round_0;
            *dst.offset((y * dst_stride + x) as isize) = clip_pixel(
                res as core::ffi::c_int
                    + ((1 as core::ffi::c_int) << bits >> 1 as core::ffi::c_int) >> bits,
            );
            x += 1;
        }
        y += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_convolve_2d_sr_intrabc_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut dst: *mut uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut filter_params_x: *const InterpFilterParams,
    mut filter_params_y: *const InterpFilterParams,
    subpel_x_qn: core::ffi::c_int,
    subpel_y_qn: core::ffi::c_int,
    mut conv_params: *mut ConvolveParams,
) {
    if !(subpel_x_qn == 8 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long
        != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 29],
                [core::ffi::c_char; 29],
            >(*b"av1_convolve_2d_sr_intrabc_c\0"))
                .as_ptr(),
            b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
            198 as core::ffi::c_int,
            b"subpel_x_qn == 8\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !(subpel_y_qn == 8 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long
        != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 29],
                [core::ffi::c_char; 29],
            >(*b"av1_convolve_2d_sr_intrabc_c\0"))
                .as_ptr(),
            b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
            199 as core::ffi::c_int,
            b"subpel_y_qn == 8\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !((*filter_params_x).taps as core::ffi::c_int == 2 as core::ffi::c_int
        && (*filter_params_y).taps as core::ffi::c_int == 2 as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 29],
                [core::ffi::c_char; 29],
            >(*b"av1_convolve_2d_sr_intrabc_c\0"))
                .as_ptr(),
            b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
            200 as core::ffi::c_int,
            b"filter_params_x->taps == 2 && filter_params_y->taps == 2\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    if !((*conv_params).round_0 + (*conv_params).round_1
        == 2 as core::ffi::c_int * 7 as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 29],
                [core::ffi::c_char; 29],
            >(*b"av1_convolve_2d_sr_intrabc_c\0"))
                .as_ptr(),
            b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
            201 as core::ffi::c_int,
            b"(conv_params->round_0 + conv_params->round_1) == 2 * FILTER_BITS\0"
                as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let mut im_block: [int16_t; 17792] = [0; 17792];
    let mut im_h: core::ffi::c_int = h + 1 as core::ffi::c_int;
    let mut im_stride: core::ffi::c_int = w;
    if !(w <= (1 as core::ffi::c_int) << 7 as core::ffi::c_int
        && h <= (1 as core::ffi::c_int) << 7 as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 29],
                [core::ffi::c_char; 29],
            >(*b"av1_convolve_2d_sr_intrabc_c\0"))
                .as_ptr(),
            b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
            211 as core::ffi::c_int,
            b"w <= MAX_SB_SIZE && h <= MAX_SB_SIZE\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    let bd: core::ffi::c_int = 8 as core::ffi::c_int;
    let mut im: *mut int16_t = im_block.as_mut_ptr();
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    while y < im_h {
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < w {
            let sum: int32_t = ((1 as int32_t) << bd)
                + *src.offset(x as isize) as int32_t
                + *src.offset((x + 1 as core::ffi::c_int) as isize) as int32_t;
            if !(0 as int32_t <= sum
                && sum < (1 as int32_t) << bd + 2 as core::ffi::c_int)
                as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 29],
                        [core::ffi::c_char; 29],
                    >(*b"av1_convolve_2d_sr_intrabc_c\0"))
                        .as_ptr(),
                    b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
                    220 as core::ffi::c_int,
                    b"0 <= sum && sum < (1 << (bd + 2))\0" as *const u8
                        as *const core::ffi::c_char,
                );
            } else {};
            *im.offset(x as isize) = sum as int16_t;
            x += 1;
        }
        src = src.offset(src_stride as isize);
        im = im.offset(im_stride as isize);
        y += 1;
    }
    let mut src_vert: *mut int16_t = im_block.as_mut_ptr();
    let mut y_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while y_0 < h {
        let mut x_0: core::ffi::c_int = 0 as core::ffi::c_int;
        while x_0 < w {
            let sum_0: int32_t = ((1 as int32_t) << bd + 2 as core::ffi::c_int)
                + *src_vert.offset(x_0 as isize) as int32_t
                + *src_vert.offset((im_stride + x_0) as isize) as int32_t;
            if !(0 as int32_t <= sum_0
                && sum_0 < (1 as int32_t) << bd + 4 as core::ffi::c_int)
                as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 29],
                        [core::ffi::c_char; 29],
                    >(*b"av1_convolve_2d_sr_intrabc_c\0"))
                        .as_ptr(),
                    b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
                    234 as core::ffi::c_int,
                    b"0 <= sum && sum < (1 << (bd + 4))\0" as *const u8
                        as *const core::ffi::c_char,
                );
            } else {};
            let res: int16_t = ((sum_0
                + ((1 as int32_t) << 2 as core::ffi::c_int >> 1 as core::ffi::c_int)
                >> 2 as core::ffi::c_int)
                - (((1 as int32_t) << bd)
                    + ((1 as int32_t) << bd - 1 as core::ffi::c_int))) as int16_t;
            *dst.offset(x_0 as isize) = clip_pixel(res as core::ffi::c_int);
            x_0 += 1;
        }
        src_vert = src_vert.offset(im_stride as isize);
        dst = dst.offset(dst_stride as isize);
        y_0 += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_convolve_y_sr_intrabc_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut dst: *mut uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut filter_params_y: *const InterpFilterParams,
    subpel_y_qn: core::ffi::c_int,
) {
    if !(subpel_y_qn == 8 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long
        != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 28],
                [core::ffi::c_char; 28],
            >(*b"av1_convolve_y_sr_intrabc_c\0"))
                .as_ptr(),
            b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
            250 as core::ffi::c_int,
            b"subpel_y_qn == 8\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !((*filter_params_y).taps as core::ffi::c_int == 2 as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 28],
                [core::ffi::c_char; 28],
            >(*b"av1_convolve_y_sr_intrabc_c\0"))
                .as_ptr(),
            b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
            251 as core::ffi::c_int,
            b"filter_params_y->taps == 2\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    while y < h {
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < w {
            let res: int32_t = *src.offset(x as isize) as int32_t
                + *src.offset((src_stride + x) as isize) as int32_t;
            *dst.offset(x as isize) = clip_pixel(
                res as core::ffi::c_int
                    + ((1 as core::ffi::c_int) << 1 as core::ffi::c_int
                        >> 1 as core::ffi::c_int) >> 1 as core::ffi::c_int,
            );
            x += 1;
        }
        src = src.offset(src_stride as isize);
        dst = dst.offset(dst_stride as isize);
        y += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_convolve_x_sr_intrabc_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut dst: *mut uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut filter_params_x: *const InterpFilterParams,
    subpel_x_qn: core::ffi::c_int,
    mut conv_params: *mut ConvolveParams,
) {
    if !(subpel_x_qn == 8 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long
        != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 28],
                [core::ffi::c_char; 28],
            >(*b"av1_convolve_x_sr_intrabc_c\0"))
                .as_ptr(),
            b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
            274 as core::ffi::c_int,
            b"subpel_x_qn == 8\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !((*filter_params_x).taps as core::ffi::c_int == 2 as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 28],
                [core::ffi::c_char; 28],
            >(*b"av1_convolve_x_sr_intrabc_c\0"))
                .as_ptr(),
            b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
            275 as core::ffi::c_int,
            b"filter_params_x->taps == 2\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !((*conv_params).round_0 + (*conv_params).round_1
        == 2 as core::ffi::c_int * 7 as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 28],
                [core::ffi::c_char; 28],
            >(*b"av1_convolve_x_sr_intrabc_c\0"))
                .as_ptr(),
            b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
            276 as core::ffi::c_int,
            b"(conv_params->round_0 + conv_params->round_1) == 2 * FILTER_BITS\0"
                as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    while y < h {
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < w {
            let res: int32_t = *src.offset(x as isize) as int32_t
                + *src.offset((x + 1 as core::ffi::c_int) as isize) as int32_t;
            *dst.offset(x as isize) = clip_pixel(
                res as core::ffi::c_int
                    + ((1 as core::ffi::c_int) << 1 as core::ffi::c_int
                        >> 1 as core::ffi::c_int) >> 1 as core::ffi::c_int,
            );
            x += 1;
        }
        src = src.offset(src_stride as isize);
        dst = dst.offset(dst_stride as isize);
        y += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_dist_wtd_convolve_2d_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut dst: *mut uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut filter_params_x: *const InterpFilterParams,
    mut filter_params_y: *const InterpFilterParams,
    subpel_x_qn: core::ffi::c_int,
    subpel_y_qn: core::ffi::c_int,
    mut conv_params: *mut ConvolveParams,
) {
    let mut dst16: *mut CONV_BUF_TYPE = (*conv_params).dst;
    let mut dst16_stride: core::ffi::c_int = (*conv_params).dst_stride;
    let mut im_block: [int16_t; 17792] = [0; 17792];
    let mut im_h: core::ffi::c_int = h + (*filter_params_y).taps as core::ffi::c_int
        - 1 as core::ffi::c_int;
    let mut im_stride: core::ffi::c_int = w;
    let fo_vert: core::ffi::c_int = (*filter_params_y).taps as core::ffi::c_int
        / 2 as core::ffi::c_int - 1 as core::ffi::c_int;
    let fo_horiz: core::ffi::c_int = (*filter_params_x).taps as core::ffi::c_int
        / 2 as core::ffi::c_int - 1 as core::ffi::c_int;
    let bd: core::ffi::c_int = 8 as core::ffi::c_int;
    let round_bits: core::ffi::c_int = 2 as core::ffi::c_int * FILTER_BITS
        - (*conv_params).round_0 - (*conv_params).round_1;
    let mut src_horiz: *const uint8_t = src.offset(-((fo_vert * src_stride) as isize));
    let mut x_filter: *const int16_t = av1_get_interp_filter_subpel_kernel(
        filter_params_x,
        subpel_x_qn & SUBPEL_MASK,
    );
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    while y < im_h {
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < w {
            let mut sum: int32_t = (1 as int32_t)
                << bd + FILTER_BITS - 1 as core::ffi::c_int;
            let mut k: core::ffi::c_int = 0 as core::ffi::c_int;
            while k < (*filter_params_x).taps as core::ffi::c_int {
                sum = (sum as core::ffi::c_int
                    + *x_filter.offset(k as isize) as core::ffi::c_int
                        * *src_horiz.offset((y * src_stride + x - fo_horiz + k) as isize)
                            as core::ffi::c_int) as int32_t;
                k += 1;
            }
            if !((*filter_params_x).taps as core::ffi::c_int > 8 as core::ffi::c_int
                || 0 as int32_t <= sum
                    && sum
                        < (1 as int32_t)
                            << bd + 7 as core::ffi::c_int + 1 as core::ffi::c_int)
                as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 27],
                        [core::ffi::c_char; 27],
                    >(*b"av1_dist_wtd_convolve_2d_c\0"))
                        .as_ptr(),
                    b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
                    321 as core::ffi::c_int,
                    b"filter_params_x->taps > 8 || (0 <= sum && sum < (1 << (bd + FILTER_BITS + 1)))\0"
                        as *const u8 as *const core::ffi::c_char,
                );
            } else {};
            im_block[(y * im_stride + x) as usize] = (sum
                + ((1 as int32_t) << (*conv_params).round_0 >> 1 as core::ffi::c_int)
                >> (*conv_params).round_0) as int16_t;
            x += 1;
        }
        y += 1;
    }
    let mut src_vert: *mut int16_t = im_block
        .as_mut_ptr()
        .offset((fo_vert * im_stride) as isize);
    let mut y_filter: *const int16_t = av1_get_interp_filter_subpel_kernel(
        filter_params_y,
        subpel_y_qn & SUBPEL_MASK,
    );
    let offset_bits: core::ffi::c_int = bd + 2 as core::ffi::c_int * FILTER_BITS
        - (*conv_params).round_0;
    let mut y_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while y_0 < h {
        let mut x_0: core::ffi::c_int = 0 as core::ffi::c_int;
        while x_0 < w {
            let mut sum_0: int32_t = (1 as int32_t) << offset_bits;
            let mut k_0: core::ffi::c_int = 0 as core::ffi::c_int;
            while k_0 < (*filter_params_y).taps as core::ffi::c_int {
                sum_0 = (sum_0 as core::ffi::c_int
                    + *y_filter.offset(k_0 as isize) as core::ffi::c_int
                        * *src_vert
                            .offset(((y_0 - fo_vert + k_0) * im_stride + x_0) as isize)
                            as core::ffi::c_int) as int32_t;
                k_0 += 1;
            }
            if !((*filter_params_y).taps as core::ffi::c_int > 8 as core::ffi::c_int
                || 0 as int32_t <= sum_0
                    && sum_0 < (1 as int32_t) << offset_bits + 2 as core::ffi::c_int)
                as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 27],
                        [core::ffi::c_char; 27],
                    >(*b"av1_dist_wtd_convolve_2d_c\0"))
                        .as_ptr(),
                    b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
                    339 as core::ffi::c_int,
                    b"filter_params_y->taps > 8 || (0 <= sum && sum < (1 << (offset_bits + 2)))\0"
                        as *const u8 as *const core::ffi::c_char,
                );
            } else {};
            let mut res: CONV_BUF_TYPE = (sum_0
                + ((1 as int32_t) << (*conv_params).round_1 >> 1 as core::ffi::c_int)
                >> (*conv_params).round_1) as CONV_BUF_TYPE;
            if (*conv_params).do_average != 0 {
                let mut tmp: int32_t = *dst16.offset((y_0 * dst16_stride + x_0) as isize)
                    as int32_t;
                if (*conv_params).use_dist_wtd_comp_avg != 0 {
                    tmp = tmp * (*conv_params).fwd_offset as int32_t
                        + res as int32_t * (*conv_params).bck_offset as int32_t;
                    tmp = tmp >> DIST_PRECISION_BITS;
                } else {
                    tmp = (tmp as core::ffi::c_int + res as core::ffi::c_int) as int32_t;
                    tmp = tmp >> 1 as core::ffi::c_int;
                }
                tmp = (tmp as core::ffi::c_int
                    - (((1 as core::ffi::c_int) << offset_bits - (*conv_params).round_1)
                        + ((1 as core::ffi::c_int)
                            << offset_bits - (*conv_params).round_1
                                - 1 as core::ffi::c_int))) as int32_t;
                *dst.offset((y_0 * dst_stride + x_0) as isize) = clip_pixel(
                    tmp as core::ffi::c_int
                        + ((1 as core::ffi::c_int) << round_bits
                            >> 1 as core::ffi::c_int) >> round_bits,
                );
            } else {
                *dst16.offset((y_0 * dst16_stride + x_0) as isize) = res;
            }
            x_0 += 1;
        }
        y_0 += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_dist_wtd_convolve_y_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut dst: *mut uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut filter_params_y: *const InterpFilterParams,
    subpel_y_qn: core::ffi::c_int,
    mut conv_params: *mut ConvolveParams,
) {
    let mut dst16: *mut CONV_BUF_TYPE = (*conv_params).dst;
    let mut dst16_stride: core::ffi::c_int = (*conv_params).dst_stride;
    let fo_vert: core::ffi::c_int = (*filter_params_y).taps as core::ffi::c_int
        / 2 as core::ffi::c_int - 1 as core::ffi::c_int;
    let bits: core::ffi::c_int = FILTER_BITS - (*conv_params).round_0;
    let bd: core::ffi::c_int = 8 as core::ffi::c_int;
    let offset_bits: core::ffi::c_int = bd + 2 as core::ffi::c_int * FILTER_BITS
        - (*conv_params).round_0;
    let round_offset: core::ffi::c_int = ((1 as core::ffi::c_int)
        << offset_bits - (*conv_params).round_1)
        + ((1 as core::ffi::c_int)
            << offset_bits - (*conv_params).round_1 - 1 as core::ffi::c_int);
    let round_bits: core::ffi::c_int = 2 as core::ffi::c_int * FILTER_BITS
        - (*conv_params).round_0 - (*conv_params).round_1;
    let mut y_filter: *const int16_t = av1_get_interp_filter_subpel_kernel(
        filter_params_y,
        subpel_y_qn & SUBPEL_MASK,
    );
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    while y < h {
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < w {
            let mut res: int32_t = 0 as int32_t;
            let mut k: core::ffi::c_int = 0 as core::ffi::c_int;
            while k < (*filter_params_y).taps as core::ffi::c_int {
                res = (res as core::ffi::c_int
                    + *y_filter.offset(k as isize) as core::ffi::c_int
                        * *src.offset(((y - fo_vert + k) * src_stride + x) as isize)
                            as core::ffi::c_int) as int32_t;
                k += 1;
            }
            res = (res as core::ffi::c_int * ((1 as core::ffi::c_int) << bits))
                as int32_t;
            res = (res
                + ((1 as int32_t) << (*conv_params).round_1 >> 1 as core::ffi::c_int)
                >> (*conv_params).round_1) + round_offset as int32_t;
            if (*conv_params).do_average != 0 {
                let mut tmp: int32_t = *dst16.offset((y * dst16_stride + x) as isize)
                    as int32_t;
                if (*conv_params).use_dist_wtd_comp_avg != 0 {
                    tmp = tmp * (*conv_params).fwd_offset as int32_t
                        + res * (*conv_params).bck_offset as int32_t;
                    tmp = tmp >> DIST_PRECISION_BITS;
                } else {
                    tmp += res;
                    tmp = tmp >> 1 as core::ffi::c_int;
                }
                tmp = (tmp as core::ffi::c_int - round_offset) as int32_t;
                *dst.offset((y * dst_stride + x) as isize) = clip_pixel(
                    tmp as core::ffi::c_int
                        + ((1 as core::ffi::c_int) << round_bits
                            >> 1 as core::ffi::c_int) >> round_bits,
                );
            } else {
                *dst16.offset((y * dst16_stride + x) as isize) = res as CONV_BUF_TYPE;
            }
            x += 1;
        }
        y += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_dist_wtd_convolve_x_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut dst: *mut uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut filter_params_x: *const InterpFilterParams,
    subpel_x_qn: core::ffi::c_int,
    mut conv_params: *mut ConvolveParams,
) {
    let mut dst16: *mut CONV_BUF_TYPE = (*conv_params).dst;
    let mut dst16_stride: core::ffi::c_int = (*conv_params).dst_stride;
    let fo_horiz: core::ffi::c_int = (*filter_params_x).taps as core::ffi::c_int
        / 2 as core::ffi::c_int - 1 as core::ffi::c_int;
    let bits: core::ffi::c_int = FILTER_BITS - (*conv_params).round_1;
    let bd: core::ffi::c_int = 8 as core::ffi::c_int;
    let offset_bits: core::ffi::c_int = bd + 2 as core::ffi::c_int * FILTER_BITS
        - (*conv_params).round_0;
    let round_offset: core::ffi::c_int = ((1 as core::ffi::c_int)
        << offset_bits - (*conv_params).round_1)
        + ((1 as core::ffi::c_int)
            << offset_bits - (*conv_params).round_1 - 1 as core::ffi::c_int);
    let round_bits: core::ffi::c_int = 2 as core::ffi::c_int * FILTER_BITS
        - (*conv_params).round_0 - (*conv_params).round_1;
    let mut x_filter: *const int16_t = av1_get_interp_filter_subpel_kernel(
        filter_params_x,
        subpel_x_qn & SUBPEL_MASK,
    );
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    while y < h {
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < w {
            let mut res: int32_t = 0 as int32_t;
            let mut k: core::ffi::c_int = 0 as core::ffi::c_int;
            while k < (*filter_params_x).taps as core::ffi::c_int {
                res = (res as core::ffi::c_int
                    + *x_filter.offset(k as isize) as core::ffi::c_int
                        * *src.offset((y * src_stride + x - fo_horiz + k) as isize)
                            as core::ffi::c_int) as int32_t;
                k += 1;
            }
            res = ((1 as int32_t) << bits)
                * (res
                    + ((1 as int32_t) << (*conv_params).round_0 >> 1 as core::ffi::c_int)
                    >> (*conv_params).round_0);
            res = (res as core::ffi::c_int + round_offset) as int32_t;
            if (*conv_params).do_average != 0 {
                let mut tmp: int32_t = *dst16.offset((y * dst16_stride + x) as isize)
                    as int32_t;
                if (*conv_params).use_dist_wtd_comp_avg != 0 {
                    tmp = tmp * (*conv_params).fwd_offset as int32_t
                        + res * (*conv_params).bck_offset as int32_t;
                    tmp = tmp >> DIST_PRECISION_BITS;
                } else {
                    tmp += res;
                    tmp = tmp >> 1 as core::ffi::c_int;
                }
                tmp = (tmp as core::ffi::c_int - round_offset) as int32_t;
                *dst.offset((y * dst_stride + x) as isize) = clip_pixel(
                    tmp as core::ffi::c_int
                        + ((1 as core::ffi::c_int) << round_bits
                            >> 1 as core::ffi::c_int) >> round_bits,
                );
            } else {
                *dst16.offset((y * dst16_stride + x) as isize) = res as CONV_BUF_TYPE;
            }
            x += 1;
        }
        y += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_dist_wtd_convolve_2d_copy_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut dst: *mut uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut conv_params: *mut ConvolveParams,
) {
    let mut dst16: *mut CONV_BUF_TYPE = (*conv_params).dst;
    let mut dst16_stride: core::ffi::c_int = (*conv_params).dst_stride;
    let bits: core::ffi::c_int = FILTER_BITS * 2 as core::ffi::c_int
        - (*conv_params).round_1 - (*conv_params).round_0;
    let bd: core::ffi::c_int = 8 as core::ffi::c_int;
    let offset_bits: core::ffi::c_int = bd + 2 as core::ffi::c_int * FILTER_BITS
        - (*conv_params).round_0;
    let round_offset: core::ffi::c_int = ((1 as core::ffi::c_int)
        << offset_bits - (*conv_params).round_1)
        + ((1 as core::ffi::c_int)
            << offset_bits - (*conv_params).round_1 - 1 as core::ffi::c_int);
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    while y < h {
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < w {
            let mut res: CONV_BUF_TYPE = ((*src.offset((y * src_stride + x) as isize)
                as core::ffi::c_int) << bits) as CONV_BUF_TYPE;
            res = (res as core::ffi::c_int + round_offset) as CONV_BUF_TYPE;
            if (*conv_params).do_average != 0 {
                let mut tmp: int32_t = *dst16.offset((y * dst16_stride + x) as isize)
                    as int32_t;
                if (*conv_params).use_dist_wtd_comp_avg != 0 {
                    tmp = tmp * (*conv_params).fwd_offset as int32_t
                        + res as int32_t * (*conv_params).bck_offset as int32_t;
                    tmp = tmp >> DIST_PRECISION_BITS;
                } else {
                    tmp = (tmp as core::ffi::c_int + res as core::ffi::c_int) as int32_t;
                    tmp = tmp >> 1 as core::ffi::c_int;
                }
                tmp = (tmp as core::ffi::c_int - round_offset) as int32_t;
                *dst.offset((y * dst_stride + x) as isize) = clip_pixel(
                    tmp as core::ffi::c_int
                        + ((1 as core::ffi::c_int) << bits >> 1 as core::ffi::c_int)
                        >> bits,
                );
            } else {
                *dst16.offset((y * dst16_stride + x) as isize) = res;
            }
            x += 1;
        }
        y += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_convolve_2d_scale_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut dst: *mut uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut filter_params_x: *const InterpFilterParams,
    mut filter_params_y: *const InterpFilterParams,
    subpel_x_qn: core::ffi::c_int,
    x_step_qn: core::ffi::c_int,
    subpel_y_qn: core::ffi::c_int,
    y_step_qn: core::ffi::c_int,
    mut conv_params: *mut ConvolveParams,
) {
    let mut im_block: [int16_t; 34304] = [0; 34304];
    let mut im_h: core::ffi::c_int = ((h - 1 as core::ffi::c_int) * y_step_qn
        + subpel_y_qn >> SCALE_SUBPEL_BITS)
        + (*filter_params_y).taps as core::ffi::c_int;
    let mut dst16: *mut CONV_BUF_TYPE = (*conv_params).dst;
    let dst16_stride: core::ffi::c_int = (*conv_params).dst_stride;
    let bits: core::ffi::c_int = FILTER_BITS * 2 as core::ffi::c_int
        - (*conv_params).round_0 - (*conv_params).round_1;
    if !(bits >= 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 24],
                [core::ffi::c_char; 24],
            >(*b"av1_convolve_2d_scale_c\0"))
                .as_ptr(),
            b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
            504 as core::ffi::c_int,
            b"bits >= 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let mut im_stride: core::ffi::c_int = w;
    let fo_vert: core::ffi::c_int = (*filter_params_y).taps as core::ffi::c_int
        / 2 as core::ffi::c_int - 1 as core::ffi::c_int;
    let fo_horiz: core::ffi::c_int = (*filter_params_x).taps as core::ffi::c_int
        / 2 as core::ffi::c_int - 1 as core::ffi::c_int;
    let bd: core::ffi::c_int = 8 as core::ffi::c_int;
    let mut src_horiz: *const uint8_t = src.offset(-((fo_vert * src_stride) as isize));
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    while y < im_h {
        let mut x_qn: core::ffi::c_int = subpel_x_qn;
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < w {
            let src_x: *const uint8_t = &*src_horiz
                .offset((x_qn >> SCALE_SUBPEL_BITS) as isize) as *const uint8_t;
            let x_filter_idx: core::ffi::c_int = (x_qn & SCALE_SUBPEL_MASK)
                >> SCALE_EXTRA_BITS;
            if !(x_filter_idx < (1 as core::ffi::c_int) << 4 as core::ffi::c_int)
                as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 24],
                        [core::ffi::c_char; 24],
                    >(*b"av1_convolve_2d_scale_c\0"))
                        .as_ptr(),
                    b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
                    517 as core::ffi::c_int,
                    b"x_filter_idx < SUBPEL_SHIFTS\0" as *const u8
                        as *const core::ffi::c_char,
                );
            } else {};
            let mut x_filter: *const int16_t = av1_get_interp_filter_subpel_kernel(
                filter_params_x,
                x_filter_idx,
            );
            let mut sum: int32_t = (1 as int32_t)
                << bd + FILTER_BITS - 1 as core::ffi::c_int;
            let mut k: core::ffi::c_int = 0 as core::ffi::c_int;
            while k < (*filter_params_x).taps as core::ffi::c_int {
                sum = (sum as core::ffi::c_int
                    + *x_filter.offset(k as isize) as core::ffi::c_int
                        * *src_x.offset((k - fo_horiz) as isize) as core::ffi::c_int)
                    as int32_t;
                k += 1;
            }
            if !((*filter_params_x).taps as core::ffi::c_int > 8 as core::ffi::c_int
                || 0 as int32_t <= sum
                    && sum
                        < (1 as int32_t)
                            << bd + 7 as core::ffi::c_int + 1 as core::ffi::c_int)
                as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 24],
                        [core::ffi::c_char; 24],
                    >(*b"av1_convolve_2d_scale_c\0"))
                        .as_ptr(),
                    b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
                    525 as core::ffi::c_int,
                    b"filter_params_x->taps > 8 || (0 <= sum && sum < (1 << (bd + FILTER_BITS + 1)))\0"
                        as *const u8 as *const core::ffi::c_char,
                );
            } else {};
            im_block[(y * im_stride + x) as usize] = (sum
                + ((1 as int32_t) << (*conv_params).round_0 >> 1 as core::ffi::c_int)
                >> (*conv_params).round_0) as int16_t;
            x += 1;
            x_qn += x_step_qn;
        }
        src_horiz = src_horiz.offset(src_stride as isize);
        y += 1;
    }
    let mut src_vert: *mut int16_t = im_block
        .as_mut_ptr()
        .offset((fo_vert * im_stride) as isize);
    let offset_bits: core::ffi::c_int = bd + 2 as core::ffi::c_int * FILTER_BITS
        - (*conv_params).round_0;
    let mut x_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while x_0 < w {
        let mut y_qn: core::ffi::c_int = subpel_y_qn;
        let mut y_0: core::ffi::c_int = 0 as core::ffi::c_int;
        while y_0 < h {
            let mut src_y: *const int16_t = &raw mut *src_vert
                .offset(((y_qn >> SCALE_SUBPEL_BITS) * im_stride) as isize)
                as *mut int16_t;
            let y_filter_idx: core::ffi::c_int = (y_qn & SCALE_SUBPEL_MASK)
                >> SCALE_EXTRA_BITS;
            if !(y_filter_idx < (1 as core::ffi::c_int) << 4 as core::ffi::c_int)
                as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 24],
                        [core::ffi::c_char; 24],
                    >(*b"av1_convolve_2d_scale_c\0"))
                        .as_ptr(),
                    b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
                    540 as core::ffi::c_int,
                    b"y_filter_idx < SUBPEL_SHIFTS\0" as *const u8
                        as *const core::ffi::c_char,
                );
            } else {};
            let mut y_filter: *const int16_t = av1_get_interp_filter_subpel_kernel(
                filter_params_y,
                y_filter_idx,
            );
            let mut sum_0: int32_t = (1 as int32_t) << offset_bits;
            let mut k_0: core::ffi::c_int = 0 as core::ffi::c_int;
            while k_0 < (*filter_params_y).taps as core::ffi::c_int {
                sum_0 = (sum_0 as core::ffi::c_int
                    + *y_filter.offset(k_0 as isize) as core::ffi::c_int
                        * *src_y.offset(((k_0 - fo_vert) * im_stride) as isize)
                            as core::ffi::c_int) as int32_t;
                k_0 += 1;
            }
            if !((*filter_params_y).taps as core::ffi::c_int > 8 as core::ffi::c_int
                || 0 as int32_t <= sum_0
                    && sum_0 < (1 as int32_t) << offset_bits + 2 as core::ffi::c_int)
                as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 24],
                        [core::ffi::c_char; 24],
                    >(*b"av1_convolve_2d_scale_c\0"))
                        .as_ptr(),
                    b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
                    548 as core::ffi::c_int,
                    b"filter_params_y->taps > 8 || (0 <= sum && sum < (1 << (offset_bits + 2)))\0"
                        as *const u8 as *const core::ffi::c_char,
                );
            } else {};
            let mut res: CONV_BUF_TYPE = (sum_0
                + ((1 as int32_t) << (*conv_params).round_1 >> 1 as core::ffi::c_int)
                >> (*conv_params).round_1) as CONV_BUF_TYPE;
            if (*conv_params).is_compound != 0 {
                if (*conv_params).do_average != 0 {
                    let mut tmp: int32_t = *dst16
                        .offset((y_0 * dst16_stride + x_0) as isize) as int32_t;
                    if (*conv_params).use_dist_wtd_comp_avg != 0 {
                        tmp = tmp * (*conv_params).fwd_offset as int32_t
                            + res as int32_t * (*conv_params).bck_offset as int32_t;
                        tmp = tmp >> DIST_PRECISION_BITS;
                    } else {
                        tmp = (tmp as core::ffi::c_int + res as core::ffi::c_int)
                            as int32_t;
                        tmp = tmp >> 1 as core::ffi::c_int;
                    }
                    tmp = tmp
                        - (((1 as int32_t) << offset_bits - (*conv_params).round_1)
                            + ((1 as int32_t)
                                << offset_bits - (*conv_params).round_1
                                    - 1 as core::ffi::c_int));
                    *dst.offset((y_0 * dst_stride + x_0) as isize) = clip_pixel(
                        tmp as core::ffi::c_int
                            + ((1 as core::ffi::c_int) << bits >> 1 as core::ffi::c_int)
                            >> bits,
                    );
                } else {
                    *dst16.offset((y_0 * dst16_stride + x_0) as isize) = res;
                }
            } else {
                let mut tmp_0: int32_t = res as int32_t
                    - (((1 as int32_t) << offset_bits - (*conv_params).round_1)
                        + ((1 as int32_t)
                            << offset_bits - (*conv_params).round_1
                                - 1 as core::ffi::c_int));
                *dst.offset((y_0 * dst_stride + x_0) as isize) = clip_pixel(
                    tmp_0 as core::ffi::c_int
                        + ((1 as core::ffi::c_int) << bits >> 1 as core::ffi::c_int)
                        >> bits,
                );
            }
            y_0 += 1;
            y_qn += y_step_qn;
        }
        src_vert = src_vert.offset(1);
        x_0 += 1;
    }
}
unsafe extern "C-unwind" fn convolve_2d_scale_wrapper(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut dst: *mut uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut filter_params_x: *const InterpFilterParams,
    mut filter_params_y: *const InterpFilterParams,
    subpel_x_qn: core::ffi::c_int,
    x_step_qn: core::ffi::c_int,
    subpel_y_qn: core::ffi::c_int,
    y_step_qn: core::ffi::c_int,
    mut conv_params: *mut ConvolveParams,
) {
    if (*conv_params).is_compound != 0 {
        if ((*conv_params).dst).is_null() as core::ffi::c_int as core::ffi::c_long != 0 {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 26],
                    [core::ffi::c_char; 26],
                >(*b"convolve_2d_scale_wrapper\0"))
                    .as_ptr(),
                b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
                585 as core::ffi::c_int,
                b"conv_params->dst != NULL\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
    }
    av1_convolve_2d_scale_c(
        src,
        src_stride,
        dst,
        dst_stride,
        w,
        h,
        filter_params_x,
        filter_params_y,
        subpel_x_qn,
        x_step_qn,
        subpel_y_qn,
        y_step_qn,
        conv_params,
    );
}
unsafe extern "C-unwind" fn convolve_2d_facade_compound(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut dst: *mut uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut filter_params_x: *const InterpFilterParams,
    mut filter_params_y: *const InterpFilterParams,
    subpel_x_qn: core::ffi::c_int,
    subpel_y_qn: core::ffi::c_int,
    mut conv_params: *mut ConvolveParams,
) {
    let need_x: bool = subpel_x_qn != 0 as core::ffi::c_int;
    let need_y: bool = subpel_y_qn != 0 as core::ffi::c_int;
    if !need_x && !need_y {
        av1_dist_wtd_convolve_2d_copy_c(
            src,
            src_stride,
            dst,
            dst_stride,
            w,
            h,
            conv_params,
        );
    } else if need_x as core::ffi::c_int != 0 && !need_y {
        av1_dist_wtd_convolve_x_c(
            src,
            src_stride,
            dst,
            dst_stride,
            w,
            h,
            filter_params_x,
            subpel_x_qn,
            conv_params,
        );
    } else if !need_x && need_y as core::ffi::c_int != 0 {
        av1_dist_wtd_convolve_y_c(
            src,
            src_stride,
            dst,
            dst_stride,
            w,
            h,
            filter_params_y,
            subpel_y_qn,
            conv_params,
        );
    } else {
        if !(need_y as core::ffi::c_int != 0 && need_x as core::ffi::c_int != 0)
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 28],
                    [core::ffi::c_char; 28],
                >(*b"convolve_2d_facade_compound\0"))
                    .as_ptr(),
                b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
                609 as core::ffi::c_int,
                b"need_y && need_x\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        av1_dist_wtd_convolve_2d_c(
            src,
            src_stride,
            dst,
            dst_stride,
            w,
            h,
            filter_params_x,
            filter_params_y,
            subpel_x_qn,
            subpel_y_qn,
            conv_params,
        );
    };
}
unsafe extern "C-unwind" fn convolve_2d_facade_single(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut dst: *mut uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut filter_params_x: *const InterpFilterParams,
    mut filter_params_y: *const InterpFilterParams,
    subpel_x_qn: core::ffi::c_int,
    subpel_y_qn: core::ffi::c_int,
    mut conv_params: *mut ConvolveParams,
) {
    let need_x: bool = subpel_x_qn != 0 as core::ffi::c_int;
    let need_y: bool = subpel_y_qn != 0 as core::ffi::c_int;
    if !need_x && !need_y {
        aom_convolve_copy_c(
            src,
            src_stride as ptrdiff_t,
            dst,
            dst_stride as ptrdiff_t,
            w,
            h,
        );
    } else if need_x as core::ffi::c_int != 0 && !need_y {
        av1_convolve_x_sr_c(
            src,
            src_stride,
            dst,
            dst_stride,
            w,
            h,
            filter_params_x,
            subpel_x_qn,
            conv_params,
        );
    } else if !need_x && need_y as core::ffi::c_int != 0 {
        av1_convolve_y_sr_c(
            src,
            src_stride,
            dst,
            dst_stride,
            w,
            h,
            filter_params_y,
            subpel_y_qn,
        );
    } else {
        if !(need_x as core::ffi::c_int != 0 && need_y as core::ffi::c_int != 0)
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 26],
                    [core::ffi::c_char; 26],
                >(*b"convolve_2d_facade_single\0"))
                    .as_ptr(),
                b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
                632 as core::ffi::c_int,
                b"need_x && need_y\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        av1_convolve_2d_sr_c(
            src,
            src_stride,
            dst,
            dst_stride,
            w,
            h,
            filter_params_x,
            filter_params_y,
            subpel_x_qn,
            subpel_y_qn,
            conv_params,
        );
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_convolve_2d_facade(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut dst: *mut uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut interp_filters: *mut *const InterpFilterParams,
    subpel_x_qn: core::ffi::c_int,
    mut x_step_q4: core::ffi::c_int,
    subpel_y_qn: core::ffi::c_int,
    mut y_step_q4: core::ffi::c_int,
    mut scaled: core::ffi::c_int,
    mut conv_params: *mut ConvolveParams,
) {
    let mut filter_params_x: *const InterpFilterParams = *interp_filters
        .offset(0 as core::ffi::c_int as isize);
    let mut filter_params_y: *const InterpFilterParams = *interp_filters
        .offset(1 as core::ffi::c_int as isize);
    if (*filter_params_x).taps as core::ffi::c_int == 2 as core::ffi::c_int
        || (*filter_params_y).taps as core::ffi::c_int == 2 as core::ffi::c_int
    {
        if !((*filter_params_x).taps as core::ffi::c_int == 2 as core::ffi::c_int
            && (*filter_params_y).taps as core::ffi::c_int == 2 as core::ffi::c_int)
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 23],
                    [core::ffi::c_char; 23],
                >(*b"av1_convolve_2d_facade\0"))
                    .as_ptr(),
                b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
                655 as core::ffi::c_int,
                b"filter_params_x->taps == 2 && filter_params_y->taps == 2\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        if (scaled != 0) as core::ffi::c_int as core::ffi::c_long != 0 {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 23],
                    [core::ffi::c_char; 23],
                >(*b"av1_convolve_2d_facade\0"))
                    .as_ptr(),
                b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
                656 as core::ffi::c_int,
                b"!scaled\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        if subpel_x_qn != 0 && subpel_y_qn != 0 {
            av1_convolve_2d_sr_intrabc_c(
                src,
                src_stride,
                dst,
                dst_stride,
                w,
                h,
                filter_params_x,
                filter_params_y,
                subpel_x_qn,
                subpel_y_qn,
                conv_params,
            );
            return;
        } else if subpel_x_qn != 0 {
            av1_convolve_x_sr_intrabc_c(
                src,
                src_stride,
                dst,
                dst_stride,
                w,
                h,
                filter_params_x,
                subpel_x_qn,
                conv_params,
            );
            return;
        } else if subpel_y_qn != 0 {
            av1_convolve_y_sr_intrabc_c(
                src,
                src_stride,
                dst,
                dst_stride,
                w,
                h,
                filter_params_y,
                subpel_y_qn,
            );
            return;
        }
    }
    if scaled != 0 {
        convolve_2d_scale_wrapper(
            src,
            src_stride,
            dst,
            dst_stride,
            w,
            h,
            filter_params_x,
            filter_params_y,
            subpel_x_qn,
            x_step_q4,
            subpel_y_qn,
            y_step_q4,
            conv_params,
        );
    } else if (*conv_params).is_compound != 0 {
        convolve_2d_facade_compound(
            src,
            src_stride,
            dst,
            dst_stride,
            w,
            h,
            filter_params_x,
            filter_params_y,
            subpel_x_qn,
            subpel_y_qn,
            conv_params,
        );
    } else {
        convolve_2d_facade_single(
            src,
            src_stride,
            dst,
            dst_stride,
            w,
            h,
            filter_params_x,
            filter_params_y,
            subpel_x_qn,
            subpel_y_qn,
            conv_params,
        );
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_highbd_convolve_x_sr_c(
    mut src: *const uint16_t,
    mut src_stride: core::ffi::c_int,
    mut dst: *mut uint16_t,
    mut dst_stride: core::ffi::c_int,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut filter_params_x: *const InterpFilterParams,
    subpel_x_qn: core::ffi::c_int,
    mut conv_params: *mut ConvolveParams,
    mut bd: core::ffi::c_int,
) {
    let fo_horiz: core::ffi::c_int = (*filter_params_x).taps as core::ffi::c_int
        / 2 as core::ffi::c_int - 1 as core::ffi::c_int;
    let bits: core::ffi::c_int = FILTER_BITS - (*conv_params).round_0;
    if !(bits >= 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 27],
                [core::ffi::c_char; 27],
            >(*b"av1_highbd_convolve_x_sr_c\0"))
                .as_ptr(),
            b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
            697 as core::ffi::c_int,
            b"bits >= 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !(7 as core::ffi::c_int - (*conv_params).round_1 >= 0 as core::ffi::c_int
        || (*conv_params).round_0 + (*conv_params).round_1
            == 2 as core::ffi::c_int * 7 as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 27],
                [core::ffi::c_char; 27],
            >(*b"av1_highbd_convolve_x_sr_c\0"))
                .as_ptr(),
            b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
            699 as core::ffi::c_int,
            b"(FILTER_BITS - conv_params->round_1) >= 0 || ((conv_params->round_0 + conv_params->round_1) == 2 * FILTER_BITS)\0"
                as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let mut x_filter: *const int16_t = av1_get_interp_filter_subpel_kernel(
        filter_params_x,
        subpel_x_qn & SUBPEL_MASK,
    );
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    while y < h {
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < w {
            let mut res: int32_t = 0 as int32_t;
            let mut k: core::ffi::c_int = 0 as core::ffi::c_int;
            while k < (*filter_params_x).taps as core::ffi::c_int {
                res = (res as core::ffi::c_int
                    + *x_filter.offset(k as isize) as core::ffi::c_int
                        * *src.offset((y * src_stride + x - fo_horiz + k) as isize)
                            as core::ffi::c_int) as int32_t;
                k += 1;
            }
            res = res
                + ((1 as int32_t) << (*conv_params).round_0 >> 1 as core::ffi::c_int)
                >> (*conv_params).round_0;
            *dst.offset((y * dst_stride + x) as isize) = clip_pixel_highbd(
                res as core::ffi::c_int
                    + ((1 as core::ffi::c_int) << bits >> 1 as core::ffi::c_int) >> bits,
                bd,
            );
            x += 1;
        }
        y += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_highbd_convolve_y_sr_c(
    mut src: *const uint16_t,
    mut src_stride: core::ffi::c_int,
    mut dst: *mut uint16_t,
    mut dst_stride: core::ffi::c_int,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut filter_params_y: *const InterpFilterParams,
    subpel_y_qn: core::ffi::c_int,
    mut bd: core::ffi::c_int,
) {
    let fo_vert: core::ffi::c_int = (*filter_params_y).taps as core::ffi::c_int
        / 2 as core::ffi::c_int - 1 as core::ffi::c_int;
    let mut y_filter: *const int16_t = av1_get_interp_filter_subpel_kernel(
        filter_params_y,
        subpel_y_qn & SUBPEL_MASK,
    );
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    while y < h {
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < w {
            let mut res: int32_t = 0 as int32_t;
            let mut k: core::ffi::c_int = 0 as core::ffi::c_int;
            while k < (*filter_params_y).taps as core::ffi::c_int {
                res = (res as core::ffi::c_int
                    + *y_filter.offset(k as isize) as core::ffi::c_int
                        * *src.offset(((y - fo_vert + k) * src_stride + x) as isize)
                            as core::ffi::c_int) as int32_t;
                k += 1;
            }
            *dst.offset((y * dst_stride + x) as isize) = clip_pixel_highbd(
                res as core::ffi::c_int
                    + ((1 as core::ffi::c_int) << 7 as core::ffi::c_int
                        >> 1 as core::ffi::c_int) >> 7 as core::ffi::c_int,
                bd,
            );
            x += 1;
        }
        y += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_highbd_convolve_2d_sr_c(
    mut src: *const uint16_t,
    mut src_stride: core::ffi::c_int,
    mut dst: *mut uint16_t,
    mut dst_stride: core::ffi::c_int,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut filter_params_x: *const InterpFilterParams,
    mut filter_params_y: *const InterpFilterParams,
    subpel_x_qn: core::ffi::c_int,
    subpel_y_qn: core::ffi::c_int,
    mut conv_params: *mut ConvolveParams,
    mut bd: core::ffi::c_int,
) {
    let mut im_block: [int16_t; 17792] = [0; 17792];
    let mut im_h: core::ffi::c_int = h + (*filter_params_y).taps as core::ffi::c_int
        - 1 as core::ffi::c_int;
    let mut im_stride: core::ffi::c_int = w;
    if !(w <= (1 as core::ffi::c_int) << 7 as core::ffi::c_int
        && h <= (1 as core::ffi::c_int) << 7 as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 28],
                [core::ffi::c_char; 28],
            >(*b"av1_highbd_convolve_2d_sr_c\0"))
                .as_ptr(),
            b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
            746 as core::ffi::c_int,
            b"w <= MAX_SB_SIZE && h <= MAX_SB_SIZE\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    let fo_vert: core::ffi::c_int = (*filter_params_y).taps as core::ffi::c_int
        / 2 as core::ffi::c_int - 1 as core::ffi::c_int;
    let fo_horiz: core::ffi::c_int = (*filter_params_x).taps as core::ffi::c_int
        / 2 as core::ffi::c_int - 1 as core::ffi::c_int;
    let bits: core::ffi::c_int = FILTER_BITS * 2 as core::ffi::c_int
        - (*conv_params).round_0 - (*conv_params).round_1;
    if !(bits >= 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 28],
                [core::ffi::c_char; 28],
            >(*b"av1_highbd_convolve_2d_sr_c\0"))
                .as_ptr(),
            b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
            751 as core::ffi::c_int,
            b"bits >= 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let mut src_horiz: *const uint16_t = src.offset(-((fo_vert * src_stride) as isize));
    let mut x_filter: *const int16_t = av1_get_interp_filter_subpel_kernel(
        filter_params_x,
        subpel_x_qn & SUBPEL_MASK,
    );
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    while y < im_h {
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < w {
            let mut sum: int32_t = (1 as int32_t)
                << bd + FILTER_BITS - 1 as core::ffi::c_int;
            let mut k: core::ffi::c_int = 0 as core::ffi::c_int;
            while k < (*filter_params_x).taps as core::ffi::c_int {
                sum = (sum as core::ffi::c_int
                    + *x_filter.offset(k as isize) as core::ffi::c_int
                        * *src_horiz.offset((y * src_stride + x - fo_horiz + k) as isize)
                            as core::ffi::c_int) as int32_t;
                k += 1;
            }
            if !((*filter_params_x).taps as core::ffi::c_int > 8 as core::ffi::c_int
                || 0 as int32_t <= sum
                    && sum
                        < (1 as int32_t)
                            << bd + 7 as core::ffi::c_int + 1 as core::ffi::c_int)
                as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 28],
                        [core::ffi::c_char; 28],
                    >(*b"av1_highbd_convolve_2d_sr_c\0"))
                        .as_ptr(),
                    b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
                    764 as core::ffi::c_int,
                    b"filter_params_x->taps > 8 || (0 <= sum && sum < (1 << (bd + FILTER_BITS + 1)))\0"
                        as *const u8 as *const core::ffi::c_char,
                );
            } else {};
            im_block[(y * im_stride + x) as usize] = (sum
                + ((1 as int32_t) << (*conv_params).round_0 >> 1 as core::ffi::c_int)
                >> (*conv_params).round_0) as int16_t;
            x += 1;
        }
        y += 1;
    }
    let mut src_vert: *mut int16_t = im_block
        .as_mut_ptr()
        .offset((fo_vert * im_stride) as isize);
    let mut y_filter: *const int16_t = av1_get_interp_filter_subpel_kernel(
        filter_params_y,
        subpel_y_qn & SUBPEL_MASK,
    );
    let offset_bits: core::ffi::c_int = bd + 2 as core::ffi::c_int * FILTER_BITS
        - (*conv_params).round_0;
    let mut y_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while y_0 < h {
        let mut x_0: core::ffi::c_int = 0 as core::ffi::c_int;
        while x_0 < w {
            let mut sum_0: int32_t = (1 as int32_t) << offset_bits;
            let mut k_0: core::ffi::c_int = 0 as core::ffi::c_int;
            while k_0 < (*filter_params_y).taps as core::ffi::c_int {
                sum_0 = (sum_0 as core::ffi::c_int
                    + *y_filter.offset(k_0 as isize) as core::ffi::c_int
                        * *src_vert
                            .offset(((y_0 - fo_vert + k_0) * im_stride + x_0) as isize)
                            as core::ffi::c_int) as int32_t;
                k_0 += 1;
            }
            if !((*filter_params_y).taps as core::ffi::c_int > 8 as core::ffi::c_int
                || 0 as int32_t <= sum_0
                    && sum_0 < (1 as int32_t) << offset_bits + 2 as core::ffi::c_int)
                as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 28],
                        [core::ffi::c_char; 28],
                    >(*b"av1_highbd_convolve_2d_sr_c\0"))
                        .as_ptr(),
                    b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
                    782 as core::ffi::c_int,
                    b"filter_params_y->taps > 8 || (0 <= sum && sum < (1 << (offset_bits + 2)))\0"
                        as *const u8 as *const core::ffi::c_char,
                );
            } else {};
            let mut res: int32_t = (sum_0
                + ((1 as int32_t) << (*conv_params).round_1 >> 1 as core::ffi::c_int)
                >> (*conv_params).round_1)
                - (((1 as int32_t) << offset_bits - (*conv_params).round_1)
                    + ((1 as int32_t)
                        << offset_bits - (*conv_params).round_1
                            - 1 as core::ffi::c_int));
            *dst.offset((y_0 * dst_stride + x_0) as isize) = clip_pixel_highbd(
                res as core::ffi::c_int
                    + ((1 as core::ffi::c_int) << bits >> 1 as core::ffi::c_int) >> bits,
                bd,
            );
            x_0 += 1;
        }
        y_0 += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_highbd_convolve_2d_sr_intrabc_c(
    mut src: *const uint16_t,
    mut src_stride: core::ffi::c_int,
    mut dst: *mut uint16_t,
    mut dst_stride: core::ffi::c_int,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut filter_params_x: *const InterpFilterParams,
    mut filter_params_y: *const InterpFilterParams,
    subpel_x_qn: core::ffi::c_int,
    subpel_y_qn: core::ffi::c_int,
    mut conv_params: *mut ConvolveParams,
    mut bd: core::ffi::c_int,
) {
    let bits: core::ffi::c_int = FILTER_BITS * 2 as core::ffi::c_int
        - (*conv_params).round_0 - (*conv_params).round_1;
    if !(bits >= 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 36],
                [core::ffi::c_char; 36],
            >(*b"av1_highbd_convolve_2d_sr_intrabc_c\0"))
                .as_ptr(),
            b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
            806 as core::ffi::c_int,
            b"bits >= 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !(subpel_x_qn == 8 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long
        != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 36],
                [core::ffi::c_char; 36],
            >(*b"av1_highbd_convolve_2d_sr_intrabc_c\0"))
                .as_ptr(),
            b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
            807 as core::ffi::c_int,
            b"subpel_x_qn == 8\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !(subpel_y_qn == 8 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long
        != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 36],
                [core::ffi::c_char; 36],
            >(*b"av1_highbd_convolve_2d_sr_intrabc_c\0"))
                .as_ptr(),
            b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
            808 as core::ffi::c_int,
            b"subpel_y_qn == 8\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !((*filter_params_x).taps as core::ffi::c_int == 2 as core::ffi::c_int
        && (*filter_params_y).taps as core::ffi::c_int == 2 as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 36],
                [core::ffi::c_char; 36],
            >(*b"av1_highbd_convolve_2d_sr_intrabc_c\0"))
                .as_ptr(),
            b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
            809 as core::ffi::c_int,
            b"filter_params_x->taps == 2 && filter_params_y->taps == 2\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    if !((*conv_params).round_0 + (*conv_params).round_1
        == 2 as core::ffi::c_int * 7 as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 36],
                [core::ffi::c_char; 36],
            >(*b"av1_highbd_convolve_2d_sr_intrabc_c\0"))
                .as_ptr(),
            b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
            810 as core::ffi::c_int,
            b"(conv_params->round_0 + conv_params->round_1) == 2 * FILTER_BITS\0"
                as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let mut im_block: [int16_t; 17792] = [0; 17792];
    let mut im_h: core::ffi::c_int = h + 1 as core::ffi::c_int;
    let mut im_stride: core::ffi::c_int = w;
    if !(w <= (1 as core::ffi::c_int) << 7 as core::ffi::c_int
        && h <= (1 as core::ffi::c_int) << 7 as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 36],
                [core::ffi::c_char; 36],
            >(*b"av1_highbd_convolve_2d_sr_intrabc_c\0"))
                .as_ptr(),
            b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
            820 as core::ffi::c_int,
            b"w <= MAX_SB_SIZE && h <= MAX_SB_SIZE\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    let mut im: *mut int16_t = im_block.as_mut_ptr();
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    while y < im_h {
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < w {
            let mut sum: int32_t = ((1 as int32_t)
                << bd + FILTER_BITS - 1 as core::ffi::c_int)
                + 64 as int32_t
                    * (*src.offset(x as isize) as int32_t
                        + *src.offset((x + 1 as core::ffi::c_int) as isize) as int32_t);
            if !(0 as int32_t <= sum
                && sum
                    < (1 as int32_t)
                        << bd + 7 as core::ffi::c_int + 1 as core::ffi::c_int)
                as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 36],
                        [core::ffi::c_char; 36],
                    >(*b"av1_highbd_convolve_2d_sr_intrabc_c\0"))
                        .as_ptr(),
                    b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
                    828 as core::ffi::c_int,
                    b"0 <= sum && sum < (1 << (bd + FILTER_BITS + 1))\0" as *const u8
                        as *const core::ffi::c_char,
                );
            } else {};
            sum = sum
                + ((1 as int32_t) << (*conv_params).round_0 >> 1 as core::ffi::c_int)
                >> (*conv_params).round_0;
            *im.offset(x as isize) = sum as int16_t;
            x += 1;
        }
        src = src.offset(src_stride as isize);
        im = im.offset(im_stride as isize);
        y += 1;
    }
    let mut src_vert: *mut int16_t = im_block.as_mut_ptr();
    let offset_bits: core::ffi::c_int = bd + 2 as core::ffi::c_int * FILTER_BITS
        - (*conv_params).round_0;
    let mut y_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while y_0 < h {
        let mut x_0: core::ffi::c_int = 0 as core::ffi::c_int;
        while x_0 < w {
            let sum_0: int32_t = ((1 as int32_t) << offset_bits)
                + 64 as int32_t
                    * (*src_vert.offset(x_0 as isize) as int32_t
                        + *src_vert.offset((im_stride + x_0) as isize) as int32_t);
            if !(0 as int32_t <= sum_0
                && sum_0 < (1 as int32_t) << offset_bits + 2 as core::ffi::c_int)
                as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 36],
                        [core::ffi::c_char; 36],
                    >(*b"av1_highbd_convolve_2d_sr_intrabc_c\0"))
                        .as_ptr(),
                    b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
                    844 as core::ffi::c_int,
                    b"0 <= sum && sum < (1 << (offset_bits + 2))\0" as *const u8
                        as *const core::ffi::c_char,
                );
            } else {};
            let res: int32_t = (sum_0
                + ((1 as int32_t) << (*conv_params).round_1 >> 1 as core::ffi::c_int)
                >> (*conv_params).round_1)
                - (((1 as int32_t) << offset_bits - (*conv_params).round_1)
                    + ((1 as int32_t)
                        << offset_bits - (*conv_params).round_1
                            - 1 as core::ffi::c_int));
            *dst.offset(x_0 as isize) = clip_pixel_highbd(
                res as core::ffi::c_int
                    + ((1 as core::ffi::c_int) << bits >> 1 as core::ffi::c_int) >> bits,
                bd,
            );
            x_0 += 1;
        }
        src_vert = src_vert.offset(im_stride as isize);
        dst = dst.offset(dst_stride as isize);
        y_0 += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_highbd_convolve_y_sr_intrabc_c(
    mut src: *const uint16_t,
    mut src_stride: core::ffi::c_int,
    mut dst: *mut uint16_t,
    mut dst_stride: core::ffi::c_int,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut filter_params_y: *const InterpFilterParams,
    subpel_y_qn: core::ffi::c_int,
    mut bd: core::ffi::c_int,
) {
    if !(subpel_y_qn == 8 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long
        != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 35],
                [core::ffi::c_char; 35],
            >(*b"av1_highbd_convolve_y_sr_intrabc_c\0"))
                .as_ptr(),
            b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
            862 as core::ffi::c_int,
            b"subpel_y_qn == 8\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !((*filter_params_y).taps as core::ffi::c_int == 2 as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 35],
                [core::ffi::c_char; 35],
            >(*b"av1_highbd_convolve_y_sr_intrabc_c\0"))
                .as_ptr(),
            b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
            863 as core::ffi::c_int,
            b"filter_params_y->taps == 2\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    while y < h {
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < w {
            let res: int32_t = *src.offset(x as isize) as int32_t
                + *src.offset((src_stride + x) as isize) as int32_t;
            *dst.offset(x as isize) = clip_pixel_highbd(
                res as core::ffi::c_int
                    + ((1 as core::ffi::c_int) << 1 as core::ffi::c_int
                        >> 1 as core::ffi::c_int) >> 1 as core::ffi::c_int,
                bd,
            );
            x += 1;
        }
        src = src.offset(src_stride as isize);
        dst = dst.offset(dst_stride as isize);
        y += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_highbd_convolve_x_sr_intrabc_c(
    mut src: *const uint16_t,
    mut src_stride: core::ffi::c_int,
    mut dst: *mut uint16_t,
    mut dst_stride: core::ffi::c_int,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut filter_params_x: *const InterpFilterParams,
    subpel_x_qn: core::ffi::c_int,
    mut conv_params: *mut ConvolveParams,
    mut bd: core::ffi::c_int,
) {
    let bits: core::ffi::c_int = FILTER_BITS - (*conv_params).round_0;
    if !(bits >= 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 35],
                [core::ffi::c_char; 35],
            >(*b"av1_highbd_convolve_x_sr_intrabc_c\0"))
                .as_ptr(),
            b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
            886 as core::ffi::c_int,
            b"bits >= 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !(subpel_x_qn == 8 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long
        != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 35],
                [core::ffi::c_char; 35],
            >(*b"av1_highbd_convolve_x_sr_intrabc_c\0"))
                .as_ptr(),
            b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
            887 as core::ffi::c_int,
            b"subpel_x_qn == 8\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !((*filter_params_x).taps as core::ffi::c_int == 2 as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 35],
                [core::ffi::c_char; 35],
            >(*b"av1_highbd_convolve_x_sr_intrabc_c\0"))
                .as_ptr(),
            b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
            888 as core::ffi::c_int,
            b"filter_params_x->taps == 2\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !((*conv_params).round_0 + (*conv_params).round_1
        == 2 as core::ffi::c_int * 7 as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 35],
                [core::ffi::c_char; 35],
            >(*b"av1_highbd_convolve_x_sr_intrabc_c\0"))
                .as_ptr(),
            b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
            889 as core::ffi::c_int,
            b"(conv_params->round_0 + conv_params->round_1) == 2 * FILTER_BITS\0"
                as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    while y < h {
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < w {
            let mut res: int32_t = 64 as int32_t
                * (*src.offset(x as isize) as int32_t
                    + *src.offset((x + 1 as core::ffi::c_int) as isize) as int32_t);
            res = res
                + ((1 as int32_t) << (*conv_params).round_0 >> 1 as core::ffi::c_int)
                >> (*conv_params).round_0;
            *dst.offset(x as isize) = clip_pixel_highbd(
                res as core::ffi::c_int
                    + ((1 as core::ffi::c_int) << bits >> 1 as core::ffi::c_int) >> bits,
                bd,
            );
            x += 1;
        }
        src = src.offset(src_stride as isize);
        dst = dst.offset(dst_stride as isize);
        y += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_highbd_dist_wtd_convolve_2d_c(
    mut src: *const uint16_t,
    mut src_stride: core::ffi::c_int,
    mut dst: *mut uint16_t,
    mut dst_stride: core::ffi::c_int,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut filter_params_x: *const InterpFilterParams,
    mut filter_params_y: *const InterpFilterParams,
    subpel_x_qn: core::ffi::c_int,
    subpel_y_qn: core::ffi::c_int,
    mut conv_params: *mut ConvolveParams,
    mut bd: core::ffi::c_int,
) {
    let mut x: core::ffi::c_int = 0;
    let mut y: core::ffi::c_int = 0;
    let mut k: core::ffi::c_int = 0;
    let mut im_block: [int16_t; 17792] = [0; 17792];
    let mut dst16: *mut CONV_BUF_TYPE = (*conv_params).dst;
    let mut dst16_stride: core::ffi::c_int = (*conv_params).dst_stride;
    let mut im_h: core::ffi::c_int = h + (*filter_params_y).taps as core::ffi::c_int
        - 1 as core::ffi::c_int;
    let mut im_stride: core::ffi::c_int = w;
    let fo_vert: core::ffi::c_int = (*filter_params_y).taps as core::ffi::c_int
        / 2 as core::ffi::c_int - 1 as core::ffi::c_int;
    let fo_horiz: core::ffi::c_int = (*filter_params_x).taps as core::ffi::c_int
        / 2 as core::ffi::c_int - 1 as core::ffi::c_int;
    let round_bits: core::ffi::c_int = 2 as core::ffi::c_int * FILTER_BITS
        - (*conv_params).round_0 - (*conv_params).round_1;
    if !(round_bits >= 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long
        != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 34],
                [core::ffi::c_char; 34],
            >(*b"av1_highbd_dist_wtd_convolve_2d_c\0"))
                .as_ptr(),
            b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
            921 as core::ffi::c_int,
            b"round_bits >= 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let mut src_horiz: *const uint16_t = src.offset(-((fo_vert * src_stride) as isize));
    let mut x_filter: *const int16_t = av1_get_interp_filter_subpel_kernel(
        filter_params_x,
        subpel_x_qn & SUBPEL_MASK,
    );
    y = 0 as core::ffi::c_int;
    while y < im_h {
        x = 0 as core::ffi::c_int;
        while x < w {
            let mut sum: int32_t = (1 as int32_t)
                << bd + FILTER_BITS - 1 as core::ffi::c_int;
            k = 0 as core::ffi::c_int;
            while k < (*filter_params_x).taps as core::ffi::c_int {
                sum = (sum as core::ffi::c_int
                    + *x_filter.offset(k as isize) as core::ffi::c_int
                        * *src_horiz.offset((y * src_stride + x - fo_horiz + k) as isize)
                            as core::ffi::c_int) as int32_t;
                k += 1;
            }
            if !((*filter_params_x).taps as core::ffi::c_int > 8 as core::ffi::c_int
                || 0 as int32_t <= sum
                    && sum
                        < (1 as int32_t)
                            << bd + 7 as core::ffi::c_int + 1 as core::ffi::c_int)
                as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 34],
                        [core::ffi::c_char; 34],
                    >(*b"av1_highbd_dist_wtd_convolve_2d_c\0"))
                        .as_ptr(),
                    b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
                    934 as core::ffi::c_int,
                    b"filter_params_x->taps > 8 || (0 <= sum && sum < (1 << (bd + FILTER_BITS + 1)))\0"
                        as *const u8 as *const core::ffi::c_char,
                );
            } else {};
            im_block[(y * im_stride + x) as usize] = (sum
                + ((1 as int32_t) << (*conv_params).round_0 >> 1 as core::ffi::c_int)
                >> (*conv_params).round_0) as int16_t;
            x += 1;
        }
        y += 1;
    }
    let mut src_vert: *mut int16_t = im_block
        .as_mut_ptr()
        .offset((fo_vert * im_stride) as isize);
    let offset_bits: core::ffi::c_int = bd + 2 as core::ffi::c_int * FILTER_BITS
        - (*conv_params).round_0;
    let mut y_filter: *const int16_t = av1_get_interp_filter_subpel_kernel(
        filter_params_y,
        subpel_y_qn & SUBPEL_MASK,
    );
    y = 0 as core::ffi::c_int;
    while y < h {
        x = 0 as core::ffi::c_int;
        while x < w {
            let mut sum_0: int32_t = (1 as int32_t) << offset_bits;
            k = 0 as core::ffi::c_int;
            while k < (*filter_params_y).taps as core::ffi::c_int {
                sum_0 = (sum_0 as core::ffi::c_int
                    + *y_filter.offset(k as isize) as core::ffi::c_int
                        * *src_vert.offset(((y - fo_vert + k) * im_stride + x) as isize)
                            as core::ffi::c_int) as int32_t;
                k += 1;
            }
            if !((*filter_params_y).taps as core::ffi::c_int > 8 as core::ffi::c_int
                || 0 as int32_t <= sum_0
                    && sum_0 < (1 as int32_t) << offset_bits + 2 as core::ffi::c_int)
                as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 34],
                        [core::ffi::c_char; 34],
                    >(*b"av1_highbd_dist_wtd_convolve_2d_c\0"))
                        .as_ptr(),
                    b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
                    953 as core::ffi::c_int,
                    b"filter_params_y->taps > 8 || (0 <= sum && sum < (1 << (offset_bits + 2)))\0"
                        as *const u8 as *const core::ffi::c_char,
                );
            } else {};
            let mut res: CONV_BUF_TYPE = (sum_0
                + ((1 as int32_t) << (*conv_params).round_1 >> 1 as core::ffi::c_int)
                >> (*conv_params).round_1) as CONV_BUF_TYPE;
            if (*conv_params).do_average != 0 {
                let mut tmp: int32_t = *dst16.offset((y * dst16_stride + x) as isize)
                    as int32_t;
                if (*conv_params).use_dist_wtd_comp_avg != 0 {
                    tmp = tmp * (*conv_params).fwd_offset as int32_t
                        + res as int32_t * (*conv_params).bck_offset as int32_t;
                    tmp = tmp >> DIST_PRECISION_BITS;
                } else {
                    tmp = (tmp as core::ffi::c_int + res as core::ffi::c_int) as int32_t;
                    tmp = tmp >> 1 as core::ffi::c_int;
                }
                tmp = (tmp as core::ffi::c_int
                    - (((1 as core::ffi::c_int) << offset_bits - (*conv_params).round_1)
                        + ((1 as core::ffi::c_int)
                            << offset_bits - (*conv_params).round_1
                                - 1 as core::ffi::c_int))) as int32_t;
                *dst.offset((y * dst_stride + x) as isize) = clip_pixel_highbd(
                    tmp as core::ffi::c_int
                        + ((1 as core::ffi::c_int) << round_bits
                            >> 1 as core::ffi::c_int) >> round_bits,
                    bd,
                );
            } else {
                *dst16.offset((y * dst16_stride + x) as isize) = res;
            }
            x += 1;
        }
        y += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_highbd_dist_wtd_convolve_x_c(
    mut src: *const uint16_t,
    mut src_stride: core::ffi::c_int,
    mut dst: *mut uint16_t,
    mut dst_stride: core::ffi::c_int,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut filter_params_x: *const InterpFilterParams,
    subpel_x_qn: core::ffi::c_int,
    mut conv_params: *mut ConvolveParams,
    mut bd: core::ffi::c_int,
) {
    let mut dst16: *mut CONV_BUF_TYPE = (*conv_params).dst;
    let mut dst16_stride: core::ffi::c_int = (*conv_params).dst_stride;
    let fo_horiz: core::ffi::c_int = (*filter_params_x).taps as core::ffi::c_int
        / 2 as core::ffi::c_int - 1 as core::ffi::c_int;
    let bits: core::ffi::c_int = FILTER_BITS - (*conv_params).round_1;
    let offset_bits: core::ffi::c_int = bd + 2 as core::ffi::c_int * FILTER_BITS
        - (*conv_params).round_0;
    let round_offset: core::ffi::c_int = ((1 as core::ffi::c_int)
        << offset_bits - (*conv_params).round_1)
        + ((1 as core::ffi::c_int)
            << offset_bits - (*conv_params).round_1 - 1 as core::ffi::c_int);
    let round_bits: core::ffi::c_int = 2 as core::ffi::c_int * FILTER_BITS
        - (*conv_params).round_0 - (*conv_params).round_1;
    if !(round_bits >= 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long
        != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 33],
                [core::ffi::c_char; 33],
            >(*b"av1_highbd_dist_wtd_convolve_x_c\0"))
                .as_ptr(),
            b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
            990 as core::ffi::c_int,
            b"round_bits >= 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !(bits >= 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 33],
                [core::ffi::c_char; 33],
            >(*b"av1_highbd_dist_wtd_convolve_x_c\0"))
                .as_ptr(),
            b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
            991 as core::ffi::c_int,
            b"bits >= 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let mut x_filter: *const int16_t = av1_get_interp_filter_subpel_kernel(
        filter_params_x,
        subpel_x_qn & SUBPEL_MASK,
    );
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    while y < h {
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < w {
            let mut res: int32_t = 0 as int32_t;
            let mut k: core::ffi::c_int = 0 as core::ffi::c_int;
            while k < (*filter_params_x).taps as core::ffi::c_int {
                res = (res as core::ffi::c_int
                    + *x_filter.offset(k as isize) as core::ffi::c_int
                        * *src.offset((y * src_stride + x - fo_horiz + k) as isize)
                            as core::ffi::c_int) as int32_t;
                k += 1;
            }
            res = ((1 as int32_t) << bits)
                * (res
                    + ((1 as int32_t) << (*conv_params).round_0 >> 1 as core::ffi::c_int)
                    >> (*conv_params).round_0);
            res = (res as core::ffi::c_int + round_offset) as int32_t;
            if (*conv_params).do_average != 0 {
                let mut tmp: int32_t = *dst16.offset((y * dst16_stride + x) as isize)
                    as int32_t;
                if (*conv_params).use_dist_wtd_comp_avg != 0 {
                    tmp = tmp * (*conv_params).fwd_offset as int32_t
                        + res * (*conv_params).bck_offset as int32_t;
                    tmp = tmp >> DIST_PRECISION_BITS;
                } else {
                    tmp += res;
                    tmp = tmp >> 1 as core::ffi::c_int;
                }
                tmp = (tmp as core::ffi::c_int - round_offset) as int32_t;
                *dst.offset((y * dst_stride + x) as isize) = clip_pixel_highbd(
                    tmp as core::ffi::c_int
                        + ((1 as core::ffi::c_int) << round_bits
                            >> 1 as core::ffi::c_int) >> round_bits,
                    bd,
                );
            } else {
                *dst16.offset((y * dst16_stride + x) as isize) = res as CONV_BUF_TYPE;
            }
            x += 1;
        }
        y += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_highbd_dist_wtd_convolve_y_c(
    mut src: *const uint16_t,
    mut src_stride: core::ffi::c_int,
    mut dst: *mut uint16_t,
    mut dst_stride: core::ffi::c_int,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut filter_params_y: *const InterpFilterParams,
    subpel_y_qn: core::ffi::c_int,
    mut conv_params: *mut ConvolveParams,
    mut bd: core::ffi::c_int,
) {
    let mut dst16: *mut CONV_BUF_TYPE = (*conv_params).dst;
    let mut dst16_stride: core::ffi::c_int = (*conv_params).dst_stride;
    let fo_vert: core::ffi::c_int = (*filter_params_y).taps as core::ffi::c_int
        / 2 as core::ffi::c_int - 1 as core::ffi::c_int;
    let bits: core::ffi::c_int = FILTER_BITS - (*conv_params).round_0;
    let offset_bits: core::ffi::c_int = bd + 2 as core::ffi::c_int * FILTER_BITS
        - (*conv_params).round_0;
    let round_offset: core::ffi::c_int = ((1 as core::ffi::c_int)
        << offset_bits - (*conv_params).round_1)
        + ((1 as core::ffi::c_int)
            << offset_bits - (*conv_params).round_1 - 1 as core::ffi::c_int);
    let round_bits: core::ffi::c_int = 2 as core::ffi::c_int * FILTER_BITS
        - (*conv_params).round_0 - (*conv_params).round_1;
    if !(round_bits >= 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long
        != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 33],
                [core::ffi::c_char; 33],
            >(*b"av1_highbd_dist_wtd_convolve_y_c\0"))
                .as_ptr(),
            b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
            1038 as core::ffi::c_int,
            b"round_bits >= 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !(bits >= 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 33],
                [core::ffi::c_char; 33],
            >(*b"av1_highbd_dist_wtd_convolve_y_c\0"))
                .as_ptr(),
            b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
            1039 as core::ffi::c_int,
            b"bits >= 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let mut y_filter: *const int16_t = av1_get_interp_filter_subpel_kernel(
        filter_params_y,
        subpel_y_qn & SUBPEL_MASK,
    );
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    while y < h {
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < w {
            let mut res: int32_t = 0 as int32_t;
            let mut k: core::ffi::c_int = 0 as core::ffi::c_int;
            while k < (*filter_params_y).taps as core::ffi::c_int {
                res = (res as core::ffi::c_int
                    + *y_filter.offset(k as isize) as core::ffi::c_int
                        * *src.offset(((y - fo_vert + k) * src_stride + x) as isize)
                            as core::ffi::c_int) as int32_t;
                k += 1;
            }
            res = (res as core::ffi::c_int * ((1 as core::ffi::c_int) << bits))
                as int32_t;
            res = (res
                + ((1 as int32_t) << (*conv_params).round_1 >> 1 as core::ffi::c_int)
                >> (*conv_params).round_1) + round_offset as int32_t;
            if (*conv_params).do_average != 0 {
                let mut tmp: int32_t = *dst16.offset((y * dst16_stride + x) as isize)
                    as int32_t;
                if (*conv_params).use_dist_wtd_comp_avg != 0 {
                    tmp = tmp * (*conv_params).fwd_offset as int32_t
                        + res * (*conv_params).bck_offset as int32_t;
                    tmp = tmp >> DIST_PRECISION_BITS;
                } else {
                    tmp += res;
                    tmp = tmp >> 1 as core::ffi::c_int;
                }
                tmp = (tmp as core::ffi::c_int - round_offset) as int32_t;
                *dst.offset((y * dst_stride + x) as isize) = clip_pixel_highbd(
                    tmp as core::ffi::c_int
                        + ((1 as core::ffi::c_int) << round_bits
                            >> 1 as core::ffi::c_int) >> round_bits,
                    bd,
                );
            } else {
                *dst16.offset((y * dst16_stride + x) as isize) = res as CONV_BUF_TYPE;
            }
            x += 1;
        }
        y += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_highbd_dist_wtd_convolve_2d_copy_c(
    mut src: *const uint16_t,
    mut src_stride: core::ffi::c_int,
    mut dst: *mut uint16_t,
    mut dst_stride: core::ffi::c_int,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut conv_params: *mut ConvolveParams,
    mut bd: core::ffi::c_int,
) {
    let mut dst16: *mut CONV_BUF_TYPE = (*conv_params).dst;
    let mut dst16_stride: core::ffi::c_int = (*conv_params).dst_stride;
    let bits: core::ffi::c_int = FILTER_BITS * 2 as core::ffi::c_int
        - (*conv_params).round_1 - (*conv_params).round_0;
    let offset_bits: core::ffi::c_int = bd + 2 as core::ffi::c_int * FILTER_BITS
        - (*conv_params).round_0;
    let round_offset: core::ffi::c_int = ((1 as core::ffi::c_int)
        << offset_bits - (*conv_params).round_1)
        + ((1 as core::ffi::c_int)
            << offset_bits - (*conv_params).round_1 - 1 as core::ffi::c_int);
    if !(bits >= 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 39],
                [core::ffi::c_char; 39],
            >(*b"av1_highbd_dist_wtd_convolve_2d_copy_c\0"))
                .as_ptr(),
            b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
            1083 as core::ffi::c_int,
            b"bits >= 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    while y < h {
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < w {
            let mut res: CONV_BUF_TYPE = ((*src.offset((y * src_stride + x) as isize)
                as core::ffi::c_int) << bits) as CONV_BUF_TYPE;
            res = (res as core::ffi::c_int + round_offset) as CONV_BUF_TYPE;
            if (*conv_params).do_average != 0 {
                let mut tmp: int32_t = *dst16.offset((y * dst16_stride + x) as isize)
                    as int32_t;
                if (*conv_params).use_dist_wtd_comp_avg != 0 {
                    tmp = tmp * (*conv_params).fwd_offset as int32_t
                        + res as int32_t * (*conv_params).bck_offset as int32_t;
                    tmp = tmp >> DIST_PRECISION_BITS;
                } else {
                    tmp = (tmp as core::ffi::c_int + res as core::ffi::c_int) as int32_t;
                    tmp = tmp >> 1 as core::ffi::c_int;
                }
                tmp = (tmp as core::ffi::c_int - round_offset) as int32_t;
                *dst.offset((y * dst_stride + x) as isize) = clip_pixel_highbd(
                    tmp as core::ffi::c_int
                        + ((1 as core::ffi::c_int) << bits >> 1 as core::ffi::c_int)
                        >> bits,
                    bd,
                );
            } else {
                *dst16.offset((y * dst16_stride + x) as isize) = res;
            }
            x += 1;
        }
        y += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_highbd_convolve_2d_scale_c(
    mut src: *const uint16_t,
    mut src_stride: core::ffi::c_int,
    mut dst: *mut uint16_t,
    mut dst_stride: core::ffi::c_int,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut filter_params_x: *const InterpFilterParams,
    mut filter_params_y: *const InterpFilterParams,
    subpel_x_qn: core::ffi::c_int,
    x_step_qn: core::ffi::c_int,
    subpel_y_qn: core::ffi::c_int,
    y_step_qn: core::ffi::c_int,
    mut conv_params: *mut ConvolveParams,
    mut bd: core::ffi::c_int,
) {
    let mut im_block: [int16_t; 34304] = [0; 34304];
    let mut im_h: core::ffi::c_int = ((h - 1 as core::ffi::c_int) * y_step_qn
        + subpel_y_qn >> SCALE_SUBPEL_BITS)
        + (*filter_params_y).taps as core::ffi::c_int;
    let mut im_stride: core::ffi::c_int = w;
    let fo_vert: core::ffi::c_int = (*filter_params_y).taps as core::ffi::c_int
        / 2 as core::ffi::c_int - 1 as core::ffi::c_int;
    let fo_horiz: core::ffi::c_int = (*filter_params_x).taps as core::ffi::c_int
        / 2 as core::ffi::c_int - 1 as core::ffi::c_int;
    let mut dst16: *mut CONV_BUF_TYPE = (*conv_params).dst;
    let dst16_stride: core::ffi::c_int = (*conv_params).dst_stride;
    let bits: core::ffi::c_int = FILTER_BITS * 2 as core::ffi::c_int
        - (*conv_params).round_0 - (*conv_params).round_1;
    if !(bits >= 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 31],
                [core::ffi::c_char; 31],
            >(*b"av1_highbd_convolve_2d_scale_c\0"))
                .as_ptr(),
            b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
            1125 as core::ffi::c_int,
            b"bits >= 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let mut src_horiz: *const uint16_t = src.offset(-((fo_vert * src_stride) as isize));
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    while y < im_h {
        let mut x_qn: core::ffi::c_int = subpel_x_qn;
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < w {
            let src_x: *const uint16_t = &*src_horiz
                .offset((x_qn >> SCALE_SUBPEL_BITS) as isize) as *const uint16_t;
            let x_filter_idx: core::ffi::c_int = (x_qn & SCALE_SUBPEL_MASK)
                >> SCALE_EXTRA_BITS;
            if !(x_filter_idx < (1 as core::ffi::c_int) << 4 as core::ffi::c_int)
                as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 31],
                        [core::ffi::c_char; 31],
                    >(*b"av1_highbd_convolve_2d_scale_c\0"))
                        .as_ptr(),
                    b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
                    1133 as core::ffi::c_int,
                    b"x_filter_idx < SUBPEL_SHIFTS\0" as *const u8
                        as *const core::ffi::c_char,
                );
            } else {};
            let mut x_filter: *const int16_t = av1_get_interp_filter_subpel_kernel(
                filter_params_x,
                x_filter_idx,
            );
            let mut sum: int32_t = (1 as int32_t)
                << bd + FILTER_BITS - 1 as core::ffi::c_int;
            let mut k: core::ffi::c_int = 0 as core::ffi::c_int;
            while k < (*filter_params_x).taps as core::ffi::c_int {
                sum = (sum as core::ffi::c_int
                    + *x_filter.offset(k as isize) as core::ffi::c_int
                        * *src_x.offset((k - fo_horiz) as isize) as core::ffi::c_int)
                    as int32_t;
                k += 1;
            }
            if !((*filter_params_x).taps as core::ffi::c_int > 8 as core::ffi::c_int
                || 0 as int32_t <= sum
                    && sum
                        < (1 as int32_t)
                            << bd + 7 as core::ffi::c_int + 1 as core::ffi::c_int)
                as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 31],
                        [core::ffi::c_char; 31],
                    >(*b"av1_highbd_convolve_2d_scale_c\0"))
                        .as_ptr(),
                    b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
                    1141 as core::ffi::c_int,
                    b"filter_params_x->taps > 8 || (0 <= sum && sum < (1 << (bd + FILTER_BITS + 1)))\0"
                        as *const u8 as *const core::ffi::c_char,
                );
            } else {};
            im_block[(y * im_stride + x) as usize] = (sum
                + ((1 as int32_t) << (*conv_params).round_0 >> 1 as core::ffi::c_int)
                >> (*conv_params).round_0) as int16_t;
            x += 1;
            x_qn += x_step_qn;
        }
        src_horiz = src_horiz.offset(src_stride as isize);
        y += 1;
    }
    let mut src_vert: *mut int16_t = im_block
        .as_mut_ptr()
        .offset((fo_vert * im_stride) as isize);
    let offset_bits: core::ffi::c_int = bd + 2 as core::ffi::c_int * FILTER_BITS
        - (*conv_params).round_0;
    let mut x_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while x_0 < w {
        let mut y_qn: core::ffi::c_int = subpel_y_qn;
        let mut y_0: core::ffi::c_int = 0 as core::ffi::c_int;
        while y_0 < h {
            let mut src_y: *const int16_t = &raw mut *src_vert
                .offset(((y_qn >> SCALE_SUBPEL_BITS) * im_stride) as isize)
                as *mut int16_t;
            let y_filter_idx: core::ffi::c_int = (y_qn & SCALE_SUBPEL_MASK)
                >> SCALE_EXTRA_BITS;
            if !(y_filter_idx < (1 as core::ffi::c_int) << 4 as core::ffi::c_int)
                as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 31],
                        [core::ffi::c_char; 31],
                    >(*b"av1_highbd_convolve_2d_scale_c\0"))
                        .as_ptr(),
                    b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
                    1156 as core::ffi::c_int,
                    b"y_filter_idx < SUBPEL_SHIFTS\0" as *const u8
                        as *const core::ffi::c_char,
                );
            } else {};
            let mut y_filter: *const int16_t = av1_get_interp_filter_subpel_kernel(
                filter_params_y,
                y_filter_idx,
            );
            let mut sum_0: int32_t = (1 as int32_t) << offset_bits;
            let mut k_0: core::ffi::c_int = 0 as core::ffi::c_int;
            while k_0 < (*filter_params_y).taps as core::ffi::c_int {
                sum_0 = (sum_0 as core::ffi::c_int
                    + *y_filter.offset(k_0 as isize) as core::ffi::c_int
                        * *src_y.offset(((k_0 - fo_vert) * im_stride) as isize)
                            as core::ffi::c_int) as int32_t;
                k_0 += 1;
            }
            if !((*filter_params_y).taps as core::ffi::c_int > 8 as core::ffi::c_int
                || 0 as int32_t <= sum_0
                    && sum_0 < (1 as int32_t) << offset_bits + 2 as core::ffi::c_int)
                as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 31],
                        [core::ffi::c_char; 31],
                    >(*b"av1_highbd_convolve_2d_scale_c\0"))
                        .as_ptr(),
                    b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
                    1164 as core::ffi::c_int,
                    b"filter_params_y->taps > 8 || (0 <= sum && sum < (1 << (offset_bits + 2)))\0"
                        as *const u8 as *const core::ffi::c_char,
                );
            } else {};
            let mut res: CONV_BUF_TYPE = (sum_0
                + ((1 as int32_t) << (*conv_params).round_1 >> 1 as core::ffi::c_int)
                >> (*conv_params).round_1) as CONV_BUF_TYPE;
            if (*conv_params).is_compound != 0 {
                if (*conv_params).do_average != 0 {
                    let mut tmp: int32_t = *dst16
                        .offset((y_0 * dst16_stride + x_0) as isize) as int32_t;
                    if (*conv_params).use_dist_wtd_comp_avg != 0 {
                        tmp = tmp * (*conv_params).fwd_offset as int32_t
                            + res as int32_t * (*conv_params).bck_offset as int32_t;
                        tmp = tmp >> DIST_PRECISION_BITS;
                    } else {
                        tmp = (tmp as core::ffi::c_int + res as core::ffi::c_int)
                            as int32_t;
                        tmp = tmp >> 1 as core::ffi::c_int;
                    }
                    tmp = tmp
                        - (((1 as int32_t) << offset_bits - (*conv_params).round_1)
                            + ((1 as int32_t)
                                << offset_bits - (*conv_params).round_1
                                    - 1 as core::ffi::c_int));
                    *dst.offset((y_0 * dst_stride + x_0) as isize) = clip_pixel_highbd(
                        tmp as core::ffi::c_int
                            + ((1 as core::ffi::c_int) << bits >> 1 as core::ffi::c_int)
                            >> bits,
                        bd,
                    );
                } else {
                    *dst16.offset((y_0 * dst16_stride + x_0) as isize) = res;
                }
            } else {
                let mut tmp_0: int32_t = res as int32_t
                    - (((1 as int32_t) << offset_bits - (*conv_params).round_1)
                        + ((1 as int32_t)
                            << offset_bits - (*conv_params).round_1
                                - 1 as core::ffi::c_int));
                *dst.offset((y_0 * dst_stride + x_0) as isize) = clip_pixel_highbd(
                    tmp_0 as core::ffi::c_int
                        + ((1 as core::ffi::c_int) << bits >> 1 as core::ffi::c_int)
                        >> bits,
                    bd,
                );
            }
            y_0 += 1;
            y_qn += y_step_qn;
        }
        src_vert = src_vert.offset(1);
        x_0 += 1;
    }
}
unsafe extern "C-unwind" fn highbd_convolve_2d_facade_compound(
    mut src: *const uint16_t,
    mut src_stride: core::ffi::c_int,
    mut dst: *mut uint16_t,
    mut dst_stride: core::ffi::c_int,
    w: core::ffi::c_int,
    h: core::ffi::c_int,
    mut filter_params_x: *const InterpFilterParams,
    mut filter_params_y: *const InterpFilterParams,
    subpel_x_qn: core::ffi::c_int,
    subpel_y_qn: core::ffi::c_int,
    mut conv_params: *mut ConvolveParams,
    mut bd: core::ffi::c_int,
) {
    let need_x: bool = subpel_x_qn != 0 as core::ffi::c_int;
    let need_y: bool = subpel_y_qn != 0 as core::ffi::c_int;
    if !need_x && !need_y {
        av1_highbd_dist_wtd_convolve_2d_copy_c(
            src,
            src_stride,
            dst,
            dst_stride,
            w,
            h,
            conv_params,
            bd,
        );
    } else if need_x as core::ffi::c_int != 0 && !need_y {
        av1_highbd_dist_wtd_convolve_x_c(
            src,
            src_stride,
            dst,
            dst_stride,
            w,
            h,
            filter_params_x,
            subpel_x_qn,
            conv_params,
            bd,
        );
    } else if !need_x && need_y as core::ffi::c_int != 0 {
        av1_highbd_dist_wtd_convolve_y_c(
            src,
            src_stride,
            dst,
            dst_stride,
            w,
            h,
            filter_params_y,
            subpel_y_qn,
            conv_params,
            bd,
        );
    } else {
        if !(need_x as core::ffi::c_int != 0 && need_y as core::ffi::c_int != 0)
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 35],
                    [core::ffi::c_char; 35],
                >(*b"highbd_convolve_2d_facade_compound\0"))
                    .as_ptr(),
                b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
                1215 as core::ffi::c_int,
                b"need_x && need_y\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        av1_highbd_dist_wtd_convolve_2d_c(
            src,
            src_stride,
            dst,
            dst_stride,
            w,
            h,
            filter_params_x,
            filter_params_y,
            subpel_x_qn,
            subpel_y_qn,
            conv_params,
            bd,
        );
    };
}
unsafe extern "C-unwind" fn highbd_convolve_2d_facade_single(
    mut src: *const uint16_t,
    mut src_stride: core::ffi::c_int,
    mut dst: *mut uint16_t,
    mut dst_stride: core::ffi::c_int,
    w: core::ffi::c_int,
    h: core::ffi::c_int,
    mut filter_params_x: *const InterpFilterParams,
    mut filter_params_y: *const InterpFilterParams,
    subpel_x_qn: core::ffi::c_int,
    subpel_y_qn: core::ffi::c_int,
    mut conv_params: *mut ConvolveParams,
    mut bd: core::ffi::c_int,
) {
    let need_x: bool = subpel_x_qn != 0 as core::ffi::c_int;
    let need_y: bool = subpel_y_qn != 0 as core::ffi::c_int;
    if !need_x && !need_y {
        aom_highbd_convolve_copy_c(
            src,
            src_stride as ptrdiff_t,
            dst,
            dst_stride as ptrdiff_t,
            w,
            h,
        );
    } else if need_x as core::ffi::c_int != 0 && !need_y {
        av1_highbd_convolve_x_sr_c(
            src,
            src_stride,
            dst,
            dst_stride,
            w,
            h,
            filter_params_x,
            subpel_x_qn,
            conv_params,
            bd,
        );
    } else if !need_x && need_y as core::ffi::c_int != 0 {
        av1_highbd_convolve_y_sr_c(
            src,
            src_stride,
            dst,
            dst_stride,
            w,
            h,
            filter_params_y,
            subpel_y_qn,
            bd,
        );
    } else {
        if !(need_x as core::ffi::c_int != 0 && need_y as core::ffi::c_int != 0)
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 33],
                    [core::ffi::c_char; 33],
                >(*b"highbd_convolve_2d_facade_single\0"))
                    .as_ptr(),
                b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
                1239 as core::ffi::c_int,
                b"need_x && need_y\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        av1_highbd_convolve_2d_sr_c(
            src,
            src_stride,
            dst,
            dst_stride,
            w,
            h,
            filter_params_x,
            filter_params_y,
            subpel_x_qn,
            subpel_y_qn,
            conv_params,
            bd,
        );
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_highbd_convolve_2d_facade(
    mut src8: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut dst8: *mut uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut interp_filters: *mut *const InterpFilterParams,
    subpel_x_qn: core::ffi::c_int,
    mut x_step_q4: core::ffi::c_int,
    subpel_y_qn: core::ffi::c_int,
    mut y_step_q4: core::ffi::c_int,
    mut scaled: core::ffi::c_int,
    mut conv_params: *mut ConvolveParams,
    mut bd: core::ffi::c_int,
) {
    let mut src: *const uint16_t = ((src8 as uintptr_t) << 1 as core::ffi::c_int)
        as *mut uint16_t;
    let mut filter_params_x: *const InterpFilterParams = *interp_filters
        .offset(0 as core::ffi::c_int as isize);
    let mut filter_params_y: *const InterpFilterParams = *interp_filters
        .offset(1 as core::ffi::c_int as isize);
    let mut dst: *mut uint16_t = ((dst8 as uintptr_t) << 1 as core::ffi::c_int)
        as *mut uint16_t;
    if (*filter_params_x).taps as core::ffi::c_int == 2 as core::ffi::c_int
        || (*filter_params_y).taps as core::ffi::c_int == 2 as core::ffi::c_int
    {
        if !((*filter_params_x).taps as core::ffi::c_int == 2 as core::ffi::c_int
            && (*filter_params_y).taps as core::ffi::c_int == 2 as core::ffi::c_int)
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 30],
                    [core::ffi::c_char; 30],
                >(*b"av1_highbd_convolve_2d_facade\0"))
                    .as_ptr(),
                b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
                1264 as core::ffi::c_int,
                b"filter_params_x->taps == 2 && filter_params_y->taps == 2\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        if (scaled != 0) as core::ffi::c_int as core::ffi::c_long != 0 {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 30],
                    [core::ffi::c_char; 30],
                >(*b"av1_highbd_convolve_2d_facade\0"))
                    .as_ptr(),
                b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
                1265 as core::ffi::c_int,
                b"!scaled\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        if subpel_x_qn != 0 && subpel_y_qn != 0 {
            av1_highbd_convolve_2d_sr_intrabc_c(
                src,
                src_stride,
                dst,
                dst_stride,
                w,
                h,
                filter_params_x,
                filter_params_y,
                subpel_x_qn,
                subpel_y_qn,
                conv_params,
                bd,
            );
            return;
        } else if subpel_x_qn != 0 {
            av1_highbd_convolve_x_sr_intrabc_c(
                src,
                src_stride,
                dst,
                dst_stride,
                w,
                h,
                filter_params_x,
                subpel_x_qn,
                conv_params,
                bd,
            );
            return;
        } else if subpel_y_qn != 0 {
            av1_highbd_convolve_y_sr_intrabc_c(
                src,
                src_stride,
                dst,
                dst_stride,
                w,
                h,
                filter_params_y,
                subpel_y_qn,
                bd,
            );
            return;
        }
    }
    if scaled != 0 {
        if (*conv_params).is_compound != 0 {
            if ((*conv_params).dst).is_null() as core::ffi::c_int as core::ffi::c_long
                != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 30],
                        [core::ffi::c_char; 30],
                    >(*b"av1_highbd_convolve_2d_facade\0"))
                        .as_ptr(),
                    b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
                    1285 as core::ffi::c_int,
                    b"conv_params->dst != NULL\0" as *const u8
                        as *const core::ffi::c_char,
                );
            } else {};
        }
        av1_highbd_convolve_2d_scale_c(
            src,
            src_stride,
            dst,
            dst_stride,
            w,
            h,
            filter_params_x,
            filter_params_y,
            subpel_x_qn,
            x_step_q4,
            subpel_y_qn,
            y_step_q4,
            conv_params,
            bd,
        );
    } else if (*conv_params).is_compound != 0 {
        highbd_convolve_2d_facade_compound(
            src,
            src_stride,
            dst,
            dst_stride,
            w,
            h,
            filter_params_x,
            filter_params_y,
            subpel_x_qn,
            subpel_y_qn,
            conv_params,
            bd,
        );
    } else {
        highbd_convolve_2d_facade_single(
            src,
            src_stride,
            dst,
            dst_stride,
            w,
            h,
            filter_params_x,
            filter_params_y,
            subpel_x_qn,
            subpel_y_qn,
            conv_params,
            bd,
        );
    };
}
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
unsafe extern "C-unwind" fn convolve_add_src_horiz_hip(
    mut src: *const uint8_t,
    mut src_stride: ptrdiff_t,
    mut dst: *mut uint16_t,
    mut dst_stride: ptrdiff_t,
    mut x_filters: *const InterpKernel,
    mut x0_q4: core::ffi::c_int,
    mut x_step_q4: core::ffi::c_int,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut round0_bits: core::ffi::c_int,
) {
    let bd: core::ffi::c_int = 8 as core::ffi::c_int;
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
            let rounding: core::ffi::c_int = ((*src_x
                .offset(
                    (SUBPEL_TAPS / 2 as core::ffi::c_int - 1 as core::ffi::c_int)
                        as isize,
                ) as core::ffi::c_int) << FILTER_BITS)
                + ((1 as core::ffi::c_int) << bd + FILTER_BITS - 1 as core::ffi::c_int);
            let sum: core::ffi::c_int = horz_scalar_product(src_x, x_filter)
                as core::ffi::c_int + rounding;
            *dst.offset(x as isize) = clamp(
                sum + ((1 as core::ffi::c_int) << round0_bits >> 1 as core::ffi::c_int)
                    >> round0_bits,
                0 as core::ffi::c_int,
                ((1 as core::ffi::c_int)
                    << bd + 1 as core::ffi::c_int + FILTER_BITS - round0_bits)
                    - 1 as core::ffi::c_int,
            ) as uint16_t;
            x_q4 += x_step_q4;
            x += 1;
        }
        src = src.offset(src_stride as isize);
        dst = dst.offset(dst_stride as isize);
        y += 1;
    }
}
unsafe extern "C-unwind" fn convolve_add_src_vert_hip(
    mut src: *const uint16_t,
    mut src_stride: ptrdiff_t,
    mut dst: *mut uint8_t,
    mut dst_stride: ptrdiff_t,
    mut y_filters: *const InterpKernel,
    mut y0_q4: core::ffi::c_int,
    mut y_step_q4: core::ffi::c_int,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut round1_bits: core::ffi::c_int,
) {
    let bd: core::ffi::c_int = 8 as core::ffi::c_int;
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
            let mut src_y: *const uint16_t = &*src
                .offset(((y_q4 >> SUBPEL_BITS) as ptrdiff_t * src_stride) as isize)
                as *const uint16_t;
            let y_filter: *const int16_t = (*y_filters
                .offset((y_q4 & SUBPEL_MASK) as isize))
                .as_ptr();
            let rounding: core::ffi::c_int = ((*src_y
                .offset(
                    ((SUBPEL_TAPS / 2 as core::ffi::c_int - 1 as core::ffi::c_int)
                        as ptrdiff_t * src_stride) as isize,
                ) as core::ffi::c_int) << FILTER_BITS)
                - ((1 as core::ffi::c_int) << bd + round1_bits - 1 as core::ffi::c_int);
            let sum: core::ffi::c_int = highbd_vert_scalar_product(
                src_y,
                src_stride,
                y_filter,
            ) as core::ffi::c_int + rounding;
            *dst.offset((y as ptrdiff_t * dst_stride) as isize) = clip_pixel(
                sum + ((1 as core::ffi::c_int) << round1_bits >> 1 as core::ffi::c_int)
                    >> round1_bits,
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
pub unsafe extern "C-unwind" fn av1_wiener_convolve_add_src_c(
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
    mut conv_params: *const WienerConvolveParams,
) {
    let filters_x: *const InterpKernel = get_filter_base(filter_x)
        as *const InterpKernel;
    let x0_q4: core::ffi::c_int = get_filter_offset(filter_x, filters_x)
        as core::ffi::c_int;
    let filters_y: *const InterpKernel = get_filter_base(filter_y)
        as *const InterpKernel;
    let y0_q4: core::ffi::c_int = get_filter_offset(filter_y, filters_y)
        as core::ffi::c_int;
    let mut temp: [uint16_t; 33664] = [0; 33664];
    let intermediate_height: core::ffi::c_int = ((h - 1 as core::ffi::c_int) * y_step_q4
        + y0_q4 >> SUBPEL_BITS) + SUBPEL_TAPS - 1 as core::ffi::c_int;
    memset(
        temp.as_mut_ptr().offset((intermediate_height * MAX_SB_SIZE) as isize)
            as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        MAX_SB_SIZE as size_t,
    );
    if !(w <= (1 as core::ffi::c_int) << 7 as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 30],
                [core::ffi::c_char; 30],
            >(*b"av1_wiener_convolve_add_src_c\0"))
                .as_ptr(),
            b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
            1418 as core::ffi::c_int,
            b"w <= MAX_SB_SIZE\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !(h <= (1 as core::ffi::c_int) << 7 as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 30],
                [core::ffi::c_char; 30],
            >(*b"av1_wiener_convolve_add_src_c\0"))
                .as_ptr(),
            b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
            1419 as core::ffi::c_int,
            b"h <= MAX_SB_SIZE\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !(y_step_q4 <= 32 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long
        != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 30],
                [core::ffi::c_char; 30],
            >(*b"av1_wiener_convolve_add_src_c\0"))
                .as_ptr(),
            b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
            1420 as core::ffi::c_int,
            b"y_step_q4 <= 32\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !(x_step_q4 <= 32 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long
        != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 30],
                [core::ffi::c_char; 30],
            >(*b"av1_wiener_convolve_add_src_c\0"))
                .as_ptr(),
            b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
            1421 as core::ffi::c_int,
            b"x_step_q4 <= 32\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    convolve_add_src_horiz_hip(
        src
            .offset(
                -((src_stride
                    * (SUBPEL_TAPS / 2 as core::ffi::c_int - 1 as core::ffi::c_int)
                        as ptrdiff_t) as isize),
            ),
        src_stride,
        temp.as_mut_ptr(),
        MAX_SB_SIZE as ptrdiff_t,
        filters_x,
        x0_q4,
        x_step_q4,
        w,
        intermediate_height,
        (*conv_params).round_0,
    );
    convolve_add_src_vert_hip(
        temp
            .as_mut_ptr()
            .offset(
                (MAX_SB_SIZE
                    * (SUBPEL_TAPS / 2 as core::ffi::c_int - 1 as core::ffi::c_int))
                    as isize,
            ),
        MAX_SB_SIZE as ptrdiff_t,
        dst,
        dst_stride,
        filters_y,
        y0_q4,
        y_step_q4,
        w,
        h,
        (*conv_params).round_1,
    );
}
unsafe extern "C-unwind" fn highbd_convolve_add_src_horiz_hip(
    mut src8: *const uint8_t,
    mut src_stride: ptrdiff_t,
    mut dst: *mut uint16_t,
    mut dst_stride: ptrdiff_t,
    mut x_filters: *const InterpKernel,
    mut x0_q4: core::ffi::c_int,
    mut x_step_q4: core::ffi::c_int,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut round0_bits: core::ffi::c_int,
    mut bd: core::ffi::c_int,
) {
    let extraprec_clamp_limit: core::ffi::c_int = (1 as core::ffi::c_int)
        << bd + 1 as core::ffi::c_int + FILTER_BITS - round0_bits;
    let mut src: *mut uint16_t = ((src8 as uintptr_t) << 1 as core::ffi::c_int)
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
            let rounding: core::ffi::c_int = ((*src_x
                .offset(
                    (SUBPEL_TAPS / 2 as core::ffi::c_int - 1 as core::ffi::c_int)
                        as isize,
                ) as core::ffi::c_int) << FILTER_BITS)
                + ((1 as core::ffi::c_int) << bd + FILTER_BITS - 1 as core::ffi::c_int);
            let sum: core::ffi::c_int = highbd_horz_scalar_product(src_x, x_filter)
                as core::ffi::c_int + rounding;
            *dst.offset(x as isize) = clamp(
                sum + ((1 as core::ffi::c_int) << round0_bits >> 1 as core::ffi::c_int)
                    >> round0_bits,
                0 as core::ffi::c_int,
                extraprec_clamp_limit - 1 as core::ffi::c_int,
            ) as uint16_t;
            x_q4 += x_step_q4;
            x += 1;
        }
        src = src.offset(src_stride as isize);
        dst = dst.offset(dst_stride as isize);
        y += 1;
    }
}
unsafe extern "C-unwind" fn highbd_convolve_add_src_vert_hip(
    mut src: *const uint16_t,
    mut src_stride: ptrdiff_t,
    mut dst8: *mut uint8_t,
    mut dst_stride: ptrdiff_t,
    mut y_filters: *const InterpKernel,
    mut y0_q4: core::ffi::c_int,
    mut y_step_q4: core::ffi::c_int,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut round1_bits: core::ffi::c_int,
    mut bd: core::ffi::c_int,
) {
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
            let mut src_y: *const uint16_t = &*src
                .offset(((y_q4 >> SUBPEL_BITS) as ptrdiff_t * src_stride) as isize)
                as *const uint16_t;
            let y_filter: *const int16_t = (*y_filters
                .offset((y_q4 & SUBPEL_MASK) as isize))
                .as_ptr();
            let rounding: core::ffi::c_int = ((*src_y
                .offset(
                    ((SUBPEL_TAPS / 2 as core::ffi::c_int - 1 as core::ffi::c_int)
                        as ptrdiff_t * src_stride) as isize,
                ) as core::ffi::c_int) << FILTER_BITS)
                - ((1 as core::ffi::c_int) << bd + round1_bits - 1 as core::ffi::c_int);
            let sum: core::ffi::c_int = highbd_vert_scalar_product(
                src_y,
                src_stride,
                y_filter,
            ) as core::ffi::c_int + rounding;
            *dst.offset((y as ptrdiff_t * dst_stride) as isize) = clip_pixel_highbd(
                sum + ((1 as core::ffi::c_int) << round1_bits >> 1 as core::ffi::c_int)
                    >> round1_bits,
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
pub unsafe extern "C-unwind" fn av1_highbd_wiener_convolve_add_src_c(
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
    mut conv_params: *const WienerConvolveParams,
    mut bd: core::ffi::c_int,
) {
    let filters_x: *const InterpKernel = get_filter_base(filter_x)
        as *const InterpKernel;
    let x0_q4: core::ffi::c_int = get_filter_offset(filter_x, filters_x)
        as core::ffi::c_int;
    let filters_y: *const InterpKernel = get_filter_base(filter_y)
        as *const InterpKernel;
    let y0_q4: core::ffi::c_int = get_filter_offset(filter_y, filters_y)
        as core::ffi::c_int;
    let mut temp: [uint16_t; 33664] = [0; 33664];
    let intermediate_height: core::ffi::c_int = ((h - 1 as core::ffi::c_int) * y_step_q4
        + y0_q4 >> SUBPEL_BITS) + SUBPEL_TAPS;
    if !(w <= (1 as core::ffi::c_int) << 7 as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 37],
                [core::ffi::c_char; 37],
            >(*b"av1_highbd_wiener_convolve_add_src_c\0"))
                .as_ptr(),
            b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
            1499 as core::ffi::c_int,
            b"w <= MAX_SB_SIZE\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !(h <= (1 as core::ffi::c_int) << 7 as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 37],
                [core::ffi::c_char; 37],
            >(*b"av1_highbd_wiener_convolve_add_src_c\0"))
                .as_ptr(),
            b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
            1500 as core::ffi::c_int,
            b"h <= MAX_SB_SIZE\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !(y_step_q4 <= 32 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long
        != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 37],
                [core::ffi::c_char; 37],
            >(*b"av1_highbd_wiener_convolve_add_src_c\0"))
                .as_ptr(),
            b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
            1501 as core::ffi::c_int,
            b"y_step_q4 <= 32\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !(x_step_q4 <= 32 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long
        != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 37],
                [core::ffi::c_char; 37],
            >(*b"av1_highbd_wiener_convolve_add_src_c\0"))
                .as_ptr(),
            b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
            1502 as core::ffi::c_int,
            b"x_step_q4 <= 32\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !(bd + 7 as core::ffi::c_int - (*conv_params).round_0 + 2 as core::ffi::c_int
        <= 16 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 37],
                [core::ffi::c_char; 37],
            >(*b"av1_highbd_wiener_convolve_add_src_c\0"))
                .as_ptr(),
            b"convolve.c\0" as *const u8 as *const core::ffi::c_char,
            1503 as core::ffi::c_int,
            b"bd + FILTER_BITS - conv_params->round_0 + 2 <= 16\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    highbd_convolve_add_src_horiz_hip(
        src
            .offset(
                -((src_stride
                    * (SUBPEL_TAPS / 2 as core::ffi::c_int - 1 as core::ffi::c_int)
                        as ptrdiff_t) as isize),
            ),
        src_stride,
        temp.as_mut_ptr(),
        MAX_SB_SIZE as ptrdiff_t,
        filters_x,
        x0_q4,
        x_step_q4,
        w,
        intermediate_height,
        (*conv_params).round_0,
        bd,
    );
    highbd_convolve_add_src_vert_hip(
        temp
            .as_mut_ptr()
            .offset(
                (MAX_SB_SIZE
                    * (SUBPEL_TAPS / 2 as core::ffi::c_int - 1 as core::ffi::c_int))
                    as isize,
            ),
        MAX_SB_SIZE as ptrdiff_t,
        dst,
        dst_stride,
        filters_y,
        y0_q4,
        y_step_q4,
        w,
        h,
        (*conv_params).round_1,
        bd,
    );
}

extern "C" {
    fn memset(
        __b: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __len: size_t,
    ) -> *mut core::ffi::c_void;
}
