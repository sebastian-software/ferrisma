//! Evidence for `docs/silent-drops-audit-2026-11-15.md`: what a successful
//! transcode drops, ignores or changes **without an error and without a
//! report field**, as of 2026-11-15.
//!
//! Read every test here as a record of today's behavior, **not as an
//! endorsement**. The owner's principle (2026-11-15) is that nothing may
//! disappear implicitly; each case below is a place where something does,
//! and the audit proposes how the engine should tell the caller instead. The
//! names end in `_today` for that reason. When the engine starts to refuse
//! or report one of these cases, change the test to the new contract; do
//! not "fix" the engine back to pass it.
//!
//! All files are real fixtures from the repository; where a few bytes are
//! patched, the helper says which and why. Run with every adapter:
//! `cargo test -p ferrisma --features cli --test silent_drops`.
#![cfg(all(
    feature = "jpeg-read",
    feature = "webp-read",
    feature = "webp-write",
    feature = "jpeg-xl-read",
    feature = "jpeg-xl-own-read",
    feature = "jpeg-xl-write",
    feature = "icc"
))]

use ferrisma::pipeline::{
    BatchItem, Engine, ImageFormat, InputProfilePolicy, InspectionLimits, JpegXlDecoder,
    JpegXlOptions, Output, OutputFormat, ProcessingError, ProcessingErrorKind, RefusalReason,
    TransformOptions, WebpOptions, inspect, transcode,
};

const ROOT: &str = env!("CARGO_MANIFEST_DIR");
const JXL_DATA: &str = "codecs/libjxl-rs/tests/data/";
const AVIF_DATA: &str = "codecs/libavif-rs/upstream/libavif/tests/data/";

const ASSUME: InputProfilePolicy = InputProfilePolicy::AssumeSrgbIfMissing;
const RENDERED: InputProfilePolicy = InputProfilePolicy::UseRenderedIcc;

fn read(path: &str) -> Vec<u8> {
    std::fs::read(format!("{ROOT}/{path}")).unwrap_or_else(|error| panic!("{path}: {error}"))
}

fn options(policy: InputProfilePolicy, decoder: JpegXlDecoder) -> TransformOptions {
    TransformOptions {
        output_format: OutputFormat::WebP,
        output_width: 6,
        output_height: 4,
        profile_policy: policy,
        jxl_decoder: decoder,
        transform_threads: 1,
        ..TransformOptions::default()
    }
}

#[track_caller]
fn accepted(result: Result<Output, ProcessingError>) -> Output {
    result.unwrap_or_else(|error| panic!("accepted today, got {error}"))
}

#[track_caller]
fn refused(result: Result<Output, ProcessingError>, kind: ProcessingErrorKind) -> ProcessingError {
    let error = result.expect_err("refused today");
    assert_eq!(error.kind(), kind, "{error}");
    error
}

fn webp_has_alpha(output: &Output) -> bool {
    inspect(
        &output.bytes,
        ImageFormat::WebP,
        InspectionLimits::default(),
    )
    .unwrap()
    .has_alpha
}

fn find(bytes: &[u8], needle: &[u8]) -> Option<usize> {
    bytes
        .windows(needle.len())
        .position(|window| window == needle)
}

// ---------------------------------------------------------------------------
// JPEG XL: the two readers disagree on extra channels.

/// jxl-oxide renders the color channels and the first alpha channel and
/// leaves every other extra channel out of its pixel stream (jxl-oxide
/// 0.12.6 `PixelFormat` knows only gray, RGB, CMYK and alpha). The adapter
/// cannot see the dropped channels, so depth (type 1), selection mask (3),
/// a black channel next to RGB color (4), CFA (5), thermal (6) and an
/// optional channel (16) pass as plain RGB. libjxl-rs refuses the same
/// files as `UnsupportedLayout`. Type 2 (spot color) is also accepted:
/// jxl-oxide composites the spot color into the RGB samples
/// (`render_spot_color`, on by default for color images), an unrequested
/// change of the color values. Not an endorsement.
#[test]
fn jxl_oxide_drops_extra_channels_that_libjxl_rs_refuses_today() {
    for kind in [1, 2, 3, 4, 5, 6, 16] {
        let bytes = read(&format!("{JXL_DATA}extra-channel-type-{kind}.jxl"));
        let output = accepted(transcode(
            &bytes,
            ImageFormat::JpegXl,
            options(RENDERED, JpegXlDecoder::JxlOxide),
        ));
        assert!(!webp_has_alpha(&output), "type {kind}");
        let error = refused(
            transcode(
                &bytes,
                ImageFormat::JpegXl,
                options(RENDERED, JpegXlDecoder::LibjxlRs),
            ),
            ProcessingErrorKind::UnsupportedInput,
        );
        assert_eq!(
            error.refusal_reason(),
            Some(RefusalReason::UnsupportedLayout),
            "type {kind}"
        );
    }
}

/// Two alpha channels, the first unassociated: jxl-oxide renders the first
/// one and drops the second without a signal; libjxl-rs refuses the file
/// (more than one extra channel). Not an endorsement.
#[test]
fn jxl_oxide_keeps_only_the_first_of_several_alpha_channels_today() {
    let bytes = read(&format!(
        "{JXL_DATA}extra-channel-alpha-first-unassociated.jxl"
    ));
    let output = accepted(transcode(
        &bytes,
        ImageFormat::JpegXl,
        options(RENDERED, JpegXlDecoder::JxlOxide),
    ));
    assert!(webp_has_alpha(&output));
    refused(
        transcode(
            &bytes,
            ImageFormat::JpegXl,
            options(RENDERED, JpegXlDecoder::LibjxlRs),
        ),
        ProcessingErrorKind::UnsupportedInput,
    );
}

/// An extra channel of the reserved type 15, which libjxl's metadata parser
/// rejects (the fixture's manifest records the original's error): libjxl-rs
/// reports `MalformedInput`, jxl-oxide decodes it and drops the channel.
/// The same bytes are malformed or fine depending on the reader. Not an
/// endorsement.
#[test]
fn jxl_oxide_accepts_a_reserved_extra_channel_type_that_libjxl_rejects_today() {
    let bytes = read(&format!("{JXL_DATA}extra-channel-rejected-15.jxl"));
    accepted(transcode(
        &bytes,
        ImageFormat::JpegXl,
        options(RENDERED, JpegXlDecoder::JxlOxide),
    ));
    refused(
        transcode(
            &bytes,
            ImageFormat::JpegXl,
            options(RENDERED, JpegXlDecoder::LibjxlRs),
        ),
        ProcessingErrorKind::MalformedInput,
    );
}

/// libjxl's `1x1_exif_xmp.jxl` container carries `Exif`, `xml ` and a
/// `jbrd` (JPEG reconstruction) box beside the codestream. Both readers
/// decode the pixels and ignore the three boxes; nothing reports that the
/// metadata or the means of a bit-exact JPEG recovery existed. Not an
/// endorsement.
#[test]
fn jpeg_xl_container_boxes_are_dropped_today() {
    let bytes = read(&format!("{JXL_DATA}decode-test/1x1_exif_xmp.jxl"));
    for kind in [b"Exif", b"xml ", b"jbrd"] {
        assert!(find(&bytes, kind).is_some());
    }
    for decoder in [JpegXlDecoder::LibjxlRs, JpegXlDecoder::JxlOxide] {
        accepted(transcode(
            &bytes,
            ImageFormat::JpegXl,
            options(RENDERED, decoder),
        ));
    }
}

/// JPEG XL output is a raw codestream (ADR-0039), which cannot carry EXIF,
/// XMP or ICC boxes: the EXIF and XMP of the source are dropped and the
/// supported matrix does not say so for JPEG XL (it does for AVIF and
/// WebP). Not an endorsement.
#[test]
fn source_exif_and_xmp_are_dropped_from_jpeg_xl_output_today() {
    let jpeg = read(&format!("{AVIF_DATA}paris_exif_xmp_icc.jpg"));
    assert!(find(&jpeg, b"Exif\0\0").is_some());
    assert!(find(&jpeg, b"http://ns.adobe.com/xap/1.0/").is_some());
    let output = accepted(transcode(
        &jpeg,
        ImageFormat::Jpeg,
        TransformOptions {
            output_format: OutputFormat::JpegXl,
            jpeg_xl: JpegXlOptions::default(),
            ..options(RENDERED, JpegXlDecoder::default())
        },
    ));
    assert_eq!(&output.bytes[..2], &[0xff, 0x0a], "raw codestream");
    assert!(find(&output.bytes, b"Exif").is_none());
    assert!(find(&output.bytes, b"xap/1.0").is_none());
}

// ---------------------------------------------------------------------------
// JPEG.

/// Ultra HDR / Apple gain-map JPEGs: the MPF secondary image (the gain map)
/// and its XMP are ignored and the SDR base image is read; the report does
/// not say that a gain map existed. Documented as "MPF secondary images
/// are not read" in the supported matrix, so (B), but nothing tells the
/// caller. Not an endorsement.
#[test]
fn jpeg_gain_maps_are_ignored_and_the_base_image_is_read_today() {
    for name in ["apple_gainmap_new.jpg", "seine_sdr_gainmap_srgb.jpg"] {
        let bytes = read(&format!("{AVIF_DATA}{name}"));
        assert!(find(&bytes, b"MPF\0").is_some(), "{name} has an MPF index");
        accepted(transcode(
            &bytes,
            ImageFormat::Jpeg,
            options(RENDERED, JpegXlDecoder::default()),
        ));
    }
}

/// RIFF chunk tags of a WebP file.
fn webp_chunks(bytes: &[u8]) -> Vec<[u8; 4]> {
    assert_eq!(&bytes[8..12], b"WEBP");
    let mut chunks = Vec::new();
    let mut at = 12;
    while at + 8 <= bytes.len() {
        chunks.push(bytes[at..at + 4].try_into().unwrap());
        let size = u32::from_le_bytes(bytes[at + 4..at + 8].try_into().unwrap()) as usize;
        at += 8 + size.next_multiple_of(2);
    }
    chunks
}

/// IPTC (APP13 `Photoshop 3.0` in `seine_sdr_gainmap_srgb.jpg`) and a
/// `COM` segment (`apple_gainmap_new.jpg`) are not read and do not reach
/// the output, whose chunks are the image and the engine's sRGB `ICCP`
/// only. Neither kind is named in the supported matrix. Not an endorsement.
#[test]
fn jpeg_iptc_and_comments_are_dropped_today() {
    for (name, marker) in [
        ("seine_sdr_gainmap_srgb.jpg", &b"Photoshop 3.0\0"[..]),
        ("apple_gainmap_new.jpg", &b"\xff\xfe"[..]),
    ] {
        let bytes = read(&format!("{AVIF_DATA}{name}"));
        assert!(find(&bytes, marker).is_some(), "{name}");
        let output = accepted(transcode(
            &bytes,
            ImageFormat::Jpeg,
            options(RENDERED, JpegXlDecoder::default()),
        ));
        for chunk in webp_chunks(&output.bytes) {
            assert!(
                matches!(&chunk, b"VP8X" | b"ICCP" | b"VP8 " | b"VP8L" | b"ALPH"),
                "{name}: {chunk:?}"
            );
        }
        assert!(find(&output.bytes, b"8BIM").is_none(), "{name}");
    }
}

/// Sixteen bytes inside the entropy-coded scan of a real photograph
/// inverted: the file decodes without an error. The adapter's
/// `fail_on_warning` turns every libjpeg warning (missing EOI, truncation,
/// marker damage) into `MalformedInput`, but Huffman-coded garbage that
/// still parses raises no warning, so there is nothing to turn into an
/// error. As for WebP, the format cannot detect it and the engine swallows
/// nothing; nothing tells the caller either. Not an endorsement.
#[test]
fn corrupted_jpeg_scan_data_decodes_without_an_error_today() {
    let mut bytes = read("tests/fixtures/decoder-work/paris-420.jpg");
    let sos = find(&bytes, &[0xff, 0xda]).unwrap();
    let header = usize::from(u16::from_be_bytes([bytes[sos + 2], bytes[sos + 3]]));
    let scan = sos + 2 + header;
    let middle = scan + (bytes.len() - 2 - scan) / 2;
    for byte in &mut bytes[middle..middle + 16] {
        *byte ^= 0x5a;
    }
    accepted(transcode(
        &bytes,
        ImageFormat::Jpeg,
        options(ASSUME, JpegXlDecoder::default()),
    ));
}

/// `sample.jpg` with its JFIF density patched from 1:1 to 2:1 (byte 15, the
/// low byte of `Xdensity`, units 0 = aspect ratio only): the pixel aspect
/// ratio is ignored. The display size stays the stored pixel size, and no
/// field reports the ratio. Not an endorsement.
#[test]
fn jpeg_pixel_aspect_ratio_is_ignored_today() {
    let mut bytes = read("tests/fixtures/sample.jpg");
    assert_eq!(&bytes[6..11], b"JFIF\0");
    assert_eq!(&bytes[13..18], &[0, 0, 1, 0, 1], "units 0, density 1:1");
    bytes[15] = 2;
    let metadata = inspect(&bytes, ImageFormat::Jpeg, InspectionLimits::default()).unwrap();
    assert_eq!(
        (metadata.width, metadata.height),
        (metadata.stored_width, metadata.stored_height)
    );
    accepted(transcode(
        &bytes,
        ImageFormat::Jpeg,
        options(ASSUME, JpegXlDecoder::default()),
    ));
}

// ---------------------------------------------------------------------------
// Corruption that the codec cannot detect, and truncation that it does.

/// Sixteen bytes in the middle of a lossy and a lossless WebP inverted:
/// both decode without an error. VP8 and VP8L have no checksum, so the
/// decoder cannot notice; nothing is swallowed by the engine, but nothing
/// tells the caller either. Truncation, by contrast, is an error
/// (`truncated_inputs_are_malformed`). Not an endorsement.
#[test]
fn corrupted_webp_payload_decodes_without_an_error_today() {
    for name in [
        "tests/fixtures/webp/reference-lossy.webp",
        "tests/fixtures/decoder-work/paris-o5-lossless.webp",
    ] {
        let mut bytes = read(name);
        let middle = bytes.len() / 2;
        for byte in &mut bytes[middle..middle + 16] {
            *byte ^= 0x5a;
        }
        accepted(transcode(
            &bytes,
            ImageFormat::WebP,
            options(ASSUME, JpegXlDecoder::default()),
        ));
    }
}

/// Truncated JPEG XL (both readers) and WebP files are `MalformedInput`,
/// not a partial image: libjxl-rs needs `FULL_IMAGE` and `SUCCESS`, and
/// jxl-oxide does not render a partially loaded frame here. (JPEG:
/// `supported_matrix::jpeg_truncated_is_malformed`; AVIF below.)
#[test]
fn truncated_inputs_are_malformed() {
    let jxl = read("tests/fixtures/jxl-lossless/rgb-300x260.jxl");
    for decoder in [JpegXlDecoder::LibjxlRs, JpegXlDecoder::JxlOxide] {
        for cut in [jxl.len() - 1, jxl.len() / 2] {
            refused(
                transcode(&jxl[..cut], ImageFormat::JpegXl, options(RENDERED, decoder)),
                ProcessingErrorKind::MalformedInput,
            );
        }
    }
    for name in [
        "tests/fixtures/webp/reference-lossy.webp",
        "tests/fixtures/decoder-work/paris-o5-lossless.webp",
    ] {
        let webp = read(name);
        refused(
            transcode(
                &webp[..webp.len() - 1],
                ImageFormat::WebP,
                options(ASSUME, JpegXlDecoder::default()),
            ),
            ProcessingErrorKind::MalformedInput,
        );
    }
}

// ---------------------------------------------------------------------------
// Options that are accepted but have no (distinct) effect.

/// `WebpOptions::Lossy::speed` maps to libwebp's method 0–6 with
/// `(6 * (10 - speed) + 4) / 9`: speeds 2 and 3, 5 and 6, 8 and 9 select the
/// same method and give the same bytes. The documented range 1–10 promises
/// ten steps; three are silently merged. Not an endorsement.
#[test]
fn webp_lossy_speeds_collapse_in_pairs_today() {
    let jpeg = read("tests/fixtures/decoder-work/paris-420.jpg");
    let encode = |speed| {
        accepted(transcode(
            &jpeg,
            ImageFormat::Jpeg,
            TransformOptions {
                webp: WebpOptions::Lossy {
                    quality: 75.0,
                    speed,
                },
                output_width: 200,
                output_height: 133,
                ..options(ASSUME, JpegXlDecoder::default())
            },
        ))
        .bytes
    };
    let bytes: Vec<Vec<u8>> = (1..=10).map(encode).collect();
    for speed in 1..10_u8 {
        let same = bytes[usize::from(speed) - 1] == bytes[usize::from(speed)];
        assert_eq!(
            same,
            matches!(speed, 2 | 5 | 8),
            "speed {speed} against {}",
            speed + 1
        );
    }
}

/// `Engine::transcode_batch` replaces both thread counts with 1 (documented
/// on `BatchItem::options`, and the report says 1), so a value that
/// `transcode` rejects as `InvalidOptions` (above 256) is accepted and
/// discarded in a batch. Not an endorsement.
#[test]
fn batch_accepts_thread_counts_that_transcode_rejects_today() {
    let jpeg = read("tests/fixtures/sample.jpg");
    let item = BatchItem {
        encoded: &jpeg,
        format: ImageFormat::Jpeg,
        options: TransformOptions {
            transform_threads: 1000,
            encoder_threads: 1000,
            ..options(ASSUME, JpegXlDecoder::default())
        },
    };
    refused(
        transcode(&jpeg, ImageFormat::Jpeg, item.options),
        ProcessingErrorKind::InvalidOptions,
    );
    let results = Engine::new(1).transcode_batch(&[item]);
    let output = results[0].as_ref().expect("accepted in a batch today");
    assert_eq!(
        (
            output.report.transform_threads,
            output.report.encoder_threads
        ),
        (1, 1)
    );
}

/// `TransformOptions::jxl_decoder` is accepted for any input format and
/// ignored for every format but JPEG XL. The report names the reader that
/// ran (`decoder_backend`), so a caller can see it; recorded as (C).
#[test]
fn jpeg_xl_reader_selection_is_ignored_for_jpeg_input_and_reported() {
    let output = accepted(transcode(
        &read("tests/fixtures/sample.jpg"),
        ImageFormat::Jpeg,
        options(ASSUME, JpegXlDecoder::JxlOxide),
    ));
    assert_eq!(output.report.decoder_backend, "libjpeg-turbo-rs");
}

// ---------------------------------------------------------------------------
// AVIF.

#[cfg(feature = "avif-read")]
/// Rename property `index` (one-based in `ipco`) to `xyzw` and mark every
/// association to it essential (version-0 `ipma` with small indices).
fn mark_unknown_essential(mut bytes: Vec<u8>, index: usize) -> Vec<u8> {
    let ipco = find(&bytes, b"ipco").unwrap();
    let size = u32::from_be_bytes(bytes[ipco - 4..ipco].try_into().unwrap()) as usize;
    let mut at = ipco + 4;
    for _ in 1..index {
        at += u32::from_be_bytes(bytes[at..at + 4].try_into().unwrap()) as usize;
    }
    assert!(at < ipco - 4 + size);
    bytes[at + 4..at + 8].copy_from_slice(b"xyzw");
    let ipma = find(&bytes, b"ipma").unwrap() + 4;
    assert!(bytes[ipma] == 0 && bytes[ipma + 3] & 1 == 0);
    let entries = u32::from_be_bytes(bytes[ipma + 4..ipma + 8].try_into().unwrap());
    let mut at = ipma + 8;
    let mut marked = 0;
    for _ in 0..entries {
        let count = usize::from(bytes[at + 2]);
        for association in &mut bytes[at + 3..at + 3 + count] {
            if usize::from(*association & 0x7f) == index {
                *association |= 0x80;
                marked += 1;
            }
        }
        at += 3 + count;
    }
    assert!(marked > 0);
    bytes
}

/// libavif's `abc_color_irot_alpha_irot.avif`, whose alpha item has its own
/// `pixi` (property 6; the color item uses property 2): that property
/// renamed to an unknown type and marked essential. The engine checks the
/// essential properties of the primary item only; libavif skips any item
/// with an unsupported essential property (`hasUnsupportedEssentialProperty`
/// in `read.c`), so the alpha plane is not found and the image is read as
/// opaque, without an error. The same property on the primary item is a
/// refusal (`supported_matrix::avif_unknown_essential_property_is_refused_as_unsupported`).
/// Not an endorsement.
#[cfg(feature = "avif-read")]
#[test]
fn avif_alpha_item_with_an_unknown_essential_property_is_dropped_today() {
    let original = read(&format!("{AVIF_DATA}abc_color_irot_alpha_irot.avif"));
    let output = accepted(transcode(
        &original,
        ImageFormat::Avif,
        options(ASSUME, JpegXlDecoder::default()),
    ));
    assert!(webp_has_alpha(&output), "the unpatched file keeps alpha");
    let patched = mark_unknown_essential(original, 6);
    let output = accepted(transcode(
        &patched,
        ImageFormat::Avif,
        options(ASSUME, JpegXlDecoder::default()),
    ));
    assert!(!webp_has_alpha(&output), "alpha is dropped today");
}

/// Non-essential properties the parser does not know (`1234`, `abcd` and a
/// `uuid` box in libavif's `circle_custom_properties.avif`) are ignored, as
/// ISO/IEC 23008-12 allows; nothing reports that they existed. The same
/// path ignores `pasp` (pixel aspect ratio), for which the repository has
/// no fixture. Not an endorsement.
#[cfg(feature = "avif-read")]
#[test]
fn avif_non_essential_unknown_properties_are_ignored_today() {
    let output = accepted(transcode(
        &read(&format!("{AVIF_DATA}circle_custom_properties.avif")),
        ImageFormat::Avif,
        options(ASSUME, JpegXlDecoder::default()),
    ));
    assert!(webp_has_alpha(&output));
}

/// libavif's `weld_sato_12B_8B_q0.avif`: the primary item is a 12-bit image;
/// an alternative `sato` (sample transform) item would reconstruct 16 bits
/// from it and an 8-bit residual. The engine reads the 12-bit primary
/// (libavif without its experimental sample-transform support) and, like
/// every input, hands 8-bit rows to the encoder; nothing reports that a
/// higher-precision alternative existed. Not an endorsement.
#[cfg(feature = "avif-read")]
#[test]
fn avif_sample_transform_alternative_is_ignored_today() {
    let bytes = read(&format!("{AVIF_DATA}weld_sato_12B_8B_q0.avif"));
    assert!(find(&bytes, b"satoSampleTransform").is_some());
    accepted(transcode(
        &bytes,
        ImageFormat::Avif,
        options(ASSUME, JpegXlDecoder::default()),
    ));
}

/// Truncated AVIF is `MalformedInput`, not a partial image.
#[cfg(feature = "avif-read")]
#[test]
fn truncated_avif_is_malformed() {
    let bytes = read(&format!("{AVIF_DATA}paris_icc_exif_xmp.avif"));
    for cut in [bytes.len() - 1, bytes.len() / 2] {
        refused(
            transcode(
                &bytes[..cut],
                ImageFormat::Avif,
                options(RENDERED, JpegXlDecoder::default()),
            ),
            ProcessingErrorKind::MalformedInput,
        );
    }
}

// ---------------------------------------------------------------------------
// The command line.

/// The CLI applies flags in order and a later flag of the same kind
/// silently replaces an earlier one (`--avif-encoder` and `--avif-depth`
/// are the only flags refused when repeated). Contradictory instructions
/// such as `--use-rendered-icc --assume-srgb` or two `--jxl-decoder=`
/// values therefore succeed with the last one. Not an endorsement.
#[cfg(feature = "cli")]
#[test]
fn cli_later_flags_silently_replace_earlier_ones_today() {
    let dir = std::path::PathBuf::from(env!("CARGO_TARGET_TMPDIR"));
    let out = dir.join(format!("silent-drops-{}.webp", std::process::id()));
    let run = |input: &str, flags: &[&str]| {
        let result = std::process::Command::new(env!("CARGO_BIN_EXE_ferrisma"))
            .arg(format!("{ROOT}/{input}"))
            .arg(&out)
            .args(["6", "4"])
            .args(flags)
            .output()
            .unwrap();
        assert!(
            result.status.success(),
            "{flags:?}: {}",
            String::from_utf8_lossy(&result.stderr)
        );
        String::from_utf8(result.stdout).unwrap()
    };
    let report = run(
        "tests/fixtures/sample.jxl",
        &[
            "--use-rendered-icc",
            "--jxl-decoder=jxl-oxide",
            "--jxl-decoder=libjxl-rs",
        ],
    );
    assert!(report.contains(" decoder=libjxl-rs "), "{report}");
    let report = run(
        "tests/fixtures/sample.jpg",
        &["--use-rendered-icc", "--assume-srgb"],
    );
    assert!(report.contains("color_signal=AssumedSrgb"), "{report}");
    let _ = std::fs::remove_file(out);
}
