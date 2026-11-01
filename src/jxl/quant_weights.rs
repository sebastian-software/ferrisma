// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! QuantTable indices and DC fields/accessors/DecodeDC from
//! lib/jxl/quant_weights.{h,cc}. Only the DC member initialization is translated;
//! the AC owners, full constructor, matrix computation, and table APIs remain
//! absent. DC decode retains assignments completed before an error.

use super::base::status::{Status, StatusCode};
use super::dec_bit_reader::BitReader;
use super::fields::f16_coder;

pub const INV_DC_QUANT: [f32; 3] = [4096.0, 512.0, 256.0];
pub const DC_QUANT: [f32; 3] = [
    1.0 / INV_DC_QUANT[0],
    1.0 / INV_DC_QUANT[1],
    1.0 / INV_DC_QUANT[2],
];

/// The source class's DC storage only. This does not implement its AC state or
/// claim that its complete constructor/quantization API has been translated.
pub struct DequantMatrices {
    dc_quant: [f32; 3],
    inv_dc_quant: [f32; 3],
}
impl Default for DequantMatrices {
    fn default() -> Self {
        Self {
            dc_quant: DC_QUANT,
            inv_dc_quant: INV_DC_QUANT,
        }
    }
}
impl DequantMatrices {
    pub fn dc_quant(&self, c: usize) -> f32 {
        self.dc_quant[c]
    }
    pub fn dc_quants(&self) -> &[f32; 3] {
        &self.dc_quant
    }
    pub fn inv_dc_quant(&self, c: usize) -> f32 {
        self.inv_dc_quant[c]
    }
    pub fn set_dc_quant(&mut self, dc: &[f32; 3]) {
        for (c, &value) in dc.iter().enumerate() {
            self.dc_quant[c] = 1.0 / value;
            self.inv_dc_quant[c] = value;
        }
    }
    pub fn decode_dc(&mut self, reader: &mut BitReader<'_>) -> Status {
        let all_default = reader.read_bits(1) != 0;
        reader.all_reads_within_bounds()?;
        if !all_default {
            for c in 0..3 {
                f16_coder::read(reader, &mut self.dc_quant[c])?;
                self.dc_quant[c] *= 1.0 / 128.0;
                if self.dc_quant[c] < 1e-8f32 {
                    return Err(StatusCode::GenericError);
                }
                self.inv_dc_quant[c] = 1.0 / self.dc_quant[c];
            }
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(usize)]
pub enum QuantTable {
    Dct = 0,
    Identity,
    Dct2X2,
    Dct4X4,
    Dct16X16,
    Dct32X32,
    Dct8X16,
    Dct8X32,
    Dct16X32,
    Dct4X8,
    Afv0,
    Dct64X64,
    Dct32X64,
    Dct128X128,
    Dct64X128,
    Dct256X256,
    Dct128X256,
}
pub const NUM_QUANT_TABLES: u8 = QuantTable::Dct128X256 as u8 + 1;
