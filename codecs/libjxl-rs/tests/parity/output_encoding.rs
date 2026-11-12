// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use super::*;
use jxl::jxl::color_encoding::*;
use jxl::jxl::color_encoding_internal::ColorEncoding;
use jxl::jxl::dec_xyb::{OpsinParams, OutputEncodingInfo, can_output_to_color_encoding};
use jxl::jxl::image_metadata::{CodecMetadata, OpsinInverseMatrix};
pub(super) fn handles(kind: &str) -> bool {
    kind.starts_with("oei-")
}
fn opsin(out: &mut String, p: &OpsinParams) {
    for values in [
        &p.inverse_opsin_matrix[..],
        &p.opsin_biases[..],
        &p.opsin_biases_cbrt[..],
        &p.quant_biases[..],
    ] {
        for v in values {
            write!(out, " {}", v.to_bits()).unwrap();
        }
    }
}
pub(super) fn observation(out: &mut String, s: &OutputEncodingInfo) {
    metadata::color_bundle(out, &s.orig_color_encoding);
    write!(out, " {}", s.orig_intensity_target.to_bits()).unwrap();
    for row in s.orig_inverse_matrix {
        for v in row {
            write!(out, " {}", v.to_bits()).unwrap();
        }
    }
    write!(
        out,
        " {} {}",
        u32::from(s.default_transform),
        u32::from(s.xyb_encoded)
    )
    .unwrap();
    metadata::color_bundle(out, &s.color_encoding);
    metadata::color_bundle(out, &s.linear_color_encoding);
    write!(out, " {}", u32::from(s.color_encoding_is_original)).unwrap();
    opsin(out, &s.opsin_params);
    write!(
        out,
        " {} {}",
        u32::from(s.all_default_opsin),
        s.inverse_gamma.to_bits()
    )
    .unwrap();
    for v in s.luminances {
        write!(out, " {}", v.to_bits()).unwrap();
    }
    write!(
        out,
        " {} {}",
        s.desired_intensity_target.to_bits(),
        u32::from(s.cms_set)
    )
    .unwrap();
}
#[allow(
    clippy::needless_range_loop,
    reason = "Keep the index loops; stable Clippy versions differ."
)]
fn custom(m: &mut OpsinInverseMatrix, mode: u32) {
    if mode == 0 {
        return;
    }
    m.all_default = false;
    for y in 0..3 {
        for x in 0..3 {
            m.inverse_matrix[y][x] = if mode == 2 {
                0.0
            } else {
                ((y * 3 + x) as i32 - 4) as f32 / 8.0
            };
        }
    }
    m.opsin_biases = [
        -0.125,
        -0.0,
        if mode == 3 {
            f32::from_bits(0x7fa12345)
        } else {
            1.0
        },
    ];
    for i in 0..4 {
        m.quant_biases[i] = i as f32 / 8.0;
    }
}
pub(super) fn observe(kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    if kind == "oei-sequence" {
        let n = number::<usize>(args);
        let mut info = OutputEncodingInfo::default();
        let mut out = String::new();
        for _ in 0..n {
            let xyb = number::<u32>(args) != 0;
            let target = f32::from_bits(number(args));
            let mode = number(args);
            let raw = number::<u32>(args) != 0;
            let mut metadata = CodecMetadata::default();
            let converted = metadata
                .m
                .color_encoding
                .from_external(&cms::color_encoding(args));
            write!(out, " E {}", status_code(&converted)).unwrap();
            if raw {
                metadata.m.color_encoding.set_icc_raw(vec![9, 8, 7]);
            }
            metadata.m.xyb_encoded = xyb;
            metadata.m.tone_mapping.intensity_target = target;
            custom(&mut metadata.transform_data.opsin_inverse_matrix, mode);
            let status = info.set_from_metadata(&metadata);
            write!(out, " M {}", status_code(&status)).unwrap();
            observation(&mut out, &info);
        }
        return out;
    }
    if kind == "oei-opsin" {
        let target = f32::from_bits(number(args));
        let mut p = OpsinParams::default();
        p.init(target);
        let mut out = String::new();
        opsin(&mut out, &p);
        return out;
    }
    assert_eq!(kind, "oei-meta");
    let xyb = number::<u32>(args) != 0;
    let target = f32::from_bits(number(args));
    let mode = number(args);
    let raw = number::<u32>(args) != 0;
    let repeats = number::<usize>(args);
    let n = number::<usize>(args);
    let mut metadata = CodecMetadata::default();
    let converted = metadata
        .m
        .color_encoding
        .from_external(&cms::color_encoding(args));
    let mut out = format!(" E {}", status_code(&converted));
    if converted.is_err() {
        return out;
    }
    if raw {
        metadata.m.color_encoding.set_icc_raw(vec![9, 8, 7]);
    }
    metadata.m.xyb_encoded = xyb;
    metadata.m.tone_mapping.intensity_target = target;
    custom(&mut metadata.transform_data.opsin_inverse_matrix, mode);
    let mut info = OutputEncodingInfo::default();
    for _ in 0..repeats {
        let status = info.set_from_metadata(&metadata);
        write!(out, " M {}", status_code(&status)).unwrap();
        observation(&mut out, &info);
    }
    for _ in 0..n {
        let raw = number::<u32>(args) != 0;
        let mut desired = ColorEncoding::default();
        let converted = desired.from_external(&cms::color_encoding(args));
        write!(out, " E {}", status_code(&converted)).unwrap();
        if raw {
            desired.set_icc_raw(vec![9, 8, 7]);
        }
        write!(
            out,
            " C {}",
            u32::from(can_output_to_color_encoding(&desired))
        )
        .unwrap();
        let status = info.maybe_set_color_encoding(&desired);
        write!(out, " S {}", status_code(&status)).unwrap();
        observation(&mut out, &info);
    }
    out
}
fn colors() -> Vec<JxlColorEncoding> {
    let mut result = Vec::new();
    for cs in [
        ColorSpace::RGB,
        ColorSpace::GRAY,
        ColorSpace::XYB,
        ColorSpace::UNKNOWN,
    ] {
        for (p, wp, tf) in [
            (Primaries::SRGB, WhitePoint::D65, TransferFunction::SRGB),
            (Primaries::P3, WhitePoint::DCI, TransferFunction::DCI),
            (Primaries::BT2100, WhitePoint::D65, TransferFunction::PQ),
            (Primaries::BT2100, WhitePoint::E, TransferFunction::HLG),
            (
                Primaries::CUSTOM,
                WhitePoint::CUSTOM,
                TransferFunction::LINEAR,
            ),
            (Primaries::SRGB, WhitePoint::D65, TransferFunction::BT709),
            (Primaries::SRGB, WhitePoint::D65, TransferFunction::GAMMA),
            (Primaries::SRGB, WhitePoint::D65, TransferFunction::UNKNOWN),
        ] {
            let mut c = cms::srgb();
            c.color_space = cs;
            c.primaries = p;
            c.white_point = wp;
            c.transfer_function = tf;
            c.gamma = 0.45;
            result.push(c);
        }
    }
    let mut bad = cms::srgb();
    bad.primaries = Primaries::CUSTOM;
    bad.primaries_red_xy = [0.0, 0.0];
    bad.primaries_green_xy = [0.0, 0.0];
    bad.primaries_blue_xy = [0.0, 0.0];
    result.push(bad);
    let mut bad = cms::srgb();
    bad.white_point = WhitePoint::CUSTOM;
    bad.white_point_xy = [1.0, 0.0];
    result.push(bad);
    result
}
#[test]
fn output_encoding_metadata_selection_reuse_and_desired_profile_partial_errors_match_original() {
    let colors = colors();
    let mut desired = String::new();
    for c in &colors {
        write!(desired, " 0 {}", cms::external_args(c)).unwrap();
    }
    write!(desired, " 1 {}", cms::external_args(&cms::srgb())).unwrap();
    let mut cases = Vec::new();
    for (i, color) in colors.iter().enumerate() {
        for xyb in [0, 1] {
            for custom in [0, 1, 2, 3] {
                cases.push(format!(
                    "oei-meta {xyb} {} {custom} {} 2 {} {}{desired}",
                    255.0f32.to_bits(),
                    u32::from(i % 7 == 0),
                    colors.len() + 1,
                    cms::external_args(color)
                ));
            }
        }
    }
    compare(cases);
}
#[test]
fn opsin_lane_expansion_intensity_thresholds_and_exceptional_float_bits_match_original() {
    let mut cases = Vec::new();
    for bits in [
        0u32, 0x80000000, 1, 0x3f800000, 0x437f0000, 0x447a0000, 0x7f800000, 0xff800000,
        0x7fc12345, 0x7fa12345, 0xffa12345,
    ] {
        cases.push(format!("oei-opsin {bits}"));
    }
    for target in [1.0f32, 1000.0, 255.0, 254.9, 255.1, f32::INFINITY, f32::NAN] {
        for bits in [
            target.to_bits().wrapping_sub(1),
            target.to_bits(),
            target.to_bits().wrapping_add(1),
        ] {
            for gray in [false, true] {
                let mut color = cms::srgb();
                if gray {
                    color.color_space = ColorSpace::GRAY;
                }
                cases.push(format!(
                    "oei-meta 1 {bits} 0 0 1 0 {}",
                    cms::external_args(&color)
                ));
            }
        }
    }
    compare(cases);
}

#[test]
fn output_encoding_reused_metadata_retains_untouched_opsin_and_partial_failures() {
    let colors = colors();
    let mut cases = Vec::new();
    for rotation in 0..colors.len() {
        let mut line = format!("oei-sequence {}", colors.len() * 2);
        for i in 0..colors.len() * 2 {
            let c = &colors[(i + rotation) % colors.len()];
            write!(
                line,
                " {} {} {} {} {}",
                (i + rotation) % 2,
                if i % 3 == 0 {
                    255.0f32.to_bits()
                } else {
                    1000.0f32.to_bits()
                },
                i % 4,
                u32::from(i % 9 == 0),
                cms::external_args(c)
            )
            .unwrap();
        }
        cases.push(line);
    }
    compare(cases);
}
