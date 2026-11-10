//! Cases of `src/dsp/rescaler.rs` whose pointers `WebPRescalerDspInitSSE2`
//! assigns (WP18): the import and export rows of a shrinking and an
//! expanding rescaler of four channels, 512 pixels wide on the large side.

use std::hint::black_box;

use webp::dsp::rescaler::{
    webp_rescaler_export_row_expand, webp_rescaler_export_row_shrink,
    webp_rescaler_import_row_expand, webp_rescaler_import_row_shrink,
};
use webp::utils::rescaler_utils::{WebPRescaler, webp_rescaler_init};

use crate::x86::{SSE2, levels};
use crate::{Case, Rng};

const MODULE: &str = "rescaler";
const CHANNELS: i32 = 4;

/// A rescaler from `src` to `dst` pixels in both directions, with one row
/// imported.
fn rescaler(src: i32, dst: i32, row: &[u8]) -> WebPRescaler {
    let work = vec![0; 2 * usize::try_from(dst * CHANNELS).unwrap()];
    let mut wrk = webp_rescaler_init(src, src, dst, dst, dst * CHANNELS, CHANNELS, work)
        .expect("a valid rescaler");
    if wrk.x_expand {
        webp_rescaler_import_row_expand(&mut wrk, row);
    } else {
        webp_rescaler_import_row_shrink(&mut wrk, row);
    }
    wrk
}

pub fn cases() -> Vec<Case> {
    let mut cases = Vec::new();
    for (function, src, dst) in [
        ("webp_rescaler_import_row_shrink", 512, 200),
        ("webp_rescaler_import_row_expand", 200, 512),
    ] {
        for level in levels(&[SSE2]) {
            let mut rng = Rng::new(51);
            let row = rng.bytes(usize::try_from(src * CHANNELS).unwrap());
            let mut wrk = rescaler(src, dst, &row);
            cases.push(level.case(MODULE, function, move || {
                if src > dst {
                    webp_rescaler_import_row_shrink(black_box(&mut wrk), black_box(&row));
                } else {
                    webp_rescaler_import_row_expand(black_box(&mut wrk), black_box(&row));
                }
            }));
        }
    }
    for (function, src, dst) in [
        ("webp_rescaler_export_row_shrink", 512, 200),
        ("webp_rescaler_export_row_expand", 200, 512),
    ] {
        for level in levels(&[SSE2]) {
            let mut rng = Rng::new(52);
            let row = rng.bytes(usize::try_from(src * CHANNELS).unwrap());
            let mut wrk = rescaler(src, dst, &row);
            let mut out = vec![0; usize::try_from(dst * CHANNELS).unwrap()];
            cases.push(level.case(MODULE, function, move || {
                if src > dst {
                    webp_rescaler_export_row_shrink(black_box(&mut wrk), black_box(&mut out));
                } else {
                    webp_rescaler_export_row_expand(black_box(&mut wrk), black_box(&mut out));
                }
            }));
        }
    }
    cases
}
