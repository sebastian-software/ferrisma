// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use super::*;
pub(super) fn handles(kind: &str) -> bool {
    kind == "ycc-header"
}
pub(super) fn observe(_kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    use jxl::jxl::{
        frame_header::{FrameHeader, read_frame_header},
        image_metadata::CodecMetadata,
    };
    let bytes = unhex(args.next().unwrap());
    let mut metadata = CodecMetadata::default();
    let mut reader = BitReader::new(&bytes);
    assert_eq!(reader.read_fixed_bits::<16>(), 0x0aff);
    jxl::jxl::headers::read_size_header(&mut reader, &mut metadata.size).unwrap();
    jxl::jxl::image_metadata::read_image_metadata(&mut reader, &mut metadata.m).unwrap();
    metadata.transform_data.nonserialized_xyb_encoded = metadata.m.xyb_encoded;
    jxl::jxl::fields::bundle::read(&mut reader, &mut metadata.transform_data).unwrap();
    assert!(
        !metadata.m.color_encoding.want_icc(),
        "fixture non-ICC header"
    );
    reader.jump_to_byte_boundary().unwrap();
    let global_bits = reader.total_bits_consumed();
    let mut header = FrameHeader::new(Some(&metadata));
    read_frame_header(&mut reader, &mut header).unwrap();
    let d = header.to_frame_dimensions();
    let mut out = format!(
        "{global_bits} {} {} {} {} {} {} {} {}",
        reader.total_bits_consumed(),
        header.color_transform.0,
        header.encoding.0,
        header.flags,
        metadata.m.bit_depth.bits_per_sample,
        u8::from(metadata.m.color_encoding.is_gray()),
        metadata.m.num_extra_channels,
        u8::from(metadata.m.color_encoding.want_icc())
    );
    for value in [
        d.xsize,
        d.ysize,
        d.xsize_padded,
        d.ysize_padded,
        d.xsize_blocks,
        d.ysize_blocks,
        d.group_dim,
        d.num_groups,
        d.xsize_groups,
        d.ysize_groups,
    ] {
        write!(out, " {value}").unwrap();
    }
    for c in 0..3 {
        write!(
            out,
            " {} {}",
            header.chroma_subsampling.h_shift(c),
            header.chroma_subsampling.v_shift(c)
        )
        .unwrap();
    }
    write!(out, " {}", status_code(&reader.close())).unwrap();
    out
}
pub(super) fn fixtures() -> [(&'static str, &'static [u8]); 25] {
    [
        ("444", include_bytes!("../data/ycbcr-jpeg-444.jxl")),
        ("422", include_bytes!("../data/ycbcr-jpeg-422.jxl")),
        ("420", include_bytes!("../data/ycbcr-jpeg-420.jxl")),
        ("440", include_bytes!("../data/ycbcr-jpeg-440.jxl")),
        ("gray", include_bytes!("../data/ycbcr-jpeg-gray.jxl")),
        ("444-1x1", include_bytes!("../data/ycbcr-jpeg-444-1x1.jxl")),
        ("422-1x1", include_bytes!("../data/ycbcr-jpeg-422-1x1.jxl")),
        ("420-1x1", include_bytes!("../data/ycbcr-jpeg-420-1x1.jxl")),
        ("440-1x1", include_bytes!("../data/ycbcr-jpeg-440-1x1.jxl")),
        (
            "gray-1x1",
            include_bytes!("../data/ycbcr-jpeg-gray-1x1.jxl"),
        ),
        ("444-9x7", include_bytes!("../data/ycbcr-jpeg-444-9x7.jxl")),
        ("422-9x7", include_bytes!("../data/ycbcr-jpeg-422-9x7.jxl")),
        ("420-9x7", include_bytes!("../data/ycbcr-jpeg-420-9x7.jxl")),
        ("440-9x7", include_bytes!("../data/ycbcr-jpeg-440-9x7.jxl")),
        (
            "gray-9x7",
            include_bytes!("../data/ycbcr-jpeg-gray-9x7.jxl"),
        ),
        (
            "444-257x3",
            include_bytes!("../data/ycbcr-jpeg-444-257x3.jxl"),
        ),
        (
            "422-257x3",
            include_bytes!("../data/ycbcr-jpeg-422-257x3.jxl"),
        ),
        (
            "420-257x3",
            include_bytes!("../data/ycbcr-jpeg-420-257x3.jxl"),
        ),
        (
            "440-257x3",
            include_bytes!("../data/ycbcr-jpeg-440-257x3.jxl"),
        ),
        (
            "gray-257x3",
            include_bytes!("../data/ycbcr-jpeg-gray-257x3.jxl"),
        ),
        (
            "444-3x257",
            include_bytes!("../data/ycbcr-jpeg-444-3x257.jxl"),
        ),
        (
            "422-3x257",
            include_bytes!("../data/ycbcr-jpeg-422-3x257.jxl"),
        ),
        (
            "420-3x257",
            include_bytes!("../data/ycbcr-jpeg-420-3x257.jxl"),
        ),
        (
            "440-3x257",
            include_bytes!("../data/ycbcr-jpeg-440-3x257.jxl"),
        ),
        (
            "gray-3x257",
            include_bytes!("../data/ycbcr-jpeg-gray-3x257.jxl"),
        ),
    ]
}
fn exact(cases: Vec<String>) {
    if let Some(path) = std::env::var_os("JXL_CHROMA_IMAGE_CAPTURE") {
        static CAPTURE: std::sync::Mutex<()> = std::sync::Mutex::new(());
        let _lock = CAPTURE.lock().unwrap();
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .unwrap();
        std::io::Write::write_all(&mut file, (cases.join("\n") + "\n").as_bytes()).unwrap();
    }
    for chunk in cases.chunks(4) {
        let originals = original_observations(chunk);
        for (request, original) in chunk.iter().zip(originals) {
            let rust = super::observe(request);
            if !super::parity_eq(request, &rust, &original) {
                let a: Vec<_> = rust.split_whitespace().collect();
                let b: Vec<_> = original.split_whitespace().collect();
                let differences: Vec<_> = a
                    .iter()
                    .zip(&b)
                    .enumerate()
                    .filter(|(_, (a, b))| a != b)
                    .take(16)
                    .collect();
                panic!(
                    "request {request}: lengths {}/{}; differences {differences:?}",
                    a.len(),
                    b.len()
                );
            }
        }
    }
    println!("{} C++/Rust differential cases matched", cases.len());
}
#[test]
fn genuine_jpeg_chroma_complete_files_preserve_all_source_pixels_and_state() {
    let mut cases = Vec::new();
    for (name, bytes) in fixtures() {
        let request = format!("fs-decode {}", hex(bytes));
        let original = original_observations(std::slice::from_ref(&request));
        let words: Vec<_> = original[0].split_whitespace().collect();
        let end = words.iter().rposition(|&w| w == "E").expect("source EOF");
        assert_eq!(words[end + 1], "0", "source success {name}");
        cases.push(request);
    }
    exact(cases);
}

#[test]
fn genuine_jpeg_chroma_headers_keep_subsampled_padding_and_group_dimensions() {
    exact(
        fixtures()
            .iter()
            .map(|(_, bytes)| format!("ycc-header {}", hex(bytes)))
            .collect(),
    );
}

#[test]
fn genuine_jpeg_chroma_truncated_payload_and_trailing_bytes_keep_defined_state() {
    let mut cases = Vec::new();
    for (_, bytes) in fixtures() {
        let mut cuts: Vec<_> = (0..=bytes.len().min(40)).collect();
        cuts.extend([bytes.len() / 4, bytes.len() / 2, bytes.len() * 3 / 4]);
        cuts.extend((1..=8).map(|n| bytes.len().saturating_sub(n)));
        cuts.sort_unstable();
        cuts.dedup();
        for cut in cuts {
            cases.push(format!("fs-decode {}", hex(&bytes[..cut])));
        }
        for trailing in [1, 2] {
            let mut padded = bytes.to_vec();
            padded.resize(bytes.len() + trailing, 0);
            cases.push(format!("fs-decode {}", hex(&padded)));
        }
        for index in [bytes.len() / 2, bytes.len() - 2, bytes.len() - 1] {
            for mask in [1, 0x80] {
                let mut mutated = bytes.to_vec();
                mutated[index] ^= mask;
                cases.push(format!("fs-decode {}", hex(&mutated)));
            }
        }
    }
    exact(cases);
}
