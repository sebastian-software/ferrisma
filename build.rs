//! Sets `cfg(with_simd)`, the port's `WITH_SIMD`, when the `simd` feature is
//! on and the port has kernels for the target architecture (ADR 0007). On
//! other architectures `WITH_SIMD` stays undefined, as upstream's CMake falls
//! back to a build without SIMD extensions there.

/// The architectures whose kernels are translated.
const SIMD_ARCHITECTURES: &[&str] = &["aarch64"];

fn main() {
    println!("cargo::rustc-check-cfg=cfg(with_simd)");
    println!("cargo::rerun-if-changed=build.rs");
    let simd = std::env::var_os("CARGO_FEATURE_SIMD").is_some();
    let architecture = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    if simd && SIMD_ARCHITECTURES.contains(&architecture.as_str()) {
        println!("cargo::rustc-cfg=with_simd");
    }
}
