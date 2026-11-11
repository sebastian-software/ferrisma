//! Ferrixel — an experimental, pipeline-first image processing engine.
//!
//! Ferrixel exposes an experimental pipeline and optional codec adapters for
//! validating color, region planning, and bounded engine-managed memory.
//! There is intentionally no stable public processing API.
//!
//! See `docs/architecture.md` and `docs/adr/` in the repository.

#![forbid(unsafe_code)]
// A backend-only build exposes the row encoder facade but has no built-in
// sink; some experimental internals are unreachable outside tests. With a
// built-in sink, dead code is reported, except scoped architecture experiments.
#![cfg_attr(
    all(not(test), not(any(feature = "avif-write", feature = "webp-write"))),
    allow(dead_code)
)]

#[cfg(feature = "avif-write")]
mod av1_tuning;
#[cfg(feature = "avif-write")]
mod avif_sink;
// The shared command accepts an external backend without a built-in writer;
// missing decoder adapters report `FeatureDisabled`.
pub mod cli;
#[cfg(feature = "icc")]
mod cms;
mod codecs;
mod color;
mod color_contract;
#[cfg_attr(
    not(test),
    allow(
        dead_code,
        reason = "encoded-sample resize baseline, exercised by tests (ADR-0013)"
    )
)]
mod decoded_resize;
mod decoder;
mod encoder_backend;
mod error;
#[cfg_attr(
    not(test),
    allow(
        dead_code,
        reason = "Gray8 vertical-slice executor, exercised by tests (ADR-0008)"
    )
)]
mod execute;
mod fast_execute;
mod geometry;
mod graph;
#[cfg_attr(
    not(test),
    allow(
        dead_code,
        reason = "strict linear-sRGB bilinear reference, exercised by tests (ADR-0014)"
    )
)]
mod linear_srgb;
mod memory;
mod orientation;
mod parallel_execute;
pub mod pipeline;
mod plan;
#[cfg_attr(
    not(test),
    allow(
        dead_code,
        reason = "borrowed YUV420P10 experiment, exercised by tests (ADR-0011)"
    )
)]
mod planar;
mod quality;
mod quality_execute;
#[cfg_attr(
    not(test),
    allow(
        dead_code,
        reason = "borrowed planar surface contract, exercised by tests (ADR-0011)"
    )
)]
mod surface;
#[cfg(feature = "webp-write")]
mod webp_sink;
mod working_color;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod planar_tests;

#[cfg(test)]
mod quality_tests;

#[cfg(test)]
mod quality_execute_tests;

#[cfg(test)]
mod fast_execute_tests;

#[cfg(test)]
mod decoder_tests;

#[cfg(test)]
mod decoded_resize_tests;

#[cfg(test)]
mod linear_srgb_tests;

#[cfg(test)]
mod working_color_tests;

#[cfg(test)]
mod color_contract_tests;

#[cfg(all(test, feature = "avif-write"))]
mod avif_sink_tests;

#[cfg(all(test, feature = "jpeg-read", feature = "avif-write"))]
mod pipeline_tests;

#[cfg(all(test, feature = "webp-read", feature = "avif-write"))]
mod webp_pipeline_tests;

#[cfg(all(test, feature = "webp-read", feature = "webp-write"))]
mod webp_output_tests;

#[cfg(all(test, feature = "jpeg-read", not(feature = "avif-write")))]
mod backend_only_tests;

#[cfg(test)]
mod hardening_tests;
