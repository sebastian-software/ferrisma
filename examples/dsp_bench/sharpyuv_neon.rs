//! Benchmark cases of `src/sharpyuv/sharpyuv_neon.rs` (WP20): the C
//! translation and, with the `simd` feature on 64-bit ARM, the NEON variant
//! of each pointer that `InitSharpYuvNEON` assigns, on rows of 256 samples at
//! the bit depth 10 and, for the filter, also 12.

use std::hint::black_box;

use webp::sharpyuv::sharpyuv_dsp as c;
#[cfg(all(feature = "simd", target_arch = "aarch64"))]
/// The NEON level of `src/sharpyuv/sharpyuv_neon.rs`, reached through the dispatch
/// functions, which select its crate-private entry points with `simd` on
/// 64-bit ARM (the residual costs while the `VP8GetCPUInfo` query answers
/// `kNEON`, as it does by default); named as the entry points.
mod neon {
    pub use webp::sharpyuv::sharpyuv_dsp::sharp_yuv_filter_row;
    pub use webp::sharpyuv::sharpyuv_dsp::sharp_yuv_update_rgb;
    pub use webp::sharpyuv::sharpyuv_dsp::sharp_yuv_update_y;
}

use super::{Case, Rng};

const W: usize = 256;

fn case(function: &'static str, variant: &'static str, run: Box<dyn FnMut()>) -> Case {
    Case {
        module: "sharpyuv",
        function,
        variant,
        run,
    }
}

/// Samples below `1 << bit_depth`.
fn samples(rng: &mut Rng, count: usize, bit_depth: u32) -> Vec<u16> {
    rng.bytes(2 * count)
        .as_chunks::<2>()
        .0
        .iter()
        .map(|b| u16::from_le_bytes(*b) >> (16 - bit_depth))
        .collect()
}

/// Chroma differences within the bit depth's range.
fn chroma(rng: &mut Rng, count: usize, bit_depth: u32) -> Vec<i16> {
    samples(rng, count, bit_depth + 1)
        .into_iter()
        .map(|v| v.cast_signed() - (1 << bit_depth))
        .collect()
}

type UpdateY = fn(&[u16], &[u16], &mut [u16], i32, i32) -> u64;
type UpdateRgb = fn(&[i16], &[i16], &mut [i16], i32);
type FilterRow = fn(&[i16], &[i16], i32, &[u16], &mut [u16], i32);

fn update_y(rng: &mut Rng, variant: &'static str, f: UpdateY) -> Case {
    let (r, s, mut d) = (
        samples(rng, W, 10),
        samples(rng, W, 10),
        samples(rng, W, 10),
    );
    case(
        "SharpYuvUpdateY",
        variant,
        Box::new(move || {
            black_box(f(&r, &s, black_box(&mut d), W as i32, 10));
        }),
    )
}

fn update_rgb(rng: &mut Rng, variant: &'static str, f: UpdateRgb) -> Case {
    let (r, s, mut d) = (chroma(rng, W, 10), chroma(rng, W, 10), chroma(rng, W, 10));
    case(
        "SharpYuvUpdateRGB",
        variant,
        Box::new(move || f(&r, &s, black_box(&mut d), W as i32)),
    )
}

fn filter_row(
    rng: &mut Rng,
    function: &'static str,
    variant: &'static str,
    bit_depth: u32,
    f: FilterRow,
) -> Case {
    let a = chroma(rng, W + 1, bit_depth);
    let b = chroma(rng, W + 1, bit_depth);
    let best_y = samples(rng, 2 * W, bit_depth);
    let mut out = vec![0_u16; 2 * W];
    let depth = bit_depth.cast_signed();
    case(
        function,
        variant,
        Box::new(move || f(&a, &b, W as i32, &best_y, black_box(&mut out), depth)),
    )
}

/// The C and NEON cases of each pointer that `InitSharpYuvNEON` assigns.
pub fn cases() -> Vec<Case> {
    let mut rng = Rng::new(0x2020_0e00);
    let mut cases = Vec::new();
    cases.extend([
        update_y(&mut rng, "C", c::sharp_yuv_update_y_c),
        update_rgb(&mut rng, "C", c::sharp_yuv_update_rgb_c),
        filter_row(
            &mut rng,
            "SharpYuvFilterRow (10 bits)",
            "C",
            10,
            c::sharp_yuv_filter_row_c,
        ),
        filter_row(
            &mut rng,
            "SharpYuvFilterRow (12 bits)",
            "C",
            12,
            c::sharp_yuv_filter_row_c,
        ),
    ]);
    #[cfg(all(feature = "simd", target_arch = "aarch64"))]
    cases.extend([
        update_y(&mut rng, "NEON", neon::sharp_yuv_update_y),
        update_rgb(&mut rng, "NEON", neon::sharp_yuv_update_rgb),
        filter_row(
            &mut rng,
            "SharpYuvFilterRow (10 bits)",
            "NEON",
            10,
            neon::sharp_yuv_filter_row,
        ),
        filter_row(
            &mut rng,
            "SharpYuvFilterRow (12 bits)",
            "NEON",
            12,
            neon::sharp_yuv_filter_row,
        ),
    ]);
    cases
}
