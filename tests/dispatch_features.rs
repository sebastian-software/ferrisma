//! The runtime selection record of ADR 0008: `webp::dsp::cpu::features()`
//! answers as the `Init` functions' queries do, follows the C ABI's
//! `VP8GetCPUInfo` pointer (`NULL` is `-noasm`: no x86 feature), and recomputes
//! its cache when the pointer changes. With `simd` on 64-bit ARM the pointer
//! is the translation's `armCPUInfo` (WP20); in the scalar build there it is
//! `NULL`, as in the reference configuration.

use core::ptr;
use std::sync::atomic::Ordering;

use webp::abi::decode::VP8_GET_CPU_INFO;
use webp::dsp::cpu::{Features, features};

#[test]
fn features_follow_the_query_and_the_noasm_pointer() {
    let detected = features();
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        assert_eq!(detected.sse2, is_x86_feature_detected!("sse2"));
        assert_eq!(detected.sse4_1, is_x86_feature_detected!("sse4.1"));
        assert_eq!(detected.avx2, is_x86_feature_detected!("avx2"));
    }
    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
    {
        assert!(!detected.sse2 && !detected.sse4_1 && !detected.avx2);
    }
    assert_eq!(detected.neon, cfg!(target_arch = "aarch64"));

    // The pointer of the configuration: the translation's query on x86 and,
    // with `simd`, on 64-bit ARM (`armCPUInfo`, WP20); `NULL` in the scalar
    // build on 64-bit ARM, as in the reference configuration.
    let installed = !VP8_GET_CPU_INFO.load(Ordering::Acquire).is_null();
    assert_eq!(
        installed,
        cfg!(any(
            target_arch = "x86",
            target_arch = "x86_64",
            all(feature = "simd", target_arch = "aarch64")
        ))
    );

    // The query's own kNEON answer, which `VP8EncDspCostInit` selects by:
    // `armCPUInfo` answers it in the NEON build, every other query does not.
    assert_eq!(
        detected.query_neon,
        cfg!(all(feature = "simd", target_arch = "aarch64"))
    );

    // -noasm: the tools set the pointer to NULL before any call; every x86
    // feature and the query's kNEON answer are off, NEON on AArch64 stays,
    // as upstream compiles it in.
    let previous = VP8_GET_CPU_INFO.swap(ptr::null_mut(), Ordering::AcqRel);
    let off = features();
    VP8_GET_CPU_INFO.store(previous, Ordering::Release);
    assert_eq!(
        off,
        Features {
            neon: detected.neon,
            ..Features::default()
        }
    );
    assert!(!off.query_neon);

    // The cache follows the pointer back.
    assert_eq!(features(), detected);
}
