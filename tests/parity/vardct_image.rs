// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use super::*;
use jxl::jxl::dec_frame::decode_vardct;
use jxl::jxl::render_pipeline::stage_from_linear::fast_linear_to_srgb;
pub(super) fn handles(kind: &str) -> bool {
    matches!(kind, "vd-decode" | "vd-srgb" | "vd-srgb-high")
}
pub(super) fn observe(kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    if kind == "vd-srgb-high" {
        return jxl::jxl::cms::transfer_functions_scalar::srgb_encoded_from_display(
            f32::from_bits(args.next().unwrap().parse().unwrap()),
        )
        .to_bits()
        .to_string();
    }
    if kind == "vd-srgb" {
        return fast_linear_to_srgb(f32::from_bits(args.next().unwrap().parse().unwrap()))
            .to_bits()
            .to_string();
    }
    let decoded = decode_vardct(&unhex(args.next().unwrap()));
    let mut out = format!(
        "{} M {} {}",
        status_code(&decoded.status),
        decoded.metadata.xsize(),
        decoded.metadata.ysize()
    );
    metadata::metadata(&mut out, &decoded.metadata.m);
    metadata::transform(&mut out, &decoded.metadata.transform_data);
    write!(
        out,
        " F {} {} {} O {} R",
        decoded.frame_size.0,
        decoded.frame_size.1,
        decoded.frame_decoded_bytes,
        hex(&decoded.pixels)
    )
    .unwrap();
    for v in decoded.float_pixels {
        write!(out, " {}", v.to_bits()).unwrap();
    }
    out
}
fn pixels(w: usize, h: usize, gray: bool, alpha: bool, depth: usize, mode: usize) -> Vec<u8> {
    let mut seed = 0x91ab7441ceef1234;
    (0..w * h * ((if gray { 1 } else { 3 }) + usize::from(alpha)))
        .flat_map(|i| {
            let v = match mode {
                0 => 0,
                1 => 32767,
                2 => (i * 113 + i * i * 17) as u16,
                3 => {
                    if (i / 3) % 11 < 5 {
                        65535
                    } else {
                        47
                    }
                }
                _ => next(&mut seed) as u16,
            };
            if depth == 8 {
                vec![(v >> 8) as u8]
            } else {
                v.to_ne_bytes().to_vec()
            }
        })
        .collect()
}
fn encode(
    size: (usize, usize),
    channels: (bool, bool),
    depth: usize,
    effort: usize,
    gab: bool,
    progressive: bool,
    mode: usize,
) -> Vec<u8> {
    let (w, h) = size;
    let (gray, alpha) = channels;
    // Public level-10 output requires a container for 16-bit alpha. The
    // first decoder profile accepts raw codestreams, so call the same original
    // one-shot encoder/header boundary directly for that configuration.
    let command = if alpha && depth == 16 {
        "vd-encode-raw"
    } else {
        "vd-encode"
    };
    let request = format!(
        "{command} {w} {h} {} {} {depth} {effort} {} {} {}",
        u8::from(gray),
        u8::from(alpha),
        u8::from(gab),
        u8::from(progressive),
        hex(&pixels(w, h, gray, alpha, depth, mode))
    );
    let observed = original_observations(&[request]);
    let fields: Vec<_> = observed[0].split_whitespace().collect();
    assert_eq!(fields[0], "0", "original VarDCT encoder");
    let bytes = unhex(fields[1]);
    payload_start(&bytes, progressive, gab);
    bytes
}
fn payload_start(bytes: &[u8], progressive: bool, gab: bool) -> usize {
    use jxl::jxl::{fields, frame_header, headers, image_metadata, toc};
    let mut r = BitReader::new(bytes);
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
    assert_eq!(header.loop_filter.gab, gab);
    if progressive {
        assert!(
            header.passes.num_passes > 1,
            "original stream must actually contain multiple AC passes"
        );
    }
    let dim = header.to_frame_dimensions();
    let n = toc::num_toc_entries(
        dim.num_groups,
        dim.num_dc_groups,
        header.passes.num_passes as usize,
    );
    toc::read_toc(n, &mut r, &mut Vec::new(), &mut Vec::new()).unwrap();
    let offset = r.total_bits_consumed() / 8;
    r.close().unwrap();
    offset
}

#[test]
fn first_complete_vardct_files_decode_original_pixels() {
    let mut cases = Vec::new();
    for (w, h) in [(1, 1), (3, 8), (17, 9), (65, 33), (258, 3)] {
        for gray in [false, true] {
            for depth in [8, 16] {
                for mode in 0..5 {
                    let bytes = encode((w, h), (gray, false), depth, 3, false, false, mode);
                    let result = decode_vardct(&bytes);
                    assert_eq!(
                        result.status,
                        Ok(()),
                        "{w}x{h} gray={gray} depth={depth} mode={mode}"
                    );
                    assert_eq!(result.reader_bits, bytes.len() * 8);
                    cases.push(format!("vd-decode {}", hex(&bytes)));
                }
            }
        }
    }
    compare(cases);
}
#[test]
fn srgb_scalar_transfer_matches_original() {
    let mut words = vec![
        0u32, 0x80000000, 1, 0x80000001, 0x7f800000, 0xff800000, 0x7fc12345, 0x7fa12345, 0xffa54321,
    ];
    for value in [0.0031308f32, 1.0, 0.25, 0.5, -0.0031308, -1.0] {
        for delta in -2i64..=2 {
            words.push((i64::from(value.to_bits()) + delta) as u32);
        }
    }
    let mut seed = 0x123456789;
    for _ in 0..4096 {
        words.push(next(&mut seed) as u32);
    }
    compare(words.into_iter().map(|b| format!("vd-srgb {b}")).collect());
}

#[test]
fn vardct_alpha_gaborish_progressive_and_group_boundaries() {
    let mut cases = Vec::new();
    for gray in [false, true] {
        for depth in [8, 16] {
            for gab in [false, true] {
                for progressive in [false, true] {
                    for size in [(17, 9), (257, 3)] {
                        let bytes = encode(size, (gray, true), depth, 7, gab, progressive, 4);
                        let decoded = decode_vardct(&bytes);
                        assert_eq!(
                            decoded.status,
                            Ok(()),
                            "alpha size={size:?} gray={gray} depth={depth} gab={gab} progressive={progressive}"
                        );
                        cases.push(format!("vd-decode {}", hex(&bytes)));
                    }
                }
            }
        }
    }
    // Odd extents across AC groups in both axes and multiple DC groups.
    for size in [(258, 258), (513, 257), (2049, 3)] {
        for effort in [2, 7, 9] {
            let bytes = encode(size, (false, false), 8, effort, true, true, 2);
            assert_eq!(
                decode_vardct(&bytes).status,
                Ok(()),
                "group boundaries {size:?} effort{effort}"
            );
            // Bound peak observation memory while comparing every pixel word.
            compare(vec![format!("vd-decode {}", hex(&bytes))]);
        }
    }
    compare(cases);
}
#[test]
fn vardct_whole_file_truncation_and_payload_errors() {
    let mut cases = Vec::new();
    for channels in [(false, false), (true, false), (false, true), (true, true)] {
        let bytes = encode((3, 8), channels, 8, 7, false, true, 4);
        for cut in 0..=bytes.len() {
            cases.push(format!("vd-decode {}", hex(&bytes[..cut])));
        }
        let start = payload_start(&bytes, true, false);
        for offset in start..bytes.len() {
            for mask in [1, 128] {
                let mut bad = bytes.clone();
                bad[offset] ^= mask;
                cases.push(format!("vd-decode {}", hex(&bad)));
            }
        }
        for suffix in [vec![0], vec![0xff], vec![0; 8]] {
            let mut bad = bytes.clone();
            bad.extend(suffix);
            cases.push(format!("vd-decode {}", hex(&bad)));
        }
    }
    compare(cases);
}
#[test]
fn srgb_high_precision_transfer_matches_selected_original_configuration() {
    let mut words = vec![
        0u32, 0x80000000, 1, 0x80000001, 0x7f800000, 0xff800000, 0x7fc12345, 0x7fa12345, 0xffa54321,
    ];
    for value in [0.0031308f32, 1.0, 0.25, 0.5, -0.0031308, -1.0] {
        for delta in -2i64..=2 {
            words.push((i64::from(value.to_bits()) + delta) as u32);
        }
    }
    let mut seed = 0x123456789;
    for _ in 0..4096 {
        words.push(next(&mut seed) as u32);
    }
    compare(
        words
            .into_iter()
            .map(|b| format!("vd-srgb-high {b}"))
            .collect(),
    );
}
