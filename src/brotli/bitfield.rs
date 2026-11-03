// Copyright 2015 Google Inc. All Rights Reserved.
// Distributed under the MIT license in licenses/BROTLI-MIT.txt.

//! Raw representation of the packed flags in c/dec/state.h. Individual source
//! assignments must retain unknown neighboring bits seeded by an allocator.
//! Loading those bits into a Rust integer would be invalid when uninitialized;
//! the native load/mask/store remains entirely inside assembly instead.

use std::mem::MaybeUninit;

/// A four-byte, source-aligned word within an exclusively borrowed live block.
/// This representation candidate is currently proved only on AArch64/x86-64.
pub struct BitfieldWord<'a> {
    storage: &'a mut [MaybeUninit<u8>; 4],
}

impl<'a> BitfieldWord<'a> {
    pub fn new(storage: &'a mut [MaybeUninit<u8>; 4]) -> Self {
        assert_eq!(storage.as_ptr() as usize % 4, 0);
        Self { storage }
    }

    fn mask(shift: u32, width: u32) -> u32 {
        assert!(width > 0 && width <= 32 && shift <= 32 - width);
        (u32::MAX >> (32 - width)) << shift
    }

    /// The original field assignment, including its unsigned truncation.
    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    pub fn write(&mut self, shift: u32, width: u32, value: u32) {
        let mask = Self::mask(shift, width);
        let bits = (value << shift) & mask;
        let word = self.storage.as_mut_ptr();
        #[cfg(target_arch = "aarch64")]
        // SAFETY: The exclusive MaybeUninit borrow owns four live, aligned
        // bytes. Only assembly observes unknown neighboring bits, preserving
        // them in the same word; no uninitialized Rust scalar is produced.
        // Early output reserves scratch separately from every live input.
        // This RMW declares memory effects, uses no stack and does not change
        // AArch64 flags (BIC/ORR without S). There are no concurrent accesses.
        unsafe {
            std::arch::asm!(
                "ldr {scratch:w}, [{word}]",
                "bic {scratch:w}, {scratch:w}, {mask:w}",
                "orr {scratch:w}, {scratch:w}, {bits:w}",
                "str {scratch:w}, [{word}]",
                word = in(reg) word,
                mask = in(reg) mask,
                bits = in(reg) bits,
                scratch = out(reg) _,
                options(nostack, preserves_flags),
            );
        }
        #[cfg(target_arch = "x86_64")]
        // SAFETY: Same live/exclusive/aligned extent and initialization proof
        // as above. Scratch cannot overlap inputs (out, not lateout). The RMW
        // declares memory effects; AND/OR change flags, so preserves_flags is
        // deliberately absent. No unknown bit is returned as a Rust value.
        unsafe {
            std::arch::asm!(
                "mov {scratch:e}, dword ptr [{word}]",
                "and {scratch:e}, {clear:e}",
                "or {scratch:e}, {bits:e}",
                "mov dword ptr [{word}], {scratch:e}",
                word = in(reg) word,
                clear = in(reg) !mask,
                bits = in(reg) bits,
                scratch = out(reg) _,
                options(nostack),
            );
        }
    }

    /// Read an initialized source field without exposing neighboring bits.
    ///
    /// # Safety
    /// Every selected field bit must have been initialized by the allocator or
    /// a reached source assignment. The other bits may remain uninitialized.
    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    pub unsafe fn read(&self, shift: u32, width: u32) -> u32 {
        let mask = Self::mask(shift, width);
        let word = self.storage.as_ptr();
        let value: u32;
        #[cfg(target_arch = "aarch64")]
        // SAFETY: The live, aligned four-byte shared borrow prevents mutation
        // throughout this operation. The caller proves initialization of the
        // selected bits; masking and shifting happen before any Rust output.
        // out reserves the result apart from the address/mask/shift inputs.
        // LDR/AND/LSR leave flags intact; the block declares its memory read.
        unsafe {
            std::arch::asm!(
                "ldr {value:w}, [{word}]",
                "and {value:w}, {value:w}, {mask:w}",
                "lsr {value:w}, {value:w}, {shift:w}",
                word = in(reg) word,
                mask = in(reg) mask,
                shift = in(reg) shift,
                value = out(reg) value,
                options(nostack, preserves_flags, readonly),
            );
        }
        #[cfg(target_arch = "x86_64")]
        // SAFETY: Same borrow/extent/selected-bit initialization proof as
        // above. The result cannot overlap the live address/mask or CL input.
        // AND/SHR clobber flags; readonly permits reads but no memory writes.
        unsafe {
            std::arch::asm!(
                "mov {value:e}, dword ptr [{word}]",
                "and {value:e}, {mask:e}",
                "shr {value:e}, cl",
                word = in(reg) word,
                mask = in(reg) mask,
                in("cl") shift as u8,
                value = out(reg) value,
                options(nostack, readonly),
            );
        }
        value
    }
}
