// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! The Highway kernels of dec_modular.cc, `MultiplySum`, `RgbFromSingle` and
//! `SingleFromSingle`, for the NEON and AVX2 targets (port ADR 0004), written
//! once against [`SimdF32`]. `dec_modular.rs` keeps the scalar (`HWY_SCALAR`)
//! translation and calls these when the decoder selected a SIMD target, as
//! `HWY_DYNAMIC_DISPATCH` does in `ModularImageToDecodedRect`.
//!
//! `Rebind<pixel_type, HWY_FULL(float)>` has the float vector's lanes; the
//! loops run whole vectors, `for (x = 0; x < xsize; x += Lanes(di))`, so they
//! read and write up to `RoundUpTo(xsize, Lanes)` samples of the padded
//! channel and pipeline rows, as the original does. Where a slice ends
//! before the last whole vector (only callers that pass unpadded rows), the
//! missing input lanes read zero and the missing output lanes are dropped;
//! every operation is lane-wise, so lanes below `xsize` are unaffected.
use super::hwy::{Kernel, SimdF32, Target, dispatch};

/// A whole vector of `src` at `x`, or the available lanes followed by zeros.
#[inline(always)]
fn load_lanes<D: SimdF32>(d: D, src: &[i32], x: usize) -> D::I {
    if src.len() >= x + D::LANES {
        d.load_i32(&src[x..])
    } else {
        let mut lanes = [0i32; 16];
        let available = src.len().saturating_sub(x);
        lanes[..available].copy_from_slice(&src[x..x + available]);
        d.load_i32(&lanes)
    }
}

/// `Store(v, df, out + x)`, or its available lanes.
#[inline(always)]
fn store_lanes<D: SimdF32>(d: D, v: D::V, dst: &mut [f32], x: usize) {
    if dst.len() >= x + D::LANES {
        d.store(v, &mut dst[x..]);
    } else {
        let mut lanes = [0.0f32; 16];
        d.store(v, &mut lanes);
        let available = dst.len().saturating_sub(x);
        dst[x..x + available].copy_from_slice(&lanes[..available]);
    }
}

struct MultiplySum<'a> {
    xsize: usize,
    row_in: &'a [i32],
    row_in_y: &'a [i32],
    factor: f32,
    row_out: &'a mut [f32],
}
impl Kernel for MultiplySum<'_> {
    type Output = ();
    #[inline(always)]
    fn run<D: SimdF32>(self, d: D) {
        let factor_v = d.set(self.factor);
        let mut x = 0;
        while x < self.xsize {
            let input = d.add_i32(
                load_lanes(d, self.row_in, x),
                load_lanes(d, self.row_in_y, x),
            );
            let out = d.mul(d.convert_to_f32(input), factor_v);
            store_lanes(d, out, self.row_out, x);
            x += D::LANES;
        }
    }
}

/// The SIMD `MultiplySum(xsize, row_in, row_in_Y, factor, row_out)`.
pub fn multiply_sum_simd(
    target: Target,
    xsize: usize,
    row_in: &[i32],
    row_in_y: &[i32],
    factor: f32,
    row_out: &mut [f32],
) {
    assert!(row_in.len() >= xsize && row_in_y.len() >= xsize && row_out.len() >= xsize);
    dispatch(
        target,
        MultiplySum {
            xsize,
            row_in,
            row_in_y,
            factor,
            row_out,
        },
    );
}

struct RgbFromSingle<'a> {
    xsize: usize,
    row_in: &'a [i32],
    factor: f32,
    out_r: &'a mut [f32],
    out_g: &'a mut [f32],
    out_b: &'a mut [f32],
}
impl Kernel for RgbFromSingle<'_> {
    type Output = ();
    #[inline(always)]
    fn run<D: SimdF32>(self, d: D) {
        let factor_v = d.set(self.factor);
        let mut x = 0;
        while x < self.xsize {
            let input = load_lanes(d, self.row_in, x);
            let out = d.mul(d.convert_to_f32(input), factor_v);
            store_lanes(d, out, self.out_r, x);
            store_lanes(d, out, self.out_g, x);
            store_lanes(d, out, self.out_b, x);
            x += D::LANES;
        }
    }
}

/// The SIMD `RgbFromSingle(xsize, row_in, factor, out_r, out_g, out_b)`.
pub fn rgb_from_single_simd(
    target: Target,
    xsize: usize,
    row_in: &[i32],
    factor: f32,
    out_r: &mut [f32],
    out_g: &mut [f32],
    out_b: &mut [f32],
) {
    assert!(
        row_in.len() >= xsize
            && out_r.len() >= xsize
            && out_g.len() >= xsize
            && out_b.len() >= xsize
    );
    dispatch(
        target,
        RgbFromSingle {
            xsize,
            row_in,
            factor,
            out_r,
            out_g,
            out_b,
        },
    );
}

struct SingleFromSingle<'a> {
    xsize: usize,
    row_in: &'a [i32],
    factor: f32,
    row_out: &'a mut [f32],
}
impl Kernel for SingleFromSingle<'_> {
    type Output = ();
    #[inline(always)]
    fn run<D: SimdF32>(self, d: D) {
        let factor_v = d.set(self.factor);
        let mut x = 0;
        while x < self.xsize {
            let input = load_lanes(d, self.row_in, x);
            let out = d.mul(d.convert_to_f32(input), factor_v);
            store_lanes(d, out, self.row_out, x);
            x += D::LANES;
        }
    }
}

/// The SIMD `SingleFromSingle(xsize, row_in, factor, row_out)`.
pub fn single_from_single_simd(
    target: Target,
    xsize: usize,
    row_in: &[i32],
    factor: f32,
    row_out: &mut [f32],
) {
    assert!(row_in.len() >= xsize && row_out.len() >= xsize);
    dispatch(
        target,
        SingleFromSingle {
            xsize,
            row_in,
            factor,
            row_out,
        },
    );
}
