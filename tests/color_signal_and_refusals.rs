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
    unused_mut,
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
    refusal(run(
        bytes,
        ImageFormat::Avif,
        policy,
        JpegXlDecoder::default(),
    ))
}

const POLICIES: [InputProfilePolicy; 3] = [
    InputProfilePolicy::Strict,
    InputProfilePolicy::AssumeSrgbIfMissing,
    InputProfilePolicy::UseRenderedIcc,
];

/// PQ and HLG are refused by both JPEG XL readers from the transfer
/// function itself, under every policy (ADR-0016 rule 7).
#[cfg(any(feature = "jpeg-xl-read", feature = "jpeg-xl-own-read"))]
#[test]
fn hdr_transfer_is_refused_with_its_reason_under_every_policy() {
    let fixtures: [&[u8]; 2] = [
        include_bytes!("fixtures/jxl-color/lossless-pq.jxl"),
        include_bytes!("fixtures/jxl-color/lossless-hlg.jxl"),
    ];
    for jxl in fixtures {
        for decoder in jxl_decoders() {
            for policy in POLICIES {
                assert_eq!(
                    refusal(run(jxl, ImageFormat::JpegXl, policy, decoder)),
                    (
                        ProcessingErrorKind::UnsupportedColor,
                        Some(RefusalReason::HdrTransfer)
                    ),
                    "{decoder:?} {policy:?}"
                );
            }
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

/// An enum colour encoding with an unknown colour space is well-formed, but
/// jxl-oxide cannot render it.
#[cfg(feature = "jpeg-xl-read")]
#[test]
fn unrenderable_jxl_colour_encoding_is_refused_with_its_reason() {
    let jxl = include_bytes!("fixtures/hostile/jxl-unknown-colour-space.jxl");
    for policy in POLICIES {
        assert_eq!(
            refusal(run(
                jxl,
                ImageFormat::JpegXl,
                policy,
                JpegXlDecoder::JxlOxide
            )),
            (
                ProcessingErrorKind::UnsupportedColor,
                Some(RefusalReason::UnsupportedColorEncoding)
            ),
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
            refusal(run(
                bytes,
                ImageFormat::Jpeg,
                policy,
                JpegXlDecoder::default()
            )),
            (
                ProcessingErrorKind::UnsupportedInput,
                Some(RefusalReason::UnsupportedLayout)
            ),
            "{policy:?}"
        );
    }
}

/// Premultiplied alpha (libjxl's conformance fixture with
/// `alpha_associated`): libjxl-rs reports it as premultiplied, jxl-oxide as
/// unspecified; the rendered-ICC stage refuses both for their alpha.
#[cfg(all(
    any(feature = "jpeg-xl-read", feature = "jpeg-xl-own-read"),
    feature = "icc"
))]
#[test]
fn associated_alpha_is_refused_with_its_reason() {
    let jxl = std::fs::read(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/codecs/libjxl-rs/tests/data/extra-channel-alpha-1-up-1-shift-0.jxl"
    ))
    .unwrap();
    for decoder in jxl_decoders() {
        assert_eq!(
            refusal(run(
                &jxl,
                ImageFormat::JpegXl,
                InputProfilePolicy::UseRenderedIcc,
                decoder
            )),
            (
                ProcessingErrorKind::UnsupportedInput,
                Some(RefusalReason::AlphaAssociation)
            ),
            "{decoder:?}"
        );
    }
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
