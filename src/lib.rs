//! Ferrixel — an experimental, pipeline-first image processing engine.
//!
//! Ferrixel currently contains a private experimental pipeline and optional
//! read-only JPEG/JPEG XL decoders for validating codec capability contracts,
//! region planning, and bounded engine-managed memory. There is intentionally no stable public processing API.
//!
//! See `docs/architecture.md` and `docs/adr/` in the repository.

#![forbid(unsafe_code)]
// Without the `avif-write` facade (ADR-0017) the crate has no public API, so
// every internal item is unreachable outside tests. With the facade, dead code
// is reported; architecture experiments carry their own scoped allowances.
#![cfg_attr(all(not(test), not(feature = "avif-write")), allow(dead_code))]

#[cfg(feature = "avif-write")]
mod avif_sink;
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
mod error;
#[cfg_attr(
    not(test),
    allow(
        dead_code,
        reason = "Gray8 vertical-slice executor, exercised by tests (ADR-0008)"
    )
)]
mod execute;
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
#[cfg(feature = "avif-write")]
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

#[cfg(test)]
mod hardening_tests;
