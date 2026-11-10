// libwebp v1.6.0, src/dec/common_dec.h:
//
// Copyright 2015 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// Definitions and macros common to encoding and decoding
//
// Author: Skal (pascal.massimino@gmail.com)

//! Translation of `src/dec/common_dec.h` from libwebp v1.6.0.
//!
//! Work package: WP6 (VP8 decoder core).
//!
//! The two anonymous `enum`s of the header are `int` constants in C, so they are
//! `i32` constants here (ADR 0003 gives a newtype only to a named enum type).
//! `IsValidColorspace` is declared here but defined in `src/dec/buffer_dec.c`,
//! which WP12 translates.

// upstream: src/dec/common_dec.h B_DC_PRED
/// 4x4 mode: DC prediction.
pub const B_DC_PRED: i32 = 0;
// upstream: src/dec/common_dec.h B_TM_PRED
/// 4x4 mode: `TrueMotion` prediction.
pub const B_TM_PRED: i32 = 1;
// upstream: src/dec/common_dec.h B_VE_PRED
/// 4x4 mode: vertical prediction.
pub const B_VE_PRED: i32 = 2;
// upstream: src/dec/common_dec.h B_HE_PRED
/// 4x4 mode: horizontal prediction.
pub const B_HE_PRED: i32 = 3;
// upstream: src/dec/common_dec.h B_RD_PRED
/// 4x4 mode: down-right prediction.
pub const B_RD_PRED: i32 = 4;
// upstream: src/dec/common_dec.h B_VR_PRED
/// 4x4 mode: vertical-right prediction.
pub const B_VR_PRED: i32 = 5;
// upstream: src/dec/common_dec.h B_LD_PRED
/// 4x4 mode: down-left prediction.
pub const B_LD_PRED: i32 = 6;
// upstream: src/dec/common_dec.h B_VL_PRED
/// 4x4 mode: vertical-left prediction.
pub const B_VL_PRED: i32 = 7;
// upstream: src/dec/common_dec.h B_HD_PRED
/// 4x4 mode: horizontal-down prediction.
pub const B_HD_PRED: i32 = 8;
// upstream: src/dec/common_dec.h B_HU_PRED
/// 4x4 mode: horizontal-up prediction.
pub const B_HU_PRED: i32 = 9;
// upstream: src/dec/common_dec.h NUM_BMODES
/// Number of 4x4 modes (10).
pub const NUM_BMODES: i32 = B_HU_PRED + 1 - B_DC_PRED;

// upstream: src/dec/common_dec.h DC_PRED
/// Luma16 or UV mode: DC prediction.
pub const DC_PRED: i32 = B_DC_PRED;
// upstream: src/dec/common_dec.h V_PRED
/// Luma16 or UV mode: vertical prediction.
pub const V_PRED: i32 = B_VE_PRED;
// upstream: src/dec/common_dec.h H_PRED
/// Luma16 or UV mode: horizontal prediction.
pub const H_PRED: i32 = B_HE_PRED;
// upstream: src/dec/common_dec.h TM_PRED
/// Luma16 or UV mode: `TrueMotion` prediction.
pub const TM_PRED: i32 = B_TM_PRED;
// upstream: src/dec/common_dec.h B_PRED
/// Refined I4x4 mode.
pub const B_PRED: i32 = NUM_BMODES;
// upstream: src/dec/common_dec.h NUM_PRED_MODES
/// Number of Luma16 or UV modes.
pub const NUM_PRED_MODES: i32 = 4;

// upstream: src/dec/common_dec.h B_DC_PRED_NOTOP
/// Special mode: DC prediction without the top samples.
pub const B_DC_PRED_NOTOP: i32 = 4;
// upstream: src/dec/common_dec.h B_DC_PRED_NOLEFT
/// Special mode: DC prediction without the left samples.
pub const B_DC_PRED_NOLEFT: i32 = 5;
// upstream: src/dec/common_dec.h B_DC_PRED_NOTOPLEFT
/// Special mode: DC prediction without the top and left samples.
pub const B_DC_PRED_NOTOPLEFT: i32 = 6;
// upstream: src/dec/common_dec.h NUM_B_DC_MODES
/// Number of 16x16 and chroma predictors, the special modes included.
pub const NUM_B_DC_MODES: i32 = 7;

// upstream: src/dec/common_dec.h MB_FEATURE_TREE_PROBS
/// Number of probabilities of the segment tree.
pub const MB_FEATURE_TREE_PROBS: i32 = 3;
// upstream: src/dec/common_dec.h NUM_MB_SEGMENTS
/// Number of segments.
pub const NUM_MB_SEGMENTS: i32 = 4;
// upstream: src/dec/common_dec.h NUM_REF_LF_DELTAS
/// Number of reference-frame loop-filter deltas.
pub const NUM_REF_LF_DELTAS: i32 = 4;
// upstream: src/dec/common_dec.h NUM_MODE_LF_DELTAS
/// Number of mode loop-filter deltas (I4x4, ZERO, *, SPLIT).
pub const NUM_MODE_LF_DELTAS: i32 = 4;
// upstream: src/dec/common_dec.h MAX_NUM_PARTITIONS
/// Maximum number of token partitions.
pub const MAX_NUM_PARTITIONS: i32 = 8;
// upstream: src/dec/common_dec.h NUM_TYPES
/// Coefficient types: 0: i16-AC, 1: i16-DC, 2: chroma-AC, 3: i4-AC.
pub const NUM_TYPES: i32 = 4;
// upstream: src/dec/common_dec.h NUM_BANDS
/// Number of coefficient bands.
pub const NUM_BANDS: i32 = 8;
// upstream: src/dec/common_dec.h NUM_CTX
/// Number of coefficient contexts.
pub const NUM_CTX: i32 = 3;
// upstream: src/dec/common_dec.h NUM_PROBAS
/// Number of probabilities per context.
pub const NUM_PROBAS: i32 = 11;
