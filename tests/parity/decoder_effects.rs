// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use super::*;
use jxl::jxl::dec_frame::decode_vardct;
use jxl::jxl::{fields, frame_header, headers, image_metadata};

fn encode(
    size: (usize, usize),
    channels: (bool, bool),
    depth: usize,
    effects: (usize, bool, bool, bool),
    mode: usize,
) -> (Vec<u8>, usize) {
    encode_with_splines(size, channels, depth, effects, mode, 0)
}

pub(super) fn encode_with_splines(
    size: (usize, usize),
    channels: (bool, bool),
    depth: usize,
    effects: (usize, bool, bool, bool),
    mode: usize,
    spline_mode: usize,
) -> (Vec<u8>, usize) {
    let (w, h) = size;
    let (gray, alpha) = channels;
    let (epf, gab, progressive, noise) = effects;
    let mut seed = 0xd34a158f72c0b619;
    let n = (if gray { 1 } else { 3 }) + usize::from(alpha);
    let pixels: Vec<u8> = (0..w * h * n)
        .flat_map(|i| {
            let value = match mode {
                0 => 32767u16,
                1 => (i * 113 + i * i * 17) as u16,
                2 => {
                    if (i / n) % 11 < 5 {
                        65535
                    } else {
                        47
                    }
                }
                _ => next(&mut seed) as u16,
            };
            if depth == 8 {
                vec![(value >> 8) as u8]
            } else {
                value.to_ne_bytes().to_vec()
            }
        })
        .collect();
    let command = if spline_mode == 0 {
        "vd-encode-effects"
    } else {
        "vd-encode-splines"
    };
    let spline_arg = if spline_mode == 0 {
        String::new()
    } else {
        format!("{spline_mode} ")
    };
    let request = format!(
        "{command} {w} {h} {} {} {depth} 7 {} {} {epf} {} {spline_arg}{}",
        u32::from(gray),
        u32::from(alpha),
        u32::from(gab),
        u32::from(progressive),
        u32::from(noise),
        hex(&pixels)
    );
    let observation = original_observations(&[request]);
    let mut words = observation[0].split_whitespace();
    assert_eq!(words.next(), Some("0"));
    let bytes = unhex(words.next().unwrap());
    let mut r = BitReader::new(&bytes);
    assert_eq!(r.read_fixed_bits::<16>(), 0x0aff);
    let mut metadata = image_metadata::CodecMetadata::default();
    headers::read_size_header(&mut r, &mut metadata.size).unwrap();
    image_metadata::read_image_metadata(&mut r, &mut metadata.m).unwrap();
    metadata.transform_data.nonserialized_xyb_encoded = metadata.m.xyb_encoded;
    fields::bundle::read(&mut r, &mut metadata.transform_data).unwrap();
    r.jump_to_byte_boundary().unwrap();
    let mut header = frame_header::FrameHeader::new(Some(&metadata));
    frame_header::read_frame_header(&mut r, &mut header).unwrap();
    assert_eq!(header.encoding, frame_header::FrameEncoding::VAR_DCT);
    assert_eq!(
        header.loop_filter.epf_iters as usize, epf,
        "original must actually enable requested EPF"
    );
    assert_eq!(header.loop_filter.gab, gab);
    assert_eq!(
        header.flags & frame_header::FLAG_SPLINES != 0,
        spline_mode != 0,
        "original must actually include splines"
    );
    assert_eq!(
        header.flags & frame_header::FLAG_NOISE != 0,
        noise,
        "original must actually enable requested noise"
    );
    if progressive {
        assert!(header.passes.num_passes > 1);
    }
    let d = header.to_frame_dimensions();
    let n = jxl::jxl::toc::num_toc_entries(
        d.num_groups,
        d.num_dc_groups,
        header.passes.num_passes as usize,
    );
    jxl::jxl::toc::read_toc(
        jxl::jxl::memory_manager_internal::MemoryManagerPtr::NATIVE,
        n,
        &mut r,
        &mut Vec::new(),
        &mut Vec::new(),
    )
    .unwrap();
    let payload = r.total_bits_consumed() / 8;
    r.close().unwrap();
    drop(r);
    (bytes, payload)
}
fn check(bytes: &[u8]) {
    let result = decode_vardct(bytes);
    assert_eq!(result.status, Ok(()), "original effects file");
    assert_eq!(result.reader_bits, bytes.len() * 8);
    compare(vec![format!("vd-decode {}", hex(bytes))]);
}

#[test]
fn complete_vardct_epf_files_match_original_pixels() {
    for epf in 1..=3 {
        for (i, size) in [(1, 1), (3, 8), (17, 9), (65, 33)].into_iter().enumerate() {
            for (gray, alpha) in [(false, false), (true, false), (false, true), (true, true)] {
                for depth in [8, 16] {
                    let (bytes, _) = encode(
                        size,
                        (gray, alpha),
                        depth,
                        (epf, (i + epf) % 2 != 0, i % 2 != 0, false),
                        i,
                    );
                    check(&bytes);
                }
            }
        }
    }
    // Group edges and several DC groups require generated/mirrored sigma rows
    // and complete pass scheduling, beyond isolated row/stage comparisons.
    for epf in 1..=3 {
        for size in [(258, 258), (513, 257), (2049, 3)] {
            for gab in [false, true] {
                let (bytes, _) = encode(size, (false, false), 8, (epf, gab, true, false), 1);
                check(&bytes);
            }
        }
    }
}

#[test]
fn complete_vardct_epf_truncation_and_payload_errors_match_original() {
    let mut requests = Vec::new();
    for epf in 1..=3 {
        let (bytes, payload) = encode((3, 8), (false, true), 8, (epf, true, true, false), 3);
        for end in 0..bytes.len() {
            requests.push(format!("vd-decode {}", hex(&bytes[..end])));
        }
        for index in payload..bytes.len() {
            for bit in [0, 3, 7] {
                let mut damaged = bytes.clone();
                damaged[index] ^= 1 << bit;
                requests.push(format!("vd-decode {}", hex(&damaged)));
            }
        }
        let mut trailing = bytes.clone();
        trailing.extend([0, 17, 255]);
        requests.push(format!("vd-decode {}", hex(&trailing)));
    }
    for chunk in requests.chunks(32) {
        compare(chunk.to_vec());
    }
}

#[test]
fn complete_vardct_noise_and_epf_combinations_match_original_pixels() {
    for epf in 0..=3 {
        for (i, size) in [(1, 1), (3, 8), (17, 9), (65, 33)].into_iter().enumerate() {
            for (gray, alpha) in [(false, false), (true, false), (false, true), (true, true)] {
                for depth in [8, 16] {
                    let (bytes, _) = encode(
                        size,
                        (gray, alpha),
                        depth,
                        (epf, i % 2 != 0, i % 2 != 0, true),
                        i,
                    );
                    check(&bytes);
                }
            }
        }
        for size in [(258, 258), (513, 257), (2049, 3)] {
            let (bytes, _) = encode(size, (false, true), 8, (epf, true, true, true), 1);
            check(&bytes);
        }
    }
}

#[test]
fn complete_vardct_splines_and_effect_combinations_match_original_pixels() {
    for epf in 0..=3 {
        for (gray, alpha) in [(false, false), (true, false), (false, true), (true, true)] {
            for (i, size) in [(17, 9), (65, 33), (258, 3)].into_iter().enumerate() {
                let (bytes, _) = encode_with_splines(
                    size,
                    (gray, alpha),
                    if i % 2 == 0 { 8 } else { 16 },
                    (epf, i % 2 != 0, true, i % 2 == 0),
                    i,
                    i % 2 + 1,
                );
                check(&bytes);
            }
        }
    }
    for size in [(258, 258), (513, 257), (2049, 3)] {
        let (bytes, _) = encode_with_splines(size, (false, true), 16, (3, true, true, true), 3, 2);
        check(&bytes);
    }
}

#[test]
fn complete_vardct_noise_and_spline_errors_match_original() {
    let mut requests = Vec::new();
    for (effects, splines) in [
        ((0, false, false, true), 0),
        ((2, true, true, false), 1),
        ((3, true, true, true), 2),
    ] {
        let (bytes, payload) = encode_with_splines((3, 8), (false, true), 8, effects, 3, splines);
        for end in 0..bytes.len() {
            requests.push(format!("vd-decode {}", hex(&bytes[..end])));
        }
        for index in payload..bytes.len() {
            for bit in [0, 3, 7] {
                let mut damaged = bytes.clone();
                damaged[index] ^= 1 << bit;
                requests.push(format!("vd-decode {}", hex(&damaged)));
            }
        }
        let mut trailing = bytes.clone();
        trailing.extend([0, 17, 255]);
        requests.push(format!("vd-decode {}", hex(&trailing)));
    }
    for chunk in requests.chunks(32) {
        compare(chunk.to_vec());
    }
}
