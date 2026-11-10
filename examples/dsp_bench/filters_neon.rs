//! Benchmark cases of `src/dsp/filters_neon.rs` (WP20): the C translation
//! and, with the `simd` feature on 64-bit ARM, the NEON variant of each
//! filter and unfilter that `VP8FiltersInitNEON` stores, on a plane of 256 by
//! 4 pixels and rows of 256 pixels.

use std::hint::black_box;

use webp::dsp::filters as c;
use webp::dsp::filters::{WebPFilterFunc, WebPUnfilterFunc};
#[cfg(all(feature = "simd", target_arch = "aarch64"))]
/// The NEON level of `src/dsp/filters_neon.rs`, reached through the dispatch
/// functions, which select its crate-private entry points with `simd` on
/// 64-bit ARM (the residual costs while the `VP8GetCPUInfo` query answers
/// `kNEON`, as it does by default); named as the entry points.
mod neon {
    pub fn gradient_filter(data: &[u8], width: i32, height: i32, stride: i32, out: &mut [u8]) {
        webp::dsp::filters::webp_filters(
            webp::dsp::dsp::WEBP_FILTER_TYPE::WEBP_FILTER_GRADIENT,
            data,
            width,
            height,
            stride,
            out,
        );
    }
    pub fn horizontal_filter(data: &[u8], width: i32, height: i32, stride: i32, out: &mut [u8]) {
        webp::dsp::filters::webp_filters(
            webp::dsp::dsp::WEBP_FILTER_TYPE::WEBP_FILTER_HORIZONTAL,
            data,
            width,
            height,
            stride,
            out,
        );
    }
    pub fn horizontal_unfilter(prev: Option<&[u8]>, input: &[u8], out: &mut [u8], width: i32) {
        webp::dsp::filters::webp_unfilters(
            webp::dsp::dsp::WEBP_FILTER_TYPE::WEBP_FILTER_HORIZONTAL,
            prev,
            input,
            out,
            width,
        );
    }
    pub fn vertical_filter(data: &[u8], width: i32, height: i32, stride: i32, out: &mut [u8]) {
        webp::dsp::filters::webp_filters(
            webp::dsp::dsp::WEBP_FILTER_TYPE::WEBP_FILTER_VERTICAL,
            data,
            width,
            height,
            stride,
            out,
        );
    }
    pub fn vertical_unfilter(prev: Option<&[u8]>, input: &[u8], out: &mut [u8], width: i32) {
        webp::dsp::filters::webp_unfilters(
            webp::dsp::dsp::WEBP_FILTER_TYPE::WEBP_FILTER_VERTICAL,
            prev,
            input,
            out,
            width,
        );
    }
}

use super::{Case, Rng};

const W: usize = 256;
const H: usize = 4;

fn filter_case(
    rng: &mut Rng,
    function: &'static str,
    variant: &'static str,
    f: WebPFilterFunc,
) -> Case {
    let input = rng.bytes(W * H);
    let mut out = vec![0_u8; W * H];
    Case {
        module: "filters",
        function,
        variant,
        run: Box::new(move || f(&input, W as i32, H as i32, W as i32, black_box(&mut out))),
    }
}

fn unfilter_case(
    rng: &mut Rng,
    function: &'static str,
    variant: &'static str,
    f: WebPUnfilterFunc,
) -> Case {
    let prev = rng.bytes(W);
    let input = rng.bytes(W);
    let mut out = vec![0_u8; W];
    Case {
        module: "filters",
        function,
        variant,
        run: Box::new(move || f(Some(&prev), &input, black_box(&mut out), W as i32)),
    }
}

/// The C and NEON cases of each entry that `VP8FiltersInitNEON` stores.
pub fn cases() -> Vec<Case> {
    let mut rng = Rng::new(0x2020_0d00);
    let mut cases = Vec::new();
    cases.extend([
        filter_case(&mut rng, "HorizontalFilter", "C", c::horizontal_filter_c),
        filter_case(&mut rng, "VerticalFilter", "C", c::vertical_filter_c),
        filter_case(&mut rng, "GradientFilter", "C", c::gradient_filter_c),
        unfilter_case(
            &mut rng,
            "HorizontalUnfilter",
            "C",
            c::horizontal_unfilter_c,
        ),
        unfilter_case(&mut rng, "VerticalUnfilter", "C", c::vertical_unfilter_c),
    ]);
    #[cfg(all(feature = "simd", target_arch = "aarch64"))]
    cases.extend([
        filter_case(
            &mut rng,
            "HorizontalFilter",
            "NEON",
            neon::horizontal_filter,
        ),
        filter_case(&mut rng, "VerticalFilter", "NEON", neon::vertical_filter),
        filter_case(&mut rng, "GradientFilter", "NEON", neon::gradient_filter),
        unfilter_case(
            &mut rng,
            "HorizontalUnfilter",
            "NEON",
            neon::horizontal_unfilter,
        ),
        unfilter_case(
            &mut rng,
            "VerticalUnfilter",
            "NEON",
            neon::vertical_unfilter,
        ),
    ]);
    cases
}
