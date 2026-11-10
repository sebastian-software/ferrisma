//! Cases of `src/dsp/alpha_processing.rs` whose pointers
//! `WebPInitAlphaProcessingSSE2` and `SSE41` assign (WP18), over planes of
//! 256 by 16 pixels and rows of 4096 pixels.

use std::hint::black_box;

use webp::dsp::alpha_processing::{
    webp_alpha_replace, webp_apply_alpha_multiply, webp_dispatch_alpha,
    webp_dispatch_alpha_to_green, webp_extract_alpha, webp_extract_green, webp_has_alpha8b,
    webp_has_alpha32b, webp_mult_argb_row, webp_mult_row,
};

use crate::x86::{SSE2, SSE41, levels};
use crate::{Case, Rng};

const MODULE: &str = "alpha";
const W: usize = 256;
const H: usize = 16;
const ROW: usize = 4096;

pub fn cases() -> Vec<Case> {
    let mut cases = Vec::new();
    for (function, inverse) in [
        ("webp_mult_argb_row", false),
        ("webp_mult_argb_row(inverse)", true),
    ] {
        for level in levels(&[SSE2]) {
            let mut rng = Rng::new(31);
            let mut row: Vec<u32> = (0..ROW).map(|_| rng.next_u64() as u32).collect();
            cases.push(level.case(MODULE, function, move || {
                webp_mult_argb_row(black_box(&mut row), ROW as i32, inverse);
            }));
        }
    }
    for (function, inverse) in [("webp_mult_row", false), ("webp_mult_row(inverse)", true)] {
        for level in levels(&[SSE2]) {
            let mut rng = Rng::new(32);
            let mut row = rng.bytes(ROW);
            let alpha = rng.bytes(ROW);
            cases.push(level.case(MODULE, function, move || {
                webp_mult_row(black_box(&mut row), &alpha, ROW as i32, inverse);
            }));
        }
    }
    for level in levels(&[SSE2]) {
        let mut rng = Rng::new(33);
        let mut rgba = rng.bytes(4 * W * H);
        cases.push(level.case(MODULE, "webp_apply_alpha_multiply", move || {
            webp_apply_alpha_multiply(
                black_box(&mut rgba),
                false,
                W as i32,
                H as i32,
                4 * W as i32,
            );
        }));
    }
    for level in levels(&[SSE2]) {
        let mut rng = Rng::new(34);
        let alpha = rng.bytes(W * H);
        let mut dst = rng.bytes(4 * W * H);
        cases.push(level.case(MODULE, "webp_dispatch_alpha", move || {
            black_box(webp_dispatch_alpha(
                &alpha,
                W as i32,
                W as i32,
                H as i32,
                black_box(&mut dst),
                4 * W as i32,
            ));
        }));
    }
    for level in levels(&[SSE2]) {
        let mut rng = Rng::new(35);
        let alpha = rng.bytes(W * H);
        let mut dst = vec![0_u32; W * H];
        cases.push(level.case(MODULE, "webp_dispatch_alpha_to_green", move || {
            webp_dispatch_alpha_to_green(
                &alpha,
                W as i32,
                W as i32,
                H as i32,
                black_box(&mut dst),
                W as i32,
            );
        }));
    }
    for level in levels(&[SSE2, SSE41]) {
        let mut rng = Rng::new(36);
        let argb = rng.bytes(4 * W * H);
        let mut alpha = vec![0; W * H];
        cases.push(level.case(MODULE, "webp_extract_alpha", move || {
            black_box(webp_extract_alpha(
                black_box(&argb),
                4 * W as i32,
                W as i32,
                H as i32,
                &mut alpha,
                W as i32,
            ));
        }));
    }
    for level in levels(&[SSE2]) {
        let mut rng = Rng::new(37);
        let argb: Vec<u32> = (0..ROW).map(|_| rng.next_u64() as u32).collect();
        let mut alpha = vec![0; ROW];
        cases.push(level.case(MODULE, "webp_extract_green", move || {
            webp_extract_green(black_box(&argb), black_box(&mut alpha), ROW as i32);
        }));
    }
    // All opaque, so that the tests scan the whole row.
    for level in levels(&[SSE2]) {
        let src = vec![0xff_u8; ROW];
        cases.push(level.case(MODULE, "webp_has_alpha8b", move || {
            black_box(webp_has_alpha8b(black_box(&src), ROW as i32));
        }));
    }
    for level in levels(&[SSE2]) {
        let src = vec![0xff_u8; 4 * ROW];
        cases.push(level.case(MODULE, "webp_has_alpha32b", move || {
            black_box(webp_has_alpha32b(black_box(&src), ROW as i32));
        }));
    }
    for level in levels(&[SSE2]) {
        let mut rng = Rng::new(38);
        let mut src: Vec<u32> = (0..ROW)
            .map(|_| {
                if rng.next_u64() & 1 == 0 {
                    0
                } else {
                    rng.next_u64() as u32
                }
            })
            .collect();
        cases.push(level.case(MODULE, "webp_alpha_replace", move || {
            webp_alpha_replace(black_box(&mut src), ROW as i32, 0x0012_3456);
        }));
    }
    cases
}
