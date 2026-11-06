// libwebp v1.6.0, src/utils/huffman_encode_utils.c:
//
// Copyright 2011 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// Author: Jyrki Alakuijala (jyrki@google.com)
//
// Entropy encoding (Huffman) for webp lossless.

// libwebp v1.6.0, src/utils/huffman_encode_utils.h:
//
// Copyright 2011 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// Author: Jyrki Alakuijala (jyrki@google.com)
//
// Entropy encoding (Huffman) for webp lossless

//! Translation of `src/utils/huffman_encode_utils.c` and `src/utils/huffman_encode_utils.h` from libwebp v1.6.0.
//!
//! Work package: WP3 (lossless shared).
//!
//! Representation: `HuffmanTreeCode` owns its code lengths and codes. The C
//! encoder carves both arrays of every code from one allocation; ADR 0003
//! lets the Rust caller check that total and allocate separate buffers.

use core::cmp::Ordering;

use crate::webp::format_constants::MAX_ALLOWED_CODE_LENGTH;

// upstream: src/utils/huffman_encode_utils.h HuffmanTreeToken
/// Struct for holding the tree header in coded form.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct HuffmanTreeToken {
    /// Value (0..15) or escape code (16, 17, 18).
    pub code: u8,
    /// Extra bits for escape codes.
    pub extra_bits: u8,
}

impl HuffmanTreeToken {
    /// The C `sizeof(HuffmanTreeToken)`, the `size` argument of its allocations.
    pub const C_SIZE: usize = 2;
}

// upstream: src/utils/huffman_encode_utils.h HuffmanTreeCode
/// Struct to represent the tree codes (depth and bits array).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct HuffmanTreeCode {
    /// Number of symbols.
    pub num_symbols: i32,
    /// Code lengths of the symbols; at least `num_symbols` of them.
    pub code_lengths: Vec<u8>,
    /// Symbol Codes; at least `num_symbols` of them.
    pub codes: Vec<u16>,
}

impl HuffmanTreeCode {
    /// The C `sizeof(HuffmanTreeCode)`: an `int` and two pointers, with the
    /// `int` padded to the pointer alignment.
    pub const C_SIZE: usize = 3 * size_of::<usize>();
}

// upstream: src/utils/huffman_encode_utils.h HuffmanTree
/// Struct to represent the Huffman tree.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct HuffmanTree {
    /// Symbol frequency.
    pub total_count: u32,
    /// Symbol value.
    pub value: i32,
    /// Index for the left sub-tree.
    pub pool_index_left: i32,
    /// Index for the right sub-tree.
    pub pool_index_right: i32,
}

impl HuffmanTree {
    /// The C `sizeof(HuffmanTree)`, the `size` argument of its allocations.
    pub const C_SIZE: usize = 16;
}

/// Converts an index of the tree construction, which is never negative, to a
/// slice position.
fn position(value: i32) -> usize {
    usize::try_from(value).expect("a tree index is not negative")
}

// -----------------------------------------------------------------------------
// Util function to optimize the symbol map for RLE coding

// upstream: src/utils/huffman_encode_utils.c ValuesShouldBeCollapsedToStrideAverage
/// Heuristics for selecting the stride ranges to collapse.
#[must_use]
fn values_should_be_collapsed_to_stride_average(a: i32, b: i32) -> bool {
    (a - b).abs() < 4
}

// upstream: src/utils/huffman_encode_utils.c OptimizeHuffmanForRle
/// Change the population counts in a way that the consequent Huffman tree
/// compression, especially its RLE-part, give smaller output.
///
/// The C `uint32_t` arithmetic wraps; the conversion of a count to the `int`
/// parameters of the heuristic keeps its bits, as Clang and GCC convert.
fn optimize_huffman_for_rle(mut length: i32, good_for_rle: &mut [u8], counts: &mut [u32]) {
    // 1) Let's make the Huffman code more compatible with rle encoding.
    while length >= 0 {
        if length == 0 {
            return; // All zeros.
        }
        if counts[position(length - 1)] != 0 {
            // Now counts[0..length - 1] does not have trailing zeros.
            break;
        }
        length -= 1;
    }
    // 2) Let's mark all population counts that already can be encoded
    // with an rle code.
    {
        // Let's not spoil any of the existing good rle codes.
        // Mark any seq of 0's that is longer as 5 as a good_for_rle.
        // Mark any seq of non-0's that is longer as 7 as a good_for_rle.
        let mut symbol: u32 = counts[0];
        let mut stride: i32 = 0;
        for i in 0..=length {
            if i == length || counts[position(i)] != symbol {
                if (symbol == 0 && stride >= 5) || (symbol != 0 && stride >= 7) {
                    for k in 0..stride {
                        good_for_rle[position(i - k - 1)] = 1;
                    }
                }
                stride = 1;
                if i != length {
                    symbol = counts[position(i)];
                }
            } else {
                stride += 1;
            }
        }
    }
    // 3) Let's replace those population counts that lead to more rle codes.
    {
        let mut stride: u32 = 0;
        let mut limit: u32 = counts[0];
        let mut sum: u32 = 0;
        for i in 0..=length {
            if i == length
                || good_for_rle[position(i)] != 0
                || (i != 0 && good_for_rle[position(i - 1)] != 0)
                || !values_should_be_collapsed_to_stride_average(
                    counts[position(i)].cast_signed(),
                    limit.cast_signed(),
                )
            {
                if stride >= 4 || (stride >= 3 && sum == 0) {
                    // The stride must end, collapse what we have, if we have enough (4).
                    let mut count: u32 = sum.wrapping_add(stride / 2) / stride;
                    if count < 1 {
                        count = 1;
                    }
                    if sum == 0 {
                        // Don't make an all zeros stride to be upgraded to ones.
                        count = 0;
                    }
                    for k in 0..stride {
                        // We don't want to change value at counts[i],
                        // that is already belonging to the next stride. Thus - 1.
                        counts[position(i) - k as usize - 1] = count;
                    }
                }
                stride = 0;
                sum = 0;
                if i < length - 3 {
                    // All interesting strides have a count of at least 4,
                    // at least when non-zeros.
                    let at = position(i);
                    limit = counts[at]
                        .wrapping_add(counts[at + 1])
                        .wrapping_add(counts[at + 2])
                        .wrapping_add(counts[at + 3])
                        .wrapping_add(2)
                        / 4;
                } else if i < length {
                    limit = counts[position(i)];
                } else {
                    limit = 0;
                }
            }
            stride += 1;
            if i != length {
                sum = sum.wrapping_add(counts[position(i)]);
                if stride >= 4 {
                    limit = sum.wrapping_add(stride / 2) / stride;
                }
            }
        }
    }
}

// upstream: src/utils/huffman_encode_utils.c CompareHuffmanTrees
/// A comparer function for two Huffman trees: sorts first by `total_count`
/// (more comes first), and then by `value` (more comes first).
///
/// The values of the sorted leaves are distinct symbols, so no two trees
/// compare equal and every sort algorithm gives the order of the C `qsort`.
#[expect(
    clippy::comparison_chain,
    reason = "mirrors the if chain of the C comparator"
)]
fn compare_huffman_trees(t1: &HuffmanTree, t2: &HuffmanTree) -> Ordering {
    if t1.total_count > t2.total_count {
        Ordering::Less
    } else if t1.total_count < t2.total_count {
        Ordering::Greater
    } else {
        debug_assert_ne!(t1.value, t2.value);
        if t1.value < t2.value {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

// upstream: src/utils/huffman_encode_utils.c SetBitDepths
/// Stores the depth of every leaf under `tree`, whose sub-trees are in `pool`.
/// The depth is truncated to the `uint8_t` of `bit_depths`, as in C.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "the C conversion of the int level to uint8_t"
)]
fn set_bit_depths(tree: &HuffmanTree, pool: &[HuffmanTree], bit_depths: &mut [u8], level: i32) {
    if tree.pool_index_left >= 0 {
        set_bit_depths(
            &pool[position(tree.pool_index_left)],
            pool,
            bit_depths,
            level + 1,
        );
        set_bit_depths(
            &pool[position(tree.pool_index_right)],
            pool,
            bit_depths,
            level + 1,
        );
    } else {
        bit_depths[position(tree.value)] = level as u8;
    }
}

// upstream: src/utils/huffman_encode_utils.c GenerateOptimalTree
/// Create an optimal Huffman tree.
///
/// (histogram, `histogram_size`): population counts.
/// `tree_depth_limit`: maximum bit depth (inclusive) of the codes.
/// `bit_depths[]`: how many bits are used for the symbol.
///
/// The catch here is that the tree cannot be arbitrarily deep.
///
/// `count_min` is the value that is to be faked as the minimum value
/// and this minimum value is raised until the tree matches the
/// maximum length requirement.
///
/// This algorithm is not of excellent performance for very long data blocks,
/// especially when population counts are longer than 2**`tree_limit`, but
/// we are not planning to use this with extremely long blocks.
///
/// See <https://en.wikipedia.org/wiki/Huffman_coding>
///
/// The leaves are the first `tree_size_orig` entries of `tree`, and the pool of
/// sub-trees follows them, as in C. The depths of symbols with a zero count are
/// left as they are.
fn generate_optimal_tree(
    histogram: &[u32],
    histogram_size: i32,
    tree: &mut [HuffmanTree],
    tree_depth_limit: i32,
    bit_depths: &mut [u8],
) {
    let histogram = &histogram[..position(histogram_size)];
    let mut tree_size_orig: i32 = 0;

    for &count in histogram {
        if count != 0 {
            tree_size_orig += 1;
        }
    }

    if tree_size_orig == 0 {
        // pretty optimal already!
        return;
    }

    let (tree, tree_pool) = tree.split_at_mut(position(tree_size_orig));

    // For block sizes with less than 64k symbols we never need to do a
    // second iteration of this loop.
    // If we actually start running inside this loop a lot, we would perhaps
    // be better off with the Katajainen algorithm.
    debug_assert!(tree_size_orig <= (1 << (tree_depth_limit - 1)));
    let mut count_min: u32 = 1;
    loop {
        let mut tree_size = tree_size_orig;
        // We need to pack the Huffman tree in tree_depth_limit bits.
        // So, we try by faking histogram entries to be at least 'count_min'.
        let mut idx: usize = 0;
        for (j, &count) in histogram.iter().enumerate() {
            if count != 0 {
                let count = if count < count_min { count_min } else { count };
                tree[idx].total_count = count;
                tree[idx].value = i32::try_from(j).expect("a symbol fits an int");
                tree[idx].pool_index_left = -1;
                tree[idx].pool_index_right = -1;
                idx += 1;
            }
        }

        // Build the Huffman tree.
        tree[..position(tree_size)].sort_unstable_by(compare_huffman_trees);

        if tree_size > 1 {
            // Normal case.
            let mut tree_pool_size: i32 = 0;
            while tree_size > 1 {
                // Finish when we have only one root.
                tree_pool[position(tree_pool_size)] = tree[position(tree_size - 1)];
                tree_pool_size += 1;
                tree_pool[position(tree_pool_size)] = tree[position(tree_size - 2)];
                tree_pool_size += 1;
                let count: u32 = tree_pool[position(tree_pool_size - 1)]
                    .total_count
                    .wrapping_add(tree_pool[position(tree_pool_size - 2)].total_count);
                tree_size -= 2;
                {
                    // Search for the insertion point.
                    let mut k: i32 = 0;
                    while k < tree_size {
                        if tree[position(k)].total_count <= count {
                            break;
                        }
                        k += 1;
                    }
                    tree.copy_within(position(k)..position(tree_size), position(k + 1));
                    let node = &mut tree[position(k)];
                    node.total_count = count;
                    node.value = -1;

                    node.pool_index_left = tree_pool_size - 1;
                    node.pool_index_right = tree_pool_size - 2;
                    tree_size += 1;
                }
            }
            set_bit_depths(&tree[0], tree_pool, bit_depths, 0);
        } else if tree_size == 1 {
            // Trivial case: only one element.
            bit_depths[position(tree[0].value)] = 1;
        }

        {
            // Test if this Huffman tree satisfies our 'tree_depth_limit' criteria.
            let mut max_depth = i32::from(bit_depths[0]);
            for &depth in &bit_depths[1..histogram.len()] {
                if max_depth < i32::from(depth) {
                    max_depth = i32::from(depth);
                }
            }
            if max_depth <= tree_depth_limit {
                break;
            }
        }
        count_min = count_min.wrapping_mul(2);
    }
}

// -----------------------------------------------------------------------------
// Coding of the Huffman tree values

/// Truncates a code or an extra-bits value to the `uint8_t` of
/// `HuffmanTreeToken`, as the C assignment does.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "the C conversion of an int to uint8_t"
)]
fn token_byte(value: i32) -> u8 {
    value as u8
}

// upstream: src/utils/huffman_encode_utils.c CodeRepeatedValues
/// Writes the tokens for `repetitions` copies of `value` at `tokens[pos..]`
/// and returns the position after them.
fn code_repeated_values(
    mut repetitions: i32,
    tokens: &mut [HuffmanTreeToken],
    mut pos: usize,
    value: i32,
    prev_value: i32,
) -> usize {
    debug_assert!(value <= MAX_ALLOWED_CODE_LENGTH);
    if value != prev_value {
        tokens[pos].code = token_byte(value);
        tokens[pos].extra_bits = 0;
        pos += 1;
        repetitions -= 1;
    }
    while repetitions >= 1 {
        if repetitions < 3 {
            for _ in 0..repetitions {
                tokens[pos].code = token_byte(value);
                tokens[pos].extra_bits = 0;
                pos += 1;
            }
            break;
        } else if repetitions < 7 {
            tokens[pos].code = 16;
            tokens[pos].extra_bits = token_byte(repetitions - 3);
            pos += 1;
            break;
        }
        tokens[pos].code = 16;
        tokens[pos].extra_bits = 3;
        pos += 1;
        repetitions -= 6;
    }
    pos
}

// upstream: src/utils/huffman_encode_utils.c CodeRepeatedZeros
/// Writes the tokens for `repetitions` zeros at `tokens[pos..]` and returns
/// the position after them.
fn code_repeated_zeros(
    mut repetitions: i32,
    tokens: &mut [HuffmanTreeToken],
    mut pos: usize,
) -> usize {
    while repetitions >= 1 {
        if repetitions < 3 {
            for _ in 0..repetitions {
                tokens[pos].code = 0; // 0-value
                tokens[pos].extra_bits = 0;
                pos += 1;
            }
            break;
        } else if repetitions < 11 {
            tokens[pos].code = 17;
            tokens[pos].extra_bits = token_byte(repetitions - 3);
            pos += 1;
            break;
        } else if repetitions < 139 {
            tokens[pos].code = 18;
            tokens[pos].extra_bits = token_byte(repetitions - 11);
            pos += 1;
            break;
        }
        tokens[pos].code = 18;
        tokens[pos].extra_bits = 0x7f; // 138 repeated 0s
        pos += 1;
        repetitions -= 138;
    }
    pos
}

// upstream: src/utils/huffman_encode_utils.c VP8LCreateCompressedHuffmanTree
/// Turn the Huffman tree into a token sequence.
/// Returns the number of tokens used.
///
/// `tokens` holds the `max_tokens` tokens of the C call.
///
/// # Panics
///
/// Panics when the tokens do not fit, where C asserts and writes past
/// `max_tokens`.
#[must_use]
pub fn vp8l_create_compressed_huffman_tree(
    tree: &HuffmanTreeCode,
    tokens: &mut [HuffmanTreeToken],
) -> i32 {
    let ending_token = tokens.len();
    let depth_size = tree.num_symbols;
    let mut prev_value: i32 = 8; // 8 is the initial value for rle.
    let mut pos: usize = 0;
    let mut i: i32 = 0;
    while i < depth_size {
        let value = i32::from(tree.code_lengths[position(i)]);
        let mut k = i + 1;
        while k < depth_size && i32::from(tree.code_lengths[position(k)]) == value {
            k += 1;
        }
        let runs = k - i;
        if value == 0 {
            pos = code_repeated_zeros(runs, tokens, pos);
        } else {
            pos = code_repeated_values(runs, tokens, pos, value, prev_value);
            prev_value = value;
        }
        i += runs;
        debug_assert!(pos <= ending_token);
    }
    i32::try_from(pos).expect("the token count fits an int")
}

// -----------------------------------------------------------------------------

// upstream: src/utils/huffman_encode_utils.c kReversedBits
/// Pre-reversed 4-bit values.
const REVERSED_BITS: [u8; 16] = [
    0x0, 0x8, 0x4, 0xc, 0x2, 0xa, 0x6, 0xe, 0x1, 0x9, 0x5, 0xd, 0x3, 0xb, 0x7, 0xf,
];

// upstream: src/utils/huffman_encode_utils.c ReverseBits
/// Reverses the `num_bits` low bits of `bits`.
#[must_use]
fn reverse_bits(num_bits: i32, mut bits: u32) -> u32 {
    let mut retval: u32 = 0;
    let mut i: i32 = 0;
    while i < num_bits {
        i += 4;
        retval |=
            u32::from(REVERSED_BITS[(bits & 0xf) as usize]) << (MAX_ALLOWED_CODE_LENGTH + 1 - i);
        bits >>= 4;
    }
    retval >>= MAX_ALLOWED_CODE_LENGTH + 1 - num_bits;
    retval
}

// upstream: src/utils/huffman_encode_utils.c ConvertBitDepthsToSymbols
/// Get the actual bit values for a tree of bit depths.
#[expect(
    clippy::cast_possible_truncation,
    reason = "the C conversion of the reversed code to uint16_t"
)]
fn convert_bit_depths_to_symbols(tree: &mut HuffmanTreeCode) {
    const DEPTHS: usize = MAX_ALLOWED_CODE_LENGTH as usize + 1;
    // 0 bit-depth means that the symbol does not exist.
    let mut next_code = [0_u32; DEPTHS];
    let mut depth_count = [0_i32; DEPTHS];

    let len = position(tree.num_symbols);
    for &code_length in &tree.code_lengths[..len] {
        debug_assert!(i32::from(code_length) <= MAX_ALLOWED_CODE_LENGTH);
        depth_count[usize::from(code_length)] += 1;
    }
    depth_count[0] = 0; // ignore unused symbol
    next_code[0] = 0;
    {
        let mut code: u32 = 0;
        for i in 1..DEPTHS {
            code = code.wrapping_add(depth_count[i - 1].cast_unsigned()) << 1;
            next_code[i] = code;
        }
    }
    for i in 0..len {
        let code_length = usize::from(tree.code_lengths[i]);
        tree.codes[i] =
            reverse_bits(i32::from(tree.code_lengths[i]), next_code[code_length]) as u16;
        next_code[code_length] = next_code[code_length].wrapping_add(1);
    }
}

// -----------------------------------------------------------------------------
// Main entry point

// upstream: src/utils/huffman_encode_utils.c VP8LCreateHuffmanTree
/// Create an optimized tree, and tokenize it.
/// `buf_rle` and `huff_tree` are pre-allocated and the `tree` is the
/// constructed huffman code tree.
///
/// `histogram` is changed by the RLE optimization, as in C. `huff_tree` needs
/// room for the leaves and the pool: `3 * num_symbols` entries suffice.
pub fn vp8l_create_huffman_tree(
    histogram: &mut [u32],
    tree_depth_limit: i32,
    buf_rle: &mut [u8],
    huff_tree: &mut [HuffmanTree],
    huff_code: &mut HuffmanTreeCode,
) {
    let num_symbols = huff_code.num_symbols;
    buf_rle[..position(num_symbols)].fill(0);
    optimize_huffman_for_rle(num_symbols, buf_rle, histogram);
    generate_optimal_tree(
        histogram,
        num_symbols,
        huff_tree,
        tree_depth_limit,
        &mut huff_code.code_lengths,
    );
    // Create the actual bit codes for the bit lengths.
    convert_bit_depths_to_symbols(huff_code);
}
