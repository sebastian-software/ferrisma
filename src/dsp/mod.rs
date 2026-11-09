//! Translation of libwebp's `src/dsp` directory.
//!
//! The portable DSP functions behind their C dispatch names and, from
//! wave 4, the SIMD variants behind runtime selection (ADR 0003, ADR 0008). The
//! SIMD modules exist only on their target architectures and with the `simd`
//! feature.
//!
//! Each module translates the upstream file of the same name; its header names
//! the upstream files and the work package that owns it. WP0 fixed this file
//! and the module list in `tools/units.json`; work packages do not edit it.

pub mod alpha_processing;
#[cfg(all(feature = "simd", target_arch = "aarch64"))]
#[allow(unsafe_code, reason = "ADR 0008: SIMD intrinsics")]
#[deny(clippy::undocumented_unsafe_blocks)]
#[deny(clippy::multiple_unsafe_ops_per_block)]
pub mod alpha_processing_neon;
#[cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]
#[allow(unsafe_code, reason = "ADR 0008: SIMD intrinsics")]
#[deny(clippy::undocumented_unsafe_blocks)]
#[deny(clippy::multiple_unsafe_ops_per_block)]
pub mod alpha_processing_sse2;
#[cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]
#[allow(unsafe_code, reason = "ADR 0008: SIMD intrinsics")]
#[deny(clippy::undocumented_unsafe_blocks)]
#[deny(clippy::multiple_unsafe_ops_per_block)]
pub mod alpha_processing_sse41;
#[cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]
#[allow(unsafe_code, reason = "ADR 0008: SIMD intrinsics")]
#[deny(clippy::undocumented_unsafe_blocks)]
#[deny(clippy::multiple_unsafe_ops_per_block)]
pub mod common_sse2;
#[cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]
#[allow(unsafe_code, reason = "ADR 0008: SIMD intrinsics")]
#[deny(clippy::undocumented_unsafe_blocks)]
#[deny(clippy::multiple_unsafe_ops_per_block)]
pub mod common_sse41;
pub mod cost;
#[cfg(all(feature = "simd", target_arch = "aarch64"))]
#[allow(unsafe_code, reason = "ADR 0008: SIMD intrinsics")]
#[deny(clippy::undocumented_unsafe_blocks)]
#[deny(clippy::multiple_unsafe_ops_per_block)]
pub mod cost_neon;
#[cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]
#[allow(unsafe_code, reason = "ADR 0008: SIMD intrinsics")]
#[deny(clippy::undocumented_unsafe_blocks)]
#[deny(clippy::multiple_unsafe_ops_per_block)]
pub mod cost_sse2;
pub mod cpu;
pub mod dec;
pub mod dec_clip_tables;
#[cfg(all(feature = "simd", target_arch = "aarch64"))]
#[allow(unsafe_code, reason = "ADR 0008: SIMD intrinsics")]
#[deny(clippy::undocumented_unsafe_blocks)]
#[deny(clippy::multiple_unsafe_ops_per_block)]
pub mod dec_neon;
#[cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]
#[allow(unsafe_code, reason = "ADR 0008: SIMD intrinsics")]
#[deny(clippy::undocumented_unsafe_blocks)]
#[deny(clippy::multiple_unsafe_ops_per_block)]
pub mod dec_sse2;
#[cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]
#[allow(unsafe_code, reason = "ADR 0008: SIMD intrinsics")]
#[deny(clippy::undocumented_unsafe_blocks)]
#[deny(clippy::multiple_unsafe_ops_per_block)]
pub mod dec_sse41;
#[expect(clippy::module_inception, reason = "keeps the upstream file name")]
pub mod dsp;
pub mod enc;
#[cfg(all(feature = "simd", target_arch = "aarch64"))]
#[allow(unsafe_code, reason = "ADR 0008: SIMD intrinsics")]
#[deny(clippy::undocumented_unsafe_blocks)]
#[deny(clippy::multiple_unsafe_ops_per_block)]
pub mod enc_neon;
#[cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]
#[allow(unsafe_code, reason = "ADR 0008: SIMD intrinsics")]
#[deny(clippy::undocumented_unsafe_blocks)]
#[deny(clippy::multiple_unsafe_ops_per_block)]
pub mod enc_sse2;
#[cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]
#[allow(unsafe_code, reason = "ADR 0008: SIMD intrinsics")]
#[deny(clippy::undocumented_unsafe_blocks)]
#[deny(clippy::multiple_unsafe_ops_per_block)]
pub mod enc_sse41;
pub mod filters;
#[cfg(all(feature = "simd", target_arch = "aarch64"))]
#[allow(unsafe_code, reason = "ADR 0008: SIMD intrinsics")]
#[deny(clippy::undocumented_unsafe_blocks)]
#[deny(clippy::multiple_unsafe_ops_per_block)]
pub mod filters_neon;
#[cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]
#[allow(unsafe_code, reason = "ADR 0008: SIMD intrinsics")]
#[deny(clippy::undocumented_unsafe_blocks)]
#[deny(clippy::multiple_unsafe_ops_per_block)]
pub mod filters_sse2;
pub mod lossless;
#[cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]
#[allow(unsafe_code, reason = "ADR 0008: SIMD intrinsics")]
#[deny(clippy::undocumented_unsafe_blocks)]
#[deny(clippy::multiple_unsafe_ops_per_block)]
pub mod lossless_avx2;
pub mod lossless_common;
pub mod lossless_enc;
#[cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]
#[allow(unsafe_code, reason = "ADR 0008: SIMD intrinsics")]
#[deny(clippy::undocumented_unsafe_blocks)]
#[deny(clippy::multiple_unsafe_ops_per_block)]
pub mod lossless_enc_avx2;
#[cfg(all(feature = "simd", target_arch = "aarch64"))]
#[allow(unsafe_code, reason = "ADR 0008: SIMD intrinsics")]
#[deny(clippy::undocumented_unsafe_blocks)]
#[deny(clippy::multiple_unsafe_ops_per_block)]
pub mod lossless_enc_neon;
#[cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]
#[allow(unsafe_code, reason = "ADR 0008: SIMD intrinsics")]
#[deny(clippy::undocumented_unsafe_blocks)]
#[deny(clippy::multiple_unsafe_ops_per_block)]
pub mod lossless_enc_sse2;
#[cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]
#[allow(unsafe_code, reason = "ADR 0008: SIMD intrinsics")]
#[deny(clippy::undocumented_unsafe_blocks)]
#[deny(clippy::multiple_unsafe_ops_per_block)]
pub mod lossless_enc_sse41;
#[cfg(all(feature = "simd", target_arch = "aarch64"))]
#[allow(unsafe_code, reason = "ADR 0008: SIMD intrinsics")]
#[deny(clippy::undocumented_unsafe_blocks)]
#[deny(clippy::multiple_unsafe_ops_per_block)]
pub mod lossless_neon;
#[cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]
#[allow(unsafe_code, reason = "ADR 0008: SIMD intrinsics")]
#[deny(clippy::undocumented_unsafe_blocks)]
#[deny(clippy::multiple_unsafe_ops_per_block)]
pub mod lossless_sse2;
#[cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]
#[allow(unsafe_code, reason = "ADR 0008: SIMD intrinsics")]
#[deny(clippy::undocumented_unsafe_blocks)]
#[deny(clippy::multiple_unsafe_ops_per_block)]
pub mod lossless_sse41;
#[cfg(all(feature = "simd", target_arch = "aarch64"))]
#[allow(unsafe_code, reason = "ADR 0008: SIMD intrinsics")]
#[deny(clippy::undocumented_unsafe_blocks)]
#[deny(clippy::multiple_unsafe_ops_per_block)]
pub mod neon;
pub mod quant;
pub mod rescaler;
#[cfg(all(feature = "simd", target_arch = "aarch64"))]
#[allow(unsafe_code, reason = "ADR 0008: SIMD intrinsics")]
#[deny(clippy::undocumented_unsafe_blocks)]
#[deny(clippy::multiple_unsafe_ops_per_block)]
pub mod rescaler_neon;
#[cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]
#[allow(unsafe_code, reason = "ADR 0008: SIMD intrinsics")]
#[deny(clippy::undocumented_unsafe_blocks)]
#[deny(clippy::multiple_unsafe_ops_per_block)]
pub mod rescaler_sse2;
pub mod ssim;
#[cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]
#[allow(unsafe_code, reason = "ADR 0008: SIMD intrinsics")]
#[deny(clippy::undocumented_unsafe_blocks)]
#[deny(clippy::multiple_unsafe_ops_per_block)]
pub mod ssim_sse2;
pub mod upsampling;
#[cfg(all(feature = "simd", target_arch = "aarch64"))]
#[allow(unsafe_code, reason = "ADR 0008: SIMD intrinsics")]
#[deny(clippy::undocumented_unsafe_blocks)]
#[deny(clippy::multiple_unsafe_ops_per_block)]
pub mod upsampling_neon;
#[cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]
#[allow(unsafe_code, reason = "ADR 0008: SIMD intrinsics")]
#[deny(clippy::undocumented_unsafe_blocks)]
#[deny(clippy::multiple_unsafe_ops_per_block)]
pub mod upsampling_sse2;
#[cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]
#[allow(unsafe_code, reason = "ADR 0008: SIMD intrinsics")]
#[deny(clippy::undocumented_unsafe_blocks)]
#[deny(clippy::multiple_unsafe_ops_per_block)]
pub mod upsampling_sse41;
pub mod yuv;
#[cfg(all(feature = "simd", target_arch = "aarch64"))]
#[allow(unsafe_code, reason = "ADR 0008: SIMD intrinsics")]
#[deny(clippy::undocumented_unsafe_blocks)]
#[deny(clippy::multiple_unsafe_ops_per_block)]
pub mod yuv_neon;
#[cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]
#[allow(unsafe_code, reason = "ADR 0008: SIMD intrinsics")]
#[deny(clippy::undocumented_unsafe_blocks)]
#[deny(clippy::multiple_unsafe_ops_per_block)]
pub mod yuv_sse2;
#[cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]
#[allow(unsafe_code, reason = "ADR 0008: SIMD intrinsics")]
#[deny(clippy::undocumented_unsafe_blocks)]
#[deny(clippy::multiple_unsafe_ops_per_block)]
pub mod yuv_sse41;
