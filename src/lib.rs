//! Ferrisma — an experimental, pipeline-first image processing engine.
//!
//! Ferrisma's processing API is [`pipeline`]: `transcode`, `inspect` and
//! `Engine::transcode_batch`, with the codec adapters behind Cargo features.
//! Options are built with a constructor or `Default` and one `with_*`
//! method per setting:
//!
//! ```no_run
//! use ferrisma::pipeline::{
//!     ImageFormat, InputProfilePolicy, OutputGeometry, OutputSettings, ResizeOptions,
//!     TransformOptions, WebpOptions, transcode,
//! };
//!
//! let jpeg = std::fs::read("photo.jpg")?;
//! let options = TransformOptions::new(OutputGeometry::Resize(ResizeOptions::default().with_width(320)))
//!     .with_output(OutputSettings::WebP(WebpOptions::default()))
//!     .with_profile_policy(InputProfilePolicy::AssumeSrgbIfMissing);
//! let thumbnail = transcode(&jpeg, ImageFormat::Jpeg, &options)?;
//! # let _ = thumbnail;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! No release is stable yet: before
//! 1.0 every 0.x minor release may break it, and the tiers proposed for 1.0
//! (stable or experimental per item) are in ADR-0040 and
//! `docs/compatibility.md`, pending the maintainer's acceptance.
//!
//! See `docs/architecture.md` and `docs/adr/` in the repository.

#![forbid(unsafe_code)]
#![warn(missing_docs)]
// A backend-only build exposes the row encoder facade but has no built-in
// sink; some experimental internals are unreachable outside tests. With a
// built-in sink, dead code is reported, except scoped architecture experiments.
#![cfg_attr(
    all(not(test), not(any(feature = "webp-write", feature = "jpeg-xl-write"))),
    allow(dead_code)
)]

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
mod discarded;
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
mod gray;
#[cfg(feature = "jpeg-xl-write")]
mod jxl_sink;
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
mod output_depth;
#[cfg(feature = "avif-write")]
mod own_avif;
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
mod resize_geometry;
#[cfg_attr(
    not(test),
    allow(
        dead_code,
        reason = "borrowed planar surface contract, exercised by tests (ADR-0011)"
    )
)]
mod surface;
mod thread_spawn;
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

#[cfg(all(test, feature = "jpeg-read", feature = "avif-write"))]
mod pipeline_tests;

#[cfg(all(test, feature = "webp-read", feature = "avif-write"))]
mod webp_pipeline_tests;

#[cfg(all(test, feature = "webp-read", feature = "webp-write"))]
mod webp_output_tests;

#[cfg(all(test, feature = "jpeg-xl-write"))]
mod jxl_output_tests;

#[cfg(all(test, feature = "jpeg-xl-write"))]
mod jxl_lossy_tests;

#[cfg(all(test, feature = "jpeg-read", not(feature = "avif-write")))]
mod backend_only_tests;

#[cfg(test)]
mod hardening_tests;

#[cfg(all(
    test,
    feature = "jpeg-read",
    feature = "webp-read",
    feature = "webp-write",
    feature = "jpeg-xl-read",
    feature = "avif-read",
    feature = "icc"
))]
mod refusal_reason_tests;

#[cfg(all(test, feature = "jpeg-xl-read"))]
mod jxl_color_tests;

#[cfg(all(test, feature = "jpeg-xl-write", feature = "jpeg-xl-read"))]
mod jxl_alpha_tests;
