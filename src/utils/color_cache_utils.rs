// libwebp v1.6.0, src/utils/color_cache_utils.c:
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
// Color Cache for WebP Lossless
//
// Author: Jyrki Alakuijala (jyrki@google.com)

// libwebp v1.6.0, src/utils/color_cache_utils.h:
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
// Color Cache for WebP Lossless
//
// Authors: Jyrki Alakuijala (jyrki@google.com)
//          Urvang Joshi (urvang@google.com)

//! Translation of `src/utils/color_cache_utils.c` and `src/utils/color_cache_utils.h` from libwebp v1.6.0.
//!
//! Work package: WP3 (lossless shared).

use crate::utils::utils::webp_safe_calloc;

// upstream: src/utils/color_cache_utils.h VP8LColorCache
/// Main color cache struct.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct VP8LColorCache {
    /// Color entries; empty where C has `colors == NULL`.
    pub colors: Vec<u32>,
    /// Hash shift: 32 - `hash_bits`.
    pub hash_shift: i32,
    pub hash_bits: i32,
}

// upstream: src/utils/color_cache_utils.h kHashMul
pub const HASH_MUL: u32 = 0x1e35_a7bd;

// upstream: src/utils/color_cache_utils.h VP8LHashPix
/// The hash of `argb`: the 32-bit product with [`HASH_MUL`], which wraps as
/// in C, shifted right by `shift`.
#[must_use]
#[inline]
pub fn vp8l_hash_pix(argb: u32, shift: i32) -> i32 {
    (argb.wrapping_mul(HASH_MUL) >> shift).cast_signed()
}

/// Converts a cache key, which is never negative, to an index.
#[inline]
fn key_index(key: i32) -> usize {
    usize::try_from(key).expect("a cache key is not negative")
}

// upstream: src/utils/color_cache_utils.h VP8LColorCacheLookup
#[must_use]
#[inline]
pub fn vp8l_color_cache_lookup(cc: &VP8LColorCache, key: u32) -> u32 {
    debug_assert_eq!(key >> cc.hash_bits, 0);
    cc.colors[key as usize]
}

// upstream: src/utils/color_cache_utils.h VP8LColorCacheSet
#[inline]
pub fn vp8l_color_cache_set(cc: &mut VP8LColorCache, key: u32, argb: u32) {
    debug_assert_eq!(key >> cc.hash_bits, 0);
    cc.colors[key as usize] = argb;
}

// upstream: src/utils/color_cache_utils.h VP8LColorCacheInsert
#[inline]
pub fn vp8l_color_cache_insert(cc: &mut VP8LColorCache, argb: u32) {
    let key = vp8l_hash_pix(argb, cc.hash_shift);
    cc.colors[key_index(key)] = argb;
}

// upstream: src/utils/color_cache_utils.h VP8LColorCacheGetIndex
#[must_use]
#[inline]
pub fn vp8l_color_cache_get_index(cc: &VP8LColorCache, argb: u32) -> i32 {
    vp8l_hash_pix(argb, cc.hash_shift)
}

// upstream: src/utils/color_cache_utils.h VP8LColorCacheContains
/// Return the key if cc contains argb, and -1 otherwise.
#[must_use]
#[inline]
pub fn vp8l_color_cache_contains(cc: &VP8LColorCache, argb: u32) -> i32 {
    let key = vp8l_hash_pix(argb, cc.hash_shift);
    if cc.colors[key_index(key)] == argb {
        key
    } else {
        -1
    }
}

// upstream: src/utils/color_cache_utils.c VP8LColorCacheInit
/// Initializes the color cache with `hash_bits` bits for the keys.
/// Returns false in case of memory error; `colors` is then empty, and the
/// other fields keep their values, as in C.
#[must_use]
pub fn vp8l_color_cache_init(color_cache: &mut VP8LColorCache, hash_bits: i32) -> bool {
    let hash_size: i32 = 1 << hash_bits;
    debug_assert!(hash_bits > 0);
    let Some(colors) = webp_safe_calloc::<u32>(i64::from(hash_size).cast_unsigned(), 4) else {
        color_cache.colors = Vec::new();
        return false;
    };
    color_cache.colors = colors;
    color_cache.hash_shift = 32 - hash_bits;
    color_cache.hash_bits = hash_bits;
    true
}

// upstream: src/utils/color_cache_utils.c VP8LColorCacheClear
/// Delete the memory associated to color cache.
pub fn vp8l_color_cache_clear(color_cache: &mut VP8LColorCache) {
    color_cache.colors = Vec::new();
}

// upstream: src/utils/color_cache_utils.c VP8LColorCacheCopy
/// Copies the `1 << dst.hash_bits` colors of `src` into `dst`.
///
/// # Panics
///
/// Panics when either cache holds fewer colors, where C copies outside them.
pub fn vp8l_color_cache_copy(src: &VP8LColorCache, dst: &mut VP8LColorCache) {
    debug_assert_eq!(src.hash_bits, dst.hash_bits);
    let count = 1_usize << dst.hash_bits;
    dst.colors[..count].copy_from_slice(&src.colors[..count]);
}
