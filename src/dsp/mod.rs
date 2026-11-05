//! Translation of libwebp's `src/dsp` directory.
//!
//! The portable DSP functions behind their C dispatch names and, from
//! wave 4, the SIMD variants behind runtime selection (ADR 0003). The SIMD
//! modules exist only on their target architectures.
//!
//! Each module translates the upstream file of the same name; its header names
//! the upstream files and the work package that owns it. WP0 fixed this file
//! and the module list in `tools/units.json`; work packages do not edit it.

pub mod alpha_processing;
#[cfg(target_arch = "aarch64")]
pub mod alpha_processing_neon;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod alpha_processing_sse2;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod alpha_processing_sse41;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod common_sse2;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod common_sse41;
pub mod cost;
#[cfg(target_arch = "aarch64")]
pub mod cost_neon;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod cost_sse2;
pub mod cpu;
pub mod dec;
pub mod dec_clip_tables;
#[cfg(target_arch = "aarch64")]
pub mod dec_neon;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod dec_sse2;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod dec_sse41;
#[expect(clippy::module_inception, reason = "keeps the upstream file name")]
pub mod dsp;
pub mod enc;
#[cfg(target_arch = "aarch64")]
pub mod enc_neon;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod enc_sse2;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod enc_sse41;
pub mod filters;
#[cfg(target_arch = "aarch64")]
pub mod filters_neon;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod filters_sse2;
pub mod lossless;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod lossless_avx2;
pub mod lossless_common;
pub mod lossless_enc;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod lossless_enc_avx2;
#[cfg(target_arch = "aarch64")]
pub mod lossless_enc_neon;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod lossless_enc_sse2;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod lossless_enc_sse41;
#[cfg(target_arch = "aarch64")]
pub mod lossless_neon;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod lossless_sse2;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod lossless_sse41;
#[cfg(target_arch = "aarch64")]
pub mod neon;
pub mod quant;
pub mod rescaler;
#[cfg(target_arch = "aarch64")]
pub mod rescaler_neon;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod rescaler_sse2;
pub mod ssim;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod ssim_sse2;
pub mod upsampling;
#[cfg(target_arch = "aarch64")]
pub mod upsampling_neon;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod upsampling_sse2;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod upsampling_sse41;
pub mod yuv;
#[cfg(target_arch = "aarch64")]
pub mod yuv_neon;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod yuv_sse2;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod yuv_sse41;
