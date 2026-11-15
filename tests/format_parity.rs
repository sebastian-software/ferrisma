//! Format parity (ADR-0043 D5): one table per property of an image that
//! several formats can carry, with one case per format that can carry it,
//! run through the public `transcode_with`. Each case asserts the same
//! outcome class, error kind, `RefusalReason`, `ColorSignal` and
//! `discarded` entries as the other formats of its row. Where formats
//! differ legitimately, the case asserts the difference and its comment
//! names the format reason; `docs/supported-matrix.md` repeats it. A format
//! that cannot carry a property has no case, and the comment says so.
//!
//! Fixtures are real files from the repository; the ones created for these
//! rows are in `tests/fixtures/parity` (provenance in its README). Run with
//! every adapter: `cargo test -p ferrisma --features cli --test
//! format_parity`; without the AVIF features (x86-64 macOS) the AVIF cases
//! are compiled out and the rest still run.
#![cfg(all(
    feature = "jpeg-read",
    feature = "webp-read",
    feature = "webp-write",
    feature = "jpeg-xl-read",
    feature = "jpeg-xl-write",
    feature = "icc"
))]

use std::sync::Mutex;

use ferrisma::pipeline::{
    ColorSignal, Crop, Discarded, EncodeError, EncodeRequest, EncodedImage, EncoderBackend,
    ImageFormat, InputProfilePolicy, JpegXlOptions, OutputFormat, ProcessingErrorKind,
    RefusalReason, RowEncoder, TransformOptions, transcode, transcode_with,
};

const ROOT: &str = env!("CARGO_MANIFEST_DIR");
const JXL_DATA: &str = "codecs/libjxl-rs/tests/data/";
#[cfg(feature = "avif-read")]
const AVIF_DATA: &str = "codecs/libavif-rs/upstream/libavif/tests/data/";

const STRICT: InputProfilePolicy = InputProfilePolicy::Strict;
const ASSUME: InputProfilePolicy = InputProfilePolicy::AssumeSrgbIfMissing;
const RENDERED: InputProfilePolicy = InputProfilePolicy::UseRenderedIcc;
const POLICIES: [InputProfilePolicy; 3] = [STRICT, ASSUME, RENDERED];

fn read(path: &str) -> Vec<u8> {
    std::fs::read(format!("{ROOT}/{path}")).unwrap_or_else(|error| panic!("{path}: {error}"))
}

// ---------------------------------------------------------------------------
// The outcome of one case, reduced to what parity compares.

/// What a transcode did with a property. `Accepted` carries the color
/// signal, the discarded classes the row is about, and what the rows show.
#[derive(Clone, Debug, PartialEq)]
enum Outcome {
    Accepted {
        signal: ColorSignal,
        /// Only the classes the row names; other classes belong to other
        /// properties of the same file (its EXIF, for example).
        discarded: Vec<Discarded>,
        /// Some output alpha is below 255.
        alpha: bool,
    },
    Refused(ProcessingErrorKind, Option<RefusalReason>),
}

struct Capture(Mutex<(Vec<u8>, bool)>);
struct CaptureRows<'a>(&'a Mutex<(Vec<u8>, bool)>);

impl EncoderBackend for Capture {
    fn format(&self) -> OutputFormat {
        OutputFormat::WebP
    }
    fn name(&self) -> &'static str {
        "test-parity-capture"
    }
    fn begin(&self, request: &EncodeRequest) -> Result<Box<dyn RowEncoder + '_>, EncodeError> {
        self.0.lock().unwrap().1 = request.has_alpha;
        Ok(Box::new(CaptureRows(&self.0)))
    }
}

impl RowEncoder for CaptureRows<'_> {
    fn write_row(&mut self, _: u32, row: &[u8]) -> Result<(), EncodeError> {
        self.0.lock().unwrap().0.extend_from_slice(row);
        Ok(())
    }
    fn finish(self: Box<Self>) -> Result<EncodedImage, EncodeError> {
        Ok(EncodedImage::new(OutputFormat::WebP, Vec::new(), 0, false))
    }
}

/// The straight RGBA8 rows of one transcode at `size`, or its error.
fn rows(
    bytes: &[u8],
    format: ImageFormat,
    policy: InputProfilePolicy,
    size: (u32, u32),
) -> Result<
    (Vec<u8>, ferrisma::pipeline::ProcessingReport),
    (ProcessingErrorKind, Option<RefusalReason>),
> {
    let capture = Capture(Mutex::new((Vec::new(), false)));
    let options = TransformOptions {
        output_format: OutputFormat::WebP,
        output_width: size.0,
        output_height: size.1,
        profile_policy: policy,
        transform_threads: 1,
        ..TransformOptions::default()
    };
    let output = transcode_with(bytes, format, options, &capture)
        .map_err(|error| (error.kind(), error.refusal_reason()))?;
    Ok((capture.0.into_inner().unwrap().0, output.report))
}

/// The outcome of `bytes` under `policy`, keeping the discarded classes in
/// `classes`.
fn outcome(
    bytes: &[u8],
    format: ImageFormat,
    policy: InputProfilePolicy,
    classes: &[Discarded],
) -> Outcome {
    match rows(bytes, format, policy, (4, 4)) {
        Ok((rows, report)) => Outcome::Accepted {
            signal: report.color_signal,
            discarded: report
                .discarded
                .iter()
                .filter(|class| classes.contains(class))
                .collect(),
            alpha: rows.as_chunks::<4>().0.iter().any(|pixel| pixel[3] < 255),
        },
        Err((kind, reason)) => Outcome::Refused(kind, reason),
    }
}

/// One row of the table: every case must have `expected`.
#[track_caller]
fn assert_row(
    property: &str,
    cases: &[(&str, ImageFormat, &[u8])],
    policy: InputProfilePolicy,
    classes: &[Discarded],
    expected: &Outcome,
) {
    for (name, format, bytes) in cases {
        assert_eq!(
            &outcome(bytes, *format, policy, classes),
            expected,
            "{property}: {name} ({format:?}) under {policy:?}"
        );
    }
}

fn accepted(signal: ColorSignal, discarded: &[Discarded], alpha: bool) -> Outcome {
    Outcome::Accepted {
        signal,
        discarded: discarded.to_vec(),
        alpha,
    }
}

fn refused(kind: ProcessingErrorKind, reason: RefusalReason) -> Outcome {
    Outcome::Refused(kind, Some(reason))
}

// ---------------------------------------------------------------------------
// Color signaling.

/// A typed sRGB signal, JPEG XL's enum color encoding and AVIF's `nclx`
/// (owner decision 2026-11-15, #222), is read under every policy and named
/// `Cicp`. JPEG and WebP have no typed color signal (format reason: their
/// only color evidence is an ICC profile), so they have no case.
#[test]
fn typed_srgb_is_read_under_every_policy() {
    let jxl = read("tests/fixtures/orientation/jxl-1.jxl");
    let lossy_jxl = read("tests/fixtures/sample.jxl");
    #[allow(unused_mut)]
    let mut cases: Vec<(&str, ImageFormat, &[u8])> = vec![
        ("jxl-1.jxl (lossless)", ImageFormat::JpegXl, &jxl),
        ("sample.jxl (lossy XYB)", ImageFormat::JpegXl, &lossy_jxl),
    ];
    #[cfg(feature = "avif-read")]
    let avif = read(&format!("{AVIF_DATA}colors_sdr_srgb.avif"));
    #[cfg(feature = "avif-read")]
    cases.push(("colors_sdr_srgb.avif", ImageFormat::Avif, &avif));
    for policy in POLICIES {
        assert_row(
            "typed sRGB",
            &cases,
            policy,
            &[Discarded::IccProfile],
            &accepted(ColorSignal::Cicp, &[], false),
        );
    }
}

/// A typed signal outside sRGB, Display P3 and linear sRGB has no
/// conversion in the engine: refused with the same reason in both formats.
#[test]
fn typed_signal_outside_the_supported_families_is_refused_alike() {
    let jxl = read("tests/fixtures/jxl-color/lossless-dci-p3.jxl");
    #[allow(unused_mut)]
    let mut cases: Vec<(&str, ImageFormat, &[u8])> =
        vec![("lossless-dci-p3.jxl", ImageFormat::JpegXl, &jxl)];
    #[cfg(feature = "avif-read")]
    let avif = read(&format!("{AVIF_DATA}colors_text_wcg_sdr_rec2020.avif"));
    #[cfg(feature = "avif-read")]
    cases.push(("colors_text_wcg_sdr_rec2020.avif", ImageFormat::Avif, &avif));
    for (policy, reason) in [
        (STRICT, RefusalReason::UnsupportedColorEncoding),
        (ASSUME, RefusalReason::UnsupportedColorEncoding),
        (RENDERED, RefusalReason::RenderedIccMissing),
    ] {
        assert_row(
            "typed signal outside the families",
            &cases,
            policy,
            &[],
            &refused(ProcessingErrorKind::UnsupportedColor, reason),
        );
    }
}

/// An embedded ICC profile needs `UseRenderedIcc` in every format, which
/// converts it and reports the profile as not written.
#[test]
fn embedded_icc_needs_the_rendered_icc_policy_in_every_format() {
    let jpeg = read("codecs/libavif-rs/upstream/libavif/tests/data/paris_exif_xmp_icc.jpg");
    let jxl = read("tests/fixtures/parity/rgb-p3-icc.jxl");
    // The engine's own WebP output carries an sRGB `ICCP` chunk.
    let webp = transcode(
        &jxl,
        ImageFormat::JpegXl,
        TransformOptions {
            output_format: OutputFormat::WebP,
            output_width: 8,
            output_height: 4,
            profile_policy: RENDERED,
            ..TransformOptions::default()
        },
    )
    .unwrap()
    .bytes;
    #[allow(unused_mut)]
    let mut cases: Vec<(&str, ImageFormat, &[u8])> = vec![
        ("paris_exif_xmp_icc.jpg", ImageFormat::Jpeg, &jpeg),
        ("engine WebP output (ICCP)", ImageFormat::WebP, &webp),
        ("rgb-p3-icc.jxl", ImageFormat::JpegXl, &jxl),
    ];
    #[cfg(feature = "avif-read")]
    let avif = read(&format!("{AVIF_DATA}paris_icc_exif_xmp.avif"));
    #[cfg(feature = "avif-read")]
    cases.push((
        "paris_icc_exif_xmp.avif (nclx 2/2/6)",
        ImageFormat::Avif,
        &avif,
    ));
    for policy in [STRICT, ASSUME] {
        assert_row(
            "embedded ICC",
            &cases,
            policy,
            &[],
            &refused(
                ProcessingErrorKind::UnsupportedColor,
                RefusalReason::RenderedIccRequiresOptIn,
            ),
        );
    }
    assert_row(
        "embedded ICC",
        &cases,
        RENDERED,
        &[Discarded::IccProfile],
        &accepted(ColorSignal::RenderedIcc, &[Discarded::IccProfile], false),
    );
}

/// The engine reads its own JPEG XL output, lossy (the default) and
/// lossless, with default settings: the strict policy (#222). Failed before
/// the owner decision of 2026-11-15 with `RenderedIccRequiresOptIn`.
#[test]
fn the_engine_reads_its_own_jpeg_xl_output_with_default_settings() {
    let source = read("tests/fixtures/orientation/jxl-1.jxl");
    for jpeg_xl in [
        JpegXlOptions::default(),
        JpegXlOptions::Lossless { effort: 2 },
    ] {
        let written = transcode(
            &source,
            ImageFormat::JpegXl,
            TransformOptions {
                output_format: OutputFormat::JpegXl,
                jpeg_xl,
                output_width: 24,
                output_height: 16,
                ..TransformOptions::default()
            },
        )
        .unwrap_or_else(|error| panic!("{jpeg_xl:?}: {error}"));
        let again = transcode(
            &written.bytes,
            ImageFormat::JpegXl,
            TransformOptions {
                output_width: 24,
                output_height: 16,
                ..TransformOptions::default()
            },
        )
        .unwrap_or_else(|error| panic!("{jpeg_xl:?} read back: {error}"));
        assert_eq!(again.report.color_signal, ColorSignal::Cicp);
        assert!(again.report.discarded.is_empty(), "{jpeg_xl:?}");
    }
}

// ---------------------------------------------------------------------------
// Alpha.

/// Straight alpha is carried in every format that has alpha; JPEG has no
/// alpha (format reason).
#[test]
fn straight_alpha_is_kept_in_every_format() {
    let jxl = read(&format!("{JXL_DATA}effects_rgb_alpha8.jxl"));
    // The engine's own WebP output with alpha (the WebP reader has no
    // association other than straight; VP8L/ALPH are unassociated).
    let webp = transcode(
        &jxl,
        ImageFormat::JpegXl,
        TransformOptions {
            output_format: OutputFormat::WebP,
            output_width: 6,
            output_height: 4,
            ..TransformOptions::default()
        },
    )
    .unwrap()
    .bytes;
    #[allow(unused_mut)]
    let mut cases: Vec<(&str, ImageFormat, &[u8])> = vec![
        ("effects_rgb_alpha8.jxl", ImageFormat::JpegXl, &jxl),
        ("engine WebP output with alpha", ImageFormat::WebP, &webp),
    ];
    #[cfg(feature = "avif-read")]
    let avif = read(&format!("{AVIF_DATA}abc_color_irot_alpha_irot.avif"));
    #[cfg(feature = "avif-read")]
    cases.push(("abc_color_irot_alpha_irot.avif", ImageFormat::Avif, &avif));
    // The WebP output carries the engine's sRGB ICCP, so its signal is the
    // rendered ICC and it is read under that policy; the JPEG XL and AVIF
    // files are typed sRGB. The alpha outcome is the row's property.
    for (name, format, bytes) in &cases {
        let policy = if *format == ImageFormat::WebP {
            RENDERED
        } else {
            STRICT
        };
        let Outcome::Accepted {
            alpha, discarded, ..
        } = outcome(bytes, *format, policy, &[Discarded::Unpremultiplied])
        else {
            panic!("{name}: refused");
        };
        assert!(alpha, "{name}: alpha kept");
        assert_eq!(discarded, [], "{name}: nothing converted");
    }
}

// ---------------------------------------------------------------------------
// Geometry.

/// Orientation is applied before crop in every format that carries it
/// (EXIF in JPEG and WebP, the codestream in JPEG XL, `irot`/`imir` in
/// AVIF): orientation 6 makes the 24 x 16 stored image 16 x 24 upright.
#[test]
fn orientation_is_applied_before_crop_in_every_format() {
    let full = |width, height| TransformOptions {
        output_format: OutputFormat::WebP,
        output_width: 4,
        output_height: 4,
        crop: Some(Crop {
            x: 0,
            y: 0,
            width,
            height,
        }),
        profile_policy: ASSUME,
        ..TransformOptions::default()
    };
    let o = "tests/fixtures/orientation/";
    #[allow(unused_mut)]
    let mut cases = vec![
        (format!("{o}jpeg-6.jpg"), ImageFormat::Jpeg, (16, 24)),
        (format!("{o}jxl-6.jxl"), ImageFormat::JpegXl, (16, 24)),
        // The WebP fixture is a 96 x 72 photograph with EXIF orientation 5.
        (
            "tests/fixtures/decoder-work/paris-o5-lossy.webp".to_owned(),
            ImageFormat::WebP,
            (72, 96),
        ),
    ];
    #[cfg(feature = "avif-read")]
    cases.push((format!("{o}avif-6.avif"), ImageFormat::Avif, (16, 24)));
    for (path, format, (width, height)) in cases {
        let bytes = read(&path);
        transcode(&bytes, format, full(width, height)).unwrap_or_else(|e| panic!("{path}: {e}"));
        let error = transcode(&bytes, format, full(width + 1, height)).unwrap_err();
        assert_eq!(error.kind(), ProcessingErrorKind::InvalidOptions, "{path}");
    }
}

/// Animation is refused without a first-frame fallback in every format
/// that can animate; JPEG cannot (format reason).
#[test]
fn animation_is_refused_alike() {
    let webp = read("tests/fixtures/decoder-work/animated.webp");
    let jxl = read(&format!(
        "{JXL_DATA}decoder-geometry/sequence-reference-overwrite-animation-1.jxl"
    ));
    #[allow(unused_mut)]
    let mut cases: Vec<(&str, ImageFormat, &[u8])> = vec![
        ("animated.webp", ImageFormat::WebP, &webp),
        (
            "sequence-reference-overwrite-animation-1.jxl",
            ImageFormat::JpegXl,
            &jxl,
        ),
    ];
    #[cfg(feature = "avif-read")]
    let avif = read(&format!("{AVIF_DATA}colors-animated-8bpc.avif"));
    #[cfg(feature = "avif-read")]
    cases.push(("colors-animated-8bpc.avif", ImageFormat::Avif, &avif));
    for policy in POLICIES {
        assert_row(
            "animation",
            &cases,
            policy,
            &[],
            &refused(
                ProcessingErrorKind::UnsupportedInput,
                RefusalReason::Animation,
            ),
        );
    }
}
