// libwebp v1.6.0, src/enc/histogram_enc.c:
//
// Copyright 2012 Google Inc. All Rights Reserved.
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

// libwebp v1.6.0, src/enc/histogram_enc.h:
//
// Copyright 2012 Google Inc. All Rights Reserved.
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
// Models the histograms of literal and distance codes.

//! Translation of `src/enc/histogram_enc.c` and `src/enc/histogram_enc.h` from libwebp v1.6.0.
//!
//! Work package: WP11 (VP8L encoder).
//!
//! # Deviations from upstream
//!
//! Changes of representation that the ADRs allow, or behavior that the Rust
//! side cannot reproduce exactly. Each records the reason.
//!
//! 1. **Free-only functions.** `VP8LFreeHistogram`, `VP8LFreeHistogramSet` and
//!    `VP8LEncoderDelete` only free; their owners drop the values at the same
//!    points, as for `VP8LHtreeGroupsFree` (`src/utils/huffman_utils.rs`).
//!    `DeleteIntervalList` is kept, but the intervals live in the manager's
//!    arena until `CostManagerClear` drops it.
//!
//! # Checks not applicable
//!
//! These checks do not apply, or cannot be made, with the reason:
//!
//! 1. **Floating point.** `GetBinIdForEntropy` computes in `double`, and the
//!    slow logarithms use `log` above 65,535. They are compared bit for bit on
//!    the development host (macOS on Apple Silicon, Apple clang,
//!    `-ffp-contract=off`), through the clustering and the encoded files;
//!    equality on other hosts is recorded, not assumed.

use crate::dsp::lossless::{VP8LBitEntropy, VP8LStreaks};
use crate::dsp::lossless_common::{
    LOG_2_PRECISION_BITS, WEBP_INT64_MAX, WEBP_UINT64_MAX, div_round, vp8l_prefix_encode_bits,
    vp8l_sub_sample_size,
};
use crate::dsp::lossless_enc::{
    vp8l_add_vector, vp8l_add_vector_eq, vp8l_bits_entropy_unrefined, vp8l_extra_cost,
    vp8l_get_combined_entropy_unrefined, vp8l_get_entropy_unrefined,
};
use crate::enc::backward_references_enc::{
    PixOrCopy, VP8LBackwardRefs, pix_or_copy_cache_idx, pix_or_copy_distance,
    pix_or_copy_is_cache_idx, pix_or_copy_is_literal, pix_or_copy_length, pix_or_copy_literal,
    safe_malloc_test, split_pair, to_index, try_filled, vp8l_refs_cursor_init,
    vp8l_refs_cursor_next, vp8l_refs_cursor_ok,
};
use crate::enc::webp_enc::{webp_encoding_set_error, webp_report_progress};
use crate::utils::utils::{WEBP_ALIGN_CST, webp_safe_malloc};
use crate::webp::encode::{WebPEncodingError, WebPPicture};
use crate::webp::format_constants::{
    CODE_LENGTH_CODES, NUM_DISTANCE_CODES, NUM_LENGTH_CODES, NUM_LITERAL_CODES,
};

//------------------------------------------------------------------------------
// histogram_enc.h

// upstream: src/enc/histogram_enc.h VP8L_NON_TRIVIAL_SYM
/// Not a trivial literal symbol.
pub const VP8L_NON_TRIVIAL_SYM: u16 = 0xffff;

// upstream: src/enc/histogram_enc.h VP8LHistogram
/// A simple container for histograms of data.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VP8LHistogram {
    /// `literal` contains green literal, palette-code and copy-length-prefix
    /// histogram. It holds `VP8LHistogramNumCodes` entries of the cache bits
    /// that the histogram was allocated with (C: the allocated buffer).
    pub literal: Vec<u32>,
    pub red: [u32; NUM_LITERAL_CODES as usize],
    pub blue: [u32; NUM_LITERAL_CODES as usize],
    pub alpha: [u32; NUM_LITERAL_CODES as usize],
    /// Backward reference prefix-code histogram.
    pub distance: [u32; NUM_DISTANCE_CODES as usize],
    pub palette_code_bits: i32,
    // The following members are only used within VP8LGetHistoImageSymbols.
    /// Index of the unique value of a histogram if any, `VP8L_NON_TRIVIAL_SYM`
    /// otherwise.
    pub trivial_symbol: [u16; 5],
    /// Cached value of total bit cost.
    pub bit_cost: u64,
    /// Cached values of entropy costs: literal, red, blue, alpha, distance
    pub costs: [u64; 5],
    /// 5 for literal, red, blue, alpha, distance
    pub is_used: [u8; 5],
    /// entropy bin index.
    pub bin_id: u16,
}

impl Default for VP8LHistogram {
    fn default() -> Self {
        Self {
            literal: Vec::new(),
            red: [0; NUM_LITERAL_CODES as usize],
            blue: [0; NUM_LITERAL_CODES as usize],
            alpha: [0; NUM_LITERAL_CODES as usize],
            distance: [0; NUM_DISTANCE_CODES as usize],
            palette_code_bits: 0,
            trivial_symbol: [0; 5],
            bit_cost: 0,
            costs: [0; 5],
            is_used: [0; 5],
            bin_id: 0,
        }
    }
}

impl VP8LHistogram {
    /// The C `sizeof(VP8LHistogram)` on 64-bit targets, which the probe checks.
    pub const C_SIZE: usize = 3312;
}

// upstream: src/enc/histogram_enc.h VP8LHistogramSet
/// Collection of histograms with fixed capacity, allocated as one big memory
/// chunk in C. The Rust set owns its `max_size` histograms in `memory`, each in
/// its own box, and `histograms` is C's array of pointers into that memory: an
/// index into `memory`, `None` where C has `NULL`. As in C, two slots can refer
/// to the same histogram: `HistogramSetRemoveHistogram` copies the last
/// pointer, and the slot behind `size` keeps referring to the moved histogram.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct VP8LHistogramSet {
    /// number of slots currently in use
    pub size: i32,
    /// maximum capacity
    pub max_size: i32,
    /// The pointers of the C set, as indices into `memory`.
    pub histograms: Vec<Option<usize>>,
    /// The histograms of the C memory chunk.
    pub memory: Vec<Box<VP8LHistogram>>,
}

impl VP8LHistogramSet {
    /// The C `sizeof(VP8LHistogramSet)`: two `int` fields and a pointer.
    pub const C_SIZE: usize = 8 + size_of::<usize>();

    /// The histogram at slot `i` (C: `set->histograms[i]`).
    ///
    /// # Panics
    ///
    /// Panics when the slot is `NULL`.
    #[must_use]
    pub fn histogram(&self, i: usize) -> &VP8LHistogram {
        &self.memory[self.histograms[i].expect("the histogram slot is not NULL")]
    }

    /// The histogram at slot `i`, mutably.
    ///
    /// # Panics
    ///
    /// Panics when the slot is `NULL`.
    pub fn histogram_mut(&mut self, i: usize) -> &mut VP8LHistogram {
        &mut self.memory[self.histograms[i].expect("the histogram slot is not NULL")]
    }
}

// upstream: src/enc/histogram_enc.h VP8LHistogramNumCodes
#[must_use]
#[inline]
pub fn vp8l_histogram_num_codes(palette_code_bits: i32) -> i32 {
    NUM_LITERAL_CODES
        + NUM_LENGTH_CODES
        + if palette_code_bits > 0 {
            1 << palette_code_bits
        } else {
            0
        }
}

//------------------------------------------------------------------------------
// histogram_enc.c

// upstream: src/enc/histogram_enc.c NUM_PARTITIONS
/// Number of partitions for the three dominant (literal, red and blue) symbol
/// costs.
const NUM_PARTITIONS: i32 = 4;
// upstream: src/enc/histogram_enc.c BIN_SIZE
/// The size of the bin-hash corresponding to the three dominant costs.
const BIN_SIZE: i32 = NUM_PARTITIONS * NUM_PARTITIONS * NUM_PARTITIONS;
// upstream: src/enc/histogram_enc.c MAX_HISTO_GREEDY
/// Maximum number of histograms allowed in greedy combining algorithm.
const MAX_HISTO_GREEDY: i32 = 100;

// upstream: src/enc/histogram_enc.c HistogramIndex
/// Enum to meaningfully access the elements of the Histogram arrays.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct HistogramIndex(u32);

impl HistogramIndex {
    // upstream: src/enc/histogram_enc.c LITERAL
    const LITERAL: Self = Self(0);
    // upstream: src/enc/histogram_enc.c RED
    const RED: Self = Self(1);
    // upstream: src/enc/histogram_enc.c BLUE
    const BLUE: Self = Self(2);
    // upstream: src/enc/histogram_enc.c ALPHA
    const ALPHA: Self = Self(3);
    // upstream: src/enc/histogram_enc.c DISTANCE
    const DISTANCE: Self = Self(4);

    /// The C `(HistogramIndex)i` of a loop index.
    #[expect(
        clippy::cast_possible_truncation,
        reason = "the loop indexes are below 5"
    )]
    const fn of(i: usize) -> Self {
        Self(i as u32)
    }
}

// upstream: src/enc/histogram_enc.c GetHistogramSize
/// Return the size of the histogram for a given `cache_bits`.
fn get_histogram_size(cache_bits: i32) -> i32 {
    let literal_size = vp8l_histogram_num_codes(cache_bits);
    let total_size = VP8LHistogram::C_SIZE + 4 * to_index(literal_size);
    debug_assert!(total_size <= 0x7fff_ffff);
    i32::try_from(total_size).expect("the histogram size fits an int")
}

// upstream: src/enc/histogram_enc.c HistogramStatsClear
fn histogram_stats_clear(h: &mut VP8LHistogram) {
    for i in 0..5 {
        h.trivial_symbol[i] = VP8L_NON_TRIVIAL_SYM;
        // By default, the histogram is assumed to be used.
        h.is_used[i] = 1;
    }
    h.bit_cost = 0;
    h.costs = [0; 5];
}

// upstream: src/enc/histogram_enc.c HistogramClear
/// Zeroes the histogram with the first `VP8LHistogramNumCodes(palette_code_bits)`
/// literal counts, which is what the C `memset` of `GetHistogramSize` bytes
/// covers, and keeps `palette_code_bits`.
fn histogram_clear(h: &mut VP8LHistogram) {
    let cache_bits = h.palette_code_bits;
    let literal_size = to_index(vp8l_histogram_num_codes(cache_bits));
    let mut literal = std::mem::take(&mut h.literal);
    literal[..literal_size].fill(0);
    *h = VP8LHistogram {
        literal,
        palette_code_bits: cache_bits,
        ..VP8LHistogram::default()
    };
    histogram_stats_clear(h);
}

// upstream: src/enc/histogram_enc.c HistogramSwap
/// Swap two histogram pointers. C swaps the pointer `cur_combo` with the set's
/// pointer `histograms[first]`; here the two boxes swap, so the set's memory
/// holds the combined histogram where `histograms[first]` points, and
/// `cur_combo` holds the previous one. No other slot below `size` points there.
fn histogram_swap(h1: &mut Box<VP8LHistogram>, h2: &mut Box<VP8LHistogram>) {
    std::mem::swap(h1, h2);
}

// upstream: src/enc/histogram_enc.c HistogramCopy
fn histogram_copy(src: &VP8LHistogram, dst: &mut VP8LHistogram) {
    let dst_cache_bits = dst.palette_code_bits;
    let literal_size = to_index(vp8l_histogram_num_codes(dst_cache_bits));
    debug_assert_eq!(src.palette_code_bits, dst_cache_bits);
    dst.literal[..literal_size].copy_from_slice(&src.literal[..literal_size]);
    dst.red = src.red;
    dst.blue = src.blue;
    dst.alpha = src.alpha;
    dst.distance = src.distance;
    dst.palette_code_bits = src.palette_code_bits;
    dst.trivial_symbol = src.trivial_symbol;
    dst.bit_cost = src.bit_cost;
    dst.costs = src.costs;
    dst.is_used = src.is_used;
    dst.bin_id = src.bin_id;
}

// upstream: src/enc/histogram_enc.c VP8LHistogramCreate
/// Create the histogram.
///
/// The input data is the `PixOrCopy` data, which models the literals, stop
/// codes and backward references (both distances and lengths). Also: if
/// `palette_code_bits` is >= 0, initialize the histogram with this value.
pub fn vp8l_histogram_create(
    h: &mut VP8LHistogram,
    refs: &VP8LBackwardRefs,
    palette_code_bits: i32,
) {
    if palette_code_bits >= 0 {
        h.palette_code_bits = palette_code_bits;
    }
    histogram_clear(h);
    vp8l_histogram_store_refs(
        refs, /*distance_modifier=*/ None, /*distance_modifier_arg0=*/ 0, h,
    );
}

// upstream: src/enc/histogram_enc.c VP8LHistogramInit
/// Set the `palette_code_bits` and reset the stats.
/// If `init_arrays` is true, the arrays are also filled with 0's.
pub fn vp8l_histogram_init(h: &mut VP8LHistogram, palette_code_bits: i32, init_arrays: bool) {
    h.palette_code_bits = palette_code_bits;
    if init_arrays {
        histogram_clear(h);
    } else {
        histogram_stats_clear(h);
    }
}

/// A histogram whose literal array holds the codes of `cache_bits`, with every
/// count zero: Rust initializes what C leaves to `malloc`. `None` when the host
/// refuses the literal array.
fn new_histogram(cache_bits: i32) -> Option<Box<VP8LHistogram>> {
    let literal = try_filled(to_index(vp8l_histogram_num_codes(cache_bits)), 0_u32)?;
    Some(Box::new(VP8LHistogram {
        literal,
        ..VP8LHistogram::default()
    }))
}

// upstream: src/enc/histogram_enc.c VP8LAllocateHistogram
/// Allocate and initialize histogram object with specified `cache_bits`.
/// Returns `None` in case of memory error.
/// Special case of `VP8LAllocateHistogramSet`, with size equals 1.
#[must_use]
pub fn vp8l_allocate_histogram(cache_bits: i32) -> Option<Box<VP8LHistogram>> {
    let total_size = get_histogram_size(cache_bits);
    if !safe_malloc_test(i64::from(total_size).cast_unsigned(), 1) {
        return None;
    }
    let mut histo = new_histogram(cache_bits)?;
    vp8l_histogram_init(&mut histo, cache_bits, /*init_arrays=*/ false);
    Some(histo)
}

// upstream: src/enc/histogram_enc.c HistogramSetTotalSize
/// Returns the total size of the `VP8LHistogramSet`, with the `size_t`
/// arithmetic of C.
fn histogram_set_total_size(size: i32, cache_bits: i32) -> u64 {
    let histo_size = i64::from(get_histogram_size(cache_bits)).cast_unsigned();
    (VP8LHistogramSet::C_SIZE as u64).wrapping_add(
        i64::from(size).cast_unsigned().wrapping_mul(
            (size_of::<usize>() as u64)
                .wrapping_add(histo_size)
                .wrapping_add(WEBP_ALIGN_CST as u64),
        ),
    )
}

// upstream: src/enc/histogram_enc.c VP8LAllocateHistogramSet
/// Allocate an array of pointer to histograms, allocated and initialized
/// using `cache_bits`. Return `None` in case of memory error. The total of the
/// C block is tested before the histograms are allocated one by one.
#[must_use]
pub fn vp8l_allocate_histogram_set(size: i32, cache_bits: i32) -> Option<VP8LHistogramSet> {
    let total_size = histogram_set_total_size(size, cache_bits);
    if !safe_malloc_test(total_size, 1) {
        return None;
    }
    let count = to_index(size);
    let mut memory = Vec::new();
    memory.try_reserve_exact(count).ok()?;
    for _ in 0..count {
        let mut histo = new_histogram(cache_bits)?;
        vp8l_histogram_init(&mut histo, cache_bits, /*init_arrays=*/ false);
        memory.push(histo);
    }
    // C: HistogramSetResetPointers.
    let histograms = try_filled(count, None)?;
    let mut set = VP8LHistogramSet {
        size,
        max_size: size,
        histograms,
        memory,
    };
    for (i, slot) in set.histograms.iter_mut().enumerate() {
        *slot = Some(i);
    }
    Some(set)
}

// upstream: src/enc/histogram_enc.c VP8LHistogramSetClear
/// Set the histograms in set to 0: every field, the whole literal arrays and
/// the stats (C clears the whole block), and then `palette_code_bits`; all
/// `max_size` slots hold their histograms again (C: `HistogramSetResetPointers`).
///
/// # Panics
///
/// Panics on a `NULL` slot, which only the original histograms of
/// `VP8LGetHistoImageSymbols` have, and which C never clears.
pub fn vp8l_histogram_set_clear(set: &mut VP8LHistogramSet) {
    let cache_bits = set.histogram(0).palette_code_bits;
    let size = set.max_size;
    for (i, slot) in set.histograms.iter_mut().enumerate() {
        // C: HistogramSetResetPointers.
        *slot = Some(i);
    }
    for histo in &mut set.memory {
        let mut literal = std::mem::take(&mut histo.literal);
        literal.fill(0);
        **histo = VP8LHistogram {
            literal,
            palette_code_bits: cache_bits,
            ..VP8LHistogram::default()
        };
    }
    set.max_size = size;
    set.size = size;
}

// upstream: src/enc/histogram_enc.c HistogramSetRemoveHistogram
/// Removes the histogram `i` from `set`. The last pointer is copied into slot
/// `i` and stays in the slot behind the new `size`, as in C:
/// `HistogramCombineStochastic` reads that slot before it renumbers its pairs.
fn histogram_set_remove_histogram(set: &mut VP8LHistogramSet, i: usize) {
    set.histograms[i] = set.histograms[to_index(set.size - 1)];
    set.size -= 1;
    debug_assert!(set.size > 0);
}

// -----------------------------------------------------------------------------

// upstream: src/enc/histogram_enc.c HistogramAddSinglePixOrCopy
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "C passes the entry by pointer"
)]
fn histogram_add_single_pix_or_copy(
    histo: &mut VP8LHistogram,
    v: &PixOrCopy,
    distance_modifier: Option<fn(i32, i32) -> i32>,
    distance_modifier_arg0: i32,
) {
    if pix_or_copy_is_literal(v) {
        histo.alpha[pix_or_copy_literal(v, 3) as usize] += 1;
        histo.red[pix_or_copy_literal(v, 2) as usize] += 1;
        histo.literal[pix_or_copy_literal(v, 1) as usize] += 1;
        histo.blue[pix_or_copy_literal(v, 0) as usize] += 1;
    } else if pix_or_copy_is_cache_idx(v) {
        let literal_ix =
            to_index(NUM_LITERAL_CODES + NUM_LENGTH_CODES) + pix_or_copy_cache_idx(v) as usize;
        debug_assert!(histo.palette_code_bits != 0);
        histo.literal[literal_ix] += 1;
    } else {
        let (code, _extra_bits) = vp8l_prefix_encode_bits(pix_or_copy_length(v).cast_signed());
        histo.literal[to_index(NUM_LITERAL_CODES + code)] += 1;
        let distance = pix_or_copy_distance(v).cast_signed();
        let (code, _extra_bits) = match distance_modifier {
            None => vp8l_prefix_encode_bits(distance),
            Some(modifier) => vp8l_prefix_encode_bits(modifier(distance_modifier_arg0, distance)),
        };
        histo.distance[to_index(code)] += 1;
    }
}

// upstream: src/enc/histogram_enc.c VP8LHistogramStoreRefs
/// Collect all the references into a histogram (without reset).
/// The distance modifier function is applied to the distance before
/// the histogram is updated. It can be `None`.
pub fn vp8l_histogram_store_refs(
    refs: &VP8LBackwardRefs,
    distance_modifier: Option<fn(i32, i32) -> i32>,
    distance_modifier_arg0: i32,
    histo: &mut VP8LHistogram,
) {
    let mut c = vp8l_refs_cursor_init(refs);
    while vp8l_refs_cursor_ok(&c) {
        histogram_add_single_pix_or_copy(
            histo,
            c.get(refs),
            distance_modifier,
            distance_modifier_arg0,
        );
        vp8l_refs_cursor_next(&mut c, refs);
    }
}

// -----------------------------------------------------------------------------
// Entropy-related functions.

// upstream: src/enc/histogram_enc.c BitsEntropyRefine
/// The `uint64_t` products and sums wrap and convert to the `int64_t`
/// arguments of `DivRound`, as in C.
#[inline]
fn bits_entropy_refine(entropy: &VP8LBitEntropy) -> u64 {
    let mix: u64;
    if entropy.nonzeros < 5 {
        if entropy.nonzeros <= 1 {
            return 0;
        }
        // Two symbols, they will be 0 and 1 in a Huffman code.
        // Let's mix in a bit of entropy to favor good clustering when
        // distributions of these are combined.
        if entropy.nonzeros == 2 {
            let numerator = 99_u64
                .wrapping_mul(u64::from(entropy.sum) << LOG_2_PRECISION_BITS)
                .wrapping_add(entropy.entropy);
            return div_round(numerator.cast_signed(), 100).cast_unsigned();
        }
        // No matter what the entropy says, we cannot be better than min_limit
        // with Huffman coding. I am mixing a bit of entropy into the
        // min_limit since it produces much better (~0.5 %) compression results
        // perhaps because of better entropy clustering.
        if entropy.nonzeros == 3 {
            mix = 950;
        } else {
            mix = 700; // nonzeros == 4.
        }
    } else {
        mix = 627;
    }

    let mut min_limit = u64::from(
        2_u32
            .wrapping_mul(entropy.sum)
            .wrapping_sub(entropy.max_val),
    ) << LOG_2_PRECISION_BITS;
    let numerator = mix
        .wrapping_mul(min_limit)
        .wrapping_add((1000 - mix).wrapping_mul(entropy.entropy));
    min_limit = div_round(numerator.cast_signed(), 1000).cast_unsigned();
    if entropy.entropy < min_limit {
        min_limit
    } else {
        entropy.entropy
    }
}

// upstream: src/enc/histogram_enc.c VP8LBitsEntropy
/// Returns the entropy for the symbols in the input array.
#[must_use]
pub fn vp8l_bits_entropy(array: &[u32], n: i32) -> u64 {
    let mut entropy = VP8LBitEntropy::default();
    vp8l_bits_entropy_unrefined(array, n, &mut entropy);

    bits_entropy_refine(&entropy)
}

// upstream: src/enc/histogram_enc.c InitialHuffmanCost
fn initial_huffman_cost() -> u64 {
    // Small bias because Huffman code length is typically not stored in
    // full length.
    const HUFFMAN_CODE_OF_HUFFMAN_CODE_SIZE: u64 = CODE_LENGTH_CODES as u64 * 3;
    // Subtract a bias of 9.1.
    (HUFFMAN_CODE_OF_HUFFMAN_CODE_SIZE << LOG_2_PRECISION_BITS)
        .wrapping_sub(div_round(91_i64 << LOG_2_PRECISION_BITS, 10).cast_unsigned())
}

// upstream: src/enc/histogram_enc.c FinalHuffmanCost
/// Finalize the Huffman cost based on streak numbers and length type (<3 or
/// >=3). The products are `int` in C and convert to the `uint32_t` sum.
fn final_huffman_cost(stats: &VP8LStreaks) -> u64 {
    // The constants in this function are empirical and got rounded from
    // their original values in 1/8 when switched to 1/1024.
    let retval = initial_huffman_cost();
    // Second coefficient: Many zeros in the histogram are covered efficiently
    // by a run-length encode. Originally 2/8.
    let mut retval_extra: u32 =
        (stats.counts[0] * 1600 + 240 * stats.streaks[0][1]).cast_unsigned();
    // Second coefficient: Constant values are encoded less efficiently, but still
    // RLE'ed. Originally 6/8.
    retval_extra = retval_extra
        .wrapping_add((stats.counts[1] * 2640 + 720 * stats.streaks[1][1]).cast_unsigned());
    // 0s are usually encoded more efficiently than non-0s.
    // Originally 15/8.
    retval_extra = retval_extra.wrapping_add((1840 * stats.streaks[0][0]).cast_unsigned());
    // Originally 26/8.
    retval_extra = retval_extra.wrapping_add((3360 * stats.streaks[1][0]).cast_unsigned());
    retval.wrapping_add(u64::from(retval_extra) << (LOG_2_PRECISION_BITS - 10))
}

// upstream: src/enc/histogram_enc.c PopulationCost
/// Get the symbol entropy for the distribution `population`.
/// Set `trivial_sym`, if there's only one symbol present in the distribution.
#[expect(
    clippy::cast_possible_truncation,
    reason = "the C conversion of the symbol index to uint16_t"
)]
fn population_cost(
    population: &[u32],
    length: i32,
    trivial_sym: Option<&mut u16>,
    is_used: Option<&mut u8>,
) -> u64 {
    let mut bit_entropy = VP8LBitEntropy::default();
    let mut stats = VP8LStreaks::default();
    vp8l_get_entropy_unrefined(population, length, &mut bit_entropy, &mut stats);
    if let Some(trivial_sym) = trivial_sym {
        *trivial_sym = if bit_entropy.nonzeros == 1 {
            bit_entropy.nonzero_code as u16
        } else {
            VP8L_NON_TRIVIAL_SYM
        };
    }
    if let Some(is_used) = is_used {
        // The histogram is used if there is at least one non-zero streak.
        *is_used = u8::from(stats.streaks[1][0] != 0 || stats.streaks[1][1] != 0);
    }

    bits_entropy_refine(&bit_entropy).wrapping_add(final_huffman_cost(&stats))
}

// upstream: src/enc/histogram_enc.c GetPopulationInfo
/// Returns the population and its length, the two out-parameters of C.
#[inline]
fn get_population_info(histo: &VP8LHistogram, index: HistogramIndex) -> (&[u32], i32) {
    match index {
        HistogramIndex::LITERAL => (
            &histo.literal,
            vp8l_histogram_num_codes(histo.palette_code_bits),
        ),
        HistogramIndex::RED => (&histo.red, NUM_LITERAL_CODES),
        HistogramIndex::BLUE => (&histo.blue, NUM_LITERAL_CODES),
        HistogramIndex::ALPHA => (&histo.alpha, NUM_LITERAL_CODES),
        _ => {
            debug_assert_eq!(index, HistogramIndex::DISTANCE);
            (&histo.distance, NUM_DISTANCE_CODES)
        }
    }
}

/// [`get_population_info`] for the output of `HistogramAdd`, which C reaches
/// through a cast of the `const` population.
#[inline]
fn get_population_info_mut(histo: &mut VP8LHistogram, index: HistogramIndex) -> (&mut [u32], i32) {
    let length = vp8l_histogram_num_codes(histo.palette_code_bits);
    match index {
        HistogramIndex::LITERAL => (&mut histo.literal, length),
        HistogramIndex::RED => (&mut histo.red, NUM_LITERAL_CODES),
        HistogramIndex::BLUE => (&mut histo.blue, NUM_LITERAL_CODES),
        HistogramIndex::ALPHA => (&mut histo.alpha, NUM_LITERAL_CODES),
        _ => {
            debug_assert_eq!(index, HistogramIndex::DISTANCE);
            (&mut histo.distance, NUM_DISTANCE_CODES)
        }
    }
}

// upstream: src/enc/histogram_enc.c GetCombinedEntropy
/// `trivial_at_end` is 1 if the two histograms only have one element that is
/// non-zero: both the zero-th one, or both the last one.
/// `index` is the index of the symbol in the histogram (literal, red, blue,
/// alpha, distance).
#[inline]
fn get_combined_entropy(h1: &VP8LHistogram, h2: &VP8LHistogram, index: HistogramIndex) -> u64 {
    let i = index.0 as usize;
    let mut stats = VP8LStreaks::default();
    let mut bit_entropy = VP8LBitEntropy::default();
    let is_h1_used = h1.is_used[i] != 0;
    let is_h2_used = h2.is_used[i] != 0;
    let is_trivial = h1.trivial_symbol[i] != VP8L_NON_TRIVIAL_SYM
        && h1.trivial_symbol[i] == h2.trivial_symbol[i];

    if is_trivial || !is_h1_used || !is_h2_used {
        if is_h1_used {
            return h1.costs[i];
        }
        return h2.costs[i];
    }
    debug_assert!(is_h1_used && is_h2_used);

    let (x, _length) = get_population_info(h1, index);
    let (y, length) = get_population_info(h2, index);
    vp8l_get_combined_entropy_unrefined(x, y, length, &mut bit_entropy, &mut stats);
    bits_entropy_refine(&bit_entropy).wrapping_add(final_huffman_cost(&stats))
}

// upstream: src/enc/histogram_enc.c VP8LHistogramEstimateBits
/// Estimates the Entropy + Huffman + other block overhead size cost.
#[must_use]
pub fn vp8l_histogram_estimate_bits(h: &VP8LHistogram) -> u64 {
    let mut cost: u64 = 0;
    for i in 0..5 {
        let (population, length) = get_population_info(h, HistogramIndex::of(i));
        cost = cost.wrapping_add(population_cost(
            population, length, /*trivial_sym=*/ None, /*is_used=*/ None,
        ));
    }
    let extra = vp8l_extra_cost(&h.literal[to_index(NUM_LITERAL_CODES)..], NUM_LENGTH_CODES)
        .wrapping_add(vp8l_extra_cost(&h.distance, NUM_DISTANCE_CODES));
    cost.wrapping_add(u64::from(extra) << LOG_2_PRECISION_BITS)
}

// -----------------------------------------------------------------------------
// Various histogram combine/cost-eval functions

// upstream: src/enc/histogram_enc.c SaturateAdd
/// Set a + b in b, saturating at `WEBP_INT64_MAX`.
#[inline]
fn saturate_add(a: u64, b: &mut i64) {
    if *b < 0 || a.cast_signed() <= WEBP_INT64_MAX - *b {
        *b += a.cast_signed();
    } else {
        *b = WEBP_INT64_MAX;
    }
}

// upstream: src/enc/histogram_enc.c GetCombinedHistogramEntropy
/// Returns 1 if the cost of the combined histogram is less than the threshold.
/// Otherwise returns 0 and the cost is invalid due to early bail-out.
#[must_use]
fn get_combined_histogram_entropy(
    a: &VP8LHistogram,
    b: &VP8LHistogram,
    cost_threshold_in: i64,
    cost: &mut u64,
    costs: &mut [u64; 5],
) -> bool {
    let cost_threshold = cost_threshold_in.cast_unsigned();
    debug_assert_eq!(a.palette_code_bits, b.palette_code_bits);
    if cost_threshold_in <= 0 {
        return false;
    }
    *cost = 0;

    // No need to add the extra cost for length and distance as it is a constant
    // that does not influence the histograms.
    for (i, slot) in costs.iter_mut().enumerate() {
        *slot = get_combined_entropy(a, b, HistogramIndex::of(i));
        *cost = cost.wrapping_add(*slot);
        if *cost >= cost_threshold {
            return false;
        }
    }

    true
}

// upstream: src/enc/histogram_enc.c HistogramAdd
/// `hout = h1 + h2`. `h2` is `None` where C passes `h2 == hout`, which adds
/// `h1` into `hout` in place; otherwise the three histograms differ.
#[inline]
fn histogram_add(h1: &VP8LHistogram, h2: Option<&VP8LHistogram>, hout: &mut VP8LHistogram) {
    if let Some(h2) = h2 {
        debug_assert_eq!(h1.palette_code_bits, h2.palette_code_bits);
    }

    for i in 0..5 {
        let index = HistogramIndex::of(i);
        let h1_used = h1.is_used[i] != 0;
        let hout_used = hout.is_used[i] != 0;
        let (p1, _) = get_population_info(h1, index);
        match h2 {
            None => {
                let (pout, length) = get_population_info_mut(hout, index);
                let n = to_index(length);
                if h1_used {
                    if hout_used {
                        vp8l_add_vector_eq(p1, pout, length);
                    } else {
                        pout[..n].copy_from_slice(&p1[..n]);
                    }
                }
            }
            Some(h2) => {
                let h2_used = h2.is_used[i] != 0;
                let (p2, _) = get_population_info(h2, index);
                let (pout, length) = get_population_info_mut(hout, index);
                let n = to_index(length);
                if h1_used {
                    if h2_used {
                        vp8l_add_vector(p1, p2, pout, length);
                    } else {
                        pout[..n].copy_from_slice(&p1[..n]);
                    }
                } else if h2_used {
                    pout[..n].copy_from_slice(&p2[..n]);
                } else {
                    pout[..n].fill(0);
                }
            }
        }
    }

    for i in 0..5 {
        let (h2_trivial, h2_used) = match h2 {
            None => (hout.trivial_symbol[i], hout.is_used[i]),
            Some(h2) => (h2.trivial_symbol[i], h2.is_used[i]),
        };
        hout.trivial_symbol[i] = if h1.trivial_symbol[i] == h2_trivial {
            h1.trivial_symbol[i]
        } else {
            VP8L_NON_TRIVIAL_SYM
        };
        hout.is_used[i] = u8::from(h1.is_used[i] != 0 || h2_used != 0);
    }
}

// upstream: src/enc/histogram_enc.c UpdateHistogramCost
fn update_histogram_cost(bit_cost: u64, costs: &[u64; 5], h: &mut VP8LHistogram) {
    h.bit_cost = bit_cost;
    h.costs = *costs;
}

// upstream: src/enc/histogram_enc.c HistogramAddEval
/// Performs out = a + b, computing the cost C(a+b) - C(a) - C(b) while
/// comparing to the threshold value `cost_threshold`. The score returned is
///  Score = C(a+b) - C(a) - C(b), where C(a) + C(b) is known and fixed.
/// Since the previous score passed is `cost_threshold`, we only need to compare
/// the partial cost against `cost_threshold + C(a) + C(b)` to possibly bail-out
/// early.
/// Returns 1 if the cost is less than the threshold.
/// Otherwise returns 0 and the cost is invalid due to early bail-out.
#[must_use]
fn histogram_add_eval(
    a: &VP8LHistogram,
    b: &VP8LHistogram,
    out: &mut VP8LHistogram,
    mut cost_threshold: i64,
) -> bool {
    let sum_cost = a.bit_cost.wrapping_add(b.bit_cost);
    let mut bit_cost: u64 = 0;
    let mut costs = [0_u64; 5];
    saturate_add(sum_cost, &mut cost_threshold);
    if !get_combined_histogram_entropy(a, b, cost_threshold, &mut bit_cost, &mut costs) {
        return false;
    }

    histogram_add(a, Some(b), out);
    update_histogram_cost(bit_cost, &costs, out);
    true
}

// upstream: src/enc/histogram_enc.c HistogramAddThresh
/// Same as `HistogramAddEval()`, except that the resulting histogram
/// is not stored. Only the cost C(a+b) - C(a) is evaluated. We omit
/// the term C(b) which is constant over all the evaluations.
/// Returns 1 if the cost is less than the threshold.
/// Otherwise returns 0 and the cost is invalid due to early bail-out.
#[must_use]
fn histogram_add_thresh(
    a: &VP8LHistogram,
    b: &VP8LHistogram,
    mut cost_threshold: i64,
    cost_out: &mut i64,
) -> bool {
    let mut cost: u64 = 0;
    let mut costs = [0_u64; 5];
    saturate_add(a.bit_cost, &mut cost_threshold);
    if !get_combined_histogram_entropy(a, b, cost_threshold, &mut cost, &mut costs) {
        return false;
    }

    *cost_out = cost.cast_signed() - a.bit_cost.cast_signed();
    true
}

// -----------------------------------------------------------------------------

// upstream: src/enc/histogram_enc.c DominantCostRange
/// The structure to keep track of cost range for the three dominant entropy
/// symbols.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct DominantCostRange {
    literal_max: u64,
    literal_min: u64,
    red_max: u64,
    red_min: u64,
    blue_max: u64,
    blue_min: u64,
}

// upstream: src/enc/histogram_enc.c DominantCostRangeInit
fn dominant_cost_range_init(c: &mut DominantCostRange) {
    c.literal_max = 0;
    c.literal_min = WEBP_UINT64_MAX;
    c.red_max = 0;
    c.red_min = WEBP_UINT64_MAX;
    c.blue_max = 0;
    c.blue_min = WEBP_UINT64_MAX;
}

// upstream: src/enc/histogram_enc.c UpdateDominantCostRange
fn update_dominant_cost_range(h: &VP8LHistogram, c: &mut DominantCostRange) {
    let literal = h.costs[HistogramIndex::LITERAL.0 as usize];
    let red = h.costs[HistogramIndex::RED.0 as usize];
    let blue = h.costs[HistogramIndex::BLUE.0 as usize];
    if c.literal_max < literal {
        c.literal_max = literal;
    }
    if c.literal_min > literal {
        c.literal_min = literal;
    }
    if c.red_max < red {
        c.red_max = red;
    }
    if c.red_min > red {
        c.red_min = red;
    }
    if c.blue_max < blue {
        c.blue_max = blue;
    }
    if c.blue_min > blue {
        c.blue_min = blue;
    }
}

// upstream: src/enc/histogram_enc.c ComputeHistogramCost
fn compute_histogram_cost(h: &mut VP8LHistogram) {
    // No need to add the extra cost for length and distance as it is a constant
    // that does not influence the histograms.
    for i in 0..5 {
        let mut trivial_symbol = h.trivial_symbol[i];
        let mut is_used = h.is_used[i];
        let (population, length) = get_population_info(h, HistogramIndex::of(i));
        let cost = population_cost(
            population,
            length,
            Some(&mut trivial_symbol),
            Some(&mut is_used),
        );
        h.costs[i] = cost;
        h.trivial_symbol[i] = trivial_symbol;
        h.is_used[i] = is_used;
    }
    h.bit_cost = h.costs[0]
        .wrapping_add(h.costs[1])
        .wrapping_add(h.costs[2])
        .wrapping_add(h.costs[3])
        .wrapping_add(h.costs[4]);
}

// upstream: src/enc/histogram_enc.c GetBinIdForEntropy
#[expect(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    reason = "the C conversions of the costs to double and of the bin to int"
)]
fn get_bin_id_for_entropy(min: u64, max: u64, val: u64) -> i32 {
    let range = max.wrapping_sub(min);
    if range > 0 {
        let delta = val.wrapping_sub(min);
        ((f64::from(NUM_PARTITIONS) - 1e-6) * delta as f64 / range as f64) as i32
    } else {
        0
    }
}

// upstream: src/enc/histogram_enc.c GetHistoBinIndex
fn get_histo_bin_index(h: &VP8LHistogram, c: &DominantCostRange, low_effort: bool) -> i32 {
    let mut bin_id = get_bin_id_for_entropy(
        c.literal_min,
        c.literal_max,
        h.costs[HistogramIndex::LITERAL.0 as usize],
    );
    debug_assert!(bin_id < NUM_PARTITIONS);
    if !low_effort {
        bin_id = bin_id * NUM_PARTITIONS
            + get_bin_id_for_entropy(
                c.red_min,
                c.red_max,
                h.costs[HistogramIndex::RED.0 as usize],
            );
        bin_id = bin_id * NUM_PARTITIONS
            + get_bin_id_for_entropy(
                c.blue_min,
                c.blue_max,
                h.costs[HistogramIndex::BLUE.0 as usize],
            );
        debug_assert!(bin_id < BIN_SIZE);
    }
    bin_id
}

// upstream: src/enc/histogram_enc.c HistogramBuild
/// Construct the histograms from backward references.
fn histogram_build(
    xsize: i32,
    histo_bits: i32,
    backward_refs: &VP8LBackwardRefs,
    image_histo: &mut VP8LHistogramSet,
) {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let histo_xsize =
        vp8l_sub_sample_size(xsize.cast_unsigned(), histo_bits.cast_unsigned()).cast_signed();
    let mut c = vp8l_refs_cursor_init(backward_refs);
    debug_assert!(histo_bits > 0);
    vp8l_histogram_set_clear(image_histo);
    while vp8l_refs_cursor_ok(&c) {
        let v = c.get(backward_refs);
        let ix = (y >> histo_bits) * histo_xsize + (x >> histo_bits);
        histogram_add_single_pix_or_copy(image_histo.histogram_mut(to_index(ix)), v, None, 0);
        x += pix_or_copy_length(v).cast_signed();
        while x >= xsize {
            x -= xsize;
            y += 1;
        }
        vp8l_refs_cursor_next(&mut c, backward_refs);
    }
}

// upstream: src/enc/histogram_enc.c HistogramCopyAndAnalyze
/// Copies the histograms and computes its `bit_cost`.
fn histogram_copy_and_analyze(
    orig_histo: &mut VP8LHistogramSet,
    image_histo: &mut VP8LHistogramSet,
) {
    debug_assert_eq!(image_histo.max_size, orig_histo.max_size);
    image_histo.size = 0;
    for i in 0..to_index(orig_histo.max_size) {
        let histo = orig_histo.histogram_mut(i);
        compute_histogram_cost(histo);

        // Skip the histogram if it is completely empty, which can happen for tiles
        // with no information (when they are skipped because of LZ77).
        if histo.is_used == [0; 5] {
            // The first histogram is always used.
            debug_assert!(i > 0);
            orig_histo.histograms[i] = None;
        } else {
            // Copy histograms from orig_histo[] to image_histo[].
            let slot = to_index(image_histo.size);
            histogram_copy(orig_histo.histogram(i), image_histo.histogram_mut(slot));
            image_histo.size += 1;
        }
    }
}

// upstream: src/enc/histogram_enc.c HistogramAnalyzeEntropyBin
/// Partition histograms to different entropy bins for three dominant (literal,
/// red and blue) symbol costs and compute the histogram aggregate `bit_cost`.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "the C conversion of the bin index to uint16_t"
)]
fn histogram_analyze_entropy_bin(image_histo: &mut VP8LHistogramSet, low_effort: bool) {
    let histo_size = to_index(image_histo.size);
    let mut cost_range = DominantCostRange::default();
    dominant_cost_range_init(&mut cost_range);

    // Analyze the dominant (literal, red and blue) entropy costs.
    for i in 0..histo_size {
        update_dominant_cost_range(image_histo.histogram(i), &mut cost_range);
    }

    // bin-hash histograms on three of the dominant (literal, red and blue)
    // symbol costs and store the resulting bin_id for each histogram.
    for i in 0..histo_size {
        let histo = image_histo.histogram_mut(i);
        histo.bin_id = get_histo_bin_index(histo, &cost_range, low_effort) as u16;
    }
}

/// One entry of the `bin_info` table of `HistogramCombineEntropyBin`.
#[derive(Clone, Copy, Debug)]
struct BinInfo {
    /// position of the histogram that accumulates all histograms with the same
    /// `bin_id`
    first: i16,
    /// number of combine failures per `bin_id`
    num_combine_failures: u16,
}

/// The shared histogram at `shared` and the mutable one at `mutable` of a set.
fn set_pair(
    set: &mut VP8LHistogramSet,
    shared: usize,
    mutable: usize,
) -> (&VP8LHistogram, &mut VP8LHistogram) {
    let a = set.histograms[shared].expect("the histogram slot is not NULL");
    let b = set.histograms[mutable].expect("the histogram slot is not NULL");
    let (a, b) = split_pair(&mut set.memory, a, b);
    (a, b)
}

// upstream: src/enc/histogram_enc.c HistogramCombineEntropyBin
/// Merges some histograms with same `bin_id` together if it's advantageous.
/// Sets the remaining histograms to `NULL`.
/// `combine_cost_factor` has to be divided by 100.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    reason = "the C conversion of the position to int16_t"
)]
fn histogram_combine_entropy_bin(
    image_histo: &mut VP8LHistogramSet,
    cur_combo: &mut Box<VP8LHistogram>,
    num_bins: i32,
    combine_cost_factor: i32,
    low_effort: bool,
) {
    let mut bin_info = [BinInfo {
        first: 0,
        num_combine_failures: 0,
    }; BIN_SIZE as usize];

    debug_assert!(num_bins <= BIN_SIZE);
    for info in &mut bin_info[..to_index(num_bins)] {
        info.first = -1;
        info.num_combine_failures = 0;
    }

    let mut idx: usize = 0;
    while idx < to_index(image_histo.size) {
        let bin_id = usize::from(image_histo.histogram(idx).bin_id);
        let first = bin_info[bin_id].first;
        if first == -1 {
            bin_info[bin_id].first = idx as i16;
            idx += 1;
        } else if low_effort {
            let first = usize::try_from(first).expect("a first position is not negative");
            let (h_idx, h_first) = set_pair(image_histo, idx, first);
            histogram_add(h_idx, None, h_first);
            histogram_set_remove_histogram(image_histo, idx);
        } else {
            let first = usize::try_from(first).expect("a first position is not negative");
            // try to merge #idx into #first (both share the same bin_id)
            let bit_cost = image_histo.histogram(idx).bit_cost;
            let bit_cost_thresh =
                -div_round(bit_cost.cast_signed() * i64::from(combine_cost_factor), 100);
            if histogram_add_eval(
                image_histo.histogram(first),
                image_histo.histogram(idx),
                cur_combo,
                bit_cost_thresh,
            ) {
                const MAX_COMBINE_FAILURES: u16 = 32;
                let red = HistogramIndex::RED.0 as usize;
                let blue = HistogramIndex::BLUE.0 as usize;
                let alpha = HistogramIndex::ALPHA.0 as usize;
                // Try to merge two histograms only if the combo is a trivial one or
                // the two candidate histograms are already non-trivial.
                // For some images, 'try_combine' turns out to be false for a lot of
                // histogram pairs. In that case, we fallback to combining
                // histograms as usual to avoid increasing the header size.
                let mut try_combine = cur_combo.trivial_symbol[red] != VP8L_NON_TRIVIAL_SYM
                    && cur_combo.trivial_symbol[blue] != VP8L_NON_TRIVIAL_SYM
                    && cur_combo.trivial_symbol[alpha] != VP8L_NON_TRIVIAL_SYM;
                if !try_combine {
                    let h_idx = image_histo.histogram(idx);
                    let h_first = image_histo.histogram(first);
                    try_combine = h_idx.trivial_symbol[red] == VP8L_NON_TRIVIAL_SYM
                        || h_idx.trivial_symbol[blue] == VP8L_NON_TRIVIAL_SYM
                        || h_idx.trivial_symbol[alpha] == VP8L_NON_TRIVIAL_SYM;
                    try_combine &= h_first.trivial_symbol[red] == VP8L_NON_TRIVIAL_SYM
                        || h_first.trivial_symbol[blue] == VP8L_NON_TRIVIAL_SYM
                        || h_first.trivial_symbol[alpha] == VP8L_NON_TRIVIAL_SYM;
                }
                if try_combine || bin_info[bin_id].num_combine_failures >= MAX_COMBINE_FAILURES {
                    // move the (better) merged histogram to its final slot
                    let slot =
                        image_histo.histograms[first].expect("the histogram slot is not NULL");
                    histogram_swap(cur_combo, &mut image_histo.memory[slot]);
                    histogram_set_remove_histogram(image_histo, idx);
                } else {
                    bin_info[bin_id].num_combine_failures += 1;
                    idx += 1;
                }
            } else {
                idx += 1;
            }
        }
    }
    if low_effort {
        // for low_effort case, update the final cost when everything is merged
        for idx in 0..to_index(image_histo.size) {
            compute_histogram_cost(image_histo.histogram_mut(idx));
        }
    }
}

// upstream: src/enc/histogram_enc.c MyRand
/// Implement a Lehmer random number generator with a multiplicative constant of
/// 48271 and a modulo constant of 2^31 - 1.
fn my_rand(seed: &mut u32) -> u32 {
    *seed = ((u64::from(*seed) * 48271) % 2_147_483_647) as u32;
    debug_assert!(*seed > 0);
    *seed
}

// -----------------------------------------------------------------------------
// Histogram pairs priority queue

// upstream: src/enc/histogram_enc.c HistogramPair
/// Pair of histograms. Negative `idx1` value means that pair is out-of-date.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct HistogramPair {
    idx1: i32,
    idx2: i32,
    cost_diff: i64,
    cost_combo: u64,
    costs: [u64; 5],
}

impl HistogramPair {
    /// The C `sizeof(HistogramPair)`, the `size` argument of the queue
    /// allocation.
    pub const C_SIZE: usize = 64;
}

// upstream: src/enc/histogram_enc.c HistoQueue
#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct HistoQueue {
    queue: Vec<HistogramPair>,
    size: i32,
    max_size: i32,
}

// upstream: src/enc/histogram_enc.c HistoQueueInit
fn histo_queue_init(histo_queue: &mut HistoQueue, max_size: i32) -> bool {
    histo_queue.size = 0;
    histo_queue.max_size = max_size;
    // We allocate max_size + 1 because the last element at index "size" is
    // used as temporary data (and it could be up to max_size).
    let count = i64::from(histo_queue.max_size + 1).cast_unsigned();
    if let Some(mut queue) = webp_safe_malloc::<HistogramPair>(count, HistogramPair::C_SIZE) {
        queue.resize(to_index(histo_queue.max_size + 1), HistogramPair::default());
        histo_queue.queue = queue;
        true
    } else {
        histo_queue.queue = Vec::new();
        false
    }
}

// upstream: src/enc/histogram_enc.c HistoQueueClear
fn histo_queue_clear(histo_queue: &mut HistoQueue) {
    histo_queue.queue = Vec::new();
    histo_queue.size = 0;
    histo_queue.max_size = 0;
}

// upstream: src/enc/histogram_enc.c HistoQueuePopPair
/// Pop a specific pair in the queue by replacing it with the last one
/// and shrinking the queue. The pair is a position in the queue.
fn histo_queue_pop_pair(histo_queue: &mut HistoQueue, pair: usize) {
    debug_assert!(pair < to_index(histo_queue.size));
    debug_assert!(histo_queue.size > 0);
    histo_queue.queue[pair] = histo_queue.queue[to_index(histo_queue.size - 1)];
    histo_queue.size -= 1;
}

// upstream: src/enc/histogram_enc.c HistoQueueUpdateHead
/// Check whether a pair in the queue should be updated as head or not.
fn histo_queue_update_head(histo_queue: &mut HistoQueue, pair: usize) {
    debug_assert!(histo_queue.queue[pair].cost_diff < 0);
    debug_assert!(pair < to_index(histo_queue.size));
    debug_assert!(histo_queue.size > 0);
    if histo_queue.queue[pair].cost_diff < histo_queue.queue[0].cost_diff {
        // Replace the best pair.
        histo_queue.queue.swap(0, pair);
    }
}

// upstream: src/enc/histogram_enc.c HistoQueueFixPair
/// Replaces the `bad_id` with `good_id` in the pair.
fn histo_queue_fix_pair(bad_id: i32, good_id: i32, pair: &mut HistogramPair) {
    if pair.idx1 == bad_id {
        pair.idx1 = good_id;
    }
    if pair.idx2 == bad_id {
        pair.idx2 = good_id;
    }
    if pair.idx1 > pair.idx2 {
        std::mem::swap(&mut pair.idx1, &mut pair.idx2);
    }
}

// upstream: src/enc/histogram_enc.c HistoQueueUpdatePair
/// Update the cost diff and combo of a pair of histograms. This needs to be
/// called when the histograms have been merged with a third one.
/// Returns 1 if the cost diff is less than the threshold.
/// Otherwise returns 0 and the cost is invalid due to early bail-out.
#[must_use]
fn histo_queue_update_pair(
    h1: &VP8LHistogram,
    h2: &VP8LHistogram,
    mut cost_threshold: i64,
    pair: &mut HistogramPair,
) -> bool {
    let sum_cost: i64 = h1.bit_cost.wrapping_add(h2.bit_cost).cast_signed();
    saturate_add(sum_cost.cast_unsigned(), &mut cost_threshold);
    if !get_combined_histogram_entropy(
        h1,
        h2,
        cost_threshold,
        &mut pair.cost_combo,
        &mut pair.costs,
    ) {
        return false;
    }
    pair.cost_diff = pair.cost_combo.cast_signed() - sum_cost;
    true
}

// upstream: src/enc/histogram_enc.c HistoQueuePush
/// Create a pair from indices `idx1` and `idx2` provided its cost
/// is inferior to `threshold`, a negative entropy.
/// It returns the cost of the pair, or 0 if it superior to threshold.
fn histo_queue_push(
    histo_queue: &mut HistoQueue,
    histograms: &VP8LHistogramSet,
    mut idx1: i32,
    mut idx2: i32,
    threshold: i64,
) -> i64 {
    let mut pair = HistogramPair::default();

    // Stop here if the queue is full.
    if histo_queue.size == histo_queue.max_size {
        return 0;
    }
    debug_assert!(threshold <= 0);
    if idx1 > idx2 {
        std::mem::swap(&mut idx1, &mut idx2);
    }
    pair.idx1 = idx1;
    pair.idx2 = idx2;
    let h1 = histograms.histogram(to_index(idx1));
    let h2 = histograms.histogram(to_index(idx2));

    // Do not even consider the pair if it does not improve the entropy.
    if !histo_queue_update_pair(h1, h2, threshold, &mut pair) {
        return 0;
    }

    let at = to_index(histo_queue.size);
    histo_queue.queue[at] = pair;
    histo_queue.size += 1;
    histo_queue_update_head(histo_queue, at);

    pair.cost_diff
}

// -----------------------------------------------------------------------------

/// Adds the histogram at `from` into the one at `into` of `set` and gives it
/// the costs of the queue head (C: the `HistogramAdd` and
/// `UpdateHistogramCost` calls of the two combining loops).
fn merge_queue_head(set: &mut VP8LHistogramSet, from: usize, into: usize, head: &HistogramPair) {
    let (h_from, h_into) = set_pair(set, from, into);
    histogram_add(h_from, None, h_into);
    update_histogram_cost(head.cost_combo, &head.costs, h_into);
}

// upstream: src/enc/histogram_enc.c HistogramCombineGreedy
/// Combines histograms by continuously choosing the one with the highest cost
/// reduction.
fn histogram_combine_greedy(image_histo: &mut VP8LHistogramSet) -> bool {
    let image_histo_size = image_histo.size;
    // Priority queue of histogram pairs.
    let mut histo_queue = HistoQueue::default();

    // image_histo_size^2 for the queue size is safe. If you look at
    // HistogramCombineGreedy, and imagine that UpdateQueueFront always pushes
    // data to the queue, you insert at most:
    // - image_histo_size*(image_histo_size-1)/2 (the first two for loops)
    // - image_histo_size - 1 in the last for loop at the first iteration of
    //   the while loop, image_histo_size - 2 at the second iteration ...
    //   therefore image_histo_size*(image_histo_size-1)/2 overall too
    if !histo_queue_init(&mut histo_queue, image_histo_size * image_histo_size) {
        histo_queue_clear(&mut histo_queue);
        return false;
    }

    // Initialize the queue.
    for i in 0..image_histo_size {
        for j in (i + 1)..image_histo_size {
            histo_queue_push(&mut histo_queue, image_histo, i, j, 0);
        }
    }

    while histo_queue.size > 0 {
        let head = histo_queue.queue[0];
        let idx1 = head.idx1;
        let idx2 = head.idx2;
        merge_queue_head(image_histo, to_index(idx2), to_index(idx1), &head);

        // Remove merged histogram.
        histogram_set_remove_histogram(image_histo, to_index(idx2));

        // Remove pairs intersecting the just combined best pair.
        let mut i: usize = 0;
        while i < to_index(histo_queue.size) {
            let p = histo_queue.queue[i];
            if p.idx1 == idx1 || p.idx2 == idx1 || p.idx1 == idx2 || p.idx2 == idx2 {
                histo_queue_pop_pair(&mut histo_queue, i);
            } else {
                histo_queue_fix_pair(image_histo.size, idx2, &mut histo_queue.queue[i]);
                histo_queue_update_head(&mut histo_queue, i);
                i += 1;
            }
        }

        // Push new pairs formed with combined histogram to the queue.
        for i in 0..image_histo.size {
            if i == idx1 {
                continue;
            }
            histo_queue_push(&mut histo_queue, image_histo, idx1, i, 0);
        }
    }

    histo_queue_clear(&mut histo_queue);
    true
}

// upstream: src/enc/histogram_enc.c HistogramCombineStochastic
/// Perform histogram aggregation using a stochastic approach.
/// `do_greedy` is set to 1 if a greedy approach needs to be performed
/// afterwards, 0 otherwise.
#[expect(
    clippy::cast_possible_wrap,
    reason = "the C conversions of the random indexes to the int parameters"
)]
fn histogram_combine_stochastic(
    image_histo: &mut VP8LHistogramSet,
    min_cluster_size: i32,
    do_greedy: &mut bool,
) -> bool {
    // Priority queue of histogram pairs. Its size of 'kHistoQueueSize'
    // impacts the quality of the compression and the speed: the smaller the
    // faster but the worse for the compression.
    const HISTO_QUEUE_SIZE: i32 = 9;
    let mut seed: u32 = 1;
    let mut tries_with_no_success: i32 = 0;
    let outer_iters = image_histo.size;
    let num_tries_no_success = outer_iters / 2;
    let mut histo_queue = HistoQueue::default();

    if image_histo.size < min_cluster_size {
        *do_greedy = true;
        return true;
    }

    if !histo_queue_init(&mut histo_queue, HISTO_QUEUE_SIZE) {
        histo_queue_clear(&mut histo_queue);
        return false;
    }

    // Collapse similar histograms in 'image_histo'.
    let mut iter: i32 = 0;
    while iter < outer_iters && image_histo.size >= min_cluster_size && {
        tries_with_no_success += 1;
        tries_with_no_success < num_tries_no_success
    } {
        let mut best_cost: i64 = if histo_queue.size == 0 {
            0
        } else {
            histo_queue.queue[0].cost_diff
        };
        let rand_range = ((image_histo.size - 1) * image_histo.size).cast_unsigned();
        // (image_histo->size) / 2 was chosen empirically. Less means faster but
        // worse compression.
        let num_tries = image_histo.size / 2;

        // Pick random samples.
        let mut j: i32 = 0;
        while image_histo.size >= 2 && j < num_tries {
            // Choose two different histograms at random and try to combine them.
            let tmp = my_rand(&mut seed) % rand_range;
            let idx1 = tmp / (image_histo.size - 1).cast_unsigned();
            let mut idx2 = tmp % (image_histo.size - 1).cast_unsigned();
            if idx2 >= idx1 {
                idx2 += 1;
            }

            // Calculate cost reduction on combination.
            let curr_cost = histo_queue_push(
                &mut histo_queue,
                image_histo,
                idx1 as i32,
                idx2 as i32,
                best_cost,
            );
            if curr_cost < 0 {
                // found a better pair?
                best_cost = curr_cost;
                // Empty the queue if we reached full capacity.
                if histo_queue.size == histo_queue.max_size {
                    break;
                }
            }
            j += 1;
        }
        if histo_queue.size == 0 {
            iter += 1;
            continue;
        }

        // Get the best histograms.
        let head = histo_queue.queue[0];
        let best_idx1 = head.idx1;
        let best_idx2 = head.idx2;
        debug_assert!(best_idx1 < best_idx2);
        // Merge the histograms and remove best_idx2 from the queue.
        merge_queue_head(image_histo, to_index(best_idx2), to_index(best_idx1), &head);
        histogram_set_remove_histogram(image_histo, to_index(best_idx2));
        // Parse the queue and update each pair that deals with best_idx1,
        // best_idx2 or image_histo_size.
        let mut j: usize = 0;
        while j < to_index(histo_queue.size) {
            let p = histo_queue.queue[j];
            let is_idx1_best = p.idx1 == best_idx1 || p.idx1 == best_idx2;
            let is_idx2_best = p.idx2 == best_idx1 || p.idx2 == best_idx2;
            // The front pair could have been duplicated by a random pick so
            // check for it all the time nevertheless.
            if is_idx1_best && is_idx2_best {
                histo_queue_pop_pair(&mut histo_queue, j);
                continue;
            }
            // Any pair containing one of the two best indices should only refer to
            // best_idx1. Its cost should also be updated.
            if is_idx1_best || is_idx2_best {
                histo_queue_fix_pair(best_idx2, best_idx1, &mut histo_queue.queue[j]);
                // Re-evaluate the cost of an updated pair.
                let fixed = histo_queue.queue[j];
                if !histo_queue_update_pair(
                    image_histo.histogram(to_index(fixed.idx1)),
                    image_histo.histogram(to_index(fixed.idx2)),
                    0,
                    &mut histo_queue.queue[j],
                ) {
                    histo_queue_pop_pair(&mut histo_queue, j);
                    continue;
                }
            }
            histo_queue_fix_pair(image_histo.size, best_idx2, &mut histo_queue.queue[j]);
            histo_queue_update_head(&mut histo_queue, j);
            j += 1;
        }
        tries_with_no_success = 0;
        iter += 1;
    }
    *do_greedy = image_histo.size <= min_cluster_size;

    histo_queue_clear(&mut histo_queue);
    true
}

// -----------------------------------------------------------------------------
// Histogram refinement

// upstream: src/enc/histogram_enc.c HistogramRemap
/// Find the best `out` histogram for each of the `in` histograms.
/// At call-time, `out` contains the histograms of the clusters.
/// Note: we assume that `out[]->bit_cost` is already up-to-date.
#[expect(
    clippy::cast_sign_loss,
    reason = "the C conversion of the best cluster index to uint32_t"
)]
#[expect(
    clippy::needless_range_loop,
    reason = "the C index loop over the input histograms and the symbols"
)]
fn histogram_remap(input: &VP8LHistogramSet, out: &mut VP8LHistogramSet, symbols: &mut [u32]) {
    let in_size = to_index(out.max_size);
    let out_size = out.size;
    if out_size > 1 {
        for i in 0..in_size {
            let mut best_out: i32 = 0;
            let mut best_bits = WEBP_INT64_MAX;
            let Some(in_histo) = input.histograms[i].map(|at| &*input.memory[at]) else {
                // Arbitrarily set to the previous value if unused to help future LZ77.
                symbols[i] = symbols[i - 1];
                continue;
            };
            for k in 0..out_size {
                let mut cur_bits: i64 = 0;
                if histogram_add_thresh(
                    out.histogram(to_index(k)),
                    in_histo,
                    best_bits,
                    &mut cur_bits,
                ) {
                    best_bits = cur_bits;
                    best_out = k;
                }
            }
            symbols[i] = best_out as u32;
        }
    } else {
        debug_assert_eq!(out_size, 1);
        symbols[..in_size].fill(0);
    }

    // Recompute each out based on raw and symbols.
    vp8l_histogram_set_clear(out);
    out.size = out_size;

    for i in 0..in_size {
        let Some(in_histo) = input.histograms[i].map(|at| &*input.memory[at]) else {
            continue;
        };
        let idx = symbols[i] as usize;
        histogram_add(in_histo, None, out.histogram_mut(idx));
    }
}

// upstream: src/enc/histogram_enc.c GetCombineCostFactor
fn get_combine_cost_factor(histo_size: i32, quality: i32) -> i32 {
    let mut combine_cost_factor: i32 = 16;
    if quality < 90 {
        if histo_size > 256 {
            combine_cost_factor /= 2;
        }
        if histo_size > 512 {
            combine_cost_factor /= 2;
        }
        if histo_size > 1024 {
            combine_cost_factor /= 2;
        }
        if quality <= 50 {
            combine_cost_factor /= 2;
        }
    }
    combine_cost_factor
}

// upstream: src/enc/histogram_enc.c VP8LGetHistoImageSymbols
/// Builds the histogram image. `pic` and `percent` are for progress.
/// Returns false in case of error (stored in `pic.error_code`).
///
/// # Errors
///
/// Returns the error that `pic` holds at the end, as C returns
/// `pic->error_code == VP8_ENC_OK`.
#[expect(
    clippy::too_many_arguments,
    reason = "the C signature of VP8LGetHistoImageSymbols has thirteen parameters"
)]
#[expect(
    clippy::cast_possible_truncation,
    reason = "the C conversion of the rounded threshold to int"
)]
#[expect(
    clippy::similar_names,
    reason = "the C names histo_xsize and histo_ysize"
)]
pub fn vp8l_get_histo_image_symbols(
    xsize: i32,
    ysize: i32,
    refs: &VP8LBackwardRefs,
    quality: i32,
    low_effort: bool,
    histogram_bits: i32,
    cache_bits: i32,
    image_histo: &mut VP8LHistogramSet,
    tmp_histo: &mut Box<VP8LHistogram>,
    histogram_symbols: &mut [u32],
    pic: &mut WebPPicture<'_>,
    percent_range: i32,
    percent: &mut i32,
) -> Result<(), WebPEncodingError> {
    let histo_xsize = if histogram_bits != 0 {
        vp8l_sub_sample_size(xsize.cast_unsigned(), histogram_bits.cast_unsigned()).cast_signed()
    } else {
        1
    };
    let histo_ysize = if histogram_bits != 0 {
        vp8l_sub_sample_size(ysize.cast_unsigned(), histogram_bits.cast_unsigned()).cast_signed()
    } else {
        1
    };
    let image_histo_raw_size = histo_xsize * histo_ysize;
    let orig_histo = vp8l_allocate_histogram_set(image_histo_raw_size, cache_bits);
    // Don't attempt linear bin-partition heuristic for
    // histograms of small sizes (as bin_map will be very sparse) and
    // maximum quality q==100 (to preserve the compression gains at that level).
    let entropy_combine_num_bins = if low_effort { NUM_PARTITIONS } else { BIN_SIZE };
    'error: {
        let Some(mut orig_histo) = orig_histo else {
            webp_encoding_set_error(pic, WebPEncodingError::VP8_ENC_ERROR_OUT_OF_MEMORY);
            break 'error;
        };

        // Construct the histograms from backward references.
        histogram_build(xsize, histogram_bits, refs, &mut orig_histo);
        histogram_copy_and_analyze(&mut orig_histo, image_histo);
        let entropy_combine = (image_histo.size > entropy_combine_num_bins * 2) && (quality < 100);

        if entropy_combine {
            let combine_cost_factor = get_combine_cost_factor(image_histo_raw_size, quality);

            histogram_analyze_entropy_bin(image_histo, low_effort);
            // Collapse histograms with similar entropy.
            histogram_combine_entropy_bin(
                image_histo,
                tmp_histo,
                entropy_combine_num_bins,
                combine_cost_factor,
                low_effort,
            );
        }

        // Don't combine the histograms using stochastic and greedy heuristics for
        // low-effort compression mode.
        if !low_effort || !entropy_combine {
            // cubic ramp between 1 and MAX_HISTO_GREEDY:
            let threshold_size = (1 + div_round(
                i64::from(quality * quality * quality * (MAX_HISTO_GREEDY - 1)),
                100 * 100 * 100,
            )) as i32;
            let mut do_greedy = false;
            if !histogram_combine_stochastic(image_histo, threshold_size, &mut do_greedy) {
                webp_encoding_set_error(pic, WebPEncodingError::VP8_ENC_ERROR_OUT_OF_MEMORY);
                break 'error;
            }
            if do_greedy && !histogram_combine_greedy(image_histo) {
                webp_encoding_set_error(pic, WebPEncodingError::VP8_ENC_ERROR_OUT_OF_MEMORY);
                break 'error;
            }
        }

        // Find the optimal map from original histograms to the final ones.
        histogram_remap(&orig_histo, image_histo, histogram_symbols);

        let progress = *percent + percent_range;
        let _ = webp_report_progress(pic, progress, Some(percent));
    }

    if pic.error_code == WebPEncodingError::VP8_ENC_OK {
        Ok(())
    } else {
        Err(pic.error_code)
    }
}
