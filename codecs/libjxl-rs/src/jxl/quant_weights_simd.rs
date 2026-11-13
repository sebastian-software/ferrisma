// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! The vector parts of ComputeQuantTable in lib/jxl/quant_weights.cc
//! (GetQuantWeights with InterpolateVec, Interpolate's one-lane FastPowf)
//! and FastLog2f/FastPow2f/FastPowf of base/fast_math-inl.h with
//! EvalRationalPolynomial of base/rational_polynomial-inl.h, for the NEON and
//! AVX2 targets (port ADR 0004), written against [`SimdF32`]. Compare them
//! only with the original's SIMD build at the same target.
//!
//! The original uses `HWY_CAPPED(float, 4)` (GetQuantWeights) and
//! `HWY_CAPPED(float, 1)` (Interpolate's FastPowf). Every operation in these
//! functions is lane-wise, so each weight is computed here in its own
//! broadcast vector with the target's operations and read from lane 0; that
//! is the value the original's lane holds. The remaining scalar statements
//! of quant_weights.cc contain no contractible product and sum; they stay
//! in [`super::quant_weights`].
use super::hwy::{Kernel, SimdF32, Target, dispatch};

/// EvalRationalPolynomial of FastLog2f's 2,2 polynomial (Horner, fused).
#[inline(always)]
fn fast_log2f<D: SimdF32>(d: D, x: D::V) -> D::V {
    const P: [f32; 3] = [
        f32::from_bits(0xb5f85ab0),
        f32::from_bits(0x3fb6e02b),
        f32::from_bits(0x3f3e11c7),
    ];
    const Q: [f32; 3] = [
        f32::from_bits(0x3f7d8625),
        f32::from_bits(0x3f813ced),
        f32::from_bits(0x3e32458c),
    ];
    let x_bits = d.bitcast_to_i32(x);
    let exp_bits = d.sub_i32(x_bits, d.bitcast_to_i32(d.set(f32::from_bits(0x3f2aaaab))));
    let exp_shifted = d.shift_right_i32::<23>(exp_bits);
    let mantissa = d.bitcast_to_f32(d.sub_i32(x_bits, d.shift_left_i32::<23>(exp_shifted)));
    let exp_val = d.convert_to_f32(exp_shifted);
    let xm = d.sub(mantissa, d.set(1.0));
    let mut yp = d.set(P[2]);
    let mut yq = d.set(Q[2]);
    yp = d.mul_add(yp, xm, d.set(P[1]));
    yq = d.mul_add(yq, xm, d.set(Q[1]));
    yp = d.mul_add(yp, xm, d.set(P[0]));
    yq = d.mul_add(yq, xm, d.set(Q[0]));
    d.add(d.div(yp, yq), exp_val)
}

#[inline(always)]
fn fast_pow2f<D: SimdF32>(d: D, x: D::V) -> D::V {
    let floorx = d.floor(x);
    let exp = d.bitcast_to_f32(d.shift_left_i32::<23>(d.add_i32(
        d.convert_to_i32(floorx),
        d.bitcast_to_i32(d.set(f32::from_bits(127))),
    )));
    let frac = d.sub(x, floorx);
    let mut num = d.add(frac, d.set(f32::from_bits(0x4122cc6b)));
    num = d.mul_add(num, frac, d.set(f32::from_bits(0x424379a1)));
    num = d.mul_add(num, frac, d.set(f32::from_bits(0x42c519f0)));
    num = d.mul(num, exp);
    let mut den = d.mul_add(
        frac,
        d.set(f32::from_bits(0x3e5749ee)),
        d.set(f32::from_bits(0xbcb621be)),
    );
    den = d.mul_add(den, frac, d.set(f32::from_bits(0xc19b8831)));
    den = d.mul_add(den, frac, d.set(f32::from_bits(0x42c519f1)));
    d.div(num, den)
}

#[inline(always)]
fn fast_powf<D: SimdF32>(d: D, base: D::V, exponent: D::V) -> D::V {
    fast_pow2f(d, d.mul(fast_log2f(d, base), exponent))
}

/// The one-lane `FastPowf(float b, float e)` of fast_math-inl.h.
#[doc(hidden)]
pub struct Powf {
    pub base: f32,
    pub exponent: f32,
}
impl Kernel for Powf {
    type Output = f32;
    #[inline(always)]
    fn run<D: SimdF32>(self, d: D) -> f32 {
        d.get_lane(fast_powf(d, d.set(self.base), d.set(self.exponent)))
    }
}

/// `FastPowf(b, e)` as called from Interpolate inside `HWY_NAMESPACE`.
pub fn fast_powf_simd(target: Target, base: f32, exponent: f32) -> f32 {
    dispatch(target, Powf { base, exponent })
}

/// GetQuantWeights' inner loop for one channel: `bands` are the channel's
/// distance bands (`num_bands` of them), `out` the ROWS x COLS weights.
#[doc(hidden)]
pub struct Weights<'a> {
    pub rows: usize,
    pub cols: usize,
    pub bands: &'a [f32],
    pub num_bands: usize,
    pub out: &'a mut [f32],
}
impl Kernel for Weights<'_> {
    type Output = ();
    #[inline(always)]
    fn run<D: SimdF32>(self, d: D) {
        let scale = (self.num_bands - 1) as f32 / (f32::from_bits(0x3fb504f3) + 1e-6f32);
        let rcpcol = scale / (self.cols - 1) as f32;
        let rcprow = scale / (self.rows - 1) as f32;
        for y in 0..self.rows {
            let dy = y as f32 * rcprow;
            let dy2 = dy * dy;
            for x in 0..self.cols {
                // Lane x % 4 of the DF4 vector at column x - x % 4.
                let dx = d.mul(
                    d.add(d.set((x - x % 4) as f32), d.set((x % 4) as f32)),
                    d.set(rcpcol),
                );
                let scaled_distance = d.sqrt(d.mul_add(dx, dx, d.set(dy2)));
                let weight = if self.num_bands == 1 {
                    d.set(self.bands[0])
                } else {
                    // InterpolateVec.
                    let idx = d.convert_to_i32(scaled_distance);
                    let frac = d.sub(scaled_distance, d.convert_to_f32(idx));
                    let mut lane = [0i32; 16];
                    d.store_i32(idx, &mut lane);
                    let i = lane[0];
                    assert!(
                        i >= 0 && (i as usize) + 1 < self.bands.len(),
                        "source gather indices must be in bounds"
                    );
                    let a = d.set(self.bands[i as usize]);
                    let b = d.set(self.bands[i as usize + 1]);
                    d.mul(a, fast_powf(d, d.div(b, a), frac))
                };
                self.out[y * self.cols + x] = d.get_lane(weight);
            }
        }
    }
}

/// GetQuantWeights' weights of one channel for one SIMD target.
pub fn quant_weights_channel_simd(
    target: Target,
    rows: usize,
    cols: usize,
    bands: &[f32],
    num_bands: usize,
    out: &mut [f32],
) {
    dispatch(
        target,
        Weights {
            rows,
            cols,
            bands,
            num_bands,
            out,
        },
    )
}
