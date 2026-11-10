// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// State and byte observations of original/translated frame bundles and geometry.

use super::*;
use jxl::jxl::base::rect::Rect;
use jxl::jxl::enc_fields::write_frame_header;
use jxl::jxl::frame_dimensions::FrameDimensions;
use jxl::jxl::frame_header::*;
use jxl::jxl::image_metadata::{CodecMetadata, ExtraChannelInfo};
use jxl::jxl::loop_filter::LoopFilter;
use jxl::jxl::pack_signed::{pack_signed, unpack_signed};

pub(super) fn handles(kind: &str) -> bool {
    kind.starts_with("frame-")
}
fn rect(out: &mut String, r: Rect) {
    write!(
        out,
        " {},{},{},{},{},{}",
        r.x0(),
        r.y0(),
        r.xsize(),
        r.ysize(),
        r.x1(),
        r.y1()
    )
    .unwrap();
}
pub(super) fn dimensions(out: &mut String, d: &FrameDimensions) {
    for v in [
        d.xsize,
        d.ysize,
        d.xsize_upsampled,
        d.ysize_upsampled,
        d.xsize_upsampled_padded,
        d.ysize_upsampled_padded,
        d.xsize_padded,
        d.ysize_padded,
        d.xsize_blocks,
        d.ysize_blocks,
        d.xsize_groups,
        d.ysize_groups,
        d.xsize_dc_groups,
        d.ysize_dc_groups,
        d.num_groups,
        d.num_dc_groups,
        d.group_dim,
        d.dc_group_dim,
    ] {
        write!(out, " {v}").unwrap();
    }
}
pub(super) fn filter(out: &mut String, f: &LoopFilter) {
    write!(
        out,
        " {},{},{},{},{},{},{},{},{}",
        u32::from(f.all_default),
        u32::from(f.gab),
        u32::from(f.gab_custom),
        f.epf_iters,
        u32::from(f.epf_sharp_custom),
        u32::from(f.epf_weight_custom),
        u32::from(f.epf_sigma_custom),
        f.extensions,
        u32::from(f.nonserialized_is_modular)
    )
    .unwrap();
    for v in [
        f.gab_x_weight1,
        f.gab_x_weight2,
        f.gab_y_weight1,
        f.gab_y_weight2,
        f.gab_b_weight1,
        f.gab_b_weight2,
    ]
    .into_iter()
    .chain(f.epf_sharp_lut)
    .chain(f.epf_channel_scale)
    .chain([
        f.epf_pass1_zeroflush,
        f.epf_pass2_zeroflush,
        f.epf_quant_mul,
        f.epf_pass0_sigma_scale,
        f.epf_pass2_sigma_scale,
        f.epf_border_sad_mul,
        f.epf_sigma_for_modular,
    ]) {
        write!(out, " {}", v.to_bits()).unwrap();
    }
}
pub(super) fn configure_filter(f: &mut LoopFilter, variant: u32) {
    if variant == 3 {
        f.epf_sigma_for_modular = 1e-8;
    }
    if variant == 1 {
        f.gab = false;
        f.epf_iters = 0;
    }
    if variant == 2 {
        f.gab_custom = true;
        f.gab_x_weight1 = 0.125;
        f.gab_y_weight2 = 0.03125;
        f.epf_sharp_custom = true;
        f.epf_sharp_lut[3] = 0.375;
        f.epf_weight_custom = true;
        f.epf_channel_scale[1] = 4.0;
        f.epf_sigma_custom = true;
        f.epf_quant_mul = 0.5;
        f.epf_pass0_sigma_scale = 0.75;
        f.epf_sigma_for_modular = 0.5;
    }
}
pub(super) fn subsampling(out: &mut String, c: &YCbCrChromaSubsampling) {
    write!(
        out,
        " {},{},{},{},{},{}",
        c.max_h_shift(),
        c.max_v_shift(),
        u32::from(c.is_444()),
        u32::from(c.is_420()),
        u32::from(c.is_422()),
        u32::from(c.is_440())
    )
    .unwrap();
    for i in 0..3 {
        write!(
            out,
            " {},{},{},{}",
            c.raw_h_shift(i),
            c.raw_v_shift(i),
            c.h_shift(i),
            c.v_shift(i)
        )
        .unwrap();
    }
}
pub(super) fn passes(out: &mut String, p: &Passes) {
    write!(out, " {},{}", p.num_passes, p.num_downsample).unwrap();
    for v in p.downsample.iter().chain(&p.last_pass).chain(&p.shift) {
        write!(out, " {v}").unwrap();
    }
}
fn pass_targets(out: &mut String, p: &Passes) {
    for i in 0..p.num_passes as usize {
        let (min, max) = p.get_downsampling_bracket(i);
        write!(out, " {min},{max}").unwrap();
    }
    for i in 0..=p.num_passes + 1 {
        write!(
            out,
            " {}",
            p.get_downsampling_target_for_completed_passes(i)
        )
        .unwrap();
    }
}
fn blending(out: &mut String, b: &BlendingInfo) {
    write!(
        out,
        " {},{},{},{},{},{}",
        b.mode.0,
        b.alpha_channel,
        u32::from(b.clamp),
        b.source,
        b.nonserialized_num_extra_channels,
        u32::from(b.nonserialized_is_partial_frame)
    )
    .unwrap();
}
pub(super) fn frame(out: &mut String, f: &FrameHeader<'_>, with_dimensions: bool) {
    write!(
        out,
        " {},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
        u32::from(f.all_default),
        f.encoding.0,
        f.frame_type.0,
        f.flags,
        f.color_transform.0,
        f.group_size_shift,
        f.x_qm_scale,
        f.b_qm_scale,
        u32::from(f.custom_size_or_origin),
        f.frame_size.xsize,
        f.frame_size.ysize,
        f.upsampling,
        f.frame_origin.x0,
        f.frame_origin.y0,
        u32::from(f.is_last),
        f.save_as_reference,
        u32::from(f.save_before_color_transform),
        f.dc_level,
        u32::from(f.nonserialized_is_preview),
        f.extensions
    )
    .unwrap();
    write!(out, " {}", hex(&f.name)).unwrap();
    subsampling(out, &f.chroma_subsampling);
    passes(out, &f.passes);
    write!(out, " {}", f.extra_channel_upsampling.len()).unwrap();
    for v in &f.extra_channel_upsampling {
        write!(out, " {v}").unwrap();
    }
    blending(out, &f.blending_info);
    write!(out, " {}", f.extra_channel_blending_info.len()).unwrap();
    for b in &f.extra_channel_blending_info {
        blending(out, b);
    }
    write!(
        out,
        " {},{},{},{},{},{},{}",
        f.animation_frame.duration,
        f.animation_frame.timecode,
        u32::from(f.animation_frame.nonserialized_metadata.is_some()),
        u32::from(f.can_be_referenced()),
        u32::from(f.needs_color_transform()),
        f.default_xsize(),
        f.default_ysize()
    )
    .unwrap();
    filter(out, &f.loop_filter);
    if with_dimensions {
        dimensions(out, &f.to_frame_dimensions());
    }
}
pub(super) fn configure_metadata(args: &mut std::str::SplitWhitespace<'_>) -> CodecMetadata {
    let xyb: u32 = number(args);
    let animation: u32 = number(args);
    let extras: usize = number(args);
    let mut m = CodecMetadata::default();
    m.size.set(513, 257).unwrap();
    m.m.preview_size.set(32, 16).unwrap();
    m.m.xyb_encoded = xyb != 0;
    m.m.have_animation = animation != 0;
    m.m.animation.have_timecodes = animation != 0;
    m.m.extra_channel_info
        .resize_with(extras, ExtraChannelInfo::default);
    for (i, e) in m.m.extra_channel_info.iter_mut().enumerate() {
        e.dim_shift = (i % 2) as u32;
    }
    m
}
pub(super) fn observe(kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    match kind {
        "frame-dimensions" => {
            let x = number(args);
            let y = number(args);
            let g = number(args);
            let h = number(args);
            let v = number(args);
            let modular: u32 = number(args);
            let up = number(args);
            let index: usize = number(args);
            let mut d = FrameDimensions::default();
            d.set(x, y, g, h, v, modular != 0, up);
            let mut out = "D".to_owned();
            dimensions(&mut out, &d);
            if d.num_groups > 0 {
                rect(&mut out, d.group_rect(index % d.num_groups));
                rect(&mut out, d.block_group_rect(index % d.num_groups));
            }
            if d.num_dc_groups > 0 {
                rect(&mut out, d.dc_group_rect(index % d.num_dc_groups));
            }
            out
        }
        "frame-rect" => {
            let dx = number(args);
            let dy = number(args);
            let border = number(args);
            let a = Rect::new(number(args), number(args), number(args), number(args));
            let b = Rect::new(number(args), number(args), number(args), number(args));
            let mut out = format!("{} {}", u32::from(a.is_inside(b)), u32::from(a.is_same(b)));
            rect(&mut out, a.intersection(b));
            rect(&mut out, a.translate(dx, dy));
            rect(&mut out, a.extend(border, b));
            out
        }
        "frame-signed" => format!(
            "{} {}",
            pack_signed(number(args)),
            unpack_signed(number(args))
        ),
        "frame-subsampling" => {
            let h = std::array::from_fn(|_| number::<u32>(args) as u8);
            let v = std::array::from_fn(|_| number::<u32>(args) as u8);
            let mut c = YCbCrChromaSubsampling::default();
            let status = c.set(&h, &v);
            let mut out = status_code(&status).to_string();
            subsampling(&mut out, &c);
            out
        }
        "frame-passes" => {
            let mut p = Passes {
                num_passes: number(args),
                num_downsample: number(args),
                ..Passes::default()
            };
            let offset = number(args);
            for a in [&mut p.downsample, &mut p.last_pass, &mut p.shift] {
                for v in a {
                    *v = number(args);
                }
            }
            let can = bundle::can_encode(&mut p);
            let (ext, bits) = can.unwrap_or((0, 0));
            let mut out = format!("{} {ext} {bits}", status_code(&can.map(|_| ())));
            if can.is_ok() {
                let mut writer = BitWriter::new();
                prefix(&mut writer, offset);
                let status = bundle::write(&mut p, &mut writer, LayerType::Header, None);
                write!(out, " {} {}", status_code(&status), writer.bits_written()).unwrap();
                pad(&mut writer);
                write!(out, " {}", hex(writer.get_span())).unwrap();
                let mut decoded = Passes::default();
                let mut br = BitReader::new(writer.get_span());
                br.skip_bits(offset);
                let read = bundle::read(&mut br, &mut decoded);
                write!(
                    out,
                    " {} {} {}",
                    status_code(&read),
                    br.total_bits_consumed(),
                    status_code(&br.close())
                )
                .unwrap();
                passes(&mut out, &decoded);
                if read.is_ok() {
                    pass_targets(&mut out, &decoded);
                }
            }
            passes(&mut out, &p);
            out
        }
        "frame-passes-read" => {
            let offset = number(args);
            let bytes = unhex(args.next().unwrap());
            let mut p = Passes::default();
            let mut br = BitReader::new(&bytes);
            br.skip_bits(offset);
            let read = bundle::read(&mut br, &mut p);
            let mut out = format!(
                "{} {} {}",
                status_code(&read),
                br.total_bits_consumed(),
                status_code(&br.close())
            );
            passes(&mut out, &p);
            if read.is_ok() {
                pass_targets(&mut out, &p);
            }
            out
        }
        "frame-filter" => {
            let modular: u32 = number(args);
            let variant = number(args);
            let offset = number(args);
            let mut f = LoopFilter {
                nonserialized_is_modular: modular != 0,
                ..LoopFilter::default()
            };
            configure_filter(&mut f, variant);
            let mut writer = BitWriter::new();
            prefix(&mut writer, offset);
            let can = bundle::can_encode(&mut f);
            let (ext, bits) = can.unwrap_or((0, 0));
            let mut out = format!("{} {ext} {bits}", status_code(&can.map(|_| ())));
            if can.is_ok() {
                let status = bundle::write(&mut f, &mut writer, LayerType::Header, None);
                write!(out, " {} {}", status_code(&status), writer.bits_written()).unwrap();
                pad(&mut writer);
                write!(out, " {}", hex(writer.get_span())).unwrap();
                let mut decoded = LoopFilter {
                    nonserialized_is_modular: modular != 0,
                    ..LoopFilter::default()
                };
                let mut br = BitReader::new(writer.get_span());
                br.skip_bits(offset);
                let read = bundle::read(&mut br, &mut decoded);
                write!(
                    out,
                    " {} {} {}",
                    status_code(&read),
                    br.total_bits_consumed(),
                    status_code(&br.close())
                )
                .unwrap();
                filter(&mut out, &decoded);
            }
            filter(&mut out, &f);
            out
        }
        "frame-write" => {
            let metadata = configure_metadata(args);
            let preview: u32 = number(args);
            let modular: u32 = number(args);
            let mut f = FrameHeader::new(Some(&metadata));
            f.nonserialized_is_preview = preview != 0;
            f.encoding = if modular != 0 {
                FrameEncoding::MODULAR
            } else {
                FrameEncoding::VAR_DCT
            };
            f.frame_type = FrameType(number(args));
            f.dc_level = u32::from(f.frame_type == FrameType::DC);
            f.flags = number(args);
            f.color_transform = ColorTransform(number(args));
            f.group_size_shift = number(args);
            f.upsampling = number(args);
            for (i, u) in f.extra_channel_upsampling.iter_mut().enumerate() {
                *u = f.upsampling.max(1 << (i % 2));
            }
            f.blending_info.mode = BlendMode(number(args));
            f.blending_info.source = 2;
            f.blending_info.alpha_channel = 0;
            f.blending_info.clamp = true;
            configure_filter(&mut f.loop_filter, number(args));
            f.is_last = number::<u32>(args) != 0;
            f.save_as_reference = 1;
            f.custom_size_or_origin = number::<u32>(args) != 0;
            f.name = b"frame".to_vec();
            f.animation_frame.duration = if metadata.m.have_animation { 17 } else { 0 };
            f.animation_frame.timecode = 0x12345678;
            if f.custom_size_or_origin {
                f.frame_size = FrameSize {
                    xsize: 400,
                    ysize: 200,
                };
                if f.frame_type == FrameType::REGULAR || f.frame_type == FrameType::SKIP_PROGRESSIVE
                {
                    f.frame_origin = FrameOrigin { x0: -2, y0: 3 };
                }
                if f.frame_type == FrameType::REFERENCE_ONLY {
                    f.save_before_color_transform = true;
                }
            }
            if f.color_transform == ColorTransform::YCBCR {
                f.chroma_subsampling.set(&[2, 1, 1], &[2, 1, 1]).unwrap();
            }
            let offset = number(args);
            let mut writer = BitWriter::new();
            prefix(&mut writer, offset);
            let mut aux = AuxOut::default();
            let can = bundle::can_encode(&mut f);
            let (ext, bits) = can.unwrap_or((0, 0));
            let mut out = format!("{} {ext} {bits}", status_code(&can.map(|_| ())));
            if can.is_ok() {
                let status = write_frame_header(&mut f, &mut writer, Some(&mut aux));
                write!(out, " {} {}", status_code(&status), writer.bits_written()).unwrap();
                pad(&mut writer);
                write!(out, " {}", hex(writer.get_span())).unwrap();
                let mut decoded = FrameHeader::new(Some(&metadata));
                decoded.nonserialized_is_preview = preview != 0;
                let mut br = BitReader::new(writer.get_span());
                br.skip_bits(offset);
                let read = read_frame_header(&mut br, &mut decoded);
                write!(
                    out,
                    " {} {} {}",
                    status_code(&read),
                    br.total_bits_consumed(),
                    status_code(&br.close())
                )
                .unwrap();
                frame(&mut out, &decoded, read.is_ok());
                let a = aux.layer(LayerType::Header);
                write!(out, " {},{}", a.total_bits, a.histogram_bits).unwrap();
            }
            frame(&mut out, &f, can.is_ok());
            out
        }
        "frame-read" => {
            let metadata = configure_metadata(args);
            let preview: u32 = number(args);
            let offset = number(args);
            let bytes = unhex(args.next().unwrap());
            let mut f = FrameHeader::new(Some(&metadata));
            f.nonserialized_is_preview = preview != 0;
            let mut br = BitReader::new(&bytes);
            br.skip_bits(offset);
            let read = read_frame_header(&mut br, &mut f);
            let mut out = format!(
                "{} {} {}",
                status_code(&read),
                br.total_bits_consumed(),
                status_code(&br.close())
            );
            frame(&mut out, &f, read.is_ok());
            out
        }
        "frame-filter-read" => {
            let modular: u32 = number(args);
            let offset = number(args);
            let bytes = unhex(args.next().unwrap());
            let mut f = LoopFilter {
                nonserialized_is_modular: modular != 0,
                ..LoopFilter::default()
            };
            let mut br = BitReader::new(&bytes);
            br.skip_bits(offset);
            let read = bundle::read(&mut br, &mut f);
            let mut out = format!(
                "{} {} {}",
                status_code(&read),
                br.total_bits_consumed(),
                status_code(&br.close())
            );
            filter(&mut out, &f);
            out
        }
        _ => panic!("unknown frame request"),
    }
}

#[test]
fn frame_dimensions_and_rectangles_preserve_units_padding_and_unsigned_edges() {
    let mut cases = Vec::new();
    for x in [0, 1, 7, 8, 15, 16, 127, 128, 129, 255, 256, 257, 513, 4096] {
        for y in [0, 1, 8, 127, 257, 4097] {
            for group in 0..4 {
                for variant in 0..8 {
                    cases.push(format!(
                        "frame-dimensions {x} {y} {group} {} {} {} {} {}",
                        variant & 1,
                        (variant >> 1) & 1,
                        (variant >> 2) & 1,
                        1 << (variant % 4),
                        variant * 17
                    ));
                }
            }
        }
    }
    let mut seed = 57219;
    for i in 0..1024 {
        let mut v = [0; 8];
        for x in &mut v {
            *x = next(&mut seed) % 1024;
        }
        if i % 4 == 0 {
            v[4..].copy_from_slice(&[0, 0, 2048, 2048]);
        }
        cases.push(format!(
            "frame-rect {} {} {} {} {} {} {} {} {} {} {}",
            (next(&mut seed) % 256) as i64 - 128,
            (next(&mut seed) % 256) as i64 - 128,
            i % 32,
            v[0],
            v[1],
            v[2],
            v[3],
            v[4],
            v[5],
            v[6],
            v[7]
        ));
    }
    for value in [0, 1, 7, usize::MAX - 7, usize::MAX] {
        cases.push(format!(
            "frame-rect -9 7 3 {value} {value} 16 4 0 0 {} {}",
            usize::MAX,
            usize::MAX
        ));
    }
    compare(cases);
}
#[test]
fn signed_packing_and_subsampling_preserve_widths_and_partial_setter_state() {
    let mut cases = Vec::new();
    let mut seed = 75429;
    for value in [
        i32::MIN,
        i32::MIN + 1,
        -65536,
        -2,
        -1,
        0,
        1,
        2,
        65535,
        i32::MAX,
    ] {
        for packed in [0, 1, 2, 3, u32::MAX as usize, usize::MAX] {
            cases.push(format!("frame-signed {value} {packed}"));
        }
    }
    for _ in 0..1024 {
        cases.push(format!(
            "frame-signed {} {}",
            next(&mut seed) as i32,
            next(&mut seed)
        ));
    }
    for modes in 0..64 {
        let v: Vec<_> = (0..6).map(|i| 1 + ((modes >> i) & 1)).collect();
        cases.push(format!(
            "frame-subsampling {} {} {} {} {} {}",
            v[0], v[1], v[2], v[3], v[4], v[5]
        ));
    }
    // The first channel changes, then the invalid second channel leaves cached
    // maxima unchanged. uint8 subtraction is promoted to int before size_t.
    cases.push("frame-subsampling 3 2 1 1 1 1".into());
    cases.push("frame-subsampling 1 1 3 1 2 1".into());
    compare(cases);
}
#[test]
fn loop_filter_bundles_match_defaults_custom_weights_and_truncated_state() {
    let mut cases = Vec::new();
    let mut seed = 97253;
    for modular in 0..2 {
        for variant in 0..4 {
            for offset in 0..8 {
                cases.push(format!("frame-filter {modular} {variant} {offset}"));
            }
        }
        for len in 0..48 {
            for trial in 0..8 {
                let bytes: Vec<_> = (0..len).map(|_| next(&mut seed) as u8).collect();
                cases.push(format!(
                    "frame-filter-read {modular} {trial} {}",
                    hex(&bytes)
                ));
            }
        }
    }
    compare(cases);
}
#[test]
fn frame_header_bytes_and_state_match_across_frame_types_and_metadata() {
    let mut cases = Vec::new();
    for xyb in 0..2 {
        for animation in 0..2 {
            for extras in [0, 2] {
                for modular in 0..2 {
                    for kind in 0..4 {
                        for custom in 0..2 {
                            for up in [1, 4] {
                                for filter in 0..3 {
                                    let flags = if kind == 1 { 32 } else { 129 };
                                    let blend = if kind == 0 { 2 } else { 4 };
                                    cases.push(format!("frame-write {xyb} {animation} {extras} 0 {modular} {kind} {flags} 2 2 {up} {blend} {filter} 0 {custom} {}",(kind+filter)%8));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    for kind in 0..4 {
        for custom in 0..2 {
            for blend in 0..5 {
                cases.push(format!(
                    "frame-write 0 1 2 1 1 {kind} 0 1 1 1 {blend} 1 1 {custom} 7"
                ));
            }
        }
    }
    compare(cases);
}
#[test]
fn arbitrary_frame_headers_preserve_status_consumption_and_partial_fields() {
    let mut cases = Vec::new();
    let mut seed = 126859;
    for len in 0..64 {
        for trial in 0..16 {
            let bytes: Vec<_> = (0..len).map(|_| next(&mut seed) as u8).collect();
            cases.push(format!(
                "frame-read {} {} {} {} {} {}",
                trial % 2,
                (trial >> 1) % 2,
                if trial & 4 != 0 { 2 } else { 0 },
                (trial >> 3) % 2,
                trial % 8,
                hex(&bytes)
            ));
        }
    }
    // Every byte truncation of translated real headers is compared against an
    // independent original parser, including names and custom filters.
    let mut words = "0 1 2".split_whitespace();
    let metadata = configure_metadata(&mut words);
    for kind in [
        FrameType::REGULAR,
        FrameType::DC,
        FrameType::REFERENCE_ONLY,
        FrameType::SKIP_PROGRESSIVE,
    ] {
        let mut f = FrameHeader::new(Some(&metadata));
        f.frame_type = kind;
        f.dc_level = u32::from(kind == FrameType::DC);
        f.encoding = FrameEncoding::MODULAR;
        f.extra_channel_upsampling = vec![1, 2];
        f.color_transform = ColorTransform::NONE;
        f.name = b"header".to_vec();
        configure_filter(&mut f.loop_filter, 2);
        let mut writer = BitWriter::new();
        write_frame_header(&mut f, &mut writer, None).unwrap();
        pad(&mut writer);
        for len in 0..=writer.get_span().len() {
            cases.push(format!(
                "frame-read 0 1 2 0 0 {}",
                hex(&writer.get_span()[..len])
            ));
        }
    }
    compare(cases);
}

#[test]
fn progressive_passes_match_sequences_resolution_brackets_and_rejections() {
    let mut cases = Vec::new();
    let mut seed = 362743;
    for count in 1..=11 {
        for levels in 0..=4 {
            for variant in 0..8 {
                let mut p = Passes {
                    num_passes: count,
                    num_downsample: levels,
                    ..Passes::default()
                };
                p.downsample[..4].copy_from_slice(&[8, 4, 2, 1]);
                p.last_pass[..4].copy_from_slice(&[0, 1, 2, 3]);
                for (i, s) in p.shift.iter_mut().enumerate() {
                    *s = ((i + variant) % 4) as u32;
                }
                if variant == 4 {
                    p.downsample[1] = 8;
                }
                if variant == 5 {
                    p.last_pass[1] = 0;
                }
                if variant == 6 {
                    p.last_pass[0] = count;
                }
                if variant == 7 {
                    p.shift[0] = 4;
                }
                let mut request = format!("frame-passes {count} {levels} {variant}");
                for v in p.downsample.iter().chain(&p.last_pass).chain(&p.shift) {
                    write!(request, " {v}").unwrap();
                }
                cases.push(request);
            }
        }
    }
    for len in 0..24 {
        for trial in 0..16 {
            let bytes: Vec<_> = (0..len).map(|_| next(&mut seed) as u8).collect();
            cases.push(format!("frame-passes-read {} {}", trial % 8, hex(&bytes)));
        }
    }
    compare(cases);
}
