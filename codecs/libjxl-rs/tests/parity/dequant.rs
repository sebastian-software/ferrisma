// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use super::*;
use jxl::jxl::ac_strategy::{AcStrategy, AcStrategyType};
use jxl::jxl::dct_util::ConstAcPtr;
use jxl::jxl::dec_group_render::{dequant_block, dequant_lane};
const DEFAULT_QUANT_BIAS: [f32; 4] = [
    1.0 - f32::from_bits(0x3d5fd8c1),
    1.0 - f32::from_bits(0x3d8f78bc),
    1.0 - f32::from_bits(0x3d4c88c0),
    0.145,
];
use jxl::jxl::quant_weights::DequantMatrices;
use jxl::jxl::quantizer::Quantizer;
use jxl::jxl::quantizer_inl::adjust_quant_bias;

pub(super) fn handles(kind: &str) -> bool {
    kind.starts_with("dq-")
}
fn float(args: &mut std::str::SplitWhitespace<'_>) -> f32 {
    f32::from_bits(number(args))
}
pub(super) fn observe(kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    if kind == "dq-bias" {
        let c = number(args);
        let n = number(args);
        let biases = std::array::from_fn(|_| float(args));
        let mut out = String::from("B");
        for _ in 0..n {
            write!(
                out,
                " {}",
                adjust_quant_bias(c, number(args), &biases).to_bits()
            )
            .unwrap();
        }
        return out;
    }
    let raw = number(args);
    let ty: u32 = number(args);
    let size: usize = number(args);
    let inv = float(args);
    let quant = number(args);
    let xdm = float(args);
    let bdm = float(args);
    let xcc = float(args);
    let bcc = float(args);
    let biases = std::array::from_fn(|_| float(args));
    let matrix: [Vec<f32>; 3] = std::array::from_fn(|_| {
        if kind == "dq-lane" {
            (0..size).map(|_| float(args)).collect()
        } else {
            Vec::new()
        }
    });
    let q32: [Vec<i32>; 3] = std::array::from_fn(|_| (0..size).map(|_| number(args)).collect());
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
    if kind == "dq-lane" {
        let scale = inv / quant as f32;
        let scaled = [scale * xdm, scale, scale * bdm];
        for k in 0..size {
            dequant_lane(
                scaled,
                matrix.each_ref().map(|v| &v[..]),
                size,
                k,
                xcc,
                bcc,
                &biases,
                &blocks,
                &mut block,
            );
        }
    } else {
        let acs = AcStrategy::from_raw_strategy(AcStrategyType(raw));
        let stride: usize = number(args);
        let sbx = std::array::from_fn(|_| number(args));
        let dc: [Vec<f32>; 3] = std::array::from_fn(|_| {
            (0..acs.covered_blocks_y() * stride + 4)
                .map(|_| float(args))
                .collect()
        });
        let mut matrices = DequantMatrices::default();
        matrices.ensure_computed(1 << raw).unwrap();
        let quantizer = Quantizer::new(&matrices);
        dequant_block(
            inv,
            quant,
            xdm,
            bdm,
            xcc,
            bcc,
            acs.strategy(),
            size,
            &quantizer,
            &matrices,
            acs.covered_blocks_x() * acs.covered_blocks_y(),
            sbx,
            dc.each_ref().map(|v| &v[..]),
            stride,
            &biases,
            &blocks,
            &mut block,
            &mut scratch,
        );
    }
    let mut out = String::from("O");
    for values in [block, scratch] {
        out.push_str(" B");
        for v in values {
            write!(out, " {}", v.to_bits()).unwrap();
        }
    }
    out
}
fn request(raw: u32, ty: u32, lane: bool, mode: usize) -> String {
    let acs = AcStrategy::from_raw_strategy(AcStrategyType(raw));
    let size = if lane {
        64
    } else {
        acs.covered_blocks_x() * acs.covered_blocks_y() * 64
    };
    let mut out = format!(
        "{} {raw} {ty} {size}",
        if lane { "dq-lane" } else { "dq-block" }
    );
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
    if lane {
        for c in 0..3 {
            for k in 0..size {
                let value = if mode >= 4 && k % 7 == 0 {
                    special[(c + k + mode) % special.len()]
                } else {
                    ((k + c * 3) as f32 / 16.0 + 0.125).to_bits()
                };
                write!(out, " {value}").unwrap();
            }
        }
    }
    for c in 0..3 {
        for k in 0..size {
            let v = match mode % 4 {
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
            };
            write!(out, " {v}").unwrap();
        }
    }
    if !lane {
        let stride = acs.covered_blocks_x() + 3;
        write!(out, " {stride} 1 2 0").unwrap();
        for c in 0..3 {
            for k in 0..acs.covered_blocks_y() * stride + 4 {
                let v = if mode >= 4 && k % 11 == 0 {
                    f32::from_bits(special[(k + c) % special.len()])
                } else {
                    ((k + c * 3) % 37) as f32 / 32.0 - 0.5
                };
                write!(out, " {}", v.to_bits()).unwrap();
            }
        }
    }
    out
}
#[test]
fn quant_bias_matches_integer_conversion_sign_bits_and_channel_biases() {
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
    compare(requests);
}
#[test]
fn dequant_lanes_match_both_coefficient_widths_and_exceptional_matrix_words() {
    let requests = (0..2)
        .flat_map(|ty| (0..17).map(move |mode| request(0, ty, true, mode)))
        .collect();
    compare(requests);
}
#[test]
fn dequant_blocks_preserve_all_strategy_llf_replacement_and_scratch_mutation() {
    let mut requests = Vec::new();
    for raw in 0..27 {
        for ty in 0..2 {
            for mode in 0..4 {
                requests.push(request(raw, ty, false, mode));
            }
        }
    }
    for chunk in requests.chunks(4) {
        compare(chunk.to_vec());
    }
}
