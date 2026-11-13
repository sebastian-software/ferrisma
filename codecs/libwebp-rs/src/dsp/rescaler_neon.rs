// libwebp v1.6.0, src/dsp/rescaler_neon.c:
//
// Copyright 2015 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// NEON version of rescaling functions
//
// Author: Skal (pascal.massimino@gmail.com)

//! Translation of `src/dsp/rescaler_neon.c` from libwebp v1.6.0.
//!
//! Work package: WP20 (NEON).
//!
//! Admitted on 2026-11-13 under Ferrisma decision D4 (a SIMD path matches the
//! original's SIMD build at the same level).
//!
//! The NEON variants of the row exports of `src/dsp/rescaler.c`. Each
//! `_neon` function is the body that upstream's `WebPRescalerDspInitNEON`
//! assigns; the safe entry point below it, named after the function without
//! the suffix, is what the dispatch function of `src/dsp/rescaler.rs` calls on
//! 64-bit ARM.
//!
//! `WebPRescalerDspInitNEON` assigns `WebPRescalerExportRowExpand` and
//! `WebPRescalerExportRowShrink`; `WebPRescalerDspInit` calls it
//! unconditionally on 64-bit ARM (`WEBP_NEON_OMIT_C_CODE`), so the two
//! dispatch functions return the NEON entry points there. The imports stay
//! with the C functions, which `WebPRescalerDspInit` assigns outside the
//! `!WEBP_NEON_OMIT_C_CODE` block.
//!
//! Upstream's kernels are not bit-exact with upstream's C functions: they
//! multiply with `vqrdmulhq_s32` by `scale >> 1` (`MAKE_HALF_CST`), which
//! drops the scale's lowest bit, so a few bytes come out one lower than
//! `WebPRescalerExportRowExpand_C` and `WebPRescalerExportRowShrink_C` give
//! (issue #80). Until 2026-11-13 the port kept the C functions on 64-bit ARM
//! (issue #79). Under decision D4 the port's SIMD build matches upstream's
//! SIMD build, so this translation is selected as upstream selects it; the
//! scalar build (`--no-default-features`) keeps the C functions and matches
//! upstream's scalar build. The gate is `tools/check.py --oracle`:
//! `tests/wp4_rescaler.rs` (300 random size pairs among its cases), the
//! scaled decodes of `tests/wp12_decode.rs` and `tests/wp16_decode.rs` and
//! the encoder's `WebPPictureRescale` cases against `release-simd` in the
//! `neon` mode, and against `release` in the scalar build.
//!
//! # Deviations from upstream
//!
//! 1. **Loads, stores and alignment** follow the deviations 1 to 3 of
//!    `src/dsp/neon.rs`.
//! 2. **`wrk->dst`** is the caller's buffer and `wrk.dst_pos` the start of the
//!    row, as in `src/dsp/rescaler.rs` (its deviation 8); `frow` and `irow`
//!    are the rescaler's side rows.
//! 3. **The macros** `LOAD_32x4`, `LOAD_32x8`, `STORE_32x8`, `MAKE_HALF_CST`,
//!    `MULT_FIX` and `MULT_FIX_FLOOR` are functions of the same names.
//! 4. **Assertions** are `debug_assert!` (ADR 0003).
//!
//! # Excluded undefined behavior
//!
//! None.
//!
//! # Checks not applicable
//!
//! 1. **`WEBP_REDUCE_SIZE`** is unset in the reference configuration, so the
//!    file is compiled; with it, upstream compiles no NEON rescaler.
//! 2. **`WEBP_RESCALER_RFIX != 32`** is an `#error` in upstream; the constant
//!    is 32.
//! 3. **A Rust-against-Rust parity test** with the `_c` functions does not
//!    apply: the kernels differ from them by design of upstream. The
//!    differential gate against upstream's NEON build replaces it.

use core::arch::aarch64::{
    int32x4_t, uint32x4_t, vcombine_u16, vcombine_u32, vdupq_n_s32, vdupq_n_u32, vget_high_u32,
    vget_low_u32, vmlal_n_u32, vmovn_u32, vmull_n_u32, vqdmulhq_s32, vqmovn_u16, vqrdmulhq_s32,
    vqsubq_u32, vreinterpretq_s32_u32, vreinterpretq_u32_s32, vrshrn_n_u64,
};

use crate::dsp::neon::{vld1q_u32, vst1_u8, vst1q_u32};
use crate::utils::rescaler_utils::{
    WEBP_RESCALER_ONE, WEBP_RESCALER_RFIX, WebPRescaler, rescaler_t, webp_rescaler_frac,
    webp_rescaler_output_done,
};

// upstream: src/dsp/rescaler_neon.c ROUNDER
const ROUNDER: u64 = WEBP_RESCALER_ONE >> 1;

// upstream: src/dsp/rescaler_neon.c MULT_FIX_C
#[must_use]
fn mult_fix_c(x: u32, y: u32) -> u64 {
    (u64::from(x) * u64::from(y) + ROUNDER) >> WEBP_RESCALER_RFIX
}

// upstream: src/dsp/rescaler_neon.c MULT_FIX_FLOOR_C
#[must_use]
fn mult_fix_floor_c(x: u32, y: u32) -> u64 {
    (u64::from(x) * u64::from(y)) >> WEBP_RESCALER_RFIX
}

/// The low 32 bits of a 64-bit value, as a C conversion to `uint32_t` keeps them.
#[expect(
    clippy::cast_possible_truncation,
    reason = "the C conversion keeps the low 32 bits"
)]
#[must_use]
fn low32(value: u64) -> u32 {
    value as u32
}

/// `(v > 255) ? 255u : (uint8_t)v` for the `int` of a 64-bit product, as the
/// C conversions `(int)` and `(uint8_t)` keep the low bits.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "the C conversions to int and uint8_t keep the low bits"
)]
#[must_use]
fn clip_to_byte(value: u64) -> u8 {
    let v = value as i32;
    if v > 255 { 255 } else { v as u8 }
}

/// The `uint32_t` value of a C `int`, with the two's-complement bits of the sign.
#[expect(
    clippy::cast_sign_loss,
    reason = "C converts a signed value to uint32_t by wrapping it"
)]
#[must_use]
fn as_u32(value: i32) -> u32 {
    value as u32
}

/// The row length `dst_width * num_channels`, which the rescaler keeps
/// positive.
#[must_use]
fn row_length(wrk: &WebPRescaler) -> usize {
    usize::try_from(wrk.dst_width * wrk.num_channels).expect("the row length is positive")
}

// upstream: src/dsp/rescaler_neon.c LOAD_32x4
#[inline]
#[must_use]
fn load_32x4(src: &[rescaler_t]) -> uint32x4_t {
    vld1q_u32(src)
}

// upstream: src/dsp/rescaler_neon.c LOAD_32x8
#[inline]
#[must_use]
fn load_32x8(src: &[rescaler_t]) -> (uint32x4_t, uint32x4_t) {
    (load_32x4(&src[0..]), load_32x4(&src[4..]))
}

// upstream: src/dsp/rescaler_neon.c STORE_32x8
#[inline]
fn store_32x8(src0: uint32x4_t, src1: uint32x4_t, dst: &mut [rescaler_t]) {
    vst1q_u32(&mut dst[0..], src0);
    vst1q_u32(&mut dst[4..], src1);
}

// upstream: src/dsp/rescaler_neon.c MAKE_HALF_CST
/// The scale halved, as the doubling multiplications below expect it.
#[target_feature(enable = "neon")]
#[must_use]
#[expect(
    clippy::cast_possible_wrap,
    reason = "the C conversion (int32_t) of the halved scale keeps its bits"
)]
fn make_half_cst(c: u32) -> int32x4_t {
    vdupq_n_s32((c >> 1) as i32)
}

// upstream: src/dsp/rescaler_neon.c MULT_FIX
/// note: B is actualy scale>>1. See `MAKE_HALF_CST`
#[target_feature(enable = "neon")]
#[must_use]
fn mult_fix(a: uint32x4_t, b: int32x4_t) -> uint32x4_t {
    vreinterpretq_u32_s32(vqrdmulhq_s32(vreinterpretq_s32_u32(a), b))
}

// upstream: src/dsp/rescaler_neon.c MULT_FIX_FLOOR
#[target_feature(enable = "neon")]
#[must_use]
fn mult_fix_floor(a: uint32x4_t, b: int32x4_t) -> uint32x4_t {
    vreinterpretq_u32_s32(vqdmulhq_s32(vreinterpretq_s32_u32(a), b))
}

// upstream: src/dsp/rescaler_neon.c Interpolate_NEON
#[target_feature(enable = "neon")]
#[must_use]
fn interpolate_neon(frow: &[rescaler_t], irow: &[rescaler_t], a: u32, b: u32) -> uint32x4_t {
    let a0 = load_32x4(frow);
    let b0 = load_32x4(irow);
    let c0 = vmull_n_u32(vget_low_u32(a0), a);
    let c1 = vmull_n_u32(vget_high_u32(a0), a);
    let d0 = vmlal_n_u32(c0, vget_low_u32(b0), b);
    let d1 = vmlal_n_u32(c1, vget_high_u32(b0), b);
    vcombine_u32(vrshrn_n_u64::<32>(d0), vrshrn_n_u64::<32>(d1))
}

// upstream: src/dsp/rescaler_neon.c RescalerExportRowExpand_NEON
#[target_feature(enable = "neon")]
fn rescaler_export_row_expand_neon(wrk: &mut WebPRescaler, dst: &mut [u8]) {
    let dst = &mut dst[wrk.dst_pos..];
    let irow = &*wrk.irow;
    let x_out_max = row_length(wrk);
    let max_span = x_out_max & !7;
    let frow = &*wrk.frow;
    let fy_scale = wrk.fy_scale;
    let fy_scale_half = make_half_cst(fy_scale);
    debug_assert!(!webp_rescaler_output_done(wrk));
    debug_assert!(wrk.y_accum <= 0);
    debug_assert!(wrk.y_expand);
    debug_assert!(wrk.y_sub != 0);
    let mut x_out = 0;
    if wrk.y_accum == 0 {
        while x_out < max_span {
            let a0 = load_32x4(&frow[x_out..]);
            let a1 = load_32x4(&frow[x_out + 4..]);
            let b0 = mult_fix(a0, fy_scale_half);
            let b1 = mult_fix(a1, fy_scale_half);
            let c0 = vmovn_u32(b0);
            let c1 = vmovn_u32(b1);
            let d = vqmovn_u16(vcombine_u16(c0, c1));
            vst1_u8(&mut dst[x_out..], d);
            x_out += 8;
        }
        while x_out < x_out_max {
            let j = frow[x_out];
            dst[x_out] = clip_to_byte(mult_fix_c(j, fy_scale));
            x_out += 1;
        }
    } else {
        let b = webp_rescaler_frac(-wrk.y_accum, wrk.y_sub);
        let a = low32(WEBP_RESCALER_ONE - u64::from(b));
        while x_out < max_span {
            let c0 = interpolate_neon(&frow[x_out..], &irow[x_out..], a, b);
            let c1 = interpolate_neon(&frow[x_out + 4..], &irow[x_out + 4..], a, b);
            let d0 = mult_fix(c0, fy_scale_half);
            let d1 = mult_fix(c1, fy_scale_half);
            let e0 = vmovn_u32(d0);
            let e1 = vmovn_u32(d1);
            let f = vqmovn_u16(vcombine_u16(e0, e1));
            vst1_u8(&mut dst[x_out..], f);
            x_out += 8;
        }
        while x_out < x_out_max {
            let i = u64::from(a)
                .wrapping_mul(u64::from(frow[x_out]))
                .wrapping_add(u64::from(b).wrapping_mul(u64::from(irow[x_out])));
            let j = low32(i.wrapping_add(ROUNDER) >> WEBP_RESCALER_RFIX);
            dst[x_out] = clip_to_byte(mult_fix_c(j, fy_scale));
            x_out += 1;
        }
    }
}

/// Entry point of `RescalerExportRowExpand_NEON` for
/// `WebPRescalerExportRowExpand`.
pub fn rescaler_export_row_expand(wrk: &mut WebPRescaler, dst: &mut [u8]) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { rescaler_export_row_expand_neon(wrk, dst) }
}

// upstream: src/dsp/rescaler_neon.c RescalerExportRowShrink_NEON
#[target_feature(enable = "neon")]
fn rescaler_export_row_shrink_neon(wrk: &mut WebPRescaler, dst: &mut [u8]) {
    let dst = &mut dst[wrk.dst_pos..];
    let x_out_max = row_length(wrk);
    let max_span = x_out_max & !7;
    let yscale = wrk.fy_scale.wrapping_mul(as_u32(-wrk.y_accum));
    let fxy_scale = wrk.fxy_scale;
    let zero = vdupq_n_u32(0);
    let yscale_half = make_half_cst(yscale);
    let fxy_scale_half = make_half_cst(fxy_scale);
    debug_assert!(!webp_rescaler_output_done(wrk));
    debug_assert!(wrk.y_accum <= 0);
    debug_assert!(!wrk.y_expand);
    let irow = &mut *wrk.irow;
    let frow = &*wrk.frow;
    let mut x_out = 0;
    if yscale != 0 {
        while x_out < max_span {
            let (in0, in1) = load_32x8(&frow[x_out..]);
            let (in2, in3) = load_32x8(&irow[x_out..]);
            let a0 = mult_fix_floor(in0, yscale_half);
            let a1 = mult_fix_floor(in1, yscale_half);
            let b0 = vqsubq_u32(in2, a0);
            let b1 = vqsubq_u32(in3, a1);
            let c0 = mult_fix(b0, fxy_scale_half);
            let c1 = mult_fix(b1, fxy_scale_half);
            let d0 = vmovn_u32(c0);
            let d1 = vmovn_u32(c1);
            let e = vqmovn_u16(vcombine_u16(d0, d1));
            vst1_u8(&mut dst[x_out..], e);
            store_32x8(a0, a1, &mut irow[x_out..]);
            x_out += 8;
        }
        while x_out < x_out_max {
            let frac = low32(mult_fix_floor_c(frow[x_out], yscale));
            dst[x_out] = clip_to_byte(mult_fix_c(irow[x_out].wrapping_sub(frac), fxy_scale));
            irow[x_out] = frac; // new fractional start
            x_out += 1;
        }
    } else {
        while x_out < max_span {
            let (in0, in1) = load_32x8(&irow[x_out..]);
            let a0 = mult_fix(in0, fxy_scale_half);
            let a1 = mult_fix(in1, fxy_scale_half);
            let b0 = vmovn_u32(a0);
            let b1 = vmovn_u32(a1);
            let c = vqmovn_u16(vcombine_u16(b0, b1));
            vst1_u8(&mut dst[x_out..], c);
            store_32x8(zero, zero, &mut irow[x_out..]);
            x_out += 8;
        }
        while x_out < x_out_max {
            dst[x_out] = clip_to_byte(mult_fix_c(irow[x_out], fxy_scale));
            irow[x_out] = 0;
            x_out += 1;
        }
    }
}

/// Entry point of `RescalerExportRowShrink_NEON` for
/// `WebPRescalerExportRowShrink`.
pub fn rescaler_export_row_shrink(wrk: &mut WebPRescaler, dst: &mut [u8]) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { rescaler_export_row_shrink_neon(wrk, dst) }
}

//------------------------------------------------------------------------------

// upstream: src/dsp/rescaler_neon.c WebPRescalerDspInitNEON
// The pointer assignments are the dispatch functions of `src/dsp/rescaler.rs`
// named in the module documentation (ADR 0008).
