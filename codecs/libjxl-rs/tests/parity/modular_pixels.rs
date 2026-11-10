// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use super::*;
use jxl::jxl::base::rect::Rect;
use jxl::jxl::color_encoding::{ColorSpace, RenderingIntent};
use jxl::jxl::dec_modular::ModularFrameDecoder;
use jxl::jxl::frame_header::{ColorTransform, FrameEncoding, FrameHeader};
use jxl::jxl::image::Image3;
use jxl::jxl::image_metadata::{CodecMetadata, ExtraChannelInfo};
use jxl::jxl::modular::modular_image::Image;
use jxl::jxl::modular::transform::transform::{Transform, TransformId};
use jxl::jxl::passes_state::PassesSharedState;
use jxl::jxl::render_pipeline::render_pipeline_stage::WriteToImage3FStage;
use jxl::jxl::render_pipeline::simple_render_pipeline::SimpleRenderPipelineBuilder;

pub(super) fn observe(args: &mut std::str::SplitWhitespace<'_>) -> String {
    let w = number(args);
    let h = number(args);
    let gray = number::<u32>(args) != 0;
    let bits = number(args);
    let fp = number::<u32>(args) != 0;
    let transform = number(args);
    let modular = number::<u32>(args) != 0;
    let shift = number(args);
    let ups = number(args);
    let extras = number::<usize>(args);
    let ecups = number(args);
    let subsample = number::<u32>(args) != 0;
    let exp = number(args);
    let ecbits = number(args);
    let ecfp = number::<u32>(args) != 0;
    let ecexp = number(args);
    let mutation = number::<u32>(args);
    let rect = Rect::new(number(args), number(args), number(args), number(args));
    let bytes = unhex(args.next().unwrap());
    let mut metadata = CodecMetadata::default();
    metadata.size.set(w, h).unwrap();
    metadata.m.bit_depth.bits_per_sample = bits;
    metadata.m.bit_depth.floating_point_sample = fp;
    metadata.m.bit_depth.exponent_bits_per_sample = exp;
    metadata.m.xyb_encoded = false;
    metadata.m.num_extra_channels = extras as u32;
    metadata.m.extra_channel_info = vec![ExtraChannelInfo::default(); extras];
    for ec in &mut metadata.m.extra_channel_info {
        ec.bit_depth.bits_per_sample = ecbits;
        ec.bit_depth.floating_point_sample = ecfp;
        ec.bit_depth.exponent_bits_per_sample = ecexp;
    }
    metadata
        .m
        .color_encoding
        .set_srgb(
            if gray {
                ColorSpace::GRAY
            } else {
                ColorSpace::RGB
            },
            RenderingIntent::RELATIVE,
        )
        .unwrap();
    let mut header = FrameHeader::new(Some(&metadata));
    header.color_transform = ColorTransform(transform);
    header.encoding = if modular {
        FrameEncoding::MODULAR
    } else {
        FrameEncoding::VAR_DCT
    };
    header.group_size_shift = shift;
    header.upsampling = ups;
    header.extra_channel_upsampling = vec![ecups; extras];
    if subsample {
        header
            .chroma_subsampling
            .set(&[2, 1, 1], &[2, 1, 1])
            .unwrap();
    }
    let dim = header.to_frame_dimensions();
    let mut decoder = ModularFrameDecoder::new();
    decoder.init(&dim);
    let mut reader = BitReader::new(&bytes);
    let mut out = String::new();
    let status = decoder.decode_global_info(&mut reader, &header, false);
    write!(out, " G {}", status_code(&status)).unwrap();
    if status.is_ok() {
        let mut gi = Image::clone_image(decoder.full_image()).unwrap();
        gi.undo_transforms(&decoder.global_header().wp_header, None);
        match mutation {
            1 => gi.transform.push(Transform::new(TransformId::RCT)),
            2 => gi.channel.clear(),
            3 if !gi.channel.is_empty() => gi.channel[0].w = 0,
            4 if gi.channel.len() > 1 => gi.channel[1].h = 0,
            5 if !gi.channel.is_empty() => gi.channel[0].hshift = 4,
            6 if !gi.channel.is_empty() => {
                gi.channel.pop();
            }
            _ => {}
        }
        let mut shared = PassesSharedState::new();
        shared.frame_dim = dim;
        shared.metadata = Some(&metadata);
        let mut output = Image3::default();
        let mut builder = SimpleRenderPipelineBuilder::new(3 + extras);
        builder
            .add_stage(Some(Box::new(WriteToImage3FStage::new(&mut output))))
            .unwrap();
        let mut pipeline = builder.finalize(dim).unwrap();
        pipeline.prepare_for_threads(1, false).unwrap();
        let mut input = pipeline.get_input_buffers(0, 0);
        for c in 0..3 + extras {
            let (plane, r) = input.get_buffer_mut(c);
            for y in 0..r.ysize() {
                plane.row_mut(r.y0() + y)[r.x0()..r.x1()].fill(-7.0);
            }
        }
        let status =
            decoder.modular_image_to_decoded_rect(&header, &gi, Some(&shared), &mut input, rect);
        write!(out, " C {}", status_code(&status)).unwrap();
        for c in 0..3 + extras {
            let (plane, r) = input.get_buffer(c);
            write!(out, " I {} {}", r.xsize(), r.ysize()).unwrap();
            for y in 0..r.ysize() {
                for x in 0..r.xsize() {
                    write!(out, " {}", plane.row(r.y0() + y)[r.x0() + x].to_bits()).unwrap();
                }
            }
        }
    }
    let consumed = reader.total_bits_consumed();
    let closed = reader.close();
    write!(out, " B {consumed} {}", status_code(&closed)).unwrap();
    out
}

#[test]
fn modular_conversion_covers_float_xyb_gray_extra_channels_and_partial_errors() {
    let mut headers = Vec::new();
    for modular in [0, 1] {
        for transform in [0, 1, 2] {
            for gray in [0, 1] {
                for (bits, fp, exp) in [
                    (8, 0, 0),
                    (16, 0, 0),
                    (23, 0, 0),
                    (31, 0, 0),
                    (16, 1, 5),
                    (32, 1, 8),
                    (32, 1, 7),
                ] {
                    for (extras, ecups, ups, subsample) in
                        [(0, 1, 1, 0), (1, 1, 1, 0), (2, 4, 2, 0), (1, 1, 1, 1)]
                    {
                        headers.push((format!("17 9 {gray} {bits} {fp} {transform} {modular} 0 {ups} {extras} {ecups} {subsample}"), exp));
                    }
                }
            }
        }
    }
    let sources: Vec<_> = headers
        .iter()
        .map(|(h, _)| format!("mfd-local-source {h} 1"))
        .collect();
    let mut cases = Vec::new();
    for ((header, exp), data) in headers.iter().zip(original_observations(&sources)) {
        for (ecbits, ecfp, ecexp) in [
            (8, 0, 0),
            (0, 0, 0),
            (32, 0, 0),
            (16, 1, 5),
            (32, 1, 8),
            (32, 1, 7),
        ] {
            cases.push(format!(
                "mg-pixels {header} {exp} {ecbits} {ecfp} {ecexp} 0 0 0 17 9 {data}"
            ));
        }
        for mutation in 1..=6 {
            cases.push(format!(
                "mg-pixels {header} {exp} 8 0 0 {mutation} 0 0 17 9 {data}"
            ));
        }
        for rect in ["0 0 16 9", "1 0 17 9", "0 1 17 9", "0 0 17 8"] {
            cases.push(format!("mg-pixels {header} {exp} 8 0 0 0 {rect} {data}"));
        }
    }
    compare(cases);
}
