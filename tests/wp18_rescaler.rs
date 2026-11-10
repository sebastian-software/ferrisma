//! WP18 parity test of `src/dsp/rescaler_sse2.rs` (ADR 0008, parity 1): the
//! four row functions that `WebPRescalerDspInitSSE2` assigns, run at every
//! SIMD level the host has against the C translation. The functions read and
//! write the rescaler's state, so the test drives whole rescalers through
//! `webp_rescaler_init`, `webp_rescaler_import` and `webp_rescaler_export`, as
//! `tests/wp4_rescaler.rs` does, and compares every step (rows needed,
//! imported and exported, the counters and the accumulator) and the output.
//!
//! The cases expand and shrink in each direction with one and four channels:
//! sources of 8 pixels and more in the SSE2 import, ratios near 1, 2, 1/2 and
//! up to the 1/128 limit of the shrinking import and beyond it (where SSE2
//! falls back to C), an expansion whose `x_add` reaches `1 << 15` (the other
//! fallback), output rows of every length modulo 8, chunks of 1, 3 and all
//! rows, flipped destinations, and noisy, smooth and saturated planes. Two and
//! three channels run where upstream's SSE2 import agrees with C: sources
//! narrower than 8 pixels when expanding, and every shrink
//! (`rescaler_sse2.rs`, excluded behavior 1).

#![cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]

#[path = "wp18_support/mod.rs"]
mod wp18_support;
#[path = "wp6_support/mod.rs"]
mod wp6_support;

use std::fmt::Write as _;

use webp::utils::rescaler_utils::{
    webp_rescale_needed_lines, webp_rescaler_export, webp_rescaler_import, webp_rescaler_init,
    webp_rescaler_input_done,
};
use wp6_support::Rng;
use wp18_support::{Level, Tally, assert_parity, simd_levels};

fn int(value: usize) -> i32 {
    i32::try_from(value).expect("the value fits in int")
}

/// A source plane: noisy, smooth or saturated.
fn plane(rng: &mut Rng, size: usize) -> Vec<u8> {
    match rng.below(3) {
        0 => rng.bytes(size),
        1 => {
            let base = rng.range(0, 255);
            (0..size)
                .map(|_| (base + rng.range(-4, 4)).clamp(0, 255) as u8)
                .collect()
        }
        _ => (0..size)
            .map(|_| if rng.chance(1, 2) { 0 } else { 255 })
            .collect(),
    }
}

/// The rescaler's run over one input: the steps and the output, written into
/// the log and the destination buffer.
struct Run {
    log: String,
    dst: Vec<u8>,
}

impl PartialEq for Run {
    fn eq(&self, other: &Self) -> bool {
        self.log == other.log && self.dst == other.dst
    }
}

impl Clone for Run {
    fn clone(&self) -> Self {
        Self {
            log: self.log.clone(),
            dst: self.dst.clone(),
        }
    }
}

impl std::fmt::Debug for Run {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}dst={:?}", self.log, self.dst)
    }
}

/// Drives a rescaler over `src` into `run`, as `tests/wp4_rescaler.rs` does.
fn drive(
    run: &mut Run,
    (src_w, src_h, dst_w, dst_h, channels): (usize, usize, usize, usize, usize),
    (dst_stride, src_stride, chunk): (usize, usize, usize),
    src: &[u8],
    flip: bool,
) {
    let work = vec![0x5a5a_5a5a; 2 * dst_w * channels];
    let stride = if flip {
        -int(dst_stride)
    } else {
        int(dst_stride)
    };
    let mut wrk = webp_rescaler_init(
        int(src_w),
        int(src_h),
        int(dst_w),
        int(dst_h),
        stride,
        int(channels),
        work,
    )
    .expect("the rescaler initializes");
    if flip {
        wrk.dst_pos = (dst_h - 1) * dst_stride;
    }
    let mut src_row = 0;
    let mut steps = 0;
    while !webp_rescaler_input_done(&wrk) {
        let needed = webp_rescale_needed_lines(&wrk, int(chunk));
        let num = chunk.min(src_h - src_row);
        let imported = webp_rescaler_import(
            &mut wrk,
            int(num),
            &src[src_row * src_stride..],
            int(src_stride),
        );
        let exported = webp_rescaler_export(&mut wrk, &mut run.dst);
        writeln!(
            run.log,
            "needed={needed} imported={imported} exported={exported} src_y={} dst_y={} y_accum={}",
            wrk.src_y, wrk.dst_y, wrk.y_accum
        )
        .unwrap();
        src_row += usize::try_from(imported).expect("imported rows are not negative");
        steps += 1;
        if steps > 100_000 || (imported == 0 && exported == 0) {
            break;
        }
    }
}

#[test]
fn rescaler_simd_matches_the_c_translation() {
    let levels: Vec<Level> = simd_levels();
    assert!(!levels.is_empty(), "an x86 host has SSE2");
    let mut rng = Rng(0x5750_3138_5245_5343);
    let mut tally = Tally::default();

    // (src_w, src_h, dst_w, dst_h, channels).
    let mut cases = vec![
        // Expanding in x through the SSE2 import, one and four channels.
        (8, 4, 9, 4, 1),
        (8, 4, 9, 4, 4),
        (9, 3, 64, 7, 1),
        (9, 3, 64, 7, 4),
        (13, 9, 40, 5, 1),
        (13, 9, 40, 5, 4),
        (33, 5, 100, 11, 1),
        (33, 5, 100, 11, 4),
        (8, 2, 255, 3, 1),
        (16, 16, 17, 33, 4),
        (100, 7, 101, 3, 1),
        (64, 2, 128, 2, 4),
        // x_add reaches 1 << 15: the C fallback.
        (9, 2, 32769, 2, 1),
        (9, 1, 32768, 1, 1),
        // Shrinking in x, at the ratio limit and beyond it.
        (9, 4, 3, 4, 4),
        (64, 8, 8, 3, 4),
        (100, 9, 33, 9, 4),
        (17, 13, 11, 19, 4),
        (128, 3, 1, 2, 4),
        (129, 3, 1, 2, 4),
        (300, 4, 3, 2, 4),
        (1000, 2, 5, 1, 4),
        (64, 40, 1, 3, 1),
        (300, 20, 1, 2, 1),
        // Equal widths (the shrinking import with x_add == x_sub).
        (7, 7, 7, 7, 4),
        (23, 5, 23, 9, 1),
        (24, 9, 24, 5, 4),
        // Two and three channels where SSE2 and C agree.
        (3, 4, 9, 4, 3),
        (7, 4, 20, 6, 2),
        (6, 9, 3, 20, 2),
        (17, 13, 11, 19, 3),
        (5, 3, 5, 3, 3),
        (33, 20, 5, 7, 2),
    ];
    // Output rows of every length modulo 8 with each channel count.
    for dst_w in 1..=17 {
        cases.push((dst_w + 9, 5, dst_w, 3, 1));
        cases.push((dst_w + 2, 3, dst_w, 6, 4));
        cases.push((8, 3, dst_w + 8, 4, 1));
    }

    let mut count = 0;
    for &(src_w, src_h, dst_w, dst_h, channels) in &cases {
        for (chunk, flip) in [
            (1_usize, false),
            (3, false),
            (1000, false),
            (1, true),
            (1000, true),
        ] {
            let dst_stride = dst_w * channels + rng.below(4) as usize;
            let src_stride = src_w * channels + rng.below(4) as usize;
            let src = plane(&mut rng, src_stride * (src_h - 1) + src_w * channels);
            let run = Run {
                log: String::new(),
                dst: rng.bytes(dst_stride * dst_h),
            };
            assert_parity(
                &levels,
                &run,
                || {
                    format!(
                        "rescaler {src_w}x{src_h} -> {dst_w}x{dst_h} channels {channels} chunk {chunk} flip {flip}"
                    )
                },
                |run| {
                    drive(
                        run,
                        (src_w, src_h, dst_w, dst_h, channels),
                        (dst_stride, src_stride, chunk),
                        &src,
                        flip,
                    );
                },
            );
            count += 1;
        }
    }
    tally.add(
        "webp_rescaler_import_row_*/export_row_* (driven)",
        count,
        &levels,
    );
    tally.print("wp18_rescaler");
}
