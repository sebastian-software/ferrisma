//! A mechanical Rust port of the JPEG decoder in libjpeg-turbo 3.2.1.
//!
//! One module per upstream file, named after it, as `tools/units.json` maps
//! them; each module starts with the upstream file's leading comment block.
//! Work packages 1 (common infrastructure: types, errors and messages, the
//! memory manager, utilities), 2 (input and markers, up to
//! `jpeg_read_header`), 3 (entropy decoding), 4 (coefficients and IDCT), 5
//! (upsampling, color, buffers) and 6 (master control and the decompression
//! API) are translated, so every module of the decoder is. `docs/adr/` holds
//! the decisions that bind them, including the scope, and the tests show what
//! is verified.
//!
//! Rust programs decode through the safe API of [`decode`], re-exported here:
//! [`Decoder`] reads the header, EXIF data and ICC profile, and
//! [`Scanlines`] streams rows with scaling, cropping and skipping
//! (ADR 0006). The translated modules stay public for the tests and for
//! callers that need a part of the libjpeg API that the safe API omits.
//!
//! The default `simd` feature is upstream's `WITH_SIMD`: the decoder runs
//! the Neon kernels of work package 9 on AArch64 and the AVX2 and SSE2
//! kernels of work package 10 on x86-64, which `jsimd` dispatches (ADR 0007,
//! ADR 0008). Their memory accesses go through `neon` and `x86`, the only
//! modules with unsafe code besides `jsimd`'s calls of the kernels. Without
//! the feature the crate is the reference configuration and has no unsafe
//! code.
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
#[cfg(all(with_simd, target_arch = "x86_64"))]
pub mod jdcolext_avx2;
#[cfg(all(with_simd, target_arch = "aarch64"))]
pub mod jdcolext_neon;
#[cfg(all(with_simd, target_arch = "x86_64"))]
pub mod jdcolext_sse2;
pub mod jdcolor;
#[cfg(all(with_simd, target_arch = "x86_64"))]
pub mod jdcolor_avx2;
#[cfg(all(with_simd, target_arch = "aarch64"))]
pub mod jdcolor_neon;
#[cfg(all(with_simd, target_arch = "x86_64"))]
pub mod jdcolor_sse2;
pub mod jdct;
pub mod jddctmgr;
pub mod jdhuff;
pub mod jdicc;
pub mod jdinput;
pub mod jdmainct;
pub mod jdmarker;
pub mod jdmaster;
pub mod jdmerge;
#[cfg(all(with_simd, target_arch = "x86_64"))]
pub mod jdmerge_avx2;
#[cfg(all(with_simd, target_arch = "aarch64"))]
pub mod jdmerge_neon;
#[cfg(all(with_simd, target_arch = "x86_64"))]
pub mod jdmerge_sse2;
pub mod jdmrgext;
#[cfg(all(with_simd, target_arch = "x86_64"))]
pub mod jdmrgext_avx2;
#[cfg(all(with_simd, target_arch = "aarch64"))]
pub mod jdmrgext_neon;
#[cfg(all(with_simd, target_arch = "x86_64"))]
pub mod jdmrgext_sse2;
pub mod jdphuff;
pub mod jdpostct;
pub mod jdsample;
#[cfg(all(with_simd, target_arch = "x86_64"))]
pub mod jdsample_avx2;
#[cfg(all(with_simd, target_arch = "aarch64"))]
pub mod jdsample_neon;
#[cfg(all(with_simd, target_arch = "x86_64"))]
pub mod jdsample_sse2;
pub mod jerror;
pub mod jidctflt;
#[cfg(all(with_simd, target_arch = "x86_64"))]
pub mod jidctflt_sse2;
pub mod jidctfst;
#[cfg(all(with_simd, target_arch = "aarch64"))]
pub mod jidctfst_neon;
#[cfg(all(with_simd, target_arch = "x86_64"))]
pub mod jidctfst_sse2;
pub mod jidctint;
#[cfg(all(with_simd, target_arch = "x86_64"))]
pub mod jidctint_avx2;
#[cfg(all(with_simd, target_arch = "aarch64"))]
pub mod jidctint_neon;
#[cfg(all(with_simd, target_arch = "x86_64"))]
pub mod jidctint_sse2;
pub mod jidctred;
#[cfg(all(with_simd, target_arch = "aarch64"))]
pub mod jidctred_neon;
#[cfg(all(with_simd, target_arch = "x86_64"))]
pub mod jidctred_sse2;
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
#[cfg(all(with_simd, target_arch = "aarch64"))]
pub mod neon;
#[cfg(all(with_simd, target_arch = "x86_64"))]
pub mod x86;

pub use decode::{
    ColorSpace, DctMethod, Decoder, DecoderOptions, Error, Header, MAX_KEPT_WARNINGS, Message,
    OutputInfo, OutputOptions, PixelFormat, Report, Scale, Scanlines,
};
