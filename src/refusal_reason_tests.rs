//! No `RefusalReason` is dead (issue #212): every variant is produced by a
//! real fixture through the public `transcode`. The match in `case` is
//! exhaustive, so a new variant does not compile until it has a fixture
//! here; `ALL` is checked against the same match. Needs every input
//! adapter, which the `cli` feature set provides (`ContradictoryColorSignals`,
//! `GeometricTransform` and `UnsupportedEssentialProperty` are AVIF-only
//! refusals).

use crate::pipeline::{
    ImageFormat, InputProfilePolicy, OutputFormat, RefusalReason, TransformOptions, transcode,
};

const ALL: [RefusalReason; 13] = [
    RefusalReason::Animation,
    RefusalReason::ColorSignalMissing,
    RefusalReason::RenderedIccRequiresOptIn,
    RefusalReason::CicpBesideIcc,
    RefusalReason::HdrTransfer,
    RefusalReason::UnsupportedColorEncoding,
    RefusalReason::RenderedIccMissing,
    RefusalReason::ContradictoryColorSignals,
    RefusalReason::UnsupportedLayout,
    RefusalReason::AlphaAssociation,
    RefusalReason::GeometricTransform,
    RefusalReason::UnsupportedEssentialProperty,
    RefusalReason::IccColorSpaceMismatch,
];

fn file(path: &str) -> Vec<u8> {
    std::fs::read(format!("{}/{path}", env!("CARGO_MANIFEST_DIR"))).unwrap()
}

const LIBAVIF: &str = "codecs/libavif-rs/upstream/libavif/tests/data/";
const LIBJXL: &str = "codecs/libjxl-rs/tests/data/";

/// `avif-1.avif` with the primary item's `pixi` (property 3 of `ipco`, one
/// entry in a version-0 small-index `ipma`) renamed to an unknown type and
/// its association marked essential, as in `tests/supported_matrix.rs`.
fn unknown_essential_property() -> Vec<u8> {
    let mut bytes = file("tests/fixtures/orientation/avif-1.avif");
    let pixi = bytes
        .windows(4)
        .position(|window| window == b"pixi")
        .unwrap();
    bytes[pixi..pixi + 4].copy_from_slice(b"xyzw");
    let ipma = bytes
        .windows(4)
        .position(|window| window == b"ipma")
        .unwrap()
        + 4;
    assert_eq!(&bytes[ipma..ipma + 8], &[0, 0, 0, 0, 0, 0, 0, 1]);
    let count = usize::from(bytes[ipma + 10]);
    let association = (ipma + 11..ipma + 11 + count)
        .find(|&at| bytes[at] & 0x7f == 3)
        .unwrap();
    bytes[association] |= 0x80;
    bytes
}

/// One fixture, format and policy per reason; the position in `ALL` is
/// returned with it.
fn case(reason: RefusalReason) -> (usize, Vec<u8>, ImageFormat, InputProfilePolicy) {
    use InputProfilePolicy::{AssumeSrgbIfMissing, Strict, UseRenderedIcc};
    let (index, path, format, policy) = match reason {
        RefusalReason::Animation => (
            0,
            "tests/fixtures/decoder-work/animated.webp".to_owned(),
            ImageFormat::WebP,
            AssumeSrgbIfMissing,
        ),
        RefusalReason::ColorSignalMissing => (
            1,
            "tests/fixtures/sample.jpg".to_owned(),
            ImageFormat::Jpeg,
            Strict,
        ),
        // A real embedded profile. (Until 2026-11-15 `sample.jxl`, whose
        // enum sRGB the strict policy now reads; owner decision, #222.)
        RefusalReason::RenderedIccRequiresOptIn => (
            2,
            "tests/fixtures/parity/rgb-p3-icc.jxl".to_owned(),
            ImageFormat::JpegXl,
            Strict,
        ),
        RefusalReason::CicpBesideIcc => (
            3,
            "tests/fixtures/avif-color/icc-nclx-1-1-6.avif".to_owned(),
            ImageFormat::Avif,
            UseRenderedIcc,
        ),
        RefusalReason::HdrTransfer => (
            4,
            "tests/fixtures/jxl-color/lossless-pq.jxl".to_owned(),
            ImageFormat::JpegXl,
            UseRenderedIcc,
        ),
        // BT.2020 SDR CICP without an ICC profile. (Until 2026-11-15 the
        // fixture was a JPEG XL enum encoding only jxl-oxide reported here.)
        RefusalReason::UnsupportedColorEncoding => (
            5,
            format!("{LIBAVIF}colors_text_wcg_sdr_rec2020.avif"),
            ImageFormat::Avif,
            Strict,
        ),
        // Untagged. (Until 2026-11-15 signalled sRGB AVIF, which the
        // rendered-ICC policy now converts as a typed signal.)
        RefusalReason::RenderedIccMissing => (
            6,
            "tests/fixtures/sample.jpg".to_owned(),
            ImageFormat::Jpeg,
            UseRenderedIcc,
        ),
        RefusalReason::ContradictoryColorSignals => {
            let mut bytes = file(&format!("{LIBAVIF}colors_sdr_srgb.avif"));
            let nclx = bytes
                .windows(8)
                .position(|window| window == b"colrnclx")
                .unwrap()
                + 8;
            // Matrix 6 (BT.601) -> 1 (BT.709): contradicts the AV1 header.
            assert_eq!(&bytes[nclx..nclx + 6], &[0, 1, 0, 13, 0, 6]);
            bytes[nclx + 5] = 1;
            return (7, bytes, ImageFormat::Avif, Strict);
        }
        RefusalReason::UnsupportedLayout => (
            8,
            "tests/fixtures/cmyk.jpg".to_owned(),
            ImageFormat::Jpeg,
            AssumeSrgbIfMissing,
        ),
        RefusalReason::AlphaAssociation => (
            9,
            format!("{LIBJXL}extra-channel-alpha-1-up-1-shift-0.jxl"),
            ImageFormat::JpegXl,
            UseRenderedIcc,
        ),
        RefusalReason::GeometricTransform => (
            10,
            format!("{LIBAVIF}clap_irot_imir_non_essential.avif"),
            ImageFormat::Avif,
            AssumeSrgbIfMissing,
        ),
        // Since 2026-11-15: an RGB profile beside the samples of a gray
        // JPEG (`tests/fixtures/parity/README.md`).
        RefusalReason::IccColorSpaceMismatch => (
            12,
            "tests/fixtures/parity/gray-rgb-icc.jpg".to_owned(),
            ImageFormat::Jpeg,
            UseRenderedIcc,
        ),
        RefusalReason::UnsupportedEssentialProperty => {
            return (
                11,
                unknown_essential_property(),
                ImageFormat::Avif,
                AssumeSrgbIfMissing,
            );
        }
    };
    (index, file(&path), format, policy)
}

#[test]
fn every_refusal_reason_is_produced_by_a_fixture() {
    let mut seen = [false; ALL.len()];
    for reason in ALL {
        let (index, bytes, format, profile_policy) = case(reason);
        assert_eq!(ALL[index], reason, "ALL and case disagree");
        assert!(!seen[index], "{reason:?} listed twice");
        seen[index] = true;
        let options = TransformOptions {
            output_format: OutputFormat::WebP,
            output_width: 4,
            output_height: 4,
            transform_threads: 1,
            profile_policy,
            ..TransformOptions::default()
        };
        let error = transcode(&bytes, format, options).expect_err("the fixture must be refused");
        assert_eq!(error.refusal_reason(), Some(reason), "{error}");
    }
    assert!(seen.iter().all(|seen| *seen));
}
