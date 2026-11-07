// libwebp v1.6.0, src/dsp/rescaler.c:
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
// Rescaling functions
//
// Author: Skal (pascal.massimino@gmail.com)

//! Translation of `src/dsp/rescaler.c` from libwebp v1.6.0.
//!
//! Work package: WP4 (pixel ops).

use crate::utils::rescaler_utils::{
    WEBP_RESCALER_ONE, WEBP_RESCALER_RFIX, WebPRescaler, webp_rescaler_frac,
};

// upstream: src/dsp/rescaler.c ROUNDER
/// The rounding term of the fixed-point products.
const ROUNDER: u64 = WEBP_RESCALER_ONE >> 1;

/// The `uint32_t` value of a C `int`, with the two's-complement bits of the sign.
#[expect(
    clippy::cast_sign_loss,
    reason = "C converts a signed count to uint32_t by wrapping it"
)]
#[must_use]
fn as_u32(value: i32) -> u32 {
    value as u32
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

/// The `int` value of a 64-bit value, as the C conversion `(int)` gives it.
#[expect(
    clippy::cast_possible_truncation,
    reason = "the C conversion to int keeps the low 32 bits, read as two's complement"
)]
#[must_use]
fn as_int(value: u64) -> i32 {
    value as i32
}

/// The low byte of a value, as a C conversion to `uint8_t` keeps it.
#[expect(
    clippy::cast_possible_truncation,
    reason = "a C conversion to uint8_t keeps the low eight bits"
)]
#[must_use]
fn low_byte(value: u32) -> u8 {
    value as u8
}

// upstream: src/dsp/rescaler.c MULT_FIX
/// `x * y` in the fixed-point precision, rounded to nearest.
#[must_use]
fn mult_fix(x: u32, y: u32) -> u64 {
    (u64::from(x) * u64::from(y) + ROUNDER) >> WEBP_RESCALER_RFIX
}

// upstream: src/dsp/rescaler.c MULT_FIX_FLOOR
/// `x * y` in the fixed-point precision, rounded down.
#[must_use]
fn mult_fix_floor(x: u32, y: u32) -> u64 {
    (u64::from(x) * u64::from(y)) >> WEBP_RESCALER_RFIX
}

/// Stores a value in a byte, clipped to 255. A negative `int` keeps its low
/// byte, which the C test `v > 255` lets through.
#[must_use]
fn clip_to_byte(v: i32) -> u8 {
    if v > 255 { 255 } else { low_byte(as_u32(v)) }
}

// upstream: src/dsp/rescaler.c WebPRescalerImportRowExpand_C
/// Imports one row of a rescaler that expands in x: each output sample is a
/// linear interpolation of the two nearest input samples of its channel.
/// # Panics
///
/// Panics when `src` is shorter than the input row that the rescaler reads.
pub fn webp_rescaler_import_row_expand_c(wrk: &mut WebPRescaler, src: &[u8]) {
    let x_stride = usize::try_from(wrk.num_channels).expect("the channel count is positive");
    let x_out_max =
        usize::try_from(wrk.dst_width * wrk.num_channels).expect("the row length is positive");
    for channel in 0..x_stride {
        let mut x_in = channel;
        let mut x_out = channel;
        // simple bilinear interpolation
        let mut accum = wrk.x_add;
        let mut left = u32::from(src[x_in]);
        let mut right = if wrk.src_width > 1 {
            u32::from(src[x_in + x_stride])
        } else {
            left
        };
        x_in += x_stride;
        loop {
            wrk.frow[x_out] = right
                .wrapping_mul(as_u32(wrk.x_add))
                .wrapping_add(left.wrapping_sub(right).wrapping_mul(as_u32(accum)));
            x_out += x_stride;
            if x_out >= x_out_max {
                break;
            }
            accum -= wrk.x_sub;
            if accum < 0 {
                left = right;
                x_in += x_stride;
                right = u32::from(src[x_in]);
                accum += wrk.x_add;
            }
        }
    }
}

// upstream: src/dsp/rescaler.c WebPRescalerImportRowShrink_C
/// Imports one row of a rescaler that shrinks in x: each output sample is the
/// area-weighted average of the input samples it covers, with the fraction of
/// the sample that straddles two outputs carried to the next one.
/// # Panics
///
/// Panics when `src` is shorter than the input row that the rescaler reads.
pub fn webp_rescaler_import_row_shrink_c(wrk: &mut WebPRescaler, src: &[u8]) {
    let x_stride = usize::try_from(wrk.num_channels).expect("the channel count is positive");
    let x_out_max =
        usize::try_from(wrk.dst_width * wrk.num_channels).expect("the row length is positive");
    for channel in 0..x_stride {
        let mut x_in = channel;
        let mut x_out = channel;
        let mut sum: u32 = 0;
        let mut accum: i32 = 0;
        while x_out < x_out_max {
            let mut base: u32 = 0;
            accum += wrk.x_add;
            while accum > 0 {
                accum -= wrk.x_sub;
                base = u32::from(src[x_in]);
                sum = sum.wrapping_add(base);
                x_in += x_stride;
            }
            {
                // Emit next horizontal pixel.
                let frac = base.wrapping_mul(as_u32(-accum));
                wrk.frow[x_out] = sum.wrapping_mul(as_u32(wrk.x_sub)).wrapping_sub(frac);
                // fresh fractional start for next pixel
                sum = low32(mult_fix(frac, wrk.fx_scale));
            }
            x_out += x_stride;
        }
    }
}

// upstream: src/dsp/rescaler.c WebPRescalerExportRowExpand_C
/// Exports one row of a rescaler that expands in y, from the side rows, into
/// `dst`, from the row `dst_pos` on.
///
/// # Panics
///
/// Panics when `dst` is shorter than the row at `dst_pos` that the export writes.
#[expect(
    clippy::many_single_char_names,
    reason = "the C names b, a, i, j and v of the export"
)]
pub fn webp_rescaler_export_row_expand_c(wrk: &mut WebPRescaler, dst: &mut [u8]) {
    let x_out_max =
        usize::try_from(wrk.dst_width * wrk.num_channels).expect("the row length is positive");
    let pos = wrk.dst_pos;
    let dst = &mut dst[pos..];
    let irow = &*wrk.irow;
    let frow = &*wrk.frow;
    if wrk.y_accum == 0 {
        for x_out in 0..x_out_max {
            let j = frow[x_out];
            let v = as_int(mult_fix(j, wrk.fy_scale));
            dst[x_out] = clip_to_byte(v);
        }
    } else {
        let b = webp_rescaler_frac(-wrk.y_accum, wrk.y_sub);
        let a = low32(WEBP_RESCALER_ONE - u64::from(b));
        for x_out in 0..x_out_max {
            let i = u64::from(a)
                .wrapping_mul(u64::from(frow[x_out]))
                .wrapping_add(u64::from(b).wrapping_mul(u64::from(irow[x_out])));
            let j = low32(i.wrapping_add(ROUNDER) >> WEBP_RESCALER_RFIX);
            let v = as_int(mult_fix(j, wrk.fy_scale));
            dst[x_out] = clip_to_byte(v);
        }
    }
}

// upstream: src/dsp/rescaler.c WebPRescalerExportRowShrink_C
/// Exports one row of a rescaler that shrinks in y, into `dst` from the row
/// `dst_pos` on, and carries the fraction of the next output row in the side
/// row.
///
/// # Panics
///
/// Panics when `dst` is shorter than the row at `dst_pos` that the export writes.
pub fn webp_rescaler_export_row_shrink_c(wrk: &mut WebPRescaler, dst: &mut [u8]) {
    let x_out_max =
        usize::try_from(wrk.dst_width * wrk.num_channels).expect("the row length is positive");
    let pos = wrk.dst_pos;
    let dst = &mut dst[pos..];
    let irow = &mut *wrk.irow;
    let frow = &*wrk.frow;
    let yscale = wrk.fy_scale.wrapping_mul(as_u32(-wrk.y_accum));
    if yscale != 0 {
        for x_out in 0..x_out_max {
            let frac = low32(mult_fix_floor(frow[x_out], yscale));
            let v = as_int(mult_fix(irow[x_out].wrapping_sub(frac), wrk.fxy_scale));
            dst[x_out] = clip_to_byte(v);
            irow[x_out] = frac; // new fractional start
        }
    } else {
        for x_out in 0..x_out_max {
            let v = as_int(mult_fix(irow[x_out], wrk.fxy_scale));
            dst[x_out] = clip_to_byte(v);
            irow[x_out] = 0;
        }
    }
}

// upstream: src/dsp/rescaler.c WebPRescalerImportRowExpand
/// The import of a row for a rescaler that expands in x. The C dispatch
/// pointer is `WebPRescalerImportRowExpand_C`.
pub fn webp_rescaler_import_row_expand(wrk: &mut WebPRescaler, src: &[u8]) {
    webp_rescaler_import_row_expand_c(wrk, src);
}

// upstream: src/dsp/rescaler.c WebPRescalerImportRowShrink
/// The import of a row for a rescaler that shrinks in x. The C dispatch
/// pointer is `WebPRescalerImportRowShrink_C`.
pub fn webp_rescaler_import_row_shrink(wrk: &mut WebPRescaler, src: &[u8]) {
    webp_rescaler_import_row_shrink_c(wrk, src);
}

// upstream: src/dsp/rescaler.c WebPRescalerExportRowExpand
/// The export of a row for a rescaler that expands in y. The C dispatch
/// pointer is `WebPRescalerExportRowExpand_C`.
pub fn webp_rescaler_export_row_expand(wrk: &mut WebPRescaler, dst: &mut [u8]) {
    webp_rescaler_export_row_expand_c(wrk, dst);
}

// upstream: src/dsp/rescaler.c WebPRescalerExportRowShrink
/// The export of a row for a rescaler that shrinks in y. The C dispatch
/// pointer is `WebPRescalerExportRowShrink_C`.
pub fn webp_rescaler_export_row_shrink(wrk: &mut WebPRescaler, dst: &mut [u8]) {
    webp_rescaler_export_row_shrink_c(wrk, dst);
}

// upstream: src/dsp/rescaler.c WebPRescalerImportRow
/// Imports one row, with the kernel of the rescaler's direction.
pub fn webp_rescaler_import_row(wrk: &mut WebPRescaler, src: &[u8]) {
    if wrk.x_expand {
        webp_rescaler_import_row_expand(wrk, src);
    } else {
        webp_rescaler_import_row_shrink(wrk, src);
    }
}

// upstream: src/dsp/rescaler.c WebPRescalerExportRow
/// Exports one row into `dst` when one is ready, then advances the destination
/// row and the vertical accumulator.
///
/// # Panics
///
/// Panics when `dst` is shorter than the row at `dst_pos` that the export writes.
pub fn webp_rescaler_export_row(wrk: &mut WebPRescaler, dst: &mut [u8]) {
    if wrk.y_accum <= 0 {
        if wrk.y_expand {
            webp_rescaler_export_row_expand(wrk, dst);
        } else if wrk.fxy_scale != 0 {
            webp_rescaler_export_row_shrink(wrk, dst);
        } else {
            // special case
            let count = usize::try_from(wrk.num_channels * wrk.dst_width)
                .expect("the row length is positive");
            let pos = wrk.dst_pos;
            for i in 0..count {
                dst[pos + i] = low_byte(wrk.irow[i]);
                wrk.irow[i] = 0;
            }
        }
        wrk.y_accum += wrk.y_add;
        let stride = usize::try_from(wrk.dst_stride).expect("the stride is positive");
        wrk.dst_pos += stride;
        wrk.dst_y += 1;
    }
}

// upstream: src/dsp/rescaler.c WebPRescalerDspInit
// The initialization assigns the four row kernels above to their dispatch
// pointers, and calls the SIMD initializers when the CPU query reports them.
// Its only portable effect is the pointer assignment, which ADR 0003 omits.
