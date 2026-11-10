// libwebp v1.6.0, src/enc/filter_enc.c:
//
// Copyright 2011 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// Selecting filter level
//
// Author: somnath@google.com (Somnath Banerjee)

//! Translation of `src/enc/filter_enc.c` from libwebp v1.6.0.
//!
//! Work package: WP10 (VP8 encoder core).
//!
//! # Deviations from upstream
//!
//! Changes of representation that the ADRs allow, or behavior that the Rust
//! side cannot reproduce exactly. Each records the reason.
//!
//! 1. **Chroma filters of `DoFilter`.** The decoder's chroma filters
//!    (`VP8HFilter8i` and `VP8VFilter8i` in `src/dsp/dec.rs`) take the U and V
//!    planes as two buffers; `yuv_out2` holds U and V side by side in each row,
//!    which no split separates. The V samples are filtered in a copy of the
//!    buffer and copied back; each filter touches only its own plane's columns,
//!    so the samples are C's.
//!
//! # Checks not applicable
//!
//! These checks do not apply, or cannot be made, with the reason:
//!
//! 1. **Compile-time switches.** `WEBP_REDUCE_SIZE` is undefined; see
//!    `src/enc/frame_enc.rs`, `# Checks not applicable`, item 1.
//! 2. **Floating point.** See `src/enc/quant_enc.rs`,
//!    `# Checks not applicable`, item 1.

use crate::dec::common_dec::NUM_MB_SEGMENTS;
use crate::dsp::dec::{
    vp8_simple_h_filter16i, vp8_simple_v_filter16i, vp8h_filter8i, vp8h_filter16i, vp8v_filter8i,
    vp8v_filter16i,
};
use crate::dsp::dsp::{BPS, VP8_SSIM_KERNEL};
use crate::dsp::ssim::vp8ssim_get_clipped;
use crate::enc::vp8i_enc::{
    MAX_LF_LEVELS, U_OFF_ENC, V_OFF_ENC, VP8EncIterator, VP8Encoder, Y_OFF_ENC, YUV_SIZE_ENC,
};
use crate::webp::encode::WebPConfig;

/// Converts a non-negative C `int` to an index.
#[expect(clippy::cast_sign_loss, reason = "the value is non-negative")]
const fn idx(v: i32) -> usize {
    v as usize
}

// upstream: src/enc/filter_enc.c MAX_DELTA_SIZE
/// This table gives, for a given sharpness, the filtering strength to be used
/// (at least) in order to filter a given edge step delta. This is constructed
/// by brute force inspection: for all delta, we iterate over all possible
/// filtering strength / thresh until `needs_filter()` returns true.
const MAX_DELTA_SIZE: i32 = 64;
// upstream: src/enc/filter_enc.c kLevelsFromDelta
static LEVELS_FROM_DELTA: [[u8; MAX_DELTA_SIZE as usize]; 8] = [
    [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
        48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63,
    ],
    [
        0, 1, 2, 3, 5, 6, 7, 8, 9, 11, 12, 13, 14, 15, 17, 18, 20, 21, 23, 24, 26, 27, 29, 30, 32,
        33, 35, 36, 38, 39, 41, 42, 44, 45, 47, 48, 50, 51, 53, 54, 56, 57, 59, 60, 62, 63, 63, 63,
        63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63,
    ],
    [
        0, 1, 2, 3, 5, 6, 7, 8, 9, 11, 12, 13, 14, 16, 17, 19, 20, 22, 23, 25, 26, 28, 29, 31, 32,
        34, 35, 37, 38, 40, 41, 43, 44, 46, 47, 49, 50, 52, 53, 55, 56, 58, 59, 61, 62, 63, 63, 63,
        63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63,
    ],
    [
        0, 1, 2, 3, 5, 6, 7, 8, 9, 11, 12, 13, 15, 16, 18, 19, 21, 22, 24, 25, 27, 28, 30, 31, 33,
        34, 36, 37, 39, 40, 42, 43, 45, 46, 48, 49, 51, 52, 54, 55, 57, 58, 60, 61, 63, 63, 63, 63,
        63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63,
    ],
    [
        0, 1, 2, 3, 5, 6, 7, 8, 9, 11, 12, 14, 15, 17, 18, 20, 21, 23, 24, 26, 27, 29, 30, 32, 33,
        35, 36, 38, 39, 41, 42, 44, 45, 47, 48, 50, 51, 53, 54, 56, 57, 59, 60, 62, 63, 63, 63, 63,
        63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63,
    ],
    [
        0, 1, 2, 4, 5, 7, 8, 9, 11, 12, 13, 15, 16, 17, 19, 20, 22, 23, 25, 26, 28, 29, 31, 32, 34,
        35, 37, 38, 40, 41, 43, 44, 46, 47, 49, 50, 52, 53, 55, 56, 58, 59, 61, 62, 63, 63, 63, 63,
        63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63,
    ],
    [
        0, 1, 2, 4, 5, 7, 8, 9, 11, 12, 13, 15, 16, 18, 19, 21, 22, 24, 25, 27, 28, 30, 31, 33, 34,
        36, 37, 39, 40, 42, 43, 45, 46, 48, 49, 51, 52, 54, 55, 57, 58, 60, 61, 63, 63, 63, 63, 63,
        63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63,
    ],
    [
        0, 1, 2, 4, 5, 7, 8, 9, 11, 12, 14, 15, 17, 18, 20, 21, 23, 24, 26, 27, 29, 30, 32, 33, 35,
        36, 38, 39, 41, 42, 44, 45, 47, 48, 50, 51, 53, 54, 56, 57, 59, 60, 62, 63, 63, 63, 63, 63,
        63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63,
    ],
];

// upstream: src/enc/filter_enc.c VP8FilterStrengthFromDelta
/// Returns the approximate filtering strength needed to smooth a edge step of
/// 'delta', given a sharpness parameter 'sharpness'.
#[must_use]
pub fn vp8_filter_strength_from_delta(sharpness: i32, delta: i32) -> i32 {
    let pos = if delta < MAX_DELTA_SIZE {
        delta
    } else {
        MAX_DELTA_SIZE - 1
    };
    debug_assert!((0..=7).contains(&sharpness));
    i32::from(LEVELS_FROM_DELTA[idx(sharpness)][idx(pos)])
}

//------------------------------------------------------------------------------
// Paragraph 15.4: compute the inner-edge filtering strength

// upstream: src/enc/filter_enc.c GetILevel
fn get_i_level(sharpness: i32, level: i32) -> i32 {
    let mut level = level;
    if sharpness > 0 {
        if sharpness > 4 {
            level >>= 2;
        } else {
            level >>= 1;
        }
        if level > 9 - sharpness {
            level = 9 - sharpness;
        }
    }
    if level < 1 {
        level = 1;
    }
    level
}

// upstream: src/enc/filter_enc.c DoFilter
/// Filters a copy of `yuv_out` into `yuv_out2`. The decoder's chroma filters
/// take the U and V planes as two buffers, which `yuv_out2` holds side by side
/// in each row; the V samples are filtered in a copy of the buffer and copied
/// back (see the `# Deviations from upstream` section of this module, item 1).
#[expect(clippy::similar_names, reason = "the C names level and ilevel")]
#[expect(clippy::bool_to_int_with_if, reason = "the C condition form")]
fn do_filter(it: &mut VP8EncIterator, enc: &VP8Encoder, config: &WebPConfig, level: i32) {
    let ilevel = get_i_level(config.filter_sharpness, level);
    let limit = 2 * level + ilevel;

    let y_dst = idx(Y_OFF_ENC);
    let u_dst = idx(U_OFF_ENC);
    let v_dst = idx(V_OFF_ENC);

    // copy current block to yuv_out2
    it.yuv_out2[y_dst..y_dst + idx(YUV_SIZE_ENC)].copy_from_slice(&it.yuv_out[..idx(YUV_SIZE_ENC)]);

    if enc.filter_hdr.simple == 1 {
        // simple
        vp8_simple_h_filter16i(&mut it.yuv_out2, y_dst, BPS, limit);
        vp8_simple_v_filter16i(&mut it.yuv_out2, y_dst, BPS, limit);
    } else {
        // complex
        let hev_thresh = if level >= 40 {
            2
        } else if level >= 15 {
            1
        } else {
            0
        };
        let mut v_copy = it.yuv_out2.clone();
        vp8h_filter16i(&mut it.yuv_out2, y_dst, BPS, limit, ilevel, hev_thresh);
        vp8h_filter8i(
            &mut it.yuv_out2,
            u_dst,
            &mut v_copy,
            v_dst,
            BPS,
            limit,
            ilevel,
            hev_thresh,
        );
        vp8v_filter16i(&mut it.yuv_out2, y_dst, BPS, limit, ilevel, hev_thresh);
        vp8v_filter8i(
            &mut it.yuv_out2,
            u_dst,
            &mut v_copy,
            v_dst,
            BPS,
            limit,
            ilevel,
            hev_thresh,
        );
        for row in 0..8 {
            let start = v_dst + row * idx(BPS);
            it.yuv_out2[start..start + 8].copy_from_slice(&v_copy[start..start + 8]);
        }
    }
}

//------------------------------------------------------------------------------
// SSIM metric for one macroblock

// upstream: src/enc/filter_enc.c GetMBSSIM
fn get_mb_ssim(yuv1: &[u8], yuv2: &[u8]) -> f64 {
    let mut sum = 0.;

    // compute SSIM in a 10 x 10 window
    for y in VP8_SSIM_KERNEL..16 - VP8_SSIM_KERNEL {
        for x in VP8_SSIM_KERNEL..16 - VP8_SSIM_KERNEL {
            sum += vp8ssim_get_clipped(
                &yuv1[idx(Y_OFF_ENC)..],
                BPS,
                &yuv2[idx(Y_OFF_ENC)..],
                BPS,
                x,
                y,
                16,
                16,
            );
        }
    }
    for x in 1..7 {
        for y in 1..7 {
            sum += vp8ssim_get_clipped(
                &yuv1[idx(U_OFF_ENC)..],
                BPS,
                &yuv2[idx(U_OFF_ENC)..],
                BPS,
                x,
                y,
                8,
                8,
            );
            sum += vp8ssim_get_clipped(
                &yuv1[idx(V_OFF_ENC)..],
                BPS,
                &yuv2[idx(V_OFF_ENC)..],
                BPS,
                x,
                y,
                8,
                8,
            );
        }
    }
    sum
}

//------------------------------------------------------------------------------
// Exposed APIs: Encoder should call the following 3 functions to adjust
// loop filter strength

// upstream: src/enc/filter_enc.c VP8InitFilter
/// Clears the filter statistics (C: `VP8SSIMDspInit` follows; it only assigns
/// portable pointers and has no counterpart, ADR 0003).
///
/// # Panics
///
/// Panics when the iterator expects filter statistics that the encoder lacks.
pub fn vp8_init_filter(it: &VP8EncIterator, enc: &mut VP8Encoder) {
    if it.lf_stats {
        let lf_stats = enc
            .lf_stats
            .as_mut()
            .expect("the encoder collects filter stats");
        for s in 0..idx(NUM_MB_SEGMENTS) {
            for i in 0..idx(MAX_LF_LEVELS) {
                lf_stats[s][i] = 0.;
            }
        }
    }
}

// upstream: src/enc/filter_enc.c VP8StoreFilterStats
/// Records the SSIM of the macroblock for the filter levels around its level.
///
/// # Panics
///
/// Panics when the iterator expects filter statistics that the encoder lacks.
pub fn vp8_store_filter_stats(it: &mut VP8EncIterator, enc: &mut VP8Encoder, config: &WebPConfig) {
    let mb = enc.mb_info[it.mb];
    let s = usize::from(mb.segment);
    let level0 = enc.dqm[s].fstrength;

    // explore +/-quant range of values around level0
    let delta_min = -enc.dqm[s].quant;
    let delta_max = enc.dqm[s].quant;
    let step_size = if delta_max - delta_min >= 4 { 4 } else { 1 };

    if !it.lf_stats {
        return;
    }

    // NOTE: Currently we are applying filter only across the sublock edges
    // There are two reasons for that.
    // 1. Applying filter on macro block edges will change the pixels in
    // the left and top macro blocks. That will be hard to restore
    // 2. Macro Blocks on the bottom and right are not yet compressed. So we
    // cannot apply filter on the right and bottom macro block edges.
    if mb.r#type == 1 && mb.skip != 0 {
        return;
    }

    // Always try filter level  zero
    let ssim0 = get_mb_ssim(&it.yuv_in, &it.yuv_out);
    enc.lf_stats
        .as_mut()
        .expect("the encoder collects filter stats")[s][0] += ssim0;

    let mut d = delta_min;
    while d <= delta_max {
        let level = level0 + d;
        if level > 0 && level < MAX_LF_LEVELS {
            do_filter(it, enc, config, level);
            let ssim = get_mb_ssim(&it.yuv_in, &it.yuv_out2);
            enc.lf_stats
                .as_mut()
                .expect("the encoder collects filter stats")[s][idx(level)] += ssim;
        }
        d += step_size;
    }
}

// upstream: src/enc/filter_enc.c VP8AdjustFilterStrength
/// Picks the final filter strengths from the statistics, or from the maximal
/// edge deltas when there are none.
///
/// # Panics
///
/// Panics when the iterator expects filter statistics that the encoder lacks.
pub fn vp8_adjust_filter_strength(it: &VP8EncIterator, enc: &mut VP8Encoder, config: &WebPConfig) {
    if it.lf_stats {
        let lf_stats = enc
            .lf_stats
            .as_ref()
            .expect("the encoder collects filter stats");
        let mut best_levels = [0; NUM_MB_SEGMENTS as usize];
        for (s, best) in best_levels.iter_mut().enumerate() {
            let mut best_level = 0;
            // Improvement over filter level 0 should be at least 1e-5 (relatively)
            let mut best_v = 1.00001 * lf_stats[s][0];
            for i in 1..MAX_LF_LEVELS {
                let v = lf_stats[s][idx(i)];
                if v > best_v {
                    best_v = v;
                    best_level = i;
                }
            }
            *best = best_level;
        }
        for (s, best) in best_levels.into_iter().enumerate() {
            enc.dqm[s].fstrength = best;
        }
        return;
    }
    if config.filter_strength > 0 {
        let mut max_level = 0;
        let sharpness = enc.filter_hdr.sharpness;
        for dqm in &mut enc.dqm {
            // this '>> 3' accounts for some inverse WHT scaling
            let delta = (dqm.max_edge * i32::from(dqm.y2.q[1])) >> 3;
            let level = vp8_filter_strength_from_delta(sharpness, delta);
            if level > dqm.fstrength {
                dqm.fstrength = level;
            }
            if max_level < dqm.fstrength {
                max_level = dqm.fstrength;
            }
        }
        enc.filter_hdr.level = max_level;
    }
}
