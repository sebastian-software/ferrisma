// libwebp v1.6.0, src/enc/vp8li_enc.h:
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
// Lossless encoder: internal header.
//
// Author: Vikas Arora (vikaas.arora@gmail.com)

//! Translation of `src/enc/vp8li_enc.h` from libwebp v1.6.0.
//!
//! Work package: WP11 (VP8L encoder).

use crate::enc::backward_references_enc::{VP8LBackwardRefs, VP8LHashChain};
use crate::webp::format_constants::{MAX_PALETTE_SIZE, MIN_TRANSFORM_BITS, NUM_TRANSFORM_BITS};

// upstream: src/enc/vp8li_enc.h WEBP_NEAR_LOSSLESS
/// Either `WEBP_NEAR_LOSSLESS` is defined as 0 in config.h when compiling to
/// disable near-lossless, or it is enabled by default. The reference
/// configuration enables it, so only the near-lossless branches are translated.
pub const WEBP_NEAR_LOSSLESS: i32 = 1;

// upstream: src/enc/vp8li_enc.h MAX_TRANSFORM_BITS
/// maximum value of `transform_bits` in `VP8LEncoder`.
pub const MAX_TRANSFORM_BITS: i32 = MIN_TRANSFORM_BITS + (1 << NUM_TRANSFORM_BITS) - 1;

// upstream: src/enc/vp8li_enc.h VP8LEncoderARGBContent
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct VP8LEncoderARGBContent(pub u32);

#[expect(non_upper_case_globals, reason = "C enumerator names")]
impl VP8LEncoderARGBContent {
    // upstream: src/enc/vp8li_enc.h kEncoderNone
    pub const kEncoderNone: Self = Self(0);
    // upstream: src/enc/vp8li_enc.h kEncoderARGB
    pub const kEncoderARGB: Self = Self(1);
    // upstream: src/enc/vp8li_enc.h kEncoderNearLossless
    pub const kEncoderNearLossless: Self = Self(2);
    // upstream: src/enc/vp8li_enc.h kEncoderPalette
    pub const kEncoderPalette: Self = Self(3);
}

// upstream: src/enc/vp8li_enc.h VP8LEncoder
/// The lossless encoder state. It holds no pointer to its configuration or its
/// picture: the functions that read them take them per call (C: `config` and
/// `pic`). The C pointers `argb`, `argb_scratch` and `transform_data` into the
/// transform memory are positions in `transform_mem`, in that order.
#[derive(Clone, Debug, PartialEq, Eq)]
#[expect(
    clippy::struct_excessive_bools,
    reason = "the four C int flags use_cross_color, use_subtract_green, use_predict and use_palette"
)]
pub struct VP8LEncoder {
    /// Transformed argb image data: its position in `transform_mem`.
    pub argb: usize,
    /// Content type of the argb buffer.
    pub argb_content: VP8LEncoderARGBContent,
    /// Scratch memory for argb rows (used for prediction): its position.
    pub argb_scratch: usize,
    /// Scratch memory for transform data: its position.
    pub transform_data: usize,
    /// Currently allocated memory; empty where C has `NULL`.
    pub transform_mem: Vec<u32>,
    /// Currently allocated memory size.
    pub transform_mem_size: usize,

    /// Corresponds to packed image width.
    pub current_width: i32,

    // Encoding parameters derived from quality parameter.
    pub histo_bits: i32,
    /// <= `MAX_TRANSFORM_BITS`
    pub predictor_transform_bits: i32,
    /// <= `MAX_TRANSFORM_BITS`
    pub cross_color_transform_bits: i32,
    /// If equal to 0, don't use color cache.
    pub cache_bits: i32,

    // Encoding parameters derived from image characteristics.
    pub use_cross_color: bool,
    pub use_subtract_green: bool,
    pub use_predict: bool,
    pub use_palette: bool,
    pub palette_size: i32,
    pub palette: [u32; MAX_PALETTE_SIZE as usize],
    /// Sorted version of palette for cache purposes.
    pub palette_sorted: [u32; MAX_PALETTE_SIZE as usize],

    // Some 'scratch' (potentially large) objects.
    /// Backward Refs array for temporaries.
    pub refs: [VP8LBackwardRefs; 4],
    /// `HashChain` data for constructing backward references.
    pub hash_chain: VP8LHashChain,
}

impl Default for VP8LEncoder {
    fn default() -> Self {
        Self {
            argb: 0,
            argb_content: VP8LEncoderARGBContent::kEncoderNone,
            argb_scratch: 0,
            transform_data: 0,
            transform_mem: Vec::new(),
            transform_mem_size: 0,
            current_width: 0,
            histo_bits: 0,
            predictor_transform_bits: 0,
            cross_color_transform_bits: 0,
            cache_bits: 0,
            use_cross_color: false,
            use_subtract_green: false,
            use_predict: false,
            use_palette: false,
            palette_size: 0,
            palette: [0; MAX_PALETTE_SIZE as usize],
            palette_sorted: [0; MAX_PALETTE_SIZE as usize],
            refs: Default::default(),
            hash_chain: VP8LHashChain::default(),
        }
    }
}

impl VP8LEncoder {
    /// The three regions of the transform memory, as the C pointers `argb`,
    /// `argb_scratch` and `transform_data` address them: each runs to the start
    /// of the next, and the last to the end of the memory.
    pub fn transform_regions(&mut self) -> (&mut [u32], &mut [u32], &mut [u32]) {
        let (head, transform_data) = self.transform_mem.split_at_mut(self.transform_data);
        let (argb, argb_scratch) = head[self.argb..].split_at_mut(self.argb_scratch - self.argb);
        (argb, argb_scratch, transform_data)
    }
}
