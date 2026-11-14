// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Component integration: encoded sections still have caller-supplied metadata.
use jxl::jxl::base::rect::Rect;
use jxl::jxl::dec_bit_reader::BitReader;
use jxl::jxl::dec_modular::{ModularFrameDecoder, ModularStreamId};
use jxl::jxl::enc_frame::{LosslessSampleDepth, LosslessThunderConfig};
use jxl::jxl::encode_internal::JxlEncoderChunkedFrameAdapter;
use jxl::jxl::image::Image3;
use jxl::jxl::passes_state::PassesSharedState;
use jxl::jxl::render_pipeline::render_pipeline_stage::WriteToImage3FStage;
use jxl::jxl::render_pipeline::simple_render_pipeline::SimpleRenderPipelineBuilder;
use jxl::jxl::toc::ac_group_index;
use jxl::jxl::types::{JxlDataType, JxlEndianness, JxlPixelFormat};

#[test]
fn native_sections_recover_gray_rgb_and_uint16_normalization_after_inplace_move() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    for w in [17, 513] {
        let h = 3;
        for gray in [false, true] {
            for depth in [8, 16] {
                let channels = if gray { 1 } else { 3 };
                let samples: Vec<u16> = (0..w * h * channels)
                    .map(|i| {
                        let v = (i * 173 + i * i * 29 + 61) as u16;
                        if depth == 8 { u16::from(v as u8) } else { v }
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
                let mut encoder = config.create_frame().unwrap();
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
                        align: 0,
                    }
                ));
                let mut codes = Vec::new();
                encoder
                    .compute_encoding_data(&input, &mut codes, None)
                    .unwrap();
                let header = encoder.header();
                let dim = header.to_frame_dimensions();
                let mut readers: Vec<_> =
                    codes.iter().map(|v| BitReader::new(v.get_span())).collect();
                let index = |id: usize| if codes.len() == 1 { 0 } else { id };
                let mut shared = PassesSharedState::new();
                shared.metadata = Some(config.metadata());
                shared.frame_dim = dim;
                shared.matrices.decode_dc(&mut readers[0]).unwrap();
                let mut decoder = ModularFrameDecoder::new();
                decoder.init(&dim);
                decoder
                    .decode_global_info(&mut readers[0], header, false)
                    .unwrap();
                for g in 0..dim.num_dc_groups {
                    decoder
                        .decode_group(
                            header,
                            Rect::new(
                                (g % dim.xsize_dc_groups) * dim.dc_group_dim,
                                (g / dim.xsize_dc_groups) * dim.dc_group_dim,
                                dim.dc_group_dim,
                                dim.dc_group_dim,
                            ),
                            Some(&mut readers[index(1 + g)]),
                            3,
                            1000,
                            &ModularStreamId::modular_dc(g),
                            false,
                            None,
                            None,
                            false,
                            None,
                        )
                        .unwrap();
                }
                decoder.maybe_drop_full_image();
                let mut output = Image3::default();
                {
                    let mut builder = SimpleRenderPipelineBuilder::new(3);
                    builder
                        .add_stage(Some(Box::new(WriteToImage3FStage::new(&mut output))))
                        .unwrap();
                    let mut pipeline = builder.finalize(dim).unwrap();
                    pipeline.prepare_for_threads(1, false).unwrap();
                    for g in 0..dim.num_groups {
                        let mut input = pipeline.get_input_buffers(g, 0);
                        let mut ready = true;
                        let (min, max) = header.passes.get_downsampling_bracket(0);
                        decoder
                            .decode_group(
                                header,
                                Rect::new(
                                    (g % dim.xsize_groups) * dim.group_dim,
                                    (g / dim.xsize_groups) * dim.group_dim,
                                    dim.group_dim,
                                    dim.group_dim,
                                ),
                                Some(
                                    &mut readers[index(ac_group_index(
                                        0,
                                        g,
                                        dim.num_groups,
                                        dim.num_dc_groups,
                                    ))],
                                ),
                                min,
                                max,
                                &ModularStreamId::modular_ac(g, 0),
                                false,
                                Some(&shared),
                                Some(&mut input),
                                false,
                                Some(&mut ready),
                            )
                            .unwrap();
                        if !decoder.uses_full_image() && ready {
                            input.done().unwrap();
                        }
                    }
                    decoder
                        .finalize_decoding(header, &shared, &mut *pipeline, None, true)
                        .unwrap();
                }
                assert_eq!((output.xsize(), output.ysize()), (w, h));
                assert_eq!(decoder.full_image().bitdepth, depth);
                for c in 0..3 {
                    for y in 0..h {
                        for x in 0..w {
                            let sample = samples[(y * w + x) * channels + if gray { 0 } else { c }];
                            let expected = f32::from(sample)
                                * (1.0f32 / if depth == 8 { 255.0 } else { 65535.0 });
                            assert_eq!(
                                output.plane_row(c, y)[x].to_bits(),
                                expected.to_bits(),
                                "w={w} gray={gray} depth={depth} c={c} x={x} y={y}"
                            );
                        }
                    }
                }
                for reader in &mut readers {
                    reader.close().unwrap();
                }
            }
        }
    }
}
