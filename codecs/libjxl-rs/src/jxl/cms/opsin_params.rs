// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Constants and constexpr cube construction from `lib/jxl/cms/opsin_params.h`.
//! Long source float literals are retained as exact IEEE-754 bit patterns.

use super::super::base::matrix_ops::{Matrix3x3, Vector3};

pub const M02: f32 = f32::from_bits(0x3d9fbe77);
pub const M00: f32 = f32::from_bits(0x3e99999a);
pub const M01: f32 = 1.0 - M02 - M00;
pub const M12: f32 = f32::from_bits(0x3d9fbe77);
pub const M10: f32 = f32::from_bits(0x3e6b851f);
pub const M11: f32 = 1.0 - M12 - M10;
pub const M20: f32 = f32::from_bits(0x3e7943cc);
pub const M21: f32 = f32::from_bits(0x3e51ae8f);
pub const M22: f32 = 1.0 - M20 - M21;
pub const B_SCALE: f32 = 1.0;
pub const Y_TO_B_RATIO: f32 = 1.0;
pub const B_TO_Y_RATIO: f32 = 1.0 / Y_TO_B_RATIO;
pub const OPSIN_ABSORBANCE_BIAS0: f32 = f32::from_bits(0x3b789536);
pub const OPSIN_ABSORBANCE_BIAS1: f32 = OPSIN_ABSORBANCE_BIAS0;
pub const OPSIN_ABSORBANCE_BIAS2: f32 = OPSIN_ABSORBANCE_BIAS0;
pub const OPSIN_ABSORBANCE_MATRIX: Matrix3x3 = [[M00, M01, M02], [M10, M11, M12], [M20, M21, M22]];
pub const DEFAULT_INVERSE_OPSIN_ABSORBANCE_MATRIX: Matrix3x3 = [
    [
        f32::from_bits(0x4130814c),
        f32::from_bits(0xc11ddf01),
        f32::from_bits(0xbe2892ee),
    ],
    [
        f32::from_bits(0xc05043f3),
        f32::from_bits(0x408d6691),
        f32::from_bits(0xbe2892ee),
    ],
    [
        f32::from_bits(0xc06a2a9f),
        f32::from_bits(0x402da088),
        f32::from_bits(0x3ff9142d),
    ],
];
pub fn default_inverse_opsin_absorbance_matrix() -> &'static Matrix3x3 {
    &DEFAULT_INVERSE_OPSIN_ABSORBANCE_MATRIX
}
pub const OPSIN_ABSORBANCE_BIAS: Vector3 = [
    OPSIN_ABSORBANCE_BIAS0,
    OPSIN_ABSORBANCE_BIAS1,
    OPSIN_ABSORBANCE_BIAS2,
];
pub const NEG_OPSIN_ABSORBANCE_BIAS_RGB: [f32; 4] = [
    -OPSIN_ABSORBANCE_BIAS0,
    -OPSIN_ABSORBANCE_BIAS1,
    -OPSIN_ABSORBANCE_BIAS2,
    1.0,
];
pub const SCALED_XYB_OFFSET: Vector3 =
    [f32::from_bits(0x3c7c1620), 0.0, f32::from_bits(0x3e8e2f4c)];
pub const SCALED_XYB_SCALE: Vector3 = [
    f32::from_bits(0x41b7f760),
    f32::from_bits(0x3f976c8c),
    f32::from_bits(0x3fc0462b),
];
// Retain the source's spelling and evaluation order.
pub const fn reciprocial_sum(r1: f32, r2: f32) -> f32 {
    (r1 * r2) / (r1 + r2)
}
pub const XYB_OFFSET: Vector3 = [
    SCALED_XYB_OFFSET[0] + SCALED_XYB_OFFSET[1],
    SCALED_XYB_OFFSET[1] - SCALED_XYB_OFFSET[0] + 1.0 / SCALED_XYB_SCALE[0],
    SCALED_XYB_OFFSET[1] + SCALED_XYB_OFFSET[2],
];
pub const XYB_SCALE: Vector3 = [
    reciprocial_sum(SCALED_XYB_SCALE[0], SCALED_XYB_SCALE[1]),
    reciprocial_sum(SCALED_XYB_SCALE[0], SCALED_XYB_SCALE[1]),
    reciprocial_sum(SCALED_XYB_SCALE[1], SCALED_XYB_SCALE[2]),
];
pub const fn scaled_xyb_scale(idx: usize) -> f32 {
    SCALED_XYB_SCALE[if idx < 2 { idx } else { 2 }]
}
pub const fn scaled_xyb_offset(idx: usize) -> f32 {
    SCALED_XYB_OFFSET[if idx < 2 { idx } else { 2 }]
}
pub const fn xyb_corner(x: usize, y: usize, b: usize, idx: usize) -> f32 {
    (if idx == 0 {
        x
    } else if idx == 1 {
        y
    } else {
        b
    }) as f32
        / scaled_xyb_scale(idx)
        - scaled_xyb_offset(idx)
}
pub const fn scaled_a2b_corner(x: usize, y: usize, b: usize, idx: usize) -> f32 {
    if idx == 0 {
        xyb_corner(x, y, b, 1) + xyb_corner(x, y, b, 0)
    } else if idx == 1 {
        xyb_corner(x, y, b, 1) - xyb_corner(x, y, b, 0)
    } else {
        xyb_corner(x, y, b, 2) + xyb_corner(x, y, b, 1)
    }
}
pub type ColorCube0D = Vector3;
pub type ColorCube1D = [ColorCube0D; 2];
pub type ColorCube2D = [ColorCube1D; 2];
pub type ColorCube3D = [ColorCube2D; 2];
pub const fn unscaled_a2b_corner(x: usize, y: usize, b: usize) -> ColorCube0D {
    [
        (scaled_a2b_corner(x, y, b, 0) + XYB_OFFSET[0]) * XYB_SCALE[0],
        (scaled_a2b_corner(x, y, b, 1) + XYB_OFFSET[1]) * XYB_SCALE[1],
        (scaled_a2b_corner(x, y, b, 2) + XYB_OFFSET[2]) * XYB_SCALE[2],
    ]
}
pub const fn unscaled_a2b_cube_xy(x: usize, y: usize) -> ColorCube1D {
    [unscaled_a2b_corner(x, y, 0), unscaled_a2b_corner(x, y, 1)]
}
pub const fn unscaled_a2b_cube_x(x: usize) -> ColorCube2D {
    [unscaled_a2b_cube_xy(x, 0), unscaled_a2b_cube_xy(x, 1)]
}
pub const fn unscaled_a2b_cube() -> ColorCube3D {
    [unscaled_a2b_cube_x(0), unscaled_a2b_cube_x(1)]
}
pub const UNSCALED_A2B_CUBE: ColorCube3D = unscaled_a2b_cube();
