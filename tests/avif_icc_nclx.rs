//! AVIF with both an ICC `colr` and an `nclx` `colr` (review of 2026-11-12,
//! ADR-0015 rule 4, ADR-0016). MIAF gives the ICC profile precedence for
//! color interpretation, but the engine applies no precedence silently: under
//! `UseRenderedIcc` the nclx must be an SDR family that agrees with the
//! profile, or the input is refused with an error that names the conflict.
//! Fixtures and provenance: `tests/fixtures/avif-color/README.md`.
#![cfg(all(feature = "avif-read", feature = "icc"))]

use ferrisma::pipeline::{
    EncodeError, EncodeRequest, EncodedImage, EncoderBackend, ImageFormat, InputProfilePolicy,
    OutputFormat, ProcessingError, ProcessingErrorKind, RowEncoder, TransformOptions,
    transcode_with,
};

/// sRGB ICC profile plus nclx 1/13/6: both signals say sRGB.
const AGREEING: &[u8] = include_bytes!("fixtures/avif-color/icc-nclx-1-13-6.avif");
/// The same sRGB ICC profile plus nclx 1/1/6: BT.709 transfer beside it.
const BT709_TRANSFER: &[u8] = include_bytes!("fixtures/avif-color/icc-nclx-1-1-6.avif");

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

fn run(bytes: &[u8], profile_policy: InputProfilePolicy) -> Result<(), ProcessingError> {
    let options = TransformOptions {
        output_format: OutputFormat::WebP,
        output_width: 8,
        output_height: 8,
        profile_policy,
        transform_threads: 1,
        ..TransformOptions::default()
    };
    transcode_with(bytes, ImageFormat::Avif, options, &Discard).map(|_| ())
}

#[test]
fn agreeing_srgb_nclx_beside_icc_converts_under_rendered_icc() {
    run(AGREEING, InputProfilePolicy::UseRenderedIcc).unwrap();
}

/// Decision pending owner confirmation (ADR-0015, 2026-11-12): BT.709
/// transfer beside an sRGB ICC profile is refused, not resolved by MIAF's ICC
/// precedence, and the error says why.
#[test]
fn bt709_transfer_nclx_beside_icc_is_refused_with_a_named_reason() {
    let error = run(BT709_TRANSFER, InputProfilePolicy::UseRenderedIcc).unwrap_err();
    assert_eq!(error.kind(), ProcessingErrorKind::UnsupportedColor);
    let message = error.to_string();
    assert!(
        message.contains("CICP beside the rendered ICC profile"),
        "{message}"
    );
    assert!(message.contains("not given precedence"), "{message}");
}

/// Without the rendered-ICC policy, an ICC-tagged file is never treated as
/// sRGB, whatever its nclx says.
#[test]
fn icc_tagged_files_need_the_rendered_icc_policy() {
    for bytes in [AGREEING, BT709_TRANSFER] {
        for policy in [
            InputProfilePolicy::Strict,
            InputProfilePolicy::AssumeSrgbIfMissing,
        ] {
            assert_eq!(
                run(bytes, policy).unwrap_err().kind(),
                ProcessingErrorKind::UnsupportedColor,
                "{policy:?}"
            );
        }
    }
}
