// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! SIMD parity of the encoder's smaller Highway exports (port ADR 0004,
//! "Encoder path", package `encoder entropy`): the port's NEON/AVX2
//! instantiations against the original built for the same single Highway
//! target (`JXL_CPP_ORACLE_SIMD`), through the `ene-*` requests of
//! tools/oracle/encoder_entropy_simd.cc. Integer words and decisions must be
//! identical without exception; float words are compared bit for bit except
//! that any NaN equals any NaN (decision D8). Every family goes through the
//! encoder's own dispatch point (`hwy::encoder_target`), forced to the SIMD
//! target, and shows that the scalar path gives different words for some of
//! its requests, so the comparison tells the two paths apart.
#![cfg(feature = "cpp-oracle-simd")]

mod simd_common;

use jxl::jxl::enc_ans_params::{ClusteringType, Histogram, HistogramParams};
use jxl::jxl::enc_cluster::cluster_histograms;
use jxl::jxl::enc_cluster_simd::{
    fast_cluster_histograms_simd, histogram_distance_simd, histogram_kl_divergence_simd,
};
use jxl::jxl::enc_frame::{LosslessSampleDepth, LosslessThunderConfig};
use jxl::jxl::encode_internal::JxlEncoderChunkedFrameAdapter;
use jxl::jxl::hwy;
use jxl::jxl::modular::modular_image::Image as ModularImage;
use jxl::jxl::modular::transform::enc_rct::{fwd_rct, fwd_rct_channels};
use jxl::jxl::types::{JxlDataType, JxlEndianness, JxlPixelFormat};
use simd_common::{check_rows, next, status};
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
fn round_up8(x: usize) -> usize {
    x.div_ceil(8) * 8
}
/// The sentinels of tools/oracle/encoder_entropy_simd.cc.
fn sentinel(salt: u32, i: usize) -> i32 {
    (0x5a00_0000u32 + salt * 0x10_0000 + (i as u32 & 0xf_ffff)) as i32
}
fn all_defined(_: usize) -> bool {
    true
}

/// Runs `requests` through `observe` with the SIMD encoder selection and
/// compares the observations with the original's target; `scalar_safe`
/// requests also run with the scalar selection, and some of them must
/// differ. `float` says which words are binary32 bits.
fn check_through_dispatch(
    family: &str,
    requests: &[String],
    scalar_safe: impl Fn(usize) -> bool,
    observe: fn(&str) -> String,
    float: fn(usize) -> bool,
) {
    check_through_dispatch_with(family, requests, scalar_safe, observe, float, true);
}

/// [`check_through_dispatch`]; `must_differ` false for an export whose
/// vector branch gives the scalar branch's results exactly (counting,
/// truncated costs), where the results cannot tell the paths apart.
fn check_through_dispatch_with(
    family: &str,
    requests: &[String],
    scalar_safe: impl Fn(usize) -> bool,
    observe: fn(&str) -> String,
    float: fn(usize) -> bool,
    must_differ: bool,
) {
    let _lock = dispatch_lock();
    hwy::set_encoder_simd_for_test(Some(false));
    let scalar: Vec<Option<String>> = requests
        .iter()
        .enumerate()
        .map(|(i, r)| scalar_safe(i).then(|| observe(r)))
        .collect();
    hwy::set_encoder_simd_for_test(Some(true));
    let simd: Vec<String> = requests.iter().map(|r| observe(r)).collect();
    hwy::set_encoder_simd_for_test(None);
    let compared = scalar.iter().filter(|s| s.is_some()).count();
    let differing = scalar
        .iter()
        .zip(&simd)
        .filter(|(a, b)| a.as_ref().is_some_and(|a| a != *b))
        .count();
    assert!(
        differing > 0 || !must_differ,
        "{family}: the scalar and SIMD paths gave the same words everywhere"
    );
    println!("{family}: the scalar path differs in {differing} of {compared} requests it can run");
    let index = std::cell::Cell::new(0);
    check_rows(
        family,
        requests,
        |_| {
            let observed = simd[index.get()].clone();
            index.set(index.get() + 1);
            (observed, Box::new(all_defined))
        },
        float,
    );
}

fn no_float(_: usize) -> bool {
    false
}

// enc_rct --------------------------------------------------------------------

/// The port's observation of an `ene-rct` request through `fwd_rct` /
/// `fwd_rct_channels` (the caller sets the encoder's selection).
fn observe_rct(request: &str) -> String {
    let mut args = request.split_whitespace();
    assert_eq!(args.next(), Some("ene-rct"));
    let mode: usize = number(&mut args);
    let rct_type: usize = number(&mut args);
    let w: usize = number(&mut args);
    let h: usize = number(&mut args);
    let padded = round_up8(w);
    let mut from = ModularImage::create(w, h, 16, 3).unwrap();
    for c in 0..3 {
        for y in 0..h {
            let row = from.channel[c].row_mut(y);
            assert!(row.len() >= padded, "row padding below 8 lanes");
            for value in &mut row[..padded] {
                *value = number::<i64>(&mut args) as i32;
            }
        }
    }
    let print = |out: &mut String, image: &ModularImage| {
        for channel in &image.channel {
            out.push_str(" C");
            for y in 0..channel.h {
                for &v in &channel.row(y)[..padded] {
                    write!(out, " {v}").unwrap();
                }
            }
        }
    };
    let mut out;
    if mode == 0 {
        let s = fwd_rct(&mut from, 0, rct_type);
        out = format!("S {}", status(&s));
        print(&mut out, &from);
    } else {
        let mut to = ModularImage::create(w, h, 16, 3).unwrap();
        for c in 0..3 {
            for y in 0..h {
                for x in 0..padded {
                    to.channel[c].row_mut(y)[x] = sentinel(c as u32, y * padded + x);
                }
            }
        }
        let [a, b, c] = &mut to.channel[..] else {
            unreachable!()
        };
        let s = fwd_rct_channels(
            [&from.channel[0], &from.channel[1], &from.channel[2]],
            [a, b, c],
            rct_type,
        );
        out = format!("S {}", status(&s));
        print(&mut out, &to);
    }
    out
}

/// Sample words of kind 0 (pixel-like, 16-bit signed: no scalar overflow),
/// 1 (any 32-bit word: the vector operations wrap) or 2 (the integer
/// extremes and their neighbors).
fn rct_samples(count: usize, kind: usize, seed: &mut u64) -> Vec<i32> {
    const EXTREMES: [i32; 10] = [
        i32::MIN,
        i32::MIN + 1,
        i32::MAX,
        i32::MAX - 1,
        -1,
        0,
        1,
        i32::MIN / 2,
        i32::MAX / 2 + 1,
        -0x4000_0001,
    ];
    (0..count)
        .map(|_| {
            let r = next(seed);
            match kind {
                0 => (r % 65536) as i32 - 32768,
                1 => r as u32 as i32,
                _ => EXTREMES[(r % EXTREMES.len() as u64) as usize],
            }
        })
        .collect()
}

#[test]
fn enc_simd_fwd_rct_through_the_encoder_dispatch_matches_the_original_target() {
    let mut requests = Vec::new();
    let mut safe = Vec::new();
    let mut seed = 0x5eed_0e7c_0000_0001u64;
    for mode in 0..2usize {
        for rct_type in 0..42usize {
            for (w, h) in [
                (1, 1),
                (3, 2),
                (4, 1),
                (5, 3),
                (8, 2),
                (9, 1),
                (16, 2),
                (31, 1),
                (33, 2),
                (67, 1),
            ] {
                for kind in 0..3usize {
                    // Every type and width with pixel-like words; the
                    // wrapping kinds over a subset of the widths.
                    if kind > 0 && !matches!(w, 1 | 5 | 9 | 33) {
                        continue;
                    }
                    let mut s = format!("ene-rct {mode} {rct_type} {w} {h}");
                    for v in rct_samples(3 * h * round_up8(w), kind, &mut seed) {
                        write!(s, " {v}").unwrap();
                    }
                    requests.push(s);
                    safe.push(kind == 0);
                }
            }
        }
    }
    // A zero-height image runs no row.
    for mode in 0..2usize {
        requests.push(format!("ene-rct {mode} 6 5 0"));
        safe.push(true);
    }
    check_through_dispatch("ene-rct", &requests, |i| safe[i], observe_rct, no_float);
}

// enc_cluster -----------------------------------------------------------------

/// `<total_count> <entropy bits> <n> <n counts>`, as EneHistogram reads it.
fn read_histogram(args: &mut Args<'_>) -> Histogram {
    let total: usize = number(args);
    let entropy = f32::from_bits(number(args));
    let n: usize = number(args);
    let counts = (0..n).map(|_| number::<i64>(args) as i32).collect();
    Histogram {
        counts,
        total_count: total,
        entropy,
    }
}
fn print_histogram(out: &mut String, h: &Histogram) {
    write!(out, " h {}", h.counts.len()).unwrap();
    for &c in &h.counts {
        write!(out, " {c}").unwrap();
    }
    write!(out, " {} {}", h.total_count, h.entropy.to_bits()).unwrap();
}
fn histogram_text(h: &Histogram) -> String {
    let mut s = format!(
        " {} {} {}",
        h.total_count,
        h.entropy.to_bits(),
        h.counts.len()
    );
    for &c in &h.counts {
        write!(s, " {c}").unwrap();
    }
    s
}

/// The cluster observations mix integers (counts, totals, symbols) and float
/// bits (entropies, distances). Every word may take the NaN rule: integer
/// words are either negative, which no NaN rule matches, or counts below
/// 0x7f80_0000 in these requests, which are no NaN patterns, so the rule
/// only ever applies to float words.
fn cluster_float(_: usize) -> bool {
    true
}

/// The port's observation of an `ene-hcond`, `ene-hent`, `ene-hdist`,
/// `ene-fast` or `ene-cluster` request (the caller sets the encoder's
/// selection; `ene-hdist` and `ene-fast` call the SIMD functions directly).
fn observe_cluster(request: &str) -> String {
    let mut args = request.split_whitespace();
    let command = args.next().unwrap();
    let mut out = String::new();
    match command {
        "ene-hcond" => {
            let mut h = read_histogram(&mut args);
            h.condition();
            out.push('H');
            print_histogram(&mut out, &h);
        }
        "ene-hent" => {
            let mut h = read_histogram(&mut args);
            let entropy = h.shannon_entropy();
            write!(out, "E {}", entropy.to_bits()).unwrap();
            print_histogram(&mut out, &h);
        }
        "ene-hdist" => {
            let a = read_histogram(&mut args);
            let b = read_histogram(&mut args);
            let t = simd_common::target();
            write!(
                out,
                "D {} {}",
                histogram_distance_simd(t, &a, &b).to_bits(),
                histogram_kl_divergence_simd(t, &a, &b).to_bits()
            )
            .unwrap();
        }
        _ => {
            let fast = command == "ene-fast";
            let mut params = HistogramParams::default();
            let max_histograms: usize;
            let mut prefix_codes = false;
            if fast {
                max_histograms = number(&mut args);
                prefix_codes = number::<usize>(&mut args) != 0;
            } else {
                assert_eq!(command, "ene-cluster");
                params.clustering = match number::<usize>(&mut args) {
                    0 => ClusteringType::Fastest,
                    1 => ClusteringType::Fast,
                    _ => ClusteringType::Best,
                };
                params.force_huffman = number::<usize>(&mut args) != 0;
                params.max_histograms = number(&mut args);
                max_histograms = number(&mut args);
            }
            let n: usize = number(&mut args);
            let mut histograms: Vec<Histogram> =
                (0..n).map(|_| read_histogram(&mut args)).collect();
            let m: usize = number(&mut args);
            let mut clustered: Vec<Histogram> = (0..m).map(|_| read_histogram(&mut args)).collect();
            let mut symbols = vec![7u32, 7, 7];
            let s = if fast {
                fast_cluster_histograms_simd(
                    simd_common::target(),
                    &mut histograms,
                    max_histograms,
                    prefix_codes,
                    &mut clustered,
                    &mut symbols,
                )
            } else {
                cluster_histograms(
                    &params,
                    &mut histograms,
                    max_histograms,
                    &mut clustered,
                    &mut symbols,
                )
            };
            write!(out, "S {} Y {}", status(&s), symbols.len()).unwrap();
            for s in &symbols {
                write!(out, " {s}").unwrap();
            }
            out.push_str(" I");
            for h in &histograms {
                write!(out, " {}", h.entropy.to_bits()).unwrap();
            }
            write!(out, " O {}", clustered.len()).unwrap();
            for h in &clustered {
                print_histogram(&mut out, h);
            }
        }
    }
    out
}

/// A histogram of `n` (a multiple of 8) counts of shape `kind`: 0 dense
/// small counts, 1 sparse, 2 one symbol, 3 all zero, 4 a skewed photo-like
/// distribution, 5 large counts. `consistent` sets `total_count` to the sum.
fn histogram(n: usize, kind: usize, seed: &mut u64, consistent: bool) -> Histogram {
    let counts: Vec<i32> = (0..n)
        .map(|i| {
            let r = next(seed);
            match kind {
                0 => (r % 40) as i32,
                1 => {
                    if r.is_multiple_of(7) {
                        (r >> 8) as i32 & 0x3ff
                    } else {
                        0
                    }
                }
                2 => 0,
                3 => 0,
                4 => ((1u64 << (20 - (i.min(19)))) / (1 + r % 3)) as i32,
                _ => (r % 0x0100_0000) as i32,
            }
        })
        .collect();
    let mut counts = counts;
    if kind == 2 {
        counts[n / 3] = 1 + (next(seed) % 5000) as i32;
    }
    let sum: i64 = counts.iter().map(|&c| i64::from(c)).sum();
    let total_count = if consistent {
        sum as usize
    } else {
        (next(seed) % 100_000) as usize
    };
    Histogram {
        counts,
        total_count,
        entropy: (next(seed) % 5000) as f32 * 0.37,
    }
}

#[test]
fn enc_simd_histogram_condition_and_entropy_through_the_encoder_dispatch_match_the_original_target()
{
    let mut seed = 0xc0d1_7105_0000_0003u64;
    let mut requests = Vec::new();
    for n in [8usize, 16, 24, 40, 64, 136, 256] {
        for kind in 0..6 {
            for consistent in [true, false] {
                let h = histogram(n, kind, &mut seed, consistent);
                requests.push(format!("ene-hcond{}", histogram_text(&h)));
                requests.push(format!("ene-hent{}", histogram_text(&h)));
            }
        }
        // Trailing zeros at every vector boundary, and a last count in each
        // lane of the last vector.
        for last in [0, 1, 3, 4, 5, 7, n / 2, n - 1] {
            let mut counts = vec![0i32; n];
            counts[last] = 3;
            counts[0] += 1;
            let h = Histogram {
                counts,
                total_count: 4,
                entropy: 0.0,
            };
            requests.push(format!("ene-hcond{}", histogram_text(&h)));
            requests.push(format!("ene-hent{}", histogram_text(&h)));
        }
    }
    // Wrapping integer sums, negative counts and a total of 0.
    for counts in [
        vec![i32::MAX, 1, 0, 0, 0, 0, 0, 0],
        vec![i32::MIN, -1, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![-3, 4, 0, 0, 0, 0, 0, 9],
        vec![0; 8],
    ] {
        for total_count in [0usize, 1, 13] {
            let h = Histogram {
                counts: counts.clone(),
                total_count,
                entropy: 1.5,
            };
            requests.push(format!("ene-hcond{}", histogram_text(&h)));
            requests.push(format!("ene-hent{}", histogram_text(&h)));
        }
    }
    check_through_dispatch(
        "ene-hcond/hent",
        &requests,
        |_| true,
        observe_cluster,
        cluster_float,
    );
}

#[test]
fn enc_simd_histogram_distance_and_kl_divergence_match_the_original_target() {
    let mut seed = 0xd157_0000_0000_0005u64;
    let mut requests = Vec::new();
    for (na, nb) in [
        (8usize, 8usize),
        (8, 24),
        (40, 16),
        (64, 64),
        (136, 8),
        (256, 120),
    ] {
        for ka in 0..6 {
            for kb in [0, 1, 2, 3, 4, 5] {
                let a = histogram(na, ka, &mut seed, true);
                let b = histogram(nb, kb, &mut seed, (ka + kb) % 3 != 0);
                requests.push(format!(
                    "ene-hdist{}{}",
                    histogram_text(&a),
                    histogram_text(&b)
                ));
            }
        }
    }
    let _lock = dispatch_lock();
    check_rows(
        "ene-hdist",
        &requests,
        |r| (observe_cluster(r), Box::new(all_defined)),
        cluster_float,
    );
}

/// `count` histograms around a few prototypes, so that clustering merges
/// some and keeps others apart.
fn cluster_inputs(count: usize, n: usize, seed: &mut u64) -> Vec<Histogram> {
    let prototypes: Vec<Histogram> = (0..4)
        .map(|k| histogram(n, [0, 1, 4, 5][k], seed, true))
        .collect();
    (0..count)
        .map(|_| {
            let r = next(seed);
            if r.is_multiple_of(9) {
                return Histogram {
                    counts: vec![0; n],
                    total_count: 0,
                    entropy: 0.0,
                };
            }
            if r % 9 == 1 {
                return histogram(n, 2, seed, true);
            }
            let p = &prototypes[(r >> 8) as usize % prototypes.len()];
            let counts: Vec<i32> = p
                .counts
                .iter()
                .map(|&c| {
                    let jitter = next(seed) % 5;
                    (c as i64 * (8 + jitter as i64) / 10) as i32
                })
                .collect();
            let total_count = counts.iter().map(|&c| c as usize).sum();
            Histogram {
                counts,
                total_count,
                entropy: 0.0,
            }
        })
        .collect()
}

fn cluster_requests() -> (Vec<String>, Vec<String>) {
    let mut seed = 0xc105_7e50_0000_0007u64;
    let (mut fast, mut cluster) = (Vec::new(), Vec::new());
    for (count, n) in [
        (1usize, 8usize),
        (3, 16),
        (8, 40),
        (20, 64),
        (40, 24),
        (64, 136),
    ] {
        for prev in [0usize, 1, 3] {
            let input = cluster_inputs(count, n, &mut seed);
            let previous: Vec<Histogram> = (0..prev)
                .map(|k| histogram(n, [0, 4, 1][k % 3], &mut seed, true))
                .collect();
            let mut tail = format!(" {}", input.len());
            for h in &input {
                tail.push_str(&histogram_text(h));
            }
            write!(tail, " {}", previous.len()).unwrap();
            for h in &previous {
                tail.push_str(&histogram_text(h));
            }
            for max in [1usize, 2, 4, 16, 255] {
                for prefix in 0..2 {
                    fast.push(format!("ene-fast {} {prefix}{tail}", prev + max));
                }
            }
            // ClusterHistograms only reaches its kBest pairing without
            // previous histograms; with them it runs FastClusterHistograms
            // and the reindexing.
            for clustering in 0..3 {
                for force_huffman in 0..2 {
                    for (param_max, max) in [(usize::MAX, 255usize), (6, 255), (255, 3)] {
                        let param_max = if param_max == usize::MAX {
                            1 << 20
                        } else {
                            param_max
                        };
                        cluster.push(format!(
                            "ene-cluster {clustering} {force_huffman} {param_max} {max}{tail}"
                        ));
                    }
                }
            }
        }
    }
    (fast, cluster)
}

#[test]
fn enc_simd_fast_cluster_histograms_matches_the_original_target() {
    let (fast, _) = cluster_requests();
    let _lock = dispatch_lock();
    check_rows(
        "ene-fast",
        &fast,
        |r| (observe_cluster(r), Box::new(all_defined)),
        cluster_float,
    );
}

#[test]
fn enc_simd_cluster_histograms_through_the_encoder_dispatch_matches_the_original_target() {
    let (_, cluster) = cluster_requests();
    check_through_dispatch(
        "ene-cluster",
        &cluster,
        |_| true,
        observe_cluster,
        cluster_float,
    );
}

// Whole lossless encodes -------------------------------------------------------

fn hex(bytes: &[u8]) -> String {
    if bytes.is_empty() {
        return "-".to_owned();
    }
    let mut result = String::with_capacity(2 * bytes.len());
    for byte in bytes {
        write!(result, "{byte:02x}").unwrap();
    }
    result
}
fn unhex(hex: &str) -> Vec<u8> {
    if hex == "-" {
        return Vec::new();
    }
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
        .collect()
}

/// The port's observation of a `df-source` request (tools/oracle/
/// decode_frame.inc: the original's EFThunderOneShot of a lossless effort-2
/// frame, `LosslessThunderConfig`'s encode), as tests/parity/decode_frame.rs
/// forms it; the caller sets the encoder's selection.
fn observe_lossless_frame(request: &str) -> String {
    let mut args = request.split_whitespace();
    assert_eq!(args.next(), Some("df-source"));
    let w: usize = number(&mut args);
    let h: usize = number(&mut args);
    let gray = number::<u32>(&mut args) != 0;
    let depth: u32 = number(&mut args);
    let bytes = unhex(args.next().unwrap());
    let config = LosslessThunderConfig::new(
        w,
        h,
        gray,
        if depth == 8 {
            LosslessSampleDepth::Uint8
        } else {
            LosslessSampleDepth::Uint16
        },
    )
    .unwrap();
    let m = config.metadata();
    let mut input = JxlEncoderChunkedFrameAdapter::new(m.xsize(), m.ysize(), 0);
    assert!(input.set_from_buffer(
        0,
        Some(&bytes),
        bytes.len(),
        JxlPixelFormat {
            num_channels: m.m.color_encoding.channels() as u32,
            data_type: if depth == 8 {
                JxlDataType::UINT8
            } else {
                JxlDataType::UINT16
            },
            endianness: JxlEndianness::NATIVE,
            align: 0
        }
    ));
    let frame = config
        .create_frame()
        .unwrap()
        .encode_one_shot(&input, None)
        .unwrap();
    format!("0 {}", hex(frame.as_slice()))
}

/// Pixel words of shape `mode`: 0 flat, 1 two levels, 2 a quadratic ramp
/// (tests/parity/decode_frame.rs), 3 a smooth photo-like field with noise,
/// 4 uniform noise, 5 a few large flat regions with edges.
fn lossless_pixels(w: usize, h: usize, channels: usize, depth: u32, mode: usize) -> Vec<u8> {
    let mut seed = 0x1055_1e55_0000_0011u64 ^ (mode as u64) << 32;
    let max = if depth == 8 { 255u64 } else { 65535 };
    (0..w * h * channels)
        .flat_map(|i| {
            let (p, c) = (i / channels, i % channels);
            let (x, y) = ((p % w) as u64, (p / w) as u64);
            let r = next(&mut seed);
            let value = match mode {
                0 => 0,
                1 => {
                    if i % 3 == 0 {
                        max
                    } else {
                        317 % (max + 1)
                    }
                }
                2 => (i as u64 * 173 + (i * i) as u64 * 29 + 61) & max,
                3 => ((x * 3 + y * 5 + c as u64 * 40) * (max + 1) / 2048 + r % 7) % (max + 1),
                4 => r & max,
                _ => {
                    if (x / 37 + y / 23 + c as u64).is_multiple_of(3) {
                        max / 3
                    } else {
                        max - (x / 64) * 9
                    }
                }
            } as u16;
            if depth == 8 {
                vec![value as u8]
            } else {
                value.to_ne_bytes().to_vec()
            }
        })
        .collect()
}

/// Whole encodes of the engine's lossless profile (effort 2, Thunder; port
/// ADR 0004 "Encoder path" condition (c) for that profile) with the encoder's
/// selection forced to the SIMD target, byte for byte against the original's
/// EFThunderOneShot built for the same single target. The two Highway
/// exports the profile reaches (`enc_rct`, `enc_cluster`) run their SIMD
/// translation; the default selection is unchanged.
#[test]
fn enc_simd_lossless_thunder_frames_with_the_encoder_simd_target_match_the_original_target() {
    let mut requests = Vec::new();
    for (w, h) in [(1, 1), (3, 2), (17, 9), (61, 33), (300, 260), (513, 3)] {
        for gray in [false, true] {
            for depth in [8u32, 16] {
                for mode in 0..6 {
                    if w * h > 10_000 && mode < 2 {
                        continue;
                    }
                    let p = lossless_pixels(w, h, if gray { 1 } else { 3 }, depth, mode);
                    requests.push(format!(
                        "df-source {w} {h} {} {depth} {}",
                        u32::from(gray),
                        hex(&p)
                    ));
                }
            }
        }
    }
    let _lock = dispatch_lock();
    hwy::select_scalar_decoder_for_test();
    hwy::set_encoder_simd_for_test(Some(false));
    let scalar: Vec<String> = requests.iter().map(|r| observe_lossless_frame(r)).collect();
    hwy::set_encoder_simd_for_test(Some(true));
    let simd: Vec<String> = requests.iter().map(|r| observe_lossless_frame(r)).collect();
    hwy::set_encoder_simd_for_test(None);
    let differing = scalar.iter().zip(&simd).filter(|(a, b)| a != b).count();
    println!(
        "lossless frames: the scalar encoder path differs in {differing} of {} requests",
        requests.len()
    );
    let index = std::cell::Cell::new(0);
    check_rows(
        "lossless frames",
        &requests,
        |_| {
            let observed = simd[index.get()].clone();
            index.set(index.get() + 1);
            (observed, Box::new(all_defined))
        },
        no_float,
    );
}
