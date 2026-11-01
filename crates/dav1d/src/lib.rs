//! Mechanical portable dav1d 1.5.4 translation.
//!
//! # Safety
//! The original headers define pointer validity, alignment, allocation sizes,
//! packet/frame ownership, callback lifetimes, and synchronization requirements.
//! Exported entry points retain those unsafe caller contracts. No safe wrapper
//! or alternative decoder algorithm is introduced by this crate.
#![feature(core_intrinsics, extern_types)]
#![cfg_attr(libavif_rs_asan, feature(sanitize))]
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(dead_code, unused_assignments, unused_mut)]

#[cfg(not(any(all(target_os = "macos", target_arch = "aarch64"),
              all(target_os = "linux", target_arch = "x86_64", target_env = "gnu"))))]
compile_error!("The recorded dav1d profiles target macOS ARM64 and GNU Linux x86-64");

#[macro_use]
extern crate c2rust_bitfields;

mod platform;
#[cfg(all(target_os = "macos", debug_assertions))]
include!("modules.rs");
#[cfg(all(target_os = "macos", not(debug_assertions)))]
include!("modules-release.rs");
#[cfg(all(target_os = "linux", debug_assertions))]
include!("modules-linux.rs");
#[cfg(all(target_os = "linux", not(debug_assertions)))]
include!("modules-linux-release.rs");

pub use common::decoder::*;
pub use common::cpu::dav1d_set_cpu_flags_mask;

extern "C" {
    /// Original sequence-header API implemented by the translated OBU unit.
    ///
    /// # Safety
    /// `out` must be live aligned writable storage. `buffer` must contain `size`
    /// readable bytes. The public record layout is checked against original C;
    /// this declaration gives Rust callers the same public type as the other APIs.
    pub fn dav1d_parse_sequence_header(out: *mut Dav1dSequenceHeader,
                                      buffer: *const u8, size: usize) -> core::ffi::c_int;
}
