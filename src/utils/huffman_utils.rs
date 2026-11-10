// libwebp v1.6.0, src/utils/huffman_utils.c:
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
// Utilities for building and looking up Huffman trees.
//
// Author: Urvang Joshi (urvang@google.com)

// libwebp v1.6.0, src/utils/huffman_utils.h:
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
// Utilities for building and looking up Huffman trees.
//
// Author: Urvang Joshi (urvang@google.com)

//! Translation of `src/utils/huffman_utils.c` and `src/utils/huffman_utils.h` from libwebp v1.6.0.
//!
//! Work package: WP3 (lossless shared).
//!
//! Representation (ADR 0003, "Readers over input"): C chains the segments of a
//! [`HuffmanTables`] through pointers, and the decoder keeps pointers to table
//! entries. Here the tables own their segments, `root` and then
//! `next_segments` in chain order, and a table reference is a
//! [`HuffmanTablesOffset`]: the index of the segment and the position of the
//! entry inside it. The owner of the tables passes them to each function that
//! reads an entry.
//!
//! # Deviations from upstream
//!
//! Changes of representation that the ADRs allow, or behavior that the Rust
//! side cannot reproduce exactly. Each records the reason.
//!
//! 1. **`HuffmanTables` stores positions (ADR 0003, "Readers over input").**
//!    The tables own their segments: `root`, then `next_segments` in the order
//!    of the C `next` chain, so `HuffmanTablesSegment` has no `next` field.
//!    `curr_segment` is the segment's index (0 is `root`), and `curr_table` is
//!    a position in the segment's `start`. A C `HuffmanCode*` into the tables
//!    is a `HuffmanTablesOffset { segment, offset }`, and `HTreeGroup::htrees`
//!    holds five of them; the owner (the decoder's `VP8LMetadata`) passes the
//!    tables to whatever reads an entry. `segment()` and `segment_mut()`
//!    resolve an index. A second-level table is addressed as in C, relative to
//!    its root entry in the same segment.
//! 2. **Segment lifetimes.** `VP8LHuffmanTablesAllocate` drops the old chain
//!    where C cuts it with `root->next = NULL` without freeing; `Deallocate`
//!    empties `root.start` and drops the chain. As in C, `Deallocate` leaves
//!    `curr_segment`, `root.curr_table` and `root.size` alone; a stale
//!    `curr_segment` panics on use where the C pointer dangles. A new segment
//!    replaces the chain after the current one, as `curr_segment->next = next`
//!    does; the current segment is always the last.
//! 3. **Tree groups.** `VP8LHtreeGroupsNew` returns `Option<Vec<HTreeGroup>>`
//!    with default groups (C leaves them uninitialized). `VP8LHtreeGroupsFree`
//!    only frees, so the owner drops the vector instead, as for `WebPSafeFree`
//!    (see the module documentation of `src/utils/utils.rs`). The flags
//!    `is_trivial_literal`, `is_trivial_code` and `use_packed_table` are
//!    `bool`, because C uses them only as predicates.
//!
//! # Excluded undefined behavior
//!
//! Inputs that are undefined behavior (or assertion failures) in C and are
//! therefore excluded from the oracle comparison (PORTING rule 8):
//!
//! 1. `VP8LBuildHuffmanTable` with a negative code length (C writes outside its
//!    histogram; Rust panics), with no code lengths (C asserts), or with
//!    `root_bits` above `MAX_ALLOWED_CODE_LENGTH` (C reads its histogram past
//!    the end). The decoder passes 7 or 8; the tests use 1 to 12.
//! 2. `VP8LHuffmanTablesAllocate(0)` and `VP8LHtreeGroupsNew(0)`: the zero-size
//!    request of `WebPSafeMalloc` (`src/utils/utils.rs`,
//!    `# Excluded undefined behavior`, item 2). `VP8LHtreeGroupsNew` above
//!    `MAX_HTREE_GROUPS` with a successful allocation asserts in C.
//!
//! # Checks not applicable
//!
//! These checks do not apply, or cannot be made, with the reason:
//!
//! 1. **Host refusals of an allocation.** The size tests of `WebPSafeMalloc`
//!    are compared through negative sizes (`alloc`, `groups`) and counts above
//!    the limit. A refusal by the host of an accepted size (the new segment,
//!    the heap-sorted array, the co-occurrence matrix) cannot be provoked here;
//!    the Rust code returns the same failure where C does (`try_reserve`,
//!    `webp_safe_malloc`).
//! 2. **Uninitialized C memory.** `VP8LHuffmanTablesAllocate`, the new segments
//!    and `VP8LHtreeGroupsNew` use `WebPSafeMalloc` without initialization;
//!    Rust initializes (ADR 0003). `BuildHuffmanTable` writes every entry of
//!    the range it returns for an accepted code, and only that range is
//!    compared. The lines of `CoOccurrenceBuild` are written before they are
//!    read.
//! 3. **`NULL` arguments.** `VP8LHtreeGroupsFree(NULL)`,
//!    `VP8LHuffmanTablesDeallocate(NULL)` and `VP8LColorCacheClear(NULL)` exist
//!    only at the C ABI (PORTING rule 8); none of these headers is public.

use crate::utils::utils::webp_safe_malloc;
use crate::webp::format_constants::{
    HUFFMAN_CODES_PER_META_CODE, MAX_ALLOWED_CODE_LENGTH, MAX_CACHE_BITS, NUM_LENGTH_CODES,
    NUM_LITERAL_CODES,
};

// upstream: src/utils/huffman_utils.h HUFFMAN_TABLE_BITS
pub const HUFFMAN_TABLE_BITS: i32 = 8;
// upstream: src/utils/huffman_utils.h HUFFMAN_TABLE_MASK
pub const HUFFMAN_TABLE_MASK: i32 = (1 << HUFFMAN_TABLE_BITS) - 1;

// upstream: src/utils/huffman_utils.h LENGTHS_TABLE_BITS
pub const LENGTHS_TABLE_BITS: i32 = 7;
// upstream: src/utils/huffman_utils.h LENGTHS_TABLE_MASK
pub const LENGTHS_TABLE_MASK: i32 = (1 << LENGTHS_TABLE_BITS) - 1;

// upstream: src/utils/huffman_utils.h HuffmanCode
/// Huffman lookup table entry.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct HuffmanCode {
    /// Number of bits used for this symbol.
    pub bits: u8,
    /// Symbol value or table offset.
    pub value: u16,
}

impl HuffmanCode {
    /// The C `sizeof(HuffmanCode)`, the `size` argument of its allocations.
    pub const C_SIZE: usize = 4;
}

// upstream: src/utils/huffman_utils.h HuffmanCode32
/// Long version of [`HuffmanCode`] for holding 32-bit values.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct HuffmanCode32 {
    /// Number of bits used for this symbol, or an impossible value if not a
    /// literal code.
    pub bits: i32,
    /// 32-bit packed ARGB value if literal, or non-literal symbol otherwise.
    pub value: u32,
}

/// A C `HuffmanCode*` into the segments of a [`HuffmanTables`]: the segment
/// (0 is `root`, `n` is `next_segments[n - 1]`) and the entry inside it. This
/// type has no C counterpart (ADR 0003, "Readers over input").
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct HuffmanTablesOffset {
    /// The segment that holds the entry.
    pub segment: usize,
    /// The position of the entry inside the segment's `start`.
    pub offset: usize,
}

// upstream: src/utils/huffman_utils.h HuffmanTablesSegment
/// Contiguous memory segment of `HuffmanCode`s.
///
/// The C `next` pointer is the order of [`HuffmanTables::next_segments`].
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct HuffmanTablesSegment {
    /// The codes of the segment; empty where C has `start == NULL`.
    pub start: Vec<HuffmanCode>,
    /// Where we are writing into the segment, as a position in `start`. Starts
    /// at 0 and cannot go beyond `size`.
    pub curr_table: usize,
    /// The number of codes in the segment.
    pub size: i32,
}

impl HuffmanTablesSegment {
    /// The C `sizeof(HuffmanTablesSegment)`: three pointers and an `int`,
    /// padded to the pointer alignment.
    pub const C_SIZE: usize = (3 * size_of::<usize>() + 4).next_multiple_of(size_of::<usize>());
}

// upstream: src/utils/huffman_utils.h HuffmanTables
/// Chained memory segments of `HuffmanCode`s.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct HuffmanTables {
    /// The first segment.
    pub root: HuffmanTablesSegment,
    /// The segments after `root`, in the order of the C `next` chain.
    pub next_segments: Vec<HuffmanTablesSegment>,
    /// Currently processed segment: 0 is `root`, `n` is `next_segments[n - 1]`.
    /// At first, this is `root`.
    pub curr_segment: usize,
}

impl HuffmanTables {
    /// The segment with index `segment`: 0 is `root`, `n` is
    /// `next_segments[n - 1]`.
    ///
    /// # Panics
    ///
    /// Panics when there is no such segment.
    #[must_use]
    pub fn segment(&self, segment: usize) -> &HuffmanTablesSegment {
        match segment.checked_sub(1) {
            None => &self.root,
            Some(index) => &self.next_segments[index],
        }
    }

    /// The segment with index `segment`, mutably; see [`HuffmanTables::segment`].
    ///
    /// # Panics
    ///
    /// Panics when there is no such segment.
    pub fn segment_mut(&mut self, segment: usize) -> &mut HuffmanTablesSegment {
        match segment.checked_sub(1) {
            None => &mut self.root,
            Some(index) => &mut self.next_segments[index],
        }
    }
}

// upstream: src/utils/huffman_utils.h HUFFMAN_PACKED_BITS
pub const HUFFMAN_PACKED_BITS: i32 = 6;
// upstream: src/utils/huffman_utils.h HUFFMAN_PACKED_TABLE_SIZE
pub const HUFFMAN_PACKED_TABLE_SIZE: u32 = 1 << HUFFMAN_PACKED_BITS;

// upstream: src/utils/huffman_utils.h HTreeGroup
/// Huffman table group.
///
/// Includes special handling for the following cases:
///  - `is_trivial_literal`: one common literal base for RED/BLUE/ALPHA (not GREEN)
///  - `is_trivial_code`: only 1 code (no bit is read from bitstream)
///  - `use_packed_table`: few enough literal symbols, so all the bit codes
///    can fit into a small look-up table `packed_table[]`
///
/// The common literal base, if applicable, is stored in `literal_arb`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HTreeGroup {
    /// The first entry of each tree, in the tables of the owner.
    pub htrees: [HuffmanTablesOffset; HUFFMAN_CODES_PER_META_CODE as usize],
    /// True, if huffman trees for Red, Blue & Alpha Symbols are trivial (have
    /// a single code).
    pub is_trivial_literal: bool,
    /// If `is_trivial_literal` is true, this is the ARGB value of the pixel,
    /// with Green channel being set to zero.
    pub literal_arb: u32,
    /// True if `is_trivial_literal` with only one code.
    pub is_trivial_code: bool,
    /// Use packed table below for short literal code.
    pub use_packed_table: bool,
    /// Table mapping input bits to a packed values, or escape case to literal code.
    pub packed_table: [HuffmanCode32; HUFFMAN_PACKED_TABLE_SIZE as usize],
}

impl Default for HTreeGroup {
    fn default() -> Self {
        Self {
            htrees: [HuffmanTablesOffset::default(); HUFFMAN_CODES_PER_META_CODE as usize],
            is_trivial_literal: false,
            literal_arb: 0,
            is_trivial_code: false,
            use_packed_table: false,
            packed_table: [HuffmanCode32::default(); HUFFMAN_PACKED_TABLE_SIZE as usize],
        }
    }
}

impl HTreeGroup {
    /// The C `sizeof(HTreeGroup)`: five pointers, four 32-bit fields and the
    /// packed table of 8-byte entries, padded to the pointer alignment.
    pub const C_SIZE: usize = (HUFFMAN_CODES_PER_META_CODE as usize * size_of::<usize>()
        + 4 * 4
        + HUFFMAN_PACKED_TABLE_SIZE as usize * 8)
        .next_multiple_of(size_of::<usize>());
}

// upstream: src/utils/huffman_utils.c MAX_HTREE_GROUPS
/// Huffman data read via `DecodeImageStream` is represented in two (red and
/// green) bytes.
const MAX_HTREE_GROUPS: i32 = 0x10000;

/// The C conversion of an `int` count to the `uint64_t` element count of
/// `WebPSafeMalloc`, which turns a negative count into a huge one.
fn element_count(count: i32) -> u64 {
    i64::from(count).cast_unsigned()
}

// upstream: src/utils/huffman_utils.c VP8LHtreeGroupsNew
/// Creates the instance of `HTreeGroup` with specified number of tree-groups.
///
/// Returns `None` where C returns `NULL`. C leaves the groups uninitialized;
/// they are default values here (ADR 0003).
///
/// # Panics
///
/// Panics only if an accepted count were negative, which the size test of
/// `WebPSafeMalloc` excludes.
#[must_use]
pub fn vp8l_htree_groups_new(num_htree_groups: i32) -> Option<Vec<HTreeGroup>> {
    let mut htree_groups =
        webp_safe_malloc::<HTreeGroup>(element_count(num_htree_groups), HTreeGroup::C_SIZE)?;
    debug_assert!(num_htree_groups <= MAX_HTREE_GROUPS);
    let count = usize::try_from(num_htree_groups).expect("an accepted count is not negative");
    htree_groups.resize(count, HTreeGroup::default());
    Some(htree_groups)
}

/// Converts an index or a length of the table construction, which is never
/// negative, to a slice position.
fn position(value: i32) -> usize {
    usize::try_from(value).expect("a table position is not negative")
}

// upstream: src/utils/huffman_utils.c GetNextKey
/// Returns reverse(reverse(key, len) + 1, len), where reverse(key, len) is the
/// bit-wise reversal of the len least significant bits of key.
#[must_use]
fn get_next_key(key: u32, len: i32) -> u32 {
    let mut step: u32 = 1 << (len - 1);
    while key & step != 0 {
        step >>= 1;
    }
    if step != 0 {
        (key & (step - 1)) + step
    } else {
        key
    }
}

// upstream: src/utils/huffman_utils.c ReplicateValue
/// Stores code in `table[base]`, `table[base + step]`, `table[base + 2*step]`,
/// ..., `table[base + end - step]`. Assumes that end is an integer multiple of
/// step.
fn replicate_value(
    table: &mut [HuffmanCode],
    base: usize,
    step: i32,
    mut end: i32,
    code: HuffmanCode,
) {
    debug_assert_eq!(end % step, 0);
    loop {
        end -= step;
        table[base + position(end)] = code;
        if end <= 0 {
            break;
        }
    }
}

// upstream: src/utils/huffman_utils.c NextTableBitSize
/// Returns the table width of the next 2nd level table. `count` is the
/// histogram of bit lengths for the remaining symbols, `len` is the code length
/// of the next processed symbol.
#[must_use]
fn next_table_bit_size(count: &[i32], mut len: i32, root_bits: i32) -> i32 {
    let mut left = 1 << (len - root_bits);
    while len < MAX_ALLOWED_CODE_LENGTH {
        left -= count[position(len)];
        if left <= 0 {
            break;
        }
        len += 1;
        left <<= 1;
    }
    len - root_bits
}

/// Truncates a code length or a table width to the `uint8_t` of
/// `HuffmanCode::bits`, as the C cast does.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "the C (uint8_t) cast"
)]
fn code_bits(value: i32) -> u8 {
    value as u8
}

// upstream: src/utils/huffman_utils.c BuildHuffmanTable
/// Builds the table of `code_lengths` into `root_table`, or only validates the
/// code lengths when `root_table` is `None`. `sorted` is a pre-allocated
/// array for sorting symbols by code length; it is `None` exactly when
/// `root_table` is. Returns the built table size, or 0 for an invalid tree.
///
/// # Panics
///
/// Panics for a negative code length, where C writes outside its histogram.
#[expect(
    clippy::too_many_lines,
    reason = "mirrors the single C function BuildHuffmanTable"
)]
fn build_huffman_table(
    mut root_table: Option<&mut [HuffmanCode]>,
    root_bits: i32,
    code_lengths: &[i32],
    mut sorted: Option<&mut [u16]>,
) -> i32 {
    const COUNT_LEN: usize = MAX_ALLOWED_CODE_LENGTH as usize + 1;
    let code_lengths_size =
        i32::try_from(code_lengths.len()).expect("a code length array fits an int");
    // next available space in table, as a position in root_table
    let mut table: usize = 0;
    // total size root table + 2nd level table
    let mut total_size: i32 = 1 << root_bits;
    // number of codes of each length:
    let mut count = [0_i32; COUNT_LEN];
    // offsets in sorted table for each length:
    let mut offset = [0_i32; COUNT_LEN];

    debug_assert!(code_lengths_size != 0);
    debug_assert_eq!(root_table.is_some(), sorted.is_some());
    debug_assert!(root_bits > 0);

    // Build histogram of code lengths.
    for &code_length in code_lengths {
        if code_length > MAX_ALLOWED_CODE_LENGTH {
            return 0;
        }
        count[position(code_length)] += 1;
    }

    // Error, all code lengths are zeros.
    if count[0] == code_lengths_size {
        return 0;
    }

    // Generate offsets into sorted symbol table by code length.
    offset[1] = 0;
    for len in 1..MAX_ALLOWED_CODE_LENGTH as usize {
        if count[len] > (1 << len) {
            return 0;
        }
        offset[len + 1] = offset[len] + count[len];
    }

    // Sort symbols by length, by symbol order within each length.
    for (symbol, &symbol_code_length) in code_lengths.iter().enumerate() {
        if symbol_code_length > 0 {
            let length = position(symbol_code_length);
            if let Some(sorted) = sorted.as_deref_mut() {
                if offset[length] >= code_lengths_size {
                    return 0;
                }
                sorted[position(offset[length])] =
                    u16::try_from(symbol).expect("a symbol index fits uint16_t");
                offset[length] += 1;
            } else {
                offset[length] += 1;
            }
        }
    }

    // Special case code with only one value.
    if offset[MAX_ALLOWED_CODE_LENGTH as usize] == 1 {
        if let Some(sorted) = sorted.as_deref() {
            let code = HuffmanCode {
                bits: 0,
                value: sorted[0],
            };
            let root_table = root_table
                .as_deref_mut()
                .expect("a sorted array comes with a root table");
            replicate_value(root_table, table, 1, total_size, code);
        }
        return total_size;
    }

    {
        // step size to replicate values in current table
        let mut step: i32;
        // low bits for current root entry
        let mut low: u32 = 0xffff_ffff;
        // mask for low bits
        let mask: u32 = (total_size - 1).cast_unsigned();
        // reversed prefix code
        let mut key: u32 = 0;
        // number of Huffman tree nodes
        let mut num_nodes: i32 = 1;
        // number of open branches in current tree level
        let mut num_open: i32 = 1;
        // key length of current table
        let mut table_bits: i32;
        // size of current table
        let mut table_size: i32 = 1 << root_bits;
        // symbol index in the sorted table
        let mut symbol: usize = 0;
        // Fill in root table.
        let mut len: i32 = 1;
        step = 2;
        while len <= root_bits {
            num_open <<= 1;
            num_nodes += num_open;
            num_open -= count[position(len)];
            if num_open < 0 {
                return 0;
            }
            if let (Some(root_table), Some(sorted)) = (root_table.as_deref_mut(), sorted.as_deref())
            {
                while count[position(len)] > 0 {
                    let code = HuffmanCode {
                        bits: code_bits(len),
                        value: sorted[symbol],
                    };
                    symbol += 1;
                    replicate_value(root_table, table + key as usize, step, table_size, code);
                    key = get_next_key(key, len);
                    count[position(len)] -= 1;
                }
            }
            len += 1;
            step <<= 1;
        }

        // Fill in 2nd level tables and add pointers to root table.
        len = root_bits + 1;
        step = 2;
        while len <= MAX_ALLOWED_CODE_LENGTH {
            num_open <<= 1;
            num_nodes += num_open;
            num_open -= count[position(len)];
            if num_open < 0 {
                return 0;
            }
            while count[position(len)] > 0 {
                if (key & mask) != low {
                    if root_table.is_some() {
                        table += position(table_size);
                    }
                    table_bits = next_table_bit_size(&count, len, root_bits);
                    table_size = 1 << table_bits;
                    total_size += table_size;
                    low = key & mask;
                    if let Some(root_table) = root_table.as_deref_mut() {
                        let entry = &mut root_table[low as usize];
                        entry.bits = code_bits(table_bits + root_bits);
                        entry.value = table_value(table - low as usize);
                    }
                }
                if let (Some(root_table), Some(sorted)) =
                    (root_table.as_deref_mut(), sorted.as_deref())
                {
                    let code = HuffmanCode {
                        bits: code_bits(len - root_bits),
                        value: sorted[symbol],
                    };
                    symbol += 1;
                    replicate_value(
                        root_table,
                        table + (key >> root_bits) as usize,
                        step,
                        table_size,
                        code,
                    );
                }
                key = get_next_key(key, len);
                count[position(len)] -= 1;
            }
            len += 1;
            step <<= 1;
        }

        // Check if tree is full.
        if num_nodes != 2 * offset[MAX_ALLOWED_CODE_LENGTH as usize] - 1 {
            return 0;
        }
    }

    total_size
}

/// Truncates the distance from a root entry to its 2nd level table to the
/// `uint16_t` of `HuffmanCode::value`, as the C cast does.
#[expect(clippy::cast_possible_truncation, reason = "the C (uint16_t) cast")]
fn table_value(distance: usize) -> u16 {
    distance as u16
}

// upstream: src/utils/huffman_utils.c MAX_CODE_LENGTHS_SIZE
/// Maximum `code_lengths_size` is 2328 (reached for 11-bit
/// `color_cache_bits`). More commonly, the value is around ~280.
const MAX_CODE_LENGTHS_SIZE: i32 = (1 << MAX_CACHE_BITS) + NUM_LITERAL_CODES + NUM_LENGTH_CODES;
// upstream: src/utils/huffman_utils.c SORTED_SIZE_CUTOFF
/// Cut-off value for switching between heap and stack allocation.
const SORTED_SIZE_CUTOFF: usize = 512;

// upstream: src/utils/huffman_utils.c VP8LBuildHuffmanTable
/// Builds Huffman lookup table assuming code lengths are in symbol order.
///
/// `code_lengths` holds the `code_lengths_size` lengths of the C call. With
/// `root_table == None` the code lengths are only validated, and nothing is
/// built. The table is built at `curr_table` of the current segment, which
/// grows a new segment when the current one is too small; `curr_table` itself
/// does not move. Returns built table size or 0 in case of error (invalid tree
/// or memory error).
///
/// # Panics
///
/// Panics for an empty `code_lengths` or a negative code length; C asserts the
/// first and writes outside its histogram for the second.
#[must_use]
pub fn vp8l_build_huffman_table(
    root_table: Option<&mut HuffmanTables>,
    root_bits: i32,
    code_lengths: &[i32],
) -> i32 {
    let total_size = build_huffman_table(None, root_bits, code_lengths, None);
    let code_lengths_size =
        i32::try_from(code_lengths.len()).expect("a code length array fits an int");
    debug_assert!(code_lengths_size <= MAX_CODE_LENGTHS_SIZE);
    let Some(root_table) = root_table else {
        return total_size;
    };
    if total_size == 0 {
        return total_size;
    }

    let curr_segment = root_table.segment(root_table.curr_segment);
    if curr_segment.curr_table + position(total_size) >= position(curr_segment.size) {
        // If 'root_table' does not have enough memory, allocate a new segment.
        // The available part of root_table->curr_segment is left unused because we
        // need a contiguous buffer.
        let segment_size = curr_segment.size;
        // The C segment struct: WebPSafeMalloc(1, sizeof(*next)) always passes the
        // size test, so only a refusal of the host remains.
        if root_table.next_segments.try_reserve(1).is_err() {
            return 0;
        }
        // Fill the new segment.
        // We need at least 'total_size' but if that value is small, it is better to
        // allocate a big chunk to prevent more allocations later. 'segment_size' is
        // therefore chosen (any other arbitrary value could be chosen).
        let size = if total_size > segment_size {
            total_size
        } else {
            segment_size
        };
        let Some(mut start) =
            webp_safe_malloc::<HuffmanCode>(element_count(size), HuffmanCode::C_SIZE)
        else {
            return 0;
        };
        start.resize(position(size), HuffmanCode::default());
        // Point to the new segment: it becomes the `next` of the current segment.
        root_table.next_segments.truncate(root_table.curr_segment);
        root_table.next_segments.push(HuffmanTablesSegment {
            start,
            curr_table: 0,
            size,
        });
        root_table.curr_segment = root_table.next_segments.len();
    }
    let curr_segment = root_table.segment_mut(root_table.curr_segment);
    let table = &mut curr_segment.start[curr_segment.curr_table..];
    if code_lengths.len() <= SORTED_SIZE_CUTOFF {
        // use local stack-allocated array.
        let mut sorted = [0_u16; SORTED_SIZE_CUTOFF];
        build_huffman_table(Some(table), root_bits, code_lengths, Some(&mut sorted));
    } else {
        // rare case. Use heap allocation.
        let Some(mut sorted) = webp_safe_malloc::<u16>(element_count(code_lengths_size), 2) else {
            return 0;
        };
        sorted.resize(code_lengths.len(), 0);
        build_huffman_table(Some(table), root_bits, code_lengths, Some(&mut sorted));
    }
    total_size
}

// upstream: src/utils/huffman_utils.c VP8LHuffmanTablesAllocate
/// Allocates a `HuffmanTables` with `size` contiguous `HuffmanCode`s. Returns
/// `false` on memory allocation error, `true` otherwise.
///
/// As in C, the current segment becomes `root` and the chain after it is cut
/// before the allocation, and a failure leaves `root.start` empty while
/// `root.curr_table` and `root.size` keep their values.
#[must_use]
pub fn vp8l_huffman_tables_allocate(size: i32, huffman_tables: &mut HuffmanTables) -> bool {
    // Have 'segment' point to the first segment for now, 'root'.
    huffman_tables.curr_segment = 0;
    huffman_tables.next_segments = Vec::new();
    let root = &mut huffman_tables.root;
    // Allocate root.
    let Some(mut start) = webp_safe_malloc::<HuffmanCode>(element_count(size), HuffmanCode::C_SIZE)
    else {
        root.start = Vec::new();
        return false;
    };
    start.resize(position(size), HuffmanCode::default());
    root.start = start;
    root.curr_table = 0;
    root.size = size;
    true
}

// upstream: src/utils/huffman_utils.c VP8LHuffmanTablesDeallocate
/// Frees the codes of every segment: `root.start` becomes empty and the chain
/// after it is dropped. `curr_segment`, `root.curr_table` and `root.size` keep
/// their values, as in C.
pub fn vp8l_huffman_tables_deallocate(huffman_tables: &mut HuffmanTables) {
    // Free the root node.
    huffman_tables.root.start = Vec::new();
    // Free the following nodes.
    huffman_tables.next_segments = Vec::new();
}
