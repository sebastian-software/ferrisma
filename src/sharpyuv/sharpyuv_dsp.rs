// libwebp v1.6.0, sharpyuv/sharpyuv_dsp.c:
//
// Copyright 2022 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// Speed-critical functions for Sharp YUV.
//
// Author: Skal (pascal.massimino@gmail.com)

// libwebp v1.6.0, sharpyuv/sharpyuv_dsp.h:
//
// Copyright 2022 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// Speed-critical functions for Sharp YUV.

//! Translation of `sharpyuv/sharpyuv_dsp.c` and `sharpyuv/sharpyuv_dsp.h` from libwebp v1.6.0.
//!
//! Work package: WP5 (`SharpYUV`).

// upstream: sharpyuv/sharpyuv_dsp.c clip
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "C integer and float conversions, kept as in the source (ADR 0003)"
)]
fn clip(v: i32, max: i32) -> u16 {
    if v < 0 {
        0
    } else if v > max {
        max as u16
    } else {
        v as u16
    }
}

// upstream: sharpyuv/sharpyuv_dsp.c SharpYuvUpdateY_C
#[expect(
    clippy::cast_sign_loss,
    reason = "C integer and float conversions, kept as in the source (ADR 0003)"
)]
fn sharp_yuv_update_y_c(
    r#ref: &[u16],
    src: &[u16],
    dst: &mut [u16],
    len: i32,
    bit_depth: i32,
) -> u64 {
    let mut diff: u64 = 0;
    let max_y = (1 << bit_depth) - 1;
    let len = len as usize;
    for ((d, &r), &s) in dst[..len].iter_mut().zip(&r#ref[..len]).zip(&src[..len]) {
        let diff_y = i32::from(r) - i32::from(s);
        let new_y = i32::from(*d) + diff_y;
        *d = clip(new_y, max_y);
        diff += u64::from(diff_y.unsigned_abs());
    }
    diff
}

// upstream: sharpyuv/sharpyuv_dsp.c SharpYuvUpdateRGB_C
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "C integer and float conversions, kept as in the source (ADR 0003)"
)]
fn sharp_yuv_update_rgb_c(r#ref: &[i16], src: &[i16], dst: &mut [i16], len: i32) {
    let len = len as usize;
    for ((d, &r), &s) in dst[..len].iter_mut().zip(&r#ref[..len]).zip(&src[..len]) {
        let diff_uv = i32::from(r) - i32::from(s);
        *d = (i32::from(*d) + diff_uv) as i16;
    }
}

// upstream: sharpyuv/sharpyuv_dsp.c SharpYuvFilterRow_C
#[expect(
    clippy::cast_sign_loss,
    reason = "C integer and float conversions, kept as in the source (ADR 0003)"
)]
fn sharp_yuv_filter_row_c(
    a: &[i16],
    b: &[i16],
    len: i32,
    best_y: &[u16],
    out: &mut [u16],
    bit_depth: i32,
) {
    let max_y = (1 << bit_depth) - 1;
    for i in 0..len as usize {
        let a0 = i32::from(a[i]);
        let a1 = i32::from(a[i + 1]);
        let b0 = i32::from(b[i]);
        let b1 = i32::from(b[i + 1]);
        let v0 = (a0 * 9 + a1 * 3 + b0 * 3 + b1 + 8) >> 4;
        let v1 = (a1 * 9 + a0 * 3 + b1 * 3 + b0 + 8) >> 4;
        out[2 * i] = clip(i32::from(best_y[2 * i]) + v0, max_y);
        out[2 * i + 1] = clip(i32::from(best_y[2 * i + 1]) + v1, max_y);
    }
}

// upstream: sharpyuv/sharpyuv_dsp.h SharpYuvUpdateY
// The function pointer of the C dispatch becomes this plain function; it calls
// the portable implementation. The SSE2 and NEON variants of WP19 and WP20 add
// their selection here.
pub fn sharp_yuv_update_y(
    src: &[u16],
    r#ref: &[u16],
    dst: &mut [u16],
    len: i32,
    bit_depth: i32,
) -> u64 {
    sharp_yuv_update_y_c(src, r#ref, dst, len, bit_depth)
}

// upstream: sharpyuv/sharpyuv_dsp.h SharpYuvUpdateRGB
pub fn sharp_yuv_update_rgb(src: &[i16], r#ref: &[i16], dst: &mut [i16], len: i32) {
    sharp_yuv_update_rgb_c(src, r#ref, dst, len);
}

// upstream: sharpyuv/sharpyuv_dsp.h SharpYuvFilterRow
pub fn sharp_yuv_filter_row(
    a: &[i16],
    b: &[i16],
    len: i32,
    best_y: &[u16],
    out: &mut [u16],
    bit_depth: i32,
) {
    sharp_yuv_filter_row_c(a, b, len, best_y, out, bit_depth);
}

// upstream: sharpyuv/sharpyuv_dsp.c SharpYuvInitDsp
// Not translated. The function assigns the three portable pointers above, which
// the dispatch functions call directly (ADR 0003). Its CPU-dependent selection of
// the SSE2 and NEON variants belongs to WP19 and WP20; the query it consults is
// `sharp_yuv_get_cpu_info` (`sharpyuv_cpu.rs`), which the wave 4 packages call.
