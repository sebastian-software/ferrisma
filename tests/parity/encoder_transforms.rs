// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Compare the encoder's forward transforms, Gaborish inverse, symmetric
// convolutions and XYB conversion with the original library functions as
// exact float words, including retained input, scratch and canary words.
use super::*;
use jxl::jxl::ac_strategy::{AcStrategy, AcStrategyType};
use jxl::jxl::base::rect::Rect;
use jxl::jxl::color_encoding::{
    ColorSpace, Primaries, RenderingIntent, TransferFunction, WhitePoint,
};
use jxl::jxl::color_encoding_internal::ColorEncoding;
use jxl::jxl::convolve::{WeightsSymmetric3, WeightsSymmetric5};
use jxl::jxl::convolve_slow::slow_symmetric3;
use jxl::jxl::convolve_symmetric5::symmetric5;
use jxl::jxl::enc_gaborish::gaborish_inverse;
use jxl::jxl::enc_transforms::{afvdct4x4, dc_from_lowest_frequencies, transform_from_pixels};
use jxl::jxl::enc_xyb::{XybInputEncoding, compute_premul_absorb, linear_rgb_row_to_xyb, to_xyb};
use jxl::jxl::image::{Image3F, ImageF};
use jxl::jxl::image_ops::{fill_image3, zero_fill_image, zero_fill_image3};

const CANARY: f32 = -123.25;

pub(super) fn handles(kind: &str) -> bool {
    kind.starts_with("etx-")
}

fn float(args: &mut std::str::SplitWhitespace<'_>) -> f32 {
    f32::from_bits(number(args))
}
fn words(out: &mut String, values: &[f32]) {
    out.push_str(" B");
    for v in values {
        write!(out, " {}", v.to_bits()).unwrap();
    }
}
fn rect(args: &mut std::str::SplitWhitespace<'_>) -> Rect {
    Rect::new(number(args), number(args), number(args), number(args))
}
fn read_plane(args: &mut std::str::SplitWhitespace<'_>, plane: &mut ImageF) {
    for y in 0..plane.ysize() {
        for x in 0..plane.xsize() {
            plane.row_mut(y)[x] = float(args);
        }
    }
}
fn canary(plane: &mut ImageF, salt: u32) {
    for y in 0..plane.ysize() {
        for x in 0..plane.xsize() {
            plane.row_mut(y)[x] = f32::from_bits(
                0x7fa00000u32
                    .wrapping_add(salt.wrapping_mul(0x10000))
                    .wrapping_add((y * 256 + x) as u32),
            );
        }
    }
}
fn print_plane(out: &mut String, plane: &ImageF) {
    out.push_str(" P");
    for y in 0..plane.ysize() {
        for x in 0..plane.xsize() {
            write!(out, " {}", plane.row(y)[x].to_bits()).unwrap();
        }
    }
}
fn print_image(out: &mut String, image: &Image3F) {
    for c in 0..3 {
        print_plane(out, image.plane(c));
    }
}
fn image(xsize: usize, ysize: usize) -> Image3F {
    Image3F::create(xsize, ysize).unwrap()
}

// The enc_gaborish_test.cc GaborishKernel helper. Its only arguments are
// zero, so the possibly contracted normalization sum is exact.
fn gaborish_kernel(weight1: f32, weight2: f32) -> WeightsSymmetric3 {
    let weight0 = 1.0f32;

    // Normalize
    let mul = 1.0f32 / (weight0 + 4.0 * (weight1 + weight2));
    let w0 = weight0 * mul;
    let w1 = weight1 * mul;
    let w2 = weight2 * mul;

    WeightsSymmetric3 {
        c: [w0; 4],
        r: [w1; 4],
        d: [w2; 4],
    }
}

pub(super) fn observe(kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    let mut out = String::new();
    // The observation unit's requests carry the same payloads.
    let kind = kind.replacen("etx-unit-", "etx-", 1);
    let kind = kind.as_str();
    match kind {
        "etx-pixels" | "etx-llf" => {
            let raw: u32 = number(args);
            let stride: usize = number(args);
            let acs = AcStrategy::from_raw_strategy(AcStrategyType(raw));
            if kind == "etx-pixels" {
                let rows = acs.covered_blocks_y() * 8;
                let cols = acs.covered_blocks_x() * 8;
                let mut pixels = vec![CANARY; rows * stride + 4];
                for v in pixels.iter_mut().take(rows * stride) {
                    *v = float(args);
                }
                let mut coefficients = vec![CANARY; rows * cols + 4];
                let mut scratch = vec![CANARY; rows * cols * 4 + 3 * (rows + cols) + 4];
                transform_from_pixels(
                    AcStrategyType(raw),
                    &pixels,
                    stride,
                    &mut coefficients,
                    &mut scratch,
                );
                out.push('T');
                for buf in [&pixels, &coefficients, &scratch] {
                    words(&mut out, buf);
                }
            } else {
                let rows = acs.covered_blocks_y();
                let cols = acs.covered_blocks_x();
                let mut block = vec![CANARY; rows * cols * 64 + 4];
                for v in block.iter_mut().take(rows * cols * 64) {
                    *v = float(args);
                }
                let mut dc = vec![CANARY; rows * stride + 4];
                let mut scratch = vec![CANARY; 4 * 32 * 32 + 4];
                dc_from_lowest_frequencies(
                    AcStrategyType(raw),
                    &block,
                    &mut dc,
                    stride,
                    &mut scratch,
                );
                out.push('L');
                for buf in [&block, &dc, &scratch] {
                    words(&mut out, buf);
                }
            }
        }
        "etx-afv" => {
            let mut pixels = [CANARY; 20];
            for v in pixels.iter_mut().take(16) {
                *v = float(args);
            }
            let mut coeffs = [CANARY; 20];
            afvdct4x4(&pixels, &mut coeffs);
            out.push('A');
            words(&mut out, &pixels);
            words(&mut out, &coeffs);
        }
        "etx-s5" => {
            let xsize: usize = number(args);
            let ysize: usize = number(args);
            let in_rect = rect(args);
            let oxsize: usize = number(args);
            let oysize: usize = number(args);
            let out_rect = rect(args);
            let mut w = [[0.0f32; 4]; 6];
            for p in &mut w {
                *p = [float(args); 4];
            }
            let weights = WeightsSymmetric5 {
                c: w[0],
                r: w[1],
                R: w[2],
                d: w[3],
                D: w[4],
                L: w[5],
            };
            let mut input = ImageF::create(xsize, ysize).unwrap();
            read_plane(args, &mut input);
            let mut output = ImageF::create(oxsize, oysize).unwrap();
            canary(&mut output, 1);
            let status = symmetric5(&input, in_rect, &weights, None, &mut output, out_rect);
            write!(out, "S {}", status_code(&status)).unwrap();
            print_plane(&mut out, &input);
            print_plane(&mut out, &output);
        }
        "etx-s3" => {
            let xsize: usize = number(args);
            let ysize: usize = number(args);
            let r = rect(args);
            let weights = WeightsSymmetric3 {
                c: [float(args); 4],
                r: [float(args); 4],
                d: [float(args); 4],
            };
            let mut input = ImageF::create(xsize, ysize).unwrap();
            read_plane(args, &mut input);
            let mut output = ImageF::create(xsize, ysize).unwrap();
            canary(&mut output, 2);
            let status = slow_symmetric3(&input, r, &weights, None, &mut output);
            write!(out, "W {}", status_code(&status)).unwrap();
            print_plane(&mut out, &output);
        }
        "etx-gab" => {
            let xsize: usize = number(args);
            let ysize: usize = number(args);
            let r = rect(args);
            let mul = [float(args), float(args), float(args)];
            let mut img = image(xsize, ysize);
            for c in 0..3 {
                read_plane(args, img.plane_mut(c));
            }
            let status = gaborish_inverse(&mut img, r, &mul, None);
            write!(out, "G {}", status_code(&status)).unwrap();
            print_image(&mut out, &img);
        }
        "etx-gabtest" => {
            let which: u32 = number(args);
            let mut input = image(20, 20);
            if which == 1 {
                fill_image3(1.0f32, &mut input);
            } else {
                zero_fill_image3(&mut input);
                if which == 2 {
                    input.plane_row_mut(1, 10)[10] = 10.0;
                }
            }
            let mut fwd = image(input.xsize(), input.ysize());
            let kernel = gaborish_kernel(0.0, 0.0);
            write!(
                out,
                "K {} {} {}",
                kernel.c[0].to_bits(),
                kernel.r[0].to_bits(),
                kernel.d[0].to_bits()
            )
            .unwrap();
            for c in 0..3 {
                let plane = input.plane(c);
                let full = Rect::new(0, 0, plane.xsize(), plane.ysize());
                let s = slow_symmetric3(plane, full, &kernel, None, fwd.plane_mut(c));
                write!(out, " {}", status_code(&s)).unwrap();
            }
            print_image(&mut out, &fwd);
            let w = 0.927_189_3_f32;
            let weights = [w, w, w];
            let full = Rect::new(0, 0, fwd.xsize(), fwd.ysize());
            let status = gaborish_inverse(&mut fwd, full, &weights, None);
            write!(out, " G {}", status_code(&status)).unwrap();
            print_image(&mut out, &fwd);
        }
        "etx-premul" => {
            let intensity = float(args);
            let mut premul = [CANARY; 16];
            compute_premul_absorb(intensity, &mut premul);
            out.push('M');
            words(&mut out, &premul);
        }
        "etx-row" => {
            let xsize: usize = number(args);
            let intensity = float(args);
            let mut premul = [0.0f32; 12];
            compute_premul_absorb(intensity, &mut premul);
            let mut rows: [Vec<f32>; 3] = std::array::from_fn(|_| vec![CANARY; xsize + 4]);
            for row in &mut rows {
                for v in row.iter_mut().take(xsize) {
                    *v = float(args);
                }
            }
            let [row0, row1, row2] = &mut rows;
            linear_rgb_row_to_xyb(row0, row1, row2, &premul, xsize);
            out.push('R');
            for row in &rows {
                words(&mut out, row);
            }
        }
        "etx-xyb" => {
            let external = super::cms::color_encoding(args);
            let mut c_current = ColorEncoding::default();
            let converted = c_current.from_external(&external);
            let intensity = float(args);
            let black_mode: usize = number(args);
            let linear_mode: usize = number(args);
            let xsize: usize = number(args);
            let ysize: usize = number(args);
            let admitted = XybInputEncoding::new(&c_current);
            let (same_linear, is_srgb) = match admitted {
                Some(XybInputEncoding::LinearSrgb) => (1, 0),
                Some(XybInputEncoding::Srgb) => (0, 1),
                None => (0, 0),
            };
            write!(out, "X {} {same_linear} {is_srgb}", status_code(&converted)).unwrap();
            let mut img = image(xsize, ysize);
            for c in 0..3 {
                read_plane(args, img.plane_mut(c));
            }
            let Some(admitted) = admitted else {
                return out;
            };
            let mut black = ImageF::default();
            if black_mode != 0 {
                black = ImageF::create(xsize + usize::from(black_mode == 2), ysize).unwrap();
                zero_fill_image(&mut black);
            }
            let mut linear = Image3F::default();
            if linear_mode != 0 {
                linear = image(xsize, ysize + usize::from(linear_mode == 2));
                for c in 0..3 {
                    canary(linear.plane_mut(c), 3 + c as u32);
                }
            }
            let status = to_xyb(
                admitted,
                intensity,
                (black_mode != 0).then_some(&black),
                None,
                &mut img,
                (linear_mode != 0).then_some(&mut linear),
            );
            write!(out, " {}", status_code(&status)).unwrap();
            print_image(&mut out, &img);
            if linear_mode != 0 {
                print_image(&mut out, &linear);
            }
        }
        _ => panic!("unknown encoder transform request {kind}"),
    }
    out
}

// Exceptional words as in the existing DCT corpus: subnormals, boundaries,
// infinities, and quiet and signaling NaNs of both signs.
const SPECIAL: [u32; 12] = [
    1, 0x80000001, 0x007fffff, 0x00800000, 0x7f7fffff, 0xff7fffff, 0x7f800000, 0xff800000,
    0x7fc12345, 0xffc98765, 0x7fa12345, 0xffa98765,
];
// Mode 0..19 value generator. Modes 7..18 place one exceptional word at
// count / 3 over a constant background; 19 is pixel-like full-mantissa noise.
const MODES: usize = 20;
fn values(count: usize, mode: usize, seed: u64) -> Vec<u32> {
    let mut seed = seed | 1;
    let mut values: Vec<u32> = (0..count)
        .map(|i| {
            let v: f32 = match mode {
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
                5 => (next(&mut seed) as i32 % 16384) as f32 / 32.0,
                6 => {
                    if i % 2 == 0 {
                        f32::MAX
                    } else {
                        -f32::MAX
                    }
                }
                19 => (next(&mut seed) % (1 << 24)) as f32 / (1u32 << 24) as f32 - 0.25,
                _ => 0.125,
            };
            v.to_bits()
        })
        .collect();
    if (7..19).contains(&mode) && count != 0 {
        values[count / 3] = SPECIAL[mode - 7];
    }
    values
}
fn push_words(s: &mut String, values: &[u32]) {
    for v in values {
        write!(s, " {v}").unwrap();
    }
}
fn bits(v: f32) -> u32 {
    v.to_bits()
}

fn pixels_request(raw: u32, pad: usize, mode: usize) -> String {
    let acs = AcStrategy::from_raw_strategy(AcStrategyType(raw));
    let rows = acs.covered_blocks_y() * 8;
    let cols = acs.covered_blocks_x() * 8;
    let stride = cols + pad;
    let mut s = format!("etx-pixels {raw} {stride}");
    push_words(
        &mut s,
        &values(rows * stride, mode, 0x9e3779b97f4a7c15 ^ raw as u64),
    );
    s
}

#[test]
fn encoder_transform_from_pixels_matches_all_strategies_strides_and_scratch() {
    let mut cases = Vec::new();
    for raw in 0..27 {
        let acs = AcStrategy::from_raw_strategy(AcStrategyType(raw));
        let side = (acs.covered_blocks_y() * 8).max(acs.covered_blocks_x() * 8);
        let modes: Vec<usize> = if side <= 32 {
            (0..MODES).collect()
        } else if side <= 64 {
            vec![2, 3, 5, 9, 16, 19]
        } else {
            vec![3, 5, 19]
        };
        for &mode in &modes {
            cases.push(pixels_request(raw, 5, mode));
        }
        if side <= 32 {
            // Unpadded and wide strides with full-mantissa noise.
            for pad in [0, 13] {
                cases.push(pixels_request(raw, pad, 19));
            }
        }
    }
    compare(cases);
}

#[test]
fn encoder_dc_from_lowest_frequencies_matches_all_strategies() {
    let mut cases = Vec::new();
    for raw in 0..27u32 {
        let acs = AcStrategy::from_raw_strategy(AcStrategyType(raw));
        let rows = acs.covered_blocks_y();
        let cols = acs.covered_blocks_x();
        for mode in 0..MODES {
            for pad in [0, 3] {
                let stride = cols + pad;
                let mut s = format!("etx-llf {raw} {stride}");
                push_words(
                    &mut s,
                    &values(rows * cols * 64, mode, 0x51ed270b ^ u64::from(raw)),
                );
                cases.push(s);
            }
        }
    }
    compare(cases);
}

#[test]
fn encoder_afvdct4x4_matches_impulses_noise_and_exceptional_values() {
    let mut cases = Vec::new();
    for position in 0..16 {
        let mut v = [0u32; 16];
        v[position] = bits(1.0);
        let mut s = String::from("etx-afv");
        push_words(&mut s, &v);
        cases.push(s);
        for &special in &SPECIAL {
            let mut v = [bits(0.125); 16];
            v[position] = special;
            let mut s = String::from("etx-afv");
            push_words(&mut s, &v);
            cases.push(s);
        }
    }
    for seed in 1..=64u64 {
        let mut s = String::from("etx-afv");
        push_words(
            &mut s,
            &values(16, if seed % 2 == 0 { 5 } else { 19 }, seed),
        );
        cases.push(s);
    }
    compare(cases);
}

// Competing NaN payloads: identity, DCT2X2, DCT4X4, the split 4x8/8x4
// blocks, all AFV kinds, rectangular DCTs, the AFV basis sum, and the
// reinterpreting IDCT. The ordinary and ASan archive objects of
// TransformFromPixels choose different operands in their 8-point and larger
// DCTs, so those requests use the dedicated `etx-unit-pixels` instantiation
// of the pristine template (see encoder_transforms_unit.cc); AFVDCT4x4 and
// DCFromLowestFrequencies agree in both archive graphs and use the archive.
#[test]
fn encoder_transforms_competing_nan_payloads_match_original_operand_order() {
    let pairs = [
        (0x7fc12345, 0xffc98765),
        (0xffc98765, 0x7fc12345),
        (0x7fa12345, 0xffc98765),
        (0xffc98765, 0x7fa12345),
        (0x7fa12345, 0xffa98765),
        (0xffa98765, 0x7fa12345),
    ];
    let mut cases = Vec::new();
    for raw in [0u32, 1, 2, 3, 6, 7, 12, 13, 14, 15, 16, 17] {
        let acs = AcStrategy::from_raw_strategy(AcStrategyType(raw));
        let rows = acs.covered_blocks_y() * 8;
        let cols = acs.covered_blocks_x() * 8;
        let stride = cols + 3;
        let positions = [
            (0, 1),
            (0, stride),
            (1, stride + 1),
            (0, 4 * stride + 4),
            (4, 4 * stride),
            (2 * stride + 2, 5 * stride + 6),
        ];
        for &(first, second) in &positions {
            for &(a, b) in &pairs {
                let mut v = vec![bits(0.125); rows * stride];
                v[first] = a;
                v[second] = b;
                let mut s = format!("etx-unit-pixels {raw} {stride}");
                push_words(&mut s, &v);
                cases.push(s);
            }
        }
    }
    for (first, second) in [(0, 1), (0, 15), (5, 10), (3, 12)] {
        for &(a, b) in &pairs {
            let mut v = [bits(0.125); 16];
            v[first] = a;
            v[second] = b;
            let mut s = String::from("etx-afv");
            push_words(&mut s, &v);
            cases.push(s);
        }
    }
    for raw in [4u32, 5, 6, 7, 10, 11] {
        let acs = AcStrategy::from_raw_strategy(AcStrategyType(raw));
        let rows = acs.covered_blocks_y();
        let cols = acs.covered_blocks_x();
        let stride = rows.max(cols) * 8;
        let last = (rows.min(cols) - 1) * stride + rows.max(cols) - 1;
        for second in [1, stride, last] {
            for &(a, b) in &pairs {
                let mut v = vec![bits(0.125); rows * cols * 64];
                v[0] = a;
                v[second] = b;
                let mut s = format!("etx-llf {raw} {}", cols + 2);
                push_words(&mut s, &v);
                cases.push(s);
            }
        }
    }
    super::dct::compare_competing_payloads(cases);
}

fn plane_values(count: usize, kind: usize, seed: u64) -> Vec<u32> {
    let mut seed = seed | 1;
    let mut v: Vec<u32> = (0..count)
        .map(|i| match kind {
            // Pixel-like values in [-0.25, 1.25).
            0 => bits((next(&mut seed) % (3 << 22)) as f32 / (1u32 << 23) as f32 - 0.25),
            1 => bits(((i * 7) % 13) as f32 / 12.0),
            _ => bits(0.5),
        })
        .collect();
    if kind >= 2 && count != 0 {
        v[(count * 2) / 5] = SPECIAL[(kind - 2) % SPECIAL.len()];
    }
    v
}

const SIZES: [(usize, usize); 22] = [
    (1, 1),
    (2, 1),
    (1, 2),
    (2, 2),
    (3, 3),
    (4, 5),
    (5, 4),
    (6, 6),
    (7, 7),
    (8, 8),
    (9, 5),
    (10, 3),
    (11, 6),
    (12, 2),
    (13, 7),
    (14, 4),
    (15, 9),
    (16, 5),
    (17, 6),
    (25, 3),
    (33, 11),
    (41, 15),
];

fn s5_request(
    size: (usize, usize),
    in_rect: [usize; 4],
    out_size: (usize, usize),
    out_rect: [usize; 4],
    weights: [f32; 6],
    kind: usize,
) -> String {
    let mut s = format!(
        "etx-s5 {} {} {} {} {} {} {} {} {} {} {} {}",
        size.0,
        size.1,
        in_rect[0],
        in_rect[1],
        in_rect[2],
        in_rect[3],
        out_size.0,
        out_size.1,
        out_rect[0],
        out_rect[1],
        out_rect[2],
        out_rect[3]
    );
    push_words(&mut s, &weights.map(bits));
    push_words(
        &mut s,
        &plane_values(size.0 * size.1, kind, (size.0 * 31 + size.1) as u64),
    );
    s
}

// GaborishInverse's weight layout (c r R d D L) for mul = 1, before normalization.
const GABORISH_WEIGHTS: [u32; 6] = [
    0x3f800000, 0xbdc2796c, 0x3c609fee, 0xbd2810e2, 0xbac1d7dd, 0x3bd55391,
];

#[test]
fn encoder_symmetric5_matches_every_xsize_mod_8_rects_borders_and_values() {
    let mut cases = Vec::new();
    let weight_sets = [
        GABORISH_WEIGHTS.map(f32::from_bits),
        [0.5, 0.125, -0.0625, 0.03125, -0.015625, 0.25],
        [1.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    ];
    for &(x, y) in &SIZES {
        let full = [0, 0, x, y];
        for (w, &weights) in weight_sets.iter().enumerate() {
            for kind in [0, 1] {
                cases.push(s5_request(
                    (x, y),
                    full,
                    (x, y),
                    full,
                    weights,
                    kind + w % 2,
                ));
            }
        }
        for kind in 2..2 + SPECIAL.len() {
            cases.push(s5_request((x, y), full, (x, y), full, weight_sets[0], kind));
        }
        // Output rectangles inside a larger owner.
        cases.push(s5_request(
            (x, y),
            full,
            (x + 5, y + 3),
            [3, 2, x, y],
            weight_sets[1],
            0,
        ));
        // Inner input rectangles: interior columns and unchanged rows.
        if x > 6 && y > 6 {
            let inner = [3, 3, x - 6, y - 6];
            cases.push(s5_request(
                (x, y),
                inner,
                (x - 6, y - 6),
                [0, 0, x - 6, y - 6],
                weight_sets[0],
                0,
            ));
            cases.push(s5_request(
                (x, y),
                [1, 2, x - 3, y - 2],
                (x, y),
                [2, 0, x - 3, y - 2],
                weight_sets[1],
                1,
            ));
        }
    }
    // Mismatched rectangle sizes return the original ensure failures.
    cases.push(s5_request(
        (8, 8),
        [0, 0, 8, 8],
        (8, 8),
        [0, 0, 7, 8],
        weight_sets[0],
        0,
    ));
    cases.push(s5_request(
        (8, 8),
        [0, 0, 8, 8],
        (8, 8),
        [0, 0, 8, 7],
        weight_sets[0],
        0,
    ));
    compare(cases);
}

fn s3_request(size: (usize, usize), r: [usize; 4], weights: [f32; 3], kind: usize) -> String {
    let mut s = format!(
        "etx-s3 {} {} {} {} {} {}",
        size.0, size.1, r[0], r[1], r[2], r[3]
    );
    push_words(&mut s, &weights.map(bits));
    push_words(
        &mut s,
        &plane_values(size.0 * size.1, kind, (size.0 * 17 + size.1) as u64),
    );
    s
}

#[test]
fn encoder_slow_symmetric3_matches_contraction_borders_and_values() {
    let mut cases = Vec::new();
    let weight_sets = [
        [1.0, 0.0, 0.0],
        [0.6, 0.075, 0.025],
        [
            f32::from_bits(0x3f2aaaab),
            f32::from_bits(0x3dcccccd),
            f32::from_bits(0xbc23d70a),
        ],
    ];
    for &(x, y) in &SIZES {
        for &weights in &weight_sets {
            for kind in 0..2 + SPECIAL.len() {
                cases.push(s3_request((x, y), [0, 0, x, y], weights, kind));
            }
        }
        if x > 2 && y > 2 {
            // Only the rectangle's size is used; it starts at the origin.
            cases.push(s3_request((x, y), [1, 1, x - 1, y - 1], weight_sets[1], 0));
        }
    }
    compare(cases);
}

fn gab_request(size: (usize, usize), r: [usize; 4], mul: [u32; 3], kind: usize) -> String {
    let mut s = format!(
        "etx-gab {} {} {} {} {} {}",
        size.0, size.1, r[0], r[1], r[2], r[3]
    );
    push_words(&mut s, &mul);
    let count = size.0 * size.1;
    for c in 0..3 {
        push_words(&mut s, &plane_values(count, kind, (count * 3 + c) as u64));
    }
    s
}

#[test]
fn encoder_gaborish_inverse_matches_every_xsize_mod_8_rects_and_weights() {
    let default = bits(0.927_189_3);
    let mut cases = Vec::new();
    let muls = [
        [default; 3],
        [bits(1.0), bits(0.5), bits(0.0)],
        // sum < 1e-5 clamps; NaN and infinite multipliers propagate.
        [bits(3.0), bits(2.258), bits(-1.0)],
        [0x7fc12345, 0x7f800000, 0xff800000],
    ];
    for &(x, y) in &SIZES {
        let full = [0, 0, x, y];
        for (m, &mul) in muls.iter().enumerate() {
            cases.push(gab_request((x, y), full, mul, m % 2));
        }
        for kind in 2..2 + SPECIAL.len() {
            cases.push(gab_request((x, y), full, muls[0], kind));
        }
        if x > 4 && y > 4 {
            // The rectangle is extended by three pixels within the image.
            cases.push(gab_request((x, y), [2, 2, x - 4, y - 4], muls[0], 0));
            cases.push(gab_request((x, y), [x / 2, y / 2, 1, 1], muls[1], 1));
        }
    }
    compare(cases);
}

#[test]
fn encoder_gaborish_test_cases_match_original_roundtrip_words() {
    // enc_gaborish_test.cc TestZero and TestFlat, plus its disabled TestDirac
    // input; exact words replace VerifyRelativeError's tolerances.
    compare((0..3).map(|which| format!("etx-gabtest {which}")).collect());
}

const INTENSITIES: [u32; 14] = [
    0x437f0000, // 255
    0x42c80000, // 100
    0x42a00000, // 80
    0x447a0000, // 1000
    0x461c4000, // 10000
    0, 0x80000000, 0x3f800000, 1, 0x7f7fffff, 0x7f800000, 0xff800000, 0x7fc12345, 0x7fa12345,
];

#[test]
fn encoder_premul_absorb_matches_intensity_targets() {
    compare(
        INTENSITIES
            .iter()
            .map(|i| format!("etx-premul {i}"))
            .collect(),
    );
}

#[test]
fn encoder_linear_rgb_row_to_xyb_matches_values_and_widths() {
    let mut cases = Vec::new();
    for xsize in 1..=17 {
        for &intensity in &INTENSITIES[..2] {
            for kind in 0..2 + SPECIAL.len() {
                let mut s = format!("etx-row {xsize} {intensity}");
                for c in 0..3 {
                    push_words(&mut s, &plane_values(xsize, kind, (xsize * 3 + c) as u64));
                }
                cases.push(s);
            }
        }
    }
    // Wide-gamut, negative, cube-root boundary and overflow inputs.
    let edge = [
        0.0f32,
        -0.0,
        -0.5,
        1.0,
        1.5,
        1e-30,
        1e-38,
        1e30,
        3.4e38,
        -3.4e38,
        0.0037930732,
        1e-3,
    ];
    for &intensity in &INTENSITIES {
        let mut s = format!("etx-row {} {intensity}", edge.len());
        for c in 0..3 {
            let row: Vec<u32> = (0..edge.len())
                .map(|i| bits(edge[(i + 5 * c) % edge.len()]))
                .collect();
            push_words(&mut s, &row);
        }
        cases.push(s);
    }
    compare(cases);
}

fn xyb_encoding(cs: ColorSpace, tf: TransferFunction, ri: RenderingIntent) -> String {
    let mut c = super::cms::srgb();
    c.color_space = cs;
    c.transfer_function = tf;
    c.rendering_intent = ri;
    super::cms::external_args(&c)
}

#[test]
fn encoder_to_xyb_matches_srgb_and_linear_branches_linear_output_and_checks() {
    let mut cases = Vec::new();
    let mut admitted = Vec::new();
    for cs in [ColorSpace::RGB, ColorSpace::GRAY] {
        for tf in [TransferFunction::SRGB, TransferFunction::LINEAR] {
            for ri in [RenderingIntent::RELATIVE, RenderingIntent::PERCEPTUAL] {
                admitted.push(xyb_encoding(cs, tf, ri));
            }
        }
    }
    let sizes = [
        (1, 1),
        (2, 1),
        (7, 3),
        (8, 2),
        (9, 2),
        (12, 1),
        (15, 2),
        (17, 3),
    ];
    for (e, encoding) in admitted.iter().enumerate() {
        for &(x, y) in &sizes {
            for kind in [0, 1] {
                for &intensity in &INTENSITIES[..2] {
                    for linear_mode in [0, 1] {
                        let mut s =
                            format!("etx-xyb {encoding} {intensity} 0 {linear_mode} {x} {y}");
                        for c in 0..3 {
                            push_words(
                                &mut s,
                                &plane_values(x * y, kind, (e * 64 + x * 8 + c) as u64),
                            );
                        }
                        cases.push(s);
                    }
                }
            }
        }
        // Exceptional samples, a same-size black plane, and the original
        // SameSize ensure failures for black and linear.
        for kind in 2..2 + SPECIAL.len() {
            for linear_mode in [0, 1] {
                let mut s = format!("etx-xyb {encoding} {} 1 {linear_mode} 9 2", INTENSITIES[0]);
                for c in 0..3 {
                    push_words(&mut s, &plane_values(18, kind, (e * 7 + c) as u64));
                }
                cases.push(s);
            }
        }
        for (black_mode, linear_mode) in [(2, 0), (0, 2), (2, 2), (1, 2)] {
            let mut s = format!(
                "etx-xyb {encoding} {} {black_mode} {linear_mode} 3 2",
                INTENSITIES[0]
            );
            for c in 0..3 {
                push_words(&mut s, &plane_values(6, 0, c as u64));
            }
            cases.push(s);
        }
    }
    // Encodings that would take ApplyColorTransform: only the original branch
    // predicates are compared, and the Rust admission rejects every one.
    let mut c = super::cms::srgb();
    c.primaries = Primaries::P3;
    let p3 = super::cms::external_args(&c);
    let mut c = super::cms::srgb();
    c.white_point = WhitePoint::DCI;
    let dci = super::cms::external_args(&c);
    let mut c = super::cms::srgb();
    c.transfer_function = TransferFunction::GAMMA;
    c.gamma = 1.0 / 2.2;
    let gamma = super::cms::external_args(&c);
    for encoding in [
        xyb_encoding(
            ColorSpace::RGB,
            TransferFunction::PQ,
            RenderingIntent::RELATIVE,
        ),
        xyb_encoding(
            ColorSpace::RGB,
            TransferFunction::HLG,
            RenderingIntent::RELATIVE,
        ),
        xyb_encoding(
            ColorSpace::RGB,
            TransferFunction::BT709,
            RenderingIntent::RELATIVE,
        ),
        xyb_encoding(
            ColorSpace::GRAY,
            TransferFunction::PQ,
            RenderingIntent::RELATIVE,
        ),
        xyb_encoding(
            ColorSpace::XYB,
            TransferFunction::LINEAR,
            RenderingIntent::PERCEPTUAL,
        ),
        p3,
        dci,
        gamma,
    ] {
        let mut s = format!("etx-xyb {encoding} {} 0 1 2 1", INTENSITIES[0]);
        for c in 0..3 {
            push_words(&mut s, &plane_values(2, 0, c as u64));
        }
        cases.push(s);
    }
    compare(cases);
}
