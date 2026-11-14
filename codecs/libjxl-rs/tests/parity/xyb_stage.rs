// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use super::*;
use jxl::jxl::color_encoding::ColorSpace;
use jxl::jxl::dec_xyb::{OpsinParams, OutputEncodingInfo};
use jxl::jxl::dec_xyb_scalar::xyb_to_rgb;
use jxl::jxl::frame_dimensions::FrameDimensions;
use jxl::jxl::image::{Image3, Plane};
use jxl::jxl::render_pipeline::render_pipeline_stage::{
    RENDER_PIPELINE_X_OFFSET as OFFSET, Row, RowBuffers, RowStorage, WriteToImage3FStage,
};
use jxl::jxl::render_pipeline::simple_render_pipeline::SimpleRenderPipelineBuilder;
use jxl::jxl::render_pipeline::stage_xyb::get_xyb_stage;

pub(super) fn handles(kind: &str) -> bool {
    kind.starts_with("xyb-")
}
fn f(args: &mut std::str::SplitWhitespace<'_>) -> f32 {
    f32::from_bits(number(args))
}
fn params(args: &mut std::str::SplitWhitespace<'_>) -> OpsinParams {
    let custom: u32 = number(args);
    let target = f(args);
    let mut p = OpsinParams::default();
    if custom == 0 {
        p.init(target);
    } else {
        for v in p
            .inverse_opsin_matrix
            .iter_mut()
            .chain(&mut p.opsin_biases)
            .chain(&mut p.opsin_biases_cbrt)
            .chain(&mut p.quant_biases)
        {
            *v = f(args);
        }
    }
    p
}
pub(super) fn observe(kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    let p = params(args);
    if kind == "xyb-helper" {
        let rgb = xyb_to_rgb(f(args), f(args), f(args), &p);
        return words(&rgb);
    }
    let space = ColorSpace(number(args));
    let w = number(args);
    let h = number(args);
    let left = number(args);
    let right = number(args);
    let offset = number(args);
    let mode = number(args);
    let mut info = OutputEncodingInfo {
        opsin_params: p,
        ..OutputEncodingInfo::default()
    };
    info.color_encoding.set_color_space(space);
    let stage = get_xyb_stage(&info);
    info.opsin_params = OpsinParams::default();
    info.color_encoding
        .set_color_space(if space == ColorSpace::XYB {
            ColorSpace::RGB
        } else {
            ColorSpace::XYB
        });
    let mut out = String::new();
    if kind == "xyb-stage" {
        let s = stage.settings();
        write!(
            out,
            "{} {} {} {} {}",
            stage.get_name(),
            s.shift_x,
            s.shift_y,
            s.border_x,
            s.border_y
        )
        .unwrap();
        for c in 0..6 {
            write!(out, " {}", stage.get_channel_mode(c) as i32).unwrap();
        }
        let mut planes: Vec<_> = (0..4)
            .map(|c| {
                let mut p = Plane::create(128, 1).unwrap();
                for x in 0..128 {
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
                    x: offset,
                }]
            })
            .collect();
        let mut buffers = RowBuffers {
            input: &mut planes,
            output: &mut [],
        };
        let status = stage.process_row(&rows, &Vec::new(), &mut buffers, left, right, w, 19, 23, 1);
        write!(out, " R {}", status_code(&status)).unwrap();
        for p in planes {
            for x in 0..128 {
                write!(out, " {}", p.row(0)[x].to_bits()).unwrap();
            }
        }
        return out;
    }
    assert_eq!(kind, "xyb-simple");
    let mut dim = FrameDimensions::default();
    dim.set(w, h, 0, 0, 0, true, 1);
    let mut image = Image3::default();
    {
        let mut builder = SimpleRenderPipelineBuilder::new(3);
        builder.add_stage(Some(stage)).unwrap();
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
fn custom(p: &OpsinParams) -> String {
    let values: Vec<_> = p
        .inverse_opsin_matrix
        .iter()
        .chain(&p.opsin_biases)
        .chain(&p.opsin_biases_cbrt)
        .chain(&p.quant_biases)
        .copied()
        .collect();
    format!("1 0 {}", words(&values))
}
fn configs() -> Vec<String> {
    let mut configs: Vec<_> = [1.0, 80.0, 255.0, 1000.0, 10000.0]
        .into_iter()
        .map(|t: f32| format!("0 {}", t.to_bits()))
        .collect();
    let mut p = OpsinParams::default();
    for (i, v) in p.inverse_opsin_matrix.iter_mut().enumerate() {
        *v = (i as f32 - 11.0) / 16.0;
    }
    p.opsin_biases = [-0.125, -0.0, 0.25, f32::from_bits(0x7fa12345)];
    p.opsin_biases_cbrt = [-0.5, 0.0, 0.625, f32::from_bits(0xffa98765)];
    p.quant_biases = [f32::from_bits(0x7fa98765); 4];
    configs.push(custom(&p));
    configs
}
#[test]
fn xyb_scalar_helper_matches_default_custom_matrices_and_exceptional_float_words() {
    let mut colors = vec![[0.0; 3], [-0.0; 3], [0.015, 0.5, 0.75], [-0.25, -1.0, 2.0]];
    let special = [
        0, 0x80000000, 1, 0x80000001, 0x7fffff, 0x800000, 0x7f7fffff, 0xff7fffff, 0x7f800000,
        0xff800000, 0x7fc12345, 0xffc98765, 0x7fa12345, 0xffa98765,
    ];
    for bits in special {
        for c in 0..3 {
            let mut rgb = [0.125, 0.5, 0.75];
            rgb[c] = f32::from_bits(bits);
            colors.push(rgb);
        }
    }
    for a in special {
        for b in special {
            colors.push([f32::from_bits(a), f32::from_bits(b), 0.75]);
        }
    }
    let mut seed = 0x123abcd9u64;
    for _ in 0..512 {
        colors.push(std::array::from_fn(|_| {
            f32::from_bits(next(&mut seed) as u32)
        }));
    }
    let mut cases = Vec::new();
    for config in configs() {
        for color in &colors {
            cases.push(format!("xyb-helper {config} {}", words(color)));
        }
    }
    // Every scalar source parameter word is independently supplied. Nonleading
    // broadcast lanes and fourth bias/quant lanes must not affect the result.
    for i in 0..48 {
        for bits in special {
            let mut p = OpsinParams::default();
            p.init(255.0);
            let v = f32::from_bits(bits);
            if i < 36 {
                p.inverse_opsin_matrix[i] = v;
            } else if i < 40 {
                p.opsin_biases[i - 36] = v;
            } else if i < 44 {
                p.opsin_biases_cbrt[i - 40] = v;
            } else {
                p.quant_biases[i - 44] = v;
            }
            for color in [
                [0.125, 0.5, 0.75],
                [f32::from_bits(0x7fc12345), f32::from_bits(0xffc98765), 0.75],
                [f32::INFINITY, f32::NEG_INFINITY, f32::from_bits(0xffa98765)],
            ] {
                cases.push(format!("xyb-helper {} {}", custom(&p), words(&color)));
            }
        }
    }
    compare(cases);
}
#[test]
fn xyb_stage_factory_copy_guards_offsets_channel_modes_and_scaled_output_match() {
    let mut cases = Vec::new();
    for config in configs() {
        for space in [0, 1, 2, 3] {
            for mode in [0, 1, 3] {
                for w in [0, 1, 3, 17, 31] {
                    for offset in [0, 1, 19] {
                        for (left, right) in [(0, 0), (1, 0), (0, 1), (2, 3)] {
                            cases.push(format!(
                                "xyb-stage {config} {space} {w} 1 {left} {right} {offset} {mode}"
                            ));
                        }
                    }
                }
            }
        }
    }
    compare(cases);
}
#[test]
fn xyb_stage_runs_through_simple_pipeline_and_reverse_group_submission() {
    let mut cases = Vec::new();
    for config in configs() {
        for space in [0, 2] {
            for mode in [0, 1, 3] {
                for (w, h) in [(1, 1), (17, 3), (257, 2), (513, 3)] {
                    cases.push(format!("xyb-simple {config} {space} {w} {h} 0 0 0 {mode}"));
                }
            }
        }
    }
    compare(cases);
}
