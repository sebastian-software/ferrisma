// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! ImageBundle storage and constructors from lib/jxl/image_bundle.h. The
//! bundle retains its memory manager; `new` selects the native allocator
//! boundary and `new_in` the source manager pointer. Size/color/extra
//! access, SetFromImage, and VerifySizes retain source checkpoints. General
//! alpha access and VerifyMetadata are included. General import/export, copy,
//! and remaining methods are absent.
use super::base::libcpp_vector::Vector;
use super::base::status::{Status, StatusCode};
use super::color_encoding_internal::ColorEncoding;
use super::frame_header::{BlendMode, ColorTransform, FrameOrigin, YCbCrChromaSubsampling};
use super::image::{Image3, Plane};
use super::image_metadata::{ExtraChannel, ImageMetadata};
use super::jpeg::jpeg_data::JpegData;
use super::memory_manager_internal::MemoryManagerPtr;

/// Borrowed representation of the source color aliases after an original
/// ImageBundle move. The destination owns the allocation; this view prevents
/// its mutation or destruction while the original aliases are observed.
pub struct MovedColorView<'a> {
    color: &'a Image3<f32>,
}
impl MovedColorView<'_> {
    pub fn color(&self) -> &Image3<f32> {
        self.color
    }
}

pub struct ImageBundle<'a> {
    memory_manager: MemoryManagerPtr,
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
    // Declared before `color`, unlike the source: Rust drops fields in
    // declaration order, so this reproduces the implicit destructor's reverse
    // member order (the extra channels, last first, before the color planes).
    // Assignment uses `move_from`, which keeps the source member order.
    extra_channels: Vector<Plane<f32>>,
    color: Image3<f32>,
    c_current: ColorEncoding,
    decoded_bytes: usize,
}
impl<'a> ImageBundle<'a> {
    pub fn new(metadata: Option<&'a ImageMetadata>) -> Self {
        Self::new_in(MemoryManagerPtr::NATIVE, metadata)
    }
    /// The source ImageBundle(memory_manager, metadata) constructor.
    pub fn new_in(memory_manager: MemoryManagerPtr, metadata: Option<&'a ImageMetadata>) -> Self {
        Self {
            memory_manager,
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
            extra_channels: Vector::default(),
            decoded_bytes: 0,
        }
    }
    pub fn have_memory_manager(&self) -> bool {
        self.memory_manager.is_some()
    }
    pub fn memory_manager(&self) -> MemoryManagerPtr {
        self.memory_manager
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
        self.color.assign(color.take().unwrap());
        self.c_current.clone_from(c_current);
        self.verify_sizes()
    }
    /// Original RemoveColor: reset only the color planes, retaining metadata,
    /// profile, origin, extra planes and frame fields.
    pub fn remove_color(&mut self) {
        self.color.assign(Image3::default());
    }
    /// Original default move assignment between distinct bundle objects.
    /// The destination object keeps its identity. Scalar fields and color-plane
    /// headers remain in the source; its moved allocation is observed through
    /// the destination, never through a second Rust storage owner. The returned
    /// view represents source color samples; source.color() retains only the
    /// scalar headers after this transfer, without an allocation owner.
    pub fn move_from<'d>(&'d mut self, source: &mut Self) -> MovedColorView<'d> {
        self.memory_manager = source.memory_manager;
        self.jpeg_data = source.jpeg_data.take();
        self.color_transform = source.color_transform;
        self.chroma_subsampling
            .clone_from(&source.chroma_subsampling);
        self.origin = source.origin;
        self.duration = source.duration;
        self.timecode = source.timecode;
        self.use_for_next_frame = source.use_for_next_frame;
        self.blend = source.blend;
        self.blendmode = source.blendmode;
        self.name = std::mem::take(&mut source.name);
        self.metadata = source.metadata;
        self.color.move_from(&mut source.color);
        self.c_current.move_from(&mut source.c_current);
        super::base::libcpp_vector::move_assign(
            &mut self.extra_channels.0,
            std::mem::take(&mut source.extra_channels.0),
        );
        self.decoded_bytes = source.decoded_bytes;
        MovedColorView { color: &self.color }
    }
    pub fn verify_sizes(&self) -> Status {
        let xs = self.xsize();
        let ys = self.ysize();
        if self.has_extra_channels() {
            if xs == 0 || ys == 0 {
                return Err(StatusCode::GenericError);
            }
            for ec in self.extra_channels.iter() {
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
    pub fn has_alpha(&self) -> bool {
        self.metadata
            .expect("source requires metadata")
            .find(ExtraChannel::ALPHA)
            .is_some()
    }
    pub fn alpha_is_premultiplied(&self) -> bool {
        self.metadata
            .expect("source requires metadata")
            .find(ExtraChannel::ALPHA)
            .is_some_and(|a| a.alpha_associated)
    }
    pub fn alpha(&self) -> Option<&Plane<f32>> {
        if !self.has_alpha() {
            return None;
        }
        let index = self
            .metadata
            .unwrap()
            .extra_channel_info
            .iter()
            .position(|a| a.channel_type == ExtraChannel::ALPHA)
            .unwrap();
        Some(&self.extra_channels[index])
    }
    pub fn verify_metadata(&self) -> Status {
        if self.c_current.icc().is_empty() {
            return Err(StatusCode::GenericError);
        }
        let metadata = self.metadata.expect("source requires metadata");
        if metadata.color_encoding.is_gray() != self.c_current.is_gray() {
            return Err(StatusCode::GenericError);
        }
        if metadata.has_alpha() && self.alpha().expect("source requires alpha plane").xsize() == 0 {
            return Err(StatusCode::GenericError);
        }
        if metadata.get_alpha_bits() > 32 {
            return Err(StatusCode::GenericError);
        }
        Ok(())
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
