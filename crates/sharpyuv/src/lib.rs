//! Mechanical portable SharpYUV 0.4.2 port from libwebp 1.6.0.
//!
//! # Safety
//! Exported functions retain the original unsafe C caller contracts: channel
//! pointers must be live, correctly aligned, and large enough for their declared
//! steps, strides, dimensions, and bit depths. Output planes must be writable.
//! Memory access and aliasing follow the original C contract. Matrices/options must be initialized
//! and live for the call. Allocation ownership and synchronization follow the
//! original source. This crate does not provide a checked safe API.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(dead_code, unused_assignments, unused_mut)]

#[cfg(not(any(all(target_os = "macos", target_arch = "aarch64"),
              all(target_os = "linux", target_arch = "x86_64"))))]
compile_error!("The portable source mapping targets macOS ARM64 and Linux x86-64");

mod sharpyuv;
mod sharpyuv_cpu;
pub mod sharpyuv_csp;
mod sharpyuv_dsp;
mod sharpyuv_gamma;
mod sharpyuv_neon;
mod sharpyuv_sse2;
mod platform;

pub use sharpyuv::*;
pub use sharpyuv_csp::{SharpYuvColorSpace, SharpYuvRange, SharpYuvMatrixType,
    SharpYuvComputeConversionMatrix, SharpYuvGetConversionMatrix,
    kSharpYuvRangeFull, kSharpYuvRangeLimited, kSharpYuvMatrixWebp,
    kSharpYuvMatrixRec601Limited, kSharpYuvMatrixRec601Full,
    kSharpYuvMatrixRec709Limited, kSharpYuvMatrixRec709Full};

/// Mechanically retained inline wrapper from upstream `sharpyuv.h`.
///
/// Copyright 2022 Google Inc. BSD terms and patent grant: `LICENSES/`.
///
/// # Safety
/// `matrix` must point to a live conversion matrix and `options` to writable,
/// aligned storage. Both pointers follow the original C header contract.
#[inline]
pub unsafe fn SharpYuvOptionsInit(matrix: *const SharpYuvConversionMatrix,
                                 options: *mut SharpYuvOptions) -> core::ffi::c_int {
    // SAFETY: The caller supplies the same live storage as the C inline wrapper.
    SharpYuvOptionsInitInternal(matrix, options, SHARPYUV_VERSION)
}
