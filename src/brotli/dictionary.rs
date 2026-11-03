// Copyright 2013 Google Inc. All Rights Reserved.
// Distributed under MIT license. See licenses/BROTLI-MIT.txt or
// https://opensource.org/licenses/MIT.
// Mechanical translation of pinned Brotli c/common/dictionary.{c,h}.

use super::tables::DICTIONARY_DATA;

pub const BROTLI_MIN_DICTIONARY_WORD_LENGTH: usize = 4;
pub const BROTLI_MAX_DICTIONARY_WORD_LENGTH: usize = 24;

/// Borrowed dictionary data replaces the source's immutable data pointer.
pub struct BrotliDictionary<'a> {
    pub size_bits_by_length: [u8; 32],
    pub offsets_by_length: [u32; 32],
    pub data_size: usize,
    pub data: &'a [u8],
}

static DICTIONARY: BrotliDictionary<'static> = BrotliDictionary {
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
    data: &DICTIONARY_DATA,
};

pub fn brotli_get_dictionary() -> &'static BrotliDictionary<'static> {
    &DICTIONARY
}

pub fn brotli_set_dictionary_data(_data: Option<&[u8]>) {
    // The pinned default configuration embeds its dictionary. The original
    // non-BROTLI_EXTERNAL_DICTIONARY_DATA branch ignores this argument.
}
