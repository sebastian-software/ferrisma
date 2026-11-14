// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! SIMD parity of the encoder's AC strategy search (enc_ac_strategy.cc, the
//! `ProcessRectACS` export; port ADR 0004, "Encoder path"): the port's
//! NEON/AVX2 instantiations against the original built for the same single
//! Highway target (`JXL_CPP_ORACLE_SIMD`), through the `esa-*` requests of
//! tools/oracle/encoder_acs_simd.cc. Costs (entropy estimates) and decisions
//! (chosen strategies, priorities) are compared word for word, and so is
//! every word the original writes into the block and scratch memory; any
//! NaN equals any NaN (decision D8). Each family also shows that the scalar
//! translation gives different words for some of its requests, so the
//! comparison tells the two paths apart.
#![cfg(feature = "cpp-oracle-simd")]

mod simd_common;

use jxl::jxl::ac_strategy::AcStrategyImage;
use jxl::jxl::ac_strategy::{AcStrategy, AcStrategyType, MAX_BLOCK_DIM, MAX_COEFF_AREA};
use jxl::jxl::base::rect::Rect;
use jxl::jxl::base::status::status_code;
use jxl::jxl::enc_ac_strategy::{
    AcsConfig, AcsConfigScalars, AcsPlanes, estimate_entropy, find_best_8x8_transform,
};
use jxl::jxl::enc_ac_strategy_simd::{
    Best8x8Args, EntropyArgs, estimate_entropy_simd, find_best_8x8_transform_simd,
};
use jxl::jxl::hwy::Target;
use jxl::jxl::image::{Image3F, ImageF};
use jxl::jxl::memory_manager_internal::MemoryManagerPtr;
use jxl::jxl::quant_weights::DequantMatrices;
use simd_common::{check_rows, next, special_words, target};
use std::fmt::Write as _;
use std::sync::OnceLock;

type Args<'a> = std::str::SplitWhitespace<'a>;

fn number<T: std::str::FromStr>(args: &mut Args<'_>) -> T
where
    T::Err: std::fmt::Debug,
{
    args.next().unwrap().parse().unwrap()
}
fn float(args: &mut Args<'_>) -> f32 {
    f32::from_bits(number(args))
}
fn words(out: &mut String, tag: &str, values: &[f32]) {
    write!(out, " {tag}").unwrap();
    for &v in values {
        write!(out, " {}", v.to_bits()).unwrap();
    }
}
fn push_floats(s: &mut String, values: &[f32]) {
    for v in values {
        write!(s, " {}", v.to_bits()).unwrap();
    }
}
/// The finite sentinel words of tools/oracle/encoder_acs_simd.cc.
fn sentinel(salt: u32, i: usize) -> f32 {
    f32::from_bits(0x5a00_0000 + salt * 0x10_0000 + (i as u32 & 0xf_ffff))
}
fn all_float(_: usize) -> bool {
    true
}
fn all_defined(_: usize) -> bool {
    true
}

const MAX_LANES: usize = 8;
const BLOCK_WORDS: usize = 3 * MAX_COEFF_AREA;
/// PrepareForThreads' mem_per_thread past the block, for 8 lanes.
const SCRATCH_WORDS: usize = 3 * MAX_COEFF_AREA + 3 * MAX_LANES * MAX_BLOCK_DIM;

/// The scratch up to its last written word plus 4.
fn scratch_words(out: &mut String, scratch: &[f32], salt: u32) {
    let used = (0..SCRATCH_WORDS)
        .rev()
        .find(|&i| scratch[i].to_bits() != sentinel(salt, i).to_bits())
        .map_or(0, |i| i + 1);
    words(out, "W", &scratch[..SCRATCH_WORDS.min(used + 4)]);
}

/// Every strategy below DCT128X128, as AcStrategyHeuristics::Init computes.
fn acs_mask() -> u32 {
    (0..AcStrategyType::DCT128X128.0).fold(0, |mask, i| mask | (1 << i))
}

/// The default matrices with the target's ComputeQuantTable, as the
/// original's single-target build computes them.
fn simd_matrices() -> &'static DequantMatrices {
    static MATRICES: OnceLock<DequantMatrices> = OnceLock::new();
    MATRICES.get_or_init(|| {
        let mut matrices = DequantMatrices::default();
        matrices
            .ensure_computed_for_target(MemoryManagerPtr::NATIVE, acs_mask(), Some(target()))
            .unwrap();
        matrices
    })
}

// Inputs ---------------------------------------------------------------------

/// The `<inputs>` of every request: the planes an ACSConfig points to and
/// its scalars.
struct Inputs {
    src: Image3F,
    qf: ImageF,
    mask: ImageF,
    mask1x1: ImageF,
    scalars: AcsConfigScalars,
}

impl Inputs {
    fn read(args: &mut Args<'_>) -> Self {
        let xsize: usize = number(args);
        let ysize: usize = number(args);
        let mut src = Image3F::create(xsize, ysize).unwrap();
        for c in 0..3 {
            for y in 0..ysize {
                for v in &mut src.plane_row_mut(c, y)[..xsize] {
                    *v = float(args);
                }
            }
        }
        let mut plane = |xs: usize, ys: usize| {
            let mut p = ImageF::create(xs, ys).unwrap();
            for y in 0..ys {
                for v in &mut p.row_mut(y)[..xs] {
                    *v = float(args);
                }
            }
            p
        };
        let qf = plane(xsize / 8, ysize / 8);
        let mask1x1 = plane(xsize, ysize);
        let scalars = AcsConfigScalars {
            quant_field_stride: qf.pixels_per_row() as usize,
            masking_field_stride: 0,
            masking1x1_field_stride: mask1x1.pixels_per_row() as usize,
            src_stride: src.plane(0).pixels_per_row() as usize,
            info_loss_multiplier: float(args),
            cost_delta: float(args),
            zeros_mul: float(args),
            mask1x1_xsize: number(args),
        };
        Self {
            src,
            qf,
            mask: ImageF::default(),
            mask1x1,
            scalars,
        }
    }
    fn planes(&self) -> AcsPlanes<'_> {
        AcsPlanes {
            src: &self.src,
            rect_in: Rect::new(0, 0, self.src.xsize(), self.src.ysize()),
            quant_field: &self.qf,
            mask: &self.mask,
            mask1x1: &self.mask1x1,
            dequant: simd_matrices(),
        }
    }
}

/// Image content of the requests (all finite unless `Special`).
#[derive(Clone, Copy, Debug)]
enum Content {
    /// Smooth opsin-like gradients with mild texture.
    Smooth,
    /// Sharp edges and lines: favors small transforms.
    Edges,
    /// Uniform noise: high entropy everywhere.
    Noise,
    /// Flat with a faint ramp: favors large transforms.
    Flat,
    /// Smooth content with the special words (NaN, infinities, extremes,
    /// subnormals) at every 37th sample.
    Special,
}
const CONTENTS: [Content; 5] = [
    Content::Smooth,
    Content::Edges,
    Content::Noise,
    Content::Flat,
    Content::Special,
];

fn unit(seed: &mut u64) -> f32 {
    (next(seed) >> 40) as f32 / (1u64 << 24) as f32
}

/// The request's `<inputs>` for a `xsize` x `ysize` image: opsin-like
/// planes (X small, Y in [0, 0.85), B near Y), a positive quant field and a
/// positive pixel masking field, the ACSConfig multipliers of Init at
/// `distance`, and `mask1x1_xsize`.
#[expect(clippy::excessive_precision, reason = "the source's decimal literals")]
fn inputs(
    xsize: usize,
    ysize: usize,
    content: Content,
    distance: f32,
    mask1x1_xsize: usize,
    seed: u64,
) -> String {
    let mut seed = seed.wrapping_mul(0x9e37_79b9_7f4a_7c15) | 1;
    let specials = special_words();
    let mut s = format!(" {xsize} {ysize}");
    for c in 0..3 {
        let (scale, offset) = [(0.05f32, 0.0f32), (0.7, 0.1), (0.6, 0.15)][c];
        let mut values = Vec::with_capacity(xsize * ysize);
        for y in 0..ysize {
            for x in 0..xsize {
                let (fx, fy) = (x as f32 / xsize as f32, y as f32 / ysize as f32);
                let v = match content {
                    Content::Smooth | Content::Special => {
                        offset
                            + scale * (0.5 + 0.3 * (fx * 3.1 + fy * 1.7).sin() * (fy * 2.3).cos())
                            + 0.01 * scale * (unit(&mut seed) - 0.5)
                    }
                    Content::Edges => {
                        let edge = ((x / 5 + y / 11) % 3) as f32 * 0.4;
                        let line = if (x + 2 * y) % 13 == 0 { 0.5 } else { 0.0 };
                        offset + scale * (edge + line)
                    }
                    Content::Noise => offset + scale * unit(&mut seed),
                    Content::Flat => offset + scale * (0.4 + 0.02 * fx - 0.01 * fy),
                };
                let i = y * xsize + x;
                let word = if matches!(content, Content::Special) && i % 37 == 11 {
                    specials[(i / 37 + c * 5) % specials.len()]
                } else {
                    v.to_bits()
                };
                values.push(f32::from_bits(word));
            }
        }
        push_floats(&mut s, &values);
    }
    let qf: Vec<f32> = (0..xsize / 8 * ysize / 8)
        .map(|_| 0.2 + 2.8 * unit(&mut seed))
        .collect();
    push_floats(&mut s, &qf);
    let mask1x1: Vec<f32> = (0..xsize * ysize)
        .map(|_| 0.01 + 1.5 * unit(&mut seed))
        .collect();
    push_floats(&mut s, &mask1x1);
    // AcStrategyHeuristics::Init.
    let bias = 0.13731742964354549f64 as f32;
    let ratio = (distance + bias) / (1.0f32 + bias);
    let ilm = 1.2f64 as f32 * ratio.powf(0.33677806662454718f64 as f32);
    let zeros_mul = 9.3089059022677905f64 as f32 * ratio.powf(0.50990926717963703f64 as f32);
    let cost_delta = 10.833273317067883f64 as f32 * ratio.powf(0.36702940662370243f64 as f32);
    push_floats(&mut s, &[ilm, cost_delta, zeros_mul]);
    write!(s, " {mask1x1_xsize}").unwrap();
    s
}

/// mask1x1_xsize variants for a strategy at pixel column `x` covering
/// `width` pixels in an image `xsize` wide: the whole width, cut inside a
/// vector of the first and last block (4- and 8-lane bounds differ there),
/// and no masking field.
fn mask_widths(x: usize, width: usize, xsize: usize) -> [usize; 4] {
    [xsize, x + 5, (x + width - 3).min(xsize), 0]
}

// esa-entropy ----------------------------------------------------------------

fn observe_entropy(request: &str, simd: Option<Target>) -> String {
    let mut args = request.split_whitespace();
    assert_eq!(args.next(), Some("esa-entropy"));
    let inputs = Inputs::read(&mut args);
    let raw: u32 = number(&mut args);
    let entropy_mul = float(&mut args);
    let x: usize = number(&mut args);
    let y: usize = number(&mut args);
    let cmap_factors = [float(&mut args), float(&mut args), float(&mut args)];
    let planes = inputs.planes();
    let config = AcsConfig::new(&inputs.scalars, &planes);
    let acs = AcStrategy::from_raw_strategy(AcStrategyType(raw));
    let mut block: Vec<f32> = (0..BLOCK_WORDS + MAX_LANES)
        .map(|i| sentinel(1, i))
        .collect();
    let mut scratch: Vec<f32> = (0..SCRATCH_WORDS).map(|i| sentinel(2, i)).collect();
    let mut quantized = vec![0x5a5a_5a5au32; MAX_COEFF_AREA];
    let mut entropy = -1.0f32;
    let status = match simd {
        Some(t) => estimate_entropy_simd(
            t,
            &acs,
            EntropyArgs {
                entropy_mul,
                x,
                y,
                config: &config,
                cmap_factors: &cmap_factors,
                block: &mut block,
                full_scratch_space: &mut scratch,
                quantized: &mut quantized,
                entropy: &mut entropy,
            },
        ),
        None => estimate_entropy(
            &acs,
            entropy_mul,
            x,
            y,
            &config,
            &cmap_factors,
            &mut block,
            &mut scratch,
            &mut quantized,
            &mut entropy,
        ),
    };
    let size = (1usize << acs.log2_covered_blocks()) * 64;
    let mut out = format!("E {} {}", status_code(&status), entropy.to_bits());
    words(&mut out, "B", &block[..3 * size + MAX_LANES]);
    scratch_words(&mut out, &scratch, 2);
    out
}

/// Runs `requests` through the port's SIMD observation and compares it with
/// the original's target, after showing that the scalar translation differs
/// in at least one request (its observations go through `observe` with
/// `None`).
fn check_family(family: &str, requests: &[String], observe: fn(&str, Option<Target>) -> String) {
    let t = target();
    let differing = requests
        .iter()
        .filter(|r| observe(r, None) != observe(r, Some(t)))
        .count();
    assert!(
        differing > 0,
        "{family}: the scalar and SIMD paths gave the same words everywhere"
    );
    println!(
        "{family}: the scalar translation differs in {differing} of {} requests",
        requests.len()
    );
    check_rows(
        family,
        requests,
        |r| (observe(r, Some(t)), Box::new(all_defined)),
        all_float,
    );
}

/// The image sizes of the requests: one 64x64 tile, a tile with a partial
/// second row and column of blocks, and a wide image whose strategies sit
/// at columns that are no multiple of 64.
const SIZES: [(usize, usize); 3] = [(64, 64), (80, 72), (136, 72)];
const DISTANCES: [f32; 4] = [0.5, 1.0, 2.9, 13.0];
const CMAPS: [[f32; 3]; 3] = [[0.0, 0.0, 0.0], [0.031, 0.0, 0.97], [-0.43, 0.0, 1.31]];

#[test]
fn enc_simd_estimate_entropy_matches_the_original_target() {
    let mut requests = Vec::new();
    for raw in 0..AcStrategyType::DCT128X128.0 {
        let acs = AcStrategy::from_raw_strategy(AcStrategyType(raw));
        let (w, h) = (8 * acs.covered_blocks_x(), 8 * acs.covered_blocks_y());
        for (si, &(xsize, ysize)) in SIZES.iter().enumerate() {
            if w > xsize || h > ysize {
                continue;
            }
            for (ci, &content) in CONTENTS.iter().enumerate() {
                let positions = [
                    (0, 0),
                    ((xsize - w) / 16 * 8, (ysize - h) / 16 * 8),
                    (xsize - w, ysize - h),
                ];
                for (pi, &(x, y)) in positions.iter().enumerate() {
                    let variant = si * 7 + ci * 3 + pi + raw as usize;
                    let widths = mask_widths(x, w, xsize);
                    let mask1x1_xsize = widths[variant % 4];
                    let distance = DISTANCES[variant % 4];
                    let seed = u64::from(raw) * 1000 + variant as u64;
                    let mut s = "esa-entropy".to_owned();
                    s.push_str(&inputs(
                        xsize,
                        ysize,
                        content,
                        distance,
                        mask1x1_xsize,
                        seed,
                    ));
                    let entropy_mul = [1.0f32, 0.8, 1.21, 2.25][(variant / 4) % 4];
                    write!(s, " {raw} {} {x} {y}", entropy_mul.to_bits()).unwrap();
                    push_floats(&mut s, &CMAPS[variant % 3]);
                    requests.push(s);
                }
            }
        }
    }
    check_family("esa-entropy", &requests, observe_entropy);
}

// esa-best8 ------------------------------------------------------------------

fn observe_best8(request: &str, simd: Option<Target>) -> String {
    let mut args = request.split_whitespace();
    assert_eq!(args.next(), Some("esa-best8"));
    let inputs = Inputs::read(&mut args);
    let x: usize = number(&mut args);
    let y: usize = number(&mut args);
    let speed_tier: i32 = number(&mut args);
    let butteraugli_target = float(&mut args);
    let cmap_factors = [float(&mut args), float(&mut args), float(&mut args)];
    let planes = inputs.planes();
    let config = AcsConfig::new(&inputs.scalars, &planes);
    let mut ac_strategy =
        AcStrategyImage::create(inputs.src.xsize() / 8, inputs.src.ysize() / 8).unwrap();
    ac_strategy.fill_dct8_full();
    let mut block: Vec<f32> = (0..BLOCK_WORDS + MAX_LANES)
        .map(|i| sentinel(1, i))
        .collect();
    let mut scratch: Vec<f32> = (0..SCRATCH_WORDS).map(|i| sentinel(2, i)).collect();
    let mut quantized = vec![0x5a5a_5a5au32; MAX_COEFF_AREA];
    let mut entropy = -1.0f32;
    let mut best = AcStrategyType::DCT128X256;
    let status = match simd {
        Some(t) => find_best_8x8_transform_simd(
            t,
            Best8x8Args {
                x,
                y,
                encoding_speed_tier: speed_tier,
                butteraugli_target,
                config: &config,
                cmap_factors: &cmap_factors,
                ac_strategy: &mut ac_strategy,
                block: &mut block,
                scratch_space: &mut scratch,
                quantized: &mut quantized,
                entropy_out: &mut entropy,
                best_tx: &mut best,
            },
        ),
        None => find_best_8x8_transform(
            x,
            y,
            speed_tier,
            butteraugli_target,
            &config,
            &cmap_factors,
            &mut ac_strategy,
            &mut block,
            &mut scratch,
            &mut quantized,
            &mut entropy,
            &mut best,
        ),
    };
    let mut out = format!(
        "T {} {} {}",
        status_code(&status),
        entropy.to_bits(),
        best.0
    );
    words(&mut out, "B", &block[..3 * 64 + MAX_LANES]);
    scratch_words(&mut out, &scratch, 2);
    out
}

#[test]
fn enc_simd_find_best_8x8_transform_matches_the_original_target() {
    // Speed tiers that admit each subset of the ten candidates (limits 9, 5
    // and 4), and distances on both sides of FindBest8x8Transform's 4, 5 and
    // 12 thresholds.
    let tiers = [0i32, 3, 4, 5, 7, 9];
    let distances = [0.5f32, 1.0, 2.9, 4.5, 5.0, 8.0, 12.0, 13.0];
    let mut requests = Vec::new();
    for (si, &(xsize, ysize)) in SIZES.iter().enumerate() {
        for (ci, &content) in CONTENTS.iter().enumerate() {
            for (di, &distance) in distances.iter().enumerate() {
                for (ti, &tier) in tiers.iter().enumerate() {
                    let variant = si * 11 + ci * 5 + di * 3 + ti;
                    let blocks = (xsize / 8) * (ysize / 8);
                    let index = (variant * 37) % blocks;
                    let (x, y) = (index % (xsize / 8) * 8, index / (xsize / 8) * 8);
                    let mask1x1_xsize = mask_widths(x, 8, xsize)[variant % 4];
                    let seed = variant as u64 * 7 + 3;
                    let mut s = "esa-best8".to_owned();
                    s.push_str(&inputs(
                        xsize,
                        ysize,
                        content,
                        distance.min(3.0),
                        mask1x1_xsize,
                        seed,
                    ));
                    write!(s, " {x} {y} {tier} {}", distance.to_bits()).unwrap();
                    push_floats(&mut s, &CMAPS[variant % 3]);
                    requests.push(s);
                }
            }
        }
    }
    // The decisions vary: most candidates win somewhere.
    let t = target();
    let chosen: std::collections::BTreeSet<String> = requests
        .iter()
        .map(|r| {
            observe_best8(r, Some(t))
                .split(' ')
                .nth(3)
                .unwrap()
                .to_owned()
        })
        .collect();
    println!("esa-best8: chosen strategies {chosen:?}");
    assert!(chosen.len() >= 6, "esa-best8 chose only {chosen:?}");
    check_family("esa-best8", &requests, observe_best8);
}
