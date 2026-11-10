// libjpeg-turbo 3.2.1, simd/arm/aarch64/jsimdcpu.c:
//
// /*
//  * Copyright (C) 2025, D. R. Commander.
//  *
//  * This software is provided 'as-is', without any express or implied
//  * warranty.  In no event will the authors be held liable for any damages
//  * arising from the use of this software.
//  *
//  * Permission is granted to anyone to use this software for any purpose,
//  * including commercial applications, and to alter it and redistribute it
//  * freely, subject to the following restrictions:
//  *
//  * 1. The origin of this software must not be misrepresented; you must not
//  *    claim that you wrote the original software. If you use this software
//  *    in a product, an acknowledgment in the product documentation would be
//  *    appreciated but is not required.
//  * 2. Altered source versions must be plainly marked as such, and must not be
//  *    misrepresented as being the original software.
//  * 3. This notice may not be removed or altered from any source distribution.
//  *
//  * This file contains the interface between the "normal" portions
//  * of the library and the SIMD implementations when running on a
//  * 64-bit Arm architecture.
//  */
// libjpeg-turbo 3.2.1, simd/x86_64/jsimdcpu.asm:
//
// ;
// ; SIMD instruction support check
// ;
// ; Copyright 2009 Pierre Ossman <ossman@cendio.se> for Cendio AB
// ; Copyright (C) 2016, D. R. Commander.
// ; Copyright (C) 2023, Aliaksiej Kandracienka.
// ;
// ; Based on
// ; x86 SIMD extension for IJG JPEG library
// ; Copyright (C) 1999-2006, MIYASAKA Masaru.
// ; For conditions of distribution and use, see copyright notice in jsimdext.inc
// ;
// ; This file should be assembled with NASM (Netwide Assembler) or Yasm.
//! Translation of `simd/arm/aarch64/jsimdcpu.c` and `simd/x86_64/jsimdcpu.asm` from libjpeg-turbo 3.2.1.
//!
//! Work package: 9 (NEON SIMD).

//!
//! Each upstream file serves one architecture, and its translation is
//! compiled only for it: the AArch64 file since work package 9, the x86-64
//! file since work package 10. On x86-64, `std::is_x86_feature_detected!`
//! replaces the CPUID and XGETBV instructions of `jsimdcpu.asm` (ADR 0004,
//! ADR 0008, decision 5).

#[cfg(target_arch = "aarch64")]
use crate::jsimd::JSIMD_NEON;
#[cfg(target_arch = "x86_64")]
use crate::jsimd::{JSIMD_AVX2, JSIMD_SSE, JSIMD_SSE2};

// upstream: simd/arm/aarch64/jsimdcpu.c jpeg_simd_cpu_support
/// The SIMD instruction sets of the CPU.
#[cfg(target_arch = "aarch64")]
#[must_use]
pub fn jpeg_simd_cpu_support() -> u32 {
    /* Armv8 architectures support Neon instructions by default.  They are no
     * longer optional as they were with Armv7.
     */
    JSIMD_NEON
}

// upstream: simd/x86_64/jsimdcpu.asm jpeg_simd_cpu_support
/// Check if the CPU supports SIMD instructions
///
/// The asm's tests map onto the detection of the standard library:
/// `is_x86_feature_detected!("avx2")` tests the AVX2 bit of CPUID leaf 7,
/// and `is_x86_feature_detected!("avx")` the OSXSAVE and AVX bits of leaf 1
/// and the XMM and YMM state that XGETBV reports. Both also check that the
/// CPU has the leaves.
#[cfg(target_arch = "x86_64")]
#[must_use]
pub fn jpeg_simd_cpu_support() -> u32 {
    let mut simd_support = 0; /* simd support flag */

    /* Assume that all x86-64 processors support SSE & SSE2 instructions */
    simd_support |= JSIMD_SSE2;
    simd_support |= JSIMD_SSE;

    /* Check for AVX2 instruction support and for AVX2 O/S support */
    if std::is_x86_feature_detected!("avx2") && std::is_x86_feature_detected!("avx") {
        simd_support |= JSIMD_AVX2;
    }

    simd_support
}
