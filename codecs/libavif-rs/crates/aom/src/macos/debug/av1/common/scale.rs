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

// Mechanically translated from upstream/aom/av1/common/scale.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
pub type int16_t = i16;
pub type int32_t = i32;
pub type int64_t = i64;
pub type MV = mv;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mv {
    pub row: int16_t,
    pub col: int16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct scale_factors {
    pub x_scale_fp: core::ffi::c_int,
    pub y_scale_fp: core::ffi::c_int,
    pub x_step_q4: core::ffi::c_int,
    pub y_step_q4: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mv32 {
    pub row: int32_t,
    pub col: int32_t,
}
pub type MV32 = mv32;
pub const SUBPEL_BITS: core::ffi::c_int = 4 as core::ffi::c_int;
pub const REF_SCALE_SHIFT: core::ffi::c_int = 14 as core::ffi::c_int;
pub const REF_INVALID_SCALE: core::ffi::c_int = -(1 as core::ffi::c_int);
#[inline]
unsafe extern "C-unwind" fn av1_scaled_x(
    mut val: core::ffi::c_int,
    mut sf: *const scale_factors,
) -> core::ffi::c_int {
    let off: core::ffi::c_int = ((*sf).x_scale_fp
        - ((1 as core::ffi::c_int) << REF_SCALE_SHIFT))
        * ((1 as core::ffi::c_int) << SUBPEL_BITS - 1 as core::ffi::c_int);
    let tval: int64_t = val as int64_t * (*sf).x_scale_fp as int64_t + off as int64_t;
    return (if tval < 0 as int64_t {
        -(-tval
            + ((1 as core::ffi::c_int as int64_t)
                << 14 as core::ffi::c_int
                    - (10 as core::ffi::c_int - 4 as core::ffi::c_int)
                >> 1 as core::ffi::c_int)
            >> 14 as core::ffi::c_int - (10 as core::ffi::c_int - 4 as core::ffi::c_int))
    } else {
        tval
            + ((1 as core::ffi::c_int as int64_t)
                << 14 as core::ffi::c_int
                    - (10 as core::ffi::c_int - 4 as core::ffi::c_int)
                >> 1 as core::ffi::c_int)
            >> 14 as core::ffi::c_int - (10 as core::ffi::c_int - 4 as core::ffi::c_int)
    }) as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn av1_scaled_y(
    mut val: core::ffi::c_int,
    mut sf: *const scale_factors,
) -> core::ffi::c_int {
    let off: core::ffi::c_int = ((*sf).y_scale_fp
        - ((1 as core::ffi::c_int) << REF_SCALE_SHIFT))
        * ((1 as core::ffi::c_int) << SUBPEL_BITS - 1 as core::ffi::c_int);
    let tval: int64_t = val as int64_t * (*sf).y_scale_fp as int64_t + off as int64_t;
    return (if tval < 0 as int64_t {
        -(-tval
            + ((1 as core::ffi::c_int as int64_t)
                << 14 as core::ffi::c_int
                    - (10 as core::ffi::c_int - 4 as core::ffi::c_int)
                >> 1 as core::ffi::c_int)
            >> 14 as core::ffi::c_int - (10 as core::ffi::c_int - 4 as core::ffi::c_int))
    } else {
        tval
            + ((1 as core::ffi::c_int as int64_t)
                << 14 as core::ffi::c_int
                    - (10 as core::ffi::c_int - 4 as core::ffi::c_int)
                >> 1 as core::ffi::c_int)
            >> 14 as core::ffi::c_int - (10 as core::ffi::c_int - 4 as core::ffi::c_int)
    }) as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn valid_ref_frame_size(
    mut ref_width: core::ffi::c_int,
    mut ref_height: core::ffi::c_int,
    mut this_width: core::ffi::c_int,
    mut this_height: core::ffi::c_int,
) -> core::ffi::c_int {
    return (2 as core::ffi::c_int * this_width >= ref_width
        && 2 as core::ffi::c_int * this_height >= ref_height
        && this_width <= 16 as core::ffi::c_int * ref_width
        && this_height <= 16 as core::ffi::c_int * ref_height) as core::ffi::c_int;
}
unsafe extern "C-unwind" fn get_fixed_point_scale_factor(
    mut other_size: core::ffi::c_int,
    mut this_size: core::ffi::c_int,
) -> core::ffi::c_int {
    return ((other_size << REF_SCALE_SHIFT) + this_size / 2 as core::ffi::c_int)
        / this_size;
}
unsafe extern "C-unwind" fn fixed_point_scale_to_coarse_point_scale(
    mut scale_fp: core::ffi::c_int,
) -> core::ffi::c_int {
    return scale_fp
        + ((1 as core::ffi::c_int) << 14 as core::ffi::c_int - 10 as core::ffi::c_int
            >> 1 as core::ffi::c_int) >> 14 as core::ffi::c_int - 10 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_scale_mv(
    mut mvq4: *const MV,
    mut x: core::ffi::c_int,
    mut y: core::ffi::c_int,
    mut sf: *const scale_factors,
) -> MV32 {
    let x_off_q4: core::ffi::c_int = av1_scaled_x(x << SUBPEL_BITS, sf)
        as core::ffi::c_int;
    let y_off_q4: core::ffi::c_int = av1_scaled_y(y << SUBPEL_BITS, sf)
        as core::ffi::c_int;
    let res: MV32 = {
        let mut init = mv32 {
            row: av1_scaled_y((y << SUBPEL_BITS) + (*mvq4).row as core::ffi::c_int, sf)
                as int32_t - y_off_q4 as int32_t,
            col: av1_scaled_x((x << SUBPEL_BITS) + (*mvq4).col as core::ffi::c_int, sf)
                as int32_t - x_off_q4 as int32_t,
        };
        init
    };
    return res;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_setup_scale_factors_for_frame(
    mut sf: *mut scale_factors,
    mut other_w: core::ffi::c_int,
    mut other_h: core::ffi::c_int,
    mut this_w: core::ffi::c_int,
    mut this_h: core::ffi::c_int,
) {
    if valid_ref_frame_size(other_w, other_h, this_w, this_h) == 0 {
        (*sf).x_scale_fp = REF_INVALID_SCALE;
        (*sf).y_scale_fp = REF_INVALID_SCALE;
        return;
    }
    (*sf).x_scale_fp = get_fixed_point_scale_factor(other_w, this_w);
    (*sf).y_scale_fp = get_fixed_point_scale_factor(other_h, this_h);
    (*sf).x_step_q4 = fixed_point_scale_to_coarse_point_scale((*sf).x_scale_fp);
    (*sf).y_step_q4 = fixed_point_scale_to_coarse_point_scale((*sf).y_scale_fp);
}
