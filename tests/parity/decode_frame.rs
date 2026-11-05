// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use super::*;
use jxl::jxl::dec_cache::GroupDecCache;
use jxl::jxl::enc_frame::{LosslessSampleDepth, LosslessThunderConfig};
use jxl::jxl::encode_internal::JxlEncoderChunkedFrameAdapter;
use jxl::jxl::types::{JxlDataType, JxlEndianness, JxlPixelFormat};
pub(super) fn handles(kind: &str) -> bool {
    kind.starts_with("df-")
}
fn config(w: usize, h: usize, gray: bool, depth: u32) -> LosslessThunderConfig {
    LosslessThunderConfig::new(
        w,
        h,
        gray,
        if depth == 8 {
            LosslessSampleDepth::Uint8
        } else {
            LosslessSampleDepth::Uint16
        },
    )
    .unwrap()
}
fn frame(config: &LosslessThunderConfig, bytes: &[u8]) -> Vec<u8> {
    let m = config.metadata();
    let mut input = JxlEncoderChunkedFrameAdapter::new(m.xsize(), m.ysize(), 0);
    assert!(input.set_from_buffer(
        0,
        Some(bytes),
        bytes.len(),
        JxlPixelFormat {
            num_channels: m.m.color_encoding.channels() as u32,
            data_type: if m.m.bit_depth.bits_per_sample == 8 {
                JxlDataType::UINT8
            } else {
                JxlDataType::UINT16
            },
            endianness: JxlEndianness::NATIVE,
            align: 0
        }
    ));
    config
        .create_frame()
        .unwrap()
        .encode_one_shot(&input, None)
        .unwrap()
        .as_slice()
        .to_vec()
}
pub(super) fn observe(kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    if kind == "df-cache" {
        let n = number::<usize>(args);
        let mut cache = GroupDecCache::default();
        let mut out = std::mem::align_of::<GroupDecCache>().to_string();
        for _ in 0..n {
            write!(out, " {}", status_code(&cache.init_dc_buffer_once())).unwrap();
        }
        shared_primitives::plane(&mut out, &cache.dc_buffer);
        for im in &cache.num_nzeroes {
            for c in 0..3 {
                shared_primitives::plane(&mut out, im.plane(c));
            }
        }
        return out;
    }
    let w = number(args);
    let h = number(args);
    let gray = number::<u32>(args) != 0;
    let depth = number(args);
    let bytes = unhex(args.next().unwrap());
    let config = config(w, h, gray, depth);
    if kind == "df-source" {
        return format!("0 {}", hex(&frame(&config, &bytes)));
    }
    let decoded = config.decode_frame(&bytes);
    let mut out = format!(
        "{} H {}",
        status_code(&decoded.status),
        u32::from(decoded.header.is_some())
    );
    if let Some(h) = decoded.header {
        write!(
            out,
            " {} {} {} {} {} {} {}",
            h.encoding.0,
            h.frame_type.0,
            h.flags,
            h.color_transform.0,
            h.group_size_shift,
            h.upsampling,
            u32::from(h.is_last)
        )
        .unwrap();
    }
    let image = decoded.image;
    write!(
        out,
        " O {} {} {} {} {} {} {}",
        image.color().xsize(),
        image.color().ysize(),
        image.decoded_bytes(),
        image.duration,
        image.origin.x0,
        image.origin.y0,
        image.extra_channels().len()
    )
    .unwrap();
    metadata::color_bundle(&mut out, image.c_current());
    if decoded.status.is_ok() {
        for c in 0..3 {
            for y in 0..image.color().ysize() {
                for x in 0..image.color().xsize() {
                    write!(out, " {}", image.color().plane_row(c, y)[x].to_bits()).unwrap();
                }
            }
        }
    }
    out
}
fn pixels(w: usize, h: usize, gray: bool, depth: u32, mode: usize) -> Vec<u8> {
    (0..w * h * if gray { 1 } else { 3 })
        .flat_map(|i| {
            let value = match mode {
                0 => 0,
                1 => {
                    if i % 3 == 0 {
                        65535
                    } else {
                        317
                    }
                }
                2 => (i * 173 + i * i * 29 + 61) as u16,
                _ => ((i * 7) % 5) as u16,
            };
            if depth == 8 {
                vec![value as u8]
            } else {
                value.to_ne_bytes().to_vec()
            }
        })
        .collect()
}
#[test]
fn complete_raw_frames_cross_decode_original_bytes_metadata_sections_and_float_pixels() {
    let mut requests = Vec::new();
    let mut native = Vec::new();
    for (w, h) in [(1, 1), (3, 2), (17, 9), (513, 3), (2049, 2)] {
        for gray in [false, true] {
            for depth in [8, 16] {
                for mode in 0..4 {
                    let p = pixels(w, h, gray, depth, mode);
                    requests.push(format!(
                        "df-source {w} {h} {} {depth} {}",
                        u32::from(gray),
                        hex(&p)
                    ));
                    native.push(frame(&config(w, h, gray, depth), &p));
                }
            }
        }
    }
    let sources = original_observations(&requests);
    let mut cases = Vec::new();
    for ((request, source), bytes) in requests.iter().zip(sources).zip(native) {
        assert_eq!(
            source,
            format!("0 {}", hex(&bytes)),
            "original encoded frame: {request}"
        );
        let mut fields = request.split_whitespace();
        fields.next();
        let w = fields.next().unwrap();
        let h = fields.next().unwrap();
        let gray = fields.next().unwrap();
        let depth = fields.next().unwrap();
        cases.push(format!(
            "df-decode {w} {h} {gray} {depth} {}",
            source.split_whitespace().nth(1).unwrap()
        ));
        cases.push(format!("df-decode {w} {h} {gray} {depth} {}", hex(&bytes)));
    }
    println!("{} C++/Rust differential cases matched", requests.len());
    compare(cases);
}
#[test]
fn complete_raw_frame_truncations_and_corrupt_sections_preserve_source_partial_output() {
    let mut cases = vec![
        "df-cache 0".into(),
        "df-cache 1".into(),
        "df-cache 2".into(),
    ];
    for gray in [false, true] {
        for depth in [8, 16] {
            let cfg = config(3, 2, gray, depth);
            let bytes = frame(&cfg, &pixels(3, 2, gray, depth, 2));
            for cut in 0..=bytes.len() {
                cases.push(format!(
                    "df-decode 3 2 {} {depth} {}",
                    u32::from(gray),
                    hex(&bytes[..cut])
                ));
            }
            let mut header = jxl::jxl::frame_header::FrameHeader::new(Some(cfg.metadata()));
            let mut r = BitReader::new(&bytes);
            jxl::jxl::frame_header::read_frame_header(&mut r, &mut header).unwrap();
            let dim = header.to_frame_dimensions();
            let mut sizes = Vec::new();
            let mut perm = Vec::new();
            jxl::jxl::toc::read_toc(
                jxl::jxl::memory_manager_internal::MemoryManagerPtr::NATIVE,
                jxl::jxl::toc::num_toc_entries(dim.num_groups, dim.num_dc_groups, 1),
                &mut r,
                &mut sizes,
                &mut perm,
            )
            .unwrap();
            let start = r.total_bits_consumed() / 8;
            r.close().unwrap();
            for index in start..bytes.len() {
                for bit in [0, 3, 7] {
                    let mut damaged = bytes.clone();
                    damaged[index] ^= 1 << bit;
                    cases.push(format!(
                        "df-decode 3 2 {} {depth} {}",
                        u32::from(gray),
                        hex(&damaged)
                    ));
                }
            }
        }
    }
    compare(cases);
}
