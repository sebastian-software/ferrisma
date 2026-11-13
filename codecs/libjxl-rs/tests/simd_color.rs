// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! SIMD parity of the color stages of ADR 0004 Plan item 1 (ToLinear, tone
//! mapping, CMS) against the original built for the same single Highway
//! target (`JXL_CPP_ORACLE_SIMD`, `cl-*` requests of
//! tools/oracle/color_stages_simd.cc). Never compared with the scalar
//! oracle. Words are compared bit for bit except that any NaN equals any NaN
//! (decision D8). Each family also runs the scalar path over the same
//! requests and requires that it differs somewhere, so that the requests can
//! tell the SIMD path from the scalar one.
#![cfg(feature = "cpp-oracle-simd")]

mod simd_common;

use jxl::jxl::color_encoding::TransferFunction;
use jxl::jxl::dec_xyb::OutputEncodingInfo;
use jxl::jxl::hwy::{decoder_target, set_decoder_simd_for_test};
use jxl::jxl::image::Plane;
use jxl::jxl::render_pipeline::render_pipeline_stage::{
    RENDER_PIPELINE_X_OFFSET as OFFSET, RenderPipelineStage, Row, RowBuffers, RowStorage,
};
use simd_common::{join, next, oracle, same, special_words, status, target};
use std::fmt::Write as _;
use std::sync::Mutex;

/// The decoder's target override is process-wide; the tests of this binary
/// switch it one at a time.
static DISPATCH: Mutex<()> = Mutex::new(());

fn float(args: &mut std::str::SplitWhitespace<'_>) -> f32 {
    f32::from_bits(args.next().unwrap().parse().unwrap())
}
fn num(args: &mut std::str::SplitWhitespace<'_>) -> usize {
    args.next().unwrap().parse().unwrap()
}

/// The rows of an in-place request, as `CLInPlace` fills and prints them:
/// five channel rows of `origin + kRenderPipelineXOffset + width + 16`
/// samples, `words[(c * 7 + x) % n]`, the stage's row at sample `origin`
/// (`get_input_row` adds the offset, as `GetInputRow` does).
fn in_place(
    stage: &mut dyn RenderPipelineStage,
    mut args: std::str::SplitWhitespace<'_>,
) -> String {
    let width = num(&mut args);
    let origin = num(&mut args);
    let n = num(&mut args);
    let words: Vec<f32> = (0..n).map(|_| float(&mut args)).collect();
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

/// Compare every request's SIMD observation (`simd = true`) with the
/// original target's, require that the scalar observation differs from it
/// in at least one request, and print the counts.
fn check_family(family: &str, requests: &[String], observe: impl Fn(&str, bool) -> String) {
    let observed = oracle(requests);
    let mut words = 0;
    let mut scalar_differs = 0;
    for (request, cpp) in requests.iter().zip(&observed) {
        let rust = observe(request, true);
        words += same(&rust, cpp, |i| i > 0, |_| true)
            .unwrap_or_else(|e| panic!("{family}: {e}\nrequest: {request}"));
        if same(&observe(request, false), cpp, |i| i > 0, |_| true).is_err() {
            scalar_differs += 1;
        }
    }
    println!(
        "{family} on {}: {} requests, {words} words matched; the scalar path differs in {scalar_differs}",
        target().name(),
        requests.len()
    );
    assert!(
        scalar_differs > 0,
        "{family}: the requests do not tell the SIMD path from the scalar one"
    );
}

/// Select the SIMD target (`true`) or the scalar path for the factories.
fn select(simd: bool) {
    set_decoder_simd_for_test(Some(simd));
    assert_eq!(
        decoder_target().map(|t| t.name()),
        simd.then(|| target().name())
    );
}

fn transfer(
    info: &mut jxl::jxl::color_encoding_internal::ColorEncoding,
    args: &mut std::str::SplitWhitespace<'_>,
) {
    let tf = num(args) as u32;
    let have_gamma = num(args) != 0;
    let gamma = num(args) as u32;
    info.tf_mut().set_transfer_function(TransferFunction(tf));
    info.tf_mut().have_gamma = have_gamma;
    info.tf_mut().gamma = gamma;
}

/// cl-tl as tools/oracle/color_stages_simd.cc observes it, through the
/// public factory `get_to_linear_stage` with the given selection.
fn cl_tl(request: &str, simd: bool) -> String {
    let mut args = request.split_whitespace();
    assert_eq!(args.next(), Some("cl-tl"));
    let mut info = OutputEncodingInfo::default();
    transfer(&mut info.color_encoding, &mut args);
    info.orig_intensity_target = float(&mut args);
    info.desired_intensity_target = float(&mut args);
    info.inverse_gamma = float(&mut args);
    for l in &mut info.luminances {
        *l = float(&mut args);
    }
    select(simd);
    let mut stage = jxl::jxl::render_pipeline::stage_to_linear::get_to_linear_stage(&info);
    in_place(stage.as_mut(), args)
}

const SRGB_LUMINANCES: [f32; 3] = [0.2126, 0.7152, 0.0722];
const BT2100_LUMINANCES: [f32; 3] = [0.2627, 0.678, 0.0593];

#[test]
fn simd_to_linear_rows_of_all_transfer_functions_match_the_original_target() {
    let _guard = DISPATCH.lock().unwrap_or_else(|e| e.into_inner());
    let bits = |v: f32| v.to_bits();
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
    // Linear, sRGB and BT.709; an unknown transfer function without gamma
    // selects OpInvalid.
    for tf in [8, 13, 1, 2] {
        push(tf, 0, 0, [255.0, 255.0, 1.0], SRGB_LUMINANCES);
    }
    // PQ: the original intensity target scales the output.
    for it in [10000.0, 1000.0, 255.0, 203.0, 4000.0] {
        push(16, 0, 0, [it, 255.0, 1.0], BT2100_LUMINANCES);
    }
    // HLG: the inverse OOTF exponent follows the original intensity target
    // (1000 nits gives none) and the primaries' luminances.
    for orig in [1000.0, 300.0, 100.0, 2000.0, 1001.0] {
        for lum in [SRGB_LUMINANCES, BT2100_LUMINANCES] {
            push(18, 0, 0, [orig, 255.0, 1.0], lum);
        }
    }
    // DCI and explicit gamma: FastPowf with 1 / inverse gamma.
    push(17, 0, 0, [255.0, 255.0, 1.0 / 2.6], SRGB_LUMINANCES);
    for inverse_gamma in [1.0 / 2.2, 0.5, 2.2, 1.0, 1.0 / 1.8] {
        push(
            2,
            1,
            4_545_455,
            [255.0, 255.0, inverse_gamma],
            SRGB_LUMINANCES,
        );
    }
    let sets = word_sets(-0.1, 1.3, &[0.04045, 0.081, 0.5, 1e-5, 1.0, 1.0 / 12.0], 21);
    let mut requests = Vec::new();
    for v in &variants {
        for words in &sets {
            for &width in WIDTHS {
                for &origin in ORIGINS {
                    requests.push(format!(
                        "cl-tl {v} {width} {origin} {} {}",
                        words.len(),
                        join(words.iter().copied())
                    ));
                }
            }
        }
    }
    check_family("cl-tl", &requests, cl_tl);
    set_decoder_simd_for_test(None);
}
