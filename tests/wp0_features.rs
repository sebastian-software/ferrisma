//! WP0 example differential test: the oracle pattern end to end.
//!
//! The probe `tools/oracle/probes/wp0/webp_get_features.c` prints what the
//! pinned C library's `WebPGetFeatures` returns for one input. WP0 translates
//! no codec code, so these tests exercise the harness itself: they run the
//! probe over every test vector and check its output against expectations
//! derived by hand from the file bytes, never from the oracle.
//!
//! A package's differential test follows the same steps and then compares the
//! oracle's fields with the fields of its Rust translation for the same input:
//!
//! ```text
//! let oracle = probe.run(&[&path], &[]).success().fields();
//! let rust = Fields::new().with("status", status).with("width", width) /* ... */;
//! assert_eq!(rust, oracle, "{}", path.display());
//! ```

#[path = "support/oracle.rs"]
mod oracle;

use oracle::{Fields, Oracle};

const FIELDS: [&str; 6] = [
    "status",
    "width",
    "height",
    "has_alpha",
    "has_animation",
    "format",
];

/// `alpha_color_cache.webp`, the first vector by name, checked by hand:
/// bytes 12..20 are the `VP8X` chunk header with a 10-byte payload; payload
/// byte 0 is `0x10` (alpha flag set, animation flag clear); bytes 24..27 are
/// `4b 02 00`, canvas width minus one = 587; bytes 27..30 are `60 00 00`,
/// canvas height minus one = 96. The image chunk at offset 1670 is `VP8 `
/// (lossy, format 1) after an `ALPH` chunk; its frame header `10 11 00 9d 01
/// 2a 4c 02 61 00` is a key frame of 0x24c = 588 by 0x61 = 97 pixels.
fn first_vector_expectation() -> Fields {
    Fields::new()
        .with("status", 0) // VP8_STATUS_OK
        .with("width", 588)
        .with("height", 97)
        .with("has_alpha", 1)
        .with("has_animation", 0)
        .with("format", 1)
}

#[test]
fn webp_get_features_over_every_vector() {
    let Some(oracle) = Oracle::locate("webp_get_features_over_every_vector") else {
        return;
    };
    let probe = oracle.probe("wp0", "webp_get_features");
    let vectors = oracle::test_vectors();
    // 131 vectors at the pinned test-data revision (docs/PORT_STATUS.md).
    assert_eq!(vectors.len(), 131, "test vector count changed");
    assert!(vectors[0].ends_with("alpha_color_cache.webp"));

    for (index, path) in vectors.iter().enumerate() {
        let fields = probe.run(&[path], &[]).success().fields();
        assert_eq!(fields.keys(), FIELDS, "{}", path.display());
        if index == 0 {
            assert_eq!(fields, first_vector_expectation(), "{}", path.display());
        }
        // Every pinned vector is a valid still or animated WebP file.
        assert_eq!(fields.int("status"), 0, "{}", path.display());
        assert!(
            fields.int("width") > 0 && fields.int("height") > 0,
            "{}",
            path.display()
        );
        assert!(matches!(fields.int("format"), 0..=2), "{}", path.display());
    }
}

#[test]
fn webp_get_features_reads_stdin_and_keeps_partial_state() {
    let Some(oracle) = Oracle::locate("webp_get_features_reads_stdin_and_keeps_partial_state")
    else {
        return;
    };
    let probe = oracle.probe("wp0", "webp_get_features");
    let path = oracle::test_data().join("alpha_color_cache.webp");
    let bytes = std::fs::read(&path).expect("first test vector");

    let from_stdin = probe.run::<&str>(&[], &bytes).success().fields();
    assert_eq!(from_stdin, first_vector_expectation());

    // The RIFF header and the VP8X chunk header without its payload: the
    // features are reset before parsing, and parsing stops for lack of data.
    let truncated = probe.run::<&str>(&[], &bytes[..20]).success().fields();
    let expected = Fields::new()
        .with("status", 7) // VP8_STATUS_NOT_ENOUGH_DATA
        .with("width", 0)
        .with("height", 0)
        .with("has_alpha", 0)
        .with("has_animation", 0)
        .with("format", 0);
    assert_eq!(truncated, expected);
}
