// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! The SIMD ToXYB, LinearRGBRowToXYB and ComputePremulAbsorb of
//! lib/jxl/enc_xyb.cc with OpsinAbsorbance, StoreXYB, LinearRGBToXYB,
//! LinearSRGBToXYB, SRGBToXYB, SRGBToXYBAndLinear and base/fast_math-inl.h's
//! CubeRootAndAdd, for the NEON and AVX2 targets (port ADR 0004, "Encoder
//! path"), written once against [`SimdF32`]. `LinearFromSRGB` is
//! `TF_SRGB().DisplayFromEncoded`, shared with the SIMD ToLinear stage
//! (`simd_transfer`). Compare it only with the original's SIMD build at the
//! same target. The scalar translation stays in `enc_xyb`; the encoder
//! reaches this module only through [`crate::jxl::hwy::encoder_target`].
//!
//! As in the original, every row is processed in whole vectors: the loads and
//! stores reach `RoundUpTo(xsize, Lanes)` samples, inside the row padding of
//! the image's planes (`bytes_per_row` rounds every row to 128 bytes). The
//! pre-broadcast constants hold `Lanes` copies of each of the 12 entries.
use super::base::data_parallel::{ThreadPool, run_on_pool_exclusive_no_init};
use super::base::status::{Status, StatusCode};
use super::cms::opsin_params::{OPSIN_ABSORBANCE_BIAS, OPSIN_ABSORBANCE_MATRIX};
use super::enc_xyb::XybInputEncoding;
use super::hwy::{Kernel, SimdF32, Target, dispatch};
use super::image::{Image3F, ImageF};
use super::image_ops::copy_image3_to;
use super::memory_manager_internal::AlignedMemory;
use super::render_pipeline::simd_transfer::srgb_display_from_encoded;

/// `CubeRootAndAdd(x, add)` of base/fast_math-inl.h: the cube root by the
/// exponent estimate and four Newton-Raphson steps, plus `add` (inputs are
/// assumed non-negative).
#[inline(always)]
pub fn cube_root_and_add<D: SimdF32>(d: D, x: D::V, add: D::V) -> D::V {
    let k_exp_bias = d.set_i32(0x5480_0000); // cast(1.) + cast(1.) / 3
    let k_exp_mul = d.set_i32(0x002A_AAAA); // shifted 1/3
    let k1_3 = d.set(1.0f32 / 3.0);
    let k4_3 = d.set(4.0f32 / 3.0);

    let xa = x; // assume inputs never negative
    let xa_3 = d.mul(k1_3, xa);

    // Multiply exponent by -1/3
    let m1 = d.bitcast_to_i32(xa);
    // Special case for 0: IfThenZeroElse keeps zero inputs from NaNs below.
    let m2 = d.if_then_zero_else_i32(
        d.eq_i32(m1, d.set_i32(0)),
        d.sub_i32(
            k_exp_bias,
            d.mul_i32(d.shift_right_i32::<23>(m1), k_exp_mul),
        ),
    );
    let mut r = d.bitcast_to_f32(m2);

    // Newton-Raphson iterations
    for _ in 0..3 {
        let r2 = d.mul(r, r);
        r = d.neg_mul_add(xa_3, d.mul(r2, r2), d.mul(k4_3, r));
    }
    // Final iteration
    let mut r2 = d.mul(r, r);
    r = d.mul_add(k1_3, d.neg_mul_add(xa, d.mul(r2, r2), r), r);
    r2 = d.mul(r, r);
    d.mul_add(r2, x, add)
}

/// `OpsinAbsorbance`: 4x3 matrix * 3x1 SIMD vectors.
#[inline(always)]
fn opsin_absorbance<D: SimdF32>(
    d: D,
    r: D::V,
    g: D::V,
    b: D::V,
    premul_absorb: &[f32],
) -> [D::V; 3] {
    let bias = &OPSIN_ABSORBANCE_BIAS;
    let n = D::LANES;
    let m = |i: usize| d.load(&premul_absorb[i * n..]);
    let (m0, m1, m2, m3, m4, m5, m6, m7, m8) =
        (m(0), m(1), m(2), m(3), m(4), m(5), m(6), m(7), m(8));
    let mixed0 = d.mul_add(m0, r, d.mul_add(m1, g, d.mul_add(m2, b, d.set(bias[0]))));
    let mixed1 = d.mul_add(m3, r, d.mul_add(m4, g, d.mul_add(m5, b, d.set(bias[1]))));
    let mixed2 = d.mul_add(m6, r, d.mul_add(m7, g, d.mul_add(m8, b, d.set(bias[2]))));
    [mixed0, mixed1, mixed2]
}

/// `StoreXYB`.
#[inline(always)]
fn store_xyb<D: SimdF32>(
    d: D,
    r: D::V,
    g: D::V,
    b: D::V,
    valx: &mut [f32],
    valy: &mut [f32],
    valz: &mut [f32],
) {
    let half = d.set(0.5);
    d.store(d.mul(half, d.sub(r, g)), valx);
    d.store(d.mul(half, d.add(r, g)), valy);
    d.store(b, valz);
}

/// `LinearRGBToXYB`: converts one RGB vector to XYB.
#[inline(always)]
#[allow(clippy::too_many_arguments, reason = "the source's signature")]
fn linear_rgb_to_xyb<D: SimdF32>(
    d: D,
    r: D::V,
    g: D::V,
    b: D::V,
    premul_absorb: &[f32],
    valx: &mut [f32],
    valy: &mut [f32],
    valz: &mut [f32],
) {
    let [mut mixed0, mut mixed1, mut mixed2] = opsin_absorbance(d, r, g, b, premul_absorb);

    // mixed* should be non-negative even for wide-gamut, so clamp to zero.
    mixed0 = d.zero_if_negative(mixed0);
    mixed1 = d.zero_if_negative(mixed1);
    mixed2 = d.zero_if_negative(mixed2);

    let n = D::LANES;
    mixed0 = cube_root_and_add(d, mixed0, d.load(&premul_absorb[9 * n..]));
    mixed1 = cube_root_and_add(d, mixed1, d.load(&premul_absorb[10 * n..]));
    mixed2 = cube_root_and_add(d, mixed2, d.load(&premul_absorb[11 * n..]));
    store_xyb(d, mixed0, mixed1, mixed2, valx, valy, valz);

    // For wide-gamut inputs, r/g/b and valx (but not y/z) are often negative.
}

/// Which input transfer the row loop applies before LinearRGBToXYB.
#[derive(Clone, Copy)]
enum RowKind {
    /// LinearRGBRowToXYB and LinearSRGBToXYB: the samples are linear.
    Linear,
    /// SRGBToXYB: LinearFromSRGB first.
    Srgb,
}

/// One row triple, converted in place over `xsize` samples in whole vectors.
struct XybRow<'a> {
    rows: [&'a mut [f32]; 3],
    premul_absorb: &'a [f32],
    xsize: usize,
    kind: RowKind,
}
impl Kernel for XybRow<'_> {
    type Output = ();
    #[inline(always)]
    fn run<D: SimdF32>(self, d: D) {
        let [row0, row1, row2] = self.rows;
        let mut x = 0;
        while x < self.xsize {
            let mut r = d.load(&row0[x..]);
            let mut g = d.load(&row1[x..]);
            let mut b = d.load(&row2[x..]);
            if let RowKind::Srgb = self.kind {
                r = srgb_display_from_encoded(d, r);
                g = srgb_display_from_encoded(d, g);
                b = srgb_display_from_encoded(d, b);
            }
            linear_rgb_to_xyb(
                d,
                r,
                g,
                b,
                self.premul_absorb,
                &mut row0[x..],
                &mut row1[x..],
                &mut row2[x..],
            );
            x += D::LANES;
        }
    }
}

/// SRGBToXYBAndLinear's row: the linear samples are stored too.
struct XybAndLinearRow<'a> {
    image: [&'a mut [f32]; 3],
    linear: [&'a mut [f32]; 3],
    premul_absorb: &'a [f32],
    xsize: usize,
}
impl Kernel for XybAndLinearRow<'_> {
    type Output = ();
    #[inline(always)]
    fn run<D: SimdF32>(self, d: D) {
        let [row_image0, row_image1, row_image2] = self.image;
        let [row_linear0, row_linear1, row_linear2] = self.linear;
        let mut x = 0;
        while x < self.xsize {
            let in_r = srgb_display_from_encoded(d, d.load(&row_image0[x..]));
            let in_g = srgb_display_from_encoded(d, d.load(&row_image1[x..]));
            let in_b = srgb_display_from_encoded(d, d.load(&row_image2[x..]));

            d.store(in_r, &mut row_linear0[x..]);
            d.store(in_g, &mut row_linear1[x..]);
            d.store(in_b, &mut row_linear2[x..]);

            linear_rgb_to_xyb(
                d,
                in_r,
                in_g,
                in_b,
                self.premul_absorb,
                &mut row_image0[x..],
                &mut row_image1[x..],
                &mut row_image2[x..],
            );
            x += D::LANES;
        }
    }
}

/// The row loop of LinearRGBRowToXYB (`srgb` false) or SRGBToXYB (`srgb`
/// true) for one descriptor, in place; `premul_absorb` holds `12 * D::LANES`
/// words ([`premul_absorb_for_lanes`]). For the 8-lane structure test.
pub fn xyb_row_d<D: SimdF32>(
    d: D,
    rows: [&mut [f32]; 3],
    premul_absorb: &[f32],
    xsize: usize,
    srgb: bool,
) {
    let kind = if srgb { RowKind::Srgb } else { RowKind::Linear };
    XybRow {
        rows,
        premul_absorb,
        xsize,
        kind,
    }
    .run(d);
}

/// Original `LinearRGBRowToXYB` for a SIMD target, in place. The rows must
/// hold `RoundUpTo(xsize, Lanes)` samples and `premul_absorb` the
/// `12 * Lanes` words of [`compute_premul_absorb_simd`].
pub fn linear_rgb_row_to_xyb_simd(
    t: Target,
    row0: &mut [f32],
    row1: &mut [f32],
    row2: &mut [f32],
    premul_absorb: &[f32],
    xsize: usize,
) {
    dispatch(
        t,
        XybRow {
            rows: [row0, row1, row2],
            premul_absorb,
            xsize,
            kind: RowKind::Linear,
        },
    );
}

/// Original `ComputePremulAbsorb` for a SIMD target: nine scaled absorbance
/// entries followed by three negated bias cube roots, each stored as one full
/// vector (`12 * Lanes` words).
pub fn compute_premul_absorb_simd(t: Target, intensity_target: f32, premul_absorb: &mut [f32]) {
    premul_absorb_for_lanes(t.lanes(), intensity_target, premul_absorb);
}

/// ComputePremulAbsorb's layout for `n` lanes.
pub fn premul_absorb_for_lanes(n: usize, intensity_target: f32, premul_absorb: &mut [f32]) {
    let mul = intensity_target / 255.0;
    for j in 0..3 {
        for i in 0..3 {
            let absorb = OPSIN_ABSORBANCE_MATRIX[j][i] * mul;
            premul_absorb[(j * 3 + i) * n..(j * 3 + i + 1) * n].fill(absorb);
        }
    }
    for i in 0..3 {
        let neg_bias_cbrt = -OPSIN_ABSORBANCE_BIAS[i].cbrt();
        premul_absorb[(9 + i) * n..(10 + i) * n].fill(neg_bias_cbrt);
    }
}

/// LinearSRGBToXYB and SRGBToXYB.
fn rows_to_xyb(
    t: Target,
    premul_absorb: &[f32],
    pool: Option<&ThreadPool>,
    image: &mut Image3F,
    kind: RowKind,
) -> Status {
    let xsize = image.xsize();
    let ysize = image.ysize();

    let process_row = |image: &mut Image3F, task: u32, _thread: usize| -> Status {
        let y = task as usize;
        let rows = image.plane_rows_mut(y);
        dispatch(
            t,
            XybRow {
                rows,
                premul_absorb,
                xsize,
                kind,
            },
        );
        Ok(())
    };
    // SAFETY: The rows write only the exclusively borrowed image and read the
    // constants through a shared borrow; neither holds a thread-affine value.
    unsafe { run_on_pool_exclusive_no_init(pool, 0, ysize as u32, image, process_row) }?;
    Ok(())
}

fn srgb_to_xyb_and_linear(
    t: Target,
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
            dispatch(
                t,
                XybAndLinearRow {
                    image: image.plane_rows_mut(y),
                    linear: linear.plane_rows_mut(y),
                    premul_absorb,
                    xsize,
                },
            );
            Ok(())
        };
    // SAFETY: The rows write only the two exclusively borrowed, distinct
    // images and read the constants through a shared borrow; none of them
    // holds a thread-affine value.
    unsafe { run_on_pool_exclusive_no_init(pool, 0, ysize as u32, &mut images, process_row) }?;
    Ok(())
}

/// Original `ToXYB` for an admitted input encoding on a SIMD target; the
/// checks, the constants' allocation and the branches are those of the
/// scalar `enc_xyb::to_xyb`.
pub fn to_xyb_simd(
    t: Target,
    c_current: XybInputEncoding,
    intensity_target: f32,
    black: Option<&ImageF>,
    pool: Option<&ThreadPool>,
    image: &mut Image3F,
    linear: Option<&mut Image3F>,
) -> Status {
    let image_size = (image.xsize(), image.ysize());
    if let Some(black) = black
        && image_size != (black.xsize(), black.ysize())
    {
        return Err(StatusCode::GenericError);
    }
    if let Some(linear) = &linear
        && image_size != (linear.xsize(), linear.ysize())
    {
        return Err(StatusCode::GenericError);
    }

    let memory_manager = image.memory_manager();
    if !memory_manager.is_some() {
        return Err(StatusCode::GenericError);
    }

    // Pre-broadcasted constants
    let mut mem = AlignedMemory::create(memory_manager, t.lanes() * 12 * size_of::<f32>(), 0)?;
    let premul_absorb = mem.f32s_mut();
    compute_premul_absorb_simd(t, intensity_target, premul_absorb);

    match c_current {
        XybInputEncoding::LinearSrgb => {
            if let Some(linear) = linear {
                copy_image3_to(image, linear)?;
            }
            rows_to_xyb(t, premul_absorb, pool, image, RowKind::Linear)?;
        }
        XybInputEncoding::Srgb => {
            if let Some(linear) = linear {
                srgb_to_xyb_and_linear(t, premul_absorb, pool, image, linear)?;
            } else {
                rows_to_xyb(t, premul_absorb, pool, image, RowKind::Srgb)?;
            }
        }
    }
    Ok(())
}
