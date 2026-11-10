//! WP20 parity test: the NEON variants of `src/dsp/lossless_enc_neon.c`
//! against the C translations they replace (`src/dsp/lossless_enc.rs`),
//! Rust against Rust, and the dispatch functions against the C translations,
//! over every count from 0 to 70 and around 128, unaligned starts, buffers
//! that end at the last pixel, multipliers with every edge value and every
//! value of each multiplier against every green and red value.

#![cfg(all(feature = "simd", target_arch = "aarch64"))]

#[path = "support/wp20.rs"]
mod support;

use support::{Rng, widths};
use webp::dsp::lossless::VP8LMultipliers;
use webp::dsp::lossless_enc as c;
/// The NEON level of `src/dsp/lossless_enc_neon.rs`, reached through the dispatch
/// functions, which select its crate-private entry points with `simd` on
/// 64-bit ARM (the residual costs while the `VP8GetCPUInfo` query answers
/// `kNEON`, as it does by default); named as the entry points.
mod neon {
    pub use webp::dsp::lossless_enc::vp8l_subtract_green_from_blue_and_red as subtract_green_from_blue_and_red;
    pub use webp::dsp::lossless_enc::vp8l_transform_color as transform_color;
}

/// Words whose bytes are edge values a quarter of the time.
fn edge_words(rng: &mut Rng, count: usize) -> Vec<u32> {
    rng.edge_bytes(4 * count)
        .as_chunks::<4>()
        .0
        .iter()
        .map(|b| u32::from_le_bytes(*b))
        .collect()
}

#[test]
fn subtract_green_matches_c() {
    let mut rng = Rng(0x2020_0201);
    let mut cases = 0;
    for n in widths().chain([255, 256, 257]) {
        for offset in 0..4 {
            let data = edge_words(&mut rng, offset + n);
            let mut expected = data.clone();
            let mut actual = data.clone();
            let mut dispatched = data;
            c::vp8l_subtract_green_from_blue_and_red_c(&mut expected[offset..], n as i32);
            neon::subtract_green_from_blue_and_red(&mut actual[offset..], n as i32);
            c::vp8l_subtract_green_from_blue_and_red(&mut dispatched[offset..], n as i32);
            assert_eq!(actual, expected, "n {n} offset {offset}");
            assert_eq!(dispatched, expected);
            cases += 1;
        }
    }
    assert_eq!(cases, (widths().count() + 3) * 4);
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

#[test]
fn transform_color_matches_c() {
    let mut rng = Rng(0x2020_0202);
    let all = multipliers(&mut rng);
    let mut cases = 0;
    for m in &all {
        for n in widths() {
            let offset = rng.below(4) as usize;
            let data = edge_words(&mut rng, offset + n);
            let mut expected = data.clone();
            let mut actual = data.clone();
            let mut dispatched = data;
            c::vp8l_transform_color_c(m, &mut expected[offset..], n as i32);
            neon::transform_color(m, &mut actual[offset..], n as i32);
            c::vp8l_transform_color(m, &mut dispatched[offset..], n as i32);
            assert_eq!(actual, expected, "{m:?} n {n}");
            assert_eq!(dispatched, expected);
            cases += 1;
        }
    }
    assert_eq!(cases, all.len() * widths().count());
}

#[test]
fn transform_color_covers_every_green_red_and_multiplier() {
    // Every green and red value against every value of each multiplier.
    let mut rng = Rng(0x2020_0203);
    let pixels: Vec<u32> = (0..65536_u32)
        .map(|gr| {
            ((gr & 0xff) << 8)
                | ((gr >> 8) << 16)
                | u32::from(rng.byte())
                | (u32::from(rng.byte()) << 24)
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
            c::vp8l_transform_color_c(&m, &mut expected, 65536);
            neon::transform_color(&m, &mut actual, 65536);
            assert_eq!(actual, expected, "{m:?}");
        }
    }
}
