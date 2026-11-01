// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use super::*;
use jxl::jxl::dec_frame::decode_lossless_thunder;
use jxl::jxl::enc_frame::{LosslessSampleDepth, LosslessThunderConfig};
pub(super) fn handles(kind: &str) -> bool {
    kind == "wi-decode"
}
pub(super) fn observe(_kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    let bytes = unhex(args.next().unwrap());
    let decoded = decode_lossless_thunder(&bytes);
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
        " F {} {} {} O {}",
        decoded.frame_size.0,
        decoded.frame_size.1,
        decoded.frame_decoded_bytes,
        hex(&decoded.pixels)
    )
    .unwrap();
    out
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
fn pixels(w: usize, h: usize, gray: bool, depth: u32, mode: usize) -> Vec<u8> {
    let mut seed = 0xc0ffee77119955u64;
    (0..w * h * if gray { 1 } else { 3 })
        .flat_map(|i| {
            let v = match mode {
                0 => 0,
                1 => {
                    if i % 3 == 0 {
                        65535
                    } else {
                        317
                    }
                }
                2 => (i * 173 + i * i * 29 + 61) as u16,
                3 => ((i * 7) % 5) as u16,
                _ => next(&mut seed) as u16,
            };
            if depth == 8 {
                vec![v as u8]
            } else {
                v.to_ne_bytes().to_vec()
            }
        })
        .collect()
}
#[test]
fn first_complete_codestreams_cross_decode_cpp_rust_metadata_and_integer_pixels() {
    let mut requests = Vec::new();
    let mut native = Vec::new();
    let mut inputs = Vec::new();
    for (w, h) in [
        (1, 1),
        (3, 2),
        (17, 9),
        (65, 33),
        (257, 3),
        (513, 3),
        (2049, 2),
    ] {
        for gray in [false, true] {
            for depth in [8, 16] {
                for mode in 0..5 {
                    let p = pixels(w, h, gray, depth, mode);
                    let bytes = config(w, h, gray, depth)
                        .encode_pixels(&p, None)
                        .unwrap()
                        .as_slice()
                        .to_vec();
                    requests.push(format!(
                        "ss-image {w} {h} {} {depth} 0 {} {}",
                        u32::from(gray),
                        hex(&p),
                        hex(&bytes)
                    ));
                    native.push(bytes);
                    inputs.push((w, h, gray, depth, p));
                }
            }
        }
    }
    let sources = original_observations(&requests);
    let mut cases = Vec::new();
    for (((request, source), bytes), (w, h, gray, depth, input)) in
        requests.iter().zip(sources).zip(native).zip(inputs)
    {
        let encoded = source.split_whitespace().nth(1).unwrap();
        assert_eq!(encoded, hex(&bytes), "encoded bytes: {request}");
        // These are two independent calls to the original public C++ decoder,
        // on its own encoder's file and the supplied Rust encoder's file.
        let results: Vec<_> = source.rsplit(" D ").take(2).collect();
        assert_eq!(results.len(), 2);
        for result in results {
            let fields: Vec<_> = result.split_whitespace().collect();
            assert_eq!(
                &fields[..5],
                &[
                    "0",
                    &w.to_string(),
                    &h.to_string(),
                    &depth.to_string(),
                    if gray { "1" } else { "3" }
                ]
            );
            assert_eq!(fields[5], hex(&input));
        }
        for data in [unhex(encoded), bytes] {
            let decoded = decode_lossless_thunder(&data);
            assert_eq!(
                decoded.status,
                Ok(()),
                "native whole decode {w}x{h} gray={gray} depth={depth}"
            );
            assert_eq!(decoded.pixels, input);
            assert_eq!(decoded.reader_bits, data.len() * 8);
            assert_eq!((decoded.metadata.xsize(), decoded.metadata.ysize()), (w, h));
            assert_eq!(decoded.metadata.m.bit_depth.bits_per_sample, depth);
            assert_eq!(
                decoded.metadata.m.color_encoding.channels(),
                if gray { 1 } else { 3 }
            );
            cases.push(format!("wi-decode {}", hex(&data)));
        }
    }
    println!("{} C++/Rust differential cases matched", requests.len());
    compare(cases);
}
#[test]
fn whole_codestream_prefixes_marker_trailing_bytes_and_payload_errors_match_original() {
    use jxl::jxl::fields::bundle;
    use jxl::jxl::frame_header::{FrameHeader, read_frame_header};
    use jxl::jxl::headers::read_size_header;
    use jxl::jxl::image_metadata::{CodecMetadata, read_image_metadata};
    let mut cases = Vec::new();
    for gray in [false, true] {
        for depth in [8, 16] {
            let bytes = config(3, 2, gray, depth)
                .encode_pixels(&pixels(3, 2, gray, depth, 4), None)
                .unwrap();
            let data = bytes.as_slice();
            for cut in 0..=data.len() {
                cases.push(format!("wi-decode {}", hex(&data[..cut])));
            }
            for index in [0, 1] {
                let mut bad = data.to_vec();
                bad[index] ^= 0x80;
                cases.push(format!("wi-decode {}", hex(&bad)));
            }
            for suffix in [vec![0], vec![0xff], vec![0; 8]] {
                let mut bad = data.to_vec();
                bad.extend_from_slice(&suffix);
                cases.push(format!("wi-decode {}", hex(&bad)));
            }
            let mut r = BitReader::new(data);
            r.read_fixed_bits::<16>();
            let mut meta = CodecMetadata::default();
            read_size_header(&mut r, &mut meta.size).unwrap();
            read_image_metadata(&mut r, &mut meta.m).unwrap();
            meta.transform_data.nonserialized_xyb_encoded = meta.m.xyb_encoded;
            bundle::read(&mut r, &mut meta.transform_data).unwrap();
            r.jump_to_byte_boundary().unwrap();
            let mut header = FrameHeader::new(Some(&meta));
            read_frame_header(&mut r, &mut header).unwrap();
            let d = header.to_frame_dimensions();
            let mut sizes = Vec::new();
            let mut perm = Vec::new();
            jxl::jxl::toc::read_toc(
                jxl::jxl::toc::num_toc_entries(d.num_groups, d.num_dc_groups, 1),
                &mut r,
                &mut sizes,
                &mut perm,
            )
            .unwrap();
            let start = r.total_bits_consumed() / 8;
            r.close().unwrap();
            for i in start..data.len() {
                for bit in [0, 3, 7] {
                    let mut bad = data.to_vec();
                    bad[i] ^= 1 << bit;
                    cases.push(format!("wi-decode {}", hex(&bad)));
                }
            }
        }
    }
    compare(cases);
}
