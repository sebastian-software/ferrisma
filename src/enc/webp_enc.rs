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
