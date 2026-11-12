// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Quantization storage/constructors and DC accessors/DecodeDC from
//! lib/jxl/quant_weights.{h,cc}. The owning QuantEncoding record includes its
//! base-class initialization. Typed payloads replace the source union; RAW owns
//! an optional vector and explicit move_from retains the source's null owner.
//! AC library constants, scalar matrix computation, and borrowed accessors retain
//! the original storage layout; slices replace the aligned pointer APIs.
//! AC/DC decode retains assignments completed before an error. RAW table
//! decoding translates ModularFrameDecoder::DecodeQuantTable in dec_modular.cc
//! beside its owning record. Tables and RAW images allocate through the
//! decoder's memory manager. Target-specific SIMD remains outside this
//! scalar/null-pool boundary.

use super::ac_strategy::{AcStrategyType, MAX_COEFF_AREA, NUM_VALID_STRATEGIES};
use super::base::status::{Status, StatusCode};
use super::dec_bit_reader::BitReader;
use super::fields::f16_coder;
use super::frame_dimensions::DCT_BLOCK_SIZE;
use super::memory_manager_internal::{AlignedMemory, MemoryManagerPtr};

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

/// Original constructor state. The aligned owner binds both coefficient arrays
/// at EnsureComputed; immutable slices retain its original per-strategy offsets.
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

#[path = "quant_weights_library.rs"]
mod library;

const ALMOST_ZERO: f32 = 1e-8;

fn mult(v: f32) -> f32 {
    if v > 0.0 { 1.0 + v } else { 1.0 / (1.0 - v) }
}

fn interpolate(pos: f32, max: f32, array: &[f32]) -> super::base::status::StatusOr<f32> {
    let scaled_pos = pos * (array.len() - 1) as f32 / max;
    // Original size_t conversion is defined only for nonnegative finite input.
    assert!(scaled_pos.is_finite() && scaled_pos >= 0.0);
    let idx = scaled_pos as usize;
    if idx + 1 >= array.len() {
        return Err(StatusCode::GenericError);
    }
    let a = array[idx];
    let b = array[idx + 1];
    Ok(a * super::base::fast_math::fast_powf(b / a, scaled_pos - idx as f32))
}

fn get_quant_weights(
    rows: usize,
    cols: usize,
    params: &DctQuantWeightParams,
    out: &mut [f32],
) -> Status {
    assert!((1..=MAX_DISTANCE_BANDS).contains(&params.num_distance_bands));
    let num_bands = params.num_distance_bands;
    for c in 0..3 {
        let mut bands = [0.0f32; MAX_DISTANCE_BANDS];
        bands[0] = params.distance_bands[c][0];
        if bands[0] < ALMOST_ZERO {
            return Err(StatusCode::GenericError);
        }
        for i in 1..num_bands {
            bands[i] = bands[i - 1] * mult(params.distance_bands[c][i]);
            if bands[i] < ALMOST_ZERO {
                return Err(StatusCode::GenericError);
            }
        }
        let scale = (num_bands - 1) as f32 / (f32::from_bits(0x3fb504f3) + 1e-6f32);
        let rcpcol = scale / (cols - 1) as f32;
        let rcprow = scale / (rows - 1) as f32;
        for y in 0..rows {
            let dy = y as f32 * rcprow;
            let dy2 = dy * dy;
            for x in 0..cols {
                let dx = (x as f32 + 0.0) * rcpcol;
                let distance = (dx * dx + dy2).sqrt();
                let weight = if num_bands == 1 {
                    bands[0]
                } else {
                    let idx = super::base::hwy_scalar::to_i32(distance);
                    assert!(
                        idx >= 0 && (idx as usize) + 1 < MAX_DISTANCE_BANDS,
                        "source gather indices must be in bounds"
                    );
                    let frac = distance - idx as f32;
                    let a = bands[idx as usize];
                    let b = bands[idx as usize + 1];
                    a * super::base::fast_math::fast_powf(b / a, frac)
                };
                out[c * cols * rows + y * cols + x] = weight;
            }
        }
    }
    Ok(())
}

fn compute_quant_table(
    encoding: &QuantEncoding,
    table: &mut [f32],
    inv_table: &mut [f32],
    kind: usize,
    pos: &mut usize,
) -> Status {
    let wrows = 8 * DequantMatrices::REQUIRED_SIZE_X[kind] as usize;
    let wcols = 8 * DequantMatrices::REQUIRED_SIZE_Y[kind] as usize;
    let num = wrows * wcols;
    let mut weights = vec![0.0f32; 3 * num];
    match &encoding.data {
        _ if encoding.mode == QuantMode::Library => return Err(StatusCode::GenericError),
        QuantEncodingData::Identity(w) => {
            if num != DCT_BLOCK_SIZE {
                return Err(StatusCode::GenericError);
            }
            for (c, channel_weights) in w.iter().enumerate() {
                weights[c * 64..(c + 1) * 64].fill(channel_weights[0]);
                weights[c * 64 + 1] = channel_weights[1];
                weights[c * 64 + 8] = channel_weights[1];
                weights[c * 64 + 9] = channel_weights[2];
            }
        }
        QuantEncodingData::Dct2(w) => {
            if num != DCT_BLOCK_SIZE {
                return Err(StatusCode::GenericError);
            }
            for (c, channel_weights) in w.iter().enumerate() {
                let start = c * 64;
                weights[start] = 0xBAD as f32;
                weights[start + 1] = channel_weights[0];
                weights[start + 8] = channel_weights[0];
                weights[start + 9] = channel_weights[1];
                for y in 0..2 {
                    for x in 0..2 {
                        weights[start + y * 8 + x + 2] = channel_weights[2];
                        weights[start + (y + 2) * 8 + x] = channel_weights[2];
                    }
                }
                for y in 0..2 {
                    for x in 0..2 {
                        weights[start + (y + 2) * 8 + x + 2] = channel_weights[3];
                    }
                }
                for y in 0..4 {
                    for x in 0..4 {
                        weights[start + y * 8 + x + 4] = channel_weights[4];
                        weights[start + (y + 4) * 8 + x] = channel_weights[4];
                    }
                }
                for y in 0..4 {
                    for x in 0..4 {
                        weights[start + (y + 4) * 8 + x + 4] = channel_weights[5];
                    }
                }
            }
        }
        QuantEncodingData::Dct4(w) => {
            if num != DCT_BLOCK_SIZE {
                return Err(StatusCode::GenericError);
            }
            let mut small = [0.0; 3 * 16];
            get_quant_weights(4, 4, &encoding.dct_params, &mut small)?;
            for (c, channel_weights) in w.iter().enumerate() {
                for y in 0..8 {
                    for x in 0..8 {
                        weights[c * num + y * 8 + x] = small[c * 16 + (y / 2) * 4 + x / 2];
                    }
                }
                weights[c * num + 1] /= channel_weights[0];
                weights[c * num + 8] /= channel_weights[0];
                weights[c * num + 9] /= channel_weights[1];
            }
        }
        QuantEncodingData::Dct4X8(w) => {
            if num != DCT_BLOCK_SIZE {
                return Err(StatusCode::GenericError);
            }
            let mut small = [0.0; 3 * 32];
            get_quant_weights(4, 8, &encoding.dct_params, &mut small)?;
            for (c, channel_weights) in w.iter().enumerate() {
                for y in 0..8 {
                    for x in 0..8 {
                        weights[c * num + y * 8 + x] = small[c * 32 + (y / 2) * 8 + x];
                    }
                }
                weights[c * num + 8] /= channel_weights;
            }
        }
        _ if encoding.mode == QuantMode::Dct => {
            get_quant_weights(wrows, wcols, &encoding.dct_params, &mut weights)?
        }
        QuantEncodingData::Raw { qtable, qtable_den } => {
            let Some(qtable) = qtable else {
                return Err(StatusCode::GenericError);
            };
            if qtable.len() != 3 * num {
                return Err(StatusCode::GenericError);
            }
            for i in 0..3 * num {
                weights[i] = 1.0 / (*qtable_den * qtable[i] as f32);
            }
        }
        QuantEncodingData::Afv(w) => {
            const FREQS: [f32; 16] = [
                0xBAD as f32,
                0xBAD as f32,
                f32::from_bits(0x3f5a0e1e),
                f32::from_bits(0x40ac16cf),
                0xBAD as f32,
                0xBAD as f32,
                f32::from_bits(0x4097830e),
                f32::from_bits(0x40ae6038),
                f32::from_bits(0x3fd47536),
                4.0,
                f32::from_bits(0x40e8d2f0),
                f32::from_bits(0x4126c58a),
                f32::from_bits(0x402a6d7c),
                f32::from_bits(0x40f42e59),
                f32::from_bits(0x410f65f2),
                f32::from_bits(0x414f8bed),
            ];
            let mut small8 = [0.0; 3 * 32];
            get_quant_weights(4, 8, &encoding.dct_params, &mut small8)?;
            let mut small4 = [0.0; 3 * 16];
            get_quant_weights(4, 4, &encoding.dct_params_afv_4x4, &mut small4)?;
            let lo = f32::from_bits(0x3f5a0e1e);
            let hi = f32::from_bits(0x414f8bed) - lo + 1e-6f32;
            for (c, channel_weights) in w.iter().enumerate() {
                let mut bands = [0.0; 4];
                bands[0] = channel_weights[5];
                if bands[0] < ALMOST_ZERO {
                    return Err(StatusCode::GenericError);
                }
                for i in 1..4 {
                    bands[i] = bands[i - 1] * mult(channel_weights[i + 5]);
                    if bands[i] < ALMOST_ZERO {
                        return Err(StatusCode::GenericError);
                    }
                }
                let start = c * 64;
                weights[start] = 1.0;
                weights[start + 8] = channel_weights[0];
                weights[start + 1] = channel_weights[1];
                weights[start + 16] = channel_weights[2];
                weights[start + 2] = channel_weights[3];
                weights[start + 18] = channel_weights[4];
                for y in 0..4 {
                    for x in 0..4 {
                        if x < 2 && y < 2 {
                            continue;
                        }
                        weights[start + (2 * y) * 8 + 2 * x] =
                            interpolate(FREQS[y * 4 + x] - lo, hi, &bands)?;
                    }
                }
                for y in 0..4 {
                    for x in 0..8 {
                        if x == 0 && y == 0 {
                            continue;
                        }
                        weights[c * num + (2 * y + 1) * 8 + x] = small8[c * 32 + y * 8 + x];
                    }
                }
                for y in 0..4 {
                    for x in 0..4 {
                        if x == 0 && y == 0 {
                            continue;
                        }
                        weights[c * num + 2 * y * 8 + 2 * x + 1] = small4[c * 16 + y * 4 + x];
                    }
                }
            }
        }
    }
    let prev_pos = *pos;
    for i in 0..num * 3 {
        let inv_val = weights[i];
        if inv_val >= 1.0 / ALMOST_ZERO || inv_val < ALMOST_ZERO {
            return Err(StatusCode::GenericError);
        }
        table[*pos + i] = 1.0 / inv_val;
        inv_table[*pos + i] = inv_val;
    }
    *pos += 3 * num;
    let mut xs = DequantMatrices::REQUIRED_SIZE_X[kind] as usize;
    let mut ys = DequantMatrices::REQUIRED_SIZE_Y[kind] as usize;
    super::coeff_order_fwd::coefficient_layout(&mut ys, &mut xs);
    for c in 0..3 {
        for y in 0..ys {
            for x in 0..xs {
                inv_table[prev_pos + c * ys * xs * DCT_BLOCK_SIZE + y * 8 * xs + x] = 0.0;
            }
        }
    }
    Ok(())
}

impl DequantMatrices {
    pub fn library_init() -> [QuantEncoding; NUM_QUANT_TABLES as usize] {
        library::library_init()
    }
    pub fn library() -> &'static [QuantEncoding; NUM_QUANT_TABLES as usize] {
        static LIBRARY: std::sync::OnceLock<[QuantEncoding; NUM_QUANT_TABLES as usize]> =
            std::sync::OnceLock::new();
        LIBRARY.get_or_init(Self::library_init)
    }

    fn tables(&self) -> &[f32] {
        let storage = self
            .table_storage
            .as_ref()
            .expect("source matrix access requires storage");
        let bytes = storage.bytes();
        // SAFETY: AlignedMemory guarantees 128-byte alignment, this allocation
        // contains exactly 2*TOTAL_TABLE_SIZE initialized f32 slots, every f32
        // bit pattern is valid, and its owner remains immutably borrowed.
        unsafe { std::slice::from_raw_parts(bytes.as_ptr().cast(), bytes.len() / 4) }
    }
    pub fn matrix(&self, strategy: AcStrategyType, c: usize) -> &[f32] {
        let strategy = strategy.0;
        assert!(strategy < NUM_VALID_STRATEGIES as u32 && c < 3);
        assert_ne!((1u32 << strategy) & self.computed_mask, 0);
        let kind = AC_STRATEGY_TO_QUANT_TABLE_MAP[strategy as usize] as usize;
        let len = Self::REQUIRED_SIZE_X[kind] as usize
            * Self::REQUIRED_SIZE_Y[kind] as usize
            * DCT_BLOCK_SIZE;
        let offset = self.table_offsets[strategy as usize * 3 + c];
        &self.tables()[offset..offset + len]
    }
    pub fn inv_matrix(&self, strategy: AcStrategyType, c: usize) -> &[f32] {
        let strategy = strategy.0;
        assert!(strategy < NUM_VALID_STRATEGIES as u32 && c < 3);
        assert_ne!((1u32 << strategy) & self.computed_mask, 0);
        let kind = AC_STRATEGY_TO_QUANT_TABLE_MAP[strategy as usize] as usize;
        let len = Self::REQUIRED_SIZE_X[kind] as usize
            * Self::REQUIRED_SIZE_Y[kind] as usize
            * DCT_BLOCK_SIZE;
        let offset = Self::TOTAL_TABLE_SIZE + self.table_offsets[strategy as usize * 3 + c];
        &self.tables()[offset..offset + len]
    }
    pub fn ensure_computed(&mut self, acs_mask: u32) -> Status {
        self.ensure_computed_in(MemoryManagerPtr::NATIVE, acs_mask)
    }
    /// The source EnsureComputed(memory_manager, acs_mask).
    pub fn ensure_computed_in(
        &mut self,
        memory_manager: MemoryManagerPtr,
        acs_mask: u32,
    ) -> Status {
        let library = Self::library();
        if self.table_storage.is_none() {
            self.table_storage = Some(AlignedMemory::create(
                memory_manager,
                2 * Self::TOTAL_TABLE_SIZE * 4,
                0,
            )?);
        }
        let mut offsets = [0usize; NUM_QUANT_TABLES as usize * 3 + 1];
        let mut pos = 0;
        for i in 0..NUM_QUANT_TABLES as usize {
            let num = Self::REQUIRED_SIZE_X[i] as usize
                * Self::REQUIRED_SIZE_Y[i] as usize
                * DCT_BLOCK_SIZE;
            for c in 0..3 {
                offsets[3 * i + c] = pos + c * num;
            }
            pos += 3 * num;
        }
        offsets[NUM_QUANT_TABLES as usize * 3] = pos;
        if pos != Self::TOTAL_TABLE_SIZE {
            return Err(StatusCode::GenericError);
        }
        let mut kind_mask = 0u32;
        let mut computed_kind_mask = 0u32;
        for (i, quant_table) in AC_STRATEGY_TO_QUANT_TABLE_MAP.iter().enumerate() {
            if acs_mask & (1 << i) != 0 {
                kind_mask |= 1 << *quant_table as u32;
            }
            if self.computed_mask & (1 << i) != 0 {
                computed_kind_mask |= 1 << *quant_table as u32;
            }
        }
        let storage = self.table_storage.as_mut().unwrap();
        let bytes = storage.bytes_mut();
        // SAFETY: The allocation is 128-byte aligned with the exact initialized
        // byte length for two f32 arrays; all f32 words are valid. The exclusive
        // owner borrow and split_at_mut make the original restrict arrays disjoint.
        let values = unsafe {
            std::slice::from_raw_parts_mut(bytes.as_mut_ptr().cast::<f32>(), bytes.len() / 4)
        };
        let (table, inv_table) = values.split_at_mut(Self::TOTAL_TABLE_SIZE);
        for kind in 0..NUM_QUANT_TABLES as usize {
            if (1 << kind) & computed_kind_mask != 0 {
                continue;
            }
            if (1 << kind) & !kind_mask != 0 {
                continue;
            }
            let mut offset = offsets[kind * 3];
            let encoding = if self.encodings[kind].mode == QuantMode::Library {
                &library[kind]
            } else {
                &self.encodings[kind]
            };
            compute_quant_table(encoding, table, inv_table, kind, &mut offset)?;
            if offset != offsets[kind * 3 + 3] {
                return Err(StatusCode::GenericError);
            }
        }
        self.computed_mask |= acs_mask;
        Ok(())
    }
}

fn decode_dct_params(reader: &mut BitReader<'_>, params: &mut DctQuantWeightParams) -> Status {
    params.num_distance_bands = reader.read_fixed_bits::<LOG2_MAX_DISTANCE_BANDS>() as usize + 1;
    for c in 0..3 {
        for i in 0..params.num_distance_bands {
            f16_coder::read(reader, &mut params.distance_bands[c][i])?;
        }
        if params.distance_bands[c][0] < ALMOST_ZERO {
            return Err(StatusCode::GenericError);
        }
        params.distance_bands[c][0] *= 64.0;
    }
    Ok(())
}

// lib/jxl/dec_modular.cc: ModularFrameDecoder::DecodeQuantTable. The optional
// decoder is read-only at this boundary; its original tree/code owners remain
// borrowed instead of passing the containing mutable pointer.
fn decode_raw_quant_table(
    memory_manager: MemoryManagerPtr,
    required_size_x: usize,
    required_size_y: usize,
    reader: &mut BitReader<'_>,
    encoding: &mut QuantEncoding,
    idx: usize,
    decoder: Option<&super::dec_modular::ModularFrameDecoder>,
) -> Status {
    use super::modular::encoding::encoding::modular_generic_decompress;
    use super::modular::modular_image::Image;
    use super::modular::options::ModularOptions;
    let QuantEncodingData::Raw { qtable, qtable_den } = &mut encoding.data else {
        unreachable!("RAW owns its payload before the original decoder call")
    };
    f16_coder::read(reader, qtable_den)?;
    if *qtable_den < ALMOST_ZERO {
        return Err(StatusCode::GenericError);
    }
    let mut image = Image::create_in(memory_manager, required_size_x, required_size_y, 8, 3)?;
    let mut options = ModularOptions::default();
    if let Some(decoder) = decoder {
        let qt = super::dec_modular::ModularStreamId::quant_table(idx)?;
        modular_generic_decompress(
            reader,
            &mut image,
            None,
            qt.id(decoder.frame_dimensions()),
            &mut options,
            true,
            Some(decoder.tree()),
            Some(decoder.code()),
            Some(decoder.context_map()),
            false,
        )?;
    } else {
        modular_generic_decompress(
            reader,
            &mut image,
            None,
            0,
            &mut options,
            true,
            None,
            None,
            None,
            false,
        )?;
    }
    if qtable.is_none() {
        *qtable = Some(vec![0; required_size_x * required_size_y * 3]);
    } else if qtable.as_ref().unwrap().len() != required_size_x * required_size_y * 3 {
        return Err(StatusCode::GenericError);
    }
    let table = qtable.as_mut().unwrap();
    for c in 0..3 {
        for y in 0..required_size_y {
            let row = image.channel[c].row(y);
            for x in 0..required_size_x {
                table[c * required_size_x * required_size_y + y * required_size_x + x] = row[x];
                if row[x] <= 0 {
                    return Err(StatusCode::GenericError);
                }
            }
        }
    }
    Ok(())
}

impl QuantEncoding {
    // Decode is entered only after DequantMatrices::Decode has recreated every
    // encoding as Library<0>. Its zeroed 27-float AFV union member therefore
    // owns every partial non-RAW write until the final mode assignment. Keeping
    // that backing on failure preserves the observable old Library mode and
    // overlapping union bytes without reading an inactive C++ member in Rust.
    fn decode(
        &mut self,
        memory_manager: MemoryManagerPtr,
        reader: &mut BitReader<'_>,
        size_x: usize,
        size_y: usize,
        idx: usize,
        decoder: Option<&super::dec_modular::ModularFrameDecoder>,
    ) -> Status {
        let size = size_x * size_y;
        let mode = reader.read_fixed_bits::<LOG2_NUM_QUANT_MODES>();
        if matches!(mode, 1..=5) && size != 1 {
            return Err(StatusCode::GenericError);
        }
        match mode {
            0 => {
                self.predefined = reader.read_fixed_bits::<CEIL_LOG2_NUM_PREDEFINED_TABLES>() as u8;
                if self.predefined as usize >= NUM_PREDEFINED_TABLES {
                    return Err(StatusCode::GenericError);
                }
            }
            1..=5 => {
                let n = match mode {
                    1 => 3,
                    2 => 6,
                    3 => 2,
                    4 => 1,
                    _ => 9,
                };
                for c in 0..3 {
                    for i in 0..n {
                        let QuantEncodingData::Afv(backing) = &mut self.data else {
                            unreachable!()
                        };
                        let slot = &mut backing[(c * n + i) / 9][(c * n + i) % 9];
                        f16_coder::read(reader, slot)?;
                        if mode != 5 && slot.abs() < ALMOST_ZERO {
                            return Err(StatusCode::GenericError);
                        }
                        if mode == 1 || mode == 2 {
                            *slot *= 64.0;
                        }
                    }
                    if mode == 5 {
                        let QuantEncodingData::Afv(backing) = &mut self.data else {
                            unreachable!()
                        };
                        for value in &mut backing[c][..6] {
                            *value *= 64.0;
                        }
                    }
                }
                if matches!(mode, 3..=5) {
                    decode_dct_params(reader, &mut self.dct_params)?;
                }
                if mode == 5 {
                    decode_dct_params(reader, &mut self.dct_params_afv_4x4)?;
                }
            }
            6 => decode_dct_params(reader, &mut self.dct_params)?,
            7 => {
                self.mode = QuantMode::Raw;
                self.data = QuantEncodingData::Raw {
                    qtable: None,
                    qtable_den: 0.0,
                };
                decode_raw_quant_table(
                    memory_manager,
                    size_x * 8,
                    size_y * 8,
                    reader,
                    self,
                    idx,
                    decoder,
                )?;
            }
            _ => return Err(StatusCode::GenericError),
        }
        if let QuantEncodingData::Afv(backing) = &self.data {
            let flat: [f32; 27] = std::array::from_fn(|i| backing[i / 9][i % 9]);
            self.data = match mode {
                1 => QuantEncodingData::Identity(std::array::from_fn(|c| {
                    std::array::from_fn(|i| flat[c * 3 + i])
                })),
                2 => QuantEncodingData::Dct2(std::array::from_fn(|c| {
                    std::array::from_fn(|i| flat[c * 6 + i])
                })),
                3 => QuantEncodingData::Dct4(std::array::from_fn(|c| {
                    std::array::from_fn(|i| flat[c * 2 + i])
                })),
                4 => QuantEncodingData::Dct4X8([flat[0], flat[1], flat[2]]),
                _ => QuantEncodingData::Afv(*backing),
            };
        }
        self.mode = match mode {
            0 => QuantMode::Library,
            1 => QuantMode::Identity,
            2 => QuantMode::Dct2,
            3 => QuantMode::Dct4,
            4 => QuantMode::Dct4X8,
            5 => QuantMode::Afv,
            6 => QuantMode::Dct,
            7 => QuantMode::Raw,
            _ => unreachable!(),
        };
        Ok(())
    }
}

impl DequantMatrices {
    pub fn decode(
        &mut self,
        reader: &mut BitReader<'_>,
        decoder: Option<&super::dec_modular::ModularFrameDecoder>,
    ) -> Status {
        self.decode_in(MemoryManagerPtr::NATIVE, reader, decoder)
    }
    /// The source Decode(memory_manager, br, modular_frame_decoder).
    #[allow(clippy::needless_range_loop, reason = "Keep the source index loops.")]
    pub fn decode_in(
        &mut self,
        memory_manager: MemoryManagerPtr,
        reader: &mut BitReader<'_>,
        decoder: Option<&super::dec_modular::ModularFrameDecoder>,
    ) -> Status {
        let all_default = reader.read_bits(1);
        let num_tables = if all_default != 0 {
            0
        } else {
            NUM_QUANT_TABLES as usize
        };
        self.encodings.clear();
        self.encodings
            .resize(NUM_QUANT_TABLES as usize, QuantEncoding::library::<0>());
        for i in 0..num_tables {
            self.encodings[i].decode(
                memory_manager,
                reader,
                Self::REQUIRED_SIZE_X[i] as usize,
                Self::REQUIRED_SIZE_Y[i] as usize,
                i,
                decoder,
            )?;
        }
        self.computed_mask = 0;
        Ok(())
    }
}
