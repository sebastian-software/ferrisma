//! WP20 parity test: the NEON `SharpYUV` kernels of `sharpyuv/sharpyuv_neon.c`
//! against the C translations they replace (`src/sharpyuv/sharpyuv_dsp.rs`),
//! Rust against Rust, and the dispatch functions against the C functions,
//! for the bit depths 8 to 14 that `SharpYuvConvert` uses (at most
//! `kMaxBitDepth`), on both sides of the 16-bit filter's limit of 10.

#![cfg(all(feature = "simd", target_arch = "aarch64"))]

#[path = "support/wp20.rs"]
mod support;

use support::Rng;
use webp::sharpyuv::sharpyuv_dsp as c;
/// The NEON level of `src/sharpyuv/sharpyuv_neon.rs`, reached through the dispatch
/// functions, which select its crate-private entry points with `simd` on
/// 64-bit ARM (the residual costs while the `VP8GetCPUInfo` query answers
/// `kNEON`, as it does by default); named as the entry points.
mod neon {
    pub use webp::sharpyuv::sharpyuv_dsp::sharp_yuv_filter_row;
    pub use webp::sharpyuv::sharpyuv_dsp::sharp_yuv_update_rgb;
    pub use webp::sharpyuv::sharpyuv_dsp::sharp_yuv_update_y;
}

fn lengths() -> impl Iterator<Item = usize> {
    (0..=40).chain([63, 64, 65, 127, 128, 129, 1000])
}

/// Samples in `0..1 << bit_depth`, with both bounds a quarter of the time.
fn samples(rng: &mut Rng, count: usize, bit_depth: u32) -> Vec<u16> {
    let max = (1_u64 << bit_depth) - 1;
    (0..count)
        .map(|_| match rng.below(8) {
            0 => 0,
            1 => max as u16,
            _ => rng.below(max + 1) as u16,
        })
        .collect()
}

#[test]
fn update_y_matches_c() {
    let mut rng = Rng(0x2020_0501);
    for bit_depth in 8..=14 {
        for len in lengths() {
            for offset in 0..4 {
                let r = samples(&mut rng, offset + len, bit_depth);
                let s = samples(&mut rng, offset + len, bit_depth);
                let d = samples(&mut rng, offset + len, bit_depth);
                let (mut expected, mut actual, mut dispatched) = (d.clone(), d.clone(), d);
                let (l, bd) = (len as i32, bit_depth as i32);
                let o = offset;
                let r_c = c::sharp_yuv_update_y_c(&r[o..], &s[o..], &mut expected[o..], l, bd);
                let r_n = neon::sharp_yuv_update_y(&r[o..], &s[o..], &mut actual[o..], l, bd);
                let r_d = c::sharp_yuv_update_y(&r[o..], &s[o..], &mut dispatched[o..], l, bd);
                assert_eq!(
                    (r_n, &actual),
                    (r_c, &expected),
                    "bit_depth={bit_depth} len={len}"
                );
                assert_eq!((r_d, &dispatched), (r_c, &expected));
            }
        }
    }
}

#[test]
fn update_rgb_matches_c() {
    let mut rng = Rng(0x2020_0502);
    for len in lengths() {
        for offset in 0..4 {
            for range in [1 << 10, 1 << 14, 32767] {
                let r = rng.i16s(offset + len, range);
                let s = rng.i16s(offset + len, range);
                let d = rng.i16s(offset + len, range);
                let (mut expected, mut actual, mut dispatched) = (d.clone(), d.clone(), d);
                let l = len as i32;
                let o = offset;
                c::sharp_yuv_update_rgb_c(&r[o..], &s[o..], &mut expected[o..], l);
                neon::sharp_yuv_update_rgb(&r[o..], &s[o..], &mut actual[o..], l);
                c::sharp_yuv_update_rgb(&r[o..], &s[o..], &mut dispatched[o..], l);
                assert_eq!(actual, expected, "len={len} range={range}");
                assert_eq!(dispatched, expected);
            }
        }
    }
}

#[test]
fn filter_row_matches_c() {
    let mut rng = Rng(0x2020_0503);
    for bit_depth in 8..=14 {
        for len in lengths() {
            for offset in 0..4 {
                // Chroma differences within the bit depth's range, both signs.
                let range = 1 << bit_depth;
                let a = rng.i16s(offset + len + 1, range);
                let b = rng.i16s(offset + len + 1, range);
                let best_y = samples(&mut rng, offset + 2 * len, bit_depth);
                let out0: Vec<u16> = (0..offset + 2 * len)
                    .map(|_| rng.next_u64() as u16)
                    .collect();
                let (mut expected, mut actual, mut dispatched) = (out0.clone(), out0.clone(), out0);
                let (l, bd) = (len as i32, bit_depth as i32);
                let o = offset;
                c::sharp_yuv_filter_row_c(
                    &a[o..],
                    &b[o..],
                    l,
                    &best_y[o..],
                    &mut expected[o..],
                    bd,
                );
                neon::sharp_yuv_filter_row(&a[o..], &b[o..], l, &best_y[o..], &mut actual[o..], bd);
                c::sharp_yuv_filter_row(
                    &a[o..],
                    &b[o..],
                    l,
                    &best_y[o..],
                    &mut dispatched[o..],
                    bd,
                );
                assert_eq!(actual, expected, "bit_depth={bit_depth} len={len}");
                assert_eq!(dispatched, expected);
            }
        }
    }
}
