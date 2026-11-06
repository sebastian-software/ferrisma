// libwebp v1.6.0, src/webp/format_constants.h:
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
//  Internal header for constants related to WebP file format.
//
// Author: Urvang (urvang@google.com)

//! Translation of `src/webp/format_constants.h` from libwebp v1.6.0.
//!
//! Work package: WP1 (types, memory, workers).

// upstream: src/webp/format_constants.h MKFOURCC
/// Packs four tag characters into a chunk identifier, first character lowest.
#[must_use]
pub fn mkfourcc(a: u8, b: u8, c: u8, d: u8) -> u32 {
    u32::from(a) | (u32::from(b) << 8) | (u32::from(c) << 16) | (u32::from(d) << 24)
}

// upstream: src/webp/format_constants.h VP8_SIGNATURE
pub const VP8_SIGNATURE: i32 = 0x009d_012a;
// upstream: src/webp/format_constants.h VP8_MAX_PARTITION0_SIZE
pub const VP8_MAX_PARTITION0_SIZE: i32 = 1 << 19;
// upstream: src/webp/format_constants.h VP8_MAX_PARTITION_SIZE
pub const VP8_MAX_PARTITION_SIZE: i32 = 1 << 24;
// upstream: src/webp/format_constants.h VP8_FRAME_HEADER_SIZE
pub const VP8_FRAME_HEADER_SIZE: i32 = 10;

// upstream: src/webp/format_constants.h VP8L_SIGNATURE_SIZE
pub const VP8L_SIGNATURE_SIZE: i32 = 1;
// upstream: src/webp/format_constants.h VP8L_MAGIC_BYTE
pub const VP8L_MAGIC_BYTE: i32 = 0x2f;
// upstream: src/webp/format_constants.h VP8L_IMAGE_SIZE_BITS
pub const VP8L_IMAGE_SIZE_BITS: i32 = 14;
// upstream: src/webp/format_constants.h VP8L_VERSION_BITS
pub const VP8L_VERSION_BITS: i32 = 3;
// upstream: src/webp/format_constants.h VP8L_VERSION
pub const VP8L_VERSION: i32 = 0;
// upstream: src/webp/format_constants.h VP8L_FRAME_HEADER_SIZE
pub const VP8L_FRAME_HEADER_SIZE: i32 = 5;

// upstream: src/webp/format_constants.h MAX_PALETTE_SIZE
pub const MAX_PALETTE_SIZE: i32 = 256;
// upstream: src/webp/format_constants.h MAX_CACHE_BITS
pub const MAX_CACHE_BITS: i32 = 11;
// upstream: src/webp/format_constants.h HUFFMAN_CODES_PER_META_CODE
pub const HUFFMAN_CODES_PER_META_CODE: i32 = 5;
// upstream: src/webp/format_constants.h ARGB_BLACK
pub const ARGB_BLACK: u32 = 0xff00_0000;

// upstream: src/webp/format_constants.h DEFAULT_CODE_LENGTH
pub const DEFAULT_CODE_LENGTH: i32 = 8;
// upstream: src/webp/format_constants.h MAX_ALLOWED_CODE_LENGTH
pub const MAX_ALLOWED_CODE_LENGTH: i32 = 15;

// upstream: src/webp/format_constants.h NUM_LITERAL_CODES
pub const NUM_LITERAL_CODES: i32 = 256;
// upstream: src/webp/format_constants.h NUM_LENGTH_CODES
pub const NUM_LENGTH_CODES: i32 = 24;
// upstream: src/webp/format_constants.h NUM_DISTANCE_CODES
pub const NUM_DISTANCE_CODES: i32 = 40;
// upstream: src/webp/format_constants.h CODE_LENGTH_CODES
pub const CODE_LENGTH_CODES: i32 = 19;

// upstream: src/webp/format_constants.h MIN_HUFFMAN_BITS
pub const MIN_HUFFMAN_BITS: i32 = 2;
// upstream: src/webp/format_constants.h NUM_HUFFMAN_BITS
pub const NUM_HUFFMAN_BITS: i32 = 3;

// upstream: src/webp/format_constants.h MIN_TRANSFORM_BITS
pub const MIN_TRANSFORM_BITS: i32 = 2;
// upstream: src/webp/format_constants.h NUM_TRANSFORM_BITS
pub const NUM_TRANSFORM_BITS: i32 = 3;

// upstream: src/webp/format_constants.h TRANSFORM_PRESENT
pub const TRANSFORM_PRESENT: i32 = 1;
// upstream: src/webp/format_constants.h NUM_TRANSFORMS
pub const NUM_TRANSFORMS: i32 = 4;

// upstream: src/webp/format_constants.h VP8LImageTransformType
/// Kinds of VP8L image transforms (C: `VP8LImageTransformType`).
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VP8LImageTransformType(pub u32);

impl VP8LImageTransformType {
    // upstream: src/webp/format_constants.h PREDICTOR_TRANSFORM
    pub const PREDICTOR_TRANSFORM: Self = Self(0);
    // upstream: src/webp/format_constants.h CROSS_COLOR_TRANSFORM
    pub const CROSS_COLOR_TRANSFORM: Self = Self(1);
    // upstream: src/webp/format_constants.h SUBTRACT_GREEN_TRANSFORM
    pub const SUBTRACT_GREEN_TRANSFORM: Self = Self(2);
    // upstream: src/webp/format_constants.h COLOR_INDEXING_TRANSFORM
    pub const COLOR_INDEXING_TRANSFORM: Self = Self(3);
}

// upstream: src/webp/format_constants.h ALPHA_HEADER_LEN
pub const ALPHA_HEADER_LEN: i32 = 1;
// upstream: src/webp/format_constants.h ALPHA_NO_COMPRESSION
pub const ALPHA_NO_COMPRESSION: i32 = 0;
// upstream: src/webp/format_constants.h ALPHA_LOSSLESS_COMPRESSION
pub const ALPHA_LOSSLESS_COMPRESSION: i32 = 1;
// upstream: src/webp/format_constants.h ALPHA_PREPROCESSED_LEVELS
pub const ALPHA_PREPROCESSED_LEVELS: i32 = 1;

// upstream: src/webp/format_constants.h TAG_SIZE
pub const TAG_SIZE: i32 = 4;
// upstream: src/webp/format_constants.h CHUNK_SIZE_BYTES
pub const CHUNK_SIZE_BYTES: i32 = 4;
// upstream: src/webp/format_constants.h CHUNK_HEADER_SIZE
pub const CHUNK_HEADER_SIZE: i32 = 8;
// upstream: src/webp/format_constants.h RIFF_HEADER_SIZE
pub const RIFF_HEADER_SIZE: i32 = 12;
// upstream: src/webp/format_constants.h ANMF_CHUNK_SIZE
pub const ANMF_CHUNK_SIZE: i32 = 16;
// upstream: src/webp/format_constants.h ANIM_CHUNK_SIZE
pub const ANIM_CHUNK_SIZE: i32 = 6;
// upstream: src/webp/format_constants.h VP8X_CHUNK_SIZE
pub const VP8X_CHUNK_SIZE: i32 = 10;

// upstream: src/webp/format_constants.h MAX_CANVAS_SIZE
pub const MAX_CANVAS_SIZE: i32 = 1 << 24;
// upstream: src/webp/format_constants.h MAX_IMAGE_AREA
pub const MAX_IMAGE_AREA: u64 = 1 << 32;
// upstream: src/webp/format_constants.h MAX_LOOP_COUNT
pub const MAX_LOOP_COUNT: i32 = 1 << 16;
// upstream: src/webp/format_constants.h MAX_DURATION
pub const MAX_DURATION: i32 = 1 << 24;
// upstream: src/webp/format_constants.h MAX_POSITION_OFFSET
pub const MAX_POSITION_OFFSET: i32 = 1 << 24;

// upstream: src/webp/format_constants.h MAX_CHUNK_PAYLOAD
/// Largest chunk payload whose header and padding still fit a `u32`: `~0U - CHUNK_HEADER_SIZE - 1`.
pub const MAX_CHUNK_PAYLOAD: u32 = u32::MAX - CHUNK_HEADER_SIZE.cast_unsigned() - 1;
