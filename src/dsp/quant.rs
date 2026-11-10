// libwebp v1.6.0, src/dsp/quant.h:
//
// Copyright 2018 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------

//! Translation of `src/dsp/quant.h` from libwebp v1.6.0.
//!
//! Work package: WP10 (VP8 encoder core).

use crate::dsp::dsp::BPS;

// upstream: src/dsp/quant.h IsFlat
/// Whether the AC levels of `num_blocks` blocks of 16 have at most `thresh`
/// non-zero entries (C: `IsFlat_C`, the portable form of `IsFlat`).
#[must_use]
pub fn is_flat_c(levels: &[i16], num_blocks: i32, thresh: i32) -> bool {
    let mut score = 0;
    let mut num_blocks = num_blocks;
    let mut levels = levels;
    while num_blocks > 0 {
        num_blocks -= 1;
        // TODO(skal): refine positional scoring?
        for &level in &levels[1..16] {
            // omit DC, we're only interested in AC
            score += i32::from(level != 0);
            if score > thresh {
                return false;
            }
        }
        levels = &levels[16..];
    }
    true
}

// upstream: src/dsp/quant.h IsFlat
/// Whether the AC levels of `num_blocks` blocks of 16 have at most `thresh`
/// non-zero entries. C selects `IsFlat_NEON` at compile time with
/// `WEBP_USE_NEON` and `IsFlat_C` otherwise; the NEON form is in
/// `src/dsp/enc_neon.rs` (its deviation 1).
#[must_use]
pub fn is_flat(levels: &[i16], num_blocks: i32, thresh: i32) -> bool {
    #[cfg(all(feature = "simd", target_arch = "aarch64"))]
    {
        return crate::dsp::enc_neon::is_flat(levels, num_blocks, thresh);
    }
    #[allow(
        unreachable_code,
        reason = "the C path is reached only without a SIMD variant"
    )]
    is_flat_c(levels, num_blocks, thresh)
}

// upstream: src/dsp/quant.h IsFlatSource16
/// Whether the 16x16 block at `src` (stride `BPS`) has one value.
#[must_use]
pub fn is_flat_source16(src: &[u8]) -> bool {
    let v = src[0];
    for i in 0..16 {
        let row = &src[i * BPS as usize..i * BPS as usize + 16];
        // C: memcmp() of each 4-byte group with src[0] * 0x01010101u
        if row.iter().any(|&sample| sample != v) {
            return false;
        }
    }
    true
}
