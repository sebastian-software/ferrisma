// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! Symmetric5 of lib/jxl/convolve_symmetric5.cc for the NEON and AVX2
//! targets (port ADR 0004, "Encoder path"), written once against
//! [`SimdF32`] with `HWY_FULL(float)` (four NEON, eight AVX2 lanes): the
//! interior of each row in whole vectors (LoadU around the center,
//! LoadDup128 of each weight array, separate Mul and Add), the first
//! `RoundUpTo(kRadius, Lanes)` pixels and the tail one pixel at a time with
//! Symmetric5Border. The border is scalar code inside `HWY_NAMESPACE`; the
//! pinned NEON and AVX2 (dispatch build) objects contain no fused operation
//! in this file, so it is the scalar translation's `symmetric5_border`.
//! The decoder never calls Symmetric5; the encoder's inverse Gaborish and
//! adaptive-quantization mask do, and dispatch here through
//! `hwy::encoder_target()`. Compare it only with the original's SIMD build
//! at the same target; the scalar (one-lane) translation stays in
//! `convolve_symmetric5`.
use super::base::data_parallel::{ThreadPool, run_on_pool_exclusive_no_init};
use super::base::rect::Rect;
use super::base::status::{Status, StatusCode};
use super::convolve::WeightsSymmetric5;
use super::convolve_symmetric5::symmetric5_border;
use super::hwy::{Kernel, SimdF32, Target, dispatch};
use super::image::ImageF;
use super::image_ops::{Wrap, WrapMirror, WrapUnchanged};

/// WeightedSum of one kernel row: `row` is the input row, `ix` the center.
/// The five loads read one bounds-checked window of the row.
#[inline(always)]
fn weighted_sum<D: SimdF32>(d: D, row: &[f32], ix: usize, wx0: D::V, wx1: D::V, wx2: D::V) -> D::V {
    // `center` is row + ix; the window covers center - 2 .. center + Lanes + 2.
    let window = &row[ix - 2..ix + D::LANES + 2];
    let in_m2 = d.load(window);
    let in_p2 = d.load(&window[4..]);
    let in_m1 = d.load(&window[1..]);
    let in_p1 = d.load(&window[3..]);
    let in_00 = d.load(&window[2..]);
    let sum_2 = d.mul(wx2, d.add(in_m2, in_p2));
    let sum_1 = d.mul(wx1, d.add(in_m1, in_p1));
    let sum_0 = d.mul(wx0, in_00);
    d.add(sum_2, d.add(sum_1, sum_0))
}

/// The LoadDup128 weight vectors of Symmetric5Interior.
#[derive(Clone, Copy)]
struct Weights<V> {
    w0: V,
    w1: V,
    w2: V,
    w4: V,
    w5: V,
    w8: V,
}

/// Symmetric5Interior: one vector's worth of pixels. `rows` are the input
/// rows `iy - 2 ..= iy + 2` after WrapY, which the original looks up per
/// vector (`in.ConstRow(WrapY()(iy + k, ysize))`) from loop-invariant values;
/// they are looked up once per output row here. Same loads and operations in
/// the same order.
#[inline(always)]
fn symmetric5_interior<D: SimdF32>(
    d: D,
    rows: &[&[f32]; 5],
    ix: usize,
    w: Weights<D::V>,
    out: &mut [f32],
) {
    // Unrolled loop over all 5 rows of the kernel.
    let mut sum0 = weighted_sum(d, rows[2], ix, w.w0, w.w1, w.w2);

    sum0 = d.add(sum0, weighted_sum(d, rows[0], ix, w.w2, w.w5, w.w8));
    let mut sum1 = weighted_sum(d, rows[4], ix, w.w2, w.w5, w.w8);

    sum0 = d.add(sum0, weighted_sum(d, rows[1], ix, w.w1, w.w4, w.w5));
    sum1 = d.add(sum1, weighted_sum(d, rows[3], ix, w.w1, w.w4, w.w5));

    d.store(d.add(sum0, sum1), out);
}

/// Symmetric5Row: `row_out` starts at the output rect's first pixel.
#[inline(always)]
pub fn symmetric5_row_d<D: SimdF32, WrapY: Wrap>(
    d: D,
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
    let n = D::LANES;
    let aligned_x = RADIUS.div_ceil(n) * n;
    while ix < aligned_x.min(xend) {
        row_out[rix] = symmetric5_border::<WrapY>(input, ix as i64, iy, weights);
        ix += 1;
        rix += 1;
    }
    if ix + n + RADIUS <= xend {
        let w = Weights {
            w0: d.load_dup128(&weights.c),
            w1: d.load_dup128(&weights.r),
            w2: d.load_dup128(&weights.R),
            w4: d.load_dup128(&weights.d),
            w5: d.load_dup128(&weights.L),
            w8: d.load_dup128(&weights.D),
        };
        let ysize = input.ysize() as i64;
        let row = |k: i64| input.row(WrapY::wrap(iy + k, ysize) as usize);
        let rows = [row(-2), row(-1), row(0), row(1), row(2)];
        while ix + n + RADIUS <= xend {
            symmetric5_interior(d, &rows, ix, w, &mut row_out[rix..rix + n]);
            ix += n;
            rix += n;
        }
    }
    while ix < xend {
        row_out[rix] = symmetric5_border::<WrapY>(input, ix as i64, iy, weights);
        ix += 1;
        rix += 1;
    }
}

struct Row<'a> {
    input: &'a ImageF,
    rect: Rect,
    iy: i64,
    weights: &'a WeightsSymmetric5,
    row_out: &'a mut [f32],
    mirror: bool,
}
impl Kernel for Row<'_> {
    type Output = ();
    #[inline(always)]
    fn run<D: SimdF32>(self, d: D) {
        if self.mirror {
            symmetric5_row_d::<D, WrapMirror>(
                d,
                self.input,
                self.rect,
                self.iy,
                self.weights,
                self.row_out,
            );
        } else {
            symmetric5_row_d::<D, WrapUnchanged>(
                d,
                self.input,
                self.rect,
                self.iy,
                self.weights,
                self.row_out,
            );
        }
    }
}

/// `Symmetric5` instantiated for one SIMD target; the contract of the
/// scalar [`super::convolve_symmetric5::symmetric5`]. Each row runs in the
/// target's instantiation (the original runs the whole function there; the
/// rows are independent).
pub fn symmetric5_simd(
    target: Target,
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

        dispatch(
            target,
            Row {
                input,
                rect: in_rect,
                iy,
                weights,
                row_out,
                mirror: iy < 2 || iy >= input.ysize() as i64 - 2,
            },
        );
        Ok(())
    };
    // SAFETY: The rows write only the exclusively borrowed output plane and
    // read the input plane and weights through shared borrows; none of them
    // holds a thread-affine value.
    unsafe { run_on_pool_exclusive_no_init(pool, 0, ysize as u32, out, process_row) }?;
    Ok(())
}
