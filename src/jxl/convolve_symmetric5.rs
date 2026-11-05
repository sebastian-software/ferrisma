// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! The one-lane Highway Symmetric5 from lib/jxl/convolve_symmetric5.cc. With
//! one scalar lane, Symmetric5Row's aligned prefix ends at the kernel radius
//! and Symmetric5Interior handles one pixel. The interior keeps Highway's
//! scalar Mul (`scalar_mul`) and separate Add; the border keeps plain binary32
//! C++ arithmetic. Each original statement is a separate rounding: the
//! pinned AArch64 and x86-64 objects contain no fused operation here. Rows are
//! slices of the caller's planes; the source's row pointer offsets become
//! indices. SIMD targets are unported.
use super::base::data_parallel::{ThreadPool, run_on_pool_exclusive_no_init};
use super::base::rect::Rect;
use super::base::status::{Status, StatusCode};
use super::convolve::WeightsSymmetric5;
use super::dct::scalar_mul;
use super::image::ImageF;
use super::image_ops::{Wrap, WrapMirror, WrapUnchanged, mirror};

#[expect(
    clippy::too_many_arguments,
    reason = "Retain the source's argument list."
)]
fn weighted_sum_border<WrapY: Wrap>(
    input: &ImageF,
    ix: i64,
    iy: i64,
    xsize: usize,
    ysize: usize,
    wx0: f32,
    wx1: f32,
    wx2: f32,
) -> f32 {
    let row = input.row(WrapY::wrap(iy, ysize as i64) as usize);
    let xsize = xsize as i64;
    let in_m2 = row[mirror(ix - 2, xsize) as usize];
    let in_p2 = row[mirror(ix + 2, xsize) as usize];
    let in_m1 = row[mirror(ix - 1, xsize) as usize];
    let in_p1 = row[mirror(ix + 1, xsize) as usize];
    let in_00 = row[ix as usize];
    let sum_2 = wx2 * (in_m2 + in_p2);
    let sum_1 = wx1 * (in_m1 + in_p1);
    let sum_0 = wx0 * in_00;
    sum_2 + (sum_1 + sum_0)
}

fn weighted_sum<WrapY: Wrap>(
    input: &ImageF,
    ix: usize,
    iy: i64,
    ysize: usize,
    wx0: f32,
    wx1: f32,
    wx2: f32,
) -> f32 {
    let row = input.row(WrapY::wrap(iy, ysize as i64) as usize);
    // LoadU at center - 2 .. center + 2 of one lane each.
    let in_m2 = row[ix - 2];
    let in_p2 = row[ix + 2];
    let in_m1 = row[ix - 1];
    let in_p1 = row[ix + 1];
    let in_00 = row[ix];
    let sum_2 = scalar_mul(wx2, in_m2 + in_p2);
    let sum_1 = scalar_mul(wx1, in_m1 + in_p1);
    let sum_0 = scalar_mul(wx0, in_00);
    sum_2 + (sum_1 + sum_0)
}

fn symmetric5_border<WrapY: Wrap>(
    input: &ImageF,
    ix: i64,
    iy: i64,
    weights: &WeightsSymmetric5,
) -> f32 {
    let w0 = weights.c[0];
    let w1 = weights.r[0];
    let w2 = weights.R[0];
    let w4 = weights.d[0];
    let w5 = weights.L[0];
    let w8 = weights.D[0];

    let xsize = input.xsize();
    let ysize = input.ysize();
    // Unrolled loop over all 5 rows of the kernel.
    let mut sum0 = weighted_sum_border::<WrapY>(input, ix, iy, xsize, ysize, w0, w1, w2);

    sum0 += weighted_sum_border::<WrapY>(input, ix, iy - 2, xsize, ysize, w2, w5, w8);
    let mut sum1 = weighted_sum_border::<WrapY>(input, ix, iy + 2, xsize, ysize, w2, w5, w8);

    sum0 += weighted_sum_border::<WrapY>(input, ix, iy - 1, xsize, ysize, w1, w4, w5);
    sum1 += weighted_sum_border::<WrapY>(input, ix, iy + 1, xsize, ysize, w1, w4, w5);

    sum0 + sum1
}

fn symmetric5_interior<WrapY: Wrap>(
    input: &ImageF,
    ix: usize,
    rix: usize,
    iy: i64,
    weights: &WeightsSymmetric5,
    row_out: &mut [f32],
) {
    // LoadDup128 of one lane reads each weight's first copy.
    let w0 = weights.c[0];
    let w1 = weights.r[0];
    let w2 = weights.R[0];
    let w4 = weights.d[0];
    let w5 = weights.L[0];
    let w8 = weights.D[0];

    let ysize = input.ysize();
    // Unrolled loop over all 5 rows of the kernel.
    let mut sum0 = weighted_sum::<WrapY>(input, ix, iy, ysize, w0, w1, w2);

    sum0 += weighted_sum::<WrapY>(input, ix, iy - 2, ysize, w2, w5, w8);
    let mut sum1 = weighted_sum::<WrapY>(input, ix, iy + 2, ysize, w2, w5, w8);

    sum0 += weighted_sum::<WrapY>(input, ix, iy - 1, ysize, w1, w4, w5);
    sum1 += weighted_sum::<WrapY>(input, ix, iy + 1, ysize, w1, w4, w5);

    row_out[rix] = sum0 + sum1;
}

fn symmetric5_row<WrapY: Wrap>(
    input: &ImageF,
    rect: Rect,
    iy: i64,
    weights: &WeightsSymmetric5,
    row_out: &mut [f32],
) {
    const RADIUS: usize = 2;
    let xend = rect.x1();

    let mut rix = 0;
    let mut ix = rect.x0();
    // Lanes(d) of the scalar target.
    const N: usize = 1;
    let aligned_x = RADIUS.div_ceil(N) * N;
    while ix < aligned_x.min(xend) {
        row_out[rix] = symmetric5_border::<WrapY>(input, ix as i64, iy, weights);
        ix += 1;
        rix += 1;
    }
    while ix + N + RADIUS <= xend {
        symmetric5_interior::<WrapY>(input, ix, rix, iy, weights, row_out);
        ix += N;
        rix += N;
    }
    while ix < xend {
        row_out[rix] = symmetric5_border::<WrapY>(input, ix as i64, iy, weights);
        ix += 1;
        rix += 1;
    }
}

/// Original Symmetric5. The input and output planes are separate owners, as
/// the source's restrict output requires; a missing pool runs rows serially.
pub fn symmetric5(
    input: &ImageF,
    in_rect: Rect,
    weights: &WeightsSymmetric5,
    pool: Option<&ThreadPool>,
    out: &mut ImageF,
    out_rect: Rect,
) -> Status {
    if in_rect.xsize() != out_rect.xsize() {
        return Err(StatusCode::GenericError);
    }
    if in_rect.ysize() != out_rect.ysize() {
        return Err(StatusCode::GenericError);
    }
    let ysize = in_rect.ysize();
    let process_row = |out: &mut ImageF, task: u32, _thread: usize| -> Status {
        let riy = i64::from(task);
        let iy = in_rect.y0() as i64 + riy;
        let row_out = &mut out.row_mut(out_rect.y0() + riy as usize)[out_rect.x0()..];

        if iy < 2 || iy >= input.ysize() as i64 - 2 {
            symmetric5_row::<WrapMirror>(input, in_rect, iy, weights, row_out);
        } else {
            symmetric5_row::<WrapUnchanged>(input, in_rect, iy, weights, row_out);
        }
        Ok(())
    };
    // SAFETY: The rows write only the exclusively borrowed output plane and
    // read the input plane and weights through shared borrows; none of them
    // holds a thread-affine value.
    unsafe { run_on_pool_exclusive_no_init(pool, 0, ysize as u32, out, process_row) }?;
    Ok(())
}
