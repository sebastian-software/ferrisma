//! WP18 parity test of `src/dsp/alpha_processing_sse2.rs` and
//! `alpha_processing_sse41.rs` (ADR 0008, parity 1): each dispatch function of
//! `src/dsp/alpha_processing.rs` that `WebPInitAlphaProcessingSSE2` or
//! `WebPInitAlphaProcessingSSE41` assigns, run at every SIMD level the host
//! has, against the C translation on copies of the same seeded input.
//!
//! The inputs cover the lengths 0 to 70 and the lengths around the vector
//! widths (8, 16, 32, 64 and their neighbors), one to four rows with loose and
//! tight strides, buffers that end exactly where the scalar function stops,
//! unaligned starts, both premultiplication directions, and the alpha values
//! 0, 255, near-opaque and random, including all-opaque planes and planes with
//! a single non-opaque value at every position.

#![cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]

#[path = "wp18_support/mod.rs"]
mod wp18_support;
#[path = "wp6_support/mod.rs"]
mod wp6_support;

use webp::dsp::alpha_processing::{
    webp_alpha_replace, webp_apply_alpha_multiply, webp_dispatch_alpha,
    webp_dispatch_alpha_to_green, webp_extract_alpha, webp_extract_green, webp_has_alpha8b,
    webp_has_alpha32b, webp_mult_argb_row, webp_mult_row,
};
use wp6_support::Rng;
use wp18_support::{Level, Tally, assert_parity, simd_levels};

/// The lengths of the row tests: every length up to 70 and the neighbors of
/// the larger vector multiples.
fn lengths() -> Vec<i32> {
    let mut lengths: Vec<i32> = (0..=70).collect();
    lengths.extend([95, 96, 97, 127, 128, 129, 255, 256, 257]);
    lengths
}

/// An alpha value: often opaque or transparent, sometimes near the ends.
fn alpha_value(rng: &mut Rng) -> u8 {
    match rng.below(8) {
        0..=2 => 0xff,
        3 => 0,
        4 => rng.range(250, 254) as u8,
        5 => rng.range(1, 5) as u8,
        _ => rng.byte(),
    }
}

/// An alpha plane: all opaque, opaque but for one value, or mixed.
fn alpha_plane(rng: &mut Rng, size: usize) -> Vec<u8> {
    match rng.below(4) {
        0 => vec![0xff; size],
        1 => {
            let mut plane = vec![0xff; size];
            if size > 0 {
                let at = rng.below(size as u64) as usize;
                plane[at] = rng.range(0, 254) as u8;
            }
            plane
        }
        _ => (0..size).map(|_| alpha_value(rng)).collect(),
    }
}

/// An ARGB pixel with an alpha value of `alpha_value`.
fn pixel(rng: &mut Rng) -> u32 {
    (u32::from(alpha_value(rng)) << 24) | (rng.next_u64() as u32 & 0x00ff_ffff)
}

fn mult_rows(levels: &[Level], tally: &mut Tally, rng: &mut Rng) {
    let mut argb = 0;
    let mut single = 0;
    for width in lengths() {
        for inverse in [false, true] {
            for case in 0..6 {
                let offset = rng.below(4) as usize;
                let pixels: Vec<u32> = (0..offset + width as usize).map(|_| pixel(rng)).collect();
                assert_parity(
                    levels,
                    &pixels,
                    || format!("webp_mult_argb_row width {width} inverse {inverse} case {case}"),
                    |pixels| webp_mult_argb_row(&mut pixels[offset..], width, inverse),
                );
                argb += 1;

                let alpha_offset = rng.below(16) as usize;
                let values = rng.bytes(offset + width as usize);
                let alpha = {
                    let mut plane = rng.bytes(alpha_offset);
                    plane.extend(alpha_plane(rng, width as usize));
                    plane
                };
                assert_parity(
                    levels,
                    &values,
                    || format!("webp_mult_row width {width} inverse {inverse} case {case}"),
                    |values| {
                        webp_mult_row(
                            &mut values[offset..],
                            &alpha[alpha_offset..],
                            width,
                            inverse,
                        );
                    },
                );
                single += 1;
            }
        }
    }
    tally.add("webp_mult_argb_row", argb, levels);
    tally.add("webp_mult_row", single, levels);
}

fn apply_alpha_multiply(levels: &[Level], tally: &mut Tally, rng: &mut Rng) {
    let mut count = 0;
    for w in (0..=20).chain([31, 32, 33, 63, 64, 65]) {
        for h in 1..=4 {
            for alpha_first in [false, true] {
                for case in 0..4 {
                    let stride = 4 * w + rng.below(3) as i32 * 4 + i32::from(case % 2 == 0);
                    let offset = rng.below(4) as usize;
                    let size = offset + (stride * (h - 1) + 4 * w) as usize;
                    let alpha_index = if alpha_first { 0 } else { 3 };
                    let mut plane = rng.bytes(size);
                    for (i, byte) in plane[offset..].iter_mut().enumerate() {
                        if i % 4 == alpha_index && (i as i32 % stride) < 4 * w {
                            *byte = alpha_value(rng);
                        }
                    }
                    assert_parity(
                        levels,
                        &plane,
                        || {
                            format!(
                                "webp_apply_alpha_multiply {w}x{h} first {alpha_first} case {case}"
                            )
                        },
                        |plane| {
                            webp_apply_alpha_multiply(
                                &mut plane[offset..],
                                alpha_first,
                                w,
                                h,
                                stride,
                            )
                        },
                    );
                    count += 1;
                }
            }
        }
    }
    tally.add("webp_apply_alpha_multiply", count, levels);
}

fn dispatch_and_extract(levels: &[Level], tally: &mut Tally, rng: &mut Rng) {
    let mut dispatch = 0;
    let mut green = 0;
    let mut extract = 0;
    for width in lengths() {
        for height in 1..=3 {
            for case in 0..4 {
                let w = width as usize;
                let h = height as usize;
                let alpha_stride = width + rng.below(3) as i32;
                let alpha_size = (alpha_stride as usize) * (h - 1) + w;
                let alpha = alpha_plane(rng, alpha_size);

                // WebPDispatchAlpha: the last row ends at its last alpha byte,
                // `dst[4 * width - 4]`, plus up to three bytes.
                let dst_stride = 4 * width + 4 * rng.below(3) as i32;
                let tail = rng.below(4) as usize;
                let dst_size = (dst_stride as usize) * (h - 1) + (4 * w).saturating_sub(3) + tail;
                let dst = rng.bytes(dst_size);
                assert_parity(
                    levels,
                    &(dst, false),
                    || format!("webp_dispatch_alpha {width}x{height} case {case}"),
                    |(dst, result)| {
                        *result = webp_dispatch_alpha(
                            &alpha,
                            alpha_stride,
                            width,
                            height,
                            dst,
                            dst_stride,
                        );
                    },
                );
                dispatch += 1;

                // WebPDispatchAlphaToGreen.
                let green_stride = width + rng.below(3) as i32;
                let green_size = (green_stride as usize) * (h - 1) + w;
                let pixels: Vec<u32> = (0..green_size).map(|_| rng.next_u64() as u32).collect();
                assert_parity(
                    levels,
                    &pixels,
                    || format!("webp_dispatch_alpha_to_green {width}x{height} case {case}"),
                    |pixels| {
                        webp_dispatch_alpha_to_green(
                            &alpha,
                            alpha_stride,
                            width,
                            height,
                            pixels,
                            green_stride,
                        );
                    },
                );
                green += 1;

                // WebPExtractAlpha: an ARGB byte plane whose last row ends at
                // its last alpha byte plus up to three bytes.
                let argb_stride = 4 * width + 4 * rng.below(3) as i32 + rng.below(2) as i32;
                let argb_size = (argb_stride as usize) * (h - 1) + (4 * w).saturating_sub(3) + tail;
                let mut argb = rng.bytes(argb_size);
                let opaque = rng.chance(1, 3);
                for j in 0..h {
                    for i in 0..w {
                        argb[j * argb_stride as usize + 4 * i] =
                            if opaque { 0xff } else { alpha_value(rng) };
                    }
                }
                if opaque && rng.chance(1, 2) && w > 0 {
                    let j = rng.below(h as u64) as usize;
                    let i = rng.below(w as u64) as usize;
                    argb[j * argb_stride as usize + 4 * i] = rng.range(0, 254) as u8;
                }
                let out_stride = width + rng.below(3) as i32;
                let out = rng.bytes((out_stride as usize) * (h - 1) + w);
                assert_parity(
                    levels,
                    &(out, false),
                    || format!("webp_extract_alpha {width}x{height} case {case}"),
                    |(out, result)| {
                        *result =
                            webp_extract_alpha(&argb, argb_stride, width, height, out, out_stride);
                    },
                );
                extract += 1;
            }
        }
    }
    tally.add("webp_dispatch_alpha", dispatch, levels);
    tally.add("webp_dispatch_alpha_to_green", green, levels);
    tally.add("webp_extract_alpha", extract, levels);
}

fn extract_green(levels: &[Level], tally: &mut Tally, rng: &mut Rng) {
    let mut count = 0;
    for size in lengths() {
        for case in 0..6 {
            let offset = rng.below(4) as usize;
            let argb: Vec<u32> = (0..offset + size as usize)
                .map(|_| rng.next_u64() as u32)
                .collect();
            let spare = rng.below(3) as usize;
            let out = rng.bytes(size as usize + spare);
            assert_parity(
                levels,
                &out,
                || format!("webp_extract_green size {size} case {case}"),
                |out| webp_extract_green(&argb[offset..], out, size),
            );
            count += 1;
        }
    }
    tally.add("webp_extract_green", count, levels);
}

fn has_alpha(levels: &[Level], tally: &mut Tally, rng: &mut Rng) {
    let mut eight = 0;
    let mut thirty_two = 0;
    for length in lengths() {
        let len = length as usize;
        for case in 0..8 {
            // WebPHasAlpha8b over a plane of exactly `length` bytes.
            let offset = rng.below(4) as usize;
            let mut plane = rng.bytes(offset);
            plane.extend(alpha_plane(rng, len));
            assert_parity(
                levels,
                &false,
                || format!("webp_has_alpha8b length {length} case {case}"),
                |result| *result = webp_has_alpha8b(&plane[offset..], length),
            );
            eight += 1;

            // WebPHasAlpha32b over `4 * length - 3` bytes, the alpha bytes at
            // every fourth, the others random.
            let mut bytes = rng.bytes(offset + (4 * len).saturating_sub(3));
            let alphas = alpha_plane(rng, len);
            for (i, &a) in alphas.iter().enumerate() {
                bytes[offset + 4 * i] = a;
            }
            assert_parity(
                levels,
                &false,
                || format!("webp_has_alpha32b length {length} case {case}"),
                |result| *result = webp_has_alpha32b(&bytes[offset..], length),
            );
            thirty_two += 1;
        }
    }
    tally.add("webp_has_alpha8b", eight, levels);
    tally.add("webp_has_alpha32b", thirty_two, levels);
}

fn alpha_replace(levels: &[Level], tally: &mut Tally, rng: &mut Rng) {
    let mut count = 0;
    for length in lengths() {
        for case in 0..6 {
            let offset = rng.below(4) as usize;
            let pixels: Vec<u32> = (0..offset + length as usize)
                .map(|_| match rng.below(4) {
                    0 => rng.next_u64() as u32 & 0x00ff_ffff,
                    1 => 0x8000_0000 | (rng.next_u64() as u32 & 0x00ff_ffff),
                    _ => rng.next_u64() as u32,
                })
                .collect();
            let color = rng.next_u64() as u32;
            assert_parity(
                levels,
                &pixels,
                || format!("webp_alpha_replace length {length} case {case}"),
                |pixels| webp_alpha_replace(&mut pixels[offset..], length, color),
            );
            count += 1;
        }
    }
    tally.add("webp_alpha_replace", count, levels);
}

#[test]
fn alpha_processing_simd_matches_the_c_translation() {
    let levels = simd_levels();
    assert!(!levels.is_empty(), "an x86 host has SSE2");
    let mut rng = Rng(0x5750_3138_414c_5048);
    let mut tally = Tally::default();
    mult_rows(&levels, &mut tally, &mut rng);
    apply_alpha_multiply(&levels, &mut tally, &mut rng);
    dispatch_and_extract(&levels, &mut tally, &mut rng);
    extract_green(&levels, &mut tally, &mut rng);
    has_alpha(&levels, &mut tally, &mut rng);
    alpha_replace(&levels, &mut tally, &mut rng);
    tally.print("wp18_alpha");
}
