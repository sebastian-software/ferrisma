//! WP4 differential test: the CPU feature query `VP8GetCPUInfo` and the model
//! table of `CheckSlowModel` (`src/dsp/cpu.c` and `src/dsp/cpu.h`, ADR 0003).
//!
//! The C probe `tools/oracle/probes/wp4/cpu.c` runs in its own process. On
//! x86 the reference configuration installs `x86CPUInfo`, and the probe answers
//! for each of the ten features on the running CPU; the Rust query must give
//! the same answers on the same host. On other targets the reference
//! configuration leaves the pointer NULL, and the probe says so; the scalar
//! Rust build then answers `false` for every feature. On 64-bit ARM the
//! `release-simd` oracle installs `armCPUInfo`, which answers `kNEON` only, and
//! so does the Rust query with the `simd` feature (WP20, ADR 0008); against
//! the reference oracle that build is compared with `armCPUInfo`'s answers,
//! and the scalar build against `release-simd` with the NULL pointer's. On x86
//! the probe also prints `CheckSlowModel` for a grid of `cpuid` leaf-1 EAX
//! values, which the Rust helper must match.

#[path = "support/oracle.rs"]
mod oracle;

use oracle::Oracle;
use webp::dsp::cpu::{CPUFeature, vp8_get_cpu_info};

/// Every `CPUFeature` enumerator, in the order the probe prints them.
const FEATURES: [CPUFeature; 10] = [
    CPUFeature::kSSE2,
    CPUFeature::kSSE3,
    CPUFeature::kSlowSSSE3,
    CPUFeature::kSSE4_1,
    CPUFeature::kAVX,
    CPUFeature::kAVX2,
    CPUFeature::kNEON,
    CPUFeature::kMIPS32,
    CPUFeature::kMIPSdspR2,
    CPUFeature::kMSA,
];

#[test]
fn cpu_query_matches_the_c_query() {
    let Some(oracle) = Oracle::locate("cpu_query_matches_the_c_query") else {
        return;
    };
    let probe = oracle.probe("wp4", "cpu");
    let no_args: [&str; 0] = [];
    let output = probe.run(&no_args, b"");
    let fields = output.success().fields();
    // The NEON build of upstream on 64-bit ARM: `armCPUInfo` reports NEON only.
    let neon_build = cfg!(all(feature = "simd", target_arch = "aarch64"));
    let arm_cpu_info = |feature: CPUFeature| feature == CPUFeature::kNEON;
    match fields.int("null") {
        1 => {
            // The reference configuration has no query on this target; the
            // Rust NEON build answers as the NEON build's `armCPUInfo` does.
            for (number, feature) in FEATURES.into_iter().enumerate() {
                assert_eq!(
                    vp8_get_cpu_info(feature),
                    neon_build && arm_cpu_info(feature),
                    "feature {number} differs from the configuration's query"
                );
            }
        }
        0 if cfg!(target_arch = "aarch64") && !neon_build => {
            // The `release-simd` oracle's `armCPUInfo`, against the scalar
            // Rust build, whose reference configuration has no query.
            for (number, feature) in FEATURES.into_iter().enumerate() {
                assert_eq!(
                    fields.int(&format!("feature{number}")) == 1,
                    arm_cpu_info(feature),
                    "the C query of the NEON build reports feature {number}"
                );
                assert!(
                    !vp8_get_cpu_info(feature),
                    "feature {number} is reported without a C query"
                );
            }
        }
        0 => {
            for (number, feature) in FEATURES.into_iter().enumerate() {
                let expected = fields.int(&format!("feature{number}")) == 1;
                assert_eq!(
                    vp8_get_cpu_info(feature),
                    expected,
                    "VP8GetCPUInfo({number}) differs from the C query on this host"
                );
            }
        }
        other => panic!("the probe printed null={other}"),
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[test]
fn slow_model_table_matches_the_c_helper() {
    let Some(oracle) = Oracle::locate("slow_model_table_matches_the_c_helper") else {
        return;
    };
    let probe = oracle.probe("wp4", "cpu");
    let no_args: [&str; 0] = [];
    let output = probe.run(&no_args, b"");
    let fields = output.success().fields();
    let mut checked = 0_u32;
    // The probe's grid: families 0, 3, 6, 9, 12 and 15, four extended models
    // and sixteen low model nibbles, as cpuid leaf 1 EAX encodes them.
    for family in [0_u32, 3, 6, 9, 12, 15] {
        for ext in 0..4_u32 {
            for low in 0..16_u32 {
                let info = (ext << 16) | (family << 8) | (low << 4);
                let expected = fields.int(&format!("slow_{info}")) == 1;
                assert_eq!(
                    webp::dsp::cpu::check_slow_model(info),
                    expected,
                    "CheckSlowModel({info:#x}) differs from the C helper"
                );
                checked += 1;
            }
        }
    }
    assert_eq!(checked, 384, "the grid has 6 families x 4 x 16 values");
}
