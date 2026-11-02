// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Scalar AdjustQuantBias from `lib/jxl/quantizer-inl.h`.

/// The scalar Highway specialization retains its integer conversion, bitwise
/// sign handling, reciprocal zero case, and channel-dependent one bias.
pub fn adjust_quant_bias(c: usize, quant_i: i32, biases: &[f32; 4]) -> f32 {
    let quant = quant_i as f32;
    let sign = quant.to_bits() & 0x8000_0000;
    let abs_quant = f32::from_bits(quant.to_bits() & !0x8000_0000);
    let is_01 = abs_quant < 1.125;
    let not_0 = abs_quant > 0.0;
    let one_bias = if not_0 {
        f32::from_bits(biases[c].to_bits() ^ sign)
    } else {
        0.0
    };
    let reciprocal = if quant == 0.0 { 0.0 } else { 1.0 / quant };
    let bias = quant - biases[3] * reciprocal;
    if is_01 { one_bias } else { bias }
}
