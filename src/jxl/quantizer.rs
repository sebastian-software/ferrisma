// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Constructors, RecomputeFromGlobalScale, DC steps, and ClearDCMul from
//! lib/jxl/quantizer.{h,cc}. Scale selection, quantization fields, parsing, and
//! other Quantizer APIs remain unported. The source's enclosing matrix pointer
//! is reified as a same-owner borrow at each operation; see ADR 0003.

use super::quant_weights::DequantMatrices;
pub const GLOBAL_SCALE_DENOM: i32 = 1 << 16;
pub const GLOBAL_SCALE_NUMERATOR: i32 = 4096;
pub const ZERO_BIAS_DEFAULT: [f32; 3] = [0.5; 3];
pub const DEFAULT_QUANT: i32 = 64;

pub struct Quantizer {
    mul_dc: [f32; 4],
    inv_mul_dc: [f32; 4],
    global_scale: i32,
    quant_dc: i32,
    inv_global_scale: f32,
    global_scale_float: f32,
    inv_quant_dc: f32,
    zero_bias: [f32; 3],
}
impl Quantizer {
    pub fn new(dequant: &DequantMatrices) -> Self {
        Self::with_values(dequant, DEFAULT_QUANT, GLOBAL_SCALE_DENOM / DEFAULT_QUANT)
    }
    pub fn with_values(dequant: &DequantMatrices, quant_dc: i32, global_scale: i32) -> Self {
        // The fourth multiplier elements are source-indeterminate until ClearDCMul.
        let mut value = Self {
            mul_dc: [0.0; 4],
            inv_mul_dc: [0.0; 4],
            global_scale,
            quant_dc,
            inv_global_scale: 0.0,
            global_scale_float: 0.0,
            inv_quant_dc: 0.0,
            zero_bias: [0.0; 3],
        };
        value.recompute_from_global_scale(dequant);
        value.inv_quant_dc = value.inv_global_scale / value.quant_dc as f32;
        value.zero_bias.copy_from_slice(&ZERO_BIAS_DEFAULT);
        value
    }
    /// dequant is the same matrix owner supplied at construction.
    // Preserve the reference's recomputation boundary. Specializing the default
    // constructor across this call removes *1 arithmetic and retains signaling
    // NaNs where the original recomputation quiets them. See ADR 0002.
    #[inline(never)]
    pub fn recompute_from_global_scale(&mut self, dequant: &DequantMatrices) {
        self.global_scale_float =
            (self.global_scale as f64 * (1.0f64 / f64::from(GLOBAL_SCALE_DENOM))) as f32;
        self.inv_global_scale =
            (1.0f64 * f64::from(GLOBAL_SCALE_DENOM) / self.global_scale as f64) as f32;
        self.inv_quant_dc = self.inv_global_scale / self.quant_dc as f32;
        for c in 0..3 {
            self.mul_dc[c] = self.dc_step(dequant, c);
            self.inv_mul_dc[c] = self.inv_dc_step(dequant, c);
        }
    }
    pub fn scale(&self) -> f32 {
        self.global_scale_float
    }
    pub fn inv_global_scale(&self) -> f32 {
        self.inv_global_scale
    }
    pub fn inv_quant_dc(&self) -> f32 {
        self.inv_quant_dc
    }
    pub fn inv_quant_ac(&self, quant: i32) -> f32 {
        self.inv_global_scale / quant as f32
    }
    pub fn dc_step(&self, dequant: &DequantMatrices, c: usize) -> f32 {
        self.inv_quant_dc * dequant.dc_quant(c)
    }
    pub fn inv_dc_step(&self, dequant: &DequantMatrices, c: usize) -> f32 {
        dequant.inv_dc_quant(c) * (self.global_scale_float * self.quant_dc as f32)
    }
    pub fn mul_dc(&self) -> &[f32; 4] {
        &self.mul_dc
    }
    pub fn inv_mul_dc(&self) -> &[f32; 4] {
        &self.inv_mul_dc
    }
    pub fn zero_bias(&self) -> &[f32; 3] {
        &self.zero_bias
    }
    pub fn raw_dc(&self) -> i32 {
        self.quant_dc
    }
    pub fn global_scale(&self) -> i32 {
        self.global_scale
    }
    pub fn clear_dc_mul(&mut self) {
        self.mul_dc.fill(1.0);
        self.inv_mul_dc.fill(1.0);
    }
}
