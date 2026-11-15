//! Evidence for `docs/supported-matrix.md` (issue #67): every case below is
//! one claim of the matrix, checked through the public processing API
//! (`transcode`, `transcode_resized`, `inspect`) with a real file from the
//! repository. A supported case decodes and encodes completely; a rejected
//! case asserts the public kind, the message's reason *and* the typed
//! `refusal_reason()` (issue #212; `None` for malformed data, limits and
//! invalid options), because one kind (for example `UnsupportedInput`) is
//! shared by unrelated causes.
//! Header inspection alone is never taken as proof here.
//!
//! Run it with every adapter (`cargo test -p ferrisma --features cli --test
//! supported_matrix`). Without the AVIF features, which build only on macOS
//! ARM64 and GNU Linux x86-64, the JPEG, WebP and JPEG XL cases still run
//! (for example on x86-64 macOS under Rosetta 2) with
//! `--features jpeg-read,webp-read,webp-write,jpeg-xl-read,jpeg-xl-write,icc`.
#![cfg(all(
    feature = "jpeg-read",
    feature = "webp-read",
    feature = "webp-write",
    feature = "jpeg-xl-read",
    feature = "jpeg-xl-write",
    feature = "icc"
))]

#[cfg(all(feature = "avif-read", feature = "avif-write"))]
use ferrisma::pipeline::ChromaSubsampling;
use ferrisma::pipeline::{
    Crop, ImageFormat, InputProfilePolicy, InspectionLimits, JpegXlOptions, Output, OutputFormat,
    ProcessingError, ProcessingErrorKind, RefusalReason, SourceAccess, TransformOptions,
    WebpOptions, inspect, transcode,
};

// ---------------------------------------------------------------------------
// Fixtures (provenance in each directory's README).

const SAMPLE_JPEG: &[u8] = include_bytes!("fixtures/sample.jpg");
const CMYK_JPEG: &[u8] = include_bytes!("fixtures/cmyk.jpg");
const ICC_JPEG: &[u8] =
    include_bytes!("../codecs/libavif-rs/upstream/libavif/tests/data/paris_exif_xmp_icc.jpg");
const LOSSY_WEBP: &[u8] = include_bytes!("fixtures/webp/reference-lossy.webp");
const ORIENTED_LOSSY_WEBP: &[u8] = include_bytes!("fixtures/decoder-work/paris-o5-lossy.webp");
const ORIENTED_LOSSLESS_WEBP: &[u8] =
    include_bytes!("fixtures/decoder-work/paris-o5-lossless.webp");
const ANIMATED_WEBP: &[u8] = include_bytes!("fixtures/decoder-work/animated.webp");
const SAMPLE_JXL: &[u8] = include_bytes!("fixtures/sample.jxl");
const JXL_DATA: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/codecs/libjxl-rs/tests/data/");
#[cfg(feature = "avif-read")]
const AVIF_DATA: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/codecs/libavif-rs/upstream/libavif/tests/data/"
);
#[cfg(feature = "avif-read")]
const AVIF_ANDROID: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/codecs/libavif-rs/upstream/libavif/android_jni/avifandroidjni/src/androidTest/assets/"
);
const ORIENTATION: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/orientation/");

fn read(path: &str) -> Vec<u8> {
    std::fs::read(path).unwrap_or_else(|error| panic!("{path}: {error}"))
}

#[cfg(feature = "avif-read")]
fn avif(name: &str) -> Vec<u8> {
    read(&format!("{AVIF_DATA}{name}"))
}

fn jxl(name: &str) -> Vec<u8> {
    read(&format!("{JXL_DATA}{name}"))
}

// ---------------------------------------------------------------------------
// The engine's reasons, as `ProcessingError`'s message prints them.

const ANIMATION: &str = "animated input is not supported; only still images are decoded";
const LAYOUT: &str = "decoder pixel or color layout is not supported";
#[cfg(feature = "avif-read")]
const TRANSFORM: &str = "orientation, mirroring or clean-aperture transforms are not supported yet";
#[cfg(feature = "avif-read")]
const ESSENTIAL: &str = "an essential container property is not supported: 'xyzw'";
const COLOR: &str =
    "unsupported or ambiguous rendered color profile; a color-management transform is required";
const MALFORMED: &str = "decoder rejected input or could not render it";
const LINEAR_SRGB: &str = "linear-sRGB resize requires unambiguous full-range RGB sRGB CICP; ICC/unknown/HDR need a managed transform";
const INPUT_BYTES: &str = "compressed image exceeds max_input_bytes";
const ZERO_LIMIT: &str = "input byte/pixel limits must be nonzero";
const OUTPUT_BYTES: &str = "encoded output exceeds max_output_bytes";
const MISMATCHED: &str = "encoder options do not match the selected output format";
const ENCODER_OPTIONS: &str = "encoder quality/speed or output dimensions are invalid";

// ---------------------------------------------------------------------------
// Helpers.

fn options(output_format: OutputFormat, policy: InputProfilePolicy) -> TransformOptions {
    TransformOptions {
        output_format,
        output_width: 6,
        output_height: 4,
        profile_policy: policy,
        transform_threads: 1,
        ..TransformOptions::default()
    }
}

const ASSUME: InputProfilePolicy = InputProfilePolicy::AssumeSrgbIfMissing;
const RENDERED: InputProfilePolicy = InputProfilePolicy::UseRenderedIcc;
const STRICT: InputProfilePolicy = InputProfilePolicy::Strict;

fn to_webp(bytes: &[u8], format: ImageFormat, policy: InputProfilePolicy) -> Output {
    transcode(bytes, format, options(OutputFormat::WebP, policy))
        .unwrap_or_else(|error| panic!("{format:?} under {policy:?}: {error}"))
}

/// A rejection that is not a refusal class of `RefusalReason` (malformed
/// data, limits, invalid options): its kind and message, and no reason.
#[track_caller]
fn rejected(
    result: Result<Output, ProcessingError>,
    kind: ProcessingErrorKind,
    reason: &str,
) -> ProcessingError {
    refused(result, kind, reason, None)
}

/// A rejection with its kind, its message prefix and its typed
/// `refusal_reason()` (issue #212); the message stays asserted because the
/// matrix documents it.
#[track_caller]
fn refused(
    result: Result<Output, ProcessingError>,
    kind: ProcessingErrorKind,
    reason: &str,
    refusal: Option<RefusalReason>,
) -> ProcessingError {
    let error = result.expect_err("the configuration must be rejected");
    assert_eq!(error.kind(), kind, "{error}");
    assert!(
        error.to_string().starts_with(reason),
        "expected reason {reason:?}, got {error:?}"
    );
    assert_eq!(error.refusal_reason(), refusal, "{error}");
    error
}

/// The upright display size the pipeline works on: the full display
/// rectangle is a valid crop, one more column or row is not.
#[track_caller]
fn assert_display_size(
    bytes: &[u8],
    format: ImageFormat,
    policy: InputProfilePolicy,
    size: (u32, u32),
) {
    let crop = |width, height| TransformOptions {
        crop: Some(Crop {
            x: 0,
            y: 0,
            width,
            height,
        }),
        ..options(OutputFormat::WebP, policy)
    };
    transcode(bytes, format, crop(size.0, size.1)).unwrap();
    for (width, height) in [(size.0 + 1, size.1), (size.0, size.1 + 1)] {
        rejected(
            transcode(bytes, format, crop(width, height)),
            ProcessingErrorKind::InvalidOptions,
            "requested region is outside image bounds",
        );
    }
}

/// WebP output metadata through the public header inspection (the output
/// itself is decoded by the cases that use this).
fn webp_has_alpha(output: &Output) -> bool {
    inspect(
        &output.bytes,
        ImageFormat::WebP,
        InspectionLimits::default(),
    )
    .unwrap()
    .has_alpha
}

/// RIFF chunk tags of a WebP file, in order.
fn webp_chunks(bytes: &[u8]) -> Vec<[u8; 4]> {
    assert_eq!(&bytes[..4], b"RIFF");
    assert_eq!(&bytes[8..12], b"WEBP");
    let mut chunks = Vec::new();
    let mut at = 12;
    while at + 8 <= bytes.len() {
        let tag: [u8; 4] = bytes[at..at + 4].try_into().unwrap();
        let size = u32::from_le_bytes(bytes[at + 4..at + 8].try_into().unwrap()) as usize;
        chunks.push(tag);
        at += 8 + size.next_multiple_of(2);
    }
    assert_eq!(at, bytes.len(), "trailing bytes after the last chunk");
    chunks
}

#[cfg(any(feature = "avif-read", feature = "avif-write"))]
fn find(bytes: &[u8], needle: &[u8]) -> Option<usize> {
    bytes
        .windows(needle.len())
        .position(|window| window == needle)
}

#[cfg(all(feature = "avif-read", feature = "avif-write"))]
/// The AV1 configuration of the (first) `av1C` property: bit depth and
/// chroma subsampling (x, y), and the monochrome flag.
fn av1_config(bytes: &[u8]) -> (u8, (bool, bool), bool) {
    let at = find(bytes, b"av1C").expect("av1C") + 4;
    let flags = bytes[at + 2];
    let high = flags & 0x40 != 0;
    let twelve = flags & 0x20 != 0;
    let depth = match (high, twelve) {
        (false, _) => 8,
        (true, false) => 10,
        (true, true) => 12,
    };
    (
        depth,
        (flags & 0x08 != 0, flags & 0x04 != 0),
        flags & 0x10 != 0,
    )
}

#[cfg(all(feature = "avif-read", feature = "avif-write"))]
/// The container's `colr nclx` code points and full-range flag, if any.
fn nclx(bytes: &[u8]) -> Option<(u16, u16, u16, bool)> {
    let at = find(bytes, b"colrnclx")? + 8;
    let u16_at = |i: usize| u16::from_be_bytes([bytes[at + i], bytes[at + i + 1]]);
    Some((u16_at(0), u16_at(2), u16_at(4), bytes[at + 6] & 0x80 != 0))
}

#[cfg(all(feature = "avif-read", feature = "avif-write"))]
const AVIF_ALPHA_URN: &[u8] = b"urn:mpeg:mpegB:cicp:systems:auxiliary:alpha";

#[cfg(all(feature = "avif-read", feature = "avif-write"))]
/// Read an encoded AVIF back through the engine's own AVIF reader under the
/// strict policy: only signalled sRGB passes it.
fn read_back_avif_strict(output: &Output) -> Output {
    transcode(
        &output.bytes,
        ImageFormat::Avif,
        options(OutputFormat::WebP, STRICT),
    )
    .expect("the AVIF output reads back as signalled sRGB")
}

// ===========================================================================
// Inputs: JPEG.

#[test]
fn jpeg_rgb_is_accepted_when_srgb_is_assumed() {
    let output = to_webp(SAMPLE_JPEG, ImageFormat::Jpeg, ASSUME);
    assert_eq!((output.width, output.height), (6, 4));
    assert_eq!(output.report.input_access, SourceAccess::CodecRegionRequest);
}

#[test]
fn jpeg_untagged_is_rejected_under_the_strict_policy() {
    refused(
        transcode(
            SAMPLE_JPEG,
            ImageFormat::Jpeg,
            options(OutputFormat::WebP, STRICT),
        ),
        ProcessingErrorKind::UnsupportedColor,
        COLOR,
        Some(RefusalReason::ColorSignalMissing),
    );
}

#[test]
fn jpeg_cmyk_is_rejected_as_a_layout() {
    refused(
        transcode(
            CMYK_JPEG,
            ImageFormat::Jpeg,
            options(OutputFormat::WebP, ASSUME),
        ),
        ProcessingErrorKind::UnsupportedInput,
        LAYOUT,
        Some(RefusalReason::UnsupportedLayout),
    );
}

#[test]
fn jpeg_grayscale_is_expanded_to_rgb() {
    // The adapter requests RGB output, so an untagged grayscale JPEG
    // behaves like untagged RGB.
    let gray = jxl("ycbcr-jpeg-gray.jpg");
    to_webp(&gray, ImageFormat::Jpeg, ASSUME);
    refused(
        transcode(
            &gray,
            ImageFormat::Jpeg,
            options(OutputFormat::WebP, STRICT),
        ),
        ProcessingErrorKind::UnsupportedColor,
        COLOR,
        Some(RefusalReason::ColorSignalMissing),
    );
}

#[test]
fn jpeg_truncated_is_malformed() {
    rejected(
        transcode(
            &SAMPLE_JPEG[..SAMPLE_JPEG.len() / 2],
            ImageFormat::Jpeg,
            options(OutputFormat::WebP, ASSUME),
        ),
        ProcessingErrorKind::MalformedInput,
        MALFORMED,
    );
}

#[test]
fn jpeg_embedded_icc_needs_the_rendered_icc_policy() {
    for policy in [STRICT, ASSUME] {
        refused(
            transcode(
                ICC_JPEG,
                ImageFormat::Jpeg,
                options(OutputFormat::WebP, policy),
            ),
            ProcessingErrorKind::UnsupportedColor,
            COLOR,
            Some(RefusalReason::RenderedIccRequiresOptIn),
        );
    }
    to_webp(ICC_JPEG, ImageFormat::Jpeg, RENDERED);
}

#[test]
fn jpeg_exif_orientations_are_applied_before_crop() {
    for n in 1..=8 {
        let bytes = read(&format!("{ORIENTATION}jpeg-{n}.jpg"));
        let size = if n >= 5 { (16, 24) } else { (24, 16) };
        assert_display_size(&bytes, ImageFormat::Jpeg, ASSUME, size);
    }
}

/// A transparent source for the cases that need no AVIF reader: a libjxl
/// test image with straight alpha, read through the rendered ICC.
fn transparent_jxl_webp() -> Output {
    to_webp(
        &jxl("effects_rgb_alpha8.jxl"),
        ImageFormat::JpegXl,
        RENDERED,
    )
}

// ===========================================================================
// Inputs: still WebP.

#[test]
fn webp_lossy_still_is_accepted() {
    to_webp(LOSSY_WEBP, ImageFormat::WebP, ASSUME);
}

#[test]
fn webp_exif_orientation_is_applied_lossy_and_lossless() {
    for bytes in [ORIENTED_LOSSY_WEBP, ORIENTED_LOSSLESS_WEBP] {
        assert_display_size(bytes, ImageFormat::WebP, ASSUME, (72, 96));
    }
}

#[test]
fn webp_animation_is_rejected_without_a_first_frame_fallback() {
    refused(
        transcode(
            ANIMATED_WEBP,
            ImageFormat::WebP,
            options(OutputFormat::WebP, ASSUME),
        ),
        ProcessingErrorKind::UnsupportedInput,
        ANIMATION,
        Some(RefusalReason::Animation),
    );
}

#[test]
fn webp_straight_alpha_input_keeps_alpha() {
    // A transparent WebP written by the engine from a JPEG XL test image.
    let transparent = transparent_jxl_webp();
    assert!(webp_has_alpha(&transparent));
    let again = to_webp(&transparent.bytes, ImageFormat::WebP, RENDERED);
    assert!(webp_has_alpha(&again));
}

#[test]
fn webp_canvas_above_the_pixel_limit_is_rejected_from_the_header() {
    let options = TransformOptions {
        max_input_pixels: 96 * 72 - 1,
        ..options(OutputFormat::WebP, ASSUME)
    };
    rejected(
        transcode(ORIENTED_LOSSY_WEBP, ImageFormat::WebP, options),
        ProcessingErrorKind::InputLimitExceeded,
        "input has 6912 pixels, exceeding the configured 6911 pixel limit",
    );
}

// ===========================================================================
// Inputs: JPEG XL, through the only reader (libjxl-rs; until 2026-11-15
// these cases also ran on jxl-oxide).

fn jxl_options(policy: InputProfilePolicy) -> TransformOptions {
    options(OutputFormat::WebP, policy)
}

/// Intended change by owner decision of 2026-11-15 (#222): the enum sRGB
/// of `sample.jxl` is a typed signal that every policy reads; only a real
/// embedded profile needs the rendered-ICC policy. (Until then this case
/// was `jpeg_xl_rgb_needs_the_rendered_icc_policy` and refused
/// `sample.jxl` under `Strict` and `AssumeSrgbIfMissing`.)
#[test]
fn jpeg_xl_enum_rgb_is_typed_and_embedded_icc_needs_the_rendered_icc_policy() {
    for policy in [STRICT, ASSUME, RENDERED] {
        let output = transcode(SAMPLE_JXL, ImageFormat::JpegXl, jxl_options(policy)).unwrap();
        assert_eq!(output.report.input_access, SourceAccess::FullFrameFallback);
    }
    let icc = read(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/parity/rgb-p3-icc.jxl"
    ));
    transcode(&icc, ImageFormat::JpegXl, jxl_options(RENDERED)).unwrap();
    for policy in [STRICT, ASSUME] {
        refused(
            transcode(&icc, ImageFormat::JpegXl, jxl_options(policy)),
            ProcessingErrorKind::UnsupportedColor,
            COLOR,
            Some(RefusalReason::RenderedIccRequiresOptIn),
        );
    }
}

#[test]
fn jpeg_xl_straight_alpha_is_kept_through_rendered_icc() {
    let output = transcode(
        &jxl("effects_rgb_alpha8.jxl"),
        ImageFormat::JpegXl,
        jxl_options(RENDERED),
    )
    .unwrap();
    assert!(webp_has_alpha(&output));
}

/// Intended change by owner decision of 2026-11-15 (ADR-0043): gray and
/// gray+alpha JPEG XL are accepted and expanded to RGB(A) like grayscale
/// JPEG; their enum gray sRGB is a typed signal, so every policy reads
/// them. (Until then this case was
/// `jpeg_xl_grayscale_is_rejected_under_every_policy`: `UnsupportedColor`
/// `RenderedIccRequiresOptIn` under `Strict`/`AssumeSrgbIfMissing`,
/// `UnsupportedInput` `UnsupportedLayout` under `UseRenderedIcc`.)
#[test]
fn jpeg_xl_grayscale_is_expanded_to_rgb_under_every_policy() {
    for (name, alpha) in [
        ("ycbcr-jpeg-gray.jxl", false),
        ("modular-gray-alpha12.jxl", true),
    ] {
        for policy in [STRICT, ASSUME, RENDERED] {
            let output = to_webp(&jxl(name), ImageFormat::JpegXl, policy);
            assert_eq!(webp_has_alpha(&output), alpha, "{name} {policy:?}");
        }
    }
}

#[test]
fn jpeg_xl_float_alpha_spot_and_extra_channels_are_rejected() {
    for name in [
        "modular-rgb-alpha-float24e7.jxl",
        "extra-channel-spot-signed-extreme-binary-name.jxl",
        "extra-channel-multiple-alpha-spots-float.jxl",
    ] {
        refused(
            transcode(&jxl(name), ImageFormat::JpegXl, jxl_options(RENDERED)),
            ProcessingErrorKind::UnsupportedInput,
            LAYOUT,
            Some(RefusalReason::UnsupportedLayout),
        );
    }
}

#[test]
fn jpeg_xl_animation_is_rejected() {
    let animated = jxl("decoder-geometry/sequence-reference-overwrite-animation-1.jxl");
    refused(
        transcode(&animated, ImageFormat::JpegXl, jxl_options(RENDERED)),
        ProcessingErrorKind::UnsupportedInput,
        ANIMATION,
        Some(RefusalReason::Animation),
    );
}

#[test]
fn jpeg_xl_pq_and_hlg_are_rejected() {
    let color = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/jxl-color/");
    for name in ["lossless-pq.jxl", "lossless-hlg.jxl", "xyb-pq.jxl"] {
        for policy in [STRICT, ASSUME, RENDERED] {
            let error = transcode(
                &read(&format!("{color}{name}")),
                ImageFormat::JpegXl,
                jxl_options(policy),
            )
            .expect_err("HDR input must be rejected");
            assert_eq!(
                error.kind(),
                ProcessingErrorKind::UnsupportedColor,
                "{name} {policy:?}: {error}"
            );
            // The reader refuses the transfer function itself, before
            // any profile policy applies.
            assert!(error.to_string().starts_with(LINEAR_SRGB), "{error}");
            assert_eq!(
                error.refusal_reason(),
                Some(RefusalReason::HdrTransfer),
                "{error}"
            );
        }
    }
}

#[test]
fn jpeg_xl_pixel_limit_is_checked_before_decoding() {
    let options = TransformOptions {
        max_input_pixels: 1,
        ..jxl_options(RENDERED)
    };
    // libjxl-rs checks in the adapter, before it decodes a pixel.
    let reason = "input has ";
    rejected(
        transcode(SAMPLE_JXL, ImageFormat::JpegXl, options),
        ProcessingErrorKind::InputLimitExceeded,
        reason,
    );
}

// ===========================================================================
// Inputs: still AVIF.

#[cfg(feature = "avif-read")]
#[test]
fn avif_irot_and_imir_are_applied_for_all_eight_orientations() {
    for n in 1..=8 {
        let bytes = read(&format!("{ORIENTATION}avif-{n}.avif"));
        let size = if n >= 5 { (16, 24) } else { (24, 16) };
        assert_display_size(&bytes, ImageFormat::Avif, ASSUME, size);
    }
}

#[cfg(feature = "avif-read")]
#[test]
fn avif_clean_aperture_is_rejected() {
    for bytes in [
        avif("clap_irot_imir_non_essential.avif"),
        read(&format!("{AVIF_ANDROID}avif/blue-and-magenta-crop.avif")),
    ] {
        refused(
            transcode(
                &bytes,
                ImageFormat::Avif,
                options(OutputFormat::WebP, ASSUME),
            ),
            ProcessingErrorKind::UnsupportedInput,
            TRANSFORM,
            Some(RefusalReason::GeometricTransform),
        );
    }
    // The same file with `clap` and `imir` renamed to unknown non-essential
    // properties (libavif's own fixture) keeps only `irot` and is accepted.
    transcode(
        &avif("clop_irot_imor.avif"),
        ImageFormat::Avif,
        options(OutputFormat::WebP, ASSUME),
    )
    .unwrap();
}

#[cfg(feature = "avif-read")]
/// Swap the `irot` and `imir` associations of every item in `ipma`, so the
/// mirror precedes the rotation.
fn mirror_before_rotation(mut bytes: Vec<u8>) -> Vec<u8> {
    let ipco = find(&bytes, b"ipco").unwrap();
    let ipco_size = u32::from_be_bytes(bytes[ipco - 4..ipco].try_into().unwrap()) as usize;
    let mut kinds = Vec::new();
    let mut at = ipco + 4;
    while at < ipco - 4 + ipco_size {
        let size = u32::from_be_bytes(bytes[at..at + 4].try_into().unwrap()) as usize;
        kinds.push(bytes[at + 4..at + 8].to_vec());
        at += size;
    }
    let index = |kind: &[u8]| kinds.iter().position(|k| k == kind).unwrap() + 1;
    let (irot, imir) = (index(b"irot"), index(b"imir"));
    let ipma = find(&bytes, b"ipma").unwrap() + 4;
    let (version, flags) = (bytes[ipma], bytes[ipma + 3]);
    assert!(
        version == 0 && flags & 1 == 0,
        "test helper expects small ipma entries"
    );
    let entries = u32::from_be_bytes(bytes[ipma + 4..ipma + 8].try_into().unwrap());
    let mut at = ipma + 8;
    let mut swapped = 0;
    for _ in 0..entries {
        let count = usize::from(bytes[at + 2]);
        let associations = at + 3..at + 3 + count;
        let rot = associations
            .clone()
            .find(|&i| usize::from(bytes[i] & 0x7f) == irot);
        let mir = associations
            .clone()
            .find(|&i| usize::from(bytes[i] & 0x7f) == imir);
        if let (Some(rot), Some(mir)) = (rot, mir) {
            bytes.swap(rot, mir);
            swapped += 1;
        }
        at = associations.end;
    }
    assert!(swapped > 0);
    bytes
}

#[cfg(feature = "avif-read")]
#[test]
fn avif_imir_before_irot_is_rejected() {
    let reordered = mirror_before_rotation(read(&format!("{ORIENTATION}avif-5.avif")));
    refused(
        transcode(
            &reordered,
            ImageFormat::Avif,
            options(OutputFormat::WebP, ASSUME),
        ),
        ProcessingErrorKind::UnsupportedInput,
        TRANSFORM,
        Some(RefusalReason::GeometricTransform),
    );
}

/// Rename the `pixi` property of a real file to an unknown type and mark
/// its association essential.
#[cfg(feature = "avif-read")]
fn unknown_essential_property(mut bytes: Vec<u8>) -> Vec<u8> {
    let ipco = find(&bytes, b"ipco").unwrap();
    let ipco_size = u32::from_be_bytes(bytes[ipco - 4..ipco].try_into().unwrap()) as usize;
    let mut index = 0;
    let mut at = ipco + 4;
    let mut pixi = None;
    while at < ipco - 4 + ipco_size {
        let size = u32::from_be_bytes(bytes[at..at + 4].try_into().unwrap()) as usize;
        index += 1;
        if &bytes[at + 4..at + 8] == b"pixi" {
            pixi = Some((at + 4, index));
        }
        at += size;
    }
    let (kind, pixi) = pixi.unwrap();
    bytes[kind..kind + 4].copy_from_slice(b"xyzw");
    let ipma = find(&bytes, b"ipma").unwrap() + 4;
    assert!(bytes[ipma] == 0 && bytes[ipma + 3] & 1 == 0);
    let entries = u32::from_be_bytes(bytes[ipma + 4..ipma + 8].try_into().unwrap());
    let mut at = ipma + 8;
    let mut marked = 0;
    for _ in 0..entries {
        let count = usize::from(bytes[at + 2]);
        for association in &mut bytes[at + 3..at + 3 + count] {
            if usize::from(*association & 0x7f) == pixi {
                *association |= 0x80;
                marked += 1;
            }
        }
        at += 3 + count;
    }
    assert!(marked > 0);
    bytes
}

#[cfg(feature = "avif-read")]
#[test]
fn avif_unknown_essential_property_is_refused_as_unsupported() {
    let patched = unknown_essential_property(read(&format!("{ORIENTATION}avif-1.avif")));
    refused(
        transcode(
            &patched,
            ImageFormat::Avif,
            options(OutputFormat::WebP, ASSUME),
        ),
        ProcessingErrorKind::UnsupportedInput,
        ESSENTIAL,
        Some(RefusalReason::UnsupportedEssentialProperty),
    );
}

/// The same box, really malformed: the primary item's association points
/// past the end of `ipco`. The container is broken, so this stays
/// `MalformedInput` without a reason.
#[cfg(feature = "avif-read")]
#[test]
fn avif_property_association_outside_ipco_is_malformed() {
    let mut bytes = read(&format!("{ORIENTATION}avif-1.avif"));
    let ipma = find(&bytes, b"ipma").unwrap() + 4;
    // Version 0, small indices, one item with three associations; the
    // last one (`pixi`, index 3) now names a property 0x7f that `ipco`,
    // with three properties, does not have.
    assert_eq!(&bytes[ipma..ipma + 11], &[0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 3]);
    bytes[ipma + 13] = 0x7f;
    rejected(
        transcode(
            &bytes,
            ImageFormat::Avif,
            options(OutputFormat::WebP, ASSUME),
        ),
        ProcessingErrorKind::MalformedInput,
        MALFORMED,
    );
}

#[cfg(feature = "avif-read")]
#[test]
fn avif_image_sequences_are_rejected_without_a_first_frame_fallback() {
    for bytes in [
        avif("colors-animated-8bpc.avif"),
        avif("colors-animated-8bpc-alpha-exif-xmp.avif"),
        avif("colors-animated-8bpc-audio.avif"),
        avif("colors-animated-8bpc-depth-exif-xmp.avif"),
        avif("colors-animated-12bpc-keyframes-0-2-3.avif"),
        read(&format!("{AVIF_ANDROID}animated_avif/alpha_video.avif")),
    ] {
        refused(
            transcode(
                &bytes,
                ImageFormat::Avif,
                options(OutputFormat::WebP, ASSUME),
            ),
            ProcessingErrorKind::UnsupportedInput,
            ANIMATION,
            Some(RefusalReason::Animation),
        );
    }
}

/// Intended change by owner decision of 2026-11-15 (ADR-0043): monochrome
/// AVIF is accepted and expanded to RGB like grayscale JPEG. (Until then
/// this case was `avif_monochrome_is_rejected_at_every_depth`:
/// `UnsupportedInput`, `UnsupportedLayout`.)
#[cfg(feature = "avif-read")]
#[test]
fn avif_monochrome_is_expanded_to_rgb_at_every_depth() {
    for name in [
        "fox.profile0.8bpc.yuv420.monochrome.avif",
        "fox.profile0.10bpc.yuv420.monochrome.avif",
        "fox.profile2.12bpc.yuv420.monochrome.avif",
    ] {
        let bytes = read(&format!("{AVIF_ANDROID}avif/{name}"));
        to_webp(&bytes, ImageFormat::Avif, ASSUME);
    }
}

#[cfg(feature = "avif-read")]
#[test]
fn avif_eight_ten_and_twelve_bit_in_every_chroma_layout_are_accepted() {
    for name in [
        "fox.profile0.8bpc.yuv420.avif",
        "fox.profile0.10bpc.yuv420.avif",
        "fox.profile1.8bpc.yuv444.avif",
        "fox.profile1.10bpc.yuv444.avif",
        "fox.profile2.8bpc.yuv422.avif",
        "fox.profile2.10bpc.yuv422.avif",
        "fox.profile2.12bpc.yuv420.avif",
        "fox.profile2.12bpc.yuv422.avif",
        "fox.profile2.12bpc.yuv444.avif",
    ] {
        let bytes = read(&format!("{AVIF_ANDROID}avif/{name}"));
        transcode(
            &bytes,
            ImageFormat::Avif,
            options(OutputFormat::WebP, ASSUME),
        )
        .unwrap_or_else(|error| panic!("{name}: {error}"));
    }
}

#[cfg(feature = "avif-read")]
#[test]
fn avif_alpha_and_grid_stills_are_accepted() {
    let alpha = to_webp(
        &avif("abc_color_irot_alpha_irot.avif"),
        ImageFormat::Avif,
        ASSUME,
    );
    assert!(webp_has_alpha(&alpha));
    for name in [
        "color_grid_alpha_grid_shared_tiles.avif",
        "sofa_grid1x5_420.avif",
    ] {
        to_webp(&avif(name), ImageFormat::Avif, ASSUME);
    }
}

#[cfg(feature = "avif-read")]
#[test]
fn avif_gain_map_is_ignored_and_the_base_image_is_read() {
    to_webp(
        &avif("seine_sdr_gainmap_srgb.avif"),
        ImageFormat::Avif,
        STRICT,
    );
}

#[cfg(feature = "avif-read")]
#[test]
fn avif_pq_is_rejected_under_every_policy() {
    for name in ["colors_hdr_rec2020.avif", "seine_hdr_rec2020.avif"] {
        for policy in [STRICT, ASSUME, RENDERED] {
            let error = transcode(
                &avif(name),
                ImageFormat::Avif,
                options(OutputFormat::WebP, policy),
            )
            .expect_err("PQ must be rejected");
            assert_eq!(
                error.kind(),
                ProcessingErrorKind::UnsupportedColor,
                "{name} {policy:?}: {error}"
            );
            // Rendered CICP 9/16 fails the sRGB check; the rendered-ICC
            // stage refuses the file because it carries no ICC profile. The
            // reason names the PQ transfer under every policy.
            let reason = if policy == RENDERED {
                LINEAR_SRGB
            } else {
                COLOR
            };
            assert!(error.to_string().starts_with(reason), "{error}");
            assert_eq!(
                error.refusal_reason(),
                Some(RefusalReason::HdrTransfer),
                "{error}"
            );
        }
    }
}

#[cfg(feature = "avif-read")]
#[test]
fn avif_container_icc_needs_the_rendered_icc_policy() {
    let bytes = avif("paris_icc_exif_xmp.avif");
    for policy in [STRICT, ASSUME] {
        refused(
            transcode(
                &bytes,
                ImageFormat::Avif,
                options(OutputFormat::WebP, policy),
            ),
            ProcessingErrorKind::UnsupportedColor,
            COLOR,
            Some(RefusalReason::RenderedIccRequiresOptIn),
        );
    }
    to_webp(&bytes, ImageFormat::Avif, RENDERED);
}

#[cfg(feature = "avif-read")]
#[test]
fn avif_signalled_srgb_passes_the_strict_policy() {
    to_webp(&avif("colors_sdr_srgb.avif"), ImageFormat::Avif, STRICT);
}

#[cfg(feature = "avif-read")]
#[test]
fn avif_pixel_limit_checks_the_rendered_and_the_coded_size() {
    let bytes = avif("sofa_grid1x5_420.avif");
    let options = TransformOptions {
        max_input_pixels: 1,
        ..options(OutputFormat::WebP, ASSUME)
    };
    rejected(
        transcode(&bytes, ImageFormat::Avif, options),
        ProcessingErrorKind::InputLimitExceeded,
        "input has ",
    );
}

// ===========================================================================
// Limits.

#[test]
fn input_byte_limit_accepts_the_exact_size_and_rejects_one_byte_less() {
    let exact = TransformOptions {
        max_input_bytes: SAMPLE_JPEG.len(),
        ..options(OutputFormat::WebP, ASSUME)
    };
    transcode(SAMPLE_JPEG, ImageFormat::Jpeg, exact).unwrap();
    let below = TransformOptions {
        max_input_bytes: SAMPLE_JPEG.len() - 1,
        ..exact
    };
    rejected(
        transcode(SAMPLE_JPEG, ImageFormat::Jpeg, below),
        ProcessingErrorKind::InputLimitExceeded,
        INPUT_BYTES,
    );
}

#[test]
fn zero_input_limits_are_invalid_options() {
    for (bytes, pixels) in [(0, 1), (1, 0)] {
        let options = TransformOptions {
            max_input_bytes: bytes,
            max_input_pixels: pixels,
            ..options(OutputFormat::WebP, ASSUME)
        };
        rejected(
            transcode(SAMPLE_JPEG, ImageFormat::Jpeg, options),
            ProcessingErrorKind::InvalidOptions,
            ZERO_LIMIT,
        );
    }
}

#[test]
fn jpeg_pixel_limit_is_checked_after_the_header() {
    let options = TransformOptions {
        max_input_pixels: 7 * 5 - 1,
        ..options(OutputFormat::WebP, ASSUME)
    };
    rejected(
        transcode(SAMPLE_JPEG, ImageFormat::Jpeg, options),
        ProcessingErrorKind::InputLimitExceeded,
        "image has 35 pixels; configured limit is 34",
    );
}

/// The exact output size passes, one byte less fails with
/// `ResourceExhausted` and `reason`.
#[track_caller]
fn assert_exact_output_limit(base: TransformOptions, reason: &str) {
    let output = transcode(SAMPLE_JPEG, ImageFormat::Jpeg, base).unwrap();
    let exact = TransformOptions {
        max_output_bytes: output.bytes.len(),
        ..base
    };
    transcode(SAMPLE_JPEG, ImageFormat::Jpeg, exact).unwrap();
    let below = TransformOptions {
        max_output_bytes: output.bytes.len() - 1,
        ..base
    };
    rejected(
        transcode(SAMPLE_JPEG, ImageFormat::Jpeg, below),
        ProcessingErrorKind::ResourceExhausted,
        reason,
    );
}

/// WebP and lossy JPEG XL stop while writing (the encoder's writer reports a
/// budget overrun); lossless JPEG XL is checked by the facade after the
/// encoder returns.
#[test]
fn output_byte_limit_applies_at_the_exact_byte_for_webp_and_jpeg_xl() {
    assert_exact_output_limit(options(OutputFormat::WebP, ASSUME), "managed buffer needs ");
    assert_exact_output_limit(
        options(OutputFormat::JpegXl, ASSUME),
        "managed buffer needs ",
    );
    assert_exact_output_limit(
        TransformOptions {
            jpeg_xl: JpegXlOptions::Lossless { effort: 2 },
            ..options(OutputFormat::JpegXl, ASSUME)
        },
        OUTPUT_BYTES,
    );
}

/// The AVIF writer is checked by the facade after the encoder returns.
/// (Until 2026-11-15 this also ran on rav1e.)
#[cfg(feature = "avif-write")]
#[test]
fn output_byte_limit_applies_at_the_exact_byte_for_avif() {
    assert_exact_output_limit(options(OutputFormat::Avif, ASSUME), OUTPUT_BYTES);
}

#[test]
fn managed_budget_exhaustion_is_reported() {
    let options = TransformOptions {
        max_managed_bytes: 64,
        ..options(OutputFormat::WebP, ASSUME)
    };
    rejected(
        transcode(SAMPLE_JPEG, ImageFormat::Jpeg, options),
        ProcessingErrorKind::ResourceExhausted,
        "managed buffer needs ",
    );
}

#[test]
fn default_budget_rejects_lossless_jpeg_xl_above_about_three_and_a_half_megapixels() {
    // 2000 x 2000 = 4 MP: the measured one-shot peak (76 B/px) exceeds the
    // default 256 MiB budget before the encoder starts.
    let options = TransformOptions {
        jpeg_xl: JpegXlOptions::Lossless { effort: 2 },
        output_width: 2000,
        output_height: 2000,
        ..options(OutputFormat::JpegXl, ASSUME)
    };
    rejected(
        transcode(SAMPLE_JPEG, ImageFormat::Jpeg, options),
        ProcessingErrorKind::ResourceExhausted,
        "managed buffer needs ",
    );
}

#[test]
fn thread_counts_above_256_and_parallel_webp_encoding_are_invalid() {
    for (transform, encoder) in [(257, 1), (1, 257)] {
        let options = TransformOptions {
            transform_threads: transform,
            encoder_threads: encoder,
            ..options(OutputFormat::Avif, ASSUME)
        };
        rejected(
            transcode(SAMPLE_JPEG, ImageFormat::Jpeg, options),
            ProcessingErrorKind::InvalidOptions,
            "transform and encoder threads must be between 0 and 256",
        );
    }
    let options = TransformOptions {
        encoder_threads: 2,
        ..options(OutputFormat::WebP, ASSUME)
    };
    rejected(
        transcode(SAMPLE_JPEG, ImageFormat::Jpeg, options),
        ProcessingErrorKind::InvalidOptions,
        "the WebP encoder is serial; set encoder_threads to 1",
    );
}

// ===========================================================================
// Outputs: AVIF through libavif/AOM, the only AVIF writer (rav1e was a second
// one until 2026-11-15).

#[cfg(all(feature = "avif-read", feature = "avif-write"))]
/// A transparent source: libavif's alpha still, decoded by the engine.
fn transparent_avif_source() -> Vec<u8> {
    avif("abc_color_irot_alpha_irot.avif")
}

#[cfg(all(feature = "avif-read", feature = "avif-write"))]
#[test]
fn libavif_is_the_default_avif_encoder_at_ten_bit_444_with_nclx() {
    let output = transcode(
        SAMPLE_JPEG,
        ImageFormat::Jpeg,
        options(OutputFormat::Avif, ASSUME),
    )
    .unwrap();
    assert_eq!(output.report.encoder_backend, "libavif-rs/aom");
    assert_eq!(av1_config(&output.bytes), (10, (false, false), false));
    assert_eq!(nclx(&output.bytes), Some((1, 13, 6, true)));
    assert!(
        find(&output.bytes, AVIF_ALPHA_URN).is_none(),
        "opaque: no alpha item"
    );
    read_back_avif_strict(&output);
}

#[cfg(all(feature = "avif-read", feature = "avif-write"))]
#[test]
fn libavif_writes_eight_bit_and_420_on_request() {
    let output = transcode(
        SAMPLE_JPEG,
        ImageFormat::Jpeg,
        TransformOptions {
            avif_depth: 8,
            chroma_subsampling: ChromaSubsampling::Yuv420,
            ..options(OutputFormat::Avif, ASSUME)
        },
    )
    .unwrap();
    assert_eq!(av1_config(&output.bytes), (8, (true, true), false));
    assert_eq!(nclx(&output.bytes), Some((1, 13, 6, true)));
    read_back_avif_strict(&output);
}

#[cfg(all(feature = "avif-read", feature = "avif-write"))]
#[test]
fn libavif_writes_an_alpha_item_only_for_transparent_input() {
    let output = transcode(
        &transparent_avif_source(),
        ImageFormat::Avif,
        options(OutputFormat::Avif, ASSUME),
    )
    .unwrap();
    assert!(find(&output.bytes, AVIF_ALPHA_URN).is_some());
    let back = read_back_avif_strict(&output);
    assert!(webp_has_alpha(&back));
}

#[cfg(all(feature = "avif-read", feature = "avif-write"))]
#[test]
fn libavif_quality_speed_and_depth_ranges() {
    let run = |avif_depth, quality, speed| {
        transcode(
            SAMPLE_JPEG,
            ImageFormat::Jpeg,
            TransformOptions {
                avif_depth,
                quality,
                speed,
                ..options(OutputFormat::Avif, ASSUME)
            },
        )
    };
    let libavif = 10;
    run(libavif, 0.0, 10).unwrap();
    run(libavif, 100.0, 0).unwrap();
    rejected(
        run(libavif, 100.5, 6),
        ProcessingErrorKind::InvalidOptions,
        "encoder backend: libavif quality must be between 0 and 100",
    );
    rejected(
        run(libavif, 75.0, 11),
        ProcessingErrorKind::InvalidOptions,
        "encoder backend: libavif speed must be between 0 and 10",
    );
    rejected(
        run(12, 75.0, 6),
        ProcessingErrorKind::InvalidOptions,
        "encoder backend: libavif-rs backend depth must be 8 or 10",
    );
}

#[cfg(feature = "avif-write")]
#[test]
fn avif_output_drops_source_metadata_and_writes_upright_pixels() {
    // EXIF orientation 6 (24x16 stored): output is upright, without an
    // EXIF/XMP item or an irot/imir property.
    let oriented = read(&format!("{ORIENTATION}jpeg-6.jpg"));
    {
        let output = transcode(
            &oriented,
            ImageFormat::Jpeg,
            TransformOptions {
                crop: Some(Crop {
                    x: 0,
                    y: 0,
                    width: 16,
                    height: 24,
                }),
                ..options(OutputFormat::Avif, ASSUME)
            },
        )
        .unwrap();
        for absent in [&b"Exif"[..], b"mime", b"irot", b"imir", b"clap"] {
            assert!(
                find(&output.bytes, absent).is_none(),
                "{}",
                String::from_utf8_lossy(absent)
            );
        }
        let source_metadata = transcode(
            ICC_JPEG,
            ImageFormat::Jpeg,
            options(OutputFormat::Avif, RENDERED),
        )
        .unwrap();
        for absent in [&b"Exif"[..], b"mime", b"prof"] {
            assert!(
                find(&source_metadata.bytes, absent).is_none(),
                "{}",
                String::from_utf8_lossy(absent)
            );
        }
    }
}

#[test]
fn avif_depth_conflicts_with_other_outputs() {
    // (Until 2026-11-15 the AVIF encoder selection, which held the depth.)
    rejected(
        transcode(
            SAMPLE_JPEG,
            ImageFormat::Jpeg,
            TransformOptions {
                avif_depth: 8,
                ..options(OutputFormat::WebP, ASSUME)
            },
        ),
        ProcessingErrorKind::InvalidOptions,
        "AVIF depth conflicts with the requested output or external backend",
    );
}

// ===========================================================================
// Outputs: still WebP.

#[test]
fn webp_lossy_and_lossless_carry_srgb_icc_and_no_exif_or_xmp() {
    for (webp, image_chunk) in [
        (WebpOptions::default(), *b"VP8 "),
        (WebpOptions::Lossless { level: 6 }, *b"VP8L"),
    ] {
        let output = transcode(
            ICC_JPEG,
            ImageFormat::Jpeg,
            TransformOptions {
                webp,
                ..options(OutputFormat::WebP, RENDERED)
            },
        )
        .unwrap();
        let chunks = webp_chunks(&output.bytes);
        assert!(chunks.contains(b"ICCP"), "{chunks:?}");
        assert!(chunks.contains(&image_chunk), "{chunks:?}");
        assert!(
            !chunks.contains(b"EXIF") && !chunks.contains(b"XMP "),
            "{chunks:?}"
        );
        let metadata = inspect(
            &output.bytes,
            ImageFormat::WebP,
            InspectionLimits::default(),
        )
        .unwrap();
        assert_eq!(&metadata.rendered_icc.unwrap()[36..40], b"acsp");
        // The engine reads its own output back through the rendered ICC.
        to_webp(&output.bytes, ImageFormat::WebP, RENDERED);
    }
}

#[test]
fn webp_straight_alpha_output_in_both_profiles() {
    let source = jxl("effects_rgb_alpha8.jxl");
    for webp in [WebpOptions::default(), WebpOptions::Lossless { level: 6 }] {
        let output = transcode(
            &source,
            ImageFormat::JpegXl,
            TransformOptions {
                webp,
                ..options(OutputFormat::WebP, RENDERED)
            },
        )
        .unwrap();
        assert!(webp_has_alpha(&output), "{webp:?}");
        let chunks = webp_chunks(&output.bytes);
        match webp {
            WebpOptions::Lossy { .. } => assert!(chunks.contains(b"ALPH"), "{chunks:?}"),
            _ => assert!(chunks.contains(b"VP8L"), "{chunks:?}"),
        }
    }
}

#[test]
fn webp_quality_speed_and_level_ranges() {
    let run = |webp| {
        transcode(
            SAMPLE_JPEG,
            ImageFormat::Jpeg,
            TransformOptions {
                webp,
                ..options(OutputFormat::WebP, ASSUME)
            },
        )
    };
    for webp in [
        WebpOptions::Lossy {
            quality: 1.0,
            speed: 1,
        },
        WebpOptions::Lossy {
            quality: 100.0,
            speed: 10,
        },
        WebpOptions::Lossless { level: 0 },
        WebpOptions::Lossless { level: 9 },
    ] {
        run(webp).unwrap_or_else(|error| panic!("{webp:?}: {error}"));
    }
    for webp in [
        WebpOptions::Lossy {
            quality: 0.0,
            speed: 6,
        },
        WebpOptions::Lossy {
            quality: 100.5,
            speed: 6,
        },
        WebpOptions::Lossy {
            quality: 75.0,
            speed: 0,
        },
        WebpOptions::Lossy {
            quality: 75.0,
            speed: 11,
        },
        WebpOptions::Lossless { level: 10 },
    ] {
        rejected(
            run(webp),
            ProcessingErrorKind::InvalidOptions,
            ENCODER_OPTIONS,
        );
    }
}

#[test]
fn avif_settings_with_webp_output_are_mismatched_options() {
    rejected(
        transcode(
            SAMPLE_JPEG,
            ImageFormat::Jpeg,
            TransformOptions {
                quality: 80.0,
                ..options(OutputFormat::WebP, ASSUME)
            },
        ),
        ProcessingErrorKind::InvalidOptions,
        MISMATCHED,
    );
}

// ===========================================================================
// Outputs: JPEG XL.

fn jxl_output(source: &[u8], format: ImageFormat, jpeg_xl: JpegXlOptions) -> Output {
    transcode(
        source,
        format,
        TransformOptions {
            jpeg_xl,
            ..options(OutputFormat::JpegXl, ASSUME)
        },
    )
    .unwrap_or_else(|error| panic!("{jpeg_xl:?}: {error}"))
}

#[test]
fn jpeg_xl_output_is_a_raw_codestream_that_reads_back() {
    for jpeg_xl in [
        JpegXlOptions::default(),
        JpegXlOptions::Lossless { effort: 2 },
    ] {
        let output = jxl_output(SAMPLE_JPEG, ImageFormat::Jpeg, jpeg_xl);
        assert_eq!(&output.bytes[..2], &[0xff, 0x0a], "{jpeg_xl:?}");
        assert_eq!(output.report.encoder_backend, "libjxl-rs");
        transcode(&output.bytes, ImageFormat::JpegXl, jxl_options(RENDERED))
            .unwrap_or_else(|error| panic!("{jpeg_xl:?}: {error}"));
        // jxl-oxide, test-only since 2026-11-15, reads it independently.
        let image = jxl_oxide::JxlImage::builder()
            .read(output.bytes.as_slice())
            .unwrap_or_else(|error| panic!("{jpeg_xl:?}: jxl-oxide: {error}"));
        image
            .render_frame(0)
            .unwrap_or_else(|error| panic!("{jpeg_xl:?}: jxl-oxide: {error}"));
    }
}

#[test]
fn jpeg_xl_output_keeps_straight_alpha_in_both_profiles() {
    for jpeg_xl in [
        JpegXlOptions::default(),
        JpegXlOptions::Lossless { effort: 2 },
    ] {
        let output = transcode(
            &jxl("effects_rgb_alpha8.jxl"),
            ImageFormat::JpegXl,
            TransformOptions {
                jpeg_xl,
                ..options(OutputFormat::JpegXl, RENDERED)
            },
        )
        .unwrap();
        let back = transcode(&output.bytes, ImageFormat::JpegXl, jxl_options(RENDERED)).unwrap();
        assert!(webp_has_alpha(&back), "{jpeg_xl:?}");
    }
}

#[test]
fn jpeg_xl_output_is_written_upright() {
    let oriented = read(&format!("{ORIENTATION}jpeg-6.jpg"));
    let output = transcode(
        &oriented,
        ImageFormat::Jpeg,
        TransformOptions {
            output_width: 16,
            output_height: 24,
            ..options(OutputFormat::JpegXl, ASSUME)
        },
    )
    .unwrap();
    // Read back without any orientation left to apply: 16 x 24 upright.
    assert_display_size(&output.bytes, ImageFormat::JpegXl, RENDERED, (16, 24));
}

#[test]
fn jpeg_xl_distance_effort_and_foreign_settings_are_checked_before_decoding() {
    let run = |jpeg_xl, quality| {
        transcode(
            SAMPLE_JPEG,
            ImageFormat::Jpeg,
            TransformOptions {
                jpeg_xl,
                quality,
                ..options(OutputFormat::JpegXl, ASSUME)
            },
        )
    };
    run(JpegXlOptions::Lossy { distance: 0.05 }, 75.0).unwrap();
    run(JpegXlOptions::Lossy { distance: 2.99 }, 75.0).unwrap();
    for jpeg_xl in [
        JpegXlOptions::Lossy { distance: 0.049 },
        JpegXlOptions::Lossy { distance: 3.0 },
        JpegXlOptions::Lossless { effort: 3 },
    ] {
        let error = run(jpeg_xl, 75.0).unwrap_err();
        assert_eq!(
            error.kind(),
            ProcessingErrorKind::InvalidOptions,
            "{jpeg_xl:?}: {error}"
        );
        assert!(
            error.to_string().starts_with("JPEG XL encoder profile: "),
            "{jpeg_xl:?}: {error}"
        );
    }
    rejected(
        run(JpegXlOptions::default(), 80.0),
        ProcessingErrorKind::InvalidOptions,
        MISMATCHED,
    );
    rejected(
        transcode(
            SAMPLE_JPEG,
            ImageFormat::Jpeg,
            TransformOptions {
                speed: 3,
                ..options(OutputFormat::JpegXl, ASSUME)
            },
        ),
        ProcessingErrorKind::InvalidOptions,
        MISMATCHED,
    );
}
