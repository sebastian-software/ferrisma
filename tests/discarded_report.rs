//! `ProcessingReport::discarded` (ADR-0042 D2): every class is reported for
//! a real fixture that has the thing and absent for one that does not, so a
//! class is neither missed nor reported unconditionally. Where a few bytes
//! are patched, the helper says which and why. Run with every adapter:
//! `cargo test -p ferrisma --features cli --test discarded_report`.
#![cfg(all(
    feature = "jpeg-read",
    feature = "webp-read",
    feature = "webp-write",
    feature = "jpeg-xl-read",
    feature = "icc"
))]

use ferrisma::pipeline::{
    Discarded, ImageFormat, InputProfilePolicy, OutputFormat, TransformOptions, transcode,
};

const ROOT: &str = env!("CARGO_MANIFEST_DIR");
const AVIF_DATA: &str = "codecs/libavif-rs/upstream/libavif/tests/data/";
const JXL_DATA: &str = "codecs/libjxl-rs/tests/data/";

const ASSUME: InputProfilePolicy = InputProfilePolicy::AssumeSrgbIfMissing;
const RENDERED: InputProfilePolicy = InputProfilePolicy::UseRenderedIcc;

fn read(path: &str) -> Vec<u8> {
    std::fs::read(format!("{ROOT}/{path}")).unwrap_or_else(|error| panic!("{path}: {error}"))
}

fn options(policy: InputProfilePolicy) -> TransformOptions {
    TransformOptions {
        output_format: OutputFormat::WebP,
        output_width: 6,
        output_height: 4,
        profile_policy: policy,
        transform_threads: 1,
        ..TransformOptions::default()
    }
}

/// The discarded classes of one successful transcode.
#[track_caller]
fn discarded(bytes: &[u8], format: ImageFormat, policy: InputProfilePolicy) -> Vec<Discarded> {
    transcode(bytes, format, options(policy))
        .unwrap_or_else(|error| panic!("{format:?} under {policy:?}: {error}"))
        .report
        .discarded
        .iter()
        .collect()
}

fn jpeg(path: &str, policy: InputProfilePolicy) -> Vec<Discarded> {
    discarded(&read(path), ImageFormat::Jpeg, policy)
}

fn webp(bytes: &[u8], policy: InputProfilePolicy) -> Vec<Discarded> {
    discarded(bytes, ImageFormat::WebP, policy)
}

/// The JPEG XL reader's classes. (Until 2026-11-15 this also asserted that
/// the jxl-oxide reader reported the same ones.)
fn jxl(path: &str, policy: InputProfilePolicy) -> Vec<Discarded> {
    discarded(&read(path), ImageFormat::JpegXl, policy)
}

use Discarded::{
    Exif, GainMap, HiddenColor, IccProfile, JpegReconstructionData, OtherMetadata,
    PrecisionReduced, Preview, UnknownContainerData, Xmp,
};

// ---------------------------------------------------------------------------
// JPEG: application segments.

#[test]
fn an_untagged_jfif_jpeg_discards_nothing() {
    assert_eq!(jpeg("tests/fixtures/sample.jpg", ASSUME), []);
}

#[test]
fn jpeg_exif_is_reported_even_when_its_orientation_is_applied() {
    assert_eq!(
        jpeg("tests/fixtures/orientation/jpeg-6.jpg", ASSUME),
        [Exif]
    );
}

#[test]
fn jpeg_xmp_icc_and_other_metadata_are_reported() {
    // EXIF, XMP, a 300 dpi JFIF density and an ICC profile.
    assert_eq!(
        jpeg(&format!("{AVIF_DATA}paris_exif_xmp_icc.jpg"), RENDERED),
        [Exif, Xmp, OtherMetadata, IccProfile]
    );
    // Extended XMP, the same density.
    assert_eq!(
        jpeg(&format!("{AVIF_DATA}paris_extended_xmp.jpg"), ASSUME),
        [Xmp, OtherMetadata]
    );
    // The density alone (a 300 dpi JFIF header).
    assert_eq!(
        jpeg("tests/fixtures/decoder-work/paris-420.jpg", ASSUME),
        [OtherMetadata]
    );
}

#[test]
fn jpeg_gain_maps_iptc_and_comments_are_reported() {
    // MPF gain map, EXIF, XMP, IPTC (APP13), ICC.
    assert_eq!(
        jpeg(&format!("{AVIF_DATA}seine_sdr_gainmap_srgb.jpg"), RENDERED),
        [GainMap, Exif, Xmp, OtherMetadata, IccProfile]
    );
    // Apple's MPF gain map, EXIF, a COM segment, ICC.
    assert_eq!(
        jpeg(&format!("{AVIF_DATA}apple_gainmap_new.jpg"), RENDERED),
        [GainMap, Exif, OtherMetadata, IccProfile]
    );
}

// ---------------------------------------------------------------------------
// WebP: RIFF chunks.

#[test]
fn webp_chunks_are_reported() {
    let plain = read("tests/fixtures/webp/reference-lossy.webp");
    assert_eq!(webp(&plain, ASSUME), []);
    let with_exif = read("tests/fixtures/decoder-work/paris-o5-lossy.webp");
    assert_eq!(webp(&with_exif, ASSUME), [Exif]);
    // The same file with its `EXIF` chunk (offset 1888) renamed to an
    // unknown tag, which the container lets a reader ignore.
    let mut unknown = with_exif.clone();
    assert_eq!(&unknown[1888..1892], b"EXIF");
    unknown[1888..1892].copy_from_slice(b"XYZW");
    assert_eq!(webp(&unknown, ASSUME), [UnknownContainerData]);
    // The engine's own WebP output carries an sRGB `ICCP` chunk.
    let output = transcode(&plain, ImageFormat::WebP, options(ASSUME))
        .unwrap()
        .bytes;
    assert_eq!(webp(&output, RENDERED), [IccProfile]);
}

// ---------------------------------------------------------------------------
// JPEG XL: container boxes, preview, precision.

#[test]
fn jpeg_xl_boxes_are_reported() {
    assert_eq!(jxl("tests/fixtures/sample.jxl", RENDERED), []);
    assert_eq!(
        jxl(&format!("{JXL_DATA}decode-test/1x1_exif_xmp.jxl"), RENDERED),
        [Exif, Xmp, JpegReconstructionData]
    );
}

#[test]
fn jpeg_xl_preview_is_reported() {
    // (The former jxl-oxide reader rejected this sequence fixture as
    // malformed.)
    let preview = read(&format!(
        "{JXL_DATA}decoder-geometry/sequence-preview-main-0-0-0.jxl"
    ));
    assert_eq!(
        discarded(&preview, ImageFormat::JpegXl, RENDERED,),
        [Preview]
    );
}

#[test]
fn jpeg_xl_precision_above_eight_bits_is_reported() {
    assert_eq!(
        jxl(&format!("{JXL_DATA}modular-palette16.jxl"), RENDERED),
        [PrecisionReduced]
    );
    assert_eq!(jxl(&format!("{JXL_DATA}vardct-rgb8.jxl"), RENDERED), []);
}

#[test]
fn hidden_color_under_zero_alpha_is_reported() {
    // Fully transparent pixels with a nonzero color in a lossless file.
    assert_eq!(
        jxl("tests/fixtures/jxl-lossless/rgba-37x19.jxl", RENDERED),
        [HiddenColor]
    );
}

// ---------------------------------------------------------------------------
// AVIF: items and properties.

#[cfg(feature = "avif-read")]
mod avif {
    use super::*;

    const STRICT: InputProfilePolicy = InputProfilePolicy::Strict;

    const ANDROID: &str = "codecs/libavif-rs/upstream/libavif/android_jni/avifandroidjni/src/androidTest/assets/avif/";

    fn avif(bytes: &[u8], policy: InputProfilePolicy) -> Vec<Discarded> {
        discarded(bytes, ImageFormat::Avif, policy)
    }

    fn file(name: &str) -> Vec<u8> {
        read(&format!("{AVIF_DATA}{name}"))
    }

    #[test]
    fn a_plain_srgb_avif_discards_nothing() {
        assert_eq!(avif(&file("colors_sdr_srgb.avif"), STRICT), []);
        assert_eq!(
            avif(&read("tests/fixtures/orientation/avif-1.avif"), ASSUME),
            []
        );
    }

    #[test]
    fn avif_metadata_items_icc_and_gain_maps_are_reported() {
        assert_eq!(
            avif(&file("paris_icc_exif_xmp.avif"), RENDERED),
            [Exif, Xmp, IccProfile]
        );
        assert_eq!(
            avif(&file("seine_sdr_gainmap_srgb.avif"), STRICT),
            [GainMap, Exif, Xmp]
        );
    }

    #[test]
    fn avif_properties_the_engine_does_not_interpret_are_reported() {
        // `1234`, `abcd` and a `uuid` property (and hidden color under its
        // transparent corners).
        assert_eq!(
            avif(&file("circle_custom_properties.avif"), ASSUME),
            [UnknownContainerData, HiddenColor]
        );
        // `pasp` (pixel aspect ratio) at 8 bits; the 10-bit sibling also
        // reduces precision.
        let eight = read(&format!("{ANDROID}fox.profile0.8bpc.yuv420.avif"));
        assert_eq!(avif(&eight, ASSUME), [UnknownContainerData]);
        let ten = read(&format!("{ANDROID}fox.profile0.10bpc.yuv420.avif"));
        assert_eq!(avif(&ten, ASSUME), [UnknownContainerData, PrecisionReduced]);
    }

    /// The output depth follows the input's (owner decision of 2026-11-15,
    /// ADR-0021 amendment), as `avifenc` writes its input's depth: a 10-bit
    /// AVIF becomes a 10-bit AVIF with 10-bit content, and nothing is
    /// reported; an explicit depth below the input's is written and
    /// reported. (Until that decision the default depth was a fixed 8 and
    /// this test, `deeper_input_at_the_default_avif_depth_reports_precision_reduced`,
    /// pinned the reduction; the expectation changed by owner decision.)
    #[cfg(feature = "avif-write")]
    #[test]
    fn deeper_input_keeps_its_depth_and_an_explicit_lower_depth_is_reported() {
        use ferrisma::pipeline::AvifOptions;
        assert_eq!(AvifOptions::default().depth, None);
        let to_avif = |bytes: &[u8], depth: Option<u8>| {
            let output = transcode(
                bytes,
                ImageFormat::Avif,
                TransformOptions {
                    output_format: OutputFormat::Avif,
                    avif: AvifOptions {
                        depth,
                        ..AvifOptions::default()
                    },
                    ..options(ASSUME)
                },
            )
            .unwrap();
            assert_eq!(output.report.encoder_backend, "libavif-rs/aom");
            output.report.discarded.iter().collect::<Vec<_>>()
        };
        let ten = read(&format!("{ANDROID}fox.profile0.10bpc.yuv420.avif"));
        assert!(!to_avif(&ten, None).contains(&PrecisionReduced));
        assert!(!to_avif(&ten, Some(12)).contains(&PrecisionReduced));
        assert!(to_avif(&ten, Some(8)).contains(&PrecisionReduced));
        let eight = read(&format!("{ANDROID}fox.profile0.8bpc.yuv420.avif"));
        assert!(!to_avif(&eight, None).contains(&PrecisionReduced));
        assert!(!to_avif(&eight, Some(10)).contains(&PrecisionReduced));
    }

    /// Insert a `prem` reference (color item 1 to alpha item 2) into the
    /// `iref` of libavif's `abc_color_irot_alpha_irot.avif`, which libavif
    /// reads as premultiplied alpha. The 14 inserted bytes grow `iref` and
    /// `meta` and move `mdat`, so the two `iloc` extent offsets (version 0,
    /// 4-byte offsets and lengths, no base offset) move by 14 as well.
    fn with_premultiplied_alpha(mut bytes: Vec<u8>) -> Vec<u8> {
        let u32_at =
            |bytes: &[u8], at: usize| u32::from_be_bytes(bytes[at..at + 4].try_into().unwrap());
        let find = |bytes: &[u8], kind: &[u8]| {
            bytes.windows(4).position(|window| window == kind).unwrap() - 4
        };
        let prem = [
            &14_u32.to_be_bytes()[..],
            b"prem",
            &1_u16.to_be_bytes(),
            &1_u16.to_be_bytes(),
            &2_u16.to_be_bytes(),
        ]
        .concat();
        let (meta, iref, iloc) = (
            find(&bytes, b"meta"),
            find(&bytes, b"iref"),
            find(&bytes, b"iloc"),
        );
        assert!(meta < iloc && iloc < iref);
        assert_eq!(&bytes[iloc + 8..iloc + 16], &[0, 0, 0, 0, 0x44, 0, 0, 2]);
        for item in 0..2 {
            // item_ID, data_reference_index, extent_count = 1, offset, length.
            let entry = iloc + 16 + item * 14;
            assert_eq!(&bytes[entry + 4..entry + 6], &[0, 1]);
            let offset = u32_at(&bytes, entry + 6) + 14;
            bytes[entry + 6..entry + 10].copy_from_slice(&offset.to_be_bytes());
        }
        let iref_size = u32_at(&bytes, iref);
        let end = iref + iref_size as usize;
        bytes.splice(end..end, prem);
        bytes[iref..iref + 4].copy_from_slice(&(iref_size + 14).to_be_bytes());
        let meta_size = u32_at(&bytes, meta);
        bytes[meta..meta + 4].copy_from_slice(&(meta_size + 14).to_be_bytes());
        bytes
    }

    #[test]
    fn premultiplied_avif_alpha_is_reported_as_unpremultiplied() {
        let straight = file("abc_color_irot_alpha_irot.avif");
        assert_eq!(avif(&straight, ASSUME), []);
        assert_eq!(
            avif(&with_premultiplied_alpha(straight), ASSUME),
            [Discarded::Unpremultiplied]
        );
    }
}

// ---------------------------------------------------------------------------
// The command line.

#[cfg(feature = "cli")]
#[test]
fn the_cli_report_line_names_the_discarded_classes() {
    let dir = std::path::PathBuf::from(env!("CARGO_TARGET_TMPDIR"));
    let out = dir.join(format!("discarded-{}.webp", std::process::id()));
    let run = |input: &str| {
        let result = std::process::Command::new(env!("CARGO_BIN_EXE_ferrisma"))
            .arg(format!("{ROOT}/{input}"))
            .arg(&out)
            .args(["6", "4", "--use-rendered-icc"])
            .output()
            .unwrap();
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        String::from_utf8(result.stdout).unwrap()
    };
    let report = run(&format!("{AVIF_DATA}paris_exif_xmp_icc.jpg"));
    assert!(
        report.contains(" discarded=exif,xmp,other-metadata,icc-profile\n"),
        "{report}"
    );
    let report = run("tests/fixtures/sample.jxl");
    assert!(report.contains(" discarded=none\n"), "{report}");
    // No strict mode (owner decision of 2026-11-15, ADR-0042): a script
    // tests the key itself. It is the last key of the `report:` line, and
    // its value is `none` exactly when nothing was left out.
    let discarded = |report: &str| {
        let line = report
            .lines()
            .find(|line| line.starts_with("report: "))
            .expect("one report line");
        let last = line.split_whitespace().last().unwrap().to_owned();
        last.strip_prefix("discarded=")
            .expect("discarded= is the last key")
            .to_owned()
    };
    assert_eq!(discarded(&report), "none");
    assert_eq!(
        discarded(&run(&format!("{AVIF_DATA}paris_exif_xmp_icc.jpg"))),
        "exif,xmp,other-metadata,icc-profile"
    );
    let _ = std::fs::remove_file(out);
}
