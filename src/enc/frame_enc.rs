// libwebp v1.6.0, src/enc/frame_enc.c:
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
//   frame coding and analysis
//
// Author: Skal (pascal.massimino@gmail.com)

//! Translation of `src/enc/frame_enc.c` from libwebp v1.6.0.
//!
//! Work package: WP10 (VP8 encoder core).
//!
//! # Upstream behavior preserved
//!
//! Behavior of the C code that a reader might take for a defect, which the
//! translation keeps:
//!
//! 1. **`StatLoop` stops early for methods 0 to 2 with one segment.**
//!    `RefineUsingDistortion` never sets the header cost `rd->H`, so
//!    `OneStatPass` returns a partition-0 size of 0, which `StatLoop` treats as
//!    a failure: it returns before `FinalizeSkipProba`, `FinalizeTokenProbas`
//!    and `VP8CalculateLevelCosts`, and `VP8EncLoop` ignores the result. The
//!    skip probability stays unused and the token probabilities stay the
//!    defaults. With two segments the segment header cost is positive and the
//!    loop runs to its end. The test
//!    `large_and_edge_pictures_match_the_c_encoder` in `tests/wp10_encode.rs`
//!    covers both.
//! 2. **An abort during `StatLoop` does not stop `VP8EncLoop`**, which ignores
//!    `StatLoop`'s result and aborts at its own next progress report.
//!
//! # Excluded undefined behavior
//!
//! Inputs that are undefined behavior (or assertion failures) in C and are
//! therefore excluded from the oracle comparison (PORTING rule 8):
//!
//! 1. **Uninitialized bit counts in `extra_info` type 6.** `StoreSideInfo`
//!    reads `it->luma_bits` and `it->uv_bits`, which only `CodeResiduals`
//!    writes: the token loop never does, and `VP8EncLoop` not before the first
//!    coded macroblock. Type 6 is compared only for `VP8EncLoop` on noise
//!    pictures, whose first macroblock is coded; Rust starts from 0.
//!
//! # Checks not applicable
//!
//! These checks do not apply, or cannot be made, with the reason:
//!
//! 1. **Compile-time switches off in the reference configuration.**
//!    `WEBP_DISABLE_STATS`, `WEBP_REDUCE_SIZE`, `DISABLE_TOKEN_BUFFER`,
//!    `PRINT_MEMORY_INFO`, `DEBUG_BLOCK` and `WEBP_MSAN` are undefined, and
//!    `SEGMENT_VISU` and `DEBUG_SEARCH` are 0, so their branches are not
//!    translated. `USE_TDISTO` is 1 and `USE_LEVEL_CODE_TABLE` is defined, so
//!    only those variants are translated. `DO_TRELLIS_UV` is 0; its branch is
//!    translated and never taken.
//! 2. **Floating point.** See `src/enc/quant_enc.rs`,
//!    `# Checks not applicable`, item 1.

use crate::dec::common_dec::{NUM_BANDS, NUM_CTX, NUM_MB_SEGMENTS, NUM_PROBAS, NUM_TYPES};
use crate::dsp::cost::VP8_ENC_BANDS;
use crate::dsp::cost::vp8_set_residual_coeffs;
use crate::dsp::enc::{vp8sse8x8, vp8sse16x16};
use crate::enc::cost_enc::{
    VP8Residual, vp8_bit_cost, vp8_calculate_level_costs, vp8_init_residual, vp8_record_coeffs,
};
use crate::enc::filter_enc::{vp8_adjust_filter_strength, vp8_init_filter, vp8_store_filter_stats};
use crate::enc::iterator_enc::{
    vp8_iterator_bytes_to_nz, vp8_iterator_export, vp8_iterator_import, vp8_iterator_init,
    vp8_iterator_next, vp8_iterator_nz_to_bytes, vp8_iterator_progress, vp8_iterator_save_boundary,
};
use crate::enc::quant_enc::{vp8_decimate, vp8_set_segment_params};
use crate::enc::syntax_enc::vp8_enc_free_bit_writers;
use crate::enc::token_enc::{
    vp8_emit_tokens, vp8_estimate_token_size, vp8_record_coeff_tokens, vp8t_buffer_clear,
};
use crate::enc::tree_enc::{VP8_COEFFS_PROBA0, VP8_COEFFS_UPDATE_PROBA};
use crate::enc::vp8i_enc::{
    U_OFF_ENC, V_OFF_ENC, VP8EncIterator, VP8EncProba, VP8Encoder, VP8ModeScore, VP8RDLevel,
    Y_OFF_ENC,
};
use crate::enc::webp_enc::{webp_encoding_set_error, webp_report_progress};
use crate::utils::bit_writer_utils::{
    VP8BitWriter, vp8_bit_writer_finish, vp8_bit_writer_init, vp8_bit_writer_pos, vp8_put_bit,
    vp8_put_bit_uniform,
};
use crate::webp::encode::{WebPConfig, WebPEncodingError, WebPPicture};
use crate::webp::format_constants::{
    CHUNK_HEADER_SIZE, RIFF_HEADER_SIZE, VP8_FRAME_HEADER_SIZE, VP8_MAX_PARTITION0_SIZE,
};

/// Converts a non-negative C `int` to an index.
#[expect(clippy::cast_sign_loss, reason = "the value is non-negative")]
const fn idx(v: i32) -> usize {
    v as usize
}

// upstream: src/enc/frame_enc.c SEGMENT_VISU
// SEGMENT_VISU is 0, so the visualization of SetBlock is not compiled.
// upstream: src/enc/frame_enc.c DEBUG_SEARCH
// DEBUG_SEARCH is 0, so the convergence printouts are not compiled.

//------------------------------------------------------------------------------
// multi-pass convergence

// upstream: src/enc/frame_enc.c HEADER_SIZE_ESTIMATE
const HEADER_SIZE_ESTIMATE: u64 =
    (RIFF_HEADER_SIZE + CHUNK_HEADER_SIZE + VP8_FRAME_HEADER_SIZE) as u64;
// upstream: src/enc/frame_enc.c DQ_LIMIT
/// convergence is considered reached if `dq < DQ_LIMIT`
const DQ_LIMIT: f64 = 0.4;
// upstream: src/enc/frame_enc.c PARTITION0_SIZE_LIMIT
/// we allow 2k of extra head-room in PARTITION0 limit.
const PARTITION0_SIZE_LIMIT: u64 = (VP8_MAX_PARTITION0_SIZE as u64 - 2048) << 11;

// upstream: src/enc/frame_enc.c Clamp
fn clamp(v: f32, min: f32, max: f32) -> f32 {
    if v < min {
        min
    } else if v > max {
        max
    } else {
        v
    }
}

// upstream: src/enc/frame_enc.c PassStats
/// struct for organizing convergence in either size or PSNR
#[derive(Clone, Copy, Debug, Default)]
struct PassStats {
    is_first: bool,
    dq: f32,
    q: f32,
    last_q: f32,
    qmin: f32,
    qmax: f32,
    /// PSNR or size
    value: f64,
    last_value: f64,
    target: f64,
    do_size_search: bool,
}

// upstream: src/enc/frame_enc.c InitPassStats
#[expect(
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    reason = "C converts target_size to uint64_t and then to double, and qmin, qmax to float"
)]
fn init_pass_stats(config: &WebPConfig, s: &mut PassStats) -> bool {
    let target_size = config.target_size as u64;
    let do_size_search = target_size != 0;
    let target_psnr = config.target_psnr;

    s.is_first = true;
    s.dq = 10.;
    s.qmin = 1. * config.qmin as f32;
    s.qmax = 1. * config.qmax as f32;
    s.q = clamp(config.quality, s.qmin, s.qmax);
    s.last_q = s.q;
    s.target = if do_size_search {
        target_size as f64
    } else if f64::from(target_psnr) > 0. {
        f64::from(target_psnr)
    } else {
        40. // default, just in case
    };
    s.value = 0.;
    s.last_value = 0.;
    s.do_size_search = do_size_search;
    do_size_search
}

// upstream: src/enc/frame_enc.c ComputeNextQ
#[expect(
    clippy::cast_possible_truncation,
    clippy::float_cmp,
    reason = "C converts the double step to float and compares the values exactly"
)]
fn compute_next_q(s: &mut PassStats) -> f32 {
    let dq: f32;
    if s.is_first {
        dq = if s.value > s.target { -s.dq } else { s.dq };
        s.is_first = false;
    } else if s.value != s.last_value {
        let slope = (s.target - s.value) / (s.last_value - s.value);
        dq = (slope * f64::from(s.last_q - s.q)) as f32;
    } else {
        dq = 0.; // we're done?!
    }
    // Limit variable to avoid large swings.
    s.dq = clamp(dq, -30., 30.);
    s.last_q = s.q;
    s.last_value = s.value;
    s.q = clamp(s.q + s.dq, s.qmin, s.qmax);
    s.q
}

//------------------------------------------------------------------------------
// Tables for level coding

// upstream: src/enc/frame_enc.c VP8Cat3
pub static VP8_CAT3: [u8; 3] = [173, 148, 140];
// upstream: src/enc/frame_enc.c VP8Cat4
pub static VP8_CAT4: [u8; 4] = [176, 155, 140, 135];
// upstream: src/enc/frame_enc.c VP8Cat5
pub static VP8_CAT5: [u8; 5] = [180, 157, 141, 134, 130];
// upstream: src/enc/frame_enc.c VP8Cat6
pub static VP8_CAT6: [u8; 11] = [254, 254, 243, 230, 196, 177, 153, 140, 133, 130, 129];

//------------------------------------------------------------------------------
// Reset the statistics about: number of skips, token proba, level cost,...

// upstream: src/enc/frame_enc.c ResetStats
fn reset_stats(enc: &mut VP8Encoder) {
    let proba = &mut enc.proba;
    vp8_calculate_level_costs(proba);
    proba.nb_skip = 0;
}

//------------------------------------------------------------------------------
// Skip decision probability

// upstream: src/enc/frame_enc.c SKIP_PROBA_THRESHOLD
/// value below which using `skip_proba` is OK.
const SKIP_PROBA_THRESHOLD: i32 = 250;

// upstream: src/enc/frame_enc.c CalcSkipProba
#[expect(
    clippy::cast_possible_truncation,
    reason = "C converts the probability to int"
)]
#[expect(clippy::manual_checked_ops, reason = "the C condition form")]
fn calc_skip_proba(nb: u64, total: u64) -> i32 {
    (if total != 0 {
        (total - nb) * 255 / total
    } else {
        255
    }) as i32
}

// upstream: src/enc/frame_enc.c FinalizeSkipProba
/// Returns the bit-cost for coding the skip probability.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "C converts the counts to uint64_t and the probability to uint8_t"
)]
fn finalize_skip_proba(enc: &mut VP8Encoder) -> i32 {
    let proba = &mut enc.proba;
    let nb_mbs = enc.mb_w * enc.mb_h;
    let nb_events = proba.nb_skip;
    proba.skip_proba = calc_skip_proba(nb_events as u64, nb_mbs as u64) as u8;
    proba.use_skip_proba = i32::from(proba.skip_proba) < SKIP_PROBA_THRESHOLD;
    let mut size = 256; // 'use_skip_proba' bit
    if proba.use_skip_proba {
        size += nb_events * vp8_bit_cost(true, proba.skip_proba)
            + (nb_mbs - nb_events) * vp8_bit_cost(false, proba.skip_proba);
        size += 8 * 256; // cost of signaling the 'skip_proba' itself.
    }
    size
}

// upstream: src/enc/frame_enc.c CalcTokenProba
/// Collect statistics and deduce probabilities for next coding pass.
/// Return the total bit-cost for coding the probability updates.
fn calc_token_proba(nb: i32, total: i32) -> i32 {
    debug_assert!(nb <= total);
    if nb != 0 { 255 - nb * 255 / total } else { 255 }
}

// upstream: src/enc/frame_enc.c BranchCost
/// Cost of coding 'nb' 1's and 'total-nb' 0's using 'proba' probability.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "C passes the probability as uint8_t"
)]
fn branch_cost(nb: i32, total: i32, proba: i32) -> i32 {
    nb * vp8_bit_cost(true, proba as u8) + (total - nb) * vp8_bit_cost(false, proba as u8)
}

// upstream: src/enc/frame_enc.c ResetTokenStats
fn reset_token_stats(enc: &mut VP8Encoder) {
    let proba = &mut enc.proba;
    proba.stats =
        [[[[0; NUM_PROBAS as usize]; NUM_CTX as usize]; NUM_BANDS as usize]; NUM_TYPES as usize];
}

// upstream: src/enc/frame_enc.c FinalizeTokenProbas
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    reason = "C extracts the 16-bit counts as int and stores the probability as uint8_t"
)]
fn finalize_token_probas(proba: &mut VP8EncProba) -> i32 {
    let mut has_changed = false;
    let mut size = 0;
    for t in 0..NUM_TYPES as usize {
        for b in 0..NUM_BANDS as usize {
            for c in 0..NUM_CTX as usize {
                for p in 0..NUM_PROBAS as usize {
                    let stats = proba.stats[t][b][c][p];
                    let nb = (stats & 0xffff) as i32;
                    let total = ((stats >> 16) & 0xffff) as i32;
                    let update_proba = VP8_COEFFS_UPDATE_PROBA[t][b][c][p];
                    let old_p = i32::from(VP8_COEFFS_PROBA0[t][b][c][p]);
                    let new_p = calc_token_proba(nb, total);
                    let old_cost =
                        branch_cost(nb, total, old_p) + vp8_bit_cost(false, update_proba);
                    let new_cost =
                        branch_cost(nb, total, new_p) + vp8_bit_cost(true, update_proba) + 8 * 256;
                    let use_new_p = old_cost > new_cost;
                    size += vp8_bit_cost(use_new_p, update_proba);
                    if use_new_p {
                        // only use proba that seem meaningful enough.
                        proba.coeffs[t][b][c][p] = new_p as u8;
                        has_changed |= new_p != old_p;
                        size += 8 * 256;
                    } else {
                        proba.coeffs[t][b][c][p] = old_p as u8;
                    }
                }
            }
        }
    }
    proba.dirty = has_changed;
    size
}

//------------------------------------------------------------------------------
// Finalize Segment probability based on the coding tree

// upstream: src/enc/frame_enc.c GetProba
fn get_proba(a: i32, b: i32) -> i32 {
    let total = a + b;
    if total == 0 {
        255 // that's the default probability.
    } else {
        (255 * a + total / 2) / total // rounded proba
    }
}

// upstream: src/enc/frame_enc.c ResetSegments
fn reset_segments(enc: &mut VP8Encoder) {
    for n in 0..idx(enc.mb_w * enc.mb_h) {
        enc.mb_info[n].segment = 0;
    }
}

// upstream: src/enc/frame_enc.c SetSegmentProbas
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "the probabilities are in 0..=255"
)]
fn set_segment_probas(enc: &mut VP8Encoder, pic: &mut WebPPicture<'_>) {
    let mut p = [0_i32; NUM_MB_SEGMENTS as usize];

    for n in 0..idx(enc.mb_w * enc.mb_h) {
        let mb = &enc.mb_info[n];
        p[usize::from(mb.segment)] += 1;
    }
    if let Some(stats) = pic.stats.as_deref_mut() {
        stats.segment_size = p;
    }
    if enc.segment_hdr.num_segments > 1 {
        let probas = &mut enc.proba.segments;
        probas[0] = get_proba(p[0] + p[1], p[2] + p[3]) as u8;
        probas[1] = get_proba(p[0], p[1]) as u8;
        probas[2] = get_proba(p[2], p[3]) as u8;
        let probas = *probas;

        enc.segment_hdr.update_map = probas[0] != 255 || probas[1] != 255 || probas[2] != 255;
        if !enc.segment_hdr.update_map {
            reset_segments(enc);
        }
        enc.segment_hdr.size = p[0]
            * (vp8_bit_cost(false, probas[0]) + vp8_bit_cost(false, probas[1]))
            + p[1] * (vp8_bit_cost(false, probas[0]) + vp8_bit_cost(true, probas[1]))
            + p[2] * (vp8_bit_cost(true, probas[0]) + vp8_bit_cost(false, probas[2]))
            + p[3] * (vp8_bit_cost(true, probas[0]) + vp8_bit_cost(true, probas[2]));
    } else {
        enc.segment_hdr.update_map = false;
        enc.segment_hdr.size = 0;
    }
}

//------------------------------------------------------------------------------
// Coefficient coding

// upstream: src/enc/frame_enc.c PutCoeffs
#[expect(
    clippy::cast_sign_loss,
    reason = "positions and contexts are non-negative indexes"
)]
#[expect(
    clippy::if_not_else,
    clippy::identity_op,
    reason = "the C condition forms and constants"
)]
fn put_coeffs(bw: &mut VP8BitWriter, ctx: i32, res: &VP8Residual<'_>) -> i32 {
    let mut n = res.first;
    // should be prob[VP8EncBands[n]], but it's equivalent for n=0 or 1
    let mut p = &res.prob[n as usize][ctx as usize];
    let pb = |p: &[u8; 11], i: usize| i32::from(p[i]);
    if !vp8_put_bit(bw, res.last >= 0, pb(p, 0)) {
        return 0;
    }

    while n < 16 {
        let c = i32::from(res.coeffs[n as usize]);
        n += 1;
        let sign = c < 0;
        let mut v = if sign { -c } else { c };
        if !vp8_put_bit(bw, v != 0, pb(p, 1)) {
            p = &res.prob[usize::from(VP8_ENC_BANDS[n as usize])][0];
            continue;
        }
        if vp8_put_bit(bw, v > 1, pb(p, 2)) {
            if !vp8_put_bit(bw, v > 4, pb(p, 3)) {
                if vp8_put_bit(bw, v != 2, pb(p, 4)) {
                    vp8_put_bit(bw, v == 4, pb(p, 5));
                }
            } else if !vp8_put_bit(bw, v > 10, pb(p, 6)) {
                if !vp8_put_bit(bw, v > 6, pb(p, 7)) {
                    vp8_put_bit(bw, v == 6, 159);
                } else {
                    vp8_put_bit(bw, v >= 9, 165);
                    vp8_put_bit(bw, v & 1 == 0, 145);
                }
            } else {
                let mut mask: i32;
                let tab: &[u8];
                if v < 3 + (8 << 1) {
                    // VP8Cat3  (3b)
                    vp8_put_bit(bw, false, pb(p, 8));
                    vp8_put_bit(bw, false, pb(p, 9));
                    v -= 3 + (8 << 0);
                    mask = 1 << 2;
                    tab = &VP8_CAT3;
                } else if v < 3 + (8 << 2) {
                    // VP8Cat4  (4b)
                    vp8_put_bit(bw, false, pb(p, 8));
                    vp8_put_bit(bw, true, pb(p, 9));
                    v -= 3 + (8 << 1);
                    mask = 1 << 3;
                    tab = &VP8_CAT4;
                } else if v < 3 + (8 << 3) {
                    // VP8Cat5  (5b)
                    vp8_put_bit(bw, true, pb(p, 8));
                    vp8_put_bit(bw, false, pb(p, 10));
                    v -= 3 + (8 << 2);
                    mask = 1 << 4;
                    tab = &VP8_CAT5;
                } else {
                    // VP8Cat6 (11b)
                    vp8_put_bit(bw, true, pb(p, 8));
                    vp8_put_bit(bw, true, pb(p, 10));
                    v -= 3 + (8 << 3);
                    mask = 1 << 10;
                    tab = &VP8_CAT6;
                }
                let mut tab_pos = 0;
                while mask != 0 {
                    vp8_put_bit(bw, v & mask != 0, i32::from(tab[tab_pos]));
                    tab_pos += 1;
                    mask >>= 1;
                }
            }
            p = &res.prob[usize::from(VP8_ENC_BANDS[n as usize])][2];
        } else {
            p = &res.prob[usize::from(VP8_ENC_BANDS[n as usize])][1];
        }
        vp8_put_bit_uniform(bw, sign);
        if n == 16 || !vp8_put_bit(bw, n <= res.last, pb(p, 0)) {
            return 1; // EOB
        }
    }
    1
}

// upstream: src/enc/frame_enc.c CodeResiduals
/// Writes the levels of `rd` into the iterator's partition (C: the `bw`
/// argument, which is always `it->bw`).
fn code_residuals(it: &mut VP8EncIterator, enc: &mut VP8Encoder, rd: &VP8ModeScore) {
    let mb = enc.mb_info[it.mb];
    let i16 = mb.r#type == 1;
    let segment = usize::from(mb.segment);

    vp8_iterator_nz_to_bytes(it, enc);

    let bw = &mut enc.parts[it.bw];
    let pos1 = vp8_bit_writer_pos(bw);
    let mut res = if i16 {
        let mut res = vp8_init_residual(0, 1, &mut enc.proba);
        vp8_set_residual_coeffs(&rd.y_dc_levels, &mut res);
        let nz = put_coeffs(bw, it.top_nz[8] + it.left_nz[8], &res);
        it.top_nz[8] = nz;
        it.left_nz[8] = nz;
        vp8_init_residual(1, 0, &mut enc.proba)
    } else {
        vp8_init_residual(0, 3, &mut enc.proba)
    };

    // luma-AC
    for y in 0..4 {
        for x in 0..4 {
            let ctx = it.top_nz[x] + it.left_nz[y];
            vp8_set_residual_coeffs(&rd.y_ac_levels[x + y * 4], &mut res);
            let nz = put_coeffs(bw, ctx, &res);
            it.top_nz[x] = nz;
            it.left_nz[y] = nz;
        }
    }
    let pos2 = vp8_bit_writer_pos(bw);

    // U/V
    let mut res = vp8_init_residual(0, 2, &mut enc.proba);
    for ch in [0, 2] {
        for y in 0..2 {
            for x in 0..2 {
                let ctx = it.top_nz[4 + ch + x] + it.left_nz[4 + ch + y];
                vp8_set_residual_coeffs(&rd.uv_levels[ch * 2 + x + y * 2], &mut res);
                let nz = put_coeffs(bw, ctx, &res);
                it.top_nz[4 + ch + x] = nz;
                it.left_nz[4 + ch + y] = nz;
            }
        }
    }
    let pos3 = vp8_bit_writer_pos(bw);
    it.luma_bits = pos2 - pos1;
    it.uv_bits = pos3 - pos2;
    it.bit_count[segment][usize::from(i16)] += it.luma_bits;
    it.bit_count[segment][2] += it.uv_bits;
    vp8_iterator_bytes_to_nz(it, enc);
}

// upstream: src/enc/frame_enc.c RecordResiduals
/// Same as `CodeResiduals`, but doesn't actually write anything. Instead, it just
/// records the event distribution.
fn record_residuals(it: &mut VP8EncIterator, enc: &mut VP8Encoder, rd: &VP8ModeScore) {
    vp8_iterator_nz_to_bytes(it, enc);

    let mut res = if enc.mb_info[it.mb].r#type == 1 {
        // i16x16
        let mut res = vp8_init_residual(0, 1, &mut enc.proba);
        vp8_set_residual_coeffs(&rd.y_dc_levels, &mut res);
        let nz = vp8_record_coeffs(it.top_nz[8] + it.left_nz[8], &mut res);
        it.top_nz[8] = nz;
        it.left_nz[8] = nz;
        vp8_init_residual(1, 0, &mut enc.proba)
    } else {
        vp8_init_residual(0, 3, &mut enc.proba)
    };

    // luma-AC
    for y in 0..4 {
        for x in 0..4 {
            let ctx = it.top_nz[x] + it.left_nz[y];
            vp8_set_residual_coeffs(&rd.y_ac_levels[x + y * 4], &mut res);
            let nz = vp8_record_coeffs(ctx, &mut res);
            it.top_nz[x] = nz;
            it.left_nz[y] = nz;
        }
    }

    // U/V
    let mut res = vp8_init_residual(0, 2, &mut enc.proba);
    for ch in [0, 2] {
        for y in 0..2 {
            for x in 0..2 {
                let ctx = it.top_nz[4 + ch + x] + it.left_nz[4 + ch + y];
                vp8_set_residual_coeffs(&rd.uv_levels[ch * 2 + x + y * 2], &mut res);
                let nz = vp8_record_coeffs(ctx, &mut res);
                it.top_nz[4 + ch + x] = nz;
                it.left_nz[4 + ch + y] = nz;
            }
        }
    }

    vp8_iterator_bytes_to_nz(it, enc);
}

//------------------------------------------------------------------------------
// Token buffer

// upstream: src/enc/frame_enc.c RecordTokens
/// Records the tokens of `rd` into the encoder's token buffer (C: the `tokens`
/// argument, which is always `&enc->tokens`).
fn record_tokens(it: &mut VP8EncIterator, enc: &mut VP8Encoder, rd: &VP8ModeScore) -> bool {
    vp8_iterator_nz_to_bytes(it, enc);
    let tokens = &mut enc.tokens;
    let mut res = if enc.mb_info[it.mb].r#type == 1 {
        // i16x16
        let ctx = it.top_nz[8] + it.left_nz[8];
        let mut res = vp8_init_residual(0, 1, &mut enc.proba);
        vp8_set_residual_coeffs(&rd.y_dc_levels, &mut res);
        let nz = vp8_record_coeff_tokens(ctx, &mut res, tokens);
        it.top_nz[8] = nz;
        it.left_nz[8] = nz;
        vp8_init_residual(1, 0, &mut enc.proba)
    } else {
        vp8_init_residual(0, 3, &mut enc.proba)
    };

    // luma-AC
    for y in 0..4 {
        for x in 0..4 {
            let ctx = it.top_nz[x] + it.left_nz[y];
            vp8_set_residual_coeffs(&rd.y_ac_levels[x + y * 4], &mut res);
            let nz = vp8_record_coeff_tokens(ctx, &mut res, tokens);
            it.top_nz[x] = nz;
            it.left_nz[y] = nz;
        }
    }

    // U/V
    let mut res = vp8_init_residual(0, 2, &mut enc.proba);
    for ch in [0, 2] {
        for y in 0..2 {
            for x in 0..2 {
                let ctx = it.top_nz[4 + ch + x] + it.left_nz[4 + ch + y];
                vp8_set_residual_coeffs(&rd.uv_levels[ch * 2 + x + y * 2], &mut res);
                let nz = vp8_record_coeff_tokens(ctx, &mut res, tokens);
                it.top_nz[4 + ch + x] = nz;
                it.left_nz[4 + ch + y] = nz;
            }
        }
    }
    vp8_iterator_bytes_to_nz(it, enc);
    !enc.tokens.error
}

//------------------------------------------------------------------------------
// ExtraInfo map / Debug function

// upstream: src/enc/frame_enc.c ResetSSE
fn reset_sse(enc: &mut VP8Encoder) {
    enc.sse[0] = 0;
    enc.sse[1] = 0;
    enc.sse[2] = 0;
    // Note: enc->sse[3] is managed by alpha.c
    enc.sse_count = 0;
}

// upstream: src/enc/frame_enc.c StoreSSE
#[expect(clippy::cast_sign_loss, reason = "squared errors are non-negative")]
fn store_sse(it: &VP8EncIterator, enc: &mut VP8Encoder) {
    let input = &it.yuv_in;
    let out = &it.yuv_out;
    // Note: not totally accurate at boundary. And doesn't include in-loop filter.
    enc.sse[0] += vp8sse16x16(&input[idx(Y_OFF_ENC)..], &out[idx(Y_OFF_ENC)..]) as u64;
    enc.sse[1] += vp8sse8x8(&input[idx(U_OFF_ENC)..], &out[idx(U_OFF_ENC)..]) as u64;
    enc.sse[2] += vp8sse8x8(&input[idx(V_OFF_ENC)..], &out[idx(V_OFF_ENC)..]) as u64;
    enc.sse_count += 16 * 16;
}

// upstream: src/enc/frame_enc.c StoreSideInfo
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "C stores the low byte of the quantizer and the clamped byte count"
)]
fn store_side_info(it: &VP8EncIterator, enc: &mut VP8Encoder, pic: &mut WebPPicture<'_>) {
    let mb = enc.mb_info[it.mb];

    if pic.stats.is_some() {
        store_sse(it, enc);
        enc.block_count[0] += i32::from(mb.r#type == 0);
        enc.block_count[1] += i32::from(mb.r#type == 1);
        enc.block_count[2] += i32::from(mb.skip != 0);
    }

    let extra_info_type = pic.extra_info_type;
    if let Some(extra_info) = pic.extra_info.as_deref_mut() {
        let info = &mut extra_info[idx(it.x + it.y * enc.mb_w)];
        *info = match extra_info_type {
            1 => mb.r#type,
            2 => mb.segment,
            3 => enc.dqm[usize::from(mb.segment)].quant as u8,
            4 => {
                if mb.r#type == 1 {
                    enc.preds[it.preds]
                } else {
                    0xff
                }
            }
            5 => mb.uv_mode,
            6 => {
                let b = ((it.luma_bits + it.uv_bits + 7) >> 3) as i32;
                if b > 255 { 255 } else { b as u8 }
            }
            7 => mb.alpha,
            _ => 0,
        };
    }
}

// upstream: src/enc/frame_enc.c ResetSideInfo
fn reset_side_info(enc: &mut VP8Encoder, pic: &WebPPicture<'_>) {
    if pic.stats.is_some() {
        enc.block_count = [0; 3];
    }
    reset_sse(enc);
}

// upstream: src/enc/frame_enc.c GetPSNR
#[expect(clippy::cast_precision_loss, reason = "C converts the sums to double")]
fn get_psnr(mse: u64, size: u64) -> f64 {
    if mse > 0 && size > 0 {
        10. * (255. * 255. * size as f64 / mse as f64).log10()
    } else {
        99.
    }
}

//------------------------------------------------------------------------------
//  StatLoop(): only collect statistics (number of skips, token usage, ...).
//  This is used for deciding optimal probabilities. It also modifies the
//  quantizer value if some target (size, PSNR) was specified.

// upstream: src/enc/frame_enc.c SetLoopParams
fn set_loop_params(enc: &mut VP8Encoder, config: &WebPConfig, pic: &mut WebPPicture<'_>, q: f32) {
    // Make sure the quality parameter is inside valid bounds
    let q = clamp(q, 0., 100.);

    vp8_set_segment_params(enc, config, q); // setup segment quantizations and filters
    set_segment_probas(enc, pic); // compute segment probabilities

    reset_stats(enc);
    reset_sse(enc);
}

// upstream: src/enc/frame_enc.c OneStatPass
#[expect(
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    reason = "C adds the int64_t scores and int sizes to uint64_t sums and converts the size to double"
)]
fn one_stat_pass(
    enc: &mut VP8Encoder,
    config: &WebPConfig,
    pic: &mut WebPPicture<'_>,
    rd_opt: VP8RDLevel,
    nb_mbs: i32,
    percent_delta: i32,
    s: &mut PassStats,
) -> u64 {
    let mut it = VP8EncIterator::default();
    let mut nb_mbs = nb_mbs;
    let mut size: u64 = 0;
    let mut size_p0: u64 = 0;
    let mut distortion: u64 = 0;
    let pixel_count = nb_mbs as u64 * 384;

    vp8_iterator_init(enc, &mut it);
    set_loop_params(enc, config, pic, s.q);
    loop {
        let mut info = VP8ModeScore::default();
        vp8_iterator_import(&mut it, pic, false);
        if vp8_decimate(&mut it, enc, &mut info, rd_opt) {
            // Just record the number of skips and act like skip_proba is not used.
            enc.proba.nb_skip += 1;
        }
        record_residuals(&mut it, enc, &info);
        size = size.wrapping_add((info.r + info.h) as u64);
        size_p0 = size_p0.wrapping_add(info.h as u64);
        distortion = distortion.wrapping_add(info.d as u64);
        if percent_delta != 0
            && vp8_iterator_progress(&it, pic, &mut enc.percent, percent_delta).is_err()
        {
            return 0;
        }
        vp8_iterator_save_boundary(&mut it, enc);
        if !vp8_iterator_next(&mut it, enc) {
            break;
        }
        nb_mbs -= 1;
        if nb_mbs <= 0 {
            break;
        }
    }

    size_p0 = size_p0.wrapping_add(enc.segment_hdr.size as u64);
    if s.do_size_search {
        size = size.wrapping_add(finalize_skip_proba(enc) as u64);
        size = size.wrapping_add(finalize_token_probas(&mut enc.proba) as u64);
        size = ((size + size_p0 + 1024) >> 11) + HEADER_SIZE_ESTIMATE;
        s.value = size as f64;
    } else {
        s.value = get_psnr(distortion, pixel_count);
    }
    size_p0
}

// upstream: src/enc/frame_enc.c StatLoop
fn stat_loop(enc: &mut VP8Encoder, config: &WebPConfig, pic: &mut WebPPicture<'_>) -> bool {
    let method = enc.method;
    let do_search = enc.do_search;
    let fast_probe = (method == 0 || method == 3) && !do_search;
    let mut num_pass_left = config.pass;
    let task_percent = 20;
    let percent_per_pass = (task_percent + num_pass_left / 2) / num_pass_left;
    let final_percent = enc.percent + task_percent;
    let rd_opt = if method >= 3 || do_search {
        VP8RDLevel::RD_OPT_BASIC
    } else {
        VP8RDLevel::RD_OPT_NONE
    };
    let mut nb_mbs = enc.mb_w * enc.mb_h;
    let mut stats = PassStats::default();

    init_pass_stats(config, &mut stats);
    reset_token_stats(enc);

    // Fast mode: quick analysis pass over few mbs. Better than nothing.
    if fast_probe {
        if method == 3 {
            // we need more stats for method 3 to be reliable.
            nb_mbs = if nb_mbs > 200 { nb_mbs >> 1 } else { 100 };
        } else {
            nb_mbs = if nb_mbs > 200 { nb_mbs >> 2 } else { 50 };
        }
    }

    while num_pass_left > 0 {
        num_pass_left -= 1;
        let is_last_pass = f64::from(stats.dq).abs() <= DQ_LIMIT
            || num_pass_left == 0
            || enc.max_i4_header_bits == 0;
        let size_p0 = one_stat_pass(
            enc,
            config,
            pic,
            rd_opt,
            nb_mbs,
            percent_per_pass,
            &mut stats,
        );
        if size_p0 == 0 {
            return false;
        }
        if enc.max_i4_header_bits > 0 && size_p0 > PARTITION0_SIZE_LIMIT {
            num_pass_left += 1;
            enc.max_i4_header_bits >>= 1; // strengthen header bit limitation...
            continue; // ...and start over
        }
        if is_last_pass {
            break;
        }
        // If no target size: just do several pass without changing 'q'
        if do_search {
            compute_next_q(&mut stats);
            if f64::from(stats.dq).abs() <= DQ_LIMIT {
                break;
            }
        }
    }
    if !do_search || !stats.do_size_search {
        // Need to finalize probas now, since it wasn't done during the search.
        finalize_skip_proba(enc);
        finalize_token_probas(&mut enc.proba);
    }
    vp8_calculate_level_costs(&mut enc.proba); // finalize costs
    webp_report_progress(pic, final_percent, Some(&mut enc.percent)).is_ok()
}

//------------------------------------------------------------------------------
// Main loops
//

// upstream: src/enc/frame_enc.c kAverageBytesPerMB
static AVERAGE_BYTES_PER_MB: [u8; 8] = [50, 24, 16, 9, 7, 5, 3, 2];

// upstream: src/enc/frame_enc.c PreLoopInitialize
#[expect(
    clippy::cast_sign_loss,
    reason = "the expected partition size is non-negative"
)]
fn pre_loop_initialize(
    enc: &mut VP8Encoder,
    pic: &mut WebPPicture<'_>,
) -> Result<(), WebPEncodingError> {
    let mut ok = true;
    let average_bytes_per_mb = i32::from(AVERAGE_BYTES_PER_MB[idx(enc.base_quant >> 4)]);
    let bytes_per_parts = enc.mb_w * enc.mb_h * average_bytes_per_mb / enc.num_parts;
    // Initialize the bit-writers
    let mut p = 0;
    while ok && p < idx(enc.num_parts) {
        ok = vp8_bit_writer_init(&mut enc.parts[p], bytes_per_parts as usize);
        p += 1;
    }
    if !ok {
        vp8_enc_free_bit_writers(enc); // malloc error occurred
        return Err(webp_encoding_set_error(
            pic,
            WebPEncodingError::VP8_ENC_ERROR_OUT_OF_MEMORY,
        ));
    }
    Ok(())
}

// upstream: src/enc/frame_enc.c PostLoopFinalize
#[expect(
    clippy::cast_possible_truncation,
    reason = "C converts the byte counts to int"
)]
fn post_loop_finalize(
    it: &VP8EncIterator,
    enc: &mut VP8Encoder,
    config: &WebPConfig,
    pic: &mut WebPPicture<'_>,
    ok: bool,
) -> Result<(), WebPEncodingError> {
    let mut ok = ok;
    if ok {
        // Finalize the partitions, check for extra errors.
        for p in 0..idx(enc.num_parts) {
            vp8_bit_writer_finish(&mut enc.parts[p]);
            ok &= !enc.parts[p].error;
        }
    }

    if ok {
        // All good. Finish up.
        if pic.stats.is_some() {
            // finalize byte counters...
            for i in 0..=2 {
                for s in 0..idx(NUM_MB_SEGMENTS) {
                    enc.residual_bytes[i][s] = ((it.bit_count[s][i] + 7) >> 3) as i32;
                }
            }
        }
        vp8_adjust_filter_strength(it, enc, config); // ...and store filter stats.
    } else {
        // Something bad happened -> need to do some memory cleanup.
        vp8_enc_free_bit_writers(enc);
        return Err(webp_encoding_set_error(
            pic,
            WebPEncodingError::VP8_ENC_ERROR_OUT_OF_MEMORY,
        ));
    }
    Ok(())
}

//------------------------------------------------------------------------------
//  VP8EncLoop(): does the final bitstream coding.

// upstream: src/enc/frame_enc.c ResetAfterSkip
fn reset_after_skip(it: &mut VP8EncIterator, enc: &mut VP8Encoder) {
    if enc.mb_info[it.mb].r#type == 1 {
        enc.nz[it.nz] = 0; // reset all predictors
        it.left_nz[8] = 0;
    } else {
        enc.nz[it.nz] &= 1 << 24; // preserve the dc_nz bit
    }
}

// upstream: src/enc/frame_enc.c VP8EncLoop
/// Main coding call, without token buffer.
///
/// # Errors
///
/// Returns the code that the picture records.
pub fn vp8_enc_loop(
    enc: &mut VP8Encoder,
    config: &WebPConfig,
    pic: &mut WebPPicture<'_>,
) -> Result<(), WebPEncodingError> {
    let mut it = VP8EncIterator::default();
    pre_loop_initialize(enc, pic)?;
    let mut ok;

    stat_loop(enc, config, pic); // stats-collection loop

    vp8_iterator_init(enc, &mut it);
    vp8_init_filter(&it, enc);
    loop {
        let mut info = VP8ModeScore::default();
        let dont_use_skip = !enc.proba.use_skip_proba;
        let rd_opt = enc.rd_opt_level;

        vp8_iterator_import(&mut it, pic, false);
        // Warning! order is important: first call VP8Decimate() and
        // *then* decide how to code the skip decision if there's one.
        if !vp8_decimate(&mut it, enc, &mut info, rd_opt) || dont_use_skip {
            code_residuals(&mut it, enc, &info);
            if enc.parts[it.bw].error {
                // enc->pic->error_code is set in PostLoopFinalize().
                ok = false;
                break;
            }
        } else {
            // reset predictors after a skip
            reset_after_skip(&mut it, enc);
        }
        store_side_info(&it, enc, pic);
        vp8_store_filter_stats(&mut it, enc, config);
        vp8_iterator_export(&it, config, pic);
        ok = vp8_iterator_progress(&it, pic, &mut enc.percent, 20).is_ok();
        vp8_iterator_save_boundary(&mut it, enc);
        if !(ok && vp8_iterator_next(&mut it, enc)) {
            break;
        }
    }

    post_loop_finalize(&it, enc, config, pic, ok)
}

//------------------------------------------------------------------------------
// Single pass using Token Buffer.

// upstream: src/enc/frame_enc.c MIN_COUNT
/// minimum number of macroblocks before updating stats
const MIN_COUNT: i32 = 96;

// upstream: src/enc/frame_enc.c VP8EncTokenLoop
/// Main coding call, with the token buffer.
///
/// # Errors
///
/// Returns the code that the picture records.
#[expect(clippy::too_many_lines, reason = "the C function translated whole")]
#[expect(
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    reason = "C adds the int64_t scores and int sizes to uint64_t sums and converts the size to double"
)]
pub fn vp8_enc_token_loop(
    enc: &mut VP8Encoder,
    config: &WebPConfig,
    pic: &mut WebPPicture<'_>,
) -> Result<(), WebPEncodingError> {
    // Roughly refresh the proba eight times per pass
    let mut max_count = (enc.mb_w * enc.mb_h) >> 3;
    let mut num_pass_left = config.pass;
    let mut remaining_progress = 40; // percents
    let do_search = enc.do_search;
    let mut it = VP8EncIterator::default();
    let rd_opt = enc.rd_opt_level;
    let pixel_count = enc.mb_w as u64 * enc.mb_h as u64 * 384;
    let mut stats = PassStats::default();
    let mut ok = true;

    init_pass_stats(config, &mut stats);
    pre_loop_initialize(enc, pic)?;

    if max_count < MIN_COUNT {
        max_count = MIN_COUNT;
    }

    debug_assert_eq!(enc.num_parts, 1);
    debug_assert!(enc.use_tokens);
    debug_assert!(!enc.proba.use_skip_proba);
    debug_assert!(rd_opt >= VP8RDLevel::RD_OPT_BASIC); // otherwise, token-buffer won't be useful
    debug_assert!(num_pass_left > 0);

    while ok && num_pass_left > 0 {
        num_pass_left -= 1;
        let is_last_pass = f64::from(stats.dq).abs() <= DQ_LIMIT
            || num_pass_left == 0
            || enc.max_i4_header_bits == 0;
        let mut size_p0: u64 = 0;
        let mut distortion: u64 = 0;
        let mut cnt = max_count;
        // The final number of passes is not trivial to know in advance.
        let pass_progress = remaining_progress / (2 + num_pass_left);
        remaining_progress -= pass_progress;
        vp8_iterator_init(enc, &mut it);
        set_loop_params(enc, config, pic, stats.q);
        if is_last_pass {
            reset_token_stats(enc);
            vp8_init_filter(&it, enc); // don't collect stats until last pass (too costly)
        }
        vp8t_buffer_clear(&mut enc.tokens);
        loop {
            let mut info = VP8ModeScore::default();
            vp8_iterator_import(&mut it, pic, false);
            cnt -= 1;
            if cnt < 0 {
                finalize_token_probas(&mut enc.proba);
                vp8_calculate_level_costs(&mut enc.proba); // refresh cost tables for rd-opt
                cnt = max_count;
            }
            vp8_decimate(&mut it, enc, &mut info, rd_opt);
            ok = record_tokens(&mut it, enc, &info);
            if !ok {
                webp_encoding_set_error(pic, WebPEncodingError::VP8_ENC_ERROR_OUT_OF_MEMORY);
                break;
            }
            size_p0 = size_p0.wrapping_add(info.h as u64);
            distortion = distortion.wrapping_add(info.d as u64);
            if is_last_pass {
                store_side_info(&it, enc, pic);
                vp8_store_filter_stats(&mut it, enc, config);
                vp8_iterator_export(&it, config, pic);
                ok = vp8_iterator_progress(&it, pic, &mut enc.percent, pass_progress).is_ok();
            }
            vp8_iterator_save_boundary(&mut it, enc);
            if !(ok && vp8_iterator_next(&mut it, enc)) {
                break;
            }
        }
        if !ok {
            break;
        }

        size_p0 = size_p0.wrapping_add(enc.segment_hdr.size as u64);
        if stats.do_size_search {
            let mut size = finalize_token_probas(&mut enc.proba) as u64;
            size = size.wrapping_add(vp8_estimate_token_size(
                &enc.tokens,
                enc.proba
                    .coeffs
                    .as_flattened()
                    .as_flattened()
                    .as_flattened(),
            ) as u64);
            size = (size + size_p0 + 1024) >> 11; // -> size in bytes
            size += HEADER_SIZE_ESTIMATE;
            stats.value = size as f64;
        } else {
            // compute and store PSNR
            stats.value = get_psnr(distortion, pixel_count);
        }

        if enc.max_i4_header_bits > 0 && size_p0 > PARTITION0_SIZE_LIMIT {
            num_pass_left += 1;
            enc.max_i4_header_bits >>= 1; // strengthen header bit limitation...
            if is_last_pass {
                reset_side_info(enc, pic);
            }
            continue; // ...and start over
        }
        if is_last_pass {
            break; // done
        }
        if do_search {
            compute_next_q(&mut stats); // Adjust q
        }
    }
    if ok {
        if !stats.do_size_search {
            finalize_token_probas(&mut enc.proba);
        }
        ok = vp8_emit_tokens(
            &mut enc.tokens,
            &mut enc.parts[0],
            enc.proba
                .coeffs
                .as_flattened()
                .as_flattened()
                .as_flattened(),
            true,
        );
    }
    ok = ok
        && webp_report_progress(
            pic,
            enc.percent + remaining_progress,
            Some(&mut enc.percent),
        )
        .is_ok();
    post_loop_finalize(&it, enc, config, pic, ok)
}
