// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! Complete scalar FromLinearStage factory from stage_from_linear.cc,
//! including the original high-precision `FromLinearStage<OpRgb>` specialization
//! and all six transfer operations. Transfer helpers follow
//! cms/transfer_functions-inl.h; FastLinearToSRGB remains a separate helper.
use super::render_pipeline_stage::{
    RenderPipelineChannelMode as Mode, RenderPipelineStage, RowBuffers, RowInfo, Settings,
};
use crate::jxl::base::hwy_scalar::mul_add;
use crate::jxl::base::status::{Status, StatusCode};

// Retain the original binary32 literal spellings.
#[allow(clippy::excessive_precision)]
pub fn fast_linear_to_srgb(v: f32) -> f32 {
    let normalized = f32::from_bits((v.to_bits() | 0x3e80_0000) & 0x3eff_ffff);
    let d1 = mul_add(normalized, 0.059914046, -0.108894556);
    let d2 = mul_add(d1, normalized, 0.107963754);
    let power = mul_add(d2, normalized, 0.018092343);
    const HIGH: [u32; 16] = [
        0x0, 0xa, 0x19, 0x26, 0x32, 0x41, 0x4d, 0x5c, 0x68, 0x75, 0x83, 0x8f, 0xa0, 0xaa, 0xb9,
        0xc6,
    ];
    const LOW: [u32; 16] = [
        0x0, 0xb7, 0x4, 0xd, 0xcb, 0xe7, 0x41, 0x68, 0x51, 0xd1, 0xeb, 0xf2, 0x0, 0xb7, 0x4, 0xd,
    ];
    let exponent = (((v.to_bits() as i32 >> 23).wrapping_sub(118)) as u32 & 15) as usize;
    let multiplier = f32::from_bits((HIGH[exponent] << 18) | (LOW[exponent] << 10) | 0x4000_0000);
    // The source computes both vector alternatives before selecting one.
    let linear = v * 12.92;
    let nonlinear = mul_add(power, multiplier, -0.055);
    if v < 0.0031308 { linear } else { nonlinear }
}

pub struct SrgbFromLinearStage;
impl RenderPipelineStage for SrgbFromLinearStage {
    fn settings(&self) -> Settings {
        Settings::default()
    }
    fn get_channel_mode(&self, c: usize) -> Mode {
        if c < 3 { Mode::InPlace } else { Mode::Ignored }
    }
    fn get_name(&self) -> &str {
        "FromLinear"
    }
    fn process_row(
        &mut self,
        input: &RowInfo,
        _output: &RowInfo,
        buffers: &mut RowBuffers<'_>,
        left: usize,
        right: usize,
        width: usize,
        _x: usize,
        _y: usize,
        _thread: usize,
    ) -> Status {
        if left != 0 || right != 0 {
            return Err(StatusCode::GenericError);
        }
        let rows: [_; 3] = std::array::from_fn(|c| self.get_input_row(input, c, 0));
        // In place on three disjoint spans resolved once per row; samples are
        // converted in the original x-then-channel order.
        let mut spans = buffers.three_rows_mut(rows, width);
        for x in 0..width {
            for span in spans.iter_mut() {
                span[x] =
                    crate::jxl::cms::transfer_functions_scalar::srgb_encoded_from_display(span[x]);
            }
        }
        Ok(())
    }
}

pub fn get_srgb_from_linear_stage() -> Box<dyn RenderPipelineStage> {
    Box::new(SrgbFromLinearStage)
}

// Each source Op constructor copies only the state used by that specialization.
enum Op {
    Linear,
    Rgb,
    Pq(crate::jxl::cms::transfer_functions_scalar::TfPq),
    Hlg(crate::jxl::cms::tone_mapping_scalar::HlgOotf),
    Rec709,
    Gamma(f32),
}
impl Op {
    fn transform(&self, rgb: &mut [f32; 3]) {
        use crate::jxl::cms::transfer_functions_scalar::{Tf709, TfHlg, srgb_encoded_from_display};
        match self {
            Self::Linear => {}
            Self::Rgb => {
                for value in rgb {
                    *value = srgb_encoded_from_display(*value);
                }
            }
            Self::Pq(pq) => {
                for value in rgb {
                    *value = pq.encoded_from_display(*value);
                }
            }
            Self::Hlg(ootf) => {
                ootf.apply(rgb);
                for value in rgb {
                    *value = TfHlg::encoded_from_display(*value);
                }
            }
            Self::Rec709 => {
                for value in rgb {
                    *value = Tf709::encoded_from_display(*value);
                }
            }
            Self::Gamma(inverse_gamma) => {
                for value in rgb {
                    let below = *value <= 1e-5f32;
                    let power = crate::jxl::base::fast_math::fast_powf(*value, *inverse_gamma);
                    *value = if below { 0.0 } else { power };
                }
            }
        }
    }
}

/// Complete scalar FromLinearStage. Operation state is copied by the factory.
pub struct FromLinearStage {
    op: Op,
}
impl RenderPipelineStage for FromLinearStage {
    fn settings(&self) -> Settings {
        Settings::default()
    }
    fn get_channel_mode(&self, c: usize) -> Mode {
        if c < 3 { Mode::InPlace } else { Mode::Ignored }
    }
    fn get_name(&self) -> &str {
        "FromLinear"
    }
    fn process_row(
        &mut self,
        input: &RowInfo,
        _output: &RowInfo,
        buffers: &mut RowBuffers<'_>,
        left: usize,
        right: usize,
        width: usize,
        _x: usize,
        _y: usize,
        _thread: usize,
    ) -> Status {
        if left != 0 || right != 0 {
            return Err(StatusCode::GenericError);
        }
        let rows: [_; 3] = std::array::from_fn(|c| self.get_input_row(input, c, 0));
        // In place on three disjoint spans resolved once per row.
        let [r0, r1, r2] = buffers.three_rows_mut(rows, width);
        for x in 0..width {
            let mut rgb = [r0[x], r1[x], r2[x]];
            self.op.transform(&mut rgb);
            [r0[x], r1[x], r2[x]] = rgb;
        }
        Ok(())
    }
}

/// Source invalid encodings are programmer errors: debug abort, release null.
pub fn get_from_linear_stage(
    info: &crate::jxl::dec_xyb::OutputEncodingInfo,
) -> Option<Box<dyn RenderPipelineStage>> {
    use crate::jxl::cms::tone_mapping_scalar::HlgOotf;
    use crate::jxl::cms::transfer_functions_scalar::TfPq;
    // HWY_DYNAMIC_DISPATCH(GetFromLinearStage) (port ADR 0004).
    if let Some(target) = crate::jxl::hwy::decoder_target() {
        return super::stage_from_linear_simd::get_from_linear_stage_simd(info, target);
    }
    let tf = info.color_encoding.tf();
    let op = if tf.is_linear() {
        Op::Linear
    } else if tf.is_srgb() {
        Op::Rgb
    } else if tf.is_pq() {
        Op::Pq(TfPq::new(info.orig_intensity_target))
    } else if tf.is_hlg() {
        Op::Hlg(HlgOotf::to_scene_light(
            info.desired_intensity_target,
            info.luminances,
        ))
    } else if tf.is709() {
        Op::Rec709
    } else if tf.have_gamma || tf.is_dci() {
        Op::Gamma(info.inverse_gamma)
    } else {
        debug_assert!(false, "Invalid target encoding");
        return None;
    };
    Some(Box::new(FromLinearStage { op }))
}
