// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use super::*;
use jxl::jxl::frame_dimensions::FrameDimensions;
use jxl::jxl::image::Image3;
use jxl::jxl::render_pipeline::render_pipeline_stage::WriteToImage3FStage;
use jxl::jxl::render_pipeline::simple_render_pipeline::SimpleRenderPipelineBuilder;
use jxl::jxl::render_pipeline::{
    render_pipeline_stage::{RENDER_PIPELINE_X_OFFSET as OFFSET, Row, RowBuffers, RowStorage},
    stage_epf::{EpfStage, get_epf_stage},
};
use jxl::jxl::{image::Plane, loop_filter::LoopFilter};
pub(super) fn handles(kind: &str) -> bool {
    kind.starts_with("epf-")
}
pub(super) fn observe(kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    let pass = match number::<u32>(args) {
        0 => EpfStage::Zero,
        1 => EpfStage::One,
        2 => EpfStage::Two,
        _ => panic!("invalid pass"),
    };
    let mut lf = LoopFilter {
        epf_iters: number(args),
        ..LoopFilter::default()
    };
    lf.epf_pass0_sigma_scale = f32::from_bits(number(args));
    lf.epf_pass2_sigma_scale = f32::from_bits(number(args));
    lf.epf_border_sad_mul = f32::from_bits(number(args));
    for v in &mut lf.epf_channel_scale {
        *v = f32::from_bits(number(args));
    }
    lf.epf_pass1_zeroflush = f32::from_bits(number(args));
    lf.epf_pass2_zeroflush = f32::from_bits(number(args));
    let mut sigma = Plane::<f32>::create(128, 64).unwrap();
    if kind == "epf-factory" {
        let mut stage = get_epf_stage(&lf, &sigma, pass);
        let mut out = u32::from(stage.is_some()).to_string();
        if let Some(stage) = &mut stage {
            let s = stage.settings();
            write!(out, " {} {} {}", s.border_x, s.border_y, stage.get_name()).unwrap();
            for c in 0..5 {
                write!(out, " {}", stage.get_channel_mode(c) as i32).unwrap();
            }
            write!(
                out,
                " {} {}",
                status_code(&stage.is_initialized()),
                status_code(&stage.prepare_for_threads(2))
            )
            .unwrap();
        }
        return out;
    }
    if kind == "epf-simple" {
        for y in 0..64 {
            for x in 0..128 {
                sigma.row_mut(y)[x] = match (x + y) % 3 {
                    0 => -10.0,
                    1 => -3.0,
                    _ => -0.1,
                };
            }
        }
        let stage = get_epf_stage(&lf, &sigma, pass);
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
        return out;
    }
    let left: usize = number(args);
    let right: usize = number(args);
    let width: usize = number(args);
    let origin: usize = number(args);
    let xpos = number(args);
    let ypos = number(args);
    let values: [f32; 9] = std::array::from_fn(|_| f32::from_bits(number(args)));
    let sigmas: [f32; 3] = std::array::from_fn(|_| f32::from_bits(number(args)));
    for y in 0..64 {
        for x in 0..128 {
            sigma.row_mut(y)[x] = sigmas[(x + y) % 3];
        }
    }
    let len = origin + OFFSET + width + right + 5;
    let mut input: Vec<_> = (0..5)
        .map(|_| Plane::<f32>::create(len, 9).unwrap())
        .collect();
    let mut output: Vec<_> = (0..5)
        .map(|_| Plane::<f32>::create(len, 3).unwrap())
        .collect();
    for c in 0..5 {
        for y in 0..9 {
            for x in 0..len {
                input[c].row_mut(y)[x] = values[(c * 7 + y * 3 + x) % 9];
            }
        }
        for y in 0..3 {
            for x in 0..len {
                output[c].row_mut(y)[x] =
                    f32::from_bits(0x7fa00000 + (c * 4096 + y * 256 + x) as u32);
            }
        }
    }
    let border = 3 - pass as usize;
    let ir = (0..5)
        .map(|c| {
            (0..2 * border + 1)
                .map(|y| Row {
                    storage: RowStorage::Input,
                    channel: c,
                    y: y + 1,
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
                y: 1,
                x: origin,
            }]
        })
        .collect();
    let status = get_epf_stage(&lf, &sigma, pass).unwrap().process_row(
        &ir,
        &or,
        &mut RowBuffers {
            input: &mut input,
            output: &mut output,
        },
        left,
        right,
        width,
        xpos,
        ypos,
        17,
    );
    let mut out = status_code(&status).to_string();
    for p in &output {
        for y in 0..3 {
            for x in 0..len {
                write!(out, " {}", p.row(y)[x].to_bits()).unwrap();
            }
        }
    }
    out
}
fn params(pass: u8, iters: u32, lf: &LoopFilter) -> String {
    let mut s = format!("{pass} {iters}");
    for v in [
        lf.epf_pass0_sigma_scale,
        lf.epf_pass2_sigma_scale,
        lf.epf_border_sad_mul,
        lf.epf_channel_scale[0],
        lf.epf_channel_scale[1],
        lf.epf_channel_scale[2],
        lf.epf_pass1_zeroflush,
        lf.epf_pass2_zeroflush,
    ] {
        write!(s, " {}", v.to_bits()).unwrap();
    }
    s
}
#[test]
fn epf_factories_settings_channels_lifecycle_match_original() {
    compare_epf(
        (0..3)
            .flat_map(|p| {
                (0..4).map(move |i| format!("epf-factory {}", params(p, i, &LoopFilter::default())))
            })
            .collect(),
    );
}
#[test]
fn epf_all_passes_sads_extras_borders_and_float_words_match_original() {
    let mut cases = Vec::new();
    let mut seed = 937;
    let mut values = vec![
        [0u32; 9],
        [0x80000000; 9],
        [0x3f800000; 9],
        [
            0, 0x80000000, 1, 0x80000001, 0x00800000, 0x7f7fffff, 0xff7fffff, 0x3f800000,
            0xbf800000,
        ],
        [
            0x7fa12345, 0xffa54321, 0x7fc98765, 0xffc43210, 0x7f800000, 0xff800000, 0x3f000000,
            0x3f800000, 0x80000000,
        ],
    ];
    for _ in 0..12 {
        values.push(std::array::from_fn(|_| next(&mut seed) as u32));
    }
    let mut filters = vec![
        LoopFilter::default(),
        LoopFilter {
            epf_channel_scale: [0.; 3],
            epf_pass0_sigma_scale: 0.,
            epf_pass2_sigma_scale: 0.,
            ..LoopFilter::default()
        },
        LoopFilter {
            epf_channel_scale: [0.25, -0.5, 2.],
            epf_border_sad_mul: 0.3,
            epf_pass0_sigma_scale: 0.9,
            epf_pass2_sigma_scale: 3.7,
            ..LoopFilter::default()
        },
    ];
    filters.push(LoopFilter {
        epf_channel_scale: [
            f32::from_bits(0x7fa12345),
            f32::from_bits(0xffc98765),
            f32::INFINITY,
        ],
        ..LoopFilter::default()
    });
    for pass in 0..3 {
        for lf in &filters {
            for values in &values {
                for (left, right, width, origin, xpos, ypos) in [
                    (0, 0, 0, 0, 0, 0),
                    (1, 2, 0, 3, 23, 41),
                    (0, 0, 1, 0, 0, 0),
                    (2, 3, 17, 1, 9, 7),
                    (20, 7, 33, 0, 23, 8),
                    (0, 0, 65, 0, 7, 15),
                ] {
                    for sigmas in [
                        [-10f32, -4., -3.],
                        [f32::from_bits(0xc079ef80), -0.0001, -1.],
                        [
                            f32::from_bits(0x7fa12345),
                            f32::NEG_INFINITY,
                            f32::from_bits(0xffc98765),
                        ],
                    ] {
                        let mut s = format!(
                            "epf-row {} {left} {right} {width} {origin} {xpos} {ypos}",
                            params(pass, 3, lf)
                        );
                        for word in values {
                            write!(s, " {word}").unwrap();
                        }
                        for v in sigmas {
                            write!(s, " {}", v.to_bits()).unwrap();
                        }
                        cases.push(s);
                    }
                }
            }
        }
    }
    compare_epf(cases);
}

#[test]
fn epf_passes_simple_pipeline_mirror_edges_groups_and_repeated_passes_match_original() {
    let mut cases = Vec::new();
    for pass in 0..3 {
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
                    cases.push(format!(
                        "epf-simple {} {w} {h} 2 {mode} {reverse} 2",
                        params(pass, 3, &LoopFilter::default())
                    ));
                }
            }
        }
    }
    compare_epf(cases);
}

fn compare_epf(requests: Vec<String>) {
    let observations = original_observations(&requests);
    let mut failures = 0;
    for (request, cpp) in requests.iter().zip(observations) {
        let rust = super::observe(request);
        if !super::parity_eq(request, &rust, &cpp) {
            failures += 1;
            if failures <= 12 {
                let differences: Vec<_> = rust
                    .split_whitespace()
                    .zip(cpp.split_whitespace())
                    .enumerate()
                    .filter(|(_, (a, b))| a != b)
                    .take(10)
                    .collect();
                eprintln!("EPF mismatch {request}\n{differences:?}");
            }
        }
    }
    assert_eq!(failures, 0, "EPF exact mismatches");
    println!("{} C++/Rust differential cases matched", requests.len());
}

#[test]
fn epf_finite_active_weights_scale_rounding_and_threshold_neighbors_match_original() {
    let mut cases = Vec::new();
    let mut seed = 7891;
    let filters = [
        LoopFilter::default(),
        LoopFilter {
            epf_channel_scale: [0.01, 0.02, 0.03],
            epf_border_sad_mul: 0.67,
            epf_pass0_sigma_scale: 0.9,
            epf_pass2_sigma_scale: 6.5,
            ..LoopFilter::default()
        },
        LoopFilter {
            epf_channel_scale: [0.0001, 0.003, 0.2],
            epf_border_sad_mul: 1.9,
            epf_pass0_sigma_scale: f32::from_bits(0x3f666665),
            epf_pass2_sigma_scale: f32::from_bits(0x40d00001),
            epf_pass1_zeroflush: f32::INFINITY,
            epf_pass2_zeroflush: f32::from_bits(0x7fa12345),
            ..LoopFilter::default()
        },
    ];
    for pass in 0..3 {
        for lf in &filters {
            for _ in 0..8 {
                let values: [f32; 9] =
                    std::array::from_fn(|_| ((next(&mut seed) as i32 % 1024) as f32) / 131072.0);
                for (left, right, width, origin, xpos, ypos) in [
                    (0, 0, 1, 0, 0, 0),
                    (1, 1, 7, 2, 9, 1),
                    (2, 3, 8, 1, 9, 7),
                    (0, 0, 9, 0, 7, 8),
                    (3, 2, 31, 5, 23, 15),
                    (20, 7, 65, 0, 23, 17),
                ] {
                    for sigma in [
                        f32::from_bits(jxl::jxl::epf::MIN_SIGMA.to_bits() + 1),
                        jxl::jxl::epf::MIN_SIGMA,
                        f32::from_bits(jxl::jxl::epf::MIN_SIGMA.to_bits() - 1),
                        -0.001,
                    ] {
                        let mut s = format!(
                            "epf-row {} {left} {right} {width} {origin} {xpos} {ypos}",
                            params(pass, 3, lf)
                        );
                        for v in values {
                            write!(s, " {}", v.to_bits()).unwrap();
                        }
                        for _ in 0..3 {
                            write!(s, " {}", sigma.to_bits()).unwrap();
                        }
                        cases.push(s);
                    }
                }
            }
        }
    }
    compare_epf(cases);
}

#[test]
fn epf_subnormals_signed_zero_cancellation_and_extreme_finite_words_match_original() {
    let patterns: [[u32; 9]; 8] = [
        [
            0, 0x80000000, 1, 0x80000001, 2, 0x80000002, 3, 0x80000003, 0,
        ],
        [
            0x007fffff, 0x807fffff, 0x00800000, 0x80800000, 0x00800001, 0x80800001, 1, 0x80000001,
            0,
        ],
        [
            0x3f800000, 0xbf800000, 0x3f000000, 0xbf000000, 0, 0x80000000, 1, 0x80000001,
            0x3f800001,
        ],
        [
            0x7f7fffff, 0xff7fffff, 0x3f800000, 0xbf800000, 0, 0x80000000, 1, 0x80000001,
            0x00800000,
        ],
        [1; 9],
        [0x80000001; 9],
        [0; 9],
        [0x80000000; 9],
    ];
    let mut cases = Vec::new();
    for pass in 0..3 {
        for pattern in patterns {
            for scale in [
                [1., 1., 1.],
                [0., 0., 0.],
                [f32::from_bits(1), f32::MIN_POSITIVE, 0.5],
            ] {
                let lf = LoopFilter {
                    epf_channel_scale: scale,
                    ..LoopFilter::default()
                };
                for (left, right, width, origin, xpos, ypos) in [
                    (0, 0, 1, 0, 0, 0),
                    (1, 2, 7, 3, 23, 1),
                    (2, 2, 8, 1, 9, 7),
                    (20, 7, 17, 0, 23, 8),
                ] {
                    let mut s = format!(
                        "epf-row {} {left} {right} {width} {origin} {xpos} {ypos}",
                        params(pass, 3, &lf)
                    );
                    for word in pattern {
                        write!(s, " {word}").unwrap();
                    }
                    for v in [-1.0f32, -0.001, -3.0] {
                        write!(s, " {}", v.to_bits()).unwrap();
                    }
                    cases.push(s);
                }
            }
        }
    }
    compare_epf(cases);
}

#[test]
fn epf_every_filter_field_special_words_and_competing_operands_match_original() {
    let lf = LoopFilter::default();
    let base = [
        lf.epf_pass0_sigma_scale,
        lf.epf_pass2_sigma_scale,
        lf.epf_border_sad_mul,
        lf.epf_channel_scale[0],
        lf.epf_channel_scale[1],
        lf.epf_channel_scale[2],
        lf.epf_pass1_zeroflush,
        lf.epf_pass2_zeroflush,
    ]
    .map(f32::to_bits);
    let words = [
        0, 0x80000000, 1, 0x80000001, 0x00800000, 0x7f7fffff, 0xff7fffff, 0x7f800000, 0xff800000,
        0x7fc12345, 0xffc98765, 0x7fa12345, 0xffa98765,
    ];
    let pixels: [[u32; 9]; 2] = [
        [0x3f800000; 9],
        [
            0x7fa12345, 0xffa54321, 0x7fc98765, 0xffc43210, 0x7f800000, 0xff800000, 0x3f000000,
            0x3f800000, 0x80000000,
        ],
    ];
    let mut cases = Vec::new();
    for pass in 0..3 {
        for slot in 0..8 {
            for word in words {
                let mut filter = base;
                filter[slot] = word;
                for values in pixels {
                    for (left, right, width, origin, xpos, ypos) in
                        [(1, 2, 3, 3, 23, 41), (0, 0, 1, 0, 0, 0)]
                    {
                        let mut s = format!("epf-row {pass} 3");
                        for word in filter {
                            write!(s, " {word}").unwrap();
                        }
                        write!(s, " {left} {right} {width} {origin} {xpos} {ypos}").unwrap();
                        for word in values {
                            write!(s, " {word}").unwrap();
                        }
                        for _ in 0..3 {
                            write!(s, " {}", (-1.0f32).to_bits()).unwrap();
                        }
                        cases.push(s);
                    }
                }
            }
        }
    }
    compare_epf(cases);
}
