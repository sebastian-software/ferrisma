//! The test inputs of the differential tests from work package 5 on:
//! upstream's test images, the `cjpeg` encoder variants, four-component
//! images from the probe `wp5/cmyk`, and the parts of a stream that the
//! damage tests cut and alter. Include it next to `support/oracle.rs`.

#![allow(dead_code, reason = "each test binary uses part of the support")]

use std::io::Write;
use std::process::{Command, Stdio};

use crate::oracle::{self, Oracle};

/// The upstream test image `name`.
pub fn image(name: &str) -> Vec<u8> {
    std::fs::read(oracle::test_image(name)).unwrap()
}

/// Encodes `testorig.ppm` with the oracle's `cjpeg` and `switches`.
pub fn encode(oracle: &Oracle, switches: &[&str]) -> Vec<u8> {
    // One file per call: tests run in parallel and each removes its own.
    static CALLS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    let call = CALLS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let output = std::env::temp_dir().join(format!(
        "libjpeg-rs-inputs-{}-{call}.jpg",
        std::process::id()
    ));
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
pub const VARIANTS: &[&[&str]] = &[
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

pub fn variants(oracle: &Oracle) -> Vec<(String, Vec<u8>)> {
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
pub enum Part {
    /// A marker without parameters (SOI, EOI, RSTn).
    Standalone(u8),
    /// A marker segment and its payload.
    Segment(u8, Vec<u8>),
    /// Entropy-coded data, up to the next marker other than RSTn.
    Entropy(Vec<u8>),
}

/// Splits a whole stream into parts. RST markers inside entropy-coded data
/// are parts of their own.
pub fn parts(jpeg: &[u8]) -> Vec<Part> {
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

pub fn join(parts: &[Part]) -> Vec<u8> {
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
pub fn entropy_ranges(jpeg: &[u8]) -> Vec<std::ops::Range<usize>> {
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
pub fn entropy_positions(jpeg: &[u8], count: usize) -> Vec<usize> {
    let positions: Vec<usize> = entropy_ranges(jpeg).into_iter().flatten().collect();
    (0..count)
        .map(|index| positions[index * positions.len() / count])
        .collect()
}

/// Whether a variant has 8-bit samples, which the port decodes.
pub fn eight_bit(label: &str) -> bool {
    !label.contains("-precision 12")
}

/// The 8-bit encoder variants.
pub fn eight_bit_variants(oracle: &Oracle) -> Vec<(String, Vec<u8>)> {
    variants(oracle)
        .into_iter()
        .filter(|(label, _)| eight_bit(label))
        .collect()
}

/// A four-component image of `testorig.ppm` from the probe `wp5/cmyk`.
pub fn four_component(oracle: &Oracle, args: &[&str]) -> Vec<u8> {
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
