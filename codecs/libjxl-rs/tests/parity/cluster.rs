// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Exact observations of the original scalar math and histogram clustering.

use super::*;
use jxl::jxl::base::fast_math::fast_log2f;
use jxl::jxl::common::SpeedTier;
use jxl::jxl::enc_ans_params::{ClusteringType, Histogram, HistogramParams, HybridUintMethod};
use jxl::jxl::enc_cluster::{cluster_histograms, histogram_distance, histogram_kl_divergence};

fn read_histogram(args: &mut std::str::SplitWhitespace<'_>, cache: f32) -> Histogram {
    let size: usize = number(args);
    let counts: Vec<i32> = (0..size).map(|_| number(args)).collect();
    Histogram {
        total_count: counts.iter().map(|&n| n as usize).sum(),
        counts,
        entropy: cache,
    }
}
fn histogram_observation(result: &mut String, histogram: &Histogram) {
    write!(
        result,
        " {},{},{}",
        histogram.counts.len(),
        histogram.total_count,
        histogram.entropy.to_bits()
    )
    .unwrap();
    for count in &histogram.counts {
        write!(result, ",{count}").unwrap();
    }
}

pub(super) fn handles(kind: &str) -> bool {
    matches!(
        kind,
        "fast-log" | "hist-measures" | "cluster" | "hist-params"
    )
}
pub(super) fn observe(kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    match kind {
        "fast-log" => fast_log2f(f32::from_bits(number(args)))
            .to_bits()
            .to_string(),
        "hist-measures" => {
            let mut a = read_histogram(args, 1.25);
            let mut b = read_histogram(args, 1.25);
            a.condition();
            b.condition();
            let mut result = String::new();
            histogram_observation(&mut result, &a);
            histogram_observation(&mut result, &b);
            write!(
                result,
                " {} {}",
                a.shannon_entropy().to_bits(),
                b.shannon_entropy().to_bits()
            )
            .unwrap();
            write!(
                result,
                " {} {} {}",
                histogram_distance(&a, &b).to_bits(),
                histogram_kl_divergence(&a, &b).to_bits(),
                histogram_kl_divergence(&b, &a).to_bits()
            )
            .unwrap();
            result
        }
        "cluster" => {
            let clustering = match number::<u32>(args) {
                0 => ClusteringType::Fastest,
                1 => ClusteringType::Fast,
                2 => ClusteringType::Best,
                _ => panic!("test clustering"),
            };
            let (max_histograms, max_param) = (number(args), number(args));
            let prefix: u32 = number(args);
            let (n, previous): (usize, usize) = (number(args), number(args));
            let params = HistogramParams {
                clustering,
                max_histograms: max_param,
                force_huffman: prefix != 0,
                ..HistogramParams::default()
            };
            let mut input: Vec<_> = (0..n).map(|_| read_histogram(args, 0.25)).collect();
            let mut out: Vec<_> = (0..previous).map(|_| read_histogram(args, 0.75)).collect();
            let mut symbols = Vec::new();
            let status =
                cluster_histograms(&params, &mut input, max_histograms, &mut out, &mut symbols);
            let mut result = format!("{} {}", status_code(&status), symbols.len());
            for symbol in symbols {
                write!(result, " {symbol}").unwrap();
            }
            write!(result, " {}", input.len()).unwrap();
            for histogram in input {
                histogram_observation(&mut result, &histogram);
            }
            write!(result, " {}", out.len()).unwrap();
            for histogram in out {
                histogram_observation(&mut result, &histogram);
            }
            result
        }
        "hist-params" => {
            let tier = match number::<i32>(args) {
                -1 => SpeedTier::TectonicPlate,
                0 => SpeedTier::Glacier,
                1 => SpeedTier::Tortoise,
                2 => SpeedTier::Kitten,
                3 => SpeedTier::Squirrel,
                4 => SpeedTier::Wombat,
                5 => SpeedTier::Hare,
                6 => SpeedTier::Cheetah,
                7 => SpeedTier::Falcon,
                8 => SpeedTier::Thunder,
                9 => SpeedTier::Lightning,
                _ => panic!("test tier"),
            };
            let method = match number::<u32>(args) {
                0 => HybridUintMethod::None,
                1 => HybridUintMethod::Uint000,
                2 => HybridUintMethod::Fast,
                3 => HybridUintMethod::ContextMap,
                4 => HybridUintMethod::Best,
                _ => panic!("test method"),
            };
            let mut params = HistogramParams::new(tier, 17);
            let mut result = format!(
                "{} {} {} {}",
                params.clustering as u32,
                params.uint_method as u32,
                params.lz77_method as u32,
                params.ans_histogram_strategy as u32
            );
            params.uint_method = method;
            let cfg = params.uint_config();
            write!(
                result,
                " {},{},{},{}",
                cfg.split_exponent, cfg.split_token, cfg.msb_in_token, cfg.lsb_in_token
            )
            .unwrap();
            result
        }
        _ => panic!("unknown cluster command"),
    }
}

fn histogram_request(line: &mut String, counts: &[i32]) {
    write!(line, " {}", counts.len()).unwrap();
    for count in counts {
        write!(line, " {count}").unwrap();
    }
}
fn generated_histogram(size: usize, trial: usize, seed: &mut u64) -> Vec<i32> {
    (0..size)
        .map(|i| match trial % 7 {
            0 => 0,
            1 => i32::from(i == trial % size) * 100000,
            2 => 1,
            3 => (i % 3 + 1) as i32,
            4 => {
                if i % 7 == 0 {
                    (next(seed) % 1000) as i32
                } else {
                    0
                }
            }
            5 => (next(seed) % 10000) as i32,
            _ => {
                if i == 0 {
                    1000000
                } else {
                    1
                }
            }
        })
        .collect()
}

#[test]
fn scalar_fast_log_matches_original_float_bits() {
    let mut cases = Vec::new();
    let mut seed = 5987;
    let mut values = vec![0, 1, 0x007fffff, 0x00800000, 0x7f7fffff, 0x7f800000];
    for exponent in 0..255 {
        for mantissa in [0, 1, 0x2aaaaa, 0x2aaaab, 0x3fffff, 0x7fffff] {
            values.push(exponent << 23 | mantissa);
        }
    }
    for _ in 0..16384 {
        values.push(next(&mut seed) as u32 % 0x7f800001);
    }
    for value in values {
        cases.push(format!("fast-log {value}"));
    }
    compare(cases);
}

#[test]
fn histogram_condition_entropy_and_divergence_match_scalar_source() {
    let mut cases = Vec::new();
    let mut seed = 10984;
    for size in [0, 1, 2, 3, 7, 8, 9, 31, 64, 256, 4096] {
        for trial in 0..28 {
            let a = generated_histogram(size, trial, &mut seed);
            let b = generated_histogram(
                if trial % 2 == 0 { size } else { size / 2 },
                trial + 3,
                &mut seed,
            );
            let mut line = "hist-measures".to_owned();
            histogram_request(&mut line, &a);
            histogram_request(&mut line, &b);
            cases.push(line);
        }
    }
    compare(cases);
}

#[test]
fn histogram_clusters_preserve_caches_pair_order_and_existing_prefix() {
    let mut cases = Vec::new();
    let mut seed = 7891;
    for n in [0usize, 1, 2, 3, 8, 18] {
        for size in [1, 3, 32, 256, 257] {
            for previous in 0..=2 {
                if n == 0 && previous > 0 {
                    continue;
                } // Source dereferences an empty max_element here.
                for clustering in 0..3 {
                    for prefix in [0, 1] {
                        let mut line = format!(
                            "cluster {clustering} {} {} {prefix} {n} {previous}",
                            n.div_ceil(2),
                            if clustering == 0 { 2 } else { 128 }
                        );
                        for i in 0..n {
                            histogram_request(
                                &mut line,
                                &generated_histogram(size, i + clustering, &mut seed),
                            );
                        }
                        for i in 0..previous {
                            histogram_request(
                                &mut line,
                                &generated_histogram(size, i + 2, &mut seed),
                            );
                        }
                        cases.push(line);
                    }
                }
            }
        }
    }
    for clustering in 0..3 {
        cases.push(format!("cluster {clustering} 0 0 0 1 0 1 3"));
    }
    for tier in -1..=9 {
        for method in 0..5 {
            cases.push(format!("hist-params {tier} {method}"));
        }
    }
    compare(cases);
}
