// libwebp v1.6.0, src/enc/analysis_enc.c:
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
// Macroblock analysis
//
// Author: Skal (pascal.massimino@gmail.com)

//! Translation of `src/enc/analysis_enc.c` from libwebp v1.6.0.
//!
//! Work package: WP10 (VP8 encoder core).

use crate::dec::common_dec::NUM_MB_SEGMENTS;
use crate::dsp::dsp::{BPS, VP8Histogram};
use crate::dsp::enc::{vp8_collect_histogram, vp8_mean16x4};
use crate::enc::iterator_enc::{
    vp8_iterator_import, vp8_iterator_init, vp8_iterator_is_done, vp8_iterator_next,
    vp8_iterator_progress, vp8_iterator_set_count_down, vp8_iterator_set_row,
    vp8_set_intra_uv_mode, vp8_set_intra4_mode, vp8_set_intra16_mode, vp8_set_segment,
    vp8_set_skip,
};
use crate::enc::quant_enc::{
    VP8I16_MODE_OFFSETS, VP8UV_MODE_OFFSETS, vp8_make_chroma8_preds, vp8_make_luma16_preds,
};
use crate::enc::vp8i_enc::{ModeRows, U_OFF_ENC, VP8EncIterator, VP8Encoder, VP8MBInfo, Y_OFF_ENC};
use crate::enc::webp_enc::{webp_encoding_set_error, webp_report_progress};
use crate::utils::utils::webp_safe_malloc;
use crate::webp::encode::{WebPConfig, WebPEncodingError, WebPPicture, WebPProgressHook};
use crate::webp::types::{PlaneOffset, PlaneStorage};

/// Converts a non-negative C `int` to an index.
#[expect(clippy::cast_sign_loss, reason = "the value is non-negative")]
const fn idx(v: i32) -> usize {
    v as usize
}

// upstream: src/enc/analysis_enc.c MAX_ITERS_K_MEANS
const MAX_ITERS_K_MEANS: i32 = 6;

//------------------------------------------------------------------------------
// Smooth the segment map by replacing isolated block by the majority of its
// neighbours.

// upstream: src/enc/analysis_enc.c SmoothSegmentMap
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "the majority segment is in 0..4"
)]
#[expect(clippy::cast_possible_wrap, reason = "the majority segment is in 0..4")]
fn smooth_segment_map(enc: &mut VP8Encoder) {
    let w = idx(enc.mb_w);
    let h = idx(enc.mb_h);
    let majority_cnt_3_x_3_grid = 5;
    // C: WebPSafeMalloc(w * h, sizeof(*tmp)); the entries it reads are written first.
    let Some(mut tmp) = webp_safe_malloc::<u8>((w * h) as u64, 1) else {
        return;
    };
    tmp.resize(w * h, 0);

    for y in 1..h.saturating_sub(1) {
        for x in 1..w.saturating_sub(1) {
            let mut cnt = [0_i32; NUM_MB_SEGMENTS as usize];
            let pos = x + w * y;
            let mb = &enc.mb_info;
            let mut majority_seg = i32::from(mb[pos].segment);
            // Check the 8 neighbouring segment values.
            cnt[usize::from(mb[pos - w - 1].segment)] += 1; // top-left
            cnt[usize::from(mb[pos - w].segment)] += 1; // top
            cnt[usize::from(mb[pos - w + 1].segment)] += 1; // top-right
            cnt[usize::from(mb[pos - 1].segment)] += 1; // left
            cnt[usize::from(mb[pos + 1].segment)] += 1; // right
            cnt[usize::from(mb[pos + w - 1].segment)] += 1; // bottom-left
            cnt[usize::from(mb[pos + w].segment)] += 1; // bottom
            cnt[usize::from(mb[pos + w + 1].segment)] += 1; // bottom-right
            for (n, &count) in cnt.iter().enumerate() {
                if count >= majority_cnt_3_x_3_grid {
                    majority_seg = n as i32;
                    break;
                }
            }
            tmp[x + y * w] = majority_seg as u8;
        }
    }
    for y in 1..h.saturating_sub(1) {
        for x in 1..w.saturating_sub(1) {
            let mb = &mut enc.mb_info[x + w * y];
            mb.segment = tmp[x + y * w] & 3;
        }
    }
}

//------------------------------------------------------------------------------
// set segment susceptibility 'alpha' / 'beta'

// upstream: src/enc/analysis_enc.c clip
fn clip(v: i32, m: i32, mm: i32) -> i32 {
    if v < m {
        m
    } else if v > mm {
        mm
    } else {
        v
    }
}

// upstream: src/enc/analysis_enc.c SetSegmentAlphas
#[expect(
    clippy::needless_range_loop,
    reason = "the C index loop over two arrays"
)]
fn set_segment_alphas(enc: &mut VP8Encoder, centers: &[i32; NUM_MB_SEGMENTS as usize], mid: i32) {
    let nb = enc.segment_hdr.num_segments;
    let mut min = centers[0];
    let mut max = centers[0];

    if nb > 1 {
        for &center in &centers[..idx(nb)] {
            if min > center {
                min = center;
            }
            if max < center {
                max = center;
            }
        }
    }
    if max == min {
        max = min + 1;
    }
    debug_assert!(mid <= max && mid >= min);
    for n in 0..idx(nb) {
        let alpha = 255 * (centers[n] - mid) / (max - min);
        let beta = 255 * (centers[n] - min) / (max - min);
        enc.dqm[n].alpha = clip(alpha, -127, 127);
        enc.dqm[n].beta = clip(beta, 0, 255);
    }
}

//------------------------------------------------------------------------------
// Compute susceptibility based on DCT-coeff histograms:
// the higher, the "easier" the macroblock is to compress.

// upstream: src/enc/analysis_enc.c MAX_ALPHA
/// 8b of precision for susceptibilities.
const MAX_ALPHA: i32 = 255;
// upstream: src/enc/analysis_enc.c ALPHA_SCALE
/// scaling factor for alpha.
const ALPHA_SCALE: i32 = 2 * MAX_ALPHA;
// upstream: src/enc/analysis_enc.c DEFAULT_ALPHA
const DEFAULT_ALPHA: i32 = -1;

// upstream: src/enc/analysis_enc.c IS_BETTER_ALPHA
fn is_better_alpha(alpha: i32, best_alpha: i32) -> bool {
    alpha > best_alpha
}

// upstream: src/enc/analysis_enc.c FinalAlphaValue
fn final_alpha_value(alpha: i32) -> i32 {
    let alpha = MAX_ALPHA - alpha;
    clip(alpha, 0, MAX_ALPHA)
}

// upstream: src/enc/analysis_enc.c GetAlpha
#[expect(clippy::trivially_copy_pass_by_ref, reason = "C passes a pointer")]
fn get_alpha(histo: &VP8Histogram) -> i32 {
    // 'alpha' will later be clipped to [0..MAX_ALPHA] range, clamping outer
    // values which happen to be mostly noise. This leaves the maximum precision
    // for handling the useful small values which contribute most.
    let max_value = histo.max_value;
    let last_non_zero = histo.last_non_zero;
    if max_value > 1 {
        ALPHA_SCALE * last_non_zero / max_value
    } else {
        0
    }
}

// upstream: src/enc/analysis_enc.c InitHistogram
fn init_histogram(histo: &mut VP8Histogram) {
    histo.max_value = 0;
    histo.last_non_zero = 1;
}

//------------------------------------------------------------------------------
// Simplified k-Means, to assign Nb segments based on alpha-histogram

// upstream: src/enc/analysis_enc.c AssignSegments
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    reason = "alphas, centers and segments are small non-negative values"
)]
fn assign_segments(
    enc: &mut VP8Encoder,
    config: &WebPConfig,
    alphas: &[i32; MAX_ALPHA as usize + 1],
) {
    // 'num_segments' is previously validated and <= NUM_MB_SEGMENTS, but an
    // explicit check is needed to avoid spurious warning about 'n + 1' exceeding
    // array bounds of 'centers' with some compilers (noticed with gcc-4.9).
    let nb = if enc.segment_hdr.num_segments < NUM_MB_SEGMENTS {
        enc.segment_hdr.num_segments
    } else {
        NUM_MB_SEGMENTS
    };
    let mut centers = [0_i32; NUM_MB_SEGMENTS as usize];
    let mut weighted_average = 0;
    let mut map = [0_i32; MAX_ALPHA as usize + 1];
    // 'int' type is ok for histo, and won't overflow
    let mut accum = [0_i32; NUM_MB_SEGMENTS as usize];
    let mut dist_accum = [0_i32; NUM_MB_SEGMENTS as usize];

    debug_assert!(nb >= 1);
    debug_assert!(nb <= NUM_MB_SEGMENTS);

    // bracket the input
    let mut n: i32 = 0;
    while n <= MAX_ALPHA && alphas[idx(n)] == 0 {
        n += 1;
    }
    let min_a = n;
    n = MAX_ALPHA;
    while n > min_a && alphas[idx(n)] == 0 {
        n -= 1;
    }
    let max_a = n;
    let range_a = max_a - min_a;

    // Spread initial centers evenly
    let mut n = 1;
    for k in 0..nb {
        debug_assert!(n < 2 * nb);
        centers[idx(k)] = min_a + (n * range_a) / (2 * nb);
        n += 2;
    }

    for _ in 0..MAX_ITERS_K_MEANS {
        // few iters are enough
        // Reset stats
        for n in 0..idx(nb) {
            accum[n] = 0;
            dist_accum[n] = 0;
        }
        // Assign nearest center for each 'a'
        let mut n: usize = 0; // track the nearest center for current 'a'
        for a in min_a..=max_a {
            if alphas[idx(a)] != 0 {
                while n + 1 < idx(nb) && (a - centers[n + 1]).abs() < (a - centers[n]).abs() {
                    n += 1;
                }
                map[idx(a)] = n as i32;
                // accumulate contribution into best centroid
                dist_accum[n] += a * alphas[idx(a)];
                accum[n] += alphas[idx(a)];
            }
        }
        // All point are classified. Move the centroids to the
        // center of their respective cloud.
        let mut displaced = 0;
        weighted_average = 0;
        let mut total_weight = 0;
        for n in 0..idx(nb) {
            if accum[n] != 0 {
                let new_center = (dist_accum[n] + accum[n] / 2) / accum[n];
                displaced += (centers[n] - new_center).abs();
                centers[n] = new_center;
                weighted_average += new_center * accum[n];
                total_weight += accum[n];
            }
        }
        weighted_average = (weighted_average + total_weight / 2) / total_weight;
        if displaced < 5 {
            break; // no need to keep on looping...
        }
    }

    // Map each original value to the closest centroid
    for n in 0..idx(enc.mb_w * enc.mb_h) {
        let mb = &mut enc.mb_info[n];
        let alpha = usize::from(mb.alpha);
        mb.segment = (map[alpha] & 3) as u8;
        mb.alpha = centers[idx(map[alpha])] as u8; // for the record.
    }

    if nb > 1 {
        let smooth = (config.preprocessing & 1) != 0;
        if smooth {
            smooth_segment_map(enc);
        }
    }

    set_segment_alphas(enc, &centers, weighted_average); // pick some alphas.
}

//------------------------------------------------------------------------------
// Macroblock analysis: collect histogram for each mode, deduce the maximal
// susceptibility and set best modes for this macroblock.
// Segment assignment is done later.

// Number of modes to inspect for 'alpha' evaluation. We don't need to test all
// the possible modes during the analysis phase: we risk falling into a local
// optimum, or be subject to boundary effect
// upstream: src/enc/analysis_enc.c MAX_INTRA16_MODE
const MAX_INTRA16_MODE: i32 = 2;
// upstream: src/enc/analysis_enc.c MAX_INTRA4_MODE
#[expect(dead_code, reason = "the C macro is unused in the C source too")]
const MAX_INTRA4_MODE: i32 = 2;
// upstream: src/enc/analysis_enc.c MAX_UV_MODE
const MAX_UV_MODE: i32 = 2;

// upstream: src/enc/analysis_enc.c MBAnalyzeBestIntra16Mode
fn mb_analyze_best_intra16_mode(
    it: &mut VP8EncIterator,
    enc: &VP8Encoder,
    rows: &mut ModeRows<'_>,
) -> i32 {
    let max_mode = MAX_INTRA16_MODE;
    let mut best_alpha = DEFAULT_ALPHA;
    let mut best_mode = 0;

    vp8_make_luma16_preds(it, enc);
    for mode in 0..max_mode {
        let mut histo = VP8Histogram::default();

        init_histogram(&mut histo);
        vp8_collect_histogram(
            &it.yuv_in[idx(Y_OFF_ENC)..],
            &it.yuv_p[usize::from(VP8I16_MODE_OFFSETS[idx(mode)])..],
            0,
            16,
            &mut histo,
        );
        let alpha = get_alpha(&histo);
        if is_better_alpha(alpha, best_alpha) {
            best_alpha = alpha;
            best_mode = mode;
        }
    }
    vp8_set_intra16_mode(it, rows, best_mode);
    best_alpha
}

// upstream: src/enc/analysis_enc.c FastMBAnalyze
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "C converts the quality to int and the threshold to uint32_t"
)]
fn fast_mb_analyze(it: &VP8EncIterator, config: &WebPConfig, rows: &mut ModeRows<'_>) -> i32 {
    // Empirical cut-off value, should be around 16 (~=block size). We use the
    // [8-17] range and favor intra4 at high quality, intra16 for low quality.
    let q = config.quality as i32;
    let k_threshold = (8 + (17 - 8) * q / 100) as u32;
    let mut dc = [0_u32; 16];
    for k in (0..16).step_by(4) {
        vp8_mean16x4(&it.yuv_in[idx(Y_OFF_ENC) + k * idx(BPS)..], &mut dc[k..]);
    }
    let mut m: u32 = 0;
    let mut m2: u32 = 0;
    for &value in &dc {
        m = m.wrapping_add(value);
        m2 = m2.wrapping_add(value.wrapping_mul(value));
    }
    if k_threshold.wrapping_mul(m2) < m.wrapping_mul(m) {
        vp8_set_intra16_mode(it, rows, 0); // DC16
    } else {
        let modes = [0_u8; 16]; // DC4
        vp8_set_intra4_mode(it, rows, &modes);
    }
    0
}

// upstream: src/enc/analysis_enc.c MBAnalyzeBestUVMode
fn mb_analyze_best_uv_mode(
    it: &mut VP8EncIterator,
    enc: &VP8Encoder,
    rows: &mut ModeRows<'_>,
) -> i32 {
    let mut best_alpha = DEFAULT_ALPHA;
    let mut smallest_alpha = 0;
    let mut best_mode = 0;
    let max_mode = MAX_UV_MODE;

    vp8_make_chroma8_preds(it, enc);
    for mode in 0..max_mode {
        let mut histo = VP8Histogram::default();
        init_histogram(&mut histo);
        vp8_collect_histogram(
            &it.yuv_in[idx(U_OFF_ENC)..],
            &it.yuv_p[usize::from(VP8UV_MODE_OFFSETS[idx(mode)])..],
            16,
            16 + 4 + 4,
            &mut histo,
        );
        let alpha = get_alpha(&histo);
        if is_better_alpha(alpha, best_alpha) {
            best_alpha = alpha;
        }
        // The best prediction mode tends to be the one with the smallest alpha.
        if mode == 0 || alpha < smallest_alpha {
            smallest_alpha = alpha;
            best_mode = mode;
        }
    }
    vp8_set_intra_uv_mode(it, rows, best_mode);
    best_alpha
}

// upstream: src/enc/analysis_enc.c MBAnalyze
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "the final alpha is in 0..=MAX_ALPHA"
)]
fn mb_analyze(
    it: &mut VP8EncIterator,
    enc: &VP8Encoder,
    config: &WebPConfig,
    rows: &mut ModeRows<'_>,
    alphas: &mut [i32; MAX_ALPHA as usize + 1],
    alpha: &mut i32,
    uv_alpha: &mut i32,
) {
    vp8_set_intra16_mode(it, rows, 0); // default: Intra16, DC_PRED
    vp8_set_skip(it, rows, 0); // not skipped
    vp8_set_segment(it, rows, 0); // default segment, spec-wise.

    let mut best_alpha = if enc.method <= 1 {
        fast_mb_analyze(it, config, rows)
    } else {
        mb_analyze_best_intra16_mode(it, enc, rows)
    };
    let best_uv_alpha = mb_analyze_best_uv_mode(it, enc, rows);

    // Final susceptibility mix
    best_alpha = (3 * best_alpha + best_uv_alpha + 2) >> 2;
    best_alpha = final_alpha_value(best_alpha);
    alphas[best_alpha as usize] += 1;
    rows.mb(it.mb).alpha = best_alpha as u8; // for later remapping.

    // Accumulate for later complexity analysis.
    *alpha += best_alpha; // mixed susceptibility (not just luma)
    *uv_alpha += best_uv_alpha;
}

// upstream: src/enc/analysis_enc.c DefaultMBInfo
fn default_mb_info(mb: &mut VP8MBInfo) {
    mb.r#type = 1; // I16x16
    mb.uv_mode = 0;
    mb.skip = 0; // not skipped
    mb.segment = 0; // default segment
    mb.alpha = 0;
}

//------------------------------------------------------------------------------
// Main analysis loop:
// Collect all susceptibilities for each macroblock and record their
// distribution in alphas[]. Segments is assigned a-posteriori, based on
// this histogram.
// We also pick an intra16 prediction mode, which shouldn't be considered
// final except for fast-encode settings. We can also pick some intra4 modes
// and decide intra4/intra16, but that's usually almost always a bad choice at
// this stage.

// upstream: src/enc/analysis_enc.c ResetAllMBInfo
fn reset_all_mb_info(enc: &mut VP8Encoder, pic: &mut WebPPicture<'_>) {
    for n in 0..idx(enc.mb_w * enc.mb_h) {
        default_mb_info(&mut enc.mb_info[n]);
    }
    // Default susceptibilities.
    enc.dqm[0].alpha = 0;
    enc.dqm[0].beta = 0;
    // Note: we can't compute this 'alpha' / 'uv_alpha' -> set to default value.
    enc.alpha = 0;
    enc.uv_alpha = 0;
    // C ignores the result: an abort is recorded on the picture only.
    let _ = webp_report_progress(pic, enc.percent + 20, Some(&mut enc.percent));
}

// upstream: src/enc/analysis_enc.c SegmentJob
/// struct used to collect job result. C embeds the job's worker; the Rust jobs
/// run on the calling thread or on a scoped thread (see `VP8EncAnalyze`).
#[derive(Clone, Debug)]
struct SegmentJob {
    alphas: [i32; MAX_ALPHA as usize + 1],
    alpha: i32,
    uv_alpha: i32,
    it: VP8EncIterator,
    delta_progress: i32,
}

impl Default for SegmentJob {
    fn default() -> Self {
        Self {
            alphas: [0; MAX_ALPHA as usize + 1],
            alpha: 0,
            uv_alpha: 0,
            it: VP8EncIterator::default(),
            delta_progress: 0,
        }
    }
}

// upstream: src/enc/analysis_enc.c DoSegmentsJob
/// main work call. `enc` is shared with the other job, `rows` holds the rows
/// this job writes, and `percent_store` is the encoder's `percent` (C:
/// `&enc->percent`), which only the job of the first row updates.
fn do_segments_job(
    job: &mut SegmentJob,
    enc: &VP8Encoder,
    config: &WebPConfig,
    pic: &mut WebPPicture<'_>,
    percent_store: &mut i32,
    rows: &mut ModeRows<'_>,
) -> bool {
    let SegmentJob {
        alphas,
        alpha,
        uv_alpha,
        it,
        delta_progress,
    } = job;
    let mut ok = true;
    if !vp8_iterator_is_done(it) {
        // C: tmp[32 + WEBP_ALIGN_CST] on the stack; the iterator owns the scratch.
        loop {
            // Let's pretend we have perfect lossless reconstruction.
            vp8_iterator_import(it, pic, true);
            mb_analyze(it, enc, config, rows, alphas, alpha, uv_alpha);
            ok = vp8_iterator_progress(it, pic, percent_store, *delta_progress).is_ok();
            if !(ok && vp8_iterator_next(it, enc)) {
                break;
            }
        }
    }
    ok
}

// upstream: src/enc/analysis_enc.c MergeJobs
fn merge_jobs(src: &SegmentJob, dst: &mut SegmentJob) {
    for i in 0..=idx(MAX_ALPHA) {
        dst.alphas[i] += src.alphas[i];
    }
    dst.alpha += src.alpha;
    dst.uv_alpha += src.uv_alpha;
}

// upstream: src/enc/analysis_enc.c InitSegmentJob
/// initialize the job struct with some tasks to perform
fn init_segment_job(enc: &mut VP8Encoder, job: &mut SegmentJob, start_row: i32, end_row: i32) {
    // C: WebPGetWorkerInterface()->Init(&job->worker) and the hook; the job
    // runs without a worker here.
    vp8_iterator_init(enc, &mut job.it);
    vp8_iterator_set_row(&mut job.it, enc, start_row);
    vp8_iterator_set_count_down(&mut job.it, (end_row - start_row) * enc.mb_w);
    job.alphas = [0; MAX_ALPHA as usize + 1];
    job.alpha = 0;
    job.uv_alpha = 0;
    // only one of both jobs can record the progress, since we don't
    // expect the user's hook to be multi-thread safe
    job.delta_progress = if start_row == 0 { 20 } else { 0 };
}

/// The planes of the picture that a job of the analysis reads, without the
/// callbacks, which are not `Send`. The side job rebuilds a picture from them
/// on its thread, and the main job works on such a picture with the
/// progress hook moved in, as WP11 does for the side context of
/// `VP8LEncodeStream`.
struct SourcePlanes<'p> {
    width: i32,
    height: i32,
    y: Option<PlaneOffset>,
    u: Option<PlaneOffset>,
    v: Option<PlaneOffset>,
    y_stride: i32,
    uv_stride: i32,
    error_code: WebPEncodingError,
    regions: Vec<&'p [u8]>,
}

impl<'p> SourcePlanes<'p> {
    /// The planes of `pic`, borrowed shared.
    fn of(pic: &'p WebPPicture<'_>) -> Self {
        let regions = match &pic.yuva {
            PlaneStorage::Internal(samples) => vec![samples.as_slice()],
            PlaneStorage::External(slices) => slices.iter().map(|slice| &**slice).collect(),
            PlaneStorage::Shared(slices) => slices.clone(),
        };
        Self {
            width: pic.width,
            height: pic.height,
            y: pic.y,
            u: pic.u,
            v: pic.v,
            y_stride: pic.y_stride,
            uv_stride: pic.uv_stride,
            error_code: pic.error_code,
            regions,
        }
    }

    /// A picture over the planes, without callbacks.
    fn into_picture(self) -> WebPPicture<'p> {
        WebPPicture {
            width: self.width,
            height: self.height,
            y: self.y,
            u: self.u,
            v: self.v,
            y_stride: self.y_stride,
            uv_stride: self.uv_stride,
            error_code: self.error_code,
            yuva: PlaneStorage::Shared(self.regions),
            ..WebPPicture::default()
        }
    }
}

// upstream: src/enc/analysis_enc.c VP8EncAnalyze
/// Main analysis loop. Decides the segmentations and complexity. Assigns a
/// first guess for Intra16 and 'uvmode' prediction modes.
///
/// The jobs write the macroblock infos and intra modes of their rows and read
/// the rest of the encoder, so `mb_info`, `preds` and `percent` leave the
/// encoder while the jobs run; with `thread_level > 0` the side job runs on a
/// scoped thread (ADR 0003, threads), and a thread that cannot be started is C's
/// failed `Reset`.
///
/// # Errors
///
/// Returns the code that the picture records.
#[expect(clippy::too_many_lines, reason = "the C function translated whole")]
#[expect(clippy::cast_sign_loss, reason = "rows and strides are non-negative")]
pub fn vp8_enc_analyze(
    enc: &mut VP8Encoder,
    config: &WebPConfig,
    pic: &mut WebPPicture<'_>,
) -> Result<(), WebPEncodingError> {
    let mut ok = true;
    let do_segments = config.emulate_jpeg_size != 0 // We need the complexity evaluation.
        || enc.segment_hdr.num_segments > 1
        || enc.method <= 1; // for method 0 - 1, we need preds[] to be filled.
    if do_segments {
        let last_row = enc.mb_h;
        let total_mb = last_row * enc.mb_w;
        // We give a little more than a half work to the main thread.
        let split_row = (9 * last_row + 15) >> 4;
        let k_min_split_row = 2; // minimal rows needed for mt to be worth it
        let do_mt = enc.thread_level > 0 && split_row >= k_min_split_row;
        let mut main_job = SegmentJob::default();
        if do_mt {
            let mut side_job = SegmentJob::default();
            init_segment_job(enc, &mut main_job, 0, split_row);
            init_segment_job(enc, &mut side_job, split_row, last_row);

            let mut mb_info = std::mem::take(&mut enc.mb_info);
            let mut preds = std::mem::take(&mut enc.preds);
            let mut percent = enc.percent;
            let preds_w = enc.preds_w;
            let mb_split = (split_row * enc.mb_w) as usize;
            // The side job's first intra mode; with two macroblock rows the side
            // job has none, and its first position lies one past the plane.
            let preds_split =
                (enc.preds_base() + (split_row * 4 * enc.preds_w) as usize).min(preds.len());
            let mut hook = pic.progress_hook.take();
            let mut main_error = pic.error_code;
            {
                let (mb_main, mb_side) = mb_info.split_at_mut(mb_split);
                let (preds_main, preds_side) = preds.split_at_mut(preds_split);
                let mut rows_main = ModeRows::whole(mb_main, preds_main, preds_w);
                let mut rows_side = ModeRows {
                    mb_info: mb_side,
                    mb_start: mb_split,
                    preds: preds_side,
                    preds_start: preds_split,
                    preds_w,
                };
                let side_planes = SourcePlanes::of(pic);
                let main_planes = SourcePlanes::of(pic);
                let enc_shared: &VP8Encoder = enc;
                let side_job = &mut side_job;
                let main_job = &mut main_job;
                std::thread::scope(|scope| {
                    // C: Reset() starts the side thread; Launch() starts the job.
                    let side = std::thread::Builder::new().spawn_scoped(scope, move || {
                        let mut picture = side_planes.into_picture();
                        let mut side_percent = 0;
                        do_segments_job(
                            side_job,
                            enc_shared,
                            config,
                            &mut picture,
                            &mut side_percent,
                            &mut rows_side,
                        )
                    });
                    match side {
                        Err(_) => ok = false,
                        Ok(side) => {
                            // C: Execute() runs the main job on this thread.
                            let mut picture = main_planes.into_picture();
                            if let Some(hook) = hook.as_mut() {
                                let forward: WebPProgressHook<'_> = Box::new(hook);
                                picture.progress_hook = Some(forward);
                            }
                            let main_ok = do_segments_job(
                                main_job,
                                enc_shared,
                                config,
                                &mut picture,
                                &mut percent,
                                &mut rows_main,
                            );
                            main_error = picture.error_code;
                            // C: Sync() of both workers; a panicking side job
                            // counts as a failed hook.
                            let side_ok = side.join().unwrap_or(false);
                            ok &= side_ok;
                            ok &= main_ok;
                        }
                    }
                });
            }
            pic.progress_hook = hook;
            if main_error != WebPEncodingError::VP8_ENC_OK {
                webp_encoding_set_error(pic, main_error);
            }
            enc.mb_info = mb_info;
            enc.preds = preds;
            enc.percent = percent;
            if ok {
                merge_jobs(&side_job, &mut main_job); // merge results together
            }
        } else {
            // Even for single-thread case, we use the generic Worker tools.
            init_segment_job(enc, &mut main_job, 0, last_row);
            let mut mb_info = std::mem::take(&mut enc.mb_info);
            let mut preds = std::mem::take(&mut enc.preds);
            let mut percent = enc.percent;
            let preds_w = enc.preds_w;
            {
                let mut rows = ModeRows::whole(&mut mb_info, &mut preds, preds_w);
                ok &= do_segments_job(&mut main_job, enc, config, pic, &mut percent, &mut rows);
            }
            enc.mb_info = mb_info;
            enc.preds = preds;
            enc.percent = percent;
        }
        if ok {
            enc.alpha = main_job.alpha / total_mb;
            enc.uv_alpha = main_job.uv_alpha / total_mb;
            assign_segments(enc, config, &main_job.alphas);
        }
    } else {
        // Use only one default segment.
        reset_all_mb_info(enc, pic);
    }
    if !ok {
        return Err(webp_encoding_set_error(
            pic,
            WebPEncodingError::VP8_ENC_ERROR_OUT_OF_MEMORY,
        )); // imprecise
    }
    Ok(())
}
