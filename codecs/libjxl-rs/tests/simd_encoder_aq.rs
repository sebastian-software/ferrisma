// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! SIMD parity of the encoder's AdaptiveQuantizationMap export (port ADR
//! 0004, "Encoder path"): the port's NEON/AVX2 instantiation against the
//! original built for the same single Highway target (`JXL_CPP_ORACLE_SIMD`),
//! through the `eaq-*` requests of tools/oracle/encoder_aq_simd.cc. Every
//! word the original writes is compared bit for bit, except that any NaN
//! equals any NaN (decision D8) and that, for a request with NaN or
//! infinity input, a word is compared only where the original's two scalar
//! reference builds agree (rule O1, ADR-0041 section 3). The requests go through the public
//! InitialQuantField with the encoder's dispatch point
//! (`hwy::encoder_target`) forced to the SIMD target; the family also shows
//! that the scalar path gives different words for some of its requests, so
//! the comparison tells the two paths apart.
#![cfg(feature = "cpp-oracle-simd")]

mod simd_common;

use jxl::jxl::base::rect::Rect;
use jxl::jxl::base::status::status_code;
use jxl::jxl::enc_adaptive_quantization::initial_quant_field;
use jxl::jxl::hwy;
use jxl::jxl::image::{Image3F, ImageF};
use simd_common::{next, special_words};
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
fn all_float(_: usize) -> bool {
    true
}
fn all_defined(_: usize) -> bool {
    true
}

/// The pad words of tools/oracle/encoder_aq_simd.cc.
const PAD_WORDS: [u32; 4] = [0x0000_0000, 0x7fc0_0000, 0x7f80_0000, 0xf149_f2ca];

fn print_plane(out: &mut String, plane: &ImageF) {
    write!(out, " P {} {}", plane.xsize(), plane.ysize()).unwrap();
    for y in 0..plane.ysize() {
        for &v in &plane.row(y)[..plane.xsize()] {
            write!(out, " {}", v.to_bits()).unwrap();
        }
    }
}

fn observe_map(request: &str) -> String {
    let mut args = request.split_whitespace();
    assert_eq!(args.next(), Some("eaq-map"));
    let distance = float(&mut args);
    let rescale = float(&mut args);
    let xsize: usize = number(&mut args);
    let ysize: usize = number(&mut args);
    let x0: usize = number(&mut args);
    let y0: usize = number(&mut args);
    let rxs: usize = number(&mut args);
    let rys: usize = number(&mut args);
    let pad_kind: usize = number(&mut args);
    let pad = f32::from_bits(PAD_WORDS[pad_kind]);
    let mut opsin = Image3F::create(xsize, ysize).unwrap();
    let stride = opsin.pixels_per_row() as usize;
    for c in 0..3 {
        for y in 0..ysize {
            let row = opsin.plane_row_mut(c, y);
            for v in &mut row[..xsize] {
                *v = float(&mut args);
            }
            for v in &mut row[xsize..stride] {
                *v = pad;
            }
        }
    }
    assert!(args.next().is_none());
    let mut mask = ImageF::default();
    let mut mask1x1 = ImageF::default();
    let qf = initial_quant_field(
        distance,
        &opsin,
        Rect::new(x0, y0, rxs, rys),
        None,
        rescale,
        &mut mask,
        &mut mask1x1,
    );
    let status = qf.as_ref().map(|_| ()).map_err(|e| *e);
    // The row stride is not compared: the encoder's planes keep the
    // scalar row padding (ADR 0004, "Encoder path"), which can be none; the
    // original's follows its target. The one sample HfModulation reads past
    // a row's last block is masked out (a port row without padding loads
    // +0.0 for it).
    let mut out = format!("Q {}", status_code(&status));
    if let Ok(field) = qf {
        print_plane(&mut out, &field);
        print_plane(&mut out, &mask);
        print_plane(&mut out, &mask1x1);
    }
    out
}

/// Value kinds of the XYB planes: photo-like smooth content, high-frequency
/// noise, flat, ramps, blue-dominated blocks (BlueModulation's branches),
/// negative and out-of-range samples (ZeroIfNegative, the clamps), large
/// magnitudes, special words sprinkled into photo content, and special
/// words only.
const KINDS: usize = 9;

fn sample(kind: usize, c: usize, x: usize, y: usize, seed: &mut u64, special: &[u32]) -> f32 {
    let r = (next(seed) >> 11) as f32 / (1u64 << 53) as f32;
    let fx = x as f32;
    let fy = y as f32;
    let smooth = 0.5 + 0.3 * (fx * 0.07).sin() * (fy * 0.05).cos();
    let photo = match c {
        0 => 0.02 * (fx * 0.11 + fy * 0.03).sin() + 0.004 * (r - 0.5),
        1 => smooth + 0.02 * (r - 0.5),
        _ => smooth * 0.9 + 0.05 * (fx * 0.02).cos() + 0.02 * (r - 0.5),
    };
    match kind {
        0 => photo,
        1 => match c {
            0 => 0.06 * (r - 0.5),
            1 => r,
            _ => r * 0.95,
        },
        2 => [0.01f32, 0.45, 0.4][c],
        3 => match c {
            0 => (fx - fy) * 1e-4,
            1 => fx / 256.0 + fy / 512.0,
            _ => fy / 300.0,
        },
        4 => {
            // Blue above Y + |X| by up to the limits, block-wise varied.
            let block = ((x / 8) * 7 + (y / 8) * 3) % 5;
            match c {
                0 => 0.003 * (r - 0.5),
                1 => 0.05 + 0.02 * block as f32 + 0.01 * r,
                _ => 0.05 + 0.006 * block as f32 * r + 0.02 * block as f32,
            }
        }
        5 => (r - 0.6) * [0.2f32, 1.5, 1.5][c],
        6 => (r - 0.5) * [2.0e3f32, 4.0e4, 1.0e5][c],
        7 => {
            if next(seed).is_multiple_of(23) {
                f32::from_bits(special[(next(seed) % special.len() as u64) as usize])
            } else {
                photo
            }
        }
        _ => f32::from_bits(special[(next(seed) % special.len() as u64) as usize]),
    }
}

#[allow(clippy::too_many_arguments)]
fn map_request(
    distance: f32,
    rescale: f32,
    xsize: usize,
    ysize: usize,
    rect: [usize; 4],
    pad_kind: usize,
    kind: usize,
    seed: u64,
) -> String {
    let special = special_words();
    let mut s = seed | 1;
    let mut request = format!(
        "eaq-map {} {} {xsize} {ysize} {} {} {} {} {pad_kind}",
        distance.to_bits(),
        rescale.to_bits(),
        rect[0],
        rect[1],
        rect[2],
        rect[3]
    );
    for c in 0..3 {
        for y in 0..ysize {
            for x in 0..xsize {
                let v = sample(kind, c, x, y, &mut s, &special);
                write!(request, " {}", v.to_bits()).unwrap();
            }
        }
    }
    request
}

/// Whole images (block multiples, one to four 64x64 tiles in each
/// direction, widths whose SIMD loops end at every remainder), images whose
/// rect leaves a partial block outside, and rects inside larger images
/// (the +-2 and +-4 pixel extensions of the 1x1 and pre-erosion passes),
/// each over the value kinds, distances across the FuzzyErosion branch and
/// the dampening ramp, rescales, and the four row-padding words.
#[test]
fn simd_adaptive_quantization_map_matches_the_original_target() {
    let mut requests = Vec::new();
    let distances = [0.05f32, 0.5, 1.0, 0.62, 1.9999, 2.0, 2.9, 4.5, 14.0, 20.0];
    let whole: [(usize, usize); 14] = [
        (8, 8),
        (16, 8),
        (8, 16),
        (24, 24),
        (32, 16),
        (40, 16),
        (48, 56),
        (56, 8),
        (64, 64),
        (72, 64),
        (64, 72),
        (80, 80),
        (136, 72),
        (200, 136),
    ];
    let mut n = 0usize;
    for (i, &(xsize, ysize)) in whole.iter().enumerate() {
        for kind in 0..KINDS {
            let big = xsize * ysize > 10_000;
            let picks: &[usize] = if big {
                &[n % 10]
            } else {
                &[n % 10, (n + 3) % 10]
            };
            for &d in picks {
                requests.push(map_request(
                    distances[d],
                    1.0,
                    xsize,
                    ysize,
                    [0, 0, xsize, ysize],
                    n % 4,
                    kind,
                    (i * 100 + kind) as u64,
                ));
                n += 1;
            }
        }
    }
    // Every distance and rescale on one photo-like image.
    for &d in &distances {
        requests.push(map_request(d, 1.0, 72, 40, [0, 0, 72, 40], 1, 0, 7));
    }
    for rescale in [0.5f32, 0.62, 2.0] {
        requests.push(map_request(1.0, rescale, 72, 40, [0, 0, 72, 40], 2, 0, 8));
    }
    // Images larger than the rect by a partial block: the export reads up
    // to four samples past the rect's last block, which these images hold
    // (a smaller margin would read outside the image in the original).
    for (k, &(xsize, ysize, rxs, rys)) in [
        (13usize, 13usize, 8usize, 8usize),
        (71, 45, 64, 40),
        (133, 77, 128, 72),
    ]
    .iter()
    .enumerate()
    {
        for kind in [0, 1, 4, 7] {
            requests.push(map_request(
                1.0,
                1.0,
                xsize,
                ysize,
                [0, 0, rxs, rys],
                (k + kind) % 4,
                kind,
                (900 + k * 10 + kind) as u64,
            ));
        }
    }
    // Rects inside a larger image.
    for (k, rect) in [
        [8usize, 8, 136, 128],
        [64, 0, 88, 144],
        [0, 64, 152, 80],
        [16, 24, 64, 40],
        [0, 0, 72, 64],
        [8, 0, 144, 144],
    ]
    .iter()
    .enumerate()
    {
        for kind in [0, 1, 7] {
            requests.push(map_request(
                1.0,
                1.0,
                152,
                144,
                *rect,
                (k + kind) % 4,
                kind,
                (700 + k * 10 + kind) as u64,
            ));
        }
    }
    check_map_through_dispatch(&requests);
}

/// Whether an `eaq-map` request's input contains a NaN or an infinity (the
/// exceptional inputs of rule O1): a word of the distance, the rescale or
/// the planes, or the row-padding word (pad kinds 1 and 2, a quiet NaN and
/// +inf), which the export's last vector of a row reads.
fn map_input_exceptional(request: &str) -> bool {
    let mut args = request.split_whitespace();
    assert_eq!(args.next(), Some("eaq-map"));
    let distance = float(&mut args);
    let rescale = float(&mut args);
    let _sizes: [usize; 6] = std::array::from_fn(|_| number(&mut args));
    let pad_kind: usize = number(&mut args);
    !distance.is_finite()
        || !rescale.is_finite()
        || !f32::from_bits(PAD_WORDS[pad_kind]).is_finite()
        || args.any(|word| !f32::from_bits(word.parse().unwrap()).is_finite())
}

/// Runs the requests with the scalar and then the SIMD encoder selection,
/// asserts that the scalar path differs somewhere, and compares the SIMD
/// observations with the original's target under rule O1
/// (`simd_common::reference_masks`, ADR-0041 section 3, #246): for a
/// request with NaN or infinity input, a word is compared only where the
/// original's two scalar reference builds for this target agree after
/// treating all NaNs as equal; every word of every other request is
/// compared. The same rule applies on NEON and AVX2.
///
/// Background: on AVX2, requests that put two or more NaNs into one block's
/// GammaModulation reduction (`SumOfLanes` over the 8x8 window) give
/// `aq_map` words that depend on which NaN survives, the compiler's operand
/// choice in the original's object, read as an integer by FastLog2f. The
/// owner's decision of 2026-11-15 excluded that class by name (46 of 286
/// requests); this gate applies the general rule O1 instead, as #246 asks.
/// Finding of 2026-11-20 (x86-64 under Rosetta 2): the original's two
/// x86-64 scalar references agree on every word of those requests (one
/// scalar `Add` order, no cross-lane reduction), so rule O1 compares the
/// words and the 46 requests fail on AVX2. Whether the class decision of
/// 2026-11-15 stands beside O1 is the owner's decision (port ADR 0004,
/// "`enc_adaptive_quantization`"); this gate does not narrow beyond O1.
fn check_map_through_dispatch(requests: &[String]) {
    let _lock = dispatch_lock();
    hwy::set_encoder_simd_for_test(Some(false));
    let scalar: Vec<String> = requests.iter().map(|r| observe_map(r)).collect();
    hwy::set_encoder_simd_for_test(Some(true));
    let simd: Vec<String> = requests.iter().map(|r| observe_map(r)).collect();
    hwy::set_encoder_simd_for_test(None);
    let differing = scalar.iter().zip(&simd).filter(|(a, b)| a != b).count();
    assert!(
        differing > 0,
        "eaq-map: the scalar and SIMD paths gave the same words everywhere"
    );
    println!(
        "eaq-map: the scalar path differs in {differing} of {} requests",
        requests.len()
    );
    let originals = simd_common::oracle(requests);
    let masks = simd_common::reference_masks(requests, map_input_exceptional);
    let exceptional = requests.iter().filter(|r| map_input_exceptional(r)).count();
    let (mut words, mut masked_requests, mut masked_words, mut masked_differing) = (0, 0, 0, 0);
    let mut failures = Vec::new();
    for (((request, rust), cpp), mask) in requests.iter().zip(&simd).zip(&originals).zip(&masks) {
        let flagged = mask
            .as_ref()
            .map_or(0, |m| m.iter().filter(|&&f| f).count());
        match simd_common::same_under_o1(rust, cpp, all_float, all_defined, mask.as_deref()) {
            Ok((compared, differ)) => {
                words += compared;
                masked_differing += differ;
            }
            Err(e) => failures.push(format!(
                "{e} (O1 words not compared: {flagged}; NaN or infinity input: {})\nrequest: {request}",
                map_input_exceptional(request)
            )),
        }
        masked_requests += usize::from(flagged > 0);
        masked_words += flagged;
    }
    println!(
        "eaq-map on {}: {} requests, {words} words matched in {} requests; NaN rule O1: \
         {exceptional} requests with NaN or infinity input, {masked_requests} of them with words \
         where the two scalar references disagree ({masked_words} words not compared, of which \
         {masked_differing} differ between the port and the original)",
        simd_common::target().name(),
        requests.len(),
        requests.len() - failures.len()
    );
    for failure in &failures {
        println!("eaq-map differs: {}", failure.lines().next().unwrap_or(""));
    }
    assert!(
        failures.is_empty(),
        "eaq-map: {} of {} requests differ from the original under rule O1; the first: {}",
        failures.len(),
        requests.len(),
        failures[0]
    );
}

// 8-lane structure ---------------------------------------------------------

/// The 8-lane structure of the SIMD AdaptiveQuantizationMap, exercised
/// natively on AArch64 without the oracle. `Neon8` is an 8-lane descriptor
/// made of two NEON vectors (as in tests/simd_vardct_lanes8.rs), with
/// SumOfLanes added. The lane count moves the boundary between ComputeTile's
/// vector loop and its scalar pixels and changes the reduction order, so
/// `aq_map` and `mask` legitimately differ from four lanes; `mask1x1` is
/// computed by scalar pixels only and must not. The test also shows that
/// every 8-lane load and store of GammaModulation, BlueModulation,
/// HfModulation (kMaskRight with one vector per row) and ComputeTile's loop
/// stays inside the planes over the parity test's geometry. The 8-lane
/// arithmetic itself is compared with the original's AVX2 build by the
/// parity test above, under Rosetta 2.
#[cfg(target_arch = "aarch64")]
mod lanes8 {
    use super::*;
    use core::arch::aarch64::*;
    use jxl::jxl::enc_adaptive_quantization_simd::adaptive_quantization_map_d;
    use jxl::jxl::hwy::{Neon, SimdF32};

    #[derive(Clone, Copy, Debug)]
    struct Neon8(Neon);

    macro_rules! halves {
        ($s:ident, $f:ident, $($a:ident),*) => {
            [$s.0.$f($($a[0]),*), $s.0.$f($($a[1]),*)]
        };
    }

    impl SimdF32 for Neon8 {
        type V = [float32x4_t; 2];
        type I = [int32x4_t; 2];
        type M = [uint32x4_t; 2];
        const LANES: usize = 8;
        const NAME: &'static str = "NEON x2";
        fn set(self, v: f32) -> Self::V {
            [self.0.set(v), self.0.set(v)]
        }
        fn zero(self) -> Self::V {
            [self.0.zero(), self.0.zero()]
        }
        fn load(self, src: &[f32]) -> Self::V {
            let src = &src[..8];
            [self.0.load(src), self.0.load(&src[4..])]
        }
        fn store(self, v: Self::V, dst: &mut [f32]) {
            let dst = &mut dst[..8];
            self.0.store(v[0], dst);
            self.0.store(v[1], &mut dst[4..]);
        }
        fn load_bits(self, src: &[u32]) -> Self::V {
            let src = &src[..8];
            [self.0.load_bits(src), self.0.load_bits(&src[4..])]
        }
        fn store_bits(self, v: Self::V, dst: &mut [u32]) {
            let dst = &mut dst[..8];
            self.0.store_bits(v[0], dst);
            self.0.store_bits(v[1], &mut dst[4..]);
        }
        fn store_i32(self, v: Self::I, dst: &mut [i32]) {
            let dst = &mut dst[..8];
            self.0.store_i32(v[0], dst);
            self.0.store_i32(v[1], &mut dst[4..]);
        }
        fn get_lane(self, v: Self::V) -> f32 {
            self.0.get_lane(v[0])
        }
        fn add(self, a: Self::V, b: Self::V) -> Self::V {
            halves!(self, add, a, b)
        }
        fn sub(self, a: Self::V, b: Self::V) -> Self::V {
            halves!(self, sub, a, b)
        }
        fn mul(self, a: Self::V, b: Self::V) -> Self::V {
            halves!(self, mul, a, b)
        }
        fn div(self, a: Self::V, b: Self::V) -> Self::V {
            halves!(self, div, a, b)
        }
        fn mul_add(self, m: Self::V, x: Self::V, a: Self::V) -> Self::V {
            halves!(self, mul_add, m, x, a)
        }
        fn neg_mul_add(self, m: Self::V, x: Self::V, a: Self::V) -> Self::V {
            halves!(self, neg_mul_add, m, x, a)
        }
        fn mul_sub(self, m: Self::V, x: Self::V, a: Self::V) -> Self::V {
            halves!(self, mul_sub, m, x, a)
        }
        fn neg_mul_sub(self, m: Self::V, x: Self::V, a: Self::V) -> Self::V {
            halves!(self, neg_mul_sub, m, x, a)
        }
        fn min(self, a: Self::V, b: Self::V) -> Self::V {
            halves!(self, min, a, b)
        }
        fn max(self, a: Self::V, b: Self::V) -> Self::V {
            halves!(self, max, a, b)
        }
        fn abs(self, v: Self::V) -> Self::V {
            halves!(self, abs, v)
        }
        fn neg(self, v: Self::V) -> Self::V {
            halves!(self, neg, v)
        }
        fn abs_diff(self, a: Self::V, b: Self::V) -> Self::V {
            halves!(self, abs_diff, a, b)
        }
        fn sqrt(self, v: Self::V) -> Self::V {
            halves!(self, sqrt, v)
        }
        fn approximate_reciprocal(self, v: Self::V) -> Self::V {
            halves!(self, approximate_reciprocal, v)
        }
        fn approximate_reciprocal_sqrt(self, v: Self::V) -> Self::V {
            halves!(self, approximate_reciprocal_sqrt, v)
        }
        fn floor(self, v: Self::V) -> Self::V {
            halves!(self, floor, v)
        }
        fn ceil(self, v: Self::V) -> Self::V {
            halves!(self, ceil, v)
        }
        fn round(self, v: Self::V) -> Self::V {
            halves!(self, round, v)
        }
        fn trunc(self, v: Self::V) -> Self::V {
            halves!(self, trunc, v)
        }
        fn zero_if_negative(self, v: Self::V) -> Self::V {
            halves!(self, zero_if_negative, v)
        }
        fn copy_sign(self, a: Self::V, b: Self::V) -> Self::V {
            halves!(self, copy_sign, a, b)
        }
        fn copy_sign_to_abs(self, a: Self::V, b: Self::V) -> Self::V {
            halves!(self, copy_sign_to_abs, a, b)
        }
        fn and(self, a: Self::V, b: Self::V) -> Self::V {
            halves!(self, and, a, b)
        }
        fn or(self, a: Self::V, b: Self::V) -> Self::V {
            halves!(self, or, a, b)
        }
        fn xor(self, a: Self::V, b: Self::V) -> Self::V {
            halves!(self, xor, a, b)
        }
        fn and_not(self, a: Self::V, b: Self::V) -> Self::V {
            halves!(self, and_not, a, b)
        }
        fn lt(self, a: Self::V, b: Self::V) -> Self::M {
            halves!(self, lt, a, b)
        }
        fn le(self, a: Self::V, b: Self::V) -> Self::M {
            halves!(self, le, a, b)
        }
        fn gt(self, a: Self::V, b: Self::V) -> Self::M {
            halves!(self, gt, a, b)
        }
        fn ge(self, a: Self::V, b: Self::V) -> Self::M {
            halves!(self, ge, a, b)
        }
        fn eq(self, a: Self::V, b: Self::V) -> Self::M {
            halves!(self, eq, a, b)
        }
        fn ne(self, a: Self::V, b: Self::V) -> Self::M {
            halves!(self, ne, a, b)
        }
        fn vec_from_mask(self, m: Self::M) -> Self::V {
            halves!(self, vec_from_mask, m)
        }
        fn if_then_else(self, m: Self::M, y: Self::V, n: Self::V) -> Self::V {
            halves!(self, if_then_else, m, y, n)
        }
        fn if_then_zero_else(self, m: Self::M, n: Self::V) -> Self::V {
            halves!(self, if_then_zero_else, m, n)
        }
        fn if_then_else_zero(self, m: Self::M, y: Self::V) -> Self::V {
            halves!(self, if_then_else_zero, m, y)
        }
        fn convert_to_i32(self, v: Self::V) -> Self::I {
            halves!(self, convert_to_i32, v)
        }
        fn convert_to_f32(self, v: Self::I) -> Self::V {
            halves!(self, convert_to_f32, v)
        }
        fn nearest_int(self, v: Self::V) -> Self::I {
            halves!(self, nearest_int, v)
        }
        fn bitcast_to_i32(self, v: Self::V) -> Self::I {
            halves!(self, bitcast_to_i32, v)
        }
        fn bitcast_to_f32(self, v: Self::I) -> Self::V {
            halves!(self, bitcast_to_f32, v)
        }
        fn add_i32(self, a: Self::I, b: Self::I) -> Self::I {
            halves!(self, add_i32, a, b)
        }
        fn sub_i32(self, a: Self::I, b: Self::I) -> Self::I {
            halves!(self, sub_i32, a, b)
        }
        fn min_i32(self, a: Self::I, b: Self::I) -> Self::I {
            halves!(self, min_i32, a, b)
        }
        fn max_i32(self, a: Self::I, b: Self::I) -> Self::I {
            halves!(self, max_i32, a, b)
        }
        fn lt_i32(self, a: Self::I, b: Self::I) -> Self::I {
            halves!(self, lt_i32, a, b)
        }
        fn eq_i32(self, a: Self::I, b: Self::I) -> Self::I {
            halves!(self, eq_i32, a, b)
        }
        fn gt_i32(self, a: Self::I, b: Self::I) -> Self::I {
            halves!(self, gt_i32, a, b)
        }
        fn shift_left_i32<const B: i32>(self, v: Self::I) -> Self::I {
            [
                self.0.shift_left_i32::<B>(v[0]),
                self.0.shift_left_i32::<B>(v[1]),
            ]
        }
        fn shift_right_i32<const B: i32>(self, v: Self::I) -> Self::I {
            [
                self.0.shift_right_i32::<B>(v[0]),
                self.0.shift_right_i32::<B>(v[1]),
            ]
        }
        fn shift_right_u32<const B: i32>(self, v: Self::I) -> Self::I {
            [
                self.0.shift_right_u32::<B>(v[0]),
                self.0.shift_right_u32::<B>(v[1]),
            ]
        }
        // Lane-crossing operations: AdaptiveQuantizationMap uses only SumOfLanes.
        // The halves added lane-wise, then NEON's reduction, broadcast: one
        // fixed order (not AVX2's; this descriptor is compared only with itself
        // and with NEON on lane-count-independent outputs).
        fn sum_of_lanes(self, v: Self::V) -> Self::V {
            let s = self.0.sum_of_lanes(self.0.add(v[0], v[1]));
            [s, s]
        }
        fn max_of_lanes(self, _: Self::V) -> Self::V {
            unimplemented!("lane-crossing")
        }
        fn min_of_lanes(self, _: Self::V) -> Self::V {
            unimplemented!("lane-crossing")
        }
        // Per 128-bit half, as AVX2's InterleaveLower/Upper per 128-bit block
        // (the 8x8 block transpose of the DCT uses them).
        fn interleave_lower(self, a: Self::V, b: Self::V) -> Self::V {
            halves!(self, interleave_lower, a, b)
        }
        fn interleave_upper(self, a: Self::V, b: Self::V) -> Self::V {
            halves!(self, interleave_upper, a, b)
        }
        fn store_interleaved2(self, _: Self::V, _: Self::V, _: &mut [f32]) {
            unimplemented!("lane-crossing")
        }
        fn store_interleaved3(self, _: Self::V, _: Self::V, _: Self::V, _: &mut [f32]) {
            unimplemented!("lane-crossing")
        }
        fn load_dup128(self, _: &[f32]) -> Self::V {
            unimplemented!("lane-crossing")
        }
        fn broadcast<const LANE: i32>(self, _: Self::V) -> Self::V {
            unimplemented!("lane-crossing")
        }
        fn store_demote_to_f16(self, _: Self::V, _: &mut [u16]) {
            unimplemented!("not used by AdaptiveQuantizationMap")
        }
        // Signed 32-bit operations of the modular and stages2 packages, each half
        // through the NEON implementation. AdaptiveQuantizationMap does not use them;
        // they are here because the layer trait requires them.
        fn set_i32(self, v: i32) -> Self::I {
            [self.0.set_i32(v), self.0.set_i32(v)]
        }
        fn load_i32(self, src: &[i32]) -> Self::I {
            let src = &src[..8];
            [self.0.load_i32(src), self.0.load_i32(&src[4..])]
        }
        fn and_i32(self, a: Self::I, b: Self::I) -> Self::I {
            halves!(self, and_i32, a, b)
        }
        fn xor_i32(self, a: Self::I, b: Self::I) -> Self::I {
            halves!(self, xor_i32, a, b)
        }
        fn abs_i32(self, v: Self::I) -> Self::I {
            halves!(self, abs_i32, v)
        }
        fn neg_i32(self, v: Self::I) -> Self::I {
            halves!(self, neg_i32, v)
        }
        fn ne_i32(self, a: Self::I, b: Self::I) -> Self::I {
            halves!(self, ne_i32, a, b)
        }
        fn if_then_else_i32(self, m: Self::I, yes: Self::I, no: Self::I) -> Self::I {
            halves!(self, if_then_else_i32, m, yes, no)
        }
        fn if_then_zero_else_i32(self, m: Self::I, no: Self::I) -> Self::I {
            halves!(self, if_then_zero_else_i32, m, no)
        }
        fn mul_i32(self, a: Self::I, b: Self::I) -> Self::I {
            halves!(self, mul_i32, a, b)
        }
        // Block-wise on AVX2, so one NEON operation per 128-bit half gives the
        // AVX2 block structure (Highway x86_256-inl.h at the pinned revision):
        // MulEven is `_mm256_mul_epi32`, which multiplies the low 32 bits of each
        // 64-bit element (lanes 0, 2 | 4, 6); MulOdd for 32-bit lanes is
        // `MulEven(DupOdd(a), DupOdd(b))` (x86_128-inl.h), DupOdd shuffling
        // within each block; InterleaveOdd (non-AVX3 path) is two
        // `_mm256_shuffle_ps`, each per 128-bit lane, giving a1 b1 a3 b3 | a5 b5
        // a7 b7; TableLookupBytes is `_mm256_shuffle_epi8`, which looks up within
        // each 128-bit lane. Per half, the byte-index rule stays NEON's
        // (`vqtbl1q_u8`), as every operation of this descriptor does.
        fn mul_even_i32(self, a: Self::I, b: Self::I) -> Self::I {
            halves!(self, mul_even_i32, a, b)
        }
        fn mul_odd_i32(self, a: Self::I, b: Self::I) -> Self::I {
            halves!(self, mul_odd_i32, a, b)
        }
        fn interleave_odd_i32(self, a: Self::I, b: Self::I) -> Self::I {
            halves!(self, interleave_odd_i32, a, b)
        }
        fn table_lookup_bytes(self, bytes: Self::I, from: Self::I) -> Self::I {
            halves!(self, table_lookup_bytes, bytes, from)
        }
        // HWY_CAPPED(float, 4): one NEON vector, as AVX2's 128-bit vector of
        // the 4-column transforms.
        type Q = <Neon as SimdF32>::V;
        fn q_load(self, src: &[f32]) -> Self::Q {
            self.0.load(src)
        }
        fn q_store(self, v: Self::Q, dst: &mut [f32]) {
            self.0.store(v, dst)
        }
        fn q_set(self, v: f32) -> Self::Q {
            self.0.set(v)
        }
        fn q_add(self, a: Self::Q, b: Self::Q) -> Self::Q {
            self.0.add(a, b)
        }
        fn q_sub(self, a: Self::Q, b: Self::Q) -> Self::Q {
            self.0.sub(a, b)
        }
        fn q_mul(self, a: Self::Q, b: Self::Q) -> Self::Q {
            self.0.mul(a, b)
        }
        fn q_mul_add(self, mul: Self::Q, x: Self::Q, add: Self::Q) -> Self::Q {
            self.0.mul_add(mul, x, add)
        }
        fn q_neg_mul_add(self, mul: Self::Q, x: Self::Q, add: Self::Q) -> Self::Q {
            self.0.neg_mul_add(mul, x, add)
        }
    }

    fn run<D: SimdF32>(d: D, request: &str) -> (ImageF, ImageF, ImageF) {
        let mut args = request.split_whitespace();
        assert_eq!(args.next(), Some("eaq-map"));
        let distance = float(&mut args);
        let rescale = float(&mut args);
        let xsize: usize = number(&mut args);
        let ysize: usize = number(&mut args);
        let rect = Rect::new(
            number(&mut args),
            number(&mut args),
            number(&mut args),
            number(&mut args),
        );
        let pad = f32::from_bits(PAD_WORDS[number::<usize>(&mut args)]);
        let mut opsin = Image3F::create(xsize, ysize).unwrap();
        let stride = opsin.pixels_per_row() as usize;
        for c in 0..3 {
            for y in 0..ysize {
                let row = opsin.plane_row_mut(c, y);
                for v in &mut row[..xsize] {
                    *v = float(&mut args);
                }
                for v in &mut row[xsize..stride] {
                    *v = pad;
                }
            }
        }
        // InitialQuantField's scale: kAcQuant / distance * rescale.
        let scale = 0.765f32 / distance * rescale;
        let mut mask = ImageF::default();
        let mut mask1x1 = ImageF::default();
        let aq = adaptive_quantization_map_d(
            d,
            distance,
            &opsin,
            rect,
            scale,
            None,
            &mut mask,
            &mut mask1x1,
        )
        .unwrap();
        (aq, mask, mask1x1)
    }

    fn words(plane: &ImageF) -> Vec<u32> {
        (0..plane.ysize())
            .flat_map(|y| plane.row(y)[..plane.xsize()].iter().map(|v| v.to_bits()))
            .collect()
    }

    #[test]
    fn eight_lane_adaptive_quantization_map_keeps_the_scalar_pixels_and_bounds() {
        let n4 = Neon::new();
        let n8 = Neon8(n4);
        let mut requests = Vec::new();
        for (i, &(xsize, ysize, rect)) in [
            (8usize, 8usize, [0usize, 0, 8, 8]),
            (16, 8, [0, 0, 16, 8]),
            (24, 24, [0, 0, 24, 24]),
            (40, 16, [0, 0, 40, 16]),
            (56, 8, [0, 0, 56, 8]),
            (72, 64, [0, 0, 72, 64]),
            (136, 72, [0, 0, 136, 72]),
            (13, 13, [0, 0, 8, 8]),
            (71, 45, [0, 0, 64, 40]),
            (152, 144, [8, 8, 136, 128]),
            (152, 144, [64, 0, 88, 144]),
            (152, 144, [16, 24, 64, 40]),
        ]
        .iter()
        .enumerate()
        {
            for kind in 0..KINDS {
                requests.push(map_request(
                    [0.5f32, 1.0, 2.9][kind % 3],
                    1.0,
                    xsize,
                    ysize,
                    rect,
                    (i + kind) % 4,
                    kind,
                    (i * 100 + kind) as u64,
                ));
            }
        }
        let same = |a: &[u32], b: &[u32]| {
            a.len() == b.len()
                && a.iter().zip(b).all(|(&x, &y)| {
                    x == y || (f32::from_bits(x).is_nan() && f32::from_bits(y).is_nan())
                })
        };
        let mut differing = 0;
        for request in &requests {
            let (aq4, mask4, m4) = run(n4, request);
            let (aq8, mask8, m8) = run(n8, request);
            assert!(
                same(&words(&m4), &words(&m8)),
                "mask1x1 depends on the lane count\nrequest: {request}"
            );
            assert_eq!((aq4.xsize(), aq4.ysize()), (aq8.xsize(), aq8.ysize()));
            assert_eq!(
                (mask4.xsize(), mask4.ysize()),
                (mask8.xsize(), mask8.ysize())
            );
            if !same(&words(&aq4), &words(&aq8)) || !same(&words(&mask4), &words(&mask8)) {
                differing += 1;
            }
        }
        println!(
            "eaq-map with 8 NEON lanes: {} requests, mask1x1 equal to 4 lanes, \
             aq_map or mask differ (lane-count dependent) in {differing}",
            requests.len()
        );
    }
}
