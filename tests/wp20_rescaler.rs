//! WP20 parity test: the NEON row exports of `src/dsp/rescaler_neon.c`
//! against the C translations they replace (`src/dsp/rescaler.rs`), Rust
//! against Rust. Two rescalers run in lockstep over the same input, one
//! exporting through the C functions and one through the NEON entry points,
//! for expansions, shrinks, mixed directions, one to four channels and row
//! lengths around the 8-lane loop; every exported row is compared. The
//! dispatch functions are compared with the C functions on the same states.

#![cfg(all(feature = "simd", target_arch = "aarch64"))]

#[path = "support/wp20.rs"]
mod support;

use support::Rng;
use webp::dsp::rescaler as c;
use webp::dsp::rescaler_neon as neon;
use webp::utils::rescaler_utils::{
    WebPRescaler, webp_rescaler_has_pending_output, webp_rescaler_import, webp_rescaler_init,
};

#[derive(Clone, Copy, PartialEq)]
enum Export {
    C,
    Neon,
    Dispatch,
}

/// `WebPRescalerExportRow` with the row kernels of `export`.
fn export_row(wrk: &mut WebPRescaler, dst: &mut [u8], export: Export) {
    assert!(wrk.y_accum <= 0);
    if wrk.y_expand {
        match export {
            Export::C => c::webp_rescaler_export_row_expand_c(wrk, dst),
            Export::Neon => neon::rescaler_export_row_expand(wrk, dst),
            Export::Dispatch => c::webp_rescaler_export_row_expand(wrk, dst),
        }
    } else if wrk.fxy_scale != 0 {
        match export {
            Export::C => c::webp_rescaler_export_row_shrink_c(wrk, dst),
            Export::Neon => neon::rescaler_export_row_shrink(wrk, dst),
            Export::Dispatch => c::webp_rescaler_export_row_shrink(wrk, dst),
        }
    } else {
        // The special case copies the side row without a kernel.
        c::webp_rescaler_export_row(wrk, dst);
        return;
    }
    wrk.y_accum += wrk.y_add;
    wrk.dst_pos += usize::try_from(wrk.dst_stride).expect("positive stride");
    wrk.dst_y += 1;
}

/// Rescales `src` with the exports of `export`, returning the output rows
/// as they were written.
fn rescale(
    src: &[u8],
    (src_w, src_h): (usize, usize),
    (dst_w, dst_h): (usize, usize),
    channels: usize,
    export: Export,
) -> Vec<u8> {
    let stride = dst_w * channels + 3;
    let mut out = vec![0xaa_u8; (dst_h - 1) * stride + dst_w * channels];
    let int = |v: usize| i32::try_from(v).expect("int");
    let mut wrk = webp_rescaler_init(
        int(src_w),
        int(src_h),
        int(dst_w),
        int(dst_h),
        int(stride),
        int(channels),
        vec![0; 2 * dst_w * channels],
    )
    .expect("rescaler");
    let mut y = 0;
    while y < src_h {
        let rows = webp_rescaler_import(
            &mut wrk,
            int(src_h - y),
            &src[y * src_w * channels..],
            int(src_w * channels),
        );
        y += usize::try_from(rows).expect("rows");
        while webp_rescaler_has_pending_output(&wrk) {
            export_row(&mut wrk, &mut out, export);
        }
    }
    out
}

fn check(rng: &mut Rng, src: (usize, usize), dst: (usize, usize), channels: usize) {
    let len = src.0 * src.1 * channels;
    for kind in 0..3 {
        let pixels: Vec<u8> = match kind {
            0 => rng.bytes(len),
            1 => vec![255; len],
            _ => (0..len).map(|i| (i * 7 % 256) as u8).collect(),
        };
        let expected = rescale(&pixels, src, dst, channels, Export::C);
        let actual = rescale(&pixels, src, dst, channels, Export::Neon);
        let dispatched = rescale(&pixels, src, dst, channels, Export::Dispatch);
        assert!(
            actual == expected,
            "src={src:?} dst={dst:?} channels={channels} kind={kind}"
        );
        assert!(dispatched == expected);
    }
}

#[test]
fn exports_match_c_on_fixed_sizes() {
    let mut rng = Rng(0x2020_0201);
    let sizes = [1_usize, 2, 3, 7, 8, 9, 15, 16, 17, 31, 33, 64, 100];
    for channels in 1..=4 {
        for &sw in &sizes {
            for &dw in &sizes {
                for (sh, dh) in [(1, 5), (5, 1), (7, 3), (3, 7), (16, 16), (9, 4)] {
                    check(&mut rng, (sw, sh), (dw, dh), channels);
                }
            }
        }
    }
}

#[test]
fn exports_match_c_on_random_sizes() {
    let mut rng = Rng(0x2020_0202);
    for _ in 0..600 {
        let sw = 1 + rng.below(300) as usize;
        let sh = 1 + rng.below(80) as usize;
        let dw = 1 + rng.below(300) as usize;
        let dh = 1 + rng.below(80) as usize;
        let channels = 1 + rng.below(4) as usize;
        check(&mut rng, (sw, sh), (dw, dh), channels);
    }
}
