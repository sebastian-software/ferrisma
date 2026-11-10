// libwebp v1.6.0, src/dsp/common_sse2.h:
//
// Copyright 2016 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// SSE2 code common to several files.
//
// Author: Vincent Rabaud (vrabaud@google.com)

//! Translation of `src/dsp/common_sse2.h` from libwebp v1.6.0.
//!
//! Work package: WP18 (SIMD x86 decoder); also used by the WP19 encoder modules.
//!
//! The SSE2 helpers that several SIMD modules share: the horizontal byte sum,
//! the double 4x4 transpose, and the channel mixing between planar and packed
//! pixels. Each is a `#[target_feature(enable = "sse2")]` function, safe to
//! call from the SSE2, SSE4.1 and AVX2 functions of the SIMD modules, whose
//! features imply SSE2 (ADR 0008).
//!
//! The module also holds the checked vector loads and stores that the SIMD
//! modules make their `unsafe` loads and stores through (ADR 0008, operation
//! 1): `loadu_si128`, `storeu_si128`, `loadl_epi64` and
//! `storel_epi64` take a slice, assert that it spans the bytes the
//! intrinsic reads or writes, and then perform the one unsafe operation. They
//! have no upstream counterpart; a C pointer cast to `__m128i*` becomes the
//! sub-slice at the pointer's index.
//!
//! # Deviations from upstream
//!
//! 1. **Unaligned loads and stores.** A Rust slice promises no alignment, so
//!    every aligned load or store of upstream (`_mm_load_si128`,
//!    `_mm_store_si128`) becomes its unaligned form in the SIMD modules of
//!    WP18. The result is the same; only the instruction differs.
//! 2. **Pointer parameters.** The `__m128i* const` parameters of the helpers
//!    become references; `VP8PlanarTo24bHelper`, a macro over six named
//!    registers, becomes a function from six registers to six.
//! 3. **Loads and stores out of bounds.** Where C reads or writes past a
//!    buffer through a vector load, the checked loads and stores panic before
//!    the access, so a panic can come earlier than in the scalar translation,
//!    which writes element by element (WP4, deviation 6). Valid inputs never
//!    reach it.
//!
//! # Excluded undefined behavior
//!
//! None: the helpers are arithmetic on registers.
//!
//! # Checks not applicable
//!
//! 1. **`PrintReg`.** The debugging helper is compiled out (`#if 0`) and is
//!    not translated.
//! 2. **Differential probe.** The helpers have no dispatch pointer of their
//!    own; the parity tests of the functions that use them cover them
//!    (`tests/wp18_*.rs`).
//!
//! # Notes for WP19
//!
//! - **Helpers.** `vp8_horizontal_add8b`, `vp8_transpose_2_4x4_16b`,
//!   `vp8_planar_to24b_sse2` and `vp8l32b_to_planar_sse2` keep the C names
//!   under ADR 0001's rule and the C argument order; out-parameters are
//!   `&mut __m128i`. Load and store through `loadu_si128`,
//!   `storeu_si128`, `loadl_epi64` and `storel_epi64` on the sub-slice
//!   at the C pointer, and read or write four bytes with
//!   `webp_mem_to_int32`/`webp_int32_to_mem` of `src/utils/utils.rs`, as
//!   upstream does with `WebPMemToInt32`/`WebPInt32ToMem`.
//! - **Entry points.** Each function that an `Init` function assigns to a
//!   pointer keeps the upstream name as a `#[target_feature]` function
//!   (`transform_sse2` for `Transform_SSE2`) and gets a safe `pub(crate)`
//!   entry point named after it without the suffix (`transform`), whose one
//!   `unsafe` block calls the body; its `SAFETY:` comment names the
//!   `features()` flag that the dispatch function tested before calling it.
//!   The entry points are `pub(crate)`, so that no caller outside the crate
//!   can reach a body without the selection.
//! - **`Init` order.** The dispatch function's x86 block tests the levels in
//!   the reverse order of the `Init` sequence (AVX2, SSE4.1, SSE2) and mirrors
//!   its nesting: `VP8DspInit`, `WebPInitAlphaProcessing` and `VP8LDspInit`
//!   run their SSE4.1 `Init` only inside the SSE2 branch (and `VP8LDspInit`
//!   its AVX2 one inside the SSE4.1 branch), while `WebPInitSamplers`,
//!   `WebPInitUpsamplers`, `WebPInitYUV444Converters` and
//!   `WebPInitConvertARGBToYUV` test SSE2 and SSE4.1 independently.
//!   `VP8EncDspInit` and the other encoder `Init` functions have their own
//!   structure; read them before writing the blocks.
//! - **Tests.** `tests/wp18_support/mod.rs` installs a `VP8GetCPUInfo` query
//!   per level (C, SSE2, SSE4.1, AVX2; each answers the running CPU's flags up
//!   to its level) behind a lock, so that a test runs the dispatch function
//!   once per level on copies of the same seeded input and compares the
//!   outputs with the C level's. The benchmark cases of
//!   `examples/dsp_bench/` select the levels the same way.

#[cfg(target_arch = "x86")]
use core::arch::x86 as arch;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64 as arch;

use arch::{
    __m128i, _mm_add_epi32, _mm_and_si128, _mm_cvtsi128_si32, _mm_loadl_epi64, _mm_loadu_si128,
    _mm_packus_epi16, _mm_sad_epu8, _mm_set1_epi16, _mm_setzero_si128, _mm_shuffle_epi32,
    _mm_srli_epi16, _mm_storel_epi64, _mm_storeu_si128, _mm_unpackhi_epi8, _mm_unpackhi_epi16,
    _mm_unpackhi_epi32, _mm_unpackhi_epi64, _mm_unpacklo_epi8, _mm_unpacklo_epi16,
    _mm_unpacklo_epi32, _mm_unpacklo_epi64,
};

//------------------------------------------------------------------------------
// Checked loads and stores (ADR 0008, operation 1). No upstream counterpart.

/// The element types of the slices that the checked loads and stores accept:
/// the primitive integers, whose bytes are all initialized, so a vector load
/// over them reads defined bytes. Implemented for no other type.
pub(crate) trait Lane: Copy {}

impl Lane for u8 {}
impl Lane for i8 {}
impl Lane for u16 {}
impl Lane for i16 {}
impl Lane for u32 {}
impl Lane for i32 {}
impl Lane for u64 {}
impl Lane for i64 {}

/// Loads the first 16 bytes of `src` (C: `_mm_loadu_si128` and
/// `_mm_load_si128` at the start of `src`).
///
/// # Panics
///
/// Panics when `src` spans fewer than 16 bytes, where C would read past it.
#[target_feature(enable = "sse2")]
#[inline]
pub(crate) fn loadu_si128<T: Lane>(src: &[T]) -> __m128i {
    assert!(
        size_of_val(src) >= 16,
        "a 16-byte load past the end of a slice"
    );
    // SAFETY: the assertion above checked that `src` spans the 16 bytes that
    // the unaligned load reads, and `Lane` types have no uninitialized bytes.
    unsafe { _mm_loadu_si128(src.as_ptr().cast()) }
}

/// Stores `value` into the first 16 bytes of `dst` (C: `_mm_storeu_si128`
/// and `_mm_store_si128` at the start of `dst`).
///
/// # Panics
///
/// Panics when `dst` spans fewer than 16 bytes, where C would write past it.
#[target_feature(enable = "sse2")]
#[inline]
pub(crate) fn storeu_si128<T: Lane>(dst: &mut [T], value: __m128i) {
    assert!(
        size_of_val(dst) >= 16,
        "a 16-byte store past the end of a slice"
    );
    // SAFETY: the assertion above checked that `dst` spans the 16 bytes that
    // the unaligned store writes, and every byte pattern is a valid `Lane`.
    unsafe { _mm_storeu_si128(dst.as_mut_ptr().cast(), value) }
}

/// Loads the first 8 bytes of `src` into the low half of a register and zeroes
/// the high half (C: `_mm_loadl_epi64` at the start of `src`).
///
/// # Panics
///
/// Panics when `src` spans fewer than 8 bytes, where C would read past it.
#[target_feature(enable = "sse2")]
#[inline]
pub(crate) fn loadl_epi64<T: Lane>(src: &[T]) -> __m128i {
    assert!(
        size_of_val(src) >= 8,
        "an 8-byte load past the end of a slice"
    );
    // SAFETY: the assertion above checked that `src` spans the 8 bytes that
    // the unaligned load reads, and `Lane` types have no uninitialized bytes.
    unsafe { _mm_loadl_epi64(src.as_ptr().cast()) }
}

/// Stores the low 8 bytes of `value` into the first 8 bytes of `dst` (C:
/// `_mm_storel_epi64` at the start of `dst`).
///
/// # Panics
///
/// Panics when `dst` spans fewer than 8 bytes, where C would write past it.
#[target_feature(enable = "sse2")]
#[inline]
pub(crate) fn storel_epi64<T: Lane>(dst: &mut [T], value: __m128i) {
    assert!(
        size_of_val(dst) >= 8,
        "an 8-byte store past the end of a slice"
    );
    // SAFETY: the assertion above checked that `dst` spans the 8 bytes that
    // the unaligned store writes, and every byte pattern is a valid `Lane`.
    unsafe { _mm_storel_epi64(dst.as_mut_ptr().cast(), value) }
}

//------------------------------------------------------------------------------
// Math functions.

// upstream: src/dsp/common_sse2.h VP8HorizontalAdd8b
/// Return the sum of all the 8b in the register.
#[target_feature(enable = "sse2")]
#[inline]
#[expect(
    dead_code,
    reason = "only the encoder's enc_sse2.rs (WP19) uses it; WP19 removes this expectation"
)]
pub(crate) fn vp8_horizontal_add8b(a: &__m128i) -> i32 {
    let zero = _mm_setzero_si128();
    let sad8x2 = _mm_sad_epu8(*a, zero);
    // sum the two sads: sad8x2[0:1] + sad8x2[8:9]
    let sum = _mm_add_epi32(sad8x2, _mm_shuffle_epi32::<2>(sad8x2));
    _mm_cvtsi128_si32(sum)
}

// upstream: src/dsp/common_sse2.h VP8Transpose_2_4x4_16b
/// Transpose two 4x4 16b matrices horizontally stored in registers.
#[target_feature(enable = "sse2")]
#[inline]
#[expect(clippy::too_many_arguments, reason = "the C parameters")]
pub(crate) fn vp8_transpose_2_4x4_16b(
    in0: &__m128i,
    in1: &__m128i,
    in2: &__m128i,
    in3: &__m128i,
    out0: &mut __m128i,
    out1: &mut __m128i,
    out2: &mut __m128i,
    out3: &mut __m128i,
) {
    // Transpose the two 4x4.
    // a00 a01 a02 a03   b00 b01 b02 b03
    // a10 a11 a12 a13   b10 b11 b12 b13
    // a20 a21 a22 a23   b20 b21 b22 b23
    // a30 a31 a32 a33   b30 b31 b32 b33
    let transpose0_0 = _mm_unpacklo_epi16(*in0, *in1);
    let transpose0_1 = _mm_unpacklo_epi16(*in2, *in3);
    let transpose0_2 = _mm_unpackhi_epi16(*in0, *in1);
    let transpose0_3 = _mm_unpackhi_epi16(*in2, *in3);
    // a00 a10 a01 a11   a02 a12 a03 a13
    // a20 a30 a21 a31   a22 a32 a23 a33
    // b00 b10 b01 b11   b02 b12 b03 b13
    // b20 b30 b21 b31   b22 b32 b23 b33
    let transpose1_0 = _mm_unpacklo_epi32(transpose0_0, transpose0_1);
    let transpose1_1 = _mm_unpacklo_epi32(transpose0_2, transpose0_3);
    let transpose1_2 = _mm_unpackhi_epi32(transpose0_0, transpose0_1);
    let transpose1_3 = _mm_unpackhi_epi32(transpose0_2, transpose0_3);
    // a00 a10 a20 a30 a01 a11 a21 a31
    // b00 b10 b20 b30 b01 b11 b21 b31
    // a02 a12 a22 a32 a03 a13 a23 a33
    // b02 b12 a22 b32 b03 b13 b23 b33
    *out0 = _mm_unpacklo_epi64(transpose1_0, transpose1_1);
    *out1 = _mm_unpackhi_epi64(transpose1_0, transpose1_1);
    *out2 = _mm_unpacklo_epi64(transpose1_2, transpose1_3);
    *out3 = _mm_unpackhi_epi64(transpose1_2, transpose1_3);
    // a00 a10 a20 a30   b00 b10 b20 b30
    // a01 a11 a21 a31   b01 b11 b21 b31
    // a02 a12 a22 a32   b02 b12 b22 b32
    // a03 a13 a23 a33   b03 b13 b23 b33
}

//------------------------------------------------------------------------------
// Channel mixing.

// upstream: src/dsp/common_sse2.h VP8PlanarTo24bHelper
/// Function used several times in `VP8PlanarTo24b`.
/// It samples the in buffer as follows: one every two unsigned char is stored
/// at the beginning of the buffer, while the other half is stored at the end.
#[target_feature(enable = "sse2")]
#[inline]
fn vp8_planar_to24b_helper(input: &[__m128i; 6]) -> [__m128i; 6] {
    let v_mask = _mm_set1_epi16(0x00ff);
    [
        // Take one every two upper 8b values.
        _mm_packus_epi16(
            _mm_and_si128(input[0], v_mask),
            _mm_and_si128(input[1], v_mask),
        ),
        _mm_packus_epi16(
            _mm_and_si128(input[2], v_mask),
            _mm_and_si128(input[3], v_mask),
        ),
        _mm_packus_epi16(
            _mm_and_si128(input[4], v_mask),
            _mm_and_si128(input[5], v_mask),
        ),
        // Take one every two lower 8b values.
        _mm_packus_epi16(_mm_srli_epi16::<8>(input[0]), _mm_srli_epi16::<8>(input[1])),
        _mm_packus_epi16(_mm_srli_epi16::<8>(input[2]), _mm_srli_epi16::<8>(input[3])),
        _mm_packus_epi16(_mm_srli_epi16::<8>(input[4]), _mm_srli_epi16::<8>(input[5])),
    ]
}

// upstream: src/dsp/common_sse2.h VP8PlanarTo24b_SSE2
/// Pack the planar buffers
/// rrrr... rrrr... gggg... gggg... bbbb... bbbb....
/// triplet by triplet in the output buffer rgb as rgbrgbrgbrgb ...
#[target_feature(enable = "sse2")]
#[inline]
pub(crate) fn vp8_planar_to24b_sse2(
    in0: &mut __m128i,
    in1: &mut __m128i,
    in2: &mut __m128i,
    in3: &mut __m128i,
    in4: &mut __m128i,
    in5: &mut __m128i,
) {
    // The input is 6 registers of sixteen 8b but for the sake of explanation,
    // let's take 6 registers of four 8b values.
    // To pack, we will keep taking one every two 8b integer and move it
    // around as follows:
    // Input:
    //   r0r1r2r3 | r4r5r6r7 | g0g1g2g3 | g4g5g6g7 | b0b1b2b3 | b4b5b6b7
    // Split the 6 registers in two sets of 3 registers: the first set as the even
    // 8b bytes, the second the odd ones:
    //   r0r2r4r6 | g0g2g4g6 | b0b2b4b6 | r1r3r5r7 | g1g3g5g7 | b1b3b5b7
    // Repeat the same permutations twice more:
    //   r0r4g0g4 | b0b4r1r5 | g1g5b1b5 | r2r6g2g6 | b2b6r3r7 | g3g7b3b7
    //   r0g0b0r1 | g1b1r2g2 | b2r3g3b3 | r4g4b4r5 | g5b5r6g6 | b6r7g7b7
    let input = [*in0, *in1, *in2, *in3, *in4, *in5];
    let tmp = vp8_planar_to24b_helper(&input);
    let input = vp8_planar_to24b_helper(&tmp);
    let tmp = vp8_planar_to24b_helper(&input);
    // We need to do it two more times than the example as we have sixteen bytes.
    let out = vp8_planar_to24b_helper(&tmp);
    let input = vp8_planar_to24b_helper(&out);
    [*in0, *in1, *in2, *in3, *in4, *in5] = input;
}

// upstream: src/dsp/common_sse2.h VP8L32bToPlanar_SSE2
/// Convert four packed four-channel buffers like argbargbargbargb... into the
/// split channels aaaaa ... rrrr ... gggg .... bbbbb ......
#[target_feature(enable = "sse2")]
#[inline]
pub(crate) fn vp8l32b_to_planar_sse2(
    in0: &mut __m128i,
    in1: &mut __m128i,
    in2: &mut __m128i,
    in3: &mut __m128i,
) {
    // Column-wise transpose.
    let a0 = _mm_unpacklo_epi8(*in0, *in1);
    let a1 = _mm_unpackhi_epi8(*in0, *in1);
    let a2 = _mm_unpacklo_epi8(*in2, *in3);
    let a3 = _mm_unpackhi_epi8(*in2, *in3);
    let b0 = _mm_unpacklo_epi8(a0, a1);
    let b1 = _mm_unpackhi_epi8(a0, a1);
    let b2 = _mm_unpacklo_epi8(a2, a3);
    let b3 = _mm_unpackhi_epi8(a2, a3);
    // C0 = g7 g6 ... g1 g0 | b7 b6 ... b1 b0
    // C1 = a7 a6 ... a1 a0 | r7 r6 ... r1 r0
    let c0 = _mm_unpacklo_epi8(b0, b1);
    let c1 = _mm_unpackhi_epi8(b0, b1);
    let c2 = _mm_unpacklo_epi8(b2, b3);
    let c3 = _mm_unpackhi_epi8(b2, b3);
    // Gather the channels.
    *in0 = _mm_unpackhi_epi64(c1, c3);
    *in1 = _mm_unpacklo_epi64(c1, c3);
    *in2 = _mm_unpackhi_epi64(c0, c2);
    *in3 = _mm_unpacklo_epi64(c0, c2);
}
