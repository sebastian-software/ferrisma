// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! DecodeFrame and the complete incremental FrameDecoder from dec_frame.{h,cc},
//! with closed lossless Thunder and first VarDCT raw-codestream profiles plus an
//! admitted shared-state sequence boundary. FrameDecoder exposes the original
//! InitFrame/InitFrameOutput, ProcessSections (partial input, permuted section
//! order, progressive pauses), Flush, FinalizeFrame, References/SavedAs, TOC,
//! pass-completion and output-descriptor methods used by decode.cc.
//!
//! Language-required deviations:
//! - Frame-owned fields live in `FrameDecoder`; each call borrows the original
//!   `PassesDecoderState` pointer target explicitly (ADR 0003), so no exclusive
//!   borrow of codec state outlives a call. `decoded_` is an `Rc<RefCell>`
//!   bundle owner shared with the retained pipeline writers.
//! - The frame's pool is the source's retained `pool_` pointer: null for the
//!   internal fixed profiles, the public decoder's pool otherwise. RunOnPool
//!   task bodies borrow non-thread-safe frame owners (`Rc`/`RefCell` and
//!   exclusive borrows, ADR 0003), so `run_on_pool_exclusive` serializes them
//!   while the runner chooses the init thread count, threads and task order.
//! - The decoder's JxlMemoryManager is supplied to `FrameDecoder::new`, because
//!   the codec state is lent per call: it is the modular frame decoder's
//!   manager and the WriteToOutputStage scratch manager, as the source's
//!   `dec_state_->memory_manager()`. The codec state's own owners use the
//!   manager it was constructed with.
//! - The constructor's use_slow_rendering_pipeline flag selects the simple
//!   or low-memory pipeline; the fixed profiles select the simple one.
//! - Private metadata/header admission excludes unported configurations before
//!   output initialization; Unsupported is an admission result, not a general
//!   decoder status. Admitted paths retain original section scheduling, DC/AC
//!   setup, scalar reconstruction, output and reader-close checkpoints.
//! - InitFrame parses into a fresh header owner initialized like the
//!   constructor's; earlier pipelines may still retain the previous header.
//! - JXL_HIGH_PRECISION=1 (the pinned default) compiles out the fast XYB->sRGB8
//!   branch of SetImageOutput/PreparePipeline.
use super::base::data_parallel::{
    ThreadPool, run_on_pool_exclusive, run_on_pool_exclusive_no_init,
};
use super::base::rect::Rect;
use super::base::status::{Status, StatusCode};
use super::cms_interface::JxlCmsInterface;
use super::color_encoding_internal::ColorEncoding;
use super::dec_bit_reader::BitReader;
use super::dec_cache::{
    GroupDecCache, ImageOutput, PassesDecoderState, PipelineOptions, PixelCallback,
    add_color_transform_stage, add_from_linear_stage_before_write, add_output_color_stages,
};
use super::dec_modular::{ModularFrameDecoder, ModularStreamId};
use super::decode::JxlProgressiveDetail;
use super::enc_frame::LosslessThunderConfig;
use super::frame_dimensions::FrameDimensions;
use super::frame_header::{
    BlendMode, ColorTransform, FLAG_SKIP_ADAPTIVE_DC_SMOOTHING, FrameEncoding, FrameHeader,
    FrameType, read_frame_header,
};
use super::image::Image3;
use super::image_bundle::ImageBundle;
use super::memory_manager::JxlMemoryManager;
use super::memory_manager_internal::MemoryManagerPtr;
use super::passes_state::initialize_passes_shared_state;
use super::quant_weights::DequantMatrices;
use super::render_pipeline::render_pipeline::RenderPipelineBuilder;
use super::render_pipeline::render_pipeline_stage::WriteToImageBundleStage;
use super::toc::{num_toc_entries, read_toc};
use super::types::{JxlDataType, JxlPixelFormat};
use std::cell::RefCell;
use std::ffi::c_void;
use std::rc::Rc;

pub struct LosslessThunderFrameDecode<'m> {
    pub status: Status,
    pub image: ImageBundle<'m>,
    // DecodeFrame copies this only after successful InitFrameOutput.
    pub header: Option<FrameHeader<'m>>,
}
/// FrameDecoder::SectionInfo. The reader borrows only this section's bytes.
pub struct SectionInfo<'r, 'b> {
    pub br: &'r mut BitReader<'b>,
    /// Logical section index, independent of TOC permutation or file position.
    pub id: usize,
    /// Index of the section in the byte order of the frame.
    pub index: usize,
}
/// FrameDecoder::TocEntry.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TocEntry {
    pub size: usize,
    pub id: usize,
}
/// FrameDecoder::SectionStatus with its original values. Default mirrors the
/// caller's value-initialized element (kDone), which ProcessSections overwrites.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(u8)]
pub enum SectionStatus {
    /// Processed correctly.
    #[default]
    Done = 0,
    /// Skipped because other required sections were not yet processed.
    Skipped = 1,
    /// Skipped because the section was already processed.
    Duplicate = 2,
    /// Only partially decoded: the section must be processed again.
    Partial = 3,
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

// Owns each parsed header. Admission bounds the implemented source paths; the
// decoder processes a header only after its profile predicate succeeds.
// Fixed configurations own their metadata; sequential metadata is borrowed
// immutably for the decoder lifetime.
struct AdmittedHeader<'m>(Rc<FrameHeader<'m>>);
impl AdmittedHeader<'_> {
    // FrameHeader::VisitFields reads the complete U64 flag field. The original
    // decoder tests individual known bits and accepts reserved bits unchanged;
    // rejecting those here would precede its output initialization and errors.
    fn admit_sequence(h: &FrameHeader<'_>) -> Status {
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
        Ok(())
    }
    fn admit(h: &FrameHeader<'_>) -> Status {
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
        Ok(())
    }
    fn admit_modular(h: &FrameHeader<'_>) -> Status {
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
        Ok(())
    }
    fn admit_vardct(h: &FrameHeader<'_>) -> Status {
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
        Ok(())
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
pub type FrameWriteTrace = Rc<RefCell<Vec<FrameWriteEvent>>>;
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
/// FrameDecoder from dec_frame.h. Source frame fields retain their owner
/// independently of each call's `PassesDecoderState` loan (the source's
/// `dec_state_` pointer). The existing frame fields keep their relative order.
/// `'m` is the codestream metadata lifetime; `'p` that of a borrowed output.
pub struct FrameDecoder<'m, 'p> {
    // Supplied explicitly; see the module deviations. Read only by the
    // WriteToOutputStage factory, under FrameDecoder::new's contract.
    memory_manager: *const JxlMemoryManager,
    metadata: &'m super::image_metadata::CodecMetadata,
    // pool_; null or valid under FrameDecoder::new's contract.
    pool: *const ThreadPool,
    // decoded_; set by InitFrame.
    output: Option<FrameOutput<'p, 'm>>,
    header: AdmittedHeader<'m>,
    // Whether `header` passed the profile admission after a successful InitFrame.
    admitted: bool,
    dim: FrameDimensions,
    modular: ModularFrameDecoder,
    render_spotcolors: bool,
    coalescing: bool,
    toc: Vec<TocEntry>,
    section_sizes_sum: u64,
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
    use_slow_rendering_pipeline: bool,
    progressive_detail: JxlProgressiveDetail,
    passes_to_pause: Vec<i32>,
    profile: DecodeProfile,
    write_trace: Option<FrameWriteTrace>,
}
// The processing scope lends disjoint frame and codec state. This view owns
// no frame payload and ends before the caller accesses either owner again.
struct FrameDecoderScope<'s, 'm, 'p> {
    state: &'s mut PassesDecoderState<'m, 'p>,
    storage: &'s mut FrameDecoder<'m, 'p>,
}
impl<'m, 'p> FrameDecoder<'m, 'p> {
    /// The retained pool; its lifetime is not tied to this borrow, so that
    /// RunOnPool may lend the decoder exclusively.
    fn pool<'x>(&self) -> Option<&'x ThreadPool> {
        // SAFETY: FrameDecoder::new's caller keeps a non-null pool valid and
        // unmoved while this decoder is used, and never mutates it meanwhile.
        unsafe { self.pool.as_ref() }
    }
    fn get_storage_location(&self, thread: usize, task: usize) -> usize {
        if self.use_task_id {
            return task;
        }
        thread
    }
    fn decoded(&self) -> &RefCell<ImageBundle<'m>> {
        self.output
            .as_deref()
            .expect("source InitFrame sets the decoded bundle")
    }
    /// The owner of the decoded bundle released it. The source's `decoded_`
    /// pointer then dangles and is not used until the next InitFrame; this
    /// handle is dropped so that the bundle is freed at the owner's checkpoint.
    pub fn release_output(&mut self) {
        self.output = None;
    }
}
impl Drop for FrameDecoder<'_, '_> {
    /// The implicit destructor's reverse member order for the owners that
    /// hold memory-manager allocations: the group caches (last first) before
    /// the modular frame decoder.
    fn drop(&mut self) {
        super::base::libcpp_vector::clear(&mut self.group_caches);
        drop(std::mem::take(&mut self.modular));
    }
}

impl FrameDecoderScope<'_, '_, '_> {
    fn init_output(&mut self) -> Status {
        let h = &self.storage.header.0;
        initialize_passes_shared_state(h, &mut self.state.shared_storage, false)?;
        self.state.init(h)?;
        self.storage.modular.init(&self.storage.dim);
        if let Some(jpeg) = self.storage.decoded().borrow_mut().jpeg_data.as_deref_mut() {
            if h.encoding == FrameEncoding::MODULAR {
                return Err(StatusCode::GenericError);
            }
            let num_components = jpeg.components.len();
            if num_components != 1 && num_components != 3 {
                return Err(StatusCode::GenericError);
            }
            if h.nonserialized_metadata.unwrap().m.xyb_encoded {
                return Err(StatusCode::GenericError);
            }
            let map = super::frame_header::jpeg_order(ColorTransform::YCBCR, num_components == 1);
            jpeg.width = self.storage.dim.xsize as i32;
            jpeg.height = self.storage.dim.ysize as i32;
            for (c, &index) in map.iter().enumerate().take(num_components) {
                let component = &mut jpeg.components[index as usize];
                component.width_in_blocks =
                    (self.storage.dim.xsize_blocks >> h.chroma_subsampling.h_shift(c)) as u32;
                component.height_in_blocks =
                    (self.storage.dim.ysize_blocks >> h.chroma_subsampling.v_shift(c)) as u32;
                component.h_samp_factor = 1 << h.chroma_subsampling.raw_h_shift(c);
                component.v_samp_factor = 1 << h.chroma_subsampling.raw_v_shift(c);
                let blocks =
                    component.width_in_blocks as usize * component.height_in_blocks as usize;
                component
                    .coeffs
                    .resize(blocks * super::frame_dimensions::DCT_BLOCK_SIZE, 0);
            }
        }
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
        let memory_manager = shared.memory_manager();
        if self.storage.header.0.flags & super::frame_header::FLAG_PATCHES != 0 {
            let mut uses_extra_channels = false;
            shared.image_features.patches.borrow_mut().decode(
                memory_manager,
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
            super::entropy_coder::decode_block_ctx_map(
                memory_manager,
                reader,
                &mut shared.block_ctx_map,
            )?;
            shared.cmap.decode_dc(reader)?;
            if self.storage.decoded().borrow().jpeg_data.is_some() {
                shared.quantizer.clear_dc_mul();
            }
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
    fn prepare_pipeline(&mut self, options: PipelineOptions) -> Status {
        if self.storage.header.0.can_be_referenced() {
            if let Some(events) = &self.storage.write_trace {
                events.borrow_mut().push(FrameWriteEvent::BundleClear {
                    target: FrameWriteTarget::ReferenceStorage,
                });
            }
            // The source move assignment, with its member release order.
            let _ = self
                .state
                .frame_storage_for_referencing
                .borrow_mut()
                .move_from(&mut ImageBundle::new_in(
                    self.state.memory_manager(),
                    Some(&self.storage.header.0.nonserialized_metadata.unwrap().m),
                ));
        }
        // PassesDecoderState::PreparePipeline with the simple implementation.
        // Header admission proves only the explicitly connected filter and
        // transform branches are reachable.
        let num_c = 3 + self
            .storage
            .header
            .0
            .nonserialized_metadata
            .unwrap()
            .m
            .num_extra_channels as usize;
        let render_noise = options.render_noise
            && self.storage.header.0.flags & super::frame_header::FLAG_NOISE != 0;
        let memory_manager = self.state.memory_manager();
        let mut builder =
            RenderPipelineBuilder::new_in(memory_manager, num_c + if render_noise { 3 } else { 0 });
        if options.use_slow_render_pipeline {
            builder.use_simple_implementation();
        }
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
                        super::render_pipeline::stage_upsampling::get_upsampling_stage_in(
                            memory_manager,
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
                    super::render_pipeline::stage_upsampling::get_upsampling_stage_in(
                        memory_manager,
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
                )
                .with_memory_manager(memory_manager);
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
        // Color selection from dec_cache.cc, with the original interleaved
        // blending and reference writers.
        let mut linear = add_color_transform_stage(
            self.storage.header.0.color_transform,
            &self.state.output_encoding_info,
            &mut builder,
        )?;
        if options.coalescing && super::blending::needs_blending(&self.storage.header.0) {
            add_from_linear_stage_before_write(
                &self.state.output_encoding_info,
                &mut builder,
                &mut linear,
            )?;
            builder.add_stage(Some(
                super::render_pipeline::stage_blending::get_blending_stage_shared(
                    self.storage.header.0.clone(),
                    self.state,
                    &self.state.output_encoding_info.color_encoding,
                ),
            ))?;
        }
        if options.coalescing
            && self.storage.header.0.can_be_referenced()
            && !self.storage.header.0.save_before_color_transform
        {
            add_from_linear_stage_before_write(
                &self.state.output_encoding_info,
                &mut builder,
                &mut linear,
            )?;
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
        let m = &self.storage.header.0.nonserialized_metadata.unwrap().m;
        if options.render_spotcolors
            && m.find(super::image_metadata::ExtraChannel::SPOT_COLOR)
                .is_some()
        {
            for (ec, eci) in m.extra_channel_info.iter().enumerate() {
                // Not Find: there may be multiple spot color channels.
                if eci.channel_type == super::image_metadata::ExtraChannel::SPOT_COLOR {
                    builder.add_stage(Some(
                        super::render_pipeline::stage_spot::get_spot_color_stage(
                            ec,
                            &eci.spot_color,
                        ),
                    ))?;
                }
            }
        }
        // SAFETY: Internal decoder states are private to this module.
        // The fixed profiles never set cms_set. SequenceDecoder sets it only
        // through set_default_cms, whose translated default CMS is static and
        // obeys cms_interface.h, or the unsafe set_cms, whose caller keeps the
        // interface valid until the decoder and its pipeline are dropped.
        // Public FrameDecoder callers carry FrameDecoder::new's contract.
        let cms_stage = unsafe {
            add_output_color_stages(&mut self.state.output_encoding_info, &mut builder, linear)
        }?;
        if cms_stage && options.use_slow_render_pipeline {
            // The original simple pipeline prepares stages for threads before
            // its first SetInputSizes, so CmsStage::PrepareForThreads would
            // initialize the CMS with its indeterminate xsize_. That sequence
            // has no defined source behavior; admission stops before it. The
            // low-memory pipeline's Init sets input sizes during Finalize,
            // before PrepareForThreads, so it is defined and admitted there;
            // decode.cc always selects that pipeline.
            return Err(StatusCode::Unsupported);
        }
        let info = &self.state.output_encoding_info;
        let main = &self.state.main_output;
        if main.callback.is_present() || !main.buffer.is_null() {
            let mut has_alpha = false;
            let mut alpha_c = 0;
            for (i, eci) in m.extra_channel_info.iter().enumerate() {
                if eci.channel_type == super::image_metadata::ExtraChannel::ALPHA {
                    has_alpha = true;
                    alpha_c = 3 + i;
                    break;
                }
            }
            // SAFETY: The fixed entry points never install descriptors after
            // InitFrameOutput's dec_state_->Init cleared them, so they cannot
            // reach this branch. For public decoders, FrameDecoder::new's
            // contract makes every present descriptor's buffers/callbacks and
            // the supplied manager valid until this pipeline is destroyed.
            let stage = unsafe {
                super::render_pipeline::stage_write::get_write_to_output_stage(
                    main,
                    self.state.width,
                    self.state.height,
                    has_alpha,
                    self.state.unpremul_alpha,
                    alpha_c,
                    self.state.undo_orientation,
                    &self.state.extra_output,
                    self.storage.memory_manager,
                )
            };
            builder.add_stage(Some(stage))?;
        } else {
            let writer = match self.storage.output.as_ref().unwrap() {
                FrameOutput::Borrowed(output) => {
                    WriteToImageBundleStage::from_shared(output, &info.color_encoding)
                }
                // The source stage keeps `decoded_` as a raw pointer; its
                // owner releases the bundle at its own checkpoint.
                FrameOutput::Owned(output) => {
                    WriteToImageBundleStage::from_pointer(output, &info.color_encoding)
                }
            };
            builder.add_stage(Some(trace_bundle_writer(
                Box::new(writer),
                &self.storage.write_trace,
                FrameWriteTarget::Output,
            )))?;
        }
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
        self.storage.decoded().borrow_mut().origin = self.storage.header.0.frame_origin;
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
            let memory_manager = self.state.memory_manager();
            let shared = &mut self.state.shared_storage;
            shared
                .matrices
                .decode_in(memory_manager, reader, Some(&self.storage.modular))?;
            shared.matrices.ensure_computed_in(memory_manager, used)?;
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
                    memory_manager,
                    orders,
                    used,
                    &mut shared.coeff_orders[start..start + shared.coeff_order_size],
                    reader,
                )?;
                let contexts =
                    shared.num_histograms * shared.block_ctx_map.num_ac_contexts() as usize;
                super::dec_ans::decode_histograms(
                    memory_manager,
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
            let kind = if max_bits < 16 && self.storage.decoded().borrow().jpeg_data.is_none() {
                super::dct_util::AcType::I16
            } else {
                super::dct_util::AcType::I32
            };
            let store = self.storage.header.0.passes.num_passes > 1;
            self.state.coefficients = Some(super::dct_util::AcImage::make_in(
                memory_manager,
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
        let mut decoded = self.storage.decoded().borrow_mut();
        if decoded.jpeg_data.is_some() {
            decoded.color_transform = self.storage.header.0.color_transform;
            decoded.chroma_subsampling = self.storage.header.0.chroma_subsampling.clone();
            let jpeg = decoded.jpeg_data.as_deref_mut().unwrap();
            let encodings = self.state.shared_storage.matrices.encodings();
            let Some(encoding) = encodings.first() else {
                return Err(StatusCode::GenericError);
            };
            let super::quant_weights::QuantEncodingData::Raw { qtable, qtable_den } =
                encoding.data()
            else {
                return Err(StatusCode::GenericError);
            };
            if (*qtable_den - 1.0f32 / (8 * 255) as f32).abs() > 1e-8f32 {
                return Err(StatusCode::GenericError);
            }
            let num_components = jpeg.components.len();
            let is_gray = num_components == 1;
            if self.storage.header.0.color_transform == ColorTransform::XYB || num_components > 3 {
                return Err(StatusCode::GenericError);
            }
            let map =
                super::frame_header::jpeg_order(self.storage.header.0.color_transform, is_gray);
            let qtable = qtable
                .as_deref()
                .expect("source RAW quantization allocation");
            if qtable.len() != 3 * 8 * 8 {
                return Err(StatusCode::GenericError);
            }
            let mut qt_set = 0usize;
            for (c, &index) in map.iter().enumerate().take(num_components) {
                let quant_c = if is_gray { 1 } else { c };
                let qpos = jpeg.components[index as usize].quant_idx as usize;
                if qpos == jpeg.quant.len() {
                    return Err(StatusCode::GenericError);
                }
                qt_set |= 1usize << qpos;
                for x in 0..8 {
                    for y in 0..8 {
                        jpeg.quant[qpos].values[x * 8 + y] = qtable[quant_c * 64 + y * 8 + x];
                    }
                }
            }
            for i in 0..jpeg.quant.len() {
                if qt_set & (1 << i) != 0 {
                    continue;
                }
                if i == 0 {
                    return Err(StatusCode::GenericError);
                }
                jpeg.quant[i].values = jpeg.quant[i - 1].values;
            }
        }
        Ok(())
    }
    // ProcessACGroup. `sections[indices[pass]]` supplies each new pass reader;
    // Flush passes no sections, zero passes and force_draw.
    #[expect(
        clippy::too_many_arguments,
        reason = "Keep the original group, reader, pass, thread and draw parameters."
    )]
    fn process_ac_group(
        &mut self,
        g: usize,
        sections: &mut [SectionInfo<'_, '_>],
        indices: &[usize],
        num_passes: usize,
        thread: usize,
        force_draw: bool,
        dc_only: bool,
    ) -> Status {
        let d = &self.storage.dim;
        let memory_manager = self.state.memory_manager();
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
            self.storage.group_caches[thread].init_once_in(
                memory_manager,
                self.storage.header.0.passes.num_passes as usize,
                self.state
                    .used_acs
                    .load(std::sync::atomic::Ordering::SeqCst) as usize,
            )?;
            let mut pass_readers: Vec<_> = (pass0..pass0 + num_passes)
                .map(|i| std::mem::take(&mut *sections[indices[i]].br))
                .collect();
            let group_state = super::dec_group::AcGroupDecoderState {
                shared: &self.state.shared_storage,
                code: &self.state.code,
                context_map: &self.state.context_map,
                coefficients: self.state.coefficients.as_mut().unwrap(),
                x_dm_multiplier: self.state.x_dm_multiplier,
                b_dm_multiplier: self.state.b_dm_multiplier,
                quant_biases: &self.state.output_encoding_info.opsin_params.quant_biases,
                upsampler8x: self.state.upsampler8x.as_deref_mut(),
            };
            let mut decoded = self.storage.output.as_deref().unwrap().borrow_mut();
            let result = super::dec_group::decode_ac_group(
                &self.storage.header.0,
                &mut pass_readers,
                num_passes,
                g,
                group_state,
                &mut self.storage.group_caches[thread],
                thread,
                &mut input,
                decoded.jpeg_data.as_deref_mut(),
                pass0,
                force_draw,
                dc_only,
                Some(&mut should_run_pipeline),
            );
            for (i, reader) in (pass0..pass0 + num_passes).zip(pass_readers) {
                *sections[indices[i]].br = reader;
            }
            result?;
        }
        // Modular rectangles are not limited to the image here (DecodeGroup does).
        let mut modular_ready = false;
        let pass1 = if force_draw {
            self.storage.header.0.passes.num_passes as usize
        } else {
            pass0 + num_passes
        };
        for i in pass0..pass1 {
            let (min, max) = self.storage.header.0.passes.get_downsampling_bracket(i);
            let mut modular_pass_ready = true;
            if i < pass0 + num_passes {
                self.storage.modular.decode_group(
                    &self.storage.header.0,
                    rect,
                    Some(&mut *sections[indices[i]].br),
                    min,
                    max,
                    &ModularStreamId::modular_ac(g, i),
                    false,
                    Some(&self.state.shared_storage),
                    Some(&mut input),
                    false,
                    Some(&mut modular_pass_ready),
                )?;
            } else {
                self.storage.modular.decode_group(
                    &self.storage.header.0,
                    rect,
                    None,
                    min,
                    max,
                    &ModularStreamId::modular_ac(g, i),
                    true,
                    Some(&self.state.shared_storage),
                    Some(&mut input),
                    false,
                    Some(&mut modular_pass_ready),
                )?;
            }
            if modular_pass_ready {
                modular_ready = true;
            }
        }
        self.storage.passes[g] = self.storage.passes[g].wrapping_add(num_passes as u8);
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
        if !self.storage.modular.uses_full_image()
            && self.storage.decoded().borrow().jpeg_data.is_none()
        {
            if should_run_pipeline && modular_ready {
                input.done()?;
            } else if force_draw {
                return Err(StatusCode::GenericError);
            }
        }
        Ok(())
    }
    fn mark_sections(&mut self, sections: &[SectionInfo<'_, '_>], statuses: &[SectionStatus]) {
        self.storage.num_sections_done =
            self.storage.num_sections_done.wrapping_add(sections.len());
        for (section, status) in sections.iter().zip(statuses) {
            if *status != SectionStatus::Done {
                self.storage.processed[section.id] = 0;
                self.storage.num_sections_done = self.storage.num_sections_done.wrapping_sub(1);
            }
        }
    }
    #[expect(
        clippy::collapsible_if,
        reason = "Mirror the source nested conditions."
    )]
    fn process_sections(
        &mut self,
        sections: &mut [SectionInfo<'_, '_>],
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
        // The number of AC passes to process for each group during this call.
        let mut desired = vec![0usize; self.storage.dim.num_groups];
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
            // Count the number of new passes per group.
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
            let status = self.process_dc_global(&mut *sections[dc_global].br);
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
            let pool = self.storage.pool();
            let mut state = (&mut *self, &mut *sections, &mut *statuses);
            // SAFETY: The tasks use the frame scope, whose owners share
            // Rc/RefCell handles (ADR 0003), and the section readers. None
            // holds a thread-affine value, and nothing else uses those owners
            // until this call returns.
            unsafe {
                run_on_pool_exclusive_no_init(
                    pool,
                    0,
                    dc.len() as u32,
                    &mut state,
                    |(scope, sections, statuses), i, _thread| -> Status {
                        let i = i as usize;
                        if dc[i] != num {
                            scope.process_dc_group(i, &mut *sections[dc[i]].br)?;
                            statuses[dc[i]] = SectionStatus::Done;
                        }
                        Ok(())
                    },
                )
            }?;
        }
        if !self.storage.dc_groups.contains(&0) && !self.storage.finalized_dc {
            let options = PipelineOptions {
                use_slow_render_pipeline: self.storage.use_slow_rendering_pipeline,
                coalescing: self.storage.coalescing,
                render_spotcolors: self.storage.render_spotcolors,
                render_noise: true,
            };
            self.prepare_pipeline(options)?;
            // FinalizeDC: adaptive DC smoothing must happen between all DC and
            // AC group processing.
            if self.storage.header.0.encoding == FrameEncoding::VAR_DCT
                && self.storage.header.0.flags & FLAG_SKIP_ADAPTIVE_DC_SMOOTHING == 0
                && self.storage.header.0.flags & super::frame_header::FLAG_USE_DC_FRAME == 0
                && self.storage.decoded().borrow().jpeg_data.is_none()
            {
                let memory_manager = self.state.memory_manager();
                let shared = &mut self.state.shared_storage;
                // vardct kernels: HWY_DYNAMIC_DISPATCH(AdaptiveDCSmoothing).
                match super::hwy::decoder_target() {
                    Some(t) => super::compressed_dc_simd::adaptive_dc_smoothing_simd(
                        t,
                        memory_manager,
                        shared.quantizer.mul_dc(),
                        &mut shared.dc_storage,
                        &self.storage.header.0.chroma_subsampling,
                        self.storage.pool(),
                    )?,
                    None => super::compressed_dc::adaptive_dc_smoothing_in(
                        memory_manager,
                        shared.quantizer.mul_dc(),
                        &mut shared.dc_storage,
                        &self.storage.header.0.chroma_subsampling,
                        self.storage.pool(),
                    )?,
                }
            }
            self.storage.finalized_dc = true;
            self.allocate_output()?;
            if self.storage.progressive_detail >= JxlProgressiveDetail::DC {
                self.mark_sections(sections, statuses);
                return Ok(());
            }
        }
        if self.storage.finalized_dc && ac_global != num && !self.storage.decoded_ac_global {
            self.process_ac_global(&mut *sections[ac_global].br)?;
            self.storage.decoded_ac_global = true;
            statuses[ac_global] = SectionStatus::Done;
        }
        if self.storage.progressive_detail >= JxlProgressiveDetail::LAST_PASSES {
            // Only process the next progression step.
            let target = self.storage.next_num_passes_to_pause();
            for (g, count) in desired.iter_mut().enumerate() {
                *count = (*count).min(target.wrapping_sub(self.storage.passes[g] as usize));
            }
        }
        if self.storage.decoded_ac_global {
            // Mark all received AC groups as not complete yet.
            for (g, &count) in desired.iter().enumerate() {
                if count != 0 {
                    self.state.render_pipeline.as_mut().unwrap().clear_done(g);
                }
            }
            let pool = self.storage.pool();
            let mut state = (&mut *self, &mut *sections, &mut *statuses);
            // SAFETY: As for the DC groups above.
            unsafe {
                run_on_pool_exclusive(
                    pool,
                    0,
                    ac.len() as u32,
                    &mut state,
                    |(scope, _, _), num_threads| -> Status {
                        let num_tasks = scope.storage.passes.len();
                        scope.prepare_storage(num_threads, num_tasks)
                    },
                    |(scope, sections, statuses), g, thread| -> Status {
                        let g = g as usize;
                        if desired[g] == 0 {
                            // No new AC pass, nothing to do.
                            return Ok(());
                        }
                        let first = scope.storage.passes[g] as usize;
                        for i in 0..desired[g] {
                            if ac[g][first + i] == num {
                                return Err(StatusCode::GenericError);
                            }
                        }
                        let location = scope.storage.get_storage_location(thread, g);
                        scope.process_ac_group(
                            g, sections, &ac[g], desired[g], location, false, false,
                        )?;
                        for i in 0..desired[g] {
                            statuses[ac[g][first + i]] = SectionStatus::Done;
                        }
                        Ok(())
                    },
                )
            }?;
        }
        self.mark_sections(sections, statuses);
        Ok(())
    }
    fn flush(&mut self) -> Status {
        let h = &self.storage.header.0;
        let mut has_blending =
            h.blending_info.mode != BlendMode::REPLACE || h.custom_size_or_origin;
        for info in &h.extra_channel_blending_info {
            if info.mode != BlendMode::REPLACE {
                has_blending = true;
            }
        }
        // No early Flush() if blending is enabled.
        if has_blending && !self.storage.is_finalized {
            return Err(StatusCode::GenericError);
        }
        // No early Flush() - nothing to do - for a kSkipProgressive frame.
        if h.frame_type == FrameType::SKIP_PROGRESSIVE && !self.storage.is_finalized {
            return Ok(());
        }
        if self.storage.decoded().borrow().jpeg_data.is_some() {
            // Nothing to do.
            return Ok(());
        }
        self.allocate_output()?;
        let num_passes = self.storage.header.0.passes.num_passes as usize;
        let completely_decoded_ac_pass = *self.storage.passes.iter().min().unwrap() as usize;
        if completely_decoded_ac_pass < num_passes {
            // Not all AC is available yet: force a draw of all missing areas.
            for g in 0..self.storage.passes.len() {
                if (self.storage.passes[g] as usize) < num_passes {
                    self.state.render_pipeline.as_mut().unwrap().clear_done(g);
                }
            }
            let pool = self.storage.pool();
            let num_tasks = self.storage.passes.len();
            // SAFETY: As in process_sections: the tasks use the frame scope,
            // whose owners hold no thread-affine value, and nothing else uses
            // them until this call returns.
            unsafe {
                run_on_pool_exclusive(
                    pool,
                    0,
                    num_tasks as u32,
                    self,
                    |scope, num_threads| -> Status {
                        let num_tasks = scope.storage.passes.len();
                        scope.prepare_storage(num_threads, num_tasks)
                    },
                    |scope, g, thread| -> Status {
                        let g = g as usize;
                        if scope.storage.passes[g] as usize == num_passes {
                            // This group was drawn already, nothing to do.
                            return Ok(());
                        }
                        let location = scope.storage.get_storage_location(thread, g);
                        let dc_only = !scope.storage.decoded_ac_global;
                        scope.process_ac_group(g, &mut [], &[], 0, location, true, dc_only)
                    },
                )
            }?;
        }
        // Undo global modular transforms and copy int pixel buffers to floats.
        let pool = self.storage.pool();
        self.storage.modular.finalize_decoding(
            &self.storage.header.0,
            &self.state.shared_storage,
            &mut **self.state.render_pipeline.as_mut().unwrap(),
            pool,
            self.storage.is_finalized,
        )
    }
    fn finalize_frame(&mut self) -> Status {
        if self.storage.is_finalized {
            return Err(StatusCode::GenericError);
        }
        self.storage.is_finalized = true;
        if self.storage.decoded().borrow().jpeg_data.is_some() {
            return Ok(());
        }
        let pool = self.storage.pool();
        self.storage.modular.finalize_decoding(
            &self.storage.header.0,
            &self.state.shared_storage,
            &mut **self.state.render_pipeline.as_mut().unwrap(),
            pool,
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
}

impl<'m, 'p> FrameDecoder<'m, 'p> {
    /// `modular_memory_manager` is the source's `dec_state_->memory_manager()`
    /// given to the modular frame decoder at construction.
    fn with_profile(
        metadata: &'m super::image_metadata::CodecMetadata,
        memory_manager: *const JxlMemoryManager,
        pool: *const ThreadPool,
        modular_memory_manager: MemoryManagerPtr,
        use_slow_rendering_pipeline: bool,
        profile: DecodeProfile,
    ) -> Self {
        Self {
            memory_manager,
            metadata,
            pool,
            output: None,
            header: AdmittedHeader(Rc::new(FrameHeader::new(Some(metadata)))),
            admitted: false,
            dim: FrameDimensions::default(),
            modular: ModularFrameDecoder::new_in(modular_memory_manager),
            render_spotcolors: true,
            coalescing: true,
            toc: Vec::new(),
            section_sizes_sum: 0,
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
            use_slow_rendering_pipeline,
            progressive_detail: JxlProgressiveDetail::FRAMES,
            passes_to_pause: Vec::new(),
            profile,
            write_trace: None,
        }
    }
    /// The FrameDecoder constructor. Metadata must outlive the decoder; the
    /// state passed to each call is the source's `dec_state`. Metadata outside
    /// the admitted sequence profile returns Unsupported.
    /// `use_slow_rendering_pipeline` selects the simple (true) or low-memory
    /// (false) render pipeline, as the original constructor flag does.
    ///
    /// # Safety
    /// For every `PassesDecoderState` passed to this decoder, until each render
    /// pipeline built during those calls is destroyed or replaced:
    /// - every present main/extra output descriptor (see `set_image_output`)
    ///   must satisfy `WriteToOutputStage::new`'s buffer and callback contract;
    /// - `memory_manager`, when such a descriptor is present, must be null or
    ///   satisfy that same contract's manager requirements;
    /// - `memory_manager` is the manager of every such `PassesDecoderState`:
    ///   null, or a manager satisfying `MemoryManagerPtr::from_raw` until this
    ///   decoder and every owner allocated through it are dropped;
    /// - an output encoding with `cms_set` must satisfy `get_cms_stage`'s CMS
    ///   callback contract.
    ///
    /// `pool` is null or a ThreadPool that stays valid, unmoved and unmutated
    /// while this decoder is used. Its RunOnPool tasks may call the output
    /// callbacks from any runner thread, one at a time.
    pub unsafe fn new(
        metadata: &'m super::image_metadata::CodecMetadata,
        memory_manager: *const JxlMemoryManager,
        pool: *const ThreadPool,
        use_slow_rendering_pipeline: bool,
    ) -> Result<Self, StatusCode> {
        admit_sequence_metadata(metadata)?;
        // SAFETY: The caller's contract above is from_raw's for this manager.
        let modular_memory_manager = unsafe { MemoryManagerPtr::from_raw(memory_manager) };
        Ok(Self::with_profile(
            metadata,
            memory_manager,
            pool,
            modular_memory_manager,
            use_slow_rendering_pipeline,
            DecodeProfile::Sequence,
        ))
    }
    fn scope<'s>(
        &'s mut self,
        state: &'s mut PassesDecoderState<'m, 'p>,
    ) -> Result<FrameDecoderScope<'s, 'm, 'p>, StatusCode> {
        if !self.admitted {
            return Err(StatusCode::Unsupported);
        }
        Ok(FrameDecoderScope {
            state,
            storage: self,
        })
    }
    /// Retain allocation and row-write checkpoints for test observation.
    pub fn set_write_trace(&mut self, trace: Option<FrameWriteTrace>) {
        self.write_trace = trace;
    }
    pub fn set_render_spotcolors(&mut self, rsc: bool) {
        self.render_spotcolors = rsc;
    }
    pub fn set_coalescing(&mut self, c: bool) {
        self.coalescing = c;
    }
    /// Read the FrameHeader and table of contents. A successfully parsed header
    /// outside the admitted profile returns Unsupported.
    pub fn init_frame(
        &mut self,
        state: &mut PassesDecoderState<'m, 'p>,
        br: &mut BitReader<'_>,
        decoded: Rc<RefCell<ImageBundle<'m>>>,
        is_preview: bool,
    ) -> Status {
        self.init_frame_source(state, br, FrameOutput::Owned(decoded), is_preview)?;
        self.admit()
    }
    // The original InitFrame body, before profile admission.
    #[expect(
        clippy::manual_is_multiple_of,
        reason = "Mirror the source modulo test."
    )]
    fn init_frame_source(
        &mut self,
        state: &mut PassesDecoderState<'m, 'p>,
        br: &mut BitReader<'_>,
        decoded: FrameOutput<'p, 'm>,
        is_preview: bool,
    ) -> Status {
        self.output = Some(decoded);
        // JXL_ENSURE(is_finalized_) leaves the current frame unchanged.
        if !self.is_finalized {
            return Err(StatusCode::GenericError);
        }
        self.admitted = false;
        // Reset the dequantization matrices to their default values.
        state.shared_storage.matrices = DequantMatrices::default();
        let mut header = FrameHeader::new(Some(self.metadata));
        header.nonserialized_is_preview = is_preview;
        let read = read_frame_header(br, &mut header);
        self.header = AdmittedHeader(Rc::new(header));
        read?;
        let header = &self.header.0;
        self.dim = header.to_frame_dimensions();
        let num_passes = header.passes.num_passes as usize;
        let num_groups = self.dim.num_groups;
        // If the previous frame was not a kRegularFrame, `decoded` may have
        // different dimensions; reset it to avoid errors.
        {
            let mut decoded = self.decoded().borrow_mut();
            decoded.color_mut().assign(Image3::default());
            super::base::libcpp_vector::clear(decoded.extra_channels_mut());
            if let Some(events) = &self.write_trace {
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
        // Read the TOC.
        let toc_entries = num_toc_entries(num_groups, self.dim.num_dc_groups, num_passes);
        let mut sizes = Vec::new();
        let mut permutation = Vec::new();
        let memory_manager = self.decoded().borrow().memory_manager();
        read_toc(
            memory_manager,
            toc_entries,
            br,
            &mut sizes,
            &mut permutation,
        )?;
        let have_permutation = !permutation.is_empty();
        self.toc.resize(toc_entries, TocEntry::default());
        self.section_sizes_sum = 0;
        for i in 0..toc_entries {
            self.toc[i].size = sizes[i] as usize;
            let index = if have_permutation {
                permutation[i] as usize
            } else {
                i
            };
            self.toc[index].id = i;
            let next = self.section_sizes_sum.wrapping_add(self.toc[i].size as u64);
            if next < self.section_sizes_sum {
                return Err(StatusCode::GenericError);
            }
            self.section_sizes_sum = next;
        }
        if br.total_bits_consumed() % 8 != 0 {
            return Err(StatusCode::GenericError);
        }
        let group_codes_begin = br.total_bits_consumed() / 8;
        if self.toc.is_empty() {
            return Err(StatusCode::GenericError);
        }
        // Overflow check.
        if (group_codes_begin as u64).wrapping_add(self.section_sizes_sum)
            < group_codes_begin as u64
        {
            return Err(StatusCode::GenericError);
        }
        Ok(())
    }
    // Profile admission of a successfully parsed header.
    fn admit(&mut self) -> Status {
        let header = self.header.0.as_ref();
        let admitted = match self.profile {
            DecodeProfile::VarDct => AdmittedHeader::admit_vardct(header),
            DecodeProfile::LosslessThunder => AdmittedHeader::admit(header),
            DecodeProfile::Modular => AdmittedHeader::admit_modular(header),
            DecodeProfile::Sequence => AdmittedHeader::admit_sequence(header),
            DecodeProfile::Jpeg => Ok(()),
        };
        self.admitted = admitted.is_ok();
        admitted
    }
    /// InitFrameOutput: initialize shared/decoder state and the frame's own
    /// section, pass and group bookkeeping.
    pub fn init_frame_output(&mut self, state: &mut PassesDecoderState<'m, 'p>) -> Status {
        self.scope(state)?.init_output()
    }
    /// ProcessSections over `sections`, filling one status per section. Each
    /// reader must contain exactly its section's bytes.
    pub fn process_sections(
        &mut self,
        state: &mut PassesDecoderState<'m, 'p>,
        sections: &mut [SectionInfo<'_, '_>],
        section_status: &mut [SectionStatus],
    ) -> Status {
        assert_eq!(sections.len(), section_status.len());
        if sections.is_empty() {
            // Nothing to process.
            return Ok(());
        }
        self.scope(state)?
            .process_sections(sections, section_status)
    }
    /// Flush all data decoded so far to pixels. The source requires a decoded
    /// DC (HasDecodedDC); decode.cc checks it before calling.
    pub fn flush(&mut self, state: &mut PassesDecoderState<'m, 'p>) -> Status {
        self.scope(state)?.flush()
    }
    /// FinalizeFrame; must be called exactly once per frame after all
    /// ProcessSections calls.
    pub fn finalize_frame(&mut self, state: &mut PassesDecoderState<'m, 'p>) -> Status {
        self.scope(state)?.finalize_frame()
    }
    fn has_dc_group_to_decode(&self) -> bool {
        self.dc_groups.contains(&0)
    }
    fn has_everything(&self) -> bool {
        if !self.decoded_dc_global {
            return false;
        }
        if !self.decoded_ac_global {
            return false;
        }
        if self.has_dc_group_to_decode() {
            return false;
        }
        for &nb_passes in &self.passes {
            if (nb_passes as u32) < self.header.0.passes.num_passes {
                return false;
            }
        }
        true
    }
    /// Reference ids this frame depends on as a bit mask: bits 0-3 for patch
    /// and blending references, bits 4-7 for DC frames. Valid after all
    /// ProcessSections calls and before FinalizeFrame.
    pub fn references(&self, state: &PassesDecoderState<'m, 'p>) -> i32 {
        if self.is_finalized {
            return 0;
        }
        if !self.has_everything() {
            return 0;
        }
        let h = &self.header.0;
        let mut result = 0;
        // Blending.
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
        // Patches.
        if h.flags & super::frame_header::FLAG_PATCHES != 0 {
            result |= state
                .shared()
                .image_features
                .patches
                .borrow()
                .get_references();
        }
        // DC level: reads from the next DC level.
        if h.flags & super::frame_header::FLAG_USE_DC_FRAME != 0 {
            let dc_level = h.dc_level + 1;
            result |= 16 << (dc_level - 1);
        }
        result
    }
    /// SavedAs: the storage location bit of this frame, or 0.
    pub fn saved_as(header: &FrameHeader<'_>) -> i32 {
        frame_saved_as(header)
    }
    pub fn sum_section_sizes(&self) -> u64 {
        self.section_sizes_sum
    }
    pub fn toc(&self) -> &[TocEntry] {
        &self.toc
    }
    pub fn get_frame_header(&self) -> &FrameHeader<'m> {
        &self.header.0
    }
    /// Whether the DC image has been decoded (FinalizeDC has run).
    pub fn has_decoded_dc(&self) -> bool {
        self.finalized_dc
    }
    pub fn has_decoded_all(&self) -> bool {
        self.toc.len() == self.num_sections_done
    }
    pub fn num_complete_passes(&self) -> usize {
        *self
            .passes
            .iter()
            .min()
            .expect("source min_element requires an AC group") as usize
    }
    /// SetPauseAtProgressive; returns the progressive detail effective for the
    /// frame. Pause points accumulate as in the source.
    #[allow(clippy::needless_range_loop, reason = "Keep the source index loops.")]
    pub fn set_pause_at_progressive(
        &mut self,
        prog_detail: JxlProgressiveDetail,
    ) -> JxlProgressiveDetail {
        let h = &self.header.0;
        let single_section = self.dim.num_groups == 1 && h.passes.num_passes == 1;
        if h.frame_type != FrameType::SKIP_PROGRESSIVE
            // A single group and pass has no separate DC section.
            && !single_section
            // Extra channels and Modular DC do not support a DC pause.
            && self
                .decoded()
                .borrow()
                .metadata()
                .expect("source decoded bundle has metadata")
                .extra_channel_info
                .is_empty()
            && h.encoding == FrameEncoding::VAR_DCT
        {
            self.progressive_detail = prog_detail;
        } else {
            self.progressive_detail = JxlProgressiveDetail::FRAMES;
        }
        if self.progressive_detail >= JxlProgressiveDetail::PASSES {
            for i in 1..h.passes.num_passes {
                self.passes_to_pause.push(i as i32);
            }
        } else if self.progressive_detail >= JxlProgressiveDetail::LAST_PASSES {
            for i in 0..h.passes.num_downsample as usize {
                self.passes_to_pause.push(h.passes.last_pass[i] as i32 + 1);
            }
            // The format does not guarantee that these values are sorted.
            self.passes_to_pause.sort_unstable();
        }
        self.progressive_detail
    }
    pub fn next_num_passes_to_pause(&self) -> usize {
        let complete = self.num_complete_passes() as i32;
        let index = self
            .passes_to_pause
            .partition_point(|&value| value <= complete);
        match self.passes_to_pause.get(index) {
            Some(&value) => value as usize,
            None => usize::MAX,
        }
    }
    /// SetImageOutput: install the pixel callback or image buffer for the
    /// state's next PreparePipeline. `undo_orientation` applies the metadata
    /// orientation. The descriptors are subject to `FrameDecoder::new`'s
    /// contract.
    #[expect(
        clippy::too_many_arguments,
        reason = "Original SetImageOutput boundary."
    )]
    pub fn set_image_output(
        &self,
        state: &mut PassesDecoderState<'m, 'p>,
        pixel_callback: PixelCallback,
        image_buffer: *mut c_void,
        image_buffer_size: usize,
        xsize: usize,
        ysize: usize,
        format: JxlPixelFormat,
        bits_per_sample: usize,
        unpremul_alpha: bool,
        undo_orientation: bool,
    ) -> Status {
        state.width = xsize;
        state.height = ysize;
        state.main_output.format = format;
        state.main_output.bits_per_sample = bits_per_sample;
        state.main_output.callback = pixel_callback;
        state.main_output.buffer = image_buffer;
        state.main_output.buffer_size = image_buffer_size;
        state.main_output.stride = get_stride(xsize, format)?;
        let decoded = self.decoded().borrow();
        let metadata = decoded
            .metadata()
            .expect("source decoded bundle has metadata");
        let alpha = metadata.find(super::image_metadata::ExtraChannel::ALPHA);
        if alpha.is_some_and(|alpha| alpha.alpha_associated) && unpremul_alpha {
            state.unpremul_alpha = true;
        }
        if undo_orientation {
            state.undo_orientation = metadata.get_orientation();
            if state.undo_orientation.0 > 4 {
                std::mem::swap(&mut state.width, &mut state.height);
            }
        }
        state.extra_output.clear();
        Ok(())
    }
    /// AddExtraChannelOutput, subject to `FrameDecoder::new`'s contract.
    pub fn add_extra_channel_output(
        &mut self,
        state: &mut PassesDecoderState<'m, 'p>,
        buffer: *mut c_void,
        buffer_size: usize,
        xsize: usize,
        format: JxlPixelFormat,
        bits_per_sample: usize,
    ) -> Status {
        let out = ImageOutput {
            format,
            bits_per_sample,
            buffer,
            buffer_size,
            stride: get_stride(xsize, format)?,
            ..ImageOutput::default()
        };
        state.extra_output.push(out);
        Ok(())
    }
}

// FrameDecoder::BytesPerChannel.
fn bytes_per_channel(data_type: JxlDataType) -> usize {
    if data_type == JxlDataType::UINT8 {
        1
    } else if data_type == JxlDataType::FLOAT {
        4
    } else {
        2
    }
}
// FrameDecoder::GetStride with SafeMul and SafeRoundUpTo.
fn get_stride(xsize: usize, format: JxlPixelFormat) -> Result<usize, StatusCode> {
    let safe_mul = |a: usize, b: usize| -> Option<usize> {
        if a == 0 || b == 0 {
            return Some(0);
        }
        a.checked_mul(b)
    };
    let x_stride = safe_mul(
        bytes_per_channel(format.data_type),
        format.num_channels as usize,
    )
    .ok_or(StatusCode::GenericError)?;
    let y_stride = safe_mul(x_stride, xsize).ok_or(StatusCode::GenericError)?;
    if format.align < 2 {
        return Ok(y_stride);
    }
    let reminder = y_stride % format.align;
    if reminder == 0 {
        return Ok(y_stride);
    }
    y_stride
        .checked_add(format.align - reminder)
        .ok_or(StatusCode::GenericError)
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
/// This boundary admits NONE/XYB frames and every parsed YCbCr channel mode,
/// references, DC frames, animation and previews, including source early/late
/// extra-channel and main spatial upsampling, associated/subsampled alpha,
/// and all source-defined extra-channel types. Orientation/intrinsic metadata
/// stays attached to the bundle; external conversion applies orientation.
/// Every metadata color encoding, ICC profile and intensity target is
/// admitted. The output-encoding setters below perform the OutputEncodingInfo
/// operations that decode.cc's color-profile, CMS and intensity-target API
/// applies; that API's own argument and state checks belong to its caller.
/// With the simple pipeline, a frame whose source pipeline needs a CMS stage
/// returns Unsupported when its pipeline is prepared: that pipeline would
/// initialize the stage before its input sizes are defined. The low-memory
/// pipeline (`with_rendering_pipeline(metadata, false)`) admits such frames.
pub struct SequenceDecoder<'m> {
    state: PassesDecoderState<'m, 'm>,
    output: Rc<RefCell<ImageBundle<'m>>>,
    metadata: &'m super::image_metadata::CodecMetadata,
    use_slow_rendering_pipeline: bool,
}
// Metadata admission of the sequence profile. Every color encoding, ICC
// profile and intensity target is admitted; with the simple pipeline, a frame
// that needs a CMS stage returns Unsupported when its pipeline is prepared.
fn admit_sequence_metadata(metadata: &super::image_metadata::CodecMetadata) -> Status {
    let m = &metadata.m;
    if !(1..=32).contains(&m.bit_depth.bits_per_sample)
        || m.num_extra_channels as usize != m.extra_channel_info.len()
    {
        return Err(StatusCode::Unsupported);
    }
    Ok(())
}

impl<'m> SequenceDecoder<'m> {
    /// Decode with the original simple pipeline (use_slow_rendering_pipeline).
    pub fn new(metadata: &'m super::image_metadata::CodecMetadata) -> Result<Self, StatusCode> {
        Self::with_rendering_pipeline(metadata, true)
    }
    /// Construct every frame's FrameDecoder with the given original
    /// use_slow_rendering_pipeline flag; false selects LowMemoryRenderPipeline.
    pub fn with_rendering_pipeline(
        metadata: &'m super::image_metadata::CodecMetadata,
        use_slow_rendering_pipeline: bool,
    ) -> Result<Self, StatusCode> {
        admit_sequence_metadata(metadata)?;
        let mut state = PassesDecoderState::new();
        state.output_encoding_info.set_from_metadata(metadata)?;
        Ok(Self {
            state,
            output: Rc::new(RefCell::new(ImageBundle::new(Some(&metadata.m)))),
            metadata,
            use_slow_rendering_pipeline,
        })
    }
    /// decode.cc's `output_encoding_info.desired_intensity_target` assignment.
    pub fn set_desired_intensity_target(&mut self, desired_intensity_target: f32) {
        self.state.output_encoding_info.desired_intensity_target = desired_intensity_target;
    }
    /// decode.cc's `output_encoding.orig_color_encoding` replacement after it
    /// parses a raw original ICC profile with the selected CMS.
    pub fn set_orig_color_encoding(&mut self, orig_color_encoding: ColorEncoding) {
        self.state.output_encoding_info.orig_color_encoding = orig_color_encoding;
    }
    /// OutputEncodingInfo::MaybeSetColorEncoding on the decoder's state.
    pub fn maybe_set_color_encoding(&mut self, c_desired: &ColorEncoding) -> Status {
        self.state
            .output_encoding_info
            .maybe_set_color_encoding(c_desired)
    }
    /// JxlDecoderSetCms's state effect with the translated default CMS.
    pub fn set_default_cms(&mut self) {
        // SAFETY: The translated default CMS is a static interface whose
        // callbacks implement cms_interface.h for every profile/buffer request.
        unsafe { self.set_cms(*super::cms::jxl_cms::get_default_cms()) };
    }
    /// JxlDecoderSetCms's state effect: store the interface and set cms_set.
    ///
    /// # Safety
    /// Every CMS callback and opaque pointer must satisfy `get_cms_stage`'s
    /// contract and stay valid until this decoder, and every pipeline it
    /// built, is dropped. Later frames reuse the stored interface.
    pub unsafe fn set_cms(&mut self, cms: JxlCmsInterface) {
        self.state.output_encoding_info.color_management_system = cms;
        self.state.output_encoding_info.cms_set = true;
    }
    pub fn decode_frame(&mut self, bytes: &[u8], preview: bool) -> SequenceFrameDecode<'m> {
        let mut header = None;
        let mut trace = FrameDecodeTrace::default();
        let status = decode_frame_with_pipeline(
            self.metadata,
            bytes,
            &mut self.state,
            FrameOutput::Owned(self.output.clone()),
            &mut header,
            DecodeProfile::Sequence,
            preview,
            Some(&mut trace),
            self.use_slow_rendering_pipeline,
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
    Sequence,
    Jpeg,
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
        false,
        None,
    )
}

/// DecodeFrame from dec_frame.cc over the FrameDecoder methods, with the null
/// pool. The fixed profiles construct their FrameDecoder with the simple
/// pipeline. The optional trace records reached checkpoints.
#[expect(
    clippy::too_many_arguments,
    reason = "DecodeFrame arguments plus the private profile, preview and trace."
)]
fn decode_frame_into<'m, 'p>(
    metadata: &'m super::image_metadata::CodecMetadata,
    bytes: &[u8],
    state: &mut PassesDecoderState<'m, 'p>,
    output: FrameOutput<'p, 'm>,
    copied_header: &mut Option<FrameHeader<'m>>,
    profile: DecodeProfile,
    is_preview: bool,
    trace: Option<&mut FrameDecodeTrace>,
) -> Status {
    decode_frame_with_pipeline(
        metadata,
        bytes,
        state,
        output,
        copied_header,
        profile,
        is_preview,
        trace,
        true,
    )
}

#[expect(
    clippy::too_many_arguments,
    reason = "Add the FrameDecoder constructor's pipeline flag to the original caller."
)]
fn decode_frame_with_pipeline<'m, 'p>(
    metadata: &'m super::image_metadata::CodecMetadata,
    bytes: &[u8],
    state: &mut PassesDecoderState<'m, 'p>,
    output: FrameOutput<'p, 'm>,
    copied_header: &mut Option<FrameHeader<'m>>,
    profile: DecodeProfile,
    is_preview: bool,
    mut trace: Option<&mut FrameDecodeTrace>,
    use_slow_rendering_pipeline: bool,
) -> Status {
    let write_trace = trace.as_ref().map(|_| Rc::new(RefCell::new(Vec::new())));
    let status = (|| {
        // The internal state never receives output descriptors or a CMS, so
        // no memory manager is reached and no foreign callback is installed.
        let mut decoder = FrameDecoder::with_profile(
            metadata,
            std::ptr::null(),
            std::ptr::null(),
            state.memory_manager(),
            use_slow_rendering_pipeline,
            profile,
        );
        decoder.set_write_trace(write_trace.clone());
        let mut reader = BitReader::new(bytes);
        // InitFrame: header and TOC parsing precede profile admission.
        let initialized = decoder.init_frame_source(state, &mut reader, output, is_preview);
        if let Some(trace) = &mut trace {
            trace.init_status = Some(initialized);
            trace.header_bits = reader.total_bits_consumed();
        }
        if let Err(e) = initialized.and_then(|()| decoder.admit()) {
            let close = reader.close();
            if let Some(trace) = &mut trace {
                trace.header_close = Some(close);
            }
            return Err(e);
        }
        let initialized = decoder.init_frame_output(state);
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
        *copied_header = Some(decoder.get_frame_header().clone());
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
        let mut readers = Vec::new();
        let mut pos = header_bytes;
        let toc = decoder.toc().to_vec();
        let work = (|| {
            for entry in &toc {
                let end = pos.wrapping_add(entry.size);
                if end > bytes.len() {
                    return Err(StatusCode::GenericError);
                }
                readers.push(BitReader::new(&bytes[pos..end]));
                pos = end;
            }
            let mut sections: Vec<_> = readers
                .iter_mut()
                .zip(&toc)
                .enumerate()
                .map(|(index, (br, entry))| SectionInfo {
                    br,
                    id: entry.id,
                    index,
                })
                .collect();
            let mut statuses = vec![SectionStatus::Skipped; sections.len()];
            let status = decoder.process_sections(state, &mut sections, &mut statuses);
            if let Some(trace) = &mut trace {
                trace.sections_status = Some(status);
                trace.section_statuses = statuses.iter().map(|status| *status as u8).collect();
            }
            status?;
            for (i, status) in statuses.iter().enumerate() {
                if *status != SectionStatus::Done {
                    return Err(StatusCode::GenericError);
                }
                processed_bytes = processed_bytes.wrapping_add(toc[i].size);
            }
            Ok(())
        })();
        let mut close_ok = Ok(());
        if let Some(trace) = &mut trace {
            trace.section_readers.resize(readers.len(), (0, Ok(())));
        }
        // Scoped closers run in reverse construction order at the physical
        // section index checkpoints.
        for (index, reader) in readers.iter_mut().enumerate().rev() {
            let bits = reader.total_bits_consumed();
            let closed = reader.close();
            if let Some(trace) = &mut trace {
                trace.section_readers[index] = (bits, closed);
            }
            if let Err(e) = closed {
                close_ok = Err(e);
            }
        }
        work?;
        close_ok?;
        if let Some(trace) = &mut trace {
            trace.references = Some(decoder.references(state));
            trace.has_decoded_dc = decoder.has_decoded_dc();
            trace.has_decoded_all = decoder.has_decoded_all();
        }
        let finalized = decoder.finalize_frame(state);
        if let Some(trace) = &mut trace {
            trace.finalize_status = Some(finalized);
        }
        finalized?;
        decoder
            .decoded()
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

/// Defined frame result retaining its original JPEGData owner and partial state.
pub struct JpegFrameDecode<'m> {
    pub status: Status,
    pub image: ImageBundle<'m>,
    pub header: Option<FrameHeader<'m>>,
    pub trace: FrameDecodeTrace,
}

/// DecodeFrame with an existing JPEGData owner, preserving coefficient and
/// metadata mutations even when a later section fails. Metadata remains borrowed.
/// The input starts at the original frame header, after codestream metadata.
pub fn decode_jpeg_frame<'m>(
    metadata: &'m super::image_metadata::CodecMetadata,
    bytes: &[u8],
    jpeg_data: Box<super::jpeg::jpeg_data::JpegData>,
) -> JpegFrameDecode<'m> {
    let output = RefCell::new(ImageBundle::new(Some(&metadata.m)));
    output.borrow_mut().jpeg_data = Some(jpeg_data);
    let mut state = PassesDecoderState::new();
    let mut header = None;
    let mut trace = FrameDecodeTrace::default();
    let status = decode_frame_into(
        metadata,
        bytes,
        &mut state,
        FrameOutput::Borrowed(&output),
        &mut header,
        DecodeProfile::Jpeg,
        false,
        Some(&mut trace),
    );
    drop(state);
    JpegFrameDecode {
        status,
        image: output.into_inner(),
        header,
        trace,
    }
}
