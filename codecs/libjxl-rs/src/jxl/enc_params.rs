// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! CompressParams, defaults, setters, and predicates from lib/jxl/enc_params.h.
//! Non-owning spline/progressive inputs borrow their callers. CMS/debug function
//! pointers remain non-owning descriptors; this module never invokes them.
//! Safe initial CMS/opaque storage has no source meaning before assignment.

use super::base::override_option::Override;
use super::cms_interface::JxlCmsInterface;
use super::color_encoding::JxlColorEncoding;
use super::common::SpeedTier;
use super::enc_progressive_split::ProgressiveMode;
use super::frame_dimensions::BLOCK_DIM;
use super::frame_header::ColorTransform;
use super::modular::encoding::dec_ma::Tree;
use super::modular::options::ModularOptions;
use super::splines::SplineDataView;
use std::ffi::{c_char, c_void};

// lib/include/jxl/encode.h's callback carrier. Execution remains unported.
pub type JxlDebugImageCallback = unsafe extern "C" fn(
    *mut c_void,
    *const c_char,
    usize,
    usize,
    *const JxlColorEncoding,
    *const u16,
);
#[derive(Clone)]
pub struct CompressParams<'a> {
    pub butteraugli_distance: f32,
    pub ec_distance: Vec<f32>,
    pub max_error_mode: bool,
    pub max_error: [f32; 3],
    pub disable_perceptual_optimizations: bool,
    pub speed_tier: SpeedTier,
    pub brotli_effort: i32,
    pub decoding_speed_tier: usize,
    pub color_transform: ColorTransform,
    pub modular_mode: bool,
    pub modular_group_size_shift: i32,
    pub preview: Override,
    pub noise: Override,
    pub dots: Override,
    pub patches: Override,
    pub gaborish: Override,
    pub epf: i32,
    pub progressive_mode: Override,
    pub qprogressive_mode: Override,
    pub centerfirst: bool,
    pub center_x: usize,
    pub center_y: usize,
    pub progressive_dc: i32,
    pub keep_invisible: Override,
    pub cms: JxlCmsInterface,
    pub cms_set: bool,
    pub force_cfl_jpeg_recompression: bool,
    pub force_lfs_jpeg_recompression: i32,
    pub jpeg_compress_boxes: bool,
    pub jpeg_keep_exif: bool,
    pub jpeg_keep_xmp: bool,
    pub jpeg_keep_jumbf: bool,
    pub photon_noise_iso: f32,
    pub options: ModularOptions,
    pub responsive: i32,
    pub colorspace: i32,
    pub move_to_front_from_channel: i32,
    pub channel_colors_pre_transform_percent: f32,
    pub channel_colors_percent: f32,
    pub palette_colors: i32,
    pub lossy_palette: bool,
    pub resampling: i32,
    pub ec_resampling: i32,
    pub already_downsampled: bool,
    pub original_butteraugli_distance: f32,
    pub quant_ac_rescale: f32,
    pub level: i32,
    pub buffering: i32,
    pub output_mode: i32,
    pub use_full_image_heuristics: bool,
    pub manual_noise: Vec<f32>,
    pub manual_xyb_factors: Vec<f32>,
    pub custom_fixed_tree: Tree,
    pub custom_splines: SplineDataView<'a>,
    pub custom_progressive_mode: Option<&'a ProgressiveMode>,
    pub debug_image: Option<JxlDebugImageCallback>,
    pub debug_image_opaque: *mut c_void,
}
impl Default for CompressParams<'_> {
    fn default() -> Self {
        Self {
            butteraugli_distance: 1.0,
            ec_distance: Vec::new(),
            max_error_mode: false,
            max_error: [0.0; 3],
            disable_perceptual_optimizations: false,
            speed_tier: SpeedTier::Squirrel,
            brotli_effort: -1,
            decoding_speed_tier: 0,
            color_transform: ColorTransform::XYB,
            modular_mode: false,
            modular_group_size_shift: -1,
            preview: Override::DEFAULT,
            noise: Override::DEFAULT,
            dots: Override::DEFAULT,
            patches: Override::DEFAULT,
            gaborish: Override::DEFAULT,
            epf: -1,
            progressive_mode: Override::DEFAULT,
            qprogressive_mode: Override::DEFAULT,
            centerfirst: false,
            center_x: usize::MAX,
            center_y: usize::MAX,
            progressive_dc: -1,
            keep_invisible: Override::DEFAULT,
            cms: JxlCmsInterface::default(),
            cms_set: false,
            force_cfl_jpeg_recompression: true,
            force_lfs_jpeg_recompression: -1,
            jpeg_compress_boxes: true,
            jpeg_keep_exif: true,
            jpeg_keep_xmp: true,
            jpeg_keep_jumbf: true,
            photon_noise_iso: 0.0,
            options: ModularOptions::default(),
            responsive: -1,
            colorspace: -1,
            move_to_front_from_channel: -1,
            channel_colors_pre_transform_percent: 95.0,
            channel_colors_percent: 80.0,
            palette_colors: 1 << 10,
            lossy_palette: false,
            resampling: -1,
            ec_resampling: -1,
            already_downsampled: false,
            original_butteraugli_distance: -1.0,
            quant_ac_rescale: 1.0,
            level: -1,
            buffering: -1,
            output_mode: 0,
            use_full_image_heuristics: true,
            manual_noise: Vec::new(),
            manual_xyb_factors: Vec::new(),
            custom_fixed_tree: Vec::new(),
            custom_splines: SplineDataView::default(),
            custom_progressive_mode: None,
            debug_image: None,
            debug_image_opaque: std::ptr::null_mut(),
        }
    }
}
impl CompressParams<'_> {
    pub fn set_cms(&mut self, cms: &JxlCmsInterface) {
        self.cms = *cms;
        self.cms_set = true;
    }
    pub fn is_lossless(&self) -> bool {
        self.modular_mode && self.modular_part_is_lossless()
    }
    pub fn modular_part_is_lossless(&self) -> bool {
        if self.modular_mode
            && (self.butteraugli_distance != 0.0 || self.color_transform == ColorTransform::XYB)
        {
            return false;
        }
        for &f in &self.ec_distance {
            if f > 0.0 {
                return false;
            }
            if f < 0.0 && self.butteraugli_distance != 0.0 {
                return false;
            }
        }
        true
    }
    pub fn set_lossless(&mut self) {
        self.modular_mode = true;
        self.butteraugli_distance = 0.0;
        self.ec_distance.fill(0.0);
        self.color_transform = ColorTransform::NONE;
    }
}
pub const MIN_BUTTERAUGLI_FOR_DYNAMIC_AR: f32 = 0.5;
pub const MIN_BUTTERAUGLI_FOR_DOTS: f32 = 3.0;
pub const MIN_BUTTERAUGLI_TO_SUBTRACT_ORIGINAL_PATCHES: f32 = 3.0;
pub const MIN_BUTTERAUGLI_FOR_NOISE: f32 = 99.0;
pub const MIN_BUTTERAUGLI_DISTANCE: f32 = 0.05;
pub const ENC_TILE_DIM: usize = 64;
pub const ENC_TILE_DIM_IN_BLOCKS: usize = ENC_TILE_DIM / BLOCK_DIM;
