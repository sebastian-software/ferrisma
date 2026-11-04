// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Channel and Image storage from lib/jxl/modular/modular_image.{h,cc}.
//! Rust moves transfer plane ownership. Explicit clone_image performs the
//! source copy; undo_transforms retains the original stack and failure state.
//! Planes allocate through the image's memory manager; the shorter
//! constructors use the native allocator boundary. Diagnostic strings and
//! custom runners remain unported.

use super::encoding::context_predict::weighted;
use super::transform::transform::Transform;
use crate::jxl::base::libcpp_vector;
use crate::jxl::base::status::{Status, StatusOr};
use crate::jxl::image::Plane;
use crate::jxl::image_ops::copy_image_to;
use crate::jxl::memory_manager_internal::MemoryManagerPtr;
pub type Pixel = i32;
pub type PixelWide = i64;
pub struct Channel {
    pub plane: Plane<Pixel>,
    pub w: usize,
    pub h: usize,
    pub hshift: i32,
    pub vshift: i32,
    pub component: i32,
}
impl Channel {
    pub fn create(w: usize, h: usize) -> StatusOr<Self> {
        Self::create_with_shifts(w, h, 0, 0)
    }
    pub fn create_with_shifts(w: usize, h: usize, hshift: i32, vshift: i32) -> StatusOr<Self> {
        Self::create_in(MemoryManagerPtr::NATIVE, w, h, hshift, vshift)
    }
    /// The source Channel::Create(memory_manager, iw, ih, hsh, vsh).
    pub fn create_in(
        memory_manager: MemoryManagerPtr,
        w: usize,
        h: usize,
        hshift: i32,
        vshift: i32,
    ) -> StatusOr<Self> {
        let plane = Plane::create_in(w, h, 0, memory_manager)?;
        Ok(Self {
            plane,
            w,
            h,
            hshift,
            vshift,
            component: -1,
        })
    }
    pub fn shrink(&mut self) -> Status {
        if self.plane.xsize() == self.w && self.plane.ysize() == self.h {
            return Ok(());
        }
        self.plane = Plane::create_in(self.w, self.h, 0, self.memory_manager())?;
        Ok(())
    }
    pub fn memory_manager(&self) -> MemoryManagerPtr {
        self.plane.memory_manager()
    }
    pub fn shrink_to(&mut self, w: i32, h: i32) -> Status {
        self.w = w as usize;
        self.h = h as usize;
        self.shrink()
    }
    pub fn row(&self, y: usize) -> &[Pixel] {
        self.plane.row(y)
    }
    pub fn row_mut(&mut self, y: usize) -> &mut [Pixel] {
        self.plane.row_mut(y)
    }
}
pub struct Image {
    pub channel: Vec<Channel>,
    pub transform: Vec<Transform>,
    pub w: usize,
    pub h: usize,
    pub bitdepth: i32,
    pub nb_meta_channels: usize,
    pub error: bool,
    memory_manager: MemoryManagerPtr,
}
impl Default for Image {
    fn default() -> Self {
        Self::empty_in(MemoryManagerPtr::NATIVE)
    }
}
impl Drop for Image {
    /// The implicit destructor: std::vector releases its channels last first.
    fn drop(&mut self) {
        libcpp_vector::clear(&mut self.channel);
    }
}
impl Image {
    /// The source Image(JxlMemoryManager*) constructor.
    pub fn empty_in(memory_manager: MemoryManagerPtr) -> Self {
        Self {
            channel: Vec::new(),
            transform: Vec::new(),
            w: 0,
            h: 0,
            bitdepth: 8,
            nb_meta_channels: 0,
            error: true,
            memory_manager,
        }
    }
    pub fn memory_manager(&self) -> MemoryManagerPtr {
        self.memory_manager
    }
    /// Image's original move assignment transfers its vectors and scalar
    /// metadata into `self`, which retains its own memory manager. The old
    /// channels are released at the vector assignment, as in the source.
    pub fn move_assign(&mut self, other: &mut Self) {
        self.w = other.w;
        self.h = other.h;
        self.bitdepth = other.bitdepth;
        self.nb_meta_channels = other.nb_meta_channels;
        self.error = other.error;
        libcpp_vector::move_assign(&mut self.channel, std::mem::take(&mut other.channel));
        self.transform = std::mem::take(&mut other.transform);
    }
    pub fn undo_transforms(&mut self, wp_header: &weighted::Header) {
        while let Some(t) = self.transform.last().cloned() {
            if t.inverse(self, wp_header).is_err() {
                self.error = true;
                return;
            }
            self.transform.pop();
        }
    }
    fn new(memory_manager: MemoryManagerPtr, w: usize, h: usize, bitdepth: i32) -> Self {
        let mut result = Self::empty_in(memory_manager);
        result.w = w;
        result.h = h;
        result.bitdepth = bitdepth;
        result.error = false;
        result
    }
    pub fn create(w: usize, h: usize, bitdepth: i32, channels: i32) -> StatusOr<Self> {
        Self::create_in(MemoryManagerPtr::NATIVE, w, h, bitdepth, channels)
    }
    /// The source Image::Create(memory_manager, iw, ih, bitdepth, nb_chans).
    pub fn create_in(
        memory_manager: MemoryManagerPtr,
        w: usize,
        h: usize,
        bitdepth: i32,
        channels: i32,
    ) -> StatusOr<Self> {
        let mut result = Self::new(memory_manager, w, h, bitdepth);
        for i in 0..channels {
            let channel = Channel::create_in(memory_manager, w, h, 0, 0)?;
            result.channel.push(channel);
            result.channel.last_mut().unwrap().component = i;
        }
        Ok(result)
    }
    pub fn empty(&self) -> bool {
        for ch in &self.channel {
            if ch.w != 0 && ch.h != 0 {
                return false;
            }
        }
        true
    }
    pub fn clone_image(that: &Self) -> StatusOr<Self> {
        let memory_manager = that.memory_manager();
        let mut result = Self::new(memory_manager, that.w, that.h, that.bitdepth);
        result.nb_meta_channels = that.nb_meta_channels;
        result.error = that.error;
        result.transform = that.transform.clone();
        for ch in &that.channel {
            let mut copy = Channel::create_in(memory_manager, ch.w, ch.h, ch.hshift, ch.vshift)?;
            copy_image_to(&ch.plane, &mut copy.plane)?;
            copy.component = ch.component;
            result.channel.push(copy);
        }
        Ok(result)
    }
}
