//! The runtime selection record of ADR 0008: `webp::dsp::cpu::features()`
//! answers as the `Init` functions' queries do, follows the C ABI's
//! `VP8GetCPUInfo` pointer (`NULL` is `-noasm`: no x86 feature), and recomputes
//! its cache when the pointer changes. With `simd` on 64-bit ARM the pointer
//! is the translation's `armCPUInfo` (WP20); in the scalar build there it is
//! `NULL`, as in the reference configuration.

use std::sync::atomic::{AtomicUsize, Ordering};

use webp::abi::decode::{
    cpu_query_is_installed, disable_cpu_query, set_cpu_query_callback, use_builtin_cpu_query,
};
use webp::dsp::cpu::{CPUFeature, Features, features};

static QUERY_CALLS: AtomicUsize = AtomicUsize::new(0);

extern "C" fn no_features(_: CPUFeature) -> i32 {
    QUERY_CALLS.fetch_add(1, Ordering::Relaxed);
    0
}

/// The gate level of `tools/check.py --oracle` (ADR 0008), with the `oracle`
/// feature only: `LIBWEBP_SIMD_LEVEL` caps the query, and the host must
/// report at least that level, or the mode would silently compare a lower one
/// (on Rosetta 2, AVX2 needs `ROSETTA_ADVERTISE_AVX=1`).
#[cfg(feature = "oracle")]
fn gate_level() -> Option<String> {
    std::env::var("LIBWEBP_SIMD_LEVEL")
        .ok()
        .filter(|level| !level.is_empty())
}

#[cfg(feature = "oracle")]
#[test]
fn the_gate_level_is_the_selected_level() {
    let Some(level) = gate_level() else {
        return;
    };
    let selected = features();
    let expected = match level.as_str() {
        "neon" => selected.neon && !selected.sse2,
        "avx2" => selected.sse2 && selected.sse4_1 && selected.avx2,
        "sse41" => selected.sse2 && selected.sse4_1 && !selected.avx && !selected.avx2,
        "sse2" => selected.sse2 && !selected.sse3 && !selected.sse4_1 && !selected.avx2,
        _ => panic!("LIBWEBP_SIMD_LEVEL={level}: unknown level"),
    };
    assert!(
        expected,
        "LIBWEBP_SIMD_LEVEL={level}, but the dispatch selects by {selected:?}"
    );
}

#[test]
fn features_follow_the_query_and_the_noasm_pointer() {
    let detected = features();
    #[cfg(feature = "oracle")]
    if gate_level().is_some() {
        // The capped record is checked by the_gate_level_is_the_selected_level.
        return;
    }
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
    let installed = cpu_query_is_installed();
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
    disable_cpu_query();
    let off = features();
    use_builtin_cpu_query();
    assert_eq!(
        off,
        Features {
            neon: detected.neon,
            ..Features::default()
        }
    );
    assert!(!off.query_neon);

    // A typed custom query is called by the dispatcher and can select the
    // scalar path without handing safe code a raw writable pointer.
    // SAFETY: this static callback never claims an unsupported feature and
    // this test binary does not dispatch concurrently.
    unsafe { set_cpu_query_callback(no_features) };
    assert_eq!(features(), off);
    assert!(QUERY_CALLS.load(Ordering::Relaxed) > 0);
    use_builtin_cpu_query();

    // The cache follows the pointer back.
    assert_eq!(features(), detected);
}
