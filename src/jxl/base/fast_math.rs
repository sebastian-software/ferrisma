// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Existing one-lane Highway specialization of FastLog2f from
//! `base/fast_math-inl.h` and its degree-two rational polynomial evaluation
//! from `base/rational_polynomial-inl.h`. Other functions/targets are unported.

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
