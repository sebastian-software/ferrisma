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
//! Translation of `simd/arm/aarch64/jsimdcpu.c` from libjpeg-turbo 3.2.1.
//!
//! Work package: 9 (NEON SIMD).

use crate::jsimd::JSIMD_NEON;

// upstream: simd/arm/aarch64/jsimdcpu.c jpeg_simd_cpu_support
/// The SIMD instruction sets of the CPU.
#[must_use]
pub fn jpeg_simd_cpu_support() -> u32 {
    /* Armv8 architectures support Neon instructions by default.  They are no
     * longer optional as they were with Armv7.
     */
    JSIMD_NEON
}
