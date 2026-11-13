// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! SIMD parity of the render stages of ADR 0004 Plan item 1 (XYB,
//! FromLinear, output conversion, ...) against the original built for the
//! same single Highway target (`JXL_CPP_ORACLE_SIMD`, `rs-*` requests of
//! tools/oracle/render_stages_simd.cc). Never compared with the scalar
//! oracle. Words are compared bit for bit except that any NaN equals any NaN
//! (decision D8).
#![cfg(feature = "cpp-oracle-simd")]

mod simd_common;

use jxl::jxl::color_encoding::ColorSpace;
use jxl::jxl::dec_xyb::{OpsinParams, OutputEncodingInfo};
use jxl::jxl::hwy::Target;
use jxl::jxl::image::Plane;
use jxl::jxl::render_pipeline::render_pipeline_stage::{
    RenderPipelineStage, Row, RowBuffers, RowStorage,
};
use simd_common::{check_rows, join, next, special_words, status, target};
use std::fmt::Write as _;

fn float(args: &mut std::str::SplitWhitespace<'_>) -> f32 {
    f32::from_bits(args.next().unwrap().parse().unwrap())
}
fn num(args: &mut std::str::SplitWhitespace<'_>) -> usize {
    args.next().unwrap().parse().unwrap()
}

/// The rows of an in-place request, as `RSInPlace` fills and prints them:
/// five channel rows of `origin + width + 16` samples, `words[(c * 7 + x) %
/// n]`, the stage's row at sample `origin`.
fn in_place(
    stage: &mut dyn RenderPipelineStage,
    args: &mut std::str::SplitWhitespace<'_>,
) -> String {
    let width = num(args);
    let origin = num(args);
    let n = num(args);
    let words: Vec<f32> = (0..n).map(|_| float(args)).collect();
    assert!(args.next().is_none(), "trailing request words");
    let len = origin + width + 16;
    let mut planes: Vec<_> = (0..5)
        .map(|c| {
            // Room for whole-vector reads past the row, as the source's
            // BytesPerRow padding.
            let mut p = Plane::<f32>::create(len + 32, 1).unwrap();
            for x in 0..len {
                p.row_mut(0)[x] = words[(c * 7 + x) % n];
            }
            p
        })
        .collect();
    let rows = (0..5)
        .map(|c| {
            vec![Row {
                storage: RowStorage::Input,
                channel: c,
                y: 0,
                x: origin,
            }]
        })
        .collect();
    let s = stage.process_row(
        &rows,
        &vec![vec![]; 5],
        &mut RowBuffers {
            input: &mut planes,
            output: &mut [],
        },
        0,
        0,
        width,
        19,
        23,
        0,
    );
    let mut out = status(&s).to_string();
    for p in &planes {
        for x in 0..len {
            write!(out, " {}", p.row(0)[x].to_bits()).unwrap();
        }
    }
    out
}

/// rs-xyb as tools/oracle/render_stages_simd.cc observes it.
fn rs_xyb(request: &str, t: Target) -> String {
    let mut args = request.split_whitespace();
    assert_eq!(args.next(), Some("rs-xyb"));
    let mut params = OpsinParams::default();
    if num(&mut args) == 0 {
        params.init(float(&mut args));
    } else {
        for v in &mut params.inverse_opsin_matrix {
            *v = float(&mut args);
        }
        for v in &mut params.opsin_biases {
            *v = float(&mut args);
        }
        for v in &mut params.opsin_biases_cbrt {
            *v = float(&mut args);
        }
        for v in &mut params.quant_biases {
            *v = float(&mut args);
        }
    }
    let mut info = OutputEncodingInfo {
        opsin_params: params,
        ..Default::default()
    };
    info.color_encoding
        .set_color_space(ColorSpace(num(&mut args) as u32));
    let mut stage = jxl::jxl::render_pipeline::stage_xyb_simd::get_xyb_stage_simd(&info, t);
    in_place(stage.as_mut(), &mut args)
}

/// Row widths that exercise whole vectors and remainders of both targets,
/// and row origins on and off a vector boundary.
const WIDTHS: &[usize] = &[
    0, 1, 2, 3, 4, 5, 7, 8, 9, 12, 15, 16, 17, 24, 31, 33, 64, 67,
];
const ORIGINS: &[usize] = &[0, 1, 3, 4, 8, 13, 32];

/// Row contents: the adversarial words, image-like samples of the stage's
/// domain `lo..hi`, its `thresholds` with their neighbor words, and random
/// bit patterns.
fn word_sets(lo: f32, hi: f32, thresholds: &[f32], seed: u64) -> Vec<Vec<u32>> {
    let mut seed = seed | 1;
    let mut sets = vec![
        special_words(),
        vec![0],
        vec![0x3f80_0000, 0x8000_0000, 0x3f00_0000],
    ];
    for count in [37, 61] {
        sets.push(
            (0..count)
                .map(|_| {
                    let u = (next(&mut seed) % 1_000_001) as f32 / 1_000_000.0;
                    (lo + (hi - lo) * u).to_bits()
                })
                .collect(),
        );
    }
    let mut edges = Vec::new();
    for &t in thresholds {
        for v in [t, -t] {
            let b = v.to_bits();
            edges.extend([b.wrapping_sub(1), b, b.wrapping_add(1)]);
        }
    }
    if !edges.is_empty() {
        sets.push(edges);
    }
    sets.push((0..53).map(|_| next(&mut seed) as u32).collect());
    sets
}

#[test]
fn simd_xyb_rows_match_the_original_target() {
    let t = target();
    let mut params = vec![
        format!("0 {}", 255.0f32.to_bits()),
        format!("0 {}", 1.0f32.to_bits()),
        format!("0 {}", 10000.0f32.to_bits()),
        format!("0 {}", 0.5f32.to_bits()),
    ];
    // Custom parameters whose four stored copies differ, so that each lane's
    // LoadDup128 copy is observable, plus special words.
    let mut seed = 77;
    for k in 0..3 {
        let mut words = Vec::new();
        for i in 0..48 {
            words.push(match k {
                0 => ((next(&mut seed) % 4001) as f32 / 1000.0 - 2.0).to_bits(),
                1 => special_words()[(i * 5 + 3) % 79],
                _ => next(&mut seed) as u32,
            });
        }
        params.push(format!("1 {}", join(words)));
    }
    let sets = word_sets(-0.05, 1.0, &[0.0, 0.0037930732, 0.155_954_2], 4);
    let mut requests = Vec::new();
    for p in &params {
        for space in [0, 2] {
            for words in &sets {
                for &width in WIDTHS {
                    for &origin in ORIGINS {
                        requests.push(format!(
                            "rs-xyb {p} {space} {width} {origin} {} {}",
                            words.len(),
                            join(words.iter().copied())
                        ));
                    }
                }
            }
        }
    }
    check_rows(
        "rs-xyb",
        &requests,
        |r| (rs_xyb(r, t), Box::new(|_| true)),
        |i| i > 0,
    );
}
