// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! The Highway kernels of dec_external_image.cc, `FloatToU32` and
//! `FloatToF16`, for the NEON and AVX2 targets (port ADR 0004), written once
//! against [`SimdF32`]. `dec_external_image.rs` keeps the scalar
//! (`HWY_SCALAR`) translation and calls these when the decoder selected a
//! SIMD target, as `HWY_DYNAMIC_DISPATCH(FloatToU32)` and
//! `HWY_DYNAMIC_DISPATCH(FloatToF16)` do.
//!
//! The source loops run whole vectors, `for (x = 0; x < num; x += Lanes(d))`,
//! so they read and write up to `RoundUpTo(num, Lanes(d))` samples; the
//! original relies on row padding for both (its MSan annotations unpoison the
//! input tail and poison the output tail again). These functions do the same
//! where the slices are long enough, which the decoder's padded plane rows
//! are for the selected target. Where a slice ends before the last whole
//! vector, the missing input lanes read zero and the missing output lanes
//! are dropped; every operation is lane-wise, so lanes up to `num` are
//! unaffected.
use super::hwy::{Kernel, SimdF32, Target, dispatch};

/// A whole vector of `src` at `x`, or the available lanes followed by zeros.
#[inline(always)]
fn load_lanes<D: SimdF32>(d: D, src: &[f32], x: usize) -> D::V {
    if src.len() >= x + D::LANES {
        d.load(&src[x..])
    } else {
        let mut lanes = [0.0f32; 16];
        let available = src.len().saturating_sub(x);
        lanes[..available].copy_from_slice(&src[x..x + available]);
        d.load(&lanes)
    }
}

/// `FloatToU32`: `Store(BitCast(du, NearestInt(Mul(Clamp(v, Zero, one),
/// scale))))` per vector.
struct FloatToU32<'a> {
    input: &'a [f32],
    output: &'a mut [u32],
    num: usize,
    mul: f32,
}
impl Kernel for FloatToU32<'_> {
    type Output = ();
    #[inline(always)]
    fn run<D: SimdF32>(self, d: D) {
        let one = d.set(1.0);
        let scale = d.set(self.mul);
        let mut x = 0;
        while x < self.num {
            let mut v = load_lanes(d, self.input, x);
            // Clamp turns NaN to 'min'.
            v = d.clamp(v, d.zero(), one);
            let i = d.nearest_int(d.mul(v, scale));
            let bits = d.bitcast_to_f32(i);
            if self.output.len() >= x + D::LANES {
                d.store_bits(bits, &mut self.output[x..]);
            } else {
                let mut lanes = [0u32; 16];
                d.store_bits(bits, &mut lanes);
                let available = self.output.len().saturating_sub(x);
                self.output[x..x + available].copy_from_slice(&lanes[..available]);
            }
            x += D::LANES;
        }
    }
}

/// The SIMD `FloatToU32(in, out, num, mul, bits_per_sample)`; the source
/// does not read `bits_per_sample`.
pub fn float_to_u32_simd(
    target: Target,
    input: &[f32],
    output: &mut [u32],
    num: usize,
    mul: f32,
    _bits_per_sample: usize,
) {
    // Each lane below `num` is loaded and stored, as in the scalar path.
    assert!(
        num <= input.len() && num <= output.len(),
        "source row-span precondition"
    );
    dispatch(
        target,
        FloatToU32 {
            input,
            output,
            num,
            mul,
        },
    );
}

/// `FloatToF16`: `Store(DemoteTo(du, Load(d, in + x)), du, out + x)` per
/// vector, the hardware binary16 conversion.
struct FloatToF16<'a> {
    input: &'a [f32],
    output: &'a mut [u16],
    num: usize,
}
impl Kernel for FloatToF16<'_> {
    type Output = ();
    #[inline(always)]
    fn run<D: SimdF32>(self, d: D) {
        let mut x = 0;
        while x < self.num {
            let v = load_lanes(d, self.input, x);
            if self.output.len() >= x + D::LANES {
                d.store_demote_to_f16(v, &mut self.output[x..]);
            } else {
                let mut lanes = [0u16; 16];
                d.store_demote_to_f16(v, &mut lanes);
                let available = self.output.len().saturating_sub(x);
                self.output[x..x + available].copy_from_slice(&lanes[..available]);
            }
            x += D::LANES;
        }
    }
}

/// The SIMD `FloatToF16(in, out, num)`.
pub fn float_to_f16_simd(target: Target, input: &[f32], output: &mut [u16], num: usize) {
    assert!(
        num <= input.len() && num <= output.len(),
        "source row-span precondition"
    );
    dispatch(target, FloatToF16 { input, output, num });
}
