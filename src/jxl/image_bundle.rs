// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! ImageBundle storage and constructors from lib/jxl/image_bundle.h. Native
//! allocator context replaces the source manager pointer. Size/color/extra
//! access, SetFromImage, and VerifySizes retain source checkpoints. General
//! import/export, copy, metadata validation, and remaining methods are absent.
use super::base::status::{Status, StatusCode};
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
    pub fn color_mut(&mut self) -> &mut Image3<f32> {
        &mut self.color
    }
    pub fn has_color(&self) -> bool {
        self.color.xsize() != 0
    }
    pub fn has_extra_channels(&self) -> bool {
        !self.extra_channels.is_empty()
    }
    pub fn xsize(&self) -> usize {
        // common.h enables JPEGXL_ENABLE_TRANSCODE_JPEG in the pinned build.
        if let Some(jpeg) = &self.jpeg_data {
            return jpeg.width as usize;
        }
        if self.color.xsize() != 0 {
            return self.color.xsize();
        }
        self.extra_channels.first().map_or(0, Plane::xsize)
    }
    pub fn ysize(&self) -> usize {
        if let Some(jpeg) = &self.jpeg_data {
            return jpeg.height as usize;
        }
        if self.color.ysize() != 0 {
            return self.color.ysize();
        }
        self.extra_channels.first().map_or(0, Plane::ysize)
    }
    /// Conditional transfer of the named C++ rvalue reference: guards retain
    /// Some(input), while the original move consumes it into None. A moved-from
    /// C++ plane's non-owning raw alias is not exposed as another native owner.
    /// The profile borrows disjoint storage, as at the render-stage call site.
    pub fn set_from_image(
        &mut self,
        color: &mut Option<Image3<f32>>,
        c_current: &ColorEncoding,
    ) -> Status {
        let source = color
            .as_ref()
            .expect("source requires a live image argument");
        if source.xsize() == 0 || source.ysize() == 0 {
            return Err(StatusCode::GenericError);
        }
        if self
            .metadata
            .expect("source requires image metadata")
            .color_encoding
            .is_gray()
            != c_current.is_gray()
        {
            return Err(StatusCode::GenericError);
        }
        self.color = color.take().unwrap();
        self.c_current.clone_from(c_current);
        self.verify_sizes()
    }
    pub fn verify_sizes(&self) -> Status {
        let xs = self.xsize();
        let ys = self.ysize();
        if self.has_extra_channels() {
            if xs == 0 || ys == 0 {
                return Err(StatusCode::GenericError);
            }
            for ec in &self.extra_channels {
                if ec.xsize() != xs {
                    return Err(StatusCode::GenericError);
                }
                if ec.ysize() != ys {
                    return Err(StatusCode::GenericError);
                }
            }
        }
        Ok(())
    }
    pub fn c_current(&self) -> &ColorEncoding {
        &self.c_current
    }
    pub fn extra_channels(&self) -> &[Plane<f32>] {
        &self.extra_channels
    }
    pub fn extra_channels_mut(&mut self) -> &mut Vec<Plane<f32>> {
        &mut self.extra_channels
    }
    pub fn decoded_bytes(&self) -> usize {
        self.decoded_bytes
    }
    pub fn set_decoded_bytes(&mut self, bytes: usize) {
        self.decoded_bytes = bytes;
    }
}
