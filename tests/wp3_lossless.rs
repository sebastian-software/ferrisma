//! WP3 differential test: the lossless DSP functions of `src/dsp/lossless.c`.
//! One script drives the C probe `tools/oracle/probes/wp3/lossless.c` and the
//! Rust translation, which builds the expected output:
//!
//! - every predictor of `VP8LPredictors` and the exported `VP8LPredictorN_C`
//!   on seeded and edge pixels;
//! - every batch predictor of `VP8LPredictorsAdd` and `VP8LPredictorsAdd_C` in
//!   the decoder's layout, with the row above in the same buffer, in place and
//!   from a separate input;
//! - `VP8LAddGreenToBlueAndRed`, `VP8LTransformColorInverse` and
//!   `VP8LMapColor32b`, in place and from a separate input, and
//!   `VP8LMapColor8b`;
//! - `VP8LInverseTransform` for the four transform types, every predictor and
//!   cross-color tile size and every color-indexing bit depth, on seeded ARGB
//!   rows of odd widths, from the first row and from later rows, in place and
//!   from a separate input;
//! - `VP8LColorIndexInverseTransformAlpha` for every bit depth;
//! - `VP8LConvertFromBGRA` for every RGB mode and the five converters with
//!   their dispatch functions.

#[path = "support/oracle.rs"]
mod oracle;

use std::fmt::Write as _;

use oracle::Oracle;
use webp::dec::vp8li_dec::VP8LTransform;
use webp::dsp::lossless::{
    VP8LMultipliers, VP8LPredictorFunc, vp8l_add_green_to_blue_and_red,
    vp8l_add_green_to_blue_and_red_c, vp8l_color_index_inverse_transform_alpha,
    vp8l_convert_bgra_to_bgr, vp8l_convert_bgra_to_bgr_c, vp8l_convert_bgra_to_rgb,
    vp8l_convert_bgra_to_rgb_c, vp8l_convert_bgra_to_rgb565, vp8l_convert_bgra_to_rgb565_c,
    vp8l_convert_bgra_to_rgba, vp8l_convert_bgra_to_rgba_c, vp8l_convert_bgra_to_rgba4444,
    vp8l_convert_bgra_to_rgba4444_c, vp8l_convert_from_bgra, vp8l_inverse_transform,
    vp8l_map_color8b, vp8l_map_color32b, vp8l_predictor2_c, vp8l_predictor3_c, vp8l_predictor4_c,
    vp8l_predictor5_c, vp8l_predictor6_c, vp8l_predictor7_c, vp8l_predictor8_c, vp8l_predictor9_c,
    vp8l_predictor10_c, vp8l_predictor11_c, vp8l_predictor12_c, vp8l_predictor13_c,
    vp8l_predictors, vp8l_predictors_add, vp8l_predictors_add_c, vp8l_transform_color_inverse,
    vp8l_transform_color_inverse_c,
};
use webp::dsp::lossless_common::vp8l_sub_sample_size;
use webp::webp::decode::WEBP_CSP_MODE;
use webp::webp::format_constants::VP8LImageTransformType;

/// SplitMix64, a small deterministic generator for the script.
struct Rng(u64);

impl Rng {
    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    fn below(&mut self, bound: usize) -> usize {
        (self.next_u64() % bound as u64) as usize
    }

    /// A pixel: random, or with channels near 0 and 255 to reach the clips.
    fn pixel(&mut self) -> u32 {
        if self.below(3) == 0 {
            let channel = |rng: &mut Self| -> u32 {
                match rng.below(4) {
                    0 => rng.below(3) as u32,
                    1 => 253 + rng.below(3) as u32,
                    2 => 126 + rng.below(4) as u32,
                    _ => rng.below(256) as u32,
                }
            };
            (channel(self) << 24) | (channel(self) << 16) | (channel(self) << 8) | channel(self)
        } else {
            self.next_u64() as u32
        }
    }

    fn pixels(&mut self, count: usize) -> Vec<u32> {
        (0..count).map(|_| self.pixel()).collect()
    }

    /// An odd width most of the time.
    fn width(&mut self, max: usize) -> usize {
        let width = 1 + self.below(max);
        if self.below(5) == 0 { width } else { width | 1 }
    }
}

fn hex32(values: &[u32]) -> String {
    if values.is_empty() {
        "-".to_owned()
    } else {
        values.iter().map(|value| format!("{value:08x}")).collect()
    }
}

fn hex(bytes: &[u8]) -> String {
    if bytes.is_empty() {
        "-".to_owned()
    } else {
        bytes.iter().map(|byte| format!("{byte:02x}")).collect()
    }
}

/// The output line of a printed array: the probe prints nothing after `=` for
/// an empty array.
fn line(key: &str, value: &str) -> String {
    format!("{key}={}\n", if value == "-" { "" } else { value })
}

const EXPORTED: [Option<VP8LPredictorFunc>; 16] = [
    None,
    None,
    Some(vp8l_predictor2_c),
    Some(vp8l_predictor3_c),
    Some(vp8l_predictor4_c),
    Some(vp8l_predictor5_c),
    Some(vp8l_predictor6_c),
    Some(vp8l_predictor7_c),
    Some(vp8l_predictor8_c),
    Some(vp8l_predictor9_c),
    Some(vp8l_predictor10_c),
    Some(vp8l_predictor11_c),
    Some(vp8l_predictor12_c),
    Some(vp8l_predictor13_c),
    None,
    None,
];

struct Script {
    rng: Rng,
    commands: String,
    expected: String,
}

impl Script {
    fn predictors(&mut self) {
        for (mode, exported) in EXPORTED.iter().enumerate() {
            let mut quads = self.rng.pixels(4 * 300);
            quads.extend_from_slice(&[0, 0, 0, 0, !0, !0, !0, !0, 0xff00_0000, 0, !0, 0x00ff_ffff]);
            writeln!(self.commands, "predict {mode} {}", hex32(&quads)).unwrap();
            let pred: Vec<u32> = quads
                .as_chunks::<4>()
                .0
                .iter()
                .map(|quad| vp8l_predictors(mode, quad, 0, quad, 2))
                .collect();
            self.expected += &line("pred", &hex32(&pred));
            if let Some(exported) = exported {
                let predc: Vec<u32> = quads
                    .as_chunks::<4>()
                    .0
                    .iter()
                    .map(|quad| exported(quad, 0, quad, 2))
                    .collect();
                self.expected += &line("predc", &hex32(&predc));
            }
        }
    }

    /// A batch predictor in the decoder's layout: the row above and the
    /// current row of `width` pixels, and the input in the current row or in a
    /// separate row after it.
    fn batch_predictor(&mut self, table: usize, mode: usize) {
        let width = 2 + self.rng.below(40);
        let x = 1 + self.rng.below(width - 1);
        let n = self.rng.below(width - x + 1);
        let in_place = self.rng.below(2) == 0;
        let upper = x;
        let out = width + x;
        let input = if in_place { out } else { 2 * width + x };
        let mut buf = self.rng.pixels(3 * width);
        writeln!(
            self.commands,
            "add {table} {mode} {n} {input} {upper} {out} {}",
            hex32(&buf)
        )
        .unwrap();
        let n = i32::try_from(n).unwrap();
        if table == 0 {
            vp8l_predictors_add(mode, &mut buf, input, upper, n, out);
        } else {
            vp8l_predictors_add_c(mode, &mut buf, input, upper, n, out);
        }
        self.expected += &line("buf", &hex32(&buf));
    }

    /// Positions of an element-wise call: in place, or from a separate region.
    fn element_layout(&mut self, n: usize) -> (usize, usize, usize) {
        let lead = self.rng.below(3);
        if self.rng.below(2) == 0 {
            (lead, lead, lead + n + self.rng.below(3))
        } else {
            let src = lead + n + self.rng.below(3);
            (src, lead, src + n + self.rng.below(3))
        }
    }

    fn element_wise(&mut self) {
        for table in 0..2 {
            let n = self.rng.width(70) - 1;
            let (src, dst, len) = self.element_layout(n);
            let mut buf = self.rng.pixels(len);
            writeln!(
                self.commands,
                "green {table} {n} {src} {dst} {}",
                hex32(&buf)
            )
            .unwrap();
            let count = i32::try_from(n).unwrap();
            if table == 0 {
                vp8l_add_green_to_blue_and_red(&mut buf, src, count, dst);
            } else {
                vp8l_add_green_to_blue_and_red_c(&mut buf, src, count, dst);
            }
            self.expected += &line("buf", &hex32(&buf));

            let m = VP8LMultipliers {
                green_to_red: self.rng.below(256) as u8,
                green_to_blue: self.rng.below(256) as u8,
                red_to_blue: self.rng.below(256) as u8,
            };
            let mut buf = self.rng.pixels(len);
            writeln!(
                self.commands,
                "color {table} {} {} {} {n} {src} {dst} {}",
                m.green_to_red,
                m.green_to_blue,
                m.red_to_blue,
                hex32(&buf)
            )
            .unwrap();
            if table == 0 {
                vp8l_transform_color_inverse(&m, &mut buf, src, count, dst);
            } else {
                vp8l_transform_color_inverse_c(&m, &mut buf, src, count, dst);
            }
            self.expected += &line("buf", &hex32(&buf));
        }

        // The color maps, over a few rows.
        let width = self.rng.width(30);
        let rows = 1 + self.rng.below(4);
        let start = self.rng.below(5) as i32;
        let n = width * rows;
        let (src, dst, len) = self.element_layout(n);
        let map = self.rng.pixels(256);
        let mut buf = self.rng.pixels(len);
        let end = start + rows as i32;
        writeln!(
            self.commands,
            "map32 {start} {end} {width} {src} {dst} {} {}",
            hex32(&map),
            hex32(&buf)
        )
        .unwrap();
        vp8l_map_color32b(&mut buf, src, &map, dst, start, end, width as i32);
        self.expected += &line("buf", &hex32(&buf));

        let source: Vec<u8> = (0..n).map(|_| self.rng.below(256) as u8).collect();
        let mut dst = vec![0xaa_u8; n + self.rng.below(3)];
        writeln!(
            self.commands,
            "map8 {start} {end} {width} {} {} {}",
            hex32(&map),
            hex(&source),
            dst.len()
        )
        .unwrap();
        vp8l_map_color8b(&source, &map, &mut dst, start, end, width as i32);
        self.expected += &line("dst", &hex(&dst));
    }

    /// `VP8LInverseTransform` in the decoder's layout: a row above the output
    /// (the previous batch's last row), the output rows, and the input rows in
    /// place or after them.
    fn inverse(&mut self, kind: VP8LImageTransformType) {
        let width = self.rng.width(67);
        let ysize = 1 + self.rng.below(12);
        let start = self.rng.below(ysize);
        let end = start + 1 + self.rng.below(ysize - start);
        let rows = end - start;
        let bits: usize = match kind {
            VP8LImageTransformType::PREDICTOR_TRANSFORM
            | VP8LImageTransformType::CROSS_COLOR_TRANSFORM => 2 + self.rng.below(8),
            VP8LImageTransformType::COLOR_INDEXING_TRANSFORM => self.rng.below(4),
            _ => 0,
        };
        let sub = |size: usize| vp8l_sub_sample_size(size as u32, bits as u32) as usize;
        let data = match kind {
            VP8LImageTransformType::PREDICTOR_TRANSFORM
            | VP8LImageTransformType::CROSS_COLOR_TRANSFORM => {
                self.rng.pixels(sub(width) * sub(ysize))
            }
            VP8LImageTransformType::COLOR_INDEXING_TRANSFORM => self.rng.pixels(1 << (8 >> bits)),
            _ => Vec::new(),
        };
        let packed = if kind == VP8LImageTransformType::COLOR_INDEXING_TRANSFORM {
            sub(width)
        } else {
            width
        };
        let in_place = self.rng.below(2) == 0;
        let out = width;
        let (input, len) = if in_place {
            (out, out + rows * width)
        } else {
            (out + rows * width, out + rows * width + rows * packed)
        };
        let spare = self.rng.below(3);
        let mut buf = self.rng.pixels(len + spare);
        writeln!(
            self.commands,
            "inverse {} {bits} {width} {ysize} {start} {end} {input} {out} {} {}",
            kind.0,
            hex32(&data),
            hex32(&buf)
        )
        .unwrap();
        let transform = VP8LTransform {
            r#type: kind,
            bits: bits as i32,
            xsize: width as i32,
            ysize: ysize as i32,
            data,
        };
        vp8l_inverse_transform(&transform, start as i32, end as i32, &mut buf, input, out);
        self.expected += &line("buf", &hex32(&buf));
    }

    fn alpha(&mut self) {
        let bits = self.rng.below(4);
        let width = self.rng.width(67);
        let ysize = 1 + self.rng.below(12);
        let start = self.rng.below(ysize);
        let end = start + 1 + self.rng.below(ysize - start);
        let rows = end - start;
        let packed = vp8l_sub_sample_size(width as u32, bits as u32) as usize;
        let data = self.rng.pixels(1 << (8 >> bits));
        let src: Vec<u8> = (0..rows * packed)
            .map(|_| self.rng.below(256) as u8)
            .collect();
        let mut dst = vec![0xaa_u8; rows * width + self.rng.below(3)];
        writeln!(
            self.commands,
            "alpha {bits} {width} {ysize} {start} {end} {} {} {}",
            hex32(&data),
            hex(&src),
            dst.len()
        )
        .unwrap();
        let transform = VP8LTransform {
            r#type: VP8LImageTransformType::COLOR_INDEXING_TRANSFORM,
            bits: bits as i32,
            xsize: width as i32,
            ysize: ysize as i32,
            data,
        };
        vp8l_color_index_inverse_transform_alpha(
            &transform,
            start as i32,
            end as i32,
            &src,
            &mut dst,
        );
        self.expected += &line("dst", &hex(&dst));
    }

    fn converters(&mut self) {
        // Bytes per pixel of MODE_RGB to MODE_rgbA_4444.
        const BYTES: [usize; 11] = [3, 4, 3, 4, 4, 2, 2, 4, 4, 4, 2];
        for (mode, bytes) in BYTES.iter().enumerate() {
            let n = self.rng.width(68) - 1;
            let input = self.rng.pixels(n);
            let mut out = vec![0xaa_u8; n * bytes + self.rng.below(3)];
            writeln!(
                self.commands,
                "convert {mode} {n} {} {}",
                hex32(&input),
                out.len()
            )
            .unwrap();
            vp8l_convert_from_bgra(&input, n as i32, WEBP_CSP_MODE(mode as u32), &mut out);
            self.expected += &line("out", &hex(&out));
        }
        type Convert = fn(&[u32], i32, &mut [u8]);
        let converters: [(Convert, usize); 10] = [
            (vp8l_convert_bgra_to_rgb_c, 3),
            (vp8l_convert_bgra_to_rgba_c, 4),
            (vp8l_convert_bgra_to_rgba4444_c, 2),
            (vp8l_convert_bgra_to_rgb565_c, 2),
            (vp8l_convert_bgra_to_bgr_c, 3),
            (vp8l_convert_bgra_to_rgb, 3),
            (vp8l_convert_bgra_to_rgba, 4),
            (vp8l_convert_bgra_to_rgba4444, 2),
            (vp8l_convert_bgra_to_rgb565, 2),
            (vp8l_convert_bgra_to_bgr, 3),
        ];
        for (kind, (convert, bytes)) in converters.iter().enumerate() {
            let n = self.rng.width(68) - 1;
            let input = self.rng.pixels(n);
            let mut out = vec![0xaa_u8; n * bytes + self.rng.below(3)];
            writeln!(
                self.commands,
                "conv {kind} {n} {} {}",
                hex32(&input),
                out.len()
            )
            .unwrap();
            convert(&input, n as i32, &mut out);
            self.expected += &line("out", &hex(&out));
        }
    }
}

fn script() -> (String, String) {
    let mut script = Script {
        rng: Rng(0x5eed_0043),
        commands: String::from("init\n"),
        expected: String::new(),
    };
    script.predictors();
    for round in 0..40 {
        for mode in 0..16 {
            script.batch_predictor(round % 2, mode);
        }
        script.element_wise();
        script.alpha();
        script.converters();
        for kind in 0..4 {
            script.inverse(VP8LImageTransformType(kind));
            script.inverse(VP8LImageTransformType(kind));
            script.inverse(VP8LImageTransformType(kind));
        }
    }
    (script.commands, script.expected)
}

#[test]
fn lossless_dsp_matches_the_c_functions() {
    let Some(oracle) = Oracle::locate("lossless_dsp_matches_the_c_functions") else {
        return;
    };
    let (commands, expected) = script();
    let observations = expected.lines().count();
    assert!(
        observations > 2000,
        "the script has {observations} observations"
    );
    let output = oracle
        .probe("wp3", "lossless")
        .run(&[] as &[&str], commands.as_bytes());
    let output = output.success();
    let actual = String::from_utf8(output.stdout.clone()).expect("UTF-8 output");
    let differs = expected
        .lines()
        .zip(actual.lines())
        .position(|(e, a)| e != a)
        .map_or_else(String::new, |index| {
            format!(
                "line {index}: expected {:.160}, the C probe printed {:.160}",
                expected.lines().nth(index).unwrap_or_default(),
                actual.lines().nth(index).unwrap_or_default()
            )
        });
    assert!(
        actual == expected,
        "the lossless DSP differs from C: {differs}"
    );
}
