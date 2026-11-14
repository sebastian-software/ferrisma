// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Entire coefficient/metadata/file captures from the independent pinned original.
use std::fmt::Write as _;
use std::str::FromStr;
#[path = "common/jpeg_data.rs"]
mod jpeg_data_protocol;
#[path = "common/jpeg_reconstruction.rs"]
mod jpeg_reconstruction_protocol;
fn number<T: FromStr>(args: &mut std::str::SplitWhitespace<'_>) -> T
where
    T::Err: std::fmt::Debug,
{
    args.next().unwrap().parse().unwrap()
}
fn hex(bytes: &[u8]) -> String {
    if bytes.is_empty() {
        return "-".to_owned();
    }
    let mut text = String::new();
    for byte in bytes {
        write!(text, "{byte:02x}").unwrap();
    }
    text
}
fn unhex(value: &str) -> Vec<u8> {
    if value == "-" {
        return Vec::new();
    }
    assert_eq!(value.len() % 2, 0);
    (0..value.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&value[i..i + 2], 16).unwrap())
        .collect()
}
#[test]
fn original_complete_and_partial_jpeg_frames_preserve_all_captured_state_and_bytes() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data");
    for row in include_str!("data/jpeg-reconstruction/native-captures.tsv").lines() {
        let values: Vec<_> = row.split('\t').collect();
        assert_eq!(values.len(), 4);
        let mut wire = std::fs::read(root.join(values[0])).unwrap();
        if values[1] == "1" {
            wire.pop();
        }
        let fields = std::fs::read_to_string(root.join(values[2])).unwrap();
        let expected = std::fs::read_to_string(root.join(values[3])).unwrap();
        let request = format!("{} {fields}", hex(&wire));
        let observed =
            jpeg_reconstruction_protocol::observe("jr-frame", &mut request.split_whitespace());
        assert_eq!(
            observed,
            expected.trim_end(),
            "native original capture {}",
            values[3]
        );
    }
}
