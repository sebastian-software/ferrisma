// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! ReconstructImage, ComputeBlockL2Distance and ComputeARHeuristics from
//! lib/jxl/enc_heuristics.cc: the adaptive-reconstruction (EPF sharpness)
//! control field chosen with the decoder in the loop. The rest of that file
//! is in `enc_heuristics.rs`.
//!
//! Language-required deviations:
//! - The thread pool argument is absent: RunOnPool without a runner calls the
//!   init function with one thread and then runs the groups in index order
//!   on thread 0, which these functions do directly (as `compute_ac_metadata`
//!   and the other serial encoder units).
//! - ReconstructImage's `dec_state->shared = &shared` is a scoped move of the
//!   encoder's PassesSharedState into the decoder state's storage slot and
//!   back, because the decoder state's shared pointer type ties the borrowed
//!   state's lifetime to its own (ADR 0003). The reached decoder paths
//!   (Init, ComputeSigma, PreparePipeline, DecodeGroupImpl) only read it.
//!   The decoder's own storage is read in the source only for a DC-level
//!   output writer (`dc_level != 0`), which a VarDCT frame never has: the
//!   encoder writes DC frames as modular frames. ReconstructImage therefore
//!   returns an admission result for `dc_level != 0`.
//!
//! Floating-point contraction (tools/encoder_coefficients_contraction.json):
//! ComputeBlockL2Distance's `err2[i] += mask2 * diff * diff` (f32) and the
//! two double sums of its weighted result use `mul_add_f32`/`mul_add_f64`,
//! fused on AArch64 and separate on x86-64. ComputeARHeuristics has no
//! contracted expression: its multiplier divides before adding.
use super::base::float::{max_f32, min_f32, mul_add_f32, mul_add_f64};
use super::base::rect::Rect;
use super::base::status::{Status, StatusCode, StatusOr};
use super::color_encoding_internal::ColorEncoding;
use super::common::SpeedTier;
use super::dct_util::AcImage;
use super::dec_cache::{GroupDecCache, PassesDecoderState, PipelineOptions};
use super::dec_group::{AcGroupDecoderState, decode_group_for_roundtrip};
use super::enc_cache::PassesEncoderState;
use super::enc_params::MIN_BUTTERAUGLI_FOR_DYNAMIC_AR;
use super::frame_dimensions::BLOCK_DIM;
use super::frame_header::{ColorTransform, FLAG_NOISE, FLAG_PATCHES, FLAG_SPLINES, FrameHeader};
use super::image::{Image3F, ImageF, Plane};
use super::image_bundle::ImageBundle;
use super::image_ops::fill_plane;
use super::passes_state::PassesSharedState;
use std::cell::RefCell;

/// FrameHeader::UpdateFlag.
fn update_flag(frame_header: &mut FrameHeader<'_>, condition: bool, flag: u64) {
    if condition {
        frame_header.flags |= flag;
    } else {
        frame_header.flags &= !flag;
    }
}

/// ReconstructImage: decode the encoder's quantized coefficients `coeffs`
/// with the shared state's current control fields through the low-memory
/// render pipeline (Gaborish, EPF, noise as the header selects), without the
/// color transform, and return the reconstructed XYB image of
/// `frame_dim.xsize` x `frame_dim.ysize`. `shared` is lent to the decoder
/// state for the call and returned unchanged.
///
/// Preconditions as in the source: `shared` has metadata, frame dimensions,
/// quantizer, matrices, strategy, quant fields, EPF sharpness, color
/// correlation map, DC and image features of the frame; `coeffs` holds one
/// 32-bit AC image per pass; the header's metadata is set.
pub fn reconstruct_image<'a>(
    orig_frame_header: &FrameHeader<'_>,
    shared: &mut PassesSharedState<'a>,
    coeffs: &[Option<Box<AcImage>>],
) -> StatusOr<Image3F> {
    let memory_manager = shared.memory_manager();
    let shared_metadata = shared
        .metadata
        .expect("source requires the shared metadata");

    let has_patches = shared.image_features.patches.borrow().has_any();
    let has_splines = shared.image_features.splines.borrow().has_any();
    let mut metadata = orig_frame_header
        .nonserialized_metadata
        .expect("source requires the frame header metadata")
        .clone();
    super::base::libcpp_vector::clear(&mut metadata.m.extra_channel_info);
    metadata.m.num_extra_channels = metadata.m.extra_channel_info.len() as u32;
    let mut frame_header = orig_frame_header.clone();
    update_flag(&mut frame_header, has_patches, FLAG_PATCHES);
    update_flag(&mut frame_header, has_splines, FLAG_SPLINES);
    frame_header.color_transform = ColorTransform::NONE;
    frame_header.nonserialized_metadata = Some(&metadata);
    frame_header.extra_channel_upsampling.clear();
    if frame_header.dc_level != 0 {
        // Admission result; see the module documentation.
        return Err(StatusCode::Unsupported);
    }

    let is_gray = shared_metadata.m.color_encoding.is_gray();
    // The decoded bundle precedes the decoder state so that the state's
    // pipeline, which borrows it, is dropped first.
    let decoded = RefCell::new(ImageBundle::new_in(
        memory_manager,
        Some(&shared_metadata.m),
    ));
    let mut dec_state: PassesDecoderState<'a, '_> = PassesDecoderState::new_in(memory_manager);
    dec_state
        .output_encoding_info
        .set_from_metadata(shared_metadata)?;
    dec_state
        .output_encoding_info
        .maybe_set_color_encoding(ColorEncoding::linear_srgb(is_gray))?;
    // dec_state->shared = &shared.
    std::mem::swap(&mut dec_state.shared_storage, shared);
    let result = reconstruct_with_state(
        &frame_header,
        shared_metadata,
        &decoded,
        &mut dec_state,
        coeffs,
    );
    dec_state.render_pipeline = None;
    std::mem::swap(&mut dec_state.shared_storage, shared);
    drop(dec_state);
    result?;
    let mut decoded = decoded.into_inner();
    Ok(std::mem::take(decoded.color_mut()))
}

fn reconstruct_with_state<'a, 'p>(
    frame_header: &'p FrameHeader<'_>,
    shared_metadata: &'a super::image_metadata::CodecMetadata,
    decoded: &'p RefCell<ImageBundle<'a>>,
    dec_state: &mut PassesDecoderState<'a, 'p>,
    coeffs: &[Option<Box<AcImage>>],
) -> Status
where
    'a: 'p,
{
    dec_state.init(frame_header)?;
    let memory_manager = dec_state.memory_manager();
    let frame_dim = dec_state.shared().frame_dim;
    {
        let mut decoded = decoded.borrow_mut();
        decoded.origin = frame_header.frame_origin;
        let mut tmp = Some(Image3F::create_in(
            frame_dim.xsize,
            frame_dim.ysize,
            memory_manager,
        )?);
        decoded.set_from_image(&mut tmp, &dec_state.output_encoding_info.color_encoding)?;
    }

    let options = PipelineOptions {
        use_slow_render_pipeline: false,
        coalescing: false,
        render_spotcolors: false,
        render_noise: true,
    };
    // SAFETY: Init cleared the main and extra outputs, and SetFromMetadata
    // and MaybeSetColorEncoding never set a CMS, so neither the CMS nor the
    // output-descriptor contract is reached.
    unsafe { dec_state.prepare_pipeline(frame_header, &shared_metadata.m, decoded, options) }?;

    // RunOnPool(pool = null): allocate_storage(1), then every group in order
    // on thread 0.
    let num_threads = 1;
    dec_state
        .render_pipeline
        .as_mut()
        .expect("PreparePipeline built the pipeline")
        .prepare_for_threads(num_threads, false)?;
    let mut group_dec_caches: Vec<GroupDecCache> = Vec::new();
    group_dec_caches.resize_with(num_threads, GroupDecCache::default);
    let thread = 0;
    for group_index in 0..frame_dim.num_groups as u32 {
        let group_index = group_index as usize;
        if frame_header.loop_filter.epf_iters > 0 {
            super::epf::compute_sigma(
                &frame_header.loop_filter,
                frame_dim.block_group_rect(group_index),
                dec_state,
            )?;
        }
        let mut input = dec_state
            .render_pipeline
            .as_mut()
            .unwrap()
            .get_input_buffers(group_index, thread);
        let group_state = AcGroupDecoderState {
            shared: &dec_state.shared_storage,
            code: &dec_state.code,
            context_map: &dec_state.context_map,
            coefficients: dec_state.coefficients.as_mut().unwrap(),
            x_dm_multiplier: dec_state.x_dm_multiplier,
            b_dm_multiplier: dec_state.b_dm_multiplier,
            quant_biases: &dec_state.output_encoding_info.opsin_params.quant_biases,
            upsampler8x: dec_state.upsampler8x.as_deref_mut(),
        };
        decode_group_for_roundtrip(
            frame_header,
            coeffs,
            group_index,
            group_state,
            &mut group_dec_caches[thread],
            &mut input,
            None,
        )?;
        if (frame_header.flags & FLAG_NOISE) != 0 {
            super::dec_noise::prepare_noise_input(
                dec_state.visible_frame_index,
                dec_state.nonvisible_frame_index,
                &frame_dim,
                frame_header,
                group_index,
                &mut input,
            );
        }
        input.done()?;
    }
    Ok(())
}

/// ComputeBlockL2Distance: the mask-weighted squared error of block
/// (`bx`, `by`) between `a` and `b`, with the source's per-channel weights.
pub fn compute_block_l2_distance(
    a: &Image3F,
    b: &Image3F,
    mask1x1: &ImageF,
    by: usize,
    bx: usize,
) -> f32 {
    let rect = Rect::new_clamped(
        bx * BLOCK_DIM,
        by * BLOCK_DIM,
        BLOCK_DIM,
        BLOCK_DIM,
        a.xsize(),
        a.ysize(),
    );
    let mut err2 = [0.0f32; 3];
    for y in 0..rect.ysize() {
        let row_a: [&[f32]; 3] =
            std::array::from_fn(|c| &a.plane(c).row(rect.y0() + y)[rect.x0()..]);
        let row_b: [&[f32]; 3] =
            std::array::from_fn(|c| &b.plane(c).row(rect.y0() + y)[rect.x0()..]);
        let row_mask = &mask1x1.row(rect.y0() + y)[rect.x0()..];
        for x in 0..rect.xsize() {
            let mask = row_mask[x];
            let mask2 = mask * mask;
            for i in 0..3 {
                let diff = row_a[i][x] - row_b[i][x];
                // err2[i] += mask2 * diff * diff (contracted).
                err2[i] = mul_add_f32(mask2 * diff, diff, err2[i]);
            }
        }
    }
    const W: [f64; 3] = [12.339445295782363, 1.0, 0.2];
    // kW[0] * err2[0] + kW[1] * err2[1] + kW[2] * err2[2] in double, both
    // additions contracted with their left product, then narrowed.
    let sum01 = mul_add_f64(W[0], f64::from(err2[0]), W[1] * f64::from(err2[1]));
    mul_add_f64(W[2], f64::from(err2[2]), sum01) as f32
}

/// ComputeARHeuristics: choose the per-block EPF sharpness (the adaptive
/// reconstruction control field `shared.epf_sharpness`) by reconstructing
/// the frame for each candidate value and comparing each block with the
/// pre-Gaborish `orig_opsin` under `initial_quant_masking1x1`.
///
/// This is the single entry the frame encoder calls after
/// InitializePassesEncoder and before ComputeACMetadata, as
/// ComputeVarDCTEncodingData does. Preconditions: `enc_state.coeffs` holds the
/// quantized coefficients of every pass, the shared state carries the
/// frame's heuristics results and DC, `enc_state.initial_quant_masking1x1`
/// is the InitialQuantField mask, and `orig_opsin` is the padded XYB image
/// before Gaborish, shrunk to the frame size. `rect` is unused, as in the
/// source; the pool argument is absent (serial).
pub fn compute_ar_heuristics(
    frame_header: &FrameHeader<'_>,
    enc_state: &mut PassesEncoderState<'_>,
    orig_opsin: &Image3F,
    _rect: Rect,
) -> Status {
    let butteraugli_distance = enc_state.cparams.butteraugli_distance;
    let speed_tier = enc_state.cparams.speed_tier;
    let frame_dim = enc_state.shared.frame_dim;
    let memory_manager = enc_state.shared.memory_manager();

    // std::min(5.0f, distance).
    let clamped_butteraugli = min_f32(5.0f32, butteraugli_distance);
    if butteraugli_distance < MIN_BUTTERAUGLI_FOR_DYNAMIC_AR
        || (speed_tier as i32) > (SpeedTier::Wombat as i32)
        || frame_header.loop_filter.epf_iters == 0
    {
        let epf_sharpness = &mut enc_state.shared.epf_sharpness;
        let full = Rect::new(0, 0, epf_sharpness.xsize(), epf_sharpness.ysize());
        fill_plane(4u8, epf_sharpness, full);
        return Ok(());
    }

    let epf_steps: Vec<u8> = if butteraugli_distance > 4.5f32 {
        vec![0, 4]
    } else {
        vec![0, 2, 7]
    };
    const NUM_EPF_VALS: usize = 8;
    let mut epf_steps_lut = [0usize; NUM_EPF_VALS];
    for (i, &step) in epf_steps.iter().enumerate() {
        epf_steps_lut[usize::from(step)] = i;
    }
    let mut error_images: [ImageF; NUM_EPF_VALS] = std::array::from_fn(|_| Plane::default());
    for &val in &epf_steps {
        {
            let epf_sharpness = &mut enc_state.shared.epf_sharpness;
            let full = Rect::new(0, 0, epf_sharpness.xsize(), epf_sharpness.ysize());
            fill_plane(val, epf_sharpness, full);
        }
        let decoded = reconstruct_image(frame_header, &mut enc_state.shared, &enc_state.coeffs)?;
        let image = &mut error_images[usize::from(val)];
        *image = Plane::create_in(
            frame_dim.xsize_blocks,
            frame_dim.ysize_blocks,
            0,
            memory_manager,
        )?;
        for by in 0..frame_dim.ysize_blocks {
            for bx in 0..frame_dim.xsize_blocks {
                image.row_mut(by)[bx] = compute_block_l2_distance(
                    orig_opsin,
                    &decoded,
                    &enc_state.initial_quant_masking1x1,
                    by,
                    bx,
                );
            }
        }
    }
    let mut histo = [[0usize; NUM_EPF_VALS]; 9];
    let mut totals = [1usize; 9];
    // Double literals narrowed to the source's float constants.
    const FAVOR_NO_SMOOTHING: f32 = 0.99f64 as f32;
    let epf_sharpness = &mut enc_state.shared.epf_sharpness;
    for by in 0..frame_dim.ysize_blocks {
        for bx in 0..frame_dim.xsize_blocks {
            let mut best_val = 0u8;
            let mut best_error = f32::MAX;
            let top_val = if by > 0 {
                epf_sharpness.row(by - 1)[bx]
            } else {
                0
            };
            let left_val = if bx > 0 {
                epf_sharpness.row(by)[bx - 1]
            } else {
                0
            };
            let top_error = error_images[usize::from(top_val)].row(by)[bx];
            let left_error = error_images[usize::from(left_val)].row(by)[bx];
            for &val in &epf_steps {
                let mut error = error_images[usize::from(val)].row(by)[bx];
                if val == 0 {
                    error *= FAVOR_NO_SMOOTHING;
                }
                if error < best_error {
                    best_val = val;
                    best_error = error;
                }
            }
            let out = if best_error < min_f32(top_error, left_error) {
                best_val
            } else if top_error < left_error {
                top_val
            } else {
                left_val
            };
            epf_sharpness.row_mut(by)[bx] = out;
            let context =
                epf_steps_lut[usize::from(top_val)] * 3 + epf_steps_lut[usize::from(left_val)];
            histo[context][usize::from(out)] += 1;
            totals[context] += 1;
        }
    }
    const C3BASE: f32 = 0.980_171_988_241_482_9_f64 as f32;
    const C3CLAMP: f32 = 0.859_703_389_199_282_9_f64 as f32;
    // std::pow(float, float) is powf.
    let c3 = max_f32(C3CLAMP, C3BASE.powf(clamped_butteraugli));
    const C5: f32 = 0.108_769_035_955_580_3_f64 as f32;
    let mut mul = [0.0f32; 3 * 3 * 3];
    for &top_val in &epf_steps {
        for &left_val in &epf_steps {
            for &val in &epf_steps {
                let context =
                    epf_steps_lut[usize::from(top_val)] * 3 + epf_steps_lut[usize::from(left_val)];
                let ctx_histo = &histo[context];
                let mulix = epf_steps_lut[usize::from(val)] + 3 * context;
                // size_t division, then the double std::log1p overload.
                let ratio = (ctx_histo[usize::from(val)] / totals[context]) as f64;
                mul[mulix] = (1.0
                    / (1.0 + f64::from(C5) * ratio.ln_1p() / f64::from(clamped_butteraugli)))
                    as f32;
                if val == 0 {
                    mul[mulix] *= c3;
                }
            }
        }
    }
    for by in 0..frame_dim.ysize_blocks {
        for bx in 0..frame_dim.xsize_blocks {
            let mut best_val = 0u8;
            let mut best_error = f32::MAX;
            let top_val = if by > 0 {
                epf_sharpness.row(by - 1)[bx]
            } else {
                0
            };
            let left_val = if bx > 0 {
                epf_sharpness.row(by)[bx - 1]
            } else {
                0
            };
            let context =
                epf_steps_lut[usize::from(top_val)] * 3 + epf_steps_lut[usize::from(left_val)];
            for &val in &epf_steps {
                let mulix = epf_steps_lut[usize::from(val)] + 3 * context;
                let error = error_images[usize::from(val)].row(by)[bx] * mul[mulix];
                if error < best_error {
                    best_val = val;
                    best_error = error;
                }
            }
            epf_sharpness.row_mut(by)[bx] = best_val;
        }
    }
    Ok(())
}
