// libwebp v1.6.0, sharpyuv/sharpyuv_csp.c:
//
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

// libwebp v1.6.0, sharpyuv/sharpyuv_csp.h:
//
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

//! Translation of `sharpyuv/sharpyuv_csp.c` and `sharpyuv/sharpyuv_csp.h` from libwebp v1.6.0.
//!
//! Work package: WP5 (`SharpYUV`).
//!
//! # Deviations from upstream
//!
//! Changes of representation that the ADRs allow, or behavior that the Rust
//! side cannot reproduce exactly. Each records the reason.
//!
//! 1. **Out-of-range float conversions.** C converts a float that is out of
//!    range for an integer type with an undefined conversion. Rust `as`
//!    saturates. This is reachable only in the excluded cases of
//!    `# Excluded undefined behavior`: item 2 here and item 1 of
//!    `src/sharpyuv/sharpyuv_gamma.rs`.
//!
//! # Excluded undefined behavior
//!
//! Inputs that are undefined behavior (or assertion failures) in C and are
//! therefore excluded from the oracle comparison (PORTING rule 8):
//!
//! 1. **Seeded color spaces (`csp.c`, `wp5_matrix.rs`).** The seeded color
//!    spaces draw `kr` and `kb` from `[0, 0.5)` and the bit depth from 8 to 15.
//!    In `SharpYuvComputeConversionMatrix`, `ToFixed16` converts
//!    `f * 65536 + 0.5` to `int`, which is undefined when `|f| >= 32768`. With
//!    these bounds every scale factor is at most 1 and every coefficient is at
//!    most 1 in magnitude, so every intermediate fits `int`.
//! 2. **Bit depth 16 in `SharpYuvComputeConversionMatrix`.** The offset
//!    `128 << (bit_depth - 8)` is `2^15` at 16 bits, and `ToFixed16` of
//!    `2^15 * 65536` overflows `int`, for every color space. The C result is
//!    undefined: an oracle run observed `INT_MIN`, and the Rust function
//!    saturates to `i32::MAX`. The bit depth 16 is excluded from the seeded
//!    color spaces. The computed matrices of the inventory (see
//!    `tests/wp5_matrix.rs`) use 8, 10 and 12 bits. `SharpYuvConvert` does not
//!    use computed matrices, so no conversion reaches this case.
//!
//! # Checks not applicable
//!
//! These checks do not apply, or cannot be made, with the reason:
//!
//! 1. **Floating point across hosts.** The matrix computation uses `float`; the
//!    full text is in `# Checks not applicable` (item 1) of
//!    `src/sharpyuv/sharpyuv_gamma.rs`.

use super::sharpyuv::SharpYuvConversionMatrix;

// upstream: sharpyuv/sharpyuv_csp.h SharpYuvRange
/// The value range of a color space; a C enumeration stored as its `unsigned int`.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SharpYuvRange(pub u32);

impl SharpYuvRange {
    // upstream: sharpyuv/sharpyuv_csp.h kSharpYuvRangeFull
    /// YUV values between [0;255] (for 8 bit).
    #[expect(non_upper_case_globals, reason = "C enumerator name")]
    pub const kSharpYuvRangeFull: Self = Self(0);
    // upstream: sharpyuv/sharpyuv_csp.h kSharpYuvRangeLimited
    /// Y in [16;235], YUV in [16;240] (for 8 bit).
    #[expect(non_upper_case_globals, reason = "C enumerator name")]
    pub const kSharpYuvRangeLimited: Self = Self(1);
}

// upstream: sharpyuv/sharpyuv_csp.h SharpYuvColorSpace
/// A color space, from which `sharp_yuv_compute_conversion_matrix` derives a matrix.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SharpYuvColorSpace {
    /// Kr and Kb are defined such that: Y = Kr * r + Kg * g + Kb * b where Kg = 1 - Kr - Kb.
    pub kr: f32,
    /// See `kr`.
    pub kb: f32,
    /// 8, 10 or 12.
    pub bit_depth: i32,
    /// The value range.
    pub range: SharpYuvRange,
}

// upstream: sharpyuv/sharpyuv_csp.h SharpYuvMatrixType
/// A predefined conversion matrix; a C enumeration stored as its `unsigned int`.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SharpYuvMatrixType(pub u32);

impl SharpYuvMatrixType {
    // upstream: sharpyuv/sharpyuv_csp.h kSharpYuvMatrixWebp
    /// WebP's matrix, similar but not identical to kSharpYuvMatrixRec601Limited.
    #[expect(non_upper_case_globals, reason = "C enumerator name")]
    pub const kSharpYuvMatrixWebp: Self = Self(0);
    // upstream: sharpyuv/sharpyuv_csp.h kSharpYuvMatrixRec601Limited
    /// Kr=0.2990f, Kb=0.1140f, 8 bits, range `kSharpYuvRangeLimited`.
    #[expect(non_upper_case_globals, reason = "C enumerator name")]
    pub const kSharpYuvMatrixRec601Limited: Self = Self(1);
    // upstream: sharpyuv/sharpyuv_csp.h kSharpYuvMatrixRec601Full
    /// Kr=0.2990f, Kb=0.1140f, 8 bits, range `kSharpYuvRangeFull`.
    #[expect(non_upper_case_globals, reason = "C enumerator name")]
    pub const kSharpYuvMatrixRec601Full: Self = Self(2);
    // upstream: sharpyuv/sharpyuv_csp.h kSharpYuvMatrixRec709Limited
    /// Kr=0.2126f, Kb=0.0722f, 8 bits, range `kSharpYuvRangeLimited`.
    #[expect(non_upper_case_globals, reason = "C enumerator name")]
    pub const kSharpYuvMatrixRec709Limited: Self = Self(3);
    // upstream: sharpyuv/sharpyuv_csp.h kSharpYuvMatrixRec709Full
    /// Kr=0.2126f, Kb=0.0722f, 8 bits, range `kSharpYuvRangeFull`.
    #[expect(non_upper_case_globals, reason = "C enumerator name")]
    pub const kSharpYuvMatrixRec709Full: Self = Self(4);
    // upstream: sharpyuv/sharpyuv_csp.h kSharpYuvMatrixNum
    /// The number of predefined matrices; not a matrix.
    #[expect(non_upper_case_globals, reason = "C enumerator name")]
    pub const kSharpYuvMatrixNum: Self = Self(5);
}

// upstream: sharpyuv/sharpyuv_csp.c ToFixed16
#[expect(
    clippy::cast_possible_truncation,
    reason = "C integer and float conversions, kept as in the source (ADR 0003)"
)]
fn to_fixed16(f: f32) -> i32 {
    (f64::from(f * 65536.0 + 0.5)).floor() as i32
}

// upstream: sharpyuv/sharpyuv_csp.c SharpYuvComputeConversionMatrix
/// Computes the RGB to YUV matrix of `yuv_color_space`.
#[must_use]
#[expect(
    clippy::cast_precision_loss,
    reason = "C int to float conversions of the scale factors"
)]
pub fn sharp_yuv_compute_conversion_matrix(
    yuv_color_space: &SharpYuvColorSpace,
) -> SharpYuvConversionMatrix {
    let kr = yuv_color_space.kr;
    let kb = yuv_color_space.kb;
    let kg = 1.0_f32 - kr - kb;
    let cb = 0.5_f32 / (1.0_f32 - kb);
    let cr = 0.5_f32 / (1.0_f32 - kr);

    let shift = yuv_color_space.bit_depth - 8;
    debug_assert!(yuv_color_space.bit_depth >= 8);

    let denom = ((1 << yuv_color_space.bit_depth) - 1) as f32;
    let mut scale_y = 1.0_f32;
    let mut add_y = 0.0_f32;
    let mut scale_u = cb;
    let mut scale_v = cr;
    let add_uv = (128 << shift) as f32;

    if yuv_color_space.range == SharpYuvRange::kSharpYuvRangeLimited {
        scale_y *= (219 << shift) as f32 / denom;
        scale_u *= (224 << shift) as f32 / denom;
        scale_v *= (224 << shift) as f32 / denom;
        add_y = (16 << shift) as f32;
    }

    SharpYuvConversionMatrix {
        rgb_to_y: [
            to_fixed16(kr * scale_y),
            to_fixed16(kg * scale_y),
            to_fixed16(kb * scale_y),
            to_fixed16(add_y),
        ],
        rgb_to_u: [
            to_fixed16(-kr * scale_u),
            to_fixed16(-kg * scale_u),
            to_fixed16((1.0 - kb) * scale_u),
            to_fixed16(add_uv),
        ],
        rgb_to_v: [
            to_fixed16((1.0 - kr) * scale_v),
            to_fixed16(-kg * scale_v),
            to_fixed16(-kb * scale_v),
            to_fixed16(add_uv),
        ],
    }
}

// upstream: sharpyuv/sharpyuv_csp.c kWebpMatrix
/// WebP's predefined matrix, the first of the `SharpYuvMatrixType` values.
const WEBP_MATRIX: SharpYuvConversionMatrix = SharpYuvConversionMatrix {
    rgb_to_y: [16839, 33059, 6420, 16 << 16],
    rgb_to_u: [-9719, -19081, 28800, 128 << 16],
    rgb_to_v: [28800, -24116, -4684, 128 << 16],
};
// upstream: sharpyuv/sharpyuv_csp.c kRec601LimitedMatrix
const REC601_LIMITED_MATRIX: SharpYuvConversionMatrix = SharpYuvConversionMatrix {
    rgb_to_y: [16829, 33039, 6416, 16 << 16],
    rgb_to_u: [-9714, -19071, 28784, 128 << 16],
    rgb_to_v: [28784, -24103, -4681, 128 << 16],
};
// upstream: sharpyuv/sharpyuv_csp.c kRec601FullMatrix
const REC601_FULL_MATRIX: SharpYuvConversionMatrix = SharpYuvConversionMatrix {
    rgb_to_y: [19595, 38470, 7471, 0],
    rgb_to_u: [-11058, -21710, 32768, 128 << 16],
    rgb_to_v: [32768, -27439, -5329, 128 << 16],
};
// upstream: sharpyuv/sharpyuv_csp.c kRec709LimitedMatrix
const REC709_LIMITED_MATRIX: SharpYuvConversionMatrix = SharpYuvConversionMatrix {
    rgb_to_y: [11966, 40254, 4064, 16 << 16],
    rgb_to_u: [-6596, -22189, 28784, 128 << 16],
    rgb_to_v: [28784, -26145, -2639, 128 << 16],
};
// upstream: sharpyuv/sharpyuv_csp.c kRec709FullMatrix
const REC709_FULL_MATRIX: SharpYuvConversionMatrix = SharpYuvConversionMatrix {
    rgb_to_y: [13933, 46871, 4732, 0],
    rgb_to_u: [-7509, -25259, 32768, 128 << 16],
    rgb_to_v: [32768, -29763, -3005, 128 << 16],
};

// upstream: sharpyuv/sharpyuv_csp.c SharpYuvGetConversionMatrix
/// Returns the predefined matrix of `matrix_type`, or `None` when the value is not one of
/// the five predefined matrices (the C function returns `NULL`).
#[must_use]
pub fn sharp_yuv_get_conversion_matrix(
    matrix_type: SharpYuvMatrixType,
) -> Option<&'static SharpYuvConversionMatrix> {
    match matrix_type {
        SharpYuvMatrixType::kSharpYuvMatrixWebp => Some(&WEBP_MATRIX),
        SharpYuvMatrixType::kSharpYuvMatrixRec601Limited => Some(&REC601_LIMITED_MATRIX),
        SharpYuvMatrixType::kSharpYuvMatrixRec601Full => Some(&REC601_FULL_MATRIX),
        SharpYuvMatrixType::kSharpYuvMatrixRec709Limited => Some(&REC709_LIMITED_MATRIX),
        SharpYuvMatrixType::kSharpYuvMatrixRec709Full => Some(&REC709_FULL_MATRIX),
        _ => None,
    }
}
