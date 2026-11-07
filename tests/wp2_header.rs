//! WP2 differential tests of the input side: `jpeg_read_header` and the
//! modules behind it (`jdapimin.c`, `jdatasrc.c`'s memory source,
//! `jdinput.c`, `jdmarker.c`, `jdicc.c`, `jpeg_abort`).
//!
//! The probe `tools/oracle/probes/wp2/header.c` runs an API sequence against
//! the C library and prints every trace, warning and error message, the
//! result of each call, the state of each suspension, and the object it
//! leaves: header fields, component layout, tables, saved markers, module
//! state, source position and memory pools. `support/wp2.rs` runs the same
//! sequence on the port; each case compares both outputs exactly.
//!
//! The inputs are upstream's test images, encoder variants that the
//! oracle's `cjpeg` makes from `testorig.ppm`, marker streams built from
//! `testorig.jpg`'s segments, every truncation and a set of byte mutations of
//! the headers, and suspending input. Upstream's fuzz seed corpus is not part
//! of the pinned tree.

#[path = "support/oracle.rs"]
mod oracle;
#[path = "support/wp2.rs"]
mod wp2;

use std::path::{Path, PathBuf};
use std::process::Command;

use oracle::{Fields, Oracle};

/// The one field where the port differs by design: with
/// `QUANT_2PASS_SUPPORTED` treated as undefined (ADR 0005), upstream's
/// `default_decompress_parms` sets `two_pass_quantize` to `FALSE`, where the
/// oracle, which compiles color quantization, sets `TRUE`. The oracle's value
/// is mapped to the port's; nothing else is changed.
fn without_two_pass_quantize(mut fields: Fields) -> Fields {
    for (key, value) in &mut fields.0 {
        if key == "params" {
            let mut items: Vec<&str> = value.split(',').collect();
            assert!(items[11] == "0" || items[11] == "1", "{value}");
            items[11] = "0";
            *value = items.join(",");
        }
    }
    fields
}

/// Runs `args` on `data` with the probe and the port and compares the output.
fn compare(oracle: &Oracle, args: &[&str], data: &[u8], label: &str) -> Fields {
    let expected = without_two_pass_quantize(
        oracle
            .probe("wp2", "header")
            .run(args, data)
            .success()
            .fields(),
    );
    let actual = wp2::run_rust(args, data);
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

/// One marker segment of a JPEG header.
#[derive(Clone, Debug)]
struct Segment {
    marker: u8,
    payload: Vec<u8>,
}

impl Segment {
    fn new(marker: u8, payload: &[u8]) -> Segment {
        Segment {
            marker,
            payload: payload.to_vec(),
        }
    }

    fn bytes(&self) -> Vec<u8> {
        let length = u16::try_from(self.payload.len() + 2).unwrap();
        let mut bytes = vec![0xFF, self.marker];
        bytes.extend_from_slice(&length.to_be_bytes());
        bytes.extend_from_slice(&self.payload);
        bytes
    }
}

/// The segments between SOI and SOS (inclusive) and the entropy-coded rest.
fn split(jpeg: &[u8]) -> (Vec<Segment>, Vec<u8>) {
    assert_eq!(&jpeg[..2], &[0xFF, 0xD8]);
    let mut segments = Vec::new();
    let mut at = 2;
    loop {
        assert_eq!(jpeg[at], 0xFF);
        let marker = jpeg[at + 1];
        let length = usize::from(u16::from_be_bytes([jpeg[at + 2], jpeg[at + 3]]));
        segments.push(Segment::new(marker, &jpeg[at + 4..at + 2 + length]));
        at += 2 + length;
        if marker == 0xDA {
            return (segments, jpeg[at..].to_vec());
        }
    }
}

fn join(segments: &[Segment], rest: &[u8]) -> Vec<u8> {
    let mut bytes = vec![0xFF, 0xD8];
    for segment in segments {
        bytes.extend(segment.bytes());
    }
    bytes.extend_from_slice(rest);
    bytes
}

/// The length of the header up to and including SOS.
fn header_length(jpeg: &[u8]) -> usize {
    let (segments, rest) = split(jpeg);
    jpeg.len() - rest.len() - usize::from(segments.is_empty())
}

/// Encoder variants of `testorig.ppm`, made with the oracle's `cjpeg`.
fn variants(oracle: &Oracle) -> Vec<(String, Vec<u8>)> {
    let directory = std::env::temp_dir().join(format!("libjpeg-rs-wp2-{}", std::process::id()));
    std::fs::create_dir_all(&directory).unwrap();
    let source = oracle::test_image("testorig.ppm");
    let icc1 = oracle::test_image("test1.icc");
    let icc3 = oracle::test_image("test3.icc");
    let switches: Vec<Vec<String>> = [
        vec!["-progressive"],
        vec!["-arithmetic"],
        vec!["-progressive", "-arithmetic"],
        vec!["-restart", "1"],
        vec!["-restart", "3B"],
        vec!["-sample", "1x1"],
        vec!["-sample", "2x1"],
        vec!["-sample", "1x2"],
        vec!["-sample", "4x1,1x1,1x1"],
        vec!["-sample", "3x2"],
        vec!["-grayscale"],
        vec!["-rgb"],
        vec!["-optimize"],
        vec!["-quality", "100,20"],
        vec!["-qslots", "1,0,0"],
        vec!["-baseline", "-quality", "5"],
        vec!["-precision", "12"],
        vec!["-precision", "12", "-progressive"],
    ]
    .iter()
    .map(|switches| switches.iter().map(|&s| s.to_owned()).collect())
    .chain([
        vec!["-icc".to_owned(), icc1.display().to_string()],
        vec!["-icc".to_owned(), icc3.display().to_string()],
    ])
    .collect();
    let mut variants = Vec::new();
    for (index, switches) in switches.iter().enumerate() {
        let output = directory.join(format!("variant{index}.jpg"));
        let status = Command::new(oracle.tool("cjpeg"))
            .args(switches)
            .arg("-outfile")
            .arg(&output)
            .arg(&source)
            .status()
            .unwrap();
        assert!(status.success(), "cjpeg {switches:?}");
        variants.push((switches.join(" "), std::fs::read(&output).unwrap()));
    }
    let _ = std::fs::remove_dir_all(&directory);
    variants
}

#[test]
fn test_images_match_the_oracle() {
    let Some(oracle) = Oracle::locate("test_images_match_the_oracle") else {
        return;
    };
    for name in [
        "testorig.jpg",
        "testimgint.jpg",
        "testimgari.jpg",
        "monkey12.jpg",
    ] {
        let data = image(name);
        for args in [
            &[][..],
            &["-trace", "3"],
            &[
                "-save", "254", "65535", "-save", "224", "0", "-save", "226", "65535", "-icc",
            ],
            &["-query", "-consume", "2", "-again", "-abort"],
            &[
                "-save",
                "224",
                "3",
                "-save",
                "225",
                "1000",
                "-processor",
                "238",
                "-trace",
                "1",
            ],
        ] {
            compare(&oracle, args, &data, name);
        }
    }
}

#[test]
fn encoder_variants_match_the_oracle() {
    let Some(oracle) = Oracle::locate("encoder_variants_match_the_oracle") else {
        return;
    };
    for (label, data) in variants(&oracle) {
        for args in [
            &["-trace", "3", "-query"][..],
            &["-save", "226", "65535", "-icc"],
            &["-suspend", "1", "-save", "226", "65535", "-icc"],
            &["-suspend", "7", "-trace", "2"],
        ] {
            compare(&oracle, args, &data, &label);
        }
    }
}

#[test]
fn restart_markers_match_the_oracle() {
    let Some(oracle) = Oracle::locate("restart_markers_match_the_oracle") else {
        return;
    };
    for (label, data) in variants(&oracle) {
        if !label.starts_with("-restart") {
            continue;
        }
        for args in [
            &["-trace", "4", "-rst", "6"][..],
            &["-trace", "4", "-resync", "0"],
            &["-trace", "4", "-resync", "3"],
            &["-trace", "4", "-resync", "6", "-rst", "2"],
            &["-suspend", "3", "-rst", "4"],
        ] {
            compare(&oracle, args, &data, &label);
        }
    }
    // A scan without restart markers: read_restart_marker scans the whole
    // entropy-coded segment to EOI.
    compare(
        &oracle,
        &["-trace", "4", "-rst", "1"],
        &image("testorig.jpg"),
        "testorig.jpg",
    );
}

/// Marker streams built from `testorig.jpg`: each exercises one branch of
/// the marker reader.
#[expect(clippy::too_many_lines, reason = "one case per branch")]
fn synthetic_streams() -> Vec<(String, Vec<u8>)> {
    let (segments, rest) = split(&image("testorig.jpg"));
    let position = |marker: u8| segments.iter().position(|s| s.marker == marker).unwrap();
    let sof = position(0xC0);
    let sos = position(0xDA);
    let mut cases: Vec<(String, Vec<u8>)> = Vec::new();
    let mut add = |label: &str, segments: Vec<Segment>, rest: &[u8]| {
        cases.push((label.to_owned(), join(&segments, rest)));
    };
    let with = |index: usize, inserted: &[Segment]| {
        let mut copy = segments.clone();
        for (offset, segment) in inserted.iter().enumerate() {
            copy.insert(index + offset, segment.clone());
        }
        copy
    };
    let replace = |index: usize, segment: Segment| {
        let mut copy = segments.clone();
        copy[index] = segment;
        copy
    };
    let without = |marker: u8| {
        segments
            .iter()
            .filter(|s| s.marker != marker)
            .cloned()
            .collect::<Vec<_>>()
    };

    // APP0: JFIF versions, thumbnails, JFXX extensions, short and unknown.
    for (label, payload) in [
        (
            "jfif-2.01",
            b"JFIF\0\x02\x01\x01\x00\x48\x00\x48\x00\x00".to_vec(),
        ),
        (
            "jfif-1.09-dpcm",
            b"JFIF\0\x01\x09\x02\x01\x2c\x01\x2c\x00\x00".to_vec(),
        ),
        ("jfif-thumbnail", {
            let mut p = b"JFIF\0\x01\x02\x00\x00\x01\x00\x01\x02\x03".to_vec();
            p.extend(vec![7; 18]);
            p
        }),
        (
            "jfif-bad-thumbnail",
            b"JFIF\0\x01\x02\x00\x00\x01\x00\x01\x02\x03\x01".to_vec(),
        ),
        ("jfxx-jpeg", b"JFXX\0\x10\xff\xd8\xff\xd9".to_vec()),
        ("jfxx-palette", b"JFXX\0\x11\x01\x01".to_vec()),
        ("jfxx-rgb", b"JFXX\0\x13\x01\x01".to_vec()),
        ("jfxx-other", b"JFXX\0\x42".to_vec()),
        ("app0-short", b"JFI".to_vec()),
        ("app0-empty", Vec::new()),
        ("app0-other", b"AVI1\0\0\0\0\0\0\0\0\0\0\0\0".to_vec()),
    ] {
        add(label, replace(0, Segment::new(0xE0, &payload)), &rest);
    }
    // APP14: Adobe transforms for 3 and 4 components, short and unknown.
    for transform in [0u8, 1, 2, 7] {
        let mut payload = b"Adobe\0\x64\x80\x00\x00\x00".to_vec();
        payload.push(transform);
        add(
            &format!("adobe-{transform}"),
            with(1, &[Segment::new(0xEE, &payload)]),
            &rest,
        );
        let mut no_jfif = without(0xE0);
        no_jfif.insert(0, Segment::new(0xEE, &payload));
        add(&format!("adobe-only-{transform}"), no_jfif.clone(), &rest);
        // Four components: CMYK or YCCK.
        let mut four = no_jfif.clone();
        let sof = four.iter().position(|s| s.marker == 0xC0).unwrap();
        let mut payload = four[sof].payload.clone();
        payload[5] = 4;
        payload.extend([4, 0x11, 1]);
        four[sof] = Segment::new(0xC0, &payload);
        add(&format!("adobe-cmyk-{transform}"), four, &rest);
    }
    add(
        "app14-short",
        with(1, &[Segment::new(0xEE, b"Adob")]),
        &rest,
    );
    add(
        "app14-other",
        with(1, &[Segment::new(0xEE, b"Other marker")]),
        &rest,
    );
    // Component IDs without JFIF or Adobe markers.
    for ids in [[1u8, 2, 3], [82, 71, 66], [0, 1, 2], [5, 6, 7]] {
        let mut copy = without(0xE0);
        let sof = copy.iter().position(|s| s.marker == 0xC0).unwrap();
        let sos = copy.iter().position(|s| s.marker == 0xDA).unwrap();
        let mut payload = copy[sof].payload.clone();
        let mut scan = copy[sos].payload.clone();
        for (index, id) in ids.iter().enumerate() {
            payload[6 + 3 * index] = *id;
            scan[1 + 2 * index] = *id;
        }
        copy[sof] = Segment::new(0xC0, &payload);
        copy[sos] = Segment::new(0xDA, &scan);
        add(&format!("ids-{ids:?}"), copy, &rest);
    }
    // Component counts: 1 (grayscale scan), 2 and 5 (unknown), 11 (too many).
    for count in [2u8, 5, 11] {
        let mut copy = segments.clone();
        let mut payload = copy[sof].payload[..6].to_vec();
        payload[5] = count;
        for id in 1..=count {
            payload.extend([id, 0x11, 0]);
        }
        copy[sof] = Segment::new(0xC0, &payload);
        copy[sos] = Segment::new(0xDA, &[1, 1, 0, 0, 63, 0]);
        add(&format!("components-{count}"), copy, &rest);
    }
    // SOF errors and limits.
    let sof_with = |edit: &dyn Fn(&mut Vec<u8>)| {
        let mut payload = segments[sof].payload.clone();
        edit(&mut payload);
        replace(sof, Segment::new(0xC0, &payload))
    };
    add(
        "height-0",
        sof_with(&|p| p[1..3].copy_from_slice(&[0, 0])),
        &rest,
    );
    add(
        "width-65501",
        sof_with(&|p| p[3..5].copy_from_slice(&65501u16.to_be_bytes())),
        &rest,
    );
    add(
        "width-65500",
        sof_with(&|p| p[3..5].copy_from_slice(&65500u16.to_be_bytes())),
        &rest,
    );
    add("precision-7", sof_with(&|p| p[0] = 7), &rest);
    add("precision-16", sof_with(&|p| p[0] = 16), &rest);
    add("sampling-0", sof_with(&|p| p[7] = 0x01), &rest);
    add("sampling-5", sof_with(&|p| p[7] = 0x51), &rest);
    add("sof-length", sof_with(&|p| p.push(0)), &rest);
    add("sof-duplicate", with(sof, &[segments[sof].clone()]), &rest);
    for marker in [
        0xC1u8, 0xC2, 0xC3, 0xC9, 0xCA, 0xCB, 0xC5, 0xC6, 0xC7, 0xC8, 0xCD, 0xCE, 0xCF,
    ] {
        add(
            &format!("sof-{marker:02x}"),
            replace(sof, Segment::new(marker, &segments[sof].payload)),
            &rest,
        );
    }
    // SOS errors.
    let sos_with = |payload: &[u8]| replace(sos, Segment::new(0xDA, payload));
    add(
        "sos-unknown-id",
        sos_with(&[3, 1, 0, 9, 0x11, 3, 0x11, 0, 63, 0]),
        &rest,
    );
    add(
        "sos-duplicate-id",
        sos_with(&[3, 1, 0, 1, 0x11, 3, 0x11, 0, 63, 0]),
        &rest,
    );
    add(
        "sos-length",
        sos_with(&[3, 1, 0, 2, 0x11, 3, 0x11, 0, 63]),
        &rest,
    );
    add("sos-count-0", sos_with(&[0, 0, 63, 0]), &rest);
    add(
        "sos-count-5",
        sos_with(&[5, 1, 0, 2, 0, 3, 0, 1, 0, 2, 0, 0, 63, 0]),
        &rest,
    );
    add("sos-one", sos_with(&[1, 2, 0x11, 0, 63, 0]), &rest);
    add(
        "sos-params",
        sos_with(&[3, 1, 0, 2, 0x11, 3, 0x11, 1, 5, 0x21]),
        &rest,
    );
    add(
        "sos-before-sof",
        {
            let mut copy = segments.clone();
            let scan = copy.remove(sos);
            copy.insert(sof, scan);
            copy
        },
        &rest,
    );
    // Tables.
    add(
        "dqt-16bit",
        with(
            sof,
            &[Segment::new(0xDB, &{
                let mut p = vec![0x12];
                for i in 0u16..64 {
                    p.extend((i * 300 + 1).to_be_bytes());
                }
                p
            })],
        ),
        &rest,
    );
    add(
        "dqt-index",
        with(
            sof,
            &[Segment::new(0xDB, &{
                let mut p = vec![0x04];
                p.extend([1; 64]);
                p
            })],
        ),
        &rest,
    );
    add(
        "dqt-length",
        with(
            sof,
            &[Segment::new(0xDB, &{
                let mut p = vec![0x02];
                p.extend([1; 65]);
                p
            })],
        ),
        &rest,
    );
    add(
        "dqt-two",
        with(
            sof,
            &[Segment::new(0xDB, &{
                let mut p = vec![0x02];
                p.extend([2; 64]);
                p.push(0x13);
                for i in 0u16..64 {
                    p.extend((i + 1).to_be_bytes());
                }
                p
            })],
        ),
        &rest,
    );
    add(
        "dht-index",
        with(
            sof,
            &[Segment::new(0xC4, &{
                let mut p = vec![0x05];
                p.extend([0; 16]);
                p
            })],
        ),
        &rest,
    );
    add(
        "dht-ac-index",
        with(
            sof,
            &[Segment::new(0xC4, &{
                let mut p = vec![0x14];
                p.extend([0; 16]);
                p
            })],
        ),
        &rest,
    );
    add(
        "dht-count",
        with(
            sof,
            &[Segment::new(0xC4, &{
                let mut p = vec![0x02];
                p.extend([255; 16]);
                p
            })],
        ),
        &rest,
    );
    add(
        "dht-length",
        with(
            sof,
            &[Segment::new(0xC4, &{
                let mut p = vec![0x02, 1];
                p.extend([0; 15]);
                p.extend([5, 6]);
                p
            })],
        ),
        &rest,
    );
    add(
        "dht-empty",
        with(
            sof,
            &[Segment::new(0xC4, &{
                let mut p = vec![0x13];
                p.extend([0; 16]);
                p
            })],
        ),
        &rest,
    );
    add(
        "dac",
        with(
            sof,
            &[Segment::new(0xCC, &[0, 0x52, 1, 0x10, 16, 7, 31, 200])],
        ),
        &rest,
    );
    add(
        "dac-index",
        with(sof, &[Segment::new(0xCC, &[32, 0])]),
        &rest,
    );
    add(
        "dac-value",
        with(sof, &[Segment::new(0xCC, &[2, 0x1F])]),
        &rest,
    );
    add(
        "dac-length",
        with(sof, &[Segment::new(0xCC, &[2, 0x11, 3])]),
        &rest,
    );
    add(
        "dri",
        with(sof, &[Segment::new(0xDD, &[0x01, 0x02])]),
        &rest,
    );
    add(
        "dri-length",
        with(sof, &[Segment::new(0xDD, &[0x01, 0x02, 0x03])]),
        &rest,
    );
    // Other markers.
    add("com", with(1, &[Segment::new(0xFE, b"a comment")]), &rest);
    add(
        "app-all",
        with(
            1,
            &(0xE1..=0xEF)
                .map(|m| Segment::new(m, &[m; 20]))
                .collect::<Vec<_>>(),
        ),
        &rest,
    );
    add("dnl", with(sof, &[Segment::new(0xDC, &[0, 10])]), &rest);
    add(
        "reserved-dhp",
        with(sof, &[Segment::new(0xDE, &[0, 0])]),
        &rest,
    );
    add(
        "reserved-jpg0",
        with(sof, &[Segment::new(0xF0, &[0])]),
        &rest,
    );
    // Stream-level variants, added directly.
    let base = join(&segments, &rest);
    let mut raw = |label: &str, bytes: Vec<u8>| cases.push((label.to_owned(), bytes));
    raw("garbage-before-marker", {
        let mut b = base.clone();
        let at = b.windows(2).position(|w| w == [0xFF, 0xDB]).unwrap();
        b.splice(at..at, [1, 2, 3, 0xFF, 0x00, 4]);
        b
    });
    raw("fill-bytes", {
        let mut b = base.clone();
        let at = b.windows(2).position(|w| w == [0xFF, 0xDB]).unwrap();
        b.splice(at..at, [0xFF, 0xFF, 0xFF]);
        b
    });
    raw("parameterless", {
        let mut b = base.clone();
        let at = b.windows(2).position(|w| w == [0xFF, 0xDB]).unwrap();
        b.splice(at..at, [0xFF, 0xD3, 0xFF, 0x01]);
        b
    });
    raw("com-bogus-length", {
        let mut b = base.clone();
        b.splice(2..2, [0xFF, 0xFE, 0x00, 0x01]);
        b
    });
    raw("app-bogus-length", {
        let mut b = base.clone();
        b.splice(2..2, [0xFF, 0xE0, 0x00, 0x00]);
        b
    });
    raw("soi-duplicate", {
        let mut b = base.clone();
        b.splice(2..2, [0xFF, 0xD8]);
        b
    });
    raw("no-soi", base[2..].to_vec());
    raw("not-jpeg", b"GIF89a.......".to_vec());
    raw("eoi-only", vec![0xFF, 0xD8, 0xFF, 0xD9]);
    raw("one-byte", vec![0xFF]);
    raw("tables-only", {
        let mut b = join(&segments[..sof], &[]);
        b.extend([0xFF, 0xD9]);
        b
    });
    raw("tables-then-image", {
        let mut b = join(&segments[..sof], &[]);
        b.extend([0xFF, 0xD9]);
        b.extend(join(&without(0xDB), &rest));
        b
    });
    raw("sof-then-eoi", {
        let mut b = join(&segments[..=sof], &[]);
        b.extend([0xFF, 0xD9]);
        b
    });
    cases
}

#[test]
fn marker_streams_match_the_oracle() {
    let Some(oracle) = Oracle::locate("marker_streams_match_the_oracle") else {
        return;
    };
    for (label, data) in synthetic_streams() {
        for args in [
            &["-trace", "3"][..],
            &["-trace", "1", "-noimage", "-again"],
            &[
                "-save", "254", "5", "-save", "238", "0", "-save", "224", "100", "-query",
            ],
            &["-suspend", "2", "-trace", "1"],
            &["-processor", "254", "-processor", "227", "-trace", "1"],
        ] {
            compare(&oracle, args, &data, &label);
        }
    }
}

fn icc_marker(sequence: u8, count: u8, data: &[u8]) -> Segment {
    let mut payload = b"ICC_PROFILE\0".to_vec();
    payload.extend([sequence, count]);
    payload.extend_from_slice(data);
    Segment::new(0xE2, &payload)
}

#[test]
fn icc_profiles_match_the_oracle() {
    let Some(oracle) = Oracle::locate("icc_profiles_match_the_oracle") else {
        return;
    };
    let (segments, rest) = split(&image("testorig.jpg"));
    let cases: Vec<(&str, Vec<Segment>)> = vec![
        (
            "two-chunks",
            vec![icc_marker(2, 2, b"world"), icc_marker(1, 2, b"hello ")],
        ),
        ("one-chunk", vec![icc_marker(1, 1, &[9; 300])]),
        (
            "missing",
            vec![icc_marker(1, 3, b"a"), icc_marker(3, 3, b"c")],
        ),
        (
            "duplicate",
            vec![icc_marker(1, 2, b"a"), icc_marker(1, 2, b"b")],
        ),
        ("sequence-0", vec![icc_marker(0, 2, b"a")]),
        ("sequence-high", vec![icc_marker(3, 2, b"a")]),
        (
            "counts",
            vec![icc_marker(1, 2, b"a"), icc_marker(2, 3, b"b")],
        ),
        ("empty", vec![icc_marker(1, 2, b""), icc_marker(2, 2, b"")]),
        (
            "not-icc",
            vec![Segment::new(0xE2, b"ICC_PROFILX\0\x01\x01data")],
        ),
        ("short", vec![Segment::new(0xE2, b"ICC_PROFILE\0\x01")]),
        ("none", vec![]),
    ];
    for (label, markers) in cases {
        let mut copy = segments.clone();
        for (index, marker) in markers.into_iter().enumerate() {
            copy.insert(1 + index, marker);
        }
        let data = join(&copy, &rest);
        compare(&oracle, &["-save", "226", "65535", "-icc"], &data, label);
        compare(&oracle, &["-save", "226", "20", "-icc"], &data, label);
        compare(&oracle, &["-icc"], &data, label);
    }
}

#[test]
fn truncations_match_the_oracle() {
    let Some(oracle) = Oracle::locate("truncations_match_the_oracle") else {
        return;
    };
    let mut inputs = vec![("testorig.jpg".to_owned(), image("testorig.jpg"))];
    inputs.extend(
        variants(&oracle)
            .into_iter()
            .filter(|(label, _)| label == "-progressive -arithmetic" || label.starts_with("-icc")),
    );
    for (label, data) in inputs {
        let end = header_length(&data) + 4;
        // Every length of testorig.jpg. The variants' headers hold whole ICC
        // profiles (test1.icc spans nine APP2 markers), and a cut inside one
        // makes the memory source insert its fake EOI once per two bytes it
        // is asked to skip, up to 32 767 times. They are cut at every length
        // of their first 48 bytes (SOI, APP0 and the first APP2 header) and
        // last 700 bytes (the tables, SOF and SOS after the profile), and at
        // 24 points in between.
        let end = end.min(data.len());
        let lengths: Vec<usize> = if label == "testorig.jpg" || end <= 748 {
            (1..=end).collect()
        } else {
            let mut lengths: Vec<usize> = (1..=48).collect();
            let stride = (end - 748).div_ceil(24).max(1);
            lengths.extend((48..end - 700).step_by(stride).skip(1));
            lengths.extend(end - 700..=end);
            lengths
        };
        for length in lengths {
            let args: &[&str] = if length % 3 == 0 {
                &["-save", "226", "65535", "-trace", "1"]
            } else {
                &["-trace", "1"]
            };
            compare(
                &oracle,
                args,
                &data[..length],
                &format!("{label}[..{length}]"),
            );
        }
    }
}

#[test]
fn mutations_match_the_oracle() {
    let Some(oracle) = Oracle::locate("mutations_match_the_oracle") else {
        return;
    };
    let data = image("testorig.jpg");
    let end = header_length(&data);
    for position in 0..end {
        let original = data[position];
        for value in [0x00, 0xFF, original ^ 0x55, original.wrapping_add(1)] {
            if value == original {
                continue;
            }
            let mut mutated = data.clone();
            mutated[position] = value;
            compare(
                &oracle,
                &["-trace", "1", "-save", "254", "100"],
                &mutated,
                &format!("testorig.jpg[{position}]={value:#04x}"),
            );
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
        label.starts_with("-icc") || label.starts_with("-progressive") || label == "-sample 3x2"
    }));
    for (label, data) in &inputs {
        for step in ["1", "2", "3", "13", "64", "100000"] {
            compare(&oracle, &["-suspend", step, "-trace", "1"], data, label);
            compare(
                &oracle,
                &[
                    "-suspend", step, "-save", "226", "30", "-save", "254", "0", "-icc",
                ],
                data,
                label,
            );
        }
        // Suspending input that ends early.
        let short = &data[..header_length(data) / 2];
        compare(&oracle, &["-suspend", "5", "-trace", "1"], short, label);
    }
}

#[test]
fn api_sequences_match_the_oracle() {
    let Some(oracle) = Oracle::locate("api_sequences_match_the_oracle") else {
        return;
    };
    let data = image("testorig.jpg");
    let size = wp2_struct_size(&oracle);
    for args in [
        vec!["-create".to_owned(), "61".to_owned(), size.clone()],
        vec!["-create".to_owned(), "62".to_owned(), "100".to_owned()],
        vec!["-create".to_owned(), "62".to_owned(), size],
        vec!["-again".to_owned()],
        vec!["-abort".to_owned(), "-again".to_owned()],
        vec!["-consume".to_owned(), "3".to_owned(), "-query".to_owned()],
        vec!["-save".to_owned(), "200".to_owned(), "1".to_owned()],
        vec!["-processor".to_owned(), "216".to_owned()],
        vec![
            "-save".to_owned(),
            "254".to_owned(),
            "4000000000".to_owned(),
        ],
    ] {
        let args: Vec<&str> = args.iter().map(String::as_str).collect();
        compare(&oracle, &args, &data, "testorig.jpg");
    }
    // Empty input fails in jpeg_mem_src.
    compare(&oracle, &[], &[], "empty input");
}

/// The C size of the decompression object, as the probe prints it.
fn wp2_struct_size(oracle: &Oracle) -> String {
    let fields = oracle
        .probe("wp2", "header")
        .run::<&str>(&[], &image("testorig.jpg"))
        .success()
        .fields();
    fields
        .get("sizeof_jpeg_decompress_struct")
        .unwrap()
        .to_owned()
}

#[test]
fn lossless_files_are_out_of_scope() {
    // Lossless JPEG is out of scope (ADR 0001): with D_LOSSLESS_SUPPORTED
    // treated as undefined, the port checks a lossless image's precision as
    // a lossy one's, so a 16-bit lossless header fails with
    // JERR_BAD_PRECISION where the oracle reads it. Not compared.
    let Some(oracle) = Oracle::locate("lossless_files_are_out_of_scope") else {
        return;
    };
    let directory = std::env::temp_dir().join(format!("libjpeg-rs-wp2-ll-{}", std::process::id()));
    std::fs::create_dir_all(&directory).unwrap();
    let output: PathBuf = directory.join("lossless16.jpg");
    let status = Command::new(oracle.tool("cjpeg"))
        .args(["-precision", "16", "-lossless", "1", "-outfile"])
        .arg(&output)
        .arg(oracle::test_image("monkey16.pgm"))
        .status()
        .unwrap();
    assert!(status.success());
    let data = std::fs::read(&output).unwrap();
    let _ = std::fs::remove_dir_all(Path::new(&directory));
    let expected = oracle
        .probe("wp2", "header")
        .run::<&str>(&[], &data)
        .success()
        .fields();
    assert_eq!(expected.int("error"), 0);
    let actual = wp2::run_rust(&[], &data);
    assert_eq!(
        actual.get("error_message"),
        Some("Unsupported JPEG data precision 16")
    );
}
