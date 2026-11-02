// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! PixelCallback/ImageOutput and PassesDecoderState storage/constructor/Init
//! from dec_cache.h, plus complete null-pool InitForAC from dec_cache.cc.
//! Pipeline storage admits the original simple implementation; general pipeline
//! preparation and custom runners remain unported.
//! Source-indeterminate scalar storage is initialized safely, without source
//! meaning until assigned. Callback/output pointers remain non-owning carriers.
use super::ac_strategy::NUM_VALID_STRATEGIES;
use super::base::status::Status;
use super::coeff_order::{COEFF_ORDER_OFFSET, STRATEGY_ORDER};
use super::common::MAX_NUM_PASSES;
use super::dct_util::{AcImage, AcType};
use super::dec_ans::ANSCode;
use super::dec_xyb::OutputEncodingInfo;
use super::frame_dimensions::DCT_BLOCK_SIZE;
use super::frame_header::FrameHeader;
use super::image::Image3;
use super::image::Plane;
use super::image_bundle::ImageBundle;
use super::image_metadata::Orientation;
use super::memory_manager_internal::AlignedMemory;
use super::passes_state::PassesSharedState;
use super::render_pipeline::render_pipeline_stage::RenderPipelineStage;
use super::render_pipeline::simple_render_pipeline::SimpleRenderPipeline;
use super::render_pipeline::stage_upsampling::get_upsampling_stage;
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
        for i in 0..num_passes {
            if self.num_nzeroes[i].xsize() == 0 {
                let dim = super::frame_dimensions::GROUP_DIM_IN_BLOCKS;
                self.num_nzeroes[i] = Image3::create(dim, dim)?;
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
            self.float_memory = Some(AlignedMemory::create(self.max_block_area * 7 * 4, 0)?);
            self.int32_memory = Some(AlignedMemory::create(self.max_block_area * 3 * 4, 0)?);
            self.int16_memory = Some(AlignedMemory::create(self.max_block_area * 3 * 2, 0)?);
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
        let (block, scratch) = float.split_at_mut(self.max_block_area * 3);
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
        if self.dc_buffer.xsize() == 0 {
            let dim = super::frame_dimensions::GROUP_DIM_IN_BLOCKS;
            let offset = super::render_pipeline::render_pipeline_stage::RENDER_PIPELINE_X_OFFSET;
            self.dc_buffer = Plane::create(dim + offset * 2, dim + 4)?;
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
    pub render_pipeline: Option<Box<SimpleRenderPipeline<'pipeline>>>,
    pub frame_storage_for_referencing: ImageBundle<'a>,
    pub output_encoding_info: OutputEncodingInfo,
}
impl Default for PassesDecoderState<'_, '_> {
    fn default() -> Self {
        Self::new()
    }
}
impl<'a, 'pipeline> PassesDecoderState<'a, 'pipeline> {
    pub fn new() -> Self {
        Self {
            shared_storage: PassesSharedState::new(),
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
            frame_storage_for_referencing: ImageBundle::new(None),
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
    pub fn init(&mut self, header: &FrameHeader<'_>) -> Status {
        self.x_dm_multiplier = (1.0f32 / 1.25f32).powf(header.x_qm_scale as f32 - 2.0f32);
        self.b_dm_multiplier = (1.0f32 / 1.25f32).powf(header.b_qm_scale as f32 - 2.0f32);
        self.main_output.callback = PixelCallback::default();
        self.main_output.buffer = std::ptr::null_mut();
        self.extra_output.clear();
        self.fast_xyb_srgb8_conversion = false;
        self.unpremul_alpha = false;
        self.undo_orientation = Orientation::IDENTITY;
        self.used_acs.store(0, Ordering::SeqCst);
        self.upsampler8x = get_upsampling_stage(
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
            *self.sigma.borrow_mut() = Plane::create(
                dim.xsize_blocks.wrapping_add(2 * SIGMA_PADDING),
                dim.ysize_blocks.wrapping_add(2 * SIGMA_PADDING),
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
