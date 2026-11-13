// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! SIMD parity of the modular and output-conversion kernels (port ADR 0004,
//! Plan item 2): the port's NEON/AVX2 instantiations against the original
//! built for the same single Highway target (`JXL_CPP_ORACLE_SIMD`). Every
//! observed word is compared bit for bit, except that any NaN equals any NaN
//! in float words (decision D8); integer words must be identical.
//!
//! - dec_external_image: `FloatToU32`/`FloatToF16` directly (`mx-u32`,
//!   `mx-f16` of tools/oracle/modular_simd.cc, 128-byte aligned buffers,
//!   every word the original writes, including the whole-vector tail), and
//!   through `ConvertChannelsToExternal` with the decoder's dispatch
//!   selecting the SIMD target (the scalar suite's `ue-channels`,
//!   `fe-channels` and `fe-row` requests, whose adapters call the original's
//!   public function and so its `HWY_DYNAMIC_DISPATCH`).
//! - dec_modular: `MultiplySum`, `RgbFromSingle`, `SingleFromSingle`
//!   directly (`mx-msum`, `mx-rgb`, `mx-single`), every written word.
//! - rct and squeeze: `InvRCT` and `InvSqueeze` through the decoder's
//!   dispatch on whole modular images (`mx-rct`, `mx-squeeze`).
#![cfg(feature = "cpp-oracle-simd")]

mod simd_common;

use jxl::jxl::dec_external_image::{
    convert_float_channels_to_buffer, convert_uint_channels_to_buffer, float_to_f16, float_to_u32,
};
use jxl::jxl::dec_external_image_simd::{float_to_f16_simd, float_to_u32_simd};
use jxl::jxl::dec_modular::{multiply_sum, rgb_from_single, single_from_single};
use jxl::jxl::dec_modular_simd::{
    multiply_sum_simd, rgb_from_single_simd, single_from_single_simd,
};
use jxl::jxl::hwy::set_decoder_simd_for_test;
use jxl::jxl::image::Plane;
use jxl::jxl::image_metadata::Orientation;
use jxl::jxl::modular::modular_image::Image as ModularImage;
use jxl::jxl::modular::transform::rct::inv_rct;
use jxl::jxl::modular::transform::squeeze::{
    default_squeeze_parameters, inv_squeeze, inv_squeeze_for_target, meta_squeeze,
};
use jxl::jxl::modular::transform::squeeze_params::SqueezeParams;
use jxl::jxl::types::JxlEndianness;
use simd_common::{check_rows, join, next, special_words, status, target};
use std::fmt::Write as _;

fn number<'a, T: std::str::FromStr>(args: &mut impl Iterator<Item = &'a str>) -> T
where
    T::Err: std::fmt::Debug,
{
    args.next().unwrap().parse().unwrap()
}

fn hex(bytes: &[u8]) -> String {
    if bytes.is_empty() {
        return "-".to_owned();
    }
    let mut result = String::new();
    for byte in bytes {
        write!(result, "{byte:02x}").unwrap();
    }
    result
}

/// tools/oracle/render_pipeline.inc RPValue (tests/parity/render_pipeline.rs).
fn rp_value(c: usize, x: usize, y: usize, pass: usize, mode: u32) -> f32 {
    if mode == 0 {
        return 0.0;
    }
    if mode == 3 {
        let words = [
            0u32, 0x80000000, 0x7fa12345, 0x7fc98765, 0x7f800000, 0xff800000, 1, 0x3f800000,
        ];
        return f32::from_bits(words[(c + x + y + pass) % 8]);
    }
    (((c * 31 + x * 17 + y * 9 + pass * 11) % 257) as i32 - 128) as f32 / 16.0
}

/// Whole vectors of every target in scope (8 lanes on AVX2) past `num`,
/// plus one more vector of defined words after them.
fn padded_len(num: usize) -> usize {
    num.div_ceil(8) * 8 + 8
}

/// Input rows for the conversion kernels: the special words, each output
/// scale's rounding ties with their neighbors, and every length 0-67 (full
/// vectors and remainders on both targets).
fn conversion_rows(bits: Option<u32>, seed: &mut u64) -> Vec<(usize, Vec<u32>)> {
    let specials = special_words();
    let mut values: Vec<u32> = specials.clone();
    values.extend([
        0xbf80_0000,
        0x3f80_1000,
        0x3f7f_ffff,
        0x3380_0000,
        0x3380_0001,
        0x477f_e000,
        0x477f_f000,
        0x477f_f001,
    ]);
    if let Some(bits) = bits {
        let scale = ((1u64 << bits) - 1) as f32;
        for n in (0..256).step_by(if bits > 8 { 7 } else { 1 }) {
            let x = (n as f32 + 0.5) / scale;
            values.extend([x.to_bits() - 1, x.to_bits(), x.to_bits() + 1]);
        }
    }
    let mut rows = Vec::new();
    for chunk in values.chunks(97) {
        let num = chunk.len();
        let mut words = chunk.to_vec();
        while words.len() < padded_len(num) {
            words.push(specials[(next(seed) % specials.len() as u64) as usize]);
        }
        rows.push((num, words));
    }
    for num in 0..=67usize {
        let words = (0..padded_len(num))
            .map(|i| values[(i * 7 + num * 3) % values.len()])
            .collect();
        rows.push((num, words));
    }
    rows
}

#[test]
fn float_to_u32_matches_the_original_target_on_every_written_word() {
    let t = target();
    let mut seed = 0x51_ed_27_u64;
    let mut requests = Vec::new();
    for bits in 1..=16u32 {
        for (num, words) in conversion_rows(Some(bits), &mut seed) {
            requests.push(format!(
                "mx-u32 {bits} {num} {} {}",
                words.len(),
                join(words.iter().copied())
            ));
        }
    }
    let scalar_differs = std::cell::Cell::new(0usize);
    check_rows(
        "FloatToU32",
        &requests,
        |request| {
            let mut args = request.split_whitespace().skip(1);
            let bits: u32 = number(&mut args);
            let num: usize = number(&mut args);
            let m: usize = number(&mut args);
            let input: Vec<f32> = (0..m).map(|_| f32::from_bits(number(&mut args))).collect();
            let mul = ((1u64 << bits) - 1) as f32;
            let mut out = vec![0xdead_beef_u32; m];
            float_to_u32_simd(t, &input, &mut out, num, mul, bits as usize);
            let mut scalar = vec![0xdead_beef_u32; m];
            float_to_u32(&input, &mut scalar, num, mul, bits as usize);
            if scalar[..num] != out[..num] {
                scalar_differs.set(scalar_differs.get() + 1);
            }
            (join(out), Box::new(|_| true) as Box<dyn Fn(usize) -> bool>)
        },
        |_| false,
    );
    // The scalar Clamp keeps 0 for a signaling NaN, NEON's fmaxnm/fminnm
    // give the upper bound, and AVX2's maxps/minps give the upper bound for
    // every NaN: the requests tell the SIMD path from the scalar one.
    println!(
        "FloatToU32: the scalar path differs below num on {} requests",
        scalar_differs.get()
    );
    assert!(scalar_differs.get() > 0, "the requests must discriminate");
}

#[test]
fn float_to_f16_matches_the_original_target_on_every_written_word() {
    let t = target();
    let mut seed = 0xf16_u64;
    let mut requests = Vec::new();
    for (num, words) in conversion_rows(None, &mut seed) {
        requests.push(format!(
            "mx-f16 {num} {} {}",
            words.len(),
            join(words.iter().copied())
        ));
    }
    // Binary16 rounding, subnormal and overflow boundaries.
    let mut words = Vec::new();
    for base in [
        0x3380_0000u32,
        0x3880_0000,
        0x477f_e000,
        0x4780_0000,
        0x3f80_0000,
    ] {
        for delta in 0..64 {
            words.push(base + delta * 0x0000_1000 - 0x0002_0000);
            words.push(base + delta);
        }
    }
    for chunk in words.chunks(61) {
        let mut row = chunk.to_vec();
        row.resize(padded_len(chunk.len()), 0x3f80_0000);
        requests.push(format!(
            "mx-f16 {} {} {}",
            chunk.len(),
            row.len(),
            join(row.iter().copied())
        ));
    }
    let scalar_differs = std::cell::Cell::new(0usize);
    check_rows(
        "FloatToF16",
        &requests,
        |request| {
            let mut args = request.split_whitespace().skip(1);
            let num: usize = number(&mut args);
            let m: usize = number(&mut args);
            let input: Vec<f32> = (0..m).map(|_| f32::from_bits(number(&mut args))).collect();
            let mut out = vec![0xdead_u16; m];
            float_to_f16_simd(t, &input, &mut out, num);
            let mut scalar = vec![0xdead_u16; m];
            float_to_f16(&input, &mut scalar, num);
            if scalar != out {
                scalar_differs.set(scalar_differs.get() + 1);
            }
            (
                join(out.iter().map(|&h| u32::from(h))),
                Box::new(|_| true) as Box<dyn Fn(usize) -> bool>,
            )
        },
        |_| false,
    );
    // The scalar reference also converts with the native binary16 cast, so
    // only the whole-vector tail (written by the SIMD loop, not by the
    // one-lane scalar loop) tells the paths apart here.
    println!(
        "FloatToF16: the scalar path differs on {} requests",
        scalar_differs.get()
    );
    assert!(scalar_differs.get() > 0, "the requests must discriminate");
}

/// The port's observation of a `ue-channels`/`fe-channels`/`fe-row` request
/// (tests/parity/uint_output.rs, float_output.rs).
fn observe_channels(request: &str) -> String {
    let mut args = request.split_whitespace();
    let kind = args.next().unwrap();
    let mut output = vec![0xa7u8; 1024];
    if kind == "fe-row" {
        let n: usize = number(&mut args);
        let bits = number(&mut args);
        let endian = JxlEndianness(number(&mut args));
        let w = number(&mut args);
        let stride = number(&mut args);
        let size = number(&mut args);
        let mut planes = Vec::new();
        for _ in 0..n {
            let mut plane = Plane::create(w, 1).unwrap();
            for x in 0..w {
                plane.row_mut(0)[x] = f32::from_bits(number(&mut args));
            }
            planes.push(plane);
        }
        let channels: Vec<_> = planes.iter().map(Some).collect();
        let s = convert_float_channels_to_buffer(
            &channels,
            bits,
            endian,
            stride,
            &mut output,
            size,
            Orientation::IDENTITY,
        );
        return format!(" S {} {}", status(&s), hex(&output));
    }
    let w = number(&mut args);
    let h = number(&mut args);
    let n: usize = number(&mut args);
    let missing: u32 = number(&mut args);
    let bits = number(&mut args);
    let orientation = Orientation(number(&mut args));
    let endian = JxlEndianness(number(&mut args));
    let stride = number(&mut args);
    let out_size = number(&mut args);
    let mode = number(&mut args);
    let planes: Vec<_> = (0..n)
        .map(|c| {
            let mut p = Plane::create(w, h).unwrap();
            for y in 0..h {
                for x in 0..w {
                    p.row_mut(y)[x] = rp_value(c, x, y, 0, mode);
                }
            }
            p
        })
        .collect();
    let channels: Vec<_> = planes
        .iter()
        .enumerate()
        .map(|(c, p)| (missing & (1 << c) == 0).then_some(p))
        .collect();
    let s = if kind == "ue-channels" {
        convert_uint_channels_to_buffer(
            &channels,
            bits,
            endian,
            stride,
            &mut output,
            out_size,
            orientation,
        )
    } else {
        convert_float_channels_to_buffer(
            &channels,
            bits,
            endian,
            stride,
            &mut output,
            out_size,
            orientation,
        )
    };
    format!(" S {} {}", status(&s), hex(&output))
}

#[test]
fn channel_conversion_through_the_decoder_dispatch_matches_the_original_target() {
    set_decoder_simd_for_test(Some(true));
    let mut requests = Vec::new();
    for kind in ["ue-channels", "fe-channels"] {
        let bit_depths: &[usize] = if kind == "ue-channels" {
            &[1, 7, 8, 12, 16]
        } else {
            &[16, 32]
        };
        for w in [1, 3, 4, 5, 7, 8, 9, 12, 15, 16, 17, 31, 33] {
            for h in [1, 3] {
                for n in 1..=4usize {
                    for &bits in bit_depths {
                        let bytes = bits.div_ceil(8);
                        let stride = n * bytes * w + 5;
                        if (h - 1) * stride + n * bytes * w > 1024 {
                            continue;
                        }
                        for orientation in [1, 6] {
                            for endian in [0, 2] {
                                for missing in [0, 2] {
                                    for mode in [1, 3] {
                                        requests.push(format!(
                                            "{kind} {w} {h} {n} {missing} {bits} {orientation} {endian} {stride} 1024 {mode}"
                                        ));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    let words = special_words();
    for w in [1usize, 4, 5, 8, 9, 16, 17, 31, 33] {
        for n in 1..=4usize {
            for endian in [0, 1, 2] {
                let stride = n * 2 * w;
                let mut request = format!("fe-row {n} 16 {endian} {w} {stride} {stride}");
                for c in 0..n {
                    for x in 0..w {
                        write!(request, " {}", words[(c * 11 + x * 5 + w) % words.len()]).unwrap();
                    }
                }
                requests.push(request);
            }
        }
    }
    check_rows(
        "ConvertChannelsToExternal",
        &requests,
        |request| {
            (
                observe_channels(request),
                Box::new(|_| true) as Box<dyn Fn(usize) -> bool>,
            )
        },
        |_| false,
    );
}

/// Signed sample words: zero, signs, the 32-bit limits and their
/// neighbors (wrapping `Add` in MultiplySum), exact and inexact binary32
/// conversions, and random words.
fn pixel_words(seed: &mut u64) -> Vec<u32> {
    let mut words = vec![
        0,
        1,
        2,
        3,
        0xffff_ffff,
        0xffff_fffe,
        0x7fff_ffff,
        0x7fff_fffe,
        0x8000_0000,
        0x8000_0001,
        0x00ff_ffff,
        0x0100_0000,
        0x0100_0001,
        0x0100_0003,
        0xff00_0000,
        0xfeff_ffff,
        0x4000_0000,
        0xc000_0000,
        255,
        256,
        65535,
        65536,
        0x007f_ffff,
        0xffff_ff01,
    ];
    for _ in 0..40 {
        words.push(next(seed) as u32);
    }
    for _ in 0..24 {
        words.push((next(seed) % 2048) as u32);
    }
    words
}

#[test]
fn modular_to_float_conversions_match_the_original_target_on_every_written_word() {
    let t = target();
    let mut seed = 0x0d_ec_0d_e5_u64;
    let pixels = pixel_words(&mut seed);
    let factors = [
        (1.0f32 / 255.0).to_bits(),
        (1.0f32 / 65535.0).to_bits(),
        ((1.0f64 / f64::from((1u32 << 22) - 1)) as f32).to_bits(),
        0x3a80_8081,
        0xbf80_0000,
        0x0000_0001,
        0x7f7f_ffff,
        0x7f80_0000,
        0x7fc0_0000,
        0x7fa1_2345,
        0x8000_0000,
        0x3f80_0000,
    ];
    let mut requests = Vec::new();
    for factor in factors {
        for xsize in 0..=67usize {
            let m = padded_len(xsize);
            let row = |offset: usize| {
                join((0..m).map(|i| pixels[(i * 5 + offset + xsize) % pixels.len()]))
            };
            requests.push(format!(
                "mx-msum {xsize} {m} {factor} {} {}",
                row(0),
                row(17)
            ));
            requests.push(format!("mx-rgb {xsize} {m} {factor} {}", row(3)));
            requests.push(format!("mx-single {xsize} {m} {factor} {}", row(11)));
        }
    }
    let tail_differs = std::cell::Cell::new(0usize);
    check_rows(
        "MultiplySum/RgbFromSingle/SingleFromSingle",
        &requests,
        |request| {
            let mut args = request.split_whitespace();
            let kind = args.next().unwrap();
            let xsize: usize = number(&mut args);
            let m: usize = number(&mut args);
            let factor = f32::from_bits(number(&mut args));
            let inputs = if kind == "mx-msum" { 2 } else { 1 };
            let rows: Vec<Vec<i32>> = (0..inputs)
                .map(|_| (0..m).map(|_| number::<u32>(&mut args) as i32).collect())
                .collect();
            let outputs = if kind == "mx-rgb" { 3 } else { 1 };
            let sentinel = f32::from_bits(0xdead_beef);
            let mut out = vec![vec![sentinel; m]; outputs];
            let mut scalar = out.clone();
            match kind {
                "mx-msum" => {
                    multiply_sum_simd(t, xsize, &rows[0], &rows[1], factor, &mut out[0]);
                    multiply_sum(xsize, &rows[0], &rows[1], factor, &mut scalar[0]);
                }
                "mx-rgb" => {
                    let [r, g, b] = &mut out[..] else {
                        unreachable!()
                    };
                    rgb_from_single_simd(t, xsize, &rows[0], factor, r, g, b);
                    let [r, g, b] = &mut scalar[..] else {
                        unreachable!()
                    };
                    rgb_from_single(xsize, &rows[0], factor, r, g, b);
                }
                _ => {
                    single_from_single_simd(t, xsize, &rows[0], factor, &mut out[0]);
                    single_from_single(xsize, &rows[0], factor, &mut scalar[0]);
                }
            }
            let bits = |rows: &[Vec<f32>]| -> Vec<u32> {
                rows.iter().flatten().map(|v| v.to_bits()).collect()
            };
            if bits(&out) != bits(&scalar) {
                tail_differs.set(tail_differs.get() + 1);
            }
            (
                join(bits(&out)),
                Box::new(|_| true) as Box<dyn Fn(usize) -> bool>,
            )
        },
        |_| true,
    );
    // Below xsize both paths convert exactly and multiply once; the scalar
    // one-lane loop does not write the whole-vector tail, which the
    // original's SIMD build writes, so the tail tells the paths apart.
    println!(
        "modular-to-float: the scalar path differs on {} requests",
        tail_differs.get()
    );
    assert!(tail_differs.get() > 0, "the requests must discriminate");
}

/// The port's observation of an `mx-rct`/`mx-squeeze` request
/// (tools/oracle/modular_simd.cc) through the decoder's dispatch.
fn observe_transform(request: &str, scalar: bool) -> String {
    let mut args = request.split_whitespace();
    let command = args.next().unwrap();
    let rct = command == "mx-rct";
    let kind: usize = number(&mut args);
    let w: usize = number(&mut args);
    let h: usize = number(&mut args);
    let n: usize = number(&mut args);
    let words: Vec<i32> = (0..n).map(|_| number::<u32>(&mut args) as i32).collect();
    let mut image = ModularImage::create(w, h, 8, if rct { 3 } else { 1 }).unwrap();
    let mut params = Vec::new();
    let mut s = Ok(());
    if !rct {
        if kind == 0 {
            default_squeeze_parameters(&mut params, &image);
        } else {
            for horizontal in [true, false] {
                if (kind == 1 && !horizontal) || (kind == 2 && horizontal) {
                    continue;
                }
                params.push(SqueezeParams {
                    horizontal,
                    in_place: true,
                    begin_c: 0,
                    num_c: 1,
                });
            }
        }
        s = meta_squeeze(&mut image, &mut params);
    }
    if s.is_ok() {
        for (c, ch) in image.channel.iter_mut().enumerate() {
            for y in 0..ch.h {
                let width = ch.w;
                let row = ch.row_mut(y);
                for (x, v) in row.iter_mut().enumerate().take(width) {
                    *v = words[(c * 7 + y * 13 + x) % n];
                }
            }
        }
        s = if rct {
            inv_rct(&mut image, 0, kind, None)
        } else if scalar {
            inv_squeeze_for_target(None, &mut image, &params, None)
        } else {
            inv_squeeze(&mut image, &params, None)
        };
    }
    let mut out = format!("{} {}", status(&s), image.channel.len());
    for ch in &image.channel {
        write!(out, " {} {}", ch.w, ch.h).unwrap();
        for y in 0..ch.h {
            for &v in &ch.row(y)[..ch.w] {
                write!(out, " {}", v as u32).unwrap();
            }
        }
    }
    out
}

/// Sample words for the transforms: small natural values (every
/// SmoothTendency branch), the 32-bit limits and their neighbors (wrapping
/// vector arithmetic, Abs(i32::MIN), the 64-bit scalar tails) and random
/// words.
fn transform_words(seed: &mut u64, natural: bool) -> Vec<u32> {
    let mut words: Vec<u32> = if natural {
        (0..61).map(|_| (next(seed) % 512) as u32).collect()
    } else {
        pixel_words(seed)
    };
    if natural {
        words.extend([0, 255, 0, 255, 128, 3, 1, 0xffff_ffff, 0xffff_fff0, 7, 9]);
    }
    words
}

#[test]
fn inverse_rct_matches_the_original_target() {
    set_decoder_simd_for_test(Some(true));
    let mut seed = 0x0_7c7_u64;
    let mut requests = Vec::new();
    for rct_type in 0..42usize {
        for (w, h) in [
            (1, 1),
            (3, 2),
            (4, 1),
            (7, 3),
            (8, 2),
            (9, 1),
            (16, 2),
            (17, 3),
            (33, 2),
            (67, 1),
        ] {
            for natural in [true, false] {
                let words = transform_words(&mut seed, natural);
                requests.push(format!(
                    "mx-rct {rct_type} {w} {h} {} {}",
                    words.len(),
                    join(words.iter().copied())
                ));
            }
        }
    }
    check_rows(
        "InvRCT",
        &requests,
        |request| {
            (
                observe_transform(request, false),
                Box::new(|_| true) as Box<dyn Fn(usize) -> bool>,
            )
        },
        |_| false,
    );
}
#[test]
fn inverse_squeeze_matches_the_original_target() {
    set_decoder_simd_for_test(Some(true));
    let mut seed = 0x5_9ee2e_u64;
    let mut requests = Vec::new();
    // Horizontal SIMD blocks need a residual wider than 16 and eight rows;
    // vertical ones eight columns per 64-column slice.
    let sizes = [
        (1, 1),
        (2, 2),
        (5, 3),
        (8, 8),
        (9, 9),
        (16, 8),
        (17, 16),
        (33, 8),
        (34, 8),
        (35, 9),
        (36, 17),
        (51, 16),
        (64, 24),
        (67, 8),
        (70, 13),
        (130, 9),
        (131, 32),
    ];
    for mode in 0..4usize {
        for (w, h) in sizes {
            for (wt, ht) in [(w, h), (h, w)] {
                for natural in [true, false] {
                    let words = transform_words(&mut seed, natural);
                    requests.push(format!(
                        "mx-squeeze {mode} {wt} {ht} {} {}",
                        words.len(),
                        join(words.iter().copied())
                    ));
                }
            }
        }
    }
    let scalar_differs = std::cell::Cell::new(0usize);
    check_rows(
        "InvSqueeze",
        &requests,
        |request| {
            let simd = observe_transform(request, false);
            let scalar = observe_transform(request, true);
            if simd != scalar {
                scalar_differs.set(scalar_differs.get() + 1);
            }
            (simd, Box::new(|_| true) as Box<dyn Fn(usize) -> bool>)
        },
        |_| false,
    );
    // FastUnsqueeze wraps at 32 bits where the scalar tails compute in
    // 64 bits, so extreme words tell the paths apart.
    println!(
        "InvSqueeze: the scalar path differs on {} requests",
        scalar_differs.get()
    );
    assert!(scalar_differs.get() > 0, "the requests must discriminate");
}
