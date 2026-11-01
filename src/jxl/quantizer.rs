// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Scalar state, parameter fields/decoding, scaling, and matrix getters from
//! lib/jxl/quantizer.{h,cc}. Quantization fields, scale selection, and diagnostic
//! dumps remain unported. The source's enclosing matrix pointer
//! is reified as a same-owner borrow at each operation; see ADR 0003.

use super::ac_strategy::AcStrategyType;
use super::base::status::Status;
use super::dec_bit_reader::BitReader;
use super::field_encodings::{Fields, U32Enc, bits_offset, val};
use super::fields::{Visitor, bundle};
use super::quant_weights::DequantMatrices;
pub const GLOBAL_SCALE_DENOM: i32 = 1 << 16;
pub const GLOBAL_SCALE_NUMERATOR: i32 = 4096;
pub const ZERO_BIAS_DEFAULT: [f32; 3] = [0.5; 3];
pub const DEFAULT_QUANT: i32 = 64;
pub const BIAS_NUMERATOR: f32 = 0.145;
pub const DEFAULT_QUANT_BIAS: [f32; 4] = [
    // Exact binary32 operands of the original decimal constant expressions.
    1.0f32 - f32::from_bits(0x3d5f_d8c1),
    1.0f32 - f32::from_bits(0x3d8f_78bc),
    1.0f32 - f32::from_bits(0x3d4c_88c0),
    BIAS_NUMERATOR,
];

#[derive(Clone, Debug)]
pub struct QuantizerParams {
    pub global_scale: u32,
    pub quant_dc: u32,
}
impl Default for QuantizerParams {
    fn default() -> Self {
        let mut value = Self {
            global_scale: 0,
            quant_dc: 0,
        };
        bundle::init(&mut value);
        value
    }
}
impl Fields for QuantizerParams {
    fn visit_fields(&mut self, visitor: &mut dyn Visitor) -> Status {
        visitor.u32(
            U32Enc::new(
                bits_offset(11, 1),
                bits_offset(11, 2049),
                bits_offset(12, 4097),
                bits_offset(16, 8193),
            ),
            1,
            &mut self.global_scale,
        )?;
        visitor.u32(
            U32Enc::new(
                val(16),
                bits_offset(5, 1),
                bits_offset(8, 1),
                bits_offset(16, 1),
            ),
            1,
            &mut self.quant_dc,
        )?;
        Ok(())
    }
}

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
    pub const QUANT_MAX: i32 = 256;

    /// Signaling NaNs are outside the reference's defined cast domain: its
    /// optimized max-number instruction can pass a NaN to the final int cast.
    pub fn clamp_val(value: f32) -> i32 {
        // Preserve std::max(1, std::min(value, kQuantMax))'s operand order,
        // including unordered comparisons: Rust float min/max differ for NaNs.
        let upper = if (Self::QUANT_MAX as f32) < value {
            Self::QUANT_MAX as f32
        } else {
            value
        };
        let clamped = if 1.0 < upper { upper } else { 1.0 };
        clamped as i32
    }

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
    pub fn get_params(&self) -> QuantizerParams {
        QuantizerParams {
            global_scale: self.global_scale as u32,
            quant_dc: self.quant_dc as u32,
        }
    }
    /// `dequant` is the same matrix owner supplied at construction.
    pub fn decode(&mut self, dequant: &DequantMatrices, reader: &mut BitReader<'_>) -> Status {
        let mut params = QuantizerParams::default();
        bundle::read(reader, &mut params)?;
        self.global_scale = params.global_scale as i32;
        self.quant_dc = params.quant_dc as i32;
        self.recompute_from_global_scale(dequant);
        Ok(())
    }
    /// `dequant` is the same matrix owner supplied at construction. The rounded
    /// binary32 product must be representable as a C++ `long`; nonfinite or
    /// out-of-long-range `std::lround` inputs are outside the source contract.
    pub fn scale_global_scale(&mut self, dequant: &DequantMatrices, scale: f32) -> f32 {
        let product = self.global_scale as f32 * scale;
        let rounded = product.round();
        assert!(
            rounded.is_finite()
                && rounded >= std::ffi::c_long::MIN as f32
                && rounded < -(std::ffi::c_long::MIN as f32)
        );
        // lround returns C long before the source narrows to int. The selected
        // Clang reference keeps the low bits of that signed integer conversion.
        let new_global_scale = rounded as std::ffi::c_long as i32;
        let scale_out = new_global_scale as f32 * 1.0 / self.global_scale as f32;
        self.global_scale = new_global_scale;
        self.recompute_from_global_scale(dequant);
        scale_out
    }
    /// `dequant` is the same matrix owner supplied at construction.
    pub fn dequant_matrix<'a>(
        &self,
        dequant: &'a DequantMatrices,
        quant_kind: AcStrategyType,
        c: usize,
    ) -> &'a [f32] {
        dequant.matrix(quant_kind, c)
    }
    /// `dequant` is the same matrix owner supplied at construction.
    pub fn inv_dequant_matrix<'a>(
        &self,
        dequant: &'a DequantMatrices,
        quant_kind: AcStrategyType,
        c: usize,
    ) -> &'a [f32] {
        dequant.inv_matrix(quant_kind, c)
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
