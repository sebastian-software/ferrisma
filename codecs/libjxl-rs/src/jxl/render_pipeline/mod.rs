// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
pub mod low_memory_render_pipeline;
#[expect(
    clippy::module_inception,
    reason = "Retain the source render_pipeline/render_pipeline.{h,cc} module path."
)]
pub mod render_pipeline;
pub mod render_pipeline_stage;
// Row kernels of the AArch64 render stages (see `pre_r1`).
#[cfg(target_arch = "aarch64")]
pub mod scalar_rows;
pub mod simple_render_pipeline;
pub mod stage_blending;
// Float render stages: AArch64 compiles the row-span/row-kernel versions
// below; every other target compiles the pre-work sources of 93c5281f
// verbatim from `pre_r1/`, so x86-64 keeps their NaN payloads and speed.
// See the port's ADR 0002 and the 2026-11-12 performance analysis.
#[cfg_attr(
    not(target_arch = "aarch64"),
    path = "pre_r1/stage_chroma_upsampling.rs"
)]
pub mod stage_chroma_upsampling;
pub mod stage_cms;
#[cfg_attr(not(target_arch = "aarch64"), path = "pre_r1/stage_epf.rs")]
pub mod stage_epf;
#[cfg_attr(not(target_arch = "aarch64"), path = "pre_r1/stage_from_linear.rs")]
pub mod stage_from_linear;
#[cfg_attr(not(target_arch = "aarch64"), path = "pre_r1/stage_gaborish.rs")]
pub mod stage_gaborish;
#[cfg_attr(not(target_arch = "aarch64"), path = "pre_r1/stage_noise.rs")]
pub mod stage_noise;
pub mod stage_patches;
pub mod stage_splines;
pub mod stage_spot;
pub mod stage_to_linear;
pub mod stage_tone_mapping;
#[cfg_attr(not(target_arch = "aarch64"), path = "pre_r1/stage_upsampling.rs")]
pub mod stage_upsampling;
pub mod stage_write;
mod stage_write_dither;
#[cfg_attr(not(target_arch = "aarch64"), path = "pre_r1/stage_xyb.rs")]
pub mod stage_xyb;
#[cfg_attr(not(target_arch = "aarch64"), path = "pre_r1/stage_ycbcr.rs")]
pub mod stage_ycbcr;
