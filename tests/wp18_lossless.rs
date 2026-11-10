//! WP18 parity test of `src/dsp/lossless_sse2.rs`, `lossless_sse41.rs` and
//! `lossless_avx2.rs` (ADR 0008, parity 1): each dispatch function of
//! `src/dsp/lossless.rs` that `VP8LDspInitSSE2`, `VP8LDspInitSSE41` or
//! `VP8LDspInitAVX2` assigns, run at every SIMD level the host has, against
//! the C translation on copies of the same seeded input.
//!
//! The inputs cover every predictor mode (the sentinels 14 and 15 included),
//! the lengths 0 to 40, the vector block boundaries of SSE2 (4 pixels), AVX2
//! (8) and the converters (8, 16, 32) with one pixel less and more, long rows,
//! a negative count, unaligned positions in the buffers, in-place and distinct
//! source and destination, random, channel-extreme and tie-prone pixels, and
//! every multiplier byte class of the color transform. The whole inverse
//! transforms of `vp8l_inverse_transform` run with every predictor tile size,
//! every color-indexing bit width (in place and not) and partial row ranges,
//! and `vp8l_convert_from_bgra` with every output mode.

#![cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]

#[path = "wp18_support/mod.rs"]
mod wp18_support;
#[path = "wp6_support/mod.rs"]
mod wp6_support;

use webp::dec::vp8li_dec::VP8LTransform;
use webp::dsp::lossless::{
    VP8LMultipliers, vp8l_add_green_to_blue_and_red, vp8l_convert_bgra_to_bgr,
    vp8l_convert_bgra_to_rgb, vp8l_convert_bgra_to_rgb565, vp8l_convert_bgra_to_rgba,
    vp8l_convert_bgra_to_rgba4444, vp8l_convert_from_bgra, vp8l_inverse_transform, vp8l_predictors,
    vp8l_predictors_add, vp8l_transform_color_inverse,
};
use webp::webp::decode::WEBP_CSP_MODE;
use webp::webp::format_constants::VP8LImageTransformType;
use wp6_support::Rng;
use wp18_support::{Level, Tally, assert_parity, simd_levels};

/// The pixel counts: every count up to 40, the block boundaries of the SIMD
/// loops with one less and one more, long rows, and a negative count.
fn lengths() -> Vec<i32> {
    let mut lengths: Vec<i32> = (0..=40).collect();
    for block in [48, 64, 96, 128, 256] {
        lengths.extend([block - 1, block, block + 1]);
    }
    lengths.extend([333, 1000, 1023, -1]);
    lengths
}

/// A byte from the classes that the arithmetic treats differently.
fn edge_byte(rng: &mut Rng) -> u32 {
    [0, 1, 2, 0x7e, 0x7f, 0x80, 0x81, 0xfd, 0xfe, 0xff][rng.below(10) as usize]
}

/// The pixel kinds: random, channel extremes, near a base (smooth), or one of
/// a few values (ties of the select predictor).
#[derive(Clone, Copy)]
enum Kind {
    Random,
    Extreme,
    Smooth(u32),
    Palette([u32; 3]),
}

fn kind(rng: &mut Rng) -> Kind {
    match rng.below(4) {
        0 => Kind::Random,
        1 => Kind::Extreme,
        2 => Kind::Smooth(rng.next_u64() as u32),
        _ => Kind::Palette([
            rng.next_u64() as u32,
            rng.next_u64() as u32,
            rng.next_u64() as u32,
        ]),
    }
}

fn pixel(rng: &mut Rng, kind: Kind) -> u32 {
    match kind {
        Kind::Random => rng.next_u64() as u32,
        Kind::Extreme => {
            edge_byte(rng) << 24 | edge_byte(rng) << 16 | edge_byte(rng) << 8 | edge_byte(rng)
        }
        Kind::Smooth(base) => {
            let mut value = 0;
            for shift in [0, 8, 16, 24] {
                let channel = ((base >> shift) & 0xff) as i64 + rng.range(-3, 3);
                value |= (channel.clamp(0, 255) as u32) << shift;
            }
            value
        }
        Kind::Palette(values) => values[rng.below(3) as usize],
    }
}

fn pixels(rng: &mut Rng, count: usize) -> Vec<u32> {
    let kind = kind(rng);
    (0..count).map(|_| pixel(rng, kind)).collect()
}

fn predictors(levels: &[Level], tally: &mut Tally, rng: &mut Rng) {
    let mut count = 0;
    for mode in 0..16 {
        for case in 0..2000 {
            let kind = kind(rng);
            let top: Vec<u32> = (0..3).map(|_| pixel(rng, kind)).collect();
            let left = vec![pixel(rng, kind)];
            assert_parity(
                levels,
                &0_u32,
                || format!("vp8l_predictors mode {mode} case {case} {left:08x?} {top:08x?}"),
                |result| *result = vp8l_predictors(mode, &left, 0, &top, 1),
            );
            count += 1;
        }
    }
    tally.add("vp8l_predictors", count, levels);
}

fn predictors_add(levels: &[Level], tally: &mut Tally, rng: &mut Rng) {
    let mut count = 0;
    for mode in 0..16 {
        for num_pixels in lengths() {
            for repeat in 0..6 {
                let in_place = repeat % 2 == 1;
                let n = usize::try_from(num_pixels).unwrap_or(0);
                // upper row (with upper[-1] and upper[n]), then the output row
                // (with out[-1]), then the residuals unless in place.
                let upper_pos = 1 + rng.below(4) as usize;
                let out_pos = upper_pos + n + 1 + rng.below(4) as usize;
                let in_pos = if in_place {
                    out_pos
                } else {
                    out_pos + n + rng.below(4) as usize
                };
                let data = pixels(rng, in_pos + n + 8);
                assert_parity(
                    levels,
                    &data,
                    || {
                        format!(
                            "vp8l_predictors_add mode {mode} num_pixels {num_pixels} in_place {in_place}"
                        )
                    },
                    |data| vp8l_predictors_add(mode, data, in_pos, upper_pos, num_pixels, out_pos),
                );
                count += 1;
            }
        }
    }
    tally.add("vp8l_predictors_add", count, levels);
}

fn add_green(levels: &[Level], tally: &mut Tally, rng: &mut Rng) {
    let mut count = 0;
    for num_pixels in lengths() {
        for repeat in 0..6 {
            let in_place = repeat % 2 == 1;
            let n = usize::try_from(num_pixels).unwrap_or(0);
            let src_pos = rng.below(4) as usize;
            let dst_pos = if in_place {
                src_pos
            } else {
                src_pos + n + rng.below(4) as usize
            };
            let data = pixels(rng, dst_pos + n + 8);
            assert_parity(
                levels,
                &data,
                || format!("vp8l_add_green_to_blue_and_red num_pixels {num_pixels}"),
                |data| vp8l_add_green_to_blue_and_red(data, src_pos, num_pixels, dst_pos),
            );
            count += 1;
        }
    }
    tally.add("vp8l_add_green_to_blue_and_red", count, levels);
}

fn multiplier(rng: &mut Rng) -> u8 {
    if rng.chance(1, 2) {
        edge_byte(rng) as u8
    } else {
        rng.byte()
    }
}

fn color_inverse(levels: &[Level], tally: &mut Tally, rng: &mut Rng) {
    let mut count = 0;
    for num_pixels in lengths() {
        for case in 0..8 {
            let m = VP8LMultipliers {
                green_to_red: multiplier(rng),
                green_to_blue: multiplier(rng),
                red_to_blue: multiplier(rng),
            };
            let n = usize::try_from(num_pixels).unwrap_or(0);
            let src_pos = rng.below(4) as usize;
            let dst_pos = if case % 2 == 0 {
                src_pos
            } else {
                src_pos + n + rng.below(4) as usize
            };
            let data = pixels(rng, dst_pos + n + 8);
            assert_parity(
                levels,
                &data,
                || format!("vp8l_transform_color_inverse num_pixels {num_pixels} {m:?}"),
                |data| vp8l_transform_color_inverse(&m, data, src_pos, num_pixels, dst_pos),
            );
            count += 1;
        }
    }
    tally.add("vp8l_transform_color_inverse", count, levels);
}

type Convert = fn(&[u32], i32, &mut [u8]);

fn converters(levels: &[Level], tally: &mut Tally, rng: &mut Rng) {
    let converters: [(&str, Convert, usize); 5] = [
        ("vp8l_convert_bgra_to_rgb", vp8l_convert_bgra_to_rgb, 3),
        ("vp8l_convert_bgra_to_rgba", vp8l_convert_bgra_to_rgba, 4),
        (
            "vp8l_convert_bgra_to_rgba4444",
            vp8l_convert_bgra_to_rgba4444,
            2,
        ),
        (
            "vp8l_convert_bgra_to_rgb565",
            vp8l_convert_bgra_to_rgb565,
            2,
        ),
        ("vp8l_convert_bgra_to_bgr", vp8l_convert_bgra_to_bgr, 3),
    ];
    for (name, convert, bytes) in converters {
        let mut count = 0;
        for num_pixels in lengths() {
            for _ in 0..4 {
                let n = usize::try_from(num_pixels).unwrap_or(0);
                let src_offset = rng.below(4) as usize;
                let dst_offset = rng.below(16) as usize;
                let src = pixels(rng, src_offset + n);
                // The output region, then sentinel bytes that no variant may
                // write.
                let dst = rng.bytes(dst_offset + n * bytes + 40);
                assert_parity(
                    levels,
                    &dst,
                    || format!("{name} num_pixels {num_pixels}"),
                    |dst| {
                        let end = dst_offset + n * bytes;
                        convert(&src[src_offset..], num_pixels, &mut dst[dst_offset..end]);
                    },
                );
                count += 1;
            }
        }
        tally.add(name, count, levels);
    }
}

fn inverse_transforms(levels: &[Level], tally: &mut Tally, rng: &mut Rng) {
    let mut count = 0;
    for (r#type, bits_range) in [
        (VP8LImageTransformType::PREDICTOR_TRANSFORM, 2..=9),
        (VP8LImageTransformType::CROSS_COLOR_TRANSFORM, 2..=9),
        (VP8LImageTransformType::SUBTRACT_GREEN_TRANSFORM, 0..=0),
        (VP8LImageTransformType::COLOR_INDEXING_TRANSFORM, 0..=3),
    ] {
        for bits in bits_range {
            for case in 0..40 {
                let width = [1, 2, 3, 4, 5, 7, 8, 9, 16, 17, 33, 64, 100][rng.below(13) as usize];
                let height = 1 + rng.below(9) as i32;
                let tiles = |size: i32| ((size + (1 << bits) - 1) >> bits) as usize;
                let data: Vec<u32> = if r#type == VP8LImageTransformType::COLOR_INDEXING_TRANSFORM {
                    // The color map, padded to 256 entries as the decoder does.
                    pixels(rng, 256)
                } else if r#type == VP8LImageTransformType::PREDICTOR_TRANSFORM {
                    // Modes 0 to 15 in the green channel.
                    (0..tiles(width) * tiles(height))
                        .map(|_| (rng.below(16) as u32) << 8 | rng.next_u64() as u32 & 0xff00_00ff)
                        .collect()
                } else {
                    pixels(rng, tiles(width) * tiles(height))
                };
                let transform = VP8LTransform {
                    r#type,
                    bits,
                    xsize: width,
                    ysize: height,
                    data,
                };
                let row = width as usize;
                let rows = height as usize;
                let row_start = if case % 3 == 0 {
                    0
                } else {
                    rng.below(rows as u64) as i32
                };
                let row_end = if case % 4 == 0 {
                    height
                } else {
                    row_start + 1 + rng.below((height - row_start) as u64) as i32
                };
                let in_place = case % 2 == 0;
                // One row above the output (the predictor's top row), the
                // output, then the input unless in place.
                let out_pos = row + rng.below(3) as usize;
                let in_pos = if in_place {
                    out_pos
                } else {
                    out_pos + rows * row + 3
                };
                let mut buffer = pixels(rng, in_pos + rows * row + 4);
                if r#type == VP8LImageTransformType::COLOR_INDEXING_TRANSFORM {
                    // Packed indices in the green channel.
                    for value in &mut buffer[in_pos..] {
                        *value = (*value & 0xffff_00ff) | (rng.byte() as u32) << 8;
                    }
                }
                assert_parity(
                    levels,
                    &buffer,
                    || format!("vp8l_inverse_transform {type:?} bits {bits} {width}x{height} rows {row_start}..{row_end} in_place {in_place}", type = r#type),
                    |buffer| {
                        vp8l_inverse_transform(
                            &transform, row_start, row_end, buffer, in_pos, out_pos,
                        );
                    },
                );
                count += 1;
            }
        }
    }
    tally.add("vp8l_inverse_transform", count, levels);
}

fn convert_from_bgra(levels: &[Level], tally: &mut Tally, rng: &mut Rng) {
    let modes = [
        (WEBP_CSP_MODE::MODE_RGB, 3),
        (WEBP_CSP_MODE::MODE_RGBA, 4),
        (WEBP_CSP_MODE::MODE_BGR, 3),
        (WEBP_CSP_MODE::MODE_BGRA, 4),
        (WEBP_CSP_MODE::MODE_ARGB, 4),
        (WEBP_CSP_MODE::MODE_RGBA_4444, 2),
        (WEBP_CSP_MODE::MODE_RGB_565, 2),
        (WEBP_CSP_MODE::MODE_rgbA, 4),
        (WEBP_CSP_MODE::MODE_bgrA, 4),
        (WEBP_CSP_MODE::MODE_Argb, 4),
        (WEBP_CSP_MODE::MODE_rgbA_4444, 2),
    ];
    let mut count = 0;
    for (mode, bytes) in modes {
        for num_pixels in lengths() {
            let n = usize::try_from(num_pixels).unwrap_or(0);
            let src = pixels(rng, n);
            let dst = rng.bytes(n * bytes);
            assert_parity(
                levels,
                &dst,
                || format!("vp8l_convert_from_bgra {mode:?} num_pixels {num_pixels}"),
                |dst| vp8l_convert_from_bgra(&src, num_pixels, mode, dst),
            );
            count += 1;
        }
    }
    tally.add("vp8l_convert_from_bgra", count, levels);
}

#[test]
fn lossless_simd_matches_the_c_translation() {
    let levels = simd_levels();
    assert!(!levels.is_empty(), "an x86 host has SSE2");
    let mut rng = Rng(0x5750_3138_4c4f_5353);
    let mut tally = Tally::default();
    predictors(&levels, &mut tally, &mut rng);
    predictors_add(&levels, &mut tally, &mut rng);
    add_green(&levels, &mut tally, &mut rng);
    color_inverse(&levels, &mut tally, &mut rng);
    converters(&levels, &mut tally, &mut rng);
    inverse_transforms(&levels, &mut tally, &mut rng);
    convert_from_bgra(&levels, &mut tally, &mut rng);
    tally.print("wp18_lossless");
}
