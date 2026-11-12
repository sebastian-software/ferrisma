// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! PixelCallback/ImageOutput and PassesDecoderState storage/constructor/Init
//! from dec_cache.h, plus complete null-pool InitForAC from dec_cache.cc.
//! Pipeline storage holds either original implementation selected by
//! RenderPipeline::Builder; custom runners remain unported. The color,
//! tone-mapping and CMS stage selection of PreparePipeline is translated below
//! in three source-ordered segments; the frame decoder adds the interleaved
//! blending and reference writers between them, retaining their original order.
//! Source-indeterminate scalar storage is initialized safely, without source
//! meaning until assigned. Callback/output pointers remain non-owning carriers.
use super::ac_strategy::NUM_VALID_STRATEGIES;
use super::base::status::{Status, StatusCode};
use super::coeff_order::{COEFF_ORDER_OFFSET, STRATEGY_ORDER};
use super::color_encoding::ColorSpace;
use super::common::MAX_NUM_PASSES;
use super::dct_util::{AcImage, AcType};
use super::dec_ans::ANSCode;
use super::dec_xyb::OutputEncodingInfo;
use super::frame_dimensions::DCT_BLOCK_SIZE;
use super::frame_header::ColorTransform;
use super::frame_header::FrameHeader;
use super::image::Image3;
use super::image::Plane;
use super::image_bundle::ImageBundle;
use super::image_metadata::Orientation;
use super::memory_manager_internal::{AlignedMemory, MemoryManagerPtr};
use super::passes_state::PassesSharedState;
use super::render_pipeline::render_pipeline::RenderPipeline;
use super::render_pipeline::render_pipeline::RenderPipelineBuilder;
use super::render_pipeline::render_pipeline_stage::RenderPipelineStage;
use super::render_pipeline::stage_cms::get_cms_stage;
use super::render_pipeline::stage_from_linear::get_from_linear_stage;
use super::render_pipeline::stage_to_linear::get_to_linear_stage;
use super::render_pipeline::stage_tone_mapping::get_tone_mapping_stage;
use super::render_pipeline::stage_upsampling::get_upsampling_stage_in;
use super::render_pipeline::stage_xyb::get_xyb_stage;
use super::render_pipeline::stage_ycbcr::get_ycbcr_stage;
use super::types::{JxlDataType, JxlEndianness, JxlPixelFormat};
use std::cell::RefCell;
use std::ffi::c_void;
use std::rc::Rc;
use std::sync::atomic::{AtomicU32, Ordering};

pub const SIGMA_BORDER: usize = 1;
pub const SIGMA_PADDING: usize = 2;
/// GroupDecCache constructor, InitOnce, and InitDCBufferOnce. Scratch pointer
/// bindings are owner-relative sample offsets, rebound after each initializer.
#[cfg_attr(any(target_arch = "x86", target_arch = "x86_64"), repr(align(64)))]
#[cfg_attr(not(any(target_arch = "x86", target_arch = "x86_64")), repr(align(16)))]
pub struct GroupDecCache {
    pub dec_group_block: Option<usize>,
    pub dec_group_qblock: Option<usize>,
    pub dec_group_qblock16: Option<usize>,
    pub scratch_space: Option<usize>,
    pub num_nzeroes: [Image3<i32>; MAX_NUM_PASSES],
    pub dc_buffer: Plane<f32>,
    float_memory: Option<AlignedMemory>,
    int32_memory: Option<AlignedMemory>,
    int16_memory: Option<AlignedMemory>,
    max_block_area: usize,
}
impl Default for GroupDecCache {
    fn default() -> Self {
        Self {
            dec_group_block: None,
            dec_group_qblock: None,
            dec_group_qblock16: None,
            scratch_space: None,
            num_nzeroes: std::array::from_fn(|_| Image3::default()),
            dc_buffer: Plane::default(),
            float_memory: None,
            int32_memory: None,
            int16_memory: None,
            max_block_area: 0,
        }
    }
}
impl Drop for GroupDecCache {
    /// The implicit destructor's reverse member order: the three aligned
    /// buffers, dc_buffer, then the nonzero-count images last first.
    fn drop(&mut self) {
        self.int16_memory = None;
        self.int32_memory = None;
        self.float_memory = None;
        drop(std::mem::take(&mut self.dc_buffer));
        for image in self.num_nzeroes.iter_mut().rev() {
            drop(std::mem::take(image));
        }
    }
}
/// Disjoint typed views of the source cache's three aligned allocation owners.
/// Nzero images are borrowed separately while entropy decoding uses these buffers.
pub struct GroupDecBuffers<'a> {
    pub block: &'a mut [f32],
    pub scratch: &'a mut [f32],
    pub qblock: &'a mut [i32],
    pub qblock16: &'a mut [i16],
}
impl GroupDecCache {
    pub fn init_once(&mut self, num_passes: usize, used_acs: usize) -> Status {
        self.init_once_in(MemoryManagerPtr::NATIVE, num_passes, used_acs)
    }
    /// The source InitOnce(memory_manager, num_passes, used_acs).
    #[allow(clippy::needless_range_loop, reason = "Keep the source index loops.")]
    pub fn init_once_in(
        &mut self,
        memory_manager: MemoryManagerPtr,
        num_passes: usize,
        used_acs: usize,
    ) -> Status {
        for i in 0..num_passes {
            if self.num_nzeroes[i].xsize() == 0 {
                let dim = super::frame_dimensions::GROUP_DIM_IN_BLOCKS;
                self.num_nzeroes[i].assign(Image3::create_in(dim, dim, memory_manager)?);
            }
        }
        let mut max_block_area = 0;
        for o in 0..NUM_VALID_STRATEGIES {
            let acs = super::ac_strategy::AcStrategy::from_raw_strategy(
                super::ac_strategy::AcStrategyType(u32::from(o)),
            );
            if (used_acs & (1usize << o)) == 0 {
                continue;
            }
            let area = acs.covered_blocks_x() * acs.covered_blocks_y() * DCT_BLOCK_SIZE;
            max_block_area = max_block_area.max(area);
        }
        if max_block_area > self.max_block_area {
            // Preserve assignment before the three allocation/failure points.
            self.max_block_area = max_block_area;
            self.float_memory = Some(AlignedMemory::create(
                memory_manager,
                self.max_block_area * 7 * 4,
                0,
            )?);
            self.int32_memory = Some(AlignedMemory::create(
                memory_manager,
                self.max_block_area * 3 * 4,
                0,
            )?);
            self.int16_memory = Some(AlignedMemory::create(
                memory_manager,
                self.max_block_area * 3 * 2,
                0,
            )?);
        }
        self.dec_group_block = self.float_memory.as_ref().map(|_| 0);
        self.scratch_space = self.float_memory.as_ref().map(|_| self.max_block_area * 3);
        self.dec_group_qblock = self.int32_memory.as_ref().map(|_| 0);
        self.dec_group_qblock16 = self.int16_memory.as_ref().map(|_| 0);
        Ok(())
    }
    pub fn split_buffers(&mut self) -> (GroupDecBuffers<'_>, &mut [Image3<i32>; MAX_NUM_PASSES]) {
        fn floats(memory: &mut Option<AlignedMemory>) -> &mut [f32] {
            let Some(memory) = memory else { return &mut [] };
            let bytes = memory.bytes_mut();
            // SAFETY: AlignedMemory supplies 128-byte aligned initialized bytes;
            // InitOnce allocates whole f32 slots, every bit pattern is valid,
            // and this exclusive owner borrow prevents aliases during the view.
            unsafe { std::slice::from_raw_parts_mut(bytes.as_mut_ptr().cast(), bytes.len() / 4) }
        }
        fn ints(memory: &mut Option<AlignedMemory>) -> &mut [i32] {
            let Some(memory) = memory else { return &mut [] };
            let bytes = memory.bytes_mut();
            // SAFETY: Same exclusive initialized aligned-owner proof as floats;
            // InitOnce allocates whole i32 slots and all bit patterns are valid.
            unsafe { std::slice::from_raw_parts_mut(bytes.as_mut_ptr().cast(), bytes.len() / 4) }
        }
        fn shorts(memory: &mut Option<AlignedMemory>) -> &mut [i16] {
            let Some(memory) = memory else { return &mut [] };
            let bytes = memory.bytes_mut();
            // SAFETY: Same owner/alignment/initialization proof; InitOnce allocates
            // whole i16 slots and no other view of this owner can coexist.
            unsafe { std::slice::from_raw_parts_mut(bytes.as_mut_ptr().cast(), bytes.len() / 2) }
        }
        let float = floats(&mut self.float_memory);
        // After a failed InitOnce allocation the source keeps the new
        // max_block_area_ with a null or smaller buffer, and its pointers are
        // only invalid if used. Absent storage gives empty views here, so a
        // Rust bounds check stops exactly at such a use.
        let mid = (self.max_block_area * 3).min(float.len());
        let (block, scratch) = float.split_at_mut(mid);
        (
            GroupDecBuffers {
                block,
                scratch,
                qblock: ints(&mut self.int32_memory),
                qblock16: shorts(&mut self.int16_memory),
            },
            &mut self.num_nzeroes,
        )
    }
    pub fn init_dc_buffer_once(&mut self) -> Status {
        self.init_dc_buffer_once_in(MemoryManagerPtr::NATIVE)
    }
    /// The source InitDCBufferOnce(memory_manager).
    pub fn init_dc_buffer_once_in(&mut self, memory_manager: MemoryManagerPtr) -> Status {
        if self.dc_buffer.xsize() == 0 {
            let dim = super::frame_dimensions::GROUP_DIM_IN_BLOCKS;
            let offset = super::render_pipeline::render_pipeline_stage::RENDER_PIPELINE_X_OFFSET;
            self.dc_buffer = Plane::create_in(dim + offset * 2, dim + 4, 0, memory_manager)?;
        }
        Ok(())
    }
    pub fn storage_sizes(&self) -> [usize; 4] {
        [
            self.float_memory.as_ref().map_or(0, |m| m.bytes().len()),
            self.int32_memory.as_ref().map_or(0, |m| m.bytes().len()),
            self.int16_memory.as_ref().map_or(0, |m| m.bytes().len()),
            self.max_block_area,
        ]
    }
}
pub type ImageOutInitCallback = unsafe extern "C" fn(*mut c_void, usize, usize) -> *mut c_void;
pub type ImageOutRunCallback =
    unsafe extern "C" fn(*mut c_void, usize, usize, usize, usize, *const c_void);
pub type ImageOutDestroyCallback = unsafe extern "C" fn(*mut c_void);
#[derive(Clone, Copy)]
pub struct PixelCallback {
    pub init: Option<ImageOutInitCallback>,
    pub run: Option<ImageOutRunCallback>,
    pub destroy: Option<ImageOutDestroyCallback>,
    pub init_opaque: *mut c_void,
}
impl Default for PixelCallback {
    fn default() -> Self {
        Self {
            init: None,
            run: None,
            destroy: None,
            init_opaque: std::ptr::null_mut(),
        }
    }
}
impl PixelCallback {
    pub fn new(
        init: Option<ImageOutInitCallback>,
        run: Option<ImageOutRunCallback>,
        destroy: Option<ImageOutDestroyCallback>,
        init_opaque: *mut c_void,
    ) -> Self {
        // Source callback-consistency assertions are debug instrumentation;
        // native builds retain the pinned release reference's carrier behavior.
        Self {
            init,
            run,
            destroy,
            init_opaque,
        }
    }
    pub fn is_present(&self) -> bool {
        self.run.is_some()
    }
    /// Invoke the original initialization callback.
    ///
    /// # Safety
    /// The callback must be present, its function/opaque data must remain valid,
    /// and its documented lifetime/thread/pixel-count preconditions must hold.
    pub unsafe fn init(&self, num_threads: usize, num_pixels: usize) -> *mut c_void {
        // SAFETY: The caller guarantees validity and the callback's contract;
        // expect enforces the source's non-null function-pointer precondition.
        unsafe {
            self.init
                .expect("source requires an initialization callback")(
                self.init_opaque,
                num_threads,
                num_pixels,
            )
        }
    }
}
#[derive(Clone, Copy)]
pub struct ImageOutput {
    pub format: JxlPixelFormat,
    pub bits_per_sample: usize,
    pub callback: PixelCallback,
    pub buffer: *mut c_void,
    pub buffer_size: usize,
    pub stride: usize,
}
impl Default for ImageOutput {
    fn default() -> Self {
        Self {
            format: JxlPixelFormat {
                num_channels: 0,
                data_type: JxlDataType::FLOAT,
                endianness: JxlEndianness::NATIVE,
                align: 0,
            },
            bits_per_sample: 0,
            callback: PixelCallback::default(),
            buffer: std::ptr::null_mut(),
            buffer_size: 0,
            stride: 0,
        }
    }
}
pub enum DecoderShared<'a> {
    Own,
    External(&'a PassesSharedState<'a>),
}
#[derive(Clone, Copy)]
pub struct PipelineOptions {
    pub use_slow_render_pipeline: bool,
    pub coalescing: bool,
    pub render_spotcolors: bool,
    pub render_noise: bool,
}
pub struct PassesDecoderState<'a, 'pipeline> {
    pub shared_storage: PassesSharedState<'a>,
    pub shared: DecoderShared<'a>,
    pub upsampler8x: Option<Box<dyn RenderPipelineStage>>,
    pub code: Vec<ANSCode>,
    pub context_map: Vec<Vec<u8>>,
    pub x_dm_multiplier: f32,
    pub b_dm_multiplier: f32,
    pub sigma: Rc<RefCell<Plane<f32>>>,
    pub width: usize,
    pub height: usize,
    pub main_output: ImageOutput,
    pub extra_output: Vec<ImageOutput>,
    pub fast_xyb_srgb8_conversion: bool,
    pub unpremul_alpha: bool,
    pub undo_orientation: Orientation,
    pub visible_frame_index: usize,
    pub nonvisible_frame_index: usize,
    pub used_acs: AtomicU32,
    pub coefficients: Option<Box<AcImage>>,
    pub render_pipeline: Option<Box<dyn RenderPipeline<'pipeline> + 'pipeline>>,
    pub frame_storage_for_referencing: Rc<RefCell<ImageBundle<'a>>>,
    pub output_encoding_info: OutputEncodingInfo,
}
impl Default for PassesDecoderState<'_, '_> {
    fn default() -> Self {
        Self::new()
    }
}
impl<'a, 'pipeline> PassesDecoderState<'a, 'pipeline> {
    pub fn new() -> Self {
        Self::new_in(MemoryManagerPtr::NATIVE)
    }
    /// The source PassesDecoderState(JxlMemoryManager*) constructor.
    pub fn new_in(memory_manager: MemoryManagerPtr) -> Self {
        Self {
            shared_storage: PassesSharedState::new_in(memory_manager),
            shared: DecoderShared::Own,
            upsampler8x: None,
            code: Vec::new(),
            context_map: Vec::new(),
            x_dm_multiplier: 0.0,
            b_dm_multiplier: 0.0,
            sigma: Rc::new(RefCell::new(Plane::default())),
            width: 0,
            height: 0,
            main_output: ImageOutput::default(),
            extra_output: Vec::new(),
            fast_xyb_srgb8_conversion: false,
            unpremul_alpha: false,
            undo_orientation: Orientation(0),
            visible_frame_index: 0,
            nonvisible_frame_index: 0,
            used_acs: AtomicU32::new(0),
            coefficients: Some(Box::new(AcImage::empty(AcType::I32))),
            render_pipeline: None,
            frame_storage_for_referencing: Rc::new(RefCell::new(ImageBundle::new_in(
                memory_manager,
                None,
            ))),
            output_encoding_info: OutputEncodingInfo::default(),
        }
    }
    pub fn shared(&self) -> &PassesSharedState<'a> {
        match &self.shared {
            DecoderShared::Own => &self.shared_storage,
            DecoderShared::External(value) => value,
        }
    }
    pub fn have_memory_manager(&self) -> bool {
        self.shared().have_memory_manager()
    }
    pub fn memory_manager(&self) -> MemoryManagerPtr {
        self.shared().memory_manager()
    }
    pub fn init(&mut self, header: &FrameHeader<'_>) -> Status {
        let memory_manager = self.memory_manager();
        self.x_dm_multiplier = (1.0f32 / 1.25f32).powf(header.x_qm_scale as f32 - 2.0f32);
        self.b_dm_multiplier = (1.0f32 / 1.25f32).powf(header.b_qm_scale as f32 - 2.0f32);
        self.main_output.callback = PixelCallback::default();
        self.main_output.buffer = std::ptr::null_mut();
        self.extra_output.clear();
        self.fast_xyb_srgb8_conversion = false;
        self.unpremul_alpha = false;
        self.undo_orientation = Orientation::IDENTITY;
        self.used_acs.store(0, Ordering::SeqCst);
        self.upsampler8x = get_upsampling_stage_in(
            memory_manager,
            &self
                .shared()
                .metadata
                .expect("source requires shared metadata")
                .transform_data,
            0,
            3,
        );
        if header.loop_filter.epf_iters > 0 {
            let dim = &self.shared().frame_dim;
            *self.sigma.borrow_mut() = Plane::create_in(
                dim.xsize_blocks.wrapping_add(2 * SIGMA_PADDING),
                dim.ysize_blocks.wrapping_add(2 * SIGMA_PADDING),
                0,
                memory_manager,
            )?;
        }
        Ok(())
    }
    pub fn init_for_ac(&mut self, num_passes: usize) -> Status {
        self.shared_storage.coeff_order_size = 0;
        for o in 0..NUM_VALID_STRATEGIES {
            if ((1u32 << o) & self.used_acs.load(Ordering::SeqCst)) == 0 {
                continue;
            }
            let ord = STRATEGY_ORDER[o as usize] as usize;
            self.shared_storage.coeff_order_size = (COEFF_ORDER_OFFSET[3 * (ord + 1)]
                * DCT_BLOCK_SIZE)
                .max(self.shared_storage.coeff_order_size);
        }
        let size = num_passes.wrapping_mul(self.shared_storage.coeff_order_size);
        if size > self.shared_storage.coeff_orders.len() {
            self.shared_storage.coeff_orders.resize(size, 0);
        }
        Ok(())
    }
}

// PassesDecoderState::PreparePipeline, the non-fast-XYB color branch. The
// fast_xyb_srgb8_conversion branch exists only without JXL_HIGH_PRECISION; the
// pinned reference builds with JXL_HIGH_PRECISION=1 and Init clears the flag.

/// Color-transform selection: the YCbCr stage, or the XYB stage followed by
/// linear output unless the requested encoding is XYB itself. kNone adds
/// nothing. Returns the source's local `linear` flag.
pub fn add_color_transform_stage<'a>(
    color_transform: ColorTransform,
    info: &OutputEncodingInfo,
    builder: &mut RenderPipelineBuilder<'a>,
) -> super::base::status::StatusOr<bool> {
    let mut linear = false;
    if color_transform == ColorTransform::YCBCR {
        builder.add_stage(Some(get_ycbcr_stage()))?;
    } else if color_transform == ColorTransform::XYB {
        builder.add_stage(Some(get_xyb_stage(info)))?;
        if info.color_encoding.get_color_space() != ColorSpace::XYB {
            linear = true;
        }
    }
    Ok(linear)
}

/// The `if (linear)` FromLinear insertion that precedes the coalescing
/// blending stage and the post-color-transform reference writer.
pub fn add_from_linear_stage_before_write<'a>(
    info: &OutputEncodingInfo,
    builder: &mut RenderPipelineBuilder<'a>,
    linear: &mut bool,
) -> Status {
    if *linear {
        builder.add_stage(get_from_linear_stage(info))?;
        *linear = false;
    }
    Ok(())
}

/// Tone mapping, linear-to-output conversion and CMS selection, from
/// GetToneMappingStage through the final CMS stage, before the output writer.
/// CreateICC caches the linear profile in `info`, as in the source member.
/// Returns whether a CMS stage was added, so the caller can apply its
/// pipeline-specific preparation contract.
///
/// GetToLinearStage returns a stage for every transfer function, including
/// its OpInvalid identity, so the source's null-stage CMS fallback inside the
/// tone-mapping branch is unreachable. The Rust factory's non-optional return
/// type encodes that; no reachable branch is omitted.
///
/// # Safety
/// When `info.cms_set` is true, `info.color_management_system` must satisfy
/// the `get_cms_stage` contract for every stage added here until the built
/// pipeline and its stages are dropped. No CMS function is invoked while
/// `cms_set` is false.
pub unsafe fn add_output_color_stages<'a>(
    info: &mut OutputEncodingInfo,
    builder: &mut RenderPipelineBuilder<'a>,
    mut linear: bool,
) -> super::base::status::StatusOr<bool> {
    let mut cms_stage_added = false;
    let tone_mapping_stage = get_tone_mapping_stage(info);
    if let Some(tone_mapping_stage) = tone_mapping_stage {
        if !linear {
            builder.add_stage(Some(get_to_linear_stage(info)))?;
            linear = true;
        }
        builder.add_stage(Some(tone_mapping_stage))?;
    }
    if linear {
        let channels_src = if info.orig_color_encoding.is_cmyk() {
            4
        } else {
            info.orig_color_encoding.channels()
        };
        let channels_dst = info.color_encoding.channels();
        let mixing_color_and_grey = channels_dst != channels_src;
        if info.color_encoding_is_original || !info.cms_set || mixing_color_and_grey {
            // The original/no-CMS/gray-mixing cases need only a linear stage.
            builder.add_stage(get_from_linear_stage(info))?;
        } else {
            if info.linear_color_encoding.create_icc().is_err() {
                return Err(StatusCode::GenericError);
            }
            // SAFETY: cms_set is true here; the caller guarantees the CMS
            // contract for the lifetime of the stage this factory may build.
            let cms_stage = unsafe { get_cms_stage(info, true) };
            if cms_stage.is_some() {
                builder.add_stage(cms_stage)?;
                cms_stage_added = true;
            }
        }
    } else {
        // SAFETY: The factory calls no CMS function unless cms_set is true; in
        // that case the caller guarantees the contract for the stage lifetime.
        let cms_stage = unsafe { get_cms_stage(info, false) };
        if cms_stage.is_some() {
            builder.add_stage(cms_stage)?;
            cms_stage_added = true;
        }
    }
    Ok(cms_stage_added)
}

impl<'a, 'pipeline> PassesDecoderState<'a, 'pipeline> {
    /// PassesDecoderState::PreparePipeline from dec_cache.cc, for callers
    /// outside FrameDecoder (the frame decoder keeps its own interleaved
    /// preparation with write tracing in dec_frame.rs). Every branch is the
    /// source's, in its order, except two admission results:
    /// - `options.coalescing` with NeedsBlending: the blending stage retains
    ///   the shared frame-header owner of the frame decoder's representation;
    ///   both encoder callers (ReconstructImage and the adaptive-quantization
    ///   round trip) pass `coalescing = false`, so the branch is unreached.
    /// - A CMS stage with the simple pipeline, for the same undefined-source
    ///   reason as in the frame decoder's preparation.
    ///
    /// The fast XYB-to-sRGB8 branch is compiled out at the pinned
    /// JXL_HIGH_PRECISION=1. `decoded` is the source's non-owning ImageBundle
    /// pointer; the write stage borrows it only per call.
    ///
    /// # Safety
    /// The contract of `add_output_color_stages` applies to
    /// `self.output_encoding_info`. When `self.main_output` has a callback or
    /// buffer, the descriptors in `main_output` and `extra_output` and the
    /// state's memory manager must satisfy `get_write_to_output_stage`'s
    /// contract until the built pipeline is dropped. A state whose Init
    /// cleared the outputs and whose CMS is not set needs nothing further.
    pub unsafe fn prepare_pipeline<'h>(
        &mut self,
        frame_header: &'pipeline FrameHeader<'h>,
        metadata: &'a super::image_metadata::ImageMetadata,
        decoded: &'pipeline RefCell<ImageBundle<'a>>,
        options: PipelineOptions,
    ) -> Status
    where
        'a: 'pipeline,
    {
        use super::render_pipeline::render_pipeline_stage::{
            WriteToImage3FStage, WriteToImageBundleStage,
        };
        use super::render_pipeline::stage_epf::{EpfStage, get_epf_stage_shared};
        let memory_manager = self.memory_manager();
        let nonserialized = frame_header
            .nonserialized_metadata
            .expect("source frame header carries its metadata");
        let num_c = 3 + nonserialized.m.num_extra_channels as usize;
        let render_noise =
            options.render_noise && (frame_header.flags & super::frame_header::FLAG_NOISE) != 0;
        let num_tmp_c = if render_noise { 3 } else { 0 };

        if frame_header.can_be_referenced() {
            // Necessary so that SetInputSizes() can allocate output buffers.
            let _ = self
                .frame_storage_for_referencing
                .borrow_mut()
                .move_from(&mut ImageBundle::new_in(memory_manager, Some(metadata)));
        }

        let mut builder: RenderPipelineBuilder<'pipeline> =
            RenderPipelineBuilder::new_in(memory_manager, num_c + num_tmp_c);
        if options.use_slow_render_pipeline {
            builder.use_simple_implementation();
        }
        if !frame_header.chroma_subsampling.is_444() {
            for c in 0..3 {
                if frame_header.chroma_subsampling.h_shift(c) != 0 {
                    builder.add_stage(Some(
                        super::render_pipeline::stage_chroma_upsampling::get_chroma_upsampling_stage(
                            c, true,
                        ),
                    ))?;
                }
                if frame_header.chroma_subsampling.v_shift(c) != 0 {
                    builder.add_stage(Some(
                        super::render_pipeline::stage_chroma_upsampling::get_chroma_upsampling_stage(
                            c, false,
                        ),
                    ))?;
                }
            }
        }
        if frame_header.loop_filter.gab {
            builder.add_stage(super::render_pipeline::stage_gaborish::get_gaborish_stage(
                &frame_header.loop_filter,
            ))?;
        }
        {
            let lf = &frame_header.loop_filter;
            if lf.epf_iters >= 3 {
                builder.add_stage(get_epf_stage_shared(lf, self.sigma.clone(), EpfStage::Zero))?;
            }
            if lf.epf_iters >= 1 {
                builder.add_stage(get_epf_stage_shared(lf, self.sigma.clone(), EpfStage::One))?;
            }
            if lf.epf_iters >= 2 {
                builder.add_stage(get_epf_stage_shared(lf, self.sigma.clone(), EpfStage::Two))?;
            }
        }
        let mut late_ec_upsample = frame_header.upsampling != 1;
        for &ecups in &frame_header.extra_channel_upsampling {
            if ecups != frame_header.upsampling {
                // If patches are applied, either upsampling == 1 or
                // late_ec_upsample is true.
                late_ec_upsample = false;
            }
        }
        if !late_ec_upsample {
            for (ec, &up) in frame_header.extra_channel_upsampling.iter().enumerate() {
                if up != 1 {
                    builder.add_stage(get_upsampling_stage_in(
                        memory_manager,
                        &nonserialized.transform_data,
                        3 + ec,
                        super::base::bits::ceil_log2_nonzero(up),
                    ))?;
                }
            }
        }
        if (frame_header.flags & super::frame_header::FLAG_PATCHES) != 0 {
            builder.add_stage(Some(
                super::render_pipeline::stage_patches::get_patches_stage_shared(
                    self.shared().image_features.patches.clone(),
                    self.shared().reference_frames.clone(),
                    &nonserialized.m.extra_channel_info,
                ),
            ))?;
        }
        if (frame_header.flags & super::frame_header::FLAG_SPLINES) != 0 {
            builder.add_stage(Some(Box::new(
                super::render_pipeline::stage_splines::get_spline_stage_shared(
                    self.shared().image_features.splines.clone(),
                ),
            )))?;
        }
        if frame_header.upsampling != 1 {
            let nb_channels = 3 + if late_ec_upsample {
                frame_header.extra_channel_upsampling.len()
            } else {
                0
            };
            for c in 0..nb_channels {
                builder.add_stage(get_upsampling_stage_in(
                    memory_manager,
                    &nonserialized.transform_data,
                    c,
                    super::base::bits::ceil_log2_nonzero(frame_header.upsampling),
                ))?;
            }
        }
        // From here on all stages have zero xextra.
        if render_noise {
            builder.add_stage(Some(
                super::render_pipeline::stage_noise::get_convolve_noise_stage(num_c),
            ))?;
            builder.add_stage(Some(
                super::render_pipeline::stage_noise::get_add_noise_stage_shared(
                    self.shared().image_features.noise_params.clone(),
                    self.shared().cmap.base_shared(),
                    num_c,
                ),
            ))?;
        }
        if frame_header.dc_level != 0 {
            builder.add_stage(Some(Box::new(
                WriteToImage3FStage::from_dc_frame(
                    self.shared_storage.dc_frames.clone(),
                    frame_header.dc_level as usize - 1,
                )
                .with_memory_manager(memory_manager),
            )))?;
        }
        if frame_header.can_be_referenced() && frame_header.save_before_color_transform {
            builder.add_stage(Some(Box::new(WriteToImageBundleStage::from_owned(
                self.frame_storage_for_referencing.clone(),
                &metadata.color_encoding,
            ))))?;
        }
        let mut has_alpha = false;
        let mut alpha_c = 0;
        for (i, eci) in metadata.extra_channel_info.iter().enumerate() {
            if eci.channel_type == super::image_metadata::ExtraChannel::ALPHA {
                has_alpha = true;
                alpha_c = 3 + i;
                break;
            }
        }
        // fast_xyb_srgb8_conversion is compiled out (JXL_HIGH_PRECISION=1).
        let mut linear = add_color_transform_stage(
            frame_header.color_transform,
            &self.output_encoding_info,
            &mut builder,
        )?;
        if options.coalescing && super::blending::needs_blending(frame_header) {
            // Admission result; see the function documentation.
            return Err(StatusCode::Unsupported);
        }
        if options.coalescing
            && frame_header.can_be_referenced()
            && !frame_header.save_before_color_transform
        {
            add_from_linear_stage_before_write(
                &self.output_encoding_info,
                &mut builder,
                &mut linear,
            )?;
            builder.add_stage(Some(Box::new(WriteToImageBundleStage::from_owned(
                self.frame_storage_for_referencing.clone(),
                &metadata.color_encoding,
            ))))?;
        }
        if options.render_spotcolors
            && nonserialized
                .m
                .find(super::image_metadata::ExtraChannel::SPOT_COLOR)
                .is_some()
        {
            for (i, eci) in metadata.extra_channel_info.iter().enumerate() {
                // Not Find: there may be multiple spot color channels.
                if eci.channel_type == super::image_metadata::ExtraChannel::SPOT_COLOR {
                    builder.add_stage(Some(
                        super::render_pipeline::stage_spot::get_spot_color_stage(
                            i,
                            &eci.spot_color,
                        ),
                    ))?;
                }
            }
        }
        // SAFETY: The caller supplies the CMS contract of this function.
        let cms_stage = unsafe {
            add_output_color_stages(&mut self.output_encoding_info, &mut builder, linear)
        }?;
        if cms_stage && options.use_slow_render_pipeline {
            // Admission result; see the function documentation.
            return Err(StatusCode::Unsupported);
        }
        if self.main_output.callback.is_present() || !self.main_output.buffer.is_null() {
            // SAFETY: The caller guarantees the output descriptors and the
            // memory manager for the lifetime of the built pipeline.
            let stage = unsafe {
                super::render_pipeline::stage_write::get_write_to_output_stage(
                    &self.main_output,
                    self.width,
                    self.height,
                    has_alpha,
                    self.unpremul_alpha,
                    alpha_c,
                    self.undo_orientation,
                    &self.extra_output,
                    memory_manager.as_raw(),
                )
            };
            builder.add_stage(Some(stage))?;
        } else {
            builder.add_stage(Some(Box::new(WriteToImageBundleStage::from_shared(
                decoded,
                &self.output_encoding_info.color_encoding,
            ))))?;
        }
        let frame_dim = self.shared().frame_dim;
        self.render_pipeline = Some(builder.finalize(frame_dim)?);
        self.render_pipeline.as_ref().unwrap().is_initialized()
    }
}
