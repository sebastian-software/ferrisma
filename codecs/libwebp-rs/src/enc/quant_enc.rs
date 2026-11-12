// libwebp v1.6.0, src/enc/quant_enc.c:
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
//   Quantization
//
// Author: Skal (pascal.massimino@gmail.com)

//! Translation of `src/enc/quant_enc.c` from libwebp v1.6.0.
//!
//! Work package: WP10 (VP8 encoder core).
//!
//! # Deviations from upstream
//!
//! Changes of representation that the ADRs allow, or behavior that the Rust
//! side cannot reproduce exactly. Each records the reason.
//!
//! 1. **Trellis.** For `n == 15`, `TrellisQuantizeBlock` reads
//!    `costs[16][ctx]`, one entry past the remapped table, and never uses the
//!    value; the Rust code stores a placeholder row there. Dead and terminal
//!    nodes are otherwise as in C. The out-of-bounds read lies inside
//!    `VP8EncProba` and is not reported by the sanitizers.
//! 2. **Pointer swaps of the mode search.** `PickBestIntra16` tracks which of
//!    its two scores is the best (C swaps `rd_cur` and `rd_best`); `PickBestUV`
//!    tracks which output buffer holds the best block (C swaps `dst` and
//!    `tmp_dst`). `PickBestIntra4` reconstructs its trials into a local 4-row
//!    block instead of `yuv_p + I4TMP`, a scratch area of the prediction buffer
//!    that nothing else reads, and tracks whether the best block is there or in
//!    `yuv_out2` (C swaps `tmp_dst` and `best_block`).
//!
//! # Checks not applicable
//!
//! These checks do not apply, or cannot be made, with the reason:
//!
//! 1. **Floating point.** `pow` in `QualityToCompression`,
//!    `QualityToJPEGCompression` and `VP8SetSegmentParams`, the `float` pass
//!    statistics of `frame_enc.c`, `log10` in `GetPSNR` of `frame_enc.c`, and
//!    the `double` SSIM sums of the filter search are compared bit for bit on
//!    the development host (ARM64, Apple clang, `-ffp-contract=off`). Rust's
//!    `f64::powf` and `f64::log10` call the platform libm, as C does; equality
//!    on other hosts is recorded, not assumed (`docs/PORTING.md`).

use crate::dec::common_dec::{NUM_BMODES, NUM_MB_SEGMENTS, NUM_PRED_MODES};
use crate::dsp::cost::VP8_ENC_BANDS;
use crate::dsp::dec::vp8_transform_wht;
use crate::dsp::dsp::BPS;
use crate::dsp::enc::{
    vp8_copy4x4, vp8_copy16x8, vp8_enc_pred_chroma8, vp8_enc_pred_luma4, vp8_enc_pred_luma16,
    vp8_enc_quantize_block, vp8_enc_quantize_block_wht, vp8_enc_quantize2_blocks, vp8f_transform,
    vp8f_transform_wht, vp8f_transform2, vp8i_transform, vp8sse4x4, vp8sse16x8, vp8sse16x16,
    vp8t_disto4x4, vp8t_disto16x16,
};
use crate::dsp::quant::{is_flat, is_flat_source16};
use crate::enc::cost_enc::{
    VP8_FIXED_COSTS_I4, VP8_FIXED_COSTS_I16, VP8_FIXED_COSTS_UV, vp8_bit_cost, vp8_get_cost_luma4,
    vp8_get_cost_luma16, vp8_get_cost_uv, vp8_level_cost,
};
use crate::enc::filter_enc::vp8_filter_strength_from_delta;
use crate::enc::iterator_enc::{
    vp8_iterator_nz_to_bytes, vp8_iterator_rotate_i4, vp8_iterator_start_i4, vp8_set_intra_uv_mode,
    vp8_set_intra4_mode, vp8_set_intra16_mode, vp8_set_skip,
};
use crate::enc::vp8i_enc::{
    C8DC8, C8HE8, C8TM8, C8VE8, I4DC4, I4HD4, I4HE4, I4HU4, I4LD4, I4RD4, I4TM4, I4VE4, I4VL4,
    I4VR4, I16DC16, I16HE16, I16TM16, I16VE16, MAX_COST, MAX_LEVEL, ModeRows, QFIX, U_OFF_ENC,
    VP8EncIterator, VP8Encoder, VP8Matrix, VP8ModeScore, VP8RDLevel, VP8SegmentInfo, Y_OFF_ENC,
    bias, quantdiv, score_t,
};
use crate::webp::encode::WebPConfig;

// upstream: src/enc/quant_enc.c DO_TRELLIS_I4
const DO_TRELLIS_I4: bool = true;
// upstream: src/enc/quant_enc.c DO_TRELLIS_I16
/// not a huge gain, but ok at low bitrate.
const DO_TRELLIS_I16: bool = true;
// upstream: src/enc/quant_enc.c DO_TRELLIS_UV
/// disable trellis for UV. Risky. Not worth.
const DO_TRELLIS_UV: bool = false;

// upstream: src/enc/quant_enc.c MID_ALPHA
/// neutral value for susceptibility
const MID_ALPHA: i32 = 64;
// upstream: src/enc/quant_enc.c MIN_ALPHA
/// lowest usable value for susceptibility
const MIN_ALPHA: i32 = 30;
// upstream: src/enc/quant_enc.c MAX_ALPHA
/// higher meaningful value for susceptibility
const MAX_ALPHA: i32 = 100;

// upstream: src/enc/quant_enc.c SNS_TO_DQ
/// Scaling constant between the sns value and the QP power-law modulation.
/// Must be strictly less than 1.
const SNS_TO_DQ: f64 = 0.9;

// number of non-zero coeffs below which we consider the block very flat
// (and apply a penalty to complex predictions)
// upstream: src/enc/quant_enc.c FLATNESS_LIMIT_I16
/// I16 mode (special case)
const FLATNESS_LIMIT_I16: i32 = 0;
// upstream: src/enc/quant_enc.c FLATNESS_LIMIT_I4
const FLATNESS_LIMIT_I4: i32 = 3;
// upstream: src/enc/quant_enc.c FLATNESS_LIMIT_UV
const FLATNESS_LIMIT_UV: i32 = 2;
// upstream: src/enc/quant_enc.c FLATNESS_PENALTY
/// roughly ~1bit per block
const FLATNESS_PENALTY: i32 = 140;

// upstream: src/enc/quant_enc.c MULT_8B
const fn mult_8b(a: i32, b: i32) -> i32 {
    (a * b + 128) >> 8
}

// upstream: src/enc/quant_enc.c RD_DISTO_MULT
/// distortion multiplier (equivalent of lambda)
const RD_DISTO_MULT: i32 = 256;

/// `BPS` as an index stride.
const BPS_U: usize = BPS as usize;

/// Converts a non-negative C `int` to an index.
#[expect(clippy::cast_sign_loss, reason = "the value is non-negative")]
const fn idx(v: i32) -> usize {
    v as usize
}

//------------------------------------------------------------------------------

// upstream: src/enc/quant_enc.c clip
fn clip(v: i32, m: i32, mm: i32) -> i32 {
    if v < m {
        m
    } else if v > mm {
        mm
    } else {
        v
    }
}

// upstream: src/enc/quant_enc.c kZigzag
static ZIGZAG: [u8; 16] = [0, 1, 4, 8, 5, 2, 3, 6, 9, 12, 13, 10, 7, 11, 14, 15];

// upstream: src/enc/quant_enc.c kDcTable
static DC_TABLE: [u8; 128] = [
    4, 5, 6, 7, 8, 9, 10, 10, 11, 12, 13, 14, 15, 16, 17, 17, 18, 19, 20, 20, 21, 21, 22, 22, 23,
    23, 24, 25, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 37, 38, 39, 40, 41, 42, 43, 44,
    45, 46, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67,
    68, 69, 70, 71, 72, 73, 74, 75, 76, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 91,
    93, 95, 96, 98, 100, 101, 102, 104, 106, 108, 110, 112, 114, 116, 118, 122, 124, 126, 128, 130,
    132, 134, 136, 138, 140, 143, 145, 148, 151, 154, 157,
];

// upstream: src/enc/quant_enc.c kAcTable
static AC_TABLE: [u16; 128] = [
    4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28,
    29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52,
    53, 54, 55, 56, 57, 58, 60, 62, 64, 66, 68, 70, 72, 74, 76, 78, 80, 82, 84, 86, 88, 90, 92, 94,
    96, 98, 100, 102, 104, 106, 108, 110, 112, 114, 116, 119, 122, 125, 128, 131, 134, 137, 140,
    143, 146, 149, 152, 155, 158, 161, 164, 167, 170, 173, 177, 181, 185, 189, 193, 197, 201, 205,
    209, 213, 217, 221, 225, 229, 234, 239, 245, 249, 254, 259, 264, 269, 274, 279, 284,
];

// upstream: src/enc/quant_enc.c kAcTable2
static AC_TABLE2: [u16; 128] = [
    8, 8, 9, 10, 12, 13, 15, 17, 18, 20, 21, 23, 24, 26, 27, 29, 31, 32, 34, 35, 37, 38, 40, 41,
    43, 44, 46, 48, 49, 51, 52, 54, 55, 57, 58, 60, 62, 63, 65, 66, 68, 69, 71, 72, 74, 75, 77, 79,
    80, 82, 83, 85, 86, 88, 89, 93, 96, 99, 102, 105, 108, 111, 114, 117, 120, 124, 127, 130, 133,
    136, 139, 142, 145, 148, 151, 155, 158, 161, 164, 167, 170, 173, 176, 179, 184, 189, 193, 198,
    203, 207, 212, 217, 221, 226, 230, 235, 240, 244, 249, 254, 258, 263, 268, 274, 280, 286, 292,
    299, 305, 311, 317, 323, 330, 336, 342, 348, 354, 362, 370, 379, 385, 393, 401, 409, 416, 424,
    432, 440,
];

// upstream: src/enc/quant_enc.c kBiasMatrices
/// [luma-ac,luma-dc,chroma][dc,ac]
static BIAS_MATRICES: [[u8; 2]; 3] = [[96, 110], [96, 108], [110, 115]];

// upstream: src/enc/quant_enc.c SHARPEN_BITS
/// Sharpening by (slightly) raising the hi-frequency coeffs. Hack-ish but
/// helpful for mid-bitrate range. Use with care. Number of descaling bits for
/// sharpening bias.
const SHARPEN_BITS: i32 = 11;
// upstream: src/enc/quant_enc.c kFreqSharpening
static FREQ_SHARPENING: [u8; 16] = [
    0, 30, 60, 90, 30, 60, 90, 90, 60, 90, 90, 90, 90, 90, 90, 90,
];

//------------------------------------------------------------------------------
// Initialize quantization parameters in VP8Matrix

// upstream: src/enc/quant_enc.c ExpandMatrix
/// Returns the average quantizer.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "C stores the int results into the uint16_t and uint32_t fields"
)]
#[allow(
    clippy::needless_range_loop,
    reason = "the C index loops over the matrix arrays"
)]
fn expand_matrix(m: &mut VP8Matrix, r#type: usize) -> i32 {
    for i in 0..2 {
        let is_ac_coeff = usize::from(i > 0);
        let bias_value = i32::from(BIAS_MATRICES[r#type][is_ac_coeff]);
        m.iq[i] = ((1 << QFIX) / i32::from(m.q[i])) as u16;
        m.bias[i] = bias(bias_value) as u32;
        // zthresh is the exact value such that QUANTDIV(coeff, iQ, B) is:
        //   * zero if coeff <= zthresh
        //   * non-zero if coeff > zthresh
        m.zthresh[i] = ((1_u32 << QFIX) - 1 - m.bias[i]) / u32::from(m.iq[i]);
    }
    for i in 2..16 {
        m.q[i] = m.q[1];
        m.iq[i] = m.iq[1];
        m.bias[i] = m.bias[1];
        m.zthresh[i] = m.zthresh[1];
    }
    let mut sum = 0;
    for i in 0..16 {
        if r#type == 0 {
            // we only use sharpening for AC luma coeffs
            m.sharpen[i] =
                ((i32::from(FREQ_SHARPENING[i]) * i32::from(m.q[i])) >> SHARPEN_BITS) as u16;
        } else {
            m.sharpen[i] = 0;
        }
        sum += i32::from(m.q[i]);
    }
    (sum + 8) >> 4
}

// upstream: src/enc/quant_enc.c CheckLambdaValue
fn check_lambda_value(v: &mut i32) {
    if *v < 1 {
        *v = 1;
    }
}

// upstream: src/enc/quant_enc.c SetupMatrices
#[expect(
    clippy::cast_possible_truncation,
    reason = "C stores the table values into the uint16_t steps"
)]
#[expect(
    clippy::cast_sign_loss,
    reason = "C stores the doubled DC step into uint16_t"
)]
#[expect(clippy::similar_names, reason = "the C names of the quantizer offsets")]
fn setup_matrices(enc: &mut VP8Encoder, config: &WebPConfig) {
    let tlambda_scale = if enc.method >= 4 {
        config.sns_strength
    } else {
        0
    };
    let num_segments = enc.segment_hdr.num_segments;
    for i in 0..idx(num_segments) {
        let (dq_y1_dc, dq_y2_dc, dq_y2_ac, dq_uv_dc, dq_uv_ac) = (
            enc.dq_y1_dc,
            enc.dq_y2_dc,
            enc.dq_y2_ac,
            enc.dq_uv_dc,
            enc.dq_uv_ac,
        );
        let m = &mut enc.dqm[i];
        let q = m.quant;
        m.y1.q[0] = u16::from(DC_TABLE[idx(clip(q + dq_y1_dc, 0, 127))]);
        m.y1.q[1] = AC_TABLE[idx(clip(q, 0, 127))];

        m.y2.q[0] = (i32::from(DC_TABLE[idx(clip(q + dq_y2_dc, 0, 127))]) * 2) as u16;
        m.y2.q[1] = AC_TABLE2[idx(clip(q + dq_y2_ac, 0, 127))];

        m.uv.q[0] = u16::from(DC_TABLE[idx(clip(q + dq_uv_dc, 0, 117))]);
        m.uv.q[1] = AC_TABLE[idx(clip(q + dq_uv_ac, 0, 127))];

        let q_i4 = expand_matrix(&mut m.y1, 0);
        let q_i16 = expand_matrix(&mut m.y2, 1);
        let q_uv = expand_matrix(&mut m.uv, 2);

        m.lambda_i4 = (3 * q_i4 * q_i4) >> 7;
        m.lambda_i16 = 3 * q_i16 * q_i16;
        m.lambda_uv = (3 * q_uv * q_uv) >> 6;
        m.lambda_mode = (q_i4 * q_i4) >> 7;
        m.lambda_trellis_i4 = (7 * q_i4 * q_i4) >> 3;
        m.lambda_trellis_i16 = (q_i16 * q_i16) >> 2;
        m.lambda_trellis_uv = (q_uv * q_uv) << 1;
        m.tlambda = (tlambda_scale * q_i4) >> 5;

        // none of these constants should be < 1
        check_lambda_value(&mut m.lambda_i4);
        check_lambda_value(&mut m.lambda_i16);
        check_lambda_value(&mut m.lambda_uv);
        check_lambda_value(&mut m.lambda_mode);
        check_lambda_value(&mut m.lambda_trellis_i4);
        check_lambda_value(&mut m.lambda_trellis_i16);
        check_lambda_value(&mut m.lambda_trellis_uv);
        check_lambda_value(&mut m.tlambda);

        m.min_disto = 20 * i32::from(m.y1.q[0]); // quantization-aware min disto
        m.max_edge = 0;

        m.i4_penalty = score_t::from(1000 * q_i4 * q_i4);
    }
}

//------------------------------------------------------------------------------
// Initialize filtering parameters

// upstream: src/enc/quant_enc.c FSTRENGTH_CUTOFF
/// Very small filter-strength values have close to no visual effect. So we can
/// save a little decoding-CPU by turning filtering off for these.
const FSTRENGTH_CUTOFF: i32 = 2;

// upstream: src/enc/quant_enc.c SetupFilterStrength
fn setup_filter_strength(enc: &mut VP8Encoder, config: &WebPConfig) {
    // level0 is in [0..500]. Using '-f 50' as filter_strength is mid-filtering.
    let level0 = 5 * config.filter_strength;
    for i in 0..idx(NUM_MB_SEGMENTS) {
        let sharpness = enc.filter_hdr.sharpness;
        let m = &mut enc.dqm[i];
        // We focus on the quantization of AC coeffs.
        let qstep = i32::from(AC_TABLE[idx(clip(m.quant, 0, 127))]) >> 2;
        let base_strength = vp8_filter_strength_from_delta(sharpness, qstep);
        // Segments with lower complexity ('beta') will be less filtered.
        let f = base_strength * level0 / (256 + m.beta);
        m.fstrength = if f < FSTRENGTH_CUTOFF {
            0
        } else if f > 63 {
            63
        } else {
            f
        };
    }
    // We record the initial strength (mainly for the case of 1-segment only).
    enc.filter_hdr.level = enc.dqm[0].fstrength;
    enc.filter_hdr.simple = i32::from(config.filter_type == 0);
    enc.filter_hdr.sharpness = config.filter_sharpness;
}

//------------------------------------------------------------------------------

// Note: if you change the values below, remember that the max range
// allowed by the syntax for DQ_UV is [-16,16].
// upstream: src/enc/quant_enc.c MAX_DQ_UV
const MAX_DQ_UV: i32 = 6;
// upstream: src/enc/quant_enc.c MIN_DQ_UV
const MIN_DQ_UV: i32 = -4;

// upstream: src/enc/quant_enc.c QualityToCompression
/// We want to emulate jpeg-like behaviour where the expected "good" quality is
/// around q=75. Internally, our "good" middle is around c=50. So we map
/// accordingly using linear piece-wise function.
fn quality_to_compression(c: f64) -> f64 {
    let linear_c = if c < 0.75 { c * (2. / 3.) } else { 2. * c - 1. };
    // The file size roughly scales as pow(quantizer, 3.). Actually, the
    // exponent is somewhere between 2.8 and 3.2, but we're mostly interested
    // in the mid-quant range. So we scale the compressibility inversely to
    // this power-law: quant ~= compression ^ 1/3. This law holds well for
    // low quant. Finer modeling for high-quant would make use of kAcTable[]
    // more explicitly.
    linear_c.powf(1. / 3.)
}

// upstream: src/enc/quant_enc.c QualityToJPEGCompression
fn quality_to_jpeg_compression(c: f64, alpha: f64) -> f64 {
    // We map the complexity 'alpha' and quality setting 'c' to a compression
    // exponent empirically matched to the compression curve of libjpeg6b.
    // On average, the WebP output size will be roughly similar to that of a
    // JPEG file compressed with same quality factor.
    let amin = 0.30;
    let amax = 0.85;
    let exp_min = 0.4;
    let exp_max = 0.9;
    let slope = (exp_min - exp_max) / (amax - amin);
    // Linearly interpolate 'expn' from exp_min to exp_max
    // in the [amin, amax] range.
    let expn = if alpha > amax {
        exp_min
    } else if alpha < amin {
        exp_max
    } else {
        exp_max + slope * (alpha - amin)
    };
    c.powf(expn)
}

// upstream: src/enc/quant_enc.c SegmentsAreEquivalent
fn segments_are_equivalent(s1: &VP8SegmentInfo, s2: &VP8SegmentInfo) -> bool {
    s1.quant == s2.quant && s1.fstrength == s2.fstrength
}

// upstream: src/enc/quant_enc.c SimplifySegments
#[expect(clippy::cast_sign_loss, reason = "the remapped segment is in 0..4")]
fn simplify_segments(enc: &mut VP8Encoder) {
    let mut map: [i32; NUM_MB_SEGMENTS as usize] = [0, 1, 2, 3];
    // 'num_segments' is previously validated and <= NUM_MB_SEGMENTS, but an
    // explicit check is needed to avoid a spurious warning about 'i' exceeding
    // array bounds of 'dqm' with some compilers (noticed with gcc-4.9).
    let num_segments = if enc.segment_hdr.num_segments < NUM_MB_SEGMENTS {
        enc.segment_hdr.num_segments
    } else {
        NUM_MB_SEGMENTS
    };
    let mut num_final_segments = 1;
    for s1 in 1..num_segments {
        // find similar segments
        let mut found = false;
        // check if we already have similar segment
        let mut s2 = 0;
        while s2 < num_final_segments {
            if segments_are_equivalent(&enc.dqm[idx(s1)], &enc.dqm[idx(s2)]) {
                found = true;
                break;
            }
            s2 += 1;
        }
        map[idx(s1)] = s2;
        if !found {
            if num_final_segments != s1 {
                enc.dqm[idx(num_final_segments)] = enc.dqm[idx(s1)];
            }
            num_final_segments += 1;
        }
    }
    if num_final_segments < num_segments {
        // Remap
        let mut i = enc.mb_w * enc.mb_h;
        while i > 0 {
            i -= 1;
            let mb = &mut enc.mb_info[idx(i)];
            mb.segment = (map[usize::from(mb.segment)] & 3) as u8;
        }
        enc.segment_hdr.num_segments = num_final_segments;
        // Replicate the trailing segment infos (it's mostly cosmetics)
        for i in num_final_segments..num_segments {
            enc.dqm[idx(i)] = enc.dqm[idx(num_final_segments - 1)];
        }
    }
}

// upstream: src/enc/quant_enc.c VP8SetSegmentParams
/// Sets up segment's quantization values, `base_quant` and filter strengths.
#[expect(
    clippy::cast_possible_truncation,
    reason = "C converts the double quantizer to int, in range for every quality"
)]
#[expect(clippy::similar_names, reason = "the C names dq_uv_ac and dq_uv_dc")]
#[allow(clippy::needless_range_loop, reason = "the C index loops")]
pub fn vp8_set_segment_params(enc: &mut VP8Encoder, config: &WebPConfig, quality: f32) {
    let num_segments = enc.segment_hdr.num_segments;
    let amp = SNS_TO_DQ * f64::from(config.sns_strength) / 100. / 128.;
    let q_value = f64::from(quality) / 100.;
    let c_base = if config.emulate_jpeg_size != 0 {
        quality_to_jpeg_compression(q_value, f64::from(enc.alpha) / 255.)
    } else {
        quality_to_compression(q_value)
    };
    for i in 0..idx(num_segments) {
        // We modulate the base coefficient to accommodate for the quantization
        // susceptibility and allow denser segments to be quantized more.
        let expn = 1. - amp * f64::from(enc.dqm[i].alpha);
        let c = c_base.powf(expn);
        let q = (127. * (1. - c)) as i32;
        debug_assert!(expn > 0.);
        enc.dqm[i].quant = clip(q, 0, 127);
    }

    // purely indicative in the bitstream (except for the 1-segment case)
    enc.base_quant = enc.dqm[0].quant;

    // fill-in values for the unused segments (required by the syntax)
    for i in idx(num_segments)..idx(NUM_MB_SEGMENTS) {
        enc.dqm[i].quant = enc.base_quant;
    }

    // uv_alpha is normally spread around ~60. The useful range is
    // typically ~30 (quite bad) to ~100 (ok to decimate UV more).
    // We map it to the safe maximal range of MAX/MIN_DQ_UV for dq_uv.
    let mut dq_uv_ac =
        (enc.uv_alpha - MID_ALPHA) * (MAX_DQ_UV - MIN_DQ_UV) / (MAX_ALPHA - MIN_ALPHA);
    // we rescale by the user-defined strength of adaptation
    dq_uv_ac = dq_uv_ac * config.sns_strength / 100;
    // and make it safe.
    dq_uv_ac = clip(dq_uv_ac, MIN_DQ_UV, MAX_DQ_UV);
    // We also boost the dc-uv-quant a little, based on sns-strength, since
    // U/V channels are quite more reactive to high quants (flat DC-blocks
    // tend to appear, and are unpleasant).
    let mut dq_uv_dc = -4 * config.sns_strength / 100;
    dq_uv_dc = clip(dq_uv_dc, -15, 15); // 4bit-signed max allowed

    enc.dq_y1_dc = 0; // TODO(skal): dq-lum
    enc.dq_y2_dc = 0;
    enc.dq_y2_ac = 0;
    enc.dq_uv_dc = dq_uv_dc;
    enc.dq_uv_ac = dq_uv_ac;

    setup_filter_strength(enc, config); // initialize segments' filtering, eventually

    if num_segments > 1 {
        simplify_segments(enc);
    }

    setup_matrices(enc, config); // finalize quantization matrices
}

//------------------------------------------------------------------------------
// Form the predictions in cache

/// A block offset of the cache layout as the `uint16_t` the C tables store.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "the offsets are in 0..PRED_SIZE_ENC"
)]
const fn off(v: i32) -> u16 {
    v as u16
}

// upstream: src/enc/quant_enc.c VP8I16ModeOffsets
/// Must be ordered using {`DC_PRED`, `TM_PRED`, `V_PRED`, `H_PRED`} as index.
pub static VP8I16_MODE_OFFSETS: [u16; 4] = [off(I16DC16), off(I16TM16), off(I16VE16), off(I16HE16)];
// upstream: src/enc/quant_enc.c VP8UVModeOffsets
pub static VP8UV_MODE_OFFSETS: [u16; 4] = [off(C8DC8), off(C8TM8), off(C8VE8), off(C8HE8)];

// upstream: src/enc/quant_enc.c VP8I4ModeOffsets
/// Must be indexed using {`B_DC_PRED` -> `B_HU_PRED`} as index.
static VP8I4_MODE_OFFSETS: [u16; NUM_BMODES as usize] = [
    off(I4DC4),
    off(I4TM4),
    off(I4VE4),
    off(I4HE4),
    off(I4RD4),
    off(I4VR4),
    off(I4LD4),
    off(I4VL4),
    off(I4HD4),
    off(I4HU4),
];

// upstream: src/enc/quant_enc.c VP8MakeLuma16Preds
/// Form all the four Intra16x16 predictions in the `yuv_p` cache.
pub fn vp8_make_luma16_preds(it: &mut VP8EncIterator, enc: &VP8Encoder) {
    let VP8EncIterator {
        x,
        y,
        yuv_p,
        yuv_left_mem,
        y_left,
        y_top,
        tmp_32,
        top_in_tmp_32,
        ..
    } = it;
    let left = if *x != 0 {
        Some((&yuv_left_mem[..], *y_left))
    } else {
        None
    };
    let top_buffer: &[u8] = if *top_in_tmp_32 { tmp_32 } else { &enc.y_top };
    let top = if *y != 0 {
        Some(&top_buffer[*y_top..])
    } else {
        None
    };
    vp8_enc_pred_luma16(yuv_p, left, top);
}

// upstream: src/enc/quant_enc.c VP8MakeChroma8Preds
/// Form all the four Chroma8x8 predictions in the `yuv_p` cache.
pub fn vp8_make_chroma8_preds(it: &mut VP8EncIterator, enc: &VP8Encoder) {
    let VP8EncIterator {
        x,
        y,
        yuv_p,
        yuv_left_mem,
        u_left,
        uv_top,
        tmp_32,
        top_in_tmp_32,
        ..
    } = it;
    let left = if *x != 0 {
        Some((&yuv_left_mem[..], *u_left))
    } else {
        None
    };
    let top_buffer: &[u8] = if *top_in_tmp_32 { tmp_32 } else { &enc.y_top };
    let top = if *y != 0 {
        Some(&top_buffer[*uv_top..])
    } else {
        None
    };
    vp8_enc_pred_chroma8(yuv_p, left, top);
}

// upstream: src/enc/quant_enc.c MakeIntra4Preds
/// Form all the ten Intra4x4 predictions in the `yuv_p` cache for the 4x4
/// block `it->i4`.
fn make_intra4_preds(it: &mut VP8EncIterator) {
    vp8_enc_pred_luma4(&mut it.yuv_p, &it.i4_boundary, it.i4_top);
}

//------------------------------------------------------------------------------
// Quantize

// Layout:
// +----+----+
// |YYYY|UUVV| 0
// |YYYY|UUVV| 4
// |YYYY|....| 8
// |YYYY|....| 12
// +----+----+

// upstream: src/enc/quant_enc.c VP8Scan
/// Luma.
pub static VP8_SCAN: [u16; 16] = [
    0,
    4,
    8,
    12,
    off(4 * BPS),
    off(4 + 4 * BPS),
    off(8 + 4 * BPS),
    off(12 + 4 * BPS),
    off(8 * BPS),
    off(4 + 8 * BPS),
    off(8 + 8 * BPS),
    off(12 + 8 * BPS),
    off(12 * BPS),
    off(4 + 12 * BPS),
    off(8 + 12 * BPS),
    off(12 + 12 * BPS),
];

// upstream: src/enc/quant_enc.c VP8ScanUV
static VP8_SCAN_UV: [u16; 4 + 4] = [
    0,
    4,
    off(4 * BPS),
    off(4 + 4 * BPS), // U
    8,
    12,
    off(8 + 4 * BPS),
    off(12 + 4 * BPS), // V
];

//------------------------------------------------------------------------------
// Distortion measurement

// upstream: src/enc/quant_enc.c kWeightY
static WEIGHT_Y: [u16; 16] = [38, 32, 20, 9, 32, 28, 17, 7, 20, 17, 10, 4, 9, 7, 4, 2];

// upstream: src/enc/quant_enc.c kWeightTrellis
/// `USE_TDISTO` is 1.
static WEIGHT_TRELLIS: [u16; 16] = [30, 27, 19, 11, 27, 24, 17, 10, 19, 17, 12, 8, 11, 10, 8, 6];

// upstream: src/enc/quant_enc.c InitScore
/// Init/Copy the common fields in score.
fn init_score(rd: &mut VP8ModeScore) {
    rd.d = 0;
    rd.sd = 0;
    rd.r = 0;
    rd.h = 0;
    rd.nz = 0;
    rd.score = MAX_COST;
}

// upstream: src/enc/quant_enc.c CopyScore
fn copy_score(dst: &mut VP8ModeScore, src: &VP8ModeScore) {
    dst.d = src.d;
    dst.sd = src.sd;
    dst.r = src.r;
    dst.h = src.h;
    dst.nz = src.nz; // note that nz is not accumulated, but just copied.
    dst.score = src.score;
}

// upstream: src/enc/quant_enc.c AddScore
fn add_score(dst: &mut VP8ModeScore, src: &VP8ModeScore) {
    dst.d += src.d;
    dst.sd += src.sd;
    dst.r += src.r;
    dst.h += src.h;
    dst.nz |= src.nz; // here, new nz bits are accumulated.
    dst.score += src.score;
}

//------------------------------------------------------------------------------
// Performs trellis-optimized quantization.

// upstream: src/enc/quant_enc.c Node
/// Trellis node.
#[derive(Clone, Copy, Debug, Default)]
struct Node {
    /// best previous node
    prev: i8,
    /// sign of `coeff_i`
    sign: i8,
    /// level
    level: i16,
}

// upstream: src/enc/quant_enc.c ScoreState
/// Score state.
#[derive(Clone, Copy, Debug)]
struct ScoreState<'a> {
    /// partial RD score
    score: score_t,
    /// shortcut to cost tables
    costs: &'a [u16; crate::enc::vp8i_enc::MAX_VARIABLE_LEVEL as usize + 1],
}

// If a coefficient was quantized to a value Q (using a neutral bias),
// we test all alternate possibilities between [Q-MIN_DELTA, Q+MAX_DELTA]
// We don't test negative values though.
// upstream: src/enc/quant_enc.c MIN_DELTA
/// how much lower level to try
const MIN_DELTA: i32 = 0;
// upstream: src/enc/quant_enc.c MAX_DELTA
/// how much higher
const MAX_DELTA: i32 = 1;
// upstream: src/enc/quant_enc.c NUM_NODES
const NUM_NODES: usize = (MIN_DELTA + 1 + MAX_DELTA) as usize;

// upstream: src/enc/quant_enc.c NODE
/// The index of the node or score state of level offset `l` (C: the macros
/// `NODE` and `SCORE_STATE`, which add `MIN_DELTA`).
const fn node(l: i32) -> usize {
    idx(l + MIN_DELTA)
}

// upstream: src/enc/quant_enc.c SetRDScore
fn set_rd_score(lambda: i32, rd: &mut VP8ModeScore) {
    rd.score =
        (rd.r + rd.h) * score_t::from(lambda) + score_t::from(RD_DISTO_MULT) * (rd.d + rd.sd);
}

// upstream: src/enc/quant_enc.c RDScoreTrellis
fn rd_score_trellis(lambda: i32, rate: score_t, distortion: score_t) -> score_t {
    rate * score_t::from(lambda) + score_t::from(RD_DISTO_MULT) * distortion
}

// Coefficient type.
// upstream: src/enc/quant_enc.c TYPE_I16_AC
const TYPE_I16_AC: i32 = 0;
// upstream: src/enc/quant_enc.c TYPE_I16_DC
#[expect(dead_code, reason = "the C enumerator is unused in the C source too")]
const TYPE_I16_DC: i32 = 1;
// upstream: src/enc/quant_enc.c TYPE_CHROMA_A
const TYPE_CHROMA_A: i32 = 2;
// upstream: src/enc/quant_enc.c TYPE_I4_AC
const TYPE_I4_AC: i32 = 3;

// upstream: src/enc/quant_enc.c TrellisQuantizeBlock
#[expect(clippy::too_many_lines, reason = "the C function translated whole")]
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    reason = "C converts between int, int8_t, int16_t and uint32_t at these points"
)]
fn trellis_quantize_block(
    enc: &VP8Encoder,
    input: &mut [i16],
    out: &mut [i16],
    ctx0: i32,
    coeff_type: i32,
    mtx: &VP8Matrix,
    lambda: i32,
) -> i32 {
    let probas = &enc.proba.coeffs[idx(coeff_type)];
    let costs = enc.proba.costs(idx(coeff_type));
    let first = i32::from(coeff_type == TYPE_I16_AC);
    let mut nodes = [[Node::default(); NUM_NODES]; 16];
    let placeholder = ScoreState {
        score: 0,
        costs: costs.row(idx(first), idx(ctx0)),
    };
    let mut score_states = [[placeholder; NUM_NODES]; 2];
    // C: ss_cur = &SCORE_STATE(0, MIN_DELTA), ss_prev = &SCORE_STATE(1, MIN_DELTA)
    let mut ss_cur: usize = 0;
    let mut ss_prev: usize = 1;
    let mut best_path: [i32; 3] = [-1, -1, -1]; // store best-last/best-level/best-previous
    let mut best_score: score_t;
    let mut last: i32;

    {
        let thresh = i32::from(mtx.q[1]) * i32::from(mtx.q[1]) / 4;
        let last_proba = probas[usize::from(VP8_ENC_BANDS[idx(first)])][idx(ctx0)][0];

        // compute the position of the last interesting coefficient
        last = first - 1;
        let mut n = 15;
        while n >= first {
            let j = usize::from(ZIGZAG[idx(n)]);
            let err = i32::from(input[j]) * i32::from(input[j]);
            if err > thresh {
                last = n;
                break;
            }
            n -= 1;
        }
        // we don't need to go inspect up to n = 16 coeffs. We can just go up
        // to last + 1 (inclusive) without losing much.
        if last < 15 {
            last += 1;
        }

        // compute 'skip' score. This is the max score one can do.
        let cost = score_t::from(vp8_bit_cost(false, last_proba));
        best_score = rd_score_trellis(lambda, cost, 0);

        // initialize source node.
        for m in -MIN_DELTA..=MAX_DELTA {
            let rate = if ctx0 == 0 {
                score_t::from(vp8_bit_cost(true, last_proba))
            } else {
                0
            };
            score_states[ss_cur][node(m)].score = rd_score_trellis(lambda, rate, 0);
            score_states[ss_cur][node(m)].costs = costs.row(idx(first), idx(ctx0));
        }
    }

    // traverse trellis.
    for n in first..=last {
        let j = usize::from(ZIGZAG[idx(n)]);
        let q = u32::from(mtx.q[j]);
        let i_q = u32::from(mtx.iq[j]);
        let b = bias(0x00) as u32; // neutral bias
        // note: it's important to take sign of the _original_ coeff,
        // so we don't have to consider level < 0 afterward.
        let sign = input[j] < 0;
        let coeff0 = (if sign {
            -i32::from(input[j])
        } else {
            i32::from(input[j])
        } + i32::from(mtx.sharpen[j])) as u32;
        let mut level0 = quantdiv(coeff0, i_q, b);
        let mut thresh_level = quantdiv(coeff0, i_q, bias(0x80) as u32);
        if thresh_level > MAX_LEVEL {
            thresh_level = MAX_LEVEL;
        }
        if level0 > MAX_LEVEL {
            level0 = MAX_LEVEL;
        }

        {
            // Swap current and previous score states
            std::mem::swap(&mut ss_cur, &mut ss_prev);
        }

        // test all alternate level values around level0.
        for m in -MIN_DELTA..=MAX_DELTA {
            let level = level0 + m;
            let ctx = if level > 2 { 2 } else { level };
            let band = usize::from(VP8_ENC_BANDS[idx(n) + 1]);
            let base_score: score_t;
            let mut best_cur_score: score_t;
            let mut best_prev: i32;
            let mut cost: score_t;
            let mut score: score_t;

            // C reads costs[16][ctx] one entry past the remapped table for
            // n == 15 and never uses the value; the placeholder stands for it.
            score_states[ss_cur][node(m)].costs = if n < 15 {
                costs.row(idx(n) + 1, idx(ctx))
            } else {
                placeholder.costs
            };
            if level < 0 || level > thresh_level {
                score_states[ss_cur][node(m)].score = MAX_COST;
                // Node is dead.
                continue;
            }

            {
                // Compute delta_error = how much coding this level will
                // subtract to max_error as distortion.
                // Here, distortion = sum of (|coeff_i| - level_i * Q_i)^2
                let new_error = coeff0.wrapping_sub((level as u32).wrapping_mul(q)) as i32;
                let delta_error = u32::from(WEIGHT_TRELLIS[j]).wrapping_mul(
                    (new_error.wrapping_mul(new_error) as u32)
                        .wrapping_sub(coeff0.wrapping_mul(coeff0)),
                ) as i32;
                base_score = rd_score_trellis(lambda, 0, score_t::from(delta_error));
            }

            // Inspect all possible non-dead predecessors. Retain only the best one.
            // The base_score is added to all scores so it is only added for the final
            // value after the loop.
            let prev_states = &score_states[ss_prev];
            cost = score_t::from(vp8_level_cost(prev_states[node(-MIN_DELTA)].costs, level));
            best_cur_score =
                prev_states[node(-MIN_DELTA)].score + rd_score_trellis(lambda, cost, 0);
            best_prev = -MIN_DELTA;
            for p in (-MIN_DELTA + 1)..=MAX_DELTA {
                // Dead nodes (with ss_prev[p].score >= MAX_COST) are automatically
                // eliminated since their score can't be better than the current best.
                cost = score_t::from(vp8_level_cost(prev_states[node(p)].costs, level));
                // Examine node assuming it's a non-terminal one.
                score = prev_states[node(p)].score + rd_score_trellis(lambda, cost, 0);
                if score < best_cur_score {
                    best_cur_score = score;
                    best_prev = p;
                }
            }
            best_cur_score += base_score;
            // Store best finding in current node.
            let cur = &mut nodes[idx(n)][node(m)];
            cur.sign = i8::from(sign);
            cur.level = level as i16;
            cur.prev = best_prev as i8;
            score_states[ss_cur][node(m)].score = best_cur_score;

            // Now, record best terminal node (and thus best entry in the graph).
            if level != 0 && best_cur_score < best_score {
                let last_pos_cost = if n < 15 {
                    score_t::from(vp8_bit_cost(false, probas[band][idx(ctx)][0]))
                } else {
                    0
                };
                let last_pos_score = rd_score_trellis(lambda, last_pos_cost, 0);
                score = best_cur_score + last_pos_score;
                if score < best_score {
                    best_score = score;
                    best_path[0] = n; // best eob position
                    best_path[1] = m; // best node index
                    best_path[2] = best_prev; // best predecessor
                }
            }
        }
    }

    // Fresh start
    // Beware! We must preserve in[0]/out[0] value for TYPE_I16_AC case.
    if coeff_type == TYPE_I16_AC {
        input[1..16].fill(0);
        out[1..16].fill(0);
    } else {
        input[..16].fill(0);
        out[..16].fill(0);
    }
    if best_path[0] == -1 {
        return 0; // skip!
    }

    {
        // Unwind the best path.
        // Note: best-prev on terminal node is not necessarily equal to the
        // best_prev for non-terminal. So we patch best_path[2] in.
        let mut nz = 0;
        let mut best_node = best_path[1];
        let mut n = best_path[0];
        nodes[idx(n)][node(best_node)].prev = best_path[2] as i8; // force best-prev for terminal

        while n >= first {
            let nd = nodes[idx(n)][node(best_node)];
            let j = usize::from(ZIGZAG[idx(n)]);
            out[idx(n)] = if nd.sign != 0 { -nd.level } else { nd.level };
            nz |= i32::from(nd.level);
            input[j] = (i32::from(out[idx(n)]) * i32::from(mtx.q[j])) as i16;
            best_node = i32::from(nd.prev);
            n -= 1;
        }
        i32::from(nz != 0)
    }
}

//------------------------------------------------------------------------------
// Performs: difference, transform, quantize, back-transform, add
// all at once. Output is the reconstructed block in *yuv_out, and the
// quantized levels in *levels.

// upstream: src/enc/quant_enc.c ReconstructIntra16
/// `yuv_out` is one of the iterator's output buffers, which the caller moves
/// out of the iterator for the call (C: a pointer into `yuv_mem`).
fn reconstruct_intra16(
    it: &mut VP8EncIterator,
    enc: &VP8Encoder,
    rd: &mut VP8ModeScore,
    yuv_out: &mut [u8],
    mode: i32,
) -> i32 {
    let ref_pos = usize::from(VP8I16_MODE_OFFSETS[idx(mode)]);
    let src_pos = idx(Y_OFF_ENC);
    let dqm = &enc.dqm[usize::from(enc.mb_info[it.mb].segment)];
    let mut nz = 0;
    let mut tmp = [0_i16; 16 * 16];
    let mut dc_tmp = [0_i16; 16];

    for n in (0..16).step_by(2) {
        let scan = usize::from(VP8_SCAN[n]);
        vp8f_transform2(
            &it.yuv_in[src_pos + scan..],
            &it.yuv_p[ref_pos + scan..],
            &mut tmp[n * 16..],
        );
    }
    vp8f_transform_wht(&tmp, &mut dc_tmp);
    nz |= vp8_enc_quantize_block_wht(&mut dc_tmp, &mut rd.y_dc_levels, &dqm.y2) << 24;

    if DO_TRELLIS_I16 && it.do_trellis {
        vp8_iterator_nz_to_bytes(it, enc);
        let mut n = 0;
        for y in 0..4 {
            for x in 0..4 {
                let ctx = it.top_nz[x] + it.left_nz[y];
                let non_zero = trellis_quantize_block(
                    enc,
                    &mut tmp[n * 16..n * 16 + 16],
                    &mut rd.y_ac_levels[n],
                    ctx,
                    TYPE_I16_AC,
                    &dqm.y1,
                    dqm.lambda_trellis_i16,
                );
                it.top_nz[x] = non_zero;
                it.left_nz[y] = non_zero;
                rd.y_ac_levels[n][0] = 0;
                nz |= non_zero << n;
                n += 1;
            }
        }
    } else {
        for n in (0..16).step_by(2) {
            // Zero-out the first coeff, so that: a) nz is correct below, and
            // b) finding 'last' non-zero coeffs in SetResidualCoeffs() is simplified.
            tmp[n * 16] = 0;
            tmp[(n + 1) * 16] = 0;
            nz |= vp8_enc_quantize2_blocks(
                &mut tmp[n * 16..n * 16 + 32],
                &mut rd.y_ac_levels.as_flattened_mut()[n * 16..n * 16 + 32],
                &dqm.y1,
            ) << n;
            debug_assert_eq!(rd.y_ac_levels[n][0], 0);
            debug_assert_eq!(rd.y_ac_levels[n + 1][0], 0);
        }
    }

    // Transform back
    vp8_transform_wht(&dc_tmp, &mut tmp);
    for n in (0..16).step_by(2) {
        let scan = usize::from(VP8_SCAN[n]);
        vp8i_transform(
            &it.yuv_p[ref_pos + scan..],
            &tmp[n * 16..],
            &mut yuv_out[scan..],
            true,
        );
    }

    nz
}

// upstream: src/enc/quant_enc.c ReconstructIntra4
/// `src` is the block in `yuv_in`; `yuv_out` is a buffer outside the iterator.
fn reconstruct_intra4(
    it: &VP8EncIterator,
    enc: &VP8Encoder,
    levels: &mut [i16; 16],
    src: &[u8],
    yuv_out: &mut [u8],
    mode: i32,
) -> i32 {
    let ref_ = &it.yuv_p[usize::from(VP8I4_MODE_OFFSETS[idx(mode)])..];
    let dqm = &enc.dqm[usize::from(enc.mb_info[it.mb].segment)];
    let mut tmp = [0_i16; 16];

    vp8f_transform(src, ref_, &mut tmp);
    let nz = if DO_TRELLIS_I4 && it.do_trellis {
        let x = idx(it.i4 & 3);
        let y = idx(it.i4 >> 2);
        let ctx = it.top_nz[x] + it.left_nz[y];
        trellis_quantize_block(
            enc,
            &mut tmp,
            levels,
            ctx,
            TYPE_I4_AC,
            &dqm.y1,
            dqm.lambda_trellis_i4,
        )
    } else {
        vp8_enc_quantize_block(&mut tmp, levels, &dqm.y1)
    };
    vp8i_transform(ref_, &tmp, yuv_out, false);
    nz
}

//------------------------------------------------------------------------------
// DC-error diffusion

// Diffusion weights. We under-correct a bit (15/16th of the error is actually
// diffused) to avoid 'rainbow' chessboard pattern of blocks at q~=0.
// upstream: src/enc/quant_enc.c C1
/// fraction of error sent to the 4x4 block below
const C1: i32 = 7;
// upstream: src/enc/quant_enc.c C2
/// fraction of error sent to the 4x4 block on the right
const C2: i32 = 8;
// upstream: src/enc/quant_enc.c DSHIFT
const DSHIFT: i32 = 4;
// upstream: src/enc/quant_enc.c DSCALE
/// storage descaling, needed to make the error fit `int8_t`
const DSCALE: i32 = 1;

// upstream: src/enc/quant_enc.c QuantizeSingle
/// Quantize as usual, but also compute and return the quantization error.
/// Error is already divided by DSHIFT.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    reason = "C converts between int, int16_t and uint32_t at these points"
)]
fn quantize_single(v: &mut i16, mtx: &VP8Matrix) -> i32 {
    let mut vv = i32::from(*v);
    let sign = vv < 0;
    if sign {
        vv = -vv;
    }
    if vv > mtx.zthresh[0] as i32 {
        let q_v = quantdiv(vv as u32, u32::from(mtx.iq[0]), mtx.bias[0]) * i32::from(mtx.q[0]);
        let err = vv - q_v;
        *v = if sign { -q_v } else { q_v } as i16;
        return (if sign { -err } else { err }) >> DSCALE;
    }
    *v = 0;
    (if sign { -vv } else { vv }) >> DSCALE
}

// upstream: src/enc/quant_enc.c CorrectDCValues
#[expect(
    clippy::cast_possible_truncation,
    reason = "C adds the corrections to int16_t and stores the errors into int8_t"
)]
fn correct_dc_values(
    it: &VP8EncIterator,
    enc: &VP8Encoder,
    mtx: &VP8Matrix,
    tmp: &mut [i16],
    rd: &mut VP8ModeScore,
) {
    //         | top[0] | top[1]
    // --------+--------+---------
    // left[0] | tmp[0]   tmp[1]  <->   err0 err1
    // left[1] | tmp[2]   tmp[3]        err2 err3
    //
    // Final errors {err1,err2,err3} are preserved and later restored
    // as top[]/left[] on the next block.
    let top_derr = enc.top_derr.as_ref().expect("the encoder diffuses errors");
    for ch in 0..=1 {
        let top = &top_derr[idx(it.x)][ch];
        let left = &it.left_derr[ch];
        let c = &mut tmp[ch * 4 * 16..];
        c[0] = (i32::from(c[0])
            + ((C1 * i32::from(top[0]) + C2 * i32::from(left[0])) >> (DSHIFT - DSCALE)))
            as i16;
        let err0 = quantize_single(&mut c[0], mtx);
        c[16] =
            (i32::from(c[16]) + ((C1 * i32::from(top[1]) + C2 * err0) >> (DSHIFT - DSCALE))) as i16;
        let err1 = quantize_single(&mut c[16], mtx);
        c[2 * 16] = (i32::from(c[2 * 16])
            + ((C1 * err0 + C2 * i32::from(left[1])) >> (DSHIFT - DSCALE)))
            as i16;
        let err2 = quantize_single(&mut c[2 * 16], mtx);
        c[3 * 16] = (i32::from(c[3 * 16]) + ((C1 * err1 + C2 * err2) >> (DSHIFT - DSCALE))) as i16;
        let err3 = quantize_single(&mut c[3 * 16], mtx);
        // error 'err' is bounded by mtx->q[0] which is 132 at max. Hence
        // err >> DSCALE will fit in an int8_t type if DSCALE>=1.
        debug_assert!(err1.abs() <= 127 && err2.abs() <= 127 && err3.abs() <= 127);
        rd.derr[ch][0] = err1 as i8;
        rd.derr[ch][1] = err2 as i8;
        rd.derr[ch][2] = err3 as i8;
    }
}

// upstream: src/enc/quant_enc.c StoreDiffusionErrors
#[expect(
    clippy::cast_possible_truncation,
    reason = "C stores the int results into int8_t"
)]
#[allow(
    clippy::needless_range_loop,
    reason = "the C index loop over two arrays"
)]
fn store_diffusion_errors(it: &mut VP8EncIterator, enc: &mut VP8Encoder, rd: &VP8ModeScore) {
    let top_derr = enc.top_derr.as_mut().expect("the encoder diffuses errors");
    for ch in 0..=1 {
        let top = &mut top_derr[idx(it.x)][ch];
        let left = &mut it.left_derr[ch];
        left[0] = rd.derr[ch][0]; // restore err1
        left[1] = ((3 * i32::from(rd.derr[ch][2])) >> 2) as i8; //     ... 3/4th of err3
        top[0] = rd.derr[ch][1]; //     ... err2
        top[1] = (i32::from(rd.derr[ch][2]) - i32::from(left[1])) as i8; //     ... 1/4th of err3.
    }
}

//------------------------------------------------------------------------------

// upstream: src/enc/quant_enc.c ReconstructUV
/// `yuv_out` is one of the iterator's output buffers from `U_OFF_ENC`, which the
/// caller moves out of the iterator for the call.
fn reconstruct_uv(
    it: &mut VP8EncIterator,
    enc: &VP8Encoder,
    rd: &mut VP8ModeScore,
    yuv_out: &mut [u8],
    mode: i32,
) -> i32 {
    let ref_pos = usize::from(VP8UV_MODE_OFFSETS[idx(mode)]);
    let src_pos = idx(U_OFF_ENC);
    let dqm = &enc.dqm[usize::from(enc.mb_info[it.mb].segment)];
    let mut nz = 0;
    let mut tmp = [0_i16; 8 * 16];

    for n in (0..8).step_by(2) {
        let scan = usize::from(VP8_SCAN_UV[n]);
        vp8f_transform2(
            &it.yuv_in[src_pos + scan..],
            &it.yuv_p[ref_pos + scan..],
            &mut tmp[n * 16..],
        );
    }
    if it.top_derr {
        correct_dc_values(it, enc, &dqm.uv, &mut tmp, rd);
    }

    if DO_TRELLIS_UV && it.do_trellis {
        let mut n = 0;
        for ch in [0, 2] {
            for y in 0..2 {
                for x in 0..2 {
                    let ctx = it.top_nz[4 + ch + x] + it.left_nz[4 + ch + y];
                    let non_zero = trellis_quantize_block(
                        enc,
                        &mut tmp[n * 16..n * 16 + 16],
                        &mut rd.uv_levels[n],
                        ctx,
                        TYPE_CHROMA_A,
                        &dqm.uv,
                        dqm.lambda_trellis_uv,
                    );
                    it.top_nz[4 + ch + x] = non_zero;
                    it.left_nz[4 + ch + y] = non_zero;
                    nz |= non_zero << n;
                    n += 1;
                }
            }
        }
    } else {
        for n in (0..8).step_by(2) {
            nz |= vp8_enc_quantize2_blocks(
                &mut tmp[n * 16..n * 16 + 32],
                &mut rd.uv_levels.as_flattened_mut()[n * 16..n * 16 + 32],
                &dqm.uv,
            ) << n;
        }
    }

    for n in (0..8).step_by(2) {
        let scan = usize::from(VP8_SCAN_UV[n]);
        vp8i_transform(
            &it.yuv_p[ref_pos + scan..],
            &tmp[n * 16..],
            &mut yuv_out[scan..],
            true,
        );
    }
    nz << 16
}

//------------------------------------------------------------------------------
// RD-opt decision. Reconstruct each modes, evalue distortion and bit-cost.
// Pick the mode is lower RD-cost = Rate + lambda * Distortion.

// upstream: src/enc/quant_enc.c StoreMaxDelta
fn store_max_delta(dqm: &mut VP8SegmentInfo, dcs: &[i16; 16]) {
    // We look at the first three AC coefficients to determine what is the average
    // delta between each sub-4x4 block.
    let v0 = i32::from(dcs[1]).abs();
    let v1 = i32::from(dcs[2]).abs();
    let v2 = i32::from(dcs[4]).abs();
    let mut max_v = if v1 > v0 { v1 } else { v0 };
    max_v = if v2 > max_v { v2 } else { max_v };
    if max_v > dqm.max_edge {
        dqm.max_edge = max_v;
    }
}

// upstream: src/enc/quant_enc.c SwapModeScore
// C swaps the pointers rd_cur and rd_best; PickBestIntra16 tracks which of
// its two scores is the best one instead.

// upstream: src/enc/quant_enc.c SwapPtr
// C swaps two sample pointers; the callers track which buffer each one names.

// upstream: src/enc/quant_enc.c SwapOut
fn swap_out(it: &mut VP8EncIterator) {
    std::mem::swap(&mut it.yuv_out, &mut it.yuv_out2);
}

/// The encoder's whole macroblock infos and intra modes, for the mode setters.
fn mode_rows(enc: &mut VP8Encoder) -> ModeRows<'_> {
    ModeRows::whole(&mut enc.mb_info, &mut enc.preds, enc.preds_w)
}

// upstream: src/enc/quant_enc.c PickBestIntra16
fn pick_best_intra16(it: &mut VP8EncIterator, enc: &mut VP8Encoder, rd: &mut VP8ModeScore) {
    let k_num_blocks = 16;
    let segment = usize::from(enc.mb_info[it.mb].segment);
    let dqm = enc.dqm[segment];
    let lambda = dqm.lambda_i16;
    let tlambda = dqm.tlambda;
    let src = idx(Y_OFF_ENC);
    let mut rd_tmp = VP8ModeScore::default();
    // C: rd_cur = &rd_tmp, rd_best = rd; SwapModeScore exchanges them.
    let mut best_is_rd = true;
    let mut is_flat_block = is_flat_source16(&it.yuv_in[idx(Y_OFF_ENC)..]);

    rd.mode_i16 = -1;
    for mode in 0..NUM_PRED_MODES {
        // scratch buffer
        let mut tmp_dst = std::mem::take(&mut it.yuv_out2);
        let rd_cur: &mut VP8ModeScore = if best_is_rd { &mut rd_tmp } else { &mut *rd };
        rd_cur.mode_i16 = mode;

        // Reconstruct
        #[expect(clippy::cast_sign_loss, reason = "nz is a bit set")]
        {
            rd_cur.nz =
                reconstruct_intra16(it, enc, rd_cur, &mut tmp_dst[idx(Y_OFF_ENC)..], mode) as u32;
        }

        // Measure RD-score
        rd_cur.d = score_t::from(vp8sse16x16(&it.yuv_in[src..], &tmp_dst[idx(Y_OFF_ENC)..]));
        rd_cur.sd = if tlambda != 0 {
            score_t::from(mult_8b(
                tlambda,
                vp8t_disto16x16(&it.yuv_in[src..], &tmp_dst[idx(Y_OFF_ENC)..], &WEIGHT_Y),
            ))
        } else {
            0
        };
        it.yuv_out2 = tmp_dst;
        rd_cur.h = score_t::from(VP8_FIXED_COSTS_I16[idx(mode)]);
        rd_cur.r = score_t::from(vp8_get_cost_luma16(it, enc, rd_cur));
        if is_flat_block {
            // refine the first impression (which was in pixel space)
            is_flat_block = is_flat(
                rd_cur.y_ac_levels.as_flattened(),
                k_num_blocks,
                FLATNESS_LIMIT_I16,
            );
            if is_flat_block {
                // Block is very flat. We put emphasis on the distortion being very low!
                rd_cur.d *= 2;
                rd_cur.sd *= 2;
            }
        }

        // Since we always examine Intra16 first, we can overwrite *rd directly.
        set_rd_score(lambda, rd_cur);
        let cur_score = rd_cur.score;
        let best_score = if best_is_rd { rd.score } else { rd_tmp.score };
        if mode == 0 || cur_score < best_score {
            best_is_rd = !best_is_rd;
            swap_out(it);
        }
    }
    if !best_is_rd {
        *rd = rd_tmp;
    }
    set_rd_score(dqm.lambda_mode, rd); // finalize score for mode decision.
    vp8_set_intra16_mode(it, &mut mode_rows(enc), rd.mode_i16);

    // we have a blocky macroblock (only DCs are non-zero) with fairly high
    // distortion, record max delta so we can later adjust the minimal filtering
    // strength needed to smooth these blocks out.
    if (rd.nz & 0x100_ffff) == 0x100_0000 && rd.d > score_t::from(dqm.min_disto) {
        store_max_delta(&mut enc.dqm[segment], &rd.y_dc_levels);
    }
}

//------------------------------------------------------------------------------

// upstream: src/enc/quant_enc.c GetCostModeI4
/// Return the cost array corresponding to the surrounding prediction modes.
fn get_cost_mode_i4(
    it: &VP8EncIterator,
    enc: &VP8Encoder,
    modes: &[u8; 16],
) -> &'static [u16; NUM_BMODES as usize] {
    let preds_w = idx(enc.preds_w);
    let x = idx(it.i4 & 3);
    let y = idx(it.i4 >> 2);
    let left = if x == 0 {
        enc.preds[it.preds + y * preds_w - 1]
    } else {
        modes[idx(it.i4) - 1]
    };
    let top = if y == 0 {
        enc.preds[it.preds - preds_w + x]
    } else {
        modes[idx(it.i4) - 4]
    };
    &VP8_FIXED_COSTS_I4[usize::from(top)][usize::from(left)]
}

/// Where a 4x4 reconstruction of `PickBestIntra4` lives: the scratch block (C:
/// `it->yuv_p + I4TMP`) or the block of `best_blocks` (C: `yuv_out2`).
#[derive(Clone, Copy, PartialEq, Eq)]
enum I4Block {
    Scratch,
    Best,
}

/// The size of the scratch block of `PickBestIntra4`: four rows of `BPS`
/// samples, of which the first four of each are used.
const I4_SCRATCH_SIZE: usize = 3 * BPS_U + 4;

// upstream: src/enc/quant_enc.c PickBestIntra4
/// C writes the 4x4 trials into `it->yuv_p + I4TMP`, a scratch area of the
/// prediction buffer that nothing else reads; here the scratch is a local block
/// (see the `# Deviations from upstream` section of this module, item 2).
fn pick_best_intra4(it: &mut VP8EncIterator, enc: &mut VP8Encoder, rd: &mut VP8ModeScore) -> bool {
    let segment = usize::from(enc.mb_info[it.mb].segment);
    let dqm = enc.dqm[segment];
    let mut rd_best = VP8ModeScore::default();

    if enc.max_i4_header_bits == 0 {
        return false;
    }

    init_score(&mut rd_best);
    rd_best.h = 211; // '211' is the value of VP8BitCost(0, 145)
    set_rd_score(dqm.lambda_mode, &mut rd_best);
    vp8_iterator_start_i4(it, enc);
    let mut best_blocks = std::mem::take(&mut it.yuv_out2);
    let selected = pick_best_intra4_blocks(it, enc, rd, &dqm, &mut rd_best, &mut best_blocks);
    it.yuv_out2 = best_blocks;
    if !selected {
        return false;
    }

    // finalize state
    copy_score(rd, &rd_best);
    vp8_set_intra4_mode(it, &mut mode_rows(enc), &rd.modes_i4);
    swap_out(it);
    rd.y_ac_levels = rd_best.y_ac_levels;
    true // select intra4x4 over intra16x16
}

/// The sub-block loop of `PickBestIntra4`; `best_blocks` is `it->yuv_out2`,
/// moved out of the iterator. Returns `false` where C returns 0 from the loop.
fn pick_best_intra4_blocks(
    it: &mut VP8EncIterator,
    enc: &mut VP8Encoder,
    rd: &mut VP8ModeScore,
    dqm: &VP8SegmentInfo,
    rd_best: &mut VP8ModeScore,
    best_blocks: &mut [u8],
) -> bool {
    let lambda = dqm.lambda_i4;
    let tlambda = dqm.tlambda;
    let src0 = idx(Y_OFF_ENC);
    let best0 = idx(Y_OFF_ENC);
    let mut total_header_bits = 0;
    let mut scratch = [0_u8; I4_SCRATCH_SIZE];

    loop {
        let k_num_blocks = 1;
        let mut rd_i4 = VP8ModeScore::default();
        let mut best_mode: i32 = -1;
        let scan = usize::from(VP8_SCAN[idx(it.i4)]);
        let src = src0 + scan;
        let mode_costs = get_cost_mode_i4(it, enc, &rd.modes_i4);
        let mut best_block = I4Block::Best;
        let mut tmp_dst = I4Block::Scratch; // scratch buffer.

        init_score(&mut rd_i4);
        make_intra4_preds(it);
        for mode in 0..NUM_BMODES {
            let mut rd_tmp = VP8ModeScore::default();
            let mut tmp_levels = [0_i16; 16];

            // Reconstruct
            let dst: &mut [u8] = match tmp_dst {
                I4Block::Scratch => &mut scratch,
                I4Block::Best => &mut best_blocks[best0 + scan..],
            };
            #[expect(clippy::cast_sign_loss, reason = "nz is a bit set")]
            {
                rd_tmp.nz =
                    (reconstruct_intra4(it, enc, &mut tmp_levels, &it.yuv_in[src..], dst, mode)
                        << it.i4) as u32;
            }

            // Compute RD-score
            rd_tmp.d = score_t::from(vp8sse4x4(&it.yuv_in[src..], dst));
            rd_tmp.sd = if tlambda != 0 {
                score_t::from(mult_8b(
                    tlambda,
                    vp8t_disto4x4(&it.yuv_in[src..], dst, &WEIGHT_Y),
                ))
            } else {
                0
            };
            rd_tmp.h = score_t::from(mode_costs[idx(mode)]);

            // Add flatness penalty, to avoid flat area to be mispredicted
            // by a complex mode.
            if mode > 0 && is_flat(&tmp_levels, k_num_blocks, FLATNESS_LIMIT_I4) {
                rd_tmp.r = score_t::from(FLATNESS_PENALTY * k_num_blocks);
            } else {
                rd_tmp.r = 0;
            }

            // early-out check
            set_rd_score(lambda, &mut rd_tmp);
            if best_mode >= 0 && rd_tmp.score >= rd_i4.score {
                continue;
            }

            // finish computing score
            rd_tmp.r += score_t::from(vp8_get_cost_luma4(it, enc, &tmp_levels));
            set_rd_score(lambda, &mut rd_tmp);

            if best_mode < 0 || rd_tmp.score < rd_i4.score {
                copy_score(&mut rd_i4, &rd_tmp);
                best_mode = mode;
                std::mem::swap(&mut tmp_dst, &mut best_block);
                rd_best.y_ac_levels[idx(it.i4)] = tmp_levels;
            }
        }
        set_rd_score(dqm.lambda_mode, &mut rd_i4);
        add_score(rd_best, &rd_i4);
        if rd_best.score >= rd.score {
            return false;
        }
        #[expect(
            clippy::cast_possible_truncation,
            reason = "C casts the header bits to int"
        )]
        {
            total_header_bits += rd_i4.h as i32; // <- equal to mode_costs[best_mode];
        }
        if total_header_bits > enc.max_i4_header_bits {
            return false;
        }
        // Copy selected samples if not in the right place already.
        if best_block != I4Block::Best {
            vp8_copy4x4(&scratch, &mut best_blocks[best0 + scan..]);
        }
        #[expect(
            clippy::cast_possible_truncation,
            clippy::cast_sign_loss,
            reason = "the mode is in 0..NUM_BMODES"
        )]
        {
            rd.modes_i4[idx(it.i4)] = best_mode as u8;
        }
        let nz = i32::from(rd_i4.nz != 0);
        it.top_nz[idx(it.i4 & 3)] = nz;
        it.left_nz[idx(it.i4 >> 2)] = nz;
        if !vp8_iterator_rotate_i4(it, &best_blocks[best0..]) {
            break;
        }
    }
    true
}

//------------------------------------------------------------------------------

// upstream: src/enc/quant_enc.c PickBestUV
fn pick_best_uv(it: &mut VP8EncIterator, enc: &mut VP8Encoder, rd: &mut VP8ModeScore) {
    let k_num_blocks = 8;
    let segment = usize::from(enc.mb_info[it.mb].segment);
    let lambda = enc.dqm[segment].lambda_uv;
    let src = idx(U_OFF_ENC);
    // C: tmp_dst = yuv_out2 + U_OFF_ENC, dst = dst0 = yuv_out + U_OFF_ENC;
    // SwapPtr(&dst, &tmp_dst) exchanges them.
    let mut dst_is_out = true;
    let mut rd_best = VP8ModeScore::default();

    rd.mode_uv = -1;
    init_score(&mut rd_best);
    for mode in 0..NUM_PRED_MODES {
        let mut rd_uv = VP8ModeScore::default();

        // Reconstruct
        let mut tmp_dst = if dst_is_out {
            std::mem::take(&mut it.yuv_out2)
        } else {
            std::mem::take(&mut it.yuv_out)
        };
        #[expect(clippy::cast_sign_loss, reason = "nz is a bit set")]
        {
            rd_uv.nz =
                reconstruct_uv(it, enc, &mut rd_uv, &mut tmp_dst[idx(U_OFF_ENC)..], mode) as u32;
        }

        // Compute RD-score
        rd_uv.d = score_t::from(vp8sse16x8(&it.yuv_in[src..], &tmp_dst[idx(U_OFF_ENC)..]));
        if dst_is_out {
            it.yuv_out2 = tmp_dst;
        } else {
            it.yuv_out = tmp_dst;
        }
        rd_uv.sd = 0; // not calling TDisto here: it tends to flatten areas.
        rd_uv.h = score_t::from(VP8_FIXED_COSTS_UV[idx(mode)]);
        rd_uv.r = score_t::from(vp8_get_cost_uv(it, enc, &rd_uv));
        if mode > 0
            && is_flat(
                rd_uv.uv_levels.as_flattened(),
                k_num_blocks,
                FLATNESS_LIMIT_UV,
            )
        {
            rd_uv.r += score_t::from(FLATNESS_PENALTY * k_num_blocks);
        }

        set_rd_score(lambda, &mut rd_uv);
        if mode == 0 || rd_uv.score < rd_best.score {
            copy_score(&mut rd_best, &rd_uv);
            rd.mode_uv = mode;
            rd.uv_levels = rd_uv.uv_levels;
            if it.top_derr {
                rd.derr = rd_uv.derr;
            }
            dst_is_out = !dst_is_out;
        }
    }
    vp8_set_intra_uv_mode(it, &mut mode_rows(enc), rd.mode_uv);
    add_score(rd, &rd_best);
    if !dst_is_out {
        // copy 16x8 block if needed
        vp8_copy16x8(
            &it.yuv_out2[idx(U_OFF_ENC)..],
            &mut it.yuv_out[idx(U_OFF_ENC)..],
        );
    }
    if it.top_derr {
        // store diffusion errors for next block
        store_diffusion_errors(it, enc, rd);
    }
}

//------------------------------------------------------------------------------
// Final reconstruction and quantization.

// upstream: src/enc/quant_enc.c SimpleQuantize
fn simple_quantize(it: &mut VP8EncIterator, enc: &mut VP8Encoder, rd: &mut VP8ModeScore) {
    let is_i16 = enc.mb_info[it.mb].r#type == 1;
    let mut nz = 0;

    let mut yuv_out = std::mem::take(&mut it.yuv_out);
    if is_i16 {
        let mode = i32::from(enc.preds[it.preds]);
        nz = reconstruct_intra16(it, enc, rd, &mut yuv_out[idx(Y_OFF_ENC)..], mode);
    } else {
        vp8_iterator_start_i4(it, enc);
        loop {
            let mode = i32::from(
                enc.preds[it.preds + idx(it.i4 & 3) + idx(it.i4 >> 2) * idx(enc.preds_w)],
            );
            let scan = usize::from(VP8_SCAN[idx(it.i4)]);
            let src = idx(Y_OFF_ENC) + scan;
            let dst = idx(Y_OFF_ENC) + scan;
            make_intra4_preds(it);
            nz |= reconstruct_intra4(
                it,
                enc,
                &mut rd.y_ac_levels[idx(it.i4)],
                &it.yuv_in[src..],
                &mut yuv_out[dst..],
                mode,
            ) << it.i4;
            if !vp8_iterator_rotate_i4(it, &yuv_out[idx(Y_OFF_ENC)..]) {
                break;
            }
        }
    }

    let uv_mode = i32::from(enc.mb_info[it.mb].uv_mode);
    nz |= reconstruct_uv(it, enc, rd, &mut yuv_out[idx(U_OFF_ENC)..], uv_mode);
    it.yuv_out = yuv_out;
    #[expect(clippy::cast_sign_loss, reason = "nz is a bit set")]
    {
        rd.nz = nz as u32;
    }
}

// upstream: src/enc/quant_enc.c RefineUsingDistortion
/// Refine intra16/intra4 sub-modes based on distortion only (not rate).
#[expect(clippy::too_many_lines, reason = "the C function translated whole")]
fn refine_using_distortion(
    it: &mut VP8EncIterator,
    enc: &mut VP8Encoder,
    try_both_modes: bool,
    refine_uv_mode: bool,
    rd: &mut VP8ModeScore,
) {
    let mut try_both_modes = try_both_modes;
    let mut best_score = MAX_COST;
    let mut nz = 0;
    let mut is_i16 = try_both_modes || enc.mb_info[it.mb].r#type == 1;

    let dqm = enc.dqm[usize::from(enc.mb_info[it.mb].segment)];
    // Some empiric constants, of approximate order of magnitude.
    let lambda_d_i16 = 106;
    let lambda_d_i4 = 11;
    let lambda_d_uv = 120;
    let mut score_i4 = dqm.i4_penalty;
    let mut i4_bit_sum: score_t = 0;
    let bit_limit = if try_both_modes {
        score_t::from(enc.mb_header_limit)
    } else {
        MAX_COST // no early-out allowed
    };

    if is_i16 {
        // First, evaluate Intra16 distortion
        let mut best_mode = -1;
        let src = idx(Y_OFF_ENC);
        for mode in 0..NUM_PRED_MODES {
            let ref_ = usize::from(VP8I16_MODE_OFFSETS[idx(mode)]);
            let score = score_t::from(vp8sse16x16(&it.yuv_in[src..], &it.yuv_p[ref_..]))
                * score_t::from(RD_DISTO_MULT)
                + score_t::from(i32::from(VP8_FIXED_COSTS_I16[idx(mode)]) * lambda_d_i16);
            if mode > 0 && score_t::from(VP8_FIXED_COSTS_I16[idx(mode)]) > bit_limit {
                continue;
            }

            if score < best_score {
                best_mode = mode;
                best_score = score;
            }
        }
        if it.x == 0 || it.y == 0 {
            // avoid starting a checkerboard resonance from the border. See bug #432.
            if is_flat_source16(&it.yuv_in[src..]) {
                best_mode = if it.x == 0 { 0 } else { 2 };
                try_both_modes = false; // stick to i16
            }
        }
        vp8_set_intra16_mode(it, &mut mode_rows(enc), best_mode);
        // we'll reconstruct later, if i16 mode actually gets selected
    }

    // Next, evaluate Intra4
    if try_both_modes || !is_i16 {
        // We don't evaluate the rate here, but just account for it through a
        // constant penalty (i4 mode usually needs more bits compared to i16).
        is_i16 = false;
        vp8_iterator_start_i4(it, enc);
        let mut yuv_out2 = std::mem::take(&mut it.yuv_out2);
        loop {
            let mut best_i4_mode = -1;
            let mut best_i4_score = MAX_COST;
            let scan = usize::from(VP8_SCAN[idx(it.i4)]);
            let src = idx(Y_OFF_ENC) + scan;
            let mode_costs = get_cost_mode_i4(it, enc, &rd.modes_i4);

            make_intra4_preds(it);
            for mode in 0..NUM_BMODES {
                let ref_ = usize::from(VP8I4_MODE_OFFSETS[idx(mode)]);
                let score = score_t::from(
                    vp8sse4x4(&it.yuv_in[src..], &it.yuv_p[ref_..]) * RD_DISTO_MULT
                        + i32::from(mode_costs[idx(mode)]) * lambda_d_i4,
                );
                if score < best_i4_score {
                    best_i4_mode = mode;
                    best_i4_score = score;
                }
            }
            i4_bit_sum += score_t::from(mode_costs[idx(best_i4_mode)]);
            #[expect(
                clippy::cast_possible_truncation,
                clippy::cast_sign_loss,
                reason = "the mode is in 0..NUM_BMODES"
            )]
            {
                rd.modes_i4[idx(it.i4)] = best_i4_mode as u8;
            }
            score_i4 += best_i4_score;
            if score_i4 >= best_score || i4_bit_sum > bit_limit {
                // Intra4 won't be better than Intra16. Bail out and pick Intra16.
                is_i16 = true;
                break;
            }
            // reconstruct partial block inside yuv_out2 buffer
            let tmp_dst = idx(Y_OFF_ENC) + scan;
            nz |= reconstruct_intra4(
                it,
                enc,
                &mut rd.y_ac_levels[idx(it.i4)],
                &it.yuv_in[src..],
                &mut yuv_out2[tmp_dst..],
                best_i4_mode,
            ) << it.i4;
            if !vp8_iterator_rotate_i4(it, &yuv_out2[idx(Y_OFF_ENC)..]) {
                break;
            }
        }
        it.yuv_out2 = yuv_out2;
    }

    // Final reconstruction, depending on which mode is selected.
    if is_i16 {
        let mut yuv_out = std::mem::take(&mut it.yuv_out);
        let mode = i32::from(enc.preds[it.preds]);
        nz = reconstruct_intra16(it, enc, rd, &mut yuv_out[idx(Y_OFF_ENC)..], mode);
        it.yuv_out = yuv_out;
    } else {
        vp8_set_intra4_mode(it, &mut mode_rows(enc), &rd.modes_i4);
        swap_out(it);
        best_score = score_i4;
    }

    // ... and UV!
    if refine_uv_mode {
        let mut best_mode = -1;
        let mut best_uv_score = MAX_COST;
        let src = idx(U_OFF_ENC);
        for mode in 0..NUM_PRED_MODES {
            let ref_ = usize::from(VP8UV_MODE_OFFSETS[idx(mode)]);
            let score = score_t::from(
                vp8sse16x8(&it.yuv_in[src..], &it.yuv_p[ref_..]) * RD_DISTO_MULT
                    + i32::from(VP8_FIXED_COSTS_UV[idx(mode)]) * lambda_d_uv,
            );
            if score < best_uv_score {
                best_mode = mode;
                best_uv_score = score;
            }
        }
        vp8_set_intra_uv_mode(it, &mut mode_rows(enc), best_mode);
    }
    let uv_mode = i32::from(enc.mb_info[it.mb].uv_mode);
    let mut yuv_out = std::mem::take(&mut it.yuv_out);
    nz |= reconstruct_uv(it, enc, rd, &mut yuv_out[idx(U_OFF_ENC)..], uv_mode);
    it.yuv_out = yuv_out;

    #[expect(clippy::cast_sign_loss, reason = "nz is a bit set")]
    {
        rd.nz = nz as u32;
    }
    rd.score = best_score;
}

//------------------------------------------------------------------------------
// Entry point

// upstream: src/enc/quant_enc.c VP8Decimate
/// Pick best modes and fills the levels. Returns true if skipped.
pub fn vp8_decimate(
    it: &mut VP8EncIterator,
    enc: &mut VP8Encoder,
    rd: &mut VP8ModeScore,
    rd_opt: VP8RDLevel,
) -> bool {
    let method = enc.method;

    init_score(rd);

    // We can perform predictions for Luma16x16 and Chroma8x8 already.
    // Luma4x4 predictions needs to be done as-we-go.
    vp8_make_luma16_preds(it, enc);
    vp8_make_chroma8_preds(it, enc);

    if rd_opt > VP8RDLevel::RD_OPT_NONE {
        it.do_trellis = rd_opt >= VP8RDLevel::RD_OPT_TRELLIS_ALL;
        pick_best_intra16(it, enc, rd);
        if method >= 2 {
            pick_best_intra4(it, enc, rd);
        }
        pick_best_uv(it, enc, rd);
        if rd_opt == VP8RDLevel::RD_OPT_TRELLIS {
            // finish off with trellis-optim now
            it.do_trellis = true;
            simple_quantize(it, enc, rd);
        }
    } else {
        // At this point we have heuristically decided intra16 / intra4.
        // For method >= 2, pick the best intra4/intra16 based on SSE (~tad slower).
        // For method <= 1, we don't re-examine the decision but just go ahead with
        // quantization/reconstruction.
        refine_using_distortion(it, enc, method >= 2, method >= 1, rd);
    }
    let is_skipped = rd.nz == 0;
    vp8_set_skip(it, &mut mode_rows(enc), i32::from(is_skipped));
    is_skipped
}
