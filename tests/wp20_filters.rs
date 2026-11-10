//! WP20 parity test: the NEON alpha-plane filters of
//! `src/dsp/filters_neon.c` against the C translations they replace
//! (`src/dsp/filters.rs`), Rust against Rust, and the table functions
//! `webp_filters` and `webp_unfilters` against the C functions. Planes and
//! rows end exactly at the last byte the C function touches and start at
//! offsets 0 to 3 of a larger allocation.

#![cfg(all(feature = "simd", target_arch = "aarch64"))]

#[path = "support/wp20.rs"]
mod support;

use support::Rng;
use webp::dsp::dsp::WEBP_FILTER_TYPE;
use webp::dsp::filters as c;
use webp::dsp::filters::{WebPFilterFunc, WebPUnfilterFunc};
use webp::dsp::filters_neon as neon;

fn plane(rng: &mut Rng, kind: u8, len: usize) -> Vec<u8> {
    match kind {
        0 => rng.bytes(len),
        // Alternating extremes drive the gradient predictor to both clips.
        1 => (0..len)
            .map(|i| if (i / 3) % 2 == 0 { 0 } else { 255 })
            .collect(),
        2 => (0..len).map(|i| (i * 3) as u8).collect(),
        _ => rng.edge_bytes(len),
    }
}

#[test]
fn filters_match_c() {
    let mut rng = Rng(0x2020_0401);
    let filters: [(WEBP_FILTER_TYPE, WebPFilterFunc, WebPFilterFunc); 3] = [
        (
            WEBP_FILTER_TYPE::WEBP_FILTER_HORIZONTAL,
            c::horizontal_filter_c,
            neon::horizontal_filter,
        ),
        (
            WEBP_FILTER_TYPE::WEBP_FILTER_VERTICAL,
            c::vertical_filter_c,
            neon::vertical_filter,
        ),
        (
            WEBP_FILTER_TYPE::WEBP_FILTER_GRADIENT,
            c::gradient_filter_c,
            neon::gradient_filter,
        ),
    ];
    let mut cases = 0;
    for (filter, c_fn, neon_fn) in filters {
        for width in (1..=40).chain([63, 64, 65, 66, 129]) {
            for height in [1_usize, 2, 3, 5] {
                for pad in [0, 3] {
                    for kind in 0..4 {
                        let offset = (width + height + pad) % 4;
                        let stride = width + pad;
                        let len = (height - 1) * stride + width;
                        let input = plane(&mut rng, kind, offset + len);
                        let out0 = rng.bytes(offset + len);
                        let (mut expected, mut actual, mut dispatched) =
                            (out0.clone(), out0.clone(), out0);
                        let (w, h, s) = (width as i32, height as i32, stride as i32);
                        c_fn(&input[offset..], w, h, s, &mut expected[offset..]);
                        neon_fn(&input[offset..], w, h, s, &mut actual[offset..]);
                        c::webp_filters(
                            filter,
                            &input[offset..],
                            w,
                            h,
                            s,
                            &mut dispatched[offset..],
                        );
                        assert_eq!(
                            actual, expected,
                            "{filter:?} {width}x{height} pad={pad} kind={kind}"
                        );
                        assert_eq!(dispatched, expected);
                        cases += 1;
                    }
                }
            }
        }
    }
    assert!(cases > 4000);
}

#[test]
fn unfilters_match_c() {
    let mut rng = Rng(0x2020_0402);
    let unfilters: [(WEBP_FILTER_TYPE, WebPUnfilterFunc, WebPUnfilterFunc); 2] = [
        (
            WEBP_FILTER_TYPE::WEBP_FILTER_HORIZONTAL,
            c::horizontal_unfilter_c,
            neon::horizontal_unfilter,
        ),
        (
            WEBP_FILTER_TYPE::WEBP_FILTER_VERTICAL,
            c::vertical_unfilter_c,
            neon::vertical_unfilter,
        ),
    ];
    for (filter, c_fn, neon_fn) in unfilters {
        for width in (1..=70).chain([127, 128, 129, 255, 256, 257]) {
            for with_prev in [false, true] {
                for kind in 0..4 {
                    let offset = width % 4;
                    let input = plane(&mut rng, kind, offset + width);
                    let prev = plane(&mut rng, (kind + 1) % 4, offset + width);
                    let prev = with_prev.then(|| &prev[offset..]);
                    let out0 = rng.bytes(offset + width);
                    let (mut expected, mut actual, mut dispatched) =
                        (out0.clone(), out0.clone(), out0);
                    let w = width as i32;
                    c_fn(prev, &input[offset..], &mut expected[offset..], w);
                    neon_fn(prev, &input[offset..], &mut actual[offset..], w);
                    c::webp_unfilters(filter, prev, &input[offset..], &mut dispatched[offset..], w);
                    assert_eq!(
                        actual, expected,
                        "{filter:?} width={width} prev={with_prev}"
                    );
                    assert_eq!(dispatched, expected);
                }
            }
        }
    }
}
