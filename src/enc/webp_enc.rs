// libwebp v1.6.0, src/enc/webp_enc.c:
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
// WebP encoder: main entry point
//
// Author: Skal (pascal.massimino@gmail.com)

//! Translation of `src/enc/webp_enc.c` from libwebp v1.6.0.
//!
//! Work package: WP10 (VP8 encoder core).
//!
//! Exception: WP9 translates `WebPEncodingSetError` and `WebPReportProgress` from `src/enc/webp_enc.c`, because the picture functions and the VP8L encoder report errors and progress through them.
//!
//! Exception: WP13 translates `WebPEncode`, `GetPSNR`, `FinalizePSNR` and `StoreStats` from `src/enc/webp_enc.c`, because `WebPEncode` joins picture conversion, the VP8, VP8L and alpha encoders and the statistics.
//!
//! # Deviations from upstream
//!
//! Changes of representation that the ADRs allow, or behavior that the Rust
//! side cannot reproduce exactly. Each records the reason.
//!
//! 1. **Views in `webp_encode`.** A view (`PlaneStorage::Shared`) cannot be
//!    written. Where `WebPEncode` writes into the picture's planes (the
//!    transparent-area cleanup unless `exact`, `show_compressed`, the
//!    replacement of transparent pixels before a lossless encoding), the view
//!    first gets its own copy of the planes, with the same strides and samples
//!    (`own_shared_yuva`, `own_shared_argb`), so the encoding is
//!    byte-identical; C writes through the view into the source picture, which
//!    the translation leaves untouched. The copy reserves fallibly and records
//!    `VP8_ENC_ERROR_OUT_OF_MEMORY` when the host refuses, an allocation that C
//!    does not make. The C ABI lends caller memory exclusively, never as a
//!    view, so C callers see C's writes.
//! 2. **Encoder context per call.** The encoder functions take
//!    `config: &WebPConfig` and `pic: &mut WebPPicture` where C reads
//!    `enc->config` and `enc->pic`: `init_vp8_encoder(config, picture)`,
//!    `vp8_enc_analyze`, `vp8_enc_loop`, `vp8_enc_token_loop`
//!    (`enc, config, pic`), `vp8_enc_write(enc, pic)`,
//!    `vp8_enc_init_alpha(enc, pic)`, `vp8_enc_start_alpha(enc, config, pic)`,
//!    `vp8_enc_finish_alpha(enc, pic)`, `vp8_enc_delete_alpha(enc, pic)` and
//!    `delete_vp8_encoder(enc, pic)`, which needs the picture because the alpha
//!    worker's results reach it at `Sync`. The iterator functions take the
//!    encoder (`&VP8Encoder` or `&mut`), the picture or the configuration that
//!    they read in C; `VP8IteratorProgress` takes the picture and the encoder's
//!    `percent`, the two things C's
//!    `WebPReportProgress(enc->pic, percent, &enc->percent)` touches.
//! 3. **Return types (ADR 0003, errors).** Functions whose every failure
//!    records an error return `Result` with the recorded code:
//!    `InitVP8Encoder`, `VP8EncAnalyze`, `VP8EncLoop`, `VP8EncTokenLoop`,
//!    `VP8EncWrite`, `VP8IteratorProgress`, `PreLoopInitialize`,
//!    `PostLoopFinalize`, `PutWebPHeaders`, `EmitPartitionsSize`,
//!    `GeneratePartition0`. Functions with a failure that records no code stay
//!    `bool`: `VP8EncStartAlpha`, `VP8EncFinishAlpha`, `VP8EncDeleteAlpha`,
//!    `DeleteVP8Encoder`, `StatLoop`, `RecordTokens`, `VP8EmitTokens`, and the
//!    alpha helpers. `VP8Decimate` returns whether the macroblock is skipped as
//!    `bool`.
//!
//! # Checks not applicable
//!
//! These checks do not apply, or cannot be made, with the reason:
//!
//! 1. **Floating point.** `log10` in `GetPSNR`, the float conversion of
//!    `FinalizePSNR`, the dithering amplitude of `WebPEncode` and every float
//!    of the statistics are compared bit for bit on the development host
//!    (ARM64, Apple clang, `-ffp-contract=off`); equality on other hosts is
//!    recorded, not assumed (PORTING).
//! 2. **`NULL` arguments.** `DeleteVP8Encoder(NULL)` and the other `NULL` cases
//!    exist only at the C ABI (PORTING rule 8).

use crate::dec::common_dec::B_DC_PRED;
use crate::enc::alpha_enc::{vp8_enc_delete_alpha, vp8_enc_init_alpha};
use crate::enc::token_enc::{vp8t_buffer_clear, vp8t_buffer_init};
use crate::enc::tree_enc::vp8_default_probas;
use crate::enc::vp8i_enc::{
    DERROR_C_SIZE, ENC_MAJ_VERSION, ENC_MIN_VERSION, ENC_REV_VERSION, ERROR_DIFFUSION_QUALITY,
    LF_STATS_C_SIZE, VP8EncFilterHeader, VP8EncProba, VP8EncSegmentHeader, VP8Encoder, VP8MBInfo,
    VP8RDLevel, VP8SegmentInfo, VP8TBuffer, score_t,
};
use crate::utils::bit_writer_utils::VP8BitWriter;
use crate::utils::thread_utils::WebPWorker;
use crate::utils::utils::{WEBP_ALIGN_CST, webp_safe_malloc};
use crate::webp::encode::{WebPConfig, WebPEncodingError, WebPPicture};

// upstream: src/enc/webp_enc.c WebPEncodingSetError
/// Records `error` as the picture's error code, unless an earlier error is already
/// recorded: the oldest error takes precedence (C: `WebPEncodingSetError`). C
/// returns 0 in every case. This returns the code that the picture holds
/// afterwards, so that a caller that propagates the failure returns
/// `Err(webp_encoding_set_error(..))` with exactly what C leaves in the field.
pub fn webp_encoding_set_error(
    picture: &mut WebPPicture<'_>,
    error: WebPEncodingError,
) -> WebPEncodingError {
    debug_assert!(error.0 < WebPEncodingError::VP8_ENC_ERROR_LAST.0);
    if picture.error_code == WebPEncodingError::VP8_ENC_OK {
        picture.error_code = error;
    }
    picture.error_code
}

// upstream: src/enc/webp_enc.c WebPReportProgress
/// Reports `percent` to the progress hook when it differs from `*percent_store`,
/// and stores it there first (C: `WebPReportProgress`). A `false` answer from the
/// hook records `VP8_ENC_ERROR_USER_ABORT` and returns that recorded code, which is
/// the failure that C signals with 0. A `NULL` `percent_store` is `None`.
/// # Errors
///
/// Returns the error that the picture records; the same code is in `error_code`.
pub fn webp_report_progress(
    picture: &mut WebPPicture<'_>,
    percent: i32,
    percent_store: Option<&mut i32>,
) -> Result<(), WebPEncodingError> {
    if let Some(store) = percent_store
        && percent != *store
    {
        *store = percent;
        if let Some(hook) = picture.progress_hook.as_mut()
            && !hook(percent)
        {
            // user abort requested
            return Err(webp_encoding_set_error(
                picture,
                WebPEncodingError::VP8_ENC_ERROR_USER_ABORT,
            ));
        }
    }
    Ok(())
}

//------------------------------------------------------------------------------

// upstream: src/enc/webp_enc.c WebPGetEncoderVersion
/// Return the encoder's version number, packed in hexadecimal using 8bits for
/// each of major/minor/revision. E.g: v2.5.7 is 0x020507.
#[must_use]
pub fn webp_get_encoder_version() -> i32 {
    (ENC_MAJ_VERSION << 16) | (ENC_MIN_VERSION << 8) | ENC_REV_VERSION
}

//------------------------------------------------------------------------------
// VP8Encoder
//------------------------------------------------------------------------------

// upstream: src/enc/webp_enc.c ResetSegmentHeader
fn reset_segment_header(enc: &mut VP8Encoder, config: &WebPConfig) {
    let hdr = &mut enc.segment_hdr;
    hdr.num_segments = config.segments;
    hdr.update_map = hdr.num_segments > 1;
    hdr.size = 0;
}

// upstream: src/enc/webp_enc.c ResetFilterHeader
fn reset_filter_header(enc: &mut VP8Encoder) {
    let hdr = &mut enc.filter_hdr;
    hdr.simple = 1;
    hdr.level = 0;
    hdr.sharpness = 0;
    hdr.i4x4_lf_delta = 0;
}

// upstream: src/enc/webp_enc.c ResetBoundaryPredictions
#[expect(clippy::cast_sign_loss, reason = "the plane dimensions are positive")]
fn reset_boundary_predictions(enc: &mut VP8Encoder) {
    // init boundary values once for all
    // Note: actually, initializing the 'preds[]' is only needed for intra4.
    let preds_w = enc.preds_w as usize;
    let top = enc.preds_base() - preds_w; // C: enc->preds - enc->preds_w
    let left = enc.preds_base() - 1; // C: enc->preds - 1
    #[expect(clippy::cast_possible_truncation, reason = "B_DC_PRED is 0")]
    let dc = B_DC_PRED as u8;
    for i in -1..4 * enc.mb_w {
        let i = isize::try_from(i).expect("the index fits in isize");
        enc.preds[top
            .checked_add_signed(i)
            .expect("the top row lies in the plane")] = dc;
    }
    for i in 0..4 * enc.mb_h as usize {
        enc.preds[left + i * preds_w] = dc;
    }
    enc.nz[0] = 0; // constant (C: enc->nz[-1])
}

// Mapping from config->method to coding tools used.
//-------------------+---+---+---+---+---+---+---+
//   Method          | 0 | 1 | 2 | 3 |(4)| 5 | 6 |
//-------------------+---+---+---+---+---+---+---+
// fast probe        | x |   |   | x |   |   |   |
//-------------------+---+---+---+---+---+---+---+
// dynamic proba     | ~ | x | x | x | x | x | x |
//-------------------+---+---+---+---+---+---+---+
// fast mode analysis|[x]|[x]|   |   | x | x | x |
//-------------------+---+---+---+---+---+---+---+
// basic rd-opt      |   |   |   | x | x | x | x |
//-------------------+---+---+---+---+---+---+---+
// disto-refine i4/16| x | x | x |   |   |   |   |
//-------------------+---+---+---+---+---+---+---+
// disto-refine uv   |   | x | x |   |   |   |   |
//-------------------+---+---+---+---+---+---+---+
// rd-opt i4/16      |   |   | ~ | x | x | x | x |
//-------------------+---+---+---+---+---+---+---+
// token buffer (opt)|   |   |   | x | x | x | x |
//-------------------+---+---+---+---+---+---+---+
// Trellis           |   |   |   |   |   | x |Ful|
//-------------------+---+---+---+---+---+---+---+
// full-SNS          |   |   |   |   | x | x | x |
//-------------------+---+---+---+---+---+---+---+

// upstream: src/enc/webp_enc.c MapConfigToTools
#[expect(
    clippy::cast_possible_truncation,
    reason = "C stores the score_t header limit into int"
)]
fn map_config_to_tools(enc: &mut VP8Encoder, config: &WebPConfig) {
    let method = config.method;
    let limit = 100 - config.partition_limit;
    enc.method = method;
    enc.rd_opt_level = if method >= 6 {
        VP8RDLevel::RD_OPT_TRELLIS_ALL
    } else if method >= 5 {
        VP8RDLevel::RD_OPT_TRELLIS
    } else if method >= 3 {
        VP8RDLevel::RD_OPT_BASIC
    } else {
        VP8RDLevel::RD_OPT_NONE
    };
    enc.max_i4_header_bits = 256 * 16 * 16 *                 // upper bound: up to 16bit per 4x4 block
        (limit * limit)
        / (100 * 100); // ... modulated with a quadratic curve.

    // partition0 = 512k max.
    enc.mb_header_limit =
        (256 as score_t * 510 * 8 * 1024 / score_t::from(enc.mb_w * enc.mb_h)) as i32;

    enc.thread_level = config.thread_level;

    enc.do_search = config.target_size > 0 || config.target_psnr > 0.;
    if config.low_memory == 0 {
        // C: DISABLE_TOKEN_BUFFER is not defined.
        enc.use_tokens = enc.rd_opt_level >= VP8RDLevel::RD_OPT_BASIC; // need rd stats
        if enc.use_tokens {
            enc.num_parts = 1; // doesn't work with multi-partition
        }
    }
}

/// `count` copies of `value`, or `None` when the host refuses the reservation,
/// which is where the C allocation fails.
fn try_filled<T: Clone>(count: usize, value: T) -> Option<Vec<T>> {
    let mut buffer = Vec::new();
    buffer.try_reserve_exact(count).ok()?;
    buffer.resize(count, value);
    Some(buffer)
}

// Memory scaling with dimensions:
//  memory (bytes) ~= 2.25 * w + 0.0625 * w * h
//
// Typical memory footprint (614x440 picture)
//              encoder: 22111
//                 info: 4368
//                preds: 17741
//          top samples: 1263
//             non-zero: 175
//             lf-stats: 0
//                total: 45658
// Transient object sizes:
//       VP8EncIterator: 3360
//         VP8ModeScore: 872
//       VP8SegmentInfo: 732
//          VP8EncProba: 18352
//              LFStats: 2048
// Picture size (yuv): 419328

// upstream: src/enc/webp_enc.c InitVP8Encoder
/// Sizes and sets up the encoder for `picture` under `config`. C carves the
/// encoder and its arrays from one block; the Rust code applies the size test of
/// `WebPSafeMalloc` to the C total, including the `WEBP_ALIGN_CST` padding and
/// `sizeof(VP8Encoder)`, and then allocates the arrays separately (ADR 0003,
/// allocation limits). `PRINT_MEMORY_INFO` is not defined.
///
/// # Errors
///
/// Returns `VP8_ENC_ERROR_OUT_OF_MEMORY`, or the error the picture already
/// holds, as recorded on the picture.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "C converts the dimensions to size_t and the page size to int"
)]
#[expect(
    clippy::cast_precision_loss,
    reason = "C multiplies the page size as float"
)]
#[expect(clippy::too_many_lines, reason = "the C function translated whole")]
#[expect(
    clippy::bool_to_int_with_if,
    reason = "the C condition form of the profile"
)]
pub fn init_vp8_encoder(
    config: &WebPConfig,
    picture: &mut WebPPicture<'_>,
) -> Result<Box<VP8Encoder>, WebPEncodingError> {
    let use_filter = config.filter_strength > 0 || config.autofilter > 0;
    let mb_w = (picture.width + 15) >> 4;
    let mb_h = (picture.height + 15) >> 4;
    let preds_w = 4 * mb_w + 1;
    let preds_h = 4 * mb_h + 1;
    let preds_size = (preds_w * preds_h) as usize;
    let top_stride = mb_w * 16;
    let nz_size = (mb_w + 1) as usize * size_of::<u32>() + WEBP_ALIGN_CST;
    let info_size = (mb_w * mb_h) as usize * VP8MBInfo::C_SIZE;
    let samples_size = 2 * top_stride as usize   // top-luma/u/v
        + WEBP_ALIGN_CST; // align all
    let lf_stats_size = if config.autofilter != 0 {
        LF_STATS_C_SIZE + WEBP_ALIGN_CST
    } else {
        0
    };
    let top_derr_size = if config.quality <= ERROR_DIFFUSION_QUALITY as f32 || config.pass > 1 {
        mb_w as usize * DERROR_C_SIZE
    } else {
        0
    };
    let size: u64 = VP8Encoder::C_SIZE as u64   // main struct
        + WEBP_ALIGN_CST as u64                 // cache alignment
        + info_size as u64                      // modes info
        + preds_size as u64                     // prediction modes
        + samples_size as u64                   // top/left samples
        + top_derr_size as u64                  // top diffusion error
        + nz_size as u64                        // coeff context bits
        + lf_stats_size as u64; // autofilter stats

    let arrays = webp_safe_malloc::<()>(size, 1).and_then(|_| {
        Some((
            try_filled((mb_w * mb_h) as usize, VP8MBInfo::default())?,
            try_filled(preds_size, 0_u8)?,
            try_filled((mb_w + 1) as usize, 0_u32)?,
            try_filled(2 * top_stride as usize, 0_u8)?,
            if top_derr_size != 0 {
                Some(try_filled(mb_w as usize, [[0_i8; 2]; 2])?)
            } else {
                None
            },
        ))
    });
    let Some((mb_info, preds, nz, y_top, top_derr)) = arrays else {
        return Err(webp_encoding_set_error(
            picture,
            WebPEncodingError::VP8_ENC_ERROR_OUT_OF_MEMORY,
        ));
    };
    let mut enc = Box::new(VP8Encoder {
        filter_hdr: VP8EncFilterHeader::default(),
        segment_hdr: VP8EncSegmentHeader::default(),
        profile: 0,
        mb_w,
        mb_h,
        preds_w,
        num_parts: 1 << config.partitions,
        bw: VP8BitWriter::default(),
        parts: Default::default(),
        tokens: VP8TBuffer::default(),
        percent: 0,
        has_alpha: false,
        alpha_data: Vec::new(),
        alpha_data_size: 0,
        alpha_worker: WebPWorker::init(),
        dqm: [VP8SegmentInfo::default(); 4],
        base_quant: 0,
        alpha: 0,
        uv_alpha: 0,
        dq_y1_dc: 0,
        dq_y2_dc: 0,
        dq_y2_ac: 0,
        dq_uv_dc: 0,
        dq_uv_ac: 0,
        proba: Box::new(VP8EncProba::default()),
        sse: [0; 4],
        sse_count: 0,
        coded_size: 0,
        residual_bytes: [[0; 4]; 3],
        block_count: [0; 3],
        method: 0,
        rd_opt_level: VP8RDLevel::RD_OPT_NONE,
        max_i4_header_bits: 0,
        mb_header_limit: 0,
        thread_level: 0,
        do_search: false,
        use_tokens: false,
        mb_info,
        preds,
        nz,
        y_top,
        uv_top: top_stride as usize,
        lf_stats: if lf_stats_size != 0 {
            Some(Box::new([[0.; 64]; 4]))
        } else {
            None
        },
        top_derr,
    });

    enc.profile = if use_filter {
        if config.filter_type == 1 { 0 } else { 1 }
    } else {
        2
    };
    enc.percent = 0;

    map_config_to_tools(&mut enc, config);
    // upstream: src/dsp/enc.c VP8EncDspInit (no counterpart: it only assigns portable pointers, ADR 0003)
    vp8_default_probas(&mut enc);
    reset_segment_header(&mut enc, config);
    reset_filter_header(&mut enc);
    reset_boundary_predictions(&mut enc);
    // upstream: src/dsp/cost.c VP8EncDspCostInit (no counterpart: it only assigns portable pointers, ADR 0003)
    vp8_enc_init_alpha(&mut enc, picture);

    // lower quality means smaller output -> we modulate a little the page
    // size based on quality. This is just a crude 1rst-order prediction.
    {
        let scale = 1. + config.quality * 5. / 100.; // in [1,6]
        vp8t_buffer_init(&mut enc.tokens, ((mb_w * mb_h * 4) as f32 * scale) as i32);
    }
    Ok(enc)
}

// upstream: src/enc/webp_enc.c DeleteVP8Encoder
/// Ends the alpha worker, moving its results into the picture as C's worker
/// writes them, and frees the encoder. Returns the result of the alpha worker.
pub fn delete_vp8_encoder(enc: Box<VP8Encoder>, picture: &mut WebPPicture<'_>) -> bool {
    let mut enc = enc;
    let ok = vp8_enc_delete_alpha(&mut enc, picture);
    vp8t_buffer_clear(&mut enc.tokens);
    drop(enc);
    ok
}

//------------------------------------------------------------------------------
// WP13: WebPEncode and the statistics (exception in tools/units.json)

use crate::dec::common_dec::NUM_MB_SEGMENTS;
use crate::enc::alpha_enc::{vp8_enc_finish_alpha, vp8_enc_start_alpha};
use crate::enc::analysis_enc::vp8_enc_analyze;
use crate::enc::config_enc::webp_validate_config;
use crate::enc::frame_enc::{vp8_enc_loop, vp8_enc_token_loop};
use crate::enc::picture_csp_enc::{
    webp_picture_argb_to_yuva_dithered, webp_picture_sharp_argb_to_yuva, webp_picture_yuva_to_argb,
};
use crate::enc::picture_enc::webp_validate_picture;
use crate::enc::picture_tools_enc::{
    webp_cleanup_transparent_area, webp_replace_transparent_pixels,
};
use crate::enc::syntax_enc::{vp8_enc_free_bit_writers, vp8_enc_write};
use crate::enc::vp8l_enc::vp8l_encode_image;
use crate::webp::encode::{WEBP_MAX_DIMENSION, WebPAuxStats, WebPEncCSP};
use crate::webp::types::{PlaneOffset, PlaneStorage};

// upstream: src/enc/webp_enc.c GetPSNR
/// The PSNR of a sum of squared errors `err` over `size` samples, capped at 99
/// (C: `GetPSNR`). `WEBP_DISABLE_STATS` is not defined.
#[expect(
    clippy::cast_precision_loss,
    reason = "C converts the 64-bit counts to double"
)]
fn get_psnr(err: u64, size: u64) -> f64 {
    if err > 0 && size > 0 {
        10. * (255. * 255. * size as f64 / err as f64).log10()
    } else {
        99.
    }
}

// upstream: src/enc/webp_enc.c FinalizePSNR
/// Stores the PSNR of Y, U, V, all three and alpha in `stats` (C: `FinalizePSNR`,
/// which reads `enc->pic->stats`; `StoreStats` passes that non-null pointer).
#[expect(
    clippy::cast_possible_truncation,
    reason = "C stores the double results into float"
)]
fn finalize_psnr(enc: &VP8Encoder, stats: &mut WebPAuxStats) {
    let size = enc.sse_count;
    let sse = &enc.sse;
    stats.psnr[0] = get_psnr(sse[0], size) as f32;
    stats.psnr[1] = get_psnr(sse[1], size / 4) as f32;
    stats.psnr[2] = get_psnr(sse[2], size / 4) as f32;
    stats.psnr[3] = get_psnr(
        sse[0].wrapping_add(sse[1]).wrapping_add(sse[2]),
        size.wrapping_mul(3) / 2,
    ) as f32;
    stats.psnr[4] = get_psnr(sse[3], size) as f32;
}

// upstream: src/enc/webp_enc.c StoreStats
/// Copies the encoder's statistics into the picture's `stats`, when the picture
/// has them (C: `StoreStats`). `WEBP_DISABLE_STATS` is not defined, so the branch
/// that reports 100 percent instead is not translated.
fn store_stats(enc: &VP8Encoder, pic: &mut WebPPicture<'_>) {
    if let Some(stats) = pic.stats.as_deref_mut() {
        for i in 0..NUM_MB_SEGMENTS as usize {
            stats.segment_level[i] = enc.dqm[i].fstrength;
            stats.segment_quant[i] = enc.dqm[i].quant;
            for s in 0..=2 {
                stats.residual_bytes[s][i] = enc.residual_bytes[s][i];
            }
        }
        finalize_psnr(enc, stats);
        stats.coded_size = enc.coded_size;
        for i in 0..3 {
            stats.block_count[i] = enc.block_count[i];
        }
    }
}

/// Rust-only (no C counterpart): the regions of a shared storage, copied one
/// after the other into one owned block, and the position of each region in it.
/// `None` when the host refuses the reservation.
fn owned_copy<T: Copy>(regions: &[&[T]]) -> Option<(Vec<T>, Vec<usize>)> {
    let total = regions.iter().map(|region| region.len()).sum();
    let mut samples = Vec::new();
    samples.try_reserve_exact(total).ok()?;
    let mut bases = Vec::with_capacity(regions.len());
    for region in regions {
        bases.push(samples.len());
        samples.extend_from_slice(region);
    }
    Some((samples, bases))
}

/// Rust-only (no C counterpart): a plane of a shared storage at its position in
/// the owned copy of [`owned_copy`].
fn rebased(plane: Option<PlaneOffset>, bases: &[usize]) -> Option<PlaneOffset> {
    plane.map(|plane| PlaneOffset {
        region: 0,
        offset: bases
            .get(plane.region)
            .expect("the plane lies in a region of its view")
            + plane.offset,
    })
}

/// Rust-only (no C counterpart): gives a view (`PlaneStorage::Shared`) its own
/// copy of the Y, U, V and A planes before `WebPEncode` writes into them, where C
/// writes through the view into the source picture. The copy keeps the strides
/// and the samples, so the encoding is the same; the source stays untouched
/// (`# Deviations from upstream`, item 1, in the module documentation).
fn own_shared_yuva(pic: &mut WebPPicture<'_>) -> Result<(), WebPEncodingError> {
    let PlaneStorage::Shared(regions) = &pic.yuva else {
        return Ok(());
    };
    let Some((samples, bases)) = owned_copy(regions) else {
        return Err(webp_encoding_set_error(
            pic,
            WebPEncodingError::VP8_ENC_ERROR_OUT_OF_MEMORY,
        ));
    };
    pic.y = rebased(pic.y, &bases);
    pic.u = rebased(pic.u, &bases);
    pic.v = rebased(pic.v, &bases);
    pic.a = rebased(pic.a, &bases);
    pic.yuva = PlaneStorage::Internal(samples);
    Ok(())
}

/// Rust-only (no C counterpart): as [`own_shared_yuva`], for the ARGB plane.
fn own_shared_argb(pic: &mut WebPPicture<'_>) -> Result<(), WebPEncodingError> {
    let PlaneStorage::Shared(regions) = &pic.argb_storage else {
        return Ok(());
    };
    let Some((samples, bases)) = owned_copy(regions) else {
        return Err(webp_encoding_set_error(
            pic,
            WebPEncodingError::VP8_ENC_ERROR_OUT_OF_MEMORY,
        ));
    };
    pic.argb = rebased(pic.argb, &bases);
    pic.argb_storage = PlaneStorage::Internal(samples);
    Ok(())
}

// upstream: src/enc/webp_enc.c WebPEncode
/// Main encoding call (C: `WebPEncode`): validates `config` and `pic`, resets the
/// statistics, converts the samples to the form the encoder needs (YUVA for lossy,
/// ARGB for lossless) and writes the compressed data through `pic.writer`. Returns
/// `false` on error, with the error code recorded in `pic.error_code`, as C returns
/// 0.
///
/// C's `NULL` checks of `pic` and `config` belong to the C ABI. The progress hook
/// and the writer run on the calling thread. A view (`PlaneStorage::Shared`) gets
/// its own copy of the planes that `WebPEncode` writes into (the transparent-area
/// cleanup, `show_compressed`); C writes into the source picture there.
pub fn webp_encode(config: &WebPConfig, pic: &mut WebPPicture<'_>) -> bool {
    let mut ok;
    // C: `if (pic == NULL) return 0;` (C ABI).

    pic.error_code = WebPEncodingError::VP8_ENC_OK; // all ok so far
    // C: `config == NULL` records VP8_ENC_ERROR_NULL_PARAMETER (C ABI).
    if !webp_validate_config(config) {
        let _ =
            webp_encoding_set_error(pic, WebPEncodingError::VP8_ENC_ERROR_INVALID_CONFIGURATION);
        return false;
    }
    if webp_validate_picture(pic).is_err() {
        return false;
    }
    if pic.width > WEBP_MAX_DIMENSION || pic.height > WEBP_MAX_DIMENSION {
        let _ = webp_encoding_set_error(pic, WebPEncodingError::VP8_ENC_ERROR_BAD_DIMENSION);
        return false;
    }

    if let Some(stats) = pic.stats.as_deref_mut() {
        *stats = WebPAuxStats::default();
    }

    if config.lossless == 0 {
        if pic.use_argb != 0 || pic.y.is_none() || pic.u.is_none() || pic.v.is_none() {
            // Make sure we have YUVA samples.
            if config.use_sharp_yuv != 0 || (config.preprocessing & 4) != 0 {
                if webp_picture_sharp_argb_to_yuva(pic).is_err() {
                    return false;
                }
            } else {
                let mut dithering = 0.0_f32;
                if (config.preprocessing & 2) != 0 {
                    let x = config.quality / 100.0;
                    let x2 = x * x;
                    // slowly decreasing from max dithering at low quality (q->0)
                    // to 0.5 dithering amplitude at high quality (q->100)
                    dithering = 1.0 + (0.5 - 1.0) * x2 * x2;
                }
                if webp_picture_argb_to_yuva_dithered(pic, WebPEncCSP::WEBP_YUV420, dithering)
                    .is_err()
                {
                    return false;
                }
            }
        }

        // Rust-only: a view gets its own planes before they are written.
        if (config.exact == 0 || config.show_compressed != 0) && own_shared_yuva(pic).is_err() {
            return false;
        }

        if config.exact == 0 {
            webp_cleanup_transparent_area(pic);
        }

        let Ok(mut enc) = init_vp8_encoder(config, pic) else {
            return false; // pic->error is already set.
        };
        // Note: each of the tasks below account for 20% in the progress report.
        ok = vp8_enc_analyze(&mut enc, config, pic).is_ok();

        // Analysis is done, proceed to actual coding.
        ok = ok && vp8_enc_start_alpha(&mut enc, config, pic); // possibly done in parallel
        if enc.use_tokens {
            ok = ok && vp8_enc_token_loop(&mut enc, config, pic).is_ok();
        } else {
            ok = ok && vp8_enc_loop(&mut enc, config, pic).is_ok();
        }
        ok = ok && vp8_enc_finish_alpha(&mut enc, pic);

        ok = ok && vp8_enc_write(&mut enc, pic).is_ok();
        store_stats(&enc, pic);
        if !ok {
            vp8_enc_free_bit_writers(&mut enc);
        }
        ok &= delete_vp8_encoder(enc, pic); // must always be called, even if !ok
    } else {
        // Make sure we have ARGB samples.
        if pic.argb.is_none() && webp_picture_yuva_to_argb(pic).is_err() {
            return false;
        }

        if config.exact == 0 {
            // Rust-only: a view gets its own plane before it is written.
            if pic.use_argb != 0 && own_shared_argb(pic).is_err() {
                return false;
            }
            webp_replace_transparent_pixels(pic, 0x0000_0000);
        }

        ok = vp8l_encode_image(config, pic).is_ok(); // Sets pic->error in case of problem.
    }

    ok
}
