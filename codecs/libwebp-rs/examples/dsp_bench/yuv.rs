//! Cases of `src/dsp/yuv.rs` whose pointers `WebPInitSamplersSSE2`/`SSE41`
//! and `WebPInitConvertARGBToYUVSSE2`/`SSE41` assign (WP18): the row samplers
//! and the RGB-to-YUV converters, over a row of 512 pixels.

use std::hint::black_box;

use webp::dsp::yuv::{
    webp_convert_argb_to_uv, webp_convert_argb_to_y, webp_convert_bgr24_to_y,
    webp_convert_rgb24_to_y, webp_convert_rgba32_to_uv, webp_samplers,
};
use webp::webp::decode::WEBP_CSP_MODE;

use crate::x86::{SSE2, SSE41, levels};
use crate::{Case, Rng};

const MODULE: &str = "yuv";
const WIDTH: usize = 512;

pub fn cases() -> Vec<Case> {
    let mut cases = Vec::new();
    let samplers = [
        (WEBP_CSP_MODE::MODE_RGB, "webp_samplers[RGB]", 3, true),
        (WEBP_CSP_MODE::MODE_RGBA, "webp_samplers[RGBA]", 4, false),
        (WEBP_CSP_MODE::MODE_BGR, "webp_samplers[BGR]", 3, true),
        (WEBP_CSP_MODE::MODE_BGRA, "webp_samplers[BGRA]", 4, false),
        (WEBP_CSP_MODE::MODE_ARGB, "webp_samplers[ARGB]", 4, false),
    ];
    for (mode, function, bytes, has_sse41) in samplers {
        let simd = if has_sse41 {
            &[SSE2, SSE41][..]
        } else {
            &[SSE2][..]
        };
        for level in levels(simd) {
            let mut rng = Rng::new(11);
            let y = rng.bytes(WIDTH);
            let u = rng.bytes(WIDTH / 2);
            let v = rng.bytes(WIDTH / 2);
            let mut dst = vec![0; WIDTH * bytes];
            cases.push(level.case(MODULE, function, move || {
                webp_samplers(
                    mode,
                    black_box(&y),
                    &u,
                    &v,
                    black_box(&mut dst),
                    WIDTH as i32,
                );
            }));
        }
    }
    for level in levels(&[SSE2, SSE41]) {
        let mut rng = Rng::new(12);
        let argb: Vec<u32> = (0..WIDTH).map(|_| rng.next_u64() as u32).collect();
        let mut y = vec![0; WIDTH];
        cases.push(level.case(MODULE, "webp_convert_argb_to_y", move || {
            webp_convert_argb_to_y(black_box(&argb), black_box(&mut y), WIDTH as i32);
        }));
    }
    for level in levels(&[SSE2, SSE41]) {
        let mut rng = Rng::new(13);
        let argb: Vec<u32> = (0..WIDTH).map(|_| rng.next_u64() as u32).collect();
        let mut u = vec![0; WIDTH / 2];
        let mut v = vec![0; WIDTH / 2];
        cases.push(level.case(MODULE, "webp_convert_argb_to_uv", move || {
            webp_convert_argb_to_uv(black_box(&argb), &mut u, black_box(&mut v), WIDTH as i32, 1);
        }));
    }
    for (function, bgr) in [
        ("webp_convert_rgb24_to_y", false),
        ("webp_convert_bgr24_to_y", true),
    ] {
        for level in levels(&[SSE2, SSE41]) {
            let mut rng = Rng::new(14);
            let rgb = rng.bytes(3 * WIDTH);
            let mut y = vec![0; WIDTH];
            cases.push(level.case(MODULE, function, move || {
                if bgr {
                    webp_convert_bgr24_to_y(black_box(&rgb), black_box(&mut y), WIDTH as i32);
                } else {
                    webp_convert_rgb24_to_y(black_box(&rgb), black_box(&mut y), WIDTH as i32);
                }
            }));
        }
    }
    for level in levels(&[SSE2, SSE41]) {
        let mut rng = Rng::new(15);
        // Sums of 2x2 pixels, as the encoder passes them.
        let rgba: Vec<u16> = (0..4 * WIDTH / 2)
            .map(|_| (rng.next_u64() % 1021) as u16)
            .collect();
        let mut u = vec![0; WIDTH / 2];
        let mut v = vec![0; WIDTH / 2];
        cases.push(level.case(MODULE, "webp_convert_rgba32_to_uv", move || {
            webp_convert_rgba32_to_uv(
                black_box(&rgba),
                &mut u,
                black_box(&mut v),
                (WIDTH / 2) as i32,
            );
        }));
    }
    cases
}
