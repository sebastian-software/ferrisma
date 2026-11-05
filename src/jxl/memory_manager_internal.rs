// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! AlignedMemory, BytesPerRow and MemoryManagerUniquePtr (with
//! MemoryManagerDeleteHelper and MemoryManagerMakeUniquePrivate) from
//! memory_manager_internal.{h,cc}.
//! An owner allocates through the caller's JxlMemoryManager when one is
//! supplied, or through the crate's native Rust allocator boundary otherwise;
//! both zero-initialize the payload so safe slices never expose indeterminate
//! bytes. Row sizing retains the scalar reference's four byte maximum vector
//! size. See ADR 0003.

use super::base::status::{StatusCode, StatusOr};
use super::memory_manager::{JxlMemoryManager, memory_manager_alloc, memory_manager_free};
use std::alloc::{Layout, alloc_zeroed, dealloc};
use std::ptr::NonNull;
use std::sync::atomic::{AtomicU32, Ordering};

pub(crate) const ALIGNMENT: usize = 128;
pub(crate) const NUM_ALIGNMENT_GROUPS: usize = 16;
pub(crate) const ALIAS: usize = NUM_ALIGNMENT_GROUPS * ALIGNMENT;

// Both storage representations below implement the same original static
// AlignedMemory constructor counter; custom allocations must not fork it.
static NEXT_GROUP: AtomicU32 = AtomicU32::new(0);
pub(crate) fn next_alignment_offset() -> usize {
    let group = NEXT_GROUP.fetch_add(1, Ordering::Relaxed) as usize;
    ALIGNMENT * (group & (NUM_ALIGNMENT_GROUPS - 1))
}

#[expect(
    clippy::manual_is_multiple_of,
    reason = "Mirror the source modulo test."
)]
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

/// The source `JxlMemoryManager*` an AlignedMemory owner allocates through.
///
/// `NULL` is the source's null pointer, which fails at its allocation
/// checkpoint. `NATIVE` is the crate's Rust allocator boundary, used by owners
/// that no caller manager reaches (the encoder and standalone units). A value
/// from `from_raw` retains the caller's live manager, whose callbacks receive
/// the original request sizes, as in the source. The representation is
/// private, so a manager pointer can only enter through `from_raw`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MemoryManagerPtr(Kind);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum Kind {
    #[default]
    Null,
    Native,
    Manager(NonNull<JxlMemoryManager>),
}

// SAFETY: A `Manager` value is created only by the unsafe `from_raw`, whose
// caller guarantees that the manager and its callbacks stay valid and callable
// from every thread that uses an owner allocated through it, as the source's
// parallel decoding requires of the same pointer. The other kinds hold no
// pointer.
unsafe impl Send for MemoryManagerPtr {}
// SAFETY: As for Send; the pointer is only read, never written through.
unsafe impl Sync for MemoryManagerPtr {}

impl MemoryManagerPtr {
    /// The source's null manager pointer.
    pub const NULL: Self = Self(Kind::Null);
    /// The crate's native Rust allocator.
    pub const NATIVE: Self = Self(Kind::Native);

    /// The caller's manager, or `NULL` for a null pointer.
    ///
    /// # Safety
    /// A nonnull `manager` must remain readable and initialized, with valid
    /// alloc/free/opaque fields, until every owner allocated through the
    /// returned value is destroyed. Its callbacks must obey memory_manager.h,
    /// be callable from any thread that uses those owners, never unwind and
    /// never reenter the owner being allocated or freed. A successful alloc
    /// returns an exclusively owned writable block of the requested size.
    pub unsafe fn from_raw(manager: *const JxlMemoryManager) -> Self {
        Self(NonNull::new(manager.cast_mut()).map_or(Kind::Null, Kind::Manager))
    }

    /// The source null test, `memory_manager != nullptr`.
    pub fn is_some(self) -> bool {
        self != Self::NULL
    }

    /// The raw source pointer: null for `NULL` and `NATIVE`.
    pub fn as_raw(self) -> *const JxlMemoryManager {
        match self.0 {
            Kind::Manager(manager) => manager.as_ptr(),
            Kind::Null | Kind::Native => std::ptr::null(),
        }
    }
}

enum Backing {
    Native(Layout),
    Manager(NonNull<JxlMemoryManager>),
}

pub(crate) struct AlignedMemory {
    allocation: NonNull<u8>,
    backing: Backing,
    address_offset: usize,
    size: usize,
}

// SAFETY: This value exclusively owns its allocation, which can be freed on
// another thread: the global allocator is thread-safe, and MemoryManagerPtr's
// from_raw contract makes a caller manager's free callable from any thread.
// Moving it transfers that ownership exactly once.
unsafe impl Send for AlignedMemory {}

// SAFETY: Shared access exposes only initialized immutable bytes; mutation
// requires &mut self. The allocators and group counter are thread-safe.
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

    pub(crate) fn usizes(&self) -> &[usize] {
        let size = std::mem::size_of::<usize>();
        assert_eq!(self.size % size, 0);
        // SAFETY: create aligns to 128 bytes, all usize bit patterns are
        // valid, and the checked byte count covers whole initialized elements.
        unsafe { std::slice::from_raw_parts(self.bytes().as_ptr().cast(), self.size / size) }
    }

    pub(crate) fn usizes_mut(&mut self) -> &mut [usize] {
        let size = std::mem::size_of::<usize>();
        assert_eq!(self.size % size, 0);
        let len = self.size / size;
        // SAFETY: Same validity/alignment/bounds proof as usizes, with
        // exclusive access through the owner for the returned slice lifetime.
        unsafe { std::slice::from_raw_parts_mut(self.bytes_mut().as_mut_ptr().cast(), len) }
    }

    pub(crate) fn isizes(&self) -> &[isize] {
        let size = std::mem::size_of::<isize>();
        assert_eq!(self.size % size, 0);
        // SAFETY: create aligns to 128 bytes, all isize bit patterns are
        // valid, and the checked byte count covers whole initialized elements.
        unsafe { std::slice::from_raw_parts(self.bytes().as_ptr().cast(), self.size / size) }
    }

    pub(crate) fn isizes_mut(&mut self) -> &mut [isize] {
        let size = std::mem::size_of::<isize>();
        assert_eq!(self.size % size, 0);
        let len = self.size / size;
        // SAFETY: Same validity/alignment/bounds proof as isizes, with
        // exclusive access through the owner for the returned slice lifetime.
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

    pub(crate) fn f32s_mut(&mut self) -> &mut [f32] {
        assert_eq!(self.size % 4, 0);
        let len = self.size / 4;
        // SAFETY: create aligns to 128 bytes, all f32 bit patterns are valid,
        // the checked byte count covers whole initialized elements, and the
        // owner gives exclusive access for the entire returned slice lifetime.
        unsafe { std::slice::from_raw_parts_mut(self.bytes_mut().as_mut_ptr().cast(), len) }
    }

    pub(crate) fn create(
        memory_manager: MemoryManagerPtr,
        size: usize,
        pre_padding: usize,
    ) -> StatusOr<Self> {
        if pre_padding > ALIAS {
            return Err(StatusCode::GenericError);
        }
        let allocation_size = size
            .checked_add(pre_padding)
            .and_then(|n| n.checked_add(ALIAS))
            .ok_or(StatusCode::GenericError)?;
        let (allocation, backing) = match memory_manager.0 {
            Kind::Null => return Err(StatusCode::GenericError),
            Kind::Native => {
                let layout = Layout::from_size_align(allocation_size, 1)
                    .map_err(|_| StatusCode::GenericError)?;
                // SAFETY: The checked layout has nonzero size (at least ALIAS)
                // and a valid alignment. The returned allocation is owned by
                // this value and is deallocated with this exact layout in
                // Drop. All bytes start valid.
                let allocation = NonNull::new(unsafe { alloc_zeroed(layout) })
                    .ok_or(StatusCode::GenericError)?;
                (allocation, Backing::Native(layout))
            }
            Kind::Manager(manager) => {
                // SAFETY: from_raw's caller keeps this manager live with valid
                // callbacks. No borrowed view of any block exists across the
                // call, and the request is the source's exact size.
                let allocated = unsafe { memory_manager_alloc(manager.as_ptr(), allocation_size) };
                let allocation =
                    NonNull::new(allocated.cast::<u8>()).ok_or(StatusCode::GenericError)?;
                // SAFETY: A successful alloc returns an exclusively owned
                // writable block of allocation_size bytes (from_raw contract).
                // Zeroing it is the representation's internal initialization
                // so that the safe slices below never read indeterminate bytes.
                unsafe { allocation.as_ptr().write_bytes(0, allocation_size) };
                (allocation, Backing::Manager(manager))
            }
        };

        // Relaxed ordering is sufficient: only a rotating numeric group is
        // shared, never the allocation or its contents, as in the source.
        let offset = next_alignment_offset();
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
            backing,
            address_offset,
            size,
        })
    }

    /// The source `memory_manager()` accessor of an allocated owner.
    pub(crate) fn memory_manager(&self) -> MemoryManagerPtr {
        match self.backing {
            Backing::Native(_) => MemoryManagerPtr::NATIVE,
            Backing::Manager(manager) => MemoryManagerPtr(Kind::Manager(manager)),
        }
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
        match self.backing {
            // SAFETY: allocation was produced by alloc_zeroed with this
            // layout, is owned exactly once, and has not been deallocated.
            Backing::Native(layout) => unsafe { dealloc(self.allocation.as_ptr(), layout) },
            // SAFETY: The block came from this live manager's alloc and is
            // released exactly once, at the source destructor checkpoint; no
            // slice into it outlives this owner.
            Backing::Manager(manager) => unsafe {
                memory_manager_free(manager.as_ptr(), self.allocation.as_ptr().cast());
            },
        }
    }
}

/// Original callback-backed AlignedMemory, retaining the live manager pointer.
/// The callback's actual block backs the payload; unwritten bytes remain
/// indeterminate. This owner exposes pointers rather than initialized slices.
pub(crate) struct CallbackAlignedMemory {
    allocation: NonNull<u8>,
    memory_manager: *const super::memory_manager::JxlMemoryManager,
    address: NonNull<u8>,
    size: usize,
}
impl CallbackAlignedMemory {
    /// # Safety
    /// A nonnull manager must remain live through this owner's destruction.
    /// Its reached callbacks/opaque values must be initialized and valid, and
    /// successful alloc must return an exclusively owned block of the exact
    /// requested size, compatible with its later free callback. User callbacks
    /// must not unwind or invalidate a manager/block still used by this call.
    pub(crate) unsafe fn create(
        manager: *const super::memory_manager::JxlMemoryManager,
        size: usize,
        pre_padding: usize,
    ) -> StatusOr<Self> {
        if pre_padding > ALIAS {
            return Err(StatusCode::GenericError);
        }
        let allocation_size = size
            .checked_add(pre_padding)
            .and_then(|n| n.checked_add(ALIAS))
            .ok_or(StatusCode::GenericError)?;
        if manager.is_null() {
            return Err(StatusCode::GenericError);
        }
        // SAFETY: The caller establishes the original manager callback
        // protocol. No borrowed view of the block exists across this call.
        let allocated =
            unsafe { super::memory_manager::memory_manager_alloc(manager, allocation_size) };
        let allocation = NonNull::new(allocated.cast::<u8>()).ok_or(StatusCode::GenericError)?;
        // Source increments this shared counter only after successful alloc.
        let offset = next_alignment_offset();
        let address = (allocation.as_ptr() as usize).wrapping_add(pre_padding);
        let mut aligned_address = (address & !(ALIAS - 1)).wrapping_add(offset);
        if aligned_address < address {
            aligned_address = aligned_address.wrapping_add(ALIAS);
        }
        let address_offset = aligned_address.wrapping_sub(allocation.as_ptr() as usize);
        assert!(
            address_offset >= pre_padding && address_offset <= allocation_size - size,
            "source allocator must return the requested live object extent"
        );
        // SAFETY: SafeAdd and the source alignment calculation place this
        // pointer within the actual callback block, retaining its provenance.
        let address = unsafe { NonNull::new_unchecked(allocation.as_ptr().add(address_offset)) };
        Ok(Self {
            allocation,
            memory_manager: manager,
            address,
            size,
        })
    }

    pub(crate) fn address(&self) -> *mut u8 {
        self.address.as_ptr()
    }
    pub(crate) fn size(&self) -> usize {
        self.size
    }
}
impl Drop for CallbackAlignedMemory {
    fn drop(&mut self) {
        // SAFETY: create retains this still-live source manager pointer and
        // unique callback allocation. Read its current free/opaque fields at
        // the original destruction checkpoint, then release exactly once.
        unsafe {
            super::memory_manager::memory_manager_free(
                self.memory_manager,
                self.allocation.as_ptr().cast(),
            );
        }
    }
}

/// MemoryManagerUniquePtr with MemoryManagerDeleteHelper, created by
/// MemoryManagerMakeUniquePrivate (JXL_MEMORY_MANAGER_MAKE_UNIQUE_OR_RETURN).
///
/// The object is allocated through the manager's own `alloc` callback with
/// the size of the Rust layout, constructed in place, and on drop destroyed
/// and released through the manager's `free` callback, as the source's
/// deleter does. The retained manager pointer is the source's
/// `memory_manager_` of the deleter.
pub(crate) struct MemoryManagerUniquePtr<T> {
    object: NonNull<T>,
    memory_manager: *const JxlMemoryManager,
}

impl<T> MemoryManagerUniquePtr<T> {
    /// MemoryManagerMakeUniquePrivate: `None` when the allocation fails; the
    /// value is then dropped, as the source's argument temporaries are.
    ///
    /// # Safety
    /// `memory_manager` is readable and initialized, with alloc/free callbacks
    /// that obey memory_manager.h and stay valid until the returned owner is
    /// dropped; it is not changed while the owner lives. A block returned by
    /// alloc is writable for the requested size and aligned for `T`, as the
    /// source's placement new requires.
    pub(crate) unsafe fn make(memory_manager: *const JxlMemoryManager, value: T) -> Option<Self> {
        // SAFETY: The caller supplies the initialized manager and callbacks.
        // The source calls its alloc field directly, as here.
        let mem = unsafe {
            let allocate = (*memory_manager)
                .alloc
                .expect("source requires an allocation callback");
            allocate((*memory_manager).opaque, std::mem::size_of::<T>())
        }
        .cast::<T>();
        let object = NonNull::new(mem)?;
        assert!(
            mem.is_aligned(),
            "the source's placement new requires an aligned allocation"
        );
        // SAFETY: The callback returned a writable block of the requested
        // size, checked aligned above.
        unsafe { mem.write(value) };
        Some(Self {
            object,
            memory_manager,
        })
    }

    /// The owned object's address, as the source's `get()`.
    pub(crate) fn as_ptr(&self) -> *mut T {
        self.object.as_ptr()
    }
}

impl<T> std::ops::Deref for MemoryManagerUniquePtr<T> {
    type Target = T;
    fn deref(&self) -> &T {
        // SAFETY: The object is initialized and exclusively owned by this
        // value; shared access only.
        unsafe { self.object.as_ref() }
    }
}

impl<T> std::ops::DerefMut for MemoryManagerUniquePtr<T> {
    fn deref_mut(&mut self) -> &mut T {
        // SAFETY: Exclusive access through the unique owner.
        unsafe { self.object.as_mut() }
    }
}

impl<T> Drop for MemoryManagerUniquePtr<T> {
    /// MemoryManagerDeleteHelper: destroy, then free through the manager.
    fn drop(&mut self) {
        // SAFETY: The object is initialized and owned exactly once; make's
        // caller keeps the manager and its free callback valid until now.
        unsafe {
            std::ptr::drop_in_place(self.object.as_ptr());
            let release = (*self.memory_manager)
                .free
                .expect("source requires a free callback");
            release((*self.memory_manager).opaque, self.object.as_ptr().cast());
        }
    }
}
