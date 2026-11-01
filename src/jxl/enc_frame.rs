// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! FrameInfo and complete parameter/header preparation from lib/jxl/
//! enc_frame.{h,cc}, plus the fixed lossless Thunder ComputeEncodingData and
//! EncodeFrameOneShot paths. The reached encode.cc global-header block connects
//! pixel buffers to a complete raw codestream with native byte ownership.
//! General image/frame encoding, streaming, and C output processors remain
//! absent. Byte names, narrowing, reused header state, and changes before errors
//! retain source order.

use super::ac_strategy::AcStrategyImage;
use super::base::float::{max_f32, mul_add_f64};
use super::base::override_option::{Override, apply_override};
use super::base::rect::Rect;
use super::base::status::{Status, StatusCode, StatusOr};
use super::chroma_from_luma::ColorCorrelationMap;
use super::coeff_order::COEFF_ORDER_MAX_SIZE;
use super::color_encoding::{ColorSpace, RenderingIntent};
use super::common::SpeedTier;
use super::dec_modular::ModularStreamId;
use super::enc_aux_out::{AuxOut, LayerType};
use super::enc_bit_writer::BitWriter;
use super::enc_cache::PassesEncoderState;
use super::enc_external_image::convert_from_external_plane_no_size_check;
use super::enc_fields::{write_codestream_headers, write_frame_header};
use super::enc_modular::ModularFrameEncoder;
use super::enc_params::{CompressParams, MIN_BUTTERAUGLI_DISTANCE, MIN_BUTTERAUGLI_FOR_NOISE};
use super::enc_progressive_split::{PassDefinition, ProgressiveMode, ProgressiveSplitter};
use super::enc_quant_weights::dequant_matrices_encode_dc;
use super::enc_toc::{write_toc_permutation, write_toc_sizes};
use super::encode_internal::{JxlEncoderChunkedFrameAdapter, bits_per_channel};
use super::frame_dimensions::GROUP_DIM;
use super::frame_header::{
    BlendMode, BlendingInfo, ColorTransform, FLAG_NOISE, FLAG_SKIP_ADAPTIVE_DC_SMOOTHING,
    FLAG_USE_DC_FRAME, FrameEncoding, FrameHeader, FrameOrigin, FrameType,
};
use super::image::{Image3, Plane};
use super::image_metadata::{CodecMetadata, ExtraChannel};
use super::image_ops::{copy_image_to, pad_image_to_block_multiple_in_place};
use super::jpeg::enc_jpeg_data::{
    set_chroma_subsampling_from_jpeg_data, set_color_transform_from_jpeg_data,
};
use super::jpeg::jpeg_data::JpegData;
use super::padded_bytes::PaddedBytes;
use super::passes_state::{DcSource, PassesSharedState};
use super::quant_weights::DequantMatrices;
use super::quantizer::Quantizer;
use super::toc::ac_group_index;
use super::types::{JxlBitDepth, JxlDataType, JxlEndianness, JxlPixelFormat};

// lib/jxl/noise.h: NoiseParams::kNumNoisePoints.
const NUM_NOISE_POINTS: usize = 8;

/// Integer metadata configurations for the first original image path.
#[derive(Clone, Copy)]
pub enum LosslessSampleDepth {
    Uint8,
    Uint16,
}

/// Fixed source configuration: SetLossless, Thunder, level 10, buffering 0,
/// one regular final grayscale/RGB frame, and all remaining parameter defaults.
/// Private metadata prevents selecting unported image paths through this API.
/// This is codec-internal configuration, not the encoder C API.
pub struct LosslessThunderConfig {
    metadata: CodecMetadata,
}
impl LosslessThunderConfig {
    /// Transfer parsed metadata only after admission to the initial source
    /// profile. Failure retains the caller's owner and reports no codec status.
    pub(crate) fn admit_decoded_metadata(owner: &mut Option<CodecMetadata>) -> Option<Self> {
        let metadata = owner.as_ref().expect("live parsed metadata owner");
        let m = &metadata.m;
        if m.bit_depth.floating_point_sample
            || ![8, 16].contains(&m.bit_depth.bits_per_sample)
            || m.xyb_encoded
            || m.num_extra_channels != 0
            || !m.extra_channel_info.is_empty()
            || m.have_animation
            || m.have_preview
            || m.have_intrinsic_size
            || m.orientation != 1
            || m.color_encoding.want_icc()
            || !m.color_encoding.is_srgb()
            || m.color_encoding.to_external().rendering_intent != RenderingIntent::RELATIVE
            || m.tone_mapping.intensity_target != 255.0
            || m.tone_mapping.min_nits != 0.0
            || m.tone_mapping.relative_to_max_display
            || m.tone_mapping.linear_below != 0.0
        {
            return None;
        }
        Some(Self {
            metadata: owner.take().unwrap(),
        })
    }
    pub(crate) fn into_metadata(self) -> CodecMetadata {
        self.metadata
    }
    pub fn new(
        xsize: usize,
        ysize: usize,
        gray: bool,
        depth: LosslessSampleDepth,
    ) -> StatusOr<Self> {
        let mut metadata = CodecMetadata::default();
        metadata.size.set(xsize, ysize)?;
        metadata.m.set_uint_samples(match depth {
            LosslessSampleDepth::Uint8 => 8,
            LosslessSampleDepth::Uint16 => 16,
        });
        metadata.m.xyb_encoded = false;
        metadata.m.color_encoding.set_srgb(
            if gray {
                ColorSpace::GRAY
            } else {
                ColorSpace::RGB
            },
            RenderingIntent::RELATIVE,
        )?;
        Ok(Self { metadata })
    }
    pub fn metadata(&self) -> &CodecMetadata {
        &self.metadata
    }
    /// Fixed-profile raw codestream output. The metadata/ICC padding block is
    /// from encode.cc; the frame is the original one-shot path below. The
    /// native return transfers its padded byte owner instead of a C callback.
    pub fn encode_pixels(
        &mut self,
        data: &[u8],
        mut aux: Option<&mut AuxOut>,
    ) -> StatusOr<PaddedBytes> {
        let mut input =
            JxlEncoderChunkedFrameAdapter::new(self.metadata.xsize(), self.metadata.ysize(), 0);
        let format = JxlPixelFormat {
            num_channels: self.metadata.m.color_encoding.channels() as u32,
            data_type: if self.metadata.m.bit_depth.bits_per_sample == 8 {
                JxlDataType::UINT8
            } else {
                JxlDataType::UINT16
            },
            endianness: JxlEndianness::NATIVE,
            align: 0,
        };
        if !input.set_from_buffer(0, Some(data), data.len(), format) {
            return Err(StatusCode::GenericError);
        }
        let mut writer = BitWriter::new();
        write_codestream_headers(&mut self.metadata, &mut writer, aux.as_deref_mut())?;
        // This private sRGB metadata has fields, so WantICC is false; no preview.
        debug_assert!(!self.metadata.m.color_encoding.want_icc());
        writer.with_max_bits(
            8,
            LayerType::Header,
            aux.as_deref_mut(),
            |w, _| {
                w.zero_pad_to_byte();
                Ok(())
            },
            false,
        )?;
        let frame_bytes = {
            let mut frame = self.create_frame()?;
            frame.encode_one_shot(&input, aux)?
        };
        writer.append_byte_aligned(frame_bytes.as_slice())?;
        Ok(writer.take_bytes())
    }
    /// CopyColorChannels with the fixed FrameInfo and no extra/alpha planes.
    /// The adapter is the original buffer-backed input branch; its release
    /// callback is a no-op and is represented by the scoped input borrow.
    pub fn copy_color_channels(
        &self,
        input: &JxlEncoderChunkedFrameAdapter<'_>,
        rect: Rect,
        color: &mut Image3<f32>,
        has_interleaved_alpha: &mut bool,
    ) -> Status {
        let mut format = input.get_color_channels_pixel_format();
        format.align = 0;
        *has_interleaved_alpha = format.num_channels == 2 || format.num_channels == 4;
        let bits_per_sample = bits_per_channel(format.data_type);
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
        let color_channels = self.metadata.m.color_encoding.channels();
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
        Ok(())
    }
    /// The reached setup in EncodeFrameOneShot, before pixel preparation.
    pub fn create_frame(&self) -> StatusOr<LosslessThunderFrame<'_>> {
        let mut cparams = CompressParams {
            speed_tier: SpeedTier::Thunder,
            level: 10,
            buffering: 0,
            ..CompressParams::default()
        };
        cparams.set_lossless();
        params_post_init(&mut cparams)?;
        let mut state = Box::new(PassesEncoderState::new());
        set_progressive_mode(&cparams, &mut state.progressive_splitter);
        let mut header = FrameHeader::new(Some(&self.metadata));
        make_frame_header(
            self.metadata.xsize(),
            self.metadata.ysize(),
            &cparams,
            &state.progressive_splitter,
            &FrameInfo::default(),
            None,
            false,
            &mut header,
        )?;
        // Component callers supply these header-derived dimensions. The outer
        // computation assigns them again at its original checkpoint.
        state.shared.frame_dim = header.to_frame_dimensions();
        let modular = ModularFrameEncoder::create(&header, &cparams, false)?;
        Ok(LosslessThunderFrame {
            config: self,
            header,
            modular,
            state,
            cparams,
        })
    }
}

/// Source frame owners for the fixed configuration. Metadata is borrowed from
/// the configuration; header/parameters cannot be mutated into unported paths.
/// This owner exposes the reached one-shot and component boundaries; general
/// frame/image orchestration remains unported.
pub struct LosslessThunderFrame<'a> {
    // The modular owner releases before its borrowed setup, as in the source.
    modular: ModularFrameEncoder<'static>,
    header: FrameHeader<'a>,
    state: Box<PassesEncoderState<'a>>,
    cparams: CompressParams<'static>,
    config: &'a LosslessThunderConfig,
}
impl LosslessThunderFrame<'_> {
    pub fn header(&self) -> &FrameHeader<'_> {
        &self.header
    }
    pub fn modular(&self) -> &ModularFrameEncoder<'static> {
        &self.modular
    }
    pub fn matrices(&self) -> &DequantMatrices {
        &self.state.shared.matrices
    }
    pub fn quantizer(&self) -> &Quantizer {
        &self.state.shared.quantizer
    }
    pub fn fields(&self) -> &PassesSharedState<'_> {
        &self.state.shared
    }
    pub fn state(&self) -> &PassesEncoderState<'_> {
        &self.state
    }
    /// ComputeEncodingData's metadata/dimensions/patch binding and reached
    /// allocation block for the fixed non-streaming modular configuration.
    pub fn allocate_frame_fields(&mut self) -> Status {
        self.state.shared.metadata = Some(&self.config.metadata);
        self.state.shared.frame_dim = self.header.to_frame_dimensions();
        self.state.shared.bind_own_reference_frames();
        let dim = &self.state.shared.frame_dim;
        self.state.shared.ac_strategy =
            AcStrategyImage::create(dim.xsize_blocks, dim.ysize_blocks)?;
        self.state.shared.raw_quant_field = Plane::create(dim.xsize_blocks, dim.ysize_blocks)?;
        self.state.shared.epf_sharpness = Plane::create(dim.xsize_blocks, dim.ysize_blocks)?;
        // The original call retains Create's default XYB=true, even here.
        self.state.shared.cmap = ColorCorrelationMap::create(dim.xsize, dim.ysize, true)?;
        self.state.shared.coeff_order_size = COEFF_ORDER_MAX_SIZE;
        self.state.shared.quant_dc = Plane::create(dim.xsize_blocks, dim.ysize_blocks)?;
        self.state.shared.dc_storage = Image3::create(dim.xsize_blocks, dim.ysize_blocks)?;
        self.state.shared.dc_source = DcSource::Storage;
        Ok(())
    }
    /// Complete reached ComputeEncodingData branches for this private profile.
    /// The buffer adapter cannot select JPEG/callback input; metadata/parameters
    /// cannot enable VarDCT, XYB, extra channels, effects, or streaming branches.
    pub fn compute_encoding_data(
        &mut self,
        input: &JxlEncoderChunkedFrameAdapter<'_>,
        codes: &mut Vec<BitWriter>,
        aux: Option<&mut AuxOut>,
    ) -> Status {
        let xsize = self.config.metadata.xsize();
        let ysize = self.config.metadata.ysize();
        if xsize > input.xsize() {
            return Err(StatusCode::GenericError);
        }
        if ysize > input.ysize() {
            return Err(StatusCode::GenericError);
        }
        self.allocate_frame_fields()?;
        // Find(alpha/black) yields no entries; optional indices replace unused
        // source null-pointer differences. Retain the reached color copy owner.
        let _c_enc = self.config.metadata.m.color_encoding.clone();
        let frame_rect = Rect::new(0, 0, input.xsize(), input.ysize());
        let frame_area_rect = Rect::new(0, 0, xsize, ysize);
        let patch_rect = frame_area_rect.extend(0, frame_rect);
        if !patch_rect.is_inside(frame_rect) {
            return Err(StatusCode::GenericError);
        }
        let round_block = |n: usize| n.wrapping_add(7) & !7;
        let mut color = Image3::<f32>::create(
            round_block(patch_rect.xsize()),
            round_block(patch_rect.ysize()),
        )?;
        color.shrink_to(patch_rect.xsize(), patch_rect.ysize())?;
        let _extra_channels: Vec<Plane<f32>> = Vec::new();
        let mut has_interleaved_alpha = false;
        self.config.copy_color_channels(
            input,
            patch_rect,
            &mut color,
            &mut has_interleaved_alpha,
        )?;
        // CopyExtraChannels iterates the private metadata's empty vector.
        self.state.cparams = self.cparams.clone();
        let _linear_storage = Image3::<f32>::default();
        // ToXYB and invisible-alpha branches are not reached: NONE and no alpha.
        pad_image_to_block_multiple_in_place(&mut color, 8)?;
        let _group_rect = Rect::new(
            0usize.wrapping_sub(patch_rect.x0()),
            0usize.wrapping_sub(patch_rect.y0()),
            round_block(xsize),
            round_block(ysize),
        );
        // ComputeChromacityAdjustments returns at its non-VarDCT guard. All
        // ComputeNoiseParams branches are false (regular frame, ISO 0, empty
        // manual noise, modular encoding). DownsampleColorChannels returns at
        // upsampling==1; ec_resampling==1 skips the extra-channel loop.
        // Non-streaming group_rect becomes Rect(color) in prepare_modular.
        self.prepare_modular(&color)?;
        self.compute_modular_tokens()?;
        self.header.update_flag(
            self.state.shared.image_features.patches.has_any(),
            super::frame_header::FLAG_PATCHES,
        );
        self.header.update_flag(
            self.state.shared.image_features.splines.has_any(),
            super::frame_header::FLAG_SPLINES,
        );
        self.encode_groups(codes, aux)
    }
    /// EncodeFrameOneShot through TakeBytes. The native core returns the owner
    /// at AppendData's input boundary; the C output-processor API is separate.
    pub fn encode_one_shot(
        &mut self,
        input: &JxlEncoderChunkedFrameAdapter<'_>,
        mut aux: Option<&mut AuxOut>,
    ) -> StatusOr<PaddedBytes> {
        let mut codes = Vec::new();
        self.compute_encoding_data(input, &mut codes, aux.as_deref_mut())?;
        let writer = self.finalize_groups(&codes, aux)?;
        Ok(writer.take_bytes())
    }
    /// Translate ModularFrameEncoder::ComputeEncodingData at the original
    /// full-image, non-streaming call site. Input is already in Image3F form;
    /// callers may also reach it through the complete fixed-profile entry point.
    pub fn prepare_modular(&mut self, color: &Image3<f32>) -> Status {
        self.modular.compute_encoding_data_lossless_thunder(
            self.config,
            &self.header,
            &self.state.progressive_splitter,
            color,
        )
    }
    /// ComputeEncodingData's reached shared-tree/token branch for this profile.
    /// Caller lifecycle: prepare pixels, compute tokens, encode groups once.
    pub fn compute_modular_tokens(&mut self) -> Status {
        self.modular.compute_tree()?;
        self.modular.compute_tokens()
    }
    /// EncodeGroups for this fixed non-streaming modular configuration. Its
    /// private header has no patch/spline/noise data and global state is always
    /// initialized. VarDCT/effect/streaming branches cannot be selected here.
    pub fn encode_groups(
        &mut self,
        group_codes: &mut Vec<BitWriter>,
        mut aux: Option<&mut AuxOut>,
    ) -> Status {
        let frame_dim = &self.state.shared.frame_dim;
        let num_groups = frame_dim.num_groups;
        let num_passes = self.state.progressive_splitter.get_num_passes();
        let is_small_image = num_groups == 1 && num_passes == 1;
        let num_toc_entries = if is_small_image {
            1
        } else {
            ac_group_index(0, 0, num_groups, frame_dim.num_dc_groups)
                .wrapping_add(num_groups.wrapping_mul(num_passes))
        };
        if !group_codes.is_empty() {
            return Err(StatusCode::GenericError);
        }
        group_codes.reserve(num_toc_entries);
        for _ in 0..num_toc_entries {
            group_codes.push(BitWriter::new());
        }
        let output_index = |index| if is_small_image { 0 } else { index };
        dequant_matrices_encode_dc(
            &self.state.shared.matrices,
            &mut group_codes[0],
            LayerType::Quant,
            aux.as_deref_mut(),
        )?;
        self.modular
            .encode_global_info(false, &mut group_codes[0], aux.as_deref_mut())?;
        self.modular.encode_stream(
            &mut group_codes[0],
            aux.as_deref_mut(),
            LayerType::ModularGlobal,
            &ModularStreamId::global(),
        )?;

        let mut aux_outs = Vec::new();
        // RunOnPool's null runner initializes one worker for a nonempty range.
        // Its public task bounds narrow to uint32_t before scheduling.
        let dc_end = frame_dim.num_dc_groups as u32;
        if dc_end != 0 {
            resize_group_aux_outs(&mut aux_outs, 1, aux.as_deref_mut())?;
            for group in 0..dc_end {
                self.modular
                    .encode_stream(
                        &mut group_codes[output_index(group as usize + 1)],
                        aux_outs[0].as_deref_mut(),
                        LayerType::ModularDcGroup,
                        &ModularStreamId::modular_dc(group as usize),
                    )
                    .map_err(|_| StatusCode::GenericError)?;
            }
        }
        let ac_end = num_groups as u32;
        if ac_end != 0 {
            resize_group_aux_outs(&mut aux_outs, 1, aux.as_deref_mut())?;
            for group in 0..ac_end {
                for pass in 0..num_passes {
                    let output = output_index(ac_group_index(
                        pass,
                        group as usize,
                        num_groups,
                        frame_dim.num_dc_groups,
                    ));
                    self.modular
                        .encode_stream(
                            &mut group_codes[output],
                            aux_outs[0].as_deref_mut(),
                            LayerType::ModularAcGroup,
                            &ModularStreamId::modular_ac(group as usize, pass),
                        )
                        .map_err(|_| StatusCode::GenericError)?;
                }
            }
        }
        let _ = resize_group_aux_outs(&mut aux_outs, 0, aux.as_deref_mut());
        for writer in group_codes {
            writer.with_max_bits(
                8,
                LayerType::Ac,
                aux.as_deref_mut(),
                |w, _| {
                    w.zero_pad_to_byte();
                    Ok(())
                },
                false,
            )?;
        }
        Ok(())
    }
    /// EncodeFrameOneShot's tail from the local writer through the final group
    /// append. This boundary returns that writer before TakeBytes/AppendData;
    /// it does not implement the outer pixel input or image entry point.
    pub fn finalize_groups(
        &mut self,
        group_codes: &[BitWriter],
        mut aux: Option<&mut AuxOut>,
    ) -> StatusOr<BitWriter> {
        let mut writer = BitWriter::new();
        let special: Vec<_> = self
            .state
            .special_frames
            .iter()
            .map(|frame| {
                frame
                    .as_deref()
                    .expect("source special-frame writer must be non-null")
            })
            .collect();
        writer.append_byte_aligned_writers(&special)?;
        write_frame_header(&mut self.header, &mut writer, aux.as_deref_mut())?;

        // PermuteGroups returns at its first !centerfirst guard for this private
        // configuration. No center-first permutation entry point is exposed.
        debug_assert!(!self.cparams.centerfirst);
        let permutation = Vec::new();
        let mut group_sizes = Vec::with_capacity(group_codes.len());
        for bw in group_codes {
            if bw.bits_written() % 8 != 0 {
                return Err(StatusCode::GenericError);
            }
            group_sizes.push(bw.bits_written() / 8);
        }
        write_toc_permutation(&permutation, &mut writer, aux.as_deref_mut())?;
        write_toc_sizes(&group_sizes, &mut writer, aux)?;
        let refs: Vec<_> = group_codes.iter().collect();
        writer.append_byte_aligned_writers(&refs)?;
        Ok(writer)
    }
}

// Parameters reify the source lambda captures into sequential disjoint borrows.
fn resize_group_aux_outs(
    aux_outs: &mut Vec<Option<Box<AuxOut>>>,
    num_threads: usize,
    aux: Option<&mut AuxOut>,
) -> Status {
    if let Some(aux) = aux {
        while aux_outs.len() > num_threads {
            aux.assimilate(aux_outs.last().unwrap().as_deref().unwrap());
            aux_outs.pop();
        }
        while num_threads > aux_outs.len() {
            aux_outs.push(Some(Box::new(AuxOut::default())));
        }
    } else {
        aux_outs.resize_with(num_threads, || None);
    }
    Ok(())
}
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
