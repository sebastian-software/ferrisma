//! `ProcessingReport::color_signal` and `ProcessingError::refusal_reason`
//! (issue #212, ADR-0017 amendment of 2026-11-14, ADR-0015): which color
//! signal a conversion used, and a machine-readable reason for each refusal
//! the vocabulary names. Every reason and signal the compiled features can
//! produce has a test here; `RefusalReason::Animation` is covered per format
//! in `animated_input.rs`, and `ColorSignal::Cicp` (AVIF `nclx` under the
//! strict policy) in the library's AVIF round-trip test.
#![cfg(any(feature = "jpeg-read", feature = "jpeg-xl-read"))]
// Each test needs its own feature combination.
#![allow(dead_code, unused_imports)]

use ferrisma::pipeline::{
    ColorSignal, EncodeError, EncodeRequest, EncodedImage, EncoderBackend, ImageFormat,
    InputProfilePolicy, Output, OutputFormat, ProcessingError, ProcessingErrorKind, RefusalReason,
    RowEncoder, TransformOptions, transcode_with,
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
) -> Result<Output, ProcessingError> {
    let options = TransformOptions {
        output_format: OutputFormat::WebP,
        output_width: 4,
        output_height: 4,
        transform_threads: 1,
        profile_policy,
        ..TransformOptions::default()
    };
    transcode_with(bytes, format, options, &Discard)
}

fn jpeg(profile_policy: InputProfilePolicy) -> Result<Output, ProcessingError> {
    let bytes = include_bytes!("fixtures/sample.jpg");
    run(bytes, ImageFormat::Jpeg, profile_policy)
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

/// The JPEG XL reader describes its output with an ICC profile, so the two
/// policies without ICC conversion refuse it and name the opt-in.
#[cfg(feature = "jpeg-xl-read")]
#[test]
fn rendered_icc_without_the_opt_in_is_refused_with_its_reason() {
    let jxl = include_bytes!("fixtures/orientation/jxl-1.jxl");
    for policy in [
        InputProfilePolicy::Strict,
        InputProfilePolicy::AssumeSrgbIfMissing,
    ] {
        assert_eq!(
            refusal(run(jxl, ImageFormat::JpegXl, policy)),
            (
                ProcessingErrorKind::UnsupportedColor,
                Some(RefusalReason::RenderedIccRequiresOptIn)
            ),
            "{policy:?}"
        );
    }
}

/// The libjxl-rs adapter reports only the rendered ICC, without CICP beside
/// it (ADR-0015, 2026-11-12). (The former jxl-oxide reader reported the
/// header's enum encoding as CICP beside it, `RenderedIccWithAgreeingCicp`.)
#[cfg(all(feature = "jpeg-xl-read", feature = "icc"))]
#[test]
fn rendered_icc_conversion_reports_whether_cicp_was_checked_beside_it() {
    let jxl = include_bytes!("fixtures/jxl-color/lossless-p3.jxl");
    let output = run(jxl, ImageFormat::JpegXl, InputProfilePolicy::UseRenderedIcc).unwrap();
    assert_eq!(output.report.color_signal, ColorSignal::RenderedIcc);
}

/// AVIF \`nclx\` beside an ICC profile is checked and, when it agrees, named in
/// the report. Since 2026-11-15 only AVIF produces this signal; before, the
/// jxl-oxide reader did too.
#[cfg(all(feature = "avif-read", feature = "icc"))]
#[test]
fn agreeing_cicp_beside_the_rendered_icc_is_reported() {
    let avif = include_bytes!("fixtures/avif-color/icc-nclx-1-13-6.avif");
    let output = run(avif, ImageFormat::Avif, InputProfilePolicy::UseRenderedIcc).unwrap();
    assert_eq!(
        output.report.color_signal,
        ColorSignal::RenderedIccWithAgreeingCicp
    );
}

/// DCI-P3 (primaries 11, transfer 17) beside the rendered profile cannot be
/// checked against it, so it is refused rather than overridden by the ICC.
#[cfg(all(feature = "jpeg-xl-read", feature = "icc"))]
#[test]
fn unverifiable_cicp_beside_the_rendered_icc_is_refused_with_its_reason() {
    let jxl = include_bytes!("fixtures/jxl-color/lossless-dci-p3.jxl");
    assert_eq!(
        refusal(run(
            jxl,
            ImageFormat::JpegXl,
            InputProfilePolicy::UseRenderedIcc
        )),
        (
            ProcessingErrorKind::UnsupportedColor,
            Some(RefusalReason::CicpBesideIcc)
        )
    );
}

// ---------------------------------------------------------------------------
// The refusal classes added on 2026-11-15 (ADR-0017 amendment): each one with
// a real fixture through the public API. `supported_matrix.rs` asserts the
// same reasons for every rejected row of `docs/supported-matrix.md`.

#[cfg(feature = "avif-read")]
const AVIF_DATA: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/codecs/libavif-rs/upstream/libavif/tests/data/"
);

#[cfg(feature = "avif-read")]
fn avif(name: &str) -> Vec<u8> {
    std::fs::read(format!("{AVIF_DATA}{name}")).unwrap()
}

#[cfg(feature = "avif-read")]
fn avif_refusal(
    bytes: &[u8],
    policy: InputProfilePolicy,
) -> (ProcessingErrorKind, Option<RefusalReason>) {
    refusal(run(bytes, ImageFormat::Avif, policy))
}

const POLICIES: [InputProfilePolicy; 3] = [
    InputProfilePolicy::Strict,
    InputProfilePolicy::AssumeSrgbIfMissing,
    InputProfilePolicy::UseRenderedIcc,
];

/// PQ and HLG are refused by the JPEG XL reader from the transfer
/// function itself, under every policy (ADR-0016 rule 7).
#[cfg(feature = "jpeg-xl-read")]
#[test]
fn hdr_transfer_is_refused_with_its_reason_under_every_policy() {
    let fixtures: [&[u8]; 2] = [
        include_bytes!("fixtures/jxl-color/lossless-pq.jxl"),
        include_bytes!("fixtures/jxl-color/lossless-hlg.jxl"),
    ];
    for jxl in fixtures {
        for policy in POLICIES {
            assert_eq!(
                refusal(run(jxl, ImageFormat::JpegXl, policy)),
                (
                    ProcessingErrorKind::UnsupportedColor,
                    Some(RefusalReason::HdrTransfer)
                ),
                "{policy:?}"
            );
        }
    }
}

/// AVIF PQ CICP (no ICC) is named as HDR under every policy, also where the
/// rendered-ICC policy finds no profile to convert.
#[cfg(feature = "avif-read")]
#[test]
fn avif_pq_is_refused_as_hdr_under_every_policy() {
    for policy in POLICIES {
        assert_eq!(
            avif_refusal(&avif("colors_hdr_rec2020.avif"), policy),
            (
                ProcessingErrorKind::UnsupportedColor,
                Some(RefusalReason::HdrTransfer)
            ),
            "{policy:?}"
        );
    }
}

/// An enum colour encoding with an unknown colour space: the workspace
/// reader, like libjxl, rejects the file as invalid, so it is malformed input
/// without a refusal reason. (Until 2026-11-15 the jxl-oxide reader refused
/// it as `UnsupportedColorEncoding`; that reason is now produced by AVIF
/// CICP only, below.)
#[cfg(feature = "jpeg-xl-read")]
#[test]
fn unknown_jxl_colour_space_is_malformed_input() {
    let jxl = include_bytes!("fixtures/hostile/jxl-unknown-colour-space.jxl");
    for policy in POLICIES {
        assert_eq!(
            refusal(run(jxl, ImageFormat::JpegXl, policy)),
            (ProcessingErrorKind::MalformedInput, None),
            "{policy:?}"
        );
    }
}

/// SDR BT.2020 CICP (9/1) without an ICC profile is outside sRGB, Display P3
/// and linear sRGB.
#[cfg(feature = "avif-read")]
#[test]
fn sdr_cicp_outside_the_supported_families_is_refused_with_its_reason() {
    for policy in [
        InputProfilePolicy::Strict,
        InputProfilePolicy::AssumeSrgbIfMissing,
    ] {
        assert_eq!(
            avif_refusal(&avif("colors_text_wcg_sdr_rec2020.avif"), policy),
            (
                ProcessingErrorKind::UnsupportedColor,
                Some(RefusalReason::UnsupportedColorEncoding)
            ),
            "{policy:?}"
        );
    }
}

/// The rendered-ICC policy needs a profile: untagged JPEG and CICP-only
/// AVIF have none.
#[cfg(all(feature = "jpeg-read", feature = "icc"))]
#[test]
fn rendered_icc_policy_without_a_profile_is_refused_with_its_reason() {
    let expected = (
        ProcessingErrorKind::UnsupportedColor,
        Some(RefusalReason::RenderedIccMissing),
    );
    assert_eq!(refusal(jpeg(InputProfilePolicy::UseRenderedIcc)), expected);
    #[cfg(feature = "avif-read")]
    assert_eq!(
        avif_refusal(
            &avif("colors_sdr_srgb.avif"),
            InputProfilePolicy::UseRenderedIcc
        ),
        expected
    );
}

/// `colors_sdr_srgb.avif` with its container `nclx` matrix changed from 6
/// (BT.601) to 1 (BT.709): the container now contradicts the AV1 sequence
/// header.
#[cfg(feature = "avif-read")]
#[test]
fn contradictory_avif_color_signals_are_refused_with_their_reason() {
    let mut bytes = avif("colors_sdr_srgb.avif");
    let nclx = bytes
        .windows(8)
        .position(|window| window == b"colrnclx")
        .unwrap()
        + 8;
    assert_eq!(&bytes[nclx..nclx + 6], &[0, 1, 0, 13, 0, 6]);
    bytes[nclx + 5] = 1;
    for policy in POLICIES {
        assert_eq!(
            avif_refusal(&bytes, policy),
            (
                ProcessingErrorKind::UnsupportedColor,
                Some(RefusalReason::ContradictoryColorSignals)
            ),
            "{policy:?}"
        );
    }
}

/// CMYK JPEG, under every policy: the adapter refuses it before color.
#[cfg(feature = "jpeg-read")]
#[test]
fn cmyk_jpeg_is_refused_as_a_layout() {
    for policy in POLICIES {
        let bytes = include_bytes!("fixtures/cmyk.jpg");
        assert_eq!(
            refusal(run(bytes, ImageFormat::Jpeg, policy,)),
            (
                ProcessingErrorKind::UnsupportedInput,
                Some(RefusalReason::UnsupportedLayout)
            ),
            "{policy:?}"
        );
    }
}

/// Premultiplied alpha (libjxl's conformance fixture with
/// `alpha_associated`): libjxl-rs reports it as premultiplied; the
/// rendered-ICC stage refuses it for its alpha.
#[cfg(all(feature = "jpeg-xl-read", feature = "icc"))]
#[test]
fn associated_alpha_is_refused_with_its_reason() {
    let jxl = std::fs::read(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/codecs/libjxl-rs/tests/data/extra-channel-alpha-1-up-1-shift-0.jxl"
    ))
    .unwrap();
    assert_eq!(
        refusal(run(
            &jxl,
            ImageFormat::JpegXl,
            InputProfilePolicy::UseRenderedIcc
        )),
        (
            ProcessingErrorKind::UnsupportedInput,
            Some(RefusalReason::AlphaAssociation)
        )
    );
}

/// AVIF clean aperture and an `imir` before `irot` are not applied.
#[cfg(feature = "avif-read")]
#[test]
fn unsupported_geometric_transforms_are_refused_with_their_reason() {
    for policy in POLICIES {
        assert_eq!(
            avif_refusal(&avif("clap_irot_imir_non_essential.avif"), policy),
            (
                ProcessingErrorKind::UnsupportedInput,
                Some(RefusalReason::GeometricTransform)
            ),
            "{policy:?}"
        );
    }
}
