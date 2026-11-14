// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use super::*;
pub(super) fn handles(kind: &str) -> bool {
    matches!(kind, "jr-frame" | "jr-frame-mut")
}
pub(super) use super::jpeg_reconstruction_protocol::observe;
pub(super) fn compare_jpeg(requests: Vec<String>) {
    let originals = original_observations(&requests);
    let rust: Vec<_> = requests
        .iter()
        .map(|request| super::observe(request))
        .collect();
    for ((request, cpp), actual) in requests.iter().zip(&originals).zip(&rust) {
        assert_eq!(actual, cpp, "request: {request}");
    }
    if let Some(path) = std::env::var_os("JXL_JPEG_RECONSTRUCTION_CAPTURE") {
        static CAPTURE: std::sync::Mutex<()> = std::sync::Mutex::new(());
        let _lock = CAPTURE.lock().unwrap();
        let path = std::path::PathBuf::from(path);
        for (target, rows) in [
            (path.clone(), &requests),
            (path.with_extension("cpp"), &originals),
            (path.with_extension("rust"), &rust),
        ] {
            let mut file = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(target)
                .unwrap();
            std::io::Write::write_all(&mut file, (rows.join("\n") + "\n").as_bytes()).unwrap();
        }
    }
    println!("{} C++/Rust differential cases matched", requests.len());
}
fn fixture_pairs() -> Vec<(Vec<u8>, String, Vec<u8>)> {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data");
    include_str!("../data/jpeg-reconstruction/fixtures.tsv")
        .lines()
        .map(|line| line.split_once('\t').unwrap())
        .map(|(jpeg_path, jxl_path)| {
            let jpg = std::fs::read(root.join(jpeg_path)).unwrap();
            let bytes = std::fs::read(root.join(jxl_path)).unwrap();
            let parsed = original_observations(&[format!("jw-fixture {}", hex(&jpg))]);
            let (status, data) = parsed[0].split_once(' ').unwrap();
            assert_eq!(status, "0");
            (bytes, data.to_owned(), jpg)
        })
        .collect()
}
#[test]
fn jpeg_frames_restore_original_coefficients_and_complete_file_bytes() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    for (bytes, data, jpeg) in fixture_pairs() {
        let request = format!("jr-frame {} {data}", hex(&bytes));
        let original = original_observations(std::slice::from_ref(&request));
        let written = original[0]
            .split_once(" W 0 ")
            .expect("original complete JPEG reconstruction")
            .1;
        assert_eq!(
            unhex(written),
            jpeg,
            "original fixture exact reconstruction"
        );
        compare_jpeg(vec![request]);
    }
}
#[test]
fn jpeg_frames_keep_partial_coefficient_and_quantization_mutations() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    for (bytes, data, _) in fixture_pairs() {
        let mut requests = Vec::new();
        for n in (0..bytes.len().min(40)).chain([bytes.len() / 2, bytes.len().saturating_sub(1)]) {
            requests.push(format!("jr-frame {} {data}", hex(&bytes[..n])));
        }
        for offset in [
            bytes.len() / 3,
            bytes.len() / 2,
            bytes.len().saturating_sub(1),
        ] {
            for mask in [1, 128] {
                let mut bad = bytes.clone();
                bad[offset] ^= mask;
                requests.push(format!("jr-frame {} {data}", hex(&bad)));
            }
        }
        for chunk in requests.chunks(16) {
            compare_jpeg(chunk.to_vec());
        }
    }
}

#[test]
fn jpeg_frame_setup_keeps_component_quantizer_and_reused_storage_errors() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    for (bytes, data, _) in fixture_pairs() {
        let mut requests = Vec::new();
        for mode in [1, 2, 4, 5, 6, 7, 9, 10] {
            requests.push(format!("jr-frame-mut {mode} {} {data}", hex(&bytes)));
            requests.push(format!(
                "jr-frame-mut {mode} {} {data}",
                hex(&bytes[..bytes.len() - 1])
            ));
        }
        compare_jpeg(requests);
    }
}
