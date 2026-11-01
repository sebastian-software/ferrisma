// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Quantization storage/constructors and DC accessors/DecodeDC from
//! lib/jxl/quant_weights.{h,cc}. The owning QuantEncoding record includes its
//! base-class initialization. Typed payloads replace the source union; RAW owns
//! an optional vector and explicit move_from retains the source's null owner.
//! Matrix computation, coefficient tables, and their pointer APIs are unported.
//! DC decode retains assignments completed before an error.

use super::ac_strategy::{MAX_COEFF_AREA, NUM_VALID_STRATEGIES};
use super::base::status::{Status, StatusCode};
use super::dec_bit_reader::BitReader;
use super::fields::f16_coder;
use super::frame_dimensions::DCT_BLOCK_SIZE;
use super::memory_manager_internal::AlignedMemory;

pub const MAX_QUANT_TABLE_SIZE: usize = MAX_COEFF_AREA;
pub const NUM_PREDEFINED_TABLES: usize = 1;
pub const CEIL_LOG2_NUM_PREDEFINED_TABLES: usize = 0;
pub const LOG2_NUM_QUANT_MODES: usize = 3;
pub const LOG2_MAX_DISTANCE_BANDS: usize = 4;
pub const MAX_DISTANCE_BANDS: usize = 1 + (1 << LOG2_MAX_DISTANCE_BANDS);
pub type DistanceBandsArray = [[f32; MAX_DISTANCE_BANDS]; 3];

#[derive(Clone, Copy, Debug)]
pub struct DctQuantWeightParams {
    pub num_distance_bands: usize,
    pub distance_bands: DistanceBandsArray,
}
impl Default for DctQuantWeightParams {
    fn default() -> Self {
        Self::empty()
    }
}
impl DctQuantWeightParams {
    pub const fn empty() -> Self {
        Self {
            num_distance_bands: 0,
            distance_bands: [[0.0; MAX_DISTANCE_BANDS]; 3],
        }
    }
    pub const fn new(distance_bands: DistanceBandsArray, num_distance_bands: usize) -> Self {
        Self {
            num_distance_bands,
            distance_bands,
        }
    }
    pub fn from_bands<const N: usize>(bands: &[[f32; N]; 3]) -> Self {
        assert!(
            N <= MAX_DISTANCE_BANDS,
            "source memcpy must fit the band array"
        );
        let mut result = Self::empty();
        result.num_distance_bands = N;
        for (to, from) in result.distance_bands.iter_mut().zip(bands) {
            to[..N].copy_from_slice(from);
        }
        result
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum QuantMode {
    Library = 0,
    Identity,
    Dct2,
    Dct4,
    Dct4X8,
    Afv,
    Dct,
    Raw,
}

#[derive(Clone, Debug)]
pub enum QuantEncodingData {
    Identity([[f32; 3]; 3]),
    Dct2([[f32; 6]; 3]),
    Dct4([[f32; 2]; 3]),
    Afv([[f32; 9]; 3]),
    Dct4X8([f32; 3]),
    Raw {
        qtable: Option<Vec<i32>>,
        qtable_den: f32,
    },
}

/// Owning QuantEncoding, including the initialization in QuantEncodingInternal.
/// The C++ base's non-owning raw-pointer copies are not exposed as a Rust API.
#[derive(Debug)]
pub struct QuantEncoding {
    mode: QuantMode,
    pub dct_params: DctQuantWeightParams,
    data: QuantEncodingData,
    pub dct_params_afv_4x4: DctQuantWeightParams,
    pub predefined: u8,
}
impl Clone for QuantEncoding {
    fn clone(&self) -> Self {
        Self {
            mode: self.mode,
            dct_params: self.dct_params,
            data: self.data.clone(),
            dct_params_afv_4x4: self.dct_params_afv_4x4,
            predefined: self.predefined,
        }
    }
    fn clone_from(&mut self, other: &Self) {
        if let QuantEncodingData::Raw { qtable, .. } = &mut self.data {
            drop(qtable.take());
        }
        self.mode = other.mode;
        self.dct_params = other.dct_params;
        self.data = other.data.clone();
        self.dct_params_afv_4x4 = other.dct_params_afv_4x4;
        self.predefined = other.predefined;
    }
}
impl QuantEncoding {
    const fn with_data(mode: QuantMode, data: QuantEncodingData) -> Self {
        Self {
            mode,
            dct_params: DctQuantWeightParams::empty(),
            data,
            dct_params_afv_4x4: DctQuantWeightParams::empty(),
            predefined: 0,
        }
    }
    pub const fn library<const A: usize>() -> Self {
        const {
            assert!(A < NUM_PREDEFINED_TABLES);
        }
        let mut result = Self::with_data(QuantMode::Library, QuantEncodingData::Afv([[0.0; 9]; 3]));
        result.predefined = A as u8;
        result
    }
    pub const fn identity(weights: &[[f32; 3]; 3]) -> Self {
        Self::with_data(QuantMode::Identity, QuantEncodingData::Identity(*weights))
    }
    pub const fn dct2(weights: &[[f32; 6]; 3]) -> Self {
        Self::with_data(QuantMode::Dct2, QuantEncodingData::Dct2(*weights))
    }
    pub const fn dct4(params: &DctQuantWeightParams, multipliers: &[[f32; 2]; 3]) -> Self {
        let mut result = Self::with_data(QuantMode::Dct4, QuantEncodingData::Dct4(*multipliers));
        result.dct_params = *params;
        result
    }
    pub const fn dct4x8(params: &DctQuantWeightParams, multipliers: &[f32; 3]) -> Self {
        let mut result =
            Self::with_data(QuantMode::Dct4X8, QuantEncodingData::Dct4X8(*multipliers));
        result.dct_params = *params;
        result
    }
    pub const fn dct(params: &DctQuantWeightParams) -> Self {
        let mut result = Self::with_data(QuantMode::Dct, QuantEncodingData::Afv([[0.0; 9]; 3]));
        result.dct_params = *params;
        result
    }
    pub const fn afv(
        params4x8: &DctQuantWeightParams,
        params4x4: &DctQuantWeightParams,
        weights: &[[f32; 9]; 3],
    ) -> Self {
        let mut result = Self::with_data(QuantMode::Afv, QuantEncodingData::Afv(*weights));
        result.dct_params = *params4x8;
        result.dct_params_afv_4x4 = *params4x4;
        result
    }
    /// RAW copies its named C++ rvalue-reference input; the caller's table stays
    /// unchanged. Borrowing and cloning the slice preserves that source copy.
    pub fn raw(qtable: &[i32], shift: i32) -> Self {
        assert!(
            (0..32).contains(&shift),
            "source signed shift must be defined"
        );
        Self::with_data(
            QuantMode::Raw,
            QuantEncodingData::Raw {
                qtable: Some(qtable.to_vec()),
                qtable_den: (1i32 << shift) as f32 * (1.0f32 / (8 * 255) as f32),
            },
        )
    }
    pub fn mode(&self) -> QuantMode {
        self.mode
    }
    pub fn data(&self) -> &QuantEncodingData {
        &self.data
    }
    pub fn raw_table_mut(&mut self) -> Option<&mut Vec<i32>> {
        if let QuantEncodingData::Raw { qtable, .. } = &mut self.data {
            qtable.as_mut()
        } else {
            None
        }
    }
    pub fn move_from(other: &mut Self) -> Self {
        let data = match &mut other.data {
            QuantEncodingData::Raw { qtable, qtable_den } => QuantEncodingData::Raw {
                qtable: qtable.take(),
                qtable_den: *qtable_den,
            },
            data => data.clone(),
        };
        Self {
            mode: other.mode,
            dct_params: other.dct_params,
            data,
            dct_params_afv_4x4: other.dct_params_afv_4x4,
            predefined: other.predefined,
        }
    }
}

pub const INV_DC_QUANT: [f32; 3] = [4096.0, 512.0, 256.0];
pub const DC_QUANT: [f32; 3] = [
    1.0 / INV_DC_QUANT[0],
    1.0 / INV_DC_QUANT[1],
    1.0 / INV_DC_QUANT[2],
];

/// Original initialized constructor state. Source-indeterminate coefficient
/// pointers remain absent until the computation/accessor methods are translated.
pub struct DequantMatrices {
    pub computed_mask: u32,
    table_storage: Option<AlignedMemory>,
    dc_quant: [f32; 3],
    inv_dc_quant: [f32; 3],
    table_offsets: [usize; NUM_VALID_STRATEGIES as usize * 3],
    encodings: Vec<QuantEncoding>,
}
impl Default for DequantMatrices {
    fn default() -> Self {
        let mut result = Self {
            computed_mask: 0,
            table_storage: None,
            dc_quant: DC_QUANT,
            inv_dc_quant: INV_DC_QUANT,
            table_offsets: [0; NUM_VALID_STRATEGIES as usize * 3],
            encodings: vec![QuantEncoding::library::<0>(); NUM_QUANT_TABLES as usize],
        };
        let mut pos = 0usize;
        let mut offsets = [0usize; NUM_QUANT_TABLES as usize * 3];
        for i in 0..NUM_QUANT_TABLES as usize {
            let num_blocks = Self::REQUIRED_SIZE_X[i] as usize * Self::REQUIRED_SIZE_Y[i] as usize;
            let num = num_blocks * DCT_BLOCK_SIZE;
            for c in 0..3 {
                offsets[3 * i + c] = pos + c * num;
            }
            pos += 3 * num;
        }
        for (i, &table) in AC_STRATEGY_TO_QUANT_TABLE_MAP.iter().enumerate() {
            for c in 0..3 {
                result.table_offsets[i * 3 + c] = offsets[table as usize * 3 + c];
            }
        }
        result
    }
}
impl DequantMatrices {
    pub const REQUIRED_SIZE_X: [i32; NUM_QUANT_TABLES as usize] =
        [1, 1, 1, 1, 2, 4, 1, 1, 2, 1, 1, 8, 4, 16, 8, 32, 16];
    pub const REQUIRED_SIZE_Y: [i32; NUM_QUANT_TABLES as usize] =
        [1, 1, 1, 1, 2, 4, 2, 4, 4, 1, 1, 8, 8, 16, 16, 32, 32];
    pub const SUM_REQUIRED_XY: usize = 2056;
    pub const TOTAL_TABLE_SIZE: usize = Self::SUM_REQUIRED_XY * DCT_BLOCK_SIZE * 3;
    pub fn have_table_storage(&self) -> bool {
        self.table_storage.is_some()
    }
    pub fn table_offsets(&self) -> &[usize; NUM_VALID_STRATEGIES as usize * 3] {
        &self.table_offsets
    }
    pub fn encodings(&self) -> &Vec<QuantEncoding> {
        &self.encodings
    }
    pub fn set_encodings(&mut self, encodings: &Vec<QuantEncoding>) {
        self.encodings.clone_from(encodings);
        self.computed_mask = 0;
    }
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
pub const AC_STRATEGY_TO_QUANT_TABLE_MAP: [QuantTable; NUM_VALID_STRATEGIES as usize] = [
    QuantTable::Dct,
    QuantTable::Identity,
    QuantTable::Dct2X2,
    QuantTable::Dct4X4,
    QuantTable::Dct16X16,
    QuantTable::Dct32X32,
    QuantTable::Dct8X16,
    QuantTable::Dct8X16,
    QuantTable::Dct8X32,
    QuantTable::Dct8X32,
    QuantTable::Dct16X32,
    QuantTable::Dct16X32,
    QuantTable::Dct4X8,
    QuantTable::Dct4X8,
    QuantTable::Afv0,
    QuantTable::Afv0,
    QuantTable::Afv0,
    QuantTable::Afv0,
    QuantTable::Dct64X64,
    QuantTable::Dct32X64,
    QuantTable::Dct32X64,
    QuantTable::Dct128X128,
    QuantTable::Dct64X128,
    QuantTable::Dct64X128,
    QuantTable::Dct256X256,
    QuantTable::Dct128X256,
    QuantTable::Dct128X256,
];
