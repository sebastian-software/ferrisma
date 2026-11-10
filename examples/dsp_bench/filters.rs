//! Cases of `src/dsp/filters.rs` whose table entries `VP8FiltersInitSSE2`
//! assigns (WP18): the three forward filters over a 256 by 64 plane and the
//! three unfilters over a row of 4096 bytes.

use std::hint::black_box;

use webp::dsp::dsp::WEBP_FILTER_TYPE;
use webp::dsp::filters::{webp_filters, webp_unfilters};

use crate::x86::{SSE2, levels};
use crate::{Case, Rng};

const MODULE: &str = "filters";
const W: usize = 256;
const H: usize = 64;
const ROW: usize = 4096;

const FILTERS: [(WEBP_FILTER_TYPE, &str, &str); 3] = [
    (
        WEBP_FILTER_TYPE::WEBP_FILTER_HORIZONTAL,
        "webp_filters[HORIZONTAL]",
        "webp_unfilters[HORIZONTAL]",
    ),
    (
        WEBP_FILTER_TYPE::WEBP_FILTER_VERTICAL,
        "webp_filters[VERTICAL]",
        "webp_unfilters[VERTICAL]",
    ),
    (
        WEBP_FILTER_TYPE::WEBP_FILTER_GRADIENT,
        "webp_filters[GRADIENT]",
        "webp_unfilters[GRADIENT]",
    ),
];

pub fn cases() -> Vec<Case> {
    let mut cases = Vec::new();
    for (filter, function, _) in FILTERS {
        for level in levels(&[SSE2]) {
            let mut rng = Rng::new(41);
            let data = rng.bytes(W * H);
            let mut filtered = vec![0; W * H];
            cases.push(level.case(MODULE, function, move || {
                webp_filters(
                    filter,
                    black_box(&data),
                    W as i32,
                    H as i32,
                    W as i32,
                    black_box(&mut filtered),
                );
            }));
        }
    }
    for (filter, _, function) in FILTERS {
        for level in levels(&[SSE2]) {
            let mut rng = Rng::new(42);
            let prev = rng.bytes(ROW);
            let input = rng.bytes(ROW);
            let mut out = vec![0; ROW];
            cases.push(level.case(MODULE, function, move || {
                webp_unfilters(
                    filter,
                    Some(&prev),
                    black_box(&input),
                    black_box(&mut out),
                    ROW as i32,
                );
            }));
        }
    }
    cases
}
