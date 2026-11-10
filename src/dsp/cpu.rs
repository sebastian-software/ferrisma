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
//! answers for the running CPU as `x86CPUInfo` does. On 64-bit ARM with the
//! `simd` feature it answers `kNEON` as `armCPUInfo` does in upstream's NEON
//! build (WP20, ADR 0008): upstream defines `WEBP_HAVE_NEON` there, assigns
//! `armCPUInfo`, and without `WEBP_HAVE_NEON_RTCD`, which only 32-bit ARM
//! builds use for their `/proc/cpuinfo` test, the query reports NEON and
//! nothing else. Elsewhere, and in the scalar build, the reference
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
use std::sync::atomic::{AtomicU64, Ordering};

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

// upstream: src/dsp/cpu.c armCPUInfo
/// The ARM query of upstream's NEON build: NEON is reported, every other
/// feature is not. `WEBP_HAVE_NEON_RTCD`, the run-time test of 32-bit ARM
/// Linux builds, is not defined for 64-bit ARM.
#[cfg(all(feature = "simd", target_arch = "aarch64"))]
fn arm_cpu_info(feature: CPUFeature) -> bool {
    feature == CPUFeature::kNEON
}

// upstream: src/dsp/cpu.c VP8GetCPUInfo
/// Reports whether the running CPU supports `feature`. On 64-bit ARM with the
/// `simd` feature the answer is `armCPUInfo`'s: NEON only, as in upstream's
/// NEON build (ADR 0008).
#[cfg(all(feature = "simd", target_arch = "aarch64"))]
#[must_use]
pub fn vp8_get_cpu_info(feature: CPUFeature) -> bool {
    arm_cpu_info(feature)
}

// upstream: src/dsp/cpu.c VP8GetCPUInfo
/// Reports whether the running CPU supports `feature`. Without an x86 query,
/// the reference configuration's NULL pointer reports no feature.
#[cfg(not(any(
    target_arch = "x86",
    target_arch = "x86_64",
    all(feature = "simd", target_arch = "aarch64")
)))]
#[must_use]
pub fn vp8_get_cpu_info(_feature: CPUFeature) -> bool {
    false
}

//------------------------------------------------------------------------------
// Runtime selection of the SIMD variants (ADR 0008). No C counterpart as a
// function: C's `WEBP_DSP_INIT` re-runs an `Init` function when `VP8GetCPUInfo`
// changed since its last run, and the `Init` function assigns the dispatch
// pointers from the query's answers.

/// The CPU features that the dispatch functions select the SIMD variants by
/// (ADR 0008). Each x86 flag answers as the `Init` functions' queries do,
/// through the query behind the C ABI's `VP8GetCPUInfo` pointer: `false` for a
/// `NULL` pointer (`-noasm`), `cpuid` for the translation's own query, and the
/// caller's function for a query it installed. `neon` is true on 64-bit ARM
/// whatever the pointer says, where upstream compiles the NEON code in and
/// omits the C code it replaces (`WEBP_NEON_OMIT_C_CODE`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[expect(
    clippy::struct_excessive_bools,
    reason = "one flag per CPU feature, as the C query answers them"
)]
pub struct Features {
    /// `VP8GetCPUInfo(kSSE2)`.
    pub sse2: bool,
    /// `VP8GetCPUInfo(kSSE3)`.
    pub sse3: bool,
    /// `VP8GetCPUInfo(kSlowSSSE3)`.
    pub slow_ssse3: bool,
    /// `VP8GetCPUInfo(kSSE4_1)`.
    pub sse4_1: bool,
    /// `VP8GetCPUInfo(kAVX)`.
    pub avx: bool,
    /// `VP8GetCPUInfo(kAVX2)`.
    pub avx2: bool,
    /// NEON, always on 64-bit ARM.
    pub neon: bool,
}

impl Features {
    const SSE2: u32 = 1 << 0;
    const SSE3: u32 = 1 << 1;
    const SLOW_SSSE3: u32 = 1 << 2;
    const SSE4_1: u32 = 1 << 3;
    const AVX: u32 = 1 << 4;
    const AVX2: u32 = 1 << 5;
    const NEON: u32 = 1 << 6;
    /// Marks a computed record in the cache word.
    const VALID: u32 = 1 << 31;

    fn bits(self) -> u32 {
        let mut bits = 0;
        for (flag, bit) in [
            (self.sse2, Self::SSE2),
            (self.sse3, Self::SSE3),
            (self.slow_ssse3, Self::SLOW_SSSE3),
            (self.sse4_1, Self::SSE4_1),
            (self.avx, Self::AVX),
            (self.avx2, Self::AVX2),
            (self.neon, Self::NEON),
        ] {
            if flag {
                bits |= bit;
            }
        }
        bits
    }

    fn from_bits(bits: u32) -> Self {
        Self {
            sse2: bits & Self::SSE2 != 0,
            sse3: bits & Self::SSE3 != 0,
            slow_ssse3: bits & Self::SLOW_SSSE3 != 0,
            sse4_1: bits & Self::SSE4_1 != 0,
            avx: bits & Self::AVX != 0,
            avx2: bits & Self::AVX2 != 0,
            neon: bits & Self::NEON != 0,
        }
    }
}

/// The cached record: the token of the `VP8GetCPUInfo` pointer it was computed
/// for in the high word, the feature bits with [`Features::VALID`] in the low
/// word. One atomic word, so a reader never pairs a token with another
/// pointer's bits.
static FEATURES_CACHE: AtomicU64 = AtomicU64::new(0);

/// The features the dispatch functions select by (ADR 0008), computed on the
/// first call and again whenever the C ABI's `VP8GetCPUInfo` pointer has
/// changed since, as `WEBP_DSP_INIT` re-runs the `Init` functions then.
#[must_use]
pub fn features() -> Features {
    let token = crate::abi::decode::cpu_query_token();
    let cached = FEATURES_CACHE.load(Ordering::Relaxed);
    #[expect(
        clippy::cast_possible_truncation,
        reason = "the low word of the cache holds the feature bits"
    )]
    let bits = cached as u32;
    let cached_token = (cached >> 32) as u32;
    if cached_token == token && bits & Features::VALID != 0 {
        return Features::from_bits(bits);
    }
    let computed = compute_features();
    FEATURES_CACHE.store(
        (u64::from(token) << 32) | u64::from(computed.bits() | Features::VALID),
        Ordering::Relaxed,
    );
    computed
}

/// The record for the current `VP8GetCPUInfo` pointer.
fn compute_features() -> Features {
    let query =
        |feature: CPUFeature| crate::abi::decode::cpu_feature_query(feature).unwrap_or(false);
    Features {
        sse2: query(CPUFeature::kSSE2),
        sse3: query(CPUFeature::kSSE3),
        slow_ssse3: query(CPUFeature::kSlowSSSE3),
        sse4_1: query(CPUFeature::kSSE4_1),
        avx: query(CPUFeature::kAVX),
        avx2: query(CPUFeature::kAVX2),
        neon: cfg!(target_arch = "aarch64"),
    }
}
