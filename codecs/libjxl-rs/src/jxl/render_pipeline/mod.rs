// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
pub mod low_memory_render_pipeline;
#[expect(
    clippy::module_inception,
    reason = "Retain the source render_pipeline/render_pipeline.{h,cc} module path."
)]
pub mod render_pipeline;
pub mod render_pipeline_stage;
pub mod simple_render_pipeline;
pub mod stage_blending;
pub mod stage_chroma_upsampling;
pub mod stage_cms;
pub mod stage_epf;
pub mod stage_from_linear;
pub mod stage_gaborish;
pub mod stage_noise;
pub mod stage_patches;
pub mod stage_splines;
pub mod stage_spot;
pub mod stage_to_linear;
pub mod stage_tone_mapping;
pub mod stage_upsampling;
pub mod stage_write;
mod stage_write_dither;
pub mod stage_xyb;
pub mod stage_ycbcr;
// SIMD (NEON/AVX2) stages, compared only with the original's SIMD build at
// the same target; not selected by the decoder yet (ADR 0004).
mod simd_rows;
pub mod stage_chroma_simd;
pub mod stage_epf_simd;
pub mod stage_gaborish_simd;
// render stages: SIMD stages of ADR 0004 Plan item 1.
mod simd_transfer;
pub mod stage_from_linear_simd;
mod stage_write_simd;
pub mod stage_xyb_simd;
// end render stages
// color: SIMD color stages of ADR 0004 Plan item 1.
pub mod stage_to_linear_simd;
// end color
