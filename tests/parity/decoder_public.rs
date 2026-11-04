// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Public decoder histories against the original public C API of the complete
// archive. Inputs are genuine files from the original public encoder.
#[path = "../support/decoder_public.rs"]
mod driver;

pub(super) fn handles(kind: &str) -> bool {
    kind == "dpub-run"
}

pub(super) fn observe(_kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    driver::observe(args)
}

/// The UBSan diagnostic of the original's raw-box memcpy with a null input.
pub(super) const NULL_COPY_DIAGNOSTIC: &str = "box_content_decoder.cc:110:23: runtime error: \
     null pointer passed as argument 2, which is declared to never be null";

/// Compares one family and retains its requests for the independent source
/// audit when JXL_DECODER_PUBLIC_CAPTURE names a directory.
///
/// A history whose ProcessInput reaches the original's raw-box memcpy with a
/// null input (the caller released its input, so `next_in` is null and zero
/// bytes are copied) makes an undefined source call. The Rust port reports
/// each such call through a test observation; those histories run
/// original-only, and every other history compares exactly.
pub(super) fn compare(family: &str, requests: Vec<String>) {
    let (undefined, defined): (Vec<String>, Vec<String>) = requests.into_iter().partition(|r| {
        jxl::jxl::box_content_decoder::take_null_input_copies();
        super::observe(r);
        jxl::jxl::box_content_decoder::take_null_input_copies() != 0
    });
    capture(family, &defined, &undefined);
    super::compare(defined);
    null_copy_probes(&undefined);
}

/// Retains a family's compared and undefined requests for the audit.
fn capture(family: &str, defined: &[String], undefined: &[String]) {
    if let Some(path) = std::env::var_os("JXL_DECODER_PUBLIC_CAPTURE") {
        use std::io::Write;
        let path = std::path::PathBuf::from(path);
        std::fs::create_dir_all(&path).unwrap();
        for (suffix, list) in [("requests", defined), ("undefined", undefined)] {
            let mut output = std::io::BufWriter::new(
                std::fs::File::create(path.join(format!("{family}.{suffix}"))).unwrap(),
            );
            for request in list {
                writeln!(output, "{request}").unwrap();
            }
        }
    }
}

/// Runs each undefined history in its own original process. Instrumented
/// Linux graphs must report exactly the null memcpy argument; the macOS libc
/// declares no nonnull memcpy arguments, so there the probe must complete.
pub(super) fn null_copy_probes(requests: &[String]) {
    use std::io::Write;
    use std::process::{Command, Stdio};
    if requests.is_empty() {
        return;
    }
    let instrumented = super::original_observations(&["patch-sanitizers".into()])[0] == "1";
    let oracle = std::env::var_os("JXL_CPP_ORACLE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| {
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/oracle/jxl_oracle")
        });
    let detects = instrumented && cfg!(target_os = "linux");
    for request in requests {
        let mut child = Command::new(&oracle)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("start original null-copy probe");
        writeln!(child.stdin.take().unwrap(), "{request}").expect("write the exact request");
        let output = child
            .wait_with_output()
            .expect("observe the original probe");
        let stderr = String::from_utf8_lossy(&output.stderr);
        if detects {
            assert!(
                !output.status.success()
                    && stderr.contains(NULL_COPY_DIAGNOSTIC)
                    && stderr.matches("runtime error:").count() == 1,
                "instrumented original must diagnose the null raw-box copy: {request}\n{stderr}"
            );
        } else {
            assert!(
                output.status.success() && stderr.is_empty(),
                "the original null-copy probe must complete here: {request}\n{stderr}"
            );
        }
    }
    println!(
        "{} original-only null raw-box copy histories; diagnosed={detects}",
        requests.len()
    );
}

/// Genuine inputs: raw codestreams and containers, with metadata, compressed
/// (brob) boxes, level and frame-index boxes and partial codestream splits.
const SPECS: &[&str] = &[
    "w=8 h=8 c=3",
    "w=13 h=7 c=1 bits=16",
    "w=8 h=8 c=3 icc=1",
    "w=9 h=5 c=1 icc=1",
    "w=8 h=8 c=3 container=1",
    "w=8 h=8 c=3 container=1 box=Exif:20:0 box=xml:30:1 box=jumb:12:0 icc=1",
    "w=16 h=12 c=3 a=1 ec=2 names=1 orient=6 intrinsic=1 container=1 box=Exif:40:1",
    "w=12 h=10 c=3 frames=3 duration=5 timecodes=1 container=1 index=1",
    "w=12 h=10 c=3 frames=2 duration=3 container=1 box=xml:64:0 late=Exif:24:1 index=1",
    "w=8 h=8 c=3 container=1 level=10",
    "w=40 h=30 c=3 lossless=0 container=1 box=Exif:16:0 late=jumb:20:1",
    "w=2305 h=8 c=3 lossless=0 container=1 mode=2 box=xml:10:0",
    "w=8 h=8 c=3 a=1 premul=1 orient=3",
];

const EVENTS: &[i32] = &[
    0, 64, 256, 320, 16384, 16448, 16640, 16704, 65536, 81920, 81984, 82176, 82240,
];

fn files() -> Vec<String> {
    super::original_observations(
        &SPECS
            .iter()
            .map(|spec| format!("dpub-encode {spec}"))
            .collect::<Vec<_>>(),
    )
}

fn events_requests() -> Vec<String> {
    let files = files();
    let mut requests = Vec::new();
    for (index, file) in files.iter().enumerate() {
        for (e, events) in EVENTS.iter().enumerate() {
            for (seed, max) in [(1, 1), (5, 7), (9, 64), (3, 1 << 20)] {
                let boxbuf = [0, 1, 3, 8, 64, 4096][(index + e + max) % 6];
                let decompress = (index + e) % 2;
                let close = (seed as usize + e) % 2;
                requests.push(format!(
                    "dpub-run 0 0 {file} s{events} d{decompress} L{seed},{max},{boxbuf},100000,{close}"
                ));
            }
        }
    }
    requests
}

#[test]
fn public_decoder_header_and_box_events_match_original() {
    compare("events", events_requests());
}

fn box_buffers_requests() -> Vec<String> {
    let files = files();
    let mut requests = Vec::new();
    for file in files.iter().skip(4) {
        for boxbuf in [1, 2, 5, 11, 16, 33, 200] {
            for decompress in 0..2 {
                for close in 0..2 {
                    for events in [16384, 81920, 82240] {
                        requests.push(format!(
                            "dpub-run 2 0 {file} s{events} d{decompress} L{boxbuf},13,{boxbuf},100000,{close}"
                        ));
                    }
                }
            }
        }
    }
    requests
}

#[test]
fn public_decoder_box_buffers_and_close_match_original() {
    compare("box-buffers", box_buffers_requests());
}

fn truncated_requests() -> Vec<String> {
    let files = files();
    let mut requests = Vec::new();
    let mut seed = 77u64;
    for file in &files {
        let bytes = driver::unhex(file);
        let mut cuts: Vec<usize> = (0..bytes.len().min(64)).collect();
        cuts.extend((64..bytes.len()).step_by(37.max(bytes.len() / 48)));
        for cut in cuts {
            let truncated = driver::hex(&bytes[..cut]);
            for events in [320, 16704, 82240] {
                requests.push(format!(
                    "dpub-run 0 0 {truncated} s{events} L{cut},9,8,100000,1"
                ));
            }
        }
        for _ in 0..48 {
            let mut mutated = bytes.clone();
            let position = (super::next(&mut seed) as usize) % mutated.len();
            mutated[position] ^= 1 << (super::next(&mut seed) % 8);
            if super::next(&mut seed).is_multiple_of(3) {
                let position = (super::next(&mut seed) as usize) % mutated.len();
                mutated[position] = super::next(&mut seed) as u8;
            }
            let mutated = driver::hex(&mutated);
            for events in [320, 82240] {
                requests.push(format!(
                    "dpub-run 0 0 {mutated} s{events} d1 L{position},11,5,100000,1"
                ));
            }
        }
    }
    requests
}

#[test]
fn public_decoder_truncated_and_mutated_inputs_match_original() {
    compare("truncated", truncated_requests());
}

fn misuse_requests() -> Vec<String> {
    let files = files();
    let mut requests = Vec::new();
    let scripts = [
        // Subscription and option timing and masks.
        "s1 s63 s64 p s64 k1 u1 r0 g0 d1",
        "s16448 I0,4 p s64 k0 u0 r1 g1",
        "s320 I0,0 I0,1 i i c I0,1 p",
        "I- I- i I0,2 c I0,2 i p p",
        // Getters before their information is available.
        "b b0 e0 n0,9 E0 E1 z0 Z0,4 t0 t1 y y0 Y Y0 h",
        // Getters after headers, including undersized and null outputs.
        "s320 L1,1000000,0,2,0 b b0 e0 e9 n0,0 n0,1 E0 E0- E1 E1- z0 z0- z1 Z0,0 Z0,1 Z1,4096 h",
        "s320 L1,1000000,0,3,0 b E0 E1 z0 z1 Z0,16 Z1,100000 Z0,100000",
        // Box buffers without an event, twice, and after release.
        "s16384 O4 o o L1,1000000,0,4,0 t0 t1 y Y O4 O4 o O0 o",
        "s16384 L1,1000000,0,1,0 O4 p o O2 p o O9 p p o L1,1000000,6,100000,1",
        // Errors persist only as the original reports them.
        "s320 I0,3 p p i I0,9 p b p p",
    ];
    for (index, file) in files.iter().enumerate() {
        for script in scripts {
            requests.push(format!("dpub-run {} 0 {file} {script}", index % 2 * 2));
        }
    }
    requests
}

#[test]
fn public_decoder_api_misuse_matches_original() {
    compare("misuse", misuse_requests());
}

fn rewind_requests() -> Vec<String> {
    let files = files();
    let mut requests = Vec::new();
    for file in &files {
        for steps in [0, 1, 2, 3, 5, 8, 13, 40] {
            for (events, box_events) in [(320, 16384), (16704, 82240), (64, 0)] {
                requests.push(format!(
                    "dpub-run 0 0 {file} s{events} d1 L7,9,6,{steps},0 w L7,9,6,100000,1 b h"
                ));
                requests.push(format!(
                    "dpub-run 0 0 {file} s{events} L3,5,3,{steps},1 x s{box_events} L3,5,3,100000,1 b"
                ));
                requests.push(format!(
                    "dpub-run 0 0 {file} s{events} k1 L2,17,0,{steps},0 w w x x k1 s{events} L2,17,0,100000,1 b E0"
                ));
            }
        }
    }
    requests
}

#[test]
fn public_decoder_rewind_and_reset_mid_stream_match_original() {
    compare("rewind", rewind_requests());
}

fn allocations_requests() -> Vec<String> {
    // The custom manager's complete event log and the first injected
    // allocation failures, for decoder, input and (with the color encoding
    // event, s320) ICC reader storage.
    let files = files();
    let mut requests = Vec::new();
    for file in &files {
        for fail in 0..12 {
            for (seed, max) in [(1, 1), (2, 7), (5, 1 << 20)] {
                for events in [64, 320, 16448, 81984] {
                    requests.push(format!(
                        "dpub-run 1 {fail} {file} s{events} d1 L{seed},{max},4,100000,1"
                    ));
                }
            }
            requests.push(format!(
                "dpub-run 1 {fail} {file} s16448 L4,3,2,9,0 w L4,3,2,100000,1"
            ));
        }
    }
    requests
}

#[test]
fn public_decoder_input_allocations_match_original() {
    compare("allocations", allocations_requests());
}

fn segment(marker: u8, payload: &[u8]) -> Vec<u8> {
    let length = payload.len() + 2;
    let mut s = vec![0xff, marker, (length >> 8) as u8, length as u8];
    s.extend_from_slice(payload);
    s
}

/// Insert whole marker segments after SOI and an initial APP0 segment.
fn with_segments(jpeg: &[u8], segments: &[Vec<u8>]) -> Vec<u8> {
    let mut pos = 2;
    if jpeg[2..4] == [0xff, 0xe0] {
        pos += 2 + usize::from(u16::from_be_bytes([jpeg[4], jpeg[5]]));
    }
    let mut out = jpeg[..pos].to_vec();
    for s in segments {
        out.extend_from_slice(s);
    }
    out.extend_from_slice(&jpeg[pos..]);
    out
}

/// Genuine JPEG reconstruction containers from the original encoder: jbrd,
/// partial codestream, and uncompressed or brob Exif/XMP boxes.
fn jpeg_files() -> Vec<String> {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/data");
    let exif = {
        let mut p = b"Exif\0\0".to_vec();
        p.extend_from_slice(&[0x49, 0x49, 0x2a, 0, 8, 0, 0, 0, 1, 0]);
        p.extend_from_slice(&[0x12, 0x01, 3, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0]);
        p.extend((0..29).map(|i| (i * 7 + 3) as u8));
        p
    };
    let xmp = {
        let mut p = b"http://ns.adobe.com/xap/1.0/\0".to_vec();
        p.extend_from_slice(b"<x:xmpmeta xmlns:x='adobe:ns:meta/'>public decoder</x:xmpmeta>");
        p
    };
    let mut requests = Vec::new();
    for path in [
        "jpeg-writer/baseline.jpg",
        "jpeg-writer/gray.jpg",
        "jpeg-writer/progressive.jpg",
    ] {
        let jpeg = std::fs::read(root.join(path)).unwrap();
        for segments in [
            vec![],
            vec![segment(0xe1, &exif)],
            vec![segment(0xe1, &xmp)],
            vec![
                segment(0xe1, &exif),
                segment(0xe1, &xmp),
                segment(0xfe, b"comment"),
            ],
        ] {
            for compress in 0..2 {
                let input = with_segments(&jpeg, &segments);
                requests.push(format!("jtj-encode {} 7 {compress}", driver::hex(&input)));
            }
        }
    }
    super::original_observations(&requests)
        .into_iter()
        .map(|observed| {
            let (prefix, wire) = observed.rsplit_once(' ').unwrap();
            assert_eq!(prefix, "API 0 0 0 0 0 W 0 0");
            wire.to_owned()
        })
        .collect()
}

fn jpeg_requests() -> Vec<String> {
    let files = jpeg_files();
    let mut requests = Vec::new();
    let mut seed = 31u64;
    for (index, file) in files.iter().enumerate() {
        for events in [8192, 8256, 8512, 24576, 90112, 24640] {
            for (chunk_seed, max) in [(1, 1), (4, 23), (9, 1 << 20)] {
                let boxbuf = [0, 3, 64][(index + chunk_seed) % 3];
                requests.push(format!(
                    "dpub-run 0 0 {file} s{events} d{} J{} L{chunk_seed},{max},{boxbuf},100000,{} j",
                    index % 2,
                    index * 13 % 50,
                    chunk_seed % 2
                ));
            }
        }
        // Buffer misuse and a second jbrd subscription across Rewind/Reset.
        requests.push(format!(
            "dpub-run 2 0 {file} j J4 J4 j j s8192 L1,40,0,3,0 J0 w L1,40,0,100000,1 x s8256 L2,9,0,100000,1"
        ));
        let bytes = driver::unhex(file);
        for _ in 0..12 {
            let mut mutated = bytes.clone();
            let position = (super::next(&mut seed) as usize) % mutated.len().min(400);
            mutated[position] ^= 1 << (super::next(&mut seed) % 8);
            requests.push(format!(
                "dpub-run 0 0 {} s24640 d1 L{position},17,5,100000,1",
                driver::hex(&mutated)
            ));
            let cut = (super::next(&mut seed) as usize) % bytes.len();
            requests.push(format!(
                "dpub-run 0 0 {} s8256 L{cut},7,0,100000,1",
                driver::hex(&bytes[..cut])
            ));
        }
    }
    requests
}

#[test]
fn public_decoder_jpeg_reconstruction_boxes_match_original() {
    compare("jpeg", jpeg_requests());
}

/// Requested output encodings as cs.wp.prim.tf.gamma-bits.ri tokens,
/// including every source rejection: unknown and XYB color spaces, invalid
/// gamma and transfer values.
fn output_encodings() -> Vec<String> {
    let gamma = |g: f64| g.to_bits();
    vec![
        "0.1.1.13.0.0".into(),
        "1.1.1.13.0.0".into(),
        "0.1.1.8.0.1".into(),
        "0.1.11.13.0.0".into(),
        "0.1.9.16.0.0".into(),
        "0.1.9.18.0.2".into(),
        format!("0.1.1.65535.{}.0", gamma(1.0 / 2.2)),
        format!("1.1.1.65535.{}.3", gamma(0.45)),
        "0.2.2.1.0.3".into(),
        "0.11.11.17.0.0".into(),
        "1.2.1.8.0.0".into(),
        "2.1.1.13.0.0".into(),
        "3.1.1.13.0.0".into(),
        "0.1.1.65535.0.0".into(),
        "0.1.1.7.0.0".into(),
    ]
}

const COLOR_SPECS: &[&str] = &[
    "w=8 h=8 c=3",
    "w=8 h=8 c=1 bits=16",
    "w=8 h=8 c=3 icc=1",
    "w=9 h=5 c=1 icc=1",
    "w=8 h=8 c=3 lossless=0",
    "w=8 h=8 c=1 lossless=0",
    "w=8 h=8 c=3 icc=1 lossless=0",
];

fn color_requests() -> Vec<String> {
    let files = super::original_observations(
        &COLOR_SPECS
            .iter()
            .map(|spec| format!("dpub-encode {spec}"))
            .collect::<Vec<_>>(),
    );
    // Original embedded profiles, read back through the original public API.
    let mut profiles = Vec::new();
    for file in [&files[2], &files[3]] {
        let size = super::original_observations(&[format!(
            "dpub-run 0 0 {file} s256 L1,1000000,0,2,0 z0"
        )])
        .remove(0);
        let size: usize = size
            .rsplit_once(',')
            .unwrap()
            .1
            .split(' ')
            .next()
            .unwrap()
            .parse()
            .unwrap();
        let icc = super::original_observations(&[format!(
            "dpub-run 0 0 {file} s256 L1,1000000,0,2,0 Z0,{size}"
        )])
        .remove(0);
        let hex = icc
            .split(" Z:0,")
            .nth(1)
            .unwrap()
            .split(' ')
            .next()
            .unwrap();
        profiles.push(hex[..hex.len() - 2].to_owned());
    }
    profiles.push(driver::hex(
        &(0..64).map(|i| (i * 37 + 5) as u8).collect::<Vec<_>>(),
    ));
    profiles.push("-".into());
    let encodings = output_encodings();
    let headers = "s256 L1,1000000,0,2,0";
    let mut requests = Vec::new();
    for file in &files {
        for encoding in &encodings {
            requests.push(format!("dpub-run 0 0 {file} {headers} P{encoding} G"));
            requests.push(format!(
                "dpub-run 0 0 {file} C {headers} P{encoding} G P{encoding} G"
            ));
            requests.push(format!(
                "dpub-run 0 0 {file} {headers} C P{encoding} P0.1.1.13.0.0 G"
            ));
        }
        for profile in &profiles {
            requests.push(format!("dpub-run 0 0 {file} {headers} Q{profile} G"));
            requests.push(format!("dpub-run 0 0 {file} {headers} C Q{profile} G"));
            requests.push(format!(
                "dpub-run 0 0 {file} C {headers} R0.1.1.13.0.0,{profile} G"
            ));
        }
        for bits in [
            0u32,
            0x3f80_0000,
            0x447a_0000,
            0xbf80_0000,
            0x7fc0_0000,
            0x8000_0000,
        ] {
            requests.push(format!(
                "dpub-run 0 0 {file} T{bits} s320 L1,1000000,0,3,0 G"
            ));
            requests.push(format!(
                "dpub-run 0 0 {file} s320 L1,1000000,0,2,0 T{bits} G L1,1000000,0,2,0 G"
            ));
        }
        // Timing errors: before the headers, after the frame loop began, and
        // after Rewind and Reset.
        requests.push(format!(
            "dpub-run 0 0 {file} P0.1.1.8.0.0 C P- s16640 L1,1000000,0,2,0 P0.1.1.8.0.0 L1,1000000,0,100,1 P0.1.1.8.0.0 G"
        ));
        requests.push(format!(
            "dpub-run 2 0 {file} C {headers} P0.1.9.16.0.0 w P0.1.1.8.0.0 {headers} G x {headers} P0.1.1.8.0.0 C P0.1.1.8.0.0 G"
        ));
    }
    requests
}

#[test]
fn public_decoder_color_profile_setters_match_original() {
    compare("color", color_requests());
}

/// Genuine inputs for frame decoding: original public encoder outputs with
/// progressive, layered, resampled, filtered and effect frames.
const FRAME_SPECS: &[&str] = &[
    "w=8 h=8 c=3",
    "w=13 h=7 c=1 bits=16",
    "w=16 h=12 c=3 a=1 ec=2 names=1 orient=6 intrinsic=1 container=1 box=Exif:40:1",
    "w=12 h=10 c=3 frames=3 duration=5 timecodes=1 container=1 index=1",
    "w=40 h=30 c=3 lossless=0",
    "w=8 h=8 c=3 a=1 premul=1 orient=3",
    "w=20 h=14 c=3 a=1 frames=4 layers=1 container=1",
    "w=20 h=14 c=3 frames=4 layers=1 duration=2",
    "w=300 h=280 c=3 lossless=0 opt=16:1",
    "w=300 h=280 c=3 lossless=0 opt=17:1 opt=18:1",
    "w=300 h=280 c=3 lossless=0 opt=19:1",
    "w=64 h=48 c=3 lossless=0 opt=2:2",
    "w=64 h=48 c=3 lossless=0 opt=6:1",
    "w=64 h=64 c=3 opt=8:1",
    "w=64 h=48 c=3 lossless=0 opt=9:3 opt=10:1",
    "w=8 h=8 c=3 icc=1",
    "w=8 h=8 c=3 icc=1 lossless=0",
    "w=8 h=8 c=1 lossless=0",
    "w=2305 h=8 c=3 lossless=0 container=1 mode=2 box=xml:10:0",
];

/// Original encoder outputs retained in tests/data (see its README).
fn retained_files() -> Vec<String> {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/data");
    let mut paths: Vec<_> = std::fs::read_dir(&root)
        .unwrap()
        .chain(std::fs::read_dir(root.join("decoder-geometry")).unwrap())
        .map(|entry| entry.unwrap().path())
        .filter(|path| {
            path.extension().is_some_and(|e| e == "jxl")
                && path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .is_some_and(|n| !n.starts_with("sampling-") || n.contains("13x11"))
        })
        .collect();
    paths.sort();
    paths
        .into_iter()
        .map(|path| driver::hex(&std::fs::read(path).unwrap()))
        .collect()
}

fn frames_requests() -> Vec<String> {
    let files = super::original_observations(
        &FRAME_SPECS
            .iter()
            .map(|spec| format!("dpub-encode {spec}"))
            .collect::<Vec<_>>(),
    );
    // Output formats: channels.type.endianness.align.mode, with types FLOAT=0,
    // UINT8=2, UINT16=3, FLOAT16=5; mode 0 buffer, 1 simple callback, 2
    // multithreaded callback, 3 no output.
    let scripts = [
        "s4096 F4.2.0.0.0 L1,1000000,0,1000,1",
        "s5184 F3.2.0.0.0 V2 L3,17,0,1000,1",
        "s5120 F4.3.1.0.1 L2,64,0,1000,1",
        "s5120 F3.0.2.8.2 L1,1,0,100000,1",
        "s5120 F1.5.0.0.0 V3 L5,9,0,1000,1",
        "s5120 g0 F4.2.0.0.0 V1 L4,33,0,1000,1",
        "s5120 k1 u1 r0 F4.3.0.0.0 L6,1000000,0,1000,1",
        "s5120 S1 F3.2.0.0.0 L7,29,0,1000,1",
        "s5120 M1 F3.2.0.0.0 L8,1000000,0,1000,1",
        "s5120 F3.3.0.0.0 W1.0.0 L9,1000000,0,1000,1",
        "s5120 F3.3.0.0.0 W2.12.0 L10,1000000,0,1000,1",
        "s5120 F3.2.0.0.0 W2.12.0 L10,1000000,0,1000,1",
        "s1024 L11,40,0,1000,1",
        "s5120 F3.2.0.0.3 L12,1000000,0,1000,1",
        "s38400 D1 M2 F3.2.0.0.0 L1,97,0,1000,1",
        "s38400 D2 M2 F3.3.0.0.2 L2,211,0,1000,1",
        "s38400 D3 M6 F4.2.0.0.0 L3,331,0,1000,1",
        "s38400 D4 L4,1000000,0,10,1",
        "s5632 F3.2.0.0.0 L5,1000000,0,1000,1",
        "s5120 F3.2.0.0.0 L1,1000000,0,4,0 w L2,7,0,1000,1",
        "s5120 F3.2.0.0.0 L1,1000000,0,4,0 K f x s5120 L2,7,0,1000,1",
    ];
    let mut requests = Vec::new();
    for file in &files {
        for script in scripts {
            requests.push(format!("dpub-run 0 0 {file} {script}"));
        }
    }
    for file in retained_files() {
        for script in [scripts[0], scripts[5], scripts[18]] {
            requests.push(format!("dpub-run 0 0 {file} {script}"));
        }
    }
    requests
}

#[test]
fn public_decoder_frame_events_and_outputs_match_original() {
    compare("frames", frames_requests());
}

fn frame_mutations_requests() -> Vec<String> {
    let files = super::original_observations(
        &FRAME_SPECS
            .iter()
            .map(|spec| format!("dpub-encode {spec}"))
            .collect::<Vec<_>>(),
    );
    let encodings = output_encodings();
    let mut requests = Vec::new();
    let mut seed = 97u64;
    for file in &files {
        let bytes = driver::unhex(file);
        for _ in 0..6 {
            let cut = (super::next(&mut seed) as usize) % bytes.len();
            requests.push(format!(
                "dpub-run 0 0 {} s5120 F3.2.0.0.0 L{cut},13,0,1000,1",
                driver::hex(&bytes[..cut])
            ));
            let mut mutated = bytes.clone();
            let position = (super::next(&mut seed) as usize) % mutated.len();
            mutated[position] ^= 1 << (super::next(&mut seed) % 8);
            requests.push(format!(
                "dpub-run 0 0 {} s5120 F3.2.0.0.0 L{position},13,0,1000,1",
                driver::hex(&mutated)
            ));
        }
        for encoding in &encodings {
            requests.push(format!(
                "dpub-run 0 0 {file} C s4352 L1,1000000,0,2,0 P{encoding} F3.0.0.0.0 L2,1000000,0,1000,1 G"
            ));
        }
        requests.push(format!(
            "dpub-run 0 0 {file} C s4352 L1,1000000,0,2,0 T1148846080 F4.0.0.0.0 L2,1000000,0,1000,1"
        ));
    }
    requests
}

#[test]
fn public_decoder_frame_truncation_and_cms_match_original() {
    compare("frame-mutations", frame_mutations_requests());
}

fn jpeg_output_requests() -> Vec<String> {
    let files = jpeg_files();
    let mut requests = Vec::new();
    for file in &files {
        for (events, jpeg) in [(12288, 100000), (12288, 64), (12352, 7), (28672, 4096)] {
            requests.push(format!(
                "dpub-run 0 0 {file} s{events} J{jpeg} F3.2.0.0.0 L1,53,32,1000,1 j"
            ));
        }
        requests.push(format!(
            "dpub-run 0 0 {file} s4096 F3.2.0.0.0 L2,1000000,0,1000,1"
        ));
        requests.push(format!(
            "dpub-run 0 0 {file} s12288 L3,1000000,0,1000,1 J100000 L3,1000000,0,1000,1 j"
        ));
    }
    requests
}

#[test]
fn public_decoder_jpeg_output_matches_original() {
    compare("jpeg-output", jpeg_output_requests());
}

/// The same history with the logging custom manager and no injected failure.
fn with_logged_manager(request: &str) -> String {
    let mut fields = request.splitn(4, ' ');
    assert_eq!(fields.next(), Some("dpub-run"));
    let (_manager, _fail) = (fields.next(), fields.next());
    format!("dpub-run 1 0 {}", fields.next().unwrap())
}

/// Like `compare`, but observes each Rust history once. A Rust panic marks a
/// history in which the original dereferences a null buffer (see
/// `allocation_failures`); those and the undefined raw-box copies run
/// original-only, and every other history compares exactly.
fn compare_observed(family: &str, requests: Vec<String>) {
    let mut defined = Vec::new();
    let mut observed = Vec::new();
    let mut undefined = Vec::new();
    let mut null_buffer = Vec::new();
    for request in requests {
        jxl::jxl::box_content_decoder::take_null_input_copies();
        let observation =
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| super::observe(&request)));
        if jxl::jxl::box_content_decoder::take_null_input_copies() != 0 {
            undefined.push(request);
        } else if let Ok(observation) = observation {
            observed.push(observation);
            defined.push(request);
        } else {
            null_buffer.push(request);
        }
    }
    capture(family, &defined, &undefined);
    let originals = super::original_observations(&defined);
    for ((request, rust), cpp) in defined.iter().zip(&observed).zip(originals) {
        assert_eq!(rust, &cpp, "request: {request}");
    }
    println!("{} C++/Rust differential cases matched", defined.len());
    null_copy_probes(&undefined);
    null_buffer_probes(&null_buffer);
}

/// Every other family again with the logging custom manager. The decoder,
/// its input, the ICC reader, the codec state, the image bundle and the frame
/// owners allocate through it, so every request size and the complete
/// allocation and free order compare.
#[test]
fn public_decoder_custom_manager_logs_match_original() {
    let requests = [
        events_requests(),
        box_buffers_requests(),
        truncated_requests(),
        misuse_requests(),
        rewind_requests(),
        jpeg_requests(),
        color_requests(),
        frames_requests(),
        frame_mutations_requests(),
        jpeg_output_requests(),
    ]
    .concat()
    .iter()
    .map(|request| with_logged_manager(request))
    .collect();
    compare_observed("custom-manager", requests);
}

/// The number of allocation calls in an observation's custom-manager log.
fn allocation_calls(observation: &str) -> usize {
    let log = observation.rsplit_once(" E ").map_or("", |(_, log)| log);
    log.split(' ')
        .filter(|event| event.starts_with('A'))
        .count()
}

/// Fails each allocation call of every base history in turn, from the decoder
/// object to the last frame owner; statuses, outputs and the complete
/// allocation and free log compare.
///
/// When a failed allocation leaves a render-pipeline or group-cache buffer
/// empty and the caller continues (here after FlushImage failed), the original
/// later dereferences that null buffer, which is undefined. The Rust port
/// panics at the same use instead. Those histories run original-only.
fn allocation_failures(family: &str, bases: Vec<String>) {
    let observations = super::original_observations(&bases);
    let mut requests = Vec::new();
    for (base, observation) in bases.iter().zip(&observations) {
        let history = base.strip_prefix("dpub-run 1 0 ").unwrap();
        for fail in 1..=allocation_calls(observation) {
            requests.push(format!("dpub-run 1 {fail} {history}"));
        }
    }
    compare_observed(family, requests);
}

fn frame_files() -> Vec<String> {
    super::original_observations(
        &FRAME_SPECS
            .iter()
            .map(|spec| format!("dpub-encode {spec}"))
            .collect::<Vec<_>>(),
    )
}

/// Whole-frame decoding of every frame input and retained original encoding.
#[test]
fn public_decoder_frame_allocation_failures_match_original() {
    let bases = frame_files()
        .iter()
        .chain(&retained_files())
        .map(|file| format!("dpub-run 1 0 {file} s4096 F4.2.0.0.0 L1,1000000,0,1000,1"))
        .collect();
    allocation_failures("frame-allocation-failures", bases);
}

/// Progressive flushing, frame skipping, rewinding and resetting. The three
/// 300x280 inputs, whose whole-frame failures are compared above, are left out
/// to bound the run time.
#[test]
fn public_decoder_progressive_allocation_failures_match_original() {
    let mut bases = Vec::new();
    for (spec, file) in FRAME_SPECS.iter().zip(frame_files()) {
        if spec.starts_with("w=300") {
            continue;
        }
        for script in [
            "s38400 D1 M2 F3.2.0.0.0 L1,97,0,1000,1",
            "s5120 M1 F3.2.0.0.0 L8,1000000,0,1000,1",
            "s5120 F3.2.0.0.0 L1,1000000,0,4,0 w L2,7,0,1000,1",
            "s5120 F3.2.0.0.0 L1,1000000,0,4,0 K f x s5120 L2,7,0,1000,1",
        ] {
            bases.push(format!("dpub-run 1 0 {file} {script}"));
        }
    }
    allocation_failures("progressive-allocation-failures", bases);
}

/// Headers with the ICC profile, in one and in small chunks, JPEG
/// reconstruction and CMS output conversion.
#[test]
fn public_decoder_header_and_jpeg_allocation_failures_match_original() {
    let mut bases = Vec::new();
    for file in files() {
        for script in ["s320 d1 L5,1048576,4,100000,1", "s320 d1 L1,7,4,100000,1"] {
            bases.push(format!("dpub-run 1 0 {file} {script}"));
        }
    }
    for file in jpeg_files() {
        for script in [
            "s4096 F3.2.0.0.0 L2,1000000,0,1000,1",
            "s12288 L3,1000000,0,1000,1 J100000 L3,1000000,0,1000,1 j",
        ] {
            bases.push(format!("dpub-run 1 0 {file} {script}"));
        }
    }
    for file in &frame_files()[15..17] {
        for encoding in ["0.1.1.8.0.1", "0.1.9.16.0.0"] {
            bases.push(format!(
                "dpub-run 1 0 {file} C s4352 L1,1000000,0,2,0 P{encoding} F3.0.0.0.0 L2,1000000,0,1000,1 G"
            ));
        }
    }
    allocation_failures("header-allocation-failures", bases);
}

/// Runs each null-buffer history in its own original process, which must
/// terminate abnormally: a signal, or an instrumented graph's report.
fn null_buffer_probes(requests: &[String]) {
    use std::io::Write;
    use std::process::{Command, Stdio};
    let oracle = std::env::var_os("JXL_CPP_ORACLE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| {
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/oracle/jxl_oracle")
        });
    for request in requests {
        let mut child = Command::new(&oracle)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("start original null-buffer probe");
        writeln!(child.stdin.take().unwrap(), "{request}").expect("write the exact request");
        let output = child
            .wait_with_output()
            .expect("observe the original probe");
        assert!(
            !output.status.success(),
            "the original must not complete a null-buffer history: {request}"
        );
    }
    println!(
        "{} original-only null-buffer allocation-failure histories",
        requests.len()
    );
}
