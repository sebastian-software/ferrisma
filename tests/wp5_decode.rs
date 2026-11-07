//! WP5 differential tests of upsampling, color conversion and the main and
//! postprocessing controllers: `jdsample.c`, `jdmerge.c`, `jdmrgext.c`,
//! `jdcolor.c`, `jdcolext.c`, `jdpostct.c` and `jdmainct.c`.
//!
//! The probe `tools/oracle/probes/wp5/decode.c` sets up the whole decoder as
//! `jdmaster.c`'s `master_selection` does (without color quantization),
//! starts the output pass as `jpeg_start_decompress` does and reads every
//! scanline through the main controller as `jpeg_read_scanlines` does, a
//! given number of rows per call. It prints every message, the output
//! dimensions and the choices made (merged upsampling, context rows, scaled
//! component sizes), a hash of every output row, every row count the main
//! controller returned, the final state, the memory pools (which show the C
//! sizes of every new module and buffer) and the error. Its rows equal
//! `djpeg`'s output for the upstream test images. `support/wp5.rs` runs the
//! same sequence on the port; each case compares both outputs exactly.
//!
//! The inputs are upstream's test images, the encoder variants of the
//! earlier packages, CMYK and YCCK images that the probe `wp5/cmyk` encodes
//! with the oracle, and streams cut and altered from them, decoded to every
//! output color space with and without fancy upsampling, at every scale,
//! with every DCT method, and with suspending input.

#[path = "support/oracle.rs"]
mod oracle;
#[path = "support/wp2.rs"]
mod wp2;
#[path = "support/wp4.rs"]
mod wp4;
#[path = "support/wp5.rs"]
mod wp5;

use std::io::Write;
use std::process::{Command, Stdio};

use oracle::{Fields, Oracle};

/// Runs `args` on `data` with the probe and the port and compares the output.
fn compare(oracle: &Oracle, args: &[&str], data: &[u8], label: &str) -> Fields {
    let expected = oracle
        .probe("wp5", "decode")
        .run(args, data)
        .success()
        .fields();
    let actual = wp5::run_rust(args, data);
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
            panic!("{label} {args:?}: output row {row:?} differs");
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
        std::env::temp_dir().join(format!("libjpeg-rs-wp5-{}-{call}.jpg", std::process::id()));
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

/// The 8-bit encoder variants.
fn eight_bit_variants(oracle: &Oracle) -> Vec<(String, Vec<u8>)> {
    variants(oracle)
        .into_iter()
        .filter(|(label, _)| eight_bit(label))
        .collect()
}

/// A four-component image of `testorig.ppm` from the probe `wp5/cmyk`.
fn four_component(oracle: &Oracle, args: &[&str]) -> Vec<u8> {
    let mut child = Command::new(oracle.directory().join("probes/wp5/cmyk"))
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(&image("testorig.ppm"))
        .unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success(), "cmyk {args:?}");
    output.stdout
}

/// Every in-scope output color space by number, `JCS_UNKNOWN` to
/// `JCS_EXT_ARGB`. `JCS_RGB565` (16) is out of scope (ADR 0005): the oracle
/// decodes it and the port fails, so `rgb565_is_out_of_scope` checks the
/// port alone.
const COLOR_SPACES: std::ops::RangeInclusive<u32> = 0..=15;

/// The output modes compared for every input.
const MODES: &[&[&str]] = &[
    &[],
    &["-nofancy"],
    &["-dct", "ifast"],
    &["-dct", "float", "-nofancy"],
    &["-scale", "1", "8"],
    &["-scale", "3", "8", "-rows", "3"],
    &["-scale", "13", "8", "-nofancy", "-rows", "2"],
    &["-out", "1"],
    &["-rows", "16"],
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
    let fields = compare(&oracle, &[], &image("monkey12.jpg"), "monkey12.jpg");
    assert_eq!(fields.get("error"), Some("15"), "JERR_BAD_PRECISION");
}

#[test]
fn encoder_variants_match_the_oracle() {
    let Some(oracle) = Oracle::locate("encoder_variants_match_the_oracle") else {
        return;
    };
    for (label, data) in eight_bit_variants(&oracle) {
        for args in MODES {
            compare(&oracle, args, &data, &label);
        }
    }
}

#[test]
fn color_spaces_match_the_oracle() {
    let Some(oracle) = Oracle::locate("color_spaces_match_the_oracle") else {
        return;
    };
    let mut inputs: Vec<(String, Vec<u8>)> = eight_bit_variants(&oracle)
        .into_iter()
        .filter(|(label, _)| {
            [
                "",
                "-sample 2x1",
                "-sample 1x1",
                "-grayscale",
                "-rgb -optimize",
                "-progressive",
            ]
            .contains(&label.as_str())
        })
        .collect();
    inputs.push(("cmyk".to_owned(), four_component(&oracle, &["cmyk"])));
    inputs.push(("ycck".to_owned(), four_component(&oracle, &["ycck"])));
    inputs.push((
        "ycck 2x2,1x1,1x1,2x2".to_owned(),
        four_component(&oracle, &["ycck", "2x2,1x1,1x1,2x2"]),
    ));
    for (label, data) in inputs {
        for space in COLOR_SPACES {
            let space = space.to_string();
            for args in [
                &["-out", &space][..],
                &["-out", &space, "-nofancy", "-rows", "2"],
                &["-out", &space, "-scale", "1", "2"],
            ] {
                compare(&oracle, args, &data, &label);
            }
        }
    }
}

#[test]
fn scaling_and_sampling_match_the_oracle() {
    let Some(oracle) = Oracle::locate("scaling_and_sampling_match_the_oracle") else {
        return;
    };
    for (label, data) in eight_bit_variants(&oracle)
        .into_iter()
        .filter(|(label, _)| {
            label.is_empty() || label.contains("-sample") || label.contains("-grayscale")
        })
    {
        for num in 1..=16 {
            let num = num.to_string();
            for args in [
                &["-scale", &num, "8"][..],
                &["-scale", &num, "8", "-nofancy"],
                &["-scale", &num, "8", "-rows", "5", "-out", "13"],
            ] {
                compare(&oracle, args, &data, &label);
            }
        }
    }
}

#[test]
fn rows_per_call_match_the_oracle() {
    let Some(oracle) = Oracle::locate("rows_per_call_match_the_oracle") else {
        return;
    };
    for (label, data) in eight_bit_variants(&oracle) {
        for rows in ["1", "2", "3", "5", "16", "200"] {
            compare(&oracle, &["-rows", rows], &data, &label);
            compare(&oracle, &["-rows", rows, "-nofancy"], &data, &label);
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
        (&["-ccir601"][..], "25"),
        (&["-out", "4"], "27"),
        (&["-out", "0"], "27"),
    ] {
        let fields = compare(&oracle, args, &data, "testorig.jpg");
        assert_eq!(fields.get("error"), Some(code), "{args:?}");
    }
    /* cjpeg refuses fractional sampling, so the SOF of a 2x1 image gets the
     * factors 3x1, 2x1, 1x1, which jinit_upsampler rejects before any data
     * is decoded. */
    let mut fractional = encode(&oracle, &["-sample", "2x1"]);
    let sof = fractional
        .windows(2)
        .position(|marker| marker == [0xFF, 0xC0])
        .expect("SOF0");
    fractional[sof + 2 + 8 + 1] = 0x31;
    fractional[sof + 2 + 8 + 3 + 1] = 0x21;
    let fields = compare(&oracle, &[], &fractional, "sampling 3x1,2x1,1x1");
    assert_eq!(fields.get("error"), Some("38"), "JERR_FRACT_SAMPLE_NOTIMPL");
}

#[test]
fn suspension_matches_the_oracle() {
    let Some(oracle) = Oracle::locate("suspension_matches_the_oracle") else {
        return;
    };
    let mut inputs = vec![("testorig.jpg".to_owned(), image("testorig.jpg"))];
    inputs.extend(
        eight_bit_variants(&oracle)
            .into_iter()
            .filter(|(label, _)| {
                [
                    "-progressive",
                    "-arithmetic",
                    "-restart 1",
                    "-sample 1x2 -progressive",
                ]
                .contains(&label.as_str())
            }),
    );
    for (label, data) in inputs {
        for step in ["1", "7", "64", "1000"] {
            compare(&oracle, &["-suspend", step], &data, &label);
            compare(
                &oracle,
                &["-suspend", step, "-nofancy", "-rows", "2"],
                &data,
                &label,
            );
        }
    }
}

#[test]
fn damaged_streams_match_the_oracle() {
    let Some(oracle) = Oracle::locate("damaged_streams_match_the_oracle") else {
        return;
    };
    for (label, data) in eight_bit_variants(&oracle) {
        for cut in entropy_positions(&data, 6) {
            let label = format!("{label} cut at {cut}");
            compare(&oracle, &[], &data[..cut], &label);
            compare(&oracle, &["-nofancy", "-rows", "3"], &data[..cut], &label);
        }
        for (index, position) in entropy_positions(&data, 4).into_iter().enumerate() {
            for value in [data[position] ^ 0x55, 0xFF] {
                let mut mutated = data.clone();
                mutated[position] = value;
                if index % 2 == 1 && position + 1 < mutated.len() {
                    mutated[position + 1] = 0x00;
                }
                let label = format!("{label} byte {position} = {value:#04x}");
                compare(&oracle, &[], &mutated, &label);
                compare(&oracle, &["-nofancy", "-out", "1"], &mutated, &label);
            }
        }
    }
}

/// `JCS_RGB565` output fails with `JERR_CONVERSION_NOTIMPL` in the color
/// deconverter and in the merged upsampler, where upstream converts (ADR
/// 0005, "Out-of-scope branches"); the oracle decodes it, so only the port
/// is checked.
#[test]
fn rgb565_is_out_of_scope() {
    let data = image("testorig.jpg");
    for args in [&["-out", "16"][..], &["-out", "16", "-nofancy"]] {
        let fields = wp5::run_rust(args, &data);
        assert_eq!(fields.get("error"), Some("27"), "{args:?}");
        assert_eq!(
            fields.get("merged"),
            Some(if args.len() > 2 { "1" } else { "0" })
        );
    }
}
