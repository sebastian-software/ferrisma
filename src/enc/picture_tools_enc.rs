// libwebp v1.6.0, src/enc/picture_tools_enc.c:
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
// WebPPicture tools: alpha handling, etc.
//
// Author: Skal (pascal.massimino@gmail.com)

//! Translation of `src/enc/picture_tools_enc.c` from libwebp v1.6.0.
//!
//! Work package: WP9 (encoder picture and config).
//!
//! # Deviations from upstream
//!
//! Changes of representation that the ADRs allow, or behavior that the Rust
//! side cannot reproduce exactly. Each records the reason.
//!
//! 1. **Missing planes.** `WebPCleanupTransparentArea` and `WebPBlendAlpha`
//!    return unchanged when a plane they need is missing, and
//!    `WebPReplaceTransparentPixels` returns when the ARGB plane is missing. C
//!    dereferences the missing plane for every block, which is a precondition
//!    violation; the cases are not generated.

use crate::dsp::alpha_processing::webp_alpha_replace;
use crate::dsp::yuv::{YUV_HALF, vp8rgb_to_u, vp8rgb_to_v, vp8rgb_to_y};
use crate::enc::picture_enc::{argb_samples_mut, yuva_planes_mut};
use crate::webp::encode::{WebPEncCSP, WebPPicture};

// upstream: src/enc/picture_tools_enc.c SIZE
/// The side of the blocks that are cleaned up.
const SIZE: i32 = 8;
// upstream: src/enc/picture_tools_enc.c SIZE2
const SIZE2: i32 = SIZE / 2;

/// A sample position as an index; a negative position is a precondition violation.
fn index(value: i32) -> usize {
    usize::try_from(value).expect("the sample position lies in the plane")
}

//------------------------------------------------------------------------------
// Helper: clean up fully transparent area to help compressibility.

// upstream: src/enc/picture_tools_enc.c IsTransparentARGBArea
/// Reports whether the `size * size` block at `ptr` has no opaque alpha, that is no
/// sample with a non-zero alpha byte.
fn is_transparent_argb_area(samples: &[u32], ptr: usize, stride: i32, size: i32) -> bool {
    let stride = index(stride);
    let mut row = ptr;
    for _ in 0..size {
        for x in 0..index(size) {
            if samples[row + x] & 0xff00_0000 != 0 {
                return false;
            }
        }
        row += stride;
    }
    true
}

// upstream: src/enc/picture_tools_enc.c Flatten
fn flatten(plane: &mut [u8], ptr: usize, v: u8, stride: i32, size: i32) {
    let stride = index(stride);
    let mut row = ptr;
    for _ in 0..size {
        plane[row..row + index(size)].fill(v);
        row += stride;
    }
}

// upstream: src/enc/picture_tools_enc.c FlattenARGB
fn flatten_argb(samples: &mut [u32], ptr: usize, v: u32, stride: i32, size: i32) {
    let stride = index(stride);
    let mut row = ptr;
    for _ in 0..size {
        for x in 0..index(size) {
            samples[row + x] = v;
        }
        row += stride;
    }
}

// upstream: src/enc/picture_tools_enc.c SmoothenBlock
/// Smoothen the luma components of transparent pixels. Returns `true` if the whole
/// block is transparent. The block starts at `a_pos` in the alpha plane and at
/// `y_pos` in the luma plane.
#[expect(
    clippy::too_many_arguments,
    reason = "the C signature has six parameters, and both planes need a position"
)]
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "the C conversion keeps the bit pattern, as the source does; the C conversion truncates as the source does"
)]
fn smoothen_block(
    a_plane: &[u8],
    a_pos: usize,
    a_stride: i32,
    y_plane: &mut [u8],
    y_pos: usize,
    y_stride: i32,
    width: i32,
    height: i32,
) -> bool {
    let mut sum = 0_i32;
    let mut count = 0_i32;
    let a_stride = index(a_stride);
    let y_stride = index(y_stride);
    let mut alpha_row = a_pos;
    let mut luma_row = y_pos;
    for _ in 0..height {
        for x in 0..index(width) {
            if a_plane[alpha_row + x] != 0 {
                count += 1;
                sum += i32::from(y_plane[luma_row + x]);
            }
        }
        alpha_row += a_stride;
        luma_row += y_stride;
    }
    if count > 0 && count < width * height {
        // C: (uint8_t)(sum / count)
        let avg_u8 = (sum / count) as u8;
        alpha_row = a_pos;
        luma_row = y_pos;
        for _ in 0..height {
            for x in 0..index(width) {
                if a_plane[alpha_row + x] == 0 {
                    y_plane[luma_row + x] = avg_u8;
                }
            }
            alpha_row += a_stride;
            luma_row += y_stride;
        }
    }
    count == 0
}

// upstream: src/enc/picture_tools_enc.c WebPCleanupTransparentArea
/// Smoothens the luma of the transparent blocks and flattens the fully transparent
/// 8x8 blocks, to help compressibility (C: `WebPCleanupTransparentArea`). The
/// right and bottom remainders are left alone, except for the luma smoothing.
///
/// A picture whose plane is missing (`NULL` in C) is left unchanged. C would
/// dereference that plane for any block, which is a precondition violation.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::too_many_lines,
    reason = "the C conversion keeps the bit pattern, as the source does; the C conversion truncates as the source does; the function follows the C control flow"
)]
pub fn webp_cleanup_transparent_area(pic: &mut WebPPicture<'_>) {
    let w = pic.width / SIZE;
    let h = pic.height / SIZE;

    // note: we ignore the left-overs on right/bottom, except for SmoothenBlock().
    if pic.use_argb != 0 {
        let Some(plane) = pic.argb else {
            return;
        };
        let stride = pic.argb_stride;
        let samples = argb_samples_mut(&mut pic.argb_storage, plane);
        let mut argb_value = 0_u32;
        for y in 0..h {
            let mut need_reset = true;
            for x in 0..w {
                let off = index((y * stride + x) * SIZE);
                if is_transparent_argb_area(samples, off, stride, SIZE) {
                    if need_reset {
                        argb_value = samples[off];
                        need_reset = false;
                    }
                    flatten_argb(samples, off, argb_value, stride, SIZE);
                } else {
                    need_reset = true;
                }
            }
        }
    } else {
        let width = pic.width;
        let height = pic.height;
        let y_stride = pic.y_stride;
        let uv_stride = pic.uv_stride;
        let a_stride = pic.a_stride;
        let [y_plane, u_plane, v_plane, a_plane] =
            yuva_planes_mut(&mut pic.yuva, [pic.y, pic.u, pic.v, pic.a]);
        let (Some(y_plane), Some(u_plane), Some(v_plane), Some(a_plane)) =
            (y_plane, u_plane, v_plane, a_plane)
        else {
            return;
        };
        let a_plane: &[u8] = a_plane;
        let mut values = [0_i32; 3];

        let mut y = 0_i32;
        while y + SIZE <= height {
            let mut need_reset = true;
            let mut x = 0_i32;
            while x + SIZE <= width {
                let a_pos = index(y * a_stride + x);
                let y_pos = index(y * y_stride + x);
                let u_pos = index((y / 2) * uv_stride + (x >> 1));
                let v_pos = index((y / 2) * uv_stride + (x >> 1));
                if smoothen_block(
                    a_plane, a_pos, a_stride, y_plane, y_pos, y_stride, SIZE, SIZE,
                ) {
                    if need_reset {
                        values[0] = i32::from(y_plane[y_pos]);
                        values[1] = i32::from(u_plane[u_pos]);
                        values[2] = i32::from(v_plane[v_pos]);
                        need_reset = false;
                    }
                    flatten(y_plane, y_pos, values[0] as u8, y_stride, SIZE);
                    flatten(u_plane, u_pos, values[1] as u8, uv_stride, SIZE2);
                    flatten(v_plane, v_pos, values[2] as u8, uv_stride, SIZE2);
                } else {
                    need_reset = true;
                }
                x += SIZE;
            }
            if x < width {
                let a_pos = index(y * a_stride + x);
                let y_pos = index(y * y_stride + x);
                smoothen_block(
                    a_plane,
                    a_pos,
                    a_stride,
                    y_plane,
                    y_pos,
                    y_stride,
                    width - x,
                    SIZE,
                );
            }
            y += SIZE;
        }
        if y < height {
            let sub_height = height - y;
            let mut x = 0_i32;
            while x + SIZE <= width {
                let a_pos = index(y * a_stride + x);
                let y_pos = index(y * y_stride + x);
                smoothen_block(
                    a_plane, a_pos, a_stride, y_plane, y_pos, y_stride, SIZE, sub_height,
                );
                x += SIZE;
            }
            if x < width {
                let a_pos = index(y * a_stride + x);
                let y_pos = index(y * y_stride + x);
                smoothen_block(
                    a_plane,
                    a_pos,
                    a_stride,
                    y_plane,
                    y_pos,
                    y_stride,
                    width - x,
                    sub_height,
                );
            }
        }
    }
}

//------------------------------------------------------------------------------
// Blend color and remove transparency info

// upstream: src/enc/picture_tools_enc.c BLEND
/// Blends `v0` and `v1` by `alpha` in 8 bits (C macro `BLEND`).
fn blend(v0: i32, v1: i32, alpha: i32) -> i32 {
    (((v0) * (255 - (alpha)) + (v1) * (alpha)) * 0x101 + 256) >> 16
}

// upstream: src/enc/picture_tools_enc.c BLEND_10BIT
/// Blends `v0` and `v1` by `alpha`, the sum of four 8-bit alphas (C macro
/// `BLEND_10BIT`).
fn blend_10bit(v0: u32, v1: u32, alpha: u32) -> u32 {
    (((v0) * (1020 - (alpha)) + (v1) * (alpha)) * 0x101 + 1024) >> 18
}

// upstream: src/enc/picture_tools_enc.c MakeARGB32
#[expect(
    clippy::cast_sign_loss,
    reason = "the C conversion keeps the bit pattern, as the source does"
)]
fn make_argb32(r: i32, g: i32, b: i32) -> u32 {
    0xff00_0000 | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}

// upstream: src/enc/picture_tools_enc.c WebPBlendAlpha
/// Blends the transparent samples of `picture` with the background color
/// `background_rgb` (`0xRRGGBB`), and makes every sample opaque (C: `WebPBlendAlpha`).
/// A YUV picture without alpha is left unchanged. A picture whose planes are missing
/// is left unchanged where C would dereference them.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    reason = "the C conversion keeps the bit pattern, as the source does; the C conversion truncates as the source does; the C conversion wraps as the source does"
)]
pub fn webp_blend_alpha(picture: &mut WebPPicture<'_>, background_rgb: u32) {
    let red = ((background_rgb >> 16) & 0xff) as i32;
    let green = ((background_rgb >> 8) & 0xff) as i32;
    let blue = ((background_rgb) & 0xff) as i32;
    if picture.use_argb == 0 {
        // omit last pixel during u/v loop
        let uv_width = picture.width >> 1;
        let y0 = vp8rgb_to_y(red, green, blue, YUV_HALF);
        // VP8RGBToU/V expects the u/v values summed over four pixels
        let u0 = vp8rgb_to_u(4 * red, 4 * green, 4 * blue, 4 * YUV_HALF);
        let v0 = vp8rgb_to_v(4 * red, 4 * green, 4 * blue, 4 * YUV_HALF);
        let has_alpha = picture.colorspace.0 & WebPEncCSP::WEBP_CSP_ALPHA_BIT.0;
        let width = picture.width;
        let height = picture.height;
        let y_stride = index(picture.y_stride);
        let uv_stride = index(picture.uv_stride);
        let a_stride = index(picture.a_stride);
        let [y_plane, u_plane, v_plane, a_plane] = yuva_planes_mut(
            &mut picture.yuva,
            [picture.y, picture.u, picture.v, picture.a],
        );
        let (Some(y_plane), Some(u_plane), Some(v_plane), Some(a_plane)) =
            (y_plane, u_plane, v_plane, a_plane)
        else {
            return;
        };
        if has_alpha == 0 {
            return; // nothing to do
        }
        for y in 0..height {
            let row = index(y);
            // Luma blending
            for x in 0..index(width) {
                let alpha = a_plane[row * a_stride + x];
                if alpha < 0xff {
                    let luma = &mut y_plane[row * y_stride + x];
                    *luma = blend(y0, i32::from(*luma), i32::from(alpha)) as u8;
                }
            }
            // Chroma blending every even line
            if (y & 1) == 0 {
                let a_row = row * a_stride;
                let a_row2 = if y + 1 == height {
                    a_row
                } else {
                    a_row + a_stride
                };
                let uv_row = (row / 2) * uv_stride;
                for x in 0..index(uv_width) {
                    // Average four alpha values into a single blending weight.
                    let alpha = u32::from(a_plane[a_row + 2 * x])
                        + u32::from(a_plane[a_row + 2 * x + 1])
                        + u32::from(a_plane[a_row2 + 2 * x])
                        + u32::from(a_plane[a_row2 + 2 * x + 1]);
                    let u = &mut u_plane[uv_row + x];
                    *u = blend_10bit(u0 as u32, u32::from(*u), alpha) as u8;
                    let v = &mut v_plane[uv_row + x];
                    *v = blend_10bit(v0 as u32, u32::from(*v), alpha) as u8;
                }
                if (width & 1) != 0 {
                    // rightmost pixel
                    let x = index(uv_width);
                    let alpha = 2
                        * (u32::from(a_plane[a_row + 2 * x]) + u32::from(a_plane[a_row2 + 2 * x]));
                    let u = &mut u_plane[uv_row + x];
                    *u = blend_10bit(u0 as u32, u32::from(*u), alpha) as u8;
                    let v = &mut v_plane[uv_row + x];
                    *v = blend_10bit(v0 as u32, u32::from(*v), alpha) as u8;
                }
            }
            // reset alpha value to opaque
            a_plane[row * a_stride..row * a_stride + index(width)].fill(0xff);
        }
    } else {
        let Some(plane) = picture.argb else {
            return;
        };
        let stride = picture.argb_stride;
        let width = picture.width;
        let height = picture.height;
        let background = make_argb32(red, green, blue);
        let samples = argb_samples_mut(&mut picture.argb_storage, plane);
        let mut row = 0_usize;
        for _ in 0..height {
            for x in 0..index(width) {
                let alpha = ((samples[row + x] >> 24) & 0xff) as i32;
                if alpha != 0xff {
                    if alpha > 0 {
                        let argb = samples[row + x];
                        let mut r = ((argb >> 16) & 0xff) as i32;
                        let mut g = ((argb >> 8) & 0xff) as i32;
                        let mut b = (argb & 0xff) as i32;
                        r = blend(red, r, alpha);
                        g = blend(green, g, alpha);
                        b = blend(blue, b, alpha);
                        samples[row + x] = make_argb32(r, g, b);
                    } else {
                        samples[row + x] = background;
                    }
                }
            }
            row += index(stride);
        }
    }
}

// upstream: src/enc/picture_tools_enc.c WebPReplaceTransparentPixels
/// Replaces the color of every fully transparent ARGB sample by `color`, whose alpha
/// is forced to zero (C: `WebPReplaceTransparentPixels`). Does nothing for a YUV
/// picture.
pub fn webp_replace_transparent_pixels(pic: &mut WebPPicture<'_>, color: u32) {
    if pic.use_argb == 0 {
        return;
    }
    let color = color & 0x00ff_ffff; // force alpha=0
    let width = pic.width;
    let stride = index(pic.argb_stride);
    let Some(plane) = pic.argb else {
        return;
    };
    let samples = argb_samples_mut(&mut pic.argb_storage, plane);
    let mut row = 0_usize;
    let mut y = pic.height;
    while y > 0 {
        y -= 1;
        webp_alpha_replace(&mut samples[row..], width, color);
        row += stride;
    }
}
