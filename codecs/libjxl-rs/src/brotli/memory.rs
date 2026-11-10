// Copyright 2015, 2016 Google Inc. All Rights Reserved.
// Distributed under the MIT license in licenses/BROTLI-MIT.txt.

//! Storage mapping for c/common/platform.c and the allocation macros in
//! c/dec/state.h. An allocation owns the actual callback-returned block; there
//! is no separate Rust payload. Source checkpoints decide when to allocate and
//! release blocks, including observable calls to free with a null address.

use std::alloc::{Layout, alloc, dealloc};
use std::ffi::c_void;
use std::mem::{MaybeUninit, align_of};
use std::ops::Range;
use std::ptr::NonNull;

/// The source malloc-like allocation callback.
pub type AllocFunc = unsafe extern "C" fn(*mut c_void, usize) -> *mut c_void;
/// The source free-like callback, including calls with a null address.
pub type FreeFunc = unsafe extern "C" fn(*mut c_void, *mut c_void);

// All source state, Huffman and dictionary fields have alignment at most this
// on the native targets. Individual typed accesses check their own alignment.
const DEFAULT_ALIGNMENT: usize = align_of::<u128>();

#[derive(Clone, Copy)]
enum Backend {
    Default,
    Callbacks {
        alloc: AllocFunc,
        free: FreeFunc,
        opaque: *mut c_void,
    },
}

/// A matched source allocator pair. This is not an allocation owner.
#[derive(Clone, Copy)]
pub struct Allocator {
    backend: Backend,
}

impl Default for Allocator {
    fn default() -> Self {
        Self {
            backend: Backend::Default,
        }
    }
}

impl Allocator {
    /// Select the default allocator or a complete custom callback pair.
    /// Mixed null/non-null pairs fail without invoking either callback, as in
    /// BrotliDecoderCreateInstance and BrotliSharedDictionaryCreateInstance.
    ///
    /// # Safety
    /// A custom pair and `opaque` must remain callable until every copy of the
    /// allocator and every returned allocation has been discarded. Allocation
    /// must return null or an exclusively owned, writable block of at least the
    /// requested size, aligned for every source type stored there. Its address
    /// must be accepted exactly once by the paired free callback. Free must
    /// also accept null. Callbacks must not unwind, reenter this decoder, or
    /// access live blocks while borrowed by Rust. They may inspect a block when
    /// freeing it only where its bytes are initialized (including bytes the
    /// allocator initialized itself).
    pub unsafe fn from_callbacks(
        alloc: Option<AllocFunc>,
        free: Option<FreeFunc>,
        opaque: *mut c_void,
    ) -> Option<Self> {
        match (alloc, free) {
            (None, None) => Some(Self::default()),
            (Some(alloc), Some(free)) => Some(Self {
                backend: Backend::Callbacks {
                    alloc,
                    free,
                    opaque,
                },
            }),
            _ => None,
        }
    }

    /// Request exactly `size` bytes from the selected allocator. The block is
    /// not initialized here: source field writes and memset checkpoints alone
    /// determine which bytes become initialized.
    pub fn allocate(self, size: usize) -> Option<Allocation> {
        let ptr = match self.backend {
            Backend::Default => {
                // Rust allocation requires a nonzero extent; source malloc(0)
                // permits either null or a freeable zero-length result. The
                // one-byte reservation is never exposed as payload capacity.
                let layout = Layout::from_size_align(size.max(1), DEFAULT_ALIGNMENT).ok()?;
                // SAFETY: The layout is valid and nonzero. Allocation below
                // uniquely owns the returned block and retains this same size
                // and allocator for its matching deallocation.
                unsafe { alloc(layout) }
            }
            Backend::Callbacks { alloc, opaque, .. } => {
                // SAFETY: from_callbacks establishes the callback lifetime,
                // matching pair, ownership, alignment and no-unwind contract.
                // The original requested size is passed without substitution.
                unsafe { alloc(opaque, size).cast() }
            }
        };
        Some(Allocation {
            ptr: NonNull::new(ptr)?,
            size,
            allocator: self,
        })
    }

    /// Execute one original free checkpoint, including its null case.
    /// Callers use this explicitly in source order rather than relying on
    /// reverse field destruction to reproduce the source cleanup sequence.
    pub fn release(self, allocation: Option<Allocation>) {
        match allocation {
            Some(allocation) => {
                // An Allocation retains its actual producing allocator. A
                // source checkpoint must never change that owner by rebinding
                // a current callback pair to an older returned block.
                drop(allocation);
            }
            None => self.free_null(),
        }
    }

    fn free_null(self) {
        if let Backend::Callbacks { free, opaque, .. } = self.backend {
            // SAFETY: Null acceptance, callback lifetime and no-unwind are
            // part of from_callbacks. No allocation is freed by this call.
            unsafe { free(opaque, std::ptr::null_mut()) };
        }
    }
}

/// Exactly one source allocation. Moving this value transfers ownership;
/// views borrow the same returned block and never clone its bytes.
pub struct Allocation {
    ptr: NonNull<u8>,
    size: usize,
    allocator: Allocator,
}

impl Allocation {
    pub fn len(&self) -> usize {
        self.size
    }
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Identity used for source-relative offsets and read-only observation.
    /// The address must not be dereferenced independently of this owner.
    pub fn as_ptr(&self) -> *const u8 {
        self.ptr.as_ptr()
    }

    pub fn storage_mut(&mut self) -> &mut [MaybeUninit<u8>] {
        // SAFETY: The callback contract/global allocator provides `size`
        // writable bytes, with a non-null address even for an empty block.
        // MaybeUninit permits bytes not initialized by the original source.
        // Exclusive owner borrowing prevents simultaneous storage aliases.
        unsafe { std::slice::from_raw_parts_mut(self.ptr.as_ptr().cast(), self.size) }
    }

    /// The source memset checkpoint; does not initialize unrelated blocks.
    pub fn fill(&mut self, value: u8) {
        for byte in self.storage_mut() {
            byte.write(value);
        }
    }

    /// Borrow a source-defined initialized range.
    ///
    /// # Safety
    /// Every byte in `range` must have been initialized by the allocator or a
    /// reached source write. Source capacity or a planned future write is not
    /// sufficient. No external callback may mutate it during this borrow.
    pub unsafe fn initialized_bytes(&self, range: Range<usize>) -> &[u8] {
        assert!(range.start <= range.end && range.end <= self.size);
        // SAFETY: The caller proves initialization; bounds were checked above.
        // The returned slice is tied to the live shared owner borrow. add(0)
        // is valid for the zero-length allocation result as well.
        unsafe { std::slice::from_raw_parts(self.ptr.as_ptr().add(range.start), range.len()) }
    }
}

impl Drop for Allocation {
    fn drop(&mut self) {
        match self.allocator.backend {
            Backend::Default => {
                let layout = Layout::from_size_align(self.size.max(1), DEFAULT_ALIGNMENT)
                    .expect("the original allocation layout remains valid");
                // SAFETY: This is the unique owner returned by alloc with
                // exactly this layout. All views have ended before Drop.
                unsafe { dealloc(self.ptr.as_ptr(), layout) };
            }
            Backend::Callbacks { free, opaque, .. } => {
                // SAFETY: The exact producing pair and address are retained;
                // ownership is unique, borrows have ended, and the constructor
                // contract guarantees callback validity and no unwinding.
                unsafe { free(opaque, self.ptr.as_ptr().cast()) };
            }
        }
    }
}
