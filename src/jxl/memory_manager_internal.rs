// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! AlignedMemory and BytesPerRow from memory_manager_internal.{h,cc}.
//! Uses Rust's allocator and zero-initializes storage. C allocator callbacks
//! remain unported. Row sizing retains the scalar reference's four
//! byte maximum vector size. See ADR 0003.

use super::base::status::{StatusCode, StatusOr};
use std::alloc::{Layout, alloc_zeroed, dealloc};
use std::ptr::NonNull;
use std::sync::atomic::{AtomicU32, Ordering};

pub(crate) const ALIGNMENT: usize = 128;
pub(crate) const NUM_ALIGNMENT_GROUPS: usize = 16;
pub(crate) const ALIAS: usize = NUM_ALIGNMENT_GROUPS * ALIGNMENT;

pub fn bytes_per_row(xsize: usize, sizeof_t: usize) -> StatusOr<usize> {
    if xsize == 0 {
        return Ok(0);
    }
    let vec_size = 4usize;
    let mut valid = xsize
        .checked_mul(sizeof_t)
        .ok_or(StatusCode::GenericError)?;
    if vec_size != 0 {
        if vec_size < sizeof_t {
            return Err(StatusCode::GenericError);
        }
        valid = valid
            .checked_add(vec_size - sizeof_t)
            .ok_or(StatusCode::GenericError)?;
    }
    let align = vec_size.max(ALIGNMENT);
    if align & (align - 1) != 0 {
        return Err(StatusCode::GenericError);
    }
    let mut stride = valid
        .checked_add(align - 1)
        .ok_or(StatusCode::GenericError)?
        / align
        * align;
    if stride % ALIAS == 0 {
        stride = stride.checked_add(align).ok_or(StatusCode::GenericError)?;
    }
    Ok(stride)
}

pub(crate) struct AlignedMemory {
    allocation: NonNull<u8>,
    layout: Layout,
    address_offset: usize,
    size: usize,
}

// SAFETY: This value exclusively owns a global-allocator allocation, which can
// be freed on another thread. Moving it transfers that ownership exactly once.
unsafe impl Send for AlignedMemory {}

// SAFETY: Shared access exposes only initialized immutable bytes; mutation
// requires &mut self. The global allocator and group counter are thread-safe.
unsafe impl Sync for AlignedMemory {}

impl AlignedMemory {
    pub(crate) fn alias_entries(&self) -> &[super::ans_common::Entry] {
        assert_eq!(
            self.size % std::mem::size_of::<super::ans_common::Entry>(),
            0
        );
        // SAFETY: Entry is packed, contains only integers, and has no padding.
        // Zero initialization and later field writes give valid entries.
        // The checked byte count covers whole entries in this live borrow.
        unsafe { std::slice::from_raw_parts(self.bytes().as_ptr().cast(), self.size / 8) }
    }

    pub(crate) fn alias_entries_mut(&mut self) -> &mut [super::ans_common::Entry] {
        assert_eq!(
            self.size % std::mem::size_of::<super::ans_common::Entry>(),
            0
        );
        let len = self.size / 8;
        // SAFETY: Same representation and bounds proof as alias_entries;
        // the exclusive owner borrow prevents simultaneous slice aliases.
        unsafe { std::slice::from_raw_parts_mut(self.bytes_mut().as_mut_ptr().cast(), len) }
    }

    pub(crate) fn u32s(&self) -> &[u32] {
        assert_eq!(self.size % 4, 0);
        // SAFETY: create aligns to 128 bytes, all u32 bit patterns are valid,
        // and the checked byte count covers whole initialized elements.
        unsafe { std::slice::from_raw_parts(self.bytes().as_ptr().cast(), self.size / 4) }
    }

    pub(crate) fn u32s_mut(&mut self) -> &mut [u32] {
        assert_eq!(self.size % 4, 0);
        let len = self.size / 4;
        // SAFETY: Same validity/alignment/bounds proof as u32s, with exclusive
        // access through the owner for the entire returned slice lifetime.
        unsafe { std::slice::from_raw_parts_mut(self.bytes_mut().as_mut_ptr().cast(), len) }
    }

    pub(crate) fn create(size: usize, pre_padding: usize) -> StatusOr<Self> {
        if pre_padding > ALIAS {
            return Err(StatusCode::GenericError);
        }
        let allocation_size = size
            .checked_add(pre_padding)
            .and_then(|n| n.checked_add(ALIAS))
            .ok_or(StatusCode::GenericError)?;
        let layout =
            Layout::from_size_align(allocation_size, 1).map_err(|_| StatusCode::GenericError)?;
        // SAFETY: The checked layout has nonzero size (at least ALIAS) and a
        // valid alignment. The returned allocation is owned by this value and
        // is deallocated with this exact layout in Drop. All bytes start valid.
        let allocation =
            NonNull::new(unsafe { alloc_zeroed(layout) }).ok_or(StatusCode::GenericError)?;

        static NEXT_GROUP: AtomicU32 = AtomicU32::new(0);
        // Relaxed ordering is sufficient: only a rotating numeric group is
        // shared, never the allocation or its contents, as in the source.
        let group = NEXT_GROUP.fetch_add(1, Ordering::Relaxed) as usize;
        let group = group & (NUM_ALIGNMENT_GROUPS - 1);
        let offset = ALIGNMENT * group;
        let address = allocation.as_ptr() as usize + pre_padding;
        let mut aligned_address = (address & !(ALIAS - 1)) + offset;
        if aligned_address < address {
            aligned_address += ALIAS;
        }
        // Keep provenance in `allocation`; use only the integer difference to
        // create references into it, rather than constructing integer pointers.
        let address_offset = aligned_address - allocation.as_ptr() as usize;
        debug_assert!(address_offset >= pre_padding);
        debug_assert!(address_offset + size <= allocation_size);
        debug_assert_eq!(aligned_address % ALIGNMENT, 0);
        Ok(Self {
            allocation,
            layout,
            address_offset,
            size,
        })
    }

    pub(crate) fn bytes(&self) -> &[u8] {
        // SAFETY: create establishes an initialized, in-bounds range of `size`
        // bytes. Its owner lives for the borrow. The shared borrow prevents a
        // simultaneous mutable borrow through bytes_mut.
        unsafe {
            std::slice::from_raw_parts(self.allocation.as_ptr().add(self.address_offset), self.size)
        }
    }

    pub(crate) fn bytes_mut(&mut self) -> &mut [u8] {
        // SAFETY: This is the same initialized range as bytes(), with exclusive
        // access through &mut self. No pointer or mutable slice escapes its
        // borrow, and the backing allocation cannot be freed during it.
        unsafe {
            std::slice::from_raw_parts_mut(
                self.allocation.as_ptr().add(self.address_offset),
                self.size,
            )
        }
    }
}

impl Drop for AlignedMemory {
    fn drop(&mut self) {
        // SAFETY: allocation was produced by alloc_zeroed with this layout,
        // is owned exactly once, and has not been resized or deallocated.
        unsafe { dealloc(self.allocation.as_ptr(), self.layout) };
    }
}
