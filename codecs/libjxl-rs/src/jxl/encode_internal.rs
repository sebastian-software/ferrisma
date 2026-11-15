// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Encoder-internal owners from lib/jxl/encode_internal.h: BitsPerChannel,
//! BytesPerPixel, the buffer-backed input path of
//! JxlEncoderChunkedFrameAdapter, MakeBoxType, the frame-index-box storage,
//! JxlEncoderFrameSettingsValues, the queued frame/box/input owners, and
//! JxlEncoderOutputProcessorWrapper (buffer mode and the external output
//! processor: seek, finalized position, external buffers and
//! AppendBufferToExternalProcessor) with JxlOutputProcessorBuffer and
//! AppendData, whose out-of-line methods are in lib/jxl/encode.cc. The public
//! encoder object and its functions are in `encode.rs`.
//!
//! Language-required deviations, each local to its representation:
//! - Native borrows replace the buffer path's callback table and no-op
//!   release. The source's nested Channel is named FrameInputChannel. A queued
//!   frame owns its copies, so `into_detached` drops the borrow lifetime once
//!   CopyBuffers has released every caller buffer.
//! - Owners that later steps install are uninhabited `Option<Infallible>`
//!   fields at their source positions: the fast-lossless frame (until
//!   enc_fast_lossless) and the statistics `aux_out` (until
//!   JxlEncoderCollectStats). Their source branches are therefore statically
//!   unreachable and are written as matches on the uninhabited value.
//! - The external output processor is held by value; the source's
//!   `std::unique_ptr` allocation through the global allocator is not
//!   observable. Its callbacks are called through the C function pointers of
//!   encode.h, under the contract of
//!   [`JxlEncoderOutputProcessorWrapper::with_processor`].
//! - JxlOutputProcessorBuffer borrows its wrapper for its lifetime instead of
//!   retaining a raw wrapper pointer; its destructor releases as the source's.
//!   The source's move assignment is not reached and is absent.
//! - GetInputSource returns [`InputSource`]: the adapter's own channels (the
//!   source's method callbacks, whose release is a no-op) or the caller's
//!   callbacks of JxlEncoderAddChunkedFrame. GetColorBuffer and
//!   GetExtraChannelBuffer return a [`ScopedInputBuffer`] that calls the
//!   caller's release when dropped, as the source's unique_ptr deleter does
//!   for a nonnull buffer. A channel whose format was never set reports a
//!   zero format (the source's is indeterminate there; it is only read
//!   before a request that returns null).
//! - JPEG input and CopyOutput remain unported. WriteBoxHeader (declared
//!   here, defined in encode.cc), AppendBoxHeader and MakeContainerHeader
//!   write into byte slices and vectors (encoder container); the
//!   frame-index-box methods are in `encode.rs`.

use super::base::status::{Status, StatusCode, StatusOr};
use super::codestream_header::{JxlBlendInfo, JxlFrameHeader};
use super::common::JXL_SIGNATURE_BOX;
use super::enc_params::CompressParams;
use super::encode::{JxlChunkedFrameInputSource, JxlEncoderOutputProcessor};
use super::memory_manager::JxlMemoryManager;
use super::memory_manager_internal::{MemoryManagerPtr, MemoryManagerUniquePtr};
use super::padded_bytes::PaddedBytes;
use super::types::JxlBitDepth;
use super::types::{JxlDataType, JxlPixelFormat};
use std::collections::BTreeMap;
use std::convert::Infallible;
use std::ops::Bound;

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
    fn copy_from_buffer(
        &mut self,
        buffer: *const u8,
        format: JxlPixelFormat,
        xs: usize,
        ys: usize,
        row_offset: usize,
    ) -> bool {
        if !self.set_format_and_dimensions(format, xs, ys) {
            return false;
        }
        if self.stride > row_offset {
            return false;
        }
        self.buffer = None;
        // Safe: set_format_and_dimensions checked ys * stride.
        self.copy.resize(ys * self.stride, 0);
        for y in 0..ys {
            // SAFETY: The input source contract: row `y` of the returned
            // rectangle starts at `buffer + y * row_offset` and holds at
            // least `stride <= row_offset` bytes (a full row of `xs`
            // pixels; the format's alignment is ignored).
            unsafe {
                std::ptr::copy_nonoverlapping(
                    buffer.add(y * row_offset),
                    self.copy.as_mut_ptr().add(y * self.stride),
                    self.stride,
                );
            }
        }
        true
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
    fn format_or_zero(&self) -> JxlPixelFormat {
        self.format.unwrap_or(JxlPixelFormat {
            num_channels: 0,
            data_type: JxlDataType(0),
            endianness: super::types::JxlEndianness(0),
            align: 0,
        })
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
    input_source: Option<JxlChunkedFrameInputSource>,
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
            input_source: None,
            channels,
        }
    }
    /// SetInputSource.
    ///
    /// # Safety
    /// The callbacks and opaque value obey encode.h whenever a later call of
    /// this adapter or of the encoder reaches them, and neither unwind nor
    /// reenter the encoder. A returned rectangle stays valid until its
    /// release, its rows start `*row_offset` bytes apart and each holds the
    /// requested width in the reported pixel format.
    pub unsafe fn set_input_source(&mut self, input_source: JxlChunkedFrameInputSource) {
        self.input_source = Some(input_source);
    }
    pub fn streaming_input(&self) -> bool {
        self.input_source.is_some()
    }
    /// GetInputSource.
    pub fn get_input_source(&self) -> InputSource<'_, 'a> {
        match self.input_source {
            Some(source) => InputSource::Callbacks(source),
            None => InputSource::Adapter(self),
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
    /// SetFromBuffer for an extra channel of an adapter that retains no
    /// caller buffer, such as a queued frame's: the bound buffer is copied at
    /// once, as SetFromBuffer's CopyBuffer does for every channel above 0.
    /// When the size check fails, the source returns with the caller pointer
    /// retained in the channel and never copied; the Rust owner stores no
    /// caller pointer then, and its other fields match the source's.
    pub fn set_extra_from_buffer(
        &mut self,
        channel: usize,
        buffer: Option<&[u8]>,
        size: usize,
        format: JxlPixelFormat,
    ) -> bool {
        assert!(channel > 0, "only extra channels are copied at once");
        if channel >= self.channels.len() {
            return false;
        }
        let (xsize, ysize) = (self.xsize, self.ysize);
        let target = &mut self.channels[channel];
        let mut staged = FrameInputChannel {
            buffer: None,
            buffer_size: target.buffer_size,
            format: target.format,
            xsize: target.xsize,
            ysize: target.ysize,
            bytes_per_pixel: target.bytes_per_pixel,
            stride: target.stride,
            copy: std::mem::take(&mut target.copy),
        };
        let accepted = staged.set_from_buffer(buffer, size, format, xsize, ysize);
        if accepted {
            staged.copy_buffer();
        }
        *target = FrameInputChannel {
            buffer: None,
            buffer_size: staged.buffer_size,
            format: staged.format,
            xsize: staged.xsize,
            ysize: staged.ysize,
            bytes_per_pixel: staged.bytes_per_pixel,
            stride: staged.stride,
            copy: staged.copy,
        };
        accepted
    }
    pub fn copy_buffers(&mut self) -> bool {
        if let Some(source) = self.input_source {
            let input = InputSource::Callbacks(source);
            let mut format = JxlPixelFormat {
                num_channels: 4,
                data_type: JxlDataType::UINT8,
                endianness: super::types::JxlEndianness::NATIVE,
                align: 0,
            };
            input.get_color_channels_pixel_format(&mut format);
            format.align = 0; // .align must be ignored
            let mut row_offset = 0;
            {
                let buffer = input.get_color_buffer(0, 0, self.xsize, self.ysize, &mut row_offset);
                if buffer.is_null() {
                    return false;
                }
                if !self.channels[0].copy_from_buffer(
                    buffer.as_ptr(),
                    format,
                    self.xsize,
                    self.ysize,
                    row_offset,
                ) {
                    return false;
                }
            }
            let mut ec = 0;
            while ec + 1 < self.channels.len() {
                input.get_extra_channel_pixel_format(ec, &mut format);
                format.align = 0; // .align must be ignored
                let buffer = input.get_extra_channel_buffer(
                    ec,
                    0,
                    0,
                    self.xsize,
                    self.ysize,
                    &mut row_offset,
                );
                if buffer.is_null() {
                    ec += 1;
                    continue;
                }
                if !self.channels[1 + ec].copy_from_buffer(
                    buffer.as_ptr(),
                    format,
                    self.xsize,
                    self.ysize,
                    row_offset,
                ) {
                    return false;
                }
                ec += 1;
            }
            self.input_source = None;
        } else {
            self.channels[0].copy_buffer();
        }
        true
    }
    /// The owner a queued frame retains after CopyBuffers. Every channel then
    /// holds no caller buffer: channel 0 was copied by CopyBuffers and extra
    /// channels by SetFromBuffer, so only the borrow lifetime changes.
    pub fn into_detached(self) -> JxlEncoderChunkedFrameAdapter<'static> {
        JxlEncoderChunkedFrameAdapter {
            xsize: self.xsize,
            ysize: self.ysize,
            input_source: self.input_source,
            channels: self
                .channels
                .into_iter()
                .map(|channel| {
                    assert!(
                        channel.buffer.is_none(),
                        "a queued frame retains no caller buffer"
                    );
                    FrameInputChannel {
                        buffer: None,
                        buffer_size: channel.buffer_size,
                        format: channel.format,
                        xsize: channel.xsize,
                        ysize: channel.ysize,
                        bytes_per_pixel: channel.bytes_per_pixel,
                        stride: channel.stride,
                        copy: channel.copy,
                    }
                })
                .collect(),
        }
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
        if ec_index >= self.channels.len() - 1 {
            return None;
        }
        self.channels[ec_index.wrapping_add(1)].get_data_at(xpos, ypos, xs, ys, row_offset)
    }
}

/// The JxlChunkedFrameInputSource that GetInputSource returns.
#[derive(Clone, Copy)]
pub enum InputSource<'s, 'a> {
    /// The adapter's own channels: GetColorChannelsPixelFormat,
    /// GetColorChannelDataAt, GetExtraChannelPixelFormat,
    /// GetExtraChannelDataAt and the no-op ReleaseCurrentData.
    Adapter(&'s JxlEncoderChunkedFrameAdapter<'a>),
    /// The caller's callbacks of JxlEncoderAddChunkedFrame.
    Callbacks(JxlChunkedFrameInputSource),
}

/// ScopedInputBuffer: a rectangle from an input source, released through the
/// source's `release_buffer` when dropped if it is nonnull.
pub struct ScopedInputBuffer<'s> {
    data: *const u8,
    /// The bytes after `data` for the adapter's own buffers.
    own: Option<&'s [u8]>,
    release: Option<JxlChunkedFrameInputSource>,
}
impl ScopedInputBuffer<'_> {
    pub fn is_null(&self) -> bool {
        self.data.is_null()
    }
    pub fn as_ptr(&self) -> *const u8 {
        self.data
    }
    /// The rectangle's bytes for `ysize` rows of `row_bytes` bytes,
    /// `row_offset` apart: the adapter's whole remaining buffer, or the
    /// caller's rows under the input source contract.
    pub fn bytes(&self, ysize: usize, row_offset: usize, row_bytes: usize) -> &[u8] {
        if let Some(own) = self.own {
            return own;
        }
        let len = if ysize == 0 {
            0
        } else {
            (ysize - 1) * row_offset + row_bytes
        };
        // SAFETY: set_input_source's contract: a nonnull rectangle stays
        // valid until its release, which happens when `self` drops, and
        // holds `ysize` rows of `row_bytes` bytes `row_offset` apart.
        unsafe { std::slice::from_raw_parts(self.data, len) }
    }
}
impl Drop for ScopedInputBuffer<'_> {
    fn drop(&mut self) {
        if let Some(source) = self.release
            && !self.data.is_null()
        {
            // SAFETY: set_input_source's contract.
            unsafe {
                source.release_buffer.expect("required input callback")(
                    source.opaque,
                    self.data.cast(),
                );
            }
        }
    }
}

impl<'s> InputSource<'s, '_> {
    pub fn get_color_channels_pixel_format(&self, pixel_format: &mut JxlPixelFormat) {
        match self {
            Self::Adapter(adapter) => *pixel_format = adapter.channels[0].format_or_zero(),
            // SAFETY: set_input_source's contract.
            Self::Callbacks(source) => unsafe {
                source
                    .get_color_channels_pixel_format
                    .expect("required input callback")(source.opaque, pixel_format);
            },
        }
    }
    pub fn get_extra_channel_pixel_format(
        &self,
        ec_index: usize,
        pixel_format: &mut JxlPixelFormat,
    ) {
        match self {
            Self::Adapter(adapter) => {
                *pixel_format = adapter.channels[1 + ec_index].format_or_zero();
            }
            // SAFETY: set_input_source's contract.
            Self::Callbacks(source) => unsafe {
                source
                    .get_extra_channel_pixel_format
                    .expect("required input callback")(
                    source.opaque, ec_index, pixel_format
                );
            },
        }
    }
    /// Whether the callbacks the alpha check of ProcessOneEnqueuedInput
    /// tests are present: color format, color data and release; and the
    /// extra-channel format and data.
    pub fn has_color_callbacks(&self) -> bool {
        match self {
            Self::Adapter(_) => true,
            Self::Callbacks(source) => {
                source.get_color_channels_pixel_format.is_some()
                    && source.get_color_channel_data_at.is_some()
                    && source.release_buffer.is_some()
            }
        }
    }
    pub fn has_extra_channel_callbacks(&self) -> bool {
        match self {
            Self::Adapter(_) => true,
            Self::Callbacks(source) => {
                source.get_extra_channel_pixel_format.is_some()
                    && source.get_extra_channel_data_at.is_some()
            }
        }
    }
    /// GetColorBuffer.
    pub fn get_color_buffer(
        &self,
        xpos: usize,
        ypos: usize,
        xsize: usize,
        ysize: usize,
        row_offset: &mut usize,
    ) -> ScopedInputBuffer<'s> {
        match *self {
            Self::Adapter(adapter) => {
                let own = adapter.get_color_channel_data_at(xpos, ypos, xsize, ysize, row_offset);
                ScopedInputBuffer {
                    data: own.map_or(std::ptr::null(), <[u8]>::as_ptr),
                    own,
                    release: None,
                }
            }
            Self::Callbacks(source) => ScopedInputBuffer {
                // SAFETY: set_input_source's contract.
                data: unsafe {
                    source
                        .get_color_channel_data_at
                        .expect("required input callback")(
                        source.opaque,
                        xpos,
                        ypos,
                        xsize,
                        ysize,
                        row_offset,
                    )
                }
                .cast(),
                own: None,
                release: Some(source),
            },
        }
    }
    /// GetExtraChannelBuffer.
    pub fn get_extra_channel_buffer(
        &self,
        ec_index: usize,
        xpos: usize,
        ypos: usize,
        xsize: usize,
        ysize: usize,
        row_offset: &mut usize,
    ) -> ScopedInputBuffer<'s> {
        match *self {
            Self::Adapter(adapter) => {
                let own = adapter
                    .get_extra_channel_data_at(ec_index, xpos, ypos, xsize, ysize, row_offset);
                ScopedInputBuffer {
                    data: own.map_or(std::ptr::null(), <[u8]>::as_ptr),
                    own,
                    release: None,
                }
            }
            Self::Callbacks(source) => ScopedInputBuffer {
                // SAFETY: set_input_source's contract.
                data: unsafe {
                    source
                        .get_extra_channel_data_at
                        .expect("required input callback")(
                        source.opaque,
                        ec_index,
                        xpos,
                        ypos,
                        xsize,
                        ysize,
                        row_offset,
                    )
                }
                .cast(),
                own: None,
                release: Some(source),
            },
        }
    }
}

/// BoxType and MakeBoxType.
pub type BoxType = [u8; 4];
pub fn make_box_type(box_type: &[u8; 4]) -> BoxType {
    *box_type
}

#[derive(Clone, Copy, Debug, Default)]
pub struct JxlEncoderFrameIndexBoxEntry {
    pub to_be_indexed: bool,
    pub duration: u32,
    pub offi: u64,
}

/// JxlEncoderFrameIndexBox storage with its default member initializers.
#[derive(Clone, Debug)]
pub struct JxlEncoderFrameIndexBox {
    pub index_box_requested_through_api: bool,
    pub tnum: i32,
    pub tden: i32,
    pub entries: Vec<JxlEncoderFrameIndexBoxEntry>,
}
impl Default for JxlEncoderFrameIndexBox {
    fn default() -> Self {
        Self {
            index_box_requested_through_api: false,
            tnum: 1,
            tden: 1000,
            entries: Vec::new(),
        }
    }
}

/// The encoder options of one frame, in source field order.
#[derive(Clone)]
pub struct JxlEncoderFrameSettingsValues {
    pub lossless: bool,
    pub cparams: CompressParams<'static>,
    pub header: JxlFrameHeader,
    pub extra_channel_blend_info: Vec<JxlBlendInfo>,
    pub frame_name: Vec<u8>,
    pub image_bit_depth: JxlBitDepth,
    pub frame_index_box: bool,
    /// Set only by JxlEncoderCollectStats, which is not yet ported.
    pub aux_out: Option<Infallible>,
    pub strip_alpha: i32,
}
impl Default for JxlEncoderFrameSettingsValues {
    /// The value-initialized source object: zero scalars, a zeroed frame
    /// header and bit depth, then the default member initializers.
    fn default() -> Self {
        Self {
            lossless: false,
            cparams: CompressParams::default(),
            header: JxlFrameHeader::default(),
            extra_channel_blend_info: Vec::new(),
            frame_name: Vec::new(),
            image_bit_depth: JxlBitDepth::default(),
            frame_index_box: false,
            aux_out: None,
            strip_alpha: -1,
        }
    }
}

pub struct JxlEncoderQueuedFrame {
    pub option_values: JxlEncoderFrameSettingsValues,
    pub frame_data: JxlEncoderChunkedFrameAdapter<'static>,
    pub ec_initialized: Vec<u8>,
}

pub struct JxlEncoderQueuedBox {
    pub box_type: BoxType,
    pub contents: Vec<u8>,
    pub compress_box: bool,
}

/// Either a frame or a box, not both. A fast-lossless frame cannot be queued
/// until enc_fast_lossless is ported.
pub struct JxlEncoderQueuedInput {
    pub(crate) frame: Option<MemoryManagerUniquePtr<JxlEncoderQueuedFrame>>,
    pub(crate) queued_box: Option<MemoryManagerUniquePtr<JxlEncoderQueuedBox>>,
    pub fast_lossless_frame: Option<Infallible>,
    pub output_mode: i32,
}
impl JxlEncoderQueuedInput {
    /// The source constructor: both owners are null and keep the encoder's
    /// manager in their deleters, which here travels with each owner.
    pub(crate) fn new() -> Self {
        Self {
            frame: None,
            queued_box: None,
            fast_lossless_frame: None,
            output_mode: -1,
        }
    }
    /// Read-only observations of the queued owners.
    pub fn frame(&self) -> Option<&JxlEncoderQueuedFrame> {
        self.frame.as_deref()
    }
    pub fn frame_address(&self) -> Option<*const JxlEncoderQueuedFrame> {
        self.frame.as_ref().map(|frame| frame.as_ptr().cast_const())
    }
    pub fn queued_box(&self) -> Option<&JxlEncoderQueuedBox> {
        self.queued_box.as_deref()
    }
}

pub const SMALL_BOX_HEADER_SIZE: usize = 8;
pub const LARGE_BOX_HEADER_SIZE: usize = 16;
pub const LARGE_BOX_CONTENT_SIZE_THRESHOLD: usize = 0x1_0000_0000 - SMALL_BOX_HEADER_SIZE;

// encoder container
/// WriteBoxHeader (defined in encode.cc): writes the header of a box with
/// `size` content bytes into `output` and returns its length. As in the
/// source, the large-header test reads `box_size`, which is still 0 there, so
/// only `force_large_box` selects the 64-bit size field; a larger `size`
/// without it keeps the low 32 bits of the box size.
#[allow(
    clippy::absurd_extreme_comparisons,
    reason = "Keep the source's test of the not yet assigned box_size."
)]
#[allow(clippy::needless_range_loop, reason = "Keep the source index loops.")]
pub fn write_box_header(
    box_type: &BoxType,
    size: usize,
    unbounded: bool,
    force_large_box: bool,
    output: &mut [u8],
) -> usize {
    let mut box_size: u64 = 0;
    let mut large_size = false;
    if !unbounded {
        if box_size >= LARGE_BOX_CONTENT_SIZE_THRESHOLD as u64 || force_large_box {
            large_size = true;
            // TODO(firsching): send a separate CL for this (+ test),
            // quick fix in the old code: box_size += 8
            box_size = size.wrapping_add(LARGE_BOX_HEADER_SIZE) as u64;
        } else {
            box_size = size.wrapping_add(SMALL_BOX_HEADER_SIZE) as u64;
        }
    }

    let mut idx = 0;
    {
        let store: u64 = if large_size { 1 } else { box_size };
        for i in 0..4 {
            output[idx] = ((store >> (8 * (3 - i))) & 0xff) as u8;
            idx += 1;
        }
    }
    for i in 0..4 {
        output[idx] = box_type[i];
        idx += 1;
    }

    if large_size {
        for i in 0..8 {
            output[idx] = ((box_size >> (8 * (7 - i))) & 0xff) as u8;
            idx += 1;
        }
    }
    idx
}

/// AppendBoxHeader: appends a JXL container box header with given type, size,
/// and unbounded properties to output.
pub fn append_box_header(box_type: &BoxType, size: usize, unbounded: bool, output: &mut Vec<u8>) {
    let current_size = output.len();
    output.resize(current_size + LARGE_BOX_HEADER_SIZE, 0);
    let header_size = write_box_header(
        box_type,
        size,
        unbounded,
        /*force_large_box=*/ false,
        &mut output[current_size..],
    );
    output.truncate(current_size + header_size);
}

/// MakeContainerHeader: the JXL container signature box and ftyp box.
/// ftyp_version: 0 = standard delivery order, 1 = out-of-order jxlp boxes.
pub fn make_container_header(ftyp_version: i32) -> Vec<u8> {
    let mut out = JXL_SIGNATURE_BOX.to_vec();
    // ftyp box: major brand "jxl ", minor version, compatible brand "jxl ".
    let ftyp: [u8; 12] = [
        b'j',
        b'x',
        b'l',
        b' ',
        0,
        0,
        0,
        ftyp_version as u8,
        b'j',
        b'x',
        b'l',
        b' ',
    ];
    append_box_header(
        &make_box_type(b"ftyp"),
        ftyp.len(),
        /*unbounded=*/ false,
        &mut out,
    );
    out.extend_from_slice(&ftyp);
    out
}
// end encoder container

struct InternalBuffer {
    // Bytes in the range `[output_position - start_of_the_buffer,
    // written_bytes)` need to be flushed out.
    written_bytes: usize,
    // If data has been buffered, it is stored in `owned_data`.
    owned_data: PaddedBytes,
}
impl InternalBuffer {
    fn new(memory_manager: *const JxlMemoryManager) -> Self {
        debug_assert!(!memory_manager.is_null());
        Self {
            written_bytes: 0,
            // SAFETY: The wrapper's manager is the encoder's stable manager
            // cell (or a caller manager under set_avail_out's contract), which
            // outlives every buffer the wrapper owns.
            owned_data: PaddedBytes::new(unsafe { MemoryManagerPtr::from_raw(memory_manager) }),
        }
    }
}

/// JxlEncoderOutputProcessorWrapper, with fields in source order.
pub struct JxlEncoderOutputProcessorWrapper {
    // Invariant: `internal_buffers` does not contain chunks that are entirely
    // below the output position.
    internal_buffers: BTreeMap<usize, InternalBuffer>,
    next_out: *mut *mut u8,
    avail_out: *mut usize,
    // Where the next GetBuffer call will write bytes to.
    position: usize,
    // The position of the last SetFinalizedPosition call.
    finalized_position: usize,
    // Either the position of the external processor or the position
    // `next_out` points to.
    output_position: usize,
    stop_requested: bool,
    has_buffer: bool,
    memory_manager: *const JxlMemoryManager,
    external_output_processor: Option<JxlEncoderOutputProcessor>,
}

macro_rules! ensure {
    ($condition:expr) => {
        if !$condition {
            return Err(StatusCode::GenericError);
        }
    };
}

impl JxlEncoderOutputProcessorWrapper {
    /// The buffer-mode constructor.
    ///
    /// # Safety
    /// A nonnull `memory_manager` stays readable, initialized and valid for
    /// memory_manager.h allocations until the wrapper and every buffer it owns
    /// are dropped.
    pub unsafe fn new(memory_manager: *const JxlMemoryManager) -> Self {
        Self {
            internal_buffers: BTreeMap::new(),
            next_out: std::ptr::null_mut(),
            avail_out: std::ptr::null_mut(),
            position: 0,
            finalized_position: 0,
            output_position: 0,
            stop_requested: false,
            has_buffer: false,
            memory_manager,
            external_output_processor: None,
        }
    }
    /// The constructor with an external output processor.
    ///
    /// # Safety
    /// As for [`Self::new`]; in addition, the processor's callbacks and
    /// opaque value obey encode.h whenever a later wrapper call reaches them,
    /// and neither unwind nor reenter the encoder. A buffer returned by
    /// `get_buffer` holds the returned size of writable bytes until the
    /// matching `release_buffer` call and does not overlap any buffer the
    /// wrapper reads.
    pub unsafe fn with_processor(
        memory_manager: *const JxlMemoryManager,
        processor: JxlEncoderOutputProcessor,
    ) -> Self {
        // SAFETY: The caller supplies the manager contract.
        let mut wrapper = unsafe { Self::new(memory_manager) };
        wrapper.external_output_processor = Some(processor);
        wrapper
    }
    pub fn has_avail_out(&self) -> bool {
        !self.avail_out.is_null()
    }
    pub fn current_position(&self) -> usize {
        self.position
    }
    pub fn was_stop_requested(&self) -> bool {
        self.stop_requested
    }
    pub fn output_processor_set(&self) -> bool {
        self.external_output_processor.is_some()
    }
    pub fn has_output_to_write(&self) -> bool {
        self.output_position < self.finalized_position
    }

    // Caller can never overwrite a previously-written buffer. Asking for a
    // buffer with `min_size` such that `position + min_size` overlaps with a
    // previously-written buffer is invalid.
    pub fn get_buffer(
        &mut self,
        min_size: usize,
        requested_size: usize,
    ) -> StatusOr<JxlOutputProcessorBuffer<'_>> {
        ensure!(min_size > 0);
        ensure!(!self.has_buffer);
        if self.stop_requested {
            return Err(StatusCode::NotEnoughBytes);
        }
        let requested_size = min_size.max(requested_size);

        // If we support seeking, output_position == position.
        if let Some(processor) = &self.external_output_processor
            && processor.seek.is_some()
        {
            ensure!(self.output_position == self.position);
        }
        // Otherwise, output_position <= position.
        ensure!(self.output_position <= self.position);
        let additional_size = self.position - self.output_position;
        ensure!(!self.memory_manager.is_null());

        if let Some(processor) = self.external_output_processor {
            // TODO(veluca): here, we cannot just ask for a larger buffer, as it
            // will be released with a prefix of the buffer that has not been
            // written yet. Figure out if there is a good way to do this more
            // efficiently.
            if additional_size == 0 {
                let mut size = requested_size;
                // SAFETY: with_processor's caller supplies the callback
                // contract; `size` is a valid local.
                let user_buffer = unsafe {
                    processor.get_buffer.expect("checked by SetOutputProcessor")(
                        processor.opaque,
                        &mut size,
                    )
                }
                .cast::<u8>();
                if size == 0 || user_buffer.is_null() {
                    self.stop_requested = true;
                    return Err(StatusCode::NotEnoughBytes);
                }
                if size < min_size {
                    // SAFETY: As above.
                    unsafe {
                        processor
                            .release_buffer
                            .expect("checked by SetOutputProcessor")(
                            processor.opaque, 0
                        );
                    }
                } else {
                    let memory_manager = self.memory_manager;
                    self.internal_buffers
                        .entry(self.position)
                        .or_insert_with(|| InternalBuffer::new(memory_manager));
                    self.has_buffer = true;
                    return Ok(JxlOutputProcessorBuffer::new(user_buffer, size, 0, self));
                }
            }
        } else if !self.avail_out.is_null() {
            // SAFETY: set_avail_out's caller keeps both caller pointers valid
            // while the wrapper writes through them.
            let avail_out = unsafe { *self.avail_out };
            if min_size.wrapping_add(additional_size) < avail_out {
                let memory_manager = self.memory_manager;
                self.internal_buffers
                    .entry(self.position)
                    .or_insert_with(|| InternalBuffer::new(memory_manager));
                self.has_buffer = true;
                // SAFETY: As above; the offset is within the caller's
                // `avail_out` writable bytes because it is smaller.
                let data = unsafe { (*self.next_out).add(additional_size) };
                return Ok(JxlOutputProcessorBuffer::new(
                    data,
                    avail_out - additional_size,
                    0,
                    self,
                ));
            }
        }
        // Otherwise, we need to allocate our own buffer.
        let memory_manager = self.memory_manager;
        let position = self.position;
        let next = self
            .internal_buffers
            .range((Bound::Excluded(position), Bound::Unbounded))
            .next()
            .map(|(&next, _)| next);
        let buffer = self
            .internal_buffers
            .entry(position)
            .or_insert_with(|| InternalBuffer::new(memory_manager));
        let mut alloc_size = requested_size;
        if let Some(next) = next {
            alloc_size = alloc_size.min(next - position);
            ensure!(alloc_size >= min_size);
        }
        buffer.owned_data.resize(alloc_size)?;
        let data = buffer.owned_data.bytes_mut().as_mut_ptr();
        self.has_buffer = true;
        Ok(JxlOutputProcessorBuffer::new(data, alloc_size, 0, self))
    }

    pub fn seek(&mut self, pos: usize) -> Status {
        ensure!(!self.has_buffer);
        if let Some(processor) = self.external_output_processor
            && let Some(seek) = processor.seek
        {
            // SAFETY: with_processor's caller supplies the callback contract.
            unsafe { seek(processor.opaque, pos as u64) };
            self.output_position = pos;
        }
        ensure!(pos >= self.finalized_position);
        self.position = pos;
        Ok(())
    }

    pub fn set_finalized_position(&mut self) -> Status {
        ensure!(!self.has_buffer);
        if let Some(processor) = self.external_output_processor
            && processor.seek.is_some()
        {
            // SAFETY: with_processor's caller supplies the callback contract.
            unsafe {
                processor
                    .set_finalized_position
                    .expect("checked by SetOutputProcessor")(
                    processor.opaque,
                    self.position as u64,
                );
            }
        }
        self.finalized_position = self.position;
        // SAFETY: The retained caller pointers are null or valid under
        // set_avail_out's contract.
        unsafe { self.flush_output(self.next_out, self.avail_out) }
    }

    /// SetAvailOut. The wrapper retains both pointers, as the source does.
    ///
    /// # Safety
    /// Both pointers are null or valid, and `*next_out` points to `*avail_out`
    /// writable bytes, whenever a later wrapper call writes output, until they
    /// are replaced by another call; no Rust reference to them is live then.
    pub unsafe fn set_avail_out(
        &mut self,
        next_out: *mut *mut u8,
        avail_out: *mut usize,
    ) -> Status {
        ensure!(self.external_output_processor.is_none());
        self.avail_out = avail_out;
        self.next_out = next_out;
        // SAFETY: The caller supplies the pointer contract.
        unsafe { self.flush_output(self.next_out, self.avail_out) }
    }

    fn release_buffer(&mut self, bytes_used: usize) -> Status {
        ensure!(self.has_buffer);
        self.has_buffer = false;
        let position = self.position;
        ensure!(self.internal_buffers.contains_key(&position));
        if bytes_used == 0 {
            if let Some(processor) = self.external_output_processor {
                // SAFETY: with_processor's caller supplies the callback
                // contract.
                unsafe {
                    processor
                        .release_buffer
                        .expect("checked by SetOutputProcessor")(
                        processor.opaque, bytes_used
                    );
                }
            }
            self.internal_buffers.remove(&position);
            return Ok(());
        }
        self.internal_buffers
            .get_mut(&position)
            .expect("checked above")
            .written_bytes = bytes_used;
        self.position = self.position.wrapping_add(bytes_used);
        if let Some((&next, _)) = self
            .internal_buffers
            .range((Bound::Excluded(position), Bound::Unbounded))
            .next()
        {
            ensure!(next >= self.position);
        }

        if let Some(processor) = self.external_output_processor {
            let owned_empty = self.internal_buffers[&position].owned_data.is_empty();
            // If the buffer was given by the user, tell the user it is not
            // needed anymore.
            if owned_empty {
                // SAFETY: with_processor's caller supplies the callback
                // contract.
                unsafe {
                    processor
                        .release_buffer
                        .expect("checked by SetOutputProcessor")(
                        processor.opaque, bytes_used
                    );
                }
                // If we don't support seeking, this implies we will never
                // modify again the bytes that were written so far. Advance the
                // finalized position and flush the output to clean up the
                // internal buffers.
                if processor.seek.is_none() {
                    self.set_finalized_position()?;
                    ensure!(self.output_position == self.finalized_position);
                    ensure!(self.output_position == self.position);
                } else {
                    // Otherwise, advance the output position accordingly.
                    self.output_position = self.output_position.wrapping_add(bytes_used);
                    ensure!(self.output_position >= self.finalized_position);
                    ensure!(self.output_position == self.position);
                }
            } else if let Some(seek) = processor.seek {
                // If we had buffered the data internally, flush it out to the
                // external processor if we can.
                // SAFETY: As above.
                unsafe { seek(processor.opaque, (self.position - bytes_used) as u64) };
                self.output_position = self.position - bytes_used;
                while self.output_position < self.position {
                    let num_to_write = self.position - self.output_position;
                    // The source's size_t sum `output_position - position + bytes_used`.
                    let offset = self.output_position + bytes_used - self.position;
                    let data = self.internal_buffers[&position].owned_data.as_slice()
                        [offset..offset + num_to_write]
                        .as_ptr();
                    // SAFETY: `data` addresses `num_to_write` initialized
                    // bytes of this internal buffer, which the call below
                    // neither moves nor frees.
                    if !unsafe { self.append_buffer_to_external_processor(data, num_to_write) } {
                        return Ok(());
                    }
                }
                self.internal_buffers
                    .get_mut(&position)
                    .expect("checked above")
                    .owned_data
                    .clear();
            }
        }
        Ok(())
    }

    // Tries to write all the bytes up to the finalized position.
    unsafe fn flush_output(&mut self, next_out: *mut *mut u8, avail_out: *mut usize) -> Status {
        ensure!(!self.has_buffer);
        if self.external_output_processor.is_none() && avail_out.is_null() {
            return Ok(());
        }
        // SAFETY: Nonnull caller pointers are valid under set_avail_out's
        // contract for the whole loop.
        while self.output_position < self.finalized_position
            && (avail_out.is_null() || unsafe { *avail_out } > 0)
        {
            let Some((&start, buffer)) = self.internal_buffers.iter().next() else {
                return Err(StatusCode::GenericError);
            };
            // If this fails, we are trying to move the finalized position past
            // data that was not written yet.
            ensure!(self.output_position >= start);
            ensure!(buffer.written_bytes != 0);
            let buffer_last_byte = start.wrapping_add(buffer.written_bytes);
            if !buffer.owned_data.is_empty() {
                let start_in_buffer = self.output_position - start;
                // Guaranteed by the invariant on `internal_buffers`.
                ensure!(buffer_last_byte > self.output_position);
                let num_to_write =
                    buffer_last_byte.min(self.finalized_position) - self.output_position;
                if !avail_out.is_null() {
                    // SAFETY: As above; `n` is at most the caller's available
                    // bytes and the buffered bytes after `start_in_buffer`.
                    unsafe {
                        let n = num_to_write.min(*avail_out);
                        let source = &buffer.owned_data.as_slice()[start_in_buffer..][..n];
                        std::ptr::copy_nonoverlapping(source.as_ptr(), *next_out, n);
                        *avail_out -= n;
                        *next_out = (*next_out).add(n);
                        self.output_position += n;
                    }
                } else {
                    ensure!(self.external_output_processor.is_some());
                    let data =
                        buffer.owned_data.as_slice()[start_in_buffer..][..num_to_write].as_ptr();
                    // SAFETY: `data` addresses `num_to_write` initialized
                    // bytes of the first internal buffer, which the call below
                    // neither moves nor frees.
                    if !unsafe { self.append_buffer_to_external_processor(data, num_to_write) } {
                        return Ok(());
                    }
                }
            } else {
                // size_t arithmetic: a seek of an external processor can
                // leave the output position past this buffer.
                let advance = buffer_last_byte
                    .min(self.finalized_position)
                    .wrapping_sub(self.output_position);
                self.output_position = self.output_position.wrapping_add(advance);
                if !avail_out.is_null() {
                    // SAFETY: As above; the bytes were written in place into
                    // the caller's buffer by a direct GetBuffer.
                    unsafe {
                        *next_out = (*next_out).add(advance);
                        *avail_out -= advance;
                    }
                }
            }
            if buffer_last_byte == self.output_position {
                self.internal_buffers.remove(&start);
            }
            if let Some(processor) = self.external_output_processor
                && processor.seek.is_none()
            {
                // SAFETY: with_processor's caller supplies the callback
                // contract.
                unsafe {
                    processor
                        .set_finalized_position
                        .expect("checked by SetOutputProcessor")(
                        processor.opaque,
                        self.output_position as u64,
                    );
                }
            }
        }
        Ok(())
    }

    /// AppendBufferToExternalProcessor.
    ///
    /// # Safety
    /// `data` addresses `count` initialized bytes that stay valid during the
    /// call and do not overlap the processor's buffer.
    unsafe fn append_buffer_to_external_processor(
        &mut self,
        data: *const u8,
        count: usize,
    ) -> bool {
        let processor = self
            .external_output_processor
            .expect("only called with an external processor");
        let mut n = count;
        // SAFETY: with_processor's caller supplies the callback contract; `n`
        // is a valid local.
        let user_buffer = unsafe {
            processor.get_buffer.expect("checked by SetOutputProcessor")(processor.opaque, &mut n)
        };
        if user_buffer.is_null() || n == 0 {
            self.stop_requested = true;
            return false;
        }
        n = n.min(count);
        // SAFETY: The processor's buffer holds at least the returned `n`
        // writable bytes; the caller supplies `count >= n` readable bytes.
        unsafe {
            std::ptr::copy_nonoverlapping(data, user_buffer.cast::<u8>(), n);
            processor
                .release_buffer
                .expect("checked by SetOutputProcessor")(processor.opaque, n);
        }
        self.output_position = self.output_position.wrapping_add(n);
        true
    }
}

/// JxlOutputProcessorBuffer: a writable window handed out by GetBuffer.
pub struct JxlOutputProcessorBuffer<'w> {
    data: *mut u8,
    size: usize,
    bytes_used: usize,
    wrapper: &'w mut JxlEncoderOutputProcessorWrapper,
}
impl<'w> JxlOutputProcessorBuffer<'w> {
    fn new(
        data: *mut u8,
        size: usize,
        bytes_used: usize,
        wrapper: &'w mut JxlEncoderOutputProcessorWrapper,
    ) -> Self {
        Self {
            data,
            size,
            bytes_used,
            wrapper,
        }
    }
    pub fn size(&self) -> usize {
        self.size
    }
    pub fn advance(&mut self, count: usize) -> Status {
        ensure!(count <= self.size);
        // SAFETY: The window holds at least `size` writable bytes.
        self.data = unsafe { self.data.add(count) };
        self.size -= count;
        self.bytes_used += count;
        Ok(())
    }
    pub fn release(&mut self) -> Status {
        let mut result = Ok(());
        if !self.data.is_null() {
            result = self.wrapper.release_buffer(self.bytes_used);
        }
        self.data = std::ptr::null_mut();
        self.size = 0;
        result
    }
    /// The source copies before its bounds check; a longer copy would
    /// overflow the window, so it is rejected before any byte is written.
    pub fn append(&mut self, data: &[u8]) -> Status {
        assert!(
            data.len() <= self.size,
            "the source copy would overflow the output window"
        );
        // SAFETY: The window holds at least `size` writable bytes, which do
        // not overlap the borrowed input.
        unsafe { std::ptr::copy_nonoverlapping(data.as_ptr(), self.data, data.len()) };
        self.advance(data.len())
    }
}
impl Drop for JxlOutputProcessorBuffer<'_> {
    fn drop(&mut self) {
        let result = self.release();
        debug_assert!(result.is_ok());
    }
}

/// AppendData.
pub fn append_data(output_processor: &mut JxlEncoderOutputProcessorWrapper, data: &[u8]) -> Status {
    let size = data.len();
    let mut written = 0;
    while written < size {
        let mut buffer = output_processor.get_buffer(1, size - written)?;
        let n = (size - written).min(buffer.size());
        buffer.append(&data[written..written + n])?;
        written += n;
    }
    Ok(())
}
