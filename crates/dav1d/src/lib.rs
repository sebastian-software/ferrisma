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

#[cfg(not(all(target_os = "macos", target_arch = "aarch64")))]
compile_error!("The dav1d translation is being validated on macOS ARM64 first");

#[macro_use]
extern crate c2rust_bitfields;

mod platform;
#[cfg(debug_assertions)]
include!("modules.rs");
#[cfg(not(debug_assertions))]
include!("modules-release.rs");

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
