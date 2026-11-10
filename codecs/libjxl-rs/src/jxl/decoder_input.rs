// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Persistent input fields and complete methods from `lib/jxl/decode.cc`:
//! CanAddBuffer/InjectNextBufferedJxlpBox and AdvanceInput through
//! GetCodestreamInput, plus SetInput/ReleaseInput/CloseInput. This is retained
//! object storage for the incremental decoder, not another complete decoder.
use super::base::status::Status;
use super::decode::JxlDecoderStatus as DecoderStatus;
use super::decoder_input_cursor::InputCursor;
use super::padded_bytes_callbacks::{MemoryManager, PaddedBytes};
use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug)]
pub struct CodestreamSpan {
    pub data: *const u8,
    pub size: usize,
}

pub struct DecoderInput<'a> {
    pub last_codestream_seen: bool,
    pub file_pos: usize,
    pub box_contents_end: usize,
    pub box_contents_unbounded: bool,
    pub codestream_copy: PaddedBytes<'a>,
    pub codestream_unconsumed: usize,
    pub codestream_pos: usize,
    pub codestream_bits_ahead: usize,
    pub next_jxlp_index: u32,
    pub jxlp_ooo_buffer: BTreeMap<u32, (Box<PaddedBytes<'a>>, bool)>,
    pub jxlp_ooo_buffer_total: usize,
    pub input: InputCursor,
}
impl<'a> DecoderInput<'a> {
    /// Reached input-field initialization from JxlDecoder Initialize/Rewind.
    /// # Safety
    /// The memory manager satisfies callback PaddedBytes' live/stable owner
    /// contract through destruction. This object and its inputs cannot be
    /// reentered or concurrently accessed through allocator callbacks.
    pub unsafe fn new(manager: *const MemoryManager) -> Self {
        Self {
            last_codestream_seen: false,
            file_pos: 0,
            box_contents_end: 0,
            box_contents_unbounded: false,
            // SAFETY: Forward the constructor's stable manager lifetime.
            codestream_copy: unsafe { PaddedBytes::new(manager) },
            codestream_unconsumed: 0,
            codestream_pos: 0,
            codestream_bits_ahead: 0,
            next_jxlp_index: 0,
            jxlp_ooo_buffer: BTreeMap::new(),
            jxlp_ooo_buffer_total: 0,
            input: InputCursor::default(),
        }
    }
    pub fn can_add_buffer(&self, length: usize) -> bool {
        let limit = 1usize << if usize::BITS == 32 { 30 } else { 48 };
        length < limit
            && length
                .wrapping_add(self.jxlp_ooo_buffer_total)
                .wrapping_add(self.codestream_copy.len())
                < limit
    }
    /// # Safety
    /// The exact source map entry's payload is initialized and remains owned
    /// and disjoint from codestream_copy throughout any allocation callback.
    pub unsafe fn inject_next_buffered_jxlp_box(&mut self, success: &mut bool) -> Status {
        *success = false;
        let index = self.next_jxlp_index;
        let Some((data, _)) = self.jxlp_ooo_buffer.get(&index) else {
            return Ok(());
        };
        let length = data.len();
        let begin = data.as_ptr();
        let end = begin.wrapping_add(length);
        // SAFETY: Caller guarantees source map payload lifetime/nonoverlap;
        // raw addresses avoid lending input bytes across allocation callbacks.
        unsafe {
            self.codestream_copy.append_raw(begin, end)?;
        }
        if self.jxlp_ooo_buffer.get(&index).unwrap().1 {
            self.last_codestream_seen = true;
        }
        self.next_jxlp_index = self.next_jxlp_index.wrapping_add(1);
        // std::map erases/destroys the payload before decrementing the total.
        drop(self.jxlp_ooo_buffer.remove(&index));
        self.jxlp_ooo_buffer_total = self.jxlp_ooo_buffer_total.wrapping_sub(length);
        *success = true;
        Ok(())
    }
    /// # Safety
    /// Original next_in denotes a valid live pointer whose reached advance is
    /// within its input allocation; avail_in >= size, as asserted by source.
    pub unsafe fn advance_input(&mut self, size: usize) {
        debug_assert!(unsafe { self.input.avail_in.assume_init() } >= size);
        // SAFETY: Caller establishes source same-allocation advance domain.
        self.input
            .next_in
            .write(unsafe { self.input.next_in.assume_init().add(size) });
        self.input
            .avail_in
            .write(unsafe { self.input.avail_in.assume_init() }.wrapping_sub(size));
        self.file_pos = self.file_pos.wrapping_add(size);
    }
    /// # Safety
    /// Source avail_in is initialized by decoder initialization/SetInput.
    pub unsafe fn available_codestream(&self) -> usize {
        let mut available = unsafe { self.input.avail_in.assume_init() };
        if !self.box_contents_unbounded {
            available = available.min(self.box_contents_end.wrapping_sub(self.file_pos));
        }
        available
    }
    /// # Safety
    /// The original source input/history and reached pointer-advance domain
    /// hold. All prior CodestreamSpan loans ended before mutation.
    pub unsafe fn advance_codestream(&mut self, size: usize) {
        let available = unsafe { self.available_codestream() };
        if self.codestream_copy.is_empty() {
            if size <= available {
                unsafe {
                    self.advance_input(size);
                }
            } else {
                self.codestream_pos = size.wrapping_sub(available);
                unsafe {
                    self.advance_input(available);
                }
            }
        } else {
            self.codestream_pos = self.codestream_pos.wrapping_add(size);
            if self.codestream_pos.wrapping_add(self.codestream_unconsumed)
                >= self.codestream_copy.len()
            {
                let advance = self.codestream_unconsumed.min(
                    self.codestream_unconsumed
                        .wrapping_add(self.codestream_pos)
                        .wrapping_sub(self.codestream_copy.len()),
                );
                unsafe {
                    self.advance_input(advance);
                }
                self.codestream_pos = self
                    .codestream_pos
                    .wrapping_sub(self.codestream_pos.min(self.codestream_copy.len()));
                self.codestream_unconsumed = 0;
                self.codestream_copy.clear();
            }
        }
    }
    /// # Safety
    /// Reached input and all forward memcpy/advance extents are initialized,
    /// live and nonoverlapping with codestream_copy through callbacks. No
    /// input/storage/span reference survives a callback or mutation.
    pub unsafe fn request_more_input(&mut self) -> DecoderStatus {
        if self.codestream_copy.is_empty() {
            let available = unsafe { self.available_codestream() };
            if !self.can_add_buffer(available) {
                return DecoderStatus::ERROR;
            }
            let end = unsafe { self.input.next_in.assume_init() }.wrapping_add(available);
            if unsafe {
                self.codestream_copy
                    .append_raw(self.input.next_in.assume_init(), end)
            }
            .is_err()
            {
                return DecoderStatus::ERROR;
            }
            unsafe {
                self.advance_input(available);
            }
        } else {
            unsafe {
                self.advance_input(self.codestream_unconsumed);
            }
            self.codestream_unconsumed = 0;
        }
        DecoderStatus::NEED_MORE_INPUT
    }
    /// # Safety
    /// Same original lifetime/nonoverlap/arithmetic domain as request_more_input.
    /// A successful span is borrowed from current external input or this SAME
    /// codestream_copy owner, valid only until the next mutation/release. The
    /// caller ends every byte loan before an allocator callback or API call.
    /// Error/need-more paths leave the supplied span unchanged.
    pub unsafe fn get_codestream_input(&mut self, span: &mut CodestreamSpan) -> DecoderStatus {
        if self.codestream_copy.is_empty() && self.codestream_pos > 0 {
            let available = unsafe { self.available_codestream() };
            let skip = self.codestream_pos.min(available);
            unsafe {
                self.advance_input(skip);
            }
            self.codestream_pos -= skip;
            if self.codestream_pos > 0 {
                return unsafe { self.request_more_input() };
            }
        }
        if self.codestream_pos > self.codestream_copy.len() {
            return DecoderStatus::ERROR;
        }
        if self.codestream_unconsumed > self.codestream_copy.len() {
            return DecoderStatus::ERROR;
        }
        let available = unsafe { self.available_codestream() };
        if self.codestream_copy.is_empty() {
            if available == 0 {
                return unsafe { self.request_more_input() };
            }
            *span = CodestreamSpan {
                data: unsafe { self.input.next_in.assume_init() },
                size: available,
            };
            DecoderStatus::SUCCESS
        } else {
            if !self.can_add_buffer(available) {
                return DecoderStatus::ERROR;
            }
            let begin = unsafe { self.input.next_in.assume_init() }
                .wrapping_add(self.codestream_unconsumed);
            let end = unsafe { self.input.next_in.assume_init() }.wrapping_add(available);
            if unsafe { self.codestream_copy.append_raw(begin, end) }.is_err() {
                return DecoderStatus::ERROR;
            }
            self.codestream_unconsumed = available;
            *span = CodestreamSpan {
                data: self
                    .codestream_copy
                    .as_ptr()
                    .wrapping_add(self.codestream_pos),
                size: self.codestream_copy.len() - self.codestream_pos,
            };
            DecoderStatus::SUCCESS
        }
    }
    /// # Safety
    /// Retain `data`'s source lifetime, provenance and initialized extent until
    /// release_input, including a non-null zero-length buffer's identity.
    pub unsafe fn set_input(&mut self, data: *const u8, size: usize) -> DecoderStatus {
        // SAFETY: Decoder initialization and the retained caller input contract
        // establish the cursor's reached fields; no other decoder fields read.
        unsafe { self.input.set_input(data, size) }
    }
    pub fn release_input(&mut self) -> usize {
        // SAFETY: Decoder initialization/SetInput initialize this exact count.
        unsafe { self.input.release_input() }
    }
    pub fn close_input(&mut self) {
        self.input.close_input();
    }
}

impl Drop for DecoderInput<'_> {
    fn drop(&mut self) {
        self.jxlp_ooo_buffer.clear();
    }
}
