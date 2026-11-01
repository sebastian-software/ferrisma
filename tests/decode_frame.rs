// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Native frame integration; codestream metadata is still caller-supplied.
use jxl::jxl::enc_frame::{LosslessSampleDepth, LosslessThunderConfig};
use jxl::jxl::encode_internal::JxlEncoderChunkedFrameAdapter;
use jxl::jxl::types::{JxlDataType, JxlEndianness, JxlPixelFormat};
#[test]
fn complete_native_frames_recover_rgb_gray_and_uint16_through_original_pipeline() {
    for (w, h) in [(3, 2), (513, 3)] {
        for gray in [false, true] {
            for depth in [8, 16] {
                let channels = if gray { 1 } else { 3 };
                let samples: Vec<u16> = (0..w * h * channels)
                    .map(|i| {
                        let n = (i * 173 + i * i * 29 + 61) as u16;
                        if depth == 8 { u16::from(n as u8) } else { n }
                    })
                    .collect();
                let bytes: Vec<_> = samples
                    .iter()
                    .flat_map(|v| {
                        if depth == 8 {
                            vec![*v as u8]
                        } else {
                            v.to_ne_bytes().to_vec()
                        }
                    })
                    .collect();
                let config = LosslessThunderConfig::new(
                    w,
                    h,
                    gray,
                    if depth == 8 {
                        LosslessSampleDepth::Uint8
                    } else {
                        LosslessSampleDepth::Uint16
                    },
                )
                .unwrap();
                let mut input = JxlEncoderChunkedFrameAdapter::new(w, h, 0);
                assert!(input.set_from_buffer(
                    0,
                    Some(&bytes),
                    bytes.len(),
                    JxlPixelFormat {
                        num_channels: channels as u32,
                        data_type: if depth == 8 {
                            JxlDataType::UINT8
                        } else {
                            JxlDataType::UINT16
                        },
                        endianness: JxlEndianness::NATIVE,
                        align: 0
                    }
                ));
                let frame = config
                    .create_frame()
                    .unwrap()
                    .encode_one_shot(&input, None)
                    .unwrap();
                let decoded = config.decode_frame(frame.as_slice());
                decoded.status.unwrap();
                let image = decoded.image;
                assert_eq!(
                    (image.xsize(), image.ysize(), image.decoded_bytes()),
                    (w, h, frame.len())
                );
                assert_eq!((image.origin.x0, image.origin.y0), (0, 0));
                assert_eq!(image.c_current().is_gray(), gray);
                for c in 0..3 {
                    for y in 0..h {
                        for x in 0..w {
                            let expected = f32::from(
                                samples[(y * w + x) * channels + if gray { 0 } else { c }],
                            ) * (1.0f32 / if depth == 8 { 255.0 } else { 65535.0 });
                            assert_eq!(
                                image.color().plane_row(c, y)[x].to_bits(),
                                expected.to_bits()
                            );
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn frame_profile_admission_prevents_entering_unported_noise_decoding() {
    use jxl::jxl::base::status::StatusCode;
    use jxl::jxl::dec_bit_reader::BitReader;
    use jxl::jxl::enc_bit_writer::BitWriter;
    use jxl::jxl::enc_fields::write_frame_header;
    use jxl::jxl::enc_toc::{write_toc_permutation, write_toc_sizes};
    use jxl::jxl::frame_header::{FLAG_NOISE, FrameHeader, read_frame_header};
    use jxl::jxl::toc::read_toc;
    let config = LosslessThunderConfig::new(1, 1, true, LosslessSampleDepth::Uint8).unwrap();
    let mut input = JxlEncoderChunkedFrameAdapter::new(1, 1, 0);
    assert!(input.set_from_buffer(
        0,
        Some(&[0]),
        1,
        JxlPixelFormat {
            num_channels: 1,
            data_type: JxlDataType::UINT8,
            endianness: JxlEndianness::NATIVE,
            align: 0
        }
    ));
    let bytes = config
        .create_frame()
        .unwrap()
        .encode_one_shot(&input, None)
        .unwrap();
    let mut reader = BitReader::new(bytes.as_slice());
    let mut header = FrameHeader::new(Some(config.metadata()));
    read_frame_header(&mut reader, &mut header).unwrap();
    let mut sizes = Vec::new();
    let mut permutation = Vec::new();
    read_toc(1, &mut reader, &mut sizes, &mut permutation).unwrap();
    let offset = reader.total_bits_consumed() / 8;
    reader.close().unwrap();
    header.flags |= FLAG_NOISE;
    let mut writer = BitWriter::new();
    write_frame_header(&mut header, &mut writer, None).unwrap();
    write_toc_permutation(&[], &mut writer, None).unwrap();
    write_toc_sizes(
        &sizes.iter().map(|&v| v as usize).collect::<Vec<_>>(),
        &mut writer,
        None,
    )
    .unwrap();
    writer
        .append_byte_aligned(&bytes.as_slice()[offset..])
        .unwrap();
    let decoded = config.decode_frame(writer.get_span());
    // This is the closed profile's admission result, not an assertion about
    // the original general decoder's behavior on a noise-enabled frame.
    assert_eq!(decoded.status, Err(StatusCode::Unsupported));
    assert!(decoded.header.is_none());
    assert!(!decoded.image.has_color());
}
