//! Benchmark cases of `src/dsp/upsampling_neon.rs` (WP20): the C
//! translation and, with the `simd` feature on 64-bit ARM, the NEON variant
//! of each upsampler that `WebPInitUpsamplersNEON` stores, on a line pair of
//! 256 pixels with its bottom row.

use std::hint::black_box;

use webp::dsp::upsampling as c;
use webp::dsp::upsampling::WebPUpsampleLinePairFunc;
#[cfg(all(feature = "simd", target_arch = "aarch64"))]
use webp::dsp::upsampling_neon as neon;

use super::{Case, Rng};

const W: usize = 256;

fn case(
    rng: &mut Rng,
    function: &'static str,
    variant: &'static str,
    xstep: usize,
    upsampler: WebPUpsampleLinePairFunc,
) -> Case {
    let top_y = rng.bytes(W);
    let bottom_y = rng.bytes(W);
    let planes: Vec<Vec<u8>> = (0..4).map(|_| rng.bytes(W / 2)).collect();
    let mut top = vec![0_u8; W * xstep];
    let mut bottom = vec![0_u8; W * xstep];
    Case {
        module: "upsampling",
        function,
        variant,
        run: Box::new(move || {
            upsampler(
                &top_y,
                Some((&bottom_y, black_box(&mut bottom))),
                &planes[0],
                &planes[1],
                &planes[2],
                &planes[3],
                black_box(&mut top),
                W as i32,
            );
        }),
    }
}

/// The NEON entry points, in the order of `FORMATS`.
#[cfg(all(feature = "simd", target_arch = "aarch64"))]
const NEON: [WebPUpsampleLinePairFunc; 7] = [
    neon::upsample_rgba_line_pair,
    neon::upsample_bgra_line_pair,
    neon::upsample_rgb_line_pair,
    neon::upsample_bgr_line_pair,
    neon::upsample_argb_line_pair,
    neon::upsample_rgba4444_line_pair,
    neon::upsample_rgb565_line_pair,
];

/// The formats: name, bytes per pixel and C function.
const FORMATS: [(&str, usize, WebPUpsampleLinePairFunc); 7] = [
    ("UpsampleRgbaLinePair", 4, c::upsample_rgba_line_pair_c),
    ("UpsampleBgraLinePair", 4, c::upsample_bgra_line_pair_c),
    ("UpsampleRgbLinePair", 3, c::upsample_rgb_line_pair_c),
    ("UpsampleBgrLinePair", 3, c::upsample_bgr_line_pair_c),
    ("UpsampleArgbLinePair", 4, c::upsample_argb_line_pair_c),
    (
        "UpsampleRgba4444LinePair",
        2,
        c::upsample_rgba4444_line_pair_c,
    ),
    ("UpsampleRgb565LinePair", 2, c::upsample_rgb565_line_pair_c),
];

/// The C and NEON cases of each upsampler that `WebPInitUpsamplersNEON`
/// stores.
pub fn cases() -> Vec<Case> {
    let mut rng = Rng::new(0x2020_0c00);
    let mut cases = Vec::new();
    #[cfg_attr(
        not(all(feature = "simd", target_arch = "aarch64")),
        expect(unused_variables, reason = "the index selects the NEON entry point")
    )]
    for (index, (function, xstep, c_fn)) in FORMATS.into_iter().enumerate() {
        cases.push(case(&mut rng, function, "C", xstep, c_fn));
        #[cfg(all(feature = "simd", target_arch = "aarch64"))]
        cases.push(case(&mut rng, function, "NEON", xstep, NEON[index]));
    }
    cases
}
