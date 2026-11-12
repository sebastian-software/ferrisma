// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! The general frame entry of lib/jxl/enc_frame.cc for the admitted lossy
//! VarDCT profile: EncodeFrame (the JxlEncoderChunkedFrameAdapter overload)
//! with its setting resolution, CanDoStreamingEncoding, EncodeFrameStreaming
//! for output modes 0 and 1 with ComputePermutationForStreaming, TOCBucket,
//! TOCSize, ComputeGroupDataOffset, ComputeDcGlobalPadding, OutputGroups,
//! RemoveUnusedHistograms and OutputAcGlobal, EncodeFrameOneShot,
//! PermuteGroups' early return, GetBitDepth, general CopyColorChannels, the
//! VarDCT branch of ComputeEncodingData, ComputeChromacityAdjustments with
//! PixelStatsForChromacityAdjustment, the reached ComputeNoiseParams and
//! DownsampleColorChannels branches, ComputeVarDCTEncodingData,
//! ComputeAllCoeffOrders, TokenizeAllCoefficients, EncodeGlobalDCInfo,
//! EncodeGlobalACInfo and EncodeGroups, with their streaming branches. Serial: RunOnPool
//! without a runner runs one thread in index order. The fixed lossless
//! Thunder configuration in `enc_frame.rs` remains a separate internal entry.
//!
//! Admission (ADR 0001): a branch of these functions that enters an
//! untranslated unit returns [`EncodeFrameError::Admission`] at that branch;
//! the public encoder converts it into its admission result. The checkpoints
//! are [`FrameAdmission`]. ComputeARHeuristics is reached through the single
//! boundary [`compute_ar_heuristics_entry`].
//!
//! Contraction: PixelStatsForChromacityAdjustment's `cur_b - cur_y * 1.2`
//! (enc_frame.cc 624:33) is one front-end fmuladd site in double precision,
//! fused on AArch64 and separate on x86-64 (`mul_add_f64`). The other
//! enc_frame.cc sites are in SimplifyInvisible (alpha), ComputeNoiseParams'
//! noise estimate, the JPEG transcoding path, PermuteGroups' center-first
//! order and ParamsPostInit (already mapped in `enc_frame.rs`); none of the
//! first four is reached by the admitted profile.

use super::ac_strategy::AcStrategyImage;
use super::base::bits::ceil_log2_nonzero;
use super::base::float::{max_f32, mul_add_f64};
use super::base::override_option::Override;
use super::base::rect::Rect;
use super::base::status::{Status, StatusCode, StatusOr};
use super::chroma_from_luma::ColorCorrelationMap;
use super::coeff_order::COEFF_ORDER_MAX_SIZE;
use super::coeff_order_fwd::CoeffOrder;
use super::common::SpeedTier;
use super::dct_util::{AcType, ConstAcPtr};
use super::dec_modular::ModularStreamId;
use super::enc_ans::EntropyEncodingData;
use super::enc_ans_params::{HistogramParams, LZ77Method};
use super::enc_aux_out::LayerType;
use super::enc_bit_writer::BitWriter;
use super::enc_cache::{PassesEncoderState, compute_ac_metadata, initialize_passes_encoder};
use super::enc_chroma_from_luma::color_correlation_encode_dc;
use super::enc_coeff_order::{compute_coeff_order, compute_used_orders, encode_coeff_orders};
use super::enc_context_map::CLUSTERS_LIMIT;
use super::enc_context_map::encode_block_ctx_map;
use super::enc_entropy::{build_and_encode_histograms, encode_histograms};
use super::enc_entropy_coder::tokenize_coefficients;
use super::enc_external_image::convert_from_external_plane_no_size_check;
use super::enc_fields::{u32_coder, write_frame_header, write_quantizer_params};
use super::enc_frame::{
    FrameInfo, make_frame_header, num_groups_for_frame, params_post_init, set_progressive_mode,
};
use super::enc_group::encode_group_tokenized_coefficients;
use super::enc_heuristics_ar::compute_ar_heuristics;
use super::enc_heuristics_lossy::lossy_frame_heuristics;
use super::enc_modular::ModularFrameEncoder;
use super::enc_params::{CompressParams, MIN_BUTTERAUGLI_DISTANCE, MIN_BUTTERAUGLI_FOR_DOTS};
use super::enc_quant_weights::{dequant_matrices_encode_dc, dequant_matrices_encode_default};
use super::enc_toc::{write_toc_permutation, write_toc_sizes};
use super::enc_xyb::{XybInputEncoding, to_xyb};
use super::encode_internal::{
    JxlEncoderChunkedFrameAdapter, JxlEncoderOutputProcessorWrapper, append_data, bits_per_channel,
};
use super::fields::u32_coder::can_encode;
use super::frame_dimensions::{BLOCK_DIM, FrameDimensions, GROUP_DIM_IN_BLOCKS};
use super::frame_header::{
    BlendMode, ColorTransform, FLAG_NOISE, FLAG_PATCHES, FLAG_SPLINES, FLAG_USE_DC_FRAME,
    FrameEncoding, FrameHeader, FrameType, ORDER_ENC,
};
use super::image::{Image3, Image3F, Plane};
use super::image_metadata::{CodecMetadata, ExtraChannel, ImageMetadata};
use super::image_ops::{copy_image_to, copy_rect3_to, pad_image_to_block_multiple_in_place};
use super::toc::{ac_group_index, num_toc_entries};

/// kNumFixedHistograms from enc_ans.h.
const NUM_FIXED_HISTOGRAMS: usize = 1;
use super::types::{JxlBitDepth, JxlBitDepthType, JxlPixelFormat};

/// The source checkpoints of the frame encoder at which the public encoder
/// returns an admission result.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FrameAdmission {
    /// EncodeFrame when CanDoStreamingEncoding selects EncodeFrameStreaming
    /// for a frame outside the admitted profile, and its container output
    /// mode 2.
    StreamingFrame,
    /// EncodeFrame for settings, metadata or input outside the admitted
    /// one-shot profile (see [`admitted_frame_profile`]), and the reached
    /// branches of the one-shot path that enter untranslated units.
    FrameProfile,
    /// FindBestPatchDictionary when FindTextLikePatches returns patches: the
    /// reference-frame encode of step 5.
    PatchDictionary,
}

/// A frame-encoder failure: the source's status, or an admission result.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EncodeFrameError {
    Status(StatusCode),
    Admission(FrameAdmission),
}
impl From<StatusCode> for EncodeFrameError {
    fn from(code: StatusCode) -> Self {
        Self::Status(code)
    }
}
pub type FrameResult<T = ()> = Result<T, EncodeFrameError>;

fn admission<T>(checkpoint: FrameAdmission) -> FrameResult<T> {
    Err(EncodeFrameError::Admission(checkpoint))
}

/// The admitted profile of encoder steps 1 and 2, decided on the resolved
/// parameters at the point where EncodeFrame chooses EncodeFrameStreaming or
/// EncodeFrameOneShot:
/// effort 7 (kSquirrel) lossy VarDCT with XYB at distances below
/// kMinButteraugliForDots, gray or RGB enum sRGB or linear sRGB without extra
/// channels, a single displayed full-size frame without blending, names or
/// references, the default memory manager, serial, raw codestream output and
/// default perceptual and progressive options. Distances from 0.7, where the
/// original runs the AR heuristics, return the AR admission result later.
pub fn admitted_frame_profile(
    cparams: &CompressParams<'_>,
    frame_info: &FrameInfo,
    metadata: &CodecMetadata,
    frame_data: &JxlEncoderChunkedFrameAdapter<'_>,
    default_memory_manager: bool,
) -> bool {
    let m = &metadata.m;
    default_memory_manager
        && cparams.speed_tier == SpeedTier::Squirrel
        && !cparams.modular_mode
        && cparams.color_transform == ColorTransform::XYB
        && m.xyb_encoded
        && cparams.butteraugli_distance >= MIN_BUTTERAUGLI_DISTANCE
        && cparams.butteraugli_distance < MIN_BUTTERAUGLI_FOR_DOTS
        && !cparams.max_error_mode
        && !cparams.disable_perceptual_optimizations
        && cparams.decoding_speed_tier == 0
        && cparams.noise == Override::DEFAULT
        && cparams.dots == Override::DEFAULT
        && cparams.patches == Override::DEFAULT
        && cparams.gaborish == Override::DEFAULT
        && cparams.epf == -1
        && cparams.progressive_mode == Override::DEFAULT
        && cparams.qprogressive_mode == Override::DEFAULT
        && cparams.custom_progressive_mode.is_none()
        && cparams.progressive_dc == 0
        && cparams.responsive < 1
        && !cparams.centerfirst
        && cparams.photon_noise_iso == 0.0
        && cparams.manual_noise.is_empty()
        && cparams.manual_xyb_factors.is_empty()
        && !cparams.custom_splines.has_any()
        && cparams.resampling == 1
        && cparams.ec_resampling == 1
        && !cparams.already_downsampled
        && !cparams.lossy_palette
        && cparams.quant_ac_rescale == CompressParams::default().quant_ac_rescale
        && cparams.output_mode == 0
        && cparams.debug_image.is_none()
        && m.num_extra_channels == 0
        && !m.have_animation
        && !m.have_preview
        && !m.color_encoding.want_icc()
        && XybInputEncoding::new(&m.color_encoding).is_some()
        && frame_info.is_last
        && !frame_info.is_preview
        && frame_info.frame_type == FrameType::REGULAR
        && frame_info.dc_level == 0
        && frame_info.save_as_reference == 0
        && !frame_info.save_before_color_transform
        && frame_info.ib_needs_color_transform
        && frame_info.origin.x0 == 0
        && frame_info.origin.y0 == 0
        && !frame_info.blend
        && frame_info.blendmode == BlendMode::REPLACE
        && frame_info.duration == 0
        && frame_info.timecode == 0
        && frame_info.name.is_empty()
        && frame_data.xsize() == metadata.xsize()
        && frame_data.ysize() == metadata.ysize()
        && frame_data.channels().len() == 1
}

/// GetBitDepth.
pub fn get_bit_depth(bit_depth: JxlBitDepth, metadata_bits: u32, format: JxlPixelFormat) -> u32 {
    if bit_depth.depth_type == JxlBitDepthType::FROM_PIXEL_FORMAT {
        bits_per_channel(format.data_type) as u32
    } else if bit_depth.depth_type == JxlBitDepthType::FROM_CODESTREAM {
        metadata_bits
    } else if bit_depth.depth_type == JxlBitDepthType::CUSTOM {
        bit_depth.bits_per_sample
    } else {
        0
    }
}

/// CopyColorChannels with the buffer-backed input source. The release
/// callback of the source's buffer is a no-op, represented by the borrow.
pub fn copy_color_channels(
    input: &JxlEncoderChunkedFrameAdapter<'_>,
    rect: Rect,
    frame_info: &FrameInfo,
    metadata: &ImageMetadata,
    color: &mut Image3F,
    alpha: Option<&mut Plane<f32>>,
    has_interleaved_alpha: &mut bool,
) -> Status {
    let mut format = input.get_color_channels_pixel_format();
    format.align = 0; // align must be ignored.
    *has_interleaved_alpha = format.num_channels == 2 || format.num_channels == 4;
    let bits_per_sample = get_bit_depth(
        frame_info.image_bit_depth,
        metadata.bit_depth.bits_per_sample,
        format,
    ) as usize;
    let mut row_offset = 0;
    let data = input
        .get_color_channel_data_at(
            rect.x0(),
            rect.y0(),
            rect.xsize(),
            rect.ysize(),
            &mut row_offset,
        )
        .ok_or(StatusCode::GenericError)?;
    let color_channels = if frame_info.ib_needs_color_transform {
        metadata.color_encoding.channels()
    } else {
        3
    };
    if (format.num_channels as usize) < color_channels {
        return Err(StatusCode::GenericError);
    }
    for c in 0..color_channels {
        convert_from_external_plane_no_size_check(
            data,
            rect.xsize(),
            rect.ysize(),
            row_offset,
            bits_per_sample,
            format,
            c,
            color.plane_mut(c),
        )?;
    }
    if color_channels == 1 {
        let [first, second, third] = color.planes_mut();
        copy_image_to(first, second)?;
        copy_image_to(first, third)?;
    }
    if let Some(alpha) = alpha {
        if *has_interleaved_alpha {
            convert_from_external_plane_no_size_check(
                data,
                rect.xsize(),
                rect.ysize(),
                row_offset,
                bits_per_sample,
                format,
                format.num_channels as usize - 1,
                alpha,
            )?;
        } else {
            // if alpha is not passed, but it is expected, then assume
            // it is all-opaque
            super::image_ops::fill_image(1.0, alpha);
        }
    }
    Ok(())
}

/// PixelStatsForChromacityAdjustment.
#[derive(Default)]
struct PixelStatsForChromacityAdjustment {
    dx: f32,
    db: f32,
    exposed_blue: f32,
}
impl PixelStatsForChromacityAdjustment {
    fn calc_plane(plane: &Plane<f32>, rect: Rect) -> f32 {
        let mut xmax = 0.0f32;
        let mut ymax = 0.0f32;
        for ty in 1..rect.ysize() {
            let row = &plane.row(rect.y0() + ty)[rect.x0()..];
            let prev_row_v = &plane.row(rect.y0() + ty - 1)[rect.x0()..];
            for tx in 1..rect.xsize() {
                let cur = row[tx];
                let prev_row = prev_row_v[tx];
                let prev = row[tx - 1];
                xmax = max_f32(xmax, (cur - prev).abs());
                ymax = max_f32(ymax, (cur - prev_row).abs());
            }
        }
        max_f32(xmax, ymax)
    }
    fn calc_exposed_blue(&mut self, plane_y: &Plane<f32>, plane_b: &Plane<f32>, rect: Rect) {
        let mut eb = 0.0f32;
        let mut xmax = 0.0f32;
        let mut ymax = 0.0f32;
        for ty in 1..rect.ysize() {
            let row_y = &plane_y.row(rect.y0() + ty)[rect.x0()..];
            let row_b = &plane_b.row(rect.y0() + ty)[rect.x0()..];
            let prow_y = &plane_y.row(rect.y0() + ty - 1)[rect.x0()..];
            let prow_b = &plane_b.row(rect.y0() + ty - 1)[rect.x0()..];
            for tx in 1..rect.xsize() {
                let cur_y = row_y[tx];
                let cur_b = row_b[tx];
                // `cur_b - cur_y * 1.2` in double: one contracted site.
                let mut exposed_b = mul_add_f64(-f64::from(cur_y), 1.2, f64::from(cur_b)) as f32;
                let diff_b = cur_b - cur_y;
                let prev_row = prow_b[tx];
                let prev = row_b[tx - 1];
                let diff_prev_row = prev_row - prow_y[tx];
                let diff_prev = prev - row_y[tx - 1];
                xmax = max_f32(xmax, (diff_b - diff_prev).abs());
                ymax = max_f32(ymax, (diff_b - diff_prev_row).abs());
                if exposed_b >= 0.0 {
                    exposed_b *= (cur_b - prev).abs() + (cur_b - prev_row).abs();
                    eb = max_f32(eb, exposed_b);
                }
            }
        }
        self.exposed_blue = eb;
        self.db = max_f32(xmax, ymax);
    }
    fn calc(&mut self, opsin: &Image3F, rect: Rect) {
        self.dx = Self::calc_plane(opsin.plane(0), rect);
        self.calc_exposed_blue(opsin.plane(1), opsin.plane(2), rect);
    }
    fn how_much_is_x_channel_pixelized(&self) -> i32 {
        if f64::from(self.dx) >= 0.026 {
            return 3;
        }
        if f64::from(self.dx) >= 0.022 {
            return 2;
        }
        if f64::from(self.dx) >= 0.015 {
            return 1;
        }
        0
    }
    fn how_much_is_b_channel_pixelized(&self) -> i32 {
        let add = i32::from(f64::from(self.exposed_blue) >= 0.13);
        if f64::from(self.db) > 0.38 {
            return 2 + add;
        }
        if f64::from(self.db) > 0.33 {
            return 1 + add;
        }
        if f64::from(self.db) > 0.28 {
            return add;
        }
        0
    }
}

/// ComputeChromacityAdjustments.
fn compute_chromacity_adjustments(
    cparams: &CompressParams<'_>,
    opsin: &Image3F,
    rect: Rect,
    frame_header: &mut FrameHeader<'_>,
) {
    if frame_header.encoding != FrameEncoding::VAR_DCT || cparams.max_error_mode {
        return;
    }
    // 1) Distance based approach for chromacity adjustment:
    let x_qm_scale_steps = [2.5f32, 5.5, 9.5];
    frame_header.x_qm_scale = 3;
    for x_qm_scale_step in x_qm_scale_steps {
        if cparams.original_butteraugli_distance > x_qm_scale_step {
            frame_header.x_qm_scale += 1;
        }
    }
    // 2) Pixel-based approach for chromacity adjustment:
    let mut pixel_stats = PixelStatsForChromacityAdjustment::default();
    if cparams.speed_tier <= SpeedTier::Squirrel {
        pixel_stats.calc(opsin, rect);
    }
    // For X take the most severe adjustment.
    frame_header.x_qm_scale = (frame_header.x_qm_scale as i32)
        .max(2 + pixel_stats.how_much_is_x_channel_pixelized())
        as u32;
    // B only adjusted by pixel-based approach.
    frame_header.b_qm_scale = (2 + pixel_stats.how_much_is_b_channel_pixelized()) as u32;
}

/// ComputeNoiseParams at its reached branches. Noise synthesis (photon noise,
/// manual noise points, the noise estimate) is not translated.
fn compute_noise_params(
    cparams: &CompressParams<'_>,
    streaming_mode: bool,
    color_is_jpeg: bool,
    frame_header: &mut FrameHeader<'_>,
) -> FrameResult {
    if frame_header.frame_type == FrameType::DC {
        frame_header.flags &= !FLAG_NOISE;
        return Ok(());
    }
    if cparams.photon_noise_iso > 0.0 || cparams.manual_noise.len() == 8 {
        return admission(FrameAdmission::FrameProfile);
    }
    if frame_header.encoding == FrameEncoding::VAR_DCT
        && frame_header.flags & FLAG_NOISE != 0
        && !color_is_jpeg
        && !streaming_mode
    {
        return admission(FrameAdmission::FrameProfile);
    }
    Ok(())
}

/// DownsampleColorChannels at its early return; downsampling is not
/// translated.
fn downsample_color_channels(
    cparams: &CompressParams<'_>,
    frame_header: &FrameHeader<'_>,
    color_is_jpeg: bool,
) -> FrameResult {
    if color_is_jpeg || frame_header.upsampling == 1 || cparams.already_downsampled {
        return Ok(());
    }
    admission(FrameAdmission::FrameProfile)
}

/// The boundary to ComputeARHeuristics (enc_heuristics.cc), called once from
/// ComputeVarDCTEncodingData with the source's arguments: the translation in
/// `enc_heuristics_ar.rs` with the decoder in the loop. Its admission result
/// for a frame with `dc_level != 0` (an LF frame) maps to `FrameProfile`.
pub fn compute_ar_heuristics_entry(
    frame_header: &FrameHeader<'_>,
    enc_state: &mut PassesEncoderState<'_>,
    orig_opsin: &Image3F,
    rect: Rect,
) -> FrameResult {
    match compute_ar_heuristics(frame_header, enc_state, orig_opsin, rect) {
        Ok(()) => Ok(()),
        Err(StatusCode::Unsupported) if frame_header.dc_level != 0 => {
            admission(FrameAdmission::FrameProfile)
        }
        Err(code) => Err(code.into()),
    }
}

/// ComputeVarDCTEncodingData.
#[expect(
    clippy::manual_is_multiple_of,
    reason = "Mirror the source modulo test."
)]
fn compute_var_dct_encoding_data(
    frame_header: &FrameHeader<'_>,
    linear: Option<&Image3F>,
    opsin: &mut Image3F,
    rect: Rect,
    enc_modular: &mut ModularFrameEncoder<'_>,
    enc_state: &mut PassesEncoderState<'_>,
) -> FrameResult {
    if rect.xsize() % BLOCK_DIM != 0 || rect.ysize() % BLOCK_DIM != 0 {
        return Err(StatusCode::GenericError.into());
    }
    let memory_manager = enc_state.shared.memory_manager();
    // Save pre-Gaborish opsin for AR control field heuristics computation.
    let mut orig_opsin = Image3F::create_in(rect.xsize(), rect.ysize(), memory_manager)?;
    let orig_rect = Rect::new(0, 0, orig_opsin.xsize(), orig_opsin.ysize());
    copy_rect3_to(rect, opsin, orig_rect, &mut orig_opsin)?;
    orig_opsin.shrink_to(
        enc_state.shared.frame_dim.xsize,
        enc_state.shared.frame_dim.ysize,
    )?;

    lossy_frame_heuristics(frame_header, enc_state, linear, opsin, rect)?;

    match initialize_passes_encoder(frame_header, opsin, rect, enc_state, enc_modular) {
        // The recursive DC frame of InitializePassesEncoder is step 9.
        Err(StatusCode::Unsupported) if frame_header.flags & FLAG_USE_DC_FRAME != 0 => {
            return admission(FrameAdmission::FrameProfile);
        }
        result => result?,
    }

    compute_ar_heuristics_entry(frame_header, enc_state, &orig_opsin, rect)?;

    compute_ac_metadata(enc_state, enc_modular)?;
    Ok(())
}

/// ComputeAllCoeffOrders.
fn compute_all_coeff_orders(
    enc_state: &mut PassesEncoderState<'_>,
    frame_dim: &FrameDimensions,
) -> Status {
    let raw = &enc_state.shared.raw_quant_field;
    let used_orders_info = compute_used_orders(
        enc_state.cparams.speed_tier,
        &enc_state.shared.ac_strategy,
        Rect::new(0, 0, raw.xsize(), raw.ysize()),
    );
    let num_passes = enc_state.progressive_splitter.get_num_passes();
    enc_state.used_orders.resize(num_passes, 0);
    for i in 0..num_passes {
        let size = enc_state.shared.coeff_order_size;
        let shared = &mut enc_state.shared;
        compute_coeff_order(
            enc_state.cparams.speed_tier,
            enc_state.coeffs[i]
                .as_ref()
                .ok_or(StatusCode::GenericError)?,
            &shared.ac_strategy,
            frame_dim,
            &mut enc_state.used_orders[i],
            enc_state.used_acs,
            used_orders_info.0,
            used_orders_info.1,
            &mut shared.coeff_orders[i * size..],
        )?;
    }
    enc_state.used_acs |= used_orders_info.0;
    Ok(())
}

/// TokenizeAllCoefficients with its per-thread EncCache.
fn tokenize_all_coefficients(
    frame_header: &FrameHeader<'_>,
    enc_state: &mut PassesEncoderState<'_>,
) -> Status {
    let memory_manager = enc_state.shared.memory_manager();
    let num_groups = enc_state.shared.frame_dim.num_groups;
    // RunOnPool's init allocates one EncCache; InitOnce allocates its
    // num_nzeroes on first use.
    let mut num_nzeroes: Option<Image3<i32>> = None;
    for group_index in 0..num_groups as u32 {
        let group_index = group_index as usize;
        let rect = enc_state.shared.frame_dim.block_group_rect(group_index);
        for idx_pass in 0..enc_state.passes.len() {
            let coeffs = enc_state.coeffs[idx_pass]
                .as_ref()
                .ok_or(StatusCode::GenericError)?;
            if coeffs.image_type() != AcType::I32 {
                return Err(StatusCode::GenericError);
            }
            let rows: [&[i32]; 3] =
                std::array::from_fn(|c| match coeffs.plane_row(c, group_index, 0) {
                    ConstAcPtr::I32(row) => row,
                    ConstAcPtr::I16(_) => unreachable!("checked I32 above"),
                });
            if num_nzeroes.is_none() {
                num_nzeroes = Some(Image3::create_in(
                    GROUP_DIM_IN_BLOCKS,
                    GROUP_DIM_IN_BLOCKS,
                    memory_manager,
                )?);
            }
            let shared = &enc_state.shared;
            let mut tokens = std::mem::take(&mut enc_state.passes[idx_pass].ac_tokens[group_index]);
            let result = tokenize_coefficients(
                &shared.coeff_orders[idx_pass * shared.coeff_order_size..],
                rect,
                rows,
                &shared.ac_strategy,
                &frame_header.chroma_subsampling,
                num_nzeroes.as_mut().expect("initialized above"),
                &mut tokens,
                &shared.quant_dc,
                &shared.raw_quant_field,
                &shared.block_ctx_map,
            );
            enc_state.passes[idx_pass].ac_tokens[group_index] = tokens;
            result?;
        }
    }
    Ok(())
}

/// EncodeGlobalDCInfo.
fn encode_global_dc_info(enc_state: &PassesEncoderState<'_>, writer: &mut BitWriter) -> Status {
    let shared = &enc_state.shared;
    // Encode quantizer DC and global scale.
    let mut params = shared.quantizer.get_params();
    write_quantizer_params(&mut params, writer, LayerType::Quant, None)?;
    encode_block_ctx_map(&shared.block_ctx_map, writer, None)?;
    color_correlation_encode_dc(&shared.cmap.base(), writer, LayerType::Dc, None)?;
    Ok(())
}

/// EncodeGlobalACInfo with the default matrices that FindBestDequantMatrices
/// selects in the admitted profile. In streaming mode it only clusters and
/// stores the histograms; OutputAcGlobal writes the AC global section.
fn encode_global_ac_info(
    enc_state: &mut PassesEncoderState<'_>,
    writer: &mut BitWriter,
) -> FrameResult {
    dequant_matrices_encode_default(&enc_state.shared.matrices, writer, LayerType::Quant, None)?;
    let num_histo_bits = ceil_log2_nonzero(enc_state.shared.frame_dim.num_groups);
    if !enc_state.streaming_mode && num_histo_bits != 0 {
        let num_histograms = enc_state.shared.num_histograms;
        writer.with_max_bits(
            num_histo_bits,
            LayerType::Ac,
            None,
            |w, _| {
                w.write(num_histo_bits, num_histograms as u64 - 1);
                Ok(())
            },
            false,
        )?;
    }

    for i in 0..enc_state.progressive_splitter.get_num_passes() {
        // Encode coefficient orders.
        if !enc_state.streaming_mode {
            let used_orders = enc_state.used_orders[i];
            let mut order_bits = 0;
            can_encode(ORDER_ENC, used_orders, &mut order_bits)?;
            writer.with_max_bits(
                order_bits,
                LayerType::Order,
                None,
                |w, _| u32_coder::write(ORDER_ENC, used_orders, w),
                false,
            )?;
            let size = enc_state.shared.coeff_order_size;
            encode_coeff_orders(
                used_orders as u16,
                &enc_state.shared.coeff_orders[i * size..],
                writer,
                LayerType::Order,
                None,
            )?;
        }

        // Encode histograms.
        let num_ac_contexts = enc_state.shared.block_ctx_map.num_ac_contexts() as usize;
        let mut hist_params =
            HistogramParams::with_tier(enc_state.cparams.speed_tier, num_ac_contexts);
        if enc_state.cparams.speed_tier > SpeedTier::Tortoise {
            hist_params.lz77_method = LZ77Method::None;
        }
        if enc_state.cparams.decoding_speed_tier >= 1 {
            hist_params.max_histograms = 6;
        }
        let mut num_histogram_groups = enc_state.shared.num_histograms;
        if enc_state.streaming_mode {
            let mut prev_num_histograms = enc_state.passes[i].codes.encoding_info.len();
            if enc_state.initialize_global_state {
                prev_num_histograms += NUM_FIXED_HISTOGRAMS;
                hist_params.add_fixed_histograms = true;
            }
            let remaining_histograms = CLUSTERS_LIMIT.wrapping_sub(prev_num_histograms);
            // Heuristic to assign budget of new histograms to DC groups.
            let max_histograms = if remaining_histograms < 20 {
                remaining_histograms.min(4)
            } else {
                remaining_histograms / 4
            };
            hist_params.max_histograms = max_histograms.min(hist_params.max_histograms);
            num_histogram_groups = 1;
        }
        hist_params.streaming_mode = enc_state.streaming_mode;
        hist_params.initialize_global_state = enc_state.initialize_global_state;
        let pass = &mut enc_state.passes[i];
        build_and_encode_histograms(
            &hist_params,
            num_histogram_groups * num_ac_contexts,
            &mut pass.ac_tokens,
            &mut pass.codes,
            Some(writer),
            LayerType::Ac,
            None,
        )?;
    }
    Ok(())
}

/// EncodeGroups for a VarDCT frame without patches, splines or noise (their
/// encoders are admission results).
fn encode_groups(
    frame_header: &FrameHeader<'_>,
    enc_state: &mut PassesEncoderState<'_>,
    enc_modular: &mut ModularFrameEncoder<'_>,
    group_codes: &mut Vec<BitWriter>,
) -> FrameResult {
    let frame_dim = enc_state.shared.frame_dim;
    let num_groups = frame_dim.num_groups;
    let num_passes = enc_state.progressive_splitter.get_num_passes();
    let global_ac_index = frame_dim.num_dc_groups + 1;
    let is_small_image = !enc_state.streaming_mode && num_groups == 1 && num_passes == 1;
    let num_toc_entries = if is_small_image {
        1
    } else {
        ac_group_index(0, 0, num_groups, frame_dim.num_dc_groups)
            .wrapping_add(num_groups.wrapping_mul(num_passes))
    };
    if !group_codes.is_empty() {
        return Err(StatusCode::GenericError.into());
    }
    group_codes.reserve(num_toc_entries);
    for _ in 0..num_toc_entries {
        group_codes.push(BitWriter::new());
    }
    let output_index = |index: usize| if is_small_image { 0 } else { index };

    if enc_state.initialize_global_state {
        if frame_header.flags & (FLAG_PATCHES | FLAG_SPLINES | FLAG_NOISE) != 0 {
            // PatchDictionaryEncoder::Encode, EncodeSplines and EncodeNoise.
            return admission(FrameAdmission::FrameProfile);
        }
        let out = &mut group_codes[output_index(0)];
        dequant_matrices_encode_dc(&enc_state.shared.matrices, out, LayerType::Quant, None)?;
        if frame_header.encoding == FrameEncoding::VAR_DCT {
            encode_global_dc_info(enc_state, out)?;
        }
        enc_modular.encode_global_info(enc_state.streaming_mode, out, None)?;
        enc_modular.encode_stream(
            out,
            None,
            LayerType::ModularGlobal,
            &ModularStreamId::global(),
        )?;
    }

    // RunOnPool over the DC groups: one thread, groups in index order. The
    // AuxOut array is null without an aux_out. Streaming encodes only the
    // current DC group.
    let dc_groups: Vec<usize> = if enc_state.streaming_mode {
        if frame_dim.num_dc_groups != 1 {
            return Err(StatusCode::GenericError.into());
        }
        vec![enc_state.dc_group_index]
    } else {
        (0..frame_dim.num_dc_groups as u32)
            .map(|g| g as usize)
            .collect()
    };
    for group_index in dc_groups {
        let input_index = if enc_state.streaming_mode {
            0
        } else {
            group_index
        };
        let output = &mut group_codes[output_index(input_index + 1)];
        if frame_header.encoding == FrameEncoding::VAR_DCT
            && frame_header.flags & FLAG_USE_DC_FRAME == 0
        {
            let precision = enc_modular.extra_dc_precision[group_index];
            output.with_max_bits(
                2,
                LayerType::Dc,
                None,
                |w, _| {
                    w.write(2, u64::from(precision));
                    Ok(())
                },
                false,
            )?;
            enc_modular.encode_stream(
                output,
                None,
                LayerType::Dc,
                &ModularStreamId::var_dct_dc(group_index),
            )?;
        }
        enc_modular.encode_stream(
            output,
            None,
            LayerType::ModularDcGroup,
            &ModularStreamId::modular_dc(group_index),
        )?;
        if frame_header.encoding == FrameEncoding::VAR_DCT {
            let rect = enc_state.shared.frame_dim.dc_group_rect(input_index);
            let nb_bits = ceil_log2_nonzero(rect.xsize() * rect.ysize());
            if nb_bits != 0 {
                let size = enc_modular.ac_metadata_size[group_index];
                output.with_max_bits(
                    nb_bits,
                    LayerType::ControlFields,
                    None,
                    |w, _| {
                        w.write(nb_bits, size as u64 - 1);
                        Ok(())
                    },
                    false,
                )?;
            }
            enc_modular.encode_stream(
                output,
                None,
                LayerType::ControlFields,
                &ModularStreamId::ac_metadata(group_index),
            )?;
        }
    }
    if frame_header.encoding == FrameEncoding::VAR_DCT {
        let output = &mut group_codes[output_index(global_ac_index)];
        encode_global_ac_info(enc_state, output)?;
    }

    for group_index in 0..num_groups as u32 {
        let group_index = group_index as usize;
        let ac_group_id = if enc_state.streaming_mode {
            enc_modular.compute_streaming_absolute_ac_group_id(
                enc_state.dc_group_index,
                group_index,
                &enc_state.shared.frame_dim,
            )
        } else {
            group_index
        };
        for i in 0..num_passes {
            let output = &mut group_codes[output_index(ac_group_index(
                i,
                group_index,
                frame_dim.num_groups,
                frame_dim.num_dc_groups,
            ))];
            if frame_header.encoding == FrameEncoding::VAR_DCT {
                encode_group_tokenized_coefficients(
                    group_index,
                    i,
                    enc_state.histogram_idx[group_index],
                    enc_state,
                    output,
                    None,
                )?;
            }
            // Write all modular encoded data (color?, alpha, depth, extra channels)
            enc_modular.encode_stream(
                output,
                None,
                LayerType::ModularAcGroup,
                &ModularStreamId::modular_ac(ac_group_id, i),
            )?;
        }
    }

    for bw in group_codes.iter_mut() {
        bw.with_max_bits(
            8,
            LayerType::Ac,
            None,
            |w, _| {
                w.zero_pad_to_byte(); // end of group.
                Ok(())
            },
            false,
        )?;
    }
    Ok(())
}

/// ComputeEncodingData for a VarDCT frame of the admitted profile, whole or
/// one DC group in streaming mode. The original's other branches (JPEG input, extra channels,
/// non-XYB or color-transformed input, streaming) are admission results at
/// their checkpoints.
#[expect(
    clippy::too_many_arguments,
    reason = "Preserve the source's ComputeEncodingData parameters"
)]
fn compute_encoding_data<'m>(
    cparams: &CompressParams<'static>,
    frame_info: &FrameInfo,
    metadata: &'m CodecMetadata,
    frame_data: &JxlEncoderChunkedFrameAdapter<'_>,
    x0: usize,
    y0: usize,
    xsize: usize,
    ysize: usize,
    mutable_frame_header: &mut FrameHeader<'m>,
    enc_modular: &mut ModularFrameEncoder<'_>,
    enc_state: &mut PassesEncoderState<'m>,
    group_codes: &mut Vec<BitWriter>,
) -> FrameResult {
    if x0 + xsize > frame_data.xsize() || y0 + ysize > frame_data.ysize() {
        return Err(StatusCode::GenericError.into());
    }
    let memory_manager = enc_state.shared.memory_manager();
    {
        let shared = &mut enc_state.shared;
        shared.metadata = Some(metadata);
        if enc_state.streaming_mode {
            shared.frame_dim.set(
                xsize,
                ysize,
                mutable_frame_header.group_size_shift as usize,
                0,
                0,
                mutable_frame_header.encoding == FrameEncoding::MODULAR,
                1,
            );
        } else {
            shared.frame_dim = mutable_frame_header.to_frame_dimensions();
        }
        shared.bind_own_reference_frames();
        let dim = shared.frame_dim;
        shared.ac_strategy = AcStrategyImage::create(dim.xsize_blocks, dim.ysize_blocks)?;
        shared.raw_quant_field =
            Plane::create_in(dim.xsize_blocks, dim.ysize_blocks, 0, memory_manager)?;
        shared.epf_sharpness =
            Plane::create_in(dim.xsize_blocks, dim.ysize_blocks, 0, memory_manager)?;
        // ColorCorrelationMap::Create's default XYB=true.
        shared.cmap = ColorCorrelationMap::create(dim.xsize, dim.ysize, true)?;
        shared.coeff_order_size = COEFF_ORDER_MAX_SIZE;
        if mutable_frame_header.encoding == FrameEncoding::VAR_DCT {
            shared.coeff_orders.resize(
                mutable_frame_header.passes.num_passes as usize * COEFF_ORDER_MAX_SIZE,
                0,
            );
        }
        shared.quant_dc = Plane::create_in(dim.xsize_blocks, dim.ysize_blocks, 0, memory_manager)?;
        shared.dc_storage = Image3::create_in(dim.xsize_blocks, dim.ysize_blocks, memory_manager)?;
        shared.dc_source = super::passes_state::DcSource::Storage;
    }
    let frame_dim = enc_state.shared.frame_dim;

    let num_extra_channels = metadata.m.num_extra_channels as usize;
    if num_extra_channels != 0 {
        // CopyExtraChannels and the modular extra-channel encode are step 4.
        return admission(FrameAdmission::FrameProfile);
    }
    let alpha_eci = metadata.m.find(ExtraChannel::ALPHA);
    let c_enc = metadata.m.color_encoding.clone();

    // Make the image patch bigger than the currently processed group in
    // streaming mode so that we can take into account border pixels around
    // the group when computing inverse Gaborish and adaptive quantization map.
    let max_border = if enc_state.streaming_mode {
        BLOCK_DIM
    } else {
        0
    };
    let frame_rect = Rect::new(0, 0, frame_data.xsize(), frame_data.ysize());
    let frame_area_rect = Rect::new(x0, y0, xsize, ysize);
    let patch_rect = frame_area_rect.extend(max_border, frame_rect);
    if !patch_rect.is_inside(frame_rect) {
        return Err(StatusCode::GenericError.into());
    }

    // Allocating a large enough image avoids a copy when padding.
    let round_up = |n: usize| n.div_ceil(BLOCK_DIM) * BLOCK_DIM;
    let mut color = Image3F::create_in(
        round_up(patch_rect.xsize()),
        round_up(patch_rect.ysize()),
        memory_manager,
    )?;
    color.shrink_to(patch_rect.xsize(), patch_rect.ysize())?;
    let mut has_interleaved_alpha = false;
    debug_assert!(alpha_eci.is_none());
    copy_color_channels(
        frame_data,
        patch_rect,
        frame_info,
        &metadata.m,
        &mut color,
        None,
        &mut has_interleaved_alpha,
    )?;

    enc_state.cparams = cparams.clone();

    let linear: Option<&Image3F> = None;
    if mutable_frame_header.color_transform == ColorTransform::XYB
        && frame_info.ib_needs_color_transform
    {
        if mutable_frame_header.encoding == FrameEncoding::VAR_DCT
            && cparams.speed_tier <= SpeedTier::Kitten
        {
            // The linear image feeds FindBestQuantization (Butteraugli).
            return admission(FrameAdmission::FrameProfile);
        }
        let Some(encoding) = XybInputEncoding::new(&c_enc) else {
            // ApplyColorTransform through the CMS is not translated.
            return admission(FrameAdmission::FrameProfile);
        };
        to_xyb(
            encoding,
            metadata.m.intensity_target(),
            None,
            None,
            &mut color,
            None,
        )?;
    } else {
        // RGB or YCbCr input, or a special frame already in XYB: VarDCT
        // without the XYB transform is outside the profile.
        return admission(FrameAdmission::FrameProfile);
    }
    // SimplifyInvisible needs alpha, which the profile excludes.
    pad_image_to_block_multiple_in_place(&mut color, BLOCK_DIM)?;

    // Rectangle within color that corresponds to the currently processed group
    // in streaming mode.
    let mut group_rect = Rect::new(
        x0.wrapping_sub(patch_rect.x0()),
        y0.wrapping_sub(patch_rect.y0()),
        round_up(xsize),
        round_up(ysize),
    );

    if enc_state.initialize_global_state {
        compute_chromacity_adjustments(cparams, &color, group_rect, mutable_frame_header);
    }

    compute_noise_params(
        cparams,
        enc_state.streaming_mode,
        false,
        mutable_frame_header,
    )?;

    downsample_color_channels(cparams, mutable_frame_header, false)?;

    if cparams.ec_resampling != 1 && !cparams.already_downsampled {
        // No extra channels: the loop has no iteration.
    }

    if !enc_state.streaming_mode {
        group_rect = Rect::new(0, 0, color.xsize(), color.ysize());
    }

    if mutable_frame_header.encoding != FrameEncoding::VAR_DCT {
        return admission(FrameAdmission::FrameProfile);
    }
    let num_passes = enc_state.progressive_splitter.get_num_passes();
    if num_passes != 1 {
        // SplitACCoefficients for several passes is step 9.
        return admission(FrameAdmission::FrameProfile);
    }
    enc_state.passes.resize_with(num_passes, Default::default);
    for pass in &mut enc_state.passes {
        pass.ac_tokens.resize_with(frame_dim.num_groups, Vec::new);
    }
    let frame_header: &FrameHeader<'_> = mutable_frame_header;
    compute_var_dct_encoding_data(
        frame_header,
        linear,
        &mut color,
        group_rect,
        enc_modular,
        enc_state,
    )?;
    compute_all_coeff_orders(enc_state, &frame_dim)?;
    if !enc_state.streaming_mode {
        enc_state.shared.num_histograms = 1;
        enc_state.histogram_idx.resize(frame_dim.num_groups, 0);
    }
    tokenize_all_coefficients(frame_header, enc_state)?;

    if cparams.modular_mode {
        return admission(FrameAdmission::FrameProfile);
    }

    if !enc_state.streaming_mode {
        // If checks pass here and CanDoStreamingEncoding returns false
        // a Global MA tree is used.
        if cparams.speed_tier < SpeedTier::Tortoise
            || !cparams.modular_part_is_lossless()
            || cparams.lossy_palette
            || (!(cparams.responsive == 1 && cparams.is_lossless()) && cparams.buffering < 3)
            || !cparams.custom_fixed_tree.is_empty()
        {
            enc_modular.compute_tree()?;
            enc_modular.compute_tokens()?;
        }
        let has_patches = enc_state.shared.image_features.patches.borrow().has_any();
        let has_splines = enc_state.shared.image_features.splines.borrow().has_any();
        mutable_frame_header.update_flag(has_patches, FLAG_PATCHES);
        mutable_frame_header.update_flag(has_splines, FLAG_SPLINES);
    }

    encode_groups(mutable_frame_header, enc_state, enc_modular, group_codes)?;
    if enc_state.streaming_mode {
        let group_index = enc_state.dc_group_index;
        enc_modular.clear_stream_data(&ModularStreamId::var_dct_dc(group_index));
        enc_modular.clear_stream_data(&ModularStreamId::ac_metadata(group_index));
        enc_modular.clear_modular_stream_data();
    }
    Ok(())
}

/// PermuteGroups at its early return; the center-first order (group order
/// option) is outside the admitted profile.
fn permute_groups(
    cparams: &CompressParams<'_>,
    frame_dim: &FrameDimensions,
    num_passes: usize,
) -> FrameResult {
    if !cparams.centerfirst || (num_passes == 1 && frame_dim.num_groups == 1) {
        return Ok(());
    }
    admission(FrameAdmission::FrameProfile)
}

/// EncodeFrameOneShot. The frame bytes reach the output processor through
/// AppendData, as in the source.
fn encode_frame_one_shot(
    cparams: &CompressParams<'static>,
    frame_info: &FrameInfo,
    metadata: &CodecMetadata,
    frame_data: &JxlEncoderChunkedFrameAdapter<'_>,
    output_processor: &mut JxlEncoderOutputProcessorWrapper,
) -> FrameResult {
    let mut enc_state = Box::new(PassesEncoderState::new());
    set_progressive_mode(cparams, &mut enc_state.progressive_splitter);
    let mut frame_header = FrameHeader::new(Some(metadata));
    make_frame_header(
        frame_data.xsize(),
        frame_data.ysize(),
        cparams,
        &enc_state.progressive_splitter,
        frame_info,
        None,
        false,
        &mut frame_header,
    )?;
    let num_passes = enc_state.progressive_splitter.get_num_passes();
    let mut enc_modular = ModularFrameEncoder::create(&frame_header, cparams, false)?;
    let mut group_codes = Vec::new();
    compute_encoding_data(
        cparams,
        frame_info,
        metadata,
        frame_data,
        0,
        0,
        frame_data.xsize(),
        frame_data.ysize(),
        &mut frame_header,
        &mut enc_modular,
        &mut enc_state,
        &mut group_codes,
    )?;

    let mut writer = BitWriter::new();
    let special: Vec<&BitWriter> = enc_state
        .special_frames
        .iter()
        .map(|frame| {
            frame
                .as_deref()
                .expect("source special-frame writer must be non-null")
        })
        .collect();
    writer.append_byte_aligned_writers(&special)?;
    write_frame_header(&mut frame_header, &mut writer, None)?;

    let permutation = Vec::new();
    permute_groups(cparams, &enc_state.shared.frame_dim, num_passes)?;

    let mut group_sizes = Vec::with_capacity(group_codes.len());
    for bw in &group_codes {
        if bw.bits_written() % 8 != 0 {
            return Err(StatusCode::GenericError.into());
        }
        group_sizes.push(bw.bits_written() / 8);
    }
    write_toc_permutation(&permutation, &mut writer, None)?;
    write_toc_sizes(&group_sizes, &mut writer, None)?;

    let refs: Vec<&BitWriter> = group_codes.iter().collect();
    writer.append_byte_aligned_writers(&refs)?;
    let frame_bytes = writer.take_bytes();
    append_data(output_processor, frame_bytes.as_slice())?;
    Ok(())
}

/// ComputePermutationForStreaming.
fn compute_permutation_for_streaming(
    xsize: usize,
    ysize: usize,
    group_size: usize,
    num_passes: usize,
    permutation: &mut Vec<CoeffOrder>,
    dc_group_order: &mut Vec<usize>,
) -> Status {
    // This is only valid in VarDCT mode, otherwise there can be group shift.
    let dc_group_size = group_size * BLOCK_DIM;
    let group_xsize = xsize.div_ceil(group_size);
    let group_ysize = ysize.div_ceil(group_size);
    let dc_group_xsize = xsize.div_ceil(dc_group_size);
    let dc_group_ysize = ysize.div_ceil(dc_group_size);
    let num_groups = group_xsize * group_ysize;
    let num_dc_groups = dc_group_xsize * dc_group_ysize;
    let num_sections = 2 + num_dc_groups + num_passes * num_groups;
    permutation.resize(num_sections, 0);
    let mut new_ix = 0usize;
    // DC Global is first
    permutation[0] = new_ix as CoeffOrder;
    new_ix += 1;
    for dc_y in 0..dc_group_ysize {
        for dc_x in 0..dc_group_xsize {
            let dc_ix = dc_y * dc_group_xsize + dc_x;
            dc_group_order.push(dc_ix);
            permutation[1 + dc_ix] = new_ix as CoeffOrder;
            new_ix += 1;
            let ac_y0 = dc_y * BLOCK_DIM;
            let ac_x0 = dc_x * BLOCK_DIM;
            let ac_y1 = group_ysize.min(ac_y0 + BLOCK_DIM);
            let ac_x1 = group_xsize.min(ac_x0 + BLOCK_DIM);
            for pass in 0..num_passes {
                for ac_y in ac_y0..ac_y1 {
                    for ac_x in ac_x0..ac_x1 {
                        let group_ix = ac_y * group_xsize + ac_x;
                        let old_ix = ac_group_index(pass, group_ix, num_groups, num_dc_groups);
                        permutation[old_ix] = new_ix as CoeffOrder;
                        new_ix += 1;
                    }
                }
            }
        }
    }
    // AC Global is last
    permutation[1 + num_dc_groups] = new_ix as CoeffOrder;
    new_ix += 1;
    if new_ix != num_sections {
        return Err(StatusCode::GenericError);
    }
    Ok(())
}

const GROUP_SIZE_OFFSET: [usize; 4] = [0, 1024, 17408, 4211712];
const TOC_BITS: [usize; 4] = [12, 16, 24, 32];

/// TOCBucket.
fn toc_bucket(group_size: usize) -> usize {
    let mut bucket = 0;
    while bucket < 3 && group_size >= GROUP_SIZE_OFFSET[bucket + 1] {
        bucket += 1;
    }
    bucket
}

/// TOCSize.
fn toc_size(group_sizes: &[usize]) -> usize {
    let toc_bits: usize = group_sizes.iter().map(|&s| TOC_BITS[toc_bucket(s)]).sum();
    toc_bits.div_ceil(8)
}

/// ComputeGroupDataOffset.
fn compute_group_data_offset(
    frame_header_size: usize,
    dc_global_size: usize,
    num_sections: usize,
    min_dc_global_size: &mut usize,
    group_offset: &mut usize,
) -> Status {
    let mut max_toc_bits = (num_sections - 1) * 32;
    let min_toc_bits = (num_sections - 1) * 12;
    let max_padding = (max_toc_bits - min_toc_bits).div_ceil(8);
    *min_dc_global_size = dc_global_size;
    let mut dc_global_bucket = toc_bucket(*min_dc_global_size);
    while toc_bucket(*min_dc_global_size + max_padding) > dc_global_bucket {
        dc_global_bucket = toc_bucket(*min_dc_global_size + max_padding);
        *min_dc_global_size = GROUP_SIZE_OFFSET[dc_global_bucket];
    }
    if toc_bucket(*min_dc_global_size) != dc_global_bucket
        || toc_bucket(*min_dc_global_size + max_padding) != dc_global_bucket
    {
        return Err(StatusCode::GenericError);
    }
    max_toc_bits += TOC_BITS[dc_global_bucket];
    let max_toc_size = max_toc_bits.div_ceil(8);
    *group_offset = frame_header_size + max_toc_size + *min_dc_global_size;
    Ok(())
}

/// ComputeDcGlobalPadding.
fn compute_dc_global_padding(
    group_sizes: &[usize],
    frame_header_size: usize,
    group_data_offset: usize,
    min_dc_global_size: usize,
) -> usize {
    let mut new_group_sizes = group_sizes.to_vec();
    new_group_sizes[0] = min_dc_global_size;
    let toc = toc_size(&new_group_sizes);
    let actual_offset = frame_header_size + toc + group_sizes[0];
    group_data_offset.wrapping_sub(actual_offset)
}

/// RemoveUnusedHistograms.
fn remove_unused_histograms(codes: &mut EntropyEncodingData) {
    let mut remap = vec![-1i32; 256];
    let mut inv_remap: Vec<u8> = Vec::new();
    for context in codes.context_map.iter_mut() {
        let histo_ix = *context;
        if remap[usize::from(histo_ix)] == -1 {
            remap[usize::from(histo_ix)] = inv_remap.len() as i32;
            inv_remap.push(histo_ix);
        }
        *context = remap[usize::from(histo_ix)] as u8;
    }
    let mut new_codes = EntropyEncodingData {
        use_prefix_code: codes.use_prefix_code,
        lz77: codes.lz77.clone(),
        context_map: std::mem::take(&mut codes.context_map),
        ..EntropyEncodingData::default()
    };
    let mut encoding_info: Vec<Option<_>> = std::mem::take(&mut codes.encoding_info)
        .into_iter()
        .map(Some)
        .collect();
    let mut encoded: Vec<Option<_>> = std::mem::take(&mut codes.encoded_histograms)
        .into_iter()
        .map(Some)
        .collect();
    for &histo_idx in &inv_remap {
        let h = usize::from(histo_idx);
        new_codes
            .encoding_info
            .push(encoding_info[h].take().unwrap_or_default());
        new_codes.uint_config.push(codes.uint_config[h]);
        new_codes
            .encoded_histograms
            .push(encoded[h].take().unwrap_or_else(BitWriter::new));
    }
    *codes = new_codes;
}

/// OutputAcGlobal.
fn output_ac_global(
    enc_state: &mut PassesEncoderState<'_>,
    frame_dim: &FrameDimensions,
) -> StatusOr<BitWriter> {
    if frame_dim.num_groups <= 1 {
        return Err(StatusCode::GenericError);
    }
    let mut writer = BitWriter::new();
    {
        let num_histo_bits = ceil_log2_nonzero(frame_dim.num_groups);
        let num_dc_groups = frame_dim.num_dc_groups;
        writer.with_max_bits(
            num_histo_bits + 1,
            LayerType::Ac,
            None,
            |w, _| {
                w.write(1, 1); // default dequant matrices
                w.write(num_histo_bits, num_dc_groups as u64 - 1);
                Ok(())
            },
            false,
        )?;
    }
    for i in 0..enc_state.progressive_splitter.get_num_passes() {
        // Encode coefficient orders.
        let used_orders = enc_state.used_orders[i];
        let mut order_bits = 0;
        can_encode(ORDER_ENC, used_orders, &mut order_bits)?;
        writer.with_max_bits(
            order_bits,
            LayerType::Order,
            None,
            |w, _| u32_coder::write(ORDER_ENC, used_orders, w),
            false,
        )?;
        let size = enc_state.shared.coeff_order_size;
        encode_coeff_orders(
            used_orders as u16,
            &enc_state.shared.coeff_orders[i * size..],
            &mut writer,
            LayerType::Order,
            None,
        )?;
        // Fix up context map and entropy codes to remove any fix histograms
        // that were not selected by clustering.
        remove_unused_histograms(&mut enc_state.passes[i].codes);
        encode_histograms(
            &mut enc_state.passes[i].codes,
            &mut writer,
            LayerType::Ac,
            None,
        )?;
    }
    writer.with_max_bits(
        8,
        LayerType::Ac,
        None,
        |w, _| {
            w.zero_pad_to_byte(); // end of group.
            Ok(())
        },
        false,
    )?;
    Ok(writer)
}

/// EncodeFrameStreaming for output modes 0 (collected sections) and 1
/// (streaming output with the TOC written after the groups); mode 2 is a
/// container mode outside the profile.
#[expect(clippy::needless_range_loop, reason = "Keep the source index loops.")]
fn encode_frame_streaming(
    cparams: &CompressParams<'static>,
    frame_info: &FrameInfo,
    metadata: &CodecMetadata,
    frame_data: &JxlEncoderChunkedFrameAdapter<'_>,
    output_processor: &mut JxlEncoderOutputProcessorWrapper,
) -> FrameResult {
    let output_mode = cparams.output_mode;
    if output_mode == 2 {
        return admission(FrameAdmission::StreamingFrame);
    }
    let mut enc_state = Box::new(PassesEncoderState::new());
    set_progressive_mode(cparams, &mut enc_state.progressive_splitter);
    let mut frame_header = FrameHeader::new(Some(metadata));
    make_frame_header(
        frame_data.xsize(),
        frame_data.ysize(),
        cparams,
        &enc_state.progressive_splitter,
        frame_info,
        None,
        true,
        &mut frame_header,
    )?;
    let num_passes = enc_state.progressive_splitter.get_num_passes();
    let mut enc_modular = ModularFrameEncoder::create(&frame_header, cparams, true)?;
    let mut permutation = Vec::new();
    let mut dc_group_order = Vec::new();
    let full_dim = frame_header.to_frame_dimensions();
    let group_size = full_dim.group_dim;
    compute_permutation_for_streaming(
        frame_data.xsize(),
        frame_data.ysize(),
        group_size,
        num_passes,
        &mut permutation,
        &mut dc_group_order,
    )?;
    enc_state.shared.num_histograms = dc_group_order.len();
    let dc_group_size = group_size * BLOCK_DIM;
    let dc_group_xsize = frame_data.xsize().div_ceil(dc_group_size);
    let mut min_dc_global_size = 0;
    let mut group_data_offset = 0;
    let mut frame_header_bytes: Vec<u8> = Vec::new();
    let mut dc_global_bytes: Vec<u8> = Vec::new();
    let mut group_sizes: Vec<usize> = Vec::new();
    let streaming_output = output_mode == 1;
    let streaming = output_mode >= 1;
    let start_pos = if streaming_output {
        output_processor.current_position()
    } else {
        0
    };
    let total_sections = num_toc_entries(full_dim.num_groups, dc_group_order.len(), num_passes);
    let mut global_group_codes: Vec<Option<BitWriter>> = if streaming {
        Vec::new()
    } else {
        (0..total_sections).map(|_| None).collect()
    };

    // PermuteGroups: no center-first order in the admitted profile.
    let group_order_perm: Vec<CoeffOrder> = Vec::new();
    permute_groups(cparams, &full_dim, num_passes)?;

    for i in 0..dc_group_order.len() {
        let dc_ix = dc_group_order[i];
        let dc_y = dc_ix / dc_group_xsize;
        let dc_x = dc_ix % dc_group_xsize;
        let y0 = dc_y * dc_group_size;
        let x0 = dc_x * dc_group_size;
        let ysize = dc_group_size.min(frame_data.ysize() - y0);
        let xsize = dc_group_size.min(frame_data.xsize() - x0);
        let group_xsize = xsize.div_ceil(group_size);
        let group_ysize = ysize.div_ceil(group_size);
        enc_state.streaming_mode = true;
        enc_state.initialize_global_state = i == 0;
        enc_state.dc_group_index = dc_ix;
        enc_state.histogram_idx = vec![i; group_xsize * group_ysize];
        let mut group_codes = Vec::new();
        compute_encoding_data(
            cparams,
            frame_info,
            metadata,
            frame_data,
            x0,
            y0,
            xsize,
            ysize,
            &mut frame_header,
            &mut enc_modular,
            &mut enc_state,
            &mut group_codes,
        )?;
        if !enc_state.special_frames.is_empty() {
            return Err(StatusCode::GenericError.into());
        }
        if i == 0 {
            let mut writer = BitWriter::new();
            write_frame_header(&mut frame_header, &mut writer, None)?;
            write_toc_permutation(
                if streaming_output {
                    &permutation
                } else {
                    &group_order_perm
                },
                &mut writer,
                None,
            )?;
            frame_header_bytes = writer.take_bytes().as_slice().to_vec();
            if streaming_output {
                dc_global_bytes = std::mem::take(&mut group_codes[0])
                    .take_bytes()
                    .as_slice()
                    .to_vec();
                compute_group_data_offset(
                    frame_header_bytes.len(),
                    dc_global_bytes.len(),
                    permutation.len(),
                    &mut min_dc_global_size,
                    &mut group_data_offset,
                )?;
                group_sizes.push(dc_global_bytes.len());
                output_processor.seek(start_pos + group_data_offset)?;
            }
        }
        if streaming {
            // OutputGroups.
            if group_codes.len() < 4 {
                return Err(StatusCode::GenericError.into());
            }
            let bytes = std::mem::take(&mut group_codes[1]).take_bytes();
            group_sizes.push(bytes.len());
            append_data(output_processor, bytes.as_slice())?;
            for code in group_codes.iter_mut().skip(3) {
                let bytes = std::mem::take(code).take_bytes();
                group_sizes.push(bytes.len());
                append_data(output_processor, bytes.as_slice())?;
            }
        } else {
            if group_codes.len() < 4 {
                return Err(StatusCode::GenericError.into());
            }
            if i == 0 {
                group_codes[0].shrink()?;
                global_group_codes[0] = Some(std::mem::take(&mut group_codes[0]));
            }
            group_codes[1].shrink()?;
            global_group_codes[1 + i] = Some(std::mem::take(&mut group_codes[1]));
            for j in 3..group_codes.len() {
                let mut patch_dim = FrameDimensions::default();
                patch_dim.set(
                    xsize,
                    ysize,
                    frame_header.group_size_shift as usize,
                    0,
                    0,
                    frame_header.encoding == FrameEncoding::MODULAR,
                    1,
                );
                let global_ac_idx =
                    enc_modular.compute_streaming_absolute_ac_group_id(i, j - 3, &patch_dim);
                group_codes[j].shrink()?;
                global_group_codes[2 + dc_group_order.len() + global_ac_idx] =
                    Some(std::mem::take(&mut group_codes[j]));
            }
        }
    }
    let mut writer = output_ac_global(&mut enc_state, &full_dim)?;
    writer.shrink()?;
    if streaming {
        let bytes = writer.take_bytes();
        group_sizes.push(bytes.len());
        append_data(output_processor, bytes.as_slice())?;
    } else {
        global_group_codes[1 + dc_group_order.len()] = Some(writer);
    }
    if streaming_output {
        if group_sizes.len() != permutation.len() {
            return Err(StatusCode::GenericError.into());
        }
        let end_pos = output_processor.current_position();
        output_processor.seek(start_pos)?;
        let padding_size = compute_dc_global_padding(
            &group_sizes,
            frame_header_bytes.len(),
            group_data_offset,
            min_dc_global_size,
        );
        group_sizes[0] += padding_size;
        let mut toc_writer = BitWriter::new();
        write_toc_sizes(&group_sizes, &mut toc_writer, None)?;
        let toc_bytes = toc_writer.take_bytes();
        let padding_bytes = vec![0u8; padding_size];
        append_data(output_processor, &frame_header_bytes)?;
        append_data(output_processor, toc_bytes.as_slice())?;
        append_data(output_processor, &dc_global_bytes)?;
        append_data(output_processor, &padding_bytes)?;
        if output_processor.current_position() != start_pos + group_data_offset {
            return Err(StatusCode::GenericError.into());
        }
        output_processor.seek(end_pos)?;
    } else {
        let mut codes = Vec::with_capacity(global_group_codes.len());
        for g in global_group_codes {
            let g = g.ok_or(StatusCode::GenericError)?;
            group_sizes.push(g.bits_written() / 8);
            codes.push(g);
        }
        if group_sizes.len() != permutation.len() {
            return Err(StatusCode::GenericError.into());
        }
        let mut combined = BitWriter::new();
        write_frame_header(&mut frame_header, &mut combined, None)?;
        write_toc_permutation(&group_order_perm, &mut combined, None)?;
        write_toc_sizes(&group_sizes, &mut combined, None)?;
        append_data(output_processor, combined.take_bytes().as_slice())?;
        for g in codes {
            append_data(output_processor, g.take_bytes().as_slice())?;
        }
    }
    Ok(())
}

/// CanDoStreamingEncoding.
pub fn can_do_streaming_encoding(
    cparams: &CompressParams<'_>,
    frame_info: &FrameInfo,
    metadata: &CodecMetadata,
    frame_data: &JxlEncoderChunkedFrameAdapter<'_>,
) -> bool {
    if cparams.buffering == -1 {
        if cparams.speed_tier < SpeedTier::Tortoise {
            return false;
        }
        if cparams.speed_tier < SpeedTier::Squirrel && cparams.butteraugli_distance > 0.5 {
            return false;
        }
        if cparams.speed_tier == SpeedTier::Squirrel && cparams.butteraugli_distance >= 3.0 {
            return false;
        }
    }
    if cparams.buffering == 0 {
        return false;
    }
    if cparams.buffering == 1 && frame_data.xsize() <= 2048 && frame_data.ysize() <= 2048 {
        return false;
    }
    // Random heuristic: disable streaming for frames with less than 8 groups.
    if num_groups_for_frame(frame_data.xsize(), frame_data.ysize(), cparams) <= 8 {
        return false;
    }
    // The buffer adapter is never JPEG input.
    if cparams.noise == Override::ON || cparams.patches == Override::ON {
        return false;
    }
    if cparams.progressive_dc != 0 || frame_info.dc_level != 0 {
        return false;
    }
    if cparams.custom_progressive_mode.is_some()
        || cparams.qprogressive_mode == Override::ON
        || cparams.progressive_mode == Override::ON
    {
        return false;
    }
    if cparams.resampling != 1 || cparams.ec_resampling != 1 {
        return false;
    }
    if cparams.lossy_palette {
        return false;
    }
    if cparams.max_error_mode {
        return false;
    }
    // Progressive lossless uses Local MA trees, but requires a full
    // buffer to compress well, so no special check.
    if (!cparams.modular_part_is_lossless() || cparams.responsive > 0)
        && (metadata.m.num_extra_channels > 0 || cparams.modular_mode)
    {
        return false;
    }
    let ok_color_transform = if cparams.modular_mode {
        ColorTransform::NONE
    } else {
        ColorTransform::XYB
    };
    if cparams.color_transform != ok_color_transform {
        return false;
    }
    true
}

/// EncodeFrame for the buffer-backed input adapter: the setting resolution,
/// then the streaming or one-shot path. `default_memory_manager` states
/// whether the encoder uses the default manager, which the admitted profile
/// requires: the original routes every image allocation of the frame encoder
/// through the caller's manager.
pub fn encode_frame(
    cparams_orig: &CompressParams<'static>,
    frame_info: &FrameInfo,
    metadata: &CodecMetadata,
    frame_data: &JxlEncoderChunkedFrameAdapter<'_>,
    output_processor: &mut JxlEncoderOutputProcessorWrapper,
    default_memory_manager: bool,
) -> FrameResult {
    let mut cparams = cparams_orig.clone();
    if cparams.speed_tier == SpeedTier::TectonicPlate && !cparams.is_lossless() {
        cparams.speed_tier = SpeedTier::Glacier;
    }
    // Lightning mode is handled externally, so switch to Thunder mode to
    // handle potentially weird cases.
    if cparams.speed_tier == SpeedTier::Lightning {
        cparams.speed_tier = SpeedTier::Thunder;
    }
    if cparams.speed_tier == SpeedTier::TectonicPlate {
        // The trial encodes of the lossless kTectonicPlate search are step 5.
        return admission(FrameAdmission::FrameProfile);
    }

    params_post_init(&mut cparams)?;

    if cparams.butteraugli_distance < 0.0 {
        return Err(StatusCode::GenericError.into());
    }

    if cparams.progressive_dc < 0 {
        if cparams.progressive_dc != -1 {
            return Err(StatusCode::GenericError.into());
        }
        cparams.progressive_dc = 0;
    }
    if cparams.ec_resampling < cparams.resampling {
        cparams.ec_resampling = cparams.resampling;
    }
    if cparams.resampling > 1 || frame_info.is_preview || metadata.m.num_extra_channels > 0 {
        cparams.progressive_dc = 0;
    }

    if frame_info.dc_level + cparams.progressive_dc as usize > 4 {
        return Err(StatusCode::GenericError.into());
    }

    if !cparams.modular_mode && cparams.butteraugli_distance < MIN_BUTTERAUGLI_DISTANCE {
        return Err(StatusCode::GenericError.into());
    }

    // The buffer adapter is never JPEG input.

    if frame_data.xsize() == 0 || frame_data.ysize() == 0 {
        return Err(StatusCode::GenericError.into());
    }

    // Assert that this metadata is correctly set up for the compression
    // params.
    if metadata.m.xyb_encoded != (cparams.color_transform == ColorTransform::XYB) {
        return Err(StatusCode::GenericError.into());
    }

    let streaming = can_do_streaming_encoding(&cparams, frame_info, metadata, frame_data);
    if !admitted_frame_profile(
        &cparams,
        frame_info,
        metadata,
        frame_data,
        default_memory_manager,
    ) {
        return admission(if streaming {
            FrameAdmission::StreamingFrame
        } else {
            FrameAdmission::FrameProfile
        });
    }
    if streaming {
        return encode_frame_streaming(
            &cparams,
            frame_info,
            metadata,
            frame_data,
            output_processor,
        );
    }
    // output_mode 2 (the jxlp gap) is a container mode, outside the profile.
    encode_frame_one_shot(&cparams, frame_info, metadata, frame_data, output_processor)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ar_boundary_fills_sharpness_below_dynamic_ar() {
        let metadata = CodecMetadata::default();
        let header = FrameHeader::new(Some(&metadata));
        let mut state = PassesEncoderState::new();
        state.cparams.butteraugli_distance = 0.4;
        state.shared.epf_sharpness = Plane::create(3, 2).unwrap();
        let orig = Image3F::default();
        assert_eq!(
            compute_ar_heuristics_entry(&header, &mut state, &orig, Rect::new(0, 0, 0, 0)),
            Ok(())
        );
        assert!((0..2).all(|y| state.shared.epf_sharpness.row(y)[..3] == [4, 4, 4]));
    }
}
