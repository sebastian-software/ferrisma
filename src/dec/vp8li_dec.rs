// libwebp v1.6.0, src/dec/vp8li_dec.h:
//
// Copyright 2012 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// Lossless decoder: internal header.
//
// Author: Skal (pascal.massimino@gmail.com)
//         Vikas Arora(vikaas.arora@gmail.com)

//! Translation of `src/dec/vp8li_dec.h` from libwebp v1.6.0.
//!
//! Work package: WP7 (VP8L decoder).
//!
//! Exception: WP3 translates `VP8LTransform` from `src/dec/vp8li_dec.h`, because the inverse transforms in `src/dsp/lossless.c` take it.

use crate::webp::format_constants::VP8LImageTransformType;

// upstream: src/dec/vp8li_dec.h VP8LTransform
/// One VP8L image transform. Exception item of WP3 in WP7's module
/// (`tools/units.json`).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct VP8LTransform {
    /// Transform type.
    pub r#type: VP8LImageTransformType,
    /// Subsampling bits defining transform window.
    pub bits: i32,
    /// Transform window X index.
    pub xsize: i32,
    /// Transform window Y index.
    pub ysize: i32,
    /// Transform data, which the transform owns; empty where C has
    /// `data == NULL`.
    pub data: Vec<u32>,
}

// The items below are WP7's.

use crate::utils::bit_reader_utils::VP8LBitReader;
use crate::utils::color_cache_utils::VP8LColorCache;
use crate::utils::huffman_utils::{HTreeGroup, HuffmanTables};
use crate::utils::rescaler_utils::WebPRescaler;
use crate::webp::decode::VP8StatusCode;
use crate::webp::format_constants::NUM_TRANSFORMS;

// The functions that this header declares are defined in `src/dec/vp8l_dec.c`
// and translated in `crate::dec::vp8l_dec`.

// upstream: src/dec/vp8li_dec.h VP8LDecodeState
/// The parsing state of the lossless decoder (C: `VP8LDecodeState`).
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VP8LDecodeState(pub u32);

impl VP8LDecodeState {
    // upstream: src/dec/vp8li_dec.h READ_DATA
    pub const READ_DATA: Self = Self(0);
    // upstream: src/dec/vp8li_dec.h READ_HDR
    pub const READ_HDR: Self = Self(1);
    // upstream: src/dec/vp8li_dec.h READ_DIM
    pub const READ_DIM: Self = Self(2);
}

// upstream: src/dec/vp8li_dec.h VP8LMetadata
/// The entropy-coding state of one image level. `Default` is the zeroed
/// structure of `InitMetadata`; empty vectors stand for C's `NULL` pointers.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct VP8LMetadata {
    /// Number of color cache entries, or 0 without a color cache.
    pub color_cache_size: i32,
    /// The color cache.
    pub color_cache: VP8LColorCache,
    /// The color cache at the last check-point, for incremental decoding.
    pub saved_color_cache: VP8LColorCache,
    /// Tile mask of the Huffman image, `~0` without one.
    pub huffman_mask: i32,
    /// Subsampling bits of the Huffman image.
    pub huffman_subsample_bits: i32,
    /// Width of the Huffman image.
    pub huffman_xsize: i32,
    /// The Huffman image: the tree group of each tile.
    pub huffman_image: Vec<u32>,
    /// Number of Huffman tree groups.
    pub num_htree_groups: i32,
    /// The Huffman tree groups, whose trees point into `huffman_tables`.
    pub htree_groups: Vec<HTreeGroup>,
    /// The tables of all Huffman trees.
    pub huffman_tables: HuffmanTables,
}

// upstream: src/dec/vp8li_dec.h VP8LDecoder
/// The lossless decoder. `Default` is the zeroed structure that `VP8LNew`
/// allocates, before `VP8LNew` sets `status` and `state`.
///
/// The C fields `io` and `output` are not stored: the functions take the
/// [`VP8Io`](crate::dec::vp8_dec::VP8Io), the output buffer and the input per
/// call (ADR 0003, "Decoding context per call"). C's `pixels` holds either `uint32_t` pixels or, for the 8-bit alpha
/// path, bytes; the two forms are the fields `pixels` and `pixels_8b`. The 32-bit
/// buffer holds the pixels, the top-prediction row and `argb_cache` in one
/// allocation, as in C, and `argb_cache` is a position in it. C's
/// `rescaler_memory` carves the rescaler, its work rows and the scaled row from
/// one block; the rescaler owns its work rows, and `rescaled_row` holds the
/// scaled row.
#[derive(Default)]
pub struct VP8LDecoder {
    /// The status of the decoder; the first error is kept.
    pub status: VP8StatusCode,
    /// The parsing state.
    pub state: VP8LDecodeState,
    /// Internal data for BGRA: the pixels, a top-prediction row and
    /// `argb_cache`; empty where C has `NULL`.
    pub pixels: Vec<u32>,
    /// Internal data for the 8-bit alpha path (C: `pixels` as `uint8_t*`).
    pub pixels_8b: Vec<u8>,
    /// Scratch area for temporary BGRA storage, as a position in `pixels`;
    /// `None` where C has `NULL`.
    pub argb_cache: Option<usize>,
    /// The bit reader over the input that the owner passes to each call.
    pub br: VP8LBitReader,
    /// If true, incremental decoding is expected.
    pub incremental: bool,
    /// The bit reader at the last check-point.
    pub saved_br: VP8LBitReader,
    /// `last_pixel` at the last check-point.
    pub saved_last_pixel: i32,
    /// Width of the current image level.
    pub width: i32,
    /// Height of the current image level.
    pub height: i32,
    /// Last input row decoded so far.
    pub last_row: i32,
    /// Last pixel decoded so far. However, it may not be transformed, scaled
    /// and color-converted yet.
    pub last_pixel: i32,
    /// Last row output so far.
    pub last_out_row: i32,
    /// The entropy-coding state.
    pub hdr: VP8LMetadata,
    /// Number of transforms read.
    pub next_transform: i32,
    /// The transforms, in reading order.
    pub transforms: [VP8LTransform; NUM_TRANSFORMS as usize],
    /// Or'd bitset storing the transforms types.
    pub transforms_seen: u32,
    /// Common rescaler for all channels.
    pub rescaler: Option<WebPRescaler>,
    /// Temporary storage for the scaled BGRA row that the rescaler writes (C:
    /// the `scaled_data` part of `rescaler_memory`), in bytes.
    pub rescaled_row: Vec<u8>,
}
