// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! ColorCorrelation and ColorCorrelationMap from lib/jxl/chroma_from_luma.{h,cc}.
//! The scalar reference's contraction sites and partial DecodeDC state remain
//! explicit. Image planes use the existing native allocator mapping.

use super::base::float::mul_add_f32;
use super::base::status::{Status, StatusCode, StatusOr};
use super::cms::opsin_params::Y_TO_B_RATIO;
use super::dec_bit_reader::BitReader;
use super::field_encodings::{U32Enc, bits_offset, val};
use super::fields::{f16_coder, u32_coder};
use super::frame_dimensions::BLOCK_DIM;
use super::image::Plane;
use super::image_ops::zero_fill_image;

pub const COLOR_TILE_DIM: usize = 64;
pub const COLOR_TILE_DIM_IN_BLOCKS: usize = COLOR_TILE_DIM / BLOCK_DIM;
pub const DEFAULT_COLOR_FACTOR: u8 = 84;
pub const CFL_FIXED_POINT_PRECISION: u8 = 11;
pub const CFL_FIXED_POINT_RATIO_MAX: i32 = (256 << CFL_FIXED_POINT_PRECISION) - 1;
pub const COLOR_FACTOR_DIST: U32Enc = U32Enc::new(
    val(DEFAULT_COLOR_FACTOR as u32),
    val(256),
    bits_offset(8, 2),
    bits_offset(16, 258),
);

pub struct ColorCorrelation {
    dc_factors: [f32; 4],
    color_factor: u32,
    color_scale: f32,
    base_correlation_x: f32,
    base_correlation_b: f32,
    ytox_dc: i32,
    ytob_dc: i32,
}
impl Default for ColorCorrelation {
    fn default() -> Self {
        Self {
            dc_factors: [0.0; 4],
            color_factor: u32::from(DEFAULT_COLOR_FACTOR),
            color_scale: 1.0 / f32::from(DEFAULT_COLOR_FACTOR),
            base_correlation_x: 0.0,
            base_correlation_b: Y_TO_B_RATIO,
            ytox_dc: 0,
            ytob_dc: 0,
        }
    }
}
impl ColorCorrelation {
    pub fn y_to_x_ratio(&self, factor: i32) -> f32 {
        mul_add_f32(factor as f32, self.color_scale, self.base_correlation_x)
    }
    pub fn y_to_b_ratio(&self, factor: i32) -> f32 {
        mul_add_f32(factor as f32, self.color_scale, self.base_correlation_b)
    }
    pub fn is_jpeg_compatible(&self) -> bool {
        self.base_correlation_x == 0.0
            && self.base_correlation_b == 0.0
            && self.ytob_dc == 0
            && self.ytox_dc == 0
            && self.color_factor == u32::from(DEFAULT_COLOR_FACTOR)
    }
    pub fn ratio_jpeg(factor: i32) -> i32 {
        factor
            .checked_mul(1 << CFL_FIXED_POINT_PRECISION)
            .expect("source fixed-point product must fit i32")
            / i32::from(DEFAULT_COLOR_FACTOR)
    }
    pub fn set_color_factor(&mut self, factor: u32) {
        self.color_factor = factor;
        self.color_scale = 1.0 / self.color_factor as f32;
        self.recompute_dc_factors();
    }
    pub fn set_y_to_b_dc(&mut self, value: i32) {
        self.ytob_dc = value;
        self.recompute_dc_factors();
    }
    pub fn set_y_to_x_dc(&mut self, value: i32) {
        self.ytox_dc = value;
        self.recompute_dc_factors();
    }
    pub fn y_to_x_dc(&self) -> i32 {
        self.ytox_dc
    }
    pub fn y_to_b_dc(&self) -> i32 {
        self.ytob_dc
    }
    pub fn color_factor(&self) -> f32 {
        self.color_factor as f32
    }
    pub fn raw_color_factor(&self) -> u32 {
        self.color_factor
    }
    pub fn color_scale(&self) -> f32 {
        self.color_scale
    }
    pub fn base_correlation_x(&self) -> f32 {
        self.base_correlation_x
    }
    pub fn base_correlation_b(&self) -> f32 {
        self.base_correlation_b
    }
    pub fn dc_factors(&self) -> &[f32; 4] {
        &self.dc_factors
    }
    pub fn recompute_dc_factors(&mut self) {
        self.dc_factors[0] = self.y_to_x_ratio(self.ytox_dc);
        self.dc_factors[2] = self.y_to_b_ratio(self.ytob_dc);
    }
    pub fn decode_dc(&mut self, reader: &mut BitReader<'_>) -> Status {
        if reader.read_fixed_bits::<1>() == 1 {
            return Ok(());
        }
        self.set_color_factor(u32_coder::read(COLOR_FACTOR_DIST, reader));
        f16_coder::read(reader, &mut self.base_correlation_x)?;
        if self.base_correlation_x.abs() > 4.0 {
            return Err(StatusCode::GenericError);
        }
        f16_coder::read(reader, &mut self.base_correlation_b)?;
        if self.base_correlation_b.abs() > 4.0 {
            return Err(StatusCode::GenericError);
        }
        self.ytox_dc = reader.read_fixed_bits::<8>() as i32 + i32::from(i8::MIN);
        self.ytob_dc = reader.read_fixed_bits::<8>() as i32 + i32::from(i8::MIN);
        self.recompute_dc_factors();
        Ok(())
    }
}

#[derive(Default)]
pub struct ColorCorrelationMap {
    pub ytox_map: Plane<i8>,
    pub ytob_map: Plane<i8>,
    base: ColorCorrelation,
}
impl ColorCorrelationMap {
    pub fn create(xsize: usize, ysize: usize, xyb: bool) -> StatusOr<Self> {
        let mut result = Self::default();
        let xblocks = xsize.wrapping_add(COLOR_TILE_DIM - 1) / COLOR_TILE_DIM;
        let yblocks = ysize.wrapping_add(COLOR_TILE_DIM - 1) / COLOR_TILE_DIM;
        result.ytox_map = Plane::create(xblocks, yblocks)?;
        result.ytob_map = Plane::create(xblocks, yblocks)?;
        zero_fill_image(&mut result.ytox_map);
        zero_fill_image(&mut result.ytob_map);
        if !xyb {
            result.base.base_correlation_b = 0.0;
        }
        result.base.recompute_dc_factors();
        Ok(result)
    }
    pub fn base(&self) -> &ColorCorrelation {
        &self.base
    }
    pub fn decode_dc(&mut self, reader: &mut BitReader<'_>) -> Status {
        self.base.decode_dc(reader)
    }
}
