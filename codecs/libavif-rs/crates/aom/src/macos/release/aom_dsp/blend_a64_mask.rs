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

// Mechanically translated from upstream/aom/aom_dsp/blend_a64_mask.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
pub type int32_t = i32;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uintptr_t = usize;
pub type CONV_BUF_TYPE = uint16_t;
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
pub const AOM_BLEND_A64_ROUND_BITS: core::ffi::c_int = 6 as core::ffi::c_int;
pub const AOM_BLEND_A64_MAX_ALPHA: core::ffi::c_int = (1 as core::ffi::c_int)
    << AOM_BLEND_A64_ROUND_BITS;
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
unsafe extern "C-unwind" fn negative_to_zero(mut value: core::ffi::c_int) -> core::ffi::c_uint {
    return (value
        & !(value
            >> (::core::mem::size_of::<core::ffi::c_int>() as usize)
                .wrapping_mul(8 as usize)
                .wrapping_sub(1 as usize))) as core::ffi::c_uint;
}
pub const FILTER_BITS: core::ffi::c_int = 7 as core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_lowbd_blend_a64_d16_mask_c(
    mut dst: *mut uint8_t,
    mut dst_stride: uint32_t,
    mut src0: *const CONV_BUF_TYPE,
    mut src0_stride: uint32_t,
    mut src1: *const CONV_BUF_TYPE,
    mut src1_stride: uint32_t,
    mut mask: *const uint8_t,
    mut mask_stride: uint32_t,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut subw: core::ffi::c_int,
    mut subh: core::ffi::c_int,
    mut conv_params: *mut ConvolveParams,
) {
    let mut i: core::ffi::c_int = 0;
    let mut j: core::ffi::c_int = 0;
    let bd: core::ffi::c_int = 8 as core::ffi::c_int;
    let offset_bits: core::ffi::c_int = bd + 2 as core::ffi::c_int * FILTER_BITS
        - (*conv_params).round_0;
    let round_offset: core::ffi::c_int = ((1 as core::ffi::c_int)
        << offset_bits - (*conv_params).round_1)
        + ((1 as core::ffi::c_int)
            << offset_bits - (*conv_params).round_1 - 1 as core::ffi::c_int);
    let round_bits: core::ffi::c_int = 2 as core::ffi::c_int * FILTER_BITS
        - (*conv_params).round_0 - (*conv_params).round_1;
    if subw == 0 as core::ffi::c_int && subh == 0 as core::ffi::c_int {
        i = 0 as core::ffi::c_int;
        while i < h {
            j = 0 as core::ffi::c_int;
            while j < w {
                let mut res: int32_t = 0;
                let m: core::ffi::c_int = *mask
                    .offset(
                        (i as uint32_t)
                            .wrapping_mul(mask_stride)
                            .wrapping_add(j as uint32_t) as isize,
                    ) as core::ffi::c_int;
                res = m as int32_t
                    * *src0
                        .offset(
                            (i as uint32_t)
                                .wrapping_mul(src0_stride)
                                .wrapping_add(j as uint32_t) as isize,
                        ) as int32_t
                    + (AOM_BLEND_A64_MAX_ALPHA as int32_t - m as int32_t)
                        * *src1
                            .offset(
                                (i as uint32_t)
                                    .wrapping_mul(src1_stride)
                                    .wrapping_add(j as uint32_t) as isize,
                            ) as int32_t >> AOM_BLEND_A64_ROUND_BITS;
                res = (res as core::ffi::c_int - round_offset) as int32_t;
                *dst
                    .offset(
                        (i as uint32_t)
                            .wrapping_mul(dst_stride)
                            .wrapping_add(j as uint32_t) as isize,
                    ) = clip_pixel(
                    res as core::ffi::c_int
                        + ((1 as core::ffi::c_int) << round_bits
                            >> 1 as core::ffi::c_int) >> round_bits,
                );
                j += 1;
            }
            i += 1;
        }
    } else if subw == 1 as core::ffi::c_int && subh == 1 as core::ffi::c_int {
        i = 0 as core::ffi::c_int;
        while i < h {
            j = 0 as core::ffi::c_int;
            while j < w {
                let mut res_0: int32_t = 0;
                let m_0: core::ffi::c_int = *mask
                    .offset(
                        ((2 as core::ffi::c_int * i) as uint32_t)
                            .wrapping_mul(mask_stride)
                            .wrapping_add((2 as core::ffi::c_int * j) as uint32_t)
                            as isize,
                    ) as core::ffi::c_int
                    + *mask
                        .offset(
                            ((2 as core::ffi::c_int * i + 1 as core::ffi::c_int)
                                as uint32_t)
                                .wrapping_mul(mask_stride)
                                .wrapping_add((2 as core::ffi::c_int * j) as uint32_t)
                                as isize,
                        ) as core::ffi::c_int
                    + *mask
                        .offset(
                            ((2 as core::ffi::c_int * i) as uint32_t)
                                .wrapping_mul(mask_stride)
                                .wrapping_add(
                                    (2 as core::ffi::c_int * j + 1 as core::ffi::c_int)
                                        as uint32_t,
                                ) as isize,
                        ) as core::ffi::c_int
                    + *mask
                        .offset(
                            ((2 as core::ffi::c_int * i + 1 as core::ffi::c_int)
                                as uint32_t)
                                .wrapping_mul(mask_stride)
                                .wrapping_add(
                                    (2 as core::ffi::c_int * j + 1 as core::ffi::c_int)
                                        as uint32_t,
                                ) as isize,
                        ) as core::ffi::c_int
                    + ((1 as core::ffi::c_int) << 2 as core::ffi::c_int
                        >> 1 as core::ffi::c_int) >> 2 as core::ffi::c_int;
                res_0 = m_0 as int32_t
                    * *src0
                        .offset(
                            (i as uint32_t)
                                .wrapping_mul(src0_stride)
                                .wrapping_add(j as uint32_t) as isize,
                        ) as int32_t
                    + (AOM_BLEND_A64_MAX_ALPHA as int32_t - m_0 as int32_t)
                        * *src1
                            .offset(
                                (i as uint32_t)
                                    .wrapping_mul(src1_stride)
                                    .wrapping_add(j as uint32_t) as isize,
                            ) as int32_t >> AOM_BLEND_A64_ROUND_BITS;
                res_0 = (res_0 as core::ffi::c_int - round_offset) as int32_t;
                *dst
                    .offset(
                        (i as uint32_t)
                            .wrapping_mul(dst_stride)
                            .wrapping_add(j as uint32_t) as isize,
                    ) = clip_pixel(
                    res_0 as core::ffi::c_int
                        + ((1 as core::ffi::c_int) << round_bits
                            >> 1 as core::ffi::c_int) >> round_bits,
                );
                j += 1;
            }
            i += 1;
        }
    } else if subw == 1 as core::ffi::c_int && subh == 0 as core::ffi::c_int {
        i = 0 as core::ffi::c_int;
        while i < h {
            j = 0 as core::ffi::c_int;
            while j < w {
                let mut res_1: int32_t = 0;
                let m_1: core::ffi::c_int = *mask
                    .offset(
                        (i as uint32_t)
                            .wrapping_mul(mask_stride)
                            .wrapping_add((2 as core::ffi::c_int * j) as uint32_t)
                            as isize,
                    ) as core::ffi::c_int
                    + *mask
                        .offset(
                            (i as uint32_t)
                                .wrapping_mul(mask_stride)
                                .wrapping_add(
                                    (2 as core::ffi::c_int * j + 1 as core::ffi::c_int)
                                        as uint32_t,
                                ) as isize,
                        ) as core::ffi::c_int
                    + ((1 as core::ffi::c_int) << 1 as core::ffi::c_int
                        >> 1 as core::ffi::c_int) >> 1 as core::ffi::c_int;
                res_1 = m_1 as int32_t
                    * *src0
                        .offset(
                            (i as uint32_t)
                                .wrapping_mul(src0_stride)
                                .wrapping_add(j as uint32_t) as isize,
                        ) as int32_t
                    + (AOM_BLEND_A64_MAX_ALPHA as int32_t - m_1 as int32_t)
                        * *src1
                            .offset(
                                (i as uint32_t)
                                    .wrapping_mul(src1_stride)
                                    .wrapping_add(j as uint32_t) as isize,
                            ) as int32_t >> AOM_BLEND_A64_ROUND_BITS;
                res_1 = (res_1 as core::ffi::c_int - round_offset) as int32_t;
                *dst
                    .offset(
                        (i as uint32_t)
                            .wrapping_mul(dst_stride)
                            .wrapping_add(j as uint32_t) as isize,
                    ) = clip_pixel(
                    res_1 as core::ffi::c_int
                        + ((1 as core::ffi::c_int) << round_bits
                            >> 1 as core::ffi::c_int) >> round_bits,
                );
                j += 1;
            }
            i += 1;
        }
    } else {
        i = 0 as core::ffi::c_int;
        while i < h {
            j = 0 as core::ffi::c_int;
            while j < w {
                let mut res_2: int32_t = 0;
                let m_2: core::ffi::c_int = *mask
                    .offset(
                        ((2 as core::ffi::c_int * i) as uint32_t)
                            .wrapping_mul(mask_stride)
                            .wrapping_add(j as uint32_t) as isize,
                    ) as core::ffi::c_int
                    + *mask
                        .offset(
                            ((2 as core::ffi::c_int * i + 1 as core::ffi::c_int)
                                as uint32_t)
                                .wrapping_mul(mask_stride)
                                .wrapping_add(j as uint32_t) as isize,
                        ) as core::ffi::c_int
                    + ((1 as core::ffi::c_int) << 1 as core::ffi::c_int
                        >> 1 as core::ffi::c_int) >> 1 as core::ffi::c_int;
                res_2 = m_2 as int32_t
                    * *src0
                        .offset(
                            (i as uint32_t)
                                .wrapping_mul(src0_stride)
                                .wrapping_add(j as uint32_t) as isize,
                        ) as int32_t
                    + (AOM_BLEND_A64_MAX_ALPHA as int32_t - m_2 as int32_t)
                        * *src1
                            .offset(
                                (i as uint32_t)
                                    .wrapping_mul(src1_stride)
                                    .wrapping_add(j as uint32_t) as isize,
                            ) as int32_t >> AOM_BLEND_A64_ROUND_BITS;
                res_2 = (res_2 as core::ffi::c_int - round_offset) as int32_t;
                *dst
                    .offset(
                        (i as uint32_t)
                            .wrapping_mul(dst_stride)
                            .wrapping_add(j as uint32_t) as isize,
                    ) = clip_pixel(
                    res_2 as core::ffi::c_int
                        + ((1 as core::ffi::c_int) << round_bits
                            >> 1 as core::ffi::c_int) >> round_bits,
                );
                j += 1;
            }
            i += 1;
        }
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_blend_a64_d16_mask_c(
    mut dst_8: *mut uint8_t,
    mut dst_stride: uint32_t,
    mut src0: *const CONV_BUF_TYPE,
    mut src0_stride: uint32_t,
    mut src1: *const CONV_BUF_TYPE,
    mut src1_stride: uint32_t,
    mut mask: *const uint8_t,
    mut mask_stride: uint32_t,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut subw: core::ffi::c_int,
    mut subh: core::ffi::c_int,
    mut conv_params: *mut ConvolveParams,
    bd: core::ffi::c_int,
) {
    let offset_bits: core::ffi::c_int = bd + 2 as core::ffi::c_int * FILTER_BITS
        - (*conv_params).round_0;
    let round_offset: core::ffi::c_int = ((1 as core::ffi::c_int)
        << offset_bits - (*conv_params).round_1)
        + ((1 as core::ffi::c_int)
            << offset_bits - (*conv_params).round_1 - 1 as core::ffi::c_int);
    let round_bits: core::ffi::c_int = 2 as core::ffi::c_int * FILTER_BITS
        - (*conv_params).round_0 - (*conv_params).round_1;
    let mut dst: *mut uint16_t = ((dst_8 as uintptr_t) << 1 as core::ffi::c_int)
        as *mut uint16_t;
    let mut saturation_value: core::ffi::c_uint = 0;
    match bd {
        10 => {
            saturation_value = 1023 as core::ffi::c_uint;
        }
        12 => {
            saturation_value = 4095 as core::ffi::c_uint;
        }
        8 | _ => {
            saturation_value = 255 as core::ffi::c_uint;
        }
    }
    if subw == 0 as core::ffi::c_int && subh == 0 as core::ffi::c_int {
        let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
        while i < h {
            let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
            while j < w {
                let mut res: int32_t = 0;
                let m: core::ffi::c_int = *mask.offset(j as isize) as core::ffi::c_int;
                res = (m * *src0.offset(j as isize) as core::ffi::c_int
                    + (AOM_BLEND_A64_MAX_ALPHA - m)
                        * *src1.offset(j as isize) as core::ffi::c_int
                    >> AOM_BLEND_A64_ROUND_BITS) as int32_t;
                res = (res as core::ffi::c_int - round_offset) as int32_t;
                let mut v: core::ffi::c_uint = negative_to_zero(
                    res as core::ffi::c_int
                        + ((1 as core::ffi::c_int) << round_bits
                            >> 1 as core::ffi::c_int) >> round_bits,
                );
                *dst.offset(j as isize) = (if v < saturation_value {
                    v
                } else {
                    saturation_value
                }) as uint16_t;
                j += 1;
            }
            mask = mask.offset(mask_stride as isize);
            src0 = src0.offset(src0_stride as isize);
            src1 = src1.offset(src1_stride as isize);
            dst = dst.offset(dst_stride as isize);
            i += 1;
        }
    } else if subw == 1 as core::ffi::c_int && subh == 1 as core::ffi::c_int {
        let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
        while i_0 < h {
            let mut j_0: core::ffi::c_int = 0 as core::ffi::c_int;
            while j_0 < w {
                let mut res_0: int32_t = 0;
                let m_0: core::ffi::c_int = *mask
                    .offset((2 as core::ffi::c_int * j_0) as isize) as core::ffi::c_int
                    + *mask
                        .offset(
                            mask_stride
                                .wrapping_add((2 as core::ffi::c_int * j_0) as uint32_t)
                                as isize,
                        ) as core::ffi::c_int
                    + *mask
                        .offset(
                            (2 as core::ffi::c_int * j_0 + 1 as core::ffi::c_int)
                                as isize,
                        ) as core::ffi::c_int
                    + *mask
                        .offset(
                            mask_stride
                                .wrapping_add((2 as core::ffi::c_int * j_0) as uint32_t)
                                .wrapping_add(1 as uint32_t) as isize,
                        ) as core::ffi::c_int
                    + ((1 as core::ffi::c_int) << 2 as core::ffi::c_int
                        >> 1 as core::ffi::c_int) >> 2 as core::ffi::c_int;
                res_0 = (m_0 * *src0.offset(j_0 as isize) as core::ffi::c_int
                    + (AOM_BLEND_A64_MAX_ALPHA - m_0)
                        * *src1.offset(j_0 as isize) as core::ffi::c_int
                    >> AOM_BLEND_A64_ROUND_BITS) as int32_t;
                res_0 = (res_0 as core::ffi::c_int - round_offset) as int32_t;
                let mut v_0: core::ffi::c_uint = negative_to_zero(
                    res_0 as core::ffi::c_int
                        + ((1 as core::ffi::c_int) << round_bits
                            >> 1 as core::ffi::c_int) >> round_bits,
                );
                *dst.offset(j_0 as isize) = (if v_0 < saturation_value {
                    v_0
                } else {
                    saturation_value
                }) as uint16_t;
                j_0 += 1;
            }
            mask = mask.offset((2 as uint32_t).wrapping_mul(mask_stride) as isize);
            src0 = src0.offset(src0_stride as isize);
            src1 = src1.offset(src1_stride as isize);
            dst = dst.offset(dst_stride as isize);
            i_0 += 1;
        }
    } else if subw == 1 as core::ffi::c_int && subh == 0 as core::ffi::c_int {
        let mut i_1: core::ffi::c_int = 0 as core::ffi::c_int;
        while i_1 < h {
            let mut j_1: core::ffi::c_int = 0 as core::ffi::c_int;
            while j_1 < w {
                let mut res_1: int32_t = 0;
                let m_1: core::ffi::c_int = *mask
                    .offset((2 as core::ffi::c_int * j_1) as isize) as core::ffi::c_int
                    + *mask
                        .offset(
                            (2 as core::ffi::c_int * j_1 + 1 as core::ffi::c_int)
                                as isize,
                        ) as core::ffi::c_int
                    + ((1 as core::ffi::c_int) << 1 as core::ffi::c_int
                        >> 1 as core::ffi::c_int) >> 1 as core::ffi::c_int;
                res_1 = (m_1 * *src0.offset(j_1 as isize) as core::ffi::c_int
                    + (AOM_BLEND_A64_MAX_ALPHA - m_1)
                        * *src1.offset(j_1 as isize) as core::ffi::c_int
                    >> AOM_BLEND_A64_ROUND_BITS) as int32_t;
                res_1 = (res_1 as core::ffi::c_int - round_offset) as int32_t;
                let mut v_1: core::ffi::c_uint = negative_to_zero(
                    res_1 as core::ffi::c_int
                        + ((1 as core::ffi::c_int) << round_bits
                            >> 1 as core::ffi::c_int) >> round_bits,
                );
                *dst.offset(j_1 as isize) = (if v_1 < saturation_value {
                    v_1
                } else {
                    saturation_value
                }) as uint16_t;
                j_1 += 1;
            }
            mask = mask.offset(mask_stride as isize);
            src0 = src0.offset(src0_stride as isize);
            src1 = src1.offset(src1_stride as isize);
            dst = dst.offset(dst_stride as isize);
            i_1 += 1;
        }
    } else {
        let mut i_2: core::ffi::c_int = 0 as core::ffi::c_int;
        while i_2 < h {
            let mut j_2: core::ffi::c_int = 0 as core::ffi::c_int;
            while j_2 < w {
                let mut res_2: int32_t = 0;
                let m_2: core::ffi::c_int = *mask.offset(j_2 as isize)
                    as core::ffi::c_int
                    + *mask.offset(mask_stride.wrapping_add(j_2 as uint32_t) as isize)
                        as core::ffi::c_int
                    + ((1 as core::ffi::c_int) << 1 as core::ffi::c_int
                        >> 1 as core::ffi::c_int) >> 1 as core::ffi::c_int;
                res_2 = (m_2 * *src0.offset(j_2 as isize) as core::ffi::c_int
                    + (AOM_BLEND_A64_MAX_ALPHA - m_2)
                        * *src1.offset(j_2 as isize) as core::ffi::c_int
                    >> AOM_BLEND_A64_ROUND_BITS) as int32_t;
                res_2 = (res_2 as core::ffi::c_int - round_offset) as int32_t;
                let mut v_2: core::ffi::c_uint = negative_to_zero(
                    res_2 as core::ffi::c_int
                        + ((1 as core::ffi::c_int) << round_bits
                            >> 1 as core::ffi::c_int) >> round_bits,
                );
                *dst.offset(j_2 as isize) = (if v_2 < saturation_value {
                    v_2
                } else {
                    saturation_value
                }) as uint16_t;
                j_2 += 1;
            }
            mask = mask.offset((2 as uint32_t).wrapping_mul(mask_stride) as isize);
            src0 = src0.offset(src0_stride as isize);
            src1 = src1.offset(src1_stride as isize);
            dst = dst.offset(dst_stride as isize);
            i_2 += 1;
        }
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_blend_a64_mask_c(
    mut dst: *mut uint8_t,
    mut dst_stride: uint32_t,
    mut src0: *const uint8_t,
    mut src0_stride: uint32_t,
    mut src1: *const uint8_t,
    mut src1_stride: uint32_t,
    mut mask: *const uint8_t,
    mut mask_stride: uint32_t,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut subw: core::ffi::c_int,
    mut subh: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0;
    let mut j: core::ffi::c_int = 0;
    if subw == 0 as core::ffi::c_int && subh == 0 as core::ffi::c_int {
        i = 0 as core::ffi::c_int;
        while i < h {
            j = 0 as core::ffi::c_int;
            while j < w {
                let m: core::ffi::c_int = *mask
                    .offset(
                        (i as uint32_t)
                            .wrapping_mul(mask_stride)
                            .wrapping_add(j as uint32_t) as isize,
                    ) as core::ffi::c_int;
                *dst
                    .offset(
                        (i as uint32_t)
                            .wrapping_mul(dst_stride)
                            .wrapping_add(j as uint32_t) as isize,
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
    } else if subw == 1 as core::ffi::c_int && subh == 1 as core::ffi::c_int {
        i = 0 as core::ffi::c_int;
        while i < h {
            j = 0 as core::ffi::c_int;
            while j < w {
                let m_0: core::ffi::c_int = *mask
                    .offset(
                        ((2 as core::ffi::c_int * i) as uint32_t)
                            .wrapping_mul(mask_stride)
                            .wrapping_add((2 as core::ffi::c_int * j) as uint32_t)
                            as isize,
                    ) as core::ffi::c_int
                    + *mask
                        .offset(
                            ((2 as core::ffi::c_int * i + 1 as core::ffi::c_int)
                                as uint32_t)
                                .wrapping_mul(mask_stride)
                                .wrapping_add((2 as core::ffi::c_int * j) as uint32_t)
                                as isize,
                        ) as core::ffi::c_int
                    + *mask
                        .offset(
                            ((2 as core::ffi::c_int * i) as uint32_t)
                                .wrapping_mul(mask_stride)
                                .wrapping_add(
                                    (2 as core::ffi::c_int * j + 1 as core::ffi::c_int)
                                        as uint32_t,
                                ) as isize,
                        ) as core::ffi::c_int
                    + *mask
                        .offset(
                            ((2 as core::ffi::c_int * i + 1 as core::ffi::c_int)
                                as uint32_t)
                                .wrapping_mul(mask_stride)
                                .wrapping_add(
                                    (2 as core::ffi::c_int * j + 1 as core::ffi::c_int)
                                        as uint32_t,
                                ) as isize,
                        ) as core::ffi::c_int
                    + ((1 as core::ffi::c_int) << 2 as core::ffi::c_int
                        >> 1 as core::ffi::c_int) >> 2 as core::ffi::c_int;
                *dst
                    .offset(
                        (i as uint32_t)
                            .wrapping_mul(dst_stride)
                            .wrapping_add(j as uint32_t) as isize,
                    ) = (m_0
                    * *src0
                        .offset(
                            (i as uint32_t)
                                .wrapping_mul(src0_stride)
                                .wrapping_add(j as uint32_t) as isize,
                        ) as core::ffi::c_int
                    + (((1 as core::ffi::c_int) << 6 as core::ffi::c_int) - m_0)
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
    } else if subw == 1 as core::ffi::c_int && subh == 0 as core::ffi::c_int {
        i = 0 as core::ffi::c_int;
        while i < h {
            j = 0 as core::ffi::c_int;
            while j < w {
                let m_1: core::ffi::c_int = *mask
                    .offset(
                        (i as uint32_t)
                            .wrapping_mul(mask_stride)
                            .wrapping_add((2 as core::ffi::c_int * j) as uint32_t)
                            as isize,
                    ) as core::ffi::c_int
                    + *mask
                        .offset(
                            (i as uint32_t)
                                .wrapping_mul(mask_stride)
                                .wrapping_add(
                                    (2 as core::ffi::c_int * j + 1 as core::ffi::c_int)
                                        as uint32_t,
                                ) as isize,
                        ) as core::ffi::c_int
                    + ((1 as core::ffi::c_int) << 1 as core::ffi::c_int
                        >> 1 as core::ffi::c_int) >> 1 as core::ffi::c_int;
                *dst
                    .offset(
                        (i as uint32_t)
                            .wrapping_mul(dst_stride)
                            .wrapping_add(j as uint32_t) as isize,
                    ) = (m_1
                    * *src0
                        .offset(
                            (i as uint32_t)
                                .wrapping_mul(src0_stride)
                                .wrapping_add(j as uint32_t) as isize,
                        ) as core::ffi::c_int
                    + (((1 as core::ffi::c_int) << 6 as core::ffi::c_int) - m_1)
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
    } else {
        i = 0 as core::ffi::c_int;
        while i < h {
            j = 0 as core::ffi::c_int;
            while j < w {
                let m_2: core::ffi::c_int = *mask
                    .offset(
                        ((2 as core::ffi::c_int * i) as uint32_t)
                            .wrapping_mul(mask_stride)
                            .wrapping_add(j as uint32_t) as isize,
                    ) as core::ffi::c_int
                    + *mask
                        .offset(
                            ((2 as core::ffi::c_int * i + 1 as core::ffi::c_int)
                                as uint32_t)
                                .wrapping_mul(mask_stride)
                                .wrapping_add(j as uint32_t) as isize,
                        ) as core::ffi::c_int
                    + ((1 as core::ffi::c_int) << 1 as core::ffi::c_int
                        >> 1 as core::ffi::c_int) >> 1 as core::ffi::c_int;
                *dst
                    .offset(
                        (i as uint32_t)
                            .wrapping_mul(dst_stride)
                            .wrapping_add(j as uint32_t) as isize,
                    ) = (m_2
                    * *src0
                        .offset(
                            (i as uint32_t)
                                .wrapping_mul(src0_stride)
                                .wrapping_add(j as uint32_t) as isize,
                        ) as core::ffi::c_int
                    + (((1 as core::ffi::c_int) << 6 as core::ffi::c_int) - m_2)
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
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_blend_a64_mask_c(
    mut dst_8: *mut uint8_t,
    mut dst_stride: uint32_t,
    mut src0_8: *const uint8_t,
    mut src0_stride: uint32_t,
    mut src1_8: *const uint8_t,
    mut src1_stride: uint32_t,
    mut mask: *const uint8_t,
    mut mask_stride: uint32_t,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut subw: core::ffi::c_int,
    mut subh: core::ffi::c_int,
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
    if subw == 0 as core::ffi::c_int && subh == 0 as core::ffi::c_int {
        i = 0 as core::ffi::c_int;
        while i < h {
            j = 0 as core::ffi::c_int;
            while j < w {
                let m: core::ffi::c_int = *mask
                    .offset(
                        (i as uint32_t)
                            .wrapping_mul(mask_stride)
                            .wrapping_add(j as uint32_t) as isize,
                    ) as core::ffi::c_int;
                *dst
                    .offset(
                        (i as uint32_t)
                            .wrapping_mul(dst_stride)
                            .wrapping_add(j as uint32_t) as isize,
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
    } else if subw == 1 as core::ffi::c_int && subh == 1 as core::ffi::c_int {
        i = 0 as core::ffi::c_int;
        while i < h {
            j = 0 as core::ffi::c_int;
            while j < w {
                let m_0: core::ffi::c_int = *mask
                    .offset(
                        ((2 as core::ffi::c_int * i) as uint32_t)
                            .wrapping_mul(mask_stride)
                            .wrapping_add((2 as core::ffi::c_int * j) as uint32_t)
                            as isize,
                    ) as core::ffi::c_int
                    + *mask
                        .offset(
                            ((2 as core::ffi::c_int * i + 1 as core::ffi::c_int)
                                as uint32_t)
                                .wrapping_mul(mask_stride)
                                .wrapping_add((2 as core::ffi::c_int * j) as uint32_t)
                                as isize,
                        ) as core::ffi::c_int
                    + *mask
                        .offset(
                            ((2 as core::ffi::c_int * i) as uint32_t)
                                .wrapping_mul(mask_stride)
                                .wrapping_add(
                                    (2 as core::ffi::c_int * j + 1 as core::ffi::c_int)
                                        as uint32_t,
                                ) as isize,
                        ) as core::ffi::c_int
                    + *mask
                        .offset(
                            ((2 as core::ffi::c_int * i + 1 as core::ffi::c_int)
                                as uint32_t)
                                .wrapping_mul(mask_stride)
                                .wrapping_add(
                                    (2 as core::ffi::c_int * j + 1 as core::ffi::c_int)
                                        as uint32_t,
                                ) as isize,
                        ) as core::ffi::c_int
                    + ((1 as core::ffi::c_int) << 2 as core::ffi::c_int
                        >> 1 as core::ffi::c_int) >> 2 as core::ffi::c_int;
                *dst
                    .offset(
                        (i as uint32_t)
                            .wrapping_mul(dst_stride)
                            .wrapping_add(j as uint32_t) as isize,
                    ) = (m_0
                    * *src0
                        .offset(
                            (i as uint32_t)
                                .wrapping_mul(src0_stride)
                                .wrapping_add(j as uint32_t) as isize,
                        ) as core::ffi::c_int
                    + (((1 as core::ffi::c_int) << 6 as core::ffi::c_int) - m_0)
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
    } else if subw == 1 as core::ffi::c_int && subh == 0 as core::ffi::c_int {
        i = 0 as core::ffi::c_int;
        while i < h {
            j = 0 as core::ffi::c_int;
            while j < w {
                let m_1: core::ffi::c_int = *mask
                    .offset(
                        (i as uint32_t)
                            .wrapping_mul(mask_stride)
                            .wrapping_add((2 as core::ffi::c_int * j) as uint32_t)
                            as isize,
                    ) as core::ffi::c_int
                    + *mask
                        .offset(
                            (i as uint32_t)
                                .wrapping_mul(mask_stride)
                                .wrapping_add(
                                    (2 as core::ffi::c_int * j + 1 as core::ffi::c_int)
                                        as uint32_t,
                                ) as isize,
                        ) as core::ffi::c_int
                    + ((1 as core::ffi::c_int) << 1 as core::ffi::c_int
                        >> 1 as core::ffi::c_int) >> 1 as core::ffi::c_int;
                *dst
                    .offset(
                        (i as uint32_t)
                            .wrapping_mul(dst_stride)
                            .wrapping_add(j as uint32_t) as isize,
                    ) = (m_1
                    * *src0
                        .offset(
                            (i as uint32_t)
                                .wrapping_mul(src0_stride)
                                .wrapping_add(j as uint32_t) as isize,
                        ) as core::ffi::c_int
                    + (((1 as core::ffi::c_int) << 6 as core::ffi::c_int) - m_1)
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
    } else {
        i = 0 as core::ffi::c_int;
        while i < h {
            j = 0 as core::ffi::c_int;
            while j < w {
                let m_2: core::ffi::c_int = *mask
                    .offset(
                        ((2 as core::ffi::c_int * i) as uint32_t)
                            .wrapping_mul(mask_stride)
                            .wrapping_add(j as uint32_t) as isize,
                    ) as core::ffi::c_int
                    + *mask
                        .offset(
                            ((2 as core::ffi::c_int * i + 1 as core::ffi::c_int)
                                as uint32_t)
                                .wrapping_mul(mask_stride)
                                .wrapping_add(j as uint32_t) as isize,
                        ) as core::ffi::c_int
                    + ((1 as core::ffi::c_int) << 1 as core::ffi::c_int
                        >> 1 as core::ffi::c_int) >> 1 as core::ffi::c_int;
                *dst
                    .offset(
                        (i as uint32_t)
                            .wrapping_mul(dst_stride)
                            .wrapping_add(j as uint32_t) as isize,
                    ) = (m_2
                    * *src0
                        .offset(
                            (i as uint32_t)
                                .wrapping_mul(src0_stride)
                                .wrapping_add(j as uint32_t) as isize,
                        ) as core::ffi::c_int
                    + (((1 as core::ffi::c_int) << 6 as core::ffi::c_int) - m_2)
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
    };
}
