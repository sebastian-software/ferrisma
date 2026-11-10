// Copyright 2013 Google Inc. All Rights Reserved.
// Distributed under the MIT license in licenses/BROTLI-MIT.txt.

//! c/dec/decode.c raw compound-dictionary attachment, indexing and copy.
//! Native descriptor fields live in the actual callback-returned allocation.
//! The fixed sidecar retains caller-backed views only; it is not a dictionary
//! payload or second allocation owner. Shared and addon prefix indices can
//! differ after a failed addon allocation, exactly as in the original.

use super::layout::CompoundDictionaryStorage;
use super::running::BROTLI_STATE_UNINITED;
use super::shared_dictionary::RawDictionaryView;
use super::state::StateOwner;
use std::mem::{align_of, size_of};

#[derive(Default)]
pub struct CompoundDictionaryBindings<'a> {
    chunks: [Option<RawDictionaryView<'a>>; 15],
}

macro_rules! state_read {
    ($owner:expr, $field:ident) => {{
        // SAFETY: Each enclosing internal helper requires the original
        // initialization history for exactly its reached source field reads.
        unsafe { $owner.storage().$field.assume_init() }
    }};
}
macro_rules! addon_read {
    ($owner:expr, $field:ident) => {{
        // SAFETY: Same reached source initialization requirement. The loan
        // ends before any allocator/free callback or payload operation.
        unsafe { $owner.compound_storage().$field.assume_init() }
    }};
}
macro_rules! addon_write {
    ($owner:expr, $field:ident, $value:expr) => {{
        let value = $value;
        $owner.compound_storage_mut().$field.write(value);
    }};
}

impl<'a> StateOwner<'a> {
    fn compound_storage(&self) -> &CompoundDictionaryStorage {
        let owner = self
            .compound_dictionary
            .as_ref()
            .expect("source live addon allocation");
        assert_eq!(owner.len(), size_of::<CompoundDictionaryStorage>());
        assert_eq!(
            owner.as_ptr() as usize % align_of::<CompoundDictionaryStorage>(),
            0
        );
        // SAFETY: Actual live callback-owned native extent/alignment. Every
        // field is MaybeUninit, so making this view reads no unknown field.
        unsafe { &*owner.as_ptr().cast::<CompoundDictionaryStorage>() }
    }
    fn compound_storage_mut(&mut self) -> &mut CompoundDictionaryStorage {
        let owner = self
            .compound_dictionary
            .as_mut()
            .expect("source live addon allocation");
        assert_eq!(owner.len(), size_of::<CompoundDictionaryStorage>());
        let storage = owner.storage_mut();
        assert_eq!(
            storage.as_ptr() as usize % align_of::<CompoundDictionaryStorage>(),
            0
        );
        // SAFETY: Exact exclusive native owner loan, all-MaybeUninit fields.
        unsafe { &mut *storage.as_mut_ptr().cast::<CompoundDictionaryStorage>() }
    }
    unsafe fn chunk_offset(&self, index: usize) -> i32 {
        assert!(index < 16);
        // SAFETY: The caller proves this particular reached slot initialized;
        // no whole array reference or unknown sibling scalar is constructed.
        unsafe {
            self.compound_storage()
                .chunk_offsets
                .as_ptr()
                .cast::<i32>()
                .add(index)
                .read()
        }
    }
    fn write_chunk_offset(&mut self, index: usize, value: i32) {
        assert!(index < 16);
        // SAFETY: Checked selected native element, exclusive actual storage.
        unsafe {
            self.compound_storage_mut()
                .chunk_offsets
                .as_mut_ptr()
                .cast::<i32>()
                .add(index)
                .write(value)
        };
    }
    unsafe fn chunk_token(&self, index: usize) -> usize {
        assert!(index < 16);
        // SAFETY: Only the original reached initialized pointer slot is read.
        unsafe {
            self.compound_storage()
                .chunks
                .as_ptr()
                .cast::<usize>()
                .add(index)
                .read()
        }
    }
    fn write_chunk_token(&mut self, index: usize, value: usize) {
        assert!(index < 16);
        // SAFETY: Checked selected native pointer slot, no payload read.
        unsafe {
            self.compound_storage_mut()
                .chunks
                .as_mut_ptr()
                .cast::<usize>()
                .add(index)
                .write(value)
        };
    }
    unsafe fn block_map(&self, index: usize) -> u8 {
        assert!(index < 256);
        // SAFETY: EnsureCompoundDictionaryInitialized wrote this reached map
        // entry; the caller does not infer initialization for its unused tail.
        unsafe {
            self.compound_storage()
                .block_map
                .as_ptr()
                .cast::<u8>()
                .add(index)
                .read()
        }
    }
    fn write_block_map(&mut self, index: usize, value: u8) {
        assert!(index < 256);
        // SAFETY: Checked selected byte in the real addon allocation.
        unsafe {
            self.compound_storage_mut()
                .block_map
                .as_mut_ptr()
                .cast::<u8>()
                .add(index)
                .write(value)
        };
    }

    /// AttachCompoundDictionary. Only actual source addon-chunk writes update
    /// the corresponding borrowed sidecar; failed shared-prefix attachments
    /// are not retroactively inserted into the addon on a later call.
    ///
    /// # Safety
    /// Bindings belongs to this same live state and retains every previous
    /// addon write. Reached fields and signed arithmetic have their source
    /// initialization/defined domains; prefix bytes remain live and stable.
    pub unsafe fn attach_compound_dictionary(
        &mut self,
        bindings: &mut CompoundDictionaryBindings<'a>,
        data: Option<&'a [u8]>,
    ) -> bool {
        let view = data.map_or_else(RawDictionaryView::null, RawDictionaryView::from_slice);
        // SAFETY: The immutable borrowed slice is a valid raw caller view; all
        // same-state field and arithmetic requirements remain the caller's.
        unsafe { self.attach_compound_dictionary_raw(bindings, view) }
    }

    /// The same source helper with the actual mutable/late-initialized caller
    /// allocation retained as pointer/length, without an initialized slice.
    ///
    /// # Safety
    /// Same state/field/arithmetic requirements as the immutable wrapper. The
    /// raw view's lifetime/access contract holds through every reached copy.
    pub unsafe fn attach_compound_dictionary_raw(
        &mut self,
        bindings: &mut CompoundDictionaryBindings<'a>,
        data: RawDictionaryView<'a>,
    ) -> bool {
        let addon = state_read!(self, compound_dictionary);
        if state_read!(self, state) != BROTLI_STATE_UNINITED {
            return false;
        }
        if addon == 0 {
            // No source-storage reference survives this allocator callback;
            // outer compound_dictionary still contains its old NULL value.
            let Some(allocation) = self
                .allocator
                .allocate(size_of::<CompoundDictionaryStorage>())
            else {
                return false;
            };
            assert!(self.compound_dictionary.is_none());
            let address = allocation.as_ptr() as usize;
            self.compound_dictionary = Some(allocation);
            addon_write!(self, num_chunks, 0);
            addon_write!(self, total_size, 0);
            addon_write!(self, br_length, 0);
            addon_write!(self, br_copied, 0);
            addon_write!(self, block_bits, -1);
            self.write_chunk_offset(0, 0);
            self.storage_mut().compound_dictionary.write(address);
        } else {
            assert_eq!(
                self.compound_dictionary
                    .as_ref()
                    .expect("source actual addon owner")
                    .as_ptr() as usize,
                addon
            );
        }
        if addon_read!(self, num_chunks) == 15 {
            return false;
        }
        let index = addon_read!(self, num_chunks) as usize;
        let address = data.as_ptr() as usize;
        self.write_chunk_token(index, address);
        assert!(index < bindings.chunks.len());
        bindings.chunks[index] = Some(data);
        addon_write!(self, num_chunks, addon_read!(self, num_chunks) + 1);
        addon_write!(
            self,
            total_size,
            addon_read!(self, total_size) + data.len() as i32
        );
        // SAFETY: The just-incremented source count is initialized and<=15.
        let index = addon_read!(self, num_chunks) as usize;
        self.write_chunk_offset(index, addon_read!(self, total_size));
        true
    }

    /// BrotliDecoderAttachDictionary, including the retained shared prefix on
    /// addon OOM. Both original state tests and prefix-write order are retained.
    ///
    /// # Safety
    /// Same live binding/defined arithmetic contract as AttachCompoundDictionary.
    pub unsafe fn attach_dictionary(
        &mut self,
        bindings: &mut CompoundDictionaryBindings<'a>,
        dictionary_type: u32,
        data: Option<&'a [u8]>,
    ) -> bool {
        // SAFETY: Actual SharedDictionary Create's memset/attach initialize
        // num_prefix. The source reads it before checking decoder state.
        let before = unsafe {
            self.dictionary
                .as_ref()
                .expect("source shared dictionary owner")
                .storage()
                .num_prefix
                .assume_init()
        };
        if state_read!(self, state) != BROTLI_STATE_UNINITED {
            return false;
        }
        if !self
            .dictionary
            .as_mut()
            .unwrap()
            .attach(dictionary_type, data)
        {
            return false;
        }
        let mut i = before;
        // SAFETY: Same initialized source count, reread on each loop condition.
        while i < unsafe {
            self.dictionary
                .as_ref()
                .unwrap()
                .storage()
                .num_prefix
                .assume_init()
        } {
            let prefix = self.dictionary.as_ref().unwrap().prefix_view(i as usize);
            // SAFETY: Source prefix slot and its caller-backed lifetime are
            // retained in this same actual shared dictionary; no owner loan
            // survives the helper's possible allocation callback.
            if !unsafe { self.attach_compound_dictionary_raw(bindings, prefix) } {
                return false;
            }
            i += 1;
        }
        true
    }

    /// BrotliDecoderAttachDictionary for an actual mutable/late-initialized
    /// caller allocation, preserving the same shared/addon mutation order.
    ///
    /// # Safety
    /// Same source initialization/arithmetic/binding requirements as the
    /// immutable API, plus the raw view's allocation and access contract.
    pub unsafe fn attach_dictionary_raw(
        &mut self,
        bindings: &mut CompoundDictionaryBindings<'a>,
        dictionary_type: u32,
        data: RawDictionaryView<'a>,
    ) -> bool {
        // SAFETY: Actual SharedDictionary Create's memset/attach initialize
        // num_prefix. The source reads it before checking decoder state.
        let before = unsafe {
            self.dictionary
                .as_ref()
                .expect("source shared dictionary owner")
                .storage()
                .num_prefix
                .assume_init()
        };
        if state_read!(self, state) != BROTLI_STATE_UNINITED {
            return false;
        }
        // SAFETY: The caller retains this raw caller view through every
        // decoder access; attachment itself reads no prefix bytes.
        if !unsafe {
            self.dictionary
                .as_mut()
                .unwrap()
                .attach_raw(dictionary_type, data)
        } {
            return false;
        }
        let mut i = before;
        // SAFETY: Same initialized source count, reread on each loop condition.
        while i < unsafe {
            self.dictionary
                .as_ref()
                .unwrap()
                .storage()
                .num_prefix
                .assume_init()
        } {
            let prefix = self.dictionary.as_ref().unwrap().prefix_view(i as usize);
            // SAFETY: Source prefix slot and its caller-backed lifetime are
            // retained in this same actual shared dictionary; no owner loan
            // survives the helper's possible allocation callback.
            if !unsafe { self.attach_compound_dictionary_raw(bindings, prefix) } {
                return false;
            }
            i += 1;
        }
        true
    }

    /// EnsureCompoundDictionaryInitialized.
    ///
    /// # Safety
    /// The addon is live and block_bits initialized. On -1, total_size>0,
    /// initialized chunk offsets and all reached signed shifts/additions have
    /// their defined source domain. Only written map entries become initialized.
    pub unsafe fn ensure_compound_dictionary_initialized(&mut self) {
        let mut block_bits = 8;
        let mut cursor = 0;
        let mut index = 0;
        if addon_read!(self, block_bits) != -1 {
            return;
        }
        while ((addon_read!(self, total_size) - 1) >> block_bits) != 0 {
            block_bits += 1;
        }
        block_bits -= 8;
        addon_write!(self, block_bits, block_bits);
        while cursor < addon_read!(self, total_size) {
            // SAFETY: The actual source initialized offsets up to num_chunks.
            while unsafe { self.chunk_offset(index + 1) } < cursor {
                index += 1;
            }
            self.write_block_map((cursor >> block_bits) as usize, index as u8);
            cursor += 1 << block_bits;
        }
    }

    /// InitializeCompoundDictionaryCopy, including its writes before a later
    /// chunk copy and its bounds-check position after source map/index reads.
    ///
    /// # Safety
    /// The actual addon/offset/map initialization and address/length arithmetic
    /// must be source-defined. Failed length checks still perform the reached
    /// map initialization/reads. State distance fields have source initialization.
    pub unsafe fn initialize_compound_dictionary_copy(
        &mut self,
        address: i32,
        length: i32,
    ) -> bool {
        // SAFETY: The caller establishes the source lazy-index init domain.
        unsafe { self.ensure_compound_dictionary_initialized() };
        // SAFETY: A reached source map/offset slot, not its unused suffix.
        let map_index = (address >> addon_read!(self, block_bits)) as usize;
        let mut index = unsafe { self.block_map(map_index) } as usize;
        // SAFETY: The source's same reached initialized offset sequence.
        while address >= unsafe { self.chunk_offset(index + 1) } {
            index += 1;
        }
        if addon_read!(self, total_size) < address + length {
            return false;
        }
        let ring_index = (state_read!(self, dist_rb_idx) & 3) as usize;
        let distance = state_read!(self, distance_code);
        // SAFETY: One original in-bounds recent-distance slot is initialized;
        // no whole array read is performed on its sibling entries.
        unsafe {
            self.storage_mut()
                .dist_rb
                .as_mut_ptr()
                .cast::<i32>()
                .add(ring_index)
                .write(distance)
        };
        let value = state_read!(self, dist_rb_idx) + 1;
        self.storage_mut().dist_rb_idx.write(value);
        let value = state_read!(self, meta_block_remaining_len) - length;
        self.storage_mut().meta_block_remaining_len.write(value);
        addon_write!(self, br_index, index as i32);
        // SAFETY: The source reached and initialized this selected offset.
        addon_write!(
            self,
            br_offset,
            address - unsafe { self.chunk_offset(index) }
        );
        addon_write!(self, br_length, length);
        addon_write!(self, br_copied, 0);
        true
    }

    /// GetCompoundDictionarySize with its original NULL short circuit.
    ///
    /// # Safety
    /// compound_dictionary is initialized; a non-NULL pointer corresponds to
    /// this state's live actual owner with initialized total_size.
    pub unsafe fn compound_dictionary_size(&self) -> i32 {
        if state_read!(self, compound_dictionary) != 0 {
            addon_read!(self, total_size)
        } else {
            0
        }
    }

    /// CopyFromCompoundDictionary, retaining exact chunk-boundary/no-space
    /// writes and zero-copy short circuit. It does not change state.pos itself.
    ///
    /// # Safety
    /// The same-state binding, addon and reached copy checkpoints are live and
    /// initialized. Each reached source pointer arithmetic/memcpy extent is
    /// valid and nonoverlapping; source NULL+0/NULL memcpy caller domains are
    /// not silently represented by a fabricated non-NULL empty slice.
    pub unsafe fn copy_from_compound_dictionary(
        &mut self,
        bindings: &CompoundDictionaryBindings<'a>,
        mut pos: i32,
    ) -> i32 {
        let orig_pos = pos;
        while addon_read!(self, br_length) != addon_read!(self, br_copied) {
            let base = state_read!(self, ringbuffer);
            let start = pos as usize;
            let index = addon_read!(self, br_index) as usize;
            // SAFETY: The current reached source chunk slot is initialized.
            let token = unsafe { self.chunk_token(index) };
            let offset = addon_read!(self, br_offset) as usize;
            let prefix =
                bindings.chunks[index].expect("source reached non-NULL caller-backed chunk");
            assert_eq!(prefix.as_ptr() as usize, token);
            let space = state_read!(self, ringbuffer_size) - pos;
            // SAFETY: Both reached source offsets were initialized by attach.
            let end_index = addon_read!(self, br_index) as usize + 1;
            let end_offset = unsafe { self.chunk_offset(end_index) };
            let start_index = addon_read!(self, br_index) as usize;
            let start_offset = unsafe { self.chunk_offset(start_index) };
            let rem_chunk_length = (end_offset - start_offset) - addon_read!(self, br_offset);
            let mut length = addon_read!(self, br_length) - addon_read!(self, br_copied);
            if length > rem_chunk_length {
                length = rem_chunk_length;
            }
            if length > space {
                length = space;
            }
            let count = length as usize;
            assert!(offset <= prefix.len() && count <= prefix.len() - offset);
            let ring = self.ringbuffer.as_mut().expect("source actual ring owner");
            assert_eq!(ring.as_ptr() as usize, base);
            assert!(start <= ring.len() && count <= ring.len() - start);
            // SAFETY: The raw caller allocation and actual ring owner give
            // checked nonoverlapping live extents. Only source memcpy bytes
            // are copied, without creating a slice of possibly unknown u8s.
            // Their initialized domain follows the caller's source history.
            unsafe { prefix.copy_to(offset, &mut ring.storage_mut()[start..start + count]) };
            pos += length;
            addon_write!(self, br_offset, addon_read!(self, br_offset) + length);
            addon_write!(self, br_copied, addon_read!(self, br_copied) + length);
            if length == rem_chunk_length {
                addon_write!(self, br_index, addon_read!(self, br_index) + 1);
                addon_write!(self, br_offset, 0);
            }
            if pos == state_read!(self, ringbuffer_size) {
                break;
            }
        }
        pos - orig_pos
    }
}
