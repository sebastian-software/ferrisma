//! AVIF with both an ICC `colr` and an `nclx` `colr` (owner decision of
//! 2026-11-15, ADR-0015): the file follows MIAF. The ICC profile governs
//! color primaries and transfer; matrix coefficients and range come from
//! `nclx`, which libavif applies when it converts YUV to RGB. Under
//! `UseRenderedIcc` such a file is converted through its profile whatever
//! the `nclx` primaries and transfer say; without the opt-in it is refused
//! like any ICC file (`RenderedIccRequiresOptIn`). Until then the `nclx`
//! had to be an SDR family that agrees with the profile, or the file was
//! refused with `CicpBesideIcc`.
//! Fixtures and provenance: `tests/fixtures/avif-color/README.md` and
//! `tests/fixtures/parity/README.md`.
#![cfg(all(feature = "avif-read", feature = "icc"))]

use std::sync::Mutex;

use ferrisma::pipeline::Threads;
use ferrisma::pipeline::{
    ColorSignal, EncodeError, EncodeRequest, EncodedImage, EncoderBackend, ImageFormat,
    InputProfilePolicy, OutputFormat, ProcessingError, ProcessingErrorKind, RefusalReason,
    RowEncoder, TransformOptions, transcode_with,
};
use ferrisma::pipeline::{OutputGeometry, OutputSettings};

/// sRGB ICC profile plus nclx 1/13/6: both signals say sRGB.
const AGREEING: &[u8] = include_bytes!("fixtures/avif-color/icc-nclx-1-13-6.avif");
/// The same sRGB ICC profile plus nclx 1/1/6: BT.709 transfer beside it.
const BT709_TRANSFER: &[u8] = include_bytes!("fixtures/avif-color/icc-nclx-1-1-6.avif");
/// What avifenc writes for a source with an ICC profile: the sRGB profile
/// plus nclx 2/2/6 (primaries and transfer unspecified; libavif's own test
/// file).
const UNSPECIFIED: &[u8] =
    include_bytes!("../codecs/libavif-rs/upstream/libavif/tests/data/paris_icc_exif_xmp.avif");
/// That file with its nclx primaries and transfer patched to 12/13
/// (Display P3) beside the sRGB profile.
const P3_BESIDE_SRGB: &[u8] = include_bytes!("fixtures/parity/icc-nclx-12-13-6.avif");

struct Discard(Mutex<Vec<u8>>);
struct DiscardRows<'a>(&'a Mutex<Vec<u8>>);

impl EncoderBackend for Discard {
    fn format(&self) -> OutputFormat {
        OutputFormat::WebP
    }
    fn name(&self) -> &'static str {
        "test-discard"
    }
    fn begin(&self, _: &EncodeRequest) -> Result<Box<dyn RowEncoder + '_>, EncodeError> {
        Ok(Box::new(DiscardRows(&self.0)))
    }
}

impl RowEncoder for DiscardRows<'_> {
    fn write_row(&mut self, _: u32, row: &[u8]) -> Result<(), EncodeError> {
        self.0.lock().unwrap().extend_from_slice(row);
        Ok(())
    }
    fn finish(self: Box<Self>) -> Result<EncodedImage, EncodeError> {
        Ok(EncodedImage::new(OutputFormat::WebP, Vec::new(), 0, false))
    }
}

/// The color signal and the rows of one transcode.
fn run(
    bytes: &[u8],
    profile_policy: InputProfilePolicy,
) -> Result<(ColorSignal, Vec<u8>), ProcessingError> {
    let options = TransformOptions::new(OutputGeometry::Exact {
        width: 8,
        height: 8,
    })
    .with_output(OutputSettings::default_for(OutputFormat::WebP))
    .with_profile_policy(profile_policy)
    .with_transform_threads(Threads::ONE);
    let capture = Discard(Mutex::new(Vec::new()));
    let output = transcode_with(bytes, ImageFormat::Avif, &options, &capture)?;
    Ok((output.report.color_signal, capture.0.into_inner().unwrap()))
}

/// Every `nclx` beside the sRGB profile converts through the profile
/// alone, reported as `RenderedIcc`: sRGB (1/13), BT.709 transfer (1/1),
/// unspecified (2/2, as avifenc writes it) and Display P3 (12/13). Until
/// 2026-11-15 only 2/2 and the agreeing 1/13 converted (the latter as
/// `RenderedIccWithAgreeingCicp`); 1/1 and 12/13 were refused with
/// `CicpBesideIcc` (`bt709_transfer_nclx_beside_icc_is_refused_with_a_named_reason`).
/// The two heif-enc files differ only in the transfer they signal, so the
/// profile alone describing the samples gives them identical rows.
#[test]
fn every_nclx_beside_an_icc_profile_converts_through_the_profile() {
    let mut rows = Vec::new();
    for (name, bytes) in [
        ("1/13/6", AGREEING),
        ("1/1/6", BT709_TRANSFER),
        ("2/2/6", UNSPECIFIED),
        ("12/13/6", P3_BESIDE_SRGB),
    ] {
        let (signal, converted) = run(bytes, InputProfilePolicy::UseRenderedIcc)
            .unwrap_or_else(|error| panic!("{name}: {error}"));
        assert_eq!(signal, ColorSignal::RenderedIcc, "{name}");
        rows.push(converted);
    }
    assert_eq!(
        rows[0], rows[1],
        "the nclx transfer does not change the samples"
    );
    assert_eq!(rows[2], rows[3], "nor do the nclx primaries and transfer");
}

/// Without the rendered-ICC policy, an ICC-tagged file is never treated as
/// sRGB, whatever its nclx says.
#[test]
fn icc_tagged_files_need_the_rendered_icc_policy() {
    for bytes in [AGREEING, BT709_TRANSFER, UNSPECIFIED, P3_BESIDE_SRGB] {
        for policy in [
            InputProfilePolicy::Strict,
            InputProfilePolicy::AssumeSrgbIfMissing,
        ] {
            let error = run(bytes, policy).unwrap_err();
            assert_eq!(
                error.kind(),
                ProcessingErrorKind::UnsupportedColor,
                "{policy:?}"
            );
            assert_eq!(
                error.refusal_reason(),
                Some(RefusalReason::RenderedIccRequiresOptIn),
                "{policy:?}"
            );
        }
    }
}
