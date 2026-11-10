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

// Mechanically translated from upstream/aom/av1/encoder/global_motion.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C-unwind" {
    fn aom_highbd_sad32x32_c(
        src_ptr: *const uint8_t,
        src_stride: core::ffi::c_int,
        ref_ptr: *const uint8_t,
        ref_stride: core::ffi::c_int,
    ) -> core::ffi::c_uint;
    fn aom_sad32x32_c(
        src_ptr: *const uint8_t,
        src_stride: core::ffi::c_int,
        ref_ptr: *const uint8_t,
        ref_stride: core::ffi::c_int,
    ) -> core::ffi::c_uint;
    fn highbd_warp_plane(
        wm: *mut WarpedMotionParams,
        ref_0: *const uint16_t,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
        stride: core::ffi::c_int,
        pred: *mut uint16_t,
        p_col: core::ffi::c_int,
        p_row: core::ffi::c_int,
        p_width: core::ffi::c_int,
        p_height: core::ffi::c_int,
        p_stride: core::ffi::c_int,
        subsampling_x: core::ffi::c_int,
        subsampling_y: core::ffi::c_int,
        bd: core::ffi::c_int,
        conv_params: *mut ConvolveParams,
    );
    fn warp_plane(
        wm: *mut WarpedMotionParams,
        ref_0: *const uint8_t,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
        stride: core::ffi::c_int,
        pred: *mut uint8_t,
        p_col: core::ffi::c_int,
        p_row: core::ffi::c_int,
        p_width: core::ffi::c_int,
        p_height: core::ffi::c_int,
        p_stride: core::ffi::c_int,
        subsampling_x: core::ffi::c_int,
        subsampling_y: core::ffi::c_int,
        conv_params: *mut ConvolveParams,
    );
    fn av1_get_shear_params(wm: *mut WarpedMotionParams) -> core::ffi::c_int;
}
pub type __darwin_size_t = usize;
pub type int8_t = i8;
pub type int16_t = i16;
pub type int32_t = i32;
pub type int64_t = i64;
pub type uintptr_t = usize;
pub type size_t = __darwin_size_t;
pub type uint8_t = u8;
pub type uint16_t = u16;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WarpedMotionParams {
    pub wmmat: [int32_t; 6],
    pub alpha: int16_t,
    pub beta: int16_t,
    pub gamma: int16_t,
    pub delta: int16_t,
    pub wmtype: TransformationType,
    pub invalid: int8_t,
}
pub type TransformationType = uint8_t;
pub type CONV_BUF_TYPE = uint16_t;
pub type C2RustUnnamed = core::ffi::c_uint;
pub const TRANS_TYPES: C2RustUnnamed = 4;
pub const AFFINE: C2RustUnnamed = 3;
pub const ROTZOOM: C2RustUnnamed = 2;
pub const TRANSLATION: C2RustUnnamed = 1;
pub const IDENTITY: C2RustUnnamed = 0;
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
pub const INT64_MAX: core::ffi::c_longlong = 9223372036854775807
    as core::ffi::c_longlong;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
#[inline]
unsafe extern "C-unwind" fn clamp(
    mut value: core::ffi::c_int,
    mut low: core::ffi::c_int,
    mut high: core::ffi::c_int,
) -> core::ffi::c_int {
    return if value < low { low } else if value > high { high } else { value };
}
pub const FILTER_BITS: core::ffi::c_int = 7 as core::ffi::c_int;
pub const WARPEDMODEL_PREC_BITS: core::ffi::c_int = 16 as core::ffi::c_int;
pub const GM_TRANS_PREC_BITS: core::ffi::c_int = 6 as core::ffi::c_int;
pub const GM_ABS_TRANS_BITS: core::ffi::c_int = 12 as core::ffi::c_int;
pub const GM_TRANS_PREC_DIFF: core::ffi::c_int = WARPEDMODEL_PREC_BITS
    - GM_TRANS_PREC_BITS;
pub const GM_TRANS_DECODE_FACTOR: core::ffi::c_int = (1 as core::ffi::c_int)
    << GM_TRANS_PREC_DIFF;
pub const GM_ALPHA_PREC_BITS: core::ffi::c_int = 15 as core::ffi::c_int;
pub const GM_ABS_ALPHA_BITS: core::ffi::c_int = 12 as core::ffi::c_int;
pub const GM_ALPHA_PREC_DIFF: core::ffi::c_int = WARPEDMODEL_PREC_BITS
    - GM_ALPHA_PREC_BITS;
pub const GM_ALPHA_DECODE_FACTOR: core::ffi::c_int = (1 as core::ffi::c_int)
    << GM_ALPHA_PREC_DIFF;
pub const GM_TRANS_MAX: core::ffi::c_int = (1 as core::ffi::c_int) << GM_ABS_TRANS_BITS;
pub const GM_ALPHA_MAX: core::ffi::c_int = (1 as core::ffi::c_int) << GM_ABS_ALPHA_BITS;
#[inline]
unsafe extern "C-unwind" fn get_wmtype(
    mut gm: *const WarpedMotionParams,
) -> TransformationType {
    if (*gm).wmmat[5 as core::ffi::c_int as usize]
        == (1 as int32_t) << WARPEDMODEL_PREC_BITS
        && (*gm).wmmat[4 as core::ffi::c_int as usize] == 0
        && (*gm).wmmat[2 as core::ffi::c_int as usize]
            == (1 as int32_t) << WARPEDMODEL_PREC_BITS
        && (*gm).wmmat[3 as core::ffi::c_int as usize] == 0
    {
        return (if (*gm).wmmat[1 as core::ffi::c_int as usize] == 0
            && (*gm).wmmat[0 as core::ffi::c_int as usize] == 0
        {
            IDENTITY as core::ffi::c_int
        } else {
            TRANSLATION as core::ffi::c_int
        }) as TransformationType;
    }
    if (*gm).wmmat[2 as core::ffi::c_int as usize]
        == (*gm).wmmat[5 as core::ffi::c_int as usize]
        && (*gm).wmmat[3 as core::ffi::c_int as usize]
            == -(*gm).wmmat[4 as core::ffi::c_int as usize]
    {
        return ROTZOOM as core::ffi::c_int as TransformationType
    } else {
        return AFFINE as core::ffi::c_int as TransformationType
    };
}
pub const ROUND0_BITS: core::ffi::c_int = 3 as core::ffi::c_int;
pub const COMPOUND_ROUND1_BITS: core::ffi::c_int = 7 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn get_conv_params_no_round(
    mut cmp_index: core::ffi::c_int,
    mut plane: core::ffi::c_int,
    mut dst: *mut CONV_BUF_TYPE,
    mut dst_stride: core::ffi::c_int,
    mut is_compound: core::ffi::c_int,
    mut bd: core::ffi::c_int,
) -> ConvolveParams {
    let mut conv_params: ConvolveParams = ConvolveParams {
        do_average: 0,
        dst: 0 as *mut CONV_BUF_TYPE,
        dst_stride: 0,
        round_0: 0,
        round_1: 0,
        plane: 0,
        is_compound: 0,
        use_dist_wtd_comp_avg: 0,
        fwd_offset: 0,
        bck_offset: 0,
    };
    if !(cmp_index == 0 || is_compound != 0) as core::ffi::c_int as core::ffi::c_long
        != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 25],
                [core::ffi::c_char; 25],
            >(*b"get_conv_params_no_round\0"))
                .as_ptr(),
            b"convolve.h\0" as *const u8 as *const core::ffi::c_char,
            73 as core::ffi::c_int,
            b"IMPLIES(cmp_index, is_compound)\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    conv_params.is_compound = is_compound;
    conv_params.use_dist_wtd_comp_avg = 0 as core::ffi::c_int;
    conv_params.round_0 = ROUND0_BITS;
    conv_params.round_1 = if is_compound != 0 {
        COMPOUND_ROUND1_BITS
    } else {
        2 as core::ffi::c_int * FILTER_BITS - conv_params.round_0
    };
    let intbufrange: core::ffi::c_int = bd + FILTER_BITS - conv_params.round_0
        + 2 as core::ffi::c_int;
    if !(!(bd < 12 as core::ffi::c_int) || intbufrange <= 16 as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 25],
                [core::ffi::c_char; 25],
            >(*b"get_conv_params_no_round\0"))
                .as_ptr(),
            b"convolve.h\0" as *const u8 as *const core::ffi::c_char,
            82 as core::ffi::c_int,
            b"IMPLIES(bd < 12, intbufrange <= 16)\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    if intbufrange > 16 as core::ffi::c_int {
        conv_params.round_0 += intbufrange - 16 as core::ffi::c_int;
        if is_compound == 0 {
            conv_params.round_1 -= intbufrange - 16 as core::ffi::c_int;
        }
    }
    conv_params.dst = dst;
    conv_params.dst_stride = dst_stride;
    conv_params.plane = plane;
    conv_params.do_average = cmp_index;
    return conv_params;
}
#[inline]
unsafe extern "C-unwind" fn get_conv_params(
    mut do_average: core::ffi::c_int,
    mut plane: core::ffi::c_int,
    mut bd: core::ffi::c_int,
) -> ConvolveParams {
    return get_conv_params_no_round(
        do_average,
        plane,
        0 as *mut CONV_BUF_TYPE,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        bd,
    );
}
static mut erroradv_prod_tr: core::ffi::c_double = 20000 as core::ffi::c_int
    as core::ffi::c_double;
static mut erroradv_early_tr: core::ffi::c_double = 0.70f64;
pub const WARP_ERROR_BLOCK_LOG: core::ffi::c_int = 5 as core::ffi::c_int;
pub const WARP_ERROR_BLOCK: core::ffi::c_int = (1 as core::ffi::c_int)
    << WARP_ERROR_BLOCK_LOG;
pub const ERRORADV_BORDER: core::ffi::c_int = 0 as core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_is_enough_erroradvantage(
    mut best_erroradvantage: core::ffi::c_double,
    mut params_cost: core::ffi::c_int,
    mut gm_erroradv_tr: core::ffi::c_double,
) -> core::ffi::c_int {
    return (best_erroradvantage < gm_erroradv_tr
        && (best_erroradvantage * params_cost as core::ffi::c_double) < erroradv_prod_tr)
        as core::ffi::c_int;
}
unsafe extern "C-unwind" fn convert_to_params(
    mut params: *const core::ffi::c_double,
    mut model: *mut int32_t,
) {
    let mut i: core::ffi::c_int = 0;
    *model.offset(0 as core::ffi::c_int as isize) = floor(
        *params.offset(0 as core::ffi::c_int as isize)
            * ((1 as core::ffi::c_int) << GM_TRANS_PREC_BITS) as core::ffi::c_double
            + 0.5f64,
    ) as int32_t;
    *model.offset(1 as core::ffi::c_int as isize) = floor(
        *params.offset(1 as core::ffi::c_int as isize)
            * ((1 as core::ffi::c_int) << GM_TRANS_PREC_BITS) as core::ffi::c_double
            + 0.5f64,
    ) as int32_t;
    *model.offset(0 as core::ffi::c_int as isize) = clamp(
        *model.offset(0 as core::ffi::c_int as isize),
        -GM_TRANS_MAX,
        GM_TRANS_MAX,
    ) as int32_t * GM_TRANS_DECODE_FACTOR as int32_t;
    *model.offset(1 as core::ffi::c_int as isize) = clamp(
        *model.offset(1 as core::ffi::c_int as isize),
        -GM_TRANS_MAX,
        GM_TRANS_MAX,
    ) as int32_t * GM_TRANS_DECODE_FACTOR as int32_t;
    i = 2 as core::ffi::c_int;
    while i < 6 as core::ffi::c_int {
        let diag_value: core::ffi::c_int = if i == 2 as core::ffi::c_int
            || i == 5 as core::ffi::c_int
        {
            (1 as core::ffi::c_int) << GM_ALPHA_PREC_BITS
        } else {
            0 as core::ffi::c_int
        };
        *model.offset(i as isize) = floor(
            *params.offset(i as isize)
                * ((1 as core::ffi::c_int) << GM_ALPHA_PREC_BITS) as core::ffi::c_double
                + 0.5f64,
        ) as int32_t;
        *model.offset(i as isize) = clamp(
            *model.offset(i as isize) - diag_value,
            -GM_ALPHA_MAX,
            GM_ALPHA_MAX,
        ) as int32_t;
        *model.offset(i as isize) = (*model.offset(i as isize) + diag_value as int32_t)
            * GM_ALPHA_DECODE_FACTOR as int32_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_convert_model_to_params(
    mut params: *const core::ffi::c_double,
    mut model: *mut WarpedMotionParams,
) {
    convert_to_params(params, ((*model).wmmat).as_mut_ptr());
    (*model).wmtype = get_wmtype(model);
    (*model).invalid = 0 as int8_t;
}
unsafe extern "C-unwind" fn add_param_offset(
    mut param_index: core::ffi::c_int,
    mut param_value: int32_t,
    mut offset: int32_t,
) -> int32_t {
    let scale_vals: [core::ffi::c_int; 2] = [GM_TRANS_PREC_DIFF, GM_ALPHA_PREC_DIFF];
    let clamp_vals: [core::ffi::c_int; 2] = [GM_TRANS_MAX, GM_ALPHA_MAX];
    let param_type: core::ffi::c_int = if param_index < 2 as core::ffi::c_int {
        0 as core::ffi::c_int
    } else {
        1 as core::ffi::c_int
    };
    let is_one_centered: core::ffi::c_int = (param_index == 2 as core::ffi::c_int
        || param_index == 5 as core::ffi::c_int) as core::ffi::c_int;
    param_value = param_value - ((is_one_centered as int32_t) << WARPEDMODEL_PREC_BITS)
        >> scale_vals[param_type as usize];
    param_value += offset;
    param_value = clamp(
        param_value as core::ffi::c_int,
        -clamp_vals[param_type as usize],
        clamp_vals[param_type as usize],
    ) as int32_t;
    param_value = (param_value as core::ffi::c_int
        * ((1 as core::ffi::c_int) << scale_vals[param_type as usize])) as int32_t;
    return param_value + ((is_one_centered as int32_t) << WARPEDMODEL_PREC_BITS);
}
unsafe extern "C-unwind" fn force_wmtype(
    mut wm: *mut WarpedMotionParams,
    mut wmtype: TransformationType,
) {
    let mut current_block_12: u64;
    match wmtype as core::ffi::c_int {
        0 => {
            (*wm).wmmat[0 as core::ffi::c_int as usize] = 0 as core::ffi::c_int
                as int32_t;
            (*wm).wmmat[1 as core::ffi::c_int as usize] = 0 as core::ffi::c_int
                as int32_t;
            current_block_12 = 0;
        }
        1 => {
            current_block_12 = 0;
        }
        2 => {
            current_block_12 = 1;
        }
        3 => {
            current_block_12 = 2;
        }
        _ => {
            if (0 as core::ffi::c_int == 0) as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 13],
                        [core::ffi::c_char; 13],
                    >(*b"force_wmtype\0"))
                        .as_ptr(),
                    b"global_motion.c\0" as *const u8 as *const core::ffi::c_char,
                    108 as core::ffi::c_int,
                    b"0\0" as *const u8 as *const core::ffi::c_char,
                );
            } else {};
            current_block_12 = 2;
        }
    }
    match current_block_12 {
        0 => {
            (*wm).wmmat[2 as core::ffi::c_int as usize] = ((1 as core::ffi::c_int)
                << WARPEDMODEL_PREC_BITS) as int32_t;
            (*wm).wmmat[3 as core::ffi::c_int as usize] = 0 as core::ffi::c_int
                as int32_t;
            current_block_12 = 1;
        }
        _ => {}
    }
    match current_block_12 {
        1 => {
            (*wm).wmmat[4 as core::ffi::c_int as usize] = -(*wm)
                .wmmat[3 as core::ffi::c_int as usize];
            (*wm).wmmat[5 as core::ffi::c_int as usize] = (*wm)
                .wmmat[2 as core::ffi::c_int as usize];
        }
        _ => {}
    }
    (*wm).wmtype = wmtype;
}
#[inline]
unsafe extern "C-unwind" fn generic_sad_highbd(
    ref_0: *const uint16_t,
    mut ref_stride: core::ffi::c_int,
    dst: *const uint16_t,
    mut dst_stride: core::ffi::c_int,
    mut p_width: core::ffi::c_int,
    mut p_height: core::ffi::c_int,
) -> core::ffi::c_int {
    if !(p_width <= (1 as core::ffi::c_int) << 5 as core::ffi::c_int
        && p_height <= (1 as core::ffi::c_int) << 5 as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"generic_sad_highbd\0"))
                .as_ptr(),
            b"global_motion.c\0" as *const u8 as *const core::ffi::c_char,
            120 as core::ffi::c_int,
            b"p_width <= WARP_ERROR_BLOCK && p_height <= WARP_ERROR_BLOCK\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    let mut sad: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < p_height {
        let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
        while j < p_width {
            sad
                += abs(
                    *dst.offset((j + i * dst_stride) as isize) as core::ffi::c_int
                        - *ref_0.offset((j + i * ref_stride) as isize)
                            as core::ffi::c_int,
                );
            j += 1;
        }
        i += 1;
    }
    return sad;
}
unsafe extern "C-unwind" fn highbd_segmented_frame_error(
    ref_0: *const uint16_t,
    mut ref_stride: core::ffi::c_int,
    dst: *const uint16_t,
    mut dst_stride: core::ffi::c_int,
    mut p_width: core::ffi::c_int,
    mut p_height: core::ffi::c_int,
    mut bd: core::ffi::c_int,
    mut segment_map: *mut uint8_t,
    mut segment_map_stride: core::ffi::c_int,
) -> int64_t {
    let mut patch_w: core::ffi::c_int = 0;
    let mut patch_h: core::ffi::c_int = 0;
    let error_bsize_w: core::ffi::c_int = if p_width
        < (1 as core::ffi::c_int) << 5 as core::ffi::c_int
    {
        p_width
    } else {
        (1 as core::ffi::c_int) << 5 as core::ffi::c_int
    };
    let error_bsize_h: core::ffi::c_int = if p_height
        < (1 as core::ffi::c_int) << 5 as core::ffi::c_int
    {
        p_height
    } else {
        (1 as core::ffi::c_int) << 5 as core::ffi::c_int
    };
    let mut sum_error: int64_t = 0 as int64_t;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < p_height {
        let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
        while j < p_width {
            let mut seg_x: core::ffi::c_int = j >> WARP_ERROR_BLOCK_LOG;
            let mut seg_y: core::ffi::c_int = i >> WARP_ERROR_BLOCK_LOG;
            if !(*segment_map.offset((seg_y * segment_map_stride + seg_x) as isize) == 0)
            {
                patch_w = if error_bsize_w < p_width - j {
                    error_bsize_w
                } else {
                    p_width - j
                };
                patch_h = if error_bsize_h < p_height - i {
                    error_bsize_h
                } else {
                    p_height - i
                };
                if patch_w == WARP_ERROR_BLOCK && patch_h == WARP_ERROR_BLOCK {
                    sum_error
                        += aom_highbd_sad32x32_c(
                            (ref_0.offset(j as isize).offset((i * ref_stride) as isize)
                                as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
                            ref_stride,
                            (dst.offset(j as isize).offset((i * dst_stride) as isize)
                                as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
                            dst_stride,
                        ) as int64_t;
                } else {
                    sum_error
                        += generic_sad_highbd(
                            ref_0.offset(j as isize).offset((i * ref_stride) as isize),
                            ref_stride,
                            dst.offset(j as isize).offset((i * dst_stride) as isize),
                            dst_stride,
                            patch_w,
                            patch_h,
                        ) as int64_t;
                }
            }
            j += WARP_ERROR_BLOCK;
        }
        i += WARP_ERROR_BLOCK;
    }
    return sum_error;
}
unsafe extern "C-unwind" fn highbd_warp_error(
    mut wm: *mut WarpedMotionParams,
    ref_0: *const uint16_t,
    mut ref_width: core::ffi::c_int,
    mut ref_height: core::ffi::c_int,
    mut ref_stride: core::ffi::c_int,
    dst: *const uint16_t,
    mut dst_stride: core::ffi::c_int,
    mut p_col: core::ffi::c_int,
    mut p_row: core::ffi::c_int,
    mut p_width: core::ffi::c_int,
    mut p_height: core::ffi::c_int,
    mut subsampling_x: core::ffi::c_int,
    mut subsampling_y: core::ffi::c_int,
    mut bd: core::ffi::c_int,
    mut best_error: int64_t,
    mut segment_map: *mut uint8_t,
    mut segment_map_stride: core::ffi::c_int,
) -> int64_t {
    let mut gm_sumerr: int64_t = 0 as int64_t;
    let error_bsize_w: core::ffi::c_int = if p_width
        < (1 as core::ffi::c_int) << 5 as core::ffi::c_int
    {
        p_width
    } else {
        (1 as core::ffi::c_int) << 5 as core::ffi::c_int
    };
    let error_bsize_h: core::ffi::c_int = if p_height
        < (1 as core::ffi::c_int) << 5 as core::ffi::c_int
    {
        p_height
    } else {
        (1 as core::ffi::c_int) << 5 as core::ffi::c_int
    };
    let mut tmp: crate::platform::Aligned32<[uint16_t; 1024]> = crate::platform::Aligned32([0; 1024]);
    let mut conv_params: ConvolveParams = get_conv_params(
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        bd,
    );
    conv_params.use_dist_wtd_comp_avg = 0 as core::ffi::c_int;
    let mut i: core::ffi::c_int = p_row;
    while i < p_row + p_height {
        let mut j: core::ffi::c_int = p_col;
        while j < p_col + p_width {
            let mut seg_x: core::ffi::c_int = j >> WARP_ERROR_BLOCK_LOG;
            let mut seg_y: core::ffi::c_int = i >> WARP_ERROR_BLOCK_LOG;
            if !(*segment_map.offset((seg_y * segment_map_stride + seg_x) as isize) == 0)
            {
                let warp_w: core::ffi::c_int = if error_bsize_w < p_col + ref_width - j {
                    error_bsize_w
                } else {
                    p_col + ref_width - j
                };
                let warp_h: core::ffi::c_int = if error_bsize_h < p_row + ref_height - i
                {
                    error_bsize_h
                } else {
                    p_row + ref_height - i
                };
                highbd_warp_plane(
                    wm,
                    ref_0,
                    ref_width,
                    ref_height,
                    ref_stride,
                    tmp.0.as_mut_ptr(),
                    j,
                    i,
                    warp_w,
                    warp_h,
                    WARP_ERROR_BLOCK,
                    subsampling_x,
                    subsampling_y,
                    bd,
                    &raw mut conv_params,
                );
                if warp_w == WARP_ERROR_BLOCK && warp_h == WARP_ERROR_BLOCK {
                    gm_sumerr
                        += aom_highbd_sad32x32_c(
                            (tmp.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int)
                                as *mut uint8_t,
                            WARP_ERROR_BLOCK,
                            (dst.offset(j as isize).offset((i * dst_stride) as isize)
                                as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
                            dst_stride,
                        ) as int64_t;
                } else {
                    gm_sumerr
                        += generic_sad_highbd(
                            tmp.0.as_mut_ptr(),
                            WARP_ERROR_BLOCK,
                            dst.offset(j as isize).offset((i * dst_stride) as isize),
                            dst_stride,
                            warp_w,
                            warp_h,
                        ) as int64_t;
                }
                if gm_sumerr > best_error {
                    return INT64_MAX as int64_t;
                }
            }
            j += WARP_ERROR_BLOCK;
        }
        i += WARP_ERROR_BLOCK;
    }
    return gm_sumerr;
}
#[inline]
unsafe extern "C-unwind" fn generic_sad(
    ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut p_width: core::ffi::c_int,
    mut p_height: core::ffi::c_int,
) -> core::ffi::c_int {
    if !(p_width <= (1 as core::ffi::c_int) << 5 as core::ffi::c_int
        && p_height <= (1 as core::ffi::c_int) << 5 as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 12],
                [core::ffi::c_char; 12],
            >(*b"generic_sad\0"))
                .as_ptr(),
            b"global_motion.c\0" as *const u8 as *const core::ffi::c_char,
            225 as core::ffi::c_int,
            b"p_width <= WARP_ERROR_BLOCK && p_height <= WARP_ERROR_BLOCK\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    let mut sad: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < p_height {
        let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
        while j < p_width {
            sad
                += abs(
                    *dst.offset((j + i * dst_stride) as isize) as core::ffi::c_int
                        - *ref_0.offset((j + i * ref_stride) as isize)
                            as core::ffi::c_int,
                );
            j += 1;
        }
        i += 1;
    }
    return sad;
}
unsafe extern "C-unwind" fn segmented_frame_error(
    ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut p_width: core::ffi::c_int,
    mut p_height: core::ffi::c_int,
    mut segment_map: *mut uint8_t,
    mut segment_map_stride: core::ffi::c_int,
) -> int64_t {
    let mut patch_w: core::ffi::c_int = 0;
    let mut patch_h: core::ffi::c_int = 0;
    let error_bsize_w: core::ffi::c_int = if p_width
        < (1 as core::ffi::c_int) << 5 as core::ffi::c_int
    {
        p_width
    } else {
        (1 as core::ffi::c_int) << 5 as core::ffi::c_int
    };
    let error_bsize_h: core::ffi::c_int = if p_height
        < (1 as core::ffi::c_int) << 5 as core::ffi::c_int
    {
        p_height
    } else {
        (1 as core::ffi::c_int) << 5 as core::ffi::c_int
    };
    let mut sum_error: int64_t = 0 as int64_t;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < p_height {
        let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
        while j < p_width {
            let mut seg_x: core::ffi::c_int = j >> WARP_ERROR_BLOCK_LOG;
            let mut seg_y: core::ffi::c_int = i >> WARP_ERROR_BLOCK_LOG;
            if !(*segment_map.offset((seg_y * segment_map_stride + seg_x) as isize) == 0)
            {
                patch_w = if error_bsize_w < p_width - j {
                    error_bsize_w
                } else {
                    p_width - j
                };
                patch_h = if error_bsize_h < p_height - i {
                    error_bsize_h
                } else {
                    p_height - i
                };
                if patch_w == WARP_ERROR_BLOCK && patch_h == WARP_ERROR_BLOCK {
                    sum_error
                        += aom_sad32x32_c(
                            ref_0.offset(j as isize).offset((i * ref_stride) as isize),
                            ref_stride,
                            dst.offset(j as isize).offset((i * dst_stride) as isize),
                            dst_stride,
                        ) as int64_t;
                } else {
                    sum_error
                        += generic_sad(
                            ref_0.offset(j as isize).offset((i * ref_stride) as isize),
                            ref_stride,
                            dst.offset(j as isize).offset((i * dst_stride) as isize),
                            dst_stride,
                            patch_w,
                            patch_h,
                        ) as int64_t;
                }
            }
            j += WARP_ERROR_BLOCK;
        }
        i += WARP_ERROR_BLOCK;
    }
    return sum_error;
}
unsafe extern "C-unwind" fn warp_error(
    mut wm: *mut WarpedMotionParams,
    ref_0: *const uint8_t,
    mut ref_width: core::ffi::c_int,
    mut ref_height: core::ffi::c_int,
    mut ref_stride: core::ffi::c_int,
    dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut p_col: core::ffi::c_int,
    mut p_row: core::ffi::c_int,
    mut p_width: core::ffi::c_int,
    mut p_height: core::ffi::c_int,
    mut subsampling_x: core::ffi::c_int,
    mut subsampling_y: core::ffi::c_int,
    mut best_error: int64_t,
    mut segment_map: *mut uint8_t,
    mut segment_map_stride: core::ffi::c_int,
) -> int64_t {
    let mut gm_sumerr: int64_t = 0 as int64_t;
    let mut warp_w: core::ffi::c_int = 0;
    let mut warp_h: core::ffi::c_int = 0;
    let error_bsize_w: core::ffi::c_int = if p_width
        < (1 as core::ffi::c_int) << 5 as core::ffi::c_int
    {
        p_width
    } else {
        (1 as core::ffi::c_int) << 5 as core::ffi::c_int
    };
    let error_bsize_h: core::ffi::c_int = if p_height
        < (1 as core::ffi::c_int) << 5 as core::ffi::c_int
    {
        p_height
    } else {
        (1 as core::ffi::c_int) << 5 as core::ffi::c_int
    };
    let mut tmp: crate::platform::Aligned16<[uint8_t; 1024]> = crate::platform::Aligned16([0; 1024]);
    let mut conv_params: ConvolveParams = get_conv_params(
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        8 as core::ffi::c_int,
    );
    conv_params.use_dist_wtd_comp_avg = 0 as core::ffi::c_int;
    let mut i: core::ffi::c_int = p_row;
    while i < p_row + p_height {
        let mut j: core::ffi::c_int = p_col;
        while j < p_col + p_width {
            let mut seg_x: core::ffi::c_int = j >> WARP_ERROR_BLOCK_LOG;
            let mut seg_y: core::ffi::c_int = i >> WARP_ERROR_BLOCK_LOG;
            if !(*segment_map.offset((seg_y * segment_map_stride + seg_x) as isize) == 0)
            {
                warp_w = if error_bsize_w < p_col + ref_width - j {
                    error_bsize_w
                } else {
                    p_col + ref_width - j
                };
                warp_h = if error_bsize_h < p_row + ref_height - i {
                    error_bsize_h
                } else {
                    p_row + ref_height - i
                };
                warp_plane(
                    wm,
                    ref_0,
                    ref_width,
                    ref_height,
                    ref_stride,
                    tmp.0.as_mut_ptr(),
                    j,
                    i,
                    warp_w,
                    warp_h,
                    WARP_ERROR_BLOCK,
                    subsampling_x,
                    subsampling_y,
                    &raw mut conv_params,
                );
                if warp_w == WARP_ERROR_BLOCK && warp_h == WARP_ERROR_BLOCK {
                    gm_sumerr
                        += aom_sad32x32_c(
                            tmp.0.as_mut_ptr(),
                            WARP_ERROR_BLOCK,
                            dst.offset(j as isize).offset((i * dst_stride) as isize),
                            dst_stride,
                        ) as int64_t;
                } else {
                    gm_sumerr
                        += generic_sad(
                            tmp.0.as_mut_ptr(),
                            WARP_ERROR_BLOCK,
                            dst.offset(j as isize).offset((i * dst_stride) as isize),
                            dst_stride,
                            warp_w,
                            warp_h,
                        ) as int64_t;
                }
                if gm_sumerr > best_error {
                    return INT64_MAX as int64_t;
                }
            }
            j += WARP_ERROR_BLOCK;
        }
        i += WARP_ERROR_BLOCK;
    }
    return gm_sumerr;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_segmented_frame_error(
    mut use_hbd: core::ffi::c_int,
    mut bd: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut dst: *mut uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut p_width: core::ffi::c_int,
    mut p_height: core::ffi::c_int,
    mut segment_map: *mut uint8_t,
    mut segment_map_stride: core::ffi::c_int,
) -> int64_t {
    if use_hbd != 0 {
        return highbd_segmented_frame_error(
            ((ref_0 as uintptr_t) << 1 as core::ffi::c_int) as *mut uint16_t,
            ref_stride,
            ((dst as uintptr_t) << 1 as core::ffi::c_int) as *mut uint16_t,
            dst_stride,
            p_width,
            p_height,
            bd,
            segment_map,
            segment_map_stride,
        );
    }
    return segmented_frame_error(
        ref_0,
        ref_stride,
        dst,
        dst_stride,
        p_width,
        p_height,
        segment_map,
        segment_map_stride,
    );
}
unsafe extern "C-unwind" fn get_warp_error(
    mut wm: *mut WarpedMotionParams,
    mut use_hbd: core::ffi::c_int,
    mut bd: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_width: core::ffi::c_int,
    mut ref_height: core::ffi::c_int,
    mut ref_stride: core::ffi::c_int,
    mut dst: *mut uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut p_col: core::ffi::c_int,
    mut p_row: core::ffi::c_int,
    mut p_width: core::ffi::c_int,
    mut p_height: core::ffi::c_int,
    mut subsampling_x: core::ffi::c_int,
    mut subsampling_y: core::ffi::c_int,
    mut best_error: int64_t,
    mut segment_map: *mut uint8_t,
    mut segment_map_stride: core::ffi::c_int,
) -> int64_t {
    if av1_get_shear_params(wm) == 0 {
        return INT64_MAX as int64_t;
    }
    if use_hbd != 0 {
        return highbd_warp_error(
            wm,
            ((ref_0 as uintptr_t) << 1 as core::ffi::c_int) as *mut uint16_t,
            ref_width,
            ref_height,
            ref_stride,
            ((dst as uintptr_t) << 1 as core::ffi::c_int) as *mut uint16_t,
            dst_stride,
            p_col,
            p_row,
            p_width,
            p_height,
            subsampling_x,
            subsampling_y,
            bd,
            best_error,
            segment_map,
            segment_map_stride,
        );
    }
    return warp_error(
        wm,
        ref_0,
        ref_width,
        ref_height,
        ref_stride,
        dst,
        dst_stride,
        p_col,
        p_row,
        p_width,
        p_height,
        subsampling_x,
        subsampling_y,
        best_error,
        segment_map,
        segment_map_stride,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_refine_integerized_param(
    mut wm: *mut WarpedMotionParams,
    mut wmtype: TransformationType,
    mut use_hbd: core::ffi::c_int,
    mut bd: core::ffi::c_int,
    mut ref_0: *mut uint8_t,
    mut r_width: core::ffi::c_int,
    mut r_height: core::ffi::c_int,
    mut r_stride: core::ffi::c_int,
    mut dst: *mut uint8_t,
    mut d_width: core::ffi::c_int,
    mut d_height: core::ffi::c_int,
    mut d_stride: core::ffi::c_int,
    mut n_refinements: core::ffi::c_int,
    mut ref_frame_error: int64_t,
    mut segment_map: *mut uint8_t,
    mut segment_map_stride: core::ffi::c_int,
    mut gm_erroradv_tr: core::ffi::c_double,
) -> int64_t {
    static mut max_trans_model_params: [core::ffi::c_int; 4] = [
        0 as core::ffi::c_int,
        2 as core::ffi::c_int,
        4 as core::ffi::c_int,
        6 as core::ffi::c_int,
    ];
    let border: core::ffi::c_int = ERRORADV_BORDER;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut p: core::ffi::c_int = 0;
    let mut n_params: core::ffi::c_int = max_trans_model_params[wmtype as usize];
    let mut param_mat: *mut int32_t = ((*wm).wmmat).as_mut_ptr();
    let mut step_error: int64_t = 0;
    let mut best_error: int64_t = 0;
    let mut step: int32_t = 0;
    let mut param: *mut int32_t = 0 as *mut int32_t;
    let mut curr_param: int32_t = 0;
    let mut best_param: int32_t = 0;
    force_wmtype(wm, wmtype);
    (*wm).wmtype = get_wmtype(wm);
    if n_refinements == 0 as core::ffi::c_int {
        let mut selection_threshold: int64_t = lrint(
            ref_frame_error as core::ffi::c_double * gm_erroradv_tr,
        ) as int64_t;
        return get_warp_error(
            wm,
            use_hbd,
            bd,
            ref_0,
            r_width,
            r_height,
            r_stride,
            dst.offset((border * d_stride) as isize).offset(border as isize),
            d_stride,
            border,
            border,
            d_width - 2 as core::ffi::c_int * border,
            d_height - 2 as core::ffi::c_int * border,
            0 as core::ffi::c_int,
            0 as core::ffi::c_int,
            selection_threshold,
            segment_map,
            segment_map_stride,
        );
    }
    let mut selection_threshold_0: int64_t = lrint(
        ref_frame_error as core::ffi::c_double * erroradv_early_tr,
    ) as int64_t;
    best_error = get_warp_error(
        wm,
        use_hbd,
        bd,
        ref_0,
        r_width,
        r_height,
        r_stride,
        dst.offset((border * d_stride) as isize).offset(border as isize),
        d_stride,
        border,
        border,
        d_width - 2 as core::ffi::c_int * border,
        d_height - 2 as core::ffi::c_int * border,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        selection_threshold_0,
        segment_map,
        segment_map_stride,
    );
    if best_error > selection_threshold_0 {
        return INT64_MAX as int64_t;
    }
    step = ((1 as core::ffi::c_int) << n_refinements - 1 as core::ffi::c_int) as int32_t;
    i = 0 as core::ffi::c_int;
    while i < n_refinements {
        p = 0 as core::ffi::c_int;
        while p < n_params {
            let mut step_dir: core::ffi::c_int = 0 as core::ffi::c_int;
            param = param_mat.offset(p as isize);
            curr_param = *param;
            best_param = curr_param;
            *param = add_param_offset(p, curr_param, -step);
            force_wmtype(wm, wmtype);
            step_error = get_warp_error(
                wm,
                use_hbd,
                bd,
                ref_0,
                r_width,
                r_height,
                r_stride,
                dst.offset((border * d_stride) as isize).offset(border as isize),
                d_stride,
                border,
                border,
                d_width - 2 as core::ffi::c_int * border,
                d_height - 2 as core::ffi::c_int * border,
                0 as core::ffi::c_int,
                0 as core::ffi::c_int,
                best_error,
                segment_map,
                segment_map_stride,
            );
            if step_error < best_error {
                best_error = step_error;
                best_param = *param;
                step_dir = -(1 as core::ffi::c_int);
            }
            *param = add_param_offset(p, curr_param, step);
            force_wmtype(wm, wmtype);
            step_error = get_warp_error(
                wm,
                use_hbd,
                bd,
                ref_0,
                r_width,
                r_height,
                r_stride,
                dst.offset((border * d_stride) as isize).offset(border as isize),
                d_stride,
                border,
                border,
                d_width - 2 as core::ffi::c_int * border,
                d_height - 2 as core::ffi::c_int * border,
                0 as core::ffi::c_int,
                0 as core::ffi::c_int,
                best_error,
                segment_map,
                segment_map_stride,
            );
            if step_error < best_error {
                best_error = step_error;
                best_param = *param;
                step_dir = 1 as core::ffi::c_int;
            }
            while step_dir != 0 {
                *param = add_param_offset(p, best_param, step * step_dir as int32_t);
                force_wmtype(wm, wmtype);
                step_error = get_warp_error(
                    wm,
                    use_hbd,
                    bd,
                    ref_0,
                    r_width,
                    r_height,
                    r_stride,
                    dst.offset((border * d_stride) as isize).offset(border as isize),
                    d_stride,
                    border,
                    border,
                    d_width - 2 as core::ffi::c_int * border,
                    d_height - 2 as core::ffi::c_int * border,
                    0 as core::ffi::c_int,
                    0 as core::ffi::c_int,
                    best_error,
                    segment_map,
                    segment_map_stride,
                );
                if step_error < best_error {
                    best_error = step_error;
                    best_param = *param;
                } else {
                    step_dir = 0 as core::ffi::c_int;
                }
            }
            *param = best_param;
            force_wmtype(wm, wmtype);
            p += 1;
        }
        i += 1;
        step >>= 1 as core::ffi::c_int;
    }
    (*wm).wmtype = get_wmtype(wm);
    if av1_get_shear_params(wm) == 0 {
        if (0 as core::ffi::c_int == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 29],
                    [core::ffi::c_char; 29],
                >(*b"av1_refine_integerized_param\0"))
                    .as_ptr(),
                b"global_motion.c\0" as *const u8 as *const core::ffi::c_char,
                476 as core::ffi::c_int,
                b"0\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
    }
    return best_error;
}
pub const FEAT_COUNT_TR: core::ffi::c_int = 3 as core::ffi::c_int;
pub const SEG_COUNT_TR: core::ffi::c_int = 48 as core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_compute_feature_segmentation_map(
    mut segment_map: *mut uint8_t,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut inliers: *mut core::ffi::c_int,
    mut num_inliers: core::ffi::c_int,
) {
    let mut seg_count: core::ffi::c_int = 0 as core::ffi::c_int;
    memset(
        segment_map as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (::core::mem::size_of::<uint8_t>() as size_t)
            .wrapping_mul(width as size_t)
            .wrapping_mul(height as size_t),
    );
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < num_inliers {
        let mut x: core::ffi::c_int = *inliers
            .offset((i * 2 as core::ffi::c_int) as isize);
        let mut y: core::ffi::c_int = *inliers
            .offset((i * 2 as core::ffi::c_int + 1 as core::ffi::c_int) as isize);
        let mut seg_x: core::ffi::c_int = x >> WARP_ERROR_BLOCK_LOG;
        let mut seg_y: core::ffi::c_int = y >> WARP_ERROR_BLOCK_LOG;
        let fresh0 = &raw mut *segment_map.offset((seg_y * width + seg_x) as isize);
        *fresh0 = (*fresh0 as core::ffi::c_int + 1 as core::ffi::c_int) as uint8_t;
        i += 1;
    }
    let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while i_0 < height {
        let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
        while j < width {
            let mut feat_count: uint8_t = *segment_map
                .offset((i_0 * width + j) as isize);
            *segment_map.offset((i_0 * width + j) as isize) = (feat_count
                as core::ffi::c_int >= FEAT_COUNT_TR) as core::ffi::c_int as uint8_t;
            seg_count
                += *segment_map.offset((i_0 * width + j) as isize) as core::ffi::c_int;
            j += 1;
        }
        i_0 += 1;
    }
    if seg_count < SEG_COUNT_TR {
        memset(
            segment_map as *mut core::ffi::c_void,
            1 as core::ffi::c_int,
            ((width * height) as size_t)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
        );
    }
}

extern "C" {
    fn abs(_: core::ffi::c_int) -> core::ffi::c_int;
    fn memset(
        __b: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __len: size_t,
    ) -> *mut core::ffi::c_void;
    fn floor(_: core::ffi::c_double) -> core::ffi::c_double;
    fn lrint(_: core::ffi::c_double) -> core::ffi::c_long;
}
