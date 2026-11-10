// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! The three contiguous JxlDecoder input fields and the complete original
//! SetInput/ReleaseInput/CloseInput bodies from `lib/jxl/decode.cc`.
//! MaybeUninit preserves their reached-field short circuits independently of
//! initialization of the surrounding eventual decoder object.
use super::decode::JxlDecoderStatus;
use std::mem::MaybeUninit;
use std::ptr;

#[repr(C)]
pub struct InputCursor {
    pub next_in: MaybeUninit<*const u8>,
    pub avail_in: MaybeUninit<usize>,
    pub input_closed: MaybeUninit<bool>,
}
impl Default for InputCursor {
    fn default() -> Self {
        Self {
            next_in: MaybeUninit::new(ptr::null()),
            avail_in: MaybeUninit::new(0),
            input_closed: MaybeUninit::new(false),
        }
    }
}
impl InputCursor {
    /// # Safety
    /// next_in is initialized. input_closed must be initialized only if next_in
    /// is null. On success data retains its original input lifetime/provenance
    /// and initialized extent through ReleaseInput. Other fields can be unknown.
    pub unsafe fn set_input(&mut self, data: *const u8, size: usize) -> JxlDecoderStatus {
        // SAFETY: The caller proves only this first reached scalar initialized.
        if !unsafe { self.next_in.assume_init() }.is_null() {
            return JxlDecoderStatus::ERROR;
        }
        // SAFETY: Only the null branch reaches this caller-proven flag.
        if unsafe { self.input_closed.assume_init() } {
            return JxlDecoderStatus::ERROR;
        }
        self.next_in.write(data);
        self.avail_in.write(size);
        JxlDecoderStatus::SUCCESS
    }
    /// # Safety
    /// Only avail_in must be initialized; neither old pointer nor flag is read.
    pub unsafe fn release_input(&mut self) -> usize {
        // SAFETY: This is precisely the caller-proven reached output scalar.
        let result = unsafe { self.avail_in.assume_init() };
        self.next_in.write(ptr::null());
        self.avail_in.write(0);
        result
    }
    pub fn close_input(&mut self) {
        self.input_closed.write(true);
    }
}
