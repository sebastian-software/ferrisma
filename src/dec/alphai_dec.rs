// libwebp v1.6.0, src/dec/alphai_dec.h:
//
// Copyright 2013 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// Alpha decoder: internal header.
//
// Author: Urvang (urvang@google.com)

//! Translation of `src/dec/alphai_dec.h` from libwebp v1.6.0.
//!
//! Work package: WP7 (VP8L decoder).

// The header also declares `WebPDeallocateAlphaMemory`, which
// `src/dec/alpha_dec.c` defines; it belongs to that module (WP6).

use crate::dec::vp8_dec::VP8Io;
use crate::dec::vp8li_dec::VP8LDecoder;
use crate::dsp::dsp::WEBP_FILTER_TYPE;

// upstream: src/dec/alphai_dec.h ALPHDecoder
/// The alpha decoder. `Default` is the zeroed structure of `ALPHNew`.
///
/// The C field `output` is not stored: the alpha plane belongs to the VP8
/// decoder, which passes it to each call, and `prev_line` is a position in it
/// (ADR 0003, "Decoding context per call"). `io` keeps no `opaque` that points
/// back here; the VP8L alpha functions take this decoder instead.
#[derive(Default)]
pub struct ALPHDecoder {
    /// Width of the alpha plane.
    pub width: i32,
    /// Height of the alpha plane.
    pub height: i32,
    /// Compression method (`ALPHA_NO_COMPRESSION` or `ALPHA_LOSSLESS_COMPRESSION`).
    pub method: i32,
    /// Prediction filter of the alpha plane.
    pub filter: WEBP_FILTER_TYPE,
    /// Pre-processing level.
    pub pre_processing: i32,
    /// The lossless decoder of the alpha data; `None` where C has `NULL`.
    pub vp8l_dec: Option<Box<VP8LDecoder>>,
    /// Dimensions and crop window of the decoding.
    pub io: VP8Io,
    /// Although the alpha channel requires only one byte per pixel, the VP8L
    /// decoder sometimes needs four bytes per pixel internally during decoding.
    pub use_8b_decode: bool,
    /// Last output row, as a position in the alpha plane; `None` where C has
    /// `NULL`.
    pub prev_line: Option<usize>,
}
