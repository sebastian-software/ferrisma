// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! DecodeFrame/FrameDecoder from dec_frame.{h,cc}, with closed lossless Thunder
//! and first VarDCT raw-codestream profiles plus an admitted shared-state
//! sequence boundary. The original simple pipeline is
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
use std::rc::Rc;

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

enum FrameOutput<'p, 'm> {
    Borrowed(&'p RefCell<ImageBundle<'m>>),
    Owned(Rc<RefCell<ImageBundle<'m>>>),
}
impl<'m> std::ops::Deref for FrameOutput<'_, 'm> {
    type Target = RefCell<ImageBundle<'m>>;
    fn deref(&self) -> &Self::Target {
        match self {
            Self::Borrowed(output) => output,
            Self::Owned(output) => output,
        }
    }
}

// Admission owns each parsed header and bounds the implemented source paths.
// Fixed configurations own their metadata; sequential metadata is borrowed
// immutably for the decoder lifetime. Decoder/output state remains private.
struct AdmittedHeader<'m>(Rc<FrameHeader<'m>>);
impl<'m> AdmittedHeader<'m> {
    // FrameHeader::VisitFields reads the complete U64 flag field. The original
    // decoder tests individual known bits and accepts reserved bits unchanged;
    // rejecting those here would precede its output initialization and errors.
    fn admit_sequence(h: FrameHeader<'m>) -> Result<Self, StatusCode> {
        if !matches!(h.encoding, FrameEncoding::MODULAR | FrameEncoding::VAR_DCT)
            || !matches!(
                h.color_transform,
                ColorTransform::NONE | ColorTransform::XYB | ColorTransform::YCBCR
            )
            || !(h.chroma_subsampling.is_444() || h.color_transform == ColorTransform::YCBCR)
            || h.extra_channel_upsampling.len()
                != h.nonserialized_metadata.unwrap().m.num_extra_channels as usize
            || h.extra_channel_blending_info.len()
                != h.nonserialized_metadata.unwrap().m.num_extra_channels as usize
        {
            return Err(StatusCode::Unsupported);
        }
        Ok(Self(Rc::new(h)))
    }
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
        Ok(Self(Rc::new(h)))
    }
    fn admit_modular(h: FrameHeader<'m>) -> Result<Self, StatusCode> {
        if h.encoding != FrameEncoding::MODULAR
            || h.frame_type != FrameType::REGULAR
            || h.nonserialized_is_preview
            || !h.is_last
            || h.can_be_referenced()
            || !matches!(
                h.color_transform,
                ColorTransform::NONE | ColorTransform::XYB
            )
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
        Ok(Self(Rc::new(h)))
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
        Ok(Self(Rc::new(h)))
    }
}
#[derive(Clone, Copy)]
pub enum FrameWriteTarget {
    Output,
    ReferenceStorage,
}
/// Retained writer checkpoints, recorded only when a caller requests a
/// DecodeFrame trace. Clear/allocation starts a new coverage generation; row
/// events identify the source-defined sample spans, including error prefixes.
#[derive(Clone)]
pub enum FrameWriteEvent {
    BundleClear {
        target: FrameWriteTarget,
    },
    BundleAllocate {
        target: FrameWriteTarget,
        sizes: Vec<(usize, usize)>,
    },
    BundleRow {
        target: FrameWriteTarget,
        channel: usize,
        x: usize,
        y: usize,
        width: usize,
    },
    BundleMoveToReference {
        slot: usize,
    },
    Allocate {
        slot: usize,
        width: usize,
        height: usize,
    },
    Row {
        slot: usize,
        x: usize,
        y: usize,
        width: usize,
    },
}
type FrameWriteTrace = Rc<RefCell<Vec<FrameWriteEvent>>>;
struct TracedDcWriter<'a> {
    original: super::render_pipeline::render_pipeline_stage::WriteToImage3FStage<'a>,
    events: FrameWriteTrace,
    slot: usize,
}
impl super::render_pipeline::render_pipeline_stage::RenderPipelineStage for TracedDcWriter<'_> {
    fn settings(&self) -> super::render_pipeline::render_pipeline_stage::Settings {
        self.original.settings()
    }
    fn set_input_sizes(&mut self, sizes: &[(usize, usize)]) -> Status {
        self.original.set_input_sizes(sizes)?;
        self.events.borrow_mut().push(FrameWriteEvent::Allocate {
            slot: self.slot,
            width: sizes[0].0,
            height: sizes[0].1,
        });
        Ok(())
    }
    fn process_row(
        &mut self,
        input: &super::render_pipeline::render_pipeline_stage::RowInfo,
        output: &super::render_pipeline::render_pipeline_stage::RowInfo,
        buffers: &mut super::render_pipeline::render_pipeline_stage::RowBuffers<'_>,
        left: usize,
        right: usize,
        width: usize,
        x: usize,
        y: usize,
        thread: usize,
    ) -> Status {
        self.original
            .process_row(input, output, buffers, left, right, width, x, y, thread)?;
        self.events.borrow_mut().push(FrameWriteEvent::Row {
            slot: self.slot,
            x,
            y,
            width,
        });
        Ok(())
    }
    fn get_channel_mode(
        &self,
        channel: usize,
    ) -> super::render_pipeline::render_pipeline_stage::RenderPipelineChannelMode {
        self.original.get_channel_mode(channel)
    }
    fn get_name(&self) -> &str {
        self.original.get_name()
    }
}
struct TracedBundleWriter<'a> {
    original: Box<dyn super::render_pipeline::render_pipeline_stage::RenderPipelineStage + 'a>,
    events: FrameWriteTrace,
    target: FrameWriteTarget,
    sizes: Vec<(usize, usize)>,
}
impl super::render_pipeline::render_pipeline_stage::RenderPipelineStage for TracedBundleWriter<'_> {
    fn settings(&self) -> super::render_pipeline::render_pipeline_stage::Settings {
        self.original.settings()
    }
    fn set_input_sizes(&mut self, sizes: &[(usize, usize)]) -> Status {
        self.original.set_input_sizes(sizes)?;
        self.sizes = sizes.to_vec();
        self.events
            .borrow_mut()
            .push(FrameWriteEvent::BundleAllocate {
                target: self.target,
                sizes: sizes.to_vec(),
            });
        Ok(())
    }
    fn process_row(
        &mut self,
        input: &super::render_pipeline::render_pipeline_stage::RowInfo,
        output: &super::render_pipeline::render_pipeline_stage::RowInfo,
        buffers: &mut super::render_pipeline::render_pipeline_stage::RowBuffers<'_>,
        left: usize,
        right: usize,
        width: usize,
        x: usize,
        y: usize,
        thread: usize,
    ) -> Status {
        let status = self
            .original
            .process_row(input, output, buffers, left, right, width, x, y, thread);
        // The original checks padding before its three color memcpy calls,
        // then checks/copies extra planes in order. Preserve that partial
        // mutation even if an extra-plane width guard returns an error.
        if left == 0 && right == 0 {
            for channel in 0..self.sizes.len() {
                if channel >= 3 && self.sizes[channel].0 < x.wrapping_add(width) {
                    break;
                }
                self.events.borrow_mut().push(FrameWriteEvent::BundleRow {
                    target: self.target,
                    channel,
                    x,
                    y,
                    width,
                });
            }
        }
        status
    }
    fn get_channel_mode(
        &self,
        c: usize,
    ) -> super::render_pipeline::render_pipeline_stage::RenderPipelineChannelMode {
        self.original.get_channel_mode(c)
    }
    fn get_name(&self) -> &str {
        self.original.get_name()
    }
    fn is_initialized(&self) -> Status {
        self.original.is_initialized()
    }
    fn prepare_for_threads(&mut self, n: usize) -> Status {
        self.original.prepare_for_threads(n)
    }
}
fn trace_bundle_writer<'a>(
    original: Box<dyn super::render_pipeline::render_pipeline_stage::RenderPipelineStage + 'a>,
    events: &Option<FrameWriteTrace>,
    target: FrameWriteTarget,
) -> Box<dyn super::render_pipeline::render_pipeline_stage::RenderPipelineStage + 'a> {
    if let Some(events) = events {
        Box::new(TracedBundleWriter {
            original,
            events: events.clone(),
            target,
            sizes: Vec::new(),
        })
    } else {
        original
    }
}
// Source frame fields retain their owner independently of the current
// processing scope's codec-state loan. Field and drop order remain unchanged.
struct FrameDecoderStorage<'m, 'p> {
    output: FrameOutput<'p, 'm>,
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
    write_trace: Option<FrameWriteTrace>,
}
// The processing scope lends disjoint frame and codec state. This view owns
// no frame payload and ends before the caller accesses either owner again.
struct FrameDecoder<'s, 'm, 'p> {
    state: &'s mut PassesDecoderState<'m, 'p>,
    storage: &'s mut FrameDecoderStorage<'m, 'p>,
}

impl FrameDecoder<'_, '_, '_> {
    fn init_output(&mut self) -> Status {
        let h = &self.storage.header.0;
        initialize_passes_shared_state(h, &mut self.state.shared_storage, false)?;
        self.state.init(h)?;
        self.storage.modular.init(&self.storage.dim);
        // The internally constructed output is non-JPEG.
        self.storage.decoded_dc_global = false;
        self.storage.decoded_ac_global = false;
        self.storage.is_finalized = false;
        self.storage.finalized_dc = false;
        self.storage.num_sections_done = 0;
        self.storage.dc_groups.clear();
        self.storage
            .dc_groups
            .resize(self.storage.dim.num_dc_groups, 0);
        self.storage.passes.clear();
        self.storage.passes.resize(self.storage.dim.num_groups, 0);
        self.storage.processed.clear();
        self.storage.processed.resize(self.storage.toc.len(), 0);
        self.storage.allocated = false;
        Ok(())
    }
    fn process_dc_global(&mut self, reader: &mut BitReader<'_>) -> Status {
        let shared = &mut self.state.shared_storage;
        if self.storage.header.0.flags & super::frame_header::FLAG_PATCHES != 0 {
            let mut uses_extra_channels = false;
            shared.image_features.patches.borrow_mut().decode(
                reader,
                self.storage.dim.xsize_padded,
                self.storage.dim.ysize_padded,
                self.storage
                    .header
                    .0
                    .nonserialized_metadata
                    .unwrap()
                    .m
                    .num_extra_channels as usize,
                &mut uses_extra_channels,
                &shared.reference_frames.borrow(),
            )?;
            if self.storage.header.0.upsampling != 1
                && self
                    .storage
                    .header
                    .0
                    .extra_channel_upsampling
                    .iter()
                    .any(|&up| up != self.storage.header.0.upsampling)
            {
                return Err(StatusCode::GenericError);
            }
        } else {
            shared.image_features.patches.borrow_mut().clear();
        }
        shared.image_features.splines.borrow_mut().clear();
        if self.storage.header.0.flags & super::frame_header::FLAG_SPLINES != 0 {
            shared
                .image_features
                .splines
                .borrow_mut()
                .decode(reader, self.storage.dim.xsize * self.storage.dim.ysize)?;
        }
        if self.storage.header.0.flags & super::frame_header::FLAG_NOISE != 0 {
            super::dec_noise::decode_noise(
                reader,
                &mut shared.image_features.noise_params.borrow_mut(),
            )?;
        }
        shared.matrices.decode_dc(reader)?;
        if self.storage.header.0.encoding == FrameEncoding::VAR_DCT {
            shared.quantizer.decode(&shared.matrices, reader)?;
            super::entropy_coder::decode_block_ctx_map(reader, &mut shared.block_ctx_map)?;
            shared.cmap.decode_dc(reader)?;
            shared.ac_strategy.fill_invalid();
        }
        if self.storage.header.0.flags & super::frame_header::FLAG_SPLINES != 0 {
            let shared = self.state.shared();
            shared
                .image_features
                .splines
                .borrow_mut()
                .initialize_draw_cache(
                    self.storage.dim.xsize_upsampled,
                    self.storage.dim.ysize_upsampled,
                    &shared.cmap.base(),
                )?;
        }
        let status = self
            .storage
            .modular
            .decode_global_info(reader, &self.storage.header.0, false);
        if status.is_ok() {
            self.storage.decoded_dc_global = true;
        }
        status
    }
    fn process_dc_group(&mut self, g: usize, reader: &mut BitReader<'_>) -> Status {
        if self.storage.header.0.encoding == FrameEncoding::VAR_DCT
            && self.storage.header.0.flags & super::frame_header::FLAG_USE_DC_FRAME == 0
        {
            self.storage.modular.decode_var_dct_dc(
                &self.storage.header.0,
                g,
                reader,
                self.state,
            )?;
        }
        let d = &self.storage.dim;
        let rect = Rect::new(
            (g % d.xsize_dc_groups) * d.dc_group_dim,
            (g / d.xsize_dc_groups) * d.dc_group_dim,
            d.dc_group_dim,
            d.dc_group_dim,
        );
        self.storage.modular.decode_group(
            &self.storage.header.0,
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
        if self.storage.header.0.encoding == FrameEncoding::VAR_DCT {
            self.storage.modular.decode_ac_metadata(
                &self.storage.header.0,
                g,
                reader,
                self.state,
            )?;
        } else if self.storage.header.0.loop_filter.epf_iters > 0 {
            super::image_ops::fill_image(
                super::epf::INV_SIGMA_NUM / self.storage.header.0.loop_filter.epf_sigma_for_modular,
                &mut self.state.sigma.borrow_mut(),
            );
        }
        self.storage.dc_groups[g] = 1;
        Ok(())
    }
    fn prepare_pipeline(&mut self) -> Status {
        if self.storage.header.0.can_be_referenced() {
            if let Some(events) = &self.storage.write_trace {
                events.borrow_mut().push(FrameWriteEvent::BundleClear {
                    target: FrameWriteTarget::ReferenceStorage,
                });
            }
            *self.state.frame_storage_for_referencing.borrow_mut() = ImageBundle::new(Some(
                &self.storage.header.0.nonserialized_metadata.unwrap().m,
            ));
        }
        // PreparePipeline with use_slow_render_pipeline=true, coalescing=true,
        // render_spotcolors=true, render_noise=true. Header admission proves
        // only the explicitly connected filter/transform branches are reachable.
        let num_c = 3 + self
            .storage
            .header
            .0
            .nonserialized_metadata
            .unwrap()
            .m
            .num_extra_channels as usize;
        let render_noise = self.storage.header.0.flags & super::frame_header::FLAG_NOISE != 0;
        let mut builder =
            SimpleRenderPipelineBuilder::new(num_c + if render_noise { 3 } else { 0 });
        if !self.storage.header.0.chroma_subsampling.is_444() {
            for c in 0..3 {
                if self.storage.header.0.chroma_subsampling.h_shift(c) != 0 {
                    builder.add_stage(Some(
                        super::render_pipeline::stage_chroma_upsampling::get_chroma_upsampling_stage(c, true),
                    ))?;
                }
                if self.storage.header.0.chroma_subsampling.v_shift(c) != 0 {
                    builder.add_stage(Some(
                        super::render_pipeline::stage_chroma_upsampling::get_chroma_upsampling_stage(c, false),
                    ))?;
                }
            }
        }
        if self.storage.header.0.loop_filter.gab {
            builder.add_stage(super::render_pipeline::stage_gaborish::get_gaborish_stage(
                &self.storage.header.0.loop_filter,
            ))?;
        }
        {
            use super::render_pipeline::stage_epf::{EpfStage, get_epf_stage_shared};
            let lf = &self.storage.header.0.loop_filter;
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
        let h = &self.storage.header.0;
        let late_ec_upsample = h.upsampling != 1
            && h.extra_channel_upsampling
                .iter()
                .all(|&up| up == h.upsampling);
        if !late_ec_upsample {
            for (ec, &up) in h.extra_channel_upsampling.iter().enumerate() {
                if up != 1 {
                    builder.add_stage(
                        super::render_pipeline::stage_upsampling::get_upsampling_stage(
                            &h.nonserialized_metadata.unwrap().transform_data,
                            3 + ec,
                            super::base::bits::ceil_log2_nonzero(up),
                        ),
                    )?;
                }
            }
        }
        if self.storage.header.0.flags & super::frame_header::FLAG_PATCHES != 0 {
            builder.add_stage(Some(
                super::render_pipeline::stage_patches::get_patches_stage_shared(
                    self.state.shared().image_features.patches.clone(),
                    self.state.shared().reference_frames.clone(),
                    &self
                        .storage
                        .header
                        .0
                        .nonserialized_metadata
                        .unwrap()
                        .m
                        .extra_channel_info,
                ),
            ))?;
        }
        if self.storage.header.0.flags & super::frame_header::FLAG_SPLINES != 0 {
            builder.add_stage(Some(Box::new(
                super::render_pipeline::stage_splines::get_spline_stage_shared(
                    self.state.shared().image_features.splines.clone(),
                ),
            )))?;
        }
        if h.upsampling != 1 {
            let channels = 3 + if late_ec_upsample {
                h.extra_channel_upsampling.len()
            } else {
                0
            };
            for c in 0..channels {
                builder.add_stage(
                    super::render_pipeline::stage_upsampling::get_upsampling_stage(
                        &h.nonserialized_metadata.unwrap().transform_data,
                        c,
                        super::base::bits::ceil_log2_nonzero(h.upsampling),
                    ),
                )?;
            }
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
        if self.storage.header.0.dc_level != 0 {
            let slot = self.storage.header.0.dc_level as usize - 1;
            let original =
                super::render_pipeline::render_pipeline_stage::WriteToImage3FStage::from_dc_frame(
                    self.state.shared_storage.dc_frames.clone(),
                    slot,
                );
            let writer: Box<
                dyn super::render_pipeline::render_pipeline_stage::RenderPipelineStage,
            > = if let Some(events) = &self.storage.write_trace {
                Box::new(TracedDcWriter {
                    original,
                    events: events.clone(),
                    slot,
                })
            } else {
                Box::new(original)
            };
            builder.add_stage(Some(writer))?;
        }
        if self.storage.header.0.can_be_referenced()
            && self.storage.header.0.save_before_color_transform
        {
            builder.add_stage(Some(trace_bundle_writer(
                Box::new(WriteToImageBundleStage::from_owned(
                    self.state.frame_storage_for_referencing.clone(),
                    &self
                        .storage
                        .header
                        .0
                        .nonserialized_metadata
                        .unwrap()
                        .m
                        .color_encoding,
                )),
                &self.storage.write_trace,
                FrameWriteTarget::ReferenceStorage,
            )))?;
        }
        let info = &self.state.output_encoding_info;
        let tone = get_tone_mapping_stage(info);
        assert!(
            tone.is_none(),
            "closed metadata has equal source/desired intensity"
        );
        let mut linear = false;
        if self.storage.header.0.color_transform == ColorTransform::YCBCR {
            builder.add_stage(Some(super::render_pipeline::stage_ycbcr::get_ycbcr_stage()))?;
        } else if self.storage.header.0.color_transform == ColorTransform::XYB {
            builder.add_stage(Some(super::render_pipeline::stage_xyb::get_xyb_stage(info)))?;
            linear = true;
        }
        if super::blending::needs_blending(&self.storage.header.0) {
            if linear {
                builder.add_stage(Some(
                    super::render_pipeline::stage_from_linear::get_srgb_from_linear_stage(),
                ))?;
                linear = false;
            }
            builder.add_stage(Some(
                super::render_pipeline::stage_blending::get_blending_stage_shared(
                    self.storage.header.0.clone(),
                    self.state,
                    &info.color_encoding,
                ),
            ))?;
        }
        if self.storage.header.0.can_be_referenced()
            && !self.storage.header.0.save_before_color_transform
        {
            if linear {
                builder.add_stage(Some(
                    super::render_pipeline::stage_from_linear::get_srgb_from_linear_stage(),
                ))?;
                linear = false;
            }
            builder.add_stage(Some(trace_bundle_writer(
                Box::new(WriteToImageBundleStage::from_owned(
                    self.state.frame_storage_for_referencing.clone(),
                    &self
                        .storage
                        .header
                        .0
                        .nonserialized_metadata
                        .unwrap()
                        .m
                        .color_encoding,
                )),
                &self.storage.write_trace,
                FrameWriteTarget::ReferenceStorage,
            )))?;
        }
        // dec_cache.cc retains coefficients in the immutable metadata owner.
        // The reference writer above observes color before spot rendering.
        for (ec, eci) in self
            .storage
            .header
            .0
            .nonserialized_metadata
            .unwrap()
            .m
            .extra_channel_info
            .iter()
            .enumerate()
        {
            if eci.channel_type == super::image_metadata::ExtraChannel::SPOT_COLOR {
                builder.add_stage(Some(
                    super::render_pipeline::stage_spot::get_spot_color_stage(ec, &eci.spot_color),
                ))?;
            }
        }
        if linear {
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
        let writer = match &self.storage.output {
            FrameOutput::Borrowed(output) => {
                WriteToImageBundleStage::from_shared(output, &info.color_encoding)
            }
            FrameOutput::Owned(output) => {
                WriteToImageBundleStage::from_owned(output.clone(), &info.color_encoding)
            }
        };
        builder.add_stage(Some(trace_bundle_writer(
            Box::new(writer),
            &self.storage.write_trace,
            FrameWriteTarget::Output,
        )))?;
        self.state.render_pipeline = Some(builder.finalize(self.state.shared().frame_dim)?);
        self.state
            .render_pipeline
            .as_ref()
            .unwrap()
            .is_initialized()
    }
    fn allocate_output(&mut self) -> Status {
        if self.storage.allocated {
            return Ok(());
        }
        self.storage.modular.maybe_drop_full_image();
        self.storage.output.borrow_mut().origin = self.storage.header.0.frame_origin;
        self.state
            .init_for_ac(self.storage.header.0.passes.num_passes as usize)?;
        self.storage.allocated = true;
        Ok(())
    }
    fn prepare_storage(&mut self, threads: usize, tasks: usize) -> Status {
        let size = threads.min(tasks);
        if size > self.storage.group_caches.len() {
            self.storage
                .group_caches
                .resize_with(size, GroupDecCache::default);
        }
        self.storage.use_task_id = threads > tasks;
        // Original full-image VarDCT/noise paths retain group input owners.
        if let Some(p) = &mut self.state.render_pipeline {
            let use_group_ids = self.storage.modular.uses_full_image()
                && (self.storage.header.0.encoding == FrameEncoding::VAR_DCT
                    || self.storage.header.0.flags & super::frame_header::FLAG_NOISE != 0);
            p.prepare_for_threads(size, use_group_ids)?;
        }
        self.state
            .upsampler8x
            .as_mut()
            .unwrap()
            .prepare_for_threads(threads)
    }
    fn process_ac_global(&mut self, reader: &mut BitReader<'_>) -> Status {
        if !self.storage.finalized_dc {
            return Err(StatusCode::GenericError);
        }
        if self.storage.header.0.encoding == FrameEncoding::VAR_DCT {
            let used = self
                .state
                .used_acs
                .load(std::sync::atomic::Ordering::SeqCst);
            let shared = &mut self.state.shared_storage;
            shared
                .matrices
                .decode(reader, Some(&self.storage.modular))?;
            shared.matrices.ensure_computed(used)?;
            let bits = super::base::bits::ceil_log2_nonzero(self.storage.dim.num_groups);
            shared.num_histograms = 1 + reader.read_bits(bits) as usize;
            self.state
                .code
                .resize_with(super::common::MAX_NUM_PASSES, Default::default);
            self.state
                .context_map
                .resize_with(super::common::MAX_NUM_PASSES, Vec::new);
            let mut max_bits = 0usize;
            for pass in 0..self.storage.header.0.passes.num_passes as usize {
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
            max_bits +=
                super::base::bits::ceil_log2_nonzero(self.storage.header.0.passes.num_passes);
            let kind = if max_bits < 16 {
                super::dct_util::AcType::I16
            } else {
                super::dct_util::AcType::I32
            };
            let store = self.storage.header.0.passes.num_passes > 1;
            self.state.coefficients = Some(super::dct_util::AcImage::make(
                kind,
                if store {
                    super::frame_dimensions::GROUP_DIM * super::frame_dimensions::GROUP_DIM
                } else {
                    0
                },
                if store {
                    self.storage.dim.num_groups
                } else {
                    0
                },
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
        let d = &self.storage.dim;
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
        let pass0 = self.storage.passes[g] as usize;
        if self.storage.header.0.encoding == FrameEncoding::VAR_DCT {
            self.storage.group_caches[thread].init_once(
                self.storage.header.0.passes.num_passes as usize,
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
                &self.storage.header.0,
                &mut pass_readers,
                std::num::NonZeroUsize::new(count).unwrap(),
                g,
                group_state,
                &mut self.storage.group_caches[thread],
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
            let (min, max) = self.storage.header.0.passes.get_downsampling_bracket(i);
            let mut pass_ready = true;
            self.storage.modular.decode_group(
                &self.storage.header.0,
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
        self.storage.passes[g] = self.storage.passes[g].wrapping_add(count as u8);
        if self.storage.header.0.flags & super::frame_header::FLAG_NOISE != 0 {
            super::dec_noise::prepare_noise_input(
                self.state.visible_frame_index,
                self.state.nonvisible_frame_index,
                &self.storage.dim,
                &self.storage.header.0,
                g,
                &mut input,
            );
        }
        if !self.storage.modular.uses_full_image() && should_run_pipeline && ready {
            input.done()?;
        }
        Ok(())
    }
    fn mark_sections(&mut self, sections: &[Section<'_>], statuses: &[SectionStatus]) {
        self.storage.num_sections_done =
            self.storage.num_sections_done.wrapping_add(sections.len());
        for (section, status) in sections.iter().zip(statuses) {
            if *status != SectionStatus::Done {
                self.storage.processed[section.id] = 0;
                self.storage.num_sections_done = self.storage.num_sections_done.wrapping_sub(1);
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
        let mut dc = vec![num; self.storage.dim.num_dc_groups];
        let mut ac = vec![
            vec![num; self.storage.header.0.passes.num_passes as usize];
            self.storage.dim.num_groups
        ];
        let mut desired = vec![0; self.storage.dim.num_groups];
        let single =
            self.storage.dim.num_groups == 1 && self.storage.header.0.passes.num_passes == 1;
        if single {
            if num != 1 || sections[0].id != 0 {
                return Err(StatusCode::GenericError);
            }
            if self.storage.processed[0] == 0 {
                self.storage.processed[0] = 1;
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
            let ac_global_index = self.storage.dim.num_dc_groups + 1;
            for i in 0..num {
                let id = sections[i].id;
                if id >= self.storage.processed.len() {
                    return Err(StatusCode::GenericError);
                }
                if self.storage.processed[id] != 0 {
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
                    let g = idx % self.storage.dim.num_groups;
                    let p = idx / self.storage.dim.num_groups;
                    if p >= self.storage.header.0.passes.num_passes as usize {
                        return Err(StatusCode::GenericError);
                    }
                    ac[g][p] = i;
                }
                self.storage.processed[id] = 1;
            }
            for g in 0..ac.len() {
                let mut j = 0;
                while j + (self.storage.passes[g] as usize)
                    < self.storage.header.0.passes.num_passes as usize
                {
                    if ac[g][j + self.storage.passes[g] as usize] == num {
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
        if self.storage.decoded_dc_global {
            for i in 0..dc.len() {
                if dc[i] != num {
                    self.process_dc_group(i, &mut sections[dc[i]].reader)?;
                    statuses[dc[i]] = SectionStatus::Done;
                }
            }
        }
        if !self.storage.dc_groups.contains(&0) && !self.storage.finalized_dc {
            self.prepare_pipeline()?;
            if self.storage.header.0.encoding == FrameEncoding::VAR_DCT
                && self.storage.header.0.flags & FLAG_SKIP_ADAPTIVE_DC_SMOOTHING == 0
                && self.storage.header.0.flags & super::frame_header::FLAG_USE_DC_FRAME == 0
            {
                let shared = &mut self.state.shared_storage;
                super::compressed_dc::adaptive_dc_smoothing(
                    shared.quantizer.mul_dc(),
                    &mut shared.dc_storage,
                    &self.storage.header.0.chroma_subsampling,
                )?;
            }
            self.storage.finalized_dc = true;
            self.allocate_output()?;
        }
        if self.storage.finalized_dc && ac_global != num && !self.storage.decoded_ac_global {
            self.process_ac_global(&mut sections[ac_global].reader)?;
            self.storage.decoded_ac_global = true;
            statuses[ac_global] = SectionStatus::Done;
        }
        if self.storage.decoded_ac_global {
            for (g, &count) in desired.iter().enumerate() {
                if count != 0 {
                    self.state.render_pipeline.as_mut().unwrap().clear_done(g);
                }
            }
            self.prepare_storage(1, self.storage.passes.len())?;
            for g in 0..ac.len() {
                if desired[g] == 0 {
                    continue;
                }
                let first = self.storage.passes[g] as usize;
                for i in 0..desired[g] {
                    if ac[g][first + i] == num {
                        return Err(StatusCode::GenericError);
                    }
                }
                let thread = if self.storage.use_task_id { g } else { 0 };
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
        if self.storage.is_finalized {
            return Err(StatusCode::GenericError);
        }
        self.storage.is_finalized = true;
        self.storage.modular.finalize_decoding(
            &self.storage.header.0,
            &self.state.shared_storage,
            self.state.render_pipeline.as_mut().unwrap(),
            true,
        )?;
        if self.storage.header.0.can_be_referenced() {
            let mut references = self.state.shared_storage.reference_frames.borrow_mut();
            let info = &mut references[self.storage.header.0.save_as_reference as usize];
            info.frame
                .as_deref_mut()
                .expect("source requires an existing reference bundle")
                .move_from(&mut self.state.frame_storage_for_referencing.borrow_mut());
            if let Some(events) = &self.storage.write_trace {
                events
                    .borrow_mut()
                    .push(FrameWriteEvent::BundleMoveToReference {
                        slot: self.storage.header.0.save_as_reference as usize,
                    });
            }
            info.ib_is_in_xyb = self.storage.header.0.save_before_color_transform;
        }
        Ok(())
    }
    fn has_everything(&self) -> bool {
        self.storage.decoded_dc_global
            && self.storage.decoded_ac_global
            && !self.storage.dc_groups.contains(&0)
            && self
                .storage
                .passes
                .iter()
                .all(|&pass| pass >= self.storage.header.0.passes.num_passes as u8)
    }
    fn references(&self) -> i32 {
        if self.storage.is_finalized || !self.has_everything() {
            return 0;
        }
        let h = &self.storage.header.0;
        let mut result = 0;
        if h.frame_type == FrameType::REGULAR || h.frame_type == FrameType::SKIP_PROGRESSIVE {
            let cropped = h.custom_size_or_origin;
            if cropped || h.blending_info.mode != BlendMode::REPLACE {
                result |= 1 << h.blending_info.source;
            }
            for info in &h.extra_channel_blending_info {
                if cropped || info.mode != BlendMode::REPLACE {
                    result |= 1 << info.source;
                }
            }
        }
        if h.flags & super::frame_header::FLAG_PATCHES != 0 {
            result |= self
                .state
                .shared()
                .image_features
                .patches
                .borrow()
                .get_references();
        }
        if h.flags & super::frame_header::FLAG_USE_DC_FRAME != 0 {
            result |= 16 << h.dc_level;
        }
        result
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
        decode_profile_frame_into(
            self.metadata(),
            bytes,
            state,
            output,
            copied_header,
            DecodeProfile::LosslessThunder,
        )
    }
}

/// Defined reader/section checkpoints of the original DecodeFrame caller.
/// An absent status denotes a source operation that was never reached.
#[derive(Default)]
pub struct FrameDecodeTrace {
    pub init_status: Option<Status>,
    pub output_status: Option<Status>,
    pub header_bits: usize,
    pub header_bounds: Option<Status>,
    pub header_close: Option<Status>,
    pub sections_status: Option<Status>,
    pub section_statuses: Vec<u8>,
    pub section_readers: Vec<(usize, Status)>,
    pub references: Option<i32>,
    pub has_decoded_dc: bool,
    pub has_decoded_all: bool,
    pub finalize_status: Option<Status>,
    pub write_events: Vec<FrameWriteEvent>,
}

pub struct SequenceFrameDecode<'m> {
    pub status: Status,
    pub header: Option<FrameHeader<'m>>,
    pub trace: FrameDecodeTrace,
}

/// The shared-state loop around the original DecodeFrame/InitFrame boundary.
/// Metadata is supplied once and borrowed unchanged by all frames. The output
/// and every retained pipeline writer share one bundle allocation owner.
/// This boundary admits sRGB NONE/XYB frames and every parsed YCbCr channel mode,
/// references, DC frames, animation and previews, including source early/late
/// extra-channel and main spatial upsampling, associated/subsampled alpha,
/// and all source-defined extra-channel types. Orientation/intrinsic metadata
/// stays attached to the bundle; external conversion applies orientation.
/// Other color configurations remain separate boundaries.
pub struct SequenceDecoder<'m> {
    state: PassesDecoderState<'m, 'm>,
    output: Rc<RefCell<ImageBundle<'m>>>,
    metadata: &'m super::image_metadata::CodecMetadata,
}
impl<'m> SequenceDecoder<'m> {
    pub fn new(metadata: &'m super::image_metadata::CodecMetadata) -> Result<Self, StatusCode> {
        use super::color_encoding::RenderingIntent;
        let m = &metadata.m;
        if !(1..=32).contains(&m.bit_depth.bits_per_sample)
            || m.num_extra_channels as usize != m.extra_channel_info.len()
            || m.color_encoding.want_icc()
            || !m.color_encoding.is_srgb()
            || m.color_encoding.to_external().rendering_intent != RenderingIntent::RELATIVE
            || m.tone_mapping.intensity_target != 255.0
            || m.tone_mapping.min_nits != 0.0
            || m.tone_mapping.relative_to_max_display
            || m.tone_mapping.linear_below != 0.0
        {
            return Err(StatusCode::Unsupported);
        }
        let mut state = PassesDecoderState::new();
        state.output_encoding_info.set_from_metadata(metadata)?;
        Ok(Self {
            state,
            output: Rc::new(RefCell::new(ImageBundle::new(Some(&metadata.m)))),
            metadata,
        })
    }
    pub fn decode_frame(&mut self, bytes: &[u8], preview: bool) -> SequenceFrameDecode<'m> {
        let mut header = None;
        let mut trace = FrameDecodeTrace::default();
        let status = decode_frame_into(
            self.metadata,
            bytes,
            &mut self.state,
            FrameOutput::Owned(self.output.clone()),
            &mut header,
            DecodeProfile::Sequence { preview },
            Some(&mut trace),
        );
        SequenceFrameDecode {
            status,
            header,
            trace,
        }
    }
    pub fn output(&self) -> std::cell::Ref<'_, ImageBundle<'m>> {
        self.output.borrow()
    }
    pub fn state(&self) -> &PassesDecoderState<'m, 'm> {
        &self.state
    }
}

/// Original FrameDecoder::SavedAs. A DC header has a parsed level in 1..=4.
pub fn frame_saved_as(header: &FrameHeader<'_>) -> i32 {
    if header.frame_type == FrameType::DC {
        16 << (header.dc_level - 1)
    } else if header.can_be_referenced() {
        1 << header.save_as_reference
    } else {
        0
    }
}

#[derive(Clone, Copy)]
enum DecodeProfile {
    LosslessThunder,
    VarDct,
    Modular,
    Sequence { preview: bool },
}

fn decode_profile_frame_into<'m, 'p>(
    metadata: &'m super::image_metadata::CodecMetadata,
    bytes: &[u8],
    state: &mut PassesDecoderState<'m, 'p>,
    output: &'p RefCell<ImageBundle<'m>>,
    copied_header: &mut Option<FrameHeader<'m>>,
    profile: DecodeProfile,
) -> Status {
    decode_frame_into(
        metadata,
        bytes,
        state,
        FrameOutput::Borrowed(output),
        copied_header,
        profile,
        None,
    )
}

fn decode_frame_into<'m, 'p>(
    metadata: &'m super::image_metadata::CodecMetadata,
    bytes: &[u8],
    state: &mut PassesDecoderState<'m, 'p>,
    output: FrameOutput<'p, 'm>,
    copied_header: &mut Option<FrameHeader<'m>>,
    profile: DecodeProfile,
    mut trace: Option<&mut FrameDecodeTrace>,
) -> Status {
    let write_trace = trace.as_ref().map(|_| Rc::new(RefCell::new(Vec::new())));
    let status = (|| {
        let mut header = FrameHeader::new(Some(metadata));
        let modular = ModularFrameDecoder::new();
        let mut reader = BitReader::new(bytes);
        // InitFrame: header and TOC parsing precede profile admission.
        state.shared_storage.matrices = DequantMatrices::default();
        let initialized = (|| {
            header.nonserialized_is_preview =
                matches!(profile, DecodeProfile::Sequence { preview: true });
            read_frame_header(&mut reader, &mut header)?;
            let dim = header.to_frame_dimensions();
            {
                let mut decoded = output.borrow_mut();
                *decoded.color_mut() = Image3::default();
                decoded.extra_channels_mut().clear();
                if let Some(events) = &write_trace {
                    events.borrow_mut().push(FrameWriteEvent::BundleClear {
                        target: FrameWriteTarget::Output,
                    });
                }
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
        if let Some(trace) = &mut trace {
            trace.init_status = Some(initialized.as_ref().map(|_| ()).map_err(|e| *e));
            trace.header_bits = reader.total_bits_consumed();
        }
        let (dim, toc) = match initialized {
            Ok(v) => v,
            Err(e) => {
                let close = reader.close();
                if let Some(trace) = &mut trace {
                    trace.header_close = Some(close);
                }
                return Err(e);
            }
        };
        let admitted = match match profile {
            DecodeProfile::VarDct => AdmittedHeader::admit_vardct(header),
            DecodeProfile::LosslessThunder => AdmittedHeader::admit(header),
            DecodeProfile::Modular => AdmittedHeader::admit_modular(header),
            DecodeProfile::Sequence { .. } => AdmittedHeader::admit_sequence(header),
        } {
            Ok(v) => v,
            Err(e) => {
                let close = reader.close();
                if let Some(trace) = &mut trace {
                    trace.header_close = Some(close);
                }
                return Err(e);
            }
        };
        let mut storage = FrameDecoderStorage {
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
            write_trace: write_trace.clone(),
        };
        let mut decoder = FrameDecoder {
            state,
            storage: &mut storage,
        };
        let initialized = decoder.init_output();
        if let Some(trace) = &mut trace {
            trace.output_status = Some(initialized);
        }
        if let Err(e) = initialized {
            let close = reader.close();
            if let Some(trace) = &mut trace {
                trace.header_close = Some(close);
            }
            return Err(e);
        }
        *copied_header = Some(decoder.storage.header.0.as_ref().clone());
        let bounds = reader.all_reads_within_bounds();
        if let Some(trace) = &mut trace {
            trace.header_bounds = Some(bounds);
        }
        if let Err(e) = bounds {
            let close = reader.close();
            if let Some(trace) = &mut trace {
                trace.header_close = Some(close);
            }
            return Err(e);
        }
        let header_bytes = reader.total_bits_consumed() / 8;
        let close = reader.close();
        if let Some(trace) = &mut trace {
            trace.header_close = Some(close);
        }
        close?;
        let mut processed_bytes = header_bytes;
        let mut sections = Vec::new();
        let mut pos = header_bytes;
        let work = (|| {
            for (index, entry) in decoder.storage.toc.iter().enumerate() {
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
            let status = decoder.process_sections(&mut sections, &mut statuses);
            if let Some(trace) = &mut trace {
                trace.sections_status = Some(status);
                trace.section_statuses = statuses.iter().map(|status| *status as u8).collect();
            }
            status?;
            for (i, status) in statuses.iter().enumerate() {
                if *status != SectionStatus::Done {
                    return Err(StatusCode::GenericError);
                }
                processed_bytes = processed_bytes.wrapping_add(decoder.storage.toc[i].size);
            }
            Ok(())
        })();
        let mut close_ok = Ok(());
        if let Some(trace) = &mut trace {
            trace.section_readers.resize(sections.len(), (0, Ok(())));
        }
        for section in sections.iter_mut().rev() {
            // index is retained at its original physical-order checkpoint.
            let _ = section.index;
            let bits = section.reader.total_bits_consumed();
            let closed = section.reader.close();
            if let Some(trace) = &mut trace {
                trace.section_readers[section.index] = (bits, closed);
            }
            if let Err(e) = closed {
                close_ok = Err(e);
            }
        }
        work?;
        close_ok?;
        if let Some(trace) = &mut trace {
            trace.references = Some(decoder.references());
            trace.has_decoded_dc = decoder.storage.finalized_dc;
            trace.has_decoded_all = decoder.storage.toc.len() == decoder.storage.num_sections_done;
        }
        let finalized = decoder.finalize_frame();
        if let Some(trace) = &mut trace {
            trace.finalize_status = Some(finalized);
        }
        finalized?;
        decoder
            .storage
            .output
            .borrow_mut()
            .set_decoded_bytes(processed_bytes);
        Ok(())
    })();
    if let Some(trace) = trace {
        trace.write_events = write_trace.unwrap().borrow().clone();
    }
    status
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
        decode_profile_frame_into(
            self.metadata(),
            bytes,
            state,
            output,
            header,
            DecodeProfile::VarDct,
        )
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

/// Explicit original external-output choice. SourceInteger retains the stored
/// integer depth (the original exporter accepts 1..16); Float32 exports native
/// binary32 samples for larger integer depths and custom floating samples.
#[derive(Clone, Copy)]
pub enum ModularOutput {
    SourceInteger,
    Float32,
}
/// Complete single final regular Modular codestream result. The metadata/header
/// owner admits NONE/XYB sRGB and same-depth full-resolution unassociated alpha.
/// References, chroma subsampling, spatial transforms and ICC remain separate.
pub type ModularImageDecode = VarDctImageDecode;

struct ModularConfig {
    metadata: super::image_metadata::CodecMetadata,
}
impl ModularConfig {
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
        if !(1..=32).contains(&m.bit_depth.bits_per_sample)
            || m.num_extra_channels as usize != m.extra_channel_info.len()
            || m.extra_channel_info.len() > 1
            || m.extra_channel_info.iter().any(|ec| {
                ec.channel_type != ExtraChannel::ALPHA
                    || ec.bit_depth.floating_point_sample != m.bit_depth.floating_point_sample
                    || ec.bit_depth.bits_per_sample != m.bit_depth.bits_per_sample
                    || ec.bit_depth.exponent_bits_per_sample != m.bit_depth.exponent_bits_per_sample
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
        decode_profile_frame_into(
            self.metadata(),
            bytes,
            state,
            output,
            header,
            DecodeProfile::Modular,
        )
    }
}

/// Decode the admitted general Modular profile through original group decoding,
/// inverse transforms, rendering, scoped reader close and explicit export.
pub fn decode_modular(bytes: &[u8], output_format: ModularOutput) -> ModularImageDecode {
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
        ModularConfig::admit_decoded_metadata(&mut metadata)
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
        let (bits, fp) = match output_format {
            ModularOutput::SourceInteger => (m.m.bit_depth.bits_per_sample as usize, false),
            ModularOutput::Float32 => (32, true),
        };
        status = (|| {
            let stride = m
                .xsize()
                .checked_mul(n)
                .and_then(|v| v.checked_mul(bits.div_ceil(8)))
                .ok_or(StatusCode::GenericError)?;
            let size = stride
                .checked_mul(m.ysize())
                .ok_or(StatusCode::GenericError)?;
            pixels.resize(size, 0);
            let convert = if fp {
                super::dec_external_image::convert_float_to_buffer
            } else {
                convert_uint_to_buffer
            };
            convert(
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
    ModularImageDecode {
        status,
        metadata: metadata.unwrap(),
        pixels,
        float_pixels,
        reader_bits,
        frame_size,
        frame_decoded_bytes,
    }
}
