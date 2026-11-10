// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! PassesEncoderState storage and constructor from lib/jxl/enc_cache.h.
//! Pass initialization, AC metadata, and VarDCT encoding remain unported.
use super::dct_util::AcImage;
use super::enc_ans::{EntropyEncodingData, Token};
use super::enc_bit_writer::BitWriter;
use super::enc_params::CompressParams;
use super::enc_progressive_split::ProgressiveSplitter;
use super::image::Plane;
use super::passes_state::PassesSharedState;
#[derive(Default)]
pub struct PassData {
    pub ac_tokens: Vec<Vec<Token>>,
    pub context_map: Vec<u8>,
    pub codes: EntropyEncodingData,
}
pub struct PassesEncoderState<'a> {
    pub shared: PassesSharedState<'a>,
    pub streaming_mode: bool,
    pub initialize_global_state: bool,
    pub dc_group_index: usize,
    pub coeffs: Vec<Option<Box<AcImage>>>,
    pub special_frames: Vec<Option<Box<BitWriter>>>,
    pub progressive_splitter: ProgressiveSplitter,
    pub cparams: CompressParams<'a>,
    pub passes: Vec<PassData>,
    pub histogram_idx: Vec<usize>,
    pub used_acs: u32,
    pub used_orders: Vec<u32>,
    pub x_qm_multiplier: f32,
    pub b_qm_multiplier: f32,
    pub initial_quant_masking1x1: Plane<f32>,
}
impl Default for PassesEncoderState<'_> {
    fn default() -> Self {
        Self::new()
    }
}
impl<'a> PassesEncoderState<'a> {
    pub fn new() -> Self {
        Self {
            shared: PassesSharedState::new(),
            streaming_mode: false,
            initialize_global_state: true,
            dc_group_index: 0,
            coeffs: Vec::new(),
            special_frames: Vec::new(),
            progressive_splitter: ProgressiveSplitter::default(),
            cparams: CompressParams::default(),
            passes: Vec::new(),
            histogram_idx: Vec::new(),
            used_acs: 0,
            used_orders: Vec::new(),
            x_qm_multiplier: 1.0,
            b_qm_multiplier: 1.0,
            initial_quant_masking1x1: Plane::default(),
        }
    }
    pub fn have_memory_manager(&self) -> bool {
        self.shared.have_memory_manager()
    }
}
