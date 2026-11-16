//! What the JPEG XL reader declares for its emitted samples, and how the
//! engine interprets it (issue #41, ADR-0015/0016/0023).
//!
//! The fixtures in `tests/fixtures/jxl-color` differ from two independently
//! encoded files only in their enum color encoding (see its README), so the
//! emitted samples are known exactly for the lossless ones. The workspace
//! libjxl-rs port is the only reader since 2026-11-15 (ADR-0032); jxl-oxide
//! remains here as an independent test-only cross-check.
#![allow(clippy::chunks_exact_to_as_chunks)]

use std::sync::Mutex;

use crate::decoder::DecodeSource;
use crate::error::Error;
use crate::pipeline::Threads;
use crate::pipeline::{
    EncodeError, EncodeRequest, EncodedImage, EncoderBackend, ImageFormat, InputProfilePolicy,
    OutputFormat, ProcessingErrorKind, ResizeFilter, RowEncoder, TransformOptions, transcode_with,
};
use crate::pipeline::{OutputGeometry, OutputSettings};

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

/// What the reader's adapter declares for a file, or why it refuses it.
fn open(bytes: &[u8]) -> Result<crate::decoder::DecoderInfo, Error> {
    crate::codecs::jxl::JxlSource::new(bytes).map(|source| source.info().clone())
}

/// The reader's typed signal for a file, and the `cicp` tag of the profile
/// libjxl synthesizes for the same DATA encoding (read through the bridge
/// directly; the adapter no longer reports that profile).
fn signals(bytes: &[u8]) -> (Option<[u8; 4]>, Option<[u8; 4]>) {
    let info = open(bytes).unwrap();
    assert_eq!(info.rendered_icc, None, "a typed signal stands alone");
    let image = ferrisma_jxl_decoder::inspect(bytes, 1 << 24).unwrap();
    (info.rendered_cicp, icc_cicp_tag(&image.rendered_icc))
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

/// The declared meaning of each fixture's samples. Since 2026-11-15 (owner
/// decision, #222) an enum encoding with an H.273 code point is reported as
/// that CICP and without the profile libjxl synthesizes from it; the code
/// point equals the `cicp` tag libjxl itself writes into that profile, so
/// the adapter's mapping agrees with the original's. (Before, the adapter
/// reported only the profile, and this test read the code point from its
/// tag.) PQ and HLG files are refused before any signal is declared
/// (review of 2026-11-12); before that change they declared
/// `[9, 16, 0, 1]` and `[9, 18, 0, 1]` here and were rejected one stage
/// later.
#[test]
fn adapters_declare_the_emitted_encoding() {
    for name in ["lossless-pq", "lossless-hlg", "xyb-pq"]
        .into_iter()
        .chain(HDR_WITHOUT_CICP)
    {
        assert_eq!(
            open(&fixture(name)).err(),
            Some(Error::HdrTransfer),
            "{name}"
        );
    }
    for (name, cicp) in [
        ("lossless-p3", [12, 13, 0, 1]),
        ("lossless-linear-srgb", [1, 8, 0, 1]),
        ("lossless-dci-p3", [11, 17, 0, 1]),
        ("xyb-p3", [12, 13, 0, 1]),
    ] {
        let (rendered, tagged) = signals(&fixture(name));
        assert_eq!(rendered, Some(cicp), "{name}");
        assert_eq!(tagged, Some(cicp), "{name}: libjxl's own code point");
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
    policy: InputProfilePolicy,
    size: (u32, u32),
) -> Result<Vec<u8>, ProcessingErrorKind> {
    let capture = Capture(Mutex::new(Vec::new()));
    transcode_with(
        bytes,
        ImageFormat::JpegXl,
        &TransformOptions::new(OutputGeometry::Exact {
            width: size.0,
            height: size.1,
        })
        .with_output(OutputSettings::default_for(OutputFormat::WebP))
        .with_filter(ResizeFilter::Area)
        .with_profile_policy(policy)
        .with_transform_threads(Threads::ONE),
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

/// Supported SDR families: the rendered-ICC transform of the reader's
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
    for (name, expected) in [
        ("lossless-p3", &as_p3),
        ("lossless-linear-srgb", &as_linear),
    ] {
        let actual = run(&fixture(name), InputProfilePolicy::UseRenderedIcc, (24, 16))
            .unwrap_or_else(|kind| panic!("{name}: {kind:?}"));
        assert_within(&actual, expected, name);
    }
}

/// XYB input is rendered into the declared Display P3: the reader's own
/// samples agree with jxl-oxide, an independent decoder kept as a test-only
/// cross-check since 2026-11-15. The samples are lossy, so there is no exact
/// reference; the bound is two codes of the two decoders' float rendering
/// differences. (Until 2026-11-15 both readers were compared after the
/// same explicit ICC stage, with the same bound.)
#[test]
fn the_reader_agrees_with_jxl_oxide_on_xyb_rendered_into_display_p3() {
    use crate::geometry::Rect;
    use crate::memory::MemoryBudget;

    let bytes = fixture("xyb-p3");
    let mut source = crate::codecs::jxl::JxlSource::new(&bytes).unwrap();
    let size = source.info().size;
    let mut own = Vec::new();
    source
        .read_region(
            Rect::new(0, 0, size.width, size.height).unwrap(),
            &MemoryBudget::new(1 << 20),
            &mut |_, row| {
                let crate::decoder::DecodedRow::F32(row) = row else {
                    panic!("JPEG XL rows are float")
                };
                own.extend_from_slice(row);
                Ok(())
            },
        )
        .unwrap();
    let image = jxl_oxide::JxlImage::builder()
        .read(bytes.as_slice())
        .unwrap();
    let render = image.render_frame(0).unwrap();
    let mut stream = render.stream();
    assert_eq!(stream.channels(), 3);
    let mut oxide = vec![0_u8; own.len()];
    stream.write_to_buffer(&mut oxide);
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // Clamped first.
    let largest = own
        .iter()
        .map(|&v| (v * 255.0).round().clamp(0.0, 255.0) as u8)
        .zip(&oxide)
        .map(|(a, b)| a.abs_diff(*b))
        .max()
        .unwrap();
    println!("xyb-p3: largest libjxl-rs/jxl-oxide difference {largest} codes");
    assert!(largest <= 2);
}

/// Controlled rejection: PQ and HLG (ADR-0016 rule 7) and a CICP family the
/// engine cannot evaluate are refused under the strict and the rendered-ICC
/// policy, with no output rows. Display P3 passes the strict policy as a
/// typed signal since 2026-11-15 (owner decision, #222; before, the strict
/// policy refused it with the other files here, as every ICC-described
/// JPEG XL).
#[test]
fn hdr_and_unverifiable_encodings_are_rejected() {
    assert!(run(&fixture("lossless-p3"), InputProfilePolicy::Strict, (3, 2)).is_ok());
    for name in ["lossless-pq", "lossless-hlg", "xyb-pq", "lossless-dci-p3"] {
        assert_eq!(
            run(&fixture(name), InputProfilePolicy::Strict, (3, 2)),
            Err(ProcessingErrorKind::UnsupportedColor),
            "{name} strict"
        );
    }
    #[cfg(feature = "icc")]
    for name in ["lossless-pq", "lossless-hlg", "xyb-pq", "lossless-dci-p3"] {
        assert_eq!(
            run(&fixture(name), InputProfilePolicy::UseRenderedIcc, (3, 2)),
            Err(ProcessingErrorKind::UnsupportedColor),
            "{name} rendered ICC"
        );
    }
}

/// Review of 2026-11-12: PQ and HLG must be refused from the transfer
/// function itself. With custom primaries (or, for libjxl, a white point
/// other than D65 or DCI) no CICP code point exists, neither beside nor
/// inside the rendered ICC, so a check keyed on CICP let the synthesized
/// PQ/HLG approximation profile through `UseRenderedIcc` and converted it as
/// SDR. Every input profile policy must refuse these files.
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
    for name in HDR_WITHOUT_CICP {
        for &policy in &policies {
            let result = run(&fixture(name), policy, (3, 2)).map(|_| ());
            if result != Err(ProcessingErrorKind::UnsupportedColor) {
                accepted.push(format!("{name} {policy:?}: {result:?}"));
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
/// header parser (test-only) rather than by the reader.
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
