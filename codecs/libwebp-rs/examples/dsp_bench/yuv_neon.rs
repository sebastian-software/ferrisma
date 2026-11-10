//! Benchmark cases of `src/dsp/yuv_neon.rs` (WP20): the C translation and,
//! with the `simd` feature on 64-bit ARM, the NEON variant of each pointer
//! that `WebPInitConvertARGBToYUVNEON` assigns, on rows of 256 pixels.

use std::hint::black_box;

use webp::dsp::yuv as c;
#[cfg(all(feature = "simd", target_arch = "aarch64"))]
/// The NEON level of `src/dsp/yuv_neon.rs`, reached through the dispatch
/// functions, which select its crate-private entry points with `simd` on
/// 64-bit ARM (the residual costs while the `VP8GetCPUInfo` query answers
/// `kNEON`, as it does by default); named as the entry points.
mod neon {
    pub use webp::dsp::yuv::webp_convert_argb_to_uv as convert_argb_to_uv;
    pub use webp::dsp::yuv::webp_convert_argb_to_y as convert_argb_to_y;
    pub use webp::dsp::yuv::webp_convert_bgr24_to_y as convert_bgr24_to_y;
    pub use webp::dsp::yuv::webp_convert_rgb24_to_y as convert_rgb24_to_y;
    pub use webp::dsp::yuv::webp_convert_rgba32_to_uv as convert_rgba32_to_uv;
}

use super::{Case, Rng};

const W: usize = 256;

fn case(function: &'static str, variant: &'static str, run: Box<dyn FnMut()>) -> Case {
    Case {
        module: "yuv",
        function,
        variant,
        run,
    }
}

fn words(rng: &mut Rng, count: usize) -> Vec<u32> {
    rng.bytes(4 * count)
        .as_chunks::<4>()
        .0
        .iter()
        .map(|b| u32::from_le_bytes(*b))
        .collect()
}

/// Accumulated RGBA samples as the encoder passes them (at most 1020).
fn sums(rng: &mut Rng, count: usize) -> Vec<u16> {
    rng.bytes(2 * count)
        .as_chunks::<2>()
        .0
        .iter()
        .map(|b| u16::from_le_bytes(*b) % 1021)
        .collect()
}

/// The C and NEON cases of each pointer that `WebPInitConvertARGBToYUVNEON`
/// assigns.
pub fn cases() -> Vec<Case> {
    let mut rng = Rng::new(0x2020_0b00);
    let mut cases = Vec::new();
    let w = W as i32;
    let rgb = rng.bytes(3 * W);
    let argb = words(&mut rng, W);
    let rgba = sums(&mut rng, 4 * W);

    for (function, bgr) in [
        ("WebPConvertRGB24ToY", false),
        ("WebPConvertBGR24ToY", true),
    ] {
        let src = rgb.clone();
        let mut y = vec![0_u8; W];
        cases.push(case(
            function,
            "C",
            Box::new(move || {
                if bgr {
                    c::convert_bgr24_to_y_c(&src, black_box(&mut y), w);
                } else {
                    c::convert_rgb24_to_y_c(&src, black_box(&mut y), w);
                }
            }),
        ));
        #[cfg(all(feature = "simd", target_arch = "aarch64"))]
        {
            let src = rgb.clone();
            let mut y = vec![0_u8; W];
            cases.push(case(
                function,
                "NEON",
                Box::new(move || {
                    if bgr {
                        neon::convert_bgr24_to_y(&src, black_box(&mut y), w);
                    } else {
                        neon::convert_rgb24_to_y(&src, black_box(&mut y), w);
                    }
                }),
            ));
        }
    }

    let src = argb.clone();
    let mut y = vec![0_u8; W];
    cases.push(case(
        "WebPConvertARGBToY",
        "C",
        Box::new(move || c::convert_argb_to_y_c(&src, black_box(&mut y), w)),
    ));
    #[cfg(all(feature = "simd", target_arch = "aarch64"))]
    {
        let src = argb.clone();
        let mut y = vec![0_u8; W];
        cases.push(case(
            "WebPConvertARGBToY",
            "NEON",
            Box::new(move || neon::convert_argb_to_y(&src, black_box(&mut y), w)),
        ));
    }

    let src = argb.clone();
    let (mut u, mut v) = (vec![0_u8; W / 2], vec![0_u8; W / 2]);
    cases.push(case(
        "WebPConvertARGBToUV",
        "C",
        Box::new(move || {
            c::webp_convert_argb_to_uv_c(&src, black_box(&mut u), black_box(&mut v), w, 0);
        }),
    ));
    #[cfg(all(feature = "simd", target_arch = "aarch64"))]
    {
        let src = argb.clone();
        let (mut u, mut v) = (vec![0_u8; W / 2], vec![0_u8; W / 2]);
        cases.push(case(
            "WebPConvertARGBToUV",
            "NEON",
            Box::new(move || {
                neon::convert_argb_to_uv(&src, black_box(&mut u), black_box(&mut v), w, 0);
            }),
        ));
    }

    let src = rgba.clone();
    let (mut u, mut v) = (vec![0_u8; W], vec![0_u8; W]);
    cases.push(case(
        "WebPConvertRGBA32ToUV",
        "C",
        Box::new(move || {
            c::webp_convert_rgba32_to_uv_c(&src, black_box(&mut u), black_box(&mut v), w);
        }),
    ));
    #[cfg(all(feature = "simd", target_arch = "aarch64"))]
    {
        let (mut u, mut v) = (vec![0_u8; W], vec![0_u8; W]);
        cases.push(case(
            "WebPConvertRGBA32ToUV",
            "NEON",
            Box::new(move || {
                neon::convert_rgba32_to_uv(&rgba, black_box(&mut u), black_box(&mut v), w);
            }),
        ));
    }
    cases
}
