// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! GaborishInverse from lib/jxl/enc_gaborish.{h,cc}.
//!
//! The kernel literals are double literals narrowed to float, as in the
//! source. Their sum and the product with `mul[i] * 4` are binary32; only the
//! final `1.0 + ...`, the clamp and the reciprocal are double. The binary32
//! product is converted before the double addition, so the pinned AArch64
//! object contains no fused operation; neither does baseline x86-64.
use super::base::data_parallel::ThreadPool;
use super::base::rect::Rect;
use super::base::status::Status;
use super::convolve::WeightsSymmetric5;
use super::convolve_symmetric5::symmetric5;
use super::image::{Image3F, ImageF};
use super::image_ops::copy_image_to;

/// Original GaborishInverse: three Symmetric5 passes over `rect` extended by
/// three pixels, through one temporary plane, then the source's plane swaps.
pub fn gaborish_inverse(
    in_out: &mut Image3F,
    rect: Rect,
    mul: &[f32; 3],
    pool: Option<&ThreadPool>,
) -> Status {
    let memory_manager = in_out.memory_manager();
    let mut weights = [WeightsSymmetric5::default(); 3];
    // Only an approximation. One or even two 3x3, and rank-1 (separable) 5x5
    // are insufficient. The numbers here have been obtained by butteraugli
    // based optimizing the whole system and the errors produced are likely
    // more favorable for good rate-distortion compromises rather than
    // just using mathematical optimization to find the inverse.
    const GABORISH: [f32; 5] = [
        -0.09495815671340026f64 as f32,
        -0.041031725066768575f64 as f32,
        0.013710004822696948f64 as f32,
        0.006510206083837737f64 as f32,
        -0.0014789063378272242f64 as f32,
    ];
    for i in 0..3 {
        let mut sum = 1.0
            + f64::from(
                mul[i]
                    * 4.0
                    * (GABORISH[0] + GABORISH[1] + GABORISH[2] + GABORISH[4] + 2.0 * GABORISH[3]),
            );
        if sum < 1e-5 {
            sum = 1e-5;
        }
        let normalize = (1.0 / sum) as f32;
        let normalize_mul = mul[i] * normalize;
        weights[i] = WeightsSymmetric5 {
            c: [normalize; 4],
            r: [normalize_mul * GABORISH[0]; 4],
            R: [normalize_mul * GABORISH[2]; 4],
            d: [normalize_mul * GABORISH[1]; 4],
            D: [normalize_mul * GABORISH[4]; 4],
            L: [normalize_mul * GABORISH[3]; 4],
        };
    }
    // Reduce memory footprint by only allocating a single plane and swapping it
    // into the output Image3F. Better still would be tiling.
    // Note that we cannot *allocate* a plane, as doing so might cause Image3F to
    // have planes of different stride. Instead, we copy one plane in a temporary
    // image and reuse the existing planes of the in/out image.
    let mut temp = ImageF::create_in(
        in_out.plane(2).xsize(),
        in_out.plane(2).ysize(),
        0,
        memory_manager,
    )?;
    copy_image_to(in_out.plane(2), &mut temp)?;
    let xrect = rect.extend(3, Rect::new(0, 0, in_out.xsize(), in_out.ysize()));
    let [plane0, plane1, plane2] = in_out.planes_mut();
    symmetric5(plane0, xrect, &weights[0], pool, plane2, xrect)?;
    symmetric5(plane1, xrect, &weights[1], pool, plane0, xrect)?;
    symmetric5(&temp, xrect, &weights[2], pool, plane1, xrect)?;
    // Now planes are 1, 2, 0.
    plane0.swap(plane1);
    // 2 1 0
    plane0.swap(plane2);
    Ok(())
}
