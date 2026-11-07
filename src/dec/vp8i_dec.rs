// libwebp v1.6.0, src/dec/vp8i_dec.h:
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
// VP8 decoder: internal header.
//
// Author: Skal (pascal.massimino@gmail.com)

//! Translation of `src/dec/vp8i_dec.h` from libwebp v1.6.0.
//!
//! Work package: WP6 (VP8 decoder core).
//!
//! The decoder's pointers into its memory chunk (`dec->mem`) are separate owned
//! vectors, positions or indexes (ADR 0003, "Pointers" and "Ownership"); the
//! size test of the one C allocation is applied to the C total before they are
//! allocated (`frame_dec.c`, `AllocateMemory`). The data that `FinishRow` and,
//! for `mt_method == 2`, `ReconstructRow` mutate on the worker thread is
//! [`VP8RowJob`], which moves into the worker at `Launch` and returns at `Sync`
//! (ADR 0003, "Threads"; `docs/status/wp6.md` describes the split). The
//! functions that this header declares are translated where C defines them.

use crate::dec::alphai_dec::ALPHDecoder;
use crate::dec::common_dec::{
    MAX_NUM_PARTITIONS, MB_FEATURE_TREE_PROBS, NUM_BANDS, NUM_CTX, NUM_MB_SEGMENTS,
    NUM_MODE_LF_DELTAS, NUM_PROBAS, NUM_REF_LF_DELTAS, NUM_TYPES,
};
use crate::dec::vp8_dec::VP8Io;
use crate::dsp::dsp::BPS;
use crate::utils::bit_reader_utils::VP8BitReader;
use crate::utils::random_utils::{VP8_RANDOM_TABLE_SIZE, VP8Random};
use crate::utils::thread_utils::WebPWorker;
use crate::webp::decode::VP8StatusCode;

//------------------------------------------------------------------------------
// Various defines and enums

// upstream: src/dec/vp8i_dec.h DEC_MAJ_VERSION
/// Major version of the decoder.
pub const DEC_MAJ_VERSION: i32 = 1;
// upstream: src/dec/vp8i_dec.h DEC_MIN_VERSION
/// Minor version of the decoder.
pub const DEC_MIN_VERSION: i32 = 6;
// upstream: src/dec/vp8i_dec.h DEC_REV_VERSION
/// Revision of the decoder.
pub const DEC_REV_VERSION: i32 = 0;

// YUV-cache parameters. Cache is 32-bytes wide (= one cacheline).
// Constraints are: We need to store one 16x16 block of luma samples (y),
// and two 8x8 chroma blocks (u/v). These are better be 16-bytes aligned,
// in order to be SIMD-friendly. We also need to store the top, left and
// top-left samples (from previously decoded blocks), along with four
// extra top-right samples for luma (intra4x4 prediction only).
// One possible layout is, using 32 * (17 + 9) bytes:
//
//   .+------   <- only 1 pixel high
//   .|yyyyt.
//   .|yyyyt.
//   .|yyyyt.
//   .|yyyy..
//   .+--.+--   <- only 1 pixel high
//   .|uu.|vv
//   .|uu.|vv
//
// Every character is a 4x4 block, with legend:
//  '.' = unused
//  'y' = y-samples   'u' = u-samples     'v' = u-samples
//  '|' = left sample,   '-' = top sample,    '+' = top-left sample
//  't' = extra top-right sample for 4x4 modes

// upstream: src/dec/vp8i_dec.h YUV_SIZE
/// Size of the block for Y, U and V (`yuv_b`).
pub const YUV_SIZE: i32 = BPS * 17 + BPS * 9;
// upstream: src/dec/vp8i_dec.h Y_OFF
/// Position of the luma block in `yuv_b`.
pub const Y_OFF: i32 = BPS + 8;
// upstream: src/dec/vp8i_dec.h U_OFF
/// Position of the U block in `yuv_b`.
pub const U_OFF: i32 = Y_OFF + BPS * 16 + BPS;
// upstream: src/dec/vp8i_dec.h V_OFF
/// Position of the V block in `yuv_b`.
pub const V_OFF: i32 = U_OFF + 16;

// upstream: src/dec/vp8i_dec.h MIN_WIDTH_FOR_THREADS
/// Minimal width under which lossy multi-threading is always disabled.
pub const MIN_WIDTH_FOR_THREADS: i32 = 512;

/// The array lengths of the header constants.
const NUM_MB_SEGMENTS_U: usize = NUM_MB_SEGMENTS as usize;
const NUM_PROBAS_U: usize = NUM_PROBAS as usize;
const NUM_CTX_U: usize = NUM_CTX as usize;
const NUM_BANDS_U: usize = NUM_BANDS as usize;
const NUM_TYPES_U: usize = NUM_TYPES as usize;

//------------------------------------------------------------------------------
// Headers

// upstream: src/dec/vp8i_dec.h VP8FrameHeader
/// The frame header.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct VP8FrameHeader {
    pub key_frame: u8,
    pub profile: u8,
    pub show: u8,
    pub partition_length: u32,
}

// upstream: src/dec/vp8i_dec.h VP8PictureHeader
/// The picture header of a key frame.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct VP8PictureHeader {
    pub width: u16,
    pub height: u16,
    pub xscale: u8,
    pub yscale: u8,
    /// 0 = YCbCr.
    pub colorspace: u8,
    pub clamp_type: u8,
}

// upstream: src/dec/vp8i_dec.h VP8SegmentHeader
/// Segment features. The `int` flags are `bool` (ADR 0003).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct VP8SegmentHeader {
    pub use_segment: bool,
    /// Whether to update the segment map or not.
    pub update_map: bool,
    /// Absolute or delta values for quantizer and filter.
    pub absolute_delta: bool,
    /// Quantization changes.
    pub quantizer: [i8; NUM_MB_SEGMENTS_U],
    /// Filter strength for segments.
    pub filter_strength: [i8; NUM_MB_SEGMENTS_U],
}

// upstream: src/dec/vp8i_dec.h VP8ProbaArray
/// Probas associated to one of the contexts.
pub type VP8ProbaArray = [u8; NUM_PROBAS_U];

// upstream: src/dec/vp8i_dec.h VP8BandProbas
/// All the probas associated to one band.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct VP8BandProbas {
    pub probas: [VP8ProbaArray; NUM_CTX_U],
}

// upstream: src/dec/vp8i_dec.h VP8Proba
/// Struct collecting all frame-persistent probabilities.
///
/// `bands_ptr[t][b]` is the index in `bands[t]` of the band that C's pointer
/// `bands_ptr[t][b]` points at (ADR 0003, "Pointers").
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct VP8Proba {
    pub segments: [u8; MB_FEATURE_TREE_PROBS as usize],
    /// Type: 0:Intra16-AC  1:Intra16-DC   2:Chroma   3:Intra4
    pub bands: [[VP8BandProbas; NUM_BANDS_U]; NUM_TYPES_U],
    pub bands_ptr: [[usize; 16 + 1]; NUM_TYPES_U],
}

// upstream: src/dec/vp8i_dec.h VP8FilterHeader
/// Filter parameters. The `int` flags are `bool` (ADR 0003).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct VP8FilterHeader {
    /// false=complex, true=simple.
    pub simple: bool,
    /// [0..63]
    pub level: i32,
    /// [0..7]
    pub sharpness: i32,
    pub use_lf_delta: bool,
    pub ref_lf_delta: [i32; NUM_REF_LF_DELTAS as usize],
    pub mode_lf_delta: [i32; NUM_MODE_LF_DELTAS as usize],
}

//------------------------------------------------------------------------------
// Informations about the macroblocks.

// upstream: src/dec/vp8i_dec.h VP8FInfo
/// Filter specs.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct VP8FInfo {
    /// Filter limit in [3..189], or 0 if no filtering.
    pub f_limit: u8,
    /// Inner limit in [1..63].
    pub f_ilevel: u8,
    /// Do inner filtering?
    pub f_inner: u8,
    /// High edge variance threshold in [0..2].
    pub hev_thresh: u8,
}

impl VP8FInfo {
    /// `sizeof(VP8FInfo)` in C, for the size test of the frame memory.
    pub const C_SIZE: usize = 4;
}

// upstream: src/dec/vp8i_dec.h VP8MB
/// Top/Left Contexts used for syntax-parsing.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct VP8MB {
    /// Non-zero AC/DC coeffs (4bit for luma + 4bit for chroma).
    pub nz: u8,
    /// Non-zero DC coeff (1bit).
    pub nz_dc: u8,
}

impl VP8MB {
    /// `sizeof(VP8MB)` in C, for the size test of the frame memory.
    pub const C_SIZE: usize = 2;
}

// upstream: src/dec/vp8i_dec.h quant_t
/// Dequantization factors: [DC / AC].
#[expect(non_camel_case_types, reason = "the C type name")]
pub type quant_t = [i32; 2];

// upstream: src/dec/vp8i_dec.h VP8QuantMatrix
/// Dequantization matrices.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct VP8QuantMatrix {
    pub y1_mat: quant_t,
    pub y2_mat: quant_t,
    pub uv_mat: quant_t,
    /// U/V quantizer value.
    pub uv_quant: i32,
    /// Dithering amplitude (0 = off, max=255).
    pub dither: i32,
}

// upstream: src/dec/vp8i_dec.h VP8MBData
/// Data needed to reconstruct a macroblock.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VP8MBData {
    /// 384 coeffs = (16+4+4) * 4*4
    pub coeffs: [i16; 384],
    /// True if intra4x4.
    pub is_i4x4: u8,
    /// One 16x16 mode (#0) or sixteen 4x4 modes.
    pub imodes: [u8; 16],
    /// Chroma prediction mode.
    pub uvmode: u8,
    // bit-wise info about the content of each sub-4x4 blocks (in decoding order).
    // Each of the 4x4 blocks for y/u/v is associated with a 2b code according to:
    //   code=0 -> no coefficient
    //   code=1 -> only DC
    //   code=2 -> first three coefficients are non-zero
    //   code=3 -> more than three coefficients are non-zero
    // This allows to call specialized transform functions.
    pub non_zero_y: u32,
    pub non_zero_uv: u32,
    /// Local dithering strength (deduced from `non_zero*`).
    pub dither: u8,
    pub skip: u8,
    pub segment: u8,
}

impl VP8MBData {
    /// `sizeof(VP8MBData)` in C, for the size test of the frame memory.
    pub const C_SIZE: usize = 800;
}

impl Default for VP8MBData {
    fn default() -> Self {
        Self {
            coeffs: [0; 384],
            is_i4x4: 0,
            imodes: [0; 16],
            uvmode: 0,
            non_zero_y: 0,
            non_zero_uv: 0,
            dither: 0,
            skip: 0,
            segment: 0,
        }
    }
}

// upstream: src/dec/vp8i_dec.h VP8ThreadContext
/// Persistent information needed by the parallel processing.
///
/// C points `f_info` and `mb_data` into the decoder's memory and swaps them with
/// the decoder's own pointers; here they are vectors that are swapped with the
/// decoder's vectors (`frame_dec.c`, `VP8ProcessRow`). An empty vector is C's
/// `NULL`. `filter_row` is a `bool` (ADR 0003).
#[derive(Clone, Debug, Default)]
pub struct VP8ThreadContext {
    /// Cache row to process (in [0..2]).
    pub id: i32,
    /// Macroblock position of the row.
    pub mb_y: i32,
    /// True if row-filtering is needed.
    pub filter_row: bool,
    /// Filter strengths (swapped with `dec.f_info`).
    pub f_info: Vec<VP8FInfo>,
    /// Reconstruction data (swapped with `dec.mb_data`).
    pub mb_data: Vec<VP8MBData>,
    /// Copy of the `VP8Io` to pass to `put()`.
    pub io: VP8Io,
}

// upstream: src/dec/vp8i_dec.h VP8TopSamples
/// Saved top samples, per macroblock. Fits into a cache-line.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct VP8TopSamples {
    pub y: [u8; 16],
    pub u: [u8; 8],
    pub v: [u8; 8],
}

impl VP8TopSamples {
    /// `sizeof(VP8TopSamples)` in C, for the size test of the frame memory.
    pub const C_SIZE: usize = 32;
}

/// The part of the decoder that `FinishRow` and, for `mt_method == 2`,
/// `ReconstructRow` mutate, with copies of the decoder's fields that they read
/// (ADR 0003, "Threads"). With `mt_method > 0` it moves into the worker at
/// `Launch` and returns at `Sync`; otherwise the calling thread uses it in place.
/// The worker is its home in both cases ([`VP8Decoder::worker`]). This type has
/// no C counterpart: C reaches these fields through `dec`.
#[derive(Clone, Debug)]
pub struct VP8RowJob {
    /// Thread context (C: `dec->thread_ctx`).
    pub thread_ctx: VP8ThreadContext,
    /// Top y/u/v samples (C: `dec->yuv_t`).
    pub yuv_t: Vec<VP8TopSamples>,
    /// Main block for Y/U/V, `YUV_SIZE` bytes (C: `dec->yuv_b`).
    pub yuv_b: Vec<u8>,
    /// Luma cache rows with the extra rows above them; C's `cache_y` points at
    /// `cache_y_pos` (C: the luma part of the cache in `dec->mem`).
    pub cache_y: Vec<u8>,
    /// U cache rows with their extra rows; C's `cache_u` is at `cache_uv_pos`.
    pub cache_u: Vec<u8>,
    /// V cache rows with their extra rows; C's `cache_v` is at `cache_uv_pos`.
    pub cache_v: Vec<u8>,
    /// Position of C's `cache_y` pointer in `cache_y`.
    pub cache_y_pos: usize,
    /// Position of C's `cache_u` and `cache_v` pointers in their vectors.
    pub cache_uv_pos: usize,
    /// Random generator for dithering (C: `dec->dithering_rg`).
    pub dithering_rg: VP8Random,
    /// Copy of `dec->mt_method`.
    pub mt_method: i32,
    /// Copy of `dec->filter_type`.
    pub filter_type: i32,
    /// Copy of `dec->dither`.
    pub dither: bool,
    /// Copy of `dec->mb_w`.
    pub mb_w: i32,
    /// Copy of `dec->mb_h`.
    pub mb_h: i32,
    /// Copy of `dec->tl_mb_x`.
    pub tl_mb_x: i32,
    /// Copy of `dec->br_mb_x`.
    pub br_mb_x: i32,
    /// Copy of `dec->br_mb_y`.
    pub br_mb_y: i32,
    /// Copy of `dec->num_caches`.
    pub num_caches: i32,
    /// Copy of `dec->cache_y_stride`.
    pub cache_y_stride: i32,
    /// Copy of `dec->cache_uv_stride`.
    pub cache_uv_stride: i32,
}

impl Default for VP8RowJob {
    fn default() -> Self {
        Self {
            thread_ctx: VP8ThreadContext::default(),
            yuv_t: Vec::new(),
            yuv_b: Vec::new(),
            cache_y: Vec::new(),
            cache_u: Vec::new(),
            cache_v: Vec::new(),
            cache_y_pos: 0,
            cache_uv_pos: 0,
            dithering_rg: zeroed_random(),
            mt_method: 0,
            filter_type: 0,
            dither: false,
            mb_w: 0,
            mb_h: 0,
            tl_mb_x: 0,
            br_mb_x: 0,
            br_mb_y: 0,
            num_caches: 0,
            cache_y_stride: 0,
            cache_uv_stride: 0,
        }
    }
}

/// The zeroed generator of a `calloc`ed decoder.
fn zeroed_random() -> VP8Random {
    VP8Random {
        index1: 0,
        index2: 0,
        tab: [0; VP8_RANDOM_TABLE_SIZE],
        amp: 0,
    }
}

/// Where C's `dec->alpha_plane` points. This type has no C counterpart.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum VP8AlphaPlane {
    /// `NULL`.
    #[default]
    None,
    /// The alpha part of the frame memory that `AllocateMemory` carves
    /// ([`VP8Decoder::frame_alpha`]); `VP8DecompressAlphaRows` replaces it before
    /// it decodes anything.
    Frame,
    /// `alpha_plane_mem` ([`VP8Decoder::alpha_plane_mem`]).
    Memory,
}

//------------------------------------------------------------------------------
// VP8Decoder: the main opaque structure handed over to user

// upstream: src/dec/vp8i_dec.h VP8Decoder
/// The VP8 decoder.
///
/// It holds no pointer to its input, its `VP8Io`, its hooks or the alpha data
/// (ADR 0003, "Decoding context per call"): the owner passes them to each call.
/// C's pointers into `dec->mem` are the vectors below and those of
/// [`VP8RowJob`]; `mem` and `mem_size` keep only the size, for C's decision to
/// reallocate. The `int` flags are `bool` (ADR 0003). `Default` is the zeroed
/// structure of `WebPSafeCalloc`, before `VP8New` sets it up.
#[derive(Default)]
#[expect(
    clippy::struct_excessive_bools,
    reason = "the int flags of the C structure, which it uses as predicates"
)]
pub struct VP8Decoder {
    pub status: VP8StatusCode,
    /// True if ready to decode a picture with `VP8Decode()`.
    pub ready: bool,
    /// Set when status is not OK; `None` is C's `NULL`.
    pub error_msg: Option<&'static str>,

    // Main data source
    pub br: VP8BitReader,
    /// If true, incremental decoding is expected.
    pub incremental: bool,

    // headers
    pub frm_hdr: VP8FrameHeader,
    pub pic_hdr: VP8PictureHeader,
    pub filter_hdr: VP8FilterHeader,
    pub segment_hdr: VP8SegmentHeader,

    // Worker
    /// The worker, and the home of the row job (C: `worker` and the parts of
    /// the decoder that `FinishRow` uses).
    pub worker: WebPWorker<VP8RowJob, ()>,
    /// Multi-thread method: 0=off, 1=[parse+recon][filter]
    /// 2=[parse][recon+filter]
    pub mt_method: i32,
    /// Current cache row.
    pub cache_id: i32,
    /// Number of cached rows of 16 pixels (1, 2 or 3).
    pub num_caches: i32,

    // dimension, in macroblock units.
    pub mb_w: i32,
    pub mb_h: i32,

    // Macroblock to process/filter, depending on cropping and filter_type.
    /// Top-left MB that must be in-loop filtered.
    pub tl_mb_x: i32,
    pub tl_mb_y: i32,
    /// Last bottom-right MB that must be decoded.
    pub br_mb_x: i32,
    pub br_mb_y: i32,

    /// Number of partitions minus one.
    pub num_parts_minus_one: u32,
    /// Per-partition boolean decoders.
    pub parts: [VP8BitReader; MAX_NUM_PARTITIONS as usize],

    /// Whether to use dithering or not (dithering strength, deduced from the
    /// decoding options). The random generator is in [`VP8RowJob`].
    pub dither: bool,

    /// Dequantization (one set of DC/AC dequant factor per segment).
    pub dqm: [VP8QuantMatrix; NUM_MB_SEGMENTS_U],

    // probabilities
    pub proba: VP8Proba,
    pub use_skip_proba: bool,
    pub skip_p: u8,

    // Boundary data cache and persistent buffers.
    /// Top intra modes values: `4 * mb_w`.
    pub intra_t: Vec<u8>,
    /// Left intra modes values.
    pub intra_l: [u8; 4],

    /// Contextual macroblock info, `mb_w + 1` entries; C's `mb_info` points at
    /// index 1, so C's `mb_info[-1]` is index 0.
    pub mb_info: Vec<VP8MB>,
    /// Filter strength info; empty where C has `NULL`.
    pub f_info: Vec<VP8FInfo>,

    /// Row stride of the luma cache.
    pub cache_y_stride: i32,
    /// Row stride of the chroma caches.
    pub cache_uv_stride: i32,

    /// Size of the main memory chunk that C allocates for the data above
    /// (C: `mem_size`; the chunk itself is the separate vectors).
    pub mem_size: usize,

    // Per macroblock non-persistent infos.
    /// Current position, in macroblock units.
    pub mb_x: i32,
    pub mb_y: i32,
    /// Parsed reconstruction data.
    pub mb_data: Vec<VP8MBData>,

    // Filtering side-info
    /// 0=off, 1=simple, 2=complex.
    pub filter_type: i32,
    /// Precalculated per-segment/type.
    pub fstrengths: [[VP8FInfo; 2]; NUM_MB_SEGMENTS_U],

    // Alpha
    /// Alpha-plane decoder object.
    pub alph_dec: Option<Box<ALPHDecoder>>,
    /// True if the alpha data is decoded in the alpha plane.
    pub is_alpha_decoded: bool,
    /// Memory allocated for the alpha plane; empty where C has `NULL`.
    pub alpha_plane_mem: Vec<u8>,
    /// The alpha part of the frame memory (C: the last `alpha_size` bytes of
    /// `dec->mem`).
    pub frame_alpha: Vec<u8>,
    /// Output. Persistent, contains the whole data (C: `alpha_plane`).
    pub alpha_plane: VP8AlphaPlane,
    /// Last decoded alpha row, as a position in the alpha plane; `None` where C
    /// has `NULL`.
    pub alpha_prev_line: Option<usize>,
    /// Derived from decoding options (0=off, 100=full).
    pub alpha_dithering: i32,

    // Threads (no C counterpart; `frame_dec.rs` describes the split)
    /// Set when a row job has been launched whose `put` part runs at the next
    /// `Sync` on the calling thread.
    pub put_pending: bool,
    /// Set when such a `put` part failed; the later `Sync`s report it, as
    /// `had_error` does in C until `InitThreadContext` resets the worker.
    pub put_error: bool,
}
