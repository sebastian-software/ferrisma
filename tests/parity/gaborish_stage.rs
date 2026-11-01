// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use super::*;
use jxl::jxl::frame_dimensions::FrameDimensions;
use jxl::jxl::image::{Image3, Plane};
use jxl::jxl::loop_filter::LoopFilter;
use jxl::jxl::render_pipeline::render_pipeline_stage::{
    RENDER_PIPELINE_X_OFFSET as OFFSET, Row, RowBuffers, RowStorage, WriteToImage3FStage,
};
use jxl::jxl::render_pipeline::simple_render_pipeline::SimpleRenderPipelineBuilder;
use jxl::jxl::render_pipeline::stage_gaborish::get_gaborish_stage;

pub(super) fn handles(kind: &str) -> bool {
    kind.starts_with("gab-")
}
fn filter(args: &mut std::str::SplitWhitespace<'_>) -> LoopFilter {
    LoopFilter {
        gab: number::<u32>(args) != 0,
        gab_x_weight1: f32::from_bits(number(args)),
        gab_x_weight2: f32::from_bits(number(args)),
        gab_y_weight1: f32::from_bits(number(args)),
        gab_y_weight2: f32::from_bits(number(args)),
        gab_b_weight1: f32::from_bits(number(args)),
        gab_b_weight2: f32::from_bits(number(args)),
        ..LoopFilter::default()
    }
}
pub(super) fn observe(kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    let lf = filter(args);
    let mut stage = get_gaborish_stage(&lf);
    if kind == "gab-factory" {
        let mut out = u32::from(stage.is_some()).to_string();
        if let Some(stage) = &mut stage {
            let s = stage.settings();
            write!(
                out,
                " {} {} {} {} {}",
                s.shift_x,
                s.shift_y,
                s.border_x,
                s.border_y,
                stage.get_name()
            )
            .unwrap();
            for i in [0, 1, 2, 3, 4, usize::MAX] {
                write!(out, " {}", stage.get_channel_mode(i) as i32).unwrap();
            }
            for n in [0, 2, 1] {
                write!(out, " {}", status_code(&stage.prepare_for_threads(n))).unwrap();
            }
            write!(out, " {}", status_code(&stage.is_initialized())).unwrap();
        }
        return out;
    }
    if kind == "gab-row" {
        let left: usize = number(args);
        let right: usize = number(args);
        let width: usize = number(args);
        let origin: usize = number(args);
        let values: [f32; 9] = std::array::from_fn(|_| f32::from_bits(number(args)));
        let len = origin + OFFSET + width + right + 3;
        let mut input: Vec<_> = (0..5).map(|_| Plane::create(len, 7).unwrap()).collect();
        let mut output: Vec<_> = (0..5).map(|_| Plane::create(len, 4).unwrap()).collect();
        for c in 0..5 {
            for y in 0..7 {
                for x in 0..len {
                    input[c].row_mut(y)[x] = values[(c * 7 + y * 3 + x) % 9];
                }
            }
            for y in 0..4 {
                for x in 0..len {
                    output[c].row_mut(y)[x] =
                        f32::from_bits(0x7fa00000 + (c * 256 + y * 64 + x) as u32);
                }
            }
        }
        let ir = (0..5)
            .map(|c| {
                [1, 3, 5]
                    .into_iter()
                    .map(|y| Row {
                        storage: RowStorage::Input,
                        channel: c,
                        y,
                        x: origin,
                    })
                    .collect()
            })
            .collect();
        let or = (0..5)
            .map(|c| {
                vec![Row {
                    storage: RowStorage::Output,
                    channel: c,
                    y: 2,
                    x: origin,
                }]
            })
            .collect();
        let status = stage.as_mut().unwrap().process_row(
            &ir,
            &or,
            &mut RowBuffers {
                input: &mut input,
                output: &mut output,
            },
            left,
            right,
            width,
            23,
            41,
            17,
        );
        let mut out = status_code(&status).to_string();
        for plane in &output {
            for y in 0..4 {
                for x in 0..len {
                    write!(out, " {}", plane.row(y)[x].to_bits()).unwrap();
                }
            }
        }
        return out;
    }
    assert_eq!(kind, "gab-simple");
    let w = number(args);
    let h = number(args);
    let threads = number::<usize>(args);
    let mode = number(args);
    let reverse = number::<u32>(args) != 0;
    let passes = number::<usize>(args);
    let mut dim = FrameDimensions::default();
    dim.set(w, h, 0, 0, 0, true, 1);
    let mut image = Image3::default();
    let mut out = String::new();
    {
        let mut builder = SimpleRenderPipelineBuilder::new(3);
        let added = builder.add_stage(stage);
        write!(out, " B {}", status_code(&added)).unwrap();
        if added.is_err() {
            return out;
        }
        builder
            .add_stage(Some(Box::new(WriteToImage3FStage::new(&mut image))))
            .unwrap();
        let mut pipeline = builder.finalize(dim).unwrap();
        let mut status = pipeline.prepare_for_threads(threads, true);
        write!(out, " T {}", status_code(&status)).unwrap();
        if status.is_ok() {
            for pass in 0..passes {
                for i in 0..dim.num_groups {
                    let g = if reverse { dim.num_groups - 1 - i } else { i };
                    let mut input = pipeline.get_input_buffers(g, pass % threads);
                    for c in 0..3 {
                        let (plane, r) = input.get_buffer_mut(c);
                        for y in 0..r.ysize() {
                            for x in 0..r.xsize() {
                                plane.row_mut(r.y0() + y)[r.x0() + x] = render_pipeline::value(
                                    c,
                                    r.x0() + x - OFFSET,
                                    r.y0() + y - OFFSET,
                                    pass,
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
                if status.is_err() {
                    break;
                }
                status = pipeline.prepare_for_threads(threads, true);
                write!(out, " A {}", status_code(&status)).unwrap();
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
fn params(gab: u32, weights: [f32; 6]) -> String {
    let mut out = gab.to_string();
    for value in weights {
        write!(out, " {}", value.to_bits()).unwrap();
    }
    out
}
fn presets() -> Vec<[f32; 6]> {
    let lf = LoopFilter::default();
    vec![
        [
            lf.gab_x_weight1,
            lf.gab_x_weight2,
            lf.gab_y_weight1,
            lf.gab_y_weight2,
            lf.gab_b_weight1,
            lf.gab_b_weight2,
        ],
        [0.0; 6],
        [0.125, -0.0625, 0.5, 0.25, -0.1, 0.35],
        [
            -0.25,
            0.0,
            f32::from_bits(0xbe7fffff),
            0.0,
            f32::from_bits(0xbe800001),
            0.0,
        ],
        [
            f32::from_bits(0x7fa12345),
            -0.0,
            f32::INFINITY,
            f32::NEG_INFINITY,
            f32::from_bits(0x7fc98765),
            f32::from_bits(1),
        ],
    ]
}
#[test]
fn scalar_gaborish_factory_settings_channel_modes_and_lifecycle_match_original() {
    compare(
        [0, 1]
            .into_iter()
            .flat_map(|gab| {
                presets()
                    .into_iter()
                    .map(move |weights| format!("gab-factory {}", params(gab, weights)))
            })
            .collect(),
    );
}

#[test]
fn scalar_gaborish_competing_nan_normalization_and_pixel_payloads_match_original() {
    let words = [
        0x7fa12345, 0xffa54321, 0x7fc98765, 0xffc43210, 0, 0x7f800000, 0xff800000, 1,
    ];
    let mut cases = Vec::new();
    for a in words.map(f32::from_bits) {
        for b in words.map(f32::from_bits) {
            let weights = [a, b, b, a, a, b];
            for pixels in [
                "1065353216 1065353216 1065353216 1065353216 1065353216 1065353216 1065353216 1065353216 1065353216",
                "2141266757 4289020705 2143913829 4291047952 2139095040 4286578688 1056964608 1065353216 2147483648",
            ] {
                cases.push(format!("gab-row {} 1 2 3 3 {pixels}", params(1, weights)));
            }
        }
    }
    compare(cases);
}
#[test]
fn scalar_gaborish_rows_normalization_extras_offsets_and_float_bits_match_original() {
    let mut cases = Vec::new();
    let mut inputs = vec![
        [0.0; 9],
        [-0.0; 9],
        [1.0; 9],
        [
            0.0,
            -0.0,
            f32::from_bits(1),
            f32::from_bits(0x80000001),
            f32::MIN_POSITIVE,
            f32::MAX,
            -f32::MAX,
            1.0,
            -1.0,
        ],
        [
            f32::from_bits(0x7fa12345),
            f32::from_bits(0xffa54321),
            f32::from_bits(0x7fc98765),
            f32::from_bits(0xffc43210),
            f32::INFINITY,
            f32::NEG_INFINITY,
            0.5,
            1.0,
            -0.0,
        ],
    ];
    let mut seed = 617;
    for _ in 0..12 {
        inputs.push(std::array::from_fn(|_| {
            f32::from_bits(next(&mut seed) as u32)
        }));
    }
    let mut weights = presets();
    for _ in 0..12 {
        weights.push(std::array::from_fn(|_| {
            f32::from_bits(next(&mut seed) as u32)
        }));
    }
    for weights in weights {
        for values in &inputs {
            for (left, right, width, origin) in [
                (0, 0, 0, 0),
                (1, 2, 0, 3),
                (0, 0, 1, 0),
                (1, 2, 3, 3),
                (0, 7, 17, 3),
                (20, 7, 17, 0),
            ] {
                let mut request = format!(
                    "gab-row {} {left} {right} {width} {origin}",
                    params(1, weights)
                );
                for value in values {
                    write!(request, " {}", value.to_bits()).unwrap();
                }
                cases.push(request);
            }
        }
    }
    compare(cases);
}
#[test]
fn scalar_gaborish_simple_pipeline_mirrors_edges_groups_and_repeated_passes_match_original() {
    let mut cases = vec![format!("gab-simple {} 3 2 1 1 0 1", params(0, [0.0; 6]))];
    for (w, h) in [
        (1, 1),
        (2, 3),
        (17, 9),
        (65, 3),
        (255, 2),
        (257, 3),
        (3, 257),
        (513, 1),
    ] {
        for weights in presets() {
            for mode in [0, 1, 3] {
                for reverse in [0, 1] {
                    cases.push(format!(
                        "gab-simple {} {w} {h} 2 {mode} {reverse} 2",
                        params(1, weights)
                    ));
                }
            }
        }
    }
    compare(cases);
}
