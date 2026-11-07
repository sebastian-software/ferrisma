// libwebp v1.6.0, tests/fuzzer/huffman_fuzzer.cc:
//
// Copyright 2023 Google Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
////////////////////////////////////////////////////////////////////////////////

//! Port of `tests/fuzzer/huffman_fuzzer.cc`: `ReadHuffmanCodesHelper` on a
//! color cache size and a number of Huffman tree groups read from the input.

#![no_main]

use libfuzzer_sys::fuzz_target;
use webp::dec::vp8l_dec::{read_huffman_codes_helper, vp8l_new};
use webp::utils::bit_reader_utils::{vp8l_init_bit_reader, vp8l_read_bits};
use webp::utils::huffman_utils::HuffmanTables;
use webp::webp::format_constants::MAX_CACHE_BITS;

// upstream: tests/fuzzer/huffman_fuzzer.cc HuffmanTest
fuzz_target!(|data: &[u8]| {
    let size = data.len();

    // Number of bits to initialize data.
    const COLOR_CACHE_BITS_BITS: i32 = 4;
    // 'num_htree_groups' is contained in the RG channel, hence 16 bits.
    const NUM_HTREE_GROUPS_BITS: i32 = 16;
    if size < (COLOR_CACHE_BITS_BITS + NUM_HTREE_GROUPS_BITS) as usize {
        return;
    }

    // A non-NULL mapping brings minor changes that are tested by the normal
    // fuzzer.
    let mapping = None;
    let mut huffman_tables = HuffmanTables::default();
    let mut htree_groups = None;

    let mut dec = vp8l_new();
    vp8l_init_bit_reader(&mut dec.br, data);

    let color_cache_bits = vp8l_read_bits(&mut dec.br, data, COLOR_CACHE_BITS_BITS) as i32;
    if !(1..=MAX_CACHE_BITS).contains(&color_cache_bits) {
        return;
    }

    let num_htree_groups = vp8l_read_bits(&mut dec.br, data, NUM_HTREE_GROUPS_BITS) as i32;
    // 'num_htree_groups' cannot be 0 as it is built from a non-empty image.
    if num_htree_groups == 0 {
        return;
    }
    // This variable is only useful when mapping is not NULL.
    let num_htree_groups_max = num_htree_groups;
    let _ = read_huffman_codes_helper(
        color_cache_bits,
        num_htree_groups,
        num_htree_groups_max,
        mapping,
        &mut dec,
        data,
        &mut huffman_tables,
        &mut htree_groups,
    );
    // C: WebPSafeFree(mapping), VP8LHtreeGroupsFree, VP8LHuffmanTablesDeallocate
    // and VP8LDelete; the values are dropped.
});
