// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! SIMD parity of the encoder's chroma-from-luma Highway exports (port ADR
//! 0004, "Encoder path"): the port's NEON/AVX2 instantiations against the
//! original built for the same single Highway target (`JXL_CPP_ORACLE_SIMD`),
//! through the `ecf-*` requests of tools/oracle/encoder_cfl_simd.cc. Every
//! word the original defines is compared bit for bit, except that any NaN
//! equals any NaN (decision D8). The requests go through the dispatching
//! entry points with the encoder's selection (`hwy::encoder_target`) forced
//! to the SIMD target; each family also shows that the scalar path gives
//! different words for some of its requests, so the comparison tells the two
//! paths apart.
#![cfg(feature = "cpp-oracle-simd")]

mod simd_common;

use jxl::jxl::base::rect::Rect;
use jxl::jxl::base::status::status_code;
use jxl::jxl::enc_chroma_from_luma::CfLHeuristics;
use jxl::jxl::hwy;
use jxl::jxl::memory_manager_internal::MemoryManagerPtr;
use simd_common::{check_rows, target};
use std::fmt::Write as _;
use std::sync::{Mutex, MutexGuard};

type Args<'a> = std::str::SplitWhitespace<'a>;

/// The encoder's dispatch override is process-wide: tests that set it run
/// one at a time.
fn dispatch_lock() -> MutexGuard<'static, ()> {
    static LOCK: Mutex<()> = Mutex::new(());
    LOCK.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn number<T: std::str::FromStr>(args: &mut Args<'_>) -> T
where
    T::Err: std::fmt::Debug,
{
    args.next().unwrap().parse().unwrap()
}
fn word(out: &mut String, v: f32) {
    write!(out, " {}", v.to_bits()).unwrap();
}
fn all_float(_: usize) -> bool {
    true
}
fn all_defined(_: usize) -> bool {
    true
}

/// Runs `requests` through `observe` with the scalar and then the SIMD
/// encoder selection, asserts that the scalar path differs somewhere, and
/// compares the SIMD observations with the original's target.
fn check_through_dispatch(family: &str, requests: &[String], observe: fn(&str) -> String) {
    let _lock = dispatch_lock();
    hwy::set_encoder_simd_for_test(Some(false));
    let scalar: Vec<String> = requests.iter().map(|r| observe(r)).collect();
    hwy::set_encoder_simd_for_test(Some(true));
    let simd: Vec<String> = requests.iter().map(|r| observe(r)).collect();
    hwy::set_encoder_simd_for_test(None);
    let differing = scalar.iter().zip(&simd).filter(|(a, b)| a != b).count();
    assert!(
        differing > 0,
        "{family}: the scalar and SIMD paths gave the same words everywhere"
    );
    println!(
        "{family}: the scalar path differs in {differing} of {} requests",
        requests.len()
    );
    let index = std::cell::Cell::new(0);
    check_rows(
        family,
        requests,
        |_| {
            let observed = simd[index.get()].clone();
            index.set(index.get() + 1);
            (observed, Box::new(all_defined))
        },
        all_float,
    );
}

// InitDCStorage --------------------------------------------------------------

/// The port's observation of an `ecf-init` request through the dispatching
/// `CfLHeuristics::init` (the caller sets the encoder's selection).
fn observe_init(request: &str) -> String {
    let mut args = request.split_whitespace();
    assert_eq!(args.next(), Some("ecf-init"));
    let xsize: usize = number(&mut args);
    let ysize: usize = number(&mut args);
    let mut cfl = CfLHeuristics::new(MemoryManagerPtr::NATIVE);
    let status = cfl.init(Rect::new(0, 0, xsize, ysize));
    // MaxVectorSize() / sizeof(float) of the original's single target.
    let lanes = target().lanes();
    let mut out = format!("I {} {lanes}", status_code(&status));
    if status.is_err() {
        return out;
    }
    let dc = &cfl.dc_values;
    write!(out, " {} {}", dc.xsize(), dc.ysize()).unwrap();
    for y in 0..dc.ysize() {
        out.push_str(" R");
        for &v in &dc.row(y)[dc.xsize() - dc.xsize().min(lanes)..dc.xsize()] {
            word(&mut out, v);
        }
    }
    out
}

#[test]
fn enc_cfl_simd_init_dc_storage_through_the_encoder_dispatch_matches_the_original_target() {
    let mut requests = Vec::new();
    // Every block count from 0 to 60 in several shapes, pixel sizes that are
    // and are not multiples of the block size, and photograph-sized frames.
    for xb in 0..=12usize {
        for yb in 0..=5usize {
            for extra in [0usize, 7] {
                requests.push(format!(
                    "ecf-init {} {}",
                    xb * 8 + extra,
                    yb * 8 + extra % 3
                ));
            }
        }
    }
    for (x, y) in [
        (1, 1),
        (7, 9),
        (9, 7),
        (257, 31),
        (403, 302),
        (2048, 2048),
        (2268, 1512),
        (4033, 9),
    ] {
        requests.push(format!("ecf-init {x} {y}"));
    }
    check_through_dispatch("ecf-init", &requests, observe_init);
}

// FindBestMultiplier and ComputeTile ----------------------------------------

use jxl::jxl::ac_strategy::{AcStrategy, AcStrategyImage, AcStrategyType, NUM_VALID_STRATEGIES};
use jxl::jxl::chroma_from_luma::ColorCorrelationMap;
use jxl::jxl::enc_chroma_from_luma_simd::find_best_multiplier_simd;
use jxl::jxl::image::{Image3F, ImageF, ImageI};
use jxl::jxl::quant_weights::DequantMatrices;
use jxl::jxl::quantizer::Quantizer;
use simd_common::{next, special_words};

fn float(args: &mut Args<'_>) -> f32 {
    f32::from_bits(number(args))
}
fn push_words(s: &mut String, values: &[f32]) {
    for v in values {
        write!(s, " {}", v.to_bits()).unwrap();
    }
}
/// The finite sentinel words of tools/oracle/encoder_cfl_simd.cc.
fn sentinel(salt: u32, i: usize) -> f32 {
    f32::from_bits(0x5a00_0000 + salt * 0x10_0000 + (i as u32 & 0xf_ffff))
}
fn uniform(seed: &mut u64, lo: f32, hi: f32) -> f32 {
    lo + (next(seed) % (1 << 24)) as f32 / (1u32 << 24) as f32 * (hi - lo)
}

/// Luma and chroma coefficient pairs of kind 0 (small noise), 1 (chroma a
/// multiple of luma, the multiplier swept by `seed`), 2 (magnitudes around
/// CFLFunction's threshold of 100), 3 (wide range), 4 (zeros with sparse
/// impulses), 5 (the special words spread in both).
fn pair_values(num: usize, kind: usize, seed: u64) -> (Vec<f32>, Vec<f32>) {
    let specials = special_words();
    let mut seed = seed.wrapping_mul(0x9e37_79b9_7f4a_7c15) | 1;
    let k = ((seed >> 7) % 401) as f32 / 4.0 - 50.0;
    let mut m = Vec::with_capacity(num);
    let mut s = Vec::with_capacity(num);
    for i in 0..num {
        let (a, b) = match kind {
            0 => (uniform(&mut seed, -1.0, 1.0), uniform(&mut seed, -1.0, 1.0)),
            1 => {
                let a = uniform(&mut seed, -30.0, 30.0);
                (a, a * k / 84.0 + uniform(&mut seed, -0.5, 0.5))
            }
            2 => (
                uniform(&mut seed, -300.0, 300.0),
                uniform(&mut seed, -300.0, 300.0),
            ),
            3 => (
                uniform(&mut seed, -1.0, 1.0) * 10f32.powi((next(&mut seed) % 9) as i32 - 4),
                uniform(&mut seed, -1.0, 1.0) * 10f32.powi((next(&mut seed) % 9) as i32 - 4),
            ),
            4 => {
                if next(&mut seed).is_multiple_of(17) {
                    (
                        uniform(&mut seed, -50.0, 50.0),
                        uniform(&mut seed, -50.0, 50.0),
                    )
                } else {
                    (0.0, 0.0)
                }
            }
            _ => {
                let a = uniform(&mut seed, -5.0, 5.0);
                let b = uniform(&mut seed, -5.0, 5.0);
                if i % 7 == 3 {
                    (f32::from_bits(specials[(i / 7) % specials.len()]), b)
                } else if i % 11 == 5 {
                    (a, f32::from_bits(specials[(i / 11 + 3) % specials.len()]))
                } else {
                    (a, b)
                }
            }
        };
        m.push(a);
        s.push(b);
    }
    (m, s)
}

fn observe_best(request: &str) -> String {
    let mut args = request.split_whitespace();
    assert_eq!(args.next(), Some("ecf-best"));
    let num: usize = number(&mut args);
    let base = float(&mut args);
    let distance_mul = float(&mut args);
    let fast: i32 = number(&mut args);
    let m: Vec<f32> = (0..num).map(|_| float(&mut args)).collect();
    let s: Vec<f32> = (0..num).map(|_| float(&mut args)).collect();
    let multiplier =
        find_best_multiplier_simd(target(), &m, &s, num, base, distance_mul, fast != 0);
    format!("M {multiplier}")
}

#[test]
fn enc_cfl_simd_find_best_multiplier_matches_the_original_target() {
    let mut requests = Vec::new();
    let distance_muls = [1e-9f32, 0.0, 1e-4, 0.01, 1.0, 50.0];
    for (ni, num) in [8usize, 16, 24, 64, 128, 512, 1024, 4096]
        .into_iter()
        .enumerate()
    {
        for kind in 0..6 {
            for (di, &distance_mul) in distance_muls.iter().enumerate() {
                for base in [0.0f32, 1.0] {
                    for fast in [0, 1] {
                        // Fewer variants for the largest searches.
                        if num >= 1024 && (di % 2 == 1 || base != 0.0 && kind % 2 == 0) {
                            continue;
                        }
                        let seed = (ni * 1000 + kind * 100 + di * 10) as u64 + fast as u64;
                        let (m, s) = pair_values(num, kind, seed);
                        let mut r = format!(
                            "ecf-best {num} {} {} {fast}",
                            base.to_bits(),
                            distance_mul.to_bits()
                        );
                        push_words(&mut r, &m);
                        push_words(&mut r, &s);
                        requests.push(r);
                    }
                }
            }
        }
    }
    check_rows(
        "ecf-best",
        &requests,
        |r| (observe_best(r), Box::new(all_defined)),
        |_| false,
    );
}

/// The default matrices with the target's ComputeQuantTable, as the
/// original's single-target build computes them.
fn simd_matrices() -> DequantMatrices {
    let mut matrices = DequantMatrices::default();
    matrices
        .ensure_computed_for_target(MemoryManagerPtr::NATIVE, !0, Some(target()))
        .unwrap();
    matrices
}

/// The port's observation of an `ecf-tile` request through the dispatching
/// `CfLHeuristics` (the caller sets the encoder's selection).
fn observe_tile(request: &str) -> String {
    let mut args = request.split_whitespace();
    assert_eq!(args.next(), Some("ecf-tile"));
    let xb: usize = number(&mut args);
    let yb: usize = number(&mut args);
    let x0: usize = number(&mut args);
    let y0: usize = number(&mut args);
    let pass: i32 = number(&mut args);
    let fast: i32 = number(&mut args);
    let quant_dc = float(&mut args);
    let quant_median = float(&mut args);
    let (xsize, ysize) = (x0 + 8 * xb + 3, y0 + 8 * yb);
    let mut acs = AcStrategyImage::create(xb, yb).unwrap();
    acs.fill_dct8_full();
    let n: usize = number(&mut args);
    for _ in 0..n {
        let bx: usize = number(&mut args);
        let by: usize = number(&mut args);
        let raw: u32 = number(&mut args);
        acs.set(bx, by, AcStrategyType(raw)).unwrap();
    }
    let mut rqf = ImageI::create(xb, yb).unwrap();
    for y in 0..yb {
        for x in 0..xb {
            rqf.row_mut(y)[x] = number(&mut args);
        }
    }
    let matrices = simd_matrices();
    let mut quantizer = Quantizer::new(&matrices);
    quantizer.compute_global_scale_and_quant(&matrices, quant_dc, quant_median, 0.0);
    let mut opsin = Image3F::create(xsize, ysize).unwrap();
    for c in 0..3 {
        for y in 0..ysize {
            for x in 0..xsize {
                opsin.plane_row_mut(c, y)[x] = float(&mut args);
            }
        }
    }
    let opsin_rect = Rect::new(x0, y0, 8 * xb, 8 * yb);
    let mut cmap = ColorCorrelationMap::create(8 * xb, 8 * yb, true).unwrap();
    for map in [&mut cmap.ytox_map, &mut cmap.ytob_map] {
        for y in 0..map.ysize() {
            for x in 0..map.xsize() {
                map.row_mut(y)[x] = ((x * 7 + y * 3) as i32 % 51 - 25) as i8;
            }
        }
    }
    let mut cfl = CfLHeuristics::new(MemoryManagerPtr::NATIVE);
    let mut status = cfl.init(opsin_rect);
    let mut out = format!("T {}", status_code(&status));
    if status.is_err() {
        return out;
    }
    for y in 0..4 {
        let xs = cfl.dc_values.xsize();
        for (x, v) in cfl.dc_values.row_mut(y)[..xs].iter_mut().enumerate() {
            *v = sentinel(1 + y as u32, x);
        }
    }
    let (tiles_x, tiles_y) = (xb.div_ceil(8), yb.div_ceil(8));
    status = cfl.prepare_for_threads(1);
    write!(out, " {}", status_code(&status)).unwrap();
    if status.is_err() {
        return out;
    }
    for t in 0..tiles_x * tiles_y {
        let (tx, ty) = (t % tiles_x, t / tiles_x);
        let (bx0, by0) = (tx * 8, ty * 8);
        let (bx1, by1) = ((bx0 + 8).min(xb), (by0 + 8).min(yb));
        let r = Rect::new(bx0, by0, bx1 - bx0, by1 - by0);
        status = if pass == 0 {
            cfl.compute_tile(
                r, &opsin, opsin_rect, &matrices, None, None, None, false, 0, &mut cmap,
            )
        } else {
            cfl.compute_tile(
                r,
                &opsin,
                opsin_rect,
                &matrices,
                Some(&acs),
                Some(&rqf),
                Some(&quantizer),
                fast != 0,
                0,
                &mut cmap,
            )
        };
        if status.is_err() {
            break;
        }
    }
    write!(out, " {}", status_code(&status)).unwrap();
    for map in [&cmap.ytox_map, &cmap.ytob_map] {
        out.push_str(" C");
        for y in 0..map.ysize() {
            for &v in &map.row(y)[..map.xsize()] {
                write!(out, " {v}").unwrap();
            }
        }
    }
    for y in 0..4 {
        out.push_str(" D");
        for &v in &cfl.dc_values.row(y)[..cfl.dc_values.xsize()] {
            word(&mut out, v);
        }
    }
    out
}

/// A strategy layout: the larger transforms at aligned positions where they
/// fit, DCT8 elsewhere (as AcStrategyHeuristics places them).
fn placements(xblocks: usize, yblocks: usize, seed: u64) -> Vec<(usize, usize, u32)> {
    let mut seed = seed | 1;
    let mut taken = vec![false; xblocks * yblocks];
    let mut out = Vec::new();
    for by in 0..yblocks {
        for bx in 0..xblocks {
            if taken[by * xblocks + bx] {
                continue;
            }
            let raw = (next(&mut seed) % u64::from(NUM_VALID_STRATEGIES)) as u32;
            let acs = AcStrategy::from_raw_strategy(AcStrategyType(raw));
            let (cx, cy) = (acs.covered_blocks_x(), acs.covered_blocks_y());
            let fits = bx % cx == 0
                && by % cy == 0
                && bx + cx <= xblocks
                && by + cy <= yblocks
                && (0..cy).all(|y| (0..cx).all(|x| !taken[(by + y) * xblocks + bx + x]));
            let (raw, cx, cy) = if fits { (raw, cx, cy) } else { (0, 1, 1) };
            for y in 0..cy {
                for x in 0..cx {
                    taken[(by + y) * xblocks + bx + x] = true;
                }
            }
            out.push((bx, by, raw));
        }
    }
    out
}

/// Opsin-like planes (X small, Y in [0, 0.85), B near Y, with texture), or
/// with the special words at every fifth sample.
fn opsin_words(n: usize, c: usize, special: bool, seed: u64) -> Vec<f32> {
    let specials = special_words();
    let mut seed = seed.wrapping_mul(0x2545_f491_4f6c_dd1d) | 1;
    let (lo, hi) = [(-0.05f32, 0.05f32), (0.0, 0.85), (0.0, 0.8)][c];
    (0..n)
        .map(|i| {
            if special && i % 5 == 2 {
                f32::from_bits(specials[(i / 5 + c) % specials.len()])
            } else {
                uniform(&mut seed, lo, hi)
            }
        })
        .collect()
}

fn tile_requests() -> Vec<String> {
    let mut requests = Vec::new();
    // Block sizes: one block, partial and whole tiles, several tiles, and a
    // frame wider than a 32-block transform.
    let sizes: [(usize, usize); 9] = [
        (1, 1),
        (3, 2),
        (8, 8),
        (9, 5),
        (16, 8),
        (12, 17),
        (24, 24),
        (33, 10),
        (40, 34),
    ];
    for (si, &(xb, yb)) in sizes.iter().enumerate() {
        for variant in 0..7u64 {
            let seed = si as u64 * 100 + variant;
            // Pass 0 (DCT8); pass 1 with the Newton (Squirrel/Hare) and the
            // fast (Wombat) search; an offset opsin_rect; special words.
            let (pass, fast) =
                [(0, 0), (1, 0), (1, 1), (0, 0), (1, 0), (1, 1), (1, 0)][variant as usize];
            let (x0, y0) = if variant >= 3 {
                (5 + si, 8 + si % 3)
            } else {
                (0, 0)
            };
            let special = variant == 6;
            let (quant_dc, quant_median) =
                [(0.5f32, 0.39f32), (1.6, 0.39), (0.3, 0.2)][variant as usize % 3];
            let mut s = format!(
                "ecf-tile {xb} {yb} {x0} {y0} {pass} {fast} {} {}",
                quant_dc.to_bits(),
                quant_median.to_bits()
            );
            let layout = placements(xb, yb, seed * 7 + 3);
            write!(s, " {}", layout.len()).unwrap();
            for (bx, by, raw) in layout {
                write!(s, " {bx} {by} {raw}").unwrap();
            }
            let mut qseed = seed | 1;
            for _ in 0..xb * yb {
                write!(s, " {}", 1 + next(&mut qseed) % 64).unwrap();
            }
            let n = (x0 + 8 * xb + 3) * (y0 + 8 * yb);
            for c in 0..3 {
                push_words(&mut s, &opsin_words(n, c, special, seed * 3 + c as u64));
            }
            requests.push(s);
        }
    }
    requests
}

#[test]
fn enc_cfl_simd_compute_tile_through_the_encoder_dispatch_matches_the_original_target() {
    check_through_dispatch("ecf-tile", &tile_requests(), observe_tile);
}

// Symmetric5 ------------------------------------------------------------------

use jxl::jxl::convolve::WeightsSymmetric5;
use jxl::jxl::convolve_symmetric5::symmetric5;

/// Weight sets: Gaborish-like and mask-like normalized kernels, random
/// ones, and one whose four copies per weight differ (the interior reads
/// all four with LoadDup128, the border the first).
fn sym5_weights(set: usize) -> [f32; 24] {
    let mut seed = (set as u64 + 1) * 0x51_7cc1;
    let mut w = [0.0f32; 24];
    let base: [f32; 6] = match set {
        0 => [
            1.0,
            0.115_169_525,
            0.061_248_592,
            0.025_877_43,
            0.006_208_9,
            0.002_184_5,
        ],
        1 => [0.25, 0.125, 0.0625, 0.125, 0.0625, 0.03125],
        _ => std::array::from_fn(|_| uniform(&mut seed, -0.5, 1.0)),
    };
    for (k, &v) in base.iter().enumerate() {
        for i in 0..4 {
            w[k * 4 + i] = if set == 3 {
                v * (1.0 + 0.01 * i as f32)
            } else {
                v
            };
        }
    }
    w
}

fn observe_sym5(request: &str) -> String {
    let mut args = request.split_whitespace();
    assert_eq!(args.next(), Some("ecf-sym5"));
    let n: Vec<usize> = (0..10).map(|_| number(&mut args)).collect();
    let [ixs, iys, rx, ry, rxs, rys, oxs, oys, ox, oy] = n[..] else {
        unreachable!()
    };
    let mut weights = WeightsSymmetric5::default();
    for w in [
        &mut weights.c,
        &mut weights.r,
        &mut weights.R,
        &mut weights.d,
        &mut weights.D,
        &mut weights.L,
    ] {
        for v in w.iter_mut() {
            *v = float(&mut args);
        }
    }
    let mut input = ImageF::create(ixs, iys).unwrap();
    for y in 0..iys {
        for x in 0..ixs {
            input.row_mut(y)[x] = float(&mut args);
        }
    }
    let mut out = ImageF::create(oxs, oys).unwrap();
    for y in 0..oys {
        for x in 0..oxs {
            out.row_mut(y)[x] = sentinel(8, y * 4096 + x);
        }
    }
    let status = symmetric5(
        &input,
        Rect::new(rx, ry, rxs, rys),
        &weights,
        None,
        &mut out,
        Rect::new(ox, oy, rxs, rys),
    );
    let mut s = format!("S {}", status_code(&status));
    for y in 0..oys {
        s.push_str(" R");
        for &v in &out.row(y)[..oxs] {
            word(&mut s, v);
        }
    }
    s
}

fn sym5_request(
    plane: (usize, usize),
    rect: (usize, usize, usize, usize),
    out: (usize, usize, usize, usize),
    set: usize,
    kind: usize,
    seed: u64,
) -> String {
    let specials = special_words();
    let mut r = format!(
        "ecf-sym5 {} {} {} {} {} {} {} {} {} {}",
        plane.0, plane.1, rect.0, rect.1, rect.2, rect.3, out.0, out.1, out.2, out.3
    );
    for w in sym5_weights(set) {
        write!(r, " {}", w.to_bits()).unwrap();
    }
    let mut seed = seed.wrapping_mul(0x9e37_79b9_7f4a_7c15) | 1;
    for i in 0..plane.0 * plane.1 {
        let v = match kind {
            0 => uniform(&mut seed, -0.1, 1.1),
            1 => ((i * 7) % 13) as f32 / 12.0,
            2 => uniform(&mut seed, -1.0, 1.0) * 10f32.powi((next(&mut seed) % 13) as i32 - 6),
            _ => {
                if i % 5 == 2 {
                    f32::from_bits(specials[(i / 5 + kind) % specials.len()])
                } else {
                    uniform(&mut seed, 0.0, 1.0)
                }
            }
        };
        write!(r, " {}", v.to_bits()).unwrap();
    }
    r
}

#[test]
fn enc_cfl_simd_symmetric5_through_the_encoder_dispatch_matches_the_original_target() {
    let mut requests = Vec::new();
    // Whole planes: every width up to 20 and around the vector multiples,
    // every height from one row (all rows mirrored) to nine.
    let widths: Vec<usize> = (1..=20)
        .chain([23, 24, 25, 31, 32, 33, 63, 64, 65, 129])
        .collect();
    for (wi, &w) in widths.iter().enumerate() {
        for (hi, h) in [1usize, 2, 3, 4, 5, 6, 9].into_iter().enumerate() {
            let set = (wi + hi) % 5;
            let kind = (wi * 3 + hi) % 5;
            requests.push(sym5_request(
                (w, h),
                (0, 0, w, h),
                (w, h, 0, 0),
                set,
                kind,
                (wi * 16 + hi) as u64,
            ));
        }
    }
    // Rects inside larger planes (the encoder's padded planes and its
    // streaming rects), into offset rects of other planes.
    let mut seed = 0x5e45_0001u64;
    for rx in [0usize, 1, 2, 3, 5, 8, 11] {
        for ry in [0usize, 1, 2, 4] {
            for rxs in [1usize, 4, 7, 9, 16, 21, 30] {
                let rys = 1 + (next(&mut seed) % 7) as usize;
                let plane = (rx + rxs + (next(&mut seed) % 4) as usize, ry + rys + 3);
                let (ox, oy) = (
                    (next(&mut seed) % 4) as usize,
                    (next(&mut seed) % 3) as usize,
                );
                let out = (ox + rxs + 2, oy + rys + 1, ox, oy);
                let set = (next(&mut seed) % 5) as usize;
                let kind = (next(&mut seed) % 5) as usize;
                requests.push(sym5_request(
                    plane,
                    (rx, ry, rxs, rys),
                    out,
                    set,
                    kind,
                    next(&mut seed),
                ));
            }
        }
    }
    check_through_dispatch("ecf-sym5", &requests, observe_sym5);
}

// The 8-lane structure, natively on AArch64 -------------------------------

/// The 8-lane structure of the chroma-from-luma kernels (port ADR 0004):
/// `Neon8` is an 8-lane descriptor made of two NEON vectors, as in
/// tests/simd_vardct_lanes8.rs, whose SumOfLanes adds in the operand order
/// of Highway's AVX2 reduction. Lane-wise results must equal the 4-lane
/// ones; the reductions are checked against a model of the original's
/// per-lane accumulation with 4 lanes and NEON's `vaddv` order, which must
/// give the NEON kernel's words, and with 8 lanes and the AVX2 order, which
/// must give `Neon8`'s. This is not AVX2 parity (that is the AVX2 oracle
/// run); it exercises the lane-count-dependent paths: `RoundUpTo(num_blocks,
/// Lanes)` and the zeroed last lanes of InitDCStorage, the `Lanes`-strided
/// accumulators and the scratch sized by `MaxVectorSize()`.
#[cfg(target_arch = "aarch64")]
mod lanes8 {
    use core::arch::aarch64::*;
    use jxl::jxl::enc_chroma_from_luma_simd::{find_best_multiplier_d, init_dc_storage_d};
    use jxl::jxl::hwy::{Neon, SimdF32};
    use jxl::jxl::image::ImageF;
    use jxl::jxl::memory_manager_internal::MemoryManagerPtr;

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
        // SumOfLanes in the operand order of Highway's AVX2 reduction (the
        // port's `avx2::reduce`): s = v + SwapAdjacentBlocks(v) gives s_i =
        // v_i + v_(i+4) in the lower block, then (s0 + s3) + (s1 + s2), the same
        // value in every lane. The other lane-crossing operations are not used
        // by the chroma-from-luma kernels.
        fn sum_of_lanes(self, v: Self::V) -> Self::V {
            let mut s = [0.0f32; 4];
            self.0.store(self.0.add(v[0], v[1]), &mut s);
            let sum = (s[0] + s[3]) + (s[1] + s[2]);
            [self.0.set(sum), self.0.set(sum)]
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
        // AVX2's LoadDup128 (`_mm256_broadcast_ps`): the 128-bit block in
        // both halves.
        fn load_dup128(self, src: &[f32]) -> Self::V {
            [self.0.load(src), self.0.load(src)]
        }
        fn broadcast<const LANE: i32>(self, _: Self::V) -> Self::V {
            unimplemented!("lane-crossing")
        }
        fn store_demote_to_f16(self, _: Self::V, _: &mut [u16]) {
            unimplemented!("not used by the chroma-from-luma kernels")
        }
        // Signed 32-bit operations, each half through the NEON implementation,
        // as tests/simd_vardct_lanes8.rs has them.
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
    fn n4() -> Neon {
        Neon::new()
    }

    #[test]
    fn eight_lane_init_dc_storage_rounds_and_zeroes_eight_lanes() {
        for num_blocks in 0..=70usize {
            let mut four = ImageF::default();
            let mut eight = ImageF::default();
            let s4 = init_dc_storage_d(n4(), MemoryManagerPtr::NATIVE, num_blocks, &mut four);
            let s8 = init_dc_storage_d(
                Neon8(n4()),
                MemoryManagerPtr::NATIVE,
                num_blocks,
                &mut eight,
            );
            assert_eq!(s4.is_ok(), num_blocks != 0, "4 lanes, {num_blocks} blocks");
            assert_eq!(s8.is_ok(), num_blocks != 0, "8 lanes, {num_blocks} blocks");
            if num_blocks == 0 {
                continue;
            }
            assert_eq!(four.xsize(), num_blocks.div_ceil(4) * 4);
            assert_eq!(eight.xsize(), num_blocks.div_ceil(8) * 8);
            assert_eq!(eight.ysize(), 4);
            for y in 0..4 {
                let row = &eight.row(y)[..eight.xsize()];
                assert!(
                    row[row.len() - 8..].iter().all(|v| v.to_bits() == 0),
                    "8 lanes, {num_blocks} blocks, row {y}: last lanes not zero"
                );
            }
        }
    }

    /// SumOfLanes of `lanes` accumulators in the target's order: NEON's
    /// `vaddv` for four, Highway's AVX2 reduction for eight.
    fn tree(l: &[f32]) -> f32 {
        if l.len() == 4 {
            (l[0] + l[1]) + (l[2] + l[3])
        } else {
            let s: [f32; 4] = std::array::from_fn(|i| l[i] + l[i + 4]);
            (s[0] + s[3]) + (s[1] + s[2])
        }
    }

    /// A model of FindBestMultiplier with `lanes` independent per-lane
    /// accumulators (lane `l` takes the samples `l, l + lanes, ...`) and
    /// fused MulAdd, as the original's HWY_FULL(float) loops run.
    fn model(lanes: usize, m: &[f32], s: &[f32], base: f32, dm: f32, fast: bool) -> i32 {
        let num = m.len();
        if num == 0 {
            return 0;
        }
        let icf = 1.0f32 / 84.0;
        let mut x: f32;
        if fast {
            let mut ca = vec![0.0f32; lanes];
            let mut cb = vec![0.0f32; lanes];
            for i in 0..num {
                let a = icf * m[i];
                let b = base * m[i] - s[i];
                ca[i % lanes] = a.mul_add(a, ca[i % lanes]);
                cb[i % lanes] = a.mul_add(b, cb[i % lanes]);
            }
            x = -tree(&cb) / (num as f32 * dm).mul_add(0.5, tree(&ca));
        } else {
            let compute = |x: f32, eps: f32| -> [f32; 3] {
                let nf = num as f32;
                let mut acc = [
                    vec![0.0f32; lanes],
                    vec![0.0f32; lanes],
                    vec![0.0f32; lanes],
                ];
                for i in 0..num {
                    let a = icf * m[i];
                    let b = base * m[i] - s[i];
                    let coeffx2 = (1.0f32 / 3.0) * 2.0;
                    let av = a.mul_add(x, b);
                    let above = av.abs() >= 100.0;
                    for (k, xk) in [x, x + eps, x - eps].into_iter().enumerate() {
                        let v = a.mul_add(xk, b);
                        let mut d = (coeffx2 * a) * (v.abs() + 1.0);
                        if v < 0.0 {
                            d = 0.0 - d;
                        }
                        acc[k][i % lanes] += if above { 0.0 } else { d };
                    }
                }
                [
                    2.0 * dm * nf * x + tree(&acc[0]),
                    2.0 * dm * nf * (x + eps) + tree(&acc[1]),
                    2.0 * dm * nf * (x - eps) + tree(&acc[2]),
                ]
            };
            x = 0.0;
            for _ in 0..20 {
                let [d_f, dfpeps, dfmeps] = compute(x, 100.0);
                let ddf = (dfpeps - dfmeps) / 200.0;
                let step = d_f / (ddf + 0.85f64 as f32);
                x -= 20.0f32.min((-20.0f32).max(step));
                if f64::from(step.abs()) < 3e-3 {
                    break;
                }
            }
        }
        let towards_zero = 2.6f64 as f32;
        if x >= towards_zero {
            x -= towards_zero;
        } else if x <= -towards_zero {
            x += towards_zero;
        } else {
            x = 0.0;
        }
        // Clamp1 passes a NaN through, as f32::clamp does.
        x.round().clamp(-128.0, 127.0) as i32
    }

    fn next(seed: &mut u64) -> u64 {
        *seed ^= *seed << 13;
        *seed ^= *seed >> 7;
        *seed ^= *seed << 17;
        *seed
    }
    fn uniform(seed: &mut u64, lo: f32, hi: f32) -> f32 {
        lo + (next(seed) % (1 << 24)) as f32 / (1u32 << 24) as f32 * (hi - lo)
    }

    #[test]
    fn eight_lane_find_best_multiplier_follows_the_eight_lane_accumulation() {
        let mut seed = 0x0cf1_8888;
        let mut cases = 0;
        let mut differ = 0;
        for num in [8usize, 16, 24, 64, 200, 1024, 4096] {
            for kind in 0..4 {
                for base in [0.0f32, 1.0] {
                    for dm in [1e-9f32, 0.01, 1.0] {
                        for fast in [false, true] {
                            let k = uniform(&mut seed, -60.0, 60.0);
                            let (m, s): (Vec<f32>, Vec<f32>) = (0..num)
                                .map(|_| {
                                    if kind == 3 {
                                        // Magnitudes over six decades: the
                                        // sums depend on their order.
                                        let a = uniform(&mut seed, -1.0, 1.0)
                                            * 10f32.powi((next(&mut seed) % 7) as i32 - 2);
                                        return (a, a * k / 84.0 + uniform(&mut seed, -1e-3, 1e-3));
                                    }
                                    let a = uniform(&mut seed, -30.0, 30.0)
                                        * [1.0f32, 0.01, 10.0][kind];
                                    (a, a * k / 84.0 + uniform(&mut seed, -1.0, 1.0))
                                })
                                .unzip();
                            let four = find_best_multiplier_d(n4(), &m, &s, num, base, dm, fast);
                            let eight =
                                find_best_multiplier_d(Neon8(n4()), &m, &s, num, base, dm, fast);
                            assert_eq!(
                                four,
                                model(4, &m, &s, base, dm, fast),
                                "4 lanes: num {num} kind {kind} base {base} dm {dm} fast {fast}"
                            );
                            assert_eq!(
                                eight,
                                model(8, &m, &s, base, dm, fast),
                                "8 lanes: num {num} kind {kind} base {base} dm {dm} fast {fast}"
                            );
                            differ += usize::from(four != eight);
                            cases += 1;
                        }
                    }
                }
            }
        }
        println!(
            "8-lane FindBestMultiplier: {cases} cases follow the model, {differ} differ from 4 lanes"
        );
    }

    #[test]
    fn eight_lane_compute_tile_equals_four_lane_compute_tile_lane_wise() {
        use jxl::jxl::ac_strategy::{
            AcStrategy, AcStrategyImage, AcStrategyType, MAX_COEFF_AREA, MAX_COEFF_BLOCKS,
        };
        use jxl::jxl::base::rect::Rect;
        use jxl::jxl::chroma_from_luma::COLOR_TILE_DIM;
        use jxl::jxl::coeff_order_fwd::coefficient_layout;
        use jxl::jxl::enc_chroma_from_luma_simd::{
            TileArgs, compute_tile_d, items_per_thread_for_lanes,
        };
        use jxl::jxl::image::{Image3F, ImageI, ImageSB};
        use jxl::jxl::quant_weights::DequantMatrices;
        use jxl::jxl::quantizer::Quantizer;

        let mut matrices = DequantMatrices::default();
        matrices
            .ensure_computed_for_target(
                MemoryManagerPtr::NATIVE,
                !0,
                Some(jxl::jxl::hwy::Target::detect().unwrap()),
            )
            .unwrap();
        let mut quantizer = Quantizer::new(&matrices);
        quantizer.compute_global_scale_and_quant(&matrices, 0.5, 0.39, 0.0);
        const TILE: usize = COLOR_TILE_DIM * COLOR_TILE_DIM;
        let coeffs_at = 3 * MAX_COEFF_AREA;
        let fixed = coeffs_at + 4 * TILE + 3 * MAX_COEFF_BLOCKS * MAX_COEFF_BLOCKS;
        let mut seed = 0x7113_8888u64;
        let mut cases = 0;
        for (xb, yb) in [(1usize, 1usize), (3, 2), (8, 8), (12, 9), (32, 32)] {
            for layout in [0u32, 1, 2, 5, 9, 13, 17, 26] {
                for (pass, fast) in [(0, false), (1, false), (1, true)] {
                    let mut acs = AcStrategyImage::create(xb, yb).unwrap();
                    acs.fill_dct8_full();
                    let kind = AcStrategy::from_raw_strategy(AcStrategyType(layout));
                    let (cx, cy) = (kind.covered_blocks_x(), kind.covered_blocks_y());
                    for by in (0..yb).step_by(cy) {
                        for bx in (0..xb).step_by(cx) {
                            if bx + cx <= xb && by + cy <= yb {
                                acs.set(bx, by, AcStrategyType(layout)).unwrap();
                            }
                        }
                    }
                    let mut rqf = ImageI::create(xb, yb).unwrap();
                    for y in 0..yb {
                        for x in 0..xb {
                            rqf.row_mut(y)[x] = 1 + (next(&mut seed) % 64) as i32;
                        }
                    }
                    let mut opsin = Image3F::create(8 * xb, 8 * yb).unwrap();
                    for c in 0..3 {
                        for y in 0..8 * yb {
                            for x in 0..8 * xb {
                                opsin.plane_row_mut(c, y)[x] = uniform(&mut seed, 0.0, 0.8);
                            }
                        }
                    }
                    let opsin_rect = Rect::new(0, 0, 8 * xb, 8 * yb);
                    let rect = Rect::new(0, 0, xb.min(8), yb.min(8));
                    let run = |eight: bool| {
                        let lanes = if eight { 8 } else { 4 };
                        let mut map_x = ImageSB::create(xb.div_ceil(8), yb.div_ceil(8)).unwrap();
                        let mut map_b = ImageSB::create(xb.div_ceil(8), yb.div_ceil(8)).unwrap();
                        let mut dc = ImageF::default();
                        let s = if eight {
                            init_dc_storage_d(
                                Neon8(n4()),
                                MemoryManagerPtr::NATIVE,
                                xb * yb,
                                &mut dc,
                            )
                        } else {
                            init_dc_storage_d(n4(), MemoryManagerPtr::NATIVE, xb * yb, &mut dc)
                        };
                        s.unwrap();
                        let mut mem = vec![-7.25f32; items_per_thread_for_lanes(lanes)];
                        let args = TileArgs {
                            opsin: &opsin,
                            opsin_rect,
                            dequant: &matrices,
                            ac_strategy: (pass == 1).then_some(&acs),
                            raw_quant_field: (pass == 1).then_some(&rqf),
                            quantizer: (pass == 1).then_some(&quantizer),
                            rect,
                            fast,
                            use_dct8: pass == 0,
                        };
                        let status = if eight {
                            compute_tile_d(
                                Neon8(n4()),
                                args,
                                &mut map_x,
                                &mut map_b,
                                &mut dc,
                                &mut mem,
                            )
                        } else {
                            compute_tile_d(n4(), args, &mut map_x, &mut map_b, &mut dc, &mut mem)
                        };
                        status.unwrap();
                        let dcs: Vec<f32> =
                            (0..4).flat_map(|y| dc.row(y)[..xb * yb].to_vec()).collect();
                        (map_x.row(0)[0], map_b.row(0)[0], dcs, mem[..fixed].to_vec())
                    };
                    let (x4, b4, dc4, mem4) = run(false);
                    let (x8, b8, dc8, mem8) = run(true);
                    let what = format!("{xb}x{yb} strategy {layout} pass {pass} fast {fast}");
                    assert_same(&format!("{what} dc_values"), &dc4, &dc8);
                    assert_same(&format!("{what} blocks, coefficients, dc"), &mem4, &mem8);
                    // The coefficients the search ran over, as ComputeTile
                    // collects them.
                    let mut num_ac = 0;
                    for y in rect.y0()..rect.y0() + rect.ysize() {
                        for x in rect.x0()..rect.x0() + rect.xsize() {
                            let a = if pass == 0 {
                                AcStrategy::from_raw_strategy(AcStrategyType::DCT)
                            } else {
                                acs.const_row(y, 0).get(x)
                            };
                            if !a.is_first_block()
                                || a.covered_blocks_x() > rect.xsize()
                                || a.covered_blocks_y() > rect.ysize()
                            {
                                continue;
                            }
                            let (mut cx, mut cy) = (a.covered_blocks_x(), a.covered_blocks_y());
                            coefficient_layout(&mut cy, &mut cx);
                            num_ac += cx * cy * 64;
                        }
                    }
                    let region =
                        |k: usize| &mem4[coeffs_at + k * TILE..coeffs_at + k * TILE + num_ac];
                    for (lanes, mx, mb) in [(4, x4, b4), (8, x8, b8)] {
                        assert_eq!(
                            i32::from(mx),
                            model(lanes, region(0), region(1), 0.0, 1e-9, fast) as i8 as i32,
                            "{what}: {lanes}-lane ytox"
                        );
                        assert_eq!(
                            i32::from(mb),
                            model(lanes, region(2), region(3), 1.0, 1e-9, fast) as i8 as i32,
                            "{what}: {lanes}-lane ytob"
                        );
                    }
                    cases += 1;
                }
            }
        }
        println!("8-lane ComputeTile: {cases} cases match lane-wise and follow the model");
    }

    /// Bit-identical words, except that any NaN equals any NaN (D8).
    fn assert_same(what: &str, four: &[f32], eight: &[f32]) {
        assert_eq!(four.len(), eight.len(), "{what}: lengths");
        for (i, (a, b)) in four.iter().zip(eight).enumerate() {
            if a.to_bits() != b.to_bits() && !(a.is_nan() && b.is_nan()) {
                panic!(
                    "{what}: 4 and 8 lanes differ at word {i}: {:#010x} != {:#010x}",
                    a.to_bits(),
                    b.to_bits()
                );
            }
        }
    }

    #[test]
    fn eight_lane_symmetric5_rows_equal_four_lane_rows() {
        use jxl::jxl::base::rect::Rect;
        use jxl::jxl::convolve::WeightsSymmetric5;
        use jxl::jxl::convolve_symmetric5_simd::symmetric5_row_d;
        use jxl::jxl::image_ops::{WrapMirror, WrapUnchanged};
        // Identical copies per weight: with them the interior lanes, the
        // border pixels and the 4- and 8-lane split of each row compute the
        // same unfused sums, so any difference is a lane-count defect (the
        // aligned prefix `RoundUpTo(2, Lanes)`, the interior bound, the tail).
        let mut seed = 0x5e45_8888u64;
        let mut cases = 0;
        for xsize in 1..=45usize {
            for ysize in [1usize, 3, 6] {
                let mut input = ImageF::create(xsize, ysize).unwrap();
                for y in 0..ysize {
                    for x in 0..xsize {
                        input.row_mut(y)[x] = uniform(&mut seed, -1.0, 1.0);
                    }
                }
                let v: [f32; 6] = std::array::from_fn(|_| uniform(&mut seed, -0.5, 1.0));
                let weights = WeightsSymmetric5 {
                    c: [v[0]; 4],
                    r: [v[1]; 4],
                    R: [v[2]; 4],
                    d: [v[3]; 4],
                    D: [v[4]; 4],
                    L: [v[5]; 4],
                };
                for x0 in [0usize, 1, 3] {
                    if x0 >= xsize {
                        continue;
                    }
                    let rect = Rect::new(x0, 0, xsize - x0, ysize);
                    for iy in 0..ysize as i64 {
                        let mirror = iy < 2 || iy >= ysize as i64 - 2;
                        let run = |eight: bool| {
                            let mut out = vec![-3.5f32; xsize + 8];
                            match (eight, mirror) {
                                (false, true) => symmetric5_row_d::<_, WrapMirror>(
                                    n4(),
                                    &input,
                                    rect,
                                    iy,
                                    &weights,
                                    &mut out,
                                ),
                                (false, false) => symmetric5_row_d::<_, WrapUnchanged>(
                                    n4(),
                                    &input,
                                    rect,
                                    iy,
                                    &weights,
                                    &mut out,
                                ),
                                (true, true) => symmetric5_row_d::<_, WrapMirror>(
                                    Neon8(n4()),
                                    &input,
                                    rect,
                                    iy,
                                    &weights,
                                    &mut out,
                                ),
                                (true, false) => symmetric5_row_d::<_, WrapUnchanged>(
                                    Neon8(n4()),
                                    &input,
                                    rect,
                                    iy,
                                    &weights,
                                    &mut out,
                                ),
                            }
                            out
                        };
                        assert_same(
                            &format!("symmetric5 row {xsize}x{ysize} x0 {x0} y {iy}"),
                            &run(false),
                            &run(true),
                        );
                        cases += 1;
                    }
                }
            }
        }
        println!("8-lane Symmetric5: {cases} rows match");
    }
}
