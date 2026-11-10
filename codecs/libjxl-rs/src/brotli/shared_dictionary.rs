// Copyright 2017 Google Inc. All Rights Reserved.
// Distributed under the MIT license in licenses/BROTLI-MIT.txt.

//! Pinned default-configuration c/common/shared_dictionary.c ownership.
//! The actual allocator block contains the complete native source layout.
//! Raw prefix data remains borrowed from its caller; no dictionary payload
//! is cloned. BROTLI_EXPERIMENTAL serialized dictionaries are not compiled
//! in the pinned reference configuration and are not admitted here.

use super::layout::SharedDictionaryStorage;
use super::memory::{Allocation, Allocator};
use super::static_descriptors::{DICTIONARY, TRANSFORMS};
use std::marker::PhantomData;
use std::mem::{MaybeUninit, align_of, size_of};
use std::ops::Range;

pub const BROTLI_SHARED_DICTIONARY_RAW: u32 = 0;
pub const BROTLI_SHARED_DICTIONARY_SERIALIZED: u32 = 1;
pub const SHARED_BROTLI_MAX_COMPOUND_DICTS: usize = 15;

/// A caller-owned pointer/length borrow, not an allocation owner or initialized
/// slice. The source accepts attachment before caller initialization and reads
/// the same live allocation after later caller writes between decoder calls.
#[derive(Clone, Copy)]
pub struct RawDictionaryView<'a> {
    pointer: *const u8,
    len: usize,
    lifetime: PhantomData<&'a [MaybeUninit<u8>]>,
}

impl<'a> RawDictionaryView<'a> {
    pub fn from_slice(bytes: &'a [u8]) -> Self {
        Self {
            pointer: bytes.as_ptr(),
            len: bytes.len(),
            lifetime: PhantomData,
        }
    }

    pub fn null() -> Self {
        Self {
            pointer: std::ptr::null(),
            len: 0,
            lifetime: PhantomData,
        }
    }

    /// Retain the actual source caller allocation without creating a slice.
    ///
    /// # Safety
    /// This retains a source pointer value without dereferencing it. Even NULL
    /// with nonzero length is accepted by source attachment, which only stores
    /// the fields; any later reached copy/read requires a live valid extent
    /// with this pointer's provenance throughout `'a`. The caller may
    /// initialize/mutate bytes between decoder calls, but never concurrently
    /// with a source access or while a conflicting Rust reference exists.
    /// Bytes need not be initialized at attachment: each later scalar/slice
    /// read separately requires initialization of its reached range. A retained
    /// NULL is never replaced by a fabricated non-NULL empty slice pointer.
    pub unsafe fn from_raw_parts(pointer: *const u8, len: usize) -> Self {
        Self {
            pointer,
            len,
            lifetime: PhantomData,
        }
    }

    pub fn as_ptr(self) -> *const u8 {
        self.pointer
    }
    pub fn len(self) -> usize {
        self.len
    }
    pub fn is_empty(self) -> bool {
        self.len == 0
    }

    /// The source memcpy representation copy; unknown bytes remain unknown.
    ///
    /// # Safety
    /// Source/destination are live nonoverlapping extents with no conflicting
    /// references or concurrent accesses. Source pointer arithmetic/memcpy is
    /// defined (in particular a NULL source is not used even for a zero copy).
    /// No byte is interpreted as an initialized Rust integer here.
    pub unsafe fn copy_to(self, offset: usize, destination: &mut [MaybeUninit<u8>]) {
        assert!(offset <= self.len && destination.len() <= self.len - offset);
        assert!(!self.pointer.is_null());
        // SAFETY: Checked source extent and caller-proven liveness/provenance,
        // exclusivity/nonoverlap. MaybeUninit preserves object representation
        // bytes without reading an unknown u8 or constructing &[u8].
        unsafe {
            std::ptr::copy_nonoverlapping(
                self.pointer.add(offset).cast::<MaybeUninit<u8>>(),
                destination.as_mut_ptr(),
                destination.len(),
            )
        };
    }

    /// Borrow only a reached initialized source range, for its local lifetime.
    ///
    /// # Safety
    /// Every selected byte is initialized, the pointer is non-NULL even for an
    /// empty range, and no caller mutation/conflicting access occurs while the
    /// returned reference lives. Source capacity alone is insufficient.
    pub unsafe fn initialized_bytes(&self, range: Range<usize>) -> &[u8] {
        assert!(range.start <= range.end && range.end <= self.len);
        assert!(!self.pointer.is_null());
        // SAFETY: Checked bounds plus the caller's range initialization and
        // reference/lifetime proof; the returned borrow never extends `'a`.
        unsafe { std::slice::from_raw_parts(self.pointer.add(range.start), range.len()) }
    }
}

/// Ownership sidecar for the exact source allocation. Pointer-sized native
/// fields refer to these same live owners or immutable source-shaped statics.
pub struct SharedDictionary<'a> {
    allocation: Option<Allocation>,
    allocator: Allocator,
    prefixes: [Option<&'a [u8]>; SHARED_BROTLI_MAX_COMPOUND_DICTS],
    raw_prefixes: [Option<RawDictionaryView<'a>>; SHARED_BROTLI_MAX_COMPOUND_DICTS],
}

impl<'a> SharedDictionary<'a> {
    /// The decoder's allocator pair has already passed source validation.
    /// Addresses are the actual selected functions/opaque context; native
    /// pointer fields are observations, never reconstructed allocation owners.
    pub fn create(
        allocator: Allocator,
        alloc_address: usize,
        free_address: usize,
        opaque: usize,
    ) -> Option<Self> {
        let mut allocation = allocator.allocate(size_of::<SharedDictionaryStorage>())?;
        assert_eq!(
            allocation.as_ptr() as usize % align_of::<SharedDictionaryStorage>(),
            0
        );
        // This exact whole-object memset is present in the original; the
        // outer decoder state has no corresponding blanket initialization.
        allocation.fill(0);
        let mut result = Self {
            allocation: Some(allocation),
            allocator,
            prefixes: [None; SHARED_BROTLI_MAX_COMPOUND_DICTS],
            raw_prefixes: [None; SHARED_BROTLI_MAX_COMPOUND_DICTS],
        };
        result.storage_mut().context_based.write(0);
        result.storage_mut().num_dictionaries.write(1);
        result.storage_mut().num_word_lists.write(0);
        result.storage_mut().num_transform_lists.write(0);
        // SAFETY: The whole source object was zero-initialized and these
        // arrays have valid aligned native storage. Each write retains all
        // other entries and targets the real allocation rather than a copy.
        unsafe {
            result.storage_mut().words.assume_init_mut()[0] =
                (&DICTIONARY as *const _ as *const ()) as usize;
            result.storage_mut().transforms.assume_init_mut()[0] =
                (&TRANSFORMS as *const _ as *const ()) as usize;
        }
        result.storage_mut().alloc_func.write(alloc_address);
        result.storage_mut().free_func.write(free_address);
        result.storage_mut().memory_manager_opaque.write(opaque);
        Some(result)
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.allocation
            .as_ref()
            .expect("live shared owner")
            .as_ptr()
    }

    /// The default source accepts raw dictionaries only. `None` maps the
    /// defined null, zero-size raw prefix; `Some` preserves even an empty
    /// slice's actual pointer. Both are retained for the decoder's lifetime.
    pub fn attach(&mut self, dictionary_type: u32, data: Option<&'a [u8]>) -> bool {
        if dictionary_type == BROTLI_SHARED_DICTIONARY_RAW {
            // SAFETY: memset and each attach initialize num_prefix.
            let index = unsafe { self.storage().num_prefix.assume_init() } as usize;
            if index >= SHARED_BROTLI_MAX_COMPOUND_DICTS {
                return false;
            }
            let (size, address) = match data {
                Some(bytes) => (bytes.len(), bytes.as_ptr() as usize),
                None => (0, 0),
            };
            // SAFETY: The original whole-object memset initialized these
            // arrays. Bounds follow from the original maximum-prefix test.
            unsafe {
                self.storage_mut().prefix_size.assume_init_mut()[index] = size;
                self.storage_mut().prefix.assume_init_mut()[index] = address;
            }
            self.prefixes[index] = data;
            self.storage_mut().num_prefix.write(index as u32 + 1);
            return true;
        }
        false
    }

    /// Legacy immutable attachment binding. Raw attachments have no persistent
    /// initialized slice; use prefix_view for their actual pointer/length.
    pub fn prefix(&self, index: usize) -> Option<&'a [u8]> {
        // SAFETY: Source memset/attach initialize the prefix count.
        assert!(index < unsafe { self.storage().num_prefix.assume_init() } as usize);
        self.prefixes[index]
    }

    /// Default source Raw attach, retaining a mutable/late-initialized caller
    /// allocation without materializing any byte reference.
    ///
    /// # Safety
    /// The view's allocation/provenance/lifetime/no-conflicting-access contract
    /// remains valid until every decoder retaining this dictionary is destroyed.
    pub unsafe fn attach_raw(&mut self, dictionary_type: u32, data: RawDictionaryView<'a>) -> bool {
        if dictionary_type == BROTLI_SHARED_DICTIONARY_RAW {
            // SAFETY: Source memset/each attach initializes this count.
            let index = unsafe { self.storage().num_prefix.assume_init() } as usize;
            if index >= SHARED_BROTLI_MAX_COMPOUND_DICTS {
                return false;
            }
            // SAFETY: The source memset initialized these arrays. Only the
            // selected in-bounds size/pointer slots are changed, in source order.
            unsafe {
                self.storage_mut().prefix_size.assume_init_mut()[index] = data.len();
                self.storage_mut().prefix.assume_init_mut()[index] = data.as_ptr() as usize;
            }
            self.raw_prefixes[index] = Some(data);
            self.storage_mut().num_prefix.write(index as u32 + 1);
            return true;
        }
        false
    }

    pub fn prefix_view(&self, index: usize) -> RawDictionaryView<'a> {
        // SAFETY: Source memset/attach initialize the actual prefix count.
        assert!(index < unsafe { self.storage().num_prefix.assume_init() } as usize);
        if let Some(view) = self.raw_prefixes[index] {
            return view;
        }
        self.prefixes[index].map_or_else(RawDictionaryView::null, RawDictionaryView::from_slice)
    }

    pub fn storage(&self) -> &SharedDictionaryStorage {
        let owner = self.allocation.as_ref().expect("live shared owner");
        assert_eq!(owner.len(), size_of::<SharedDictionaryStorage>());
        assert_eq!(
            owner.as_ptr() as usize % align_of::<SharedDictionaryStorage>(),
            0
        );
        // SAFETY: The actual block has the checked native size/alignment and
        // every storage field is MaybeUninit, accepting unknown bit patterns.
        // Here source memset additionally initialized all bytes. The shared
        // reference cannot outlive the allocation or coexist with mutation.
        unsafe { &*owner.as_ptr().cast::<SharedDictionaryStorage>() }
    }

    fn storage_mut(&mut self) -> &mut SharedDictionaryStorage {
        let owner = self.allocation.as_mut().expect("live shared owner");
        assert_eq!(owner.len(), size_of::<SharedDictionaryStorage>());
        let storage = owner.storage_mut();
        assert_eq!(
            storage.as_ptr() as usize % align_of::<SharedDictionaryStorage>(),
            0
        );
        // SAFETY: Checked native extent/alignment, all fields MaybeUninit;
        // the exclusive actual-owner borrow prevents any competing alias.
        unsafe { &mut *storage.as_mut_ptr().cast::<SharedDictionaryStorage>() }
    }
}

impl Drop for SharedDictionary<'_> {
    fn drop(&mut self) {
        // The default graph never allocates these experimental arrays. The
        // source still invokes each free(NULL), in exactly this order.
        self.allocator.release(None); // words_instances
        self.allocator.release(None); // transforms_instances
        self.allocator.release(None); // prefix_suffix_maps
        self.allocator.release(self.allocation.take());
    }
}
