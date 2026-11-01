// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use super::*;
use jxl::jxl::ac_strategy::{AcStrategy, AcStrategyType};
use jxl::jxl::dct::{compute_scaled_dct, compute_scaled_idct};
use jxl::jxl::dec_transforms::{lowest_frequencies_from_dc, transform_to_pixels};

pub(super) fn handles(kind: &str) -> bool {
    kind.starts_with("dct-")
}
pub(super) fn observe(kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    if kind == "dct-tables" {
        use jxl::jxl::dct_scales::{SQRT0_5, SQRT2, total_resample_scale, wc_multipliers};
        let mut values = vec![SQRT2, SQRT0_5];
        for n in [8, 16, 32, 64, 128, 256] {
            for (a, b) in [(n, n / 8), (n / 8, n)] {
                for x in 0..n / 8 {
                    values.push(total_resample_scale(a, b, x));
                }
            }
        }
        for n in [4, 8, 16, 32, 64, 128, 256] {
            values.extend_from_slice(wc_multipliers(n));
        }
        let mut out = String::from("B");
        for v in values {
            write!(out, " {}", v.to_bits()).unwrap();
        }
        return out;
    }
    if kind == "dct-strategy" {
        let acs = AcStrategy::from_raw_strategy(AcStrategyType(number(args)));
        let mut out = format!(
            "{} {} {} {} {} {} {} {}",
            acs.raw_strategy(),
            acs.strategy().0,
            u8::from(acs.is_first_block()),
            u8::from(acs.is_multiblock()),
            acs.covered_blocks_x(),
            acs.covered_blocks_y(),
            acs.log2_covered_blocks(),
            AcStrategy::type_bit(acs.strategy())
        );
        let count = acs.covered_blocks_x() * acs.covered_blocks_y() * 64;
        let mut order = vec![0x31415926; count + 3];
        let mut lut = order.clone();
        acs.compute_natural_coeff_order(&mut order);
        acs.compute_natural_coeff_order_lut(&mut lut);
        for v in order.into_iter().chain(lut) {
            write!(out, " {v}").unwrap();
        }
        return out;
    }
    let rows: usize = number(args);
    let cols: usize = number(args);
    let stride: usize = number(args);
    let option: u32 = number(args);
    let inverse = kind != "dct-scaled" || option != 0;
    let count = if kind == "dct-llf" {
        rows * stride
    } else if inverse {
        rows * cols
    } else {
        rows * stride
    };
    let mut input = vec![-123.25; count + 4];
    for v in input.iter_mut().take(count) {
        *v = f32::from_bits(number(args));
    }
    let mut output = vec![
        -123.25;
        if kind == "dct-llf" {
            rows * cols * 64 + 4
        } else {
            (if inverse { rows * stride } else { rows * cols }) + 4
        }
    ];
    let mut scratch = vec![-123.25; rows * cols * 4 + 3 * (rows + cols) + 4];
    match kind {
        "dct-scaled" if inverse => {
            compute_scaled_idct(rows, cols, &mut input, &mut output, stride, &mut scratch)
        }
        "dct-scaled" => compute_scaled_dct(rows, cols, &input, stride, &mut output, &mut scratch),
        "dct-pixels" => transform_to_pixels(
            AcStrategyType(option),
            &mut input,
            &mut output,
            stride,
            &mut scratch,
        ),
        "dct-llf" => lowest_frequencies_from_dc(
            AcStrategyType(option),
            &input,
            stride,
            &mut output,
            &mut scratch,
        ),
        _ => unreachable!(),
    }
    let mut out = String::new();
    for buf in [input, output, scratch] {
        out.push_str(" B");
        for v in buf {
            write!(out, " {}", v.to_bits()).unwrap();
        }
    }
    out.trim_start().to_owned()
}
fn request(kind: &str, rows: usize, cols: usize, pad: usize, option: u32, mode: usize) -> String {
    let stride = cols + pad;
    let count = if kind == "dct-llf" || (kind == "dct-scaled" && option == 0) {
        rows * stride
    } else {
        rows * cols
    };
    let mut values = vec![0.0f32; count];
    let mut seed = 0x27182818;
    for (i, v) in values.iter_mut().enumerate() {
        *v = match mode {
            0 => 0.0,
            1 => -0.0,
            2 => {
                if i == 0 {
                    1.0
                } else {
                    0.0
                }
            }
            3 => {
                if i == count / 3 {
                    -1.75
                } else {
                    0.0
                }
            }
            4 => (i as i32 % 23 - 11) as f32 / 16.0,
            5 => (next(&mut seed) as i32 % 16384) as f32 / 32.0,
            6 => {
                if i % 2 == 0 {
                    f32::MAX
                } else {
                    -f32::MAX
                }
            }
            _ => 0.125,
        };
    }
    let special = [
        1, 0x80000001, 0x007fffff, 0x00800000, 0x7f7fffff, 0xff7fffff, 0x7f800000, 0xff800000,
        0x7fc12345, 0xffc98765, 0x7fa12345, 0xffa98765,
    ];
    if mode >= 7 {
        values[count / 3] = f32::from_bits(special[(mode - 7) % special.len()]);
    }
    let mut s = format!("{kind} {rows} {cols} {stride} {option}");
    for v in values {
        write!(s, " {}", v.to_bits()).unwrap();
    }
    s
}
#[test]
fn dct_strategy_geometry_and_natural_orders_match_original_for_every_strategy() {
    compare((0..27).map(|i| format!("dct-strategy {i}")).collect());
}
#[test]
fn dct_scaled_forward_inverse_match_recursion_layout_strides_and_scratch() {
    let mut cases = Vec::new();
    for rows in [1, 2, 4, 8, 16, 32] {
        for cols in [1, 2, 4, 8, 16, 32] {
            for inverse in 0..2 {
                for mode in 0..19 {
                    cases.push(request("dct-scaled", rows, cols, 3, inverse, mode));
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
    ] {
        for inverse in 0..2 {
            for mode in [2, 3, 5] {
                cases.push(request("dct-scaled", rows, cols, 7, inverse, mode));
            }
        }
    }
    compare(cases);
}
#[test]
fn dct_transform_to_pixels_matches_all_strategies_and_mutable_coefficients() {
    let mut cases = Vec::new();
    for raw in 0..27 {
        let acs = AcStrategy::from_raw_strategy(AcStrategyType(raw));
        let rows = acs.covered_blocks_y() * 8;
        let cols = acs.covered_blocks_x() * 8;
        for mode in 0..if rows.max(cols) <= 32 { 19 } else { 6 } {
            cases.push(request("dct-pixels", rows, cols, 5, raw, mode));
        }
    }
    compare(cases);
}
#[test]
fn dct_lowest_frequencies_match_original_sparse_writes_and_warm_scratch_branch() {
    let mut cases = Vec::new();
    for raw in 0..27 {
        let acs = AcStrategy::from_raw_strategy(AcStrategyType(raw));
        let rows = acs.covered_blocks_y();
        let cols = acs.covered_blocks_x();
        for mode in 0..19 {
            cases.push(request("dct-llf", rows, cols, 3, raw, mode));
        }
    }
    compare(cases);
}

#[test]
fn dct_literal_scale_and_multiplier_words_match_original_float_tables() {
    compare(vec!["dct-tables".to_owned()]);
}

#[test]
fn dct_identity_keeps_selected_original_subnormal_and_signed_zero_rounding() {
    let mut cases = Vec::new();
    for sign in [0, 0x80000000] {
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
                cases.push(request);
            }
        }
    }
    compare(cases);
}

#[test]
fn dct_competing_nan_payloads_match_identity_afv_and_anisotropic_transform_paths() {
    let pairs = [
        (0x7fc12345, 0xffc98765),
        (0xffc98765, 0x7fc12345),
        (0x7fa12345, 0xffc98765),
        (0xffc98765, 0x7fa12345),
        (0x7fa12345, 0xffa98765),
        (0xffa98765, 0x7fa12345),
    ];
    let mut cases = Vec::new();
    let mut add =
        |kind: &str, rows: usize, cols: usize, option: u32, positions: &[(usize, usize)]| {
            let stride = cols + 3;
            let count = if kind == "dct-llf" || (kind == "dct-scaled" && option == 0) {
                rows * stride
            } else {
                rows * cols
            };
            for &(first, second) in positions {
                for &(a, b) in &pairs {
                    let mut values = vec![0.125f32.to_bits(); count];
                    values[first] = a;
                    values[second] = b;
                    let mut s = format!("{kind} {rows} {cols} {stride} {option}");
                    for v in values {
                        write!(s, " {v}").unwrap();
                    }
                    cases.push(s);
                }
            }
        };
    for raw in [0, 1, 3, 6, 7, 14, 15, 16, 17] {
        let acs = AcStrategy::from_raw_strategy(AcStrategyType(raw));
        add(
            "dct-pixels",
            acs.covered_blocks_y() * 8,
            acs.covered_blocks_x() * 8,
            raw,
            &[(0, 1), (0, 8), (21, 42)],
        );
    }
    for (rows, cols) in [(4, 8), (8, 4), (8, 8), (8, 16), (16, 8)] {
        for inverse in 0..2 {
            let fs = if inverse == 0 { cols + 3 } else { cols };
            add(
                "dct-scaled",
                rows,
                cols,
                inverse,
                &[(0, 1), (0, (rows - 1) * fs + cols - 1)],
            );
        }
    }
    for raw in [4, 6, 7, 10, 11] {
        let acs = AcStrategy::from_raw_strategy(AcStrategyType(raw));
        let rows = acs.covered_blocks_y();
        let cols = acs.covered_blocks_x();
        let second = if cols > 1 { 1 } else { cols + 3 };
        add(
            "dct-llf",
            rows,
            cols,
            raw,
            &[(0, second), (0, (rows - 1) * (cols + 3) + cols - 1)],
        );
    }
    compare(cases);
}
