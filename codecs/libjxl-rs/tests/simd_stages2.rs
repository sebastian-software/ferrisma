// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! SIMD parity of the upsampling, noise, blending and splines render stages
//! of ADR 0004 Plan item 1 against the original built for the same single
//! Highway target (`JXL_CPP_ORACLE_SIMD`, `s2-*` requests of
//! tools/oracle/render_stages2_simd.cc). Never compared with the scalar
//! oracle. Words are compared bit for bit except that any NaN equals any NaN
//! (decision D8).
#![cfg(feature = "cpp-oracle-simd")]

mod simd_common;

use jxl::jxl::frame_dimensions::FrameDimensions;
use jxl::jxl::hwy::Target;
use jxl::jxl::image::{Image3, Plane};
use jxl::jxl::image_metadata::CustomTransformData;
use jxl::jxl::memory_manager_internal::MemoryManagerPtr;
use jxl::jxl::render_pipeline::render_pipeline::RenderPipelineBuilder;
use jxl::jxl::render_pipeline::render_pipeline_stage::{
    RENDER_PIPELINE_X_OFFSET as OFFSET, Row, RowBuffers, RowStorage, WriteToImage3FStage,
};
use simd_common::{check_rows, join, next, special_words, status, target};
use std::fmt::Write as _;

fn float(args: &mut std::str::SplitWhitespace<'_>) -> f32 {
    f32::from_bits(args.next().unwrap().parse().unwrap())
}
fn num(args: &mut std::str::SplitWhitespace<'_>) -> usize {
    args.next().unwrap().parse().unwrap()
}
fn words(args: &mut std::str::SplitWhitespace<'_>) -> Vec<f32> {
    let n = num(args);
    (0..n).map(|_| float(args)).collect()
}

/// Force the decoder's dispatch onto the SIMD target, so that the port's
/// public factories return the SIMD stages and planes take that target's
/// row padding, like the original's dispatch on the oracle's single target.
fn select_simd(t: Target) {
    jxl::jxl::hwy::set_decoder_simd_for_test(Some(true));
    assert_eq!(
        jxl::jxl::hwy::decoder_target().map(|d| d.name()),
        Some(t.name())
    );
}

/// `S2Weights`: nw == 0 keeps the defaults, otherwise weight i of table k is
/// `w[(i + 5 * k) % nw]`.
fn ups_weights(args: &mut std::str::SplitWhitespace<'_>) -> CustomTransformData {
    let mut factors = CustomTransformData::default();
    let w = words(args);
    if !w.is_empty() {
        for (k, table) in [
            &mut factors.upsampling2_weights[..],
            &mut factors.upsampling4_weights[..],
            &mut factors.upsampling8_weights[..],
        ]
        .into_iter()
        .enumerate()
        {
            for (i, v) in table.iter_mut().enumerate() {
                *v = w[(i + 5 * k) % w.len()];
            }
        }
    }
    factors
}

fn ups_out_len(width: usize, n: usize) -> usize {
    OFFSET + (width + 8) / 8 * 8 * n + 16
}

/// s2-ups as tools/oracle/render_stages2_simd.cc observes it.
fn s2_ups(request: &str) -> String {
    let mut args = request.split_whitespace();
    assert_eq!(args.next(), Some("s2-ups"));
    let c = num(&mut args);
    let shift = num(&mut args);
    let factors = ups_weights(&mut args);
    let width = num(&mut args);
    let w = words(&mut args);
    assert!(args.next().is_none(), "trailing request words");
    let mut stage = jxl::jxl::render_pipeline::stage_upsampling::get_upsampling_stage_in(
        MemoryManagerPtr::NATIVE,
        &factors,
        c,
        shift,
    )
    .unwrap();
    let n = 1usize << shift;
    let in_len = OFFSET + width + 40;
    let out_len = ups_out_len(width, n);
    let mut input = vec![Plane::<f32>::create(in_len, 5).unwrap()];
    let mut output = vec![Plane::<f32>::create(out_len, n).unwrap()];
    for y in 0..5 {
        for x in 0..in_len {
            input[0].row_mut(y)[x] = w[(y * 7 + x) % w.len()];
        }
    }
    for y in 0..n {
        output[0].row_mut(y)[..out_len].fill(f32::from_bits(0xc123_4567));
    }
    // RowInfo of c + 1 channels, only channel c holding rows, as the
    // request builds it; the planes are this test's channel 0.
    let mut input_rows = vec![vec![]; c + 1];
    let mut output_rows = vec![vec![]; c + 1];
    for y in 0..5 {
        input_rows[c].push(Row {
            storage: RowStorage::Input,
            channel: 0,
            y,
            x: 0,
        });
    }
    for y in 0..n {
        output_rows[c].push(Row {
            storage: RowStorage::Output,
            channel: 0,
            y,
            x: 0,
        });
    }
    let prepared = stage.prepare_for_threads(1);
    let s = stage.process_row(
        &input_rows,
        &output_rows,
        &mut RowBuffers {
            input: &mut input,
            output: &mut output,
        },
        0,
        0,
        width,
        3,
        5,
        0,
    );
    let mut out = format!("{} {}", status(&prepared), status(&s));
    for y in 0..n {
        for x in 0..out_len {
            write!(out, " {}", output[0].row(y)[x].to_bits()).unwrap();
        }
    }
    out
}

/// Words of an `s2-ups` observation the original computes from column
/// extrema its scratch never received in this call (AVX2 only; see
/// stage_upsampling_simd.rs): in the last 1024-sample chunk of length `len`,
/// lanes `j >= len` with `j + 4 >= RoundUpTo(len + 4, Lanes)`.
fn ups_defined(width: usize, n: usize, lanes: usize) -> Box<dyn Fn(usize) -> bool> {
    let out_len = ups_out_len(width, n);
    let mut undefined = vec![false; 2 + n * out_len];
    if width > 0 {
        let x0 = (width - 1) / 1024 * 1024;
        let len = width - x0;
        let written = (len + 4).div_ceil(lanes) * lanes;
        for j in len..len.div_ceil(lanes) * lanes {
            if j + 4 >= written {
                for oy in 0..n {
                    for k in 0..n {
                        undefined[2 + oy * out_len + OFFSET + (x0 + j) * n + k] = true;
                    }
                }
            }
        }
    }
    Box::new(move |i| !undefined[i])
}

/// Weight sets: the defaults, image-like kernels, special words.
fn ups_weight_sets() -> Vec<Vec<u32>> {
    let mut seed = 0x51;
    let mut sets = vec![vec![]];
    sets.push(
        (0..37)
            .map(|_| ((next(&mut seed) % 2001) as f32 / 4000.0 - 0.1).to_bits())
            .collect(),
    );
    sets.push(special_words());
    sets.push((0..23).map(|_| next(&mut seed) as u32).collect());
    sets
}

/// Sample sets: special words, image-like samples, random bit patterns.
fn sample_sets(seed: u64) -> Vec<Vec<u32>> {
    let mut seed = seed | 1;
    vec![
        special_words(),
        vec![0x3f80_0000, 0x8000_0000, 0x3f00_0000],
        (0..61)
            .map(|_| ((next(&mut seed) % 1_000_001) as f32 / 1_000_000.0 - 0.05).to_bits())
            .collect(),
        (0..53).map(|_| next(&mut seed) as u32).collect(),
    ]
}

#[test]
fn simd_upsampling_rows_match_the_original_target() {
    let t = target();
    select_simd(t);
    let widths = [
        0usize, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 13, 15, 16, 17, 24, 31, 33, 64, 67, 1023, 1024,
        1025, 1029, 1031, 2050,
    ];
    let mut requests = Vec::new();
    let mut defined = Vec::new();
    for shift in 1..=3usize {
        for (wi, weights) in ups_weight_sets().iter().enumerate() {
            for (si, samples) in sample_sets(shift as u64 * 7 + 3).iter().enumerate() {
                for &width in &widths {
                    // The wide rows with every weight set but fewer sample
                    // sets, to bound the request size.
                    if width > 100 && (si + wi) % 2 == 1 {
                        continue;
                    }
                    let c = (width + si) % 3;
                    requests.push(format!(
                        "s2-ups {c} {shift} {} {} {width} {} {}",
                        weights.len(),
                        join(weights.iter().copied()),
                        samples.len(),
                        join(samples.iter().copied())
                    ));
                    defined.push((width, 1usize << shift));
                }
            }
        }
    }
    let lanes = t.lanes();
    let index = |r: &str| requests.iter().position(|q| q == r).unwrap();
    check_rows(
        "s2-ups",
        &requests,
        |r| {
            let (width, n) = defined[index(r)];
            (s2_ups(r), ups_defined(width, n, lanes))
        },
        |i| i > 1,
    );
}

/// s2-ups-pipe as tools/oracle/render_stages2_simd.cc observes it.
fn s2_ups_pipe(request: &str) -> String {
    let mut args = request.split_whitespace();
    assert_eq!(args.next(), Some("s2-ups-pipe"));
    let simple = num(&mut args) != 0;
    let w = num(&mut args);
    let h = num(&mut args);
    let shift = num(&mut args);
    let c = num(&mut args);
    let factors = ups_weights(&mut args);
    let samples = words(&mut args);
    assert!(args.next().is_none(), "trailing request words");
    let mut dim = FrameDimensions::default();
    dim.set(w, h, 0, 0, 0, true, 1);
    let mut image = Image3::default();
    let mut out = String::new();
    {
        let mut builder = RenderPipelineBuilder::new(3);
        if simple {
            builder.use_simple_implementation();
        }
        builder
            .add_stage(
                jxl::jxl::render_pipeline::stage_upsampling::get_upsampling_stage_in(
                    MemoryManagerPtr::NATIVE,
                    &factors,
                    c,
                    shift,
                ),
            )
            .unwrap();
        builder
            .add_stage(Some(Box::new(WriteToImage3FStage::new(&mut image))))
            .unwrap();
        let mut pipeline = builder.finalize(dim).unwrap();
        let mut s = pipeline.prepare_for_threads(1, true);
        write!(out, "{}", status(&s)).unwrap();
        let mut g = 0;
        while s.is_ok() && g < dim.num_groups {
            let mut input = pipeline.get_input_buffers(g, 0);
            for cc in 0..3 {
                let (plane, r) = input.get_buffer_mut(cc);
                for y in 0..r.ysize() {
                    for x in 0..r.xsize() {
                        plane.row_mut(r.y0() + y)[r.x0() + x] =
                            samples[(cc * 31 + x * 17 + y * 9) % samples.len()];
                    }
                }
            }
            s = input.done();
            write!(out, " {}", status(&s)).unwrap();
            g += 1;
        }
    }
    write!(out, " O {} {}", image.xsize(), image.ysize()).unwrap();
    for cc in 0..3 {
        for y in 0..image.ysize() {
            for x in 0..image.xsize() {
                write!(out, " {}", image.plane_row(cc, y)[x].to_bits()).unwrap();
            }
        }
    }
    out
}

/// The stage inside the simple and the low-memory pipeline: the pipelines
/// mirror the two-sample border (`Settings::Symmetric(shift, 2)`) at every
/// image edge and hand the stage group rows, so the edges, group seams and
/// the 1024-sample chunks are all exercised.
#[test]
fn simd_upsampling_in_both_pipelines_matches_the_original_target() {
    let t = target();
    select_simd(t);
    let mut requests = Vec::new();
    for simple in [1, 0] {
        for (w, h) in [
            (1, 1),
            (2, 3),
            (5, 2),
            (17, 9),
            (65, 3),
            (257, 5),
            (8, 67),
            (1030, 2),
        ] {
            for shift in 1..=3usize {
                for c in 0..3 {
                    for (wi, weights) in ups_weight_sets().iter().enumerate() {
                        if (w * h > 2000 || shift == 3) && wi % 2 == 1 {
                            continue;
                        }
                        let samples = &sample_sets(w as u64 + 11)[(c + wi) % 4];
                        requests.push(format!(
                            "s2-ups-pipe {simple} {w} {h} {shift} {c} {} {} {} {}",
                            weights.len(),
                            join(weights.iter().copied()),
                            samples.len(),
                            join(samples.iter().copied())
                        ));
                    }
                }
            }
        }
    }
    check_rows(
        "s2-ups-pipe",
        &requests,
        |r| (s2_ups_pipe(r), Box::new(|_| true)),
        |i| i > 0,
    );
}
