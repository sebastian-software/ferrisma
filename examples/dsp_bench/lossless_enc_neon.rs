//! Benchmark cases of `src/dsp/lossless_enc_neon.rs` (WP20): the C
//! translation and, with the `simd` feature on 64-bit ARM, the NEON variant
//! of each pointer that `VP8LEncDspInitNEON` assigns, on rows of 256 pixels.

use std::hint::black_box;

use webp::dsp::lossless::VP8LMultipliers;
use webp::dsp::lossless_enc as c;
#[cfg(all(feature = "simd", target_arch = "aarch64"))]
use webp::dsp::lossless_enc_neon as neon;

use super::{Case, Rng};

/// Pixels per row.
const WIDTH: usize = 256;

/// A case of an in-place transform of a row of seeded pixels.
fn transform_case(
    rng: &mut Rng,
    function: &'static str,
    variant: &'static str,
    run: impl Fn(&mut [u32]) + 'static,
) -> Case {
    let mut data: Vec<u32> = rng
        .bytes(4 * WIDTH)
        .as_chunks::<4>()
        .0
        .iter()
        .map(|b| u32::from_le_bytes(*b))
        .collect();
    Case {
        module: "lossless_enc",
        function,
        variant,
        run: Box::new(move || run(black_box(&mut data))),
    }
}

/// The C and NEON cases of each pointer that `VP8LEncDspInitNEON` assigns.
pub fn cases() -> Vec<Case> {
    let mut rng = Rng::new(0x2020_1100);
    let mut cases = Vec::new();
    cases.push(transform_case(
        &mut rng,
        "VP8LSubtractGreenFromBlueAndRed",
        "C",
        |d| c::vp8l_subtract_green_from_blue_and_red_c(d, WIDTH as i32),
    ));
    #[cfg(all(feature = "simd", target_arch = "aarch64"))]
    cases.push(transform_case(
        &mut rng,
        "VP8LSubtractGreenFromBlueAndRed",
        "NEON",
        |d| neon::subtract_green_from_blue_and_red(d, WIDTH as i32),
    ));
    let m = VP8LMultipliers {
        green_to_red: 0x35,
        green_to_blue: 0xc4,
        red_to_blue: 0x7b,
    };
    cases.push(transform_case(
        &mut rng,
        "VP8LTransformColor",
        "C",
        move |d| {
            c::vp8l_transform_color_c(&m, d, WIDTH as i32);
        },
    ));
    #[cfg(all(feature = "simd", target_arch = "aarch64"))]
    cases.push(transform_case(
        &mut rng,
        "VP8LTransformColor",
        "NEON",
        move |d| {
            neon::transform_color(&m, d, WIDTH as i32);
        },
    ));
    cases
}
