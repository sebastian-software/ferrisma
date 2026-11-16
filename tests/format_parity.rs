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

/// An ICC profile beside a typed signal: only AVIF can carry both (`colr
/// prof` and `colr nclx`); a JPEG XL codestream signals either an enum or a
/// profile, and JPEG and WebP have no typed signal (format reasons). AVIF
/// follows MIAF since 2026-11-15 (owner decision): the profile governs, so
/// every `nclx` gets the embedded-ICC outcome above, whatever its primaries
/// and transfer. Before, a concrete `nclx` other than sRGB beside the
/// profile was refused with `CicpBesideIcc`.
#[cfg(feature = "avif-read")]
#[test]
fn an_icc_profile_beside_nclx_governs_like_any_embedded_profile() {
    let files = [
        read("tests/fixtures/avif-color/icc-nclx-1-13-6.avif"),
        read("tests/fixtures/avif-color/icc-nclx-1-1-6.avif"),
        read(&format!("{AVIF_DATA}paris_icc_exif_xmp.avif")),
        read("tests/fixtures/parity/icc-nclx-12-13-6.avif"),
    ];
    let names = [
        "nclx 1/13/6",
        "nclx 1/1/6",
        "nclx 2/2/6 (avifenc)",
        "nclx 12/13/6",
    ];
    let cases: Vec<(&str, ImageFormat, &[u8])> = names
        .iter()
        .zip(&files)
        .map(|(name, bytes)| (*name, ImageFormat::Avif, bytes.as_slice()))
        .collect();
    for policy in [STRICT, ASSUME] {
        assert_row(
            "ICC beside nclx",
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
        "ICC beside nclx",
        &cases,
        RENDERED,
        &[Discarded::IccProfile],
        &accepted(ColorSignal::RenderedIcc, &[Discarded::IccProfile], false),
    );
}

/// A `cicp` tag inside a profile is part of the profile (ADR-0016): when
/// it contradicts the profile's own transform the file is refused with
/// `CicpBesideIcc` in every format that embeds the profile; the fixture is
/// JPEG (`rgb-icc-cicp-p3.jpg`), the code path is the one rendered-ICC
/// stage for every format.
#[test]
fn a_contradicting_cicp_tag_inside_the_profile_is_refused() {
    let jpeg = read("tests/fixtures/parity/rgb-icc-cicp-p3.jpg");
    assert_row(
        "cicp tag inside the profile",
        &[("rgb-icc-cicp-p3.jpg", ImageFormat::Jpeg, &jpeg)],
        RENDERED,
        &[],
        &refused(
            ProcessingErrorKind::UnsupportedColor,
            RefusalReason::CicpBesideIcc,
        ),
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
        JpegXlOptions::Lossless {
            effort: 2,
            depth: None,
        },
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
            // The default policy (`Strict`) and options; WebP output so
            // that builds without `avif-write` (x86-64 macOS) run it too.
            TransformOptions {
                output_format: OutputFormat::WebP,
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
// Gray (owner decision 2026-11-15): accepted in every format that carries it
// and expanded to RGB (gray with alpha to RGBA) by one copy, R = G = B,
// after color handling (`src/gray.rs`). WebP has no gray layout (format
// reason: VP8 and VP8L code RGB(A) only), so it has no case.

#[cfg(feature = "avif-read")]
const AVIF_ANDROID: &str =
    "codecs/libavif-rs/upstream/libavif/android_jni/avifandroidjni/src/androidTest/assets/avif/";

/// The rows of a gray input at its own size, checked to be R = G = B.
#[track_caller]
fn assert_expanded_gray(
    name: &str,
    bytes: &[u8],
    format: ImageFormat,
    policy: InputProfilePolicy,
    size: (u32, u32),
) -> (Vec<u8>, ferrisma::pipeline::ProcessingReport) {
    let (rows, report) = rows(bytes, format, policy, size)
        .unwrap_or_else(|error| panic!("{name} under {policy:?}: {error:?}"));
    assert_eq!(rows.len(), size.0 as usize * size.1 as usize * 4, "{name}");
    for pixel in rows.as_chunks::<4>().0 {
        assert!(
            pixel[0] == pixel[1] && pixel[1] == pixel[2],
            "{name}: {pixel:?}"
        );
    }
    (rows, report)
}

/// Gray is accepted and expanded in JPEG, JPEG XL and AVIF. Each file's
/// color signal is its own (untagged JPEG and the Android monochrome AVIF
/// files need `AssumeSrgbIfMissing`; the JPEG XL file is enum gray sRGB, a
/// typed signal), which the color-signaling rows cover; this row asserts
/// the layout outcome: accepted, R = G = B, opaque. (Until 2026-11-15 only
/// JPEG was accepted; JPEG XL and AVIF were refused with
/// `UnsupportedLayout`.)
#[test]
fn gray_is_expanded_to_rgb_in_every_format_that_carries_it() {
    let jpeg = read(&format!("{JXL_DATA}ycbcr-jpeg-gray.jpg"));
    let jxl = read(&format!("{JXL_DATA}ycbcr-jpeg-gray.jxl"));
    #[allow(unused_mut)]
    let mut cases: Vec<(String, ImageFormat, Vec<u8>, ColorSignal)> = vec![
        (
            "ycbcr-jpeg-gray.jpg".into(),
            ImageFormat::Jpeg,
            jpeg,
            ColorSignal::AssumedSrgb,
        ),
        (
            "ycbcr-jpeg-gray.jxl".into(),
            ImageFormat::JpegXl,
            jxl,
            ColorSignal::Cicp,
        ),
    ];
    #[cfg(feature = "avif-read")]
    for depth in ["profile0.8bpc", "profile0.10bpc", "profile2.12bpc"] {
        let name = format!("fox.{depth}.yuv420.monochrome.avif");
        let bytes = read(&format!("{AVIF_ANDROID}{name}"));
        cases.push((name, ImageFormat::Avif, bytes, ColorSignal::AssumedSrgb));
    }
    for (name, format, bytes, signal) in &cases {
        let (rows, report) = assert_expanded_gray(name, bytes, *format, ASSUME, (8, 6));
        assert_eq!(report.color_signal, *signal, "{name}");
        assert!(
            rows.as_chunks::<4>().0.iter().all(|pixel| pixel[3] == 255),
            "{name}"
        );
    }
}

/// Gray with straight alpha is accepted and expanded to RGBA with the alpha
/// kept. JPEG has no alpha (format reason). AVIF can carry a monochrome
/// image with an alpha item, but the repository has no such file and the
/// workspace AVIF writer cannot write one, so AVIF gray with alpha is
/// **untested** (no fixture) and not claimed; the code path is the gray
/// path above plus libavif's `GRAYA` output.
#[test]
fn gray_with_alpha_is_expanded_to_rgba() {
    for name in [
        "modular-gray-alpha12.jxl",
        "effects_gray_alpha16.jxl",
        "vardct-gray-alpha16.jxl",
    ] {
        let bytes = read(&format!("{JXL_DATA}{name}"));
        for policy in POLICIES {
            let (rows, report) =
                assert_expanded_gray(name, &bytes, ImageFormat::JpegXl, policy, (8, 6));
            assert_eq!(report.color_signal, ColorSignal::Cicp, "{name}");
            assert!(
                rows.as_chunks::<4>().0.iter().any(|pixel| pixel[3] < 255),
                "{name}: alpha"
            );
            assert!(
                !report
                    .discarded
                    .iter()
                    .any(|class| class == Discarded::Unpremultiplied)
            );
        }
    }
}

/// A gray ICC profile (`GRAY`, one gamma-2.2 `kTRC` curve) is converted by
/// its own tone curve under `UseRenderedIcc` and needs that policy like any
/// profile. The result is checked against an independent computation,
/// linear = (v / 255)^2.19921875 for every gray sample v, R = G = B, within
/// the 8-bit encoding of a 1e-5 linear bound. For the JPEG XL file
/// (lossless, written by cjxl) the samples are known from
/// `tools/make.py`; the JPEG file is libjxl's gray JPEG with the profile
/// inserted, whose samples are those of the same file without the profile,
/// read as assumed sRGB at its own size (an identity that returns the
/// decoded codes). AVIF can carry a gray profile beside a monochrome image,
/// but no fixture exists: **untested**.
#[test]
fn a_gray_icc_profile_is_converted_by_its_tone_curve() {
    let jpeg = read("tests/fixtures/parity/gray-gray-icc.jpg");
    let jxl = read("tests/fixtures/parity/gray-gray-icc.jxl");
    let cases: [(&str, ImageFormat, &[u8]); 2] = [
        ("gray-gray-icc.jxl", ImageFormat::JpegXl, &jxl),
        ("gray-gray-icc.jpg", ImageFormat::Jpeg, &jpeg),
    ];
    for policy in [STRICT, ASSUME] {
        assert_row(
            "gray ICC",
            &cases,
            policy,
            &[],
            &refused(
                ProcessingErrorKind::UnsupportedColor,
                RefusalReason::RenderedIccRequiresOptIn,
            ),
        );
    }
    let encode = |linear: f64| {
        let encoded = if linear <= 0.003_130_8 {
            12.92 * linear
        } else {
            1.055 * linear.powf(1.0 / 2.4) - 0.055
        };
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let code = (encoded * 255.0).round().clamp(0.0, 255.0) as u8;
        code
    };
    let known: Vec<u8> = (0..8)
        .flat_map(|y| (0..16).map(move |x| u8::try_from((16 * x + 8 * y) % 256).unwrap()))
        .collect();
    let untagged = read(&format!("{JXL_DATA}ycbcr-jpeg-gray.jpg"));
    let header = ferrisma::pipeline::inspect(
        &untagged,
        ImageFormat::Jpeg,
        ferrisma::pipeline::InspectionLimits::default(),
    )
    .unwrap();
    let jpeg_size = (header.width, header.height);
    let (stored, _) = assert_expanded_gray(
        "ycbcr-jpeg-gray.jpg",
        &untagged,
        ImageFormat::Jpeg,
        ASSUME,
        jpeg_size,
    );
    let stored: Vec<u8> = stored
        .as_chunks::<4>()
        .0
        .iter()
        .map(|pixel| pixel[0])
        .collect();
    for ((name, format, bytes), (samples, size)) in cases
        .into_iter()
        .zip([(known, (16, 8)), (stored, jpeg_size)])
    {
        let (rows, report) = assert_expanded_gray(name, bytes, format, RENDERED, size);
        assert_eq!(report.color_signal, ColorSignal::RenderedIcc, "{name}");
        assert!(
            report
                .discarded
                .iter()
                .any(|class| class == Discarded::IccProfile)
        );
        assert_eq!(rows.len(), samples.len() * 4, "{name}");
        for (index, (pixel, &value)) in rows.as_chunks::<4>().0.iter().zip(&samples).enumerate() {
            let linear = (f64::from(value) / 255.0).powf(2.199_218_75);
            let (lo, hi) = (encode(linear - 1e-5), encode(linear + 1e-5));
            assert!(
                (lo..=hi).contains(&pixel[0]) && pixel[3] == 255,
                "{name} pixel {index} (gray {value}): {} not in {lo}..={hi}",
                pixel[0]
            );
        }
    }
}

/// A profile of the other color space than the decoded channels (an RGB
/// profile beside gray samples, a gray profile beside RGB samples) is
/// refused under `UseRenderedIcc`, never reinterpreted (new reason
/// `IccColorSpaceMismatch`, 2026-11-15). Only JPEG fixtures exist: JPEG XL
/// requires a gray profile for gray channels in the codestream itself.
#[test]
fn an_icc_profile_of_the_other_color_space_is_refused() {
    let gray_rgb = read("tests/fixtures/parity/gray-rgb-icc.jpg");
    let rgb_gray = read("tests/fixtures/parity/rgb-gray-icc.jpg");
    assert_row(
        "ICC color space mismatch",
        &[
            ("gray-rgb-icc.jpg", ImageFormat::Jpeg, &gray_rgb),
            ("rgb-gray-icc.jpg", ImageFormat::Jpeg, &rgb_gray),
        ],
        RENDERED,
        &[],
        &refused(
            ProcessingErrorKind::UnsupportedColor,
            RefusalReason::IccColorSpaceMismatch,
        ),
    );
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

/// Premultiplied (associated) alpha is accepted and converted to straight
/// alpha in both formats that carry it, and reported as
/// `Discarded::Unpremultiplied` (owner decision 2026-11-15; until then
/// JPEG XL refused it with `AlphaAssociation`). Each format uses its
/// original's own step, which ADR-0043 D2 allows only with a format reason,
/// written into the supported matrix: libavif divides 8/10/12-bit integers,
/// `min(round(c * max / a), max)` with color 0 under alpha 0
/// (`avifRGBImageUnpremultiplyAlpha`); libjxl multiplies float samples by
/// `1 / max(a, 2^-26)` without rounding or clamping (`UnpremultiplyAlpha`),
/// so a color above its alpha in lossy data stays above 1 into the
/// working space. The JPEG XL arithmetic is checked bit for bit in
/// `jxl_alpha_tests::associated_alpha_is_unpremultiplied_by_libjxl`.
/// JPEG and WebP have no premultiplied alpha (format reason).
#[test]
fn premultiplied_alpha_is_converted_and_reported_alike() {
    let jxl = read(&format!("{JXL_DATA}extra-channel-alpha-1-up-1-shift-0.jxl"));
    #[allow(unused_mut)]
    let mut cases: Vec<(&str, ImageFormat, &[u8])> = vec![(
        "extra-channel-alpha-1-up-1-shift-0.jxl",
        ImageFormat::JpegXl,
        &jxl,
    )];
    #[cfg(feature = "avif-read")]
    let avif = read("tests/fixtures/parity/premultiplied-alpha.avif");
    #[cfg(feature = "avif-read")]
    cases.push(("premultiplied-alpha.avif", ImageFormat::Avif, &avif));
    for (name, format, bytes) in &cases {
        for policy in [ASSUME, RENDERED] {
            let Outcome::Accepted {
                alpha, discarded, ..
            } = outcome(bytes, *format, policy, &[Discarded::Unpremultiplied])
            else {
                panic!("{name} under {policy:?}: refused");
            };
            assert!(alpha, "{name}: alpha kept");
            assert_eq!(
                discarded,
                [Discarded::Unpremultiplied],
                "{name} under {policy:?}"
            );
        }
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

// ---------------------------------------------------------------------------
// Bit depth (ADR-0043 decided application, owner decision of 2026-11-15;
// ADR-0021 amendment): the output depth follows the input's by one rule in
// every format that can represent it, the nearest admitted depth below where
// it cannot, and `PrecisionReduced` exactly when the output holds fewer bits
// than the input. WebP is 8-bit by its format (VP8 and VP8L); AVIF ends at
// 12 bits (AV1's largest); JPEG XL holds 8 to 16 integer bits. These are the
// format reasons of the matrix row "Bit depth".

/// (output depth, `PrecisionReduced`) of one transcode.
fn depth_outcome(
    bytes: &[u8],
    input: ImageFormat,
    output: OutputFormat,
    depth: Option<u8>,
) -> (u8, u8, bool) {
    let mut options = TransformOptions {
        output_format: output,
        output_width: 16,
        output_height: 9,
        profile_policy: ASSUME,
        transform_threads: 1,
        ..TransformOptions::default()
    };
    match output {
        OutputFormat::JpegXl => {
            options.jpeg_xl = JpegXlOptions::Lossless { effort: 2, depth };
        }
        OutputFormat::Avif => options.avif.depth = depth,
        _ => assert!(depth.is_none(), "WebP has no depth setting"),
    }
    let result = transcode(bytes, input, options)
        .unwrap_or_else(|error| panic!("{input:?} to {output:?} at {depth:?}: {error}"));
    let reduced = result
        .report
        .discarded
        .iter()
        .any(|class| class == Discarded::PrecisionReduced);
    (
        result.report.input_depth,
        result.report.output_depth,
        reduced,
    )
}

#[test]
fn depth_follows_the_input_in_every_output() {
    #[cfg_attr(not(feature = "avif-read"), allow(unused_mut))]
    let mut inputs = vec![
        (read("tests/fixtures/sample.jpg"), ImageFormat::Jpeg, 8),
        (
            read("tests/fixtures/webp/reference-lossy.webp"),
            ImageFormat::WebP,
            8,
        ),
        (
            read("tests/fixtures/jxl-lossless/rgb-37x19.jxl"),
            ImageFormat::JpegXl,
            8,
        ),
        (
            read("tests/fixtures/jxl-deep/d10-37x19-a0-lossless.jxl"),
            ImageFormat::JpegXl,
            10,
        ),
        (
            read("tests/fixtures/jxl-deep/d12-37x19-a1-lossless.jxl"),
            ImageFormat::JpegXl,
            12,
        ),
        (
            read("tests/fixtures/jxl-deep/d16-37x19-a0-lossless.jxl"),
            ImageFormat::JpegXl,
            16,
        ),
    ];
    #[cfg(feature = "avif-read")]
    {
        const ANDROID: &str = "codecs/libavif-rs/upstream/libavif/android_jni/avifandroidjni/src/androidTest/assets/avif/";
        for (file, depth) in [
            ("fox.profile0.8bpc.yuv420.avif", 8),
            ("fox.profile0.10bpc.yuv420.avif", 10),
            ("fox.profile2.12bpc.yuv444.avif", 12),
        ] {
            inputs.push((read(&format!("{ANDROID}{file}")), ImageFormat::Avif, depth));
        }
    }
    let mut outputs = vec![
        (OutputFormat::WebP, &[8_u8][..]),
        (
            OutputFormat::JpegXl,
            &[8, 9, 10, 11, 12, 13, 14, 15, 16][..],
        ),
    ];
    if cfg!(feature = "avif-write") {
        outputs.push((OutputFormat::Avif, &[8, 10, 12][..]));
    }
    for (bytes, input, depth) in &inputs {
        for &(output, admitted) in &outputs {
            // The one rule: the largest admitted depth at most the input's.
            let expected = admitted
                .iter()
                .copied()
                .filter(|d| d <= depth)
                .max()
                .unwrap();
            assert_eq!(
                depth_outcome(bytes, *input, output, None),
                (*depth, expected, expected < *depth),
                "{input:?} {depth}-bit to {output:?}"
            );
            // An explicit depth overrides the rule, in each format that
            // has the setting; below the input it is reported, above it
            // is not (the content is quantized at that depth).
            if output != OutputFormat::WebP {
                for &explicit in admitted {
                    assert_eq!(
                        depth_outcome(bytes, *input, output, Some(explicit)),
                        (*depth, explicit, explicit < *depth),
                        "{input:?} {depth}-bit to {output:?} at {explicit}"
                    );
                }
            }
        }
    }
}
