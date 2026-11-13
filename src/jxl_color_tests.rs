//! What the JPEG XL adapters declare for their emitted samples, and how the
//! engine interprets it (issue #41, ADR-0015/0016/0023).
//!
//! The fixtures in `tests/fixtures/jxl-color` differ from two independently
//! encoded files only in their enum color encoding (see its README), so the
//! emitted samples are known exactly for the lossless ones. Every check runs
//! against each compiled JPEG XL decoder, so the same expectations apply
//! when the default decoder changes (#55).
#![allow(clippy::chunks_exact_to_as_chunks)]

use std::sync::Mutex;

use crate::decoder::DecodeSource;
use crate::error::Error;
use crate::pipeline::{
    EncodeError, EncodeRequest, EncodedImage, EncoderBackend, ImageFormat, InputProfilePolicy,
    JpegXlDecoder, OutputFormat, ProcessingErrorKind, ResizeFilter, RowEncoder, TransformOptions,
    transcode_with,
};

#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::doc_markdown,
    clippy::many_single_char_names,
    clippy::chunks_exact_to_as_chunks
)]
#[path = "../tests/support/resize_oracle.rs"]
pub(crate) mod oracle;

fn fixture(name: &str) -> Vec<u8> {
    std::fs::read(format!(
        "{}/tests/fixtures/jxl-color/{name}.jxl",
        env!("CARGO_MANIFEST_DIR")
    ))
    .unwrap()
}

#[allow(clippy::vec_init_then_push)] // Entries depend on the features.
fn decoders() -> Vec<JpegXlDecoder> {
    let mut all = Vec::new();
    #[cfg(feature = "jpeg-xl-read")]
    all.push(JpegXlDecoder::JxlOxide);
    #[cfg(feature = "jpeg-xl-own-read")]
    all.push(JpegXlDecoder::LibjxlRs);
    all
}

/// What one decoder's adapter declares for a file, or why it refuses it.
fn open(decoder: JpegXlDecoder, bytes: &[u8]) -> Result<crate::decoder::DecoderInfo, Error> {
    match decoder {
        #[cfg(feature = "jpeg-xl-read")]
        JpegXlDecoder::JxlOxide => {
            crate::codecs::jxl_oxide::JxlSource::new(bytes).map(|source| source.info().clone())
        }
        #[cfg(feature = "jpeg-xl-own-read")]
        JpegXlDecoder::LibjxlRs => {
            crate::codecs::jxl::JxlSource::new(bytes).map(|source| source.info().clone())
        }
        #[allow(unreachable_patterns)]
        _ => unreachable!(),
    }
}

/// The rendered signals of one decoder: (CICP, the ICC's own `cicp` tag).
fn signals(decoder: JpegXlDecoder, bytes: &[u8]) -> (Option<[u8; 4]>, Option<[u8; 4]>) {
    let info = open(decoder, bytes).unwrap();
    let icc = info
        .rendered_icc
        .expect("JPEG XL adapters always render an ICC");
    (info.rendered_cicp, icc_cicp_tag(&icc))
}

/// The payload of an ICC v4.4 `cicp` tag, read by tag-table offset.
fn icc_cicp_tag(icc: &[u8]) -> Option<[u8; 4]> {
    let count = u32::from_be_bytes(icc[128..132].try_into().unwrap()) as usize;
    (0..count).find_map(|i| {
        let entry = &icc[132 + 12 * i..144 + 12 * i];
        (&entry[0..4] == b"cicp").then(|| {
            let offset = u32::from_be_bytes(entry[4..8].try_into().unwrap()) as usize;
            icc[offset + 8..offset + 12].try_into().unwrap()
        })
    })
}

/// The declared meaning of each fixture's samples, per decoder. jxl-oxide
/// reports CICP beside its synthesized ICC; the libjxl-rs adapter reports
/// only the ICC, whose `cicp` tag carries the same code points. PQ and HLG
/// files are refused by both adapters before any profile is declared
/// (review of 2026-11-12); before that change they declared `[9, 16, 0, 1]`
/// and `[9, 18, 0, 1]` here and were rejected one stage later.
#[test]
fn adapters_declare_the_emitted_encoding() {
    for decoder in decoders() {
        for name in ["lossless-pq", "lossless-hlg", "xyb-pq"]
            .into_iter()
            .chain(HDR_WITHOUT_CICP)
        {
            assert_eq!(
                open(decoder, &fixture(name)).err(),
                Some(Error::UnsupportedColorEncoding),
                "{decoder:?} {name}"
            );
        }
        for (name, cicp) in [
            ("lossless-p3", [12, 13, 0, 1]),
            ("lossless-linear-srgb", [1, 8, 0, 1]),
            ("lossless-dci-p3", [11, 17, 0, 1]),
            ("xyb-p3", [12, 13, 0, 1]),
        ] {
            let (rendered, tagged) = signals(decoder, &fixture(name));
            match decoder {
                JpegXlDecoder::JxlOxide => {
                    // Regression: jxl-oxide 0.12.6 itself reports primaries
                    // 11 (DCI white) for D65 Display P3.
                    assert_eq!(rendered, Some(cicp), "{decoder:?} {name}");
                    if let Some(tag) = tagged {
                        assert_eq!(tag[..2], cicp[..2], "{decoder:?} {name}");
                    }
                }
                JpegXlDecoder::LibjxlRs => {
                    assert_eq!(rendered, None, "{decoder:?} {name}");
                    assert_eq!(tagged, Some(cicp), "{decoder:?} {name}");
                }
            }
        }
    }
}

struct Capture(Mutex<Vec<u8>>);
struct CaptureRows<'a>(&'a Mutex<Vec<u8>>);

impl EncoderBackend for Capture {
    fn format(&self) -> OutputFormat {
        OutputFormat::WebP
    }
    fn name(&self) -> &'static str {
        "test-row-capture"
    }
    fn begin(&self, _: &EncodeRequest) -> Result<Box<dyn RowEncoder + '_>, EncodeError> {
        Ok(Box::new(CaptureRows(&self.0)))
    }
}

impl RowEncoder for CaptureRows<'_> {
    fn write_row(&mut self, _: u32, row: &[u8]) -> Result<(), EncodeError> {
        self.0.lock().unwrap().extend_from_slice(row);
        Ok(())
    }
    fn finish(self: Box<Self>) -> Result<EncodedImage, EncodeError> {
        Ok(EncodedImage::new(OutputFormat::WebP, Vec::new(), 0, false))
    }
}

fn run(
    bytes: &[u8],
    decoder: JpegXlDecoder,
    policy: InputProfilePolicy,
    size: (u32, u32),
) -> Result<Vec<u8>, ProcessingErrorKind> {
    let capture = Capture(Mutex::new(Vec::new()));
    transcode_with(
        bytes,
        ImageFormat::JpegXl,
        TransformOptions {
            output_format: OutputFormat::WebP,
            output_width: size.0,
            output_height: size.1,
            filter: ResizeFilter::Area,
            profile_policy: policy,
            jxl_decoder: decoder,
            transform_threads: 1,
            ..TransformOptions::default()
        },
        &capture,
    )
    .map_err(|error| error.kind())?;
    Ok(capture.0.into_inner().unwrap())
}

/// The lossless fixtures' samples: the 24 x 16 orientation pattern.
#[cfg(feature = "icc")]
fn pattern() -> Vec<u8> {
    include_bytes!("../tests/fixtures/orientation/jxl-1.rgb").to_vec()
}

/// Bound per decoded sample: the ADR-0023 moxcms versus reference-matrix
/// bound (`2e-3` linear) for a profile the engine did not author.
#[cfg(feature = "icc")]
const BOUND: f64 = 2e-3;

#[cfg(feature = "icc")]
fn assert_within(actual: &[u8], expected_linear: &[[f64; 3]], label: &str) {
    assert_eq!(actual.len(), expected_linear.len() * 4, "{label}");
    for (px, want) in actual.chunks_exact(4).zip(expected_linear) {
        assert_eq!(px[3], 255, "{label}");
        for c in 0..3 {
            let lo = oracle::encode8(want[c] - BOUND);
            let hi = oracle::encode8(want[c] + BOUND);
            assert!(
                (lo..=hi).contains(&px[c]),
                "{label}: code {} outside {lo}..={hi}",
                px[c]
            );
        }
    }
}

/// Supported SDR families: the rendered-ICC transform of each decoder's
/// declared profile reproduces the exactly known samples, converted by the
/// independent oracle (primaries-derived P3 matrix, IEC sRGB curves). The
/// identity geometry isolates color from resampling.
#[cfg(feature = "icc")]
#[test]
fn rendered_icc_reproduces_known_samples_for_sdr_families() {
    let p3 = oracle::p3_to_srgb_matrix();
    let rgb = pattern();
    let as_p3: Vec<[f64; 3]> = rgb
        .chunks_exact(3)
        .map(|p| oracle::decode_rgb([p[0], p[1], p[2]], oracle::Primaries::DisplayP3, &p3))
        .collect();
    let as_linear: Vec<[f64; 3]> = rgb
        .chunks_exact(3)
        .map(|p| [0, 1, 2].map(|c| f64::from(p[c]) / 255.0))
        .collect();
    for decoder in decoders() {
        for (name, expected) in [
            ("lossless-p3", &as_p3),
            ("lossless-linear-srgb", &as_linear),
        ] {
            let actual = run(
                &fixture(name),
                decoder,
                InputProfilePolicy::UseRenderedIcc,
                (24, 16),
            )
            .unwrap_or_else(|kind| panic!("{decoder:?} {name}: {kind:?}"));
            assert_within(&actual, expected, &format!("{decoder:?} {name}"));
        }
    }
}

/// XYB input is rendered by each decoder into the declared Display P3; the
/// two independent decoders agree after the same explicit ICC stage. The
/// samples are lossy, so there is no exact reference: the bound is two codes,
/// the decoders' float rendering differences plus the profile bound above.
#[cfg(all(
    feature = "icc",
    feature = "jpeg-xl-read",
    feature = "jpeg-xl-own-read"
))]
#[test]
fn both_decoders_agree_on_xyb_rendered_into_display_p3() {
    let bytes = fixture("xyb-p3");
    let oxide = run(
        &bytes,
        JpegXlDecoder::JxlOxide,
        InputProfilePolicy::UseRenderedIcc,
        (7, 5),
    )
    .unwrap();
    let libjxl = run(
        &bytes,
        JpegXlDecoder::LibjxlRs,
        InputProfilePolicy::UseRenderedIcc,
        (7, 5),
    )
    .unwrap();
    let largest = oxide
        .iter()
        .zip(&libjxl)
        .map(|(a, b)| a.abs_diff(*b))
        .max()
        .unwrap();
    println!("xyb-p3: largest jxl-oxide/libjxl-rs difference {largest} codes");
    assert!(largest <= 2);
}

/// Controlled rejection: PQ and HLG (ADR-0016 rule 7) and a CICP family the
/// engine cannot evaluate are refused under the rendered-ICC policy, with no
/// output rows; the strict policy refuses every ICC-described JPEG XL.
#[test]
fn hdr_and_unverifiable_encodings_are_rejected() {
    for decoder in decoders() {
        for name in [
            "lossless-pq",
            "lossless-hlg",
            "xyb-pq",
            "lossless-dci-p3",
            "lossless-p3",
        ] {
            assert_eq!(
                run(&fixture(name), decoder, InputProfilePolicy::Strict, (3, 2)),
                Err(ProcessingErrorKind::UnsupportedColor),
                "{decoder:?} {name} strict"
            );
        }
        #[cfg(feature = "icc")]
        for name in ["lossless-pq", "lossless-hlg", "xyb-pq", "lossless-dci-p3"] {
            assert_eq!(
                run(
                    &fixture(name),
                    decoder,
                    InputProfilePolicy::UseRenderedIcc,
                    (3, 2)
                ),
                Err(ProcessingErrorKind::UnsupportedColor),
                "{decoder:?} {name} rendered ICC"
            );
        }
    }
}

/// Review of 2026-11-12: PQ and HLG must be refused from the transfer
/// function itself. With custom primaries (or, for libjxl, a white point
/// other than D65 or DCI) no CICP code point exists, neither beside nor
/// inside the rendered ICC, so a check keyed on CICP let the synthesized
/// PQ/HLG approximation profile through `UseRenderedIcc` and converted it as
/// SDR. Every decoder and every input profile policy must refuse these files.
#[test]
fn hdr_transfer_is_rejected_without_any_cicp_equivalent() {
    #[allow(unused_mut)] // `UseRenderedIcc` needs the `icc` feature.
    let mut policies = vec![
        InputProfilePolicy::Strict,
        InputProfilePolicy::AssumeSrgbIfMissing,
    ];
    #[cfg(feature = "icc")]
    policies.push(InputProfilePolicy::UseRenderedIcc);
    let mut accepted = Vec::new();
    for decoder in decoders() {
        for name in HDR_WITHOUT_CICP {
            for &policy in &policies {
                let result = run(&fixture(name), decoder, policy, (3, 2)).map(|_| ());
                if result != Err(ProcessingErrorKind::UnsupportedColor) {
                    accepted.push(format!("{decoder:?} {name} {policy:?}: {result:?}"));
                }
            }
        }
    }
    assert!(accepted.is_empty(), "{accepted:#?}");
}

/// PQ/HLG fixtures whose gamut has no H.273 code point.
const HDR_WITHOUT_CICP: [&str; 4] = [
    "lossless-custom-pq",
    "lossless-custom-hlg",
    "lossless-e-white-pq",
    "xyb-custom-pq",
];

/// The new fixtures declare what their names say, read by jxl-oxide's own
/// header parser rather than by either adapter.
#[cfg(feature = "jpeg-xl-read")]
#[test]
fn hdr_fixtures_without_cicp_declare_their_enum_encoding() {
    use jxl_oxide::JxlImage;
    use jxl_oxide::color::{ColourEncoding, Primaries, TransferFunction, WhitePoint};
    for (name, custom, white, tf) in [
        (
            "lossless-custom-pq",
            true,
            WhitePoint::D65,
            TransferFunction::Pq,
        ),
        (
            "lossless-custom-hlg",
            true,
            WhitePoint::D65,
            TransferFunction::Hlg,
        ),
        (
            "lossless-e-white-pq",
            false,
            WhitePoint::E,
            TransferFunction::Pq,
        ),
        ("xyb-custom-pq", true, WhitePoint::D65, TransferFunction::Pq),
    ] {
        let bytes = fixture(name);
        let image = JxlImage::builder().read(bytes.as_slice()).unwrap();
        let ColourEncoding::Enum(encoding) = &image.image_header().metadata.colour_encoding else {
            panic!("{name}: enum encoding expected");
        };
        assert_eq!(
            matches!(encoding.primaries, Primaries::Custom { .. }),
            custom,
            "{name}"
        );
        assert_eq!(encoding.white_point, white, "{name}");
        assert_eq!(encoding.tf, tf, "{name}");
    }
}
