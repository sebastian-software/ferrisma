//! WP18 parity test of `src/dsp/yuv_sse2.rs` and `yuv_sse41.rs` (ADR 0008,
//! parity 1): the dispatch functions of `src/dsp/yuv.rs` that
//! `WebPInitSamplersSSE2/SSE41` and `WebPInitConvertARGBToYUVSSE2/SSE41`
//! assign, run at every SIMD level the host has, against the C translation on
//! copies of the same seeded input.
//!
//! The row samplers run in all eleven RGB modes over lengths 0 to 100 and
//! around the 8- and 32-pixel blocks of the SIMD loops (up to 257), with
//! unaligned positions and slices of exactly the length the C function reads
//! or writes, so a SIMD access past the scalar footprint panics. Planes are
//! random, flat, black, white or alternating extremes. The converters run over
//! the same widths; `WebPConvertARGBToUV` with both store modes over random
//! previous U and V, and `WebPConvertRGBA32ToUV` over the accumulated sums of
//! up to four pixels (0 to 1020) that `picture_csp_enc.c` passes, and
//! occasionally over larger values up to 32767, the range in which the 16-bit
//! lanes of `_mm_madd_epi16` read them as the C `int` arithmetic does.

#![cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]

#[path = "wp18_support/mod.rs"]
mod wp18_support;
#[path = "wp6_support/mod.rs"]
mod wp6_support;

use webp::dsp::yuv::{
    webp_convert_argb_to_uv, webp_convert_argb_to_y, webp_convert_bgr24_to_y,
    webp_convert_rgb24_to_y, webp_convert_rgba32_to_uv, webp_samplers,
};
use webp::webp::decode::WEBP_CSP_MODE;
use wp6_support::Rng;
use wp18_support::{Level, Tally, assert_parity, simd_levels};

/// The sampler modes, `MODE_RGB` through `MODE_rgbA_4444`.
const SAMPLER_MODES: u32 = 11;

/// Row lengths: every length up to 100 and the lengths around the 8- and
/// 32-pixel blocks of the SIMD loops beyond it.
fn lengths() -> Vec<usize> {
    let mut lengths: Vec<usize> = (0..=100).collect();
    lengths.extend([127, 128, 129, 159, 160, 161, 191, 192, 193, 255, 256, 257]);
    lengths
}

/// Output bytes per pixel of a sampler mode.
fn bytes_per_pixel(mode: u32) -> usize {
    match mode {
        0 | 2 => 3,
        5 | 6 | 10 => 2,
        _ => 4,
    }
}

/// `count` samples: random, flat, black, white, or alternating extremes.
fn plane(rng: &mut Rng, count: usize) -> Vec<u8> {
    match rng.below(6) {
        0 => vec![rng.byte(); count],
        1 => vec![0; count],
        2 => vec![255; count],
        3 => (0..count)
            .map(|_| if rng.chance(1, 2) { 0 } else { 255 })
            .collect(),
        _ => rng.bytes(count),
    }
}

/// `count` bytes after `offset` filler bytes.
fn padded(rng: &mut Rng, offset: usize, count: usize) -> Vec<u8> {
    let mut buffer = rng.bytes(offset);
    buffer.extend(plane(rng, count));
    buffer
}

fn int(value: usize) -> i32 {
    i32::try_from(value).expect("a length that fits in int")
}

fn samplers(levels: &[Level], tally: &mut Tally, rng: &mut Rng) {
    for mode in 0..SAMPLER_MODES {
        let mut count = 0;
        let xstep = bytes_per_pixel(mode);
        for len in lengths() {
            for _ in 0..4 {
                let (yo, uo, vo, d) = (
                    rng.below(16) as usize,
                    rng.below(16) as usize,
                    rng.below(16) as usize,
                    rng.below(16) as usize,
                );
                let uv_len = len.div_ceil(2);
                let y = padded(rng, yo, len);
                let u = padded(rng, uo, uv_len);
                let v = padded(rng, vo, uv_len);
                let dst = rng.bytes(d + len * xstep);
                assert_parity(
                    levels,
                    &dst,
                    || format!("webp_samplers mode {mode} len {len}"),
                    |dst| {
                        webp_samplers(
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
        tally.add(&format!("webp_samplers mode {mode}"), count, levels);
    }
}

fn argb(rng: &mut Rng, offset: usize, count: usize) -> Vec<u32> {
    let kind = rng.below(4);
    (0..offset + count)
        .map(|_| match kind {
            0 => 0,
            1 => u32::MAX,
            2 => {
                if rng.chance(1, 2) {
                    0xff00_ff00
                } else {
                    0x00ff_00ff
                }
            }
            _ => rng.next_u64() as u32,
        })
        .collect()
}

fn converters(levels: &[Level], tally: &mut Tally, rng: &mut Rng) {
    let mut count = 0;
    for width in lengths() {
        for _ in 0..4 {
            let (s, d) = (rng.below(8) as usize, rng.below(16) as usize);
            let src = argb(rng, s, width);
            let y = rng.bytes(d + width);
            assert_parity(
                levels,
                &y,
                || format!("webp_convert_argb_to_y width {width}"),
                |y| webp_convert_argb_to_y(&src[s..], &mut y[d..], int(width)),
            );
            count += 1;
        }
    }
    tally.add("webp_convert_argb_to_y", count, levels);

    let mut count = 0;
    for width in lengths() {
        for do_store in [0, 1] {
            for _ in 0..4 {
                let (s, uo, vo) = (
                    rng.below(8) as usize,
                    rng.below(16) as usize,
                    rng.below(16) as usize,
                );
                let src = argb(rng, s, width);
                let uv_width = width.div_ceil(2);
                let u = padded(rng, uo, uv_width);
                let v = padded(rng, vo, uv_width);
                assert_parity(
                    levels,
                    &(u, v),
                    || format!("webp_convert_argb_to_uv width {width} do_store {do_store}"),
                    |(u, v)| {
                        webp_convert_argb_to_uv(
                            &src[s..],
                            &mut u[uo..],
                            &mut v[vo..],
                            int(width),
                            do_store,
                        );
                    },
                );
                count += 1;
            }
        }
    }
    tally.add("webp_convert_argb_to_uv", count, levels);

    for bgr in [false, true] {
        let mut count = 0;
        for width in lengths() {
            for _ in 0..4 {
                let (s, d) = (rng.below(16) as usize, rng.below(16) as usize);
                let src = padded(rng, s, 3 * width);
                let y = rng.bytes(d + width);
                assert_parity(
                    levels,
                    &y,
                    || format!("rgb24/bgr24 to y width {width} bgr {bgr}"),
                    |y| {
                        if bgr {
                            webp_convert_bgr24_to_y(&src[s..], &mut y[d..], int(width));
                        } else {
                            webp_convert_rgb24_to_y(&src[s..], &mut y[d..], int(width));
                        }
                    },
                );
                count += 1;
            }
        }
        let name = if bgr {
            "webp_convert_bgr24_to_y"
        } else {
            "webp_convert_rgb24_to_y"
        };
        tally.add(name, count, levels);
    }

    let mut count = 0;
    for width in lengths() {
        for _ in 0..4 {
            let (s, uo, vo) = (
                rng.below(8) as usize,
                rng.below(16) as usize,
                rng.below(16) as usize,
            );
            let kind = rng.below(5);
            let rgb: Vec<u16> = (0..s + 4 * width)
                .map(|_| match kind {
                    0 => 0,
                    1 => 1020,
                    2 => {
                        if rng.chance(1, 2) {
                            0
                        } else {
                            1020
                        }
                    }
                    3 => rng.below(32768) as u16,
                    _ => rng.below(1021) as u16,
                })
                .collect();
            let u = rng.bytes(uo + width);
            let v = rng.bytes(vo + width);
            assert_parity(
                levels,
                &(u, v),
                || format!("webp_convert_rgba32_to_uv width {width} kind {kind}"),
                |(u, v)| {
                    webp_convert_rgba32_to_uv(&rgb[s..], &mut u[uo..], &mut v[vo..], int(width));
                },
            );
            count += 1;
        }
    }
    tally.add("webp_convert_rgba32_to_uv", count, levels);
}

#[test]
fn yuv_simd_matches_the_c_translation() {
    let levels = simd_levels();
    assert!(!levels.is_empty(), "an x86 host has SSE2");
    let mut rng = Rng(0x0057_5031_3859_5556);
    let mut tally = Tally::default();
    samplers(&levels, &mut tally, &mut rng);
    converters(&levels, &mut tally, &mut rng);
    tally.print("wp18_yuv");
}
