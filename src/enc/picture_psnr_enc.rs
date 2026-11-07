// libwebp v1.6.0, src/enc/picture_psnr_enc.c:
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
// WebPPicture tools for measuring distortion
//
// Author: Skal (pascal.massimino@gmail.com)

//! Translation of `src/enc/picture_psnr_enc.c` from libwebp v1.6.0.
//!
//! Work package: WP9 (encoder picture and config).
//!
//! The C file is compiled only without `WEBP_DISABLE_STATS` and `WEBP_REDUCE_SIZE`,
//! which the reference configuration does not define. `WebPPictureDistortion` reads
//! both inputs through shared views (`WebPPictureView`), which leave them unchanged.

use crate::dsp::dsp::VP8_SSIM_KERNEL;
use crate::dsp::ssim::{vp8_accumulate_sse, vp8ssim_get, vp8ssim_get_clipped};
use crate::enc::picture_csp_enc::webp_picture_yuva_to_argb;
use crate::enc::picture_enc::{argb_bytes, argb_samples};
use crate::enc::picture_rescale_enc::webp_picture_view;
use crate::utils::utils::webp_safe_malloc;
use crate::webp::encode::WebPPicture;

// upstream: src/enc/picture_psnr_enc.c RADIUS
/// The search radius of the local-minimum distortion. It shouldn't be too large.
const RADIUS: i32 = 2;

// upstream: src/enc/picture_psnr_enc.c kMinDistortion_dB
/// Max value returned in case of exact similarity.
const MIN_DISTORTION_DB: f64 = 99.0;

// upstream: src/enc/picture_psnr_enc.c AccumulateFunc
/// A metric over a plane: source and reference with their strides, the width and the
/// height, returning the accumulated distortion.
type AccumulateFunc = fn(&[u8], i32, &[u8], i32, i32, i32) -> f64;

/// A sample position as an index. The C loops only form positions inside the
/// planes, so a negative value is a precondition violation.
fn index(value: i32) -> usize {
    usize::try_from(value).expect("the sample position lies in the plane")
}

//------------------------------------------------------------------------------
// local-min distortion
//
// For every pixel in the *reference* picture, we search for the local best
// match in the compressed image. This is not a symmetrical measure.

// upstream: src/enc/picture_psnr_enc.c AccumulateLSIM
fn accumulate_lsim(
    src: &[u8],
    src_stride: i32,
    reference: &[u8],
    ref_stride: i32,
    w: i32,
    h: i32,
) -> f64 {
    let mut total_sse = 0.0_f64;
    for y in 0..h {
        let y_0 = if y - RADIUS < 0 { 0 } else { y - RADIUS };
        let y_1 = if y + RADIUS + 1 >= h {
            h
        } else {
            y + RADIUS + 1
        };
        for x in 0..w {
            let x_0 = if x - RADIUS < 0 { 0 } else { x - RADIUS };
            let x_1 = if x + RADIUS + 1 >= w {
                w
            } else {
                x + RADIUS + 1
            };
            let mut best_sse = 255.0 * 255.0;
            let value = f64::from(reference[index(y * ref_stride + x)]);
            for j in y_0..y_1 {
                let s = j * src_stride;
                for i in x_0..x_1 {
                    let diff = f64::from(src[index(s + i)]) - value;
                    let sse = diff * diff;
                    if sse < best_sse {
                        best_sse = sse;
                    }
                }
            }
            total_sse += best_sse;
        }
    }
    total_sse
}

// upstream: src/enc/picture_psnr_enc.c AccumulateSSE
fn accumulate_sse(
    src: &[u8],
    src_stride: i32,
    reference: &[u8],
    ref_stride: i32,
    w: i32,
    h: i32,
) -> f64 {
    let mut total_sse = 0.0_f64;
    let mut src_pos = 0_usize;
    let mut ref_pos = 0_usize;
    for _ in 0..h {
        total_sse += f64::from(vp8_accumulate_sse(
            &src[src_pos..],
            &reference[ref_pos..],
            w,
        ));
        src_pos = crate::enc::picture_enc::offset_by(src_pos, i64::from(src_stride));
        ref_pos = crate::enc::picture_enc::offset_by(ref_pos, i64::from(ref_stride));
    }
    total_sse
}

//------------------------------------------------------------------------------

// upstream: src/enc/picture_psnr_enc.c AccumulateSSIM
fn accumulate_ssim(
    src: &[u8],
    src_stride: i32,
    reference: &[u8],
    ref_stride: i32,
    w: i32,
    h: i32,
) -> f64 {
    let w0 = if w < VP8_SSIM_KERNEL {
        w
    } else {
        VP8_SSIM_KERNEL
    };
    let w1 = w - VP8_SSIM_KERNEL - 1;
    let h0 = if h < VP8_SSIM_KERNEL {
        h
    } else {
        VP8_SSIM_KERNEL
    };
    let h1 = h - VP8_SSIM_KERNEL - 1;
    let mut sum = 0.0_f64;
    let mut y = 0_i32;
    while y < h0 {
        for x in 0..w {
            sum += vp8ssim_get_clipped(src, src_stride, reference, ref_stride, x, y, w, h);
        }
        y += 1;
    }
    while y < h1 {
        let mut x = 0_i32;
        while x < w0 {
            sum += vp8ssim_get_clipped(src, src_stride, reference, ref_stride, x, y, w, h);
            x += 1;
        }
        while x < w1 {
            let off1 = x - VP8_SSIM_KERNEL + (y - VP8_SSIM_KERNEL) * src_stride;
            let off2 = x - VP8_SSIM_KERNEL + (y - VP8_SSIM_KERNEL) * ref_stride;
            sum += vp8ssim_get(
                &src[index(off1)..],
                src_stride,
                &reference[index(off2)..],
                ref_stride,
            );
            x += 1;
        }
        while x < w {
            sum += vp8ssim_get_clipped(src, src_stride, reference, ref_stride, x, y, w, h);
            x += 1;
        }
        y += 1;
    }
    while y < h {
        for x in 0..w {
            sum += vp8ssim_get_clipped(src, src_stride, reference, ref_stride, x, y, w, h);
        }
        y += 1;
    }
    sum
}

//------------------------------------------------------------------------------
// Distortion

// upstream: src/enc/picture_psnr_enc.c GetPSNR
fn get_psnr(v: f64, size: f64) -> f64 {
    if v > 0.0 && size > 0.0 {
        -4.342_944_8 * (v / (size * 255.0 * 255.0)).ln()
    } else {
        MIN_DISTORTION_DB
    }
}

// upstream: src/enc/picture_psnr_enc.c GetLogSSIM
fn get_log_ssim(v: f64, size: f64) -> f64 {
    let v = if size > 0.0 { v / size } else { 1.0 };
    if v < 1.0 {
        -10.0 * (1.0 - v).log10()
    } else {
        MIN_DISTORTION_DB
    }
}

// upstream: src/enc/picture_psnr_enc.c WebPPlaneDistortion
/// Measures the distortion between two planes with the metric `kind`: 0 for PSNR
/// (squared error), 1 for SSIM and 2 for the local-minimum measure. The samples are
/// `x_step` bytes apart in a row, and the rows are `src_stride` and `ref_stride`
/// bytes apart. Returns `false` without a result when the stride cannot hold a
/// row, or when the packed plane cannot be allocated.
///
/// As in C, the metric reads the packed or unpacked plane with the stride `width`
/// only when `x_step` is 1; the strides are used only when the plane is packed.
#[expect(
    clippy::cast_sign_loss,
    reason = "C converts the signed height to uint64_t, sign-extended, in the allocation size"
)]
#[expect(
    clippy::too_many_arguments,
    reason = "the C signature has ten parameters"
)]
#[expect(
    clippy::cast_possible_truncation,
    reason = "the C conversion truncates as the source does"
)]
#[expect(
    clippy::if_not_else,
    reason = "the branches keep the order of the C code"
)]
pub fn webp_plane_distortion(
    src: &[u8],
    src_stride: usize,
    reference: &[u8],
    ref_stride: usize,
    width: i32,
    height: i32,
    x_step: usize,
    kind: i32,
    distortion: &mut f32,
    result: &mut f32,
) -> bool {
    let metric: AccumulateFunc = if kind == 0 {
        accumulate_sse
    } else if kind == 1 {
        accumulate_ssim
    } else {
        accumulate_lsim
    };
    // C compares size_t values: a negative width converts to a huge size_t here.
    let Ok(width_len) = usize::try_from(width) else {
        return false;
    };
    if src_stride < x_step * width_len || ref_stride < x_step * width_len {
        return false;
    }

    // VP8SSIMDspInit() only assigns portable pointers, and ADR 0003 omits it.
    if x_step != 1 {
        // extract a packed plane if needed
        // C: 2ULL * width * height, where the signed height is sign-extended.
        let nmemb = 2_u64
            .wrapping_mul(width_len as u64)
            .wrapping_mul(i64::from(height) as u64);
        let Some(mut buffer) = webp_safe_malloc::<u8>(nmemb, std::mem::size_of::<u8>()) else {
            return false;
        };
        let plane_len = width_len * index(height);
        buffer.resize(2 * plane_len, 0);
        let (tmp1, tmp2) = buffer.split_at_mut(plane_len);
        for y in 0..index(height) {
            for x in 0..width_len {
                tmp1[x + y * width_len] = src[x * x_step + y * src_stride];
                tmp2[x + y * width_len] = reference[x * x_step + y * ref_stride];
            }
        }
        let value = metric(tmp1, width, tmp2, width, width, height);
        *distortion = value as f32;
    } else {
        // The metric reads both planes with the stride `width`, as C does.
        let value = metric(src, width, reference, width, width, height);
        *distortion = value as f32;
    }

    let area = f64::from(width) * f64::from(height);
    *result = if kind == 1 {
        get_log_ssim(f64::from(*distortion), area) as f32
    } else {
        get_psnr(f64::from(*distortion), area) as f32
    };
    true
}

// upstream: src/enc/picture_psnr_enc.c BLUE_OFFSET
/// The byte of an ARGB word that holds the blue channel, in native byte order (C:
/// `BLUE_OFFSET`). The results are reported as BGRA.
const BLUE_OFFSET: usize = if cfg!(target_endian = "big") { 3 } else { 0 };

// upstream: src/enc/picture_psnr_enc.c WebPPictureDistortion
/// Measures the distortion between `src` and `reference` in ARGB, one channel at a
/// time, with the metric `kind`, and stores the four channel results and the total
/// in `results[4]` (C: `WebPPictureDistortion`). Both inputs are read through shared
/// views, so neither is changed. A failure stops at the channel that fails; the
/// results already written stay, as in C. Returns `false` where C returns 0.
///
/// # Panics
///
/// Panics on an ARGB plane that C would dereference while it is missing.
#[expect(
    clippy::cast_possible_truncation,
    reason = "the totals are stored as float, as C stores them"
)]
pub fn webp_picture_distortion(
    src: &WebPPicture<'_>,
    reference: &WebPPicture<'_>,
    kind: i32,
    results: &mut [f32; 5],
) -> bool {
    if src.width != reference.width || src.height != reference.height {
        return false;
    }
    // VP8SSIMDspInit() only assigns portable pointers (ADR 0003).
    let width = src.width;
    let height = src.height;
    let Some(mut p0) = webp_picture_view(src, 0, 0, width, height) else {
        return false;
    };
    let Some(mut p1) = webp_picture_view(reference, 0, 0, width, height) else {
        return false;
    };

    // We always measure distortion in ARGB space.
    if p0.use_argb == 0 && webp_picture_yuva_to_argb(&mut p0).is_err() {
        return false;
    }
    if p1.use_argb == 0 && webp_picture_yuva_to_argb(&mut p1).is_err() {
        return false;
    }

    let stride0 = 4 * index(p0.argb_stride);
    let stride1 = 4 * index(p1.argb_stride);
    let bytes0 = argb_bytes(argb_samples(
        &p0.argb_storage,
        p0.argb.expect("the ARGB plane of the view is set"),
    ));
    let bytes1 = argb_bytes(argb_samples(
        &p1.argb_storage,
        p1.argb.expect("the ARGB plane of the view is set"),
    ));

    let mut total_distortion = 0.0_f64;
    let mut total_size = 0.0_f64;
    for (channel, channel_result) in results.iter_mut().take(4).enumerate() {
        // results are reported as BGRA
        let offset = channel ^ BLUE_OFFSET;
        let mut distortion = 0.0_f32;
        if !webp_plane_distortion(
            &bytes0[offset..],
            stride0,
            &bytes1[offset..],
            stride1,
            width,
            height,
            4,
            kind,
            &mut distortion,
            channel_result,
        ) {
            return false;
        }
        total_distortion += f64::from(distortion);
        total_size += f64::from(width * height);
    }

    results[4] = if kind == 1 {
        get_log_ssim(total_distortion, total_size) as f32
    } else {
        get_psnr(total_distortion, total_size) as f32
    };
    true
}
