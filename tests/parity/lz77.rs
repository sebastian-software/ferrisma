// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Exact cost and token observations for the original LZ77 implementations.

use super::*;
use jxl::jxl::dec_ans::LZ77Params;
use jxl::jxl::enc_ans::Token;
use jxl::jxl::enc_ans_params::{Histogram, HistogramParams, LZ77Method};
use jxl::jxl::enc_lz77::{SymbolCostEstimator, apply_lz77, dist_cost, len_cost};

pub(super) fn handles(kind: &str) -> bool {
    kind.starts_with("lz-")
}
fn method(raw: u32) -> LZ77Method {
    match raw {
        0 => LZ77Method::None,
        1 => LZ77Method::RLE,
        2 => LZ77Method::LZ77b1w3f,
        3 => LZ77Method::LZ77b3w3f,
        4 => LZ77Method::LZ77b7w3f,
        5 => LZ77Method::LZ77b15w3f,
        6 => LZ77Method::LZ77b31w3f,
        7 => LZ77Method::LZ77b1w3t,
        8 => LZ77Method::LZ77b3w3t,
        9 => LZ77Method::LZ77b7w3t,
        10 => LZ77Method::LZ77b15w3t,
        11 => LZ77Method::LZ77b31w3t,
        12 => LZ77Method::Optc1,
        13 => LZ77Method::Optc3,
        14 => LZ77Method::Optc8,
        15 => LZ77Method::Optc256,
        _ => panic!("test LZ77 method"),
    }
}
fn token_streams(args: &mut std::str::SplitWhitespace<'_>, flags: bool) -> Vec<Vec<Token>> {
    let count: usize = number(args);
    (0..count)
        .map(|_| {
            let size: usize = number(args);
            (0..size)
                .map(|_| {
                    let mut token = Token::new(number(args), number(args));
                    if flags {
                        token.set_is_lz77_length(number::<u32>(args) != 0);
                    }
                    token
                })
                .collect()
        })
        .collect()
}
pub(super) fn observe(kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    if kind == "lz-static" {
        return format!(
            "{} {}",
            len_cost(number(args)).to_bits(),
            dist_cost(number(args)).to_bits()
        );
    }
    let method = if kind == "lz-apply" {
        method(number(args))
    } else {
        LZ77Method::None
    };
    let prefix: u32 = number(args);
    let contexts: usize = number(args);
    let widths = if kind == "lz-apply" {
        let size: usize = number(args);
        (0..size).map(|_| number(args)).collect()
    } else {
        Vec::new()
    };
    let params = HistogramParams {
        force_huffman: prefix != 0,
        lz77_method: method,
        image_widths: widths,
        ..HistogramParams::default()
    };
    let tokens = token_streams(args, kind == "lz-cost");
    let lz77 = LZ77Params {
        min_symbol: if prefix != 0 { 512 } else { 224 },
        nonserialized_distance_context: contexts,
        ..LZ77Params::default()
    };
    if kind == "lz-apply" {
        let streams = apply_lz77(&params, contexts, &tokens, &lz77);
        let mut result = streams.len().to_string();
        for stream in streams {
            write!(result, " {}", stream.len()).unwrap();
            for token in stream {
                write!(
                    result,
                    " {},{},{}",
                    token.context(),
                    token.value,
                    u32::from(token.is_lz77_length())
                )
                .unwrap();
            }
        }
        result
    } else {
        let model = SymbolCostEstimator::new(contexts, prefix != 0, &tokens, &lz77);
        let mut histograms = vec![Histogram::default(); contexts];
        let mut alphabet = 0;
        for stream in &tokens {
            for token in stream {
                let config = if token.is_lz77_length() {
                    lz77.length_uint_config
                } else {
                    jxl::jxl::dec_ans::HybridUintConfig::default()
                };
                let (mut symbol, _, _) = config.encode(token.value);
                if token.is_lz77_length() {
                    symbol += lz77.min_symbol;
                }
                histograms[token.context() as usize].add(symbol as usize);
                alphabet = alphabet.max(histograms[token.context() as usize].counts.len());
            }
        }
        let mut result = format!("{contexts} {alphabet}");
        for ctx in 0..contexts {
            write!(result, " {}", model.add_symbol_cost(ctx).to_bits()).unwrap();
            for sym in 0..alphabet {
                write!(result, " {}", model.bits(ctx, sym).to_bits()).unwrap();
            }
        }
        result
    }
}

fn input_values(size: usize, trial: usize, seed: &mut u64) -> Vec<u32> {
    (0..size)
        .map(|i| match trial {
            0 => 0,
            1 => (i % 2) as u32,
            2 => (i % 13) as u32,
            3 => {
                if i % 67 < 63 {
                    0
                } else {
                    i as u32
                }
            }
            4 => (next(seed) % 8) as u32,
            5 => next(seed) as u32,
            _ => ((i / 31) % 5) as u32,
        })
        .collect()
}

#[test]
fn lz77_cost_tables_and_model_float_bits_match_original() {
    let mut cases = Vec::new();
    let mut seed = 2181;
    for value in (0..4096usize).chain([65535, 1 << 20, u32::MAX as usize, usize::MAX]) {
        cases.push(format!("lz-static {value} {}", value.wrapping_mul(31)));
    }
    for contexts in [1, 2, 3] {
        for size in [0, 1, 7, 31, 257, 1024] {
            for trial in 0..7 {
                for prefix in [0, 1] {
                    let mut line = format!("lz-cost {prefix} {contexts} 2");
                    for stream in 0..2 {
                        let values = input_values(size, trial, &mut seed);
                        write!(line, " {size}").unwrap();
                        for (i, value) in values.into_iter().enumerate() {
                            write!(
                                line,
                                " {} {value} {}",
                                (i + stream) % contexts,
                                u32::from(trial == 6 && i % 7 == 0)
                            )
                            .unwrap();
                        }
                    }
                    cases.push(line);
                }
            }
        }
    }
    let mut regression = "lz-cost 1 1 1 13".to_owned();
    for value in 0..13 {
        write!(regression, " 0 {value} 0").unwrap();
    }
    cases.push(regression);
    // Upstream disables Clang auto-vectorization, retaining fused AArch64
    // rounding in both the scalar accumulation and the final penalty.
    let mut regression = "lz-cost 0 1 1 514".to_owned();
    for (value, count) in [73, 62, 56, 58, 56, 71, 68, 70].into_iter().enumerate() {
        for _ in 0..count {
            write!(regression, " 0 {value} 0").unwrap();
        }
    }
    cases.push(regression);
    compare(cases);
}

#[test]
fn all_lz77_methods_preserve_exact_token_streams_and_distance_collisions() {
    let mut cases = Vec::new();
    let mut seed = 8391;
    for size in [0, 1, 2, 3, 4, 7, 31, 64, 257, 1024] {
        for trial in 0..7 {
            let values = input_values(size, trial, &mut seed);
            for method in 0..16 {
                for prefix in [0, 1] {
                    for width in [0, 1, 3] {
                        let mut line =
                            format!("lz-apply {method} {prefix} 3 2 {width} {} 2", width * 7);
                        for stream in 0..2 {
                            write!(line, " {size}").unwrap();
                            for (i, &value) in values.iter().enumerate() {
                                write!(line, " {} {value}", (i + stream) % 3).unwrap();
                            }
                        }
                        cases.push(line);
                    }
                }
            }
        }
    }
    compare(cases);
}
