//! WP20 parity test: the NEON variants of `src/dsp/lossless_neon.c` against
//! the C translations they replace (`src/dsp/lossless.rs`), Rust against
//! Rust, and the dispatch functions against the C translations. The
//! one-pixel predictors run over every byte combination of their channels;
//! the batch predictors, the inverse transforms and the converters run over
//! every count from 0 to 70 and around 128, in the buffer layouts the decoder
//! uses (separate rows, in place, the row above the output) with buffers that
//! end at the last pixel C touches.

#![cfg(all(feature = "simd", target_arch = "aarch64"))]

#[path = "support/wp20.rs"]
mod support;

use support::{Rng, widths};
use webp::dsp::lossless::{self as c, VP8LMultipliers, VP8LPredictorAddSubFunc, VP8LPredictorFunc};
use webp::dsp::lossless_neon as neon;

/// Words whose bytes are edge values a quarter of the time.
fn edge_words(rng: &mut Rng, count: usize) -> Vec<u32> {
    rng.edge_bytes(4 * count)
        .as_chunks::<4>()
        .0
        .iter()
        .map(|b| u32::from_le_bytes(*b))
        .collect()
}

/// Packs four byte values into a pixel, one per channel.
fn pack(bytes: [u32; 4]) -> u32 {
    bytes[0] | (bytes[1] << 8) | (bytes[2] << 16) | (bytes[3] << 24)
}

/// The NEON one-pixel predictors with the C functions they replace.
fn one_pixel_predictors() -> [(usize, VP8LPredictorFunc, VP8LPredictorFunc); 4] {
    [
        (5, neon::predictor5, c::vp8l_predictor5_c),
        (6, neon::predictor6, c::vp8l_predictor6_c),
        (7, neon::predictor7, c::vp8l_predictor7_c),
        (13, neon::predictor13, c::vp8l_predictor13_c),
    ]
}

#[test]
fn one_pixel_predictors_cover_every_channel_triple() {
    // Each call carries four (left, top, top-left or top-right) byte triples,
    // one per channel; the 2^24 triples take 2^22 calls per predictor.
    for (mode, predictor_neon, predictor_c) in one_pixel_predictors() {
        let mut calls = 0_u32;
        for base in (0..1_u32 << 24).step_by(4) {
            let mut left = [0_u32; 4];
            let mut top = [0_u32; 4];
            let mut other = [0_u32; 4];
            for k in 0..4 {
                let triple = base + k as u32;
                left[k] = triple & 0xff;
                top[k] = (triple >> 8) & 0xff;
                other[k] = triple >> 16;
            }
            // `top` holds top[-1], top[0] and top[1]; the predictors read the
            // other value at -1 (modes 6 and 13) or +1 (mode 5).
            let row = [pack(other), pack(top), pack(other)];
            let left = [pack(left)];
            let expected = predictor_c(&left, 0, &row, 1);
            assert_eq!(predictor_neon(&left, 0, &row, 1), expected, "mode {mode}");
            assert_eq!(c::vp8l_predictors(mode, &left, 0, &row, 1), expected);
            calls += 1;
        }
        assert_eq!(calls, 1 << 22);
    }
}

/// The NEON batch predictors of `VP8LPredictorsAdd[0..14]`.
fn batch_predictors() -> [VP8LPredictorAddSubFunc; 14] {
    [
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
    ]
}

/// A buffer layout of one batch-predictor call: the data and the positions
/// of `in`, `upper` and `out`.
struct Layout {
    data: Vec<u32>,
    in_pos: usize,
    upper_pos: usize,
    out_pos: usize,
}

/// The layouts for `n` pixels: separate regions in both orders, ending at
/// the last pixel C reads or writes (`upper[n]` or `out[n - 1]`); `in` equal
/// to `out`; and the decoder's rows, where `upper` is the row above `out`
/// (`width` pixels before it) and `in` is a row of its own or `out` itself.
fn layouts(rng: &mut Rng, n: usize) -> Vec<Layout> {
    let mut layouts = Vec::new();
    // [upper[-1..=n]] [in[0..n]] [out[-1..n]]
    let data = edge_words(rng, (n + 2) + n + (n + 1));
    layouts.push(Layout {
        data,
        upper_pos: 1,
        in_pos: n + 2,
        out_pos: n + 2 + n + 1,
    });
    // [out[-1..n]] [in[0..n]] [upper[-1..=n]]
    let data = edge_words(rng, (n + 1) + n + (n + 2));
    layouts.push(Layout {
        data,
        out_pos: 1,
        in_pos: n + 1,
        upper_pos: n + 1 + n + 1,
    });
    // In place: [upper[-1..=n]] [out[-1..n] = in]
    let data = edge_words(rng, (n + 2) + (n + 1));
    layouts.push(Layout {
        data,
        upper_pos: 1,
        in_pos: n + 3,
        out_pos: n + 3,
    });
    // The decoder's rows: `width = n + 1 + pad` pixels, the output from
    // column 1 of row 1, the row above it as `upper`; `in` is row 2 or `out`.
    for pad in [0, 3] {
        let width = n + 1 + pad;
        for in_place in [false, true] {
            let data = edge_words(rng, 3 * width);
            let out_pos = width + 1;
            layouts.push(Layout {
                data,
                upper_pos: 1,
                in_pos: if in_place { out_pos } else { 2 * width + 1 },
                out_pos,
            });
        }
    }
    layouts
}

#[test]
fn batch_predictors_match_c() {
    let mut rng = Rng(0x2020_0101);
    let neon_predictors = batch_predictors();
    let mut cases = 0;
    for n in widths() {
        for layout in layouts(&mut rng, n) {
            for mode in 0..16 {
                let num = n as i32;
                let mut expected = layout.data.clone();
                c::vp8l_predictors_add_c(
                    mode,
                    &mut expected,
                    layout.in_pos,
                    layout.upper_pos,
                    num,
                    layout.out_pos,
                );
                if let Some(predictor_add) = neon_predictors.get(mode) {
                    let mut actual = layout.data.clone();
                    predictor_add(
                        &mut actual,
                        layout.in_pos,
                        layout.upper_pos,
                        num,
                        layout.out_pos,
                    );
                    assert_eq!(actual, expected, "mode {mode} n {n}");
                }
                let mut dispatched = layout.data.clone();
                c::vp8l_predictors_add(
                    mode,
                    &mut dispatched,
                    layout.in_pos,
                    layout.upper_pos,
                    num,
                    layout.out_pos,
                );
                assert_eq!(dispatched, expected, "dispatch mode {mode} n {n}");
                cases += 1;
            }
        }
    }
    assert_eq!(cases, widths().count() * 7 * 16);
}

#[test]
fn batch_predictors_cover_the_channel_extremes() {
    // Rows of only 0x00 and 0xff bytes reach the saturation of the clamped
    // predictors (12 and 13) and the sign edges of their differences.
    let mut rng = Rng(0x2020_0102);
    let neon_predictors = batch_predictors();
    for n in [4, 8, 13, 64] {
        for _ in 0..200 {
            let mut data: Vec<u32> = (0..3 * (n + 2))
                .map(|_| {
                    let mut word = 0;
                    for k in 0..4 {
                        if rng.below(2) == 1 {
                            word |= 0xff << (8 * k);
                        }
                    }
                    word
                })
                .collect();
            data[0] = rng.next_u64() as u32;
            for (mode, predictor_add) in neon_predictors.iter().enumerate() {
                let (upper_pos, in_pos, out_pos) = (1, n + 2, 2 * (n + 2) + 1);
                let mut expected = data.clone();
                let mut actual = data.clone();
                c::vp8l_predictors_add_c(mode, &mut expected, in_pos, upper_pos, n as i32, out_pos);
                predictor_add(&mut actual, in_pos, upper_pos, n as i32, out_pos);
                assert_eq!(actual, expected, "mode {mode} n {n}");
            }
        }
    }
}

/// Multipliers with every edge value (0, 1, 127, 128, 129, 255) in each
/// field, and seeded ones.
fn multipliers(rng: &mut Rng) -> Vec<VP8LMultipliers> {
    const EDGES: [u8; 6] = [0, 1, 127, 128, 129, 255];
    let mut all = Vec::new();
    for &green_to_red in &EDGES {
        for &green_to_blue in &EDGES {
            for &red_to_blue in &EDGES {
                all.push(VP8LMultipliers {
                    green_to_red,
                    green_to_blue,
                    red_to_blue,
                });
            }
        }
    }
    for _ in 0..40 {
        all.push(VP8LMultipliers {
            green_to_red: rng.byte(),
            green_to_blue: rng.byte(),
            red_to_blue: rng.byte(),
        });
    }
    all
}

/// The positions of `src` and `dst` for `n` pixels: in place, `src` before
/// `dst`, `dst` before `src`, each region exactly `n` pixels.
fn transform_layouts(n: usize) -> [(usize, usize, usize); 3] {
    [(n, 0, 0), (2 * n, 0, n), (2 * n, n, 0)]
}

#[test]
fn add_green_to_blue_and_red_matches_c() {
    let mut rng = Rng(0x2020_0103);
    for n in widths() {
        for (len, src_pos, dst_pos) in transform_layouts(n) {
            for offset in 0..4 {
                let data = edge_words(&mut rng, offset + len);
                let num = n as i32;
                let mut expected = data.clone();
                let mut actual = data.clone();
                let mut dispatched = data;
                c::vp8l_add_green_to_blue_and_red_c(
                    &mut expected,
                    offset + src_pos,
                    num,
                    offset + dst_pos,
                );
                neon::add_green_to_blue_and_red(
                    &mut actual,
                    offset + src_pos,
                    num,
                    offset + dst_pos,
                );
                c::vp8l_add_green_to_blue_and_red(
                    &mut dispatched,
                    offset + src_pos,
                    num,
                    offset + dst_pos,
                );
                assert_eq!(actual, expected, "n {n} src {src_pos} dst {dst_pos}");
                assert_eq!(dispatched, expected);
            }
        }
    }
}

#[test]
fn transform_color_inverse_matches_c() {
    let mut rng = Rng(0x2020_0104);
    let all = multipliers(&mut rng);
    let mut cases = 0;
    for m in &all {
        for n in widths() {
            for (len, src_pos, dst_pos) in transform_layouts(n) {
                let offset = rng.below(4) as usize;
                let data = edge_words(&mut rng, offset + len);
                let num = n as i32;
                let (src, dst) = (offset + src_pos, offset + dst_pos);
                let mut expected = data.clone();
                let mut actual = data.clone();
                let mut dispatched = data;
                c::vp8l_transform_color_inverse_c(m, &mut expected, src, num, dst);
                neon::transform_color_inverse(m, &mut actual, src, num, dst);
                c::vp8l_transform_color_inverse(m, &mut dispatched, src, num, dst);
                assert_eq!(actual, expected, "{m:?} n {n} src {src_pos} dst {dst_pos}");
                assert_eq!(dispatched, expected);
                cases += 1;
            }
        }
    }
    assert_eq!(cases, all.len() * widths().count() * 3);
}

#[test]
fn transform_color_inverse_covers_every_green_and_multiplier() {
    // Every green and red value against every value of each multiplier.
    let mut rng = Rng(0x2020_0105);
    let pixels: Vec<u32> = (0..65536_u32)
        .map(|gr| {
            ((gr & 0xff) << 8) | ((gr >> 8) << 16) | (rng.byte() as u32) | (rng.byte() as u32) << 24
        })
        .collect();
    for value in 0..=255_u8 {
        for m in [
            VP8LMultipliers {
                green_to_red: value,
                green_to_blue: rng.byte(),
                red_to_blue: rng.byte(),
            },
            VP8LMultipliers {
                green_to_red: rng.byte(),
                green_to_blue: value,
                red_to_blue: rng.byte(),
            },
            VP8LMultipliers {
                green_to_red: rng.byte(),
                green_to_blue: rng.byte(),
                red_to_blue: value,
            },
        ] {
            let mut expected = pixels.clone();
            let mut actual = pixels.clone();
            c::vp8l_transform_color_inverse_c(&m, &mut expected, 0, 65536, 0);
            neon::transform_color_inverse(&m, &mut actual, 0, 65536, 0);
            assert_eq!(actual, expected, "{m:?}");
        }
    }
}

type Converter = fn(&[u32], i32, &mut [u8]);

#[test]
fn converters_match_c() {
    let converters: [(&str, usize, Converter, Converter, Converter); 3] = [
        (
            "rgba",
            4,
            neon::convert_bgra_to_rgba,
            c::vp8l_convert_bgra_to_rgba_c,
            c::vp8l_convert_bgra_to_rgba,
        ),
        (
            "bgr",
            3,
            neon::convert_bgra_to_bgr,
            c::vp8l_convert_bgra_to_bgr_c,
            c::vp8l_convert_bgra_to_bgr,
        ),
        (
            "rgb",
            3,
            neon::convert_bgra_to_rgb,
            c::vp8l_convert_bgra_to_rgb_c,
            c::vp8l_convert_bgra_to_rgb,
        ),
    ];
    let mut rng = Rng(0x2020_0106);
    for (name, bytes, convert_neon, convert_c, dispatch) in converters {
        for n in widths().chain([255, 256, 257]) {
            for offset in 0..4 {
                let src = edge_words(&mut rng, offset + n);
                let dst = rng.bytes(offset + bytes * n);
                let mut expected = dst.clone();
                let mut actual = dst.clone();
                let mut dispatched = dst;
                convert_c(&src[offset..], n as i32, &mut expected[offset..]);
                convert_neon(&src[offset..], n as i32, &mut actual[offset..]);
                dispatch(&src[offset..], n as i32, &mut dispatched[offset..]);
                assert_eq!(actual, expected, "{name} n {n} offset {offset}");
                assert_eq!(dispatched, expected);
            }
        }
    }
}
