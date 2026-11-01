// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Channel and Image storage from lib/jxl/modular/modular_image.{h,cc}.
//! Rust moves transfer plane ownership. Explicit clone_image performs the
//! source copy; transform execution and diagnostic strings remain unported.

use super::transform::transform::Transform;
use crate::jxl::base::status::{Status, StatusOr};
use crate::jxl::image::Plane;
use crate::jxl::image_ops::copy_image_to;
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
        let plane = Plane::create(w, h)?;
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
        self.plane = Plane::create_in(self.w, self.h, 0, self.plane.have_memory_manager())?;
        Ok(())
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
}
impl Default for Image {
    fn default() -> Self {
        Self {
            channel: Vec::new(),
            transform: Vec::new(),
            w: 0,
            h: 0,
            bitdepth: 8,
            nb_meta_channels: 0,
            error: true,
        }
    }
}
impl Image {
    fn new(w: usize, h: usize, bitdepth: i32) -> Self {
        Self {
            w,
            h,
            bitdepth,
            error: false,
            ..Self::default()
        }
    }
    pub fn create(w: usize, h: usize, bitdepth: i32, channels: i32) -> StatusOr<Self> {
        let mut result = Self::new(w, h, bitdepth);
        for i in 0..channels {
            let channel = Channel::create(w, h)?;
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
        let mut result = Self::new(that.w, that.h, that.bitdepth);
        result.nb_meta_channels = that.nb_meta_channels;
        result.error = that.error;
        result.transform = that.transform.clone();
        for ch in &that.channel {
            let mut copy = Channel::create_with_shifts(ch.w, ch.h, ch.hshift, ch.vshift)?;
            copy_image_to(&ch.plane, &mut copy.plane)?;
            copy.component = ch.component;
            result.channel.push(copy);
        }
        Ok(result)
    }
}
