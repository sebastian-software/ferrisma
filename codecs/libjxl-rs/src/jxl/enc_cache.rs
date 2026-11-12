// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! PassesEncoderState storage and constructor from lib/jxl/enc_cache.h, and
//! ComputeACMetadata and InitializePassesEncoder (without progressive DC)
//! from enc_cache.cc. `std::pow(1.25f, scale - 2.0f)` is the host powf.
use super::base::rect::Rect;
use super::base::status::{Status, StatusCode};
use super::common::SpeedTier;
use super::compressed_dc::adaptive_dc_smoothing_in;
use super::dct_util::{AcImage, AcType};
use super::enc_ans::{EntropyEncodingData, Token};
use super::enc_bit_writer::BitWriter;
use super::enc_group::compute_coefficients;
use super::enc_modular::ModularFrameEncoder;
use super::enc_params::CompressParams;
use super::enc_progressive_split::ProgressiveSplitter;
use super::enc_quant_weights::dequant_matrices_scale_dc;
use super::frame_dimensions::GROUP_DIM;
use super::frame_header::{FLAG_SKIP_ADAPTIVE_DC_SMOOTHING, FLAG_USE_DC_FRAME, FrameHeader};
use super::image::Image3;
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

/// ComputeACMetadata from enc_cache.cc, serial (RunOnPool without a runner
/// runs the DC groups in index order).
pub fn compute_ac_metadata(
    enc_state: &mut PassesEncoderState<'_>,
    modular_frame_encoder: &mut ModularFrameEncoder<'_>,
) -> Status {
    let num_dc_groups = enc_state.shared.frame_dim.num_dc_groups;
    for group_index in 0..num_dc_groups as u32 {
        let group_index = group_index as usize;
        let r = enc_state.shared.frame_dim.dc_group_rect(group_index);
        let mut modular_group_index = group_index;
        if enc_state.streaming_mode {
            if group_index != 0 {
                return Err(StatusCode::GenericError);
            }
            modular_group_index = enc_state.dc_group_index;
        }
        modular_frame_encoder.add_ac_metadata(r, modular_group_index, false, enc_state)?;
    }
    Ok(())
}

/// InitializePassesEncoder from enc_cache.cc without the progressive-DC
/// branch: a frame header with kUseDcFrame is outside the translated path and
/// returns an error before any state changes. Serial schedules run in index
/// order, as RunOnPool does without a runner.
pub fn initialize_passes_encoder(
    frame_header: &FrameHeader<'_>,
    opsin: &Image3<f32>,
    rect: Rect,
    enc_state: &mut PassesEncoderState<'_>,
    modular_frame_encoder: &mut ModularFrameEncoder<'_>,
) -> Status {
    if frame_header.flags & FLAG_USE_DC_FRAME != 0 {
        // The recursive DC frame (EncodeFrame + DecodeFrame) is step 9.
        return Err(StatusCode::Unsupported);
    }
    let memory_manager = enc_state.shared.memory_manager();

    enc_state.x_qm_multiplier = 1.25f32.powf(frame_header.x_qm_scale as f32 - 2.0f32);
    enc_state.b_qm_multiplier = 1.25f32.powf(frame_header.b_qm_scale as f32 - 2.0f32);

    let num_passes = frame_header.passes.num_passes as usize;
    if enc_state.coeffs.len() < num_passes {
        enc_state.coeffs.reserve(num_passes);
        for _ in enc_state.coeffs.len()..num_passes {
            // Allocate enough coefficients for each group on every row.
            let coeffs = AcImage::make_in(
                memory_manager,
                AcType::I32,
                GROUP_DIM * GROUP_DIM,
                enc_state.shared.frame_dim.num_groups,
            )?;
            enc_state.coeffs.push(Some(coeffs));
        }
    }
    while enc_state.coeffs.len() > num_passes {
        enc_state.coeffs.pop();
    }

    if enc_state.initialize_global_state {
        let shared = &mut enc_state.shared;
        let scale = shared
            .quantizer
            .scale_global_scale(&shared.matrices, enc_state.cparams.quant_ac_rescale);
        dequant_matrices_scale_dc(&mut shared.matrices, scale)?;
        shared
            .quantizer
            .recompute_from_global_scale(&shared.matrices);
    }

    let mut dc = Image3::create_in(
        enc_state.shared.frame_dim.xsize_blocks,
        enc_state.shared.frame_dim.ysize_blocks,
        memory_manager,
    )?;
    for group_idx in 0..enc_state.shared.frame_dim.num_groups as u32 {
        compute_coefficients(group_idx as usize, enc_state, opsin, rect, &mut dc)?;
    }

    let num_dc_groups = enc_state.shared.frame_dim.num_dc_groups;
    for group_index in 0..num_dc_groups as u32 {
        let group_index = group_index as usize;
        let r = enc_state.shared.frame_dim.dc_group_rect(group_index);
        let mut modular_group_index = group_index;
        if enc_state.streaming_mode {
            if group_index != 0 {
                return Err(StatusCode::GenericError);
            }
            modular_group_index = enc_state.dc_group_index;
        }
        let nl_dc = enc_state.cparams.speed_tier < SpeedTier::Falcon;
        modular_frame_encoder.add_var_dct_dc(
            frame_header,
            &dc,
            r,
            modular_group_index,
            nl_dc,
            enc_state,
            false,
        )?;
    }
    // TODO(veluca): this is only useful in tests and if inspection is enabled.
    if frame_header.flags & FLAG_SKIP_ADAPTIVE_DC_SMOOTHING == 0 {
        let mul_dc = *enc_state.shared.quantizer.mul_dc();
        adaptive_dc_smoothing_in(
            memory_manager,
            &mul_dc,
            &mut enc_state.shared.dc_storage,
            &frame_header.chroma_subsampling,
            None,
        )?;
    }
    Ok(())
}
