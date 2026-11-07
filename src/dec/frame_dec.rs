// libwebp v1.6.0, src/dec/frame_dec.c:
//
// Copyright 2010 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// Frame-reconstruction function. Memory allocation.
//
// Author: Skal (pascal.massimino@gmail.com)

//! Translation of `src/dec/frame_dec.c` from libwebp v1.6.0.
//!
//! Work package: WP6 (VP8 decoder core).
//!
//! # Threads
//!
//! C's `FinishRow` runs on the worker when `mt_method > 0` and calls `put()`
//! there. The worker of WP1 receives only owned data (`Send + 'static`), and
//! `put()`, the emit functions and the alpha decoding work on memory that the
//! caller lends for one call, so the work is split (ADR 0003, "Threads",
//! amendment of 2026-11-07):
//!
//! - The worker job is [`VP8RowJob`]: the thread context, the cache rows with
//!   their extra rows, `yuv_b`, `yuv_t` and the dithering generator, with copies
//!   of the decoder's scalars. It runs `ReconstructRow` for `mt_method == 2`, the
//!   deblocking, the dithering and the rotation of the top samples. It lives in
//!   the worker, which is its home also without a thread, and the calling thread
//!   reaches it between `Sync` and `Launch`.
//! - The rest of `FinishRow`, the alpha-row decoding, the crop and `put()`,
//!   runs on the calling thread at the next `Sync` (in `VP8ProcessRow`,
//!   `ParseFrame` and `VP8ExitCritical`) for the row that the job finished. A
//!   failed `put()` is recorded in a sticky flag of the decoder, which the later
//!   `Sync`s report as C's `had_error` does.
//! - In the threaded path the rotation of the top samples runs before that
//!   `put()`; their regions are disjoint (`docs/status/wp6.md`). The
//!   single-threaded path keeps C's order.

use crate::dec::alpha_dec::vp8_decompress_alpha_rows;
use crate::dec::common_dec::{
    B_DC_PRED, B_DC_PRED_NOLEFT, B_DC_PRED_NOTOP, B_DC_PRED_NOTOPLEFT, NUM_MB_SEGMENTS,
};
use crate::dec::vp8_dec::{VP8Io, VP8IoHooks, VP8IoRows, vp8_init_scanline, vp8_set_error};
use crate::dec::vp8i_dec::{
    MIN_WIDTH_FOR_THREADS, U_OFF, V_OFF, VP8AlphaPlane, VP8Decoder, VP8FInfo, VP8MB, VP8MBData,
    VP8RowJob, VP8TopSamples, Y_OFF, YUV_SIZE,
};
use crate::dsp::dec::{
    vp8_dither_combine8x8, vp8_pred_chroma8, vp8_pred_luma4, vp8_pred_luma16,
    vp8_simple_h_filter16, vp8_simple_h_filter16i, vp8_simple_v_filter16, vp8_simple_v_filter16i,
    vp8_transform, vp8_transform_ac3, vp8_transform_dc, vp8_transform_dcuv, vp8_transform_uv,
    vp8h_filter8, vp8h_filter8i, vp8h_filter16, vp8h_filter16i, vp8v_filter8, vp8v_filter8i,
    vp8v_filter16, vp8v_filter16i,
};
use crate::dsp::dsp::{BPS, VP8_DITHER_AMP_BITS};
use crate::utils::random_utils::{VP8_RANDOM_DITHER_FIX, vp8_init_random, vp8_random_bits2};
use crate::utils::thread_utils::{WebPWorker, WebPWorkerStatus};
use crate::utils::utils::{WEBP_ALIGN_CST, check_size_overflow, webp_safe_malloc};
use crate::webp::decode::{VP8StatusCode, WebPDecoderOptions};

/// `BPS` as an index step.
const BPS_U: usize = BPS as usize;

/// A C `int` count, size or position that is not negative.
fn index(value: i32) -> usize {
    usize::try_from(value).expect("a count or position is not negative")
}

/// The row job, which is home in the worker while the worker is idle.
///
/// # Panics
///
/// Panics while the worker works, or when `VP8New` did not set the job up.
pub(crate) fn row_job(worker: &mut WebPWorker<VP8RowJob, ()>) -> &mut VP8RowJob {
    worker
        .job_data_mut()
        .expect("the row job is home while the worker is idle")
        .0
}

//------------------------------------------------------------------------------
// Main reconstruction function.

/// `BPS` as the element type of `kScan`.
const BPS16: u16 = 32;
const _: () = assert!(BPS16 as i32 == BPS);

// upstream: src/dec/frame_dec.c kScan
#[rustfmt::skip]
const SCAN: [u16; 16] = [
    0, 4, 8, 12,
    4 * BPS16, 4 + 4 * BPS16, 8 + 4 * BPS16, 12 + 4 * BPS16,
    8 * BPS16, 4 + 8 * BPS16, 8 + 8 * BPS16, 12 + 8 * BPS16,
    12 * BPS16, 4 + 12 * BPS16, 8 + 12 * BPS16, 12 + 12 * BPS16,
];

// upstream: src/dec/frame_dec.c CheckMode
fn check_mode(mb_x: i32, mb_y: i32, mode: i32) -> i32 {
    if mode == B_DC_PRED {
        if mb_x == 0 {
            return if mb_y == 0 {
                B_DC_PRED_NOTOPLEFT
            } else {
                B_DC_PRED_NOLEFT
            };
        }
        return if mb_y == 0 {
            B_DC_PRED_NOTOP
        } else {
            B_DC_PRED
        };
    }
    mode
}

// upstream: src/dec/frame_dec.c Copy32b
/// Copies 4 bytes from `src` to `dst` in `buf`.
fn copy32b(buf: &mut [u8], dst: usize, src: usize) {
    buf.copy_within(src..src + 4, dst);
}

// upstream: src/dec/frame_dec.c DoTransform
fn do_transform(bits: u32, src: &[i16], dst: &mut [u8]) {
    match bits >> 30 {
        3 => vp8_transform(src, dst, false),
        2 => vp8_transform_ac3(src, dst),
        1 => vp8_transform_dc(src, dst),
        _ => {}
    }
}

// upstream: src/dec/frame_dec.c DoUVTransform
fn do_uv_transform(bits: u32, src: &[i16], dst: &mut [u8]) {
    if (bits & 0xff) != 0 {
        // any non-zero coeff at all?
        if (bits & 0xaa) != 0 {
            // any non-zero AC coefficient?
            vp8_transform_uv(src, dst); // note we don't use the AC3 variant for U/V
        } else {
            vp8_transform_dcuv(src, dst);
        }
    }
}

/// The position `delta` bytes from `pos` in `yuv_b`, as C's `dst[delta]`.
fn yuv_at(pos: usize, delta: isize) -> usize {
    pos.checked_add_signed(delta)
        .expect("a position inside yuv_b")
}

// upstream: src/dec/frame_dec.c ReconstructRow
/// Reconstructs the macroblock row of the thread context into its cache row.
/// C reads `dec` and `ctx`; the job holds both parts.
#[expect(clippy::too_many_lines, reason = "the C function")]
fn reconstruct_row(job: &mut VP8RowJob) {
    let VP8RowJob {
        thread_ctx: ctx,
        yuv_t,
        yuv_b,
        cache_y,
        cache_u,
        cache_v,
        cache_y_pos,
        cache_uv_pos,
        mb_w,
        mb_h,
        cache_y_stride,
        cache_uv_stride,
        ..
    } = job;
    let mb_y = ctx.mb_y;
    let cache_id = ctx.id;
    let y_dst = index(Y_OFF);
    let u_dst = index(U_OFF);
    let v_dst = index(V_OFF);
    let bps = BPS as isize;

    // Initialize left-most block.
    for j in 0..16 {
        yuv_b[y_dst + j * BPS_U - 1] = 129;
    }
    for j in 0..8 {
        yuv_b[u_dst + j * BPS_U - 1] = 129;
        yuv_b[v_dst + j * BPS_U - 1] = 129;
    }

    // Init top-left sample on left column too.
    if mb_y > 0 {
        yuv_b[y_dst - 1 - BPS_U] = 129;
        yuv_b[u_dst - 1 - BPS_U] = 129;
        yuv_b[v_dst - 1 - BPS_U] = 129;
    } else {
        // we only need to do this init once at block (0,0).
        // Afterward, it remains valid for the whole topmost row.
        let (y_top, u_top, v_top) = (y_dst - BPS_U - 1, u_dst - BPS_U - 1, v_dst - BPS_U - 1);
        yuv_b[y_top..][..21].fill(127); // 16 + 4 + 1
        yuv_b[u_top..][..9].fill(127); // 8 + 1
        yuv_b[v_top..][..9].fill(127); // 8 + 1
    }

    // Reconstruct one row.
    for mb_x in 0..*mb_w {
        let x = index(mb_x);
        let block = &ctx.mb_data[x];

        // Rotate in the left samples from previously decoded block. We move four
        // pixels at a time for alignment reason, and because of in-loop filter.
        if mb_x > 0 {
            for j in -1..16_isize {
                copy32b(
                    yuv_b,
                    yuv_at(y_dst, j * bps - 4),
                    yuv_at(y_dst, j * bps + 12),
                );
            }
            for j in -1..8_isize {
                copy32b(
                    yuv_b,
                    yuv_at(u_dst, j * bps - 4),
                    yuv_at(u_dst, j * bps + 4),
                );
                copy32b(
                    yuv_b,
                    yuv_at(v_dst, j * bps - 4),
                    yuv_at(v_dst, j * bps + 4),
                );
            }
        }
        {
            // bring top samples into the cache
            let coeffs = &block.coeffs;
            let mut bits = block.non_zero_y;

            if mb_y > 0 {
                let top = &yuv_t[x];
                yuv_b[y_dst - BPS_U..y_dst - BPS_U + 16].copy_from_slice(&top.y);
                yuv_b[u_dst - BPS_U..u_dst - BPS_U + 8].copy_from_slice(&top.u);
                yuv_b[v_dst - BPS_U..v_dst - BPS_U + 8].copy_from_slice(&top.v);
            }

            // predict and add residuals
            if block.is_i4x4 != 0 {
                // 4x4
                let top_right = y_dst - BPS_U + 16;

                if mb_y > 0 {
                    if mb_x >= *mb_w - 1 {
                        // on rightmost border
                        let value = yuv_t[x].y[15];
                        yuv_b[top_right..top_right + 4].fill(value);
                    } else {
                        yuv_b[top_right..top_right + 4].copy_from_slice(&yuv_t[x + 1].y[..4]);
                    }
                }
                // replicate the top-right pixels below (C indexes the
                // uint32_t pointer, so each step is 4 * BPS bytes)
                for k in 1..=3 {
                    copy32b(yuv_b, top_right + k * 4 * BPS_U, top_right);
                }

                // predict and add residuals for all 4x4 blocks in turn.
                for n in 0..16 {
                    let dst = y_dst + usize::from(SCAN[n]);
                    vp8_pred_luma4(i32::from(block.imodes[n]), yuv_b, dst);
                    do_transform(bits, &coeffs[n * 16..], &mut yuv_b[dst..]);
                    bits <<= 2;
                }
            } else {
                // 16x16
                let pred_func = check_mode(mb_x, mb_y, i32::from(block.imodes[0]));
                vp8_pred_luma16(pred_func, yuv_b, y_dst);
                if bits != 0 {
                    for n in 0..16 {
                        do_transform(
                            bits,
                            &coeffs[n * 16..],
                            &mut yuv_b[y_dst + usize::from(SCAN[n])..],
                        );
                        bits <<= 2;
                    }
                }
            }
            {
                // Chroma
                let bits_uv = block.non_zero_uv;
                let pred_func = check_mode(mb_x, mb_y, i32::from(block.uvmode));
                vp8_pred_chroma8(pred_func, yuv_b, u_dst);
                vp8_pred_chroma8(pred_func, yuv_b, v_dst);
                do_uv_transform(bits_uv, &coeffs[16 * 16..], &mut yuv_b[u_dst..]);
                do_uv_transform(bits_uv >> 8, &coeffs[20 * 16..], &mut yuv_b[v_dst..]);
            }

            // stash away top samples for next block
            if mb_y < *mb_h - 1 {
                let top = &mut yuv_t[x];
                top.y
                    .copy_from_slice(&yuv_b[y_dst + 15 * BPS_U..y_dst + 15 * BPS_U + 16]);
                top.u
                    .copy_from_slice(&yuv_b[u_dst + 7 * BPS_U..u_dst + 7 * BPS_U + 8]);
                top.v
                    .copy_from_slice(&yuv_b[v_dst + 7 * BPS_U..v_dst + 7 * BPS_U + 8]);
            }
        }
        // Transfer reconstructed samples from yuv_b cache to final destination.
        {
            let y_stride = index(*cache_y_stride);
            let uv_stride = index(*cache_uv_stride);
            let y_offset = index(cache_id) * 16 * y_stride;
            let uv_offset = index(cache_id) * 8 * uv_stride;
            let y_out = *cache_y_pos + x * 16 + y_offset;
            let u_out = *cache_uv_pos + x * 8 + uv_offset;
            let v_out = *cache_uv_pos + x * 8 + uv_offset;
            for j in 0..16 {
                let row = y_out + j * y_stride;
                cache_y[row..row + 16]
                    .copy_from_slice(&yuv_b[y_dst + j * BPS_U..y_dst + j * BPS_U + 16]);
            }
            for j in 0..8 {
                let u_row = u_out + j * uv_stride;
                let v_row = v_out + j * uv_stride;
                cache_u[u_row..u_row + 8]
                    .copy_from_slice(&yuv_b[u_dst + j * BPS_U..u_dst + j * BPS_U + 8]);
                cache_v[v_row..v_row + 8]
                    .copy_from_slice(&yuv_b[v_dst + j * BPS_U..v_dst + j * BPS_U + 8]);
            }
        }
    }
}

//------------------------------------------------------------------------------
// Filtering

// upstream: src/dec/frame_dec.c kFilterExtraRows
/// How many extra lines are needed on the MB boundary for caching, given a
/// filtering level.
/// Simple filter:  up to 2 luma samples are read and 1 is written.
/// Complex filter: up to 4 luma samples are read and 3 are written. Same for
///                 U/V, so it's 8 samples total (because of the 2x upsampling).
const FILTER_EXTRA_ROWS: [u8; 3] = [0, 2, 8];

/// `kFilterExtraRows[filter_type]`.
fn filter_extra_rows(filter_type: i32) -> i32 {
    i32::from(FILTER_EXTRA_ROWS[index(filter_type)])
}

// upstream: src/dec/frame_dec.c DoFilter
fn do_filter(job: &mut VP8RowJob, mb_x: i32, mb_y: i32) {
    let ctx = &job.thread_ctx;
    let cache_id = index(ctx.id);
    let y_bps = job.cache_y_stride;
    let f_info = ctx.f_info[index(mb_x)];
    let y_dst = job.cache_y_pos + cache_id * 16 * index(y_bps) + index(mb_x) * 16;
    let ilevel = i32::from(f_info.f_ilevel);
    let limit = i32::from(f_info.f_limit);
    if limit == 0 {
        return;
    }
    debug_assert!(limit >= 3);
    if job.filter_type == 1 {
        // simple
        let cache_y = &mut job.cache_y;
        if mb_x > 0 {
            vp8_simple_h_filter16(cache_y, y_dst, y_bps, limit + 4);
        }
        if f_info.f_inner != 0 {
            vp8_simple_h_filter16i(cache_y, y_dst, y_bps, limit);
        }
        if mb_y > 0 {
            vp8_simple_v_filter16(cache_y, y_dst, y_bps, limit + 4);
        }
        if f_info.f_inner != 0 {
            vp8_simple_v_filter16i(cache_y, y_dst, y_bps, limit);
        }
    } else {
        // complex
        let uv_bps = job.cache_uv_stride;
        let u_dst = job.cache_uv_pos + cache_id * 8 * index(uv_bps) + index(mb_x) * 8;
        let v_dst = u_dst;
        let hev_thresh = i32::from(f_info.hev_thresh);
        let VP8RowJob {
            cache_y,
            cache_u,
            cache_v,
            ..
        } = job;
        if mb_x > 0 {
            vp8h_filter16(cache_y, y_dst, y_bps, limit + 4, ilevel, hev_thresh);
            vp8h_filter8(
                cache_u,
                u_dst,
                cache_v,
                v_dst,
                uv_bps,
                limit + 4,
                ilevel,
                hev_thresh,
            );
        }
        if f_info.f_inner != 0 {
            vp8h_filter16i(cache_y, y_dst, y_bps, limit, ilevel, hev_thresh);
            vp8h_filter8i(
                cache_u, u_dst, cache_v, v_dst, uv_bps, limit, ilevel, hev_thresh,
            );
        }
        if mb_y > 0 {
            vp8v_filter16(cache_y, y_dst, y_bps, limit + 4, ilevel, hev_thresh);
            vp8v_filter8(
                cache_u,
                u_dst,
                cache_v,
                v_dst,
                uv_bps,
                limit + 4,
                ilevel,
                hev_thresh,
            );
        }
        if f_info.f_inner != 0 {
            vp8v_filter16i(cache_y, y_dst, y_bps, limit, ilevel, hev_thresh);
            vp8v_filter8i(
                cache_u, u_dst, cache_v, v_dst, uv_bps, limit, ilevel, hev_thresh,
            );
        }
    }
}

// upstream: src/dec/frame_dec.c FilterRow
/// Filter the decoded macroblock row (if needed).
fn filter_row(job: &mut VP8RowJob) {
    let mb_y = job.thread_ctx.mb_y;
    debug_assert!(job.thread_ctx.filter_row);
    for mb_x in job.tl_mb_x..job.br_mb_x {
        do_filter(job, mb_x, mb_y);
    }
}

//------------------------------------------------------------------------------
// Precompute the filtering strength for each segment and each i4x4/i16x16 mode.

// upstream: src/dec/frame_dec.c PrecomputeFilterStrengths
#[expect(clippy::similar_names, reason = "the C names level and ilevel")]
fn precompute_filter_strengths(dec: &mut VP8Decoder) {
    if dec.filter_type > 0 {
        let hdr = dec.filter_hdr;
        for s in 0..NUM_MB_SEGMENTS as usize {
            // First, compute the initial level
            let base_level = if dec.segment_hdr.use_segment {
                let mut base_level = i32::from(dec.segment_hdr.filter_strength[s]);
                if !dec.segment_hdr.absolute_delta {
                    base_level += hdr.level;
                }
                base_level
            } else {
                hdr.level
            };
            for i4x4 in 0..=1 {
                let info = &mut dec.fstrengths[s][i4x4];
                let mut level = base_level;
                if hdr.use_lf_delta {
                    level += hdr.ref_lf_delta[0];
                    if i4x4 != 0 {
                        level += hdr.mode_lf_delta[0];
                    }
                }
                level = level.clamp(0, 63);
                if level > 0 {
                    let mut ilevel = level;
                    if hdr.sharpness > 0 {
                        if hdr.sharpness > 4 {
                            ilevel >>= 2;
                        } else {
                            ilevel >>= 1;
                        }
                        if ilevel > 9 - hdr.sharpness {
                            ilevel = 9 - hdr.sharpness;
                        }
                    }
                    if ilevel < 1 {
                        ilevel = 1;
                    }
                    info.f_ilevel = byte(ilevel);
                    info.f_limit = byte(2 * level + ilevel);
                    info.hev_thresh = if level >= 40 {
                        2
                    } else {
                        u8::from(level >= 15)
                    };
                } else {
                    info.f_limit = 0; // no filtering
                }
                info.f_inner = u8::from(i4x4 != 0);
            }
        }
    }
}

/// The `uint8_t` that C stores from an `int` in its range.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "C stores the int in a uint8_t"
)]
fn byte(v: i32) -> u8 {
    v as u8
}

//------------------------------------------------------------------------------
// Dithering

// upstream: src/dec/frame_dec.c MIN_DITHER_AMP
/// Minimal amp that will provide a non-zero dithering effect.
const MIN_DITHER_AMP: u8 = 4;

// upstream: src/dec/frame_dec.c DITHER_AMP_TAB_SIZE
const DITHER_AMP_TAB_SIZE: i32 = 12;
// upstream: src/dec/frame_dec.c kQuantToDitherAmp
/// Roughly, it's `dqm->uv_mat[1]`.
const QUANT_TO_DITHER_AMP: [u8; DITHER_AMP_TAB_SIZE as usize] =
    [8, 7, 6, 4, 4, 2, 2, 2, 1, 1, 1, 1];

// upstream: src/dec/frame_dec.c VP8InitDithering
/// Initialize dithering post-process if needed. `None` is C's `NULL` options.
///
/// # Panics
///
/// Panics while the worker works, because the dithering generator is in the
/// row job.
pub fn vp8_init_dithering(options: Option<&WebPDecoderOptions>, dec: &mut VP8Decoder) {
    if let Some(options) = options {
        let d = options.dithering_strength;
        let max_amp = (1 << VP8_RANDOM_DITHER_FIX) - 1;
        let f = if d < 0 {
            0
        } else if d > 100 {
            max_amp
        } else {
            d * max_amp / 100
        };
        if f > 0 {
            let mut all_amp = 0;
            for dqm in &mut dec.dqm {
                if dqm.uv_quant < DITHER_AMP_TAB_SIZE {
                    let idx = if dqm.uv_quant < 0 { 0 } else { dqm.uv_quant };
                    dqm.dither = (f * i32::from(QUANT_TO_DITHER_AMP[index(idx)])) >> 3;
                }
                all_amp |= dqm.dither;
            }
            if all_amp != 0 {
                vp8_init_random(&mut row_job(&mut dec.worker).dithering_rg, 1.0);
                dec.dither = true;
            }
        }
        // potentially allow alpha dithering
        dec.alpha_dithering = options.alpha_dithering_strength.clamp(0, 100);
    }
}

// upstream: src/dec/frame_dec.c Dither8x8
/// Convert to range: [-2,2] for dither=50, [-4,4] for dither=100.
fn dither8x8(rg: &mut crate::utils::random_utils::VP8Random, dst: &mut [u8], bps: i32, amp: i32) {
    let mut dither = [0u8; 64];
    for value in &mut dither {
        // C stores the int in a uint8_t.
        *value = byte(vp8_random_bits2(rg, VP8_DITHER_AMP_BITS + 1, amp) & 0xff);
    }
    vp8_dither_combine8x8(&dither, dst, bps);
}

// upstream: src/dec/frame_dec.c DitherRow
fn dither_row(job: &mut VP8RowJob) {
    debug_assert!(job.dither);
    let VP8RowJob {
        thread_ctx: ctx,
        cache_u,
        cache_v,
        cache_uv_pos,
        cache_uv_stride,
        dithering_rg,
        tl_mb_x,
        br_mb_x,
        ..
    } = job;
    for mb_x in *tl_mb_x..*br_mb_x {
        let data = &ctx.mb_data[index(mb_x)];
        let cache_id = index(ctx.id);
        let uv_bps = *cache_uv_stride;
        if data.dither >= MIN_DITHER_AMP {
            let dst = *cache_uv_pos + cache_id * 8 * index(uv_bps) + index(mb_x) * 8;
            let amp = i32::from(data.dither);
            dither8x8(dithering_rg, &mut cache_u[dst..], uv_bps, amp);
            dither8x8(dithering_rg, &mut cache_v[dst..], uv_bps, amp);
        }
    }
}

//------------------------------------------------------------------------------
// This function is called after a row of macroblocks is finished decoding.
// It also takes into account the following restrictions:
//  * In case of in-loop filtering, we must hold off sending some of the bottom
//    pixels as they are yet unfiltered. They will be when the next macroblock
//    row is decoded. Meanwhile, we must preserve them by rotating them in the
//    cache area. This doesn't hold for the very bottom row of the uncropped
//    picture of course.
//  * we must clip the remaining pixels against the cropping area. The VP8Io
//    struct must have the following fields set correctly before calling put():

// upstream: src/dec/frame_dec.c MACROBLOCK_VPOS
/// Vertical position of a MB.
const fn macroblock_vpos(mb_y: i32) -> i32 {
    mb_y * 16
}

/// The positions in the cache vectors that `FinishRow` computes for the
/// row of the thread context: C's `ydst`, `udst`, `vdst` (the extra rows above
/// the row) and the start of the row itself.
struct RowPositions {
    extra_y_rows: i32,
    ysize: usize,
    uvsize: usize,
    ydst: usize,
    udst: usize,
    y_row: usize,
    uv_row: usize,
}

fn row_positions(job: &VP8RowJob) -> RowPositions {
    let cache_id = index(job.thread_ctx.id);
    let extra_y_rows = filter_extra_rows(job.filter_type);
    let ysize = index(extra_y_rows * job.cache_y_stride);
    let uvsize = index((extra_y_rows / 2) * job.cache_uv_stride);
    let y_offset = cache_id * 16 * index(job.cache_y_stride);
    let uv_offset = cache_id * 8 * index(job.cache_uv_stride);
    RowPositions {
        extra_y_rows,
        ysize,
        uvsize,
        ydst: job.cache_y_pos - ysize + y_offset,
        udst: job.cache_uv_pos - uvsize + uv_offset,
        y_row: job.cache_y_pos + y_offset,
        uv_row: job.cache_uv_pos + uv_offset,
    }
}

// upstream: src/dec/frame_dec.c FinishRow
/// The part of `FinishRow` before the `put()` branch: the reconstruction for
/// `mt_method == 2`, the deblocking and the dithering.
fn finish_row_before_put(job: &mut VP8RowJob) {
    if job.mt_method == 2 {
        reconstruct_row(job);
    }

    if job.thread_ctx.filter_row {
        filter_row(job);
    }

    if job.dither {
        dither_row(job);
    }
}

// upstream: src/dec/frame_dec.c FinishRow
/// The last part of `FinishRow`: rotate top samples if needed.
fn finish_row_rotate(job: &mut VP8RowJob) {
    let pos = row_positions(job);
    let is_last_row = job.thread_ctx.mb_y >= job.br_mb_y - 1;
    if job.thread_ctx.id + 1 == job.num_caches && !is_last_row {
        let y_from = pos.ydst + 16 * index(job.cache_y_stride);
        let uv_from = pos.udst + 8 * index(job.cache_uv_stride);
        let y_to = job.cache_y_pos - pos.ysize;
        let uv_to = job.cache_uv_pos - pos.uvsize;
        job.cache_y.copy_within(y_from..y_from + pos.ysize, y_to);
        job.cache_u
            .copy_within(uv_from..uv_from + pos.uvsize, uv_to);
        job.cache_v
            .copy_within(uv_from..uv_from + pos.uvsize, uv_to);
    }
}

// upstream: src/dec/frame_dec.c FinishRow
/// The worker hook: `FinishRow` without its `put()` branch, which runs on the
/// calling thread at the next `Sync` (see the module documentation). It does not
/// fail; C's `FinishRow` fails only in the `put()` branch.
fn finish_row_job(job: &mut VP8RowJob, _unused: &mut ()) -> bool {
    finish_row_before_put(job);
    finish_row_rotate(job);
    true
}

/// The hook that `VP8New` sets on the worker (C: `worker->hook = FinishRow`).
pub(crate) const FINISH_ROW_HOOK: fn(&mut VP8RowJob, &mut ()) -> bool = finish_row_job;

/// The alpha plane that `dec.alpha_plane` designates.
fn alpha_plane<'d>(alpha_plane: VP8AlphaPlane, mem: &'d [u8], frame: &'d [u8]) -> &'d [u8] {
    match alpha_plane {
        VP8AlphaPlane::Memory => mem,
        VP8AlphaPlane::Frame => frame,
        VP8AlphaPlane::None => panic!("the decoder has no alpha plane"),
    }
}

// upstream: src/dec/frame_dec.c FinishRow
/// The `put()` branch of `FinishRow` for the row of the thread context: the
/// alpha rows, the crop and `put()`. Returns false in case of user-abort or
/// alpha error. `io` is the decoder's io (single-threaded) or the thread
/// context's copy (threaded), as in C.
fn finish_row_put(
    dec: &mut VP8Decoder,
    io: &mut VP8Io,
    alpha_data: Option<&[u8]>,
    hooks: Option<&mut (dyn VP8IoHooks + '_)>,
) -> bool {
    let Some(hooks) = hooks.filter(|hooks| hooks.has_put()) else {
        return true;
    };
    let (pos, mb_y, br_mb_y, cache_y_stride, cache_uv_stride) = {
        let job = row_job(&mut dec.worker);
        (
            row_positions(job),
            job.thread_ctx.mb_y,
            job.br_mb_y,
            job.cache_y_stride,
            job.cache_uv_stride,
        )
    };
    let is_first_row = mb_y == 0;
    let is_last_row = mb_y >= br_mb_y - 1;
    let mut y_start = macroblock_vpos(mb_y);
    let mut y_end = macroblock_vpos(mb_y + 1);
    let (mut y, mut u) = if is_first_row {
        (pos.y_row, pos.uv_row)
    } else {
        y_start -= pos.extra_y_rows;
        (pos.ydst, pos.udst)
    };

    if !is_last_row {
        y_end -= pos.extra_y_rows;
    }
    if y_end > io.crop_bottom {
        y_end = io.crop_bottom; // make sure we don't overflow on last row.
    }
    // If dec->alpha_data is not NULL, we have some alpha plane present.
    let mut a = None;
    if let Some(alpha_data) = alpha_data
        && y_start < y_end
    {
        a = vp8_decompress_alpha_rows(dec, io, alpha_data, y_start, y_end - y_start);
        if a.is_none() {
            let _ = vp8_set_error(
                dec,
                VP8StatusCode::VP8_STATUS_BITSTREAM_ERROR,
                "Could not decode alpha data.",
            );
            return false;
        }
    }
    if y_start < io.crop_top {
        let delta_y = io.crop_top - y_start;
        y_start = io.crop_top;
        debug_assert_eq!(delta_y & 1, 0);
        y += index(cache_y_stride * delta_y);
        u += index(cache_uv_stride * (delta_y >> 1));
        if let Some(a) = &mut a {
            *a += index(io.width * delta_y);
        }
    }
    let mut ok = true;
    if y_start < y_end {
        y += index(io.crop_left);
        u += index(io.crop_left >> 1);
        if let Some(a) = &mut a {
            *a += index(io.crop_left);
        }
        io.mb_y = y_start - io.crop_top;
        io.mb_w = io.crop_right - io.crop_left;
        io.mb_h = y_end - y_start;
        let VP8Decoder {
            worker,
            alpha_plane: plane,
            alpha_plane_mem,
            frame_alpha,
            ..
        } = dec;
        let job = row_job(worker);
        let mut rows = VP8IoRows {
            y: &mut job.cache_y[y..],
            u: &job.cache_u[u..],
            v: &job.cache_v[u..],
            a: a.map(|_| alpha_plane(*plane, alpha_plane_mem, frame_alpha)),
            a_offset: a.unwrap_or(0),
        };
        ok = hooks.put(io, &mut rows);
    }
    ok
}

// upstream: src/dec/frame_dec.c FinishRow
/// Finalize and transmit a complete row, on the calling thread
/// (`mt_method == 0`). Return false in case of user-abort.
fn finish_row(
    dec: &mut VP8Decoder,
    io: &mut VP8Io,
    alpha_data: Option<&[u8]>,
    hooks: Option<&mut (dyn VP8IoHooks + '_)>,
) -> bool {
    finish_row_before_put(row_job(&mut dec.worker));
    let ok = finish_row_put(dec, io, alpha_data, hooks);
    finish_row_rotate(row_job(&mut dec.worker));
    ok
}

/// `WebPGetWorkerInterface()->Sync(&dec->worker)` with the `put()` branch of the
/// row that the worker finished, which C runs inside the job: returns false
/// when the job or a `put()` failed, now or since the worker was reset.
fn sync_worker(
    dec: &mut VP8Decoder,
    alpha_data: Option<&[u8]>,
    hooks: Option<&mut (dyn VP8IoHooks + '_)>,
) -> bool {
    let worker_ok = dec.worker.sync();
    if dec.put_pending {
        dec.put_pending = false;
        if worker_ok {
            let mut io = row_job(&mut dec.worker).thread_ctx.io;
            let ok = finish_row_put(dec, &mut io, alpha_data, hooks);
            row_job(&mut dec.worker).thread_ctx.io = io;
            if !ok {
                dec.put_error = true;
            }
        }
    }
    worker_ok && !dec.put_error
}

/// `WebPGetWorkerInterface()->Sync(&dec->worker)` of `ParseFrame`, with the
/// `put()` branch of the last row (see the module documentation). This
/// function has no C counterpart of its own.
pub(crate) fn vp8_sync_rows(
    dec: &mut VP8Decoder,
    alpha_data: Option<&[u8]>,
    hooks: Option<&mut (dyn VP8IoHooks + '_)>,
) -> bool {
    sync_worker(dec, alpha_data, hooks)
}

/// Copies the decoder's fields that the row job reads into the job (C reads
/// them through `dec`).
fn update_job(dec: &mut VP8Decoder) {
    let (mt_method, filter_type, dither, mb_w, mb_h) = (
        dec.mt_method,
        dec.filter_type,
        dec.dither,
        dec.mb_w,
        dec.mb_h,
    );
    let (tl_mb_x, br_mb_x, br_mb_y, num_caches) =
        (dec.tl_mb_x, dec.br_mb_x, dec.br_mb_y, dec.num_caches);
    let (cache_y_stride, cache_uv_stride) = (dec.cache_y_stride, dec.cache_uv_stride);
    let job = row_job(&mut dec.worker);
    job.mt_method = mt_method;
    job.filter_type = filter_type;
    job.dither = dither;
    job.mb_w = mb_w;
    job.mb_h = mb_h;
    job.tl_mb_x = tl_mb_x;
    job.br_mb_x = br_mb_x;
    job.br_mb_y = br_mb_y;
    job.num_caches = num_caches;
    job.cache_y_stride = cache_y_stride;
    job.cache_uv_stride = cache_uv_stride;
}

//------------------------------------------------------------------------------

// upstream: src/dec/frame_dec.c VP8ProcessRow
/// Process the last decoded row (filtering + output). `alpha_data` is C's
/// `dec->alpha_data[..alpha_data_size]` and `hooks` the io's hooks (ADR 0003).
/// Returns false in case of error.
///
/// # Panics
///
/// Panics when the decoder was not set up by `VP8InitFrame`.
pub fn vp8_process_row(
    dec: &mut VP8Decoder,
    io: &mut VP8Io,
    alpha_data: Option<&[u8]>,
    hooks: Option<&mut (dyn VP8IoHooks + '_)>,
) -> bool {
    let mut ok = true;
    let filter_row = dec.filter_type > 0 && dec.mb_y >= dec.tl_mb_y && dec.mb_y <= dec.br_mb_y;
    if dec.mt_method == 0 {
        update_job(dec);
        // ctx->id and ctx->f_info are already set. C's ctx->mb_data and
        // ctx->f_info are dec->mb_data and dec->f_info here: they move into the
        // context for the row.
        {
            let VP8Decoder {
                worker,
                mb_data,
                f_info,
                mb_y,
                ..
            } = dec;
            let ctx = &mut row_job(worker).thread_ctx;
            ctx.mb_y = *mb_y;
            ctx.filter_row = filter_row;
            std::mem::swap(&mut ctx.mb_data, mb_data);
            std::mem::swap(&mut ctx.f_info, f_info);
        }
        reconstruct_row(row_job(&mut dec.worker));
        ok = finish_row(dec, io, alpha_data, hooks);
        let VP8Decoder {
            worker,
            mb_data,
            f_info,
            ..
        } = dec;
        let ctx = &mut row_job(worker).thread_ctx;
        std::mem::swap(&mut ctx.mb_data, mb_data);
        std::mem::swap(&mut ctx.f_info, f_info);
    } else {
        // Finish previous job *before* updating context
        ok &= sync_worker(dec, alpha_data, hooks);
        debug_assert_eq!(dec.worker.status(), WebPWorkerStatus::OK);
        if ok {
            update_job(dec);
            // spawn a new deblocking/output job
            let VP8Decoder {
                worker,
                mb_data,
                f_info,
                mb_y,
                cache_id,
                mt_method,
                ..
            } = dec;
            let job = row_job(worker);
            let ctx = &mut job.thread_ctx;
            ctx.io = *io;
            ctx.id = *cache_id;
            ctx.mb_y = *mb_y;
            ctx.filter_row = filter_row;
            // swap macroblock data; for mt_method 1 too, where C shares one
            // buffer between the parser and the worker (docs/status/wp6.md).
            std::mem::swap(&mut ctx.mb_data, mb_data);
            if *mt_method != 2 {
                // perform reconstruction directly in main thread
                reconstruct_row(job);
            }
            if filter_row {
                // swap filter info
                std::mem::swap(&mut row_job(worker).thread_ctx.f_info, f_info);
            }
            // (reconstruct)+filter in parallel
            dec.worker.launch();
            dec.put_pending = true;
            dec.cache_id += 1;
            if dec.cache_id == dec.num_caches {
                dec.cache_id = 0;
            }
        }
    }
    ok
}

//------------------------------------------------------------------------------
// Finish setting up the decoding parameter once user's setup() is called.

// upstream: src/dec/frame_dec.c VP8EnterCritical
/// Call `setup()` first and finish setting up scan parameters. After this call
/// returns, one must always call `VP8ExitCritical()` with the same parameters.
/// Both functions should be used in pair. Returns `Err` with the recorded status
/// if the setup hook fails.
///
/// # Errors
///
/// Returns `VP8_STATUS_USER_ABORT` (or an older recorded status) when the setup
/// hook fails.
pub fn vp8_enter_critical(
    dec: &mut VP8Decoder,
    io: &mut VP8Io,
    hooks: Option<&mut (dyn VP8IoHooks + '_)>,
) -> Result<(), VP8StatusCode> {
    // Call setup() first. This may trigger additional decoding features on 'io'.
    // Note: Afterward, we must call teardown() no matter what.
    if let Some(hooks) = hooks
        && hooks.has_setup()
        && !hooks.setup(io)
    {
        let _ = vp8_set_error(
            dec,
            VP8StatusCode::VP8_STATUS_USER_ABORT,
            "Frame setup failed",
        );
        return Err(dec.status);
    }

    // Disable filtering per user request
    if io.bypass_filtering {
        dec.filter_type = 0;
    }

    // Define the area where we can skip in-loop filtering, in case of cropping.
    //
    // 'Simple' filter reads two luma samples outside of the macroblock
    // and filters one. It doesn't filter the chroma samples. Hence, we can
    // avoid doing the in-loop filtering before crop_top/crop_left position.
    // For the 'Complex' filter, 3 samples are read and up to 3 are filtered.
    // Means: there's a dependency chain that goes all the way up to the
    // top-left corner of the picture (MB #0). We must filter all the previous
    // macroblocks.
    {
        let extra_pixels = filter_extra_rows(dec.filter_type);
        if dec.filter_type == 2 {
            // For complex filter, we need to preserve the dependency chain.
            dec.tl_mb_x = 0;
            dec.tl_mb_y = 0;
        } else {
            // For simple filter, we can filter only the cropped region.
            // We include 'extra_pixels' on the other side of the boundary, since
            // vertical or horizontal filtering of the previous macroblock can
            // modify some abutting pixels.
            dec.tl_mb_x = (io.crop_left - extra_pixels) >> 4;
            dec.tl_mb_y = (io.crop_top - extra_pixels) >> 4;
            if dec.tl_mb_x < 0 {
                dec.tl_mb_x = 0;
            }
            if dec.tl_mb_y < 0 {
                dec.tl_mb_y = 0;
            }
        }
        // We need some 'extra' pixels on the right/bottom.
        dec.br_mb_y = (io.crop_bottom + 15 + extra_pixels) >> 4;
        dec.br_mb_x = (io.crop_right + 15 + extra_pixels) >> 4;
        if dec.br_mb_x > dec.mb_w {
            dec.br_mb_x = dec.mb_w;
        }
        if dec.br_mb_y > dec.mb_h {
            dec.br_mb_y = dec.mb_h;
        }
    }
    precompute_filter_strengths(dec);
    Ok(())
}

// upstream: src/dec/frame_dec.c VP8ExitCritical
/// Must always be called in pair with `VP8EnterCritical()`. Returns false in
/// case of error.
pub fn vp8_exit_critical(
    dec: &mut VP8Decoder,
    io: &mut VP8Io,
    alpha_data: Option<&[u8]>,
    hooks: Option<&mut (dyn VP8IoHooks + '_)>,
) -> bool {
    let mut hooks = hooks;
    let mut ok = true;
    if dec.mt_method > 0 {
        ok = sync_worker(dec, alpha_data, hooks.as_deref_mut());
    }

    if let Some(hooks) = hooks
        && hooks.has_teardown()
    {
        hooks.teardown(io);
    }
    ok
}

//------------------------------------------------------------------------------
// For multi-threaded decoding we need to use 3 rows of 16 pixels as delay line.
//
// Reason is: the deblocking filter cannot deblock the bottom horizontal edges
// immediately, and needs to wait for first few rows of the next macroblock to
// be decoded. Hence, deblocking is lagging behind by 4 or 8 pixels (depending
// on strength).
// With two threads, the vertical positions of the rows being decoded are:
// Decode:  [ 0..15][16..31][32..47][48..63][64..79][...
// Deblock:         [ 0..11][12..27][28..43][44..59][...
// If we use two threads and two caches of 16 pixels, the sequence would be:
// Decode:  [ 0..15][16..31][ 0..15!!][16..31][ 0..15][...
// Deblock:         [ 0..11][12..27!!][-4..11][12..27][...
// The problem occurs during row [12..15!!] that both the decoding and
// deblocking threads are writing simultaneously.
// With 3 cache lines, one get a safe write pattern:
// Decode:  [ 0..15][16..31][32..47][ 0..15][16..31][32..47][0..
// Deblock:         [ 0..11][12..27][28..43][-4..11][12..27][28...
// Note that multi-threaded output _without_ deblocking can make use of two
// cache lines of 16 pixels only, since there's no lagging behind. The decoding
// and output process have non-concurrent writing:
// Decode:  [ 0..15][16..31][ 0..15][16..31][...
// io->put:         [ 0..15][16..31][ 0..15][...

// upstream: src/dec/frame_dec.c MT_CACHE_LINES
const MT_CACHE_LINES: i32 = 3;
// upstream: src/dec/frame_dec.c ST_CACHE_LINES
/// 1 cache row only for single-threaded case.
const ST_CACHE_LINES: i32 = 1;

// upstream: src/dec/frame_dec.c InitThreadContext
/// Initialize multi/single-thread worker. C sets the worker's data and hook
/// here; `VP8New` set the job and the hook once, and the job's buffers are
/// allocated by `AllocateMemory`.
fn init_thread_context(dec: &mut VP8Decoder) -> bool {
    dec.cache_id = 0;
    if dec.mt_method > 0 {
        if !dec.worker.reset() {
            let _ = vp8_set_error(
                dec,
                VP8StatusCode::VP8_STATUS_OUT_OF_MEMORY,
                "thread initialization failed.",
            );
            return false;
        }
        // C: Reset clears had_error; the deferred put's flags go with it.
        dec.put_pending = false;
        dec.put_error = false;
        dec.num_caches = if dec.filter_type > 0 {
            MT_CACHE_LINES
        } else {
            MT_CACHE_LINES - 1
        };
    } else {
        dec.num_caches = ST_CACHE_LINES;
    }
    true
}

// upstream: src/dec/frame_dec.c VP8GetThreadMethod
/// Return the multi-threading method to use (0=off), depending on options and
/// bitstream size. Only for lossy decoding. C also takes the
/// `WebPHeaderStructure`, which it reads only in
/// `assert(headers == NULL || !headers->is_lossless)`; the type is WP12's, and
/// its caller keeps the assertion (`docs/status/wp6.md`).
#[must_use]
pub fn vp8_get_thread_method(options: Option<&WebPDecoderOptions>, width: i32, height: i32) -> i32 {
    let Some(options) = options else {
        return 0;
    };
    if options.use_threads == 0 {
        return 0;
    }
    let _ = height;
    // WEBP_USE_THREAD is defined in the reference configuration.
    if width >= MIN_WIDTH_FOR_THREADS {
        return 2;
    }
    0
}

//------------------------------------------------------------------------------
// Memory setup

// upstream: src/dec/frame_dec.c AllocateMemory
/// Applies the size test of the one block that C allocates and carves, then
/// allocates the parts separately (ADR 0003, "Allocation limits"). C reuses its
/// block when it is large enough; the parts that it does not initialize are
/// written before they are read, so fresh zeroed parts decode the same.
fn allocate_memory(dec: &mut VP8Decoder, has_alpha_data: bool) -> bool {
    let num_caches = dec.num_caches;
    let mb_w = index(dec.mb_w);
    // Note: we use 'size_t' when there's no overflow risk, uint64_t otherwise.
    let intra_pred_mode_size = 4 * mb_w;
    let top_size = VP8TopSamples::C_SIZE * mb_w;
    let mb_info_size = (mb_w + 1) * VP8MB::C_SIZE;
    let f_info_size = if dec.filter_type > 0 {
        mb_w * if dec.mt_method > 0 { 2 } else { 1 } * VP8FInfo::C_SIZE
    } else {
        0
    };
    let yuv_size = index(YUV_SIZE);
    let mb_data_size = if dec.mt_method == 2 { 2 } else { 1 } * mb_w * VP8MBData::C_SIZE;
    let cache_height = index(16 * num_caches + filter_extra_rows(dec.filter_type)) * 3 / 2;
    let cache_size = top_size * cache_height;
    // alpha_size is the only one that scales as width x height.
    let alpha_size: u64 = if has_alpha_data {
        u64::from(dec.pic_hdr.width) * u64::from(dec.pic_hdr.height)
    } else {
        0
    };
    let needed: u64 = intra_pred_mode_size as u64
        + top_size as u64
        + mb_info_size as u64
        + f_info_size as u64
        + yuv_size as u64
        + mb_data_size as u64
        + cache_size as u64
        + alpha_size
        + WEBP_ALIGN_CST as u64;

    if !check_size_overflow(needed) {
        return false; // check for overflow
    }
    if needed > dec.mem_size as u64 {
        dec.mem_size = 0;
        if webp_safe_malloc::<u8>(needed, 1).is_none() {
            let _ = vp8_set_error(
                dec,
                VP8StatusCode::VP8_STATUS_OUT_OF_MEMORY,
                "no memory during frame initialization.",
            );
            return false;
        }
        // down-cast is ok, thanks to WebPSafeMalloc() above.
        dec.mem_size = usize::try_from(needed).expect("an accepted size fits");
    }

    let filter_type = dec.filter_type;
    let mt_method = dec.mt_method;
    dec.intra_t = vec![0; intra_pred_mode_size];
    dec.mb_info = vec![VP8MB::default(); mb_w + 1];
    dec.f_info = if f_info_size > 0 {
        vec![VP8FInfo::default(); mb_w]
    } else {
        Vec::new()
    };
    dec.mb_data = vec![VP8MBData::default(); mb_w];
    dec.cache_y_stride = 16 * dec.mb_w;
    dec.cache_uv_stride = 8 * dec.mb_w;
    dec.cache_id = 0;
    {
        let job = row_job(&mut dec.worker);
        job.yuv_t = vec![VP8TopSamples::default(); mb_w];
        job.thread_ctx.id = 0;
        // secondary cache line. The deblocking process need to make use of the
        // filtering strength from previous macroblock row, while the new ones
        // are being decoded in parallel. We'll just swap the pointers.
        job.thread_ctx.f_info = if filter_type > 0 && mt_method > 0 {
            vec![VP8FInfo::default(); mb_w]
        } else {
            Vec::new()
        };
        job.yuv_b = vec![0; yuv_size];
        // C gives the thread context its own half for mt_method 2 and shares
        // dec->mb_data otherwise; mt_method 1 gets its own buffer here.
        job.thread_ctx.mb_data = if mt_method > 0 {
            vec![VP8MBData::default(); mb_w]
        } else {
            Vec::new()
        };
        let extra_rows = index(filter_extra_rows(filter_type));
        let y_stride = 16 * mb_w;
        let uv_stride = 8 * mb_w;
        let extra_y = extra_rows * y_stride;
        let extra_uv = (extra_rows / 2) * uv_stride;
        let caches = index(num_caches);
        job.cache_y = vec![0; extra_y + 16 * caches * y_stride];
        job.cache_u = vec![0; extra_uv + 8 * caches * uv_stride];
        job.cache_v = vec![0; extra_uv + 8 * caches * uv_stride];
        job.cache_y_pos = extra_y;
        job.cache_uv_pos = extra_uv;
    }

    // alpha plane
    dec.frame_alpha = vec![0; usize::try_from(alpha_size).expect("an accepted size fits")];
    dec.alpha_plane = if alpha_size > 0 {
        VP8AlphaPlane::Frame
    } else {
        VP8AlphaPlane::None
    };

    // note: left/top-info is initialized once for all.
    vp8_init_scanline(dec); // initialize left too.

    // initialize top
    dec.intra_t.fill(byte(B_DC_PRED));

    true
}

// upstream: src/dec/frame_dec.c InitIo
/// Prepares `io`. C also points `io->y`, `io->u` and `io->v` at the cache and
/// sets `io->a` to `NULL`; those are the rows of each `put()` here.
fn init_io(dec: &VP8Decoder, io: &mut VP8Io) {
    // prepare 'io'
    io.mb_y = 0;
    io.y_stride = dec.cache_y_stride;
    io.uv_stride = dec.cache_uv_stride;
}

// upstream: src/dec/frame_dec.c VP8InitFrame
/// Allocates the frame memory and sets up the worker and `io`. `alpha_data` is
/// C's `dec->alpha_data`, whose presence sizes the alpha part. Returns false on
/// error, which C does not always record in the status.
pub fn vp8_init_frame(dec: &mut VP8Decoder, io: &mut VP8Io, alpha_data: Option<&[u8]>) -> bool {
    if !init_thread_context(dec) {
        return false; // call first. Sets dec->num_caches.
    }
    if !allocate_memory(dec, alpha_data.is_some()) {
        return false;
    }
    init_io(dec, io);
    update_job(dec);
    // upstream: src/dsp/dec.c VP8DspInit (omitted, ADR 0003)
    true
}
