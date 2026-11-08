//! A mechanical Rust port of the JPEG decoder in libjpeg-turbo 3.2.1.
//!
//! One module per upstream file, named after it, as `tools/units.json` maps
//! them; each module starts with the upstream file's leading comment block.
//! Work packages 1 (common infrastructure: types, errors and messages, the
//! memory manager, utilities), 2 (input and markers, up to
//! `jpeg_read_header`), 3 (entropy decoding), 4 (coefficients and IDCT), 5
//! (upsampling, color, buffers) and 6 (master control and the decompression
//! API) are translated, so every module of the decoder is. See `docs/PORT_PLAN.md` for the scope and the
//! work packages, `docs/PORT_STATUS.md` for what is translated and verified,
//! and `docs/adr/` for the decisions that bind them.
//!
//! Rust programs decode through the safe API of [`decode`], re-exported here:
//! [`Decoder`] reads the header, EXIF data and ICC profile, and
//! [`Scanlines`] streams rows with scaling, cropping and skipping
//! (ADR 0006). The translated modules stay public for the tests and for
//! callers that need a part of the libjpeg API that the safe API omits.
//!
//! The default `simd` feature is upstream's `WITH_SIMD`: on AArch64 the
//! decoder runs the Neon kernels of work package 9, which `jsimd` dispatches
//! (ADR 0007). Their memory accesses go through `neon`, the only module with
//! unsafe code besides `jsimd`'s calls of the kernels. Without the feature
//! the crate is the reference configuration and has no unsafe code.
//!
//! This software is based in part on the work of the Independent JPEG Group.

#![deny(unsafe_code)]
#![warn(clippy::all, clippy::pedantic)]

pub mod decode;
pub mod jaricom;
pub mod jcomapi;
pub mod jconfig;
pub mod jdapimin;
pub mod jdapistd;
pub mod jdarith;
pub mod jdatasrc;
pub mod jdcoefct;
pub mod jdcolext;
#[cfg(with_simd)]
pub mod jdcolext_neon;
pub mod jdcolor;
#[cfg(with_simd)]
pub mod jdcolor_neon;
pub mod jdct;
pub mod jddctmgr;
pub mod jdhuff;
pub mod jdicc;
pub mod jdinput;
pub mod jdmainct;
pub mod jdmarker;
pub mod jdmaster;
pub mod jdmerge;
#[cfg(with_simd)]
pub mod jdmerge_neon;
pub mod jdmrgext;
#[cfg(with_simd)]
pub mod jdmrgext_neon;
pub mod jdphuff;
pub mod jdpostct;
pub mod jdsample;
#[cfg(with_simd)]
pub mod jdsample_neon;
pub mod jerror;
pub mod jidctflt;
pub mod jidctfst;
#[cfg(with_simd)]
pub mod jidctfst_neon;
pub mod jidctint;
#[cfg(with_simd)]
pub mod jidctint_neon;
pub mod jidctred;
#[cfg(with_simd)]
pub mod jidctred_neon;
pub mod jmemmgr;
pub mod jmemnobs;
pub mod jmemsys;
pub mod jmorecfg;
pub mod jpegint;
pub mod jpeglib;
#[cfg(with_simd)]
pub mod jsimd;
#[cfg(with_simd)]
pub mod jsimdcpu;
pub mod jstdhuff;
pub mod jutils;
pub mod jversion;
#[cfg(with_simd)]
pub mod neon;

pub use decode::{
    ColorSpace, DctMethod, Decoder, DecoderOptions, Error, Header, MAX_KEPT_WARNINGS, Message,
    OutputInfo, OutputOptions, PixelFormat, Report, Scale, Scanlines,
};
