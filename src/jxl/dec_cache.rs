// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! PixelCallback/ImageOutput and PassesDecoderState storage/constructor/Init
//! from dec_cache.h, plus complete null-pool InitForAC from dec_cache.cc.
//! Pipeline storage admits the original simple implementation; general pipeline
//! preparation, GroupDecCache, and custom thread runners remain unported.
//! Source-indeterminate scalar storage is initialized safely, without source
//! meaning until assigned. Callback/output pointers remain non-owning carriers.
use super::ac_strategy::NUM_VALID_STRATEGIES;
use super::base::status::Status;
use super::coeff_order::{COEFF_ORDER_OFFSET, STRATEGY_ORDER};
use super::dct_util::{AcImage, AcType};
use super::dec_ans::ANSCode;
use super::dec_xyb::OutputEncodingInfo;
use super::frame_dimensions::DCT_BLOCK_SIZE;
use super::frame_header::FrameHeader;
use super::image::Plane;
use super::image_bundle::ImageBundle;
use super::image_metadata::Orientation;
use super::passes_state::PassesSharedState;
use super::render_pipeline::render_pipeline_stage::RenderPipelineStage;
use super::render_pipeline::simple_render_pipeline::SimpleRenderPipeline;
use super::render_pipeline::stage_upsampling::get_upsampling_stage;
use super::types::{JxlDataType, JxlEndianness, JxlPixelFormat};
use std::ffi::c_void;
use std::sync::atomic::{AtomicU32, Ordering};

pub const SIGMA_BORDER: usize = 1;
pub const SIGMA_PADDING: usize = 2;
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
    pub sigma: Plane<f32>,
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
            sigma: Plane::default(),
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
            self.sigma = Plane::create(
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
