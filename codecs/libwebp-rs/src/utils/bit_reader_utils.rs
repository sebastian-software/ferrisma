// libwebp v1.6.0, src/utils/bit_reader_utils.c:
//
// Copyright 2010 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// Boolean decoder non-inlined methods
//
// Author: Skal (pascal.massimino@gmail.com)

// libwebp v1.6.0, src/utils/bit_reader_utils.h:
//
// Copyright 2010 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// Boolean decoder
//
// Author: Skal (pascal.massimino@gmail.com)
//         Vikas Arora (vikaas.arora@gmail.com)

//! Translation of `src/utils/bit_reader_utils.c` and `src/utils/bit_reader_utils.h` from libwebp v1.6.0.
//!
//! Work package: WP2 (bit I/O).
//!
//! The readers hold no reference to their input (ADR 0003, as amended on
//! 2026-11-06). `VP8BitReader` and `VP8LBitReader` store state and positions:
//! every C pointer becomes a `usize` position into the input that the owner holds,
//! and the owner passes that input to each function that reads it. A C `buf` that
//! may be `NULL` becomes the flag `VP8BitReader::has_buf`.
//!
//! `VP8RemapBitReader` adds a signed offset to the three positions of a reader
//! with an input, as C adds it to the pointers. The owner calls it when the same
//! bytes have moved by that many indexes of its input.
//!
//! The bit-trace tool (`BITTRACE`) is off in the reference configuration, so the
//! labels of the reader functions are removed, as the header's macros do.
//!
//! # Deviations from upstream
//!
//! Changes of representation that the ADRs allow, or behavior that the Rust
//! side cannot reproduce exactly. Each records the reason.
//!
//! 1. **Public fields.** The fields of the four structs (`VP8BitReader`,
//!    `VP8LBitReader`, `VP8BitWriter` and `VP8LBitWriter`) are `pub`, as the C
//!    structs are public in their headers, so that the tests can read the
//!    state. The C field `buf_end` is stored as `VP8BitReader::buf_end`,
//!    because it no longer comes from a slice length; `VP8LBitReader::len` is
//!    stored for the same reason.
//! 2. **Remap by an index delta.** `VP8RemapBitReader` adds a signed offset to
//!    the positions. The owner passes the index delta of its bytes, not the
//!    pointer difference of two allocations, which C's caller computes. Both
//!    sides of the remap test move the input inside one allocation, so that the
//!    C pointer difference and the index delta are the same number. The test
//!    does not move the input to a second allocation, because a pointer
//!    difference between two allocations contains the distance of their bases,
//!    which an index delta does not.
//!
//! # Excluded undefined behavior
//!
//! Inputs that are undefined behavior (or assertion failures) in C and are
//! therefore excluded from the oracle comparison (PORTING rule 8). The
//! asan-ubsan oracle found these, and the generators keep the tests away from
//! them:
//!
//! 1. `VP8LBitReaderSetBuffer` with a length shorter than the reader's position
//!    is a parameter error (the comment of the function says so), and
//!    upstream's own debug assertion in `VP8LIsEndOfStream` fires there. The
//!    Rust debug assertion mirrors it. The tests substitute `fill` for such a
//!    call.
//! 2. `VP8LDoFillBitWindow` requires `bit_pos >= VP8L_WBITS` (debug assertion).
//!    The tests call it only there.
//! 3. `VP8LReadBits` with a negative count indexes `kBitMask` out of range. The
//!    Rust index panics in every profile, after the debug assertion; C reads
//!    outside the table. No test issues it.
//! 4. `VP8RemapBitReader` with a position that would become negative is outside
//!    the contract of the owner (the pointer would leave its allocation). The
//!    Rust version panics; no test issues it.
//!
//! # Checks not applicable
//!
//! These checks do not apply, or cannot be made, with the reason:
//!
//! 1. Layout of `repr(C)` mirrors. None of the four structs is exported by a
//!    public header; the C declarations are internal to `bit_reader_utils.h`
//!    and `bit_writer_utils.h`. ADR 0004 gives no ABI mirror to them.
//! 2. The other `BITS` values of the header (8, 16, 32 and 48) are selected by
//!    no target that upstream names, so they are not translated. `bit_t` and
//!    `lbit_t` follow the two selected values.

use crate::utils::bit_reader_inl_utils::{LBIT_BYTES, vp8_get_bit, vp8_load_new_bytes};
use crate::utils::endian_inl_utils::h_to_le32;
use crate::utils::utils::webp_mem_to_uint32;

// upstream: src/utils/bit_reader_utils.h BITS
/// Number of bits that the boolean decoder loads at once. Upstream chooses 56 on
/// x86-64, `AArch64` and WebAssembly, and 24 on every other target; this constant
/// makes the same choice for the Rust target architecture.
pub const BITS: i32 = if cfg!(any(
    target_arch = "x86_64",
    target_arch = "aarch64",
    target_arch = "wasm32",
    target_arch = "wasm64"
)) {
    56
} else {
    24
};

// upstream: src/utils/bit_reader_utils.h bit_t
/// Natural register type of the boolean decoder's `value`: 64 bits when
/// `BITS > 24`, and 32 bits otherwise.
#[cfg(any(
    target_arch = "x86_64",
    target_arch = "aarch64",
    target_arch = "wasm32",
    target_arch = "wasm64"
))]
#[expect(non_camel_case_types, reason = "C type name")]
pub type bit_t = u64;

// upstream: src/utils/bit_reader_utils.h bit_t
/// Natural register type of the boolean decoder's `value`: 64 bits when
/// `BITS > 24`, and 32 bits otherwise.
#[cfg(not(any(
    target_arch = "x86_64",
    target_arch = "aarch64",
    target_arch = "wasm32",
    target_arch = "wasm64"
)))]
#[expect(non_camel_case_types, reason = "C type name")]
pub type bit_t = u32;

// upstream: src/utils/bit_reader_utils.h range_t
/// Register for `range`, which holds eight significant bits.
#[expect(non_camel_case_types, reason = "C type name")]
pub type range_t = u32;

// upstream: src/utils/bit_reader_utils.h VP8BitReader
/// The boolean decoder: its value window, its range, and the positions of its
/// input. It holds no reference to the input; see the module documentation.
#[derive(Debug, Default)]
pub struct VP8BitReader {
    /// Current value; keeps `BITS + 8` bits.
    pub value: bit_t,
    /// Current range minus one, in `[127, 254]`.
    pub range: range_t,
    /// Number of valid bits left.
    pub bits: i32,
    /// Set once the reader has an input. This is the C test `buf != NULL`, which
    /// only a reader that was set up passes.
    pub has_buf: bool,
    /// Index in the input of the next byte to be read (C `buf`).
    pub buf: usize,
    /// Index in the input where the input ends (C `buf_end`).
    pub buf_end: usize,
    /// Index in the input of the limit of the packed reads (C `buf_max`).
    pub buf_max: usize,
    /// Set when the input is exhausted.
    pub eof: bool,
}

// upstream: src/utils/bit_reader_utils.c kVP8Log2Range
/// `kVP8Log2Range[range]` is the number of renormalization shifts for `range`.
pub const VP8_LOG2_RANGE: [u8; 128] = [
    7, 6, 6, 5, 5, 5, 5, 4, 4, 4, 4, 4, 4, 4, 4, //
    3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, //
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, //
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, //
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, //
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, //
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, //
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, //
    0,
];

// upstream: src/utils/bit_reader_utils.c kVP8NewRange
/// `kVP8NewRange[range]` is the renormalized range: `((range + 1) << shift) - 1`.
pub const VP8_NEW_RANGE: [u8; 128] = [
    127, 127, 191, 127, 159, 191, 223, 127, 143, 159, 175, 191, 207, 223, 239, 127, //
    135, 143, 151, 159, 167, 175, 183, 191, 199, 207, 215, 223, 231, 239, 247, 127, //
    131, 135, 139, 143, 147, 151, 155, 159, 163, 167, 171, 175, 179, 183, 187, 191, //
    195, 199, 203, 207, 211, 215, 219, 223, 227, 231, 235, 239, 243, 247, 251, 127, //
    129, 131, 133, 135, 137, 139, 141, 143, 145, 147, 149, 151, 153, 155, 157, 159, //
    161, 163, 165, 167, 169, 171, 173, 175, 177, 179, 181, 183, 185, 187, 189, 191, //
    193, 195, 197, 199, 201, 203, 205, 207, 209, 211, 213, 215, 217, 219, 221, 223, //
    225, 227, 229, 231, 233, 235, 237, 239, 241, 243, 245, 247, 249, 251, 253, 127,
];

// upstream: src/utils/bit_reader_utils.c VP8BitReaderSetBuffer
/// Gives the reader an input of `size` bytes, which the owner passes from now on,
/// and rewinds the reader to the first byte. The value, the range and the
/// end-of-input flag are not changed.
pub fn vp8_bit_reader_set_buffer(br: &mut VP8BitReader, size: usize) {
    br.has_buf = true;
    br.buf = 0;
    br.buf_end = size;
    br.buf_max = if size >= LBIT_BYTES {
        size + 1 - LBIT_BYTES
    } else {
        0
    };
}

// upstream: src/utils/bit_reader_utils.c VP8InitBitReader
/// Initializes the boolean decoder over `data`, which is the whole input of the
/// reader, and loads its first bytes.
pub fn vp8_init_bit_reader(br: &mut VP8BitReader, data: &[u8]) {
    debug_assert!(data.len() < (1 << 31)); // limit ensured by format and upstream checks
    br.range = 255 - 1;
    br.value = 0;
    br.bits = -8; // to load the very first 8bits
    br.eof = false;
    vp8_bit_reader_set_buffer(br, data.len());
    vp8_load_new_bytes(br, data);
}

// upstream: src/utils/bit_reader_utils.c VP8RemapBitReader
/// Adds `offset` to the positions of a reader that has an input, as C adds it to
/// the pointers. The owner calls it when the same bytes have moved by `offset`
/// indexes of its input. A reader without an input (C `buf == NULL`) is left as
/// it is.
///
/// # Panics
///
/// Panics when a position would become negative, which means that `offset` does
/// not describe a move of the input's bytes.
pub fn vp8_remap_bit_reader(br: &mut VP8BitReader, offset: isize) {
    if br.has_buf {
        br.buf = br
            .buf
            .checked_add_signed(offset)
            .expect("the remapped position stays in the input");
        br.buf_end = br
            .buf_end
            .checked_add_signed(offset)
            .expect("the remapped end stays in the input");
        br.buf_max = br
            .buf_max
            .checked_add_signed(offset)
            .expect("the remapped limit stays in the input");
    }
}

// upstream: src/utils/bit_reader_utils.c VP8LoadFinalBytes
/// Loads one byte at a time near the end of the input. Once the input is
/// exhausted, the first call sets `eof` and shifts in zeros, and later calls
/// only keep the state.
pub fn vp8_load_final_bytes(br: &mut VP8BitReader, data: &[u8]) {
    // Only read 8bits at a time
    if br.buf < br.buf_end {
        br.bits += 8;
        br.value = bit_t::from(data[br.buf]) | (br.value << 8);
        br.buf += 1;
    } else if !br.eof {
        br.value <<= 8;
        br.bits += 8;
        br.eof = true;
    } else {
        br.bits = 0; // This is to avoid undefined behaviour with shifts.
    }
}

// upstream: src/utils/bit_reader_utils.c VP8GetValue
/// Reads `num_bits` bits, most significant first, each with probability one
/// half.
#[expect(clippy::cast_sign_loss, reason = "each bit is 0 or 1")]
pub fn vp8_get_value(br: &mut VP8BitReader, data: &[u8], num_bits: i32) -> u32 {
    let mut v: u32 = 0;
    let mut bits = num_bits;
    while bits > 0 {
        bits -= 1;
        v |= (vp8_get_bit(br, data, 0x80) as u32) << bits;
    }
    v
}

// upstream: src/utils/bit_reader_utils.h VP8Get
/// Reads one bit with probability one half. The header's macro of the same name
/// expands to `VP8GetValue(br, 1)`.
pub fn vp8_get(br: &mut VP8BitReader, data: &[u8]) -> u32 {
    vp8_get_value(br, data, 1)
}

// upstream: src/utils/bit_reader_utils.c VP8GetSignedValue
/// Reads `num_bits` bits, then a sign bit that negates the value when set.
#[expect(clippy::cast_possible_wrap, reason = "C converts the uint32_t to int")]
pub fn vp8_get_signed_value(br: &mut VP8BitReader, data: &[u8], num_bits: i32) -> i32 {
    let value = vp8_get_value(br, data, num_bits) as i32;
    if vp8_get(br, data) != 0 {
        -value
    } else {
        value
    }
}

// upstream: src/utils/bit_reader_utils.h VP8L_LBITS
/// Number of bits prefetched by the lossless reader: the bit size of `vp8l_val_t`.
pub const VP8L_LBITS: i32 = 64;

// upstream: src/utils/bit_reader_utils.h VP8L_WBITS
/// Minimum number of bits that `VP8LFillBitWindow` makes available.
pub const VP8L_WBITS: i32 = 32;

// upstream: src/utils/bit_reader_utils.h VP8L_MAX_NUM_BIT_READ
/// Largest number of bits one `VP8LReadBits` call may read.
pub const VP8L_MAX_NUM_BIT_READ: i32 = 24;

// upstream: src/utils/bit_reader_utils.h vp8l_val_t
/// The prefetched bits of the lossless reader: the right register is 64 bits.
#[expect(non_camel_case_types, reason = "C type name")]
pub type vp8l_val_t = u64;

// upstream: src/utils/bit_reader_utils.c VP8L_LOG8_WBITS
/// Number of bytes needed to store `VP8L_WBITS` bits.
const VP8L_LOG8_WBITS: usize = 4;

// upstream: src/utils/bit_reader_utils.c VP8L_USE_FAST_LOAD
/// Whether `VP8LDoFillBitWindow` reads four bytes at once. Upstream defines the
/// macro for ARM, `AArch64`, x86, x86-64 and WebAssembly.
pub const VP8L_USE_FAST_LOAD: bool = cfg!(any(
    target_arch = "arm",
    target_arch = "aarch64",
    target_arch = "x86",
    target_arch = "x86_64",
    target_arch = "wasm32",
    target_arch = "wasm64",
));

// upstream: src/utils/bit_reader_utils.c kBitMask
/// `kBitMask[n]` is the mask of the low `n` bits.
const BIT_MASK: [u32; 25] = [
    0,
    0x0000_0001,
    0x0000_0003,
    0x0000_0007,
    0x0000_000f,
    0x0000_001f,
    0x0000_003f,
    0x0000_007f,
    0x0000_00ff,
    0x0000_01ff,
    0x0000_03ff,
    0x0000_07ff,
    0x0000_0fff,
    0x0000_1fff,
    0x0000_3fff,
    0x0000_7fff,
    0x0000_ffff,
    0x0001_ffff,
    0x0003_ffff,
    0x0007_ffff,
    0x000f_ffff,
    0x001f_ffff,
    0x003f_ffff,
    0x007f_ffff,
    0x00ff_ffff,
];

// upstream: src/utils/bit_reader_utils.h VP8LBitReader
/// The lossless bit reader: a window of prefetched bits and the positions of its
/// input. It holds no reference to the input; see the module documentation.
#[derive(Debug, Default)]
pub struct VP8LBitReader {
    /// Prefetched bits.
    pub val: vp8l_val_t,
    /// Length of the input (C `len`).
    pub len: usize,
    /// Byte position in the input of the next byte to be loaded.
    pub pos: usize,
    /// Current bit-reading position in `val`.
    pub bit_pos: i32,
    /// Set when a bit was read past the end of the input.
    pub eos: bool,
}

// upstream: src/utils/bit_reader_utils.c VP8LInitBitReader
/// Initializes the lossless reader over `data`, which is the whole input of the
/// reader, and prefetches its first eight bytes.
pub fn vp8l_init_bit_reader(br: &mut VP8LBitReader, data: &[u8]) {
    debug_assert!(data.len() < 0xffff_fff8); // can't happen with a RIFF chunk.
    let length = data.len().min(size_of::<vp8l_val_t>());
    let mut value: vp8l_val_t = 0;
    for (i, &byte) in data[..length].iter().enumerate() {
        value |= vp8l_val_t::from(byte) << (8 * i);
    }
    br.len = data.len();
    br.val = value;
    br.pos = length;
    br.bit_pos = 0;
    br.eos = false;
}

// upstream: src/utils/bit_reader_utils.c VP8LBitReaderSetBuffer
/// Sets a new input length, which the owner passes from now on. The position and
/// the prefetched bits are kept, and the end-of-stream flag is recomputed, so a
/// position past the new end sets `eos`.
pub fn vp8l_bit_reader_set_buffer(br: &mut VP8LBitReader, len: usize) {
    debug_assert!(len < 0xffff_fff8); // can't happen with a RIFF chunk.
    br.len = len;
    // 'pos' > 'len' should be considered a param error.
    br.eos = (br.pos > br.len) || vp8l_is_end_of_stream(br);
}

// upstream: src/utils/bit_reader_utils.c VP8LSetEndOfStream
/// Sets `eos` and clears the position, to avoid undefined shifts.
fn vp8l_set_end_of_stream(br: &mut VP8LBitReader) {
    br.eos = true;
    br.bit_pos = 0; // To avoid undefined behaviour with shifts.
}

// upstream: src/utils/bit_reader_utils.c ShiftBytes
/// Reloads up to `VP8L_LBITS` bits byte by byte, while the input lasts.
fn shift_bytes(br: &mut VP8LBitReader, data: &[u8]) {
    while br.bit_pos >= 8 && br.pos < br.len {
        br.val >>= 8;
        br.val |= vp8l_val_t::from(data[br.pos]) << (VP8L_LBITS - 8);
        br.pos += 1;
        br.bit_pos -= 8;
    }
    if vp8l_is_end_of_stream(br) {
        vp8l_set_end_of_stream(br);
    }
}

// upstream: src/utils/bit_reader_utils.c VP8LReadBits
/// Reads `n_bits` bits. Reads with more than `VP8L_MAX_NUM_BIT_READ` bits, or
/// after the end of the stream, return zero and set `eos`.
///
/// # Panics
///
/// Debug builds panic when `n_bits` is negative, which C leaves undefined.
#[expect(
    clippy::cast_sign_loss,
    reason = "n_bits is checked not to be negative"
)]
pub fn vp8l_read_bits(br: &mut VP8LBitReader, data: &[u8], n_bits: i32) -> u32 {
    debug_assert!(n_bits >= 0);
    // Flag an error if end_of_stream or n_bits is more than allowed limit.
    if !br.eos && n_bits <= VP8L_MAX_NUM_BIT_READ {
        let val = vp8l_prefetch_bits(br) & BIT_MASK[n_bits as usize];
        let new_bits = br.bit_pos + n_bits;
        br.bit_pos = new_bits;
        shift_bytes(br, data);
        val
    } else {
        vp8l_set_end_of_stream(br);
        0
    }
}

// upstream: src/utils/bit_reader_utils.h VP8LPrefetchBits
/// Returns the prefetched bits, so that they can be looked up.
#[must_use]
#[expect(clippy::cast_possible_truncation, reason = "C keeps the low 32 bits")]
pub fn vp8l_prefetch_bits(br: &VP8LBitReader) -> u32 {
    (br.val >> (br.bit_pos & (VP8L_LBITS - 1))) as u32
}

// upstream: src/utils/bit_reader_utils.h VP8LIsEndOfStream
/// Reports whether a bit was read past the end of the input. Does not set `eos`.
#[must_use]
pub fn vp8l_is_end_of_stream(br: &VP8LBitReader) -> bool {
    debug_assert!(br.pos <= br.len);
    br.eos || ((br.pos == br.len) && (br.bit_pos > VP8L_LBITS))
}

// upstream: src/utils/bit_reader_utils.h VP8LSetBitPos
/// Jumps to the bit position `val` of the prefetched bits. Does not set `eos`;
/// use with care.
pub fn vp8l_set_bit_pos(br: &mut VP8LBitReader, val: i32) {
    br.bit_pos = val;
}

// upstream: src/utils/bit_reader_utils.c VP8LDoFillBitWindow
/// Advances the input by four bytes to make room for the next 32 bits. With
/// `VP8L_USE_FAST_LOAD` it reads four bytes at once while nine or more remain;
/// otherwise, and near the end, it shifts in byte by byte.
///
/// # Panics
///
/// Debug builds panic when `bit_pos` is below `VP8L_WBITS`.
pub fn vp8l_do_fill_bit_window(br: &mut VP8LBitReader, data: &[u8]) {
    debug_assert!(br.bit_pos >= VP8L_WBITS);
    if VP8L_USE_FAST_LOAD && br.pos + size_of::<vp8l_val_t>() < br.len {
        br.val >>= VP8L_WBITS;
        br.bit_pos -= VP8L_WBITS;
        let word = h_to_le32(webp_mem_to_uint32(&data[br.pos..]));
        br.val |= vp8l_val_t::from(word) << (VP8L_LBITS - VP8L_WBITS);
        br.pos += VP8L_LOG8_WBITS;
        return;
    }
    shift_bytes(br, data); // Slow path.
}

// upstream: src/utils/bit_reader_utils.h VP8LFillBitWindow
/// Makes room for the next 32 bits when fewer than 32 bits are prefetched.
pub fn vp8l_fill_bit_window(br: &mut VP8LBitReader, data: &[u8]) {
    if br.bit_pos >= VP8L_WBITS {
        vp8l_do_fill_bit_window(br, data);
    }
}
