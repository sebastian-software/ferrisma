//! WP20 parity test: the NEON fancy upsamplers of
//! `src/dsp/upsampling_neon.c` against the C translations they replace
//! (`src/dsp/upsampling.rs`), Rust against Rust, for the seven formats, the
//! eleven table entries of `WebPUpsamplers` and `WebPGetLinePairConverter`.
//! Rows and outputs end exactly at the last sample the C function touches and
//! start at offsets 0 to 3 of a larger allocation.

#![cfg(all(feature = "simd", target_arch = "aarch64"))]

#[path = "support/wp20.rs"]
mod support;

use support::Rng;
use webp::dsp::upsampling as c;
use webp::dsp::upsampling::WebPUpsampleLinePairFunc;
use webp::dsp::upsampling_neon as neon;
use webp::webp::decode::WEBP_CSP_MODE;

/// The formats: C function, NEON entry point, bytes per pixel, modes.
fn formats() -> Vec<(
    WebPUpsampleLinePairFunc,
    WebPUpsampleLinePairFunc,
    usize,
    &'static [WEBP_CSP_MODE],
)> {
    vec![
        (
            c::upsample_rgba_line_pair_c,
            neon::upsample_rgba_line_pair,
            4,
            &[WEBP_CSP_MODE::MODE_RGBA, WEBP_CSP_MODE::MODE_rgbA],
        ),
        (
            c::upsample_bgra_line_pair_c,
            neon::upsample_bgra_line_pair,
            4,
            &[WEBP_CSP_MODE::MODE_BGRA, WEBP_CSP_MODE::MODE_bgrA],
        ),
        (
            c::upsample_rgb_line_pair_c,
            neon::upsample_rgb_line_pair,
            3,
            &[WEBP_CSP_MODE::MODE_RGB],
        ),
        (
            c::upsample_bgr_line_pair_c,
            neon::upsample_bgr_line_pair,
            3,
            &[WEBP_CSP_MODE::MODE_BGR],
        ),
        (
            c::upsample_argb_line_pair_c,
            neon::upsample_argb_line_pair,
            4,
            &[WEBP_CSP_MODE::MODE_ARGB, WEBP_CSP_MODE::MODE_Argb],
        ),
        (
            c::upsample_rgba4444_line_pair_c,
            neon::upsample_rgba4444_line_pair,
            2,
            &[WEBP_CSP_MODE::MODE_RGBA_4444, WEBP_CSP_MODE::MODE_rgbA_4444],
        ),
        (
            c::upsample_rgb565_line_pair_c,
            neon::upsample_rgb565_line_pair,
            2,
            &[WEBP_CSP_MODE::MODE_RGB_565],
        ),
    ]
}

/// The inputs of one line pair, each with `offset` bytes before it.
struct Pair {
    offset: usize,
    top_y: Vec<u8>,
    bottom_y: Vec<u8>,
    planes: [Vec<u8>; 4],
}

impl Pair {
    fn new(rng: &mut Rng, offset: usize, len: usize, flat: Option<u8>) -> Self {
        let uv_len = len.div_ceil(2);
        let mut row = |count: usize| match flat {
            Some(value) => vec![value; offset + count],
            None => rng.edge_bytes(offset + count),
        };
        Self {
            offset,
            top_y: row(len),
            bottom_y: row(len),
            planes: [row(uv_len), row(uv_len), row(uv_len), row(uv_len)],
        }
    }

    /// Runs `f` and returns the top and bottom outputs.
    fn run(
        &self,
        f: impl Fn(&[u8], Option<(&[u8], &mut [u8])>, &[u8], &[u8], &[u8], &[u8], &mut [u8], i32),
        len: usize,
        xstep: usize,
        bottom: bool,
        fill: u8,
    ) -> (Vec<u8>, Vec<u8>) {
        let o = self.offset;
        let mut top = vec![fill; o + len * xstep];
        let mut bot = vec![fill; o + len * xstep];
        let [tu, tv, cu, cv] = &self.planes;
        f(
            &self.top_y[o..],
            bottom.then(|| (&self.bottom_y[o..], &mut bot[o..])),
            &tu[o..],
            &tv[o..],
            &cu[o..],
            &cv[o..],
            &mut top[o..],
            i32::try_from(len).expect("int"),
        );
        (top, bot)
    }
}

#[test]
fn upsamplers_match_c() {
    let mut rng = Rng(0x2020_0301);
    let lengths: Vec<usize> = (1..=70).chain([127, 128, 129, 255, 256, 257]).collect();
    let mut cases = 0;
    for (c_fn, neon_fn, xstep, modes) in formats() {
        for &len in &lengths {
            for offset in 0..4 {
                for flat in [None, Some(0), Some(255)] {
                    let pair = Pair::new(&mut rng, offset, len, flat);
                    for bottom in [false, true] {
                        let fill = rng.byte();
                        let expected = pair.run(c_fn, len, xstep, bottom, fill);
                        let actual = pair.run(neon_fn, len, xstep, bottom, fill);
                        assert!(
                            actual == expected,
                            "xstep={xstep} len={len} bottom={bottom}"
                        );
                        for &mode in modes {
                            let dispatched = pair.run(
                                |ty, b, tu, tv, cu, cv, d, l| {
                                    c::webp_upsamplers(mode, ty, b, tu, tv, cu, cv, d, l);
                                },
                                len,
                                xstep,
                                bottom,
                                fill,
                            );
                            assert!(dispatched == expected, "mode={mode:?} len={len}");
                        }
                        cases += 1;
                    }
                }
            }
        }
    }
    assert!(cases > 10_000);
}

#[test]
fn line_pair_converter_matches_c() {
    let mut rng = Rng(0x2020_0302);
    for (alpha_is_last, c_fn) in [
        (
            true,
            c::upsample_bgra_line_pair_c as WebPUpsampleLinePairFunc,
        ),
        (false, c::upsample_argb_line_pair_c),
    ] {
        let converter = c::webp_get_line_pair_converter(alpha_is_last);
        for len in 1..=66 {
            let pair = Pair::new(&mut rng, len % 4, len, None);
            for bottom in [false, true] {
                let expected = pair.run(c_fn, len, 4, bottom, 0x55);
                let actual = pair.run(converter, len, 4, bottom, 0x55);
                assert!(
                    actual == expected,
                    "alpha_is_last={alpha_is_last} len={len}"
                );
            }
        }
    }
}
