// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! DecodeFrame/FrameDecoder from dec_frame.{h,cc}, with closed lossless Thunder
//! and first VarDCT raw-codestream profiles. The original simple pipeline is
//! explicitly selected. Private metadata/header admission excludes unported
//! configurations before output initialization; it is not a general decoder
//! status replacement. Admitted paths retain original section scheduling,
//! DC/AC setup, scalar reconstruction, output and reader-close checkpoints.
use super::base::rect::Rect;
use super::base::status::{Status, StatusCode};
use super::dec_bit_reader::BitReader;
use super::dec_cache::{GroupDecCache, PassesDecoderState};
use super::dec_modular::{ModularFrameDecoder, ModularStreamId};
use super::enc_frame::LosslessThunderConfig;
use super::frame_dimensions::FrameDimensions;
use super::frame_header::{
    BlendMode, ColorTransform, FLAG_SKIP_ADAPTIVE_DC_SMOOTHING, FrameEncoding, FrameHeader,
    FrameType, read_frame_header,
};
use super::image::Image3;
use super::image_bundle::ImageBundle;
use super::passes_state::initialize_passes_shared_state;
use super::quant_weights::DequantMatrices;
use super::render_pipeline::render_pipeline_stage::WriteToImageBundleStage;
use super::render_pipeline::simple_render_pipeline::SimpleRenderPipelineBuilder;
use super::render_pipeline::stage_cms::get_cms_stage;
use super::render_pipeline::stage_tone_mapping::get_tone_mapping_stage;
use super::toc::{num_toc_entries, read_toc};
use std::cell::RefCell;

pub struct LosslessThunderFrameDecode<'m> {
    pub status: Status,
    pub image: ImageBundle<'m>,
    // DecodeFrame copies this only after successful InitFrameOutput.
    pub header: Option<FrameHeader<'m>>,
}
#[derive(Clone, Copy, Default)]
struct TocEntry {
    size: usize,
    id: usize,
}
#[derive(Clone, Copy, PartialEq)]
enum SectionStatus {
    Done,
    Skipped,
    Duplicate,
    Partial,
}
struct Section<'a> {
    reader: BitReader<'a>,
    id: usize,
    index: usize,
}

// Only this constructor admits a parsed header into the fixed decoder. The
// metadata owner is the private encoder configuration; callers cannot mutate
// the admitted header or output-encoding state into an unported source path.
struct AdmittedHeader<'m>(FrameHeader<'m>);
impl<'m> AdmittedHeader<'m> {
    fn admit(h: FrameHeader<'m>) -> Result<Self, StatusCode> {
        if h.encoding != FrameEncoding::MODULAR
            || h.frame_type != FrameType::REGULAR
            || h.nonserialized_is_preview
            || !h.is_last
            || h.can_be_referenced()
            || h.color_transform != ColorTransform::NONE
            || !h.chroma_subsampling.is_444()
            || h.flags & !FLAG_SKIP_ADAPTIVE_DC_SMOOTHING != 0
            || h.loop_filter.gab
            || h.loop_filter.epf_iters != 0
            || h.dc_level != 0
            || h.upsampling != 1
            || h.extra_channel_upsampling.len()
                != h.nonserialized_metadata.unwrap().m.num_extra_channels as usize
            || h.extra_channel_upsampling.iter().any(|&up| up != 1)
            || h.extra_channel_blending_info.len()
                != h.nonserialized_metadata.unwrap().m.num_extra_channels as usize
            || h.extra_channel_blending_info
                .iter()
                .any(|blend| blend.mode != BlendMode::REPLACE)
            || h.custom_size_or_origin
            || h.blending_info.mode != BlendMode::REPLACE
            || h.passes.num_passes != 1
            || h.passes.num_downsample != 0
        {
            return Err(StatusCode::Unsupported);
        }
        Ok(Self(h))
    }
    fn admit_vardct(h: FrameHeader<'m>) -> Result<Self, StatusCode> {
        if h.encoding != FrameEncoding::VAR_DCT
            || h.frame_type != FrameType::REGULAR
            || h.nonserialized_is_preview
            || !h.is_last
            || h.can_be_referenced()
            || h.color_transform != ColorTransform::XYB
            || !h.chroma_subsampling.is_444()
            || h.flags
                & !(FLAG_SKIP_ADAPTIVE_DC_SMOOTHING
                    | super::frame_header::FLAG_NOISE
                    | super::frame_header::FLAG_SPLINES
                    | super::frame_header::FLAG_PATCHES)
                != 0
            || h.dc_level != 0
            || h.upsampling != 1
            || h.extra_channel_upsampling.len()
                != h.nonserialized_metadata.unwrap().m.num_extra_channels as usize
            || h.extra_channel_upsampling.iter().any(|&up| up != 1)
            || h.extra_channel_blending_info.len()
                != h.nonserialized_metadata.unwrap().m.num_extra_channels as usize
            || h.extra_channel_blending_info
                .iter()
                .any(|blend| blend.mode != BlendMode::REPLACE)
            || h.custom_size_or_origin
            || h.blending_info.mode != BlendMode::REPLACE
        {
            return Err(StatusCode::Unsupported);
        }
        Ok(Self(h))
    }
}
struct FrameDecoder<'s, 'm, 'p> {
    state: &'s mut PassesDecoderState<'m, 'p>,
    output: &'p RefCell<ImageBundle<'m>>,
    header: AdmittedHeader<'m>,
    dim: FrameDimensions,
    modular: ModularFrameDecoder,
    toc: Vec<TocEntry>,
    processed: Vec<u8>,
    passes: Vec<u8>,
    dc_groups: Vec<u8>,
    decoded_dc_global: bool,
    decoded_ac_global: bool,
    finalized_dc: bool,
    num_sections_done: usize,
    is_finalized: bool,
    allocated: bool,
    group_caches: Vec<GroupDecCache>,
    use_task_id: bool,
}
impl FrameDecoder<'_, '_, '_> {
    fn init_output(&mut self) -> Status {
        let h = &self.header.0;
        initialize_passes_shared_state(h, &mut self.state.shared_storage, false)?;
        self.state.init(h)?;
        self.modular.init(&self.dim);
        // The internally constructed output is non-JPEG.
        self.decoded_dc_global = false;
        self.decoded_ac_global = false;
        self.is_finalized = false;
        self.finalized_dc = false;
        self.num_sections_done = 0;
        self.dc_groups.clear();
        self.dc_groups.resize(self.dim.num_dc_groups, 0);
        self.passes.clear();
        self.passes.resize(self.dim.num_groups, 0);
        self.processed.clear();
        self.processed.resize(self.toc.len(), 0);
        self.allocated = false;
        Ok(())
    }
    fn process_dc_global(&mut self, reader: &mut BitReader<'_>) -> Status {
        let shared = &mut self.state.shared_storage;
        if self.header.0.flags & super::frame_header::FLAG_PATCHES != 0 {
            let mut uses_extra_channels = false;
            shared.image_features.patches.borrow_mut().decode(
                reader,
                self.dim.xsize_padded,
                self.dim.ysize_padded,
                self.header
                    .0
                    .nonserialized_metadata
                    .unwrap()
                    .m
                    .num_extra_channels as usize,
                &mut uses_extra_channels,
                &shared.reference_frames.borrow(),
            )?;
        } else {
            shared.image_features.patches.borrow_mut().clear();
        }
        shared.image_features.splines.borrow_mut().clear();
        if self.header.0.flags & super::frame_header::FLAG_SPLINES != 0 {
            shared
                .image_features
                .splines
                .borrow_mut()
                .decode(reader, self.dim.xsize * self.dim.ysize)?;
        }
        if self.header.0.flags & super::frame_header::FLAG_NOISE != 0 {
            super::dec_noise::decode_noise(
                reader,
                &mut shared.image_features.noise_params.borrow_mut(),
            )?;
        }
        shared.matrices.decode_dc(reader)?;
        if self.header.0.encoding == FrameEncoding::VAR_DCT {
            shared.quantizer.decode(&shared.matrices, reader)?;
            super::entropy_coder::decode_block_ctx_map(reader, &mut shared.block_ctx_map)?;
            shared.cmap.decode_dc(reader)?;
            shared.ac_strategy.fill_invalid();
        }
        if self.header.0.flags & super::frame_header::FLAG_SPLINES != 0 {
            let shared = self.state.shared();
            shared
                .image_features
                .splines
                .borrow_mut()
                .initialize_draw_cache(
                    self.dim.xsize_upsampled,
                    self.dim.ysize_upsampled,
                    &shared.cmap.base(),
                )?;
        }
        let status = self
            .modular
            .decode_global_info(reader, &self.header.0, false);
        if status.is_ok() {
            self.decoded_dc_global = true;
        }
        status
    }
    fn process_dc_group(&mut self, g: usize, reader: &mut BitReader<'_>) -> Status {
        if self.header.0.encoding == FrameEncoding::VAR_DCT {
            self.modular
                .decode_var_dct_dc(&self.header.0, g, reader, self.state)?;
        }
        let d = &self.dim;
        let rect = Rect::new(
            (g % d.xsize_dc_groups) * d.dc_group_dim,
            (g / d.xsize_dc_groups) * d.dc_group_dim,
            d.dc_group_dim,
            d.dc_group_dim,
        );
        self.modular.decode_group(
            &self.header.0,
            rect,
            Some(reader),
            3,
            1000,
            &ModularStreamId::modular_dc(g),
            false,
            None,
            None,
            false,
            None,
        )?;
        if self.header.0.encoding == FrameEncoding::VAR_DCT {
            self.modular
                .decode_ac_metadata(&self.header.0, g, reader, self.state)?;
        }
        self.dc_groups[g] = 1;
        Ok(())
    }
    fn prepare_pipeline(&mut self) -> Status {
        // PreparePipeline with use_slow_render_pipeline=true, coalescing=true,
        // render_spotcolors=true, render_noise=true. Header admission proves
        // only the explicitly connected filter/transform branches are reachable. One unassociated
        // full-resolution alpha plane needs no additional processing stage.
        let num_c = 3 + self
            .header
            .0
            .nonserialized_metadata
            .unwrap()
            .m
            .num_extra_channels as usize;
        let render_noise = self.header.0.flags & super::frame_header::FLAG_NOISE != 0;
        let mut builder =
            SimpleRenderPipelineBuilder::new(num_c + if render_noise { 3 } else { 0 });
        if self.header.0.loop_filter.gab {
            builder.add_stage(super::render_pipeline::stage_gaborish::get_gaborish_stage(
                &self.header.0.loop_filter,
            ))?;
        }
        {
            use super::render_pipeline::stage_epf::{EpfStage, get_epf_stage_shared};
            let lf = &self.header.0.loop_filter;
            if lf.epf_iters >= 3 {
                builder.add_stage(get_epf_stage_shared(
                    lf,
                    self.state.sigma.clone(),
                    EpfStage::Zero,
                ))?;
            }
            if lf.epf_iters >= 1 {
                builder.add_stage(get_epf_stage_shared(
                    lf,
                    self.state.sigma.clone(),
                    EpfStage::One,
                ))?;
            }
            if lf.epf_iters >= 2 {
                builder.add_stage(get_epf_stage_shared(
                    lf,
                    self.state.sigma.clone(),
                    EpfStage::Two,
                ))?;
            }
        }
        if self.header.0.flags & super::frame_header::FLAG_PATCHES != 0 {
            builder.add_stage(Some(
                super::render_pipeline::stage_patches::get_patches_stage_shared(
                    self.state.shared().image_features.patches.clone(),
                    self.state.shared().reference_frames.clone(),
                    &self
                        .header
                        .0
                        .nonserialized_metadata
                        .unwrap()
                        .m
                        .extra_channel_info,
                ),
            ))?;
        }
        if self.header.0.flags & super::frame_header::FLAG_SPLINES != 0 {
            builder.add_stage(Some(Box::new(
                super::render_pipeline::stage_splines::get_spline_stage_shared(
                    self.state.shared().image_features.splines.clone(),
                ),
            )))?;
        }
        if render_noise {
            builder.add_stage(Some(
                super::render_pipeline::stage_noise::get_convolve_noise_stage(num_c),
            ))?;
            builder.add_stage(Some(
                super::render_pipeline::stage_noise::get_add_noise_stage_shared(
                    self.state.shared().image_features.noise_params.clone(),
                    self.state.shared().cmap.base_shared(),
                    num_c,
                ),
            ))?;
        }
        let info = &self.state.output_encoding_info;
        let tone = get_tone_mapping_stage(info);
        assert!(
            tone.is_none(),
            "closed metadata has equal source/desired intensity"
        );
        if self.header.0.color_transform == ColorTransform::XYB {
            builder.add_stage(Some(super::render_pipeline::stage_xyb::get_xyb_stage(info)))?;
            // Admission proves sRGB output. This is the original
            // FromLinearStage<OpRgb>, including its scalar approximation.
            builder.add_stage(Some(
                super::render_pipeline::stage_from_linear::get_srgb_from_linear_stage(),
            ))?;
        }
        // SAFETY: State is private to this fixed entry point. SetFromMetadata
        // leaves cms_set=false, so the actual factory invokes no foreign code
        // and creates no context with a foreign lifetime obligation.
        let cms = unsafe { get_cms_stage(info, false) };
        if cms.is_some() {
            builder.add_stage(cms)?;
        }
        builder.add_stage(Some(Box::new(WriteToImageBundleStage::from_shared(
            self.output,
            &info.color_encoding,
        ))))?;
        self.state.render_pipeline = Some(builder.finalize(self.state.shared().frame_dim)?);
        self.state
            .render_pipeline
            .as_ref()
            .unwrap()
            .is_initialized()
    }
    fn allocate_output(&mut self) -> Status {
        if self.allocated {
            return Ok(());
        }
        self.modular.maybe_drop_full_image();
        self.output.borrow_mut().origin = self.header.0.frame_origin;
        self.state
            .init_for_ac(self.header.0.passes.num_passes as usize)?;
        self.allocated = true;
        Ok(())
    }
    fn prepare_storage(&mut self, threads: usize, tasks: usize) -> Status {
        let size = threads.min(tasks);
        if size > self.group_caches.len() {
            self.group_caches.resize_with(size, GroupDecCache::default);
        }
        self.use_task_id = threads > tasks;
        // Noise is outside admission; VarDCT full-image output retains group input.
        if let Some(p) = &mut self.state.render_pipeline {
            let use_group_ids =
                self.modular.uses_full_image() && self.header.0.encoding == FrameEncoding::VAR_DCT;
            p.prepare_for_threads(size, use_group_ids)?;
        }
        self.state
            .upsampler8x
            .as_mut()
            .unwrap()
            .prepare_for_threads(threads)
    }
    fn process_ac_global(&mut self, reader: &mut BitReader<'_>) -> Status {
        if !self.finalized_dc {
            return Err(StatusCode::GenericError);
        }
        if self.header.0.encoding == FrameEncoding::VAR_DCT {
            let used = self
                .state
                .used_acs
                .load(std::sync::atomic::Ordering::SeqCst);
            let shared = &mut self.state.shared_storage;
            shared.matrices.decode(reader, Some(&self.modular))?;
            shared.matrices.ensure_computed(used)?;
            let bits = super::base::bits::ceil_log2_nonzero(self.dim.num_groups);
            shared.num_histograms = 1 + reader.read_bits(bits) as usize;
            self.state
                .code
                .resize_with(super::common::MAX_NUM_PASSES, Default::default);
            self.state
                .context_map
                .resize_with(super::common::MAX_NUM_PASSES, Vec::new);
            let mut max_bits = 0usize;
            for pass in 0..self.header.0.passes.num_passes as usize {
                let orders =
                    super::fields::u32_coder::read(super::frame_header::ORDER_ENC, reader) as u16;
                let start = pass * shared.coeff_order_size;
                super::coeff_order::decode_coeff_orders(
                    orders,
                    used,
                    &mut shared.coeff_orders[start..start + shared.coeff_order_size],
                    reader,
                )?;
                let contexts =
                    shared.num_histograms * shared.block_ctx_map.num_ac_contexts() as usize;
                super::dec_ans::decode_histograms(
                    reader,
                    contexts,
                    &mut self.state.code[pass],
                    &mut self.state.context_map[pass],
                    false,
                )?;
                self.state.context_map[pass].resize(contexts + 16, 0);
                max_bits = max_bits.max(self.state.code[pass].max_num_bits);
            }
            max_bits += super::base::bits::ceil_log2_nonzero(self.header.0.passes.num_passes);
            let kind = if max_bits < 16 {
                super::dct_util::AcType::I16
            } else {
                super::dct_util::AcType::I32
            };
            let store = self.header.0.passes.num_passes > 1;
            self.state.coefficients = Some(super::dct_util::AcImage::make(
                kind,
                if store {
                    super::frame_dimensions::GROUP_DIM * super::frame_dimensions::GROUP_DIM
                } else {
                    0
                },
                if store { self.dim.num_groups } else { 0 },
            )?);
            if store {
                self.state.coefficients.as_mut().unwrap().zero_fill();
            }
        }
        Ok(())
    }
    fn process_ac_group(
        &mut self,
        g: usize,
        readers: &mut [Section<'_>],
        indices: &[usize],
        count: usize,
        thread: usize,
    ) -> Status {
        let d = &self.dim;
        let mut input = self
            .state
            .render_pipeline
            .as_mut()
            .unwrap()
            .get_input_buffers(g, thread);
        let rect = Rect::new(
            (g % d.xsize_groups) * d.group_dim,
            (g / d.xsize_groups) * d.group_dim,
            d.group_dim,
            d.group_dim,
        );
        let mut should_run_pipeline = true;
        let pass0 = self.passes[g] as usize;
        if self.header.0.encoding == FrameEncoding::VAR_DCT {
            self.group_caches[thread].init_once(
                self.header.0.passes.num_passes as usize,
                self.state
                    .used_acs
                    .load(std::sync::atomic::Ordering::SeqCst) as usize,
            )?;
            let mut pass_readers: Vec<_> = (pass0..pass0 + count)
                .map(|i| std::mem::take(&mut readers[indices[i]].reader))
                .collect();
            let group_state = super::dec_group::AcGroupDecoderState {
                shared: &self.state.shared_storage,
                code: &self.state.code,
                context_map: &self.state.context_map,
                coefficients: self.state.coefficients.as_mut().unwrap(),
                x_dm_multiplier: self.state.x_dm_multiplier,
                b_dm_multiplier: self.state.b_dm_multiplier,
                quant_biases: &self.state.output_encoding_info.opsin_params.quant_biases,
            };
            let result = super::dec_group::decode_ac_group_pixels(
                &self.header.0,
                &mut pass_readers,
                std::num::NonZeroUsize::new(count).unwrap(),
                g,
                group_state,
                &mut self.group_caches[thread],
                &mut input,
                pass0,
                false,
                Some(&mut should_run_pipeline),
            );
            for (i, reader) in (pass0..pass0 + count).zip(pass_readers) {
                readers[indices[i]].reader = reader;
            }
            result?;
        }
        let mut ready = false;
        for i in pass0..pass0 + count {
            let (min, max) = self.header.0.passes.get_downsampling_bracket(i);
            let mut pass_ready = true;
            self.modular.decode_group(
                &self.header.0,
                rect,
                Some(&mut readers[indices[i]].reader),
                min,
                max,
                &ModularStreamId::modular_ac(g, i),
                false,
                Some(&self.state.shared_storage),
                Some(&mut input),
                false,
                Some(&mut pass_ready),
            )?;
            if pass_ready {
                ready = true;
            }
        }
        self.passes[g] = self.passes[g].wrapping_add(count as u8);
        if self.header.0.flags & super::frame_header::FLAG_NOISE != 0 {
            super::dec_noise::prepare_noise_input(
                self.state.visible_frame_index,
                self.state.nonvisible_frame_index,
                &self.dim,
                &self.header.0,
                g,
                &mut input,
            );
        }
        if !self.modular.uses_full_image() && should_run_pipeline && ready {
            input.done()?;
        }
        Ok(())
    }
    fn mark_sections(&mut self, sections: &[Section<'_>], statuses: &[SectionStatus]) {
        self.num_sections_done = self.num_sections_done.wrapping_add(sections.len());
        for (section, status) in sections.iter().zip(statuses) {
            if *status != SectionStatus::Done {
                self.processed[section.id] = 0;
                self.num_sections_done = self.num_sections_done.wrapping_sub(1);
            }
        }
    }
    fn process_sections(
        &mut self,
        sections: &mut [Section<'_>],
        statuses: &mut [SectionStatus],
    ) -> Status {
        let num = sections.len();
        if num == 0 {
            return Ok(());
        }
        statuses.fill(SectionStatus::Skipped);
        let mut dc_global = num;
        let mut ac_global = num;
        let mut dc = vec![num; self.dim.num_dc_groups];
        let mut ac = vec![vec![num; self.header.0.passes.num_passes as usize]; self.dim.num_groups];
        let mut desired = vec![0; self.dim.num_groups];
        let single = self.dim.num_groups == 1 && self.header.0.passes.num_passes == 1;
        if single {
            if num != 1 || sections[0].id != 0 {
                return Err(StatusCode::GenericError);
            }
            if self.processed[0] == 0 {
                self.processed[0] = 1;
                ac[0].resize(1, num);
                dc_global = 0;
                ac_global = 0;
                dc[0] = 0;
                ac[0][0] = 0;
                desired[0] = 1;
            } else {
                statuses[0] = SectionStatus::Duplicate;
            }
        } else {
            let ac_global_index = self.dim.num_dc_groups + 1;
            for i in 0..num {
                let id = sections[i].id;
                if id >= self.processed.len() {
                    return Err(StatusCode::GenericError);
                }
                if self.processed[id] != 0 {
                    statuses[i] = SectionStatus::Duplicate;
                    continue;
                }
                if id == 0 {
                    dc_global = i;
                } else if id < ac_global_index {
                    dc[id - 1] = i;
                } else if id == ac_global_index {
                    ac_global = i;
                } else {
                    let idx = id - ac_global_index - 1;
                    let g = idx % self.dim.num_groups;
                    let p = idx / self.dim.num_groups;
                    if p >= self.header.0.passes.num_passes as usize {
                        return Err(StatusCode::GenericError);
                    }
                    ac[g][p] = i;
                }
                self.processed[id] = 1;
            }
            for g in 0..ac.len() {
                let mut j = 0;
                while j + (self.passes[g] as usize) < self.header.0.passes.num_passes as usize {
                    if ac[g][j + self.passes[g] as usize] == num {
                        break;
                    }
                    j += 1;
                }
                desired[g] = j;
            }
        }
        if dc_global != num {
            let status = self.process_dc_global(&mut sections[dc_global].reader);
            if let Err(e) = status {
                if e.is_fatal_error() {
                    return Err(e);
                }
            }
            statuses[dc_global] = if status.is_ok() {
                SectionStatus::Done
            } else {
                SectionStatus::Partial
            };
        }
        if self.decoded_dc_global {
            for i in 0..dc.len() {
                if dc[i] != num {
                    self.process_dc_group(i, &mut sections[dc[i]].reader)?;
                    statuses[dc[i]] = SectionStatus::Done;
                }
            }
        }
        if !self.dc_groups.contains(&0) && !self.finalized_dc {
            self.prepare_pipeline()?;
            if self.header.0.encoding == FrameEncoding::VAR_DCT
                && self.header.0.flags & FLAG_SKIP_ADAPTIVE_DC_SMOOTHING == 0
            {
                let shared = &mut self.state.shared_storage;
                super::compressed_dc::adaptive_dc_smoothing(
                    shared.quantizer.mul_dc(),
                    &mut shared.dc_storage,
                    &self.header.0.chroma_subsampling,
                )?;
            }
            self.finalized_dc = true;
            self.allocate_output()?;
        }
        if self.finalized_dc && ac_global != num && !self.decoded_ac_global {
            self.process_ac_global(&mut sections[ac_global].reader)?;
            self.decoded_ac_global = true;
            statuses[ac_global] = SectionStatus::Done;
        }
        if self.decoded_ac_global {
            for (g, &count) in desired.iter().enumerate() {
                if count != 0 {
                    self.state.render_pipeline.as_mut().unwrap().clear_done(g);
                }
            }
            self.prepare_storage(1, self.passes.len())?;
            for g in 0..ac.len() {
                if desired[g] == 0 {
                    continue;
                }
                let first = self.passes[g] as usize;
                for i in 0..desired[g] {
                    if ac[g][first + i] == num {
                        return Err(StatusCode::GenericError);
                    }
                }
                let thread = if self.use_task_id { g } else { 0 };
                self.process_ac_group(g, sections, &ac[g], desired[g], thread)?;
                for i in 0..desired[g] {
                    statuses[ac[g][first + i]] = SectionStatus::Done;
                }
            }
        }
        self.mark_sections(sections, statuses);
        Ok(())
    }
    fn finalize_frame(&mut self) -> Status {
        if self.is_finalized {
            return Err(StatusCode::GenericError);
        }
        self.is_finalized = true;
        self.modular.finalize_decoding(
            &self.header.0,
            &self.state.shared_storage,
            self.state.render_pipeline.as_mut().unwrap(),
            true,
        )
    }
}

impl LosslessThunderConfig {
    /// Decode a complete raw frame (header, TOC, all sections), with metadata
    /// supplied by this fixed configuration. This is not complete codestream
    /// decoding. Unsupported means the parsed header is outside the admitted
    /// profile; it does not stand in for an unported generic decoder branch.
    pub fn decode_frame<'m>(&'m self, bytes: &[u8]) -> LosslessThunderFrameDecode<'m> {
        let output = RefCell::new(ImageBundle::new(Some(&self.metadata().m)));
        let mut copied_header = None;
        let mut state = PassesDecoderState::new();
        let status = state
            .output_encoding_info
            .set_from_metadata(self.metadata())
            .and_then(|()| self.decode_frame_into(bytes, &mut state, &output, &mut copied_header));
        drop(state);
        LosslessThunderFrameDecode {
            status,
            image: output.into_inner(),
            header: copied_header,
        }
    }
    fn decode_frame_into<'m, 'p>(
        &'m self,
        bytes: &[u8],
        state: &mut PassesDecoderState<'m, 'p>,
        output: &'p RefCell<ImageBundle<'m>>,
        copied_header: &mut Option<FrameHeader<'m>>,
    ) -> Status {
        decode_profile_frame_into(self.metadata(), bytes, state, output, copied_header, false)
    }
}

fn decode_profile_frame_into<'m, 'p>(
    metadata: &'m super::image_metadata::CodecMetadata,
    bytes: &[u8],
    state: &mut PassesDecoderState<'m, 'p>,
    output: &'p RefCell<ImageBundle<'m>>,
    copied_header: &mut Option<FrameHeader<'m>>,
    vardct: bool,
) -> Status {
    (|| {
        let mut header = FrameHeader::new(Some(metadata));
        let modular = ModularFrameDecoder::new();
        let mut reader = BitReader::new(bytes);
        // InitFrame: header and TOC parsing precede profile admission.
        state.shared_storage.matrices = DequantMatrices::default();
        let initialized = (|| {
            header.nonserialized_is_preview = false;
            read_frame_header(&mut reader, &mut header)?;
            let dim = header.to_frame_dimensions();
            {
                let mut decoded = output.borrow_mut();
                *decoded.color_mut() = Image3::default();
                decoded.extra_channels_mut().clear();
                decoded.duration = header.animation_frame.duration;
            }
            if !header.nonserialized_is_preview
                && (header.is_last || header.animation_frame.duration > 0)
                && (header.frame_type == FrameType::REGULAR
                    || header.frame_type == FrameType::SKIP_PROGRESSIVE)
            {
                state.visible_frame_index = state.visible_frame_index.wrapping_add(1);
                state.nonvisible_frame_index = 0;
            } else {
                state.nonvisible_frame_index = state.nonvisible_frame_index.wrapping_add(1);
            }
            let n = num_toc_entries(
                dim.num_groups,
                dim.num_dc_groups,
                header.passes.num_passes as usize,
            );
            let mut sizes = Vec::new();
            let mut permutation = Vec::new();
            read_toc(n, &mut reader, &mut sizes, &mut permutation)?;
            let mut toc = vec![TocEntry::default(); n];
            let mut sum = 0u64;
            for i in 0..n {
                toc[i].size = sizes[i] as usize;
                let index = if permutation.is_empty() {
                    i
                } else {
                    permutation[i] as usize
                };
                toc[index].id = i;
                let next = sum.wrapping_add(toc[i].size as u64);
                if next < sum {
                    return Err(StatusCode::GenericError);
                }
                sum = next;
            }
            if reader.total_bits_consumed() % 8 != 0 || toc.is_empty() {
                return Err(StatusCode::GenericError);
            }
            let begin = reader.total_bits_consumed() / 8;
            if (begin as u64).wrapping_add(sum) < begin as u64 {
                return Err(StatusCode::GenericError);
            }
            Ok((dim, toc))
        })();
        let (dim, toc) = match initialized {
            Ok(v) => v,
            Err(e) => {
                let _ = reader.close();
                return Err(e);
            }
        };
        let admitted = match if vardct {
            AdmittedHeader::admit_vardct(header)
        } else {
            AdmittedHeader::admit(header)
        } {
            Ok(v) => v,
            Err(e) => {
                let _ = reader.close();
                return Err(e);
            }
        };
        let mut decoder = FrameDecoder {
            state,
            output,
            header: admitted,
            dim,
            modular,
            toc,
            processed: Vec::new(),
            passes: Vec::new(),
            dc_groups: Vec::new(),
            decoded_dc_global: false,
            decoded_ac_global: false,
            finalized_dc: true,
            num_sections_done: 0,
            is_finalized: true,
            allocated: false,
            group_caches: Vec::new(),
            use_task_id: false,
        };
        if let Err(e) = decoder.init_output() {
            let _ = reader.close();
            return Err(e);
        }
        *copied_header = Some(decoder.header.0.clone());
        if let Err(e) = reader.all_reads_within_bounds() {
            let _ = reader.close();
            return Err(e);
        }
        let header_bytes = reader.total_bits_consumed() / 8;
        reader.close()?;
        let mut processed_bytes = header_bytes;
        let mut sections = Vec::new();
        let mut pos = header_bytes;
        let work = (|| {
            for (index, entry) in decoder.toc.iter().enumerate() {
                let end = pos.wrapping_add(entry.size);
                if end > bytes.len() {
                    return Err(StatusCode::GenericError);
                }
                sections.push(Section {
                    reader: BitReader::new(&bytes[pos..end]),
                    id: entry.id,
                    index,
                });
                pos = end;
            }
            let mut statuses = vec![SectionStatus::Skipped; sections.len()];
            decoder.process_sections(&mut sections, &mut statuses)?;
            for (i, status) in statuses.iter().enumerate() {
                if *status != SectionStatus::Done {
                    return Err(StatusCode::GenericError);
                }
                processed_bytes = processed_bytes.wrapping_add(decoder.toc[i].size);
            }
            Ok(())
        })();
        let mut close_ok = Ok(());
        for section in sections.iter_mut().rev() {
            // index is retained at its original physical-order checkpoint.
            let _ = section.index;
            if let Err(e) = section.reader.close() {
                close_ok = Err(e);
            }
        }
        work?;
        close_ok?;
        decoder.finalize_frame()?;
        output.borrow_mut().set_decoded_bytes(processed_bytes);
        Ok(())
    })()
}

/// Complete first-profile codestream result with owned metadata and integer
/// samples. Error reports retain parsed metadata, consumed bits, and defined
/// frame dimensions/byte accounting; no self-referential bundle escapes.
pub struct LosslessThunderImageDecode {
    pub status: Status,
    pub metadata: super::image_metadata::CodecMetadata,
    pub pixels: Vec<u8>,
    pub reader_bits: usize,
    pub frame_size: (usize, usize),
    pub frame_decoded_bytes: usize,
}

/// The fixed-profile path from render_pipeline_test.cc DecodeFile, followed by
/// the integer buffer branch of dec_external_image.cc ConvertToExternal.
/// Global fields are actually parsed from input; dimensions and samples are
/// not supplied by the caller. ICC/preview/animation and other configurations
/// remain outside this entry point's explicit admission boundary.
pub fn decode_lossless_thunder(bytes: &[u8]) -> LosslessThunderImageDecode {
    use super::dec_external_image::convert_uint_to_buffer;
    use super::fields::bundle;
    use super::headers::read_size_header;
    use super::image_metadata::{CodecMetadata, Orientation, read_image_metadata};
    use super::types::JxlEndianness;
    let mut metadata = Some(CodecMetadata::default());
    let mut pixels = Vec::new();
    let mut frame_size = (0, 0);
    let mut frame_decoded_bytes = 0;
    let mut reader = BitReader::new(bytes);
    let parsed = (|| {
        if reader.read_fixed_bits::<16>() != 0x0aff {
            return Err(StatusCode::GenericError);
        }
        {
            let m = metadata.as_mut().unwrap();
            read_size_header(&mut reader, &mut m.size)?;
            read_image_metadata(&mut reader, &mut m.m)?;
            m.transform_data.nonserialized_xyb_encoded = m.m.xyb_encoded;
            bundle::read(&mut reader, &mut m.transform_data)?;
        }
        Ok(())
    })();
    let config = if parsed.is_ok() {
        LosslessThunderConfig::admit_decoded_metadata(&mut metadata)
    } else {
        None
    };
    let output = config
        .as_ref()
        .map(|cfg| RefCell::new(ImageBundle::new(Some(&cfg.metadata().m))));
    let body = (|| {
        parsed?;
        let cfg = config.as_ref().ok_or(StatusCode::Unsupported)?;
        // These external owners keep the source decoder's pointer bindings
        // live. Empty/default bundle construction allocates no pixel storage.
        let preview = ImageBundle::new(Some(&cfg.metadata().m));
        let output = output.as_ref().unwrap();
        let mut state = PassesDecoderState::new();
        state
            .output_encoding_info
            .set_from_metadata(cfg.metadata())?;
        reader.jump_to_byte_boundary()?;
        let mut header = None;
        // The admitted final regular frame makes both original do/while
        // continuation conditions false. No additional frame is skipped.
        let start = reader.total_bits_consumed() / 8;
        let frame_status = cfg.decode_frame_into(&bytes[start..], &mut state, output, &mut header);
        {
            let image = output.borrow();
            frame_size = (image.color().xsize(), image.color().ysize());
            frame_decoded_bytes = image.decoded_bytes();
        }
        frame_status?;
        reader.skip_bits(frame_decoded_bytes.wrapping_mul(8));
        if reader.total_bits_consumed() != bytes.len().wrapping_mul(8) {
            return Err(StatusCode::GenericError);
        }
        if reader.all_reads_within_bounds().is_err() {
            return Err(StatusCode::GenericError);
        }
        // CodecInOut::CheckMetadata's actual no-preview/one-frame path.
        let m = cfg.metadata();
        if m.m.bit_depth.bits_per_sample == 0 || m.m.color_encoding.icc().is_empty() {
            return Err(StatusCode::GenericError);
        }
        if preview.xsize() != 0 {
            preview.verify_metadata()?;
        }
        if !std::ptr::eq(preview.metadata().unwrap(), &m.m) {
            return Err(StatusCode::GenericError);
        }
        let image = output.borrow();
        image.verify_metadata()?;
        if !std::ptr::eq(image.metadata().unwrap(), &m.m) {
            return Err(StatusCode::GenericError);
        }
        Ok(())
    })();
    let reader_bits = reader.total_bits_consumed();
    let closed = reader.close();
    let mut status = body.and(closed);
    if status.is_ok() {
        // The source decoder state and scoped reader close have completed
        // before the caller invokes the original external conversion.
        let cfg = config.as_ref().unwrap();
        let m = cfg.metadata();
        let n = m.m.color_encoding.channels() + usize::from(m.m.has_alpha());
        let bits = m.m.bit_depth.bits_per_sample as usize;
        status = (|| {
            let stride = m
                .xsize()
                .checked_mul(n)
                .and_then(|v| v.checked_mul(bits / 8))
                .ok_or(StatusCode::GenericError)?;
            let size = stride
                .checked_mul(m.ysize())
                .ok_or(StatusCode::GenericError)?;
            pixels.resize(size, 0);
            convert_uint_to_buffer(
                &output.as_ref().unwrap().borrow(),
                bits,
                n,
                JxlEndianness::NATIVE,
                stride,
                &mut pixels,
                size,
                Orientation::IDENTITY,
                false,
            )
        })();
    }
    drop(output);
    if let Some(cfg) = config {
        metadata = Some(cfg.into_metadata());
    }
    LosslessThunderImageDecode {
        status,
        metadata: metadata.unwrap(),
        pixels,
        reader_bits,
        frame_size,
        frame_decoded_bytes,
    }
}

// Immutable metadata ownership keeps this first complete VarDCT profile closed
// until its other original rendering/frame branches have been translated.
struct VarDctConfig {
    metadata: super::image_metadata::CodecMetadata,
}
impl VarDctConfig {
    fn metadata(&self) -> &super::image_metadata::CodecMetadata {
        &self.metadata
    }
    fn into_metadata(self) -> super::image_metadata::CodecMetadata {
        self.metadata
    }
    fn admit_decoded_metadata(
        owner: &mut Option<super::image_metadata::CodecMetadata>,
    ) -> Option<Self> {
        use super::color_encoding::RenderingIntent;
        use super::image_metadata::ExtraChannel;
        let m = &owner.as_ref()?.m;
        if m.bit_depth.floating_point_sample
            || ![8, 16].contains(&m.bit_depth.bits_per_sample)
            || !m.xyb_encoded
            || m.num_extra_channels as usize != m.extra_channel_info.len()
            || m.extra_channel_info.len() > 1
            || m.extra_channel_info.iter().any(|ec| {
                ec.channel_type != ExtraChannel::ALPHA
                    || ec.bit_depth.floating_point_sample
                    || ec.bit_depth.bits_per_sample != m.bit_depth.bits_per_sample
                    || ec.bit_depth.exponent_bits_per_sample != 0
                    || ec.dim_shift != 0
                    || ec.alpha_associated
            })
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
    fn decode_frame_into<'m, 'p>(
        &'m self,
        bytes: &[u8],
        state: &mut PassesDecoderState<'m, 'p>,
        output: &'p RefCell<ImageBundle<'m>>,
        header: &mut Option<FrameHeader<'m>>,
    ) -> Status {
        decode_profile_frame_into(self.metadata(), bytes, state, output, header, true)
    }
}

/// Complete raw-codestream VarDCT result for the admitted sRGB XYB profile.
/// Integer samples retain the source bit depth; float samples are interleaved
/// sRGB color from the original image-bundle output stage, before quantization.
/// General frames, saved references, resampling and container boxes remain outside
/// admission. Scalar EPF, noise and spline effects follow their original stages.
pub struct VarDctImageDecode {
    pub status: Status,
    pub metadata: super::image_metadata::CodecMetadata,
    pub pixels: Vec<u8>,
    pub float_pixels: Vec<f32>,
    pub reader_bits: usize,
    pub frame_size: (usize, usize),
    pub frame_decoded_bytes: usize,
}

/// Parse a complete original VarDCT codestream and decode its DC/AC groups,
/// dequantization, inverse transforms, optional Gaborish, XYB and sRGB output.
/// The private admission boundary excludes still-unported codec configurations.
pub fn decode_vardct(bytes: &[u8]) -> VarDctImageDecode {
    use super::dec_external_image::convert_uint_to_buffer;
    use super::fields::bundle;
    use super::headers::read_size_header;
    use super::image_metadata::{CodecMetadata, Orientation, read_image_metadata};
    use super::types::JxlEndianness;
    let mut metadata = Some(CodecMetadata::default());
    let mut pixels = Vec::new();
    let mut frame_size = (0, 0);
    let mut frame_decoded_bytes = 0;
    let mut reader = BitReader::new(bytes);
    let parsed = (|| {
        if reader.read_fixed_bits::<16>() != 0x0aff {
            return Err(StatusCode::GenericError);
        }
        {
            let m = metadata.as_mut().unwrap();
            read_size_header(&mut reader, &mut m.size)?;
            read_image_metadata(&mut reader, &mut m.m)?;
            m.transform_data.nonserialized_xyb_encoded = m.m.xyb_encoded;
            bundle::read(&mut reader, &mut m.transform_data)?;
        }
        Ok(())
    })();
    let config = if parsed.is_ok() {
        VarDctConfig::admit_decoded_metadata(&mut metadata)
    } else {
        None
    };
    let output = config
        .as_ref()
        .map(|cfg| RefCell::new(ImageBundle::new(Some(&cfg.metadata().m))));
    let body = (|| {
        parsed?;
        let cfg = config.as_ref().ok_or(StatusCode::Unsupported)?;
        // These external owners keep the source decoder's pointer bindings
        // live. Empty/default bundle construction allocates no pixel storage.
        let preview = ImageBundle::new(Some(&cfg.metadata().m));
        let output = output.as_ref().unwrap();
        let mut state = PassesDecoderState::new();
        state
            .output_encoding_info
            .set_from_metadata(cfg.metadata())?;
        reader.jump_to_byte_boundary()?;
        let mut header = None;
        // The admitted final regular frame makes both original do/while
        // continuation conditions false. No additional frame is skipped.
        let start = reader.total_bits_consumed() / 8;
        let frame_status = cfg.decode_frame_into(&bytes[start..], &mut state, output, &mut header);
        {
            let image = output.borrow();
            frame_size = (image.color().xsize(), image.color().ysize());
            frame_decoded_bytes = image.decoded_bytes();
        }
        frame_status?;
        reader.skip_bits(frame_decoded_bytes.wrapping_mul(8));
        if reader.total_bits_consumed() != bytes.len().wrapping_mul(8) {
            return Err(StatusCode::GenericError);
        }
        if reader.all_reads_within_bounds().is_err() {
            return Err(StatusCode::GenericError);
        }
        // CodecInOut::CheckMetadata's actual no-preview/one-frame path.
        let m = cfg.metadata();
        if m.m.bit_depth.bits_per_sample == 0 || m.m.color_encoding.icc().is_empty() {
            return Err(StatusCode::GenericError);
        }
        if preview.xsize() != 0 {
            preview.verify_metadata()?;
        }
        if !std::ptr::eq(preview.metadata().unwrap(), &m.m) {
            return Err(StatusCode::GenericError);
        }
        let image = output.borrow();
        image.verify_metadata()?;
        if !std::ptr::eq(image.metadata().unwrap(), &m.m) {
            return Err(StatusCode::GenericError);
        }
        Ok(())
    })();
    let reader_bits = reader.total_bits_consumed();
    let closed = reader.close();
    let mut status = body.and(closed);
    if status.is_ok() {
        // The source decoder state and scoped reader close have completed
        // before the caller invokes the original external conversion.
        let cfg = config.as_ref().unwrap();
        let m = cfg.metadata();
        let n = m.m.color_encoding.channels() + usize::from(m.m.has_alpha());
        let bits = m.m.bit_depth.bits_per_sample as usize;
        status = (|| {
            let stride = m
                .xsize()
                .checked_mul(n)
                .and_then(|v| v.checked_mul(bits / 8))
                .ok_or(StatusCode::GenericError)?;
            let size = stride
                .checked_mul(m.ysize())
                .ok_or(StatusCode::GenericError)?;
            pixels.resize(size, 0);
            convert_uint_to_buffer(
                &output.as_ref().unwrap().borrow(),
                bits,
                n,
                JxlEndianness::NATIVE,
                stride,
                &mut pixels,
                size,
                Orientation::IDENTITY,
                false,
            )
        })();
    }
    let mut float_pixels = Vec::new();
    if status.is_ok() {
        let image = output.as_ref().unwrap().borrow();
        for y in 0..image.color().ysize() {
            for x in 0..image.color().xsize() {
                for c in 0..3 {
                    float_pixels.push(image.color().plane(c).row(y)[x]);
                }
            }
        }
    }
    drop(output);
    if let Some(cfg) = config {
        metadata = Some(cfg.into_metadata());
    }
    VarDctImageDecode {
        status,
        metadata: metadata.unwrap(),
        pixels,
        float_pixels,
        reader_bits,
        frame_size,
        frame_decoded_bytes,
    }
}
