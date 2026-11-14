// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! One-lane Highway ToXYB, LinearRGBRowToXYB and ComputePremulAbsorb from
//! lib/jxl/enc_xyb.{h,cc}, with OpsinAbsorbance, StoreXYB, LinearRGBToXYB,
//! LinearSRGBToXYB, SRGBToXYB and SRGBToXYBAndLinear. Highway's scalar Mul is
//! `scalar_mul`; MulAdd adds its product in a separate binary32 operation. The
//! pinned AArch64 and x86-64 objects contain no fused operation in this unit.
//!
//! ToXYB is translated for the two color encodings that avoid the color
//! transform: linear sRGB and sRGB, gray or color. `XybInputEncoding` admits
//! exactly those, so ApplyColorTransform (enc_image_bundle.cc), its CMS
//! interface argument and its `black` plane use are not selectable; only the
//! source's `black` size check remains. ScaleXYB, ScaleXYBRow and RgbToYcbcr
//! remain unported.
use super::base::data_parallel::{ThreadPool, run_on_pool_exclusive_no_init};
use super::base::fast_math::cube_root_and_add;
use super::base::hwy_scalar::zero_if_negative;
use super::base::status::{Status, StatusCode};
use super::cms::opsin_params::{OPSIN_ABSORBANCE_BIAS, OPSIN_ABSORBANCE_MATRIX};
use super::cms::transfer_functions_scalar::srgb_display_from_encoded;
use super::color_encoding_internal::ColorEncoding;
use super::dct::scalar_mul;
use super::image::{Image3F, ImageF};
use super::image_ops::copy_image3_to;
use super::memory_manager_internal::AlignedMemory;

/// Lanes(HWY_FULL(float)) of the scalar target.
const N: usize = 1;

/// The ToXYB branch that a color encoding selects, decided in source order:
/// first SameColorEncoding with LinearSRGB of the same grayness, then IsSRGB.
/// Other encodings take ApplyColorTransform, which is not translated, so they
/// have no value of this type.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum XybInputEncoding {
    LinearSrgb,
    Srgb,
}
impl XybInputEncoding {
    pub fn new(c_current: &ColorEncoding) -> Option<Self> {
        let c_linear_srgb = ColorEncoding::linear_srgb(c_current.is_gray());
        if c_linear_srgb.same_color_encoding(c_current) {
            Some(Self::LinearSrgb)
        } else if c_current.is_srgb() {
            Some(Self::Srgb)
        } else {
            None
        }
    }
}

fn mul_add(mul: f32, x: f32, add: f32) -> f32 {
    scalar_mul(mul, x) + add
}

// 4x3 matrix * 3x1 SIMD vectors
fn opsin_absorbance(r: f32, g: f32, b: f32, premul_absorb: &[f32]) -> [f32; 3] {
    let bias = &OPSIN_ABSORBANCE_BIAS;
    let m0 = premul_absorb[0];
    let m1 = premul_absorb[N];
    let m2 = premul_absorb[2 * N];
    let m3 = premul_absorb[3 * N];
    let m4 = premul_absorb[4 * N];
    let m5 = premul_absorb[5 * N];
    let m6 = premul_absorb[6 * N];
    let m7 = premul_absorb[7 * N];
    let m8 = premul_absorb[8 * N];
    let mixed0 = mul_add(m0, r, mul_add(m1, g, mul_add(m2, b, bias[0])));
    let mixed1 = mul_add(m3, r, mul_add(m4, g, mul_add(m5, b, bias[1])));
    let mixed2 = mul_add(m6, r, mul_add(m7, g, mul_add(m8, b, bias[2])));
    [mixed0, mixed1, mixed2]
}

fn store_xyb(r: f32, g: f32, b: f32) -> [f32; 3] {
    let half = 0.5f32;
    [scalar_mul(half, r - g), scalar_mul(half, r + g), b]
}

// Converts one RGB vector to XYB.
fn linear_rgb_to_xyb(r: f32, g: f32, b: f32, premul_absorb: &[f32]) -> [f32; 3] {
    let [mut mixed0, mut mixed1, mut mixed2] = opsin_absorbance(r, g, b, premul_absorb);

    // mixed* should be non-negative even for wide-gamut, so clamp to zero.
    mixed0 = zero_if_negative(mixed0);
    mixed1 = zero_if_negative(mixed1);
    mixed2 = zero_if_negative(mixed2);

    mixed0 = cube_root_and_add(mixed0, premul_absorb[9 * N]);
    mixed1 = cube_root_and_add(mixed1, premul_absorb[10 * N]);
    mixed2 = cube_root_and_add(mixed2, premul_absorb[11 * N]);
    store_xyb(mixed0, mixed1, mixed2)

    // For wide-gamut inputs, r/g/b and valx (but not y/z) are often negative.
}

/// Original LinearRGBRowToXYB, in place over the first `xsize` samples of the
/// three rows.
pub fn linear_rgb_row_to_xyb(
    row0: &mut [f32],
    row1: &mut [f32],
    row2: &mut [f32],
    premul_absorb: &[f32],
    xsize: usize,
) {
    let mut x = 0;
    while x < xsize {
        let r = row0[x];
        let g = row1[x];
        let b = row2[x];
        [row0[x], row1[x], row2[x]] = linear_rgb_to_xyb(r, g, b, premul_absorb);
        x += N;
    }
}

// Input/output uses the codec.h scaling: nominally 0-1 if in-gamut.
fn linear_from_srgb(encoded: f32) -> f32 {
    srgb_display_from_encoded(encoded)
}

fn linear_srgb_to_xyb(
    premul_absorb: &[f32],
    pool: Option<&ThreadPool>,
    image: &mut Image3F,
) -> Status {
    let xsize = image.xsize();
    let ysize = image.ysize();

    let process_row = |image: &mut Image3F, task: u32, _thread: usize| -> Status {
        let y = task as usize;
        let [row0, row1, row2] = image.plane_rows_mut(y);

        let mut x = 0;
        while x < xsize {
            let in_r = row0[x];
            let in_g = row1[x];
            let in_b = row2[x];
            [row0[x], row1[x], row2[x]] = linear_rgb_to_xyb(in_r, in_g, in_b, premul_absorb);
            x += N;
        }
        Ok(())
    };
    // SAFETY: The rows write only the exclusively borrowed image and read the
    // constants through a shared borrow; neither holds a thread-affine value.
    unsafe { run_on_pool_exclusive_no_init(pool, 0, ysize as u32, image, process_row) }?;
    Ok(())
}

fn srgb_to_xyb(premul_absorb: &[f32], pool: Option<&ThreadPool>, image: &mut Image3F) -> Status {
    let xsize = image.xsize();
    let ysize = image.ysize();

    let process_row = |image: &mut Image3F, task: u32, _thread: usize| -> Status {
        let y = task as usize;
        let [row0, row1, row2] = image.plane_rows_mut(y);

        let mut x = 0;
        while x < xsize {
            let in_r = linear_from_srgb(row0[x]);
            let in_g = linear_from_srgb(row1[x]);
            let in_b = linear_from_srgb(row2[x]);
            [row0[x], row1[x], row2[x]] = linear_rgb_to_xyb(in_r, in_g, in_b, premul_absorb);
            x += N;
        }
        Ok(())
    };
    // SAFETY: As in linear_srgb_to_xyb.
    unsafe { run_on_pool_exclusive_no_init(pool, 0, ysize as u32, image, process_row) }?;
    Ok(())
}

fn srgb_to_xyb_and_linear(
    premul_absorb: &[f32],
    pool: Option<&ThreadPool>,
    image: &mut Image3F,
    linear: &mut Image3F,
) -> Status {
    let xsize = image.xsize();
    let ysize = image.ysize();

    let mut images = (image, linear);
    let process_row =
        |(image, linear): &mut (&mut Image3F, &mut Image3F), task: u32, _thread: usize| -> Status {
            let y = task as usize;
            let [row_image0, row_image1, row_image2] = image.plane_rows_mut(y);
            let [row_linear0, row_linear1, row_linear2] = linear.plane_rows_mut(y);

            let mut x = 0;
            while x < xsize {
                let in_r = linear_from_srgb(row_image0[x]);
                let in_g = linear_from_srgb(row_image1[x]);
                let in_b = linear_from_srgb(row_image2[x]);

                row_linear0[x] = in_r;
                row_linear1[x] = in_g;
                row_linear2[x] = in_b;

                [row_image0[x], row_image1[x], row_image2[x]] =
                    linear_rgb_to_xyb(in_r, in_g, in_b, premul_absorb);
                x += N;
            }
            Ok(())
        };
    // SAFETY: The rows write only the two exclusively borrowed, distinct
    // images and read the constants through a shared borrow; none of them
    // holds a thread-affine value.
    unsafe { run_on_pool_exclusive_no_init(pool, 0, ysize as u32, &mut images, process_row) }?;
    Ok(())
}

/// Original ComputePremulAbsorb: nine scaled absorbance entries followed by
/// three negated bias cube roots, each broadcast to one lane.
pub fn compute_premul_absorb(intensity_target: f32, premul_absorb: &mut [f32]) {
    let mul = intensity_target / 255.0;
    for j in 0..3 {
        for i in 0..3 {
            let absorb = OPSIN_ABSORBANCE_MATRIX[j][i] * mul;
            premul_absorb[(j * 3 + i) * N] = absorb;
        }
    }
    for i in 0..3 {
        let neg_bias_cbrt = -OPSIN_ABSORBANCE_BIAS[i].cbrt();
        premul_absorb[(9 + i) * N] = neg_bias_cbrt;
    }
}

fn same_size(a: (usize, usize), b: (usize, usize)) -> bool {
    a == b
}

/// Original ToXYB for an admitted input encoding. This is different from
/// Butteraugli's OpsinDynamicsImage() in the sense that it does not contain a
/// sensitivity multiplier based on the blurred image.
pub fn to_xyb(
    c_current: XybInputEncoding,
    intensity_target: f32,
    black: Option<&ImageF>,
    pool: Option<&ThreadPool>,
    image: &mut Image3F,
    linear: Option<&mut Image3F>,
) -> Status {
    // HWY_DYNAMIC_DISPATCH(ToXYB): the encoder's own selection, off by
    // default (port ADR 0004, "Encoder path").
    if let Some(target) = super::hwy::encoder_target() {
        return super::enc_xyb_simd::to_xyb_simd(
            target,
            c_current,
            intensity_target,
            black,
            pool,
            image,
            linear,
        );
    }
    let image_size = (image.xsize(), image.ysize());
    if let Some(black) = black
        && !same_size(image_size, (black.xsize(), black.ysize()))
    {
        return Err(StatusCode::GenericError);
    }
    if let Some(linear) = &linear
        && !same_size(image_size, (linear.xsize(), linear.ysize()))
    {
        return Err(StatusCode::GenericError);
    }

    let memory_manager = image.memory_manager();
    if !memory_manager.is_some() {
        return Err(StatusCode::GenericError);
    }

    // Pre-broadcasted constants
    let mut mem = AlignedMemory::create(memory_manager, N * 12 * size_of::<f32>(), 0)?;
    let premul_absorb = mem.f32s_mut();
    compute_premul_absorb(intensity_target, premul_absorb);

    // Linear sRGB inputs are rare but can be useful for the fastest encoders, for
    // which undoing the sRGB transfer function would be a large part of the cost.
    match c_current {
        XybInputEncoding::LinearSrgb => {
            // This only happens if kitten or slower, moving ImageBundle might be
            // possible but the encoder is much slower than this copy.
            if let Some(linear) = linear {
                copy_image3_to(image, linear)?;
            }
            linear_srgb_to_xyb(premul_absorb, pool, image)?;
        }
        // Common case: already sRGB, can avoid the color transform
        XybInputEncoding::Srgb => {
            // Common case: can avoid allocating/copying
            if let Some(linear) = linear {
                // Slow encoder also wants linear sRGB.
                srgb_to_xyb_and_linear(premul_absorb, pool, image, linear)?;
            } else {
                srgb_to_xyb(premul_absorb, pool, image)?;
            }
        }
    }
    Ok(())
}
