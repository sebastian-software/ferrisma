//! WP18 parity test of `src/dsp/upsampling_sse2.rs` and
//! `upsampling_sse41.rs` (ADR 0008, parity 1): `webp_upsamplers`,
//! `webp_get_line_pair_converter` and `webp_yuv444_converters` of
//! `src/dsp/upsampling.rs`, run at every SIMD level the host has, against the
//! C translation on copies of the same seeded input.
//!
//! The upsamplers run in all eleven RGB modes, with and without the bottom
//! row, over lengths 1 to 100 and around the 32-pixel blocks of the SIMD loop
//! and its last block (up to 258); the converters over lengths 0 to 100 and
//! around the 32-pixel blocks. Every slice has exactly the length the C
//! function reads or writes, at an unaligned position, so a SIMD access past
//! the scalar footprint panics. Planes are random, flat, black, white or
//! alternating extremes.

#![cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]

#[path = "wp18_support/mod.rs"]
mod wp18_support;
#[path = "wp6_support/mod.rs"]
mod wp6_support;

use webp::dsp::upsampling::{
    webp_get_line_pair_converter, webp_upsamplers, webp_yuv444_converters,
};
use webp::webp::decode::WEBP_CSP_MODE;
use wp6_support::Rng;
use wp18_support::{Level, Tally, assert_parity, at_level, simd_levels};

/// The modes with an upsampler and a converter, `MODE_RGB` to `MODE_rgbA_4444`.
const MODES: u32 = 11;

/// Lengths up to 100 and around the 32-pixel blocks beyond it.
fn lengths(from: usize) -> Vec<usize> {
    let mut lengths: Vec<usize> = (from..=100).collect();
    lengths.extend([127, 128, 129, 130, 159, 160, 161, 162, 255, 256, 257, 258]);
    lengths
}

/// Output bytes per pixel of a mode.
fn bytes_per_pixel(mode: u32) -> usize {
    match mode {
        0 | 2 => 3,
        5 | 6 | 10 => 2,
        _ => 4,
    }
}

/// `count` samples after `offset` filler bytes: random, flat, black, white,
/// or alternating extremes.
fn plane(rng: &mut Rng, offset: usize, count: usize) -> Vec<u8> {
    let mut buffer = rng.bytes(offset);
    let samples: Vec<u8> = match rng.below(6) {
        0 => vec![rng.byte(); count],
        1 => vec![0; count],
        2 => vec![255; count],
        3 => (0..count)
            .map(|_| if rng.chance(1, 2) { 0 } else { 255 })
            .collect(),
        _ => rng.bytes(count),
    };
    buffer.extend(samples);
    buffer
}

fn offset(rng: &mut Rng) -> usize {
    rng.below(16) as usize
}

fn int(value: usize) -> i32 {
    i32::try_from(value).expect("a length that fits in int")
}

/// The inputs of one line pair: the luma rows and the four chroma rows, each
/// with its offset.
struct LinePair {
    top_y: (Vec<u8>, usize),
    bottom_y: (Vec<u8>, usize),
    top_u: (Vec<u8>, usize),
    top_v: (Vec<u8>, usize),
    cur_u: (Vec<u8>, usize),
    cur_v: (Vec<u8>, usize),
}

impl LinePair {
    fn new(rng: &mut Rng, len: usize) -> Self {
        let uv_len = len.div_ceil(2);
        let mut row = |count: usize| {
            let o = offset(rng);
            (plane(rng, o, count), o)
        };
        Self {
            top_y: row(len),
            bottom_y: row(len),
            top_u: row(uv_len),
            top_v: row(uv_len),
            cur_u: row(uv_len),
            cur_v: row(uv_len),
        }
    }
}

fn s(row: &(Vec<u8>, usize)) -> &[u8] {
    &row.0[row.1..]
}

fn upsamplers(levels: &[Level], tally: &mut Tally, rng: &mut Rng) {
    for mode in 0..MODES {
        let xstep = bytes_per_pixel(mode);
        let mut count = 0;
        for len in lengths(1) {
            for with_bottom in [false, true] {
                for _ in 0..3 {
                    let pair = LinePair::new(rng, len);
                    let (top_off, bottom_off) = (offset(rng), offset(rng));
                    let dst = (
                        rng.bytes(top_off + len * xstep),
                        rng.bytes(bottom_off + len * xstep),
                    );
                    assert_parity(
                        levels,
                        &dst,
                        || format!("webp_upsamplers mode {mode} len {len} bottom {with_bottom}"),
                        |(top_dst, bottom_dst)| {
                            let bottom = with_bottom
                                .then(|| (s(&pair.bottom_y), &mut bottom_dst[bottom_off..]));
                            webp_upsamplers(
                                WEBP_CSP_MODE(mode),
                                s(&pair.top_y),
                                bottom,
                                s(&pair.top_u),
                                s(&pair.top_v),
                                s(&pair.cur_u),
                                s(&pair.cur_v),
                                &mut top_dst[top_off..],
                                int(len),
                            );
                        },
                    );
                    count += 1;
                }
            }
        }
        tally.add(&format!("webp_upsamplers mode {mode}"), count, levels);
    }

    let mut count = 0;
    for alpha_is_last in [false, true] {
        for len in lengths(1) {
            let pair = LinePair::new(rng, len);
            let dst = (rng.bytes(4 * len), rng.bytes(4 * len));
            let mut expected = dst.clone();
            at_level(Level::C, || {
                let converter = webp_get_line_pair_converter(alpha_is_last);
                let (top_dst, bottom_dst) = &mut expected;
                converter(
                    s(&pair.top_y),
                    Some((s(&pair.bottom_y), bottom_dst)),
                    s(&pair.top_u),
                    s(&pair.top_v),
                    s(&pair.cur_u),
                    s(&pair.cur_v),
                    top_dst,
                    int(len),
                );
            });
            for &level in levels {
                let mut actual = dst.clone();
                at_level(level, || {
                    let converter = webp_get_line_pair_converter(alpha_is_last);
                    let (top_dst, bottom_dst) = &mut actual;
                    converter(
                        s(&pair.top_y),
                        Some((s(&pair.bottom_y), bottom_dst)),
                        s(&pair.top_u),
                        s(&pair.top_v),
                        s(&pair.cur_u),
                        s(&pair.cur_v),
                        top_dst,
                        int(len),
                    );
                });
                assert!(
                    actual == expected,
                    "webp_get_line_pair_converter({alpha_is_last}) len {len}: {level:?} differs from C"
                );
            }
            count += 1;
        }
    }
    tally.add("webp_get_line_pair_converter", count, levels);
}

fn converters(levels: &[Level], tally: &mut Tally, rng: &mut Rng) {
    for mode in 0..MODES {
        let xstep = bytes_per_pixel(mode);
        let mut count = 0;
        for len in lengths(0) {
            for _ in 0..4 {
                let (yo, uo, vo, d) = (offset(rng), offset(rng), offset(rng), offset(rng));
                let y = plane(rng, yo, len);
                let u = plane(rng, uo, len);
                let v = plane(rng, vo, len);
                let dst = rng.bytes(d + len * xstep);
                assert_parity(
                    levels,
                    &dst,
                    || format!("webp_yuv444_converters mode {mode} len {len}"),
                    |dst| {
                        webp_yuv444_converters(
                            WEBP_CSP_MODE(mode),
                            &y[yo..],
                            &u[uo..],
                            &v[vo..],
                            &mut dst[d..],
                            int(len),
                        );
                    },
                );
                count += 1;
            }
        }
        tally.add(
            &format!("webp_yuv444_converters mode {mode}"),
            count,
            levels,
        );
    }
}

#[test]
fn upsampling_simd_matches_the_c_translation() {
    let levels = simd_levels();
    assert!(!levels.is_empty(), "an x86 host has SSE2");
    let mut rng = Rng(0x0057_5031_3855_5053);
    let mut tally = Tally::default();
    upsamplers(&levels, &mut tally, &mut rng);
    converters(&levels, &mut tally, &mut rng);
    tally.print("wp18_upsampling");
}
