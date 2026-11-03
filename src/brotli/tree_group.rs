// Copyright 2013 Google Inc. All Rights Reserved.
// Distributed under the MIT license in licenses/BROTLI-MIT.txt.
//! Mechanical translation of decode.c:1021-1040. Both output regions borrow
//! the actual single pointer-table/code allocation made by source GroupInit.

use super::bit_reader::{BitReader, Reg};
use super::huffman::HuffmanCode;
use super::huffman_reader::{HuffmanHeaderFields, read_huffman_code, split_header};
use super::layout::{HeaderArenaStorage, HuffmanTreeGroupStorage};
use super::running::{BROTLI_STATE_TREE_GROUP_LOOP, BROTLI_STATE_TREE_GROUP_NONE};
use super::status::{BROTLI_DECODER_SUCCESS, BrotliDecoderErrorCode};
use std::mem::{MaybeUninit, align_of, size_of};

pub struct TreeGroupFields<'a> {
    pub substate: &'a mut MaybeUninit<u32>,
    pub index: &'a mut MaybeUninit<i32>,
    pub next: &'a mut MaybeUninit<usize>,
}

/// Borrow the sixteen disjoint actual header fields. The existing sealed
/// thirteen-field view remains unchanged; no owned header or payload copy is
/// constructed, and no whole-header reference is retained during decoding.
pub fn split_tree_group(
    header: &mut HeaderArenaStorage,
) -> (TreeGroupFields<'_>, HuffmanHeaderFields<'_>) {
    let actual = std::ptr::from_mut(header);
    let (huffman, _) = split_header(header);
    // SAFETY: the completed split_header call retains only these thirteen
    // field loans: substate_huffman, sub_loop_counter, repeat_code_len,
    // prev_code_len, symbol, repeat, space, table, symbol_lists,
    // symbols_lists_array, next_symbol, code_length_code_lengths and
    // code_length_histo. The discarded context_map_table loan is unused.
    // The context_map_table sibling also overlaps none of the three tree
    // fields. None of these sixteen retained field loans overlap each other.
    // The original exclusive owner pointer still supplies those disjoint live
    // fields; no whole-header reference is recreated or used after the split.
    let fields = unsafe {
        TreeGroupFields {
            substate: &mut *std::ptr::addr_of_mut!((*actual).substate_tree_group),
            index: &mut *std::ptr::addr_of_mut!((*actual).htree_index),
            next: &mut *std::ptr::addr_of_mut!((*actual).next),
        }
    };
    (fields, huffman)
}

/// Borrowed actual GroupInit allocation geometry, not an allocation owner.
/// No pointer slot/code field is read merely by constructing this view.
pub struct TreeGroupOutput<'a> {
    storage: &'a mut [MaybeUninit<u8>],
    pointer_count: usize,
    code_offset: usize,
}
impl<'a> TreeGroupOutput<'a> {
    pub fn new(storage: &'a mut [MaybeUninit<u8>], pointer_count: usize) -> Self {
        let code_offset = pointer_count.checked_mul(size_of::<usize>()).unwrap();
        assert!(code_offset <= storage.len());
        assert_eq!((storage.len() - code_offset) % size_of::<HuffmanCode>(), 0);
        Self {
            storage,
            pointer_count,
            code_offset,
        }
    }
    fn code_capacity(&self) -> usize {
        (self.storage.len() - self.code_offset) / size_of::<HuffmanCode>()
    }
    fn code_address(&self, index: usize) -> usize {
        assert!(index <= self.code_capacity());
        self.storage
            .as_ptr()
            .wrapping_add(self.code_offset + index * size_of::<HuffmanCode>()) as usize
    }
    fn code_index(&self, token: usize) -> usize {
        let offset = token.checked_sub(self.code_address(0)).unwrap();
        assert_eq!(offset % size_of::<HuffmanCode>(), 0);
        let index = offset / size_of::<HuffmanCode>();
        assert!(index <= self.code_capacity());
        index
    }
    fn codes_at(&mut self, token: usize) -> &mut [MaybeUninit<HuffmanCode>] {
        let index = self.code_index(token);
        let count = self.code_capacity() - index;
        let offset = self.code_offset + index * size_of::<HuffmanCode>();
        let pointer = self.storage.as_mut_ptr().wrapping_add(offset);
        assert_eq!((pointer as usize) % align_of::<HuffmanCode>(), 0);
        // SAFETY: original actual allocation geometry supplies count complete,
        // aligned code slots at this owner-relative offset. MaybeUninit permits
        // all fields/padding to remain unknown. Exclusivity covers only this
        // real code suffix; sibling pointer-table bytes are not included.
        unsafe { std::slice::from_raw_parts_mut(pointer.cast(), count) }
    }
    fn write_pointer(&mut self, table_token: usize, index: i32, value: usize) {
        assert_eq!(table_token, self.storage.as_ptr() as usize);
        let index = usize::try_from(index).unwrap();
        assert!(index < self.pointer_count);
        let pointer = self.storage.as_mut_ptr();
        assert_eq!((pointer as usize) % align_of::<usize>(), 0);
        // SAFETY: checked actual pointer-table origin/index/alignment. This
        // complete native pointer-representation store initializes only the
        // reached sibling slot, without reading unknown slots or code padding.
        unsafe {
            pointer
                .cast::<MaybeUninit<usize>>()
                .add(index)
                .write(MaybeUninit::new(value))
        };
    }
    fn advance(&self, token: usize, count: Reg) -> usize {
        let index = self.code_index(token);
        let count = usize::try_from(count).unwrap();
        self.code_address(index.checked_add(count).unwrap())
    }
}

/// # Safety
/// Descriptor/state fields have the original source initialization history.
/// substate and every reached num_htrees/index condition are initialized. A
/// non-LOOP entry requires initialized group.codes, then initializes next/index.
/// Only a reached iteration needs initialized alphabet fields/next and the
/// frozen read_huffman_code checkpoint. Only successful headers need initialized
/// group.htrees and an in-bounds pointer slot/index increment. Output geometry
/// matches the actual callback allocation, whose owner/address remains stable
/// through every partial resume. Pointer tokens are resolved only relative to
/// that owner; no token is dereferenced or used to reconstruct ownership.
/// All group/header/reader/input/output loans are disjoint actual storage.
pub unsafe fn huffman_tree_group_decode(
    group: &HuffmanTreeGroupStorage,
    output: &mut TreeGroupOutput<'_>,
    fields: &mut TreeGroupFields<'_>,
    huffman: &mut HuffmanHeaderFields<'_>,
    br: &mut BitReader<'_>,
) -> BrotliDecoderErrorCode {
    // SAFETY: source initialization/resume always supplies this raw state.
    if unsafe { fields.substate.assume_init_read() } != BROTLI_STATE_TREE_GROUP_LOOP {
        // SAFETY: non-LOOP source entry requires only this descriptor pointer;
        // no code payload/alphabet/pointer-table slot is read at this checkpoint.
        fields.next.write(unsafe { group.codes.assume_init_read() });
        fields.index.write(0);
        fields.substate.write(BROTLI_STATE_TREE_GROUP_LOOP);
    }
    // SAFETY: non-LOOP initialization or the source resume supplies the signed
    // index; group initialization supplies num_htrees at every loop condition.
    while unsafe { fields.index.assume_init_read() }
        < i32::from(unsafe { group.num_htrees.assume_init_read() })
    {
        let mut table_size = MaybeUninit::uninit();
        // SAFETY: only a reached iteration requires these exact alphabet and
        // next fields. No whole descriptor/array initialized borrow is formed.
        let max = Reg::from(unsafe { group.alphabet_size_max.assume_init_read() });
        // SAFETY: same reached source call argument checkpoint.
        let limit = Reg::from(unsafe { group.alphabet_size_limit.assume_init_read() });
        // SAFETY: source non-LOOP entry or retained partial iteration wrote next.
        let next = unsafe { fields.next.assume_init_read() };
        // SAFETY: stable source checkpoint and actual code-suffix backing meet
        // the frozen header reader's exact initialization/nonalias contracts.
        let result = unsafe {
            read_huffman_code(
                max,
                limit,
                output.codes_at(next),
                Some(&mut table_size),
                br,
                huffman,
            )
        };
        if result != BROTLI_DECODER_SUCCESS {
            return result;
        }
        // SAFETY: this descriptor pointer is reached only after success; the
        // exact group allocation proves its live owner-relative table origin.
        let pointer_table = unsafe { group.htrees.assume_init_read() };
        // SAFETY: source initialized index before the successful iteration.
        let index = unsafe { fields.index.assume_init_read() };
        // SAFETY: header reader borrows none of these tree-group fields.
        let next = unsafe { fields.next.assume_init_read() };
        output.write_pointer(pointer_table, index, next);
        // SAFETY: source next pointer and successful size output initialized.
        let next = unsafe { fields.next.assume_init_read() };
        // SAFETY: only a successful read_huffman_code return initializes size.
        let size = unsafe { table_size.assume_init() };
        fields.next.write(output.advance(next, size));
        // SAFETY: same source index, with source-defined signed increment.
        let index = unsafe { fields.index.assume_init_read() };
        fields.index.write(index + 1);
    }
    fields.substate.write(BROTLI_STATE_TREE_GROUP_NONE);
    BROTLI_DECODER_SUCCESS
}
