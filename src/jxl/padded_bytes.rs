// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Ported from `lib/jxl/padded_bytes.h`, for BitWriter and ICC command storage.
//! Storage allocates through the owner's memory manager; `Default` selects the
//! native allocator boundary. Additional initialization is documented in ADR
//! 0003.

use super::base::status::{Status, StatusCode};
use super::memory_manager_internal::{AlignedMemory, MemoryManagerPtr};

pub struct PaddedBytes {
    memory_manager: MemoryManagerPtr,
    size: usize,
    capacity: usize,
    data: Option<AlignedMemory>,
}

impl Default for PaddedBytes {
    fn default() -> Self {
        Self::new(MemoryManagerPtr::NATIVE)
    }
}

impl PaddedBytes {
    /// The source PaddedBytes(JxlMemoryManager*) constructor.
    pub fn new(memory_manager: MemoryManagerPtr) -> Self {
        Self {
            memory_manager,
            size: 0,
            capacity: 0,
            data: None,
        }
    }
    pub fn with_initial_space(size: usize) -> Result<Self, StatusCode> {
        Self::with_initial_space_in(MemoryManagerPtr::NATIVE, size)
    }
    /// The source WithInitialSpace(memory_manager, size).
    pub fn with_initial_space_in(
        memory_manager: MemoryManagerPtr,
        size: usize,
    ) -> Result<Self, StatusCode> {
        let mut result = Self::new(memory_manager);
        result.size = size;
        result.reserve(size)?;
        Ok(result)
    }
    pub fn memory_manager(&self) -> MemoryManagerPtr {
        self.memory_manager
    }
    pub fn clear(&mut self) {
        self.resize(0).expect("resizing to zero cannot allocate");
    }
    pub fn push_back(&mut self, value: u8) -> Status {
        if self.size == self.capacity {
            self.reserve(self.capacity.wrapping_add(1))?;
        }
        let pos = self.size;
        self.bytes_mut()[pos] = value;
        self.size += 1;
        Ok(())
    }
    pub fn append(&mut self, bytes: &[u8]) -> Status {
        if !bytes.is_empty() {
            let pos = self.size;
            let size = pos
                .checked_add(bytes.len())
                .ok_or(StatusCode::GenericError)?;
            self.resize(size)?;
            self.bytes_mut()[pos..size].copy_from_slice(bytes);
        }
        Ok(())
    }
    pub fn as_slice(&self) -> &[u8] {
        &self.bytes()[..self.size]
    }
    pub fn len(&self) -> usize {
        self.size
    }
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
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
        let mut new_data = AlignedMemory::create(self.memory_manager, padded_capacity, 0)?;
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
