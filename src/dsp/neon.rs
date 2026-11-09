// libwebp v1.6.0, src/dsp/neon.h:
//
// Copyright 2014 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
//  NEON common code.

//! Translation of `src/dsp/neon.h` from libwebp v1.6.0.
//!
//! Work package: WP20 (NEON).
//!
//! `neon.h` holds the NEON helpers that the `*_neon.c` files share:
//! `Transpose4x4_NEON` and the `INIT_VECTOR` macros. This module adds the
//! bounds-checked forms of the vector loads and stores that the NEON modules
//! make (ADR 0008, operation 1); they have no counterpart in upstream, which
//! loads and stores through pointers.
//!
//! # Deviations from upstream
//!
//! 1. **Loads and stores through slices.** Each `vld*`/`vst*` intrinsic that
//!    the NEON modules use has a function of the same name here that takes a
//!    slice from the first element the intrinsic reads or writes, checks that
//!    the slice holds every element it touches, and performs the intrinsic in
//!    one `unsafe` block. A NEON module imports these by name, which takes
//!    precedence over the glob import of `core::arch::aarch64`, so a
//!    translated line reads `vld1q_u8(&src[i..])` where C reads
//!    `vld1q_u8(src + i)`. A slice too short for the access panics where C
//!    would read or write outside its buffer, which is undefined.
//! 2. **Unaligned accesses only.** The `vld1`/`vst1` instructions that the
//!    intrinsics compile to need no alignment, and a Rust slice promises none;
//!    the byte forms read the elements of `u32` and `i16` buffers through a
//!    `u8` pointer, as the C code casts them. Where upstream relied on the
//!    alignment of a buffer, for example of `uint32_t tmp[2]` that
//!    `vst1_u8` writes, the Rust code uses the same unaligned instruction,
//!    with no effect on the result. This is the deviation class of ADR 0008;
//!    the other NEON modules refer to it.
//! 3. **Lane loads and stores of four bytes** (`vld1_lane_u32`,
//!    `vst1_lane_u32` and their `q` forms) are `u32::from_ne_bytes` and
//!    `to_ne_bytes` on a four-byte sub-slice with `vset_lane`/`vget_lane`,
//!    as ADR 0008 prefers for a few bytes; the result is the same lane.
//! 4. **`INIT_VECTOR2`, `INIT_VECTOR3` and `INIT_VECTOR4`** assign the
//!    members of a NEON array struct. Rust builds the tuple struct
//!    (`uint8x8x4_t(a, b, c, d)`) or assigns its fields (`.0` to `.3`); the
//!    macros have no function of their own.
//!
//! # Excluded undefined behavior
//!
//! None: the helpers check every access.
//!
//! # Checks not applicable
//!
//! 1. **`WEBP_USE_INTRINSICS`** is defined on 64-bit ARM (`WEBP_AARCH64`), so
//!    the NEON modules translate the intrinsic branches only; the inline
//!    assembly of the `#else` branches is not compiled on this architecture
//!    and has no translation.
//! 2. **`WORK_AROUND_GCC`** (and `WEBP_NEON_WORK_AROUND_GCC` of `cpu.h`) is
//!    not defined on 64-bit ARM, so the `!defined(WORK_AROUND_GCC)` branches are
//!    translated and the work-around branches are not.
//! 3. **`PRINT_REG`** is under `#if 0` and not compiled.
//! 4. **`WEBP_HAVE_NEON_RTCD`** selects the `/proc/cpuinfo` test of
//!    `armCPUInfo` for 32-bit ARM; on 64-bit ARM the query answers `kNEON`
//!    unconditionally (`src/dsp/cpu.rs`).
//! 5. **32-bit ARM.** The NEON modules are compiled for `target_arch =
//!    "aarch64"` only (`tools/units.json`); the `!WEBP_AARCH64` branches of
//!    the C files (`vpaddq` replacements, `vtbl` forms for 32-bit ARM) are not
//!    translated.

use core::arch::aarch64::{
    int16x4_t, int16x8_t, int32x4_t, int32x4x4_t, uint8x8_t, uint8x8x2_t, uint8x8x3_t, uint8x8x4_t,
    uint8x16_t, uint8x16x3_t, uint8x16x4_t, uint16x8_t, uint16x8x4_t, uint32x2_t, uint32x4_t,
    uint64x2x2_t, vcombine_u64, vget_high_u64, vget_low_u64, vreinterpretq_s32_u64,
    vreinterpretq_u64_s32, vtrnq_s32,
};

// upstream: src/dsp/neon.h Transpose4x4_NEON
/// Transposes the 4x4 matrix of 32-bit values in `rows`.
#[target_feature(enable = "neon")]
#[must_use]
#[expect(
    clippy::similar_names,
    reason = "the C names row01, row23, row0h, row2l and the like"
)]
#[expect(
    dead_code,
    reason = "the callers in dec_neon.rs and enc_neon.rs are translated next"
)]
pub(crate) fn transpose4x4_neon(rows: int32x4x4_t) -> int32x4x4_t {
    let mut row01 = uint64x2x2_t(vreinterpretq_u64_s32(rows.0), vreinterpretq_u64_s32(rows.1));
    let mut row23 = uint64x2x2_t(vreinterpretq_u64_s32(rows.2), vreinterpretq_u64_s32(rows.3));
    // Transpose 64-bit values (there's no vswp equivalent)
    {
        let row0h = vget_high_u64(row01.0);
        let row2l = vget_low_u64(row23.0);
        let row1h = vget_high_u64(row01.1);
        let row3l = vget_low_u64(row23.1);
        row01.0 = vcombine_u64(vget_low_u64(row01.0), row2l);
        row23.0 = vcombine_u64(row0h, vget_high_u64(row23.0));
        row01.1 = vcombine_u64(vget_low_u64(row01.1), row3l);
        row23.1 = vcombine_u64(row1h, vget_high_u64(row23.1));
    }
    {
        let out01 = vtrnq_s32(
            vreinterpretq_s32_u64(row01.0),
            vreinterpretq_s32_u64(row01.1),
        );
        let out23 = vtrnq_s32(
            vreinterpretq_s32_u64(row23.0),
            vreinterpretq_s32_u64(row23.1),
        );
        int32x4x4_t(out01.0, out01.1, out23.0, out23.1)
    }
}

//------------------------------------------------------------------------------
// Bounds-checked loads and stores (ADR 0008, operation 1). No upstream
// counterpart: C passes pointers to the intrinsics.

/// Defines a load of `$count` elements of type `$elem` from the start of a
/// slice through the intrinsic of the same name.
macro_rules! load {
    ($($name:ident: [$elem:ty; $count:literal] -> $vector:ty;)*) => {$(
        #[doc = concat!("`", stringify!($name), "` on the first ", stringify!($count),
            " elements of `src`, which must hold them.")]
        #[inline]
        #[must_use]
        pub fn $name(src: &[$elem]) -> $vector {
            let src = &src[..$count];
            // SAFETY: `src` holds the elements that the intrinsic reads (the
            // slice above checked the length); the load needs no alignment.
            unsafe { core::arch::aarch64::$name(src.as_ptr()) }
        }
    )*};
}

/// Defines a store of `$count` elements of type `$elem` to the start of a
/// slice through the intrinsic of the same name.
macro_rules! store {
    ($($name:ident: [$elem:ty; $count:literal] <- $vector:ty;)*) => {$(
        #[doc = concat!("`", stringify!($name), "` to the first ", stringify!($count),
            " elements of `dst`, which must hold them.")]
        #[inline]
        pub fn $name(dst: &mut [$elem], value: $vector) {
            let dst = &mut dst[..$count];
            // SAFETY: `dst` holds the elements that the intrinsic writes (the
            // slice above checked the length); the store needs no alignment.
            unsafe { core::arch::aarch64::$name(dst.as_mut_ptr(), value) }
        }
    )*};
}

load! {
    vld1_u8: [u8; 8] -> uint8x8_t;
    vld1q_u8: [u8; 16] -> uint8x16_t;
    vld3_u8: [u8; 24] -> uint8x8x3_t;
    vld4_u8: [u8; 32] -> uint8x8x4_t;
    vld4q_u8: [u8; 64] -> uint8x16x4_t;
    vld1_s16: [i16; 4] -> int16x4_t;
    vld1q_s16: [i16; 8] -> int16x8_t;
    vld1q_u16: [u16; 8] -> uint16x8_t;
    vld4q_u16: [u16; 32] -> uint16x8x4_t;
    vld1_u32: [u32; 2] -> uint32x2_t;
    vld1q_u32: [u32; 4] -> uint32x4_t;
    vld1q_s32: [i32; 4] -> int32x4_t;
}

store! {
    vst1_u8: [u8; 8] <- uint8x8_t;
    vst1q_u8: [u8; 16] <- uint8x16_t;
    vst2_u8: [u8; 16] <- uint8x8x2_t;
    vst3_u8: [u8; 24] <- uint8x8x3_t;
    vst3q_u8: [u8; 48] <- uint8x16x3_t;
    vst4_u8: [u8; 32] <- uint8x8x4_t;
    vst4q_u8: [u8; 64] <- uint8x16x4_t;
    vst1_s16: [i16; 4] <- int16x4_t;
    vst1q_s16: [i16; 8] <- int16x8_t;
    vst1q_u16: [u16; 8] <- uint16x8_t;
    vst1_u32: [u32; 2] <- uint32x2_t;
    vst1q_u32: [u32; 4] <- uint32x4_t;
    vst1q_s32: [i32; 4] <- int32x4_t;
}

/// `vld4_u8` on the bytes of the first 8 words of `src`, as C casts a
/// `uint32_t*` to `uint8_t*` for the load.
#[inline]
#[must_use]
pub fn vld4_u8_words(src: &[u32]) -> uint8x8x4_t {
    let src = &src[..8];
    // SAFETY: the 8 words of `src` (checked by the slice above) are the 32
    // bytes that the load reads; a byte pointer needs no alignment.
    unsafe { core::arch::aarch64::vld4_u8(src.as_ptr().cast::<u8>()) }
}

/// `vst4_u8` to the bytes of the first 8 words of `dst`, as C casts a
/// `uint32_t*` to `uint8_t*` for the store.
#[inline]
pub fn vst4_u8_words(dst: &mut [u32], value: uint8x8x4_t) {
    let dst = &mut dst[..8];
    // SAFETY: the 8 words of `dst` (checked by the slice above) are the 32
    // bytes that the store writes; a byte pointer needs no alignment.
    unsafe { core::arch::aarch64::vst4_u8(dst.as_mut_ptr().cast::<u8>(), value) }
}

/// `vld4q_u8` on the bytes of the first 16 words of `src`, as C casts a
/// `uint32_t*` to `uint8_t*` for the load.
#[inline]
#[must_use]
pub fn vld4q_u8_words(src: &[u32]) -> uint8x16x4_t {
    let src = &src[..16];
    // SAFETY: the 16 words of `src` (checked by the slice above) are the 64
    // bytes that the load reads; a byte pointer needs no alignment.
    unsafe { core::arch::aarch64::vld4q_u8(src.as_ptr().cast::<u8>()) }
}

/// `vst4q_u8` to the bytes of the first 16 words of `dst`, as C casts a
/// `uint32_t*` to `uint8_t*` for the store.
#[inline]
pub fn vst4q_u8_words(dst: &mut [u32], value: uint8x16x4_t) {
    let dst = &mut dst[..16];
    // SAFETY: the 16 words of `dst` (checked by the slice above) are the 64
    // bytes that the store writes; a byte pointer needs no alignment.
    unsafe { core::arch::aarch64::vst4q_u8(dst.as_mut_ptr().cast::<u8>(), value) }
}

/// The 4 bytes at the start of `src` as a native-endian word, which is what
/// `vld1_lane_u32` and `vld1q_lane_u32` load into their lane (deviation 3).
#[inline]
#[must_use]
pub fn load_u32_ne(src: &[u8]) -> u32 {
    u32::from_ne_bytes([src[0], src[1], src[2], src[3]])
}

/// Stores `value` to the first 4 bytes of `dst` in native byte order, which
/// is what `vst1_lane_u32` and `vst1q_lane_u32` store from their lane
/// (deviation 3).
#[inline]
pub fn store_u32_ne(dst: &mut [u8], value: u32) {
    dst[..4].copy_from_slice(&value.to_ne_bytes());
}
