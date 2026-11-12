//! Animated input policy (#68, ADR-0019): every supported input format that
//! can carry an animation rejects it through the facade as unsupported input.
//! There is no first-frame fallback; a still image of the same format is
//! accepted. JPEG has no animation; its MPF secondary images are not read.
#![cfg(any(
    feature = "webp-read",
    feature = "jpeg-xl-read",
    feature = "jpeg-xl-own-read",
    feature = "avif-read"
))]

use ferrisma::pipeline::{
    EncodeError, EncodeRequest, EncodedImage, EncoderBackend, ImageFormat, InputProfilePolicy,
    OutputFormat, ProcessingErrorKind, RowEncoder, TransformOptions, transcode_with,
};

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

fn kind(bytes: &[u8], format: ImageFormat, options: TransformOptions) -> ProcessingErrorKind {
    let options = TransformOptions {
        output_format: OutputFormat::WebP,
        output_width: 4,
        output_height: 4,
        ..options
    };
    transcode_with(bytes, format, options, &Discard)
        .expect_err("animated input must be rejected")
        .kind()
}

fn srgb() -> TransformOptions {
    TransformOptions {
        profile_policy: InputProfilePolicy::AssumeSrgbIfMissing,
        ..TransformOptions::default()
    }
}

/// A two-frame lossless animation written by `img2webp` 1.6.0
/// (`tests/fixtures/decoder-work/README.md`).
#[cfg(feature = "webp-read")]
#[test]
fn animated_webp_is_rejected() {
    let animated = include_bytes!("fixtures/decoder-work/animated.webp");
    assert_eq!(
        kind(animated, ImageFormat::WebP, srgb()),
        ProcessingErrorKind::UnsupportedInput
    );
}

#[cfg(feature = "jpeg-xl-read")]
#[test]
fn animated_jpeg_xl_is_rejected_by_jxl_oxide() {
    let animated = include_bytes!(
        "../codecs/libjxl-rs/tests/data/decoder-geometry/sequence-reference-overwrite-animation-1.jxl"
    );
    assert_eq!(
        kind(animated, ImageFormat::JpegXl, srgb()),
        ProcessingErrorKind::UnsupportedInput
    );
}

#[cfg(feature = "jpeg-xl-own-read")]
#[test]
fn animated_jpeg_xl_is_rejected_by_libjxl_rs() {
    let animated = include_bytes!(
        "../codecs/libjxl-rs/tests/data/decoder-geometry/sequence-reference-overwrite-animation-1.jxl"
    );
    let options = TransformOptions {
        jxl_decoder: ferrisma::pipeline::JpegXlDecoder::LibjxlRs,
        ..srgb()
    };
    assert_eq!(
        kind(animated, ImageFormat::JpegXl, options),
        ProcessingErrorKind::UnsupportedInput
    );
}

/// An `avis` image sequence from the libavif test data.
#[cfg(feature = "avif-read")]
#[test]
fn animated_avif_is_rejected() {
    let animated = include_bytes!(
        "../codecs/libavif-rs/upstream/libavif/tests/data/colors-animated-8bpc.avif"
    );
    assert_eq!(
        kind(animated, ImageFormat::Avif, srgb()),
        ProcessingErrorKind::UnsupportedInput
    );
}
