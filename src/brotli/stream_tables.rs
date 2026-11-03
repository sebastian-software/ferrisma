// Copyright 2013 Google Inc. All Rights Reserved.
// Distributed under the MIT license in licenses/BROTLI-MIT.txt.

//! Native decoder table/byte views. Tokens are resolved only against their
//! actual allocator owner; no integer is dereferenced and no payload copied.
use super::layout::HuffmanCodeStorage;
use super::memory::Allocation;
use super::symbols::HuffmanTable;
use std::mem::{align_of, size_of};

pub(crate) struct NativeTable<'a> {
    owner: &'a Allocation,
    offset: usize,
}
impl<'a> NativeTable<'a> {
    /// # Safety
    /// This is an internal source-algorithm view, not a generic table API.
    /// The retained source tree/history initializes each bits/value field at
    /// every index the subsequent source algorithm reaches through this view.
    pub(crate) unsafe fn new(owner: &'a Allocation, token: usize) -> Self {
        let offset = token.checked_sub(owner.as_ptr() as usize).unwrap();
        assert!(offset <= owner.len());
        assert_eq!(offset % size_of::<HuffmanCodeStorage>(), 0);
        assert_eq!(token % align_of::<HuffmanCodeStorage>(), 0);
        Self { owner, offset }
    }
    fn code(&self, index: usize) -> *const HuffmanCodeStorage {
        let at = self
            .offset
            .checked_add(index.checked_mul(size_of::<HuffmanCodeStorage>()).unwrap())
            .unwrap();
        assert!(at <= self.owner.len() && size_of::<HuffmanCodeStorage>() <= self.owner.len() - at);
        // SAFETY: Checked same actual owner extent/alignment, address only.
        unsafe { self.owner.as_ptr().add(at).cast() }
    }
}
impl HuffmanTable for NativeTable<'_> {
    fn bits(&self, index: usize) -> u8 {
        // SAFETY: Private construction/source algorithm contract covers this
        // exact reached field; value/padding/unused entries are never read.
        unsafe { (*self.code(index)).bits.assume_init_read() }
    }
    fn value(&self, index: usize) -> u16 {
        unsafe { (*self.code(index)).value.assume_init_read() }
    }
}
pub(crate) unsafe fn byte_at(owner: &Allocation, token: usize, index: usize) -> u8 {
    let offset = token.checked_sub(owner.as_ptr() as usize).unwrap();
    let at = offset.checked_add(index).unwrap();
    assert!(at < owner.len());
    // SAFETY: Same owner, checked source index and caller-proven initialized
    // reached byte. No whole allocation initialized-slice view is constructed.
    unsafe { owner.as_ptr().add(at).read() }
}
pub(crate) unsafe fn pointer_at(owner: &Allocation, token: usize, index: usize) -> usize {
    assert_eq!(token, owner.as_ptr() as usize);
    let at = index.checked_mul(size_of::<usize>()).unwrap();
    assert!(at <= owner.len() && size_of::<usize>() <= owner.len() - at);
    // SAFETY: Native GroupInit puts the initialized pointer-table prefix at
    // this same aligned allocation base; the caller proves this selected slot.
    unsafe { owner.as_ptr().add(at).cast::<usize>().read() }
}
