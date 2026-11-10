// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Ported from `lib/jxl/dec_bit_reader.{h,cc}`.
//! Borrowed bytes and byte offsets replace C++ pointers. Buffering, refill,
//! zero-extension, bounds checkpoints, and close behavior follow the source.

use super::base::byte_order::load_le64;
use super::base::status::{Status, StatusCode};
use std::ops::{Deref, DerefMut};

#[derive(Default)]
pub struct BitReader<'a> {
    buf: u64,
    bits_in_buf: usize,
    bytes: Option<&'a [u8]>,
    next_byte: usize,
    overread_bytes: u64,
    close_called: bool,
    checked_out_of_bounds_bits: u64,
}

impl<'a> BitReader<'a> {
    pub const MAX_BITS_PER_CALL: usize = 56;

    pub fn new(bytes: &'a [u8]) -> Self {
        let mut reader = Self {
            bytes: Some(bytes),
            ..Self::default()
        };
        reader.refill();
        reader
    }

    pub fn refill(&mut self) {
        let bytes = self.bytes.unwrap_or_default();
        if bytes.len() - self.next_byte < 8 {
            self.bounds_checked_refill();
        } else {
            self.buf |= load_le64(&bytes[self.next_byte..]) << self.bits_in_buf;
            self.next_byte += (63 - self.bits_in_buf) >> 3;
            self.bits_in_buf |= 56;
            debug_assert!(56 <= self.bits_in_buf && self.bits_in_buf < 64);
        }
    }

    pub fn peek_fixed_bits<const N: usize>(&self) -> u64 {
        assert!(N <= Self::MAX_BITS_PER_CALL);
        debug_assert!(!self.close_called);
        self.buf & ((1u64 << N) - 1)
    }

    pub fn peek_bits(&self, nbits: usize) -> u64 {
        assert!(nbits <= Self::MAX_BITS_PER_CALL);
        debug_assert!(!self.close_called);
        self.buf & ((1u64 << nbits) - 1)
    }

    pub fn consume(&mut self, num_bits: usize) {
        debug_assert!(!self.close_called);
        assert!(num_bits <= self.bits_in_buf);
        self.bits_in_buf -= num_bits;
        self.buf >>= num_bits;
    }

    pub fn read_bits(&mut self, nbits: usize) -> u64 {
        debug_assert!(!self.close_called);
        self.refill();
        let bits = self.peek_bits(nbits);
        self.consume(nbits);
        bits
    }

    pub fn read_fixed_bits<const N: usize>(&mut self) -> u64 {
        debug_assert!(!self.close_called);
        self.refill();
        let bits = self.peek_fixed_bits::<N>();
        self.consume(N);
        bits
    }

    pub fn skip_bits(&mut self, mut skip: usize) {
        debug_assert!(!self.close_called);
        if skip <= self.bits_in_buf {
            self.consume(skip);
            return;
        }

        skip -= self.bits_in_buf;
        self.bits_in_buf = 0;
        self.buf = 0;

        let whole_bytes = skip / 8;
        skip %= 8;
        if whole_bytes > self.total_bytes() - self.next_byte {
            self.next_byte = self.total_bytes();
            skip += 8;
        } else {
            self.next_byte += whole_bytes;
        }
        self.refill();
        self.consume(skip);
    }

    pub fn total_bits_consumed(&self) -> usize {
        (self.next_byte as u64)
            .wrapping_add(self.overread_bytes)
            .wrapping_mul(8)
            .wrapping_sub(self.bits_in_buf as u64) as usize
    }

    pub fn jump_to_byte_boundary(&mut self) -> Status {
        let remainder = self.total_bits_consumed() % 8;
        if remainder == 0 {
            return Ok(());
        }
        if self.read_bits(8 - remainder) != 0 {
            return Err(StatusCode::GenericError);
        }
        Ok(())
    }

    pub fn first_byte(&self) -> Option<&'a [u8]> {
        self.bytes
    }

    pub fn total_bytes(&self) -> usize {
        self.bytes.unwrap_or_default().len()
    }

    pub fn mark_unhealthy(&mut self) {
        self.next_byte = self.total_bytes();
        self.overread_bytes = self.overread_bytes.wrapping_add(8);
    }

    pub fn all_reads_within_bounds(&mut self) -> Status {
        self.checked_out_of_bounds_bits = self.total_bits_consumed() as u64;
        if self.total_bits_consumed() > self.total_bytes().wrapping_mul(8) {
            return Err(StatusCode::GenericError);
        }
        Ok(())
    }

    pub fn close(&mut self) -> Status {
        debug_assert!(!self.close_called);
        self.close_called = true;
        if self.bytes.is_none() {
            return Ok(());
        }
        if self.total_bits_consumed() as u64 > self.checked_out_of_bounds_bits
            && self.total_bits_consumed() > self.total_bytes().wrapping_mul(8)
        {
            return Err(StatusCode::GenericError);
        }
        Ok(())
    }

    fn bounds_checked_refill(&mut self) {
        let bytes = self.bytes.unwrap_or_default();
        while self.bits_in_buf < 56 {
            if self.next_byte >= bytes.len() {
                break;
            }
            self.buf |= u64::from(bytes[self.next_byte]) << self.bits_in_buf;
            self.next_byte += 1;
            self.bits_in_buf += 8;
        }
        debug_assert!(self.bits_in_buf < 64);
        let extra_bytes = (63 - self.bits_in_buf) / 8;
        self.overread_bytes = self.overread_bytes.wrapping_add(extra_bytes as u64);
        self.bits_in_buf += extra_bytes * 8;
        debug_assert!(56 <= self.bits_in_buf && self.bits_in_buf < 64);
    }
}

impl Drop for BitReader<'_> {
    fn drop(&mut self) {
        // Preserve a primary Rust panic instead of aborting with a second panic
        // during unwinding. Normal destruction still checks the close contract.
        debug_assert!(self.close_called || self.bytes.is_none() || std::thread::panicking());
    }
}

/// A borrowed guard that propagates a close failure when its scope ends.
/// Access the reader through the guard while it owns the mutable borrow.
pub struct BitReaderScopedCloser<'r, 'a> {
    reader: &'r mut BitReader<'a>,
    status: &'r mut Status,
}

impl<'r, 'a> BitReaderScopedCloser<'r, 'a> {
    pub fn new(reader: &'r mut BitReader<'a>, status: &'r mut Status) -> Self {
        Self { reader, status }
    }
}

impl<'a> Deref for BitReaderScopedCloser<'_, 'a> {
    type Target = BitReader<'a>;

    fn deref(&self) -> &Self::Target {
        self.reader
    }
}

impl DerefMut for BitReaderScopedCloser<'_, '_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.reader
    }
}

impl Drop for BitReaderScopedCloser<'_, '_> {
    fn drop(&mut self) {
        if let Err(code) = self.reader.close() {
            *self.status = Err(code);
        }
    }
}
