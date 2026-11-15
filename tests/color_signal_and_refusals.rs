//! `ProcessingReport::color_signal` and `ProcessingError::refusal_reason`
//! (issue #212, ADR-0017 amendment of 2026-11-14, ADR-0015): which color
//! signal a conversion used, and a machine-readable reason for each refusal
//! the vocabulary names. Every reason and signal the compiled features can
//! produce has a test here; `RefusalReason::Animation` is covered per format
//! in `animated_input.rs`, and `ColorSignal::Cicp` (AVIF `nclx` under the
//! strict policy) in the library's AVIF round-trip test.
#![cfg(any(
    feature = "jpeg-read",
    feature = "jpeg-xl-read",
    feature = "jpeg-xl-own-read"
))]
// Each test needs its own feature combination.
#![allow(dead_code, unused_imports)]

use ferrisma::pipeline::{
    ColorSignal, EncodeError, EncodeRequest, EncodedImage, EncoderBackend, ImageFormat,
    InputProfilePolicy, JpegXlDecoder, Output, OutputFormat, ProcessingError, ProcessingErrorKind,
    RefusalReason, RowEncoder, TransformOptions, transcode_with,
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

fn run(
    bytes: &[u8],
    format: ImageFormat,
    profile_policy: InputProfilePolicy,
    jxl_decoder: JpegXlDecoder,
) -> Result<Output, ProcessingError> {
    let options = TransformOptions {
        output_format: OutputFormat::WebP,
        output_width: 4,
        output_height: 4,
        transform_threads: 1,
        profile_policy,
        jxl_decoder,
        ..TransformOptions::default()
    };
    transcode_with(bytes, format, options, &Discard)
}

fn jpeg(profile_policy: InputProfilePolicy) -> Result<Output, ProcessingError> {
    let bytes = include_bytes!("fixtures/sample.jpg");
    run(
        bytes,
        ImageFormat::Jpeg,
        profile_policy,
        JpegXlDecoder::default(),
    )
}

#[allow(
    clippy::vec_init_then_push,
    reason = "each push is behind its own feature"
)]
fn jxl_decoders() -> Vec<JpegXlDecoder> {
    let mut decoders = Vec::new();
    #[cfg(feature = "jpeg-xl-read")]
    decoders.push(JpegXlDecoder::JxlOxide);
    #[cfg(feature = "jpeg-xl-own-read")]
    decoders.push(JpegXlDecoder::LibjxlRs);
    decoders
}

fn refusal(
    result: Result<Output, ProcessingError>,
) -> (ProcessingErrorKind, Option<RefusalReason>) {
    let error = result.expect_err("input must be refused");
    (error.kind(), error.refusal_reason())
}

#[cfg(feature = "jpeg-read")]
#[test]
fn untagged_input_under_the_strict_policy_has_no_color_signal() {
    assert_eq!(
        refusal(jpeg(InputProfilePolicy::Strict)),
        (
            ProcessingErrorKind::UnsupportedColor,
            Some(RefusalReason::ColorSignalMissing)
        )
    );
}

#[cfg(feature = "jpeg-read")]
#[test]
fn assumed_srgb_is_reported_as_the_signal_used() {
    let output = jpeg(InputProfilePolicy::AssumeSrgbIfMissing).unwrap();
    assert_eq!(output.report.color_signal, ColorSignal::AssumedSrgb);
}

/// Errors that are not refusals of the input carry no reason.
#[cfg(feature = "jpeg-read")]
#[test]
fn other_errors_have_no_refusal_reason() {
    let bytes = include_bytes!("fixtures/sample.jpg");
    let truncated = run(
        &bytes[..bytes.len() / 2],
        ImageFormat::Jpeg,
        InputProfilePolicy::AssumeSrgbIfMissing,
        JpegXlDecoder::default(),
    );
    assert_eq!(
        refusal(truncated),
        (ProcessingErrorKind::MalformedInput, None)
    );
    let options = TransformOptions {
        output_format: OutputFormat::WebP,
        output_width: 0,
        ..TransformOptions::default()
    };
    let error = transcode_with(bytes, ImageFormat::Jpeg, options, &Discard).unwrap_err();
    assert_eq!(
        (error.kind(), error.refusal_reason()),
        (ProcessingErrorKind::InvalidOptions, None)
    );
}

/// Both JPEG XL adapters describe their output with an ICC profile, so the
/// two policies without ICC conversion refuse it and name the opt-in.
#[cfg(any(feature = "jpeg-xl-read", feature = "jpeg-xl-own-read"))]
#[test]
fn rendered_icc_without_the_opt_in_is_refused_with_its_reason() {
    let jxl = include_bytes!("fixtures/orientation/jxl-1.jxl");
    for decoder in jxl_decoders() {
        for policy in [
            InputProfilePolicy::Strict,
            InputProfilePolicy::AssumeSrgbIfMissing,
        ] {
            assert_eq!(
                refusal(run(jxl, ImageFormat::JpegXl, policy, decoder)),
                (
                    ProcessingErrorKind::UnsupportedColor,
                    Some(RefusalReason::RenderedIccRequiresOptIn)
                ),
                "{decoder:?} {policy:?}"
            );
        }
    }
}

/// jxl-oxide reports the header's enum encoding as CICP beside its rendered
/// ICC; the libjxl-rs adapter reports only the ICC (ADR-0015, 2026-11-12).
#[cfg(all(
    any(feature = "jpeg-xl-read", feature = "jpeg-xl-own-read"),
    feature = "icc"
))]
#[test]
fn rendered_icc_conversion_reports_whether_cicp_was_checked_beside_it() {
    let jxl = include_bytes!("fixtures/jxl-color/lossless-p3.jxl");
    for decoder in jxl_decoders() {
        let output = run(
            jxl,
            ImageFormat::JpegXl,
            InputProfilePolicy::UseRenderedIcc,
            decoder,
        )
        .unwrap();
        let expected = match decoder {
            JpegXlDecoder::JxlOxide => ColorSignal::RenderedIccWithAgreeingCicp,
            _ => ColorSignal::RenderedIcc,
        };
        assert_eq!(output.report.color_signal, expected, "{decoder:?}");
    }
}

/// DCI-P3 (primaries 11, transfer 17) beside the rendered profile cannot be
/// checked against it, so it is refused rather than overridden by the ICC.
#[cfg(all(
    any(feature = "jpeg-xl-read", feature = "jpeg-xl-own-read"),
    feature = "icc"
))]
#[test]
fn unverifiable_cicp_beside_the_rendered_icc_is_refused_with_its_reason() {
    let jxl = include_bytes!("fixtures/jxl-color/lossless-dci-p3.jxl");
    for decoder in jxl_decoders() {
        assert_eq!(
            refusal(run(
                jxl,
                ImageFormat::JpegXl,
                InputProfilePolicy::UseRenderedIcc,
                decoder
            )),
            (
                ProcessingErrorKind::UnsupportedColor,
                Some(RefusalReason::CicpBesideIcc)
            ),
            "{decoder:?}"
        );
    }
}
