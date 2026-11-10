// libwebp v1.6.0, src/dsp/dec_neon.c:
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
// ARM NEON version of dsp functions and loop filtering.
//
// Authors: Somnath Banerjee (somnath@google.com)
//          Johann Koenig (johannkoenig@google.com)

//! Translation of `src/dsp/dec_neon.c` from libwebp v1.6.0.
//!
//! Work package: WP20 (NEON).
//!
//! The NEON variants of the inverse transforms, the loop filters and the
//! intra predictors of `src/dsp/dec.c`. Each `_neon` function keeps the
//! upstream name; the safe entry point below it, named after the function
//! without the suffix, is what the dispatch functions of `src/dsp/dec.rs`
//! call on 64-bit ARM.
//!
//! `VP8DspInitNEON` assigns `VP8Transform` (`TransformTwo_NEON`),
//! `VP8TransformAC3`, `VP8TransformDC`, `VP8TransformWHT`, the twelve loop
//! filters (`VP8VFilter16`, `VP8VFilter16i`, `VP8HFilter16`, `VP8HFilter16i`,
//! `VP8VFilter8`, `VP8VFilter8i`, `VP8HFilter8`, `VP8HFilter8i`,
//! `VP8SimpleVFilter16`, `VP8SimpleHFilter16`, `VP8SimpleVFilter16i`,
//! `VP8SimpleHFilter16i`), the entries 0, 1, 2, 4 and 6 of `VP8PredLuma4`
//! and every entry of `VP8PredLuma16` and `VP8PredChroma8`. `VP8DspInit`
//! calls it unconditionally on 64-bit ARM (`WEBP_NEON_OMIT_C_CODE`, which
//! also omits exactly those C functions there), so those dispatch functions
//! and table entries return the NEON entry points; `VP8TransformUV`,
//! `VP8TransformDCUV`, `VP8DitherCombine8x8` and the entries 3, 5, 7, 8 and
//! 9 of `VP8PredLuma4` stay the C functions. `TransformUV_C` and
//! `TransformDCUV_C` call the `VP8Transform` and `VP8TransformDC` dispatch
//! functions, and so the NEON transforms, as in C.
//!
//! # Deviations from upstream
//!
//! 1. **Loads, stores and alignment** follow the deviations 1 to 3 of
//!    `src/dsp/neon.rs`. The four-byte row stores of the transforms and
//!    predictors (`vst1_lane_u32` through a `uint32_t*` cast of the byte
//!    pointer) and the four-byte loads of `Add4x4_NEON` (`vld1_lane_u32`)
//!    are `store_u32_ne` and `load_u32_ne` with the lane.
//! 2. **Pointers and strides.** As in `src/dsp/dec.rs`, a pointer that the
//!    code reads backwards from is a slice and the position of the pointer in
//!    it: the predictors' `dst`, the filters' `p`, `u` and `v`. The transforms'
//!    `in`, `dst` and `out` are slices from the pointer. A position before
//!    the start of the slice or an access past its end panics, where C would
//!    read or write outside the buffer.
//! 3. **Output parameters.** The helpers that C gives pointers to their
//!    results (`Load4x16_NEON`, `DoFilter2_NEON`, `TransformPass_NEON` and the
//!    other loaders and filters) return their results as tuples, in the
//!    order of the C parameters; the in-place updates of
//!    `ApplyFilter2NoFlip_NEON` and `TransformPass_NEON` assign the returned
//!    values.
//! 4. **Lane and duplicating loads and stores.** `vld4_lane_u8`,
//!    `vst2_lane_u8`, `vst3_lane_u8` and `vst4_lane_u8` move 2 to 4 bytes of
//!    one lane; they are safe functions of this module that read or write
//!    those bytes of a sub-slice with `vset_lane_u8` and `vget_lane_u8`.
//!    `vld1_dup_u8`, `vld1q_dup_u8` and `vld1_dup_s16` load one element into
//!    every lane; they are `vdup_n_u8`, `vdupq_n_u8` and `vdup_n_s16` of the
//!    element read from the slice. The result is the same vector or memory.
//! 5. **Eight-byte stores of `uint32x2_t`.** `vst1_u32` through a
//!    `uint32_t*` cast of the byte pointer (in `TrueMotion_NEON` and
//!    `DC8_NEON`) is `vst1_u8` of the same register reinterpreted as bytes,
//!    which writes the same eight bytes.
//! 6. **Macros.** `LOAD_UV_8`, `STORE6_LANE` and `STORE_WHT` are functions;
//!    `STORE6_LANE` and `STORE_WHT` return the advanced position that the
//!    macros assign to their pointer argument.
//! 7. **Unset vectors.** `DC8_NEON` and `DC16_NEON` leave `sum_top` or
//!    `sum_left` unset when the edge is absent and never read it then; Rust
//!    initializes both to zero first.
//!
//! # Excluded undefined behavior
//!
//! None. The functions read and write only through checked slices.
//!
//! # Checks not applicable
//!
//! 1. **Inline assembly.** `WEBP_USE_INTRINSICS` is defined on 64-bit ARM, so
//!    the assembly versions of `SimpleVFilter16_NEON`, `SimpleHFilter16_NEON`
//!    and `TransformOne_NEON`, with the `LOAD8x4`, `STORE8x2`, `QRegs`,
//!    `FLIP_SIGN_BIT2`, `FLIP_SIGN_BIT4`, `NEEDS_FILTER`, `GET_BASE_DELTA`,
//!    `DO_SIMPLE_FILTER` and `DO_FILTER2` macros, are not compiled there and
//!    not translated (`src/dsp/neon.rs`, checks not applicable 1).
//! 2. **`WORK_AROUND_GCC`** is not defined on 64-bit ARM: the
//!    `!defined(WORK_AROUND_GCC)` branches (`Load4x8_NEON`, the lane form of
//!    `Load4x16_NEON`, `Load8x8x2T_NEON`, `Store4x8_NEON`, `Store4x16_NEON`,
//!    `Store6x8x2_NEON`, `Store4x8x2_NEON`, `HFilter16i_NEON`,
//!    `HFilter8_NEON`, `HFilter8i_NEON`) are translated, and the
//!    work-around `Load4x16_NEON` with `LOADQ_LANE_32b` is not.
//! 3. **32-bit ARM.** The `!WEBP_AARCH64` branches of `DC8_NEON` and
//!    `DC16_NEON` (`vpaddl`/`vpadd` cascades instead of `vaddlv_u8` and
//!    `vaddlvq_u8`) are not compiled on 64-bit ARM.
//! 4. **`WEBP_RESTRICT`** has no counterpart: the slices cannot alias.
//! 5. **Filter thresholds.** `NeedsFilter_NEON`, `NeedsFilter2_NEON` and
//!    `NeedsHev_NEON` convert `thresh`, `ithresh` and `hev_thresh` to
//!    `uint8_t` and compare with saturated sums, where the C functions
//!    compare `int` sums with `2 * thresh + 1`. The two agree for every
//!    threshold from 0 to 254, which covers what the decoder passes (at most
//!    `2 * 63 + 63 + 4`); the parity test covers that range. Above it
//!    upstream's NEON and C functions differ, and the translation keeps the
//!    NEON conversion. `tests/wp6_dsp.rs`, whose filter records draw
//!    thresholds up to 1200, verifies the C translations directly (decided on
//!    issue #28); against the `release-simd` oracle the Rust filters gave
//!    upstream's NEON bytes for those records.
//! 6. **Transform coefficients outside `[-2048, 2047]`.** `TransformOne_NEON`
//!    (and with it `TransformTwo_NEON`, which `VP8Transform` and
//!    `TransformUV_C` reach) and `TransformAC3_NEON` compute in saturating
//!    16-bit lanes (`vqadd_s16`, `vqdmulhq_n_s16`), where `TransformOne_C` and
//!    `TransformAC3_C` compute in `int`. The two agree for coefficients in
//!    `[-2048, 2047]`, the input range that `TransformOne_C`'s own comment
//!    assumes and that the coefficients of encoded streams stay in; the parity
//!    test covers it, with every sign pattern of the bounds. A corrupt or
//!    crafted stream can code larger dequantized coefficients, and decodes
//!    differently in upstream's NEON and C builds; the translation keeps
//!    upstream's NEON arithmetic and selection, so on 64-bit ARM it decodes
//!    such streams as upstream's NEON build does (admitted on issue #28,
//!    checkpoint 4; ADR 0008 through #41). `tests/wp6_dsp.rs` verifies the C
//!    transforms directly for its records of up to ±15000, and
//!    `tests/wp6_decode.rs` skips, with `simd` on 64-bit ARM only, the
//!    mutated streams that code a block outside the range, such as the
//!    mutated `lossy_alpha1.webp` that codes -21546 at `in[15]`; against the
//!    `release-simd` oracle both gave upstream's NEON bytes.
//!
//! # Notes for WP18 and WP19
//!
//! WP20 added its 64-bit ARM selection to these dispatch functions, each in the
//! layout of the brief: the `#[cfg(all(feature = "simd", target_arch =
//! "aarch64"))]` block that returns the NEON entry point directly above the
//! C call, which carries `#[allow(unreachable_code, reason = "the C path is
//! reached only without a SIMD variant")]`. The x86 block goes above the
//! 64-bit ARM block, so a merge keeps both.
//!
//! - `src/dsp/dec.rs`: `vp8_transform`, `vp8_transform_ac3`,
//!   `vp8_transform_dc`, `vp8_transform_wht`, the twelve loop filters
//!   (`vp8_simple_v_filter16` to `vp8h_filter8i`); the tables
//!   `vp8_pred_luma4`, `vp8_pred_luma16` and `vp8_pred_chroma8` select per
//!   entry from a NEON table before indexing the C table (`VP8PredLuma4`
//!   entries 0, 1, 2, 4 and 6 only). `vp8_transform_uv`, `vp8_transform_dcuv`
//!   and `vp8_dither_combine8x8` have no NEON variant.
//! - `src/dsp/enc.rs`: `vp8_collect_histogram`, `vp8i_transform`,
//!   `vp8f_transform`, `vp8f_transform_wht`, `vp8_enc_pred_luma4`,
//!   `vp8_enc_pred_luma16`, `vp8sse16x16`, `vp8sse16x8`, `vp8sse8x8`,
//!   `vp8sse4x4`, `vp8t_disto4x4`, `vp8t_disto16x16` and the three quantizers;
//!   `src/dsp/quant.rs`: `is_flat` (whose portable body is now `is_flat_c`).
//! - `src/dsp/cost.rs`: `vp8_get_residual_cost` and `vp8_set_residual_coeffs`
//!   select on `Features::query_neon` (the query's `kNEON` answer), as
//!   `VP8EncDspCostInit` does, and fall through to the C call.
//! - `src/dsp/lossless.rs`: `vp8l_add_green_to_blue_and_red`,
//!   `vp8l_transform_color_inverse`, the converters to RGBA, BGR and RGB; the
//!   tables `vp8l_predictors` (entries 5, 6, 7, 13) and `vp8l_predictors_add`
//!   (0 to 13) select per entry. `src/dsp/lossless_enc.rs`:
//!   `vp8l_subtract_green_from_blue_and_red`, `vp8l_transform_color`.
//! - `src/dsp/alpha_processing.rs`: `webp_apply_alpha_multiply`,
//!   `webp_dispatch_alpha`, `webp_dispatch_alpha_to_green`,
//!   `webp_extract_alpha`, `webp_extract_green`. `src/dsp/yuv.rs`: the five
//!   `webp_convert_*` functions. `src/dsp/upsampling.rs`: `webp_upsamplers`
//!   and `webp_get_line_pair_converter` per mode. `src/dsp/filters.rs`:
//!   `webp_filters` and `webp_unfilters` per filter type.
//!   `src/sharpyuv/sharpyuv_dsp.rs`: the three dispatch functions.
//! - `src/dsp/rescaler.rs`: none; the NEON export rows are not selected
//!   (`src/dsp/rescaler_neon.rs`).
//!
//! The replaced private `fn xxx_c` functions became plain `pub fn xxx_c` for
//! the parity tests, with `# Panics` sections and `#[must_use]` where Clippy
//! asked and nothing else changed on the line; make the identical change if
//! you need the same function. `src/dsp/cpu.rs` gained `Features::query_neon`,
//! and `vp8_get_cpu_info` and the C ABI's `VP8GetCPUInfo` answer `kNEON` only
//! with `simd` on 64-bit ARM. The benchmark modules are named after the NEON
//! modules (`examples/dsp_bench/dec_neon.rs` and so on) and registered under
//! `#[cfg(target_arch = "aarch64")]`.
//!
//! # Notes for Ferrixel
//!
//! With the default `simd` feature on 64-bit ARM, these functions have a NEON
//! path, selected without a run-time query as upstream does (`-noasm` through
//! the C ABI turns off only the residual cost functions): the decoder's
//! inverse transforms, loop filters and intra predictors; the encoder's
//! transforms, distortions, sums of squared errors, histogram, quantizers,
//! intra predictors, flatness test and residual costs; the lossless decoder's
//! predictors, inverse transforms and color converters, and the lossless
//! encoder's subtract-green and color transforms; the alpha dispatch,
//! extraction and premultiplication; the RGB to YUV converters; the fancy
//! upsamplers; the alpha-plane filters; and the `SharpYUV` kernels. The
//! rescaler's export rows stay scalar (issue #38). Each NEON function gives
//! the C function's bytes on the inputs the library passes; the recorded
//! exceptions are corrupt or crafted streams whose transform coefficients
//! leave `[-2048, 2047]` (checks not applicable 6), which decode as upstream's
//! NEON build decodes them. `--no-default-features` builds the scalar
//! translation. The benchmark table (host, CPU, `rustc`) is in the
//! description of the WP20 pull request; `cargo run --release --example
//! dsp_bench` reproduces it.

use core::arch::aarch64::{
    int8x16_t, int16x8_t, int16x8x2_t, int32x4x4_t, uint8x8_t, uint8x8x2_t, uint8x8x3_t,
    uint8x8x4_t, uint8x16_t, uint32x2_t, vabdq_u8, vaddl_s16, vaddl_u8, vaddlv_u8, vaddlvq_u8,
    vaddq_s32, vaddq_u16, vaddw_u8, vandq_s8, vandq_u8, vcgeq_u8, vcgtq_u8, vcombine_s8,
    vcombine_s16, vcombine_u8, vcombine_u16, vcreate_s16, vcreate_u64, vdup_lane_u8, vdup_n_s8,
    vdup_n_s16, vdup_n_u8, vdup_n_u32, vdupq_lane_u8, vdupq_n_s8, vdupq_n_s16, vdupq_n_s32,
    vdupq_n_u8, vdupq_n_u16, veorq_s8, veorq_u8, vget_high_s8, vget_high_s16, vget_high_u8,
    vget_lane_u8, vget_lane_u32, vget_low_s8, vget_low_s16, vget_low_u8, vgetq_lane_s32, vhadd_u8,
    vmaxq_u8, vmlal_s8, vmovl_u8, vorr_u64, vpadd_u16, vpaddl_u8, vqadd_s16, vqaddq_s8, vqaddq_s16,
    vqaddq_u8, vqdmulhq_n_s16, vqmovun_s16, vqrshrn_n_s16, vqsub_s16, vqsubq_s8, vqsubq_s16,
    vreinterpret_u8_u32, vreinterpret_u8_u64, vreinterpret_u32_u8, vreinterpret_u32_u64,
    vreinterpret_u64_u8, vreinterpretq_s8_u8, vreinterpretq_s16_u16, vreinterpretq_u8_s8,
    vreinterpretq_u8_u32, vreinterpretq_u16_u8, vreinterpretq_u32_u16, vrhadd_u8, vrshrn_n_u16,
    vrshrq_n_s8, vrsraq_n_s16, vset_lane_u8, vset_lane_u32, vshl_n_u64, vshr_n_u64, vshrq_n_s8,
    vshrq_n_s32, vshrq_n_u8, vsraq_n_s16, vsubl_s16, vsubl_u8, vsubq_s32, vtrnq_u8, vtrnq_u16,
    vtrnq_u32, vzipq_s16,
};

use crate::dsp::dsp::{
    BPS, WEBP_TRANSFORM_AC3_C1, WEBP_TRANSFORM_AC3_C2, webp_transform_ac3_mul1,
    webp_transform_ac3_mul2,
};
use crate::dsp::neon::{
    load_u32_ne, store_u32_ne, transpose4x4_neon, vld1_s16, vld1_u8, vld1q_s16, vld1q_u8, vst1_u8,
    vst1q_u8,
};

/// `BPS` as an index step.
const BPS_U: usize = BPS as usize;

/// Eight vectors, as the 8-row loaders return them (`p3` to `q3`).
type Vectors8 = (
    uint8x16_t,
    uint8x16_t,
    uint8x16_t,
    uint8x16_t,
    uint8x16_t,
    uint8x16_t,
    uint8x16_t,
    uint8x16_t,
);

/// Six vectors, as the 6-point filters return them (`op2` to `oq2`).
type Vectors6 = (
    uint8x16_t,
    uint8x16_t,
    uint8x16_t,
    uint8x16_t,
    uint8x16_t,
    uint8x16_t,
);

/// Four vectors (`p1`, `p0`, `q0`, `q1` or their filtered values).
type Vectors4 = (uint8x16_t, uint8x16_t, uint8x16_t, uint8x16_t);

/// The position `delta` elements away from `pos`, as C's pointer arithmetic
/// `p + delta`.
fn at(pos: usize, delta: i32) -> usize {
    pos.checked_add_signed(isize::try_from(delta).expect("an i32 fits in isize"))
        .expect("a position before the start of the buffer")
}

//------------------------------------------------------------------------------
// Lane loads and stores of a few bytes (deviation 4). No unsafe: the bytes are
// read and written through the slice.

/// `vld4_lane_u8`: the 4 bytes at the start of `src` into lane `LANE` of the
/// four vectors of `v`.
#[target_feature(enable = "neon")]
fn vld4_lane_u8<const LANE: i32>(src: &[u8], v: uint8x8x4_t) -> uint8x8x4_t {
    let src = &src[..4];
    uint8x8x4_t(
        vset_lane_u8::<LANE>(src[0], v.0),
        vset_lane_u8::<LANE>(src[1], v.1),
        vset_lane_u8::<LANE>(src[2], v.2),
        vset_lane_u8::<LANE>(src[3], v.3),
    )
}

/// `vst2_lane_u8`: lane `LANE` of the two vectors of `v` to the first 2 bytes
/// of `dst`.
#[target_feature(enable = "neon")]
fn vst2_lane_u8<const LANE: i32>(dst: &mut [u8], v: uint8x8x2_t) {
    let dst = &mut dst[..2];
    dst[0] = vget_lane_u8::<LANE>(v.0);
    dst[1] = vget_lane_u8::<LANE>(v.1);
}

/// `vst3_lane_u8`: lane `LANE` of the three vectors of `v` to the first 3
/// bytes of `dst`.
#[target_feature(enable = "neon")]
fn vst3_lane_u8<const LANE: i32>(dst: &mut [u8], v: uint8x8x3_t) {
    let dst = &mut dst[..3];
    dst[0] = vget_lane_u8::<LANE>(v.0);
    dst[1] = vget_lane_u8::<LANE>(v.1);
    dst[2] = vget_lane_u8::<LANE>(v.2);
}

/// `vst4_lane_u8`: lane `LANE` of the four vectors of `v` to the first 4
/// bytes of `dst`.
#[target_feature(enable = "neon")]
fn vst4_lane_u8<const LANE: i32>(dst: &mut [u8], v: uint8x8x4_t) {
    let dst = &mut dst[..4];
    dst[0] = vget_lane_u8::<LANE>(v.0);
    dst[1] = vget_lane_u8::<LANE>(v.1);
    dst[2] = vget_lane_u8::<LANE>(v.2);
    dst[3] = vget_lane_u8::<LANE>(v.3);
}

/// `vst1_lane_u32(dst, v, 0)` through a `uint32_t*` cast of the byte
/// pointer: lane 0 of `v` as a native-endian word at the start of `dst`.
#[target_feature(enable = "neon")]
fn store_lane0_u32(dst: &mut [u8], v: uint32x2_t) {
    store_u32_ne(dst, vget_lane_u32::<0>(v));
}

//------------------------------------------------------------------------------
// NxM Loading functions

// upstream: src/dsp/dec_neon.c Load4x8_NEON
/// Loads the 4 bytes at `pos` of each of 8 rows into the lanes of four
/// vectors.
#[target_feature(enable = "neon")]
fn load4x8_neon(src: &[u8], pos: usize, stride: i32) -> uint8x8x4_t {
    let zero = vdup_n_u8(0);
    let mut out = uint8x8x4_t(zero, zero, zero, zero);
    out = vld4_lane_u8::<0>(&src[pos..], out);
    out = vld4_lane_u8::<1>(&src[at(pos, stride)..], out);
    out = vld4_lane_u8::<2>(&src[at(pos, 2 * stride)..], out);
    out = vld4_lane_u8::<3>(&src[at(pos, 3 * stride)..], out);
    out = vld4_lane_u8::<4>(&src[at(pos, 4 * stride)..], out);
    out = vld4_lane_u8::<5>(&src[at(pos, 5 * stride)..], out);
    out = vld4_lane_u8::<6>(&src[at(pos, 6 * stride)..], out);
    out = vld4_lane_u8::<7>(&src[at(pos, 7 * stride)..], out);
    out
}

// upstream: src/dsp/dec_neon.c Load4x16_NEON
/// Loads the columns `pos - 2` to `pos + 1` of 16 rows: `(p1, p0, q0, q1)`.
#[target_feature(enable = "neon")]
fn load4x16_neon(src: &[u8], pos: usize, stride: i32) -> Vectors4 {
    // row0 = p1[0..7]|p0[0..7]|q0[0..7]|q1[0..7]
    // row8 = p1[8..15]|p0[8..15]|q0[8..15]|q1[8..15]
    let row0 = load4x8_neon(src, at(pos, -2), stride);
    let row8 = load4x8_neon(src, at(pos, -2 + 8 * stride), stride);
    (
        vcombine_u8(row0.0, row8.0),
        vcombine_u8(row0.1, row8.1),
        vcombine_u8(row0.2, row8.2),
        vcombine_u8(row0.3, row8.3),
    )
}

// upstream: src/dsp/dec_neon.c Load8x16_NEON
/// Loads the columns `pos - 4` to `pos + 3` of 16 rows: `(p3, p2, p1, p0,
/// q0, q1, q2, q3)`.
#[target_feature(enable = "neon")]
fn load8x16_neon(src: &[u8], pos: usize, stride: i32) -> Vectors8 {
    let (p3, p2, p1, p0) = load4x16_neon(src, at(pos, -2), stride);
    let (q0, q1, q2, q3) = load4x16_neon(src, pos + 2, stride);
    (p3, p2, p1, p0, q0, q1, q2, q3)
}

// upstream: src/dsp/dec_neon.c Load16x4_NEON
/// Loads the 16 bytes of the rows `pos - 2 * stride` to `pos + stride`:
/// `(p1, p0, q0, q1)`.
#[target_feature(enable = "neon")]
fn load16x4_neon(src: &[u8], pos: usize, stride: i32) -> Vectors4 {
    (
        vld1q_u8(&src[at(pos, -2 * stride)..]),
        vld1q_u8(&src[at(pos, -stride)..]),
        vld1q_u8(&src[pos..]),
        vld1q_u8(&src[at(pos, stride)..]),
    )
}

// upstream: src/dsp/dec_neon.c Load16x8_NEON
/// Loads the 16 bytes of the rows `pos - 4 * stride` to `pos + 3 * stride`.
#[target_feature(enable = "neon")]
fn load16x8_neon(src: &[u8], pos: usize, stride: i32) -> Vectors8 {
    let (p3, p2, p1, p0) = load16x4_neon(src, at(pos, -2 * stride), stride);
    let (q0, q1, q2, q3) = load16x4_neon(src, at(pos, 2 * stride), stride);
    (p3, p2, p1, p0, q0, q1, q2, q3)
}

// upstream: src/dsp/dec_neon.c Load8x8x2_NEON
/// Loads 8 bytes of the rows `-4 * stride` to `3 * stride` of `u` and `v`,
/// the U samples in the low and the V samples in the high halves.
#[target_feature(enable = "neon")]
fn load8x8x2_neon(u: &[u8], u_pos: usize, v: &[u8], v_pos: usize, stride: i32) -> Vectors8 {
    // We pack the 8x8 u-samples in the lower half of the uint8x16_t destination
    // and the v-samples on the higher half.
    (
        vcombine_u8(
            vld1_u8(&u[at(u_pos, -4 * stride)..]),
            vld1_u8(&v[at(v_pos, -4 * stride)..]),
        ),
        vcombine_u8(
            vld1_u8(&u[at(u_pos, -3 * stride)..]),
            vld1_u8(&v[at(v_pos, -3 * stride)..]),
        ),
        vcombine_u8(
            vld1_u8(&u[at(u_pos, -2 * stride)..]),
            vld1_u8(&v[at(v_pos, -2 * stride)..]),
        ),
        vcombine_u8(
            vld1_u8(&u[at(u_pos, -stride)..]),
            vld1_u8(&v[at(v_pos, -stride)..]),
        ),
        vcombine_u8(vld1_u8(&u[u_pos..]), vld1_u8(&v[v_pos..])),
        vcombine_u8(
            vld1_u8(&u[at(u_pos, stride)..]),
            vld1_u8(&v[at(v_pos, stride)..]),
        ),
        vcombine_u8(
            vld1_u8(&u[at(u_pos, 2 * stride)..]),
            vld1_u8(&v[at(v_pos, 2 * stride)..]),
        ),
        vcombine_u8(
            vld1_u8(&u[at(u_pos, 3 * stride)..]),
            vld1_u8(&v[at(v_pos, 3 * stride)..]),
        ),
    )
}

// upstream: src/dsp/dec_neon.c LOAD_UV_8
/// The 8 bytes from `u - 4` and from `v - 4` of row `row`.
#[target_feature(enable = "neon")]
fn load_uv_8(u: &[u8], u_pos: usize, v: &[u8], v_pos: usize, stride: i32, row: i32) -> uint8x16_t {
    vcombine_u8(
        vld1_u8(&u[at(u_pos, -4 + row * stride)..]),
        vld1_u8(&v[at(v_pos, -4 + row * stride)..]),
    )
}

// upstream: src/dsp/dec_neon.c Load8x8x2T_NEON
/// Loads the columns `-4` to `3` of 8 rows of `u` and `v`, transposed.
#[target_feature(enable = "neon")]
#[expect(
    clippy::similar_names,
    reason = "the C names row01, row02, row04 and the like"
)]
fn load8x8x2t_neon(u: &[u8], u_pos: usize, v: &[u8], v_pos: usize, stride: i32) -> Vectors8 {
    // We pack the 8x8 u-samples in the lower half of the uint8x16_t destination
    // and the v-samples on the higher half.
    let row0 = load_uv_8(u, u_pos, v, v_pos, stride, 0);
    let row1 = load_uv_8(u, u_pos, v, v_pos, stride, 1);
    let row2 = load_uv_8(u, u_pos, v, v_pos, stride, 2);
    let row3 = load_uv_8(u, u_pos, v, v_pos, stride, 3);
    let row4 = load_uv_8(u, u_pos, v, v_pos, stride, 4);
    let row5 = load_uv_8(u, u_pos, v, v_pos, stride, 5);
    let row6 = load_uv_8(u, u_pos, v, v_pos, stride, 6);
    let row7 = load_uv_8(u, u_pos, v, v_pos, stride, 7);
    // Perform two side-by-side 8x8 transposes
    // u00 u01 u02 u03 u04 u05 u06 u07 | v00 v01 v02 v03 v04 v05 v06 v07
    // u10 u11 u12 u13 u14 u15 u16 u17 | v10 v11 v12 ...
    // u20 u21 u22 u23 u24 u25 u26 u27 | v20 v21 ...
    // u30 u31 u32 u33 u34 u35 u36 u37 | ...
    // u40 u41 u42 u43 u44 u45 u46 u47 | ...
    // u50 u51 u52 u53 u54 u55 u56 u57 | ...
    // u60 u61 u62 u63 u64 u65 u66 u67 | v60 ...
    // u70 u71 u72 u73 u74 u75 u76 u77 | v70 v71 v72 ...
    let row01 = vtrnq_u8(row0, row1); // u00 u10 u02 u12 ...
    //                                   u01 u11 u03 u13 ...
    let row23 = vtrnq_u8(row2, row3); // u20 u30 u22 u32 ...
    //                                   u21 u31 u23 u33 ...
    let row45 = vtrnq_u8(row4, row5); // ...
    let row67 = vtrnq_u8(row6, row7); // ...
    let row02 = vtrnq_u16(vreinterpretq_u16_u8(row01.0), vreinterpretq_u16_u8(row23.0));
    let row13 = vtrnq_u16(vreinterpretq_u16_u8(row01.1), vreinterpretq_u16_u8(row23.1));
    let row46 = vtrnq_u16(vreinterpretq_u16_u8(row45.0), vreinterpretq_u16_u8(row67.0));
    let row57 = vtrnq_u16(vreinterpretq_u16_u8(row45.1), vreinterpretq_u16_u8(row67.1));
    let row04 = vtrnq_u32(
        vreinterpretq_u32_u16(row02.0),
        vreinterpretq_u32_u16(row46.0),
    );
    let row26 = vtrnq_u32(
        vreinterpretq_u32_u16(row02.1),
        vreinterpretq_u32_u16(row46.1),
    );
    let row15 = vtrnq_u32(
        vreinterpretq_u32_u16(row13.0),
        vreinterpretq_u32_u16(row57.0),
    );
    let row37 = vtrnq_u32(
        vreinterpretq_u32_u16(row13.1),
        vreinterpretq_u32_u16(row57.1),
    );
    (
        vreinterpretq_u8_u32(row04.0),
        vreinterpretq_u8_u32(row15.0),
        vreinterpretq_u8_u32(row26.0),
        vreinterpretq_u8_u32(row37.0),
        vreinterpretq_u8_u32(row04.1),
        vreinterpretq_u8_u32(row15.1),
        vreinterpretq_u8_u32(row26.1),
        vreinterpretq_u8_u32(row37.1),
    )
}

// upstream: src/dsp/dec_neon.c Store2x8_NEON
/// Stores lane `k` of the two vectors to the 2 bytes at row `k`.
#[target_feature(enable = "neon")]
fn store2x8_neon(v: uint8x8x2_t, dst: &mut [u8], pos: usize, stride: i32) {
    vst2_lane_u8::<0>(&mut dst[pos..], v);
    vst2_lane_u8::<1>(&mut dst[at(pos, stride)..], v);
    vst2_lane_u8::<2>(&mut dst[at(pos, 2 * stride)..], v);
    vst2_lane_u8::<3>(&mut dst[at(pos, 3 * stride)..], v);
    vst2_lane_u8::<4>(&mut dst[at(pos, 4 * stride)..], v);
    vst2_lane_u8::<5>(&mut dst[at(pos, 5 * stride)..], v);
    vst2_lane_u8::<6>(&mut dst[at(pos, 6 * stride)..], v);
    vst2_lane_u8::<7>(&mut dst[at(pos, 7 * stride)..], v);
}

// upstream: src/dsp/dec_neon.c Store2x16_NEON
/// Stores `p0` and `q0` to the columns `pos - 1` and `pos` of 16 rows.
#[target_feature(enable = "neon")]
fn store2x16_neon(p0: uint8x16_t, q0: uint8x16_t, dst: &mut [u8], pos: usize, stride: i32) {
    let lo = uint8x8x2_t(vget_low_u8(p0), vget_low_u8(q0));
    let hi = uint8x8x2_t(vget_high_u8(p0), vget_high_u8(q0));
    store2x8_neon(lo, dst, at(pos, -1), stride);
    store2x8_neon(hi, dst, at(pos, -1 + 8 * stride), stride);
}

// upstream: src/dsp/dec_neon.c Store4x8_NEON
/// Stores lane `k` of the four vectors to the 4 bytes at row `k`.
#[target_feature(enable = "neon")]
fn store4x8_neon(v: uint8x8x4_t, dst: &mut [u8], pos: usize, stride: i32) {
    vst4_lane_u8::<0>(&mut dst[pos..], v);
    vst4_lane_u8::<1>(&mut dst[at(pos, stride)..], v);
    vst4_lane_u8::<2>(&mut dst[at(pos, 2 * stride)..], v);
    vst4_lane_u8::<3>(&mut dst[at(pos, 3 * stride)..], v);
    vst4_lane_u8::<4>(&mut dst[at(pos, 4 * stride)..], v);
    vst4_lane_u8::<5>(&mut dst[at(pos, 5 * stride)..], v);
    vst4_lane_u8::<6>(&mut dst[at(pos, 6 * stride)..], v);
    vst4_lane_u8::<7>(&mut dst[at(pos, 7 * stride)..], v);
}

// upstream: src/dsp/dec_neon.c Store4x16_NEON
/// Stores `p1`, `p0`, `q0` and `q1` to the columns `pos - 2` to `pos + 1` of
/// 16 rows.
#[target_feature(enable = "neon")]
fn store4x16_neon(
    p1: uint8x16_t,
    p0: uint8x16_t,
    q0: uint8x16_t,
    q1: uint8x16_t,
    dst: &mut [u8],
    pos: usize,
    stride: i32,
) {
    let lo = uint8x8x4_t(
        vget_low_u8(p1),
        vget_low_u8(p0),
        vget_low_u8(q0),
        vget_low_u8(q1),
    );
    let hi = uint8x8x4_t(
        vget_high_u8(p1),
        vget_high_u8(p0),
        vget_high_u8(q0),
        vget_high_u8(q1),
    );
    store4x8_neon(lo, dst, at(pos, -2), stride);
    store4x8_neon(hi, dst, at(pos, -2 + 8 * stride), stride);
}

// upstream: src/dsp/dec_neon.c Store16x2_NEON
/// Stores `p0` to the row above `pos` and `q0` to the row at `pos`.
#[target_feature(enable = "neon")]
fn store16x2_neon(p0: uint8x16_t, q0: uint8x16_t, dst: &mut [u8], pos: usize, stride: i32) {
    vst1q_u8(&mut dst[at(pos, -stride)..], p0);
    vst1q_u8(&mut dst[pos..], q0);
}

// upstream: src/dsp/dec_neon.c Store16x4_NEON
/// Stores the four vectors to the rows `pos - 2 * stride` to `pos + stride`.
#[target_feature(enable = "neon")]
fn store16x4_neon(
    p1: uint8x16_t,
    p0: uint8x16_t,
    q0: uint8x16_t,
    q1: uint8x16_t,
    dst: &mut [u8],
    pos: usize,
    stride: i32,
) {
    store16x2_neon(p1, p0, dst, at(pos, -stride), stride);
    store16x2_neon(q0, q1, dst, at(pos, stride), stride);
}

// upstream: src/dsp/dec_neon.c Store8x2x2_NEON
/// Stores the U halves of `p0` and `q0` to the rows above and at `u_pos`, and
/// the V halves likewise at `v_pos`.
#[target_feature(enable = "neon")]
fn store8x2x2_neon(
    p0: uint8x16_t,
    q0: uint8x16_t,
    u: &mut [u8],
    u_pos: usize,
    v: &mut [u8],
    v_pos: usize,
    stride: i32,
) {
    // p0 and q0 contain the u+v samples packed in low/high halves.
    vst1_u8(&mut u[at(u_pos, -stride)..], vget_low_u8(p0));
    vst1_u8(&mut u[u_pos..], vget_low_u8(q0));
    vst1_u8(&mut v[at(v_pos, -stride)..], vget_high_u8(p0));
    vst1_u8(&mut v[v_pos..], vget_high_u8(q0));
}

// upstream: src/dsp/dec_neon.c Store8x4x2_NEON
/// Stores the four vectors to the rows `-2 * stride` to `stride` of `u` and
/// `v`.
#[target_feature(enable = "neon")]
#[expect(
    clippy::too_many_arguments,
    reason = "the C parameters with two positions"
)]
fn store8x4x2_neon(
    p1: uint8x16_t,
    p0: uint8x16_t,
    q0: uint8x16_t,
    q1: uint8x16_t,
    u: &mut [u8],
    u_pos: usize,
    v: &mut [u8],
    v_pos: usize,
    stride: i32,
) {
    // The p1...q1 registers contain the u+v samples packed in low/high halves.
    store8x2x2_neon(p1, p0, u, at(u_pos, -stride), v, at(v_pos, -stride), stride);
    store8x2x2_neon(q0, q1, u, at(u_pos, stride), v, at(v_pos, stride), stride);
}

// upstream: src/dsp/dec_neon.c STORE6_LANE
/// Stores lane `LANE` of `val0` to the 3 bytes before `pos` and of `val1` to
/// the 3 bytes from `pos`; returns the position one row down.
#[target_feature(enable = "neon")]
fn store6_lane<const LANE: i32>(
    dst: &mut [u8],
    pos: usize,
    val0: uint8x8x3_t,
    val1: uint8x8x3_t,
    stride: i32,
) -> usize {
    vst3_lane_u8::<LANE>(&mut dst[at(pos, -3)..], val0);
    vst3_lane_u8::<LANE>(&mut dst[pos..], val1);
    at(pos, stride)
}

// upstream: src/dsp/dec_neon.c Store6x8x2_NEON
/// Stores the six vectors to the columns `-3` to `2` of 8 rows of `u` and `v`.
#[target_feature(enable = "neon")]
#[expect(
    clippy::too_many_arguments,
    reason = "the C parameters with two positions"
)]
fn store6x8x2_neon(
    p2: uint8x16_t,
    p1: uint8x16_t,
    p0: uint8x16_t,
    q0: uint8x16_t,
    q1: uint8x16_t,
    q2: uint8x16_t,
    u: &mut [u8],
    u_pos: usize,
    v: &mut [u8],
    v_pos: usize,
    stride: i32,
) {
    let u0 = uint8x8x3_t(vget_low_u8(p2), vget_low_u8(p1), vget_low_u8(p0));
    let u1 = uint8x8x3_t(vget_low_u8(q0), vget_low_u8(q1), vget_low_u8(q2));
    let v0 = uint8x8x3_t(vget_high_u8(p2), vget_high_u8(p1), vget_high_u8(p0));
    let v1 = uint8x8x3_t(vget_high_u8(q0), vget_high_u8(q1), vget_high_u8(q2));
    let mut u_pos = u_pos;
    u_pos = store6_lane::<0>(u, u_pos, u0, u1, stride);
    u_pos = store6_lane::<1>(u, u_pos, u0, u1, stride);
    u_pos = store6_lane::<2>(u, u_pos, u0, u1, stride);
    u_pos = store6_lane::<3>(u, u_pos, u0, u1, stride);
    u_pos = store6_lane::<4>(u, u_pos, u0, u1, stride);
    u_pos = store6_lane::<5>(u, u_pos, u0, u1, stride);
    u_pos = store6_lane::<6>(u, u_pos, u0, u1, stride);
    store6_lane::<7>(u, u_pos, u0, u1, stride);
    let mut v_pos = v_pos;
    v_pos = store6_lane::<0>(v, v_pos, v0, v1, stride);
    v_pos = store6_lane::<1>(v, v_pos, v0, v1, stride);
    v_pos = store6_lane::<2>(v, v_pos, v0, v1, stride);
    v_pos = store6_lane::<3>(v, v_pos, v0, v1, stride);
    v_pos = store6_lane::<4>(v, v_pos, v0, v1, stride);
    v_pos = store6_lane::<5>(v, v_pos, v0, v1, stride);
    v_pos = store6_lane::<6>(v, v_pos, v0, v1, stride);
    store6_lane::<7>(v, v_pos, v0, v1, stride);
}

// upstream: src/dsp/dec_neon.c Store4x8x2_NEON
/// Stores the four vectors to the columns `-2` to `1` of 8 rows of `u` and
/// `v`.
#[target_feature(enable = "neon")]
#[expect(
    clippy::too_many_arguments,
    reason = "the C parameters with two positions"
)]
fn store4x8x2_neon(
    p1: uint8x16_t,
    p0: uint8x16_t,
    q0: uint8x16_t,
    q1: uint8x16_t,
    u: &mut [u8],
    u_pos: usize,
    v: &mut [u8],
    v_pos: usize,
    stride: i32,
) {
    let u0 = uint8x8x4_t(
        vget_low_u8(p1),
        vget_low_u8(p0),
        vget_low_u8(q0),
        vget_low_u8(q1),
    );
    let v0 = uint8x8x4_t(
        vget_high_u8(p1),
        vget_high_u8(p0),
        vget_high_u8(q0),
        vget_high_u8(q1),
    );
    vst4_lane_u8::<0>(&mut u[at(u_pos, -2)..], u0);
    vst4_lane_u8::<1>(&mut u[at(u_pos, -2 + stride)..], u0);
    vst4_lane_u8::<2>(&mut u[at(u_pos, -2 + 2 * stride)..], u0);
    vst4_lane_u8::<3>(&mut u[at(u_pos, -2 + 3 * stride)..], u0);
    vst4_lane_u8::<4>(&mut u[at(u_pos, -2 + 4 * stride)..], u0);
    vst4_lane_u8::<5>(&mut u[at(u_pos, -2 + 5 * stride)..], u0);
    vst4_lane_u8::<6>(&mut u[at(u_pos, -2 + 6 * stride)..], u0);
    vst4_lane_u8::<7>(&mut u[at(u_pos, -2 + 7 * stride)..], u0);
    vst4_lane_u8::<0>(&mut v[at(v_pos, -2)..], v0);
    vst4_lane_u8::<1>(&mut v[at(v_pos, -2 + stride)..], v0);
    vst4_lane_u8::<2>(&mut v[at(v_pos, -2 + 2 * stride)..], v0);
    vst4_lane_u8::<3>(&mut v[at(v_pos, -2 + 3 * stride)..], v0);
    vst4_lane_u8::<4>(&mut v[at(v_pos, -2 + 4 * stride)..], v0);
    vst4_lane_u8::<5>(&mut v[at(v_pos, -2 + 5 * stride)..], v0);
    vst4_lane_u8::<6>(&mut v[at(v_pos, -2 + 6 * stride)..], v0);
    vst4_lane_u8::<7>(&mut v[at(v_pos, -2 + 7 * stride)..], v0);
}

// upstream: src/dsp/dec_neon.c ConvertU8ToS16_NEON
/// Zero extend `v` to an `int16x8_t`.
#[target_feature(enable = "neon")]
fn convert_u8_to_s16_neon(v: uint8x8_t) -> int16x8_t {
    vreinterpretq_s16_u16(vmovl_u8(v))
}

// upstream: src/dsp/dec_neon.c SaturateAndStore4x4_NEON
/// Performs unsigned 8b saturation on `dst01` and `dst23` storing the result
/// to the corresponding rows of `dst`.
#[target_feature(enable = "neon")]
fn saturate_and_store4x4_neon(dst: &mut [u8], dst01: int16x8_t, dst23: int16x8_t) {
    // Unsigned saturate to 8b.
    let dst01_u8 = vqmovun_s16(dst01);
    let dst23_u8 = vqmovun_s16(dst23);

    // Store the results.
    store_u32_ne(dst, vget_lane_u32::<0>(vreinterpret_u32_u8(dst01_u8)));
    store_u32_ne(
        &mut dst[BPS_U..],
        vget_lane_u32::<1>(vreinterpret_u32_u8(dst01_u8)),
    );
    store_u32_ne(
        &mut dst[2 * BPS_U..],
        vget_lane_u32::<0>(vreinterpret_u32_u8(dst23_u8)),
    );
    store_u32_ne(
        &mut dst[3 * BPS_U..],
        vget_lane_u32::<1>(vreinterpret_u32_u8(dst23_u8)),
    );
}

// upstream: src/dsp/dec_neon.c Add4x4_NEON
/// Adds the descaled rows `row01` and `row23` to the 4x4 block of `dst`.
#[target_feature(enable = "neon")]
fn add4x4_neon(row01: int16x8_t, row23: int16x8_t, dst: &mut [u8]) {
    let mut dst01 = vdup_n_u32(0);
    let mut dst23 = vdup_n_u32(0);

    // Load the source pixels.
    dst01 = vset_lane_u32::<0>(load_u32_ne(dst), dst01);
    dst23 = vset_lane_u32::<0>(load_u32_ne(&dst[2 * BPS_U..]), dst23);
    dst01 = vset_lane_u32::<1>(load_u32_ne(&dst[BPS_U..]), dst01);
    dst23 = vset_lane_u32::<1>(load_u32_ne(&dst[3 * BPS_U..]), dst23);

    {
        // Convert to 16b.
        let dst01_s16 = convert_u8_to_s16_neon(vreinterpret_u8_u32(dst01));
        let dst23_s16 = convert_u8_to_s16_neon(vreinterpret_u8_u32(dst23));

        // Descale with rounding.
        let out01 = vrsraq_n_s16::<3>(dst01_s16, row01);
        let out23 = vrsraq_n_s16::<3>(dst23_s16, row23);
        // Add the inverse transform.
        saturate_and_store4x4_neon(dst, out01, out23);
    }
}

//-----------------------------------------------------------------------------
// Simple In-loop filtering (Paragraph 15.2)

/// `(uint8_t)value`, as C converts the thresholds.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "C converts the int threshold to uint8_t"
)]
fn to_u8(value: i32) -> u8 {
    value as u8
}

// upstream: src/dsp/dec_neon.c NeedsFilter_NEON
#[target_feature(enable = "neon")]
fn needs_filter_neon(
    p1: uint8x16_t,
    p0: uint8x16_t,
    q0: uint8x16_t,
    q1: uint8x16_t,
    thresh: i32,
) -> uint8x16_t {
    let thresh_v = vdupq_n_u8(to_u8(thresh));
    let a_p0_q0 = vabdq_u8(p0, q0); // abs(p0-q0)
    let a_p1_q1 = vabdq_u8(p1, q1); // abs(p1-q1)
    let a_p0_q0_2 = vqaddq_u8(a_p0_q0, a_p0_q0); // 2 * abs(p0-q0)
    let a_p1_q1_2 = vshrq_n_u8::<1>(a_p1_q1); // abs(p1-q1) / 2
    let sum = vqaddq_u8(a_p0_q0_2, a_p1_q1_2);
    vcgeq_u8(thresh_v, sum)
}

// upstream: src/dsp/dec_neon.c FlipSign_NEON
#[target_feature(enable = "neon")]
fn flip_sign_neon(v: uint8x16_t) -> int8x16_t {
    let sign_bit = vdupq_n_u8(0x80);
    vreinterpretq_s8_u8(veorq_u8(v, sign_bit))
}

// upstream: src/dsp/dec_neon.c FlipSignBack_NEON
#[target_feature(enable = "neon")]
fn flip_sign_back_neon(v: int8x16_t) -> uint8x16_t {
    // C: vdupq_n_s8(0x80), the int 0x80 converted to int8_t, -128.
    let sign_bit = vdupq_n_s8(i8::MIN);
    vreinterpretq_u8_s8(veorq_s8(v, sign_bit))
}

// upstream: src/dsp/dec_neon.c GetBaseDelta_NEON
#[target_feature(enable = "neon")]
fn get_base_delta_neon(p1: int8x16_t, p0: int8x16_t, q0: int8x16_t, q1: int8x16_t) -> int8x16_t {
    let q0_p0 = vqsubq_s8(q0, p0); // (q0-p0)
    let p1_q1 = vqsubq_s8(p1, q1); // (p1-q1)
    let s1 = vqaddq_s8(p1_q1, q0_p0); // (p1-q1) + 1 * (q0 - p0)
    let s2 = vqaddq_s8(q0_p0, s1); // (p1-q1) + 2 * (q0 - p0)
    vqaddq_s8(q0_p0, s2) // (p1-q1) + 3 * (q0 - p0)
}

// upstream: src/dsp/dec_neon.c GetBaseDelta0_NEON
#[target_feature(enable = "neon")]
fn get_base_delta0_neon(p0: int8x16_t, q0: int8x16_t) -> int8x16_t {
    let q0_p0 = vqsubq_s8(q0, p0); // (q0-p0)
    let s1 = vqaddq_s8(q0_p0, q0_p0); // 2 * (q0 - p0)
    vqaddq_s8(q0_p0, s1) // 3 * (q0 - p0)
}

//------------------------------------------------------------------------------

// upstream: src/dsp/dec_neon.c ApplyFilter2NoFlip_NEON
/// Returns `(op0, oq0)` in the signed domain.
#[target_feature(enable = "neon")]
fn apply_filter2_no_flip_neon(
    p0s: int8x16_t,
    q0s: int8x16_t,
    delta: int8x16_t,
) -> (int8x16_t, int8x16_t) {
    let k_cst3 = vdupq_n_s8(0x03);
    let k_cst4 = vdupq_n_s8(0x04);
    let delta_p3 = vqaddq_s8(delta, k_cst3);
    let delta_p4 = vqaddq_s8(delta, k_cst4);
    let delta3 = vshrq_n_s8::<3>(delta_p3);
    let delta4 = vshrq_n_s8::<3>(delta_p4);
    (vqaddq_s8(p0s, delta3), vqsubq_s8(q0s, delta4))
}

// upstream: src/dsp/dec_neon.c ApplyFilter2_NEON
/// Returns `(op0, oq0)`.
#[target_feature(enable = "neon")]
fn apply_filter2_neon(
    p0s: int8x16_t,
    q0s: int8x16_t,
    delta: int8x16_t,
) -> (uint8x16_t, uint8x16_t) {
    let k_cst3 = vdupq_n_s8(0x03);
    let k_cst4 = vdupq_n_s8(0x04);
    let delta_p3 = vqaddq_s8(delta, k_cst3);
    let delta_p4 = vqaddq_s8(delta, k_cst4);
    let delta3 = vshrq_n_s8::<3>(delta_p3);
    let delta4 = vshrq_n_s8::<3>(delta_p4);
    let sp0 = vqaddq_s8(p0s, delta3);
    let sq0 = vqsubq_s8(q0s, delta4);
    (flip_sign_back_neon(sp0), flip_sign_back_neon(sq0))
}

// upstream: src/dsp/dec_neon.c DoFilter2_NEON
/// Returns `(op0, oq0)`.
#[target_feature(enable = "neon")]
fn do_filter2_neon(
    p1: uint8x16_t,
    p0: uint8x16_t,
    q0: uint8x16_t,
    q1: uint8x16_t,
    mask: uint8x16_t,
) -> (uint8x16_t, uint8x16_t) {
    let p1s = flip_sign_neon(p1);
    let p0s = flip_sign_neon(p0);
    let q0s = flip_sign_neon(q0);
    let q1s = flip_sign_neon(q1);
    let delta0 = get_base_delta_neon(p1s, p0s, q0s, q1s);
    let delta1 = vandq_s8(delta0, vreinterpretq_s8_u8(mask));
    apply_filter2_neon(p0s, q0s, delta1)
}

// upstream: src/dsp/dec_neon.c SimpleVFilter16_NEON
#[target_feature(enable = "neon")]
fn simple_v_filter16_neon(p: &mut [u8], pos: usize, stride: i32, thresh: i32) {
    let (p1, p0, q0, q1) = load16x4_neon(p, pos, stride);
    let (op0, oq0) = {
        let mask = needs_filter_neon(p1, p0, q0, q1, thresh);
        do_filter2_neon(p1, p0, q0, q1, mask)
    };
    store16x2_neon(op0, oq0, p, pos, stride);
}

/// Entry point of `SimpleVFilter16_NEON` for `VP8SimpleVFilter16`.
pub fn simple_v_filter16(p: &mut [u8], pos: usize, stride: i32, thresh: i32) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { simple_v_filter16_neon(p, pos, stride, thresh) }
}

// upstream: src/dsp/dec_neon.c SimpleHFilter16_NEON
#[target_feature(enable = "neon")]
fn simple_h_filter16_neon(p: &mut [u8], pos: usize, stride: i32, thresh: i32) {
    let (p1, p0, q0, q1) = load4x16_neon(p, pos, stride);
    let (op0, oq0) = {
        let mask = needs_filter_neon(p1, p0, q0, q1, thresh);
        do_filter2_neon(p1, p0, q0, q1, mask)
    };
    store2x16_neon(op0, oq0, p, pos, stride);
}

/// Entry point of `SimpleHFilter16_NEON` for `VP8SimpleHFilter16`.
pub fn simple_h_filter16(p: &mut [u8], pos: usize, stride: i32, thresh: i32) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { simple_h_filter16_neon(p, pos, stride, thresh) }
}

// upstream: src/dsp/dec_neon.c SimpleVFilter16i_NEON
#[target_feature(enable = "neon")]
fn simple_v_filter16i_neon(p: &mut [u8], pos: usize, stride: i32, thresh: i32) {
    let mut pos = pos;
    for _ in 0..3 {
        pos = at(pos, 4 * stride);
        simple_v_filter16_neon(p, pos, stride, thresh);
    }
}

/// Entry point of `SimpleVFilter16i_NEON` for `VP8SimpleVFilter16i`.
pub fn simple_v_filter16i(p: &mut [u8], pos: usize, stride: i32, thresh: i32) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { simple_v_filter16i_neon(p, pos, stride, thresh) }
}

// upstream: src/dsp/dec_neon.c SimpleHFilter16i_NEON
#[target_feature(enable = "neon")]
fn simple_h_filter16i_neon(p: &mut [u8], pos: usize, stride: i32, thresh: i32) {
    let mut pos = pos;
    for _ in 0..3 {
        pos += 4;
        simple_h_filter16_neon(p, pos, stride, thresh);
    }
}

/// Entry point of `SimpleHFilter16i_NEON` for `VP8SimpleHFilter16i`.
pub fn simple_h_filter16i(p: &mut [u8], pos: usize, stride: i32, thresh: i32) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { simple_h_filter16i_neon(p, pos, stride, thresh) }
}

//------------------------------------------------------------------------------
// Complex In-loop filtering (Paragraph 15.3)

// upstream: src/dsp/dec_neon.c NeedsHev_NEON
#[target_feature(enable = "neon")]
fn needs_hev_neon(
    p1: uint8x16_t,
    p0: uint8x16_t,
    q0: uint8x16_t,
    q1: uint8x16_t,
    hev_thresh: i32,
) -> uint8x16_t {
    let hev_thresh_v = vdupq_n_u8(to_u8(hev_thresh));
    let a_p1_p0 = vabdq_u8(p1, p0); // abs(p1 - p0)
    let a_q1_q0 = vabdq_u8(q1, q0); // abs(q1 - q0)
    let a_max = vmaxq_u8(a_p1_p0, a_q1_q0);
    vcgtq_u8(a_max, hev_thresh_v)
}

// upstream: src/dsp/dec_neon.c NeedsFilter2_NEON
#[target_feature(enable = "neon")]
#[expect(clippy::too_many_arguments, reason = "the C parameters")]
#[expect(
    clippy::similar_names,
    reason = "the C names a_p3_p2, a_p2_p1, max12 and the like"
)]
fn needs_filter2_neon(
    p3: uint8x16_t,
    p2: uint8x16_t,
    p1: uint8x16_t,
    p0: uint8x16_t,
    q0: uint8x16_t,
    q1: uint8x16_t,
    q2: uint8x16_t,
    q3: uint8x16_t,
    ithresh: i32,
    thresh: i32,
) -> uint8x16_t {
    let ithresh_v = vdupq_n_u8(to_u8(ithresh));
    let a_p3_p2 = vabdq_u8(p3, p2); // abs(p3 - p2)
    let a_p2_p1 = vabdq_u8(p2, p1); // abs(p2 - p1)
    let a_p1_p0 = vabdq_u8(p1, p0); // abs(p1 - p0)
    let a_q3_q2 = vabdq_u8(q3, q2); // abs(q3 - q2)
    let a_q2_q1 = vabdq_u8(q2, q1); // abs(q2 - q1)
    let a_q1_q0 = vabdq_u8(q1, q0); // abs(q1 - q0)
    let max1 = vmaxq_u8(a_p3_p2, a_p2_p1);
    let max2 = vmaxq_u8(a_p1_p0, a_q3_q2);
    let max3 = vmaxq_u8(a_q2_q1, a_q1_q0);
    let max12 = vmaxq_u8(max1, max2);
    let max123 = vmaxq_u8(max12, max3);
    let mask2 = vcgeq_u8(ithresh_v, max123);
    let mask1 = needs_filter_neon(p1, p0, q0, q1, thresh);
    vandq_u8(mask1, mask2)
}

//  4-points filter

// upstream: src/dsp/dec_neon.c ApplyFilter4_NEON
/// Returns `(op1, op0, oq0, oq1)`.
#[target_feature(enable = "neon")]
fn apply_filter4_neon(
    p1: int8x16_t,
    p0: int8x16_t,
    q0: int8x16_t,
    q1: int8x16_t,
    delta0: int8x16_t,
) -> Vectors4 {
    let k_cst3 = vdupq_n_s8(0x03);
    let k_cst4 = vdupq_n_s8(0x04);
    let delta1 = vqaddq_s8(delta0, k_cst4);
    let delta2 = vqaddq_s8(delta0, k_cst3);
    let a1 = vshrq_n_s8::<3>(delta1);
    let a2 = vshrq_n_s8::<3>(delta2);
    let a3 = vrshrq_n_s8::<1>(a1); // a3 = (a1 + 1) >> 1
    let op0 = flip_sign_back_neon(vqaddq_s8(p0, a2)); // clip(p0 + a2)
    let oq0 = flip_sign_back_neon(vqsubq_s8(q0, a1)); // clip(q0 - a1)
    let op1 = flip_sign_back_neon(vqaddq_s8(p1, a3)); // clip(p1 + a3)
    let oq1 = flip_sign_back_neon(vqsubq_s8(q1, a3)); // clip(q1 - a3)
    (op1, op0, oq0, oq1)
}

// upstream: src/dsp/dec_neon.c DoFilter4_NEON
/// Returns `(op1, op0, oq0, oq1)`.
#[target_feature(enable = "neon")]
fn do_filter4_neon(
    p1: uint8x16_t,
    p0: uint8x16_t,
    q0: uint8x16_t,
    q1: uint8x16_t,
    mask: uint8x16_t,
    hev_mask: uint8x16_t,
) -> Vectors4 {
    // This is a fused version of DoFilter2() calling ApplyFilter2 directly
    let p1s = flip_sign_neon(p1);
    let mut p0s = flip_sign_neon(p0);
    let mut q0s = flip_sign_neon(q0);
    let q1s = flip_sign_neon(q1);
    let simple_lf_mask = vandq_u8(mask, hev_mask);

    // do_filter2 part (simple loopfilter on pixels with hev)
    {
        let delta = get_base_delta_neon(p1s, p0s, q0s, q1s);
        let simple_lf_delta = vandq_s8(delta, vreinterpretq_s8_u8(simple_lf_mask));
        (p0s, q0s) = apply_filter2_no_flip_neon(p0s, q0s, simple_lf_delta);
    }

    // do_filter4 part (complex loopfilter on pixels without hev)
    {
        let delta0 = get_base_delta0_neon(p0s, q0s);
        // we use: (mask & hev_mask) ^ mask = mask & !hev_mask
        let complex_lf_mask = veorq_u8(simple_lf_mask, mask);
        let complex_lf_delta = vandq_s8(delta0, vreinterpretq_s8_u8(complex_lf_mask));
        apply_filter4_neon(p1s, p0s, q0s, q1s, complex_lf_delta)
    }
}

//  6-points filter

// upstream: src/dsp/dec_neon.c ApplyFilter6_NEON
/// Returns `(op2, op1, op0, oq0, oq1, oq2)`.
#[target_feature(enable = "neon")]
#[expect(
    clippy::similar_names,
    reason = "the C names a1_lo, a2_lo and the like"
)]
fn apply_filter6_neon(
    p2: int8x16_t,
    p1: int8x16_t,
    p0: int8x16_t,
    q0: int8x16_t,
    q1: int8x16_t,
    q2: int8x16_t,
    delta: int8x16_t,
) -> Vectors6 {
    // We have to compute: X = (9*a+63) >> 7, Y = (18*a+63)>>7, Z = (27*a+63) >> 7
    // Turns out, there's a common sub-expression S=9 * a - 1 that can be used
    // with the special vqrshrn_n_s16 rounding-shift-and-narrow instruction:
    //   X = (S + 64) >> 7, Y = (S + 32) >> 6, Z = (18 * a + S + 64) >> 7
    let delta_lo = vget_low_s8(delta);
    let delta_hi = vget_high_s8(delta);
    let k_cst9 = vdup_n_s8(9);
    let k_cstm1 = vdupq_n_s16(-1);
    let k_cst18 = vdup_n_s8(18);
    let s_lo = vmlal_s8(k_cstm1, k_cst9, delta_lo); // S = 9 * a - 1
    let s_hi = vmlal_s8(k_cstm1, k_cst9, delta_hi);
    let z_lo = vmlal_s8(s_lo, k_cst18, delta_lo); // S + 18 * a
    let z_hi = vmlal_s8(s_hi, k_cst18, delta_hi);
    let a3_lo = vqrshrn_n_s16::<7>(s_lo); // (9 * a + 63) >> 7
    let a3_hi = vqrshrn_n_s16::<7>(s_hi);
    let a2_lo = vqrshrn_n_s16::<6>(s_lo); // (9 * a + 31) >> 6
    let a2_hi = vqrshrn_n_s16::<6>(s_hi);
    let a1_lo = vqrshrn_n_s16::<7>(z_lo); // (27 * a + 63) >> 7
    let a1_hi = vqrshrn_n_s16::<7>(z_hi);
    let a1 = vcombine_s8(a1_lo, a1_hi);
    let a2 = vcombine_s8(a2_lo, a2_hi);
    let a3 = vcombine_s8(a3_lo, a3_hi);

    let op0 = flip_sign_back_neon(vqaddq_s8(p0, a1)); // clip(p0 + a1)
    let oq0 = flip_sign_back_neon(vqsubq_s8(q0, a1)); // clip(q0 - q1)
    let oq1 = flip_sign_back_neon(vqsubq_s8(q1, a2)); // clip(q1 - a2)
    let op1 = flip_sign_back_neon(vqaddq_s8(p1, a2)); // clip(p1 + a2)
    let oq2 = flip_sign_back_neon(vqsubq_s8(q2, a3)); // clip(q2 - a3)
    let op2 = flip_sign_back_neon(vqaddq_s8(p2, a3)); // clip(p2 + a3)
    (op2, op1, op0, oq0, oq1, oq2)
}

// upstream: src/dsp/dec_neon.c DoFilter6_NEON
/// Returns `(op2, op1, op0, oq0, oq1, oq2)`.
#[target_feature(enable = "neon")]
#[expect(clippy::too_many_arguments, reason = "the C parameters")]
fn do_filter6_neon(
    p2: uint8x16_t,
    p1: uint8x16_t,
    p0: uint8x16_t,
    q0: uint8x16_t,
    q1: uint8x16_t,
    q2: uint8x16_t,
    mask: uint8x16_t,
    hev_mask: uint8x16_t,
) -> Vectors6 {
    // This is a fused version of DoFilter2() calling ApplyFilter2 directly
    let p2s = flip_sign_neon(p2);
    let p1s = flip_sign_neon(p1);
    let mut p0s = flip_sign_neon(p0);
    let mut q0s = flip_sign_neon(q0);
    let q1s = flip_sign_neon(q1);
    let q2s = flip_sign_neon(q2);
    let simple_lf_mask = vandq_u8(mask, hev_mask);
    let delta0 = get_base_delta_neon(p1s, p0s, q0s, q1s);

    // do_filter2 part (simple loopfilter on pixels with hev)
    {
        let simple_lf_delta = vandq_s8(delta0, vreinterpretq_s8_u8(simple_lf_mask));
        (p0s, q0s) = apply_filter2_no_flip_neon(p0s, q0s, simple_lf_delta);
    }

    // do_filter6 part (complex loopfilter on pixels without hev)
    {
        // we use: (mask & hev_mask) ^ mask = mask & !hev_mask
        let complex_lf_mask = veorq_u8(simple_lf_mask, mask);
        let complex_lf_delta = vandq_s8(delta0, vreinterpretq_s8_u8(complex_lf_mask));
        apply_filter6_neon(p2s, p1s, p0s, q0s, q1s, q2s, complex_lf_delta)
    }
}

// on macroblock edges

// upstream: src/dsp/dec_neon.c VFilter16_NEON
#[target_feature(enable = "neon")]
fn v_filter16_neon(
    p: &mut [u8],
    pos: usize,
    stride: i32,
    thresh: i32,
    ithresh: i32,
    hev_thresh: i32,
) {
    let (p3, p2, p1, p0, q0, q1, q2, q3) = load16x8_neon(p, pos, stride);
    {
        let mask = needs_filter2_neon(p3, p2, p1, p0, q0, q1, q2, q3, ithresh, thresh);
        let hev_mask = needs_hev_neon(p1, p0, q0, q1, hev_thresh);
        let (op2, op1, op0, oq0, oq1, oq2) =
            do_filter6_neon(p2, p1, p0, q0, q1, q2, mask, hev_mask);
        store16x2_neon(op2, op1, p, at(pos, -2 * stride), stride);
        store16x2_neon(op0, oq0, p, pos, stride);
        store16x2_neon(oq1, oq2, p, at(pos, 2 * stride), stride);
    }
}

/// Entry point of `VFilter16_NEON` for `VP8VFilter16`.
pub fn v_filter16(
    p: &mut [u8],
    pos: usize,
    stride: i32,
    thresh: i32,
    ithresh: i32,
    hev_thresh: i32,
) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { v_filter16_neon(p, pos, stride, thresh, ithresh, hev_thresh) }
}

// upstream: src/dsp/dec_neon.c HFilter16_NEON
#[target_feature(enable = "neon")]
fn h_filter16_neon(
    p: &mut [u8],
    pos: usize,
    stride: i32,
    thresh: i32,
    ithresh: i32,
    hev_thresh: i32,
) {
    let (p3, p2, p1, p0, q0, q1, q2, q3) = load8x16_neon(p, pos, stride);
    {
        let mask = needs_filter2_neon(p3, p2, p1, p0, q0, q1, q2, q3, ithresh, thresh);
        let hev_mask = needs_hev_neon(p1, p0, q0, q1, hev_thresh);
        let (op2, op1, op0, oq0, oq1, oq2) =
            do_filter6_neon(p2, p1, p0, q0, q1, q2, mask, hev_mask);
        store2x16_neon(op2, op1, p, at(pos, -2), stride);
        store2x16_neon(op0, oq0, p, pos, stride);
        store2x16_neon(oq1, oq2, p, pos + 2, stride);
    }
}

/// Entry point of `HFilter16_NEON` for `VP8HFilter16`.
pub fn h_filter16(
    p: &mut [u8],
    pos: usize,
    stride: i32,
    thresh: i32,
    ithresh: i32,
    hev_thresh: i32,
) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { h_filter16_neon(p, pos, stride, thresh, ithresh, hev_thresh) }
}

// on three inner edges
// upstream: src/dsp/dec_neon.c VFilter16i_NEON
#[target_feature(enable = "neon")]
fn v_filter16i_neon(
    p: &mut [u8],
    pos: usize,
    stride: i32,
    thresh: i32,
    ithresh: i32,
    hev_thresh: i32,
) {
    let mut pos = pos;
    let (mut p3, mut p2, mut p1, mut p0) = load16x4_neon(p, at(pos, 2 * stride), stride);
    for _ in 0..3 {
        pos = at(pos, 4 * stride);
        let (q0, q1, q2, q3) = load16x4_neon(p, at(pos, 2 * stride), stride);
        {
            let mask = needs_filter2_neon(p3, p2, p1, p0, q0, q1, q2, q3, ithresh, thresh);
            let hev_mask = needs_hev_neon(p1, p0, q0, q1, hev_thresh);
            // p3 and p2 are not just temporary variables here: they will be
            // re-used for next span. And q2/q3 will become p1/p0 accordingly.
            (p1, p0, p3, p2) = do_filter4_neon(p1, p0, q0, q1, mask, hev_mask);
            store16x4_neon(p1, p0, p3, p2, p, pos, stride);
            p1 = q2;
            p0 = q3;
        }
    }
}

/// Entry point of `VFilter16i_NEON` for `VP8VFilter16i`.
pub fn v_filter16i(
    p: &mut [u8],
    pos: usize,
    stride: i32,
    thresh: i32,
    ithresh: i32,
    hev_thresh: i32,
) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { v_filter16i_neon(p, pos, stride, thresh, ithresh, hev_thresh) }
}

// upstream: src/dsp/dec_neon.c HFilter16i_NEON
#[target_feature(enable = "neon")]
fn h_filter16i_neon(
    p: &mut [u8],
    pos: usize,
    stride: i32,
    thresh: i32,
    ithresh: i32,
    hev_thresh: i32,
) {
    let mut pos = pos;
    let (mut p3, mut p2, mut p1, mut p0) = load4x16_neon(p, pos + 2, stride);
    for _ in 0..3 {
        pos += 4;
        let (q0, q1, q2, q3) = load4x16_neon(p, pos + 2, stride);
        {
            let mask = needs_filter2_neon(p3, p2, p1, p0, q0, q1, q2, q3, ithresh, thresh);
            let hev_mask = needs_hev_neon(p1, p0, q0, q1, hev_thresh);
            (p1, p0, p3, p2) = do_filter4_neon(p1, p0, q0, q1, mask, hev_mask);
            store4x16_neon(p1, p0, p3, p2, p, pos, stride);
            p1 = q2;
            p0 = q3;
        }
    }
}

/// Entry point of `HFilter16i_NEON` for `VP8HFilter16i`.
pub fn h_filter16i(
    p: &mut [u8],
    pos: usize,
    stride: i32,
    thresh: i32,
    ithresh: i32,
    hev_thresh: i32,
) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { h_filter16i_neon(p, pos, stride, thresh, ithresh, hev_thresh) }
}

// 8-pixels wide variant, for chroma filtering
// upstream: src/dsp/dec_neon.c VFilter8_NEON
#[target_feature(enable = "neon")]
#[expect(
    clippy::too_many_arguments,
    reason = "the C parameters with two positions"
)]
fn v_filter8_neon(
    u: &mut [u8],
    u_pos: usize,
    v: &mut [u8],
    v_pos: usize,
    stride: i32,
    thresh: i32,
    ithresh: i32,
    hev_thresh: i32,
) {
    let (p3, p2, p1, p0, q0, q1, q2, q3) = load8x8x2_neon(u, u_pos, v, v_pos, stride);
    {
        let mask = needs_filter2_neon(p3, p2, p1, p0, q0, q1, q2, q3, ithresh, thresh);
        let hev_mask = needs_hev_neon(p1, p0, q0, q1, hev_thresh);
        let (op2, op1, op0, oq0, oq1, oq2) =
            do_filter6_neon(p2, p1, p0, q0, q1, q2, mask, hev_mask);
        store8x2x2_neon(
            op2,
            op1,
            u,
            at(u_pos, -2 * stride),
            v,
            at(v_pos, -2 * stride),
            stride,
        );
        store8x2x2_neon(op0, oq0, u, u_pos, v, v_pos, stride);
        store8x2x2_neon(
            oq1,
            oq2,
            u,
            at(u_pos, 2 * stride),
            v,
            at(v_pos, 2 * stride),
            stride,
        );
    }
}

/// Entry point of `VFilter8_NEON` for `VP8VFilter8`.
#[expect(
    clippy::too_many_arguments,
    reason = "the C parameters with two positions"
)]
pub fn v_filter8(
    u: &mut [u8],
    u_pos: usize,
    v: &mut [u8],
    v_pos: usize,
    stride: i32,
    thresh: i32,
    ithresh: i32,
    hev_thresh: i32,
) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { v_filter8_neon(u, u_pos, v, v_pos, stride, thresh, ithresh, hev_thresh) }
}

// upstream: src/dsp/dec_neon.c VFilter8i_NEON
#[target_feature(enable = "neon")]
#[expect(
    clippy::too_many_arguments,
    reason = "the C parameters with two positions"
)]
fn v_filter8i_neon(
    u: &mut [u8],
    u_pos: usize,
    v: &mut [u8],
    v_pos: usize,
    stride: i32,
    thresh: i32,
    ithresh: i32,
    hev_thresh: i32,
) {
    let u_pos = at(u_pos, 4 * stride);
    let v_pos = at(v_pos, 4 * stride);
    let (p3, p2, p1, p0, q0, q1, q2, q3) = load8x8x2_neon(u, u_pos, v, v_pos, stride);
    {
        let mask = needs_filter2_neon(p3, p2, p1, p0, q0, q1, q2, q3, ithresh, thresh);
        let hev_mask = needs_hev_neon(p1, p0, q0, q1, hev_thresh);
        let (op1, op0, oq0, oq1) = do_filter4_neon(p1, p0, q0, q1, mask, hev_mask);
        store8x4x2_neon(op1, op0, oq0, oq1, u, u_pos, v, v_pos, stride);
    }
}

/// Entry point of `VFilter8i_NEON` for `VP8VFilter8i`.
#[expect(
    clippy::too_many_arguments,
    reason = "the C parameters with two positions"
)]
pub fn v_filter8i(
    u: &mut [u8],
    u_pos: usize,
    v: &mut [u8],
    v_pos: usize,
    stride: i32,
    thresh: i32,
    ithresh: i32,
    hev_thresh: i32,
) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { v_filter8i_neon(u, u_pos, v, v_pos, stride, thresh, ithresh, hev_thresh) }
}

// upstream: src/dsp/dec_neon.c HFilter8_NEON
#[target_feature(enable = "neon")]
#[expect(
    clippy::too_many_arguments,
    reason = "the C parameters with two positions"
)]
fn h_filter8_neon(
    u: &mut [u8],
    u_pos: usize,
    v: &mut [u8],
    v_pos: usize,
    stride: i32,
    thresh: i32,
    ithresh: i32,
    hev_thresh: i32,
) {
    let (p3, p2, p1, p0, q0, q1, q2, q3) = load8x8x2t_neon(u, u_pos, v, v_pos, stride);
    {
        let mask = needs_filter2_neon(p3, p2, p1, p0, q0, q1, q2, q3, ithresh, thresh);
        let hev_mask = needs_hev_neon(p1, p0, q0, q1, hev_thresh);
        let (op2, op1, op0, oq0, oq1, oq2) =
            do_filter6_neon(p2, p1, p0, q0, q1, q2, mask, hev_mask);
        store6x8x2_neon(op2, op1, op0, oq0, oq1, oq2, u, u_pos, v, v_pos, stride);
    }
}

/// Entry point of `HFilter8_NEON` for `VP8HFilter8`.
#[expect(
    clippy::too_many_arguments,
    reason = "the C parameters with two positions"
)]
pub fn h_filter8(
    u: &mut [u8],
    u_pos: usize,
    v: &mut [u8],
    v_pos: usize,
    stride: i32,
    thresh: i32,
    ithresh: i32,
    hev_thresh: i32,
) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { h_filter8_neon(u, u_pos, v, v_pos, stride, thresh, ithresh, hev_thresh) }
}

// upstream: src/dsp/dec_neon.c HFilter8i_NEON
#[target_feature(enable = "neon")]
#[expect(
    clippy::too_many_arguments,
    reason = "the C parameters with two positions"
)]
fn h_filter8i_neon(
    u: &mut [u8],
    u_pos: usize,
    v: &mut [u8],
    v_pos: usize,
    stride: i32,
    thresh: i32,
    ithresh: i32,
    hev_thresh: i32,
) {
    let u_pos = u_pos + 4;
    let v_pos = v_pos + 4;
    let (p3, p2, p1, p0, q0, q1, q2, q3) = load8x8x2t_neon(u, u_pos, v, v_pos, stride);
    {
        let mask = needs_filter2_neon(p3, p2, p1, p0, q0, q1, q2, q3, ithresh, thresh);
        let hev_mask = needs_hev_neon(p1, p0, q0, q1, hev_thresh);
        let (op1, op0, oq0, oq1) = do_filter4_neon(p1, p0, q0, q1, mask, hev_mask);
        store4x8x2_neon(op1, op0, oq0, oq1, u, u_pos, v, v_pos, stride);
    }
}

/// Entry point of `HFilter8i_NEON` for `VP8HFilter8i`.
#[expect(
    clippy::too_many_arguments,
    reason = "the C parameters with two positions"
)]
pub fn h_filter8i(
    u: &mut [u8],
    u_pos: usize,
    v: &mut [u8],
    v_pos: usize,
    stride: i32,
    thresh: i32,
    ithresh: i32,
    hev_thresh: i32,
) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { h_filter8i_neon(u, u_pos, v, v_pos, stride, thresh, ithresh, hev_thresh) }
}

//-----------------------------------------------------------------------------
// Inverse transforms (Paragraph 14.4)

// Technically these are unsigned but vqdmulh is only available in signed.
// vqdmulh returns high half (effectively >> 16) but also doubles the value,
// changing the >> 16 to >> 15 and requiring an additional >> 1.
// We use this to our advantage with kC2. The canonical value is 35468.
// However, the high bit is set so treating it as signed will give incorrect
// results. We avoid this by down shifting by 1 here to clear the highest bit.
// Combined with the doubling effect of vqdmulh we get >> 16.
// This can not be applied to kC1 because the lowest bit is set. Down shifting
// the constant would reduce precision.

// libwebp uses a trick to avoid some extra addition that libvpx does.
// Instead of:
// temp2 = ip[12] + ((ip[12] * cospi8sqrt2minus1) >> 16);
// libwebp adds 1 << 16 to cospi8sqrt2minus1 (kC1). However, this causes the
// same issue with kC1 and vqdmulh that we work around by down shifting kC2

// upstream: src/dsp/dec_neon.c kC1
#[expect(
    clippy::cast_possible_truncation,
    reason = "C initializes the int16_t with the int constant 20091"
)]
const C1: i16 = WEBP_TRANSFORM_AC3_C1 as i16;
// upstream: src/dsp/dec_neon.c kC2
/// Half of `WEBP_TRANSFORM_AC3_C2`, actually. See comment above.
#[expect(
    clippy::cast_possible_truncation,
    reason = "C initializes the int16_t with the int constant 17734"
)]
const C2: i16 = (WEBP_TRANSFORM_AC3_C2 / 2) as i16;

// upstream: src/dsp/dec_neon.c Transpose8x2_NEON
#[target_feature(enable = "neon")]
fn transpose8x2_neon(in0: int16x8_t, in1: int16x8_t) -> int16x8x2_t {
    // a0 a1 a2 a3 | b0 b1 b2 b3   => a0 b0 c0 d0 | a1 b1 c1 d1
    // c0 c1 c2 c3 | d0 d1 d2 d3      a2 b2 c2 d2 | a3 b3 c3 d3
    let tmp0 = vzipq_s16(in0, in1); // a0 c0 a1 c1 a2 c2 ...
    //                                 b0 d0 b1 d1 b2 d2 ...
    vzipq_s16(tmp0.0, tmp0.1)
}

// upstream: src/dsp/dec_neon.c TransformPass_NEON
#[target_feature(enable = "neon")]
fn transform_pass_neon(rows: int16x8x2_t) -> int16x8x2_t {
    // {rows} = in0 | in4
    //          in8 | in12
    // B1 = in4 | in12
    let b1 = vcombine_s16(vget_high_s16(rows.0), vget_high_s16(rows.1));
    // C0 = kC1 * in4 | kC1 * in12
    // C1 = kC2 * in4 | kC2 * in12
    let c0 = vsraq_n_s16::<1>(b1, vqdmulhq_n_s16(b1, C1));
    let c1 = vqdmulhq_n_s16(b1, C2);
    let a = vqadd_s16(vget_low_s16(rows.0), vget_low_s16(rows.1)); // in0 + in8
    let b = vqsub_s16(vget_low_s16(rows.0), vget_low_s16(rows.1)); // in0 - in8
    // c = kC2 * in4 - kC1 * in12
    // d = kC1 * in4 + kC2 * in12
    let c = vqsub_s16(vget_low_s16(c1), vget_high_s16(c0));
    let d = vqadd_s16(vget_low_s16(c0), vget_high_s16(c1));
    let d0 = vcombine_s16(a, b); // D0 = a | b
    let d1 = vcombine_s16(d, c); // D1 = d | c
    let e0 = vqaddq_s16(d0, d1); // a+d | b+c
    let e_tmp = vqsubq_s16(d0, d1); // a-d | b-c
    let e1 = vcombine_s16(vget_high_s16(e_tmp), vget_low_s16(e_tmp));
    transpose8x2_neon(e0, e1)
}

// upstream: src/dsp/dec_neon.c TransformOne_NEON
#[target_feature(enable = "neon")]
fn transform_one_neon(input: &[i16], dst: &mut [u8]) {
    let mut rows = int16x8x2_t(vld1q_s16(input), vld1q_s16(&input[8..]));
    rows = transform_pass_neon(rows);
    rows = transform_pass_neon(rows);
    add4x4_neon(rows.0, rows.1, dst);
}

// upstream: src/dsp/dec_neon.c TransformTwo_NEON
#[target_feature(enable = "neon")]
fn transform_two_neon(input: &[i16], dst: &mut [u8], do_two: bool) {
    transform_one_neon(input, dst);
    if do_two {
        transform_one_neon(&input[16..], &mut dst[4..]);
    }
}

/// Entry point of `TransformTwo_NEON` for `VP8Transform`.
pub fn transform_two(input: &[i16], dst: &mut [u8], do_two: bool) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { transform_two_neon(input, dst, do_two) }
}

// upstream: src/dsp/dec_neon.c TransformDC_NEON
#[target_feature(enable = "neon")]
fn transform_dc_neon(input: &[i16], dst: &mut [u8]) {
    let dc = vdupq_n_s16(input[0]);
    add4x4_neon(dc, dc, dst);
}

/// Entry point of `TransformDC_NEON` for `VP8TransformDC`.
pub fn transform_dc(input: &[i16], dst: &mut [u8]) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { transform_dc_neon(input, dst) }
}

//------------------------------------------------------------------------------

// upstream: src/dsp/dec_neon.c STORE_WHT
/// Stores lane `COL` of the four rows to `out[dst]`, `out[dst + 16]`,
/// `out[dst + 32]` and `out[dst + 48]`; returns `dst + 64`.
#[target_feature(enable = "neon")]
#[expect(
    clippy::cast_possible_truncation,
    reason = "C stores the int32_t lanes in int16_t"
)]
fn store_wht<const COL: i32>(out: &mut [i16], dst: usize, rows: int32x4x4_t) -> usize {
    out[dst] = vgetq_lane_s32::<COL>(rows.0) as i16;
    out[dst + 16] = vgetq_lane_s32::<COL>(rows.1) as i16;
    out[dst + 32] = vgetq_lane_s32::<COL>(rows.2) as i16;
    out[dst + 48] = vgetq_lane_s32::<COL>(rows.3) as i16;
    dst + 64
}

// upstream: src/dsp/dec_neon.c TransformWHT_NEON
#[target_feature(enable = "neon")]
fn transform_wht_neon(input: &[i16], out: &mut [i16]) {
    let mut tmp;

    {
        // Load the source.
        let in00_03 = vld1_s16(input);
        let in04_07 = vld1_s16(&input[4..]);
        let in08_11 = vld1_s16(&input[8..]);
        let in12_15 = vld1_s16(&input[12..]);
        let a0 = vaddl_s16(in00_03, in12_15); // in[0..3] + in[12..15]
        let a1 = vaddl_s16(in04_07, in08_11); // in[4..7] + in[8..11]
        let a2 = vsubl_s16(in04_07, in08_11); // in[4..7] - in[8..11]
        let a3 = vsubl_s16(in00_03, in12_15); // in[0..3] - in[12..15]
        tmp = int32x4x4_t(
            vaddq_s32(a0, a1),
            vaddq_s32(a3, a2),
            vsubq_s32(a0, a1),
            vsubq_s32(a3, a2),
        );
        // Arrange the temporary results column-wise.
        tmp = transpose4x4_neon(tmp);
    }

    {
        let k_cst3 = vdupq_n_s32(3);
        let dc = vaddq_s32(tmp.0, k_cst3); // add rounder
        let a0 = vaddq_s32(dc, tmp.3);
        let a1 = vaddq_s32(tmp.1, tmp.2);
        let a2 = vsubq_s32(tmp.1, tmp.2);
        let a3 = vsubq_s32(dc, tmp.3);

        tmp.0 = vaddq_s32(a0, a1);
        tmp.1 = vaddq_s32(a3, a2);
        tmp.2 = vsubq_s32(a0, a1);
        tmp.3 = vsubq_s32(a3, a2);

        // right shift the results by 3.
        tmp.0 = vshrq_n_s32::<3>(tmp.0);
        tmp.1 = vshrq_n_s32::<3>(tmp.1);
        tmp.2 = vshrq_n_s32::<3>(tmp.2);
        tmp.3 = vshrq_n_s32::<3>(tmp.3);

        let mut dst = 0;
        dst = store_wht::<0>(out, dst, tmp);
        dst = store_wht::<1>(out, dst, tmp);
        dst = store_wht::<2>(out, dst, tmp);
        store_wht::<3>(out, dst, tmp);
    }
}

/// Entry point of `TransformWHT_NEON` for `VP8TransformWHT`.
pub fn transform_wht(input: &[i16], out: &mut [i16]) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { transform_wht_neon(input, out) }
}

//------------------------------------------------------------------------------

/// `(uint64_t)(value & 0xffff)`.
#[expect(clippy::cast_sign_loss, reason = "the masked value is not negative")]
fn low16(value: i32) -> u64 {
    (value & 0xffff) as u64
}

// upstream: src/dsp/dec_neon.c TransformAC3_NEON
#[target_feature(enable = "neon")]
#[expect(
    clippy::cast_possible_truncation,
    reason = "C passes the int products to the int16_t parameter of vdup_n_s16"
)]
fn transform_ac3_neon(input: &[i16], dst: &mut [u8]) {
    let a = vdup_n_s16(input[0]); // vld1_dup_s16(in)
    let c4 = vdup_n_s16(webp_transform_ac3_mul2(i32::from(input[4])) as i16);
    let d4 = vdup_n_s16(webp_transform_ac3_mul1(i32::from(input[4])) as i16);
    let c1 = webp_transform_ac3_mul2(i32::from(input[1]));
    let d1 = webp_transform_ac3_mul1(i32::from(input[1]));
    let cd: u64 = low16(d1) | (low16(c1) << 16) | (low16(-c1) << 32) | (low16(-d1) << 48);
    let cd_v = vcreate_s16(cd);
    let b = vqadd_s16(a, cd_v);
    let m0_m1 = vcombine_s16(vqadd_s16(b, d4), vqadd_s16(b, c4));
    let m2_m3 = vcombine_s16(vqsub_s16(b, c4), vqsub_s16(b, d4));
    add4x4_neon(m0_m1, m2_m3, dst);
}

/// Entry point of `TransformAC3_NEON` for `VP8TransformAC3`.
pub fn transform_ac3(input: &[i16], dst: &mut [u8]) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { transform_ac3_neon(input, dst) }
}

//------------------------------------------------------------------------------
// 4x4

// upstream: src/dsp/dec_neon.c DC4_NEON
/// DC.
#[target_feature(enable = "neon")]
fn dc4_neon(buf: &mut [u8], dst: usize) {
    let a = vld1_u8(&buf[dst - BPS_U..]); // top row
    let p0 = vpaddl_u8(a); // cascading summation of the top
    let p1 = vpadd_u16(p0, p0);
    let l0 = vld1_u8(&buf[dst - 1..]);
    let l1 = vld1_u8(&buf[dst + BPS_U - 1..]);
    let l2 = vld1_u8(&buf[dst + 2 * BPS_U - 1..]);
    let l3 = vld1_u8(&buf[dst + 3 * BPS_U - 1..]);
    let s0 = vaddl_u8(l0, l1);
    let s1 = vaddl_u8(l2, l3);
    let s01 = vaddq_u16(s0, s1);
    let sum = vaddq_u16(s01, vcombine_u16(p1, p1));
    let dc0 = vrshrn_n_u16::<3>(sum); // (sum + 4) >> 3
    let dc = vdup_lane_u8::<0>(dc0);
    for i in 0..4 {
        store_lane0_u32(&mut buf[dst + i * BPS_U..], vreinterpret_u32_u8(dc));
    }
}

/// Entry point of `DC4_NEON` for `VP8PredLuma4[0]`.
pub fn dc4(buf: &mut [u8], dst: usize) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { dc4_neon(buf, dst) }
}

// upstream: src/dsp/dec_neon.c TrueMotion_NEON
/// `TrueMotion` (4x4 + 8x8).
#[target_feature(enable = "neon")]
fn true_motion_neon(buf: &mut [u8], dst: usize, size: usize) {
    let tl = vdup_n_u8(buf[dst - BPS_U - 1]); // top-left pixel 'A[-1]'
    let t = vld1_u8(&buf[dst - BPS_U..]); // top row 'A[0..3]'
    let d = vsubl_u8(t, tl); // A[c] - A[-1]
    let mut dst = dst;
    let mut y = 0;
    while y < size {
        // left edge
        let l0 = vdup_n_u8(buf[dst - 1]);
        let l1 = vdup_n_u8(buf[dst + BPS_U - 1]);
        let l2 = vdup_n_u8(buf[dst + 2 * BPS_U - 1]);
        let l3 = vdup_n_u8(buf[dst + 3 * BPS_U - 1]);
        // L[r] + A[c] - A[-1]
        let r0 = vreinterpretq_s16_u16(vaddw_u8(d, l0));
        let r1 = vreinterpretq_s16_u16(vaddw_u8(d, l1));
        let r2 = vreinterpretq_s16_u16(vaddw_u8(d, l2));
        let r3 = vreinterpretq_s16_u16(vaddw_u8(d, l3));
        // Saturate and store the result.
        let r0_u32 = vreinterpret_u32_u8(vqmovun_s16(r0));
        let r1_u32 = vreinterpret_u32_u8(vqmovun_s16(r1));
        let r2_u32 = vreinterpret_u32_u8(vqmovun_s16(r2));
        let r3_u32 = vreinterpret_u32_u8(vqmovun_s16(r3));
        if size == 4 {
            store_lane0_u32(&mut buf[dst..], r0_u32);
            store_lane0_u32(&mut buf[dst + BPS_U..], r1_u32);
            store_lane0_u32(&mut buf[dst + 2 * BPS_U..], r2_u32);
            store_lane0_u32(&mut buf[dst + 3 * BPS_U..], r3_u32);
        } else {
            vst1_u8(&mut buf[dst..], vreinterpret_u8_u32(r0_u32));
            vst1_u8(&mut buf[dst + BPS_U..], vreinterpret_u8_u32(r1_u32));
            vst1_u8(&mut buf[dst + 2 * BPS_U..], vreinterpret_u8_u32(r2_u32));
            vst1_u8(&mut buf[dst + 3 * BPS_U..], vreinterpret_u8_u32(r3_u32));
        }
        dst += 4 * BPS_U;
        y += 4;
    }
}

// upstream: src/dsp/dec_neon.c TM4_NEON
#[target_feature(enable = "neon")]
fn tm4_neon(buf: &mut [u8], dst: usize) {
    true_motion_neon(buf, dst, 4);
}

/// Entry point of `TM4_NEON` for `VP8PredLuma4[1]`.
pub fn tm4(buf: &mut [u8], dst: usize) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { tm4_neon(buf, dst) }
}

// upstream: src/dsp/dec_neon.c VE4_NEON
/// Vertical.
#[target_feature(enable = "neon")]
fn ve4_neon(buf: &mut [u8], dst: usize) {
    // NB: avoid vld1_u64 here as an alignment hint may be added -> SIGBUS.
    let a0 = vreinterpret_u64_u8(vld1_u8(&buf[dst - BPS_U - 1..])); // top row
    let a1 = vshr_n_u64::<8>(a0);
    let a2 = vshr_n_u64::<16>(a0);
    let abcdefgh = vreinterpret_u8_u64(a0);
    let bcdefgh0 = vreinterpret_u8_u64(a1);
    let cdefgh00 = vreinterpret_u8_u64(a2);
    let b = vhadd_u8(abcdefgh, cdefgh00);
    let avg = vrhadd_u8(b, bcdefgh0);
    for i in 0..4 {
        store_lane0_u32(&mut buf[dst + i * BPS_U..], vreinterpret_u32_u8(avg));
    }
}

/// Entry point of `VE4_NEON` for `VP8PredLuma4[2]`.
pub fn ve4(buf: &mut [u8], dst: usize) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { ve4_neon(buf, dst) }
}

// upstream: src/dsp/dec_neon.c RD4_NEON
/// Down-right. The C names spell a zero byte as `_`; here it is `z`.
#[target_feature(enable = "neon")]
#[expect(
    clippy::many_single_char_names,
    clippy::similar_names,
    reason = "the C sample names I, J, K, L, D and the shifted rows"
)]
fn rd4_neon(buf: &mut [u8], dst: usize) {
    let xabcd_u8 = vld1_u8(&buf[dst - BPS_U - 1..]);
    let xabcd = vreinterpret_u64_u8(xabcd_u8);
    let zzzzxabc = vshl_n_u64::<32>(xabcd);
    let i = u32::from(buf[dst - 1]);
    let j = u32::from(buf[dst - 1 + BPS_U]);
    let k = u32::from(buf[dst - 1 + 2 * BPS_U]);
    let l = u32::from(buf[dst - 1 + 3 * BPS_U]);
    let lkjizzzz = vcreate_u64(u64::from(l | (k << 8) | (j << 16) | (i << 24)));
    let lkjixabc = vorr_u64(lkjizzzz, zzzzxabc);
    let kjixabcz = vreinterpret_u8_u64(vshr_n_u64::<8>(lkjixabc));
    let jixabczz = vreinterpret_u8_u64(vshr_n_u64::<16>(lkjixabc));
    let d = vget_lane_u8::<4>(xabcd_u8);
    let jixabcdz = vset_lane_u8::<6>(d, jixabczz);
    let lkjixabc_u8 = vreinterpret_u8_u64(lkjixabc);
    let avg1 = vhadd_u8(jixabcdz, lkjixabc_u8);
    let avg2 = vrhadd_u8(avg1, kjixabcz);
    let avg2_u64 = vreinterpret_u64_u8(avg2);
    let r3 = vreinterpret_u32_u8(avg2);
    let r2 = vreinterpret_u32_u64(vshr_n_u64::<8>(avg2_u64));
    let r1 = vreinterpret_u32_u64(vshr_n_u64::<16>(avg2_u64));
    let r0 = vreinterpret_u32_u64(vshr_n_u64::<24>(avg2_u64));
    store_lane0_u32(&mut buf[dst..], r0);
    store_lane0_u32(&mut buf[dst + BPS_U..], r1);
    store_lane0_u32(&mut buf[dst + 2 * BPS_U..], r2);
    store_lane0_u32(&mut buf[dst + 3 * BPS_U..], r3);
}

/// Entry point of `RD4_NEON` for `VP8PredLuma4[4]`.
pub fn rd4(buf: &mut [u8], dst: usize) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { rd4_neon(buf, dst) }
}

// upstream: src/dsp/dec_neon.c LD4_NEON
/// Down-left.
#[target_feature(enable = "neon")]
#[expect(clippy::similar_names, reason = "the C names CDEFGH00 and CDEFGHH0")]
fn ld4_neon(buf: &mut [u8], dst: usize) {
    // Note using the same shift trick as VE4() is slower here.
    let abcdefgh = vld1_u8(&buf[dst - BPS_U..]);
    let bcdefgh0 = vld1_u8(&buf[dst - BPS_U + 1..]);
    let cdefgh00 = vld1_u8(&buf[dst - BPS_U + 2..]);
    let cdefghh0 = vset_lane_u8::<6>(buf[dst - BPS_U + 7], cdefgh00);
    let avg1 = vhadd_u8(abcdefgh, cdefghh0);
    let avg2 = vrhadd_u8(avg1, bcdefgh0);
    let avg2_u64 = vreinterpret_u64_u8(avg2);
    let r0 = vreinterpret_u32_u8(avg2);
    let r1 = vreinterpret_u32_u64(vshr_n_u64::<8>(avg2_u64));
    let r2 = vreinterpret_u32_u64(vshr_n_u64::<16>(avg2_u64));
    let r3 = vreinterpret_u32_u64(vshr_n_u64::<24>(avg2_u64));
    store_lane0_u32(&mut buf[dst..], r0);
    store_lane0_u32(&mut buf[dst + BPS_U..], r1);
    store_lane0_u32(&mut buf[dst + 2 * BPS_U..], r2);
    store_lane0_u32(&mut buf[dst + 3 * BPS_U..], r3);
}

/// Entry point of `LD4_NEON` for `VP8PredLuma4[6]`.
pub fn ld4(buf: &mut [u8], dst: usize) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { ld4_neon(buf, dst) }
}

//------------------------------------------------------------------------------
// Chroma

// upstream: src/dsp/dec_neon.c VE8uv_NEON
/// Vertical.
#[target_feature(enable = "neon")]
fn ve8uv_neon(buf: &mut [u8], dst: usize) {
    let top = vld1_u8(&buf[dst - BPS_U..]);
    for j in 0..8 {
        vst1_u8(&mut buf[dst + j * BPS_U..], top);
    }
}

/// Entry point of `VE8uv_NEON` for `VP8PredChroma8[2]`.
pub fn ve8uv(buf: &mut [u8], dst: usize) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { ve8uv_neon(buf, dst) }
}

// upstream: src/dsp/dec_neon.c HE8uv_NEON
/// Horizontal.
#[target_feature(enable = "neon")]
fn he8uv_neon(buf: &mut [u8], dst: usize) {
    let mut dst = dst;
    for _ in 0..8 {
        let left = vdup_n_u8(buf[dst - 1]); // vld1_dup_u8(dst - 1)
        vst1_u8(&mut buf[dst..], left);
        dst += BPS_U;
    }
}

/// Entry point of `HE8uv_NEON` for `VP8PredChroma8[3]`.
pub fn he8uv(buf: &mut [u8], dst: usize) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { he8uv_neon(buf, dst) }
}

// upstream: src/dsp/dec_neon.c DC8_NEON
#[target_feature(enable = "neon")]
fn dc8_neon(buf: &mut [u8], dst: usize, do_top: bool, do_left: bool) {
    // C leaves the sums unset when they are not computed (deviation 7).
    let mut sum_top = vdupq_n_u16(0);
    let mut sum_left = vdupq_n_u16(0);

    if do_top {
        let a = vld1_u8(&buf[dst - BPS_U..]); // top row
        let p2 = vaddlv_u8(a);
        sum_top = vdupq_n_u16(p2);
    }

    if do_left {
        let l0 = vld1_u8(&buf[dst - 1..]);
        let l1 = vld1_u8(&buf[dst + BPS_U - 1..]);
        let l2 = vld1_u8(&buf[dst + 2 * BPS_U - 1..]);
        let l3 = vld1_u8(&buf[dst + 3 * BPS_U - 1..]);
        let l4 = vld1_u8(&buf[dst + 4 * BPS_U - 1..]);
        let l5 = vld1_u8(&buf[dst + 5 * BPS_U - 1..]);
        let l6 = vld1_u8(&buf[dst + 6 * BPS_U - 1..]);
        let l7 = vld1_u8(&buf[dst + 7 * BPS_U - 1..]);
        let s0 = vaddl_u8(l0, l1);
        let s1 = vaddl_u8(l2, l3);
        let s2 = vaddl_u8(l4, l5);
        let s3 = vaddl_u8(l6, l7);
        let s01 = vaddq_u16(s0, s1);
        let s23 = vaddq_u16(s2, s3);
        sum_left = vaddq_u16(s01, s23);
    }

    let dc0 = if do_top && do_left {
        let sum = vaddq_u16(sum_left, sum_top);
        vrshrn_n_u16::<4>(sum)
    } else if do_top {
        vrshrn_n_u16::<3>(sum_top)
    } else if do_left {
        vrshrn_n_u16::<3>(sum_left)
    } else {
        vdup_n_u8(0x80)
    };

    {
        let dc = vdup_lane_u8::<0>(dc0);
        for i in 0..8 {
            vst1_u8(&mut buf[dst + i * BPS_U..], dc);
        }
    }
}

// upstream: src/dsp/dec_neon.c DC8uv_NEON
#[target_feature(enable = "neon")]
fn dc8uv_neon(buf: &mut [u8], dst: usize) {
    dc8_neon(buf, dst, true, true);
}

/// Entry point of `DC8uv_NEON` for `VP8PredChroma8[0]`.
pub fn dc8uv(buf: &mut [u8], dst: usize) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { dc8uv_neon(buf, dst) }
}

// upstream: src/dsp/dec_neon.c DC8uvNoTop_NEON
#[target_feature(enable = "neon")]
fn dc8uv_no_top_neon(buf: &mut [u8], dst: usize) {
    dc8_neon(buf, dst, false, true);
}

/// Entry point of `DC8uvNoTop_NEON` for `VP8PredChroma8[4]`.
pub fn dc8uv_no_top(buf: &mut [u8], dst: usize) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { dc8uv_no_top_neon(buf, dst) }
}

// upstream: src/dsp/dec_neon.c DC8uvNoLeft_NEON
#[target_feature(enable = "neon")]
fn dc8uv_no_left_neon(buf: &mut [u8], dst: usize) {
    dc8_neon(buf, dst, true, false);
}

/// Entry point of `DC8uvNoLeft_NEON` for `VP8PredChroma8[5]`.
pub fn dc8uv_no_left(buf: &mut [u8], dst: usize) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { dc8uv_no_left_neon(buf, dst) }
}

// upstream: src/dsp/dec_neon.c DC8uvNoTopLeft_NEON
#[target_feature(enable = "neon")]
fn dc8uv_no_top_left_neon(buf: &mut [u8], dst: usize) {
    dc8_neon(buf, dst, false, false);
}

/// Entry point of `DC8uvNoTopLeft_NEON` for `VP8PredChroma8[6]`.
pub fn dc8uv_no_top_left(buf: &mut [u8], dst: usize) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { dc8uv_no_top_left_neon(buf, dst) }
}

// upstream: src/dsp/dec_neon.c TM8uv_NEON
#[target_feature(enable = "neon")]
fn tm8uv_neon(buf: &mut [u8], dst: usize) {
    true_motion_neon(buf, dst, 8);
}

/// Entry point of `TM8uv_NEON` for `VP8PredChroma8[1]`.
pub fn tm8uv(buf: &mut [u8], dst: usize) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { tm8uv_neon(buf, dst) }
}

//------------------------------------------------------------------------------
// 16x16

// upstream: src/dsp/dec_neon.c VE16_NEON
/// Vertical.
#[target_feature(enable = "neon")]
fn ve16_neon(buf: &mut [u8], dst: usize) {
    let top = vld1q_u8(&buf[dst - BPS_U..]);
    for j in 0..16 {
        vst1q_u8(&mut buf[dst + j * BPS_U..], top);
    }
}

/// Entry point of `VE16_NEON` for `VP8PredLuma16[2]`.
pub fn ve16(buf: &mut [u8], dst: usize) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { ve16_neon(buf, dst) }
}

// upstream: src/dsp/dec_neon.c HE16_NEON
/// Horizontal.
#[target_feature(enable = "neon")]
fn he16_neon(buf: &mut [u8], dst: usize) {
    let mut dst = dst;
    for _ in 0..16 {
        let left = vdupq_n_u8(buf[dst - 1]); // vld1q_dup_u8(dst - 1)
        vst1q_u8(&mut buf[dst..], left);
        dst += BPS_U;
    }
}

/// Entry point of `HE16_NEON` for `VP8PredLuma16[3]`.
pub fn he16(buf: &mut [u8], dst: usize) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { he16_neon(buf, dst) }
}

// upstream: src/dsp/dec_neon.c DC16_NEON
#[target_feature(enable = "neon")]
fn dc16_neon(buf: &mut [u8], dst: usize, do_top: bool, do_left: bool) {
    // C leaves the sums unset when they are not computed (deviation 7).
    let mut sum_top = vdupq_n_u16(0);
    let mut sum_left = vdupq_n_u16(0);

    if do_top {
        let a = vld1q_u8(&buf[dst - BPS_U..]); // top row
        let p3 = vaddlvq_u8(a);
        sum_top = vdupq_n_u16(p3);
    }

    if do_left {
        sum_left = vdupq_n_u16(0);
        let mut i = 0;
        while i < 16 {
            let l0 = vld1_u8(&buf[dst + i * BPS_U - 1..]);
            let l1 = vld1_u8(&buf[dst + (i + 1) * BPS_U - 1..]);
            let l2 = vld1_u8(&buf[dst + (i + 2) * BPS_U - 1..]);
            let l3 = vld1_u8(&buf[dst + (i + 3) * BPS_U - 1..]);
            let l4 = vld1_u8(&buf[dst + (i + 4) * BPS_U - 1..]);
            let l5 = vld1_u8(&buf[dst + (i + 5) * BPS_U - 1..]);
            let l6 = vld1_u8(&buf[dst + (i + 6) * BPS_U - 1..]);
            let l7 = vld1_u8(&buf[dst + (i + 7) * BPS_U - 1..]);
            let s0 = vaddl_u8(l0, l1);
            let s1 = vaddl_u8(l2, l3);
            let s2 = vaddl_u8(l4, l5);
            let s3 = vaddl_u8(l6, l7);
            let s01 = vaddq_u16(s0, s1);
            let s23 = vaddq_u16(s2, s3);
            let sum = vaddq_u16(s01, s23);
            sum_left = vaddq_u16(sum_left, sum);
            i += 8;
        }
    }

    let dc0 = if do_top && do_left {
        let sum = vaddq_u16(sum_left, sum_top);
        vrshrn_n_u16::<5>(sum)
    } else if do_top {
        vrshrn_n_u16::<4>(sum_top)
    } else if do_left {
        vrshrn_n_u16::<4>(sum_left)
    } else {
        vdup_n_u8(0x80)
    };

    {
        let dc = vdupq_lane_u8::<0>(dc0);
        for i in 0..16 {
            vst1q_u8(&mut buf[dst + i * BPS_U..], dc);
        }
    }
}

// upstream: src/dsp/dec_neon.c DC16TopLeft_NEON
#[target_feature(enable = "neon")]
fn dc16_top_left_neon(buf: &mut [u8], dst: usize) {
    dc16_neon(buf, dst, true, true);
}

/// Entry point of `DC16TopLeft_NEON` for `VP8PredLuma16[0]`.
pub fn dc16_top_left(buf: &mut [u8], dst: usize) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { dc16_top_left_neon(buf, dst) }
}

// upstream: src/dsp/dec_neon.c DC16NoTop_NEON
#[target_feature(enable = "neon")]
fn dc16_no_top_neon(buf: &mut [u8], dst: usize) {
    dc16_neon(buf, dst, false, true);
}

/// Entry point of `DC16NoTop_NEON` for `VP8PredLuma16[4]`.
pub fn dc16_no_top(buf: &mut [u8], dst: usize) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { dc16_no_top_neon(buf, dst) }
}

// upstream: src/dsp/dec_neon.c DC16NoLeft_NEON
#[target_feature(enable = "neon")]
fn dc16_no_left_neon(buf: &mut [u8], dst: usize) {
    dc16_neon(buf, dst, true, false);
}

/// Entry point of `DC16NoLeft_NEON` for `VP8PredLuma16[5]`.
pub fn dc16_no_left(buf: &mut [u8], dst: usize) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { dc16_no_left_neon(buf, dst) }
}

// upstream: src/dsp/dec_neon.c DC16NoTopLeft_NEON
#[target_feature(enable = "neon")]
fn dc16_no_top_left_neon(buf: &mut [u8], dst: usize) {
    dc16_neon(buf, dst, false, false);
}

/// Entry point of `DC16NoTopLeft_NEON` for `VP8PredLuma16[6]`.
pub fn dc16_no_top_left(buf: &mut [u8], dst: usize) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { dc16_no_top_left_neon(buf, dst) }
}

// upstream: src/dsp/dec_neon.c TM16_NEON
#[target_feature(enable = "neon")]
fn tm16_neon(buf: &mut [u8], dst: usize) {
    let tl = vdup_n_u8(buf[dst - BPS_U - 1]); // top-left pixel 'A[-1]'
    let t = vld1q_u8(&buf[dst - BPS_U..]); // top row 'A[0..15]'
    // A[c] - A[-1]
    let d_lo = vsubl_u8(vget_low_u8(t), tl);
    let d_hi = vsubl_u8(vget_high_u8(t), tl);
    let mut dst = dst;
    let mut y = 0;
    while y < 16 {
        // left edge
        let l0 = vdup_n_u8(buf[dst - 1]);
        let l1 = vdup_n_u8(buf[dst + BPS_U - 1]);
        let l2 = vdup_n_u8(buf[dst + 2 * BPS_U - 1]);
        let l3 = vdup_n_u8(buf[dst + 3 * BPS_U - 1]);
        // L[r] + A[c] - A[-1]
        let r0_lo = vreinterpretq_s16_u16(vaddw_u8(d_lo, l0));
        let r1_lo = vreinterpretq_s16_u16(vaddw_u8(d_lo, l1));
        let r2_lo = vreinterpretq_s16_u16(vaddw_u8(d_lo, l2));
        let r3_lo = vreinterpretq_s16_u16(vaddw_u8(d_lo, l3));
        let r0_hi = vreinterpretq_s16_u16(vaddw_u8(d_hi, l0));
        let r1_hi = vreinterpretq_s16_u16(vaddw_u8(d_hi, l1));
        let r2_hi = vreinterpretq_s16_u16(vaddw_u8(d_hi, l2));
        let r3_hi = vreinterpretq_s16_u16(vaddw_u8(d_hi, l3));
        // Saturate and store the result.
        let row0 = vcombine_u8(vqmovun_s16(r0_lo), vqmovun_s16(r0_hi));
        let row1 = vcombine_u8(vqmovun_s16(r1_lo), vqmovun_s16(r1_hi));
        let row2 = vcombine_u8(vqmovun_s16(r2_lo), vqmovun_s16(r2_hi));
        let row3 = vcombine_u8(vqmovun_s16(r3_lo), vqmovun_s16(r3_hi));
        vst1q_u8(&mut buf[dst..], row0);
        vst1q_u8(&mut buf[dst + BPS_U..], row1);
        vst1q_u8(&mut buf[dst + 2 * BPS_U..], row2);
        vst1q_u8(&mut buf[dst + 3 * BPS_U..], row3);
        dst += 4 * BPS_U;
        y += 4;
    }
}

/// Entry point of `TM16_NEON` for `VP8PredLuma16[1]`.
pub fn tm16(buf: &mut [u8], dst: usize) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { tm16_neon(buf, dst) }
}

//------------------------------------------------------------------------------
// Entry point

// upstream: src/dsp/dec_neon.c VP8DspInitNEON
// The pointer and table assignments are the dispatch functions of
// `src/dsp/dec.rs` and its NEON tables, named in the module documentation
// (ADR 0008).
