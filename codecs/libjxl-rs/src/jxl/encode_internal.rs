// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Encoder-internal owners from lib/jxl/encode_internal.h: BitsPerChannel,
//! BytesPerPixel, the buffer-backed input path of
//! JxlEncoderChunkedFrameAdapter, MakeBoxType, the frame-index-box storage,
//! JxlEncoderFrameSettingsValues, the queued frame/box/input owners, and the
//! buffer mode of JxlEncoderOutputProcessorWrapper with
//! JxlOutputProcessorBuffer and AppendData, whose out-of-line methods are in
//! lib/jxl/encode.cc. The public encoder object and its functions are in
//! `encode.rs`.
//!
//! Language-required deviations, each local to its representation:
//! - Native borrows replace the buffer path's callback table and no-op
//!   release. The source's nested Channel is named FrameInputChannel. A queued
//!   frame owns its copies, so `into_detached` drops the borrow lifetime once
//!   CopyBuffers has released every caller buffer.
//! - Owners that later steps install are uninhabited `Option<Infallible>`
//!   fields at their source positions: the external output processor (until
//!   JxlEncoderSetOutputProcessor), the fast-lossless frame (until
//!   enc_fast_lossless) and the statistics `aux_out` (until
//!   JxlEncoderCollectStats). Their source branches are therefore statically
//!   unreachable and are written as matches on the uninhabited value.
//! - JxlOutputProcessorBuffer borrows its wrapper for its lifetime instead of
//!   retaining a raw wrapper pointer; its destructor releases as the source's.
//!   The source's move assignment is not reached and is absent.
//! - Callback/JPEG input, CopyFromBuffer, CopyOutput, MakeContainerHeader,
//!   AppendBoxHeader and the frame-index-box methods remain unported.

use super::base::status::{Status, StatusCode, StatusOr};
use super::codestream_header::{JxlBlendInfo, JxlFrameHeader};
use super::enc_params::CompressParams;
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
        // No SetInputSource API exists yet, so only the original buffer branch
        // can be selected. Extra channel buffers were copied by SetFromBuffer.
        self.channels[0].copy_buffer();
        true
    }
    /// The owner a queued frame retains after CopyBuffers. Every channel then
    /// holds no caller buffer: channel 0 was copied by CopyBuffers and extra
    /// channels by SetFromBuffer, so only the borrow lifetime changes.
    pub fn into_detached(self) -> JxlEncoderChunkedFrameAdapter<'static> {
        JxlEncoderChunkedFrameAdapter {
            xsize: self.xsize,
            ysize: self.ysize,
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
        self.channels[ec_index.wrapping_add(1)].get_data_at(xpos, ypos, xs, ys, row_offset)
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

/// JxlEncoderOutputProcessorWrapper in buffer mode, with fields in source
/// order. The external processor cannot be installed yet, so its branches
/// match on an uninhabited value.
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
    external_output_processor: Option<Infallible>,
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
        if let Some(never) = &self.external_output_processor {
            match *never {}
        }
        ensure!(self.output_position <= self.position);
        let additional_size = self.position - self.output_position;
        ensure!(!self.memory_manager.is_null());
        if let Some(never) = &self.external_output_processor {
            match *never {}
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
        if let Some(never) = &self.external_output_processor {
            match *never {}
        }
        ensure!(pos >= self.finalized_position);
        self.position = pos;
        Ok(())
    }

    pub fn set_finalized_position(&mut self) -> Status {
        ensure!(!self.has_buffer);
        if let Some(never) = &self.external_output_processor {
            match *never {}
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
            if let Some(never) = &self.external_output_processor {
                match *never {}
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
        if let Some(never) = &self.external_output_processor {
            match *never {}
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
                    let Some(never) = &self.external_output_processor else {
                        return Err(StatusCode::GenericError);
                    };
                    match *never {}
                }
            } else {
                let advance = buffer_last_byte.min(self.finalized_position) - self.output_position;
                self.output_position += advance;
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
            if let Some(never) = &self.external_output_processor {
                match *never {}
            }
        }
        Ok(())
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
