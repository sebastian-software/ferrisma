//! Mechanical libavif translation. The original C headers define the ABI and
//! caller obligations. Pointers, allocation ownership, and callbacks retain the
//! source contract; exported functions are unsafe rather than a safe Rust API.
//!
//! This translation currently targets macOS on AArch64. Other targets require
//! regeneration and independent ABI and behavioral validation.
#![feature(extern_types)]
// Generated identifiers and temporaries intentionally match the C source.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(dead_code, unused_assignments, unused_mut)]

#[cfg(not(all(target_os = "macos", target_arch = "aarch64")))]
compile_error!("This generated configuration is validated only for aarch64-apple-darwin");

pub mod alpha;
pub mod avif;
pub mod codec_aom;
pub mod codec_dav1d;
pub mod colr;
pub mod colrconvert;
pub mod diag;
pub mod exif;
pub mod gainmap;
pub mod io;
pub mod mem;
pub mod obu;
pub mod properties;
pub mod rawdata;
pub mod read;
pub mod reformat;
pub mod reformat_libsharpyuv;
pub mod reformat_libyuv;
pub mod sampletransform;
pub mod scale;
pub mod stream;
pub mod utils;
pub mod write;

pub mod libyuv {
    pub mod planar_functions;
    pub mod row_common;
    pub mod scale;
    pub mod scale_any;
    pub mod scale_common;
}
