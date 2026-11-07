//! Mechanical translation of the portable libaom 3.15.2 C archive, configured
//! without VMAF tuning (ADR 0057).
//!
//! # Safety
//! Pointer validity, record/buffer alignment, allocation extent, ownership,
//! callbacks, error state, and thread synchronization follow the original
//! headers in `upstream/aom`. These entry points retain those unsafe caller
//! contracts.
#![feature(core_intrinsics, extern_types, c_variadic)]
#![cfg_attr(libavif_rs_asan, feature(sanitize))]
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(dead_code, unused_assignments, unused_mut)]

#[cfg(panic = "abort")]
compile_error!("libaom's mapped nonlocal error handling requires panic=unwind");

#[cfg(not(any(all(target_os = "macos", target_arch = "aarch64"),
              all(target_os = "linux", target_arch = "x86_64", target_env = "gnu"))))]
compile_error!("The libaom translation profiles target macOS ARM64 and GNU Linux x86-64");

#[macro_use]
extern crate c2rust_bitfields;

mod platform;
mod jump;
pub mod simd;
#[cfg(feature = "port-macro-tests")]
mod test_macros;
#[cfg(all(target_os = "macos", debug_assertions))]
include!("modules-macos-debug.rs");
#[cfg(all(target_os = "macos", not(debug_assertions)))]
include!("modules-macos-release.rs");
#[cfg(all(target_os = "linux", debug_assertions))]
include!("modules-linux-debug.rs");
#[cfg(all(target_os = "linux", not(debug_assertions)))]
include!("modules-linux-release.rs");

/// Raw original API with one nominal Rust representation for public records.
pub mod raw {
    #[cfg(all(target_os = "macos", debug_assertions))]
    include!("api-macos-debug.rs");
    #[cfg(all(target_os = "macos", not(debug_assertions)))]
    include!("api-macos-release.rs");
    #[cfg(all(target_os = "linux", debug_assertions))]
    include!("api-linux-debug.rs");
    #[cfg(all(target_os = "linux", not(debug_assertions)))]
    include!("api-linux-release.rs");
}
