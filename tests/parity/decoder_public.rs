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

/// Compares one family and retains its requests for the independent source
/// audit when JXL_DECODER_PUBLIC_CAPTURE names a directory.
fn compare(family: &str, requests: Vec<String>) {
    if let Some(path) = std::env::var_os("JXL_DECODER_PUBLIC_CAPTURE") {
        use std::io::Write;
        let path = std::path::PathBuf::from(path);
        std::fs::create_dir_all(&path).unwrap();
        let mut output = std::io::BufWriter::new(
            std::fs::File::create(path.join(format!("{family}.requests"))).unwrap(),
        );
        for request in &requests {
            writeln!(output, "{request}").unwrap();
        }
    }
    super::compare(requests);
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

#[test]
fn public_decoder_header_and_box_events_match_original() {
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
    compare("events", requests);
}

#[test]
fn public_decoder_box_buffers_and_close_match_original() {
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
    compare("box-buffers", requests);
}

#[test]
fn public_decoder_truncated_and_mutated_inputs_match_original() {
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
    compare("truncated", requests);
}

#[test]
fn public_decoder_api_misuse_matches_original() {
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
    compare("misuse", requests);
}

#[test]
fn public_decoder_rewind_and_reset_mid_stream_match_original() {
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
    compare("rewind", requests);
}

#[test]
fn public_decoder_input_allocations_match_original() {
    // The custom manager's complete event log is compared where the original
    // allocates only decoder and input storage: no ICC profile is read.
    let files = files();
    let mut requests = Vec::new();
    for (index, spec) in SPECS.iter().enumerate() {
        if spec.contains("icc=1") {
            continue;
        }
        let file = &files[index];
        for fail in 0..12 {
            for (seed, max) in [(1, 1), (2, 7), (5, 1 << 20)] {
                for events in [64, 16448, 81984] {
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
    compare("allocations", requests);
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

#[test]
fn public_decoder_jpeg_reconstruction_boxes_match_original() {
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
    compare("jpeg", requests);
}
