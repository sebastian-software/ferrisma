// libwebp v1.6.0, src/enc/backward_references_enc.c:
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

// libwebp v1.6.0, src/enc/backward_references_enc.h:
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

//! Translation of `src/enc/backward_references_enc.c` and `src/enc/backward_references_enc.h` from libwebp v1.6.0.
//!
//! Work package: WP11 (VP8L encoder).
//!
//! # Deviations from upstream
//!
//! Changes of representation that the ADRs allow, or behavior that the Rust
//! side cannot reproduce exactly. Each records the reason.
//!
//! 1. **Allocation totals.** Where C allocates one block and carves several
//!    arrays from it (the reference blocks, the histogram sets, the cost model,
//!    the Huffman code arrays of `GetHuffBitLengthsAndCodes`, see
//!    `HuffmanTreeCode`), the Rust code applies the size test of
//!    `WebPSafeMalloc` to the C total, including `sizeof` of the header and
//!    `WEBP_ALIGN_CST` padding, before it allocates separate buffers
//!    (`safe_malloc_test` reserves zero-sized elements, so it allocates
//!    nothing). The `C_SIZE` constants are compared with the C compiler.
//!
//! # Excluded undefined behavior
//!
//! Inputs that are undefined behavior (or assertion failures) in C and are
//! therefore excluded from the oracle comparison (PORTING rule 8):
//!
//! 1. Pictures large enough that the `int` progress products overflow, such as
//!    `percent_range * pos` in `VP8LHashChainFill` (about 100 million pixels);
//!    the tests stay far below.
//!
//! # Checks not applicable
//!
//! These checks do not apply, or cannot be made, with the reason:
//!
//! 1. **Host refusals of an allocation.** The size tests of `WebPSafeMalloc`
//!    and `WebPSafeCalloc` are reproduced with the C totals (item 1 of
//!    `# Deviations from upstream`), but a refusal by the host of an accepted
//!    size cannot be provoked; the Rust code fails where C fails
//!    (`try_reserve`). The fixed-size structures that C allocates once
//!    (`VP8LEncoder`, `CostManager`, `CostModel`, `Histograms`) are Rust values
//!    or boxes; their size tests always pass in C.

use crate::dsp::lossless_common::{WEBP_UINT64_MAX, vp8l_prefix_encode};
use crate::dsp::lossless_enc::vp8l_vector_mismatch;
use crate::enc::backward_references_cost_enc::vp8l_backward_references_trace_backwards;
use crate::enc::histogram_enc::{
    VP8LHistogram, vp8l_allocate_histogram, vp8l_histogram_create, vp8l_histogram_estimate_bits,
    vp8l_histogram_init,
};
use crate::enc::webp_enc::{webp_encoding_set_error, webp_report_progress};
use crate::utils::color_cache_utils::{
    VP8LColorCache, vp8l_color_cache_clear, vp8l_color_cache_contains, vp8l_color_cache_get_index,
    vp8l_color_cache_init, vp8l_color_cache_insert, vp8l_color_cache_lookup, vp8l_color_cache_set,
    vp8l_hash_pix,
};
use crate::utils::utils::webp_safe_malloc;
use crate::webp::encode::{WebPEncodingError, WebPPicture};
use crate::webp::format_constants::{NUM_LENGTH_CODES, NUM_LITERAL_CODES};

//------------------------------------------------------------------------------
// Rust-only allocation helpers (ADR 0003, allocation limits).

/// The C size test of `WebPSafeMalloc(nmemb, size)` without an allocation (a
/// reservation of zero-sized elements allocates nothing). Used where C
/// allocates one block that the Rust code allocates as separate buffers: the
/// same totals fail.
#[must_use]
pub(crate) fn safe_malloc_test(nmemb: u64, size: usize) -> bool {
    webp_safe_malloc::<()>(nmemb, size).is_some()
}

/// `count` copies of `value`, or `None` when the host refuses the reservation,
/// which is where the C allocation returns `NULL`.
#[must_use]
pub(crate) fn try_filled<T: Clone>(count: usize, value: T) -> Option<Vec<T>> {
    let mut buffer = Vec::new();
    buffer.try_reserve_exact(count).ok()?;
    buffer.resize(count, value);
    Some(buffer)
}

/// Converts a C `int` index or count that is not negative to `usize`.
#[inline]
pub(crate) fn to_index(value: i32) -> usize {
    usize::try_from(value).expect("a C index or count is not negative")
}

/// A shared and a mutable borrow of two different elements of one slice, for
/// C calls that pass two pointers into one array.
///
/// # Panics
///
/// Panics when `shared == mutable`.
pub(crate) fn split_pair<T>(slice: &mut [T], shared: usize, mutable: usize) -> (&T, &mut T) {
    assert_ne!(shared, mutable, "the two elements differ");
    if shared < mutable {
        let (low, high) = slice.split_at_mut(mutable);
        (&low[shared], &mut high[0])
    } else {
        let (low, high) = slice.split_at_mut(shared);
        (&high[0], &mut low[mutable])
    }
}

/// Two mutable borrows of two different elements of one slice.
///
/// # Panics
///
/// Panics when `a == b`.
pub(crate) fn split_two<T>(slice: &mut [T], a: usize, b: usize) -> (&mut T, &mut T) {
    assert_ne!(a, b, "the two elements differ");
    if a < b {
        let (low, high) = slice.split_at_mut(b);
        (&mut low[a], &mut high[0])
    } else {
        let (low, high) = slice.split_at_mut(a);
        (&mut high[0], &mut low[b])
    }
}

//------------------------------------------------------------------------------
// backward_references_enc.h

// upstream: src/enc/backward_references_enc.h MAX_COLOR_CACHE_BITS
/// The maximum allowed limit is 11.
pub const MAX_COLOR_CACHE_BITS: i32 = 10;

// -----------------------------------------------------------------------------
// PixOrCopy

// upstream: src/enc/backward_references_enc.h Mode
/// The kind of a `PixOrCopy` entry (C: `enum Mode`).
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Mode(pub u32);

#[expect(non_upper_case_globals, reason = "C enumerator names")]
impl Mode {
    // upstream: src/enc/backward_references_enc.h kLiteral
    pub const kLiteral: Self = Self(0);
    // upstream: src/enc/backward_references_enc.h kCacheIdx
    pub const kCacheIdx: Self = Self(1);
    // upstream: src/enc/backward_references_enc.h kCopy
    pub const kCopy: Self = Self(2);
    // upstream: src/enc/backward_references_enc.h kNone
    pub const kNone: Self = Self(3);
}

/// The C conversion of a `Mode` enumerator to the `uint8_t` field.
#[expect(
    clippy::cast_possible_truncation,
    reason = "the enumerators fit uint8_t, as the C assignment assumes"
)]
const fn mode_byte(mode: Mode) -> u8 {
    mode.0 as u8
}

// upstream: src/enc/backward_references_enc.h PixOrCopy
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PixOrCopy {
    /// mode as `uint8_t` to make the memory layout to be exactly 8 bytes.
    pub mode: u8,
    pub len: u16,
    pub argb_or_distance: u32,
}

impl PixOrCopy {
    /// The C `sizeof(PixOrCopy)`.
    pub const C_SIZE: usize = 8;
}

// upstream: src/enc/backward_references_enc.h PixOrCopyCreateCopy
#[must_use]
#[inline]
pub fn pix_or_copy_create_copy(distance: u32, len: u16) -> PixOrCopy {
    PixOrCopy {
        mode: mode_byte(Mode::kCopy),
        argb_or_distance: distance,
        len,
    }
}

// upstream: src/enc/backward_references_enc.h PixOrCopyCreateCacheIdx
#[must_use]
#[inline]
pub fn pix_or_copy_create_cache_idx(idx: i32) -> PixOrCopy {
    debug_assert!(idx >= 0);
    debug_assert!(idx < (1 << MAX_COLOR_CACHE_BITS));
    PixOrCopy {
        mode: mode_byte(Mode::kCacheIdx),
        argb_or_distance: idx.cast_unsigned(),
        len: 1,
    }
}

// upstream: src/enc/backward_references_enc.h PixOrCopyCreateLiteral
#[must_use]
#[inline]
pub fn pix_or_copy_create_literal(argb: u32) -> PixOrCopy {
    PixOrCopy {
        mode: mode_byte(Mode::kLiteral),
        argb_or_distance: argb,
        len: 1,
    }
}

// upstream: src/enc/backward_references_enc.h PixOrCopyIsLiteral
#[must_use]
#[inline]
pub fn pix_or_copy_is_literal(p: &PixOrCopy) -> bool {
    p.mode == mode_byte(Mode::kLiteral)
}

// upstream: src/enc/backward_references_enc.h PixOrCopyIsCacheIdx
#[must_use]
#[inline]
pub fn pix_or_copy_is_cache_idx(p: &PixOrCopy) -> bool {
    p.mode == mode_byte(Mode::kCacheIdx)
}

// upstream: src/enc/backward_references_enc.h PixOrCopyIsCopy
#[must_use]
#[inline]
pub fn pix_or_copy_is_copy(p: &PixOrCopy) -> bool {
    p.mode == mode_byte(Mode::kCopy)
}

// upstream: src/enc/backward_references_enc.h PixOrCopyLiteral
#[must_use]
#[inline]
pub fn pix_or_copy_literal(p: &PixOrCopy, component: i32) -> u32 {
    debug_assert_eq!(p.mode, mode_byte(Mode::kLiteral));
    (p.argb_or_distance >> (component * 8)) & 0xff
}

// upstream: src/enc/backward_references_enc.h PixOrCopyLength
#[must_use]
#[inline]
pub fn pix_or_copy_length(p: &PixOrCopy) -> u32 {
    u32::from(p.len)
}

// upstream: src/enc/backward_references_enc.h PixOrCopyCacheIdx
#[must_use]
#[inline]
pub fn pix_or_copy_cache_idx(p: &PixOrCopy) -> u32 {
    debug_assert_eq!(p.mode, mode_byte(Mode::kCacheIdx));
    debug_assert!(p.argb_or_distance < (1_u32 << MAX_COLOR_CACHE_BITS));
    p.argb_or_distance
}

// upstream: src/enc/backward_references_enc.h PixOrCopyDistance
#[must_use]
#[inline]
pub fn pix_or_copy_distance(p: &PixOrCopy) -> u32 {
    debug_assert_eq!(p.mode, mode_byte(Mode::kCopy));
    p.argb_or_distance
}

// -----------------------------------------------------------------------------
// VP8LHashChain

// upstream: src/enc/backward_references_enc.h HASH_BITS
pub const HASH_BITS: i32 = 18;
// upstream: src/enc/backward_references_enc.h HASH_SIZE
pub const HASH_SIZE: i32 = 1 << HASH_BITS;

// upstream: src/enc/backward_references_enc.h MAX_LENGTH_BITS
/// If you change this, you need `MAX_LENGTH_BITS + WINDOW_SIZE_BITS <= 32` as it
/// is used in `VP8LHashChain`.
pub const MAX_LENGTH_BITS: i32 = 12;
// upstream: src/enc/backward_references_enc.h WINDOW_SIZE_BITS
pub const WINDOW_SIZE_BITS: i32 = 20;
// upstream: src/enc/backward_references_enc.h MAX_LENGTH
/// We want the max value to be attainable and stored in `MAX_LENGTH_BITS` bits.
pub const MAX_LENGTH: i32 = (1 << MAX_LENGTH_BITS) - 1;
const _: () = assert!(
    MAX_LENGTH_BITS + WINDOW_SIZE_BITS <= 32,
    "MAX_LENGTH_BITS + WINDOW_SIZE_BITS > 32"
);

// upstream: src/enc/backward_references_enc.h VP8LHashChain
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct VP8LHashChain {
    /// The 20 most significant bits contain the offset at which the best match
    /// is found. These 20 bits are the limit defined by
    /// `GetWindowSizeForHashChain` (through `WINDOW_SIZE = 1<<20`).
    /// The lower 12 bits contain the length of the match. The 12 bit limit is
    /// defined in `MaxFindCopyLength` with `MAX_LENGTH=4096`.
    /// Empty where C has `NULL`.
    pub offset_length: Vec<u32>,
    /// This is the maximum size of the `hash_chain` that can be constructed.
    /// Typically this is the pixel count (width x height) for a given image.
    pub size: i32,
}

// upstream: src/enc/backward_references_enc.h VP8LHashChainFindOffset
#[must_use]
#[inline]
pub fn vp8l_hash_chain_find_offset(p: &VP8LHashChain, base_position: i32) -> i32 {
    (p.offset_length[to_index(base_position)] >> MAX_LENGTH_BITS).cast_signed()
}

// upstream: src/enc/backward_references_enc.h VP8LHashChainFindLength
#[must_use]
#[inline]
pub fn vp8l_hash_chain_find_length(p: &VP8LHashChain, base_position: i32) -> i32 {
    (p.offset_length[to_index(base_position)] & ((1_u32 << MAX_LENGTH_BITS) - 1)).cast_signed()
}

// upstream: src/enc/backward_references_enc.h VP8LHashChainFindCopy
/// Returns `(offset, length)`, the two out-parameters of the C function.
#[must_use]
#[inline]
pub fn vp8l_hash_chain_find_copy(p: &VP8LHashChain, base_position: i32) -> (i32, i32) {
    (
        vp8l_hash_chain_find_offset(p, base_position),
        vp8l_hash_chain_find_length(p, base_position),
    )
}

// -----------------------------------------------------------------------------
// VP8LBackwardRefs (block-based backward-references storage)

// upstream: src/enc/backward_references_enc.h MAX_REFS_BLOCK_PER_IMAGE
/// maximum number of reference blocks the image will be segmented into
pub const MAX_REFS_BLOCK_PER_IMAGE: i32 = 16;

// upstream: src/enc/backward_references_enc.c PixOrCopyBlock
/// One block of references. The blocks of a list are kept in a vector in the
/// order of the C `next` chain, so the block has no `next` field.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PixOrCopyBlock {
    /// data start: `block_size` entries, of which `size` are used.
    pub start: Vec<PixOrCopy>,
    /// currently used size
    pub size: i32,
}

impl PixOrCopyBlock {
    /// The C `sizeof(PixOrCopyBlock)`: two pointers and an `int`, padded to the
    /// pointer alignment.
    pub const C_SIZE: usize = 3 * size_of::<usize>();
}

// upstream: src/enc/backward_references_enc.h VP8LBackwardRefs
/// Container for blocks chain. C's `tail` and `last_block` pointers always
/// point at the end of the `refs` chain, so they are derived from `refs`: the
/// last block is `refs.last()`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct VP8LBackwardRefs {
    /// common block-size
    pub block_size: i32,
    /// set to true if some memory error occurred
    pub error: bool,
    /// list of currently used blocks
    pub refs: Vec<PixOrCopyBlock>,
    /// free-list; the head of the C list is the last element.
    pub free_blocks: Vec<PixOrCopyBlock>,
}

// upstream: src/enc/backward_references_enc.h VP8LRefsCursor
/// Cursor for iterating on references content. C's `cur_pos` and `last_pos`
/// pointers are positions in the block `cur_block` of the list, and the `NULL`
/// position at the end is `ok == false`. The functions that move the cursor take
/// the list, which C reaches through the block's `next` pointer.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct VP8LRefsCursor {
    /// current position, in the block `cur_block`
    pub cur_pos: usize,
    /// current block in the refs list
    pub cur_block: usize,
    /// sentinel for switching to next block
    pub last_pos: usize,
    /// `false` where C's `cur_pos` is `NULL`.
    pub ok: bool,
}

impl VP8LRefsCursor {
    /// The entry at the cursor (C: `*c.cur_pos`).
    #[must_use]
    pub fn get<'r>(&self, refs: &'r VP8LBackwardRefs) -> &'r PixOrCopy {
        &refs.refs[self.cur_block].start[self.cur_pos]
    }

    /// The entry at the cursor, mutably.
    pub fn get_mut<'r>(&self, refs: &'r mut VP8LBackwardRefs) -> &'r mut PixOrCopy {
        &mut refs.refs[self.cur_block].start[self.cur_pos]
    }
}

// upstream: src/enc/backward_references_enc.h VP8LRefsCursorOk
/// Returns true if cursor is pointing at a valid position.
#[must_use]
#[inline]
pub fn vp8l_refs_cursor_ok(c: &VP8LRefsCursor) -> bool {
    c.ok
}

// upstream: src/enc/backward_references_enc.h VP8LRefsCursorNext
/// Move to next position, or `NULL`. Should not be called if
/// `!VP8LRefsCursorOk()`.
#[inline]
pub fn vp8l_refs_cursor_next(c: &mut VP8LRefsCursor, refs: &VP8LBackwardRefs) {
    debug_assert!(vp8l_refs_cursor_ok(c));
    c.cur_pos += 1;
    if c.cur_pos == c.last_pos {
        vp8l_refs_cursor_next_block(c, refs);
    }
}

// -----------------------------------------------------------------------------
// Main entry points

// upstream: src/enc/backward_references_enc.h VP8LLZ77Type
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct VP8LLZ77Type(pub u32);

#[expect(non_upper_case_globals, reason = "C enumerator names")]
impl VP8LLZ77Type {
    // upstream: src/enc/backward_references_enc.h kLZ77Standard
    pub const kLZ77Standard: Self = Self(1);
    // upstream: src/enc/backward_references_enc.h kLZ77RLE
    pub const kLZ77RLE: Self = Self(2);
    // upstream: src/enc/backward_references_enc.h kLZ77Box
    pub const kLZ77Box: Self = Self(4);
}

//------------------------------------------------------------------------------
// backward_references_enc.c

// upstream: src/enc/backward_references_enc.c MIN_BLOCK_SIZE
/// minimum block size for backward references
const MIN_BLOCK_SIZE: i32 = 256;

// upstream: src/enc/backward_references_enc.c WINDOW_SIZE
/// 1M window (4M bytes) minus 120 special codes for short distances.
const WINDOW_SIZE: i32 = (1 << WINDOW_SIZE_BITS) - 120;

// upstream: src/enc/backward_references_enc.c MIN_LENGTH
/// Minimum number of pixels for which it is cheaper to encode a
/// distance + length instead of each pixel as a literal.
const MIN_LENGTH: i32 = 4;

// -----------------------------------------------------------------------------

// upstream: src/enc/backward_references_enc.c plane_to_code_lut
static PLANE_TO_CODE_LUT: [u8; 128] = [
    96, 73, 55, 39, 23, 13, 5, 1, 255, 255, 255, 255, 255, 255, 255, 255, 101, 78, 58, 42, 26, 16,
    8, 2, 0, 3, 9, 17, 27, 43, 59, 79, 102, 86, 62, 46, 32, 20, 10, 6, 4, 7, 11, 21, 33, 47, 63,
    87, 105, 90, 70, 52, 37, 28, 18, 14, 12, 15, 19, 29, 38, 53, 71, 91, 110, 99, 82, 66, 48, 35,
    30, 24, 22, 25, 31, 36, 49, 67, 83, 100, 115, 108, 94, 76, 64, 50, 44, 40, 34, 41, 45, 51, 65,
    77, 95, 109, 118, 113, 103, 92, 80, 68, 60, 56, 54, 57, 61, 69, 81, 93, 104, 114, 119, 116,
    111, 106, 97, 88, 84, 74, 72, 75, 85, 89, 98, 107, 112, 117,
];

// upstream: src/enc/backward_references_enc.c VP8LDistanceToPlaneCode
#[must_use]
pub fn vp8l_distance_to_plane_code(xsize: i32, dist: i32) -> i32 {
    let yoffset = dist / xsize;
    let xoffset = dist - yoffset * xsize;
    if xoffset <= 8 && yoffset < 8 {
        i32::from(PLANE_TO_CODE_LUT[to_index(yoffset * 16 + 8 - xoffset)]) + 1
    } else if xoffset > xsize - 8 && yoffset < 7 {
        i32::from(PLANE_TO_CODE_LUT[to_index((yoffset + 1) * 16 + 8 + (xsize - xoffset))]) + 1
    } else {
        dist + 120
    }
}

// upstream: src/enc/backward_references_enc.c FindMatchLength
/// Returns the exact index where `array1` and `array2` are different. For an
/// index inferior or equal to `best_len_match`, the return value just has to be
/// strictly inferior to `best_len_match`. The current behavior is to return 0 if
/// this index is `best_len_match`, and the index itself otherwise.
/// If no two elements are the same, it returns `max_limit`.
#[inline]
fn find_match_length(array1: &[u32], array2: &[u32], best_len_match: i32, max_limit: i32) -> i32 {
    // Before 'expensive' linear match, check if the two arrays match at the
    // current best length index.
    if array1[to_index(best_len_match)] != array2[to_index(best_len_match)] {
        return 0;
    }

    vp8l_vector_mismatch(array1, array2, max_limit)
}

// -----------------------------------------------------------------------------
//  VP8LBackwardRefs

// upstream: src/enc/backward_references_enc.c VP8LClearBackwardRefs
/// Moves every used block to the free list, ahead of the blocks already there,
/// as C links the free list behind the used list.
pub fn vp8l_clear_backward_refs(refs: &mut VP8LBackwardRefs) {
    // recycle all blocks at once
    while let Some(block) = refs.refs.pop() {
        refs.free_blocks.push(block);
    }
}

// upstream: src/enc/backward_references_enc.c VP8LBackwardRefsClear
/// Release memory for backward references.
pub fn vp8l_backward_refs_clear(refs: &mut VP8LBackwardRefs) {
    vp8l_clear_backward_refs(refs);
    refs.free_blocks = Vec::new();
}

// upstream: src/enc/backward_references_enc.c BackwardRefsSwap
/// Swaps the content of two `VP8LBackwardRefs`. The C fix-up of the `tail`
/// pointers has no counterpart: the tails are derived.
fn backward_refs_swap(refs1: &mut VP8LBackwardRefs, refs2: &mut VP8LBackwardRefs) {
    std::mem::swap(refs1, refs2);
}

// upstream: src/enc/backward_references_enc.c VP8LBackwardRefsInit
/// Initialize the object. `block_size` is the common block size to store
/// references (typically, width * height / `MAX_REFS_BLOCK_PER_IMAGE`).
pub fn vp8l_backward_refs_init(refs: &mut VP8LBackwardRefs, block_size: i32) {
    *refs = VP8LBackwardRefs::default();
    refs.block_size = if block_size < MIN_BLOCK_SIZE {
        MIN_BLOCK_SIZE
    } else {
        block_size
    };
}

// upstream: src/enc/backward_references_enc.c VP8LRefsCursorInit
/// Returns a cursor positioned at the beginning of the references list.
#[must_use]
pub fn vp8l_refs_cursor_init(refs: &VP8LBackwardRefs) -> VP8LRefsCursor {
    match refs.refs.first() {
        Some(block) => VP8LRefsCursor {
            cur_pos: 0,
            cur_block: 0,
            last_pos: to_index(block.size),
            ok: true,
        },
        None => VP8LRefsCursor::default(),
    }
}

// upstream: src/enc/backward_references_enc.c VP8LRefsCursorNextBlock
/// Move to next block of references. Internal, not to be called directly.
pub fn vp8l_refs_cursor_next_block(c: &mut VP8LRefsCursor, refs: &VP8LBackwardRefs) {
    let b = c.cur_block + 1;
    if let Some(block) = refs.refs.get(b) {
        c.cur_pos = 0;
        c.last_pos = to_index(block.size);
        c.ok = true;
    } else {
        c.cur_pos = 0;
        c.last_pos = 0;
        c.ok = false;
    }
    c.cur_block = b;
}

// upstream: src/enc/backward_references_enc.c BackwardRefsNewBlock
/// Create a new block, either from the free list or allocated. Returns `false`
/// where C returns `NULL`, with `error` set; the new block is the last of
/// `refs`.
fn backward_refs_new_block(refs: &mut VP8LBackwardRefs) -> bool {
    let block = if let Some(block) = refs.free_blocks.pop() {
        // recycle from free-list
        block
    } else {
        // allocate new memory chunk
        let block_size = to_index(refs.block_size);
        let total_size =
            PixOrCopyBlock::C_SIZE as u64 + block_size as u64 * PixOrCopy::C_SIZE as u64;
        let start = usize::try_from(total_size)
            .ok()
            .filter(|&total| safe_malloc_test(1, total))
            .and_then(|_| try_filled(block_size, PixOrCopy::default()));
        let Some(start) = start.filter(|_| refs.refs.try_reserve(1).is_ok()) else {
            refs.error = true;
            return false;
        };
        PixOrCopyBlock { start, size: 0 }
    };
    refs.refs.push(PixOrCopyBlock { size: 0, ..block });
    true
}

// upstream: src/enc/backward_references_enc.c BackwardRefsClone
/// Return 1 on success, 0 on error.
fn backward_refs_clone(from: &VP8LBackwardRefs, to: &mut VP8LBackwardRefs) -> bool {
    vp8l_clear_backward_refs(to);
    for block_from in &from.refs {
        if !backward_refs_new_block(to) {
            return false;
        }
        let block_to = to.refs.last_mut().expect("the new block is the last one");
        let size = to_index(block_from.size);
        block_to.start[..size].copy_from_slice(&block_from.start[..size]);
        block_to.size = block_from.size;
    }
    true
}

// upstream: src/enc/backward_references_enc.c VP8LBackwardRefsCursorAdd
/// Appends `v`, in a new block when the last one is full. A failed allocation
/// sets `error` and drops `v`, as in C.
///
/// # Panics
///
/// Panics when a new block is not the last one, which cannot happen.
pub fn vp8l_backward_refs_cursor_add(refs: &mut VP8LBackwardRefs, v: PixOrCopy) {
    let full = refs
        .refs
        .last()
        .is_none_or(|block| block.size == refs.block_size);
    if full && !backward_refs_new_block(refs) {
        return; // refs->error is set
    }
    let b = refs.refs.last_mut().expect("a block to add to");
    b.start[to_index(b.size)] = v;
    b.size += 1;
}

// -----------------------------------------------------------------------------
// Hash chains

// upstream: src/enc/backward_references_enc.c VP8LHashChainInit
/// Must be called first, to set size.
#[must_use]
pub fn vp8l_hash_chain_init(p: &mut VP8LHashChain, size: i32) -> bool {
    debug_assert_eq!(p.size, 0);
    debug_assert_eq!(p.offset_length.len(), 0);
    debug_assert!(size > 0);
    let count = i64::from(size).cast_unsigned();
    let Some(offset_length) = webp_safe_malloc::<u32>(count, 4).map(|mut buffer| {
        buffer.resize(to_index(size), 0);
        buffer
    }) else {
        return false;
    };
    p.offset_length = offset_length;
    p.size = size;

    true
}

// upstream: src/enc/backward_references_enc.c VP8LHashChainClear
/// release memory
pub fn vp8l_hash_chain_clear(p: &mut VP8LHashChain) {
    p.size = 0;
    p.offset_length = Vec::new();
}

// -----------------------------------------------------------------------------

// upstream: src/enc/backward_references_enc.c kHashMultiplierHi
const HASH_MULTIPLIER_HI: u32 = 0xc6a4_a793;
// upstream: src/enc/backward_references_enc.c kHashMultiplierLo
const HASH_MULTIPLIER_LO: u32 = 0x5bd1_e996;

// upstream: src/enc/backward_references_enc.c GetPixPairHash64
/// The hash of the pixel pair `argb[0]`, `argb[1]`; the products wrap.
#[inline]
fn get_pix_pair_hash64(argb: &[u32]) -> u32 {
    let mut key = argb[1].wrapping_mul(HASH_MULTIPLIER_HI);
    key = key.wrapping_add(argb[0].wrapping_mul(HASH_MULTIPLIER_LO));
    key >> (32 - HASH_BITS)
}

// upstream: src/enc/backward_references_enc.c GetMaxItersForQuality
/// Returns the maximum number of hash chain lookups to do for a
/// given compression quality. Return value in range [8, 86].
fn get_max_iters_for_quality(quality: i32) -> i32 {
    8 + (quality * quality) / 128
}

// upstream: src/enc/backward_references_enc.c GetWindowSizeForHashChain
fn get_window_size_for_hash_chain(quality: i32, xsize: i32) -> i32 {
    let max_window_size = if quality > 75 {
        WINDOW_SIZE
    } else if quality > 50 {
        xsize << 8
    } else if quality > 25 {
        xsize << 6
    } else {
        xsize << 4
    };
    debug_assert!(xsize > 0);
    if max_window_size > WINDOW_SIZE {
        WINDOW_SIZE
    } else {
        max_window_size
    }
}

// upstream: src/enc/backward_references_enc.c MaxFindCopyLength
#[inline]
fn max_find_copy_length(len: i32) -> i32 {
    if len < MAX_LENGTH { len } else { MAX_LENGTH }
}

/// The progress value `percent_start + percent_range * done / total` that
/// `VP8LHashChainFill` computes in `uint32_t` for its second pass, where the
/// unsigned position makes the whole expression unsigned; it wraps as in C.
#[expect(
    clippy::cast_possible_wrap,
    reason = "the C conversion of the unsigned sum to the int parameter"
)]
fn unsigned_progress(percent_start: i32, percent_range: i32, done: u32, total: u32) -> i32 {
    percent_start
        .cast_unsigned()
        .wrapping_add(percent_range.cast_unsigned().wrapping_mul(done) / total) as i32
}

// upstream: src/enc/backward_references_enc.c VP8LHashChainFill
/// Pre-compute the best matches for `argb`. `pic` and `percent` are for
/// progress. The first pass keeps the hash chain in `offset_length`, read as
/// `int32_t`, as C does through its cast.
///
/// # Errors
///
/// Returns the error that `pic` records: an allocation failure or a user abort.
#[expect(
    clippy::too_many_arguments,
    reason = "the C signature of VP8LHashChainFill has nine parameters"
)]
#[expect(
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    reason = "the C conversions between the int and uint32_t positions"
)]
#[expect(
    clippy::too_many_lines,
    reason = "the C function VP8LHashChainFill is translated as one function"
)]
pub fn vp8l_hash_chain_fill(
    p: &mut VP8LHashChain,
    quality: i32,
    argb: &[u32],
    xsize: i32,
    ysize: i32,
    low_effort: bool,
    pic: &mut WebPPicture<'_>,
    mut percent_range: i32,
    percent: &mut i32,
) -> Result<(), WebPEncodingError> {
    let size = xsize * ysize;
    let iter_max = get_max_iters_for_quality(quality);
    let window_size = get_window_size_for_hash_chain(quality, xsize) as u32;
    let mut remaining_percent = percent_range;
    let mut percent_start = *percent;
    // Temporarily use the p->offset_length as a hash chain.
    debug_assert!(size > 0);
    debug_assert!(p.size != 0);
    debug_assert_ne!(p.offset_length.len(), 0);

    if size <= 2 {
        p.offset_length[to_index(size - 1)] = 0;
        p.offset_length[0] = 0;
        return Ok(());
    }

    let Some(mut hash_to_first_index) = webp_safe_malloc::<i32>(HASH_SIZE as u64, 4) else {
        return Err(webp_encoding_set_error(
            pic,
            WebPEncodingError::VP8_ENC_ERROR_OUT_OF_MEMORY,
        ));
    };

    percent_range = remaining_percent / 2;
    remaining_percent -= percent_range;

    // Set the int32_t array to -1.
    hash_to_first_index.resize(to_index(HASH_SIZE), -1);
    let chain = &mut p.offset_length;
    // Fill the chain linking pixels with the same hash.
    let mut argb_comp = argb[0] == argb[1];
    let mut pos: i32 = 0;
    while pos < size - 2 {
        let hash_code: u32;
        let argb_comp_next = argb[to_index(pos + 1)] == argb[to_index(pos + 2)];
        if argb_comp && argb_comp_next {
            // Consecutive pixels with the same color will share the same hash.
            // We therefore use a different hash: the color and its repetition
            // length.
            let mut tmp = [0_u32; 2];
            let mut len: u32 = 1;
            tmp[0] = argb[to_index(pos)];
            // Figure out how far the pixels are the same.
            // The last pixel has a different 64 bit hash, as its next pixel does
            // not have the same color, so we just need to get to the last pixel equal
            // to its follower.
            while pos + len as i32 + 2 < size
                && argb[to_index(pos) + len as usize + 2] == argb[to_index(pos)]
            {
                len += 1;
            }
            if len > MAX_LENGTH as u32 {
                // Skip the pixels that match for distance=1 and length>MAX_LENGTH
                // because they are linked to their predecessor and we automatically
                // check that in the main for loop below. Skipping means setting no
                // predecessor in the chain, hence -1.
                let skip = (len - MAX_LENGTH as u32) as usize;
                chain[to_index(pos)..to_index(pos) + skip].fill(u32::MAX);
                pos += (len - MAX_LENGTH as u32) as i32;
                len = MAX_LENGTH as u32;
            }
            // Process the rest of the hash chain.
            while len != 0 {
                tmp[1] = len;
                len -= 1;
                let code = get_pix_pair_hash64(&tmp);
                chain[to_index(pos)] = hash_to_first_index[code as usize].cast_unsigned();
                hash_to_first_index[code as usize] = pos;
                pos += 1;
            }
            argb_comp = false;
        } else {
            // Just move one pixel forward.
            hash_code = get_pix_pair_hash64(&argb[to_index(pos)..]);
            chain[to_index(pos)] = hash_to_first_index[hash_code as usize].cast_unsigned();
            hash_to_first_index[hash_code as usize] = pos;
            pos += 1;
            argb_comp = argb_comp_next;
        }

        let progress = percent_start + percent_range * pos / (size - 2);
        webp_report_progress(pic, progress, Some(&mut *percent))?;
    }
    // Process the penultimate pixel.
    chain[to_index(pos)] =
        hash_to_first_index[get_pix_pair_hash64(&argb[to_index(pos)..]) as usize].cast_unsigned();

    drop(hash_to_first_index);

    percent_start += percent_range;
    webp_report_progress(pic, percent_start, Some(&mut *percent))?;
    percent_range = remaining_percent;

    // Find the best match interval at each pixel, defined by an offset to the
    // pixel and a length. The right-most pixel cannot match anything to the right
    // (hence a best length of 0) and the left-most pixel nothing to the left
    // (hence an offset of 0).
    debug_assert!(size > 2);
    p.offset_length[to_index(size - 1)] = 0;
    p.offset_length[0] = 0;
    let mut base_position = (size - 2) as u32;
    while base_position > 0 {
        let max_len =
            max_find_copy_length((size - 1).cast_unsigned().wrapping_sub(base_position) as i32);
        let argb_start = base_position as usize;
        let mut iter = iter_max;
        let mut best_length: i32 = 0;
        let mut best_distance: u32 = 0;
        let min_pos: i32 = if base_position > window_size {
            (base_position - window_size) as i32
        } else {
            0
        };
        let length_max = if max_len < 256 { max_len } else { 256 };

        let mut pos: i32 = p.offset_length[argb_start].cast_signed();
        if !low_effort {
            let mut curr_length;
            // Heuristic: use the comparison with the above line as an initialization.
            if base_position >= xsize as u32 {
                curr_length = find_match_length(
                    &argb[argb_start - to_index(xsize)..],
                    &argb[argb_start..],
                    best_length,
                    max_len,
                );
                if curr_length > best_length {
                    best_length = curr_length;
                    best_distance = xsize as u32;
                }
                iter -= 1;
            }
            // Heuristic: compare to the previous pixel.
            curr_length = find_match_length(
                &argb[argb_start - 1..],
                &argb[argb_start..],
                best_length,
                max_len,
            );
            if curr_length > best_length {
                best_length = curr_length;
                best_distance = 1;
            }
            iter -= 1;
            // Skip the for loop if we already have the maximum.
            if best_length == MAX_LENGTH {
                pos = min_pos - 1;
            }
        }
        let mut best_argb = argb[argb_start + to_index(best_length)];

        while pos >= min_pos && {
            iter -= 1;
            iter != 0
        } {
            debug_assert!(base_position > pos as u32);

            if argb[to_index(pos + best_length)] == best_argb {
                let curr_length =
                    vp8l_vector_mismatch(&argb[to_index(pos)..], &argb[argb_start..], max_len);
                if best_length < curr_length {
                    best_length = curr_length;
                    best_distance = base_position.wrapping_sub(pos as u32);
                    best_argb = argb[argb_start + to_index(best_length)];
                    // Stop if we have reached a good enough length.
                    if best_length >= length_max {
                        break;
                    }
                }
            }
            pos = p.offset_length[to_index(pos)].cast_signed();
        }
        // We have the best match but in case the two intervals continue matching
        // to the left, we have the best matches for the left-extended pixels.
        let mut max_base_position = base_position;
        loop {
            debug_assert!(best_length <= MAX_LENGTH);
            debug_assert!(best_distance <= WINDOW_SIZE as u32);
            p.offset_length[base_position as usize] =
                (best_distance << MAX_LENGTH_BITS) | best_length as u32;
            base_position -= 1;
            // Stop if we don't have a match or if we are out of bounds.
            if best_distance == 0 || base_position == 0 {
                break;
            }
            // Stop if we cannot extend the matching intervals to the left.
            if base_position < best_distance
                || argb[(base_position - best_distance) as usize] != argb[base_position as usize]
            {
                break;
            }
            // Stop if we are matching at its limit because there could be a closer
            // matching interval with the same maximum length. Then again, if the
            // matching interval is as close as possible (best_distance == 1), we will
            // never find anything better so let's continue.
            if best_length == MAX_LENGTH
                && best_distance != 1
                && base_position.wrapping_add(MAX_LENGTH as u32) < max_base_position
            {
                break;
            }
            if best_length < MAX_LENGTH {
                best_length += 1;
                max_base_position = base_position;
            }
        }

        let total = (size - 2) as u32;
        let progress = unsigned_progress(
            percent_start,
            percent_range,
            total.wrapping_sub(base_position),
            total,
        );
        webp_report_progress(pic, progress, Some(&mut *percent))?;
    }

    webp_report_progress(pic, percent_start + percent_range, Some(percent))
}

// upstream: src/enc/backward_references_enc.c AddSingleLiteral
/// `hashers` is only used when `use_color_cache` is set, as in C.
#[inline]
fn add_single_literal(
    pixel: u32,
    use_color_cache: bool,
    hashers: &mut VP8LColorCache,
    refs: &mut VP8LBackwardRefs,
) {
    let v = if use_color_cache {
        let key = vp8l_color_cache_get_index(hashers, pixel).cast_unsigned();
        if vp8l_color_cache_lookup(hashers, key) == pixel {
            pix_or_copy_create_cache_idx(key.cast_signed())
        } else {
            vp8l_color_cache_set(hashers, key, pixel);
            pix_or_copy_create_literal(pixel)
        }
    } else {
        pix_or_copy_create_literal(pixel)
    };
    vp8l_backward_refs_cursor_add(refs, v);
}

/// The C conversion of a copy length, at most `MAX_LENGTH`, to `uint16_t`.
#[inline]
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "the C conversion of the int length to uint16_t"
)]
pub(crate) fn copy_len(len: i32) -> u16 {
    len as u16
}

// upstream: src/enc/backward_references_enc.c BackwardReferencesRle
fn backward_references_rle(
    xsize: i32,
    ysize: i32,
    argb: &[u32],
    cache_bits: i32,
    refs: &mut VP8LBackwardRefs,
) -> bool {
    let pix_count = xsize * ysize;
    let use_color_cache = cache_bits > 0;
    let mut hashers = VP8LColorCache::default();

    if use_color_cache && !vp8l_color_cache_init(&mut hashers, cache_bits) {
        return false;
    }
    vp8l_clear_backward_refs(refs);
    // Add first pixel as literal.
    add_single_literal(argb[0], use_color_cache, &mut hashers, refs);
    let mut i: i32 = 1;
    while i < pix_count {
        let max_len = max_find_copy_length(pix_count - i);
        let rle_len = find_match_length(&argb[to_index(i)..], &argb[to_index(i - 1)..], 0, max_len);
        let prev_row_len = if i < xsize {
            0
        } else {
            find_match_length(
                &argb[to_index(i)..],
                &argb[to_index(i - xsize)..],
                0,
                max_len,
            )
        };
        if rle_len >= prev_row_len && rle_len >= MIN_LENGTH {
            vp8l_backward_refs_cursor_add(refs, pix_or_copy_create_copy(1, copy_len(rle_len)));
            // We don't need to update the color cache here since it is always the
            // same pixel being copied, and that does not change the color cache
            // state.
            i += rle_len;
        } else if prev_row_len >= MIN_LENGTH {
            vp8l_backward_refs_cursor_add(
                refs,
                pix_or_copy_create_copy(xsize.cast_unsigned(), copy_len(prev_row_len)),
            );
            if use_color_cache {
                for k in 0..prev_row_len {
                    vp8l_color_cache_insert(&mut hashers, argb[to_index(i + k)]);
                }
            }
            i += prev_row_len;
        } else {
            add_single_literal(argb[to_index(i)], use_color_cache, &mut hashers, refs);
            i += 1;
        }
    }
    if use_color_cache {
        vp8l_color_cache_clear(&mut hashers);
    }
    !refs.error
}

// upstream: src/enc/backward_references_enc.c BackwardReferencesLz77
fn backward_references_lz77(
    xsize: i32,
    ysize: i32,
    argb: &[u32],
    cache_bits: i32,
    hash_chain: &VP8LHashChain,
    refs: &mut VP8LBackwardRefs,
) -> bool {
    let mut i_last_check: i32 = -1;
    let use_color_cache = cache_bits > 0;
    let pix_count = xsize * ysize;
    let mut hashers = VP8LColorCache::default();

    if use_color_cache {
        let cc_init = vp8l_color_cache_init(&mut hashers, cache_bits);
        if !cc_init {
            return false;
        }
    }
    vp8l_clear_backward_refs(refs);
    let mut i: i32 = 0;
    while i < pix_count {
        // Alternative#1: Code the pixels starting at 'i' using backward reference.
        let (offset, mut len) = vp8l_hash_chain_find_copy(hash_chain, i);
        if len >= MIN_LENGTH {
            let len_ini = len;
            let mut max_reach: i32 = 0;
            let j_max = if i + len_ini >= pix_count {
                pix_count - 1
            } else {
                i + len_ini
            };
            // Only start from what we have not checked already.
            i_last_check = if i > i_last_check { i } else { i_last_check };
            // We know the best match for the current pixel but we try to find the
            // best matches for the current pixel AND the next one combined.
            // The naive method would use the intervals:
            // [i,i+len) + [i+len, length of best match at i+len)
            // while we check if we can use:
            // [i,j) (where j<=i+len) + [j, length of best match at j)
            let mut j = i_last_check + 1;
            while j <= j_max {
                let len_j = vp8l_hash_chain_find_length(hash_chain, j);
                let reach = j + if len_j >= MIN_LENGTH { len_j } else { 1 }; // 1 for single literal.
                if reach > max_reach {
                    len = j - i;
                    max_reach = reach;
                    if max_reach >= pix_count {
                        break;
                    }
                }
                j += 1;
            }
        } else {
            len = 1;
        }
        // Go with literal or backward reference.
        debug_assert!(len > 0);
        if len == 1 {
            add_single_literal(argb[to_index(i)], use_color_cache, &mut hashers, refs);
        } else {
            vp8l_backward_refs_cursor_add(
                refs,
                pix_or_copy_create_copy(offset.cast_unsigned(), copy_len(len)),
            );
            if use_color_cache {
                for j in i..i + len {
                    vp8l_color_cache_insert(&mut hashers, argb[to_index(j)]);
                }
            }
        }
        i += len;
    }

    let ok = !refs.error;
    if use_color_cache {
        vp8l_color_cache_clear(&mut hashers);
    }
    ok
}

// upstream: src/enc/backward_references_enc.c WINDOW_OFFSETS_SIZE_MAX
/// Compute an LZ77 by forcing matches to happen within a given distance cost.
/// We therefore limit the algorithm to the lowest 32 values in the `PlaneCode`
/// definition.
const WINDOW_OFFSETS_SIZE_MAX: i32 = 32;

// upstream: src/enc/backward_references_enc.c BackwardReferencesLz77Box
#[expect(
    clippy::cast_sign_loss,
    reason = "the C conversions of the offset and length to the packed uint32_t"
)]
#[expect(
    clippy::too_many_lines,
    reason = "the C function BackwardReferencesLz77Box is translated as one function"
)]
fn backward_references_lz77_box(
    xsize: i32,
    ysize: i32,
    argb: &[u32],
    cache_bits: i32,
    hash_chain_best: &VP8LHashChain,
    hash_chain: &mut VP8LHashChain,
    refs: &mut VP8LBackwardRefs,
) -> bool {
    const OFFSETS: usize = WINDOW_OFFSETS_SIZE_MAX as usize;
    let pix_count = xsize * ysize;
    let mut window_offsets = [0_i32; OFFSETS];
    let mut window_offsets_new = [0_i32; OFFSETS];
    let mut window_offsets_size: usize = 0;
    let mut window_offsets_new_size: usize = 0;
    let Some(mut counts_ini) = webp_safe_malloc::<u16>(i64::from(xsize * ysize).cast_unsigned(), 2)
    else {
        return false;
    };
    counts_ini.resize(to_index(pix_count), 0);
    let mut best_offset_prev: i32 = -1;
    let mut best_length_prev: i32 = -1;

    // counts[i] counts how many times a pixel is repeated starting at position i.
    counts_ini[to_index(pix_count - 1)] = 1;
    let mut i = pix_count - 2;
    while i >= 0 {
        let at = to_index(i);
        if argb[at] == argb[at + 1] {
            // Max out the counts to MAX_LENGTH.
            let next = counts_ini[at + 1];
            counts_ini[at] = next + u16::from(i32::from(next) != MAX_LENGTH);
        } else {
            counts_ini[at] = 1;
        }
        i -= 1;
    }

    // Figure out the window offsets around a pixel. They are stored in a
    // spiraling order around the pixel as defined by VP8LDistanceToPlaneCode.
    {
        for y in 0..=6 {
            for x in -6..=6 {
                let offset = y * xsize + x;
                // Ignore offsets that bring us after the pixel.
                if offset <= 0 {
                    continue;
                }
                let plane_code = vp8l_distance_to_plane_code(xsize, offset) - 1;
                if plane_code >= WINDOW_OFFSETS_SIZE_MAX {
                    continue;
                }
                window_offsets[to_index(plane_code)] = offset;
            }
        }
        // For narrow images, not all plane codes are reached, so remove those.
        for i in 0..OFFSETS {
            if window_offsets[i] == 0 {
                continue;
            }
            window_offsets[window_offsets_size] = window_offsets[i];
            window_offsets_size += 1;
        }
        // Given a pixel P, find the offsets that reach pixels unreachable from P-1
        // with any of the offsets in window_offsets[].
        for i in 0..window_offsets_size {
            let mut is_reachable = false;
            let mut j = 0;
            while j < window_offsets_size && !is_reachable {
                is_reachable |= window_offsets[i] == window_offsets[j] + 1;
                j += 1;
            }
            if !is_reachable {
                window_offsets_new[window_offsets_new_size] = window_offsets[i];
                window_offsets_new_size += 1;
            }
        }
    }

    hash_chain.offset_length[0] = 0;
    for i in 1..pix_count {
        let mut best_length = vp8l_hash_chain_find_length(hash_chain_best, i);
        let mut best_offset: i32 = 0;
        let mut do_compute = true;

        if best_length >= MAX_LENGTH {
            // Do not recompute the best match if we already have a maximal one in the
            // window.
            best_offset = vp8l_hash_chain_find_offset(hash_chain_best, i);
            for &window_offset in &window_offsets[..window_offsets_size] {
                if best_offset == window_offset {
                    do_compute = false;
                    break;
                }
            }
        }
        if do_compute {
            // Figure out if we should use the offset/length from the previous pixel
            // as an initial guess and therefore only inspect the offsets in
            // window_offsets_new[].
            let use_prev = (best_length_prev > 1) && (best_length_prev < MAX_LENGTH);
            let num_ind = if use_prev {
                window_offsets_new_size
            } else {
                window_offsets_size
            };
            best_length = if use_prev { best_length_prev - 1 } else { 0 };
            best_offset = if use_prev { best_offset_prev } else { 0 };
            // Find the longest match in a window around the pixel.
            for ind in 0..num_ind {
                let mut curr_length: i32 = 0;
                let mut j = i;
                let mut j_offset = if use_prev {
                    i - window_offsets_new[ind]
                } else {
                    i - window_offsets[ind]
                };
                if j_offset < 0 || argb[to_index(j_offset)] != argb[to_index(i)] {
                    continue;
                }
                // The longest match is the sum of how many times each pixel is
                // repeated.
                loop {
                    let counts_j_offset = i32::from(counts_ini[to_index(j_offset)]);
                    let counts_j = i32::from(counts_ini[to_index(j)]);
                    if counts_j_offset != counts_j {
                        curr_length += if counts_j_offset < counts_j {
                            counts_j_offset
                        } else {
                            counts_j
                        };
                        break;
                    }
                    // The same color is repeated counts_pos times at j_offset and j.
                    curr_length += counts_j_offset;
                    j_offset += counts_j_offset;
                    j += counts_j_offset;
                    if !(curr_length <= MAX_LENGTH
                        && j < pix_count
                        && argb[to_index(j_offset)] == argb[to_index(j)])
                    {
                        break;
                    }
                }
                if best_length < curr_length {
                    best_offset = if use_prev {
                        window_offsets_new[ind]
                    } else {
                        window_offsets[ind]
                    };
                    if curr_length >= MAX_LENGTH {
                        best_length = MAX_LENGTH;
                        break;
                    }
                    best_length = curr_length;
                }
            }
        }

        debug_assert!(i + best_length <= pix_count);
        debug_assert!(best_length <= MAX_LENGTH);
        if best_length <= MIN_LENGTH {
            hash_chain.offset_length[to_index(i)] = 0;
            best_offset_prev = 0;
            best_length_prev = 0;
        } else {
            hash_chain.offset_length[to_index(i)] =
                ((best_offset as u32) << MAX_LENGTH_BITS) | best_length as u32;
            best_offset_prev = best_offset;
            best_length_prev = best_length;
        }
    }
    hash_chain.offset_length[0] = 0;
    drop(counts_ini);

    backward_references_lz77(xsize, ysize, argb, cache_bits, hash_chain, refs)
}

// -----------------------------------------------------------------------------

// upstream: src/enc/backward_references_enc.c BackwardReferences2DLocality
fn backward_references_2d_locality(xsize: i32, refs: &mut VP8LBackwardRefs) {
    let mut c = vp8l_refs_cursor_init(refs);
    while vp8l_refs_cursor_ok(&c) {
        let v = c.get_mut(refs);
        if pix_or_copy_is_copy(v) {
            let dist = v.argb_or_distance.cast_signed();
            let transformed_dist = vp8l_distance_to_plane_code(xsize, dist);
            v.argb_or_distance = transformed_dist.cast_unsigned();
        }
        vp8l_refs_cursor_next(&mut c, refs);
    }
}

// upstream: src/enc/backward_references_enc.c CalculateBestCacheSize
/// Evaluate optimal cache bits for the local color cache.
/// The input `*best_cache_bits` sets the maximum cache bits to use (passing 0
/// implies disabling the local color cache). The local color cache is also
/// disabled for the lower (<= 25) quality.
/// Returns 0 in case of memory error.
#[expect(
    clippy::many_single_char_names,
    reason = "the C names a, r, g, b of the channels"
)]
fn calculate_best_cache_size(
    argb: &[u32],
    quality: i32,
    refs: &VP8LBackwardRefs,
    best_cache_bits: &mut i32,
) -> bool {
    const SLOTS: usize = MAX_COLOR_CACHE_BITS as usize + 1;
    let cache_bits_max = if quality <= 25 { 0 } else { *best_cache_bits };
    let mut entropy_min = WEBP_UINT64_MAX;
    let mut hashers: [VP8LColorCache; SLOTS] = Default::default();
    let mut c = vp8l_refs_cursor_init(refs);
    let mut histos: [Option<Box<VP8LHistogram>>; SLOTS] = Default::default();
    let max = to_index(cache_bits_max);

    debug_assert!((0..=MAX_COLOR_CACHE_BITS).contains(&cache_bits_max));

    if cache_bits_max == 0 {
        *best_cache_bits = 0;
        // Local color cache is disabled.
        return true;
    }

    // Allocate data.
    for i in 0..=max {
        let bits = i32::try_from(i).expect("cache bits fit an int");
        let Some(mut histo) = vp8l_allocate_histogram(bits) else {
            return false;
        };
        vp8l_histogram_init(&mut histo, bits, /*init_arrays=*/ true);
        histos[i] = Some(histo);
        if i == 0 {
            continue;
        }
        if !vp8l_color_cache_init(&mut hashers[i], bits) {
            return false;
        }
    }
    let mut h: Vec<&mut VP8LHistogram> = histos
        .iter_mut()
        .take(max + 1)
        .map(|histo| &mut **histo.as_mut().expect("allocated above"))
        .collect();

    // Find the cache_bits giving the lowest entropy. The search is done in a
    // brute-force way as the function (entropy w.r.t cache_bits) can be
    // anything in practice.
    let mut argb_pos: usize = 0;
    while vp8l_refs_cursor_ok(&c) {
        let v = c.get(refs);
        if pix_or_copy_is_literal(v) {
            let pix = argb[argb_pos];
            argb_pos += 1;
            let a = ((pix >> 24) & 0xff) as usize;
            let r = ((pix >> 16) & 0xff) as usize;
            let g = ((pix >> 8) & 0xff) as usize;
            let b = (pix & 0xff) as usize;
            // The keys of the caches can be derived from the longest one.
            let mut key = vp8l_hash_pix(pix, 32 - cache_bits_max);
            // Do not use the color cache for cache_bits = 0.
            h[0].blue[b] += 1;
            h[0].literal[g] += 1;
            h[0].red[r] += 1;
            h[0].alpha[a] += 1;
            // Deal with cache_bits > 0.
            for i in (1..=max).rev() {
                if vp8l_color_cache_lookup(&hashers[i], key.cast_unsigned()) == pix {
                    h[i].literal[to_index(NUM_LITERAL_CODES + NUM_LENGTH_CODES + key)] += 1;
                } else {
                    vp8l_color_cache_set(&mut hashers[i], key.cast_unsigned(), pix);
                    h[i].blue[b] += 1;
                    h[i].literal[g] += 1;
                    h[i].red[r] += 1;
                    h[i].alpha[a] += 1;
                }
                key >>= 1;
            }
        } else {
            // We should compute the contribution of the (distance,length)
            // histograms but those are the same independently from the cache size.
            // As those constant contributions are in the end added to the other
            // histogram contributions, we can ignore them, except for the length
            // prefix that is part of the 'literal' histogram.
            let mut len = pix_or_copy_length(v).cast_signed();
            let mut argb_prev = argb[argb_pos] ^ 0xffff_ffff;
            let (code, _extra_bits, _extra_bits_value) = vp8l_prefix_encode(len);
            for histo in &mut h {
                histo.literal[to_index(NUM_LITERAL_CODES + code)] += 1;
            }
            // Update the color caches.
            loop {
                if argb[argb_pos] != argb_prev {
                    // Efficiency: insert only if the color changes.
                    let mut key = vp8l_hash_pix(argb[argb_pos], 32 - cache_bits_max);
                    for i in (1..=max).rev() {
                        hashers[i].colors[to_index(key)] = argb[argb_pos];
                        key >>= 1;
                    }
                    argb_prev = argb[argb_pos];
                }
                argb_pos += 1;
                len -= 1;
                if len == 0 {
                    break;
                }
            }
        }
        vp8l_refs_cursor_next(&mut c, refs);
    }

    for (i, histo) in (0..).zip(h.iter()) {
        let entropy = vp8l_histogram_estimate_bits(histo);
        if i == 0 || entropy < entropy_min {
            entropy_min = entropy;
            *best_cache_bits = i;
        }
    }
    for hasher in &mut hashers[1..=max] {
        vp8l_color_cache_clear(hasher);
    }
    true
}

// upstream: src/enc/backward_references_enc.c BackwardRefsWithLocalCache
/// Update (in-place) backward references for specified `cache_bits`.
fn backward_refs_with_local_cache(
    argb: &[u32],
    cache_bits: i32,
    refs: &mut VP8LBackwardRefs,
) -> bool {
    let mut pixel_index: usize = 0;
    let mut hashers = VP8LColorCache::default();
    let mut c = vp8l_refs_cursor_init(refs);
    if !vp8l_color_cache_init(&mut hashers, cache_bits) {
        return false;
    }

    while vp8l_refs_cursor_ok(&c) {
        let v = c.get_mut(refs);
        if pix_or_copy_is_literal(v) {
            let argb_literal = v.argb_or_distance;
            let ix = vp8l_color_cache_contains(&hashers, argb_literal);
            if ix >= 0 {
                // hashers contains argb_literal
                *v = pix_or_copy_create_cache_idx(ix);
            } else {
                vp8l_color_cache_insert(&mut hashers, argb_literal);
            }
            pixel_index += 1;
        } else {
            // refs was created without local cache, so it can not have cache indexes.
            debug_assert!(pix_or_copy_is_copy(v));
            for _ in 0..v.len {
                vp8l_color_cache_insert(&mut hashers, argb[pixel_index]);
                pixel_index += 1;
            }
        }
        vp8l_refs_cursor_next(&mut c, refs);
    }
    vp8l_color_cache_clear(&mut hashers);
    true
}

// upstream: src/enc/backward_references_enc.c GetBackwardReferencesLowEffort
/// Returns `false` where C returns `NULL`; on success the references are in
/// `refs_lz77`, the pointer that C returns.
fn get_backward_references_low_effort(
    width: i32,
    height: i32,
    argb: &[u32],
    cache_bits: &mut i32,
    hash_chain: &VP8LHashChain,
    refs_lz77: &mut VP8LBackwardRefs,
) -> bool {
    *cache_bits = 0;
    if !backward_references_lz77(width, height, argb, 0, hash_chain, refs_lz77) {
        return false;
    }
    backward_references_2d_locality(width, refs_lz77);
    true
}

// upstream: src/enc/backward_references_enc.c GetBackwardReferences
/// `refs` holds the three (`do_no_cache`) or two references of the C array.
#[expect(
    clippy::too_many_arguments,
    reason = "the C signature of GetBackwardReferences has ten parameters"
)]
#[expect(
    clippy::too_many_lines,
    reason = "the C function GetBackwardReferences is translated as one function"
)]
#[expect(clippy::similar_names, reason = "the C names refs and res")]
fn get_backward_references(
    width: i32,
    height: i32,
    argb: &[u32],
    quality: i32,
    mut lz77_types_to_try: u32,
    cache_bits_max: i32,
    do_no_cache: bool,
    hash_chain: &VP8LHashChain,
    refs: &mut [VP8LBackwardRefs],
    cache_bits_best: &mut i32,
) -> bool {
    // Index 0 is for a color cache, index 1 for no cache (if needed).
    let mut lz77_types_best: [u32; 2] = [0, 0];
    let mut bit_costs_best: [u64; 2] = [WEBP_UINT64_MAX, WEBP_UINT64_MAX];
    let mut hash_chain_box = VP8LHashChain::default();
    let tmp_index: usize = if do_no_cache { 2 } else { 1 };

    let Some(mut histo) = vp8l_allocate_histogram(MAX_COLOR_CACHE_BITS) else {
        return false;
    };

    let mut lz77_type: u32 = 1;
    while lz77_types_to_try != 0 {
        if (lz77_types_to_try & lz77_type) != 0 {
            let mut bit_cost: u64 = 0;
            let res = match VP8LLZ77Type(lz77_type) {
                VP8LLZ77Type::kLZ77RLE => {
                    backward_references_rle(width, height, argb, 0, &mut refs[tmp_index])
                }
                VP8LLZ77Type::kLZ77Standard => {
                    // Compute LZ77 with no cache (0 bits), as the ideal LZ77 with a color
                    // cache is not that different in practice.
                    backward_references_lz77(
                        width,
                        height,
                        argb,
                        0,
                        hash_chain,
                        &mut refs[tmp_index],
                    )
                }
                VP8LLZ77Type::kLZ77Box => {
                    if !vp8l_hash_chain_init(&mut hash_chain_box, width * height) {
                        return false;
                    }
                    backward_references_lz77_box(
                        width,
                        height,
                        argb,
                        0,
                        hash_chain,
                        &mut hash_chain_box,
                        &mut refs[tmp_index],
                    )
                }
                _ => {
                    debug_assert!(false, "an unknown LZ77 type");
                    false
                }
            };
            if !res {
                return false;
            }

            // Start with the no color cache case.
            for i in (0..=1_usize).rev() {
                let mut cache_bits = if i == 1 { 0 } else { cache_bits_max };

                if i == 1 && !do_no_cache {
                    continue;
                }

                if i == 0 {
                    // Try with a color cache.
                    if !calculate_best_cache_size(argb, quality, &refs[tmp_index], &mut cache_bits)
                    {
                        return false;
                    }
                    if cache_bits > 0
                        && !backward_refs_with_local_cache(argb, cache_bits, &mut refs[tmp_index])
                    {
                        return false;
                    }
                }

                if i == 0 && do_no_cache && cache_bits == 0 {
                    // No need to re-compute bit_cost as it was computed at i == 1.
                } else {
                    vp8l_histogram_create(&mut histo, &refs[tmp_index], cache_bits);
                    bit_cost = vp8l_histogram_estimate_bits(&histo);
                }

                if bit_cost < bit_costs_best[i] {
                    if i == 1 {
                        // Do not swap as the full cache analysis would have the wrong
                        // VP8LBackwardRefs to start with.
                        let (refs_tmp, refs_1) = split_pair(refs, tmp_index, 1);
                        if !backward_refs_clone(refs_tmp, refs_1) {
                            return false;
                        }
                    } else {
                        let (refs_tmp, refs_0) = split_two(refs, tmp_index, 0);
                        backward_refs_swap(refs_tmp, refs_0);
                    }
                    bit_costs_best[i] = bit_cost;
                    lz77_types_best[i] = lz77_type;
                    if i == 0 {
                        *cache_bits_best = cache_bits;
                    }
                }
            }
        }
        lz77_types_to_try &= !lz77_type;
        lz77_type <<= 1;
    }
    debug_assert!(lz77_types_best[0] > 0);
    debug_assert!(!do_no_cache || lz77_types_best[1] > 0);

    // Improve on simple LZ77 but only for high quality (TraceBackwards is
    // costly).
    for i in (0..=1_usize).rev() {
        if i == 1 && !do_no_cache {
            continue;
        }
        if (lz77_types_best[i] == VP8LLZ77Type::kLZ77Standard.0
            || lz77_types_best[i] == VP8LLZ77Type::kLZ77Box.0)
            && quality >= 25
        {
            let hash_chain_tmp = if lz77_types_best[i] == VP8LLZ77Type::kLZ77Standard.0 {
                hash_chain
            } else {
                &hash_chain_box
            };
            let cache_bits = if i == 1 { 0 } else { *cache_bits_best };
            let (refs_i, refs_tmp) = split_pair(refs, i, tmp_index);
            if !vp8l_backward_references_trace_backwards(
                width,
                height,
                argb,
                cache_bits,
                hash_chain_tmp,
                refs_i,
                refs_tmp,
            ) {
                return false;
            }
            vp8l_histogram_create(&mut histo, refs_tmp, cache_bits);
            let bit_cost_trace = vp8l_histogram_estimate_bits(&histo);
            if bit_cost_trace < bit_costs_best[i] {
                let (refs_tmp, refs_i) = split_two(refs, tmp_index, i);
                backward_refs_swap(refs_tmp, refs_i);
            }
        }

        backward_references_2d_locality(width, &mut refs[i]);

        if i == 1 && lz77_types_best[0] == lz77_types_best[1] && *cache_bits_best == 0 {
            // If the best cache size is 0 and we have the same best LZ77, just copy
            // the data over and stop here.
            let (refs_1, refs_0) = split_pair(refs, 1, 0);
            if !backward_refs_clone(refs_1, refs_0) {
                return false;
            }
            break;
        }
    }
    true
}

// upstream: src/enc/backward_references_enc.c VP8LGetBackwardReferences
/// Evaluates best possible backward references for specified quality.
/// The input `cache_bits` to `VP8LGetBackwardReferences` sets the maximum cache
/// bits to use (passing 0 implies disabling the local color cache).
/// The optimal cache bits is evaluated and set for the `*cache_bits_best`
/// parameter with the matching `refs_best`.
/// If `do_no_cache == 0`, refs is an array of 2 values and the best
/// `VP8LBackwardRefs` is put in the first element.
/// If `do_no_cache != 0`, refs is an array of 3 values and the best
/// `VP8LBackwardRefs` is put in the first element, the best value with no-cache in
/// the second element.
/// In both cases, the last element is used as temporary internally.
/// `pic` and `percent` are for progress.
///
/// # Errors
///
/// Returns the error that `pic` records: an allocation failure or a user abort.
#[expect(
    clippy::too_many_arguments,
    reason = "the C signature of VP8LGetBackwardReferences has fourteen parameters"
)]
pub fn vp8l_get_backward_references(
    width: i32,
    height: i32,
    argb: &[u32],
    quality: i32,
    low_effort: bool,
    lz77_types_to_try: u32,
    cache_bits_max: i32,
    do_no_cache: bool,
    hash_chain: &VP8LHashChain,
    refs: &mut [VP8LBackwardRefs],
    cache_bits_best: &mut i32,
    pic: &mut WebPPicture<'_>,
    percent_range: i32,
    percent: &mut i32,
) -> Result<(), WebPEncodingError> {
    if low_effort {
        *cache_bits_best = cache_bits_max;
        if !get_backward_references_low_effort(
            width,
            height,
            argb,
            cache_bits_best,
            hash_chain,
            &mut refs[0],
        ) {
            return Err(webp_encoding_set_error(
                pic,
                WebPEncodingError::VP8_ENC_ERROR_OUT_OF_MEMORY,
            ));
        }
        // Set it in first position. C swaps refs[0] with the pointer that
        // GetBackwardReferencesLowEffort returned, which is refs[0] itself, so the
        // swap changes nothing.
    } else if !get_backward_references(
        width,
        height,
        argb,
        quality,
        lz77_types_to_try,
        cache_bits_max,
        do_no_cache,
        hash_chain,
        refs,
        cache_bits_best,
    ) {
        return Err(webp_encoding_set_error(
            pic,
            WebPEncodingError::VP8_ENC_ERROR_OUT_OF_MEMORY,
        ));
    }

    let progress = *percent + percent_range;
    webp_report_progress(pic, progress, Some(percent))
}
