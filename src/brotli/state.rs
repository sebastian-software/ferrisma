// Copyright 2015 Google Inc. All Rights Reserved.
// Distributed under the MIT license in licenses/BROTLI-MIT.txt.

//! c/dec/state.{c,h}: the actual source allocation plus its ownership sidecar.
//! Field views borrow callback-returned storage only for individual source
//! reads/writes. No such borrow survives an allocator/free callback.
//! This component does not yet expose the complete streaming decoder API.

use super::bit_reader::Reg;
use super::bitfield::BitfieldWord;
use super::layout::{DecoderStateStorage, HuffmanCodeStorage, HuffmanTreeGroupStorage};
use super::memory::{Allocation, Allocator};
use super::running::*;
use super::shared_dictionary::SharedDictionary;
use std::mem::{align_of, offset_of, size_of};

pub const BROTLI_BLOCK_SIZE_CAP: Reg = 1 << 24;

/// Fixed ownership metadata, separate from the source payload. Every native
/// pointer targets its corresponding real allocation or an interior view.
/// Moving this sidecar does not move any source block.
pub struct StateOwner<'a> {
    allocation: Option<Allocation>,
    pub(crate) allocator: Allocator,
    pub(crate) dictionary: Option<SharedDictionary<'a>>,
    pub(crate) compound_dictionary: Option<Allocation>,
    pub(crate) ringbuffer: Option<Allocation>,
    pub(crate) block_type_trees: Option<Allocation>,
    pub(crate) context_modes: Option<Allocation>,
    pub(crate) context_map: Option<Allocation>,
    pub(crate) dist_context_map: Option<Allocation>,
    pub(crate) hgroups: [Option<Allocation>; 3],
    initialized: bool,
}

macro_rules! write {
    ($s:expr, $field:ident, $value:expr) => {{
        $s.storage_mut().$field.write($value);
    }};
}
macro_rules! array_write {
    ($s:expr, $field:ident, $index:expr, $ty:ty, $value:expr) => {{
        // SAFETY: Every invocation below uses an original in-bounds constant
        // index. as_mut_ptr forms no reference to an uninitialized element;
        // this one write initializes exactly the selected source element.
        unsafe {
            $s.storage_mut()
                .$field
                .as_mut_ptr()
                .cast::<$ty>()
                .add($index)
                .write($value);
        }
    }};
}

impl<'a> StateOwner<'a> {
    /// Perform the complete source StateInit on an already allocated source
    /// state block. CreateInstance's allocator selection/static checkpoint is
    /// kept outside this internal component. The actual producing Allocation
    /// remains the sole backing owner even on the failing dictionary path.
    #[expect(
        clippy::question_mark,
        reason = "Keep the explicit source dictionary-failure checkpoint and its distinct cleanup order"
    )]
    pub fn initialize(
        allocation: Allocation,
        allocator: Allocator,
        shared_allocator: Allocator,
        alloc_address: usize,
        free_address: usize,
        state_opaque: usize,
        shared_opaque: usize,
    ) -> Option<Self> {
        assert_eq!(allocation.len(), size_of::<DecoderStateStorage>());
        assert_eq!(
            allocation.as_ptr() as usize % align_of::<DecoderStateStorage>(),
            0
        );
        let mut result = Self {
            allocation: Some(allocation),
            allocator,
            dictionary: None,
            compound_dictionary: None,
            ringbuffer: None,
            block_type_trees: None,
            context_modes: None,
            context_map: None,
            dist_context_map: None,
            hgroups: [None, None, None],
            initialized: false,
        };
        write!(result, alloc_func, alloc_address);
        write!(result, free_func, free_address);
        write!(result, memory_manager_opaque, state_opaque);
        write!(result, error_code, 0);
        // BrotliInitBitReader initializes only these two scalar fields. Its
        // pointer slots must retain their allocator-seeded/unknown bytes.
        result.bit_reader_init();
        write!(result, state, BROTLI_STATE_UNINITED);
        result.write_flag(5, 1, 0); // large_window
        write!(
            result,
            substate_metablock_header,
            BROTLI_STATE_METABLOCK_HEADER_NONE
        );
        write!(
            result,
            substate_uncompressed,
            BROTLI_STATE_UNCOMPRESSED_NONE
        );
        write!(
            result,
            substate_decode_uint8,
            BROTLI_STATE_DECODE_UINT8_NONE
        );
        write!(
            result,
            substate_read_block_length,
            BROTLI_STATE_READ_BLOCK_LENGTH_NONE
        );
        write!(result, buffer_length, 0);
        write!(result, loop_counter, 0);
        write!(result, pos, 0);
        write!(result, rb_roundtrips, 0);
        write!(result, partial_pos_out, 0);
        write!(result, used_input, 0);
        write!(result, block_type_trees, 0);
        write!(result, block_len_trees, 0);
        write!(result, ringbuffer, 0);
        write!(result, ringbuffer_size, 0);
        write!(result, new_ringbuffer_size, 0);
        write!(result, ringbuffer_mask, 0);
        write!(result, context_map, 0);
        write!(result, context_modes, 0);
        write!(result, dist_context_map, 0);
        write!(result, context_map_slice, 0);
        write!(result, dist_context_map_slice, 0);
        result.group_mut(0).codes.write(0);
        result.group_mut(0).htrees.write(0);
        result.group_mut(1).codes.write(0);
        result.group_mut(1).htrees.write(0);
        result.group_mut(2).codes.write(0);
        result.group_mut(2).htrees.write(0);
        result.write_flag(0, 1, 0); // is_last_metablock
        result.write_flag(1, 1, 0); // is_uncompressed
        result.write_flag(2, 1, 0); // is_metadata
        result.write_flag(3, 1, 0); // should_wrap_ringbuffer
        result.write_flag(4, 1, 1); // canny_ringbuffer_allocation
        result.write_flag(6, 6, 0); // window_bits
        write!(result, max_distance, 0);
        array_write!(result, dist_rb, 0, i32, 16);
        array_write!(result, dist_rb, 1, i32, 15);
        array_write!(result, dist_rb, 2, i32, 11);
        array_write!(result, dist_rb, 3, i32, 4);
        write!(result, dist_rb_idx, 0);
        write!(result, block_type_trees, 0);
        write!(result, block_len_trees, 0);
        write!(result, mtf_upper_bound, 63);
        write!(result, compound_dictionary, 0);
        // RHS allocation runs while dictionary still contains its original
        // unknown/seeded bytes. All field borrows above have ended, allowing
        // the custom allocator to observe the actual outer block now.
        result.dictionary =
            SharedDictionary::create(shared_allocator, alloc_address, free_address, shared_opaque);
        let dictionary_address = result
            .dictionary
            .as_ref()
            .map_or(0, |owner| owner.as_ptr() as usize);
        write!(result, dictionary, dictionary_address);
        if result.dictionary.is_none() {
            // Source CreateInstance releases only the state block after an
            // Init failure; Cleanup's null-free callbacks must not be added.
            return None;
        }
        write!(result, metadata_start_func, 0);
        write!(result, metadata_chunk_func, 0);
        write!(result, metadata_callback_opaque, 0);
        result.initialized = true;
        Some(result)
    }

    fn bit_reader_init(&mut self) {
        let br = self.storage_mut().br.as_mut_ptr();
        // SAFETY: br points into the checked native allocation. Its storage
        // fields are MaybeUninit; this writes only the two source Init fields.
        unsafe {
            (*br).val.write(0);
            (*br).bit_pos.write(0);
        }
    }

    pub fn metablock_begin(&mut self) {
        write!(self, meta_block_remaining_len, 0);
        array_write!(self, block_length, 0, Reg, BROTLI_BLOCK_SIZE_CAP);
        array_write!(self, block_length, 1, Reg, BROTLI_BLOCK_SIZE_CAP);
        array_write!(self, block_length, 2, Reg, BROTLI_BLOCK_SIZE_CAP);
        array_write!(self, num_block_types, 0, Reg, 1);
        array_write!(self, num_block_types, 1, Reg, 1);
        array_write!(self, num_block_types, 2, Reg, 1);
        array_write!(self, block_type_rb, 0, Reg, 1);
        array_write!(self, block_type_rb, 1, Reg, 0);
        array_write!(self, block_type_rb, 2, Reg, 1);
        array_write!(self, block_type_rb, 3, Reg, 0);
        array_write!(self, block_type_rb, 4, Reg, 1);
        array_write!(self, block_type_rb, 5, Reg, 0);
        write!(self, context_map, 0);
        write!(self, context_modes, 0);
        write!(self, dist_context_map, 0);
        write!(self, context_map_slice, 0);
        write!(self, literal_htree, 0);
        write!(self, dist_context_map_slice, 0);
        write!(self, dist_htree_index, 0);
        write!(self, context_lookup, 0);
        self.group_mut(0).codes.write(0);
        self.group_mut(0).htrees.write(0);
        self.group_mut(1).codes.write(0);
        self.group_mut(1).htrees.write(0);
        self.group_mut(2).codes.write(0);
        self.group_mut(2).htrees.write(0);
    }

    pub fn cleanup_after_metablock(&mut self) {
        // The source writes each pointer to null only AFTER its free callback.
        // No outer storage reference survives any callback below.
        self.allocator.release(self.context_modes.take());
        write!(self, context_modes, 0);
        self.allocator.release(self.context_map.take());
        write!(self, context_map, 0);
        self.allocator.release(self.dist_context_map.take());
        write!(self, dist_context_map, 0);
        self.allocator.release(self.hgroups[0].take());
        self.group_mut(0).htrees.write(0);
        self.allocator.release(self.hgroups[1].take());
        self.group_mut(1).htrees.write(0);
        self.allocator.release(self.hgroups[2].take());
        self.group_mut(2).htrees.write(0);
    }

    /// Source's literal/insert-copy/distance group selection, respectively.
    /// Reached Init sites have released any prior group before allocating.
    pub fn huffman_group_init(
        &mut self,
        group: usize,
        maximum: Reg,
        limit: Reg,
        ntrees: Reg,
    ) -> bool {
        assert!(group < 3 && self.hgroups[group].is_none());
        let max_table_size = limit.wrapping_add(376) as usize;
        let code_size = size_of::<HuffmanCodeStorage>()
            .wrapping_mul(ntrees as usize)
            .wrapping_mul(max_table_size);
        let htree_size = size_of::<usize>().wrapping_mul(ntrees as usize);
        let allocation = self.allocator.allocate(code_size.wrapping_add(htree_size));
        let address = allocation
            .as_ref()
            .map_or(0, |owner| owner.as_ptr() as usize);
        self.hgroups[group] = allocation;
        self.group_mut(group)
            .alphabet_size_max
            .write(maximum as u16);
        self.group_mut(group)
            .alphabet_size_limit
            .write(limit as u16);
        self.group_mut(group).num_htrees.write(ntrees as u16);
        self.group_mut(group).htrees.write(address);
        self.group_mut(group).codes.write(if address != 0 {
            address.wrapping_add(htree_size)
        } else {
            0
        });
        address != 0
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.allocation.as_ref().expect("live state owner").as_ptr()
    }

    pub fn storage(&self) -> &DecoderStateStorage {
        let owner = self.allocation.as_ref().expect("live state owner");
        assert_eq!(owner.len(), size_of::<DecoderStateStorage>());
        assert_eq!(
            owner.as_ptr() as usize % align_of::<DecoderStateStorage>(),
            0
        );
        // SAFETY: The checked native block is exclusively owned for its
        // lifetime. Every field is MaybeUninit, so unknown source fields are
        // valid storage rather than prematurely initialized Rust scalars.
        unsafe { &*owner.as_ptr().cast::<DecoderStateStorage>() }
    }

    pub(crate) fn storage_mut(&mut self) -> &mut DecoderStateStorage {
        let owner = self.allocation.as_mut().expect("live state owner");
        assert_eq!(owner.len(), size_of::<DecoderStateStorage>());
        let storage = owner.storage_mut();
        assert_eq!(
            storage.as_ptr() as usize % align_of::<DecoderStateStorage>(),
            0
        );
        // SAFETY: Checked native extent/alignment and all-MaybeUninit fields;
        // the exclusive actual-owner borrow prevents competing raw aliases.
        unsafe { &mut *storage.as_mut_ptr().cast::<DecoderStateStorage>() }
    }

    pub(crate) fn write_flag(&mut self, shift: u32, width: u32, value: u32) {
        let owner = self.allocation.as_mut().expect("live state owner");
        let offset = offset_of!(DecoderStateStorage, flags);
        let bytes = &mut owner.storage_mut()[offset..offset + 4];
        BitfieldWord::new(bytes.try_into().expect("four native flag bytes"))
            .write(shift, width, value);
    }

    fn group_mut(&mut self, group: usize) -> &mut HuffmanTreeGroupStorage {
        let s = self.storage_mut();
        let ptr = match group {
            0 => s.literal_hgroup.as_mut_ptr(),
            1 => s.insert_copy_hgroup.as_mut_ptr(),
            2 => s.distance_hgroup.as_mut_ptr(),
            _ => panic!("source Huffman group"),
        };
        // SAFETY: The nested native structure has checked parent extent and
        // alignment. Each of its fields is MaybeUninit, permitting unknown
        // alphabet fields while source Init writes its two pointer fields.
        unsafe { &mut *ptr }
    }
}

impl Drop for StateOwner<'_> {
    fn drop(&mut self) {
        if self.initialized {
            self.cleanup_after_metablock();
            self.allocator.release(self.compound_dictionary.take());
            write!(self, compound_dictionary, 0);
            // Its own source cleanup invokes three null frees then frees the
            // actual shared block while state's pointer still has its value.
            drop(self.dictionary.take());
            write!(self, dictionary, 0);
            self.allocator.release(self.ringbuffer.take());
            write!(self, ringbuffer, 0);
            self.allocator.release(self.block_type_trees.take());
            write!(self, block_type_trees, 0);
        }
        self.allocator.release(self.allocation.take());
    }
}
