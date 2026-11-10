// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Frame filter parameter bundle from `lib/jxl/loop_filter.{h,cc}`.
//! Pixel kernels are translated separately in `render_pipeline`; the scalar
//! Gaborish stage is available there, while EPF kernels remain unported.

use super::base::float::mul_add_f32;
use super::base::status::{Status, StatusCode};
use super::field_encodings::Fields;
use super::fields::{Visitor, bundle};
pub const EPF_SHARP_ENTRIES: usize = 8;
#[derive(Clone, Debug)]
pub struct LoopFilter {
    pub all_default: bool,
    pub gab: bool,
    pub gab_custom: bool,
    pub gab_x_weight1: f32,
    pub gab_x_weight2: f32,
    pub gab_y_weight1: f32,
    pub gab_y_weight2: f32,
    pub gab_b_weight1: f32,
    pub gab_b_weight2: f32,
    pub epf_iters: u32,
    pub epf_sharp_custom: bool,
    pub epf_sharp_lut: [f32; EPF_SHARP_ENTRIES],
    pub epf_weight_custom: bool,
    pub epf_channel_scale: [f32; 3],
    pub epf_pass1_zeroflush: f32,
    pub epf_pass2_zeroflush: f32,
    pub epf_sigma_custom: bool,
    pub epf_quant_mul: f32,
    pub epf_pass0_sigma_scale: f32,
    pub epf_pass2_sigma_scale: f32,
    pub epf_border_sad_mul: f32,
    pub epf_sigma_for_modular: f32,
    pub extensions: u64,
    pub nonserialized_is_modular: bool,
}
impl Default for LoopFilter {
    fn default() -> Self {
        let mut value = Self {
            all_default: false,
            gab: false,
            gab_custom: false,
            gab_x_weight1: 0.0,
            gab_x_weight2: 0.0,
            gab_y_weight1: 0.0,
            gab_y_weight2: 0.0,
            gab_b_weight1: 0.0,
            gab_b_weight2: 0.0,
            epf_iters: 0,
            epf_sharp_custom: false,
            epf_sharp_lut: [0.0; 8],
            epf_weight_custom: false,
            epf_channel_scale: [0.0; 3],
            epf_pass1_zeroflush: 0.0,
            epf_pass2_zeroflush: 0.0,
            epf_sigma_custom: false,
            epf_quant_mul: 0.0,
            epf_pass0_sigma_scale: 0.0,
            epf_pass2_sigma_scale: 0.0,
            epf_border_sad_mul: 0.0,
            epf_sigma_for_modular: 0.0,
            extensions: 0,
            nonserialized_is_modular: false,
        };
        bundle::init(&mut value);
        value
    }
}
impl LoopFilter {
    pub fn padding(&self) -> usize {
        [0, 2, 3, 6][self.epf_iters as usize] + usize::from(self.gab)
    }
}
impl Fields for LoopFilter {
    fn visit_fields(&mut self, visitor: &mut dyn Visitor) -> Status {
        let mut all_default = self.all_default;
        let result = visitor.all_default(self, &mut all_default);
        self.all_default = all_default;
        if result {
            visitor.set_default(self);
            return Ok(());
        }
        visitor.bool(true, &mut self.gab)?;
        if visitor.conditional(self.gab) {
            visitor.bool(false, &mut self.gab_custom)?;
            if visitor.conditional(self.gab_custom) {
                // The source multiplies a float constant by the double 1.1,
                // then narrows the default passed to F16.
                const W1: f32 = (1.1f64 * f32::from_bits(0x3dd66cba) as f64) as f32;
                const W2: f32 = (1.1f64 * f32::from_bits(0x3d641147) as f64) as f32;
                visitor.f16(W1, &mut self.gab_x_weight1)?;
                visitor.f16(W2, &mut self.gab_x_weight2)?;
                if f64::from(mul_add_f32(self.gab_x_weight1 + self.gab_x_weight2, 4.0, 1.0).abs())
                    < 1e-8
                {
                    return Err(StatusCode::GenericError);
                }
                visitor.f16(W1, &mut self.gab_y_weight1)?;
                visitor.f16(W2, &mut self.gab_y_weight2)?;
                if f64::from(mul_add_f32(self.gab_y_weight1 + self.gab_y_weight2, 4.0, 1.0).abs())
                    < 1e-8
                {
                    return Err(StatusCode::GenericError);
                }
                visitor.f16(W1, &mut self.gab_b_weight1)?;
                visitor.f16(W2, &mut self.gab_b_weight2)?;
                if f64::from(mul_add_f32(self.gab_b_weight1 + self.gab_b_weight2, 4.0, 1.0).abs())
                    < 1e-8
                {
                    return Err(StatusCode::GenericError);
                }
            }
        }
        visitor.bits(2, 2, &mut self.epf_iters)?;
        if visitor.conditional(self.epf_iters > 0) {
            if visitor.conditional(!self.nonserialized_is_modular) {
                visitor.bool(false, &mut self.epf_sharp_custom)?;
                if visitor.conditional(self.epf_sharp_custom) {
                    for (i, value) in self.epf_sharp_lut.iter_mut().enumerate() {
                        visitor.f16(i as f32 / (EPF_SHARP_ENTRIES - 1) as f32, value)?;
                    }
                }
            }
            visitor.bool(false, &mut self.epf_weight_custom)?;
            if visitor.conditional(self.epf_weight_custom) {
                visitor.f16(40.0, &mut self.epf_channel_scale[0])?;
                visitor.f16(5.0, &mut self.epf_channel_scale[1])?;
                visitor.f16(3.5, &mut self.epf_channel_scale[2])?;
                visitor.f16(0.45, &mut self.epf_pass1_zeroflush)?;
                visitor.f16(0.6, &mut self.epf_pass2_zeroflush)?;
            }
            visitor.bool(false, &mut self.epf_sigma_custom)?;
            if visitor.conditional(self.epf_sigma_custom) {
                if visitor.conditional(!self.nonserialized_is_modular) {
                    visitor.f16(0.46, &mut self.epf_quant_mul)?;
                }
                visitor.f16(0.9, &mut self.epf_pass0_sigma_scale)?;
                visitor.f16(6.5, &mut self.epf_pass2_sigma_scale)?;
                visitor.f16(f32::from_bits(0x3f2aaaab), &mut self.epf_border_sad_mul)?;
            }
            if visitor.conditional(self.nonserialized_is_modular) {
                visitor.f16(1.0, &mut self.epf_sigma_for_modular)?;
                // The unsuffixed source threshold is double, so promote the
                // stored float instead of rounding the threshold down to f32.
                if f64::from(self.epf_sigma_for_modular) < 1e-8 {
                    return Err(StatusCode::GenericError);
                }
            }
        }
        visitor.begin_extensions(&mut self.extensions)?;
        visitor.end_extensions()
    }
}
