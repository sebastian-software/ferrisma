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
use jxl::jxl::chroma_from_luma::ColorCorrelationMap;
use jxl::jxl::common::SpeedTier;
use jxl::jxl::enc_ac_strategy::{
    AcStrategyHeuristics, AcsConfig, AcsConfigScalars, AcsPlanes, estimate_entropy,
    find_best_8x8_transform, find_best_first_level_division_for_square, try_merge_acs,
};
use jxl::jxl::enc_ac_strategy_simd::{
    Best8x8Args, EntropyArgs, MergeState, SquareArgs, TryMergeArgs, estimate_entropy_simd,
    find_best_8x8_transform_simd, find_best_first_level_division_for_square_simd,
    try_merge_acs_simd,
};
use jxl::jxl::enc_aux_out::AuxOut;
use jxl::jxl::enc_params::CompressParams;
use jxl::jxl::frame_dimensions::FrameDimensions;
use jxl::jxl::hwy::{self, Target};
use jxl::jxl::image::{Image3F, ImageF};
use jxl::jxl::memory_manager_internal::MemoryManagerPtr;
use jxl::jxl::quant_weights::DequantMatrices;
use simd_common::{check_rows, next, special_words, target};
use std::fmt::Write as _;
use std::sync::{Mutex, MutexGuard, OnceLock};

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

/// Runs a scalar observation: the scalar translation's transforms dispatch
/// on the encoder's selection, which the esa-rect test overrides while it
/// holds [`dispatch_lock`].
fn scalar<T>(observe: impl FnOnce() -> T) -> T {
    let _lock = dispatch_lock();
    observe()
}

/// Runs `requests` through the port's SIMD observation and compares it with
/// the original's target, after showing that the scalar translation differs
/// in at least one request (its observations go through `observe` with
/// `None`).
fn check_family(family: &str, requests: &[String], observe: fn(&str, Option<Target>) -> String) {
    let t = target();
    let differing = requests
        .iter()
        .filter(|r| scalar(|| observe(r, None)) != observe(r, Some(t)))
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

/// [`check_family`] for observations that mix float words with `raw:first`
/// strategy words (compared exactly).
fn check_family_with(
    family: &str,
    requests: &[String],
    observe: fn(&str, Option<Target>) -> String,
) {
    let t = target();
    let differing = requests
        .iter()
        .filter(|r| scalar(|| observe(r, None)) != observe(r, Some(t)))
        .count();
    assert!(
        differing > 0,
        "{family}: the scalar and SIMD paths gave the same words everywhere"
    );
    println!(
        "{family}: the scalar translation differs in {differing} of {} requests",
        requests.len()
    );
    let observed = simd_common::oracle(requests);
    let mut words = 0;
    for (request, cpp) in requests.iter().zip(&observed) {
        let rust = observe(request, Some(t));
        words += simd_common::same(&rust, cpp, strategy_float(&rust), all_defined)
            .unwrap_or_else(|e| panic!("{family}: {e}\nrequest: {request}"));
    }
    println!(
        "{family} on {}: {} requests, {words} words matched",
        t.name(),
        requests.len()
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

// esa-merge, esa-square -------------------------------------------------------

fn read_layout(args: &mut Args<'_>, xblocks: usize, yblocks: usize) -> AcStrategyImage {
    let mut image = AcStrategyImage::create(xblocks, yblocks).unwrap();
    image.fill_dct8_full();
    let n: usize = number(args);
    for _ in 0..n {
        let bx: usize = number(args);
        let by: usize = number(args);
        let raw: u32 = number(args);
        image.set(bx, by, AcStrategyType(raw)).unwrap();
    }
    image
}
fn print_strategy(out: &mut String, image: &AcStrategyImage) {
    out.push_str(" S");
    for y in 0..image.ysize() {
        let row = image.const_row(y, 0);
        for x in 0..image.xsize() {
            let acs = row.get(x);
            write!(
                out,
                " {}:{}",
                acs.raw_strategy(),
                u8::from(acs.is_first_block())
            )
            .unwrap();
        }
    }
}
/// The strategy words are integers: compare them exactly.
fn strategy_float(out: &str) -> impl Fn(usize) -> bool + use<> {
    let words: Vec<bool> = out.split_whitespace().map(|w| !w.contains(':')).collect();
    move |i| *words.get(i).unwrap_or(&true)
}

fn observe_merge(request: &str, simd: Option<Target>) -> String {
    let mut args = request.split_whitespace();
    let command = args.next().unwrap();
    let inputs = Inputs::read(&mut args);
    let mut ac_strategy = read_layout(&mut args, inputs.src.xsize() / 8, inputs.src.ysize() / 8);
    let planes = inputs.planes();
    let config = AcsConfig::new(&inputs.scalars, &planes);
    let mut block: Vec<f32> = (0..BLOCK_WORDS + MAX_LANES)
        .map(|i| sentinel(1, i))
        .collect();
    let mut scratch: Vec<f32> = (0..SCRATCH_WORDS).map(|i| sentinel(2, i)).collect();
    let mut quantized = vec![0x5a5a_5a5au32; MAX_COEFF_AREA];
    let mut entropy_estimate = [0.0f32; 64];
    let mut out;
    let block_words;
    if command == "esa-merge" {
        let raw: u32 = number(&mut args);
        let bx: usize = number(&mut args);
        let by: usize = number(&mut args);
        let cx: usize = number(&mut args);
        let cy: usize = number(&mut args);
        let entropy_mul = float(&mut args);
        let candidate_priority: u8 = number(&mut args);
        let cmap_factors = [float(&mut args), float(&mut args), float(&mut args)];
        let mut priority = [0u8; 64];
        for p in &mut priority {
            *p = number(&mut args);
        }
        for e in &mut entropy_estimate {
            *e = float(&mut args);
        }
        let status = match simd {
            Some(t) => try_merge_acs_simd(
                t,
                MergeState {
                    bx,
                    by,
                    config: &config,
                    cmap_factors: &cmap_factors,
                    ac_strategy: &mut ac_strategy,
                    entropy_estimate: &mut entropy_estimate,
                    block: &mut block,
                    scratch_space: &mut scratch,
                    quantized: &mut quantized,
                },
                TryMergeArgs {
                    acs_raw: AcStrategyType(raw),
                    cx,
                    cy,
                    entropy_mul,
                    candidate_priority,
                    priority: &mut priority,
                },
            ),
            None => try_merge_acs(
                AcStrategyType(raw),
                bx,
                by,
                cx,
                cy,
                &config,
                &cmap_factors,
                &mut ac_strategy,
                entropy_mul,
                candidate_priority,
                &mut priority,
                &mut entropy_estimate,
                &mut block,
                &mut scratch,
                &mut quantized,
            ),
        };
        out = format!("M {} P", status_code(&status));
        for p in priority {
            write!(out, " {p}").unwrap();
        }
        let acs = AcStrategy::from_raw_strategy(AcStrategyType(raw));
        block_words = 3 * (1usize << acs.log2_covered_blocks()) * 64;
    } else {
        assert_eq!(command, "esa-square");
        let blocks: usize = number(&mut args);
        let allow_square = number::<i32>(&mut args) != 0;
        let bx: usize = number(&mut args);
        let by: usize = number(&mut args);
        let cx: usize = number(&mut args);
        let cy: usize = number(&mut args);
        let mul_jxk = float(&mut args);
        let mul_jxj = float(&mut args);
        let cmap_factors = [float(&mut args), float(&mut args), float(&mut args)];
        for e in &mut entropy_estimate {
            *e = float(&mut args);
        }
        let status = match simd {
            Some(t) => find_best_first_level_division_for_square_simd(
                t,
                MergeState {
                    bx,
                    by,
                    config: &config,
                    cmap_factors: &cmap_factors,
                    ac_strategy: &mut ac_strategy,
                    entropy_estimate: &mut entropy_estimate,
                    block: &mut block,
                    scratch_space: &mut scratch,
                    quantized: &mut quantized,
                },
                SquareArgs {
                    blocks,
                    allow_square_transform: allow_square,
                    cx,
                    cy,
                    entropy_mul_JXK: mul_jxk,
                    entropy_mul_JXJ: mul_jxj,
                },
            ),
            None => find_best_first_level_division_for_square(
                blocks,
                allow_square,
                bx,
                by,
                cx,
                cy,
                &config,
                &cmap_factors,
                &mut ac_strategy,
                mul_jxk,
                mul_jxj,
                &mut entropy_estimate,
                &mut block,
                &mut scratch,
                &mut quantized,
            ),
        };
        out = format!("F {}", status_code(&status));
        block_words = 3 * blocks * blocks * 64;
    }
    words(&mut out, "E", &entropy_estimate);
    print_strategy(&mut out, &ac_strategy);
    words(&mut out, "B", &block[..block_words + MAX_LANES]);
    scratch_words(&mut out, &scratch, 2);
    out
}

/// A strategy layout over the image's blocks: random strategies below
/// DCT128X128 (DCT8 in half of the draws) that fit inside their 64x64 tile
/// without overlapping, at positions aligned to their size when `aligned`
/// (as the hierarchical merges place them) or anywhere (as the floating,
/// non-aligned merges do).
fn layout(xblocks: usize, yblocks: usize, aligned: bool, seed: u64) -> String {
    let mut seed = seed | 1;
    let mut taken = vec![false; xblocks * yblocks];
    let mut placed = Vec::new();
    for by in 0..yblocks {
        for bx in 0..xblocks {
            if taken[by * xblocks + bx] {
                continue;
            }
            let draw = next(&mut seed);
            let raw = if draw.is_multiple_of(2) {
                0
            } else {
                ((draw >> 8) % u64::from(AcStrategyType::DCT128X128.0)) as u32
            };
            let acs = AcStrategy::from_raw_strategy(AcStrategyType(raw));
            let (w, h) = (acs.covered_blocks_x(), acs.covered_blocks_y());
            let fits = (!aligned || (bx % w == 0 && by % h == 0))
                && bx % 8 + w <= 8
                && by % 8 + h <= 8
                && bx + w <= xblocks
                && by + h <= yblocks
                && (0..h).all(|y| (0..w).all(|x| !taken[(by + y) * xblocks + bx + x]));
            let (raw, w, h) = if fits { (raw, w, h) } else { (0, 1, 1) };
            for y in 0..h {
                for x in 0..w {
                    taken[(by + y) * xblocks + bx + x] = true;
                }
            }
            if raw != 0 {
                placed.push((bx, by, raw));
            }
        }
    }
    let mut s = format!(" {}", placed.len());
    for (bx, by, raw) in placed {
        write!(s, " {bx} {by} {raw}").unwrap();
    }
    s
}

/// ProcessRectACS-like entropy estimates of a tile: each block's best 8x8
/// entropy times the 8x8 multiplier (the scalar translation computes them;
/// they are request inputs), scaled by `scale` to move the merge decisions,
/// with the covered blocks of some earlier merges zeroed.
fn estimates(request_inputs: &str, bx: usize, by: usize, scale: f32, seed: u64) -> Vec<f32> {
    let mut args = request_inputs.split_whitespace();
    let inputs = Inputs::read(&mut args);
    let planes = inputs.planes();
    let config = AcsConfig::new(&inputs.scalars, &planes);
    let mut block = vec![0.0f32; BLOCK_WORDS];
    let mut scratch = vec![0.0f32; SCRATCH_WORDS];
    let mut quantized = vec![0u32; MAX_COEFF_AREA];
    let mut image =
        AcStrategyImage::create(inputs.src.xsize() / 8, inputs.src.ysize() / 8).unwrap();
    // The scalar translation's transforms dispatch on the encoder's
    // selection, which the esa-rect test overrides.
    let _lock = dispatch_lock();
    let mut seed = seed | 1;
    let mut out = vec![0.0f32; 64];
    for iy in 0..8 {
        for ix in 0..8 {
            let (x, y) = (8 * (bx + ix), 8 * (by + iy));
            if x + 8 > inputs.src.xsize() || y + 8 > inputs.src.ysize() {
                continue;
            }
            let mut entropy = 0.0f32;
            let mut best = AcStrategyType::DCT;
            find_best_8x8_transform(
                x,
                y,
                3,
                1.0,
                &config,
                &[0.0; 3],
                &mut image,
                &mut block,
                &mut scratch,
                &mut quantized,
                &mut entropy,
                &mut best,
            )
            .unwrap();
            let jitter = 0.9 + 0.2 * unit(&mut seed);
            out[iy * 8 + ix] = entropy * (1.0 - 0.4 / (1.0 + 1.4)) * scale * jitter;
            if next(&mut seed).is_multiple_of(11) {
                out[iy * 8 + ix] = 0.0;
            }
        }
    }
    out
}

/// The tile origins (in blocks) of an image and the tile's size in blocks.
fn tiles(xsize: usize, ysize: usize) -> Vec<(usize, usize, usize, usize)> {
    let (xb, yb) = (xsize / 8, ysize / 8);
    let mut out = Vec::new();
    for by in (0..yb).step_by(8) {
        for bx in (0..xb).step_by(8) {
            out.push((bx, by, (xb - bx).min(8), (yb - by).min(8)));
        }
    }
    out
}

const MERGE_KINDS: [AcStrategyType; 9] = [
    AcStrategyType::DCT16X8,
    AcStrategyType::DCT8X16,
    AcStrategyType::DCT16X32,
    AcStrategyType::DCT32X16,
    AcStrategyType::DCT64X32,
    AcStrategyType::DCT32X64,
    AcStrategyType::DCT16X16,
    AcStrategyType::DCT32X8,
    AcStrategyType::DCT64X64,
];

#[test]
fn enc_simd_try_merge_acs_matches_the_original_target() {
    let mut requests = Vec::new();
    let mut variant = 0usize;
    for &(xsize, ysize) in &SIZES {
        for (ci, &content) in CONTENTS.iter().enumerate() {
            let distance = DISTANCES[ci % 4];
            let mask1x1_xsize = [xsize, xsize - 3, 13][ci % 3];
            let seed = (xsize * 1000 + ysize + ci) as u64;
            let base = inputs(xsize, ysize, content, distance, mask1x1_xsize, seed);
            for (ti, &(bx, by, tw, th)) in tiles(xsize, ysize).iter().enumerate() {
                for (ki, &kind) in MERGE_KINDS.iter().enumerate() {
                    let acs = AcStrategy::from_raw_strategy(kind);
                    let (w, h) = (acs.covered_blocks_x(), acs.covered_blocks_y());
                    if w > tw || h > th {
                        continue;
                    }
                    for pick in 0..3 {
                        variant += 1;
                        let cx = [0, (tw - w) / 2, tw - w][(pick + ki) % 3];
                        let cy = [0, th - h, (th - h) / 2][(pick + ti) % 3];
                        let scale = [0.6f32, 1.0, 1.3, 2.5][variant % 4];
                        let entropy = estimates(&base, bx, by, scale, variant as u64);
                        let candidate_priority = [2u8, 4, 6][ki / 2 % 3];
                        let mut priority = [0u8; 64];
                        let mut pseed = variant as u64 * 13 + 1;
                        for p in &mut priority {
                            if next(&mut pseed).is_multiple_of(9) {
                                *p = [2, 4, 6][(next(&mut pseed) % 3) as usize];
                            }
                        }
                        let mut s = "esa-merge".to_owned();
                        s.push_str(&base);
                        s.push_str(&layout(
                            xsize / 8,
                            ysize / 8,
                            variant.is_multiple_of(2),
                            seed + variant as u64,
                        ));
                        let mul = [1.21f32, 1.49, 2.25, 0.8][ki / 2 % 4];
                        write!(
                            s,
                            " {} {bx} {by} {cx} {cy} {} {candidate_priority}",
                            kind.0,
                            mul.to_bits()
                        )
                        .unwrap();
                        push_floats(&mut s, &CMAPS[variant % 3]);
                        for p in priority {
                            write!(s, " {p}").unwrap();
                        }
                        push_floats(&mut s, &entropy);
                        requests.push(s);
                    }
                }
            }
        }
    }
    // Some candidates are accepted and some are not.
    let t = target();
    let accepted = requests
        .iter()
        .filter(|r| {
            let out = observe_merge(r, Some(t));
            let priorities: Vec<&str> = out.split(' ').skip(3).take(64).collect();
            let given: Vec<&str> = r.split(' ').rev().skip(64).take(64).collect();
            priorities.iter().ne(given.iter().rev())
        })
        .count();
    println!(
        "esa-merge: {accepted} of {} candidates accepted",
        requests.len()
    );
    assert!(accepted > requests.len() / 10 && accepted < requests.len() * 9 / 10);
    check_family_with("esa-merge", &requests, observe_merge);
}

#[test]
fn enc_simd_find_best_first_level_division_for_square_matches_the_original_target() {
    let mut requests = Vec::new();
    let mut variant = 0usize;
    for &(xsize, ysize) in &SIZES {
        for (ci, &content) in CONTENTS.iter().enumerate() {
            let distance = DISTANCES[(ci + 1) % 4];
            let mask1x1_xsize = [xsize, 21, xsize - 5][ci % 3];
            let seed = (xsize * 1000 + ysize + ci) as u64 + 77;
            let base = inputs(xsize, ysize, content, distance, mask1x1_xsize, seed);
            for &(bx, by, tw, th) in &tiles(xsize, ysize) {
                for (blocks, mul_jxk, mul_jxj) in
                    [(2usize, 1.21f32, 1.34f32), (4, 1.49, 1.48), (8, 2.25, 2.25)]
                {
                    if blocks > tw || blocks > th {
                        continue;
                    }
                    // Aligned and floating positions.
                    let mut positions = vec![(0, 0), (tw - blocks, th - blocks)];
                    if blocks < 8 {
                        positions.push((1.min(tw - blocks), 1.min(th - blocks)));
                        positions.push(((tw - blocks) / 2, 0));
                    }
                    for (pi, &(cx, cy)) in positions.iter().enumerate() {
                        for allow in [true, false] {
                            variant += 1;
                            let scale = [0.7f32, 1.0, 1.4, 3.0][variant % 4];
                            let entropy = estimates(&base, bx, by, scale, variant as u64);
                            let mut s = "esa-square".to_owned();
                            s.push_str(&base);
                            s.push_str(&layout(
                                xsize / 8,
                                ysize / 8,
                                !(pi + variant).is_multiple_of(3),
                                seed + variant as u64,
                            ));
                            write!(
                                s,
                                " {blocks} {} {bx} {by} {cx} {cy} {} {}",
                                u8::from(allow),
                                mul_jxk.to_bits(),
                                mul_jxj.to_bits()
                            )
                            .unwrap();
                            push_floats(&mut s, &CMAPS[variant % 3]);
                            push_floats(&mut s, &entropy);
                            requests.push(s);
                        }
                    }
                }
            }
        }
    }
    // The decisions vary: some squares split, merge or stay.
    let t = target();
    let outcomes: std::collections::BTreeSet<String> = requests
        .iter()
        .map(|r| {
            let out = observe_merge(r, Some(t));
            let start = out.find(" S").unwrap();
            let strategies = &out[start..out.find(" B").unwrap()];
            strategies
                .split(' ')
                .filter(|w| w.ends_with(":1") && !w.starts_with("0:"))
                .map(|w| w.split(':').next().unwrap().to_owned())
                .collect::<Vec<_>>()
                .join(",")
        })
        .collect();
    println!("esa-square: {} distinct strategy outcomes", outcomes.len());
    check_family_with("esa-square", &requests, observe_merge);
}

// esa-rect: ProcessRectACS through the encoder's dispatch -------------------

/// The words of the thread memory an `esa-rect` request prints: the block
/// and the first kMaxCoeffArea scratch words (EstimateEntropy's `mem`).
const MEMORY_WORDS: usize = 4 * MAX_COEFF_AREA;

/// The encoder's dispatch override is process-wide: tests that set it run
/// one at a time.
fn dispatch_lock() -> MutexGuard<'static, ()> {
    static LOCK: Mutex<()> = Mutex::new(());
    LOCK.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn speed_tier(tier: i32) -> SpeedTier {
    match tier {
        0 => SpeedTier::Glacier,
        1 => SpeedTier::Tortoise,
        2 => SpeedTier::Kitten,
        3 => SpeedTier::Squirrel,
        4 => SpeedTier::Wombat,
        5 => SpeedTier::Hare,
        7 => SpeedTier::Falcon,
        other => panic!("speed tier {other}"),
    }
}

/// The port's observation of an `esa-rect` request through the dispatching
/// `AcStrategyHeuristics::process_rect` (the caller sets the encoder's
/// selection).
fn observe_rect(request: &str) -> String {
    let mut args = request.split_whitespace();
    assert_eq!(args.next(), Some("esa-rect"));
    let cparams = CompressParams {
        butteraugli_distance: float(&mut args),
        speed_tier: speed_tier(number(&mut args)),
        decoding_speed_tier: number(&mut args),
        ..CompressParams::default()
    };
    let xsize: usize = number(&mut args);
    let ysize: usize = number(&mut args);
    let mask_xsize: usize = number(&mut args);
    let (bxs, bys) = (xsize / 8, ysize / 8);
    let mut src = Image3F::create(xsize, ysize).unwrap();
    for c in 0..3 {
        for y in 0..ysize {
            for v in &mut src.plane_row_mut(c, y)[..xsize] {
                *v = float(&mut args);
            }
        }
    }
    let mut plane = |xs: usize, ys: usize| {
        let mut p = ImageF::create(xs, ys).unwrap();
        for y in 0..ys {
            for v in &mut p.row_mut(y)[..xs] {
                *v = float(&mut args);
            }
        }
        p
    };
    let qf = plane(bxs, bys);
    let mask = plane(bxs, bys);
    let mask1x1 = plane(mask_xsize, ysize);
    let mut cmap = ColorCorrelationMap::create(xsize, ysize, true).unwrap();
    for map in [&mut cmap.ytox_map, &mut cmap.ytob_map] {
        for y in 0..map.ysize() {
            for x in 0..map.xsize() {
                map.row_mut(y)[x] = number::<i32>(&mut args) as i8;
            }
        }
    }
    let mut ac_strategy = read_layout(&mut args, bxs, bys);
    let rect = Rect::new(0, 0, xsize, ysize);
    // The original's Init runs the target's ComputeQuantTable; the port's
    // follows decoder_target() (scalar outside decoding), so the target's
    // tables are computed first and Init finds them computed.
    let mut matrices = DequantMatrices::default();
    matrices
        .ensure_computed_for_target(MemoryManagerPtr::NATIVE, acs_mask(), Some(target()))
        .unwrap();
    let mut heuristics = AcStrategyHeuristics::new(MemoryManagerPtr::NATIVE, &cparams);
    let mut status = heuristics.init(&src, rect, &qf, &mask, &mask1x1, &mut matrices);
    if status.is_ok() {
        status = heuristics.prepare_for_threads(1);
    }
    let mut out = format!(
        "H {} {} {} {} {}",
        status_code(&status),
        heuristics.config.info_loss_multiplier.to_bits(),
        heuristics.config.cost_delta.to_bits(),
        heuristics.config.zeros_mul.to_bits(),
        heuristics.config.mask1x1_xsize
    );
    if status.is_err() {
        return out;
    }
    // The thread's block and the following kMaxCoeffArea scratch words
    // start as sentinels and are printed after the search: they hold the
    // costs of its last EstimateEntropy calls.
    let memory = heuristics.thread_memory_mut(0).unwrap();
    for (i, v) in memory[..MEMORY_WORDS].iter_mut().enumerate() {
        *v = sentinel(3, i);
    }
    let planes = AcsPlanes {
        src: &src,
        rect_in: rect,
        quant_field: &qf,
        mask: &mask,
        mask1x1: &mask1x1,
        dequant: &matrices,
    };
    for (bx, by, w, h) in tiles(xsize, ysize) {
        if status.is_err() {
            break;
        }
        status =
            heuristics.process_rect(Rect::new(bx, by, w, h), &cmap, &mut ac_strategy, 0, &planes);
    }
    let mut aux = AuxOut::default();
    if status.is_ok() {
        status = heuristics.finalize(&FrameDimensions::default(), &ac_strategy, Some(&mut aux));
    }
    write!(out, " {}", status_code(&status)).unwrap();
    print_strategy(&mut out, &ac_strategy);
    let memory = heuristics.thread_memory_mut(0).unwrap();
    let used = (0..MEMORY_WORDS)
        .rev()
        .find(|&i| memory[i].to_bits() != sentinel(3, i).to_bits())
        .map_or(0, |i| i + 1);
    words(&mut out, "W", &memory[..MEMORY_WORDS.min(used + 4)]);
    write!(
        out,
        " N {} {} {} {} {} {} {} {} {} {} {}",
        aux.num_small_blocks,
        aux.num_dct4x8_blocks,
        aux.num_afv_blocks,
        aux.num_dct8_blocks,
        aux.num_dct8x16_blocks,
        aux.num_dct8x32_blocks,
        aux.num_dct16_blocks,
        aux.num_dct16x32_blocks,
        aux.num_dct32_blocks,
        aux.num_dct32x64_blocks,
        aux.num_dct64_blocks
    )
    .unwrap();
    out
}

/// An `esa-rect` request: the planes of [`inputs`] (whose ACSConfig
/// multipliers Init recomputes, so they are dropped), a positive masking
/// field, a mask1x1 field `mask_xsize` wide and random color-correlation
/// tiles.
#[expect(clippy::too_many_arguments, reason = "one parameter per request field")]
fn rect_request(
    xsize: usize,
    ysize: usize,
    content: Content,
    distance: f32,
    tier: i32,
    decoding: usize,
    mask_xsize: usize,
    seed: u64,
) -> String {
    let planes = inputs(xsize, ysize, content, distance, xsize, seed);
    let mut words: Vec<&str> = planes.split_whitespace().collect();
    words.truncate(words.len() - 4);
    let (xs, ys) = (xsize, ysize);
    let src_and_qf = 2 + 3 * xs * ys + xs / 8 * (ys / 8);
    let mut s = format!(
        "esa-rect {} {tier} {decoding} {xsize} {ysize} {mask_xsize}",
        distance.to_bits()
    );
    for w in &words[2..src_and_qf] {
        write!(s, " {w}").unwrap();
    }
    let mut mseed = seed | 1;
    let mask: Vec<f32> = (0..xs / 8 * (ys / 8))
        .map(|_| 0.1 + unit(&mut mseed))
        .collect();
    push_floats(&mut s, &mask);
    // The mask1x1 rows: the first `mask_xsize` samples of each row.
    for y in 0..ys {
        for w in &words[src_and_qf + y * xs..src_and_qf + y * xs + mask_xsize] {
            write!(s, " {w}").unwrap();
        }
    }
    let tiles = xs.div_ceil(64) * ys.div_ceil(64);
    for _ in 0..2 * tiles {
        write!(s, " {}", (next(&mut mseed) % 81) as i32 - 40).unwrap();
    }
    s.push_str(" 0");
    s
}

#[test]
fn enc_simd_process_rect_acs_through_the_encoder_dispatch_matches_the_original_target() {
    let sizes: [(usize, usize); 6] = [(8, 8), (24, 40), (64, 64), (136, 72), (72, 136), (200, 136)];
    let tiers = [0i32, 1, 2, 3, 4, 5, 7];
    let distances = [0.5f32, 1.0, 2.9, 4.5, 13.0];
    let mut requests = Vec::new();
    for (si, &(xsize, ysize)) in sizes.iter().enumerate() {
        for (ci, &content) in CONTENTS.iter().enumerate() {
            for (di, &distance) in distances.iter().enumerate() {
                let variant = si * 7 + ci * 3 + di;
                let tier = tiers[variant % tiers.len()];
                let decoding = [0usize, 0, 1, 2, 4][variant % 5];
                let mask_xsize = [xsize, xsize, xsize - 3, xsize - 4][variant % 4];
                requests.push(rect_request(
                    xsize,
                    ysize,
                    content,
                    distance,
                    tier,
                    decoding,
                    mask_xsize,
                    variant as u64 + 900,
                ));
            }
        }
    }
    // Every speed and decoding tier on one larger image.
    for tier in tiers {
        for decoding in [0usize, 1, 2, 4] {
            requests.push(rect_request(
                136,
                136,
                Content::Smooth,
                1.0,
                tier,
                decoding,
                136,
                tier as u64 * 10 + decoding as u64,
            ));
        }
    }
    let _lock = dispatch_lock();
    hwy::set_encoder_simd_for_test(Some(false));
    let scalar: Vec<String> = requests.iter().map(|r| observe_rect(r)).collect();
    hwy::set_encoder_simd_for_test(Some(true));
    let simd: Vec<String> = requests.iter().map(|r| observe_rect(r)).collect();
    hwy::set_encoder_simd_for_test(None);
    let differing = scalar.iter().zip(&simd).filter(|(a, b)| a != b).count();
    assert!(
        differing > 0,
        "esa-rect: the scalar and SIMD paths chose the same strategies everywhere"
    );
    println!(
        "esa-rect: the scalar path differs in {differing} of {} requests",
        requests.len()
    );
    // The decisions cover the strategies the search can choose.
    let chosen: std::collections::BTreeSet<u32> = simd
        .iter()
        .flat_map(|out| {
            out.split_whitespace()
                .filter(|w| w.ends_with(":1"))
                .map(|w| w.split(':').next().unwrap().parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();
    println!("esa-rect: chosen strategies {chosen:?}");
    assert!(chosen.len() >= 12, "esa-rect chose only {chosen:?}");
    let observed = simd_common::oracle(&requests);
    let mut words = 0;
    for ((request, rust), cpp) in requests.iter().zip(&simd).zip(&observed) {
        words += simd_common::same(rust, cpp, strategy_float(rust), all_defined)
            .unwrap_or_else(|e| panic!("esa-rect: {e}\nrequest: {request}"));
    }
    println!(
        "esa-rect on {}: {} requests, {words} words matched",
        target().name(),
        requests.len()
    );
}

// The 8-lane structure ------------------------------------------------------

/// The 8-lane structure of the AC strategy search, exercised natively on
/// AArch64. `Neon8` is the 8-lane descriptor of tests/simd_vardct_lanes8.rs
/// (two NEON vectors, every lane-wise operation NEON's on each half), with
/// `sum_of_lanes` in Highway's AVX2 reduction order. EstimateEntropy depends
/// on the lane count (the coefficient loop step, the masking bound
/// `x + ix * 8 + dx + Lanes(df8) <= mask1x1_xsize`, the lane accumulators
/// and their reductions), so its 8-lane result differs from NEON's 4-lane
/// one by design; it is compared with a lane-explicit model written from
/// the original's loop for eight lanes over the same transforms. This is
/// not AVX2 parity, which only the AVX2 oracle run shows.
#[cfg(target_arch = "aarch64")]
mod lanes8 {
    use super::*;
    use core::arch::aarch64::*;
    use jxl::jxl::base::bits::ceil_log2_nonzero;
    use jxl::jxl::dec_transforms_simd::transform_to_pixels_d;
    use jxl::jxl::enc_ac_strategy_simd::{estimate_entropy_d, process_rect_acs_d};
    use jxl::jxl::enc_transforms_simd::transform_from_pixels_d;
    use jxl::jxl::hwy::{Neon, SimdF32};
    use jxl::jxl::quant_weights_simd::fast_powf_simd;

    #[derive(Clone, Copy, Debug)]
    struct Neon8(Neon);

    macro_rules! halves {
        ($s:ident, $f:ident, $($a:ident),*) => {
            [$s.0.$f($($a[0]),*), $s.0.$f($($a[1]),*)]
        };
    }

    impl SimdF32 for Neon8 {
        type V = [float32x4_t; 2];
        type I = [int32x4_t; 2];
        type M = [uint32x4_t; 2];
        const LANES: usize = 8;
        const NAME: &'static str = "NEON x2";
        fn set(self, v: f32) -> Self::V {
            [self.0.set(v), self.0.set(v)]
        }
        fn zero(self) -> Self::V {
            [self.0.zero(), self.0.zero()]
        }
        fn load(self, src: &[f32]) -> Self::V {
            let src = &src[..8];
            [self.0.load(src), self.0.load(&src[4..])]
        }
        fn store(self, v: Self::V, dst: &mut [f32]) {
            let dst = &mut dst[..8];
            self.0.store(v[0], dst);
            self.0.store(v[1], &mut dst[4..]);
        }
        fn load_bits(self, src: &[u32]) -> Self::V {
            let src = &src[..8];
            [self.0.load_bits(src), self.0.load_bits(&src[4..])]
        }
        fn store_bits(self, v: Self::V, dst: &mut [u32]) {
            let dst = &mut dst[..8];
            self.0.store_bits(v[0], dst);
            self.0.store_bits(v[1], &mut dst[4..]);
        }
        fn store_i32(self, v: Self::I, dst: &mut [i32]) {
            let dst = &mut dst[..8];
            self.0.store_i32(v[0], dst);
            self.0.store_i32(v[1], &mut dst[4..]);
        }
        fn get_lane(self, v: Self::V) -> f32 {
            self.0.get_lane(v[0])
        }
        fn add(self, a: Self::V, b: Self::V) -> Self::V {
            halves!(self, add, a, b)
        }
        fn sub(self, a: Self::V, b: Self::V) -> Self::V {
            halves!(self, sub, a, b)
        }
        fn mul(self, a: Self::V, b: Self::V) -> Self::V {
            halves!(self, mul, a, b)
        }
        fn div(self, a: Self::V, b: Self::V) -> Self::V {
            halves!(self, div, a, b)
        }
        fn mul_add(self, m: Self::V, x: Self::V, a: Self::V) -> Self::V {
            halves!(self, mul_add, m, x, a)
        }
        fn neg_mul_add(self, m: Self::V, x: Self::V, a: Self::V) -> Self::V {
            halves!(self, neg_mul_add, m, x, a)
        }
        fn mul_sub(self, m: Self::V, x: Self::V, a: Self::V) -> Self::V {
            halves!(self, mul_sub, m, x, a)
        }
        fn neg_mul_sub(self, m: Self::V, x: Self::V, a: Self::V) -> Self::V {
            halves!(self, neg_mul_sub, m, x, a)
        }
        fn min(self, a: Self::V, b: Self::V) -> Self::V {
            halves!(self, min, a, b)
        }
        fn max(self, a: Self::V, b: Self::V) -> Self::V {
            halves!(self, max, a, b)
        }
        fn abs(self, v: Self::V) -> Self::V {
            halves!(self, abs, v)
        }
        fn neg(self, v: Self::V) -> Self::V {
            halves!(self, neg, v)
        }
        fn abs_diff(self, a: Self::V, b: Self::V) -> Self::V {
            halves!(self, abs_diff, a, b)
        }
        fn sqrt(self, v: Self::V) -> Self::V {
            halves!(self, sqrt, v)
        }
        fn approximate_reciprocal(self, v: Self::V) -> Self::V {
            halves!(self, approximate_reciprocal, v)
        }
        fn approximate_reciprocal_sqrt(self, v: Self::V) -> Self::V {
            halves!(self, approximate_reciprocal_sqrt, v)
        }
        fn floor(self, v: Self::V) -> Self::V {
            halves!(self, floor, v)
        }
        fn ceil(self, v: Self::V) -> Self::V {
            halves!(self, ceil, v)
        }
        fn round(self, v: Self::V) -> Self::V {
            halves!(self, round, v)
        }
        fn trunc(self, v: Self::V) -> Self::V {
            halves!(self, trunc, v)
        }
        fn zero_if_negative(self, v: Self::V) -> Self::V {
            halves!(self, zero_if_negative, v)
        }
        fn copy_sign(self, a: Self::V, b: Self::V) -> Self::V {
            halves!(self, copy_sign, a, b)
        }
        fn copy_sign_to_abs(self, a: Self::V, b: Self::V) -> Self::V {
            halves!(self, copy_sign_to_abs, a, b)
        }
        fn and(self, a: Self::V, b: Self::V) -> Self::V {
            halves!(self, and, a, b)
        }
        fn or(self, a: Self::V, b: Self::V) -> Self::V {
            halves!(self, or, a, b)
        }
        fn xor(self, a: Self::V, b: Self::V) -> Self::V {
            halves!(self, xor, a, b)
        }
        fn and_not(self, a: Self::V, b: Self::V) -> Self::V {
            halves!(self, and_not, a, b)
        }
        fn lt(self, a: Self::V, b: Self::V) -> Self::M {
            halves!(self, lt, a, b)
        }
        fn le(self, a: Self::V, b: Self::V) -> Self::M {
            halves!(self, le, a, b)
        }
        fn gt(self, a: Self::V, b: Self::V) -> Self::M {
            halves!(self, gt, a, b)
        }
        fn ge(self, a: Self::V, b: Self::V) -> Self::M {
            halves!(self, ge, a, b)
        }
        fn eq(self, a: Self::V, b: Self::V) -> Self::M {
            halves!(self, eq, a, b)
        }
        fn ne(self, a: Self::V, b: Self::V) -> Self::M {
            halves!(self, ne, a, b)
        }
        fn vec_from_mask(self, m: Self::M) -> Self::V {
            halves!(self, vec_from_mask, m)
        }
        fn if_then_else(self, m: Self::M, y: Self::V, n: Self::V) -> Self::V {
            halves!(self, if_then_else, m, y, n)
        }
        fn if_then_zero_else(self, m: Self::M, n: Self::V) -> Self::V {
            halves!(self, if_then_zero_else, m, n)
        }
        fn if_then_else_zero(self, m: Self::M, y: Self::V) -> Self::V {
            halves!(self, if_then_else_zero, m, y)
        }
        fn convert_to_i32(self, v: Self::V) -> Self::I {
            halves!(self, convert_to_i32, v)
        }
        fn convert_to_f32(self, v: Self::I) -> Self::V {
            halves!(self, convert_to_f32, v)
        }
        fn nearest_int(self, v: Self::V) -> Self::I {
            halves!(self, nearest_int, v)
        }
        fn bitcast_to_i32(self, v: Self::V) -> Self::I {
            halves!(self, bitcast_to_i32, v)
        }
        fn bitcast_to_f32(self, v: Self::I) -> Self::V {
            halves!(self, bitcast_to_f32, v)
        }
        fn add_i32(self, a: Self::I, b: Self::I) -> Self::I {
            halves!(self, add_i32, a, b)
        }
        fn sub_i32(self, a: Self::I, b: Self::I) -> Self::I {
            halves!(self, sub_i32, a, b)
        }
        fn min_i32(self, a: Self::I, b: Self::I) -> Self::I {
            halves!(self, min_i32, a, b)
        }
        fn max_i32(self, a: Self::I, b: Self::I) -> Self::I {
            halves!(self, max_i32, a, b)
        }
        fn lt_i32(self, a: Self::I, b: Self::I) -> Self::I {
            halves!(self, lt_i32, a, b)
        }
        fn eq_i32(self, a: Self::I, b: Self::I) -> Self::I {
            halves!(self, eq_i32, a, b)
        }
        fn gt_i32(self, a: Self::I, b: Self::I) -> Self::I {
            halves!(self, gt_i32, a, b)
        }
        fn shift_left_i32<const B: i32>(self, v: Self::I) -> Self::I {
            [
                self.0.shift_left_i32::<B>(v[0]),
                self.0.shift_left_i32::<B>(v[1]),
            ]
        }
        fn shift_right_i32<const B: i32>(self, v: Self::I) -> Self::I {
            [
                self.0.shift_right_i32::<B>(v[0]),
                self.0.shift_right_i32::<B>(v[1]),
            ]
        }
        fn shift_right_u32<const B: i32>(self, v: Self::I) -> Self::I {
            [
                self.0.shift_right_u32::<B>(v[0]),
                self.0.shift_right_u32::<B>(v[1]),
            ]
        }
        // Highway's AVX2 SumOfLanes (x86_256-inl.h): the two 128-bit halves
        // added lane-wise, then (s0 + s3) + (s1 + s2) within the block.
        fn sum_of_lanes(self, v: Self::V) -> Self::V {
            let mut a = [0.0f32; 8];
            self.store(v, &mut a);
            let s: [f32; 4] = std::array::from_fn(|j| a[j] + a[j + 4]);
            self.set((s[0] + s[3]) + (s[1] + s[2]))
        }
        fn max_of_lanes(self, _: Self::V) -> Self::V {
            unimplemented!("lane-crossing")
        }
        fn min_of_lanes(self, _: Self::V) -> Self::V {
            unimplemented!("lane-crossing")
        }
        fn interleave_lower(self, _: Self::V, _: Self::V) -> Self::V {
            unimplemented!("lane-crossing")
        }
        fn interleave_upper(self, _: Self::V, _: Self::V) -> Self::V {
            unimplemented!("lane-crossing")
        }
        fn store_interleaved2(self, _: Self::V, _: Self::V, _: &mut [f32]) {
            unimplemented!("lane-crossing")
        }
        fn store_interleaved3(self, _: Self::V, _: Self::V, _: Self::V, _: &mut [f32]) {
            unimplemented!("lane-crossing")
        }
        fn load_dup128(self, _: &[f32]) -> Self::V {
            unimplemented!("lane-crossing")
        }
        fn broadcast<const LANE: i32>(self, _: Self::V) -> Self::V {
            unimplemented!("lane-crossing")
        }
        fn store_demote_to_f16(self, _: Self::V, _: &mut [u16]) {
            unimplemented!("not used by the VarDCT kernels")
        }
        // Signed 32-bit operations of the modular and stages2 packages, each half
        // through the NEON implementation. The VarDCT kernels do not use them;
        // they are here because the layer trait requires them.
        fn set_i32(self, v: i32) -> Self::I {
            [self.0.set_i32(v), self.0.set_i32(v)]
        }
        fn load_i32(self, src: &[i32]) -> Self::I {
            let src = &src[..8];
            [self.0.load_i32(src), self.0.load_i32(&src[4..])]
        }
        fn and_i32(self, a: Self::I, b: Self::I) -> Self::I {
            halves!(self, and_i32, a, b)
        }
        fn xor_i32(self, a: Self::I, b: Self::I) -> Self::I {
            halves!(self, xor_i32, a, b)
        }
        fn abs_i32(self, v: Self::I) -> Self::I {
            halves!(self, abs_i32, v)
        }
        fn neg_i32(self, v: Self::I) -> Self::I {
            halves!(self, neg_i32, v)
        }
        fn ne_i32(self, a: Self::I, b: Self::I) -> Self::I {
            halves!(self, ne_i32, a, b)
        }
        fn if_then_else_i32(self, m: Self::I, yes: Self::I, no: Self::I) -> Self::I {
            halves!(self, if_then_else_i32, m, yes, no)
        }
        fn if_then_zero_else_i32(self, m: Self::I, no: Self::I) -> Self::I {
            halves!(self, if_then_zero_else_i32, m, no)
        }
        fn mul_i32(self, a: Self::I, b: Self::I) -> Self::I {
            halves!(self, mul_i32, a, b)
        }
        // Block-wise on AVX2, so one NEON operation per 128-bit half gives the
        // AVX2 block structure (Highway x86_256-inl.h at the pinned revision):
        // MulEven is `_mm256_mul_epi32`, which multiplies the low 32 bits of each
        // 64-bit element (lanes 0, 2 | 4, 6); MulOdd for 32-bit lanes is
        // `MulEven(DupOdd(a), DupOdd(b))` (x86_128-inl.h), DupOdd shuffling
        // within each block; InterleaveOdd (non-AVX3 path) is two
        // `_mm256_shuffle_ps`, each per 128-bit lane, giving a1 b1 a3 b3 | a5 b5
        // a7 b7; TableLookupBytes is `_mm256_shuffle_epi8`, which looks up within
        // each 128-bit lane. Per half, the byte-index rule stays NEON's
        // (`vqtbl1q_u8`), as every operation of this descriptor does.
        fn mul_even_i32(self, a: Self::I, b: Self::I) -> Self::I {
            halves!(self, mul_even_i32, a, b)
        }
        fn mul_odd_i32(self, a: Self::I, b: Self::I) -> Self::I {
            halves!(self, mul_odd_i32, a, b)
        }
        fn interleave_odd_i32(self, a: Self::I, b: Self::I) -> Self::I {
            halves!(self, interleave_odd_i32, a, b)
        }
        fn table_lookup_bytes(self, bytes: Self::I, from: Self::I) -> Self::I {
            halves!(self, table_lookup_bytes, bytes, from)
        }
    }

    /// The original EstimateEntropy for eight lanes, lane by lane.
    fn model(
        acs: &AcStrategy,
        entropy_mul: f32,
        x: usize,
        y: usize,
        config: &AcsConfig<'_>,
        cmap_factors: &[f32; 3],
    ) -> f32 {
        const L: usize = 8;
        let d = Neon8(Neon::new());
        let mut block = vec![0.0f32; BLOCK_WORDS];
        let mut scratch = vec![0.0f32; SCRATCH_WORDS];
        let size = (1usize << acs.log2_covered_blocks()) * 64;
        for c in 0..3 {
            transform_from_pixels_d(
                d,
                acs.strategy(),
                config.pixel(c, x, y),
                config.src_stride,
                &mut block[size * c..],
                &mut scratch[MAX_COEFF_AREA..],
            );
        }
        let (cbx, cby) = (acs.covered_blocks_x(), acs.covered_blocks_y());
        let num_blocks = cbx * cby;
        let quant_norm16 = if num_blocks == 1 {
            config.quant(x / 8, y / 8)
        } else if num_blocks == 2 {
            let other = if cby == 2 {
                config.quant(x / 8, y / 8 + 1)
            } else {
                config.quant(x / 8 + 1, y / 8)
            };
            let first = config.quant(x / 8, y / 8);
            if first < other { other } else { first }
        } else {
            let mut sum = 0.0f32;
            for iy in 0..cby {
                for ix in 0..cbx {
                    let q = config.quant(x / 8 + ix, y / 8 + iy);
                    let q8 = (q * q) * (q * q);
                    let q8 = q8 * q8;
                    sum = q8.mul_add(q8, sum);
                }
            }
            fast_powf_simd(
                Target::detect().unwrap(),
                sum / num_blocks as f32,
                1.0 / 16.0,
            )
        };
        let reduce = |a: [f32; L]| {
            let s: [f32; 4] = std::array::from_fn(|j| a[j] + a[j + 4]);
            (s[0] + s[3]) + (s[1] + s[2])
        };
        let mut entropy = 0.0f32;
        let mut loss = [0.0f32; L];
        for c in 0..3 {
            let inv = config.dequant.inv_matrix(acs.strategy(), c);
            let matrix = config.dequant.matrix(acs.strategy(), c);
            let mut entropy_v = [0.0f32; L];
            let mut nzeros_v = [0.0f32; L];
            let (mem, rest) = scratch.split_at_mut(MAX_COEFF_AREA);
            for i in 0..num_blocks * 64 {
                let val = (block[c * size + i] - block[size + i] * cmap_factors[c])
                    * (inv[i] * quant_norm16);
                let rval = val.round_ties_even();
                mem[i] = matrix[i] * (val - rval);
                let q = rval.abs();
                entropy_v[i % L] += q.sqrt();
                nzeros_v[i % L] += if q == 0.0 { 0.0 } else { 1.0 };
            }
            transform_to_pixels_d(d, acs.strategy(), mem, &mut block, cbx * 8, rest);
            let off = [12.0f32, 0.0, 4.0][c];
            let mut lossc = [0.0f32; L];
            for iy in 0..cby {
                for ix in 0..cbx {
                    for dy in 0..8 {
                        // One vector of eight lanes per block row.
                        if x + ix * 8 + L <= config.mask1x1_xsize {
                            let mask = config.masking_ptr1x1(x + ix * 8, y + iy * 8 + dy);
                            for lane in 0..L {
                                let v = block[(iy * 8 + dy) * (cbx * 8) + ix * 8 + lane];
                                let mut v = (mask[lane] + off) * v;
                                v *= v;
                                v *= v;
                                v *= v;
                                lossc[lane] += v;
                            }
                        }
                    }
                }
            }
            let mul = [8.2f64.powf(8.0), 1.0f64.powf(8.0), 1.03f64.powf(8.0)][c] as f32;
            for lane in 0..L {
                loss[lane] += mul * lossc[lane];
            }
            entropy = config.cost_delta.mul_add(reduce(entropy_v), entropy);
            let nbits = ceil_log2_nonzero(reduce(nzeros_v) as usize + 1) + 1;
            entropy = config
                .zeros_mul
                .mul_add((ceil_log2_nonzero(nbits + 17) + nbits) as f32, entropy);
            if c == 0 && num_blocks >= 2 {
                let w = (1.0 + 3.0f64.min(num_blocks as f64 / 8.0)) as f32;
                entropy *= w;
                for l in &mut loss {
                    *l *= w;
                }
            }
        }
        let n = (num_blocks * 64) as f32;
        let loss_scalar = (reduce(loss) / n).powf(1.0 / 8.0) * n / quant_norm16;
        entropy *= entropy_mul;
        config.info_loss_multiplier.mul_add(loss_scalar, entropy)
    }

    #[test]
    fn eight_lane_estimate_entropy_follows_the_original_loop_structure() {
        let d = Neon8(Neon::new());
        let mut cases = 0;
        let mut four_differs = 0;
        for raw in 0..AcStrategyType::DCT128X128.0 {
            let acs = AcStrategy::from_raw_strategy(AcStrategyType(raw));
            let (w, h) = (8 * acs.covered_blocks_x(), 8 * acs.covered_blocks_y());
            for (si, &(xsize, ysize)) in SIZES.iter().enumerate() {
                if w > xsize || h > ysize {
                    continue;
                }
                let (x, y) = (xsize - w, (ysize - h) / 16 * 8);
                // Every cut of the masking width around the strategy's
                // columns: 4- and 8-lane bounds differ at x + 4 .. x + 7.
                for (mi, m) in (0..=8).map(|k| x + k).chain([x + w - 3, xsize]).enumerate() {
                    let content = CONTENTS[(si + mi + raw as usize) % 4];
                    let request = inputs(
                        xsize,
                        ysize,
                        content,
                        1.0,
                        m.min(xsize),
                        u64::from(raw) * 31 + mi as u64,
                    );
                    let mut args = request.split_whitespace();
                    let inputs = Inputs::read(&mut args);
                    let planes = inputs.planes();
                    let config = AcsConfig::new(&inputs.scalars, &planes);
                    let cmap = CMAPS[mi % 3];
                    let mut block = vec![0.0f32; BLOCK_WORDS + MAX_LANES];
                    let mut scratch = vec![0.0f32; SCRATCH_WORDS];
                    let mut quantized = vec![0u32; MAX_COEFF_AREA];
                    let mut eight = 0.0f32;
                    estimate_entropy_d(
                        d,
                        &acs,
                        1.21,
                        x,
                        y,
                        &config,
                        &cmap,
                        &mut block,
                        &mut scratch,
                        &mut quantized,
                        &mut eight,
                    )
                    .unwrap();
                    let expected = model(&acs, 1.21, x, y, &config, &cmap);
                    assert_eq!(
                        eight.to_bits(),
                        expected.to_bits(),
                        "strategy {raw}, image {xsize}x{ysize}, mask1x1_xsize {m}"
                    );
                    let mut four = 0.0f32;
                    estimate_entropy_d(
                        Neon::new(),
                        &acs,
                        1.21,
                        x,
                        y,
                        &config,
                        &cmap,
                        &mut block,
                        &mut scratch,
                        &mut quantized,
                        &mut four,
                    )
                    .unwrap();
                    four_differs += usize::from(four.to_bits() != eight.to_bits());
                    cases += 1;
                }
            }
        }
        println!(
            "8-lane EstimateEntropy: {cases} cases equal the model, {four_differs} differ from 4 lanes"
        );
        assert!(four_differs > 0);
    }

    /// The search above EstimateEntropy runs with eight lanes inside the
    /// buffers PrepareForThreads sizes (no slice bound is hit).
    #[test]
    fn eight_lane_process_rect_acs_fits_its_buffers() {
        let d = Neon8(Neon::new());
        for (xsize, ysize) in [(136usize, 72usize), (24, 40)] {
            let request = inputs(xsize, ysize, Content::Smooth, 1.0, xsize - 3, 5);
            let mut args = request.split_whitespace();
            let inputs = Inputs::read(&mut args);
            let planes = inputs.planes();
            let config = AcsConfig::new(&inputs.scalars, &planes);
            let cmap = ColorCorrelationMap::create(xsize, ysize, true).unwrap();
            let mut ac_strategy = AcStrategyImage::create(xsize / 8, ysize / 8).unwrap();
            let mut memory = vec![0.0f32; 6 * MAX_COEFF_AREA + 3 * MAX_LANES * MAX_BLOCK_DIM];
            let mut quantized = vec![0u32; MAX_COEFF_AREA];
            for tier in [0, 3] {
                let cparams = CompressParams {
                    speed_tier: speed_tier(tier),
                    ..CompressParams::default()
                };
                for (bx, by, w, h) in tiles(xsize, ysize) {
                    process_rect_acs_d(
                        d,
                        &cparams,
                        &config,
                        Rect::new(bx, by, w, h),
                        &cmap,
                        &mut memory,
                        &mut quantized,
                        &mut ac_strategy,
                    )
                    .unwrap();
                }
            }
        }
    }
}
