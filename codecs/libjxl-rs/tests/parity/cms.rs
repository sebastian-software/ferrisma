// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Compare numeric intermediates and complete source-generated ICC profiles.

use super::*;
use jxl::jxl::base::matrix_ops::{MatrixScalar, inv3x3_matrix, mul3x3_matrix, mul3x3_vector};
use jxl::jxl::cms::icc::*;
use jxl::jxl::cms::tone_mapping::{HlgOotfBase, Rec2408ToneMapperBase, gamut_map_scalar};
use jxl::jxl::cms::transfer_functions::{TfHlgBase, TfPqBase};
use jxl::jxl::color_encoding::*;

pub(super) fn handles(kind: &str) -> bool {
    kind.starts_with("cms-")
}
fn f64_arg(args: &mut std::str::SplitWhitespace<'_>) -> f64 {
    f64::from_bits(number(args))
}
fn f32_arg(args: &mut std::str::SplitWhitespace<'_>) -> f32 {
    f32::from_bits(number(args))
}
pub(super) fn color_encoding(args: &mut std::str::SplitWhitespace<'_>) -> JxlColorEncoding {
    JxlColorEncoding {
        color_space: ColorSpace(number(args)),
        white_point: WhitePoint(number(args)),
        primaries: Primaries(number(args)),
        transfer_function: TransferFunction(number(args)),
        rendering_intent: RenderingIntent(number(args)),
        gamma: f64_arg(args),
        white_point_xy: [f64_arg(args), f64_arg(args)],
        primaries_red_xy: [f64_arg(args), f64_arg(args)],
        primaries_green_xy: [f64_arg(args), f64_arg(args)],
        primaries_blue_xy: [f64_arg(args), f64_arg(args)],
    }
}
fn matrix_observation<T: MatrixScalar>(args: &mut std::str::SplitWhitespace<'_>) -> String {
    let mut a = [[T::default(); 3]; 3];
    let mut b = a;
    let mut c = a;
    for m in [&mut a, &mut b] {
        for row in m {
            for value in row {
                *value = T::from_f64(f64_arg(args));
            }
        }
    }
    let mut v = [T::default(); 3];
    for value in &mut v {
        *value = T::from_f64(f64_arg(args));
    }
    let mut w = v;
    mul3x3_matrix(&a, &b, &mut c);
    mul3x3_vector(&a, &v, &mut w);
    let mut out = String::new();
    for row in c {
        for value in row {
            write!(out, "{} ", value.to_f64().to_bits()).unwrap();
        }
    }
    for value in w {
        write!(out, "{} ", value.to_f64().to_bits()).unwrap();
    }
    let status = inv3x3_matrix(&mut a);
    write!(out, "{}", status_code(&status)).unwrap();
    for row in a {
        for value in row {
            write!(out, " {}", value.to_f64().to_bits()).unwrap();
        }
    }
    out
}
pub(super) fn observe(kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    match kind {
        "cms-format" => source_float_to_string(f64_arg(args)),
        "cms-md5" => hex(&icc_compute_md5(&unhex(args.next().unwrap()))),
        "cms-matrix" => {
            if number::<u32>(args) == 32 {
                matrix_observation::<f32>(args)
            } else {
                matrix_observation::<f64>(args)
            }
        }
        "cms-transfer" => {
            let which: u32 = number(args);
            let target = f32_arg(args);
            let x = f64_arg(args);
            let value = match which {
                0 => TfHlgBase::display_from_encoded(x),
                1 => TfHlgBase::encoded_from_display(x),
                2 => TfPqBase::display_from_encoded(target, x),
                _ => TfPqBase::encoded_from_display(target, x),
            };
            value.to_bits().to_string()
        }
        "cms-tone" => {
            let which: u32 = number(args);
            let source = [f32_arg(args), f32_arg(args)];
            let target = [f32_arg(args), f32_arg(args)];
            let lum = [f32_arg(args), f32_arg(args), f32_arg(args)];
            let mut rgb = [f32_arg(args), f32_arg(args), f32_arg(args)];
            let saturation = f32_arg(args);
            match which {
                0 => Rec2408ToneMapperBase::new(source, target, lum).tone_map(&mut rgb),
                1 => HlgOotfBase::new(source[1], target[1], lum).apply(&mut rgb),
                _ => gamut_map_scalar(&mut rgb, lum, saturation),
            }
            format!(
                "{} {} {}",
                rgb[0].to_bits(),
                rgb[1].to_bits(),
                rgb[2].to_bits()
            )
        }
        "cms-profile" => {
            let c = color_encoding(args);
            let mut icc = vec![21, 37, 42];
            let status = maybe_create_profile(&c, &mut icc);
            format!(
                "{} {} {} {}",
                status_code(&status),
                color_encoding_description(&c, true),
                color_encoding_description(&c, false),
                hex(&icc)
            )
        }
        "cms-color-matrix" => {
            let c = color_encoding(args);
            let which: u32 = number(args);
            let mut m = [[0.0; 3]; 3];
            let function = if which == 0 {
                primaries_to_xyz
            } else {
                primaries_to_xyz_d50
            };
            let status = function(
                c.primaries_red_xy[0] as f32,
                c.primaries_red_xy[1] as f32,
                c.primaries_green_xy[0] as f32,
                c.primaries_green_xy[1] as f32,
                c.primaries_blue_xy[0] as f32,
                c.primaries_blue_xy[1] as f32,
                c.white_point_xy[0] as f32,
                c.white_point_xy[1] as f32,
                &mut m,
            );
            let mut out = status_code(&status).to_string();
            for row in m {
                for value in row {
                    write!(out, " {}", value.to_bits()).unwrap();
                }
            }
            out
        }
        "cms-curve" => {
            let pq: u32 = number(args);
            let tone: u32 = number(args);
            let table = create_table_curve::<64>(pq != 0, tone != 0);
            let mut out = table.len().to_string();
            for value in table {
                write!(out, " {value}").unwrap();
            }
            out
        }
        _ => panic!("test CMS command"),
    }
}
pub(super) fn external_args(c: &JxlColorEncoding) -> String {
    let mut out = format!(
        "{} {} {} {} {} {}",
        c.color_space.0,
        c.white_point.0,
        c.primaries.0,
        c.transfer_function.0,
        c.rendering_intent.0,
        c.gamma.to_bits()
    );
    for xy in [
        c.white_point_xy,
        c.primaries_red_xy,
        c.primaries_green_xy,
        c.primaries_blue_xy,
    ] {
        for value in xy {
            write!(out, " {}", value.to_bits()).unwrap();
        }
    }
    out
}
pub(super) fn srgb() -> JxlColorEncoding {
    JxlColorEncoding {
        color_space: ColorSpace::RGB,
        white_point: WhitePoint::D65,
        white_point_xy: [0.3127, 0.3290],
        primaries: Primaries::SRGB,
        primaries_red_xy: [0.639998686, 0.330010138],
        primaries_green_xy: [0.300003784, 0.600003357],
        primaries_blue_xy: [0.150002046, 0.059997204],
        transfer_function: TransferFunction::SRGB,
        gamma: 0.0,
        rendering_intent: RenderingIntent::RELATIVE,
    }
}

#[test]
fn matrix_intermediates_float_and_double_match_original() {
    let mut cases = Vec::new();
    let mut seed = 414;
    for scalar in [32, 64] {
        for trial in 0..1024 {
            let mut line = format!("cms-matrix {scalar}");
            for i in 0..21 {
                let mut value = ((next(&mut seed) % 200001) as f64 - 100000.0) / 1000.0;
                if trial % 13 == 0 && i < 9 {
                    value = if i % 4 == 0 { 1.0 } else { 0.0 };
                }
                if trial % 17 == 0 && i < 9 {
                    value = 0.0;
                }
                write!(line, " {}", value.to_bits()).unwrap();
            }
            cases.push(line);
        }
    }
    compare(cases);
}

#[test]
fn source_general_float_format_and_icc_md5_match_original() {
    let mut cases = Vec::new();
    let mut seed = 141;
    let values = [
        0.0,
        -0.0,
        1.0,
        -1.0,
        1e-4,
        1e-5,
        1e6,
        999999.5,
        9.999995e-5,
        f64::MIN_POSITIVE,
        f64::MAX,
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::NAN,
        f64::from_bits(0xfff8000000000001),
    ];
    for value in values {
        cases.push(format!("cms-format {}", value.to_bits()));
    }
    for _ in 0..32768 {
        cases.push(format!("cms-format {}", next(&mut seed)));
    }
    for len in (0..257).chain([511, 512, 513, 1024, 4096]) {
        let bytes: Vec<_> = (0..len).map(|_| next(&mut seed) as u8).collect();
        cases.push(format!("cms-md5 {}", hex(&bytes)));
    }
    compare(cases);
}

#[test]
fn transfer_functions_tone_mapping_and_curve_tables_match_original() {
    let mut cases = Vec::new();
    let mut seed = 31;
    for which in 0..4 {
        for target in [1.0f32, 80.0, 255.0, 10000.0] {
            for i in 0..=1024 {
                let value = i as f64 / 1024.0 * if which == 2 { 1.0 } else { 4.0 };
                for sign in [1.0, -1.0] {
                    cases.push(format!(
                        "cms-transfer {which} {} {}",
                        target.to_bits(),
                        (value * sign).to_bits()
                    ));
                }
            }
        }
    }
    for which in 0..3 {
        for _ in 0..1024 {
            let mut line = format!("cms-tone {which}");
            let rgb = [
                next(&mut seed) % 10001,
                next(&mut seed) % 10001,
                next(&mut seed) % 10001,
            ]
            .map(|v| v as f32 / 10000.0);
            for value in [
                0.0, 10000.0, 0.0, 255.0, 0.2627, 0.6780, 0.0593, rgb[0], rgb[1], rgb[2], 0.3,
            ] {
                write!(line, " {}", value.to_bits()).unwrap();
            }
            cases.push(line);
        }
    }
    for pq in [0, 1] {
        for tone in [0, 1] {
            cases.push(format!("cms-curve {pq} {tone}"));
        }
    }
    // Vary the scalar constructor parameters in the ranges used by the
    // upstream tone_mapping_test.cc scenarios. Include out-of-gamut colors.
    for which in 0..3 {
        for _ in 0..1024 {
            let mut random =
                |low: f32, range: f32| low + (next(&mut seed) % 100001) as f32 / 100000.0 * range;
            let source = if which == 1 {
                random(250.0, 100.0)
            } else {
                random(10850.0, 300.0)
            };
            let target = if which == 1 {
                random(75.0, 10.0)
            } else {
                random(245.0, 10.0)
            };
            let lum = [random(0.2, 0.2), random(0.2, 0.2), random(0.2, 0.2)];
            let rgb = if which == 2 {
                [random(-0.5, 2.0), random(-0.5, 2.0), random(-0.5, 2.0)]
            } else {
                [random(0.0, 1.0), random(0.0, 1.0), random(0.0, 1.0)]
            };
            let mut line = format!("cms-tone {which}");
            for value in [
                0.0,
                source,
                0.0,
                target,
                lum[0],
                lum[1],
                lum[2],
                rgb[0],
                rgb[1],
                rgb[2],
                random(0.2, 0.2),
            ] {
                write!(line, " {}", value.to_bits()).unwrap();
            }
            cases.push(line);
        }
    }
    compare(cases);
}

#[test]
fn complete_icc_profiles_matrices_descriptions_and_error_state_match_original() {
    let mut cases = Vec::new();
    for color in [ColorSpace::RGB, ColorSpace::GRAY, ColorSpace::XYB] {
        for tf in [
            TransferFunction::SRGB,
            TransferFunction::LINEAR,
            TransferFunction::BT709,
            TransferFunction::PQ,
            TransferFunction::HLG,
            TransferFunction::DCI,
            TransferFunction::GAMMA,
        ] {
            for primaries in [
                Primaries::SRGB,
                Primaries::BT2100,
                Primaries::P3,
                Primaries::CUSTOM,
            ] {
                for wp in [
                    WhitePoint::D65,
                    WhitePoint::DCI,
                    WhitePoint::E,
                    WhitePoint::CUSTOM,
                ] {
                    let mut c = srgb();
                    c.color_space = color;
                    c.transfer_function = tf;
                    c.gamma = 1.0 / 2.2;
                    c.primaries = primaries;
                    c.white_point = wp;
                    c.rendering_intent = if color == ColorSpace::XYB {
                        RenderingIntent::PERCEPTUAL
                    } else {
                        RenderingIntent::RELATIVE
                    };
                    if primaries == Primaries::BT2100 {
                        c.primaries_red_xy = [0.708, 0.292];
                        c.primaries_green_xy = [0.170, 0.797];
                        c.primaries_blue_xy = [0.131, 0.046];
                    }
                    if primaries == Primaries::P3 {
                        c.primaries_red_xy = [0.680, 0.320];
                        c.primaries_green_xy = [0.265, 0.690];
                        c.primaries_blue_xy = [0.150, 0.060];
                    }
                    if wp == WhitePoint::DCI {
                        c.white_point_xy = [0.314, 0.351];
                    } else if wp == WhitePoint::E {
                        c.white_point_xy = [1.0 / 3.0; 2];
                    } else if wp == WhitePoint::CUSTOM {
                        c.white_point_xy = [0.345669, 0.358496];
                    }
                    let args = external_args(&c);
                    cases.push(format!("cms-profile {args}"));
                    for which in [0, 1] {
                        cases.push(format!("cms-color-matrix {args} {which}"));
                    }
                }
            }
        }
    }
    for wp in [[0.0, 0.0], [-0.1, 0.3], [1.1, 0.3], [0.3, 1e-40]] {
        let mut c = srgb();
        c.white_point = WhitePoint::CUSTOM;
        c.white_point_xy = wp;
        cases.push(format!("cms-profile {}", external_args(&c)));
    }
    let mut c = srgb();
    c.color_space = ColorSpace::UNKNOWN;
    cases.push(format!("cms-profile {}", external_args(&c)));
    let mut c = srgb();
    c.transfer_function = TransferFunction::UNKNOWN;
    cases.push(format!("cms-profile {}", external_args(&c)));
    for gamma in [0.0, 1.0 / 32768.0, -0.5, f64::NAN, f64::INFINITY] {
        let mut c = srgb();
        c.transfer_function = TransferFunction::GAMMA;
        c.gamma = gamma;
        cases.push(format!("cms-profile {}", external_args(&c)));
    }
    let mut seed = 716;
    for _ in 0..128 {
        let mut c = srgb();
        c.primaries = Primaries::CUSTOM;
        c.white_point = WhitePoint::CUSTOM;
        let mut delta = || ((next(&mut seed) % 2001) as f64 - 1000.0) / 100000.0;
        for xy in [
            &mut c.white_point_xy,
            &mut c.primaries_red_xy,
            &mut c.primaries_green_xy,
            &mut c.primaries_blue_xy,
        ] {
            for value in xy {
                *value += delta();
            }
        }
        cases.push(format!("cms-profile {}", external_args(&c)));
    }
    compare(cases);
}
