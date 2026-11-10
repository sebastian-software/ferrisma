// Copyright 2013 Google Inc. All Rights Reserved.
// Distributed under the MIT license in licenses/BROTLI-MIT.txt.

//! Representation candidate for c/common/transform.c's p[1] ^= 32 and
//! p[2] ^= 5 when the destination tail is still MaybeUninit. The whole
//! read/XOR/write stays in assembly; no unknown byte becomes a Rust scalar.
//! This helper is not yet used by decoder state or a raw transform owner.

use std::mem::MaybeUninit;

/// Apply the original byte XOR without changing its initialization domain.
/// An initialized byte retains a defined transformed value; an unknown byte
/// remains MaybeUninit. Native AArch64/x86-64 proofs are required before use;
/// no guessed implementation is supplied for other architectures.
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub fn xor_byte(byte: &mut MaybeUninit<u8>, mask: u8) {
    let address = byte.as_mut_ptr();
    #[cfg(target_arch = "aarch64")]
    // SAFETY: The exclusive MaybeUninit borrow provides one live byte, with
    // alignment one and no competing aliases. Assembly alone loads the
    // potentially unknown bits and stores their XOR into the same byte; its
    // scratch is discarded and cannot overlap any still-live input (out,
    // not lateout). Memory effects are declared. LDRB/EOR/STRB do not change
    // flags, allocate, unwind, access another byte or produce a Rust value.
    unsafe {
        std::arch::asm!(
            "ldrb {scratch:w}, [{address}]",
            "eor {scratch:w}, {scratch:w}, {mask:w}",
            "strb {scratch:w}, [{address}]",
            address = in(reg) address,
            mask = in(reg) u32::from(mask),
            scratch = out(reg) _,
            options(nostack, preserves_flags),
        );
    }
    #[cfg(target_arch = "x86_64")]
    // SAFETY: The same live, exclusive single-byte extent applies. This one
    // RMW reads/writes only that MaybeUninit byte and produces no Rust output.
    // Both operands are read-only registers; the instruction changes flags,
    // so preserves_flags is absent. No nomem/readonly claim hides the write.
    unsafe {
        std::arch::asm!(
            "xor byte ptr [{address}], {mask}",
            address = in(reg) address,
            mask = in(reg_byte) mask,
            options(nostack),
        );
    }
}
