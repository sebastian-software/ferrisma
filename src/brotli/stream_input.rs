// Copyright 2013 Google Inc. All Rights Reserved.
// Distributed under the MIT license in licenses/BROTLI-MIT.txt.

//! Source-relative loans of the actual native reader over its current input.
//! This fixed metadata retains a caller/internal-buffer pointer, not a payload
//! copy or allocation owner. Native pointer slots are observations resolved
//! against this SAME proven allocation; no integer is dereferenced.

use super::bit_reader::{BitReader, FAST_INPUT_SLACK, SourceReaderState};
use super::layout::BitReaderStorage;
use std::marker::PhantomData;
use std::mem::MaybeUninit;

#[derive(Clone, Copy)]
pub struct SourceInput<'a> {
    pointer: *const u8,
    initialized_len: usize,
    lifetime: PhantomData<&'a [MaybeUninit<u8>]>,
}
impl<'a> SourceInput<'a> {
    pub fn from_slice(input: &'a [u8]) -> Self {
        Self {
            pointer: input.as_ptr(),
            initialized_len: input.len(),
            lifetime: PhantomData,
        }
    }

    /// Retain an external input or the actual initialized internal-buffer
    /// prefix without lending any source bytes yet. NULL/zero stays NULL.
    ///
    /// # Safety
    /// Nonempty input denotes the actual live initialized allocation extent
    /// through `'a`. No conflicting references/mutation may exist during a
    /// reader loan. Caller writes between ended loans are permitted through
    /// the same raw allocation, without a retained shared byte reference.
    /// It must not overlap native reader storage or any simultaneous mutable
    /// field loan. Empty NULL input has no allocation and is never dereferenced.
    pub unsafe fn from_raw_parts(pointer: *const u8, initialized_len: usize) -> Self {
        assert!(initialized_len == 0 || !pointer.is_null());
        assert!(initialized_len <= isize::MAX as usize);
        Self {
            pointer,
            initialized_len,
            lifetime: PhantomData,
        }
    }

    /// A bounds-checked address in the same live source allocation, without
    /// borrowing or reading any byte. NULL/zero remains exactly NULL.
    pub fn pointer_at(self, offset: usize) -> *const u8 {
        assert!(offset <= self.initialized_len);
        if offset == 0 {
            self.pointer
        } else {
            // SAFETY: The constructor's live extent, checked offset, and
            // non-NULL nonempty domain establish actual allocation provenance.
            unsafe { self.pointer.add(offset) }
        }
    }
    fn offset(self, address: usize) -> usize {
        let offset = address
            .checked_sub(self.pointer as usize)
            .expect("same source input anchor");
        assert!(offset <= self.initialized_len);
        offset
    }

    /// Literal BrotliBitReaderSetInput field assignments at its source call
    /// checkpoint. This does not read accumulator or prior pointer fields.
    pub fn set_input(self, storage: &mut BitReaderStorage, start: usize, available: usize) {
        assert!(start <= self.initialized_len && available <= self.initialized_len - start);
        let next = self.pointer_at(start) as usize;
        storage.next_in.write(next);
        storage.last_in.write(if available == 0 {
            next
        } else {
            self.pointer_at(start + available) as usize
        });
        storage.guard_in.write(if available + 1 > FAST_INPUT_SLACK {
            self.pointer_at(start + available + 1 - FAST_INPUT_SLACK) as usize
        } else {
            next
        });
    }

    /// A reached source br->next_in argument, with its actual pointer
    /// provenance. Merely returning this pointer does not borrow its bytes.
    ///
    /// # Safety
    /// SetInput or a previous reader loan initialized next_in for this binding.
    pub unsafe fn next_pointer(self, storage: &BitReaderStorage) -> *const u8 {
        // SAFETY: The caller proves this exact scalar initialized.
        self.pointer_at(self.offset(unsafe { storage.next_in.assume_init() }))
    }

    /// Lend initialized native fields and the current input only for a source
    /// operation; flush all changed fields before returning to a callback site.
    /// The callback must be invoked AFTER this method returns, not inside it.
    ///
    /// # Safety
    /// All five native fields were initialized at actual source checkpoints
    /// and refer to this same binding. The source operation's own initialized
    /// fields/arithmetic/read extents must hold. No external callback or
    /// conflicting access occurs inside the loan. A NULL input cannot advance
    /// or perform pointer arithmetic beyond its zero position.
    pub unsafe fn with_reader<R>(
        self,
        storage: &mut BitReaderStorage,
        operation: impl FnOnce(&mut BitReader<'_>) -> R,
    ) -> R {
        // SAFETY: Reached Init/SetInput/previous operations initialized exactly
        // these values. This creates no reference to any unknown capacity.
        let initial = unsafe {
            SourceReaderState {
                val: storage.val.assume_init(),
                bit_pos: storage.bit_pos.assume_init(),
                next: self.offset(storage.next_in.assume_init()),
                guard: self.offset(storage.guard_in.assume_init()),
                last: self.offset(storage.last_in.assume_init()),
            }
        };
        let fields;
        let result;
        {
            let input = if self.pointer.is_null() {
                &[][..]
            } else {
                // SAFETY: Only the constructor-proven initialized extent is
                // borrowed. This short reference ends before any callback.
                unsafe { std::slice::from_raw_parts(self.pointer, self.initialized_len) }
            };
            let mut reader = BitReader::from_source_state(input, initial);
            result = operation(&mut reader);
            fields = reader.source_state();
        }
        storage.val.write(fields.val);
        storage.bit_pos.write(fields.bit_pos);
        storage.next_in.write(self.pointer_at(fields.next) as usize);
        storage
            .guard_in
            .write(self.pointer_at(fields.guard) as usize);
        storage.last_in.write(self.pointer_at(fields.last) as usize);
        result
    }
}
