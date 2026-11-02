// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use super::*;
use jxl::jxl::decode::{self as api, JxlDecoderStatus as Status, JxlSignature as Signature};

// Capture only this additive source corpus for the independent two-graph audit.
fn compare_requests(requests: Vec<String>) {
    if let Some(path) = std::env::var_os("JXL_DECODER_API_REQUEST_CAPTURE") {
        static CAPTURE: std::sync::Mutex<()> = std::sync::Mutex::new(());
        let _guard = CAPTURE.lock().unwrap();
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .unwrap();
        let mut lines = requests.join("\n");
        lines.push('\n');
        std::io::Write::write_all(&mut file, lines.as_bytes()).unwrap();
    }
    compare(requests);
}

pub(super) fn handles(kind: &str) -> bool {
    kind.starts_with("da-")
}

pub(super) fn observe(kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    match kind {
        "da-constants" => {
            let mut out = format!(
                "{} {} {} {} {}",
                std::mem::size_of::<Signature>(),
                std::mem::align_of::<Signature>(),
                std::mem::size_of::<Status>(),
                std::mem::align_of::<Status>(),
                api::initial_basic_info_size_hint()
            );
            for value in [
                Signature::NOT_ENOUGH_BYTES,
                Signature::INVALID,
                Signature::CODESTREAM,
                Signature::CONTAINER,
            ] {
                write!(out, " {}", value.0).unwrap();
            }
            for value in [
                Status::SUCCESS,
                Status::ERROR,
                Status::NEED_MORE_INPUT,
                Status::NEED_PREVIEW_OUT_BUFFER,
                Status::NEED_IMAGE_OUT_BUFFER,
                Status::JPEG_NEED_MORE_OUTPUT,
                Status::BOX_NEED_MORE_OUTPUT,
                Status::BASIC_INFO,
                Status::COLOR_ENCODING,
                Status::PREVIEW_IMAGE,
                Status::FRAME,
                Status::FULL_IMAGE,
                Status::JPEG_RECONSTRUCTION,
                Status::BOX,
                Status::FRAME_PROGRESSION,
                Status::BOX_COMPLETE,
            ] {
                write!(out, " {}", value.0).unwrap();
            }
            out
        }
        "da-bounds" => format!(
            "{}",
            u8::from(api::out_of_bounds(number(args), number(args), number(args)))
        ),
        "da-signature" => {
            let mut pos = number(args);
            let data = unhex(args.next().unwrap());
            let status = api::read_signature(&data, &mut pos);
            format!("{} {pos} {}", status.0, api::jxl_signature_check(&data).0)
        }
        "da-box" => {
            let pos = number(args);
            let file_pos = number(args);
            let mut box_size = number(args);
            let mut header_size = number(args);
            let mut box_type: [u8; 4] = unhex(args.next().unwrap()).try_into().unwrap();
            let data = unhex(args.next().unwrap());
            let status = api::parse_box_header(
                &data,
                pos,
                file_pos,
                &mut box_type,
                &mut box_size,
                &mut header_size,
            );
            format!("{} {box_size} {header_size} {}", status.0, hex(&box_type))
        }
        _ => unreachable!(),
    }
}

#[test]
fn decoder_api_constants_match_public_headers_and_source_size_hint() {
    compare_requests(vec!["da-constants".into()]);
}

#[test]
fn decoder_api_bounds_preserve_unsigned_overflow_and_closed_end() {
    let words = [
        0,
        1,
        2,
        7,
        8,
        15,
        16,
        u32::MAX as usize,
        usize::MAX / 2,
        usize::MAX / 2 + 1,
        usize::MAX - 16,
        usize::MAX - 1,
        usize::MAX,
    ];
    let mut requests = Vec::new();
    for a in words {
        for b in words {
            for size in words {
                requests.push(format!("da-bounds {a} {b} {size}"));
            }
        }
    }
    compare_requests(requests);
}

#[test]
fn decoder_api_signatures_exhaust_two_byte_domain_and_container_prefixes() {
    let mut requests = vec!["da-signature 0 -".into()];
    for first in 0..=255u8 {
        requests.push(format!("da-signature 0 {first:02x}"));
        for second in 0..=255u8 {
            requests.push(format!("da-signature 0 {first:02x}{second:02x}"));
        }
    }
    let signature = jxl::jxl::common::JXL_SIGNATURE_BOX;
    for offset in 0..4 {
        let mut data = vec![0x91; offset];
        data.extend_from_slice(&signature);
        data.extend_from_slice(&[0xff, 0x0a, 0x93]);
        for len in 0..=data.len() {
            for pos in [offset, len, len + 1, usize::MAX - 1, usize::MAX] {
                requests.push(format!("da-signature {pos} {}", hex(&data[..len])));
            }
        }
    }
    for index in 0..signature.len() {
        for value in 0..=255u8 {
            let mut changed = signature;
            changed[index] = value;
            requests.push(format!("da-signature 0 {}", hex(&changed)));
            let mut prefixed = vec![0x91, 0x92, 0x93];
            prefixed.extend_from_slice(&changed);
            requests.push(format!("da-signature 3 {}", hex(&prefixed)));
        }
    }
    compare_requests(requests);
}

fn box_request(pos: usize, file_pos: usize, data: &[u8]) -> String {
    format!(
        "da-box {pos} {file_pos} {} {} aabbccdd {}",
        usize::MAX,
        usize::MAX - 1,
        hex(data)
    )
}

#[test]
fn decoder_api_box_headers_retain_partial_assignments_and_overflow_errors() {
    let mut requests = Vec::new();
    let sizes = [
        0,
        1,
        2,
        7,
        8,
        9,
        15,
        16,
        17,
        255,
        u32::MAX as u64,
        u32::MAX as u64 + 1,
        u64::MAX - 16,
        u64::MAX,
    ];
    for extended in [false, true] {
        for size in sizes {
            if !extended && size > u32::MAX as u64 {
                continue;
            }
            let mut header = if extended { 1u32 } else { size as u32 }
                .to_be_bytes()
                .to_vec();
            header.extend_from_slice(b"jxlp");
            if extended {
                header.extend_from_slice(&size.to_be_bytes());
            }
            header.extend_from_slice(&[0x80, 0, 0, 0]);
            for offset in [0, 1, 3] {
                let mut data = vec![0x91; offset];
                data.extend_from_slice(&header);
                for len in 0..=data.len() {
                    for file_pos in [0, 1, usize::MAX - 16, usize::MAX] {
                        requests.push(box_request(offset, file_pos, &data[..len]));
                    }
                }
            }
        }
    }
    for extended in [false, true] {
        let mut original = if extended { 1u32 } else { 8u32 }.to_be_bytes().to_vec();
        original.extend_from_slice(b"jxlc");
        if extended {
            original.extend_from_slice(&16u64.to_be_bytes());
        }
        for index in 0..original.len() {
            for value in 0..=255u8 {
                let mut data = original.clone();
                data[index] = value;
                for file_pos in [0, usize::MAX - 100] {
                    requests.push(box_request(0, file_pos, &data));
                }
            }
        }
    }
    for len in [0, 1, 7, 8, 15, 16, 31] {
        let data = vec![0x5a; len];
        for pos in [len, len + 1, usize::MAX - 7, usize::MAX - 1, usize::MAX] {
            requests.push(box_request(pos, usize::MAX, &data));
        }
    }
    compare_requests(requests);
}
