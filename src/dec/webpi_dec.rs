// libwebp v1.6.0, src/dec/webpi_dec.h:
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
// Internal header: WebP decoding parameters and custom IO on buffer
//
// Author: somnath@google.com (Somnath Banerjee)

//! Translation of `src/dec/webpi_dec.h` from libwebp v1.6.0.
//!
//! Work package: WP12 (decoder integration).
//!
//! Exception: WP7 translates `WebPDecParams`, `OutputFunc`, `OutputAlphaFunc` and `OutputRowFunc` from `src/dec/webpi_dec.h`, because `VP8LDecodeImage` reads `WebPDecParams` through `io->opaque`.
//!
//! # Deviations from upstream
//!
//! Changes of representation that the ADRs allow, or behavior that the Rust
//! side cannot reproduce exactly. Each records the reason.
//!
//! 1. **`WebPHeaderStructure`** holds the input as a slice and `alpha_data` as
//!    a position in it; after a successful parse, `have_all_data` is false, as
//!    C's `*headers = hdrs` leaves it.

// The items below are WP7's exception in this module (`tools/units.json`):
// `WebPDecParams` and its three output function types. ADR 0003 ("Decoding
// context per call") fixes their form.

use crate::dec::vp8_dec::{VP8Io, VP8IoRows};
use crate::utils::rescaler_utils::WebPRescaler;
use crate::webp::decode::{WebPDecBuffer, WebPDecoderOptions};

// upstream: src/dec/webpi_dec.h OutputFunc
/// Outputs RGB or YUV samples from the rows of one `put()` call into `output`.
/// Returns the number of rows written.
pub type OutputFunc =
    fn(&VP8Io, &mut VP8IoRows<'_>, &mut WebPDecParams<'_>, &mut WebPDecBuffer<'_>) -> i32;

// upstream: src/dec/webpi_dec.h OutputAlphaFunc
/// Outputs the alpha channel of one `put()` call into `output`; the last
/// argument is `expected_num_out_lines`.
pub type OutputAlphaFunc =
    fn(&VP8Io, &mut VP8IoRows<'_>, &mut WebPDecParams<'_>, &mut WebPDecBuffer<'_>, i32) -> i32;

// upstream: src/dec/webpi_dec.h OutputRowFunc
/// Outputs one line of rescaled alpha values into `output`; the integer
/// arguments are `y_pos` and `max_out_lines`.
pub type OutputRowFunc = fn(&mut WebPDecParams<'_>, &mut WebPDecBuffer<'_>, i32, i32) -> i32;

// upstream: src/dec/webpi_dec.h WebPDecParams
/// Decoding output parameters. Transient internal object.
///
/// The C field `output` is not stored: the functions that write samples take the
/// output buffer next to the parameters (ADR 0003, "Decoding context per call").
/// `tmp_y`, `tmp_u` and `tmp_v` are positions in `memory`, where C has pointers
/// into it. Each rescaler owns its work rows, so `memory` holds only what C
/// carves from it besides them. `Default` is the zeroed structure of
/// `WebPResetDecParams`.
#[derive(Default)]
pub struct WebPDecParams<'a> {
    /// Cache for the fancy upsampler or used for temporary rescaling: luma,
    /// as a position in `memory`.
    pub tmp_y: usize,
    /// Chroma U cache, as a position in `memory`.
    pub tmp_u: usize,
    /// Chroma V cache, as a position in `memory`.
    pub tmp_v: usize,
    /// Coordinate of the line that was last output.
    pub last_y: i32,
    /// If not `None`, use alternative decoding features.
    pub options: Option<&'a WebPDecoderOptions>,
    /// Luma rescaler.
    pub scaler_y: Option<WebPRescaler>,
    /// Chroma U rescaler.
    pub scaler_u: Option<WebPRescaler>,
    /// Chroma V rescaler.
    pub scaler_v: Option<WebPRescaler>,
    /// Alpha rescaler.
    pub scaler_a: Option<WebPRescaler>,
    /// Overall scratch memory for the output work; empty where C has `NULL`.
    pub memory: Vec<u8>,
    /// Outputs RGB or YUV samples.
    pub emit: Option<OutputFunc>,
    /// Outputs the alpha channel.
    pub emit_alpha: Option<OutputAlphaFunc>,
    /// Outputs one line of rescaled alpha values.
    pub emit_alpha_row: Option<OutputRowFunc>,
}

// The items below translate the rest of `src/dec/webpi_dec.h` (WP12). The
// header declares functions that `src/dec/webp_dec.c` and `src/dec/buffer_dec.c`
// define; they are in those modules.

//------------------------------------------------------------------------------
// Header parsing helpers

// upstream: src/dec/webpi_dec.h WebPHeaderStructure
/// Structure storing a description of the RIFF headers.
///
/// `data` is C's `data` and `data_size`. `alpha_data` is the position in `data`
/// of the ALPH chunk's payload, where C has a pointer into it (`None` for
/// `NULL`). The `int` flags are `bool` (ADR 0003).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct WebPHeaderStructure<'d> {
    /// Input buffer (C: `data` and `data_size`).
    pub data: &'d [u8],
    /// True if all data is known to be available.
    pub have_all_data: bool,
    /// Offset to main data chunk (VP8 or VP8L).
    pub offset: usize,
    /// Points to alpha chunk (if present), as a position in `data`.
    pub alpha_data: Option<usize>,
    /// Alpha chunk size.
    pub alpha_data_size: usize,
    /// VP8/VP8L compressed data size.
    pub compressed_size: usize,
    /// Size of the riff payload (or 0 if absent).
    pub riff_size: usize,
    /// True if a VP8L chunk is present.
    pub is_lossless: bool,
}
