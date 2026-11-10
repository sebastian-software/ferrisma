//! WP3 differential tests of entropy decoding: `jdhuff.c` (with
//! `jstdhuff.c`), `jdphuff.c`, `jdarith.c` and `jaricom.c`.
//!
//! The probe `tools/oracle/probes/wp3/entropy.c` reads a header, starts the
//! entropy decoder that `jdmaster.c` selects and drives it through
//! `jdinput.c` with a coefficient controller that mirrors `jdcoefct.c`'s
//! multi-scan input side and keeps every block. It prints every message, the
//! decoder state after each scan (progression status included), every
//! result of `consume_input`, and a hash of every block row of the image.
//! `support/wp3.rs` runs the same sequence on the port; each case compares
//! both outputs exactly.
//!
//! The inputs are upstream's test images, encoder variants that the
//! oracle's `cjpeg` makes from `testorig.ppm` (sequential, progressive and
//! arithmetic coding, restart intervals, sampling factors, 12-bit data),
//! streams built from them (standard tables, damaged and missing tables,
//! changed conditioning, reordered and altered scans, damaged restart
//! markers), truncations and byte mutations of the entropy-coded data, and
//! suspending input.

#[path = "support/oracle.rs"]
mod oracle;
#[path = "support/wp2.rs"]
mod wp2;
#[path = "support/wp3.rs"]
mod wp3;

use std::process::Command;

use oracle::{Fields, Oracle};

/// Runs `args` on `data` with the probe and the port and compares the output.
fn compare(oracle: &Oracle, args: &[&str], data: &[u8], label: &str) -> Fields {
    let expected = oracle
        .probe("wp3", "entropy")
        .run(args, data)
        .success()
        .fields();
    let actual = wp3::run_rust(args, data);
    if actual != expected {
        let first = actual
            .0
            .iter()
            .zip(&expected.0)
            .position(|(a, e)| a != e)
            .unwrap_or(actual.0.len().min(expected.0.len()));
        panic!(
            "{label} {args:?}: outputs differ at field {first}\n  port:   {:?}\n  oracle: {:?}",
            actual.0.get(first),
            expected.0.get(first)
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
        std::env::temp_dir().join(format!("libjpeg-rs-wp3-{}-{call}.jpg", std::process::id()));
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

#[test]
fn test_images_match_the_oracle() {
    let Some(oracle) = Oracle::locate("test_images_match_the_oracle") else {
        return;
    };
    for name in [
        "testorig.jpg",
        "testimgari.jpg",
        "testimgint.jpg",
        "monkey12.jpg",
    ] {
        let fields = compare(&oracle, &[], &image(name), name);
        assert_eq!(fields.get("error"), Some("0"), "{name}");
    }
}

#[test]
fn encoder_variants_match_the_oracle() {
    let Some(oracle) = Oracle::locate("encoder_variants_match_the_oracle") else {
        return;
    };
    for (label, data) in variants(&oracle) {
        let fields = compare(&oracle, &["-trace", "3"], &data, &label);
        assert_eq!(fields.get("error"), Some("0"), "{label}");
    }
}

#[test]
fn skipped_coefficients_match_the_oracle() {
    let Some(oracle) = Oracle::locate("skipped_coefficients_match_the_oracle") else {
        return;
    };
    for (label, data) in variants(&oracle) {
        let sequential = !label.contains("progressive") && !label.contains("test.scan");
        let mut cases: Vec<Vec<&str>> = vec![
            vec!["-scaled", "1"],
            vec!["-scaled", "2"],
            vec!["-unneeded", "0"],
            vec!["-unneeded", "1", "-unneeded", "2"],
        ];
        if sequential {
            cases.push(vec!["-nullrows", "1"]);
            cases.push(vec!["-nullrows", "4", "-scaled", "1"]);
            cases.push(vec!["-nullrows", "100"]);
        }
        for args in cases {
            compare(&oracle, &args, &data, &label);
        }
    }
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
            "-progressive -restart 1",
            "-quality 100 -progressive",
            "-scans test.scan",
        ]
        .contains(&label.as_str())
    }));
    for (label, data) in inputs {
        for step in ["1", "7", "64", "1000"] {
            compare(&oracle, &["-suspend", step], &data, &label);
        }
    }
}

#[test]
fn truncations_match_the_oracle() {
    let Some(oracle) = Oracle::locate("truncations_match_the_oracle") else {
        return;
    };
    for (label, data) in variants(&oracle) {
        for cut in entropy_positions(&data, 12) {
            compare(&oracle, &[], &data[..cut], &format!("{label} cut at {cut}"));
        }
    }
}

#[test]
fn mutations_match_the_oracle() {
    let Some(oracle) = Oracle::locate("mutations_match_the_oracle") else {
        return;
    };
    for (label, data) in variants(&oracle) {
        for (index, position) in entropy_positions(&data, 8).into_iter().enumerate() {
            for value in [data[position] ^ 0x55, 0xFF, 0x00, 0xD9] {
                let mut mutated = data.clone();
                mutated[position] = value;
                if index % 2 == 1 && position + 1 < mutated.len() {
                    mutated[position + 1] = 0x00;
                }
                compare(
                    &oracle,
                    &[],
                    &mutated,
                    &format!("{label} byte {position} = {value:#04x}"),
                );
            }
        }
    }
}

/// A DHT payload for one table.
fn dht(class_and_id: u8, bits: &[u8; 16], values: &[u8]) -> Vec<u8> {
    let mut payload = vec![class_and_id];
    payload.extend(bits);
    payload.extend(values);
    payload
}

/// `parts` with every DHT segment replaced by `tables` (none when empty).
fn with_tables(parts: &[Part], tables: &[Vec<u8>]) -> Vec<Part> {
    let mut result = Vec::new();
    let mut inserted = false;
    for part in parts {
        if let Part::Segment(0xC4, _) = part {
            if !inserted {
                result.extend(
                    tables
                        .iter()
                        .map(|table| Part::Segment(0xC4, table.clone())),
                );
                inserted = true;
            }
        } else {
            result.push(part.clone());
        }
    }
    result
}

/// `parts` with the parameters of the scan headers changed by `edit`, which
/// receives the scan number and the SOS payload.
fn with_scans(parts: &[Part], mut edit: impl FnMut(usize, &mut Vec<u8>)) -> Vec<Part> {
    let mut scan = 0;
    parts
        .iter()
        .map(|part| match part {
            Part::Segment(0xDA, payload) => {
                let mut payload = payload.clone();
                edit(scan, &mut payload);
                scan += 1;
                Part::Segment(0xDA, payload)
            }
            other => other.clone(),
        })
        .collect()
}

/// The scans of `parts`: each SOS segment with its entropy-coded data and
/// restart markers, after the table segments that precede it (those of the
/// first scan stay in the head), and the parts before the first and from
/// EOI on.
fn scans(parts: &[Part]) -> (Vec<Part>, Vec<Vec<Part>>, Vec<Part>) {
    let first = parts
        .iter()
        .position(|part| matches!(part, Part::Segment(0xDA, _)))
        .unwrap();
    let mut scans: Vec<Vec<Part>> = Vec::new();
    let mut pending = Vec::new();
    let mut at = first;
    while at < parts.len() {
        match &parts[at] {
            Part::Segment(0xDA, _) => {
                let mut scan = std::mem::take(&mut pending);
                scan.push(parts[at].clone());
                scans.push(scan);
            }
            Part::Entropy(_) | Part::Standalone(0xD0..=0xD7) => {
                scans.last_mut().unwrap().push(parts[at].clone());
            }
            Part::Standalone(0xD9) => break,
            other => pending.push(other.clone()),
        }
        at += 1;
    }
    pending.extend(parts[at..].iter().cloned());
    (parts[..first].to_vec(), scans, pending)
}

fn assemble(head: &[Part], scans: &[Vec<Part>], tail: &[Part]) -> Vec<u8> {
    let mut parts = head.to_vec();
    for scan in scans {
        parts.extend(scan.iter().cloned());
    }
    parts.extend(tail.iter().cloned());
    join(&parts)
}

/// A change to the last three bytes of a scan header (Ss, Se, Ah/Al).
type ScanEdit = fn(&mut [u8]);

/// Streams built from the variants: tables, conditioning, scan order and
/// parameters, and restart markers.
#[expect(clippy::too_many_lines, reason = "one block per kind of stream")]
fn synthetic_streams(oracle: &Oracle) -> Vec<(String, Vec<u8>)> {
    let mut streams = Vec::new();
    let baseline = parts(&encode(oracle, &[]));
    let progressive = parts(&encode(oracle, &["-progressive"]));
    let arithmetic = parts(&encode(oracle, &["-arithmetic"]));
    let arithmetic_progressive = parts(&encode(oracle, &["-arithmetic", "-progressive"]));
    let restart = parts(&encode(oracle, &["-restart", "1"]));
    let restart_progressive = parts(&encode(oracle, &["-progressive", "-restart", "1"]));

    // Tables: none (the standard tables of jstdhuff.c), an overfull code, a
    // DC symbol beyond 15, a table that the scan names but nobody defines.
    streams.push((
        "baseline without DHT".to_owned(),
        join(&with_tables(&baseline, &[])),
    ));
    streams.push((
        "progressive without DHT".to_owned(),
        join(&with_tables(&progressive, &[])),
    ));
    let mut overfull = [0; 16];
    overfull[0] = 3;
    streams.push((
        "overfull DC table".to_owned(),
        join(&with_tables(&baseline, &[dht(0x00, &overfull, &[0, 1, 2])])),
    ));
    let mut dc_bits = [0; 16];
    dc_bits[1] = 3;
    streams.push((
        "DC symbol 16".to_owned(),
        join(&with_tables(&baseline, &[dht(0x00, &dc_bits, &[0, 1, 16])])),
    ));
    streams.push((
        "progressive DC symbol 16".to_owned(),
        join(&with_tables(
            &progressive,
            &[dht(0x00, &dc_bits, &[0, 1, 16])],
        )),
    ));
    let mut ac_bits = [0; 16];
    ac_bits[0] = 1;
    ac_bits[15] = 1;
    streams.push((
        "AC table with a 16-bit code".to_owned(),
        join(&with_tables(
            &baseline,
            &[dht(0x10, &ac_bits, &[0x00, 0x11])],
        )),
    ));
    for selector in [0x22, 0x02, 0x20, 0x44] {
        streams.push((
            format!("baseline tables {selector:#04x}"),
            join(&with_scans(&baseline, |_, payload| payload[2] = selector)),
        ));
        streams.push((
            format!("progressive tables {selector:#04x}"),
            join(&with_scans(&progressive, |_, payload| {
                payload[2] = selector
            })),
        ));
        streams.push((
            format!("arithmetic tables {selector:#04x}"),
            join(&with_scans(&arithmetic, |_, payload| payload[2] = selector)),
        ));
    }
    streams.push((
        "arithmetic table 16".to_owned(),
        join(&with_scans(&arithmetic, |_, payload| payload[2] = 0xF0)),
    ));

    // Arithmetic conditioning: DAC values that the encoder did not use.
    for (index, (class_and_id, value)) in [
        (0x00, 0x52),
        (0x00, 0x00),
        (0x00, 0xFF),
        (0x10, 1),
        (0x10, 63),
        (0x11, 20),
    ]
    .into_iter()
    .enumerate()
    {
        for (name, stream) in [
            ("arithmetic", &arithmetic),
            ("arithmetic progressive", &arithmetic_progressive),
        ] {
            let mut changed = Vec::new();
            for part in stream {
                if let Part::Segment(0xDA, _) = part
                    && !changed
                        .iter()
                        .any(|part| matches!(part, Part::Segment(0xDA, _)))
                {
                    changed.push(Part::Segment(0xCC, vec![class_and_id, value]));
                }
                changed.push(part.clone());
            }
            streams.push((format!("{name} DAC {index}"), join(&changed)));
        }
    }

    // A second scan in a single-scan stream.
    let (head, baseline_scans, tail) = scans(&baseline);
    let twice = [baseline_scans[0].clone(), baseline_scans[0].clone()];
    streams.push((
        "baseline scan twice".to_owned(),
        assemble(&head, &twice, &tail),
    ));

    // Scan order: an AC scan first, a scan twice, the last scan dropped.
    for (name, stream) in [
        ("progressive", &progressive),
        ("arithmetic progressive", &arithmetic_progressive),
    ] {
        let (head, scans, tail) = scans(stream);
        let mut reordered = scans.clone();
        reordered.swap(0, 1);
        streams.push((
            format!("{name} AC first"),
            assemble(&head, &reordered, &tail),
        ));
        let mut repeated = scans.clone();
        repeated.insert(2, scans[1].clone());
        streams.push((
            format!("{name} scan twice"),
            assemble(&head, &repeated, &tail),
        ));
        let mut refinement_first = scans.clone();
        let last = refinement_first.pop().unwrap();
        refinement_first.insert(1, last);
        streams.push((
            format!("{name} refinement first"),
            assemble(&head, &refinement_first, &tail),
        ));
        streams.push((
            format!("{name} only DC"),
            assemble(&head, &scans[..1], &tail),
        ));
    }

    // Scan parameters: the last three bytes of SOS are Ss, Se and Ah/Al.
    let parameters: &[(&str, ScanEdit)] = &[
        ("Al 14", |p| p[2] = (p[2] & 0xF0) | 14),
        ("Al 13", |p| p[2] = (p[2] & 0xF0) | 13),
        ("Ah 1", |p| p[2] = 0x10 | (p[2] & 0x0F)),
        ("Ah 5", |p| p[2] = 0x50 | (p[2] & 0x0F)),
        ("Se 0", |p| p[1] = 0),
        ("Se 64", |p| p[1] = 64),
        ("Ss 5", |p| p[0] = 5),
        ("Ss 63", |p| p[0] = 63),
        ("Ss 1 Se 63", |p| {
            p[0] = 1;
            p[1] = 63;
        }),
    ];
    for &(name, edit) in parameters {
        for (stream_name, stream) in [
            ("baseline", &baseline),
            ("progressive", &progressive),
            ("arithmetic", &arithmetic),
            ("arithmetic progressive", &arithmetic_progressive),
        ] {
            for scan in [0, 1, 4] {
                let changed = with_scans(stream, |index, payload| {
                    if index == scan {
                        let at = payload.len() - 3;
                        edit(&mut payload[at..]);
                    }
                });
                streams.push((format!("{stream_name} scan {scan} {name}"), join(&changed)));
            }
        }
    }

    // Restart markers: a wrong number, one missing, one doubled, fill bytes.
    for (name, stream) in [
        ("restart", &restart),
        ("progressive restart", &restart_progressive),
    ] {
        let markers: Vec<usize> = stream
            .iter()
            .enumerate()
            .filter(|(_, part)| matches!(part, Part::Standalone(0xD0..=0xD7)))
            .map(|(index, _)| index)
            .collect();
        for &at in [
            markers[0],
            markers[markers.len() / 2],
            markers[markers.len() - 1],
        ]
        .iter()
        {
            let mut wrong = stream.clone();
            if let Part::Standalone(marker) = &mut wrong[at] {
                *marker = 0xD0 + ((*marker - 0xD0 + 3) & 7);
            }
            streams.push((format!("{name} wrong RST at {at}"), join(&wrong)));
            let mut missing = stream.clone();
            missing.remove(at);
            streams.push((format!("{name} missing RST at {at}"), join(&missing)));
            let mut doubled = stream.clone();
            doubled.insert(at, stream[at].clone());
            streams.push((format!("{name} doubled RST at {at}"), join(&doubled)));
            let mut filled = stream.clone();
            filled.insert(at, Part::Entropy(vec![0xFF, 0xFF, 0xFF]));
            streams.push((format!("{name} fill before RST at {at}"), join(&filled)));
        }
    }
    streams
}

/// Packs bits into entropy-coded bytes, with FF/00 stuffing and 1-bit
/// padding.
#[derive(Default)]
struct BitWriter {
    bytes: Vec<u8>,
    buffer: u32,
    count: u32,
}

impl BitWriter {
    fn put(&mut self, bits: u32, count: u32) {
        for index in (0..count).rev() {
            self.buffer = (self.buffer << 1) | ((bits >> index) & 1);
            self.count += 1;
            if self.count == 8 {
                let byte = u8::try_from(self.buffer).unwrap();
                self.bytes.push(byte);
                if byte == 0xFF {
                    self.bytes.push(0);
                }
                self.buffer = 0;
                self.count = 0;
            }
        }
    }

    fn finish(mut self) -> Vec<u8> {
        while self.count != 0 {
            self.put(1, 1);
        }
        self.bytes
    }
}

/// A one-component image of 8187 x 9 blocks whose every DC difference is
/// +32767, so that the DC prediction leaves the range of `int` after 65538
/// blocks: `jdphuff.c` reports `JERR_BAD_DCT_COEF`, `jdhuff.c` wraps.
fn dc_overflow(progressive: bool) -> Vec<u8> {
    let mut parts = vec![
        Part::Standalone(0xD8),
        Part::Segment(0xDB, [vec![0], vec![1; 64]].concat()),
    ];
    let (width, height): (u16, u16) = (8187 * 8, 9 * 8);
    let mut sof = vec![8];
    sof.extend(height.to_be_bytes());
    sof.extend(width.to_be_bytes());
    sof.extend([1, 1, 0x11, 0]);
    parts.push(Part::Segment(if progressive { 0xC2 } else { 0xC0 }, sof));
    let mut one_code = [0; 16];
    one_code[0] = 1;
    parts.push(Part::Segment(0xC4, dht(0x00, &one_code, &[15])));
    parts.push(Part::Segment(0xC4, dht(0x10, &one_code, &[0x00])));
    let se = if progressive { 0 } else { 63 };
    parts.push(Part::Segment(0xDA, vec![1, 1, 0x00, 0, se, 0]));
    let mut writer = BitWriter::default();
    for _ in 0..8187 * 9 {
        writer.put(0, 1); /* DC category 15 */
        writer.put(0x7FFF, 15); /* +32767 */
        if !progressive {
            writer.put(0, 1); /* EOB */
        }
    }
    parts.push(Part::Entropy(writer.finish()));
    parts.push(Part::Standalone(0xD9));
    join(&parts)
}

#[test]
fn dc_overflow_matches_the_oracle() {
    let Some(oracle) = Oracle::locate("dc_overflow_matches_the_oracle") else {
        return;
    };
    let fields = compare(&oracle, &[], &dc_overflow(false), "sequential");
    assert_eq!(fields.get("error"), Some("0"));
    let fields = compare(&oracle, &[], &dc_overflow(true), "progressive");
    assert_eq!(fields.get("error"), Some("6"), "JERR_BAD_DCT_COEF");
}

#[test]
fn synthetic_streams_match_the_oracle() {
    let Some(oracle) = Oracle::locate("synthetic_streams_match_the_oracle") else {
        return;
    };
    for (label, data) in synthetic_streams(&oracle) {
        compare(&oracle, &["-trace", "3"], &data, &label);
    }
}

#[test]
fn standard_tables_reproduce_the_coefficients() {
    // Without DHT, jstdhuff.c's tables are the ones cjpeg used, so the
    // coefficients are those of the original stream.
    let Some(oracle) = Oracle::locate("standard_tables_reproduce_the_coefficients") else {
        return;
    };
    let baseline = encode(&oracle, &[]);
    let original = wp3::run_rust(&[], &baseline);
    let stripped = wp3::run_rust(&[], &join(&with_tables(&parts(&baseline), &[])));
    for key in ["rows0", "rows1", "rows2", "error"] {
        assert_eq!(original.get(key), stripped.get(key), "{key}");
    }
}
