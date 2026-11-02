// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use super::*;
use jxl::jxl::render_pipeline::{
    render_pipeline_stage::{
        RENDER_PIPELINE_X_OFFSET as OFFSET, RenderPipelineStage, Row, RowBuffers, RowStorage,
        WriteToImage3FStage,
    },
    simple_render_pipeline::SimpleRenderPipelineBuilder,
    stage_chroma_upsampling::get_chroma_upsampling_stage,
    stage_ycbcr::get_ycbcr_stage,
};
use jxl::jxl::{
    frame_dimensions::FrameDimensions,
    image::{Image3, Plane},
};
pub(super) fn handles(kind: &str) -> bool {
    kind.starts_with("cs-")
}
fn stage(kind: u32, channel: usize) -> Box<dyn RenderPipelineStage> {
    if kind == 0 {
        get_ycbcr_stage()
    } else {
        get_chroma_upsampling_stage(channel, kind == 1)
    }
}
pub(super) fn observe(kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    if kind == "cs-simple" {
        let mask: u32 = number(args);
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
            for c in 0..3 {
                for (bit, horizontal) in [(c, true), (c + 3, false)] {
                    if mask & (1 << bit) != 0 {
                        let status =
                            builder.add_stage(Some(get_chroma_upsampling_stage(c, horizontal)));
                        write!(out, " B {}", status_code(&status)).unwrap();
                        if status.is_err() {
                            return out;
                        }
                    }
                }
            }
            if mask & 64 != 0 {
                let status = builder.add_stage(Some(get_ycbcr_stage()));
                write!(out, " B {}", status_code(&status)).unwrap();
                if status.is_err() {
                    return out;
                }
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
        return out;
    }
    let t: u32 = number(args);
    let c = number(args);
    let mut stage = stage(t, c);
    if kind == "cs-factory" {
        let s = stage.settings();
        let mut out = format!(
            "{} {} {} {} {}",
            s.shift_x,
            s.shift_y,
            s.border_x,
            s.border_y,
            stage.get_name()
        );
        for c in [0, 1, 2, 3, 4, usize::MAX] {
            write!(out, " {}", stage.get_channel_mode(c) as i32).unwrap();
        }
        for n in [0, 2, 1] {
            write!(out, " {}", status_code(&stage.prepare_for_threads(n))).unwrap();
        }
        write!(out, " {}", status_code(&stage.is_initialized())).unwrap();
        return out;
    }
    let left: usize = number(args);
    let right: usize = number(args);
    let width: usize = number(args);
    let origin: usize = number(args);
    let values: [f32; 9] = std::array::from_fn(|_| f32::from_bits(number(args)));
    let len = origin + OFFSET + 2 * (width + right) + 5;
    let mut input: Vec<_> = (0..5)
        .map(|_| Plane::<f32>::create(len, 7).unwrap())
        .collect();
    let mut output: Vec<_> = (0..5)
        .map(|_| Plane::<f32>::create(len, 5).unwrap())
        .collect();
    for c in 0..5 {
        for y in 0..7 {
            for x in 0..len {
                input[c].row_mut(y)[x] = values[(c * 7 + y * 3 + x) % 9];
            }
        }
        for y in 0..5 {
            for x in 0..len {
                output[c].row_mut(y)[x] =
                    f32::from_bits(0x7fa00000 + (c * 8192 + y * 1024 + x) as u32);
            }
        }
    }
    let ir = (0..5)
        .map(|c| {
            if t == 2 { vec![1, 3, 5] } else { vec![3] }
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
            if t == 2 { vec![1, 3] } else { vec![2] }
                .into_iter()
                .map(|y| Row {
                    storage: RowStorage::Output,
                    channel: c,
                    y,
                    x: origin,
                })
                .collect()
        })
        .collect();
    let status = stage.process_row(
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
    for planes in [&input, &output] {
        for p in planes {
            for y in 0..p.ysize() {
                for x in 0..len {
                    write!(out, " {}", p.row(y)[x].to_bits()).unwrap();
                }
            }
        }
    }
    out
}
fn compare_chroma(cases: Vec<String>) {
    let observed = original_observations(&cases);
    let mut failures = 0;
    for (request, cpp) in cases.iter().zip(observed) {
        let rust = super::observe(request);
        if rust != cpp {
            failures += 1;
            if failures <= 8 {
                let diffs: Vec<_> = rust
                    .split_whitespace()
                    .zip(cpp.split_whitespace())
                    .enumerate()
                    .filter(|(_, (a, b))| a != b)
                    .take(8)
                    .collect();
                eprintln!("Chroma mismatch {request}\n{diffs:?}");
            }
        }
    }
    assert_eq!(failures, 0, "Chroma exact mismatches");
    println!("{} C++/Rust differential cases matched", cases.len());
}
#[test]
fn chroma_stage_factories_settings_modes_and_lifecycle_match_original() {
    compare_chroma(
        (0..3)
            .flat_map(|t| {
                [0, 1, 2, 3, 4, usize::MAX]
                    .into_iter()
                    .map(move |c| format!("cs-factory {t} {c}"))
            })
            .collect(),
    );
}
#[test]
fn chroma_ycbcr_rows_guards_extras_and_special_float_words_match_original() {
    let mut patterns = vec![
        [0u32; 9],
        [0x80000000; 9],
        [0x3f800000; 9],
        [
            0, 0x80000000, 1, 0x80000001, 0x007fffff, 0x00800000, 0x7f7fffff, 0xff7fffff,
            0x3f800001,
        ],
        [
            0x7fa12345, 0xffa54321, 0x7fc98765, 0xffc43210, 0x7f800000, 0xff800000, 0x3f000000,
            0x3f800000, 0x80000000,
        ],
    ];
    let mut seed = 7919;
    for _ in 0..20 {
        patterns.push(std::array::from_fn(|_| next(&mut seed) as u32));
    }
    for _ in 0..12 {
        patterns.push(std::array::from_fn(|_| {
            ((next(&mut seed) as i32 % 1024) as f32 / 512.0).to_bits()
        }));
    }
    let mut cases = Vec::new();
    for t in 0..3 {
        for c in 0..5 {
            for values in &patterns {
                for (left, right, width, origin) in [
                    (0, 0, 0, 0),
                    (0, 0, 1, 3),
                    (0, 0, 7, 0),
                    (1, 2, 0, 3),
                    (2, 3, 8, 1),
                    (10, 7, 33, 0),
                ] {
                    let mut s = format!("cs-row {t} {c} {left} {right} {width} {origin}");
                    for word in values {
                        write!(s, " {word}").unwrap();
                    }
                    cases.push(s);
                }
            }
        }
    }
    compare_chroma(cases);
}
#[test]
fn chroma_simple_pipeline_mirrors_subsampled_odd_dimensions_groups_and_repeated_passes_match_original()
 {
    let mut cases = Vec::new();
    for mask in [64, 1, 8, 9, 5, 40, 45, 63, 64 | 45, 64 | 63] {
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
            for mode in [0, 1, 3] {
                for reverse in [0, 1] {
                    cases.push(format!("cs-simple {mask} {w} {h} 2 {mode} {reverse} 2"));
                }
            }
        }
    }
    compare_chroma(cases);
}

#[test]
fn chroma_competing_special_float_pairs_keep_original_operand_words() {
    let words = [
        0u32, 0x80000000, 1, 0x80000001, 0x00800000, 0x7f7fffff, 0xff7fffff, 0x7f800000,
        0xff800000, 0x7fc12345, 0xffc98765, 0x7fa12345, 0xffa98765,
    ];
    let mut cases = Vec::new();
    for t in 0..3 {
        for a in words {
            for b in words {
                for (left, right, width, origin) in [(0, 0, 3, 0), (2, 3, 7, 1)] {
                    let mut s = format!("cs-row {t} 2 {left} {right} {width} {origin}");
                    for word in [a, b, 0x3f000000, a, b, 0x3f000000, a, b, 0x3f000000] {
                        write!(s, " {word}").unwrap();
                    }
                    cases.push(s);
                }
            }
        }
    }
    compare_chroma(cases);
}
