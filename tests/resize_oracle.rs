//! Area and Lanczos3 against oracles that are independent of the engine's
//! own `f64` reference (issue #18). See `tests/fixtures/resize/README.md`.
//!
//! Two oracles:
//! 1. `ImageMagick` 7.1.2-35 `Q16-HDRI` linear-light results, retained as
//!    float32 fixtures (`tests/fixtures/resize/*.f32`).
//! 2. `support/resize_oracle.rs`, a separately written direct 2D summation
//!    from the filter definitions, itself checked against (1).
//!
//! Inputs reach the engine as lossless WebP through the public facade and
//! the final rows are captured from an encoder backend, so decode, crop,
//! linearization, association, both filter passes and the final sRGB8
//! quantization are all inside the comparison.

#![cfg(feature = "webp-read")]
// Test-only arithmetic on small, bounded images and prose naming tools.
#![allow(
    clippy::assert_is_empty,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::chunks_exact_to_as_chunks,
    clippy::doc_markdown,
    clippy::many_single_char_names,
    clippy::trivially_copy_pass_by_ref,
    clippy::type_complexity
)]

#[path = "support/resize_oracle.rs"]
mod resize_oracle;

use ferrisma::pipeline::{
    Crop, EncodeError, EncodeRequest, EncodedImage, EncoderBackend, ImageFormat,
    InputProfilePolicy, OutputFormat, ResizeFilter, RowEncoder, TransformOptions, WorkingPrecision,
    transcode_with,
};
use resize_oracle::{Kernel, Linear, Primaries, Request};
use std::sync::Mutex;
use webp::api::encode::{PixelLayout, PixelRows, encode_lossless};

/// Collects the final straight-alpha sRGB8 rows the engine hands an encoder.
struct Capture(Mutex<Vec<u8>>);
struct CaptureRows<'a>(&'a Mutex<Vec<u8>>);

impl EncoderBackend for Capture {
    fn format(&self) -> OutputFormat {
        OutputFormat::WebP
    }
    fn name(&self) -> &'static str {
        "test-row-capture"
    }
    fn begin(&self, _: &EncodeRequest) -> Result<Box<dyn RowEncoder + '_>, EncodeError> {
        self.0.lock().unwrap().clear();
        Ok(Box::new(CaptureRows(&self.0)))
    }
}

impl RowEncoder for CaptureRows<'_> {
    fn write_row(&mut self, _: u32, row: &[u8]) -> Result<(), EncodeError> {
        self.0.lock().unwrap().extend_from_slice(row);
        Ok(())
    }
    fn finish(self: Box<Self>) -> Result<EncodedImage, EncodeError> {
        Ok(EncodedImage::new(OutputFormat::WebP, Vec::new(), 0, false))
    }
}

pub struct Input {
    pub width: u32,
    pub height: u32,
    pub alpha: bool,
    pub seed: u64,
    /// FNV-1a 64 printed by `generate.py`.
    pub fnv: u64,
}

const OPAQUE: Input = Input {
    width: 48,
    height: 36,
    alpha: false,
    seed: 0x5EED_0001,
    fnv: 0x30b0_f565_c6c5_260b,
};
const ALPHA: Input = Input {
    width: 40,
    height: 30,
    alpha: true,
    seed: 0x5EED_0002,
    fnv: 0x993d_2ea6_c942_cb59,
};
const LARGE_OPAQUE: Input = Input {
    width: 240,
    height: 160,
    alpha: false,
    seed: 0x5EED_0003,
    fnv: 0x684a_822c_501a_4db5,
};
const LARGE_ALPHA: Input = Input {
    width: 240,
    height: 160,
    alpha: true,
    seed: 0x5EED_0004,
    fnv: 0x89e5_6454_1ce9_e416,
};

fn pixels(input: &Input) -> Vec<u8> {
    let data = resize_oracle::pattern(input.width, input.height, input.alpha, input.seed);
    assert_eq!(
        resize_oracle::fnv1a64(&data),
        input.fnv,
        "the Rust pattern drifted from generate.py"
    );
    data
}

fn webp(input: &Input, rgba: &[u8]) -> Vec<u8> {
    if input.alpha {
        encode_lossless(PixelRows {
            layout: PixelLayout::Rgba,
            data: rgba,
            width: input.width,
            height: input.height,
            stride: input.width as usize * 4,
        })
        .unwrap()
    } else {
        let rgb: Vec<u8> = rgba
            .chunks_exact(4)
            .flat_map(|p| [p[0], p[1], p[2]])
            .collect();
        encode_lossless(PixelRows {
            layout: PixelLayout::Rgb,
            data: &rgb,
            width: input.width,
            height: input.height,
            stride: input.width as usize * 3,
        })
        .unwrap()
    }
}

fn engine(
    encoded: &[u8],
    request: &Request,
    precision: WorkingPrecision,
    policy: InputProfilePolicy,
) -> Vec<u8> {
    let capture = Capture(Mutex::new(Vec::new()));
    let (x, y, width, height) = request.crop;
    let options = TransformOptions {
        output_format: OutputFormat::WebP,
        output_width: request.output.0,
        output_height: request.output.1,
        crop: Some(Crop {
            x,
            y,
            width,
            height,
        }),
        filter: match request.kernel {
            Kernel::Area => ResizeFilter::Area,
            Kernel::Lanczos3 => ResizeFilter::Lanczos3,
        },
        precision,
        profile_policy: policy,
        transform_threads: 1,
        ..TransformOptions::default()
    };
    let output = transcode_with(encoded, ImageFormat::WebP, options, &capture).unwrap();
    assert_eq!((output.width, output.height), request.output);
    capture.0.into_inner().unwrap()
}

#[derive(Debug, Default)]
struct Stats {
    compared_rgb: usize,
    exact_rgb: usize,
    max_rgb: u8,
    max_alpha: u8,
    exact_alpha: usize,
    alpha_samples: usize,
}

/// Compare two straight RGBA8 images: alpha everywhere, RGB only where both
/// alpha bytes are nonzero (ADR-0021 rule E). Behind alpha 0 each side must
/// emit RGB 0 (ADR-0014).
fn compare8(actual: &[u8], expected: &[u8]) -> Stats {
    assert_eq!(actual.len(), expected.len());
    let mut stats = Stats::default();
    for (a, e) in actual.chunks_exact(4).zip(expected.chunks_exact(4)) {
        for side in [a, e] {
            if side[3] == 0 {
                assert_eq!(&side[..3], &[0, 0, 0], "visible color behind alpha 0");
            }
        }
        let da = a[3].abs_diff(e[3]);
        stats.max_alpha = stats.max_alpha.max(da);
        stats.alpha_samples += 1;
        stats.exact_alpha += usize::from(da == 0);
        if a[3] != 0 && e[3] != 0 {
            for c in 0..3 {
                let d = a[c].abs_diff(e[c]);
                stats.max_rgb = stats.max_rgb.max(d);
                stats.compared_rgb += 1;
                stats.exact_rgb += usize::from(d == 0);
            }
        }
    }
    stats
}

/// Every alpha byte that differs from the oracle must sit on a rounding tie:
/// the oracle's exact `255 * alpha` is within `1e-6` of a half code. (The
/// 0/255 stripes and symmetric weights produce exact halves, which `f32`
/// and differently ordered `f64` sums round either way.)
fn assert_alpha_differences_are_ties(name: &str, actual: &[u8], oracle: &[u8], linear: &Linear) {
    for ((a, e), l) in actual
        .chunks_exact(4)
        .zip(oracle.chunks_exact(4))
        .zip(linear.rgba.chunks_exact(4))
    {
        if a[3] != e[3] {
            let scaled = l[3].clamp(0.0, 1.0) * 255.0;
            assert!(
                (scaled - scaled.floor() - 0.5).abs() < 1e-6,
                "{name}: alpha {} vs {} is not a tie (255 * alpha = {scaled})",
                a[3],
                e[3]
            );
        }
    }
}

impl Stats {
    fn add(&mut self, other: &Stats) {
        self.compared_rgb += other.compared_rgb;
        self.exact_rgb += other.exact_rgb;
        self.max_rgb = self.max_rgb.max(other.max_rgb);
        self.max_alpha = self.max_alpha.max(other.max_alpha);
        self.exact_alpha += other.exact_alpha;
        self.alpha_samples += other.alpha_samples;
    }
}

#[allow(clippy::cast_precision_loss)]
fn share(n: usize, d: usize) -> f64 {
    if d == 0 { 1.0 } else { n as f64 / d as f64 }
}

/// ImageMagick's result, re-associated: it stores straight color divided by
/// the unclamped filtered alpha, so `color * alpha` recovers the associated
/// filter output (also where alpha is negative or above one).
fn read_magick(name: &str, output: (u32, u32)) -> Linear {
    let path = format!(
        "{}/tests/fixtures/resize/{name}.f32",
        env!("CARGO_MANIFEST_DIR")
    );
    let bytes = std::fs::read(&path).unwrap_or_else(|e| panic!("{path}: {e}"));
    let straight: Vec<f64> = bytes
        .chunks_exact(4)
        .map(|b| f64::from(f32::from_le_bytes([b[0], b[1], b[2], b[3]])))
        .collect();
    assert_eq!(straight.len(), (output.0 * output.1 * 4) as usize, "{path}");
    let rgba = straight
        .chunks_exact(4)
        .flat_map(|p| [p[0] * p[3], p[1] * p[3], p[2] * p[3], p[3]])
        .collect();
    Linear {
        width: output.0,
        height: output.1,
        rgba,
    }
}

/// Largest absolute difference of associated linear RGB and of alpha.
fn linear_distance(a: &Linear, b: &Linear) -> (f64, f64) {
    let mut color = 0.0_f64;
    let mut alpha = 0.0_f64;
    for (p, q) in a.rgba.chunks_exact(4).zip(b.rgba.chunks_exact(4)) {
        alpha = alpha.max((p[3] - q[3]).abs());
        for c in 0..3 {
            color = color.max((p[c] - q[c]).abs());
        }
    }
    (color, alpha)
}

/// Bound between the independent oracle and ImageMagick in linear light.
/// ImageMagick Q16-HDRI stores every intermediate image and the result as
/// `float` scaled by 65535 (relative step `2^-24` = 6.0e-8). A sample passes
/// through at most three float roundings (decode, the intermediate pass,
/// the result) and a weighted sum whose absolute weight total is below 1.3
/// (Lanczos3, ADR-0021 E7), so the representation error is below
/// `3 * 1.3 * 6.0e-8 * max|value|` ~ 2.6e-7 for values up to 1.1. The
/// bound adds a factor of four for the order of float summation over at
/// most 1,920 taps per output and the `pow`/`sin` implementations: 1e-6.
/// That is 1/300 of the darkest sRGB8 code step (3.0e-4 linear).
const IMAGEMAGICK_LINEAR_BOUND: f64 = 1e-6;

struct Case {
    name: &'static str,
    input: &'static Input,
    crop: Option<(u32, u32, u32, u32)>,
    output: (u32, u32),
    kernel: Kernel,
}

const CASES: &[Case] = &[
    Case {
        name: "opaque-up-lanczos3",
        input: &OPAQUE,
        crop: None,
        output: (101, 77),
        kernel: Kernel::Lanczos3,
    },
    Case {
        name: "opaque-up-area",
        input: &OPAQUE,
        crop: None,
        output: (101, 77),
        kernel: Kernel::Area,
    },
    Case {
        name: "alpha-up-lanczos3",
        input: &ALPHA,
        crop: None,
        output: (87, 61),
        kernel: Kernel::Lanczos3,
    },
    Case {
        name: "alpha-up-area",
        input: &ALPHA,
        crop: None,
        output: (87, 61),
        kernel: Kernel::Area,
    },
    Case {
        name: "large-opaque-extreme-lanczos3",
        input: &LARGE_OPAQUE,
        crop: None,
        output: (3, 2),
        kernel: Kernel::Lanczos3,
    },
    Case {
        name: "large-opaque-extreme-area",
        input: &LARGE_OPAQUE,
        crop: None,
        output: (3, 2),
        kernel: Kernel::Area,
    },
    Case {
        name: "large-alpha-extreme-lanczos3",
        input: &LARGE_ALPHA,
        crop: None,
        output: (4, 3),
        kernel: Kernel::Lanczos3,
    },
    Case {
        name: "large-alpha-extreme-area",
        input: &LARGE_ALPHA,
        crop: None,
        output: (4, 3),
        kernel: Kernel::Area,
    },
    Case {
        name: "opaque-crop-mixed-lanczos3",
        input: &OPAQUE,
        crop: Some((7, 5, 29, 23)),
        output: (11, 40),
        kernel: Kernel::Lanczos3,
    },
    Case {
        name: "opaque-crop-mixed-area",
        input: &OPAQUE,
        crop: Some((7, 5, 29, 23)),
        output: (11, 40),
        kernel: Kernel::Area,
    },
    Case {
        name: "alpha-crop-odd-lanczos3",
        input: &ALPHA,
        crop: Some((3, 1, 31, 27)),
        output: (9, 8),
        kernel: Kernel::Lanczos3,
    },
    Case {
        name: "alpha-crop-odd-area",
        input: &ALPHA,
        crop: Some((3, 1, 31, 27)),
        output: (9, 8),
        kernel: Kernel::Area,
    },
    Case {
        name: "alpha-column-lanczos3",
        input: &ALPHA,
        crop: Some((13, 0, 1, 30)),
        output: (2, 7),
        kernel: Kernel::Lanczos3,
    },
    Case {
        name: "large-alpha-mixed-lanczos3",
        input: &LARGE_ALPHA,
        crop: Some((1, 3, 237, 9)),
        output: (5, 31),
        kernel: Kernel::Lanczos3,
    },
    Case {
        name: "large-alpha-mixed-area",
        input: &LARGE_ALPHA,
        crop: Some((1, 3, 237, 9)),
        output: (5, 31),
        kernel: Kernel::Area,
    },
];

fn request(case: &Case) -> Request {
    Request {
        kernel: case.kernel,
        primaries: Primaries::Srgb,
        crop: case
            .crop
            .unwrap_or((0, 0, case.input.width, case.input.height)),
        output: case.output,
        alpha: case.input.alpha,
        clip_before_filter: false,
    }
}

#[test]
fn independent_oracle_matches_imagemagick_in_linear_light() {
    for case in CASES {
        let rgba = pixels(case.input);
        let oracle = resize_oracle::resize(&rgba, case.input.width, &request(case));
        let magick = read_magick(case.name, case.output);
        let (color, alpha) = linear_distance(&oracle, &magick);
        println!(
            "{}: oracle vs ImageMagick color {color:.3e} alpha {alpha:.3e}",
            case.name
        );
        assert!(
            color <= IMAGEMAGICK_LINEAR_BOUND && alpha <= IMAGEMAGICK_LINEAR_BOUND,
            "{}: oracle and ImageMagick differ by {color:e} (color) / {alpha:e} (alpha)",
            case.name
        );
    }
}

/// Engine output against both oracles after the explicit final quantization.
///
/// Tolerances are ADR-0021 rule 4 for `f32` paths, adopted unchanged rather
/// than fitted to this run: rule A, at most one code on every compared
/// sample (any implementation whose rounding points differ from the
/// oracle's can land on the other side of a code boundary at a tie); rules B
/// and C', at least 99.9 % exactly equal RGB and 98 % exactly equal alpha,
/// pooled over all cases so that tiny outputs (6 samples at 80x) cannot
/// dominate a percentage. A wrong kernel, edge rule, transfer function or
/// alpha association moves most samples, not a few ties. The `f64` engine
/// reference must satisfy the same bound against these oracles.
#[test]
fn engine_matches_imagemagick_and_independent_oracle_after_quantization() {
    let mut pooled: Vec<(String, Stats)> = Vec::new();
    for case in CASES {
        let rgba = pixels(case.input);
        let encoded = webp(case.input, &rgba);
        let request = request(case);
        let oracle_linear = resize_oracle::resize(&rgba, case.input.width, &request);
        let oracle = resize_oracle::quantize(&oracle_linear);
        // ImageMagick's filter output through the documented output step.
        let magick = resize_oracle::quantize(&read_magick(case.name, case.output));
        for precision in [WorkingPrecision::F32, WorkingPrecision::F64Reference] {
            let actual = engine(
                &encoded,
                &request,
                precision,
                InputProfilePolicy::AssumeSrgbIfMissing,
            );
            assert_alpha_differences_are_ties(case.name, &actual, &oracle, &oracle_linear);
            for (label, expected) in [("oracle", &oracle), ("ImageMagick", &magick)] {
                let stats = compare8(&actual, expected);
                println!("{} {precision:?} vs {label}: {stats:?}", case.name);
                assert!(
                    stats.max_rgb <= 1 && stats.max_alpha <= 1,
                    "{} {precision:?} vs {label}: {stats:?}",
                    case.name
                );
                let key = format!("{precision:?} vs {label}");
                if let Some((_, total)) = pooled.iter_mut().find(|(k, _)| *k == key) {
                    total.add(&stats);
                } else {
                    pooled.push((key, stats));
                }
            }
        }
    }
    for (key, total) in &pooled {
        println!("pooled {key}: {total:?}");
        assert!(
            share(total.exact_rgb, total.compared_rgb) >= 0.999
                && share(total.exact_alpha, total.alpha_samples) >= 0.98,
            "pooled {key}: {total:?}"
        );
    }
}

/// A wider geometry sweep against the in-tree oracle alone (no retained
/// ImageMagick output): identity, mild up/down, 16x, 120x by 160x, a 1x1
/// output, an anisotropic 1x/16x, a crop touching the far image corner and
/// one- and two-pixel sources. Same bound as above.
#[test]
fn engine_matches_independent_oracle_across_geometry_sweep() {
    let sweep: &[(&Input, Option<(u32, u32, u32, u32)>, (u32, u32))] = &[
        (&OPAQUE, None, (48, 36)),
        (&ALPHA, None, (40, 30)),
        (&ALPHA, None, (60, 45)),
        (&LARGE_ALPHA, None, (101, 67)),
        (&LARGE_ALPHA, None, (15, 10)),
        (&LARGE_ALPHA, None, (2, 1)),
        (&LARGE_OPAQUE, None, (1, 1)),
        (&LARGE_ALPHA, None, (240, 10)),
        (&LARGE_ALPHA, Some((229, 151, 11, 9)), (23, 4)),
        (&ALPHA, Some((39, 29, 1, 1)), (5, 3)),
        (&ALPHA, Some((38, 28, 2, 2)), (7, 7)),
        (&OPAQUE, Some((47, 0, 1, 36)), (1, 5)),
    ];
    let mut total = Stats::default();
    for &(input, crop, output) in sweep {
        let rgba = pixels(input);
        let encoded = webp(input, &rgba);
        for kernel in [Kernel::Area, Kernel::Lanczos3] {
            let request = Request {
                kernel,
                primaries: Primaries::Srgb,
                crop: crop.unwrap_or((0, 0, input.width, input.height)),
                output,
                alpha: input.alpha,
                clip_before_filter: false,
            };
            let linear = resize_oracle::resize(&rgba, input.width, &request);
            let oracle = resize_oracle::quantize(&linear);
            for precision in [WorkingPrecision::F32, WorkingPrecision::F64Reference] {
                let actual = engine(
                    &encoded,
                    &request,
                    precision,
                    InputProfilePolicy::AssumeSrgbIfMissing,
                );
                let name = format!("{crop:?}->{output:?} {kernel:?} {precision:?}");
                assert_alpha_differences_are_ties(&name, &actual, &oracle, &linear);
                let stats = compare8(&actual, &oracle);
                assert!(
                    stats.max_rgb <= 1 && stats.max_alpha <= 1,
                    "{name}: {stats:?}"
                );
                total.add(&stats);
            }
        }
    }
    println!("sweep pooled: {total:?}");
    assert!(
        share(total.exact_rgb, total.compared_rgb) >= 0.999
            && share(total.exact_alpha, total.alpha_samples) >= 0.98,
        "{total:?}"
    );
}

/// The final quantization alone: an identity geometry carries every sRGB8
/// code through decode, association, both filter passes and the explicit
/// output encode unchanged.
#[test]
fn identity_geometry_round_trips_every_srgb8_code() {
    let rgba: Vec<u8> = (0..=255_u8)
        .flat_map(|c| [c, 255 - c, c.wrapping_mul(7), 255])
        .collect();
    let input = Input {
        width: 256,
        height: 1,
        alpha: false,
        seed: 0,
        fnv: 0,
    };
    let encoded = webp(&input, &rgba);
    for kernel in [Kernel::Area, Kernel::Lanczos3] {
        let request = Request {
            kernel,
            primaries: Primaries::Srgb,
            crop: (0, 0, 256, 1),
            output: (256, 1),
            alpha: false,
            clip_before_filter: false,
        };
        assert_eq!(
            resize_oracle::quantize(&resize_oracle::resize(&rgba, 256, &request)),
            rgba
        );
        for precision in [WorkingPrecision::F32, WorkingPrecision::F64Reference] {
            let actual = engine(
                &encoded,
                &request,
                precision,
                InputProfilePolicy::AssumeSrgbIfMissing,
            );
            assert_eq!(actual, rgba, "{kernel:?} {precision:?}");
        }
    }
}

/// An intentional difference from ImageMagick, retained as a regression:
/// where Lanczos overshoot pushes the filtered alpha above one, the engine
/// clamps alpha first and divides the associated color by the clamped
/// alpha (so the visible, opaque result equals the filtered associated
/// light); ImageMagick divides by the unclamped alpha. Both filters agree
/// (see the linear-light test); only this output convention differs, and
/// only on pixels whose filtered alpha exceeds one.
#[test]
fn alpha_overshoot_unassociation_differs_from_imagemagick_only_above_one() {
    let case = CASES
        .iter()
        .find(|case| case.name == "alpha-up-lanczos3")
        .unwrap();
    let rgba = pixels(case.input);
    let actual = engine(
        &webp(case.input, &rgba),
        &request(case),
        WorkingPrecision::F32,
        InputProfilePolicy::AssumeSrgbIfMissing,
    );
    let magick = read_magick(case.name, case.output);
    let unclamped = resize_oracle::quantize_unclamped_division(&magick);
    let mut overshoot_pixels = 0;
    let mut largest = 0;
    for ((a, m), l) in actual
        .chunks_exact(4)
        .zip(unclamped.chunks_exact(4))
        .zip(magick.rgba.chunks_exact(4))
    {
        let difference = (0..3).map(|c| a[c].abs_diff(m[c])).max().unwrap();
        if l[3] > 1.0 + 1e-6 {
            overshoot_pixels += 1;
            largest = largest.max(difference);
        } else if a[3] != 0 && m[3] != 0 {
            assert!(
                difference <= 1,
                "difference outside the overshoot: {a:?} {m:?}"
            );
        }
    }
    println!("{overshoot_pixels} pixels with alpha > 1, largest RGB difference {largest}");
    assert!(overshoot_pixels > 0 && largest > 1);
}

/// Wrap a simple (VP8L) WebP into an extended file with an ICCP chunk.
#[cfg(feature = "icc")]
fn with_icc(simple: &[u8], width: u32, height: u32, icc: &[u8]) -> Vec<u8> {
    fn chunk(tag: &[u8; 4], payload: &[u8]) -> Vec<u8> {
        let mut out = tag.to_vec();
        out.extend_from_slice(&u32::try_from(payload.len()).unwrap().to_le_bytes());
        out.extend_from_slice(payload);
        if payload.len() % 2 == 1 {
            out.push(0);
        }
        out
    }
    let mut vp8x = vec![0x20, 0, 0, 0];
    vp8x.extend_from_slice(&(width - 1).to_le_bytes()[..3]);
    vp8x.extend_from_slice(&(height - 1).to_le_bytes()[..3]);
    let mut body = chunk(b"VP8X", &vp8x);
    body.extend(chunk(b"ICCP", icc));
    body.extend_from_slice(&simple[12..]);
    let mut out = b"RIFF".to_vec();
    out.extend_from_slice(&u32::try_from(body.len() + 4).unwrap().to_le_bytes());
    out.extend_from_slice(b"WEBP");
    out.extend(body);
    out
}

/// Display P3 input through the explicit rendered-ICC policy (moxcms) and
/// both filters, against the oracle's own primaries-derived P3 matrix.
///
/// The per-sample bound is derived, not fitted: ADR-0023 bounds moxcms
/// against LittleCMS 2.19 by `8e-4` linear on this exact profile
/// (`DisplayP3-v4.icc`); the profile's s15Fixed16 colorants and D50
/// adaptation add at most about `2e-4` against exact chromaticity math, so
/// each decoded sample is within `1e-3`. Area weights are nonnegative and
/// sum to one; Lanczos3 absolute weights sum to at most `1.2857` per axis
/// (ADR-0021 E7), so a filtered sample is within `1e-3 * 1.2857^2` =
/// `1.66e-3`. The engine's code must lie between the oracle's encodings of
/// `v - bound` and `v + bound`.
///
/// The noise regions of the patterns contain P3 colors outside sRGB. The
/// same comparison against an oracle that clips before filtering must fail
/// somewhere, which proves the test can see premature gamut clipping.
#[cfg(feature = "icc")]
#[test]
fn display_p3_icc_input_keeps_out_of_gamut_values_until_output() {
    const BOUND: f64 = 1e-3 * 1.2857 * 1.2857;
    let profile = include_bytes!("fixtures/icc/DisplayP3-v4.icc");
    let geometry: &[(&Input, Option<(u32, u32, u32, u32)>, (u32, u32))] = &[
        (&OPAQUE, None, (101, 77)),
        (&OPAQUE, Some((7, 5, 29, 23)), (11, 40)),
        (&LARGE_OPAQUE, None, (37, 23)),
        (&LARGE_OPAQUE, None, (3, 2)),
    ];
    let mut clipped_outside = 0_usize;
    let mut compared = 0_usize;
    for &(input, crop, output) in geometry {
        let rgba = pixels(input);
        let encoded = with_icc(&webp(input, &rgba), input.width, input.height, profile);
        for kernel in [Kernel::Area, Kernel::Lanczos3] {
            let mut request = Request {
                kernel,
                primaries: Primaries::DisplayP3,
                crop: crop.unwrap_or((0, 0, input.width, input.height)),
                output,
                alpha: false,
                clip_before_filter: false,
            };
            let linear = resize_oracle::resize(&rgba, input.width, &request);
            request.clip_before_filter = true;
            let clipped =
                resize_oracle::quantize(&resize_oracle::resize(&rgba, input.width, &request));
            for precision in [WorkingPrecision::F32, WorkingPrecision::F64Reference] {
                let actual = engine(
                    &encoded,
                    &request,
                    precision,
                    InputProfilePolicy::UseRenderedIcc,
                );
                for ((a, l), c) in actual
                    .chunks_exact(4)
                    .zip(linear.rgba.chunks_exact(4))
                    .zip(clipped.chunks_exact(4))
                {
                    assert_eq!(a[3], 255);
                    for channel in 0..3 {
                        let lo = resize_oracle::encode8(l[channel] - BOUND);
                        let hi = resize_oracle::encode8(l[channel] + BOUND);
                        assert!(
                            (lo..=hi).contains(&a[channel]),
                            "{kernel:?} {precision:?} {output:?}: code {} outside {lo}..={hi} (linear {})",
                            a[channel],
                            l[channel]
                        );
                        compared += 1;
                        clipped_outside += usize::from(!(lo..=hi).contains(&c[channel]));
                    }
                }
            }
        }
    }
    println!(
        "{compared} samples; a clip-before-filter oracle leaves the bound on {clipped_outside}"
    );
    assert!(clipped_outside > 0);
}

/// The rendered-ICC adapter is RGB-only and opaque-only today: an ICC-tagged
/// RGBA input is a controlled `UnsupportedInput`, not a silent drop of alpha
/// or of the profile.
#[cfg(feature = "icc")]
#[test]
fn icc_tagged_alpha_input_is_rejected_before_output() {
    use ferrisma::pipeline::{ProcessingErrorKind, transcode_with};
    let rgba = pixels(&ALPHA);
    let mut encoded = with_icc(
        &webp(&ALPHA, &rgba),
        ALPHA.width,
        ALPHA.height,
        include_bytes!("fixtures/icc/DisplayP3-v4.icc"),
    );
    encoded[20] |= 0x10; // VP8X alpha flag
    let capture = Capture(Mutex::new(Vec::new()));
    let error = transcode_with(
        &encoded,
        ImageFormat::WebP,
        TransformOptions {
            output_format: OutputFormat::WebP,
            output_width: 4,
            output_height: 3,
            profile_policy: InputProfilePolicy::UseRenderedIcc,
            ..TransformOptions::default()
        },
        &capture,
    )
    .unwrap_err();
    assert_eq!(error.kind(), ProcessingErrorKind::UnsupportedInput);
    assert!(capture.0.into_inner().unwrap().is_empty());
}
