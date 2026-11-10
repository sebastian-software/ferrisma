//! Cases of the decoder DSP of `src/dsp/dec.rs` whose pointers
//! `VP8DspInitSSE2` and `VP8DspInitSSE41` assign (WP18): the inverse
//! transform, the loop filters and the intra predictors, at the C level and
//! each x86 level that selects another variant.

use std::hint::black_box;

use webp::dsp::dec::{
    vp8_pred_chroma8, vp8_pred_luma4, vp8_pred_luma16, vp8_simple_h_filter16,
    vp8_simple_h_filter16i, vp8_simple_v_filter16, vp8_simple_v_filter16i, vp8_transform,
    vp8h_filter8, vp8h_filter8i, vp8h_filter16, vp8h_filter16i, vp8v_filter8, vp8v_filter8i,
    vp8v_filter16, vp8v_filter16i,
};

use crate::x86::{Level, SSE2, SSE41, levels};
use crate::{Case, Rng};

const MODULE: &str = "dec";
const BPS: usize = 32;
/// The luma plane of the filter cases: a stride and rows around the edges.
const STRIDE: usize = 64;

/// Coefficients within ±2048, as a valid stream produces them.
fn coefficients(rng: &mut Rng) -> Vec<i16> {
    (0..64)
        .map(|_| (rng.next_u64() % 4097) as i16 - 2048)
        .collect()
}

/// A smooth plane with a step, so that the filters take their branches.
fn plane(rng: &mut Rng, size: usize) -> Vec<u8> {
    (0..size)
        .map(|i| {
            let base = if (i % STRIDE) < 20 { 90 } else { 110 };
            base + (rng.next_u64() % 5) as u8
        })
        .collect()
}

fn transform_cases(cases: &mut Vec<Case>) {
    for (function, do_two) in [("vp8_transform(two)", true), ("vp8_transform(one)", false)] {
        for level in levels(&[SSE2]) {
            let mut rng = Rng::new(1);
            let coeffs = coefficients(&mut rng);
            let mut dst = rng.bytes(8 * BPS);
            cases.push(level.case(MODULE, function, move || {
                vp8_transform(black_box(&coeffs), black_box(&mut dst), do_two);
            }));
        }
    }
}

type Simple = fn(&mut [u8], usize, i32, i32);
type Luma = fn(&mut [u8], usize, i32, i32, i32, i32);
type Chroma = fn(&mut [u8], usize, &mut [u8], usize, i32, i32, i32, i32);

fn filter_cases(cases: &mut Vec<Case>) {
    let simple: [(&'static str, Simple); 4] = [
        ("vp8_simple_v_filter16", vp8_simple_v_filter16),
        ("vp8_simple_h_filter16", vp8_simple_h_filter16),
        ("vp8_simple_v_filter16i", vp8_simple_v_filter16i),
        ("vp8_simple_h_filter16i", vp8_simple_h_filter16i),
    ];
    for (function, filter) in simple {
        for level in levels(&[SSE2]) {
            let mut rng = Rng::new(2);
            let mut buf = plane(&mut rng, STRIDE * 26);
            cases.push(level.case(MODULE, function, move || {
                filter(black_box(&mut buf), 5 * STRIDE + 8, STRIDE as i32, 40);
            }));
        }
    }
    let luma: [(&'static str, Luma); 4] = [
        ("vp8v_filter16", vp8v_filter16),
        ("vp8h_filter16", vp8h_filter16),
        ("vp8v_filter16i", vp8v_filter16i),
        ("vp8h_filter16i", vp8h_filter16i),
    ];
    for (function, filter) in luma {
        for level in levels(&[SSE2]) {
            let mut rng = Rng::new(3);
            let mut buf = plane(&mut rng, STRIDE * 26);
            cases.push(level.case(MODULE, function, move || {
                filter(
                    black_box(&mut buf),
                    5 * STRIDE + 8,
                    STRIDE as i32,
                    40,
                    20,
                    1,
                );
            }));
        }
    }
    let chroma: [(&'static str, Chroma); 4] = [
        ("vp8v_filter8", vp8v_filter8),
        ("vp8h_filter8", vp8h_filter8),
        ("vp8v_filter8i", vp8v_filter8i),
        ("vp8h_filter8i", vp8h_filter8i),
    ];
    for (function, filter) in chroma {
        for level in levels(&[SSE2]) {
            let mut rng = Rng::new(4);
            let mut u = plane(&mut rng, STRIDE * 18);
            let mut v = plane(&mut rng, STRIDE * 18);
            let pos = 5 * STRIDE + 8;
            cases.push(level.case(MODULE, function, move || {
                filter(
                    black_box(&mut u),
                    pos,
                    black_box(&mut v),
                    pos,
                    STRIDE as i32,
                    40,
                    20,
                    1,
                );
            }));
        }
    }
}

type Predictor = fn(i32, &mut [u8], usize);

/// The predictor modes with an SSE2 (or SSE4.1) variant, with their names.
const LUMA4: [(i32, &str); 6] = [
    (1, "vp8_pred_luma4[TM4]"),
    (2, "vp8_pred_luma4[VE4]"),
    (4, "vp8_pred_luma4[RD4]"),
    (5, "vp8_pred_luma4[VR4]"),
    (6, "vp8_pred_luma4[LD4]"),
    (7, "vp8_pred_luma4[VL4]"),
];
const LUMA16: [(i32, &str); 7] = [
    (0, "vp8_pred_luma16[DC16]"),
    (1, "vp8_pred_luma16[TM16]"),
    (2, "vp8_pred_luma16[VE16]"),
    (3, "vp8_pred_luma16[HE16]"),
    (4, "vp8_pred_luma16[DC16NoTop]"),
    (5, "vp8_pred_luma16[DC16NoLeft]"),
    (6, "vp8_pred_luma16[DC16NoTopLeft]"),
];
const CHROMA8: [(i32, &str); 6] = [
    (0, "vp8_pred_chroma8[DC8uv]"),
    (1, "vp8_pred_chroma8[TM8uv]"),
    (2, "vp8_pred_chroma8[VE8uv]"),
    (4, "vp8_pred_chroma8[DC8uvNoTop]"),
    (5, "vp8_pred_chroma8[DC8uvNoLeft]"),
    (6, "vp8_pred_chroma8[DC8uvNoTopLeft]"),
];

fn predictor_cases(cases: &mut Vec<Case>) {
    let families: [(Predictor, &[(i32, &'static str)]); 3] = [
        (vp8_pred_luma4, &LUMA4),
        (vp8_pred_luma16, &LUMA16),
        (vp8_pred_chroma8, &CHROMA8),
    ];
    for (predict, modes) in families {
        for &(mode, function) in modes {
            // HE16 is the only predictor with an SSE4.1 variant.
            let simd: &[Level] = if function.ends_with("[HE16]") {
                &[SSE2, SSE41]
            } else {
                &[SSE2]
            };
            for level in levels(simd) {
                let mut rng = Rng::new(5);
                let mut buf = rng.bytes(BPS * 20);
                cases.push(level.case(MODULE, function, move || {
                    predict(mode, black_box(&mut buf), BPS + 8);
                }));
            }
        }
    }
}

pub fn cases() -> Vec<Case> {
    let mut cases = Vec::new();
    transform_cases(&mut cases);
    filter_cases(&mut cases);
    predictor_cases(&mut cases);
    cases
}
