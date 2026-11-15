//! `ProcessingReport::decoder_backend` and `encoder_backend` (issue #217,
//! ADR-0017 amendment of 2026-11-14): every reader and writer the compiled
//! features can select names itself in the report, including the JPEG XL
//! reader that the build's default selects through feature unification.
#![cfg(any(
    feature = "jpeg-read",
    feature = "webp-read",
    feature = "jpeg-xl-read",
    feature = "jpeg-xl-own-read",
    feature = "avif-read"
))]
// Each test needs its own feature combination; with some features alone the
// shared helpers are compiled but unused.
#![allow(dead_code, unused_imports)]

use ferrisma::pipeline::{
    AvifEncoder, EncodeError, EncodeRequest, EncodedImage, EncoderBackend, ImageFormat,
    InputProfilePolicy, JpegXlDecoder, Output, OutputFormat, RowEncoder, TransformOptions,
    transcode, transcode_with,
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
        TransformOptions {
            output_format: OutputFormat::WebP,
            output_width: 4,
            output_height: 4,
            transform_threads: 1,
            ..options
        },
        &Discard,
    )
    .unwrap()
}

fn srgb() -> TransformOptions {
    TransformOptions {
        profile_policy: InputProfilePolicy::AssumeSrgbIfMissing,
        ..TransformOptions::default()
    }
}

/// JPEG XL adapters describe their output with an ICC profile.
fn rendered_icc(decoder: JpegXlDecoder) -> TransformOptions {
    TransformOptions {
        jxl_decoder: decoder,
        profile_policy: InputProfilePolicy::UseRenderedIcc,
        ..TransformOptions::default()
    }
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

#[cfg(all(feature = "jpeg-xl-read", feature = "icc"))]
#[test]
fn explicitly_selected_jxl_oxide_is_reported() {
    let jxl = include_bytes!("fixtures/orientation/jxl-1.jxl");
    let report = read(
        jxl,
        ImageFormat::JpegXl,
        rendered_icc(JpegXlDecoder::JxlOxide),
    )
    .report;
    assert_eq!(report.decoder_backend, "jxl-oxide");
}

#[cfg(all(feature = "jpeg-xl-own-read", feature = "icc"))]
#[test]
fn explicitly_selected_libjxl_rs_is_reported() {
    let jxl = include_bytes!("fixtures/orientation/jxl-1.jxl");
    let report = read(
        jxl,
        ImageFormat::JpegXl,
        rendered_icc(JpegXlDecoder::LibjxlRs),
    )
    .report;
    assert_eq!(report.decoder_backend, "libjxl-rs");
}

/// The case issue #217 is about: the default reader follows the compiled
/// features, and the report says which one ran.
#[cfg(all(
    any(feature = "jpeg-xl-read", feature = "jpeg-xl-own-read"),
    feature = "icc"
))]
#[test]
fn default_jpeg_xl_reader_is_reported() {
    let jxl = include_bytes!("fixtures/orientation/jxl-1.jxl");
    let report = read(
        jxl,
        ImageFormat::JpegXl,
        rendered_icc(JpegXlDecoder::default()),
    )
    .report;
    let expected = if cfg!(feature = "jpeg-xl-own-read") {
        "libjxl-rs"
    } else {
        "jxl-oxide"
    };
    assert_eq!(report.decoder_backend, expected);
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
fn write(output_format: OutputFormat, avif_encoder: AvifEncoder, speed: u8) -> Output {
    let jpeg = include_bytes!("fixtures/decoder-work/paris-420.jpg");
    transcode(
        jpeg,
        ImageFormat::Jpeg,
        TransformOptions {
            output_format,
            avif_encoder,
            speed,
            output_width: 16,
            output_height: 12,
            transform_threads: 1,
            ..srgb()
        },
    )
    .unwrap()
}

#[cfg(all(feature = "jpeg-read", feature = "webp-write"))]
#[test]
fn webp_writer_is_reported() {
    let output = write(OutputFormat::WebP, AvifEncoder::default(), 6);
    assert_eq!(output.report.decoder_backend, "libjpeg-turbo-rs");
    assert_eq!(output.report.encoder_backend, "libwebp-rs");
}

#[cfg(all(feature = "jpeg-read", feature = "jpeg-xl-write"))]
#[test]
fn jpeg_xl_writer_is_reported() {
    let output = write(OutputFormat::JpegXl, AvifEncoder::default(), 6);
    assert_eq!(output.report.encoder_backend, "libjxl-rs");
}

#[cfg(all(feature = "jpeg-read", feature = "avif-write"))]
#[test]
fn explicitly_selected_rav1e_is_reported() {
    let output = write(OutputFormat::Avif, AvifEncoder::Rav1e, 10);
    assert_eq!(output.report.encoder_backend, "rav1e");
}

#[cfg(all(feature = "jpeg-read", feature = "avif-own-write"))]
#[test]
fn explicitly_selected_libavif_is_reported() {
    let output = write(OutputFormat::Avif, AvifEncoder::Libavif { depth: 8 }, 10);
    assert_eq!(output.report.encoder_backend, "libavif-rs/aom");
}
