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
pub use libsharpyuv_core::sharpyuv_dsp::{
    sharp_yuv_filter_row_c, sharp_yuv_update_rgb_c, sharp_yuv_update_y_c,
};

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
    #[cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]
    {
        // `SharpYuvInitDsp` selects the SSE2 variant when the query answers
        // `kSSE2`.
        if crate::dsp::cpu::features().sse2 {
            return crate::sharpyuv::sharpyuv_sse2::sharp_yuv_update_y(
                src, r#ref, dst, len, bit_depth,
            );
        }
    }
    #[cfg(all(feature = "simd", target_arch = "aarch64"))]
    {
        return crate::sharpyuv::sharpyuv_neon::sharp_yuv_update_y(src, r#ref, dst, len, bit_depth);
    }
    #[allow(
        unreachable_code,
        reason = "the C path is reached only without a SIMD variant"
    )]
    sharp_yuv_update_y_c(src, r#ref, dst, len, bit_depth)
}

// upstream: sharpyuv/sharpyuv_dsp.h SharpYuvUpdateRGB
pub fn sharp_yuv_update_rgb(src: &[i16], r#ref: &[i16], dst: &mut [i16], len: i32) {
    #[cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]
    {
        // `SharpYuvInitDsp` selects the SSE2 variant when the query answers
        // `kSSE2`.
        if crate::dsp::cpu::features().sse2 {
            return crate::sharpyuv::sharpyuv_sse2::sharp_yuv_update_rgb(src, r#ref, dst, len);
        }
    }
    #[cfg(all(feature = "simd", target_arch = "aarch64"))]
    {
        return crate::sharpyuv::sharpyuv_neon::sharp_yuv_update_rgb(src, r#ref, dst, len);
    }
    #[allow(
        unreachable_code,
        reason = "the C path is reached only without a SIMD variant"
    )]
    sharp_yuv_update_rgb_c(src, r#ref, dst, len);
}

// upstream: sharpyuv/sharpyuv_dsp.h SharpYuvFilterRow
/// With `simd` on 64-bit ARM, chroma values beyond the converter's bit depth
/// follow upstream's NEON kernel, not the C function
/// (`src/sharpyuv/sharpyuv_neon.rs`, checks not applicable 1).
pub fn sharp_yuv_filter_row(
    a: &[i16],
    b: &[i16],
    len: i32,
    best_y: &[u16],
    out: &mut [u16],
    bit_depth: i32,
) {
    #[cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]
    {
        // `SharpYuvInitDsp` selects the SSE2 variant when the query answers
        // `kSSE2`.
        if crate::dsp::cpu::features().sse2 {
            return crate::sharpyuv::sharpyuv_sse2::sharp_yuv_filter_row(
                a, b, len, best_y, out, bit_depth,
            );
        }
    }
    #[cfg(all(feature = "simd", target_arch = "aarch64"))]
    {
        return crate::sharpyuv::sharpyuv_neon::sharp_yuv_filter_row(
            a, b, len, best_y, out, bit_depth,
        );
    }
    #[allow(
        unreachable_code,
        reason = "the C path is reached only without a SIMD variant"
    )]
    sharp_yuv_filter_row_c(a, b, len, best_y, out, bit_depth);
}

// upstream: sharpyuv/sharpyuv_dsp.c SharpYuvInitDsp
// Not translated. The function assigns the three portable pointers above, which
// the dispatch functions call directly (ADR 0003). Its CPU-dependent selection of
// the SSE2 and NEON variants belongs to WP19 and WP20; the query it consults is
// `sharp_yuv_get_cpu_info` (`sharpyuv_cpu.rs`), which the wave 4 packages call.
