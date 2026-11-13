// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! SIMD parity of the VarDCT decode kernels (port ADR 0004, Plan item 2):
//! the port's NEON/AVX2 instantiations of ComputeScaled(I)DCT,
//! TransformToPixels, LowestFrequenciesFromDC, AdjustQuantBias and
//! DequantBlock against the original built for the same single Highway
//! target (`JXL_CPP_ORACLE_SIMD`). The requests are the scalar parity
//! suite's `dct-*`/`dq-*` protocol (tools/oracle/dct.cc, dequant.cc), whose
//! adapters call the original's target namespace (`ORACLE_HWY_NS`). Every
//! observed word (mutated input, output, scratch) is compared bit for bit
//! except that any NaN equals any NaN (decision D8). `dq-lane` is not used:
//! its adapter calls DequantLane at every `k`, so the original's full-vector
//! loads read past the request's matrix (memory it never initialized);
//! `dq-block` steps by `Lanes` as the decoder does.
#![cfg(feature = "cpp-oracle-simd")]

mod simd_common;

use jxl::jxl::ac_strategy::{AcStrategy, AcStrategyType};
use jxl::jxl::dct_util::ConstAcPtr;
use jxl::jxl::dec_group_simd::{DequantBlockArgs, adjust_quant_bias_simd, dequant_block_simd};
use jxl::jxl::dec_transforms_simd::{
    compute_scaled_simd, lowest_frequencies_from_dc_simd, transform_to_pixels_simd,
};
use jxl::jxl::hwy::Target;
use jxl::jxl::memory_manager_internal::MemoryManagerPtr;
use jxl::jxl::quant_weights::{
    AC_STRATEGY_TO_QUANT_TABLE_MAP, DctQuantWeightParams, DequantMatrices, QuantEncoding,
    QuantEncodingData,
};
use jxl::jxl::quantizer::Quantizer;
use simd_common::{next, oracle, same, target};
use std::fmt::Write as _;

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
fn buffers(out: &mut String, buffers: &[&[f32]]) {
    for buf in buffers {
        out.push_str(" B");
        for v in *buf {
            write!(out, " {}", v.to_bits()).unwrap();
        }
    }
}

/// The port's observation of a `dct-scaled`/`dct-pixels`/`dct-llf` request,
/// in the format of tools/oracle/dct.cc.
fn observe_dct(request: &str, t: Target) -> String {
    let mut args = request.split_whitespace();
    let kind = args.next().unwrap();
    let rows: usize = number(&mut args);
    let cols: usize = number(&mut args);
    let stride: usize = number(&mut args);
    let option: u32 = number(&mut args);
    let inverse = kind != "dct-scaled" || option != 0;
    let count = if kind == "dct-llf" || !inverse {
        rows * stride
    } else {
        rows * cols
    };
    let mut input = vec![-123.25; count + 4];
    for v in input.iter_mut().take(count) {
        *v = float(&mut args);
    }
    let out_len = if kind == "dct-llf" {
        rows * cols * 64
    } else if inverse {
        rows * stride
    } else {
        rows * cols
    };
    let mut output = vec![-123.25; out_len + 4];
    let mut scratch = vec![-123.25; rows * cols * 4 + 3 * (rows + cols) + 4];
    match kind {
        "dct-scaled" => compute_scaled_simd(
            t,
            inverse,
            rows,
            cols,
            &mut input,
            stride,
            &mut output,
            &mut scratch,
        ),
        "dct-pixels" => transform_to_pixels_simd(
            t,
            AcStrategyType(option),
            &mut input,
            &mut output,
            stride,
            &mut scratch,
        ),
        "dct-llf" => lowest_frequencies_from_dc_simd(
            t,
            AcStrategyType(option),
            &input,
            stride,
            &mut output,
            &mut scratch,
        ),
        other => panic!("unknown request {other}"),
    }
    let mut out = String::new();
    buffers(&mut out, &[&input, &output, &scratch]);
    out.trim_start().to_owned()
}

const SPECIAL: [u32; 12] = [
    1, 0x80000001, 0x007fffff, 0x00800000, 0x7f7fffff, 0xff7fffff, 0x7f800000, 0xff800000,
    0x7fc12345, 0xffc98765, 0x7fa12345, 0xffa98765,
];

/// The scalar suite's `dct-*` request (tests/parity/dct.rs) with value
/// modes 0-18 (zeros, signed zeros, impulses, ramps, random finite values,
/// extremes, and one special word at `count / 3`), extended by modes 19-23:
/// random finite values of other seeds and magnitudes, subnormals, and
/// special words spread over the block.
fn dct_request(
    kind: &str,
    rows: usize,
    cols: usize,
    pad: usize,
    option: u32,
    mode: usize,
) -> String {
    let stride = cols + pad;
    let count = if kind == "dct-llf" || (kind == "dct-scaled" && option == 0) {
        rows * stride
    } else {
        rows * cols
    };
    let mut values = vec![0.0f32; count];
    let mut seed = 0x27182818 + mode as u64 * 7919;
    for (i, v) in values.iter_mut().enumerate() {
        *v = match mode {
            0 => 0.0,
            1 => -0.0,
            2 => f32::from(u8::from(i == 0)),
            3 => {
                if i == count / 3 {
                    -1.75
                } else {
                    0.0
                }
            }
            4 => (i as i32 % 23 - 11) as f32 / 16.0,
            5 | 19 => (next(&mut seed) as i32 % 16384) as f32 / 32.0,
            6 => {
                if i % 2 == 0 {
                    f32::MAX
                } else {
                    -f32::MAX
                }
            }
            20 => ((next(&mut seed) % 2_000_001) as f32 - 1_000_000.0) / 3.0,
            21 => f32::from_bits((next(&mut seed) as u32 & 0x807f_ffff) | 1),
            22 => f32::from_bits(next(&mut seed) as u32),
            23 => {
                if i % 5 == 0 {
                    f32::from_bits(SPECIAL[(i / 5) % SPECIAL.len()])
                } else {
                    (next(&mut seed) as i32 % 4096) as f32 / 64.0
                }
            }
            _ => 0.125,
        };
    }
    if (7..19).contains(&mode) {
        values[count / 3] = f32::from_bits(SPECIAL[(mode - 7) % SPECIAL.len()]);
    }
    let mut s = format!("{kind} {rows} {cols} {stride} {option}");
    for v in values {
        write!(s, " {}", v.to_bits()).unwrap();
    }
    s
}

const MODES: usize = 24;

fn check(family: &str, requests: &[String], observe: impl Fn(&str) -> String) {
    let t = target();
    let observed = oracle(requests);
    let mut words = 0;
    for (request, cpp) in requests.iter().zip(&observed) {
        let rust = observe(request);
        words += same(&rust, cpp, |_| true, |_| true)
            .unwrap_or_else(|e| panic!("{family} on {}: {e}\nrequest: {request}", t.name()));
    }
    println!(
        "{family} on {}: {} requests, {words} words matched",
        t.name(),
        requests.len()
    );
}

#[test]
fn simd_scaled_dct_and_idct_match_the_original_target() {
    let t = target();
    let mut requests = Vec::new();
    for rows in [1, 2, 4, 8, 16, 32] {
        for cols in [1, 2, 4, 8, 16, 32] {
            for inverse in 0..2 {
                for pad in [0, 3] {
                    for mode in 0..MODES {
                        requests.push(dct_request("dct-scaled", rows, cols, pad, inverse, mode));
                    }
                }
            }
        }
    }
    // All large strategy shapes plus anisotropic one-dimensional recursion.
    for (rows, cols) in [
        (64, 32),
        (32, 64),
        (64, 64),
        (128, 64),
        (64, 128),
        (128, 128),
        (256, 128),
        (128, 256),
        (256, 256),
        (1, 256),
        (256, 1),
        (2, 128),
        (128, 4),
    ] {
        for inverse in 0..2 {
            for mode in [2, 3, 5, 9, 15, 21, 23] {
                requests.push(dct_request("dct-scaled", rows, cols, 7, inverse, mode));
            }
        }
    }
    check("dct-scaled", &requests, |r| observe_dct(r, t));
}

#[test]
fn simd_transform_to_pixels_matches_the_original_target_for_every_strategy() {
    let t = target();
    let mut requests = Vec::new();
    for raw in 0..27 {
        let acs = AcStrategy::from_raw_strategy(AcStrategyType(raw));
        let rows = acs.covered_blocks_y() * 8;
        let cols = acs.covered_blocks_x() * 8;
        let modes: Vec<usize> = if rows.max(cols) <= 32 {
            (0..MODES).collect()
        } else {
            vec![0, 2, 3, 4, 5, 9, 14, 19, 21, 23]
        };
        for pad in [0, 5] {
            for &mode in &modes {
                requests.push(dct_request("dct-pixels", rows, cols, pad, raw, mode));
            }
        }
    }
    // IDENTITY's contracted residual: subnormal and signed-zero rounding
    // (the scalar suite's cases).
    for sign in [0, 0x80000000u32] {
        for dc in [0, 1, 2, 3, 0x7fffff, 0x800000] {
            for count in 0..16 {
                let mut values = [0u32; 64];
                values[0] = sign | dc;
                let positions = [2, 4, 6, 16, 18, 20, 22, 32, 34, 36, 38, 48, 50, 52, 54];
                for &i in positions.iter().take(count) {
                    values[i] = sign | 1;
                }
                let mut request = String::from("dct-pixels 8 8 13 1");
                for v in values {
                    write!(request, " {v}").unwrap();
                }
                requests.push(request);
            }
        }
    }
    check("dct-pixels", &requests, |r| observe_dct(r, t));
}

#[test]
fn simd_lowest_frequencies_from_dc_match_the_original_target() {
    let t = target();
    let mut requests = Vec::new();
    for raw in 0..27 {
        let acs = AcStrategy::from_raw_strategy(AcStrategyType(raw));
        let rows = acs.covered_blocks_y();
        let cols = acs.covered_blocks_x();
        for pad in [0, 3] {
            for mode in 0..MODES {
                requests.push(dct_request("dct-llf", rows, cols, pad, raw, mode));
            }
        }
    }
    check("dct-llf", &requests, |r| observe_dct(r, t));
}

const DEFAULT_QUANT_BIAS: [f32; 4] = [
    1.0 - f32::from_bits(0x3d5fd8c1),
    1.0 - f32::from_bits(0x3d8f78bc),
    1.0 - f32::from_bits(0x3d4c88c0),
    0.145,
];

#[test]
fn simd_adjust_quant_bias_matches_the_original_target() {
    let t = target();
    let mut integers = vec![
        0,
        1,
        -1,
        2,
        -2,
        i32::MIN,
        i32::MAX,
        16777215,
        16777216,
        16777217,
        -16777217,
    ];
    for n in 1..31 {
        for offset in [-1, 0, 1] {
            integers.push((1i32 << n) + offset);
            integers.push(-(1i32 << n) + offset);
        }
    }
    let mut seed = 0x5eed;
    for _ in 0..200 {
        integers.push(next(&mut seed) as i32 >> (next(&mut seed) % 31));
    }
    let words = [
        0, 0x80000000, 1, 0x007fffff, 0x00800000, 0x3f000000, 0x3f800000, 0x7f7fffff, 0x7f800000,
        0xff800000, 0x7fc12345, 0xffc98765, 0x7fa12345, 0xffa98765,
    ];
    let mut requests = Vec::new();
    for c in 0..3 {
        for index in 0..4 {
            for word in words {
                let mut biases = DEFAULT_QUANT_BIAS;
                biases[index] = f32::from_bits(word);
                let mut s = format!("dq-bias {c} {}", integers.len());
                for v in biases {
                    write!(s, " {}", v.to_bits()).unwrap();
                }
                for v in &integers {
                    write!(s, " {v}").unwrap();
                }
                requests.push(s);
            }
        }
    }
    check("dq-bias", &requests, |r| {
        let mut args = r.split_whitespace();
        args.next();
        let c: usize = number(&mut args);
        let n: usize = number(&mut args);
        let biases: [f32; 4] = std::array::from_fn(|_| float(&mut args));
        let quant: Vec<i32> = (0..n).map(|_| number(&mut args)).collect();
        let mut out = String::from("B");
        for v in adjust_quant_bias_simd(t, c, &quant, &biases) {
            write!(out, " {}", v.to_bits()).unwrap();
        }
        out
    });
}

/// The scalar suite's `dq-block` request (tests/parity/dequant.rs), with
/// more value modes: 0-3 the scalar suite's quantized patterns with finite
/// DC, 4-7 the same with special DC words, 8-11 random quantized values.
fn dq_request(raw: u32, ty: u32, mode: usize) -> String {
    let acs = AcStrategy::from_raw_strategy(AcStrategyType(raw));
    let size = acs.covered_blocks_x() * acs.covered_blocks_y() * 64;
    let mut out = format!("dq-block {raw} {ty} {size}");
    let quant = [1, 3, 256, 65536][mode % 4];
    write!(
        out,
        " {} {quant}",
        [0.125f32, 3.75, 1.0 / 65536.0, 128.0][mode % 4].to_bits()
    )
    .unwrap();
    for v in [0.8f32, 1.25, -0.125, 0.375]
        .into_iter()
        .chain(DEFAULT_QUANT_BIAS)
    {
        write!(out, " {}", v.to_bits()).unwrap();
    }
    let special = [
        0, 0x80000000, 1, 0x80000001, 0x00800000, 0x7f7fffff, 0xff7fffff, 0x7f800000, 0xff800000,
        0x7fc12345, 0xffc98765, 0x7fa12345, 0xffa98765,
    ];
    let mut seed = 0xd00d + mode as u64;
    for c in 0..3 {
        for k in 0..size {
            let v = if mode >= 8 {
                (next(&mut seed) as i32) >> (next(&mut seed) % 32)
            } else {
                match mode % 4 {
                    0 => [0, 1, -1, 2, -2, 3, -3, 32767, -32768][(k + c) % 9],
                    1 => (k as i32 * 173 + c as i32 * 31) % 65536 - 32768,
                    2 => {
                        if k == size / 3 {
                            i32::MIN + c as i32
                        } else {
                            0
                        }
                    }
                    _ => {
                        if k % 11 == 0 {
                            i32::MAX - c as i32
                        } else {
                            (k % 193) as i32 - 96
                        }
                    }
                }
            };
            write!(out, " {v}").unwrap();
        }
    }
    let stride = acs.covered_blocks_x() + 3;
    write!(out, " {stride} 1 2 0").unwrap();
    for c in 0..3 {
        for k in 0..acs.covered_blocks_y() * stride + 4 {
            let v = if (4..8).contains(&mode) && k % 11 == 0 {
                f32::from_bits(special[(k + c) % special.len()])
            } else if mode >= 8 {
                ((next(&mut seed) % 4001) as f32 - 2000.0) / 512.0
            } else {
                ((k + c * 3) % 37) as f32 / 32.0 - 0.5
            };
            write!(out, " {}", v.to_bits()).unwrap();
        }
    }
    out
}

fn observe_dq_block(request: &str, t: Target) -> String {
    let mut args = request.split_whitespace();
    assert_eq!(args.next(), Some("dq-block"));
    let raw: u32 = number(&mut args);
    let ty: u32 = number(&mut args);
    let size: usize = number(&mut args);
    let inv = float(&mut args);
    let quant: i32 = number(&mut args);
    let xdm = float(&mut args);
    let bdm = float(&mut args);
    let xcc = float(&mut args);
    let bcc = float(&mut args);
    let biases: [f32; 4] = std::array::from_fn(|_| float(&mut args));
    let q32: [Vec<i32>; 3] =
        std::array::from_fn(|_| (0..size).map(|_| number(&mut args)).collect());
    let q16: [Vec<i16>; 3] = q32
        .each_ref()
        .map(|values| values.iter().map(|&v| v as i16).collect());
    let blocks = if ty == 0 {
        q16.each_ref().map(|v| ConstAcPtr::I16(v))
    } else {
        q32.each_ref().map(|v| ConstAcPtr::I32(v))
    };
    let mut block = vec![-123.25; size * 3 + 4];
    let mut scratch = vec![-123.25; size * 4 + 260];
    let acs = AcStrategy::from_raw_strategy(AcStrategyType(raw));
    let stride: usize = number(&mut args);
    let sbx = std::array::from_fn(|_| number(&mut args));
    let dc: [Vec<f32>; 3] = std::array::from_fn(|_| {
        (0..acs.covered_blocks_y() * stride + 4)
            .map(|_| float(&mut args))
            .collect()
    });
    // The original's EnsureComputed dispatches ComputeQuantTable to its
    // target too (quant_weights.cc is a Highway export).
    let mut matrices = DequantMatrices::default();
    matrices
        .ensure_computed_for_target(MemoryManagerPtr::NATIVE, 1 << raw, Some(t))
        .unwrap();
    let quantizer = Quantizer::new(&matrices);
    dequant_block_simd(
        t,
        DequantBlockArgs {
            inv_global_scale: inv,
            quant,
            x_dm_multiplier: xdm,
            b_dm_multiplier: bdm,
            x_cc_mul: xcc,
            b_cc_mul: bcc,
            kind: acs.strategy(),
            size,
            quantizer: &quantizer,
            matrices: &matrices,
            covered_blocks: acs.covered_blocks_x() * acs.covered_blocks_y(),
            sbx,
            dc_rows: dc.each_ref().map(|v| &v[..]),
            dc_stride: stride,
            biases: &biases,
            qblock: &blocks,
            block: &mut block,
            scratch: &mut scratch,
        },
    );
    let mut out = String::from("O");
    buffers(&mut out, &[&block, &scratch]);
    out
}

#[test]
fn simd_dequant_blocks_match_the_original_target_for_every_strategy() {
    let t = target();
    let mut requests = Vec::new();
    for raw in 0..27 {
        for ty in 0..2 {
            for mode in 0..12 {
                requests.push(dq_request(raw, ty, mode));
            }
        }
    }
    check("dq-block", &requests, |r| observe_dq_block(r, t));
}

/// The scalar suite's `qac-compute` encoding recipe (tests/parity/quant_ac.rs).
fn recipe(q: &QuantEncoding) -> String {
    let mut out = (q.mode() as u32).to_string();
    for p in [&q.dct_params, &q.dct_params_afv_4x4] {
        write!(out, " 1 {}", p.num_distance_bands).unwrap();
        for row in p.distance_bands {
            for v in row {
                write!(out, " {}", v.to_bits()).unwrap();
            }
        }
    }
    let mut weights = [0.0f32; 27];
    match q.data() {
        QuantEncodingData::Identity(w) => {
            for c in 0..3 {
                weights[c * 3..c * 3 + 3].copy_from_slice(&w[c]);
            }
        }
        QuantEncodingData::Dct2(w) => {
            for c in 0..3 {
                weights[c * 6..c * 6 + 6].copy_from_slice(&w[c]);
            }
        }
        QuantEncodingData::Dct4(w) => {
            for c in 0..3 {
                weights[c * 2..c * 2 + 2].copy_from_slice(&w[c]);
            }
        }
        QuantEncodingData::Dct4X8(w) => weights[..3].copy_from_slice(w),
        QuantEncodingData::Afv(w) => {
            for c in 0..3 {
                weights[c * 9..c * 9 + 9].copy_from_slice(&w[c]);
            }
        }
        QuantEncodingData::Raw { .. } => {}
    }
    for v in weights {
        write!(out, " {}", v.to_bits()).unwrap();
    }
    if let QuantEncodingData::Raw {
        qtable: Some(table),
        ..
    } = q.data()
    {
        write!(out, " 0 {}", table.len()).unwrap();
        for v in table {
            write!(out, " {v}").unwrap();
        }
    } else {
        out.push_str(" 0 0");
    }
    out
}

/// The port's `qac-compute` observation with ComputeQuantTable on `t`.
fn observe_quant_table(t: Target, strategy: u8, q: &QuantEncoding, masks: &[u32]) -> String {
    let mut encodings = vec![QuantEncoding::library::<0>(); 17];
    encodings[AC_STRATEGY_TO_QUANT_TABLE_MAP[strategy as usize] as usize] = q.clone();
    let mut m = DequantMatrices::default();
    m.set_encodings(&encodings);
    let mut out = String::from("C");
    for &mask in masks {
        let status = m.ensure_computed_for_target(MemoryManagerPtr::NATIVE, mask, Some(t));
        write!(
            out,
            " {}",
            jxl::jxl::base::status::status_code(&status) as i32
        )
        .unwrap();
        write!(
            out,
            " {} {}",
            m.computed_mask,
            u8::from(m.have_table_storage())
        )
        .unwrap();
        if m.computed_mask & (1 << strategy) == 0 {
            continue;
        }
        for c in 0..3 {
            for (&value, &inverse) in m
                .matrix(AcStrategyType(u32::from(strategy)), c)
                .iter()
                .zip(m.inv_matrix(AcStrategyType(u32::from(strategy)), c))
            {
                write!(out, " {},{}", value.to_bits(), inverse.to_bits()).unwrap();
            }
        }
    }
    out
}

#[test]
fn simd_quant_tables_match_the_original_target() {
    let t = target();
    let mut cases: Vec<(u8, QuantEncoding, Vec<u32>)> = Vec::new();
    // The library tables of every strategy, alone and with shared kinds.
    for strategy in 0..27u8 {
        cases.push((strategy, QuantEncoding::library::<0>(), vec![1 << strategy]));
    }
    for (a, b) in [
        (6, 7),
        (8, 9),
        (10, 11),
        (12, 13),
        (14, 17),
        (19, 20),
        (22, 23),
        (25, 26),
    ] {
        cases.push((
            a,
            QuantEncoding::library::<0>(),
            vec![1 << b, 1 << a, (1 << a) | (1 << b)],
        ));
    }
    // Custom modes (the scalar suite's set) and random distance bands.
    let mut band_sets = vec![DctQuantWeightParams::from_bands(&[
        [128.0, 0.25, -0.5],
        [256.0, -0.25, 0.5],
        [64.0, 1.0, -1.0],
    ])];
    let mut seed = 0xbadc0ffe;
    for count in [1usize, 2, 4, 6, 8, 16] {
        for _ in 0..3 {
            let mut bands = [[0.0f32; 17]; 3];
            for row in bands.iter_mut() {
                row[0] = 1.0 + (next(&mut seed) % 4000) as f32 / 8.0;
                for value in row.iter_mut().take(count).skip(1) {
                    *value = ((next(&mut seed) % 4001) as f32 - 2000.0) / 1000.0;
                }
            }
            band_sets.push(DctQuantWeightParams::new(bands, count));
        }
    }
    for bands in &band_sets {
        let encodings = [
            QuantEncoding::dct4(bands, &[[0.5, 2.0], [2.0, 0.5], [1.0, 1.0]]),
            QuantEncoding::dct4x8(bands, &[0.5, 1.0, 2.0]),
            QuantEncoding::afv(
                bands,
                bands,
                &[[1.0, 2.0, 3.0, 4.0, 5.0, 10.0, 0.5, -0.5, 1.0]; 3],
            ),
            QuantEncoding::dct(bands),
        ];
        for strategy in [0u8, 1, 2, 3, 4, 6, 8, 10, 12, 14, 18, 19, 21, 22, 24, 25] {
            for q in &encodings {
                if strategy >= 4 && strategy != 12 && strategy != 14 && q.mode() as u32 != 6 {
                    continue;
                }
                if strategy == 14 && q.mode() as u32 != 5 && q.mode() as u32 != 6 {
                    continue;
                }
                cases.push((strategy, q.clone(), vec![1 << strategy]));
            }
        }
    }
    cases.push((
        1,
        QuantEncoding::identity(&[[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]),
        vec![1 << 1],
    ));
    cases.push((
        2,
        QuantEncoding::dct2(&[[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]; 3]),
        vec![1 << 2],
    ));
    let mut table = vec![0; 192];
    for (i, v) in table.iter_mut().enumerate() {
        *v = 1 + (i as i32 * 13) % 255;
    }
    cases.push((0, QuantEncoding::raw(&table, 0), vec![1]));
    // Single-band NaN words bypass FastPowf (the scalar suite's cases).
    for word in [0x7fc1_2345u32, 0xffc2_3456, 0x7fa1_2345] {
        let bands = DctQuantWeightParams::from_bands(&[[f32::from_bits(word)]; 3]);
        cases.push((
            0,
            QuantEncoding::dct4x8(&bands, &[f32::from_bits(word); 3]),
            vec![1],
        ));
    }
    let requests: Vec<String> = cases
        .iter()
        .map(|(strategy, q, masks)| {
            let mut r = format!("qac-compute {strategy} {} {}", recipe(q), masks.len());
            for m in masks {
                write!(r, " {m}").unwrap();
            }
            r
        })
        .collect();
    let observed = oracle(&requests);
    let mut words = 0;
    for ((request, cpp), (strategy, q, masks)) in requests.iter().zip(&observed).zip(&cases) {
        let rust = observe_quant_table(t, *strategy, q, masks);
        // Table words are "value,inverse" pairs; compare each half with the
        // NaN rule.
        let split = |s: &str| s.replace(',', " ");
        words += same(&split(&rust), &split(cpp), |_| true, |_| true)
            .unwrap_or_else(|e| panic!("qac-compute on {}: {e}\nrequest: {request}", t.name()));
    }
    println!(
        "qac-compute on {}: {} requests, {words} words matched",
        t.name(),
        requests.len()
    );
}
