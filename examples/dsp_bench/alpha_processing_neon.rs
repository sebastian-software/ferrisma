//! Benchmark cases of `src/dsp/alpha_processing_neon.rs` (WP20): the C
//! translation and, with the `simd` feature on 64-bit ARM, the NEON variant
//! of each pointer that `WebPInitAlphaProcessingNEON` assigns, on a plane of
//! 64 by 4 pixels.

use std::hint::black_box;

use webp::dsp::alpha_processing as c;
#[cfg(all(feature = "simd", target_arch = "aarch64"))]
use webp::dsp::alpha_processing_neon as neon;

use super::{Case, Rng};

const W: usize = 64;
const H: usize = 4;

fn case(function: &'static str, variant: &'static str, run: Box<dyn FnMut()>) -> Case {
    Case {
        module: "alpha_processing",
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

/// The C and NEON cases of each pointer that `WebPInitAlphaProcessingNEON`
/// assigns.
#[expect(
    clippy::too_many_lines,
    reason = "one C and one NEON case per dispatch function"
)]
pub fn cases() -> Vec<Case> {
    let mut rng = Rng::new(0x2020_0a00);
    let mut cases = Vec::new();
    let (w, h) = (W as i32, H as i32);

    let rgba = rng.bytes(4 * W * H);
    let mut buf = rgba.clone();
    cases.push(case(
        "WebPApplyAlphaMultiply",
        "C",
        Box::new(move || {
            buf.copy_from_slice(&rgba);
            c::apply_alpha_multiply_c(black_box(&mut buf), false, w, h, 4 * w);
        }),
    ));
    #[cfg(all(feature = "simd", target_arch = "aarch64"))]
    {
        let rgba = rng.bytes(4 * W * H);
        let mut buf = rgba.clone();
        cases.push(case(
            "WebPApplyAlphaMultiply",
            "NEON",
            Box::new(move || {
                buf.copy_from_slice(&rgba);
                neon::apply_alpha_multiply(black_box(&mut buf), false, w, h, 4 * w);
            }),
        ));
    }

    let alpha = rng.bytes(W * H);
    let mut dst = rng.bytes(4 * W * H);
    let alpha_c = alpha.clone();
    cases.push(case(
        "WebPDispatchAlpha",
        "C",
        Box::new(move || {
            black_box(c::dispatch_alpha_c(
                &alpha_c,
                w,
                w,
                h,
                black_box(&mut dst),
                4 * w,
            ));
        }),
    ));
    #[cfg(all(feature = "simd", target_arch = "aarch64"))]
    {
        let alpha = alpha.clone();
        let mut dst = rng.bytes(4 * W * H);
        cases.push(case(
            "WebPDispatchAlpha",
            "NEON",
            Box::new(move || {
                black_box(neon::dispatch_alpha(
                    &alpha,
                    w,
                    w,
                    h,
                    black_box(&mut dst),
                    4 * w,
                ));
            }),
        ));
    }

    let mut green = words(&mut rng, W * H);
    let alpha_c = alpha.clone();
    cases.push(case(
        "WebPDispatchAlphaToGreen",
        "C",
        Box::new(move || {
            c::dispatch_alpha_to_green_c(&alpha_c, w, w, h, black_box(&mut green), w);
        }),
    ));
    #[cfg(all(feature = "simd", target_arch = "aarch64"))]
    {
        let alpha = alpha.clone();
        let mut green = words(&mut rng, W * H);
        cases.push(case(
            "WebPDispatchAlphaToGreen",
            "NEON",
            Box::new(move || {
                neon::dispatch_alpha_to_green(&alpha, w, w, h, black_box(&mut green), w);
            }),
        ));
    }

    let argb = rng.bytes(4 * W * H);
    let mut out = vec![0_u8; W * H];
    let argb_c = argb.clone();
    cases.push(case(
        "WebPExtractAlpha",
        "C",
        Box::new(move || {
            black_box(c::extract_alpha_c(
                &argb_c,
                4 * w,
                w,
                h,
                black_box(&mut out),
                w,
            ));
        }),
    ));
    #[cfg(all(feature = "simd", target_arch = "aarch64"))]
    {
        let argb = argb.clone();
        let mut out = vec![0_u8; W * H];
        cases.push(case(
            "WebPExtractAlpha",
            "NEON",
            Box::new(move || {
                black_box(neon::extract_alpha(
                    &argb,
                    4 * w,
                    w,
                    h,
                    black_box(&mut out),
                    w,
                ));
            }),
        ));
    }

    let pixels = words(&mut rng, W * H);
    let mut out = vec![0_u8; W * H];
    let pixels_c = pixels.clone();
    cases.push(case(
        "WebPExtractGreen",
        "C",
        Box::new(move || c::extract_green_c(&pixels_c, black_box(&mut out), w * h)),
    ));
    #[cfg(all(feature = "simd", target_arch = "aarch64"))]
    {
        let mut out = vec![0_u8; W * H];
        cases.push(case(
            "WebPExtractGreen",
            "NEON",
            Box::new(move || neon::extract_green(&pixels, black_box(&mut out), w * h)),
        ));
    }
    cases
}
