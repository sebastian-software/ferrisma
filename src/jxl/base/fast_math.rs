// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Existing one-lane Highway specialization of FastLog2f from
//! `base/fast_math-inl.h` and its degree-two rational polynomial evaluation
//! from `base/rational_polynomial-inl.h`, plus FastPow2f/FastPowf. Other
//! functions/targets are unported.

use super::hwy_scalar::{floor, mul_add, to_i32};

pub fn fast_pow2f(x: f32) -> f32 {
    let floorx = floor(x);
    let exp = f32::from_bits(to_i32(floorx).wrapping_add(127).wrapping_shl(23) as u32);
    let frac = x - floorx;
    let mut num = frac + f32::from_bits(0x4122cc6b);
    num = mul_add(num, frac, f32::from_bits(0x424379a1));
    num = mul_add(num, frac, f32::from_bits(0x42c519f0));
    num *= exp;
    let mut den = mul_add(frac, f32::from_bits(0x3e5749ee), f32::from_bits(0xbcb621be));
    den = mul_add(den, frac, f32::from_bits(0xc19b8831));
    den = mul_add(den, frac, f32::from_bits(0x42c519f1));
    num / den
}
pub fn fast_powf(base: f32, exponent: f32) -> f32 {
    fast_pow2f(fast_log2f(base) * exponent)
}

/// The source leaves negative and NaN arguments undefined.
pub fn fast_log2f(x: f32) -> f32 {
    // Exact f32 encodings of the source's p/q literals, in increasing degree.
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
    let x_bits = x.to_bits() as i32;
    let exp_bits = x_bits.wrapping_sub(0x3f2aaaab);
    let exp_shifted = exp_bits >> 23;
    let mantissa = f32::from_bits(x_bits.wrapping_sub(exp_shifted.wrapping_shl(23)) as u32);
    let exp_val = exp_shifted as f32;
    let x = mantissa - 1.0;
    let mut yp = P[2];
    let mut yq = Q[2];
    yp = yp * x + P[1];
    yq = yq * x + Q[1];
    yp = yp * x + P[0];
    yq = yq * x + Q[0];
    yp / yq + exp_val
}
