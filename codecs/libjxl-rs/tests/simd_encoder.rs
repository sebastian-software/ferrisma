// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! SIMD parity of the encoder's Highway exports (port ADR 0004, "Encoder
//! path"): the port's NEON/AVX2 instantiations against the original built
//! for the same single Highway target (`JXL_CPP_ORACLE_SIMD`), through the
//! `esx-*` requests of tools/oracle/encoder_simd.cc. Every word the original
//! may write (whole vectors past the row length included) is compared bit
//! for bit, except that any NaN equals any NaN (decision D8). The encoder's
//! own dispatch point (`hwy::encoder_target`) is forced to the SIMD target
//! where a test goes through it; each such family also shows that the scalar
//! path gives different words for some of its requests, so the comparison
//! tells the two paths apart.
#![cfg(feature = "cpp-oracle-simd")]

mod simd_common;

use jxl::jxl::ac_strategy::{AcStrategy, AcStrategyType, NUM_VALID_STRATEGIES};
use jxl::jxl::base::status::status_code;
use jxl::jxl::color_encoding::{
    ColorSpace, JxlColorEncoding, Primaries, RenderingIntent, TransferFunction, WhitePoint,
};
use jxl::jxl::color_encoding_internal::ColorEncoding;
use jxl::jxl::enc_transforms::{afvdct4x4, dc_from_lowest_frequencies, transform_from_pixels};
use jxl::jxl::enc_xyb::{XybInputEncoding, to_xyb};
use jxl::jxl::enc_xyb_simd::{compute_premul_absorb_simd, linear_rgb_row_to_xyb_simd};
use jxl::jxl::hwy::{self, Target};
use jxl::jxl::image::{Image3F, ImageF};
use jxl::jxl::image_ops::zero_fill_image;
use simd_common::{check_rows, next, special_words, target};
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
fn float(args: &mut Args<'_>) -> f32 {
    f32::from_bits(number(args))
}
fn f64_arg(args: &mut Args<'_>) -> f64 {
    f64::from_bits(number(args))
}
fn word(out: &mut String, v: f32) {
    write!(out, " {}", v.to_bits()).unwrap();
}
fn words(out: &mut String, values: &[f32]) {
    out.push_str(" B");
    for &v in values {
        word(out, v);
    }
}
fn round_up8(x: usize) -> usize {
    x.div_ceil(8) * 8
}
/// The finite sentinel words of tools/oracle/encoder_simd.cc.
fn sentinel(salt: u32, i: usize) -> f32 {
    f32::from_bits(0x5a00_0000 + salt * 0x10_0000 + (i as u32 & 0xf_ffff))
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

// enc_xyb ------------------------------------------------------------------

fn observe_premul(request: &str, t: Target) -> String {
    let mut args = request.split_whitespace();
    assert_eq!(args.next(), Some("esx-premul"));
    let intensity = float(&mut args);
    let mut premul: Vec<f32> = (0..12 * 8 + 4).map(|i| sentinel(1, i)).collect();
    compute_premul_absorb_simd(t, intensity, &mut premul);
    let mut out = "M".to_owned();
    words(&mut out, &premul);
    out
}

fn observe_row(request: &str, t: Target) -> String {
    let mut args = request.split_whitespace();
    assert_eq!(args.next(), Some("esx-row"));
    let xsize: usize = number(&mut args);
    let intensity = float(&mut args);
    let mut premul = vec![0.0; 12 * 8];
    compute_premul_absorb_simd(t, intensity, &mut premul);
    let padded = round_up8(xsize);
    let mut rows: [Vec<f32>; 3] = std::array::from_fn(|c| {
        let mut row: Vec<f32> = (0..padded).map(|_| float(&mut args)).collect();
        row.extend((padded..padded + 4).map(|x| sentinel(2 + c as u32, x)));
        row
    });
    let [row0, row1, row2] = &mut rows;
    linear_rgb_row_to_xyb_simd(t, row0, row1, row2, &premul, xsize);
    let mut out = "R".to_owned();
    for row in &rows {
        words(&mut out, row);
    }
    out
}

fn print_image(out: &mut String, image: &Image3F, padded: usize) {
    for c in 0..3 {
        out.push_str(" P");
        for y in 0..image.ysize() {
            for &v in &image.plane_row(c, y)[..padded] {
                word(out, v);
            }
        }
    }
}

/// The port's observation of an `esx-xyb` request through the encoder's
/// dispatching `to_xyb` (the caller sets the encoder's selection).
fn observe_xyb(request: &str) -> String {
    let mut args = request.split_whitespace();
    assert_eq!(args.next(), Some("esx-xyb"));
    let external = JxlColorEncoding {
        color_space: ColorSpace(number(&mut args)),
        white_point: WhitePoint(number(&mut args)),
        primaries: Primaries(number(&mut args)),
        transfer_function: TransferFunction(number(&mut args)),
        rendering_intent: RenderingIntent(number(&mut args)),
        gamma: f64_arg(&mut args),
        white_point_xy: [f64_arg(&mut args), f64_arg(&mut args)],
        primaries_red_xy: [f64_arg(&mut args), f64_arg(&mut args)],
        primaries_green_xy: [f64_arg(&mut args), f64_arg(&mut args)],
        primaries_blue_xy: [f64_arg(&mut args), f64_arg(&mut args)],
    };
    let mut c_current = ColorEncoding::default();
    let converted = c_current.from_external(&external);
    let intensity = float(&mut args);
    let black_mode: usize = number(&mut args);
    let linear_mode: usize = number(&mut args);
    let xsize: usize = number(&mut args);
    let ysize: usize = number(&mut args);
    let admitted = XybInputEncoding::new(&c_current);
    let (same_linear, is_srgb) = match admitted {
        Some(XybInputEncoding::LinearSrgb) => (1, 0),
        Some(XybInputEncoding::Srgb) => (0, 1),
        None => (0, 0),
    };
    let mut out = format!("X {} {same_linear} {is_srgb}", status_code(&converted));
    let padded = round_up8(xsize);
    let mut image = Image3F::create(xsize, ysize).unwrap();
    assert!(
        image.pixels_per_row() as usize >= padded,
        "row padding below 8 lanes"
    );
    for c in 0..3 {
        for y in 0..ysize {
            for v in &mut image.plane_row_mut(c, y)[..padded] {
                *v = float(&mut args);
            }
        }
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
        linear = Image3F::create(xsize, ysize + usize::from(linear_mode == 2)).unwrap();
        for c in 0..3 {
            for y in 0..linear.ysize() {
                for (x, v) in linear.plane_row_mut(c, y)[..padded].iter_mut().enumerate() {
                    *v = sentinel(5 + c as u32, y * padded + x);
                }
            }
        }
    }
    let status = to_xyb(
        admitted,
        intensity,
        (black_mode != 0).then_some(&black),
        None,
        &mut image,
        (linear_mode != 0).then_some(&mut linear),
    );
    write!(out, " {}", status_code(&status)).unwrap();
    print_image(&mut out, &image, padded);
    if linear_mode != 0 {
        print_image(&mut out, &linear, padded);
    }
    out
}

/// The scalar encoder suite's intensity targets (tests/parity/encoder_transforms.rs)
/// plus the special words.
const INTENSITIES: [u32; 14] = [
    0x437f0000, // 255
    0x42c80000, // 100
    0x42a00000, // 80
    0x447a0000, // 1000
    0x461c4000, // 10000
    0, 0x80000000, 0x3f800000, 1, 0x7f7fffff, 0x7f800000, 0xff800000, 0x7fc12345, 0x7fa12345,
];

/// Sample words of kind 0 (pixel-like in [-0.25, 1.25)), 1 (a ramp),
/// 2 (wide gamut and HDR: [-4, 60)), 3 (cube-root and sRGB knee boundaries:
/// tiny, subnormal, around 0.04045 and the opsin bias), 4 + k (pixel-like
/// with the special word k at every fifth sample).
fn sample_words(count: usize, kind: usize, seed: u64) -> Vec<u32> {
    let specials = special_words();
    let mut seed = seed.wrapping_mul(0x9e37_79b9_7f4a_7c15) | 1;
    (0..count)
        .map(|i| {
            let r = next(&mut seed);
            match kind {
                0 => ((r % (3 << 22)) as f32 / (1u32 << 23) as f32 - 0.25).to_bits(),
                1 => (((i * 7) % 13) as f32 / 12.0).to_bits(),
                2 => ((r % (1 << 24)) as f32 / (1u32 << 24) as f32 * 64.0 - 4.0).to_bits(),
                3 => [
                    0.0f32,
                    -0.0,
                    1e-30,
                    1e-38,
                    1.2e-38,
                    0.04045,
                    0.040_450_003,
                    0.040_449_996,
                    0.003_793_073_2,
                    1e-3,
                    1.0,
                    0.5,
                    -1e-7,
                    3.4e38,
                    -3.4e38,
                    7.0,
                ][(i + r as usize) % 16]
                    .to_bits(),
                k => {
                    if i % 5 == 2 {
                        specials[(k - 4 + i / 5) % specials.len()]
                    } else {
                        ((r % (3 << 22)) as f32 / (1u32 << 23) as f32 - 0.25).to_bits()
                    }
                }
            }
        })
        .collect()
}
const KINDS: usize = 4 + 8;

fn push_words(s: &mut String, values: &[u32]) {
    for v in values {
        write!(s, " {v}").unwrap();
    }
}

fn external_args(c: &JxlColorEncoding) -> String {
    let mut out = format!(
        "{} {} {} {} {} {}",
        c.color_space.0,
        c.white_point.0,
        c.primaries.0,
        c.transfer_function.0,
        c.rendering_intent.0,
        c.gamma.to_bits()
    );
    for xy in [
        c.white_point_xy,
        c.primaries_red_xy,
        c.primaries_green_xy,
        c.primaries_blue_xy,
    ] {
        for value in xy {
            write!(out, " {}", value.to_bits()).unwrap();
        }
    }
    out
}
fn srgb() -> JxlColorEncoding {
    JxlColorEncoding {
        color_space: ColorSpace::RGB,
        white_point: WhitePoint::D65,
        white_point_xy: [0.3127, 0.3290],
        primaries: Primaries::SRGB,
        primaries_red_xy: [0.639998686, 0.330010138],
        primaries_green_xy: [0.300003784, 0.600003357],
        primaries_blue_xy: [0.150002046, 0.059997204],
        transfer_function: TransferFunction::SRGB,
        gamma: 0.0,
        rendering_intent: RenderingIntent::RELATIVE,
    }
}
fn encoding(cs: ColorSpace, tf: TransferFunction, ri: RenderingIntent) -> String {
    let mut c = srgb();
    c.color_space = cs;
    c.transfer_function = tf;
    c.rendering_intent = ri;
    external_args(&c)
}

#[test]
fn enc_simd_premul_absorb_matches_the_original_target() {
    let t = target();
    let requests: Vec<String> = INTENSITIES
        .iter()
        .chain(&special_words())
        .map(|i| format!("esx-premul {i}"))
        .collect();
    check_rows(
        "esx-premul",
        &requests,
        |r| (observe_premul(r, t), Box::new(all_defined)),
        all_float,
    );
}

#[test]
fn enc_simd_linear_rgb_row_to_xyb_matches_the_original_target() {
    let t = target();
    let mut requests = Vec::new();
    for xsize in (1..=40).chain([63, 64, 65, 127, 129]) {
        for &intensity in &INTENSITIES[..3] {
            for kind in 0..KINDS {
                let mut s = format!("esx-row {xsize} {intensity}");
                for c in 0..3 {
                    push_words(
                        &mut s,
                        &sample_words(round_up8(xsize), kind, (xsize * 31 + kind * 7 + c) as u64),
                    );
                }
                requests.push(s);
            }
        }
    }
    for &intensity in &INTENSITIES[3..] {
        for kind in [0, 3, 4] {
            let mut s = format!("esx-row 13 {intensity}");
            for c in 0..3 {
                push_words(&mut s, &sample_words(16, kind, (kind * 3 + c) as u64));
            }
            requests.push(s);
        }
    }
    check_rows(
        "esx-row",
        &requests,
        |r| (observe_row(r, t), Box::new(all_defined)),
        all_float,
    );
}

fn xyb_requests() -> Vec<String> {
    let mut admitted = Vec::new();
    for cs in [ColorSpace::RGB, ColorSpace::GRAY] {
        for tf in [TransferFunction::SRGB, TransferFunction::LINEAR] {
            for ri in [RenderingIntent::RELATIVE, RenderingIntent::PERCEPTUAL] {
                admitted.push(encoding(cs, tf, ri));
            }
        }
    }
    let sizes = [
        (1, 1),
        (2, 1),
        (3, 2),
        (4, 1),
        (5, 3),
        (7, 3),
        (8, 2),
        (9, 2),
        (12, 1),
        (15, 2),
        (16, 3),
        (17, 3),
        (31, 2),
        (33, 1),
        (64, 2),
        (65, 2),
        (129, 1),
    ];
    let mut requests = Vec::new();
    for (e, encoding) in admitted.iter().enumerate() {
        for &(x, y) in &sizes {
            for kind in 0..KINDS {
                for linear_mode in [0, 1] {
                    let intensity = INTENSITIES[(kind + x) % 3];
                    let mut s = format!("esx-xyb {encoding} {intensity} 0 {linear_mode} {x} {y}");
                    for c in 0..3 {
                        push_words(
                            &mut s,
                            &sample_words(
                                y * round_up8(x),
                                kind,
                                (e * 4096 + x * 64 + kind * 4 + c) as u64,
                            ),
                        );
                    }
                    requests.push(s);
                }
            }
        }
        // A same-size black plane, the SameSize ensure failures for black and
        // linear, and the exceptional intensities.
        for (black_mode, linear_mode) in [(1, 0), (1, 1), (2, 0), (0, 2), (2, 2), (1, 2)] {
            let mut s = format!(
                "esx-xyb {encoding} {} {black_mode} {linear_mode} 3 2",
                INTENSITIES[0]
            );
            for c in 0..3 {
                push_words(&mut s, &sample_words(16, 0, c as u64));
            }
            requests.push(s);
        }
        for &intensity in &INTENSITIES[3..] {
            let mut s = format!("esx-xyb {encoding} {intensity} 0 1 11 2");
            for c in 0..3 {
                push_words(&mut s, &sample_words(32, 4, (e + c) as u64));
            }
            requests.push(s);
        }
    }
    // Encodings that take ApplyColorTransform: only the branch predicates.
    let mut c = srgb();
    c.primaries = Primaries::P3;
    let p3 = external_args(&c);
    for encoding in [
        encoding(
            ColorSpace::RGB,
            TransferFunction::PQ,
            RenderingIntent::RELATIVE,
        ),
        encoding(
            ColorSpace::XYB,
            TransferFunction::LINEAR,
            RenderingIntent::PERCEPTUAL,
        ),
        p3,
    ] {
        let mut s = format!("esx-xyb {encoding} {} 0 1 2 1", INTENSITIES[0]);
        for c in 0..3 {
            push_words(&mut s, &sample_words(8, 0, c as u64));
        }
        requests.push(s);
    }
    requests
}

#[test]
fn enc_simd_to_xyb_through_the_encoder_dispatch_matches_the_original_target() {
    check_through_dispatch("esx-xyb", &xyb_requests(), observe_xyb);
}

#[test]
fn enc_simd_the_encoder_selects_the_scalar_path_by_default() {
    let _lock = dispatch_lock();
    const { assert!(!hwy::ENCODER_SELECTS_SIMD) };
    hwy::set_encoder_simd_for_test(None);
    assert!(hwy::encoder_target().is_none());
    hwy::set_encoder_simd_for_test(Some(true));
    assert_eq!(
        hwy::encoder_target().map(Target::name),
        Some(target().name())
    );
    // The decoder's selection is a separate switch.
    assert!(
        hwy::DispatchScope::Outside
            .enter(hwy::decoder_target)
            .is_none()
    );
    hwy::set_encoder_simd_for_test(None);
    assert!(hwy::encoder_target().is_none());
}

// enc_transforms ------------------------------------------------------------

/// The encoder's callers' DCT scratch for 8 lanes (tools/oracle/encoder_simd.cc).
const CALLER_SCRATCH: usize = 2 * 256 * 256 + 3 * 8 * 256;

fn scratch_words(out: &mut String, scratch: &[f32]) {
    let used = (0..scratch.len())
        .rev()
        .find(|&i| scratch[i].to_bits() != sentinel(9, i).to_bits())
        .map_or(0, |i| i + 1);
    words(out, &scratch[..(used + 4).min(scratch.len())]);
}

/// The port's observation of an `esx-pixels`, `esx-llf` or `esx-afv`
/// request through the encoder's dispatching functions (the caller sets the
/// encoder's selection).
fn observe_transform(request: &str) -> String {
    let mut args = request.split_whitespace();
    let kind = args.next().unwrap();
    if kind == "esx-afv" {
        let mut pixels: Vec<f32> = (0..16).map(|_| float(&mut args)).collect();
        pixels.extend((16..24).map(|i| sentinel(14, i)));
        let mut coeffs: Vec<f32> = (0..24).map(|i| sentinel(15, i)).collect();
        afvdct4x4(&pixels, &mut coeffs);
        let mut out = "A".to_owned();
        words(&mut out, &pixels);
        words(&mut out, &coeffs);
        return out;
    }
    let raw: u32 = number(&mut args);
    let stride: usize = number(&mut args);
    let strategy = AcStrategyType(raw);
    let acs = AcStrategy::from_raw_strategy(strategy);
    let (bx, by) = (acs.covered_blocks_x(), acs.covered_blocks_y());
    let mut scratch: Vec<f32> = (0..CALLER_SCRATCH).map(|i| sentinel(9, i)).collect();
    let mut out;
    if kind == "esx-pixels" {
        let (rows, cols) = (by * 8, bx * 8);
        let mut pixels: Vec<f32> = (0..rows * stride).map(|_| float(&mut args)).collect();
        pixels.extend((rows * stride..rows * stride + 8).map(|i| sentinel(10, i)));
        let mut coefficients: Vec<f32> = (0..rows * cols + 8).map(|i| sentinel(11, i)).collect();
        transform_from_pixels(strategy, &pixels, stride, &mut coefficients, &mut scratch);
        out = "T".to_owned();
        words(&mut out, &pixels);
        words(&mut out, &coefficients);
    } else {
        assert_eq!(kind, "esx-llf");
        let n = by * bx * 64;
        let mut block: Vec<f32> = (0..n).map(|_| float(&mut args)).collect();
        block.extend((n..n + 8).map(|i| sentinel(12, i)));
        let mut dc: Vec<f32> = (0..by * stride + 8).map(|i| sentinel(13, i)).collect();
        dc_from_lowest_frequencies(strategy, &block, &mut dc, stride, &mut scratch);
        out = "L".to_owned();
        words(&mut out, &block);
        words(&mut out, &dc);
    }
    scratch_words(&mut out, &scratch);
    out
}

/// Every strategy, strides with and without padding, and the value kinds
/// (fewer kinds for the largest transforms).
fn pixels_requests() -> Vec<String> {
    let mut requests = Vec::new();
    for raw in 0..u32::from(NUM_VALID_STRATEGIES) {
        let acs = AcStrategy::from_raw_strategy(AcStrategyType(raw));
        let (rows, cols) = (acs.covered_blocks_y() * 8, acs.covered_blocks_x() * 8);
        let large = rows * cols > 64 * 64;
        for pad in [0, 1, 5, 8] {
            for kind in 0..KINDS {
                if large && !(pad == 0 || pad == 5) || large && kind % 3 != 0 {
                    continue;
                }
                let stride = cols + pad;
                let mut s = format!("esx-pixels {raw} {stride}");
                push_words(
                    &mut s,
                    &sample_words(
                        rows * stride,
                        kind,
                        u64::from(raw) * 131 + pad as u64 * 17 + kind as u64,
                    ),
                );
                requests.push(s);
            }
        }
    }
    requests
}

#[test]
fn enc_simd_transform_from_pixels_through_the_encoder_dispatch_matches_the_original_target() {
    check_through_dispatch("esx-pixels", &pixels_requests(), observe_transform);
}

#[test]
fn enc_simd_dc_from_lowest_frequencies_through_the_encoder_dispatch_matches_the_original_target() {
    let mut requests = Vec::new();
    for raw in 0..u32::from(NUM_VALID_STRATEGIES) {
        let acs = AcStrategy::from_raw_strategy(AcStrategyType(raw));
        let (bx, by) = (acs.covered_blocks_x(), acs.covered_blocks_y());
        for pad in [0, 3, 8] {
            for kind in 0..KINDS {
                let stride = bx + pad;
                let mut s = format!("esx-llf {raw} {stride}");
                push_words(
                    &mut s,
                    &sample_words(
                        bx * by * 64,
                        kind,
                        u64::from(raw) * 77 + pad as u64 + kind as u64 * 5,
                    ),
                );
                requests.push(s);
            }
        }
    }
    check_through_dispatch("esx-llf", &requests, observe_transform);
}

#[test]
fn enc_simd_afvdct4x4_through_the_encoder_dispatch_matches_the_original_target() {
    let mut requests = Vec::new();
    for kind in 0..KINDS {
        for seed in 0..24u64 {
            let mut s = "esx-afv".to_owned();
            push_words(&mut s, &sample_words(16, kind, seed * 1000 + kind as u64));
            requests.push(s);
        }
    }
    // Impulses at every position.
    for i in 0..16 {
        let mut s = "esx-afv".to_owned();
        let impulse: Vec<u32> = (0..16)
            .map(|j| if i == j { 1.0f32 } else { 0.0 }.to_bits())
            .collect();
        push_words(&mut s, &impulse);
        requests.push(s);
    }
    check_through_dispatch("esx-afv", &requests, observe_transform);
}
