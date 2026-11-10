//! Benchmark cases of `src/dsp/lossless_neon.rs` (WP20): the C translation
//! and, with the `simd` feature on 64-bit ARM, the NEON variant of each
//! pointer that `VP8LDspInitNEON` assigns, on rows of 256 pixels in the
//! decoder's layout (the row above the output, the output in place).

use std::hint::black_box;

use webp::dsp::lossless::{self as c, VP8LMultipliers, VP8LPredictorFunc};
#[cfg(all(feature = "simd", target_arch = "aarch64"))]
/// The NEON level of `src/dsp/lossless_neon.rs`, reached through the dispatch
/// functions, which select its crate-private entry points with `simd` on
/// 64-bit ARM (the residual costs while the `VP8GetCPUInfo` query answers
/// `kNEON`, as it does by default); named as the entry points.
mod neon {
    pub use webp::dsp::lossless::vp8l_add_green_to_blue_and_red as add_green_to_blue_and_red;
    pub use webp::dsp::lossless::vp8l_convert_bgra_to_bgr as convert_bgra_to_bgr;
    pub use webp::dsp::lossless::vp8l_convert_bgra_to_rgb as convert_bgra_to_rgb;
    pub use webp::dsp::lossless::vp8l_convert_bgra_to_rgba as convert_bgra_to_rgba;
    pub fn predictor13(left: &[u32], left_pos: usize, top: &[u32], top_pos: usize) -> u32 {
        webp::dsp::lossless::vp8l_predictors(13, left, left_pos, top, top_pos)
    }
    pub fn predictor5(left: &[u32], left_pos: usize, top: &[u32], top_pos: usize) -> u32 {
        webp::dsp::lossless::vp8l_predictors(5, left, left_pos, top, top_pos)
    }
    pub fn predictor6(left: &[u32], left_pos: usize, top: &[u32], top_pos: usize) -> u32 {
        webp::dsp::lossless::vp8l_predictors(6, left, left_pos, top, top_pos)
    }
    pub fn predictor7(left: &[u32], left_pos: usize, top: &[u32], top_pos: usize) -> u32 {
        webp::dsp::lossless::vp8l_predictors(7, left, left_pos, top, top_pos)
    }
    pub fn predictor_add0(
        data: &mut [u32],
        in_pos: usize,
        upper_pos: usize,
        num_pixels: i32,
        out_pos: usize,
    ) {
        webp::dsp::lossless::vp8l_predictors_add(0, data, in_pos, upper_pos, num_pixels, out_pos);
    }
    pub fn predictor_add1(
        data: &mut [u32],
        in_pos: usize,
        upper_pos: usize,
        num_pixels: i32,
        out_pos: usize,
    ) {
        webp::dsp::lossless::vp8l_predictors_add(1, data, in_pos, upper_pos, num_pixels, out_pos);
    }
    pub fn predictor_add10(
        data: &mut [u32],
        in_pos: usize,
        upper_pos: usize,
        num_pixels: i32,
        out_pos: usize,
    ) {
        webp::dsp::lossless::vp8l_predictors_add(10, data, in_pos, upper_pos, num_pixels, out_pos);
    }
    pub fn predictor_add11(
        data: &mut [u32],
        in_pos: usize,
        upper_pos: usize,
        num_pixels: i32,
        out_pos: usize,
    ) {
        webp::dsp::lossless::vp8l_predictors_add(11, data, in_pos, upper_pos, num_pixels, out_pos);
    }
    pub fn predictor_add12(
        data: &mut [u32],
        in_pos: usize,
        upper_pos: usize,
        num_pixels: i32,
        out_pos: usize,
    ) {
        webp::dsp::lossless::vp8l_predictors_add(12, data, in_pos, upper_pos, num_pixels, out_pos);
    }
    pub fn predictor_add13(
        data: &mut [u32],
        in_pos: usize,
        upper_pos: usize,
        num_pixels: i32,
        out_pos: usize,
    ) {
        webp::dsp::lossless::vp8l_predictors_add(13, data, in_pos, upper_pos, num_pixels, out_pos);
    }
    pub fn predictor_add2(
        data: &mut [u32],
        in_pos: usize,
        upper_pos: usize,
        num_pixels: i32,
        out_pos: usize,
    ) {
        webp::dsp::lossless::vp8l_predictors_add(2, data, in_pos, upper_pos, num_pixels, out_pos);
    }
    pub fn predictor_add3(
        data: &mut [u32],
        in_pos: usize,
        upper_pos: usize,
        num_pixels: i32,
        out_pos: usize,
    ) {
        webp::dsp::lossless::vp8l_predictors_add(3, data, in_pos, upper_pos, num_pixels, out_pos);
    }
    pub fn predictor_add4(
        data: &mut [u32],
        in_pos: usize,
        upper_pos: usize,
        num_pixels: i32,
        out_pos: usize,
    ) {
        webp::dsp::lossless::vp8l_predictors_add(4, data, in_pos, upper_pos, num_pixels, out_pos);
    }
    pub fn predictor_add5(
        data: &mut [u32],
        in_pos: usize,
        upper_pos: usize,
        num_pixels: i32,
        out_pos: usize,
    ) {
        webp::dsp::lossless::vp8l_predictors_add(5, data, in_pos, upper_pos, num_pixels, out_pos);
    }
    pub fn predictor_add6(
        data: &mut [u32],
        in_pos: usize,
        upper_pos: usize,
        num_pixels: i32,
        out_pos: usize,
    ) {
        webp::dsp::lossless::vp8l_predictors_add(6, data, in_pos, upper_pos, num_pixels, out_pos);
    }
    pub fn predictor_add7(
        data: &mut [u32],
        in_pos: usize,
        upper_pos: usize,
        num_pixels: i32,
        out_pos: usize,
    ) {
        webp::dsp::lossless::vp8l_predictors_add(7, data, in_pos, upper_pos, num_pixels, out_pos);
    }
    pub fn predictor_add8(
        data: &mut [u32],
        in_pos: usize,
        upper_pos: usize,
        num_pixels: i32,
        out_pos: usize,
    ) {
        webp::dsp::lossless::vp8l_predictors_add(8, data, in_pos, upper_pos, num_pixels, out_pos);
    }
    pub fn predictor_add9(
        data: &mut [u32],
        in_pos: usize,
        upper_pos: usize,
        num_pixels: i32,
        out_pos: usize,
    ) {
        webp::dsp::lossless::vp8l_predictors_add(9, data, in_pos, upper_pos, num_pixels, out_pos);
    }
    pub use webp::dsp::lossless::vp8l_transform_color_inverse as transform_color_inverse;
}

use super::{Case, Rng};

/// Pixels per row.
const WIDTH: usize = 256;

/// A batch predictor: `(data, in, upper, num_pixels, out)`.
type PredictorAdd = fn(&mut [u32], usize, usize, i32, usize);

/// A converter from BGRA pixels to bytes.
type Converter = fn(&[u32], i32, &mut [u8]);

/// The function names of `VP8LPredictorsAdd[0..14]`.
const PREDICTORS_ADD: [&str; 14] = [
    "VP8LPredictorsAdd[0]",
    "VP8LPredictorsAdd[1]",
    "VP8LPredictorsAdd[2]",
    "VP8LPredictorsAdd[3]",
    "VP8LPredictorsAdd[4]",
    "VP8LPredictorsAdd[5]",
    "VP8LPredictorsAdd[6]",
    "VP8LPredictorsAdd[7]",
    "VP8LPredictorsAdd[8]",
    "VP8LPredictorsAdd[9]",
    "VP8LPredictorsAdd[10]",
    "VP8LPredictorsAdd[11]",
    "VP8LPredictorsAdd[12]",
    "VP8LPredictorsAdd[13]",
];

/// `count` seeded pixels.
fn words(rng: &mut Rng, count: usize) -> Vec<u32> {
    rng.bytes(4 * count)
        .as_chunks::<4>()
        .0
        .iter()
        .map(|b| u32::from_le_bytes(*b))
        .collect()
}

/// A case of a batch predictor on the second of two rows, in place.
fn predictor_add_case(
    rng: &mut Rng,
    function: &'static str,
    variant: &'static str,
    run: impl Fn(&mut [u32], usize, usize, i32, usize) + 'static,
) -> Case {
    let mut data = words(rng, 2 * WIDTH + 1);
    Case {
        module: "lossless",
        function,
        variant,
        run: Box::new(move || {
            // `upper` is row 0 from column 1, `in` and `out` row 1 from column 1.
            run(
                black_box(&mut data),
                WIDTH + 2,
                1,
                (WIDTH - 1) as i32,
                WIDTH + 2,
            );
        }),
    }
}

/// A case of a one-pixel predictor over a row.
fn predictor_case(
    rng: &mut Rng,
    function: &'static str,
    variant: &'static str,
    predictor: VP8LPredictorFunc,
) -> Case {
    let left = words(rng, WIDTH);
    let top = words(rng, WIDTH + 2);
    Case {
        module: "lossless",
        function,
        variant,
        run: Box::new(move || {
            let mut sum = 0_u32;
            for x in 0..WIDTH {
                sum = sum.wrapping_add(predictor(black_box(&left), x, &top, x + 1));
            }
            black_box(sum);
        }),
    }
}

/// A case of an in-place transform of a row.
fn transform_case(
    rng: &mut Rng,
    function: &'static str,
    variant: &'static str,
    run: impl Fn(&mut [u32]) + 'static,
) -> Case {
    let mut data = words(rng, WIDTH);
    Case {
        module: "lossless",
        function,
        variant,
        run: Box::new(move || run(black_box(&mut data))),
    }
}

/// A case of a converter of a row.
fn convert_case(
    rng: &mut Rng,
    function: &'static str,
    variant: &'static str,
    convert: Converter,
) -> Case {
    let src = words(rng, WIDTH);
    let mut dst = vec![0_u8; 4 * WIDTH];
    Case {
        module: "lossless",
        function,
        variant,
        run: Box::new(move || convert(black_box(&src), WIDTH as i32, black_box(&mut dst))),
    }
}

/// The NEON entry point of `VP8LPredictorsAdd[mode]`.
#[cfg(all(feature = "simd", target_arch = "aarch64"))]
fn neon_predictor_add(mode: usize) -> Option<PredictorAdd> {
    let table: [PredictorAdd; 14] = [
        neon::predictor_add0,
        neon::predictor_add1,
        neon::predictor_add2,
        neon::predictor_add3,
        neon::predictor_add4,
        neon::predictor_add5,
        neon::predictor_add6,
        neon::predictor_add7,
        neon::predictor_add8,
        neon::predictor_add9,
        neon::predictor_add10,
        neon::predictor_add11,
        neon::predictor_add12,
        neon::predictor_add13,
    ];
    Some(table[mode])
}

/// No NEON variant without the `simd` feature on 64-bit ARM.
#[cfg(not(all(feature = "simd", target_arch = "aarch64")))]
fn neon_predictor_add(_mode: usize) -> Option<PredictorAdd> {
    None
}

/// The NEON entry point of `VP8LPredictors[mode]` for the modes 5, 6, 7
/// and 13.
#[cfg(all(feature = "simd", target_arch = "aarch64"))]
fn neon_predictor(mode: usize) -> Option<VP8LPredictorFunc> {
    match mode {
        5 => Some(neon::predictor5),
        6 => Some(neon::predictor6),
        7 => Some(neon::predictor7),
        13 => Some(neon::predictor13),
        _ => None,
    }
}

/// No NEON variant without the `simd` feature on 64-bit ARM.
#[cfg(not(all(feature = "simd", target_arch = "aarch64")))]
fn neon_predictor(_mode: usize) -> Option<VP8LPredictorFunc> {
    None
}

/// The NEON entry point of the converter `index` (RGBA, BGR, RGB).
#[cfg(all(feature = "simd", target_arch = "aarch64"))]
fn neon_converter(index: usize) -> Option<Converter> {
    let table: [Converter; 3] = [
        neon::convert_bgra_to_rgba,
        neon::convert_bgra_to_bgr,
        neon::convert_bgra_to_rgb,
    ];
    Some(table[index])
}

/// No NEON variant without the `simd` feature on 64-bit ARM.
#[cfg(not(all(feature = "simd", target_arch = "aarch64")))]
fn neon_converter(_index: usize) -> Option<Converter> {
    None
}

/// The C and NEON cases of each pointer that `VP8LDspInitNEON` assigns.
pub fn cases() -> Vec<Case> {
    let mut rng = Rng::new(0x2020_1000);
    let mut cases = Vec::new();
    for (mode, function) in PREDICTORS_ADD.into_iter().enumerate() {
        cases.push(predictor_add_case(
            &mut rng,
            function,
            "C",
            move |d, i, u, n, o| {
                c::vp8l_predictors_add_c(mode, d, i, u, n, o);
            },
        ));
        if let Some(predictor_add) = neon_predictor_add(mode) {
            cases.push(predictor_add_case(
                &mut rng,
                function,
                "NEON",
                predictor_add,
            ));
        }
    }
    let one_pixel: [(usize, &'static str, VP8LPredictorFunc); 4] = [
        (5, "VP8LPredictors[5]", c::vp8l_predictor5_c),
        (6, "VP8LPredictors[6]", c::vp8l_predictor6_c),
        (7, "VP8LPredictors[7]", c::vp8l_predictor7_c),
        (13, "VP8LPredictors[13]", c::vp8l_predictor13_c),
    ];
    for (mode, function, predictor) in one_pixel {
        cases.push(predictor_case(&mut rng, function, "C", predictor));
        if let Some(predictor) = neon_predictor(mode) {
            cases.push(predictor_case(&mut rng, function, "NEON", predictor));
        }
    }
    cases.push(transform_case(
        &mut rng,
        "VP8LAddGreenToBlueAndRed",
        "C",
        |d| {
            c::vp8l_add_green_to_blue_and_red_c(d, 0, WIDTH as i32, 0);
        },
    ));
    #[cfg(all(feature = "simd", target_arch = "aarch64"))]
    cases.push(transform_case(
        &mut rng,
        "VP8LAddGreenToBlueAndRed",
        "NEON",
        |d| {
            neon::add_green_to_blue_and_red(d, 0, WIDTH as i32, 0);
        },
    ));
    let m = VP8LMultipliers {
        green_to_red: 0x35,
        green_to_blue: 0xc4,
        red_to_blue: 0x7b,
    };
    cases.push(transform_case(
        &mut rng,
        "VP8LTransformColorInverse",
        "C",
        move |d| {
            c::vp8l_transform_color_inverse_c(&m, d, 0, WIDTH as i32, 0);
        },
    ));
    #[cfg(all(feature = "simd", target_arch = "aarch64"))]
    cases.push(transform_case(
        &mut rng,
        "VP8LTransformColorInverse",
        "NEON",
        move |d| {
            neon::transform_color_inverse(&m, d, 0, WIDTH as i32, 0);
        },
    ));
    let converters: [(&'static str, Converter); 3] = [
        ("VP8LConvertBGRAToRGBA", c::vp8l_convert_bgra_to_rgba_c),
        ("VP8LConvertBGRAToBGR", c::vp8l_convert_bgra_to_bgr_c),
        ("VP8LConvertBGRAToRGB", c::vp8l_convert_bgra_to_rgb_c),
    ];
    for (index, (function, convert)) in converters.into_iter().enumerate() {
        cases.push(convert_case(&mut rng, function, "C", convert));
        if let Some(convert) = neon_converter(index) {
            cases.push(convert_case(&mut rng, function, "NEON", convert));
        }
    }
    cases
}
