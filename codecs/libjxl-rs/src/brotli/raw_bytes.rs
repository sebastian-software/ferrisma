// Copyright 2013 Google Inc. All Rights Reserved.
// Distributed under the MIT license in licenses/BROTLI-MIT.txt.

//! c/common/transform.c's p[1] ^= 32 and p[2] ^= 5 on one destination byte.
//! Stable Rust cannot XOR a byte that may be uninitialized, so this helper
//! takes only initialized bytes; `RawByteDestination::xor` applies it inside
//! its initialized prefix and leaves a byte past that prefix unchanged (owner
//! decision of 2026-11-16; port ADR 0003, "Brotli flag word and transform
//! tail bytes"; ADR-0044).

use std::mem::MaybeUninit;

/// Apply the original byte XOR to an initialized byte.
///
/// # Safety
/// `byte` must be initialized.
pub unsafe fn xor_byte(byte: &mut MaybeUninit<u8>, mask: u8) {
    // SAFETY: The caller guarantees that the byte is initialized.
    let value = unsafe { byte.assume_init() };
    byte.write(value ^ mask);
}
