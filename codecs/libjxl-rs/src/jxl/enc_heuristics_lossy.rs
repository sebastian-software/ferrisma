// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! LossyFrameHeuristics and FindBestDequantMatrices from
//! lib/jxl/enc_heuristics.cc, serial. They live beside `enc_heuristics.rs`
//! (FindBestBlockEntropyModel) so that the decoder-in-the-loop units of the
//! same source file (ComputeARHeuristics, ReconstructImage) can be translated
//! there independently.
//!
//! Reached branches: FindSplines returns no splines at the pinned revision,
//! so InitializeDrawCache and SubtractFrom draw nothing. FindBestPatchDictionary
//! runs FindTextLikePatches; a non-empty result enters the reference-frame
//! encode of step 5 and is an admission result here, as is the dot detection
//! (`dots` at distances from kMinButteraugliForDots). FindBestQuantizer has no
//! effect at kSquirrel and faster without `max_error_mode` (it optimizes only
//! with a linear image at kKitten and slower); `max_error_mode` and the
//! kKitten branch are admission results. The tile loop is RunOnPool without a
//! runner: one thread, tiles in index order.
//!
//! Contraction: the front-end fmuladd sites of enc_heuristics.cc are in
//! FindBestBlockEntropyModel's caller-independent code and the unported
//! downsampling and AR units; this file's arithmetic (`0.39 / distance`,
//! `0.79 / distance`, `1 / (q + 0.001)`, `distance * 0.62`) has none.

use super::base::override_option::apply_override;
use super::base::rect::Rect;
use super::common::SpeedTier;
use super::enc_ac_strategy::{AcStrategyHeuristics, AcsPlanes};
use super::enc_adaptive_quantization::{initial_quant_dc, initial_quant_field};
use super::enc_cache::PassesEncoderState;
use super::enc_chroma_from_luma::CfLHeuristics;
use super::enc_frame_vardct::{EncodeFrameError, FrameAdmission, FrameResult};
use super::enc_gaborish::gaborish_inverse;
use super::enc_heuristics::find_best_block_entropy_model;
use super::enc_params::{ENC_TILE_DIM_IN_BLOCKS, MIN_BUTTERAUGLI_FOR_DOTS};
use super::enc_patch_dictionary::find_text_like_patches;
use super::frame_header::FrameHeader;
use super::image::{Image3F, ImageF, Plane};
use super::image_ops::{fill_image, fill_plane};
use super::quant_weights::DequantMatrices;
use super::splines::Splines;

/// FindBestDequantMatrices: the default matrices. Custom matrices for
/// `max_error_mode` and `disable_perceptual_optimizations` are outside the
/// admitted profile.
fn find_best_dequant_matrices(
    enc_state: &mut PassesEncoderState<'_>,
    dequant_matrices: &mut DequantMatrices,
) -> FrameResult {
    *dequant_matrices = DequantMatrices::default();
    if enc_state.cparams.max_error_mode || enc_state.cparams.disable_perceptual_optimizations {
        return Err(EncodeFrameError::Admission(FrameAdmission::FrameProfile));
    }
    Ok(())
}

/// FindBestPatchDictionary up to its empty-result return, with the source's
/// default `is_xyb = true`.
fn find_best_patch_dictionary(opsin: &Image3F, enc_state: &PassesEncoderState<'_>) -> FrameResult {
    let cparams = &enc_state.cparams;
    let info = find_text_like_patches(cparams, opsin, &enc_state.shared.frame_dim, true)?;
    if info.is_empty()
        && apply_override(
            cparams.dots,
            cparams.speed_tier <= SpeedTier::Squirrel
                && cparams.butteraugli_distance >= MIN_BUTTERAUGLI_FOR_DOTS
                && !cparams.disable_perceptual_optimizations,
        )
    {
        // FindDotDictionary (enc_detect_dots.cc) is not translated.
        return Err(EncodeFrameError::Admission(FrameAdmission::FrameProfile));
    }
    if info.is_empty() {
        return Ok(());
    }
    // The bin packing, RoundtripPatchFrame and the reference-frame encode
    // are step 5.
    Err(EncodeFrameError::Admission(FrameAdmission::PatchDictionary))
}

/// LossyFrameHeuristics.
pub fn lossy_frame_heuristics(
    frame_header: &FrameHeader<'_>,
    enc_state: &mut PassesEncoderState<'_>,
    linear: Option<&Image3F>,
    opsin: &mut Image3F,
    rect: Rect,
) -> FrameResult {
    let cparams = enc_state.cparams.clone();
    let streaming_mode = enc_state.streaming_mode;
    let initialize_global_state = enc_state.initialize_global_state;
    let frame_dim = enc_state.shared.frame_dim;
    let memory_manager = enc_state.shared.memory_manager();

    // Find and subtract splines.
    let override_splines = cparams.custom_splines.has_any();
    if override_splines {
        // Custom splines are set only by internal callers.
        return Err(EncodeFrameError::Admission(FrameAdmission::FrameProfile));
    }
    if !streaming_mode && cparams.speed_tier <= SpeedTier::Squirrel {
        // FindSplines: spline detection is not implemented at the pinned
        // revision; it returns an empty Splines with the image's manager.
        *enc_state.shared.image_features.splines.borrow_mut() =
            Splines::new_in(opsin.memory_manager());
        let correlation = enc_state.shared.cmap.base();
        let mut splines = enc_state.shared.image_features.splines.borrow_mut();
        splines.initialize_draw_cache(opsin.xsize(), opsin.ysize(), &correlation)?;
        splines.subtract_from(opsin);
    }

    // Find and subtract patches/dots.
    if !streaming_mode && apply_override(cparams.patches, cparams.speed_tier <= SpeedTier::Squirrel)
    {
        find_best_patch_dictionary(opsin, enc_state)?;
        // PatchDictionaryEncoder::SubtractFrom with no patches changes nothing.
    }

    let quant_dc = initial_quant_dc(cparams.butteraugli_distance);

    let mut acs_heuristics = AcStrategyHeuristics::new(memory_manager, &cparams);
    let mut cfl_heuristics = CfLHeuristics::new(memory_manager);
    let mut initial_quant_field_image: ImageF;
    let mut initial_quant_masking = ImageF::default();

    // Compute an initial estimate of the quantization field.
    if cparams.speed_tier > SpeedTier::Hare || cparams.disable_perceptual_optimizations {
        initial_quant_field_image = Plane::create_in(
            frame_dim.xsize_blocks,
            frame_dim.ysize_blocks,
            0,
            memory_manager,
        )?;
        initial_quant_masking = Plane::create_in(
            frame_dim.xsize_blocks,
            frame_dim.ysize_blocks,
            0,
            memory_manager,
        )?;
        let q = (0.79 / f64::from(cparams.butteraugli_distance)) as f32;
        fill_image(q, &mut initial_quant_field_image);
        let masking = 1.0f32 / (q + 0.001f32);
        fill_image(masking, &mut initial_quant_masking);
        if cparams.disable_perceptual_optimizations {
            enc_state.initial_quant_masking1x1 =
                Plane::create_in(frame_dim.xsize, frame_dim.ysize, 0, memory_manager)?;
            fill_image(masking, &mut enc_state.initial_quant_masking1x1);
        }
        let shared = &mut enc_state.shared;
        shared
            .quantizer
            .compute_global_scale_and_quant(&shared.matrices, quant_dc, q, 0.0);
    } else {
        // Call this here, as it relies on pre-gaborish values.
        let mut butteraugli_distance_for_iqf = cparams.butteraugli_distance;
        if !frame_header.loop_filter.gab {
            butteraugli_distance_for_iqf *= 0.62f32;
        }
        initial_quant_field_image = initial_quant_field(
            butteraugli_distance_for_iqf,
            opsin,
            rect,
            None,
            1.0,
            &mut initial_quant_masking,
            &mut enc_state.initial_quant_masking1x1,
        )?;
        let q = (0.39 / f64::from(cparams.butteraugli_distance)) as f32;
        let shared = &mut enc_state.shared;
        shared
            .quantizer
            .compute_global_scale_and_quant(&shared.matrices, quant_dc, q, 0.0);
    }

    // Apply inverse-gaborish.
    if frame_header.loop_filter.gab {
        let weight = [1.0f32, 1.0, 1.0];
        gaborish_inverse(opsin, rect, &weight, None)?;
    }

    if initialize_global_state {
        let mut matrices = std::mem::take(&mut enc_state.shared.matrices);
        let result = find_best_dequant_matrices(enc_state, &mut matrices);
        enc_state.shared.matrices = matrices;
        result?;
    }

    cfl_heuristics.init(rect)?;
    acs_heuristics.init(
        opsin,
        rect,
        &initial_quant_field_image,
        &initial_quant_masking,
        &enc_state.initial_quant_masking1x1,
        &mut enc_state.shared.matrices,
    )?;

    let n_enc_tiles = frame_dim.xsize_blocks.div_ceil(ENC_TILE_DIM_IN_BLOCKS);
    let num_tiles = n_enc_tiles * frame_dim.ysize_blocks.div_ceil(ENC_TILE_DIM_IN_BLOCKS);
    // RunOnPool without a runner: prepare one thread, then tiles in order.
    if num_tiles != 0 {
        acs_heuristics.prepare_for_threads(1)?;
        cfl_heuristics.prepare_for_threads(1)?;
    }
    for tid in 0..num_tiles as u32 {
        let thread = 0;
        let tid = tid as usize;
        let tx = tid % n_enc_tiles;
        let ty = tid / n_enc_tiles;
        let by0 = ty * ENC_TILE_DIM_IN_BLOCKS;
        let by1 = ((ty + 1) * ENC_TILE_DIM_IN_BLOCKS).min(frame_dim.ysize_blocks);
        let bx0 = tx * ENC_TILE_DIM_IN_BLOCKS;
        let bx1 = ((tx + 1) * ENC_TILE_DIM_IN_BLOCKS).min(frame_dim.xsize_blocks);
        let r = Rect::new(bx0, by0, bx1 - bx0, by1 - by0);
        let shared = &mut enc_state.shared;

        // For speeds up to Wombat, we only compute the color correlation map
        // once we know the transform type and the quantization map.
        if cparams.speed_tier <= SpeedTier::Squirrel {
            cfl_heuristics.compute_tile(
                r,
                opsin,
                rect,
                &shared.matrices,
                None,
                None,
                None,
                false,
                thread,
                &mut shared.cmap,
            )?;
        }

        // Choose block sizes.
        let planes = AcsPlanes {
            src: opsin,
            rect_in: rect,
            quant_field: &initial_quant_field_image,
            mask: &initial_quant_masking,
            mask1x1: &enc_state.initial_quant_masking1x1,
            dequant: &shared.matrices,
        };
        acs_heuristics.process_rect(r, &shared.cmap, &mut shared.ac_strategy, thread, &planes)?;

        // Always set the initial quant field, so we can compute the CfL map
        // with more accuracy.
        super::enc_adaptive_quantization::adjust_quant_field(
            &shared.ac_strategy,
            r,
            cparams.butteraugli_distance,
            &mut initial_quant_field_image,
        )?;
        shared.quantizer.set_quant_field_rect(
            &initial_quant_field_image,
            r,
            &mut shared.raw_quant_field,
        );

        // Compute a non-default CfL map if we are at Hare speed, or slower.
        if cparams.speed_tier <= SpeedTier::Hare {
            cfl_heuristics.compute_tile(
                r,
                opsin,
                rect,
                &shared.matrices,
                Some(&shared.ac_strategy),
                Some(&shared.raw_quant_field),
                Some(&shared.quantizer),
                cparams.speed_tier >= SpeedTier::Wombat,
                thread,
                &mut shared.cmap,
            )?;
        }
    }

    acs_heuristics.finalize(&frame_dim, &enc_state.shared.ac_strategy, None)?;

    // Refine quantization levels.
    if !streaming_mode && !cparams.disable_perceptual_optimizations {
        let epf_sharpness = &mut enc_state.shared.epf_sharpness;
        let full = Rect::new(0, 0, epf_sharpness.xsize(), epf_sharpness.ysize());
        fill_plane(4u8, epf_sharpness, full);
        // FindBestQuantizer.
        if cparams.max_error_mode || (linear.is_some() && cparams.speed_tier <= SpeedTier::Kitten) {
            // FindBestQuantizationMaxError and FindBestQuantization (with
            // Butteraugli) are not translated.
            return Err(EncodeFrameError::Admission(FrameAdmission::FrameProfile));
        }
    }

    // Choose a context model that depends on the amount of quantization for AC.
    if cparams.speed_tier < SpeedTier::Falcon && initialize_global_state {
        let shared = &mut enc_state.shared;
        find_best_block_entropy_model(
            &cparams,
            &shared.raw_quant_field,
            &shared.ac_strategy,
            &mut shared.block_ctx_map,
        );
    }
    Ok(())
}
