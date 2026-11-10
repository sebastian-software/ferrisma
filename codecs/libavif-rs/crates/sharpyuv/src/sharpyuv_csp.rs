// Copyright 2022 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// Colorspace utilities.

// Mechanically translated from upstream/libwebp/sharpyuv/sharpyuv_csp.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C" {
    fn floor(_: core::ffi::c_double) -> core::ffi::c_double;
}
pub use crate::sharpyuv::SharpYuvConversionMatrix;
pub type SharpYuvRange = core::ffi::c_uint;
pub const kSharpYuvRangeLimited: SharpYuvRange = 1;
pub const kSharpYuvRangeFull: SharpYuvRange = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SharpYuvColorSpace {
    pub kr: core::ffi::c_float,
    pub kb: core::ffi::c_float,
    pub bit_depth: core::ffi::c_int,
    pub range: SharpYuvRange,
}
pub type SharpYuvMatrixType = core::ffi::c_uint;
pub const kSharpYuvMatrixNum: SharpYuvMatrixType = 5;
pub const kSharpYuvMatrixRec709Full: SharpYuvMatrixType = 4;
pub const kSharpYuvMatrixRec709Limited: SharpYuvMatrixType = 3;
pub const kSharpYuvMatrixRec601Full: SharpYuvMatrixType = 2;
pub const kSharpYuvMatrixRec601Limited: SharpYuvMatrixType = 1;
pub const kSharpYuvMatrixWebp: SharpYuvMatrixType = 0;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
unsafe extern "C" fn ToFixed16(mut f: core::ffi::c_float) -> core::ffi::c_int {
    return floor(
        (f * ((1 as core::ffi::c_int) << 16 as core::ffi::c_int) as core::ffi::c_float
            + 0.5f32) as core::ffi::c_double,
    ) as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn SharpYuvComputeConversionMatrix(
    mut yuv_color_space: *const SharpYuvColorSpace,
    mut matrix: *mut SharpYuvConversionMatrix,
) {
    let kr: core::ffi::c_float = (*yuv_color_space).kr;
    let kb: core::ffi::c_float = (*yuv_color_space).kb;
    let kg: core::ffi::c_float = 1.0f32 - kr - kb;
    let cb: core::ffi::c_float = 0.5f32 / (1.0f32 - kb);
    let cr: core::ffi::c_float = 0.5f32 / (1.0f32 - kr);
    let shift: core::ffi::c_int = (*yuv_color_space).bit_depth - 8 as core::ffi::c_int;
    let denom: core::ffi::c_float = (((1 as core::ffi::c_int)
        << (*yuv_color_space).bit_depth) - 1 as core::ffi::c_int) as core::ffi::c_float;
    let mut scale_y: core::ffi::c_float = 1.0f32;
    let mut add_y: core::ffi::c_float = 0.0f32;
    let mut scale_u: core::ffi::c_float = cb;
    let mut scale_v: core::ffi::c_float = cr;
    let mut add_uv: core::ffi::c_float = ((128 as core::ffi::c_int) << shift)
        as core::ffi::c_float;
    if !((*yuv_color_space).bit_depth >= 8 as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 32],
                [core::ffi::c_char; 32],
            >(*b"SharpYuvComputeConversionMatrix\0"))
                .as_ptr(),
            b"sharpyuv_csp.c\0" as *const u8 as *const core::ffi::c_char,
            38 as core::ffi::c_int,
            b"yuv_color_space->bit_depth >= 8\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if (*yuv_color_space).range as core::ffi::c_uint
        == kSharpYuvRangeLimited as core::ffi::c_int as core::ffi::c_uint
    {
        scale_y *= ((219 as core::ffi::c_int) << shift) as core::ffi::c_float / denom;
        scale_u *= ((224 as core::ffi::c_int) << shift) as core::ffi::c_float / denom;
        scale_v *= ((224 as core::ffi::c_int) << shift) as core::ffi::c_float / denom;
        add_y = ((16 as core::ffi::c_int) << shift) as core::ffi::c_float;
    }
    (*matrix).rgb_to_y[0 as core::ffi::c_int as usize] = ToFixed16(kr * scale_y);
    (*matrix).rgb_to_y[1 as core::ffi::c_int as usize] = ToFixed16(kg * scale_y);
    (*matrix).rgb_to_y[2 as core::ffi::c_int as usize] = ToFixed16(kb * scale_y);
    (*matrix).rgb_to_y[3 as core::ffi::c_int as usize] = ToFixed16(add_y);
    (*matrix).rgb_to_u[0 as core::ffi::c_int as usize] = ToFixed16(-kr * scale_u);
    (*matrix).rgb_to_u[1 as core::ffi::c_int as usize] = ToFixed16(-kg * scale_u);
    (*matrix).rgb_to_u[2 as core::ffi::c_int as usize] = ToFixed16(
        (1 as core::ffi::c_int as core::ffi::c_float - kb) * scale_u,
    );
    (*matrix).rgb_to_u[3 as core::ffi::c_int as usize] = ToFixed16(add_uv);
    (*matrix).rgb_to_v[0 as core::ffi::c_int as usize] = ToFixed16(
        (1 as core::ffi::c_int as core::ffi::c_float - kr) * scale_v,
    );
    (*matrix).rgb_to_v[1 as core::ffi::c_int as usize] = ToFixed16(-kg * scale_v);
    (*matrix).rgb_to_v[2 as core::ffi::c_int as usize] = ToFixed16(-kb * scale_v);
    (*matrix).rgb_to_v[3 as core::ffi::c_int as usize] = ToFixed16(add_uv);
}
static mut kWebpMatrix: SharpYuvConversionMatrix = {
    let mut init = SharpYuvConversionMatrix {
        rgb_to_y: [
            16839 as core::ffi::c_int,
            33059 as core::ffi::c_int,
            6420 as core::ffi::c_int,
            (16 as core::ffi::c_int) << 16 as core::ffi::c_int,
        ],
        rgb_to_u: [
            -(9719 as core::ffi::c_int),
            -(19081 as core::ffi::c_int),
            28800 as core::ffi::c_int,
            (128 as core::ffi::c_int) << 16 as core::ffi::c_int,
        ],
        rgb_to_v: [
            28800 as core::ffi::c_int,
            -(24116 as core::ffi::c_int),
            -(4684 as core::ffi::c_int),
            (128 as core::ffi::c_int) << 16 as core::ffi::c_int,
        ],
    };
    init
};
static mut kRec601LimitedMatrix: SharpYuvConversionMatrix = {
    let mut init = SharpYuvConversionMatrix {
        rgb_to_y: [
            16829 as core::ffi::c_int,
            33039 as core::ffi::c_int,
            6416 as core::ffi::c_int,
            (16 as core::ffi::c_int) << 16 as core::ffi::c_int,
        ],
        rgb_to_u: [
            -(9714 as core::ffi::c_int),
            -(19071 as core::ffi::c_int),
            28784 as core::ffi::c_int,
            (128 as core::ffi::c_int) << 16 as core::ffi::c_int,
        ],
        rgb_to_v: [
            28784 as core::ffi::c_int,
            -(24103 as core::ffi::c_int),
            -(4681 as core::ffi::c_int),
            (128 as core::ffi::c_int) << 16 as core::ffi::c_int,
        ],
    };
    init
};
static mut kRec601FullMatrix: SharpYuvConversionMatrix = {
    let mut init = SharpYuvConversionMatrix {
        rgb_to_y: [
            19595 as core::ffi::c_int,
            38470 as core::ffi::c_int,
            7471 as core::ffi::c_int,
            0 as core::ffi::c_int,
        ],
        rgb_to_u: [
            -(11058 as core::ffi::c_int),
            -(21710 as core::ffi::c_int),
            32768 as core::ffi::c_int,
            (128 as core::ffi::c_int) << 16 as core::ffi::c_int,
        ],
        rgb_to_v: [
            32768 as core::ffi::c_int,
            -(27439 as core::ffi::c_int),
            -(5329 as core::ffi::c_int),
            (128 as core::ffi::c_int) << 16 as core::ffi::c_int,
        ],
    };
    init
};
static mut kRec709LimitedMatrix: SharpYuvConversionMatrix = {
    let mut init = SharpYuvConversionMatrix {
        rgb_to_y: [
            11966 as core::ffi::c_int,
            40254 as core::ffi::c_int,
            4064 as core::ffi::c_int,
            (16 as core::ffi::c_int) << 16 as core::ffi::c_int,
        ],
        rgb_to_u: [
            -(6596 as core::ffi::c_int),
            -(22189 as core::ffi::c_int),
            28784 as core::ffi::c_int,
            (128 as core::ffi::c_int) << 16 as core::ffi::c_int,
        ],
        rgb_to_v: [
            28784 as core::ffi::c_int,
            -(26145 as core::ffi::c_int),
            -(2639 as core::ffi::c_int),
            (128 as core::ffi::c_int) << 16 as core::ffi::c_int,
        ],
    };
    init
};
static mut kRec709FullMatrix: SharpYuvConversionMatrix = {
    let mut init = SharpYuvConversionMatrix {
        rgb_to_y: [
            13933 as core::ffi::c_int,
            46871 as core::ffi::c_int,
            4732 as core::ffi::c_int,
            0 as core::ffi::c_int,
        ],
        rgb_to_u: [
            -(7509 as core::ffi::c_int),
            -(25259 as core::ffi::c_int),
            32768 as core::ffi::c_int,
            (128 as core::ffi::c_int) << 16 as core::ffi::c_int,
        ],
        rgb_to_v: [
            32768 as core::ffi::c_int,
            -(29763 as core::ffi::c_int),
            -(3005 as core::ffi::c_int),
            (128 as core::ffi::c_int) << 16 as core::ffi::c_int,
        ],
    };
    init
};
#[no_mangle]
pub unsafe extern "C" fn SharpYuvGetConversionMatrix(
    mut matrix_type: SharpYuvMatrixType,
) -> *const SharpYuvConversionMatrix {
    match matrix_type as core::ffi::c_uint {
        0 => return core::ptr::addr_of!(kWebpMatrix),
        1 => return core::ptr::addr_of!(kRec601LimitedMatrix),
        2 => return core::ptr::addr_of!(kRec601FullMatrix),
        3 => return core::ptr::addr_of!(kRec709LimitedMatrix),
        4 => return core::ptr::addr_of!(kRec709FullMatrix),
        5 => return 0 as *const SharpYuvConversionMatrix,
        _ => {}
    }
    return 0 as *const SharpYuvConversionMatrix;
}
