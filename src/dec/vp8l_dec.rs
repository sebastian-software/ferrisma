// libwebp v1.6.0, src/dec/vp8l_dec.c:
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
// main entry for the decoder
//
// Authors: Vikas Arora (vikaas.arora@gmail.com)
//          Jyrki Alakuijala (jyrki@google.com)

//! Translation of `src/dec/vp8l_dec.c` from libwebp v1.6.0.
//!
//! Work package: WP7 (VP8L decoder).
//!
//! The decoder takes its input, its `VP8Io`, its output buffer and, for alpha,
//! the alpha plane per call (ADR 0003, "Decoding context per call"). Pointers
//! into the decoder's pixel buffer and into the output are positions.
//!
//! # Deviations from upstream
//!
//! Changes of representation that the ADRs allow, or behavior that the Rust
//! side cannot reproduce exactly. Each records the reason.
//!
//! 1. **Decoding context per call (ADR 0003).** `VP8Io` keeps its scalar
//!    fields; its `int` flags are `bool`. The input of `io->data` is a
//!    `data: &[u8]` argument of `vp8l_decode_header`, `vp8l_decode_image`,
//!    `vp8l_decode_alpha_header` and `vp8l_decode_alpha_image_stream`, and of
//!    every internal function that reads bits; the image call must receive the
//!    same bytes at the same positions as the header call, as the
//!    `VP8LBitReader` keeps positions. `VP8LDecoder` stores no `io` and no
//!    `output`: `vp8l_decode_image` takes the `VP8Io`, the parameters and the
//!    output buffer; C stores `output` at the first call and reads it later,
//!    and the Rust caller passes the same buffer each time. `ALPHDecoder`
//!    stores no `output`: the VP8 decoder's alpha plane is an argument of
//!    `vp8l_decode_alpha_image_stream`, and `prev_line` is a position in it.
//!    `alph_dec->io.opaque` and `dec->io` are not stored; the alpha functions
//!    take `alph_dec`. `WebPDecParams` holds `options` as a borrow, `tmp_y`,
//!    `tmp_u` and `tmp_v` as positions in `memory`, the rescalers by value, and
//!    the emit functions as `Option` plain function pointers.
//! 2. **Internal pixel buffers.** C's `pixels` is `uint32_t*` for the 32-bit
//!    path and `uint8_t*` for the paletted alpha path; Rust has
//!    `pixels: Vec<u32>` and `pixels_8b: Vec<u8>`. The 32-bit buffer holds the
//!    pixels, the top-prediction row and `argb_cache` in one allocation, as C
//!    does and as the in-place transforms of `src/dsp/lossless.rs` require;
//!    `argb_cache` is an `Option<usize>` position, `None` where C has `NULL`.
//!    `VP8LClear` leaves it, as C leaves the stale pointer. The decode loop
//!    borrows the pixel buffer out of the decoder for its duration, and the
//!    row-processing function receives it.
//! 3. **Rescaler memory.** C carves the rescaler, its work rows and the scaled
//!    row from one block (`rescaler_memory`). The size test of `WebPSafeMalloc`
//!    is applied to that total with `WebPRescaler::C_SIZE` (ADR 0003) by
//!    reserving and dropping it; then the rescaler receives its work rows
//!    (`WebPRescaler` owns them, ADR 0003) and the decoder holds the scaled row
//!    as bytes (`rescaled_row`).
//! 4. **ARGB rows and the byte rescaler.** C casts the `uint32_t` rows to
//!    `uint8_t*` for `WebPRescalerImport` and the rescaler's byte row back to
//!    `uint32_t*` for `WebPMultARGBRow` and `VP8LConvertFromBGRA`. Rust
//!    converts the rows with `to_ne_bytes` and `from_ne_bytes`, one copy per
//!    row with the same bytes. The premultiplication before the import happens
//!    in place on the pixel buffer, as in C; the unpremultiplication after the
//!    export happens on the converted row, where C changes the rescaler's row
//!    in place, which the next export overwrites before anything reads it.
//!    `WebPExtractAlpha` reads a byte copy of the row.
//! 5. **`ProcessRowsFunc`** is a
//!    `&mut dyn FnMut(&mut VP8LDecoder, &mut [u32], i32)` closure that carries
//!    its context: the `VP8Io` and the output buffer for `ProcessRows`, and the
//!    alpha parts for `ExtractAlphaRows` (C reads both through `dec->io` and
//!    `io->opaque`). `None` is C's `NULL`.
//! 6. **Errors (ADR 0003).** `VP8LSetError` keeps its precedence and returns
//!    the status that the decoder holds afterwards. `vp8l_decode_header`,
//!    `vp8l_decode_image` and `vp8l_decode_alpha_image_stream` return
//!    `Result<(), VP8StatusCode>` with the recorded status; a suspended
//!    incremental decoding is `Ok` with `dec.status == VP8_STATUS_SUSPENDED`,
//!    as C returns 1. The internal functions keep their `int` results as `bool`
//!    or sizes, because several of them fail without recording an error (a
//!    repeated transform, the end of the stream in `ReadHuffmanCodes`, a
//!    mismatched mapping), so the status field is the only record, as in C.
//!    `vp8l_decode_alpha_header` returns `bool`: its decoder is dropped on
//!    failure, as C deletes it.
//! 7. **`ReadHuffmanCodesHelper`** takes the tables and the groups separately
//!    from the decoder, as C does; the decoder's own tables are moved out of
//!    `dec.hdr` for the call, because C passes `&dec->hdr.huffman_tables` next
//!    to `dec` and the helper reaches the decoder only for its bit reader and
//!    status. `htree_groups` is `Option<Vec<HTreeGroup>>`, `None` where C has
//!    `NULL`, and becomes `None` on failure. Tree pointers are
//!    `HuffmanTablesOffset` positions (`src/utils/huffman_utils.rs`).
//! 8. **`VP8LNew` and `VP8LDelete`.** `VP8LNew` returns a `Box`; its fixed-size
//!    allocation cannot fail the size test, and a host refusal aborts.
//!    `VP8LDelete` only clears and frees, so the owner drops the box instead,
//!    as for `WebPSafeFree` (`src/utils/utils.rs`).
//! 9. **Bit reader copies.** `SaveState` and `RestoreState` copy the bit reader
//!    field by field, as the C struct assignment does; `VP8LBitReader` has no
//!    `Clone`.
//! 10. **Output rows.** Row `y` of a plane starts at `offset + y * stride`,
//!     computed per row with the signed stride, so a flipped buffer never forms
//!     the position past its last row that the C pointer arithmetic forms.
//!     `ConvertToYUVA` borrows the U and V rows together, from one region
//!     (internal storage) or two (external storage).
//! 11. **Pattern copies.** `CopySmallPattern8b` and `CopySmallPattern32b` test
//!     the address of the destination for alignment, as C does, and store the
//!     pattern through its native bytes; the 64-bit swap is `rotate_left(32)`.
//!     The `goto Copy` of `CopyBlock8b` is a fallthrough when no pattern
//!     applies.
//! 12. **`DecodeImageData`'s jumps.** The `goto AdvanceByOne` and `goto Error`
//!     of the decode loop are a labeled block that yields whether to advance,
//!     and a flag; the order of every read, write, check-point and row callback
//!     is C's.
//! 13. **Lint expectations.** `VP8Io` keeps its flags
//!     (`struct_excessive_bools`); the C names `dst` and `dist`,
//!     `huffman_xsize` and `huffman_ysize`, `transform_xsize` and
//!     `transform_ysize` stay (`similar_names`); the pointer-alignment tests
//!     keep their masks (`verbose_bit_mask`); the two `None` branches of
//!     `VP8LGetInfo` stay apart (`if_same_then_else`); the debug assertion of
//!     `DecodeImageData` keeps its C form (`nonminimal_bool`);
//!     `ReadHuffmanCodesHelper` and `DecodeImageData` keep their length
//!     (`too_many_lines`), and functions with the C parameters plus the
//!     per-call context keep them (`too_many_arguments`). Each names the C
//!     construct (ADR 0005).
//!
//! # Excluded undefined behavior
//!
//! Inputs that are undefined behavior (or assertion failures) in C and are
//! therefore excluded from the oracle comparison (PORTING rule 8):
//!
//! 1. `VP8LDecodeImage` after it failed or completed: C asserts that the
//!    Huffman tables exist and that `last_row` grows. The tests stop after a
//!    failure or when the status is no longer `VP8_STATUS_SUSPENDED`, as the
//!    incremental decoder does.
//! 2. `VP8LDecodeAlphaImageStream` with `last_row` above the height or the crop
//!    bottom (C asserts), or without a decoded alpha header.
//! 3. Output buffers smaller than the decoded area, or whose planes overlap, or
//!    a `WebPDecBufferPlanes` variant that does not match the colorspace: C
//!    writes outside the buffer or reads the other union member; Rust panics.
//! 4. `ReadHuffmanCodesHelper` with a mapping that leaves groups unfilled or
//!    maps outside them, or with color cache bits outside 0 to 11: C reads
//!    uninitialized groups or indexes `kTableSize` outside its bounds.
//! 5. `WebPISetIOHooks` combined with a lossless image: C reads the user data
//!    as `WebPDecParams` in `VP8LDecodeImage`. The Rust decoder takes the
//!    parameters per call, so this case is defined there; it belongs to the
//!    incremental decoder (`src/dec/idec_dec.rs`).
//!
//! # Checks not applicable
//!
//! These checks do not apply, or cannot be made, with the reason:
//!
//! 1. **No `put` hook in VP8L.** `VP8LDecodeImage` writes the rows into the
//!    output buffer itself and calls none of the `VP8Io` hooks; the ARGB rows
//!    that a `put` callback would receive are therefore compared in the output
//!    buffer, after the header and after every step, including the bytes
//!    outside the decoded area. The hooks are called by `frame_dec.c`
//!    (`src/dec/frame_dec.rs`), and `tests/wp6_decode.rs` tests them.
//! 2. **`Init` functions.** `VP8LDspInit` (in `VP8LNew`),
//!    `WebPInitAlphaProcessing` and `WebPInitConvertARGBToYUV` (in
//!    `VP8LDecodeImage` and `VP8LDecodeAlphaImageStream`) only assign portable
//!    pointers, and ADR 0003 omits them with an `// upstream:` comment. The
//!    alpha probe calls `VP8FiltersInit`, as `ALPHInit` does before the VP8L
//!    alpha calls.
//! 3. **`WEBP_REDUCE_SIZE`** is unset in the reference configuration, so the
//!    rescaling branches are translated and its `else` branches (which reject
//!    scaling with `VP8_STATUS_INVALID_PARAM`) are not compiled.
//! 4. **Host refusals of an allocation.** The size test of `WebPSafeMalloc` is
//!    applied with the C sizes, but a refusal by the host of an accepted size
//!    cannot be provoked here; the Rust code returns `VP8_STATUS_OUT_OF_MEMORY`
//!    where C does. `VP8LNew` allocates a fixed-size structure, whose size test
//!    always passes (`# Deviations from upstream`, item 8).
//! 5. **Big-endian hosts.** `Rotate8b`, the two-byte pattern of `CopyBlock8b`,
//!    the alpha byte of `ConvertToYUVA` and the native byte order of the casts
//!    keep both branches; the host is little-endian, so the tests compare the
//!    little-endian branches only.
//! 6. **32-bit ARM and MIPS branches of `CopyBlock8b`** (`__arm__`,
//!    `WEBP_USE_MIPS_DSP_R2`) compute the same pattern values as the
//!    multiplication; neither target is built.
//! 7. **`NULL` arguments.** `VP8LDecodeHeader(NULL, ...)`,
//!    `VP8LDecodeHeader(dec, NULL)` (which records `VP8_STATUS_INVALID_PARAM`),
//!    `VP8LDecodeImage(NULL)`, `VP8LGetInfo(NULL, ...)` and its `NULL`
//!    out-parameters exist only at the C ABI (PORTING rule 8); these headers
//!    are not public.
//! 8. **Uninitialized C memory.** `VP8LHtreeGroupsNew` does not initialize the
//!    groups, and C sets `literal_arb` only for a trivial literal and the
//!    packed table only when it is used; the probes print them only then. The
//!    pixel and transform buffers come from `WebPSafeMalloc` and are written
//!    before they are read; Rust zero-initializes them (ADR 0003).
//! 9. **Threads.** The VP8L decoder starts no worker.
//! 10. **Floating point.** No function translated here computes in floating
//!     point.
//! 11. **Type sizes.** This module allocates no array of a type of its own; the
//!     C sizes it uses are those of `HuffmanCode`, `HTreeGroup` and
//!     `WebPRescaler`, which their modules compare with the C `sizeof`, and of
//!     `int` and `uint32_t`.

use crate::dec::alphai_dec::ALPHDecoder;
use crate::dec::vp8_dec::VP8Io;
use crate::dec::vp8li_dec::{VP8LDecodeState, VP8LDecoder, VP8LMetadata, VP8LTransform};
use crate::dec::webp_dec::webp_io_init_from_options;
use crate::dec::webpi_dec::WebPDecParams;
use crate::dsp::alpha_processing::{
    webp_extract_alpha, webp_extract_green, webp_mult_argb_row, webp_mult_argb_rows,
};
use crate::dsp::dsp::WEBP_FILTER_TYPE;
use crate::dsp::filters::webp_unfilters;
use crate::dsp::lossless::{
    vp8l_color_index_inverse_transform_alpha, vp8l_convert_from_bgra, vp8l_inverse_transform,
};
use crate::dsp::lossless_common::vp8l_sub_sample_size;
use crate::dsp::rescaler::webp_rescaler_export_row;
use crate::dsp::yuv::{webp_convert_argb_to_uv, webp_convert_argb_to_y};
use crate::utils::bit_reader_utils::{
    VP8LBitReader, vp8l_fill_bit_window, vp8l_init_bit_reader, vp8l_is_end_of_stream,
    vp8l_prefetch_bits, vp8l_read_bits, vp8l_set_bit_pos,
};
use crate::utils::color_cache_utils::{
    vp8l_color_cache_clear, vp8l_color_cache_copy, vp8l_color_cache_init, vp8l_color_cache_insert,
    vp8l_color_cache_lookup,
};
use crate::utils::huffman_utils::{
    HTreeGroup, HUFFMAN_PACKED_BITS, HUFFMAN_PACKED_TABLE_SIZE, HUFFMAN_TABLE_BITS,
    HUFFMAN_TABLE_MASK, HuffmanCode, HuffmanCode32, HuffmanTables, HuffmanTablesOffset,
    LENGTHS_TABLE_BITS, LENGTHS_TABLE_MASK, vp8l_build_huffman_table, vp8l_htree_groups_new,
    vp8l_huffman_tables_allocate, vp8l_huffman_tables_deallocate,
};
use crate::utils::rescaler_utils::{
    WebPRescaler, rescaler_t, webp_rescale_needed_lines, webp_rescaler_has_pending_output,
    webp_rescaler_import, webp_rescaler_init,
};
use crate::utils::utils::{webp_safe_calloc, webp_safe_malloc};
use crate::webp::decode::{
    VP8StatusCode, WEBP_CSP_MODE, WebPDecBuffer, WebPDecBufferPlanes, webp_is_rgb_mode,
};
use crate::webp::format_constants::{
    DEFAULT_CODE_LENGTH, HUFFMAN_CODES_PER_META_CODE, MAX_CACHE_BITS, MIN_HUFFMAN_BITS,
    MIN_TRANSFORM_BITS, NUM_DISTANCE_CODES, NUM_HUFFMAN_BITS, NUM_LENGTH_CODES, NUM_LITERAL_CODES,
    NUM_TRANSFORM_BITS, NUM_TRANSFORMS, VP8L_FRAME_HEADER_SIZE, VP8L_IMAGE_SIZE_BITS,
    VP8L_MAGIC_BYTE, VP8L_VERSION_BITS, VP8LImageTransformType,
};
use crate::webp::types::PlaneStorage;

// upstream: src/dec/vp8l_dec.c NUM_ARGB_CACHE_ROWS
const NUM_ARGB_CACHE_ROWS: i32 = 16;

// upstream: src/dec/vp8l_dec.c kCodeLengthLiterals
const CODE_LENGTH_LITERALS: i32 = 16;
// upstream: src/dec/vp8l_dec.c kCodeLengthRepeatCode
const CODE_LENGTH_REPEAT_CODE: i32 = 16;
// upstream: src/dec/vp8l_dec.c kCodeLengthExtraBits
const CODE_LENGTH_EXTRA_BITS: [u8; 3] = [2, 3, 7];
// upstream: src/dec/vp8l_dec.c kCodeLengthRepeatOffsets
const CODE_LENGTH_REPEAT_OFFSETS: [u8; 3] = [3, 3, 11];

// upstream: src/dec/vp8l_dec.c HuffIndex
/// Five Huffman codes are used at each meta code:
///  1. green + length prefix codes + color cache codes,
///  2. alpha,
///  3. red,
///  4. blue, and,
///  5. distance prefix codes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct HuffIndex(u32);

impl HuffIndex {
    // upstream: src/dec/vp8l_dec.c GREEN
    const GREEN: Self = Self(0);
    // upstream: src/dec/vp8l_dec.c RED
    const RED: Self = Self(1);
    // upstream: src/dec/vp8l_dec.c BLUE
    const BLUE: Self = Self(2);
    // upstream: src/dec/vp8l_dec.c ALPHA
    const ALPHA: Self = Self(3);
    // upstream: src/dec/vp8l_dec.c DIST
    const DIST: Self = Self(4);

    /// The enumerator as an array index.
    const fn index(self) -> usize {
        self.0 as usize
    }
}

// upstream: src/dec/vp8l_dec.c kAlphabetSize
#[expect(
    clippy::cast_possible_truncation,
    reason = "the C initializers convert the int constants to uint16_t"
)]
const ALPHABET_SIZE: [u16; HUFFMAN_CODES_PER_META_CODE as usize] = [
    (NUM_LITERAL_CODES + NUM_LENGTH_CODES) as u16,
    NUM_LITERAL_CODES as u16,
    NUM_LITERAL_CODES as u16,
    NUM_LITERAL_CODES as u16,
    NUM_DISTANCE_CODES as u16,
];

// upstream: src/dec/vp8l_dec.c kLiteralMap
const LITERAL_MAP: [u8; HUFFMAN_CODES_PER_META_CODE as usize] = [0, 1, 1, 1, 0];

// upstream: src/dec/vp8l_dec.c NUM_CODE_LENGTH_CODES
const NUM_CODE_LENGTH_CODES: usize = 19;
// upstream: src/dec/vp8l_dec.c kCodeLengthCodeOrder
const CODE_LENGTH_CODE_ORDER: [u8; NUM_CODE_LENGTH_CODES] = [
    17, 18, 0, 1, 2, 3, 4, 5, 16, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
];

// upstream: src/dec/vp8l_dec.c CODE_TO_PLANE_CODES
const CODE_TO_PLANE_CODES: i32 = 120;
// upstream: src/dec/vp8l_dec.c kCodeToPlane
const CODE_TO_PLANE: [u8; CODE_TO_PLANE_CODES as usize] = [
    0x18, 0x07, 0x17, 0x19, 0x28, 0x06, 0x27, 0x29, 0x16, 0x1a, //
    0x26, 0x2a, 0x38, 0x05, 0x37, 0x39, 0x15, 0x1b, 0x36, 0x3a, //
    0x25, 0x2b, 0x48, 0x04, 0x47, 0x49, 0x14, 0x1c, 0x35, 0x3b, //
    0x46, 0x4a, 0x24, 0x2c, 0x58, 0x45, 0x4b, 0x34, 0x3c, 0x03, //
    0x57, 0x59, 0x13, 0x1d, 0x56, 0x5a, 0x23, 0x2d, 0x44, 0x4c, //
    0x55, 0x5b, 0x33, 0x3d, 0x68, 0x02, 0x67, 0x69, 0x12, 0x1e, //
    0x66, 0x6a, 0x22, 0x2e, 0x54, 0x5c, 0x43, 0x4d, 0x65, 0x6b, //
    0x32, 0x3e, 0x78, 0x01, 0x77, 0x79, 0x53, 0x5d, 0x11, 0x1f, //
    0x64, 0x6c, 0x42, 0x4e, 0x76, 0x7a, 0x21, 0x2f, 0x75, 0x7b, //
    0x31, 0x3f, 0x63, 0x6d, 0x52, 0x5e, 0x00, 0x74, 0x7c, 0x41, //
    0x4f, 0x10, 0x20, 0x62, 0x6e, 0x30, 0x73, 0x7d, 0x51, 0x5f, //
    0x40, 0x72, 0x7e, 0x61, 0x6f, 0x50, 0x71, 0x7f, 0x60, 0x70, //
];

// upstream: src/dec/vp8l_dec.c FIXED_TABLE_SIZE
/// Memory needed for lookup tables of one Huffman tree group. Red, blue, alpha
/// and distance alphabets are constant (256 for red, blue and alpha, 40 for
/// distance) and lookup table sizes for them in worst case are 630 and 410
/// respectively. Size of green alphabet depends on color cache size and is equal
/// to 256 (green component values) + 24 (length prefix values) +
/// `color_cache_size` (between 0 and 2048).
/// All values computed for 8-bit first level lookup with Mark Adler's tool:
/// <https://github.com/madler/zlib/blob/v1.2.5/examples/enough.c>
const FIXED_TABLE_SIZE: u16 = 630 * 3 + 410;
// upstream: src/dec/vp8l_dec.c kTableSize
const TABLE_SIZE: [u16; 12] = [
    FIXED_TABLE_SIZE + 654,
    FIXED_TABLE_SIZE + 656,
    FIXED_TABLE_SIZE + 658,
    FIXED_TABLE_SIZE + 662,
    FIXED_TABLE_SIZE + 670,
    FIXED_TABLE_SIZE + 686,
    FIXED_TABLE_SIZE + 718,
    FIXED_TABLE_SIZE + 782,
    FIXED_TABLE_SIZE + 912,
    FIXED_TABLE_SIZE + 1168,
    FIXED_TABLE_SIZE + 1680,
    FIXED_TABLE_SIZE + 2704,
];

/// A C `int` that is a count, size or position and is not negative, as a
/// `usize`.
fn index(value: i32) -> usize {
    usize::try_from(value).expect("a position or size is not negative")
}

/// `VP8LReadBits` as the C `int` that its callers store. The value has at most
/// `VP8L_MAX_NUM_BIT_READ` (24) bits, so the C conversion of the `uint32_t`
/// result to `int` is exact.
fn read_bits(br: &mut VP8LBitReader, data: &[u8], n_bits: i32) -> i32 {
    i32::try_from(vp8l_read_bits(br, data, n_bits)).expect("at most 24 bits are read")
}

/// The entry `index` of the table at `table` (C: `table[index]`).
fn huffman_entry(tables: &HuffmanTables, table: HuffmanTablesOffset, index: usize) -> HuffmanCode {
    tables.segment(table.segment).start[table.offset + index]
}

/// The position of row `y` of a plane that starts at `offset` with `stride`,
/// as C computes `base + y * stride` with a signed stride.
///
/// # Panics
///
/// Panics when the row starts before the buffer, where the C pointer is
/// outside it.
fn row_position(offset: usize, y: i32, stride: i32) -> usize {
    let delta = isize::try_from(i64::from(y) * i64::from(stride)).expect("the row offset fits");
    offset
        .checked_add_signed(delta)
        .expect("the row starts inside the buffer")
}

/// A copy of a bit reader, field by field, as the C struct assignment makes
/// it.
fn copy_bit_reader(br: &VP8LBitReader) -> VP8LBitReader {
    VP8LBitReader {
        val: br.val,
        len: br.len,
        pos: br.pos,
        bit_pos: br.bit_pos,
        eos: br.eos,
    }
}

// upstream: src/dec/vp8l_dec.c VP8LSetError
/// Records `error` unless an older error is recorded: the oldest error reported
/// takes precedence over the new one. Returns the status that `dec` holds
/// afterwards (ADR 0003); C returns 0.
fn vp8l_set_error(dec: &mut VP8LDecoder, error: VP8StatusCode) -> VP8StatusCode {
    // The oldest error reported takes precedence over the new one.
    if dec.status == VP8StatusCode::VP8_STATUS_OK
        || dec.status == VP8StatusCode::VP8_STATUS_SUSPENDED
    {
        dec.status = error;
    }
    dec.status
}

//------------------------------------------------------------------------------

// upstream: src/dec/vp8l_dec.c VP8LCheckSignature
/// Returns true if the next byte(s) in data is a VP8L signature. The C size is
/// `data.len()`.
#[must_use]
pub fn vp8l_check_signature(data: &[u8]) -> bool {
    data.len() >= index(VP8L_FRAME_HEADER_SIZE)
        && i32::from(data[0]) == VP8L_MAGIC_BYTE
        && (data[4] >> 5) == 0 // version
}

// upstream: src/dec/vp8l_dec.c ReadImageInfo
/// Reads the image header: the signature byte, the dimensions, the alpha flag
/// and the version. Like C, it writes the three out-parameters before the
/// version check.
fn read_image_info(
    br: &mut VP8LBitReader,
    data: &[u8],
    width: &mut i32,
    height: &mut i32,
    has_alpha: &mut i32,
) -> bool {
    if read_bits(br, data, 8) != VP8L_MAGIC_BYTE {
        return false;
    }
    *width = read_bits(br, data, VP8L_IMAGE_SIZE_BITS) + 1;
    *height = read_bits(br, data, VP8L_IMAGE_SIZE_BITS) + 1;
    *has_alpha = read_bits(br, data, 1);
    if read_bits(br, data, VP8L_VERSION_BITS) != 0 {
        return false;
    }
    !br.eos
}

// upstream: src/dec/vp8l_dec.c VP8LGetInfo
/// Validates the VP8L data-header and retrieves basic header information:
/// `(width, height, has_alpha)`. Returns `None` in case of formatting error,
/// where C returns 0 and leaves its out-parameters alone. The C `data_size` is
/// `data.len()`.
#[must_use]
#[expect(
    clippy::if_same_then_else,
    reason = "the C branches for missing data and a bad signature"
)]
pub fn vp8l_get_info(data: &[u8]) -> Option<(i32, i32, bool)> {
    if data.len() < index(VP8L_FRAME_HEADER_SIZE) {
        None // not enough data
    } else if !vp8l_check_signature(data) {
        None // bad signature
    } else {
        let mut w = 0;
        let mut h = 0;
        let mut a = 0;
        let mut br = VP8LBitReader::default();
        vp8l_init_bit_reader(&mut br, data);
        if !read_image_info(&mut br, data, &mut w, &mut h, &mut a) {
            return None;
        }
        Some((w, h, a != 0))
    }
}

//------------------------------------------------------------------------------

// upstream: src/dec/vp8l_dec.c GetCopyDistance
/// The distance of a backward reference from its prefix symbol and extra bits.
fn get_copy_distance(distance_symbol: i32, br: &mut VP8LBitReader, data: &[u8]) -> i32 {
    if distance_symbol < 4 {
        return distance_symbol + 1;
    }
    let extra_bits = (distance_symbol - 2) >> 1;
    let offset = (2 + (distance_symbol & 1)) << extra_bits;
    offset + read_bits(br, data, extra_bits) + 1
}

// upstream: src/dec/vp8l_dec.c GetCopyLength
/// The length of a backward reference. Length and distance prefixes are encoded
/// the same way.
fn get_copy_length(length_symbol: i32, br: &mut VP8LBitReader, data: &[u8]) -> i32 {
    // Length and distance prefixes are encoded the same way.
    get_copy_distance(length_symbol, br, data)
}

// upstream: src/dec/vp8l_dec.c PlaneCodeToDistance
/// Maps a distance code to a distance in pixels.
fn plane_code_to_distance(xsize: i32, plane_code: i32) -> i32 {
    if plane_code > CODE_TO_PLANE_CODES {
        plane_code - CODE_TO_PLANE_CODES
    } else {
        let dist_code = i32::from(CODE_TO_PLANE[index(plane_code - 1)]);
        let yoffset = dist_code >> 4;
        let xoffset = 8 - (dist_code & 0xf);
        let dist = yoffset * xsize + xoffset;
        if dist >= 1 { dist } else { 1 } // dist<1 can happen if xsize is very small
    }
}

//------------------------------------------------------------------------------

// upstream: src/dec/vp8l_dec.c ReadSymbol
/// Decodes the next Huffman code from bit-stream with the tree at `table`.
/// `VP8LFillBitWindow(br)` needs to be called at minimum every second call to
/// `ReadSymbol`, in order to pre-fetch enough bits.
fn read_symbol(table: HuffmanTablesOffset, tables: &HuffmanTables, br: &mut VP8LBitReader) -> i32 {
    let codes = &tables.segment(table.segment).start;
    let mut table = table.offset;
    let mut val = vp8l_prefetch_bits(br);
    table += (val & HUFFMAN_TABLE_MASK.cast_unsigned()) as usize;
    let nbits = i32::from(codes[table].bits) - HUFFMAN_TABLE_BITS;
    if nbits > 0 {
        vp8l_set_bit_pos(br, br.bit_pos + HUFFMAN_TABLE_BITS);
        val = vp8l_prefetch_bits(br);
        table += usize::from(codes[table].value);
        table += (val & ((1_u32 << nbits) - 1)) as usize;
    }
    vp8l_set_bit_pos(br, br.bit_pos + i32::from(codes[table].bits));
    i32::from(codes[table].value)
}

// upstream: src/dec/vp8l_dec.c BITS_SPECIAL_MARKER
/// Something large enough (and a bit-mask).
const BITS_SPECIAL_MARKER: i32 = 0x100;
// upstream: src/dec/vp8l_dec.c PACKED_NON_LITERAL_CODE
/// Must be < `NUM_LITERAL_CODES`.
const PACKED_NON_LITERAL_CODE: i32 = 0;

// upstream: src/dec/vp8l_dec.c ReadPackedSymbols
/// Reads packed symbol depending on GREEN channel.
fn read_packed_symbols(group: &HTreeGroup, br: &mut VP8LBitReader, dst: &mut u32) -> i32 {
    let val = vp8l_prefetch_bits(br) & (HUFFMAN_PACKED_TABLE_SIZE - 1);
    let code = group.packed_table[val as usize];
    debug_assert!(group.use_packed_table);
    if code.bits < BITS_SPECIAL_MARKER {
        vp8l_set_bit_pos(br, br.bit_pos + code.bits);
        *dst = code.value;
        PACKED_NON_LITERAL_CODE
    } else {
        vp8l_set_bit_pos(br, br.bit_pos + code.bits - BITS_SPECIAL_MARKER);
        debug_assert!(code.value >= NUM_LITERAL_CODES.cast_unsigned());
        i32::try_from(code.value).expect("a non-literal symbol fits an int")
    }
}

// upstream: src/dec/vp8l_dec.c AccumulateHCode
/// Adds the bits and the shifted value of `hcode` to `huff`; returns the bits.
fn accumulate_h_code(hcode: HuffmanCode, shift: i32, huff: &mut HuffmanCode32) -> i32 {
    huff.bits += i32::from(hcode.bits);
    huff.value |= u32::from(hcode.value) << shift;
    debug_assert!(huff.bits <= HUFFMAN_TABLE_BITS);
    i32::from(hcode.bits)
}

// upstream: src/dec/vp8l_dec.c BuildPackedTable
/// Fills the packed table of `htree_group` from its trees in `tables`.
fn build_packed_table(htree_group: &mut HTreeGroup, tables: &HuffmanTables) {
    let htrees = htree_group.htrees;
    for code in 0..HUFFMAN_PACKED_TABLE_SIZE {
        let mut bits = code;
        let huff = &mut htree_group.packed_table[bits as usize];
        let hcode = huffman_entry(tables, htrees[HuffIndex::GREEN.index()], bits as usize);
        if i32::from(hcode.value) >= NUM_LITERAL_CODES {
            huff.bits = i32::from(hcode.bits) + BITS_SPECIAL_MARKER;
            huff.value = u32::from(hcode.value);
        } else {
            huff.bits = 0;
            huff.value = 0;
            bits >>= accumulate_h_code(hcode, 8, huff);
            bits >>= accumulate_h_code(
                huffman_entry(tables, htrees[HuffIndex::RED.index()], bits as usize),
                16,
                huff,
            );
            bits >>= accumulate_h_code(
                huffman_entry(tables, htrees[HuffIndex::BLUE.index()], bits as usize),
                0,
                huff,
            );
            bits >>= accumulate_h_code(
                huffman_entry(tables, htrees[HuffIndex::ALPHA.index()], bits as usize),
                24,
                huff,
            );
            let _ = bits;
        }
    }
}

// upstream: src/dec/vp8l_dec.c ReadHuffmanCodeLengths
/// Reads the code lengths of a Huffman code that are themselves coded with the
/// code `code_length_code_lengths`. Records a bitstream error on failure, also
/// when the lookup table cannot be allocated, as C does.
fn read_huffman_code_lengths(
    dec: &mut VP8LDecoder,
    data: &[u8],
    code_length_code_lengths: &[i32; NUM_CODE_LENGTH_CODES],
    num_symbols: i32,
    code_lengths: &mut [i32],
) -> bool {
    let mut ok = false;
    let mut prev_code_len = DEFAULT_CODE_LENGTH;
    let mut tables = HuffmanTables::default();

    'end: {
        if !vp8l_huffman_tables_allocate(1 << LENGTHS_TABLE_BITS, &mut tables)
            || vp8l_build_huffman_table(
                Some(&mut tables),
                LENGTHS_TABLE_BITS,
                code_length_code_lengths,
            ) == 0
        {
            break 'end;
        }

        let br = &mut dec.br;
        let mut max_symbol = if read_bits(br, data, 1) != 0 {
            // use length
            let length_nbits = 2 + 2 * read_bits(br, data, 3);
            let max_symbol = 2 + read_bits(br, data, length_nbits);
            if max_symbol > num_symbols {
                break 'end;
            }
            max_symbol
        } else {
            num_symbols
        };

        let mut symbol = 0;
        while symbol < num_symbols {
            let decrement = max_symbol;
            max_symbol -= 1;
            if decrement == 0 {
                break;
            }
            vp8l_fill_bit_window(br, data);
            let segment = tables.segment(tables.curr_segment);
            let p = segment.start
                [(vp8l_prefetch_bits(br) & LENGTHS_TABLE_MASK.cast_unsigned()) as usize];
            vp8l_set_bit_pos(br, br.bit_pos + i32::from(p.bits));
            let code_len = i32::from(p.value);
            if code_len < CODE_LENGTH_LITERALS {
                code_lengths[index(symbol)] = code_len;
                symbol += 1;
                if code_len != 0 {
                    prev_code_len = code_len;
                }
            } else {
                let use_prev = code_len == CODE_LENGTH_REPEAT_CODE;
                let slot = index(code_len - CODE_LENGTH_LITERALS);
                let extra_bits = i32::from(CODE_LENGTH_EXTRA_BITS[slot]);
                let repeat_offset = i32::from(CODE_LENGTH_REPEAT_OFFSETS[slot]);
                let mut repeat = read_bits(br, data, extra_bits) + repeat_offset;
                if symbol + repeat > num_symbols {
                    break 'end;
                }
                let length = if use_prev { prev_code_len } else { 0 };
                while repeat > 0 {
                    repeat -= 1;
                    code_lengths[index(symbol)] = length;
                    symbol += 1;
                }
            }
        }
        ok = true;
    }

    vp8l_huffman_tables_deallocate(&mut tables);
    if !ok {
        let _ = vp8l_set_error(dec, VP8StatusCode::VP8_STATUS_BITSTREAM_ERROR);
        return false;
    }
    ok
}

// upstream: src/dec/vp8l_dec.c ReadHuffmanCode
/// Reads one Huffman code of `alphabet_size` symbols and builds its table in
/// `table`, or only validates it when `table` is `None`. `code_lengths` is a
/// pre-allocated temporary buffer, used for creating the Huffman tree; a simple
/// code may set an entry past `alphabet_size`, as in C. Returns the size of the
/// built table, or 0 after recording a bitstream error.
fn read_huffman_code(
    alphabet_size: i32,
    dec: &mut VP8LDecoder,
    data: &[u8],
    code_lengths: &mut [i32],
    table: Option<&mut HuffmanTables>,
) -> i32 {
    let mut ok;
    let mut size = 0;
    let simple_code = read_bits(&mut dec.br, data, 1);

    code_lengths[..index(alphabet_size)].fill(0);

    if simple_code != 0 {
        // Read symbols, codes & code lengths directly.
        let br = &mut dec.br;
        let num_symbols = read_bits(br, data, 1) + 1;
        let first_symbol_len_code = read_bits(br, data, 1);
        // The first code is either 1 bit or 8 bit code.
        let mut symbol = read_bits(br, data, if first_symbol_len_code == 0 { 1 } else { 8 });
        code_lengths[index(symbol)] = 1;
        // The second code (if present), is always 8 bits long.
        if num_symbols == 2 {
            symbol = read_bits(br, data, 8);
            code_lengths[index(symbol)] = 1;
        }
        ok = true;
    } else {
        // Decode Huffman-coded code lengths.
        let mut code_length_code_lengths = [0_i32; NUM_CODE_LENGTH_CODES];
        let num_codes = read_bits(&mut dec.br, data, 4) + 4;
        debug_assert!(index(num_codes) <= NUM_CODE_LENGTH_CODES);

        for &order in &CODE_LENGTH_CODE_ORDER[..index(num_codes)] {
            code_length_code_lengths[usize::from(order)] = read_bits(&mut dec.br, data, 3);
        }
        ok = read_huffman_code_lengths(
            dec,
            data,
            &code_length_code_lengths,
            alphabet_size,
            code_lengths,
        );
    }

    ok = ok && !dec.br.eos;
    if ok {
        size = vp8l_build_huffman_table(
            table,
            HUFFMAN_TABLE_BITS,
            &code_lengths[..index(alphabet_size)],
        );
    }
    if !ok || size == 0 {
        let _ = vp8l_set_error(dec, VP8StatusCode::VP8_STATUS_BITSTREAM_ERROR);
        return 0;
    }
    size
}

// upstream: src/dec/vp8l_dec.c ReadHuffmanCodes
/// Reads the meta Huffman image, when `allow_recursion` permits one, and the
/// Huffman codes of every tree group into `dec.hdr`. On failure the tables and
/// the local images are freed, as in C.
#[expect(
    clippy::similar_names,
    reason = "the C names huffman_xsize and huffman_ysize"
)]
fn read_huffman_codes(
    dec: &mut VP8LDecoder,
    data: &[u8],
    xsize: i32,
    ysize: i32,
    color_cache_bits: i32,
    allow_recursion: bool,
) -> bool {
    let mut huffman_image: Vec<u32> = Vec::new();
    let mut htree_groups: Option<Vec<HTreeGroup>> = None;
    let mut num_htree_groups = 1;
    let mut num_htree_groups_max = 1;
    let mut mapping: Option<Vec<i32>> = None;
    let mut ok = false;

    // Check the table has been 0 initialized (through InitMetadata).
    debug_assert_eq!(dec.hdr.huffman_tables.root.start.len(), 0);
    debug_assert_eq!(dec.hdr.huffman_tables.curr_segment, 0);

    'error: {
        if allow_recursion && read_bits(&mut dec.br, data, 1) != 0 {
            // use meta Huffman codes.
            let huffman_precision =
                MIN_HUFFMAN_BITS + read_bits(&mut dec.br, data, NUM_HUFFMAN_BITS);
            let huffman_xsize = sub_sample_size(xsize, huffman_precision);
            let huffman_ysize = sub_sample_size(ysize, huffman_precision);
            let huffman_pixs = huffman_xsize * huffman_ysize;
            if !decode_image_stream(
                huffman_xsize,
                huffman_ysize,
                /*is_level0=*/ false,
                dec,
                data,
                Some(&mut huffman_image),
            ) {
                break 'error;
            }
            dec.hdr.huffman_subsample_bits = huffman_precision;
            for pixel in &mut huffman_image[..index(huffman_pixs)] {
                // The huffman data is stored in red and green bytes.
                let group = i32::try_from((*pixel >> 8) & 0xffff).expect("16 bits fit an int");
                *pixel = group.cast_unsigned();
                if group >= num_htree_groups_max {
                    num_htree_groups_max = group + 1;
                }
            }
            // Check the validity of num_htree_groups_max. If it seems too big, use a
            // smaller value for later. This will prevent big memory allocations to end
            // up with a bad bitstream anyway.
            // The value of 1000 is totally arbitrary. We know that num_htree_groups_max
            // is smaller than (1 << 16) and should be smaller than the number of pixels
            // (though the format allows it to be bigger).
            if num_htree_groups_max > 1000 || num_htree_groups_max > xsize * ysize {
                // Create a mapping from the used indices to the minimal set of used
                // values [0, num_htree_groups)
                let Some(mut map) = webp_safe_malloc::<i32>(
                    u64::from(num_htree_groups_max.cast_unsigned()),
                    size_of::<i32>(),
                ) else {
                    let _ = vp8l_set_error(dec, VP8StatusCode::VP8_STATUS_OUT_OF_MEMORY);
                    break 'error;
                };
                // -1 means a value is unmapped, and therefore unused in the Huffman
                // image.
                map.resize(index(num_htree_groups_max), -1);
                num_htree_groups = 0;
                for pixel in &mut huffman_image[..index(huffman_pixs)] {
                    // Get the current mapping for the group and remap the Huffman image.
                    let mapped_group = &mut map[*pixel as usize];
                    if *mapped_group == -1 {
                        *mapped_group = num_htree_groups;
                        num_htree_groups += 1;
                    }
                    *pixel = mapped_group.cast_unsigned();
                }
                mapping = Some(map);
            } else {
                num_htree_groups = num_htree_groups_max;
            }
        }

        if dec.br.eos {
            break 'error;
        }

        let mut huffman_tables = std::mem::take(&mut dec.hdr.huffman_tables);
        let helper_ok = read_huffman_codes_helper(
            color_cache_bits,
            num_htree_groups,
            num_htree_groups_max,
            mapping.as_deref(),
            dec,
            data,
            &mut huffman_tables,
            &mut htree_groups,
        );
        dec.hdr.huffman_tables = huffman_tables;
        if !helper_ok {
            break 'error;
        }
        ok = true;

        // All OK. Finalize pointers.
        dec.hdr.huffman_image = std::mem::take(&mut huffman_image);
        dec.hdr.num_htree_groups = num_htree_groups;
        dec.hdr.htree_groups = htree_groups.take().unwrap_or_default();
    }

    drop(mapping);
    if !ok {
        drop(huffman_image);
        vp8l_huffman_tables_deallocate(&mut dec.hdr.huffman_tables);
        drop(htree_groups);
    }
    ok
}

// upstream: src/dec/vp8l_dec.c ReadHuffmanCodesHelper
/// Helper function for reading the different Huffman codes and storing them in
/// `huffman_tables` and `htree_groups`.
///
/// If `mapping` is `None`, `num_htree_groups_max` must equal `num_htree_groups`.
/// If it is not `None`, it maps `num_htree_groups_max` indices to the
/// `num_htree_groups` groups. If `num_htree_groups_max > num_htree_groups`,
/// some of those indices map to -1. This is used for non-balanced codes to
/// limit memory usage. `htree_groups` is `None` where C has `NULL`; on failure
/// the tables are freed and `htree_groups` becomes `None`, as in C.
///
/// # Panics
///
/// Panics when `color_cache_bits` is outside 0 to 11, or `mapping` is shorter
/// than `num_htree_groups_max` or maps outside the groups, where C reads or
/// writes outside its arrays.
#[expect(
    clippy::too_many_arguments,
    reason = "mirrors the parameters of ReadHuffmanCodesHelper plus the input"
)]
#[expect(
    clippy::too_many_lines,
    reason = "mirrors ReadHuffmanCodesHelper, whose loops share its error exit"
)]
#[must_use]
pub fn read_huffman_codes_helper(
    color_cache_bits: i32,
    num_htree_groups: i32,
    num_htree_groups_max: i32,
    mapping: Option<&[i32]>,
    dec: &mut VP8LDecoder,
    data: &[u8],
    huffman_tables: &mut HuffmanTables,
    htree_groups: &mut Option<Vec<HTreeGroup>>,
) -> bool {
    let mut ok = false;
    let max_alphabet_size = i32::from(ALPHABET_SIZE[0])
        + if color_cache_bits > 0 {
            1 << color_cache_bits
        } else {
            0
        };
    let table_size = i32::from(TABLE_SIZE[index(color_cache_bits)]);
    let mut code_lengths: Option<Vec<i32>> = None;

    'error: {
        if (mapping.is_none() && num_htree_groups != num_htree_groups_max)
            || num_htree_groups > num_htree_groups_max
        {
            break 'error;
        }

        code_lengths = webp_safe_calloc::<i32>(
            u64::from(max_alphabet_size.cast_unsigned()),
            size_of::<i32>(),
        );
        *htree_groups = vp8l_htree_groups_new(num_htree_groups);

        let (Some(groups), Some(code_lengths)) = (htree_groups.as_mut(), code_lengths.as_mut())
        else {
            let _ = vp8l_set_error(dec, VP8StatusCode::VP8_STATUS_OUT_OF_MEMORY);
            break 'error;
        };
        if !vp8l_huffman_tables_allocate(num_htree_groups * table_size, huffman_tables) {
            let _ = vp8l_set_error(dec, VP8StatusCode::VP8_STATUS_OUT_OF_MEMORY);
            break 'error;
        }

        for i in 0..index(num_htree_groups_max) {
            // If the index "i" is unused in the Huffman image, just make sure the
            // coefficients are valid but do not store them.
            if let Some(mapping) = mapping
                && mapping[i] == -1
            {
                for (j, &size) in ALPHABET_SIZE.iter().enumerate() {
                    let mut alphabet_size = i32::from(size);
                    if j == 0 && color_cache_bits > 0 {
                        alphabet_size += 1 << color_cache_bits;
                    }
                    // Passing in NULL so that nothing gets filled.
                    if read_huffman_code(alphabet_size, dec, data, code_lengths, None) == 0 {
                        break 'error;
                    }
                }
            } else {
                let group = match mapping {
                    None => i,
                    Some(mapping) => index(mapping[i]),
                };
                let htree_group = &mut groups[group];
                let mut total_size = 0;
                let mut is_trivial_literal = true;
                let mut max_bits = 0;
                for (j, &size) in ALPHABET_SIZE.iter().enumerate() {
                    let mut alphabet_size = i32::from(size);
                    if j == 0 && color_cache_bits > 0 {
                        alphabet_size += 1 << color_cache_bits;
                    }
                    let size = read_huffman_code(
                        alphabet_size,
                        dec,
                        data,
                        code_lengths,
                        Some(huffman_tables),
                    );
                    let curr_segment = huffman_tables.curr_segment;
                    htree_group.htrees[j] = HuffmanTablesOffset {
                        segment: curr_segment,
                        offset: huffman_tables.segment(curr_segment).curr_table,
                    };
                    if size == 0 {
                        break 'error;
                    }
                    let first = huffman_entry(huffman_tables, htree_group.htrees[j], 0);
                    if is_trivial_literal && LITERAL_MAP[j] == 1 {
                        is_trivial_literal = first.bits == 0;
                    }
                    total_size += i32::from(first.bits);
                    huffman_tables.segment_mut(curr_segment).curr_table += index(size);
                    if j <= HuffIndex::ALPHA.index() {
                        let mut local_max_bits = code_lengths[0];
                        for &code_length in &code_lengths[1..index(alphabet_size)] {
                            if code_length > local_max_bits {
                                local_max_bits = code_length;
                            }
                        }
                        max_bits += local_max_bits;
                    }
                }
                htree_group.is_trivial_literal = is_trivial_literal;
                htree_group.is_trivial_code = false;
                if is_trivial_literal {
                    let htrees = htree_group.htrees;
                    let red = i32::from(
                        huffman_entry(huffman_tables, htrees[HuffIndex::RED.index()], 0).value,
                    );
                    let blue = i32::from(
                        huffman_entry(huffman_tables, htrees[HuffIndex::BLUE.index()], 0).value,
                    );
                    let alpha = u32::from(
                        huffman_entry(huffman_tables, htrees[HuffIndex::ALPHA.index()], 0).value,
                    );
                    htree_group.literal_arb =
                        (alpha << 24) | (red << 16).cast_unsigned() | blue.cast_unsigned();
                    let green = huffman_entry(huffman_tables, htrees[HuffIndex::GREEN.index()], 0);
                    if total_size == 0 && i32::from(green.value) < NUM_LITERAL_CODES {
                        htree_group.is_trivial_code = true;
                        htree_group.literal_arb |= (i32::from(green.value) << 8).cast_unsigned();
                    }
                }
                htree_group.use_packed_table =
                    !htree_group.is_trivial_code && (max_bits < HUFFMAN_PACKED_BITS);
                if htree_group.use_packed_table {
                    build_packed_table(htree_group, huffman_tables);
                }
            }
        }
        ok = true;
    }

    drop(code_lengths);
    if !ok {
        vp8l_huffman_tables_deallocate(huffman_tables);
        *htree_groups = None;
    }
    ok
}

/// `VP8LSubSampleSize` on the C `int` arguments that the decoder passes; the
/// result, at most the size, converts back to `int` exactly.
fn sub_sample_size(size: i32, sampling_bits: i32) -> i32 {
    i32::try_from(vp8l_sub_sample_size(
        size.cast_unsigned(),
        sampling_bits.cast_unsigned(),
    ))
    .expect("a subsampled size fits an int")
}

//------------------------------------------------------------------------------
// Scaling.

// upstream: src/dec/vp8l_dec.c AllocateAndInitRescaler
/// Allocates the rescaler of the decoder for `mb_w` by `mb_h` pixels to
/// `scaled_width` by `scaled_height`, with 4 channels. C carves the rescaler,
/// its work rows and the scaled row from one block; the size test of
/// `WebPSafeMalloc` is applied to that total before the separate buffers are
/// allocated (ADR 0003).
fn allocate_and_init_rescaler(dec: &mut VP8LDecoder, io: &VP8Io) -> bool {
    let num_channels = 4_i32;
    let in_width = io.mb_w;
    let out_width = io.scaled_width;
    let in_height = io.mb_h;
    let out_height = io.scaled_height;
    let out_width_u64 = i64::from(out_width).cast_unsigned();
    let work_size: u64 = 2 * u64::from(num_channels.cast_unsigned()) * out_width_u64;
    let scaled_data_size: u64 = out_width_u64;
    let memory_size: u64 = (WebPRescaler::C_SIZE as u64)
        .wrapping_add(work_size.wrapping_mul(size_of::<rescaler_t>() as u64))
        .wrapping_add(scaled_data_size.wrapping_mul(size_of::<u32>() as u64));
    if webp_safe_malloc::<u8>(memory_size, size_of::<u8>()).is_none() {
        return vp8l_set_error(dec, VP8StatusCode::VP8_STATUS_OUT_OF_MEMORY)
            == VP8StatusCode::VP8_STATUS_OK;
    }
    let (Some(mut work), Some(mut scaled_data)) = (
        webp_safe_malloc::<rescaler_t>(work_size, size_of::<rescaler_t>()),
        webp_safe_malloc::<u8>(scaled_data_size * 4, size_of::<u8>()),
    ) else {
        return vp8l_set_error(dec, VP8StatusCode::VP8_STATUS_OUT_OF_MEMORY)
            == VP8StatusCode::VP8_STATUS_OK;
    };
    work.resize(
        usize::try_from(work_size).expect("an accepted size fits"),
        0,
    );
    scaled_data.resize(
        usize::try_from(scaled_data_size * 4).expect("an accepted size fits"),
        0,
    );
    debug_assert!(dec.rescaler.is_none());
    dec.rescaled_row = scaled_data;

    match webp_rescaler_init(
        in_width,
        in_height,
        out_width,
        out_height,
        0,
        num_channels,
        work,
    ) {
        Some(rescaler) => {
            dec.rescaler = Some(rescaler);
            true
        }
        None => false,
    }
}

//------------------------------------------------------------------------------
// Export to ARGB

/// The `uint32_t` pixels that C reads through a cast of a byte row of the
/// rescaler: `width` pixels in native byte order.
fn argb_from_bytes(bytes: &[u8], width: i32) -> Vec<u32> {
    bytes[..4 * index(width)]
        .as_chunks::<4>()
        .0
        .iter()
        .map(|pixel| u32::from_ne_bytes(*pixel))
        .collect()
}

/// The bytes that C reads through a cast of `num_rows` rows of `uint32_t`
/// pixels starting at `pos`, `stride` pixels apart, `width` pixels each: the
/// rows in native byte order at the same byte offsets, the last row ending after
/// its `width` pixels.
fn argb_rows_as_bytes(
    data: &[u32],
    pos: usize,
    stride: usize,
    width: i32,
    num_rows: i32,
) -> Vec<u8> {
    let length = (index(num_rows) - 1) * stride + index(width);
    data[pos..pos + length]
        .iter()
        .flat_map(|pixel| pixel.to_ne_bytes())
        .collect()
}

// upstream: src/dec/vp8l_dec.c Export
/// We have special "export" function since we need to convert from BGRA.
/// Writes the pending rows of the rescaler into `out` from `rgba` on, `rgba_stride`
/// bytes apart, and returns their number.
fn export(
    rescaler: &mut WebPRescaler,
    scaled_row: &mut [u8],
    colorspace: WEBP_CSP_MODE,
    rgba_stride: i32,
    out: &mut [u8],
    rgba: usize,
) -> i32 {
    let src = rescaler.dst_pos;
    let dst_width = rescaler.dst_width;
    let mut num_lines_out = 0;
    while webp_rescaler_has_pending_output(rescaler) {
        webp_rescaler_export_row(rescaler, scaled_row);
        let mut pixels = argb_from_bytes(&scaled_row[src..], dst_width);
        webp_mult_argb_row(&mut pixels, dst_width, true);
        let dst = row_position(rgba, num_lines_out, rgba_stride);
        vp8l_convert_from_bgra(&pixels, dst_width, colorspace, &mut out[dst..]);
        num_lines_out += 1;
    }
    num_lines_out
}

// upstream: src/dec/vp8l_dec.c EmitRescaledRowsRGBA
/// Emit scaled rows: `mb_h` rows of `data` from `in_pos` on, `in_stride` pixels
/// apart, through the rescaler into `out` from `out_pos` on, `out_stride` bytes
/// apart. Returns the number of rows written.
#[expect(
    clippy::too_many_arguments,
    reason = "mirrors the parameters of EmitRescaledRowsRGBA with the buffers passed per call"
)]
fn emit_rescaled_rows_rgba(
    dec: &mut VP8LDecoder,
    colorspace: WEBP_CSP_MODE,
    data: &mut [u32],
    in_pos: usize,
    in_stride: usize,
    mb_h: i32,
    out: &mut [u8],
    out_pos: usize,
    out_stride: i32,
) -> i32 {
    let VP8LDecoder {
        rescaler,
        rescaled_row,
        ..
    } = dec;
    let rescaler = rescaler.as_mut().expect("the rescaler is allocated");
    let mut num_lines_in = 0;
    let mut num_lines_out = 0;
    while num_lines_in < mb_h {
        let row_in = in_pos + index(num_lines_in) * in_stride;
        let row_out = row_position(out_pos, num_lines_out, out_stride);
        let lines_left = mb_h - num_lines_in;
        let needed_lines = webp_rescale_needed_lines(rescaler, lines_left);
        debug_assert!(needed_lines > 0 && needed_lines <= lines_left);
        webp_mult_argb_rows(
            &mut data[row_in..],
            stride_int(in_stride),
            rescaler.src_width,
            needed_lines,
            false,
        );
        let rows = argb_rows_as_bytes(data, row_in, in_stride, rescaler.src_width, lines_left);
        let lines_imported =
            webp_rescaler_import(rescaler, lines_left, &rows, stride_int(4 * in_stride));
        debug_assert_eq!(lines_imported, needed_lines);
        num_lines_in += lines_imported;
        num_lines_out += export(rescaler, rescaled_row, colorspace, out_stride, out, row_out);
    }
    num_lines_out
}

/// A stride that C keeps in an `int`.
fn stride_int(stride: usize) -> i32 {
    i32::try_from(stride).expect("a row stride fits an int")
}

// upstream: src/dec/vp8l_dec.c EmitRows
/// Emit rows without any scaling: `mb_h` rows of `mb_w` pixels of `data` from
/// `row_in` on, `in_stride` pixels apart, into `out` from `out_pos` on,
/// `out_stride` bytes apart.
#[expect(
    clippy::too_many_arguments,
    reason = "mirrors the parameters of EmitRows with the buffers passed per call"
)]
fn emit_rows(
    colorspace: WEBP_CSP_MODE,
    data: &[u32],
    row_in: usize,
    in_stride: usize,
    mb_w: i32,
    mb_h: i32,
    out: &mut [u8],
    out_pos: usize,
    out_stride: i32,
) -> i32 {
    let mut lines = mb_h;
    let mut row_in = row_in;
    let mut row = 0;
    while lines > 0 {
        lines -= 1;
        let row_out = row_position(out_pos, row, out_stride);
        vp8l_convert_from_bgra(&data[row_in..], mb_w, colorspace, &mut out[row_out..]);
        row_in += in_stride;
        row += 1;
    }
    mb_h // Num rows out == num rows in.
}

//------------------------------------------------------------------------------
// Export to YUVA

/// Two rows of a decoding buffer that the converters write together: `first`
/// and `second` are `(region, position)` pairs.
///
/// # Panics
///
/// Panics when a region is missing or the rows overlap.
fn two_rows_mut<'s>(
    storage: &'s mut PlaneStorage<'_, u8>,
    first: (usize, usize),
    second: (usize, usize),
) -> (&'s mut [u8], &'s mut [u8]) {
    if first.0 == second.0 {
        let region = storage
            .region_mut(first.0)
            .expect("the plane's region exists");
        if first.1 < second.1 {
            let (head, tail) = region.split_at_mut(second.1);
            (&mut head[first.1..], tail)
        } else {
            let (head, tail) = region.split_at_mut(first.1);
            (tail, &mut head[second.1..])
        }
    } else {
        let PlaneStorage::External(slices) = storage else {
            panic!("an output buffer with several regions has external storage");
        };
        let [a, b] = slices
            .get_disjoint_mut([first.0, second.0])
            .expect("the planes' regions exist");
        (&mut a[first.1..], &mut b[second.1..])
    }
}

// upstream: src/dec/vp8l_dec.c ConvertToYUVA
/// Converts one row of `width` BGRA pixels to the row `y_pos` of the YUVA
/// output.
fn convert_to_yuva(src: &[u32], width: i32, y_pos: i32, output: &mut WebPDecBuffer<'_>) {
    let WebPDecBufferPlanes::YUVA(buf) = output.u else {
        panic!("a YUV colorspace has YUVA planes");
    };

    // first, the luma plane
    {
        let y = row_position(buf.y.offset, y_pos, buf.y_stride);
        let region = output
            .storage
            .region_mut(buf.y.region)
            .expect("the luma region exists");
        webp_convert_argb_to_y(src, &mut region[y..], width);
    }

    // then U/V planes
    {
        let u = row_position(buf.u.offset, y_pos >> 1, buf.u_stride);
        let v = row_position(buf.v.offset, y_pos >> 1, buf.v_stride);
        let (u, v) = two_rows_mut(&mut output.storage, (buf.u.region, u), (buf.v.region, v));
        // even lines: store values
        // odd lines: average with previous values
        webp_convert_argb_to_uv(src, u, v, width, i32::from((y_pos & 1) == 0));
    }
    // Lastly, store alpha if needed.
    if let Some(a) = buf.a {
        let a_row = row_position(a.offset, y_pos, buf.a_stride);
        let region = output
            .storage
            .region_mut(a.region)
            .expect("the alpha region exists");
        let bytes: Vec<u8> = src[..index(width)]
            .iter()
            .flat_map(|pixel| pixel.to_ne_bytes())
            .collect();
        // C: (uint8_t*)src + 0 on big-endian hosts, + 3 otherwise.
        let alpha_byte = if cfg!(target_endian = "big") { 0 } else { 3 };
        let _ = webp_extract_alpha(&bytes[alpha_byte..], 0, width, 1, &mut region[a_row..], 0);
    }
}

// upstream: src/dec/vp8l_dec.c ExportYUVA
/// Writes the pending rows of the rescaler to the YUVA output from row `y_pos`
/// on, and returns their number.
fn export_yuva(dec: &mut VP8LDecoder, y_pos: i32, output: &mut WebPDecBuffer<'_>) -> i32 {
    let VP8LDecoder {
        rescaler,
        rescaled_row,
        ..
    } = dec;
    let rescaler = rescaler.as_mut().expect("the rescaler is allocated");
    let src = rescaler.dst_pos;
    let dst_width = rescaler.dst_width;
    let mut y_pos = y_pos;
    let mut num_lines_out = 0;
    while webp_rescaler_has_pending_output(rescaler) {
        webp_rescaler_export_row(rescaler, rescaled_row);
        let mut pixels = argb_from_bytes(&rescaled_row[src..], dst_width);
        webp_mult_argb_row(&mut pixels, dst_width, true);
        convert_to_yuva(&pixels, dst_width, y_pos, output);
        y_pos += 1;
        num_lines_out += 1;
    }
    num_lines_out
}

// upstream: src/dec/vp8l_dec.c EmitRescaledRowsYUVA
/// Emit scaled rows to the YUVA output: `mb_h` rows of `data` from `in_pos` on,
/// `in_stride` pixels apart. Returns the next output row.
fn emit_rescaled_rows_yuva(
    dec: &mut VP8LDecoder,
    data: &mut [u32],
    in_pos: usize,
    in_stride: usize,
    mb_h: i32,
    output: &mut WebPDecBuffer<'_>,
) -> i32 {
    let mut in_pos = in_pos;
    let mut num_lines_in = 0;
    let mut y_pos = dec.last_out_row;
    while num_lines_in < mb_h {
        let lines_left = mb_h - num_lines_in;
        let rescaler = dec.rescaler.as_mut().expect("the rescaler is allocated");
        let needed_lines = webp_rescale_needed_lines(rescaler, lines_left);
        webp_mult_argb_rows(
            &mut data[in_pos..],
            stride_int(in_stride),
            rescaler.src_width,
            needed_lines,
            false,
        );
        let rows = argb_rows_as_bytes(data, in_pos, in_stride, rescaler.src_width, lines_left);
        let lines_imported =
            webp_rescaler_import(rescaler, lines_left, &rows, stride_int(4 * in_stride));
        debug_assert_eq!(lines_imported, needed_lines);
        num_lines_in += lines_imported;
        in_pos += index(needed_lines) * in_stride;
        y_pos += export_yuva(dec, y_pos, output);
    }
    y_pos
}

// upstream: src/dec/vp8l_dec.c EmitRowsYUVA
/// Emit rows without scaling to the YUVA output: `num_rows` rows of `mb_w`
/// pixels of `data` from `in_pos` on, `in_stride` pixels apart. Returns the next
/// output row.
fn emit_rows_yuva(
    dec: &VP8LDecoder,
    data: &[u32],
    in_pos: usize,
    in_stride: usize,
    mb_w: i32,
    num_rows: i32,
    output: &mut WebPDecBuffer<'_>,
) -> i32 {
    let mut y_pos = dec.last_out_row;
    let mut in_pos = in_pos;
    let mut num_rows = num_rows;
    while num_rows > 0 {
        num_rows -= 1;
        convert_to_yuva(&data[in_pos..], mb_w, y_pos, output);
        in_pos += in_stride;
        y_pos += 1;
    }
    y_pos
}

//------------------------------------------------------------------------------
// Cropping.

// upstream: src/dec/vp8l_dec.c SetCropWindow
/// Sets `io.mb_y`, `io.mb_h` and `io.mb_w` according to start row, end row and
/// crop options. Also updates the input position, so that it points to the
/// start of the cropped window. `in_data` and `pixel_stride` count pixels,
/// where C counts the bytes of the same ARGB pixels. Returns true if the crop
/// window is not empty.
fn set_crop_window(
    io: &mut VP8Io,
    y_start: i32,
    y_end: i32,
    in_data: &mut usize,
    pixel_stride: usize,
) -> bool {
    let mut y_start = y_start;
    let mut y_end = y_end;
    debug_assert!(y_start < y_end);
    debug_assert!(io.crop_left < io.crop_right);
    if y_end > io.crop_bottom {
        y_end = io.crop_bottom; // make sure we don't overflow on last row.
    }
    if y_start < io.crop_top {
        let delta = io.crop_top - y_start;
        y_start = io.crop_top;
        *in_data += index(delta) * pixel_stride;
    }
    if y_start >= y_end {
        return false; // Crop window is empty.
    }

    *in_data += index(io.crop_left);

    io.mb_y = y_start - io.crop_top;
    io.mb_w = io.crop_right - io.crop_left;
    io.mb_h = y_end - y_start;
    true // Non-empty crop window.
}

//------------------------------------------------------------------------------

// upstream: src/dec/vp8l_dec.c GetMetaIndex
/// The tree group of the tile at `x`, `y` in the Huffman image.
fn get_meta_index(image: &[u32], xsize: i32, bits: i32, x: i32, y: i32) -> i32 {
    if bits == 0 {
        return 0;
    }
    i32::try_from(image[index(xsize * (y >> bits) + (x >> bits))])
        .expect("a remapped group fits an int")
}

// upstream: src/dec/vp8l_dec.c GetHtreeGroupForPos
/// The index in `hdr.htree_groups` of the tree group at `x`, `y`.
fn get_htree_group_for_pos(hdr: &VP8LMetadata, x: i32, y: i32) -> usize {
    let meta_index = get_meta_index(
        &hdr.huffman_image,
        hdr.huffman_xsize,
        hdr.huffman_subsample_bits,
        x,
        y,
    );
    debug_assert!(meta_index < hdr.num_htree_groups);
    index(meta_index)
}

//------------------------------------------------------------------------------
// Main loop, with custom row-processing function

// upstream: src/dec/vp8l_dec.c ProcessRowsFunc
/// A row-processing function with its context: it receives the decoder, the
/// pixel buffer that C reads through `dec->pixels`, and the row up to which
/// rows are decoded.
type ProcessRowsFunc<'f> = &'f mut dyn FnMut(&mut VP8LDecoder, &mut [u32], i32);

// upstream: src/dec/vp8l_dec.c ApplyInverseTransforms
/// Applies the inverse transforms to `num_rows` rows from `start_row` on, read
/// from `rows` in `data`, into `argb_cache`.
fn apply_inverse_transforms(
    dec: &VP8LDecoder,
    data: &mut [u32],
    start_row: i32,
    num_rows: i32,
    rows: usize,
) {
    let mut n = dec.next_transform;
    let cache_pixs = dec.width * num_rows;
    let end_row = start_row + num_rows;
    let mut rows_in = rows;
    let rows_out = dec.argb_cache.expect("argb_cache is allocated");

    // Inverse transforms.
    while n > 0 {
        n -= 1;
        let transform: &VP8LTransform = &dec.transforms[index(n)];
        vp8l_inverse_transform(transform, start_row, end_row, data, rows_in, rows_out);
        rows_in = rows_out;
    }
    if rows_in != rows_out {
        // No transform called, hence just copy.
        data.copy_within(rows_in..rows_in + index(cache_pixs), rows_out);
    }
}

// upstream: src/dec/vp8l_dec.c ProcessRows
/// Processes (transforms, scales & color-converts) the rows decoded after the
/// last call.
fn process_rows(
    dec: &mut VP8LDecoder,
    data: &mut [u32],
    row: i32,
    io: &mut VP8Io,
    output: &mut WebPDecBuffer<'_>,
) {
    let rows = index(dec.width * dec.last_row);
    let num_rows = row - dec.last_row;

    debug_assert!(row <= io.crop_bottom);
    // We can't process more than NUM_ARGB_CACHE_ROWS at a time (that's the size
    // of argb_cache), but we currently don't need more than that.
    debug_assert!(num_rows <= NUM_ARGB_CACHE_ROWS);
    if num_rows > 0 {
        // Emit output.
        let mut rows_data = dec.argb_cache.expect("argb_cache is allocated");
        let in_stride = index(io.width); // in unit of RGBA
        apply_inverse_transforms(dec, data, dec.last_row, num_rows, rows);
        if !set_crop_window(io, dec.last_row, row, &mut rows_data, in_stride) {
            // Nothing to output (this time).
        } else if webp_is_rgb_mode(output.colorspace) {
            // convert to RGBA
            let colorspace = output.colorspace;
            let WebPDecBufferPlanes::RGBA(buf) = output.u else {
                panic!("an RGB colorspace has an RGBA plane");
            };
            let out = output
                .storage
                .region_mut(buf.rgba.region)
                .expect("the RGBA region exists");
            let rgba = row_position(buf.rgba.offset, dec.last_out_row, buf.stride);
            let num_rows_out = if io.use_scaling {
                emit_rescaled_rows_rgba(
                    dec, colorspace, data, rows_data, in_stride, io.mb_h, out, rgba, buf.stride,
                )
            } else {
                emit_rows(
                    colorspace, data, rows_data, in_stride, io.mb_w, io.mb_h, out, rgba, buf.stride,
                )
            };
            // Update 'last_out_row'.
            dec.last_out_row += num_rows_out;
            debug_assert!(dec.last_out_row <= output.height);
        } else {
            // convert to YUVA
            dec.last_out_row = if io.use_scaling {
                emit_rescaled_rows_yuva(dec, data, rows_data, in_stride, io.mb_h, output)
            } else {
                emit_rows_yuva(dec, data, rows_data, in_stride, io.mb_w, io.mb_h, output)
            };
            debug_assert!(dec.last_out_row <= output.height);
        }
    }

    // Update 'last_row'.
    dec.last_row = row;
    debug_assert!(dec.last_row <= dec.height);
}

// upstream: src/dec/vp8l_dec.c Is8bOptimizable
/// Row-processing for the special case when alpha data contains only one
/// transform (color indexing), and trivial non-green literals.
fn is8b_optimizable(hdr: &VP8LMetadata) -> bool {
    if hdr.color_cache_size > 0 {
        return false;
    }
    // When the Huffman tree contains only one symbol, we can skip the
    // call to ReadSymbol() for red/blue/alpha channels.
    for group in &hdr.htree_groups[..index(hdr.num_htree_groups)] {
        let htrees = &group.htrees;
        let tables = &hdr.huffman_tables;
        if huffman_entry(tables, htrees[HuffIndex::RED.index()], 0).bits > 0 {
            return false;
        }
        if huffman_entry(tables, htrees[HuffIndex::BLUE.index()], 0).bits > 0 {
            return false;
        }
        if huffman_entry(tables, htrees[HuffIndex::ALPHA.index()], 0).bits > 0 {
            return false;
        }
    }
    true
}

/// The parts of an `ALPHDecoder` that the alpha row functions use, with the
/// alpha plane that the VP8 decoder passes (C: `alph_dec` through
/// `dec->io->opaque`, and `alph_dec->output`).
struct AlphaOutput<'o> {
    /// The dimensions and the crop window (C: `dec->io`).
    io: &'o VP8Io,
    /// The prediction filter (C: `alph_dec->filter`).
    filter: WEBP_FILTER_TYPE,
    /// The last output row, as a position in `output` (C: `alph_dec->prev_line`).
    prev_line: &'o mut Option<usize>,
    /// The alpha plane (C: `alph_dec->output`).
    output: &'o mut [u8],
}

// upstream: src/dec/vp8l_dec.c AlphaApplyFilter
/// Unfilters the rows `first_row` to `last_row` of the alpha plane in place,
/// from `out` on, `stride` bytes apart.
fn alpha_apply_filter(
    alph: &mut AlphaOutput<'_>,
    first_row: i32,
    last_row: i32,
    out: usize,
    stride: i32,
) {
    if alph.filter != WEBP_FILTER_TYPE::WEBP_FILTER_NONE {
        let width = index(stride);
        let mut out = out;
        let mut prev_line = *alph.prev_line;
        for _ in first_row..last_row {
            // The unfilter reads each input sample before it writes the same
            // index, so a copy of the row gives the bytes of the in-place call.
            let input = alph.output[out..out + width].to_vec();
            let (head, tail) = alph.output.split_at_mut(out);
            let prev = prev_line.map(|prev| &head[prev..prev + width]);
            webp_unfilters(alph.filter, prev, &input, &mut tail[..width], stride);
            prev_line = Some(out);
            out += width;
        }
        *alph.prev_line = prev_line;
    }
}

// upstream: src/dec/vp8l_dec.c ExtractPalettedAlphaRows
/// Applies the color indexing transform to the 8-bit indexes of `pixels` and
/// unfilters the rows up to `last_row` into the alpha plane.
fn extract_paletted_alpha_rows(
    dec: &mut VP8LDecoder,
    pixels: &[u8],
    last_row: i32,
    alph: &mut AlphaOutput<'_>,
) {
    // For vertical and gradient filtering, we need to decode the part above the
    // crop_top row, in order to have the correct spatial predictors.
    let top_row = if alph.filter == WEBP_FILTER_TYPE::WEBP_FILTER_NONE
        || alph.filter == WEBP_FILTER_TYPE::WEBP_FILTER_HORIZONTAL
    {
        alph.io.crop_top
    } else {
        dec.last_row
    };
    let first_row = if dec.last_row < top_row {
        top_row
    } else {
        dec.last_row
    };
    debug_assert!(last_row <= alph.io.crop_bottom);
    if last_row > first_row {
        // Special method for paletted alpha data. We only process the cropped area.
        let width = alph.io.width;
        let out = index(width * first_row);
        let input = index(dec.width * first_row);
        let transform = &dec.transforms[0];
        debug_assert_eq!(dec.next_transform, 1);
        debug_assert_eq!(
            transform.r#type,
            VP8LImageTransformType::COLOR_INDEXING_TRANSFORM
        );
        vp8l_color_index_inverse_transform_alpha(
            transform,
            first_row,
            last_row,
            &pixels[input..],
            &mut alph.output[out..],
        );
        alpha_apply_filter(alph, first_row, last_row, out, width);
    }
    dec.last_row = last_row;
    dec.last_out_row = last_row;
}

//------------------------------------------------------------------------------
// Helper functions for fast pattern copy (8b and 32b)

// upstream: src/dec/vp8l_dec.c Rotate8b
/// Cyclic rotation of pattern word.
fn rotate8b(v: u32) -> u32 {
    if cfg!(target_endian = "big") {
        ((v & 0xff00_0000) >> 24) | (v << 8)
    } else {
        ((v & 0xff) << 24) | (v >> 8)
    }
}

/// The address of element `pos` of `data`, which C tests for alignment.
fn address<T>(data: &[T], pos: usize) -> usize {
    data.as_ptr().addr() + pos * size_of::<T>()
}

// upstream: src/dec/vp8l_dec.c CopySmallPattern8b
/// Copy 1, 2 or 4-bytes pattern.
fn copy_small_pattern8b(data: &mut [u8], src: usize, dst: usize, length: i32, pattern: u32) {
    let mut src = src;
    let mut dst = dst;
    let mut length = length;
    let mut pattern = pattern;
    // align 'dst' to 4-bytes boundary. Adjust the pattern along the way.
    while address(data, dst) & 3 != 0 {
        data[dst] = data[src];
        dst += 1;
        src += 1;
        pattern = rotate8b(pattern);
        length -= 1;
    }
    // Copy the pattern 4 bytes at a time.
    let words = index(length >> 2);
    for i in 0..words {
        data[dst + 4 * i..dst + 4 * i + 4].copy_from_slice(&pattern.to_ne_bytes());
    }
    // Finish with left-overs. 'pattern' is still correctly positioned,
    // so no Rotate8b() call is needed.
    for i in words << 2..index(length) {
        data[dst + i] = data[src + i];
    }
}

// upstream: src/dec/vp8l_dec.c CopyBlock8b
/// Copies `length` bytes from `dist` bytes before `dst` to `dst`; the regions may
/// overlap, and the copy proceeds forward as in C.
#[expect(clippy::similar_names, reason = "the C names dst and dist")]
fn copy_block8b(data: &mut [u8], dst: usize, dist: i32, length: i32) {
    let src = dst - index(dist);
    if length >= 8 {
        // C's branches for 32-bit ARM and MIPS compute the same patterns.
        let pattern = match dist {
            1 => Some(0x0101_0101_u32.wrapping_mul(u32::from(data[src]))),
            2 => {
                let pattern = if cfg!(target_endian = "big") {
                    (u32::from(data[src]) << 8) | u32::from(data[src + 1])
                } else {
                    u32::from(u16::from_ne_bytes([data[src], data[src + 1]]))
                };
                Some(0x0001_0001_u32.wrapping_mul(pattern))
            }
            4 => Some(u32::from_ne_bytes([
                data[src],
                data[src + 1],
                data[src + 2],
                data[src + 3],
            ])),
            _ => None,
        };
        if let Some(pattern) = pattern {
            copy_small_pattern8b(data, src, dst, length, pattern);
            return;
        }
    }
    // Copy:
    if dist >= length {
        // no overlap -> use memcpy()
        data.copy_within(src..src + index(length), dst);
    } else {
        for i in 0..index(length) {
            data[dst + i] = data[src + i];
        }
    }
}

// upstream: src/dec/vp8l_dec.c CopySmallPattern32b
/// Copy pattern of 1 or 2 `uint32_t`'s.
fn copy_small_pattern32b(data: &mut [u32], src: usize, dst: usize, length: i32, pattern: u64) {
    let mut src = src;
    let mut dst = dst;
    let mut length = length;
    let mut pattern = pattern;
    if address(data, dst) & 4 != 0 {
        // Align 'dst' to 8-bytes boundary.
        data[dst] = data[src];
        dst += 1;
        src += 1;
        pattern = pattern.rotate_left(32);
        length -= 1;
    }
    debug_assert_eq!(address(data, dst) & 7, 0);
    let bytes = pattern.to_ne_bytes();
    let first = u32::from_ne_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
    let second = u32::from_ne_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
    let pairs = index(length >> 1);
    for i in 0..pairs {
        // Copy the pattern 8 bytes at a time.
        data[dst + 2 * i] = first;
        data[dst + 2 * i + 1] = second;
    }
    if length & 1 != 0 {
        // Finish with left-over.
        data[dst + (pairs << 1)] = data[src + (pairs << 1)];
    }
}

// upstream: src/dec/vp8l_dec.c CopyBlock32b
/// Copies `length` pixels from `dist` pixels before `dst` to `dst`; the regions
/// may overlap, and the copy proceeds forward as in C.
#[expect(clippy::similar_names, reason = "the C names dst and dist")]
#[expect(
    clippy::verbose_bit_mask,
    reason = "the C test of the pointer's low bits"
)]
fn copy_block32b(data: &mut [u32], dst: usize, dist: i32, length: i32) {
    let src = dst - index(dist);
    if dist <= 2 && length >= 4 && (address(data, dst) & 3) == 0 {
        let pattern = if dist == 1 {
            let pattern = u64::from(data[src]);
            pattern | (pattern << 32)
        } else {
            let first = data[src].to_ne_bytes();
            let second = data[src + 1].to_ne_bytes();
            u64::from_ne_bytes([
                first[0], first[1], first[2], first[3], second[0], second[1], second[2], second[3],
            ])
        };
        copy_small_pattern32b(data, src, dst, length, pattern);
    } else if dist >= length {
        // no overlap
        data.copy_within(src..src + index(length), dst);
    } else {
        for i in 0..index(length) {
            data[dst + i] = data[src + i];
        }
    }
}

//------------------------------------------------------------------------------

// upstream: src/dec/vp8l_dec.c DecodeAlphaData
/// Decodes the 8-bit alpha indexes into `data` up to `last_row`, extracting the
/// finished row blocks into the alpha plane.
fn decode_alpha_data(
    dec: &mut VP8LDecoder,
    data: &mut [u8],
    input: &[u8],
    width: i32,
    height: i32,
    last_row: i32,
    alph: &mut AlphaOutput<'_>,
) -> bool {
    let mut ok = true;
    let mut row = dec.last_pixel / width;
    let mut col = dec.last_pixel % width;
    let mut pos = dec.last_pixel; // current position
    let end = width * height; // End of data
    let last = width * last_row; // Last pixel to decode
    let len_code_limit = NUM_LITERAL_CODES + NUM_LENGTH_CODES;
    let mask = dec.hdr.huffman_mask;
    let mut htree_group = if pos < last {
        Some(get_htree_group_for_pos(&dec.hdr, col, row))
    } else {
        None
    };
    debug_assert!(pos <= end);
    debug_assert!(last_row <= height);
    debug_assert!(is8b_optimizable(&dec.hdr));

    'end: {
        while !dec.br.eos && pos < last {
            // Only update when changing tile.
            if (col & mask) == 0 {
                htree_group = Some(get_htree_group_for_pos(&dec.hdr, col, row));
            }
            let group = htree_group.expect("a tree group is selected");
            vp8l_fill_bit_window(&mut dec.br, input);
            let htrees = dec.hdr.htree_groups[group].htrees;
            let code = read_symbol(
                htrees[HuffIndex::GREEN.index()],
                &dec.hdr.huffman_tables,
                &mut dec.br,
            );
            if code < NUM_LITERAL_CODES {
                // Literal
                data[index(pos)] = u8::try_from(code).expect("a literal is a byte");
                pos += 1;
                col += 1;
                if col >= width {
                    col = 0;
                    row += 1;
                    if row <= last_row && (row % NUM_ARGB_CACHE_ROWS == 0) {
                        extract_paletted_alpha_rows(dec, data, row, alph);
                    }
                }
            } else if code < len_code_limit {
                // Backward reference
                let length_sym = code - NUM_LITERAL_CODES;
                let length = get_copy_length(length_sym, &mut dec.br, input);
                let dist_symbol = read_symbol(
                    htrees[HuffIndex::DIST.index()],
                    &dec.hdr.huffman_tables,
                    &mut dec.br,
                );
                vp8l_fill_bit_window(&mut dec.br, input);
                let dist_code = get_copy_distance(dist_symbol, &mut dec.br, input);
                let dist = plane_code_to_distance(width, dist_code);
                if pos >= dist && end - pos >= length {
                    copy_block8b(data, index(pos), dist, length);
                } else {
                    ok = false;
                    break 'end;
                }
                pos += length;
                col += length;
                while col >= width {
                    col -= width;
                    row += 1;
                    if row <= last_row && (row % NUM_ARGB_CACHE_ROWS == 0) {
                        extract_paletted_alpha_rows(dec, data, row, alph);
                    }
                }
                if pos < last && (col & mask) != 0 {
                    htree_group = Some(get_htree_group_for_pos(&dec.hdr, col, row));
                }
            } else {
                // Not reached
                ok = false;
                break 'end;
            }
            dec.br.eos = vp8l_is_end_of_stream(&dec.br);
        }
        // Process the remaining rows corresponding to last row-block.
        extract_paletted_alpha_rows(dec, data, if row > last_row { last_row } else { row }, alph);
    }

    // End:
    dec.br.eos = vp8l_is_end_of_stream(&dec.br);
    if !ok || (dec.br.eos && pos < end) {
        let error = if dec.br.eos {
            VP8StatusCode::VP8_STATUS_SUSPENDED
        } else {
            VP8StatusCode::VP8_STATUS_BITSTREAM_ERROR
        };
        let _ = vp8l_set_error(dec, error);
        return false;
    }
    dec.last_pixel = pos;
    ok
}

// upstream: src/dec/vp8l_dec.c SaveState
/// Saves the bit reader, `last_pixel` and the color cache as a check-point.
fn save_state(dec: &mut VP8LDecoder, last_pixel: i32) {
    debug_assert!(dec.incremental);
    dec.saved_br = copy_bit_reader(&dec.br);
    dec.saved_last_pixel = last_pixel;
    if dec.hdr.color_cache_size > 0 {
        vp8l_color_cache_copy(&dec.hdr.color_cache, &mut dec.hdr.saved_color_cache);
    }
}

// upstream: src/dec/vp8l_dec.c RestoreState
/// Returns to the last check-point and records `VP8_STATUS_SUSPENDED`.
fn restore_state(dec: &mut VP8LDecoder) {
    debug_assert!(dec.br.eos);
    dec.status = VP8StatusCode::VP8_STATUS_SUSPENDED;
    dec.br = copy_bit_reader(&dec.saved_br);
    dec.last_pixel = dec.saved_last_pixel;
    if dec.hdr.color_cache_size > 0 {
        vp8l_color_cache_copy(&dec.hdr.saved_color_cache, &mut dec.hdr.color_cache);
    }
}

// upstream: src/dec/vp8l_dec.c SYNC_EVERY_N_ROWS
/// Minimum number of rows between check-points.
const SYNC_EVERY_N_ROWS: i32 = 8;

// upstream: src/dec/vp8l_dec.c DecodeImageData
/// Decodes the entropy-coded pixels of the current image level into `data` up
/// to `last_row`, calling `process_func` for each finished block of rows.
#[expect(
    clippy::too_many_lines,
    reason = "mirrors DecodeImageData, whose jumps share one loop body"
)]
fn decode_image_data(
    dec: &mut VP8LDecoder,
    data: &mut [u32],
    input: &[u8],
    width: i32,
    height: i32,
    last_row: i32,
    mut process_func: Option<ProcessRowsFunc<'_>>,
) -> bool {
    let mut row = dec.last_pixel / width;
    let mut col = dec.last_pixel % width;
    let mut src = index(dec.last_pixel);
    let mut last_cached = src;
    let src_end = index(width * height); // End of data
    let src_last = index(width * last_row); // Last pixel to decode
    let len_code_limit = NUM_LITERAL_CODES + NUM_LENGTH_CODES;
    let color_cache_limit = len_code_limit + dec.hdr.color_cache_size;
    let mut next_sync_row = if dec.incremental { row } else { 1 << 24 };
    let has_color_cache = dec.hdr.color_cache_size > 0;
    let mask = dec.hdr.huffman_mask;
    let mut htree_group = if src < src_last {
        Some(get_htree_group_for_pos(&dec.hdr, col, row))
    } else {
        None
    };
    debug_assert!(dec.last_row < last_row);
    debug_assert!(src_last <= src_end);

    let mut error = false;
    'decode: while src < src_last {
        let code;
        if row >= next_sync_row {
            save_state(
                dec,
                i32::try_from(src).expect("a pixel position fits an int"),
            );
            next_sync_row = row + SYNC_EVERY_N_ROWS;
        }
        // Only update when changing tile. Note we could use this test:
        // if "((((prev_col ^ col) | prev_row ^ row)) > mask)" -> tile changed
        // but that's actually slower and needs storing the previous col/row.
        if (col & mask) == 0 {
            htree_group = Some(get_htree_group_for_pos(&dec.hdr, col, row));
        }
        let group = htree_group.expect("a tree group is selected");
        // C jumps to AdvanceByOne; the flag stands for that label.
        let advance_by_one = 'step: {
            let current = &dec.hdr.htree_groups[group];
            if current.is_trivial_code {
                data[src] = current.literal_arb;
                break 'step true;
            }
            vp8l_fill_bit_window(&mut dec.br, input);
            if current.use_packed_table {
                code = read_packed_symbols(current, &mut dec.br, &mut data[src]);
                if vp8l_is_end_of_stream(&dec.br) {
                    break 'decode;
                }
                if code == PACKED_NON_LITERAL_CODE {
                    break 'step true;
                }
            } else {
                code = read_symbol(
                    current.htrees[HuffIndex::GREEN.index()],
                    &dec.hdr.huffman_tables,
                    &mut dec.br,
                );
            }
            if vp8l_is_end_of_stream(&dec.br) {
                break 'decode;
            }
            if code < NUM_LITERAL_CODES {
                // Literal
                if current.is_trivial_literal {
                    data[src] = current.literal_arb | (code << 8).cast_unsigned();
                } else {
                    let htrees = current.htrees;
                    let tables = &dec.hdr.huffman_tables;
                    let red = read_symbol(htrees[HuffIndex::RED.index()], tables, &mut dec.br);
                    vp8l_fill_bit_window(&mut dec.br, input);
                    let blue = read_symbol(htrees[HuffIndex::BLUE.index()], tables, &mut dec.br);
                    let alpha = read_symbol(htrees[HuffIndex::ALPHA.index()], tables, &mut dec.br);
                    if vp8l_is_end_of_stream(&dec.br) {
                        break 'decode;
                    }
                    data[src] = (alpha.cast_unsigned() << 24)
                        | (red << 16).cast_unsigned()
                        | (code << 8).cast_unsigned()
                        | blue.cast_unsigned();
                }
                break 'step true;
            } else if code < len_code_limit {
                // Backward reference
                let htrees = current.htrees;
                let length_sym = code - NUM_LITERAL_CODES;
                let length = get_copy_length(length_sym, &mut dec.br, input);
                let dist_symbol = read_symbol(
                    htrees[HuffIndex::DIST.index()],
                    &dec.hdr.huffman_tables,
                    &mut dec.br,
                );
                vp8l_fill_bit_window(&mut dec.br, input);
                let dist_code = get_copy_distance(dist_symbol, &mut dec.br, input);
                let dist = plane_code_to_distance(width, dist_code);

                if vp8l_is_end_of_stream(&dec.br) {
                    break 'decode;
                }
                if src < index(dist) || src_end - src < index(length) {
                    error = true;
                    break 'decode;
                }
                copy_block32b(data, src, dist, length);
                src += index(length);
                col += length;
                while col >= width {
                    col -= width;
                    row += 1;
                    if let Some(process_func) = process_func.as_mut()
                        && row <= last_row
                        && (row % NUM_ARGB_CACHE_ROWS == 0)
                    {
                        process_func(dec, data, row);
                    }
                }
                // Because of the check done above (before 'src' was incremented by
                // 'length'), the following holds true.
                debug_assert!(src <= src_end);
                if (col & mask) != 0 {
                    htree_group = Some(get_htree_group_for_pos(&dec.hdr, col, row));
                }
                if has_color_cache {
                    while last_cached < src {
                        vp8l_color_cache_insert(&mut dec.hdr.color_cache, data[last_cached]);
                        last_cached += 1;
                    }
                }
                break 'step false;
            } else if code < color_cache_limit {
                // Color cache
                let key = code - len_code_limit;
                debug_assert!(has_color_cache);
                while last_cached < src {
                    vp8l_color_cache_insert(&mut dec.hdr.color_cache, data[last_cached]);
                    last_cached += 1;
                }
                data[src] = vp8l_color_cache_lookup(&dec.hdr.color_cache, key.cast_unsigned());
                break 'step true;
            }
            // Not reached
            error = true;
            break 'decode;
        };
        if advance_by_one {
            // AdvanceByOne:
            src += 1;
            col += 1;
            if col >= width {
                col = 0;
                row += 1;
                if let Some(process_func) = process_func.as_mut()
                    && row <= last_row
                    && (row % NUM_ARGB_CACHE_ROWS == 0)
                {
                    process_func(dec, data, row);
                }
                if has_color_cache {
                    while last_cached < src {
                        vp8l_color_cache_insert(&mut dec.hdr.color_cache, data[last_cached]);
                        last_cached += 1;
                    }
                }
            }
        }
    }

    if !error {
        dec.br.eos = vp8l_is_end_of_stream(&dec.br);
        // In incremental decoding:
        // br->eos && src < src_last: if 'br' reached the end of the buffer and
        // 'src_last' has not been reached yet, there is not enough data. 'dec' has to
        // be reset until there is more data.
        // !br->eos && src < src_last: this cannot happen as either the buffer is
        // fully read, either enough has been read to reach 'src_last'.
        // src >= src_last: 'src_last' is reached, all is fine. 'src' can actually go
        // beyond 'src_last' in case the image is cropped and an LZ77 goes further.
        // The buffer might have been enough or there is some left. 'br->eos' does
        // not matter.
        #[expect(clippy::nonminimal_bool, reason = "the assertion of the C source")]
        {
            debug_assert!(!dec.incremental || (dec.br.eos && src < src_last) || src >= src_last);
        }
        if dec.incremental && dec.br.eos && src < src_last {
            restore_state(dec);
            return true;
        } else if (dec.incremental && src >= src_last) || !dec.br.eos {
            // Process the remaining rows corresponding to last row-block.
            if let Some(process_func) = process_func.as_mut() {
                process_func(dec, data, if row > last_row { last_row } else { row });
            }
            dec.status = VP8StatusCode::VP8_STATUS_OK;
            dec.last_pixel = i32::try_from(src).expect("a pixel position fits an int"); // end-of-scan marker
            return true;
        }
        // if not incremental, and we are past the end of buffer (eos=1), then this
        // is a real bitstream error.
    }

    // Error:
    let _ = vp8l_set_error(dec, VP8StatusCode::VP8_STATUS_BITSTREAM_ERROR);
    false
}

// -----------------------------------------------------------------------------
// VP8LTransform

// upstream: src/dec/vp8l_dec.c ClearTransform
/// Frees the data of a transform.
fn clear_transform(transform: &mut VP8LTransform) {
    transform.data = Vec::new();
}

// upstream: src/dec/vp8l_dec.c ExpandColorMap
/// For security reason, we need to remap the color map to span the total
/// possible bundled values, and not just the `num_colors`. The color map is
/// delta-coded per byte, as C reads it through a `uint8_t` cast.
fn expand_color_map(num_colors: i32, transform: &mut VP8LTransform) -> bool {
    let final_num_colors = 1_i32 << (8 >> transform.bits);
    let Some(mut new_color_map) = webp_safe_malloc::<u32>(
        u64::from(final_num_colors.cast_unsigned()),
        size_of::<u32>(),
    ) else {
        return false;
    };
    let data: Vec<u8> = transform
        .data
        .iter()
        .flat_map(|pixel| pixel.to_ne_bytes())
        .collect();
    let mut new_data = vec![0_u8; 4 * index(final_num_colors)];
    new_data[..4].copy_from_slice(&transform.data[0].to_ne_bytes());
    let mut i = 4;
    while i < 4 * index(num_colors) {
        // Equivalent to VP8LAddPixels(), on a byte-basis.
        new_data[i] = data[i].wrapping_add(new_data[i - 4]);
        i += 1;
    }
    while i < 4 * index(final_num_colors) {
        new_data[i] = 0; // black tail.
        i += 1;
    }
    new_color_map.extend(
        new_data
            .as_chunks::<4>()
            .0
            .iter()
            .map(|pixel| u32::from_ne_bytes(*pixel)),
    );
    transform.data = new_color_map;
    true
}

// upstream: src/dec/vp8l_dec.c ReadTransform
/// Reads one transform and its data. A repeated transform type fails without
/// recording an error, as in C.
#[expect(
    clippy::similar_names,
    reason = "the C names transform->xsize and transform->ysize"
)]
fn read_transform(xsize: &mut i32, ysize: i32, dec: &mut VP8LDecoder, data: &[u8]) -> bool {
    let mut ok = true;
    let r#type = VP8LImageTransformType(vp8l_read_bits(&mut dec.br, data, 2));

    // Each transform type can only be present once in the stream.
    if dec.transforms_seen & (1_u32 << r#type.0) != 0 {
        return false; // Already there, let's not accept the second same transform.
    }
    dec.transforms_seen |= 1_u32 << r#type.0;

    let current = index(dec.next_transform);
    {
        let transform = &mut dec.transforms[current];
        transform.r#type = r#type;
        transform.xsize = *xsize;
        transform.ysize = ysize;
        transform.data = Vec::new();
    }
    dec.next_transform += 1;
    debug_assert!(dec.next_transform <= NUM_TRANSFORMS);

    match r#type {
        VP8LImageTransformType::PREDICTOR_TRANSFORM
        | VP8LImageTransformType::CROSS_COLOR_TRANSFORM => {
            let bits = MIN_TRANSFORM_BITS + read_bits(&mut dec.br, data, NUM_TRANSFORM_BITS);
            dec.transforms[current].bits = bits;
            let transform_xsize = dec.transforms[current].xsize;
            let transform_ysize = dec.transforms[current].ysize;
            let mut transform_data = Vec::new();
            ok = decode_image_stream(
                sub_sample_size(transform_xsize, bits),
                sub_sample_size(transform_ysize, bits),
                /*is_level0=*/ false,
                dec,
                data,
                Some(&mut transform_data),
            );
            dec.transforms[current].data = transform_data;
        }
        VP8LImageTransformType::COLOR_INDEXING_TRANSFORM => {
            let num_colors = read_bits(&mut dec.br, data, 8) + 1;
            let bits = if num_colors > 16 {
                0
            } else if num_colors > 4 {
                1
            } else if num_colors > 2 {
                2
            } else {
                3
            };
            *xsize = sub_sample_size(dec.transforms[current].xsize, bits);
            dec.transforms[current].bits = bits;
            let mut transform_data = Vec::new();
            ok = decode_image_stream(
                num_colors,
                /*ysize=*/ 1,
                /*is_level0=*/ false,
                dec,
                data,
                Some(&mut transform_data),
            );
            dec.transforms[current].data = transform_data;
            if ok && !expand_color_map(num_colors, &mut dec.transforms[current]) {
                let _ = vp8l_set_error(dec, VP8StatusCode::VP8_STATUS_OUT_OF_MEMORY);
                return false;
            }
        }
        VP8LImageTransformType::SUBTRACT_GREEN_TRANSFORM => {}
        _ => debug_assert!(false, "can't happen"),
    }

    ok
}

// -----------------------------------------------------------------------------
// VP8LMetadata

// upstream: src/dec/vp8l_dec.c InitMetadata
/// Zeroes the metadata.
fn init_metadata(hdr: &mut VP8LMetadata) {
    *hdr = VP8LMetadata::default();
}

// upstream: src/dec/vp8l_dec.c ClearMetadata
/// Frees the metadata and zeroes it.
fn clear_metadata(hdr: &mut VP8LMetadata) {
    hdr.huffman_image = Vec::new();
    vp8l_huffman_tables_deallocate(&mut hdr.huffman_tables);
    hdr.htree_groups = Vec::new();
    vp8l_color_cache_clear(&mut hdr.color_cache);
    vp8l_color_cache_clear(&mut hdr.saved_color_cache);
    init_metadata(hdr);
}

// -----------------------------------------------------------------------------
// VP8LDecoder

// upstream: src/dec/vp8l_dec.c VP8LNew
/// Allocates and initialize a new lossless decoder instance. C's
/// `VP8LDspInit()` only assigns portable pointers and is omitted (ADR 0003).
#[must_use]
pub fn vp8l_new() -> Box<VP8LDecoder> {
    let mut dec = Box::<VP8LDecoder>::default();
    dec.status = VP8StatusCode::VP8_STATUS_OK;
    dec.state = VP8LDecodeState::READ_DIM;

    // upstream: src/dsp/lossless.c VP8LDspInit (omitted, ADR 0003)

    dec
}

// upstream: src/dec/vp8l_dec.c VP8LClear
/// Resets the decoder in its initial state, reclaiming memory. Preserves the
/// `status` value, and, as in C, `state` and `argb_cache`.
fn vp8l_clear(dec: &mut VP8LDecoder) {
    clear_metadata(&mut dec.hdr);

    dec.pixels = Vec::new();
    dec.pixels_8b = Vec::new();
    for transform in &mut dec.transforms[..index(dec.next_transform)] {
        clear_transform(transform);
    }
    dec.next_transform = 0;
    dec.transforms_seen = 0;

    dec.rescaler = None;
    dec.rescaled_row = Vec::new();

    // C also sets dec->output to NULL: leave no trace behind. The output is not
    // stored here.
}

// upstream: src/dec/vp8l_dec.c UpdateDecoder
/// Sets the dimensions of the current image level and its Huffman tiling.
fn update_decoder(dec: &mut VP8LDecoder, width: i32, height: i32) {
    let num_bits = dec.hdr.huffman_subsample_bits;
    dec.width = width;
    dec.height = height;

    dec.hdr.huffman_xsize = sub_sample_size(width, num_bits);
    dec.hdr.huffman_mask = if num_bits == 0 {
        !0
    } else {
        (1 << num_bits) - 1
    };
}

// upstream: src/dec/vp8l_dec.c DecodeImageStream
/// Reads an image level: at level 0 its transforms, color cache and Huffman
/// codes; at other levels also its pixels, which go to `decoded_data` (C:
/// `decoded_data` is `NULL` exactly at level 0).
#[expect(
    clippy::similar_names,
    reason = "the C names transform_xsize and transform_ysize"
)]
fn decode_image_stream(
    xsize: i32,
    ysize: i32,
    is_level0: bool,
    dec: &mut VP8LDecoder,
    input: &[u8],
    decoded_data: Option<&mut Vec<u32>>,
) -> bool {
    let mut ok = true;
    let mut transform_xsize = xsize;
    let transform_ysize = ysize;
    let mut data: Vec<u32> = Vec::new();
    let mut color_cache_bits = 0;

    'end: {
        // Read the transforms (may recurse).
        if is_level0 {
            while ok && read_bits(&mut dec.br, input, 1) != 0 {
                ok = read_transform(&mut transform_xsize, transform_ysize, dec, input);
            }
        }

        // Color cache
        if ok && read_bits(&mut dec.br, input, 1) != 0 {
            color_cache_bits = read_bits(&mut dec.br, input, 4);
            ok = (1..=MAX_CACHE_BITS).contains(&color_cache_bits);
            if !ok {
                let _ = vp8l_set_error(dec, VP8StatusCode::VP8_STATUS_BITSTREAM_ERROR);
                break 'end;
            }
        }

        // Read the Huffman codes (may recurse).
        ok = ok
            && read_huffman_codes(
                dec,
                input,
                transform_xsize,
                transform_ysize,
                color_cache_bits,
                is_level0,
            );
        if !ok {
            let _ = vp8l_set_error(dec, VP8StatusCode::VP8_STATUS_BITSTREAM_ERROR);
            break 'end;
        }

        // Finish setting up the color-cache
        if color_cache_bits > 0 {
            dec.hdr.color_cache_size = 1 << color_cache_bits;
            if !vp8l_color_cache_init(&mut dec.hdr.color_cache, color_cache_bits) {
                ok = false;
                let _ = vp8l_set_error(dec, VP8StatusCode::VP8_STATUS_OUT_OF_MEMORY);
                break 'end;
            }
        } else {
            dec.hdr.color_cache_size = 0;
        }
        update_decoder(dec, transform_xsize, transform_ysize);

        if is_level0 {
            // level 0 complete
            dec.state = VP8LDecodeState::READ_HDR;
            break 'end;
        }

        {
            let total_size = i64::from(transform_xsize).cast_unsigned()
                * i64::from(transform_ysize).cast_unsigned();
            let Some(mut pixels) = webp_safe_malloc::<u32>(total_size, size_of::<u32>()) else {
                ok = false;
                let _ = vp8l_set_error(dec, VP8StatusCode::VP8_STATUS_OUT_OF_MEMORY);
                break 'end;
            };
            pixels.resize(
                usize::try_from(total_size).expect("an accepted size fits"),
                0,
            );
            data = pixels;
        }

        // Use the Huffman trees to decode the LZ77 encoded data.
        ok = decode_image_data(
            dec,
            &mut data,
            input,
            transform_xsize,
            transform_ysize,
            transform_ysize,
            None,
        );
        ok = ok && !dec.br.eos;
    }

    // End:
    if ok {
        if let Some(decoded_data) = decoded_data {
            *decoded_data = data;
        } else {
            // We allocate image data in this function only for transforms. At level 0
            // (that is: not the transforms), we shouldn't have allocated anything.
            debug_assert_eq!(data.len(), 0);
            debug_assert!(is_level0);
        }
        dec.last_pixel = 0; // Reset for future DECODE_DATA_FUNC() calls.
        if !is_level0 {
            clear_metadata(&mut dec.hdr); // Clean up temporary data behind.
        }
    } else {
        drop(data);
        clear_metadata(&mut dec.hdr);
    }
    ok
}

//------------------------------------------------------------------------------

// upstream: src/dec/vp8l_dec.c AllocateInternalBuffers32b
/// Allocates the internal buffers `pixels` and `argb_cache`, in one buffer as in
/// C.
fn allocate_internal_buffers32b(dec: &mut VP8LDecoder, final_width: i32) -> bool {
    let num_pixels = i64::from(dec.width).cast_unsigned() * i64::from(dec.height).cast_unsigned();
    // Scratch buffer corresponding to top-prediction row for transforming the
    // first row in the row-blocks. Not needed for paletted alpha.
    #[expect(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        reason = "the C cast to uint16_t"
    )]
    let cache_top_pixels = u64::from(final_width as u16);
    // Scratch buffer for temporary BGRA storage. Not needed for paletted alpha.
    let cache_pixels =
        i64::from(final_width).cast_unsigned() * u64::from(NUM_ARGB_CACHE_ROWS.cast_unsigned());
    let total_num_pixels = num_pixels + cache_top_pixels + cache_pixels;

    debug_assert!(dec.width <= final_width);
    let Some(mut pixels) = webp_safe_malloc::<u32>(total_num_pixels, size_of::<u32>()) else {
        dec.argb_cache = None; // for soundness
        let _ = vp8l_set_error(dec, VP8StatusCode::VP8_STATUS_OUT_OF_MEMORY);
        return false;
    };
    pixels.resize(
        usize::try_from(total_num_pixels).expect("an accepted size fits"),
        0,
    );
    dec.pixels = pixels;
    dec.argb_cache =
        Some(usize::try_from(num_pixels + cache_top_pixels).expect("an accepted size fits"));
    true
}

// upstream: src/dec/vp8l_dec.c AllocateInternalBuffers8b
/// Allocates the 8-bit internal buffer of the paletted alpha path.
fn allocate_internal_buffers8b(dec: &mut VP8LDecoder) -> bool {
    let total_num_pixels =
        i64::from(dec.width).cast_unsigned() * i64::from(dec.height).cast_unsigned();
    dec.argb_cache = None; // for soundness
    let Some(mut pixels) = webp_safe_malloc::<u8>(total_num_pixels, size_of::<u8>()) else {
        let _ = vp8l_set_error(dec, VP8StatusCode::VP8_STATUS_OUT_OF_MEMORY);
        return false;
    };
    pixels.resize(
        usize::try_from(total_num_pixels).expect("an accepted size fits"),
        0,
    );
    dec.pixels_8b = pixels;
    true
}

//------------------------------------------------------------------------------

// upstream: src/dec/vp8l_dec.c ExtractAlphaRows
/// Special row-processing that only stores the alpha data.
fn extract_alpha_rows(
    dec: &mut VP8LDecoder,
    data: &mut [u32],
    last_row: i32,
    alph: &mut AlphaOutput<'_>,
) {
    let mut cur_row = dec.last_row;
    let mut num_rows = last_row - cur_row;
    let mut input = index(dec.width * cur_row);

    debug_assert!(last_row <= alph.io.crop_bottom);
    while num_rows > 0 {
        let num_rows_to_process = if num_rows > NUM_ARGB_CACHE_ROWS {
            NUM_ARGB_CACHE_ROWS
        } else {
            num_rows
        };
        // Extract alpha (which is stored in the green plane).
        let width = alph.io.width; // the final width (!= dec->width)
        let cache_pixs = width * num_rows_to_process;
        let dst = index(width * cur_row);
        let src = dec.argb_cache.expect("argb_cache is allocated");
        apply_inverse_transforms(dec, data, cur_row, num_rows_to_process, input);
        webp_extract_green(&data[src..], &mut alph.output[dst..], cache_pixs);
        alpha_apply_filter(alph, cur_row, cur_row + num_rows_to_process, dst, width);
        num_rows -= num_rows_to_process;
        input += index(num_rows_to_process * dec.width);
        cur_row += num_rows_to_process;
    }
    debug_assert_eq!(cur_row, last_row);
    dec.last_row = last_row;
    dec.last_out_row = last_row;
}

// upstream: src/dec/vp8l_dec.c VP8LDecodeAlphaHeader
/// Decodes image header for alpha data stored using lossless compression.
/// `data` is the lossless stream after the alpha header byte; the VP8 decoder
/// passes the same bytes to [`vp8l_decode_alpha_image_stream`]. Returns false in
/// case of error; the lossless decoder is then dropped and `alph_dec.vp8l_dec`
/// stays `None`, as in C.
#[must_use]
pub fn vp8l_decode_alpha_header(alph_dec: &mut ALPHDecoder, data: &[u8]) -> bool {
    let mut dec = vp8l_new();

    dec.width = alph_dec.width;
    dec.height = alph_dec.height;
    // C points dec->io at alph_dec->io and alph_dec->io.opaque at alph_dec; the
    // alpha functions take alph_dec instead (ADR 0003).
    alph_dec.io.width = alph_dec.width;
    alph_dec.io.height = alph_dec.height;

    dec.status = VP8StatusCode::VP8_STATUS_OK;
    vp8l_init_bit_reader(&mut dec.br, data);

    if !decode_image_stream(
        alph_dec.width,
        alph_dec.height,
        /*is_level0=*/ true,
        &mut dec,
        data,
        /*decoded_data=*/ None,
    ) {
        return false;
    }

    // Special case: if alpha data uses only the color indexing transform and
    // doesn't use color cache (a frequent case), we will use DecodeAlphaData()
    // method that only needs allocation of 1 byte per pixel (alpha channel).
    let ok = if dec.next_transform == 1
        && dec.transforms[0].r#type == VP8LImageTransformType::COLOR_INDEXING_TRANSFORM
        && is8b_optimizable(&dec.hdr)
    {
        alph_dec.use_8b_decode = true;
        allocate_internal_buffers8b(&mut dec)
    } else {
        // Allocate internal buffers (note that dec->width may have changed here).
        alph_dec.use_8b_decode = false;
        allocate_internal_buffers32b(&mut dec, alph_dec.width)
    };

    if !ok {
        return false;
    }

    // Only set here, once we are sure it is valid (to avoid thread races).
    alph_dec.vp8l_dec = Some(dec);
    true
}

// upstream: src/dec/vp8l_dec.c VP8LDecodeAlphaImageStream
/// Decodes *at least* `last_row` rows of alpha into `alpha_plane`, the VP8
/// decoder's alpha plane (C: `alph_dec->output`). If some of the initial rows
/// are already decoded in previous call(s), it will resume decoding from where
/// it was paused. `data` is the input passed to [`vp8l_decode_alpha_header`].
/// Returns the recorded status in case of bitstream error.
///
/// # Errors
///
/// Returns the status of the lossless decoder when decoding fails.
///
/// # Panics
///
/// Panics when `alph_dec` holds no lossless decoder, which C asserts.
pub fn vp8l_decode_alpha_image_stream(
    alph_dec: &mut ALPHDecoder,
    last_row: i32,
    data: &[u8],
    alpha_plane: &mut [u8],
) -> Result<(), VP8StatusCode> {
    let ALPHDecoder {
        vp8l_dec,
        io,
        filter,
        use_8b_decode,
        prev_line,
        ..
    } = alph_dec;
    let dec = vp8l_dec
        .as_deref_mut()
        .expect("the alpha header is decoded");
    debug_assert!(last_row <= dec.height);

    if dec.last_row >= last_row {
        return Ok(()); // done
    }

    // upstream: src/dsp/alpha_processing.c WebPInitAlphaProcessing (omitted, ADR 0003)

    let mut alph = AlphaOutput {
        io,
        filter: *filter,
        prev_line,
        output: alpha_plane,
    };
    // Decode (with special row processing).
    let (width, height) = (dec.width, dec.height);
    let ok = if *use_8b_decode {
        let mut pixels = std::mem::take(&mut dec.pixels_8b);
        let ok = decode_alpha_data(dec, &mut pixels, data, width, height, last_row, &mut alph);
        dec.pixels_8b = pixels;
        ok
    } else {
        let mut pixels = std::mem::take(&mut dec.pixels);
        let mut process = |dec: &mut VP8LDecoder, pixels: &mut [u32], row: i32| {
            extract_alpha_rows(dec, pixels, row, &mut alph);
        };
        let ok = decode_image_data(
            dec,
            &mut pixels,
            data,
            width,
            height,
            last_row,
            Some(&mut process),
        );
        dec.pixels = pixels;
        ok
    };
    if ok { Ok(()) } else { Err(dec.status) }
}

//------------------------------------------------------------------------------

// upstream: src/dec/vp8l_dec.c VP8LDecodeHeader
/// Decodes the image header from `data`, the input that C reads through
/// `io->data` and `io->data_size`, and sets `io.width` and `io.height`.
///
/// # Errors
///
/// Returns the recorded status, never `VP8_STATUS_OK`, after clearing the
/// decoder as C does.
pub fn vp8l_decode_header(
    dec: &mut VP8LDecoder,
    io: &mut VP8Io,
    data: &[u8],
) -> Result<(), VP8StatusCode> {
    let mut width = 0;
    let mut height = 0;
    let mut has_alpha = 0;

    // C's `dec == NULL` and `io == NULL` checks exist only at the C ABI.
    // C also sets dec->io = io; the VP8Io is passed per call (ADR 0003).
    dec.status = VP8StatusCode::VP8_STATUS_OK;
    vp8l_init_bit_reader(&mut dec.br, data);
    'error: {
        if !read_image_info(&mut dec.br, data, &mut width, &mut height, &mut has_alpha) {
            let _ = vp8l_set_error(dec, VP8StatusCode::VP8_STATUS_BITSTREAM_ERROR);
            break 'error;
        }
        dec.state = VP8LDecodeState::READ_DIM;
        io.width = width;
        io.height = height;

        if !decode_image_stream(
            width, height, /*is_level0=*/ true, dec, data, /*decoded_data=*/ None,
        ) {
            break 'error;
        }
        return Ok(());
    }

    // Error:
    vp8l_clear(dec);
    debug_assert!(dec.status != VP8StatusCode::VP8_STATUS_OK);
    Err(dec.status)
}

// upstream: src/dec/vp8l_dec.c VP8LDecodeImage
/// Decodes an image. It's required to decode the lossless header before calling
/// this function. `data` is the input passed to [`vp8l_decode_header`], or the
/// same bytes at the same positions when the owner's buffer grew; `params` and
/// `output` are what C reads through `io->opaque`. The rows go to `output`,
/// and `params.last_y` receives the last output row.
///
/// # Errors
///
/// Returns the recorded status, never `VP8_STATUS_OK`, after clearing the
/// decoder as C does. A suspended incremental decoding succeeds with
/// `dec.status == VP8_STATUS_SUSPENDED`, as in C.
pub fn vp8l_decode_image(
    dec: &mut VP8LDecoder,
    io: &mut VP8Io,
    data: &[u8],
    params: &mut WebPDecParams<'_>,
    output: &mut WebPDecBuffer<'_>,
) -> Result<(), VP8StatusCode> {
    // C's `dec == NULL` check exists only at the C ABI.
    debug_assert_ne!(dec.hdr.huffman_tables.root.start.len(), 0);
    debug_assert_ne!(dec.hdr.htree_groups.len(), 0);
    debug_assert!(dec.hdr.num_htree_groups > 0);

    'err: {
        // Initialization.
        if dec.state != VP8LDecodeState::READ_DATA {
            // C sets dec->output = params->output; the output is passed per call.

            if !webp_io_init_from_options(params.options, io, WEBP_CSP_MODE::MODE_BGRA) {
                let _ = vp8l_set_error(dec, VP8StatusCode::VP8_STATUS_INVALID_PARAM);
                break 'err;
            }

            if !allocate_internal_buffers32b(dec, io.width) {
                break 'err;
            }

            if io.use_scaling && !allocate_and_init_rescaler(dec, io) {
                break 'err;
            }
            // upstream: src/dsp/alpha_processing.c WebPInitAlphaProcessing (omitted, ADR 0003)
            // upstream: src/dsp/yuv.c WebPInitConvertARGBToYUV (omitted, ADR 0003)
            if dec.incremental
                && dec.hdr.color_cache_size > 0
                && dec.hdr.saved_color_cache.colors.is_empty()
                && !vp8l_color_cache_init(
                    &mut dec.hdr.saved_color_cache,
                    dec.hdr.color_cache.hash_bits,
                )
            {
                let _ = vp8l_set_error(dec, VP8StatusCode::VP8_STATUS_OUT_OF_MEMORY);
                break 'err;
            }
            dec.state = VP8LDecodeState::READ_DATA;
        }

        // Decode.
        let mut pixels = std::mem::take(&mut dec.pixels);
        let (width, height) = (dec.width, dec.height);
        let crop_bottom = io.crop_bottom;
        let mut process = |dec: &mut VP8LDecoder, pixels: &mut [u32], row: i32| {
            process_rows(dec, pixels, row, io, output);
        };
        let ok = decode_image_data(
            dec,
            &mut pixels,
            data,
            width,
            height,
            crop_bottom,
            Some(&mut process),
        );
        dec.pixels = pixels;
        if !ok {
            break 'err;
        }

        params.last_y = dec.last_out_row;
        return Ok(());
    }

    // Err:
    vp8l_clear(dec);
    debug_assert!(dec.status != VP8StatusCode::VP8_STATUS_OK);
    Err(dec.status)
}
