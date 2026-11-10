//! WP18 parity test of `src/dsp/filters_sse2.rs` (ADR 0008, parity 1): the
//! table entries of `webp_filters` and `webp_unfilters` in `src/dsp/filters.rs`
//! that `VP8FiltersInitSSE2` assigns, run at every SIMD level the host has,
//! against the C translation on copies of the same seeded input.
//!
//! The forward filters run over planes of widths 1 to 70 and around the vector
//! widths (8, 16, 32 and their multiples), one to six rows, strides with and
//! without padding and unaligned starts, on noisy, smooth and saturated
//! planes. The unfilters run over the same widths with and without a previous
//! row, and with the output starting as random bytes, as a copy of the input
//! (the in-place form of C) or as a copy of the previous row (C's previous
//! row equal to the output), which are the shapes the Rust signature allows
//! (WP4, deviation 15). Widths start at 1: upstream's SSE2 unfilters write
//! `out[0]` at width 0 where C writes nothing (`filters_sse2.rs`, excluded
//! behavior 1); the vertical unfilter with a previous row, where both agree,
//! also runs at width 0.

#![cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]

#[path = "wp18_support/mod.rs"]
mod wp18_support;
#[path = "wp6_support/mod.rs"]
mod wp6_support;

use webp::dsp::dsp::WEBP_FILTER_TYPE;
use webp::dsp::filters::{webp_filters, webp_unfilters};
use wp6_support::Rng;
use wp18_support::{Level, Tally, assert_parity, simd_levels};

/// The widths: every width up to 70 and the neighbors of the larger vector
/// multiples.
fn widths() -> Vec<usize> {
    let mut widths: Vec<usize> = (1..=70).collect();
    widths.extend([95, 96, 97, 127, 128, 129, 255, 256, 257]);
    widths
}

/// A plane of `size` bytes: noisy, smooth, saturated or with steps, so that
/// the gradient predictor clips in both directions.
fn plane(rng: &mut Rng, size: usize) -> Vec<u8> {
    match rng.below(4) {
        0 => rng.bytes(size),
        1 => {
            let base = rng.range(0, 255);
            (0..size)
                .map(|_| (base + rng.range(-3, 3)).clamp(0, 255) as u8)
                .collect()
        }
        2 => (0..size)
            .map(|_| if rng.chance(1, 2) { 0 } else { 255 })
            .collect(),
        _ => {
            let a = rng.byte();
            let b = rng.byte();
            let step = rng.range(1, 40) as usize;
            (0..size)
                .map(|i| if i % 41 < step { a } else { b })
                .collect()
        }
    }
}

const NAMES: [&str; 4] = ["none", "horizontal", "vertical", "gradient"];

fn filters(levels: &[Level], tally: &mut Tally, rng: &mut Rng) {
    for filter in 1..=3_u32 {
        let mut count = 0;
        for width in widths() {
            for height in [1, 2, 3, 6] {
                for case in 0..2 {
                    let stride = width + rng.below(4) as usize;
                    let offset = rng.below(16) as usize;
                    let data = plane(rng, offset + stride * (height - 1) + width);
                    let out_offset = rng.below(16) as usize;
                    let out = rng.bytes(out_offset + stride * (height - 1) + width);
                    let filter_type = WEBP_FILTER_TYPE(filter);
                    assert_parity(
                        levels,
                        &out,
                        || {
                            format!(
                                "webp_filters {} {width}x{height} stride {stride} case {case}",
                                NAMES[filter as usize]
                            )
                        },
                        |out| {
                            webp_filters(
                                filter_type,
                                &data[offset..],
                                width as i32,
                                height as i32,
                                stride as i32,
                                &mut out[out_offset..],
                            );
                        },
                    );
                    count += 1;
                }
            }
        }
        tally.add(
            &format!("webp_filters({})", NAMES[filter as usize]),
            count,
            levels,
        );
    }
}

fn unfilters(levels: &[Level], tally: &mut Tally, rng: &mut Rng) {
    for filter in 0..=3_u32 {
        let mut count = 0;
        let mut all_widths = widths();
        if filter == 2 {
            all_widths.insert(0, 0);
        }
        for width in all_widths {
            for has_prev in [false, true] {
                if width == 0 && !has_prev {
                    continue;
                }
                for form in 0..3 {
                    if form == 2 && !has_prev {
                        continue;
                    }
                    for case in 0..3 {
                        let prev_offset = rng.below(16) as usize;
                        let in_offset = rng.below(16) as usize;
                        let out_offset = rng.below(16) as usize;
                        let prev = plane(rng, prev_offset + width);
                        let input = plane(rng, in_offset + width);
                        let mut out = rng.bytes(out_offset + width);
                        match form {
                            // In place: the output starts as the input.
                            1 => out[out_offset..].copy_from_slice(&input[in_offset..]),
                            // The previous row is the output.
                            2 => out[out_offset..].copy_from_slice(&prev[prev_offset..]),
                            _ => {}
                        }
                        let filter_type = WEBP_FILTER_TYPE(filter);
                        assert_parity(
                            levels,
                            &out,
                            || {
                                format!(
                                    "webp_unfilters {} width {width} prev {has_prev} form {form} case {case}",
                                    NAMES[filter as usize]
                                )
                            },
                            |out| {
                                webp_unfilters(
                                    filter_type,
                                    has_prev.then(|| &prev[prev_offset..]),
                                    &input[in_offset..],
                                    &mut out[out_offset..],
                                    width as i32,
                                );
                            },
                        );
                        count += 1;
                    }
                }
            }
        }
        tally.add(
            &format!("webp_unfilters({})", NAMES[filter as usize]),
            count,
            levels,
        );
    }
}

#[test]
fn filters_simd_matches_the_c_translation() {
    let levels = simd_levels();
    assert!(!levels.is_empty(), "an x86 host has SSE2");
    let mut rng = Rng(0x5750_3138_4649_4c54);
    let mut tally = Tally::default();
    filters(&levels, &mut tally, &mut rng);
    unfilters(&levels, &mut tally, &mut rng);
    tally.print("wp18_filters");
}
