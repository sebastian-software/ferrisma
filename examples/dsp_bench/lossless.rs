//! Cases of `src/dsp/lossless.rs` whose pointers `VP8LDspInitSSE2`, `SSE41`
//! and `AVX2` assign (WP18): the predictors, the batch predictors, the green
//! and color inverse transforms and the BGRA converters, over a row of 1024
//! pixels.

use std::hint::black_box;

use webp::dsp::lossless::{
    VP8LMultipliers, vp8l_add_green_to_blue_and_red, vp8l_convert_bgra_to_bgr,
    vp8l_convert_bgra_to_rgb, vp8l_convert_bgra_to_rgb565, vp8l_convert_bgra_to_rgba,
    vp8l_convert_bgra_to_rgba4444, vp8l_predictors, vp8l_predictors_add,
    vp8l_transform_color_inverse,
};

use crate::x86::{AVX2, Level, SSE2, SSE41, levels};
use crate::{Case, Rng};

const MODULE: &str = "lossless";
const ROW: usize = 1024;

/// The names of the 14 predictors and batch predictors.
const PREDICTORS: [&str; 14] = [
    "vp8l_predictors[0]",
    "vp8l_predictors[1]",
    "vp8l_predictors[2]",
    "vp8l_predictors[3]",
    "vp8l_predictors[4]",
    "vp8l_predictors[5]",
    "vp8l_predictors[6]",
    "vp8l_predictors[7]",
    "vp8l_predictors[8]",
    "vp8l_predictors[9]",
    "vp8l_predictors[10]",
    "vp8l_predictors[11]",
    "vp8l_predictors[12]",
    "vp8l_predictors[13]",
];
const PREDICTORS_ADD: [&str; 14] = [
    "vp8l_predictors_add[0]",
    "vp8l_predictors_add[1]",
    "vp8l_predictors_add[2]",
    "vp8l_predictors_add[3]",
    "vp8l_predictors_add[4]",
    "vp8l_predictors_add[5]",
    "vp8l_predictors_add[6]",
    "vp8l_predictors_add[7]",
    "vp8l_predictors_add[8]",
    "vp8l_predictors_add[9]",
    "vp8l_predictors_add[10]",
    "vp8l_predictors_add[11]",
    "vp8l_predictors_add[12]",
    "vp8l_predictors_add[13]",
];

fn pixels(rng: &mut Rng, count: usize) -> Vec<u32> {
    (0..count).map(|_| rng.next_u64() as u32).collect()
}

pub fn cases() -> Vec<Case> {
    let mut cases = Vec::new();
    // VP8LPredictors[5..14] have SSE2 variants.
    for (mode, &function) in PREDICTORS.iter().enumerate().skip(5) {
        for level in levels(&[SSE2]) {
            let mut rng = Rng::new(61);
            let data = pixels(&mut rng, 2 * ROW);
            cases.push(level.case(MODULE, function, move || {
                black_box(vp8l_predictors(mode, black_box(&data), ROW + 1, &data, 2));
            }));
        }
    }
    // VP8LPredictorsAdd: SSE2 for all, AVX2 for 0 to 4 and 8 to 12.
    for (mode, &function) in PREDICTORS_ADD.iter().enumerate() {
        let simd: &[Level] = if matches!(mode, 0..=4 | 8..=12) {
            &[SSE2, AVX2]
        } else {
            &[SSE2]
        };
        for level in levels(simd) {
            let mut rng = Rng::new(62);
            let mut data = pixels(&mut rng, 3 * ROW + 1);
            cases.push(level.case(MODULE, function, move || {
                vp8l_predictors_add(
                    mode,
                    black_box(&mut data),
                    2 * ROW + 1,
                    1,
                    ROW as i32,
                    2 * ROW + 1,
                );
            }));
        }
    }
    for level in levels(&[SSE2, AVX2]) {
        let mut rng = Rng::new(63);
        let mut data = pixels(&mut rng, 2 * ROW);
        cases.push(
            level.case(MODULE, "vp8l_add_green_to_blue_and_red", move || {
                vp8l_add_green_to_blue_and_red(black_box(&mut data), 0, ROW as i32, ROW);
            }),
        );
    }
    for level in levels(&[SSE2, SSE41, AVX2]) {
        let mut rng = Rng::new(64);
        let mut data = pixels(&mut rng, 2 * ROW);
        let m = VP8LMultipliers {
            green_to_red: 0x35,
            green_to_blue: 0xc2,
            red_to_blue: 0x17,
        };
        cases.push(level.case(MODULE, "vp8l_transform_color_inverse", move || {
            vp8l_transform_color_inverse(&m, black_box(&mut data), 0, ROW as i32, ROW);
        }));
    }
    type Convert = fn(&[u32], i32, &mut [u8]);
    let converters: [(&'static str, Convert, usize, &[Level]); 5] = [
        (
            "vp8l_convert_bgra_to_rgb",
            vp8l_convert_bgra_to_rgb,
            3,
            &[SSE2, SSE41],
        ),
        (
            "vp8l_convert_bgra_to_rgba",
            vp8l_convert_bgra_to_rgba,
            4,
            &[SSE2, AVX2],
        ),
        (
            "vp8l_convert_bgra_to_rgba4444",
            vp8l_convert_bgra_to_rgba4444,
            2,
            &[SSE2],
        ),
        (
            "vp8l_convert_bgra_to_rgb565",
            vp8l_convert_bgra_to_rgb565,
            2,
            &[SSE2],
        ),
        (
            "vp8l_convert_bgra_to_bgr",
            vp8l_convert_bgra_to_bgr,
            3,
            &[SSE2, SSE41],
        ),
    ];
    for (function, convert, bytes, simd) in converters {
        for level in levels(simd) {
            let mut rng = Rng::new(65);
            let src = pixels(&mut rng, ROW);
            let mut dst = vec![0; bytes * ROW];
            cases.push(level.case(MODULE, function, move || {
                convert(black_box(&src), ROW as i32, black_box(&mut dst));
            }));
        }
    }
    cases
}
