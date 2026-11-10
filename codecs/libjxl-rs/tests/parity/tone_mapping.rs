// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use super::*;
use jxl::jxl::base::fast_math::{fast_pow2f, fast_powf};
use jxl::jxl::cms::tone_mapping_scalar::{HlgOotf, Rec2408ToneMapper, gamut_map};
use jxl::jxl::cms::transfer_functions_scalar::TfPq;
use jxl::jxl::color_encoding::TransferFunction;
use jxl::jxl::dec_xyb::OutputEncodingInfo;
use jxl::jxl::frame_dimensions::FrameDimensions;
use jxl::jxl::image::{Image3, Plane};
use jxl::jxl::render_pipeline::render_pipeline_stage::{
    RENDER_PIPELINE_X_OFFSET as OFFSET, RenderPipelineStage, Row, RowBuffers, RowStorage,
    WriteToImage3FStage,
};
use jxl::jxl::render_pipeline::simple_render_pipeline::SimpleRenderPipelineBuilder;
use jxl::jxl::render_pipeline::stage_tone_mapping::{ToneMappingStage, get_tone_mapping_stage};
pub(super) fn handles(kind: &str) -> bool {
    kind.starts_with("tm-")
}
fn f(args: &mut std::str::SplitWhitespace<'_>) -> f32 {
    f32::from_bits(number(args))
}
pub(super) fn observe(kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    if kind == "tm-fast" {
        let which: u32 = number(args);
        let target = f(args);
        let x = f(args);
        let e = f(args);
        return match which {
            0 => fast_pow2f(x),
            1 => fast_powf(x, e),
            2 => TfPq::new(target).display_from_encoded(x),
            _ => TfPq::new(target).encoded_from_display(x),
        }
        .to_bits()
        .to_string();
    }
    if kind == "tm-color" {
        let which: u32 = number(args);
        let source = [f(args), f(args)];
        let target = [f(args), f(args)];
        let lum = [f(args), f(args), f(args)];
        let mut rgb = [f(args), f(args), f(args)];
        let preserve = f(args);
        let mut gamut = false;
        match which {
            0 => Rec2408ToneMapper::new(source, target, lum).tone_map(&mut rgb),
            4 => gamut_map(&mut rgb, lum, preserve),
            _ => {
                let ootf = match which {
                    1 => HlgOotf::new(source[1], target[1], lum),
                    2 => HlgOotf::from_scene_light(target[1], lum),
                    _ => HlgOotf::to_scene_light(target[1], lum),
                };
                gamut = ootf.warrants_gamut_mapping();
                ootf.apply(&mut rgb);
            }
        }
        return format!(
            "{} {} {} {}",
            u32::from(gamut),
            rgb[0].to_bits(),
            rgb[1].to_bits(),
            rgb[2].to_bits()
        );
    }
    let orig = TransferFunction(number(args));
    let dest = TransferFunction(number(args));
    let mut info = OutputEncodingInfo {
        orig_intensity_target: f(args),
        desired_intensity_target: f(args),
        luminances: [f(args), f(args), f(args)],
        ..OutputEncodingInfo::default()
    };
    info.orig_color_encoding
        .tf_mut()
        .set_transfer_function(orig);
    info.color_encoding.tf_mut().set_transfer_function(dest);
    let w = number(args);
    let h = number(args);
    let left = number(args);
    let right = number(args);
    let mode = number(args);
    let factory = get_tone_mapping_stage(&info);
    let mut out = format!(" H {}", u32::from(factory.is_some()));
    if kind == "tm-stage" {
        let mut stage = ToneMappingStage::new(info);
        let s = stage.settings();
        write!(
            out,
            " {} {} {} {} {} {}",
            u32::from(stage.is_needed()),
            stage.get_name(),
            s.shift_x,
            s.shift_y,
            s.border_x,
            s.border_y
        )
        .unwrap();
        for c in 0..5 {
            write!(out, " {}", stage.get_channel_mode(c) as i32).unwrap();
        }
        let mut planes: Vec<_> = (0..4)
            .map(|c| {
                let mut p = Plane::create(64, 1).unwrap();
                for x in 0..64 {
                    p.row_mut(0)[x] = render_pipeline::value(c, x, 0, 0, mode);
                }
                p
            })
            .collect();
        let rows = (0..4)
            .map(|channel| {
                vec![Row {
                    storage: RowStorage::Input,
                    channel,
                    y: 0,
                    x: 0,
                }]
            })
            .collect();
        let mut buffers = RowBuffers {
            input: &mut planes,
            output: &mut [],
        };
        let status = stage.process_row(&rows, &Vec::new(), &mut buffers, left, right, w, 19, 23, 0);
        write!(out, " R {}", status_code(&status)).unwrap();
        for p in planes {
            for x in 0..64 {
                write!(out, " {}", p.row(0)[x].to_bits()).unwrap();
            }
        }
        return out;
    }
    assert_eq!(kind, "tm-simple");
    let mut dim = FrameDimensions::default();
    dim.set(w, h, 0, 0, 0, true, 1);
    let mut image = Image3::default();
    {
        let mut builder = SimpleRenderPipelineBuilder::new(3);
        if factory.is_some() {
            builder.add_stage(factory).unwrap();
        }
        builder
            .add_stage(Some(Box::new(WriteToImage3FStage::new(&mut image))))
            .unwrap();
        let mut pipeline = builder.finalize(dim).unwrap();
        let mut status = pipeline.prepare_for_threads(2, true);
        write!(out, " T {}", status_code(&status)).unwrap();
        if status.is_ok() {
            for i in 0..dim.num_groups {
                let mut input = pipeline.get_input_buffers(dim.num_groups - 1 - i, i % 2);
                for c in 0..3 {
                    let (plane, r) = input.get_buffer_mut(c);
                    for y in 0..r.ysize() {
                        for x in 0..r.xsize() {
                            plane.row_mut(r.y0() + y)[r.x0() + x] = render_pipeline::value(
                                c,
                                r.x0() + x - OFFSET,
                                r.y0() + y - OFFSET,
                                0,
                                mode,
                            );
                        }
                    }
                }
                status = input.done();
                write!(out, " D {}", status_code(&status)).unwrap();
                if status.is_err() {
                    break;
                }
            }
        }
        render_pipeline::pipeline(&mut out, &pipeline);
    }
    modular_groups::output(&mut out, &image);
    out
}
fn words(values: &[f32]) -> String {
    values
        .iter()
        .map(|v| v.to_bits().to_string())
        .collect::<Vec<_>>()
        .join(" ")
}
#[test]
fn scalar_fast_power_and_pq_rational_signs_thresholds_and_exceptional_values_match() {
    let mut xs = vec![
        -256.5, -150.0, -128.0, -1.5, -1.0, -0.0, 0.0, 1e-6, 1e-4, 0.1, 0.5, 1.0, 1.5, 126.75,
        128.0, 256.5,
    ];
    for bits in [
        1, 0x7fffff, 0x800000, 0x38d1b716, 0x38d1b717, 0x38d1b718, 0x7f7fffff, 0x7f800000,
        0xff800000, 0x7fc98765, 0x7fa12345, 0xffa12345, 0xcf000000, 0x4f000000,
    ] {
        xs.push(f32::from_bits(bits));
    }
    let mut seed = 0xab01ce6u64;
    for _ in 0..256 {
        xs.push(f32::from_bits(next(&mut seed) as u32));
    }
    let mut cases = Vec::new();
    for x in xs {
        cases.push(format!("tm-fast 0 {}", words(&[1.0, x, 0.0])));
        for e in [-2.0, -0.0, 0.0, 0.01, 0.25, 1.0, 3.5] {
            cases.push(format!("tm-fast 1 {}", words(&[1.0, x, e])));
        }
        for target in [1.0, 80.0, 255.0, 1000.0, 10000.0] {
            for which in [2, 3] {
                cases.push(format!("tm-fast {which} {}", words(&[target, x, 0.0])));
            }
        }
    }
    compare(cases);
}
#[test]
fn scalar_tone_mapping_ootf_and_gamut_operations_match_original() {
    let mut cases = Vec::new();
    // The ordinary native regression's independently captured source input.
    // Keep exact per-target NaN payload evidence in the real oracle as well.
    cases.push(format!(
        "tm-color 4 {}",
        words(&[
            0.0,
            1000.0,
            0.0,
            255.0,
            0.2126,
            0.7152,
            0.0722,
            -0.0,
            f32::from_bits(0x7fa12345),
            f32::from_bits(0x7fc98765),
            0.0,
        ])
    ));
    for which in 0..5 {
        for (source, target) in [
            (1000.0, 255.0),
            (255.0, 1000.0),
            (1000.0, 1000.0),
            (10000.0, 80.0),
            (1000.0, 990.0),
            (1000.0, 1010.0),
        ] {
            for lum in [
                [0.2126, 0.7152, 0.0722],
                [0.2627, 0.6780, 0.0593],
                [1.0, 0.0, 0.0],
            ] {
                let mut colors = vec![
                    [0.0; 3],
                    [-0.0; 3],
                    [1e-9, 1e-6, 1e-3],
                    [1.0; 3],
                    [-1.0, 3.5, 0.25],
                    [0.25, 0.5, 2.0],
                ];
                for mode in [1, 3] {
                    for x in 0..16 {
                        colors.push(std::array::from_fn(|c| {
                            render_pipeline::value(c, x, 0, 0, mode)
                        }));
                    }
                }
                for rgb in colors {
                    for preserve in [0.0, 0.1, 1.0] {
                        cases.push(format!(
                            "tm-color {which} {}",
                            words(&[
                                0.0, source, 0.0, target, lum[0], lum[1], lum[2], rgb[0], rgb[1],
                                rgb[2], preserve
                            ])
                        ));
                    }
                }
            }
        }
    }
    compare(cases);
}
#[test]
fn tone_mapping_stage_factory_no_op_guards_pq_scaling_and_simple_pipeline_match() {
    let mut cases = Vec::new();
    for orig in [8, 13, 16, 18] {
        for dest in [8, 13, 16, 18] {
            for (source, target) in [(1000.0, 255.0), (255.0, 1000.0), (1000.0, 1000.0)] {
                let params = format!(
                    "{orig} {dest} {}",
                    words(&[source, target, 0.2126, 0.7152, 0.0722])
                );
                for mode in [0, 1, 3] {
                    for w in [0, 1, 17] {
                        for (left, right) in [(0, 0), (1, 0), (0, 1)] {
                            cases.push(format!("tm-stage {params} {w} 1 {left} {right} {mode}"));
                        }
                    }
                    for (w, h) in [(1, 1), (17, 3), (257, 2)] {
                        cases.push(format!("tm-simple {params} {w} {h} 0 0 {mode}"));
                    }
                }
            }
        }
    }
    compare(cases);
}
