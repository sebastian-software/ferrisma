//! WP0 example differential test: the oracle pattern end to end.
//!
//! The probe `tools/oracle/probes/wp0/read_header.c` prints what the pinned C
//! library's `jpeg_read_header` leaves in the decompression object. WP0
//! translates no decoder unit, so these tests exercise the harness itself:
//! they check the probe's output against expectations derived by hand from
//! the file bytes and from `jerror.h`, `jpeglib.h` and `jpegint.h`, never from
//! the oracle.
//!
//! A package's differential test follows the same steps and then compares the
//! oracle's fields with the fields of its Rust translation for the same input:
//!
//! ```text
//! let oracle = probe.run(&[&path], &[]).success().fields();
//! let rust = Fields::new().with("status", status).with("image_width", width) /* ... */;
//! assert_eq!(rust, oracle, "{}", path.display());
//! ```

#[path = "support/oracle.rs"]
mod oracle;

use oracle::{Fields, Oracle};

// jpegint.h: DSTATE_START, DSTATE_INHEADER, DSTATE_READY.
const DSTATE_START: i64 = 200;
const DSTATE_INHEADER: i64 = 201;
const DSTATE_READY: i64 = 202;
// jpeglib.h J_COLOR_SPACE: JCS_RGB and JCS_YCbCr.
const JCS_RGB: i64 = 2;
const JCS_YCBCR: i64 = 3;
// Positions in jerror.h's JMESSAGE list as compiled with JPEG_LIB_VERSION 62
// (jconfig.h), which leaves out the entries guarded by JPEG_LIB_VERSION >= 70.
const JERR_INPUT_EMPTY: i64 = 42;
const JERR_NO_IMAGE: i64 = 51;
const JWRN_JPEG_EOF: i64 = 120;

/// `testorig.jpg`, checked by hand: the APP0 segment at offset 2 is `JFIF\0`
/// version `01 01`, units `00`, densities `0001` and `0001`. The SOF0 segment
/// at offset 158 has precision 8, height `0095` = 149, width `00e3` = 227 and
/// three components: id 1 with sampling `22` and table 0, ids 2 and 3 with
/// sampling `11` and table 1. A JFIF file with three components is YCbCr,
/// which `jpeg_read_header` sets up for RGB output.
fn testorig_expectation() -> Fields {
    Fields::new()
        .with("status", 1) // JPEG_HEADER_OK
        .with("global_state", DSTATE_READY)
        .with("image_width", 227)
        .with("image_height", 149)
        .with("num_components", 3)
        .with("jpeg_color_space", JCS_YCBCR)
        .with("out_color_space", JCS_RGB)
        .with("data_precision", 8)
        .with("progressive_mode", 0)
        .with("arith_code", 0)
        .with("restart_interval", 0)
        .with("saw_JFIF_marker", 1)
        .with("JFIF_version", "1.01")
        .with("density_unit", 0)
        .with("X_density", 1)
        .with("Y_density", 1)
        .with("saw_Adobe_marker", 0)
        .with("max_h_samp_factor", 2)
        .with("max_v_samp_factor", 2)
        .with("component0", "id:1,h:2,v:2,tq:0")
        .with("component1", "id:2,h:1,v:1,tq:1")
        .with("component2", "id:3,h:1,v:1,tq:1")
        .with("error", 0)
        .with("num_warnings", 0)
        .with("warnings", "")
}

#[test]
fn read_header_of_the_test_images() {
    let Some(oracle) = Oracle::locate("read_header_of_the_test_images") else {
        return;
    };
    let probe = oracle.probe("wp0", "read_header");

    let testorig = oracle::test_image("testorig.jpg");
    let fields = probe.run(&[&testorig], &[]).success().fields();
    assert_eq!(fields, testorig_expectation(), "{}", testorig.display());

    // testimgint.jpg has the same headers as testorig.jpg (SOF0 at offset 158).
    let testimgint = oracle::test_image("testimgint.jpg");
    let fields = probe.run(&[&testimgint], &[]).success().fields();
    assert_eq!(fields, testorig_expectation(), "{}", testimgint.display());

    // testimgari.jpg: SOF9 (arithmetic, sequential) at offset 158, same layout.
    let testimgari = oracle::test_image("testimgari.jpg");
    let fields = probe.run(&[&testimgari], &[]).success().fields();
    let mut expected = testorig_expectation();
    for (key, value) in &mut expected.0 {
        if key == "arith_code" {
            *value = "1".to_owned();
        }
    }
    assert_eq!(fields, expected, "{}", testimgari.display());

    // monkey12.jpg: SOF1 at offset 3200 with precision 0c = 12, height 00e3 =
    // 227, width 0095 = 149. The 8-bit API still reads the header.
    let monkey12 = oracle::test_image("monkey12.jpg");
    let fields = probe.run(&[&monkey12], &[]).success().fields();
    assert_eq!(fields.int("status"), 1);
    assert_eq!(fields.int("data_precision"), 12);
    assert_eq!(fields.int("image_width"), 149);
    assert_eq!(fields.int("image_height"), 227);
    assert_eq!(fields.int("error"), 0);
}

#[test]
fn read_header_reads_stdin_and_reports_errors() {
    let Some(oracle) = Oracle::locate("read_header_reads_stdin_and_reports_errors") else {
        return;
    };
    let probe = oracle.probe("wp0", "read_header");
    let bytes = std::fs::read(oracle::test_image("testorig.jpg")).expect("testorig.jpg");

    let from_stdin = probe.run::<&str>(&[], &bytes).success().fields();
    assert_eq!(from_stdin, testorig_expectation());

    // SOI only: the memory source inserts a fake EOI after warning
    // JWRN_JPEG_EOF, and the marker reader then fails with JERR_NO_IMAGE in
    // the header state; jpeg_read_header never returns.
    let truncated = probe.run::<&str>(&[], &bytes[..2]).success().fields();
    assert_eq!(truncated.int("status"), -1);
    assert_eq!(truncated.int("global_state"), DSTATE_INHEADER);
    assert_eq!(truncated.int("error"), JERR_NO_IMAGE);
    assert_eq!(
        truncated.get("error_message"),
        Some("JPEG datastream contains no image")
    );
    assert_eq!(truncated.int("num_warnings"), 1);
    assert_eq!(
        truncated.get("warnings"),
        Some(JWRN_JPEG_EOF.to_string().as_str())
    );

    // No input at all: jpeg_mem_src rejects it before any state change.
    let empty = probe.run::<&str>(&[], &[]).success().fields();
    assert_eq!(empty.int("status"), -1);
    assert_eq!(empty.int("global_state"), DSTATE_START);
    assert_eq!(empty.int("error"), JERR_INPUT_EMPTY);
    assert_eq!(empty.get("error_message"), Some("Empty input file"));
    assert_eq!(empty.int("num_warnings"), 0);
}
