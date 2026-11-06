// libwebp v1.6.0, src/dsp/cpu.c:
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
// CPU detection
//
// Author: Christian Duvivier (cduvivier@google.com)

// libwebp v1.6.0, src/dsp/cpu.h:
//
// Copyright 2022 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
//   CPU detection functions and macros.
//
// Author: Skal (pascal.massimino@gmail.com)

//! Translation of `src/dsp/cpu.c` and `src/dsp/cpu.h` from libwebp v1.6.0.
//!
//! Work package: WP4 (pixel ops).
//!
//! `VP8GetCPUInfo` is a query, not a function pointer (ADR 0003). On x86 it
//! answers for the running CPU as `x86CPUInfo` does. Elsewhere the reference
//! configuration leaves the pointer NULL, and the query answers `false`.
//! The `WEBP_DSP_INIT` guards of `cpu.h` have no counterpart: they protect
//! pointer assignments, which ADR 0003 omits. The compile-time macros of
//! `cpu.h` (`WEBP_USE_SSE2` and the other `WEBP_HAVE_*` tests, the sanitizer
//! and byte-order macros) become `cfg` or are not compiled in the reference
//! configuration, which defines no SIMD, uses no sanitizer and is
//! little-endian.

#[cfg(target_arch = "x86")]
use core::arch::x86::__cpuid_count;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::__cpuid_count;

// upstream: src/dsp/cpu.h CPUFeature
/// A CPU feature that the query reports on. The C enumeration has no negative
/// values, so its type is `unsigned int` (ADR 0003).
#[must_use]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CPUFeature(pub u32);

#[expect(
    non_upper_case_globals,
    reason = "C enumerator names such as kSSE2 keep their case"
)]
impl CPUFeature {
    // upstream: src/dsp/cpu.h kSSE2
    pub const kSSE2: Self = Self(0);
    // upstream: src/dsp/cpu.h kSSE3
    pub const kSSE3: Self = Self(1);
    // upstream: src/dsp/cpu.h kSlowSSSE3
    pub const kSlowSSSE3: Self = Self(2);
    // upstream: src/dsp/cpu.h kSSE4_1
    pub const kSSE4_1: Self = Self(3);
    // upstream: src/dsp/cpu.h kAVX
    pub const kAVX: Self = Self(4);
    // upstream: src/dsp/cpu.h kAVX2
    pub const kAVX2: Self = Self(5);
    // upstream: src/dsp/cpu.h kNEON
    pub const kNEON: Self = Self(6);
    // upstream: src/dsp/cpu.h kMIPS32
    pub const kMIPS32: Self = Self(7);
    // upstream: src/dsp/cpu.h kMIPSdspR2
    pub const kMIPSdspR2: Self = Self(8);
    // upstream: src/dsp/cpu.h kMSA
    pub const kMSA: Self = Self(9);
}

// upstream: src/dsp/cpu.h VP8CPUInfo
/// The signature of the feature query: whether the CPU supports the feature.
pub type VP8CPUInfo = fn(CPUFeature) -> bool;

// upstream: src/dsp/cpu.c GetCPUInfo
/// Runs `cpuid` with `info_type` in EAX and ECX zero, and returns EAX, EBX,
/// ECX and EDX.
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn get_cpu_info(info_type: u32) -> [u32; 4] {
    let result = __cpuid_count(info_type, 0);
    [result.eax, result.ebx, result.ecx, result.edx]
}

// upstream: src/dsp/cpu.c CheckSlowModel
/// Whether the display model in `info`, the EAX value of `cpuid` leaf 1, is
/// one of the Intel models with the longer latencies of `bsr` and of some
/// SSSE3 instructions such as `pshufb`.
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[must_use]
pub fn check_slow_model(info: u32) -> bool {
    // Table listing display models with longer latencies for the bsr instruction
    // (ie 2 cycles vs 10/16 cycles) and some SSSE3 instructions like pshufb.
    // Refer to Intel 64 and IA-32 Architectures Optimization Reference Manual.
    const SLOW_MODELS: [u32; 6] = [
        0x37, 0x4a, 0x4d, // Silvermont Microarchitecture
        0x1c, 0x26, 0x27, // Atom Microarchitecture
    ];
    let model = ((info & 0xf_0000) >> 12) | ((info >> 4) & 0xf);
    let family = (info >> 8) & 0xf;
    if family == 0x06 {
        return SLOW_MODELS.contains(&model);
    }
    false
}

// upstream: src/dsp/cpu.c x86CPUInfo
/// The x86 query. The reference configuration installs it on x86 targets.
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[expect(
    clippy::cast_possible_wrap,
    reason = "cpuid returns the highest leaf in the EAX of the C int that is compared with 1 and 7"
)]
fn x86cpu_info(feature: CPUFeature) -> bool {
    const VENDOR_ID_INTEL_EBX: u32 = 0x756e_6547; // uneG
    const VENDOR_ID_INTEL_EDX: u32 = 0x4965_6e69; // Ieni
    const VENDOR_ID_INTEL_ECX: u32 = 0x6c65_746e; // letn
    // get the highest feature value cpuid supports
    let cpu_info = get_cpu_info(0);
    let max_cpuid_value = cpu_info[0] as i32;
    if max_cpuid_value < 1 {
        return false;
    }
    let is_intel = cpu_info[1] == VENDOR_ID_INTEL_EBX
        && cpu_info[2] == VENDOR_ID_INTEL_ECX
        && cpu_info[3] == VENDOR_ID_INTEL_EDX; // genuine Intel?

    let cpu_info = get_cpu_info(1);
    if feature == CPUFeature::kSSE2 {
        return cpu_info[3] & (1 << 26) != 0;
    }
    if feature == CPUFeature::kSSE3 {
        return cpu_info[2] & (1 << 0) != 0;
    }
    if feature == CPUFeature::kSlowSSSE3 {
        if is_intel && cpu_info[2] & (1 << 9) != 0 {
            // SSSE3?
            return check_slow_model(cpu_info[0]);
        }
        return false;
    }
    if feature == CPUFeature::kSSE4_1 {
        return cpu_info[2] & (1 << 19) != 0;
    }
    if feature == CPUFeature::kAVX {
        // bits 27 (OSXSAVE) & 28 (256-bit AVX)
        if cpu_info[2] & 0x1800_0000 == 0x1800_0000 {
            // XMM state and YMM state enabled by the OS. The C code reads XCR0
            // with xgetbv and tests bits 1 and 2; the standard library's AVX
            // detection performs that same test, and it is safe Rust.
            return std::is_x86_feature_detected!("avx");
        }
    }
    if feature == CPUFeature::kAVX2 && x86cpu_info(CPUFeature::kAVX) && max_cpuid_value >= 7 {
        let cpu_info = get_cpu_info(7);
        return cpu_info[1] & (1 << 5) != 0;
    }
    false
}

// upstream: src/dsp/cpu.c VP8GetCPUInfo
/// Reports whether the running CPU supports `feature`.
///
/// On x86 the answer comes from `cpuid`, as in `x86CPUInfo`. On other targets
/// the reference configuration has no query, `VP8GetCPUInfo` is NULL, and the
/// answer is `false` for every feature. The NULL pointer itself exists only at
/// the C ABI (ADR 0004).
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[must_use]
pub fn vp8_get_cpu_info(feature: CPUFeature) -> bool {
    x86cpu_info(feature)
}

// upstream: src/dsp/cpu.c VP8GetCPUInfo
/// Reports whether the running CPU supports `feature`. Without an x86 query,
/// the reference configuration's NULL pointer reports no feature.
#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
#[must_use]
pub fn vp8_get_cpu_info(_feature: CPUFeature) -> bool {
    false
}
