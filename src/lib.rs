//! Mechanical libavif translation. The original C headers define the ABI and
//! caller obligations. Pointers, allocation ownership, and callbacks retain the
//! source contract; exported functions are unsafe rather than a safe Rust API.
//!
//! This translation currently targets macOS on AArch64. Other targets require
//! regeneration and independent ABI and behavioral validation.
#![feature(extern_types)]
#![cfg_attr(libavif_rs_asan, feature(sanitize))]
// Generated identifiers and temporaries intentionally match the C source.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(dead_code, unused_assignments, unused_mut)]

#[cfg(not(all(target_os = "macos", target_arch = "aarch64")))]
compile_error!("This generated configuration is validated only for aarch64-apple-darwin");

// Retain source-corresponding workspace dependency symbols in all artifacts.
extern crate sharpyuv as _;
extern crate dav1d as _;
extern crate aom as _;

include!("modules.rs");
