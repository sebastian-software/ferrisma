// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use super::*;
use jxl::jxl::frame_dimensions::FrameDimensions;
use jxl::jxl::image::Image3;
use jxl::jxl::image_metadata::CustomTransformData;
use jxl::jxl::render_pipeline::render_pipeline_stage::{
    RENDER_PIPELINE_X_OFFSET as OFFSET, WriteToImage3FStage,
};
use jxl::jxl::render_pipeline::simple_render_pipeline::SimpleRenderPipelineBuilder;
use jxl::jxl::render_pipeline::stage_upsampling::get_upsampling_stage;
pub(super) fn handles(kind: &str) -> bool {
    kind.starts_with("up-")
}
fn weights(factors: &mut CustomTransformData, mode: u32) {
    if mode == 0 {
        return;
    }
    for weights in [
        &mut factors.upsampling2_weights[..],
        &mut factors.upsampling4_weights[..],
        &mut factors.upsampling8_weights[..],
    ] {
        for (i, w) in weights.iter_mut().enumerate() {
            *w = match mode {
                1 => ((i % 17) as i32 - 8) as f32 / 32.0,
                2 => 0.0,
                _ => f32::from_bits(0x7fc98765),
            };
        }
    }
}
pub(super) fn observe(kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    if kind == "up-guard" {
        use jxl::jxl::image::Plane;
        use jxl::jxl::render_pipeline::render_pipeline_stage::{Row, RowBuffers, RowStorage};
        let left = number(args);
        let right = number(args);
        let width = number(args);
        let stage = get_upsampling_stage(&CustomTransformData::default(), 0, 1).unwrap();
        let mut input = [Plane::create(64, 8).unwrap()];
        let mut output = [Plane::create(64, 8).unwrap()];
        for y in 0..8 {
            input[0].row_mut(y)[..64].fill(-7.0);
            output[0].row_mut(y)[..64].fill(-11.0);
        }
        let ir = vec![
            (0..5)
                .map(|y| Row {
                    storage: RowStorage::Input,
                    channel: 0,
                    y,
                    x: 0,
                })
                .collect(),
        ];
        let or = vec![
            (0..2)
                .map(|y| Row {
                    storage: RowStorage::Output,
                    channel: 0,
                    y,
                    x: 0,
                })
                .collect(),
        ];
        let mut buffers = RowBuffers {
            input: &mut input,
            output: &mut output,
        };
        let prepared = stage.prepare_for_threads(1);
        let status = stage.process_row(&ir, &or, &mut buffers, left, right, width, 0, 0, 0);
        let mut out = format!("{} {}", status_code(&prepared), status_code(&status));
        for y in 0..8 {
            for x in 0..64 {
                write!(out, " {}", output[0].row(y)[x].to_bits()).unwrap();
            }
        }
        return out;
    }
    if kind == "up-factory" {
        let shift = number(args);
        let c = number(args);
        let mut stage = get_upsampling_stage(&CustomTransformData::default(), c, shift);
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
            for i in 0..6 {
                write!(out, " {}", stage.get_channel_mode(i) as i32).unwrap();
            }
            for n in [0, 2, 1] {
                write!(out, " {}", status_code(&stage.prepare_for_threads(n))).unwrap();
            }
        }
        return out;
    }
    assert_eq!(kind, "up-simple");
    let w = number(args);
    let h = number(args);
    let shift = number(args);
    let c = number(args);
    let threads = number::<usize>(args);
    let mode = number(args);
    let weight_mode = number(args);
    let reverse = number::<u32>(args) != 0;
    let passes = number::<usize>(args);
    let mut factors = CustomTransformData::default();
    weights(&mut factors, weight_mode);
    let mut dim = FrameDimensions::default();
    dim.set(w, h, 0, 0, 0, true, 1);
    let mut image = Image3::default();
    let mut out = String::new();
    {
        let mut builder = SimpleRenderPipelineBuilder::new(3);
        let added = builder.add_stage(get_upsampling_stage(&factors, c, shift));
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

#[test]
fn scalar_upsampling_kernel_weights_mirrors_clamps_and_group_orders_match_original() {
    let mut cases = Vec::new();
    for (w, h) in [(1, 1), (3, 2), (17, 9), (65, 3), (257, 3), (8, 67)] {
        for shift in 1..=3 {
            for c in 0..3 {
                for mode in [0, 1, 3] {
                    for weights in 0..4 {
                        for reverse in [0, 1] {
                            cases.push(format!(
                                "up-simple {w} {h} {shift} {c} 2 {mode} {weights} {reverse} 1"
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
fn scalar_upsampling_chunks_and_factory_preconditions_match_original() {
    let mut cases = Vec::new();
    for shift in [0, 1, 2, 3, 4, usize::MAX] {
        for c in [0, 1, 2, 5] {
            cases.push(format!("up-factory {shift} {c}"));
        }
    }
    for shift in 1..=3 {
        for w in [1023, 1024, 1025, 2049, 8197] {
            for weights in [0, 1] {
                cases.push(format!("up-simple {w} 1 {shift} 0 1 1 {weights} 1 1"));
            }
        }
    }
    cases.push("up-simple 3 2 0 0 1 1 0 0 1".into());
    cases.push("up-simple 3 2 4 0 1 1 0 0 1".into());
    for (left, right) in [(0, 0), (1, 0), (0, 1), (2, 3)] {
        for width in [0, 1] {
            cases.push(format!("up-guard {left} {right} {width}"));
        }
    }
    compare(cases);
}
