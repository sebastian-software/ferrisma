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
                jxl::jxl::memory_manager_internal::MemoryManagerPtr::NATIVE,
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

fn alpha_pixels(w: usize, h: usize, gray: bool, depth: u32, mode: usize) -> Vec<u8> {
    let channels = if gray { 2 } else { 4 };
    let mut seed = 0xaa558811cdef0023;
    (0..w * h * channels)
        .flat_map(|i| {
            let value = if i % channels == channels - 1 {
                match mode {
                    0 => 0,
                    1 => u16::MAX,
                    2 => [0, u16::MAX, 1, 32768, 129][(i / channels) % 5],
                    _ => next(&mut seed) as u16,
                }
            } else {
                match mode {
                    0 => (i * 173 + i * i * 29 + 61) as u16,
                    1 => 0,
                    2 => [1, 65534, 257, 32767][i % 4],
                    3 => ((i / channels) % 5) as u16,
                    _ => next(&mut seed) as u16,
                }
            };
            if depth == 8 {
                vec![value as u8]
            } else {
                value.to_ne_bytes().to_vec()
            }
        })
        .collect()
}
fn alpha_config(w: usize, h: usize, gray: bool, depth: u32) -> LosslessThunderConfig {
    LosslessThunderConfig::new_with_alpha(
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
#[test]
fn alpha_complete_codestreams_match_original_bytes_metadata_cross_decode_and_hidden_colors() {
    let mut cases = Vec::new();
    let mut whole = Vec::new();
    for (w, h) in [
        (1, 1),
        (3, 2),
        (17, 9),
        (65, 33),
        (257, 3),
        (513, 3),
        (2049, 2),
        (3, 257),
        (3, 2049),
        (257, 257),
    ] {
        for gray in [false, true] {
            for depth in [8, 16] {
                for mode in 0..5 {
                    let input = alpha_pixels(w, h, gray, depth, mode);
                    for has_aux in [false, true] {
                        let encoded = alpha_config(w, h, gray, depth)
                            .encode_pixels(&input, None)
                            .unwrap();
                        cases.push(format!(
                            "ss-image-alpha {w} {h} {} {depth} {} {} {}",
                            u32::from(gray),
                            u32::from(has_aux),
                            hex(&input),
                            hex(encoded.as_slice())
                        ));
                        // C++ compares its own original one-shot output with the
                        // native file and independently cross-decodes both.
                        let decoded = decode_lossless_thunder(encoded.as_slice());
                        assert_eq!(decoded.status, Ok(()));
                        assert_eq!(decoded.pixels, input);
                        assert_eq!(decoded.metadata.m.get_alpha_bits(), depth);
                        assert_eq!(decoded.metadata.m.num_extra_channels, 1);
                        whole.push(format!("wi-decode {}", hex(encoded.as_slice())));
                    }
                }
            }
        }
    }
    compare(cases);
    compare(whole);
}
#[test]
fn alpha_codestream_prefixes_and_modular_payload_errors_match_original_partial_state() {
    use jxl::jxl::fields::bundle;
    use jxl::jxl::frame_header::{FrameHeader, read_frame_header};
    use jxl::jxl::headers::read_size_header;
    use jxl::jxl::image_metadata::{CodecMetadata, read_image_metadata};
    let mut cases = Vec::new();
    for gray in [false, true] {
        for depth in [8, 16] {
            let bytes = alpha_config(3, 2, gray, depth)
                .encode_pixels(&alpha_pixels(3, 2, gray, depth, 2), None)
                .unwrap();
            let data = bytes.as_slice();
            // Metadata prefixes remain validly admitted only after the complete
            // alpha description. Both implementations retain parsed fields.
            for cut in 0..=data.len() {
                cases.push(format!("wi-decode {}", hex(&data[..cut])));
            }
            for suffix in [vec![0], vec![0xff], vec![0; 8]] {
                let mut bad = data.to_vec();
                bad.extend_from_slice(&suffix);
                cases.push(format!("wi-decode {}", hex(&bad)));
            }
            let mut r = BitReader::new(data);
            r.read_fixed_bits::<16>();
            let mut metadata = CodecMetadata::default();
            read_size_header(&mut r, &mut metadata.size).unwrap();
            read_image_metadata(&mut r, &mut metadata.m).unwrap();
            metadata.transform_data.nonserialized_xyb_encoded = metadata.m.xyb_encoded;
            bundle::read(&mut r, &mut metadata.transform_data).unwrap();
            r.jump_to_byte_boundary().unwrap();
            let mut header = FrameHeader::new(Some(&metadata));
            read_frame_header(&mut r, &mut header).unwrap();
            let d = header.to_frame_dimensions();
            let mut sizes = Vec::new();
            let mut permutation = Vec::new();
            jxl::jxl::toc::read_toc(
                jxl::jxl::memory_manager_internal::MemoryManagerPtr::NATIVE,
                jxl::jxl::toc::num_toc_entries(d.num_groups, d.num_dc_groups, 1),
                &mut r,
                &mut sizes,
                &mut permutation,
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

#[test]
fn alpha_short_pixel_buffers_match_original_errors_and_untouched_statistics() {
    let mut cases = Vec::new();
    for gray in [false, true] {
        for depth in [8, 16] {
            let input = alpha_pixels(3, 2, gray, depth, 2);
            for cut in 0..input.len() {
                for has_aux in [false, true] {
                    cases.push(format!(
                        "ss-image-alpha 3 2 {} {depth} {} {} -",
                        u32::from(gray),
                        u32::from(has_aux),
                        hex(&input[..cut])
                    ));
                }
            }
        }
    }
    compare(cases);
}

#[test]
fn separate_alpha_buffers_match_original_opaque_fill_extra_copy_and_complete_frame_bytes() {
    use jxl::jxl::encode_internal::JxlEncoderChunkedFrameAdapter;
    use jxl::jxl::fields::bundle;
    use jxl::jxl::headers::read_size_header;
    use jxl::jxl::image_metadata::{CodecMetadata, read_image_metadata};
    use jxl::jxl::types::{JxlDataType, JxlEndianness, JxlPixelFormat};
    let mut cases = Vec::new();
    let mut expected = Vec::new();
    for (w, h) in [(1, 1), (17, 9), (257, 3), (3, 257)] {
        for gray in [false, true] {
            for depth in [8, 16] {
                let data = alpha_pixels(w, h, gray, depth, 2);
                let mut config = alpha_config(w, h, gray, depth);
                let full = config.encode_pixels(&data, None).unwrap();
                let color_channels = if gray { 1 } else { 3 };
                let sample = depth as usize / 8;
                let mut color = Vec::new();
                let mut alpha = Vec::new();
                for pixel in data.chunks_exact((color_channels + 1) * sample) {
                    color.extend_from_slice(&pixel[..color_channels * sample]);
                    alpha.extend_from_slice(&pixel[color_channels * sample..]);
                }
                let mut input = JxlEncoderChunkedFrameAdapter::new(w, h, 1);
                let format = JxlPixelFormat {
                    num_channels: color_channels as u32,
                    data_type: if depth == 8 {
                        JxlDataType::UINT8
                    } else {
                        JxlDataType::UINT16
                    },
                    endianness: JxlEndianness::NATIVE,
                    align: 0,
                };
                assert!(input.set_from_buffer(0, Some(&color), color.len(), format));
                assert!(input.set_from_buffer(
                    1,
                    Some(&alpha),
                    alpha.len(),
                    JxlPixelFormat {
                        num_channels: 1,
                        ..format
                    }
                ));
                let frame = config
                    .create_frame()
                    .unwrap()
                    .encode_one_shot(&input, None)
                    .unwrap();
                // The original complete header writer has already run. Read
                // its real global fields to find the independently encoded
                // one-shot frame, rather than substituting a framing algorithm.
                let mut r = BitReader::new(full.as_slice());
                r.read_fixed_bits::<16>();
                let mut metadata = CodecMetadata::default();
                read_size_header(&mut r, &mut metadata.size).unwrap();
                read_image_metadata(&mut r, &mut metadata.m).unwrap();
                metadata.transform_data.nonserialized_xyb_encoded = metadata.m.xyb_encoded;
                bundle::read(&mut r, &mut metadata.transform_data).unwrap();
                r.jump_to_byte_boundary().unwrap();
                let frame_start = r.total_bits_consumed() / 8;
                r.close().unwrap();
                assert_eq!(frame.as_slice(), &full.as_slice()[frame_start..]);
                let request = format!(
                    "ss-image-alpha-planar {w} {h} {} {depth} 0 {} {}",
                    u32::from(gray),
                    hex(&data),
                    hex(full.as_slice())
                );
                let mut args = request.split_whitespace();
                args.next();
                // Output is the same encoded bytes, auxiliary state, and two
                // independent original public-decoder observations. The C++
                // command supplies separate original adapter buffers; Rust's
                // actual separate-buffer frame is compared above.
                expected.push(shared_primitives::observe("ss-image-alpha", &mut args));
                cases.push(request);
            }
        }
    }
    let sources = original_observations(&cases);
    for ((source, native), request) in sources.iter().zip(expected).zip(&cases) {
        assert_eq!(source, &native, "{request}");
    }
    println!("{} C++/Rust differential cases matched", cases.len());
}
