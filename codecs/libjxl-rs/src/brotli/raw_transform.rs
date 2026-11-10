// Copyright 2013 Google Inc. All Rights Reserved.
// Distributed under the MIT license in licenses/BROTLI-MIT.txt.

//! Borrowed storage representation for c/common/transform.c. The original
//! writes prefix/word/suffix bytes in order, but its uppercase operation can
//! XOR a still-unknown destination tail. No unknown byte becomes a Rust value.
//! Native proof is required before this candidate enters decoder state.

use super::raw_bytes::xor_byte;
use super::transform::ByteDestination;
use std::mem::MaybeUninit;

/// An exclusive view of the actual destination block, with initialized-byte
/// knowledge separate from physical capacity. This view owns no allocation.
/// It models the contiguous writes performed by the dictionary transformer;
/// it does not claim that all of a ringbuffer's capacity is initialized.
pub struct RawByteDestination<'a> {
    storage: &'a mut [MaybeUninit<u8>],
    initialized_prefix: usize,
}

impl<'a> RawByteDestination<'a> {
    /// No prior byte is assumed initialized, even if an allocator happened to
    /// seed its block. Reached ordinary writes alone extend the known prefix.
    pub fn new(storage: &'a mut [MaybeUninit<u8>]) -> Self {
        Self {
            storage,
            initialized_prefix: 0,
        }
    }

    /// Retain prior initialized-byte knowledge without copying the block.
    ///
    /// # Safety
    /// Every byte before `initialized_prefix` must already be initialized.
    /// The exclusive storage borrow rules out allocator/caller mutation for
    /// the view's lifetime. Bytes after that prefix need not be initialized.
    pub unsafe fn with_initialized_prefix(
        storage: &'a mut [MaybeUninit<u8>],
        initialized_prefix: usize,
    ) -> Self {
        assert!(initialized_prefix <= storage.len());
        Self {
            storage,
            initialized_prefix,
        }
    }

    pub fn initialized_prefix_len(&self) -> usize {
        self.initialized_prefix
    }

    /// Only bytes proved initialized by prior knowledge or actual writes.
    pub fn initialized_prefix(&self) -> &[u8] {
        // SAFETY: The constructor requires initialization for its prefix;
        // write extends it only by initializing the next byte, and xor keeps
        // an initialized byte initialized. This shared borrow cannot outlive
        // or coexist with mutation through the exclusive destination view.
        unsafe { std::slice::from_raw_parts(self.storage.as_ptr().cast(), self.initialized_prefix) }
    }
}

impl ByteDestination for RawByteDestination<'_> {
    fn read(&self, index: usize) -> u8 {
        assert!(index < self.initialized_prefix);
        // SAFETY: The checked contiguous-prefix invariant proves this byte
        // initialized. No callback or other alias mutates the borrowed block.
        unsafe { self.storage[index].assume_init() }
    }

    fn write(&mut self, index: usize, value: u8) {
        assert!(index <= self.initialized_prefix);
        self.storage[index].write(value);
        if index == self.initialized_prefix {
            self.initialized_prefix += 1;
        }
    }

    fn xor(&mut self, index: usize, mask: u8) {
        // xor_byte operates only inside this exclusively borrowed live byte.
        // A source tail XOR preserves its unknown domain; it is not a write
        // of a known Rust scalar and must not extend initialized_prefix.
        xor_byte(&mut self.storage[index], mask);
    }
}
