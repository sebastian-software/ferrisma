// libwebp v1.6.0, src/dsp/alpha_processing.c:
//
// Copyright 2013 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// Utilities for processing transparent channel.
//
// Author: Skal (pascal.massimino@gmail.com)

//! Translation of `src/dsp/alpha_processing.c` from libwebp v1.6.0.
//!
//! Work package: WP4 (pixel ops).

// upstream: src/dsp/alpha_processing.c MFIX
/// Fixed-point precision of the alpha multiplication.
const MFIX: u32 = 24;
// upstream: src/dsp/alpha_processing.c HALF
/// The rounding term of the alpha multiplication.
const HALF: u32 = (1 << MFIX) >> 1;
// upstream: src/dsp/alpha_processing.c KINV_255
/// The inverse of 255 in the fixed-point precision.
const KINV_255: u32 = (1 << MFIX) / 255;

// upstream: src/dsp/alpha_processing.c MULTIPLIER (premultiplied modes)
// upstream: src/dsp/alpha_processing.c PREMULTIPLY (premultiplied modes)
// The `#if 1` branch of the source: `(int)(x * a / 255.)`.
const MULTIPLIER_SCALE: u32 = 32897;

/// The low byte of a value, as a C conversion to `uint8_t` keeps it.
#[expect(
    clippy::cast_possible_truncation,
    reason = "a C conversion to uint8_t keeps the low eight bits"
)]
#[must_use]
fn low_byte(value: u32) -> u8 {
    value as u8
}

// upstream: src/dsp/alpha_processing.c Mult
/// Multiplies one channel by a fixed-point scale. The product wraps in 32 bits
/// as the C `uint32_t` arithmetic does.
fn mult(x: u8, scale: u32) -> u32 {
    let v = (u32::from(x).wrapping_mul(scale).wrapping_add(HALF)) >> MFIX;
    debug_assert!(v <= 255); // <- 24bit precision is enough to ensure that.
    v
}

// upstream: src/dsp/alpha_processing.c GetScale
/// The scale of one alpha value, inverse or direct. The inverse scale is
/// undefined for alpha 0, and the callers never ask for it.
#[must_use]
fn get_scale(a: u32, inverse: bool) -> u32 {
    if inverse {
        (255 << MFIX) / a
    } else {
        a * KINV_255
    }
}

// upstream: src/dsp/alpha_processing.c WebPMultARGBRow_C
/// Premultiplies (or, with `inverse`, un-premultiplies) `width` ARGB pixels.
/// Opaque pixels are unchanged and fully transparent ones become zero.
pub fn webp_mult_argb_row_c(ptr: &mut [u32], width: i32, inverse: bool) {
    let width = usize::try_from(width).unwrap_or(0);
    for pixel in &mut ptr[..width] {
        let argb = *pixel;
        if argb < 0xff00_0000 {
            // alpha < 255
            if argb <= 0x00ff_ffff {
                // alpha == 0
                *pixel = 0;
            } else {
                let alpha = (argb >> 24) & 0xff;
                let scale = get_scale(alpha, inverse);
                let mut out = argb & 0xff00_0000;
                out |= mult(low_byte(argb), scale);
                out |= mult(low_byte(argb >> 8), scale) << 8;
                out |= mult(low_byte(argb >> 16), scale) << 16;
                *pixel = out;
            }
        }
    }
}

// upstream: src/dsp/alpha_processing.c WebPMultRow_C
/// Premultiplies (or un-premultiplies) one row of single values with the
/// alpha values of the same width.
pub fn webp_mult_row_c(ptr: &mut [u8], alpha: &[u8], width: i32, inverse: bool) {
    let width = usize::try_from(width).unwrap_or(0);
    for x in 0..width {
        let a = u32::from(alpha[x]);
        if a != 255 {
            if a == 0 {
                ptr[x] = 0;
            } else {
                let scale = get_scale(a, inverse);
                ptr[x] = low_byte(mult(ptr[x], scale));
            }
        }
    }
}

// upstream: src/dsp/alpha_processing.c WebPMultARGBRow
/// Premultiplies `width` ARGB pixels. The C dispatch pointer is `WebPMultARGBRow_C`.
pub fn webp_mult_argb_row(ptr: &mut [u32], width: i32, inverse: bool) {
    #[cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]
    {
        let features = crate::dsp::cpu::features();
        if features.sse2 {
            return crate::dsp::alpha_processing_sse2::mult_argb_row(ptr, width, inverse);
        }
    }
    #[allow(
        unreachable_code,
        reason = "the C path is reached only without a SIMD variant"
    )]
    webp_mult_argb_row_c(ptr, width, inverse);
}

// upstream: src/dsp/alpha_processing.c WebPMultRow
/// Premultiplies one row of single values. The C dispatch pointer is `WebPMultRow_C`.
pub fn webp_mult_row(ptr: &mut [u8], alpha: &[u8], width: i32, inverse: bool) {
    #[cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]
    {
        let features = crate::dsp::cpu::features();
        if features.sse2 {
            return crate::dsp::alpha_processing_sse2::mult_row(ptr, alpha, width, inverse);
        }
    }
    #[allow(
        unreachable_code,
        reason = "the C path is reached only without a SIMD variant"
    )]
    webp_mult_row_c(ptr, alpha, width, inverse);
}

// upstream: src/dsp/alpha_processing.c WebPMultARGBRows
/// Premultiplies `num_rows` rows of ARGB pixels. `stride` counts `u32`
/// elements, where C counts bytes of the same buffer (ADR 0003).
///
/// # Panics
///
/// Panics when `ptr` holds fewer than `num_rows` rows of `stride` elements
/// with `width` pixels each.
pub fn webp_mult_argb_rows(ptr: &mut [u32], stride: i32, width: i32, num_rows: i32, inverse: bool) {
    let stride = usize::try_from(stride).expect("the stride is not negative");
    for n in 0..usize::try_from(num_rows).unwrap_or(0) {
        webp_mult_argb_row(&mut ptr[n * stride..], width, inverse);
    }
}

// upstream: src/dsp/alpha_processing.c WebPMultRows
/// Premultiplies `num_rows` rows of single values, with the alpha values of
/// each row in `alpha`.
///
/// # Panics
///
/// Panics when a slice is shorter than the rows that `num_rows` addresses.
pub fn webp_mult_rows(
    ptr: &mut [u8],
    stride: i32,
    alpha: &[u8],
    alpha_stride: i32,
    width: i32,
    num_rows: i32,
    inverse: bool,
) {
    let stride = usize::try_from(stride).expect("the stride is not negative");
    let alpha_stride = usize::try_from(alpha_stride).expect("the stride is not negative");
    for n in 0..usize::try_from(num_rows).unwrap_or(0) {
        webp_mult_row(
            &mut ptr[n * stride..],
            &alpha[n * alpha_stride..],
            width,
            inverse,
        );
    }
}

// upstream: src/dsp/alpha_processing.c MULTIPLIER and PREMULTIPLY
/// `PREMULTIPLY(x, m)` for the premultiplied modes, with the scale of
/// `MULTIPLIER(a)`.
#[must_use]
fn premultiply(x: u8, m: u32) -> u8 {
    low_byte((u32::from(x) * m) >> 23)
}

// upstream: src/dsp/alpha_processing.c ApplyAlphaMultiply_C
/// Premultiplies the color channels of an RGBA or BGRA plane, or of an ARGB
/// plane when `alpha_first` is set, in place.
fn apply_alpha_multiply_c(rgba: &mut [u8], alpha_first: bool, w: i32, h: i32, stride: i32) {
    let w = usize::try_from(w).unwrap_or(0);
    let stride = usize::try_from(stride).expect("the stride is not negative");
    let rgb_offset = usize::from(alpha_first);
    let alpha_offset = if alpha_first { 0 } else { 3 };
    for row in 0..usize::try_from(h).unwrap_or(0) {
        let base = row * stride;
        for i in 0..w {
            let a = u32::from(rgba[base + alpha_offset + 4 * i]);
            if a != 0xff {
                let mult = a * MULTIPLIER_SCALE;
                let rgb = base + rgb_offset + 4 * i;
                rgba[rgb] = premultiply(rgba[rgb], mult);
                rgba[rgb + 1] = premultiply(rgba[rgb + 1], mult);
                rgba[rgb + 2] = premultiply(rgba[rgb + 2], mult);
            }
        }
    }
}

// upstream: src/dsp/alpha_processing.c WebPApplyAlphaMultiply
/// Premultiplies an RGBA, BGRA or ARGB plane of `w` by `h` pixels in place.
/// `alpha_first` is set for ARGB, where alpha comes first.
pub fn webp_apply_alpha_multiply(rgba: &mut [u8], alpha_first: bool, w: i32, h: i32, stride: i32) {
    #[cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]
    {
        let features = crate::dsp::cpu::features();
        if features.sse2 {
            return crate::dsp::alpha_processing_sse2::apply_alpha_multiply(
                rgba,
                alpha_first,
                w,
                h,
                stride,
            );
        }
    }
    #[allow(
        unreachable_code,
        reason = "the C path is reached only without a SIMD variant"
    )]
    apply_alpha_multiply_c(rgba, alpha_first, w, h, stride);
}

// upstream: src/dsp/alpha_processing.c dither_hi
/// Keeps the high nibble and repeats it in the low one.
#[must_use]
fn dither_hi(x: u8) -> u8 {
    (x & 0xf0) | (x >> 4)
}

// upstream: src/dsp/alpha_processing.c dither_lo
/// Keeps the low nibble and repeats it in the high one. The shift drops the
/// high bits, as the `uint8_t` return of the C function does.
#[must_use]
fn dither_lo(x: u8) -> u8 {
    (x & 0x0f) | (x << 4)
}

// upstream: src/dsp/alpha_processing.c multiply
/// Multiplies a nibble-expanded channel by the RGBA4444 scale.
#[must_use]
fn multiply(x: u8, m: u32) -> u8 {
    low_byte((u32::from(x) * m) >> 16)
}

// upstream: src/dsp/alpha_processing.c ApplyAlphaMultiply4444_C
/// Premultiplies an RGBA4444 or rgbA4444 plane in place. `rg_byte_pos` is the
/// byte of the pixel that holds red and green, 0 or 1.
#[expect(
    clippy::many_single_char_names,
    reason = "the C names w, h, r, g, b and a of the pixel channels"
)]
fn apply_alpha_multiply4444_c(
    rgba4444: &mut [u8],
    w: i32,
    h: i32,
    stride: i32,
    rg_byte_pos: usize,
) {
    let w = usize::try_from(w).unwrap_or(0);
    let stride = usize::try_from(stride).expect("the stride is not negative");
    for row in 0..usize::try_from(h).unwrap_or(0) {
        let base = row * stride;
        for i in 0..w {
            let rg_index = base + 2 * i + rg_byte_pos;
            let ba_index = base + 2 * i + (rg_byte_pos ^ 1);
            let rg = rgba4444[rg_index];
            let ba = rgba4444[ba_index];
            let a = ba & 0x0f;
            let mult = u32::from(a) * 0x1111; // MULTIPLIER(a)
            let r = multiply(dither_hi(rg), mult);
            let g = multiply(dither_lo(rg), mult);
            let b = multiply(dither_hi(ba), mult);
            rgba4444[rg_index] = (r & 0xf0) | ((g >> 4) & 0x0f);
            rgba4444[ba_index] = (b & 0xf0) | a;
        }
    }
}

// upstream: src/dsp/alpha_processing.c ApplyAlphaMultiply_16b_C
/// The RGBA4444 variant of `WebPApplyAlphaMultiply4444`, with red and green in
/// the first byte, as `WEBP_SWAP_16BIT_CSP` is 0 in the reference configuration.
fn apply_alpha_multiply_16b_c(rgba4444: &mut [u8], w: i32, h: i32, stride: i32) {
    apply_alpha_multiply4444_c(rgba4444, w, h, stride, 0);
}

// upstream: src/dsp/alpha_processing.c WebPApplyAlphaMultiply4444
/// Premultiplies an RGBA4444 plane of `w` by `h` pixels in place.
pub fn webp_apply_alpha_multiply4444(rgba4444: &mut [u8], w: i32, h: i32, stride: i32) {
    apply_alpha_multiply_16b_c(rgba4444, w, h, stride);
}

// upstream: src/dsp/alpha_processing.c DispatchAlpha_C
/// Copies the alpha plane into the A byte of each ARGB pixel of `dst`, which
/// has `dst_stride` bytes per row. Returns whether an alpha value is not 0xff.
fn dispatch_alpha_c(
    alpha: &[u8],
    alpha_stride: i32,
    width: i32,
    height: i32,
    dst: &mut [u8],
    dst_stride: i32,
) -> bool {
    let width = usize::try_from(width).unwrap_or(0);
    let alpha_stride = usize::try_from(alpha_stride).expect("the stride is not negative");
    let dst_stride = usize::try_from(dst_stride).expect("the stride is not negative");
    let mut alpha_mask = 0xff_u8;
    for j in 0..usize::try_from(height).unwrap_or(0) {
        for i in 0..width {
            let alpha_value = alpha[j * alpha_stride + i];
            dst[j * dst_stride + 4 * i] = alpha_value;
            alpha_mask &= alpha_value;
        }
    }
    alpha_mask != 0xff
}

// upstream: src/dsp/alpha_processing.c WebPDispatchAlpha
/// Dispatches the alpha values of `alpha` to the A bytes of `dst`. Returns
/// whether an alpha value is not 0xff. The C dispatch pointer is `DispatchAlpha_C`.
pub fn webp_dispatch_alpha(
    alpha: &[u8],
    alpha_stride: i32,
    width: i32,
    height: i32,
    dst: &mut [u8],
    dst_stride: i32,
) -> bool {
    #[cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]
    {
        let features = crate::dsp::cpu::features();
        if features.sse2 {
            return crate::dsp::alpha_processing_sse2::dispatch_alpha(
                alpha,
                alpha_stride,
                width,
                height,
                dst,
                dst_stride,
            );
        }
    }
    #[allow(
        unreachable_code,
        reason = "the C path is reached only without a SIMD variant"
    )]
    dispatch_alpha_c(alpha, alpha_stride, width, height, dst, dst_stride)
}

// upstream: src/dsp/alpha_processing.c DispatchAlphaToGreen_C
/// Stores the alpha values in the green channel of each `u32` pixel of `dst`,
/// with the other channels zeroed. `dst_stride` counts `u32` elements.
fn dispatch_alpha_to_green_c(
    alpha: &[u8],
    alpha_stride: i32,
    width: i32,
    height: i32,
    dst: &mut [u32],
    dst_stride: i32,
) {
    let width = usize::try_from(width).unwrap_or(0);
    let alpha_stride = usize::try_from(alpha_stride).expect("the stride is not negative");
    let dst_stride = usize::try_from(dst_stride).expect("the stride is not negative");
    for j in 0..usize::try_from(height).unwrap_or(0) {
        for i in 0..width {
            dst[j * dst_stride + i] = u32::from(alpha[j * alpha_stride + i]) << 8; // leave A/R/B channels zero'd.
        }
    }
}

// upstream: src/dsp/alpha_processing.c WebPDispatchAlphaToGreen
/// Stores the alpha values in the green channel of `dst`. The C dispatch
/// pointer is `DispatchAlphaToGreen_C`.
pub fn webp_dispatch_alpha_to_green(
    alpha: &[u8],
    alpha_stride: i32,
    width: i32,
    height: i32,
    dst: &mut [u32],
    dst_stride: i32,
) {
    #[cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]
    {
        let features = crate::dsp::cpu::features();
        if features.sse2 {
            return crate::dsp::alpha_processing_sse2::dispatch_alpha_to_green(
                alpha,
                alpha_stride,
                width,
                height,
                dst,
                dst_stride,
            );
        }
    }
    #[allow(
        unreachable_code,
        reason = "the C path is reached only without a SIMD variant"
    )]
    dispatch_alpha_to_green_c(alpha, alpha_stride, width, height, dst, dst_stride);
}

// upstream: src/dsp/alpha_processing.c ExtractAlpha_C
/// Copies the A bytes of an ARGB plane to `alpha`. Returns whether every alpha
/// value is 0xff.
fn extract_alpha_c(
    argb: &[u8],
    argb_stride: i32,
    width: i32,
    height: i32,
    alpha: &mut [u8],
    alpha_stride: i32,
) -> bool {
    let width = usize::try_from(width).unwrap_or(0);
    let argb_stride = usize::try_from(argb_stride).expect("the stride is not negative");
    let alpha_stride = usize::try_from(alpha_stride).expect("the stride is not negative");
    let mut alpha_mask = 0xff_u8;
    for j in 0..usize::try_from(height).unwrap_or(0) {
        for i in 0..width {
            let alpha_value = argb[j * argb_stride + 4 * i];
            alpha[j * alpha_stride + i] = alpha_value;
            alpha_mask &= alpha_value;
        }
    }
    alpha_mask == 0xff
}

// upstream: src/dsp/alpha_processing.c WebPExtractAlpha
/// Extracts the alpha values of an ARGB byte plane. Returns whether every
/// alpha value is 0xff. The C dispatch pointer is `ExtractAlpha_C`.
pub fn webp_extract_alpha(
    argb: &[u8],
    argb_stride: i32,
    width: i32,
    height: i32,
    alpha: &mut [u8],
    alpha_stride: i32,
) -> bool {
    #[cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]
    {
        let features = crate::dsp::cpu::features();
        if features.sse2 && features.sse4_1 {
            return crate::dsp::alpha_processing_sse41::extract_alpha(
                argb,
                argb_stride,
                width,
                height,
                alpha,
                alpha_stride,
            );
        }
        if features.sse2 {
            return crate::dsp::alpha_processing_sse2::extract_alpha(
                argb,
                argb_stride,
                width,
                height,
                alpha,
                alpha_stride,
            );
        }
    }
    #[allow(
        unreachable_code,
        reason = "the C path is reached only without a SIMD variant"
    )]
    extract_alpha_c(argb, argb_stride, width, height, alpha, alpha_stride)
}

// upstream: src/dsp/alpha_processing.c ExtractGreen_C
/// Copies the green byte of each `u32` pixel to `alpha`.
fn extract_green_c(argb: &[u32], alpha: &mut [u8], size: i32) {
    let size = usize::try_from(size).unwrap_or(0);
    for i in 0..size {
        alpha[i] = low_byte(argb[i] >> 8);
    }
}

// upstream: src/dsp/alpha_processing.c WebPExtractGreen
/// Copies the green byte of each `u32` pixel to `alpha`. The C dispatch
/// pointer is `ExtractGreen_C`.
pub fn webp_extract_green(argb: &[u32], alpha: &mut [u8], size: i32) {
    #[cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]
    {
        let features = crate::dsp::cpu::features();
        if features.sse2 {
            return crate::dsp::alpha_processing_sse2::extract_green(argb, alpha, size);
        }
    }
    #[allow(
        unreachable_code,
        reason = "the C path is reached only without a SIMD variant"
    )]
    extract_green_c(argb, alpha, size);
}

// upstream: src/dsp/alpha_processing.c MakeARGB32
/// Packs four channels into one ARGB pixel.
#[must_use]
fn make_argb32(a: u32, r: u32, g: u32, b: u32) -> u32 {
    (a << 24) | (r << 16) | (g << 8) | b
}

// upstream: src/dsp/alpha_processing.c PackRGB_C
/// Packs `len` RGB or BGR pixels into opaque ARGB pixels. `step` is the number
/// of bytes per pixel of the planes, 3 or 4, with the pixels of each plane at
/// that stride.
///
/// # Panics
///
/// Panics when a plane or `out` is shorter than the pixels that `len` needs.
pub fn pack_rgb_c(r: &[u8], g: &[u8], b: &[u8], len: i32, step: i32, out: &mut [u32]) {
    let len = usize::try_from(len).unwrap_or(0);
    let step = usize::try_from(step).expect("the step is not negative");
    let mut offset = 0_usize;
    for pixel in &mut out[..len] {
        *pixel = make_argb32(
            0xff,
            u32::from(r[offset]),
            u32::from(g[offset]),
            u32::from(b[offset]),
        );
        offset += step;
    }
}

// upstream: src/dsp/alpha_processing.c WebPPackRGB
/// Packs RGB or BGR planes into opaque ARGB pixels. The C dispatch pointer is `PackRGB_C`.
pub fn webp_pack_rgb(r: &[u8], g: &[u8], b: &[u8], len: i32, step: i32, out: &mut [u32]) {
    pack_rgb_c(r, g, b, len, step, out);
}

// upstream: src/dsp/alpha_processing.c HasAlpha8b_C
/// Whether any of the first `length` bytes of `src` is not 0xff.
fn has_alpha8b_c(src: &[u8], length: i32) -> bool {
    let length = usize::try_from(length).unwrap_or(0);
    src[..length].iter().any(|&value| value != 0xff)
}

// upstream: src/dsp/alpha_processing.c WebPHasAlpha8b
/// Whether any of the first `length` bytes of `src` is not 0xff. The C dispatch
/// pointer is `HasAlpha8b_C`.
#[must_use]
pub fn webp_has_alpha8b(src: &[u8], length: i32) -> bool {
    #[cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]
    {
        let features = crate::dsp::cpu::features();
        if features.sse2 {
            return crate::dsp::alpha_processing_sse2::has_alpha8b(src, length);
        }
    }
    #[allow(
        unreachable_code,
        reason = "the C path is reached only without a SIMD variant"
    )]
    has_alpha8b_c(src, length)
}

// upstream: src/dsp/alpha_processing.c HasAlpha32b_C
/// Whether the alpha byte of any of the first `length` ARGB pixels in the
/// byte plane `src` is not 0xff.
fn has_alpha32b_c(src: &[u8], length: i32) -> bool {
    let length = usize::try_from(length).unwrap_or(0);
    (0..length).any(|x| src[4 * x] != 0xff)
}

// upstream: src/dsp/alpha_processing.c WebPHasAlpha32b
/// Whether the alpha byte of any of the first `length` pixels of the RGBA byte
/// plane `src` is not 0xff. The C dispatch pointer is `HasAlpha32b_C`.
#[must_use]
pub fn webp_has_alpha32b(src: &[u8], length: i32) -> bool {
    #[cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]
    {
        let features = crate::dsp::cpu::features();
        if features.sse2 {
            return crate::dsp::alpha_processing_sse2::has_alpha32b(src, length);
        }
    }
    #[allow(
        unreachable_code,
        reason = "the C path is reached only without a SIMD variant"
    )]
    has_alpha32b_c(src, length)
}

// upstream: src/dsp/alpha_processing.c AlphaReplace_C
/// Replaces every pixel of `src` whose alpha is zero by `color`.
fn alpha_replace_c(src: &mut [u32], length: i32, color: u32) {
    let length = usize::try_from(length).unwrap_or(0);
    for pixel in &mut src[..length] {
        if *pixel >> 24 == 0 {
            *pixel = color;
        }
    }
}

// upstream: src/dsp/alpha_processing.c WebPAlphaReplace
/// Replaces every fully transparent pixel of `src` by `color`. The C dispatch
/// pointer is `AlphaReplace_C`.
pub fn webp_alpha_replace(src: &mut [u32], length: i32, color: u32) {
    #[cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]
    {
        let features = crate::dsp::cpu::features();
        if features.sse2 {
            return crate::dsp::alpha_processing_sse2::alpha_replace(src, length, color);
        }
    }
    #[allow(
        unreachable_code,
        reason = "the C path is reached only without a SIMD variant"
    )]
    alpha_replace_c(src, length, color);
}
