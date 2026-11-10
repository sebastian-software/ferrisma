// Copyright 2013 Google Inc. All Rights Reserved.
// Distributed under the MIT license in licenses/BROTLI-MIT.txt.

//! Source-shaped immutable descriptors for c/common/{dictionary,transform}.h.
//! The decoder stores their real addresses in its shared dictionary block.
//! Payload pointers refer to the existing exact crate-owned static tables;
//! these descriptors do not allocate or clone dictionary/transform payloads.
//! In the pinned default configuration dictionary data is embedded and
//! BrotliSetDictionaryData ignores its argument. External dictionary-data
//! configuration would require a separate source-matched mutable boundary.

use super::tables::{DICTIONARY_DATA, PREFIX_SUFFIX, PREFIX_SUFFIX_MAP, TRANSFORMS_DATA};
use std::mem::align_of;

const POINTER_ALIGNMENT: usize = align_of::<*const u8>();
const U16_PADDING: usize = (POINTER_ALIGNMENT - 2 % POINTER_ALIGNMENT) % POINTER_ALIGNMENT;
const U32_PADDING: usize = (POINTER_ALIGNMENT - 4 % POINTER_ALIGNMENT) % POINTER_ALIGNMENT;
const TAIL_PADDING: usize = (POINTER_ALIGNMENT - 20 % POINTER_ALIGNMENT) % POINTER_ALIGNMENT;

#[repr(C)]
pub struct DictionaryDescriptor {
    pub size_bits_by_length: [u8; 32],
    pub offsets_by_length: [u32; 32],
    pub data_size: usize,
    pub data: *const u8,
}

// SAFETY: This source-default descriptor is immutable; its pointer targets
// the immutable, crate-owned embedded dictionary for the entire program.
// The pinned non-external SetDictionaryData branch performs no mutation.
unsafe impl Sync for DictionaryDescriptor {}

pub static DICTIONARY: DictionaryDescriptor = DictionaryDescriptor {
    size_bits_by_length: [
        0, 0, 0, 0, 10, 10, 11, 11, 10, 10, 10, 10, 10, 9, 9, 8, 7, 7, 8, 7, 7, 6, 6, 5, 5, 0, 0,
        0, 0, 0, 0, 0,
    ],
    offsets_by_length: [
        0, 0, 0, 0, 0, 4096, 9216, 21504, 35840, 44032, 53248, 63488, 74752, 87040, 93696, 100864,
        104704, 106752, 108928, 113536, 115968, 118528, 119872, 121280, 122016, 122784, 122784,
        122784, 122784, 122784, 122784, 122784,
    ],
    data_size: 122784,
    data: DICTIONARY_DATA.as_ptr(),
};

#[repr(C)]
pub struct TransformDescriptor {
    pub prefix_suffix_size: u16,
    // Source static storage has zero-initialized padding. Explicit byte
    // members retain those defined observations without reading Rust padding.
    padding_after_size: [u8; U16_PADDING],
    pub prefix_suffix: *const u8,
    pub prefix_suffix_map: *const u16,
    pub num_transforms: u32,
    padding_after_count: [u8; U32_PADDING],
    pub transforms: *const u8,
    pub params: *const u8,
    pub cut_off_transforms: [i16; 10],
    trailing_padding: [u8; TAIL_PADDING],
}

// SAFETY: Every nonnull pointer targets the same immutable crate-owned table
// that the borrowed transform helper uses, with static lifetime and no writes.
// params is null because no source-default transform is a Shift transform.
unsafe impl Sync for TransformDescriptor {}

pub static TRANSFORMS: TransformDescriptor = TransformDescriptor {
    prefix_suffix_size: PREFIX_SUFFIX.len() as u16,
    padding_after_size: [0; U16_PADDING],
    prefix_suffix: PREFIX_SUFFIX.as_ptr(),
    prefix_suffix_map: PREFIX_SUFFIX_MAP.as_ptr(),
    num_transforms: (TRANSFORMS_DATA.len() / 3) as u32,
    padding_after_count: [0; U32_PADDING],
    transforms: TRANSFORMS_DATA.as_ptr(),
    params: std::ptr::null(),
    cut_off_transforms: [0, 12, 27, 23, 42, 63, 56, 48, 59, 64],
    trailing_padding: [0; TAIL_PADDING],
};
