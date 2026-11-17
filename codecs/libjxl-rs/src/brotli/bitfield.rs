// Copyright 2015 Google Inc. All Rights Reserved.
// Distributed under the MIT license in licenses/BROTLI-MIT.txt.

//! Raw representation of the packed flags in c/dec/state.h. The source's
//! individual field assignments are read-modify-writes of one four-byte word.
//! Stable Rust cannot merge bits into a word whose other bits are
//! uninitialized, so the decoder zeroes the whole word, padding included,
//! when it creates the state (`StateOwner::initialize`), and every access is
//! then an ordinary integer load, mask and store of an initialized word. The
//! original leaves the padding bits as the allocator returned them; that is
//! the only difference (owner decision of 2026-11-16; port ADR 0003, "Brotli
//! flag word and transform tail bytes"; ADR-0044).

use std::mem::MaybeUninit;

/// A four-byte, source-aligned, initialized word within an exclusively
/// borrowed live block.
pub struct BitfieldWord<'a> {
    storage: &'a mut [MaybeUninit<u8>; 4],
}

impl<'a> BitfieldWord<'a> {
    /// # Safety
    /// All four bytes must be initialized. The decoder's flag word is: state
    /// creation zeroes it before the first field assignment.
    pub unsafe fn new(storage: &'a mut [MaybeUninit<u8>; 4]) -> Self {
        assert_eq!(storage.as_ptr() as usize % 4, 0);
        Self { storage }
    }

    fn mask(shift: u32, width: u32) -> u32 {
        assert!(width > 0 && width <= 32 && shift <= 32 - width);
        (u32::MAX >> (32 - width)) << shift
    }

    fn load(&self) -> u32 {
        // SAFETY: `new` requires four initialized bytes, aligned to four, and
        // the exclusive borrow keeps them live and unaliased.
        unsafe { self.storage.as_ptr().cast::<u32>().read() }
    }

    /// The original field assignment, including its unsigned truncation.
    pub fn write(&mut self, shift: u32, width: u32, value: u32) {
        let mask = Self::mask(shift, width);
        let bits = (value << shift) & mask;
        let word = (self.load() & !mask) | bits;
        // SAFETY: The same live, aligned, exclusively borrowed four bytes.
        unsafe { self.storage.as_mut_ptr().cast::<u32>().write(word) }
    }

    /// The original field read.
    pub fn read(&self, shift: u32, width: u32) -> u32 {
        let mask = Self::mask(shift, width);
        (self.load() & mask) >> shift
    }
}
