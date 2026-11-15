//! No `RefusalReason` is dead (issue #212): every variant is produced by a
//! real fixture through the public `transcode`. The match in `case` is
//! exhaustive, so a new variant does not compile until it has a fixture
//! here; `ALL` is checked against the same match. Needs every input
//! adapter, which the `cli` feature set provides (`ContradictoryColorSignals`
//! and `GeometricTransform` are AVIF-only refusals).

use crate::pipeline::{
    ImageFormat, InputProfilePolicy, JpegXlDecoder, OutputFormat, RefusalReason, TransformOptions,
    transcode,
};

const ALL: [RefusalReason; 11] = [
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
];

fn file(path: &str) -> Vec<u8> {
    std::fs::read(format!("{}/{path}", env!("CARGO_MANIFEST_DIR"))).unwrap()
}

const LIBAVIF: &str = "codecs/libavif-rs/upstream/libavif/tests/data/";
const LIBJXL: &str = "codecs/libjxl-rs/tests/data/";

/// One fixture, format, policy and JPEG XL reader per reason; the position
/// in `ALL` is returned with it.
fn case(
    reason: RefusalReason,
) -> (
    usize,
    Vec<u8>,
    ImageFormat,
    InputProfilePolicy,
    JpegXlDecoder,
) {
    use InputProfilePolicy::{AssumeSrgbIfMissing, Strict, UseRenderedIcc};
    use JpegXlDecoder::{JxlOxide, LibjxlRs};
    let (index, path, format, policy, decoder) = match reason {
        RefusalReason::Animation => (
            0,
            "tests/fixtures/decoder-work/animated.webp".to_owned(),
            ImageFormat::WebP,
            AssumeSrgbIfMissing,
            LibjxlRs,
        ),
        RefusalReason::ColorSignalMissing => (
            1,
            "tests/fixtures/sample.jpg".to_owned(),
            ImageFormat::Jpeg,
            Strict,
            LibjxlRs,
        ),
        RefusalReason::RenderedIccRequiresOptIn => (
            2,
            "tests/fixtures/sample.jxl".to_owned(),
            ImageFormat::JpegXl,
            Strict,
            LibjxlRs,
        ),
        RefusalReason::CicpBesideIcc => (
            3,
            "tests/fixtures/avif-color/icc-nclx-1-1-6.avif".to_owned(),
            ImageFormat::Avif,
            UseRenderedIcc,
            LibjxlRs,
        ),
        RefusalReason::HdrTransfer => (
            4,
            "tests/fixtures/jxl-color/lossless-pq.jxl".to_owned(),
            ImageFormat::JpegXl,
            UseRenderedIcc,
            LibjxlRs,
        ),
        RefusalReason::UnsupportedColorEncoding => (
            5,
            "tests/fixtures/hostile/jxl-unknown-colour-space.jxl".to_owned(),
            ImageFormat::JpegXl,
            Strict,
            JxlOxide,
        ),
        RefusalReason::RenderedIccMissing => (
            6,
            format!("{LIBAVIF}colors_sdr_srgb.avif"),
            ImageFormat::Avif,
            UseRenderedIcc,
            LibjxlRs,
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
            return (7, bytes, ImageFormat::Avif, Strict, LibjxlRs);
        }
        RefusalReason::UnsupportedLayout => (
            8,
            "tests/fixtures/cmyk.jpg".to_owned(),
            ImageFormat::Jpeg,
            AssumeSrgbIfMissing,
            LibjxlRs,
        ),
        RefusalReason::AlphaAssociation => (
            9,
            format!("{LIBJXL}extra-channel-alpha-1-up-1-shift-0.jxl"),
            ImageFormat::JpegXl,
            UseRenderedIcc,
            LibjxlRs,
        ),
        RefusalReason::GeometricTransform => (
            10,
            format!("{LIBAVIF}clap_irot_imir_non_essential.avif"),
            ImageFormat::Avif,
            AssumeSrgbIfMissing,
            LibjxlRs,
        ),
    };
    (index, file(&path), format, policy, decoder)
}

#[test]
fn every_refusal_reason_is_produced_by_a_fixture() {
    let mut seen = [false; ALL.len()];
    for reason in ALL {
        let (index, bytes, format, profile_policy, jxl_decoder) = case(reason);
        assert_eq!(ALL[index], reason, "ALL and case disagree");
        assert!(!seen[index], "{reason:?} listed twice");
        seen[index] = true;
        let options = TransformOptions {
            output_format: OutputFormat::WebP,
            output_width: 4,
            output_height: 4,
            transform_threads: 1,
            profile_policy,
            jxl_decoder,
            ..TransformOptions::default()
        };
        let error = transcode(&bytes, format, options).expect_err("the fixture must be refused");
        assert_eq!(error.refusal_reason(), Some(reason), "{error}");
    }
    assert!(seen.iter().all(|seen| *seen));
}
