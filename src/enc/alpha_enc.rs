// libwebp v1.6.0, src/enc/alpha_enc.c:
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
// Alpha-plane compression.
//
// Author: Skal (pascal.massimino@gmail.com)

//! Translation of `src/enc/alpha_enc.c` from libwebp v1.6.0.
//!
//! Work package: WP10 (VP8 encoder core).
//!
//! # Deviations from upstream
//!
//! Changes of representation that the ADRs allow, or behavior that the Rust
//! side cannot reproduce exactly. Each records the reason.
//!
//! 1. **Alpha worker (ADR 0003, threads: a worker that outlives one call).**
//!    The worker's job is `VP8EncAlphaJob`: the copy of the alpha plane without
//!    stride (the first step of C's `EncodeAlpha`, made on the calling thread
//!    before `Launch`; `None` when its allocation fails, which the job reports
//!    as C would), the dimensions, `method`, `alpha_quality`,
//!    `alpha_compression` and `alpha_filtering`, a copy of the picture's
//!    statistics, and the results: the alpha data and size, `sse[3]`, the job's
//!    error code and statistics. They move into the encoder and the picture at
//!    `Sync` in `VP8EncFinishAlpha`, or in `VP8EncDeleteAlpha` when the loop
//!    failed; only the ten statistics fields that C's job writes are copied
//!    back, and the main thread writes none of them meanwhile. C sets the hook
//!    in `VP8EncInitAlpha`; the job is set at `VP8EncStartAlpha`, after
//!    `Reset`, because it carries the plane. In place (`thread_level` 0) the
//!    same job runs in `VP8EncStartAlpha` and its results move at once, which
//!    is C's order exactly. With the thread, C's worker records its error on
//!    the picture whenever it fails, racing with the main thread; the Rust
//!    job's error is recorded at `Sync`, after any error of the main thread.
//!    The order only matters when both fail, which needs an allocation failure
//!    in the job.
//! 2. **Allocation helpers.** `ApplyFiltersAndEncode` and the alpha copy: see
//!    `src/enc/analysis_enc.rs`, `# Deviations from upstream`, item 2.
//!
//! # Excluded undefined behavior
//!
//! Inputs that are undefined behavior (or assertion failures) in C and are
//! therefore excluded from the oracle comparison (PORTING rule 8):
//!
//! 1. **Uninitialized lossless statistics of uncompressed alpha.** With
//!    `alpha_compression` 0, `EncodeAlphaInternal` never writes the statistics
//!    of its `FilterTrial`, an uninitialized stack object, and
//!    `ApplyFiltersAndEncode` copies them into `pic->stats`. The encode tests
//!    do not request statistics for pictures with alpha and `alpha_compression`
//!    0; Rust copies zeros.
//! 2. **Data races of the threaded alpha job on failure.** When the main loop
//!    fails with alpha and `thread_level > 0`, C's `StoreStats` runs while the
//!    worker may still write `pic->stats`; such cases do not request
//!    statistics.
//!
//! # Checks not applicable
//!
//! These checks do not apply, or cannot be made, with the reason:
//!
//! 1. **Assertions** are `debug_assert!`;
//!    `assert(sizeof(header) == ALPHA_HEADER_LEN)` is a constant assertion.

use crate::dsp::alpha_processing::webp_dispatch_alpha_to_green;
use crate::dsp::dsp::WEBP_FILTER_TYPE;
use crate::dsp::filters::webp_filters;
use crate::enc::picture_csp_enc::webp_picture_has_transparency;
use crate::enc::picture_enc::{argb_samples_mut, webp_picture_alloc, webp_picture_free};
use crate::enc::vp8i_enc::VP8Encoder;
use crate::enc::vp8l_enc::vp8l_encode_stream;
use crate::enc::webp_enc::{webp_encoding_set_error, webp_report_progress};
use crate::utils::bit_writer_utils::{
    VP8BitWriter, VP8LBitWriter, vp8_bit_writer_append, vp8_bit_writer_init, vp8_bit_writer_size,
    vp8_bit_writer_wipe_out, vp8l_bit_writer_finish, vp8l_bit_writer_init,
    vp8l_bit_writer_num_bytes, vp8l_bit_writer_wipe_out,
};
use crate::utils::filters_utils::webp_estimate_best_filter;
use crate::utils::quant_levels_utils::quantize_levels;
use crate::utils::thread_utils::WebPWorker;
use crate::utils::utils::{webp_copy_plane, webp_safe_malloc};
use crate::webp::encode::{
    WebPAuxStats, WebPConfig, WebPEncodingError, WebPPicture, webp_config_init, webp_picture_init,
};
use crate::webp::format_constants::{
    ALPHA_HEADER_LEN, ALPHA_LOSSLESS_COMPRESSION, ALPHA_NO_COMPRESSION, ALPHA_PREPROCESSED_LEVELS,
};

/// Converts a non-negative C `int` to an index.
#[expect(clippy::cast_sign_loss, reason = "the value is non-negative")]
const fn idx(v: i32) -> usize {
    v as usize
}

/// `n` zeroed bytes after the size test of `WebPSafeMalloc(1ULL, n)`, or `None`
/// where C's allocation returns `NULL`.
fn safe_bytes(n: usize) -> Option<Vec<u8>> {
    webp_safe_malloc::<()>(1, n)?;
    let mut buffer = Vec::new();
    buffer.try_reserve_exact(n).ok()?;
    buffer.resize(n, 0);
    Some(buffer)
}

// -----------------------------------------------------------------------------
// Encodes the given alpha data via specified compression method 'method'.
// The pre-processing (quantization) is performed if 'quality' is less than 100.
// For such cases, the encoding is lossy. The valid range is [0, 100] for
// 'quality' and [0, 1] for 'method':
//   'method = 0' - No compression;
//   'method = 1' - Use lossless coder on the alpha plane only
// 'filter' values [0, 4] correspond to prediction modes none, horizontal,
// vertical & gradient filters. The prediction mode 4 will try all the
// prediction modes 0 to 3 and pick the best one.
// 'effort_level': specifies how much effort must be spent to try and reduce
//  the compressed output size. In range 0 (quick) to 6 (slow).
//
// 'output' corresponds to the buffer containing compressed alpha data.
//          This buffer is allocated by this method and caller should call
//          WebPSafeFree(*output) when done.
// 'output_size' corresponds to size of this compressed alpha buffer.
//
// Returns 1 on successfully encoding the alpha and
//         0 if either:
//           invalid quality or method, or
//           memory allocation for the compressed data fails.

// upstream: src/enc/alpha_enc.c EncodeLossless
#[expect(
    clippy::cast_precision_loss,
    reason = "C multiplies the effort level as float"
)]
fn encode_lossless(
    data: &[u8],
    width: i32,
    height: i32,
    effort_level: i32, // in [0..6] range
    use_quality_100: bool,
    bw: &mut VP8LBitWriter,
    stats: &mut WebPAuxStats,
) -> bool {
    let mut config = WebPConfig::default();
    let mut picture = WebPPicture::default();

    if !webp_picture_init(&mut picture) {
        return false;
    }
    picture.width = width;
    picture.height = height;
    picture.use_argb = 1;
    picture.stats = Some(stats);
    if webp_picture_alloc(&mut picture).is_err() {
        return false;
    }

    // Transfer the alpha values to the green channel.
    {
        let argb_stride = picture.argb_stride;
        let argb = argb_samples_mut(
            &mut picture.argb_storage,
            picture
                .argb
                .expect("the allocated picture has an ARGB plane"),
        );
        webp_dispatch_alpha_to_green(data, width, width, height, argb, argb_stride);
    }

    if !webp_config_init(&mut config) {
        return false;
    }
    config.lossless = 1;
    // Enable exact, or it would alter RGB values of transparent alpha, which is
    // normally OK but not here since we are not encoding the input image but  an
    // internal encoding-related image containing necessary exact information in
    // RGB channels.
    config.exact = 1;
    config.method = effort_level; // impact is very small
    // Set a low default quality for encoding alpha. Ensure that Alpha quality at
    // lower methods (3 and below) is less than the threshold for triggering
    // costly 'BackwardReferencesTraceBackwards'.
    // If the alpha quality is set to 100 and the method to 6, allow for a high
    // lossless quality to trigger the cruncher.
    config.quality = if use_quality_100 && effort_level == 6 {
        100.
    } else {
        8. * effort_level as f32
    };
    debug_assert!(config.quality >= 0. && config.quality <= 100.);

    let mut ok = vp8l_encode_stream(&config, &mut picture, bw).is_ok();
    webp_picture_free(&mut picture);
    ok = ok && !bw.error;
    if !ok {
        vp8l_bit_writer_wipe_out(bw);
        return false;
    }
    true
}

// -----------------------------------------------------------------------------

// upstream: src/enc/alpha_enc.c FilterTrial
/// Small struct to hold the result of a filter mode compression attempt.
#[derive(Debug, Default)]
struct FilterTrial {
    score: usize,
    bw: VP8BitWriter,
    stats: WebPAuxStats,
}

// upstream: src/enc/alpha_enc.c EncodeAlphaInternal
/// This function always returns an initialized 'bw' object, even upon error.
/// `tmp_alpha` is the buffer for the filtered samples, `None` where C passes
/// `NULL` (no filter).
#[expect(clippy::too_many_arguments, reason = "the C parameters")]
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "the alpha header is one byte"
)]
fn encode_alpha_internal(
    data: &[u8],
    width: i32,
    height: i32,
    method: i32,
    filter: i32,
    reduce_levels: bool,
    effort_level: i32, // in [0..6] range
    tmp_alpha: Option<&mut [u8]>,
    result: &mut FilterTrial,
) -> bool {
    let mut method = method;
    let mut ok = false;
    let data_size = idx(width * height);
    let mut tmp_bw = VP8LBitWriter::default();

    debug_assert_eq!(
        i64::try_from(data_size).ok(),
        Some(i64::from(width) * i64::from(height))
    ); // as per spec
    debug_assert!(u32::try_from(filter).is_ok_and(|f| f < WEBP_FILTER_TYPE::WEBP_FILTER_LAST.0));
    debug_assert!(method >= ALPHA_NO_COMPRESSION);
    debug_assert!(method <= ALPHA_LOSSLESS_COMPRESSION);
    const { assert!(ALPHA_HEADER_LEN == 1) };

    // C: filter_func = WebPFilters[filter], NULL for WEBP_FILTER_NONE.
    let filter_type = WEBP_FILTER_TYPE(filter as u32);
    let alpha_src: &[u8] = if filter_type == WEBP_FILTER_TYPE::WEBP_FILTER_NONE {
        data
    } else {
        let tmp_alpha = tmp_alpha.expect("a filter writes into the temporary plane");
        webp_filters(filter_type, data, width, height, width, tmp_alpha);
        tmp_alpha
    };

    let mut output_size: usize = 0;
    let mut compressed = false;
    if method != ALPHA_NO_COMPRESSION {
        ok = vp8l_bit_writer_init(&mut tmp_bw, data_size >> 3);
        ok = ok
            && encode_lossless(
                alpha_src,
                width,
                height,
                effort_level,
                !reduce_levels,
                &mut tmp_bw,
                &mut result.stats,
            );
        if ok {
            vp8l_bit_writer_finish(&mut tmp_bw);
            if tmp_bw.error {
                vp8l_bit_writer_wipe_out(&mut tmp_bw);
                result.bw = VP8BitWriter::default();
                return false;
            }
            output_size = vp8l_bit_writer_num_bytes(&tmp_bw);
            if output_size > data_size {
                // compressed size is larger than source! Revert to uncompressed mode.
                method = ALPHA_NO_COMPRESSION;
                vp8l_bit_writer_wipe_out(&mut tmp_bw);
            } else {
                compressed = true;
            }
        } else {
            vp8l_bit_writer_wipe_out(&mut tmp_bw);
            result.bw = VP8BitWriter::default();
            return false;
        }
    }

    if method == ALPHA_NO_COMPRESSION {
        output_size = data_size;
        ok = true;
    }
    let output: &[u8] = if compressed {
        &tmp_bw.buf[..output_size]
    } else {
        &alpha_src[..output_size]
    };

    // Emit final result.
    let mut header = (method | (filter << 2)) as u8;
    if reduce_levels {
        header |= (ALPHA_PREPROCESSED_LEVELS << 4) as u8;
    }

    if !vp8_bit_writer_init(&mut result.bw, idx(ALPHA_HEADER_LEN) + output_size) {
        ok = false;
    }
    ok = ok && vp8_bit_writer_append(&mut result.bw, &[header]);
    ok = ok && vp8_bit_writer_append(&mut result.bw, output);

    if method != ALPHA_NO_COMPRESSION {
        vp8l_bit_writer_wipe_out(&mut tmp_bw);
    }
    ok = ok && !result.bw.error;
    result.score = vp8_bit_writer_size(&result.bw);
    ok
}

// -----------------------------------------------------------------------------

// upstream: src/enc/alpha_enc.c GetNumColors
fn get_num_colors(data: &[u8], width: i32, height: i32, stride: i32) -> i32 {
    let mut colors = 0;
    let mut color = [0_u8; 256];

    for j in 0..idx(height) {
        let p = &data[j * idx(stride)..];
        for &value in &p[..idx(width)] {
            color[usize::from(value)] = 1;
        }
    }
    for &seen in &color {
        if seen > 0 {
            colors += 1;
        }
    }
    colors
}

// upstream: src/enc/alpha_enc.c FILTER_TRY_NONE
const FILTER_TRY_NONE: u32 = 1 << WEBP_FILTER_TYPE::WEBP_FILTER_NONE.0;
// upstream: src/enc/alpha_enc.c FILTER_TRY_ALL
const FILTER_TRY_ALL: u32 = (1 << WEBP_FILTER_TYPE::WEBP_FILTER_LAST.0) - 1;

// upstream: src/enc/alpha_enc.c GetFilterMap
/// Given the input 'filter' option, return an OR'd bit-set of filters to try.
fn get_filter_map(
    alpha: &[u8],
    width: i32,
    height: i32,
    filter: WEBP_FILTER_TYPE,
    effort_level: i32,
) -> u32 {
    let mut bit_map = 0_u32;
    if filter == WEBP_FILTER_TYPE::WEBP_FILTER_FAST {
        // Quick estimate of the best candidate.
        let try_filter_none = effort_level > 3;
        let k_min_colors_for_filter_none = 16;
        let k_max_colors_for_filter_none = 192;
        let num_colors = get_num_colors(alpha, width, height, width);
        // For low number of colors, NONE yields better compression.
        let filter = if num_colors <= k_min_colors_for_filter_none {
            WEBP_FILTER_TYPE::WEBP_FILTER_NONE
        } else {
            webp_estimate_best_filter(alpha, width, height, width)
        };
        bit_map |= 1 << filter.0;
        // For large number of colors, try FILTER_NONE in addition to the best
        // filter as well.
        if try_filter_none || num_colors > k_max_colors_for_filter_none {
            bit_map |= FILTER_TRY_NONE;
        }
    } else if filter == WEBP_FILTER_TYPE::WEBP_FILTER_NONE {
        bit_map = FILTER_TRY_NONE;
    } else {
        // WEBP_FILTER_BEST -> try all
        bit_map = FILTER_TRY_ALL;
    }
    bit_map
}

// upstream: src/enc/alpha_enc.c InitFilterTrial
fn init_filter_trial(score: &mut FilterTrial) {
    score.score = usize::try_from(!0_u32).expect("usize holds 32 bits"); // C: (size_t)~0U
    vp8_bit_writer_init(&mut score.bw, 0);
}

// upstream: src/enc/alpha_enc.c ApplyFiltersAndEncode
/// On success, `output` receives the buffer of the best trial and
/// `output_size` its size (C: `*output = VP8BitWriterBuf(&best.bw)`).
#[expect(clippy::too_many_arguments, reason = "the C parameters")]
#[expect(
    clippy::cast_possible_wrap,
    reason = "the filter is in 0..WEBP_FILTER_LAST"
)]
#[expect(clippy::if_not_else, reason = "the C condition form")]
fn apply_filters_and_encode(
    alpha: &[u8],
    width: i32,
    height: i32,
    data_size: usize,
    method: i32,
    filter: WEBP_FILTER_TYPE,
    reduce_levels: bool,
    effort_level: i32,
    output: &mut Vec<u8>,
    output_size: &mut usize,
    stats: Option<&mut WebPAuxStats>,
) -> bool {
    let mut ok = true;
    let mut best = FilterTrial::default();
    let mut try_map = get_filter_map(alpha, width, height, filter, effort_level);
    init_filter_trial(&mut best);

    if try_map != FILTER_TRY_NONE {
        let Some(mut filtered_alpha) = safe_bytes(data_size) else {
            return false;
        };

        let mut filter = WEBP_FILTER_TYPE::WEBP_FILTER_NONE.0;
        while ok && try_map != 0 {
            if try_map & 1 != 0 {
                let mut trial = FilterTrial::default();
                ok = encode_alpha_internal(
                    alpha,
                    width,
                    height,
                    method,
                    filter as i32,
                    reduce_levels,
                    effort_level,
                    Some(&mut filtered_alpha),
                    &mut trial,
                );
                if ok && trial.score < best.score {
                    vp8_bit_writer_wipe_out(&mut best.bw);
                    best = trial;
                } else {
                    vp8_bit_writer_wipe_out(&mut trial.bw);
                }
            }
            filter += 1;
            try_map >>= 1;
        }
    } else {
        ok = encode_alpha_internal(
            alpha,
            width,
            height,
            method,
            WEBP_FILTER_TYPE::WEBP_FILTER_NONE.0 as i32,
            reduce_levels,
            effort_level,
            None,
            &mut best,
        );
    }
    if ok {
        if let Some(stats) = stats {
            stats.lossless_features = best.stats.lossless_features;
            stats.histogram_bits = best.stats.histogram_bits;
            stats.transform_bits = best.stats.transform_bits;
            stats.cross_color_transform_bits = best.stats.cross_color_transform_bits;
            stats.cache_bits = best.stats.cache_bits;
            stats.palette_size = best.stats.palette_size;
            stats.lossless_size = best.stats.lossless_size;
            stats.lossless_hdr_size = best.stats.lossless_hdr_size;
            stats.lossless_data_size = best.stats.lossless_data_size;
        }
        *output_size = vp8_bit_writer_size(&best.bw);
        *output = std::mem::take(&mut best.bw.buf);
    } else {
        vp8_bit_writer_wipe_out(&mut best.bw);
    }
    ok
}

// upstream: src/enc/alpha_enc.c CompressAlphaJob
/// The alpha job: what `CompressAlphaJob` reads from the encoder and the
/// picture, and what it writes back. C runs the hook on `enc` and writes
/// `enc->alpha_data`, `enc->alpha_data_size`, `enc->sse[3]`, the picture's error
/// code and its statistics from the worker; the job owns copies of its inputs
/// and holds those results until `VP8EncFinishAlpha` or `VP8EncDeleteAlpha`
/// moves them into the encoder and the picture (ADR 0003, threads). The copy of
/// the alpha plane, the first step of C's `EncodeAlpha`, is made on the calling
/// thread before the job starts.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct VP8EncAlphaJob {
    /// The picture's dimensions.
    pub width: i32,
    pub height: i32,
    /// The alpha plane without its stride (C: `quant_alpha`); `None` when its
    /// allocation failed.
    pub quant_alpha: Option<Vec<u8>>,
    /// The configuration values the job reads (C: `config->method`,
    /// `alpha_quality`, `alpha_compression` and `alpha_filtering`).
    pub method: i32,
    pub alpha_quality: i32,
    pub alpha_compression: i32,
    pub alpha_filtering: i32,
    /// The error the job records (C: on the picture), `VP8_ENC_OK` if none.
    pub error_code: WebPEncodingError,
    /// The picture's statistics as the job writes them (C: `pic->stats`):
    /// a copy taken at launch, `None` when the picture has none.
    pub stats: Option<WebPAuxStats>,
    /// The squared error of the quantized alpha (C: `enc->sse[3]`), when
    /// written.
    pub sse: Option<u64>,
    /// The encoded alpha (C: `enc->alpha_data` and `alpha_data_size`), when the
    /// job succeeded.
    pub alpha_data: Option<(Vec<u8>, u32)>,
    /// True from the run of the hook until the results are moved out.
    pub pending: bool,
}

impl VP8EncAlphaJob {
    /// The job for `pic` under `config`, with the copy of the alpha plane.
    fn new(config: &WebPConfig, pic: &WebPPicture<'_>) -> Self {
        let width = pic.width;
        let height = pic.height;
        let data_size = idx(width * height);
        let a = pic
            .a
            .expect("a picture with transparency has an alpha plane");
        let quant_alpha = safe_bytes(data_size).map(|mut quant_alpha| {
            // Extract alpha data (width x height) from raw_data (stride x height).
            let region = pic
                .yuva
                .region(a.region)
                .expect("the alpha plane lies in a region of its storage");
            webp_copy_plane(
                region,
                a.offset,
                pic.a_stride,
                &mut quant_alpha,
                0,
                width,
                width,
                height,
            );
            quant_alpha
        });
        Self {
            width,
            height,
            quant_alpha,
            method: config.method,
            alpha_quality: config.alpha_quality,
            alpha_compression: config.alpha_compression,
            alpha_filtering: config.alpha_filtering,
            error_code: WebPEncodingError::VP8_ENC_OK,
            stats: pic.stats.as_deref().copied(),
            sse: None,
            alpha_data: None,
            pending: false,
        }
    }

    /// Records `error` unless the job has already recorded one (C:
    /// `WebPEncodingSetError` on the picture).
    fn set_error(&mut self, error: WebPEncodingError) {
        if self.error_code == WebPEncodingError::VP8_ENC_OK {
            self.error_code = error;
        }
    }
}

// upstream: src/enc/alpha_enc.c EncodeAlpha
#[expect(
    clippy::cast_possible_truncation,
    reason = "C converts the coded size to int"
)]
#[expect(
    clippy::cast_possible_wrap,
    reason = "C converts the coded size to int"
)]
fn encode_alpha(
    job: &mut VP8EncAlphaJob,
    quality: i32,
    method: i32,
    filter: WEBP_FILTER_TYPE,
    effort_level: i32,
    output: &mut Vec<u8>,
    output_size: &mut usize,
) -> bool {
    let width = job.width;
    let height = job.height;
    let mut filter = filter;

    let data_size = idx(width * height);
    let mut sse: u64 = 0;
    let mut ok = true;
    let reduce_levels = quality < 100;

    // quick correctness checks
    debug_assert_eq!(
        i64::try_from(data_size).ok(),
        Some(i64::from(width) * i64::from(height))
    ); // as per spec
    debug_assert!(width > 0 && height > 0);
    debug_assert!(
        filter >= WEBP_FILTER_TYPE::WEBP_FILTER_NONE
            && filter <= WEBP_FILTER_TYPE::WEBP_FILTER_FAST
    );

    if !(0..=100).contains(&quality) {
        job.set_error(WebPEncodingError::VP8_ENC_ERROR_INVALID_CONFIGURATION);
        return false;
    }

    if !(ALPHA_NO_COMPRESSION..=ALPHA_LOSSLESS_COMPRESSION).contains(&method) {
        job.set_error(WebPEncodingError::VP8_ENC_ERROR_INVALID_CONFIGURATION);
        return false;
    }

    if method == ALPHA_NO_COMPRESSION {
        // Don't filter, as filtering will make no impact on compressed size.
        filter = WEBP_FILTER_TYPE::WEBP_FILTER_NONE;
    }

    // C allocates quant_alpha here and copies the alpha plane into it; the job
    // received the copy.
    let Some(mut quant_alpha) = job.quant_alpha.take() else {
        job.set_error(WebPEncodingError::VP8_ENC_ERROR_OUT_OF_MEMORY);
        return false;
    };

    if reduce_levels {
        // No Quantization required for 'quality = 100'.
        // 16 alpha levels gives quite a low MSE w.r.t original alpha plane hence
        // mapped to moderate quality 70. Hence Quality:[0, 70] -> Levels:[2, 16]
        // and Quality:]70, 100] -> Levels:]16, 256].
        let alpha_levels = if quality <= 70 {
            2 + quality / 5
        } else {
            16 + (quality - 70) * 8
        };
        ok = quantize_levels(
            Some(&mut quant_alpha),
            width,
            height,
            alpha_levels,
            Some(&mut sse),
        );
    }

    if ok {
        // C: VP8FiltersInit() only assigns portable pointers (ADR 0003).
        ok = apply_filters_and_encode(
            &quant_alpha,
            width,
            height,
            data_size,
            method,
            filter,
            reduce_levels,
            effort_level,
            output,
            output_size,
            job.stats.as_mut(),
        );
        if !ok {
            job.set_error(WebPEncodingError::VP8_ENC_ERROR_OUT_OF_MEMORY); // imprecise
        }
        if let Some(stats) = job.stats.as_mut() {
            // need stats?
            stats.coded_size += *output_size as i32;
            job.sse = Some(sse);
        }
    }

    ok
}

//------------------------------------------------------------------------------
// Main calls

// upstream: src/enc/alpha_enc.c CompressAlphaJob
/// The hook of the alpha worker (C: `CompressAlphaJob(enc, NULL)`).
#[expect(
    clippy::cast_possible_truncation,
    reason = "C keeps the size as uint32_t after its soundness check"
)]
pub fn compress_alpha_job(job: &mut VP8EncAlphaJob, _unused: &mut ()) -> bool {
    job.pending = true;
    let mut alpha_data = Vec::new();
    let mut alpha_size: usize = 0;
    let effort_level = job.method; // maps to [0..6]
    let filter = if job.alpha_filtering == 0 {
        WEBP_FILTER_TYPE::WEBP_FILTER_NONE
    } else if job.alpha_filtering == 1 {
        WEBP_FILTER_TYPE::WEBP_FILTER_FAST
    } else {
        WEBP_FILTER_TYPE::WEBP_FILTER_BEST
    };
    let (quality, compression) = (job.alpha_quality, job.alpha_compression);
    if !encode_alpha(
        job,
        quality,
        compression,
        filter,
        effort_level,
        &mut alpha_data,
        &mut alpha_size,
    ) {
        return false;
    }
    if alpha_size != alpha_size as u32 as usize {
        // Soundness check.
        return false;
    }
    job.alpha_data = Some((alpha_data, alpha_size as u32));
    true
}

/// Moves the results of a job that ran into the encoder and the picture, where
/// C's hook writes them. A job without pending results moves nothing.
fn move_alpha_results(
    job: &mut VP8EncAlphaJob,
    alpha_data: &mut Vec<u8>,
    alpha_data_size: &mut u32,
    sse: &mut [u64; 4],
    pic: &mut WebPPicture<'_>,
) {
    if !job.pending {
        return;
    }
    job.pending = false;
    if job.error_code != WebPEncodingError::VP8_ENC_OK {
        webp_encoding_set_error(pic, job.error_code);
    }
    if let (Some(job_stats), Some(stats)) = (job.stats.as_ref(), pic.stats.as_deref_mut()) {
        stats.lossless_features = job_stats.lossless_features;
        stats.histogram_bits = job_stats.histogram_bits;
        stats.transform_bits = job_stats.transform_bits;
        stats.cross_color_transform_bits = job_stats.cross_color_transform_bits;
        stats.cache_bits = job_stats.cache_bits;
        stats.palette_size = job_stats.palette_size;
        stats.lossless_size = job_stats.lossless_size;
        stats.lossless_hdr_size = job_stats.lossless_hdr_size;
        stats.lossless_data_size = job_stats.lossless_data_size;
        stats.coded_size = job_stats.coded_size;
    }
    if let Some(value) = job.sse.take() {
        sse[3] = value;
    }
    if let Some((data, size)) = job.alpha_data.take() {
        *alpha_data_size = size;
        *alpha_data = data;
    }
}

/// Moves the results of the worker's job (see [`move_alpha_results`]).
fn move_worker_results(enc: &mut VP8Encoder, pic: &mut WebPPicture<'_>) {
    if let Some((job, ())) = enc.alpha_worker.job_data_mut() {
        move_alpha_results(
            job,
            &mut enc.alpha_data,
            &mut enc.alpha_data_size,
            &mut enc.sse,
            pic,
        );
    }
}

// upstream: src/enc/alpha_enc.c VP8EncInitAlpha
/// Initialize alpha compression. C: `WebPInitAlphaProcessing()` first, which
/// only assigns portable pointers (ADR 0003); the worker's job is set when it
/// starts, because it carries the copy of the alpha plane.
pub fn vp8_enc_init_alpha(enc: &mut VP8Encoder, pic: &WebPPicture<'_>) {
    enc.has_alpha = webp_picture_has_transparency(pic);
    enc.alpha_data = Vec::new();
    enc.alpha_data_size = 0;
    if enc.thread_level > 0 {
        enc.alpha_worker = WebPWorker::init();
    }
}

// upstream: src/enc/alpha_enc.c VP8EncStartAlpha
/// Start alpha coding process. Returns `false` on failure, with the picture's
/// error code recorded where C records one.
pub fn vp8_enc_start_alpha(
    enc: &mut VP8Encoder,
    config: &WebPConfig,
    pic: &mut WebPPicture<'_>,
) -> bool {
    if enc.has_alpha {
        if enc.thread_level > 0 {
            // Makes sure worker is good to go.
            if !enc.alpha_worker.reset() {
                webp_encoding_set_error(pic, WebPEncodingError::VP8_ENC_ERROR_OUT_OF_MEMORY);
                return false;
            }
            let job = VP8EncAlphaJob::new(config, pic);
            enc.alpha_worker.set_job(compress_alpha_job, job, ());
            enc.alpha_worker.launch();
            return true;
        }
        // just do the job right away
        let mut job = VP8EncAlphaJob::new(config, pic);
        let ok = compress_alpha_job(&mut job, &mut ());
        move_alpha_results(
            &mut job,
            &mut enc.alpha_data,
            &mut enc.alpha_data_size,
            &mut enc.sse,
            pic,
        );
        return ok;
    }
    true
}

// upstream: src/enc/alpha_enc.c VP8EncFinishAlpha
/// Finalize compressed data.
pub fn vp8_enc_finish_alpha(enc: &mut VP8Encoder, pic: &mut WebPPicture<'_>) -> bool {
    if enc.has_alpha && enc.thread_level > 0 {
        let ok = enc.alpha_worker.sync();
        move_worker_results(enc, pic);
        if !ok {
            return false; // error
        }
    }
    webp_report_progress(pic, enc.percent + 20, Some(&mut enc.percent)).is_ok()
}

// upstream: src/enc/alpha_enc.c VP8EncDeleteAlpha
/// Delete compressed data.
pub fn vp8_enc_delete_alpha(enc: &mut VP8Encoder, pic: &mut WebPPicture<'_>) -> bool {
    let mut ok = true;
    if enc.thread_level > 0 {
        // finish anything left in flight
        ok = enc.alpha_worker.sync();
        move_worker_results(enc, pic);
        // still need to end the worker, even if !ok
        enc.alpha_worker.end();
    }
    enc.alpha_data = Vec::new();
    enc.alpha_data_size = 0;
    enc.has_alpha = false;
    ok
}
