// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! BitsPerChannel, BytesPerPixel, and the buffer-backed input path of
//! JxlEncoderChunkedFrameAdapter from lib/jxl/encode_internal.h. Callback/JPEG
//! input, CopyFromBuffer, queued frames, and output processors remain unported.
//! Native borrows replace the buffer path's callback table and no-op release.
//! The source's nested Channel is named FrameInputChannel in Rust.

use super::types::{JxlDataType, JxlPixelFormat};

pub fn bits_per_channel(data_type: JxlDataType) -> usize {
    match data_type {
        JxlDataType::UINT8 => 8,
        JxlDataType::UINT16 => 16,
        JxlDataType::FLOAT => 32,
        JxlDataType::FLOAT16 => 16,
        _ => 0,
    }
}
pub fn bytes_per_pixel(format: JxlPixelFormat) -> usize {
    (format.num_channels as usize).wrapping_mul(bits_per_channel(format.data_type)) / 8
}

#[derive(Default)]
pub struct FrameInputChannel<'a> {
    buffer: Option<&'a [u8]>,
    buffer_size: usize,
    format: Option<JxlPixelFormat>,
    xsize: usize,
    ysize: usize,
    bytes_per_pixel: usize,
    stride: usize,
    copy: Vec<u8>,
}
impl<'a> FrameInputChannel<'a> {
    fn set_format_and_dimensions(&mut self, format: JxlPixelFormat, xs: usize, ys: usize) -> bool {
        self.format = Some(format);
        self.xsize = xs;
        self.ysize = ys;
        self.bytes_per_pixel = bytes_per_pixel(format);
        let Some(last_row_size) = self.xsize.checked_mul(self.bytes_per_pixel) else {
            return false;
        };
        // SafeRoundUpTo writes its output even when SafeAdd reports overflow.
        if format.align < 2 {
            self.stride = last_row_size;
        } else {
            let reminder = last_row_size % format.align;
            if reminder == 0 {
                self.stride = last_row_size;
            } else {
                let (stride, overflow) = last_row_size.overflowing_add(format.align - reminder);
                self.stride = stride;
                if overflow {
                    return false;
                }
            }
        }
        self.ysize.checked_mul(self.stride).is_some()
    }
    fn set_from_buffer(
        &mut self,
        buffer: Option<&'a [u8]>,
        size: usize,
        format: JxlPixelFormat,
        xs: usize,
        ys: usize,
    ) -> bool {
        if !self.set_format_and_dimensions(format, xs, ys) {
            return false;
        }
        if self.ysize == 0 {
            return false;
        }
        self.buffer = buffer;
        self.buffer_size = size;
        let min_buffer_size = self
            .stride
            .wrapping_mul(self.ysize - 1)
            .wrapping_add(self.xsize.wrapping_mul(self.bytes_per_pixel));
        min_buffer_size <= size
    }
    fn copy_buffer(&mut self) {
        if let Some(buffer) = self.buffer {
            self.copy = buffer[..self.buffer_size].to_vec();
            self.buffer = None;
        }
    }
    fn get_data_at(
        &self,
        xpos: usize,
        ypos: usize,
        xs: usize,
        ys: usize,
        row_offset: &mut usize,
    ) -> Option<&[u8]> {
        let buffer = if self.copy.is_empty() {
            self.buffer
        } else {
            Some(self.copy.as_slice())
        };
        let inside = xpos <= self.xsize
            && xs <= self.xsize - xpos
            && ypos <= self.ysize
            && ys <= self.ysize - ypos;
        if !inside {
            return None;
        }
        let buffer = buffer?;
        *row_offset = self.stride;
        let offset = ypos
            .wrapping_mul(self.stride)
            .wrapping_add(xpos.wrapping_mul(self.bytes_per_pixel));
        Some(&buffer[offset..])
    }
    // Read-only observations of the source's private channel storage. Fields
    // without source initial values are meaningful only after their assignment.
    pub fn format(&self) -> JxlPixelFormat {
        self.format.expect("source format must be initialized")
    }
    pub fn xsize(&self) -> usize {
        self.xsize
    }
    pub fn ysize(&self) -> usize {
        self.ysize
    }
    pub fn bytes_per_pixel(&self) -> usize {
        self.bytes_per_pixel
    }
    pub fn stride(&self) -> usize {
        self.stride
    }
    pub fn buffer_size(&self) -> usize {
        self.buffer_size
    }
    pub fn borrowed_buffer(&self) -> Option<&[u8]> {
        self.buffer
    }
    pub fn copied_buffer(&self) -> &[u8] {
        &self.copy
    }
}

pub struct JxlEncoderChunkedFrameAdapter<'a> {
    xsize: usize,
    ysize: usize,
    channels: Vec<FrameInputChannel<'a>>,
}
impl<'a> JxlEncoderChunkedFrameAdapter<'a> {
    pub fn new(xs: usize, ys: usize, num_extra_channels: usize) -> Self {
        let count = 1usize.wrapping_add(num_extra_channels);
        let mut channels = Vec::with_capacity(count);
        channels.resize_with(count, FrameInputChannel::default);
        Self {
            xsize: xs,
            ysize: ys,
            channels,
        }
    }
    pub fn xsize(&self) -> usize {
        self.xsize
    }
    pub fn ysize(&self) -> usize {
        self.ysize
    }
    pub fn channels(&self) -> &[FrameInputChannel<'a>] {
        &self.channels
    }
    pub fn set_from_buffer(
        &mut self,
        channel: usize,
        buffer: Option<&'a [u8]>,
        size: usize,
        format: JxlPixelFormat,
    ) -> bool {
        if channel >= self.channels.len() {
            return false;
        }
        if !self.channels[channel].set_from_buffer(buffer, size, format, self.xsize, self.ysize) {
            return false;
        }
        if channel > 0 {
            self.channels[channel].copy_buffer();
        }
        true
    }
    pub fn copy_buffers(&mut self) -> bool {
        // No SetInputSource API exists yet, so only the original buffer branch
        // can be selected. Extra channel buffers were copied by SetFromBuffer.
        self.channels[0].copy_buffer();
        true
    }
    pub fn get_color_channels_pixel_format(&self) -> JxlPixelFormat {
        self.channels[0].format()
    }
    pub fn get_extra_channel_pixel_format(&self, ec_index: usize) -> JxlPixelFormat {
        self.channels[ec_index.wrapping_add(1)].format()
    }
    pub fn get_color_channel_data_at(
        &self,
        xpos: usize,
        ypos: usize,
        xs: usize,
        ys: usize,
        row_offset: &mut usize,
    ) -> Option<&[u8]> {
        self.channels[0].get_data_at(xpos, ypos, xs, ys, row_offset)
    }
    pub fn get_extra_channel_data_at(
        &self,
        ec_index: usize,
        xpos: usize,
        ypos: usize,
        xs: usize,
        ys: usize,
        row_offset: &mut usize,
    ) -> Option<&[u8]> {
        self.channels[ec_index.wrapping_add(1)].get_data_at(xpos, ypos, xs, ys, row_offset)
    }
}
