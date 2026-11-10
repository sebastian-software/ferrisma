// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Description and numeric text from `lib/jxl/cms/jxl_cms_internal.h` and
//! `base/common.h`. Rust's decimal formatter supplies C's default %g precision
//! (six significant digits); select fixed/exponent form after rounding and
//! retain signed zero and C spelling. No C formatter is linked.

use super::super::color_encoding::{
    ColorSpace, JxlColorEncoding, Primaries, RenderingIntent, TransferFunction, WhitePoint,
};

pub fn source_float_to_string(value: f64) -> String {
    if value.is_nan() {
        // Darwin's %g formatter omits a NaN sign. Other reference targets
        // retain the conventional sign; only Darwin has runtime evidence here.
        return if value.is_sign_negative() && !cfg!(target_vendor = "apple") {
            "-nan"
        } else {
            "nan"
        }
        .to_owned();
    }
    if value.is_infinite() {
        return if value.is_sign_negative() {
            "-inf"
        } else {
            "inf"
        }
        .to_owned();
    }
    let scientific = format!("{value:.5e}");
    let (mantissa, exponent) = scientific.split_once('e').unwrap();
    let exponent: i32 = exponent.parse().unwrap();
    let trim = |s: &str| s.trim_end_matches('0').trim_end_matches('.').to_owned();
    if !(-4..6).contains(&exponent) {
        format!("{}e{exponent:+03}", trim(mantissa))
    } else {
        let precision = (5 - exponent) as usize;
        let fixed = format!("{value:.precision$}");
        if fixed.contains('.') {
            trim(&fixed)
        } else {
            fixed
        }
    }
}
pub fn color_encoding_description(c: &JxlColorEncoding, uniquename: bool) -> String {
    fn close(a: f64, b: f64) -> bool {
        (a - b).abs() < 3e-5
    }
    if c.color_space == ColorSpace::RGB && !uniquename {
        if c.white_point == WhitePoint::D65 {
            if c.transfer_function == TransferFunction::SRGB {
                if c.primaries == Primaries::SRGB {
                    return "sRGB".to_owned();
                }
                if c.primaries == Primaries::P3 {
                    return "DisplayP3".to_owned();
                }
            }
            if c.primaries == Primaries::BT2100 {
                if c.transfer_function == TransferFunction::PQ {
                    return "Rec2100PQ".to_owned();
                }
                if c.transfer_function == TransferFunction::HLG {
                    return "Rec2100HLG".to_owned();
                }
            }
            if c.primaries == Primaries::CUSTOM
                && close(c.primaries_red_xy[0], 0.64)
                && close(c.primaries_red_xy[1], 0.33)
                && close(c.primaries_green_xy[0], 0.21)
                && close(c.primaries_green_xy[1], 0.71)
                && close(c.primaries_blue_xy[0], 0.15)
                && close(c.primaries_blue_xy[1], 0.06)
                && c.transfer_function == TransferFunction::GAMMA
                && close(c.gamma, 256.0 / 563.0)
            {
                return "Adobe98".to_owned();
            }
        }
        if c.white_point == WhitePoint::CUSTOM
            && close(c.white_point_xy[0], 0.345669)
            && close(c.white_point_xy[1], 0.358496)
            && c.primaries == Primaries::CUSTOM
            && close(c.primaries_red_xy[0], 0.734699)
            && close(c.primaries_red_xy[1], 0.265301)
            && close(c.primaries_green_xy[0], 0.159597)
            && close(c.primaries_green_xy[1], 0.840403)
            && close(c.primaries_blue_xy[0], 0.036598)
            && close(c.primaries_blue_xy[1], 0.000105)
            && c.transfer_function == TransferFunction::GAMMA
            && close(c.gamma, 1.0 / 1.8)
        {
            return "ProPhoto".to_owned();
        }
    }
    let mut d = match c.color_space {
        ColorSpace::RGB => "RGB",
        ColorSpace::GRAY => "Gra",
        ColorSpace::XYB => "XYB",
        ColorSpace::UNKNOWN => "CS?",
        _ => "Invalid",
    }
    .to_owned();
    let explicit_wp_tf = c.color_space != ColorSpace::XYB;
    if explicit_wp_tf {
        d.push('_');
        if c.white_point == WhitePoint::CUSTOM {
            d.push_str(&format!(
                "{};{}",
                source_float_to_string(c.white_point_xy[0]),
                source_float_to_string(c.white_point_xy[1])
            ));
        } else {
            d.push_str(match c.white_point {
                WhitePoint::D65 => "D65",
                WhitePoint::E => "EER",
                WhitePoint::DCI => "DCI",
                _ => "Invalid",
            });
        }
    }
    if c.color_space != ColorSpace::GRAY && c.color_space != ColorSpace::XYB {
        d.push('_');
        if c.primaries == Primaries::CUSTOM {
            let values = [
                c.primaries_red_xy[0],
                c.primaries_red_xy[1],
                c.primaries_green_xy[0],
                c.primaries_green_xy[1],
                c.primaries_blue_xy[0],
                c.primaries_blue_xy[1],
            ];
            d.push_str(&values.map(source_float_to_string).join(";"));
        } else {
            d.push_str(match c.primaries {
                Primaries::SRGB => "SRG",
                Primaries::BT2100 => "202",
                Primaries::P3 => "DCI",
                _ => "Invalid",
            });
        }
    }
    d.push('_');
    d.push_str(match c.rendering_intent {
        RenderingIntent::PERCEPTUAL => "Per",
        RenderingIntent::RELATIVE => "Rel",
        RenderingIntent::SATURATION => "Sat",
        RenderingIntent::ABSOLUTE => "Abs",
        _ => "Invalid",
    });
    if explicit_wp_tf {
        d.push('_');
        if c.transfer_function == TransferFunction::GAMMA {
            d.push('g');
            d.push_str(&source_float_to_string(c.gamma));
        } else {
            d.push_str(match c.transfer_function {
                TransferFunction::SRGB => "SRG",
                TransferFunction::LINEAR => "Lin",
                TransferFunction::BT709 => "709",
                TransferFunction::PQ => "PeQ",
                TransferFunction::HLG => "HLG",
                TransferFunction::DCI => "DCI",
                TransferFunction::UNKNOWN => "TF?",
                _ => "Invalid",
            });
        }
    }
    d
}
