// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! SIMD parity of the render stages of ADR 0004 Plan item 1 (XYB,
//! FromLinear, output conversion, ...) against the original built for the
//! same single Highway target (`JXL_CPP_ORACLE_SIMD`, `rs-*` requests of
//! tools/oracle/render_stages_simd.cc). Never compared with the scalar
//! oracle. Words are compared bit for bit except that any NaN equals any NaN
//! (decision D8).
#![cfg(feature = "cpp-oracle-simd")]

#[path = "common/output_stage.rs"]
#[allow(dead_code, reason = "only the wo-run observer is used here")]
mod rust_observer;
mod simd_common;

use jxl::jxl::color_encoding::{ColorSpace, TransferFunction};
use jxl::jxl::dec_xyb::{OpsinParams, OutputEncodingInfo};
use jxl::jxl::hwy::Target;
use jxl::jxl::image::Plane;
use jxl::jxl::render_pipeline::render_pipeline_stage::{
    RENDER_PIPELINE_X_OFFSET as OFFSET, RenderPipelineStage, Row, RowBuffers, RowStorage,
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
/// five channel rows of `origin + kRenderPipelineXOffset + width + 16`
/// samples, `words[(c * 7 + x) % n]`, the stage's row at sample `origin`
/// (`get_input_row` adds the offset, as `GetInputRow` does).
fn in_place(
    stage: &mut dyn RenderPipelineStage,
    args: &mut std::str::SplitWhitespace<'_>,
) -> String {
    let width = num(args);
    let origin = num(args);
    let n = num(args);
    let words: Vec<f32> = (0..n).map(|_| float(args)).collect();
    assert!(args.next().is_none(), "trailing request words");
    let len = origin + OFFSET + width + 16;
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

/// rs-fl as tools/oracle/render_stages_simd.cc observes it.
fn rs_fl(request: &str, t: Target) -> String {
    let mut args = request.split_whitespace();
    assert_eq!(args.next(), Some("rs-fl"));
    let tf = num(&mut args) as u32;
    let have_gamma = num(&mut args) != 0;
    let gamma = num(&mut args) as u32;
    let mut info = OutputEncodingInfo::default();
    info.color_encoding
        .tf_mut()
        .set_transfer_function(TransferFunction(tf));
    info.color_encoding.tf_mut().have_gamma = have_gamma;
    info.color_encoding.tf_mut().gamma = gamma;
    info.orig_intensity_target = float(&mut args);
    info.desired_intensity_target = float(&mut args);
    info.inverse_gamma = float(&mut args);
    for l in &mut info.luminances {
        *l = float(&mut args);
    }
    let mut stage =
        jxl::jxl::render_pipeline::stage_from_linear_simd::get_from_linear_stage_simd(&info, t)
            .expect("a valid target encoding");
    in_place(stage.as_mut(), &mut args)
}

#[test]
fn simd_from_linear_rows_of_all_transfer_functions_match_the_original_target() {
    let t = target();
    let bits = |v: f32| v.to_bits();
    let srgb_luminances = [0.2126f32, 0.7152, 0.0722];
    let bt2100_luminances = [0.2627f32, 0.678, 0.0593];
    // tf have_gamma gamma orig_it desired_it inverse_gamma lum0 lum1 lum2
    let mut variants = Vec::new();
    let mut push = |tf: u32, have_gamma: u32, gamma: u32, it: [f32; 3], lum: [f32; 3]| {
        variants.push(format!(
            "{tf} {have_gamma} {gamma} {} {} {} {}",
            bits(it[0]),
            bits(it[1]),
            bits(it[2]),
            join(lum.map(f32::to_bits))
        ));
    };
    // Linear, sRGB and BT.709.
    for tf in [8, 13, 1] {
        push(tf, 0, 0, [255.0, 255.0, 1.0], srgb_luminances);
    }
    // PQ: the original intensity target scales the input.
    for it in [10000.0, 1000.0, 255.0, 203.0, 4000.0] {
        push(16, 0, 0, [it, 255.0, 1.0], bt2100_luminances);
    }
    // HLG: the OOTF exponent follows the desired intensity target (1000 nits
    // gives none) and the primaries' luminances.
    for desired in [1000.0, 300.0, 100.0, 2000.0, 1001.0] {
        for lum in [srgb_luminances, bt2100_luminances] {
            push(18, 0, 0, [1000.0, desired, 1.0], lum);
        }
    }
    // DCI and explicit gamma: FastPowf with the inverse gamma.
    push(17, 0, 0, [255.0, 255.0, 1.0 / 2.6], srgb_luminances);
    for inverse_gamma in [1.0 / 2.2, 0.5, 2.2, 1.0, 1.0 / 1.8] {
        push(
            2,
            1,
            4_545_455,
            [255.0, 255.0, inverse_gamma],
            srgb_luminances,
        );
    }
    let sets = word_sets(
        -0.1,
        1.3,
        &[0.0031308, 1e-4, 1.0 / 12.0, 0.018, 1e-5, 0.5, 1.0],
        12,
    );
    let mut requests = Vec::new();
    for v in &variants {
        for words in &sets {
            for &width in WIDTHS {
                for &origin in ORIGINS {
                    requests.push(format!(
                        "rs-fl {v} {width} {origin} {} {}",
                        words.len(),
                        join(words.iter().copied())
                    ));
                }
            }
        }
    }
    check_rows(
        "rs-fl",
        &requests,
        |r| (rs_fl(r, t), Box::new(|_| true)),
        |i| i > 0,
    );
}

/// WriteToOutputStage through the existing `wo-run` observer
/// (tools/oracle/output_stage.cc, tests/common/output_stage.rs), with the
/// default memory manager so that only the output bytes (buffers and
/// callback pixels) are observed, never the scratch blocks' padding lanes.
/// The port's factory follows `hwy::decoder_target()`, here the SIMD target.
#[test]
fn simd_output_conversion_rows_match_the_original_target() {
    let t = target();
    jxl::jxl::hwy::set_decoder_simd_for_test(Some(true));
    assert_eq!(
        jxl::jxl::hwy::decoder_target().map(|d| d.name()),
        Some(t.name())
    );
    let mut seed = 31;
    let mut values: Vec<[u32; 9]> = vec![
        [
            0,
            0x8000_0000,
            0x3f80_0000,
            0x3f00_0000,
            0x7fc0_0000,
            0xffa5_4321,
            0x7f80_0000,
            0xff80_0000,
            0x0000_0001,
        ],
        [
            0x3eff_ffff,
            0x3f00_0001,
            0x4f80_0000,
            0xbf80_0000,
            0x3f7f_ffff,
            0x3f80_0001,
            0x7f7f_ffff,
            0x3b80_0000,
            0x3b7f_ffff,
        ],
        // binary16 rounding, subnormal and overflow boundaries.
        [
            0x3380_0000,
            0x3300_0000,
            0x3880_0000,
            0x387f_ffff,
            0x477f_e000,
            0x477f_f000,
            0x4780_0000,
            0x3f80_1000,
            0x3f80_3000,
        ],
    ];
    for _ in 0..3 {
        values.push(std::array::from_fn(|_| {
            ((next(&mut seed) % 1_200_001) as f32 / 1_000_000.0 - 0.1).to_bits()
        }));
    }
    values.push(std::array::from_fn(|_| next(&mut seed) as u32));
    let alpha_values: [u32; 9] = [
        0x3f80_0000,
        0x3f00_0000,
        0,
        0x8000_0000,
        0x3380_0000,
        0x7fc0_0000,
        0x3dcc_cccd,
        0x3f7f_ffff,
        0x2000_0000,
    ];
    // (data type, bits): FLOAT, UINT8, UINT16, FLOAT16.
    let types = [(2, 8), (2, 5), (3, 16), (3, 10), (5, 16), (0, 32)];
    let mut requests = Vec::new();
    for &(kind, bits) in &types {
        for n in 1..=4u32 {
            for (orientation, width, endian) in [
                (1, 17usize, 0),
                (2, 9, 2),
                (3, 33, 0),
                (5, 8, 0),
                (1, 1030, 2),
                (1, 5, 1),
            ] {
                let height = 3;
                for (has_alpha, unpremul) in [(0, 0), (1, 0), (1, 1)] {
                    for (vi, v) in values.iter().enumerate() {
                        if width > 64 && vi % 3 != 0 {
                            continue;
                        }
                        let v = if has_alpha == 1 && vi % 2 == 1 {
                            alpha_values
                        } else {
                            *v
                        };
                        let mut actions = vec!["0 1".to_owned()];
                        for y in 0..height {
                            actions.push(format!("1 0 0 {width} 0 {y} 0"));
                        }
                        // A row that starts inside the image (dither phase,
                        // flip offsets) and one past its right edge.
                        actions.push(format!("1 0 0 {} 3 1 0", width.saturating_sub(3).max(1)));
                        actions.push(format!("1 0 0 4 {width} 2 0"));
                        for mode in [0, 1] {
                            requests.push(format!(
                                "wo-run {width} {height} {has_alpha} {unpremul} 3 {orientation} 1 1 \
                                 {kind} {bits} {n} {mode} 0 {endian} \
                                 {kind} {bits} 1 0 3 {endian} \
                                 {} {} {}",
                                actions.len(),
                                actions.join(" "),
                                join(v)
                            ));
                        }
                    }
                }
            }
        }
    }
    check_rows(
        "wo-run (SIMD)",
        &requests,
        |r| {
            let mut args = r.split_whitespace();
            let kind = args.next().unwrap();
            let out = rust_observer::observe(kind, &mut args);
            assert!(args.next().is_none());
            (out, Box::new(|_| true))
        },
        |_| false,
    );
}
