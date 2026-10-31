// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Ported bundles and field APIs from `lib/jxl/image_metadata.{h,cc}`.
//! Strings retain arbitrary bytes. Preserve default-visitor side effects,
//! field order, unsigned offsets, and state changes before failed visits.
//! Diagnostic strings and ExtraFieldsDefault remain unported.

use super::base::matrix_ops::Matrix3x3;
use super::base::status::{Status, StatusCode};
use super::cms::opsin_params::{
    DEFAULT_INVERSE_OPSIN_ABSORBANCE_MATRIX, NEG_OPSIN_ABSORBANCE_BIAS_RGB,
};
use super::color_encoding_internal::ColorEncoding;
use super::dec_bit_reader::BitReader;
use super::field_encodings::{FieldEnum, Fields, U32Enc, bits, bits_offset, val};
use super::fields::{Visitor, bundle, visit_enum};
use super::frame_header::visit_name_string;
use super::headers::{AnimationHeader, PreviewHeader, SizeHeader};
use super::image_metadata_weights::{WEIGHTS2, WEIGHTS4, WEIGHTS8};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Orientation(pub u32);
impl Orientation {
    pub const IDENTITY: Self = Self(1);
    pub const FLIP_HORIZONTAL: Self = Self(2);
    pub const ROTATE_180: Self = Self(3);
    pub const FLIP_VERTICAL: Self = Self(4);
    pub const TRANSPOSE: Self = Self(5);
    pub const ROTATE_90: Self = Self(6);
    pub const ANTI_TRANSPOSE: Self = Self(7);
    pub const ROTATE_270: Self = Self(8);
}
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ExtraChannel(pub u32);
impl ExtraChannel {
    pub const ALPHA: Self = Self(0);
    pub const DEPTH: Self = Self(1);
    pub const SPOT_COLOR: Self = Self(2);
    pub const SELECTION_MASK: Self = Self(3);
    pub const BLACK: Self = Self(4);
    pub const CFA: Self = Self(5);
    pub const THERMAL: Self = Self(6);
    pub const RESERVED0: Self = Self(7);
    pub const RESERVED1: Self = Self(8);
    pub const RESERVED2: Self = Self(9);
    pub const RESERVED3: Self = Self(10);
    pub const RESERVED4: Self = Self(11);
    pub const RESERVED5: Self = Self(12);
    pub const RESERVED6: Self = Self(13);
    pub const RESERVED7: Self = Self(14);
    pub const UNKNOWN: Self = Self(15);
    pub const OPTIONAL: Self = Self(16);
}
impl FieldEnum for ExtraChannel {
    const BITS: u64 = 0x7f | (1 << 15) | (1 << 16);
    const NAME: &'static str = "ExtraChannel";
    fn from_raw(value: u32) -> Self {
        Self(value)
    }
    fn raw(self) -> u32 {
        self.0
    }
}
#[derive(Clone, Debug)]
pub struct BitDepth {
    pub floating_point_sample: bool,
    pub bits_per_sample: u32,
    pub exponent_bits_per_sample: u32,
}
impl Default for BitDepth {
    fn default() -> Self {
        let mut value = Self {
            floating_point_sample: false,
            bits_per_sample: 0,
            exponent_bits_per_sample: 0,
        };
        bundle::init(&mut value);
        value
    }
}
impl Fields for BitDepth {
    fn visit_fields(&mut self, visitor: &mut dyn Visitor) -> Status {
        visitor.bool(false, &mut self.floating_point_sample)?;
        if !self.floating_point_sample {
            visitor.u32(
                U32Enc::new(val(8), val(10), val(12), bits_offset(6, 1)),
                8,
                &mut self.bits_per_sample,
            )?;
            self.exponent_bits_per_sample = 0;
        } else {
            visitor.u32(
                U32Enc::new(val(32), val(16), val(24), bits_offset(6, 1)),
                32,
                &mut self.bits_per_sample,
            )?;
            self.exponent_bits_per_sample = self.exponent_bits_per_sample.wrapping_sub(1);
            visitor.bits(4, 7, &mut self.exponent_bits_per_sample)?;
            self.exponent_bits_per_sample = self.exponent_bits_per_sample.wrapping_add(1);
        }
        if self.floating_point_sample {
            if self.exponent_bits_per_sample < 2 || self.exponent_bits_per_sample > 8 {
                return Err(StatusCode::GenericError);
            }
            let mantissa = (self.bits_per_sample as i32)
                .wrapping_sub(self.exponent_bits_per_sample as i32)
                .wrapping_sub(1);
            if !(2..=23).contains(&mantissa) {
                return Err(StatusCode::GenericError);
            }
        } else if self.bits_per_sample > 31 {
            return Err(StatusCode::GenericError);
        }
        Ok(())
    }
}
#[derive(Clone, Debug)]
pub struct ExtraChannelInfo {
    pub all_default: bool,
    pub channel_type: ExtraChannel,
    pub bit_depth: BitDepth,
    pub dim_shift: u32,
    pub name: Vec<u8>,
    pub alpha_associated: bool,
    pub spot_color: [f32; 4],
    pub cfa_channel: u32,
}
impl Default for ExtraChannelInfo {
    fn default() -> Self {
        let mut value = Self {
            all_default: false,
            channel_type: ExtraChannel::default(),
            bit_depth: BitDepth::default(),
            dim_shift: 0,
            name: Vec::new(),
            alpha_associated: false,
            spot_color: [0.0; 4],
            cfa_channel: 0,
        };
        bundle::init(&mut value);
        value
    }
}
impl Fields for ExtraChannelInfo {
    fn visit_fields(&mut self, visitor: &mut dyn Visitor) -> Status {
        let mut all_default = self.all_default;
        let result = visitor.all_default(self, &mut all_default);
        self.all_default = all_default;
        if result {
            visitor.set_default(self);
            return Ok(());
        }
        visit_enum(visitor, ExtraChannel::ALPHA, &mut self.channel_type)?;
        visitor.visit_nested(&mut self.bit_depth)?;
        visitor.u32(
            U32Enc::new(val(0), val(3), val(4), bits_offset(3, 1)),
            0,
            &mut self.dim_shift,
        )?;
        if (1u32 << self.dim_shift) > 8 {
            return Err(StatusCode::GenericError);
        }
        visit_name_string(visitor, &mut self.name)?;
        if visitor.conditional(self.channel_type == ExtraChannel::ALPHA) {
            visitor.bool(false, &mut self.alpha_associated)?;
        }
        if visitor.conditional(self.channel_type == ExtraChannel::SPOT_COLOR) {
            for color in &mut self.spot_color {
                visitor.f16(0.0, color)?;
            }
        }
        if visitor.conditional(self.channel_type == ExtraChannel::CFA) {
            visitor.u32(
                U32Enc::new(val(1), bits(2), bits_offset(4, 3), bits_offset(8, 19)),
                1,
                &mut self.cfa_channel,
            )?;
        }
        if self.channel_type == ExtraChannel::UNKNOWN || (7..=14).contains(&self.channel_type.0) {
            return Err(StatusCode::GenericError);
        }
        Ok(())
    }
}
// Constants from lib/jxl/quantizer.h; quantizer operations remain unported.
const DEFAULT_QUANT_BIAS: [f32; 4] = [
    1.0 - f32::from_bits(0x3d5fd8c1),
    1.0 - f32::from_bits(0x3d8f78bc),
    1.0 - f32::from_bits(0x3d4c88c0),
    0.145,
];
#[derive(Clone, Debug)]
pub struct OpsinInverseMatrix {
    pub all_default: bool,
    pub inverse_matrix: Matrix3x3,
    pub opsin_biases: [f32; 3],
    pub quant_biases: [f32; 4],
}
impl Default for OpsinInverseMatrix {
    fn default() -> Self {
        let mut value = Self {
            all_default: false,
            inverse_matrix: [[0.0; 3]; 3],
            opsin_biases: [0.0; 3],
            quant_biases: [0.0; 4],
        };
        bundle::init(&mut value);
        value
    }
}
impl Fields for OpsinInverseMatrix {
    fn visit_fields(&mut self, visitor: &mut dyn Visitor) -> Status {
        let mut all_default = self.all_default;
        let result = visitor.all_default(self, &mut all_default);
        self.all_default = all_default;
        if result {
            visitor.set_default(self);
            return Ok(());
        }
        for (j, row) in DEFAULT_INVERSE_OPSIN_ABSORBANCE_MATRIX.iter().enumerate() {
            for (i, &value) in row.iter().enumerate() {
                visitor.f16(value, &mut self.inverse_matrix[j][i])?;
            }
        }
        for (i, &value) in NEG_OPSIN_ABSORBANCE_BIAS_RGB.iter().take(3).enumerate() {
            visitor.f16(value, &mut self.opsin_biases[i])?;
        }
        for (i, &value) in DEFAULT_QUANT_BIAS.iter().enumerate() {
            visitor.f16(value, &mut self.quant_biases[i])?;
        }
        Ok(())
    }
}
#[derive(Clone, Debug)]
pub struct ToneMapping {
    pub all_default: bool,
    pub intensity_target: f32,
    pub min_nits: f32,
    pub relative_to_max_display: bool,
    pub linear_below: f32,
}
impl Default for ToneMapping {
    fn default() -> Self {
        let mut value = Self {
            all_default: false,
            intensity_target: 0.0,
            min_nits: 0.0,
            relative_to_max_display: false,
            linear_below: 0.0,
        };
        bundle::init(&mut value);
        value
    }
}
impl Fields for ToneMapping {
    fn visit_fields(&mut self, visitor: &mut dyn Visitor) -> Status {
        let mut all_default = self.all_default;
        let result = visitor.all_default(self, &mut all_default);
        self.all_default = all_default;
        if result {
            visitor.set_default(self);
            return Ok(());
        }
        visitor.f16(255.0, &mut self.intensity_target)?;
        if self.intensity_target <= 0.0 {
            return Err(StatusCode::GenericError);
        }
        visitor.f16(0.0, &mut self.min_nits)?;
        if self.min_nits < 0.0 || self.min_nits > self.intensity_target {
            return Err(StatusCode::GenericError);
        }
        visitor.bool(false, &mut self.relative_to_max_display)?;
        visitor.f16(0.0, &mut self.linear_below)?;
        if self.linear_below < 0.0 || (self.relative_to_max_display && self.linear_below > 1.0) {
            return Err(StatusCode::GenericError);
        }
        Ok(())
    }
}
#[derive(Clone, Debug)]
pub struct CustomTransformData {
    pub nonserialized_xyb_encoded: bool,
    pub all_default: bool,
    pub opsin_inverse_matrix: OpsinInverseMatrix,
    pub custom_weights_mask: u32,
    pub upsampling2_weights: [f32; 15],
    pub upsampling4_weights: [f32; 55],
    pub upsampling8_weights: [f32; 210],
}
impl Default for CustomTransformData {
    fn default() -> Self {
        let mut value = Self {
            nonserialized_xyb_encoded: false,
            all_default: false,
            opsin_inverse_matrix: OpsinInverseMatrix::default(),
            custom_weights_mask: 0,
            upsampling2_weights: [0.0; 15],
            upsampling4_weights: [0.0; 55],
            upsampling8_weights: [0.0; 210],
        };
        bundle::init(&mut value);
        value
    }
}
impl Fields for CustomTransformData {
    fn visit_fields(&mut self, visitor: &mut dyn Visitor) -> Status {
        let mut all_default = self.all_default;
        let result = visitor.all_default(self, &mut all_default);
        self.all_default = all_default;
        if result {
            visitor.set_default(self);
            return Ok(());
        }
        if visitor.conditional(self.nonserialized_xyb_encoded) {
            visitor.visit_nested(&mut self.opsin_inverse_matrix)?;
        }
        visitor.bits(3, 0, &mut self.custom_weights_mask)?;
        if visitor.conditional(self.custom_weights_mask & 1 != 0) {
            for (i, &value) in WEIGHTS2.iter().enumerate() {
                visitor.f16(value, &mut self.upsampling2_weights[i])?;
            }
        }
        if visitor.conditional(self.custom_weights_mask & 2 != 0) {
            for (i, &value) in WEIGHTS4.iter().enumerate() {
                visitor.f16(value, &mut self.upsampling4_weights[i])?;
            }
        }
        if visitor.conditional(self.custom_weights_mask & 4 != 0) {
            for (i, &value) in WEIGHTS8.iter().enumerate() {
                visitor.f16(value, &mut self.upsampling8_weights[i])?;
            }
        }
        Ok(())
    }
}
pub struct ImageMetadata {
    pub all_default: bool,
    pub bit_depth: BitDepth,
    pub modular_16_bit_buffer_sufficient: bool,
    pub xyb_encoded: bool,
    pub color_encoding: ColorEncoding,
    pub orientation: u32,
    pub have_preview: bool,
    pub have_animation: bool,
    pub have_intrinsic_size: bool,
    pub intrinsic_size: SizeHeader,
    pub tone_mapping: ToneMapping,
    pub num_extra_channels: u32,
    pub extra_channel_info: Vec<ExtraChannelInfo>,
    pub preview_size: PreviewHeader,
    pub animation: AnimationHeader,
    pub extensions: u64,
    pub nonserialized_only_parse_basic_info: bool,
}
impl Default for ImageMetadata {
    fn default() -> Self {
        let mut value = Self {
            all_default: false,
            bit_depth: BitDepth::default(),
            modular_16_bit_buffer_sufficient: false,
            xyb_encoded: false,
            color_encoding: ColorEncoding::default(),
            orientation: 1,
            have_preview: false,
            have_animation: false,
            have_intrinsic_size: false,
            intrinsic_size: SizeHeader::default(),
            tone_mapping: ToneMapping::default(),
            num_extra_channels: 0,
            extra_channel_info: Vec::new(),
            preview_size: PreviewHeader::default(),
            animation: AnimationHeader::default(),
            extensions: 0,
            nonserialized_only_parse_basic_info: false,
        };
        bundle::init(&mut value);
        value
    }
}
impl Fields for ImageMetadata {
    fn visit_fields(&mut self, visitor: &mut dyn Visitor) -> Status {
        let mut all_default = self.all_default;
        let result = visitor.all_default(self, &mut all_default);
        self.all_default = all_default;
        if result {
            visitor.set_default(self);
            return Ok(());
        }
        let tone_default = if visitor.is_reading() {
            false
        } else {
            bundle::all_default(&mut self.tone_mapping)
        };
        let mut extra = self.orientation != 1
            || self.have_preview
            || self.have_animation
            || self.have_intrinsic_size
            || !tone_default;
        visitor.bool(false, &mut extra)?;
        if visitor.conditional(extra) {
            self.orientation = self.orientation.wrapping_sub(1);
            visitor.bits(3, 0, &mut self.orientation)?;
            self.orientation = self.orientation.wrapping_add(1);
            visitor.bool(false, &mut self.have_intrinsic_size)?;
            if visitor.conditional(self.have_intrinsic_size) {
                visitor.visit_nested(&mut self.intrinsic_size)?;
            }
            visitor.bool(false, &mut self.have_preview)?;
            if visitor.conditional(self.have_preview) {
                visitor.visit_nested(&mut self.preview_size)?;
            }
            visitor.bool(false, &mut self.have_animation)?;
            if visitor.conditional(self.have_animation) {
                visitor.visit_nested(&mut self.animation)?;
            }
        } else {
            self.orientation = 1;
            self.have_intrinsic_size = false;
            self.have_preview = false;
            self.have_animation = false;
        }
        visitor.visit_nested(&mut self.bit_depth)?;
        visitor.bool(true, &mut self.modular_16_bit_buffer_sufficient)?;
        self.num_extra_channels = self.extra_channel_info.len() as u32;
        visitor.u32(
            U32Enc::new(val(0), val(1), bits_offset(4, 2), bits_offset(12, 1)),
            0,
            &mut self.num_extra_channels,
        )?;
        if visitor.conditional(self.num_extra_channels != 0) {
            if visitor.is_reading() {
                self.extra_channel_info
                    .resize_with(self.num_extra_channels as usize, ExtraChannelInfo::default);
            }
            for info in &mut self.extra_channel_info {
                visitor.visit_nested(info)?;
            }
        }
        visitor.bool(true, &mut self.xyb_encoded)?;
        visitor.visit_nested(&mut self.color_encoding)?;
        if visitor.conditional(extra) {
            visitor.visit_nested(&mut self.tone_mapping)?;
        }
        if visitor.is_reading() && self.nonserialized_only_parse_basic_info {
            return Ok(());
        }
        visitor.begin_extensions(&mut self.extensions)?;
        visitor.end_extensions()
    }
}
impl ImageMetadata {
    pub fn find(&self, channel_type: ExtraChannel) -> Option<&ExtraChannelInfo> {
        self.extra_channel_info
            .iter()
            .find(|info| info.channel_type == channel_type)
    }
    pub fn find_mut(&mut self, channel_type: ExtraChannel) -> Option<&mut ExtraChannelInfo> {
        self.extra_channel_info
            .iter_mut()
            .find(|info| info.channel_type == channel_type)
    }
    pub fn get_alpha_bits(&self) -> u32 {
        self.find(ExtraChannel::ALPHA).map_or(0, |info| {
            debug_assert_ne!(info.bit_depth.bits_per_sample, 0);
            info.bit_depth.bits_per_sample
        })
    }
    pub fn has_alpha(&self) -> bool {
        self.get_alpha_bits() != 0
    }
    pub fn set_alpha_bits(&mut self, bits: u32, associated: bool) {
        let position = self
            .extra_channel_info
            .iter()
            .position(|info| info.channel_type == ExtraChannel::ALPHA);
        if bits == 0 {
            if position.is_some() {
                self.extra_channel_info
                    .retain(|info| info.channel_type != ExtraChannel::ALPHA);
            }
        } else if let Some(index) = position {
            let info = &mut self.extra_channel_info[index];
            info.bit_depth.bits_per_sample = bits;
            info.bit_depth.floating_point_sample = false;
            info.bit_depth.exponent_bits_per_sample = 0;
            info.alpha_associated = associated;
        } else {
            let mut info = ExtraChannelInfo {
                channel_type: ExtraChannel::ALPHA,
                ..ExtraChannelInfo::default()
            };
            info.bit_depth.bits_per_sample = bits;
            info.dim_shift = 0;
            info.alpha_associated = associated;
            self.extra_channel_info.insert(0, info);
        }
        self.num_extra_channels = self.extra_channel_info.len() as u32;
        if bits > 12 {
            self.modular_16_bit_buffer_sufficient = false;
        }
    }
    pub fn set_uint_samples(&mut self, bits: u32) {
        self.bit_depth.bits_per_sample = bits;
        self.bit_depth.exponent_bits_per_sample = 0;
        self.bit_depth.floating_point_sample = false;
        if bits > 12 {
            self.modular_16_bit_buffer_sufficient = false;
        }
    }
    pub fn set_float32_samples(&mut self) {
        self.bit_depth.bits_per_sample = 32;
        self.bit_depth.exponent_bits_per_sample = 8;
        self.bit_depth.floating_point_sample = true;
        self.modular_16_bit_buffer_sufficient = false;
    }
    pub fn set_float16_samples(&mut self) {
        self.bit_depth.bits_per_sample = 16;
        self.bit_depth.exponent_bits_per_sample = 5;
        self.bit_depth.floating_point_sample = true;
        self.modular_16_bit_buffer_sufficient = false;
    }
    pub fn set_intensity_target(&mut self, value: f32) {
        self.tone_mapping.intensity_target = value;
    }
    pub fn intensity_target(&self) -> f32 {
        debug_assert_ne!(self.tone_mapping.intensity_target, 0.0);
        self.tone_mapping.intensity_target
    }
    pub fn get_orientation(&self) -> Orientation {
        Orientation(self.orientation)
    }
}
#[derive(Default)]
pub struct CodecMetadata {
    pub m: ImageMetadata,
    pub size: SizeHeader,
    pub transform_data: CustomTransformData,
}
impl CodecMetadata {
    pub fn xsize(&self) -> usize {
        self.size.xsize()
    }
    pub fn ysize(&self) -> usize {
        self.size.ysize()
    }
    pub fn oriented_xsize(&self, keep: bool) -> usize {
        if self.m.orientation > 4 && !keep {
            self.ysize()
        } else {
            self.xsize()
        }
    }
    pub fn oriented_ysize(&self, keep: bool) -> usize {
        if self.m.orientation > 4 && !keep {
            self.xsize()
        } else {
            self.ysize()
        }
    }
    pub fn oriented_preview_xsize(&self, keep: bool) -> usize {
        if self.m.orientation > 4 && !keep {
            self.m.preview_size.ysize()
        } else {
            self.m.preview_size.xsize()
        }
    }
    pub fn oriented_preview_ysize(&self, keep: bool) -> usize {
        if self.m.orientation > 4 && !keep {
            self.m.preview_size.xsize()
        } else {
            self.m.preview_size.ysize()
        }
    }
}
pub fn read_image_metadata(reader: &mut BitReader<'_>, metadata: &mut ImageMetadata) -> Status {
    bundle::read(reader, metadata)
}
