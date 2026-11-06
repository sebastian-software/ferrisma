// libwebp v1.6.0, src/utils/bit_writer_utils.c:
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
// Bit writing and boolean coder
//
// Author: Skal (pascal.massimino@gmail.com)
//         Vikas Arora (vikaas.arora@gmail.com)

// libwebp v1.6.0, src/utils/bit_writer_utils.h:
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
// Bit writing and boolean coder
//
// Author: Skal (pascal.massimino@gmail.com)

//! Translation of `src/utils/bit_writer_utils.c` and `src/utils/bit_writer_utils.h` from libwebp v1.6.0.
//!
//! Work package: WP2 (bit I/O).
//!
//! `VP8BitWriter` and `VP8LBitWriter` own their byte buffers. The buffer is a
//! `Vec<u8>` whose length is the allocated size (C `max_pos`, or `end - buf`), and
//! `cur` is an offset into it (ADR 0003). `Finish` returns the buffer as a borrow,
//! and `WipeOut` drops it.
//!
//! The `VP8LBitWriter` accumulator and write word depend on the target: upstream
//! uses 64-bit and 32-bit types on x86-64, and 32-bit and 16-bit types elsewhere.
//! The constants below make the same choice, so that the internal state matches
//! the oracle built for the same host. The output bytes are the same for both
//! choices.

use crate::utils::utils::{check_size_overflow, webp_safe_calloc};

// upstream: src/utils/bit_writer_utils.c MIN_EXTRA_SIZE
/// The smallest growth step of the `VP8LBitWriter` buffer, in bytes.
const MIN_EXTRA_SIZE: u64 = 32768;

// upstream: src/utils/bit_writer_utils.h vp8l_atype_t
/// Bit accumulator of `VP8LBitWriter`: 64 bits on x86-64, 32 bits elsewhere.
#[cfg(target_arch = "x86_64")]
#[expect(non_camel_case_types, reason = "C type name")]
pub type vp8l_atype_t = u64;

// upstream: src/utils/bit_writer_utils.h vp8l_atype_t
/// Bit accumulator of `VP8LBitWriter`: 64 bits on x86-64, 32 bits elsewhere.
#[cfg(not(target_arch = "x86_64"))]
#[expect(non_camel_case_types, reason = "C type name")]
pub type vp8l_atype_t = u32;

// upstream: src/utils/bit_writer_utils.h vp8l_wtype_t
/// Write word of `VP8LBitWriter`: 32 bits on x86-64, 16 bits elsewhere.
#[cfg(target_arch = "x86_64")]
#[expect(non_camel_case_types, reason = "C type name")]
pub type vp8l_wtype_t = u32;

// upstream: src/utils/bit_writer_utils.h vp8l_wtype_t
/// Write word of `VP8LBitWriter`: 32 bits on x86-64, 16 bits elsewhere.
#[cfg(not(target_arch = "x86_64"))]
#[expect(non_camel_case_types, reason = "C type name")]
pub type vp8l_wtype_t = u16;

// upstream: src/utils/bit_writer_utils.h VP8L_WRITER_BYTES
/// `sizeof(vp8l_wtype_t)`.
#[cfg(target_arch = "x86_64")]
pub const VP8L_WRITER_BYTES: usize = 4;
// upstream: src/utils/bit_writer_utils.h VP8L_WRITER_BYTES
/// `sizeof(vp8l_wtype_t)`.
#[cfg(not(target_arch = "x86_64"))]
pub const VP8L_WRITER_BYTES: usize = 2;

// upstream: src/utils/bit_writer_utils.h VP8L_WRITER_BITS
/// `8 * sizeof(vp8l_wtype_t)`.
#[cfg(target_arch = "x86_64")]
pub const VP8L_WRITER_BITS: i32 = 32;
// upstream: src/utils/bit_writer_utils.h VP8L_WRITER_BITS
/// `8 * sizeof(vp8l_wtype_t)`.
#[cfg(not(target_arch = "x86_64"))]
pub const VP8L_WRITER_BITS: i32 = 16;

// upstream: src/utils/bit_writer_utils.h VP8L_WRITER_MAX_BITS
/// `8 * sizeof(vp8l_atype_t)`.
#[cfg(target_arch = "x86_64")]
pub const VP8L_WRITER_MAX_BITS: i32 = 64;
// upstream: src/utils/bit_writer_utils.h VP8L_WRITER_MAX_BITS
/// `8 * sizeof(vp8l_atype_t)`.
#[cfg(not(target_arch = "x86_64"))]
pub const VP8L_WRITER_MAX_BITS: i32 = 32;

const _: () = assert!(size_of::<vp8l_wtype_t>() == VP8L_WRITER_BYTES);
const _: () = assert!(VP8L_WRITER_MAX_BITS == 2 * VP8L_WRITER_BITS);

// upstream: src/utils/bit_writer_utils.c kNorm
/// Renormalization shifts: `renorm_sizes[i] = 8 - log2(i)`.
const NORM: [u8; 128] = [
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

// upstream: src/utils/bit_writer_utils.c kNewRange
/// `range = ((range + 1) << kNorm[range]) - 1`.
const NEW_RANGE: [u8; 128] = [
    127, 127, 191, 127, 159, 191, 223, 127, 143, 159, 175, 191, 207, 223, 239, 127, //
    135, 143, 151, 159, 167, 175, 183, 191, 199, 207, 215, 223, 231, 239, 247, 127, //
    131, 135, 139, 143, 147, 151, 155, 159, 163, 167, 171, 175, 179, 183, 187, 191, //
    195, 199, 203, 207, 211, 215, 219, 223, 227, 231, 235, 239, 243, 247, 251, 127, //
    129, 131, 133, 135, 137, 139, 141, 143, 145, 147, 149, 151, 153, 155, 157, 159, //
    161, 163, 165, 167, 169, 171, 173, 175, 177, 179, 181, 183, 185, 187, 189, 191, //
    193, 195, 197, 199, 201, 203, 205, 207, 209, 211, 213, 215, 217, 219, 221, 223, //
    225, 227, 229, 231, 233, 235, 237, 239, 241, 243, 245, 247, 249, 251, 253, 127,
];

// upstream: src/utils/bit_writer_utils.h VP8BitWriter
/// The boolean encoder of one partition, with the bytes it has written.
#[derive(Debug, Default)]
pub struct VP8BitWriter {
    /// Range minus one.
    pub range: i32,
    pub value: i32,
    /// Number of outstanding `0xff` bytes that wait for a possible carry.
    pub run: i32,
    /// Number of pending bits.
    pub nb_bits: i32,
    /// The byte buffer. Its length is the allocated size (C `max_pos`).
    pub buf: Vec<u8>,
    /// Number of bytes written into `buf`.
    pub pos: usize,
    /// Set after a failed memory allocation.
    pub error: bool,
}

// upstream: src/utils/bit_writer_utils.c BitWriterResize
/// Makes room for `extra_size` more bytes, growing the buffer as C does: at
/// least twice the old size, at least the needed size, and at least 1024 bytes.
/// The reservation uses the size test of `WebPSafeMalloc(1, new_size)`, which
/// `webp_safe_calloc::<u8>(new_size, 1)` applies with the same decision.
fn bit_writer_resize(bw: &mut VP8BitWriter, extra_size: usize) -> bool {
    let needed_size_64b = bw.pos as u64 + extra_size as u64;
    let Ok(needed_size) = usize::try_from(needed_size_64b) else {
        bw.error = true;
        return false;
    };
    if needed_size <= bw.buf.len() {
        return true;
    }
    let mut new_size = bw.buf.len().wrapping_mul(2);
    if new_size < needed_size {
        new_size = needed_size;
    }
    if new_size < 1024 {
        new_size = 1024;
    }
    let Some(mut new_buf) = webp_safe_calloc::<u8>(new_size as u64, 1) else {
        bw.error = true;
        return false;
    };
    new_buf[..bw.pos].copy_from_slice(&bw.buf[..bw.pos]);
    bw.buf = new_buf;
    true
}

// upstream: src/utils/bit_writer_utils.c Flush
/// Moves the top bits of `value` into the buffer, delaying `0xff` bytes until a
/// possible carry is known.
#[expect(
    clippy::if_not_else,
    clippy::cast_sign_loss,
    reason = "keeps the branch order of C; the run and the byte are non-negative"
)]
fn flush(bw: &mut VP8BitWriter) {
    let s = 8 + bw.nb_bits;
    let bits = bw.value >> s;
    debug_assert!(bw.nb_bits >= 0);
    bw.value -= bits << s;
    bw.nb_bits -= 8;
    if (bits & 0xff) != 0xff {
        let mut pos = bw.pos;
        if !bit_writer_resize(bw, bw.run as usize + 1) {
            return;
        }
        if (bits & 0x100) != 0 && pos > 0 {
            // overflow: propagate the carry over the pending 0xff bytes
            bw.buf[pos - 1] = bw.buf[pos - 1].wrapping_add(1);
        }
        if bw.run > 0 {
            let value: u8 = if (bits & 0x100) != 0 { 0x00 } else { 0xff };
            while bw.run > 0 {
                bw.buf[pos] = value;
                pos += 1;
                bw.run -= 1;
            }
        }
        bw.buf[pos] = (bits & 0xff) as u8;
        pos += 1;
        bw.pos = pos;
    } else {
        bw.run += 1;
    }
}

// upstream: src/utils/bit_writer_utils.c VP8PutBit
/// Writes `bit` with the probability `prob` of a zero, and returns `bit`.
#[expect(
    clippy::cast_sign_loss,
    reason = "the renormalized range is in 0..128, a table index"
)]
pub fn vp8_put_bit(bw: &mut VP8BitWriter, bit: bool, prob: i32) -> bool {
    let split = (bw.range * prob) >> 8;
    if bit {
        bw.value += split + 1;
        bw.range -= split + 1;
    } else {
        bw.range = split;
    }
    if bw.range < 127 {
        // emit 'shift' bits out and renormalize
        let shift = i32::from(NORM[bw.range as usize]);
        bw.range = i32::from(NEW_RANGE[bw.range as usize]);
        bw.value <<= shift;
        bw.nb_bits += shift;
        if bw.nb_bits > 0 {
            flush(bw);
        }
    }
    bit
}

// upstream: src/utils/bit_writer_utils.c VP8PutBitUniform
/// Writes `bit` with probability one half, and returns `bit`.
#[expect(
    clippy::cast_sign_loss,
    reason = "the renormalized range is in 0..128, a table index"
)]
pub fn vp8_put_bit_uniform(bw: &mut VP8BitWriter, bit: bool) -> bool {
    let split = bw.range >> 1;
    if bit {
        bw.value += split + 1;
        bw.range -= split + 1;
    } else {
        bw.range = split;
    }
    if bw.range < 127 {
        bw.range = i32::from(NEW_RANGE[bw.range as usize]);
        bw.value <<= 1;
        bw.nb_bits += 1;
        if bw.nb_bits > 0 {
            flush(bw);
        }
    }
    bit
}

// upstream: src/utils/bit_writer_utils.c VP8PutBits
/// Writes the `nb_bits` low bits of `value`, most significant first, uniformly.
pub fn vp8_put_bits(bw: &mut VP8BitWriter, value: u32, nb_bits: i32) {
    debug_assert!(nb_bits > 0 && nb_bits < 32);
    let mut mask: u32 = 1u32 << (nb_bits - 1);
    while mask != 0 {
        vp8_put_bit_uniform(bw, value & mask != 0);
        mask >>= 1;
    }
}

// upstream: src/utils/bit_writer_utils.c VP8PutSignedBits
/// Writes a flag for a nonzero `value`, then its magnitude and sign.
///
/// # Panics
///
/// Debug builds panic when `nb_bits` is outside `0..31`, as `VP8PutBits` does.
#[expect(clippy::cast_sign_loss, reason = "the branch takes value >= 0")]
pub fn vp8_put_signed_bits(bw: &mut VP8BitWriter, value: i32, nb_bits: i32) {
    if !vp8_put_bit_uniform(bw, value != 0) {
        return;
    }
    if value < 0 {
        // C computes `(-value) << 1 | 1` as an int; the magnitude is the same
        // for every value that does not overflow.
        vp8_put_bits(bw, (value.unsigned_abs() << 1) | 1, nb_bits + 1);
    } else {
        vp8_put_bits(bw, (value as u32) << 1, nb_bits + 1);
    }
}

// upstream: src/utils/bit_writer_utils.c VP8BitWriterInit
/// Initializes the encoder and reserves `expected_size` bytes when it is
/// positive. Returns `false` after a failed allocation, with `error` set.
pub fn vp8_bit_writer_init(bw: &mut VP8BitWriter, expected_size: usize) -> bool {
    *bw = VP8BitWriter {
        range: 255 - 1,
        value: 0,
        run: 0,
        nb_bits: -8,
        buf: Vec::new(),
        pos: 0,
        error: false,
    };
    if expected_size > 0 {
        bit_writer_resize(bw, expected_size)
    } else {
        true
    }
}

// upstream: src/utils/bit_writer_utils.c VP8BitWriterFinish
/// Pads the stream with zero bits, flushes it, and returns the buffer. The
/// first `vp8_bit_writer_size` bytes of the returned slice are the stream.
pub fn vp8_bit_writer_finish(bw: &mut VP8BitWriter) -> &[u8] {
    vp8_put_bits(bw, 0, 9 - bw.nb_bits);
    bw.nb_bits = 0;
    flush(bw);
    &bw.buf
}

// upstream: src/utils/bit_writer_utils.c VP8BitWriterAppend
/// Appends `data` to the buffer. Refused, and returns `false`, unless the
/// encoder is byte aligned (`nb_bits == -8`), as C requires.
pub fn vp8_bit_writer_append(bw: &mut VP8BitWriter, data: &[u8]) -> bool {
    if bw.nb_bits != -8 {
        return false; // Flush() must have been called
    }
    if !bit_writer_resize(bw, data.len()) {
        return false;
    }
    bw.buf[bw.pos..bw.pos + data.len()].copy_from_slice(data);
    bw.pos += data.len();
    true
}

// upstream: src/utils/bit_writer_utils.c VP8BitWriterWipeOut
/// Releases the buffer and zeroes the encoder. Only useful after an error, when
/// the buffer has not been taken.
pub fn vp8_bit_writer_wipe_out(bw: &mut VP8BitWriter) {
    *bw = VP8BitWriter::default();
}

// upstream: src/utils/bit_writer_utils.h VP8BitWriterPos
/// The approximate write position, in bits.
#[must_use]
#[expect(
    clippy::cast_sign_loss,
    reason = "nb_bits is at least -8 and run is not negative"
)]
pub fn vp8_bit_writer_pos(bw: &VP8BitWriter) -> u64 {
    let nb_bits = (8 + bw.nb_bits) as u64; // bw.nb_bits is <= 0, note
    ((bw.pos as u64) + (bw.run as u64)) * 8 + nb_bits
}

// upstream: src/utils/bit_writer_utils.h VP8BitWriterBuf
/// The internal buffer, including the bytes past `vp8_bit_writer_size`.
#[must_use]
pub fn vp8_bit_writer_buf(bw: &VP8BitWriter) -> &[u8] {
    &bw.buf
}

// upstream: src/utils/bit_writer_utils.h VP8BitWriterSize
/// The number of bytes written.
#[must_use]
pub fn vp8_bit_writer_size(bw: &VP8BitWriter) -> usize {
    bw.pos
}

// upstream: src/utils/bit_writer_utils.c VP8LBitWriterResize
/// Makes room for `extra_size` more bytes of the `VP8LBitWriter` buffer. The new
/// size is one and a half times the old, at least the needed size, rounded up to
/// a multiple of 1024 bytes, and it always adds at least one such step.
fn vp8l_bit_writer_resize(bw: &mut VP8LBitWriter, extra_size: usize) -> bool {
    let max_bytes = bw.buf.len();
    let current_size = bw.cur;
    let size_required_64b = current_size as u64 + extra_size as u64;
    let Ok(size_required) = usize::try_from(size_required_64b) else {
        bw.error = true;
        return false;
    };
    if max_bytes > 0 && size_required <= max_bytes {
        return true;
    }
    let mut allocated_size = (3 * max_bytes) >> 1;
    if allocated_size < size_required {
        allocated_size = size_required;
    }
    // make allocated size multiple of 1k
    allocated_size = ((allocated_size >> 10) + 1) << 10;
    let Some(mut allocated_buf) = webp_safe_calloc::<u8>(allocated_size as u64, 1) else {
        bw.error = true;
        return false;
    };
    allocated_buf[..current_size].copy_from_slice(&bw.buf[..current_size]);
    bw.buf = allocated_buf;
    bw.cur = current_size;
    true
}

// upstream: src/utils/bit_writer_utils.h VP8LBitWriter
/// The lossless bit writer: an accumulator of `used` pending bits and a buffer
/// that the complete words are written into.
#[derive(Debug, Default)]
pub struct VP8LBitWriter {
    /// Pending bits, least significant first.
    pub bits: vp8l_atype_t,
    /// Number of bits used in `bits`.
    pub used: i32,
    /// The byte buffer. Its length is the allocated size (C `end - buf`).
    pub buf: Vec<u8>,
    /// Offset in `buf` of the current write position (C `cur`).
    pub cur: usize,
    /// Set after a failed memory allocation.
    pub error: bool,
}

// upstream: src/utils/bit_writer_utils.h VP8LBitWriterNumBytes
/// The number of bytes that the stream occupies if it is finished now.
#[must_use]
#[expect(clippy::cast_sign_loss, reason = "used is not negative")]
pub fn vp8l_bit_writer_num_bytes(bw: &VP8LBitWriter) -> usize {
    bw.cur + ((bw.used + 7) >> 3) as usize
}

// upstream: src/utils/bit_writer_utils.c VP8LBitWriterInit
/// Zeroes the writer and reserves `expected_size` bytes. Returns `false` after a
/// failed allocation, with `error` set.
pub fn vp8l_bit_writer_init(bw: &mut VP8LBitWriter, expected_size: usize) -> bool {
    *bw = VP8LBitWriter::default();
    vp8l_bit_writer_resize(bw, expected_size)
}

// upstream: src/utils/bit_writer_utils.c VP8LBitWriterClone
/// Copies the written bytes and the pending bits of `src` into `dst`. Returns
/// `false` when `dst` cannot grow.
pub fn vp8l_bit_writer_clone(src: &VP8LBitWriter, dst: &mut VP8LBitWriter) -> bool {
    let current_size = src.cur;
    debug_assert!(src.cur <= src.buf.len());
    if !vp8l_bit_writer_resize(dst, current_size) {
        return false;
    }
    dst.buf[..current_size].copy_from_slice(&src.buf[..current_size]);
    dst.bits = src.bits;
    dst.used = src.used;
    dst.error = src.error;
    dst.cur = current_size;
    true
}

// upstream: src/utils/bit_writer_utils.c VP8LBitWriterWipeOut
/// Releases the buffer and zeroes the writer.
pub fn vp8l_bit_writer_wipe_out(bw: &mut VP8LBitWriter) {
    *bw = VP8LBitWriter::default();
}

// upstream: src/utils/bit_writer_utils.c VP8LBitWriterReset
/// Returns the cursor of `bw` to the position that `bw_init` had when it was
/// initialized, with the same pending bits and error state. The bytes are not
/// copied, so `bw` must hold the bytes that `bw_init` had written.
pub fn vp8l_bit_writer_reset(bw_init: &VP8LBitWriter, bw: &mut VP8LBitWriter) {
    bw.bits = bw_init.bits;
    bw.used = bw_init.used;
    bw.cur = bw_init.cur;
    debug_assert!(bw.cur <= bw.buf.len());
    bw.error = bw_init.error;
}

// upstream: src/utils/bit_writer_utils.c VP8LBitWriterSwap
/// Swaps the memory and the state of two writers.
pub fn vp8l_bit_writer_swap(src: &mut VP8LBitWriter, dst: &mut VP8LBitWriter) {
    std::mem::swap(src, dst);
}

// upstream: src/utils/bit_writer_utils.c VP8LBitWriterFinish
/// Writes the pending bits as whole bytes and returns the buffer. The first
/// `vp8l_bit_writer_num_bytes` bytes of the returned slice are the stream.
#[expect(
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    reason = "used is not negative; C stores the low byte"
)]
pub fn vp8l_bit_writer_finish(bw: &mut VP8LBitWriter) -> &[u8] {
    // flush leftover bits
    if vp8l_bit_writer_resize(bw, ((bw.used + 7) >> 3) as usize) {
        while bw.used > 0 {
            bw.buf[bw.cur] = bw.bits as u8;
            bw.cur += 1;
            bw.bits >>= 8;
            bw.used -= 8;
        }
        bw.used = 0;
    }
    &bw.buf
}

// upstream: src/utils/bit_writer_utils.c VP8LPutBitsFlushBits
/// Writes one write word of the accumulator and makes room for it first, by
/// growing the buffer. After a failed growth the cursor returns to the start of
/// the buffer and `error` is set, as C does.
#[expect(
    clippy::cast_possible_truncation,
    reason = "the size is checked by check_size_overflow first"
)]
pub fn vp8l_put_bits_flush_bits(bw: &mut VP8LBitWriter) {
    // If needed, make some room by flushing some bits out.
    if bw.cur + VP8L_WRITER_BYTES > bw.buf.len() {
        let extra_size = bw.buf.len() as u64 + MIN_EXTRA_SIZE;
        if !check_size_overflow(extra_size) || !vp8l_bit_writer_resize(bw, extra_size as usize) {
            bw.cur = 0;
            bw.error = true;
            return;
        }
    }
    let word = bw.bits as vp8l_wtype_t;
    bw.buf[bw.cur..bw.cur + VP8L_WRITER_BYTES].copy_from_slice(&word.to_le_bytes());
    bw.cur += VP8L_WRITER_BYTES;
    bw.bits >>= VP8L_WRITER_BITS;
    bw.used -= VP8L_WRITER_BITS;
}

// upstream: src/utils/bit_writer_utils.c VP8LPutBitsInternal
/// Writes up to 32 bits into the accumulator in the write-word size of 16 bits,
/// which is the path of every target except x86-64. Its state after the call
/// matches C for every input except `n_bits == 32` at `used == 0`: C shifts
/// `uint32_t` by 32 there, which is undefined behavior, so that input is excluded
/// from the differential tests.
#[expect(
    clippy::cast_possible_truncation,
    reason = "the size is checked by check_size_overflow first"
)]
pub fn vp8l_put_bits_internal(bw: &mut VP8LBitWriter, mut bits: u32, mut n_bits: i32) {
    debug_assert!(n_bits <= 32);
    if n_bits > 0 {
        let mut lbits = bw.bits;
        let mut used = bw.used;
        // Special case of overflow handling for 32bit accumulator (2-steps flush).
        if VP8L_WRITER_BITS == 16 && used + n_bits >= VP8L_WRITER_MAX_BITS {
            // Fill up all the VP8L_WRITER_MAX_BITS so it can be flushed out below.
            let shift = VP8L_WRITER_MAX_BITS - used;
            lbits |= vp8l_atype_t::from(bits) << used;
            used = VP8L_WRITER_MAX_BITS;
            n_bits -= shift;
            bits >>= shift;
            debug_assert!(n_bits <= VP8L_WRITER_MAX_BITS);
        }
        // If needed, make some room by flushing some bits out.
        while used >= VP8L_WRITER_BITS {
            if bw.cur + VP8L_WRITER_BYTES > bw.buf.len() {
                let extra_size = bw.buf.len() as u64 + MIN_EXTRA_SIZE;
                if !check_size_overflow(extra_size)
                    || !vp8l_bit_writer_resize(bw, extra_size as usize)
                {
                    bw.cur = 0;
                    bw.error = true;
                    return;
                }
            }
            let word = lbits as vp8l_wtype_t;
            bw.buf[bw.cur..bw.cur + VP8L_WRITER_BYTES].copy_from_slice(&word.to_le_bytes());
            bw.cur += VP8L_WRITER_BYTES;
            lbits >>= VP8L_WRITER_BITS;
            used -= VP8L_WRITER_BITS;
        }
        bw.bits = lbits | (vp8l_atype_t::from(bits) << used);
        bw.used = used + n_bits;
    }
}

// upstream: src/utils/bit_writer_utils.h VP8LPutBits
/// Writes the `n_bits` low bits of `bits`, least significant first. Sets `error`
/// on a failed memory allocation. The 32-bit write word writes the accumulator
/// directly; the 16-bit word delegates to `vp8l_put_bits_internal`.
pub fn vp8l_put_bits(bw: &mut VP8LBitWriter, bits: u32, n_bits: i32) {
    if VP8L_WRITER_BYTES == 4 {
        if n_bits > 0 {
            if bw.used >= 32 {
                vp8l_put_bits_flush_bits(bw);
            }
            bw.bits |= vp8l_atype_t::from(bits) << bw.used;
            bw.used += n_bits;
        }
    } else {
        vp8l_put_bits_internal(bw, bits, n_bits);
    }
}
