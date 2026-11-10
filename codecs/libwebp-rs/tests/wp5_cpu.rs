//! WP5 differential test: `SharpYuvGetCPUInfo` of `sharpyuv/sharpyuv_cpu.c`, the
//! CPU feature query that the C file compiles a second time from `src/dsp/cpu.c`.
//!
//! The C probe `tools/oracle/probes/wp5/cpu.c` runs in its own process. On x86 the
//! reference configuration installs `x86CPUInfo`, and the probe answers for each
//! feature on the running CPU; the Rust query must give the same answers on the
//! same host. On other targets the reference configuration leaves the pointer NULL,
//! and the probe says so; the scalar Rust build then answers `false` for every
//! feature. On 64-bit ARM the `release-simd` oracle installs `armCPUInfo`, which
//! answers `kNEON` only, and so does the Rust query with the `simd` feature (WP20,
//! ADR 0008); each build is compared with the query of its own configuration.

#[path = "support/oracle.rs"]
mod oracle;

use oracle::Oracle;
use webp::dsp::cpu::{CPUFeature, VP8CPUInfo, vp8_get_cpu_info};
use webp::sharpyuv::sharpyuv_cpu::sharp_yuv_get_cpu_info;

/// Every `CPUFeature` enumerator in the order the probe prints them, followed by
/// the value 10, which is not a feature and must be answered `false`.
const FEATURES: [CPUFeature; 11] = [
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
    CPUFeature(10),
];

#[test]
fn sharp_yuv_cpu_query_matches_the_c_pointer() {
    let Some(oracle) = Oracle::locate("sharp_yuv_cpu_query_matches_the_c_pointer") else {
        return;
    };
    let probe = oracle.probe("wp5", "cpu");
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
                    sharp_yuv_get_cpu_info(feature),
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
                    !sharp_yuv_get_cpu_info(feature),
                    "feature {number} is reported without a C query"
                );
            }
        }
        0 => {
            for (number, feature) in FEATURES.into_iter().enumerate() {
                let expected = fields.int(&format!("feature{number}")) == 1;
                assert_eq!(
                    sharp_yuv_get_cpu_info(feature),
                    expected,
                    "SharpYuvGetCPUInfo({number}) differs from the C pointer on this host"
                );
            }
        }
        other => panic!("the probe printed null={other}"),
    }
}

#[test]
fn sharp_yuv_cpu_query_is_the_wp4_query() {
    // The C pointer has the `VP8CPUInfo` type; the Rust function must coerce to it.
    let query: VP8CPUInfo = sharp_yuv_get_cpu_info;
    for feature in FEATURES {
        assert_eq!(query(feature), vp8_get_cpu_info(feature));
    }
}
