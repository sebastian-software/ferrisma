// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Ported from `lib/jxl/padded_bytes.h`, for the BitWriter storage boundary.
//! Additional initialization and the Rust allocator are documented in ADR 0003.

use super::base::status::{Status, StatusCode};
use super::memory_manager_internal::AlignedMemory;

#[derive(Default)]
pub(crate) struct PaddedBytes {
    size: usize,
    capacity: usize,
    data: Option<AlignedMemory>,
}

impl PaddedBytes {
    pub(crate) fn reserve(&mut self, capacity: usize) -> Status {
        if capacity <= self.capacity {
            return Ok(());
        }
        let new_capacity = self
            .capacity
            .checked_add(self.capacity / 2)
            .ok_or(StatusCode::GenericError)?;
        self.change_capacity(capacity.max(new_capacity))
    }

    pub(crate) fn shrink_to_fit(&mut self) -> Status {
        self.change_capacity(self.size)
    }

    pub(crate) fn resize(&mut self, size: usize) -> Status {
        self.reserve(size)?;
        self.size = size;
        Ok(())
    }

    pub(crate) fn size(&self) -> usize {
        self.size
    }

    pub(crate) fn bytes(&self) -> &[u8] {
        self.data.as_ref().map_or(&[], AlignedMemory::bytes)
    }

    pub(crate) fn bytes_mut(&mut self) -> &mut [u8] {
        self.data.as_mut().map_or(&mut [], AlignedMemory::bytes_mut)
    }

    fn change_capacity(&mut self, new_capacity: usize) -> Status {
        debug_assert!(new_capacity >= self.size);
        let new_capacity = new_capacity.max(64);
        let padded_capacity = new_capacity
            .checked_add(8)
            .ok_or(StatusCode::GenericError)?;
        let mut new_data = AlignedMemory::create(padded_capacity, 0)?;
        if let Some(data) = &self.data {
            new_data.bytes_mut()[..self.size].copy_from_slice(&data.bytes()[..self.size]);
            new_data.bytes_mut()[self.size] = 0;
        } else {
            new_data.bytes_mut()[0] = 0;
        }
        self.capacity = new_capacity;
        self.data = Some(new_data);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::jxl::memory_manager_internal::ALIGNMENT;

    #[test]
    fn storage_preserves_alignment_growth_padding_and_initialized_prefix() {
        let mut bytes = PaddedBytes::default();
        for size in [1, 20, 64, 65, 170, 4096] {
            let previous = bytes.size();
            let old = bytes.bytes()[..previous].to_vec();
            bytes.resize(size).unwrap();
            assert_eq!(&bytes.bytes()[..previous], old);
            assert_eq!(bytes.bytes().as_ptr() as usize % ALIGNMENT, 0);
            assert!(bytes.bytes().len() >= size + 8);
            assert_eq!(bytes.bytes()[previous], 0);
            for i in previous..size {
                bytes.bytes_mut()[i] = i as u8;
            }
        }
        bytes.resize(170).unwrap();
        bytes.shrink_to_fit().unwrap();
        assert_eq!(bytes.capacity, 170);
        assert_eq!(bytes.bytes().len(), 178);
        assert_eq!(bytes.bytes()[170], 0);
        assert_eq!(bytes.reserve(usize::MAX), Err(StatusCode::GenericError));
        assert_eq!(bytes.size(), 170);
        assert_eq!(bytes.bytes()[169], 169);
    }
}
