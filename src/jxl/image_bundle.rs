// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! ImageBundle storage and constructors from lib/jxl/image_bundle.h. Native
//! allocator context replaces the source manager pointer. Image import/export,
//! copy, metadata validation, and remaining methods are absent until translated.
use super::color_encoding_internal::ColorEncoding;
use super::frame_header::{BlendMode, ColorTransform, FrameOrigin, YCbCrChromaSubsampling};
use super::image::{Image3, Plane};
use super::image_metadata::ImageMetadata;
use super::jpeg::jpeg_data::JpegData;

pub struct ImageBundle<'a> {
    memory_manager_present: bool,
    pub jpeg_data: Option<Box<JpegData>>,
    pub color_transform: ColorTransform,
    pub chroma_subsampling: YCbCrChromaSubsampling,
    pub origin: FrameOrigin,
    pub duration: u32,
    pub timecode: u32,
    pub use_for_next_frame: bool,
    pub blend: bool,
    pub blendmode: BlendMode,
    pub name: Vec<u8>,
    metadata: Option<&'a ImageMetadata>,
    color: Image3<f32>,
    c_current: ColorEncoding,
    extra_channels: Vec<Plane<f32>>,
    decoded_bytes: usize,
}
impl<'a> ImageBundle<'a> {
    pub fn new(metadata: Option<&'a ImageMetadata>) -> Self {
        Self {
            memory_manager_present: true,
            jpeg_data: None,
            color_transform: ColorTransform::NONE,
            chroma_subsampling: YCbCrChromaSubsampling::default(),
            origin: FrameOrigin { x0: 0, y0: 0 },
            duration: 0,
            timecode: 0,
            use_for_next_frame: false,
            blend: false,
            blendmode: BlendMode::BLEND,
            name: Vec::new(),
            metadata,
            color: Image3::default(),
            c_current: ColorEncoding::default(),
            extra_channels: Vec::new(),
            decoded_bytes: 0,
        }
    }
    pub fn have_memory_manager(&self) -> bool {
        self.memory_manager_present
    }
    pub fn metadata(&self) -> Option<&ImageMetadata> {
        self.metadata
    }
    pub fn color(&self) -> &Image3<f32> {
        &self.color
    }
    pub fn c_current(&self) -> &ColorEncoding {
        &self.c_current
    }
    pub fn extra_channels(&self) -> &[Plane<f32>] {
        &self.extra_channels
    }
    pub fn decoded_bytes(&self) -> usize {
        self.decoded_bytes
    }
    pub fn set_decoded_bytes(&mut self, bytes: usize) {
        self.decoded_bytes = bytes;
    }
}
