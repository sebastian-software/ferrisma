//! Mechanical libavif translation. The original C headers define the ABI and
//! caller obligations. Pointers, allocation ownership, and callbacks retain the
//! source contract; exported functions are unsafe rather than a safe Rust API.
//!
//! This translation targets macOS on AArch64 and, through generated native
//! profiles, GNU Linux on x86-64 for the feature sets with production evidence.
//! Other targets require regeneration and independent ABI and behavioral
//! validation.
#![feature(extern_types)]
#![cfg_attr(libavif_rs_asan, feature(sanitize))]
// Generated identifiers and temporaries intentionally match the C source.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(dead_code, unused_assignments, unused_mut)]

#[cfg(not(any(
    all(target_os = "macos", target_arch = "aarch64"),
    all(target_os = "linux", target_arch = "x86_64", target_env = "gnu"),
)))]
compile_error!("This generated configuration is validated only for aarch64-apple-darwin and x86_64-unknown-linux-gnu");

// Retain source-corresponding workspace dependency symbols in all artifacts.
extern crate sharpyuv as _;
extern crate dav1d as _;
extern crate aom as _;

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
include!("modules.rs");

// GNU Linux compiles the generated profile of its identical original C build.
#[cfg(all(target_os = "linux", target_arch = "x86_64", target_env = "gnu"))]
include!("linux_profiles.rs");
