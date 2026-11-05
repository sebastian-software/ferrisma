//! Translation of libwebp's top-level `sharpyuv` directory.
//!
//! The `SharpYUV` RGB to YUV converter that the encoder uses.
//!
//! Each module translates the upstream file of the same name; its header names
//! the upstream files and the work package that owns it. WP0 fixed this file
//! and the module list in `tools/units.json`; work packages do not edit it.

#[expect(clippy::module_inception, reason = "keeps the upstream file name")]
pub mod sharpyuv;
pub mod sharpyuv_cpu;
pub mod sharpyuv_csp;
pub mod sharpyuv_dsp;
pub mod sharpyuv_gamma;
#[cfg(target_arch = "aarch64")]
pub mod sharpyuv_neon;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod sharpyuv_sse2;
