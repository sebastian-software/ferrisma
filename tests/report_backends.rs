//! `ProcessingReport::decoder_backend` and `encoder_backend` (issue #217,
//! ADR-0017 amendment of 2026-11-14): every reader and writer the compiled
//! features can select names itself in the report.
#![cfg(any(
    feature = "jpeg-read",
    feature = "webp-read",
    feature = "jpeg-xl-read",
    feature = "avif-read"
))]
// Each test needs its own feature combination; with some features alone the
// shared helpers are compiled but unused.
#![allow(dead_code, unused_imports)]

use ferrisma::pipeline::Threads;
use ferrisma::pipeline::{AvifOptions, OutputGeometry, OutputSettings};
use ferrisma::pipeline::{
    EncodeError, EncodeRequest, EncodedImage, EncoderBackend, ImageFormat, InputProfilePolicy,
    Output, OutputFormat, RowEncoder, TransformOptions, transcode, transcode_with,
};

/// Discards rows, so the readers run without a compiled writer.
struct Discard;
struct DiscardRows;

impl EncoderBackend for Discard {
    fn format(&self) -> OutputFormat {
        OutputFormat::WebP
    }
    fn name(&self) -> &'static str {
        "test-discard"
    }
    fn begin(&self, _: &EncodeRequest) -> Result<Box<dyn RowEncoder + '_>, EncodeError> {
        Ok(Box::new(DiscardRows))
    }
}

impl RowEncoder for DiscardRows {
    fn write_row(&mut self, _: u32, _: &[u8]) -> Result<(), EncodeError> {
        Ok(())
    }
    fn finish(self: Box<Self>) -> Result<EncodedImage, EncodeError> {
        Ok(EncodedImage::new(OutputFormat::WebP, Vec::new(), 0, false))
    }
}

fn read(bytes: &[u8], format: ImageFormat, options: TransformOptions) -> Output {
    transcode_with(
        bytes,
        format,
        &options
            .with_output(OutputSettings::default_for(OutputFormat::WebP))
            .with_geometry(OutputGeometry::Exact {
                width: 4,
                height: 4,
            })
            .with_transform_threads(Threads::ONE),
        &Discard,
    )
    .unwrap()
}

fn srgb() -> TransformOptions {
    TransformOptions::new(OutputGeometry::Exact {
        width: 1,
        height: 1,
    })
    .with_profile_policy(InputProfilePolicy::AssumeSrgbIfMissing)
}

/// The JPEG XL reader describes its output with an ICC profile.
fn rendered_icc() -> TransformOptions {
    TransformOptions::new(OutputGeometry::Exact {
        width: 1,
        height: 1,
    })
    .with_profile_policy(InputProfilePolicy::UseRenderedIcc)
}

#[cfg(feature = "jpeg-read")]
#[test]
fn jpeg_reader_is_reported() {
    let jpeg = include_bytes!("fixtures/decoder-work/paris-420.jpg");
    let report = read(jpeg, ImageFormat::Jpeg, srgb()).report;
    assert_eq!(report.decoder_backend, "libjpeg-turbo-rs");
    assert_eq!(report.encoder_backend, "test-discard");
}

#[cfg(feature = "webp-read")]
#[test]
fn webp_reader_is_reported() {
    let webp = include_bytes!("fixtures/decoder-work/paris-o5-lossless.webp");
    let report = read(webp, ImageFormat::WebP, srgb()).report;
    assert_eq!(report.decoder_backend, "libwebp-rs");
}

/// One JPEG XL reader since 2026-11-15 (ADR-0032), named in the report.
/// (Before, the default reader followed the compiled features, issue #217.)
#[cfg(all(feature = "jpeg-xl-read", feature = "icc"))]
#[test]
fn jpeg_xl_reader_is_reported() {
    let jxl = include_bytes!("fixtures/orientation/jxl-1.jxl");
    let report = read(jxl, ImageFormat::JpegXl, rendered_icc()).report;
    assert_eq!(report.decoder_backend, "libjxl-rs");
}

#[cfg(feature = "avif-read")]
#[test]
fn avif_reader_is_reported() {
    let avif = include_bytes!("fixtures/orientation/avif-6.avif");
    let report = read(avif, ImageFormat::Avif, srgb()).report;
    assert_eq!(report.decoder_backend, "libavif-rs/aom");
}

/// One small JPEG through a built-in writer; `speed` applies to AVIF only.
#[cfg(feature = "jpeg-read")]
fn write(output_format: OutputFormat, speed: u8) -> Output {
    let jpeg = include_bytes!("fixtures/decoder-work/paris-420.jpg");
    transcode(
        jpeg,
        ImageFormat::Jpeg,
        &srgb()
            .with_output(match output_format {
                OutputFormat::Avif => {
                    OutputSettings::Avif(AvifOptions::default().with_speed(speed))
                }
                other => OutputSettings::default_for(other),
            })
            .with_geometry(OutputGeometry::Exact {
                width: 16,
                height: 12,
            })
            .with_transform_threads(Threads::ONE),
    )
    .unwrap()
}

#[cfg(all(feature = "jpeg-read", feature = "webp-write"))]
#[test]
fn webp_writer_is_reported() {
    let output = write(OutputFormat::WebP, 6);
    assert_eq!(output.report.decoder_backend, "libjpeg-turbo-rs");
    assert_eq!(output.report.encoder_backend, "libwebp-rs");
}

#[cfg(all(feature = "jpeg-read", feature = "jpeg-xl-write"))]
#[test]
fn jpeg_xl_writer_is_reported() {
    let output = write(OutputFormat::JpegXl, 6);
    assert_eq!(output.report.encoder_backend, "libjxl-rs");
}

#[cfg(all(feature = "jpeg-read", feature = "avif-write"))]
#[test]
fn avif_writer_is_reported() {
    let output = write(OutputFormat::Avif, 10);
    assert_eq!(output.report.encoder_backend, "libavif-rs/aom");
}
