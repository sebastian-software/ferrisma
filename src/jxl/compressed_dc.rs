// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Scalar DequantDC and AdaptiveDCSmoothing from lib/jxl/compressed_dc.cc.
//! One-lane Highway expressions retain their original order. The row ranges
//! run through RunOnPool: serially with a null pool, and serialized by
//! `run_on_pool_exclusive` under a runner. Storage swaps remain explicit.
use super::ac_context::BlockCtxMap;
use super::base::data_parallel::{ThreadPool, run_on_pool_exclusive_no_init};
use super::base::rect::Rect;
use super::base::status::Status;
use super::frame_header::YCbCrChromaSubsampling;
use super::image::{Image3, Plane};
use super::modular::modular_image::Image;

const W1: f32 = 0.203_451_4;
const W2: f32 = 0.033_482_92;
const W0: f32 = 1.0 - 4.0 * (W1 + W2);

// Copyright (c) The Highway Project Authors. All rights reserved.
// SPDX-License-Identifier: BSD-3-Clause
// Use of these derived scalar helpers is governed by Highway's offered BSD
// alternative in licenses/HIGHWAY-BSD-3-Clause.txt.
// Scalar Mul promotes to binary64 before narrowing; MulAdd then adds separately.
#[inline(never)]
fn add(a: f32, b: f32) -> f32 {
    a + b
}
#[inline(never)]
fn promote(a: f32) -> f64 {
    f64::from(a)
}
#[inline(never)]
fn multiply_and_demote(a: f64, b: f64) -> f32 {
    (a * b) as f32
}
fn mul(a: f32, b: f32) -> f32 {
    multiply_and_demote(promote(a), promote(b))
}
fn mul_add(a: f32, b: f32, c: f32) -> f32 {
    add(mul(a, b), c)
}
fn hwy_max(a: f32, b: f32) -> f32 {
    if a.is_nan() {
        b
    } else if b.is_nan() || a > b {
        a
    } else {
        b
    }
}
fn pixel_channel(
    factor: f32,
    top: &[f32],
    row: &[f32],
    bottom: &[f32],
    gap: &mut f32,
    x: usize,
) -> (f32, f32) {
    let mc = row[x];
    let corner = add(
        add(top[x - 1], top[x + 1]),
        add(bottom[x - 1], bottom[x + 1]),
    );
    let side = add(add(row[x - 1], row[x + 1]), add(top[x], bottom[x]));
    let sm = mul_add(corner, W2, mul_add(side, W1, mul(mc, W0)));
    *gap = hwy_max(*gap, ((mc - sm) / factor).abs());
    (mc, sm)
}

pub fn adaptive_dc_smoothing(
    dc_factors: &[f32],
    dc: &mut Image3<f32>,
    chroma: &YCbCrChromaSubsampling,
    pool: Option<&ThreadPool>,
) -> Status {
    let xsize = dc.xsize();
    let ysize = dc.ysize();
    if ysize <= 2 || xsize <= 2 {
        return Ok(());
    }
    if chroma.is_444() {
        let mut smoothed = Image3::create(xsize, ysize)?;
        for c in 0..3 {
            for y in [0, ysize - 1] {
                smoothed.plane_row_mut(c, y)[..xsize].copy_from_slice(&dc.plane_row(c, y)[..xsize]);
            }
        }
        let process_row = |smoothed: &mut Image3<f32>, y: u32, _thread: usize| -> Status {
            let y = y as usize;
            for x in [0, xsize - 1] {
                for c in 0..3 {
                    smoothed.plane_row_mut(c, y)[x] = dc.plane_row(c, y)[x];
                }
            }
            for x in 1..xsize - 1 {
                let mut gap = 0.5;
                let mut mc = [0.0; 3];
                let mut sm = [0.0; 3];
                for c in 0..3 {
                    (mc[c], sm[c]) = pixel_channel(
                        dc_factors[c],
                        dc.plane_row(c, y - 1),
                        dc.plane_row(c, y),
                        dc.plane_row(c, y + 1),
                        &mut gap,
                        x,
                    );
                }
                let factor = mul_add(-4.0, gap, 3.0);
                let factor = if factor.is_sign_negative() {
                    0.0
                } else {
                    factor
                };
                for c in 0..3 {
                    smoothed.plane_row_mut(c, y)[x] = mul_add(sm[c] - mc[c], factor, mc[c]);
                }
            }
            Ok(())
        };
        // SAFETY: The rows write only the local smoothed image and read the
        // DC image through a shared borrow; neither holds a thread-affine
        // value.
        unsafe {
            run_on_pool_exclusive_no_init(pool, 1, (ysize - 1) as u32, &mut smoothed, process_row)
        }?;
        dc.swap(&mut smoothed);
    } else {
        let mut smoothed = Plane::create(xsize, ysize)?;
        for (c, &dc_factor) in dc_factors.iter().enumerate().take(3) {
            let width = xsize >> chroma.h_shift(c);
            let height = ysize >> chroma.v_shift(c);
            if height <= 2 || width <= 2 {
                continue;
            }
            for y in [0, height - 1] {
                smoothed.row_mut(y)[..width].copy_from_slice(&dc.plane_row(c, y)[..width]);
            }
            let process_row = |smoothed: &mut Plane<f32>, y: u32, _thread: usize| -> Status {
                let y = y as usize;
                for x in [0, width - 1] {
                    smoothed.row_mut(y)[x] = dc.plane_row(c, y)[x];
                }
                for x in 1..width - 1 {
                    let mut gap = 0.5;
                    let (mc, sm) = pixel_channel(
                        dc_factor,
                        dc.plane_row(c, y - 1),
                        dc.plane_row(c, y),
                        dc.plane_row(c, y + 1),
                        &mut gap,
                        x,
                    );
                    let factor = mul_add(-4.0, gap, 3.0);
                    let factor = if factor.is_sign_negative() {
                        0.0
                    } else {
                        factor
                    };
                    smoothed.row_mut(y)[x] = mul_add(sm - mc, factor, mc);
                }
                Ok(())
            };
            // SAFETY: As above, for the single smoothed plane.
            unsafe {
                run_on_pool_exclusive_no_init(
                    pool,
                    1,
                    (height - 1) as u32,
                    &mut smoothed,
                    process_row,
                )
            }?;
            dc.plane_mut(c).swap(&mut smoothed);
        }
    }
    Ok(())
}

#[expect(
    clippy::too_many_arguments,
    reason = "Preserve the original DequantDC argument boundary."
)]
pub fn dequant_dc(
    r: Rect,
    dc: &mut Image3<f32>,
    quant_dc: &mut Plane<u8>,
    input: &Image,
    dc_factors: &[f32],
    multiplier: f32,
    cfl_factors: &[f32],
    chroma: &YCbCrChromaSubsampling,
    block_context: &BlockCtxMap,
) {
    if chroma.is_444() {
        let factors = [
            dc_factors[0] * multiplier,
            dc_factors[1] * multiplier,
            dc_factors[2] * multiplier,
        ];
        for y in 0..r.ysize() {
            let qx = input.channel[1].plane.row(y);
            let qy = input.channel[0].plane.row(y);
            let qb = input.channel[2].plane.row(y);
            for x in 0..r.xsize() {
                let in_x = qx[x] as f32 * factors[0];
                let in_y = qy[x] as f32 * factors[1];
                let in_b = qb[x] as f32 * factors[2];
                dc.plane_row_mut(1, r.y0() + y)[r.x0() + x] = in_y;
                dc.plane_row_mut(0, r.y0() + y)[r.x0() + x] = mul_add(in_y, cfl_factors[0], in_x);
                dc.plane_row_mut(2, r.y0() + y)[r.x0() + x] = mul_add(in_y, cfl_factors[2], in_b);
            }
        }
    } else {
        for c in [1, 0, 2] {
            let rect = Rect::new(
                r.x0() >> chroma.h_shift(c),
                r.y0() >> chroma.v_shift(c),
                r.xsize() >> chroma.h_shift(c),
                r.ysize() >> chroma.v_shift(c),
            );
            let factor = dc_factors[c] * multiplier;
            let ch = &input.channel[if c < 2 { c ^ 1 } else { c }];
            for y in 0..rect.ysize() {
                let quant = ch.plane.row(y);
                let out = dc.plane_row_mut(c, rect.y0() + y);
                for x in 0..rect.xsize() {
                    out[rect.x0() + x] = quant[x] as f32 * factor;
                }
            }
        }
    }
    if block_context.num_dc_ctxs <= 1 {
        for y in 0..r.ysize() {
            quant_dc.row_mut(r.y0() + y)[r.x0()..r.x1()].fill(0);
        }
    } else {
        for y in 0..r.ysize() {
            let qx = input.channel[1].plane.row(y >> chroma.v_shift(0));
            let qy = input.channel[0].plane.row(y >> chroma.v_shift(1));
            let qb = input.channel[2].plane.row(y >> chroma.v_shift(2));
            let out = &mut quant_dc.row_mut(r.y0() + y)[r.x0()..];
            for x in 0..r.xsize() {
                let mut bx = 0i32;
                let mut by = 0i32;
                let mut bb = 0i32;
                for &threshold in &block_context.dc_thresholds[0] {
                    if qx[x >> chroma.h_shift(0)] > threshold {
                        bx += 1;
                    }
                }
                for &threshold in &block_context.dc_thresholds[1] {
                    if qy[x >> chroma.h_shift(1)] > threshold {
                        by += 1;
                    }
                }
                for &threshold in &block_context.dc_thresholds[2] {
                    if qb[x >> chroma.h_shift(2)] > threshold {
                        bb += 1;
                    }
                }
                let mut bucket = bx;
                bucket =
                    (bucket as usize).wrapping_mul(block_context.dc_thresholds[2].len() + 1) as i32;
                bucket += bb;
                bucket =
                    (bucket as usize).wrapping_mul(block_context.dc_thresholds[1].len() + 1) as i32;
                bucket += by;
                out[x] = bucket as u8;
            }
        }
    }
}
