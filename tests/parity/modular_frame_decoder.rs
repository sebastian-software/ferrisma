// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use super::*;
use jxl::jxl::color_encoding::{ColorSpace, RenderingIntent};
use jxl::jxl::dec_modular::ModularFrameDecoder;
use jxl::jxl::frame_header::{ColorTransform, FrameEncoding, FrameHeader};
use jxl::jxl::image_metadata::{CodecMetadata, ExtraChannelInfo};
use jxl::jxl::modular::modular_image::Image;

pub(super) fn handles(kind: &str) -> bool {
    kind == "mfd-global"
}
fn decoder(out: &mut String, d: &ModularFrameDecoder, pixels: bool) {
    write!(
        out,
        " D {} {} {} {} {}",
        u32::from(d.have_memory_manager()),
        u32::from(d.do_color()),
        u32::from(d.have_dc()),
        u32::from(d.uses_full_image()),
        u32::from(d.all_same_shift())
    )
    .unwrap();
    frame::dimensions(out, d.frame_dimensions());
    modular_frame::image(out, d.full_image(), false);
    let mut reading = pixels;
    for (c, ch) in d.full_image().channel.iter().enumerate() {
        if c >= d.full_image().nb_meta_channels
            && (ch.w > d.frame_dimensions().group_dim || ch.h > d.frame_dimensions().group_dim)
        {
            reading = false;
        }
        let ready = reading && ch.plane.xsize() != 0 && ch.plane.ysize() != 0;
        write!(out, " {}", u32::from(ready)).unwrap();
        if ready {
            for y in 0..ch.h {
                for x in 0..ch.w {
                    write!(out, " {}", ch.row(y)[x]).unwrap();
                }
            }
        }
    }
    let moved = Image {
        transform: d.global_transforms().to_vec(),
        ..Image::default()
    };
    modular_frame::image(out, &moved, false);
    ma::tree(out, d.tree());
    write!(out, " {}", d.context_map().len()).unwrap();
    for v in d.context_map() {
        write!(out, " {v}").unwrap();
    }
    ans::code_observation(out, d.code(), false);
    ma::header(out, &mut d.global_header().clone());
}
pub(super) fn observe(kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    assert_eq!(kind, "mfd-global");
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
    let allow = number::<u32>(args) != 0;
    let drop = number::<u32>(args) != 0;
    let repeat = number::<usize>(args);
    let prefix = number(args);
    let bytes = unhex(args.next().unwrap());
    let mut metadata = CodecMetadata::default();
    metadata.size.set(w, h).unwrap();
    metadata.m.bit_depth.bits_per_sample = bits;
    metadata.m.bit_depth.floating_point_sample = fp;
    metadata.m.xyb_encoded = false;
    metadata.m.num_extra_channels = extras as u32;
    metadata.m.extra_channel_info = vec![ExtraChannelInfo::default(); extras];
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
    let mut d = ModularFrameDecoder::new();
    d.init(&header.to_frame_dimensions());
    let mut out = String::new();
    for _ in 0..repeat {
        let mut reader = BitReader::new(&bytes);
        reader.skip_bits(prefix);
        let status = d.decode_global_info(&mut reader, &header, allow);
        let consumed = reader.total_bits_consumed();
        let closed = reader.close();
        if drop {
            d.maybe_drop_full_image();
        }
        write!(
            out,
            "{} {consumed} {}",
            status_code(&status),
            status_code(&closed)
        )
        .unwrap();
        decoder(&mut out, &d, status.is_ok());
        out.push_str(" | ");
    }
    out
}

#[test]
fn original_thunder_global_streams_decode_with_shared_trees_partial_state_and_plane_release() {
    let mut sources = Vec::new();
    let mut settings = Vec::new();
    for (w, h) in [
        (1, 1),
        (17, 9),
        (129, 3),
        (257, 3),
        (513, 3),
        (2049, 3),
        (65, 67),
    ] {
        for gray in [0, 1] {
            for depth in [8, 16] {
                for mode in 0..3 {
                    sources.push(format!("mfd-source {w} {h} {gray} {depth} {mode}"));
                    settings.push((w, h, gray, depth));
                }
            }
        }
    }
    let mut cases = Vec::new();
    for ((w, h, gray, depth), output) in settings.into_iter().zip(original_observations(&sources)) {
        let mut fields = output.split_whitespace();
        let shift = fields.next().unwrap();
        let data = unhex(fields.next().unwrap());
        let mut lengths = vec![
            0,
            1,
            2,
            data.len() / 2,
            data.len().saturating_sub(1),
            data.len(),
        ];
        lengths.sort_unstable();
        lengths.dedup();
        for len in lengths {
            if len > data.len() {
                continue;
            }
            for allow in [0, 1] {
                for drop in [0, 1] {
                    cases.push(format!("mfd-global {w} {h} {gray} {depth} 0 1 1 {shift} 1 0 1 0 {allow} {drop} 2 1 {}", hex(&data[..len])));
                }
            }
        }
        for at in [0, 1, data.len() / 2, data.len() - 1] {
            for mask in [1, 128] {
                let mut corrupt = data.clone();
                corrupt[at] ^= mask;
                cases.push(format!(
                    "mfd-global {w} {h} {gray} {depth} 0 1 1 {shift} 1 0 1 0 0 0 1 1 {}",
                    hex(&corrupt)
                ));
            }
        }
    }
    compare(cases);
}

#[test]
fn local_global_streams_cover_gray_ycbcr_xyb_extra_shifts_vardct_and_depth_rejections() {
    let mut headers = Vec::new();
    for modular in [0, 1] {
        for transform in [0, 1, 2] {
            for gray in [0, 1] {
                for (extras, ecups, ups) in [(0, 1, 1), (1, 1, 1), (2, 4, 2), (1, 8, 1)] {
                    for bits in [8, 16, 32, 33] {
                        let fp = u32::from(bits == 32);
                        headers.push(format!("17 9 {gray} {bits} {fp} {transform} {modular} 0 {ups} {extras} {ecups} {}", u32::from(transform == 2)));
                    }
                }
            }
        }
    }
    let sources: Vec<_> = headers
        .iter()
        .map(|h| format!("mfd-local-source {h} 1"))
        .collect();
    let mut cases = Vec::new();
    for (header, output) in headers.iter().zip(original_observations(&sources)) {
        let data = unhex(&output);
        for len in [0, 1, data.len() / 2, data.len()] {
            for allow in [0, 1] {
                cases.push(format!(
                    "mfd-global {header} {allow} 0 2 0 {}",
                    hex(&data[..len])
                ));
            }
        }
    }
    // The original >=32-bit guards occur after the tree flag and before image
    // construction, including the special uint32 rejection and XYB exemption.
    for fp in [0, 1] {
        for bits in [31, 32, 33, u32::MAX] {
            for transform in [0, 1, 2] {
                cases.push(format!(
                    "mfd-global 1 1 0 {bits} {fp} {transform} 1 0 1 0 1 0 0 0 1 0 00"
                ));
            }
        }
    }
    compare(cases);
}
