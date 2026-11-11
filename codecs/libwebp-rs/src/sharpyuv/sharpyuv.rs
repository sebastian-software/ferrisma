// libwebp v1.6.0, sharpyuv/sharpyuv.c:
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
// Sharp RGB to YUV conversion.
//
// Author: Skal (pascal.massimino@gmail.com)

// libwebp v1.6.0, sharpyuv/sharpyuv.h:
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
// Sharp RGB to YUV conversion.

//! Translation of `sharpyuv/sharpyuv.c` and `sharpyuv/sharpyuv.h` from libwebp v1.6.0.
//!
//! Work package: WP5 (`SharpYUV`).
use super::sharpyuv_dsp::{sharp_yuv_filter_row, sharp_yuv_update_rgb, sharp_yuv_update_y};
pub use libsharpyuv_core::sharpyuv::*;

/// Converts RGB to YUV through WebP's existing safe slice and SIMD boundary.
/// The origins, steps, and strides must address storage within the supplied
/// slices; an undersized slice panics when indexed, as before extraction.
#[must_use]
#[expect(clippy::too_many_arguments, reason = "mirrors the upstream API")]
pub fn sharp_yuv_convert_with_options(
    r_ptr: &[u8],
    r_origin: usize,
    g_ptr: &[u8],
    g_origin: usize,
    b_ptr: &[u8],
    b_origin: usize,
    rgb_step: i32,
    rgb_stride: i32,
    rgb_bit_depth: i32,
    y_ptr: &mut [u8],
    y_origin: usize,
    y_stride: i32,
    u_ptr: &mut [u8],
    u_origin: usize,
    u_stride: i32,
    v_ptr: &mut [u8],
    v_origin: usize,
    v_stride: i32,
    yuv_bit_depth: i32,
    width: i32,
    height: i32,
    options: &SharpYuvOptions<'_>,
) -> bool {
    libsharpyuv_core::sharpyuv::sharp_yuv_convert_with_options_using(
        r_ptr,
        r_origin,
        g_ptr,
        g_origin,
        b_ptr,
        b_origin,
        rgb_step,
        rgb_stride,
        rgb_bit_depth,
        y_ptr,
        y_origin,
        y_stride,
        u_ptr,
        u_origin,
        u_stride,
        v_ptr,
        v_origin,
        v_stride,
        yuv_bit_depth,
        width,
        height,
        options,
        Dsp {
            update_y: sharp_yuv_update_y,
            update_rgb: sharp_yuv_update_rgb,
            filter_row: sharp_yuv_filter_row,
        },
    )
}

/// Converts RGB to YUV with sRGB transfer and the selected matrix.
#[must_use]
#[expect(clippy::too_many_arguments, reason = "mirrors the upstream API")]
pub fn sharp_yuv_convert(
    r_ptr: &[u8],
    r_origin: usize,
    g_ptr: &[u8],
    g_origin: usize,
    b_ptr: &[u8],
    b_origin: usize,
    rgb_step: i32,
    rgb_stride: i32,
    rgb_bit_depth: i32,
    y_ptr: &mut [u8],
    y_origin: usize,
    y_stride: i32,
    u_ptr: &mut [u8],
    u_origin: usize,
    u_stride: i32,
    v_ptr: &mut [u8],
    v_origin: usize,
    v_stride: i32,
    yuv_bit_depth: i32,
    width: i32,
    height: i32,
    yuv_matrix: &SharpYuvConversionMatrix,
) -> bool {
    let options = SharpYuvOptions {
        yuv_matrix,
        transfer_type: SharpYuvTransferFunctionType::kSharpYuvTransferFunctionSrgb,
    };
    sharp_yuv_convert_with_options(
        r_ptr,
        r_origin,
        g_ptr,
        g_origin,
        b_ptr,
        b_origin,
        rgb_step,
        rgb_stride,
        rgb_bit_depth,
        y_ptr,
        y_origin,
        y_stride,
        u_ptr,
        u_origin,
        u_stride,
        v_ptr,
        v_origin,
        v_stride,
        yuv_bit_depth,
        width,
        height,
        &options,
    )
}
