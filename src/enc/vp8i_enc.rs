// libwebp v1.6.0, src/enc/vp8i_enc.h:
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
//   WebP encoder: internal header.
//
// Author: Skal (pascal.massimino@gmail.com)

//! Translation of `src/enc/vp8i_enc.h` from libwebp v1.6.0.
//!
//! Work package: WP10 (VP8 encoder core).
//!
//! # Representation
//!
//! 1. **Statistics.** The fields that `StoreStats` (`src/enc/webp_enc.rs`)
//!    copies stay encoder fields; where C writes `pic->stats` directly, the
//!    Rust code does too.
//!
//! # Deviations from upstream
//!
//! Changes of representation that the ADRs allow, or behavior that the Rust
//! side cannot reproduce exactly. Each records the reason.
//!
//! 1. **Encoder memory.** C carves `mb_info`, `preds`, `nz`, `lf_stats`,
//!    `y_top`, `uv_top` and `top_derr` from one block behind the encoder; here
//!    they are vectors, allocated with a fallible reservation after the size
//!    test of `WebPSafeMalloc` on C's total (with `sizeof(VP8Encoder)` and the
//!    `WEBP_ALIGN_CST` padding). The pointers are positions: C's `enc->preds`
//!    is `preds_base()` in the whole `preds` plane, `enc->nz` is position 1 of
//!    `nz` (`nz[-1]` is position 0), and `uv_top` is a position in `y_top`.
//!    `lf_stats` and `top_derr` are `Option` (C's `NULL`), `proba` is boxed,
//!    and `InitVP8Encoder` returns
//!    `Result<Box<VP8Encoder>, WebPEncodingError>`. Rust zero-initializes the
//!    arrays that C leaves uninitialized; every element is written before it is
//!    read on the compared inputs.
//! 2. **Iterator.** `VP8EncIterator` holds positions instead of pointers: `mb`,
//!    `bw`, `preds`, `nz`, `y_top`, `uv_top` into the encoder, `y_left`,
//!    `u_left`, `v_left` into `yuv_left_mem`, `i4_top` into `i4_boundary`.
//!    `yuv_in`, `yuv_out`, `yuv_out2` and `yuv_p` are four buffers instead of
//!    four parts of `yuv_mem`, so `SwapOut` swaps two buffers. A function that
//!    C gives a destination inside the iterator (`ReconstructIntra16`,
//!    `ReconstructUV`, the loop of `PickBestIntra4`) gets the buffer moved out
//!    of the iterator for the call. The 32-byte scratch that the analysis
//!    passes to `VP8IteratorImport` is the iterator's `tmp_32`, and
//!    `top_in_tmp_32` says whether `y_top` and `uv_top` point there (C's
//!    argument is a flag here). `lf_stats` and `top_derr` record whether the
//!    encoder has them. `i4_boundary` has 40 entries on AArch64 and 37
//!    elsewhere, as C's `WEBP_AARCH64 && BPS == 32` decides. `Default` gives
//!    zeroed buffers where C declares an uninitialized iterator.
//!
//! # Checks not applicable
//!
//! These checks do not apply, or cannot be made, with the reason:
//!
//! 1. **32-bit C size.** `VP8Encoder::C_SIZE` is 23,704 on the 64-bit hosts,
//!    which the constants probe confirms (`tests/wp10_constants.rs`), and
//!    22,680 for i386 (from the C compiler's i386 target); 32-bit targets are
//!    not run.

use crate::dec::common_dec::{
    MAX_NUM_PARTITIONS, NUM_BANDS, NUM_CTX, NUM_MB_SEGMENTS, NUM_PROBAS, NUM_TYPES,
};
use crate::dsp::dsp::BPS;
use crate::enc::alpha_enc::VP8EncAlphaJob;
use crate::utils::bit_writer_utils::VP8BitWriter;
use crate::utils::thread_utils::WebPWorker;

//------------------------------------------------------------------------------
// Various defines and enums

// upstream: src/enc/vp8i_enc.h ENC_MAJ_VERSION
pub const ENC_MAJ_VERSION: i32 = 1;
// upstream: src/enc/vp8i_enc.h ENC_MIN_VERSION
pub const ENC_MIN_VERSION: i32 = 6;
// upstream: src/enc/vp8i_enc.h ENC_REV_VERSION
pub const ENC_REV_VERSION: i32 = 0;

// upstream: src/enc/vp8i_enc.h MAX_LF_LEVELS
/// Maximum loop filter level.
pub const MAX_LF_LEVELS: i32 = 64;
// upstream: src/enc/vp8i_enc.h MAX_VARIABLE_LEVEL
/// Last (inclusive) level with variable cost.
pub const MAX_VARIABLE_LEVEL: i32 = 67;
// upstream: src/enc/vp8i_enc.h MAX_LEVEL
/// Max level (note: max codable is 2047 + 67).
pub const MAX_LEVEL: i32 = 2047;

// upstream: src/enc/vp8i_enc.h VP8RDLevel
/// Rate-distortion optimization levels (C: `VP8RDLevel`).
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VP8RDLevel(pub u32);

impl VP8RDLevel {
    // upstream: src/enc/vp8i_enc.h RD_OPT_NONE
    /// No rd-opt.
    pub const RD_OPT_NONE: Self = Self(0);
    // upstream: src/enc/vp8i_enc.h RD_OPT_BASIC
    /// Basic scoring (no trellis).
    pub const RD_OPT_BASIC: Self = Self(1);
    // upstream: src/enc/vp8i_enc.h RD_OPT_TRELLIS
    /// Perform trellis-quant on the final decision only.
    pub const RD_OPT_TRELLIS: Self = Self(2);
    // upstream: src/enc/vp8i_enc.h RD_OPT_TRELLIS_ALL
    /// Trellis-quant for every scoring (much slower).
    pub const RD_OPT_TRELLIS_ALL: Self = Self(3);
}

// YUV-cache parameters. Cache is 32-bytes wide (= one cacheline).
// The original or reconstructed samples can be accessed using VP8Scan[].
// The predicted blocks can be accessed using offsets to 'yuv_p' and
// the arrays VP8*ModeOffsets[].
// * YUV Samples area ('yuv_in'/'yuv_out'/'yuv_out2')
//   (see VP8Scan[] for accessing the blocks, along with
//   Y_OFF_ENC/U_OFF_ENC/V_OFF_ENC):
//             +----+----+
//  Y_OFF_ENC  |YYYY|UUVV|
//  U_OFF_ENC  |YYYY|UUVV|
//  V_OFF_ENC  |YYYY|....| <- 25% wasted U/V area
//             |YYYY|....|
//             +----+----+
// * Prediction area ('yuv_p', size = PRED_SIZE_ENC)
//   Intra16 predictions (16x16 block each, two per row):
//         |I16DC16|I16TM16|
//         |I16VE16|I16HE16|
//   Chroma U/V predictions (16x8 block each, two per row):
//         |C8DC8|C8TM8|
//         |C8VE8|C8HE8|
//   Intra 4x4 predictions (4x4 block each)
//         |I4DC4 I4TM4 I4VE4 I4HE4|I4RD4 I4VR4 I4LD4 I4VL4|
//         |I4HD4 I4HU4 I4TMP .....|.......................| <- ~31% wasted

// upstream: src/enc/vp8i_enc.h YUV_SIZE_ENC
pub const YUV_SIZE_ENC: i32 = BPS * 16;
// upstream: src/enc/vp8i_enc.h PRED_SIZE_ENC
/// I16+Chroma+I4 preds.
pub const PRED_SIZE_ENC: i32 = 32 * BPS + 16 * BPS + 8 * BPS;
// upstream: src/enc/vp8i_enc.h Y_OFF_ENC
pub const Y_OFF_ENC: i32 = 0;
// upstream: src/enc/vp8i_enc.h U_OFF_ENC
pub const U_OFF_ENC: i32 = 16;
// upstream: src/enc/vp8i_enc.h V_OFF_ENC
pub const V_OFF_ENC: i32 = 16 + 8;

// Layout of prediction blocks
// intra 16x16
// upstream: src/enc/vp8i_enc.h I16DC16
pub const I16DC16: i32 = 0;
// upstream: src/enc/vp8i_enc.h I16TM16
pub const I16TM16: i32 = I16DC16 + 16;
// upstream: src/enc/vp8i_enc.h I16VE16
pub const I16VE16: i32 = 16 * BPS;
// upstream: src/enc/vp8i_enc.h I16HE16
pub const I16HE16: i32 = I16VE16 + 16;
// chroma 8x8, two U/V blocks side by side (hence: 16x8 each)
// upstream: src/enc/vp8i_enc.h C8DC8
pub const C8DC8: i32 = 2 * 16 * BPS;
// upstream: src/enc/vp8i_enc.h C8TM8
pub const C8TM8: i32 = C8DC8 + 16;
// upstream: src/enc/vp8i_enc.h C8VE8
pub const C8VE8: i32 = 2 * 16 * BPS + 8 * BPS;
// upstream: src/enc/vp8i_enc.h C8HE8
pub const C8HE8: i32 = C8VE8 + 16;
// intra 4x4
// upstream: src/enc/vp8i_enc.h I4DC4
pub const I4DC4: i32 = 3 * 16 * BPS;
// upstream: src/enc/vp8i_enc.h I4TM4
pub const I4TM4: i32 = I4DC4 + 4;
// upstream: src/enc/vp8i_enc.h I4VE4
pub const I4VE4: i32 = I4DC4 + 8;
// upstream: src/enc/vp8i_enc.h I4HE4
pub const I4HE4: i32 = I4DC4 + 12;
// upstream: src/enc/vp8i_enc.h I4RD4
pub const I4RD4: i32 = I4DC4 + 16;
// upstream: src/enc/vp8i_enc.h I4VR4
pub const I4VR4: i32 = I4DC4 + 20;
// upstream: src/enc/vp8i_enc.h I4LD4
pub const I4LD4: i32 = I4DC4 + 24;
// upstream: src/enc/vp8i_enc.h I4VL4
pub const I4VL4: i32 = I4DC4 + 28;
// upstream: src/enc/vp8i_enc.h I4HD4
pub const I4HD4: i32 = 3 * 16 * BPS + 4 * BPS;
// upstream: src/enc/vp8i_enc.h I4HU4
pub const I4HU4: i32 = I4HD4 + 4;
// upstream: src/enc/vp8i_enc.h I4TMP
pub const I4TMP: i32 = I4HD4 + 8;

// upstream: src/enc/vp8i_enc.h score_t
/// Type used for scores, rate, distortion.
#[expect(non_camel_case_types, reason = "C type name")]
pub type score_t = i64;

// upstream: src/enc/vp8i_enc.h MAX_COST
/// Note that `MAX_COST` is not the maximum allowed by `sizeof(score_t)`, in
/// order to allow overflowing computations.
pub const MAX_COST: score_t = 0x007f_ffff_ffff_ffff;

// upstream: src/enc/vp8i_enc.h QFIX
pub const QFIX: i32 = 17;

// upstream: src/enc/vp8i_enc.h BIAS
/// The rounding bias `b`, in `QFIX` fixed point (C: the macro `BIAS`).
#[must_use]
pub const fn bias(b: i32) -> i32 {
    b << (QFIX - 8)
}

// upstream: src/enc/vp8i_enc.h QUANTDIV
/// Fun fact: this is the _only_ line where we're actually being lossy and
/// discarding bits. The `uint32_t` arithmetic wraps, as in C.
#[must_use]
#[expect(
    clippy::cast_possible_wrap,
    reason = "C converts the uint32_t quotient to int"
)]
pub const fn quantdiv(n: u32, i_q: u32, b: u32) -> i32 {
    (n.wrapping_mul(i_q).wrapping_add(b) >> QFIX) as i32
}

// upstream: src/enc/vp8i_enc.h ERROR_DIFFUSION_QUALITY
/// Quality below which error-diffusion is enabled.
pub const ERROR_DIFFUSION_QUALITY: i32 = 98;

//------------------------------------------------------------------------------
// Headers

// upstream: src/enc/vp8i_enc.h proba_t
/// 16b + 16b.
#[expect(non_camel_case_types, reason = "C type name")]
pub type proba_t = u32;

// upstream: src/enc/vp8i_enc.h ProbaArray
pub type ProbaArray = [[u8; NUM_PROBAS as usize]; NUM_CTX as usize];

// upstream: src/enc/vp8i_enc.h StatsArray
pub type StatsArray = [[proba_t; NUM_PROBAS as usize]; NUM_CTX as usize];

// upstream: src/enc/vp8i_enc.h CostArray
pub type CostArray = [[u16; MAX_VARIABLE_LEVEL as usize + 1]; NUM_CTX as usize];

// upstream: src/enc/vp8i_enc.h CostArrayMap
/// The remapped cost tables of one coefficient type. C stores, for each
/// coefficient position and context, a pointer to the row
/// `level_cost[type][band][ctx]`; the Rust table stores the `band` of that row,
/// which is the position the pointer stands for (ADR 0003, pointers).
pub type CostArrayMap = [[u8; NUM_CTX as usize]; 16];

// upstream: src/enc/vp8i_enc.h CostArrayPtr
/// The cost rows of one coefficient type, indexed as C indexes its
/// `CostArrayPtr` (`costs[n][ctx]`): the cost tables of the type and their
/// remapping by coefficient position.
#[derive(Clone, Copy, Debug)]
pub struct CostArrayPtr<'a> {
    /// The level costs of the coefficient type (C: `level_cost[type]`).
    pub level_cost: &'a [CostArray; NUM_BANDS as usize],
    /// The band of each remapped row (C: `remapped_costs[type]`).
    pub remapped: &'a CostArrayMap,
}

impl<'a> CostArrayPtr<'a> {
    /// The cost row that C reads as `costs[n][ctx]`.
    #[must_use]
    pub fn row(&self, n: usize, ctx: usize) -> &'a [u16; MAX_VARIABLE_LEVEL as usize + 1] {
        &self.level_cost[usize::from(self.remapped[n][ctx])][ctx]
    }
}

// upstream: src/enc/vp8i_enc.h LFStats
/// Filter stats.
pub type LFStats = [[f64; MAX_LF_LEVELS as usize]; NUM_MB_SEGMENTS as usize];

/// The C size of `LFStats`, for the allocation size of `InitVP8Encoder`.
pub const LF_STATS_C_SIZE: usize = 8 * MAX_LF_LEVELS as usize * NUM_MB_SEGMENTS as usize;

// upstream: src/enc/vp8i_enc.h VP8EncSegmentHeader
/// Segment features.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct VP8EncSegmentHeader {
    /// Actual number of segments. 1 segment only = unused.
    pub num_segments: i32,
    /// Whether to update the segment map or not. Must be 0 if there's only 1
    /// segment.
    pub update_map: bool,
    /// Bit-cost for transmitting the segment map.
    pub size: i32,
}

// upstream: src/enc/vp8i_enc.h VP8EncProba
/// Struct collecting all frame-persistent probabilities.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VP8EncProba {
    /// Probabilities for segment tree.
    pub segments: [u8; 3],
    /// Final probability of being skipped.
    pub skip_proba: u8,
    /// 1056 bytes.
    pub coeffs: [[ProbaArray; NUM_BANDS as usize]; NUM_TYPES as usize],
    /// 4224 bytes.
    pub stats: [[StatsArray; NUM_BANDS as usize]; NUM_TYPES as usize],
    /// 13056 bytes.
    pub level_cost: [[CostArray; NUM_BANDS as usize]; NUM_TYPES as usize],
    /// 1536 bytes in C, as pointers (see [`CostArrayMap`]).
    pub remapped_costs: [CostArrayMap; NUM_TYPES as usize],
    /// If true, need to call `VP8CalculateLevelCosts()`.
    pub dirty: bool,
    /// Note: we always use `skip_proba` for now.
    pub use_skip_proba: bool,
    /// Number of skipped blocks.
    pub nb_skip: i32,
}

impl Default for VP8EncProba {
    fn default() -> Self {
        Self {
            segments: [0; 3],
            skip_proba: 0,
            coeffs: [[[[0; NUM_PROBAS as usize]; NUM_CTX as usize]; NUM_BANDS as usize];
                NUM_TYPES as usize],
            stats: [[[[0; NUM_PROBAS as usize]; NUM_CTX as usize]; NUM_BANDS as usize];
                NUM_TYPES as usize],
            level_cost: [[[[0; MAX_VARIABLE_LEVEL as usize + 1]; NUM_CTX as usize];
                NUM_BANDS as usize]; NUM_TYPES as usize],
            remapped_costs: [[[0; NUM_CTX as usize]; 16]; NUM_TYPES as usize],
            dirty: false,
            use_skip_proba: false,
            nb_skip: 0,
        }
    }
}

impl VP8EncProba {
    /// The level costs of `coeff_type`, remapped by coefficient position (C: the
    /// `CostArrayPtr` cast of `remapped_costs[coeff_type]`).
    #[must_use]
    pub fn costs(&self, coeff_type: usize) -> CostArrayPtr<'_> {
        CostArrayPtr {
            level_cost: &self.level_cost[coeff_type],
            remapped: &self.remapped_costs[coeff_type],
        }
    }
}

// upstream: src/enc/vp8i_enc.h VP8EncFilterHeader
/// Filter parameters. Not actually used in the code (we don't perform the
/// in-loop filtering), but filled from user's config.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct VP8EncFilterHeader {
    /// Filtering type: 0=complex, 1=simple.
    pub simple: i32,
    /// Base filter level [0..63].
    pub level: i32,
    /// [0..7].
    pub sharpness: i32,
    /// Delta filter level for i4x4 relative to i16x16.
    pub i4x4_lf_delta: i32,
}

//------------------------------------------------------------------------------
// Informations about the macroblocks.

// upstream: src/enc/vp8i_enc.h VP8MBInfo
/// Information about one macroblock. C packs `type`, `uv_mode`, `skip` and
/// `segment` into bit-fields of 2, 2, 1 and 2 bits; every Rust assignment keeps
/// only those low bits, as a C bit-field assignment does.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct VP8MBInfo {
    /// Block type: 0=i4x4, 1=i16x16.
    pub r#type: u8,
    pub uv_mode: u8,
    pub skip: u8,
    pub segment: u8,
    /// Quantization-susceptibility.
    pub alpha: u8,
}

impl VP8MBInfo {
    /// The C size of `VP8MBInfo` (one `unsigned int` of bit-fields with the
    /// `uint8_t` in its second byte), for the allocation size of `InitVP8Encoder`.
    pub const C_SIZE: usize = 4;
}

// upstream: src/enc/vp8i_enc.h VP8Matrix
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct VP8Matrix {
    /// Quantizer steps.
    pub q: [u16; 16],
    /// Reciprocals, fixed point.
    pub iq: [u16; 16],
    /// Rounding bias.
    pub bias: [u32; 16],
    /// Value below which a coefficient is zeroed.
    pub zthresh: [u32; 16],
    /// Frequency boosters for slight sharpening.
    pub sharpen: [u16; 16],
}

// upstream: src/enc/vp8i_enc.h VP8SegmentInfo
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct VP8SegmentInfo {
    /// Quantization matrices.
    pub y1: VP8Matrix,
    pub y2: VP8Matrix,
    pub uv: VP8Matrix,
    /// Quant-susceptibility, range [-127,127]. Zero is neutral. Lower values
    /// indicate a lower risk of blurriness.
    pub alpha: i32,
    /// Filter-susceptibility, range [0,255].
    pub beta: i32,
    /// Final segment quantizer.
    pub quant: i32,
    /// Final in-loop filtering strength.
    pub fstrength: i32,
    /// Max edge delta (for filtering strength).
    pub max_edge: i32,
    /// Minimum distortion required to trigger filtering record.
    pub min_disto: i32,
    // reactivities
    pub lambda_i16: i32,
    pub lambda_i4: i32,
    pub lambda_uv: i32,
    pub lambda_mode: i32,
    pub lambda_trellis: i32,
    pub tlambda: i32,
    pub lambda_trellis_i16: i32,
    pub lambda_trellis_i4: i32,
    pub lambda_trellis_uv: i32,
    // lambda values for distortion-based evaluation
    /// Penalty for using Intra4.
    pub i4_penalty: score_t,
}

// upstream: src/enc/vp8i_enc.h DError
/// [u/v][top or left].
pub type DError = [[i8; 2]; 2];

/// The C size of `DError`, for the allocation size of `InitVP8Encoder`.
pub const DERROR_C_SIZE: usize = 4;

// upstream: src/enc/vp8i_enc.h VP8ModeScore
/// Handy transient struct to accumulate score and info during RD-optimization
/// and mode evaluation. The C fields `D`, `SD`, `H` and `R` are `d`, `sd`, `h`
/// and `r` (ADR 0001).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct VP8ModeScore {
    /// Distortion, spectral distortion.
    pub d: score_t,
    pub sd: score_t,
    /// Header bits, rate, score.
    pub h: score_t,
    pub r: score_t,
    pub score: score_t,
    /// Quantized levels for luma-DC, luma-AC, chroma.
    pub y_dc_levels: [i16; 16],
    pub y_ac_levels: [[i16; 16]; 16],
    pub uv_levels: [[i16; 16]; 4 + 4],
    /// Mode number for intra16 prediction.
    pub mode_i16: i32,
    /// Mode numbers for intra4 predictions.
    pub modes_i4: [u8; 16],
    /// Mode number of chroma prediction.
    pub mode_uv: i32,
    /// Non-zero blocks.
    pub nz: u32,
    /// DC diffusion errors for U/V for blocks #1/2/3.
    pub derr: [[i8; 3]; 2],
}

/// The size of the intra 4x4 boundary of the iterator: 32+8 samples on `AArch64`,
/// where the NEON predictions read three more, and 32+5 elsewhere (C: the
/// `WEBP_AARCH64 && BPS == 32` condition on `i4_boundary`).
pub const I4_BOUNDARY_SIZE: usize = if cfg!(target_arch = "aarch64") {
    40
} else {
    37
};

/// The size of the left samples of the iterator: 17 + 16 + 16 + 8 (C:
/// `yuv_left_mem` without the `WEBP_ALIGN_CST` alignment slack).
pub const YUV_LEFT_MEM_SIZE: usize = 17 + 16 + 16 + 8;

// upstream: src/enc/vp8i_enc.h VP8EncIterator
/// Iterator structure to iterate through macroblocks, pointing to the right
/// neighbouring data (samples, predictions, contexts, ...).
///
/// The iterator holds no pointer to its encoder: the functions that read or
/// write the encoder take it per call (C: `enc`). The C pointers into the
/// encoder's memory are positions: `mb` in `mb_info`, `bw` in `parts`, `preds`
/// in `preds`, `nz` in `nz` and `y_top`/`uv_top` in `y_top`. `lf_stats` and
/// `top_derr` only record whether the encoder has them (C: a pointer to the
/// encoder's array or `NULL`). C carves `yuv_in`, `yuv_out`, `yuv_out2` and
/// `yuv_p` from `yuv_mem`; here they are four buffers, and swapping
/// `yuv_out` with `yuv_out2` swaps the buffers. `y_left`, `u_left` and `v_left`
/// are positions in `yuv_left_mem`. The 32-byte scratch that `VP8IteratorImport`
/// receives from the analysis is `tmp_32`, owned here; `top_in_tmp_32` tells
/// whether `y_top` and `uv_top` point into it.
#[derive(Clone, Debug, PartialEq, Eq)]
#[expect(
    clippy::struct_excessive_bools,
    reason = "the C int flags lf_stats, do_trellis and top_derr, and the scratch flag"
)]
pub struct VP8EncIterator {
    /// Current macroblock.
    pub x: i32,
    pub y: i32,
    /// Input samples.
    pub yuv_in: Vec<u8>,
    /// Output samples.
    pub yuv_out: Vec<u8>,
    /// Secondary buffer swapped with `yuv_out`.
    pub yuv_out2: Vec<u8>,
    /// Scratch buffer for prediction.
    pub yuv_p: Vec<u8>,
    /// Current macroblock: index into the encoder's `mb_info`.
    pub mb: usize,
    /// Current bit-writer: index into the encoder's `parts`.
    pub bw: usize,
    /// Intra mode predictors (4x4 blocks): position in the encoder's `preds`.
    pub preds: usize,
    /// Non-zero pattern: position in the encoder's `nz`.
    pub nz: usize,
    /// 32+5 boundary samples needed by intra 4x4 (32+8 on `AArch64`).
    pub i4_boundary: [u8; I4_BOUNDARY_SIZE],
    /// Position of the current top boundary sample in `i4_boundary`.
    pub i4_top: usize,
    /// Current intra4x4 mode being tested.
    pub i4: i32,
    /// Top-non-zero context.
    pub top_nz: [i32; 9],
    /// Left-non-zero. `left_nz[8]` is independent.
    pub left_nz: [i32; 9],
    /// Bit counters for coded levels.
    pub bit_count: [[u64; 3]; 4],
    /// Macroblock bit-cost for luma.
    pub luma_bits: u64,
    /// Macroblock bit-cost for chroma.
    pub uv_bits: u64,
    /// Whether the encoder collects filter stats (C: borrowed from enc).
    pub lf_stats: bool,
    /// If true, perform extra level optimisation.
    pub do_trellis: bool,
    /// Number of mb still to be processed.
    pub count_down: i32,
    /// Starting counter value (for progress).
    pub count_down0: i32,
    /// Saved initial progress percent.
    pub percent0: i32,
    /// Left error diffusion (u/v).
    pub left_derr: DError,
    /// Whether the encoder diffuses the top error (C: `NULL` if disabled).
    pub top_derr: bool,
    /// Left luma samples (addressable from index -1 to 15): position in
    /// `yuv_left_mem`.
    pub y_left: usize,
    /// Left u samples (addressable from index -1 to 7).
    pub u_left: usize,
    /// Left v samples (addressable from index -1 to 7).
    pub v_left: usize,
    /// Top luma samples at position 'x'.
    pub y_top: usize,
    /// Top u/v samples at position 'x', packed as 16 bytes.
    pub uv_top: usize,
    /// Memory for storing `y_left`, `u_left` and `v_left`.
    pub yuv_left_mem: [u8; YUV_LEFT_MEM_SIZE],
    /// The scratch of `VP8IteratorImport` (C: its `tmp_32` argument).
    pub tmp_32: [u8; 32],
    /// Whether `y_top` and `uv_top` are positions in `tmp_32`.
    pub top_in_tmp_32: bool,
}

impl Default for VP8EncIterator {
    /// An iterator with zeroed buffers, before `VP8IteratorInit` (C: an
    /// uninitialized stack object).
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            yuv_in: vec![0; YUV_SIZE_ENC as usize],
            yuv_out: vec![0; YUV_SIZE_ENC as usize],
            yuv_out2: vec![0; YUV_SIZE_ENC as usize],
            yuv_p: vec![0; PRED_SIZE_ENC as usize],
            mb: 0,
            bw: 0,
            preds: 0,
            nz: 0,
            i4_boundary: [0; I4_BOUNDARY_SIZE],
            i4_top: 0,
            i4: 0,
            top_nz: [0; 9],
            left_nz: [0; 9],
            bit_count: [[0; 3]; 4],
            luma_bits: 0,
            uv_bits: 0,
            lf_stats: false,
            do_trellis: false,
            count_down: 0,
            count_down0: 0,
            percent0: 0,
            left_derr: [[0; 2]; 2],
            top_derr: false,
            y_left: 0,
            u_left: 0,
            v_left: 0,
            y_top: 0,
            uv_top: 0,
            yuv_left_mem: [0; YUV_LEFT_MEM_SIZE],
            tmp_32: [0; 32],
            top_in_tmp_32: false,
        }
    }
}

/// The macroblock infos and intra modes that the mode setters write (C: the
/// memory behind `it->mb` and `it->preds`). Normally these are the encoder's
/// whole `mb_info` and `preds` arrays; a job of the threaded analysis holds only
/// its own rows, which start at macroblock `mb_start` and at byte `preds_start`
/// of the encoder's arrays, so the iterator's positions stay those of C.
#[derive(Debug)]
pub struct ModeRows<'a> {
    /// The macroblock infos from index `mb_start`.
    pub mb_info: &'a mut [VP8MBInfo],
    pub mb_start: usize,
    /// The intra modes from position `preds_start`.
    pub preds: &'a mut [u8],
    pub preds_start: usize,
    /// The stride of the intra modes (C: `enc->preds_w`).
    pub preds_w: i32,
}

impl<'a> ModeRows<'a> {
    /// The encoder's whole arrays.
    pub fn whole(mb_info: &'a mut [VP8MBInfo], preds: &'a mut [u8], preds_w: i32) -> Self {
        Self {
            mb_info,
            mb_start: 0,
            preds,
            preds_start: 0,
            preds_w,
        }
    }

    /// The macroblock info at index `mb` of the encoder's array.
    pub fn mb(&mut self, mb: usize) -> &mut VP8MBInfo {
        &mut self.mb_info[mb - self.mb_start]
    }

    /// The intra modes from position `pos` of the encoder's array.
    pub fn preds_at(&mut self, pos: usize) -> &mut [u8] {
        &mut self.preds[pos - self.preds_start..]
    }
}

//------------------------------------------------------------------------------
// Paginated token buffer

// upstream: src/enc/vp8i_enc.h VP8TBuffer
/// The token pages and the write state of the current page. C links the pages
/// through their `next` pointer and keeps `last_page` and `tokens` as pointers
/// to the last page; here the pages are a vector in the same order, the last
/// page is its last element, and `tokens` is that page.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct VP8TBuffer {
    /// The pages, first page first (C: `pages` and the `next` chain).
    pub pages: Vec<Vec<u16>>,
    /// How many free tokens left before the page is full.
    pub left: i32,
    /// Number of tokens per page.
    pub page_size: i32,
    /// True in case of malloc error.
    pub error: bool,
}

//------------------------------------------------------------------------------
// VP8Encoder

// upstream: src/enc/vp8i_enc.h VP8Encoder
/// The VP8 encoder. It stores neither `config` nor `pic`: every function that
/// reads them takes `config: &WebPConfig` and `pic: &mut WebPPicture` per call
/// (ADR 0003, context per call, as WP11 does for `VP8LEncoder`).
///
/// C carves `mb_info`, `preds`, `nz`, `lf_stats`, `y_top`, `uv_top` and
/// `top_derr` from the allocation of the encoder; here they are separate
/// vectors, and the C pointers into them are positions:
///
/// - `preds` holds the whole `preds_w * preds_h` plane; C's `enc->preds` is
///   position [`VP8Encoder::preds_base`] (one row and one column in).
/// - `nz` holds `mb_w + 1` words; C's `enc->nz` is position 1, so that
///   `enc->nz[-1]` is position 0.
/// - `y_top` holds the top luma samples followed by the top U/V samples; C's
///   `enc->uv_top` is position `uv_top`.
/// - `lf_stats` and `top_derr` are `None` where C has `NULL`.
///
/// The alpha data and statistics that the alpha worker produces move into the
/// encoder at `Sync` (see `alpha_enc.rs`).
pub struct VP8Encoder {
    // headers
    /// Filtering information.
    pub filter_hdr: VP8EncFilterHeader,
    /// Segment information.
    pub segment_hdr: VP8EncSegmentHeader,

    /// VP8's profile, deduced from Config.
    pub profile: i32,

    /// Dimension, in macroblock units.
    pub mb_w: i32,
    pub mb_h: i32,
    /// Stride of the `*preds` prediction plane (`4 * mb_w + 1`).
    pub preds_w: i32,

    /// Number of partitions (1, 2, 4 or 8 = `MAX_NUM_PARTITIONS`).
    pub num_parts: i32,

    /// Per-partition boolean decoders: part0.
    pub bw: VP8BitWriter,
    /// Token partitions.
    pub parts: [VP8BitWriter; MAX_NUM_PARTITIONS as usize],
    /// Token buffer.
    pub tokens: VP8TBuffer,

    /// For progress.
    pub percent: i32,

    // transparency blob
    pub has_alpha: bool,
    /// Non-empty if transparency is present (C: non-`NULL`).
    pub alpha_data: Vec<u8>,
    pub alpha_data_size: u32,
    /// The worker of the alpha job; it owns the job from `Launch` to `Sync`.
    pub alpha_worker: WebPWorker<VP8EncAlphaJob, ()>,

    /// Quantization info (one set of DC/AC dequant factor per segment).
    pub dqm: [VP8SegmentInfo; NUM_MB_SEGMENTS as usize],
    /// Nominal quantizer value. Only used for relative coding of segments'
    /// quant.
    pub base_quant: i32,
    /// Global susceptibility (<=> complexity).
    pub alpha: i32,
    /// U/V quantization susceptibility.
    pub uv_alpha: i32,
    // global offset of quantizers, shared by all segments
    pub dq_y1_dc: i32,
    pub dq_y2_dc: i32,
    pub dq_y2_ac: i32,
    pub dq_uv_dc: i32,
    pub dq_uv_ac: i32,

    // probabilities and statistics
    pub proba: Box<VP8EncProba>,
    /// Sum of Y/U/V/A squared errors for all macroblocks.
    pub sse: [u64; 4],
    /// Pixel count for the `sse[]` stats.
    pub sse_count: u64,
    pub coded_size: i32,
    pub residual_bytes: [[i32; 4]; 3],
    pub block_count: [i32; 3],

    // quality/speed settings
    /// 0=fastest, 6=best/slowest.
    pub method: i32,
    /// Deduced from method.
    pub rd_opt_level: VP8RDLevel,
    /// Partition #0 safeness factor.
    pub max_i4_header_bits: i32,
    /// Rough limit for header bits per MB.
    pub mb_header_limit: i32,
    /// Derived from `config->thread_level`.
    pub thread_level: i32,
    /// Derived from `config->target_XXX`.
    pub do_search: bool,
    /// If true, use token buffer.
    pub use_tokens: bool,

    // Memory
    /// Contextual macroblock infos (`mb_w * mb_h`).
    pub mb_info: Vec<VP8MBInfo>,
    /// Predictions modes: `(4 * mb_w + 1) * (4 * mb_h + 1)`.
    pub preds: Vec<u8>,
    /// Non-zero bit context: `mb_w + 1`.
    pub nz: Vec<u32>,
    /// Top luma samples, then the top u/v samples.
    pub y_top: Vec<u8>,
    /// Top u/v samples: position in `y_top`. U and V are packed into 16 bytes
    /// (8 U + 8 V).
    pub uv_top: usize,
    /// Autofilter stats (if `None`, autofilter is off).
    pub lf_stats: Option<Box<LFStats>>,
    /// Diffusion error (`None` if disabled).
    pub top_derr: Option<Vec<DError>>,
}

impl VP8Encoder {
    /// The C size of `VP8Encoder`, for the allocation size of `InitVP8Encoder`:
    /// 23704 bytes on the 64-bit targets (compared with the C compiler by the
    /// constants probe) and 22680 on i386.
    pub const C_SIZE: usize = if size_of::<usize>() == 8 {
        23704
    } else {
        22680
    };

    /// The position of C's `enc->preds` in `preds`: one row and one column into
    /// the plane (C: `mem + 1 + enc->preds_w`).
    #[must_use]
    #[expect(
        clippy::cast_sign_loss,
        reason = "preds_w is positive for every encoder"
    )]
    pub fn preds_base(&self) -> usize {
        self.preds_w as usize + 1
    }
}
