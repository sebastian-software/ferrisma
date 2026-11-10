// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Direct module paths let this isolated support shard run before registration.

#[path = "../src/brotli/constants.rs"]
pub mod constants;
#[path = "../src/brotli/huffman.rs"]
pub mod huffman;

use huffman::*;
use std::mem::MaybeUninit;

#[cfg(feature = "cpp-oracle")]
#[path = "parity/brotli_huffman.rs"]
mod parity;

fn initialized_table(size: usize) -> Vec<MaybeUninit<HuffmanCode>> {
    vec![MaybeUninit::new(HuffmanCode::new(165, 48879)); size]
}

fn initialized_codes(table: &[MaybeUninit<HuffmanCode>]) -> Vec<HuffmanCode> {
    table
        .iter()
        .map(|slot| {
            // SAFETY: test tables initialize every slot to a valid sentinel before
            // the builders run; every builder write stores another valid code.
            unsafe { slot.assume_init() }
        })
        .collect()
}

fn symbol_lists(lengths: &[u8]) -> (Vec<u16>, [u16; 16]) {
    let mut links = vec![65535; lengths.len() + 16];
    let mut next = std::array::from_fn::<_, 16, _>(|i| i);
    let mut count = [0u16; 16];
    count[0] = 4660;
    for (symbol, &length) in lengths.iter().enumerate() {
        if length != 0 {
            let length = usize::from(length);
            count[length] += 1;
            links[next[length]] = symbol as u16;
            next[length] = symbol + 16;
        }
    }
    (links, count)
}

fn partial_symbol_lists(lengths: &[u8]) -> (Vec<MaybeUninit<u16>>, Vec<bool>) {
    let mut words = vec![MaybeUninit::uninit(); lengths.len() + 16];
    let mut initialized = vec![false; words.len()];
    let mut next = std::array::from_fn::<_, 16, _>(|i| i);
    // Match ReadHuffmanCode's 16 assigned negative-index heads.
    for i in 0..16 {
        words[i].write(65535);
        initialized[i] = true;
    }
    // Match ProcessSingleCodeLength's forward-link writes. Zero-length
    // symbols and each chain's final link deliberately remain uninitialized.
    for (symbol, &length) in lengths.iter().enumerate() {
        if length != 0 {
            let length = usize::from(length);
            words[next[length]].write(symbol as u16);
            initialized[next[length]] = true;
            next[length] = symbol + 16;
        }
    }
    (words, initialized)
}

#[test]
fn native_code_layout_matches_the_pinned_native_source() {
    assert_eq!(std::mem::size_of::<HuffmanCode>(), 4);
    assert_eq!(std::mem::align_of::<HuffmanCode>(), 2);
    assert_eq!(std::mem::offset_of!(HuffmanCode, bits), 0);
    assert_eq!(std::mem::offset_of!(HuffmanCode, value), 2);
}

#[test]
fn simple_sorting_and_replication_keep_the_original_mutation() {
    let mut table = initialized_table(264);
    let mut values = [9, 1, 7, 3];
    assert_eq!(
        brotli_build_simple_huffman_table(&mut table, 8, &mut values, 3),
        256
    );
    assert_eq!(values, [1, 3, 7, 9]);
    let codes = initialized_codes(&table);
    assert_eq!(
        &codes[..4],
        &[
            HuffmanCode::new(2, 1),
            HuffmanCode::new(2, 7),
            HuffmanCode::new(2, 3),
            HuffmanCode::new(2, 9)
        ]
    );
    for i in 4..256 {
        assert_eq!(codes[i], codes[i % 4]);
    }
    assert!(
        codes[256..]
            .iter()
            .all(|&code| code == HuffmanCode::new(165, 48879))
    );
}

#[test]
fn code_length_singleton_fills_all_slots_without_changing_counts() {
    let mut lengths = [0u8; 18];
    lengths[17] = 5;
    let (_, mut count) = symbol_lists(&lengths);
    let old_count = count;
    let mut table = initialized_table(40);
    brotli_build_code_lengths_huffman_table(&mut table, &lengths, &mut count);
    assert_eq!(count, old_count);
    let codes = initialized_codes(&table);
    assert!(
        codes[..32]
            .iter()
            .all(|&code| code == HuffmanCode::new(0, 17))
    );
    assert!(
        codes[32..]
            .iter()
            .all(|&code| code == HuffmanCode::new(165, 48879))
    );
}

#[test]
fn second_level_tables_consume_only_long_counts_and_preserve_symbol_links() {
    let lengths = [1, 2, 3, 4, 5, 6, 7, 8, 9, 9];
    let (links, mut count) = symbol_lists(&lengths);
    let old_count = count;
    let mut table = initialized_table(lengths.len() + 384);
    let size = brotli_build_huffman_table(&mut table, 8, &links, 16, &mut count);
    assert_eq!(size, 258);
    assert_eq!(&count[..9], &old_count[..9]);
    assert_eq!(count[9], 0);
    assert_eq!(links, symbol_lists(&lengths).0);
    let codes = initialized_codes(&table);
    assert_eq!(
        &codes[256..258],
        &[HuffmanCode::new(1, 8), HuffmanCode::new(1, 9)]
    );
    assert!(
        codes[258..]
            .iter()
            .all(|&code| code == HuffmanCode::new(165, 48879))
    );
}

#[test]
fn indexed_getter_reads_only_source_initialized_heads_and_forward_links() {
    let lengths = [1, 0, 2, 0, 3, 4, 5, 6, 7, 8, 9, 9];
    let (words, initialized) = partial_symbol_lists(&lengths);
    assert!(initialized[..16].iter().all(|&value| value));
    assert!(!initialized[16 + 1]); // Zero-length symbol.
    assert!(!initialized[16 + 3]); // Zero-length symbol.
    assert!(initialized[16 + 10]); // The sole nonterminal forward link.
    assert!(!initialized[16 + 11]); // Final link is never assigned in C.
    let (links, mut count) = symbol_lists(&lengths);
    let mut expected_count = count;
    let mut table = initialized_table(lengths.len() + 384);
    let mut expected_table = initialized_table(table.len());
    let expected_size =
        brotli_build_huffman_table(&mut expected_table, 8, &links, 16, &mut expected_count);
    let mut indices = Vec::new();
    let size = brotli_build_huffman_table_with_symbols(
        &mut table,
        8,
        |index| {
            indices.push(index);
            let slot = 16usize.checked_add_signed(index).unwrap();
            assert!(initialized[slot], "uninitialized symbol-list read: {index}");
            // SAFETY: the independent metadata is set only at the exact head
            // and forward-link writes above; the immediately preceding check
            // proves this slot was written. The borrowed arena remains live
            // and immutable throughout this getter call.
            unsafe { words[slot].assume_init() }
        },
        &mut count,
    );
    assert_eq!(size, expected_size);
    assert_eq!(count, expected_count);
    assert_eq!(
        initialized_codes(&table),
        initialized_codes(&expected_table)
    );
    assert_eq!(
        indices,
        [
            -1, -2, -3, -4, -5, -6, -7, -15, -14, -13, -12, -11, -10, -9, -8, -7, 10
        ]
    );
}

#[test]
fn distance_limits_preserve_the_small_direct_and_first_group_branches() {
    use constants::*;
    assert_eq!(
        brotli_calculate_distance_code_limit(2, 0, 5),
        BrotliDistanceCodeLimit {
            max_alphabet_size: 18,
            max_distance: 2
        }
    );
    assert_eq!(
        brotli_calculate_distance_code_limit(1, 0, 0),
        BrotliDistanceCodeLimit {
            max_alphabet_size: 16,
            max_distance: 0
        }
    );
    assert_eq!(
        BROTLI_PREFIX_CODE_RANGES[25],
        BrotliPrefixCodeRange {
            offset: 16625,
            nbits: 24
        }
    );
}
