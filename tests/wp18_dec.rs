//! WP18 parity test of `src/dsp/dec_sse2.rs` and `dec_sse41.rs` (ADR 0008,
//! parity 1): each dispatch function of `src/dsp/dec.rs` that
//! `VP8DspInitSSE2` or `VP8DspInitSSE41` assigns, run at every SIMD level the
//! host has, against the C translation on copies of the same seeded input.
//!
//! The inputs cover both transform sizes, every predictor mode, the twelve
//! loop filters, unaligned positions in the buffers, flat, stepped and noisy
//! planes, and the full threshold ranges in which upstream's SSE2 and C
//! filters agree (`thresh` 0 to 254, `ithresh` and `hev_thresh` 0 to 255).
//! Transform coefficients stay within ±2048, where the 16-bit lanes of
//! `Transform_SSE2` cannot wrap (`dec_sse2.rs`, excluded behavior 1); the
//! extremes are drawn often.

#![cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]

#[path = "wp18_support/mod.rs"]
mod wp18_support;
#[path = "wp6_support/mod.rs"]
mod wp6_support;

use webp::dsp::dec::{
    vp8_pred_chroma8, vp8_pred_luma4, vp8_pred_luma16, vp8_simple_h_filter16,
    vp8_simple_h_filter16i, vp8_simple_v_filter16, vp8_simple_v_filter16i, vp8_transform,
    vp8_transform_uv, vp8h_filter8, vp8h_filter8i, vp8h_filter16, vp8h_filter16i, vp8v_filter8,
    vp8v_filter8i, vp8v_filter16, vp8v_filter16i,
};
use wp6_support::Rng;
use wp18_support::{Level, Tally, assert_parity, simd_levels};

const BPS: usize = 32;

/// A coefficient within ±2048: often zero, small, or one of the extremes.
fn coefficient(rng: &mut Rng) -> i16 {
    match rng.below(10) {
        0..=2 => 0,
        3..=4 => rng.range(-16, 16) as i16,
        5..=7 => rng.range(-2048, 2048) as i16,
        8 => 2048,
        _ => -2048,
    }
}

fn transforms(levels: &[Level], tally: &mut Tally, rng: &mut Rng) {
    for (name, kind) in [
        ("vp8_transform(do_two)", 0),
        ("vp8_transform", 1),
        ("vp8_transform_uv", 2),
    ] {
        let mut count = 0;
        for case in 0..3000 {
            let in_offset = rng.below(8) as usize;
            let dst_offset = rng.below(16) as usize;
            let mut coeffs: Vec<i16> = (0..in_offset + 64).map(|_| coefficient(rng)).collect();
            if case % 50 == 0 {
                // Every coefficient at one extreme.
                let value = if case % 100 == 0 { 2048 } else { -2048 };
                coeffs.iter_mut().for_each(|c| *c = value);
            }
            let dst = if rng.chance(1, 4) {
                vec![if rng.chance(1, 2) { 0 } else { 255 }; dst_offset + 8 * BPS]
            } else {
                rng.bytes(dst_offset + 8 * BPS)
            };
            assert_parity(
                levels,
                &dst,
                || format!("{name} case {case}"),
                |dst| {
                    let input = &coeffs[in_offset..];
                    let out = &mut dst[dst_offset..];
                    match kind {
                        0 => vp8_transform(input, out, true),
                        1 => vp8_transform(input, out, false),
                        _ => vp8_transform_uv(input, out),
                    }
                },
            );
            count += 1;
        }
        tally.add(name, count, levels);
    }
}

fn predictors(levels: &[Level], tally: &mut Tally, rng: &mut Rng) {
    for (family, modes) in [
        ("vp8_pred_luma4", 10),
        ("vp8_pred_luma16", 7),
        ("vp8_pred_chroma8", 7),
    ] {
        let mut count = 0;
        for mode in 0..modes {
            for case in 0..300 {
                let buf: Vec<u8> = match rng.below(4) {
                    0 => {
                        // Flat or nearly flat neighbors, as in smooth pictures.
                        let base = rng.byte();
                        (0..BPS * 20)
                            .map(|_| base.wrapping_add(rng.range(-2, 2) as u8))
                            .collect()
                    }
                    1 => (0..BPS * 20)
                        .map(|_| if rng.chance(1, 2) { 0 } else { 255 })
                        .collect(),
                    _ => rng.bytes(BPS * 20),
                };
                let pos = BPS * rng.range(1, 2) as usize + rng.range(4, 15) as usize;
                assert_parity(
                    levels,
                    &buf,
                    || format!("{family} mode {mode} case {case} pos {pos}"),
                    |buf| match family {
                        "vp8_pred_luma4" => vp8_pred_luma4(mode, buf, pos),
                        "vp8_pred_luma16" => vp8_pred_luma16(mode, buf, pos),
                        _ => vp8_pred_chroma8(mode, buf, pos),
                    },
                );
                count += 1;
            }
        }
        tally.add(family, count, levels);
    }
}

/// A plane with smooth areas, edges and noise, so that the filters take every
/// branch (the generator of `tests/wp6_dsp.rs`).
fn filter_plane(rng: &mut Rng, size: usize) -> Vec<u8> {
    match rng.below(5) {
        0 => rng.bytes(size),
        1 => {
            let base = rng.range(0, 255);
            let spread = rng.range(0, 8);
            (0..size)
                .map(|_| (base + rng.range(-spread, spread)).clamp(0, 255) as u8)
                .collect()
        }
        2 => {
            // Steps between areas, with noise.
            let a = rng.range(0, 255);
            let b = rng.range(0, 255);
            let step = rng.range(1, 31) as usize;
            (0..size)
                .map(|i| {
                    let base = if (i % 32) < step { a } else { b };
                    (base + rng.range(-3, 3)).clamp(0, 255) as u8
                })
                .collect()
        }
        3 => {
            let a = rng.range(0, 255);
            let b = rng.range(0, 255);
            let row = rng.range(1, 24) as usize;
            let stride = 64;
            (0..size)
                .map(|i| {
                    let base = if i / stride < row { a } else { b };
                    (base + rng.range(-2, 2)).clamp(0, 255) as u8
                })
                .collect()
        }
        _ => (0..size)
            .map(|_| if rng.chance(1, 2) { 0 } else { 255 })
            .collect(),
    }
}

/// `thresh`: mostly the decoder's range (up to `2 * 63 + 63 + 4`), sometimes
/// up to 254, the highest value at which upstream's SSE2 and C agree.
fn threshold(rng: &mut Rng) -> i32 {
    if rng.chance(1, 5) {
        rng.range(0, 254) as i32
    } else {
        rng.range(0, 193) as i32
    }
}

/// `ithresh` and `hev_thresh`: mostly the decoder's ranges, sometimes any byte.
fn small_threshold(rng: &mut Rng, decoder_max: i64) -> i32 {
    if rng.chance(1, 5) {
        rng.range(0, 255) as i32
    } else {
        rng.range(0, decoder_max) as i32
    }
}

fn filters(levels: &[Level], tally: &mut Tally, rng: &mut Rng) {
    for name in ["v16", "h16", "v16i", "h16i"] {
        let mut count = 0;
        for case in 0..1500 {
            let stride = rng.range(32, 64) as usize;
            let buf = filter_plane(rng, stride * 26);
            let pos = 5 * stride + rng.range(4, 12) as usize;
            let thresh = threshold(rng);
            let s = stride as i32;
            assert_parity(
                levels,
                &buf,
                || format!("simple {name} case {case} stride {stride} thresh {thresh}"),
                |buf| match name {
                    "v16" => vp8_simple_v_filter16(buf, pos, s, thresh),
                    "h16" => vp8_simple_h_filter16(buf, pos, s, thresh),
                    "v16i" => vp8_simple_v_filter16i(buf, pos, s, thresh),
                    _ => vp8_simple_h_filter16i(buf, pos, s, thresh),
                },
            );
            count += 1;
        }
        tally.add(&format!("vp8_simple_{name}"), count, levels);
    }
    for name in ["v16", "h16", "v16i", "h16i"] {
        let mut count = 0;
        for case in 0..1500 {
            let stride = rng.range(32, 64) as usize;
            let buf = filter_plane(rng, stride * 26);
            let pos = 5 * stride + rng.range(4, 12) as usize;
            let thresh = threshold(rng);
            let ithresh = small_threshold(rng, 63);
            let hev = small_threshold(rng, 2);
            let s = stride as i32;
            assert_parity(
                levels,
                &buf,
                || format!("luma {name} case {case} {thresh} {ithresh} {hev}"),
                |buf| match name {
                    "v16" => vp8v_filter16(buf, pos, s, thresh, ithresh, hev),
                    "h16" => vp8h_filter16(buf, pos, s, thresh, ithresh, hev),
                    "v16i" => vp8v_filter16i(buf, pos, s, thresh, ithresh, hev),
                    _ => vp8h_filter16i(buf, pos, s, thresh, ithresh, hev),
                },
            );
            count += 1;
        }
        tally.add(&format!("vp8{name}"), count, levels);
    }
    for name in ["v8", "h8", "v8i", "h8i"] {
        let mut count = 0;
        for case in 0..1500 {
            let stride = rng.range(16, 40) as usize;
            let u = filter_plane(rng, stride * 18);
            let v = filter_plane(rng, stride * 18);
            let upos = 5 * stride + rng.range(4, 8) as usize;
            let vpos = 5 * stride + rng.range(4, 8) as usize;
            let thresh = threshold(rng);
            let ithresh = small_threshold(rng, 63);
            let hev = small_threshold(rng, 2);
            let s = stride as i32;
            assert_parity(
                levels,
                &(u, v),
                || format!("chroma {name} case {case} {thresh} {ithresh} {hev}"),
                |(u, v)| match name {
                    "v8" => vp8v_filter8(u, upos, v, vpos, s, thresh, ithresh, hev),
                    "h8" => vp8h_filter8(u, upos, v, vpos, s, thresh, ithresh, hev),
                    "v8i" => vp8v_filter8i(u, upos, v, vpos, s, thresh, ithresh, hev),
                    _ => vp8h_filter8i(u, upos, v, vpos, s, thresh, ithresh, hev),
                },
            );
            count += 1;
        }
        tally.add(&format!("vp8{name}"), count, levels);
    }
}

#[test]
fn decoder_simd_matches_the_c_translation() {
    let levels = simd_levels();
    assert!(!levels.is_empty(), "an x86 host has SSE2");
    let mut rng = Rng(0x5750_3138_4445_43);
    let mut tally = Tally::default();
    transforms(&levels, &mut tally, &mut rng);
    predictors(&levels, &mut tally, &mut rng);
    filters(&levels, &mut tally, &mut rng);
    tally.print("wp18_dec");
}
