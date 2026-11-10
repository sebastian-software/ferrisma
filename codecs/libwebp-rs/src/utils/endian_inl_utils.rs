// libwebp v1.6.0, src/utils/endian_inl_utils.h:
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
// Endian related functions.

//! Translation of `src/utils/endian_inl_utils.h` from libwebp v1.6.0.
//!
//! Work package: WP1 (types, memory, workers).
//!
//! The `HAVE_BUILTIN_*` detection and the MIPS and x86 assembly alternatives of
//! the byte swaps are preprocessor choices of one implementation; each Rust
//! function below is the portable operation that every C variant computes.

// upstream: src/utils/endian_inl_utils.h HToLE32
/// Converts a host `u32` to little-endian order; the identity on little-endian hosts.
#[must_use]
pub fn h_to_le32(x: u32) -> u32 {
    x.to_le()
}

// upstream: src/utils/endian_inl_utils.h HToLE16
/// Converts a host `u16` to little-endian order; the identity on little-endian hosts.
#[must_use]
pub fn h_to_le16(x: u16) -> u16 {
    x.to_le()
}

// upstream: src/utils/endian_inl_utils.h BSwap16
/// Reverses the bytes of a `u16`.
#[must_use]
pub fn b_swap16(x: u16) -> u16 {
    x.swap_bytes()
}

// upstream: src/utils/endian_inl_utils.h BSwap32
/// Reverses the bytes of a `u32`.
#[must_use]
pub fn b_swap32(x: u32) -> u32 {
    x.swap_bytes()
}

// upstream: src/utils/endian_inl_utils.h BSwap64
/// Reverses the bytes of a `u64`.
#[must_use]
pub fn b_swap64(x: u64) -> u64 {
    x.swap_bytes()
}
