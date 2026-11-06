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
//! `VP8BitReader` and `VP8LBitReader` borrow their input, `data` and `buf`, as a
//! slice with a lifetime, and keep offsets into it where C keeps pointers (ADR
//! 0003). The C `buf_end` and `len` are the slice length.
//!
//! Not translated: `VP8RemapBitReader`. It moves a reader's pointers when its
//! owner relocates the buffer, which a borrowing reader cannot express. The
//! incremental decoder uses it, so the question is open; see
//! `docs/status/wp2.md`.
//!
//! The bit-trace tool (`BITTRACE`) is off in the reference configuration, so
//! the labels of the reader functions are removed, as the header's macros do.

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
/// The boolean decoder: its value window, its range and its input.
#[derive(Debug, Default)]
pub struct VP8BitReader<'a> {
    /// Current value; keeps `BITS + 8` bits.
    pub value: bit_t,
    /// Current range minus one, in `[127, 254]`.
    pub range: range_t,
    /// Number of valid bits left.
    pub bits: i32,
    /// The input: the bytes from `buf_start` to `buf_end` of the C reader.
    pub data: &'a [u8],
    /// Offset in `data` of the next byte to be read (C `buf`).
    pub buf: usize,
    /// Offset in `data` of the limit of the packed reads (C `buf_max`).
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
/// Sets the input of the reader to `start` and rewinds it to its first byte. The
/// value, the range and the end-of-input flag are not changed.
pub fn vp8_bit_reader_set_buffer<'a>(br: &mut VP8BitReader<'a>, start: &'a [u8]) {
    br.data = start;
    br.buf = 0;
    br.buf_max = if start.len() >= LBIT_BYTES {
        start.len() + 1 - LBIT_BYTES
    } else {
        0
    };
}

// upstream: src/utils/bit_reader_utils.c VP8InitBitReader
/// Initializes the boolean decoder over `start` and loads its first bytes.
pub fn vp8_init_bit_reader<'a>(br: &mut VP8BitReader<'a>, start: &'a [u8]) {
    debug_assert!(start.len() < (1 << 31)); // limit ensured by format and upstream checks
    br.range = 255 - 1;
    br.value = 0;
    br.bits = -8; // to load the very first 8bits
    br.eof = false;
    vp8_bit_reader_set_buffer(br, start);
    vp8_load_new_bytes(br);
}

// upstream: src/utils/bit_reader_utils.c VP8LoadFinalBytes
/// Loads one byte at a time near the end of the input. Once the input is
/// exhausted, the first call sets `eof` and shifts in zeros, and later calls
/// only keep the state.
pub fn vp8_load_final_bytes(br: &mut VP8BitReader<'_>) {
    // Only read 8bits at a time
    if br.buf < br.data.len() {
        br.bits += 8;
        br.value = bit_t::from(br.data[br.buf]) | (br.value << 8);
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
pub fn vp8_get_value(br: &mut VP8BitReader<'_>, num_bits: i32) -> u32 {
    let mut v: u32 = 0;
    let mut bits = num_bits;
    while bits > 0 {
        bits -= 1;
        v |= (vp8_get_bit(br, 0x80) as u32) << bits;
    }
    v
}

// upstream: src/utils/bit_reader_utils.h VP8Get
/// Reads one bit with probability one half. The header's macro of the same name
/// expands to `VP8GetValue(br, 1)`.
pub fn vp8_get(br: &mut VP8BitReader<'_>) -> u32 {
    vp8_get_value(br, 1)
}

// upstream: src/utils/bit_reader_utils.c VP8GetSignedValue
/// Reads `num_bits` bits, then a sign bit that negates the value when set.
#[expect(clippy::cast_possible_wrap, reason = "C converts the uint32_t to int")]
pub fn vp8_get_signed_value(br: &mut VP8BitReader<'_>, num_bits: i32) -> i32 {
    let value = vp8_get_value(br, num_bits) as i32;
    if vp8_get(br) != 0 { -value } else { value }
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
/// The lossless bit reader: a window of prefetched bits over its input.
#[derive(Debug, Default)]
pub struct VP8LBitReader<'a> {
    /// Prefetched bits.
    pub val: vp8l_val_t,
    /// The input (C `buf` with `len` bytes).
    pub buf: &'a [u8],
    /// Byte position in `buf` of the next byte to be loaded.
    pub pos: usize,
    /// Current bit-reading position in `val`.
    pub bit_pos: i32,
    /// Set when a bit was read past the end of the input.
    pub eos: bool,
}

// upstream: src/utils/bit_reader_utils.c VP8LInitBitReader
/// Initializes the lossless reader over `start` and prefetches its first eight
/// bytes.
pub fn vp8l_init_bit_reader<'a>(br: &mut VP8LBitReader<'a>, start: &'a [u8]) {
    debug_assert!(start.len() < 0xffff_fff8); // can't happen with a RIFF chunk.
    let length = start.len().min(size_of::<vp8l_val_t>());
    let mut value: vp8l_val_t = 0;
    for (i, &byte) in start[..length].iter().enumerate() {
        value |= vp8l_val_t::from(byte) << (8 * i);
    }
    br.val = value;
    br.pos = length;
    br.bit_pos = 0;
    br.eos = false;
    br.buf = start;
}

// upstream: src/utils/bit_reader_utils.c VP8LBitReaderSetBuffer
/// Sets a new input. The position and the prefetched bits are kept, and the
/// end-of-stream flag is recomputed, so a position past the new end sets `eos`.
pub fn vp8l_bit_reader_set_buffer<'a>(br: &mut VP8LBitReader<'a>, buf: &'a [u8]) {
    debug_assert!(buf.len() < 0xffff_fff8); // can't happen with a RIFF chunk.
    br.buf = buf;
    // 'pos' > 'len' should be considered a param error.
    br.eos = (br.pos > br.buf.len()) || vp8l_is_end_of_stream(br);
}

// upstream: src/utils/bit_reader_utils.c VP8LSetEndOfStream
/// Sets `eos` and clears the position, to avoid undefined shifts.
fn vp8l_set_end_of_stream(br: &mut VP8LBitReader<'_>) {
    br.eos = true;
    br.bit_pos = 0; // To avoid undefined behaviour with shifts.
}

// upstream: src/utils/bit_reader_utils.c ShiftBytes
/// Reloads up to `VP8L_LBITS` bits byte by byte, while the input lasts.
fn shift_bytes(br: &mut VP8LBitReader<'_>) {
    while br.bit_pos >= 8 && br.pos < br.buf.len() {
        br.val >>= 8;
        br.val |= vp8l_val_t::from(br.buf[br.pos]) << (VP8L_LBITS - 8);
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
pub fn vp8l_read_bits(br: &mut VP8LBitReader<'_>, n_bits: i32) -> u32 {
    debug_assert!(n_bits >= 0);
    // Flag an error if end_of_stream or n_bits is more than allowed limit.
    if !br.eos && n_bits <= VP8L_MAX_NUM_BIT_READ {
        let val = vp8l_prefetch_bits(br) & BIT_MASK[n_bits as usize];
        let new_bits = br.bit_pos + n_bits;
        br.bit_pos = new_bits;
        shift_bytes(br);
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
pub fn vp8l_prefetch_bits(br: &VP8LBitReader<'_>) -> u32 {
    (br.val >> (br.bit_pos & (VP8L_LBITS - 1))) as u32
}

// upstream: src/utils/bit_reader_utils.h VP8LIsEndOfStream
/// Reports whether a bit was read past the end of the input. Does not set `eos`.
#[must_use]
pub fn vp8l_is_end_of_stream(br: &VP8LBitReader<'_>) -> bool {
    debug_assert!(br.pos <= br.buf.len());
    br.eos || ((br.pos == br.buf.len()) && (br.bit_pos > VP8L_LBITS))
}

// upstream: src/utils/bit_reader_utils.h VP8LSetBitPos
/// Jumps to the bit position `val` of the prefetched bits. Does not set `eos`;
/// use with care.
pub fn vp8l_set_bit_pos(br: &mut VP8LBitReader<'_>, val: i32) {
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
pub fn vp8l_do_fill_bit_window(br: &mut VP8LBitReader<'_>) {
    debug_assert!(br.bit_pos >= VP8L_WBITS);
    if VP8L_USE_FAST_LOAD && br.pos + size_of::<vp8l_val_t>() < br.buf.len() {
        br.val >>= VP8L_WBITS;
        br.bit_pos -= VP8L_WBITS;
        let word = h_to_le32(webp_mem_to_uint32(&br.buf[br.pos..]));
        br.val |= vp8l_val_t::from(word) << (VP8L_LBITS - VP8L_WBITS);
        br.pos += VP8L_LOG8_WBITS;
        return;
    }
    shift_bytes(br); // Slow path.
}

// upstream: src/utils/bit_reader_utils.h VP8LFillBitWindow
/// Makes room for the next 32 bits when fewer than 32 bits are prefetched.
pub fn vp8l_fill_bit_window(br: &mut VP8LBitReader<'_>) {
    if br.bit_pos >= VP8L_WBITS {
        vp8l_do_fill_bit_window(br);
    }
}
