// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Ported from `lib/jxl/enc_bit_writer.{h,cc}`.
//! Owned stack indices replace allotment parent pointers; closures receive an
//! exclusive writer borrow. Custom allocators and PaddedBytes transfer remain
//! outside the current boundary. See ADR 0003.

use super::base::status::{Status, StatusCode};
use super::enc_aux_out::{AuxOut, LayerType};
use super::padded_bytes::PaddedBytes;

#[derive(Default)]
pub struct BitWriter {
    bits_written: usize,
    storage: PaddedBytes,
    allotments: Vec<Allotment>,
}

struct Allotment {
    prev_bits_written: usize,
    max_bits: usize,
    histogram_bits: usize,
}

impl BitWriter {
    pub const MAX_BITS_PER_CALL: usize = 56;

    pub fn new() -> Self {
        Self::default()
    }

    pub fn bits_written(&self) -> usize {
        self.bits_written
    }

    pub fn get_span(&self) -> &[u8] {
        assert_eq!(self.bits_written % 8, 0);
        &self.storage.bytes()[..self.bits_written / 8]
    }

    pub fn shrink(&mut self) -> Status {
        self.storage.shrink_to_fit()
    }

    pub fn append_byte_aligned(&mut self, span: &[u8]) -> Status {
        if span.is_empty() {
            return Ok(());
        }
        let size = self.storage.size().wrapping_add(span.len()).wrapping_add(1);
        self.storage.resize(size)?;
        if self.bits_written % 8 != 0 {
            return Err(StatusCode::GenericError);
        }
        let mut pos = self.bits_written / 8;
        self.storage.bytes_mut()[pos..pos + span.len()].copy_from_slice(span);
        pos += span.len();
        if pos >= self.storage.size() {
            return Err(StatusCode::GenericError);
        }
        self.storage.bytes_mut()[pos] = 0;
        self.bits_written = self.bits_written.wrapping_add(span.len().wrapping_mul(8));
        Ok(())
    }

    pub fn append_byte_aligned_writers(&mut self, others: &[&BitWriter]) -> Status {
        let mut other_bytes: usize = 0;
        for writer in others {
            if writer.bits_written % 8 != 0 {
                return Err(StatusCode::GenericError);
            }
            other_bytes = other_bytes.wrapping_add(writer.bits_written / 8);
        }
        if other_bytes == 0 {
            return Ok(());
        }
        let size = self
            .storage
            .size()
            .wrapping_add(other_bytes)
            .wrapping_add(1);
        self.storage.resize(size)?;
        if self.bits_written % 8 != 0 {
            return Err(StatusCode::GenericError);
        }
        let mut pos = self.bits_written / 8;
        for writer in others {
            let span = writer.get_span();
            self.storage.bytes_mut()[pos..pos + span.len()].copy_from_slice(span);
            pos += span.len();
        }
        if pos >= self.storage.size() {
            return Err(StatusCode::GenericError);
        }
        self.storage.bytes_mut()[pos] = 0;
        self.bits_written = self.bits_written.wrapping_add(other_bytes.wrapping_mul(8));
        Ok(())
    }

    pub fn append_unaligned(&mut self, other: &BitWriter) -> Status {
        self.with_max_bits(
            other.bits_written(),
            LayerType::Header,
            None,
            |writer, _| {
                let full_bytes = other.bits_written / 8;
                let remaining_bits = other.bits_written % 8;
                for i in 0..full_bytes {
                    writer.write(8, u64::from(other.storage.bytes()[i]));
                }
                if remaining_bits > 0 {
                    writer.write(
                        remaining_bits,
                        u64::from(other.storage.bytes()[full_bytes]) & ((1 << remaining_bits) - 1),
                    );
                }
                Ok(())
            },
            false,
        )
    }

    pub fn write(&mut self, n_bits: usize, mut bits: u64) {
        assert!(n_bits <= Self::MAX_BITS_PER_CALL);
        assert_eq!(bits >> n_bits, 0);
        let bytes_written = self.bits_written / 8;
        debug_assert!(bytes_written <= self.storage.size());
        let bits_in_first_byte = self.bits_written % 8;
        bits <<= bits_in_first_byte;
        let mut v = u64::from(self.storage.bytes()[bytes_written]);
        debug_assert_eq!(v >> bits_in_first_byte, 0);
        v |= bits;
        self.storage.bytes_mut()[bytes_written..bytes_written + 8]
            .copy_from_slice(&v.to_le_bytes());
        self.bits_written = self.bits_written.wrapping_add(n_bits);
    }

    pub fn zero_pad_to_byte(&mut self) {
        let remainder_bits = self.bits_written.wrapping_add(7) & !7;
        let remainder_bits = remainder_bits.wrapping_sub(self.bits_written);
        if remainder_bits == 0 {
            return;
        }
        self.write(remainder_bits, 0);
        debug_assert_eq!(self.bits_written % 8, 0);
    }

    pub fn with_max_bits(
        &mut self,
        max_bits: usize,
        layer: LayerType,
        mut aux_out: Option<&mut AuxOut>,
        function: impl FnOnce(&mut Self, Option<&mut AuxOut>) -> Status,
        finished_histogram: bool,
    ) -> Status {
        self.allotments
            .try_reserve(1)
            .map_err(|_| StatusCode::GenericError)?;
        let prev_bytes = self.storage.size();
        let next_bytes = max_bits.wrapping_add(7) / 8;
        self.storage.resize(prev_bytes.wrapping_add(next_bytes))?;
        self.allotments.push(Allotment {
            prev_bits_written: self.bits_written,
            max_bits,
            histogram_bits: 0,
        });

        // Reborrow the source's callback context instead of aliasing the
        // mutable AuxOut through a captured C++ pointer.
        let result = function(self, aux_out.as_deref_mut());
        let allotment = self.allotments.last_mut().expect("active allotment");
        if result.is_ok() && finished_histogram {
            debug_assert!(self.bits_written >= allotment.prev_bits_written);
            allotment.histogram_bits = self.bits_written - allotment.prev_bits_written;
        }

        let allotment = self.allotments.pop().expect("active allotment");
        assert!(self.bits_written >= allotment.prev_bits_written);
        let used_bits = self.bits_written - allotment.prev_bits_written;
        assert!(used_bits <= allotment.max_bits, "bit allotment exceeded");
        let unused_bits = allotment.max_bits - used_bits;
        let unused_bytes = unused_bits / 8;
        if self.storage.size() < unused_bytes {
            return Err(StatusCode::GenericError);
        }
        self.storage.resize(self.storage.size() - unused_bytes)?;
        for parent in &mut self.allotments {
            parent.prev_bits_written = parent.prev_bits_written.wrapping_add(used_bits);
        }
        if let Some(aux_out) = aux_out {
            let totals = aux_out.layer_mut(layer);
            totals.total_bits = totals.total_bits.wrapping_add(used_bits);
            totals.histogram_bits = totals.histogram_bits.wrapping_add(allotment.histogram_bits);
        }
        result
    }
}
