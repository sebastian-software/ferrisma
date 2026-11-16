// Copyright © 2018-2022, VideoLAN and dav1d authors
// Copyright © 2018-2022, Two Orioles, LLC
// All rights reserved. BSD-2-Clause; see LICENSES/ in this crate.

//! CPU detection of `upstream/dav1d/src/{cpu.h,x86/cpu.c,arm/cpu.c}`
//! without assembly (engine ADR-0044; libavif-rs ADR 0062).
//!
//! The original reads CPUID and XCR0 through `src/x86/cpuid.asm`; this file
//! uses `core::arch::x86_64::__cpuid_count` and `_xgetbv(0)` for the same
//! leaves and register. On AArch64 NEON is part of the baseline
//! (`dav1d_get_default_cpu_flags`). DotProd, I8MM, SVE and SVE2 are above
//! the v1.0 scope of the supported matrix and the port has no kernel for
//! them, so they are not reported; nothing in the port would select on them.
use core::ffi::c_uint;

// src/arm/cpu.h
pub const DAV1D_ARM_CPU_FLAG_NEON: c_uint = 1 << 0;

// src/x86/cpu.h
pub const DAV1D_X86_CPU_FLAG_SSE2: c_uint = 1 << 0;
pub const DAV1D_X86_CPU_FLAG_SSSE3: c_uint = 1 << 1;
pub const DAV1D_X86_CPU_FLAG_SSE41: c_uint = 1 << 2;
pub const DAV1D_X86_CPU_FLAG_AVX2: c_uint = 1 << 3;
pub const DAV1D_X86_CPU_FLAG_AVX512ICL: c_uint = 1 << 4;
pub const DAV1D_X86_CPU_FLAG_SLOW_GATHER: c_uint = 1 << 5;

/// `dav1d_get_cpu_flags()` (src/cpu.h, `TRIM_DSP_FUNCTIONS` 0): the
/// detected flags restricted by `dav1d_set_cpu_flags_mask`.
#[inline(always)]
pub fn dav1d_get_cpu_flags() -> c_uint {
    use crate::common::cpu::{dav1d_cpu_flags, dav1d_cpu_flags_mask};
    // SAFETY: plain reads of the translated C globals. They are written by
    // dav1d_init_cpu (once, under the library's pthread_once) and by
    // dav1d_set_cpu_flags_mask, which the original documents as to be called
    // before any decoder is opened, as the C reads of the same globals are.
    unsafe { core::ptr::read(&raw const dav1d_cpu_flags) & core::ptr::read(&raw const dav1d_cpu_flags_mask) }
}

/// `dav1d_get_cpu_flags_arm()` for AArch64: the baseline NEON flag.
#[cfg(all(target_arch = "aarch64", not(dav1d_portable)))]
pub fn dav1d_get_cpu_flags_native() -> c_uint {
    DAV1D_ARM_CPU_FLAG_NEON
}

/// The build configuration `--cfg dav1d_portable` is the original's
/// portable build (`-Denable_asm=false`, `HAVE_ASM` 0): `dav1d_init_cpu`
/// reports nothing there, so every DSP slot keeps its C kernel, and msac
/// calls its C functions (`simd/arm/msac.rs`).
#[cfg(dav1d_portable)]
pub fn dav1d_get_cpu_flags_native() -> c_uint {
    0
}

#[cfg(all(target_arch = "x86_64", not(dav1d_portable)))]
#[target_feature(enable = "xsave")]
unsafe fn xgetbv0() -> u64 {
    core::arch::x86_64::_xgetbv(0)
}

/// `dav1d_get_cpu_flags_x86()` (src/x86/cpu.c, ARCH_X86_64), statement by
/// statement; `dav1d_get_default_cpu_flags()` contributes SSE2.
#[cfg(all(target_arch = "x86_64", not(dav1d_portable)))]
pub fn dav1d_get_cpu_flags_native() -> c_uint {
    use core::arch::x86_64::__cpuid_count;
    let x = |reg: u32, mask: u32| reg & mask == mask;
    // SAFETY: CPUID is available on every x86-64 CPU.
    let leaf0 = unsafe { __cpuid_count(0, 0) };
    let max_leaf = leaf0.eax;
    let mut vendor = [0u8; 12];
    vendor[0..4].copy_from_slice(&leaf0.ebx.to_le_bytes());
    vendor[4..8].copy_from_slice(&leaf0.edx.to_le_bytes());
    vendor[8..12].copy_from_slice(&leaf0.ecx.to_le_bytes());
    let mut flags = DAV1D_X86_CPU_FLAG_SSE2;
    if max_leaf >= 1 {
        // SAFETY: leaf 1 exists (max_leaf >= 1).
        let r = unsafe { __cpuid_count(1, 0) };
        let family = ((r.eax >> 8) & 0x0f) + ((r.eax >> 20) & 0xff);
        if x(r.edx, 0x0600_8000) {
            flags |= DAV1D_X86_CPU_FLAG_SSE2;
            if x(r.ecx, 0x0000_0201) {
                flags |= DAV1D_X86_CPU_FLAG_SSSE3;
                if x(r.ecx, 0x0008_0000) {
                    flags |= DAV1D_X86_CPU_FLAG_SSE41;
                }
            }
        }
        if x(r.ecx, 0x1800_0000) {
            // SAFETY: OSXSAVE is set, so XGETBV is enabled.
            let xcr0 = unsafe { xgetbv0() } as u32;
            if x(xcr0, 0x0000_0006) && max_leaf >= 7 {
                // SAFETY: leaf 7 exists (max_leaf >= 7).
                let r = unsafe { __cpuid_count(7, 0) };
                if x(r.ebx, 0x0000_0128) {
                    flags |= DAV1D_X86_CPU_FLAG_AVX2;
                    if x(xcr0, 0x0000_00e0) && x(r.ebx, 0xd023_0000) && x(r.ecx, 0x0000_5f42) {
                        flags |= DAV1D_X86_CPU_FLAG_AVX512ICL;
                    }
                }
            }
        }
        if &vendor == b"AuthenticAMD" && flags & DAV1D_X86_CPU_FLAG_AVX2 != 0 && family <= 0x19 {
            flags |= DAV1D_X86_CPU_FLAG_SLOW_GATHER;
        }
    }
    flags
}

/// Serializes the unit tests that change the process-wide CPU mask.
#[cfg(test)]
pub static CPU_MASK_TEST_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
