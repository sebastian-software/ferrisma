// libwebp v1.6.0, src/utils/bit_reader_inl_utils.h:
//
// Copyright 2014 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// Specific inlined methods for boolean decoder [VP8GetBit() ...]
// This file should be included by the .c sources that actually need to call
// these methods.
//
// Author: Skal (pascal.massimino@gmail.com)

//! Translation of `src/utils/bit_reader_inl_utils.h` from libwebp v1.6.0.
//!
//! Work package: WP2 (bit I/O).
//!
//! The `static WEBP_INLINE` functions of the header are `pub` functions here, so
//! that the decoders call them as C does. The `BT_TRACK` and `label` parameters of
//! the bit-trace tool are removed, as in the reference configuration (BITTRACE 0).
//! The sanitizer attributes (`WEBP_UBSAN_IGNORE_*`) have no counterpart: the
//! wrapping operations below state the unsigned wraparound that they permit.

use crate::utils::bit_reader_utils::{
    BITS, VP8_LOG2_RANGE, VP8_NEW_RANGE, VP8BitReader, bit_t, range_t, vp8_load_final_bytes,
};
use crate::utils::utils::bits_log2_floor;

// upstream: src/utils/bit_reader_inl_utils.h lbit_t
/// Natural type for memory reads: 64 bits when `BITS > 32`, and 32 bits when
/// `BITS > 16`. The other sizes of the header are chosen by no target that
/// upstream names.
#[cfg(any(
    target_arch = "x86_64",
    target_arch = "aarch64",
    target_arch = "wasm32",
    target_arch = "wasm64"
))]
#[expect(non_camel_case_types, reason = "C type name")]
pub type lbit_t = u64;

// upstream: src/utils/bit_reader_inl_utils.h lbit_t
/// Natural type for memory reads: 64 bits when `BITS > 32`, and 32 bits when
/// `BITS > 16`. The other sizes of the header are chosen by no target that
/// upstream names.
#[cfg(not(any(
    target_arch = "x86_64",
    target_arch = "aarch64",
    target_arch = "wasm32",
    target_arch = "wasm64"
)))]
#[expect(non_camel_case_types, reason = "C type name")]
pub type lbit_t = u32;

// upstream: src/utils/bit_reader_utils.c VP8BitReaderSetBuffer (sizeof(lbit_t))
/// `sizeof(lbit_t)`: the number of bytes that one load reads.
pub const LBIT_BYTES: usize = size_of::<lbit_t>();

// upstream: src/utils/bit_reader_inl_utils.h VP8LoadNewBytes (`BITS >> 3`)
/// Bytes that one load consumes, `BITS >> 3`.
const LOAD_BYTES: usize = (BITS >> 3) as usize;

// upstream: src/utils/bit_reader_inl_utils.h VP8GetBit (the `(range_t)` cast of its window)
/// The low 32 bits of a register value, as C's `(range_t)` cast takes them. The
/// window of a valid stream fits in eight bits, but a corrupt stream can make it
/// wider, and C then keeps the low bits; so does this conversion.
#[inline]
fn low_range(value: bit_t) -> range_t {
    let bytes = value.to_le_bytes();
    range_t::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
}

// upstream: src/utils/bit_reader_inl_utils.h VP8LoadNewBytes (`bits >>= 8 * sizeof(lbit_t) - BITS`)
/// Shift that moves the `BITS` bits of one load to the bottom of the register.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    reason = "sizeof(lbit_t) is at most 8, which fits in an int"
)]
const LOAD_SHIFT: i32 = 8 * LBIT_BYTES as i32 - BITS;

// upstream: src/utils/bit_reader_inl_utils.h VP8LoadNewBytes
/// Makes sure that `value` holds at least `BITS` bits of data, by loading `BITS`
/// bits at once while the input allows, and byte by byte near its end. `data` is
/// the input of the reader.
///
/// The big-endian value of the `LOAD_BYTES` bytes is the one that the C code
/// computes on every byte order, so the result does not depend on the host.
#[inline]
pub fn vp8_load_new_bytes(br: &mut VP8BitReader, data: &[u8]) {
    debug_assert!(br.has_buf); // assert(br != NULL && br->buf != NULL)
    // Read 'BITS' bits at a time if possible.
    if br.buf < br.buf_max {
        // Read `lbit_t` in big-endian order: the first byte is the most significant.
        let mut in_bits: lbit_t = 0;
        for &byte in &data[br.buf..br.buf + LBIT_BYTES] {
            in_bits = (in_bits << 8) | lbit_t::from(byte);
        }
        br.buf += LOAD_BYTES;
        let bits: bit_t = in_bits >> LOAD_SHIFT;
        br.value = bits | (br.value << BITS);
        br.bits += BITS;
    } else {
        vp8_load_final_bytes(br, data); // no need to be inlined
    }
}

// upstream: src/utils/bit_reader_inl_utils.h VP8GetBit
/// Reads one bit that is zero with probability `prob / 256`. Speed-critical. `data`
/// is the input of the reader.
#[inline]
#[expect(
    clippy::cast_sign_loss,
    reason = "C converts the int probability to uint32_t"
)]
pub fn vp8_get_bit(br: &mut VP8BitReader, data: &[u8], prob: i32) -> i32 {
    // Don't move this declaration! It makes a big speed difference to store
    // 'range' *before* calling VP8LoadNewBytes(), even if this function doesn't
    // alter br->range value.
    let mut range = br.range;
    if br.bits < 0 {
        vp8_load_new_bytes(br, data);
    }
    let pos = br.bits;
    let split = range.wrapping_mul(prob as range_t) >> 8;
    let value = low_range(br.value >> pos);
    let bit = value > split;
    if bit {
        range -= split;
        br.value -= bit_t::from(split + 1) << pos;
    } else {
        range = split + 1;
    }
    let shift = 7 ^ bits_log2_floor(range);
    range <<= shift;
    br.bits -= shift;
    br.range = range - 1;
    i32::from(bit)
}

// upstream: src/utils/bit_reader_inl_utils.h VP8GetSigned
/// Reads one bit with probability one half and applies it to `v`: returns `v`
/// with its sign flipped when the bit is set. A simplified `VP8GetBit` for
/// probability `0x80`. `data` is the input of the reader.
#[inline]
#[expect(
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    reason = "C reinterprets the unsigned difference as int32_t and back"
)]
pub fn vp8_get_signed(br: &mut VP8BitReader, data: &[u8], v: i32) -> i32 {
    if br.bits < 0 {
        vp8_load_new_bytes(br, data);
    }
    let pos = br.bits;
    let split: range_t = br.range >> 1;
    let value = low_range(br.value >> pos);
    // -1 or 0. The subtraction wraps, as the unsigned C arithmetic does.
    let mask = (split.wrapping_sub(value) as i32) >> 31;
    br.bits -= 1;
    br.range = br.range.wrapping_add(mask as range_t);
    br.range |= 1;
    br.value -= bit_t::from((split + 1) & (mask as range_t)) << pos;
    (v ^ mask) - mask
}

// upstream: src/utils/bit_reader_inl_utils.h VP8GetBitAlt
/// The alternative of `VP8GetBit`, which renormalizes with a table lookup. `data`
/// is the input of the reader.
#[inline]
#[expect(
    clippy::cast_sign_loss,
    reason = "C converts the int probability to uint32_t"
)]
pub fn vp8_get_bit_alt(br: &mut VP8BitReader, data: &[u8], prob: i32) -> i32 {
    // Don't move this declaration! It makes a big speed difference to store
    // 'range' *before* calling VP8LoadNewBytes(), even if this function doesn't
    // alter br->range value.
    let mut range = br.range;
    if br.bits < 0 {
        vp8_load_new_bytes(br, data);
    }
    let pos = br.bits;
    let split = range.wrapping_mul(prob as range_t) >> 8;
    let value = low_range(br.value >> pos);
    let bit;
    if value > split {
        range -= split + 1;
        br.value -= bit_t::from(split + 1) << pos;
        bit = 1;
    } else {
        range = split;
        bit = 0;
    }
    if range <= 0x7e {
        let shift = i32::from(VP8_LOG2_RANGE[range as usize]);
        range = u32::from(VP8_NEW_RANGE[range as usize]);
        br.bits -= shift;
    }
    br.range = range;
    bit
}
