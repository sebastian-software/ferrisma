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

use jxl::jxl::base::rect::Rect;
use jxl::jxl::chroma_from_luma::ColorCorrelation;
use jxl::jxl::dec_bit_reader::BitReader;
use jxl::jxl::frame_dimensions::FrameDimensions;
use jxl::jxl::hwy::Target;
use jxl::jxl::image::{Image3, Plane};
use jxl::jxl::image_metadata::CustomTransformData;
use jxl::jxl::memory_manager_internal::MemoryManagerPtr;
use jxl::jxl::noise::NoiseParams;
use jxl::jxl::render_pipeline::render_pipeline::RenderPipelineBuilder;
use jxl::jxl::render_pipeline::render_pipeline_stage::{
    RENDER_PIPELINE_X_OFFSET as OFFSET, Row, RowBuffers, RowStorage, WriteToImage3FStage,
};
use jxl::jxl::render_pipeline::stage_noise::{get_add_noise_stage, get_convolve_noise_stage};
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
    let stage = jxl::jxl::render_pipeline::stage_upsampling::get_upsampling_stage_in(
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

/// `S2Correlation`: base correlations from binary16 words through
/// `ColorCorrelation::decode_dc` (65535 keeps the default).
fn correlation(args: &mut std::str::SplitWhitespace<'_>) -> ColorCorrelation {
    let x16 = num(args) as u64;
    let b16 = num(args) as u64;
    let mut cc = ColorCorrelation::default();
    if x16 != 65535 {
        let bits = (x16 << 3) | (b16 << 19) | (128 << 35) | (128 << 43);
        let bytes = bits.to_le_bytes();
        let mut br = BitReader::new(&bytes);
        cc.decode_dc(&mut br).unwrap();
        br.close().unwrap();
    }
    cc
}

/// One-row planes for `channels` channels of `rows` rows each.
fn row_planes(channels: usize, rows: usize, len: usize) -> Vec<Plane<f32>> {
    (0..channels)
        .map(|_| Plane::<f32>::create(len + 32, rows).unwrap())
        .collect()
}
fn rows_of(storage: RowStorage, channels: usize, rows: usize, x: usize) -> Vec<Vec<Row>> {
    (0..channels)
        .map(|channel| {
            (0..rows)
                .map(|y| Row {
                    storage,
                    channel,
                    y,
                    x,
                })
                .collect()
        })
        .collect()
}

/// s2-noise-add as tools/oracle/render_stages2_simd.cc observes it.
fn s2_noise_add(request: &str) -> String {
    let mut args = request.split_whitespace();
    assert_eq!(args.next(), Some("s2-noise-add"));
    let first_c = num(&mut args);
    let origin = num(&mut args);
    let width = num(&mut args);
    let mut params = NoiseParams::default();
    for v in &mut params.lut {
        *v = float(&mut args);
    }
    let cc = correlation(&mut args);
    let w = words(&mut args);
    assert!(args.next().is_none(), "trailing request words");
    let channels = first_c + 3;
    let len = origin + OFFSET + width + 16;
    let mut planes = row_planes(channels, 1, len);
    for (c, p) in planes.iter_mut().enumerate() {
        for x in 0..len {
            p.row_mut(0)[x] = w[(c * 7 + x) % w.len()];
        }
    }
    let stage = get_add_noise_stage(&params, &cc, first_c);
    let s = stage.process_row(
        &rows_of(RowStorage::Input, channels, 1, origin),
        &vec![vec![]; channels],
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

/// s2-noise-conv as tools/oracle/render_stages2_simd.cc observes it.
fn s2_noise_conv(request: &str) -> String {
    let mut args = request.split_whitespace();
    assert_eq!(args.next(), Some("s2-noise-conv"));
    let first_c = num(&mut args);
    let origin = num(&mut args);
    let width = num(&mut args);
    let left = num(&mut args);
    let right = num(&mut args);
    let w = words(&mut args);
    assert!(args.next().is_none(), "trailing request words");
    let channels = first_c + 3;
    let len = origin + OFFSET + width + right + 16;
    let mut input = row_planes(channels, 5, len);
    let mut output = row_planes(channels, 1, len);
    for c in first_c..channels {
        for y in 0..5 {
            for x in 0..len {
                input[c].row_mut(y)[x] = w[(c * 7 + y * 3 + x) % w.len()];
            }
        }
        output[c].row_mut(0)[..len].fill(f32::from_bits(0xc123_4567));
    }
    let stage = get_convolve_noise_stage(first_c);
    let s = stage.process_row(
        &rows_of(RowStorage::Input, channels, 5, origin),
        &rows_of(RowStorage::Output, channels, 1, origin),
        &mut RowBuffers {
            input: &mut input,
            output: &mut output,
        },
        left,
        right,
        width,
        19,
        23,
        0,
    );
    let mut out = status(&s).to_string();
    for p in &output[first_c..] {
        for x in 0..len {
            write!(out, " {}", p.row(0)[x].to_bits()).unwrap();
        }
    }
    out
}

/// s2-noise-random as tools/oracle/render_stages2_simd.cc observes it.
fn s2_noise_random(request: &str) -> String {
    let mut args = request.split_whitespace();
    assert_eq!(args.next(), Some("s2-noise-random"));
    let [visible, nonvisible, x0, y0, w, h, rx, ry] = std::array::from_fn(|_| num(&mut args));
    let mut planes: Vec<Plane<f32>> = (0..3)
        .map(|c| {
            let mut p = Plane::<f32>::create(rx + w + 24, ry + h + 2).unwrap();
            for y in 0..p.ysize() {
                let xsize = p.xsize();
                p.row_mut(y)[..xsize].fill(-17.0 - c as f32);
            }
            p
        })
        .collect();
    let r = Rect::new(rx, ry, w, h);
    let [p0, p1, p2] = &mut planes[..] else {
        unreachable!()
    };
    jxl::jxl::dec_noise::random_three_planes(
        visible,
        nonvisible,
        x0,
        y0,
        [(p0, r), (p1, r), (p2, r)],
    );
    let mut out = String::new();
    for p in &planes {
        for y in 0..p.ysize() {
            for x in 0..p.xsize() {
                write!(out, " {}", p.row(y)[x].to_bits()).unwrap();
            }
        }
    }
    out
}

/// Noise LUTs: none (HasAny false), the encoder's typical range, a flat and
/// a steep table, values past the clamp to one and negative values, special
/// words.
fn noise_luts() -> Vec<[u32; 8]> {
    let f = |v: [f32; 8]| v.map(f32::to_bits);
    let special = special_words();
    vec![
        [0; 8],
        f([0.0, 0.01, 0.02, 0.03, 0.05, 0.08, 0.1, 0.12]),
        f([0.3; 8]),
        f([0.0, 0.2, 0.5, 0.9, 1.3, 2.0, -0.4, 0.7]),
        f([-0.5, -0.1, 0.0, 0.25, 0.5, 0.75, 1.0, 1.5]),
        std::array::from_fn(|i| special[(i * 9 + 4) % special.len()]),
    ]
}

/// Rows the noise strength sees across the LUT range: (Y + X) / 2 and
/// (Y - X) / 2 from about -0.1 to 1.3, plus the LUT's knot positions.
fn noise_sample_sets(seed: u64) -> Vec<Vec<u32>> {
    let mut sets = sample_sets(seed);
    let mut seed = seed | 1;
    sets.push(
        (0..67)
            .map(|_| ((next(&mut seed) % 1_400_001) as f32 / 1_000_000.0 - 0.1).to_bits())
            .collect(),
    );
    let mut knots = Vec::new();
    for k in 0..=8 {
        let b = (k as f32 / 3.0).to_bits();
        knots.extend([b.wrapping_sub(1), b, b.wrapping_add(1)]);
    }
    sets.push(knots);
    sets
}

const NOISE_WIDTHS: &[usize] = &[0, 1, 2, 3, 4, 5, 7, 8, 9, 12, 15, 16, 17, 31, 33, 67];
const NOISE_ORIGINS: &[usize] = &[0, 1, 3, 8, 13];

#[test]
fn simd_add_noise_rows_match_the_original_target() {
    let t = target();
    select_simd(t);
    // Base correlations (binary16 x, b): the default, zero, typical, extremes.
    let correlations = [
        (65535, 65535),
        (0, 0),
        (0x3400, 0x3c00), // 0.25, 1.0
        (0xb800, 0x4000), // -0.5, 2.0
        (0x43ff, 0xc3ff), // 3.998, -3.998
    ];
    let mut requests = Vec::new();
    for (li, lut) in noise_luts().iter().enumerate() {
        for (ci, &(x16, b16)) in correlations.iter().enumerate() {
            for (si, samples) in noise_sample_sets(li as u64 * 13 + 5).iter().enumerate() {
                if (li + ci + si) % 2 == 1 && ci > 1 {
                    continue;
                }
                for &width in NOISE_WIDTHS {
                    for &origin in NOISE_ORIGINS {
                        let first_c = 3 + (width + origin) % 2;
                        requests.push(format!(
                            "s2-noise-add {first_c} {origin} {width} {} {x16} {b16} {} {}",
                            join(lut.iter().copied()),
                            samples.len(),
                            join(samples.iter().copied())
                        ));
                    }
                }
            }
        }
    }
    check_rows(
        "s2-noise-add",
        &requests,
        |r| (s2_noise_add(r), Box::new(|_| true)),
        |i| i > 0,
    );
}

/// The convolution reads two samples and two rows of border on each side
/// (`Settings::SymmetricBorderOnly(2)`); the requests fill those borders
/// and the extra columns `xextra_left`/`xextra_right` the pipelines pass.
#[test]
fn simd_convolve_noise_rows_match_the_original_target() {
    let t = target();
    select_simd(t);
    let mut requests = Vec::new();
    for (si, samples) in noise_sample_sets(91).iter().enumerate() {
        for &width in NOISE_WIDTHS {
            for &origin in NOISE_ORIGINS {
                for (left, right) in [(0, 0), (1, 0), (0, 1), (2, 2), (3, 5), (8, 7)] {
                    let first_c = 3 + (si + width) % 3;
                    requests.push(format!(
                        "s2-noise-conv {first_c} {origin} {width} {left} {right} {} {}",
                        samples.len(),
                        join(samples.iter().copied())
                    ));
                }
            }
        }
    }
    check_rows(
        "s2-noise-conv",
        &requests,
        |r| (s2_noise_conv(r), Box::new(|_| true)),
        |i| i > 0,
    );
}

/// Random3Planes: whole batches of 16 samples, then the remainder in whole
/// vectors, written past the rectangle into the planes' columns.
#[test]
fn simd_noise_generation_matches_the_original_target() {
    let t = target();
    select_simd(t);
    let mut requests = Vec::new();
    for (visible, nonvisible, x0, y0) in [(0, 0, 0, 0), (1, 0, 256, 0), (3, 2, 512, 768)] {
        for w in [
            0usize, 1, 3, 4, 5, 8, 9, 15, 16, 17, 20, 31, 32, 33, 47, 63, 100,
        ] {
            for (h, rx, ry) in [(1, 0, 0), (3, 2, 1), (2, 7, 3)] {
                requests.push(format!(
                    "s2-noise-random {visible} {nonvisible} {x0} {y0} {w} {h} {rx} {ry}"
                ));
            }
        }
    }
    check_rows(
        "s2-noise-random",
        &requests,
        |r| (s2_noise_random(r), Box::new(|_| true)),
        |_| false,
    );
}

/// The scalar parity suite's request families of one module, replayed
/// against the SIMD oracle with the port's SIMD dispatch selected: the
/// module's own tests call this `compare`, which compares word by word with
/// the NaN rule (D8) instead of the scalar oracle's.
macro_rules! replay_module {
    ($name:ident, $path:literal, $family:literal) => {
        mod $name {
            #![allow(dead_code, unused_imports, reason = "shared with the scalar suite")]
            pub(super) use std::fmt::Write as _;
            pub(super) use std::str::FromStr;
            pub(super) fn number<T: FromStr>(args: &mut std::str::SplitWhitespace<'_>) -> T
            where
                T::Err: std::fmt::Debug,
            {
                args.next().expect("test argument").parse().unwrap()
            }
            pub(super) use jxl::jxl::dec_bit_reader::BitReader;
            pub(super) fn hex(bytes: &[u8]) -> String {
                if bytes.is_empty() {
                    return "-".to_owned();
                }
                let mut result = String::new();
                for byte in bytes {
                    write!(result, "{byte:02x}").unwrap();
                }
                result
            }
            pub(super) fn unhex(hex: &str) -> Vec<u8> {
                if hex == "-" {
                    return Vec::new();
                }
                (0..hex.len())
                    .step_by(2)
                    .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
                    .collect()
            }
            /// Request generators of the module, answered by the SIMD oracle.
            pub(super) fn original_observations(requests: &[String]) -> Vec<String> {
                super::simd_common::oracle(requests)
            }
            pub(super) fn status_code(s: &jxl::jxl::base::status::Status) -> i32 {
                jxl::jxl::base::status::status_code(s)
            }
            pub(super) fn compare(requests: Vec<String>) {
                super::select_simd(super::target());
                super::check_rows(
                    $family,
                    &requests,
                    |r| {
                        let mut args = r.split_whitespace();
                        let kind = args.next().unwrap();
                        (inner::observe(kind, &mut args), Box::new(|_| true))
                    },
                    |_| true,
                );
            }
            mod inner {
                include!($path);
            }
        }
    };
}
replay_module!(
    simd_blending,
    "parity/blending_stage.rs",
    "blendstage (SIMD replay)"
);
replay_module!(simd_splines, "parity/splines.rs", "sp (SIMD replay)");
