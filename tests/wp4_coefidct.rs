//! WP4 differential tests of the coefficient buffer controller
//! (`jdcoefct.c`, `jdcoefct.h`) driving the inverse DCT.
//!
//! The probe `tools/oracle/probes/wp4/coefidct.c` sets up the IDCT, the
//! entropy decoder and the coefficient controller as `jdmaster.c` does, and
//! runs output passes as `jpeg_start_decompress`, `jpeg_finish_decompress`
//! and buffered-image mode's `jpeg_start_output` and `jpeg_finish_output`
//! do, with a zeroed sample array per component in place of the main
//! controller's buffer. It prints every message, a hash of all output rows
//! after each iMCU row of every pass, every result of `consume_input` and
//! `decompress_data`, the final state, the memory pools (which show the C
//! sizes of the controller, its virtual arrays and the latch) and the error.
//! `support/wp4.rs` runs the same sequence on the port; each case compares
//! both outputs exactly.
//!
//! The inputs are upstream's test images and the encoder variants of the
//! WP3 tests, in single-pass and multi-pass modes, with every DCT method,
//! scaled sizes per component, unneeded components, cropping, block
//! smoothing, a change of method between buffered passes, suspending input,
//! truncations and byte mutations.

#[path = "support/oracle.rs"]
mod oracle;
#[path = "support/wp2.rs"]
mod wp2;
#[path = "support/wp4.rs"]
mod wp4;

use std::process::Command;

use oracle::{Fields, Oracle};

/// Runs `args` on `data` with the probe and the port and compares the output.
fn compare(oracle: &Oracle, args: &[&str], data: &[u8], label: &str) -> Fields {
    let expected = oracle
        .probe("wp4", "coefidct")
        .run(args, data)
        .success()
        .fields();
    let actual = wp4::run_coefidct(args, data);
    if actual != expected {
        let first = actual
            .0
            .iter()
            .zip(&expected.0)
            .position(|(a, e)| a != e)
            .unwrap_or(actual.0.len().min(expected.0.len()));
        let (port, c) = (actual.0.get(first), expected.0.get(first));
        if let (Some((key, port)), Some((_, c))) = (port, c)
            && key == "rows"
        {
            let row = port.split(',').zip(c.split(',')).position(|(a, e)| a != e);
            panic!("{label} {args:?}: field {first}: iMCU row {row:?} differs");
        }
        panic!(
            "{label} {args:?}: outputs differ at field {first}\n  port:   {port:?}\n  oracle: {c:?}"
        );
    }
    expected
}

fn image(name: &str) -> Vec<u8> {
    std::fs::read(oracle::test_image(name)).unwrap()
}

/// Encodes `testorig.ppm` with the oracle's `cjpeg` and `switches`.
fn encode(oracle: &Oracle, switches: &[&str]) -> Vec<u8> {
    // One file per call: tests run in parallel and each removes its own.
    static CALLS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    let call = CALLS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let output =
        std::env::temp_dir().join(format!("libjpeg-rs-wp4-{}-{call}.jpg", std::process::id()));
    let status = Command::new(oracle.tool("cjpeg"))
        .args(switches)
        .arg("-outfile")
        .arg(&output)
        .arg(oracle::test_image("testorig.ppm"))
        .status()
        .unwrap();
    assert!(status.success(), "cjpeg {switches:?}");
    let data = std::fs::read(&output).unwrap();
    let _ = std::fs::remove_file(&output);
    data
}

/// The encoder variants: `cjpeg` switches.
const VARIANTS: &[&[&str]] = &[
    &[],
    &["-optimize"],
    &["-progressive"],
    &["-arithmetic"],
    &["-progressive", "-arithmetic"],
    &["-restart", "1"],
    &["-restart", "3B"],
    &["-progressive", "-restart", "1"],
    &["-arithmetic", "-restart", "2"],
    &["-progressive", "-arithmetic", "-restart", "1"],
    &["-sample", "1x1"],
    &["-sample", "2x1"],
    &["-sample", "1x2", "-progressive"],
    &["-sample", "4x1,1x1,1x1"],
    &["-sample", "3x2", "-arithmetic"],
    &["-grayscale"],
    &["-grayscale", "-progressive"],
    &["-grayscale", "-arithmetic", "-progressive"],
    &["-rgb", "-optimize"],
    &["-quality", "100"],
    &["-quality", "100", "-progressive"],
    &["-quality", "100", "-arithmetic"],
    &["-quality", "100", "-progressive", "-arithmetic"],
    &["-baseline", "-quality", "5"],
    &["-quality", "5", "-progressive"],
    &["-precision", "12"],
    &["-precision", "12", "-progressive"],
    &["-precision", "12", "-arithmetic"],
    &["-precision", "12", "-progressive", "-arithmetic"],
];

fn variants(oracle: &Oracle) -> Vec<(String, Vec<u8>)> {
    let mut variants: Vec<(String, Vec<u8>)> = VARIANTS
        .iter()
        .map(|switches| (switches.join(" "), encode(oracle, switches)))
        .collect();
    let scans = oracle::test_image("test.scan");
    let scans = scans.to_str().unwrap();
    variants.push((
        "-scans test.scan".to_owned(),
        encode(oracle, &["-scans", scans]),
    ));
    variants.push((
        "-scans test.scan -arithmetic".to_owned(),
        encode(oracle, &["-scans", scans, "-arithmetic"]),
    ));
    variants
}

/// One part of a JPEG stream.
#[derive(Clone, Debug, PartialEq, Eq)]
enum Part {
    /// A marker without parameters (SOI, EOI, RSTn).
    Standalone(u8),
    /// A marker segment and its payload.
    Segment(u8, Vec<u8>),
    /// Entropy-coded data, up to the next marker other than RSTn.
    Entropy(Vec<u8>),
}

/// Splits a whole stream into parts. RST markers inside entropy-coded data
/// are parts of their own.
fn parts(jpeg: &[u8]) -> Vec<Part> {
    let mut parts = Vec::new();
    let mut at = 0;
    while at < jpeg.len() {
        assert_eq!(jpeg[at], 0xFF, "marker expected at {at}");
        let marker = jpeg[at + 1];
        at += 2;
        if marker == 0xD8 || marker == 0xD9 || (0xD0..=0xD7).contains(&marker) {
            parts.push(Part::Standalone(marker));
        } else {
            let length = usize::from(u16::from_be_bytes([jpeg[at], jpeg[at + 1]]));
            parts.push(Part::Segment(marker, jpeg[at + 2..at + length].to_vec()));
            at += length;
        }
        if marker == 0xDA || (0xD0..=0xD7).contains(&marker) {
            let start = at;
            while at < jpeg.len() && !(jpeg[at] == 0xFF && at + 1 < jpeg.len() && jpeg[at + 1] != 0)
            {
                at += 1;
            }
            parts.push(Part::Entropy(jpeg[start..at].to_vec()));
        }
    }
    parts
}

fn join(parts: &[Part]) -> Vec<u8> {
    let mut bytes = Vec::new();
    for part in parts {
        match part {
            Part::Standalone(marker) => bytes.extend([0xFF, *marker]),
            Part::Segment(marker, payload) => {
                bytes.extend([0xFF, *marker]);
                bytes.extend(u16::try_from(payload.len() + 2).unwrap().to_be_bytes());
                bytes.extend(payload);
            }
            Part::Entropy(data) => bytes.extend(data),
        }
    }
    bytes
}

/// The byte ranges of the entropy-coded data, RST markers included.
fn entropy_ranges(jpeg: &[u8]) -> Vec<std::ops::Range<usize>> {
    let mut ranges = Vec::new();
    let mut at = 0;
    for part in parts(jpeg) {
        let length = join(std::slice::from_ref(&part)).len();
        if let Part::Entropy(_) = part {
            ranges.push(at..at + length);
        }
        at += length;
    }
    ranges
}

/// `count` positions spread over the entropy-coded data.
fn entropy_positions(jpeg: &[u8], count: usize) -> Vec<usize> {
    let positions: Vec<usize> = entropy_ranges(jpeg).into_iter().flatten().collect();
    (0..count)
        .map(|index| positions[index * positions.len() / count])
        .collect()
}

/// Whether a variant has 8-bit samples, which the port decodes.
fn eight_bit(label: &str) -> bool {
    !label.contains("-precision 12")
}

/// Whether a variant has more than one scan.
fn multiscan(label: &str) -> bool {
    label.contains("progressive") || label.contains("test.scan")
}

/// The output modes compared for every input: single-pass or full-image
/// decoding as `jpeg_start_decompress` does it, buffered-image mode with and
/// without block smoothing, and the other DCT methods.
const MODES: &[&[&str]] = &[
    &[],
    &["-buffered"],
    &["-buffered", "-nosmooth"],
    &["-dct", "ifast"],
    &["-dct", "float"],
    &["-buffered", "-dct", "float", "-dct2", "ifast"],
];

#[test]
fn test_images_match_the_oracle() {
    let Some(oracle) = Oracle::locate("test_images_match_the_oracle") else {
        return;
    };
    for name in ["testorig.jpg", "testimgari.jpg", "testimgint.jpg"] {
        for args in MODES {
            let fields = compare(&oracle, args, &image(name), name);
            assert_eq!(fields.get("error"), Some("0"), "{name} {args:?}");
        }
    }
    /* 12-bit samples fail in jinit_inverse_dct, as they do upstream without
     * the 12-bit pipeline (ADR 0005). */
    let fields = compare(&oracle, &[], &image("monkey12.jpg"), "monkey12.jpg");
    assert_eq!(fields.get("error"), Some("15"), "JERR_BAD_PRECISION");
}

#[test]
fn encoder_variants_match_the_oracle() {
    let Some(oracle) = Oracle::locate("encoder_variants_match_the_oracle") else {
        return;
    };
    for (label, data) in variants(&oracle) {
        for args in MODES {
            let fields = compare(&oracle, args, &data, &label);
            if eight_bit(&label) {
                assert_eq!(fields.get("error"), Some("0"), "{label} {args:?}");
            }
        }
    }
}

#[test]
fn scaled_sizes_match_the_oracle() {
    let Some(oracle) = Oracle::locate("scaled_sizes_match_the_oracle") else {
        return;
    };
    for (label, data) in variants(&oracle)
        .into_iter()
        .filter(|(label, _)| eight_bit(label))
    {
        for size in 1..=16 {
            let size = size.to_string();
            compare(&oracle, &["-scaled", &size], &data, &label);
            if multiscan(&label) {
                compare(&oracle, &["-scaled", &size, "-buffered"], &data, &label);
            }
        }
        for args in [
            &[
                "-scaledc", "0", "16", "-scaledc", "1", "4", "-scaledc", "2", "2",
            ][..],
            &[
                "-scaledc", "0", "1", "-scaledc", "1", "13", "-scaledc", "2", "7",
            ],
            &["-scaledc", "0", "5", "-scaled", "9", "-dct", "ifast"],
            &["-scaledc", "1", "3", "-buffered", "-dct", "float"],
        ] {
            compare(&oracle, args, &data, &label);
        }
    }
}

#[test]
fn unneeded_components_and_cropping_match_the_oracle() {
    let Some(oracle) = Oracle::locate("unneeded_components_and_cropping_match_the_oracle") else {
        return;
    };
    for (label, data) in variants(&oracle)
        .into_iter()
        .filter(|(label, _)| eight_bit(label))
    {
        for args in [
            &["-unneeded", "0"][..],
            &["-unneeded", "1", "-unneeded", "2"],
            &["-unneeded", "2", "-buffered"],
            &["-crop", "0", "0"],
            &["-crop", "1", "3"],
            &["-crop", "2", "5", "-buffered"],
            &["-crop", "1", "4", "-scaled", "2", "-buffered", "-nosmooth"],
            &["-crop", "3", "3", "-unneeded", "0", "-scaled", "11"],
        ] {
            compare(&oracle, args, &data, &label);
        }
    }
}

#[test]
fn errors_match_the_oracle() {
    let Some(oracle) = Oracle::locate("errors_match_the_oracle") else {
        return;
    };
    let data = image("testorig.jpg");
    for (args, code) in [
        (&["-scaled", "17"][..], "7"),
        (&["-scaledc", "1", "18"], "7"),
        (&["-dct", "7"], "48"),
        (&["-dct", "7", "-buffered"], "48"),
    ] {
        let fields = compare(&oracle, args, &data, "testorig.jpg");
        assert_eq!(fields.get("error"), Some(code), "{args:?}");
    }
    /* A negative size wraps the probe's buffer width, which alloc_sarray
     * rejects before start_pass sees the size. */
    let fields = compare(&oracle, &["-scaledc", "1", "-3"], &data, "testorig.jpg");
    assert_ne!(fields.get("error"), Some("0"));
}

#[test]
fn suspension_matches_the_oracle() {
    let Some(oracle) = Oracle::locate("suspension_matches_the_oracle") else {
        return;
    };
    let mut inputs = vec![("testorig.jpg".to_owned(), image("testorig.jpg"))];
    inputs.extend(variants(&oracle).into_iter().filter(|(label, _)| {
        [
            "-progressive",
            "-arithmetic",
            "-restart 1",
            "-progressive -arithmetic",
            "-scans test.scan",
        ]
        .contains(&label.as_str())
    }));
    for (label, data) in inputs {
        for step in ["1", "7", "64", "1000"] {
            compare(&oracle, &["-suspend", step], &data, &label);
            compare(&oracle, &["-suspend", step, "-buffered"], &data, &label);
        }
    }
}

#[test]
fn truncations_match_the_oracle() {
    let Some(oracle) = Oracle::locate("truncations_match_the_oracle") else {
        return;
    };
    for (label, data) in variants(&oracle)
        .into_iter()
        .filter(|(label, _)| eight_bit(label))
    {
        for cut in entropy_positions(&data, 12) {
            let label = format!("{label} cut at {cut}");
            compare(&oracle, &[], &data[..cut], &label);
            compare(&oracle, &["-buffered"], &data[..cut], &label);
        }
    }
}

#[test]
fn mutations_match_the_oracle() {
    let Some(oracle) = Oracle::locate("mutations_match_the_oracle") else {
        return;
    };
    for (label, data) in variants(&oracle)
        .into_iter()
        .filter(|(label, _)| eight_bit(label))
    {
        for (index, position) in entropy_positions(&data, 8).into_iter().enumerate() {
            for value in [data[position] ^ 0x55, 0xFF, 0x00, 0xD9] {
                let mut mutated = data.clone();
                mutated[position] = value;
                if index % 2 == 1 && position + 1 < mutated.len() {
                    mutated[position + 1] = 0x00;
                }
                let label = format!("{label} byte {position} = {value:#04x}");
                compare(&oracle, &[], &mutated, &label);
                compare(&oracle, &["-buffered"], &mutated, &label);
            }
        }
    }
}
