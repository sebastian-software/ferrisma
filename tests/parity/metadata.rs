// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Full field/state observations for original and translated metadata bundles.

use super::*;
use jxl::jxl::cms::color_encoding_cms as storage;
use jxl::jxl::color_encoding::*;
use jxl::jxl::color_encoding_internal::ColorEncoding;
use jxl::jxl::enc_fields::{write_codestream_headers, write_image_metadata};
use jxl::jxl::image_metadata::*;

pub(super) fn handles(kind: &str) -> bool {
    kind.starts_with("meta-")
}
fn color_storage(out: &mut String, s: &storage::ColorEncoding) {
    write!(
        out,
        " {},{},{},{},{},{},{},{},{}",
        s.color_space.0,
        s.white_point.0,
        s.primaries.0,
        s.rendering_intent.0,
        u32::from(s.have_fields),
        u32::from(s.cmyk),
        u32::from(s.tf.have_gamma),
        s.tf.gamma,
        s.tf.transfer_function.0
    )
    .unwrap();
    for xy in [&s.white, &s.red, &s.green, &s.blue] {
        write!(out, ",{},{}", xy.x, xy.y).unwrap();
    }
    write!(out, " {}", hex(&s.icc)).unwrap();
}
pub(super) fn color_bundle(out: &mut String, c: &ColorEncoding) {
    write!(
        out,
        " {},{},{},{}",
        u32::from(c.all_default),
        u32::from(c.want_icc()),
        u32::from(c.is_srgb()),
        u32::from(c.is_linear_srgb())
    )
    .unwrap();
    color_storage(out, c.view());
}
fn depth(out: &mut String, b: &BitDepth) {
    write!(
        out,
        " {},{},{}",
        u32::from(b.floating_point_sample),
        b.bits_per_sample,
        b.exponent_bits_per_sample
    )
    .unwrap();
}
fn extra(out: &mut String, e: &ExtraChannelInfo) {
    write!(
        out,
        " {},{},{},{},{}",
        u32::from(e.all_default),
        e.channel_type.0,
        e.dim_shift,
        u32::from(e.alpha_associated),
        e.cfa_channel
    )
    .unwrap();
    depth(out, &e.bit_depth);
    write!(out, " {}", hex(&e.name)).unwrap();
    for value in e.spot_color {
        write!(out, " {}", value.to_bits()).unwrap();
    }
}
fn tone(out: &mut String, t: &ToneMapping) {
    write!(
        out,
        " {},{},{},{},{}",
        u32::from(t.all_default),
        t.intensity_target.to_bits(),
        t.min_nits.to_bits(),
        u32::from(t.relative_to_max_display),
        t.linear_below.to_bits()
    )
    .unwrap();
}
fn transform(out: &mut String, t: &CustomTransformData) {
    write!(
        out,
        " {},{},{},{}",
        u32::from(t.all_default),
        u32::from(t.nonserialized_xyb_encoded),
        t.custom_weights_mask,
        u32::from(t.opsin_inverse_matrix.all_default)
    )
    .unwrap();
    for row in t.opsin_inverse_matrix.inverse_matrix {
        for v in row {
            write!(out, " {}", v.to_bits()).unwrap();
        }
    }
    for v in t.opsin_inverse_matrix.opsin_biases {
        write!(out, " {}", v.to_bits()).unwrap();
    }
    for v in t.opsin_inverse_matrix.quant_biases {
        write!(out, " {}", v.to_bits()).unwrap();
    }
    for &v in t
        .upsampling2_weights
        .iter()
        .chain(&t.upsampling4_weights)
        .chain(&t.upsampling8_weights)
    {
        write!(out, " {}", v.to_bits()).unwrap();
    }
}
fn metadata(out: &mut String, m: &ImageMetadata) {
    write!(
        out,
        " {},{},{},{},{},{},{},{},{},{}",
        u32::from(m.all_default),
        u32::from(m.modular_16_bit_buffer_sufficient),
        u32::from(m.xyb_encoded),
        m.orientation,
        u32::from(m.have_preview),
        u32::from(m.have_animation),
        u32::from(m.have_intrinsic_size),
        m.num_extra_channels,
        m.extensions,
        u32::from(m.nonserialized_only_parse_basic_info)
    )
    .unwrap();
    depth(out, &m.bit_depth);
    tone(out, &m.tone_mapping);
    color_bundle(out, &m.color_encoding);
    write!(
        out,
        " {},{},{},{},{},{},{},{}",
        m.intrinsic_size.xsize(),
        m.intrinsic_size.ysize(),
        m.preview_size.xsize(),
        m.preview_size.ysize(),
        m.animation.tps_numerator,
        m.animation.tps_denominator,
        m.animation.num_loops,
        u32::from(m.animation.have_timecodes)
    )
    .unwrap();
    write!(out, " {}", m.extra_channel_info.len()).unwrap();
    for e in &m.extra_channel_info {
        extra(out, e);
    }
}
fn configure(args: &mut std::str::SplitWhitespace<'_>) -> CodecMetadata {
    let mut codec = CodecMetadata::default();
    let m = &mut codec.m;
    m.bit_depth.floating_point_sample = number::<u32>(args) != 0;
    m.bit_depth.bits_per_sample = number(args);
    m.bit_depth.exponent_bits_per_sample = number(args);
    m.orientation = number(args);
    let flags: u32 = number(args);
    let extras: usize = number(args);
    m.xyb_encoded = number::<u32>(args) != 0;
    m.have_preview = flags & 1 != 0;
    m.have_animation = flags & 2 != 0;
    m.have_intrinsic_size = flags & 4 != 0;
    m.color_encoding
        .from_external(&cms::color_encoding(args))
        .unwrap();
    m.tone_mapping.intensity_target = f32::from_bits(number(args));
    m.tone_mapping.min_nits = f32::from_bits(number(args));
    m.tone_mapping.relative_to_max_display = number::<u32>(args) != 0;
    m.tone_mapping.linear_below = f32::from_bits(number(args));
    codec.size.set(257, 129).unwrap();
    m.preview_size.set(32, 16).unwrap();
    m.intrinsic_size.set(300, 150).unwrap();
    m.animation.tps_numerator = 1001;
    m.animation.tps_denominator = 30;
    m.animation.num_loops = 2;
    m.animation.have_timecodes = true;
    for _ in 0..extras {
        let mut e = ExtraChannelInfo {
            channel_type: ExtraChannel(number(args)),
            ..ExtraChannelInfo::default()
        };
        e.bit_depth.bits_per_sample = number(args);
        e.dim_shift = number(args);
        e.alpha_associated = number::<u32>(args) != 0;
        e.name = unhex(args.next().unwrap());
        e.spot_color = [0.125, 0.5, 1.0, 0.75];
        m.extra_channel_info.push(e);
    }
    codec
}
pub(super) fn observe(kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    if kind == "meta-store" {
        let have: u32 = number(args);
        let external = cms::color_encoding(args);
        let mut c = storage::ColorEncoding {
            have_fields: have != 0,
            icc: vec![9, 8, 7],
            ..storage::ColorEncoding::default()
        };
        let status = c.from_external(&external);
        let mut out = status_code(&status).to_string();
        color_storage(&mut out, &c);
        return out;
    }
    if kind == "meta-color-state" {
        let external = cms::color_encoding(args);
        let mut c = ColorEncoding::default();
        let status = c.from_external(&external);
        let mut out = status_code(&status).to_string();
        color_bundle(&mut out, &c);
        return out;
    }
    if kind == "meta-color" {
        let raw: u32 = number(args);
        let offset = number(args);
        let external = cms::color_encoding(args);
        let mut c = ColorEncoding::default();
        let converted = c.from_external(&external);
        let mut out = status_code(&converted).to_string();
        color_bundle(&mut out, &c);
        if raw != 0 {
            c.set_icc_raw(vec![255, 0, 17]);
        }
        let mut writer = BitWriter::new();
        prefix(&mut writer, offset);
        let can = bundle::can_encode(&mut c);
        let (ext, bits) = can.unwrap_or((0, 0));
        write!(out, " {} {ext} {bits}", status_code(&can.map(|_| ()))).unwrap();
        if can.is_ok() {
            let written = bundle::write(&mut c, &mut writer, LayerType::Header, None);
            write!(out, " {} {}", status_code(&written), writer.bits_written()).unwrap();
            pad(&mut writer);
            write!(out, " {}", hex(writer.get_span())).unwrap();
            let mut decoded = ColorEncoding::default();
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
            color_bundle(&mut out, &decoded);
        }
        color_bundle(&mut out, &c);
        return out;
    }
    if kind == "meta-transform" {
        let xyb: u32 = number(args);
        let mask = number(args);
        let alter: u32 = number(args);
        let offset = number(args);
        let mut value = CustomTransformData {
            nonserialized_xyb_encoded: xyb != 0,
            custom_weights_mask: mask,
            ..CustomTransformData::default()
        };
        if alter != 0 {
            value.opsin_inverse_matrix.inverse_matrix[0][0] = 2.25;
            value.opsin_inverse_matrix.opsin_biases[0] = 0.03125;
            value.opsin_inverse_matrix.quant_biases[3] = 0.25;
            value.upsampling2_weights[0] += 1.0;
            value.upsampling8_weights[209] -= 0.125;
        }
        let mut writer = BitWriter::new();
        prefix(&mut writer, offset);
        let can = bundle::can_encode(&mut value);
        let (ext, bits) = can.unwrap_or((0, 0));
        let mut out = format!("{} {ext} {bits}", status_code(&can.map(|_| ())));
        if can.is_ok() {
            let written = bundle::write(&mut value, &mut writer, LayerType::Header, None);
            write!(out, " {} {}", status_code(&written), writer.bits_written()).unwrap();
            pad(&mut writer);
            write!(out, " {}", hex(writer.get_span())).unwrap();
            let mut decoded = CustomTransformData {
                nonserialized_xyb_encoded: xyb != 0,
                ..CustomTransformData::default()
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
            transform(&mut out, &decoded);
        }
        transform(&mut out, &value);
        return out;
    }
    if kind == "meta-depth" {
        let mut value = BitDepth {
            floating_point_sample: number::<u32>(args) != 0,
            bits_per_sample: number(args),
            exponent_bits_per_sample: number(args),
        };
        let offset = number(args);
        let mut writer = BitWriter::new();
        prefix(&mut writer, offset);
        let can = bundle::can_encode(&mut value);
        let (ext, bits) = can.unwrap_or((0, 0));
        let mut out = format!("{} {ext} {bits}", status_code(&can.map(|_| ())));
        depth(&mut out, &value);
        if can.is_ok() {
            let written = bundle::write(&mut value, &mut writer, LayerType::Header, None);
            write!(out, " {} {}", status_code(&written), writer.bits_written()).unwrap();
            pad(&mut writer);
            write!(out, " {}", hex(writer.get_span())).unwrap();
            let mut decoded = BitDepth::default();
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
            depth(&mut out, &decoded);
        }
        return out;
    }
    if kind == "meta-write" {
        let full: u32 = number(args);
        let offset = number(args);
        let mut codec = configure(args);
        let mut writer = BitWriter::new();
        prefix(&mut writer, offset);
        let mut aux = AuxOut::default();
        let can = bundle::can_encode(&mut codec.m);
        let (ext, bits) = can.unwrap_or((0, 0));
        let mut out = format!("{} {ext} {bits}", status_code(&can.map(|_| ())));
        if can.is_ok() {
            let written = if full != 0 {
                write_codestream_headers(&mut codec, &mut writer, Some(&mut aux))
            } else {
                write_image_metadata(&mut codec.m, &mut writer, LayerType::Header, Some(&mut aux))
            };
            write!(out, " {} {}", status_code(&written), writer.bits_written()).unwrap();
            pad(&mut writer);
            write!(out, " {}", hex(writer.get_span())).unwrap();
            let a = aux.layer(LayerType::Header);
            write!(out, " {},{}", a.total_bits, a.histogram_bits).unwrap();
        }
        metadata(&mut out, &codec.m);
        if full != 0 {
            transform(&mut out, &codec.transform_data);
        }
        return out;
    }
    assert_eq!(kind, "meta-read");
    let ty = args.next().unwrap();
    let offset = number(args);
    let basic: u32 = number(args);
    let bytes = unhex(args.next().unwrap());
    let mut br = BitReader::new(&bytes);
    br.skip_bits(offset);
    if ty == "color" {
        let mut c = ColorEncoding::default();
        let read = bundle::read(&mut br, &mut c);
        let mut out = format!(
            "{} {} {}",
            status_code(&read),
            br.total_bits_consumed(),
            status_code(&br.close())
        );
        color_bundle(&mut out, &c);
        out
    } else if ty == "transform" {
        let mut t = CustomTransformData {
            nonserialized_xyb_encoded: basic != 0,
            ..CustomTransformData::default()
        };
        let read = bundle::read(&mut br, &mut t);
        let mut out = format!(
            "{} {} {}",
            status_code(&read),
            br.total_bits_consumed(),
            status_code(&br.close())
        );
        transform(&mut out, &t);
        out
    } else if ty == "depth" {
        let mut d = BitDepth::default();
        let read = bundle::read(&mut br, &mut d);
        let mut out = format!(
            "{} {} {}",
            status_code(&read),
            br.total_bits_consumed(),
            status_code(&br.close())
        );
        depth(&mut out, &d);
        out
    } else {
        let mut m = ImageMetadata {
            nonserialized_only_parse_basic_info: basic != 0,
            ..ImageMetadata::default()
        };
        if ty == "reused" {
            let info = ExtraChannelInfo {
                channel_type: ExtraChannel::DEPTH,
                name: b"old".to_vec(),
                ..ExtraChannelInfo::default()
            };
            m.extra_channel_info.push(info.clone());
            m.extra_channel_info.push(info);
            m.orientation = 8;
            m.have_animation = true;
        }
        let read = read_image_metadata(&mut br, &mut m);
        let mut out = format!(
            "{} {} {}",
            status_code(&read),
            br.total_bits_consumed(),
            status_code(&br.close())
        );
        metadata(&mut out, &m);
        out
    }
}

#[test]
fn metadata_bit_depth_ranges_offsets_and_error_state_match_original() {
    let mut cases = Vec::new();
    for fp in [0, 1] {
        for bits in 0..=65 {
            for exponent in 0..=17 {
                cases.push(format!("meta-depth {fp} {bits} {exponent} {}", bits % 8));
            }
        }
    }
    compare(cases);
}
#[test]
fn color_bundles_configs_profiles_and_raw_icc_match_original() {
    let mut cases = Vec::new();
    for cs in [ColorSpace::RGB, ColorSpace::GRAY, ColorSpace::XYB] {
        for tf in [
            TransferFunction::SRGB,
            TransferFunction::LINEAR,
            TransferFunction::PQ,
            TransferFunction::HLG,
            TransferFunction::GAMMA,
        ] {
            for raw in [0, 1] {
                for offset in [0, 1, 7] {
                    let mut c = cms::srgb();
                    c.color_space = cs;
                    c.transfer_function = tf;
                    c.gamma = 0.123;
                    c.rendering_intent = if cs == ColorSpace::XYB {
                        RenderingIntent::PERCEPTUAL
                    } else {
                        RenderingIntent::RELATIVE
                    };
                    cases.push(format!(
                        "meta-color {raw} {offset} {}",
                        cms::external_args(&c)
                    ));
                }
            }
        }
    }
    compare(cases);
}

#[test]
fn color_storage_conversion_rounding_and_partial_errors_match_original() {
    let mut cases = Vec::new();
    let mut seed = 211;
    for trial in 0..256 {
        let mut c = cms::srgb();
        c.white_point = WhitePoint::CUSTOM;
        c.primaries = Primaries::CUSTOM;
        c.transfer_function = TransferFunction::GAMMA;
        c.color_space = ColorSpace((trial % 4) as u32);
        c.rendering_intent = RenderingIntent((trial % 4) as u32);
        c.gamma = 0.25 + (next(&mut seed) % 500001) as f64 / 1000000.0;
        for xy in [
            &mut c.white_point_xy,
            &mut c.primaries_red_xy,
            &mut c.primaries_green_xy,
            &mut c.primaries_blue_xy,
        ] {
            for value in xy {
                *value = 0.25 + (next(&mut seed) % 500001) as f64 / 1000000.0;
            }
        }
        for have in [0, 1] {
            cases.push(format!("meta-store {have} {}", cms::external_args(&c)));
        }
        cases.push(format!("meta-color-state {}", cms::external_args(&c)));
    }
    for which in 0..5 {
        // External C enums without a fixed underlying type have a bounded
        // defined range in C++. Exercise gaps inside it, not enum-load UB.
        let values: &[u32] = match which {
            0 | 4 => &[0, 1, 2, 3],
            1 | 2 => &[0, 3, 4, 7, 15],
            _ => &[0, 3, 4, 7, 63, 65535],
        };
        for &value in values {
            let mut c = cms::srgb();
            match which {
                0 => c.color_space = ColorSpace(value),
                1 => c.white_point = WhitePoint(value),
                2 => c.primaries = Primaries(value),
                3 => c.transfer_function = TransferFunction(value),
                _ => c.rendering_intent = RenderingIntent(value),
            }
            cases.push(format!("meta-store 1 {}", cms::external_args(&c)));
        }
    }
    for xy in [
        [-4.0, 0.1],
        [-2.097152, 0.1],
        [2.097151, 0.1],
        [2.097152, 0.1],
        [0.1, 3.0],
        [0.1, f64::INFINITY],
        [0.0, 0.3],
    ] {
        let mut c = cms::srgb();
        c.white_point = WhitePoint::CUSTOM;
        c.white_point_xy = xy;
        cases.push(format!("meta-store 1 {}", cms::external_args(&c)));
    }
    for gamma in [
        0.0,
        1.0 / 8192.0,
        1.0 / 8192.0 - 1e-9,
        0.123,
        1.0 / 2.6 - 0.001,
        1.0 / 2.6 + 0.001,
        0.999,
        1.0,
        1.001,
    ] {
        let mut c = cms::srgb();
        c.transfer_function = TransferFunction::GAMMA;
        c.gamma = gamma;
        cases.push(format!("meta-store 1 {}", cms::external_args(&c)));
    }
    compare(cases);
}
fn metadata_request(
    full: u32,
    offset: usize,
    orientation: u32,
    flags: u32,
    variant: usize,
) -> String {
    let mut c = cms::srgb();
    if variant % 2 != 0 {
        c.color_space = ColorSpace::GRAY;
    }
    if variant % 3 == 1 {
        c.transfer_function = TransferFunction::LINEAR;
    }
    let extras = if variant % 3 == 0 { 0 } else { 2 };
    let mut line = format!(
        "meta-write {full} {offset} 0 {} 0 {orientation} {flags} {extras} {} {} {} {} 0 {}",
        [8, 10, 16][variant % 3],
        variant % 2,
        cms::external_args(&c),
        if variant % 3 == 2 {
            1000.0f32.to_bits()
        } else {
            255.0f32.to_bits()
        },
        0.0f32.to_bits(),
        0.0f32.to_bits()
    );
    if extras != 0 {
        write!(line, " 0 8 0 1 616c706861 5 12 1 0 ff00626179").unwrap();
    }
    line
}
#[test]
fn image_metadata_and_complete_codestream_headers_match_original() {
    let mut cases = Vec::new();
    for full in [0, 1] {
        for offset in [0, 1, 7] {
            for orientation in 1..=8 {
                for flags in [0, 1, 2, 4, 7] {
                    for variant in 0..3 {
                        cases.push(metadata_request(full, offset, orientation, flags, variant));
                    }
                }
            }
        }
    }
    compare(cases);
}
#[test]
fn truncated_and_damaged_metadata_preserves_reused_state() {
    let mut cases = Vec::new();
    let mut seed = 193;
    for ty in ["depth", "color", "image", "reused", "transform"] {
        for size in 0..32 {
            for trial in 0..4 {
                let bytes: Vec<_> = (0..size)
                    .map(|_| {
                        if trial == 0 {
                            0
                        } else if trial == 1 {
                            255
                        } else {
                            next(&mut seed) as u8
                        }
                    })
                    .collect();
                cases.push(format!(
                    "meta-read {ty} {} {} {}",
                    trial % 2,
                    trial % 2,
                    hex(&bytes)
                ));
            }
        }
    }
    compare(cases);
}

#[test]
fn custom_transform_fields_weights_and_skipped_branches_match_original() {
    let mut cases = Vec::new();
    for xyb in [0, 1] {
        for mask in 0..=8 {
            for alter in [0, 1] {
                for offset in [0, 1, 7] {
                    cases.push(format!("meta-transform {xyb} {mask} {alter} {offset}"));
                }
            }
        }
    }
    compare(cases);
}
