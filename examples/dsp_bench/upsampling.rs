//! Cases of `src/dsp/upsampling.rs` whose pointers `WebPInitUpsamplersSSE2`/
//! `SSE41` and `WebPInitYUV444ConvertersSSE2`/`SSE41` assign (WP18): the
//! fancy upsamplers of a line pair and the 4:4:4 converters, over a row of
//! 512 pixels.

use std::hint::black_box;

use webp::dsp::upsampling::{webp_upsamplers, webp_yuv444_converters};
use webp::webp::decode::WEBP_CSP_MODE;

use crate::x86::{SSE2, SSE41, levels};
use crate::{Case, Rng};

const MODULE: &str = "upsampling";
const WIDTH: usize = 512;

/// The modes with an SSE2 variant, their names, bytes per pixel, and whether
/// SSE4.1 overrides them.
const MODES: [(WEBP_CSP_MODE, &str, &str, usize, bool); 7] = [
    (
        WEBP_CSP_MODE::MODE_RGBA,
        "webp_upsamplers[RGBA]",
        "webp_yuv444_converters[RGBA]",
        4,
        false,
    ),
    (
        WEBP_CSP_MODE::MODE_BGRA,
        "webp_upsamplers[BGRA]",
        "webp_yuv444_converters[BGRA]",
        4,
        false,
    ),
    (
        WEBP_CSP_MODE::MODE_RGB,
        "webp_upsamplers[RGB]",
        "webp_yuv444_converters[RGB]",
        3,
        true,
    ),
    (
        WEBP_CSP_MODE::MODE_BGR,
        "webp_upsamplers[BGR]",
        "webp_yuv444_converters[BGR]",
        3,
        true,
    ),
    (
        WEBP_CSP_MODE::MODE_ARGB,
        "webp_upsamplers[ARGB]",
        "webp_yuv444_converters[ARGB]",
        4,
        false,
    ),
    (
        WEBP_CSP_MODE::MODE_RGB_565,
        "webp_upsamplers[RGB_565]",
        "webp_yuv444_converters[RGB_565]",
        2,
        false,
    ),
    (
        WEBP_CSP_MODE::MODE_RGBA_4444,
        "webp_upsamplers[RGBA_4444]",
        "webp_yuv444_converters[RGBA_4444]",
        2,
        false,
    ),
];

pub fn cases() -> Vec<Case> {
    let mut cases = Vec::new();
    for (mode, upsampler, _, bytes, has_sse41) in MODES {
        let simd = if has_sse41 {
            &[SSE2, SSE41][..]
        } else {
            &[SSE2][..]
        };
        for level in levels(simd) {
            let mut rng = Rng::new(21);
            let top_y = rng.bytes(WIDTH);
            let bottom_y = rng.bytes(WIDTH);
            let uv: Vec<Vec<u8>> = (0..4).map(|_| rng.bytes(WIDTH / 2 + 1)).collect();
            let mut top_dst = vec![0; WIDTH * bytes];
            let mut bottom_dst = vec![0; WIDTH * bytes];
            cases.push(level.case(MODULE, upsampler, move || {
                webp_upsamplers(
                    mode,
                    black_box(&top_y),
                    Some((&bottom_y, black_box(&mut bottom_dst))),
                    &uv[0],
                    &uv[1],
                    &uv[2],
                    &uv[3],
                    black_box(&mut top_dst),
                    WIDTH as i32,
                );
            }));
        }
    }
    for (mode, _, converter, bytes, has_sse41) in MODES {
        let simd = if has_sse41 {
            &[SSE2, SSE41][..]
        } else {
            &[SSE2][..]
        };
        for level in levels(simd) {
            let mut rng = Rng::new(22);
            let y = rng.bytes(WIDTH);
            let u = rng.bytes(WIDTH);
            let v = rng.bytes(WIDTH);
            let mut dst = vec![0; WIDTH * bytes];
            cases.push(level.case(MODULE, converter, move || {
                webp_yuv444_converters(
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
    cases
}
