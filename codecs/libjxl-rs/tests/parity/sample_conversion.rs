// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Independent row adapters for the unchanged modular frame sample functions.

use super::*;
use jxl::jxl::dec_modular::{
    int_to_float, multiply_sum, rgb_from_single, single_from_single, single_from_single_accurate,
};
use jxl::jxl::enc_modular::float_to_int;

pub(super) fn handles(kind: &str) -> bool {
    kind.starts_with("sc-")
}
fn words(args: &mut std::str::SplitWhitespace<'_>, n: usize) -> Vec<u32> {
    (0..n).map(|_| number(args)).collect()
}
fn output(out: &mut String, values: impl IntoIterator<Item = u32>) {
    for value in values {
        write!(out, " {value}").unwrap();
    }
}
pub(super) fn observe(kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    let mut out;
    match kind {
        "sc-f2i" => {
            let bits = number(args);
            let exp_bits = number(args);
            let fp = number::<u32>(args) != 0;
            let factor = f64::from_bits(number(args));
            let xsize = number(args);
            let n = number(args);
            let input: Vec<_> = words(args, n).into_iter().map(f32::from_bits).collect();
            let mut result: Vec<_> = words(args, n).into_iter().map(|v| v as i32).collect();
            let status = float_to_int(&input, &mut result, xsize, bits, exp_bits, fp, factor);
            out = status_code(&status).to_string();
            output(&mut out, result.into_iter().map(|v| v as u32));
        }
        "sc-i2f" => {
            let bits = number(args);
            let exp_bits = number(args);
            let xsize = number(args);
            let n = number(args);
            let input: Vec<_> = words(args, n).into_iter().map(|v| v as i32).collect();
            let mut result: Vec<_> = words(args, n).into_iter().map(f32::from_bits).collect();
            let status = int_to_float(&input, &mut result, xsize, bits, exp_bits);
            out = status_code(&status).to_string();
            output(&mut out, result.into_iter().map(f32::to_bits));
        }
        "sc-kernel" => {
            let mode: u32 = number(args);
            let double_factor = f64::from_bits(number(args));
            let factor = f32::from_bits(number(args));
            let xsize = number(args);
            let n = number(args);
            let input: Vec<_> = words(args, n).into_iter().map(|v| v as i32).collect();
            let y: Vec<_> = words(args, n).into_iter().map(|v| v as i32).collect();
            let mut r: Vec<_> = words(args, n).into_iter().map(f32::from_bits).collect();
            let mut g: Vec<_> = words(args, n).into_iter().map(f32::from_bits).collect();
            let mut b: Vec<_> = words(args, n).into_iter().map(f32::from_bits).collect();
            match mode {
                0 => multiply_sum(xsize, &input, &y, factor, &mut r),
                1 => rgb_from_single(xsize, &input, factor, &mut r, &mut g, &mut b),
                2 => single_from_single(xsize, &input, factor, &mut r),
                3 => single_from_single_accurate(xsize, &input, double_factor, &mut r),
                _ => panic!("unknown sample kernel"),
            }
            out = "K".to_owned();
            for row in [r, g, b] {
                output(&mut out, row.into_iter().map(f32::to_bits));
            }
        }
        _ => panic!("unknown sample conversion"),
    }
    out
}

fn f2i(bits: u32, exp: u32, fp: bool, factor: f64, xsize: usize, input: &[u32]) -> String {
    let mut case = format!(
        "sc-f2i {bits} {exp} {} {} {xsize} {}",
        u32::from(fp),
        factor.to_bits(),
        input.len()
    );
    output(&mut case, input.iter().copied());
    output(
        &mut case,
        (0..input.len()).map(|i| 0x9abc_def0u32.wrapping_add(i as u32)),
    );
    case
}
fn i2f(bits: i32, exp: i32, xsize: usize, input: &[u32]) -> String {
    let mut case = format!("sc-i2f {bits} {exp} {xsize} {}", input.len());
    output(&mut case, input.iter().copied());
    output(
        &mut case,
        (0..input.len()).map(|i| 0x7fa1_2300u32 + i as u32),
    );
    case
}

#[test]
fn sample_integer_quantization_preserves_width_rounding_and_untouched_suffixes() {
    let mut cases = Vec::new();
    let mut seed = 0xa6b8_c90d_32f7_e811;
    for bits in [0, 1, 8, 16, 22, 23, 24, 25, 30, 31, 32, 33] {
        for factor in [0.0, -0.0, 1.0, -1.0, 255.0, 65535.0, 4194303.0] {
            for n in 0..=20 {
                let mut input: Vec<_> = (0..n)
                    .map(|_| ((next(&mut seed) as i32 as f64 / 2147483648.0) as f32).to_bits())
                    .collect();
                input.extend([0.0f32, -0.0, 0.5, -0.5, 1.0, -1.0].map(f32::to_bits));
                cases.push(f2i(bits, 0, false, factor, n, &input));
            }
        }
        if bits > 22 && bits <= 32 {
            let factor = ((1u64 << (bits - 1)) - 1) as f64;
            for n in 1..=64 {
                let input: Vec<_> = (0..n)
                    .map(|_| ((next(&mut seed) as i32 as f64 / 2147483648.0) as f32).to_bits())
                    .collect();
                cases.push(f2i(bits, 0, false, factor, n, &input));
            }
        }
    }
    // Exact half-integer neighbors stress the source's multiply/add contraction.
    for bits in [22, 23, 31] {
        for factor in [3.0f64, 127.0, 255.0, 65535.0, 4194303.0] {
            for value in [-10000.5f64, -1.5, -0.5, 0.5, 1.5, 10000.5] {
                let center = ((value / factor) as f32).to_bits();
                let input = [center - 1, center, center + 1];
                cases.push(f2i(bits, 0, false, factor, 3, &input));
            }
        }
    }
    compare(cases);
}

#[test]
fn sample_custom_float_fields_preserve_all_half_patterns_and_raw_sign_bits() {
    let mut cases = Vec::new();
    // Exhaustive binary16 bit patterns, including every subnormal and NaN payload.
    for base in (0..65536u32).step_by(64) {
        let input: Vec<_> = (base..base + 64).collect();
        cases.push(i2f(16, 5, 64, &input));
    }
    let mut seed = 0x3908_7acf_641d_5581;
    for exp in 1..=8 {
        for mant in 0..=23 {
            let bits = 1 + exp + mant;
            if bits >= 32 {
                continue;
            }
            let sign = 1u32 << (bits - 1);
            let max_exp = ((1u32 << exp) - 1) << mant;
            let mut input = vec![0, sign, 1, sign | 1, max_exp, sign | max_exp, u32::MAX];
            input.extend((0..41).map(|_| next(&mut seed) as u32));
            for n in [0, 1, 3, input.len()] {
                cases.push(i2f(bits, exp, n, &input));
            }
        }
    }
    for exp in [0, 1, 7, 8, 9, 31] {
        let input: Vec<_> = (0..64).map(|_| next(&mut seed) as u32).collect();
        for n in [0, 1, 64] {
            cases.push(i2f(32, exp, n, &input));
        }
        if exp != 8 {
            cases.push(i2f(32, exp, usize::MAX, &[]));
        }
    }
    compare(cases);
}

#[test]
fn sample_all_half_patterns_cross_original_decode_and_rust_encode() {
    let requests: Vec<_> = (0..65536u32)
        .step_by(64)
        .map(|base| i2f(16, 5, 64, &(base..base + 64).collect::<Vec<_>>()))
        .collect();
    let decoded = original_observations(&requests);
    let mut cases = Vec::new();
    for (request, cpp) in requests.iter().zip(decoded) {
        super::assert_parity(request, &super::observe(request), &cpp);
        let mut args = cpp.split_whitespace();
        assert_eq!(number::<i32>(&mut args), 0);
        let input = words(&mut args, 64);
        cases.push(f2i(16, 5, true, 1.0, 64, &input));
    }
    println!("{} C++/Rust differential cases matched", requests.len());
    compare(cases);
}

#[test]
fn sample_custom_float_encoding_preserves_failure_prefixes_and_special_values() {
    let mut cases = Vec::new();
    let mut seed = 0xe4b9_6c0f_8d3a_5721;
    let special = [
        0,
        1 << 31,
        0x7f80_0000,
        0xff80_0000,
        0x7fc1_2345,
        0xff81_2345,
    ];
    for exp in 1..=8 {
        for mant in 0..=23 {
            let bits = 1 + exp + mant;
            if bits >= 32 {
                continue;
            }
            let input: Vec<_> = special
                .into_iter()
                .chain((0..24).map(|_| next(&mut seed) as u32))
                .collect();
            cases.push(f2i(bits, exp, true, 1.0, 0, &input));
            cases.push(f2i(bits, exp, true, 1.0, input.len(), &input));
            // Each failure is preceded by valid zero/NaN/Inf samples so source
            // early returns can be compared against an observable output prefix.
            for invalid in [1, 0x0080_0000, 0x3f80_0001, 0x7f7f_ffff] {
                let input: Vec<_> = special.into_iter().chain([invalid, 0x3f80_0000]).collect();
                cases.push(f2i(bits, exp, true, 1.0, input.len(), &input));
            }
            // Ordinary exactly representable powers include exponent boundaries.
            for exponent in [1u32, 63, 112, 113, 126, 127, 128, 142, 143, 190, 254] {
                let input = [0, exponent << 23, 0x8000_0000 | exponent << 23, 0x3f80_0000];
                cases.push(f2i(bits, exp, true, 1.0, input.len(), &input));
            }
        }
    }
    for bits in [0, 32, 33, u32::MAX] {
        for exp in [0, 7, 8, 9] {
            let input: Vec<_> = special
                .into_iter()
                .chain((0..40).map(|_| next(&mut seed) as u32))
                .collect();
            cases.push(f2i(bits, exp, true, 1.0, input.len(), &input));
            if bits != 32 || exp != 8 {
                cases.push(f2i(bits, exp, true, 1.0, usize::MAX, &[]));
            }
        }
    }
    compare(cases);
}

#[test]
fn sample_scalar_decoder_kernels_preserve_wraps_precision_and_three_outputs() {
    let mut cases = Vec::new();
    let mut seed = 0x8cb1_a703_d69e_5f24;
    for mode in 0..4 {
        for factor in [
            0.0f64,
            -0.0,
            1.0,
            -1.0,
            1.0 / 255.0,
            1.0 / 65535.0,
            1.0 / 2147483647.0,
            f64::MIN_POSITIVE,
            1e300,
            f64::INFINITY,
            f64::NEG_INFINITY,
            f64::NAN,
        ] {
            for n in 0..=33 {
                let len = n + 4;
                let mut case = format!(
                    "sc-kernel {mode} {} {} {n} {len}",
                    factor.to_bits(),
                    (factor as f32).to_bits()
                );
                let mut input = vec![0, 1, i32::MAX as u32, i32::MIN as u32];
                input.extend((4..len).map(|_| next(&mut seed) as u32));
                output(&mut case, input);
                output(&mut case, (0..len).map(|_| next(&mut seed) as u32));
                for c in 0..3 {
                    output(
                        &mut case,
                        (0..len).map(|i| 0x7fa0_0000 + c * 1024 + i as u32),
                    );
                }
                cases.push(case);
            }
        }
    }
    compare(cases);
}
