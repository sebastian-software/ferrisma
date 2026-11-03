// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Callback-owned PaddedBytes from `lib/jxl/padded_bytes.h` and the reached
//! AlignedMemory allocation/move/free path in `memory_manager_internal.cc`.
//! Unlike the existing global-allocator representation, unwritten padding is
//! unknown. Only append's initialized logical prefix is exposed as bytes.
use super::base::status::{Status, StatusCode};
use super::memory_manager_internal::next_alignment_offset;
use std::marker::PhantomData;
use std::ptr;

pub use super::memory_manager::{
    JpegxlAllocFunc as AllocFunc, JpegxlFreeFunc as FreeFunc, JxlMemoryManager as MemoryManager,
};

struct Allocation<'a> {
    allocation: *mut u8,
    address: *mut u8,
    manager: *const MemoryManager,
    lifetime: PhantomData<&'a MemoryManager>,
}
impl Allocation<'_> {
    unsafe fn create(manager: *const MemoryManager, size: usize) -> Result<Self, StatusCode> {
        let allocation_size = size.checked_add(2048).ok_or(StatusCode::GenericError)?;
        // SAFETY: PaddedBytes' constructor contract retains the live native
        // manager and matching callable fields. No input/storage loan is held.
        let allocation =
            unsafe { ((*manager).alloc.unwrap())((*manager).opaque, allocation_size) }.cast::<u8>();
        if allocation.is_null() {
            return Err(StatusCode::GenericError);
        }
        let offset = next_alignment_offset();
        let address = allocation as usize;
        let mut aligned = (address & !2047) + offset;
        if aligned < address {
            aligned += 2048;
        }
        // SAFETY: Source's 2048-byte over-allocation covers the checked same
        // allocation offset and `size`; preserve provenance from the callback.
        let pointer = unsafe { allocation.add(aligned - address) };
        Ok(Self {
            allocation,
            address: pointer,
            manager,
            lifetime: PhantomData,
        })
    }
    fn release(&mut self) {
        if self.allocation.is_null() {
            return;
        }
        // SAFETY: This is the unique still-live callback allocation; the
        // manager outlives it. Keep its old fields visible through the callback,
        // then relinquish ownership exactly once. Callbacks cannot unwind.
        unsafe {
            ((*self.manager).free.unwrap())((*self.manager).opaque, self.allocation.cast());
        }
        self.allocation = ptr::null_mut();
    }
}
impl Drop for Allocation<'_> {
    fn drop(&mut self) {
        self.release();
    }
}

pub struct PaddedBytes<'a> {
    manager: *const MemoryManager,
    size: usize,
    capacity: usize,
    data: Option<Allocation<'a>>,
}
impl<'a> PaddedBytes<'a> {
    /// # Safety
    /// `manager` is a stable live initialized memory-manager object through
    /// this owner and every allocation's destruction. Its non-null callbacks
    /// obey memory_manager.h, return live nonoverlapping writable allocations
    /// of the requested size, and cannot unwind or reenter this owner. No
    /// reference to source input/storage may survive an allocator/free callback.
    pub unsafe fn new(manager: *const MemoryManager) -> Self {
        Self {
            manager,
            size: 0,
            capacity: 0,
            data: None,
        }
    }
    pub fn len(&self) -> usize {
        self.size
    }
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
    pub fn capacity(&self) -> usize {
        self.capacity
    }
    pub fn as_ptr(&self) -> *const u8 {
        self.data.as_ref().map_or(ptr::null(), |data| data.address)
    }
    pub fn as_slice(&self) -> &[u8] {
        if self.size == 0 {
            return &[];
        }
        // SAFETY: Only successful append grows size; every logical byte has
        // been copied from initialized input. The borrow cannot outlive owner.
        unsafe { std::slice::from_raw_parts(self.as_ptr(), self.size) }
    }
    pub fn clear(&mut self) {
        self.size = 0;
    }
    fn reserve(&mut self, capacity: usize) -> Status {
        if capacity <= self.capacity {
            return Ok(());
        }
        let new_capacity = self
            .capacity
            .checked_add(self.capacity / 2)
            .ok_or(StatusCode::GenericError)?
            .max(capacity)
            .max(64);
        let padded = new_capacity
            .checked_add(8)
            .ok_or(StatusCode::GenericError)?;
        // SAFETY: Constructor retains this same initialized live manager.
        let new_data = unsafe { Allocation::create(self.manager, padded)? };
        // SAFETY: New allocation covers padded capacity; old logical prefix is
        // initialized. The distinct live owners cannot overlap. Source writes
        // only first byte on first allocation, or first new byte after copying.
        unsafe {
            if let Some(old) = &self.data {
                ptr::copy(old.address, new_data.address, self.size);
                new_data.address.add(self.size).write(0);
            } else {
                new_data.address.write(0);
            }
        }
        self.capacity = new_capacity;
        if let Some(old) = &mut self.data {
            old.release();
        }
        self.data = Some(new_data);
        Ok(())
    }
    /// Source append(begin,end), including a no-op for reversed/empty ranges.
    /// # Safety
    /// A forward range is initialized input in one live allocation; it is
    /// nonoverlapping with this owner and remains live through any callback.
    /// The callback may mutate raw input between ended operations; no input
    /// reference is created or retained while it runs. Pointer arithmetic and
    /// the source memcpy extents must be valid. Callbacks cannot reenter owner.
    pub unsafe fn append_raw(&mut self, begin: *const u8, end: *const u8) -> Status {
        if end > begin {
            let length = (end as usize) - (begin as usize);
            let old_size = self.size;
            let size = old_size
                .checked_add(length)
                .ok_or(StatusCode::GenericError)?;
            self.reserve(size)?;
            self.size = size;
            // SAFETY: Caller proves initialized/nonoverlapping forward source
            // range; successful reserve covers new size. No loan spans callback.
            unsafe {
                ptr::copy_nonoverlapping(
                    begin,
                    self.data.as_ref().unwrap().address.add(old_size),
                    length,
                );
            }
        }
        Ok(())
    }
}
