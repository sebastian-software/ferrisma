// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! FrameInfo and complete parameter/header preparation from lib/jxl/
//! enc_frame.{h,cc}. Pixel encoding, section orchestration, streaming, and the
//! remaining EncodeFrame entry points are absent until translated. Byte names,
//! narrowing, reused header state, and changes before errors retain source order.

use super::base::float::{max_f32, mul_add_f64};
use super::base::override_option::{Override, apply_override};
use super::base::status::{Status, StatusCode};
use super::common::SpeedTier;
use super::enc_params::{CompressParams, MIN_BUTTERAUGLI_DISTANCE, MIN_BUTTERAUGLI_FOR_NOISE};
use super::enc_progressive_split::{PassDefinition, ProgressiveMode, ProgressiveSplitter};
use super::frame_dimensions::GROUP_DIM;
use super::frame_header::{
    BlendMode, BlendingInfo, ColorTransform, FLAG_NOISE, FLAG_SKIP_ADAPTIVE_DC_SMOOTHING,
    FLAG_USE_DC_FRAME, FrameEncoding, FrameHeader, FrameOrigin, FrameType,
};
use super::image_metadata::ExtraChannel;
use super::jpeg::enc_jpeg_data::{
    set_chroma_subsampling_from_jpeg_data, set_color_transform_from_jpeg_data,
};
use super::jpeg::jpeg_data::JpegData;
use super::types::JxlBitDepth;

// lib/jxl/noise.h: NoiseParams::kNumNoisePoints.
const NUM_NOISE_POINTS: usize = 8;
#[derive(Clone)]
pub struct FrameInfo {
    pub save_before_color_transform: bool,
    pub ib_needs_color_transform: bool,
    pub frame_type: FrameType,
    pub dc_level: usize,
    pub is_last: bool,
    pub is_preview: bool,
    pub save_as_reference: usize,
    pub source: usize,
    pub clamp: bool,
    pub alpha_channel: i32,
    pub origin: FrameOrigin,
    pub blend: bool,
    pub blendmode: BlendMode,
    pub image_bit_depth: JxlBitDepth,
    pub duration: u32,
    pub timecode: u32,
    pub name: Vec<u8>,
    pub extra_channel_blending_info: Vec<BlendingInfo>,
}
impl Default for FrameInfo {
    fn default() -> Self {
        Self {
            save_before_color_transform: false,
            ib_needs_color_transform: true,
            frame_type: FrameType::REGULAR,
            dc_level: 0,
            is_last: true,
            is_preview: false,
            save_as_reference: 0,
            source: 1,
            clamp: true,
            alpha_channel: -1,
            origin: FrameOrigin::default(),
            blend: false,
            blendmode: BlendMode::BLEND,
            image_bit_depth: JxlBitDepth::default(),
            duration: 0,
            timecode: 0,
            name: Vec::new(),
            extra_channel_blending_info: Vec::new(),
        }
    }
}
pub fn params_post_init(p: &mut CompressParams<'_>) -> Status {
    if !p.manual_noise.is_empty() && p.manual_noise.len() != NUM_NOISE_POINTS {
        return Err(StatusCode::GenericError);
    }
    if !p.manual_xyb_factors.is_empty() && p.manual_xyb_factors.len() != 3 {
        return Err(StatusCode::GenericError);
    }
    if !p.modular_mode && p.butteraugli_distance < MIN_BUTTERAUGLI_DISTANCE {
        p.butteraugli_distance = MIN_BUTTERAUGLI_DISTANCE;
    }
    if p.original_butteraugli_distance == -1.0 {
        p.original_butteraugli_distance = p.butteraugli_distance;
    }
    if p.resampling <= 0 {
        p.resampling = 1;
        if !p.already_downsampled && p.butteraugli_distance >= 10.0 {
            p.resampling = 2;
            p.butteraugli_distance =
                mul_add_f64(f64::from(p.butteraugli_distance), 0.25, 0.25) as f32;
        }
    }
    if p.ec_resampling <= 0 {
        p.ec_resampling = p.resampling;
    }
    if p.progressive_mode == Override::ON || p.qprogressive_mode == Override::ON {
        p.responsive = 1;
        if p.is_lossless() {
            p.qprogressive_mode = Override::ON;
        }
    }
    Ok(())
}
pub fn get_group_size_shift(xsize: usize, ysize: usize, cparams: &CompressParams<'_>) -> u32 {
    if !cparams.modular_mode {
        return 1;
    }
    if cparams.modular_group_size_shift >= 0 {
        return cparams.modular_group_size_shift as u32;
    }
    if cparams.decoding_speed_tier >= 2
        || (cparams.decoding_speed_tier >= 1 && cparams.responsive == 1 && cparams.is_lossless())
    {
        return 0;
    }
    if xsize <= 128 && ysize <= 128 {
        return 0;
    }
    if xsize <= 256 && ysize <= 256 {
        return 1;
    }
    if xsize <= 400 && ysize <= 400 {
        return 2;
    }
    1
}
pub fn num_groups_for_frame(xsize: usize, ysize: usize, cparams: &CompressParams<'_>) -> usize {
    let shift = get_group_size_shift(xsize, ysize, cparams) as usize;
    let dim = (GROUP_DIM >> 1) << shift;
    (xsize.wrapping_add(dim - 1) / dim).wrapping_mul(ysize.wrapping_add(dim - 1) / dim)
}
pub fn set_progressive_mode(cparams: &CompressParams<'_>, splitter: &mut ProgressiveSplitter) {
    const PASSES: [PassDefinition; 3] = [
        PassDefinition {
            num_coefficients: 2,
            shift: 0,
            suitable_for_downsampling_of_at_least: 4,
        },
        PassDefinition {
            num_coefficients: 3,
            shift: 0,
            suitable_for_downsampling_of_at_least: 2,
        },
        PassDefinition {
            num_coefficients: 8,
            shift: 0,
            suitable_for_downsampling_of_at_least: 0,
        },
    ];
    const QUANTIZED: [PassDefinition; 2] = [
        PassDefinition {
            num_coefficients: 8,
            shift: 1,
            suitable_for_downsampling_of_at_least: 2,
        },
        PassDefinition {
            num_coefficients: 8,
            shift: 0,
            suitable_for_downsampling_of_at_least: 0,
        },
    ];
    let progressive = apply_override(cparams.progressive_mode, false);
    let quantized = apply_override(cparams.qprogressive_mode, false);
    if let Some(mode) = cparams.custom_progressive_mode {
        splitter.set_progressive_mode(*mode);
    } else if quantized {
        splitter.set_progressive_mode(ProgressiveMode::new(&QUANTIZED));
    } else if progressive {
        splitter.set_progressive_mode(ProgressiveMode::new(&PASSES));
    }
}
pub fn frame_flags_from_params(cparams: &CompressParams<'_>) -> u64 {
    let mut flags = 0;
    if apply_override(
        cparams.noise,
        cparams.butteraugli_distance >= MIN_BUTTERAUGLI_FOR_NOISE,
    ) || cparams.photon_noise_iso > 0.0
        || cparams.manual_noise.len() == NUM_NOISE_POINTS
    {
        flags |= FLAG_NOISE;
    }
    if cparams.progressive_dc > 0 && !cparams.modular_mode {
        flags |= FLAG_USE_DC_FRAME;
    }
    flags
}
pub fn loop_filter_from_params(
    cparams: &CompressParams<'_>,
    _streaming_mode: bool,
    header: &mut FrameHeader<'_>,
) -> Status {
    let filter = &mut header.loop_filter;
    filter.gab = apply_override(
        cparams.gaborish,
        cparams.speed_tier <= SpeedTier::Hare
            && header.encoding == FrameEncoding::VAR_DCT
            && cparams.decoding_speed_tier < 4
            && cparams.butteraugli_distance > 0.5
            && !cparams.disable_perceptual_optimizations,
    );
    if cparams.epf != -1 {
        filter.epf_iters = cparams.epf as u32;
    } else if cparams.disable_perceptual_optimizations {
        filter.epf_iters = 0;
        return Ok(());
    } else if header.encoding == FrameEncoding::MODULAR {
        filter.epf_iters = 0;
    } else {
        const THRESHOLDS: [f32; 3] = [0.7, 1.5, 4.0];
        filter.epf_iters = 0;
        if cparams.decoding_speed_tier < 3 {
            for &threshold in THRESHOLDS
                .iter()
                .skip(usize::from(cparams.decoding_speed_tier == 2))
            {
                if cparams.butteraugli_distance >= threshold {
                    filter.epf_iters += 1;
                }
            }
        }
    }
    if header.encoding == FrameEncoding::MODULAR && !cparams.is_lossless() {
        filter.epf_sigma_for_modular = max_f32(cparams.butteraugli_distance, 1.0);
    }
    if header.encoding == FrameEncoding::MODULAR && cparams.lossy_palette {
        filter.epf_sigma_for_modular = 1.0;
    }
    Ok(())
}
#[expect(
    clippy::too_many_arguments,
    reason = "Preserve source frame/header preparation inputs"
)]
pub fn make_frame_header(
    xsize: usize,
    ysize: usize,
    cparams: &CompressParams<'_>,
    splitter: &ProgressiveSplitter,
    info: &FrameInfo,
    jpeg_data: Option<&JpegData>,
    streaming: bool,
    header: &mut FrameHeader<'_>,
) -> Status {
    header.nonserialized_is_preview = info.is_preview;
    header.is_last = info.is_last;
    header.save_before_color_transform = info.save_before_color_transform;
    header.frame_type = info.frame_type;
    header.name = info.name.clone();
    splitter.init_passes(&mut header.passes)?;
    if cparams.modular_mode {
        header.encoding = FrameEncoding::MODULAR;
        header.group_size_shift = get_group_size_shift(xsize, ysize, cparams);
    }
    if let Some(jpg) = jpeg_data {
        header.encoding = FrameEncoding::VAR_DCT;
        header.x_qm_scale = 2;
        header.b_qm_scale = 2;
        set_chroma_subsampling_from_jpeg_data(jpg, &mut header.chroma_subsampling)?;
        set_color_transform_from_jpeg_data(jpg, &mut header.color_transform)?;
    } else {
        header.color_transform = cparams.color_transform;
        if !cparams.modular_mode
            && (header.chroma_subsampling.max_h_shift() != 0
                || header.chroma_subsampling.max_v_shift() != 0)
        {
            return Err(StatusCode::GenericError);
        }
    }
    if header.color_transform != ColorTransform::YCBCR
        && (header.chroma_subsampling.max_h_shift() != 0
            || header.chroma_subsampling.max_v_shift() != 0)
    {
        return Err(StatusCode::GenericError);
    }
    header.flags = frame_flags_from_params(cparams);
    if header.encoding != FrameEncoding::VAR_DCT
        && cparams.photon_noise_iso == 0.0
        && cparams.manual_noise.is_empty()
    {
        header.update_flag(false, FLAG_NOISE);
    }
    loop_filter_from_params(cparams, streaming, header)?;
    header.dc_level = info.dc_level as u32;
    if header.dc_level > 2 {
        return Err(StatusCode::GenericError);
    }
    if cparams.progressive_dc > 0 && (cparams.ec_resampling != 1 || cparams.resampling != 1) {
        return Err(StatusCode::GenericError);
    }
    if ![1, 2, 4, 8].contains(&cparams.resampling) {
        return Err(StatusCode::GenericError);
    }
    if ![1, 2, 4, 8].contains(&cparams.ec_resampling) {
        return Err(StatusCode::GenericError);
    }
    if info.frame_type != FrameType::DC {
        header.frame_origin = info.origin;
        let ups = if cparams.already_downsampled {
            cparams.resampling as usize
        } else {
            1
        };
        header.frame_size.xsize = xsize.wrapping_mul(ups) as u32;
        header.frame_size.ysize = ysize.wrapping_mul(ups) as u32;
        if info.origin.x0 != 0
            || info.origin.y0 != 0
            || header.frame_size.xsize as usize != header.default_xsize()
            || header.frame_size.ysize as usize != header.default_ysize()
        {
            header.custom_size_or_origin = true;
        }
    }
    header.upsampling = cparams.resampling as u32;
    let extra = &header
        .nonserialized_metadata
        .expect("MakeFrameHeader requires metadata")
        .m
        .extra_channel_info;
    header.extra_channel_upsampling.clear();
    header
        .extra_channel_upsampling
        .resize(extra.len(), cparams.ec_resampling as u32);
    header.save_as_reference = info.save_as_reference as u32;
    if info.blend || header.custom_size_or_origin {
        let mut index = 0;
        if info.alpha_channel == -1 {
            if extra.len() > 1 {
                for (i, channel) in extra.iter().enumerate() {
                    if channel.channel_type == ExtraChannel::ALPHA {
                        index = i;
                        break;
                    }
                }
            }
        } else {
            index = info.alpha_channel as usize;
            if index != 0 && index >= extra.len() {
                return Err(StatusCode::GenericError);
            }
        }
        header.blending_info.alpha_channel = index as u32;
        header.blending_info.mode = if info.blend {
            info.blendmode
        } else {
            BlendMode::REPLACE
        };
        header.blending_info.source = info.source as u32;
        header.blending_info.clamp = info.clamp;
        for (i, channel) in extra.iter().enumerate() {
            if i < info.extra_channel_blending_info.len() {
                header.extra_channel_blending_info[i] = info.extra_channel_blending_info[i].clone();
            } else {
                let blend = &mut header.extra_channel_blending_info[i];
                blend.alpha_channel = index as u32;
                let mut default_blend = info.blendmode;
                if channel.channel_type != ExtraChannel::BLACK && i != index {
                    default_blend = BlendMode::ADD;
                }
                blend.mode = if info.blend {
                    default_blend
                } else {
                    BlendMode::REPLACE
                };
                blend.source = 1;
            }
        }
    }
    header.animation_frame.duration = info.duration;
    header.animation_frame.timecode = info.timecode;
    if jpeg_data.is_some() {
        header.update_flag(false, FLAG_USE_DC_FRAME);
        if !((cparams.force_lfs_jpeg_recompression == -1 && header.chroma_subsampling.is_444())
            || cparams.force_lfs_jpeg_recompression == 1)
        {
            header.update_flag(true, FLAG_SKIP_ADAPTIVE_DC_SMOOTHING);
        }
    }
    Ok(())
}
