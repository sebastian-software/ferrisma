// Copyright 2013, 2015, 2017 Google Inc. All Rights Reserved.
// Distributed under the MIT license in licenses/BROTLI-MIT.txt.

//! Native storage layout from c/dec/{state,bit_reader,huffman}.h and
//! c/common/shared_dictionary_internal.h. Every field remains MaybeUninit
//! until its original source checkpoint. Pointer-sized slots retain source
//! addresses as observations; actual borrows must resolve through the live
//! owner, never reconstruct an owner or dereference an integer address.

use super::bit_reader::Reg;
use std::mem::{ManuallyDrop, MaybeUninit, align_of, offset_of, size_of};

macro_rules! storage {
    ($name:ident { $($field:ident: $ty:ty),* $(,)? }) => {
        #[repr(C)]
        pub struct $name { $(pub $field: MaybeUninit<$ty>),* }
    };
}

storage!(BitReaderStorage {
    val: Reg,
    bit_pos: Reg,
    next_in: usize,
    guard_in: usize,
    last_in: usize,
});
storage!(HuffmanCodeStorage {
    bits: u8,
    value: u16
});
storage!(HuffmanTreeGroupStorage {
    htrees: usize,
    codes: usize,
    alphabet_size_max: u16,
    alphabet_size_limit: u16,
    num_htrees: u16,
});
storage!(CompoundDictionaryStorage {
    num_chunks: i32,
    total_size: i32,
    br_index: i32,
    br_offset: i32,
    br_length: i32,
    br_copied: i32,
    chunks: [usize; 16],
    chunk_offsets: [i32; 16],
    block_bits: i32,
    block_map: [u8; 256],
});
storage!(HeaderArenaStorage {
    substate_tree_group: u32,
    substate_context_map: u32,
    substate_huffman: u32,
    sub_loop_counter: Reg,
    repeat_code_len: Reg,
    prev_code_len: Reg,
    symbol: Reg,
    repeat: Reg,
    space: Reg,
    table: [HuffmanCodeStorage; 32],
    symbol_lists: usize,
    symbols_lists_array: [u16; 720],
    next_symbol: [i32; 32],
    code_length_code_lengths: [u8; 18],
    code_length_histo: [u16; 16],
    htree_index: i32,
    next: usize,
    context_index: Reg,
    max_run_length_prefix: Reg,
    code: Reg,
    context_map_table: [HuffmanCodeStorage; 646],
});
storage!(BodyArenaStorage {
    dist_extra_bits: [u8; 544],
    dist_offset: [Reg; 544]
});
#[repr(C)]
pub union ArenaStorage {
    pub header: ManuallyDrop<MaybeUninit<HeaderArenaStorage>>,
    pub body: ManuallyDrop<MaybeUninit<BodyArenaStorage>>,
}

storage!(SharedDictionaryStorage {
    num_prefix: u32,
    prefix_size: [usize; 15],
    prefix: [usize; 15],
    context_based: i32,
    context_map: [u8; 64],
    num_dictionaries: u8,
    words: [usize; 64],
    transforms: [usize; 64],
    num_word_lists: u8,
    words_instances: usize,
    num_transform_lists: u8,
    transforms_instances: usize,
    prefix_suffix_maps: usize,
    alloc_func: usize,
    free_func: usize,
    memory_manager_opaque: usize,
});

storage!(DecoderStateStorage {
    state: u32,
    loop_counter: i32,
    br: BitReaderStorage,
    alloc_func: usize,
    free_func: usize,
    memory_manager_opaque: usize,
    // The original u64/u8[8] union has uint64_t size and alignment.
    buffer: u64,
    buffer_length: Reg,
    pos: i32,
    max_backward_distance: i32,
    max_distance: i32,
    ringbuffer_size: i32,
    ringbuffer_mask: i32,
    dist_rb_idx: i32,
    dist_rb: [i32; 4],
    error_code: i32,
    meta_block_remaining_len: i32,
    ringbuffer: usize,
    ringbuffer_end: usize,
    htree_command: usize,
    context_lookup: usize,
    context_map_slice: usize,
    dist_context_map_slice: usize,
    literal_hgroup: HuffmanTreeGroupStorage,
    insert_copy_hgroup: HuffmanTreeGroupStorage,
    distance_hgroup: HuffmanTreeGroupStorage,
    block_type_trees: usize,
    block_len_trees: usize,
    trivial_literal_context: i32,
    distance_context: i32,
    block_length: [Reg; 3],
    block_length_index: Reg,
    num_block_types: [Reg; 3],
    block_type_rb: [Reg; 6],
    distance_postfix_bits: Reg,
    num_direct_distance_codes: Reg,
    num_dist_htrees: Reg,
    dist_context_map: usize,
    literal_htree: usize,
    rb_roundtrips: usize,
    partial_pos_out: usize,
    mtf_upper_bound: Reg,
    mtf: [u32; 65],
    copy_length: i32,
    distance_code: i32,
    dist_htree_index: u8,
    metadata_start_func: usize,
    metadata_chunk_func: usize,
    metadata_callback_opaque: usize,
    used_input: u64,
    substate_metablock_header: u32,
    substate_uncompressed: u32,
    substate_decode_uint8: u32,
    substate_read_block_length: u32,
    new_ringbuffer_size: i32,
    // This word is not read as u32 before every bit is initialized. Individual
    // fields use the separately verified raw bitfield representation.
    flags: u32,
    num_literal_htrees: Reg,
    context_map: usize,
    context_modes: usize,
    dictionary: usize,
    compound_dictionary: usize,
    trivial_literal_contexts: [u32; 8],
    arena: ArenaStorage,
});

pub fn source_sizes() -> [(&'static str, usize, usize); 8] {
    macro_rules! item {
        ($name:literal, $ty:ty) => {
            ($name, size_of::<$ty>(), align_of::<$ty>())
        };
    }
    [
        item!("BrotliDecoderState", DecoderStateStorage),
        item!("BrotliBitReader", BitReaderStorage),
        item!("HuffmanCode", HuffmanCodeStorage),
        item!("HuffmanTreeGroup", HuffmanTreeGroupStorage),
        item!("BrotliSharedDictionaryInternal", SharedDictionaryStorage),
        item!("BrotliDecoderCompoundDictionary", CompoundDictionaryStorage),
        item!("BrotliMetablockHeaderArena", HeaderArenaStorage),
        item!("BrotliMetablockBodyArena", BodyArenaStorage),
    ]
}

pub fn source_offsets() -> [(&'static str, usize); 13] {
    macro_rules! item {
        ($name:literal, $ty:ty, $field:ident) => {
            ($name, offset_of!($ty, $field))
        };
    }
    [
        item!("BrotliDecoderState.br", DecoderStateStorage, br),
        item!(
            "BrotliDecoderState.alloc_func",
            DecoderStateStorage,
            alloc_func
        ),
        item!("BrotliDecoderState.buffer", DecoderStateStorage, buffer),
        item!(
            "BrotliDecoderState.ringbuffer",
            DecoderStateStorage,
            ringbuffer
        ),
        item!(
            "BrotliDecoderState.literal_hgroup",
            DecoderStateStorage,
            literal_hgroup
        ),
        item!("BrotliDecoderState.mtf", DecoderStateStorage, mtf),
        item!("BrotliDecoderState.arena", DecoderStateStorage, arena),
        item!(
            "BrotliDecoderState.dictionary",
            DecoderStateStorage,
            dictionary
        ),
        item!(
            "BrotliSharedDictionaryInternal.prefix_size",
            SharedDictionaryStorage,
            prefix_size
        ),
        item!(
            "BrotliSharedDictionaryInternal.words",
            SharedDictionaryStorage,
            words
        ),
        item!(
            "BrotliSharedDictionaryInternal.transforms_instances",
            SharedDictionaryStorage,
            transforms_instances
        ),
        item!("HuffmanTreeGroup.codes", HuffmanTreeGroupStorage, codes),
        item!("HuffmanCode.value", HuffmanCodeStorage, value),
    ]
}

pub fn flags_offset() -> usize {
    offset_of!(DecoderStateStorage, flags)
}
