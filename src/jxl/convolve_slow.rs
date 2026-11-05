// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! SlowSymmetric3 from lib/jxl/convolve_slow.cc, which enc_gaborish_test.cc
//! uses as the forward Gaborish kernel. SlowSeparable5 remains unported.
//!
//! Contraction: Clang's default `-ffp-contract=on` fuses the first product of
//! `row_in[ix] * wc + (row_in[xm1] + row_in[xp1]) * wlr` with the addition in
//! the pinned native AArch64 object (FMADD after the separate second product),
//! then adds that sum to `sum` separately. Baseline x86-64 has no FMA and keeps
//! both products and additions separate. `kernel_row` reproduces both.
use super::base::data_parallel::{ThreadPool, run_on_pool_exclusive_no_init};
use super::base::rect::Rect;
use super::base::status::Status;
use super::convolve::WeightsSymmetric3;
use super::image::ImageF;
use super::image_ops::{Wrap, WrapMirror, WrapUnchanged};

#[cfg(target_arch = "aarch64")]
fn kernel_row(center: f32, wc: f32, neighbors: f32, wlr: f32) -> f32 {
    center.mul_add(wc, neighbors * wlr)
}
#[cfg(not(target_arch = "aarch64"))]
fn kernel_row(center: f32, wc: f32, neighbors: f32, wlr: f32) -> f32 {
    center * wc + neighbors * wlr
}

fn slow_symmetric3_pixel<WrapX: Wrap, WrapY: Wrap>(
    input: &ImageF,
    ix: i64,
    iy: i64,
    xsize: i64,
    ysize: i64,
    weights: &WeightsSymmetric3,
) -> f32 {
    let mut sum = 0.0f32;

    // ix: image; kx: kernel
    for ky in -1i64..=1 {
        let y = WrapY::wrap(iy + ky, ysize);
        let row_in = input.row(y as usize);

        let wc = if ky == 0 { weights.c[0] } else { weights.r[0] };
        let wlr = if ky == 0 { weights.r[0] } else { weights.d[0] };

        let xm1 = WrapX::wrap(ix - 1, xsize);
        let xp1 = WrapX::wrap(ix + 1, xsize);
        sum += kernel_row(
            row_in[ix as usize],
            wc,
            row_in[xm1 as usize] + row_in[xp1 as usize],
            wlr,
        );
    }
    sum
}

fn slow_symmetric3_row<WrapY: Wrap>(
    input: &ImageF,
    iy: i64,
    xsize: i64,
    ysize: i64,
    weights: &WeightsSymmetric3,
    row_out: &mut [f32],
) {
    row_out[0] = slow_symmetric3_pixel::<WrapMirror, WrapY>(input, 0, iy, xsize, ysize, weights);
    for ix in 1..xsize - 1 {
        row_out[ix as usize] =
            slow_symmetric3_pixel::<WrapUnchanged, WrapY>(input, ix, iy, xsize, ysize, weights);
    }
    {
        let ix = xsize - 1;
        row_out[ix as usize] =
            slow_symmetric3_pixel::<WrapMirror, WrapY>(input, ix, iy, xsize, ysize, weights);
    }
}

/// Original SlowSymmetric3. As in the source, rows and columns are indexed
/// from the planes' origins and only the rectangle's size is used; the input
/// and output planes are separate owners.
pub fn slow_symmetric3(
    input: &ImageF,
    rect: Rect,
    weights: &WeightsSymmetric3,
    pool: Option<&ThreadPool>,
    out: &mut ImageF,
) -> Status {
    let xsize = rect.xsize() as i64;
    let ysize = rect.ysize() as i64;
    const RADIUS: i64 = 1;

    let process_row = |out: &mut ImageF, task: u32, _thread: usize| -> Status {
        let iy = i64::from(task);
        let out_row = out.row_mut(iy as usize);

        if iy < RADIUS || iy >= ysize - RADIUS {
            slow_symmetric3_row::<WrapMirror>(input, iy, xsize, ysize, weights, out_row);
        } else {
            slow_symmetric3_row::<WrapUnchanged>(input, iy, xsize, ysize, weights, out_row);
        }
        Ok(())
    };
    // SAFETY: The rows write only the exclusively borrowed output plane and
    // read the input plane and weights through shared borrows; none of them
    // holds a thread-affine value.
    unsafe { run_on_pool_exclusive_no_init(pool, 0, ysize as u32, out, process_row) }?;
    Ok(())
}
